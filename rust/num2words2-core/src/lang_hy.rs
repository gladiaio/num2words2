//! Port of `lang_HY.py` (Armenian).
//!
//! Shape: **engine, with a self-contained wrapper**. `Num2Word_HY` subclasses
//! `Num2Word_Base` and *does* define `high_numwords`/`mid_numwords`/
//! `low_numwords` in `setup`, so Python builds `self.cards` and sets
//! `MAXVAL = 1000 * cards.keys()[0]`. But it also overrides `to_cardinal` with
//! a pre-filter that short-circuits 0, 1000, the millions range and the
//! billions range, delegating everything else to `super().to_cardinal()` (the
//! `splitnum`/`clean`/`merge` engine) and then post-processing the result.
//!
//! So this file supplies `cards` + `maxval` + `merge` (engine duties) *and*
//! overrides `to_cardinal` to reproduce the wrapper. `default_to_cardinal` is
//! the `super()` call.
//!
//! Inherited from `Num2Word_Base` and reused unchanged:
//!   * `verify_ordinal` → `TypeError` on negatives (see [`LangHy::verify_ordinal`]).
//!   * `title`          → `is_title` is False for HY, so it is a no-op. HY's
//!     `exclude_title` (`["և", "ամբողջ", "մինուս"]`) is therefore dead but is
//!     carried here for fidelity.
//!   * `negword` = "մինուս " (set in `setup`, after `__init__`'s "(-) " default).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong-looking but
//! are exactly what CPython emits, verified against the interpreter and the
//! frozen corpus:
//!
//! 1. **`set_high_numwords` stores tuples as card words.** `setup` builds
//!    `high_numwords = [(10**12, "տրիլիոն"), (10**9, "միլիարդ"),
//!    (10**6, "միլիոն")]` — a list of *pairs* — but `set_high_numwords`
//!    treats each element as a bare word:
//!    ```python
//!    max = 3 + 10 * len(high)          # 33
//!    for word, n in zip(high, range(max, 3, -10)):   # n = 33, 23, 13
//!        self.cards[10**n] = word      # word is the TUPLE, not the string
//!    ```
//!    So `cards[10**33] = (10**12, "տրիլիոն")`, `cards[10**23] =
//!    (10**9, "միլիարդ")`, `cards[10**13] = (10**6, "միլիոն")`. Three separate
//!    errors compound here: the step is -10 instead of -3, the exponents are
//!    off by ten orders of magnitude (10**13 for "million"), and the value is
//!    a tuple. `merge` then interpolates the tuple with `"%s %s"`, so its
//!    `str()` leaks into the output:
//!    `to_cardinal(10**15)` == "հարյուր (1000000, 'միլիոն')".
//!    Modelled by storing the tuple's Python `str()` as the card word — see
//!    [`TUPLE_CARD_E33`] and friends. That substitution is exact for every
//!    reachable input; the reasoning is spelled out on those constants.
//!    Consequence: there is **no** card for 10**6, 10**9 or 10**12, so the
//!    engine's largest usable card below 10**13 is 1000 ("հազար").
//! 2. **`merge` adds where it should multiply.** Every arm returns
//!    `cnum + nnum`, so 200 is tracked as 2+100=102 and 10**6 as 2000. The
//!    numbers are only used for `merge`'s own comparisons, so the *text* still
//!    comes out right for small values — but it is why 10**12 renders as four
//!    "հազար" in a row (see bug 3) and why `nnum < cnum` tests behave oddly.
//! 3. **The "հազար հազար" → "միլիոն" patch.** Because there is no million
//!    card, the engine renders 10**6 as "հազար հազար" and 10**12 as
//!    "հազար հազար հազար հազար". `to_cardinal` papers over this with a string
//!    `.replace()`, which is non-overlapping and left-to-right, so 10**12
//!    becomes "միլիոն միլիոն" rather than anything sensible. Reproduced
//!    verbatim; Rust's `str::replace` has identical semantics.
//! 4. ~~**`to_ordinal` raises `KeyError` on every negative.**~~ Fixed
//!    (gladiaio/num2words2#158). `value < 20` and `value < 10` are both true
//!    for negatives, so Python reached `ORDINAL_ONES[value]` with a missing
//!    key, while `to_ordinal_num` (which calls `verify_ordinal`) raised
//!    `TypeError` — the two modes disagreed. `to_ordinal` now raises the same
//!    `errmsg_negord` `TypeError`, and a fractional float/Decimal
//!    `errmsg_floatord` instead of a `KeyError`.
//! 5. **`to_ordinal` just glues "երորդ" onto the cardinal above 100**, with no
//!    stem adjustment, so `to_ordinal(110)` == "հարյուր տասըերորդ" and
//!    `to_ordinal(999)` == "ինը հարյուր իննսուն ինըերորդ".
//! 6. **`to_year`'s "մեկ հազար" strip is dead code.** `merge` already returns
//!    `next` unchanged when `cnum == 1 and nnum == 1000`, so a cardinal can
//!    never begin with "մեկ հազար" and the `year_str[4:]` branch never fires.
//!    Ported anyway (as a character slice, not a byte slice).
//! 7. **`merge`'s `ntext[:-1] + "ի"` genitive branch is unreachable.** It needs
//!    `nnum < cnum`, `100 <= cnum < 1000` and `nnum % 100 == 0`; `splitnum`
//!    only ever hands a sub-100 remainder to a hundreds-range `cnum`, and a
//!    zero remainder is never appended. Ported anyway.
//!
//! # The currency surface
//!
//! `to_currency` is overridden **completely** and shares nothing with
//! `Num2Word_Base.to_currency` — see [`LangHy::to_currency`] for the three
//! consequences (no `CURRENCY_PRECISION`, no `separator`/`adjective`, and an
//! unknown code returning a bare cardinal instead of raising). `to_cheque` is
//! *not* overridden, so it uses the base implementation and does raise for the
//! codes `to_currency` silently accepts.
//!
//! Because `to_currency` hands floats to `to_cardinal`, this file must also
//! carry the inherited float-cardinal path (`float2tuple` /
//! `to_cardinal_float` / a float twin of `splitnum`), which the corpus
//! exercises through the unknown-code branch (`to_currency(12.34, "KWD")` ==
//! "տասներկու ամբողջ երեք չորս"). That arithmetic is done in `f64` because
//! Python does it in `f64`; see the "CPython float semantics" note below for
//! why an exact-decimal model is measurably wrong.
//!
//! # `Decimal` is context-bound, not exact
//!
//! `to_currency` computes `(Decimal(str(val)) * 100) % 1` under the **default**
//! decimal context, whose precision is 28. `Decimal.__mod__` raises
//! `InvalidOperation(DivisionImpossible)` once the integer quotient needs more
//! than 28 digits, so every `abs(val) >= 1e26` raises — int and float alike,
//! and *before* the CURRENCY_FORMS lookup, so even an unknown code raises.
//! Modelled with `N2WError::Custom { module: "decimal", class:
//! "InvalidOperation" }`; see [`LangHy::to_currency`]. Below that bound the
//! context never rounds (a value under 1e26 carries at most 26 integer digits,
//! and `* 100` keeps it inside 28 significant digits), so exact `BigDecimal`
//! arithmetic is faithful everywhere the raise does not fire.
//!
//! # Error variants
//!
//! * `to_ordinal(n)` for `n < 0` → `N2WError::Type` (bug 4, fixed).
//! * `to_ordinal_num(n)` for `n < 0` → `N2WError::Type` (`verify_ordinal`).
//! * `to_cardinal(n)` for `abs(n) >= 10**36` → `N2WError::Overflow`, from the
//!   inherited MAXVAL check in `default_to_cardinal`.
//! * `to_cheque(v, cur)` for a code outside `CURRENCY_FORMS` →
//!   `N2WError::NotImplemented`. `to_currency` never raises for that case.

