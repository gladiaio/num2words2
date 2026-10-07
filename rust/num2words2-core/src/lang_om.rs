//! Port of `lang_OM.py` (Oromo / Afaan Oromoo).
//!
//! Registry check: `__init__.py` line 394 maps `"om"` → `lang_OM.Num2Word_OM()`,
//! so this file ports `Num2Word_OM` and nothing else.
//!
//! Shape: **self-contained**. `Num2Word_OM` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int_to_word`. Consequently `cards`/`merge` stay at
//! their trait defaults here. Python has **no overflow check** — see the
//! digit-passthrough note below; this port raises `OverflowError` from 10^12
//! (`maxval`).
//!
//! Inherited from `Num2Word_Base`, unchanged by OM:
//!   * `is_title` stays `False` (OM's `setup` never sets it), so `title()` is
//!     the identity and `exclude_title` is dead weight. Both are still modelled
//!     below so the trait reports what Python holds.
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following look wrong but are exactly what
//! Python emits, and every one is pinned by a corpus row:
//!
//! 1. **Digit passthrough above 999,999,999 (fixed, gladiaio/num2words2#147).**
//!    Python's `_int_to_word` has no branch for 10^9 and up; it falls off the
//!    end and `return str(number)`, so `to_cardinal(10**9)` was
//!    `"1000000000"`. This port adds biliyoona (10^9; Afaan Oromoo press
//!    usage, https://www.ena.et/web/orm/w/om_3014, press.et/oromifa),
//!    composed exactly like the million branch: `to_cardinal(10**9)` ==
//!    `"biliyoona"`, `to_cardinal(2 * 10**9)` == `"lama biliyoona"`. No 10^12
//!    word is attested, so 10^12 raises `OverflowError` (`maxval`). See
//!    [`LangOm::int_to_word`].
//! 2. **`to_ordinal` just glues "ffaa" onto the cardinal**, with no regard for
//!    where the cardinal ends. Hence `to_ordinal(101)` == "dhibba fi tokkoffaa"
//!    (suffix lands on the final word only) and `to_ordinal(10**9)` ==
//!    `"biliyoonaffaa"` (Python: `"1000000000ffaa"`).
//! 3. **Negatives flow into the ordinal suffix.** `to_ordinal` special-cases
//!    only `number == 1`, so `to_ordinal(-1)` == "minus tokkoffaa" rather than
//!    raising. `Num2Word_Base.verify_ordinal` is never called by this module,
//!    so the usual "Cannot treat negative num as ordinal" TypeError never fires.
//! 4. **"fi" ("and") joins every level**, including between a thousands group
//!    and its remainder: `to_cardinal(1234)` == "kuma fi lama dhibba fi soddoma
//!    fi afur". The leading multiplier is dropped when it is 1 ("kuma", not
//!    "tokko kuma"), because each branch guards on `> 1`.
//!
//! # Notes on unreachable Python paths
//!
//! `to_cardinal` stringifies its argument and branches on a `"."` in the text;
//! `str(int)` never contains one, so the fractional path (`pointword`, per-digit
//! decimals) is unreachable for the integer input this port handles, and is out
//! of scope per PORTING.md. Likewise `_int_to_word` is never handed a negative
//! (`to_cardinal` strips the sign before recursing), so Python's latent
//! negative-index hazard there — `self.ones[number]` would wrap to `ones[-1]`
//! == "sagal" for `number == -1` — is unreachable from the four supported
//! modes and is deliberately not modelled.
//!
//! No cross-call mutable state: every method is a pure function of its
//! argument. Nothing for the dispatcher to skip.
//!
//! # The currency surface
//!
//! Two halves that share almost nothing:
//!
//! * **`to_currency` is a wholesale override.** None of
//!   `Num2Word_Base.to_currency` runs — no `parse_currency_parts`, no
//!   `ROUND_HALF_UP` quantize, no per-currency divisor, no `pluralize`, and
//!   **no `NotImplementedError`**. An unknown code silently borrows ETB's
//!   forms via `.get(currency, list(CURRENCY_FORMS.values())[0])`, which is why
//!   `to_currency(2, "JPY")` is "lama birrii" and not an error. Cents are a
//!   *digit slice* of `str(val)`, not arithmetic — see [`split_currency`].
//! * **`to_cheque` is inherited untouched** from `Num2Word_Base`, so it *does*
//!   subscript `CURRENCY_FORMS[currency]` and *does* raise
//!   `NotImplementedError` on a miss. Hence the corpus split: `currency:GBP`
//!   → "zeeroo birrii", `cheque:GBP` → NotImplementedError. The two disagree
//!   on purpose; do not unify them.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` (OM defines
//! neither, so `Num2Word_Base`'s empty dicts stand). The trait defaults —
//! precision 100, adjective `None` — already say exactly that, so neither hook
//! is overridden here. `adjective=True` is inert regardless: OM's
//! `to_currency` declares the parameter and never reads it.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `setup`: `self.negword = "minus "` — note the trailing space, which the
/// `.strip()` at the end of `to_cardinal`'s negative branch makes invisible.
const NEGWORD: &str = "minus ";

