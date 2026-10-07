//! Port of `lang_KM.py` (Khmer).
//!
//! Registry check: `__init__.py` maps `"km": lang_KM.Num2Word_KM()`, so this
//! file ports `Num2Word_KM` — the only class the module defines.
//!
//! Shape: **self-contained**. `Num2Word_KM` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and never assigns
//! `self.MAXVAL`. `to_cardinal` is overridden outright and delegates to
//! `_int_to_word`, a hand-written cascade of magnitude branches. Consequently
//! `cards`/`merge` stay at their trait defaults here. Python has **no
//! overflow check at all** — see bug 1 below for what it did instead; this
//! port raises `OverflowError` from 10^18 (`maxval`).
//!
//! `setup()` sets only `negword = "ដក "` and `pointword = "ចំណុច"` plus the
//! `ones`/`tens` tables. `pointword` is unreachable for *integer* input (the
//! `"." in n` branch of `to_cardinal` never fires for an int), but it *is*
//! reached for float/Decimal input — see the float path below.
//!
//! # The float/Decimal path
//!
//! Unlike almost every other language, `Num2Word_KM` does **not** override
//! `to_cardinal_float` and does **not** use `base.float2tuple`. It overrides
//! `to_cardinal` outright and handles non-integers *inline* by string surgery
//! on `str(number)`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"): n = n[1:]; ret = self.negword
//! else: ret = ""
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     for digit in right: ret += self._int_to_word(int(digit)) + " "
//!     return ret.strip()
//! else:
//!     return (ret + self._int_to_word(int(n))).strip()
//! ```
//!
//! Because this reads `str(number)` digit by digit, it is **exact** where the
//! shared `float2tuple` path is lossy: `float2tuple` computes
//! `(value - pre) * 10**precision` in binary f64 and loses the last fractional
//! digit past ~12 places, whereas `str(float)` is the shortest round-trip
//! `repr`. The default trait `to_cardinal_float` therefore diverges from KM on
//! `-0.0` (KM prints the negword because `str(-0.0)` begins `"-"`) and on
//! high-precision floats (last-digit drift). Hence `to_cardinal_float` is
//! overridden here to reproduce the string-surgery cascade.
//!
//! Consequences reproduced verbatim:
//!
//! * **`precision=` is ignored.** KM's `to_cardinal` never reads
//!   `self.precision`; it reads `str(number)`. So the `precision_override`
//!   kwarg cannot change the output, and it is dropped here.
//! * **`-0.0` renders as negative.** `str(-0.0) == "-0.0"` starts with `"-"`,
//!   so the negword is prefixed. `value.is_sign_negative()` catches this f64
//!   negative zero, which `value < 0.0` would miss.
//! * **The million-space bug of `_int_to_word` carries over** (and so does
//!   the 10^18 ceiling) because the whole part is rendered through the very
//!   same function.
//!
//! ## Float routing: every float/Decimal goes through the string surgery
//!
//! Because `to_cardinal` branches on `"." in str(number)` — not on
//! `int(value) == value` — a *whole* float still reads its trailing zero:
//! `to_cardinal(5.0)` == `"ប្រាំ ចំណុច សូន្យ"`, and `Decimal("5.00")` reads two
//! zero digits. The base trait's `cardinal_float_entry` (whole → int path)
//! is therefore overridden to send **all** float/Decimal input through the
//! cascade. `to_ordinal`/`to_year` are plain prefixings of `to_cardinal`, so
//! their float entries prefix the same string-surgery output, and
//! `to_ordinal_num`'s float entry prefixes the binding-provided `str(value)`.
//!
//! ## Exponential reprs are reconstructed, and raise like Python
//!
//! `str(number)` is rebuilt here: `str(Decimal)` via
//! [`crate::strnum::python_decimal_str`] (exact spec port, so
//! `Decimal("1E+2")` yields `"1E+2"`), and `str(float)` via [`py_float_str`],
//! which mirrors CPython's shortest-repr formatting *including* the switch to
//! exponent form at `|v| >= 1e16` or `0 < |v| < 1e-4`. Feeding those strings
//! through the very same `int()` surgery reproduces Python's failures
//! verbatim: `int("1e+16")` / `int("1E+2")` raise `ValueError` (no `"."` in
//! the repr), and a mantissa-dotted form like `1.23e+16` dies on `int("e")`
//! inside the digit loop. Corpus rows pin `1e+16`, `1e+20`, `Decimal("1E+2")`
//! and `Decimal("1E+20")` on ValueError for cardinal/ordinal/year, while
//! `ordinal_num` echoes them (`"ទី1e+16"`).
//!
//! `Decimal("-0.0")` reaches this module as `FloatValue::Float { -0.0 }` (the
//! binding converts it: `BigDecimal` cannot carry a signed zero), and the
//! f64 sign bit keeps the negword — `"ដក សូន្យ ចំណុច សូន្យ"`, as pinned.
//!
//! ## String input: Infinity/NaN raise ValueError, not OverflowError
//!
//! `num2words("Infinity", lang="km")` parses to `Decimal("Infinity")` and
//! lands in this `to_cardinal`, where `str()` gives `"Infinity"` — no `"."` —
//! and `int("Infinity")` raises **ValueError**. The binding would otherwise
//! map `ParsedNumber::Inf` to the base path's OverflowError before any KM
//! code runs, so `str_to_number` intercepts Inf (and NaN, whose binding
//! message differs from `int("NaN")`'s) and raises the ValueError here.
//! Known gap (unpinned): Python's `to_ordinal_num(Decimal("Infinity"))`
//! would echo `"ទីInfinity"`; the entry-level interception raises instead.
//! The strings corpus pins Infinity/NaN under `to=cardinal` only.
//!
//! Every in-scope method is overridden by `Num2Word_KM`; nothing is inherited
//! from `Num2Word_Base` for `to_cardinal`/`to_ordinal`/`to_ordinal_num`/
//! `to_year`. In particular `verify_ordinal` is **never called**, so negative
//! ordinals do not raise — they render (bug 3).
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's final
//!    `else` of `_int_to_word` is `return str(number)`, so `to_cardinal(10**9)`
//!    was the bare digit string `"1000000000"`. Khmer says 10^9 as ពាន់លាន
//!    "thousand million" and 10^12 as លានលាន "million million" (Wikipedia
//!    "Khmer numerals"; Wiktionary ពាន់លាន, លានលាន), so the million branch
//!    now takes any multiplier below 10^12 — 10^9 == `"មួយពាន់ លាន"`,
//!    10^12 == `"មួយ លាន លាន"` — and `OverflowError` is raised from 10^18.
//!    កោដិ (10^7) is deliberately not used: it is ambiguous with 10^9
//!    colloquially.
//! 2. **Inconsistent spacing around the "million" word.** Every other
//!    magnitude word is glued to its multiplier with no space
//!    (`"មួយ" + "រយ"` -> `"មួយរយ"`), but the million branch is
//!    `self._int_to_word(millions_val) + " លាន"` — with a leading space. So
//!    `to_cardinal(10**6)` == `"មួយ លាន"` (spaced) while
//!    `to_cardinal(1000)` == `"មួយពាន់"` (unspaced). Preserved verbatim.
//! 3. **`to_ordinal` prefixes the negword-bearing cardinal**, giving
//!    `to_ordinal(-1)` == `"ទីដក មួយ"` — the ordinal marker "ទី" lands in
//!    front of the minus word rather than the number. Likewise
//!    `to_ordinal(10**9)` was `"ទី1000000000"` because of bug 1; it is
//!    `"ទីមួយពាន់ លាន"` now.
//! 4. **`to_year` ignores its `longval` argument entirely** and is just
//!    `"ឆ្នាំ " + to_cardinal(val)`. There is no BC/AD handling and no
//!    two-chunk year reading: `to_year(1984)` reads as the plain cardinal.
//!    Negative years keep the cardinal's negword, so `to_year(-500)` ==
//!    `"ឆ្នាំ ដក ប្រាំរយ"`.
//!
//! # Currency
//!
//! `Num2Word_KM` overrides `to_currency` outright but **not** `to_cheque`,
//! which splits the two surfaces apart:
//!
//! * `to_currency` looks its code up with `.get(currency, CURRENCY_FORMS["KHR"])`,
//!   so an unimplemented code silently renders in riel instead of raising.
//! * `to_cheque` is `Num2Word_Base`'s, which subscripts `CURRENCY_FORMS[currency]`
//!   and turns the `KeyError` into `NotImplementedError`.
//!
//! Hence `currency:GBP` -> "សូន្យ រៀល" but `cheque:GBP` -> NotImplementedError,
//! both confirmed by the corpus. `to_cheque` needs no override at all: its
//! `_money_verbose` is `self.to_cardinal`, matching the trait default, and
//! `.upper()` is a no-op on Khmer (the script is caseless), so
//! `default_to_cheque` already emits KM's bytes exactly.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both empty (inherited
//! unmodified from `Num2Word_Base`), so precision defaults to 100 — read only
//! by the cheque path — and `adjective` can never prefix anything.
//!
//! # Known gap: floats whose `repr` is exponential
//!
//! KM's `to_currency` is alone in doing string surgery on `str(val)` rather
//! than arithmetic, which makes it sensitive to a detail `CurrencyValue`
//! cannot carry: a float's *textual* form. The shim parses `str(value)` into a
//! `BigDecimal`, and that parse is lossy for exponential reprs — the original
//! spelling is gone by the time this file sees it. Divergences, none of which
//! the corpus exercises (its args are all plain decimals):
//!
//! | input | Python | here |
//! |---|---|---|
//! | `1e16` | ValueError (`int("1e+16")`) | ValueError — Display round-trips `"1e+16"`, so this one agrees by luck |
//! | `1e-05` | ValueError (`int("1e-05")`) | `"សូន្យ រៀល"` — Display normalises to `"0.00001"` |
//! | `1.2345678901234568e+17` | `"មួយ រៀល ម្ភៃបី សេន"` (left=1, right=23!) | the digit fallback, `"123456789012345680 រៀល"` |
//!
//! Closing this would require the shim to pass `str(value)` through verbatim
//! rather than pre-parsed, which is outside this file. Flagged rather than
//! papered over: reproducing Python's `repr(float)` here would be a second
//! shortest-round-trip formatter and a permanent source of drift, exactly what
//! `currency.rs`'s header warns against.
//!
//! # No cross-call mutable state
//!
//! `Num2Word_KM` stashes no flags between methods (no `_pending_ordinal`-style
//! handshake as in `lang_ES`). `setup()` only populates constant tables, and
//! `self.precision` is touched solely by float paths, which are out of scope.
//! The stateless Rust translation is therefore exact. The currency forms table
//! is built once in `new()` and only ever read.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.negword`, set in `setup()`. Note the **trailing space** — it is part
/// of the literal and is what separates it from the number, since KM's
/// `to_cardinal` concatenates `ret + word` directly rather than joining.
const NEGWORD: &str = "ដក ";

