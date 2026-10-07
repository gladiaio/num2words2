//! Port of `lang_BN.py` (Bengali / Bangladesh format).
//!
//! Shape: **self-contained**. `Num2Word_BN` has **no base class at all** — it
//! is a bare `class Num2Word_BN:` that does not subclass `Num2Word_Base`, so
//! there is no inheritance chain to chase and nothing from `base.py` applies.
//! It defines its own `to_cardinal`/`to_ordinal`/`to_ordinal_num`/`to_year`
//! and never builds `cards`, `MAXVAL` or `merge`. Those trait items therefore
//! stay at their defaults here and are unreachable.
//!
//! Registry check: `__init__.py` line 228 has `"bn": lang_BN.Num2Word_BN()`,
//! so the key really does resolve to the class in this file.
//!
//! Numbering is the Indian/Bangladeshi system: হাজার (10^3), লাখ (10^5),
//! কোটি (10^7), and `_number_to_bengali_word` recurses on the কোটি quotient,
//! which is why large values stack the word: 10^15 == "দশ কোটি কোটি".
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Each item below is wrong-looking but is
//! exactly what CPython emits; all were verified against the interpreter.
//!
//! 1. **The 28-significant-digit rounding bug** (the big one). Every entry
//!    point funnels through `str_to_number`:
//!
//!    ```python
//!    return abs(Decimal(str(number)))
//!    ```
//!
//!    `Decimal(str(n))` is exact, but `abs()` on a `Decimal` is **not** a
//!    plain sign flip — `Decimal.__abs__` dispatches to `__pos__`/`__neg__`,
//!    which call `_fix(context)` and round to the active context's precision.
//!    The default context is `prec=28, ROUND_HALF_EVEN`, so any input with
//!    more than 28 significant digits is silently rounded before a single
//!    word is emitted. Verified:
//!
//!    * `to_cardinal(int("1"*29))` ends "...একশত দশ" (110), not "...এগারো"
//!      (111), because 11111111111111111111111111111 is rounded down to
//!      11111111111111111111111111110.
//!    * Ties really do go half-even: `10**28 + 5` → `...000`, but
//!      `10**28 + 15` → `...020`.
//!
//!    Modelled by [`round_to_28_significant`]. The frozen corpus tops out at
//!    22 digits, so no corpus row exercises this — but the ceiling checks
//!    below do depend on it.
//!
//! 2. **`MAX_NUMBER` itself is rejected**, as a direct consequence of bug 1.
//!    `MAX_NUMBER` is 10^307 - 1 (307 nines) and the guard is an inclusive
//!    `MAX_NUMBER >= number`, so the constant ought to be accepted. But
//!    `str_to_number` rounds 307 nines *up* to 10^307, which then fails the
//!    guard. `to_cardinal(10**307 - 1)` raises `NumberTooLargeError` even
//!    though the value equals the documented maximum. The true ceiling is
//!    `9999999999999999999999999999 * 10**279` (the largest value that is
//!    already 28-significant and stays under 10^307).
//!
//! 3. ~~**`to_ordinal_num` and `to_year` silently discard the sign.**~~
//!    Both did `self.to_cardinal(int(abs(number)))`, so `to_year(-44)` ==
//!    `to_year(44)`, and `to_ordinal` was a bare alias for `to_cardinal`.
//!    Fixed (gladiaio/num2words2#250): `to_ordinal` returns the ordinal
//!    word (প্রথম … দশম, then cardinal + "তম"), `to_ordinal_num` the
//!    abbreviation (১ম, ২য়, ৩য়, ৪র্থ, ৬ষ্ঠ, ১১তম, …), both reject a
//!    negative with Base's TypeError, and a negative year reads
//!    "খ্রিস্টপূর্ব … সাল".
//!
//! 4. **`DOSOK[61]` is `"একাত্তর "` with a stray trailing space** where every
//!    other entry has none. It is invisible in practice: the tens branch is
//!    always the last fragment appended (it sets `number = 0`, blocking the
//!    units branch), so the doubled space only ever lands at the end of the
//!    string and `words.strip()` eats it. Preserved verbatim anyway — if a
//!    later edit ever appends after the tens branch, the space becomes
//!    observable exactly as it would in Python.
//!
//! 5. ~~**The ordinal suffix was picked by a bare `endswith("ত")` test**~~,
//!    so 100 read "একশতম" for একশততম. Fixed (#250): every ordinal past
//!    দশম takes "তম".
//!
//! # The currency surface
//!
//! `Num2Word_BN` defines `to_currency` **from scratch** and has no base class,
//! so none of `base.py`'s currency machinery applies. Consequences, all
//! interpreter-verified:
//!
//! 6. **The `currency` argument is read once and then completely ignored.**
//!    There is no `CURRENCY_FORMS`, no `CURRENCY_PRECISION`, no
//!    `CURRENCY_ADJECTIVES`, no `pluralize`; the output is hardcoded to টাকা
//!    (taka) and পয়সা (paisa). `to_currency(12.34, "JPY")`,
//!    `(12.34, "KWD")` and even `(12.34, "ZZZZZZ")` all return
//!    "বারো টাকা চৌত্রিশ পয়সা". So BN **never** raises the
//!    `Currency code "X" not implemented` NotImplementedError, and the
//!    0-decimal (JPY) and 3-decimal (KWD/BHD) divisors never apply — all 108
//!    corpus rows across 9 currency codes are byte-identical per `arg`.
//!    `separator`, `cents` and `adjective` are ignored likewise.
//!
//! 7. **`to_currency` silently discards the sign.** It routes through
//!    `str_to_number` (`abs(...)`) and never consults `negword`, so
//!    `to_currency(-12.34)` == `to_currency(12.34)`. Corpus-confirmed.
//!
//! 8. ~~**`parse_paisa`'s trailing-zero fix corrupts leading zeros.**~~ It did
//!    `str(int(paisa_str) * 100)[:2]` to re-pad a `Decimal`-stripped trailing
//!    zero, but `int()` also eats *leading* zeros, so `0.01` → "01" → `int` 1
//!    → 100 → `"100"[:2]` == "10" → **10 paisa**, and 3.05 taka read as 3.50.
//!    Fixed (gladiaio/num2words2#255): the paisa are the first two fraction
//!    digits, right-padded ("05" → 5, "5" → 50). Still truncating rather than
//!    rounding, so `2.675` → 67 paisa, and clamped to 0..=99.
//!
//! 9. **`to_cheque` does not exist.** No base class means no inherited
//!    `to_cheque`, so Python raised `AttributeError`; the port raises
//!    NotImplementedError ("does not support to='cheque'", #223).
//!
//! 10. ~~**`(Decimal(str(val)) * 100) % 1` raises for `abs(val) >= 1e26`.**~~
//!     The `has_fractional_cents` probe ran under the *default* decimal
//!     context (`prec=28`), so `to_currency(10**26)` raised
//!     `InvalidOperation(DivisionImpossible)` while `to_cardinal(10**26)`
//!     answered. Fixed (gladiaio/num2words2#215): the probe's result is never
//!     observable (see below), so the port no longer evaluates it.
//!
//! ## The `has_fractional_cents` branch is dead code
//!
//! `to_currency` computes `has_fractional_cents` and branches on it, picking
//! `to_cardinal(float(decimal_part))` over `_number_to_bengali_word(decimal_part)`.
//! The two are **indistinguishable**: `parse_paisa` clamps `decimal_part` to
//! the integers 0..=99 via `[:2]`, and for every one of those
//! `to_cardinal(float(dp)) == _number_to_bengali_word(dp)` — checked
//! exhaustively against the interpreter for `dp` in 1..=99, zero differences
//! (`decimal_part == 0` is intercepted earlier by the "শূন্য পয়সা" branch).
//! So the fractional-cents path collapses into the plain one, and the currency
//! surface needs no float cardinal: `cardinal_from_decimal` stays at its
//! default, per PORTING_CURRENCY.md. Only the *raise* in bug 10 is observable,
//! not the flag.
//!
//! This survives [`LangBn::to_cardinal_float`] existing. `to_cardinal(float(dp))`
//! for an integral `dp` re-enters that same method, where `str(34.0)` is "34.0"
//! → `parse_number` finds a fraction of "0" → `int("0") == 0` → the
//! `decimal_part > 0` branch is skipped → it returns exactly
//! `_number_to_bengali_word(dp)`. The two branches stay indistinguishable, so
//! `to_currency` deliberately keeps calling `number_to_bengali_word` directly
//! rather than routing back through the float path.
//!
//! # Error variants
//!
//! * `to_cardinal`/`to_ordinal`/`to_ordinal_num`/`to_year` past `MAX_NUMBER` →
//!   BN's own `NumberTooLargeError`, a bare `Exception` subclass defined in
//!   `lang_BN.py`. Modelled as `N2WError::Custom { module:
//!   "num2words2.lang_BN", class: "NumberTooLargeError" }` — the binding
//!   imports and raises the real class, so `except NumberTooLargeError` keeps
//!   working. `base.rs` documents this file as the motivating case for the
//!   variant.
//! * `to_currency` for `abs(val) >= MAX_NUMBER` → the same
//!   `NumberTooLargeError`, but with a **different message** ("Number is too
//!   large. Max: ..." vs `_is_smaller_than_max_number`'s "Too Large number
//!   maximum value=...").
//! * `to_cheque` → `N2WError::NotImplemented` for every input (bug 9).
//!
//! # The float/Decimal cardinal path
//!
//! `Num2Word_BN` has **no `to_cardinal_float`** — it has no base class, so
//! there is nothing to inherit one from. `to_cardinal` takes the float or
//! `Decimal` directly and handles it inline, which means `base.float2tuple`
//! and `Num2Word_Base.to_cardinal_float` are **never involved**. Two
//! consequences that invert the usual porting advice:
//!
//! * **No f64 artefacts.** The usual trap is that `float2tuple` computes
//!   `abs(value - pre) * 10**precision` in binary and `2.675` yields
//!   `674.9999999999998`. BN never does that arithmetic: `str_to_number` is
//!   `abs(Decimal(str(number)))`, so the digits come from `repr(float)` —
//!   shortest round-trip — and are then exact in base 10. `2.675` really does
//!   give "ছয় সাত পাঁচ" (6-7-5), which the corpus confirms. Reconstructing
//!   from the decimal string is here the *right* answer, not the wrong one.
//! * **No `pointword`, no `precision`.** BN defines neither. The separator is
//!   the hardcoded [`DOSHOMIK`], and `num2words(..., precision=N)` is a silent
//!   no-op for BN: the dispatcher only applies it `if hasattr(converter,
//!   "precision")`, and BN has no such attribute. `precision_override` is
//!   therefore ignored — interpreter-verified, `precision=1` and `precision=8`
//!   on `3.14159` both give all five fractional digits.
//!
//! ## Further faithfully reproduced Python bugs (float path)
//!
//! 11. ~~**`int()` eats the fraction's leading zeros.**~~ `parse_number`
//!     recovers the digit run after the "." and then calls `int()` on it,
//!     collapsing "01" to 1 and "005" to 5, so in Python `0.01` read
//!     "শূন্য দশমিক এক" ("zero point *one*") and `0.05` read like `0.5`. Fixed
//!     (gladiaio/num2words2#205): the port reads the digit run itself, so
//!     `0.05` is "শূন্য দশমিক শূন্য পাঁচ".
//!
//! 12. ~~**An interior zero digit emits a bare double space.**~~ `AKOK[0]` is
//!     `""`, so in Python `1.102` read `'এক দশমিক এক  দুই'`. Fixed with 11:
//!     every zero digit reads "শূন্য" ("এক দশমিক এক শূন্য দুই"); trailing zeros
//!     of a Decimal (`Decimal("1.10")`) are still dropped, as Python's strip
//!     did.
//!
//! 13. **The 1e-7 cliff, and a `ValueError` just past it.** `parse_number`
//!     splits `str(fraction)` on "." — but `str(Decimal)` switches to
//!     scientific notation once `adjusted_exp < -6`, and then there *is* no
//!     ".". So `1e-07` → "1E-7" → no dot → decimal_part 0 → plain "শূন্য",
//!     whereas `1e-05` → "0.00001" → "এক". Worse, a scientific string with a
//!     mantissa splits to a garbage tail: `1.5e-07` → "1.5E-7" → `int("5E-7")`
//!     → **ValueError**. All three interpreter-verified; [`frac_after_dot`]
//!     already models the to-sci-string rule, so this falls out for free.
//!
//! 14. **Non-finite floats crash in `int()`.** `Decimal("inf")` parses fine and
//!     `abs()` passes it through, but `parse_number`'s `int(number)` raises
//!     `OverflowError("cannot convert Infinity to integer")`; `nan` raises
//!     `ValueError("cannot convert NaN to integer")`. Nothing observable
//!     happens between `str_to_number` and that `int()`, so
//!     [`float_str_to_number`] raises them up front. Verified.

