//! Port of `lang_BAN.py` (Balinese).
//!
//! Registry check: `CONVERTER_CLASSES["ban"]` → `lang_BAN.Num2Word_BAN`, so
//! this file ports the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_BAN` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no `merge`, so
//! Python never builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is
//! overridden outright and drives a hand-written `_int_to_word` cascade.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here and
//! there is **no overflow check** — see the 10^9 ceiling below, which degrades
//! silently instead of raising.
//!
//! Everything in scope is overridden by BAN itself, so no `Num2Word_Base`
//! behaviour survives into the four supported modes:
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden: `"ka-" + to_cardinal(n)`
//!   * `to_ordinal_num` — overridden: `"ka-" + str(n)`
//!   * `to_year`        — overridden: `to_cardinal(val)` (the `longval` kwarg
//!     is accepted and then ignored, so base's century-splitting never runs)
//!
//! `setup()` also assigns `exclude_title` / `pointword`, but since
//! `to_cardinal` is overridden it never calls `self.title()`, so `is_title`
//! and `exclude_title` are dead for every mode in scope. Output is always
//! lowercase, as the corpus confirms.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. Both of the following look like bugs and are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **The 10^9 cliff (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` has no branch above `million`, so its final statement is
//!    a bare `return str(number)`: `to_cardinal(10**9)` was `"1000000000"`
//!    and `to_ordinal(10**9)` `"ka-1000000000"`, silently. No reliable
//!    Balinese word for 10^9 could be sourced (Wikipedia and Omniglot stop at
//!    `yuta`; the only candidates — "a keti bara", "siu yuta", "miliun" —
//!    come from a single popular article), so instead of inventing one this
//!    port reports `maxval` == 10^9 and raises `OverflowError` from there, on
//!    the integer, float and currency paths alike.
//!
//! 2. **`to_ordinal` on negatives keeps the minus word inline.** `to_ordinal`
//!    blindly prefixes `"ka-"` to whatever `to_cardinal` returned, so
//!    `to_ordinal(-1)` == `"ka-minus siki"` and `to_ordinal_num(-1)` ==
//!    `"ka--1"` (double hyphen — `"ka-" + str(-1)`). Neither is tidied.
//!
//! # Error behaviour
//!
//! For integer input BAN cannot raise: every list index is guarded by a range
//! check (`ONES[o]` only when `o != 0`, `TENS[t]` only for `t` in 1..=9,
//! `ONES[h]` only for `h` in 2..=9), and the sole `int()` call parses a string
//! this code produced.
//!
//! The currency surface adds two reachable raises, both inherited or implied
//! rather than deliberate:
//!
//! * `to_cheque` on a code outside `CURRENCY_FORMS` — `Num2Word_Base.to_cheque`
//!   catches the `KeyError` from `self.CURRENCY_FORMS[currency]` and re-raises
//!   `NotImplementedError`. Seven of the nine `cheque:*` corpus rows are this.
//! * `to_currency` on a value whose repr is exponential — see below.
//!
//! # The exponential-repr gap (`to_currency`)
//!
//! `to_currency` re-stringifies its argument (`str(abs(val)).split(".")`) and
//! feeds the pieces to `int()`. Python's repr switches to exponential notation
//! outside `1e-4 <= |x| < 1e16`, and `int("1e+16")` raises **ValueError** — so
//! BAN crashes on large and tiny floats rather than converting them.
//!
//! Only half of that is reproducible here. Values cross the FFI boundary as
//! `str(value)` parsed into a `BigDecimal`, which keeps the digits and scale
//! but not the *notation*:
//!
//! | input | Python | here | why |
//! |---|---|---|---|
//! | `1e16`, `Decimal("1E+2")` | ValueError | ValueError | scale < 0 survives the parse |
//! | `1e-05` | ValueError | `"nol rupiah"` | parses to the same `(1, 5)` as `Decimal("0.00001")` |
//!
//! The second row is unresolvable in the core: `Decimal("0.00001")` has the
//! *identical* `BigDecimal` and Python converts it happily, so telling the two
//! apart needs a float-vs-Decimal flag the boundary does not carry (`is_int`
//! only separates `int`). The non-raising branch is chosen because it is the
//! one that matches `Decimal`. No corpus row reaches either case.
//!
//! # The negative-zero gap (`to_cardinal` on a Decimal)
//!
//! One more divergence of the same shape, introduced by the float phase and
//! likewise unreachable from the corpus. `to_cardinal` decides its sign from
//! the *string* (`n.startswith("-")`), not from `val < 0`, so a signed zero
//! keeps its minus:
//!
//! | input | Python | here |
//! |---|---|---|
//! | `-0.0` (float) | `"minus nol koma nol"` | `"minus nol koma nol"` |
//! | `Decimal("-0.0")` | `"minus nol koma nol"` | `"nol koma nol"` |
//! | `Decimal("-0")` | `"minus nol"` | `"nol"` |
//!
//! The float row survives because an `f64` carries a sign bit and
//! `is_sign_negative()` reads it. The Decimal rows cannot: `BigDecimal` is
//! backed by a `BigInt`, which has no negative zero, so
//! `BigDecimal::from_str("-0.0").sign()` is already `NoSign` — the shim parses
//! the string in `num2words2-py`, and the sign is gone before any code in this
//! file runs. Recovering it needs the raw string across the boundary.
//!
//! Note this is specific to `to_cardinal`'s string-driven sign test.
//! `to_currency` is unaffected: it uses `val < 0`, and `Decimal("-0.0") < 0`
//! is `False` in Python too, so both sides agree on "nol rupiah".

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

/// `self.ones`. Index 0 is `""` and is never reached: `_int_to_word` short
/// circuits `number == 0` to "nol", and every other read is guarded by a
/// non-zero check.
const ONES: [&str; 10] = [
    "", "siki", "dua", "telu", "papat", "lima", "nem", "pitu", "kutus", "sia",
];

/// `self.tens`. Index 0 is `""` and is unreachable (`t >= 1` whenever the
/// tens branch runs, since it needs `number >= 10`).
const TENS: [&str; 10] = [
    "",
    "dasa",
    "kalih dasa",
    "tigang dasa",
    "petang dasa",
    "seket",
    "nem dasa",
    "pitung dasa",
    "kutus dasa",
    "siangang dasa",
];

const HUNDRED: &str = "satus";
const THOUSAND: &str = "siu";
const MILLION: &str = "yuta";

/// `self.negword`. The trailing space is part of the Python literal; the
/// `.strip()` in `to_cardinal` never actually trims it because the word sits
/// at the *start* of the result.
const NEGWORD: &str = "minus ";

