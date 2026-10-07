//! S'gaw Karen (`ksw`), in the S'gaw Karen script (Myanmar block).
//!
//! `ksw` is one of the languages added by this project (upstream `num2words`
//! has no `lang_KSW.py`), so there is no upstream output to stay byte-for-byte
//! with. Python's `lang_KSW.py` spelled every numeral in an ad-hoc Latin
//! transliteration ("lwisi di khi" for 42) and fell back to digits from 10^9.
//! gladiaio/num2words2#143 replaced it with the script spellings that two
//! independent sources agree on, and makes everything else raise rather than
//! invent a word or mix scripts.
//!
//! # Lexicon (sources)
//!
//! Omniglot, *Numbers in Sgaw Karen*
//! (<https://www.omniglot.com/language/numbers/sgawkaren.htm>) and the English
//! Wiktionary category *S'gaw Karen numerals*
//! (<https://en.wiktionary.org/wiki/Category:S%27gaw_Karen_numerals>):
//!
//! | n | word | | n | word |
//! |---|---|---|---|---|
//! | 1 | တ (final: တၢ) | | 10 | တဆံ |
//! | 2 | ခံ | | 11 | တဆံတၢ |
//! | 3 | သၢ | | 12 | တဆံခံ |
//! | 4 | လွံၢ် | | 20 | ခံဆံ |
//! | 5 | ယဲၢ် | | 40 | လွံၢ်ဆံ |
//! | 6 | ဃု | | 100 | တကယၤ |
//! | 7 | နွံ | | 1000 | တကထိ |
//! | 8 | ဃိး | | | |
//! | 9 | ခွံ | | | |
//!
//! Composition, as in Gilmore, *A Grammar of the Sgaw Karen* (1898) §88:
//! place values high to low, each `digit + place word` (the multiplier "one"
//! is တ: တကယၤ, တကထိ; 200 is ခံကယၤ), separated by spaces, tens and
//! units written together (`digit + ဆံ + unit`). A unit "one" after anything
//! else takes the form တၢ, as in 11 တဆံတၢ. Omniglot writes 50 with
//! ဟ although its 5 is ယဲၢ် and its transliteration of 50 is "ye hsee";
//! this module uses the regular ယဲၢ်ဆံ, as Gilmore's 52 does.
//!
//! # What raises, and why
//!
//! No source found gives a S'gaw Karen word for zero, the minus sign, the
//! decimal point, an ordinal form (Gilmore's ordinals need noun classifiers),
//! a currency unit, or a place above 1000 that two sources agree on
//! (Wiktionary lists 10^4 and 10^5 only as classifiers). So:
//!
//! * `abs(n) >= 10^4` -> `OverflowError` (`maxval` is 10^4; #147).
//! * zero, negative numbers, and any float/Decimal whose `str()` has a
//!   decimal point (including `5.0`) -> `NotImplementedError`. An integral
//!   `Decimal("5")` reads like the integer 5.
//! * `to='ordinal'`, `to='ordinal_num'`, `to='currency'`, `to='cheque'`
//!   -> `NotImplementedError`. `to='year'` reads the cardinal (1..9999).
//!
//! A scientific `str(number)` ("1e+16") still raises `ValueError` from
//! `int()`, as before.

use crate::base::{check_maxval, pow10_big, unsupported_mode, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::sync::OnceLock;

/// Units 1..=9 (index 0 unused: there is no verified word for zero).
const ONES: [&str; 10] = [
    "", "တ", "ခံ", "သၢ", "လွံၢ်", "ယဲၢ်", "ဃု", "နွံ", "ဃိး", "ခွံ",
];
/// "One" as a unit after another place, e.g. 11 တဆံတၢ.
const ONE_FINAL: &str = "တၢ";
/// The tens place word: `digit + ဆံ` (20 ခံဆံ; 10 တဆံ).
const TEN: &str = "ဆံ";
/// The hundreds place word: `digit + ကယၤ` (100 တကယၤ).
const HUNDRED: &str = "ကယၤ";
/// The thousands place word: `digit + ကထိ` (1000 တကထိ).
const THOUSAND: &str = "ကထိ";

/// The exclusive ceiling: no place word above 1000 is attested by two
/// sources (gladiaio/num2words2#143, #147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(4))
}

