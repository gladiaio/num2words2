//! Port of `lang_PA.py` (Punjabi / Gurmukhi).
//!
//! Shape: **self-contained**. `Num2Word_PA` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` gate in `Num2Word_Base.__init__` never fires: Python never builds
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal`, `to_ordinal`,
//! `to_ordinal_num` and `to_year` are all overridden outright, so the
//! `splitnum`/`clean`/`merge` engine is entirely unreachable. Accordingly
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check** — see the "digit fallback" note below for what happens
//! past 10^9 instead.
//!
//! Numbering is the Indian system: hundred / thousand / lakh (10^5) /
//! crore (10^7).
//!
//! No cross-call mutable state: all four integer methods are pure functions of
//! their argument, and none of them can fail. PA has no error paths at all for
//! *integer* input.
//!
//! # The float / Decimal path
//!
//! `Num2Word_PA` never defines `to_cardinal_float`; instead its overridden
//! `to_cardinal` works entirely on `str(number)` and branches on `"." in n`:
//!
//! ```text
//! n = str(number).strip()
//! if n.startswith("-"): n = n[1:]; ret = self.negword
//! else: ret = ""
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     ret += " ".join(self._int_to_word(int(d)) for d in right)
//!     return ret.strip()
//! else:
//!     return (ret + self._int_to_word(int(n))).strip()
//! ```
//!
//! So `Num2Word_Base.to_cardinal_float`/`float2tuple` are **never reached**, and
//! neither is the `< 0.01` artefact heuristic: `repr(2.675)` is `"2.675"`, so
//! the `674.9999999999998` that heuristic exists to repair is never computed.
//! Every fractional digit is read straight from the repr and spoken separately,
//! with no cap and no `precision`. Two consequences invert the usual advice:
//!
//! * The `precision=` kwarg is accepted by the dispatcher but `to_cardinal`
//!   never reads it, so it is silently dropped — verified live:
//!   `num2words(1.23456, lang="pa", precision=1)` keeps all six fractional
//!   words. `precision_override` is therefore ignored here.
//! * A value whose `str()` uses exponent notation raises `ValueError`, but the
//!   failing literal in the message depends on whether the repr carries a dot.
//!   A single-digit mantissa has none: `1e16` → `str` `"1e+16"` → the else branch
//!   runs `int("1e+16")`, so the message names `'1e+16'` (likewise `1e-05` names
//!   `'1e-05'`). A multi-digit mantissa does: `1.5e+16` → `str` `"1.5e+16"` → the
//!   `left` `"1"` parses, then the digit loop hits `'e'` and `int("e")` names
//!   `'e'`. Both surface as `N2WError::Value` with Python's exact
//!   `"invalid literal for int() with base 10: '…'"` message.
//!
//! Reproducing this faithfully means reproducing `str(float)`/`str(Decimal)`
//! byte-for-byte — Rust's own `{}` will not do (it never switches to exponent
//! notation and prints `1`, not `1.0`, for integral floats). [`py_float_repr`]
//! and [`py_decimal_str`] port CPython's `format_float_short` and
//! `Decimal.__str__`; they share the reviewed implementation used across the
//! sibling Indian-system ports (e.g. `lang_as`). [`cardinal_from_str`] then does
//! the string surgery. Because PA is byte-identical to those siblings apart from
//! its own digit words, negword and pointword, the same code carries over.
//!
//! # Faithfully reproduced Python quirks
//!
//! These all look wrong but are exactly what Python emits (each is pinned by a
//! corpus row):
//!
//! 1. **The digit fallback.** `_int_to_word` handles values below 10^9 and then
//!    gives up: `else: return str(number)`. So `to_cardinal(10**9)` is the
//!    literal ASCII string `"1000000000"`, not Punjabi words — and
//!    `to_ordinal(10**9)` is `"1000000000ਵਾਂ"`, digits with a Gurmukhi ordinal
//!    suffix glued on. This is PA's de facto ceiling: it degrades to digits
//!    rather than raising `OverflowError`. Corpus confirms this up to 10^21.
//!    Modelled by the final `n.to_string()` arm of [`int_to_word`].
//! 2. **No tens/units copula.** 21 is "ਵੀਹ ਇੱਕ" (literally "twenty one"),
//!    where idiomatic Punjabi would be "ਇੱਕੀ". The module just juxtaposes
//!    `tens[n // 10]` and `ones[n % 10]`. Likewise 99 → "ਨੱਬੇ ਨੌ". Preserved.
//! 3. **Ordinals 6+ are cardinal + suffix, with no sandhi and no negative
//!    guard.** `to_ordinal` special-cases only 1..=5 and otherwise appends
//!    "ਵਾਂ" to the cardinal. Since it never calls `verify_ordinal`, negatives
//!    sail through: `to_ordinal(-42)` == "ਮਾਇਨਸ ਚਾਲੀ ਦੋਵਾਂ" and
//!    `to_ordinal(0)` == "ਸਿਫਰਵਾਂ" (both in the corpus). Most other modules
//!    raise `TypeError` on a negative ordinal; PA does not.
//! 4. **`to_ordinal_num` ignores the language entirely** and returns
//!    `str(number) + "."` — so `to_ordinal_num(-1)` == "-1.". It is the base
//!    class's value-passthrough plus a period, not a Punjabi form.
//! 5. **`to_year` prefixes the ASCII string "BC "** (not a Punjabi era marker)
//!    for negatives, and its `longval` parameter is accepted but never read —
//!    there is no year-pairing logic at all, so `to_year(1999)` is just
//!    `to_cardinal(1999)` == "ਇੱਕ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੱਬੇ ਨੌ" ("one thousand nine
//!    hundred ninety nine"), never "nineteen ninety-nine".
//!
//! # Dead code carried over
//!
//! `_int_to_word`'s `if number < 0` arm is unreachable from *every* mode,
//! currency included: `to_cardinal` strips the sign from the *string* before
//! calling, and `to_currency` takes `abs(val)` before splitting. It is ported
//! anyway so [`int_to_word`] stays a faithful mirror of the Python function.
//!
//! # Currency
//!
//! `Num2Word_PA` overrides `to_currency` **wholesale** — it never reaches
//! `Num2Word_Base.to_currency`, so none of `pluralize` / `_money_verbose` /
//! `_cents_verbose` / `_cents_terse` / `CURRENCY_PRECISION` /
//! `CURRENCY_ADJECTIVES` participate. Those hooks are deliberately left at
//! their trait defaults here. See quirks 6-8 below for what the override does
//! instead.
//!
//! `to_cheque` is *not* overridden, so `Num2Word_Base.to_cheque` runs: it does
//! a strict `self.CURRENCY_FORMS[currency]` (NotImplementedError on a miss),
//! reads `CURRENCY_PRECISION.get(currency, 100)` — PA defines no
//! `CURRENCY_PRECISION`, so every code is 100 — and calls `_money_verbose`,
//! which is `self.to_cardinal`, i.e. PA's override. The trait defaults model
//! all of that already, so [`Lang::to_cheque`] is left alone and only
//! [`Lang::currency_forms`] and [`Lang::lang_name`] are supplied.
//!
//! # Faithfully reproduced Python quirks (currency)
//!
//! 6. **An unknown currency code silently becomes rupees.** `to_currency` does
//!    `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])` — a
//!    `dict.get` with an INR *default*, not the `[currency]` lookup every other
//!    module uses. So `to_currency(1, "GBP")` is "ਇੱਕ ਰੁਪਈਆ" (one rupee), not a
//!    NotImplementedError. Only INR/USD/EUR are real; the corpus pins GBP, JPY,
//!    KWD, BHD, CNY and CHF all rendering as rupees/paise.
//!    `to_cheque` does *not* share the fallback (it is base's strict `[...]`
//!    lookup), so the same code raises there — hence [`Lang::currency_forms`]
//!    stays strict and the fallback lives inside `to_currency` alone.
//! 7. **`CURRENCY_PRECISION` is ignored by `to_currency`.** The override
//!    hardcodes a 2-digit subunit, so the 3-decimal (KWD/BHD, divisor 1000) and
//!    0-decimal (JPY, divisor 1) currencies get neither treatment — and since
//!    they are not in `CURRENCY_FORMS` anyway they arrive as rupees first. Per
//!    the corpus, `to_currency(12.34, "JPY")` == `to_currency(12.34, "KWD")` ==
//!    "ਬਾਰਾਂ ਰੁਪਏ ਤੀਹ ਚਾਰ ਪੈਸੇ".
//! 8. **`adjective` is accepted and never read.** PA defines no
//!    `CURRENCY_ADJECTIVES`, and the override's body never mentions the
//!    parameter, so `adjective=True` is a no-op rather than a prefix.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const NEGWORD: &str = "ਮਾਇਨਸ ";
const POINTWORD: &str = "ਦਸ਼ਮਲਵ";
const ZERO: &str = "ਸਿਫਰ";
const HUNDRED: &str = "ਸੌ";
const THOUSAND: &str = "ਹਜ਼ਾਰ";
const LAKH: &str = "ਲੱਖ";
const CRORE: &str = "ਕਰੋੜ";

