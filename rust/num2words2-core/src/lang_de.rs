//! Port of `lang_DE.py` (via its `lang_EUR` → `Num2Word_Base` ancestry).
//!
//! DE is an *engine-style* language: it supplies `high/mid/low_numwords` +
//! `merge` and lets `Num2Word_Base.to_cardinal` drive `splitnum`/`clean`.
//!
//! Unlike EN, DE does **not** override `set_high_numwords`, so it inherits
//! `Num2Word_EUR`'s long scale (step -6, pairing `illion`/`illiarde`):
//! 10^6 Million, 10^9 Milliarde, 10^12 Billion, 10^15 Billiarde, ...
//! The top card is 10^603 `zentilliarde`, so MAXVAL is 10^606 — well beyond
//! any fixed-width integer, hence `BigInt` throughout.
//!
//! Note the case asymmetry inherited from Python: DE's `lows` are
//! capitalised (`M`, `B`, `Tr`, ...) while the generated Latin stems are
//! lowercase. So 10^6 renders as "Million" but 10^600 as "zentillion".
//! That is the Python behaviour and is reproduced verbatim.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// The elision rules are plain sequential string replaces, not morphological
/// logic — e.g. "novem"+"nonagint" = "novemnonagint", and replacing the
/// 6-char "novemn" with the 5-char "noven" yields "novenonagint" (a single
/// `n`, not "novennonagint"). Verified against Python: the resulting
/// 100-entry list matches `Num2Word_DE().high_numwords` exactly.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — tens is the outer loop.
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
        out = out.iter().map(|o| o.replace(*k, *v)).collect();
    }
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// `self.ords` from `setup()`. **Order is load-bearing**: `to_ordinal`
/// iterates the dict and breaks on the first suffix hit, and Python 3.7+
/// dicts iterate in insertion order. A `HashMap` would be wrong here.
///
/// In particular `rde` precedes `rden`, but `rden` is still reachable:
/// "milliarden" does not end in "rde" (its last three chars are "den").
const ORDS: &[(&str, &str)] = &[
    ("eins", "ers"),
    ("drei", "drit"),
    ("acht", "ach"),
    ("sieben", "sieb"),
    ("ig", "igs"),
    ("ert", "erts"),
    ("end", "ends"),
    ("ion", "ions"),
    ("nen", "ns"),
    ("rde", "rds"),
    ("rden", "rds"),
];

/// `[a-z]+` — one or more ASCII lowercase. Deliberately excludes the
/// non-ASCII letters that appear in DE output (ü, ö, ä, ß): Python's
/// `[a-z]` is a literal codepoint range and does not match them either.
fn is_ascii_lower_run(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase())
}

/// Predicate for `^[a-z]+(illion|illiard)ste$`.
///
/// The trailing `$` pins the alternation's end to `len-3`, so only two start
/// positions are ever viable and they are mutually exclusive (a string
/// cannot end in both "illionste" and "illiardste"). That makes the greedy
/// backtracking order irrelevant and reduces the regex to two suffix tests.
fn matches_illion_ste(rest: &str) -> bool {
    if let Some(core) = rest.strip_suffix("illionste") {
        if is_ascii_lower_run(core) {
            return true;
        }
    }
    if let Some(core) = rest.strip_suffix("illiardste") {
        if is_ascii_lower_run(core) {
            return true;
        }
    }
    false
}

/// `re.sub(r"eine ([a-z]+(illion|illiard)ste)$", lambda m: m.group(1), res)`
///
/// Group 1 spans from just after "eine " to end-of-string, so the
/// replacement is simply "drop the literal `eine `". `$` permits at most one
/// match; Python scans left-to-right, so the leftmost viable `eine ` wins.
fn sub_eine_illion_ste(res: &str) -> String {
    let b = res.as_bytes();
    let n = b.len();
    let mut i = 0usize;
    while i + 5 <= n {
        // ASCII bytes never occur inside a multi-byte UTF-8 sequence, so a
        // byte match on "eine " guarantees i and i+5 are char boundaries.
        if &b[i..i + 5] == b"eine " {
            let rest = &res[i + 5..];
            if matches_illion_ste(rest) {
                return format!("{}{}", &res[..i], rest);
            }
        }
        i += 1;
    }
    res.to_string()
}

