//! Port of `lang_FR_CH.py` (Swiss French).
//!
//! Shape: **engine**. `Num2Word_FR_CH` subclasses `Num2Word_FR` →
//! `Num2Word_EUR` → `Num2Word_Base`. It supplies `high_numwords` (from EUR's
//! `setup`), `mid_numwords` (its own), and `low_numwords` (from FR's `setup`),
//! and overrides `merge`. `Num2Word_Base.to_cardinal` therefore drives
//! `splitnum`/`clean`, so the default `to_cardinal` in `base.rs` runs here and
//! this file supplies `cards`/`maxval`/`merge` plus the FR-inherited
//! `to_ordinal`/`to_ordinal_num`/`to_year`/`to_fraction` and their
//! float/Decimal entry hooks (all defined on `Num2Word_FR`).
//!
//! # Inheritance chain, resolved
//!
//! `Num2Word_FR_CH.setup` calls `Num2Word_FR.setup(self)` **first** and then
//! reassigns `self.mid_numwords` outright. Since `Num2Word_Base.__init__`
//! calls `setup()` before `set_numwords()`, FR's mid table (which contains
//! `(80, "quatre-vingts")` and no 70/90 entries) is **never** inserted into
//! `cards` — only the Swiss list below survives. Everything else FR's `setup`
//! assigns (negword, low_numwords, ords, exclude_title) is inherited as-is.
//!
//! Inherited unchanged from `Num2Word_FR` (ported verbatim below):
//!   * `to_ordinal`     — big-unit handling, `ords` table, `-ième` suffixing
//!   * `to_ordinal_num` — `str(value) + ("er" if value == 1 else "me")`
//!   * `to_year`        — `return self.to_cardinal(int(val))`, nothing more.
//!     No century splitting, no BC/AD suffix: `to_year == to_cardinal`.
//!   * `to_fraction`    — idiomatic demi/tiers/quart, ordinal-as-noun otherwise
//!
//! Inherited from `Num2Word_EUR`: `set_high_numwords` (long scale — GIGA
//! "illiard" at 10^n and MEGA "illion" at 10^(n-3), stepping -6) and
//! `gen_high_numwords`. `high_numwords` has 100 entries → `cap = 603`, the
//! highest card is 10^603 ("centilliard") and `MAXVAL = 1000 * 10^603 =
//! 10^606`. Values are unbounded BigInt; nothing here may be narrowed.
//!
//! `is_title` stays `false` (FR only populates `exclude_title`, never sets
//! `is_title`), so `title()` is a pass-through. `exclude_title` is carried
//! anyway to mirror the Python object.
//!
//! # What makes this Swiss
//!
//! Two deviations from `lang_FR`, both in scope:
//!
//! 1. **Vigesimal tens are gone.** `mid_numwords` supplies `nonante` (90),
//!    `huitante` (80) and `septante` (70) as first-class cards, so 70/80/90
//!    never decompose into `soixante-dix` / `quatre-vingts` / `quatre-vingt-dix`.
//! 2. **`merge` joins "et" with spaces, not hyphens**, and drops FR's
//!    `cnum != 80` guard. FR emits `"%s-et-%s"` → "vingt-et-un"; FR_CH emits
//!    `"%s et %s"` → "vingt et un". And because the `cnum != 80` guard is
//!    absent, 81 → "huitante et un" (FR would give "quatre-vingt-un").
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Verified against the frozen corpus:
//!
//! 1. `to_ordinal(0)` == **"zéroième"**. The `for…else` fallback blindly
//!    appends "ième" to "zéro"; the trailing char is 'o', not 'e', so nothing
//!    is stripped. Not a word in any French.
//! 2. `to_ordinal_num(0)` == **"0me"**, `to_ordinal_num(2)` == "2me". The real
//!    French abbreviation is "2e"/"2ème", never "2me".
//! 3. **Big-unit ordinals — plural suffix fixed (#165).** Python emits
//!    `to_ordinal(10**7)` == "dix millionièmes"; the port drops the
//!    agreement `s` like "cents"/"vingts" (#65): "dix millionième",
//!    "dix milliardième". Still reproduced: `to_ordinal(10**6)` ==
//!    "millionième" — the leading "un " is dropped for one but the count
//!    word is kept from two up.
//! 4. `to_ordinal(1000001)` == "un million unième" — the big-unit branch does
//!    not fire (the cardinal has a tail), so the "un " prefix survives here
//!    while `to_ordinal(1000000)` strips it.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// `Num2Word_FR._BIG_UNITS`, in Python's declaration order.
///
/// Order is load-bearing: `to_ordinal` returns from the first unit that
/// matches, and the suffixes overlap ("trilliard" vs "trillion", "milliard"
/// vs "million"), so a reordering would change output.
const BIG_UNITS: [&str; 6] = [
    "trilliard",
    "trillion",
    "billiard",
    "billion",
    "milliard",
    "million",
];

