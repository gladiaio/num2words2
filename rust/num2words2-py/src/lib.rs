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
use num2words2_core::base::{
    floatord_error, py_num_str, year_float_error, Kwargs, KwVal, Lang,
};
use num2words2_core::currency::to_currency_checked;
use num2words2_core::floatpath::cardinal_with_precision;
use num2words2_core::presentation::{self, CentsArg};
use num2words2_core::strnum::{
    has_py_digit, is_malformed_number, number_notation, parse_grouped, python_decimal_str,
    python_int_parse, Grouped, ParsedNumber,
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

/// `NotImplementedError` for an input the core does not handle. The old
/// Python shim raised it with an empty message; keep the type (callers catch
/// it) but say what was declined so the error is actionable.
fn not_implemented(msg: String) -> PyErr {
    PyNotImplementedError::new_err(msg)
}

fn unknown_lang(lang: &str) -> PyErr {
    not_implemented(format!(
        "Language '{}' is not supported; run `num2words2 --list-languages` \
         for the supported codes",
        lang
    ))
}

fn unknown_converter(to: &str) -> PyErr {
    not_implemented(format!(
        "to='{}' is not supported; expected one of: {}",
        to,
        CONVERTER_TYPES.join(", ")
    ))
}

/// The core declined this (lang, to, input, kwargs) combination. Name the
/// keyword arguments, which are the usual cause.
fn declined(lang: &str, to: &str, kwargs: Option<&Bound<'_, PyDict>>) -> PyErr {
    let keys: Vec<String> = kwargs
        .map(|d| {
            d.keys()
                .iter()
                .filter_map(|k| k.extract::<String>().ok())
                .map(|k| format!("{}=", k))
                .collect()
        })
        .unwrap_or_default();
    if keys.is_empty() {
        not_implemented(format!(
            "lang='{}', to='{}' is not implemented for this input",
            lang, to
        ))
    } else {
        not_implemented(format!(
            "lang='{}', to='{}' does not support {} for this input",
            lang,
            to,
            keys.join(", ")
        ))
    }
}

/// Name the language in a core "does not support to='...'" raise: the core
/// does not know its own key, so `base::unsupported_mode` leaves the prefix
/// to the binder (`lang='ru' does not support to='fraction'`).
fn name_lang(lang: &str, e: N2WError) -> N2WError {
    match e {
        N2WError::NotImplemented(m) if m.starts_with("does not support") => {
            N2WError::NotImplemented(format!("lang='{}' {}", lang, m))
        }
        other => other,
    }
}

/// `to='fraction'` takes a "numerator/denominator" string; any other input
/// is a caller error (#217).
fn fraction_type_error(got: &str) -> N2WError {
    N2WError::Type(format!(
        "to='fraction' expects a 'numerator/denominator' string, got {}",
        got
    ))
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
// Subclasses OverflowError so `except OverflowError` catches "too large" from
// every language, bn included.
pyo3::create_exception!(_rust, NumberTooLargeError, pyo3::exceptions::PyOverflowError);

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
    get_lang(lang).ok_or_else(|| unknown_lang(lang))
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

/// The currency `to='currency'` uses when `currency=` is omitted.
#[pyfunction]
fn default_currency(lang: &str) -> PyResult<String> {
    Ok(need_lang(lang)?.default_currency().to_string())
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
    finish(fraction_core(need_lang(lang)?, lang, &numerator, &denominator))
}

/// "n/d": a zero denominator is ZeroDivisionError in every language, ahead
/// of a language's "does not support to='fraction'" (#217).
fn fraction_core(
    l: &'static (dyn Lang + Sync),
    lang: &str,
    n: &BigInt,
    d: &BigInt,
) -> Result<String, N2WError> {
    use bigdecimal::num_traits::Zero;
    if d.is_zero() {
        return Err(N2WError::ZeroDivision("denominator must not be zero".into()));
    }
    l.to_fraction(n, d).map_err(|e| name_lang(lang, e))
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
        // BigDecimal has no signed zero, so Decimal('-0.0') reads as zero
        // (#237).
        Ok(FloatValue::Decimal { value: d, precision })
    }
}

/// Float/Decimal input across all four int modes, kwargs included.
/// `repr_str` is Python's `str(number)`: the written value precision= reads
/// when `decimal_str` is empty (#218).
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
    // A negative zero reads as zero, without "minus" (#237).
    let v = float_value(value + 0.0, precision, decimal_str)?;
    // The integer modes take an integral value of any input type as that
    // integer — 1999.0, Decimal('1999.0') and '1999' read like 1999
    // (gladiaio/num2words2#213); the language never sees the float form.
    if matches!(to, "ordinal" | "ordinal_num" | "year") {
        return match v.as_whole_int() {
            Some(n) => int_int_mode(l, to, &n, kw),
            None => Err(fraction_error(to, &v)),
        };
    }
    let r = match to {
        // precision= reads the value as written, never the f64 (#218).
        "cardinal" if precision_override.is_some() => {
            let exact = if decimal_str.is_empty() { repr_str } else { decimal_str };
            match BigDecimal::from_str(exact) {
                Ok(d) => cardinal_with_precision(l, &d, precision_override.unwrap(), kw),
                Err(e) => Err(N2WError::Value(e.to_string())),
            }
        }
        "cardinal" => {
            if kw.is_empty() {
                l.cardinal_float_entry(&v, precision_override)
            } else {
                l.to_cardinal_float_kw(&v, precision_override, kw)
            }
        }
        other => Err(N2WError::Fallback(other.to_string())),
    };
    opt(r)
}