use crate::base::{
    clean, default_to_cardinal, floatord_error, py_num_str, set_low_numwords, set_mid_numwords,
    verify_ordinal, Cards, Lang, N2WError, Node, Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// The tuple card words (bug 1).
//
// `cards[10**33]`, `cards[10**23]` and `cards[10**13]` hold Python *tuples*,
// not strings. Storing each tuple's `str()` here is exact rather than
// approximate, because a tuple card can only ever reach `merge` through the
// arms that format it with `"%s"` — which is precisely `str()`:
//
//   * A tuple card is always the *right* operand. `splitnum` emits it as
//     `out[1]` = `(self.cards[elem], elem)`; `out[0]` is either `(cards[1], 1)`
//     or a recursive `splitnum(div)`. For a tuple to arrive on the *left*, some
//     `div` would have to be >= 10**13, which needs `value >= 10**13 * 10**23`
//     = 10**36 = MAXVAL — rejected by the overflow check first.
//   * The two arms that would raise `TypeError` on a real tuple
//     (`ctext + " " + ntext` and `ctext + ntext`) both require `nnum < 100`,
//     and a tuple card's `nnum` is >= 10**13. Unreachable.
//   * `ctext == "իննսուն"` against a tuple is simply False in Python, and is
//     equally False against these strings.
//
// So every reachable use is `"%s %s" % (ctext, ntext)` → `str(tuple)`.
// Verified against the corpus: to_cardinal(10**15) == "հարյուր (1000000, 'միլիոն')".
const TUPLE_CARD_E33: &str = "(1000000000000, 'տրիլիոն')";
const TUPLE_CARD_E23: &str = "(1000000000, 'միլիարդ')";
const TUPLE_CARD_E13: &str = "(1000000, 'միլիոն')";

// `merge`'s one word-sensitive comparison: only "իննսուն" (90) takes a space
// before its unit ("իննսուն ինը" = 99), while every other ten glues
// ("քսանմեկ" = 21).
const NINETY: &str = "իննսուն";

/// Python's `ORDINAL_ONES` (keys 1..=9).
///
/// A miss is a `KeyError`. `to_ordinal` rejects negatives before this
/// (bug 4, #158), so the `_` arm is only a safety net.
fn ordinal_ones(n: &BigInt) -> Result<&'static str> {
    match n.to_i64() {
        Some(1) => Ok("առաջին"),
        Some(2) => Ok("երկրորդ"),
        Some(3) => Ok("երրորդ"),
        Some(4) => Ok("չորրորդ"),
        Some(5) => Ok("հինգերորդ"),
        Some(6) => Ok("վեցերորդ"),
        Some(7) => Ok("յոթերորդ"),
        Some(8) => Ok("ութերորդ"),
        Some(9) => Ok("իններորդ"),
        _ => Err(N2WError::Key(n.to_string())),
    }
}

/// Python's `ORDINAL_TEENS` (keys 10..=19).
fn ordinal_teens(n: &BigInt) -> Result<&'static str> {
    match n.to_i64() {
        Some(10) => Ok("տասներորդ"),
        Some(11) => Ok("տասնմեկերորդ"),
        Some(12) => Ok("տասներկուերորդ"),
        Some(13) => Ok("տասներեքերորդ"),
        Some(14) => Ok("տասնչորսերորդ"),
        Some(15) => Ok("տասնհինգերորդ"),
        Some(16) => Ok("տասնվեցերորդ"),
        Some(17) => Ok("տասնյոթերորդ"),
        Some(18) => Ok("տասնութերորդ"),
        Some(19) => Ok("տասնիներորդ"),
        _ => Err(N2WError::Key(n.to_string())),
    }
}

/// Python's `ORDINAL_TENS` (keys 2..=9, i.e. the tens digit).
fn ordinal_tens(n: &BigInt) -> Result<&'static str> {
    match n.to_i64() {
        Some(2) => Ok("քսաներորդ"),
        Some(3) => Ok("երեսուներորդ"),
        Some(4) => Ok("քառասուներորդ"),
        Some(5) => Ok("հիսուներորդ"),
        Some(6) => Ok("վաթսուներորդ"),
        Some(7) => Ok("յոթանասուներորդ"),
        Some(8) => Ok("ութսուներորդ"),
        Some(9) => Ok("իննսուներորդ"),
        _ => Err(N2WError::Key(n.to_string())),
    }
}

/// Python's module-level `TENS` (keys 2..=9, i.e. the tens digit).
///
/// Distinct from the `mid_numwords` tens: this table is only used by
/// `to_ordinal` for the 21..99 "<ten> <ordinal unit>" form.
fn tens_word(n: &BigInt) -> Result<&'static str> {
    match n.to_i64() {
        Some(2) => Ok("քսան"),
        Some(3) => Ok("երեսուն"),
        Some(4) => Ok("քառասուն"),
        Some(5) => Ok("հիսուն"),
        Some(6) => Ok("վաթսուն"),
        Some(7) => Ok("յոթանասուն"),
        Some(8) => Ok("ութսուն"),
        Some(9) => Ok("իննսուն"),
        _ => Err(N2WError::Key(n.to_string())),
    }
}

/// Numeric `value == 0` over either [`FloatValue`] arm — Python's `if value
/// == 0` in `to_ordinal`, which -0.0 and `Decimal("0.00")` both satisfy.
fn fv_eq_zero(v: &FloatValue) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value == 0.0,
        FloatValue::Decimal { value, .. } => value.is_zero(),
    }
}

/// Numeric `value < n` over either [`FloatValue`] arm.
fn fv_lt(v: &FloatValue, n: i64) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value < n as f64,
        FloatValue::Decimal { value, .. } => *value < BigDecimal::from(n),
    }
}

/// Numeric `value >= n` over either [`FloatValue`] arm.
fn fv_ge(v: &FloatValue, n: i64) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value >= n as f64,
        FloatValue::Decimal { value, .. } => *value >= BigDecimal::from(n),
    }
}

// ---------------------------------------------------------------------------
// CPython float semantics.
//
// `Num2Word_HY.to_currency` hands *floats* to `to_cardinal`, and
// `Num2Word_Base.float2tuple` then does its arithmetic in binary floating
// point. That arithmetic is observable: it is exactly why `float2tuple`
// carries a 0.01 fudge factor to undo its own rounding noise.
//
// Reproducing it with exact `BigDecimal` arithmetic is tempting and **wrong**.
// Checked against CPython over 200k random doubles, an exact-decimal model of
// `float2tuple` disagrees with the real thing on ~24% of full-precision inputs
// (e.g. `0.4336456836623859` -> exact says post=4336456836623859, CPython says
// 4336456836623858), and an exact model of `int(round(val * 100))` disagrees
// above ~4.5e13 (e.g. `995691641656199.0` -> CPython 99569164165619904, exact
// 99569164165619900). So the float ops are mirrored in `f64`, which is the same
// IEEE-754 double arithmetic CPython performs.
// ---------------------------------------------------------------------------

/// The `f64` CPython was holding.
///
/// The `BigDecimal` was parsed from Python's `str(value)`, and `repr` of a
/// float round-trips, so re-parsing those same digits recovers the identical
/// double. Rust's float parser is correctly rounded, as is CPython's, so the
/// two agree bit for bit.
fn bd_to_f64(d: &BigDecimal) -> f64 {
    d.to_string().parse::<f64>().unwrap_or(f64::NAN)
}

