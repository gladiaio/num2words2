//! Port of `lang_EN_NE.py` — English as used in Nepal.
//!
//! `Num2Word_EN_NE` subclasses `Num2Word_EN` and overrides exactly one method:
//! `set_high_numwords`. Everything else — `setup`, `merge`, `to_ordinal`,
//! `to_ordinal_num`, `to_year` — is inherited verbatim from `Num2Word_EN`, so
//! this file reproduces those alongside the replaced card table.
//!
//! The override *ignores its `high` argument entirely*:
//!
//! ```python
//! def set_high_numwords(self, high):
//!     self.cards[10 ** 17] = "shankha"
//!     ...
//! ```
//!
//! `high` is still computed by `Num2Word_EUR.setup` (the Latin illion stems:
//! "cent", "novemnonagint", ... ), and `Num2Word_Base.set_numwords` still
//! passes it in — it is simply dropped on the floor. That makes EUR's
//! `gen_high_numwords` dead code for this language, so it is not ported here.
//! The practical consequence: **EN_NE has no million/billion/trillion cards**.
//! 10^6 is rendered on the South-Asian scale as "ten lakh", not "one million".
//!
//! Card table, in `OrderedDict` insertion order (which is descending, so the
//! sorted `Cards` vec in `base.rs` reproduces `splitnum`'s iteration exactly):
//!
//! | value | word    |
//! |-------|---------|
//! | 10^17 | shankha |
//! | 10^15 | padam   |
//! | 10^13 | neel    |
//! | 10^11 | kharba  |
//! | 10^9  | arba    |
//! | 10^7  | crore   |
//! | 10^5  | lakh    |
//!
//! then EN's mid (1000 thousand, 100 hundred, 90..30) and low (20..0).
//!
//! `MAXVAL = 1000 * list(self.cards.keys())[0]` = 1000 * 10^17 = **10^20**,
//! far below plain EN's 10^306. `to_cardinal(10**20)` is an OverflowError.
//!
//! Quirks preserved from the Python (do not "fix" these):
//!
//! - The inherited `ords` dict still maps "million" -> "millionth" and
//!   "billion" -> "billionth". Those words can never appear in EN_NE cardinal
//!   output because the cards were replaced, so the entries are dead — but
//!   they are kept verbatim rather than pruned.
//! - The high numwords have no ordinal entries, so `to_ordinal` falls through
//!   to the generic "+th" suffix rule: "one lakhth", "one arbath",
//!   "ten shankhath", "one croreth". That is what Python emits.
//! - `to_ordinal(100001)` -> "one lakh and first": the EN algorithm only
//!   ordinalises the final space-separated word, and the cardinal ends in
//!   "and one".

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

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