/// `setup`: `self.pointword = "tuqaa"`. Only the (out-of-scope) float path
/// reads it.
const POINTWORD: &str = "tuqaa";

/// `setup`: `self.ones`. Index 0 is `""` and is never read — `_int_to_word`
/// returns "zeeroo" for 0 before any lookup, and the `h > 1` / `t > 1` guards
/// keep the multiplier branches off it too.
const ONES: [&str; 10] = [
    "", "tokko", "lama", "sadii", "afur", "shan", "ja'a", "torba", "saddeet", "sagal",
];

/// `setup`: `self.tens`. Index 0 is `""` and unreachable (the `number < 100`
/// branch only runs for `number >= 10`, so `t >= 1`). Index 1 is "kudhan", so
/// the teens are built compositionally: 11 → "kudhan fi tokko".
const TENS: [&str; 10] = [
    "",
    "kudhan",
    "digdama",
    "soddoma",
    "afurtama",
    "shantama",
    "jaatama",
    "torbaatama",
    "saddeettama",
    "sagaltama",
];

const HUNDRED: &str = "dhibba";
const THOUSAND: &str = "kuma";
const MILLION: &str = "miliyoona";
/// 10^9, absent from Python (gladiaio/num2words2#147): "biliyoona", as in
/// Afaan Oromoo press usage (ena.et, press.et).
const BILLION: &str = "biliyoona";

/// The exclusive ceiling (#147): no attested Oromo word for 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// The zero word, returned by `_int_to_word(0)` (and, in the out-of-scope float
/// path, substituted for the empty `ones[0]`).
const ZEROWORD: &str = "zeeroo";

/// `" fi "` — the conjunction Python interpolates at every join.
const AND: &str = " fi ";

/// `Num2Word_OM.CURRENCY_FORMS`, in class-body order: `(code, unit, subunit)`.
///
/// The order is load-bearing. `to_currency`'s fallback is
/// `list(self.CURRENCY_FORMS.values())[0]` and CPython dicts iterate in
/// insertion order, so "the first value" is ETB's — permanently. Unlike the
/// `lang_EUR`/`lang_EN` class-attribute mutation documented in
/// PORTING_CURRENCY.md, this dict is OM's own and no other class writes to it,
/// so the literal here *is* what runs. Verified against the live interpreter:
/// `list(Num2Word_OM().CURRENCY_FORMS.values())[0]` is ETB's pair.
///
/// Every entry carries exactly two forms — singular and plural are identical
/// in all three — which is what lets `to_currency` index `[1]` unguarded.
const CURRENCY_FORMS: [(&str, [&str; 2], [&str; 2]); 3] = [
    ("ETB", ["birrii", "birrii"], ["saantima", "saantima"]),
    ("USD", ["doolaara", "doolaara"], ["saantima", "saantima"]),
    ("EUR", ["yuuroo", "yuuroo"], ["saantima", "saantima"]),
];

/// Python's `parse_currency_parts` equivalent — except it is nothing of the
/// sort, because `Num2Word_OM.to_currency` does its own string surgery:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `to_currency` has already taken `abs`, so this only ever sees a
/// non-negative value; the abs is re-applied here to keep the two steps
/// together.
///
/// Consequences, all corpus-pinned:
///
/// * `right` is the *first two fraction digits, left-justified* — a digit
///   slice, not a multiplication. `0.5` → `"5"` → `"50"` → 50 saantima, while
///   `0.05` → `"05"` → 5. Anything past the second decimal is truncated with
///   no rounding: `12.349` → 34, `1.005` → 0. (Base's ROUND_HALF_UP would say
///   35 and 1 — but base's `to_currency` never runs here.)
/// * A fraction of `"0"` (i.e. `1.0`) gives `right == 0`, which `to_currency`
///   treats as falsy and drops the cents clause outright: `1.0` → "tokko
///   yuuroo", not "... zeeroo saantima". So OM's float path agrees with base's
///   *int* path here, and `has_decimal` is never consulted — `Decimal("5")`
///   and `Decimal("5.00")` both render "shan yuuroo".
/// * A `CurrencyValue::Int` stringifies without a `"."`, so `parts` has
///   length 1 and `right` stays 0 — same outcome, different route.
///
/// # The exponent-notation hole
///
/// `str(float)` flips to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal: `to_currency(1e16)`
/// raises `ValueError: invalid literal for int() with base 10: '1e+16'`.
///
/// A negative `BigDecimal` scale is exactly the "the source string used `e+`
/// notation" signal — a plain digit string parses to scale 0, never below — and
/// `str(Decimal("1E+16"))` fails identically, so that arm is reproduced.
///
/// The `e-` side is **not** reproducible at this boundary and is flagged in the
/// port report: `1e-05` and `Decimal("0.00001")` parse to the same
/// `BigDecimal` (digits 1, scale 5), yet Python raises `ValueError` for the
/// first and returns "zeeroo yuuroo" for the second. The discriminator is the
/// original `str()`, which `CurrencyValue` does not carry.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let d = match val {
        // str(int) never contains ".", so parts == [digits] and right == 0.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value, .. } => value.abs(),
    };

    // value == digits * 10^-scale
    let (digits, scale) = d.as_bigint_and_exponent();

    if scale < 0 {
        // str(d) is "1e+16"-shaped and int() rejects it. Python's message
        // quotes the offending literal verbatim; the exact spelling ("1e+16"
        // vs "1E+2") is unrecoverable once parsed, but the exception *type* is
        // what callers observe, so ValueError is what matters.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            d
        )));
    }

    // No "." in str(d): parts == [str(d)], so right stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    // `d` is non-negative, so this is a bare ASCII digit string.
    let s = digits.to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