use crate::base::{strictly_negative, verify_ordinal, verify_ordinal_float, Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::str::FromStr;
use std::sync::OnceLock;

/// NFC spelling: য় is য + nukta (U+09AF U+09BC), as in the cardinal tables;
/// the precomposed U+09DF is a composition exclusion that NFC never emits.
const RANKING: &[&str] = &[
    "", // 0 — unreachable: `number in range(1, 11)` excludes 0
    "প্রথম", // 1
    "দ্বিতীয়", // 2
    "তৃতীয়", // 3
    "চতুর্থ", // 4
    "পঞ্চম", // 5
    "ষষ্ঠ", // 6
    "সপ্তম", // 7
    "অষ্টম", // 8
    "নবম", // 9
    "দশম", // 10
];

const AKOK: &[&str] = &[
    "", // 0
    "এক", // 1
    "দুই", // 2
    "তিন", // 3
    "চার", // 4
    "পাঁচ", // 5
    "ছয়", // 6
    "সাত", // 7
    "আট", // 8
    "নয়", // 9
];

/// `DOSOK[i]` is the word for `i + 10`, covering 10..=99.
const DOSOK: &[&str] = &[
    "দশ", // 10
    "এগারো", // 11
    "বারো", // 12
    "তেরো", // 13
    "চৌদ্দ", // 14
    "পনের", // 15
    "ষোল", // 16
    "সতের", // 17
    "আঠারো", // 18
    "উনিশ", // 19
    "বিশ", // 20
    "একুশ", // 21
    "বাইশ", // 22
    "তেইশ", // 23
    "চব্বিশ", // 24
    "পঁচিশ", // 25
    "ছাব্বিশ", // 26
    "সাতাশ", // 27
    "আটাশ", // 28
    "উনত্রিশ", // 29
    "ত্রিশ", // 30
    "একত্রিশ", // 31
    "বত্রিশ", // 32
    "তেত্রিশ", // 33
    "চৌত্রিশ", // 34
    "পঁইত্রিশ", // 35
    "ছত্রিশ", // 36
    "সাতত্রিশ", // 37
    "আটত্রিশ", // 38
    "উনচল্লিশ", // 39
    "চল্লিশ", // 40
    "একচল্লিশ", // 41
    "বিয়াল্লিশ", // 42
    "তেতাল্লিশ", // 43
    "চৌচল্লিশ", // 44
    "পঁয়তাল্লিশ", // 45
    "ছেচল্লিশ", // 46
    "সাতচল্লিশ", // 47
    "আটচল্লিশ", // 48
    "উনপঞ্চাশ", // 49
    "পঞ্চাশ", // 50
    "একান্ন", // 51
    "বাহান্ন", // 52
    "তিপ্পান্ন", // 53
    "চুয়ান্ন", // 54
    "পঞ্চান্ন", // 55
    "ছাপ্পান্ন", // 56
    "সাতান্ন", // 57
    "আটান্ন", // 58
    "উনষাট", // 59
    "ষাট", // 60
    "একষট্টি", // 61
    "বাষট্টি", // 62
    "তেষট্টি", // 63
    "চৌষট্টি", // 64
    "পঁয়ষট্টি", // 65
    "ছিষট্টি", // 66
    "সাতষট্টি", // 67
    "আটষট্টি", // 68
    "উনসত্তর", // 69
    "সত্তর", // 70
    "একাত্তর ", // 71 — NOTE the trailing space; see module bug 4
    "বাহাত্তর", // 72
    "তিয়াত্তর", // 73
    "চুয়াত্তর", // 74
    "পঁচাত্তর", // 75
    "ছিয়াত্তর", // 76
    "সাতাত্তর", // 77
    "আটাত্তর", // 78
    "উনআশি", // 79
    "আশি", // 80
    "একাশি", // 81
    "বিরাশি", // 82
    "তিরাশি", // 83
    "চুরাশি", // 84
    "পঁচাশি", // 85
    "ছিয়াশি", // 86
    "সাতাশি", // 87
    "আটাশি", // 88
    "উননব্বই", // 89
    "নব্বই", // 90
    "একানব্বই", // 91
    "বিরানব্বই", // 92
    "তিরানব্বই", // 93
    "চুরানব্বই", // 94
    "পঁচানব্বই", // 95
    "ছিয়ানব্বই", // 96
    "সাতানব্বই", // 97
    "আটানব্বই", // 98
    "নিরানব্বই", // 99
];

const HAZAR: &str = " হাজার ";
const LAKH: &str = " লাখ ";
const KOTI: &str = " কোটি ";
const SHATA: &str = "শত ";
const ZERO_WORD: &str = "শূন্য";
const NEGWORD: &str = "ঋণাত্মক";
const SHAL: &str = " সাল";

/// The era word for a negative year: খ্রিস্টপূর্ব, "before Christ" (#250).
const BCE: &str = "খ্রিস্টপূর্ব ";

/// Bengali digits ০..৯, for `to_ordinal_num`.
const BN_DIGITS: [char; 10] = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];
/// দশমিক — the fractional separator, interpolated as `f" দশমিক{...}"`. BN's
/// stand-in for `pointword`, which it does not define (the trait default
/// "(.)" is therefore unreachable here).
const DOSHOMIK: &str = " দশমিক";
/// টাকা — the currency unit `to_currency` hardcodes, for every code (bug 6).
const TAKA: &str = "টাকা";
/// পয়সা — likewise the hardcoded subunit.
const PAISA: &str = "পয়সা";