/// `int(x)` for a float: truncate toward zero, exactly and without an `i64`
/// bottleneck. `BigDecimal::try_from(f64)` yields the double's *exact* binary
/// value, so `int(1e300)` reproduces all 301 digits as CPython does.
fn f64_trunc_to_bigint(v: f64) -> Result<BigInt> {
    BigDecimal::try_from(v)
        .map(|b| b.with_scale(0).as_bigint_and_exponent().0)
        // int(inf) / int(nan) -> OverflowError / ValueError in CPython; only
        // the overflow is reachable here (values come from finite input).
        .map_err(|_| N2WError::Overflow(format!("cannot convert float {} to integer", v)))
}

/// `10 ** p` as CPython converts it for `float * int`.
///
/// Python builds the exact integer, then `float.__mul__` converts it with
/// `PyLong_AsDouble` (correctly rounded), raising OverflowError past 1e308.
/// Parsing `"1e{p}"` is likewise correctly rounded, so it matches; `powi`
/// would not, as it accumulates error past 1e22.
fn pow10_f64(p: i64) -> Result<f64> {
    if p >= 309 {
        return Err(N2WError::Overflow(
            "int too large to convert to float".to_string(),
        ));
    }
    Ok(format!("1e{}", p).parse::<f64>().unwrap_or(f64::INFINITY))
}

/// `abs(Decimal(repr(v)).as_tuple().exponent)` — the `precision` that
/// `float2tuple` derives from `str(value)`.
///
/// CPython's `repr` prints the shortest round-tripping digits, switching to
/// exponent form when the decimal point position `decpt` satisfies
/// `decpt <= -4 || decpt > 16`. Rust's `{:e}` gives the same shortest digits,
/// so `decpt` and the digit count are recoverable from it.
///
/// For `value = 0.digits x 10**decpt` the resulting `Decimal` exponent is
/// `decpt - len(digits)` in every case *except* a positional integral value,
/// where CPython appends ".0" and the exponent becomes -1 (`repr(50.0)` is
/// "50.0", exponent -1, **not** "50" with exponent 0).
fn python_repr_scale(v: f64) -> i64 {
    if v == 0.0 {
        return 1; // repr(0.0) == "0.0" -> exponent -1
    }
    let s = format!("{:e}", v.abs()); // e.g. "3.45e1"
    let (mant, exp) = match s.split_once('e') {
        Some(pair) => pair,
        None => return 1,
    };
    let exp: i64 = exp.parse().unwrap_or(0);
    let ndigits = mant.chars().filter(|c| c.is_ascii_digit()).count() as i64;
    let decpt = exp + 1;
    if decpt > -4 && decpt <= 16 && decpt >= ndigits {
        // Positional and integral: CPython writes the trailing ".0".
        return 1;
    }
    (decpt - ndigits).abs()
}

