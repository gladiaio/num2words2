//! Port of `lang_HMN.py` (Hmong).
//!
//! Registry check: `CONVERTER_CLASSES["hmn"]` is `lang_HMN.Num2Word_HMN`
//! (`__init__.py:385`), which is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_HMN` subclasses `Num2Word_Base` but
//! defines no `high_numwords` / `mid_numwords` / `low_numwords`, so the
//! `hasattr` guard in `Num2Word_Base.__init__` never fires: `self.cards` is
//! never built and `self.MAXVAL` is never set. `to_cardinal` is overridden
//! outright and drives `_int_to_word` recursively. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check at all** — see bug 1 below for what happens instead.
//!
//! Every in-scope method is overridden by Python, so nothing is inherited
//! from `Num2Word_Base` except `__init__`'s attribute defaults, which
//! `setup()` then overwrites. `is_title` stays `False`, so `title()` is a
//! no-op and `exclude_title` (`["thiab", "lab", "tsis", "txaus"]`) is dead
//! weight — it is never consulted on any of the four in-scope paths.
//!
//! # Faithfully reproduced Python bugs and oddities
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits:
//!
//! 1. **`_int_to_word` falls through to `str(number)` at 10^9.** The chain of
//!    `if number < ...` guards stops at `1000000000`, and the final statement
//!    is a bare `return str(number)`. So `to_cardinal(10**9)` returns the
//!    *digit string* `"1000000000"`, not words — no exception, no overflow
//!    error. Corpus confirms this for 10^9 through 10^21. Negatives compose:
//!    `to_cardinal(-10**9)` == `"tsis txaus 1000000000"`, and
//!    `to_ordinal(10**9)` == `"thib 1000000000"`. This is why the value must
//!    stay a `BigInt` — the fallback has to print arbitrary precision exactly.
//!
//! 2. **The tens table mixes two spellings of the same morpheme.** 30–50 use
//!    "caug" ("peb caug", "plaub caug", "tsib caug") while 60–90 use "caum"
//!    ("rau caum", "xya caum", "yim caum", "cuaj caum"). Preserved verbatim.
//!
//! 3. **"tawm rau" for million, "lab" for the decimal point.** `self.million`
//!    is "tawm rau" (literally "out six"), while "lab" — the actual Hmong
//!    word for a million — is used as `pointword` instead. Both are kept as
//!    written; only `million` is in scope here.
//!
//! 4. **Inconsistent joiners.** Hundreds glue their remainder with `" thiab "`
//!    ("and"), but thousands and millions glue theirs with a plain space. So
//!    101 == "ib puas thiab ib" while 1001 == "ib txhiab ib".
//!
//! 5. **No scale word between thousand and million.** There is no "hundred
//!    thousand" grouping: 100000 == "ib puas txhiab" ("one hundred thousand"
//!    built as 100 × txhiab).
//!
//! 6. **`to_ordinal` accepts negatives and zero without complaint**, unlike
//!    most modules — it never calls `verify_ordinal`. `to_ordinal(0)` ==
//!    "thib xoom", `to_ordinal(-1)` == "thib tsis txaus ib".
//!
//! No cross-call mutable state: `setup()` only assigns constant tables, and
//! no method sets a flag that another consumes. The stateless Rust path is
//! faithful.
//!
//! # Float / Decimal cardinal path
//!
//! `Num2Word_HMN` does **not** override `to_cardinal_float`; its overridden
//! `to_cardinal` handles non-integers inline, entirely through `str(number)`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"):
//!     return (self.negword + self.to_cardinal(n[1:])).strip()
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret = self._int_to_word(int(left)) + " " + self.pointword
//!     for digit in right:
//!         ret += " " + (self.ones[int(digit)] or "xoom")
//!     return ret.strip()
//! return self._int_to_word(int(n))
//! ```
//!
//! It never touches `base.float2tuple`, the `< 0.01` rounding heuristic, or
//! `precision=`. So this port reconstructs `str(number)` — Python's `repr`
//! for a `float`, `Decimal.__str__` for a `Decimal` — and runs that string
//! algorithm verbatim. Consequences, all reproduced:
//!
//! 11. **The fractional part is taken digit-for-digit from `str(number)`, not
//!     rounded.** `2.675` -> "ob lab rau xya tsib" (digits 6-7-5 straight from
//!     the repr), `1.005` -> "ib lab xoom xoom tsib". A `0` digit prints
//!     "xoom" (`ones[0]` is "" -> `or "xoom"`); every other digit is `ones[d]`,
//!     which for a single digit equals `_int_to_word(d)`.
//!
//! 12. **`precision=` is ignored.** HMN's `to_cardinal` takes no precision
//!     argument and reads no `self.precision`, so the kwarg the facade threads
//!     through has no effect: `num2words(12.34, lang="hmn", precision=5)` ==
//!     `num2words(12.34, lang="hmn")`. `precision_override` is dropped here.
//!
//! 13. **Scientific-notation `str(number)` raises `ValueError`, never renders.**
//!     When `str` goes exponential, either there is no "." (`"1e+16"`,
//!     `"1E+3"`, `"1E-7"`) so `int(n)` chokes on the whole token, or the "."
//!     splits a mantissa that still runs into the exponent marker (`"1.5e+17"`
//!     -> `right == "5e+17"`, and `int("e")` fails in the digit loop). Both
//!     surface as `ValueError`. A float's repr goes scientific at a shortest
//!     decimal exponent `>= 16` or `<= -5` (Python's `decpt > 16 || decpt <=
//!     -4`); a `Decimal.__str__` goes scientific at `exponent > 0` or
//!     `adjusted < -6`. Reconstructing the exact `str` and feeding it through
//!     `int()` reproduces both the raise *and* the quoted literal.
//!
//! 14. **The sign comes from `str(number)`, which is why `-0.0` keeps its
//!     negword.** `str(-0.0)` is `"-0.0"`, so `startswith("-")` fires and the
//!     result is "tsis txaus xoom lab xoom" — even though `-0.0 < 0` is False.
//!     For a `float` the sign is `f64::is_sign_negative()` (the repr's "-").
//!     For a `Decimal` the sign rides on the `BigDecimal`; a *negative zero*
//!     `Decimal("-0.0")`/`Decimal("-0")` is the one input this port cannot
//!     reproduce — `BigDecimal` has no signed zero, so the "-" is gone before
//!     the core sees it. See `concerns`; it is pathological and unattested.
//!
//! # Currency
//!
//! `Num2Word_HMN` subclasses `Num2Word_Base` **directly**, not `Num2Word_EUR`,
//! and declares its own three-entry `CURRENCY_FORMS` class attribute. So the
//! `lang_EUR.py` mutation trap does not apply here: `Num2Word_EN.__init__`
//! rewrites `Num2Word_EUR.CURRENCY_FORMS` in place, but HMN never reads that
//! dict. Verified against the live interpreter — HMN sees exactly
//! `{USD, EUR, LAK}`, `CURRENCY_PRECISION == {}` and
//! `CURRENCY_ADJECTIVES == {}`.
//!
//! Consequences of those two empty dicts:
//!
//! * `currency_precision` stays at the trait default 100 for **every** code.
//!   HMN has no 3-decimal and no 0-decimal currency, so KWD/BHD render with a
//!   divisor of 100 and JPY still shows subunits. The corpus confirms it:
//!   `currency:KWD 12.34` and `currency:JPY 12.34` both come out identical to
//!   USD. Do not "fix" this by importing EN's precision table — EN *rebinds*
//!   `CURRENCY_PRECISION` on itself, so nothing leaks to HMN.
//! * `currency_adjective` stays `None`, and HMN's `to_currency` ignores its
//!   `adjective=` argument outright, so `adjective=True` is a silent no-op.
//!
//! `to_currency` is overridden wholesale and shares nothing with
//! `Num2Word_Base.to_currency` — see bugs 7-9. `to_cheque`, `_money_verbose`,
//! `_cents_verbose` and `_cents_terse` are all inherited unchanged, so the
//! trait defaults already mirror them and only the data tables, `lang_name`
//! and `pluralize` need overriding.
//!
//! ## More faithfully reproduced Python bugs
//!
//! 7. **`to_currency` never raises for an unknown code.** It does
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`,
//!    so GBP/JPY/KWD/BHD/INR/CNY/CHF silently borrow **USD's** forms and print
//!    "nyiaj kub"/"xees" instead of raising NotImplementedError. `to_cheque`,
//!    inherited from Base, does `self.CURRENCY_FORMS[currency]` and *does*
//!    raise — so the same code succeeds through one entry point and fails
//!    through the other. The corpus pins both halves. See [`FALLBACK_CODE`].
//!
//! 8. **Cents are truncated, not rounded.** `parts[1][:2]` slices the first two
//!    fractional characters off `str(val)`, so 2.675 -> 67 cents, and 1.005 ->
//!    0 cents (and therefore no cents segment at all). Base's `to_currency`
//!    would ROUND_HALF_UP to 68 and 1 respectively. HMN never calls
//!    `parse_currency_parts`, so none of that applies.
//!
//! 9. **`str(val).split(".")` mangles exponential `repr`s.** `str(1e16)` is
//!    "1e+16", which contains no ".", so `parts == ["1e+16"]` and `int()`
//!    raises `ValueError` — not any currency error. But a longer coefficient
//!    does not raise, it *lies*: `str(Decimal("1.01E+7"))` splits into
//!    `["1", "01E+7"]`, so ten million renders as "ib nyiaj kub ib xees" —
//!    one dollar one cent — with the exponent silently read as cents. Only 1-
//!    and 2-digit coefficients raise. See [`split_currency`], which also
//!    documents the one band this port cannot reproduce.
//!
//! 10. **Bug 1 leaks into currency.** `to_currency` calls `_int_to_word`, so a
//!     unit amount >= 10^9 prints as digits: `to_currency(10**9, "USD")` ==
//!     "1000000000 nyiaj kub".

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.negword`. The trailing space is load-bearing: it is the separator
/// between the negative marker and the number, since `to_cardinal`
/// concatenates without one.
const NEGWORD: &str = "tsis txaus ";

