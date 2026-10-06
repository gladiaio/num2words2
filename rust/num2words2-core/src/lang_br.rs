//! Port of `lang_BR.py` (Breton).
//!
//! Registry check: `__init__.py` maps `"br"` → `lang_BR.Num2Word_BR()`, so this
//! file ports `Num2Word_BR` — the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_BR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the `__init__`
//! guard
//!
//! ```python
//! if any(hasattr(self, f) for f in ["high_numwords", "mid_numwords", "low_numwords"]):
//! ```
//!
//! never fires: Python builds **no** `self.cards` and sets **no** `self.MAXVAL`
//! (verified: `hasattr(b, "cards") is False`, `hasattr(b, "MAXVAL") is False`).
//! `to_cardinal` is overridden outright and drives `_int_to_word`, so
//! `cards`/`maxval`/`merge` stay at their trait defaults here and **no overflow
//! check exists** — there is no input this module rejects.
//!
//! The Python inheritance chain is exactly `Num2Word_BR` → `Num2Word_Base` →
//! `object`. Every in-scope method is overridden by BR, so nothing is inherited
//! for the four supported modes:
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden: cardinal + `"-vet"`
//!   * `to_ordinal_num` — overridden: `str(number) + "."`
//!   * `to_year`        — overridden: delegates to `to_cardinal`, ignoring the
//!     `longval` keyword entirely
//!
//! # The float / Decimal path
//!
//! BR does **not** override `to_cardinal_float`; it overrides `to_cardinal`,
//! whose very first line is `n = str(number).strip()`. Every float and every
//! `Decimal` therefore reaches the *same* body as an int and is dispatched on
//! whether that string contains a `"."`. `Num2Word_Base.to_cardinal_float` and
//! `Num2Word_Base.float2tuple` are inherited but **dead**: BR's `to_cardinal`
//! never calls them, so [`crate::floatpath`] is deliberately not used here.
//!
//! That single fact drives everything below. The digits BR speaks are the
//! digits `repr()` prints — not the ones `float2tuple` derives:
//!
//! ```python
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     for digit in right:
//!         ret += self._int_to_word(int(digit)) + " "
//!     return ret.strip()
//! ```
//!
//! ## Further Python quirks reproduced (float / Decimal)
//!
//! 14. **`float2tuple`'s artefact heuristic never runs, and neither does its
//!     rounding.** The `< 0.01` rescue in `base.float2tuple` — the thing that
//!     turns `2.675`'s `674.9999999999998` back into `675` — sits on a code
//!     path BR cannot reach; BR just reads `repr(2.675) == "2.675"` and spells
//!     the characters. There is no `round()` anywhere in this file, so the
//!     banker's-rounding trap does not apply to it either.
//!
//!     The two routes agree on the corpus's headline cases (`1.005` and `2.675`
//!     both come out `"… pemp"`, base by rescuing its artefact and BR by never
//!     creating one), but they are not interchangeable in general: over 30k
//!     random floats they disagree on **8798**. The cheapest witness is a long
//!     fraction, where `post = abs(value - pre) * 10**precision` is evaluated
//!     in f64 and loses the last digit that `repr` kept —
//!     `0.17108284528077355` has `precision == 17`, and that product is
//!     `1.7108284528077356e16`, so `float2tuple` reports `…356` where `repr`
//!     says `…355`. Base ends `"… pemp c'hwec'h"` (6); BR ends `"… pemp pemp"`
//!     (5). `num2words(0.17108284528077355, lang="br")` returns BR's, which is
//!     what makes the string route the observable one and not a detail.
//! 15. **`self.precision` is never read, so `precision=` does nothing.**
//!     `Num2Word_Base.__init__` sets `self.precision = 2` and the dispatcher
//!     happily assigns to it (`hasattr(converter, "precision")` is True), but
//!     BR's `to_cardinal` spells *every* character of `right` regardless.
//!     Verified against the interpreter: `num2words(1.2345, lang="br",
//!     precision=2)` == `num2words(1.2345, lang="br", precision=8)` ==
//!     `"unan point daou tri pevar pemp"` — all four digits, both times. Hence
//!     `precision_override` is accepted and ignored below.
//! 16. **Scientific notation is a `ValueError`, and *which* one depends on the
//!     digit count.** `repr` switches to exponent form at `decpt <= -4` or
//!     `decpt > 16`, and neither `int()` call in the body tolerates it:
//!       * one significant digit (`repr(1e16) == "1e+16"`) has no `"."`, so the
//!         integer branch runs and `int("1e+16")` raises
//!         `invalid literal for int() with base 10: '1e+16'`;
//!       * more than one (`repr(1.5e16) == "1.5e+16"`) *does* have a `"."`, so
//!         the float branch runs, `int("1")` succeeds, the digit loop eats
//!         `"5"`, and then `int("e")` raises
//!         `invalid literal for int() with base 10: 'e'`.
//!     `inf` and `nan` die the same way as the first case. So BR converts no
//!     float at or beyond `1e16`, and none below `1e-4`.
//! 17. **A `Decimal` keeps its trailing zeros, because `str` does.**
//!     `str(Decimal("1.10")) == "1.10"`, so 1.10 is `"unan point unan zero"` —
//!     two fraction words for a value a float would render with one. Likewise
//!     `Decimal("5.00")` is `"pemp point zero zero"` while `Decimal("5")`, whose
//!     `str` has no `"."` at all, takes the *integer* branch and is plain
//!     `"pemp"`. This is why the value cannot be normalised on the way in.
//! 18. **`Decimal`'s exponent threshold is not `float`'s.** `Decimal.__str__`
//!     goes scientific at `_exp > 0` or `adjusted exponent < -6`, and writes the
//!     exponent unpadded with a capital `E`: `str(Decimal("1e-7")) == "1E-7"`
//!     (versus `repr(1e-07) == "1e-07"`), and `str(Decimal("1e-5")) ==
//!     "0.00001"` where the float renders `"1e-05"` and raises. So `Decimal`
//!     spells fractions four orders of magnitude smaller than `float` can.
//! 19. **`98746251323029.99` survives as a `Decimal` and would not as a float**
//!     (issue #603). It renders `"98746251323029 point nav nav"` — the integer
//!     part going through `_int_to_word`'s 10^9 digit fallback (quirk 1).
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are odd-looking but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **`_int_to_word` falls back to digits at 10^9.** The if/elif chain stops
//!    at `number < 1000000000` and the final `else` is
//!    `return str(number)  # Fallback for very large numbers`. So
//!    `to_cardinal(10**9)` == `"1000000000"` — bare digits, not words — and
//!    `to_ordinal(10**9)` == `"1000000000-vet"`. The negative sign is stripped
//!    by `to_cardinal` before `_int_to_word` runs, so
//!    `to_cardinal(-10**9)` == `"minus 1000000000"`. This is why the fallback
//!    must use arbitrary-precision `to_string()`: the corpus reaches 10^21,
//!    well past `u64`. Confirmed by corpus rows for 10^9, 10^12, 10^18, 10^21.
//! 2. **Zero is spelled by an accident of falsiness.** `_int_to_word(0)` reads
//!    `return self.ones[0] if self.ones[0] else "zero"`. `ones[0]` is the empty
//!    string, which is falsy in Python, so the ternary *always* takes the else
//!    branch and the first operand is dead. Result: `"zero"`.
//! 3. **No `verify_ordinal` call.** `Num2Word_Base.verify_ordinal` would raise
//!    `TypeError` for negative ordinals, but BR's `to_ordinal` never calls it.
//!    So `to_ordinal(-1)` == `"minus unan-vet"` rather than raising — the
//!    suffix lands on the end of the whole phrase, not on the number word.
//!    Likewise `to_ordinal(-100)` == `"minus unan kant-vet"`.
//! 4. **Hundreds are never bare.** The `number < 1000` branch always emits
//!    `self.ones[hundreds_val] + " " + self.hundred`, so 100 is `"unan kant"`
//!    ("one hundred"), never `"kant"`. Same for `mil` and `milion` via the
//!    recursive calls: 1000 is `"unan mil"`, 10^6 is `"unan milion"`.
//! 5. **Breton's vigesimal tens are stored whole**, spaces and hyphens
//!    included: `tens[7]` is `"dek ha tri-ugent"` (70) and `tens[9]` is
//!    `"dek ha pevar-ugent"` (90). Concatenation is therefore blind to word
//!    count — 99 becomes `"dek ha pevar-ugent nav"`.
//! 6. **`negword` keeps its trailing space.** `setup` sets `"minus "` (unlike
//!    `Num2Word_Base.to_cardinal`, which does `self.negword.strip()`). BR's
//!    `to_cardinal` prepends it raw and `.strip()`s the finished string, so the
//!    single space survives between sign and number and nowhere else.
//! 7. **`_int_to_word`'s own negative branch is dead code** on every in-scope
//!    path: `to_cardinal` strips the `"-"` at the string level and feeds
//!    `int(n)` a non-negative value. It is only reachable from `to_currency`
//!    (out of scope). Reproduced anyway in [`int_to_word`] for fidelity.
//!
//! # Errors
//!
//! For **integer** input: none. BR raises nothing in the four supported modes:
//! no `MAXVAL` means no `OverflowError`, no `verify_ordinal` means no
//! `TypeError`, and every table index is bounded by an arithmetic guard.
//! Accordingly every integer method here returns `Ok`, and the frozen corpus
//! agrees — all 324 in-scope `br` rows carry `"ok": true`.
//!
//! For **float / Decimal** input the only error is `ValueError`, raised by
//! `int()` on a token it cannot parse (quirk 16), and it is always
//! [`N2WError::Value`]. There is still no `OverflowError` — `1e16` fails
//! because `repr` wrote an `e`, not because BR checked a bound.
//!
//! # The currency surface
//!
//! `Num2Word_BR` defines `CURRENCY_FORMS` (EUR and USD only) and overrides
//! `to_currency` outright. It defines **no** `CURRENCY_ADJECTIVES`, **no**
//! `CURRENCY_PRECISION`, and does not override `pluralize`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse` or `to_cheque` — all of those stay at
//! `Num2Word_Base`, which is exactly what the trait defaults already model. So
//! only [`Lang::lang_name`], [`Lang::currency_forms`] and [`Lang::to_currency`]
//! are overridden here.
//!
//! `to_cheque` is therefore the inherited `Num2Word_Base.to_cheque`, driving
//! `currency::default_to_cheque`. It looks the code up in `CURRENCY_FORMS`
//! directly and raises `NotImplementedError` on a miss, which is why
//! `cheque:EUR`/`cheque:USD` succeed while `cheque:GBP` and friends raise —
//! matching the corpus exactly. Its `divisor` is
//! `CURRENCY_PRECISION.get(currency, 100)`, and BR's `CURRENCY_PRECISION` is
//! the empty inherited dict, so it is **always** 100: `cheque:KWD` would use
//! `/100`, not `/1000`, if KWD were in the table at all.
//!
//! ## Further Python quirks reproduced (currency)
//!
//! 8. **An unknown currency code silently becomes euros.** `to_currency` reads
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
//!    The fallback is the *first inserted* entry — EUR — so `currency="GBP"`,
//!    `"JPY"`, `"KWD"` … all render as euros instead of raising. Verified
//!    against the interpreter: `to_currency(12.34, currency="JPY")` ==
//!    `"dek daou euroioù tregont pevar sentimoù"`. This fallback lives *only*
//!    in `to_currency`; [`Lang::currency_forms`] stays a faithful
//!    `CURRENCY_FORMS[code]` (returning `None` on a miss) so the inherited
//!    `to_cheque` still raises for the same codes.
//! 9. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** The cents
//!    field is hard-coded to two digits via `parts[1][:2].ljust(2, "0")`, so
//!    3-decimal currencies get no special handling and the `divisor`
//!    machinery of `base.to_currency` is bypassed entirely. `0.5` is 50
//!    subunits for every code.
//! 10. **The int/float split is invisible here.** `base.to_currency` branches
//!    on `isinstance(val, int)`; BR does not — it branches on whether
//!    `str(val)` contains a `"."`. The two agree anyway, because a whole
//!    float stringifies with a `"0"` fraction: `1.0` → `parts[1] == "0"` →
//!    `int("00")` → `0`, which is falsy, so `if cents and right:` drops the
//!    segment just as the int path would. Both `1` and `1.0` give
//!    `"unan euro"`. The distinction is still honoured in the value passed in.
//! 11. **`cents=False` drops the cents segment rather than going terse.**
//!    `if cents and right:` is the only guard; there is no `_cents_terse`
//!    branch, so `cents=False` loses the subunits entirely.
//! 12. **A float too large to stringify plainly raises `ValueError`.** Python
//!    takes `str(val)`, so `1e16` and above render in scientific notation and
//!    `int("1e+16")` raises. Confirmed against the interpreter:
//!    `to_currency(1e21)` → `ValueError`. `BigDecimal`'s `Display` preserves
//!    the same `1e+21` form, so `BigInt::from_str` fails on exactly the same
//!    inputs and [`N2WError::Value`] reproduces the type natively.
//! 13. **`_int_to_word`'s digit fallback is reachable from currency.** Unlike
//!    the four integer modes, `to_currency(1234567890.0)` yields
//!    `"1234567890 euroioù"` — bare digits, per quirk 1.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `setup`: `self.negword`. Trailing space is intentional — see quirk 6.
const NEGWORD: &str = "minus ";