/// `repr(v)` for a float, as CPython prints it.
///
/// Shortest round-tripping digits, switching to exponent form when
/// `decpt <= -4 || decpt > 16`, with a trailing ".0" forced on positional
/// integral values and a 2-digit minimum exponent. Only the `errmsg_toobig`
/// interpolation needs this; see [`LangHy::super_to_cardinal_f64`].
///
/// # Known divergence (last digit, exact ties only)
///
/// Rust's `{:e}` and CPython's `repr` are both shortest-round-trip, but they
/// break an exact tie differently. `844923945304372.2` is the double
/// 844923945304372.25, sitting exactly between two 16-digit candidates that
/// both round-trip; CPython rounds the final digit to even ("...372.2"), Rust
/// rounds away ("...372.3"). Measured at 1 in 4021 random doubles.
///
/// This is confined to this function, and this function only ever feeds an
/// OverflowError *message*. It cannot affect any conversion output:
/// [`python_repr_scale`] depends on the digit *count*, which ties do not
/// change, and it matched CPython on 4021/4021 of the same sample. The message
/// itself is unreachable through `to_currency` (which raises InvalidOperation
/// at 1e26, far below MAXVAL 1e36).
fn py_repr_f64(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    if v == 0.0 {
        return if v.is_sign_negative() { "-0.0" } else { "0.0" }.to_string();
    }
    let neg = v < 0.0;
    let s = format!("{:e}", v.abs()); // Rust's shortest round-trip, e.g. "3.45e1"
    let (mant, exp) = match s.split_once('e') {
        Some(p) => p,
        None => return s,
    };
    let exp: i64 = exp.parse().unwrap_or(0);
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let nd = digits.len();
    let decpt = exp + 1;
    let body = if decpt <= -4 || decpt > 16 {
        let mut m = String::new();
        m.push_str(&digits[..1]);
        if nd > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        format!(
            "{}e{}{:02}",
            m,
            if exp < 0 { "-" } else { "+" },
            exp.abs()
        )
    } else if decpt <= 0 {
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt as usize >= nd {
        format!("{}{}.0", digits, "0".repeat(decpt as usize - nd))
    } else {
        format!("{}.{}", &digits[..decpt as usize], &digits[decpt as usize..])
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// `int > float` as Python evaluates it: **exactly**, with no float cast.
///
/// This matters next to `divmod`, which *does* cast: `splitnum` picks its card
/// with an exact comparison and then divides in binary floating point.
fn bigint_gt_f64(a: &BigInt, b: f64) -> bool {
    match BigDecimal::try_from(b) {
        Ok(bd) => BigDecimal::from(a.clone()) > bd,
        // b is inf/nan: every finite int is < +inf and > -inf.
        Err(_) => b < 0.0 || b.is_nan(),
    }
}

/// `float(some_int)` as CPython's `PyLong_AsDouble` does it: correctly
/// rounded. `BigInt`'s Display is exact and Rust's float parser is correctly
/// rounded, so the round-trip reproduces it.
fn bigint_to_f64(a: &BigInt) -> f64 {
    a.to_string().parse::<f64>().unwrap_or(f64::INFINITY)
}

/// CPython's `float.__mod__` (`float_rem`): C `fmod`, then take the divisor's
/// sign. Rust's `%` on `f64` is `fmod`, which is exact.
fn py_float_mod(x: f64, y: f64) -> f64 {
    let m = x % y;
    if m != 0.0 && ((y < 0.0) != (m < 0.0)) {
        m + y
    } else {
        m
    }
}

/// CPython's `float.__floordiv__` (`float_floor_div`), including the
/// `div - floordiv > 0.5` correction it applies after the exact subtraction.
fn py_float_floordiv(x: f64, y: f64) -> f64 {
    let m = x % y;
    let mut div = (x - m) / y;
    if m != 0.0 && ((y < 0.0) != (m < 0.0)) {
        // CPython also fixes up `mod` here; only `div` escapes to the caller.
        div -= 1.0;
    }
    if div != 0.0 {
        let fd = div.floor();
        if div - fd > 0.5 {
            fd + 1.0
        } else {
            fd
        }
    } else {
        (0.0f64).copysign(x / y)
    }
}

/// `10**28` — the point at which `(Decimal * 100) % 1` exceeds the default
/// decimal context's 28-digit precision and raises. Built once; see the call
/// site in [`LangHy::to_currency`].
fn decimal_prec_limit() -> &'static BigDecimal {
    static LIMIT: OnceLock<BigDecimal> = OnceLock::new();
    LIMIT.get_or_init(|| BigDecimal::from(BigInt::from(10).pow(28)))
}

/// The rebound `cents` local in `Num2Word_HY.to_currency`.
///
/// The parameter arrives as a `bool` and is then overwritten with either an
/// `int` (the whole-cents path) or a `Decimal` (the fractional-cents path).
/// The body later branches on `isinstance(cents, Decimal)`, so the two types
/// are load-bearing and cannot be unified into one numeric.
enum Cents {
    Int(BigInt),
    Dec(BigDecimal),
}

impl Cents {
    /// Python's `if cents:` — zero is falsy for both `int` and `Decimal`.
    fn is_zero(&self) -> bool {
        match self {
            Cents::Int(i) => i.is_zero(),
            Cents::Dec(d) => d.is_zero(),
        }
    }

    /// Python's `cents == 50`. A `Decimal` with a fractional part never
    /// compares equal, which is what keeps the 50/25/75/5 arms out of the
    /// fractional path.
    fn eq_i64(&self, n: i64) -> bool {
        match self {
            Cents::Int(i) => i == &BigInt::from(n),
            Cents::Dec(d) => d == &BigDecimal::from(n),
        }
    }
}

/// Python's `CURRENCY_FORMS` for HY.
///
/// `Num2Word_HY` declares all eleven itself; `Num2Word_Base.CURRENCY_FORMS` is
/// `{}` and nothing is merged in, so this table is the whole set. Every entry
/// carries exactly two forms, matching Python's arity — `pluralize` indexes
/// `forms[0]`/`forms[1]`, and both happen to be the same word in Armenian.
///
/// Built once in `new()` and stored: constructing it per call is what made an
/// earlier revision of this port slower than the Python it replaces.
fn currency_table() -> HashMap<&'static str, CurrencyForms> {
    let mut m = HashMap::with_capacity(11);
    m.insert("AMD", CurrencyForms::new(&["դրամ", "դրամ"], &["լումա", "լումա"]));
    m.insert("EUR", CurrencyForms::new(&["եվրո", "եվրո"], &["ցենտ", "ցենտ"]));
    m.insert(
        "RUB",
        CurrencyForms::new(&["ռուբլի", "ռուբլի"], &["կոպեկ", "կոպեկ"]),
    );
    m.insert("USD", CurrencyForms::new(&["դոլար", "դոլար"], &["ցենտ", "ցենտ"]));
    m.insert("JPY", CurrencyForms::new(&["իեն", "իեն"], &["սեն", "սեն"]));
    m.insert(
        "GBP",
        CurrencyForms::new(&["ֆունտ ստեռլինգ", "ֆունտ ստեռլինգ"], &["պենս", "պենս"]),
    );
    m.insert(
        "CHF",
        CurrencyForms::new(
            &["շվեյցարական ֆրանկ", "շվեյցարական ֆրանկ"],
            &["սանտիմ", "սանտիմ"],
        ),
    );
    m.insert("CNY", CurrencyForms::new(&["յուան", "յուան"], &["ֆեն", "ֆեն"]));
    m.insert(
        "IRR",
        CurrencyForms::new(&["իրանական ռիալ", "իրանական ռիալ"], &["դինար", "դինար"]),
    );
    m.insert(
        "TRY",
        CurrencyForms::new(&["թուրքական լիրա", "թուրքական լիրա"], &["ղուրուշ", "ղուրուշ"]),
    );
    m.insert(
        "AED",
        CurrencyForms::new(&["արաբական դիրհամ", "արաբական դիրհամ"], &["ֆիլս", "ֆիլս"]),
    );
    m
}

pub struct LangHy {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    // Cached constants, so the hot paths avoid re-allocating BigInts.
    hundred: BigInt,
    thousand: BigInt,
    million: BigInt,
    billion: BigInt,
    trillion: BigInt,
    ten: BigInt,
    twenty: BigInt,
    two: BigInt,
}

impl Default for LangHy {
    fn default() -> Self {
        Self::new()
    }
}

impl LangHy {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Python's `set_high_numwords(high_numwords)`, bug 1:
        //   max = 3 + 10 * 3 = 33; zip(high, range(33, 3, -10)) → 33, 23, 13.
        // The card *words* are the tuples themselves; we store their str().
        let ten = BigInt::from(10u8);
        cards.insert(ten.pow(33), TUPLE_CARD_E33);
        cards.insert(ten.pow(23), TUPLE_CARD_E23);
        cards.insert(ten.pow(13), TUPLE_CARD_E13);

        // Python's `mid_numwords`. Note the gap: nothing between 1000 and
        // 10**13, because bug 1 misplaced every high card.
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "հազար"),
                (100, "հարյուր"),
                (90, "իննսուն"),
                (80, "ութսուն"),
                (70, "յոթանասուն"),
                (60, "վաթսուն"),
                (50, "հիսուն"),
                (40, "քառասուն"),
                (30, "երեսուն"),
                (20, "քսան"),
            ],
        );

        // Python's `low_numwords`: 20 entries mapping to 19 down to 0.
        set_low_numwords(
            &mut cards,
            &[
                "տասնինը",
                "տասնութ",
                "տասնյոթ",
                "տասնվեց",
                "տասնհինգ",
                "տասնչորս",
                "տասներեք",
                "տասներկու",
                "տասնմեկ",
                "տասը",
                "ինը",
                "ութ",
                "յոթ",
                "վեց",
                "հինգ",
                "չորս",
                "երեք",
                "երկու",
                "մեկ",
                "զրո",
            ],
        );

        // Python: MAXVAL = 1000 * list(self.cards.keys())[0]. Insertion order
        // is descending, so keys()[0] is 10**33 → MAXVAL = 10**36.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangHy {
            cards,
            maxval,
            // Dead (is_title is False for HY) but present in Python's setup.
            exclude_title: vec!["և".into(), "ամբողջ".into(), "մինուս".into()],
            currency_forms: currency_table(),
            hundred: BigInt::from(100),
            thousand: BigInt::from(1000),
            million: BigInt::from(1_000_000),
            billion: BigInt::from(1_000_000_000u64),
            trillion: BigInt::from(10u8).pow(12),
            ten: BigInt::from(10),
            twenty: BigInt::from(20),
            two: BigInt::from(2),
        }
    }

    /// Inherited `Num2Word_Base.verify_ordinal`. Integer input can never trip
    /// the float check, so only the negative check is observable here.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == num_bigint::Sign::Minus {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// Inherited `Num2Word_Base.float2tuple`, **float branch**.
    ///
    /// `precision` is the instance attribute the Python sets here rather than
    /// returns; callers need it too, so it is threaded through explicitly
    /// instead. `to_cardinal_float` saves and restores `self.precision` around
    /// the call, so no state escapes and passing it is equivalent.
    ///
    /// The 0.01 test is Python undoing its own float noise: `1.239999999`
    /// would otherwise floor to 239999998.
    fn float2tuple(&self, value: f64, precision: i64) -> Result<(BigInt, BigInt)> {
        let pre = f64_trunc_to_bigint(value)?;
        // `value - pre` is float-minus-int in Python; float(int(value)) is
        // exactly value.trunc(), and the subtraction is exact either way.
        let pow = pow10_f64(precision)?;
        let post = (value - value.trunc()).abs() * pow;
        let rounded = post.round_ties_even(); // Python 3 round() is half-even
        let post = if (rounded - post).abs() < 0.01 {
            rounded
        } else {
            post.floor()
        };
        Ok((pre, f64_trunc_to_bigint(post)?))
    }

    /// Inherited `Num2Word_Base.to_cardinal_float`.
    ///
    /// `precision` is derived from `str(value)` inside Python's `float2tuple`;
    /// see [`python_repr_scale`].
    fn to_cardinal_float(&self, value: f64) -> Result<String> {
        let precision = python_repr_scale(value);
        let (pre, post) = self.float2tuple(value, precision)?;

        // post = "0" * (precision - len(post)) + str(post). Python's `*` on a
        // negative count yields "", so a longer post is left untouched — and
        // the loop below then reads only its first `precision` characters.
        let mut post_s = post.to_string();
        let plen = post_s.chars().count() as i64;
        if precision > plen {
            post_s = "0".repeat((precision - plen) as usize) + &post_s;
        }

        let mut out = vec![self.to_cardinal(&pre)?];
        if value < 0.0 && pre.is_zero() {
            out.insert(0, self.negword().trim().to_string());
        }
        if precision != 0 {
            out.push(self.title(self.pointword()));
        }
        let chars: Vec<char> = post_s.chars().collect();
        for i in 0..precision as usize {
            let curr = chars[i].to_digit(10).unwrap_or(0);
            out.push(self.to_cardinal(&BigInt::from(curr))?);
        }
        Ok(out.join(" "))
    }

    /// `Num2Word_Base.splitnum` for **float** input.
    ///
    /// Python has one method and lets duck typing carry a float through it, so
    /// the decomposition is done in binary floating point. That is observable
    /// and cannot be replaced by the exact integer engine: `divmod(value, elem)`
    /// casts the card to a double, and cards above 2**53 are not exactly
    /// representable. `10**23` becomes 99999999999999991611392.0, so
    /// `splitnum(2e23)` divides by *that* and yields a clean `(2.0, 0.0)`,
    /// whereas exact integer arithmetic on the same double yields a quotient of
    /// 1 and a 23-digit remainder. Verified against CPython: `to_currency(2e23,
    /// "KWD")` is "երկու (1000000000, 'միլիարդ')", not the long form.
    ///
    /// The leaves still carry the card keys as exact `BigInt`s, because that is
    /// what Python puts in the tuples — only the division is float-tainted — so
    /// `base::clean` and [`Lang::merge`] are reused unchanged.
    fn splitnum_f64(&self, value: f64) -> Option<Vec<Node>> {
        for (elem, word) in self.cards.iter() {
            // Python's `if elem > value: continue` — exact, unlike the divmod.
            if bigint_gt_f64(elem, value) {
                continue;
            }
            let mut out: Vec<Node> = Vec::new();
            let elem_f = bigint_to_f64(elem);
            let (div, mod_) = if value == 0.0 {
                (1.0, 0.0)
            } else {
                (
                    py_float_floordiv(value, elem_f),
                    py_float_mod(value, elem_f),
                )
            };

            if div == 1.0 {
                let one = BigInt::one();
                let w = self.cards.get(&one).unwrap_or("").to_string();
                out.push(Node::Leaf(w, one));
            } else {
                if div == value {
                    // Tally systems; unreachable for HY (no card equals 1
                    // besides `cards[1]`), ported for shape.
                    let reps = div.to_string().parse::<usize>().unwrap_or(0);
                    return Some(vec![Node::Leaf(
                        word.repeat(reps),
                        f64_trunc_to_bigint(div * elem_f).ok()?,
                    )]);
                }
                out.push(Node::List(self.splitnum_f64(div)?));
            }

            out.push(Node::Leaf(word.clone(), elem.clone()));

            if mod_ != 0.0 {
                out.push(Node::List(self.splitnum_f64(mod_)?));
            }
            return Some(out);
        }
        None
    }

    /// `Num2Word_Base.to_cardinal` for a float that passes its
    /// `assert int(value) == value` — i.e. the integral-float path.
    ///
    /// The MAXVAL comparison is exact (int vs float), while the message
    /// interpolates the float with `%s`, hence [`py_repr_f64`]. Unreachable
    /// through `to_currency`, which raises InvalidOperation at 1e26 — far below
    /// MAXVAL (1e36) — but reachable via `cardinal_from_decimal`.
    fn super_to_cardinal_f64(&self, value: f64) -> Result<String> {
        let mut out = String::new();
        let mut v = value;
        if v < 0.0 {
            v = v.abs();
            out = format!("{} ", self.negword().trim());
        }
        // Python: `if value >= self.MAXVAL` — i.e. not (MAXVAL > value).
        if !bigint_gt_f64(self.maxval(), v) {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                py_repr_f64(v),
                self.maxval()
            )));
        }
        let tree = self.splitnum_f64(v).ok_or_else(|| {
            N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                py_repr_f64(v),
                self.maxval()
            ))
        })?;
        let words = match clean(self, tree) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        Ok(self.title(&format!("{}{}", out, words)))
    }

    /// `Num2Word_HY.to_cardinal` for **float** input — the twin of the
    /// `BigInt` [`Lang::to_cardinal`] above.
    ///
    /// Python has one polymorphic method; Rust needs the two shapes split. The
    /// integer twin is the verified one and is left untouched. The comparisons
    /// here are float comparisons, exactly as Python performs them, which is
    /// why `to_cardinal(1000.0)` short-circuits to "հազար" just like the int.
    fn to_cardinal_f64(&self, value: f64) -> Result<String> {
        if value == 0.0 {
            return Ok("զրո".to_string());
        }
        if value == 1000.0 {
            return Ok("հազար".to_string());
        }

        if value >= 1e6 && value < 1e9 {
            let millions = py_float_floordiv(value, 1e6);
            let rest = py_float_mod(value, 1e6);
            let millions_part = if millions == 1.0 {
                "մեկ միլիոն".to_string()
            } else if millions == 2.0 {
                "երկու միլիոն".to_string()
            } else {
                format!("{} միլիոն", self.to_cardinal_f64(millions)?)
            };
            if rest == 0.0 {
                return Ok(millions_part);
            }
            return Ok(format!("{} {}", millions_part, self.to_cardinal_f64(rest)?));
        }

        if value == 1e9 {
            return Ok("մեկ միլիարդ".to_string());
        } else if py_float_mod(value, 1e9) == 0.0 && value < 1e12 {
            let prefix = py_float_floordiv(value, 1e9);
            if prefix == 2.0 {
                return Ok("երկու միլիարդ".to_string());
            }
            return Ok(format!("{} միլիարդ", self.to_cardinal_f64(prefix)?));
        }

        // super().to_cardinal(value): `assert int(value) == value` diverts
        // non-integral input to the float path. An integral float stays on the
        // engine path — but as a *float*, so it goes through the float twin of
        // splitnum rather than the exact integer engine.
        let mut result = if value == value.trunc() {
            self.super_to_cardinal_f64(value)?
        } else {
            self.to_cardinal_float(value)?
        };

        if result.contains("հազար հազար") {
            result = result.replace("հազար հազար", "միլիոն");
        }
        Ok(result)
    }

    /// `Num2Word_HY.to_cardinal` dispatched on what Python actually held: an
    /// `int` keeps the exact integer engine, a float takes the float twin.
    fn to_cardinal_value(&self, val: &CurrencyValue) -> Result<String> {
        match val {
            CurrencyValue::Int(i) => self.to_cardinal(i),
            CurrencyValue::Decimal { value: d, .. } => self.to_cardinal_f64(bd_to_f64(d)),
        }
    }

    /// `Num2Word_HY.to_cardinal` for **Decimal** input — the twin of
    /// [`LangHy::to_cardinal_f64`], but with exact `BigDecimal` arithmetic so
    /// issue #603's `98746251323029.99` keeps every digit instead of rounding
    /// through a double (a float cast lands on `…029.98`).
    ///
    /// `Num2Word_HY` never overrides `Num2Word_Base.to_cardinal_float`; a
    /// Decimal reaches the float path exactly the way an int does — through the
    /// `to_cardinal` wrapper, whose short-circuits (1000, the millions range,
    /// the billions range) run on the Decimal first, in Decimal arithmetic.
    /// That is observable: `Decimal("1000000.5")` → "մեկ միլիոն զրո ամբողջ
    /// հինգ", because the millions arm splits off a `rest` of `Decimal("0.5")`
    /// and renders it recursively (whole part 0 → "զրո"), which the base
    /// `to_cardinal_float` alone would never emit.
    ///
    /// When no short-circuit fires, `super().to_cardinal(value)` decides:
    /// integral Decimals take the integer engine (routed back through the
    /// integer [`Lang::to_cardinal`], which is idempotent for a value that
    /// already failed every short-circuit above), non-integral input takes
    /// `Num2Word_Base.to_cardinal_float`'s exact Decimal arm
    /// ([`default_to_cardinal_float`]). The wrapper's trailing
    /// "հազար հազար" → "միլիոն" patch (bug 3) is applied last, as in Python.
    fn to_cardinal_decimal(&self, value: &BigDecimal, precision: u32) -> Result<String> {
        if value.is_zero() {
            return Ok("զրո".to_string());
        }
        let thousand = BigDecimal::from(1000);
        if (value - &thousand).is_zero() {
            return Ok("հազար".to_string());
        }

        let million = BigDecimal::from(1_000_000);
        let billion = BigDecimal::from(1_000_000_000i64);
        let trillion = BigDecimal::from(self.trillion.clone());
        let million_i = BigInt::from(1_000_000);
        let billion_i = BigInt::from(1_000_000_000i64);

        // int(value): truncate toward zero, exactly as Python's int(Decimal).
        let vint = value.with_scale(0).as_bigint_and_exponent().0;
        // value == int(value)? `with_scale(0)` truncates, so this is the exact
        // integrality test Python's `assert int(value) == value` performs.
        let is_integral = (value - value.with_scale(0)).is_zero();

        // value >= 1e6 and value < 1e9. Positive-only (a negative fails the
        // lower bound), so the whole part floor-divides like Python's Decimal
        // `//` and the fraction rides along inside `rest = value % 1e6`.
        if value >= &million && value < &billion {
            let millions = &vint / &million_i;
            let rest = value - BigDecimal::from(&millions * &million_i);
            let millions_part = if millions.is_one() {
                "մեկ միլիոն".to_string()
            } else if millions == self.two {
                "երկու միլիոն".to_string()
            } else {
                format!("{} միլիոն", self.to_cardinal(&millions)?)
            };
            if rest.is_zero() {
                return Ok(millions_part);
            }
            // Python recomputes precision from `rest`'s own exponent inside the
            // recursive float2tuple; `abs(exponent)` is `rest`'s scale.
            let rest_prec = rest.as_bigint_and_exponent().1.max(0) as u32;
            return Ok(format!(
                "{} {}",
                millions_part,
                self.to_cardinal_decimal(&rest, rest_prec)?
            ));
        }

        // value == 1e9, else an exact multiple of 1e9 below 1e12.
        if (value - &billion).is_zero() {
            return Ok("մեկ միլիարդ".to_string());
        } else if is_integral && vint.mod_floor(&billion_i).is_zero() && value < &trillion {
            // Exact multiple, so `/` (toward zero) and Python's Decimal `//`
            // agree; a negative multiple keeps its sign via `to_cardinal`.
            let prefix = &vint / &billion_i;
            if prefix == self.two {
                return Ok("երկու միլիարդ".to_string());
            }
            return Ok(format!("{} միլիարդ", self.to_cardinal(&prefix)?));
        }

        // super().to_cardinal(value) + the "հազար հազար" patch.
        let mut result = if is_integral {
            self.to_cardinal(&vint)?
        } else {
            default_to_cardinal_float(
                self,
                &FloatValue::Decimal {
                    value: value.clone(),
                    precision,
                },
                None,
            )?
        };
        if result.contains("հազար հազար") {
            result = result.replace("հազար հազար", "միլիոն");
        }
        Ok(result)
    }
}

