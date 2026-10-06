//! Port of `lang_SL.py` (Slovenian) via its `lang_EUR` → `Num2Word_Base` ancestry.
//!
//! Registry check: `__init__.py` maps `"sl"` to `lang_SL.Num2Word_SL()`, so this
//! is the right class. Note the registry holds a *module-level singleton*, which
//! matters for `ordflag` — see "Cross-call mutable state" below.
//!
//! Shape: **engine**. SL supplies `mid_numwords`/`low_numwords` and inherits
//! `set_high_numwords` from `Num2Word_EUR` (long scale, step -6, pairing
//! `GIGA_SUFFIX = "ilijard"` at 10^n with `MEGA_SUFFIX = "ilijon"` at 10^(n-3)).
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean`/`merge`.
//!
//! Card table: 100 high words ("cent" + the generated Latin stems) zipped with
//! `range(603, 3, -6)`, giving 10^603 "centilijard" … 10^9 "milijard" and
//! 10^600 "centilijon" … 10^6 "milijon". `MAXVAL = 1000 * 10^603 = 10^606`, so
//! values genuinely reach BigInt range — never narrow to a fixed-width int.
//! Python takes `list(cards.keys())[0]` (first *inserted*); insertion order here
//! is strictly descending, so `Cards::highest()` picks the same key.
//!
//! # Cross-call mutable state (IMPORTANT — flagged for the dispatcher)
//!
//! `Num2Word_SL` carries `self.ordflag`, set in `to_ordinal`, consumed by
//! `merge`, and cleared afterwards. Because the converter is a shared singleton
//! and `to_ordinal` does **not** clear the flag on an exception path
//! (`ordflag = True` → `to_cardinal` raises OverflowError → `ordflag = False` is
//! never reached), a single `to_ordinal(10**606)` permanently poisons the
//! instance and every later `to_cardinal` silently renders in ordinal mode.
//! This Rust port is stateless: `merge` takes `ordflag` as a parameter, the
//! cardinal path always passes `false`, and the ordinal path runs the same
//! engine through `SlOrdinal` with `true`. The leak is therefore NOT reproduced
//! (it is unreachable in-process here), and the Python dispatcher must be taught
//! not to depend on it.
//!
//! # Faithfully reproduced Python quirks
//!
//! - `to_ordinal(0)` is not in the `ordinals` table, so it falls through to the
//!   flag path and yields the non-word "niči" (cardinal "nič" + "i").
//! - `merge`'s 10^6 `else` arm tests `ntext.endswith("d")` twice; the second
//!   (`ntext += "e"`) is dead code, shadowed by the first (`ntext += "a"`).
//!   Kept verbatim.
//! - The `2 < cnum < 5` arm's `elif not ntext.endswith("d")` is a redundant
//!   negation of the branch it already failed. Kept verbatim.
//! - `if cnum == 1: return next` returns the *original* right tuple, discarding
//!   any `ctext`/`ntext` edits made above it (e.g. the "ena" → "en" trim at
//!   `nnum >= 1000`). Mirrored by returning `(rtext, rnum)` untouched.
//! - `to_ordinal` calls `verify_ordinal` twice (harmless, kept).
//! - `to_ordinal_num` carries a literal `# Is this correct??` comment upstream;
//!   it just appends ".".
//!
//! Python `len()`/slicing on `str` counts *characters*, and Slovenian numwords
//! contain multi-byte č/š/ž. Every trim here goes through `drop_last_char`
//! (char-wise) or `strip_suffix` (exact-suffix, byte-safe), never byte offsets.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Duplicated locally rather than imported from `lang_en` so this file stays
/// self-contained (the registry is generated mechanically; no cross-language
/// module dependency).
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    // Python: [u + t for t in tens for u in units] — `tens` is the outer loop.
    let mut out: Vec<String> = Vec::new();
    for t in tens {
        for u in units {
            out.push(format!("{}{}", u, t));
        }
    }
    out.reverse();

    const REPLACEMENTS: &[(&str, &str)] = &[
        ("novemn", "noven"),
        ("novemo", "novo"),
        ("octoo", "octo"),
        ("quintd", "quind"),
        ("quintn", "quin"),
        ("quintq", "quinq"),
        ("quints", "quins"),
        ("quintt", "quint"),
        ("quintv", "quinv"),
        ("septenn", "septen"),
        ("septent", "sept"),
        ("sexn", "sen"),
        ("sexs", "ses"),
        ("tresd", "tred"),
        ("tresn", "tren"),
        ("tress", "tres"),
        ("tresv", "trev"),
        ("unno", "uno"),
    ];
    for (k, v) in REPLACEMENTS {
        out = out.iter().map(|o| o.replace(k, v)).collect();
    }
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// Python `s[:-1]` / `s[: len(s) - 1]` — drop the last **character**.
fn drop_last_char(s: &str) -> String {
    let mut it = s.chars();
    it.next_back();
    it.as_str().to_string()
}

fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