// ---- the float / Decimal string path ------------------------------------
//
// `Num2Word_OM` **overrides `to_cardinal`**, not `to_cardinal_float`. In Python
// a float or Decimal reaches the converter as `to_cardinal(number)`, which
// renders `str(number)` *textually*: no `float2tuple`, no rounding, no
// `precision`. So the whole float/Decimal path is "reconstruct `str(number)`,
// then run OM's own string algorithm on it" — see [`LangOm::cardinal_from_pystr`].
//
// Consequences worth stating up front:
//   * Neither the banker's-rounding trap nor the f64-artefact trap applies —
//     OM reads the `repr` digits, never `abs(value - pre) * 10**precision`. So
//     `2.675` renders "ja'a torba shan" straight from the shortest `repr`.
//   * `precision_override` is irrelevant: Python's `to_cardinal(self, number)`
//     takes no such argument, so `precision=` raises `TypeError` upstream rather
//     than reaching here. The field is ignored, matching that.
//   * The Float / Decimal split is load-bearing: `str(1.1)` is "1.1" (one
//     fractional digit) but `str(Decimal("1.10"))` is "1.10" (trailing zero
//     kept) — different words. `precision` is unused for the Decimal arm too;
//     the coefficient/exponent carried by the `BigDecimal` is the spec.

