//! Port of `lang_BO.py` (Tibetan).
//!
//! Shape: **self-contained**. `Num2Word_BO` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives a recursive
//! `_int_to_word`. Consequently `cards`/`merge` stay at their trait defaults
//! here, and Python has **no overflow check and no ceiling at all** — the top
//! branch of `_int_to_word` recurses on `number // 10**8` indefinitely,
//! stacking "དུང་ཕྱུར་" once per 10^8 factor (`to_cardinal(10**21)` is
//! "གཅིག་འབུམ་དུང་ཕྱུར་དུང་ཕྱུར་"). On a large enough integer the port's
//! recursion overflowed the native stack, so it adds a ceiling
//! (gladiaio/num2words2#203): `maxval` is 10^16, where the multiplier would
//! itself need "དུང་ཕྱུར་", and every mode raises `OverflowError` from there.
//!
//! `setup()` sets `negword = "མེད་ཆ་ "` (note the **trailing ASCII space**, which
//! is part of the literal) and `pointword = "ཚེག་"`.
//!
//! Inherited from `Num2Word_Base` but overridden by BO, so the trait defaults
//! are *not* used for any of the four in-scope modes:
//!   * `to_cardinal` — own algorithm (below).
//!   * `to_ordinal(n)` → `to_cardinal(n) + "པ་"`.
//!   * `to_ordinal_num(n)` → `str(n) + "པ"` (BO overrides the base, which
//!     returns the value unchanged).
//!   * `to_year(n)` → `"སྤྱི་ལོ་" + to_cardinal(n)` (BO overrides the base, which
//!     delegates to `to_cardinal`).
//!
//! # Behaviour worth flagging (reproduced verbatim, not fixed)
//!
//! 1. **No `verify_ordinal` call.** `Num2Word_Base.to_ordinal` would normally
//!    be guarded by `verify_ordinal`, which raises `TypeError` on negatives.
//!    BO's override skips it entirely, so `to_ordinal(-1000000)` happily
//!    returns "མེད་ཆ་ གཅིག་ས་ཡ་པ་" — the negative-marker prefix with an ordinal
//!    suffix glued on. Confirmed against the frozen corpus.
//! 2. **`to_year` prepends its era marker *outside* the sign.** Because
//!    `to_year` is `"སྤྱི་ལོ་" + self.to_cardinal(val)` and `to_cardinal` emits the
//!    negword itself, a negative year reads "སྤྱི་ལོ་" then "མེད་ཆ་ " — i.e.
//!    `to_year(-500)` == "སྤྱི་ལོ་མེད་ཆ་ ལྔ་བརྒྱ་", with the era word preceding the
//!    minus marker. Confirmed against the frozen corpus.
//! 3. **Asymmetric multiplier handling.** The `< 1000` branch indexes `ones`
//!    directly (`ones[number // 100]`) while every branch above it recurses
//!    (`self._int_to_word(number // 1000)` etc.). This is why 100 is
//!    "གཅིག་བརྒྱ་" (an explicit "one hundred") and why the >= 10^8 branch can
//!    nest "དུང་ཕྱུར་" arbitrarily deep.
//! 4. **`negword` is used raw, not stripped.** BO's `to_cardinal` does
//!    `ret = self.negword` where `Num2Word_Base.to_cardinal` would do
//!    `"%s " % self.negword.strip()`. Both happen to yield "མེད་ཆ་ " here
//!    because the literal already carries exactly one trailing space, but the
//!    literal is what is reproduced below.
//!
//! # The float / Decimal path
//!
//! BO does **not** override `to_cardinal_float`; it overrides `to_cardinal`
//! and handles non-integers inline. So `Num2Word_Base.to_cardinal_float` —
//! and with it `float2tuple` — is **never reached**, and none of that
//! machinery applies here:
//!
//! * **No `float2tuple`, so no f64 artefacts to preserve.** The base class
//!   computes `post = abs(value - pre) * 10**precision` in binary f64 and
//!   leans on a `< 0.01` heuristic to rescue `2.675` from `674.9999999999998`.
//!   BO never does any float arithmetic at all: it reads the *digits of
//!   `str(number)`*. `2.675` is "two point six seven five" because the
//!   characters `6`, `7`, `5` are literally in `repr(2.675)`, not because a
//!   rounding heuristic recovered them. `1.005` likewise.
//! * **No `round()`, so no banker's-rounding trap.** There is no rounding
//!   anywhere on this path.
//! * **`self.precision` is never read**, so the `precision=` kwarg (issue
//!   #580) is inert: `num2words(2.675, lang="bo", precision=1)` still emits
//!   all three fractional digits. `precision_override` is accordingly ignored
//!   — see [`LangBo::to_cardinal_float`].
//!
//! What the path *does* need is `str(number)`, byte for byte, because every
//! branch keys off it: the `"-"` prefix, the `"."`, and one word per
//! character of the fractional part. That makes the two `FloatValue` arms
//! genuinely different functions — `repr(float)` and `Decimal.__str__` are
//! separate CPython routines with separate exponent-notation rules — so
//! [`python_float_repr`] and [`python_decimal_str`] reproduce them
//! separately. Both are reconstructions, and their limits are documented on
//! each.
//!
//! Trailing zeros are the clearest proof that this is a string path rather
//! than a numeric one: `Decimal("1.10")` renders as "one point one zero"
//! because `str()` kept the zero, while `1.10` as a *float* renders as "one
//! point one" because `repr()` dropped it. A numeric port collapses the two.
//!
//! # Error variants
//!
//! For the four integer modes: none. Every in-scope path is total for integer
//! input: `_int_to_word` only ever indexes `ones`/`tens` with a value it has
//! already bounded to 0..=9, and the sign is stripped before recursion begins,
//! so no `IndexError`, `KeyError` or `ValueError` site exists. The only
//! error is the port's `OverflowError` at the 10^16 ceiling (#203).
//!
//! The currency surface adds two, both detailed below: `NotImplementedError`
//! from `to_cheque` on an unknown code, and `ValueError` from `_split_currency`
//! when `str(val)` came out in exponent notation.
//!
//! The float path adds one more, for the same underlying reason: `int()` is
//! fed raw characters of `str(number)`, so any value whose `str()` went
//! scientific raises **`ValueError`** rather than converting. See
//! [`int_token`].
//!
//! # The currency surface
//!
//! `Num2Word_BO` declares `CURRENCY_FORMS` for exactly three codes (CNY, USD,
//! EUR) and overrides `to_currency` outright. It inherits `CURRENCY_ADJECTIVES`
//! and `CURRENCY_PRECISION` from `Num2Word_Base`, where **both are empty** — so
//! `CURRENCY_PRECISION.get(code, 100)` is 100 for *every* code and the trait
//! defaults for `currency_precision`/`currency_adjective` are already correct.
//! This is why BO has no 3-decimal or 0-decimal behaviour: `to_currency("KWD")`
//! divides by 100, not 1000, and `to_currency("JPY", 12.34)` still renders a
//! cents segment rather than rounding to a whole unit. Confirmed against the
//! frozen corpus for both codes.
//!
//! `to_cheque` is *not* overridden, so `Num2Word_Base.to_cheque` runs verbatim
//! — which means the two halves of the surface disagree about unknown codes:
//!
//! * `to_currency` does `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["CNY"])`
//!   and so **silently falls back to CNY** — `to_currency(1, currency="GBP")`
//!   is "གཅིག་ སྒོར་", never an error.
//! * `to_cheque` does `CURRENCY_FORMS[currency]` and so raises
//!   `NotImplementedError` for the very same code.
//!
//! Both are reproduced: [`LangBo::to_currency`] resolves the fallback itself,
//! while `currency_forms` returns `None` for an unknown code so that the
//! trait's default `to_cheque` (== `default_to_cheque`, a faithful port of the
//! base method) raises. Corpus: `currency:GBP` succeeds with CNY words,
//! `cheque:GBP` raises `NotImplementedError`.
//!
//! ## `parse_currency` does not exist
//!
//! `Num2Word_BO.to_currency` opens with
//!
//! ```text
//! try:
//!     left, right, is_negative = self.parse_currency(val)
//! except AttributeError:
//!     ...
//! ```
//!
//! No `parse_currency` method is defined anywhere in num2words2 — not on
//! `Num2Word_BO`, not on `Num2Word_Base` (`currency.py` exports the free
//! function `parse_currency_parts`, which is a different name and never bound
//! to the instance). The attribute lookup therefore *always* raises
//! `AttributeError`, the `try` body never completes, and the `except` branch is
//! the only live path. It is ported as straight-line code with no error plumbing,
//! because the exception is unconditional and always swallowed.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `_int_to_word`'s early return for 0.
const ZERO_WORD: &str = "ཀླད་ཀོར་";