/// `abs(Decimal(str(f)).as_tuple().exponent)` for an f64 — the fractional-digit
/// count of its shortest round-trip repr.
///
/// A local copy of `floatpath`'s private helper (SL's `to_cardinal_float`
/// recomputes `self.precision` inside `float2tuple(float(value))`, so it needs
/// this reduction rather than the repr-derived precision the dispatcher carried
/// in the `FloatValue`). Rust's `{}` for f64 is shortest-round-trip like
/// Python's `repr`, so counting the digits after the point matches. Only
/// non-integral values reach SL's float path — integral floats (`1.0`) take the
/// integer branch of `Num2Word_Base.to_cardinal` — so the exponent-form arm is
/// defensive only.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Best-effort Python `str(value)` for the verify_ordinal TypeError
/// messages. Only the exception *type* is corpus-observable; the exact
/// digits ride along for debuggability. `str(Decimal)` is exact via
/// `python_decimal_str`; a whole float displays as Rust would ("5" for 5.0),
/// which is close enough for an error string.
fn sl_float_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// `Num2Word_SL.CURRENCY_FORMS` — note the **3-tuple** shape.
///
/// SL shadows `Num2Word_EUR.CURRENCY_FORMS` with its own class-body dict, so
/// none of the EUR/EN runtime mutation described in PORTING_CURRENCY.md reaches
/// it: `Num2Word_EN.__init__` writes into `Num2Word_EUR`'s dict, which SL never
/// reads. Verified against the live interpreter — SL's table is exactly these
/// two codes, and every other code raises NotImplementedError.
///
/// Each entry is `(unit_forms, subunit_forms, cur_separator)`: a third element
/// that no other class in the library carries. `CurrencyForms` models only the
/// first two, so the separator rides alongside it in the map value. Its value
/// (`""` for both codes) is load-bearing twice:
///
/// * `to_currency` branches on `if cur_separator:` — empty means the cents
///   segment is joined by a plain space, never by the `separator=" in"` kwarg.
/// * `Num2Word_Base.to_cheque` unpacks the entry into **two** names, so the
///   third element makes every implemented code raise ValueError.
///
/// Four unit forms per code, indexed 0..=3 by `pluralize`, so the arity is
/// load-bearing (dropping one turns "pet evrov" into an IndexError).
fn build_currency_forms() -> HashMap<&'static str, (CurrencyForms, &'static str)> {
    const CENTS: [&str; 4] = ["cent", "centa", "cente", "centov"];

    let mut m: HashMap<&'static str, (CurrencyForms, &'static str)> = HashMap::new();
    m.insert(
        "EUR",
        (
            CurrencyForms::new(&["evro", "evra", "evre", "evrov"], &CENTS),
            "",
        ),
    );
    m.insert(
        "USD",
        (
            CurrencyForms::new(&["dolar", "dolarja", "dolarje", "dolarjev"], &CENTS),
            "",
        ),
    );
    m
}

/// `Num2Word_SL.pluralize`'s index selection, over the `n % 100` residue.
///
/// ```python
/// if n % 100 == 1:      return forms[0]
/// elif n % 100 == 2:    return forms[1]
/// elif n % 100 in [3,4]: return forms[2]
/// else:                 return forms[3]
/// ```
///
/// The residue is a `BigDecimal` because Python has two callers with two types:
/// the `pluralize` hook is handed an `int`, while `to_currency`'s
/// fractional-cents branch hands it a `Decimal`. Python compares both against
/// the same literals — `Decimal("1.00") == 1` is True, and `BigDecimal`'s
/// `PartialEq` is likewise numeric rather than representational, so one
/// comparison ladder serves both.
///
/// The 1/2/3/4 literals are interned rather than rebuilt per comparison: this
/// runs on every `to_currency` call, twice.
fn plural_form_index(rem: &BigDecimal) -> usize {
    static SMALL: OnceLock<[BigDecimal; 4]> = OnceLock::new();
    let s = SMALL.get_or_init(|| {
        [
            BigDecimal::from(1),
            BigDecimal::from(2),
            BigDecimal::from(3),
            BigDecimal::from(4),
        ]
    });
    if *rem == s[0] {
        0
    } else if *rem == s[1] {
        1
    } else if *rem == s[2] || *rem == s[3] {
        2
    } else {
        3
    }
}

