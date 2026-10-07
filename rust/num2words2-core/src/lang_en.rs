//! Port of `lang_EN.py` (via its `lang_EUR` → `Num2Word_Base` ancestry).
//!
//! EN overrides `set_high_numwords` to the short scale (step -3), so the
//! EUR long-scale illiard/illion pairing does not apply here — only EUR's
//! `gen_high_numwords` name generation is reused.
//!
//! Beyond the integer engine this file also ports the float/Decimal entry
//! behaviour that Python reaches when the dispatcher hands a `float` or
//! `Decimal` straight to the converter:
//!
//! - `verify_ordinal`'s two TypeErrors (non-integral value, negative value),
//!   with the quirk that `-0.0` passes *both* numeric comparisons and so
//!   ordinalises to "zeroth" rather than raising;
//! - `to_ordinal_num`'s `"%s%s" % (value, ordinal[-2:])`, which glues the
//!   Python string form of the value ("5.00", "1e+16", "-0.0") to the last
//!   two characters of the worded ordinal;
//! - `to_year`'s issue-#67 guard: non-integer *floats* raise TypeError, but
//!   Decimals of any scale skip the `isinstance(val, float)` check and are
//!   silently truncated by `int(val)` (Decimal("1.5") -> year 1);
//! - `to_year`'s `suffix=`/`longval=` kwargs (`suffix="BCE"` replaces the
//!   default "BC" for negatives and is appended to positives too);
//! - `Num2Word_EN.to_fraction`'s idiomatic "half/halves" and
//!   "quarter/quarters" forms for denominators 2 and 4 (issue #584).