/// Reconstruct Python's `str(f)` (== `repr(f)`) for an f64, digit-exact.
///
/// Rust's `{:e}` gives the shortest round-trip scientific form — the same
/// significant digits `repr` chooses — and [`py_str`] re-lays it out under
/// CPython's float rule (fixed for `-4 < decpt <= 16`, exponential otherwise,
/// always a trailing ".0" on integral values). Together they reproduce `repr`
/// at every magnitude, including the `"1e+16"` / `"1e-05"` edges that then make
/// the string algorithm raise `ValueError`.
///
/// `-0.0` is spelled `"-0.0"` (the sign comes from the bit, `is_sign_negative`,
/// not from `< 0`), which is why OM prepends the negword for it while
/// `base.float2tuple`'s `value < 0` test would not.
fn py_float_str(f: f64) -> String {
    let neg = f.is_sign_negative();
    let sci = format!("{:e}", f.abs()); // "2.675e0", "1e-2", "0e0", "1e16"
    let (mant, exp_str) = sci.split_once('e').expect("{:e} always emits an 'e'");
    let exp: i64 = exp_str.parse().expect("{:e} exponent is a base-10 integer");
    let (int_part, frac_part) = mant.split_once('.').unwrap_or((mant, ""));
    let digits = BigInt::from_str(&format!("{}{}", int_part, frac_part))
        .expect("mantissa digits are ASCII 0-9");
    let scale = frac_part.len() as i64 - exp;
    let body = py_str(&digits, scale);
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// CPython's `repr(float)` layout of `digits * 10^-scale` (digits already the
/// shortest round-trip significand). `decpt` is the point's position relative to
/// the digit string; CPython emits fixed notation for `-4 < decpt <= 16` and
/// exponential outside it — which is why `str(1e15)` is `"1000000000000000.0"`
/// but `str(1e16)` is `"1e+16"`, and `str(1e-4)` is `"0.0001"` but `str(1e-5)`
/// is `"1e-05"`.
fn py_str(digits: &BigInt, scale: i64) -> String {
    debug_assert!(!digits.is_negative(), "py_str expects abs(value)'s digits");
    let ds = digits.to_string();
    // `ds` is BigInt::to_string of a non-negative: ASCII digits only, so every
    // byte slice below lands on a char boundary by construction.
    let decpt = ds.len() as i64 - scale;

    if decpt > 16 || decpt < -3 {
        let mantissa = if ds.len() > 1 {
            format!("{}.{}", &ds[..1], &ds[1..])
        } else {
            ds
        };
        let exp = decpt - 1;
        let sign = if exp < 0 { '-' } else { '+' };
        return format!("{}e{}{:02}", mantissa, sign, exp.abs());
    }

    if decpt <= 0 {
        // 0.5 -> decpt 0; 0.01 -> decpt -1.
        format!("0.{}{}", "0".repeat((-decpt) as usize), ds)
    } else if decpt as usize >= ds.len() {
        // Integral: repr appends ".0" (Py_DTSF_ADD_DOT_0). 100.0, 1e15, ...
        format!("{}{}.0", ds, "0".repeat(decpt as usize - ds.len()))
    } else {
        format!("{}.{}", &ds[..decpt as usize], &ds[decpt as usize..])
    }
}

/// Reconstruct Python's `str(Decimal)` for `value`.
///
/// This is deliberately **not** [`py_str`]'s float rule: `Decimal.__str__` uses
/// fixed-point whenever `exp <= 0 and (exp + len(digits)) > -6`, and scientific
/// (uppercase `E`, signed exponent) otherwise — a different boundary from
/// `repr(float)`. Every corpus `cardinal_dec` row is fixed-point, but
/// `Decimal("1E-7")` really does stringify to "1E-7" and then hits
/// `int("1E-7")` -> `ValueError`; only the correct rule reproduces that. Ported
/// from CPython's `decimal.Decimal.__str__` (non-engineering branch).
fn py_decimal_str(value: &BigDecimal) -> String {
    let neg = value.is_negative();
    // `bigdecimal` does not normalise, so this hands back exactly the
    // coefficient and exponent of the Decimal the shim parsed (trailing zeros
    // of "1.10" preserved as (110, scale=2)).
    let (coeff, scale) = value.abs().as_bigint_and_exponent();
    let int_str = coeff.to_string(); // coefficient digits, non-negative ASCII
    let ndigits = int_str.len() as i64;
    let exp = -scale; // Python Decimal's `_exp`
    let leftdigits = exp + ndigits; // adjusted point position

    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1 // non-engineering scientific notation
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_str),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", int_str, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        // ASCII digit string, so byte slicing lands on char boundaries.
        let dp = dotplace as usize;
        (int_str[..dp].to_string(), format!(".{}", &int_str[dp..]))
    };

    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        // Python: `['e','E'][capitals] + "%+d" % (leftdigits - dotplace)`; the
        // default Decimal context has `capitals == 1`, i.e. uppercase 'E'.
        format!("E{:+}", leftdigits - dotplace)
    };

    let body = format!("{}{}{}", intpart, fracpart, exp_part);
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Python's `int(s)` on a slice of `str(val)`, keeping the ValueError.
///
/// `int()` also accepts surrounding whitespace and `_` separators; neither can
/// occur in a `repr` fragment, so plain `BigInt::from_str` is exact here. The
/// message is CPython's verbatim — `1e+16` and `5e` both reach a caller that
/// may be matching on it.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Python's `self.ones[int(digit)] or "zeeroo"` for one fractional character.
///
/// `int(digit)` is `ValueError` on a non-digit (e.g. the `'e'`/`'E'` of an
/// exponential `repr`), matching Python exactly. A digit char is always
/// `0..=9`, so the `ONES[...]` index can never be out of range; index 0 holds
/// the empty string, which `or "zeeroo"` turns into "zeeroo".
fn ones_or_zeeroo(ch: char) -> Result<&'static str> {
    let d = py_int(&ch.to_string())?;
    let idx = d
        .to_usize()
        .expect("a single decimal digit is 0..=9, always a valid usize");
    Ok(if ONES[idx].is_empty() {
        ZEROWORD
    } else {
        ONES[idx]
    })
}

pub struct LangOm {
    /// `setup`: `self.exclude_title = ["fi", "tuqaa", "minus"]`. Inert, because
    /// `is_title` is False, but held so the trait reports Python's state.
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS` as a lookup. Built once in `new()`, never per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(CURRENCY_FORMS.values())[0]` — ETB's pair, precomputed. Python
    /// re-evaluates this on every `to_currency` call as the `dict.get` default;
    /// the value is constant, so hoisting it changes nothing observable.
    fallback_forms: CurrencyForms,
}

impl Default for LangOm {
    fn default() -> Self {
        Self::new()
    }
}

