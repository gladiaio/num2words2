//! Port of `base.py`'s float/Decimal path: `float2tuple` and
//! `to_cardinal_float`.
//!
//! # Why the f64 arrives as an f64
//!
//! The float branch of `float2tuple` is *binary* arithmetic:
//!
//! ```python
//! post = abs(value - pre) * 10**precision
//! if abs(round(post) - post) < 0.01:
//!     post = int(round(post))
//! else:
//!     post = int(math.floor(post))
//! ```
//!
//! Those artefacts are load-bearing. `2.675` really does produce
//! `674.9999999999998`, and the `< 0.01` heuristic is what rescues it back to
//! `675`. Re-deriving that from a decimal string would compute the *right*
//! answer and therefore the *wrong* one. Python floats and Rust `f64` are both
//! IEEE-754 doubles, so passing the raw double across reproduces it exactly.
//!
//! `precision` still comes from Python: it is `abs(Decimal(str(value))
//! .as_tuple().exponent)`, which depends on `repr(float)` — shortest
//! round-trip formatting that we deliberately do not reimplement.
//!
//! # The one true rounding trap
//!
//! Python's `round()` is round-half-to-**even** (`round(2.5) == 2`), while
//! Rust's `f64::round()` is half-away-from-zero (`2.5_f64.round() == 3.0`).
//! Every rounding here uses `round_ties_even()`.

use crate::base::{Lang, N2WError, Result};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};

/// The value handed to the float path, mirroring what Python's dispatcher saw.
///
/// `base.float2tuple` branches on `isinstance(value, Decimal)`: the Decimal
/// arm is exact arbitrary precision, the float arm is lossy f64. They are not
/// interchangeable — issue #603 exists precisely because a `float()` cast
/// silently rounded `98_746_251_323_029.99` at trillion scale.
#[derive(Debug, Clone)]
pub enum FloatValue {
    /// Python passed a `float`. `precision` is derived from `repr` on the
    /// Python side.
    Float { value: f64, precision: u32 },
    /// Python passed a `Decimal`. `precision` is `abs(exponent)`.
    Decimal { value: BigDecimal, precision: u32 },
}

impl FloatValue {
    pub fn precision(&self) -> u32 {
        match self {
            FloatValue::Float { precision, .. } => *precision,
            FloatValue::Decimal { precision, .. } => *precision,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            // Sign-bit aware: the converters see the *string* form, and
            // str(-0.0) is "-0.0", so Python renders the negword for
            // negative zero. `< 0.0` would miss it.
            FloatValue::Float { value, .. } => value.is_sign_negative(),
            FloatValue::Decimal { value, .. } => value.is_negative(),
        }
    }

    /// Whether Python's string form of the value shows a decimal point —
    /// the `"." in str(value)` test many self-contained converters route on.
    /// `repr(float)` has a point unless repr picked exponent form (1e+16);
    /// `str(Decimal)` has one exactly when the scale is positive.
    pub fn has_visible_point(&self) -> bool {
        match self {
            // Rust's Display renders 21.0 as "21", so it cannot stand in for
            // Python's repr. The rule that matters: repr picks exponent form
            // (no dot) at |v| >= 1e16; every other finite float prints with
            // a point ("21.0", "0.0"). Fractional values below 1e-4 also go
            // e-form, but they never reach a whole-vs-float routing test.
            FloatValue::Float { value, .. } => {
                value.is_finite() && (value.fract() != 0.0 || value.abs() < 1e16)
            }
            FloatValue::Decimal { value, .. } => {
                crate::strnum::python_decimal_str(value).contains('.')
            }
        }
    }

    /// Python's `int(value) == value` routing test: `Some(int)` when the
    /// value is whole, `None` when fractional. NaN/±inf return `None` — the
    /// dispatcher keeps those on the Python side (base's `assert` raises
    /// OverflowError/ValueError there, outside the whole-value route).
    pub fn as_whole_int(&self) -> Option<BigInt> {
        match self {
            FloatValue::Float { value, .. } => {
                if !value.is_finite() || value.fract() != 0.0 {
                    return None;
                }
                // Exact: every whole f64 is exactly representable.
                BigInt::from_f64(*value)
            }
            FloatValue::Decimal { value, .. } => {
                if value.is_integer() {
                    Some(value.with_scale(0).as_bigint_and_exponent().0)
                } else {
                    None
                }
            }
        }
    }
}

/// Python's `Num2Word_Base.float2tuple`. Returns `(pre, post)`.
///
/// `precision` is taken from the value rather than recomputed; Python assigns
/// `self.precision` as a side effect here, and callers read it afterwards.
pub fn float2tuple(v: &FloatValue) -> (BigInt, BigInt) {
    match v {
        FloatValue::Decimal { value, precision } => {
            // pre = int(value) — truncates toward zero.
            let pre = value.with_scale(0).as_bigint_and_exponent().0;
            // post = abs(value - Decimal(pre)) * 10**precision
            let post = (value - BigDecimal::from(pre.clone())).abs()
                * BigDecimal::from(BigInt::from(10).pow(*precision));
            (pre, post.with_scale(0).as_bigint_and_exponent().0)
        }
        FloatValue::Float { value, precision } => {
            let pre = value.trunc();
            // Python: abs(value - pre) * 10**precision, all in f64.
            let post = (value - pre).abs() * 10f64.powi(*precision as i32);

            // The heuristic: values like 1.239999999 are float noise for
            // 1.24, so round rather than floor when we are within 0.01 of an
            // integer. round() is banker's in Python — round_ties_even here,
            // NOT f64::round(), which would break every exact .5 tie.
            let rounded = post.round_ties_even();
            let out = if (rounded - post).abs() < 0.01 {
                rounded
            } else {
                post.floor()
            };
            (
                BigInt::from(pre as i128),
                BigInt::from(out as i128),
            )
        }
    }
}