/// `Num2Word_FR.ords`, as an ordered slice.
///
/// Python 3.7+ dicts iterate in insertion order and the loop `break`s on the
/// first `endswith` hit, so a `Vec`/slice preserves the semantics exactly
/// where a `HashMap` would not.
const ORDS: [(&str, &str); 2] = [("cinq", "cinquième"), ("neuf", "neuvième")];

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Defined locally rather than imported from `lang_en` to keep this file
/// self-contained — the registry is generated mechanically and no other
/// language file may be depended on.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — `tens` is the outer loop.
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

fn pow10(n: i64) -> BigInt {
    BigInt::from(10u8).pow(n as u32)
}

/// `CURRENCY_FORMS` as `Num2Word_FR_CH` actually sees it at runtime.
///
/// # This is FR's table, *not* EUR's mutated one
///
/// `PORTING_CURRENCY.md` lists `Num2Word_FR_CH` among the 16 classes that read
/// the shared `Num2Word_EUR.CURRENCY_FORMS` dict after `Num2Word_EN.__init__`
/// has rewritten it in place. **That is not true of this class**, and the live
/// interpreter is the authority:
///
/// ```text
/// $ python3 -c "...; pprint(CONVERTER_CLASSES['fr_CH'].CURRENCY_FORMS)"
/// {'EUR': (('euro', 'euros'), ('centime', 'centimes')), ...}   # 14 codes
/// ```
///
/// `Num2Word_FR` declares its *own* `CURRENCY_FORMS` in its class body, which
/// shadows `Num2Word_EUR`'s attribute outright. EN's in-place mutation of the
/// EUR dict therefore never reaches FR or FR_CH, and none of EN's ~24 extra
/// codes (CHF, KWD, BHD, NGN, …) are visible here. The corpus confirms it from
/// the other side: `currency:CHF` and `currency:KWD` raise below rather than
/// resolving to EN's "franc"/"dinar".
///
/// So this is a verbatim transcription of `lang_FR.py`'s literal — the one
/// case where reading the source *is* correct. It coincidentally lands on the
/// same EUR forms EN would have injected (`("euro", "euros")`), which is why
/// the EUR rows pass either way; GBP is the discriminator, and the corpus's
/// "livre"/"livres" proves FR's table is the live one.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const DOLLARS: [&str; 2] = ["dollar", "dollars"];
    const CENTS: [&str; 2] = ["cent", "cents"];
    const CENTIMES: [&str; 2] = ["centime", "centimes"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();

    // ---- Num2Word_FR.CURRENCY_FORMS (class body), in declaration order ----
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &CENTIMES));
    m.insert("USD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("FRF", CurrencyForms::new(&["franc", "francs"], &CENTIMES));
    m.insert("CAD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("AUD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("HKD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("GBP", CurrencyForms::new(&["livre", "livres"], &["penny", "pence"]));
    // "fen" pluralizing to "jiaos" is wrong — a jiao is 10 fen, a different
    // unit entirely — so 0.01 CNY is "un fen" but 0.34 CNY is "trente-quatre
    // jiaos". Transcribed as-is from lang_FR.py; the corpus depends on it.
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuans"], &["fen", "jiaos"]));
    m.insert("JPY", CurrencyForms::new(&["yen", "yens"], &["sen", "sens"]));
    m.insert("INR", CurrencyForms::new(&["roupie", "roupies"], &["paisa", "paisas"]));
    m.insert("RUB", CurrencyForms::new(&["rouble", "roubles"], &["kopeck", "kopecks"]));
    m.insert("KRW", CurrencyForms::new(&["won", "wons"], &["jeon", "jeons"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &["centavo", "centavos"]));

    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged.
///
/// Unlike `CURRENCY_FORMS`, FR never declares its own `CURRENCY_ADJECTIVES`,
/// so this attribute *does* resolve up the MRO to `Num2Word_EUR` — confirmed
/// against the live class. The values are English ("Australian", "US") because
/// they are EUR's; no French translation exists anywhere in the chain.
///
/// Mostly dead weight in practice: `adjective=True` is only honoured on the
/// float path (FR's integer branch drops the flag — see `to_currency`), and
/// 8 of these 16 codes (BYN, EEK, HUF, ISK, NOK, RON, SAR, UZS) are absent
/// from FR's `CURRENCY_FORMS`, so they raise before the adjective is read.
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Australian"),
        ("BYN", "Belarusian"),
        ("CAD", "Canadian"),
        ("EEK", "Estonian"),
        ("USD", "US"),
        ("RUB", "Russian"),
        ("NOK", "Norwegian"),
        ("MXN", "Mexican"),
        ("RON", "Romanian"),
        ("INR", "Indian"),
        ("HUF", "Hungarian"),
        ("ISK", "íslenskar"),
        ("UZS", "Uzbekistan"),
        ("SAR", "Saudi"),
        ("JPY", "Japanese"),
        ("KRW", "Korean"),
    ]
    .into_iter()
    .collect()
}

/// Python `str(float)`/`repr(float)`, used only to build error-message text
/// (`errmsg_floatord`, `errmsg_negord`). The corpora compare exception
/// *types*; the text follows the Python format strings. Whole floats keep
/// their ".0" ("21.0"), |v| >= 1e16 switches to Python's exponent form
/// ("1e+20"), and -0.0 keeps its sign.
fn py_float_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        };
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Python exponent form: mantissa, 'e', sign, two-digit-minimum exponent.
        let s = format!("{:e}", f);
        if let Some((m, e)) = s.split_once('e') {
            let (sign, digits) = match e.strip_prefix('-') {
                Some(d) => ("-", d.to_string()),
                None => ("+", e.to_string()),
            };
            let digits = if digits.len() < 2 {
                format!("0{}", digits)
            } else {
                digits
            };
            return format!("{}e{}{}", m, sign, digits);
        }
        s
    } else if f.fract() == 0.0 {
        // repr keeps the trailing ".0" that Rust's `{}` would drop.
        format!("{:.1}", f)
    } else {
        format!("{}", f)
    }
}

