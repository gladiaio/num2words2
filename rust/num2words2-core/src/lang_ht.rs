//! Port of `lang_HT.py` (Haitian Creole / Kreyòl ayisyen).
//!
//! Shape: **self-contained**. `Num2Word_HT` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `__init__` never enters the `if any(hasattr(...))` branch: `self.cards` is
//! never built and **`self.MAXVAL` is never set**. `to_cardinal` is overridden
//! outright and drives a plain recursive `_int_to_word`. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is no
//! overflow check — HT never raises `OverflowError` (nor anything else; see
//! "Error behaviour" below).
//!
//! Overridden by HT (all four in-scope modes):
//!   * `to_cardinal(number)`  — string-based sign split, then `_int_to_word`.
//!   * `to_ordinal(number)`   — `to_cardinal(number) + "-yèm"`.
//!   * `to_ordinal_num(number)` — `str(number) + "."`.
//!   * `to_year(val, longval=True)` — `self.to_cardinal(val)`, i.e. exactly the
//!     `Num2Word_Base.to_year` default, so the trait default is left in place.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Everything below is linguistically wrong but
//! is exactly what Python emits, and every item is confirmed against the frozen
//! corpus:
//!
//! 1. **No teens, no vigesimal contractions.** `_int_to_word` composes 10..99
//!    as `tens[n//10] + " " + ones[n%10]`, so 11 == "dis en" (real Kreyòl:
//!    "onz"), 17 == "dis sèt" ("disèt"), 71 == "swasantdis en" ("swasanteonz"),
//!    91 == "katrevendis en" ("katrevenonz"). Corpus rows 11/15/16/17/19/71/91
//!    all confirm the naive two-word form.
//! 2. **`ones[1]` is emitted before "san"/"mil"/"milyon".** 100 == "en san"
//!    (real Kreyòl: "san"), 1000 == "en mil" ("mil"), 10^6 == "en milyon"
//!    ("yon milyon"). The hundreds arm unconditionally prefixes
//!    `self.ones[hundreds_val]`, and the thousand/million arms recurse into
//!    `_int_to_word(1)` == "en".
//! 3. **Digit fallback at 10^9.** `_int_to_word`'s final `else` arm returns
//!    `str(number)` verbatim — the *digits*, not words — for every
//!    `number >= 1_000_000_000`. So `to_cardinal(10**9)` == "1000000000" and,
//!    because `to_ordinal` just appends to the cardinal,
//!    `to_ordinal(10**9)` == "1000000000-yèm". Confirmed by corpus rows for
//!    10^9 … 10^21. This is the reason no overflow ever occurs: arbitrarily
//!    large BigInts stringify instead of raising.
//! 4. **`to_ordinal` accepts negatives and zero.** HT never calls
//!    `verify_ordinal`, so `to_ordinal(-1)` == "minus en-yèm" and
//!    `to_ordinal(0)` == "zero-yèm" rather than raising `TypeError`.
//! 5. **Zero is spelled via a falsy-string dodge.** `_int_to_word(0)` returns
//!    `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""`, which is
//!    falsy, so the branch always yields "zero". The `self.ones[0]` arm is
//!    unreachable dead code.
//!
//! # Error behaviour
//!
//! None of the four in-scope modes can raise for integer input: there is no
//! MAXVAL check, no dict lookup that can miss, no `int()` of a parsed token,
//! and no `verify_ordinal`. The corpus agrees — of 549 `ht` rows, 37 are
//! errors and **every one** is a `cheque:*`/`fraction`/currency row, i.e. out
//! of scope. All `cardinal`/`ordinal`/`ordinal_num`/`year` rows are `ok: true`.
//! Hence every function here returns `Ok`.
//!
//! # State
//!
//! No cross-call mutable state: `setup()` only assigns constant tables, and no
//! method writes to `self`. The Rust stateless path is a faithful equivalent.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dola / ewo with santim. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword` — note the **trailing space**, which `to_cardinal` relies on
/// as the separator before the number words (and `.strip()` later trims when
/// the number part is empty, which cannot actually happen here).
const NEGWORD: &str = "minus ";

/// `self.pointword` (float path only; out of scope, kept for trait fidelity).
const POINTWORD: &str = "point";

/// `self.ones`. Index 0 is `""` — see bug 5 in the module docs.
const ONES: [&str; 10] = [
    "", "en", "de", "twa", "kat", "senk", "sis", "sèt", "uit", "nèf",
];