pub struct LangEnNe {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS` — built once here, never per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `CURRENCY_ADJECTIVES` — inherited from `Num2Word_EUR` untouched.
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEnNe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnNe {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_EN_NE.set_high_numwords — the `high` argument is ignored.
        let ten = BigInt::from(10u8);
        for (exp, word) in [
            (17u32, "shankha"),
            (15, "padam"),
            (13, "neel"),
            (11, "kharba"),
            (9, "arba"),
            (7, "crore"),
            (5, "lakh"),
        ] {
            cards.insert(ten.pow(exp), word);
        }

        // Inherited from Num2Word_EN.setup.
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

        // MAXVAL = 1000 * highest card = 1000 * 10^17 = 10^20.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Inherited from Num2Word_EN.setup, verbatim — including the
        // million/billion entries that EN_NE's card table makes unreachable.
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

        LangEnNe {
            cards,
            maxval,
            ords,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
            currency_forms: Self::build_currency_forms(),
            currency_adjectives: Self::build_currency_adjectives(),
        }
    }

    /// The effective `CURRENCY_FORMS` for `Num2Word_EN_NE`.
    ///
    /// Layered exactly as Python builds it: the `Num2Word_EUR` class-attribute
    /// table, then the reassignments `Num2Word_EN.__init__` performs on top.
    ///
    /// **Python quirk (load-bearing).** `Num2Word_EN` declares no class-level
    /// `CURRENCY_FORMS`, so `self.CURRENCY_FORMS["EUR"] = ...` in its
    /// `__init__` resolves to — and mutates **in place** — the dict object on
    /// `Num2Word_EUR`. Verified against the interpreter:
    /// `Num2Word_EN_NE().CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` -> True.
    /// It is a shared-mutable-class-attribute bug, but it is deterministic
    /// here: `CONVERTER_CLASSES` builds one instance per class at import time
    /// and `Num2Word_EN.__init__` is the only writer to that dict (`HU`/`SV`
    /// are the other EUR subclasses without their own table, and they only
    /// read). So the merged result below is stable regardless of import order.
    ///
    /// Built once per `LangEnNe`; `currency_forms()` only ever borrows.
    fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
        // ---- Num2Word_EUR.CURRENCY_FORMS (class attribute) ----
        let mut m: HashMap<&'static str, CurrencyForms> = [
            ("AUD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("BYN", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"])),
            ("CAD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("EEK", CurrencyForms::new(&["kroon", "kroons"], &["sent", "senti"])),
            ("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cents"])),
            (
                "GBP",
                CurrencyForms::new(&["pound sterling", "pounds sterling"], &["penny", "pence"]),
            ),
            ("LTL", CurrencyForms::new(&["litas", "litas"], &["cent", "cents"])),
            ("LVL", CurrencyForms::new(&["lat", "lats"], &["santim", "santims"])),
            ("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("RUB", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"])),
            ("SEK", CurrencyForms::new(&["krona", "kronor"], &["öre", "öre"])),
            ("NOK", CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"])),
            // Three unit forms; `pluralize` only ever indexes 0 or 1, but the
            // arity is preserved verbatim rather than trimmed.
            ("PLN", CurrencyForms::new(&["zloty", "zlotys", "zlotu"], &["grosz", "groszy"])),
            ("MXN", CurrencyForms::new(&["peso", "pesos"], &["cent", "cents"])),
            (
                "RON",
                CurrencyForms::new(&["leu", "lei", "de lei"], &["ban", "bani", "de bani"]),
            ),
            ("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"])),
            ("HUF", CurrencyForms::new(&["forint", "forint"], &["fillér", "fillér"])),
            ("ISK", CurrencyForms::new(&["króna", "krónur"], &["aur", "aurar"])),
            ("UZS", CurrencyForms::new(&["sum", "sums"], &["tiyin", "tiyins"])),
            (
                "SAR",
                CurrencyForms::new(&["saudi riyal", "saudi riyals"], &["halalah", "halalas"]),
            ),
            ("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"])),
            ("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"])),
        ]
        .into_iter()
        .collect();

        // ---- Num2Word_EN.__init__ overrides, applied in source order ----
        // English pluralisation: EUR's ("euro","euro") becomes ("euro","euros"),
        // GBP loses "sterling", SAR loses "saudi".
        for (code, forms) in [
            ("EUR", CurrencyForms::new(&["euro", "euros"], &["cent", "cents"])),
            ("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("GBP", CurrencyForms::new(&["pound", "pounds"], &["penny", "pence"])),
            ("NGN", CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"])),
            // Common ISO 4217 codes that downstream users hit (#74).
            ("AUD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("CAD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("NZD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("HKD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("SGD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"])),
            ("CHF", CurrencyForms::new(&["franc", "francs"], &["rappen", "rappen"])),
            ("AED", CurrencyForms::new(&["dirham", "dirhams"], &["fils", "fils"])),
            ("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"])),
            ("CNY", CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"])),
            ("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"])),
            ("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"])),
            ("MXN", CurrencyForms::new(&["peso", "pesos"], &["cent", "cents"])),
            ("BRL", CurrencyForms::new(&["real", "reais"], &["cent", "cents"])),
            ("ZAR", CurrencyForms::new(&["rand", "rand"], &["cent", "cents"])),
            ("SAR", CurrencyForms::new(&["riyal", "riyals"], &["halalah", "halalas"])),
            ("QAR", CurrencyForms::new(&["riyal", "riyals"], &["dirham", "dirhams"])),
            ("KWD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"])),
            // 3-decimal currencies (mils as subunit), issue #256.
            ("BHD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"])),
            ("OMR", CurrencyForms::new(&["rial", "rials"], &["baisa", "baisa"])),
            ("JOD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"])),
            ("TND", CurrencyForms::new(&["dinar", "dinars"], &["millime", "millimes"])),
            ("LYD", CurrencyForms::new(&["dinar", "dinars"], &["dirham", "dirhams"])),
            ("IQD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"])),
        ] {
            m.insert(code, forms);
        }
        m
    }

    /// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — `Num2Word_EN`
    /// never touches it, so codes EN added (CHF, CNY, KWD, ...) have no
    /// adjective and `adjective=True` silently leaves them unprefixed, exactly
    /// as Python's `if currency in self.CURRENCY_ADJECTIVES` guard does.
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

    /// `Num2Word_Base.verify_ordinal`. Integer input can never trip the
    /// float branch, so only the negative check is reachable.
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
}

impl Lang for LangEnNe {
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

    /// `Num2Word_EN.merge`, inherited unchanged.
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

    /// `Num2Word_EN.to_ordinal`, inherited unchanged.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        let last = outwords.last().unwrap().clone();
        let mut lastwords: Vec<String> = last.split('-').map(|s| s.to_string()).collect();
        let lastword = lastwords.last().unwrap().to_lowercase();

        // Python: try self.ords[lastword] / except KeyError: y -> ie, then +th.
        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => o.to_string(),
            None => {
                if lastword.ends_with('y') {
                    // lastword[:-1] + "ie" + "th"
                    let mut chars = lastword.chars().collect::<Vec<_>>();
                    chars.pop();
                    format!("{}ieth", chars.into_iter().collect::<String>())
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

    /// `Num2Word_EN.to_ordinal_num`: "%s%s" % (value, self.to_ordinal(value)[-2:]).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        let suffix: String = {
            let chars: Vec<char> = ord.chars().collect();
            let start = chars.len().saturating_sub(2);
            chars[start..].iter().collect()
        };
        Ok(format!("{}{}", value, suffix))
    }

    /// `Num2Word_EN.to_year`. Integer input, so the non-integer-float
    /// TypeError guard is unreachable; `suffix` is always None on entry.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut val = value.clone();
        let mut suffix: Option<&str> = None;
        if val.sign() == num_bigint::Sign::Minus {
            val = -val;
            suffix = Some("BC");
        }
        let hundred = BigInt::from(100);
        let high = &val / &hundred;
        let low = &val % &hundred;

        // 00XX, X00X, or beyond 9999 fall back to plain cardinal.
        let ten = BigInt::from(10);
        let valtext = if high.is_zero() || ((&high % &ten).is_zero() && low < ten) || high >= hundred
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

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float checks and `to_year`'s
    // non-integer-float guard become reachable here — unlike on the BigInt
    // hooks above, where they are dead code. `to_cardinal` needs no
    // override: EN_NE inherits base's `assert int(value) == value` routing,
    // which is exactly the trait default (whole -> int path).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "fifth", 1e+16 -> "ten padamth");
    /// fractional or negative values raise TypeError. 1e+20 passes the
    /// verify but overflows EN_NE's 10^20 MAXVAL inside `to_cardinal`,
    /// surfacing the OverflowError Python raises.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s" % (value, to_ordinal(value)[-2:])`.
    /// The numeric half is Python's `str(value)` (`repr_str`), so
    /// Decimal("5.00") yields "5.00th", 1.0 yields "1.0st" and -0.0 yields
    /// "-0.0th". 1e+20 propagates `to_ordinal`'s OverflowError (MAXVAL 10^20).
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