/// Python `str(value)` for the float-or-Decimal union handed to the hooks.
fn py_num_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => py_float_str(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

pub struct LangFrCh {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangFrCh {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFrCh {
    pub fn new() -> Self {
        // Num2Word_EUR.setup
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

        // Num2Word_EUR.set_high_numwords: long scale.
        //   cap = 3 + 6 * len(high)                       (= 603)
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**n]     = word + GIGA_SUFFIX     ("illiard")
        //       cards[10**(n-3)] = word + MEGA_SUFFIX     ("illion")
        // The `n > 3` guard reproduces zip() stopping at the shorter operand.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(pow10(n), format!("{}illiard", word));
            cards.insert(pow10(n - 3), format!("{}illion", word));
            n -= 6;
        }

        // Num2Word_FR_CH.setup — replaces FR's mid table wholesale.
        // No "quatre-vingts"; septante/huitante/nonante are real cards.
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mille"),
                (100, "cent"),
                (90, "nonante"),
                (80, "huitante"),
                (70, "septante"),
                (60, "soixante"),
                (50, "cinquante"),
                (40, "quarante"),
                (30, "trente"),
            ],
        );

        // Num2Word_FR.setup — inherited unchanged.
        set_low_numwords(
            &mut cards,
            &[
                "vingt",
                "dix-neuf",
                "dix-huit",
                "dix-sept",
                "seize",
                "quinze",
                "quatorze",
                "treize",
                "douze",
                "onze",
                "dix",
                "neuf",
                "huit",
                "sept",
                "six",
                "cinq",
                "quatre",
                "trois",
                "deux",
                "un",
                "zéro",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]; the OrderedDict's first
        // key is the highest card (10^603), so MAXVAL = 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangFrCh {
            cards,
            maxval,
            exclude_title: vec!["et".into(), "virgule".into(), "moins".into()],
            // Built once here, never per call: these are class attributes in
            // Python, evaluated at import time.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check cannot fire on integer input; only the negative check
    /// is reachable, and it raises `TypeError` (matching the corpus's
    /// `err: "TypeError"` on every negative ordinal row).
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.verify_ordinal` for float/Decimal input, checks in
    /// Python's order:
    ///
    ///   1. `not value == int(value)` -> TypeError(errmsg_floatord)
    ///   2. `not abs(value) == value` -> TypeError(errmsg_negord)
    ///
    /// Both comparisons are *numeric*, so -0.0 passes both (`int(-0.0) ==
    /// -0.0` and `abs(-0.0) == -0.0` in IEEE): `to_ordinal(-0.0)` is
    /// "zéroième", not an error. A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
    /// Returns the integral value for the integer-path continuation.
    /// (NaN/±inf would make `int(value)` raise before either check, but the
    /// dispatcher keeps those on the Python side; `as_whole_int` maps them to
    /// the float-message arm, which is unreachable for them in practice.)
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_num_str(value)
                    )))
                } else {
                    Ok(i)
                }
            }
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_num_str(value)
            ))),
        }
    }
}