/// `forms[plural_form_index(rem)]`.
///
/// Python indexes the tuple directly, so a form list shorter than the selected
/// index raises IndexError. Both SL entries carry four forms, so that is
/// unreachable — but it is mapped to `Index` rather than panicking so the
/// exception type survives if the table ever changes.
fn pick_form(rem: &BigDecimal, forms: &[String]) -> Result<String> {
    forms
        .get(plural_form_index(rem))
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// Port of `Num2Word_SL.merge`, with `self.ordflag` lifted to a parameter.
fn sl_merge(l: (&str, &BigInt), r: (&str, &BigInt), ordflag: bool) -> (String, BigInt) {
    // Python: ctext, cnum, ntext, nnum = curr + next
    let (ltext, cnum) = l;
    let (rtext, nnum) = r;
    let mut ctext = ltext.to_string();
    let mut ntext = rtext.to_string();

    let m6 = pow10(6); // 10**6 / 1000000
    let m9 = pow10(9); // 1000000000
    let c2 = BigInt::from(2u8);
    let c5 = BigInt::from(5u8);
    let c10 = BigInt::from(10u8);
    let c20 = BigInt::from(20u8);
    let c100 = BigInt::from(100u8); // 10**2
    let c1000 = BigInt::from(1000u16);

    if ctext.ends_with("dve") && ordflag && *nnum <= m6 {
        // "dve" -> "dva"
        ctext = format!("{}a", drop_last_char(&ctext));
    }

    if ctext == "dve" && !ordflag && *nnum < m9 {
        ctext = "dva".to_string();
    }

    if (ctext.ends_with("tri") || ctext.ends_with("štiri")) && *nnum == m6 && !ordflag {
        if ctext.ends_with("štiri") {
            // "štiri" -> "štir"  (char-wise: š is 2 bytes)
            ctext = drop_last_char(&ctext);
        }
        ctext.push_str("je"); // "trije" / "štirje"
    }

    if *cnum >= c20 && *cnum < c100 && *nnum == c2 {
        ntext = "dva".to_string();
    }

    if ctext.ends_with("ena") && *nnum >= c1000 {
        // "ena" -> "en"
        ctext = drop_last_char(&ctext);
    }

    if cnum.is_one() {
        if *nnum < m6 || ordflag {
            // Python `return next`: the ORIGINAL right tuple — any ctext/ntext
            // edits above are discarded.
            return (rtext.to_string(), nnum.clone());
        }
        ctext = String::new();
    }

    let val: BigInt;
    if nnum > cnum {
        if *nnum >= m6 {
            if ordflag {
                ntext.push('t');
            } else if *cnum == c2 {
                if ntext.ends_with('d') {
                    ntext.push('i');
                } else {
                    ntext.push('a');
                }
            } else if *cnum > c2 && *cnum < c5 {
                if ntext.ends_with('d') {
                    ntext.push('e');
                } else if !ntext.ends_with('d') {
                    // Redundant negation upstream; kept verbatim.
                    ntext.push('i');
                }
            } else if ctext.ends_with("en") {
                if ntext.ends_with('d') || ntext.ends_with('n') {
                    // Python: ntext += "" — deliberate no-op.
                }
            } else if ctext.ends_with("dve") && ntext.ends_with('n') {
                ctext = format!("{}a", drop_last_char(&ctext));
                ntext.push('a');
            } else if ctext.ends_with("je") && ntext.ends_with('n') {
                ntext.push('i');
            } else if ntext.ends_with('d') {
                ntext.push('a');
            } else if ntext.ends_with('n') {
                // Python: ntext += "" — deliberate no-op.
            } else if ntext.ends_with('d') {
                // Dead branch upstream (shadowed by the `+= "a"` arm above).
                ntext.push('e');
            } else {
                ntext.push_str("ov");
            }
        }

        if *nnum >= c100 && !ordflag && !ctext.is_empty() {
            ctext.push(' ');
        }

        val = cnum * nnum;
    } else {
        if *nnum < c10 && c10 < *cnum && *cnum < c100 {
            // Python: ntext, ctext = ctext, ntext + "in"  (RHS evaluated first)
            let old_ctext = ctext;
            ctext = format!("{}in", ntext);
            ntext = old_ctext;
        } else if *cnum >= c100 && !ordflag {
            ctext.push(' ');
        }
        val = cnum + nnum;
    }

    (format!("{}{}", ctext, ntext), val)
}

/// The `Num2Word_SL.to_ordinal` "flag method": the identical engine, run with
/// `ordflag = True`. Borrows `LangSl`'s tables so the 10^603 card build isn't
/// repeated.
struct SlOrdinal<'a> {
    cards: &'a Cards,
    maxval: &'a BigInt,
}

impl Lang for SlOrdinal<'_> {
    fn cards(&self) -> &Cards {
        self.cards
    }
    fn maxval(&self) -> &BigInt {
        self.maxval
    }
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "vejica"
    }
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        sl_merge(l, r, true)
    }
}

pub struct LangSl {
    cards: Cards,
    maxval: BigInt,
    /// `self.ords`, in Python dict **insertion order** — `to_ordinal` breaks on
    /// the first suffix hit, so the order is load-bearing.
    ords: Vec<(&'static str, &'static str)>,
    /// `CURRENCY_FORMS`, built once here rather than per `to_currency` call.
    /// Value is `(forms, cur_separator)` — see `build_currency_forms`.
    currency_forms: HashMap<&'static str, (CurrencyForms, &'static str)>,
}

impl Default for LangSl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSl {
    pub fn new() -> Self {
        // Num2Word_EUR.setup()
        let lows = ["non", "oct", "sept", "sext", "quint", "quadr", "tr", "b", "m"];
        let units = [
            "", "un", "duo", "tres", "quattuor", "quint", "sex", "septen", "octo", "novem",
        ];
        let tens = [
            "dec",
            "vigint",
            "trigint",
            "quadragint",
            "quinquagint",
            "sexagint",
            "septuagint",
            "octogint",
            "nonagint",
        ];
        let mut high = vec!["cent".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords: cap = 3 + 6*len(high) = 603;
        // zip(high, range(cap, 3, -6)) — 100 words, 100 exponents (603..=9).
        // SL: GIGA_SUFFIX = "ilijard" at 10^n, MEGA_SUFFIX = "ilijon" at 10^(n-3).
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // range() exhausted; zip() stops at the shorter sequence.
            }
            cards.insert(pow10(n as u32), format!("{}ilijard", word));
            cards.insert(pow10((n - 3) as u32), format!("{}ilijon", word));
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "tisoč"),
                (900, "devetsto"),
                (800, "osemsto"),
                (700, "sedemsto"),
                (600, "šeststo"),
                (500, "petsto"),
                (400, "štiristo"),
                (300, "tristo"),
                (200, "dvesto"),
                (100, "sto"),
                (90, "devetdeset"),
                (80, "osemdeset"),
                (70, "sedemdeset"),
                (60, "šestdeset"),
                (50, "petdeset"),
                (40, "štirideset"),
                (30, "trideset"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "dvajset",
                "devetnajst",
                "osemnajst",
                "sedemnajst",
                "šestnajst",
                "petnajst",
                "štirinajst",
                "trinajst",
                "dvanajst",
                "enajst",
                "deset",
                "devet",
                "osem",
                "sedem",
                "šest",
                "pet",
                "štiri",
                "tri",
                "dve",
                "ena",
                "nič",
            ],
        );

        // MAXVAL = 1000 * list(cards.keys())[0] = 1000 * 10**603 = 10**606.
        // Insertion order is strictly descending here, so first == highest.
        let maxval = cards.highest().cloned().unwrap_or_else(|| BigInt::from(0u8)) * 1000;

        LangSl {
            cards,
            maxval,
            ords: vec![
                ("ena", "prv"),
                ("dve", "drug"),
                ("tri", "tretj"),
                ("štiri", "četrt"),
                ("sedem", "sedm"),
                ("osem", "osm"),
                ("sto", "stot"),
                ("tisoč", "tisoč"),
                ("milijon", "milijont"),
            ],
            // Built once, never per call: `to_currency` only ever reads it.
            currency_forms: build_currency_forms(),
        }
    }

    /// `self.CURRENCY_FORMS[currency]` with Python's `except KeyError` arm.
    ///
    /// Both `to_currency` and `Num2Word_Base.to_cheque` wrap the subscript in
    /// the same try/except and raise the same NotImplementedError, so the
    /// lookup and its error live in one place.
    fn lookup_currency(&self, currency: &str) -> Result<&(CurrencyForms, &'static str)> {
        self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })
    }

    /// `Num2Word_Base.verify_ordinal` for integral input: negatives raise
    /// TypeError via `errmsg_negord` (SL does not override that message).
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// SL's `setup` overrides `errmsg_toobig`; `base.rs` hardcodes the
    /// `Num2Word_Base` wording, so re-emit with SL's text. Type is unchanged.
    fn retag_overflow(&self, r: Result<String>, value: &BigInt) -> Result<String> {
        match r {
            Err(N2WError::Overflow(_)) => Err(N2WError::Overflow(format!(
                "Number is too large to convert to words (abs({}) > {}).",
                value.abs(),
                self.maxval
            ))),
            other => other,
        }
    }
}