/// `ones`; index 0 is "" and is never read (0 short-circuits to `zero`, and the
/// `number % 10` / `number // 100` lookups are guarded by non-zero checks).
const ONES: [&str; 10] = [
    "", "ਇੱਕ", "ਦੋ", "ਤਿੰਨ", "ਚਾਰ", "ਪੰਜ", "ਛੇ", "ਸੱਤ", "ਅੱਠ", "ਨੌ",
];

/// `tens`, indexed by the tens digit. Indices 0 and 1 are unreachable from the
/// `20 <= n < 100` branch that reads this table; note that index 1 duplicates
/// `TEENS[0]` ("ਦਸ" / ten) in the Python source.
const TENS: [&str; 10] = [
    "", "ਦਸ", "ਵੀਹ", "ਤੀਹ", "ਚਾਲੀ", "ਪੰਜਾਹ", "ਸੱਠ", "ਸੱਤਰ", "ਅੱਸੀ", "ਨੱਬੇ",
];

/// `teens`, indexed by `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "ਦਸ", "ਗਿਆਰਾਂ", "ਬਾਰਾਂ", "ਤੇਰਾਂ", "ਚੌਦਾਂ", "ਪੰਦਰਾਂ", "ਸੋਲਾਂ", "ਸਤਾਰਾਂ", "ਅਠਾਰਾਂ", "ਉੱਨੀ",
];