/// `self.ones`, indices 0..=9.
const ONES: [&str; 10] = [
    "សូន្យ",      // 0
    "មួយ",        // 1
    "ពីរ",        // 2
    "បី",         // 3
    "បួន",        // 4
    "ប្រាំ",       // 5
    "ប្រាំមួយ",     // 6
    "ប្រាំពីរ",     // 7
    "ប្រាំបី",      // 8
    "ប្រាំបួន",     // 9
];

/// `self.tens`, indices 0..=9. Index 0 is the empty string and is never read
/// (the `< 100` branch only runs for `number >= 20`, so `tens_val >= 2`).
const TENS: [&str; 10] = [
    "",           // 0
    "ដប់",        // 10
    "ម្ភៃ",        // 20
    "សាមសិប",     // 30
    "សែសិប",      // 40
    "ហាសិប",      // 50
    "ហុកសិប",     // 60
    "ចិតសិប",     // 70
    "ប៉ែតសិប",     // 80
    "កៅសិប",      // 90
];

/// The magnitude words, glued to their multiplier without a separator.
const HUNDRED: &str = "រយ";
const THOUSAND: &str = "ពាន់";
const TEN_THOUSAND: &str = "ម៉ឺន";
const HUNDRED_THOUSAND: &str = "សែន";
/// The million word carries a *leading space* at its use site — see bug 2.
const MILLION: &str = " លាន";