/// Python's `MAX_NUMBER`, written out as 307 nines: `10**307 - 1`.
///
/// Verified against the interpreter rather than transcribed by eye — the
/// literal in `lang_BN.py` is a single unbroken run of 307 nines.
fn max_number() -> BigInt {
    BigInt::from(10u8).pow(307) - BigInt::one()
}

/// Python's default decimal context precision.
const DECIMAL_PREC: u64 = 28;

fn number_too_large() -> N2WError {
    // Python: raise NumberTooLargeError(f"Too Large number maximum value={MAX_NUMBER}")
    //
    // NumberTooLargeError subclasses Exception, NOT OverflowError, so callers
    // doing `except NumberTooLargeError` would miss an OverflowError. The
    // binding imports the real class out of lang_BN and raises it.
    N2WError::Custom {
        module: "num2words2.lang_BN",
        class: "NumberTooLargeError",
        msg: format!("Too Large number maximum value={}", max_number()),
    }
}

/// Reproduces `abs(Decimal(str(n)))` for integral `n`: the magnitude rounded
/// to 28 significant digits with `ROUND_HALF_EVEN` (module bug 1).
///
/// `n` must be non-negative (callers pass `|value|`). Rounding the magnitude
/// is sign-symmetric, so it does not matter that Python routes positives
/// through `__pos__` and negatives through `__neg__`.
pub fn round_to_28_significant(n: &BigInt) -> BigInt {
    if n.is_zero() {
        // Decimal `_fix` leaves a zero coefficient alone.
        return BigInt::zero();
    }
    let digits = n.to_string().len() as u64;
    if digits <= DECIMAL_PREC {
        return n.clone();
    }
    let drop = digits - DECIMAL_PREC;
    let divisor = BigInt::from(10u8).pow(drop as u32);
    let (q, r) = n.div_rem(&divisor);
    // divisor is 10^drop with drop >= 1, so it is even and half is exact.
    let half = &divisor / 2u8;
    let rounded = match r.cmp(&half) {
        std::cmp::Ordering::Greater => q + BigInt::one(),
        std::cmp::Ordering::Less => q,
        // Exact tie → ROUND_HALF_EVEN: round to the even coefficient.
        std::cmp::Ordering::Equal => {
            if q.is_odd() {
                q + BigInt::one()
            } else {
                q
            }
        }
    };
    rounded * divisor
}

/// `MAX_NUMBER`, cached. `max_number()` rebuilds `10**307` from scratch on
/// every call, which the `to_cardinal` family already pays once per call; the
/// currency guards below run on the hot path and compare against it twice, so
/// they read the cached copy instead. Same value, by construction.
fn max_number_ref() -> &'static BigInt {
    static MAX: OnceLock<BigInt> = OnceLock::new();
    MAX.get_or_init(max_number)
}

/// `MAX_NUMBER` as a `BigDecimal`, for the float arm of guard 1.
fn max_number_decimal() -> &'static BigDecimal {
    static MAX: OnceLock<BigDecimal> = OnceLock::new();
    MAX.get_or_init(|| BigDecimal::from(max_number_ref().clone()))
}

/// Python's `raise NumberTooLargeError(f"Number is too large. Max: {MAX_NUMBER}")`
/// — `to_currency`'s own guard, whose message differs from
/// `_is_smaller_than_max_number`'s. Same class, so the same `Custom` variant.
fn number_too_large_currency() -> N2WError {
    N2WError::Custom {
        module: "num2words2.lang_BN",
        class: "NumberTooLargeError",
        msg: format!("Number is too large. Max: {}", max_number_ref()),
    }
}

