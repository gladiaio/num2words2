//! Port of `lang_LG.py` (Luganda).
//!
//! Shape: **self-contained**. `Num2Word_LG` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords` and never
//! calls `set_high_numwords`, so Python builds neither `self.cards` nor
//! `MAXVAL`. `to_cardinal` is overridden outright and delegates to a hand-rolled
//! `_int_to_word` ladder. Consequently `cards`/`maxval`/`merge` stay at their
//! trait defaults here, and **no code path can ever raise** — see "Error
//! variants" below.
//!
//! `Num2Word_Base.__init__` sets `self.is_title = False` and LG's `setup()`
//! never touches it, so `title()` is a no-op; in any case LG's `to_cardinal`
//! bypasses the base entirely and never calls `title()`. The
//! `exclude_title` list LG populates (`["ne", "akatonnyeze", "wansi", "wa"]`)
//! is therefore dead weight in Python and is not modelled here.
//!
//! All four in-scope modes are overridden by LG itself; nothing is inherited:
//!   * `to_cardinal`    — the string-driven algorithm below
//!   * `to_ordinal`     — `"ow' " + to_cardinal(n)`
//!   * `to_ordinal_num` — `"ow' " + str(n)`
//!   * `to_year(val, longval=True)` — plain `to_cardinal(val)`; the `longval`
//!     flag is accepted and ignored, so years get no special treatment
//!     (`to_year(1492)` == `to_cardinal(1492)` == "lukumi mu nnya kikumi mu
//!     kyenda mu bbiri", not a "fourteen ninety-two" style split).
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. Both of the following look wrong and are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the bare digits.** The
//!    ladder stops after the `number < 1000000` (thousand) and
//!    `number < 1000000000` (million) branches; the final statement is a plain
//!    `return str(number)`. So `to_cardinal(10**9)` == "1000000000" — an
//!    unconverted numeral, not words, and *not* an `OverflowError`. Corpus rows
//!    confirm this up to 10^21. By extension `to_ordinal(10**9)` ==
//!    "ow' 1000000000". Modelled in [`int_to_word`]. Note this is a silent
//!    wrong-looking answer rather than a raise, so callers get no signal.
//! 2. **The multiplier "emu" (one) is suppressed on every scale word**, because
//!    each branch guards the multiplier with `if h > 1` / `if t > 1` /
//!    `if m > 1` rather than `!= 0`. Hence 100 → "kikumi" (not "emu kikumi"),
//!    1000 → "lukumi", 10^6 → "kakadde". That is idiomatic for Luganda and
//!    almost certainly intentional, but it is worth stating explicitly since
//!    the same guard would be a bug in most European languages.
//!
//! # Error variants
//!
//! None. Unlike most modules, LG has no reachable crash site in the four
//! in-scope modes: there is no table lookup that can miss, no `int()` of a
//! non-numeric token, and no overflow check (the 10^9 ceiling degrades to
//! digits per quirk 1 instead of raising). Every in-scope call returns `Ok`.
//!
//! # Unreachable Python bug, deliberately not modelled
//!
//! `_int_to_word(-1)` would fall through `number == 0` into the `number < 10`
//! branch and evaluate `self.ones[-1]`, which Python resolves as a *negative
//! index* — yielding "mwenda" (nine) for minus one. It is unreachable from all
//! four in-scope modes because `to_cardinal` strips the sign at the **string**
//! level (`n.startswith("-")` → recurse on `n[1:]`) before any `int()` is
//! taken, so `_int_to_word` only ever sees non-negative values. [`int_to_word`]
//! documents and relies on that invariant rather than reproducing the quirk.
//!
//! # Currency
//!
//! LG owns `to_currency` and `pluralize`; everything else on the currency
//! surface (`to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse`)
//! is `Num2Word_Base`'s, which the trait defaults already mirror. Both
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are `{}` — never populated,
//! never rebound — so `currency_adjective()` stays `None` and
//! `currency_precision()` stays 100 for *every* code. LG therefore has no
//! 3-decimal (KWD/BHD) or 0-decimal (JPY) currency: those codes are simply
//! absent from the table and take the fallback described below. Corpus rows for
//! `currency:KWD` / `currency:JPY` confirm no divisor ever varies.
//!
//! The two halves diverge sharply on an unknown code, and that is not a typo:
//!
//! * `to_currency` does `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`
//!   — a `dict.get` **with a default**, so an unknown code silently renders as
//!   the first entry (UGX, "shilingi") instead of raising. Hence
//!   `to_currency(1, "GBP")` == "emu shilingi", and every unimplemented code
//!   quietly reports Ugandan shillings.
//! * `to_cheque` is Base's, which does `CURRENCY_FORMS[currency]` and converts
//!   the `KeyError` into `NotImplementedError`. So `cheque:GBP` *does* raise.
//!
//! Same missing code, two different answers. [`LangLg::currency_forms`] is the
//! strict lookup (feeding `to_cheque`); the fallback lives inside
//! [`LangLg::to_currency`] where Python puts it.
//!
//! ## `str(val)` is the spec, and it leaks `repr(float)`
//!
//! `to_currency` never touches `parse_currency_parts`. It splits the *text* of
//! the value:
//!
//! ```python
//! parts = str(val).split(".")
//! left  = int(parts[0]) if parts[0] else 0
//! right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
//! ```
//!
//! Arithmetic (truncate, then scale the remainder by 100) agrees with that on
//! every value `repr` spells in fixed notation — but not otherwise, and the
//! disagreement is spectacular. Once `repr` switches to exponential the slicing
//! keeps going and reads the *exponent's* digits as cents:
//!
//! | value | `str(val)` | Python's answer |
//! |---|---|---|
//! | `1e16` | `"1e+16"` | ValueError — `int("1e+16")` |
//! | `1e21` | `"1e+21"` | ValueError |
//! | `1.5e20` | `"1.5e+20"` | ValueError — `int("5e")` |
//! | `1.25e20` | `"1.25e+20"` | **"emu yuro abiri mu ttaano senti"** |
//!
//! That last row is not a rounding artefact: 1.25e20 euros comes back as *one
//! euro twenty-five cents*, because `parts[1][:2]` of `"25e+20"` is `"25"` and
//! `int("25")` happens to succeed. All four verified against the live
//! interpreter. Reproducing them requires rebuilding the string Python saw, so
//! [`py_str`] does exactly that and the rest is a literal transcription of the
//! slicing. See [`py_str`] for what is and is not recoverable.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.ones`. Index 0 is the empty string; `_int_to_word` never reads it
/// (zero is caught by the `number == 0` branch first). Index 0 *is* read on the
/// float/Decimal fraction path via [`ones_or_nuli`], where Python's
/// `self.ones[int(digit)] or "nuli"` turns that empty string into "nuli".
const ONES: [&str; 10] = [
    "", "emu", "bbiri", "ssatu", "nnya", "ttaano", "mukaaga", "musanvu", "munaana", "mwenda",
];

