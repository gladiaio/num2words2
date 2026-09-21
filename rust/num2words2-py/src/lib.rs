//! PyO3 bindings for the Rust num2words2 core.
//!
//! Hand-maintained (formerly generated): all per-language resolution lives
//! in the core's `get_lang_by_key`, so this file only exposes entry points.
//!
//! Language instances are built once and cached core-side: constructing a
//! language generates its full card table (up to 10^303 for en), which costs
//! far more than a single conversion.

mod sentencepath;

use bigdecimal::BigDecimal;
use num2words2_core::base::{Kwargs, KwVal, Lang};
use num2words2_core::presentation::{self, CentsArg};
use num2words2_core::strnum::{
    has_py_digit, python_decimal_str, python_int_parse, ParsedNumber,
};
use num2words2_core::N2WError;
use num2words2_core::{CurrencyValue, FloatValue};
use num_bigint::BigInt;
use pyo3::exceptions::{
    PyAssertionError, PyAttributeError, PyIndexError, PyKeyError, PyNotImplementedError,
    PyOverflowError, PyRuntimeError, PyTypeError, PyValueError, PyZeroDivisionError,
};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyStringMethods, PyTuple};
use std::str::FromStr;

/// Modes that map to a value-type entry point (`_RUST_TYPES` in the old shim).
const RUST_TYPES: [&str; 4] = ["cardinal", "ordinal", "ordinal_num", "year"];
/// Every `to=` the dispatcher accepts (`CONVERTES_TYPES` in the old shim).
const CONVERTER_TYPES: [&str; 7] = [
    "cardinal",
    "ordinal",
    "ordinal_num",
    "year",
    "currency",
    "cheque",
    "fraction",
];

/// The empty-message `NotImplementedError` the old Python shim raised
/// (`raise NotImplementedError()`), reproduced exactly.
fn not_implemented() -> PyErr {
    PyNotImplementedError::new_err("")
}

// The Rust-core-declines signal. NOT a NotImplementedError subclass: the
// shim catches THIS to fall back to the original Python converter, while a
// genuine NotImplementedError (Welsh >100, unknown Japanese counter) is left
// to propagate natively. Making it a subclass would put us back where we
// started — `except NotImplementedError` would swallow the genuine raise too.
pyo3::create_exception!(_rust, RustFallback, pyo3::exceptions::PyException);

// `NumberTooLargeError` was defined in `num2words2/lang_BN.py`, which the
// pure binder no longer ships. Define it natively so bn's past-MAX_NUMBER
// raise keeps its exception type (`except NumberTooLargeError` / a
// `type(e).__name__` check) instead of degrading to ModuleNotFoundError when
// the Custom arm tries to import the deleted module.
pyo3::create_exception!(_rust, NumberTooLargeError, pyo3::exceptions::PyException);

fn map_err(e: N2WError) -> PyErr {
    match e {
        N2WError::Overflow(m) => PyOverflowError::new_err(m),
        N2WError::Type(m) => PyTypeError::new_err(m),
        N2WError::NotImplemented(m) => PyNotImplementedError::new_err(m),
        // Declined — the shim's `except _rust.RustFallback` re-runs Python.
        N2WError::Fallback(m) => RustFallback::new_err(m),
        N2WError::ZeroDivision(m) => PyZeroDivisionError::new_err(m),
        // These mirror crashes in the Python original, not deliberate
        // raises. Reproducing the exception type is required for parity.
        N2WError::Index(m) => PyIndexError::new_err(m),
        N2WError::Key(m) => PyKeyError::new_err(m),
        N2WError::Value(m) => PyValueError::new_err(m),
        N2WError::Attribute(m) => PyAttributeError::new_err(m),
        N2WError::Assertion(m) => PyAssertionError::new_err(m),
        // Intercepted by every entry point before reaching here; this arm
        // exists only for exhaustiveness.
        N2WError::ReturnsNone => PyRuntimeError::new_err(
            "internal: ReturnsNone must be handled by the caller",
        ),
        // A language (or the decimal module) defines the exception class:
        // import and raise the real thing so `except That` keeps working.
        // num2words2's own exception classes lived in the pure-Python lang
        // modules, which the binder no longer ships; raise the natively-defined
        // equivalent so the exception type stays correct. Classes from
        // importable modules (e.g. decimal.InvalidOperation) still import.
        N2WError::Custom { module, class, msg }
            if module.starts_with("num2words2") && class == "NumberTooLargeError" =>
        {
            NumberTooLargeError::new_err(msg)
        }
        N2WError::Custom { module, class, msg } => Python::attach(|py| {
            match py
                .import(module)
                .and_then(|m| m.getattr(class))
                .and_then(|c| c.call1((msg.clone(),)))
            {
                Ok(inst) => PyErr::from_value(inst),
                // If the class can't be imported, surface that rather than
                // silently degrading to a different exception type.
                Err(e) => e,
            }
        }),
    }
}

/// Resolve a language code to a cached core implementation.
/// `None` means the Rust core does not implement it — the Python side then
/// falls back to its original converter.
fn get_lang(lang: &str) -> Option<&'static (dyn Lang + Sync)> {
    num2words2_core::get_lang_by_key(lang)
}

fn need_lang(lang: &str) -> PyResult<&'static (dyn Lang + Sync)> {
    get_lang(lang).ok_or_else(|| PyNotImplementedError::new_err(lang.to_string()))
}

/// A kwarg value as the shim passes it. Bool must precede Int: Python bools
/// extract as ints too, and `plural=True` arriving as `Int(1)` would change
/// which trait-hook branch fires.
#[derive(FromPyObject)]
enum PyKw {
    Bool(bool),
    Int(i64),
    Str(String),
    List(Vec<String>),
}

type PyKwargs = Vec<(String, Option<PyKw>)>;