/// `setup(): self.negword` — the trailing ASCII space is part of the literal.
const NEGWORD: &str = "མེད་ཆ་ ";

/// `setup(): self.pointword`.
const POINTWORD: &str = "ཚེག་";

/// `to_year`'s era prefix.
const YEAR_PREFIX: &str = "སྤྱི་ལོ་";

/// `Num2Word_BO.to_currency`'s own default `separator=" དང་ "`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot simply be a parameter default.
const SEPARATOR_DEFAULT: &str = " དང་ ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_BO.to_currency` declares `separator=" དང་ "`, but the `Lang` trait
/// has no per-language defaults: `__init__.py`'s currency fast path (and
/// `diff_test.py`) substitute `kwargs.get("separator", ",")` — **`Num2Word_Base`'s**
/// default — before the value ever reaches Rust. By then "caller omitted
/// separator" and "caller explicitly passed a comma" are the same string, and
/// the information needed to tell them apart no longer exists on this side of
/// the boundary.
///
/// So `,` is read back as the unset sentinel and BO's own default restored.
/// This is the only reading that matches the oracle: every float row of the
/// `bo` currency corpus was generated by `num2words(v, lang="bo",
/// to="currency", currency=c)` with no `separator=`, and every one of them
/// expects " དང་ ".
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " དང་ " here where Python would give ",". Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report. This
/// mirrors the convention `lang_ca.rs` established for the same problem.
const SEPARATOR_UNSET: &str = ",";

/// `to_currency`'s fallback code: `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["CNY"])`.
///
/// Note the Python evaluates `self.CURRENCY_FORMS["CNY"]` *eagerly* as the
/// `.get` default, so a class that dropped the CNY entry would raise `KeyError`
/// on every call. BO always has it, so the lookup below is infallible.
const FALLBACK_CURRENCY: &str = "CNY";