/// The generic ordinal suffix appended to the cardinal for every value outside
/// the 1..=5 special cases (the masculine form; the module's docstring mentions
/// a feminine "ਵੀਂ" but never emits it).
const ORDINAL_SUFFIX: &str = "ਵਾਂ";

/// The hardcoded ordinals for 1..=5, indexed by value. Index 0 is unused.
const ORDINAL_SPECIALS: [&str; 6] = [
    "",        // unused: 0 falls through to cardinal + suffix
    "ਪਹਿਲਾ",   // first
    "ਦੂਜਾ",    // second
    "ਤੀਜਾ",    // third
    "ਚੌਥਾ",    // fourth
    "ਪੰਜਵਾਂ",  // fifth
];

/// Narrow a `BigInt` the caller has already proven small enough to index with.
///
/// Every call site sits inside a `n < 1000` branch, so the `expect` is
/// unreachable by construction.
fn to_usize(n: &BigInt) -> usize {
    n.to_usize().expect("caller proved this value is < 1000")
}

/// Port of `Num2Word_PA._int_to_word`.
///
/// Recursion depth is bounded by the fixed ladder (crore → lakh → thousand →
/// hundred → tens), so it cannot blow the stack regardless of input size.
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO.to_string();
    }

    // Unreachable from the four modes in scope — see "Dead code" in the module
    // docs. Kept so this mirrors the Python function exactly.
    if n.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&n.abs()));
    }

    // From here n >= 1, so Python's `//` and `%` agree with div_mod_floor.
    if n < &BigInt::from(10) {
        return ONES[to_usize(n)].to_string();
    }

    if n < &BigInt::from(20) {
        return TEENS[to_usize(n) - 10].to_string();
    }

    if n < &BigInt::from(100) {
        let v = to_usize(n);
        let mut result = TENS[v / 10].to_string();
        // Python: `if number % 10:` — append the unit only when non-zero.
        if v % 10 != 0 {
            result.push(' ');
            result.push_str(ONES[v % 10]);
        }
        return result;
    }

    if n < &BigInt::from(1000) {
        let v = to_usize(n);
        // NB: the hundreds digit indexes ONES directly rather than recursing,
        // so this arm is only correct because v // 100 is always 1..=9 here.
        let mut result = format!("{} {}", ONES[v / 100], HUNDRED);
        let remainder = v % 100;
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    // The Indian-system ladder. Python's inline comments name the *next*
    // boundary rather than the branch's own unit (`elif number < 100000:
    // # Lakh` is in fact the *thousand* branch), which is why they read as
    // off-by-one. The behaviour below follows the code, not the comments.
    for (limit, divisor, word) in [
        (100_000u64, 1_000u64, THOUSAND),
        (10_000_000u64, 100_000u64, LAKH),
        (1_000_000_000u64, 10_000_000u64, CRORE),
    ] {
        if n < &BigInt::from(limit) {
            let (quotient, remainder) = n.div_mod_floor(&BigInt::from(divisor));
            let mut result = format!("{} {}", int_to_word(&quotient), word);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&int_to_word(&remainder));
            }
            return result;
        }
    }

    // `else: return str(number)` — the digit fallback. See quirk 1.
    n.to_string()
}