use crate::base::{set_low_numwords, set_mid_numwords, Cards, KwVal, Kwargs, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Builds the Latin-prefix illion stems and applies the elision rules that
/// turn e.g. "septen" + "vigint" into "septemvigint"-style correct forms.
pub fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
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
        out = out.iter().map(|o| o.replace(k, v)).collect();
    }
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// `CURRENCY_FORMS` as `Num2Word_EN` actually sees it at runtime.
///
/// Built in two steps on purpose, mirroring Python: `Num2Word_EUR`'s
/// class-body dict first, then the overrides `Num2Word_EN.__init__` writes on
/// top of it. Pre-merging the two would hide which entries are EN's own, and
/// that list is exactly the diff a reviewer checks against `lang_EN.py`.
///
/// Note the arity: PLN carries three unit forms and RON three on both sides.
/// EUR's `pluralize` only ever indexes 0 or 1, so the third form is dead for
/// EN — but it is kept because the tuple shape is part of the ported data.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const DOLLARS: [&str; 2] = ["dollar", "dollars"];
    const CENTS: [&str; 2] = ["cent", "cents"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();

    // ---- Num2Word_EUR.CURRENCY_FORMS (class body) ----
    m.insert("AUD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("BYN", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"]));
    m.insert("CAD", CurrencyForms::new(&DOLLARS, &CENTS));
    // replaced by EUR
    m.insert("EEK", CurrencyForms::new(&["kroon", "kroons"], &["sent", "senti"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &CENTS));
    m.insert(
        "GBP",
        CurrencyForms::new(&["pound sterling", "pounds sterling"], &["penny", "pence"]),
    );
    // replaced by EUR
    m.insert("LTL", CurrencyForms::new(&["litas", "litas"], &CENTS));
    m.insert("LVL", CurrencyForms::new(&["lat", "lats"], &["santim", "santims"]));
    m.insert("USD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("RUB", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"]));
    m.insert("SEK", CurrencyForms::new(&["krona", "kronor"], &["öre", "öre"]));
    m.insert("NOK", CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"]));
    m.insert(
        "PLN",
        CurrencyForms::new(&["zloty", "zlotys", "zlotu"], &["grosz", "groszy"]),
    );
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &CENTS));
    m.insert(
        "RON",
        CurrencyForms::new(&["leu", "lei", "de lei"], &["ban", "bani", "de bani"]),
    );
    m.insert("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"]));
    m.insert("HUF", CurrencyForms::new(&["forint", "forint"], &["fillér", "fillér"]));
    m.insert("ISK", CurrencyForms::new(&["króna", "krónur"], &["aur", "aurar"]));
    m.insert("UZS", CurrencyForms::new(&["sum", "sums"], &["tiyin", "tiyins"]));
    m.insert(
        "SAR",
        CurrencyForms::new(&["saudi riyal", "saudi riyals"], &["halalah", "halalas"]),
    );
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"]));

    // ---- Num2Word_EN.__init__ overrides ----
    // Proper English pluralization.
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &CENTS));
    m.insert("USD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("GBP", CurrencyForms::new(&["pound", "pounds"], &["penny", "pence"]));
    m.insert("NGN", CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"]));
    // Common ISO 4217 codes that downstream users hit (#74).
    m.insert("AUD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("CAD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("HKD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("SGD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("CHF", CurrencyForms::new(&["franc", "francs"], &["rappen", "rappen"]));
    m.insert("AED", CurrencyForms::new(&["dirham", "dirhams"], &["fils", "fils"]));
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"]));
    m.insert("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"]));
    m.insert("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &CENTS));
    m.insert("BRL", CurrencyForms::new(&["real", "reais"], &CENTS));
    m.insert("ZAR", CurrencyForms::new(&["rand", "rand"], &CENTS));
    m.insert("SAR", CurrencyForms::new(&["riyal", "riyals"], &["halalah", "halalas"]));
    m.insert("QAR", CurrencyForms::new(&["riyal", "riyals"], &["dirham", "dirhams"]));
    m.insert("KWD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    // 3-decimal currencies (mils as subunit). Issue #256.
    m.insert("BHD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("OMR", CurrencyForms::new(&["rial", "rials"], &["baisa", "baisa"]));
    m.insert("JOD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("TND", CurrencyForms::new(&["dinar", "dinars"], &["millime", "millimes"]));
    m.insert("LYD", CurrencyForms::new(&["dinar", "dinars"], &["dirham", "dirhams"]));
    m.insert("IQD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));

    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`. EN never touches it.
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

/// `Num2Word_EN.__init__` *rebinds* `CURRENCY_PRECISION` to this dict, so the
/// empty base mapping is replaced outright rather than extended.
///
/// Only the seven 3-decimal (mils) currencies appear. JPY/KRW deliberately
/// stay at the default 100 — `lang_EN.py` keeps their historical sen/jeon
/// subunits because the fixtures expect them — so EN has no 0-decimal
/// currency and the `divisor == 1` branch is unreachable here.
fn build_currency_precision() -> HashMap<&'static str, i64> {
    [
        ("BHD", 1000),
        ("KWD", 1000),
        ("OMR", 1000),
        ("JOD", 1000),
        ("TND", 1000),
        ("LYD", 1000),
        ("IQD", 1000),
    ]
    .into_iter()
    .collect()
}

/// Python `str(float)`/`repr(float)`, used only to build error-message text
/// (`errmsg_floatord`, `errmsg_negord`, `to_year`'s TypeError). The corpora
/// compare exception *types*; the text follows the Python format strings.
/// Whole floats keep their ".0" ("21.0"), |v| >= 1e16 switches to Python's
/// exponent form ("1e+20"), and -0.0 keeps its sign.
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

pub struct LangEn {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
    currency_precision: HashMap<&'static str, i64>,
}

impl Default for LangEn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEn {
    pub fn new() -> Self {
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

        // EN's set_high_numwords: max = 3 + 3*len(high); zip(high, range(max, 3, -3))
        let max = 3 + 3 * high.len() as i64;
        let mut n = max;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illion", word));
            n -= 3;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "thousand"),
                (100, "hundred"),
                (90, "ninety"),
                (80, "eighty"),
                (70, "seventy"),
                (60, "sixty"),
                (50, "fifty"),
                (40, "forty"),
                (30, "thirty"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "twenty", "nineteen", "eighteen", "seventeen", "sixteen", "fifteen", "fourteen",
                "thirteen", "twelve", "eleven", "ten", "nine", "eight", "seven", "six", "five",
                "four", "three", "two", "one", "zero",
            ],
        );

        // MAXVAL = 1000 * highest card
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<&str, &str> = [
            ("one", "first"),
            ("two", "second"),
            ("three", "third"),
            ("four", "fourth"),
            ("five", "fifth"),
            ("six", "sixth"),
            ("seven", "seventh"),
            ("eight", "eighth"),
            ("nine", "ninth"),
            ("ten", "tenth"),
            ("eleven", "eleventh"),
            ("twelve", "twelfth"),
            ("twenty", "twentieth"),
            ("thirty", "thirtieth"),
            ("forty", "fortieth"),
            ("fifty", "fiftieth"),
            ("sixty", "sixtieth"),
            ("seventy", "seventieth"),
            ("eighty", "eightieth"),
            ("ninety", "ninetieth"),
            ("hundred", "hundredth"),
            ("thousand", "thousandth"),
            ("million", "millionth"),
            ("billion", "billionth"),
        ]
        .into_iter()
        .collect();

        LangEn {
            cards,
            maxval,
            ords,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
            // Built once here, never per call: `to_currency` only ever reads
            // these, and rebuilding them on each call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
            currency_precision: build_currency_precision(),
        }
    }

    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == num_bigint::Sign::Minus {
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
    /// "zeroth", not an error. A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
    /// Returns the integral value for the integer-path continuation.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        match value.as_whole_int() {
            Some(i) => {
                if i.sign() == num_bigint::Sign::Minus {
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

    /// Body of `Num2Word_EN.to_year`: the caller's (truthy) suffix wins,
    /// negatives default to "BC" otherwise.
    fn year_with_suffix(&self, value: &BigInt, suffix: Option<&str>) -> Result<String> {
        let mut val = value.clone();
        let mut suffix = suffix;
        if val.sign() == num_bigint::Sign::Minus {
            val = -val;
            if suffix.is_none() {
                suffix = Some("BC");
            }
        }
        let hundred = BigInt::from(100);
        let high = &val / &hundred;
        let low = &val % &hundred;

        // 00XX, X00X, or beyond 9999 fall back to plain cardinal.
        let ten = BigInt::from(10);
        let valtext = if high.is_zero()
            || (( &high % &ten).is_zero() && low < ten)
            || high >= hundred
        {
            self.to_cardinal(&val)?
        } else {
            let hightext = self.to_cardinal(&high)?;
            let lowtext = if low.is_zero() {
                "hundred".to_string()
            } else if low < ten {
                format!("oh-{}", self.to_cardinal(&low)?)
            } else {
                self.to_cardinal(&low)?
            };
            format!("{} {}", hightext, lowtext)
        };
        Ok(match suffix {
            Some(s) => format!("{} {}", valtext, s),
            None => valtext,
        })
    }
}

impl Lang for LangEn {
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
        ","
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
        "point"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);

        if lnum.is_one() && rnum < &hundred {
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            (format!("{}-{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            (format!("{} and {}", ltext, rtext), lnum + rnum)
        } else if rnum > lnum {
            (format!("{} {}", ltext, rtext), lnum * rnum)
        } else {
            (format!("{}, {}", ltext, rtext), lnum + rnum)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        let last = outwords.last().unwrap().clone();
        let mut lastwords: Vec<String> = last.split('-').map(|s| s.to_string()).collect();
        let lastword = lastwords.last().unwrap().to_lowercase();

        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => o.to_string(),
            None => {
                if lastword.ends_with('y') {
                    format!("{}ieth", &lastword[..lastword.len() - 1])
                } else {
                    format!("{}th", lastword)
                }
            }
        };
        let n = lastwords.len();
        lastwords[n - 1] = self.title(&newlast);
        let n2 = outwords.len();
        outwords[n2 - 1] = lastwords.join("-");
        Ok(outwords.join(" "))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        // Python: "%s%s" % (value, self.to_ordinal(value)[-2:]) — last two chars.
        let suffix: String = ord.chars().rev().take(2).collect::<Vec<_>>().into_iter().rev().collect();
        Ok(format!("{}{}", value, suffix))
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float checks and `to_year`'s
    // non-integer-float guard become reachable here — unlike on the BigInt
    // hooks above, where they are dead code. `to_cardinal` needs no
    // override: EN inherits base's `assert int(value) == value` routing,
    // which is exactly the trait default (whole -> int path).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "fifth", Decimal("1E+2") -> "one
    /// hundredth"); fractional or negative values raise TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s" % (value, to_ordinal(value)[-2:])`.
    /// The numeric half is Python's `str(value)` (`repr_str`), so
    /// Decimal("5.00") yields "5.00th", 1.0 yields "1.0st" and -0.0 yields
    /// "-0.0th". An oversized value propagates `to_ordinal`'s OverflowError.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        let ord = self.to_ordinal(&i)?;
        let suffix: String = {
            let chars: Vec<char> = ord.chars().collect();
            let start = chars.len().saturating_sub(2);
            chars[start..].iter().collect()
        };
        Ok(format!("{}{}", repr_str, suffix))
    }

    /// `Num2Word_EN.to_year(float/Decimal)`: non-integer *floats* raise
    /// TypeError (issue #67) — NaN/±inf fail `is_integer()` the same way —
    /// while Decimals of any scale skip the guard (`isinstance(val, float)`
    /// is False) and truncate via `int(val)`, so Decimal("1.5") is year 1.
    /// Whole values continue into the integer `to_year` (BC for negatives).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                if !(f.is_finite() && f.fract() == 0.0) {
                    return Err(N2WError::Type(format!(
                        "to='year' expects an integer; got non-integer float {}",
                        py_float_str(*f)
                    )));
                }
                let i = value
                    .as_whole_int()
                    .ok_or_else(|| N2WError::Type("unreachable: whole float".into()))?;
                self.to_year(&i)
            }
            FloatValue::Decimal { value: d, .. } => {
                // int(Decimal) truncates toward zero.
                let i = d.with_scale(0).as_bigint_and_exponent().0;
                self.to_year(&i)
            }
        }
    }

    /// Port of `Num2Word_EN.to_fraction` (issue #584): idiomatic
    /// "half/halves" and "quarter/quarters" for denominators 2 and 4; every
    /// other denominator is `to_ordinal` + bare "s" ("7/100" -> "seven one
    /// hundredths"). The sign is the literal "minus " prefix from the Python
    /// source; `d == 1` (exactly 1, not -1) or `n == 0` short-circuits to
    /// the signed cardinal, before any sign normalisation.
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
        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "half" } else { "halves" }).to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "quarter" } else { "quarters" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            // 1/100 is "one hundredth", not "one one hundredth" (#217): the
            // ordinal of a power of ten already starts with "one".
            if matches!(abs_d.to_string().strip_prefix('1'), Some(z) if z.bytes().all(|b| b == b'0')) {
                if let Some(rest) = w.strip_prefix("one ") {
                    w = rest.to_string();
                }
            }
            if !abs_n.is_one() {
                w.push('s');
            }
            w
        };
        let sign = if is_negative { "minus " } else { "" };
        Ok(format!("{}{} {}", sign, self.to_cardinal(&abs_n)?, den_word))
    }

    // ---- currency -------------------------------------------------------
    //
    // EN inherits `to_currency`, `to_cheque`, `_money_verbose`,
    // `_cents_verbose` and `_cents_terse` unchanged from `Num2Word_Base`, and
    // `pluralize` from `Num2Word_EUR`. Only the data tables, the class name
    // and that one plural rule are language-specific, so only they are
    // overridden here — the trait defaults already mirror the rest.

    fn lang_name(&self) -> &str {
        "Num2Word_EN"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    fn currency_precision(&self, code: &str) -> i64 {
        // CURRENCY_PRECISION.get(code, 100)
        self.currency_precision.get(code).copied().unwrap_or(100)
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// raises IndexError. Every entry in EN's table has at least two forms, so
    /// this is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.year_with_suffix(value, None)
    }

    /// `Num2Word_EN.to_year(val, suffix=None, longval=True)` with the
    /// caller's kwargs. `longval` is accepted and ignored — the EN body
    /// never reads it. `suffix` is tested truthily twice: a falsy suffix
    /// lets negatives default to "BC", and is never appended, so
    /// `suffix=""` behaves like `suffix=None`. Non-str suffix values are
    /// declined rather than guessing at Python's `str()` interpolation.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let suffix = match kw.get("suffix") {
            None | Some(KwVal::None) => None,
            Some(KwVal::Str(s)) if s.is_empty() => None,
            Some(KwVal::Str(s)) => Some(s.as_str()),
            _ => return Err(N2WError::Fallback("kwargs".into())),
        };
        self.year_with_suffix(value, suffix)
    }
}