/// `to_ordinal`'s suffix (note the trailing tsheg).
const ORDINAL_SUFFIX: &str = "པ་";

/// `to_ordinal_num`'s suffix (note: **no** trailing tsheg, unlike `to_ordinal`).
const ORDINAL_NUM_SUFFIX: &str = "པ";

/// `ones`, index 0..=9. Index 0 is the empty string in Python.
const ONES: [&str; 10] = [
    "",
    "གཅིག་",
    "གཉིས་",
    "གསུམ་",
    "བཞི་",
    "ལྔ་",
    "དྲུག་",
    "བདུན་",
    "བརྒྱད་",
    "དགུ་",
];

/// `tens`, index 0..=9 (the multiple-of-ten words). Index 0 is the empty
/// string in Python and is unreachable (the `< 10` branch catches it first).
const TENS: [&str; 10] = [
    "",
    "བཅུ་",
    "ཉི་ཤུ་",
    "སུམ་ཅུ་",
    "བཞི་བཅུ་",
    "ལྔ་བཅུ་",
    "དྲུག་ཅུ་",
    "བདུན་ཅུ་",
    "བརྒྱད་ཅུ་",
    "དགུ་བཅུ་",
];

/// The hundreds word, spliced between `ones[number // 100]` and the remainder.
const HUNDRED: &str = "བརྒྱ་";

/// The scale branches above 1000, in the exact order of Python's `elif` chain.
///
/// Each entry is `(limit_exp, div_exp, word)` standing for Python's
/// `elif number < 10**limit_exp: _int_to_word(number // 10**div_exp) + word +
/// (_int_to_word(number % 10**div_exp) if number % 10**div_exp else "")`.
///
/// The final entry (`10^8`, "དུང་ཕྱུར་") is Python's unguarded `else` arm; its
/// `limit_exp` is never consulted, which is what makes the recursion
/// unbounded. See [`int_to_word`].
const SCALES: [(u32, u32, &str); 6] = [
    (4, 3, "སྟོང་"),        // < 10_000        → thousands
    (5, 4, "ཁྲི་"),         // < 100_000       → ten-thousands
    (6, 5, "འབུམ་"),        // < 1_000_000     → hundred-thousands
    (7, 6, "ས་ཡ་"),        // < 10_000_000    → millions
    (8, 7, "བྱེ་བ་"),        // < 100_000_000   → ten-millions
    (0, 8, "དུང་ཕྱུར་"),      // else (limit unused) → hundred-millions
];

/// `10**exp` as a `BigInt`.
fn pow10(exp: u32) -> BigInt {
    BigInt::from(10u8).pow(exp)
}

/// Narrow a `BigInt` the caller has already bounded to 0..=999.
///
/// Every call site sits behind a `n < 1000` comparison that proves the range,
/// so the `expect` is unreachable rather than a lurking panic. Below 1000 the
/// arithmetic is done in `usize`: the values are provably bounded, and
/// Python's `//`/`%` agree with Rust's `/`/`%` on non-negatives.
fn small(n: &BigInt) -> usize {
    n.to_usize().expect("caller proved this value is < 1000")
}

/// Port of `Num2Word_BO._int_to_word`.
///
/// Only ever reached with a non-negative value: `to_cardinal` strips the
/// leading "-" from the *string* before calling `int()`, so the sign never
/// arrives here. (Were it to, Python's negative list indexing would silently
/// wrap — `ones[-1]` == "དགུ་" — but that path is unreachable for the four
/// in-scope modes.)
///
/// Recursion depth is `O(log_1e8(n))`, so even a 10^606 input nests only ~76
/// deep — no fixed-width cast is needed or made anywhere.
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO_WORD.to_string();
    }

    // if number < 10: return ones[number]
    if n < &pow10(1) {
        return ONES[small(n)].to_string();
    }

    // elif number < 100:
    //     return tens[number // 10] + (ones[number % 10] if number % 10 else "")
    if n < &pow10(2) {
        let v = small(n);
        let mut out = TENS[v / 10].to_string();
        if v % 10 != 0 {
            out.push_str(ONES[v % 10]);
        }
        return out;
    }

    // elif number < 1000:
    //     return ones[number // 100] + "བརྒྱ་"
    //          + (self._int_to_word(number % 100) if number % 100 else "")
    //
    // NB: the multiplier is a *direct index* here, not a recursive call —
    // see bug note 3 in the module docs.
    if n < &pow10(3) {
        let v = small(n);
        let mut out = ONES[v / 100].to_string();
        out.push_str(HUNDRED);
        if v % 100 != 0 {
            out.push_str(&int_to_word(&BigInt::from(v % 100)));
        }
        return out;
    }

    // The remaining `elif` arms share one shape; the last is Python's `else`,
    // whose limit is never tested — hence the unbounded "དུང་ཕྱུར་" nesting.
    for (i, &(limit_exp, div_exp, word)) in SCALES.iter().enumerate() {
        let is_else_arm = i == SCALES.len() - 1;
        if !is_else_arm && n >= &pow10(limit_exp) {
            continue;
        }
        let d = pow10(div_exp);
        let q = n / &d;
        let r = n % &d;
        let mut out = int_to_word(&q);
        out.push_str(word);
        if !r.is_zero() {
            out.push_str(&int_to_word(&r));
        }
        return out;
    }

    unreachable!("the final SCALES entry is an unconditional else arm")
}