/// `self.tens`, keyed by the tens digit. Index 0 is `""` and is unreachable
/// (the `number < 100` arm is only entered when `number >= 10`, so
/// `number // 10 >= 1`).
const TENS: [&str; 10] = [
    "",
    "dis",
    "ven",
    "trant",
    "karant",
    "senkant",
    "swasant",
    "swasantdis",
    "katreven",
    "katrevendis",
];

const HUNDRED: &str = "san";
const THOUSAND: &str = "mil";
const MILLION: &str = "milyon";

/// `Num2Word_HT.to_currency`'s own default `separator=" "`.
const HT_SEPARATOR: &str = " ";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_HT.to_currency` declares `separator=" "`, but the `Lang` trait
/// takes the separator as a plain `&str`: both `num2words2/__init__.py`'s Rust
/// fast path and `bench/diff_test.py` substitute `kwargs.get("separator", ",")`
/// — **`Num2Word_Base`'s** default, not HT's — before the value ever crosses the
/// boundary. By then "caller omitted separator" and "caller explicitly passed a
/// comma" are the same eight bits, and HT's own default is unrecoverable.
///
/// Every currency row in the frozen corpus was generated by
/// `num2words(v, lang="ht", to="currency", currency=c)` with no `separator=`,
/// and all 54 cents-bearing rows expect `" "`:
///
/// ```text
/// {"lang": "ht", "to": "currency:EUR", "arg": "12.34",
///  "out": "dis de euros trant kat cents"}
/// ```
///
/// So `","` is read back as "unset" and HT's default restored. The one input
/// this gets wrong is an explicit `separator=","`, which yields `" "` where
/// Python yields `","` (Python emits `'dis de euros,trant kat cents'` — the
/// separator is concatenated raw, with no space added). That case is not
/// expressible at this ABI and is flagged in the port report. `lang_es.rs`,
/// `lang_ca.rs` and `lang_eu.rs` resolve the identical conflict the same way.
const SEPARATOR_UNSET: &str = ",";

/// The `_int_to_word` fallback threshold: `number < 1000000000` is the last
/// worded arm, so anything `>= 10^9` stringifies. See bug 3 in the module docs.
fn billion() -> BigInt {
    BigInt::from(1_000_000_000u64)
}

/// Port of `Num2Word_HT._int_to_word`, guard order preserved.
///
/// Python's arm order is `== 0`, `< 0`, `< 10`, `< 100`, `< 1000`, `< 10^6`,
/// `< 10^9`, `else str(number)`. The first three arms are handled here; the
/// rest are delegated to [`int_to_word_small`] once the value is *proven* to
/// be below 10^9 and therefore safe to narrow to `u64`.
fn int_to_word(number: &BigInt) -> String {
    // `self.ones[0] if self.ones[0] else "zero"` — ones[0] == "" is falsy.
    if number.is_zero() {
        return "zero".to_string();
    }

    // Dead code in practice: `to_cardinal` strips the sign before calling, and
    // every recursive call passes a non-negative quotient or remainder. Ported
    // anyway so the arm order matches the Python source 1:1.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    // `else: return str(number)` — the digit fallback for >= 10^9.
    if number >= &billion() {
        return number.to_string();
    }

    // Proven `0 < number < 10^9` here, so the narrowing cannot lose data
    // (10^9 < u64::MAX). PORTING.md permits the cast on a proven bound.
    let n = number
        .to_u64()
        .expect("bounded above by 10^9 and below by 1 — narrowing is total");
    int_to_word_small(n)
}