/// `re.sub(r" ([a-z]+(illion|illiard)ste)$", lambda m: m.group(1), res)`
fn sub_space_illion_ste(res: &str) -> String {
    let b = res.as_bytes();
    for i in 0..b.len() {
        if b[i] == b' ' {
            let rest = &res[i + 1..];
            if matches_illion_ste(rest) {
                return format!("{}{}", &res[..i], rest);
            }
        }
    }
    res.to_string()
}

/// `re.sub(r"eine (million|milliard|billion|billiard)", r"ein\1", res)`
///
/// Unanchored, so this replaces *every* non-overlapping occurrence.
/// Alternation order matters in principle; in practice only one alternative
/// can match at any given position ("million" and "milliard" diverge at the
/// 6th char), so the outcome is order-independent. "eine milliarde" matches
/// only the "milliard" stem, leaving the trailing "e" → "einmilliarde".
fn sub_eine_prefix(res: &str) -> String {
    const ALTS: [&str; 4] = ["million", "milliard", "billion", "billiard"];
    let b = res.as_bytes();
    let n = b.len();
    let mut out = String::new();
    let mut i = 0usize;
    while i < n {
        let mut matched = false;
        if i + 5 <= n && &b[i..i + 5] == b"eine " {
            for alt in ALTS.iter() {
                let end = i + 5 + alt.len();
                if end <= n && &b[i + 5..end] == alt.as_bytes() {
                    out.push_str("ein");
                    out.push_str(alt);
                    i = end;
                    matched = true;
                    break;
                }
            }
        }
        if !matched {
            let ch = res[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

/// `Num2Word_DE.FEMININE_CURRENCIES` (issue #69).
///
/// Currencies whose German noun is feminine, so a value ending in 1 needs the
/// numeral "eine" rather than "ein"/"eins" — "eine Rupie", not "eins Rupie".
const FEMININE_CURRENCIES: [&str; 2] = ["DEM", "INR"];

/// Currencies whose German *subunit* noun is feminine: "eine Kopeke".
const FEMININE_SUBUNITS: [&str; 1] = ["RUB"];

/// The numeral as it stands before a noun (#253): a cardinal ending in "eins"
/// drops the "s" — "ein Euro", "einhundertein Dollar" — and takes "eine"
/// before a feminine noun ("eine Mark"). "eins" is the standalone form only.
fn attributive(cardinal: String, feminine: bool) -> String {
    match cardinal.strip_suffix("eins") {
        Some(stem) => format!("{}{}", stem, if feminine { "eine" } else { "ein" }),
        None => cardinal,
    }
}

/// `Num2Word_DE.CURRENCY_FORMS`.
///
/// DE declares its **own** class-level dict, so it shadows `Num2Word_EUR`'s
/// and is therefore *not* one of the 16 tables `Num2Word_EN.__init__` rewrites
/// in place. The lang_EUR-source-vs-runtime trap does not apply here, and this
/// is verified against the live interpreter: 15 entries, EUR staying
/// ("Euro", "Euro") and GBP ("Pfund", "Pfund").
///
/// The corollary matters more: DE cannot see the ~24 codes EN *adds* to the
/// shared EUR dict, so KWD/BHD/SGD/... genuinely raise NotImplementedError for
/// German. The corpus agrees — every KWD and BHD row is a NotImplementedError.
///
/// Two entries look like data bugs and are ports, not typos:
/// * CNY's subunits are ("Jiao", "Fen") — two *different* coins (1/10 and
///   1/100 yuan) filling the singular/plural slots, so 0.01 CNY renders
///   "ein Jiao" and 1.0 CNY "null Fen". Corpus-confirmed.
/// * INR's subunit is ("Paisa", "Paisa"); the real plural is "Paise".
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const DOLLAR: [&str; 2] = ["Dollar", "Dollar"];
    const CENT: [&str; 2] = ["Cent", "Cent"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["Euro", "Euro"], &CENT));
    m.insert("GBP", CurrencyForms::new(&["Pfund", "Pfund"], &["Penny", "Pence"]));
    m.insert("USD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("CAD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("AUD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("NZD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("HKD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("CNY", CurrencyForms::new(&["Yuan", "Yuan"], &["Jiao", "Fen"]));
    m.insert("DEM", CurrencyForms::new(&["Mark", "Mark"], &["Pfennig", "Pfennig"]));
    m.insert(
        "CHF",
        CurrencyForms::new(
            &["Schweizer Franken", "Schweizer Franken"],
            &["Rappen", "Rappen"],
        ),
    );
    m.insert("JPY", CurrencyForms::new(&["Yen", "Yen"], &["Sen", "Sen"]));
    m.insert("INR", CurrencyForms::new(&["Rupie", "Rupien"], &["Paisa", "Paisa"]));
    m.insert("RUB", CurrencyForms::new(&["Rubel", "Rubel"], &["Kopeke", "Kopeken"]));
    m.insert("KRW", CurrencyForms::new(&["Won", "Won"], &["Jeon", "Jeon"]));
    m.insert("MXN", CurrencyForms::new(&["Peso", "Pesos"], &["Centavo", "Centavos"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, which DE inherits untouched — it
/// declares no table of its own, and `Num2Word_EN` never writes to this dict
/// (only to `CURRENCY_FORMS`), so the source literal *is* what runs. Confirmed
/// against the live interpreter.
///
/// The adjectives stay English ("US", "Indian") in a German converter, and
/// most of these codes are not even in DE's `CURRENCY_FORMS`. Only the eight
/// in both tables are reachable, and only on the float path — DE's int path
/// ignores `adjective` entirely. `to_currency(12.34, "USD", adjective=True)`
/// is "zwölf US Dollar und vierunddreißig Cent".
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

pub struct LangDe {
    cards: Cards,
    maxval: BigInt,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangDe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangDe {
    pub fn new() -> Self {
        let lows = ["Non", "Okt", "Sept", "Sext", "Quint", "Quadr", "Tr", "B", "M"];
        let units = [
            "", "un", "duo", "tre", "quattuor", "quin", "sex", "sept", "okto", "novem",
        ];
        let tens = [
            "dez",
            "vigint",
            "trigint",
            "quadragint",
            "quinquagint",
            "sexagint",
            "septuagint",
            "oktogint",
            "nonagint",
        ];
        let mut high = vec!["zent".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords: cap = 3 + 6*len(high) = 603;
        // zip(high, range(cap, 3, -6)). Both sides have exactly 100 elements,
        // so nothing is truncated. `n <= 3` emulates range exhaustion.
        // Both GIGA_SUFFIX ("illiarde") and MEGA_SUFFIX ("illion") are
        // non-empty for DE, so both inserts always run.
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illiarde", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illion", word),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "tausend"),
                (100, "hundert"),
                (90, "neunzig"),
                (80, "achtzig"),
                (70, "siebzig"),
                (60, "sechzig"),
                (50, "f\u{fc}nfzig"),
                (40, "vierzig"),
                (30, "drei\u{df}ig"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "zwanzig",
                "neunzehn",
                "achtzehn",
                "siebzehn",
                "sechzehn",
                "f\u{fc}nfzehn",
                "vierzehn",
                "dreizehn",
                "zw\u{f6}lf",
                "elf",
                "zehn",
                "neun",
                "acht",
                "sieben",
                "sechs",
                "f\u{fc}nf",
                "vier",
                "drei",
                "zwei",
                "eins",
                "null",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]. Python's OrderedDict is
        // insertion-ordered and highs go in first, so key[0] is the largest:
        // 10^603 → MAXVAL 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangDe {
            cards,
            maxval,
            // Built once here, never per call: `to_currency` only reads these,
            // and rebuilding them per call is what made an earlier revision of
            // this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`, with DE's `errmsg_negord`.
    /// The float check is unreachable here (integer input only).
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Die negative Zahl {} kann nicht in eine Ordnungszahl konvertiert werden.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.verify_ordinal` on a float/Decimal, with DE's German
    /// error messages. Order matters and both checks are **numeric**:
    ///
    /// ```python
    /// if not value == int(value):   # 1.5, Decimal("3.25") -> errmsg_floatord
    ///     raise TypeError(...)
    /// if not abs(value) == value:   # -21.0 -> errmsg_negord; -0.0 PASSES
    ///     raise TypeError(...)
    /// ```
    ///
    /// Returns the whole value as a `BigInt` on success (for -0.0 that is 0,
    /// the sign gone — exactly what `int(value)` hands the integer path).
    /// `int(inf)`/`int(nan)` raise OverflowError/ValueError inside the first
    /// comparison; corpus-unreachable but mapped for totality.
    ///
    /// `display` feeds the `%s` in the message — only the exception *type* is
    /// corpus-checked, so a close-enough rendering suffices where the exact
    /// Python repr is not to hand.
    fn verify_ordinal_float(&self, value: &FloatValue, display: &str) -> Result<BigInt> {
        if let FloatValue::Float { value: f, .. } = value {
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
        }
        let whole = value.as_whole_int().ok_or_else(|| {
            N2WError::Type(format!(
                "Die Gleitkommazahl {} kann nicht in eine Ordnungszahl konvertiert werden.",
                display
            ))
        })?;
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Die negative Zahl {} kann nicht in eine Ordnungszahl konvertiert werden.",
                display
            )));
        }
        Ok(whole)
    }
}

/// `den_word[0].upper() + den_word[1:]` — Python's char-based first-letter
/// capitalisation. `to_uppercase()` can expand to more than one char (ß), but
/// no reachable stem starts with one; iterate chars to stay UTF-8-safe.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

/// A close-enough `str(value)` for DE's TypeError messages on the plain
/// ordinal float path, where the binding supplies no repr. Only the exception
/// type is corpus-checked.
fn float_display(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, precision } => {
            if value.is_finite() {
                format!("{:.*}", *precision as usize, value)
            } else {
                format!("{}", value)
            }
        }
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

impl Lang for LangDe {
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
        " und"
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
        "Komma"
    }

    /// Port of `Num2Word_DE.merge`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let one = BigInt::from(1);
        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let mega = BigInt::from(10u8).pow(6);

        if cnum == &one {
            if nnum == &hundred || nnum == &thousand {
                return (format!("ein{}", ntext), nnum.clone());
            } else if nnum < &mega {
                // Python `return next` — hands back the right-hand pair as-is.
                return (ntext, nnum.clone());
            }
            ctext = "eine".to_string();
        }

        let val: BigInt;
        if nnum > cnum {
            if nnum >= &mega {
                if cnum > &one {
                    // "Milliarde" → "Milliarden"; "Million" → "Millionen".
                    if ntext.ends_with('e') {
                        ntext.push('n');
                    } else {
                        ntext.push_str("en");
                    }
                }
                ctext.push(' ');
            }
            val = cnum * nnum;
        } else {
            // Python's chained comparison: nnum < 10 AND 10 < cnum AND cnum < 100.
            if nnum < &ten && &ten < cnum && cnum < &hundred {
                if nnum == &one {
                    ntext = "ein".to_string();
                }
                // Python: ntext, ctext = ctext, ntext + "und"  (RHS uses the
                // OLD values) — e.g. 20 + 1 → "ein" + "und" + "zwanzig".
                let old_ctext = ctext.clone();
                ctext = format!("{}und", ntext);
                ntext = old_ctext;
            } else if cnum >= &mega {
                ctext.push(' ');
            }
            val = cnum + nnum;
        }

        (format!("{}{}", ctext, ntext), val)
    }

    /// Delegates to the base engine, but pre-checks the overflow bound so the
    /// message is DE's `errmsg_toobig` rather than base.rs's English default.
    /// The condition is identical, so `default_to_cardinal`'s own check never
    /// fires afterwards.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let v = value.abs();
        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "Die Zahl {} muss kleiner als {} sein.",
                v,
                self.maxval()
            )));
        }
        default_to_cardinal(self, value)
    }

    /// Port of `Num2Word_DE.to_ordinal`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut outword = self.to_cardinal(value)?.to_lowercase();

        // First matching suffix wins, then break — see ORDS on why order matters.
        for (key, repl) in ORDS.iter() {
            if outword.ends_with(*key) {
                // All ORDS keys are ASCII, so a byte-length cut is identical
                // to Python's char-based outword[:len(outword)-len(key)].
                let cut = outword.len() - key.len();
                let newword = format!("{}{}", &outword[..cut], repl);
                outword = newword;
                break;
            }
        }

        let mut res = format!("{}te", outword);

        // "hundertste"/"tausendste" are preferred over "einhundertste"/"eintausendste".
        if res == "eintausendste" || res == "einhundertste" {
            res = res.replacen("ein", "", 1);
        }
        res = sub_eine_illion_ste(&res);
        res = sub_space_illion_ste(&res);
        res = sub_eine_prefix(&res);
        res = res.replace(' ', "");

        Ok(res)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_DE.to_year`.
    ///
    /// 1000..=1999 use the "<century>hundert<rest>" reading, so 1000 renders
    /// as "zehnhundert" (not "eintausend"); everything else is plain cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let thousand = BigInt::from(1000);
        let two_thousand = BigInt::from(2000);
        let max_year = BigInt::from(2999);

        if value < &thousand || value > &max_year {
            return self.to_cardinal(value);
        }
        if value < &two_thousand {
            // value is in 1000..=1999 here, so truncating / and % agree with
            // Python's floor // and % — no negative operands can reach this.
            let hundred = BigInt::from(100);
            let century = value / &hundred;
            let remainder = value % &hundred;
            let head = self.to_cardinal(&century)?;
            if remainder.is_zero() {
                Ok(format!("{}hundert", head))
            } else {
                Ok(format!(
                    "{}hundert{}",
                    head,
                    self.to_cardinal(&remainder)?
                ))
            }
        } else {
            self.to_cardinal(value)
        }
    }

    /// `to_ordinal(float/Decimal)` — `verify_ordinal` first (TypeError for
    /// any fractional or negative value, in that order), then the ordinal
    /// machinery on the whole value: `5.0` -> "fünfte", `-0.0` -> "nullte",
    /// `1e16` -> "zehnbilliardste", `-3.0`/`0.5` -> TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = self.verify_ordinal_float(value, &float_display(value))?;
        self.to_ordinal(&whole)
    }

    /// `to_ordinal_num(float/Decimal)` — `verify_ordinal`, then
    /// `str(value) + "."` with the repr verbatim: "5.00.", "1e+16.",
    /// "-0.0." (negative zero passes the numeric abs check).
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        self.verify_ordinal_float(value, repr_str)?;
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)` — `val = int(val)` **truncates** before the
    /// century logic: `1.5` -> "eins", `1000.0` -> "zehnhundert", `3.25` ->
    /// "drei". No validation, so negatives keep their cardinal reading.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let val = match value {
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
                BigInt::from_f64(f.trunc()).expect("finite after the guards above")
            }
            // `with_scale(0)` truncates toward zero — exactly `int(Decimal)`.
            FloatValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
        };
        self.to_year(&val)
    }

    /// Port of `Num2Word_DE.to_fraction` (issue #584): capitalised fraction
    /// noun with invariant plural.
    ///
    /// ```python
    /// if denominator == 0: raise ZeroDivisionError("denominator must not be zero")
    /// if denominator == 1 or numerator == 0: return self.to_cardinal(numerator)
    /// is_negative = (numerator < 0) ^ (denominator < 0)
    /// abs_n, abs_d = abs(int(numerator)), abs(int(denominator))
    /// ```
    ///
    /// * `abs_d == 2` -> "halb" (singular) / "halbe" (plural, adjectival).
    /// * 3..=12 -> the idiomatic noun table (Drittel, Viertel, …, Zwölftel).
    /// * 13..=19 -> cardinal + "tel", first letter upper-cased.
    /// * 20+ -> cardinal + "stel"; a stem that **equals** "einhundert" /
    ///   "eintausend" loses its "ein" ("Hundertstel", "Tausendstel"). The
    ///   loop also probes "einemillion"/"einemilliarde", but those can never
    ///   match — `to_cardinal(10**6)` is "eine Million", space and capital M
    ///   — so 1/1000000 really is "ein Eine Millionstel" (corpus-pinned).
    /// * numerator: "ein" for |n| == 1 (noun phrase, not "eins"), else the
    ///   cardinal; sign word from the XOR, so "1/-2" -> "minus ein halb" and
    ///   "-3/-4" -> "drei Viertel".
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();

        let de_frac: [(i64, &str); 10] = [
            (3, "Drittel"),
            (4, "Viertel"),
            (5, "Fünftel"),
            (6, "Sechstel"),
            (7, "Siebtel"),
            (8, "Achtel"),
            (9, "Neuntel"),
            (10, "Zehntel"),
            (11, "Elftel"),
            (12, "Zwölftel"),
        ];

        let den_word = if abs_d == BigInt::from(2) {
            if abs_n.is_one() { "halb" } else { "halbe" }.to_string()
        } else if let Some((_, word)) = de_frac
            .iter()
            .find(|(k, _)| abs_d == BigInt::from(*k))
        {
            word.to_string()
        } else if abs_d < BigInt::from(20) {
            let stem = self.to_cardinal(&abs_d)?;
            capitalize_first(&format!("{}tel", stem))
        } else {
            let mut stem = self.to_cardinal(&abs_d)?;
            // `if stem.startswith(prefix) and stem == prefix` — the equality
            // makes the startswith redundant; only the exact strings match.
            for prefix in ["einhundert", "eintausend", "einemillion", "einemilliarde"] {
                if stem == prefix {
                    stem = stem[3..].to_string(); // drop "ein"
                    break;
                }
            }
            capitalize_first(&format!("{}stel", stem))
        };

        let num_word = if abs_n.is_one() {
            "ein".to_string()
        } else {
            self.to_cardinal(&abs_n)?
        };
        // `"%s " % self.negword.strip()`.
        let sign = if is_negative { "minus " } else { "" };
        Ok(format!("{}{} {}", sign, num_word, den_word))
    }

    // ---- currency -------------------------------------------------------
    //
    // DE overrides only `to_currency`, and only its integer half. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` come from
    // `Num2Word_Base` and `pluralize` from `Num2Word_EUR`, so the trait
    // defaults already cover them.
    //
    // `currency_precision` is deliberately NOT overridden. DE never rebinds
    // `CURRENCY_PRECISION`, and EN's rebinding assigns an *instance* attribute
    // rather than mutating the class dict, so DE still reads Base's empty
    // mapping: `.get(code, 100)` is 100 for every code, JPY and KRW included.
    // The corpus proves it — `JPY 12.34` renders "zwölf Yen und vierunddreißig
    // Sen" rather than rounding to a whole yen — which also makes
    // `default_to_currency`'s `divisor == 1` branch unreachable for German.

    fn lang_name(&self) -> &str {
        "Num2Word_DE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. Every DE entry has two forms, so this is
    /// unreachable — mapped to `Index` rather than panicking so the exception
    /// type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// The whole-unit numeral in its attributive form (#253).
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        Ok(attributive(self.to_cardinal(number)?, FEMININE_CURRENCIES.contains(&currency)))
    }

    /// The subunit numeral in its attributive form (#253): "ein Cent".
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        Ok(attributive(self.to_cardinal(number)?, FEMININE_SUBUNITS.contains(&currency)))
    }

    /// Port of `Num2Word_DE.to_currency`.
    ///
    /// Only the `isinstance(val, int)` branch is DE's own; floats defer to
    /// `super()`, which is `Num2Word_Base.to_currency` (EUR does not override
    /// it). The two halves disagree in ways that look like oversights and are
    /// ported verbatim:
    ///
    /// * the int path never consults `adjective`, so
    ///   `to_currency(2, "USD", adjective=True)` is "zwei Dollar" while
    ///   `12.34` gives "zwölf US Dollar und vierunddreißig Cent";
    /// * Python's feminine-numeral fix-up was int-only (`1.0` INR was "eins
    ///   Rupie und null Paisa") and left "eins" before masculine nouns ("eins
    ///   Euro"). Both paths now take the attributive numeral from
    ///   `money_verbose`/`cents_verbose`: "ein Euro und ein Cent", "eine
    ///   Rupie", "einhundertein Dollar" (#253).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(v) = val {
            // Python catches `(KeyError, AttributeError)` around the forms
            // lookup and hands the call to `super()`, which repeats the lookup
            // and turns the KeyError into NotImplementedError. Delegating
            // reproduces that rather than raising here, so the message and its
            // wording keep coming from one place.
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };

            // `minus_str = self.negword if val < 0 else ""` — DE takes the raw
            // negword, trailing space intact, where Base uses
            // `negword.strip() + " "`. Same bytes for "minus ", kept literal.
            let minus_str = if v.is_negative() { self.negword() } else { "" };
            let abs_val = v.abs();
            // The attributive numeral, "ein Euro" / "eine Rupie" (#69, #253).
            let money_str = self.money_verbose(&abs_val, currency)?;

            // `cr1[0]` when abs_val == 1, else `cr1[1] if len(cr1) > 1 else cr1[0]`.
            let unit = &forms.unit;
            let currency_str = if abs_val.is_one() {
                unit.first()
            } else {
                unit.get(1).or_else(|| unit.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            // Python's trailing `.strip()`.
            return Ok(format!("{}{} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