/// `setup`: `self.pointword`. Read by the float/Decimal branch of
/// [`cardinal_from_py_str`], which splices it between the two halves.
const POINTWORD: &str = "point";

/// `setup`: `self.ones`. Index 0 is the empty string; see quirk 2.
const ONES: [&str; 10] = [
    "",
    "unan",
    "daou",
    "tri",
    "pevar",
    "pemp",
    "c'hwec'h",
    "seizh",
    "eizh",
    "nav",
];

/// `setup`: `self.tens`, indexed by the tens digit. Breton counts in twenties,
/// so these are multi-word forms stored verbatim; see quirk 5.
const TENS: [&str; 10] = [
    "",
    "dek",
    "ugent",
    "tregont",
    "daou-ugent",
    "hanter-kant",
    "tri-ugent",
    "dek ha tri-ugent",
    "pevar-ugent",
    "dek ha pevar-ugent",
];

/// `Num2Word_BR.to_currency`'s own default `separator=" "`, confirmed against
/// the interpreter: `Num2Word_BR.to_currency.__defaults__` is
/// `("EUR", True, " ", False)`.
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default `separator=","`.
///
/// The dispatcher (`__init__.py`) sends `kwargs.get("separator", ",")` down the
/// Rust fast path — *base's* default, not BR's — so a caller who says nothing
/// about the separator is indistinguishable from one who explicitly asked for
/// `","`. `to_currency` treats this value as "caller said nothing" and
/// substitutes [`DEFAULT_SEPARATOR`]; see the note on [`LangBr::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// `setup`: `self.hundred`.
const HUNDRED: &str = "kant";
/// `setup`: `self.thousand`.
const THOUSAND: &str = "mil";
/// `setup`: `self.million`.
const MILLION: &str = "milion";