/// The worded arms of `_int_to_word`, restricted to the proven `< 10^9` domain.
///
/// Keeps the `n == 0 -> "zero"` head so the recursion mirrors Python's
/// `_int_to_word` exactly, even though every call site guards against 0
/// (`if remainder:`, and `number // 1000` / `number // 1000000` are `>= 1` by
/// the enclosing range check).
fn int_to_word_small(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    if n < 10 {
        ONES[n as usize].to_string()
    } else if n < 100 {
        let tens_val = (n / 10) as usize;
        let ones_val = (n % 10) as usize;
        if ones_val == 0 {
            TENS[tens_val].to_string()
        } else {
            // Bug 1: naive composition, no teens and no vigesimal contraction.
            format!("{} {}", TENS[tens_val], ONES[ones_val])
        }
    } else if n < 1000 {
        let hundreds_val = (n / 100) as usize;
        let remainder = n % 100;
        // Bug 2: `ones[1]` is emitted, so 100 == "en san".
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else if n < 1_000_000 {
        let thousands_val = n / 1000;
        let remainder = n % 1000;
        let mut result = format!("{} {}", int_to_word_small(thousands_val), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else {
        // n < 1_000_000_000 guaranteed by the caller.
        let millions_val = n / 1_000_000;
        let remainder = n % 1_000_000;
        let mut result = format!("{} {}", int_to_word_small(millions_val), MILLION);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    }
}

/// Port of `Num2Word_HT.to_currency`'s decimal-string dissection:
///
/// ```python
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// HT never consults `CURRENCY_PRECISION` and never calls
/// `parse_currency_parts`; it slices the *text* of the number. So this works
/// off `str(val)` rather than any arithmetic, and the two quirks that follow
/// from that are preserved:
///
/// * **Cents are truncated, never rounded.** `parts[1][:2]` drops everything
///   past the second decimal digit, so `1234.567` -> 56 cents (not 57).
/// * **Short fractions are right-padded.** `.ljust(2, "0")` turns `str(0.5)`'s
///   `"5"` into `"50"`, so `0.5` is fifty cents, not five.
///
/// `val` is already absolute here: Python does `val = abs(val)` *before*
/// `str(val)`.
///
/// # Reconstructing `str(val)`
///
/// The binding parses `str(value)` into a `BigDecimal` before we see it, so the
/// original text is gone and is rebuilt from `as_bigint_and_exponent()`
/// (mantissa + scale), which round-trips it exactly. `BigDecimal`'s own
/// `Display` is *not* usable for this: it renders `Decimal("1E+2")` as `"100"`,
/// where Python's `str` keeps `"1E+2"` — a difference this function must see,
/// because `int("1E+2")` raises where `int("100")` does not.
///
/// Python's `str()` switches to exponent notation for large magnitudes, and
/// `int("1e+21")` then raises `ValueError`. A negative scale is exactly that
/// case for both sources — `str(1e16) == "1e+16"` and
/// `str(Decimal("1E+2")) == "1E+2"` both parse to `scale < 0` — so it maps to
/// `N2WError::Value`, matching Python. `scale == 0` is `Decimal("100")`, whose
/// `str` is plain `"100"` with no `"."`: a single part, no cents, no error.
///
/// # Known gap: small-magnitude floats (`0 < |val| < 1e-4`)
///
/// Python also uses exponent notation *below* `1e-4`, but only for `float`:
///
/// | input | `str(input)` | Python `to_currency` |
/// |---|---|---|
/// | `1e-05` (float) | `"1e-05"` | `ValueError` |
/// | `Decimal("0.00001")` | `"0.00001"` | `"zero euros"` |
///
/// Both parse to the *same* `BigDecimal` (mantissa 1, scale 5), because
/// `BigDecimal::from_str` normalises `"1e-05"` and `"0.00001"` to one value.
/// `CurrencyValue::Decimal` carries no float/Decimal tag, so the distinction is
/// unrecoverable here and no rule can satisfy both. This takes the
/// `Decimal`-correct branch (`"zero euros"`), which also stays correct for every
/// float `|val| >= 1e-4` — including the corpus's smallest, `0.01`, and
/// `0.001`, whose `str` is still positional. A float in `(0, 1e-4)` is the one
/// input that diverges: Python raises `ValueError`, this returns `"zero euros"`.
/// Nothing in the corpus reaches it; flagged in the port report.
fn split_currency_parts(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let d = match val {
        // A true `int`: `str(val)` is bare digits, so `split(".")` yields one
        // part and `right` stays 0. `abs()` mirrors Python's `val = abs(val)`.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    let (mantissa, scale) = d.as_bigint_and_exponent();

    if scale <= 0 {
        if scale == 0 {
            // `str` has no ".", e.g. Decimal("100") -> parts == ["100"].
            return Ok((mantissa, BigInt::zero()));
        }
        // Exponent notation: Python's `int("1e+21")` raises ValueError.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            d
        )));
    }

    // `mantissa` is non-negative (we took `abs`), so this is ASCII digits only.
    let digits = mantissa.to_string();
    let scale = scale as usize;
    // Left-pad so there is at least one integer digit: 0.5 -> mantissa 5,
    // scale 1 -> "05", matching str(0.5) == "0.5" -> parts[0] == "0".
    let padded = if digits.len() <= scale {
        format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits)
    } else {
        digits
    };
    let split_at = padded.len() - scale;

    let int_part: String = padded.chars().take(split_at).collect();
    let frac_part: String = padded.chars().skip(split_at).collect();

    // `int(parts[0]) if parts[0] else 0` — the padding above keeps this
    // non-empty, so the `else 0` arm is unreachable, as in Python.
    let left = BigInt::from_str(&int_part).map_err(|e| N2WError::Value(e.to_string()))?;

    // `parts[1][:2].ljust(2, "0")`.
    let mut frac: String = frac_part.chars().take(2).collect();
    while frac.chars().count() < 2 {
        frac.push('0');
    }
    let right = BigInt::from_str(&frac).map_err(|e| N2WError::Value(e.to_string()))?;

    Ok((left, right))
}