impl LangOm {
    pub fn new() -> Self {
        // `list(CURRENCY_FORMS.values())[0]`, taken from the ordered table
        // rather than by key so it tracks the literal if that is ever reordered.
        let (_, fb_unit, fb_subunit) = CURRENCY_FORMS[0];

        LangOm {
            exclude_title: vec!["fi".to_string(), "tuqaa".to_string(), "minus".to_string()],
            currency_forms: CURRENCY_FORMS
                .iter()
                .map(|(code, unit, subunit)| (*code, CurrencyForms::new(unit, subunit)))
                .collect(),
            fallback_forms: CurrencyForms::new(&fb_unit, &fb_subunit),
        }
    }

    /// A single decimal digit as an index. Every call site below has already
    /// bounded the value to 0..=9 by construction (it is a `divmod` remainder
    /// against 10, or a quotient of a value `< 100` by 10), so the conversion
    /// cannot fail; `unwrap_or(0)` only keeps the function total.
    fn digit(n: &BigInt) -> usize {
        n.to_usize().unwrap_or(0)
    }

    /// Port of `Num2Word_OM._int_to_word`.
    ///
    /// Only ever called with a non-negative value: `to_cardinal` strips the
    /// sign before recursing. The cascade is `0 / <10 / <100 / <1000 / <10^6 /
    /// <10^9`, and anything at or above 10^9 falls through to `str(number)` —
    /// bug 1 in the module docs. All arithmetic stays in `BigInt`: the
    /// fallthrough must stringify the full value, and the corpus exercises it
    /// at 10^21, well past `u64`.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_zero() {
            return Ok(ZEROWORD.to_string());
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        // if number < 10: return self.ones[number]
        if number < &ten {
            return Ok(ONES[Self::digit(number)].to_string());
        }

        // if number < 100:
        //     t, o = divmod(number, 10)
        //     return self.tens[t] + (" fi " + self.ones[o] if o else "")
        if number < &hundred {
            let (t, o) = number.div_rem(&ten);
            let mut out = TENS[Self::digit(&t)].to_string();
            if !o.is_zero() {
                out.push_str(AND);
                out.push_str(ONES[Self::digit(&o)]);
            }
            return Ok(out);
        }

        // if number < 1000:
        //     h, r = divmod(number, 100)
        //     base = (self.ones[h] + " " if h > 1 else "") + self.hundred
        //     return base + (" fi " + self._int_to_word(r) if r else "")
        if number < &thousand {
            let (h, r) = number.div_rem(&hundred);
            let mut out = String::new();
            if h > BigInt::one() {
                out.push_str(ONES[Self::digit(&h)]);
                out.push(' ');
            }
            out.push_str(HUNDRED);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }

        // if number < 1000000:
        //     t, r = divmod(number, 1000)
        //     base = (self._int_to_word(t) + " " if t > 1 else "") + self.thousand
        //     return base + (" fi " + self._int_to_word(r) if r else "")
        if number < &million {
            let (t, r) = number.div_rem(&thousand);
            let mut out = String::new();
            if t > BigInt::one() {
                out.push_str(&self.int_to_word(&t)?);
                out.push(' ');
            }
            out.push_str(THOUSAND);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }

        // if number < 1000000000:
        //     m, r = divmod(number, 1000000)
        //     base = (self._int_to_word(m) + " " if m > 1 else "") + self.million
        //     return base + (" fi " + self._int_to_word(r) if r else "")
        if number < &billion {
            let (m, r) = number.div_rem(&million);
            let mut out = String::new();
            if m > BigInt::one() {
                out.push_str(&self.int_to_word(&m)?);
                out.push(' ');
            }
            out.push_str(MILLION);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }

        // Python: `return str(number)` -- bug 1. The biliyoona branch
        // instead, composed like the million one (#147); the maxval check
        // above keeps `b` below 1000.
        let (b, r) = number.div_rem(&billion);
        let mut out = String::new();
        if b > BigInt::one() {
            out.push_str(&self.int_to_word(&b)?);
            out.push(' ');
        }
        out.push_str(BILLION);
        if !r.is_zero() {
            out.push_str(AND);
            out.push_str(&self.int_to_word(&r)?);
        }
        Ok(out)
    }

    /// `Num2Word_OM.to_cardinal` operating on the already-reconstructed
    /// `n = str(number).strip()` — the entire float/Decimal path:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "zeeroo")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// Two Python quirks reproduced verbatim:
    /// * `int(left)` / `int(digit)` raise `ValueError` on any non-numeric
    ///   character. Once `repr` goes exponential (`str(1e16)` == "1e+16",
    ///   `str(1.5e20)` == "1.5e+20") the slices feed `int("1e+16")` / `int("e")`
    ///   and Python raises — so those inputs raise here too, via [`py_int`].
    /// * a `'0'` fractional digit renders as "zeeroo", because `self.ones[0]` is
    ///   the empty string and `"" or "zeeroo"` is `"zeeroo"`.
    fn cardinal_from_pystr(&self, n: &str) -> Result<String> {
        let n = n.trim();

        // `if n.startswith("-"): return (negword + to_cardinal(n[1:])).strip()`.
        // The sign is detached at the *string* level and the rest re-enters,
        // which is why negatives never reach `int()` with a leading minus.
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_pystr(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        match n.split_once('.') {
            Some((left, right)) => {
                // ret = _int_to_word(int(left)) + " " + pointword
                let left_int = py_int(left)?;
                let mut ret = format!("{} {}", self.int_to_word(&left_int)?, POINTWORD);
                // for digit in right: ret += " " + (ones[int(digit)] or "zeeroo")
                for ch in right.chars() {
                    ret.push(' ');
                    ret.push_str(ones_or_zeeroo(ch)?);
                }
                Ok(ret.trim().to_string())
            }
            // No ".": `return self._int_to_word(int(n))`. Reachable on the float
            // path only when `repr` produced exponential form (no dot), where
            // `int(n)` raises — reproduced by `py_int`.
            None => self.int_to_word(&py_int(n)?),
        }
    }
}