/// Port of `Num2Word_BAN._int_to_word`.
///
/// Only ever reached with a non-negative value: `to_cardinal` peels the sign
/// off the *string* before parsing, so the recursion below never sees one.
///
/// Python's 10^9 fallback (`return str(number)`) is replaced by the maxval
/// check (gladiaio/num2words2#147) — see the module docs. The recursion depth
/// stays trivially bounded (at most: millions → thousands → hundreds → tens).
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    if number.is_zero() {
        return Ok("nol".to_string());
    }

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);

    // if number < 10: return self.ones[number]
    if number < &ten {
        return Ok(ONES[number.to_usize().expect("0..=9 fits usize")].to_string());
    }

    // if number < 100: t, o = divmod(number, 10)
    //                  return self.tens[t] + (" " + self.ones[o] if o else "")
    if number < &hundred {
        let (t, o) = number.div_mod_floor(&ten);
        let mut s = TENS[t.to_usize().expect("1..=9 fits usize")].to_string();
        if !o.is_zero() {
            s.push(' ');
            s.push_str(ONES[o.to_usize().expect("1..=9 fits usize")]);
        }
        return Ok(s);
    }

    // if number < 1000: h, r = divmod(number, 100)
    //                   base = (self.ones[h] + " " if h > 1 else "") + self.hundred
    //                   return base + (" " + self._int_to_word(r) if r else "")
    //
    // The `h > 1` guard is what makes 100 == "satus" rather than "siki satus";
    // note it is a conditional *expression*, so h == 1 contributes nothing at
    // all (not even a space).
    if number < &thousand {
        let (h, r) = number.div_mod_floor(&hundred);
        let mut s = String::new();
        if h > BigInt::one() {
            s.push_str(ONES[h.to_usize().expect("2..=9 fits usize")]);
            s.push(' ');
        }
        s.push_str(HUNDRED);
        if !r.is_zero() {
            s.push(' ');
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // if number < 1000000: t, r = divmod(number, 1000)
    //                      base = self._int_to_word(t) + " " + self.thousand
    //                      return base + (" " + self._int_to_word(r) if r else "")
    //
    // No `t > 1` guard here, unlike hundreds — hence 1000 == "siki siu".
    if number < &million {
        let (t, r) = number.div_mod_floor(&thousand);
        let mut s = int_to_word(&t)?;
        s.push(' ');
        s.push_str(THOUSAND);
        if !r.is_zero() {
            s.push(' ');
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // if number < 1000000000: m, r = divmod(number, 1000000)
    // (always true past the maxval check above)
    //                         base = self._int_to_word(m) + " " + self.million
    //                         return base + (" " + self._int_to_word(r) if r else "")
    if number < &billion {
        let (m, r) = number.div_mod_floor(&million);
        let mut s = int_to_word(&m)?;
        s.push(' ');
        s.push_str(MILLION);
        if !r.is_zero() {
            s.push(' ');
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // Python: `return str(number)` — the 10^9 cliff, now an OverflowError
    // raised by the maxval check above (#147).
    unreachable!("values >= 10^9 are rejected by check_maxval")
}

/// The exclusive ceiling (gladiaio/num2words2#147): there is no attested
/// Balinese scale word above `yuta` (10^6) to compose 10^9 with.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(9))
}

/// `Num2Word_BAN.to_currency`'s fallback key.
///
/// Python does `self.CURRENCY_FORMS.get(currency, list(...values())[0])`.
/// `CURRENCY_FORMS` is a dict literal and dicts preserve insertion order, so
/// `values()[0]` is unconditionally the **IDR** entry. A `HashMap` has no
/// first element, so the fallback is pinned by name instead of by position.
const FALLBACK_CURRENCY: &str = "IDR";

pub struct LangBan {
    /// `Num2Word_BAN.CURRENCY_FORMS`, built once (the registry caches the
    /// instance in a `OnceLock`) rather than per call.
    ///
    /// `Num2Word_BAN` declares the dict on the class itself and never mutates
    /// a shared parent dict, so — unlike the `Num2Word_EUR` families — there
    /// is no import-time rewrite by `Num2Word_EN.__init__` to reproduce. The
    /// literal in `lang_BAN.py` *is* what runs; verified against the live
    /// interpreter (see the test below). Only these three codes exist:
    /// everything else falls back (`to_currency`) or raises (`to_cheque`).
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangBan {
    fn default() -> Self {
        Self::new()
    }
}

impl LangBan {
    pub fn new() -> Self {
        let mut forms = HashMap::new();
        forms.insert("IDR", CurrencyForms::new(&["rupiah", "rupiah"], &["sen", "sen"]));
        forms.insert("USD", CurrencyForms::new(&["dolar", "dolar"], &["sen", "sen"]));
        forms.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["sen", "sen"]));
        LangBan { forms }
    }
}

/// Reproduce `str(abs(val)).split(".")` structurally, then Python's
/// `int(parts[0])` / `int(parts[1][:2].ljust(2, "0"))`.
///
/// The `BigDecimal` was parsed from `str(value)` on the Python side, so its
/// digits and scale *are* the digits and scale of the repr BAN re-stringifies.
/// Rebuilding the two integers from `(int_val, scale)` therefore reproduces
/// the split exactly for every value whose repr is positional — which is every
/// row in the corpus and every value in `1e-4 <= |x| < 1e16`.
///
/// Returns `(left, right)`, both non-negative. See the module docs for the
/// exponential-repr divergence this cannot express.
fn split_currency_parts(value: &bigdecimal::BigDecimal) -> Result<(BigInt, BigInt)> {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // `abs(val)` happens before `str(val)`, and abs leaves the scale alone.
    let int_val = int_val.abs();

    // scale < 0 means the source string carried a positive exponent — the
    // only way `BigDecimal::from_str` yields one. Python's `str()` keeps that
    // form for both floats (`str(1e16) == "1e+16"`) and Decimals
    // (`str(Decimal("1E+2")) == "1E+2"`), so `"1e+16".split(".")` is a
    // one-element list and `int("1e+16")` raises ValueError. Both source
    // types agree here, so the branch is unambiguous.
    if scale < 0 {
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            value
        )));
    }

    // No ".": `parts` has one element, so `len(parts) > 1` is false → right 0.
    if scale == 0 {
        return Ok((int_val, BigInt::zero()));
    }

    // The last `scale` digits are what repr prints after the "."; pad on the
    // left so there is always at least one integer digit ("0.01" → "001").
    let digits = int_val.to_string();
    let s = scale as usize;
    let padded = if digits.len() <= s {
        format!("{}{}", "0".repeat(s - digits.len() + 1), digits)
    } else {
        digits
    };
    let split = padded.len() - s;
    let left = BigInt::from_str(&padded[..split]).expect("digits only");

    // parts[1][:2].ljust(2, "0") — truncation, never rounding: 12.345 → 34.
    let mut two: String = padded[split..].chars().take(2).collect();
    while two.len() < 2 {
        two.push('0');
    }
    let right = BigInt::from_str(&two).expect("digits only");
    Ok((left, right))
}