fn kwbag(kwargs: PyKwargs) -> Kwargs {
    Kwargs(
        kwargs
            .into_iter()
            .map(|(k, v)| {
                let v = match v {
                    None => KwVal::None,
                    Some(PyKw::Bool(b)) => KwVal::Bool(b),
                    Some(PyKw::Int(i)) => KwVal::Int(i),
                    Some(PyKw::Str(s)) => KwVal::Str(s),
                    Some(PyKw::List(l)) => KwVal::List(l),
                };
                (k, v)
            })
            .collect(),
    )
}

/// lang_VI's bare-`None` return becomes `Ok(None)`; every other error stays an
/// error. Kept in `N2WError` space so the caller decides how to map it (the
/// unified `num2words` entry turns `Fallback` into `NotImplementedError`, while
/// the standalone entry points surface it as `RustFallback`).
fn opt(r: Result<String, N2WError>) -> Result<Option<String>, N2WError> {
    match r {
        Ok(s) => Ok(Some(s)),
        Err(N2WError::ReturnsNone) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Unwrap a conversion result the way every entry point must: lang_VI's
/// bare-`None` return becomes Python None, everything else maps through.
fn finish(r: Result<String, N2WError>) -> PyResult<Option<String>> {
    opt(r).map_err(map_err)
}

#[pyfunction]
fn supported_langs() -> Vec<&'static str> {
    num2words2_core::supported_lang_keys()
}

#[pyfunction]
fn to_cardinal(lang: &str, value: BigInt) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_cardinal(&value))
}

#[pyfunction]
fn to_ordinal(lang: &str, value: BigInt) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_ordinal(&value))
}

#[pyfunction]
fn to_ordinal_num(lang: &str, value: BigInt) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_ordinal_num(&value))
}

#[pyfunction]
fn to_year(lang: &str, value: BigInt) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_year(&value))
}

#[pyfunction]
fn to_fraction(lang: &str, numerator: BigInt, denominator: BigInt) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_fraction(&numerator, &denominator))
}

/// `value` is `str(val)` from the Python side and `is_int` says whether the
/// caller passed a true `int`. Python's parse_currency_parts does
/// `Decimal(str(value))`, so stringifying there and parsing here reproduces it
/// exactly — and keeps repr(float) as Python's problem, not ours.
#[pyfunction]
#[pyo3(signature = (lang, value, is_int, has_decimal, is_float, currency, cents, separator, adjective))]
fn to_currency(
    lang: &str,
    value: &str,
    is_int: bool,
    has_decimal: bool,
    is_float: bool,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
) -> PyResult<Option<String>> {
    let l = need_lang(lang)?;
    let v = CurrencyValue::parse(value, is_int, has_decimal, is_float).map_err(map_err)?;
    // None => caller omitted the kwarg; the language's own default applies
    // (Mongolian's adjective is True, en_IN's currency is INR).
    let adjective = adjective.unwrap_or(l.default_adjective());
    let currency = currency.unwrap_or(l.default_currency());
    finish(l.to_currency(&v, currency, cents, separator, adjective))
}

/// The float/Decimal cardinal path (legacy fractional-only entry; `to_float`
/// below is the full router). `value` is the raw f64 — Python floats and
/// Rust f64 are both IEEE-754 doubles, so the binary artefacts base.py's
/// float2tuple depends on survive the crossing. `decimal_str` is non-empty
/// only when the caller passed a Decimal, which takes the exact
/// arbitrary-precision arm instead.
#[pyfunction]
#[pyo3(signature = (lang, value, precision, decimal_str, precision_override))]
fn to_cardinal_float(
    lang: &str,
    value: f64,
    precision: u32,
    decimal_str: &str,
    precision_override: Option<u32>,
) -> PyResult<Option<String>> {
    let l = need_lang(lang)?;
    let v = float_value(value, precision, decimal_str).map_err(map_err)?;
    finish(l.cardinal_float_entry(&v, precision_override))
}

/// The language's raw float grammar, bypassing the whole-value routing —
/// used by the classification harness to test routing hypotheses per
/// language against the corpus.
#[pyfunction]
#[pyo3(signature = (lang, value, precision, decimal_str, precision_override))]
fn to_cardinal_float_raw(
    lang: &str,
    value: f64,
    precision: u32,
    decimal_str: &str,
    precision_override: Option<u32>,
) -> PyResult<Option<String>> {
    let l = need_lang(lang)?;
    let v = float_value(value, precision, decimal_str).map_err(map_err)?;
    finish(l.to_cardinal_float(&v, precision_override))
}

fn float_value(value: f64, precision: u32, decimal_str: &str) -> Result<FloatValue, N2WError> {
    if decimal_str.is_empty() {
        Ok(FloatValue::Float { value, precision })
    } else {
        let d = BigDecimal::from_str(decimal_str).map_err(|e| N2WError::Value(e.to_string()))?;
        use bigdecimal::num_traits::Zero;
        // BigDecimal cannot carry Decimal("-0.0")'s sign. For zero, both
        // float2tuple arms produce pre=0/post=0, so demoting to the float
        // arm with a signed zero is behaviourally identical — except in the
        // languages that render the two differently, which declare it and
        // get the Python fallback (byte-correct by construction).
        // Neg-zero Decimal is handled by the caller (which knows `to`) via
        // `neg_zero_decimal`; here it simply demotes to a signed-zero float,
        // exact wherever the language does not distinguish the two.
        if d.is_zero() && decimal_str.trim_start().starts_with('-') {
            return Ok(FloatValue::Float { value: -0.0, precision });
        }
        Ok(FloatValue::Decimal { value: d, precision })
    }
}

/// `Decimal('-0.0')`: a zero-valued decimal string with a leading minus.
fn is_neg_zero_decimal(decimal_str: &str, value: f64) -> bool {
    !decimal_str.is_empty() && value == 0.0 && decimal_str.trim_start().starts_with('-')
}