/// `self.ones`. Index 0 is `""`; `_int_to_word` guards it with `if o`/`n == 0`
/// on every path that could reach it, so the empty string never surfaces on
/// an integer path.
const ONES: [&str; 10] = [
    "", "ib", "ob", "peb", "plaub", "tsib", "rau", "xya", "yim", "cuaj",
];

/// `self.tens`. Note the "caug" (30–50) vs "caum" (60–90) split — bug 2.
const TENS: [&str; 10] = [
    "",
    "kaum",
    "nees nkaum",
    "peb caug",
    "plaub caug",
    "tsib caug",
    "rau caum",
    "xya caum",
    "yim caum",
    "cuaj caum",
];

/// `self.hundred`.
const HUNDRED: &str = "puas";
/// `self.thousand`.
const THOUSAND: &str = "txhiab";
/// `self.million` — "tawm rau", see bug 3.
const MILLION: &str = "tawm rau";

/// Zero, produced by `_int_to_word` rather than living in `ones`.
const ZERO_WORD: &str = "xoom";

/// The code whose forms an unknown currency silently borrows — bug 7.
///
/// Python spells this `list(self.CURRENCY_FORMS.values())[0]`: the *first
/// value* of the class dict, which since 3.7 means the first key in source
/// order. `CURRENCY_FORMS` is written `USD, EUR, LAK`, so the fallback is
/// USD's `(("nyiaj kub", "nyiaj kub"), ("xees", "xees"))` — confirmed against
/// the live class dict. A `HashMap` has no insertion order, so the identity of
/// that first entry is pinned here rather than left to iteration order.
const FALLBACK_CODE: &str = "USD";

/// `Num2Word_HMN.CURRENCY_FORMS`, verbatim.
///
/// Both sides of every entry carry exactly two forms, which is what makes the
/// `cr1[1]`/`cr1[0]` indexing in `to_currency` safe. The singular and plural
/// are identical throughout (Hmong does not inflect for number), so the
/// `left != 1` test is invisible in the output — it is ported anyway because
/// the table, not the test, is what a future edit would change.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "USD",
        CurrencyForms::new(&["nyiaj kub", "nyiaj kub"], &["xees", "xees"]),
    );
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["xees", "xees"]));
    m.insert("LAK", CurrencyForms::new(&["kib", "kib"], &["att", "att"]));
    m
}

