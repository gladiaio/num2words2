//! Port of `currency.py` plus the currency half of `base.py`.
//!
//! # Why values arrive as strings
//!
//! Python's `parse_currency_parts` does `Decimal(str(value))` — it stringifies
//! the float *first*, so `12.34` becomes the exact decimal 12.34 rather than
//! the binary double 12.3399999...  Reproducing `repr(float)` in Rust would be
//! a second implementation of shortest-round-trip formatting and a permanent
//! source of drift.
//!
//! So the Python shim passes `str(value)` across the boundary and the core
//! parses that. The stringification stays in the one place that already
//! defines it, and the two sides cannot disagree about it.

use crate::base::{Kwargs, Lang, N2WError, Result};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::str::FromStr;

/// A `CURRENCY_FORMS` entry: `(unit_forms, subunit_forms)`.
///
/// Python stores each side as a tuple of 1-3 strings — `("euro", "euros")`,
/// or `("zloty", "zlotys", "zlotu")` for languages with a third plural form.
/// `pluralize` indexes into it, so the arity is load-bearing and a language
/// must keep however many forms Python has.
#[derive(Debug, Clone)]
pub struct CurrencyForms {
    pub unit: Vec<String>,
    pub subunit: Vec<String>,
}

impl CurrencyForms {
    pub fn new(unit: &[&str], subunit: &[&str]) -> Self {
        CurrencyForms {
            unit: unit.iter().map(|s| s.to_string()).collect(),
            subunit: subunit.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// The value passed to `to_currency`, mirroring what Python's dispatcher saw.
///
/// The int/non-int distinction is not cosmetic: `base.to_currency` branches on
/// `isinstance(val, int)` and skips the cents segment entirely for true ints,
/// while a float that happens to be whole (`1.0`) still renders cents.
#[derive(Debug, Clone)]
pub enum CurrencyValue {
    /// Caller passed a Python `int`.
    Int(BigInt),
    /// Caller passed a float/Decimal.
    ///
    /// `has_decimal` mirrors Python's
    /// `isinstance(val, float) or str(val).find(".") != -1` — the guard that
    /// decides whether the cents segment appears at all. It cannot be derived
    /// from the parsed value: `Decimal("5")` and `Decimal("5.00")` are equal
    /// numerically but only the second prints cents, while `1e21` is a float
    /// whose repr has no dot yet still counts as decimal. So the flag is
    /// computed on the Python side, where `str(val)` already lives.
    Decimal {
        value: BigDecimal,
        has_decimal: bool,
        /// `isinstance(val, float)` on the Python side. A handful of
        /// converters (SQ) branch on the *origin type*, not the value:
        /// float 12.34 renders cents, Decimal("12.34") does not. Cannot be
        /// derived from `value`/`has_decimal` — Decimal("5.00") has a dot
        /// yet is not a float.
        is_float: bool,
    },
}

impl CurrencyValue {
    /// `Int` from a decimal string, else `Decimal`. `is_int` and `has_decimal`
    /// come from the Python side because only it can tell `1` from `1.0`, or
    /// `Decimal("5")` from `Decimal("5.00")`.
    pub fn parse(s: &str, is_int: bool, has_decimal: bool, is_float: bool) -> Result<Self> {
        if is_int {
            BigInt::from_str(s)
                .map(CurrencyValue::Int)
                .map_err(|e| N2WError::Value(e.to_string()))
        } else {
            BigDecimal::from_str(s)
                .map(|value| CurrencyValue::Decimal { value, has_decimal, is_float })
                .map_err(|e| N2WError::Value(e.to_string()))
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            CurrencyValue::Int(i) => i.is_negative(),
            CurrencyValue::Decimal { value, .. } => value.is_negative(),
        }
    }
}

/// Python's `prefix_currency(prefix, base)`.
pub fn prefix_currency(prefix: &str, base: &[String]) -> Vec<String> {
    base.iter().map(|i| format!("{} {}", prefix, i)).collect()
}

/// `ROUND_HALF_UP` at `scale` decimal places.
///
/// `BigDecimal`'s own rounding is half-even, so this is open-coded: Python's
/// `quantize(..., rounding=ROUND_HALF_UP)` takes .5 away from zero, which
/// differs from half-even on exact ties (2.675 -> 2.68, not 2.68 vs 2.67).
fn round_half_up(v: &BigDecimal, scale: i64) -> BigDecimal {
    let neg = v.is_negative();
    let abs = v.abs();
    let shift = BigDecimal::from(BigInt::from(10).pow(scale.max(0) as u32));
    let shifted = &abs * &shift;
    // Truncate, then bump when the discarded remainder is >= 0.5.
    let floor = shifted.with_scale(0);
    let frac = &shifted - &floor;
    let half = BigDecimal::from_str("0.5").unwrap();
    let rounded = if frac >= half { floor + BigDecimal::from(1) } else { floor };
    let out = rounded / shift;
    if neg {
        -out
    } else {
        out
    }
}

/// Python's `parse_currency_parts`. Returns `(integer, subunits, negative)`.
///
/// `keep_precision` returns fractional subunits (e.g. 65.3 cents) as a
/// `BigDecimal`; otherwise they are truncated to whole subunits after the
/// ROUND_HALF_UP quantize.
pub fn parse_currency_parts(
    value: &CurrencyValue,
    is_int_with_cents: bool,
    keep_precision: bool,
    divisor: i64,
) -> (BigInt, BigDecimal, bool) {
    match value {
        CurrencyValue::Int(v) => {
            let negative = v.is_negative();
            let v = v.abs();
            if is_int_with_cents && divisor != 0 {
                let d = BigInt::from(divisor);
                (&v / &d, BigDecimal::from(&v % &d), negative)
            } else {
                (v, BigDecimal::zero(), negative)
            }
        }
        CurrencyValue::Decimal { value: v, .. } => {
            let mut v = v.clone();
            if !keep_precision && divisor > 1 {
                // quant = Decimal(1)/Decimal(divisor) — i.e. 100 -> 2 places.
                let scale = (divisor as f64).log10().round() as i64;
                v = round_half_up(&v, scale);
            }
            let negative = v.is_negative();
            let v = v.abs();
            let integer = v.with_scale(0); // divmod(value, 1) -> floor for +ve
            let fraction = &v - &integer;
            let cents = fraction * BigDecimal::from(divisor);
            let cents = if keep_precision {
                cents
            } else {
                cents.with_scale(0) // int(fraction * divisor) truncates
            };
            (integer.as_bigint_and_exponent().0, cents, negative)
        }
    }
}

/// `NotImplementedError` for a currency code the language has no words for.
pub fn unknown_currency<L: Lang + ?Sized>(lang: &L, code: &str) -> N2WError {
    N2WError::NotImplemented(format!(
        "Currency code \"{}\" not implemented for \"{}\"",
        code,
        lang.lang_name()
    ))
}

/// `to_currency[_kw]` with the options every language must honour the same
/// way.
///
/// * **`currency=` (#219).** Many languages override `to_currency` wholesale
///   and read only their own currency, so `currency="GBP"` silently printed
///   the default ("dous euros", "اثنان ريالان"). A different code must give
///   different words: when the output for `currency` is identical to the
///   output for the language's default, the code was ignored and this raises
///   like an unknown code does in `base.to_currency`. Codes whose forms are
///   genuinely the same as the default's (two "dollar" currencies) are exempt.
/// * **`cents=False` (#220)** prints the cents as digits ("two euros, 50
///   cents"). Languages whose own `to_currency` drops the cents or ignores the
///   flag get the digits spliced into their verbose reading in place of the
///   cents numeral; where that numeral cannot be found (an inflected or
///   counter form), this raises rather than change the amount.
#[allow(clippy::too_many_arguments)]
pub fn to_currency_checked<L: Lang + ?Sized>(
    lang: &L,
    val: &CurrencyValue,
    currency: &str,
    cents: bool,
    separator: Option<&str>,
    adjective: bool,
    kw: &Kwargs,
) -> Result<String> {
    let render = |code: &str, cents: bool| {
        if kw.is_empty() {
            lang.to_currency(val, code, cents, separator, adjective)
        } else {
            lang.to_currency_kw(val, code, cents, separator, adjective, kw)
        }
    };
    let out = render(currency, cents)?;
    let default = lang.default_currency();
    if currency != default && !lang.same_currency(currency, default) {
        let same_forms = match (lang.currency_forms(currency), lang.currency_forms(default)) {
            (Some(a), Some(b)) => a.unit == b.unit && a.subunit == b.subunit,
            _ => false,
        };
        if !same_forms && matches!(render(default, cents), Ok(d) if d == out) {
            return Err(unknown_currency(lang, currency));
        }
    }
    if cents {
        return Ok(out);
    }
    terse_cents(lang, val, currency, out, || render(currency, true))
}

/// The cents-as-digits guarantee behind `cents=False` (see
/// [`to_currency_checked`]). `out` is the language's own `cents=False` text.
fn terse_cents<L: Lang + ?Sized>(
    lang: &L,
    val: &CurrencyValue,
    currency: &str,
    out: String,
    verbose: impl FnOnce() -> Result<String>,
) -> Result<String> {
    let divisor = lang.currency_precision(currency);
    if divisor <= 1 || !matches!(val, CurrencyValue::Decimal { .. }) {
        return Ok(out);
    }
    // Fractional cents (1.011) keep base's numeric reading of the fraction.
    if let CurrencyValue::Decimal { value, .. } = val {
        let scaled = value * BigDecimal::from(divisor);
        if &scaled - scaled.with_scale(0) != BigDecimal::zero() {
            return Ok(out);
        }
    }
    let (left, right, _) = parse_currency_parts(val, false, false, divisor);
    let right = right.as_bigint_and_exponent().0;
    if right.is_zero() {
        return Ok(out);
    }
    let digits = [default_cents_terse(&right, divisor), right.to_string()];
    let has_token = |text: &str, tok: &str| {
        text.match_indices(tok).any(|(i, _)| {
            let before = text[..i].chars().next_back();
            let after = text[i + tok.len()..].chars().next();
            !before.is_some_and(|c| c.is_ascii_digit()) && !after.is_some_and(|c| c.is_ascii_digit())
        })
    };
    if digits.iter().any(|d| has_token(&out, d)) {
        return Ok(out);
    }
    let unsupported = || N2WError::NotImplemented("does not support cents=False".into());
    let verbose = verbose()?;
    let word = lang.cents_verbose(&right, currency)?;
    let pos = verbose.rfind(word.as_str()).ok_or_else(unsupported)?;
    // The numeral must stand alone (not "five" inside "twenty-five" in a
    // space-separated script) and follow the whole-unit amount.
    let joins = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() && (c as u32) < 0x0590 || c == '-');
    if joins(verbose[..pos].chars().next_back()) || joins(verbose[pos + word.len()..].chars().next()) {
        return Err(unsupported());
    }
    if let Ok(money) = lang.money_verbose(&left, currency) {
        if let Some(m) = verbose.find(money.as_str()) {
            if pos < m + money.len() {
                return Err(unsupported());
            }
        }
    }
    Ok(format!("{}{}{}", &verbose[..pos], digits[0], &verbose[pos + word.len()..]))
}

/// Python's `Num2Word_Base.to_currency`.
///
/// Kept as a free function taking `&dyn Lang` so a language can override
/// `to_currency` wholesale (134 of them do) while still delegating here.
pub fn default_to_currency<L: Lang + ?Sized>(
    lang: &L,
    val: &CurrencyValue,
    currency: &str,
    cents: bool,
    separator: &str,
    adjective: bool,
) -> Result<String> {
    let divisor = lang.currency_precision(currency);

    // Zero-decimal currencies (JPY, KRW): any fractional input rounds to a
    // whole unit and the cents segment is skipped entirely.
    let val = if divisor == 1 {
        match val {
            CurrencyValue::Decimal { value, .. } => {
                CurrencyValue::Int(round_half_up(value, 0).as_bigint_and_exponent().0)
            }
            other => other.clone(),
        }
    } else {
        val.clone()
    };

    let forms = lang
        .currency_forms(currency)
        .ok_or_else(|| unknown_currency(lang, currency))?;

    let mut cr1 = forms.unit.clone();
    let cr2 = forms.subunit.clone();
    if adjective {
        if let Some(adj) = lang.currency_adjective(currency) {
            cr1 = prefix_currency(adj, &cr1);
        }
    }

    // Pure ints never show cents.
    if let CurrencyValue::Int(v) = &val {
        let minus = if v.is_negative() {
            format!("{} ", lang.negword().trim())
        } else {
            String::new()
        };
        let abs = v.abs();
        let money = lang.money_verbose(&abs, currency)?;
        return Ok(format!(
            "{}{} {}",
            minus,
            money,
            lang.pluralize(&abs, &cr1)?
        ));
    }

    let (d, has_decimal, is_float) = match &val {
        CurrencyValue::Decimal { value, has_decimal, is_float } => (value.clone(), *has_decimal, *is_float),
        _ => unreachable!(),
    };
    // has_fractional_cents: (value * divisor) % 1 != 0
    let scaled = &d * BigDecimal::from(divisor);
    let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

    let (left, right, is_negative) = parse_currency_parts(
        &CurrencyValue::Decimal { value: d, has_decimal, is_float },
        false,
        has_fractional_cents,
        divisor,
    );

    let minus = if is_negative {
        format!("{} ", lang.negword().trim())
    } else {
        String::new()
    };
    let money = lang.money_verbose(&left, currency)?;
    let right_int = right.as_bigint_and_exponent().0;

    // Python: `if has_decimal or right > 0: <with cents> else: <no cents>`.
    // Dropping this guard makes `Decimal("5")` render "five dollars, zero
    // cents" where Python says "five dollars".
    if !has_decimal && right_int.is_zero() {
        return Ok(format!(
            "{}{} {}",
            minus,
            money,
            lang.pluralize(&left, &cr1)?
        ));
    }
    if has_fractional_cents {
        // Python takes `cr2[1] if len(cr2) > 1 else cr2` here — the plural
        // form unconditionally, without consulting pluralize(). At 1.011 USD
        // that is "one point one cents", not "...cent".
        let sub = cr2.get(1).or_else(|| cr2.first()).cloned().unwrap_or_default();
        return Ok(format!(
            "{}{} {}{} {} {}",
            minus,
            money,
            lang.pluralize(&left, &cr1)?,
            separator,
            lang.cardinal_from_decimal(&right)?,
            sub
        ));
    }
    let cents_str = if cents {
        lang.cents_verbose(&right_int, currency)?
    } else {
        lang.cents_terse(&right_int, currency)?
    };

    Ok(format!(
        "{}{} {}{} {} {}",
        minus,
        money,
        lang.pluralize(&left, &cr1)?,
        separator,
        cents_str,
        lang.pluralize(&right_int, &cr2)?
    ))
}

/// Python's `Num2Word_Base._cents_terse`.
pub fn default_cents_terse(n: &BigInt, divisor: i64) -> String {
    if divisor <= 1 {
        return n.to_string();
    }
    let width = divisor.to_string().len() - 1;
    crate::strnum::zero_pad_left(&n.to_string(), width)
}

/// Python's `Num2Word_Base.to_cheque`.
pub fn default_to_cheque<L: Lang + ?Sized>(
    lang: &L,
    val: &BigDecimal,
    currency: &str,
) -> Result<String> {
    // Cheque convention always takes the plural form.
    cheque_with_unit(lang, val, currency, |_, forms| {
        Ok(forms.unit.last().cloned().unwrap_or_default())
    })
}

/// [`default_to_cheque`] with the unit word chosen by `unit` from the whole
/// amount and the currency's forms — for a language whose cheque noun agrees
/// with the count (is: "EIN … KRÓNA", "TVÆR … KRÓNUR"; #197).
pub fn cheque_with_unit<L: Lang + ?Sized>(
    lang: &L,
    val: &BigDecimal,
    currency: &str,
    unit: impl Fn(&BigInt, &CurrencyForms) -> Result<String>,
) -> Result<String> {
    let forms = lang
        .currency_forms(currency)
        .ok_or_else(|| unknown_currency(lang, currency))?;

    let divisor = lang.currency_precision(currency);
    let is_negative = val.is_negative();
    let abs_val = val.abs();
    let whole = abs_val.with_scale(0).as_bigint_and_exponent().0;

    let fraction_str = if divisor > 1 {
        let sub = (&abs_val - BigDecimal::from(whole.clone())) * BigDecimal::from(divisor);
        let sub = sub.with_scale(0).as_bigint_and_exponent().0;
        let digits = divisor.to_string().len() - 1;
        format!("{}/{}", crate::strnum::zero_pad_left(&sub.to_string(), digits), divisor)
    } else {
        String::new()
    };

    let words = lang.money_verbose(&whole, currency)?;
    let unit = unit(&whole, forms)?;
    let sign = if is_negative { "MINUS " } else { "" };
    let body = if fraction_str.is_empty() {
        format!("{} {}", words, unit)
    } else {
        format!("{} AND {} {}", words, fraction_str, unit)
    };
    Ok(format!("{}{}", sign, body).to_uppercase())
}