/// Reproduces `abs(Decimal(...))`'s `_fix` for a non-negative `BigDecimal`:
/// round the *coefficient* to at most `DECIMAL_PREC` significant digits with
/// `ROUND_HALF_EVEN`, compensating the exponent (module bug 1, decimal arm).
///
/// `BigDecimal`'s `(int_val, scale)` maps onto Python's `(coefficient, exp)`
/// with `exp == -scale`, verified for parsed reprs, so the exponent bookkeeping
/// carries straight over.
///
/// Unreachable from a `float`: `repr(float)` never exceeds 17 significant
/// digits. It fires only for a `Decimal` argument carrying more than 28, where
/// it matters a great deal — `Decimal("0." + "9"*30)` rounds up to exactly
/// `1.000...`, taking the paisa from 99 to 0.
fn fix_prec28_decimal(v: &BigDecimal) -> BigDecimal {
    let (coeff, scale) = v.as_bigint_and_exponent();
    if coeff.is_zero() {
        // `_fix` leaves a zero coefficient's exponent alone.
        return v.clone();
    }
    let digits = coeff.magnitude().to_string().len() as u64;
    if digits <= DECIMAL_PREC {
        return v.clone();
    }
    let drop = digits - DECIMAL_PREC;
    let divisor = BigInt::from(10u8).pow(drop as u32);
    let (q, r) = coeff.div_rem(&divisor);
    // divisor is 10^drop with drop >= 1, so it is even and half is exact.
    let half = &divisor / 2u8;
    let mut rounded = match r.cmp(&half) {
        std::cmp::Ordering::Greater => q + BigInt::one(),
        std::cmp::Ordering::Less => q,
        std::cmp::Ordering::Equal => {
            if q.is_odd() {
                q + BigInt::one()
            } else {
                q
            }
        }
    };
    let mut new_scale = scale - drop as i64;
    // `_fix` again: a carry can widen the coefficient to prec+1 digits (28
    // nines -> 10**28), and Python then divides it by 10 and bumps the
    // exponent. Value-preserving, but it changes the (coeff, exp) pair — and
    // `parse_paisa` reads the exponent through `str()`, so it is observable.
    if rounded.magnitude().to_string().len() as u64 > DECIMAL_PREC {
        rounded /= BigInt::from(10u8);
        new_scale -= 1;
    }
    BigDecimal::new(rounded, new_scale)
}

/// Reproduces `str(number - int(number)).split(".")[1:]` for `number >= 0`:
/// the digit run after the decimal point, or `None` when the string has no dot
/// (Python then falls back to the *integer* `0`, which is falsy).
///
/// This has to model Decimal's `to-sci-string` rather than just format the
/// value, because the choice between plain and scientific notation is what
/// decides whether a "." appears at all — and `parse_paisa` splits on it.
/// Python's rule: plain iff `exp <= 0 and adjusted_exp >= -6`, where
/// `adjusted_exp == exp + len(digits) - 1`. Hence `Decimal("0.000001")` prints
/// "0.000001" (→ 10 paisa) but `Decimal("0.0000001")` prints "1E-7" (→ 0
/// paisa), an abrupt cliff at 1e-7 that is verified against the interpreter.
fn frac_after_dot(number: &BigDecimal) -> Option<String> {
    let (coeff, scale) = number.as_bigint_and_exponent();
    // `number - int(number)` has exponent `min(number.exp, 0)`, and
    // `number.exp == -scale`.
    if scale <= 0 {
        // exp >= 0 → number is integral → frac is Decimal(0) at exp 0 → "0".
        return None;
    }
    let modulus = BigInt::from(10u8).pow(scale as u32);
    // frac's coefficient: coeff mod 10^scale. `number` is non-negative here.
    let f = coeff.abs().mod_floor(&modulus);
    let digits = f.to_string(); // "0" when f == 0, so len is always >= 1
    let exp = -scale;
    let adjusted = exp + digits.chars().count() as i64 - 1;
    if adjusted >= -6 {
        // Plain: "0." + the coefficient left-padded with zeros to `scale`.
        Some(crate::strnum::zero_pad_left(&digits, scale as usize))
    } else if digits.chars().count() == 1 {
        // Scientific with a single digit: "1E-8" — no ".", so no fraction.
        None
    } else {
        // Scientific with more: "1.5E-8" splits to ["5E-8"], which `int()`
        // then chokes on. Reproduced verbatim so the ValueError message
        // matches; `adjusted` is negative here, so `{}` renders the "-".
        let rest: String = digits.chars().skip(1).collect();
        Some(format!("{}E{}", rest, adjusted))
    }
}

/// Python's `parse_paisa`. Returns `(int(number), paisa)` for `number >= 0`.
///
/// ```python
/// paisa = str(number - int(number)).split(".")[1:]
/// paisa_str = "".join(paisa) if paisa else 0
/// if paisa_str:
///     paisa_str = str(int(paisa_str) * 100)[:2]
/// return int(number), int(paisa_str)
/// ```
///
/// The `* 100` then `[:2]` is the buggy trailing-zero fix of module bug 8,
/// replaced by a right-pad of the digit run (#255).
fn parse_paisa(number: &BigDecimal) -> Result<(BigInt, u32)> {
    // int(number) — truncation, and `number` is non-negative here.
    let int_part = number.with_scale(0).as_bigint_and_exponent().0;

    let paisa = match frac_after_dot(number) {
        // `paisa_str = 0`, the int — falsy, so the fix-up is skipped and
        // `int(paisa_str)` is `int(0)`.
        None => 0u32,
        // `if paisa_str:` — any non-empty string is truthy, "0" included.
        Some(s) => {
            BigInt::from_str(&s).map_err(|_| {
                // int("5E-8") → ValueError, Python's message verbatim.
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
            })?;
            // The first two fraction digits, right-padded: "05" -> 5, "5" -> 50,
            // "675" -> 67 (truncating, never rounding). Python's
            // `str(int(paisa_str) * 100)[:2]` dropped the leading zero first,
            // so 3.05 read as 50 paisa (bug 8, #255).
            let two: String = format!("{:0<2}", s).chars().take(2).collect();
            two.parse::<u32>()
                .expect("two decimal digits of the fraction")
        }
    };
    Ok((int_part, paisa))
}

/// Python's `parse_number`. Returns `(int(number), decimal_part)` for
/// `number >= 0`.
///
/// ```python
/// with localcontext() as ctx:
///     ctx.prec = max(len(number.as_tuple().digits) + 1, ctx.prec)
///     fraction = number - int(number)
/// dosomik = str(fraction).split(".")[1:]
/// dosomik_str = "".join(dosomik) if dosomik else 0
/// return int(number), int(dosomik_str)
/// ```
///
/// The `localcontext` widening exists to make the subtraction *exact* (its
/// docstring blames lang_AR for globally raising `prec` and never restoring
/// it). Exact is precisely what [`frac_after_dot`] computes, so it ports
/// straight across: `str_to_number` has already capped the coefficient at 28
/// digits, so `prec` lands at 29 and the subtraction cannot round.
///
/// Unlike [`parse_paisa`] there is no `* 100` / `[:2]` fix-up — the digit run
/// goes straight into `int()`, which is what eats the leading zeros (module
/// bug 11) and what chokes on a scientific tail (module bug 13).
fn parse_number(number: &BigDecimal) -> Result<(BigInt, BigInt, String)> {
    // int(number) — truncation, and `number` is non-negative here.
    let int_part = number.with_scale(0).as_bigint_and_exponent().0;

    let (decimal_part, digits) = match frac_after_dot(number) {
        // `dosomik_str = 0`, the int — so `int(dosomik_str)` is `int(0)`.
        None => (BigInt::zero(), String::new()),
        // `int(dosomik_str)` validates the run (and in Python dropped its
        // leading zeros, bug 11); the digit string itself is kept for
        // reading, so 0.05 no longer reads like 0.5.
        Some(s) => (
            BigInt::from_str(&s).map_err(|_| {
                // int("5E-7") → ValueError, Python's message verbatim (bug 13).
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
            })?,
            s,
        ),
    };
    Ok((int_part, decimal_part, digits))
}