/// `str(x)` for a non-finite float, for the `int()` ValueError message.
/// Python: `str(float('inf')) == "inf"`, `str(float('nan')) == "nan"`.
fn python_nonfinite_str(v: f64) -> String {
    if v.is_nan() {
        "nan".to_string()
    } else if v > 0.0 {
        "inf".to_string()
    } else {
        "-inf".to_string()
    }
}

/// Best-effort reconstruction of Python's `str(float)` scientific form (e.g.
/// `"1e+16"`, `"1e-05"`, `"1.5e-05"`), used only for the ValueError *message*
/// on a scientific-notation float. The exact repr is not recoverable here —
/// that is the whole reason the float path is handed a Python-computed
/// `precision` — so this normalises Rust's `{:e}` to Python's punctuation
/// (explicit sign, exponent zero-padded to two digits). The message is not
/// exercised by the corpus; only the `ValueError` *variant* is load-bearing.
fn py_float_sci_string(v: f64) -> String {
    let s = format!("{:e}", v);
    match s.split_once('e') {
        Some((mant, exp)) => {
            let (sign, mag) = match exp.strip_prefix('-') {
                Some(rest) => ("-", rest),
                None => ("+", exp),
            };
            let mag = if mag.len() < 2 {
                format!("0{}", mag)
            } else {
                mag.to_string()
            };
            format!("{}e{}{}", mant, sign, mag)
        }
        None => s,
    }
}

/// Rebuild Python's `str(Decimal)` scientific form from `(mantissa, scale)`,
/// following the decimal `to-scientific-string` algorithm, for the `int()`
/// ValueError message. Only called on the scientific branch. Examples:
/// `Decimal("1E+2")` -> `"1E+2"`, `Decimal("9.9E+3")` -> `"9.9E+3"`,
/// `Decimal("1E-7")` -> `"1E-7"`.
fn decimal_scientific_string(mantissa: &BigInt, scale: i64) -> String {
    let exp = -scale;
    let negative = mantissa.is_negative();
    let digits = mantissa.abs().to_string();
    let adjusted = exp + digits.chars().count() as i64 - 1;

    let mut s = String::new();
    if negative {
        s.push('-');
    }
    let mut chars = digits.chars();
    s.push(chars.next().unwrap_or('0'));
    let rest: String = chars.collect();
    if !rest.is_empty() {
        s.push('.');
        s.push_str(&rest);
    }
    s.push('E');
    if adjusted >= 0 {
        s.push('+');
    }
    s.push_str(&adjusted.to_string());
    s
}

/// If `str(number)` is in scientific notation, return the token HT would feed
/// to `int()` (which raises `ValueError`); otherwise `None`.
///
/// * **float** — CPython's shortest repr (mode `'r'`) uses exponential form
///   iff `decpt <= -4 || decpt > 16`, i.e. `|v| < 1e-4` or `|v| >= 1e16`
///   (`v == 0.0` stays positional: `"0.0"`). Non-finite values (`inf`/`nan`)
///   have no `"."` either, so they take the same ValueError path.
/// * **Decimal** — `Decimal.__str__` is positional iff `exp <= 0` and the
///   adjusted exponent `exp + len(digits) - 1 >= -6`, else exponential.
///   `exp == -scale` and `len(digits)` come from `as_bigint_and_exponent`.
fn scientific_int_token(value: &FloatValue) -> Option<String> {
    match value {
        FloatValue::Float { value: v, .. } => {
            if !v.is_finite() {
                return Some(python_nonfinite_str(*v));
            }
            if *v == 0.0 {
                return None;
            }
            let a = v.abs();
            if a >= 1e16 || a < 1e-4 {
                Some(py_float_sci_string(*v))
            } else {
                None
            }
        }
        FloatValue::Decimal { value: d, .. } => {
            let (mantissa, scale) = d.as_bigint_and_exponent();
            let exp = -scale;
            let ndigits = if mantissa.is_zero() {
                1
            } else {
                mantissa.abs().to_string().chars().count() as i64
            };
            let adjusted = exp + ndigits - 1;
            if exp <= 0 && adjusted >= -6 {
                None
            } else {
                Some(decimal_scientific_string(&mantissa, scale))
            }
        }
    }
}

