//! Port of `lang_HY.py` (Armenian).
//!
//! Shape: **engine, with a self-contained wrapper**. `Num2Word_HY` subclasses
//! `Num2Word_Base` and *does* define `high_numwords`/`mid_numwords`/
//! `low_numwords` in `setup`, so Python builds `self.cards` and sets
//! `MAXVAL = 1000 * cards.keys()[0]` (but see bug 1). It also overrides `to_cardinal` with
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
//! 1. ~~**`set_high_numwords` stores tuples as card words.**~~ `setup`
//!    builds `high_numwords = [(10**12, "տրիլիոն"), (10**9, "միլիարդ"),
//!    (10**6, "միլիոն")]` — a list of *pairs* — and `set_high_numwords`
//!    stores each pair as the card word at 10**33, 10**23 and 10**13 (step
//!    -10, not -3). Python's `merge` then formats the tuple, so
//!    `to_cardinal(10**13)` == "մեկ (1000000, 'միլիոն')", and MAXVAL comes out
//!    as 10**36. Fixed (gladiaio/num2words2#215): no high card is built,
//!    `to_cardinal` reads everything from 10**6 up on the three scale words
//!    itself, and `maxval` is 10**15 (1000 x տրիլիոն).
//! 2. **`merge` adds where it should multiply.** Every arm returns
//!    `cnum + nnum`, so 200 is tracked as 2+100=102. The numbers are only used
//!    for `merge`'s own comparisons, so the text still comes out right for
//!    the values below 10**6 the engine now sees.
//! 3. ~~**The "հազար հազար" → "միլիոն" patch.**~~ With no million card the
//!    Python engine rendered 10**6 as "հազար հազար", 10**12 as "միլիոն
//!    միլիոն" after a string `.replace()`, and 1234567890 as "հազար երկու
//!    հարյուր … հազար …". Fixed with bug 1: the engine never sees 10**6 or
//!    more. The float/Decimal twins keep the `.replace()` for parity on the
//!    non-integral values they still hand to the engine.
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
//! consequences (no `CURRENCY_PRECISION`, no `separator`/`adjective`; an
//! unknown code, which Python answered with a bare cardinal, now raises like
//! the base implementation, #219). `to_cheque` is *not* overridden.
//!
//! Because `to_currency` hands floats to `to_cardinal`, this file must also
//! carry the inherited float-cardinal path (`float2tuple` /
//! `to_cardinal_float` / a float twin of `splitnum`). That arithmetic is done in `f64` because
//! Python does it in `f64`; see the "CPython float semantics" note below for
//! why an exact-decimal model is measurably wrong.
//!
//! # `Decimal` is context-bound, not exact
//!
//! Python's `to_currency` computes `(Decimal(str(val)) * 100) % 1` under the
//! default 28-digit decimal context, which raised
//! `InvalidOperation(DivisionImpossible)` for every `abs(val) >= 1e26`. The
//! port's arithmetic is exact and the 10^15 ceiling is checked first, so that
//! raise is gone (gladiaio/num2words2#215).
//!
//! # Error variants
//!
//! * `to_ordinal(n)` for `n < 0` → `N2WError::Type` (bug 4, fixed).
//! * `to_ordinal_num(n)` for `n < 0` → `N2WError::Type` (`verify_ordinal`).
//! * every mode for `abs(n) >= 10**15` → `N2WError::Overflow` (#215; Python's
//!   MAXVAL was 10**36, see bug 1).
//! * `to_cheque(v, cur)` for a code outside `CURRENCY_FORMS` →
//!   `N2WError::NotImplemented`. `to_currency` never raises for that case.

use crate::base::{
    check_maxval, default_to_cardinal, floatord_error, py_num_str, set_low_numwords,
    set_mid_numwords, verify_ordinal, Cards, Lang, N2WError, Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

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

        // Python's `set_high_numwords(high_numwords)` stored tuples as card
        // words at 10**33/10**23/10**13 (bug 1). None is built here: every
        // value from 10**6 up is read by `to_cardinal` itself, so the engine
        // only ever sees values below 10**6.

        // Python's `mid_numwords`.
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

        // Python's MAXVAL was 10**36 (1000 x the misplaced 10**33 tuple card).
        // The real ceiling is 1000 x the largest scale word, տրիլիոն (#215).
        let maxval = BigInt::from(10u8).pow(15);

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
        // Integral floats take the integer reading (#215); Python's float
        // splitnum met the tuple cards of bug 1 there.
        let mut result = if value == value.trunc() {
            self.to_cardinal(&f64_trunc_to_bigint(value)?)?
        } else {
            self.to_cardinal_float(value)?
        };

        if result.contains("հազար հազար") {
            result = result.replace("հազար հազար", "միլիոն");
        }
        Ok(result)
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
    ///
    /// Python only short-circuited the millions range and exact multiples of
    /// 10**9; everything else above 10**6 went to the engine, which has no
    /// million/billion/trillion card (bug 1) and so read 1234567890 as
    /// "հազար երկու հարյուր … հազար …" and 10**13 with a tuple in it. Every
    /// value from 10**6 up to the 10**15 ceiling is now split on the three
    /// scale words here (gladiaio/num2words2#215); the engine keeps the rest.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{}{}", self.negword(), self.to_cardinal(&-value)?));
        }
        check_maxval(value, &self.maxval)?;
        if value.is_zero() {
            return Ok("զրո".to_string());
        }

        // Simple cases
        if value == &self.thousand {
            return Ok("հազար".to_string());
        }

        for (scale, word) in [
            (&self.trillion, "տրիլիոն"),
            (&self.billion, "միլիարդ"),
            (&self.million, "միլիոն"),
        ] {
            if value >= scale {
                let (count, rest) = value.div_rem(scale);
                let part = format!("{} {}", self.to_cardinal(&count)?, word);
                if rest.is_zero() {
                    return Ok(part);
                }
                return Ok(format!("{} {}", part, self.to_cardinal(&rest)?));
            }
        }

        // Below 10**6: the standard implementation (super()).
        default_to_cardinal(self, value)
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
    /// 3. **An unknown code raises (fixed, #219).** Python's `else` returned
    ///    `to_cardinal(abs(val))`, so `to_currency(12.34, "KWD")` was
    ///    "տասներկու ամբողջ երեք չորս" — no currency at all. The port raises
    ///    NotImplementedError, like the base and `to_cheque`.
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

        // Python's `(decimal_val * 100) % 1` ran under the 28-digit default
        // decimal context and raised InvalidOperation from 1e26, far below
        // its MAXVAL. The arithmetic here is exact, and the ceiling is checked
        // up front instead, before even the unknown-code branch (#215).
        check_maxval(&decimal_val.with_scale(0).as_bigint_and_exponent().0, &self.maxval)?;

        let has_fractional_cents = (&scaled - scaled.with_scale(0)) != BigDecimal::zero();

        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            // Python's `else: return self.to_cardinal(val)` printed a bare
            // number for an unknown code; raise instead (#219).
            None => return Err(crate::currency::unknown_currency(self, currency)),
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
        } else if cents_val.is_zero() {
            // Python skips both segments for a zero amount and returns "";
            // spell it like any other whole amount instead (#160).
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