/// The literal `ordinals` table inside `Num2Word_SL.to_ordinal`.
/// Note 0 is absent — that is why `to_ordinal(0)` falls through to "niči".
fn simple_ordinal(value: &BigInt) -> Option<&'static str> {
    let n = value.to_u32()?;
    Some(match n {
        1 => "prvi",
        2 => "drugi",
        3 => "tretji",
        4 => "četrti",
        5 => "peti",
        6 => "šesti",
        7 => "sedmi",
        8 => "osmi",
        9 => "deveti",
        10 => "deseti",
        11 => "enajsti",
        12 => "dvanajsti",
        13 => "trinajsti",
        14 => "štirinajsti",
        15 => "petnajsti",
        16 => "šestnajsti",
        17 => "sedemnajsti",
        18 => "osemnajsti",
        19 => "devetnajsti",
        20 => "dvajseti",
        30 => "trideseti",
        40 => "štirideseti",
        50 => "petdeseti",
        60 => "šestdeseti",
        70 => "sedemdeseti",
        80 => "osemdeseti",
        90 => "devetdeseti",
        100 => "stoti",
        1000 => "tisoči",
        _ => return None,
    })
}

impl Lang for LangSl {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "EUR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " in"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "vejica"
    }
    // setup() sets exclude_title = [] but leaves is_title False, so title() is
    // the identity — the base defaults already match.

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        sl_merge(l, r, false)
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let r = default_to_cardinal(self, value);
        self.retag_overflow(r, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        // Python: `num = int(value)` cannot fail for integral input, so the
        // `except (ValueError, TypeError): return str(value)` arm is dead here.
        if let Some(word) = simple_ordinal(value) {
            return Ok(word.to_string());
        }

        // Python calls verify_ordinal a second time before setting the flag.
        self.verify_ordinal(value)?;

        // ordflag = True; outword = self.to_cardinal(value); ordflag = False
        let eng = SlOrdinal {
            cards: &self.cards,
            maxval: &self.maxval,
        };
        let r = default_to_cardinal(&eng, value);
        let mut outword = self.retag_overflow(r, value)?;

        // for key in self.ords: if outword.endswith(key): ... break
        // `outword[: len(outword) - len(key)]` with a confirmed suffix is
        // exactly `strip_suffix`, and stays char-correct for č/š/ž.
        for (key, replacement) in &self.ords {
            if let Some(stem) = outword.strip_suffix(key) {
                outword = format!("{}{}", stem, replacement);
                break;
            }
        }

        Ok(format!("{}i", outword))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}.", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Num2Word_SL.to_year ignores `longval` and defers to to_cardinal.
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_SL.to_cardinal_float(self, value)`.
    ///
    /// SL's own one-arg override, NOT `Num2Word_Base.to_cardinal_float` (the
    /// default this trait method would otherwise route to). It diverges twice:
    ///
    /// 1. **Fraction as a single number.** Where the base loops the padded
    ///    fractional string digit by digit, SL emits it whole:
    ///    `self.to_cardinal(int(post_str))`. So `12.34` is "dvanajst vejica
    ///    štiriintrideset" (34), never "... tri štiri"; `12.345` is
    ///    "dvanajst vejica tristo petinštirideset" (345). Python's `int()`
    ///    also dropped leading zeros, so `0.05` read exactly like `0.5`; the
    ///    port reads each leading zero as "nič" (gladiaio/num2words2#205):
    ///    `0.05` is "nič vejica nič pet".
    ///
    /// 2. **Always float-casts, even Decimal.** Python does
    ///    `self.float2tuple(float(value))` unconditionally, so the exact-Decimal
    ///    arm of `base.float2tuple` is never taken here. This *reintroduces*
    ///    issue #603 that the Decimal arm exists to avoid: `Decimal(
    ///    "98746251323029.99")` casts to the f64 `98746251323029.98`, so the
    ///    fraction reads 98 ("osemindevetdeset"), not 99. Likewise
    ///    `Decimal("1.10")` casts to `1.1` — precision 1, "ena", not the
    ///    precision-2 "deset" the exact arm would give. Reproduced by taking
    ///    `to_f64()` on the `Decimal` variant and running the Float branch.
    ///
    /// SL's signature has **no** `precision=` parameter, so `precision_override`
    /// is dead: even when the dispatcher pre-sets `self.precision`,
    /// `float2tuple` immediately overwrites it from `repr(float(value))`.
    /// Verified live — `num2words(0.5, lang='sl', precision=5)` is still
    /// "nič vejica pet". Ignored here to match.
    ///
    /// Only non-integral values arrive: `Num2Word_Base.to_cardinal` sends
    /// integral floats (`1.0`, `0.0`) down its integer branch, so `precision`
    /// is always `>= 1` in practice and the fraction segment always fires.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `float(value)` — cast unconditionally, including the Decimal arm.
        let f: f64 = match value {
            FloatValue::Float { value, .. } => *value,
            FloatValue::Decimal { value, .. } => value
                .to_f64()
                .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?,
        };

        // `pre, post = self.float2tuple(float(value))`. precision is recomputed
        // from repr(f) exactly as base.float2tuple's float branch does, NOT read
        // off the FloatValue (whose Decimal arm carries the exact exponent).
        let precision = float_repr_precision(f);
        let (pre, post) = float2tuple(&FloatValue::Float { value: f, precision });

        // post = str(post); post = "0" * (precision - len(post)) + post
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        let mut out = vec![self.to_cardinal(&pre)?];

        // if value < 0 and pre == 0: out = [negword.strip()] + out
        // `value.is_negative()` reads the ORIGINAL sign (Decimal or float),
        // matching Python's `value < 0` on the untouched parameter.
        if value.is_negative() && pre.is_zero() {
            out.insert(0, self.negword().trim().to_string());
        }

        if precision > 0 {
            out.push(self.title(self.pointword()));
            // self.to_cardinal(int(post_str)) — the whole fraction, one number.
            // Python's int() dropped the leading zeros, so 0.05 read like 0.5
            // ("nič vejica pet"); each one is now read as "nič" first
            // (gladiaio/num2words2#205): "nič vejica nič pet".
            let digits = post_str.trim_start_matches('0');
            for _ in 0..post_str.len() - digits.len() {
                out.push(self.to_cardinal(&BigInt::zero())?);
            }
            if !digits.is_empty() {
                // All ASCII digits, so the parse cannot fail.
                let post_num =
                    BigInt::parse_bytes(digits.as_bytes(), 10).unwrap_or_else(BigInt::zero);
                out.push(self.to_cardinal(&post_num)?);
            }
        }

        Ok(out.join(" "))
    }

    /// `to_ordinal(float/Decimal)`.
    ///
    /// `verify_ordinal(value)` runs first: a non-integral value raises
    /// TypeError (`errmsg_floatord`), a negative one TypeError
    /// (`errmsg_negord`) — note `-0.0` passes both (`int(-0.0) == -0.0` and
    /// `abs(-0.0) == -0.0` numerically). A surviving value is whole and
    /// non-negative; `num = int(value)` then walks the same table/ordflag
    /// machinery as the integer path, and `to_cardinal(<whole float>)`
    /// inside the ordflag arm takes Base's integer branch (the
    /// `assert int(value) == value` passes), so the output is byte-identical
    /// to `to_ordinal(int(value))`: `5.0` -> "peti", `0.0`/`-0.0` -> "niči",
    /// `1e+16` -> "desetbilijardti".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let Some(i) = value.as_whole_int() else {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                sl_float_repr(value)
            )));
        };
        if i.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                sl_float_repr(value)
            )));
        }
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)` — `verify_ordinal(value)`, then
    /// `str(value) + "."` with the *original* string form: `5.0` -> "5.0.",
    /// `-0.0` -> "-0.0.", `1e+16` -> "1e+16.", `Decimal("5.00")` -> "5.00.".
    /// Non-integral or negative values raise TypeError exactly as above.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let Some(i) = value.as_whole_int() else {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            )));
        };
        if i.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                repr_str
            )));
        }
        Ok(format!("{}.", repr_str))
    }

    // year_float_entry: SL's to_year is `self.to_cardinal(val)`, which is
    // exactly what the trait default (via cardinal_float_entry) does — whole
    // values take the integer branch, fractional ones SL's own
    // to_cardinal_float above.

    // ---- currency -------------------------------------------------------
    //
    // SL supplies `CURRENCY_FORMS`, `pluralize` and a `to_currency` that
    // replaces Base's outright — it reimplements the int/float split by hand
    // "to avoid `is_int_with_cents` issue" (its words). `to_cheque` is Base's,
    // and crashes on SL's data (see below). The rest of the currency path is
    // never reached, so the trait defaults stand:
    //
    //   * `currency_precision` — `CURRENCY_PRECISION` is Base's empty dict
    //     (EN rebinds rather than mutates it, so nothing leaks in); the default
    //     100 is what `.get(code, 100)` returns for every code. Moot regardless:
    //     SL's `to_currency` never consults it and hardcodes a divisor of 100,
    //     so the `divisor == 1` (JPY) and `divisor == 1000` (KWD/BHD) branches
    //     do not exist for SL. Those codes are absent from `CURRENCY_FORMS` and
    //     raise NotImplementedError first anyway — corpus rows confirm all 12
    //     JPY/KWD/BHD args raise rather than rendering.
    //   * `money_verbose` / `cents_verbose` / `cents_terse` — SL's
    //     `to_currency` inlines `self.to_cardinal` and never calls the first
    //     two; `cents=False` therefore does nothing and the third is dead
    //     (`to_currency(12.34, "EUR", cents=False)` still says "štiriintrideset
    //     centov", not "34"). Verified live.
    //   * `cardinal_from_decimal` — left at the default per
    //     PORTING_CURRENCY.md. This is the one known gap; see `to_currency`.
    //
    // `currency_adjective` is the one place where "default = Base's" is not the
    // whole story. SL does *not* define `CURRENCY_ADJECTIVES`, so it inherits
    // `Num2Word_EUR`'s populated 16-entry dict rather than Base's empty one —
    // but SL's `to_currency` ignores its own `adjective` kwarg entirely, and
    // `default_to_currency`/`default_to_cheque` (the only other readers of the
    // hook) are both overridden here. The table is unreachable, so it is not
    // transcribed; `adjective=True` is a no-op in Python too. Verified live:
    // `to_currency(12.34, "EUR", adjective=True)` == the plain rendering.

    fn lang_name(&self) -> &str {
        "Num2Word_SL"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code).map(|(forms, _)| forms)
    }

    /// Port of `Num2Word_SL.pluralize(n, forms)`.
    ///
    /// Python's `%` on `int` floors, so a negative `n` wraps *up* into a
    /// positive residue: `(-99) % 100 == 1` selects `forms[0]`. Rust's `%`
    /// truncates — `-99 % 100 == -99`, which matches nothing and falls to
    /// `forms[3]` — so `mod_floor` is required. The two agree on `-1`/`-100`
    /// and disagree on `-99`/`-98`/`-97`, which is why the test pins `-99`.
    ///
    /// Unreachable from `to_currency` (both operands are absolute values), but
    /// `pluralize` is a public hook and the residue rule is the whole point.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let rem = n.mod_floor(&BigInt::from(100));
        pick_form(&BigDecimal::from(rem), forms)
    }

    /// Port of `Num2Word_SL.to_currency`.
    ///
    /// Three of the five parameters are dead upstream and so are ignored here:
    ///
    /// * `separator` — SL declares `separator=" in"` and then never reads it,
    ///   using the third `CURRENCY_FORMS` element instead. The generated
    ///   `default_separator()` above still reports " in" because that is the
    ///   live signature's default; it simply has no effect. Passing
    ///   `separator=" XX"` changes nothing in Python. Verified live.
    /// * `cents` — no `_cents_terse` call exists on this path.
    /// * `adjective` — no `CURRENCY_ADJECTIVES` lookup exists on this path.
    ///
    /// # The int/float split
    ///
    /// `is_integer_input = isinstance(val, int)` gates the cents segment: a
    /// true `int` never prints cents, a float always does — including `1.0`,
    /// which is why `currency:EUR` of `1` is "ena evro" but of `1.0` is "ena
    /// evro nič centov". `has_decimal` is therefore unused here: SL does not
    /// consult it, and the `Int`/`Decimal` variants already carry the only
    /// distinction SL makes.
    ///
    /// # Known gap: fractional cents
    ///
    /// When `(Decimal(str(val)) * 100) % 1 != 0`, `parse_currency_parts` keeps
    /// the subunit as a `Decimal` and Python renders it with
    /// **`Num2Word_SL.to_cardinal_float`** — SL's own one-arg override, which
    /// emits the fractional digits as a *single number*
    /// (`to_cardinal(int(post_str))`). `cardinal_from_decimal`'s default routes
    /// to `Num2Word_Base.to_cardinal_float`, which emits them *digit by digit*.
    /// The two agree whenever the residue is a single digit and diverge beyond
    /// it:
    ///
    /// ```text
    /// to_currency(1.005,  "EUR")  "ena evro nič vejica pet centov"          both
    /// to_currency(1.0025, "EUR")  "ena evro nič vejica petindvajset centov" Python
    ///                             "ena evro nič vejica dve pet centov"      here
    /// ```
    ///
    /// Measured, not assumed: diffed against the live interpreter over 1751
    /// cases (17 currency codes x 51 ints incl. 10^100 and negatives x 50
    /// floats, plus cheques). Exactly 8 diverge — 1.0025, 0.0001, 1e-07 and
    /// 3.14159 under EUR and USD — and all 8 are this one cause. No corpus row
    /// is among them; reaching the branch needs 4+ decimal places.
    ///
    /// Closing it means porting SL's `to_cardinal_float`, which needs
    /// `repr(float)` semantics that this crate deliberately keeps on the Python
    /// side (see `currency.rs`'s header): `1e-07` renders "nič vejica ena"
    /// precisely *because* Python's repr goes exponential and SL then reads the
    /// zero-padded digits back as one number. That is the later float phase,
    /// not this one — and note `currency::cardinal_from_bigdecimal` calls
    /// `default_to_cardinal_float` directly rather than through
    /// `lang.to_cardinal_float`, so porting SL's override alone will not
    /// reroute this hook.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Python parses before the CURRENCY_FORMS lookup. Neither arm can
        // raise, so the order is not observable — but `to_cardinal` (which
        // *can* raise OverflowError) runs strictly after the lookup, so a
        // too-large value under an unknown code must report the missing code.
        // That ordering is preserved below.
        let (left, right, is_negative, fractional) = match val {
            CurrencyValue::Int(v) => (v.abs(), BigDecimal::zero(), v.is_negative(), false),
            CurrencyValue::Decimal { value, .. } => {
                // has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0.
                // `with_scale(0)` truncates; `x - trunc(x) != 0` iff x is
                // non-integral, which is what Python's `% 1 != 0` tests. The
                // sign is irrelevant to a `!= 0` test, so Decimal's
                // truncating `%` and a floor agree here.
                let scaled = value * BigDecimal::from(100);
                let hfc = &scaled - scaled.with_scale(0) != BigDecimal::zero();
                // SL calls parse_currency_parts with the default divisor=100.
                let (l, r, neg) = parse_currency_parts(val, false, hfc, 100);
                (l, r, neg, hfc)
            }
        };

        let (forms, cur_separator) = self.lookup_currency(currency)?;

        // Python: `"%s " % self.negword.strip()` — negword is "minus ", so
        // this is "minus ". Derived rather than spelled out so the two cannot
        // drift apart.
        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        let money_str = self.to_cardinal(&left)?;
        let left_form = self.pluralize(&left, &forms.unit)?;

        // Integer: no cents.
        if matches!(val, CurrencyValue::Int(_)) {
            return Ok(format!("{}{} {}", minus_str, money_str, left_form));
        }

        // Float: always show cents, even at zero. The `right > 0` guard is
        // redundant upstream — `to_cardinal(0)` is "nič" too — but it is the
        // shape Python has, and it is what keeps the fractional branch from
        // calling the float path on an exact zero.
        let cents_str = if right > BigDecimal::zero() {
            if fractional {
                // Python: self.to_cardinal_float(float(right)). See the
                // "Known gap" note above — this is not that function.
                self.cardinal_from_decimal(&right)?
            } else {
                // parse_currency_parts already applied `with_scale(0)` on this
                // path, so the unscaled value is the whole subunit count.
                self.to_cardinal(&right.as_bigint_and_exponent().0)?
            }
        } else {
            "nič".to_string()
        };

        // `pluralize(right, cr2_forms)` — Python passes `right` itself, an int
        // on the exact path and a Decimal on the fractional one.
        // `parse_currency_parts` guarantees `0 <= right < 100` here (it is
        // `fraction * 100` with `0 <= fraction < 1`), so `right % 100` is the
        // identity and no modulo is needed. A fractional residue matches none
        // of 1/2/3/4 and lands on `forms[3]` — "ena vejica ena centov", never
        // "…cent".
        let right_form = pick_form(&right, &forms.subunit)?;

        // `if cur_separator:` — statically false for both SL codes, but ported
        // rather than folded away: the empty string is data, not a constant.
        if cur_separator.is_empty() {
            Ok(format!(
                "{}{} {} {} {}",
                minus_str, money_str, left_form, cents_str, right_form
            ))
        } else {
            Ok(format!(
                "{}{} {}{} {} {}",
                minus_str, money_str, left_form, cur_separator, cents_str, right_form
            ))
        }
    }

    // SL's CURRENCY_FORMS entries carry more than two forms, so Base's
    // `cr1, _cr2 = ...` unpack raised ValueError for every known code.
    // No cheque rules, so NotImplementedError ("lang='sl' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Drive a corpus row exactly as `bench/diff_test.py::_currency_call`
    /// does — `arg` is `repr(value)`, so the int/float split comes from the
    /// literal's shape, not from re-parsing the number.
    fn currency(arg: &str, code: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let val = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangSl::new().to_currency(&val, code, true, None, false)
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangSl::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    /// The 12 args every currency code is exercised with in the corpus.
    const ARGS: [&str; 12] = [
        "0", "1", "2", "100", "12.34", "0.01", "1.0", "99.99", "1234.56", "-12.34", "1000000",
        "0.5",
    ];

    /// The 24 rendering rows: SL's only two implemented codes x `ARGS`.
    #[test]
    fn corpus_currency_implemented() {
        let eur = [
            "nič evrov",
            "ena evro",
            "dve evra",
            "sto evrov",
            "dvanajst evrov štiriintrideset centov",
            "nič evrov ena cent",
            "ena evro nič centov",
            "devetindevetdeset evrov devetindevetdeset centov",
            "tisoč dvesto štiriintrideset evrov šestinpetdeset centov",
            "minus dvanajst evrov štiriintrideset centov",
            "milijon evrov",
            "nič evrov petdeset centov",
        ];
        let usd = [
            "nič dolarjev",
            "ena dolar",
            "dve dolarja",
            "sto dolarjev",
            "dvanajst dolarjev štiriintrideset centov",
            "nič dolarjev ena cent",
            "ena dolar nič centov",
            "devetindevetdeset dolarjev devetindevetdeset centov",
            "tisoč dvesto štiriintrideset dolarjev šestinpetdeset centov",
            "minus dvanajst dolarjev štiriintrideset centov",
            "milijon dolarjev",
            "nič dolarjev petdeset centov",
        ];
        for (code, want) in [("EUR", &eur), ("USD", &usd)] {
            for (arg, want) in ARGS.iter().zip(want.iter()) {
                assert_eq!(currency(arg, code).unwrap(), *want, "{} {}", code, arg);
            }
        }
    }

    /// The other 84 rows. Every code outside SL's two-entry table raises,
    /// including the ones EN adds to the *EUR* dict (SL shadows it) and the
    /// 0-decimal/3-decimal ones whose divisor branches SL therefore never has.
    #[test]
    fn corpus_currency_not_implemented() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for arg in ARGS {
                match currency(arg, code) {
                    Err(N2WError::NotImplemented(m)) => assert_eq!(
                        m,
                        format!("Currency code \"{}\" not implemented for \"Num2Word_SL\"", code)
                    ),
                    other => panic!("{} {}: want NotImplemented, got {:?}", code, arg, other),
                }
            }
        }
    }

    /// SL has no cheque rules (#223): every code raises NotImplementedError,
    /// where Python crashed with ValueError on the 3-tuple unpack.
    #[test]
    fn corpus_cheque() {
        for code in ["EUR", "USD", "GBP", "JPY", "KWD"] {
            assert!(
                matches!(cheque("1234.56", code), Err(N2WError::NotImplemented(_))),
                "{}",
                code
            );
        }
    }

    /// `pluralize`'s four residue classes, and the `% 100` wrap that puts
    /// 101/102 back on the 1/2 forms.
    #[test]
    fn plural_residue_classes() {
        for (n, want) in [
            (0, "nič evrov"),
            (3, "tri evre"),
            (4, "štiri evre"),
            (5, "pet evrov"),
            (21, "enaindvajset evrov"),
            (101, "sto ena evro"),
            (102, "sto dve evra"),
            (103, "sto tri evre"),
            (104, "sto štiri evre"),
        ] {
            assert_eq!(currency(&n.to_string(), "EUR").unwrap(), want, "{}", n);
        }
        // Python's `%` floors, so a negative residue wraps up: (-1) % 100 == 99.
        let l = LangSl::new();
        let forms: Vec<String> = ["evro", "evra", "evre", "evrov"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(l.pluralize(&BigInt::from(-1), &forms).unwrap(), "evrov");
        // (-99) % 100 == 1 under a floor, so this is forms[0], not forms[3].
        assert_eq!(l.pluralize(&BigInt::from(-99), &forms).unwrap(), "evro");
    }

    /// Negative ints keep the cents segment off, and take `negword.strip()`.
    #[test]
    fn negative_int_has_no_cents() {
        assert_eq!(currency("-5", "EUR").unwrap(), "minus pet evrov");
        assert_eq!(currency("-1", "EUR").unwrap(), "minus ena evro");
    }

    /// `cents`, `separator` and `adjective` are all dead upstream.
    #[test]
    fn ignored_kwargs() {
        let l = LangSl::new();
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        let want = "dvanajst evrov štiriintrideset centov";
        assert_eq!(l.to_currency(&v, "EUR", false, None, false).unwrap(), want);
        assert_eq!(l.to_currency(&v, "EUR", true, Some(" XX"), false).unwrap(), want);
        assert_eq!(l.to_currency(&v, "EUR", true, None, true).unwrap(), want);
    }

    /// A float `cardinal` row, called exactly as the py binding builds it:
    /// `FloatValue::Float`, precision from repr, `precision_override = None`.
    fn cardinal_float(f: f64) -> String {
        LangSl::new()
            .to_cardinal_float(
                &FloatValue::Float {
                    value: f,
                    precision: float_repr_precision(f),
                },
                None,
            )
            .unwrap()
    }

    /// A `cardinal_dec` row: `FloatValue::Decimal` carrying the exact BigDecimal
    /// and its `abs(exponent)` precision, as the py binding constructs it.
    fn cardinal_dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        // The py binding carries abs(exponent); SL ignores it (it recomputes
        // from repr(float)), so the digit count of the literal is faithful.
        let precision = s.split_once('.').map_or(0, |(_, f)| f.len()) as u32;
        LangSl::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// The `to: "cardinal"` corpus rows whose arg has a dot (float input). SL
    /// renders the fraction as a single number, not digit by digit.
    #[test]
    fn corpus_cardinal_float() {
        for (f, want) in [
            (0.5, "nič vejica pet"),
            (1.5, "ena vejica pet"),
            (2.25, "dve vejica petindvajset"),
            (3.14, "tri vejica štirinajst"),
            (0.01, "nič vejica nič ena"),
            (0.1, "nič vejica ena"),
            (0.99, "nič vejica devetindevetdeset"),
            (1.01, "ena vejica nič ena"),
            (12.34, "dvanajst vejica štiriintrideset"),
            (99.99, "devetindevetdeset vejica devetindevetdeset"),
            (100.5, "sto vejica pet"),
            (1234.56, "tisoč dvesto štiriintrideset vejica šestinpetdeset"),
            (-0.5, "minus nič vejica pet"),
            (-1.5, "minus ena vejica pet"),
            (-12.34, "minus dvanajst vejica štiriintrideset"),
            // f64-artefact cases: 1.005 -> post 5 (padded "005"), 2.675 -> 675.
            (1.005, "ena vejica nič nič pet"),
            (2.675, "dve vejica šeststo petinsedemdeset"),
        ] {
            assert_eq!(cardinal_float(f), want, "{}", f);
        }
    }

    /// The `to: "cardinal_dec"` corpus rows. SL float-casts every Decimal, which
    /// reintroduces #603 (98746251323029.99 -> the f64 ...98) and drops
    /// Decimal("1.10")'s trailing zero to precision 1.
    #[test]
    fn corpus_cardinal_dec() {
        for (s, want) in [
            ("0.01", "nič vejica nič ena"),
            ("1.10", "ena vejica ena"),
            ("12.345", "dvanajst vejica tristo petinštirideset"),
            (
                "98746251323029.99",
                "osemindevetdeset bilijon sedemsto šestinštirideset milijarda dvesto \
                 enainpetdeset milijon tristo triindvajset tisoč devetindvajset vejica \
                 osemindevetdeset",
            ),
            ("0.001", "nič vejica nič nič ena"),
        ] {
            assert_eq!(cardinal_dec(s), want, "{}", s);
        }
    }

    /// The `precision=` kwarg is dead on SL's float path — float2tuple always
    /// recomputes precision from repr(float(value)).
    #[test]
    fn precision_override_ignored() {
        let v = FloatValue::Float {
            value: 0.5,
            precision: 1,
        };
        assert_eq!(
            LangSl::new().to_cardinal_float(&v, Some(5)).unwrap(),
            "nič vejica pet"
        );
    }
}