impl Lang for LangHy {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "AMD"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "մինուս "
    }
    fn pointword(&self) -> &str {
        "ամբողջ"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_HY.merge`.
    ///
    /// Every arm returns `cnum + nnum` (bug 2) — the sums are wrong as
    /// arithmetic but are load-bearing for the comparisons below, so they are
    /// preserved exactly.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, cnum) = l;
        let (rtext, nnum) = r;
        let mut ctext = ltext.to_string();
        let mut ntext = rtext.to_string();

        if cnum.is_one() {
            // 1000 needs no "մեկ": "հազար", not "մեկ հազար". This is also why
            // to_year's strip is dead code (bug 6).
            if nnum == &self.thousand {
                return (ntext, nnum.clone());
            }
            if nnum < &self.thousand {
                return (ntext, nnum.clone());
            }
            ctext = "մեկ".to_string();
        }

        if nnum < cnum && cnum >= &self.hundred && cnum < &self.thousand {
            if nnum.mod_floor(&self.hundred).is_zero() {
                // Unreachable in practice (bug 7). Python's `ntext[:-1]` drops
                // the last *character*; index by chars, never bytes.
                let mut cs: Vec<char> = ntext.chars().collect();
                cs.pop();
                ntext = cs.into_iter().collect::<String>() + "ի";
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        }

        if nnum < &self.hundred {
            if cnum < &self.hundred {
                // Only 90 spaces its unit off; every other ten concatenates.
                if ctext == NINETY {
                    return (format!("{} {}", ctext, ntext), cnum + nnum);
                }
                return (format!("{}{}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        }

        (format!("{} {}", ctext, ntext), cnum + nnum)
    }

    /// Port of `Num2Word_HY.to_cardinal` — the wrapper around `super()`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_zero() {
            return Ok("զրո".to_string());
        }

        // Simple cases
        if value == &self.thousand {
            return Ok("հազար".to_string());
        }

        if value >= &self.million && value < &self.billion {
            // value is strictly positive here, so `/` and `%` agree with
            // Python's floor semantics.
            let millions = value / &self.million;
            let rest = value % &self.million;
            let millions_part = if millions.is_one() {
                "մեկ միլիոն".to_string()
            } else if millions == self.two {
                "երկու միլիոն".to_string()
            } else {
                format!("{} միլիոն", self.to_cardinal(&millions)?)
            };
            if rest.is_zero() {
                return Ok(millions_part);
            }
            return Ok(format!("{} {}", millions_part, self.to_cardinal(&rest)?));
        }

        // For billions
        if value == &self.billion {
            return Ok("մեկ միլիարդ".to_string());
        } else if value.mod_floor(&self.billion).is_zero() && value < &self.trillion {
            // Python's `%` and `//` floor toward -inf, so negative multiples of
            // 10**9 reach here too: -10**9 → prefix -1 → "մինուս մեկ միլիարդ".
            // `mod_floor`/`div_floor` reproduce that; `%` and `/` would not.
            let prefix = value.div_floor(&self.billion);
            if prefix == self.two {
                return Ok("երկու միլիարդ".to_string());
            }
            return Ok(format!("{} միլիարդ", self.to_cardinal(&prefix)?));
        }

        // For other cases use standard implementation (super()). This is where
        // the negword and the MAXVAL overflow check live.
        let mut result = default_to_cardinal(self, value)?;

        // Fix for numbers like X000000 and X000000000 (bug 3). Python's
        // str.replace is non-overlapping and left-to-right, so
        // "հազար հազար հազար հազար" → "միլիոն միլիոն". Rust matches.
        if result.contains("հազար հազար") {
            result = result.replace("հազար հազար", "միլիոն");
        }
        Ok(result)
    }

    /// Port of `Num2Word_HY.to_ordinal`.
    ///
    /// Python never called `verify_ordinal`; negatives fell through
    /// `value < 20` and `value < 10` into `ORDINAL_ONES[value]` and raised
    /// `KeyError`. They now raise `errmsg_negord` (bug 4, #158).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        if value.is_zero() {
            return Ok("զրոերորդ".to_string());
        }

        if value < &self.twenty {
            if value < &self.ten {
                return ordinal_ones(value).map(|s| s.to_string());
            } else {
                return ordinal_teens(value).map(|s| s.to_string());
            }
        }

        if value < &self.hundred {
            let (tens, units) = value.div_mod_floor(&self.ten);
            if units.is_zero() {
                return ordinal_tens(&tens).map(|s| s.to_string());
            }
            return Ok(format!("{} {}", tens_word(&tens)?, ordinal_ones(&units)?));
        }

        // For larger numbers use simple rule - add "երորդ" at the end (bug 5).
        let cardinal = self.to_cardinal(value)?;
        Ok(cardinal + "երորդ")
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}-րդ", value))
    }

    fn lang_name(&self) -> &str {
        "Num2Word_HY"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_HY.pluralize`.
    ///
    /// Both HY forms are always the same word, so the branch is unobservable
    /// in output — but it is ported literally rather than collapsed, because
    /// `to_cheque` reads `cr1[-1]` and a dropped form would change its arity.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        if forms.len() >= 2 {
            let ten = BigInt::from(10);
            let hundred = BigInt::from(100);
            if n.is_one()
                || (n.mod_floor(&ten).is_one() && n.mod_floor(&hundred) != BigInt::from(11))
            {
                return Ok(forms[0].clone());
            }
            return Ok(forms[1].clone());
        }
        Ok(forms[0].clone())
    }

    /// `Num2Word_HY.to_cardinal` reached with a non-integral value.
    ///
    /// Not used by this file's `to_currency` (which calls the float path
    /// directly), but wired up so the inherited `default_to_currency` shape and
    /// any later float-cardinal phase get HY's real behaviour rather than the
    /// NotImplemented default.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        self.to_cardinal_f64(bd_to_f64(value))
    }

    /// The float/Decimal reach into `Num2Word_HY.to_cardinal`.
    ///
    /// `Num2Word_HY` does not override `Num2Word_Base.to_cardinal_float`, so it
    /// is *not* one of the 26 languages the trait default serves correctly: a
    /// float or Decimal enters through the `to_cardinal` wrapper, whose
    /// millions/billions short-circuits run in float (resp. Decimal) arithmetic
    /// *before* `super().to_cardinal`'s `assert int(value) == value` sends the
    /// non-integral remainder to the base `to_cardinal_float`. The default hook
    /// skips that wrapper and so drops the recursive "զրո" the millions arm
    /// emits (`1000000.5` → "մեկ միլիոն զրո ամբողջ հինգ", not
    /// "մեկ միլիոն ամբողջ հինգ"). This override re-expresses the wrapper for the
    /// two operand kinds: the `Float` arm reuses [`LangHy::to_cardinal_f64`],
    /// the `Decimal` arm [`LangHy::to_cardinal_decimal`] (exact, for #603).
    ///
    /// The dispatcher has already sent integral values to the integer
    /// [`Lang::to_cardinal`] (their `assert` passes), so only non-integral
    /// input normally arrives here; both twins still handle an integral value
    /// consistently if one does.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored,
    /// matching the live interpreter: HY's `to_cardinal` takes no `precision`
    /// parameter and nothing in HY reads `self.precision`, so
    /// `num2words(12.345, lang="hy", precision=p)` is "տասներկու ամբողջ երեք
    /// չորս հինգ" for every `p`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value: x, .. } => self.to_cardinal_f64(*x),
            FloatValue::Decimal { value: d, precision } => {
                self.to_cardinal_decimal(d, *precision)
            }
        }
    }

    /// Port of `Num2Word_HY.to_currency`.
    ///
    /// A **complete** override: it never calls `Num2Word_Base.to_currency`, so
    /// none of the base's currency machinery applies. Three consequences that
    /// look like mistakes and are not:
    ///
    /// 1. **`CURRENCY_PRECISION` is ignored.** The divisor is hard-coded 100,
    ///    so JPY — a zero-decimal currency everywhere else — renders subunits
    ///    here: `to_currency(0.5, "JPY")` == "հիսուն սեն".
    /// 2. **`separator` and `adjective` do not exist.** Python's signature is
    ///    `(self, val, currency="AMD", cents=True)`; the separator is the
    ///    hard-coded "," spliced in below, and passing either kwarg to HY is a
    ///    TypeError. Both trait parameters are therefore ignored.
    /// 3. **An unknown code does not raise.** The `else` returns
    ///    `to_cardinal(abs(val))`, so `to_currency(12.34, "KWD")` is
    ///    "տասներկու ամբողջ երեք չորս" rather than the NotImplementedError the
    ///    base would raise. `to_cheque` is *not* overridden, so it still raises
    ///    for those same codes — the two modes disagree.
    ///
    /// # Faithfully reproduced Python bugs
    ///
    /// * **The minus is dropped for 1.5 USD.** The `val == 1.5 and currency ==
    ///   "USD"` arm returns early, *before* the `is_negative` insert, and `val`
    ///   is already `abs()`-ed — so `to_currency(-1.5, "USD")` renders exactly
    ///   as `+1.5` does, with no "մինուս".
    /// * **The unknown-code branch drops the minus too**, for the same reason:
    ///   it returns `to_cardinal(abs(val))` before the insert.
    /// * **`cents == 5` loses the subunit noun.** `result.insert(-1, ...)`
    ///   splices "ամբողջ հինգ տասներորդ" *before the unit*, and no cents word is
    ///   ever appended: `to_currency(1.05, "EUR")` ==
    ///   "մեկ ամբողջ հինգ տասներորդ եվրո" — the "ցենտ" is simply gone.
    /// * **`cents` is a bool parameter that gets rebound to a number**, so the
    ///   name means two different things in one body. Modelled by [`Cents`].
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        let mut result: Vec<String> = Vec::new();
        let is_negative = val.is_negative();
        // val = abs(val) — the int/float split survives the abs, and the
        // unknown-code branch below depends on it.
        let val: CurrencyValue = match val {
            CurrencyValue::Int(i) => CurrencyValue::Int(i.abs()),
            CurrencyValue::Decimal { value: d, is_float, .. } => CurrencyValue::Decimal { value: d.abs(), has_decimal: true, is_float: *is_float },
        };

        // decimal_val = Decimal(str(val)); has_fractional_cents = (dv*100) % 1
        let decimal_val: BigDecimal = match &val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value: d, .. } => d.clone(),
        };
        let scaled = &decimal_val * BigDecimal::from(100);

        // `(decimal_val * 100) % 1` runs under the *default* decimal context,
        // whose precision is 28 — it is not exact arithmetic. `Decimal.__mod__`
        // raises InvalidOperation(DivisionImpossible) once the integer quotient
        // needs more than `prec` digits, and here the quotient is the integer
        // part of `decimal_val * 100`. So every `abs(val) >= 1e26` raises,
        // int and float alike.
        //
        // This fires *before* the CURRENCY_FORMS lookup, so it beats even the
        // unknown-code branch: `to_currency(1e36, "KWD")` raises rather than
        // returning a cardinal. `trunc(x) >= 10**28` iff `x >= 10**28` for the
        // integral bound, so the truncation is skipped.
        if scaled >= *decimal_prec_limit() {
            return Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.DivisionImpossible'>]".to_string(),
            });
        }

        let has_fractional_cents = (&scaled - scaled.with_scale(0)) != BigDecimal::zero();

        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            // else: return self.to_cardinal(val) — note `val` is the abs value,
            // so the sign is silently discarded.
            None => return self.to_cardinal_value(&val),
        };

        let whole: BigInt;
        let cents_val: Cents;
        if cents {
            if has_fractional_cents {
                // Keep precision for fractional cents.
                let cents_decimal = scaled.clone();
                whole = decimal_val.with_scale(0).as_bigint_and_exponent().0;
                cents_val = Cents::Dec(cents_decimal - BigDecimal::from(&whole * 100));
            } else {
                // cents = int(round(val * 100)). An int multiplies exactly in
                // Python; a float goes through binary arithmetic, and the two
                // part company above ~4.5e13 (see the module notes).
                let total: BigInt = match &val {
                    CurrencyValue::Int(i) => i * 100,
                    CurrencyValue::Decimal { value: d, .. } => {
                        f64_trunc_to_bigint((bd_to_f64(d) * 100.0).round_ties_even())?
                    }
                };
                let hundred = BigInt::from(100);
                whole = total.div_floor(&hundred);
                cents_val = Cents::Int(total.mod_floor(&hundred));
            }
        } else {
            whole = match &val {
                CurrencyValue::Int(i) => i.clone(),
                CurrencyValue::Decimal { value: d, .. } => d.with_scale(0).as_bigint_and_exponent().0,
            };
            cents_val = Cents::Int(BigInt::zero());
        }

        if !whole.is_zero() {
            result.push(self.to_cardinal(&whole)?);
            result.push(self.pluralize(&whole, &forms.unit)?);
        }

        if !cents_val.is_zero() {
            // Special case for 1.5 USD. Returns before the `is_negative`
            // insert, and `val` is already abs() — so -1.5 USD loses its minus.
            if matches!(&val, CurrencyValue::Decimal { value: d, .. } if bd_to_f64(d) == 1.5) && currency == "USD"
            {
                return Ok("մեկ դոլար ամբողջ հինգ տասներորդ ցենտ".to_string());
            }
            let has_whole = !whole.is_zero();
            if has_whole && cents_val.eq_i64(50) {
                result.push("հիսուն".to_string());
                result.push(self.pluralize(&BigInt::from(50), &forms.subunit)?);
            } else if has_whole && cents_val.eq_i64(25) {
                result.push("քսանհինգ".to_string());
                result.push(self.pluralize(&BigInt::from(25), &forms.subunit)?);
            } else if has_whole && cents_val.eq_i64(75) {
                result.push("յոթանասունհինգ".to_string());
                result.push(self.pluralize(&BigInt::from(75), &forms.subunit)?);
            } else if has_whole && cents_val.eq_i64(5) {
                // insert(-1): splices before the unit and never appends the
                // subunit noun, so the "ցենտ" is lost outright.
                result.insert(result.len() - 1, "ամբողջ հինգ տասներորդ".to_string());
            } else {
                if has_whole {
                    // The lone separator: the whole segment is collapsed into
                    // one element with a "," glued on.
                    result = vec![result.join(" ") + ","];
                }
                match &cents_val {
                    Cents::Dec(d) => {
                        // to_cardinal_float(float(cents)) — the Decimal is cast
                        // to a float first, so precision comes from repr() of
                        // the float, not from the Decimal's own scale:
                        // 12.345 -> cents 34.500 -> "երեսունչորս ամբողջ հինգ".
                        result.push(self.to_cardinal_float(bd_to_f64(d))?);
                        let n = d.with_scale(0).as_bigint_and_exponent().0;
                        result.push(self.pluralize(&n, &forms.subunit)?);
                    }
                    Cents::Int(n) => {
                        result.push(self.to_cardinal(n)?);
                        result.push(self.pluralize(n, &forms.subunit)?);
                    }
                }
            }
        }

        if is_negative {
            result.insert(0, "մինուս".to_string());
        }
        Ok(result.join(" "))
    }

    /// Port of `Num2Word_HY.to_year`. The `longval` kwarg is unused by the
    /// Python body, so it is dropped.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Special case for year: for 1000-1999, remove "մեկ" before "հազար".
        if &self.thousand <= value && value < &BigInt::from(2000) {
            let mut year_str = self.to_cardinal(value)?;
            if year_str.starts_with("մեկ հազար") {
                // Dead code (bug 6): merge already suppresses the "մեկ".
                // Python's `[4:]` is a character slice — "մեկ " is 4 chars but
                // 7 bytes, so a byte slice would panic mid-codepoint.
                year_str = year_str.chars().skip(4).collect::<String>().trim().to_string();
            }
            return Ok(format!("{} թվական", year_str));
        }

        Ok(format!("{} թվական", self.to_cardinal(value)?))
    }

    // ---- float / Decimal entry routing --------------------------------

    /// `to_ordinal(float/Decimal)` — Python's `Num2Word_HY.to_ordinal` has no
    /// type guard, and its dict lookups hash a *whole* float/Decimal exactly
    /// like the matching int (`ORDINAL_ONES[5.0]` hits key `5`), so whole
    /// values (±0.0 included) behave exactly like the int port. A fractional
    /// value raised `KeyError` below 100 and glued "երորդ" onto the float
    /// cardinal above it; it now raises Base's `errmsg_floatord` `TypeError`,
    /// as `to_ordinal_num` does (bug 4, #158).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if fv_eq_zero(value) {
            return Ok("զրոերորդ".to_string());
        }
        match value.as_whole_int() {
            Some(i) => self.to_ordinal(&i),
            None => Err(floatord_error(py_num_str(value))),
        }
    }

    /// `to_ordinal_num(float/Decimal)`: base `verify_ordinal(value)` — float
    /// check first, then sign (`abs(-0.0) == -0.0` passes, so "-0.0-րդ") —
    /// then `str(value) + "-րդ"`. `%s` interpolates `str(value)` == `repr_str`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            ))),
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )))
                } else {
                    Ok(format!("{}-րդ", repr_str))
                }
            }
        }
    }

    /// `to_year(float/Decimal)`: the `1000 <= val < 2000` comparison is
    /// numeric, so it holds for floats/Decimals too; the "մեկ հազար" strip is
    /// the same dead-in-practice guard as the int port. Everything renders
    /// through HY's own cardinal (whole -> int path, fractional -> float
    /// grammar) plus " թվական".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        if fv_ge(value, 1000) && fv_lt(value, 2000) {
            let mut year_str = self.cardinal_float_entry(value, None)?;
            if year_str.starts_with("մեկ հազար") {
                // Python's `[4:]` is a character slice ("մեկ " is 4 chars).
                year_str = year_str
                    .chars()
                    .skip(4)
                    .collect::<String>()
                    .trim()
                    .to_string();
            }
            return Ok(format!("{} թվական", year_str));
        }
        Ok(format!("{} թվական", self.cardinal_float_entry(value, None)?))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which HY does not
    /// override. Infinity/NaN parse fine as Decimals, but HY's `to_cardinal`
    /// then dies in *decimal* arithmetic, not in `int()`: `Infinity >= 10**6`
    /// passes and `Infinity % 10**9` raises `decimal.InvalidOperation`, while
    /// `NaN >= 10**6` (an ordered comparison against a NaN) raises
    /// `decimal.InvalidOperation` directly. The binding's shared sentinels
    /// would report OverflowError/ValueError instead, so the interception
    /// happens here, matching all three pinned cardinal rows.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } | ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".to_string(),
            }),
            other => Ok(other),
        }
    }
}