/// Exact `BigDecimal` value of a **non-negative, finite** `f64`.
///
/// An `f64` is a dyadic rational `mantissa * 2^exp2`, so it has an exact finite
/// decimal expansion (`2^-k == 5^k / 10^k`). Reconstructing it exactly — rather
/// than via `f64`'s shortest `{}` formatter — is essential: Rust's shortest
/// float formatter and Python's `repr` disagree on the round-half tie
/// (`816128197856440.25f64` prints as `...440.3` in Rust but `...440.2` in
/// Python), so the formatter cannot be used to recover Python's repr digits.
fn f64_to_exact_bigdecimal(v: f64) -> BigDecimal {
    debug_assert!(v.is_finite() && v >= 0.0);
    if v == 0.0 {
        return BigDecimal::from(0);
    }
    let bits = v.to_bits();
    let exp_field = ((bits >> 52) & 0x7ff) as i64;
    let mant_field = (bits & 0x000f_ffff_ffff_ffff) as u64;
    // v == mant * 2^exp2 (IEEE-754 double; hidden bit set for normals).
    let (mant, exp2) = if exp_field == 0 {
        (mant_field, -1074) // subnormal
    } else {
        (mant_field | 0x0010_0000_0000_0000, exp_field - 1075)
    };
    let mant_bi = BigInt::from(mant);
    if exp2 >= 0 {
        BigDecimal::from(mant_bi * BigInt::from(2).pow(exp2 as u32))
    } else {
        let k = (-exp2) as u32;
        // mant * 2^-k == (mant * 5^k) * 10^-k
        BigDecimal::new(mant_bi * BigInt::from(5).pow(k), k as i64)
    }
}

/// Round a **non-negative** `BigDecimal` to an integer, half-to-**even** —
/// Python's `round()` / shortest-repr tie rule. Pure `BigInt` arithmetic so
/// there is no `f64` precision loss (the reason `float2tuple`'s
/// `frac * 10**precision` drops a digit for long reprs like
/// `0.00021738279773348277`).
fn round_half_even_nonneg(bd: &BigDecimal) -> BigInt {
    let (mant, scale) = bd.as_bigint_and_exponent();
    if scale <= 0 {
        // Already an integer: mant * 10^(-scale).
        return mant * BigInt::from(10).pow((-scale) as u32);
    }
    let divisor = BigInt::from(10).pow(scale as u32);
    // mant >= 0, so `/` and `%` floor.
    let floor = &mant / &divisor;
    let rem = &mant % &divisor;
    match (BigInt::from(2) * &rem).cmp(&divisor) {
        Ordering::Less => floor,
        Ordering::Greater => &floor + BigInt::one(),
        Ordering::Equal => {
            if (&floor % BigInt::from(2)).is_zero() {
                floor
            } else {
                &floor + BigInt::one()
            }
        }
    }
}

/// Reconstruct `str(number)` for a *positional* (non-scientific) value as
/// `(is_negative, integer_part, Some(fraction_digits))`, or `None` fraction
/// when `str(number)` has no decimal point.
///
/// This is the sign / left / right split `Num2Word_HT.to_cardinal` performs on
/// `str(number)`, reproduced from the typed value. Both arms reduce to: take
/// the exact value, `pre = trunc(|value|)`, and emit `precision` fractional
/// digits obtained by rounding `frac(|value|) * 10**precision` half-to-even —
/// which is exactly the fixed-length shortest-repr digit string Python prints.
///
/// * **float** — `precision` is Python's repr length (`repr(float)` always
///   shows a point, so it is `>= 1` and a fraction is always present). The
///   exact `BigDecimal` of the `f64` is used so the digits match Python's
///   `repr` in every regime — both long low-magnitude reprs (where
///   `float2tuple` loses the last digit) and round-half ties (where Rust's
///   formatter picks the wrong digit). The sign is `is_sign_negative`, so
///   `-0.0` keeps its minus like `str(-0.0) == "-0.0"`.
/// * **Decimal** — `BigDecimal`'s own value is already exact; `precision` is
///   `abs(exponent)`. `precision == 0` is a bare integer (`str` has no point).
///   `BigDecimal`'s `Display` is *not* used (it renders `1E+2` as `100`, where
///   Python keeps `1E+2`), which is why scientific Decimals are rejected
///   upstream rather than reaching here.
fn positional_parts(value: &FloatValue) -> (bool, BigInt, Option<String>) {
    let (exact_abs, precision, neg) = match value {
        FloatValue::Float { value: v, precision } => {
            (f64_to_exact_bigdecimal(v.abs()), *precision, v.is_sign_negative())
        }
        FloatValue::Decimal { value: d, precision } => {
            (d.abs(), *precision, d.is_negative())
        }
    };

    // pre = trunc(|value|) toward zero; exact_abs >= 0, so this is floor.
    let (mant, scale) = exact_abs.as_bigint_and_exponent();
    let pre = if scale <= 0 {
        mant.clone() * BigInt::from(10).pow((-scale) as u32)
    } else {
        &mant / &BigInt::from(10).pow(scale as u32)
    };

    if precision == 0 {
        // No fractional part in str(number) (Decimal integers only).
        return (neg, pre, None);
    }

    // frac(|value|) * 10**precision, exact, then round half-to-even.
    let frac_scaled = (&exact_abs - BigDecimal::from(pre.clone()))
        * BigDecimal::from(BigInt::from(10).pow(precision));
    let post = round_half_even_nonneg(&frac_scaled);

    // Left-pad to exactly `precision` digits, as str(number)'s fraction is.
    let mut frac = post.to_string();
    let p = precision as usize;
    if frac.chars().count() < p {
        frac = format!("{}{}", "0".repeat(p - frac.chars().count()), frac);
    }
    (neg, pre, Some(frac))
}