/// Float/Decimal input across all four int modes, kwargs included.
/// `repr_str` is Python's `str(number)` — base's to_ordinal_num returns the
/// value unchanged and the dispatcher str()s it, which Rust cannot recompute
/// (repr(float) is shortest-round-trip; str(Decimal) has its own spec).
#[pyfunction]
#[pyo3(signature = (lang, to, value, precision, decimal_str, repr_str, precision_override, kwargs))]
#[allow(clippy::too_many_arguments)]
fn to_float(
    lang: &str,
    to: &str,
    value: f64,
    precision: u32,
    decimal_str: &str,
    repr_str: &str,
    precision_override: Option<u32>,
    kwargs: PyKwargs,
) -> PyResult<Option<String>> {
    let l = need_lang(lang)?;
    to_float_core(
        l,
        to,
        value,
        precision,
        decimal_str,
        repr_str,
        precision_override,
        &kwbag(kwargs),
    )
    .map_err(map_err)
}

/// The float/Decimal router, factored so both the `to_float` entry point and
/// the unified `num2words` dispatcher share one implementation.
#[allow(clippy::too_many_arguments)]
fn to_float_core(
    l: &'static (dyn Lang + Sync),
    to: &str,
    value: f64,
    precision: u32,
    decimal_str: &str,
    repr_str: &str,
    precision_override: Option<u32>,
    kw: &Kwargs,
) -> Result<Option<String>, N2WError> {
    // Decimal('-0.0') the language renders specially (BigDecimal can't hold
    // the sign) — serve it natively before the demotion to Float{-0.0}.
    if is_neg_zero_decimal(decimal_str, value) && kw.is_empty() {
        if let Some(res) = l.neg_zero_decimal(to) {
            return opt(res);
        }
    }
    let v = float_value(value, precision, decimal_str)?;
    let r = match to {
        "cardinal" => {
            if kw.is_empty() {
                l.cardinal_float_entry(&v, precision_override)
            } else {
                l.to_cardinal_float_kw(&v, precision_override, kw)
            }
        }
        // kwargs on the non-cardinal float modes stay on the Python side.
        _ if !kw.is_empty() => Err(N2WError::Fallback("kwargs".into())),
        "ordinal" => l.ordinal_float_entry(&v),
        // PR savoirfairelinux/num2words#666: an integer-valued float (1.0,
        // 2.0, 21.0) is a valid ordinal and must format without the decimal
        // point ("1st", not "1.0st"). Mirror the dispatcher's int-conversion:
        // route a whole float through the integer to_ordinal_num, which the
        // corpus already covers. The dispatcher guards on `isinstance(number,
        // float)`, so this must NOT fire for Decimal input — including
        // Decimal('-0.0'), which the binder demotes to Float{-0.0}. A non-empty
        // `decimal_str` marks a Decimal; gate on it so Decimals keep their
        // scale ("5.00th", "-0.0ste") via the unchanged entry.
        "ordinal_num" => match (decimal_str.is_empty(), v.whole_float_int()) {
            (true, Some(n)) => l.to_ordinal_num(&n),
            _ => l.ordinal_num_float_entry(&v, repr_str),
        },
        "year" => l.year_float_entry(&v),
        other => Err(N2WError::Fallback(other.to_string())),
    };
    opt(r)
}

#[pyfunction]
fn to_cardinal_kw(lang: &str, value: BigInt, kwargs: PyKwargs) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_cardinal_kw(&value, &kwbag(kwargs)))
}

#[pyfunction]
fn to_ordinal_kw(lang: &str, value: BigInt, kwargs: PyKwargs) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_ordinal_kw(&value, &kwbag(kwargs)))
}

#[pyfunction]
fn to_ordinal_num_kw(lang: &str, value: BigInt, kwargs: PyKwargs) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_ordinal_num_kw(&value, &kwbag(kwargs)))
}

#[pyfunction]
fn to_year_kw(lang: &str, value: BigInt, kwargs: PyKwargs) -> PyResult<Option<String>> {
    finish(need_lang(lang)?.to_year_kw(&value, &kwbag(kwargs)))
}

#[pyfunction]
#[pyo3(signature = (lang, value, is_int, has_decimal, is_float, currency, cents, separator, adjective, kwargs))]
#[allow(clippy::too_many_arguments)]
fn to_currency_kw(
    lang: &str,
    value: &str,
    is_int: bool,
    has_decimal: bool,
    is_float: bool,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
    kwargs: PyKwargs,
) -> PyResult<Option<String>> {
    let l = need_lang(lang)?;
    let v = CurrencyValue::parse(value, is_int, has_decimal, is_float).map_err(map_err)?;
    let adjective = adjective.unwrap_or(l.default_adjective());
    let currency = currency.unwrap_or(l.default_currency());
    finish(l.to_currency_kw(&v, currency, cents, separator, adjective, &kwbag(kwargs)))
}

/// String input — Python's `num2words("1.50", ...)` path.
///
/// Returns `(kind, result)`:
///   kind 0 — converted; `result` is the value (None reproduces lang_VI's
///            bare-None return).
///   kind 1 — the Rust side cannot decide (str_to_number failed with digits
///            present -> sentence fallback, or a hook the language hasn't
///            ported yet). The shim reruns the ORIGINAL Python string path,
///            which owns every one of those cases, so behaviour is
///            unchanged.
/// Genuine errors raise, exactly typed (decimal.InvalidOperation for
/// unparseable digit-free strings, ZeroDivisionError for "1/0", ...).
#[pyfunction]
#[pyo3(signature = (lang, s, to, currency, cents, separator, adjective, kwargs))]
#[allow(clippy::too_many_arguments)]
fn from_string(
    lang: &str,
    s: &str,
    to: &str,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
    kwargs: PyKwargs,
) -> PyResult<(u8, Option<String>)> {
    let l = need_lang(lang)?;
    from_string_core(
        l,
        lang,
        s,
        to,
        currency,
        cents,
        separator,
        adjective,
        &kwbag(kwargs),
    )
}