/// Shortest round-trip decimal digits of a **non-negative, finite** f64, plus
/// the decimal point position `decpt` (value == 0.d1d2… × 10^decpt, i.e. the
/// first digit sits just left of position `decpt`). This is CPython's
/// `_Py_dg_dtoa(mode 0)`, the David Gay shortest-representation used by `repr`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with Gay on the digit
/// *count* and, almost always, the digits. It disagrees on **exact ties**: when
/// the value sits precisely halfway between two shortest candidates, Gay's dtoa
/// picks the one with an **even** last digit while Rust rounds half **up**.
/// `repr(-78198386800398.125)` is `'-78198386800398.12'`; Rust's `{:e}` says
/// `…13`. This mirrors the reviewed sibling implementation (`lang_as`), fuzzed
/// over millions of values, where that tie is the only divergence.
///
/// Detecting the tie needs no bignum. Write `a = m·2^e` with `m` odd and let
/// `q = digits.len() - decpt`. The tie condition reduces to `e + q + 1 == 0`,
/// plus (when `q < 0`) `5^-q | m` with `-q <= 22`. In a tie `2k+1 == m·5^q`
/// (or `m / 5^-q`), and since `5 ≡ 1 (mod 4)` that odd integer is `≡ m (mod 4)`,
/// so `k` is even exactly when `m % 4 == 1`. The fix-up steps Rust's odd last
/// digit toward the even neighbour, `k`'s parity choosing the direction.
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let mut digits: Vec<u8> = mant.bytes().filter(|c| *c != b'.').collect();
    let mut decpt: i32 = exp.parse::<i32>().expect("{:e} exponent is an integer") + 1;

    // Decompose a == m * 2**e exactly, then reduce m to odd.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mut m, mut e) = if biased == 0 {
        (frac, -1074i32) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };
    if m == 0 {
        // a == 0.0: dtoa reports digits "0", decpt 1. No tie to break.
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }
    let z = m.trailing_zeros() as i32;
    m >>= z;
    e += z;

    let q = digits.len() as i32 - decpt;
    let mut tie = e + q + 1 == 0;
    if tie && q < 0 {
        let r = -q as u32;
        tie = r <= 22 && m % 5u64.pow(r) == 0;
    }
    if !tie {
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }

    let last = digits[digits.len() - 1] - b'0';
    if last % 2 == 1 {
        if m % 4 == 1 {
            // k even: Python wants k, Rust gave k+1. Odd last digit, so no borrow.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k odd: Python wants k+1, Rust gave k. Carry like dtoa's roundoff.
            let mut i = digits.len();
            loop {
                if i == 0 {
                    digits.insert(0, b'1');
                    decpt += 1;
                    break;
                }
                i -= 1;
                if digits[i] == b'9' {
                    digits[i] = b'0';
                } else {
                    digits[i] += 1;
                    break;
                }
            }
        }
        // dtoa never emits trailing zeros; stripping them leaves decpt alone.
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), which `Num2Word_PA.to_cardinal`
/// promotes from a formatting detail to *the entire specification* of the float
/// path. This is CPython's `format_float_short(..., 'r', ...)` in `pystrtod.c`.
///
/// Rust's `{}` cannot stand in: it never switches to exponent notation
/// (`format!("{}", 1e16_f64)` is `"10000000000000000"`, where Python says
/// `'1e+16'` — the difference that makes `to_cardinal(1e16)` raise `ValueError`)
/// and it prints `1`, not `1.0`, for integral floats. The rules, from
/// `format_float_short`:
///
/// * exponent notation iff `decpt <= -4 || decpt > 16` (the `> 16`, not `> 17`,
///   is deliberate upstream);
/// * the exponent is `%+.02d`: signed, zero-padded to two digits — `1e-05` but
///   `1e+100`;
/// * `Py_DTSF_ADD_DOT_0` appends `.0` to an otherwise integral fixed-notation
///   result, but never in exponent notation (`repr(1e16) == '1e+16'`);
/// * `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and to_cardinal strips
    // that minus textually into a negword.
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_digits(value.abs());
    let ndigits = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let exp = decpt - 1;
        let mut mant = String::from(&digits[..1]);
        if digits.len() > 1 {
            mant.push('.');
            mant.push_str(&digits[1..]);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        // 0.5 -> decpt 0 -> "0." + "" + "5"; 0.01 -> decpt -1 -> "0." + "0" + "1".
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        // Integral: pad right with zeros, then ADD_DOT_0. 1.0 -> "1" + ".0".
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - ndigits) as usize)
        )
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__` with `eng=False` and
/// the default context (uppercase `E`).
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`: the shim builds this with `BigDecimal::from_str(str)`,
/// preserving trailing zeros and negative exponents, so `"1.10"` round-trips as
/// `(110, 2)` and `"1E+16"` as `(1, -16)`.
///
/// # The negative-zero hole
///
/// `Decimal` carries `_sign` independently, so `Decimal("-0.0")` is signed zero
/// and `str()` gives `'-0.0'`; `to_cardinal` then strips that minus textually and
/// answers "ਮਾਇਨਸ ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ". A `BigDecimal` cannot represent it — its
/// `int_val` is a `BigInt` with no negative zero, so `BigDecimal::from_str("-0.0")`
/// has already discarded the sign before this function runs. We emit `'0.0'` and
/// drop the negword. The discriminator is the original string, which the
/// `FloatValue::Decimal` boundary (in `num2words2-py`) does not carry, so the fix
/// is out of this port's remit. Flagged in the report. Blast radius is exactly
/// the fixed-notation negative zeros (`-0` … `-0.000000`); beyond that the `E±n`
/// form raises `ValueError` regardless of sign.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // i128 so that `-scale` cannot overflow for a pathological i64::MIN scale.
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string(); // Decimal._int
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let d = dotplace as usize;
        (int_digits[..d].to_string(), format!(".{}", &int_digits[d..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — signed, but *not* zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)`, for the fragments [`cardinal_from_str`] hands it — always
/// ASCII pieces of `str(float)`/`str(Decimal)`. Ports the underscore rule
/// (`int("1_0") == 10`, `int("1_")` raises) and, crucially, the error message,
/// which formats the original argument with `%.200R` (i.e. `repr(s)`); every
/// literal that reaches here is plain ASCII, so `'{}'` matches what `repr` prints.
fn py_int(s: &str) -> Result<BigInt> {
    let err = || {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    };
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    // int() permits '_' as a digit separator, but not leading, trailing or doubled.
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body.chars().all(|c| c.is_ascii_digit() || c == '_')
    {
        return Err(err());
    }
    let digits: String = body.chars().filter(|c| *c != '_').collect();
    let n: BigInt = digits.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

/// The body of `Num2Word_PA.to_cardinal` when its argument is a float/Decimal,
/// driven by `str(number)` (see the module docs for the Python).
///
/// Three details that matter, all shared with the sibling ports:
///
/// * The sign is stripped **textually**, so `-0.0` (whose `str` is `"-0.0"`)
///   keeps its negword even though the value is not `< 0`: Python answers
///   "ਮਾਇਨਸ ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ".
/// * `split(".", 1)` caps at one split, so a second dot stays inside `right` and
///   detonates in the digit loop rather than being ignored.
/// * `int(left)` runs *before* the digit generator, so for `1.5e+16` the failing
///   literal reported is `'e'`, not `'1.5e+16'` — order is load-bearing for the
///   `ValueError` message.
///
/// PA's `to_cardinal` `.strip()`s both return values; the trailing `.trim()`
/// here mirrors that. In practice it is a no-op (`int_to_word` never returns ""
/// and the string never ends in a space), kept for fidelity.
fn cardinal_from_str(number: &str) -> Result<String> {
    let n = number.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    let Some(dot) = n.find('.') else {
        // else: (ret + self._int_to_word(int(n))).strip()
        ret.push_str(&int_to_word(&py_int(n)?));
        return Ok(ret.trim().to_string());
    };

    // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
    let (left, right) = (&n[..dot], &n[dot + 1..]);
    ret.push_str(&int_to_word(&py_int(left)?));
    ret.push(' ');
    ret.push_str(POINTWORD);
    ret.push(' ');

    // " ".join(self._int_to_word(int(d)) for d in right) — iterate *characters*.
    let mut first = true;
    for d in right.chars() {
        if !first {
            ret.push(' ');
        }
        first = false;
        let mut buf = [0u8; 4];
        ret.push_str(&int_to_word(&py_int(d.encode_utf8(&mut buf))?));
    }
    Ok(ret.trim().to_string())
}

/// Python's `parse`-by-string-surgery in `to_currency`, as `(left, right)`.
///
/// The Python is three lines of string manipulation on `str(abs(val))`:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `str(val)` is gone by the time we get here — the boundary hands us
/// `Decimal(str(value))` — so this reproduces the surgery on the decimal's own
/// plain digit string, which is what `str()` renders for every value the two
/// sides can agree on. Equivalences worth stating, because they are the whole
/// reason this is safe:
///
/// * `parts[0]` is the integer digits, so `left` is `trunc(|val|)`.
/// * `parts[1][:2]` **truncates** to two digits and `.ljust(2, "0")` pads a
///   1-digit fraction on the *right* — so `0.5` is 50 paise, not 5, and
///   `12.345` is 34, never rounded up to 35.
/// * A missing `"."` and a fraction of `"0"` both yield `right == 0`, so
///   `1.0` and `1` agree here. They diverge only via `CurrencyValue`'s
///   int/decimal split, which this function keeps: an `Int` never has a
///   fractional part to read.
///
/// Scale `<= 0` (an integral `Decimal` such as `Decimal("12")`) has no `"."`
/// in `str()` either, so `right` is 0 there too.
fn split_currency_parts(val: &CurrencyValue) -> (BigInt, BigInt) {
    let d = match val {
        // `str(int)` never contains ".", so parts[1] does not exist.
        CurrencyValue::Int(v) => return (v.abs(), BigInt::zero()),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    // value == unscaled * 10^-scale; `abs()` above makes unscaled >= 0.
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let digits = unscaled.to_string();

    if scale <= 0 {
        // Integral: str() has no "." -> right = 0. Trailing zeros are implied
        // by the negative scale, so append them to get the integer digits.
        let mut int_digits = digits;
        int_digits.push_str(&"0".repeat((-scale) as usize));
        return (parse_digits(&int_digits), BigInt::zero());
    }

    // Left-pad so there is at least one integer digit, exactly as str() writes
    // "0.01" rather than ".01".
    let scale = scale as usize;
    let padded = if digits.len() <= scale {
        format!("{}{}", "0".repeat(scale - digits.len() + 1), digits)
    } else {
        digits
    };
    let split_at = padded.len() - scale;
    let (int_part, frac_part) = padded.split_at(split_at);

    // `parts[1][:2].ljust(2, "0")` — truncate to two digits, then pad right.
    let mut cents: String = frac_part.chars().take(2).collect();
    while cents.len() < 2 {
        cents.push('0');
    }

    (parse_digits(int_part), parse_digits(&cents))
}

/// `int(<ascii digit string>)`. Every caller feeds it digits it just built.
fn parse_digits(s: &str) -> BigInt {
    s.parse::<BigInt>()
        .expect("digit string built from a BigInt's own decimal form")
}

pub struct LangPa {
    /// `CURRENCY_FORMS`, built once. `Num2Word_PA` declares it directly on the
    /// class and inherits nothing from `lang_EUR`/`lang_EU`, so these three
    /// codes are the whole table — INR is Punjabi, USD/EUR are (in the Python)
    /// untranslated English.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangPa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPa {
    pub fn new() -> Self {
        let mut forms = HashMap::new();
        forms.insert(
            "INR",
            CurrencyForms::new(&["ਰੁਪਈਆ", "ਰੁਪਏ"], &["ਪੈਸਾ", "ਪੈਸੇ"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]),
        );
        LangPa { forms }
    }

    /// `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`.
    ///
    /// The INR fallback is `to_currency`-only — see quirk 6. Kept private so
    /// it cannot be mistaken for the strict [`Lang::currency_forms`] hook that
    /// `to_cheque` needs.
    fn forms_or_inr(&self, currency: &str) -> &CurrencyForms {
        self.forms
            .get(currency)
            .or_else(|| self.forms.get("INR"))
            .expect("INR is always present in the table")
    }
}

impl Lang for LangPa {

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
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "INR"
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
        "ਦਸ਼ਮਲਵ"
    }

    /// The float/Decimal path. `Num2Word_PA` never defines `to_cardinal_float`;
    /// its `to_cardinal` handles both via `str(number)` and branches on
    /// `"." in n`. `Num2Word_Base.to_cardinal_float`/`float2tuple` are therefore
    /// never reached — see the module docs.
    ///
    /// `precision_override` (the `precision=` kwarg) is accepted by the
    /// dispatcher but `to_cardinal` never reads it, so it is dropped here, the
    /// same shape as `to_year`'s `longval` (quirk 5) and `to_currency`'s
    /// `adjective` (quirk 8).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            // Python's str(float). The raw f64 crosses the boundary precisely so
            // that repr() can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    /// Port of `Num2Word_PA.to_cardinal`, integer path only.
    ///
    /// Python works on `str(number).strip()` and tests `n.startswith("-")` to
    /// split the sign, then `"." in n` to pick the float path. `str(int)` never
    /// contains a ".", so integers always take the `else` branch:
    /// `(ret + self._int_to_word(int(n))).strip()`. Stripping the sign from the
    /// string and re-parsing is exactly `abs`, so that is what we do.
    ///
    /// The trailing `.strip()` is load-bearing only because `negword` ends in a
    /// space: `_int_to_word` never returns "" (0 short-circuits to `zero`), so
    /// the space is always followed by a word and `trim` is a no-op in practice.
    /// Kept regardless, to mirror Python.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };
        Ok(format!("{}{}", ret, int_to_word(&value.abs()))
            .trim()
            .to_string())
    }

    /// Port of `Num2Word_PA.to_ordinal`.
    ///
    /// Special forms for 1..=5, otherwise cardinal + "ਵਾਂ". No `verify_ordinal`
    /// call, so negatives and 0 are accepted rather than raising — see quirk 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_positive() {
            if let Some(i) = value.to_usize() {
                if i <= 5 {
                    return Ok(ORDINAL_SPECIALS[i].to_string());
                }
            }
        }
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_PA.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Overrides the base class (which returns the value untouched), so the
    /// trait default is *not* correct here — see quirk 4.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_PA.to_year`.
    ///
    /// `longval` is accepted by Python but never read; the positive branch is
    /// literally `"" + self.to_cardinal(val)`. See quirk 5.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("BC {}", self.to_cardinal(&value.abs())?))
        } else {
            self.to_cardinal(value)
        }
    }

    /// `to_ordinal(float/Decimal)`: the `number == 1 ... == 5` chain matches
    /// numerically, so whole 1..=5 (1.0, Decimal("5.00")) take the special
    /// forms; everything else is `to_cardinal(number) + "ਵਾਂ"` off the
    /// value's own str() grammar ("ਸੱਤ ਦਸ਼ਮਲਵ ਸਿਫਰਵਾਂ" for 7.0).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i.is_positive() && i <= BigInt::from(5) {
                return self.to_ordinal(&i);
            }
        }
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — Python's str,
    /// handed in as `repr_str` ("5.0.", "-0.0.", "1E+2.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `if val < 0` (numeric) → `"BC " +
    /// to_cardinal(abs(val))`, keeping the float grammar.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if negative {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: value.abs(),
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: value.abs(),
                    precision: *precision,
                },
            };
            Ok(format!("BC {}", self.cardinal_float_entry(&abs, None)?))
        } else {
            self.cardinal_float_entry(value, None)
        }
    }

    /// `str_to_number` stays Base's `Decimal(value)`, but PA's `to_cardinal`
    /// then runs `int()` over the dot-free `str()` form, so `"Infinity"`
    /// raises **ValueError** (`int('Infinity')`) rather than the shared Inf
    /// sentinel's OverflowError.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_PA"
    }

    /// The strict `self.CURRENCY_FORMS[currency]` lookup.
    ///
    /// Only `to_cheque` (inherited from `Num2Word_Base`) reads this, and it
    /// must raise NotImplementedError on a miss — so, unlike `to_currency`,
    /// there is **no** INR fallback here. See quirk 6.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Port of `Num2Word_PA.to_currency`, which replaces the base version
    /// entirely.
    ///
    /// ```text
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// ...
    /// left_str = self._int_to_word(left)
    /// result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     cents_str = self._int_to_word(right)
    ///     result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// Three things this does *not* do, all of them base-class behaviour PA
    /// never inherits: it does not call `pluralize` (it indexes the form tuple
    /// inline), it does not consult `CURRENCY_PRECISION` (quirk 7), and it
    /// does not honour `adjective` (quirk 8).
    ///
    /// Note `if cents and right:` gates on `right` being *truthy*, so a float
    /// with zero cents — `1.0` — prints no cents segment at all. That is PA
    /// diverging from `Num2Word_Base`, which would render "... zero cents"
    /// for the same input. The int/float distinction still survives elsewhere:
    /// it is what `split_currency_parts` preserves.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // `adjective` is in the Python signature but never read — quirk 8.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let is_negative = val.is_negative();
        // Python takes abs(val) before the split; split_currency_parts does the
        // same, so `left`/`right` are always non-negative from here.
        let (left, right) = split_currency_parts(val);
        let forms = self.forms_or_inr(currency);

        // `cr1[1] if left != 1 else cr1[0]`. Both PA tables carry exactly two
        // forms, so the fixed indices cannot go out of range.
        let unit = if left.is_one() {
            &forms.unit[0]
        } else {
            &forms.unit[1]
        };
        let mut result = format!("{} {}", int_to_word(&left), unit);

        // `if cents and right:` — a zero `right` is falsy and skips the whole
        // segment, no matter how the value was written.
        if cents && !right.is_zero() {
            let subunit = if right.is_one() {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            };
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        if is_negative {
            // `self.negword + result` — negword already ends in a space, so
            // this is not the `"%s " % negword.strip()` form base uses.
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's trailing .strip(). A no-op in practice: int_to_word never
        // returns "" and the string always ends in a currency word. Mirrored
        // anyway.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use std::str::FromStr;

    fn flt(v: f64) -> String {
        // `precision` is what the shim derives from repr() on the Python side.
        // Num2Word_PA ignores it, so any value here is equally correct.
        let precision = py_float_repr(v)
            .split_once('.')
            .map(|(_, f)| f.len() as u32)
            .unwrap_or(0);
        LangPa::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        LangPa::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `"lang": "pa", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(flt(0.0), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ");
        assert_eq!(flt(0.5), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਪੰਜ");
        assert_eq!(flt(1.0), "ਇੱਕ ਦਸ਼ਮਲਵ ਸਿਫਰ");
        assert_eq!(flt(1.5), "ਇੱਕ ਦਸ਼ਮਲਵ ਪੰਜ");
        assert_eq!(flt(2.25), "ਦੋ ਦਸ਼ਮਲਵ ਦੋ ਪੰਜ");
        assert_eq!(flt(3.14), "ਤਿੰਨ ਦਸ਼ਮਲਵ ਇੱਕ ਚਾਰ");
        assert_eq!(flt(0.01), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ ਇੱਕ");
        assert_eq!(flt(0.1), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਇੱਕ");
        assert_eq!(flt(0.99), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਨੌ ਨੌ");
        assert_eq!(flt(1.01), "ਇੱਕ ਦਸ਼ਮਲਵ ਸਿਫਰ ਇੱਕ");
        assert_eq!(flt(12.34), "ਬਾਰਾਂ ਦਸ਼ਮਲਵ ਤਿੰਨ ਚਾਰ");
        assert_eq!(flt(99.99), "ਨੱਬੇ ਨੌ ਦਸ਼ਮਲਵ ਨੌ ਨੌ");
        assert_eq!(flt(100.5), "ਇੱਕ ਸੌ ਦਸ਼ਮਲਵ ਪੰਜ");
        assert_eq!(flt(1234.56), "ਇੱਕ ਹਜ਼ਾਰ ਦੋ ਸੌ ਤੀਹ ਚਾਰ ਦਸ਼ਮਲਵ ਪੰਜ ਛੇ");
        assert_eq!(flt(-0.5), "ਮਾਇਨਸ ਸਿਫਰ ਦਸ਼ਮਲਵ ਪੰਜ");
        assert_eq!(flt(-1.5), "ਮਾਇਨਸ ਇੱਕ ਦਸ਼ਮਲਵ ਪੰਜ");
        assert_eq!(flt(-12.34), "ਮਾਇਨਸ ਬਾਰਾਂ ਦਸ਼ਮਲਵ ਤਿੰਨ ਚਾਰ");
        // The two f64-artefact cases. Right answer, but via repr(), not via
        // float2tuple's `< 0.01` rescue — see the module docs.
        assert_eq!(flt(1.005), "ਇੱਕ ਦਸ਼ਮਲਵ ਸਿਫਰ ਸਿਫਰ ਪੰਜ");
        assert_eq!(flt(2.675), "ਦੋ ਦਸ਼ਮਲਵ ਛੇ ਸੱਤ ਪੰਜ");
    }

    /// Every `"lang": "pa", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(dec("0.01"), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ ਇੱਕ");
        // Trailing zero survives: str(Decimal("1.10")) == "1.10", two digits.
        assert_eq!(dec("1.10"), "ਇੱਕ ਦਸ਼ਮਲਵ ਇੱਕ ਸਿਫਰ");
        assert_eq!(dec("12.345"), "ਬਾਰਾਂ ਦਸ਼ਮਲਵ ਤਿੰਨ ਚਾਰ ਪੰਜ");
        // Issue #603's value: exact at trillion scale, no float() cast. The
        // integer part overflows PA's ladder and degrades to ASCII digits.
        assert_eq!(dec("98746251323029.99"), "98746251323029 ਦਸ਼ਮਲਵ ਨੌ ਨੌ");
        assert_eq!(dec("0.001"), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ ਸਿਫਰ ਇੱਕ");
    }

    /// -0.0 is not `< 0`, but its str() starts with '-', and to_cardinal strips
    /// the sign textually.
    #[test]
    fn negative_zero_keeps_negword() {
        assert_eq!(flt(-0.0), "ਮਾਇਨਸ ਸਿਫਰ ਦਸ਼ਮਲਵ ਸਿਫਰ");
    }

    /// repr() ties round half-to-even like Gay's dtoa, not half-up. The integer
    /// part overflows the ladder and degrades to digits; the fraction is "12".
    #[test]
    fn repr_tie_to_even() {
        assert_eq!(flt(-78198386800398.125), "ਮਾਇਨਸ 78198386800398 ਦਸ਼ਮਲਵ ਇੱਕ ਦੋ");
    }

    /// A value whose str() is exponent notation raises ValueError with Python's
    /// exact message. Single-digit mantissa (no dot) names the whole literal;
    /// multi-digit mantissa (has a dot) fails on 'e' inside the digit loop.
    #[test]
    fn exponent_notation_raises_value_error() {
        let cases = [
            (1e16, "1e+16"),
            (1e-05, "1e-05"),
            (1.5e16, "e"),
            (1.5e-05, "e"),
        ];
        for (v, lit) in cases {
            let precision = py_float_repr(v)
                .split_once('.')
                .map(|(_, f)| f.len() as u32)
                .unwrap_or(0);
            match LangPa::new()
                .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", lit),
                    "value {:?}",
                    v
                ),
                other => panic!("expected ValueError for {:?}, got {:?}", v, other),
            }
        }
    }

    /// precision= is accepted by the dispatcher but Num2Word_PA never reads it.
    #[test]
    fn precision_override_ignored() {
        assert_eq!(
            flt(1.23456),
            LangPa::new()
                .to_cardinal_float(
                    &FloatValue::Float {
                        value: 1.23456,
                        precision: 5,
                    },
                    Some(1),
                )
                .unwrap()
        );
    }
}