pub struct LangHt {
    /// `Num2Word_HT.CURRENCY_FORMS`, built once here rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the fallback `to_currency`
    /// uses for an unknown code. Python dicts iterate in insertion order and
    /// `CURRENCY_FORMS` is written HTG, USD, EUR, so this is **HTG**. Cached
    /// because a `HashMap` has no first element to take.
    fallback_forms: CurrencyForms,
}

impl Default for LangHt {
    fn default() -> Self {
        Self::new()
    }
}

impl LangHt {
    pub fn new() -> Self {
        // `Num2Word_HT` defines CURRENCY_FORMS itself and inherits nothing from
        // lang_EUR/lang_EU, so English's import-time mutation of the shared
        // Num2Word_EUR class dict never reaches it: HT's EUR really is
        // ("euro", "euros") in its own source, and HT sees none of the ~24
        // extra codes EN adds. Verified against the live interpreter.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "HTG",
            CurrencyForms::new(&["goud", "goud"], &["santim", "santim"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dola", "dola"], &["santim", "santim"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["ewo", "ewo"], &["santim", "santim"]),
        );
        LangHt {
            fallback_forms: currency_forms["HTG"].clone(),
            currency_forms,
        }
    }
}

impl Lang for LangHt {

    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("comma nulla"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` — Python's `to_ordinal` is
    /// `to_cardinal(number) + "-yèm"` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "senk point zero-yèm".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}-yèm", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is the
    /// dispatcher's exact `str(value)` (float repr / `Decimal.__str__`), so
    /// trailing zeros and `1E+2`-style exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
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
        "HTG"
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
        "point"
    }

    /// Port of `Num2Word_HT.to_cardinal`, integer path only.
    ///
    /// Python works on the *string*:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword          # "minus " — trailing space included
    /// else:
    ///     ret = ""
    /// ...
    /// return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// For integer input `str(number)` never contains `"."`, so the float
    /// branch (`pointword` + per-digit decimals) is unreachable and out of
    /// scope. Stripping a leading `"-"` from `str(value)` and re-parsing is
    /// exactly `value.abs()`, so the round-trip through text is elided.
    ///
    /// The final `.strip()` only ever matters if `_int_to_word` returned `""`,
    /// which it cannot (0 yields "zero", and every other arm yields a word or
    /// the digit fallback). It is kept for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        Ok(format!("{}{}", ret, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Port of `Num2Word_HT.to_ordinal`: `self.to_cardinal(number) + "-yèm"`.
    ///
    /// No `verify_ordinal` call, so negatives and zero pass straight through
    /// (bug 4), and the 10^9 digit fallback leaks into the ordinal (bug 3):
    /// `to_ordinal(10**9)` == "1000000000-yèm".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}-yèm", cardinal))
    }

    /// Port of `Num2Word_HT.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Note HT *overrides* `Num2Word_Base.to_ordinal_num` (which returns the
    /// value untouched), so the trait default is not sufficient here. The sign
    /// is preserved: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_HT.to_cardinal`'s float/Decimal branch.
    ///
    /// HT overrides `to_cardinal` (not `to_cardinal_float`) and drives the
    /// whole float path off `str(number)`:
    ///
    /// ```python
    /// n = str(number).strip()
    /// # strip a leading "-": ret = self.negword ("minus ") else ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// Two consequences drive this port:
    ///
    /// * **The `precision=` kwarg never reaches `to_cardinal`** (which takes no
    ///   `precision` parameter), so it is ignored here — verified live:
    ///   `num2words(2.675, lang="ht", precision=1)` == the un-kwarg'd result.
    ///   Hence `precision_override` is dropped and `None` passed downstream.
    ///
    /// * **Scientific-notation `str(number)` raises `ValueError`.** HT then
    ///   feeds an exponent token to `int()` — `int("1e+16")`, or, for a
    ///   mantissa like `"1.5e-05"`, per-char `int("e")`. Reproduced as
    ///   `N2WError::Value` with the same `invalid literal for int()...` message
    ///   shape (see [`scientific_int_token`]).
    ///
    /// This port re-derives `str(number)` and runs HT's per-character split
    /// rather than delegating to `float2tuple`, because two reachable classes
    /// of input make the base float path diverge from HT:
    ///
    /// * **Negative zero.** `str(-0.0) == "-0.0"`, so HT prepends "minus"
    ///   (`"-0.0".startswith("-")`), but `float2tuple`'s sign test is the
    ///   numeric `value < 0`, and `-0.0 < 0.0` is false — it would drop the
    ///   sign. The reconstruction uses `f64::is_sign_negative`, which matches
    ///   the string test (true for `-0.0`, false for `+0.0`).
    /// * **Small high-precision floats.** For e.g. `0.00021738279773348277`
    ///   (repr precision 20), HT reads the exact repr digits, but
    ///   `float2tuple` computes `frac * 10**20` in f64 and floors, losing the
    ///   final digit — the two disagree on the last word. Reading the digits
    ///   straight from the shortest repr (Rust's `{}` == Python's, both
    ///   shortest round-trip) is what HT actually does.
    ///
    /// Scientific-notation `str(number)` still raises `ValueError`
    /// (see [`scientific_int_token`]).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Scientific notation: HT feeds an exponent token to int() -> ValueError.
        if let Some(token) = scientific_int_token(value) {
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                token
            )));
        }