/// `self.tens`. Index 1 ("kkumi") is the standalone word for ten.
const TENS: [&str; 10] = [
    "", "kkumi", "abiri", "asatu", "ana", "ataano", "nkaaga", "nsanvu", "kinaana", "kyenda",
];

const ZERO_WORD: &str = "nuli";
const HUNDRED: &str = "kikumi";
const THOUSAND: &str = "lukumi";
const MILLION: &str = "kakadde";

/// `self.pointword`. Mirrors [`LangLg::pointword`] (same string); duplicated as
/// a `const` only so the free-function float path can name it without a `self`.
/// LG's `to_cardinal` splices it verbatim (no `title()`), so the raw form is
/// what appears in output.
const POINTWORD: &str = "akatonnyeze";

/// `self.negword`. Keeps its trailing space: Python concatenates it directly
/// (`self.negword + self.to_cardinal(...)`) and only strips the *outer* ends.
const NEGWORD: &str = "wansi wa ";

/// The joiner Python splices between every scale and its remainder.
const JOIN: &str = " mu ";

/// Python's `_int_to_word`, entry point.
///
/// # Invariant
///
/// `number` must be non-negative. `to_cardinal` guarantees this by stripping
/// the minus sign from the *string* form before calling `int()`, so the
/// negative-index quirk described in the module docs is structurally
/// unreachable. Nothing else in this module calls into here.
fn int_to_word(number: &BigInt) -> String {
    // Python checks `number == 0` first; zero is below every bound that
    // follows, so hoisting the check above the 10^9 test is equivalent.
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Python's ladder ends with a bare `return str(number)` for anything the
    // million branch does not cover. Reproduced verbatim: digits, not words,
    // and not an OverflowError. See module docs, quirk 1.
    let billion = BigInt::from(1_000_000_000u32);
    if number >= &billion {
        return number.to_string();
    }

    // Proven bounded: 0 < number < 10^9, so u64 is safe here. The BigInt input
    // is only load-bearing above, where oversized values short-circuit to
    // their decimal form.
    let n = number
        .to_u64()
        .expect("0 < number < 10^9 was just established");
    int_to_word_small(n)
}