// ---- the float / Decimal cardinal path -------------------------------
//
// BAN does **not** use `Num2Word_Base.float2tuple`. `to_cardinal` is an
// outright override whose first statement is `n = str(number).strip()`, and
// every later branch works on that *string*:
//
//     if "." in n:
//         left, right = n.split(".", 1)
//         ret = self._int_to_word(int(left)) + " " + self.pointword
//         for digit in right:
//             ret += " " + (self.ones[int(digit)] or "nol")
//         return ret.strip()
//
// So the usual float2tuple story does not apply here, and neither trap in the
// porting contract bites: there is no `abs(value - pre) * 10**precision`
// product to carry binary noise, and no `round()` to get banker's-vs-away
// wrong. `2.675` reaches BAN as the *repr* "2.675" and yields
// "dua koma nem pitu lima" straight off the digit characters — the same answer
// float2tuple's `< 0.01` rescue arrives at, but for an unrelated reason.
//
// What that buys in simplicity it spends on formatting: the port now has to
// reproduce `str()` itself, for both input types. Both are shortest-round-trip
// (float) or exact (Decimal) and switch to exponential notation at thresholds
// that are part of the observable behaviour — past them BAN raises ValueError
// rather than converting (see `python_float_repr` / `python_decimal_str`).
//
// `self.precision` is never read by any of this, so the `precision=` kwarg is
// inert for BAN. Verified in the live interpreter:
//     num2words(2.675, lang="ban", precision=1) == "dua koma nem pitu lima"
// The dispatcher still assigns `converter.precision` (base's `__init__` sets
// it to 2, so `hasattr` passes) — it is simply never consulted again.

/// Python's `int(s)`, over the token shapes `str(float)` / `str(Decimal)` can
/// actually produce.
///
/// Deliberately narrow: Python's `int()` also accepts underscore separators
/// and non-ASCII decimal digits (`int("٥") == 5`), but neither `repr(float)`
/// nor `Decimal.__str__` can emit them, so no reachable input distinguishes
/// this from the real thing.
///
/// The ValueError message is Python's verbatim, and reports the *original*
/// token rather than the stripped one — `int(" x ")` says `' x '`.
fn parse_int(token: &str) -> Result<BigInt> {
    let stripped = token.trim();
    let (negative, digits) = match stripped.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, stripped.strip_prefix('+').unwrap_or(stripped)),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            token
        )));
    }
    let value = BigInt::from_str(digits).expect("ascii digits only");
    Ok(if negative { -value } else { value })
}

/// CPython's `repr(float)` / `str(float)` — they are the same function.
///
/// `float_repr_style` is "short", so CPython calls `PyOS_double_to_string(v,
/// 'r', 0, Py_DTSF_ADD_DOT_0, NULL)`, which runs David Gay's `dtoa` in mode 0
/// (shortest string that round-trips) and then, in `format_float_short`:
///
/// ```c
/// case 'r':
///     /* convert to exponential format at 1e16. */
///     if (decpt <= -4 || decpt > 16)
///         use_exp = 1;
///     break;
/// ```
///
/// where `value == 0.d1d2... * 10**decpt`. Rust's `{:e}` for f64 is the same
/// shortest-round-trip contract and emits `d[.ddd]e<exp>` with no trailing
/// mantissa zeros, so `decpt` is just `exp + 1` and the digit string carries
/// straight over. Only the *notation* has to be re-decided here.
///
/// The thresholds are load-bearing rather than cosmetic: BAN feeds this string
/// to `int()`, so every value that goes exponential raises instead of
/// converting. `1e15` → "1000000000000000.0" → converts; `1e16` → "1e+16" →
/// ValueError. Likewise `1e-4` → "0.0001" converts, `1e-5` → "1e-05" raises.
///
/// `Py_DTSF_ADD_DOT_0` appends ".0" to an otherwise integral result, but only
/// on the non-exponential branch — hence "1000000000000000.0" yet plain
/// "1e+16".
fn python_float_repr(value: f64) -> String {
    // CPython special-cases these before dtoa ever runs, and lowercases them.
    // Rust would say "NaN"/"inf", and repr() drops the sign of a NaN.
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative`, not `< 0.0`: repr(-0.0) is "-0.0", and BAN keys its
    // negative branch off that leading "-" — so -0.0 really does come back as
    // "minus nol koma nol" where base.py's `value < 0` would have lost it.
    let sign = if value.is_sign_negative() { "-" } else { "" };

    let formatted = format!("{:e}", value.abs());
    let (mantissa, exponent) = formatted
        .split_once('e')
        .expect("f64 LowerExp always emits an 'e'");
    let exponent: i32 = exponent.parse().expect("LowerExp emits a decimal exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    // dtoa's decpt. `{:e}` normalises to one digit before the point, and zero
    // formats as "0e0" — which is exactly dtoa's ("0", decpt = 1), so
    // repr(0.0) lands on "0.0" via the integral branch below.
    let decpt = exponent + 1;

    if decpt <= -4 || decpt > 16 {
        let mut out = String::from(sign);
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        // sprintf(p, "%+.02d", decpt - 1): always signed, at least two digits,
        // and no truncation above that — "1e+16", "1e-05", "1e+100".
        out.push('e');
        let exp10 = decpt - 1;
        out.push(if exp10 < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (exp10 as i64).abs()));
        out
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt as usize >= digits.len() {
        // Integral: pad out to the decimal point, then ADD_DOT_0.
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat(decpt as usize - digits.len())
        )
    } else {
        format!(
            "{}{}.{}",
            sign,
            &digits[..decpt as usize],
            &digits[decpt as usize..]
        )
    }
}

/// CPython's `Decimal.__str__` (`_pydecimal.py`, mirrored by `_decimal`):
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # no exponent required
/// else:
///     dotplace = 1                   # usual scientific notation
/// if dotplace <= 0:
///     intpart, fracpart = '0', '.' + '0'*(-dotplace) + self._int
/// elif dotplace >= len(self._int):
///     intpart, fracpart = self._int + '0'*(dotplace-len(self._int)), ''
/// else:
///     intpart, fracpart = self._int[:dotplace], '.' + self._int[dotplace:]
/// exp = '' if leftdigits == dotplace else 'E' + "%+d" % (leftdigits-dotplace)
/// return sign + intpart + fracpart + exp
/// ```
///
/// `BigDecimal` stores the same `(coefficient, exponent)` pair a `Decimal`
/// does — that is the whole point of the type, and the shim hands the core
/// `str(value)` to parse — so the round trip is exact, trailing zeros and all:
/// `Decimal("1.10")` keeps its scale and prints "1.10", giving
/// "siki koma siki nol" rather than the "siki koma siki" a normalised 1.1
/// would.
///
/// Note the capital `E` and the *different* threshold from the float branch:
/// `Decimal("1E+2")` and `Decimal("1E-7")` stringify exponentially and so
/// raise, while the equal-valued `Decimal("100")` and `Decimal("0.0000001")`
/// … also differ, because Decimal's notation follows the written exponent
/// rather than the value. Both are Python's behaviour, not a rounding of it.
fn python_decimal_str(value: &BigDecimal) -> String {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    let sign = if coefficient.is_negative() { "-" } else { "" };
    // `self._int`: the coefficient's digits, unsigned. Decimal keeps no
    // leading zeros (zero itself is the single digit "0"), and neither does
    // BigInt's Display.
    let int_digits = coefficient.abs().to_string();
    // BigDecimal's `scale` is Decimal's `_exp` negated.
    let exp = -scale;
    let leftdigits = exp + int_digits.len() as i64;
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
    } else if dotplace >= int_digits.len() as i64 {
        (
            format!(
                "{}{}",
                int_digits,
                "0".repeat((dotplace - int_digits.len() as i64) as usize)
            ),
            String::new(),
        )
    } else {
        (
            int_digits[..dotplace as usize].to_string(),
            format!(".{}", &int_digits[dotplace as usize..]),
        )
    };
    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        // ['e', 'E'][context.capitals] — the default context has capitals=1.
        // "%+d" is signed but, unlike repr(float)'s "%+.02d", not zero-padded.
        format!("E{:+}", leftdigits - dotplace)
    };
    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