/// The one rule for a non-integral value in an integer mode
/// (gladiaio/num2words2#214): `TypeError`, raised here before the language
/// is called. Languages used to truncate (cs 2.5 -> "druhý"), glue an
/// ordinal suffix onto the cardinal float ("daou point pemp-vet") or fall
/// back to the cardinal.
fn fraction_error(to: &str, v: &FloatValue) -> N2WError {
    if to == "year" {
        year_float_error(v)
    } else {
        floatord_error(py_num_str(v))
    }
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
        sentencepath::Errors::Ignore,
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
    errors: sentencepath::Errors,
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
            return finish(fraction_core(l, lang, &n, &d)).map(|r| (0, r));
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
            // A pure numeric string with thousands separators ("1,000",
            // "1.000.000", "1 000") is the number it spells; the sentence
            // converter below would split it at the separator and read a
            // different number (#151). Whether a lone "1,000" is grouping
            // depends on the language's notation (#177; "1.000" never gets
            // here: it is a valid Decimal). Ambiguous or malformed grouping
            // raises instead of guessing.
            let grouped = if catchable {
                parse_grouped(s, number_notation(lang))
            } else {
                Grouped::NotGrouped
            };
            match grouped {
                Grouped::Number { canonical, decimal_comma } => {
                    match l.str_to_number(&canonical).map_err(map_err)? {
                        // pt_BR reads a dot-only string as US-style "ponto";
                        // the canonical form always uses '.', so restore
                        // "vírgula" when the writer used ','.
                        ParsedNumber::DecPoint { value, .. } if decimal_comma => {
                            ParsedNumber::Dec(value)
                        }
                        p => p,
                    }
                }
                Grouped::Invalid(msg) => return Err(map_err(N2WError::Value(msg))),
                Grouped::NotGrouped => {
                    if catchable {
                        if is_malformed_number(s) {
                            return Err(map_err(N2WError::Value(format!(
                                "cannot read {:?} as a number",
                                s.trim()
                            ))));
                        }
                        if has_py_digit(s) {
                            // The dispatcher routes a mixed text+digit string to
                            // num2words_sentence — now the Rust sentence converter,
                            // so serve it natively instead of declining. kwargs on
                            // this path are exotic (the sentence converter takes
                            // none); defer those rare cases.
                            if kw.is_empty() {
                                return match sentencepath::convert_with(s, lang, to, errors) {
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
            }
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
        // An integer in exponent form ("1e3", "1.5e2"): its plain digits
        // go to the integer modes (#211).
        ParsedNumber::Dec(value) if value.as_bigint_and_exponent().1 < 0 => {
            let n = value.with_scale(0).as_bigint_and_exponent().0;
            int_mode(l, to, &n, kw, currency, cents, separator, adjective)
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
        other => finish(other.map_err(|e| name_lang(lang, e))).map(|v| (0, v)),
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
        "ordinal_num" => ordinal_num_signed(l, n, kw),
        "year" => l.to_year_kw(n, kw),
        "currency" => {
            let adjective = adjective.unwrap_or(l.default_adjective());
            let currency = currency.unwrap_or(l.default_currency());
            to_currency_checked(
                l,
                &CurrencyValue::Int(n.clone()),
                currency,
                cents,
                separator,
                adjective,
                kw,
            )
        }
        // A plain number string ("5", "1.5") is not a fraction (#217).
        "fraction" => Err(fraction_type_error("a plain number")),
        // Cheque amounts usually arrive as strings (#223).
        "cheque" => l.to_cheque(
            &BigDecimal::from(n.clone()),
            currency.unwrap_or(l.default_currency()),
        ),
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
    // '1999.0' in an integer mode reads like 1999 (#213).
    if matches!(to, "ordinal" | "ordinal_num" | "year") {
        return match fv.as_whole_int() {
            Some(n) => int_mode(l, to, &n, kw, currency, cents, separator, adjective),
            None => Err(fraction_error(to, &fv)),
        };
    }
    match to {
        "cardinal" => {
            if kw.is_empty() {
                l.cardinal_float_entry(&fv, None)
            } else {
                l.to_cardinal_float_kw(&fv, None, kw)
            }
        }
        // Ahead of the kwargs guard: to_currency_kw owns its kwargs (en_NG
        // kobo=, ...) and declines the ones it does not take.
        "currency" => {
            let adjective = adjective.unwrap_or(l.default_adjective());
            let currency = currency.unwrap_or(l.default_currency());
            to_currency_checked(
                l,
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
        _ if !kw.is_empty() => Err(N2WError::Fallback("kwargs".into())),
        // A plain number string ("5", "1.5") is not a fraction (#217).
        "fraction" => Err(fraction_type_error("a plain number")),
        // Cheque amounts usually arrive as strings (#223).
        "cheque" => l.to_cheque(value, currency.unwrap_or(l.default_currency())),
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

/// `style='us'` on a cheque (#220): drop the "AND" inside the amount words
/// ("ONE HUNDRED ONE AND 50/100"), keeping the one that joins the cents.
fn cheque_style(out: &str, style: Option<&str>, lang: &str) -> String {
    if style != Some("us") || !lang.starts_with("en") {
        return out.to_string();
    }
    match out.rfind(" AND ") {
        Some(i) if out[i..].contains('/') => {
            format!("{}{}", out[..i].replace(" AND ", " "), &out[i..])
        }
        _ => out.replace(" AND ", " "),
    }
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
        "ordinal_num" => ordinal_num_signed(l, n, kw),
        "year" if kw.is_empty() => l.to_year(n),
        "year" => l.to_year_kw(n, kw),
        // int_int_mode is only ever called with `to` in RUST_TYPES.
        other => Err(N2WError::Fallback(other.to_string())),
    };
    opt(r)
}

/// `to='ordinal_num'` accepts a negative value exactly when the language's
/// `to='ordinal'` does (#214). be/et/fi/ja/pl/sv/uk/zh/... raised "Cannot
/// treat negative num" for the ordinal but returned "-3." / "第-3" for the
/// numeral, so the ordinal's error wins; ce/hi/hu/sq read -3 as an ordinal
/// but rejected the numeral, which is then the positive numeral with a
/// minus sign ("-3.").
fn ordinal_num_signed(
    l: &'static (dyn Lang + Sync),
    n: &BigInt,
    kw: &Kwargs,
) -> Result<String, N2WError> {
    if n.sign() != num_bigint::Sign::Minus {
        return l.to_ordinal_num_kw(n, kw);
    }
    match l.to_ordinal_kw(n, kw) {
        Err(N2WError::Fallback(_)) => l.to_ordinal_num_kw(n, kw),
        Err(e) => Err(e),
        Ok(_) => match l.to_ordinal_num_kw(n, kw) {
            Err(N2WError::Type(_)) => l.to_ordinal_num_kw(&-n, kw).map(|s| format!("-{}", s)),
            other => other,
        },
    }
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
    opt(to_currency_checked(l, &v, currency, cents, separator, adjective, kw))
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

/// `precision` — an `Option<u32>` kwarg. A negative value is a caller
/// error, not an internal conversion failure (#218).
fn get_precision(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Option<u32>> {
    match dict_get(kwargs, "precision")? {
        Some(v) if !v.is_none() => {
            let p = v.extract::<i64>()?;
            u32::try_from(p).map(Some).map_err(|_| {
                PyValueError::new_err(format!(
                    "precision= must be a non-negative integer, got {}",
                    p
                ))
            })
        }
        _ => Ok(None),
    }
}

/// Classify the `cents=` object and run the core's normalisation + guard.
/// A value outside the five accepted ones is a caller error (#220).
fn classify_cents(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<(bool, bool)> {
    let got = match dict_get(kwargs, "cents")? {
        Some(v) => v.repr()?.to_string(),
        None => String::new(),
    };
    classify_cents_raw(kwargs)?.ok_or_else(|| {
        PyValueError::new_err(format!(
            "cents= must be True, False, 'verbose', 'terse' or 'omit'; got {}",
            got
        ))
    })
}

fn classify_cents_raw(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Option<(bool, bool)>> {
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

/// A finite float/Decimal argument, normalised once before any language
/// sees it (gladiaio/num2words2#211). Python writes large and tiny values in
/// exponent form (`1e+21`, `1e-05`, `Decimal('1E+3')`), and the language
/// readers `int()` or digit-walk that string: ~80 raised `ValueError`, ce/cy/
/// rm* `IndexError`, en_AERO read the mantissa digits.
enum Normalised {
    /// An integer written in exponent form (`1e+21`, `Decimal('1E+3')`):
    /// served by the integer modes, as its plain digit string.
    Int(BigInt),
    /// A value in positional notation. `decimal` selects the exact Decimal
    /// arm of the float path; a float whose repr used an exponent (`1e-05`)
    /// takes it too, with its repr written out (`0.00001`).
    Frac { value: f64, repr: String, decimal: bool },
}

/// `repr` is Python's `str(number)`.
fn normalise_num(repr: &str, value: f64, is_decimal: bool) -> Normalised {
    if !repr.contains(['e', 'E']) {
        return Normalised::Frac { value, repr: repr.to_string(), decimal: is_decimal };
    }
    match BigDecimal::from_str(repr) {
        Ok(d) if d.is_integer() && (!is_decimal || d.as_bigint_and_exponent().1 <= 0) => {
            Normalised::Int(d.with_scale(0).as_bigint_and_exponent().0)
        }
        Ok(d) => Normalised::Frac { value, repr: python_decimal_str(&d), decimal: true },
        Err(_) => Normalised::Frac { value, repr: repr.to_string(), decimal: is_decimal },
    }
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
    // The core keys off the *arrival* type (an int vs a float/Decimal vs a
    // str), not a post-parse value. Numeric types are normalised first
    // (gladiaio/num2words2#236): anything with `__index__` (IntEnum, numpy
    // ints) is an int, anything else with `__float__` (numpy floats) a
    // float, in every mode. bool is rejected: True is not the number one.
    if number.is_instance_of::<PyBool>() {
        return Err(PyTypeError::new_err("bool is not a number"));
    }
    let is_str = number.is_instance_of::<PyString>();
    let mut is_float = number.is_instance_of::<PyFloat>();
    // Only import `decimal` for objects that could actually be a Decimal.
    let is_decimal = if is_str || is_float || number.is_instance_of::<PyInt>() {
        false
    } else {
        let decimal_cls = py.import("decimal")?.getattr("Decimal")?;
        number.is_instance(&decimal_cls)?
    };
    // -0.0 / Decimal('-0') read as zero, without "minus", in every
    // language (#237): '-0.0' as a string already did.
    let normalised: Bound<'_, PyAny> = if is_str {
        number.clone()
    } else if is_decimal {
        if number.call_method0("is_zero")?.is_truthy()? {
            number.call_method0("copy_abs")?
        } else {
            number.clone()
        }
    } else if is_float || !number.hasattr("__index__")? {
        if !is_float && !number.hasattr("__float__")? {
            return Err(PyTypeError::new_err(format!(
                "expected a number or a numeric string, got {}",
                number.get_type().name()?
            )));
        }
        is_float = true;
        // `+ 0.0` turns -0.0 into 0.0 and leaves every other value alone.
        PyFloat::new(py, number.extract::<f64>()? + 0.0).into_any()
    } else {
        py.import("operator")?.getattr("index")?.call1((number,))?
    };
    let number = &normalised;
    let plain_int = !is_str && !is_float && !is_decimal; // an int, via __index__
    let intish = plain_int;
    let plain_num = is_float || is_decimal;

    // `errors=` belongs to the dispatcher, never to a language converter
    // (which would decline an unknown kwarg): read it and drop it here.
    let (errors, stripped) = take_errors(kwargs, "raise")?;
    let kwargs = stripped.as_ref().or(kwargs);

    let resolved = presentation::resolve_lang(lang).ok_or_else(|| unknown_lang(lang))?;
    let lang = resolved.as_str();
    let l = need_lang(lang)?;
    let style = get_style(kwargs)?;

    // ---- string input: from_string owns the whole pipeline.
    if is_str {
        let s: String = number.extract()?;
        let to_final = if ordinal { "ordinal" } else { to };
        if !CONVERTER_TYPES.contains(&to_final) {
            return Err(unknown_converter(to_final));
        }
        // cents= only matters for currency; elsewhere an odd value keeps
        // declining as before.
        let cents = if to_final == "currency" {
            Some(classify_cents(kwargs)?)
        } else {
            classify_cents_raw(kwargs)?
        };
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
        let (cents_bool, drop_cents, kw) = match (cents, extras) {
            (Some((c, drop)), Some(kw)) => (c, drop, kw),
            _ => return Err(declined(lang, to_final, kwargs)),
        };
        let currency = get_opt_str(kwargs, "currency")?;
        let separator = get_opt_str(kwargs, "separator")?;
        let adjective = get_opt_bool(kwargs, "adjective")?;
        // "NaN"/"inf" have no numeric ordinal; some languages echoed the token
        // ("NaN", "Infinity-তম"), others raised InvalidOperation (#224).
        if to_final == "ordinal_num"
            && matches!(
                num2words2_core::strnum::python_decimal_parse(&s),
                Ok(ParsedNumber::Inf { .. }) | Ok(ParsedNumber::NaN)
            )
        {
            return Err(PyValueError::new_err(format!(
                "to='ordinal_num' needs a finite number, got {:?}",
                s.trim()
            )));
        }
        // precision= applies to a numeric string like to a float (#218).
        if let (Some(p), "cardinal") = (get_precision(kwargs)?, to_final) {
            if let Ok(ParsedNumber::Dec(d)) | Ok(ParsedNumber::DecPoint { value: d, .. }) =
                l.str_to_number(&s)
            {
                return match cardinal_with_precision(l, &d, p, &kw) {
                    Ok(o) => Ok(Some(presentation::apply_style(&o, style.as_deref(), to_final, lang))),
                    Err(N2WError::Fallback(_)) => Err(declined(lang, to_final, kwargs)),
                    Err(e) => Err(map_err(name_lang(lang, e))),
                };
            }
        }
        // cents='omit' truncates toward zero, as int() does for a float or
        // a Decimal (#220): "1.99" -> "one euro".
        let s = if drop_cents && to_final == "currency" {
            match l.str_to_number(&s) {
                Ok(ParsedNumber::Dec(d)) | Ok(ParsedNumber::DecPoint { value: d, .. }) => {
                    d.with_scale_round(0, bigdecimal::RoundingMode::Down).to_string()
                }
                _ => s,
            }
        } else {
            s
        };
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
            errors,
        )? {
            (0, out) => Ok(out.map(|o| {
                if to_final == "cheque" {
                    cheque_style(&o, style.as_deref(), lang)
                } else {
                    presentation::apply_style(&o, style.as_deref(), to_final, lang)
                }
            })),
            _ => Err(declined(lang, to_final, kwargs)),
        };
    }

    // ---- non-string input.
    let to = if ordinal { "ordinal" } else { to };
    if !CONVERTER_TYPES.contains(&to) {
        return Err(unknown_converter(to));
    }

    // integer modes with a plain int
    if plain_int && RUST_TYPES.contains(&to) {
        if let Some(kw) = extras_to_kwargs(kwargs, &["style", "precision"])? {
            let n: BigInt = number.extract()?;
            return match int_int_mode(l, to, &n, &kw) {
                Ok(out) => {
                    Ok(out.map(|o| presentation::apply_style(&o, style.as_deref(), to, lang)))
                }
                Err(N2WError::Fallback(_)) => Err(declined(lang, to, kwargs)),
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
            let r = match normalise_num(&pystr(number)?, value, is_decimal) {
                Normalised::Int(n) => int_int_mode(l, to, &n, &kw),
                Normalised::Frac { value, repr, decimal } => {
                    let prec = decimal_scale(&repr);
                    let decimal_str = if decimal { repr.clone() } else { String::new() };
                    let precision_override = get_precision(kwargs)?;
                    to_float_core(
                        l,
                        to,
                        value,
                        prec,
                        &decimal_str,
                        &repr,
                        precision_override,
                        &kw,
                    )
                }
            };
            return match r {
                Ok(out) => {
                    Ok(out.map(|o| presentation::apply_style(&o, style.as_deref(), to, lang)))
                }
                Err(N2WError::Fallback(_)) => Err(declined(lang, to, kwargs)),
                Err(e) => Err(map_err(name_lang(lang, e))),
            };
        }
        // NaN / ±inf: the same outcome as the strings "NaN" / "inf"; the
        // language decides (base: ValueError / OverflowError).
        if !finite {
            let f = number.extract::<f64>()?;
            let r = if f.is_nan() {
                l.nan_result(to)
            } else {
                l.inf_result(f < 0.0, to)
            };
            return finish(r);
        }
        // items None -> fall through
    }

    // NaN / ±inf has no amount to spell (#236): a clear ValueError rather
    // than the Rust parser's "invalid digit found in string".
    if plain_num && (to == "currency" || to == "cheque") {
        let f = number.extract::<f64>()?;
        if !f.is_finite() {
            return Err(PyValueError::new_err(format!(
                "cannot convert {} to {}",
                if f.is_nan() { "NaN" } else { "Infinity" },
                to
            )));
        }
    }

    // currency
    if to == "currency" && (intish || is_float || is_decimal) {
        {
            let (cents_bool, drop) = classify_cents(kwargs)?;
            // cents='omit' on a float or Decimal truncates to an int (toward
            // zero, Python's int()) so no cents segment appears (the int path
            // drops cents naturally). #220: Decimal used to keep its cents.
            let num_obj: Bound<'_, PyAny> = if drop && (is_float || is_decimal) {
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
                let mut value_str = pystr(&num_obj)?;
                // #211: '1E+3' / '1e+21' written out before the language
                // int()s it.
                if value_str.contains(['e', 'E']) {
                    if let Ok(d) = BigDecimal::from_str(&value_str) {
                        value_str = python_decimal_str(&d);
                    }
                }
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
                    Err(N2WError::Fallback(_)) => Err(declined(lang, to, kwargs)),
                    Err(e) => Err(map_err(name_lang(lang, e))),
                    // style='us' applies to every input type (#220).
                    Ok(out) => {
                        Ok(out.map(|o| presentation::apply_style(&o, style.as_deref(), to, lang)))
                    }
                };
            }
            // items None -> fall through
        }
    }

    // cheque
    if to == "cheque" && (intish || is_float || is_decimal) {
        let value_str = pystr(number)?;
        let currency = get_opt_str(kwargs, "currency")?;
        return match cheque_core(l, &value_str, currency.as_deref()) {
            Err(N2WError::Fallback(_)) => Err(declined(lang, to, kwargs)),
            Err(e) => Err(map_err(name_lang(lang, e))),
            Ok(out) => Ok(out.map(|o| cheque_style(&o, style.as_deref(), lang))),
        };
    }

    // fraction
    if to == "fraction" {
        let tname = number.get_type().name()?.to_string();
        return Err(map_err(fraction_type_error(&tname)));
    }

    Err(declined(lang, to, kwargs))
}

/// The `errors=` keyword ("raise" | "ignore", default `default`) and, when
/// it was given, a copy of `kwargs` without it.
fn take_errors<'py>(
    kwargs: Option<&Bound<'py, PyDict>>,
    default: &str,
) -> PyResult<(sentencepath::Errors, Option<Bound<'py, PyDict>>)> {
    let Some(kw) = kwargs else {
        return Ok((sentencepath::Errors::parse(default).map_err(map_err)?, None));
    };
    let Some(v) = kw.get_item("errors")? else {
        return Ok((sentencepath::Errors::parse(default).map_err(map_err)?, None));
    };
    let mode = match v.extract::<String>() {
        Ok(m) => m,
        Err(_) => pystr(&v)?,
    };
    let errors = sentencepath::Errors::parse(&mode).map_err(map_err)?;
    let rest = kw.copy()?;
    rest.del_item("errors")?;
    Ok((errors, Some(rest)))
}

/// `num2words_sentence` — dispatches on `lang=None` (auto-detect) vs a fixed
/// language, so the Python surface is a pass-through. `errors="ignore"` (the
/// default) leaves a token it cannot convert as written, `errors="raise"`
/// raises ValueError naming it. Other `**kwargs` are accepted and ignored,
/// matching the historic signature.
#[pyfunction]
#[pyo3(signature = (sentence, lang=Some("en".to_string()), to="cardinal", **kwargs))]
fn num2words_sentence(
    sentence: &str,
    lang: Option<String>,
    to: &str,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let (errors, _) = take_errors(kwargs, "ignore")?;
    match lang.as_deref() {
        None => sentencepath::convert_auto(sentence, to, errors).map_err(map_err),
        Some(l) => sentencepath::convert_with(sentence, l, to, errors).map_err(map_err),
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
    sentencepath::convert_auto(text, to, sentencepath::Errors::Ignore).map_err(map_err)
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
    m.add_function(wrap_pyfunction!(default_currency, m)?)?;
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