/// Index a numword table with a `BigInt` the caller has already bounded to
/// `0..=9` by an arithmetic guard. The `expect` is unreachable: every call site
/// sits behind a `< 10`, `/ 10` (of a value `< 100`), or `/ 100` (of a value
/// `< 1000`) test, so Python's own list indexing could not go out of range
/// either.
fn digit(n: &BigInt) -> usize {
    n.to_usize().expect("guarded to 0..=9 by the caller")
}

/// Port of `Num2Word_BR._int_to_word`.
///
/// Mirrors the Python if/elif chain arm for arm, including the negative branch
/// that `to_cardinal` can never reach (quirk 7) and the digits fallback above
/// 10^9 (quirk 1). Stays on `BigInt` throughout: the fallback arm must render
/// values the corpus takes to 10^21, and narrowing early would lose them.
pub fn int_to_word(number: &BigInt) -> String {
    // Python: `return self.ones[0] if self.ones[0] else "zero"` — ones[0] is
    // "", which is falsy, so the else branch always wins. See quirk 2.
    if number.is_zero() {
        return "zero".to_string();
    }

    // Dead on every in-scope path (quirk 7), kept for fidelity.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    // From here `number` is strictly positive, so BigInt's truncating `/` and
    // `%` agree with Python's floor-division `//` and `%`.
    if *number < BigInt::from(10u32) {
        return ONES[digit(number)].to_string();
    }

    if *number < BigInt::from(100u32) {
        let tens_val = number / 10u32;
        let ones_val = number % 10u32;
        if ones_val.is_zero() {
            return TENS[digit(&tens_val)].to_string();
        }
        return format!("{} {}", TENS[digit(&tens_val)], ONES[digit(&ones_val)]);
    }

    if *number < BigInt::from(1_000u32) {
        // Python reads self.ones[hundreds_val] directly here rather than
        // recursing, so 100 is always "unan kant" — never bare "kant".
        let hundreds_val = number / 100u32;
        let remainder = number % 100u32;
        let mut result = format!("{} {}", ONES[digit(&hundreds_val)], HUNDRED);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if *number < BigInt::from(1_000_000u32) {
        let thousands_val = number / 1_000u32;
        let remainder = number % 1_000u32;
        let mut result = format!("{} {}", int_to_word(&thousands_val), THOUSAND);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if *number < BigInt::from(1_000_000_000u32) {
        let millions_val = number / 1_000_000u32;
        let remainder = number % 1_000_000u32;
        let mut result = format!("{} {}", int_to_word(&millions_val), MILLION);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    // Python: `return str(number)  # Fallback for very large numbers`.
    // Quirk 1 — bare digits, no words, no error.
    number.to_string()
}

/// Python's `int(<str>)`, for the tokens `str(float)`/`str(Decimal)` can hand
/// it.
///
/// The message is reproduced verbatim because it is the *only* thing that
/// distinguishes BR's two failure modes from each other (quirk 16), and because
/// `BigInt::from_str`'s own message ("invalid digit found in string") would leak
/// Rust into a `ValueError` a caller may well be printing.
///
/// `BigInt::from_str` is narrower than `int()` in ways this caller cannot
/// reach: it rejects the surrounding whitespace, `_` separators and non-ASCII
/// decimal digits that `int()` accepts. Neither `repr(float)` nor
/// `Decimal.__str__` ever emits any of those, so the two agree on every string
/// that actually arrives here.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `repr(float)` — i.e. `PyOS_double_to_string(v, 'r', 0,
/// Py_DTSF_ADD_DOT_0)`, which `float.__str__` is an alias of.
///
/// BR spells the characters of this string, so it is the whole specification of
/// the float path and has to be exact, `".0"` suffix and `"e+16"` threshold
/// included. It cannot be replaced by `format!("{}", v)`: Rust's `Display`
/// prints `1.0` as `"1"`, which has no `"."` and would silently route 1.0
/// through the *integer* branch to `"unan"` instead of `"unan point zero"`.
///
/// # Why the digits are taken in two steps
///
/// `_Py_dg_dtoa(mode=0)` is not merely "some string that round-trips": it is
/// the *n*-significant-digit decimal **nearest** to `v`, ties to **even**, with
/// *n* the shortest length that round-trips. The two halves of that come from
/// two different Rust formatters, and using only the first is a real bug:
///
/// * `{:e}` is shortest-round-trip, so it settles *n* — and on `n` the two
///   agree, both being minimal-length.
/// * `{:e}` does **not** settle the digits. When `v` sits exactly halfway
///   between two *n*-digit decimals both algorithms accept, Gay rounds the last
///   digit to even and Rust's shortest rounds it up. `f64::from_bits(
///   0x4313781b63796605)` is exactly `1370020896463233.25`: Python prints
///   `"1370020896463233.2"`, `{:e}` yields `…3.3`, and BR would say `"… tri"`
///   for a value Python calls `"… daou"`. A 200k-value fuzz against the live
///   interpreter hit this 1462 times.
/// * `{:.*e}` *is* exact and correctly rounded, and `flt2dec`'s `format_exact`
///   documents the same tie rule Gay uses ("if the following digits are exactly
///   half, round to even"). So the digits come from there, at the width `{:e}`
///   established.
///
/// The rest is CPython's `format_float_short` layout logic transcribed.
fn py_float_str(v: f64) -> String {
    // CPython prints these from a fixed table, and `repr` drops the sign of a
    // nan (`repr(-float("nan")) == "nan"`, sign bit or not) while keeping it on
    // an inf.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative`, not `v < 0.0`: `_Py_dg_dtoa` reports the sign *bit*,
    // so `repr(-0.0) == "-0.0"` and BR answers "minus zero point zero". A
    // `< 0.0` test says false for -0.0 and would drop the minus.
    let sign = if v.is_sign_negative() { "-" } else { "" };

    // Step 1: the shortest round-trip form, "d[.ddd]e<exp>" — e.g. 0.5 ->
    // "5e-1", 1.005 -> "1.005e0", 0.0 -> "0e0". Only its *digit count* is
    // trusted; see the tie discussion above.
    let shortest = format!("{:e}", v.abs());
    let ndigits = shortest
        .split_once('e')
        .expect("{:e} always emits an exponent marker")
        .0
        .chars()
        .filter(char::is_ascii_digit)
        .count();

    // Step 2: re-render at exactly that width. `{:.*e}` is the exact,
    // correctly-rounded, ties-to-even formatter, which is what dtoa mode 0
    // actually computes. The exponent is re-read from *this* call rather than
    // carried over, so a hypothetical rounding carry stays self-consistent.
    // saturating_sub only to keep an impossible ndigits == 0 from underflowing
    // a usize; `{:e}` always emits at least one mantissa digit.
    let exact = format!("{:.*e}", ndigits.saturating_sub(1), v.abs());
    let (mant, exp) = exact
        .split_once('e')
        .expect("{:e} always emits an exponent marker");
    let exp: i32 = exp.parse().expect("{:e} always emits a decimal exponent");
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let digits_len = digits.chars().count() as i32;
    // CPython's decpt: the decimal point's position within `digits`.
    let decpt = exp + 1;

    // format_float_short, `case 'r'`:
    //
    //     /* Convert to exponential format at 1e16. ... */
    //     if (decpt <= -4 || decpt > 16)
    //         use_exp = 1;
    if decpt <= -4 || decpt > 16 {
        // exp = decpt - 1; decpt = 1;
        let e = decpt - 1;
        let mut out = String::from(sign);
        let mut it = digits.chars();
        out.push(it.next().expect("dtoa never returns an empty digit string"));
        let rest: String = it.collect();
        // Py_DTSF_ADD_DOT_0 is not applied in exponent form, so a lone digit
        // stays lone: repr(1e16) is "1e+16", never "1.0e+16".
        if !rest.is_empty() {
            out.push('.');
            out.push_str(&rest);
        }
        // CPython: `sprintf(p, "%+.02d", exp)` — always signed, at least two
        // digits, never truncated. 16 -> "+16", -5 -> "-05", -324 -> "-324".
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (e as i64).abs()));
        out
    } else if decpt <= 0 {
        // 0.0001 <= |v| < 1: "0." then -decpt padding zeros, then the digits.
        format!(
            "{}0.{}{}",
            sign,
            "0".repeat((-decpt) as usize),
            digits
        )
    } else if decpt >= digits_len {
        // Integral: pad right with zeros, then Py_DTSF_ADD_DOT_0 appends ".0".
        // This is the arm that makes repr(1.0) == "1.0" and repr(0.0) == "0.0",
        // and therefore the arm that puts every whole float on the *float*
        // branch of to_cardinal.
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - digits_len) as usize)
        )
    } else {
        // 0 < decpt < digits_len: split the digit string in place.
        let head: String = digits.chars().take(decpt as usize).collect();
        let tail: String = digits.chars().skip(decpt as usize).collect();
        format!("{}{}.{}", sign, head, tail)
    }
}