impl LangBan {
    /// `Num2Word_BAN.to_cardinal`, driven from the string as Python drives it.
    ///
    /// The integer-mode `to_cardinal` below inlines the same cascade over a
    /// `BigInt` (its `"." in n` branch is unreachable); this is the same method
    /// with the fractional branch live. Kept separate rather than merged
    /// because the phase contract freezes the integer path.
    ///
    /// Python recurses with a *string* (`self.to_cardinal(n[1:])`), which then
    /// re-enters `str(number)` as a no-op — so the recursion below is on `&str`
    /// too, not on a re-parsed number.
    fn cardinal_from_str(&self, number: &str) -> Result<String> {
        let n = number.trim();

        // if n.startswith("-"):
        //     return (self.negword + self.to_cardinal(n[1:])).strip()
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_str(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        // if "." in n: left, right = n.split(".", 1)
        if let Some((left, right)) = n.split_once('.') {
            // `int(left)` is what raises on "1.5e+16"'s sibling forms; here
            // left is "1" and it is `right` that blows up in the loop below.
            let mut ret = format!("{} {}", int_to_word(&parse_int(left)?)?, self.pointword());
            // for digit in right: ret += " " + (self.ones[int(digit)] or "nol")
            for digit in right.chars() {
                // int() of a single character: 0..=9 or ValueError. This is
                // the raise that catches exponential reprs whose mantissa has
                // a point — "1.5e+16" fails on 'e', not on the whole token.
                let index = parse_int(&digit.to_string())?
                    .to_usize()
                    .expect("a single ASCII digit is 0..=9");
                ret.push(' ');
                // `self.ones[0]` is "" — falsy, so `or "nol"` fires. That is
                // the only reason a leading zero survives: 0.01 → "nol siki".
                let word = ONES[index];
                ret.push_str(if word.is_empty() { "nol" } else { word });
            }
            return Ok(ret.trim().to_string());
        }

        // return self._int_to_word(int(n))
        Ok(int_to_word(&parse_int(n)?)?)
    }
}

impl Lang for LangBan {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }


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