/// `NotImplementedError` for something the sources give no word for. It
/// names the language itself: the binder only adds the `lang='ksw' ` prefix
/// on some paths, and leaves a message that already has it alone.
fn no_word(what: &str) -> N2WError {
    N2WError::NotImplemented(format!(
        "lang='ksw' does not support {}: no verified S'gaw Karen word for it; only whole numbers 1-9999 are supported",
        what
    ))
}

#[derive(Default)]
pub struct LangKsw;

impl LangKsw {
    pub fn new() -> Self {
        LangKsw
    }
}

/// Words for `1 <= n <= 9999` (see the module docs for the composition).
fn words(n: u32) -> String {
    debug_assert!((1..=9999).contains(&n));
    let digit = |d: u32| ONES[d as usize];
    let (th, h, t, o) = (n / 1000, n / 100 % 10, n / 10 % 10, n % 10);
    let mut parts: Vec<String> = Vec::new();
    if th != 0 {
        parts.push(format!("{}{}", digit(th), THOUSAND));
    }
    if h != 0 {
        parts.push(format!("{}{}", digit(h), HUNDRED));
    }
    // A unit "one" after any other place takes its final form.
    let unit = |first: bool| if o == 1 && !first { ONE_FINAL } else { digit(o) };
    if t != 0 {
        let u = if o != 0 { unit(false) } else { "" };
        parts.push(format!("{}{}{}", digit(t), TEN, u));
    } else if o != 0 {
        parts.push(unit(parts.is_empty()).to_string());
    }
    parts.join(" ")
}

/// The integer cardinal: 1..=9999, `OverflowError` beyond, and
/// `NotImplementedError` for zero and negatives.
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    if number.is_zero() {
        return Err(no_word("zero"));
    }
    if number.is_negative() {
        return Err(no_word("negative numbers"));
    }
    Ok(words(number.to_u32().expect("1 <= n < 10^4 after the checks above")))
}