    /// `Num2Word_EN.to_year(float/Decimal)`, inherited unchanged by EN_NE:
    /// non-integer *floats* raise TypeError (issue #67) — NaN/±inf fail
    /// `is_integer()` the same way — while Decimals of any scale skip the
    /// guard (`isinstance(val, float)` is False) and truncate via `int(val)`,
    /// so Decimal("1.5") is year 1. Whole values continue into the integer
    /// `to_year` (BC for negatives).
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

    /// Port of `Num2Word_EN.to_fraction` (issue #584), inherited unchanged
    /// by EN_NE: idiomatic "half/halves" and "quarter/quarters" for
    /// denominators 2 and 4; every other denominator is `to_ordinal` + bare
    /// "s" ("7/100" -> "seven one hundredths"). The sign is the literal
    /// "minus " prefix from the Python source; `d == 1` (exactly 1, not -1)
    /// or `n == 0` short-circuits to the signed cardinal, before any sign
    /// normalisation.
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
    // `Num2Word_EN_NE` defines no currency behaviour of its own; everything
    // below is inherited from `Num2Word_EN` / `Num2Word_EUR` / `Num2Word_Base`.
    //
    // Not overridden, because the base.rs defaults already match Python:
    //   _money_verbose / _cents_verbose -> to_cardinal          (Num2Word_Base)
    //   _cents_terse    -> default_cents_terse(n, precision)    (Num2Word_Base)
    //   to_currency     -> default_to_currency                  (Num2Word_Base)
    //   to_cheque       -> default_to_cheque                    (Num2Word_Base)
    //
    // Because `_money_verbose` routes to this language's `to_cardinal`, money
    // amounts render on the South-Asian scale: 1000000 EUR is "ten lakh euros",
    // not "one million euros".

    /// `self.__class__.__name__`, for the NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_EN_NE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `self.CURRENCY_PRECISION.get(code, 100)`.
    ///
    /// `Num2Word_EN.__init__` *reassigns* `self.CURRENCY_PRECISION` rather than
    /// updating it, creating an instance attribute that shadows
    /// `Num2Word_Base`'s empty class dict — so this table is exactly the seven
    /// 3-decimal (mil) currencies and nothing else.
    ///
    /// **JPY and KRW are deliberately absent** and therefore fall back to 100,
    /// despite being real-world zero-decimal currencies: Python's comment says
    /// their historical sen/jeon subunits are still expected by the fixtures.
    /// The corpus agrees — `currency:JPY 12.34` is "twelve yen, thirty-four
    /// sen", and `cheque:JPY` renders "56/100". A consequence is that the
    /// `divisor == 1` zero-decimal branch in `default_to_currency` is
    /// unreachable for this language.
    fn currency_precision(&self, code: &str) -> i64 {
        match code {
            "BHD" | "KWD" | "OMR" | "JOD" | "TND" | "LYD" | "IQD" => 1000,
            _ => 100,
        }
    }

    /// `Num2Word_EUR.pluralize`: `form = 0 if n == 1 else 1; return forms[form]`.
    ///
    /// Callers always pass a non-negative `n` (the base path abs()-es first).
    /// The out-of-range arm mirrors Python indexing a 1-element tuple with `1`
    /// (`IndexError`); every entry in this language's table carries at least
    /// two forms, so it is unreachable — it exists to keep the exception *type*
    /// honest rather than to paper over a missing form with a silent fallback.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