/// `Decimal(str(value))` for an `f64` — the float arm of `str_to_number`.
///
/// Python's `repr(float)` and Rust's `{}` for `f64` are both shortest
/// round-trip, so they emit the **same digits**. They differ only in framing:
/// Python switches to scientific notation outside `1e-4 ..< 1e16`, Rust always
/// prints positionally. Neither difference can reach the output:
///
/// * Below 1e-4 — `repr(1e-05) == '1e-05'` vs Rust `"0.00001"` — the two parse
///   to the *identical* `(coefficient, exponent)`, because leading zeros are
///   not significant and both discard them. This is the case that matters:
///   it is the only one where a fraction survives, and [`frac_after_dot`]
///   reads exactly that pair.
/// * At or above 1e16 — `repr(1e16) == '1e+16'` vs Rust `"10000000000000000"`
///   — the pairs differ by trailing zeros. Unobservable: every f64 that large
///   is an integer, so `parse_number` finds no fraction either way, and the
///   padding zeros only ever divide out exactly in [`fix_prec28_decimal`],
///   preserving the value. Checked up to `f64::MAX`.
///
/// Non-finite input raises here rather than at `int(number)` where Python
/// raises it (module bug 14); nothing observable sits in between.
fn float_str_to_number(value: f64) -> Result<BigDecimal> {
    if value.is_nan() {
        return Err(N2WError::Value("cannot convert NaN to integer".to_string()));
    }
    if value.is_infinite() {
        return Err(N2WError::Overflow(
            "cannot convert Infinity to integer".to_string(),
        ));
    }
    // `-0.0` prints as "-0", which parses to an unsigned zero — matching
    // `abs(Decimal("-0.0"))`. The sign was already read off the raw f64.
    BigDecimal::from_str(&format!("{}", value)).map_err(|e| N2WError::Value(e.to_string()))
}

/// Python's `int(abs(number))` — truncation toward zero, sign dropped — for
/// either [`FloatValue`] arm. Non-finite floats never reach this (the
/// dispatcher's `_finite` guard keeps them on the Python side, and the string
/// path intercepts Inf/NaN in `str_to_number`).
fn fv_trunc_abs(value: &FloatValue) -> Result<BigInt> {
    let d = match value {
        FloatValue::Float { value, .. } => float_str_to_number(*value)?,
        FloatValue::Decimal { value, .. } => value.clone(),
    };
    // `with_scale(0)` truncates the fractional digits (toward zero for the
    // non-negative magnitude), matching Python's `int()`.
    Ok(d.abs().with_scale(0).as_bigint_and_exponent().0)
}

pub struct LangBn;

impl LangBn {
    /// `self._is_smaller_than_max_number(number)` with the RAW float/Decimal
    /// argument: `MAX_NUMBER >= number`, so any negative passes trivially and
    /// only a magnitude beyond ~1e307 raises `NumberTooLargeError`.
    fn check_max_float(&self, value: &FloatValue) -> Result<()> {
        let raw = match value {
            // Finite by the time it reaches a mode hook; "-0.0" prints "-0",
            // which parses fine.
            FloatValue::Float { value, .. } => float_str_to_number(*value)?,
            FloatValue::Decimal { value, .. } => value.clone(),
        };
        if raw > *max_number_decimal() {
            return Err(number_too_large());
        }
        Ok(())
    }
    pub fn new() -> Self {
        LangBn
    }

    /// Python's `_is_smaller_than_max_number`: `if MAX_NUMBER >= number: True`
    /// else raise. Note the guard is inclusive and takes the value *as given* —
    /// callers differ on whether they have already rounded/abs'd it.
    fn check_max(&self, number: &BigInt) -> Result<()> {
        if max_number() >= *number {
            Ok(())
        } else {
            Err(number_too_large())
        }
    }

    /// Python's `_number_to_bengali_word`.
    ///
    /// Recursive over কোটি (10^7), লাখ (10^5) and হাজার (10^3). Each branch
    /// reduces `number` before the next is tested, so by the hundreds branch
    /// `number < 1000` and `number / 100` is always in 1..=9 — `AKOK` can
    /// never be indexed out of range here.
    fn number_to_bengali_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        let mut number = number.clone();
        let mut words = String::new();

        let koti = BigInt::from(10u8).pow(7);
        if number >= koti {
            words.push_str(&self.number_to_bengali_word(&(&number / &koti)));
            words.push_str(KOTI);
            number %= &koti;
        }

        let lakh = BigInt::from(10u8).pow(5);
        if number >= lakh {
            words.push_str(&self.number_to_bengali_word(&(&number / &lakh)));
            words.push_str(LAKH);
            number %= &lakh;
        }

        let thousand = BigInt::from(1000u16);
        if number >= thousand {
            words.push_str(&self.number_to_bengali_word(&(&number / &thousand)));
            words.push_str(HAZAR);
            number %= &thousand;
        }

        let hundred = BigInt::from(100u8);
        if number >= hundred {
            let idx = usize::try_from(&number / &hundred).expect("number/100 is 1..=9 here");
            words.push_str(AKOK[idx]);
            words.push_str(SHATA);
            number %= &hundred;
        }

        // Python: `if 10 <= number <= 99`. Always the final fragment — it
        // zeroes `number`, so the units branch below cannot also fire.
        if number >= BigInt::from(10u8) && number <= BigInt::from(99u8) {
            let idx = usize::try_from(&number - BigInt::from(10u8)).expect("0..=89 here");
            words.push_str(DOSOK[idx]);
            words.push(' ');
            number = BigInt::zero();
        }

        // Python: `if 0 < number < 10`.
        if number > BigInt::zero() && number < BigInt::from(10u8) {
            let idx = usize::try_from(&number).expect("1..=9 here");
            words.push_str(AKOK[idx]);
            words.push(' ');
        }

        // Python's str.strip() — only ASCII spaces are ever produced here, so
        // trim() is equivalent.
        words.trim().to_string()
    }

    /// Python's `_dosomik_to_bengali_word`: spell the fractional digits out
    /// one by one.
    ///
    /// ```python
    /// word = ""
    /// for i in str(number):
    ///     word += " " + AKOK[int(i)]
    /// return word
    /// ```
    ///
    /// Every digit contributes a leading space, so the result always starts
    /// with one — that is what separates it from the "দশমিক" before it.
    /// Python read `str(int(digits))`, dropping the leading zeros (bug 11),
    /// and a zero digit hit `AKOK[0] == ""`, leaving a bare space (bug 12).
    /// The port reads the digit run itself, minus insignificant trailing
    /// zeros, and says "শূন্য" for every zero in it (gladiaio/num2words2#205).
    /// Callers only reach here with `decimal_part > 0`, so the run holds a
    /// non-zero digit.
    fn dosomik_to_bengali_word(&self, digits: &str) -> String {
        let mut word = String::new();
        for ch in digits.trim_end_matches('0').chars() {
            let d = ch.to_digit(10).expect("parse_number validated the digit run");
            word.push(' ');
            word.push_str(if d == 0 { ZERO_WORD } else { AKOK[d as usize] });
        }
        word
    }

    /// The shared body of `to_cardinal`, given the already-known sign.
    ///
    /// Mirrors Python's ordering exactly: round first (`str_to_number`), then
    /// range-check the *rounded* value, then convert, then prepend the negword.
    fn cardinal_inner(&self, value: &BigInt, is_negative: bool) -> Result<String> {
        // str_to_number: abs(Decimal(str(number))) — rounds to 28 sig digits.
        let number = round_to_28_significant(&value.abs());
        // parse_number: for integral input `number - int(number)` is 0, so
        // `dosomik` is the empty list and `decimal_part` is always 0. The
        // `decimal_part > 0` branch is therefore dead for integer input.
        self.check_max(&number)?;

        let words = self.number_to_bengali_word(&number);
        // words is already stripped; Python's outer .strip() is a no-op.
        if is_negative {
            Ok(format!("{} {}", NEGWORD, words))
        } else {
            Ok(words)
        }
    }
}