/// Python's `Num2Word_Base.to_cardinal_float`.
///
/// `precision_override` is the `precision=` kwarg (issue #580). Python saves
/// and restores `self.precision` around the call; the Rust core is stateless,
/// so the effective precision is threaded through instead.
pub fn default_to_cardinal_float<L: Lang + ?Sized>(
    lang: &L,
    v: &FloatValue,
    precision_override: Option<u32>,
) -> Result<String> {
    default_to_cardinal_float_by(lang, v, precision_override, |pre| lang.to_cardinal(pre))
}

/// [`default_to_cardinal_float`] with the integer part rendered by
/// `pre_word` instead of `to_cardinal` — for a language whose
/// `to_cardinal_float` forwards a kwarg (HE's `gender=`) to the integer
/// part only; the fractional digits stay plain `to_cardinal`.
pub fn default_to_cardinal_float_by<L: Lang + ?Sized>(
    lang: &L,
    v: &FloatValue,
    precision_override: Option<u32>,
    pre_word: impl Fn(&BigInt) -> Result<String>,
) -> Result<String> {
    let precision = precision_override.unwrap_or_else(|| v.precision());
    let v = match (v, precision_override) {
        // A caller-supplied precision replaces the repr-derived one.
        (FloatValue::Float { value, .. }, Some(p)) => FloatValue::Float {
            value: *value,
            precision: p,
        },
        (FloatValue::Decimal { value, .. }, Some(p)) => FloatValue::Decimal {
            value: value.clone(),
            precision: p,
        },
        (other, None) => other.clone(),
    };

    let (pre, post) = float2tuple(&v);

    // post = str(post); post = "0" * (precision - len(post)) + post
    let post_str = post.to_string();
    let post_str = format!(
        "{}{}",
        "0".repeat((precision as usize).saturating_sub(post_str.len())),
        post_str
    );

    let mut out = vec![pre_word(&pre)?];
    // Python: `if value < 0 and pre == 0: out = [negword] + out` — the sign is
    // otherwise lost, because int(-0.5) == 0 carries no minus.
    if v.is_negative() && pre.is_zero() {
        out.insert(0, lang.negword().trim().to_string());
    }

    if precision > 0 {
        out.push(lang.title(lang.pointword()));
    }

    for ch in post_str.chars().take(precision as usize) {
        let d = ch.to_digit(10).ok_or_else(|| {
            N2WError::Value(format!("non-digit {:?} in fractional part", ch))
        })?;
        out.push(lang.to_cardinal(&BigInt::from(d))?);
    }

    Ok(out.join(" "))
}

/// "Condition C" float routing — `"." in str(value)` decides. Most
/// self-contained converters route on the value's *string* form: visible
/// decimals take the language's float grammar even when the value is whole
/// (5.0 -> "pět čárka nula"), plain ints don't (Decimal("5") -> "pět").
/// Languages with this shape point their `cardinal_float_entry` here.
pub fn point_routed_float_entry<L: Lang + ?Sized>(
    lang: &L,
    v: &FloatValue,
    precision_override: Option<u32>,
) -> Result<String> {
    if !v.has_visible_point() {
        if let Some(i) = v.as_whole_int() {
            return lang.to_cardinal(&i);
        }
    }
    lang.to_cardinal_float(v, precision_override)
}

/// Render a `BigDecimal` through the float path — the fractional-cents entry
/// point. `right` is e.g. 65.3 cents, so its own scale is the precision.
pub fn cardinal_from_bigdecimal<L: Lang + ?Sized>(
    lang: &L,
    value: &BigDecimal,
) -> Result<String> {
    // Python reaches here via `self.to_cardinal(float(right))`, so the value
    // goes through a float cast first — reproduce that rather than staying in
    // arbitrary precision, or the digits can differ.
    let f = value.to_f64().ok_or_else(|| {
        N2WError::Value(format!("cannot represent {} as f64", value))
    })?;
    let precision = float_repr_precision(f);
    // Through the TRAIT, not the free function: languages that override
    // to_cardinal_float (ru's "целых", hu's "egész") must see their own
    // float grammar here too — Python reaches this as self.to_cardinal(float),
    // which dispatches virtually.
    lang.to_cardinal_float(&FloatValue::Float { value: f, precision }, None)
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64.
///
/// Rust's `{}` for f64 is shortest-round-trip, the same contract as Python's
/// `repr`, so counting the digits after the point matches. Exponent form
/// (`1e21`) has no fractional digits, matching Python's exponent-0 tuple.
pub fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}