/// The string-input router, factored so both the `from_string` entry point and
/// the unified `num2words` dispatcher share one implementation.
#[allow(clippy::too_many_arguments)]
fn from_string_core(
    l: &'static (dyn Lang + Sync),
    lang: &str,
    s: &str,
    to: &str,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
    kw: &Kwargs,
) -> PyResult<(u8, Option<String>)> {
    // "n/d" fraction strings route straight to to_fraction, whatever `to`
    // says — mirroring the dispatcher, where this check precedes the mode
    // dispatch entirely.
    let stripped = s.trim();
    if stripped.matches('/').count() == 1 {
        let (np, dp) = stripped.split_once('/').unwrap();
        if let (Some(n), Some(d)) = (python_int_parse(np.trim()), python_int_parse(dp.trim())) {
            if !kw.is_empty() {
                // to_fraction takes no kwargs in Python — TypeError there;
                // let the original raise it.
                return Ok((1, None));
            }
            return finish(l.to_fraction(&n, &d)).map(|r| (0, r));
        }
    }

    let parsed = match l.str_to_number(s) {
        Ok(p) => p,
        Err(e) => {
            // The dispatcher catches (InvalidOperation, ValueError): with a
            // digit anywhere in the string it goes to the sentence
            // converter, otherwise the exception propagates. Anything else
            // (lang_DV raises TypeError) propagates unconditionally.
            let catchable = matches!(
                &e,
                N2WError::Value(_)
                    | N2WError::Custom { module: "decimal", class: "InvalidOperation", .. }
            );
            if catchable {
                if has_py_digit(s) {
                    // The dispatcher routes a mixed text+digit string to
                    // num2words_sentence — now the Rust sentence converter,
                    // so serve it natively instead of declining. kwargs on
                    // this path are exotic (the sentence converter takes
                    // none); defer those rare cases.
                    if kw.is_empty() {
                        return match sentencepath::convert(s, lang, to) {
                            Ok(out) => Ok((0, Some(out))),
                            Err(N2WError::Fallback(_)) => Ok((1, None)),
                            Err(e) => Err(map_err(e)),
                        };
                    }
                    return Ok((1, None));
                }
                return Err(map_err(e));
            }
            return Err(map_err(e));
        }
    };

    let r: Result<String, N2WError> = match parsed {
        ParsedNumber::EsOrdinal { n, gender } => {
            // Python stashes `_pending_ordinal` in str_to_number; it fires the
            // *next* time `to_cardinal(value)` runs on the same value. Which
            // mode reaches to_cardinal (and with which gender) decides the
            // result — reproduced per mode against the oracle:
            let gkw = Kwargs(vec![("gender".into(), KwVal::Str(gender.to_string()))]);
            match to {
                // to_cardinal fires directly.  "2da" -> "segunda".
                "cardinal" => l.to_ordinal_kw(&n, &gkw),
                // to_year -> to_cardinal fires with the stashed gender too.
                "year" => l.to_ordinal_kw(&n, &gkw),
                // to_ordinal runs directly, never consulting the stash, so it
                // uses its OWN default gender: "2da" -> "segundo", not "segunda".
                "ordinal" => l.to_ordinal(&n),
                // to_ordinal_num echoes the numeral; the stash is left unfired.
                "ordinal_num" => l.to_ordinal_num(&n),
                // to_currency: the whole-part cardinal fires the stash and
                // becomes the ordinal ("segundo euros"), EXCEPT value 1, whose
                // apocopated "un euro" bypasses to_cardinal entirely.
                "currency" => {
                    let cur = currency.unwrap_or(l.default_currency());
                    let adj = adjective.unwrap_or(l.default_adjective());
                    let normal = l.to_currency(
                        &CurrencyValue::Int(n.clone()), cur, cents, separator, adj,
                    );
                    let one = n == BigInt::from(1);
                    // ES apocopates "un euro" at 1 and the ordinal never fires;
                    // es_XX inherit Base.to_currency, whose money_verbose calls
                    // to_cardinal(1) so the ordinal DOES fire → "primero córdoba".
                    if one && !l.es_currency_ordinal_fires() {
                        normal
                    } else {
                        // Replace the whole-number word(s) with the ordinal. For
                        // n>=2 that word is to_cardinal(n); for the apocopated 1
                        // it is the first token of the currency string ("un").
                        match (normal, l.to_ordinal_kw(&n, &gkw)) {
                            (Ok(norm), Ok(ord)) => {
                                let card = if one {
                                    norm.split_once(' ').map(|(a, _)| a.to_string())
                                        .unwrap_or_default()
                                } else {
                                    l.to_cardinal(&n).unwrap_or_default()
                                };
                                Ok(norm.strip_prefix(&card)
                                    .map(|rest| format!("{}{}", ord, rest))
                                    .unwrap_or(norm))
                            }
                            (other, _) => other,
                        }
                    }
                }
                // fraction: getattr TypeError (missing denominator), same as
                // any int -> int_mode reproduces it.
                _ => int_mode(l, to, &n, kw, currency, cents, separator, adjective),
            }
        }
        ParsedNumber::DecPoint { value, pointword } => {
            let prec = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
            let fv = FloatValue::Decimal { value: value.clone(), precision: prec };
            match to {
                "cardinal" if kw.is_empty() => l.cardinal_with_pointword(&fv, pointword, None),
                _ => dec_mode(l, to, &value, kw, currency, cents, separator, adjective),
            }
        }
        ParsedNumber::Dec(value) => {
            dec_mode(l, to, &value, kw, currency, cents, separator, adjective)
        }
        // Inf/NaN behaviour is per-language: base raises OverflowError /
        // ValueError, but the self-contained converters that int() the raw
        // token raise ValueError / InvalidOperation. The language decides.
        ParsedNumber::Inf { negative } => l.inf_result(negative, to),
        ParsedNumber::NaN => l.nan_result(to),
    };
    match r {
        // A hook the language hasn't ported yet: let the original Python
        // string path handle it rather than guessing.
        Err(N2WError::Fallback(_)) => Ok((1, None)),
        other => finish(other).map(|v| (0, v)),
    }
}