/// The word-producing half of `_int_to_word`, for `0 <= n < 10^9`.
///
/// Split out from [`int_to_word`] so the recursive calls (which Python makes
/// back into the full `_int_to_word`) stay on plain integers. Each recursion
/// site is guarded by `if r` / `if t > 1` in Python, so `n == 0` never actually
/// recurses — but the zero branch is kept here anyway to mirror the Python
/// function's own shape exactly.
fn int_to_word_small(n: u64) -> String {
    if n == 0 {
        return ZERO_WORD.to_string();
    }
    if n < 10 {
        return ONES[n as usize].to_string();
    }
    if n < 100 {
        // Python: t, o = divmod(number, 10)
        //         return self.tens[t] + (" mu " + self.ones[o] if o else "")
        let (t, o) = (n / 10, n % 10);
        let mut s = TENS[t as usize].to_string();
        if o != 0 {
            s.push_str(JOIN);
            s.push_str(ONES[o as usize]);
        }
        return s;
    }
    if n < 1000 {
        // Python: base = (self.ones[h] + " " if h > 1 else "") + self.hundred
        let (h, r) = (n / 100, n % 100);
        let mut s = String::new();
        if h > 1 {
            s.push_str(ONES[h as usize]);
            s.push(' ');
        }
        s.push_str(HUNDRED);
        if r != 0 {
            s.push_str(JOIN);
            s.push_str(&int_to_word_small(r));
        }
        return s;
    }
    if n < 1_000_000 {
        // Python: base = (self._int_to_word(t) + " " if t > 1 else "") + self.thousand
        let (t, r) = (n / 1000, n % 1000);
        let mut s = String::new();
        if t > 1 {
            s.push_str(&int_to_word_small(t));
            s.push(' ');
        }
        s.push_str(THOUSAND);
        if r != 0 {
            s.push_str(JOIN);
            s.push_str(&int_to_word_small(r));
        }
        return s;
    }
    // n < 10^9, guaranteed by the caller.
    // Python: base = (self._int_to_word(m) + " " if m > 1 else "") + self.million
    let (m, r) = (n / 1_000_000, n % 1_000_000);
    let mut s = String::new();
    if m > 1 {
        s.push_str(&int_to_word_small(m));
        s.push(' ');
    }
    s.push_str(MILLION);
    if r != 0 {
        s.push_str(JOIN);
        s.push_str(&int_to_word_small(r));
    }
    s
}