/// Python's `int(token)`, for the tokens `to_cardinal` actually feeds it.
///
/// `to_cardinal` never validates: it hands `int()` whatever slice of
/// `str(number)` it carved out, and lets the exception escape. The reachable
/// tokens all come from `repr(float)` or `Decimal.__str__` with at most one
/// leading `"-"` already peeled off, so their alphabet is
/// `[0-9] . - + e E i n f a`. Over that alphabet "every character is an ASCII
/// digit" is exactly Python's accept condition, and everything else raises
/// `ValueError` with the token quoted verbatim — `'e'`, `'1e+16'`, `'1E-7'`,
/// `'inf'`, `'nan'`.
///
/// The wider `int()` grammar (surrounding whitespace, a `+`/`-` sign,
/// `1_000` underscores, non-ASCII decimal digits like `'٥'`) is deliberately
/// *not* implemented: none of it is reachable from a `str()` of a float or a
/// Decimal, and accepting it would only paper over inputs Python rejects. The
/// argument does not actually depend on the alphabet being complete — a
/// `Decimal("Infinity")` would widen it, though `BigDecimal::from_str` rejects
/// that at the shim before it can — only on none of these sources emitting
/// the sign/space/underscore forms `int()` accepts and this rejects. None do.
fn int_token(token: &str) -> Result<BigInt> {
    if !token.is_empty() && token.chars().all(|c| c.is_ascii_digit()) {
        // Leading zeros are fine — Python's int("007") is 7, and so is this.
        if let Some(v) = BigInt::parse_bytes(token.as_bytes(), 10) {
            return Ok(v);
        }
    }
    Err(N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        token
    )))
}

/// CPython's `repr(float)` — what `str(number)` returns for a `float`.
///
/// `precision` is the shim's `abs(Decimal(repr(value)).as_tuple().exponent)`,
/// i.e. the number of fractional digits `repr` emitted. Its whole purpose is
/// to keep shortest-round-trip digit selection on the Python side, so the
/// fixed-notation branch just asks for exactly that many digits:
/// `{:.p$}` is correctly-rounded fixed formatting, and the nearest
/// `p`-digit decimal to the double *is* `repr`. (No tie can split them: a tie
/// would mean the double sits exactly halfway between two `p`-digit decimals,
/// which forces `repr` to spend a `p+1`-th digit and contradicts `p`.)
///
/// # The one thing that must be reimplemented
///
/// `repr` switches to exponent notation, and `precision` cannot reveal when:
/// `repr(1e-05)` is `'1e-05'` and `repr(0.00012)` is `'0.00012'`, and both
/// report `precision == 5`. CPython decides in `format_float_short`:
///
/// ```text
/// case 'r':  /* convert to exponential format at 1e16.  This is used for repr. */
///     if (decpt <= -4 || decpt > 16)
///         use_exp = 1;
/// ```
///
/// where `decpt` positions the decimal point against the *shortest* digits
/// (`value == 0.<digits> * 10**decpt`). Reading `decpt` back out would mean
/// reimplementing the digit selection this function exists to avoid — but the
/// test collapses to a comparison on the value itself:
///
/// * `decpt > 16` ⟺ `|v| >= 1e16`. A shortest decimal `D < 10**16` can only
///   round to a double `>= 10**16` if that double *is* `1e16` (doubles are
///   spaced 2 apart there, and `float("9999999999999999")` ties to even and
///   lands on it) — and `1e16`'s own shortest form is the 1-digit `"1"` with
///   `decpt == 17`, so `decpt <= 16` never survives.
/// * `decpt <= -4` ⟺ `0 < |v| < 1e-4`. The double nearest `1e-4` sits just
///   *above* it, so no `D >= 10**-4` rounds below the boundary and vice
///   versa.
/// * `v == 0.0` is neither: CPython's dtoa reports `decpt == 1` for zero, so
///   `repr(0.0)` is the fixed `'0.0'` (and `repr(-0.0)` is `'-0.0'`, a sign
///   `{:.1}` preserves and `is_negative()` would not).
///
/// Checked against CPython over 608,900 doubles — random bit patterns plus
/// 300-step `nextafter` walks either side of `1e16`, `1e-4`, `1e-5`, `1e15`,
/// `1e17`, `1e23`, `5e-324` and `f64::MAX` — with zero divergence.
///
/// The exponent branch then reuses Rust's `{:e}`, whose mantissa is already
/// normalised to one leading digit exactly like CPython's; only the exponent
/// spelling differs (`e16` vs `e+16`, `e-5` vs `e-05`). That branch *does*
/// lean on Rust and CPython agreeing on the shortest digits, which the same
/// 608,900-value run confirms. Every value reaching it raises `ValueError`
/// regardless (see [`int_token`]) — the digits only pick which token the
/// message quotes.
fn python_float_repr(v: f64, precision: u32) -> String {
    // repr(nan)/repr(inf) are lowercase in Python; Rust's Display renders NaN
    // as "NaN". Neither is reachable through the shim (the Python-side
    // precision computation does abs() on a Decimal exponent of 'F'/'n',
    // which is a str and raises TypeError first), but `int()` would reject
    // both tokens anyway, so spell them Python's way and let int_token raise.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    let a = v.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // "-1.234e-5" -> "-1.234e-05"; "1e16" -> "1e+16".
        let sci = format!("{:e}", v);
        let (mantissa, exp) = sci
            .split_once('e')
            .expect("Rust's {:e} always emits an 'e' separator");
        let exp: i32 = exp
            .parse()
            .expect("Rust's {:e} always emits a decimal exponent");
        return format!(
            "{}e{}{:02}",
            mantissa,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }

    format!("{:.*}", precision as usize, v)
}