/// Python's `Decimal.__str__` (the spec's *to-scientific-string*), transcribed
/// from `_pydecimal.Decimal.__str__` with `eng=False` and the default
/// `context.capitals == 1`.
///
/// Not interchangeable with [`py_float_str`]: the thresholds differ (quirk 18),
/// the exponent is unpadded and capital-`E`, and — the reason this exists at all
/// — the coefficient is whatever the caller wrote, so `Decimal("1.10")`
/// stringifies with its trailing zero intact (quirk 17).
///
/// `BigDecimal`'s own `Display` is close but not this: it is free to normalise
/// and to choose its own exponent threshold, and either divergence changes how
/// many words BR speaks.
fn py_decimal_str(d: &BigDecimal) -> String {
    // BigDecimal is coefficient * 10^-scale; Decimal is _int * 10^_exp.
    let (coefficient, scale) = d.as_bigint_and_exponent();
    let exp = -scale; // Decimal._exp
    let sign = if coefficient.is_negative() { "-" } else { "" };
    // Decimal._int — the coefficient's digits, unsigned and unnormalised.
    let int_digits = coefficient.abs().to_string();
    let len = int_digits.chars().count() as i64;

    // leftdigits = self._exp + len(self._int)
    let leftdigits = exp + len;

    // if self._exp <= 0 and leftdigits > -6: dotplace = leftdigits
    // elif not eng:                          dotplace = 1
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let head: String = int_digits.chars().take(dotplace as usize).collect();
        let tail: String = int_digits.chars().skip(dotplace as usize).collect();
        (head, format!(".{}", tail))
    };

    // exp = ['e', 'E'][context.capitals] + "%+d" % (leftdigits - dotplace)
    //
    // "%+d", not the float path's "%+.02d": no zero padding, which is why
    // str(Decimal("1e-7")) is "1E-7" and repr(1e-07) is "1e-07".
    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        let e = leftdigits - dotplace;
        format!("E{}{}", if e < 0 { "-" } else { "+" }, e.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Port of `Num2Word_BR.to_cardinal`'s body, driven by `str(number)` exactly as
/// Python drives it.
///
/// Python's method takes the *number* and stringifies it on line one; the two
/// callers here have already done that, because reproducing `str` is where all
/// the difficulty lives ([`py_float_str`], [`py_decimal_str`]) and it differs
/// per type. Everything after that line is type-blind, which is precisely why
/// [`Lang::to_cardinal`] — the integer specialisation of this same body — can
/// skip the round-trip through text: an integer's `str` has no `"."`, so it
/// always takes the `else` branch and `int(str(v))` is just `v`.
fn cardinal_from_py_str(number: &str) -> Result<String> {
    // n = str(number).strip()
    let n = number.trim();

    // if n.startswith("-"): n = n[1:]; ret = self.negword
    // else:                           ret = ""
    let (ret, n) = match n.strip_prefix('-') {
        Some(rest) => (NEGWORD, rest),
        None => ("", n),
    };

    // if "." in n: left, right = n.split(".", 1)
    match n.split_once('.') {
        Some((left, right)) => {
            // ret += self._int_to_word(int(left)) + " " + self.pointword + " "
            let mut out = String::from(ret);
            out.push_str(&int_to_word(&py_int(left)?));
            out.push(' ');
            out.push_str(POINTWORD);
            out.push(' ');

            // for digit in right: ret += self._int_to_word(int(digit)) + " "
            //
            // Iterating a str yields one character at a time, so this is
            // int() on a single character — and any non-digit character (the
            // "e" of an exponent form, quirk 16) raises ValueError right here,
            // after the earlier digits have already been appended and thrown
            // away with the exception.
            for digit in right.chars() {
                let d = py_int(digit.encode_utf8(&mut [0u8; 4]))?;
                out.push_str(&int_to_word(&d));
                out.push(' ');
            }

            // return ret.strip() — this is what removes the loop's last space.
            Ok(out.trim().to_string())
        }
        // return (ret + self._int_to_word(int(n))).strip()
        None => Ok(format!("{}{}", ret, int_to_word(&py_int(n)?))
            .trim()
            .to_string()),
    }
}

pub struct LangBr {
    /// `Num2Word_BR.CURRENCY_FORMS`. Built once in [`LangBr::new`] and only
    /// read thereafter — the generated registry parks each language in a
    /// `OnceLock` and calls `new` through `get_or_init`, so this table is
    /// constructed a single time per process rather than per conversion.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the fallback `.get` uses for
    /// an unknown code (quirk 8).
    ///
    /// Python evaluates that expression eagerly on every call and it resolves
    /// to the *first inserted* entry, which under CPython's insertion-ordered
    /// dicts is `EUR`. A `HashMap` has no first element, so the choice is
    /// pinned here rather than left to iteration order.
    fallback_forms: CurrencyForms,
}

impl LangBr {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::new();
        // Arity is load-bearing: `to_currency` indexes `cr1[0]`/`cr1[1]`, so
        // both forms of both tuples must survive verbatim.
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euroioù"], &["sentim", "sentimoù"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        let fallback_forms = currency_forms
            .get("EUR")
            .expect("CURRENCY_FORMS[\"EUR\"] is inserted directly above")
            .clone();
        LangBr {
            currency_forms,
            fallback_forms,
        }
    }
}