/// Python's
/// ```python
/// val = abs(val)
/// parts = str(val).split(".")
/// left = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
/// reconstructed from the parsed value.
///
/// The binding hands the core `Decimal(str(value))`, so the digits and the
/// scale survive but the *rendering* does not. Rebuilding `str(val)` from
/// `(digits, scale)` recovers it for every plain-decimal repr, which is every
/// repr Python produces for `1e-4 <= |x| < 1e16`.
///
/// Values outside that window get a **scientific** repr, and Python splits
/// that string just as blindly — which is where this gets interesting. It is
/// not simply "exponential => raise":
///
/// | `str(val)` | `parts` | result |
/// |---|---|---|
/// | `"1E+3"` | `["1E+3"]` | `int("1E+3")` -> ValueError |
/// | `"1.5E+7"` | `["1", "5E+7"]` | `int("5E")` -> ValueError |
/// | `"1.01E+7"` | `["1", "01E+7"]` | left=1, right=1 -> "ib nyiaj kub ib xees" |
///
/// So a 3+-digit coefficient *succeeds*, reading the exponent as if it were
/// cents: `Decimal("1.01E+7")` — ten million — renders as **one dollar one
/// cent**. Only 1- and 2-digit coefficients raise. Verified against the live
/// interpreter for both `float` and `Decimal`, which agree throughout: the
/// mantissa is normalised to one leading digit by both `repr(float)` and
/// `Decimal.__str__`, and `parts[1][:2]` never reaches past it.
///
/// Deciding *whether* `str(val)` was scientific is the whole difficulty, since
/// the notation is chosen by the producer and then dissolved by
/// `CurrencyValue::parse`. Three of the four cases are recoverable:
///
/// * `scale < 0` — exponent > 0. A plain decimal string always parses to
///   `scale >= 0`, and a `float` only reaches a negative scale when its repr
///   already went scientific. Exact for both producers.
/// * `scale == 0` with `has_decimal` — a `Decimal` at scale 0 prints as a
///   plain integer (no dot, so `has_decimal` would be false), and a plain
///   `float` repr always carries at least ".0" (scale >= 1). So a dot at scale
///   0 can only be a scientific mantissa: `1.2345678901234568e+16`.
/// * `scale > 0` without `has_decimal` — no dot, yet a fraction exists. A
///   plain decimal string with fractional digits always has a dot, so this is
///   `Decimal("1E-7")`.
///
/// The fourth is **irreducibly ambiguous and this port diverges**. `float` and
/// `Decimal` disagree about when to go scientific — `repr(float)` switches
/// below 1e-4, `Decimal.__str__` only below 1e-6 (adjusted exponent < -6) —
/// so for `|x|` in `[1e-6, 1e-4)` the notation depends on a type that no
/// longer exists here. `1e-05` and `Decimal("0.00001")` both arrive as
/// `digits=1, scale=5, has_decimal=true`: identical, yet Python raises
/// ValueError for the first and returns "xoom nyiaj kub" for the second. The
/// exponent is present in the `str(value)` the shim sends and is consumed by
/// `parse` before this code runs, so nothing in this file can recover it.
/// This branch takes the `Decimal` reading. See `concerns`.
///
/// Both Err arms quote a literal (`'1e+21'`, `'5E'`) that the parsed value
/// cannot reproduce; the exception *type* is what a caller catches, and that
/// is exact.
///
/// On the plain path, `ljust(2, "0")` right-pads, so fractional trailing zeros
/// cannot matter: dropping them never changes the first two characters once
/// the slice is padded back out to two. `BigDecimal` happens not to normalize
/// on parse ("5.00" stays `digits=500, scale=2`), but this would hold either
/// way.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let (d, has_decimal) = match val {
        // str(int) never contains ".", so parts == [digits] and right == 0.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value, has_decimal, .. } => (value.abs(), *has_decimal),
    };

    // Python's Decimal holds (digits, exponent); BigDecimal holds
    // (int_val, scale) with exponent == -scale. value == digits * 10^-scale.
    let (digits, scale) = d.as_bigint_and_exponent();
    // `d` is non-negative, so this is a bare ASCII digit string.
    let s = digits.to_string();
    let ndigits = s.len() as i64;
    // Python's Decimal.adjusted(): exponent + len(digits) - 1.
    let adjusted = ndigits - 1 - scale;

    let scientific = if scale < 0 {
        true
    } else if scale == 0 {
        has_decimal
    } else if !has_decimal {
        true
    } else {
        // Decimal's threshold. A float would switch at `adjusted <= -5`, and
        // the two disagree on exactly [-6, -5] — the ambiguous band above.
        adjusted < -6
    };

    if scientific {
        // str(val) is "D.DD...E±k", or "DE±k" for a single-digit coefficient.
        // parts[0] is the leading digit; parts[1] is the rest of the
        // coefficient running straight into the exponent marker.
        if ndigits < 3 {
            // 1 digit: no "." at all, so int() gets the whole "1E+3".
            // 2 digits: parts[1][:2] is "5E", which int() rejects too.
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                d
            )));
        }
        // 3+ digits: parts[1][:2] lands entirely inside the coefficient.
        let left = s[0..1].parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
        let right = s[1..3].parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
        return Ok((left, right));
    }

    // No "." in str(d): parts == [str(d)], so right stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 -> "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    // `int(parts[0])` — a bare digit string, so this cannot fail.
    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

pub struct LangHmn {
    /// `CURRENCY_FORMS`. Built once here rather than per call: `get_lang`
    /// holds `LangHmn` in a `OnceLock`, so `new()` runs exactly once per
    /// process and every `to_currency` after that is a bare hash lookup.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangHmn {
    pub fn new() -> Self {
        LangHmn {
            currency_forms: build_currency_forms(),
        }
    }

    /// Python's `_int_to_word`.
    ///
    /// Only ever reached with a non-negative value: `to_cardinal` strips the
    /// sign before calling in, and every recursive call passes a remainder or
    /// quotient of a non-negative number. (Were it reached with a negative,
    /// Python's `self.ones[number]` would silently index from the *end* of the
    /// list — that path is unreachable on the four in-scope modes, so it is
    /// not modelled.)
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        // `if number < 10: return self.ones[number]`
        if number < &ten {
            return ONES[idx(number)].to_string();
        }

        // `t, o = divmod(number, 10)`
        if number < &hundred {
            let (t, o) = number.div_mod_floor(&ten);
            let mut out = TENS[idx(&t)].to_string();
            if !o.is_zero() {
                out.push_str(" thiab ");
                out.push_str(ONES[idx(&o)]);
            }
            return out;
        }