/// CPython's `Decimal.__str__` — what `str(number)` returns for a `Decimal`.
///
/// A transcription of `_pydecimal.Decimal.__str__`, which works off the
/// coefficient digits and the exponent rather than the value:
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # no exponent required
/// else:
///     dotplace = 1                   # scientific: 1 digit left of the point
/// ...
/// if leftdigits == dotplace: exp = ''
/// else:                     exp = 'E%+d' % (leftdigits - dotplace)
/// ```
///
/// `BigDecimal::as_bigint_and_exponent` hands back exactly the pair Python
/// stores — `scale` is the negation of `_exp`, and the coefficient keeps its
/// significant digits verbatim, trailing zeros included (`"1.10"` is 110×10⁻²
/// on both sides, `"0.00"` is 0×10⁻²). That fidelity is the whole reason this
/// is reconstructed rather than delegated: **`BigDecimal`'s own `Display` is
/// not `str(Decimal)`.** It prints `"0.00"` as `"0"` (dropping a scale Python
/// keeps, which would silently eat two fractional words) and `"1E+16"` as
/// lowercase `"1e+16"` (Python emits uppercase `E`, and that string is quoted
/// verbatim in the `ValueError` this value raises).
///
/// Note the thresholds differ from [`python_float_repr`]'s: `Decimal` goes
/// scientific whenever `_exp > 0` or `leftdigits <= -6`, so `Decimal("1E+16")`
/// stringifies as `'1E+16'` and raises where the numerically equal
/// `Decimal(10**16)` stringifies as plain digits and converts. Reproducing
/// `repr(float)`'s rule here instead would be wrong in both directions.
///
/// # Known gap: negative zero
///
/// `str(Decimal("-0.0"))` is `'-0.0'`, so BO emits its negword. `BigInt` has
/// no signed zero, so `BigDecimal::from_str("-0.0")` arrives here as `+0`
/// scale 1 and the sign is already gone — this returns `"0.0"` and the negword
/// is dropped. The sign is lost at the boundary, before this function runs, so
/// it cannot be recovered here. Flagged in the port report; not in the corpus.
fn python_decimal_str(d: &BigDecimal) -> String {
    let (coefficient, scale) = d.as_bigint_and_exponent();
    // Python's `_exp`; `as_bigint_and_exponent` reports the negated form.
    let exp = -scale;
    let sign = if coefficient.is_negative() { "-" } else { "" };
    // Python's `_int`: the unsigned coefficient digits. BigInt renders ASCII
    // only, so the byte slicing below is char slicing.
    let digits = coefficient.abs().to_string();

    let leftdigits = exp + digits.len() as i64;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    // Every repeat below is provably tiny, so none of this can blow up on a
    // `Decimal("1E+1000000")`: the scientific branch pins dotplace to 1, and
    // the plain branch has exp <= 0, hence dotplace == leftdigits <= len.
    let (intpart, fracpart) = if dotplace <= 0 {
        // 0.<zeros><digits> — at most 5 zeros, since leftdigits > -6.
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), digits),
        )
    } else if dotplace as usize >= digits.len() {
        (
            format!("{}{}", digits, "0".repeat(dotplace as usize - digits.len())),
            String::new(),
        )
    } else {
        (
            digits[..dotplace as usize].to_string(),
            format!(".{}", &digits[dotplace as usize..]),
        )
    };

    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        // Python's 'E%+d' — a mandatory sign and no zero padding, unlike
        // repr(float)'s two-digit minimum.
        let e = leftdigits - dotplace;
        format!("E{}{}", if e < 0 { '-' } else { '+' }, e.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

/// `str(number)` as `Num2Word_BO.to_cardinal` sees it.
///
/// The `FloatValue` split is exactly the `float`/`Decimal` split CPython makes
/// when choosing which `__str__`/`__repr__` to run, so it maps straight onto
/// the two reconstructions.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => python_float_repr(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Port of `Num2Word_BO._split_currency`, applied to `abs(val)`.
///
/// ```text
/// def _split_currency(self, n):
///     parts = str(n).split(".")
///     left = int(parts[0]) if parts[0] else 0
///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
///     return left, right
/// ```
///
/// This is a *string* operation on `str(n)`, so the port has to reproduce what
/// `str()` would have produced rather than just doing decimal arithmetic:
///
/// * **`CurrencyValue::Int`** — `str(int)` is plain digits with no "." and no
///   exponent at any magnitude, so `parts` has one element: `left` is the
///   integer, `right` is 0. An int therefore never shows cents, which is the
///   `isinstance(val, int)` distinction the boundary preserves.
///
/// * **`CurrencyValue::Decimal`** — the `BigDecimal` was parsed from the exact
///   `str(value)` the Python side produced, so its digits *are* the characters
///   Python split on. For any plain-decimal `str()`, Python's
///   `int(parts[1][:2].ljust(2, "0"))` — take the first two fractional digits,
///   right-pad with zeros — is exactly `floor(frac * 100)`, which is what the
///   truncating arithmetic below computes:
///
///   | `str(n)` | `parts[1][:2].ljust(2,"0")` | `right` | `floor(frac*100)` |
///   |---|---|---|---|
///   | `"12.34"` | `"34"` | 34 | 34 |
///   | `"0.5"`   | `"50"` | 50 | 50 |
///   | `"1.0"`   | `"00"` | 0  | 0 |
///   | `"0.001"` | `"00"` | 0  | 0 |
///   | `"0.567"` | `"56"` | 56 | 56 |
///
///   The `ljust` and the `[:2]` truncation are both subsumed: `BigDecimal`
///   carries the trailing zeros of `"1.0"` in its scale, and `with_scale(0)`
///   truncates rather than rounds (`99.99 -> 99`), matching `[:2]` discarding
///   the third digit instead of rounding on it.
///
/// # The exponent-notation `ValueError`
///
/// `str()` switches to exponent notation for a float outside roughly
/// `1e-4 <= |v| < 1e16`, and `int()` chokes on the resulting token:
/// `str(1e16)` is `"1e+16"`, `parts` is `["1e+16"]`, and `int("1e+16")` raises
/// **`ValueError`**. (`str(1.5e16)` is `"1.5e+16"`, where `parts[1][:2]` is
/// `"5e"` and `int("5e")` raises `ValueError` too — same exception, different
/// token.) So the whole function raises for large floats rather than returning
/// anything.
///
/// The check below keys off the *representation*, which is the same thing
/// Python's `str().split(".")` does, so it stays faithful for `Decimal` inputs
/// too (the shim accepts those alongside floats): `Decimal("1E+16")` stringifies
/// with an exponent and raises, while `Decimal(10**16)` stringifies as plain
/// digits and succeeds — and the two parse to different scales here (-16 vs 0),
/// so both are handled correctly.
///
/// A **positive** exponent is therefore recoverable: plain decimal notation can
/// never yield a negative `BigDecimal` scale, so `scale < 0` proves `str()`
/// emitted an exponent, and the `ValueError` is raised here. Verified against
/// CPython: the message matches `int("1e+16")`'s byte for byte.
///
/// A **negative** exponent is not recoverable, and this is the one known gap.
/// `str(1e-05)` is `"1e-05"` (CPython goes scientific below `1e-4`) and raises,
/// but it parses to the very same `(1, scale=5)` as `Decimal("0.00001")`, which
/// stringifies as `"0.00001"` and does *not* raise. The two are indistinguishable
/// once across the boundary, so a float `< 1e-4` silently returns `(0, 0)` here
/// where Python raises `ValueError`. Discriminating on the value instead
/// (`|v| < 1e-4`) would fix the float and break the `Decimal`, so the
/// representation-faithful rule is kept and the gap is flagged in the report
/// rather than papered over. Reproducing it properly needs `repr(float)`, which
/// `currency.rs` deliberately keeps on the Python side.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    match val {
        CurrencyValue::Int(v) => Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => {
            let d = d.abs();
            if d.as_bigint_and_exponent().1 < 0 {
                // str(n) used exponent notation; int() on that token raises.
                return Err(N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    d
                )));
            }
            // int(parts[0]) — `d` is non-negative, so with_scale(0) truncating
            // toward zero is the integer part.
            let left = d.with_scale(0);
            // int(parts[1][:2].ljust(2, "0")) == floor(frac * 100).
            let frac = &d - &left;
            let right = (frac * BigDecimal::from(100)).with_scale(0);
            Ok((
                left.as_bigint_and_exponent().0,
                right.as_bigint_and_exponent().0,
            ))
        }
    }
}