impl Default for LangBn {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangBn {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "BDT"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn negword(&self) -> &str {
        // Python sets self.negword = "ঋণাত্মক" in __init__ and interpolates it
        // as `self.negword + " " + result`, i.e. with no trailing space of its
        // own (unlike base.py's "(-) ").
        NEGWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python decides the sign from the *original* argument, before
        // str_to_number takes the absolute value.
        self.cardinal_inner(value, value.is_negative())
    }

    /// The ordinal word: প্রথম … দশম for 1..=10, then the cardinal + "তম"
    /// (এগারোতম, একুশতম, একশততম). Python's `to_ordinal` was a bare alias
    /// for `to_cardinal` and its `to_ordinal_num` held these words
    /// (gladiaio/num2words2#250).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        self.check_max(value)?;
        if *value >= BigInt::one() && *value <= BigInt::from(10u8) {
            let idx = usize::try_from(value).expect("1..=10 here");
            return Ok(RANKING[idx].to_string());
        }
        // Always "তম": Python glued a bare "ম" onto a cardinal ending in ত,
        // so 100 read "একশতম" for একশততম (শত + তম, as in শততম).
        Ok(format!("{}তম", self.cardinal_inner(value, false)?))
    }

    /// Bengali digits + the written ordinal suffix: ১ম, ২য়, ৩য়, ৪র্থ, ৫ম,
    /// ৬ষ্ঠ, ৭ম … ১০ম, then ১১তম, ২১তম, … (#250; Python returned the words).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        self.check_max(value)?;
        let digits: String = value
            .to_string()
            .chars()
            .map(|c| BN_DIGITS[c.to_digit(10).expect("decimal digit") as usize])
            .collect();
        let suffix = match value.to_u8() {
            Some(2) | Some(3) => "য়",
            Some(4) => "র্থ",
            Some(6) => "ষ্ঠ",
            Some(1..=10) => "ম",
            _ => "তম",
        };
        Ok(format!("{}{}", digits, suffix))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Same raw-value guard, then `to_cardinal(int(abs(number)))`. Python
        // dropped the sign; a negative year now carries the era word
        // খ্রিস্টপূর্ব ("before Christ") in front, as en adds "BC" (#250).
        self.check_max(value)?;
        let words = self.cardinal_inner(&value.abs(), false)?;
        if value.is_negative() {
            return Ok(format!("{}{}{}", BCE, words, SHAL));
        }
        Ok(format!("{}{}", words, SHAL))
    }

    // ---- float / Decimal entry routing --------------------------------

    /// `to_ordinal(float/Decimal)` / `to_ordinal_num(float/Decimal)`: a
    /// whole, non-negative value reads like the integer; anything else raises
    /// Base's ordinal `TypeError`. (Python's `to_ordinal_num` truncated, and
    /// raised a "list indices" TypeError for 1.0..=10.0.)
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.check_max_float(value)?;
        let i = verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        self.check_max_float(value)?;
        let i = verify_ordinal_float(value)?;
        self.to_ordinal_num(&i)
    }

    /// `to_year(float/Decimal)`: the raw-value guard, then
    /// `to_cardinal(int(abs(number))) + " সাল"` — truncation drops both the
    /// fraction and the sign: `-21.0` -> "একুশ সাল", `1.5` -> "এক সাল".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.check_max_float(value)?;
        let words = self.cardinal_inner(&fv_trunc_abs(value)?, false)?;
        if strictly_negative(value) {
            return Ok(format!("{}{}{}", BCE, words, SHAL));
        }
        Ok(format!("{}{}", words, SHAL))
    }

    /// `converter.str_to_number` — BN's own static method is
    /// `abs(Decimal(str(number)))`, so in Python the sign of a string argument
    /// was swallowed before any mode saw it ("-17" -> "সতের", while -17 and
    /// `Decimal("-17")` read "ঋণাত্মক সতের"). Fixed (gladiaio/num2words2#156):
    /// the parsed value keeps its sign and each mode treats it exactly like
    /// the matching Decimal (the modes that drop the sign for every input
    /// type, bugs 3 and 7, still do).
    ///
    /// * NaN parses, but the pinned path then dies in `parse_number`'s
    ///   `int(number)`, which for a Decimal NaN raises
    ///   `decimal.InvalidOperation` — intercepted here because the binding's
    ///   shared NaN sentinel maps to ValueError.
    /// * Infinity keeps the shared sentinel (sign folded by the abs): the
    ///   same `int(number)` raises `OverflowError: cannot convert Infinity to
    ///   integer`, which is exactly what the binding reports for `Inf`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Dec(d) => Ok(ParsedNumber::Dec(d)),
            ParsedNumber::Inf { .. } => Ok(ParsedNumber::Inf { negative: false }),
            ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".to_string(),
            }),
            other => Ok(other),
        }
    }

    // ---- fractions ------------------------------------------------------

    // `Num2Word_BN` had no `to_fraction` (AttributeError). BN has no fraction
    // rules, so the trait default raises NotImplementedError (#217).

    /// `Num2Word_BN.to_cardinal` for float / `Decimal` input.
    ///
    /// BN defines no `to_cardinal_float`; the same `to_cardinal` handles every
    /// numeric type, so this is that one method read with a non-integral
    /// argument. `base.float2tuple` is never reached — see the module docs.
    ///
    /// `precision_override` is ignored, and that is the faithful behaviour:
    /// `num2words()` only applies `precision=` `if hasattr(converter,
    /// "precision")`, and `Num2Word_BN` has no such attribute, so the kwarg is
    /// popped and dropped on the floor. Verified against the interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Python reads the sign off the *original* argument, before
        // `str_to_number`'s abs() erases it:
        //     if isinstance(number, (int, float, Decimal)) and number < 0:
        // `-0.0 < 0` is False in Python, and so is `-0.0 < 0.0` here.
        let is_negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => *value < BigDecimal::zero(),
        };

        // number = self.str_to_number(number) = abs(Decimal(str(number))).
        //
        // The two arms are not interchangeable (issue #603): the Decimal arm is
        // exact at any width, while the float arm can only ever carry what
        // repr(float) round-trips. `Decimal(str(d))` reproduces `d` exactly for
        // a Decimal, since to-sci-string and parsing are inverse on
        // (sign, coefficient, exponent).
        let number = match value {
            FloatValue::Float { value, .. } => float_str_to_number(*value)?,
            FloatValue::Decimal { value, .. } => value.clone(),
        };
        // The abs() is Decimal.__abs__, which dispatches to __pos__/__neg__ and
        // so rounds to the context's 28 significant digits (module bug 1). That
        // is live here, not theoretical: Decimal("0." + "9"*30) rounds up to
        // exactly 1, turning "শূন্য দশমিক নয় নয় ..." into a bare "এক".
        let number = fix_prec28_decimal(&number.abs());

        // `number, decimal_part = self.parse_number(number)` — Python rebinds
        // `number` to the integer part here.
        let (number, decimal_part, digits) = parse_number(&number)?;
        self.check_max(&number)?;

        // `if decimal_part > 0:` — note `> 0`, not "is there a fraction". A
        // fraction of all zeros (Decimal("1.00") → decimal_part 0) takes the
        // else branch and prints no দশমিক at all.
        let dosomik_word = if decimal_part > BigInt::zero() {
            Some(format!(
                "{}{}",
                DOSHOMIK,
                self.dosomik_to_bengali_word(&digits)
            ))
        } else {
            None
        };

        let words = self.number_to_bengali_word(&number);

        // Python: `(words + dosomik_word).strip()` / `words.strip()`. The strip
        // is load-bearing on the dosomik branch — a trailing zero digit leaves a
        // trailing space (Decimal("1.10") → " এক "). Only ASCII spaces and
        // Bengali letters are ever produced, so trim() is equivalent to strip().
        let result = match dosomik_word {
            Some(d) => format!("{}{}", words, d).trim().to_string(),
            None => words.trim().to_string(),
        };

        // `result = self.negword + " " + result` — negword carries no trailing
        // space of its own, unlike base.py's "(-) ".
        if is_negative {
            Ok(format!("{} {}", NEGWORD, result))
        } else {
            Ok(result)
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_BN` has no base class, so there is nothing to inherit and
    // nothing of `base.py`'s currency machinery applies. It defines exactly one
    // currency method, `to_currency`, and builds the whole string itself.
    //
    // Deliberately NOT overridden, because Python has no counterpart to
    // override them with: `currency_forms`, `currency_adjective`,
    // `currency_precision`, `pluralize`, `money_verbose`, `cents_verbose`,
    // `cents_terse`, `cardinal_from_decimal`. `to_currency` below never calls
    // any of them (module bug 6), and `to_cheque` raises before it would, so
    // their trait defaults are unreachable rather than merely unused.

    /// Only reachable through defaults that BN never triggers, but it is this
    /// class's name and the hook exists for exactly this.
    fn lang_name(&self) -> &str {
        "Num2Word_BN"
    }

    /// Port of `Num2Word_BN.to_currency`.
    ///
    /// Everything except `val` is discarded — see module bug 6. `currency` is
    /// never looked up, so no code can raise NotImplementedError and JPY/KWD
    /// precision never applies; `cents`, `separator` and `adjective` are read
    /// by the signature and then ignored outright.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        _currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Guard 1: `if abs(val) >= MAX_NUMBER: raise NumberTooLargeError(...)`.
        // Runs on the raw argument, before Decimal() is involved at all.
        let too_large = match val {
            CurrencyValue::Int(i) => &i.abs() >= max_number_ref(),
            CurrencyValue::Decimal { value, .. } => &value.abs() >= max_number_decimal(),
        };
        if too_large {
            return Err(number_too_large_currency());
        }

        // Python then evaluated `(Decimal(str(val)) * 100) % 1` for a flag
        // that selects between two identical branches (see the module docs);
        // only its 1e26 InvalidOperation was observable, and that is gone
        // (bug 10, #215).

        // `if isinstance(val, int):` — pure ints get no paisa segment at all.
        // Note this is the *type* test, not a whole-number test: 1.0 is a float
        // and does render paisa. `has_decimal` is deliberately not consulted —
        // BN never looks at `str(val)` for a dot, so unlike base.py's
        // to_currency a `Decimal("5")` takes this same float path and prints
        // "শূন্য পয়সা".
        if let CurrencyValue::Int(i) = val {
            // number = str_to_number(val) = abs(Decimal(str(val))) — the abs
            // silently drops the sign (bug 7), and rounds to 28 significant
            // digits (bug 1).
            let number = round_to_28_significant(&i.abs());
            // Provably dead: guard 2 already bounded `number` below 10**26.
            // Ported anyway — it is what the source does.
            self.check_max(&number)?;
            // Python's .strip() is a no-op; number_to_bengali_word already
            // strips and never returns empty.
            return Ok(format!("{} {}", self.number_to_bengali_word(&number), TAKA));
        }

        // The float/Decimal path.
        let value = match val {
            CurrencyValue::Decimal { value, .. } => value,
            CurrencyValue::Int(_) => unreachable!("the int arm returned above"),
        };
        // number = str_to_number(val): abs() first, then Decimal's prec-28 _fix.
        let number = fix_prec28_decimal(&value.abs());
        // `number, decimal_part = self.parse_paisa(number)` — note Python
        // rebinds `number` to the *integer* part here.
        let (number, decimal_part) = parse_paisa(&number)?;
        // Dead for the same reason as above.
        self.check_max(&number)?;

        // `if decimal_part == 0 / elif decimal_part > 0` — parse_paisa clamps
        // to 0..=99, so those two arms are exhaustive and the implicit
        // "no dosomik_word" fallthrough (which would drop the paisa segment)
        // cannot be reached.
        let dosomik_word = if decimal_part == 0 {
            format!(" {} {}", ZERO_WORD, PAISA)
        } else {
            // Both sides of the `if has_fractional_cents` branch collapse to
            // this — see the module docs for the exhaustive check over 1..=99.
            format!(
                " {} {}",
                self.number_to_bengali_word(&BigInt::from(decimal_part)),
                PAISA
            )
        };

        let words = format!("{} {}", self.number_to_bengali_word(&number), TAKA);
        // `if dosomik_word:` is always true — every arm above is non-empty.
        Ok(format!("{}{}", words, dosomik_word))
    }

    // `Num2Word_BN` had no `to_cheque` (AttributeError).
    // No cheque rules, so NotImplementedError ("lang='bn' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }

    // cards/maxval/merge: Num2Word_BN has no base class and never defines
    // them. The trait defaults stand and are unreachable, since to_cardinal is
    // overridden above and never calls splitnum/clean.
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;

    /// A float call shaped like the binding's: the raw f64 plus the
    /// repr-derived precision. BN ignores `precision`, so it is computed the
    /// cheap way rather than mirrored from Python.
    fn f(v: f64) -> String {
        let s = format!("{}", v);
        let precision = match s.split_once('.') {
            Some((_, frac)) => frac.len() as u32,
            None => 0,
        };
        LangBn::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn f_err(v: f64) -> N2WError {
        LangBn::new()
            .to_cardinal_float(
                &FloatValue::Float {
                    value: v,
                    precision: 0,
                },
                None,
            )
            .unwrap_err()
    }

    /// A `Decimal` call: the exact arm, as `cardinal_dec` corpus rows take.
    fn d(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.max(0) as u32;
        LangBn::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `"lang": "bn", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(f(0.0), "শূন্য");
        assert_eq!(f(0.5), "শূন্য দশমিক পাঁচ");
        assert_eq!(f(1.0), "এক");
        assert_eq!(f(1.5), "এক দশমিক পাঁচ");
        assert_eq!(f(2.25), "দুই দশমিক দুই পাঁচ");
        assert_eq!(f(3.14), "তিন দশমিক এক চার");
        assert_eq!(f(0.01), "শূন্য দশমিক শূন্য এক");
        assert_eq!(f(0.1), "শূন্য দশমিক এক");
        assert_eq!(f(0.99), "শূন্য দশমিক নয় নয়");
        assert_eq!(f(1.01), "এক দশমিক শূন্য এক");
        assert_eq!(f(12.34), "বারো দশমিক তিন চার");
        assert_eq!(f(99.99), "নিরানব্বই দশমিক নয় নয়");
        assert_eq!(f(100.5), "একশত দশমিক পাঁচ");
        assert_eq!(f(1234.56), "এক হাজার দুইশত চৌত্রিশ দশমিক পাঁচ ছয়");
        assert_eq!(f(-0.5), "ঋণাত্মক শূন্য দশমিক পাঁচ");
        assert_eq!(f(-1.5), "ঋণাত্মক এক দশমিক পাঁচ");
        assert_eq!(f(-12.34), "ঋণাত্মক বারো দশমিক তিন চার");
        assert_eq!(f(1.005), "এক দশমিক শূন্য শূন্য পাঁচ");
        assert_eq!(f(2.675), "দুই দশমিক ছয় সাত পাঁচ");
    }

    /// Every `"lang": "bn", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_cardinal_dec() {
        assert_eq!(d("0.01"), "শূন্য দশমিক শূন্য এক");
        assert_eq!(d("1.10"), "এক দশমিক এক");
        assert_eq!(d("12.345"), "বারো দশমিক তিন চার পাঁচ");
        assert_eq!(
            d("98746251323029.99"),
            "আটানব্বই লাখ চুয়াত্তর হাজার ছয়শত পঁচিশ কোটি তেরো লাখ তেইশ হাজার উনত্রিশ দশমিক নয় নয়"
        );
        assert_eq!(d("0.001"), "শূন্য দশমিক শূন্য শূন্য এক");
    }

    /// Module bug 11 (fixed, #205): Python's `int()` on the digit run dropped
    /// the fraction's leading zeros, so 0.05 read like 0.5.
    #[test]
    fn leading_zeros_are_kept() {
        assert_ne!(f(0.01), f(0.1));
        assert_ne!(f(0.05), f(0.5));
        assert_eq!(f(0.05), "শূন্য দশমিক শূন্য পাঁচ");
        assert_eq!(f(1.005), "এক দশমিক শূন্য শূন্য পাঁচ");
        assert_eq!(f(3.001), "তিন দশমিক শূন্য শূন্য এক");
    }

    /// Module bug 12 (fixed, #205): `AKOK[0] == ""` left a bare double space
    /// for an interior zero digit; it now reads "শূন্য".
    #[test]
    fn interior_zero_reads_zero() {
        assert_eq!(f(1.102), "এক দশমিক এক শূন্য দুই");
        assert_eq!(f(1.507), "এক দশমিক পাঁচ শূন্য সাত");
        // A *trailing* zero digit strips away instead — the corpus "1.10" row.
        assert_eq!(d("1.10"), "এক দশমিক এক");
        assert_eq!(d("0.10"), "শূন্য দশমিক এক");
        // 0.1 + 0.2 == 0.30000000000000004 -> fifteen interior zeros.
        assert_eq!(f(0.1 + 0.2), format!("শূন্য দশমিক তিন{} চার", " শূন্য".repeat(15)));
    }

    /// The float path is decimal-string based, so the f64 artefacts that
    /// `base.float2tuple` would surface never appear: 2.675 keeps its 675.
    #[test]
    fn no_float2tuple_artefacts() {
        assert_eq!(f(2.675), "দুই দশমিক ছয় সাত পাঁচ");
        assert_eq!(f(12.34), "বারো দশমিক তিন চার");
    }

    /// Module bug 13: str(Decimal) goes scientific below 1e-7, and then there
    /// is no "." to split on — or worse, a garbage tail for int().
    #[test]
    fn sci_notation_cliff() {
        assert_eq!(f(1e-5), "শূন্য দশমিক শূন্য শূন্য শূন্য শূন্য এক");
        assert_eq!(f(1e-7), "শূন্য");
        assert_eq!(f(5e-324), "শূন্য");
        match f_err(1.5e-7) {
            N2WError::Value(m) => {
                assert_eq!(m, "invalid literal for int() with base 10: '5E-7'")
            }
            e => panic!("expected ValueError, got {:?}", e),
        }
    }

    /// A whole-valued fraction takes the `decimal_part > 0` else branch: no
    /// দশমিক at all, not a "point zero".
    #[test]
    fn zero_fraction_prints_no_point() {
        assert_eq!(f(1.0), "এক");
        assert_eq!(f(0.0), "শূন্য");
        assert_eq!(f(-0.0), "শূন্য");
        assert_eq!(d("1.00"), "এক");
        assert_eq!(d("1E+2"), "একশত");
    }

    /// Module bug 1's decimal arm: abs() rounds to 28 significant digits, so
    /// thirty nines round up to a bare "one".
    #[test]
    fn prec28_rounding_is_live() {
        assert_eq!(d(&format!("0.{}", "9".repeat(30))), "এক");
        assert_eq!(d(&format!("0.{}", "9".repeat(2))), "শূন্য দশমিক নয় নয়");
    }

    /// Large values, and the `_is_smaller_than_max_number` ceiling reached
    /// through the float arm.
    #[test]
    fn large_values() {
        // 1e16: repr is scientific ('1e+16'), Rust prints it positionally.
        // Both land on the same value with no fraction.
        assert_eq!(f(1e16), "একশত কোটি কোটি");
        assert_eq!(f(1e300), LangBn::new().to_cardinal(&(BigInt::from(10u8).pow(300))).unwrap());
        match f_err(1e307) {
            N2WError::Custom { module, class, .. } => {
                assert_eq!(module, "num2words2.lang_BN");
                assert_eq!(class, "NumberTooLargeError");
            }
            e => panic!("expected NumberTooLargeError, got {:?}", e),
        }
    }

    /// Module bug 14: non-finite floats crash in Python's `int()`.
    #[test]
    fn non_finite() {
        match f_err(f64::INFINITY) {
            N2WError::Overflow(m) => assert_eq!(m, "cannot convert Infinity to integer"),
            e => panic!("expected OverflowError, got {:?}", e),
        }
        match f_err(f64::NEG_INFINITY) {
            N2WError::Overflow(m) => assert_eq!(m, "cannot convert Infinity to integer"),
            e => panic!("expected OverflowError, got {:?}", e),
        }
        match f_err(f64::NAN) {
            N2WError::Value(m) => assert_eq!(m, "cannot convert NaN to integer"),
            e => panic!("expected ValueError, got {:?}", e),
        }
    }

    /// Locks the invariant the currency port leans on: `to_currency`'s
    /// `has_fractional_cents` branch picks `to_cardinal(float(decimal_part))`
    /// over `_number_to_bengali_word(decimal_part)`, and for every paisa value
    /// `parse_paisa` can produce (1..=99) the two are identical. If a future
    /// edit to the float path broke this, `to_currency` would start diverging.
    #[test]
    fn float_cardinal_matches_word_for_every_paisa() {
        let l = LangBn::new();
        for dp in 1..=99u32 {
            assert_eq!(
                f(dp as f64),
                l.number_to_bengali_word(&BigInt::from(dp)),
                "paisa {} diverges",
                dp
            );
        }
    }

    /// `precision=` is a no-op: num2words only applies it when the converter
    /// has a `precision` attribute, and Num2Word_BN has none.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangBn::new();
        let v = FloatValue::Float {
            value: 3.14159,
            precision: 5,
        };
        let expected = "তিন দশমিক এক চার এক পাঁচ নয়";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), expected);
        assert_eq!(l.to_cardinal_float(&v, Some(1)).unwrap(), expected);
        assert_eq!(l.to_cardinal_float(&v, Some(8)).unwrap(), expected);
    }
}