/// Rebuild Python's `str(v)` for the **absolute** value `v`, from the
/// `(digits, scale)` that `bigdecimal` parsed out of it.
///
/// This is not a re-implementation of float formatting. The shim already sent
/// `str(value)` across, and `bigdecimal`'s parser stores that text's digits
/// verbatim as `(int_val, scale)` without normalising — `as_bigint_and_exponent`
/// hands back exactly the digits `repr` chose. The only thing lost in the round
/// trip is the **layout**: where the point went, and whether `repr` picked
/// exponential form. That is a range check on one integer, not an algorithm.
///
/// `decpt` is the point's position relative to the digit string
/// (`value == 0.<digits> * 10^decpt`). CPython emits fixed notation for
/// `-4 < decpt <= 16` and exponential outside it — which is why `str(1e15)` is
/// `"1000000000000000.0"` but `str(1e16)` is `"1e+16"`, and `str(1e-4)` is
/// `"0.0001"` but `str(1e-5)` is `"1e-05"`. Both edges verified against the
/// interpreter.
///
/// # What this cannot recover
///
/// The layout rule reproduced here is `repr(float)`'s. `decimal.Decimal`
/// formats itself by a *different* rule, and by the time a value reaches here
/// the two are indistinguishable — `1e-05` and `Decimal("0.00001")` both parse
/// to `(1, scale=5)`, yet Python spells the first `"1e-05"` (→ ValueError) and
/// the second `"0.00001"` (→ "nuli yuro"). A `Decimal` outside the fixed range
/// therefore takes the float spelling. Flagged in the report; no corpus row
/// reaches it, and the shim's own callers pass floats.
///
/// Integral `Decimal`s are a benign version of the same gap: `str(Decimal("5"))`
/// is `"5"` (one part, `right = 0`) while this returns `"5.0"` (two parts,
/// `right = int("00") = 0`). Different text, same `left`/`right`, same output.
fn py_str(digits: &BigInt, scale: i64) -> String {
    debug_assert!(!digits.is_negative(), "py_str expects abs(value)'s digits");
    let ds = digits.to_string();
    // `ds` is BigInt::to_string of a non-negative: ASCII digits only, so every
    // byte slice below lands on a char boundary by construction.
    let decpt = ds.len() as i64 - scale;

    if decpt > 16 || decpt < -3 {
        // Exponential: one digit, the rest, then a >=2-digit signed exponent.
        // `repr` never pads the mantissa with trailing zeros, and `ds` is from
        // `repr`, so no trimming is needed here.
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

/// Python's `Num2Word_LG.to_cardinal` operating on the already-computed
/// `n = str(number).strip()`.
///
/// LG **overrides `to_cardinal`** and handles non-integers there, working purely
/// on the *text* of `str(number)` — it never calls `float2tuple`, never rounds,
/// and takes no `precision`. Floats and Decimals reach it via `str(number)`, so
/// this is the entire float/Decimal path:
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     return (self.negword + self.to_cardinal(n[1:])).strip()
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "nuli")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// Two Python quirks reproduced verbatim:
/// * `int(left)` / `int(digit)` raise `ValueError` on any non-numeric
///   character. Once `repr` goes exponential (`str(1e16)` == "1e+16",
///   `str(1.5e20)` == "1.5e+20") the slices feed `int("1e+16")` / `int("e")` and
///   Python raises — so those inputs raise here too, via [`py_int`].
/// * a `'0'` fractional digit renders as "nuli", because `self.ones[0]` is the
///   empty string and `"" or "nuli"` is `"nuli"`.
fn cardinal_from_pystr(n: &str) -> Result<String> {
    let n = n.trim();

    // Python: `if n.startswith("-"): return (negword + to_cardinal(n[1:])).strip()`.
    // The sign is detached at the *string* level and the rest re-enters, which is
    // why negatives never reach `int()` with a leading minus.
    if let Some(rest) = n.strip_prefix('-') {
        let inner = cardinal_from_pystr(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }

    match n.split_once('.') {
        Some((left, right)) => {
            // ret = _int_to_word(int(left)) + " " + pointword
            let left_int = py_int(left)?;
            let mut ret = format!("{} {}", int_to_word(&left_int), POINTWORD);
            // for digit in right: ret += " " + (ones[int(digit)] or "nuli")
            for ch in right.chars() {
                ret.push(' ');
                ret.push_str(ones_or_nuli(ch)?);
            }
            Ok(ret.trim().to_string())
        }
        // No ".": `return self._int_to_word(int(n))`. Reachable on the float
        // path only when `repr` produced exponential form (no dot), where
        // `int(n)` raises — reproduced by `py_int`.
        None => Ok(int_to_word(&py_int(n)?)),
    }
}

/// Python's `self.ones[int(digit)] or "nuli"` for a single fractional character.
///
/// `int(digit)` is `ValueError` on a non-digit (e.g. the `'e'` of an exponential
/// `repr`), matching Python exactly. A digit char is always `0..=9`, so the
/// `self.ones[...]` index can never be out of range; index 0 holds the empty
/// string, which `or "nuli"` turns into "nuli".
fn ones_or_nuli(ch: char) -> Result<&'static str> {
    let d = py_int(&ch.to_string())?;
    let idx = d
        .to_usize()
        .expect("a single decimal digit is 0..=9, always a valid usize");
    Ok(if ONES[idx].is_empty() {
        ZERO_WORD
    } else {
        ONES[idx]
    })
}

/// Reconstruct Python's `str(f)` (== `repr(f)`) for an f64.
///
/// Rust's `{:e}` yields the shortest round-trip scientific form — the same
/// significant digits `repr` chooses — and [`py_str`] re-lays it out under
/// CPython's float rule (fixed for `-4 < decpt <= 16`, exponential otherwise,
/// always a trailing ".0" on integral values). Together they reproduce `repr`
/// at every magnitude, including the `"1e+16"` / `"1e-05"` edges that then make
/// the string algorithm raise `ValueError`.
///
/// LG's algorithm never looks at `precision`; it only splits this string. So the
/// f64-artefact trap that bites `base.float2tuple` (2.675 → 674.9999…) does not
/// arise: `{:e}` of the f64 `2.675` is `"2.675e0"`, exactly as `repr` spells it.
fn py_float_str(f: f64) -> String {
    // Python spells -0.0 as "-0.0" (a leading minus that `to_cardinal` strips
    // and recurses on), so the sign is read from the bit, not from `< 0`.
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

/// Reconstruct Python's `str(Decimal)` for `value`.
///
/// This is deliberately **not** [`py_str`]'s float rule: `Decimal.__str__` uses
/// fixed-point notation whenever `exp <= 0 and (exp + len(digits)) > -6`, and
/// scientific (uppercase `E`, signed exponent) otherwise — a different boundary
/// from `repr(float)`. Every corpus `cardinal_dec` row is fixed-point, but
/// `Decimal("1E-7")` really does stringify to "1E-7" and then hits
/// `int("1E-7")` → `ValueError`; only the correct rule reproduces that. Ported
/// from CPython's `decimal.Decimal.__str__` (non-engineering branch).
fn py_decimal_str(value: &BigDecimal) -> String {
    let neg = value.is_negative();
    // `bigdecimal` does not normalise, so this hands back exactly the
    // coefficient and exponent of the Decimal the shim parsed (trailing zeros
    // of "1.10" preserved as (110, scale=2)); see the `bigdecimal_does_not_normalise`
    // test guarding this premise.
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
        // Python: `['e','E'][capitals] + "%+d" % (leftdigits - dotplace)`;
        // the default Decimal context has `capitals == 1`, i.e. uppercase 'E'.
        format!("E{:+}", leftdigits - dotplace)
    };

    let body = format!("{}{}{}", intpart, fracpart, exp_part);
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Python's `cr[1] if n != 1 else cr[0]`, as `to_currency` spells it inline.
///
/// Deliberately *not* routed through [`LangLg::pluralize`]: that method picks
/// `forms[-1]`, this picks `forms[1]`. The two coincide for every two-form
/// entry LG has, but they are separate code in Python and the arities are not
/// guaranteed to stay at two. A one-form entry would raise IndexError here, so
/// the miss is typed rather than panicked — unreachable today, since all three
/// entries and the fallback carry exactly two forms.
fn pick_form(forms: &[String], n: &BigInt) -> Result<String> {
    let idx = if n.is_one() { 0 } else { 1 };
    forms
        .get(idx)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// `CURRENCY_FORMS`. Only three codes; no other class touches this dict.
///
/// `Num2Word_LG` subclasses `Num2Word_Base` directly, **not** `Num2Word_EUR`,
/// and declares its own `CURRENCY_FORMS` in the class body. So the shared-dict
/// mutation `Num2Word_EN.__init__` performs (which rewrites EUR/GBP and adds
/// ~24 codes onto `Num2Word_EUR`'s table at import time) cannot reach here:
/// LG's EUR really is `("yuro", "yuro")`, and GBP really is absent. Confirmed
/// against the live interpreter, and by the corpus — `currency:GBP` renders
/// "shilingi" via the fallback rather than "pound".
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const SENTI: [&str; 2] = ["senti", "senti"];
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("UGX", CurrencyForms::new(&["shilingi", "shilingi"], &SENTI));
    m.insert("USD", CurrencyForms::new(&["doola", "doola"], &SENTI));
    m.insert("EUR", CurrencyForms::new(&["yuro", "yuro"], &SENTI));
    m
}

pub struct LangLg {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — `to_currency`'s fallback for
    /// an unknown code.
    ///
    /// Python dicts iterate in insertion order, and LG's class body opens with
    /// `"UGX"`, so the first value is UGX's forms. Pinned as a field rather
    /// than recomputed: a `HashMap` has no first entry, and re-deriving "first"
    /// from an unordered map would be a coin flip. Verified against the live
    /// interpreter (`list(c.CURRENCY_FORMS.values())[0]` -> shilingi/senti).
    fallback_forms: CurrencyForms,
}

impl Default for LangLg {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLg {
    pub fn new() -> Self {
        // `setup()` only assigns constant tables, which live as `const`s above.
        // There is no per-instance state and no cross-call mutable flag.
        //
        // The currency tables are built once, here: `to_currency` only ever
        // reads them, and rebuilding them per call is what made an earlier
        // revision of this port slower than the Python it replaces.
        LangLg {
            currency_forms: build_currency_forms(),
            fallback_forms: CurrencyForms::new(
                &["shilingi", "shilingi"],
                &["senti", "senti"],
            ),
        }
    }
}

impl Lang for LangLg {

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
    /// `"ow' " + to_cardinal(number)` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "ow' ttaano akatonnyeze nuli".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("ow' {}", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ow' " + str(number)`. `repr_str` is the
    /// dispatcher's exact `str(value)` (float repr / `Decimal.__str__`), so
    /// trailing zeros and `1E+2`-style exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("ow' {}", repr_str))
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
        "UGX"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    // cards() / maxval() / merge() stay at their trait defaults: LG never
    // builds a card table and the default to_cardinal is fully overridden, so
    // splitnum/clean/merge are unreachable.

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "akatonnyeze"
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// ...
    /// return self._int_to_word(int(n))
    /// ```
    /// Integer input never contains ".", so the fraction branch is skipped.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // The recursion in Python re-enters to_cardinal with the digits
            // only, which lands straight on `_int_to_word(int(n))`.
            let inner = int_to_word(&value.abs());
            // Python applies .strip() to the whole concatenation. NEGWORD's
            // trailing space is interior by then, so only the (already absent)
            // outer whitespace is affected — applied anyway for fidelity.
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        // Note: the non-negative branch has NO .strip() in Python. It makes no
        // difference (int_to_word never pads), but the asymmetry is real.
        Ok(int_to_word(value))
    }

    /// Python: `return "ow' " + self.to_cardinal(number)`.
    ///
    /// Negatives are accepted rather than rejected — LG defines no
    /// `errmsg_negord` guard — so `to_ordinal(-1)` == "ow' wansi wa emu".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ow' {}", self.to_cardinal(value)?))
    }

    /// Python: `return "ow' " + str(number)`. The number is never converted to
    /// words, and the sign survives: `to_ordinal_num(-1)` == "ow' -1".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ow' {}", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is ignored; years are plain cardinals.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / Decimal cardinal path.
    ///
    /// LG **overrides `to_cardinal`**, not `to_cardinal_float`: in Python a float
    /// or Decimal reaches the converter as `to_cardinal(number)`, which renders
    /// `str(number)` directly with no `float2tuple`, no rounding, and no
    /// `precision`. This override reconstructs `str(number)` and runs that exact
    /// string algorithm (see [`cardinal_from_pystr`]).
    ///
    /// Consequences worth stating:
    /// * Neither the banker's-rounding trap nor the f64-artefact trap applies —
    ///   LG reads the `repr` digits, not `abs(value - pre) * 10**precision`.
    /// * The `precision=` kwarg is irrelevant and `precision_override` is
    ///   ignored: Python's `to_cardinal(self, number)` takes no such argument, so
    ///   passing `precision=` raises `TypeError` upstream of this call rather
    ///   than altering the output here. The corpus never exercises it.
    /// * The Float / Decimal split is load-bearing: `str(1.1)` is "1.1" (one
    ///   fractional digit) but `str(Decimal("1.10"))` is "1.10" (trailing zero
    ///   kept) — different words. `precision` is likewise unused for the Decimal
    ///   arm; the coefficient/exponent carried by the `BigDecimal` is the spec.
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
        cardinal_from_pystr(&n)
    }

    // ---- currency -------------------------------------------------------
    //
    // LG overrides `to_currency` and `pluralize`, nothing else. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` are Base's, which
    // the trait defaults already mirror exactly — including `to_cheque`'s
    // `unit = cr1[-1]` plural rule and its KeyError -> NotImplementedError
    // conversion. `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both `{}`,
    // so `currency_adjective` (None) and `currency_precision` (100) are correct
    // at their defaults too.
    //
    // `cardinal_from_decimal` stays at its default: LG's `to_currency` truncates
    // to two fractional digits with `parts[1][:2]` and has no fractional-cents
    // concept at all, so nothing can reach it.

    fn lang_name(&self) -> &str {
        "Num2Word_LG"
    }

    /// The **strict** lookup — `CURRENCY_FORMS[code]`, the one `to_cheque`
    /// performs. `None` here is what turns `cheque:GBP` into
    /// `NotImplementedError`, matching the corpus.
    ///
    /// `to_currency` must *not* come through here: it applies a fallback and
    /// never raises. See [`LangLg::to_currency`].
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
    /// Note `forms[-1]`, the *last* form, not `forms[1]`; and the empty-tuple
    /// guard, which means this can never raise. Dead code for LG as it stands —
    /// `to_currency` is overridden and inlines its own `cr[1]`/`cr[0]` choice,
    /// and Base's `to_cheque` does not consult `pluralize` — but ported because
    /// the class defines it.
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

    /// Python:
    /// ```python
    /// def to_currency(self, val, currency="UGX", cents=True,
    ///                 separator=" ", adjective=False):
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
    /// `adjective` is in the signature and never read — `CURRENCY_ADJECTIVES`
    /// is empty anyway — so it is accepted and dropped, as in Python.
    ///
    /// The `currency="UGX"` default is not modelled: the trait takes `currency`
    /// unconditionally and the shim always supplies it, so the default has no
    /// reachable call site.
    ///
    /// Note what is *absent*: no `parse_currency_parts`, no divisor, no
    /// `has_decimal` guard, no rounding. `right` is two truncated digits of
    /// text, and `if cents and right` keys off `right == 0` — which subsumes
    /// `has_decimal`, since `1.0` and `Decimal("1.00")` both slice to `"00"`
    /// and drop the cents segment exactly as a true `int` does. That is why
    /// this override ignores `has_decimal` entirely rather than threading it
    /// through.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Already resolved through `default_separator()` (" ") when the caller
        // omitted the kwarg.
        let separator = separator.unwrap_or(self.default_separator());

        // Python: is_negative = val < 0  (then `val = abs(val)`, folded below).
        // `-0.0 < 0` is False and bigdecimal has no signed zero, so both sides
        // agree that -0.0 is non-negative.
        let is_negative = val.is_negative();

        let (left, right) = match val {
            // `str(abs(int))` is bare digits: one part, so `right` stays 0 and
            // `int(parts[0])` round-trips to the value. Reproducing the text
            // would be a no-op, so the shortcut is exact rather than an
            // approximation. This is also the only branch that can carry a
            // value past 10^9, where `_int_to_word` degrades to digits.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            // Python: str(abs(val)).split("."), on the text repr produced.
            CurrencyValue::Decimal { value, .. } => {
                let (digits, scale) = value.abs().as_bigint_and_exponent();
                let s = py_str(&digits, scale);
                let mut parts = s.splitn(2, '.');
                let p0 = parts.next().unwrap_or("");
                let p1 = parts.next();

                // left = int(parts[0]) if parts[0] else 0
                let left = if p0.is_empty() {
                    BigInt::zero()
                } else {
                    py_int(p0)?
                };
                // right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
                let right = match p1 {
                    Some(f) if !f.is_empty() => {
                        // `[:2]` slices *characters*; `ljust(2, "0")` pads right
                        // to exactly 2 since the slice is at most 2 long.
                        let head: String = f.chars().take(2).collect();
                        py_int(&format!("{:0<2}", head))?
                    }
                    _ => BigInt::zero(),
                };
                (left, right)
            }
        };

        // Python: CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])
        // — `.get` with a default, so an unknown code silently becomes UGX
        // rather than raising. `currency_forms()` above is the strict lookup
        // and is deliberately bypassed here.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            pick_form(&forms.unit, &left)?
        );

        // Python: `if cents and right:` — a zero `right` is falsy, so ".00"
        // never prints a cents segment.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(&pick_form(&forms.subunit, &right)?);
        }

        if is_negative {
            // negword keeps its trailing space; it is interior after this.
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's .strip(). Nothing here can pad the ends — every form and
        // every `_int_to_word` result is non-empty — but applied for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    /// `py_str`'s load-bearing premise, and the one thing here that a
    /// dependency bump could silently break: `bigdecimal` parses `str(value)`
    /// **without normalising**, so `as_bigint_and_exponent` hands back exactly
    /// the digits `repr` chose. If a future version ever normalised (stripping
    /// `1.0` to `(1, 0)`, or canonicalising `1e+16` to scale 0), `py_str` would
    /// start reconstructing a string Python never produced.
    #[test]
    fn bigdecimal_does_not_normalise() {
        for (s, want_digits, want_scale) in [
            ("1.0", "10", 1i64),
            ("0.5", "5", 1),
            ("0.01", "1", 2),
            ("12.34", "1234", 2),
            ("1234.56", "123456", 2),
            ("1.50", "150", 2),
            ("1e+16", "1", -16),
            ("1e+21", "1", -21),
            ("1.25e+20", "125", -18),
            ("1e-05", "1", 5),
            ("1000000000000000.0", "10000000000000000", 1),
        ] {
            let (d, sc) = BigDecimal::from_str(s).unwrap().as_bigint_and_exponent();
            assert_eq!((d.to_string().as_str(), sc), (want_digits, want_scale), "{s}");
        }
    }

    /// `py_str(parse(repr(x))) == repr(x)` across CPython's layout boundary.
    /// Every string here is a literal `repr()` output taken from the
    /// interpreter, including both edges (1e15 fixed / 1e16 exponential, and
    /// 0.0001 fixed / 1e-05 exponential).
    #[test]
    fn py_str_round_trips_repr() {
        for s in [
            "0.5", "0.01", "1.0", "12.34", "99.99", "1234.56", "100.0", "0.0001",
            "1000000000000000.0", "1e+16", "1.5e+16", "1e+21", "1.25e+20",
            "1.5e+20", "1e-05", "1.5e-05", "1e+100", "0.0",
        ] {
            let (d, sc) = BigDecimal::from_str(s).unwrap().as_bigint_and_exponent();
            assert_eq!(py_str(&d, sc), s);
        }
    }

    fn currency(v: &str, is_int: bool, code: &str) -> Result<String> {
        let val = CurrencyValue::parse(v, is_int, !is_int, !is_int).unwrap();
        LangLg::new().to_currency(&val, code, true, None, false)
    }

    /// Corpus rows, byte for byte.
    #[test]
    fn to_currency_matches_corpus() {
        for (arg, is_int, code, want) in [
            ("0", true, "EUR", "nuli yuro"),
            ("1", true, "EUR", "emu yuro"),
            ("100", true, "EUR", "kikumi yuro"),
            ("1000000", true, "EUR", "kakadde yuro"),
            ("12.34", false, "EUR", "kkumi mu bbiri yuro asatu mu nnya senti"),
            ("0.01", false, "EUR", "nuli yuro emu senti"),
            // A float that is whole still takes the int-shaped output, because
            // ".0" slices to right == 0 rather than because of `has_decimal`.
            ("1.0", false, "EUR", "emu yuro"),
            ("0.5", false, "EUR", "nuli yuro ataano senti"),
            ("-12.34", false, "EUR", "wansi wa kkumi mu bbiri yuro asatu mu nnya senti"),
            (
                "1234.56",
                false,
                "USD",
                "lukumi mu bbiri kikumi mu asatu mu nnya doola ataano mu mukaaga senti",
            ),
            // Unknown codes fall back to UGX rather than raising — the
            // `dict.get(code, first_value)` quirk.
            ("1", true, "GBP", "emu shilingi"),
            ("12.34", false, "JPY", "kkumi mu bbiri shilingi asatu mu nnya senti"),
            // No CURRENCY_PRECISION entry exists, so KWD is *not* 3-decimal.
            ("0.5", false, "KWD", "nuli shilingi ataano senti"),
            ("1", true, "ZZZ", "emu shilingi"),
        ] {
            assert_eq!(currency(arg, is_int, code).unwrap(), want, "{arg} {code}");
        }
    }

    /// `str(val)`-slicing artefacts once `repr` goes exponential. Verified
    /// against the live interpreter; see the module docs.
    #[test]
    fn to_currency_reproduces_repr_slicing_bugs() {
        // int("1e+16") / int("5e") -> ValueError.
        for arg in ["1e+16", "1e+21", "1.5e+20", "1e-05", "1.5e-05"] {
            match currency(arg, false, "EUR") {
                Err(N2WError::Value(_)) => {}
                other => panic!("{arg}: expected ValueError, got {other:?}"),
            }
        }
        // ...but int("25") succeeds, so 1.25e20 euros is one euro 25 cents.
        assert_eq!(
            currency("1.25e+20", false, "EUR").unwrap(),
            "emu yuro abiri mu ttaano senti"
        );
        // A true int never gets stringified, so it is immune: no ValueError,
        // just _int_to_word's bare-digits fallback past 10^9.
        assert_eq!(
            currency("10000000000000000", true, "EUR").unwrap(),
            "10000000000000000 yuro"
        );
    }

    fn card_f(f: f64) -> Result<String> {
        LangLg::new().to_cardinal_float(&FloatValue::Float { value: f, precision: 0 }, None)
    }

    fn card_d(s: &str) -> Result<String> {
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: 0,
        };
        LangLg::new().to_cardinal_float(&v, None)
    }

    /// Float `cardinal` rows (arg with a dot), byte for byte from the corpus.
    /// LG reads `str(number)` == `repr`, so 1.005 / 2.675 need no f64-artefact
    /// rescue: `{:e}` of the f64 already spells the shortest `repr` digits.
    #[test]
    fn to_cardinal_float_matches_corpus() {
        for (f, want) in [
            (0.0, "nuli akatonnyeze nuli"),
            (0.5, "nuli akatonnyeze ttaano"),
            (1.0, "emu akatonnyeze nuli"),
            (1.5, "emu akatonnyeze ttaano"),
            (2.25, "bbiri akatonnyeze bbiri ttaano"),
            (3.14, "ssatu akatonnyeze emu nnya"),
            (0.01, "nuli akatonnyeze nuli emu"),
            (0.1, "nuli akatonnyeze emu"),
            (0.99, "nuli akatonnyeze mwenda mwenda"),
            (1.01, "emu akatonnyeze nuli emu"),
            (12.34, "kkumi mu bbiri akatonnyeze ssatu nnya"),
            (99.99, "kyenda mu mwenda akatonnyeze mwenda mwenda"),
            (100.5, "kikumi akatonnyeze ttaano"),
            (
                1234.56,
                "lukumi mu bbiri kikumi mu asatu mu nnya akatonnyeze ttaano mukaaga",
            ),
            (-0.5, "wansi wa nuli akatonnyeze ttaano"),
            (-1.5, "wansi wa emu akatonnyeze ttaano"),
            (-12.34, "wansi wa kkumi mu bbiri akatonnyeze ssatu nnya"),
            // The two f64-artefact traps: str(number) sidesteps them entirely.
            (1.005, "emu akatonnyeze nuli nuli ttaano"),
            (2.675, "bbiri akatonnyeze mukaaga musanvu ttaano"),
        ] {
            assert_eq!(card_f(f).unwrap(), want, "{f}");
        }
    }

    /// `-0.0` keeps its minus in `repr` ("-0.0"), so LG prepends negword —
    /// verified against the live interpreter.
    #[test]
    fn to_cardinal_float_negative_zero() {
        assert_eq!(card_f(-0.0).unwrap(), "wansi wa nuli akatonnyeze nuli");
    }

    /// Once `repr` goes exponential the string algorithm feeds `int()` a
    /// non-numeric slice and Python raises `ValueError`; reproduced here.
    #[test]
    fn to_cardinal_float_exponential_raises() {
        // str==="1e+16"/"1e+21"/"1e-05": no dot, int("1e+16") -> ValueError.
        // str==="1.5e+20": dot, but int("e") in the fraction loop -> ValueError.
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
            ("0.01", "nuli akatonnyeze nuli emu"),
            ("1.10", "emu akatonnyeze emu nuli"),
            ("12.345", "kkumi mu bbiri akatonnyeze ssatu nnya ttaano"),
            ("98746251323029.99", "98746251323029 akatonnyeze mwenda mwenda"),
            ("0.001", "nuli akatonnyeze nuli nuli emu"),
            // Integral Decimal: str is "5" (no dot) -> plain _int_to_word.
            ("5", "ttaano"),
            ("0.0", "nuli akatonnyeze nuli"),
        ] {
            assert_eq!(card_d(s).unwrap(), want, "{s}");
        }
    }

    /// `py_decimal_str` follows Decimal's own `__str__`, not `py_str`'s float
    /// rule: uppercase 'E', and fixed-point down to exponent -6.
    #[test]
    fn py_decimal_str_follows_decimal_rule() {
        for (s, want) in [
            ("0.01", "0.01"),
            ("1.10", "1.10"),
            ("98746251323029.99", "98746251323029.99"),
            ("5", "5"),
            ("0.0", "0.0"),
            // Fixed-point holds to exp -6 (float repr would go exponential at -5).
            ("0.00001", "0.00001"),
            // Beyond that, and for positive exponents, scientific with 'E'.
            ("1E-7", "1E-7"),
            ("1E+2", "1E+2"),
        ] {
            let d = BigDecimal::from_str(s).unwrap();
            assert_eq!(py_decimal_str(&d), want, "{s}");
        }
    }

    /// to_cheque is Base's: it raises where to_currency silently falls back.
    #[test]
    fn to_cheque_matches_corpus() {
        let lg = LangLg::new();
        let v = BigDecimal::from_str("1234.56").unwrap();
        assert_eq!(
            lg.to_cheque(&v, "EUR").unwrap(),
            "LUKUMI MU BBIRI KIKUMI MU ASATU MU NNYA AND 56/100 YURO"
        );
        assert_eq!(
            lg.to_cheque(&v, "USD").unwrap(),
            "LUKUMI MU BBIRI KIKUMI MU ASATU MU NNYA AND 56/100 DOOLA"
        );
        // Same code that to_currency renders as "shilingi" raises here.
        match lg.to_cheque(&v, "GBP") {
            Err(N2WError::NotImplemented(m)) => {
                assert_eq!(m, "Currency code \"GBP\" not implemented for \"Num2Word_LG\"")
            }
            other => panic!("expected NotImplementedError, got {other:?}"),
        }
        assert_eq!(currency("1", true, "GBP").unwrap(), "emu shilingi");
    }
}