impl Lang for LangOm {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }


    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("comma nulla"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` — Python's `to_ordinal` is
    /// `"tokkoffaa"` when `number == 1`, else `to_cardinal(number) + "ffaa"`,
    /// for *any* input (no `verify_ordinal`). `number == 1` is *numeric*
    /// equality, so `1.0` and `Decimal("1.00")` both take the special word.
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let is_one = match value {
            FloatValue::Float { value: f, .. } => *f == 1.0,
            FloatValue::Decimal { value: d, .. } => d == &bigdecimal::BigDecimal::from(1),
        };
        if is_one {
            return Ok("tokkoffaa".to_string());
        }
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}ffaa", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "ffaa"` — no `== 1`
    /// special case here. `repr_str` is the dispatcher's exact `str(value)`
    /// (float repr / `Decimal.__str__`), so trailing zeros and `1E+2`-style
    /// exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}ffaa", repr_str))
    }

    /// `converter.str_to_number` — the base `Decimal(value)` parse, except the
    /// Infinity sentinel becomes the ValueError this language's own
    /// `to_cardinal` raises (`int("Infinity")` after the `"." in n` test
    /// fails); the shared dispatcher would otherwise report Base's
    /// OverflowError. NaN keeps the base sentinel: the dispatcher's
    /// ValueError for it already matches `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ETB"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "tuqaa"
    }

    // is_title is False in Python (base sets it; OM's setup leaves it alone),
    // so the trait default `false` is correct and title() stays the identity.

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_OM.to_cardinal`, integer path only.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n: ...            # unreachable for int input
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The negative branch recurses with the *string* tail `n[1:]`, which the
    /// re-entered call parses back to an int — equivalent to `int_to_word` on
    /// the absolute value, since the tail can no longer carry a sign. The
    /// trailing `.strip()` is a no-op here (negword's trailing space is always
    /// followed by a word, and `int_to_word` never pads its edges) but is kept
    /// so the shape matches the original.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        self.int_to_word(value)
    }

    /// Port of `Num2Word_OM.to_ordinal`.
    ///
    /// ```python
    /// if number == 1: return "tokkoffaa"
    /// return self.to_cardinal(number) + "ffaa"
    /// ```
    ///
    /// Only `1` is special-cased, so negatives sail through to
    /// "minus <word>ffaa" (bug 3) and values >= 10^9 become "<digits>ffaa"
    /// (bug 2). No `verify_ordinal`, so nothing raises.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("tokkoffaa".to_string());
        }
        Ok(format!("{}ffaa", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_OM.to_ordinal_num`: `str(number) + "ffaa"`.
    /// Note this overrides `Num2Word_Base.to_ordinal_num` (which returns the
    /// value untouched), and that the sign is preserved: `-1` → "-1ffaa".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}ffaa", value))
    }

    /// Port of `Num2Word_OM.to_year`: `to_cardinal(val)`, ignoring `longval`.
    /// No era handling, so negative years render with the plain minus word:
    /// `to_year(-500)` == "minus shan dhibba".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / Decimal cardinal path.
    ///
    /// OM **overrides `to_cardinal`**, not `to_cardinal_float`: in Python a float
    /// or Decimal reaches the converter as `to_cardinal(number)`, which renders
    /// `str(number)` directly with no `float2tuple`, no rounding, no `precision`.
    /// This override reconstructs `str(number)` ([`py_float_str`] /
    /// [`py_decimal_str`]) and runs OM's exact string algorithm on it
    /// ([`LangOm::cardinal_from_pystr`]).
    ///
    /// `precision_override` is ignored — Python's `to_cardinal(self, number)`
    /// takes no precision argument, so `precision=` raises `TypeError` upstream
    /// rather than reaching here; the corpus never exercises it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let n = match value {
            FloatValue::Float { value, .. } => py_float_str(*value),
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        self.cardinal_from_pystr(&n)
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for the inherited
    /// `Num2Word_Base.to_cheque`'s `NotImplementedError` message. OM's own
    /// `to_currency` never raises it (it falls back to ETB instead), so this is
    /// reachable through `to_cheque` only.
    fn lang_name(&self) -> &str {
        "Num2Word_OM"
    }

    /// `CURRENCY_FORMS[code]`.
    ///
    /// Consulted by the inherited `Num2Word_Base.to_cheque`, where a `None` is
    /// the `KeyError` that becomes `NotImplementedError`. OM's own
    /// `to_currency` deliberately does *not* route through this hook — it uses
    /// `.get(currency, <ETB>)` and never raises — so the corpus's
    /// `currency:GBP` → "zeeroo birrii" and `cheque:GBP` → NotImplementedError
    /// are both correct at once.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python:
    /// ```python
    /// def pluralize(self, n, forms):
    ///     if not forms:
    ///         return ""
    ///     return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code on every reachable OM path — `to_currency` indexes the tuple
    /// itself and the inherited `to_cheque` takes `cr1[-1]` unconditionally —
    /// but OM defines it, so it is carried rather than left to the trait
    /// default (which raises `NotImplementedError`, as
    /// `Num2Word_Base.pluralize` does).
    ///
    /// Note `forms[-1]` is the *last* form, not the second: a 3-form tuple
    /// would skip its middle entry. OM only ever has 2, both identical.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Port of `Num2Word_OM.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="ETB", cents=True, separator=" ",
    ///                 adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// A wholesale override; see the module docs for what that skips. Points of
    /// order:
    ///
    /// * Words come from `_int_to_word`, **not** `to_cardinal`, so the unit
    ///   shares its scales and 10^12 ceiling (#147; Python's digit fallback
    ///   gave `"1000000000 yuuroo"`) without inheriting the negword handling.
    ///   `is_negative` is read from the *original* value
    ///   and re-applied at the end, which is why stripping the sign up front is
    ///   safe.
    /// * The `left != 1` test runs on the **absolute** value, so
    ///   `to_currency(-1)` takes the singular: "minus tokko yuuroo".
    /// * `cents and right` is an `and` over a Python int, so a zero `right`
    ///   drops the whole clause — floats ending `.0` render like ints.
    /// * The separator is concatenated raw, with no space of its own: an
    ///   explicit `separator=","` gives "yuuroo,soddoma fi afur saantima".
    /// * `negword` carries a trailing space and Python `.strip()`s the join.
    ///   The space is always absorbed by the following word, so the strip
    ///   no-ops; reproduced via `trim()` rather than assumed away.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted separator=; resolve
        // it to this language's own default (" ") before the ported body.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is evaluated before `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // `.get(currency, list(...values())[0])` — an unknown code silently
        // borrows ETB's forms instead of raising.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        let one = BigInt::one();
        // cr1[1] if left != 1 else cr1[0] — a direct index, not pluralize().
        // Every entry in CURRENCY_FORMS has two forms, so [1] cannot panic.
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };
        let mut result = format!("{} {}", self.int_to_word(&left)?, unit);

        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            result.push_str(&format!(
                "{}{} {}",
                separator,
                self.int_to_word(&right)?,
                subunit
            ));
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;

    fn card_f(f: f64) -> Result<String> {
        let precision = match format!("{}", f).split_once('.') {
            Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
            _ => 0,
        };
        LangOm::new().to_cardinal_float(&FloatValue::Float { value: f, precision }, None)
    }

    fn card_d(s: &str) -> Result<String> {
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: 0,
        };
        LangOm::new().to_cardinal_float(&v, None)
    }

    /// `bigdecimal` must parse `str(value)` **without normalising** — the
    /// premise `py_decimal_str` and the Decimal arm rest on. A dependency bump
    /// that canonicalised `1.10` to `(11, 1)` would silently break the trailing
    /// -zero cases.
    #[test]
    fn bigdecimal_does_not_normalise() {
        for (s, want_digits, want_scale) in [
            ("1.10", "110", 2i64),
            ("5.00", "500", 2),
            ("0.01", "1", 2),
            ("0.001", "1", 3),
            ("98746251323029.99", "9874625132302999", 2),
            ("1E+16", "1", -16),
            ("1E-7", "1", 7),
        ] {
            let (d, sc) = BigDecimal::from_str(s).unwrap().as_bigint_and_exponent();
            assert_eq!(
                (d.to_string().as_str(), sc),
                (want_digits, want_scale),
                "{s}"
            );
        }
    }

    /// `py_str(parse(repr(x))) == repr(x)` across CPython's layout boundary.
    #[test]
    fn py_str_round_trips_repr() {
        for s in [
            "0.5", "0.01", "1.0", "12.34", "99.99", "1234.56", "100.0", "0.0001",
            "1000000000000000.0", "1e+16", "1e+21", "1e-05", "0.0",
        ] {
            let (d, sc) = BigDecimal::from_str(s).unwrap().as_bigint_and_exponent();
            assert_eq!(py_str(&d, sc), s);
        }
    }

    /// Float `cardinal` rows (arg with a dot), byte for byte from the corpus.
    #[test]
    fn to_cardinal_float_matches_corpus() {
        for (f, want) in [
            (0.0, "zeeroo tuqaa zeeroo"),
            (0.5, "zeeroo tuqaa shan"),
            (1.0, "tokko tuqaa zeeroo"),
            (1.5, "tokko tuqaa shan"),
            (2.25, "lama tuqaa lama shan"),
            (3.14, "sadii tuqaa tokko afur"),
            (0.01, "zeeroo tuqaa zeeroo tokko"),
            (0.1, "zeeroo tuqaa tokko"),
            (0.99, "zeeroo tuqaa sagal sagal"),
            (1.01, "tokko tuqaa zeeroo tokko"),
            (12.34, "kudhan fi lama tuqaa sadii afur"),
            (99.99, "sagaltama fi sagal tuqaa sagal sagal"),
            (100.5, "dhibba tuqaa shan"),
            (
                1234.56,
                "kuma fi lama dhibba fi soddoma fi afur tuqaa shan ja'a",
            ),
            (-0.5, "minus zeeroo tuqaa shan"),
            (-1.5, "minus tokko tuqaa shan"),
            (-12.34, "minus kudhan fi lama tuqaa sadii afur"),
            // The two f64-artefact traps: str(number) sidesteps them entirely.
            (1.005, "tokko tuqaa zeeroo zeeroo shan"),
            (2.675, "lama tuqaa ja'a torba shan"),
        ] {
            assert_eq!(card_f(f).unwrap(), want, "{f}");
        }
    }

    /// `-0.0` keeps its minus in `repr` ("-0.0"), so OM prepends negword —
    /// verified against the live interpreter.
    #[test]
    fn to_cardinal_float_negative_zero() {
        assert_eq!(card_f(-0.0).unwrap(), "minus zeeroo tuqaa zeeroo");
    }

    /// Once `repr` goes exponential the string algorithm feeds `int()` a
    /// non-numeric slice and Python raises `ValueError`; reproduced here.
    #[test]
    fn to_cardinal_float_exponential_raises() {
        // str=="1e+16"/"1e+21"/"1e-05": no dot, int("1e+16") -> ValueError.
        // str=="1.5e+20": dot, but int("e") in the fraction loop -> ValueError.
        for f in [1e16, 1e21, 1e-5, 1.5e20, 1.25e20] {
            match card_f(f) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{f}: expected ValueError, got {other:?}"),
            }
        }
    }

    /// Decimal `cardinal_dec` rows, byte for byte. The Decimal arm is exact:
    /// "1.10" keeps its trailing zero (unlike float 1.1), and issue-#603's
    /// trillion-scale value survives without a float() cast.
    #[test]
    fn to_cardinal_decimal_matches_corpus() {
        for (s, want) in [
            ("0.01", "zeeroo tuqaa zeeroo tokko"),
            ("1.10", "tokko tuqaa tokko zeeroo"),
            ("12.345", "kudhan fi lama tuqaa sadii afur shan"),
            // Past the 10^12 maxval since #147 (Python: the digits).
            ("987462513.99", "sagal dhibba fi saddeettama fi torba miliyoona fi afur dhibba fi jaatama fi lama kuma fi shan dhibba fi kudhan fi sadii tuqaa sagal sagal"),
            ("0.001", "zeeroo tuqaa zeeroo zeeroo tokko"),
            // Integral Decimal: str is "5" (no dot) -> plain _int_to_word.
            ("5", "shan"),
            ("5.00", "shan tuqaa zeeroo zeeroo"),
            ("0.0", "zeeroo tuqaa zeeroo"),
            ("-0.5", "minus zeeroo tuqaa shan"),
            ("-12.34", "minus kudhan fi lama tuqaa sadii afur"),
        ] {
            assert_eq!(card_d(s).unwrap(), want, "{s}");
        }
    }

    /// Exponential Decimals stringify with an 'E' and then hit `int()` ->
    /// ValueError, exactly like the live interpreter.
    #[test]
    fn to_cardinal_decimal_exponential_raises() {
        for s in ["1E+16", "1E-7"] {
            match card_d(s) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{s}: expected ValueError, got {other:?}"),
            }
        }
    }
}