pub struct LangBo {
    /// `Num2Word_BO.CURRENCY_FORMS`, built once in [`LangBo::new`].
    ///
    /// The binding holds each `LangBo` in a `OnceLock`, so this table is
    /// constructed exactly once per process rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangBo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangBo {
    pub fn new() -> Self {
        // Num2Word_BO.CURRENCY_FORMS — three codes, each with two identical
        // forms per side. Tibetan does not inflect for number, so the singular
        // and plural slots carry the same word; the arity is kept at Python's
        // 2 because `to_cheque` reads `cr1[-1]`.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "CNY",
            CurrencyForms::new(&["སྒོར་", "སྒོར་"], &["ཕན་", "ཕན་"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["ཌོ་ལར་", "ཌོ་ལར་"], &["སེན་", "སེན་"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["ཡུ་རོ་", "ཡུ་རོ་"], &["སེན་", "སེན་"]),
        );
        LangBo { currency_forms }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// དུང་ཕྱུར་ (10^8), so from 10^16 its multiplier would itself need དུང་ཕྱུར་
/// and the word would stack.
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(16))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl Lang for LangBo {
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
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "CNY"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " དང་ "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "ཚེག་"
    }

    /// Port of `Num2Word_BO.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input, strips it, peels a leading "-" into
    /// `ret = self.negword`, then looks for "."; `str(int)` never contains
    /// one, so integers always take the `else` branch:
    /// `return ret + self._int_to_word(int(n))`. The float branch of the same
    /// method is [`LangBo::to_cardinal_float`].
    ///
    /// The negword is concatenated with **no separator** — the space that
    /// separates it from the number is the literal's own trailing space.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (prefix, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", prefix, checked_int_to_word(&magnitude)?))
    }