impl Lang for LangFrCh {
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
        " et"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "moins "
    }
    fn pointword(&self) -> &str {
        "virgule"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_FR_CH.merge`.
    ///
    /// Differs from `Num2Word_FR.merge` in exactly two places, both marked
    /// below: the "et" join uses spaces, and the `cnum != 80` guard is gone.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, cnum) = l;
        let (rtext, nnum) = r;

        let mut ctext = ltext.to_string();
        let mut ntext = rtext.to_string();

        let eighty = BigInt::from(80);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let ten = BigInt::from(10);

        if cnum.is_one() {
            if nnum < &million {
                return (rtext.to_string(), nnum.clone());
            }
            // NB: Python falls through here — when cnum == 1 and
            // nnum >= 1_000_000 neither branch of the if/else runs, so no
            // 's' is stripped or added. Hence "un million", not "un millions".
        } else {
            // Drop trailing 's' from 'cents' / 'vingts' before a continuation.
            // `(cnum - 80) % 100` uses Python's floor-mod: (30-80) % 100 == 50
            // in Python but -50 in Rust's `%`, so mod_floor is required for
            // parity even though both happen to be non-zero here.
            let ends_in_80 = (cnum - &eighty).mod_floor(&hundred).is_zero();
            let round_hundred = (cnum % &hundred).is_zero() && cnum < &thousand;
            if (ends_in_80 || round_hundred)
                && nnum < &million
                && ctext.chars().last() == Some('s')
            {
                ctext.pop();
            }
            // Pluralize a round multiplier that is not itself 'mille':
            // 2 * 100 -> "deux cents".
            if cnum < &thousand
                && nnum != &thousand
                && ntext.chars().last() != Some('s')
                && (nnum % &hundred).is_zero()
            {
                ntext.push('s');
            }
        }

        if nnum < cnum && cnum < &hundred {
            if (nnum % &ten).is_one() {
                // FR_CH: spaces around "et" (FR uses "%s-et-%s"), and no
                // `cnum != 80` guard — 81 becomes "huitante et un".
                return (format!("{} et {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{}-{}", ctext, ntext), cnum + nnum);
        }
        if nnum > cnum {
            return (format!("{} {}", ctext, ntext), cnum * nnum);
        }
        (format!("{} {}", ctext, ntext), cnum + nnum)
    }

    /// Port of `Num2Word_FR.to_ordinal` (inherited unchanged by FR_CH).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("premier".to_string());
        }
        let word = self.to_cardinal(value)?;

        // Big-unit ordinals. Checked in _BIG_UNITS order; the first hit wins.
        for unit in BIG_UNITS {
            if word == format!("un {}", unit) || word.ends_with(&format!(" {}s", unit)) {
                // Python's str.rstrip("s") strips *every* trailing 's'.
                let trimmed = word.trim_end_matches('s');
                let stripped = trimmed.strip_prefix("un ").unwrap_or(trimmed);
                // The plural "s" of "millions" is an agreement marker and
                // drops in the ordinal, as for "cents"/"vingts" (#65, #165):
                // "deux millionième", not "deux millionièmes".
                return Ok(format!("{}ième", stripped));
            }
            if word == unit {
                return Ok(format!("{}ième", unit));
            }
        }

        // Python's for/else: the else arm runs only if no `break` fired.
        // `replaced` carries the hit out of the loop so that the borrow taken
        // by `strip_suffix` ends before `w` is reassigned.
        let mut w = word;
        let mut replaced: Option<String> = None;
        for (src, repl) in ORDS {
            if let Some(head) = w.strip_suffix(src) {
                replaced = Some(format!("{}{}", head, repl));
                break;
            }
        }
        if let Some(hit) = replaced {
            w = hit;
        } else {
            // `word[-1]` / `word[:-1]` are character ops in Python; String::pop
            // is UTF-8 aware, so "zéro" is never split mid-codepoint.
            if w.chars().last() == Some('e') {
                w.pop();
            }
            // 200 -> "deux centième", not "deux centsième". (`endswith("ents")`
            // is subsumed by `endswith("ts")`; kept verbatim.)
            if w.ends_with("ts") || w.ends_with("ents") {
                w.pop();
            }
            w.push_str("ième");
        }
        Ok(w)
    }

    /// Port of `Num2Word_FR.to_ordinal_num`: `str(value) + ("er" | "me")`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let suffix = if value.is_one() { "er" } else { "me" };
        Ok(format!("{}{}", value, suffix))
    }

    /// Port of `Num2Word_FR.to_year`: `return self.to_cardinal(int(val))`.
    ///
    /// No century splitting and no era suffix — negatives simply pick up
    /// `negword` from `to_cardinal` ("moins cinq cents" for -500).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float/negative checks and `to_year`'s
    // `int()` truncation become reachable here — unlike on the BigInt hooks
    // above, where they are dead code. `to_cardinal` needs no override: the
    // FR_CH → FR → EUR → Base chain inherits base's `assert int(value) ==
    // value` routing, which is exactly the trait default (whole -> int path).

    /// `Num2Word_FR.to_ordinal(float/Decimal)` (inherited by FR_CH):
    /// `verify_ordinal`, then the integer path. Whole values ordinalise
    /// (5.0 -> "cinquième", 1.0 -> "premier" via the `value == 1` special
    /// case, 21.0 -> the Swiss "vingt et unième", Decimal("1E+2") ->
    /// "centième", 1e+16 -> "dix billiardième");
    /// fractional or negative values raise TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `Num2Word_FR.to_ordinal_num(float/Decimal)`:
    /// `str(value) + ("er" if value == 1 else "me")`. The numeric half is
    /// Python's `str(value)` (`repr_str`), so 1.0 yields "1.0er",
    /// Decimal("5.00") "5.00me", -0.0 "-0.0me" and Decimal("1E+20")
    /// "1E+20me". No overflow is possible — Python never words the value.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        Ok(format!(
            "{}{}",
            repr_str,
            if i.is_one() { "er" } else { "me" }
        ))
    }

    /// `Num2Word_FR.to_year(float/Decimal)`: `self.to_cardinal(int(val))` —
    /// `int()` truncates toward zero, so 2.5 -> "deux", -1.5 -> "moins un",
    /// 0.5 -> "zéro". No non-integer guard (contrast EN's TypeError):
    /// fractional values are silently truncated, floats and Decimals alike.
    /// NaN/±inf floats reproduce `int()`'s ValueError/OverflowError, though
    /// the dispatcher keeps those on the Python side.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = match value {
            FloatValue::Float { value: f, .. } => {
                if f.is_nan() {
                    return Err(N2WError::Value(
                        "cannot convert float NaN to integer".into(),
                    ));
                }
                if f.is_infinite() {
                    return Err(N2WError::Overflow(
                        "cannot convert float infinity to integer".into(),
                    ));
                }
                // Exact: every whole f64 is exactly representable.
                BigInt::from_f64(f.trunc()).ok_or_else(|| {
                    N2WError::Value("cannot convert float NaN to integer".into())
                })?
            }
            // int(Decimal) truncates toward zero.
            FloatValue::Decimal { value: d, .. } => d.with_scale(0).as_bigint_and_exponent().0,
        };
        self.to_cardinal(&i)
    }

    // ---- fractions --------------------------------------------------------

    /// Port of `Num2Word_FR.to_fraction` (issue #584), inherited unchanged by
    /// `Num2Word_FR_CH`: idiomatic demi/tiers/quart for denominators 2/3/4 —
    /// "tiers" is invariant in the plural ("1000000/3" -> "un million
    /// tiers"), demi/quart take -s. Every other denominator is
    /// ordinal-as-noun, appending "s" only when the numerator isn't 1 *and*
    /// the ordinal doesn't already end in 's' (a guard only;
    /// `to_ordinal` no longer pluralises big units, #165). `d == 1`
    /// (exactly 1, not -1) or `n == 0` short-circuits to the signed cardinal
    /// before any sign normalisation, and a zero denominator raises
    /// ZeroDivisionError first ("0/0" raises).
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        // is_negative = (numerator < 0) ^ (denominator < 0)
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();
        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "demi" } else { "demis" }).to_string()
        } else if abs_d == BigInt::from(3) {
            // 'tiers' is invariant in plural.
            "tiers".to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "quart" } else { "quarts" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() && !w.ends_with('s') {
                w.push('s');
            }
            w
        };
        let sign = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        Ok(format!("{}{} {}", sign, self.to_cardinal(&abs_n)?, den_word))
    }

    // ---- currency -----------------------------------------------------
    //
    // FR_CH adds nothing of its own here; every hook below resolves up the
    // MRO to `Num2Word_FR` (forms, pluralize, to_currency) or `Num2Word_EUR`
    // (adjectives). Only `lang_name` is genuinely FR_CH's, and only because
    // Python reads `self.__class__.__name__` off the instance.

    fn lang_name(&self) -> &str {
        // `self.__class__.__name__` — the *runtime* class, so the
        // NotImplementedError raised by FR's inherited code still names
        // FR_CH: 'Currency code "CHF" not implemented for "Num2Word_FR_CH"'.
        "Num2Word_FR_CH"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    // `currency_precision` is deliberately left at the trait default of 100.
    // `CURRENCY_PRECISION` is `{}` on the live FR_CH class: base declares it
    // empty, FR/FR_CH never touch it, and EN's `self.CURRENCY_PRECISION = {…}`
    // *rebinds* rather than mutates, so its 3-decimal entries do not leak.
    // Consequences, both corpus-confirmed:
    //   * JPY keeps divisor 100, so 12.34 JPY is "douze yens et trente-quatre
    //     sens" — the zero-decimal branch of `default_to_currency` is dead here.
    //   * KWD/BHD would be divisor 100 too, but they are absent from FR's
    //     forms and raise before precision is ever consulted.

    /// Port of `Num2Word_FR.pluralize`, which *overrides* `Num2Word_EUR`'s.
    ///
    /// EUR's rule is `0 if n == 1 else 1`; FR's is `forms[0] if abs(n) <= 1`.
    /// The difference is entirely about **zero**, and it is load-bearing: the
    /// corpus row `1.0 EUR -> "un euro et zéro centime"` needs `pluralize(0,
    /// ("centime", "centimes"))` to pick the singular. EUR's rule would emit
    /// "zéro centimes". Ports savoirfairelinux/num2words#532 (issue #70).
    ///
    /// `abs(n)` also makes -1 singular, though `to_currency` only ever passes
    /// non-negative counts, so that arm is unreachable from the corpus.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.abs() <= BigInt::one() { 0 } else { 1 };
        // Python indexes `forms[1]` unguarded; a 1-form tuple would raise
        // IndexError. Every FR entry has exactly 2 forms, so this is
        // unreachable — kept faithful rather than defaulted.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_FR.to_currency`.
    ///
    /// Two paths, split on `isinstance(val, int)` — *not* on whether the
    /// number is whole. `1` is "un euro"; `1.0` is "un euro et zéro centime".
    ///
    /// The integer path is FR's own hand-rolled branch and diverges from
    /// `Num2Word_Base.to_currency` in three observable ways, all reproduced:
    ///
    /// 1. **An unknown currency raises (fixed, #219).** Python caught the
    ///    KeyError and returned a bare `self.to_cardinal(val)` with no
    ///    currency word (`currency:CHF, 1000000 -> "un million"`) while the
    ///    float path raised; both raise NotImplementedError now.
    /// 2. **`adjective` is dropped on the floor.** The integer branch never
    ///    consults `CURRENCY_ADJECTIVES`, so `to_currency(2, "USD",
    ///    adjective=True)` is "deux dollars" while the float `2.0` is "deux US
    ///    dollars, ...". Not exercised by the corpus (every row is
    ///    `adjective=False`), but it is what Python does.
    /// 3. **`pluralize` is inlined, not called.** The branch open-codes
    ///    `cr1[0] if abs_val <= 1 else (cr1[1] if len(cr1) > 1 else cr1[0])`.
    ///    That is FR's `pluralize` rule plus a `len` guard the method lacks,
    ///    so the two agree on every FR entry. Kept as written.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` means the caller omitted `separator=`; resolve it to FR's own
        // signature default (" et") before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        // ---- integer path: never shows cents ----
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                // Python's `except KeyError: return self.to_cardinal(val)`
                // printed a bare number; raise like the float path (#219).
                None => return Err(crate::currency::unknown_currency(self, currency)),
            };

            // See divergence (2) above: the flag is accepted and ignored.
            let _ = adjective;

            let minus_str = if v.is_negative() {
                format!("{} ", self.negword().trim())
            } else {
                String::new()
            };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            let cr1 = &forms.unit;
            let currency_str = if abs_val <= BigInt::one() {
                cr1.first()
            } else if cr1.len() > 1 {
                cr1.get(1)
            } else {
                cr1.first()
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // ---- float path: `super().to_currency(...)` ----
        //
        // `Num2Word_EUR` defines no `to_currency`, so `super(Num2Word_FR,
        // self)` resolves straight through to `Num2Word_Base.to_currency` —
        // i.e. `default_to_currency`. FR forwards every kwarg unchanged.
        crate::currency::default_to_currency(self, val, currency, cents, separator, adjective)
    }

    // `to_cheque` is not overridden anywhere in the chain, so
    // `default_to_cheque` runs: 1234.56 EUR -> "MILLE DEUX CENT TRENTE-QUATRE
    // AND 56/100 EUROS". Note it takes `cr1[-1]` unconditionally, so the
    // English "AND" and the plural unit appear even in French, and an unknown
    // code (CHF/KWD/BHD) raises NotImplementedError with no int fallback.
}