        // Reconstruct str(number) as (sign, integer part, optional fraction).
        let (neg, pre, frac) = positional_parts(value);

        // Mirror Num2Word_HT.to_cardinal's string assembly exactly:
        //   ret = "minus " if negative else ""
        //   ret += _int_to_word(int(left)) [+ " point " + per-digit words]
        //   return ret.strip()
        let mut ret = String::new();
        if neg {
            ret.push_str(NEGWORD); // "minus " — trailing space preserved
        }
        ret.push_str(&int_to_word(&pre));

        if let Some(frac) = frac {
            ret.push(' ');
            ret.push_str(POINTWORD); // self.pointword == "point"
            for ch in frac.chars() {
                // `int(digit)` for a single character: always 0..=9 here, but
                // reproduce the ValueError shape defensively for parity.
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                ret.push(' ');
                ret.push_str(&int_to_word(&BigInt::from(d)));
            }
        }

        Ok(ret.trim().to_string())
    }

    // to_year: HT's override is `return self.to_cardinal(val)`, which is
    // byte-identical to the `Num2Word_Base.to_year` default the trait already
    // implements (it delegates through `&self` and so picks up the
    // `to_cardinal` override above). The `longval=True` parameter is accepted
    // and ignored by Python. Left at the default deliberately.

    // ---- currency ------------------------------------------------------
    //
    // `Num2Word_HT` overrides `to_currency` outright and inherits everything
    // else from `Num2Word_Base`. Hooks deliberately left at their defaults:
    //
    //   * `currency_precision` — HT inherits `CURRENCY_PRECISION = {}` (it
    //     defines none), so every code takes `.get(code, 100)`'s default of
    //     100. HT's own `to_currency` never reads it at all, and `to_cheque`
    //     reads it only for codes that reach past the forms lookup. This is
    //     why `currency:KWD`/`currency:BHD` are *not* 3-decimal here and
    //     `currency:JPY` is *not* 0-decimal: HT's `to_currency` ignores
    //     precision entirely, and none of those codes are even in its table
    //     (see `fallback_forms`). The corpus confirms all three:
    //     `{"to": "currency:JPY", "arg": "12.34", "out": "dis de goud trant
    //     kat santim"}` — cents shown, HTG words, no rounding.
    //   * `currency_adjective` — HT inherits `CURRENCY_ADJECTIVES = {}`.
    //   * `pluralize` — abstract in `Num2Word_Base`. HT never calls it: its
    //     `to_currency` inlines `cr1[1] if left != 1 else cr1[0]`, and
    //     `to_cheque` takes `cr1[-1]`. So the default (which raises
    //     NotImplementedError, exactly as Python's does) is correct and
    //     unreachable.
    //   * `money_verbose` / `cents_verbose` / `cents_terse` — not overridden
    //     by HT; the defaults already delegate to `to_cardinal`.
    //   * `to_cheque` — HT does not override it, and `Num2Word_Base.to_cheque`
    //     is reproduced exactly by `currency::default_to_cheque`: the forms
    //     lookup is a bare `self.CURRENCY_FORMS[currency]`, so it raises
    //     `NotImplementedError` for an unknown code instead of falling back
    //     the way `to_currency` does. Hence the corpus's split verdict on the
    //     same input — `cheque:EUR 1234.56` succeeds, `cheque:GBP 1234.56`
    //     raises. Traced below.
    //   * `cardinal_from_decimal` — HT's `to_currency` never produces
    //     fractional cents (`right` is always a whole `int` from a 2-char
    //     slice), so the float path is genuinely unreachable from here.

    /// For the `Currency code "%s" not implemented for "%s"` message that
    /// `to_cheque` raises. `self.__class__.__name__` is `Num2Word_HT`.
    fn lang_name(&self) -> &str {
        "Num2Word_HT"
    }

    /// `CURRENCY_FORMS[code]` — a plain lookup with **no** fallback.
    ///
    /// The HTG fallback belongs to `to_currency`'s `.get(currency, ...)` alone.
    /// `to_cheque` subscripts the dict directly and must still raise for an
    /// unknown code, so applying the fallback here would wrongly turn
    /// `cheque:GBP` into `"... GOUD"` instead of `NotImplementedError`.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_HT.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="HTG", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
    ///
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///
    ///     if is_negative:
    ///         result = self.negword + result
    ///
    ///     return result.strip()
    /// ```
    ///
    /// This shares nothing with `Num2Word_Base.to_currency` — no
    /// `parse_currency_parts`, no `pluralize`, no `CURRENCY_PRECISION`, no
    /// `_money_verbose` — so it does **not** delegate to
    /// `currency::default_to_currency`.
    ///
    /// # Faithfully reproduced Python bugs
    ///
    /// 6. **An unknown currency code silently becomes HTG.** `.get(currency,
    ///    list(self.CURRENCY_FORMS.values())[0])` falls back to the first dict
    ///    value rather than raising, so `currency:GBP` renders Haitian gourdes:
    ///    `to_currency(0, "GBP")` == "zero goud". Only HTG/USD/EUR are real;
    ///    GBP, JPY, KWD, BHD, INR, CNY and CHF — 7 of the corpus's 9 codes —
    ///    all silently print "goud"/"santim". `to_cheque` does *not* share this
    ///    fallback, which is why it raises on the same codes.
    /// 7. **`adjective` is accepted and ignored.** The parameter is never read,
    ///    so `adjective=True` changes nothing. (Moot in practice: HT inherits
    ///    an empty `CURRENCY_ADJECTIVES`, so the base class would also be a
    ///    no-op here.)
    /// 8. **`cents=False` drops the cents segment entirely** rather than
    ///    switching to the terse digit form `_cents_terse` gives. The guard is
    ///    `if cents and right`, so `to_currency(12.34, cents=False)` ==
    ///    "dis de euros", losing the 34 cents outright.
    /// 9. **A whole float shows no cents**, unlike `Num2Word_Base`. Base keys
    ///    the cents segment off `isinstance(val, int)`, so a float `1.0` still
    ///    renders "... zero cents"; HT keys it off `if ... right` — a *value*
    ///    test — so `1.0` -> "en euro" and `100` -> "en san euros" collapse to
    ///    the same shape. The int/float split still matters for the general
    ///    case and is preserved via `CurrencyValue`, but for HT it only decides
    ///    whether `str(val)` has a `"."` to split on at all.
    /// 10. **The 10^9 digit fallback leaks into money.** `_int_to_word` is used
    ///    directly, so `to_currency(1e9, "EUR")` == "1000000000 euros".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore HT's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            HT_SEPARATOR
        } else {
            separator
        };
        // Bug 7: Python binds `adjective` and never reads it.
        let _ = adjective;

        // `if val < 0: is_negative = True; val = abs(val)`. The `abs` is folded
        // into split_currency_parts, which Python reaches only after it.
        let is_negative = val.is_negative();
        let (left, right) = split_currency_parts(val)?;

        // Bug 6: `.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::one();
        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // Bugs 8 and 9: a *value* test, not a type test, and not a terse form.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        if is_negative {
            // `self.negword` keeps its trailing space: "minus " + "de euros".
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