    /// Port of `Num2Word_BO.to_ordinal`: `to_cardinal(number) + "པ་"`.
    ///
    /// No `verify_ordinal` guard — negatives and zero pass straight through.
    /// See bug note 1 in the module docs.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_BO.to_ordinal_num`: `str(number) + "པ"`.
    ///
    /// Overrides the base (which returns the value unchanged). The suffix
    /// carries no trailing tsheg, unlike `to_ordinal`'s — that asymmetry is
    /// in the Python source and is preserved. A negative keeps its ASCII
    /// minus: `to_ordinal_num(-1)` == "-1པ".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_NUM_SUFFIX))
    }

    /// Port of `Num2Word_BO.to_cardinal`, non-integer path.
    ///
    /// ```text
    /// def to_cardinal(self, number):
    ///     n = str(number).strip()
    ///     if n.startswith("-"):
    ///         n = n[1:]
    ///         ret = self.negword
    ///     else:
    ///         ret = ""
    ///     if "." in n:
    ///         left, right = n.split(".", 1)
    ///         ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///         ret += " ".join(self._int_to_word(int(d)) for d in right)
    ///         return ret
    ///     else:
    ///         return ret + self._int_to_word(int(n))
    /// ```
    ///
    /// The same method as [`LangBo::to_cardinal`]; the trait splits what
    /// Python keeps as one function, so this arm carries the `"."` branch and
    /// the `else` that a scientific-notation `str()` still falls into. Nothing
    /// from `Num2Word_Base.to_cardinal_float` runs — see the module docs for
    /// why `float2tuple`, its f64 artefacts and its banker's rounding are all
    /// irrelevant here.
    ///
    /// Points worth naming:
    ///
    /// 1. **`precision_override` is ignored, deliberately.** `Num2Word_BO` does
    ///    inherit a `precision` attribute (2), and the dispatcher does assign
    ///    to it for `precision=`, but nothing on this path ever reads it —
    ///    the digit count comes from `str(number)`. Verified against the live
    ///    interpreter: `num2words(2.675, lang="bo", precision=1)` and
    ///    `precision=5` both return the unmodified three-digit form. Honouring
    ///    the override here would invent behaviour Python does not have.
    /// 2. **The sign comes from the string, not the value.** Python tests
    ///    `n.startswith("-")`, which is true for `-0.0` (`repr` keeps the sign
    ///    bit) where `FloatValue::is_negative`'s `value < 0.0` is false. So
    ///    `-0.0` correctly keeps its negword.
    /// 3. **`negword` is used raw, not stripped** — `ret = self.negword`,
    ///    where `Num2Word_Base.to_cardinal_float` would insert
    ///    `negword.strip()` as a separate word. Both land on "མེད་ཆ་ " here,
    ///    the literal already ending in exactly one space, but that is a
    ///    coincidence of the literal rather than shared logic.
    /// 4. **One word per *character*, not per digit position.** The fractional
    ///    part is iterated as characters of `str()`, so it is as long as the
    ///    string says: `Decimal("1.10")` keeps its trailing zero ("one point
    ///    one zero") while the float `1.10` does not ("one point one").
    /// 5. **`left` is converted before `right` is inspected**, matching the
    ///    statement order, so a bad `left` raises before any fractional
    ///    character is looked at. `join` fully consumes the generator, so
    ///    within `right` the *first* bad character wins.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() — str() of a float or a Decimal never has
        // surrounding whitespace, so the strip is a no-op Python keeps for
        // the str-input path that never reaches this hook.
        let s = python_str(value);
        let n = s.trim();

        // if n.startswith("-"): n = n[1:]; ret = self.negword
        let (n, mut ret) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD.to_string()),
            None => (n, String::new()),
        };

        // `split(".", 1)` — the first "." only.
        let Some((left, right)) = n.split_once('.') else {
            // else: return ret + self._int_to_word(int(n))
            ret.push_str(&checked_int_to_word(&int_token(n)?)?);
            return Ok(ret);
        };

        // ret += self._int_to_word(int(left)) + " " + self.pointword + " "
        ret.push_str(&checked_int_to_word(&int_token(left)?)?);
        ret.push(' ');
        // POINTWORD is `setup()`'s literal, the same string `pointword()`
        // reports; Python reads the attribute this const stands for.
        ret.push_str(POINTWORD);
        ret.push(' ');

        // ret += " ".join(self._int_to_word(int(d)) for d in right)
        let mut fraction = Vec::new();
        for d in right.chars() {
            let mut buf = [0u8; 4];
            fraction.push(checked_int_to_word(&int_token(d.encode_utf8(&mut buf))?)?);
        }
        ret.push_str(&fraction.join(" "));
        Ok(ret)
    }

    /// Port of `Num2Word_BO.to_year`: `"སྤྱི་ལོ་" + to_cardinal(val)`.
    ///
    /// Python's `longval=True` parameter is accepted and then ignored, so the
    /// trait's single-argument signature loses nothing. Negative years put
    /// the era marker *before* the negword — see bug note 2 in the module
    /// docs.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", YEAR_PREFIX, cardinal))
    }

    // ---- float / Decimal entry routing --------------------------------
    //
    // `to_ordinal` / `to_ordinal_num` / `to_year` have no type guard in
    // Python, so a float or Decimal flows through them exactly as an int
    // does: the suffix / era prefix wraps the *full* decimal phrase (or, for
    // `to_ordinal_num`, the raw `str(number)`).

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "པ་"` — the
    /// full decimal phrase plus the suffix; the exponential-form ValueError
    /// from the cardinal propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "པ"` — no `int()`, so
    /// it succeeds where the other modes raise ("1e+16པ") and "-0.0" keeps
    /// its textual minus. `repr_str` is Python's `str(number)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_NUM_SUFFIX))
    }

    /// `to_year(float/Decimal)`: `"སྤྱི་ལོ་" + self.to_cardinal(val)` — era
    /// marker before the negword, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            YEAR_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which BO does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError; the
    /// binding's shared Inf sentinel would otherwise raise OverflowError.
    /// (NaN needs no interception: the binding's ValueError already matches
    /// `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    /// For `to_cheque`'s `'X' not implemented for 'Num2Word_BO'` message.
    fn lang_name(&self) -> &str {
        "Num2Word_BO"
    }

    /// `Num2Word_BO.CURRENCY_FORMS[code]`, with `None` for a missing code.
    ///
    /// Only the inherited `to_cheque` reads currency forms through this hook,
    /// and it wants the strict `CURRENCY_FORMS[currency]` lookup that raises
    /// `NotImplementedError` on a `KeyError`. BO's own `to_currency` uses the
    /// *lenient* `.get(currency, CURRENCY_FORMS["CNY"])` instead and so reads
    /// the field directly rather than going through here — see
    /// [`LangBo::to_currency`].
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_BO.to_currency`.
    ///
    /// ```text
    /// def to_currency(self, val, currency="CNY", cents=True,
    ///                 separator=" དང་ ", adjective=False):
    ///     try:
    ///         left, right, is_negative = self.parse_currency(val)
    ///     except AttributeError:
    ///         is_negative = False
    ///         if val < 0:
    ///             is_negative = True
    ///             val = abs(val)
    ///         left, right = self._split_currency(val)
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["CNY"])
    ///     left_str = self._int_to_word(int(left))
    ///     cents_str = self._int_to_word(int(right)) if cents and right else ""
    ///     result = left_str + " " + cr1[0]
    ///     if cents_str:
    ///         result += separator + cents_str + " " + cr2[0]
    ///     return self.negword + result if is_negative else result
    /// ```
    ///
    /// The `try` is dead — `parse_currency` exists nowhere, so the
    /// `AttributeError` branch always wins (see the module docs). The rest is
    /// a direct transcription. Points worth naming, all corpus-confirmed:
    ///
    /// 1. **This override never raises `NotImplementedError`.** The `.get`
    ///    fallback means an unknown code renders in CNY words, in flat
    ///    contradiction to the inherited `to_cheque`. `currency:GBP`,
    ///    `currency:JPY`, `currency:KWD`, `currency:BHD`, `currency:INR` and
    ///    `currency:CHF` all come out as CNY in the corpus.
    /// 2. **A float with zero cents drops the cents segment.** `cents_str` is
    ///    gated on `right` being *truthy*, so `1.0` — which `_split_currency`
    ///    reduces to `right = 0` — renders "གཅིག་ ཡུ་རོ་", with no cents at all.
    ///    `Num2Word_Base.to_currency` would emit "... and zero cents" here; BO
    ///    does not, and the int/float distinction is invisible in its output.
    ///    Both `1` and `1.0` give "གཅིག་ ཡུ་རོ་". The distinction is still
    ///    threaded through faithfully (see [`split_currency`]) because it is
    ///    load-bearing everywhere else and only *coincidentally* collapses here.
    /// 3. **`cents=False` omits the cents instead of making them terse.** The
    ///    base class switches to `_cents_terse` (digits); BO's `cents and right`
    ///    guard just suppresses the whole segment, so `_cents_terse` is
    ///    unreachable and the trait default is left alone.
    /// 4. **`adjective` is accepted and completely ignored.** BO declares the
    ///    parameter, never reads it, and inherits an empty `CURRENCY_ADJECTIVES`
    ///    anyway — hence `_adjective` below and no `currency_adjective` override.
    /// 5. **`negword` is used raw, not stripped.** `self.negword + result`,
    ///    where the base would do `"%s " % self.negword.strip()`. Both give
    ///    "མེད་ཆ་ " because the literal already ends in exactly one space.
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
        // Restore BO's own `separator=" དང་ "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // The `except AttributeError` branch: capture the sign, then split
        // abs(val). `split_currency` takes the absolute value itself.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // CURRENCY_FORMS.get(currency, CURRENCY_FORMS["CNY"]) — lenient, unlike
        // the strict lookup `to_cheque` performs through `currency_forms()`.
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CURRENCY))
            .expect("CURRENCY_FORMS always carries the CNY fallback entry");

        let left_str = checked_int_to_word(&left)?;
        // `cents and right` — a zero `right` is falsy, so no cents segment.
        // `_int_to_word` is never "" for a non-zero input, so testing the
        // Python string for truthiness is the same as testing this condition.
        let show_cents = cents && !right.is_zero();

        let mut result = format!("{} {}", left_str, forms.unit[0]);
        if show_cents {
            result.push_str(separator);
            result.push_str(&checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        Ok(if is_negative {
            format!("{}{}", NEGWORD, result)
        } else {
            result
        })
    }
}