#[allow(clippy::too_many_arguments)]
fn int_mode(
    l: &'static (dyn Lang + Sync),
    to: &str,
    n: &BigInt,
    kw: &Kwargs,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
) -> Result<String, N2WError> {
    match to {
        "cardinal" => l.to_cardinal_kw(n, kw),
        "ordinal" => l.to_ordinal_kw(n, kw),
        "ordinal_num" => l.to_ordinal_num_kw(n, kw),
        "year" => l.to_year_kw(n, kw),
        "currency" => {
            let adjective = adjective.unwrap_or(l.default_adjective());
            let currency = currency.unwrap_or(l.default_currency());
            l.to_currency_kw(
                &CurrencyValue::Int(n.clone()),
                currency,
                cents,
                separator,
                adjective,
                kw,
            )
        }
        // Python: getattr(converter, "to_fraction")(number) — TypeError
        // (missing denominator) when the class has the method, AttributeError
        // when it doesn't (BN/ID/DV). Their Rust to_fraction reproduces the
        // AttributeError, so probe with the cheap (1,1) call (denominator==1
        // short-circuits to to_cardinal(1) everywhere else).
        "fraction" => match l.to_fraction(&BigInt::from(1), &BigInt::from(1)) {
            Err(e @ N2WError::Attribute(_)) => Err(e),
            _ => Err(N2WError::Type(
                "to_fraction() missing 1 required positional argument: 'denominator'".into(),
            )),
        },
        other => Err(N2WError::Fallback(other.to_string())),
    }
}