/// `to_ordinal`'s prefix, "ទី".
const ORDINAL_PREFIX: &str = "ទី";
/// `to_year`'s prefix, "ឆ្នាំ " (trailing space included, as in Python).
const YEAR_PREFIX: &str = "ឆ្នាំ ";

/// The default currency and the fallback `to_currency` reaches for when the
/// requested code is absent — see `to_currency`'s `.get(currency, ...)` note.
const KHR: &str = "KHR";

pub struct LangKm {
    /// `Num2Word_KM.CURRENCY_FORMS`, built once in `new()`.
    ///
    /// `Num2Word_KM` derives straight from `Num2Word_Base` (MRO is
    /// `[Num2Word_KM, Num2Word_Base, object]`), **not** from `Num2Word_EUR`,
    /// so the class dict that `Num2Word_EN.__init__` mutates at import time
    /// never reaches it. This table is exactly the three codes the source
    /// declares — verified against the live interpreter, which reports only
    /// `{EUR, KHR, USD}` and empty `CURRENCY_PRECISION`/`CURRENCY_ADJECTIVES`.
    ///
    /// Both unit forms and both subunit forms are identical in Khmer, but the
    /// 2-tuple arity is kept as Python has it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangKm {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::with_capacity(3);
        currency_forms.insert(KHR, CurrencyForms::new(&["រៀល", "រៀល"], &["សេន", "សេន"]));
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["ដុល្លារ", "ដុល្លារ"], &["សេន", "សេន"]),
        );
        currency_forms.insert("EUR", CurrencyForms::new(&["អឺរ៉ូ", "អឺរ៉ូ"], &["សេន", "សេន"]));
        LangKm { currency_forms }
    }

    /// Python's `int(s)` for the digit runs `to_currency` slices out of
    /// `str(val)`.
    ///
    /// Normally infallible — the operands come from `BigInt`/`BigDecimal`
    /// Display and are plain digits. It can still fail, and *must*: Python
    /// feeds `str(val)` to `int()` unguarded, so a float whose repr is
    /// exponential (`str(1e16) == "1e+16"`) raises `ValueError`.
    /// `BigDecimal`'s Display reproduces `"1e+16"` verbatim, so that row lands
    /// on the same `ValueError` here. The message mirrors CPython's.
    fn py_int(s: &str) -> Result<BigInt> {
        BigInt::from_str(s).map_err(|_| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                s
            ))
        })
    }

    /// `Num2Word_KM._int_to_word`.
    ///
    /// Reached only with a non-negative value from `to_cardinal` (which strips
    /// the sign off the *string* before calling `int()`), and recursion never
    /// produces a negative. The `number < 0` arm below is therefore dead for
    /// the four in-scope modes; it is kept because it is dead in Python too,
    /// and reproducing the cascade verbatim keeps the branch bounds honest.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_zero() {
            return Ok(ONES[0].to_string());
        }

        if number.is_negative() {
            // Dead on every path. `to_cardinal` strips the sign off the string
            // first, and `to_currency` — the other caller — does `val = abs(val)`
            // before it ever slices digits out, so both operands it derives are
            // non-negative. Mirrors `self.negword + self._int_to_word(abs(n))`,
            // and is dead in Python for the same reasons.
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&number.abs())?));
        }

        // Each guard below bounds `number`, so the digit extracted for an
        // ONES/TENS index is provably 0..=9 and the usize cast cannot fail.
        // The final `else` needs no cast — it echoes the digits.
        macro_rules! small {
            ($v:expr) => {
                $v.to_usize().expect("bounded by the enclosing guard")
            };
        }

        let ten = BigInt::from(10);
        let twenty = BigInt::from(20);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let ten_thousand = BigInt::from(10_000);
        let hundred_thousand = BigInt::from(100_000);
        let million = BigInt::from(1_000_000);

        if *number < ten {
            return Ok(ONES[small!(number)].to_string());
        }
        if *number == ten {
            return Ok(TENS[1].to_string());
        }
        if *number < twenty {
            // `self.tens[1] + self.ones[number - 10]`; number-10 is 1..=9.
            return Ok(format!("{}{}", TENS[1], ONES[small!(number - &ten)]));
        }
        if *number < hundred {
            // Python's `//` and `%` are floor-based, but `number` is positive
            // here so truncating and flooring agree; BigInt `/` and `%` are
            // truncating, hence identical results.
            let tens_val = number / &ten;
            let ones_val = number % &ten;
            if ones_val.is_zero() {
                return Ok(TENS[small!(&tens_val)].to_string());
            }
            return Ok(format!("{}{}", TENS[small!(&tens_val)], ONES[small!(&ones_val)]));
        }

        // The four "glued" magnitudes share one shape: ONES[digit] + word,
        // then `" " + recurse(remainder)` when the remainder is non-zero.
        // Kept as an explicit cascade to mirror the Python branch order.
        if *number < thousand {
            return self.glued(number, &hundred, HUNDRED);
        }
        if *number < ten_thousand {
            return self.glued(number, &thousand, THOUSAND);
        }
        if *number < hundred_thousand {
            return self.glued(number, &ten_thousand, TEN_THOUSAND);
        }
        if *number < million {
            return self.glued(number, &hundred_thousand, HUNDRED_THOUSAND);
        }

        {
            // The one branch that *recurses* on the multiplier and the one
            // that puts a space before its magnitude word. Python stopped at
            // a multiplier of 999 and returned `str(number)` beyond (bug 1);
            // the multiplier now runs up to the maxval check's 10^12 - 1, so
            // 10^9 reads "thousand million" and 10^12 "million million" (#147).
            let millions_val = number / &million;
            let remainder = number % &million;
            let mut result = format!("{}{}", self.int_to_word(&millions_val)?, MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            Ok(result)
        }
    }

    /// The shared body of the hundred / thousand / ten-thousand /
    /// hundred-thousand branches: `self.ones[n // unit] + word`, then
    /// `" " + self._int_to_word(n % unit)` if that remainder is truthy.
    ///
    /// The caller's guard bounds `n // unit` to 1..=9, so the ONES index is
    /// always valid — unlike the million branch, this never recurses on the
    /// multiplier.
    fn glued(&self, number: &BigInt, unit: &BigInt, word: &str) -> Result<String> {
        let digit = number / unit;
        let remainder = number % unit;
        let mut result = format!(
            "{}{}",
            ONES[digit.to_usize().expect("bounded by the enclosing guard")],
            word
        );
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&self.int_to_word(&remainder)?);
        }
        Ok(result)
    }

    /// The body of `Num2Word_KM.to_cardinal`, run on a rebuilt `str(number)`.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"): n = n[1:]; ret = self.negword
    /// else: ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right: ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// Working on the *string* keeps every Python failure mode intact: an
    /// exponential repr with no dot dies in `int("1e+16")` (ValueError, whole
    /// token in the message), one with a mantissa dot dies on the first
    /// non-digit of the fractional walk (`int("e")`), and `str(inf)`/`"NaN"`
    /// die in `int(...)` the same way. `split_once` is Python's
    /// `split(".", 1)`; only the first dot separates.
    fn cardinal_from_pystr(&self, s: &str) -> Result<String> {
        let n = s.trim();
        let (ret, n) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", n),
        };

        if let Some((left, right)) = n.split_once('.') {
            let mut out = String::from(ret);
            out.push_str(&self.int_to_word(&Self::py_int(left)?)?);
            out.push(' ');
            out.push_str(self.pointword());
            out.push(' ');
            // `for digit in right: ret += self._int_to_word(int(digit)) + " "`.
            // `int(digit)` on each char reproduces the ValueError verbatim
            // when `right` holds a non-digit (exponential mantissa's 'e').
            for ch in right.chars() {
                let d = Self::py_int(&ch.to_string())?;
                out.push_str(&self.int_to_word(&d)?);
                out.push(' ');
            }
            Ok(out.trim().to_string())
        } else {
            // `else:` branch — `(ret + self._int_to_word(int(n))).strip()`.
            Ok(format!("{}{}", ret, self.int_to_word(&Self::py_int(n)?)?)
                .trim()
                .to_string())
        }
    }
}