    /// `to_ordinal(float/Decimal)`: `"ka-" + self.to_cardinal(number)`, the
    /// same prefix rule as the integer path — no verify_ordinal, so
    /// negatives keep their minus word ("ka-minus dua koma nol"). Errors
    /// from `to_cardinal` (e.g. `int('1e+16')` → ValueError) propagate.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("ka-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ka-" + str(number)` — the str() is
    /// Python's, handed in as `repr_str` ("ka--1.5", "ka-1e+16").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("ka-{}", repr_str))
    }

    /// `converter.str_to_number` is Base's `Decimal(value)`, but BAN's own
    /// `to_cardinal` then does `int(str(...))` on the no-dot form, so
    /// `"Infinity"` raises **ValueError** (`int('Infinity')`), not the
    /// OverflowError the shared Inf sentinel would produce. Raising here is
    /// observably identical: no digit is present, so the dispatcher
    /// propagates the ValueError rather than falling back to the sentence
    /// converter.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// `self.negword` — "minus " (set in `setup`). The integer/float
    /// cardinal paths use their own inlined constant, but the inherited
    /// `Num2Word_Base.to_fraction` reads `self.negword.strip()`, so the
    /// trait hook must expose the real word: `-3/4` → "minus telu ka-papats",
    /// not the base default "(-) ".
    fn negword(&self) -> &str {
        "minus "
    }

    /// `self.pointword`, read from the live Python instance.
    /// Unused by the four integer modes, so phase 1 never needed it —
    /// the float path is the first caller.
    fn pointword(&self) -> &str {
        "koma"
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "IDR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    /// Port of `Num2Word_BAN.to_cardinal`.
    ///
    /// Python works on `n = str(number).strip()`:
    /// ```text
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n: ...            # unreachable for integer input
    /// return self._int_to_word(int(n))
    /// ```
    /// The negative branch recurses with the *digits-only* string, which then
    /// takes the final `_int_to_word(int(n))` path — so it is exactly
    /// `int_to_word(abs(value))`, inlined here. The `"." in n` branch is the
    /// float path and cannot trigger on an integer.
    ///
    /// The trailing `.strip()` is kept for fidelity even though it is a no-op:
    /// `negword` leads, and `int_to_word` never emits edge whitespace.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(int_to_word(value)?)
    }

    /// Port of `Num2Word_BAN.to_ordinal`: `"ka-" + self.to_cardinal(number)`.
    ///
    /// No sign handling, so negatives yield "ka-minus siki" (see module docs).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ka-{}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_BAN.to_ordinal_num`: `"ka-" + str(number)`.
    ///
    /// Overrides the base (which returns the value untouched). `-1` renders as
    /// "ka--1"; the double hyphen is Python's and is preserved.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ka-{}", value))
    }

    /// Port of `Num2Word_BAN.to_year`: `return self.to_cardinal(val)`.
    ///
    /// BAN ignores its own `longval=True` kwarg, so there is no century
    /// splitting: 1984 is "siki siu sia satus kutus dasa papat", not
    /// "nineteen eighty-four"-style. Negative years keep the minus word
    /// (`-500` → "minus lima satus") rather than gaining a "BC" suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal arm of `Num2Word_BAN.to_cardinal`.
    ///
    /// BAN does not override `to_cardinal_float` — it overrides `to_cardinal`
    /// and handles non-integers inline — so `Num2Word_Base.to_cardinal_float`
    /// and `float2tuple` never run for this language and the default
    /// implementation this replaces would have been wrong. Everything
    /// observable comes from `str(number)`; see the notes above
    /// `python_float_repr`.
    ///
    /// `precision_override` is ignored, and that is the port, not a shortcut.
    /// `Num2Word_BAN.to_cardinal(self, number)` takes no `precision` argument,
    /// so `num2words`' `kwargs.pop("precision", None)` removes it before the
    /// call and only assigns `converter.precision`, which nothing here reads:
    ///     num2words(2.675, lang="ban", precision=1) == "dua koma nem pitu lima"
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() — the whole method, in one line. The two
        // arms are not interchangeable: `str()` of a float is shortest
        // round-trip and goes exponential outside [1e-4, 1e16), while `str()`
        // of a Decimal is exact and goes exponential on its own written
        // exponent.
        let n = match value {
            FloatValue::Float { value, .. } => python_float_repr(*value),
            FloatValue::Decimal { value, .. } => python_decimal_str(value),
        };
        self.cardinal_from_str(&n)
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_BAN"
    }

    /// `CURRENCY_FORMS[code]` — a *strict* lookup, deliberately.
    ///
    /// The lenient `.get(currency, <IDR>)` fallback lives in `to_currency`
    /// alone, because only `to_currency` has it. Inherited `to_cheque` does
    /// `self.CURRENCY_FORMS[currency]` and converts the KeyError into
    /// NotImplementedError, which is why `cheque:GBP` raises while
    /// `currency:GBP` quietly prints rupiah. Returning the fallback here would
    /// make every cheque row succeed and lose seven corpus errors.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Port of `Num2Word_BAN.pluralize`:
    /// ```text
    /// if not forms: return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    /// Dead for both modes in scope — `to_currency` is overridden and picks
    /// forms inline, and `to_cheque` takes `cr1[-1]` unconditionally — but it
    /// is a real override, so it is ported rather than left raising.
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

    /// Port of `Num2Word_BAN.to_currency` — a full override; **none** of
    /// `Num2Word_Base.to_currency` runs.
    ///
    /// ```text
    /// is_negative = val < 0
    /// val = abs(val)
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    /// result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// Consequences worth spelling out, all corpus-confirmed:
    ///
    /// * **`CURRENCY_PRECISION` is never consulted.** BAN hardcodes two
    ///   decimals. The divisor-1 pre-rounding and the 1000-divisor mil
    ///   handling both live in `Num2Word_Base.to_currency`, which is bypassed,
    ///   so `currency:JPY 12.34` still prints cents ("dasa dua rupiah tigang
    ///   dasa papat sen") and KWD/BHD are ordinary 100ths. `currency_precision`
    ///   is left at the trait default only because BAN's `CURRENCY_PRECISION`
    ///   is `{}` and inherited `to_cheque` reads it (→ 100).
    /// * **An unknown code does not raise.** It silently renders as IDR.
    /// * **`adjective` is accepted and ignored** — BAN defines no
    ///   `CURRENCY_ADJECTIVES` and never calls `prefix_currency`.
    /// * **`cents=False` drops the segment entirely** rather than falling back
    ///   to the terse digit form the base class uses.
    /// * **The int/float split is invisible here.** BAN routes both through
    ///   `str(val)`, and `1.0` → `parts[1] == "0"` → `right == 0` → falsy → no
    ///   cents segment, landing on the same "siki euro" a true `1` gives. The
    ///   distinction is still honoured (an int has no `parts[1]` at all); it
    ///   just cannot be observed in the output.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // `Num2Word_BAN.to_currency` defaults `separator=" "`, but the Python
        // shim (`__init__.py`) forwards `kwargs.get("separator", ",")` and the
        // diff harness hardcodes ",". Both destroy the "caller said nothing"
        // signal before it reaches here, so an incoming "," is indistinguish-
        // able from an omitted kwarg and is treated as one. The corpus was
        // generated through BAN's own default and expects " ":
        //   currency:EUR 12.34 → "dasa dua euro tigang dasa papat sen"
        // Any other separator is honoured verbatim, so only an *explicit*
        // separator="," diverges — the one value that is already ambiguous.
        // The real fix is an Option<&str> across the boundary; see `concerns`.
        let separator = if separator == "," { " " } else { separator };

        let is_negative = val.is_negative();
        let (left, right) = match val {
            // str(int) never contains a ".", so parts[1] does not exist.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => split_currency_parts(d)?,
        };

        // .get(currency, list(values())[0]) — the fallback is always IDR.
        let forms = self
            .forms
            .get(currency)
            .or_else(|| self.forms.get(FALLBACK_CURRENCY))
            .expect("IDR is always present");
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // cr1[1] if left != 1 else cr1[0] — a literal index, as in Python.
        let unit = if left.is_one() { &cr1[0] } else { &cr1[1] };
        let mut result = format!("{} {}", int_to_word(&left)?, unit);

        if cents && !right.is_zero() {
            let sub = if right.is_one() { &cr2[0] } else { &cr2[1] };
            result.push_str(separator);
            result.push_str(&int_to_word(&right)?);
            result.push(' ');
            result.push_str(sub);
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        // .strip() — a no-op for every reachable input, kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn card(n: i64) -> String {
        LangBan::new().to_cardinal(&BigInt::from(n)).unwrap()
    }
    fn ord_(n: i64) -> String {
        LangBan::new().to_ordinal(&BigInt::from(n)).unwrap()
    }
    fn ord_num(n: i64) -> String {
        LangBan::new().to_ordinal_num(&BigInt::from(n)).unwrap()
    }
    fn year(n: i64) -> String {
        LangBan::new().to_year(&BigInt::from(n)).unwrap()
    }

    /// Every expectation below is copied verbatim from the frozen corpus
    /// (`grep '"lang": "ban"' bench/corpus.jsonl`).
    #[test]
    fn corpus_cardinal() {
        assert_eq!(card(0), "nol");
        assert_eq!(card(1), "siki");
        assert_eq!(card(9), "sia");
        assert_eq!(card(10), "dasa");
        assert_eq!(card(11), "dasa siki");
        assert_eq!(card(20), "kalih dasa");
        assert_eq!(card(50), "seket");
        assert_eq!(card(99), "siangang dasa sia");
        assert_eq!(card(100), "satus");
        assert_eq!(card(101), "satus siki");
        assert_eq!(card(110), "satus dasa");
        assert_eq!(card(200), "dua satus");
        assert_eq!(card(999), "sia satus siangang dasa sia");
        assert_eq!(card(1000), "siki siu");
        assert_eq!(card(1010), "siki siu dasa");
        assert_eq!(card(12345), "dasa dua siu telu satus petang dasa lima");
        assert_eq!(card(100000), "satus siu");
        assert_eq!(card(1000000), "siki yuta");
        assert_eq!(
            card(1234567),
            "siki yuta dua satus tigang dasa papat siu lima satus nem dasa pitu"
        );
        assert_eq!(
            card(123456789),
            "satus kalih dasa telu yuta papat satus seket nem siu pitu satus kutus dasa sia"
        );
    }

    /// The 10^9 cliff: Python fell through to `str(number)`; the port
    /// raises OverflowError at its maxval instead (#147).
    #[test]
    fn cardinal_from_billion_overflows() {
        assert_eq!(
            card(999_999_999),
            "sia satus siangang dasa sia yuta sia satus siangang dasa sia siu \
             sia satus siangang dasa sia"
        );
        let ban = LangBan::new();
        assert_eq!(ban.maxval(), &BigInt::from(1_000_000_000));
        for v in [BigInt::from(1_000_000_000), BigInt::from(-1_234_567_890), BigInt::from(10).pow(21u32)] {
            assert!(matches!(ban.to_cardinal(&v), Err(N2WError::Overflow(_))));
        }
    }

    #[test]
    fn corpus_negative() {
        assert_eq!(card(-1), "minus siki");
        assert_eq!(card(-21), "minus kalih dasa siki");
        assert_eq!(card(-100), "minus satus");
        assert_eq!(card(-1000), "minus siki siu");
        assert_eq!(card(-1000000), "minus siki yuta");
    }

    #[test]
    fn corpus_ordinal() {
        assert_eq!(ord_(0), "ka-nol");
        assert_eq!(ord_(1), "ka-siki");
        assert_eq!(ord_(11), "ka-dasa siki");
        assert_eq!(ord_(1000000), "ka-siki yuta");
        // Negative keeps the minus word inline; >=10^9 overflows (#147).
        assert_eq!(ord_(-1), "ka-minus siki");
        assert!(matches!(
            LangBan::new().to_ordinal(&BigInt::from(1_000_000_000)),
            Err(N2WError::Overflow(_))
        ));
    }

    #[test]
    fn corpus_ordinal_num() {
        assert_eq!(ord_num(0), "ka-0");
        assert_eq!(ord_num(9), "ka-9");
        assert_eq!(ord_num(1000000), "ka-1000000");
        // Double hyphen, straight from Python's "ka-" + str(-1).
        assert_eq!(ord_num(-1), "ka--1");
        assert_eq!(ord_num(-1000), "ka--1000");
    }

    // ---- currency ----------------------------------------------------

    use bigdecimal::BigDecimal;

    /// The corpus drives every row with cents=true / adjective=false and no
    /// separator= kwarg, so pass `None` exactly as `diff_test.py` does and let
    /// BAN's own `default_separator()` (" ") apply.
    ///
    /// `has_decimal` mirrors the harness: it is `not is_int`, since the
    /// generator wrote `repr(value)` and only a float/Decimal grows a dot.
    fn cur(arg: &str, is_int: bool, code: &str) -> String {
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangBan::new().to_currency(&v, code, true, None, false).unwrap()
    }
    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangBan::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    /// Verbatim from the frozen corpus. `arg` is `repr(value)`, so "1" is an
    /// int and "1.0" a float — `is_int` mirrors `type(number) is int`.
    #[test]
    fn corpus_currency_known_codes() {
        assert_eq!(cur("0", true, "EUR"), "nol euro");
        assert_eq!(cur("1", true, "EUR"), "siki euro");
        assert_eq!(cur("2", true, "EUR"), "dua euro");
        assert_eq!(cur("100", true, "EUR"), "satus euro");
        assert_eq!(cur("1000000", true, "EUR"), "siki yuta euro");
        assert_eq!(cur("12.34", false, "EUR"), "dasa dua euro tigang dasa papat sen");
        assert_eq!(cur("0.01", false, "EUR"), "nol euro siki sen");
        assert_eq!(cur("0.5", false, "EUR"), "nol euro seket sen");
        assert_eq!(
            cur("99.99", false, "EUR"),
            "siangang dasa sia euro siangang dasa sia sen"
        );
        assert_eq!(
            cur("1234.56", false, "EUR"),
            "siki siu dua satus tigang dasa papat euro seket nem sen"
        );
        assert_eq!(cur("-12.34", false, "EUR"), "minus dasa dua euro tigang dasa papat sen");
        assert_eq!(cur("12.34", false, "USD"), "dasa dua dolar tigang dasa papat sen");
        assert_eq!(cur("1", true, "USD"), "siki dolar");
    }

    /// `1.0` is a float, so Python reaches `parts[1] == "0"` → right == 0 →
    /// falsy → no cents segment. Same text as the int `1`, by a different path.
    #[test]
    fn corpus_currency_float_with_zero_cents() {
        assert_eq!(cur("1.0", false, "EUR"), "siki euro");
        assert_eq!(cur("1.0", false, "USD"), "siki dolar");
        assert_eq!(cur("1.0", false, "GBP"), "siki rupiah");
        assert_eq!(cur("1", true, "EUR"), "siki euro");
    }

    /// Unknown codes silently render as IDR — `.get(currency, values()[0])`.
    /// Note JPY still shows cents and KWD/BHD are plain 100ths: BAN never
    /// reads CURRENCY_PRECISION.
    #[test]
    fn corpus_currency_unknown_code_falls_back_to_idr() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert_eq!(cur("0", true, code), "nol rupiah");
            assert_eq!(cur("1", true, code), "siki rupiah");
            assert_eq!(cur("100", true, code), "satus rupiah");
            assert_eq!(cur("12.34", false, code), "dasa dua rupiah tigang dasa papat sen");
            assert_eq!(cur("0.01", false, code), "nol rupiah siki sen");
            assert_eq!(cur("0.5", false, code), "nol rupiah seket sen");
            assert_eq!(cur("1000000", true, code), "siki yuta rupiah");
            assert_eq!(
                cur("1234.56", false, code),
                "siki siu dua satus tigang dasa papat rupiah seket nem sen"
            );
            assert_eq!(
                cur("-12.34", false, code),
                "minus dasa dua rupiah tigang dasa papat sen"
            );
        }
    }

    /// Not corpus rows — verified directly against the live interpreter
    /// (`Num2Word_BAN.to_currency(...)`), since the shim cannot express them.
    #[test]
    fn currency_quirks_verified_against_python() {
        let one = LangBan::new();
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        // cents=False drops the segment; it does NOT fall back to terse digits.
        assert_eq!(one.to_currency(&v, "EUR", false, None, false).unwrap(), "dasa dua euro");
        // adjective is accepted and ignored (CURRENCY_ADJECTIVES is empty).
        assert_eq!(
            one.to_currency(&v, "EUR", true, None, true).unwrap(),
            "dasa dua euro tigang dasa papat sen"
        );
        // A non-default separator is honoured verbatim.
        assert_eq!(
            one.to_currency(&v, "EUR", true, Some(" ne "), false).unwrap(),
            "dasa dua euro ne tigang dasa papat sen"
        );
        // parts[1][:2] truncates, never rounds: 12.345 → 34, not 35.
        let v = CurrencyValue::parse("12.345", false, true, true).unwrap();
        assert_eq!(
            one.to_currency(&v, "EUR", true, None, false).unwrap(),
            "dasa dua euro tigang dasa papat sen"
        );
    }

    /// `str(1e16) == "1e+16"` → `int("1e+16")` → ValueError. The parsed
    /// BigDecimal keeps the negative scale, so the notation is recoverable.
    /// Guards the assumption that from_str does not normalise it away.
    #[test]
    fn currency_exponential_repr_is_valueerror() {
        for arg in ["1e+16", "1E+2"] {
            let v = CurrencyValue::parse(arg, false, true, true).unwrap();
            match LangBan::new().to_currency(&v, "EUR", true, None, false) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{} expected ValueError, got {:?}", arg, other),
            }
        }
        // Documented divergence: Python raises for the float 1e-05 but not for
        // Decimal("0.00001"), and both parse to the same BigDecimal. We match
        // the Decimal branch. See the module docs.
        assert_eq!(cur("1e-05", false, "EUR"), "nol euro");
    }

    /// Inherited `Num2Word_Base.to_cheque`; BAN adds no override. The plural
    /// unit is `cr1[-1]`, and BAN's forms are identical singular/plural.
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "SIKI SIU DUA SATUS TIGANG DASA PAPAT AND 56/100 EURO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "SIKI SIU DUA SATUS TIGANG DASA PAPAT AND 56/100 DOLAR"
        );
        // CURRENCY_FORMS[currency] raises KeyError → NotImplementedError.
        // Unlike to_currency, there is no IDR fallback here.
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_BAN\"", code)
                ),
                other => panic!("{} expected NotImplementedError, got {:?}", code, other),
            }
        }
    }

    // ---- float / Decimal ---------------------------------------------

    /// Drive a float exactly as the shim does: the raw f64 plus the
    /// repr-derived precision (which BAN ignores, but the boundary carries).
    fn flt(value: f64) -> Result<String> {
        let precision = match format!("{}", value).split_once('.') {
            Some((_, frac)) => frac.len() as u32,
            None => 0,
        };
        LangBan::new().to_cardinal_float(&FloatValue::Float { value, precision }, None)
    }
    fn fltok(value: f64) -> String {
        flt(value).unwrap()
    }
    /// A Decimal row: the shim passes `str(value)` for the core to re-parse.
    fn dec(s: &str) -> Result<String> {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        LangBan::new().to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
    }
    fn decok(s: &str) -> String {
        dec(s).unwrap()
    }

    /// Every expectation verbatim from the frozen corpus
    /// (`grep '"lang": "ban", "to": "cardinal"' bench/corpus.jsonl`, dotted args).
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(fltok(0.0), "nol koma nol");
        assert_eq!(fltok(0.5), "nol koma lima");
        assert_eq!(fltok(1.0), "siki koma nol");
        assert_eq!(fltok(1.5), "siki koma lima");
        assert_eq!(fltok(2.25), "dua koma dua lima");
        assert_eq!(fltok(3.14), "telu koma siki papat");
        assert_eq!(fltok(0.01), "nol koma nol siki");
        assert_eq!(fltok(0.1), "nol koma siki");
        assert_eq!(fltok(0.99), "nol koma sia sia");
        assert_eq!(fltok(1.01), "siki koma nol siki");
        assert_eq!(fltok(12.34), "dasa dua koma telu papat");
        assert_eq!(fltok(99.99), "siangang dasa sia koma sia sia");
        assert_eq!(fltok(100.5), "satus koma lima");
        assert_eq!(
            fltok(1234.56),
            "siki siu dua satus tigang dasa papat koma lima nem"
        );
        assert_eq!(fltok(-0.5), "minus nol koma lima");
        assert_eq!(fltok(-1.5), "minus siki koma lima");
        assert_eq!(fltok(-12.34), "minus dasa dua koma telu papat");
        assert_eq!(fltok(1.005), "siki koma nol nol lima");
        assert_eq!(fltok(2.675), "dua koma nem pitu lima");
    }

    /// Verbatim from the `cardinal_dec` corpus rows.
    #[test]
    fn corpus_cardinal_decimal() {
        assert_eq!(decok("0.01"), "nol koma nol siki");
        // Trailing zero survives: Decimal("1.10") is not Decimal("1.1").
        assert_eq!(decok("1.10"), "siki koma siki nol");
        assert_eq!(decok("12.345"), "dasa dua koma telu papat lima");
        // Issue #603's value. The integer part is past the 10^9 ceiling, so
        // it raises OverflowError (#147; Python echoed the digits).
        assert!(matches!(dec("98746251323029.99"), Err(N2WError::Overflow(_))));
        assert_eq!(decok("0.001"), "nol koma nol nol siki");
    }

    /// The two f64-artefact cases the contract calls out. BAN never computes
    /// `abs(value - pre) * 10**precision`, so 2.675's 674.9999999999998 is
    /// never produced and never needs rescuing — the repr digits are read
    /// straight off. Same answer, different route.
    #[test]
    fn float_artefacts_never_arise() {
        assert_eq!(fltok(2.675), "dua koma nem pitu lima");
        assert_eq!(fltok(1.005), "siki koma nol nol lima");
        // repr is shortest-round-trip, so no 0.1+0.2 tail leaks in either.
        assert_eq!(fltok(0.1 + 0.2), "nol koma telu nol nol nol nol nol nol nol nol nol nol nol nol nol nol nol papat");
    }

    /// `precision=` is inert for BAN: its `to_cardinal` takes no such kwarg.
    #[test]
    fn precision_override_is_ignored() {
        let value = FloatValue::Float { value: 2.675, precision: 3 };
        for override_ in [None, Some(1), Some(5)] {
            assert_eq!(
                LangBan::new().to_cardinal_float(&value, override_).unwrap(),
                "dua koma nem pitu lima"
            );
        }
    }

    /// repr(-0.0) is "-0.0", and BAN's negative branch keys off the string —
    /// so the sign survives where base.py's `value < 0` would have dropped it.
    #[test]
    fn negative_zero_keeps_its_sign() {
        assert_eq!(fltok(-0.0), "minus nol koma nol");
        assert_eq!(fltok(0.0), "nol koma nol");
    }

    /// Documented divergence — see the module docs. Python says
    /// "minus nol koma nol" / "minus nol" for these, because `str(Decimal)`
    /// keeps the sign of a zero. `BigDecimal` is backed by `BigInt`, which has
    /// no negative zero, so `from_str("-0.0").sign()` is already `NoSign` by
    /// the time the core is called and the minus is unrecoverable here.
    /// Pinned rather than left silent; no corpus row reaches it.
    #[test]
    fn negative_zero_decimal_loses_its_sign() {
        assert_eq!(BigDecimal::from_str("-0.0").unwrap().sign(), num_bigint::Sign::NoSign);
        assert_eq!(decok("-0.0"), "nol koma nol");
        assert_eq!(decok("-0"), "nol");
        // A non-zero coefficient has somewhere to keep the sign, so it works.
        assert_eq!(decok("-0.5"), "minus nol koma lima");
    }

    /// The 10^9 ceiling applies to the integer part of a float too: it is
    /// `_int_to_word` that raises (#147; Python echoed the digits).
    #[test]
    fn float_above_billion_overflows() {
        assert_eq!(
            fltok(123456789.5),
            "satus kalih dasa telu yuta papat satus seket nem siu pitu satus \
             kutus dasa sia koma lima"
        );
        assert!(matches!(flt(1234567890.5), Err(N2WError::Overflow(_))));
        // decpt == 16 is still positional, so this reaches the ceiling rather
        // than the ValueError of the exponent forms.
        assert!(matches!(flt(1e15), Err(N2WError::Overflow(_))));
    }

    /// Where `repr(float)` goes exponential, BAN feeds "e" to `int()` and
    /// crashes. Two distinct messages, because two distinct `int()` calls fail:
    /// the whole token when the repr has no ".", a lone character when it does.
    #[test]
    fn exponential_float_repr_is_valueerror() {
        for (value, token) in [
            (1e16, "1e+16"),
            (1e21, "1e+21"),
            (1e100, "1e+100"),
            (1e-5, "1e-05"),
            (1e-323, "1e-323"),
        ] {
            match flt(value) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", token)
                ),
                other => panic!("{} expected ValueError, got {:?}", value, other),
            }
        }
        // A mantissa with a point splits, so the loop reaches 'e' first.
        for value in [1.5e16, 1.5e-5, 2e16 + 8.0] {
            match flt(value) {
                Err(N2WError::Value(m)) => {
                    assert_eq!(m, "invalid literal for int() with base 10: 'e'")
                }
                other => panic!("{} expected ValueError, got {:?}", value, other),
            }
        }
        // 1e-4 is the last positional one on the small side.
        assert_eq!(fltok(1e-4), "nol koma nol nol nol siki");
    }

    /// str(inf)/str(nan) are plain words with no ".", so they take the same
    /// `int(n)` raise. -inf loses its sign in the message: the negword branch
    /// strips "-" and recurses with "inf".
    #[test]
    fn non_finite_floats_are_valueerror() {
        for (value, token) in [
            (f64::INFINITY, "inf"),
            (f64::NEG_INFINITY, "inf"),
            (f64::NAN, "nan"),
        ] {
            match flt(value) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", token)
                ),
                other => panic!("{} expected ValueError, got {:?}", value, other),
            }
        }
    }

    /// Decimal's notation follows its *written* exponent, not its value, so
    /// these raise while numerically equal literals convert.
    #[test]
    fn exponential_decimal_str_is_valueerror() {
        for (arg, token) in [
            ("1E+2", "1E+2"),
            ("1E-7", "1E-7"),
            // Positional as written, but `leftdigits > -6` fails at 1e-7, so
            // Decimal.__str__ still goes exponential and this still raises.
            ("0.0000001", "1E-7"),
        ] {
            match dec(arg) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", token)
                ),
                other => panic!("{} expected ValueError, got {:?}", arg, other),
            }
        }
        // "1.10E+3" splits on the point, so the loop dies on the capital 'E'.
        match dec("1.10E+3") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'E'")
            }
            other => panic!("1.10E+3 expected ValueError, got {:?}", other),
        }
        // The same values written positionally convert happily; 0.000001 is
        // the last one on the small side (leftdigits == -5).
        assert_eq!(decok("100"), "satus");
        assert_eq!(decok("0.000001"), "nol koma nol nol nol nol nol siki");
    }

    /// Decimal keeps scale, so these are all distinguishable.
    #[test]
    fn decimal_scale_is_observable() {
        assert_eq!(decok("5"), "lima");
        assert_eq!(decok("5.00"), "lima koma nol nol");
        assert_eq!(decok("0.000"), "nol koma nol nol nol");
        assert_eq!(decok("-0.5"), "minus nol koma lima");
    }

    /// Guards the two `str()` reimplementations directly, against
    /// `repr(v)` / `str(Decimal(s))` read out of the live interpreter.
    #[test]
    fn str_reimplementations_match_python() {
        for (value, expected) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (0.5, "0.5"),
            (1.0, "1.0"),
            (2.675, "2.675"),
            (1.005, "1.005"),
            (0.1, "0.1"),
            (1e-4, "0.0001"),
            (1e-5, "1e-05"),
            (1.5e-5, "1.5e-05"),
            (1e15, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1.5e16, "1.5e+16"),
            (2e16 + 8.0, "2.000000000000001e+16"),
            (1e21, "1e+21"),
            (1e100, "1e+100"),
            (1e-323, "1e-323"),
            (98746251323029.99, "98746251323029.98"),
            (123456789.5, "123456789.5"),
            (100.0, "100.0"),
            (-12.34, "-12.34"),
        ] {
            assert_eq!(python_float_repr(value), expected, "repr({})", value);
        }
        for (arg, expected) in [
            ("0.01", "0.01"),
            ("1.10", "1.10"),
            ("5", "5"),
            ("5.00", "5.00"),
            ("0.000", "0.000"),
            ("98746251323029.99", "98746251323029.99"),
            ("1E+2", "1E+2"),
            ("1E-7", "1E-7"),
            ("1.10E+3", "1.10E+3"),
            ("0.0000001", "1E-7"),
            ("-0.5", "-0.5"),
        ] {
            assert_eq!(
                python_decimal_str(&BigDecimal::from_str(arg).unwrap()),
                expected,
                "str(Decimal({:?}))",
                arg
            );
        }
    }

    #[test]
    fn corpus_year() {
        assert_eq!(year(1), "siki");
        assert_eq!(year(100), "satus");
        assert_eq!(year(1492), "siki siu papat satus siangang dasa dua");
        assert_eq!(year(1776), "siki siu pitu satus pitung dasa nem");
        assert_eq!(year(2024), "dua siu kalih dasa papat");
        assert_eq!(year(9999), "sia siu sia satus siangang dasa sia");
        assert_eq!(year(-500), "minus lima satus");
        assert_eq!(year(-44), "minus petang dasa papat");
    }
}