impl Default for LangBr {
    fn default() -> Self {
        LangBr::new()
    }
}

impl Lang for LangBr {

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

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + "-vet"` with no verify_ordinal guard, so a
    /// float keeps its spelled-out ".0" tail: `to_ordinal(5.0)` ==
    /// "pemp point zero-vet", `to_ordinal(-1.5)` == "minus unan point
    /// pemp-vet". Exponent-form reprs raise the cardinal path's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-vet", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr
    /// verbatim, sign and trailing zeros included: `-0.0` → "-0.0.",
    /// `Decimal("5.00")` → "5.00.", `1e16` → "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, except
    /// that an Infinity parse is surfaced as the ValueError BR's own
    /// `to_cardinal` raises one step later: `str(Decimal("Infinity"))` has no
    /// "." and `int("Infinity")` chokes on the literal. The dispatcher's
    /// default maps `ParsedNumber::Inf` to base's OverflowError, which BR can
    /// never raise. NaN keeps the default routing (ValueError either way).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "EUR"
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

    /// Port of `Num2Word_BR.to_cardinal` for integral input.
    ///
    /// Python works on the *string* form:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
    /// else:
    ///     ret = ""
    /// # no "." in an integer's repr, so the float branch is skipped
    /// return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// Stripping the leading `"-"` and re-parsing is exactly `abs()`, so the
    /// value handed to `_int_to_word` is never negative — hence quirk 7. The
    /// trailing `.strip()` is a no-op for every integer input (`negword`'s
    /// space always sits between two non-empty tokens), but is kept so the
    /// shape matches the original.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, int_to_word(&n)).trim().to_string())
    }

    /// The float/Decimal arm of `Num2Word_BR.to_cardinal` — *not* an override
    /// of `to_cardinal_float`, which BR inherits and never calls.
    ///
    /// `num2words(0.5, lang="br")` runs `Num2Word_BR.to_cardinal(0.5)`, which
    /// stringifies the argument and reads the characters. So the work is all in
    /// reproducing `str`, and the two arms of [`FloatValue`] need two different
    /// `str`s — `repr(float)` and `Decimal.__str__` — which is exactly the split
    /// [`FloatValue`] preserves. Collapsing them here would not merely lose
    /// precision (issue #603); it would lose *digits BR speaks aloud*, since
    /// `Decimal("1.10")` is two fraction words and `float(1.10)` is one.
    ///
    /// `precision_override` is discarded, because BR never reads
    /// `self.precision` (quirk 15). The dispatcher does set the attribute — the
    /// inherited `__init__` defines it — but nothing on this path looks at it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Python: `n = str(number).strip()`, where `number` is whatever the
        // caller passed num2words(). No float() cast, no float2tuple, no
        // rounding — the sole difference between the arms is `str` itself.
        let n = match value {
            FloatValue::Float { value, .. } => py_float_str(*value),
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_py_str(&n)
    }

    /// Port of `Num2Word_BR.to_ordinal`: cardinal + `"-vet"`, with no
    /// `verify_ordinal` guard (quirk 3). The suffix attaches to the end of the
    /// whole phrase, so `to_ordinal(-100)` == `"minus unan kant-vet"` and
    /// `to_ordinal(10**9)` == `"1000000000-vet"`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-vet", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_BR.to_ordinal_num`: `str(number) + "."`. The number is
    /// never spelled, so the sign survives verbatim: `-1` → `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_BR.to_year`: `return self.to_cardinal(val)`. The
    /// `longval=True` parameter is accepted and then ignored, so years get no
    /// special pairing — 1999 is the plain cardinal
    /// `"unan mil nav kant dek ha pevar-ugent nav"`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ------------------------------------------------------
    //
    // Only the three hooks below are overridden. BR defines no
    // CURRENCY_ADJECTIVES and no CURRENCY_PRECISION, and leaves `pluralize`,
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` at
    // `Num2Word_Base` — whose behaviour the trait defaults already reproduce.

    /// `self.__class__.__name__`, for the inherited `to_cheque`'s
    /// `NotImplementedError` message.
    fn lang_name(&self) -> &str {
        "Num2Word_BR"
    }

    /// `CURRENCY_FORMS[code]` — a plain lookup that misses for anything but
    /// EUR/USD.
    ///
    /// Deliberately **not** the `.get(code, <EUR>)` fallback of quirk 8: that
    /// fallback is local to `to_currency`. The inherited `to_cheque` subscripts
    /// `CURRENCY_FORMS[currency]` and catches `KeyError`, so it must still see
    /// a miss here — that is what makes `cheque:GBP` raise
    /// `NotImplementedError` while `currency:GBP` quietly prints euros.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_BR.to_currency`.
    ///
    /// BR ignores `base.to_currency` wholesale — no `parse_currency_parts`, no
    /// `divisor`, no `pluralize`. It slices the decimal *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// # The `separator` argument is not what Python's default would be
    ///
    /// BR's own signature defaults to `separator=" "`, but the trait can't
    /// carry a per-language default argument: the dispatcher resolves it before
    /// the call and uses `Num2Word_Base`'s `","`. Since the frozen corpus was
    /// generated through Python — where BR's `" "` applies — `","` is read back
    /// as the "unset" sentinel it is. This is exact for both a caller who omits
    /// `separator` and one who passes anything other than `","`, and wrong only
    /// for an explicit `separator=","`, which Python renders with a comma and
    /// this renders with a space. See `concerns`; `lang_sa.rs` takes the same
    /// approach for the same reason.
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
        // Python accepts `adjective` and never reads it: BR has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency, so
        // `adjective=True` is byte-identical to the plain call.
        let _ = adjective;

        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` happens *before*
        // `str(val)`, so the string never carries a sign. The abs() is
        // conditional in Python, so it stays conditional here.
        let is_negative = val.is_negative();
        let s = match val {
            CurrencyValue::Int(v) => {
                if is_negative {
                    v.abs().to_string()
                } else {
                    v.to_string()
                }
            }
            CurrencyValue::Decimal { value: d, .. } => {
                if is_negative {
                    d.abs().to_string()
                } else {
                    d.to_string()
                }
            }
        };

        // `str(val).split(".")` splits on every dot; Python then reads only
        // parts[0] and parts[1], so trailing fragments are ignored either way.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`. Evaluated before `right`, so a
        // scientific-notation string raises here first (quirk 12).
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads, so "5" -> "50" (0.5 is 50
        // subunits) and "01" -> "01" (0.01 is 1). Sliced by chars, not bytes.
        // A whole float gives "0" -> "00" -> 0, which is falsy (quirk 10).
        let right = match part1 {
            Some(f) if !f.is_empty() => {
                let mut two: String = f.chars().take(2).collect();
                while two.chars().count() < 2 {
                    two.push('0');
                }
                BigInt::from_str(&two).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — quirk 8.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::one();

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`. Note 0 takes
        // the plural: "zero euroioù".
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 10). `cents=False`
        // drops it too, with no terse fallback (quirk 11).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — raw, keeping the trailing space of
        // "minus " (quirk 6).
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input, but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn f(v: f64) -> Result<String> {
        LangBr::new().to_cardinal_float(
            &FloatValue::Float {
                value: v,
                // BR never reads precision (quirk 15); any value does.
                precision: 0,
            },
            None,
        )
    }

    fn d(s: &str) -> Result<String> {
        LangBr::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 0,
            },
            None,
        )
    }

    fn ok(r: Result<String>) -> String {
        r.expect("expected Ok")
    }

    fn value_err(r: Result<String>) -> String {
        match r {
            Err(N2WError::Value(m)) => m,
            other => panic!("expected N2WError::Value, got {:?}", other),
        }
    }

    /// Every `"lang": "br", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(ok(f(0.0)), "zero point zero");
        assert_eq!(ok(f(0.5)), "zero point pemp");
        assert_eq!(ok(f(1.0)), "unan point zero");
        assert_eq!(ok(f(1.5)), "unan point pemp");
        assert_eq!(ok(f(2.25)), "daou point daou pemp");
        assert_eq!(ok(f(3.14)), "tri point unan pevar");
        assert_eq!(ok(f(0.01)), "zero point zero unan");
        assert_eq!(ok(f(0.1)), "zero point unan");
        assert_eq!(ok(f(0.99)), "zero point nav nav");
        assert_eq!(ok(f(1.01)), "unan point zero unan");
        assert_eq!(ok(f(12.34)), "dek daou point tri pevar");
        assert_eq!(ok(f(99.99)), "dek ha pevar-ugent nav point nav nav");
        assert_eq!(ok(f(100.5)), "unan kant point pemp");
        assert_eq!(
            ok(f(1234.56)),
            "unan mil daou kant tregont pevar point pemp c'hwec'h"
        );
        assert_eq!(ok(f(-0.5)), "minus zero point pemp");
        assert_eq!(ok(f(-1.5)), "minus unan point pemp");
        assert_eq!(ok(f(-12.34)), "minus dek daou point tri pevar");
        assert_eq!(ok(f(1.005)), "unan point zero zero pemp");
        assert_eq!(ok(f(2.675)), "daou point c'hwec'h seizh pemp");
    }

    /// Every `"lang": "br", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(ok(d("0.01")), "zero point zero unan");
        assert_eq!(ok(d("1.10")), "unan point unan zero");
        assert_eq!(ok(d("12.345")), "dek daou point tri pevar pemp");
        assert_eq!(
            ok(d("98746251323029.99")),
            "98746251323029 point nav nav"
        );
        assert_eq!(ok(d("0.001")), "zero point zero zero unan");
    }

    /// Quirk 14: BR reads `repr`, not `float2tuple`, and the two are not
    /// interchangeable.
    ///
    /// `float2tuple(0.17108284528077355)` evaluates `(v - 0) * 10**17` in f64,
    /// gets `1.7108284528077356e16` and reports a final digit of 6;
    /// `repr` kept the 5. Live check: `num2words(0.17108284528077355,
    /// lang="br")` ends "… pemp pemp", i.e. BR's answer, not base's
    /// "… pemp c'hwec'h".
    #[test]
    fn does_not_take_the_float2tuple_route() {
        assert_eq!(
            ok(f(0.17108284528077355)),
            "zero point unan seizh unan zero eizh daou eizh pevar pemp daou \
             eizh zero seizh seizh tri pemp pemp"
        );
        // The corpus's artefact cases, where the two routes happen to agree.
        assert_eq!(ok(f(1.005)), "unan point zero zero pemp");
        assert_eq!(ok(f(2.675)), "daou point c'hwec'h seizh pemp");
    }

    /// Quirk 15: `precision=` is inert — BR never reads `self.precision`.
    /// Live check: num2words(1.2345, lang="br", precision=2) spells all four.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangBr::new();
        let v = FloatValue::Float {
            value: 1.2345,
            precision: 4,
        };
        let expected = "unan point daou tri pevar pemp";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), expected);
        assert_eq!(l.to_cardinal_float(&v, Some(2)).unwrap(), expected);
        assert_eq!(l.to_cardinal_float(&v, Some(8)).unwrap(), expected);
    }

    /// `repr(-0.0)` is "-0.0": the sign *bit*, not `v < 0.0`, drives the minus.
    #[test]
    fn negative_zero_float_keeps_its_sign() {
        assert_eq!(ok(f(-0.0)), "minus zero point zero");
        assert_eq!(ok(f(0.0)), "zero point zero");
    }

    /// Quirk 16: scientific notation dies in `int()`, and which `int()` call
    /// catches it depends on whether `repr` emitted a ".".
    #[test]
    fn scientific_notation_raises_value_error() {
        // One significant digit -> no "." -> the integer branch's int(n).
        assert_eq!(
            value_err(f(1e16)),
            "invalid literal for int() with base 10: '1e+16'"
        );
        assert_eq!(
            value_err(f(1e21)),
            "invalid literal for int() with base 10: '1e+21'"
        );
        assert_eq!(
            value_err(f(1e-5)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        // More than one -> there *is* a "." -> the digit loop's int(digit),
        // which trips on the "e" after consuming the mantissa's digits.
        assert_eq!(
            value_err(f(1.5e16)),
            "invalid literal for int() with base 10: 'e'"
        );
        assert_eq!(
            value_err(f(f64::MAX)),
            "invalid literal for int() with base 10: 'e'"
        );
        // inf / nan take the integer branch too.
        assert_eq!(
            value_err(f(f64::INFINITY)),
            "invalid literal for int() with base 10: 'inf'"
        );
        assert_eq!(
            value_err(f(f64::NEG_INFINITY)),
            "invalid literal for int() with base 10: 'inf'"
        );
        assert_eq!(
            value_err(f(f64::NAN)),
            "invalid literal for int() with base 10: 'nan'"
        );
    }

    /// The repr thresholds are exactly `decpt <= -4` / `decpt > 16`, so these
    /// neighbours land on opposite sides of "works" and "raises".
    #[test]
    fn repr_thresholds() {
        // 1e15 is positional, 1e16 is not.
        assert_eq!(ok(f(1e15)), "1000000000000000 point zero");
        assert!(f(1e16).is_err());
        // 1e-4 is positional, 1e-5 is not.
        assert_eq!(ok(f(1e-4)), "zero point zero zero zero unan");
        assert!(f(1e-5).is_err());
    }

    /// The tie that `{:e}` alone gets wrong: this double is exactly
    /// 1370020896463233.25, equidistant from ...33.2 and ...33.3, and Python
    /// prints the even one. Rust's shortest formatter prints ...33.3.
    #[test]
    fn exact_decimal_ties_round_to_even_like_gay() {
        let v = f64::from_bits(0x4313781b63796605);
        assert_eq!(py_float_str(v), "1370020896463233.2");
        assert_eq!(py_float_str(20682518724725.062), "20682518724725.062");
        // And a plain (non-tie) rounding is unaffected.
        assert_eq!(py_float_str(0.30000000000000004), "0.30000000000000004");
    }

    /// Quirk 17/18: `Decimal` keeps trailing zeros and has its own exponent
    /// thresholds, so it does not agree with `float` on either.
    #[test]
    fn decimal_str_is_not_float_repr() {
        // Trailing zeros are spoken.
        assert_eq!(ok(d("5.00")), "pemp point zero zero");
        // No "." in str(Decimal("5")) -> the *integer* branch.
        assert_eq!(ok(d("5")), "pemp");
        // Decimal goes positional four orders further down than float does.
        assert_eq!(ok(d("1e-5")), "zero point zero zero zero zero unan");
        assert!(f(1e-5).is_err());
        // ... and gives up one order earlier, with a capital, unpadded E.
        assert_eq!(
            value_err(d("1e-7")),
            "invalid literal for int() with base 10: '1E-7'"
        );
        assert_eq!(
            value_err(d("1E+21")),
            "invalid literal for int() with base 10: '1E+21'"
        );
    }

    /// `py_decimal_str` is a port of `Decimal.__str__`, not `BigDecimal`'s
    /// `Display` — which normalises "0.0" to "0" and would drop a word.
    #[test]
    fn decimal_str_matches_python_not_bigdecimal_display() {
        assert_eq!(py_decimal_str(&BigDecimal::from_str("0.0").unwrap()), "0.0");
        assert_eq!(BigDecimal::from_str("0.0").unwrap().to_string(), "0");
        assert_eq!(ok(d("0.0")), "zero point zero");
        assert_eq!(py_decimal_str(&BigDecimal::from_str("1.10").unwrap()), "1.10");
        assert_eq!(py_decimal_str(&BigDecimal::from_str("1e-6").unwrap()), "0.000001");
        assert_eq!(py_decimal_str(&BigDecimal::from_str("-1.10").unwrap()), "-1.10");
    }

    /// Quirk 19 / issue #603: the integer part goes through the 10^9 digit
    /// fallback (quirk 1), and the Decimal arm keeps the trillion-scale digits
    /// a float cast would have rounded away.
    #[test]
    fn large_values() {
        assert_eq!(ok(d("98746251323029.99")), "98746251323029 point nav nav");
        assert_eq!(ok(f(1234567890.5)), "1234567890 point pemp");
        // Just under the fallback: still spelled.
        assert_eq!(
            ok(f(123456789.5)),
            "unan kant ugent tri milion pevar kant hanter-kant c'hwec'h mil \
             seizh kant pevar-ugent nav point pemp"
        );
    }
}