/// Python's `str(float)` for a finite-or-not f64, sign included.
///
/// CPython's `float.__str__` is the shortest round-trip repr, printed in
/// fixed notation while the decimal point sits in `-3 <= decpt <= 16` and in
/// exponent notation outside it — i.e. e-form iff `|v| >= 1e16` or
/// `0 < |v| < 1e-4` (`str(1e16)` == `"1e+16"`, `str(1e15)` ==
/// `"1000000000000000.0"`, `str(0.0001)` == `"0.0001"`, `str(1e-05)` ==
/// `"1e-05"`).
///
/// * **Fixed form** — `format!("{:.*}", precision, mag)`: `precision` is the
///   repr-derived fractional digit count the binding carried over
///   (`abs(Decimal(repr(v)).as_tuple().exponent)`), and correctly rounding
///   the *exact* f64 to that many places reproduces the shortest-repr digits
///   byte for byte (both sides are IEEE-754 doubles).
/// * **Exponent form** — Rust's `{:e}` is shortest-round-trip too
///   (`"1e16"`, `"1.23e-5"`); reformatted to Python's spelling: explicit
///   `+`, exponent zero-padded to two digits (`1e+16`, `1e-05`, `1e+100`).
/// * **inf/nan** — `str(float("inf"))` == `"inf"`, `str(float("nan"))` ==
///   `"nan"` (CPython prints NaN unsigned regardless of the sign bit).
///
/// The sign comes from the *bit*, not a `< 0` compare: `str(-0.0)` is
/// `"-0.0"`, which is exactly why KM renders the negword for negative zero.
fn py_float_str(value: f64, precision: u32) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    let neg = value.is_sign_negative();
    let mag = value.abs();
    let body = if mag.is_infinite() {
        "inf".to_string()
    } else if mag != 0.0 && !(1e-4..1e16).contains(&mag) {
        // repr picks exponent form. {:e} gives e.g. "1e16" / "9.99e-5".
        let s = format!("{:e}", mag);
        let (mant, exp) = s.split_once('e').expect("{:e} always contains 'e'");
        let e: i32 = exp.parse().expect("{:e} exponent is a valid i32");
        format!("{}e{}{:02}", mant, if e < 0 { '-' } else { '+' }, e.abs())
    } else {
        format!("{:.*}", precision as usize, mag)
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// `str(number)` for the value the dispatcher handed over: `repr(float)` via
/// [`py_float_str`], `str(Decimal)` via the spec port in `strnum` (which
/// keeps trailing zeros — `"5.00"` — and exponent forms — `"1E+2"`).
fn py_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, precision } => py_float_str(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

impl Default for LangKm {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#147): the million branch
/// composes 10^9 as "thousand million" and 10^12 as "million million"; a
/// multiplier of 10^12 would stack a third លាន, which no source attests.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(18))
}

impl Lang for LangKm {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "KHR"
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
        "ចំណុច"
    }

    /// `Num2Word_KM.to_cardinal`.
    ///
    /// Python works on `str(number).strip()`, detaches a leading `"-"`, and —
    /// because integer input never contains `"."` — falls into
    /// `(ret + self._int_to_word(int(n))).strip()`.
    ///
    /// Splitting the sign off the string is equivalent to `abs()` here:
    /// `str()` of an int yields at most one leading `"-"`, and there is no
    /// negative zero. The trailing `.strip()` is likewise a no-op for integers
    /// (`_int_to_word` never returns an empty or space-padded string, and
    /// NEGWORD's trailing space is always followed by a word), but it is
    /// applied anyway to keep the shape of the original.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };
        let out = format!("{}{}", ret, self.int_to_word(&value.abs())?);
        Ok(out.trim().to_string())
    }

    /// `"ទី" + self.to_cardinal(number)`. No `verify_ordinal`, so negatives
    /// and the >= 10^9 digit fallback both flow straight through — bug 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, self.to_cardinal(value)?))
    }

    /// `"ទី" + str(number)`. Note KM overrides the base's `to_ordinal_num`,
    /// which returns the bare value; the sign is kept, so
    /// `to_ordinal_num(-1)` == "ទី-1".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, value))
    }

    /// `"ឆ្នាំ " + self.to_cardinal(val)`. The `longval` parameter is accepted
    /// and ignored by Python — bug 4.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", YEAR_PREFIX, self.to_cardinal(value)?))
    }

    /// The float/Decimal cardinal path — a direct port of `Num2Word_KM.
    /// to_cardinal` run on the rebuilt `str(number)`, *not* `float2tuple`.
    ///
    /// `precision_override` is dropped on purpose: KM's `to_cardinal` reads
    /// `str(number)`, never `self.precision`, so `num2words(x, precision=n)`
    /// leaves the output unchanged (verified against the live interpreter).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_from_pystr(&py_str(value))
    }

    /// `to_cardinal(float/Decimal)`, whole values included. KM routes on
    /// `"." in str(number)`, never on `int(value) == value`, so a whole float
    /// still reads its point digits (`5.0` -> "ប្រាំ ចំណុច សូន្យ",
    /// `Decimal("5.00")` -> two zero digits) while a point-free `Decimal("5")`
    /// takes the integer reading — both fall out of the same string surgery,
    /// so *everything* is sent there, overriding the base whole -> int route.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `"ទី" + self.to_cardinal(number)` — the
    /// prefix lands in front of whatever the cardinal produced, negword and
    /// point digits included ("ទីដក សូន្យ ចំណុច សូន្យ" for -0.0), and the
    /// cardinal's ValueError on exponential reprs propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            ORDINAL_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ទី" + str(number)`. `repr_str` is
    /// the binding's Python `str(value)`, so `"ទី1e+16"` / `"ទី5.00"` echo
    /// exactly — no conversion, no ValueError.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, repr_str))
    }

    /// `to_year(float/Decimal)`: `"ឆ្នាំ " + self.to_cardinal(val)`, same
    /// full-cardinal routing (and the same ValueError pass-through).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            YEAR_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which KM does not
    /// override. The Inf/NaN interception reproduces what happens *next* on
    /// the pinned path: `to_cardinal(Decimal("Infinity"))` reads
    /// `str(number)` == "Infinity" (canonical capitalization, minus sign
    /// already peeled by the `startswith("-")` branch), finds no ".", and
    /// dies in `int("Infinity")` with ValueError; NaN dies in `int("NaN")`.
    /// The binding otherwise maps `ParsedNumber::Inf` to the base integer
    /// path's OverflowError (and NaN to a differently-worded ValueError)
    /// before any KM code runs, so the exact errors must be raised here.
    ///
    /// Known gap (unpinned): Python's `to_ordinal_num(Decimal("Infinity"))`
    /// echoes `"ទីInfinity"`, which this entry-level interception turns into
    /// the cardinal path's ValueError. The strings corpus pins Infinity/NaN
    /// under `to=cardinal` only.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_KM"
    }

    /// `CURRENCY_FORMS[code]` — a *strict* lookup that returns `None` for an
    /// unknown code.
    ///
    /// Note the asymmetry with `to_currency` below, which is not an oversight
    /// but the whole point: `to_currency` does `.get(currency, ...KHR)` and so
    /// never fails, while `to_cheque` (inherited from `Num2Word_Base`) does a
    /// bare `self.CURRENCY_FORMS[currency]` and converts the `KeyError` into
    /// `NotImplementedError`. This hook backs the *cheque* path, so it must
    /// keep the strict semantics; the fallback lives in `to_currency` alone.
    /// That is why `currency:GBP` renders in riel while `cheque:GBP` raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_KM.to_currency` — overridden wholesale, sharing nothing with
    /// `Num2Word_Base.to_currency`.
    ///
    /// The Python body is string surgery on `str(val)`, not arithmetic:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Four consequences, all reproduced verbatim:
    ///
    /// 1. **`CURRENCY_PRECISION` is never consulted.** There is no divisor and
    ///    no quantize, so the 3-decimal (KWD/BHD) and 0-decimal (JPY) currencies
    ///    get no special handling whatsoever — `12.34` is "twelve X thirty-four
    ///    sen" for every code alike. Because this method replaces the base's,
    ///    the `divisor == 1` pre-rounding in `default_to_currency` is bypassed
    ///    too. Corpus rows for KWD/BHD/JPY confirm all three.
    /// 2. **`[:2].ljust(2, "0")` reads the fraction positionally**, so `0.5`
    ///    is 50 sen, not 5, and a third decimal is silently truncated
    ///    (`1.239` -> 23 sen) rather than rounded.
    /// 3. **`if cents and right:` drops a zero cents segment.** `1.0` is a
    ///    float, so `Num2Word_Base` would render "one X zero sen" — but here
    ///    `right == 0` is falsy and the segment vanishes, making `1.0` print
    ///    identically to the int `1`. The `Int`/`Decimal` split therefore does
    ///    not change this language's output, though it is still honoured below.
    /// 4. **`adjective` is accepted and ignored** — `CURRENCY_ADJECTIVES` is
    ///    empty for KM anyway, so no prefixing can occur.
    ///
    /// `cr1[0]`/`cr2[0]` are indexed directly rather than pluralized, so
    /// `pluralize` (abstract in `Num2Word_Base`) is never reached and stays at
    /// its raising default.
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
        // `if val < 0: is_negative = True; val = abs(val)`. Taking `abs` before
        // stringifying is what Python does, and it is why no operand below can
        // be negative. `-0.0` is not `< 0` in Python and is not negative here
        // either, and its `int("-0")` still lands on 0 — the paths agree.
        let is_negative = val.is_negative();

        // `str(val)`. The value reached Rust as `str(value)` parsed on the
        // Python side, and `BigDecimal`'s Display round-trips every plain
        // decimal string bit for bit, so re-rendering it reconstructs exactly
        // what Python's `str()` handed to `.split(".")`. (Floats whose repr is
        // exponential are the one gap — see the module concerns.)
        let text = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `.split(".")` then index [0]/[1]: only the first dot separates, and
        // Python's guards make a missing or empty side mean 0.
        let mut parts = text.splitn(2, '.');
        let whole_part = parts.next().unwrap_or("");
        let frac_part = parts.next();

        let left = if whole_part.is_empty() {
            BigInt::zero()
        } else {
            Self::py_int(whole_part)?
        };

        let right = match frac_part {
            // `if len(parts) > 1 and parts[1]` — an empty fraction ("12.") is
            // falsy and yields 0 rather than `int("")`.
            Some(frac) if !frac.is_empty() => {
                // `parts[1][:2].ljust(2, "0")`: take two *characters*, then pad
                // on the right (never the left) so "5" reads as 50 sen.
                let mut digits: String = frac.chars().take(2).collect();
                while digits.chars().count() < 2 {
                    digits.push('0');
                }
                Self::py_int(&digits)?
            }
            _ => BigInt::zero(),
        };

        // `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["KHR"])`.
        // Unknown codes silently become riel; only a missing "KHR" entry could
        // raise, and Python would raise KeyError there, so that is the variant
        // used. `new()` always inserts KHR, making this unreachable.
        let forms = self
            .currency_forms(currency)
            .or_else(|| self.currency_forms(KHR))
            .ok_or_else(|| N2WError::Key(format!("'{}'", KHR)))?;

        // `cr1[0]`/`cr2[0]`: always the first form, never pluralized. Both
        // entries are 2-tuples built in `new()`, so the index is provably safe.
        let mut result = format!("{} {}", self.int_to_word(&left)?, forms.unit[0]);

        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right)?);
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        // `(self.negword if is_negative else "") + result` — the raw negword,
        // trailing space and all. Unlike `Num2Word_Base.to_currency`, which
        // does `negword.strip() + " "`, KM concatenates it unmodified. Same
        // bytes here, but ported as written.
        Ok(if is_negative {
            format!("{}{}", NEGWORD, result)
        } else {
            result
        })
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;

    fn dec(s: &str) -> FloatValue {
        let bd = BigDecimal::from_str(s).unwrap();
        let precision = bd.as_bigint_and_exponent().1.unsigned_abs() as u32;
        FloatValue::Decimal { value: bd, precision }
    }

    #[test]
    fn whole_floats_keep_their_point_digits() {
        let km = LangKm::new();
        // "." in str(5.0) -> the float grammar, never the int path.
        assert_eq!(
            km.cardinal_float_entry(&FloatValue::Float { value: 5.0, precision: 1 }, None)
                .unwrap(),
            "ប្រាំ ចំណុច សូន្យ"
        );
        // str(-0.0) begins "-": the sign bit keeps the negword.
        assert_eq!(
            km.cardinal_float_entry(&FloatValue::Float { value: -0.0, precision: 1 }, None)
                .unwrap(),
            "ដក សូន្យ ចំណុច សូន្យ"
        );
        // Decimal("5.00") reads both trailing zeros; Decimal("5") does not.
        assert_eq!(
            km.cardinal_float_entry(&dec("5.00"), None).unwrap(),
            "ប្រាំ ចំណុច សូន្យ សូន្យ"
        );
        assert_eq!(km.cardinal_float_entry(&dec("5"), None).unwrap(), "ប្រាំ");
        // >= 10^9 whole part reads "thousand million" (#147, was digits).
        assert_eq!(
            km.cardinal_float_entry(
                &FloatValue::Float { value: 1_000_000_000.0, precision: 1 },
                None
            )
            .unwrap(),
            "មួយពាន់ លាន ចំណុច សូន្យ"
        );
    }

    #[test]
    fn float_entries_prefix_like_python() {
        let km = LangKm::new();
        let v = FloatValue::Float { value: -3.0, precision: 1 };
        assert_eq!(
            km.ordinal_float_entry(&v).unwrap(),
            "ទីដក បី ចំណុច សូន្យ"
        );
        assert_eq!(km.year_float_entry(&v).unwrap(), "ឆ្នាំ ដក បី ចំណុច សូន្យ");
        assert_eq!(km.ordinal_num_float_entry(&v, "-3.0").unwrap(), "ទី-3.0");
    }

    #[test]
    fn exponential_reprs_raise_value_error() {
        let km = LangKm::new();
        // str(1e16) == "1e+16": no ".", int("1e+16") -> ValueError.
        let e16 = FloatValue::Float { value: 1e16, precision: 16 };
        assert!(matches!(
            km.cardinal_float_entry(&e16, None),
            Err(N2WError::Value(_))
        ));
        // #211: str(Decimal("1E+2")) is written out ("100"), so it reads.
        assert_eq!(
            km.year_float_entry(&dec("1E+2")).unwrap(),
            km.to_year(&BigInt::from(100)).unwrap()
        );
        // ordinal_num echoes the repr instead of converting.
        assert_eq!(km.ordinal_num_float_entry(&e16, "1e+16").unwrap(), "ទី1e+16");
    }

    #[test]
    fn python_float_str_matches_cpython() {
        assert_eq!(py_float_str(5.0, 1), "5.0");
        assert_eq!(py_float_str(-0.0, 1), "-0.0");
        assert_eq!(py_float_str(1e15, 1), "1000000000000000.0");
        assert_eq!(py_float_str(1e16, 16), "1e+16");
        assert_eq!(py_float_str(1e20, 20), "1e+20");
        assert_eq!(py_float_str(0.0001, 4), "0.0001");
        assert_eq!(py_float_str(1e-5, 5), "1e-05");
        assert_eq!(py_float_str(f64::INFINITY, 0), "inf");
        assert_eq!(py_float_str(f64::NAN, 0), "nan");
    }

    #[test]
    fn infinity_string_raises_value_error() {
        let km = LangKm::new();
        assert!(matches!(km.str_to_number("Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(km.str_to_number("-Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(km.str_to_number("NaN"), Err(N2WError::Value(_))));
        assert!(matches!(km.str_to_number("1.5"), Ok(ParsedNumber::Dec(_))));
    }
}