#[allow(clippy::too_many_arguments)]
fn dec_mode(
    l: &'static (dyn Lang + Sync),
    to: &str,
    value: &BigDecimal,
    kw: &Kwargs,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
) -> Result<String, N2WError> {
    let prec = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
    let fv = FloatValue::Decimal { value: value.clone(), precision: prec };
    let repr = python_decimal_str(value);
    match to {
        "cardinal" => {
            if kw.is_empty() {
                l.cardinal_float_entry(&fv, None)
            } else {
                l.to_cardinal_float_kw(&fv, None, kw)
            }
        }
        _ if !kw.is_empty() => Err(N2WError::Fallback("kwargs".into())),
        "ordinal" => l.ordinal_float_entry(&fv),
        "ordinal_num" => l.ordinal_num_float_entry(&fv, &repr),
        "year" => l.year_float_entry(&fv),
        "currency" => {
            let adjective = adjective.unwrap_or(l.default_adjective());
            let currency = currency.unwrap_or(l.default_currency());
            l.to_currency_kw(
                &CurrencyValue::Decimal {
                    value: value.clone(),
                    has_decimal: repr.contains('.'),
                    // str_to_number yields a Decimal — never a float origin.
                    is_float: false,
                },
                currency,
                cents,
                separator,
                adjective,
                kw,
            )
        }
        // Python: getattr(converter, "to_fraction")(number) — TypeError
        // (missing denominator) when the class has the method, AttributeError
        // when it doesn't (BN/ID/DV). Their Rust to_fraction reproduces the
        // AttributeError, so probe with the cheap (1,1) call (denominator==1
        // short-circuits to to_cardinal(1) everywhere else).
        "fraction" => match l.to_fraction(&BigInt::from(1), &BigInt::from(1)) {
            Err(e @ N2WError::Attribute(_)) => Err(e),
            _ => Err(N2WError::Type(
                "to_fraction() missing 1 required positional argument: 'denominator'".into(),
            )),
        },
        other => Err(N2WError::Fallback(other.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (lang, value, currency=None))]
fn to_cheque(lang: &str, value: &str, currency: Option<&str>) -> PyResult<Option<String>> {
    cheque_core(need_lang(lang)?, value, currency).map_err(map_err)
}

/// The cheque router, shared by the `to_cheque` entry point and `num2words`.
fn cheque_core(
    l: &'static (dyn Lang + Sync),
    value: &str,
    currency: Option<&str>,
) -> Result<Option<String>, N2WError> {
    let currency = currency.unwrap_or(l.default_currency());
    let d = BigDecimal::from_str(value).map_err(|e| N2WError::Value(e.to_string()))?;
    opt(l.to_cheque(&d, currency))
}

/// The whole-number int modes (`_RUST_TYPES`), shared by `num2words`. Mirrors
/// the shim's `getattr(_RUST, "to_%s[_kw]" % to)(...)`: the `_kw` variant only
/// when kwargs are present.
fn int_int_mode(
    l: &'static (dyn Lang + Sync),
    to: &str,
    n: &BigInt,
    kw: &Kwargs,
) -> Result<Option<String>, N2WError> {
    let r = match to {
        "cardinal" if kw.is_empty() => l.to_cardinal(n),
        "cardinal" => l.to_cardinal_kw(n, kw),
        "ordinal" if kw.is_empty() => l.to_ordinal(n),
        "ordinal" => l.to_ordinal_kw(n, kw),
        "ordinal_num" if kw.is_empty() => l.to_ordinal_num(n),
        "ordinal_num" => l.to_ordinal_num_kw(n, kw),
        "year" if kw.is_empty() => l.to_year(n),
        "year" => l.to_year_kw(n, kw),
        // int_int_mode is only ever called with `to` in RUST_TYPES.
        other => Err(N2WError::Fallback(other.to_string())),
    };
    opt(r)
}

/// The currency router, shared by `num2words`. Mirrors the shim's
/// `to_currency[_kw]` selection (the `_kw` variant only when kwargs present).
#[allow(clippy::too_many_arguments)]
fn currency_core(
    l: &'static (dyn Lang + Sync),
    value: &str,
    is_int: bool,
    has_decimal: bool,
    is_float: bool,
    currency: Option<&str>,
    cents: bool,
    separator: Option<&str>,
    adjective: Option<bool>,
    kw: &Kwargs,
) -> Result<Option<String>, N2WError> {
    let v = CurrencyValue::parse(value, is_int, has_decimal, is_float)?;
    let adjective = adjective.unwrap_or(l.default_adjective());
    let currency = currency.unwrap_or(l.default_currency());
    let r = if kw.is_empty() {
        l.to_currency(&v, currency, cents, separator, adjective)
    } else {
        l.to_currency_kw(&v, currency, cents, separator, adjective, kw)
    };
    opt(r)
}

/// `to='fraction'` with a non-string number. The shim probed
/// `_RUST.to_fraction(lang, 1, 1)`: an `AttributeError` (BN/ID/DV have no
/// `to_fraction`) re-raises, anything else becomes the missing-`denominator`
/// `TypeError`.
fn fraction_probe(l: &'static (dyn Lang + Sync)) -> N2WError {
    match l.to_fraction(&BigInt::from(1), &BigInt::from(1)) {
        Err(e @ N2WError::Attribute(_)) => e,
        _ => N2WError::Type(
            "to_fraction() missing 1 required positional argument: 'denominator'".into(),
        ),
    }
}

// --- Argument classification for the unified `num2words` entry -------------

/// `str(obj)` as a Rust `String`.
fn pystr(obj: &Bound<'_, PyAny>) -> PyResult<String> {
    Ok(obj.str()?.to_cow()?.into_owned())
}

/// `kwargs.get(key)` — `None` for a missing key.
fn dict_get<'py>(
    kwargs: Option<&Bound<'py, PyDict>>,
    key: &str,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    match kwargs {
        Some(d) => d.get_item(key),
        None => Ok(None),
    }
}

/// `kwargs.get("style")` reduced to the string the post-processor compares
/// against ("terse"/"us"); a non-str value can never match and becomes None.
fn get_style(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Option<String>> {
    Ok(dict_get(kwargs, "style")?.and_then(|v| v.extract::<String>().ok()))
}

/// An `Option<&str>` kwarg (`currency`/`separator`): missing or explicit `None`
/// -> None; a str -> Some; anything else raises TypeError exactly as passing it
/// to the old pyfunction did.
fn get_opt_str(kwargs: Option<&Bound<'_, PyDict>>, key: &str) -> PyResult<Option<String>> {
    match dict_get(kwargs, key)? {
        Some(v) if !v.is_none() => Ok(Some(v.extract::<String>()?)),
        _ => Ok(None),
    }
}

/// `adjective` — an `Option<bool>` kwarg with the same rules as `get_opt_str`.
fn get_opt_bool(kwargs: Option<&Bound<'_, PyDict>>, key: &str) -> PyResult<Option<bool>> {
    match dict_get(kwargs, key)? {
        Some(v) if !v.is_none() => Ok(Some(v.extract::<bool>()?)),
        _ => Ok(None),
    }
}

/// `precision` — an `Option<u32>` kwarg.
fn get_precision(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Option<u32>> {
    match dict_get(kwargs, "precision")? {
        Some(v) if !v.is_none() => Ok(Some(v.extract::<u32>()?)),
        _ => Ok(None),
    }
}

/// Classify the `cents=` object and run the core's normalisation + guard.
fn classify_cents(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Option<(bool, bool)>> {
    Ok(match dict_get(kwargs, "cents")? {
        None => presentation::normalize_cents(CentsArg::Absent),
        Some(v) => {
            if v.is_none() {
                presentation::normalize_cents(CentsArg::Other)
            } else if let Ok(b) = v.cast::<PyBool>() {
                presentation::normalize_cents(CentsArg::Bool(b.is_true()))
            } else if let Ok(s) = v.extract::<String>() {
                presentation::normalize_cents(CentsArg::Str(&s))
            } else {
                presentation::normalize_cents(CentsArg::Other)
            }
        }
    })
}

/// Marshal the caller's kwargs into a `Kwargs` bag, skipping `skip` keys.
/// Returns `None` when a value has a type the core cannot carry — the shim's
/// `_rust_kw_items` returning None, which makes the caller decline the branch.
fn extras_to_kwargs(
    kwargs: Option<&Bound<'_, PyDict>>,
    skip: &[&str],
) -> PyResult<Option<Kwargs>> {
    let dict = match kwargs {
        Some(d) => d,
        None => return Ok(Some(Kwargs::default())),
    };
    let mut out: Vec<(String, KwVal)> = Vec::new();
    for (k, v) in dict.iter() {
        let key: String = k.extract()?;
        if skip.contains(&key.as_str()) {
            continue;
        }
        // Order mirrors `isinstance(v, (bool, int, str))` then `(list, tuple)`:
        // bool before int (a Python bool is an int).
        let val = if v.is_none() {
            KwVal::None
        } else if let Ok(b) = v.cast::<PyBool>() {
            KwVal::Bool(b.is_true())
        } else if v.is_instance_of::<PyInt>() {
            match v.extract::<i64>() {
                Ok(i) => KwVal::Int(i),
                // A plain int the core cannot carry -> decline the whole bag.
                Err(_) => return Ok(None),
            }
        } else if let Ok(s) = v.extract::<String>() {
            KwVal::Str(s)
        } else if v.is_instance_of::<PyList>() || v.is_instance_of::<PyTuple>() {
            let mut items: Vec<String> = Vec::new();
            let mut all_str = true;
            for x in v.try_iter()? {
                match x?.extract::<String>() {
                    Ok(s) => items.push(s),
                    Err(_) => {
                        all_str = false;
                        break;
                    }
                }
            }
            if all_str {
                KwVal::List(items)
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };
        out.push((key, val));
    }
    Ok(Some(Kwargs(out)))
}

/// abs(exponent) of `Decimal(str(number))` — the shim's fractional-precision
/// computation. BigDecimal parses the same repr forms (`"1.5"`, `"1e-05"`, ...)
/// and yields the identical scale.
fn decimal_scale(s: &str) -> u32 {
    BigDecimal::from_str(s)
        .map(|d| d.as_bigint_and_exponent().1.unsigned_abs() as u32)
        .unwrap_or(0)
}

/// The whole of the old `num2words2.__init__.num2words` pipeline: number-type
/// classification, language resolution, mode dispatch and the `style=`
/// post-processing — so the Python surface is a straight pass-through.
#[pyfunction]
#[pyo3(signature = (number, ordinal=false, lang="en", to="cardinal", **kwargs))]
fn num2words(
    py: Python<'_>,
    number: &Bound<'_, PyAny>,
    ordinal: bool,
    lang: &str,
    to: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Option<String>> {
    // Captured before any normalisation: the core keys off the *arrival* type
    // (a plain int vs a float/Decimal vs a str), not a post-parse value.
    let is_str = number.is_instance_of::<PyString>();
    let plain_int = number.is_exact_instance_of::<PyInt>(); // type(number) is int
    let intish = number.is_instance_of::<PyInt>(); // isinstance(number, int) — incl. bool
    let is_float = number.is_instance_of::<PyFloat>();
    // Only import `decimal` for objects that could actually be a Decimal.
    let is_decimal = if is_str || is_float || intish {
        false
    } else {
        let decimal_cls = py.import("decimal")?.getattr("Decimal")?;
        number.is_instance(&decimal_cls)?
    };
    let plain_num = is_float || is_decimal;

    let resolved = presentation::resolve_lang(lang).ok_or_else(not_implemented)?;
    let lang = resolved.as_str();
    let l = need_lang(lang)?;
    let style = get_style(kwargs)?;

    // ---- string input: from_string owns the whole pipeline.
    if is_str {
        let s: String = number.extract()?;
        let to_final = if ordinal { "ordinal" } else { to };
        if !CONVERTER_TYPES.contains(&to_final) {
            return Err(not_implemented());
        }
        let cents = classify_cents(kwargs)?;
        let extras = extras_to_kwargs(
            kwargs,
            &[
                "currency",
                "cents",
                "separator",
                "adjective",
                "style",
                "precision",
            ],
        )?;
        let (cents_bool, kw) = match (cents, extras) {
            (Some((c, _drop)), Some(kw)) => (c, kw),
            _ => return Err(not_implemented()),
        };
        let currency = get_opt_str(kwargs, "currency")?;
        let separator = get_opt_str(kwargs, "separator")?;
        let adjective = get_opt_bool(kwargs, "adjective")?;
        return match from_string_core(
            l,
            lang,
            &s,
            to_final,
            currency.as_deref(),
            cents_bool,
            separator.as_deref(),
            adjective,
            &kw,
        )? {
            (0, out) => Ok(out
                .map(|o| presentation::apply_style(&o, style.as_deref(), to_final, lang))),
            _ => Err(not_implemented()),
        };
    }

    // ---- non-string input.
    let to = if ordinal { "ordinal" } else { to };
    if !CONVERTER_TYPES.contains(&to) {
        return Err(not_implemented());
    }

    // integer modes with a plain int
    if plain_int && RUST_TYPES.contains(&to) {
        if let Some(kw) = extras_to_kwargs(kwargs, &["style", "precision"])? {
            let n: BigInt = number.extract()?;
            return match int_int_mode(l, to, &n, &kw) {
                Ok(out) => {
                    Ok(out.map(|o| presentation::apply_style(&o, style.as_deref(), to, lang)))
                }
                Err(N2WError::Fallback(_)) => Err(not_implemented()),
                Err(e) => Err(map_err(e)),
            };
        }
        // items None -> fall through
    }

    // float / Decimal, all four integer modes
    if plain_num && RUST_TYPES.contains(&to) {
        let finite = matches!(number.extract::<f64>(), Ok(f) if f.is_finite());
        let items = extras_to_kwargs(
            kwargs,
            &[
                "style",
                "precision",
                "currency",
                "cents",
                "separator",
                "adjective",
            ],
        )?;
        if let (true, Some(kw)) = (finite, items) {
            let value = number.extract::<f64>()?;
            let repr_str = pystr(number)?;
            let prec = decimal_scale(&repr_str);
            let decimal_str = if is_decimal {
                repr_str.clone()
            } else {
                String::new()
            };
            let precision_override = get_precision(kwargs)?;
            return match to_float_core(
                l,
                to,
                value,
                prec,
                &decimal_str,
                &repr_str,
                precision_override,
                &kw,
            ) {
                Ok(out) => {
                    Ok(out.map(|o| presentation::apply_style(&o, style.as_deref(), to, lang)))
                }
                Err(N2WError::Fallback(_)) => Err(not_implemented()),
                Err(e) => Err(map_err(e)),
            };
        }
        // not finite or items None -> fall through
    }

    // currency
    if to == "currency" && (intish || is_float || is_decimal) {
        if let Some((cents_bool, drop)) = classify_cents(kwargs)? {
            // cents='omit' on a float truncates to an int so no cents segment
            // appears (the int path drops cents naturally).
            let num_obj: Bound<'_, PyAny> = if drop && is_float {
                py.import("builtins")?.getattr("int")?.call1((number,))?
            } else {
                number.clone()
            };
            if let Some(kw) = extras_to_kwargs(
                kwargs,
                &[
                    "style",
                    "precision",
                    "currency",
                    "cents",
                    "separator",
                    "adjective",
                ],
            )? {
                let value_str = pystr(&num_obj)?;
                let is_int_arg = num_obj.is_exact_instance_of::<PyInt>();
                let is_float_arg = num_obj.is_instance_of::<PyFloat>();
                let has_decimal_arg = is_float_arg || value_str.contains('.');
                let currency = get_opt_str(kwargs, "currency")?;
                let separator = get_opt_str(kwargs, "separator")?;
                let adjective = get_opt_bool(kwargs, "adjective")?;
                return match currency_core(
                    l,
                    &value_str,
                    is_int_arg,
                    has_decimal_arg,
                    is_float_arg,
                    currency.as_deref(),
                    cents_bool,
                    separator.as_deref(),
                    adjective,
                    &kw,
                ) {
                    Err(N2WError::Fallback(_)) => Err(not_implemented()),
                    Err(e) => Err(map_err(e)),
                    Ok(out) => Ok(out),
                };
            }
            // items None -> fall through
        }
        // cents guard fail -> fall through
    }

    // cheque
    if to == "cheque" && (intish || is_float || is_decimal) {
        let value_str = pystr(number)?;
        let currency = get_opt_str(kwargs, "currency")?;
        return match cheque_core(l, &value_str, currency.as_deref()) {
            Err(N2WError::Fallback(_)) => Err(not_implemented()),
            Err(e) => Err(map_err(e)),
            Ok(out) => Ok(out),
        };
    }

    // fraction
    if to == "fraction" {
        return Err(map_err(fraction_probe(l)));
    }

    Err(not_implemented())
}

/// `num2words_sentence` — dispatches on `lang=None` (auto-detect) vs a fixed
/// language, so the Python surface is a pass-through. `**kwargs` are accepted
/// and ignored, matching the historic signature.
#[pyfunction]
#[pyo3(signature = (sentence, lang=Some("en".to_string()), to="cardinal", **_kwargs))]
fn num2words_sentence(
    sentence: &str,
    lang: Option<String>,
    to: &str,
    _kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    match lang.as_deref() {
        None => sentencepath::convert_auto(sentence, to).map_err(map_err),
        Some(l) => sentencepath::convert(sentence, l, to).map_err(map_err),
    }
}

/// `num2words2.grouping.group_digits`. The shim keeps the isinstance check
/// (TypeError message needs Python's %r of the type).
#[pyfunction]
#[pyo3(signature = (value, locale, separator))]
fn group_digits(value: BigInt, locale: &str, separator: &str) -> PyResult<String> {
    fn group(s: &str, size: usize, sep: &str) -> String {
        let bytes = s.as_bytes();
        let mut parts: Vec<&str> = Vec::new();
        let mut end = bytes.len();
        while end > 0 {
            let start = end.saturating_sub(size);
            parts.push(&s[start..end]);
            end = start;
        }
        parts.reverse();
        parts.join(sep)
    }
    let sign = if value.sign() == num_bigint::Sign::Minus { "-" } else { "" };
    let s = value.magnitude().to_string();
    let out = match locale {
        "western" => format!("{}{}", sign, group(&s, 3, separator)),
        "indian" => {
            if s.len() <= 3 {
                format!("{}{}", sign, s)
            } else {
                let (rest, last3) = s.split_at(s.len() - 3);
                format!("{}{}{}{}", sign, group(rest, 2, separator), separator, last3)
            }
        }
        "chinese" => format!("{}{}", sign, group(&s, 4, separator)),
        other => return Err(PyValueError::new_err(format!("Unknown locale: '{}'", other))),
    };
    Ok(out)
}

/// `num2words2.maxval(lang)` — the per-language MAXVAL ceiling (issue #582).
#[pyfunction]
fn maxval(lang: &str) -> PyResult<Option<BigInt>> {
    let l = match get_lang(lang) {
        Some(l) => l,
        None => {
            let nl = lang.replace('-', "_");
            if let Some(l) = get_lang(&nl) {
                l
            } else {
                let prefix: String = nl.chars().take(2).collect();
                get_lang(&prefix).ok_or_else(|| {
                    PyNotImplementedError::new_err(format!("No MAXVAL for lang='{}'", lang))
                })?
            }
        }
    };
    Ok(l.python_maxval())
}

/// `num2words_sentence` — ported in `sentencepath.rs`. NotImplementedError
/// until the port lands; the shim falls back to the Python converter.
#[pyfunction]
#[pyo3(signature = (text, lang, to))]
fn sentence(text: &str, lang: &str, to: &str) -> PyResult<String> {
    sentencepath::convert(text, lang, to).map_err(map_err)
}

/// `num2words_sentence(text)` with `lang=None` — lingua-rs detection, then
/// conversion. Detection is best-effort (see sentencepath::detect_language);
/// NotImplementedError still falls back to the Python converter.
#[pyfunction]
#[pyo3(signature = (text, to))]
fn sentence_auto(text: &str, to: &str) -> PyResult<String> {
    sentencepath::convert_auto(text, to).map_err(map_err)
}

/// Detection alone, for the agreement harness. None on slim builds.
#[pyfunction]
fn detect_language(text: &str) -> Option<String> {
    sentencepath::detect_language(text)
}

#[pymodule]
fn _rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("RustFallback", m.py().get_type::<RustFallback>())?;
    m.add(
        "NumberTooLargeError",
        m.py().get_type::<NumberTooLargeError>(),
    )?;
    m.add_function(wrap_pyfunction!(supported_langs, m)?)?;
    m.add_function(wrap_pyfunction!(to_cardinal, m)?)?;
    m.add_function(wrap_pyfunction!(to_ordinal, m)?)?;
    m.add_function(wrap_pyfunction!(to_ordinal_num, m)?)?;
    m.add_function(wrap_pyfunction!(to_year, m)?)?;
    m.add_function(wrap_pyfunction!(to_fraction, m)?)?;
    m.add_function(wrap_pyfunction!(to_currency, m)?)?;
    m.add_function(wrap_pyfunction!(to_cheque, m)?)?;
    m.add_function(wrap_pyfunction!(to_cardinal_float, m)?)?;
    m.add_function(wrap_pyfunction!(to_cardinal_float_raw, m)?)?;
    m.add_function(wrap_pyfunction!(to_float, m)?)?;
    m.add_function(wrap_pyfunction!(to_cardinal_kw, m)?)?;
    m.add_function(wrap_pyfunction!(to_ordinal_kw, m)?)?;
    m.add_function(wrap_pyfunction!(to_ordinal_num_kw, m)?)?;
    m.add_function(wrap_pyfunction!(to_year_kw, m)?)?;
    m.add_function(wrap_pyfunction!(to_currency_kw, m)?)?;
    m.add_function(wrap_pyfunction!(from_string, m)?)?;
    m.add_function(wrap_pyfunction!(num2words, m)?)?;
    m.add_function(wrap_pyfunction!(num2words_sentence, m)?)?;
    m.add_function(wrap_pyfunction!(group_digits, m)?)?;
    m.add_function(wrap_pyfunction!(maxval, m)?)?;
    m.add_function(wrap_pyfunction!(sentence, m)?)?;
    m.add_function(wrap_pyfunction!(sentence_auto, m)?)?;
    m.add_function(wrap_pyfunction!(detect_language, m)?)?;
    Ok(())
}