/// Reconstruct Python's `str(f)` (== `repr(f)`) for a finite/`inf`/`nan` f64.
///
/// `precision` is the repr's fractional-digit count, supplied by the binding as
/// `abs(Decimal(str(f)).as_tuple().exponent)`. In the plain-decimal window
/// (`repr` exponent in `-4..=15`) formatting the abs value to that many places
/// reproduces the repr exactly. Outside it, `repr` is scientific and this
/// mirrors CPython's `"1e+16"` / `"1e-05"` shape (two-digit zero-padded
/// exponent) — a form the string algorithm then rejects via `int()`.
fn float_repr(value: f64, precision: u32) -> String {
    let neg = value.is_sign_negative();
    let abs = value.abs();
    let body = if abs.is_finite() && abs != 0.0 {
        // "1.2345e3" / "1e-5" — LowerExp is shortest round-trip, like repr.
        let es = format!("{:e}", abs);
        let epos = es.find('e').expect("LowerExp of a finite nonzero has 'e'");
        let exp: i32 = es[epos + 1..]
            .parse()
            .expect("LowerExp exponent is an integer");
        if exp >= 16 || exp <= -5 {
            let mantissa = &es[..epos];
            let sign = if exp < 0 { "-" } else { "+" };
            format!("{}e{}{:02}", mantissa, sign, exp.abs())
        } else {
            format!("{:.*}", precision as usize, abs)
        }
    } else {
        // 0.0 -> "0.0" (precision is 1); inf/nan -> "inf"/"NaN", both of which
        // have no "." and fail `int()` exactly as Python's do.
        format!("{:.*}", precision as usize, abs)
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Reconstruct Python's `Decimal.__str__` from a `BigDecimal`.
///
/// A faithful port of CPython's `Decimal.__str__` (the `not eng` arm): recovers
/// the `(coefficient, exponent)` pair and places the point per `dotplace`,
/// emitting an `E±d` suffix in the same band Python does. This is what makes a
/// trailing-zero `Decimal("1.10")` render "1.10" (scale is preserved) and a
/// large-scale `Decimal("1E+2")` render "1E+2" (and then raise, no ".").
fn decimal_repr(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    let exp = -scale; // Python `_exp`
    let sign = if int_val.is_negative() { "-" } else { "" };
    let digits = int_val.magnitude().to_string(); // Python `_int`
    let ndigits = digits.len() as i64;
    let leftdigits = exp + ndigits;

    // `if self._exp <= 0 and leftdigits > -6: dotplace = leftdigits` else 1.
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), digits),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", digits, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let dp = dotplace as usize;
        (digits[..dp].to_string(), format!(".{}", &digits[dp..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// `int(s)` for the integer part. Python raises `ValueError` on a non-numeric
/// token (e.g. a scientific mantissa `"1e+16"`), which maps to `N2WError::Value`
/// with Python's exact message text.
fn parse_pyint(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

impl LangKsw {
    /// The cardinal of a float/Decimal, from Python's `str(number)`: a sign or
    /// a decimal point raises (no verified word for either), an integral
    /// `Decimal("5")` reads as the integer, and a scientific token raises
    /// `ValueError` from `int()`.
    fn cardinal_from_str(&self, s: &str) -> Result<String> {
        let s = s.trim();
        if s.starts_with('-') {
            return Err(no_word("negative numbers"));
        }
        if s.contains('.') {
            return Err(no_word("the decimal point"));
        }
        int_to_word(&parse_pyint(s)?)
    }
}

impl Lang for LangKsw {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// Every float/Decimal goes through `str(number)` (see `cardinal_from_str`).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    fn ordinal_float_entry(&self, _value: &FloatValue) -> Result<String> {
        Err(no_word("to='ordinal'"))
    }

    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(no_word("to='ordinal_num'"))
    }

    /// `Decimal("Infinity")` parses, then fails in `int()` as `ValueError`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
    }

    fn default_currency(&self) -> &str {
        "MMK"
    }

    fn default_separator(&self) -> &str {
        " "
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        int_to_word(value)
    }

    /// Ordinals need noun classifiers (Gilmore §96) that no source pins down.
    fn to_ordinal(&self, _value: &BigInt) -> Result<String> {
        Err(no_word("to='ordinal'"))
    }

    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(no_word("to='ordinal_num'"))
    }

    /// The plain cardinal, 1..=9999.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let s = match value {
            FloatValue::Float { value, precision } => float_repr(*value, *precision),
            FloatValue::Decimal { value, .. } => decimal_repr(value),
        };
        self.cardinal_from_str(&s)
    }

    fn lang_name(&self) -> &str {
        "Num2Word_KSW"
    }

    fn currency_forms(&self, _code: &str) -> Option<&CurrencyForms> {
        None
    }

    /// No verified currency unit names (#143).
    fn to_currency(
        &self,
        _val: &CurrencyValue,
        _currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        Err(unsupported_mode("currency"))
    }

    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(unsupported_mode("cheque"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omniglot_forms() {
        let k = LangKsw::new();
        let c = |n: i64| k.to_cardinal(&BigInt::from(n)).unwrap();
        assert_eq!(c(1), "တ");
        assert_eq!(c(10), "တဆံ");
        assert_eq!(c(11), "တဆံတၢ");
        assert_eq!(c(40), "လွံၢ်ဆံ");
        assert_eq!(c(100), "တကယၤ");
        assert_eq!(c(200), "ခံကယၤ");
        assert_eq!(c(1000), "တကထိ");
        assert_eq!(c(9999), "ခွံကထိ ခွံကယၤ ခွံဆံခွံ");
        assert!(matches!(k.to_cardinal(&BigInt::from(10_000)), Err(N2WError::Overflow(_))));
        assert!(matches!(k.to_cardinal(&BigInt::from(0)), Err(N2WError::NotImplemented(_))));
        assert!(matches!(k.to_cardinal(&BigInt::from(-1)), Err(N2WError::NotImplemented(_))));
    }
}