        // `h, r = divmod(number, 100)` — hundreds join with " thiab " (bug 4).
        if number < &thousand {
            let (h, r) = number.div_mod_floor(&hundred);
            let mut out = format!("{} {}", ONES[idx(&h)], HUNDRED);
            if !r.is_zero() {
                out.push_str(" thiab ");
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `t, r = divmod(number, 1000)` — thousands join with a bare space.
        if number < &million {
            let (t, r) = number.div_mod_floor(&thousand);
            let mut out = format!("{} {}", self.int_to_word(&t), THOUSAND);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `m, r = divmod(number, 1000000)` — millions also join with a space.
        if number < &billion {
            let (m, r) = number.div_mod_floor(&million);
            let mut out = format!("{} {}", self.int_to_word(&m), MILLION);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `return str(number)` — bug 1: digits, not words, from 10^9 up.
        number.to_string()
    }

    /// The string body of Python's `to_cardinal`, run on an already-built
    /// `str(number)` (`s`). Mirrors the three branches verbatim; the recursion
    /// on the sign strip is `self.to_cardinal(n[1:])`.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "xoom")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// `str(number)` is pure ASCII (digits, ".", "-", and — on the scientific
    /// branch — "e"/"E"/"+"), so byte slicing at the "." never splits a char.
    fn cardinal_from_str(&self, s: &str) -> Result<String> {
        // `n = str(number).strip()` — a no-op for a reconstructed repr, but run
        // it so a stray sign/space could not change the `startswith`/`split`.
        let s = s.trim();

        // `if n.startswith("-")` — the sign is textual, so `-0.0` (`str` ==
        // "-0.0") takes this branch even though it is not numerically < 0.
        if let Some(rest) = s.strip_prefix('-') {
            let inner = self.cardinal_from_str(rest)?;
            // `(self.negword + ...).strip()`. NEGWORD carries its own trailing
            // space; the outer strip is otherwise a no-op.
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        // `if "." in n:` — split on the FIRST ".", as `n.split(".", 1)` does.
        if let Some(dot) = s.find('.') {
            let left = &s[..dot];
            let right = &s[dot + 1..];
            // `self._int_to_word(int(left))` — `int()` raises ValueError on a
            // non-numeric left (a scientific mantissa never reaches here with
            // one, but a hand-built string could).
            let mut ret = format!("{} {}", self.int_to_word(&parse_pyint(left)?), self.pointword());
            for ch in right.chars() {
                // `self.ones[int(digit)] or "xoom"`. `int(digit)` is ValueError
                // for 'e'/'E'/'+'/'-', which is exactly how a scientific
                // mantissa such as "5e+17" raises.
                let d = pydigit(ch)?;
                ret.push(' ');
                ret.push_str(if d == 0 { ZERO_WORD } else { ONES[d as usize] });
            }
            // `return ret.strip()` — no-op, ret is never space-edged.
            return Ok(ret.trim().to_string());
        }

        // `return self._int_to_word(int(n))` — `int(n)` raises ValueError for a
        // no-dot scientific token like "1e+16" / "1E+3".
        Ok(self.int_to_word(&parse_pyint(s)?))
    }
}

impl Default for LangHmn {
    fn default() -> Self {
        Self::new()
    }
}

/// Index a 1-digit table. The callers have already proven the value is in
/// 0..=9 via the `< 10` / `divmod(_, 10)` guards, so the unwrap cannot fire.
fn idx(n: &BigInt) -> usize {
    n.to_usize().expect("guarded to 0..=9 by the caller")
}

/// Python's `int(s)` for the tokens `str(number)` can hand it. `str::parse`
/// matches `int()` on every one of them: a bare decimal string parses, and an
/// exponential token like "1e+16" / "1E+3" — plus the empty string — is
/// rejected. The message mirrors Python's `ValueError`; the quoted literal is
/// exact here because the token was reconstructed, not re-derived.
fn parse_pyint(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Python's `int(digit)` on a single character of the fractional string. ASCII
/// '0'..='9' yield their value; anything else ('e', 'E', '+', '-') is the
/// `ValueError` that a scientific mantissa's marker triggers in the digit loop.
fn pydigit(ch: char) -> Result<u32> {
    ch.to_digit(10).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

/// Reproduce Python's `repr(float)` == `str(float)`.
///
/// The sign is `is_sign_negative()`, so `-0.0` yields a leading "-" (Python's
/// `repr(-0.0)` is "-0.0") that `cardinal_from_str` then turns into the
/// negword. The magnitude is either a fixed string with `precision` fractional
/// digits — `precision` is the shortest-round-trip fractional length the shim
/// derived from Python's own `repr`, so `{:.precision}` reproduces those exact
/// digits — or, when the repr goes exponential, a rebuilt `m e±dd` token.
///
/// Python's `repr` switches to exponential at a shortest decimal exponent
/// `>= 16` or `<= -5` (its `decpt > 16 || decpt <= -4`, with `decpt` = that
/// exponent + 1). `format!("{:e}")` gives the same shortest mantissa and the
/// exponent to test. The rebuilt token keeps the mantissa and formats the
/// exponent as Python does — sign always shown, at least two digits — so both
/// the raise and the literal `int()` reports come out identical downstream.
fn float_repr(value: f64, precision: u32) -> String {
    let neg = value.is_sign_negative();
    let abs = value.abs();
    let body = if abs.is_finite() && abs != 0.0 {
        // "1.2345e3" / "1e-5" — LowerExp is shortest round-trip, like repr.
        let es = format!("{:e}", abs);
        let epos = es.find('e').expect("LowerExp of a finite nonzero has 'e'");
        let exp: i32 = es[epos + 1..]
            .parse()
            .expect("LowerExp exponent is an integer");
        if exp >= 16 || exp <= -5 {
            let mantissa = &es[..epos];
            let sign = if exp < 0 { "-" } else { "+" };
            format!("{}e{}{:02}", mantissa, sign, exp.abs())
        } else {
            format!("{:.*}", precision as usize, abs)
        }
    } else {
        // 0.0 -> "0.0" (precision is 1); inf/nan -> "inf"/"NaN", both of which
        // have no "." and fail `int()` exactly as Python's do.
        format!("{:.*}", precision as usize, abs)
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Reproduce Python's `Decimal.__str__` from the `BigDecimal` the shim parsed
/// out of `str(value)`. A direct port of CPython's `_pydecimal.Decimal.__str__`
/// (non-engineering branch): `_int` is the coefficient digit string
/// (`int_val.magnitude()`, which is "0" for a zero coefficient, exactly as
/// Python stores it), `_exp` is `-scale`.
///
/// The exponential form (`_exp > 0` or `adjusted < -6`, i.e. `leftdigits <=
/// -6`) is rebuilt with the `E±d` marker Python uses; feeding it through
/// `cardinal_from_str` then raises `ValueError` just like `int("1E+3")` does.
/// `BigDecimal` cannot hold a signed zero, so `Decimal("-0.0")` arrives with
/// its sign already gone — the one case this cannot reproduce.
fn decimal_repr(value: &bigdecimal::BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    let exp = -scale; // Python `_exp`
    let sign = if int_val.is_negative() { "-" } else { "" };
    let digits = int_val.magnitude().to_string(); // Python `_int`
    let ndigits = digits.len() as i64;
    let leftdigits = exp + ndigits;

    // `if self._exp <= 0 and leftdigits > -6: dotplace = leftdigits` else 1
    // (the `not eng` arm — HMN never asks for engineering notation).
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), digits),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", digits, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let dp = dotplace as usize;
        (digits[..dp].to_string(), format!(".{}", &digits[dp..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // `['e','E'][context.capitals]` is 'E' by default; `"%+d"`.
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

impl Lang for LangHmn {

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
    /// `"thawj"` when `number == 1`, else `"thib " + to_cardinal(number)`,
    /// for *any* input (no `verify_ordinal`). `number == 1` is *numeric*
    /// equality, so `1.0` and `Decimal("1.00")` both take the special word.
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let is_one = match value {
            FloatValue::Float { value: f, .. } => *f == 1.0,
            FloatValue::Decimal { value: d, .. } => d == &bigdecimal::BigDecimal::from(1),
        };
        if is_one {
            return Ok("thawj".to_string());
        }
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("thib {}", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `"thib " + str(number)` — no `== 1`
    /// special case here. `repr_str` is the dispatcher's exact `str(value)`
    /// (float repr / `Decimal.__str__`), so trailing zeros and `1E+2`-style
    /// exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("thib {}", repr_str))
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
        "USD"
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
        "lab"
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// ...
    /// return self._int_to_word(int(n))
    /// ```
    /// The `"." in n` branch is float-only and out of scope. `str(BigInt)`
    /// never yields "-0", leading whitespace or a "+" sign, so the round-trip
    /// through a string is unobservable for integer input.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // Mirrors Python's recursion on the sign-stripped digit string.
            let inner = self.to_cardinal(&value.abs())?;
            // `.strip()` is a no-op in practice (the concatenation is never
            // blank and never whitespace-edged), but it is what Python runs.
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// Python: `if number == 1: return "thawj"` else `"thib " + to_cardinal`.
    /// No `verify_ordinal` call, so zero and negatives pass through — bug 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("thawj".to_string());
        }
        Ok(format!("thib {}", self.to_cardinal(value)?))
    }

    /// Python: `return "thib " + str(number)`. Overriding is required — the
    /// trait default returns the bare digits with no "thib " prefix.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("thib {}", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is accepted and ignored; there is no BC/AD suffix and no
    /// two-part "nineteen eighty-four" split, so years read as plain
    /// cardinals: 1984 == "ib txhiab cuaj puas thiab yim caum thiab plaub".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal cardinal path. HMN has no `to_cardinal_float`; its
    /// overridden `to_cardinal` stringifies the number and walks `str(number)`
    /// char by char (see the module header). This reconstructs that exact
    /// string — `repr` for a float, `Decimal.__str__` for a Decimal — and runs
    /// the same walk, so `float2tuple`, the rounding heuristic and `precision=`
    /// are all bypassed just as Python bypasses them.
    ///
    /// `precision_override` is dropped: HMN's `to_cardinal` neither takes a
    /// `precision` argument nor reads `self.precision`, so `precision=` is a
    /// no-op there (bug 12).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let s = match value {
            FloatValue::Float { value, precision } => float_repr(*value, *precision),
            FloatValue::Decimal { value, .. } => decimal_repr(value),
        };
        self.cardinal_from_str(&s)
    }

    // ---- currency -------------------------------------------------------
    //
    // `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
    // inherited from `Num2Word_Base` untouched, and `CURRENCY_PRECISION` /
    // `CURRENCY_ADJECTIVES` are both empty, so `to_cheque`,
    // `money_verbose`, `cents_verbose`, `cents_terse`, `currency_precision`
    // and `currency_adjective` all stay at their trait defaults — those
    // already mirror Base exactly. `cardinal_from_decimal` stays at its
    // default too: HMN's `to_currency` truncates to whole cents before it ever
    // reaches a fractional-cents branch, so nothing can call it.

    fn lang_name(&self) -> &str {
        "Num2Word_HMN"
    }

    /// `CURRENCY_FORMS[code]`, returning `None` for a miss.
    ///
    /// The miss is what the *inherited* `to_cheque` turns into
    /// NotImplementedError. `to_currency` must **not** go through here — it
    /// swallows the miss and falls back to USD (bug 7), so it does its own
    /// lookup below.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_HMN.pluralize`:
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    /// Note `forms[-1]`, the *last* form — not `forms[1]` as `Num2Word_EUR`
    /// uses. The two agree only for two-element tuples, which is all HMN has.
    /// Empty `forms` returns "" rather than raising, so unlike most languages
    /// this one cannot produce an IndexError here.
    ///
    /// Dead on every path the corpus exercises: `to_currency` is overridden
    /// and indexes the tuple itself, and Base's `to_cheque` takes `cr1[-1]`
    /// directly. Ported because it is a real override — leaving the trait
    /// default in place would raise NotImplementedError where Python returns a
    /// word.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms.last() {
            None => Ok(String::new()),
            Some(last) => Ok(if n.is_one() {
                forms[0].clone()
            } else {
                last.clone()
            }),
        }
    }

    /// Python:
    /// ```python
    /// def to_currency(self, val, currency="USD", cents=True,
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
    /// A complete rewrite of Base's version, sharing none of its machinery:
    /// no `parse_currency_parts`, no `pluralize`, no `CURRENCY_PRECISION`, no
    /// ROUND_HALF_UP, and no NotImplementedError. The `int`/`float` split that
    /// Base leans on is invisible here — `str(val)` flattens both and
    /// `Decimal("5")`, `Decimal("5.00")`, `5` and `5.0` all land on
    /// "tsib nyiaj kub" — so `has_decimal` is deliberately unused.
    ///
    /// `adjective` is accepted and ignored, exactly as Python does.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `None` == the caller omitted `separator=`; resolve to HMN's own
        // `separator=" "` default before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is evaluated *before* `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // `.get(currency, list(...values())[0])` — bug 7: an unknown code
        // borrows USD's forms rather than raising.
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CODE))
            .expect("FALLBACK_CODE is inserted by build_currency_forms");

        let one = BigInt::one();
        // `cr1[1] if left != 1 else cr1[0]` — a raw tuple index, not
        // pluralize(). Python would raise IndexError on a 1-form entry; every
        // HMN entry has 2, so the Err is unreachable, but the type is kept.
        let unit = pick(&forms.unit, &left, &one)?;
        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `if cents and right:` — `right == 0` is falsy, so 1.0 and 5.00 lose
        // the cents segment entirely while 0.01 keeps it.
        if cents && !right.is_zero() {
            let subunit = pick(&forms.subunit, &right, &one)?;
            // Python concatenates the separator raw, contributing no space of
            // its own: an explicit separator="," gives "...nyiaj kub,peb caug".
            result.push_str(&format!(
                "{}{} {}",
                separator,
                self.int_to_word(&right),
                subunit
            ));
        }

        if is_negative {
            // NEGWORD's trailing space is the only thing separating it from
            // the amount.
            result = format!("{}{}", NEGWORD, result);
        }
        // `.strip()` — a no-op for every reachable input, but it is what
        // Python runs.
        Ok(result.trim().to_string())
    }
}

/// `forms[1] if n != 1 else forms[0]`, Python's raw tuple index.
fn pick(forms: &[String], n: &BigInt, one: &BigInt) -> Result<String> {
    let i = if n != one { 1 } else { 0 };
    forms
        .get(i)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

#[cfg(test)]
mod currency_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// Drive `to_currency` exactly as `num2words2-py`'s binding does: the
    /// Python side sends `str(val)` plus the two flags it alone can compute,
    /// and omits `separator=` (so `None`, which resolves to HMN's " ").
    ///
    /// `is_int` / `has_decimal` mirror the shim: `isinstance(val, int)` and
    /// `isinstance(val, float) or "." in str(val)`.
    fn cur(arg: &str, code: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let has_decimal = !is_int;
        let v = CurrencyValue::parse(arg, is_int, has_decimal, has_decimal).unwrap();
        LangHmn::new().to_currency(&v, code, true, None, false)
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangHmn::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    /// Every `"lang": "hmn", "to": "currency:*"` row in bench/corpus.jsonl.
    #[test]
    fn corpus_currency() {
        // The three codes HMN actually declares.
        for (code, unit, sub) in [
            ("USD", "nyiaj kub", "xees"),
            ("EUR", "euro", "xees"),
            // No corpus row (nothing asks for LAK), but it is in the table and
            // it is the only entry whose words are unique to it.
            ("LAK", "kib", "att"),
        ] {
            assert_eq!(cur("0", code).unwrap(), format!("xoom {}", unit));
            assert_eq!(cur("1", code).unwrap(), format!("ib {}", unit));
            assert_eq!(cur("2", code).unwrap(), format!("ob {}", unit));
            assert_eq!(cur("100", code).unwrap(), format!("ib puas {}", unit));
            assert_eq!(cur("1000000", code).unwrap(), format!("ib tawm rau {}", unit));
            assert_eq!(
                cur("12.34", code).unwrap(),
                format!("kaum thiab ob {} peb caug thiab plaub {}", unit, sub)
            );
            assert_eq!(cur("0.01", code).unwrap(), format!("xoom {} ib {}", unit, sub));
            // 1.0 is a float, yet the cents segment vanishes: right == 0.
            assert_eq!(cur("1.0", code).unwrap(), format!("ib {}", unit));
            assert_eq!(
                cur("99.99", code).unwrap(),
                format!(
                    "cuaj caum thiab cuaj {} cuaj caum thiab cuaj {}",
                    unit, sub
                )
            );
            assert_eq!(
                cur("1234.56", code).unwrap(),
                format!(
                    "ib txhiab ob puas thiab peb caug thiab plaub {} tsib caug thiab rau {}",
                    unit, sub
                )
            );
            assert_eq!(
                cur("-12.34", code).unwrap(),
                format!(
                    "tsis txaus kaum thiab ob {} peb caug thiab plaub {}",
                    unit, sub
                )
            );
            // "0.5" -> parts[1] == "5" -> "5".ljust(2,"0") == "50", not 5.
            assert_eq!(cur("0.5", code).unwrap(), format!("xoom {} tsib caug {}", unit, sub));
        }
    }

    /// Bug 7: every code HMN does *not* declare silently renders as USD.
    /// These are real corpus rows — none of them is a NotImplementedError.
    #[test]
    fn corpus_currency_unknown_code_falls_back_to_usd() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert_eq!(cur("0", code).unwrap(), "xoom nyiaj kub");
            assert_eq!(cur("1", code).unwrap(), "ib nyiaj kub");
            assert_eq!(cur("2", code).unwrap(), "ob nyiaj kub");
            assert_eq!(cur("100", code).unwrap(), "ib puas nyiaj kub");
            assert_eq!(cur("1000000", code).unwrap(), "ib tawm rau nyiaj kub");
            // KWD/BHD do *not* get divisor 1000 and JPY does *not* get divisor
            // 1: CURRENCY_PRECISION is empty, so all three behave like USD.
            assert_eq!(
                cur("12.34", code).unwrap(),
                "kaum thiab ob nyiaj kub peb caug thiab plaub xees"
            );
            assert_eq!(cur("0.01", code).unwrap(), "xoom nyiaj kub ib xees");
            assert_eq!(cur("1.0", code).unwrap(), "ib nyiaj kub");
            assert_eq!(
                cur("99.99", code).unwrap(),
                "cuaj caum thiab cuaj nyiaj kub cuaj caum thiab cuaj xees"
            );
            assert_eq!(
                cur("1234.56", code).unwrap(),
                "ib txhiab ob puas thiab peb caug thiab plaub nyiaj kub tsib caug thiab rau xees"
            );
            assert_eq!(
                cur("-12.34", code).unwrap(),
                "tsis txaus kaum thiab ob nyiaj kub peb caug thiab plaub xees"
            );
            assert_eq!(cur("0.5", code).unwrap(), "xoom nyiaj kub tsib caug xees");
        }
    }

    /// Every `"lang": "hmn", "to": "cheque:*"` row. Note the asymmetry with
    /// `to_currency`: the *same* unknown codes raise here.
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "IB TXHIAB OB PUAS THIAB PEB CAUG THIAB PLAUB AND 56/100 NYIAJ KUB"
        );
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "IB TXHIAB OB PUAS THIAB PEB CAUG THIAB PLAUB AND 56/100 EURO"
        );
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!(
                        "Currency code \"{}\" not implemented for \"Num2Word_HMN\"",
                        code
                    )
                ),
                other => panic!("{}: expected NotImplementedError, got {:?}", code, other),
            }
        }
    }

    /// Values not in the corpus, each checked against the live interpreter.
    #[test]
    fn python_parity_edge_cases() {
        // Truncation, not ROUND_HALF_UP: Base would say 68 and 1 here.
        assert_eq!(cur("2.675", "USD").unwrap(), "ob nyiaj kub rau caum thiab xya xees");
        assert_eq!(cur("1.005", "USD").unwrap(), "ib nyiaj kub");
        assert_eq!(
            cur("1.999", "USD").unwrap(),
            "ib nyiaj kub cuaj caum thiab cuaj xees"
        );
        // Anything below a cent truncates away — including the sign's amount,
        // which still keeps its negword.
        assert_eq!(cur("0.0001", "USD").unwrap(), "xoom nyiaj kub");
        assert_eq!(cur("-0.001", "USD").unwrap(), "tsis txaus xoom nyiaj kub");
        assert_eq!(cur("0.009", "USD").unwrap(), "xoom nyiaj kub");
        // -0.0 is not negative in Python either (`-0.0 < 0` is False).
        assert_eq!(cur("-0.0", "USD").unwrap(), "xoom nyiaj kub");
        assert_eq!(cur("0.0", "USD").unwrap(), "xoom nyiaj kub");
        assert_eq!(cur("-1", "USD").unwrap(), "tsis txaus ib nyiaj kub");
        assert_eq!(cur("1.01", "USD").unwrap(), "ib nyiaj kub ib xees");
        assert_eq!(cur("1.1", "USD").unwrap(), "ib nyiaj kub kaum xees");
        assert_eq!(cur("0.1", "USD").unwrap(), "xoom nyiaj kub kaum xees");
        // Decimal("5") and Decimal("5.00") both drop the cents segment, so
        // has_decimal genuinely does not matter to this language.
        let d5 = CurrencyValue::parse("5", false, false, false).unwrap();
        assert_eq!(
            LangHmn::new().to_currency(&d5, "USD", true, None, false).unwrap(),
            "tsib nyiaj kub"
        );
        let d500 = CurrencyValue::parse("5.00", false, true, true).unwrap();
        assert_eq!(
            LangHmn::new().to_currency(&d500, "USD", true, None, false).unwrap(),
            "tsib nyiaj kub"
        );
        // Bug 10: bug 1's digit fallback reaches the currency surface.
        assert_eq!(cur("1000000000", "USD").unwrap(), "1000000000 nyiaj kub");
        assert_eq!(cur("1000000000.0", "USD").unwrap(), "1000000000 nyiaj kub");
    }

    /// Bug 9, raising half: a 1- or 2-digit coefficient leaves `int()` holding
    /// "1e+21" or "5E", so it raises ValueError — not NotImplementedError, and
    /// not any currency error.
    ///
    /// `has_decimal` is the shim's `isinstance(val, float) or "." in str(val)`,
    /// so it is `true` for every float and `false` for a `Decimal` whose
    /// scientific repr has a single-digit coefficient — which is what makes
    /// `Decimal("1E-7")` detectable at all.
    #[test]
    fn exponential_repr_raises_value_error() {
        for (arg, has_decimal) in [
            ("1e+21", true),   // float 1e21          -> "1e+21", no dot
            ("1e+16", true),   // float 1e16          -> "1e+16", no dot
            ("-1e+16", true),  // float -1e16
            ("1.5e+17", true), // float 1.5e17        -> parts[1][:2] == "5e"
            ("1E+3", false),   // Decimal("1E+3")     -> "1E+3", no dot
            ("15E+3", false),  // Decimal("15E+3")    -> "1.5E+4", parts[1][:2] == "5E"
            ("1E-7", false),   // Decimal("1E-7")     -> "1E-7", no dot
            ("0E-7", false),   // Decimal("0E-7")     -> "0E-7", no dot
            ("1.5E-7", true),  // Decimal("1.5E-7")   -> "1.5E-7", parts[1][:2] == "5E"
        ] {
            let v = CurrencyValue::parse(arg, false, has_decimal, has_decimal).unwrap();
            match LangHmn::new().to_currency(&v, "USD", true, None, false) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{}: expected ValueError, got {:?}", arg, other),
            }
        }
    }

    /// Bug 9, lying half: a 3+-digit coefficient does *not* raise. Python
    /// splits "1.01E+7" into `["1", "01E+7"]` and reads the exponent as cents,
    /// so ten million comes out as one dollar one cent. `float` and `Decimal`
    /// agree here — both normalise the mantissa to one leading digit.
    #[test]
    fn exponential_repr_with_long_coefficient_misreads_as_cents() {
        for (arg, has_decimal, want) in [
            // Decimal("1.01E+7") == 10_100_000
            ("1.01E+7", false, "ib nyiaj kub ib xees"),
            // Decimal("1.10E+7"): trailing zero is kept, so right == 10.
            ("1.10E+7", false, "ib nyiaj kub kaum xees"),
            ("2.23E+11", false, "ob nyiaj kub nees nkaum thiab peb xees"),
            ("3.09E+9", false, "peb nyiaj kub cuaj xees"),
            // Coefficient longer than the slice: only the first two count.
            ("1.2345E+20", false, "ib nyiaj kub nees nkaum thiab peb xees"),
            // float 1.01e17 — the same shape, the same answer.
            ("1.01e+17", true, "ib nyiaj kub ib xees"),
            // float 1.2345678901234568e+16 — scale 0 with a dot, so the
            // mantissa rule applies even though the scale is not negative.
            ("1.2345678901234568e+16", true, "ib nyiaj kub nees nkaum thiab peb xees"),
        ] {
            let v = CurrencyValue::parse(arg, false, has_decimal, has_decimal).unwrap();
            assert_eq!(
                LangHmn::new().to_currency(&v, "USD", true, None, false).unwrap(),
                want,
                "{}",
                arg
            );
        }
    }

    /// The neighbours of the scientific-notation window, which must *not*
    /// raise. `Decimal("0.00001")` (adjusted exponent -5) still prints plain,
    /// so Python returns a word for it — unlike the float `1e-05`, which this
    /// port cannot tell apart. See `split_currency`.
    #[test]
    fn plain_reprs_near_the_exponential_window_do_not_raise() {
        // Decimal("1E-6") -> str "0.000001"; Decimal("0.00001") -> "0.00001".
        for arg in ["0.000001", "0.00001", "0.0001", "0.0"] {
            let v = CurrencyValue::parse(arg, false, true, true).unwrap();
            assert_eq!(
                LangHmn::new().to_currency(&v, "USD", true, None, false).unwrap(),
                "xoom nyiaj kub",
                "{}",
                arg
            );
        }
        // Decimal("5") / Decimal("1000"): no dot, but scale == 0, so they are
        // plain integer reprs and must not be mistaken for scientific ones.
        for (arg, want) in [("5", "tsib nyiaj kub"), ("1000", "ib txhiab nyiaj kub")] {
            let v = CurrencyValue::parse(arg, false, false, false).unwrap();
            assert_eq!(
                LangHmn::new().to_currency(&v, "USD", true, None, false).unwrap(),
                want
            );
        }
    }

    /// The kwargs HMN accepts. `adjective` is inert (CURRENCY_ADJECTIVES is
    /// empty *and* the body never reads it); `separator` is concatenated raw.
    #[test]
    fn kwargs() {
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        let l = LangHmn::new();
        assert_eq!(
            l.to_currency(&v, "USD", false, None, false).unwrap(),
            "kaum thiab ob nyiaj kub"
        );
        // An explicit separator brings no space of its own.
        assert_eq!(
            l.to_currency(&v, "USD", true, Some(","), false).unwrap(),
            "kaum thiab ob nyiaj kub,peb caug thiab plaub xees"
        );
        assert_eq!(
            l.to_currency(&v, "USD", true, None, true).unwrap(),
            "kaum thiab ob nyiaj kub peb caug thiab plaub xees"
        );
        assert_eq!(l.default_separator(), " ");
    }

    /// `forms[-1]`, not `forms[1]` — and "" rather than IndexError on empty.
    #[test]
    fn pluralize_is_last_form_not_second() {
        let l = LangHmn::new();
        let three: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        assert_eq!(l.pluralize(&BigInt::from(1), &three).unwrap(), "a");
        assert_eq!(l.pluralize(&BigInt::from(2), &three).unwrap(), "c");
        assert_eq!(l.pluralize(&BigInt::from(2), &[]).unwrap(), "");
        assert_eq!(l.pluralize(&BigInt::from(1), &[]).unwrap(), "");
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A `float` cardinal call: `precision` is the shortest-round-trip
    /// fractional length the shim derives from Python's `repr`, passed
    /// explicitly here (Rust's `Display` drops the trailing ".0", so it cannot
    /// stand in for repr on integer-valued floats like 0.0).
    fn f(value: f64, precision: u32) -> Result<String> {
        LangHmn::new().to_cardinal_float(&FloatValue::Float { value, precision }, None)
    }

    /// A `Decimal` cardinal call. `decimal_repr` reads `str(value)` back out of
    /// the `BigDecimal` alone, so `precision` is unused — pass 0.
    fn d(s: &str) -> Result<String> {
        LangHmn::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 0,
            },
            None,
        )
    }

    /// Every `"lang": "hmn", "to": "cardinal"` row with a dotted `arg`.
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(f(0.0, 1).unwrap(), "xoom lab xoom");
        assert_eq!(f(0.5, 1).unwrap(), "xoom lab tsib");
        assert_eq!(f(1.0, 1).unwrap(), "ib lab xoom");
        assert_eq!(f(1.5, 1).unwrap(), "ib lab tsib");
        assert_eq!(f(2.25, 2).unwrap(), "ob lab ob tsib");
        assert_eq!(f(3.14, 2).unwrap(), "peb lab ib plaub");
        assert_eq!(f(0.01, 2).unwrap(), "xoom lab xoom ib");
        assert_eq!(f(0.1, 1).unwrap(), "xoom lab ib");
        assert_eq!(f(0.99, 2).unwrap(), "xoom lab cuaj cuaj");
        assert_eq!(f(1.01, 2).unwrap(), "ib lab xoom ib");
        assert_eq!(f(12.34, 2).unwrap(), "kaum thiab ob lab peb plaub");
        assert_eq!(f(99.99, 2).unwrap(), "cuaj caum thiab cuaj lab cuaj cuaj");
        assert_eq!(f(100.5, 1).unwrap(), "ib puas lab tsib");
        assert_eq!(
            f(1234.56, 2).unwrap(),
            "ib txhiab ob puas thiab peb caug thiab plaub lab tsib rau"
        );
        assert_eq!(f(-0.5, 1).unwrap(), "tsis txaus xoom lab tsib");
        assert_eq!(f(-1.5, 1).unwrap(), "tsis txaus ib lab tsib");
        assert_eq!(f(-12.34, 2).unwrap(), "tsis txaus kaum thiab ob lab peb plaub");
        // The f64-artefact cases: taken digit-for-digit from repr, no rounding.
        assert_eq!(f(1.005, 3).unwrap(), "ib lab xoom xoom tsib");
        assert_eq!(f(2.675, 3).unwrap(), "ob lab rau xya tsib");
    }

    /// Every `"lang": "hmn", "to": "cardinal_dec"` row (Decimal input).
    #[test]
    fn corpus_cardinal_dec() {
        assert_eq!(d("0.01").unwrap(), "xoom lab xoom ib");
        assert_eq!(d("1.10").unwrap(), "ib lab ib xoom");
        assert_eq!(d("12.345").unwrap(), "kaum thiab ob lab peb plaub tsib");
        assert_eq!(d("98746251323029.99").unwrap(), "98746251323029 lab cuaj cuaj");
        assert_eq!(d("0.001").unwrap(), "xoom lab xoom xoom ib");
    }

    /// `str(-0.0)` is "-0.0", so the textual sign check keeps the negword even
    /// though `-0.0 < 0` is False (bug 14). The `float` sign survives as the
    /// IEEE sign bit; the `Decimal` one does not — see `neg_zero_decimal`.
    #[test]
    fn neg_zero_float_keeps_negword() {
        assert_eq!(f(-0.0, 1).unwrap(), "tsis txaus xoom lab xoom");
        assert_eq!(f(0.0, 1).unwrap(), "xoom lab xoom");
    }

    /// More Decimal cases checked against the live interpreter: trailing zeros
    /// in the coefficient are preserved by `Decimal.__str__`, and a negative
    /// integer-valued Decimal keeps its sign.
    #[test]
    fn decimal_str_parity() {
        assert_eq!(d("5.00").unwrap(), "tsib lab xoom xoom");
        assert_eq!(d("1.100").unwrap(), "ib lab ib xoom xoom");
        assert_eq!(d("0.10").unwrap(), "xoom lab ib xoom");
        assert_eq!(d("2.50").unwrap(), "ob lab tsib xoom");
        assert_eq!(d("-0.001").unwrap(), "tsis txaus xoom lab xoom xoom ib");
        assert_eq!(d("-12.34").unwrap(), "tsis txaus kaum thiab ob lab peb plaub");
        assert_eq!(d("100").unwrap(), "ib puas");
        assert_eq!(d("0.0").unwrap(), "xoom lab xoom");
        // Bug 1 leaks through the Decimal path too: 14-digit int part -> digits.
        assert_eq!(d("1000000000.5").unwrap(), "1000000000 lab tsib");
    }

    /// The plain neighbours of the scientific window must render, not raise.
    /// `Decimal.__str__` stays plain down to adjusted exponent -6.
    /// `Decimal("1E-6")` and `Decimal("0.000001")` are the same value and both
    /// print "0.000001"; `Decimal("1.00E-5")` prints "0.0000100" (7 fractional
    /// digits, the trailing zeros kept).
    #[test]
    fn decimal_plain_near_window() {
        assert_eq!(d("0.000001").unwrap(), "xoom lab xoom xoom xoom xoom xoom ib");
        assert_eq!(d("1E-6").unwrap(), "xoom lab xoom xoom xoom xoom xoom ib");
        assert_eq!(d("1E-5").unwrap(), "xoom lab xoom xoom xoom xoom ib");
        assert_eq!(
            d("1.00E-5").unwrap(),
            "xoom lab xoom xoom xoom xoom ib xoom xoom"
        );
    }

    /// A `float` whose repr goes exponential raises `ValueError` (bug 13):
    /// shortest decimal exponent `>= 16` or `<= -5`.
    #[test]
    fn scientific_float_raises_value_error() {
        for (v, p) in [
            (1e16, 16u32),
            (1e-5, 5),
            (1.5e17, 16),
            (9.999e-5, 5),
            (1e21, 21),
            (-1e16, 16),
        ] {
            match f(v, p) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{:e}: expected ValueError, got {:?}", v, other),
            }
        }
        // The immediate plain neighbours must NOT raise (exponent -4 / 15).
        assert_eq!(f(0.0001, 4).unwrap(), "xoom lab xoom xoom xoom ib");
        assert_eq!(
            f(1e15, 1).unwrap(),
            // 1e15 reprs plain as "1000000000000000.0" -> bug 1 digit fallback.
            "1000000000000000 lab xoom"
        );
    }

    /// A `Decimal` whose `str` goes exponential raises `ValueError` (bug 13):
    /// `exponent > 0` or `adjusted < -6`. `Decimal("0.0000001")` normalises to
    /// "1E-7", so it raises even though it was written in plain form.
    #[test]
    fn scientific_decimal_raises_value_error() {
        for s in ["1E+3", "1E-7", "1.5E+3", "0.0000001", "1E+2", "1E-8", "1.23E+11"] {
            match d(s) {
                Err(N2WError::Value(_)) => {}
                other => panic!("{}: expected ValueError, got {:?}", s, other),
            }
        }
    }

    /// The one input this port cannot reproduce: a *negative-zero* Decimal.
    /// Python's `str(Decimal("-0.0"))` is "-0.0" and keeps the negword, but
    /// `BigDecimal` has no signed zero, so the sign is gone before the core is
    /// called. Documented here as a known, unattested divergence (see
    /// `concerns`): the core returns the unsigned rendering.
    #[test]
    fn neg_zero_decimal_loses_sign_known_divergence() {
        // Python would say "tsis txaus xoom lab xoom" / "tsis txaus xoom".
        assert_eq!(d("-0.0").unwrap(), "xoom lab xoom");
        assert_eq!(d("-0").unwrap(), "xoom");
    }

    /// `precision=` is inert on HMN (bug 12): the override changes nothing,
    /// because `str(number)` carries its own fractional length.
    #[test]
    fn precision_override_ignored() {
        let v = FloatValue::Float {
            value: 12.34,
            precision: 2,
        };
        let l = LangHmn::new();
        assert_eq!(
            l.to_cardinal_float(&v, None).unwrap(),
            l.to_cardinal_float(&v, Some(5)).unwrap()
        );
        assert_eq!(
            l.to_cardinal_float(&v, Some(5)).unwrap(),
            "kaum thiab ob lab peb plaub"
        );
    }
}
