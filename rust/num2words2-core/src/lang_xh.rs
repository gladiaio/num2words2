//! Port of `lang_XH.py` (Xhosa).
//!
//! Shape: **self-contained**. `Num2Word_XH` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` guard in `Num2Word_Base.__init__` never fires: Python builds
//! neither `self.cards` nor `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives a hand-written `_int_to_word` recursion.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here,
//! and there is **no overflow check** — see bug 1 below for what happens
//! instead of an `OverflowError`.
//!
//! Inherited from `Num2Word_Base` and left alone by XH:
//!   * `is_title` stays `False` (set in `__init__`, never overridden), so
//!     `title()` is a no-op and the `exclude_title` list XH populates in
//!     `setup()` is dead weight. The trait default `is_title() -> false`
//!     reproduces this, so `exclude_title` is not modelled.
//!
//! XH overrides all four in-scope entry points (`to_cardinal`, `to_ordinal`,
//! `to_ordinal_num`, `to_year`), so no base-class conversion logic is reached.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong-looking but
//! are exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **10^9 and up (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` gives up at 10^9: the final `return str(number)` made
//!    `to_cardinal(10**9)` the decimal numeral "1000000000". This port adds
//!    `ibhiliyoni` (10^9; Wiktionary "billion" translations, and the SA
//!    National Treasury's Xhosa budget guide 2011, "I R2.8 bhiliyoni"),
//!    composed exactly like `isigidi` (`m > 1` guard, " na " joiner), and
//!    raises `OverflowError` from 10^12 (`maxval`), where no word is attested.
//! 2. **`to_ordinal` blindly prefixes "we"** to the cardinal for every value
//!    except exactly 1. This yields `to_ordinal(0) == "weiqanda"` and, for
//!    negatives, `to_ordinal(-1) == "wengaphantsi kwe nye"` — "we" glued onto
//!    the negative word "ngaphantsi" ("below"). Preserved verbatim.
//! 3. **`to_ordinal_num` does no formatting at all**: `"we" + str(number)`,
//!    so `to_ordinal_num(-1) == "we-1"` (sign kept, glued on).
//! 4. `self.ones[0]` is `""`. It is unreachable on the integer path
//!    (`_int_to_word` returns "iqanda" for 0 before the `< 10` branch), and is
//!    reachable only via the float path's `self.ones[int(digit)] or "qanda"`,
//!    where the empty string is falsy and the fraction digit 0 therefore
//!    renders as the filler "qanda" (a *different* word from the integer
//!    zero "iqanda"). See [`LangXh::to_cardinal_float`]. Kept in [`ONES`] to
//!    preserve the indexing.
//!
//! # Float / Decimal cardinal path
//!
//! XH does **not** use `Num2Word_Base.to_cardinal_float`/`float2tuple`. Its
//! overridden `to_cardinal` handles non-integers inline off `str(number)`:
//! split on the first ".", speak the integer part through `_int_to_word`, emit
//! the pointword "ichaphaza", then read each fraction *digit* left-to-right.
//! Because the digits come straight from `str(number)` there is no
//! banker's-rounding or f64-artefact rescue: `2.675` speaks its literal repr
//! digits 6,7,5, and `1.005` speaks 0,0,5. See [`LangXh::to_cardinal_float`].
//!
//! And because the route is the *string* form, it applies to every
//! float/Decimal, whole values included: `to_cardinal(5.0)` is
//! "ntlanu ichaphaza qanda", never "ntlanu" — Base's `int(value) == value`
//! whole-value shortcut does not exist here. [`LangXh::cardinal_float_entry`]
//! pins that full routing (`year_float_entry` inherits it, matching `to_year`
//! == `to_cardinal`); [`LangXh::ordinal_float_entry`] reproduces `to_ordinal`'s
//! `number == 1` test and "we" prefix on the same path (`to_ordinal(1.0)` ==
//! "okokuqala" because `1.0 == 1`, `to_ordinal(5.0)` == "wentlanu ichaphaza
//! qanda"); [`LangXh::ordinal_num_float_entry`] is `"we" + str(number)`, so
//! "we5.0" and even "we1e+16" are real outputs.
//!
//! # String inputs: Infinity/NaN punt to Python
//!
//! XH inherits Base's `str_to_number` (`Decimal(value)`), which parses
//! `"Infinity"`/`"NaN"` *successfully* — the failure only happens later,
//! inside XH's `to_cardinal` (`int("Infinity")` → ValueError; "-Infinity"
//! recurses through the negword branch first, so the message always quotes
//! 'Infinity') — while `to_ordinal_num("Infinity")` is *answered*, with
//! "weInfinity". The Rust dispatcher hard-codes Base semantics for non-finite
//! parses (OverflowError), which XH never executes, so
//! [`LangXh::str_to_number`] bounces them back to the pure-Python original
//! via NotImplemented; the fallback then raises (or returns) the byte-exact
//! original per mode.
//!
//! Because everything keys on `str(number)`, *scientific-notation* strings
//! crash the digit loop — a Python bug reproduced here (bug 10 below):
//!
//! 10. **Values whose `str()` uses exponent notation raise ValueError.**
//!     A float below 10^-4 reprs as `'1e-05'`/`'1.5e-05'`; a Decimal whose
//!     adjusted exponent is `< -6` prints as `'1E-7'`/`'1.5E-7'` (note
//!     `str(Decimal("0.0000001")) == "1E-7"` — Python *normalises* it into
//!     exponent form). With no "." the whole token hits `int()`
//!     (`ValueError: invalid literal for int() with base 10: '1e-05'`); with
//!     a "." the digit loop reaches the exponent marker and dies on
//!     `int('e')` (lowercase for floats, uppercase `int('E')` for Decimals).
//!     The reconstruction below therefore mirrors *both* of CPython's
//!     switchover rules so the same inputs fail with the same messages.
//!
//! # Currency
//!
//! XH overrides `to_currency` **wholesale** and shares almost nothing with
//! `Num2Word_Base`'s version — see [`LangXh::to_currency`]. It inherits
//! `to_cheque` unchanged, so the two disagree with each other about what an
//! unknown currency code means (bug 5 below).
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both left at
//! `Num2Word_Base`'s empty dicts, so `CURRENCY_PRECISION.get(code, 100)` is
//! *always* 100 and `adjective and currency in self.CURRENCY_ADJECTIVES` is
//! always false. The trait defaults already say exactly that, so neither hook
//! is overridden here. The practical effect: JPY is not treated as a 0-decimal
//! currency and KWD/BHD are not treated as 3-decimal ones — everything gets
//! two decimal places. The corpus pins this.
//!
//! # Faithfully reproduced Python bugs (currency)
//!
//! 5. **An unknown currency code silently becomes ZAR in `to_currency`, but
//!    raises in `to_cheque`.** XH's `to_currency` looks the code up with
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a *default*, not a `[]` — so `to_currency(1, "JPY")` is
//!    "nye randi", quietly denominated in rand. Inherited `to_cheque` still
//!    does `self.CURRENCY_FORMS[currency]` and raises NotImplementedError for
//!    the same code. Both halves are pinned by the corpus.
//! 6. **The plural form is chosen by direct tuple index, not `pluralize`.**
//!    `cr1[1] if left != 1 else cr1[0]` bypasses the `pluralize` XH defines,
//!    leaving that method dead code on every path (ported anyway, see
//!    [`LangXh::pluralize`]).
//! 7. **`adjective` is accepted and ignored.** It is not even read, so
//!    `adjective=True` changes nothing regardless of `CURRENCY_ADJECTIVES`.
//! 8. **Cents come from string slicing, so they truncate rather than round**:
//!    `int(parts[1][:2].ljust(2, "0"))`. `1.005` -> `"00"` -> 0 cents ->
//!    "nye randi" with no cents segment at all; `1.999` -> `"99"` -> 99.
//! 9. **The separator is glued straight onto the next word.** Python does
//!    `result += separator + self._int_to_word(right)`, with no space between
//!    them, so `separator=" na"` yields "...iyuro naamashumi amathathu...".
//!    Only the default `separator=" "` looks right.
//!
//! # Error variants
//!
//! * The four integer modes cannot raise: no `MAXVAL` check, no dict lookup,
//!   and every list index is bounded by an enclosing magnitude test. The
//!   unreachable `expect`s in [`digit`] document that reasoning.
//! * `to_currency` cannot raise either — the `.get` default removes the only
//!   dict lookup (see bug 5), and the plural index is provably in range for
//!   every entry in the table.
//! * `to_cheque` raises `NotImplemented` for a code outside the table, via the
//!   inherited `Num2Word_Base.to_cheque`. That is the only reachable error on
//!   the currency surface.
//!
//! # Cross-call mutable state
//!
//! None. `setup()` only assigns constant tables; no method sets a flag that
//! another consumes.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword`. The trailing space is load-bearing — Python concatenates
/// it directly (`self.negword + self.to_cardinal(...)`) rather than joining.
const NEGWORD: &str = "ngaphantsi kwe ";

/// `self.pointword`. Only reachable on the out-of-scope float path; kept so
/// the trait's `pointword()` reports what `setup()` actually installs.
const POINTWORD: &str = "ichaphaza";

/// `_int_to_word(0)`. Note this is a *different* word from the float path's
/// zero-digit filler "qanda".
const ZERO_WORD: &str = "iqanda";

/// `self.ones`. Index 0 is `""` in Python — see bug 4.
const ONES: [&str; 10] = [
    "", "nye", "mbini", "ntathu", "ne", "ntlanu", "ntandathu", "sixhenxe", "sibhozo", "lithoba",
];

/// `self.tens`. Index 0 is `""` and is unreachable (`number < 100` implies
/// `t >= 1` once `number >= 10`).
const TENS: [&str; 10] = [
    "",
    "lishumi",
    "amashumi amabini",
    "amashumi amathathu",
    "amashumi amane",
    "amashumi amahlanu",
    "amashumi amathandathu",
    "amashumi asixhenxe",
    "amashumi asibhozo",
    "amashumi alithoba",
];

const HUNDRED: &str = "ikhulu";
const THOUSAND: &str = "iwaka";
const MILLION: &str = "isigidi";
/// 10^9, which Python lacks (gladiaio/num2words2#147): Wiktionary
/// "billion" (Xhosa), SA National Treasury Xhosa budget guide 2011.
const BILLION: &str = "ibhiliyoni";

/// The exclusive ceiling (#147): 10^12 would need a word above `ibhiliyoni`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// Narrow a provably-small `BigInt` to a table index.
///
/// Every call site is guarded by an enclosing magnitude test that bounds the
/// value to 0..=9, so the `expect` is unreachable. It is an `expect` rather
/// than an `unwrap_or(0)` because a silent 0 would emit `""` and corrupt the
/// output instead of failing loudly.
fn digit(n: &BigInt) -> usize {
    n.to_usize()
        .expect("XH: table index provably in 0..=9 by the enclosing bound")
}

/// Python's `Num2Word_XH._int_to_word`.
///
/// Faithful to the original's cascade of magnitude tests. The input is always
/// non-negative: `to_cardinal` strips the sign before recursing, and every
/// internal recursion passes a quotient or remainder of a non-negative value.
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    if number.is_zero() {
        return Ok(ZERO_WORD.to_string());
    }

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);
    let one = BigInt::one();

    // if number < 10: return self.ones[number]
    if number < &ten {
        return Ok(ONES[digit(number)].to_string());
    }

    // if number < 100:
    //     t, o = divmod(number, 10)
    //     return self.tens[t] + (" ana " + self.ones[o] if o else "")
    if number < &hundred {
        let (t, o) = number.div_mod_floor(&ten);
        let mut out = TENS[digit(&t)].to_string();
        if !o.is_zero() {
            out.push_str(" ana ");
            out.push_str(ONES[digit(&o)]);
        }
        return Ok(out);
    }

    // if number < 1000:
    //     h, r = divmod(number, 100)
    //     base = (self.ones[h] + " " if h > 1 else "") + self.hundred
    //     return base + (" na " + self._int_to_word(r) if r else "")
    //
    // Note the asymmetry with the thousand/million branches below: the
    // hundreds branch indexes `self.ones[h]` directly instead of recursing,
    // which is safe only because `h` is bounded to 1..=9 here.
    if number < &thousand {
        let (h, r) = number.div_mod_floor(&hundred);
        let mut out = String::new();
        if h > one {
            out.push_str(ONES[digit(&h)]);
            out.push(' ');
        }
        out.push_str(HUNDRED);
        if !r.is_zero() {
            out.push_str(" na ");
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    // if number < 1000000:
    //     t, r = divmod(number, 1000)
    //     base = (self._int_to_word(t) + " " if t > 1 else "") + self.thousand
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if number < &million {
        let (t, r) = number.div_mod_floor(&thousand);
        let mut out = String::new();
        if t > one {
            out.push_str(&int_to_word(&t)?);
            out.push(' ');
        }
        out.push_str(THOUSAND);
        if !r.is_zero() {
            out.push_str(" na ");
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    // if number < 1000000000:
    //     m, r = divmod(number, 1000000)
    //     base = (self._int_to_word(m) + " " if m > 1 else "") + self.million
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if number < &billion {
        let (m, r) = number.div_mod_floor(&million);
        let mut out = String::new();
        if m > one {
            out.push_str(&int_to_word(&m)?);
            out.push(' ');
        }
        out.push_str(MILLION);
        if !r.is_zero() {
            out.push_str(" na ");
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    // Python: `return str(number)` -- bug 1: words ran out at 10^9. The
    // ibhiliyoni band instead, composed like the million one; the maxval
    // check above keeps `b` below 1000 (#147).
    let (b, r) = number.div_mod_floor(&billion);
    let mut out = String::new();
    if b > one {
        out.push_str(&int_to_word(&b)?);
        out.push(' ');
    }
    out.push_str(BILLION);
    if !r.is_zero() {
        out.push_str(" na ");
        out.push_str(&int_to_word(&r)?);
    }
    Ok(out)
}

// Reconstructing `str(number)` for a `FloatValue::Decimal` is
// [`python_decimal_str`] — CPython's `Decimal.__str__` (the spec's
// *to-scientific-string*), shared crate-wide. It preserves the written scale
// (`Decimal("1.10")` → `"1.10"`, `Decimal("1.5000")` → `"1.5000"`), flips to
// `'1E-7'` form once the adjusted exponent drops below -6 — including
// `Decimal("0.0000001")`, which Python *normalises* to `"1E-7"` (and XH then
// crashes on, bug 10) — and prints positive-exponent values the way Python
// does (`Decimal("1E+2")` → `"1E+2"`, again a bug-10 crash, corpus-pinned).
// The sign is stripped by the caller before the digit walk.

/// Reconstruct `str(abs(number))` for a `FloatValue::Float`.
///
/// CPython reprs a float in fixed notation only while its decimal exponent is
/// in `[-4, 16)`; outside that it switches to exponent form (`repr(1e-05) ==
/// '1e-05'`, `repr(1e16) == '1e+16'`), which XH's digit loop then chokes on
/// (bug 10). Inside the fixed window, `format!("{:.p}")` with the repr-derived
/// `precision` reproduces `str()` byte for byte — correctly rounding the
/// binary value to the repr's own digit count re-derives exactly those digits
/// (shortest-round-trip is unique at its own length), and it restores the
/// trailing ".0" that Rust's plain `{}` drops from whole floats.
///
/// In the exponent window the shortest digits are recovered from Rust's `{}`
/// (also shortest-round-trip, hence the same digits as Python's repr) and
/// re-assembled Python-style: one leading digit, "." only if more digits
/// follow, and a signed, two-digit-minimum exponent (`1.5e-05`, `5e-324`).
/// `precision` is irrelevant there — nothing survives to count digits with.
///
/// The `>= 1e16` arm is live: every f64 at that magnitude is whole, and since
/// [`LangXh::cardinal_float_entry`] routes whole floats through the string
/// path too, `1e16`/`1e20` arrive here and must come out as `"1e+16"`/
/// `"1e+20"` — no dot, so `int()` raises exactly Python's ValueError
/// (corpus-pinned across cardinal/ordinal/year). Non-finite values print as
/// Python does ('inf'/'nan' — Rust would say "NaN"), and then fail `int()`
/// with that token, as in Python; those are binding-unreachable.
fn float_unsigned_repr(abs: f64, precision: u32) -> String {
    if abs.is_nan() {
        return "nan".to_string();
    }
    if abs.is_infinite() {
        return "inf".to_string();
    }
    if abs != 0.0 && abs < 1e-4 {
        // '1e-05' side. Rust `{}` is always positional: "0." + zeros + digits.
        let s = format!("{}", abs);
        let frac = &s[2..];
        let zeros = frac.bytes().take_while(|&b| b == b'0').count();
        let sig = &frac[zeros..];
        let coeff = if sig.len() == 1 {
            sig.to_string()
        } else {
            format!("{}.{}", &sig[..1], &sig[1..])
        };
        return format!("{}e-{:02}", coeff, zeros + 1);
    }
    if abs >= 1e16 {
        // '1e+16' side: at this magnitude `{}` is all integer digits.
        let s = format!("{}", abs);
        let sig = s.trim_end_matches('0');
        let coeff = if sig.len() == 1 {
            sig.to_string()
        } else {
            format!("{}.{}", &sig[..1], &sig[1..])
        };
        return format!("{}e+{:02}", coeff, s.len() - 1);
    }
    format!("{:.*}", precision as usize, abs)
}

/// Python's `Num2Word_XH.to_cardinal` applied to an *unsigned* `str(number)`
/// (the sign is stripped and re-applied by the caller, mirroring the
/// `n.startswith("-")` recursion):
///
/// ```text
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "qanda")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// The trailing `.strip()` is a no-op here (`ret` neither starts nor ends with
/// whitespace), so it is dropped. `int(...)` on a non-numeric token raises
/// `ValueError` in Python, and scientific-notation `str()`s genuinely reach it
/// (bug 10): `'1e-05'` has no "." so the whole token hits the integer parse,
/// while `'1.5e-05'` splits and then dies in the digit loop on the exponent
/// marker — `int('e')` (or `int('E')` for a Decimal). Both error messages
/// mirror CPython's byte for byte, including which token they quote.
fn xh_string_to_words(n: &str) -> Result<String> {
    if let Some((left, right)) = n.split_once('.') {
        let left_int = left.parse::<BigInt>().map_err(|_| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", left))
        })?;
        let mut ret = int_to_word(&left_int)?;
        ret.push(' ');
        ret.push_str(POINTWORD);
        for ch in right.chars() {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
            })? as usize;
            ret.push(' ');
            // `self.ones[int(digit)] or "qanda"`: ONES[0] is "" (falsy) -> filler.
            if ONES[d].is_empty() {
                ret.push_str("qanda");
            } else {
                ret.push_str(ONES[d]);
            }
        }
        Ok(ret)
    } else {
        let v = n.parse::<BigInt>().map_err(|_| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", n))
        })?;
        Ok(int_to_word(&v)?)
    }
}

/// Python's `parts = str(abs(val)).split(".")` reduced to `(is_negative, left,
/// right)`, without reconstructing the string.
///
/// Python slices the *decimal representation*: `left = int(parts[0])` and
/// `right = int(parts[1][:2].ljust(2, "0"))`. For any value whose `str()` is
/// plain (non-exponential) decimal that is exactly:
///
/// * `left  == trunc(abs)` — `parts[0]` is the integer-part digits, and `abs`
///   is non-negative so truncation is floor;
/// * `right == trunc(frac(abs) * 100)` — taking the first two fraction digits
///   and right-padding with "0" to width two *is* a truncating scale-by-100.
///   `"5"` -> `"50"` -> 50 == trunc(0.5*100); `"004"` -> `"00"` -> 0 ==
///   trunc(0.004*100). Note this truncates where the rest of the library
///   rounds — bug 8.
///
/// `has_decimal` is exactly "`str(val)` contains a dot" for plain-form values,
/// which is the condition guarding `parts[1]`, so it selects the `right`
/// branch. A true `int` never has one, hence no cents for ints — matching
/// `Num2Word_Base`'s `isinstance(val, int)` split even though XH arrives there
/// by a different route.
///
/// The abs-before-str order matters and is preserved: Python computes
/// `val = abs(val)` *first*, so no "-" ever reaches the split.
fn split_value(val: &CurrencyValue) -> (bool, BigInt, BigInt) {
    match val {
        // str(int) has no ".", so parts has length 1 and `right` stays 0.
        CurrencyValue::Int(v) => (v.is_negative(), v.abs(), BigInt::zero()),
        CurrencyValue::Decimal { value, has_decimal, .. } => {
            let is_negative = value.is_negative();
            let abs = value.abs();
            // with_scale(0) truncates toward zero, i.e. Python's int().
            let left_dec = abs.with_scale(0);
            let left = left_dec.as_bigint_and_exponent().0;
            let right = if *has_decimal {
                let frac = &abs - &left_dec;
                (frac * BigDecimal::from(100))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0
            } else {
                BigInt::zero()
            };
            (is_negative, left, right)
        }
    }
}

/// Python's `forms[1] if n != 1 else forms[0]` — the direct tuple index XH
/// uses in place of `pluralize` (bug 6).
///
/// A one-element entry would raise IndexError for `n != 1`. Every entry in
/// [`LangXh`]'s table has two forms, so this is unreachable; it is mapped to
/// `Index` rather than panicking so the exception type survives if the table
/// ever changes.
fn indexed_form(forms: &[String], n: &BigInt) -> Result<String> {
    let i = if n.is_one() { 0 } else { 1 };
    forms
        .get(i)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

pub struct LangXh {
    /// `Num2Word_XH.CURRENCY_FORMS`, verbatim. Built once in [`LangXh::new`]
    /// and only read afterwards.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the default `to_currency`
    /// falls back to for an unknown code (bug 5).
    ///
    /// This depends on dict *iteration order*, which since Python 3.7 is
    /// insertion order, and ZAR is written first in the class body — confirmed
    /// against the live interpreter and pinned by the corpus's GBP/JPY/KWD
    /// rows, which all come out in rand. A `HashMap` has no order, so the
    /// choice is resolved here, once, instead of being left to iteration.
    fallback_forms: CurrencyForms,
}

impl Default for LangXh {
    fn default() -> Self {
        Self::new()
    }
}

impl LangXh {
    pub fn new() -> Self {
        // Declared as an ordered array rather than straight into the HashMap
        // because `fallback_forms` is defined by position, not by key.
        const ISENTI: [&str; 2] = ["isenti", "isenti"];
        let entries = [
            ("ZAR", CurrencyForms::new(&["randi", "randi"], &ISENTI)),
            ("USD", CurrencyForms::new(&["idola", "idola"], &ISENTI)),
            ("EUR", CurrencyForms::new(&["iyuro", "iyuro"], &ISENTI)),
        ];
        let fallback_forms = entries[0].1.clone();
        LangXh {
            currency_forms: entries.into_iter().collect(),
            fallback_forms,
        }
    }
}

impl Lang for LangXh {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ZAR"
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
        "ichaphaza"
    }

    /// Python:
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n: ...          # float path, out of scope
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// For integer input the string round-trip is inert: `str(BigInt)` never
    /// contains whitespace or a ".", and the negative branch's
    /// `to_cardinal(n[1:])` re-enters with the digits of `abs(number)` and
    /// falls straight through to `_int_to_word`. So the sign test collapses to
    /// `is_negative()` and the recursion to `int_to_word(abs)`.
    ///
    /// The trailing `.strip()` is a no-op here (`_int_to_word` never returns
    /// padded output and NEGWORD starts with 'n'), but is kept for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let words = int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, words).trim().to_string());
        }
        Ok(int_to_word(value)?)
    }

    /// Python:
    /// ```text
    /// if number == 1: return "okokuqala"
    /// return "we" + self.to_cardinal(number)
    /// ```
    /// See bug 2 — no guard for zero or negatives.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("okokuqala".to_string());
        }
        Ok(format!("we{}", self.to_cardinal(value)?))
    }

    /// Python: `return "we" + str(number)`. See bug 3.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("we{}", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is accepted and ignored, so years get no era/pair treatment.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// XH's float/Decimal cardinal path. `Num2Word_XH` does **not** override
    /// `to_cardinal_float`; its overridden `to_cardinal` handles non-integers
    /// inline off `str(number)`:
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "qanda")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// Everything keys on `str(number)`, so — unlike the shared
    /// `float2tuple` path — there is no `10**precision` binary arithmetic and no
    /// `< 0.01` rescue: `2.675` speaks its repr digits 6,7,5 and `1.005` speaks
    /// 0,0,5. The exact `str()` is reconstructed by [`float_unsigned_repr`]
    /// (fixed `{:.p}` inside repr's fixed window, Python-style `1e+16`/`1.5e-05`
    /// form outside it) and [`python_decimal_str`] (written scale preserved, or
    /// `1E+2`/`1.5E-7` exponent form exactly where CPython flips to it). The
    /// exponent forms contain no speakable digits — they exist so the same
    /// inputs raise the same `ValueError`s Python does (bug 10).
    ///
    /// The sign test is `str(number).startswith("-")`, so it keys on the sign
    /// *bit*, not `< 0`: `str(-0.0) == "-0.0"` earns the negword, matching the
    /// live interpreter's `num2words(-0.0, lang='xh')`. `-0.0` reaches XH's
    /// integer overload as an int and never comes here, so this only affects
    /// genuine `-0.0` floats.
    ///
    /// `precision_override` (the `precision=` kwarg) is ignored: XH's
    /// `to_cardinal` never reads `self.precision`, so the override — which the
    /// dispatcher only writes onto that attribute — cannot change its output.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let (is_negative, unsigned) = match value {
            FloatValue::Float { value, precision } => (
                value.is_sign_negative(),
                float_unsigned_repr(value.abs(), *precision),
            ),
            // `python_decimal_str` of the abs() — the sign is re-applied
            // below, mirroring Python's `n.startswith("-")` recursion.
            // (BigDecimal cannot carry Decimal("-0.0")'s sign anyway; the
            // binding reroutes that one value through the Float arm.)
            FloatValue::Decimal { value, .. } => (
                value.is_negative(),
                python_decimal_str(&value.abs()),
            ),
        };

        let body = xh_string_to_words(&unsigned)?;
        if is_negative {
            // Python: `(self.negword + <stripped body>).strip()`. NEGWORD keeps
            // its trailing space; the outer strip trims nothing here (body has
            // no edge whitespace), but is kept for fidelity.
            Ok(format!("{}{}", NEGWORD, body).trim().to_string())
        } else {
            Ok(body)
        }
    }

    // ---- float/Decimal routing --------------------------------------------

    /// `to_cardinal(float/Decimal)` full routing. Python's `to_cardinal` keys
    /// on `str(number)`, so *every* float/Decimal takes the string route —
    /// whole values included: 5.0 keeps its ".0" tail ("ntlanu ichaphaza
    /// qanda"), `Decimal("5.00")` speaks both zeros ("ntlanu ichaphaza qanda
    /// qanda"), and only a point-less string (`Decimal("5")` -> "5") lands on
    /// the integer branch, via the same `int(n)` call. Base's whole-value
    /// shortcut never runs.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`. Python:
    /// ```text
    /// if number == 1: return "okokuqala"
    /// return "we" + self.to_cardinal(number)
    /// ```
    /// The `== 1` test is *numeric*, so `1.0 == 1` and `Decimal("1.00") == 1`
    /// are both True and short-circuit to "okokuqala" before any string work
    /// (corpus: `to_ordinal(1.0)` == "okokuqala"). Everything else rides the
    /// full string route and takes the blind "we" prefix (bug 2), negatives
    /// included: `to_ordinal(-0.0)` == "wengaphantsi kwe iqanda ichaphaza
    /// qanda". An exponent-notation repr (`str(1e16)` == "1e+16",
    /// `str(Decimal("1E+2"))` == "1E+2") makes the inner `int()` raise its
    /// ValueError *before* Python's `"we" +` runs — the `?` reproduces that
    /// ordering. `as_whole_int` is exactly Python's numeric equality here:
    /// it yields the value iff the value is whole, sign included.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int().is_some_and(|i| i.is_one()) {
            return Ok("okokuqala".to_string());
        }
        Ok(format!("we{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `"we" + str(number)` never casts to
    /// int, so it *succeeds* on every float and Decimal — "we5.0", "we-0.0",
    /// "we1e+16" and "we1E+2" are all real Python outputs (bug 3's
    /// sign-and-repr passthrough extends to floats). `repr_str` is the
    /// binding's Python `str(value)`, exactly the string Python concatenates.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("we{}", repr_str))
    }

    // `year_float_entry` is deliberately NOT overridden: the trait default
    // routes to `cardinal_float_entry`, and Python's `to_year` is a plain
    // `return self.to_cardinal(val)` — identical by construction.

    /// `Decimal('Infinity')` / `-Infinity` from a string arg. XH inherits
    /// `Num2Word_Base`'s `str_to_number` (`Decimal(value)`), so the token parses
    /// *successfully* and only blows up later, inside each mode:
    ///
    /// * `to_cardinal` / `to_year` reach `int("Infinity")` → `ValueError`.
    ///   `-Infinity` recurses through the negword branch first, so the message
    ///   always quotes `'Infinity'`.
    /// * `to_ordinal` is `"okokuqala"` only for `number == 1` (false here), then
    ///   `"we" + self.to_cardinal(number)` → the `int()` raises → `ValueError`.
    /// * `to_ordinal_num` == `"we" + str(number)` never casts, so it *answers*:
    ///   `"weInfinity"` / `"we-Infinity"`.
    ///
    /// Byte-exact against the pure-Python oracle (verified live).
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "we{}Infinity",
                if negative { "-" } else { "" }
            )),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
        }
    }

    /// `Decimal('NaN')` from a string arg. Same shape as [`LangXh::inf_result`]:
    /// `int("NaN")` → `ValueError` on the cardinal/ordinal/year paths, while
    /// `to_ordinal_num` answers `"weNaN"`.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("weNaN".into()),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // XH overrides only `to_currency` and `pluralize`. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` are inherited from
    // `Num2Word_Base` untouched, and the trait defaults already mirror them —
    // in particular the default `money_verbose` calls `self.to_cardinal`,
    // which resolves to XH's override, exactly as Python's does.
    //
    // `currency_precision` and `currency_adjective` are deliberately NOT
    // overridden: both dicts are empty on XH, so the trait defaults (100, and
    // None) are already what Python computes. See the module header.
    //
    // `cardinal_from_decimal` is left at its default. It is reachable only
    // from `default_to_currency`'s fractional-cents branch, and XH replaces
    // `to_currency` outright, so nothing on this language can reach it.

    fn lang_name(&self) -> &str {
        "Num2Word_XH"
    }

    /// The raw `CURRENCY_FORMS[code]` lookup, i.e. the *strict* one.
    ///
    /// This backs the inherited `to_cheque`, which subscripts the dict and
    /// raises NotImplementedError on KeyError. `to_currency` does **not** go
    /// through here — it applies a `.get(code, <first value>)` fallback of its
    /// own (bug 5), so the two halves genuinely disagree.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python:
    /// ```text
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code in practice: the only caller in `Num2Word_Base` is
    /// `to_currency`, which XH replaces with a version that indexes the tuple
    /// directly (bug 6), and `to_cheque` takes `cr1[-1]` unconditionally. It is
    /// ported because Python defines it — the hook exists, so it should answer
    /// the way Python would.
    ///
    /// Note the empty-forms guard returns `""` rather than raising, so unlike
    /// most languages this override cannot produce an IndexError.
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
    /// ```text
    /// def to_currency(self, val, currency="ZAR", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (
    ///             cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// This shares nothing with `Num2Word_Base.to_currency` beyond the name:
    /// no `parse_currency_parts`, no `CURRENCY_PRECISION`, no `pluralize`, no
    /// `_money_verbose`/`_cents_verbose`, no NotImplementedError. It calls
    /// `_int_to_word` directly — note *not* `to_cardinal` — which is why a
    /// negative never double-prefixes the negword.
    ///
    /// The cents guard is `if cents and right`, testing `right` for
    /// *truthiness* only. `Num2Word_Base` instead asks `has_decimal or right >
    /// 0`, so where Base renders "five dollars, zero cents" for `5.0`, XH
    /// renders plain "ntlanu randi". `has_decimal` therefore only selects
    /// whether `right` gets computed at all (see [`split_value`]); it never
    /// forces a zero-cents segment.
    ///
    /// `separator: None` means the caller omitted the kwarg. The trait resolves
    /// that through `default_separator` (" " for XH) only inside its *default*
    /// body, which this override replaces — so the `unwrap_or` below is doing
    /// that job, not duplicating it. `adjective` is ignored, as in Python
    /// (bug 7).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        let separator = separator.unwrap_or(self.default_separator());
        let (is_negative, left, right) = split_value(val);

        // .get(currency, list(...)[0]) -- a default, not a raise. Bug 5.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        let mut result = format!(
            "{} {}",
            int_to_word(&left)?,
            indexed_form(&forms.unit, &left)?
        );

        if cents && !right.is_zero() {
            // Python concatenates `separator + word` with no space between
            // them -- bug 9. The space before cr2 is a separate literal.
            result.push_str(separator);
            result.push_str(&int_to_word(&right)?);
            result.push(' ');
            result.push_str(&indexed_form(&forms.subunit, &right)?);
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `.strip()`: inert here (NEGWORD starts with 'n', and no form is
        // empty), kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// Rebuild the `CurrencyValue` the binding would hand us for a corpus
    /// `arg`, which is `repr(value)`: no dot and no exponent means Python had
    /// an `int`, and `has_decimal` mirrors
    /// `isinstance(val, float) or "." in str(val)`.
    fn cur_sep(arg: &str, code: &str, cents: bool, sep: Option<&str>) -> String {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangXh::new()
            .to_currency(&v, code, cents, sep, false)
            .unwrap()
    }

    fn cur(arg: &str, code: &str) -> String {
        cur_sep(arg, code, true, None)
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangXh::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    /// The corpus's currency rows for the three codes XH actually defines.
    #[test]
    fn corpus_currency_known_codes() {
        for (code, unit) in [("EUR", "iyuro"), ("USD", "idola")] {
            assert_eq!(cur("0", code), format!("iqanda {unit}"));
            assert_eq!(cur("1", code), format!("nye {unit}"));
            assert_eq!(cur("2", code), format!("mbini {unit}"));
            assert_eq!(cur("100", code), format!("ikhulu {unit}"));
            assert_eq!(cur("1000000", code), format!("isigidi {unit}"));
            // 1.0 is a float, yet right == 0 -> no cents segment at all.
            assert_eq!(cur("1.0", code), format!("nye {unit}"));
            assert_eq!(
                cur("12.34", code),
                format!("lishumi ana mbini {unit} amashumi amathathu ana ne isenti")
            );
            // 0.01: left == 0 -> "iqanda", and right == 1 -> singular isenti.
            assert_eq!(cur("0.01", code), format!("iqanda {unit} nye isenti"));
            // 0.5: "5".ljust(2, "0") == "50", not 5.
            assert_eq!(
                cur("0.5", code),
                format!("iqanda {unit} amashumi amahlanu isenti")
            );
            assert_eq!(
                cur("99.99", code),
                format!(
                    "amashumi alithoba ana lithoba {unit} \
                     amashumi alithoba ana lithoba isenti"
                )
            );
            assert_eq!(
                cur("1234.56", code),
                format!(
                    "iwaka na mbini ikhulu na amashumi amathathu ana ne {unit} \
                     amashumi amahlanu ana ntandathu isenti"
                )
            );
            assert_eq!(
                cur("-12.34", code),
                format!("ngaphantsi kwe lishumi ana mbini {unit} amashumi amathathu ana ne isenti")
            );
        }
    }

    /// Bug 5: every code outside the table silently renders in rand, including
    /// the 0-decimal (JPY) and 3-decimal (KWD/BHD) ones — precision is never
    /// consulted, so 12.34 JPY still shows cents.
    #[test]
    fn corpus_currency_unknown_codes_fall_back_to_zar() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert_eq!(cur("0", code), "iqanda randi");
            assert_eq!(cur("1", code), "nye randi");
            assert_eq!(cur("2", code), "mbini randi");
            assert_eq!(cur("100", code), "ikhulu randi");
            assert_eq!(cur("1000000", code), "isigidi randi");
            assert_eq!(cur("1.0", code), "nye randi");
            assert_eq!(
                cur("12.34", code),
                "lishumi ana mbini randi amashumi amathathu ana ne isenti"
            );
            assert_eq!(cur("0.01", code), "iqanda randi nye isenti");
            assert_eq!(cur("0.5", code), "iqanda randi amashumi amahlanu isenti");
            assert_eq!(
                cur("99.99", code),
                "amashumi alithoba ana lithoba randi amashumi alithoba ana lithoba isenti"
            );
            assert_eq!(
                cur("1234.56", code),
                "iwaka na mbini ikhulu na amashumi amathathu ana ne randi \
                 amashumi amahlanu ana ntandathu isenti"
            );
            assert_eq!(
                cur("-12.34", code),
                "ngaphantsi kwe lishumi ana mbini randi amashumi amathathu ana ne isenti"
            );
        }
        // ZAR itself is the fallback, so it must agree with the unknown codes.
        assert_eq!(cur("12.34", "ZAR"), cur("12.34", "GBP"));
    }

    /// Behaviour beyond the corpus, checked against the live interpreter.
    #[test]
    fn python_parity_edge_cases() {
        // Bug 8: cents truncate. 1.005 -> "00" -> 0 -> no cents segment.
        assert_eq!(cur("1.005", "ZAR"), "nye randi");
        assert_eq!(
            cur("1.999", "ZAR"),
            "nye randi amashumi alithoba ana lithoba isenti"
        );
        assert_eq!(
            cur("12.349", "ZAR"),
            "lishumi ana mbini randi amashumi amathathu ana ne isenti"
        );
        // -0.0 < 0 is False in Python, so no negword.
        assert_eq!(cur("-0.0", "ZAR"), "iqanda randi");
        assert_eq!(cur("-1", "ZAR"), "ngaphantsi kwe nye randi");
        // Bug 1 leaked into currency (Python: "1000000000 randi"); the
        // ibhiliyoni band and the 10^12 ceiling reach it too (#147).
        assert_eq!(cur("1000000000", "ZAR"), "ibhiliyoni randi");
        // A Decimal with no dot in str() takes the no-cents branch; one with a
        // dot but a zero fraction still ends up with right == 0.
        let five = CurrencyValue::parse("5", false, false, false).unwrap();
        assert_eq!(
            LangXh::new().to_currency(&five, "ZAR", true, None, false).unwrap(),
            "ntlanu randi"
        );
        let five_00 = CurrencyValue::parse("5.00", false, true, true).unwrap();
        assert_eq!(
            LangXh::new().to_currency(&five_00, "ZAR", true, None, false).unwrap(),
            "ntlanu randi"
        );
        // cents=False drops the segment; bug 7: adjective changes nothing.
        assert_eq!(cur_sep("12.34", "EUR", false, None), "lishumi ana mbini iyuro");
        // Bug 9: separator is glued to the following word.
        assert_eq!(
            cur_sep("12.34", "EUR", true, Some(" na")),
            "lishumi ana mbini iyuro naamashumi amathathu ana ne isenti"
        );
    }

    /// Invoke the float cardinal path with a raw f64 and its repr-derived
    /// precision, exactly as the binding builds `FloatValue::Float`.
    fn card_float(value: f64, precision: u32) -> String {
        LangXh::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// Invoke the Decimal cardinal path. `precision` mirrors the binding's
    /// `abs(Decimal(str(value)).as_tuple().exponent)` — the fraction-digit count
    /// of the plain decimal string.
    fn card_dec(s: &str) -> String {
        let precision = s
            .split_once('.')
            .map(|(_, f)| f.len() as u32)
            .unwrap_or(0);
        let value = BigDecimal::from_str(s).unwrap();
        LangXh::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// The corpus's `to: "cardinal"` rows whose `arg` is a float (has a dot).
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(card_float(0.0, 1), "iqanda ichaphaza qanda");
        assert_eq!(card_float(0.5, 1), "iqanda ichaphaza ntlanu");
        assert_eq!(card_float(1.0, 1), "nye ichaphaza qanda");
        assert_eq!(card_float(1.5, 1), "nye ichaphaza ntlanu");
        assert_eq!(card_float(2.25, 2), "mbini ichaphaza mbini ntlanu");
        assert_eq!(card_float(3.14, 2), "ntathu ichaphaza nye ne");
        assert_eq!(card_float(0.01, 2), "iqanda ichaphaza qanda nye");
        assert_eq!(card_float(0.1, 1), "iqanda ichaphaza nye");
        assert_eq!(card_float(0.99, 2), "iqanda ichaphaza lithoba lithoba");
        assert_eq!(card_float(1.01, 2), "nye ichaphaza qanda nye");
        assert_eq!(card_float(12.34, 2), "lishumi ana mbini ichaphaza ntathu ne");
        assert_eq!(
            card_float(99.99, 2),
            "amashumi alithoba ana lithoba ichaphaza lithoba lithoba"
        );
        assert_eq!(card_float(100.5, 1), "ikhulu ichaphaza ntlanu");
        assert_eq!(
            card_float(1234.56, 2),
            "iwaka na mbini ikhulu na amashumi amathathu ana ne ichaphaza ntlanu ntandathu"
        );
        assert_eq!(card_float(-0.5, 1), "ngaphantsi kwe iqanda ichaphaza ntlanu");
        assert_eq!(card_float(-1.5, 1), "ngaphantsi kwe nye ichaphaza ntlanu");
        assert_eq!(
            card_float(-12.34, 2),
            "ngaphantsi kwe lishumi ana mbini ichaphaza ntathu ne"
        );
        // f64-artefact cases: str()-driven, so NO banker's rounding / rescue.
        // 1.005 speaks 0,0,5 and 2.675 speaks 6,7,5 (their literal repr digits).
        assert_eq!(card_float(1.005, 3), "nye ichaphaza qanda qanda ntlanu");
        assert_eq!(
            card_float(2.675, 3),
            "mbini ichaphaza ntandathu sixhenxe ntlanu"
        );
    }

    /// The corpus's `to: "cardinal_dec"` (Decimal input) rows, plus the trailing
    /// zero preservation the `float()` path would lose (issue #603).
    #[test]
    fn corpus_cardinal_decimal() {
        assert_eq!(card_dec("0.01"), "iqanda ichaphaza qanda nye");
        assert_eq!(card_dec("1.10"), "nye ichaphaza nye qanda");
        assert_eq!(card_dec("12.345"), "lishumi ana mbini ichaphaza ntathu ne ntlanu");
        // Bug 1 leaked in: Python printed the 10^13 integer part as digits;
        // it is past the 10^12 ceiling now (#147).
        assert!(matches!(
            card_dec_p("98746251323029.99", 2),
            Err(N2WError::Overflow(_))
        ));
        assert_eq!(card_dec("0.001"), "iqanda ichaphaza qanda qanda nye");
        // Beyond the corpus, checked against the live interpreter.
        assert_eq!(card_dec("1.5000"), "nye ichaphaza ntlanu qanda qanda qanda");
        assert_eq!(card_dec("-12.34"), "ngaphantsi kwe lishumi ana mbini ichaphaza ntathu ne");
    }

    /// `str(-0.0) == "-0.0"` starts with "-", so a genuine negative-zero float
    /// earns the negword — keyed on the sign bit, not `< 0`. Checked against
    /// `num2words(-0.0, lang='xh')`.
    #[test]
    fn negative_zero_float_gets_negword() {
        assert_eq!(card_float(-0.0, 1), "ngaphantsi kwe iqanda ichaphaza qanda");
        assert_eq!(card_float(0.0, 1), "iqanda ichaphaza qanda");
    }

    /// Invoke the float path expecting Python's ValueError.
    fn card_float_err(value: f64, precision: u32) -> N2WError {
        LangXh::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap_err()
    }

    /// Invoke the Decimal path with an explicit binding-side precision
    /// (`abs(Decimal(str(v)).as_tuple().exponent)`), needed where the plain
    /// string form has no "." to count from (e.g. "1E-7" → precision 7).
    fn card_dec_p(s: &str, precision: u32) -> Result<String> {
        LangXh::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision,
            },
            None,
        )
    }

    fn assert_value_err(err: N2WError, token: &str) {
        match err {
            N2WError::Value(msg) => assert_eq!(
                msg,
                format!("invalid literal for int() with base 10: '{token}'")
            ),
            other => panic!("expected ValueError, got {other:?}"),
        }
    }

    /// Bug 10: exponent-notation `str()`s raise ValueError, exactly as pure
    /// Python does (verified live with the Rust fast path disabled). The
    /// quoted token depends on whether the string has a "." — the whole repr
    /// when it does not, the exponent marker 'e'/'E' when it does. Precisions
    /// are the binding's `abs(Decimal(str(v)).as_tuple().exponent)` values.
    #[test]
    fn scientific_notation_str_raises_valueerror() {
        // Floats: repr flips to '1e-05' form below 1e-4.
        assert_value_err(card_float_err(1e-05, 5), "1e-05");
        assert_value_err(card_float_err(1.5e-05, 6), "e");
        // The error escapes before the negword is applied, as in Python.
        assert_value_err(card_float_err(-1.5e-05, 6), "e");
        assert_value_err(card_float_err(9.999999999999999e-05, 20), "e");
        // Smallest subnormal: exponent grows past two digits, unpadded.
        assert_value_err(card_float_err(5e-324, 324), "5e-324");
        // Boundary: 1e-4 itself still reprs fixed and speaks normally.
        assert_eq!(card_float(0.0001, 4), "iqanda ichaphaza qanda qanda qanda nye");

        // Decimals: Python's str() flips to '1E-7' form once the adjusted
        // exponent is < -6; the shared str is written out instead (#211),
        // so these read digit by digit.
        assert_eq!(
            card_dec_p("1E-7", 7).unwrap(),
            "iqanda ichaphaza qanda qanda qanda qanda qanda qanda nye"
        );
        assert_eq!(card_dec_p("0.0000001", 7).unwrap(), card_dec_p("1E-7", 7).unwrap());
        // Boundaries that stay fixed: adjusted exponent exactly -6 — one
        // significant digit at the 10^-6 place, or a two-digit coefficient
        // holding a 10^-7-scale value up at -6.
        assert_eq!(
            card_dec_p("0.000001", 6).unwrap(),
            "iqanda ichaphaza qanda qanda qanda qanda qanda nye"
        );
        assert_eq!(
            card_dec_p("0.0000010", 7).unwrap(),
            "iqanda ichaphaza qanda qanda qanda qanda qanda nye qanda"
        );
    }

    /// Large values: Python's `_int_to_word` gave up at 10^9 (bug 1) and
    /// spoke digits before the pointword ('1234567890 ichaphaza ntlanu'). The
    /// port spells the ibhiliyoni band and raises OverflowError from 10^12
    /// (#147), e.g. for the 17-digit Decimal Python printed as
    /// '12345678901234567 ichaphaza sibhozo lithoba'.
    #[test]
    fn large_values_fixed_window() {
        assert_eq!(
            card_float(1234567890.5, 1),
            "ibhiliyoni na mbini ikhulu na amashumi amathathu ana ne isigidi na \
             ntlanu ikhulu na amashumi amathandathu ana sixhenxe iwaka na sibhozo \
             ikhulu na amashumi alithoba ichaphaza ntlanu"
        );
        assert!(matches!(
            card_dec_p("12345678901234567.89", 2),
            Err(N2WError::Overflow(_))
        ));
    }

    /// The whole-value routing: Python's `to_cardinal` keys on `str(number)`,
    /// so 5.0 keeps its ".0" tail and 1e16 dies in `int()`. The ordinal rides
    /// the same route behind the numeric `== 1` test, and `to_ordinal_num` is
    /// a pure string concat. All corpus-pinned.
    #[test]
    fn corpus_float_entry_routing() {
        let l = LangXh::new();
        let f = |v: f64, p: u32| FloatValue::Float { value: v, precision: p };
        let d = |s: &str, p: u32| FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: p,
        };

        // cardinal: whole floats/Decimals speak their visible decimals...
        assert_eq!(
            l.cardinal_float_entry(&f(5.0, 1), None).unwrap(),
            "ntlanu ichaphaza qanda"
        );
        assert_eq!(
            l.cardinal_float_entry(&f(-0.0, 1), None).unwrap(),
            "ngaphantsi kwe iqanda ichaphaza qanda"
        );
        assert_eq!(
            l.cardinal_float_entry(&d("5.00", 2), None).unwrap(),
            "ntlanu ichaphaza qanda qanda"
        );
        // ...a point-less Decimal lands on the integer branch...
        assert_eq!(l.cardinal_float_entry(&d("5", 0), None).unwrap(), "ntlanu");
        // ...and exponent-form strings raise Python's ValueError (bug 10).
        assert_value_err(
            l.cardinal_float_entry(&f(1e16, 16), None).unwrap_err(),
            "1e+16",
        );
        assert_value_err(
            l.cardinal_float_entry(&f(1e20, 20), None).unwrap_err(),
            "1e+20",
        );
        // #211: str(Decimal) is written out ("100"), so it reads.
        assert_eq!(
            l.cardinal_float_entry(&d("1E+2", 2), None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );

        // ordinal: 1.0 == 1 short-circuits; everything else gets "we".
        assert_eq!(l.ordinal_float_entry(&f(1.0, 1)).unwrap(), "okokuqala");
        assert_eq!(
            l.ordinal_float_entry(&f(5.0, 1)).unwrap(),
            "wentlanu ichaphaza qanda"
        );
        assert_eq!(
            l.ordinal_float_entry(&f(-0.0, 1)).unwrap(),
            "wengaphantsi kwe iqanda ichaphaza qanda"
        );
        assert_eq!(l.ordinal_float_entry(&d("5", 0)).unwrap(), "wentlanu");
        assert_eq!(l.ordinal_float_entry(&d("0", 0)).unwrap(), "weiqanda");
        // #211: str(Decimal) is written out, so 1E+9 reads as an integer;
        // 1E+20 is past the 10^12 ceiling (#147, Python echoed the digits).
        assert_eq!(l.ordinal_float_entry(&d("1E+9", 9)).unwrap(), "weibhiliyoni");
        assert!(matches!(
            l.ordinal_float_entry(&d("1E+20", 20)),
            Err(N2WError::Overflow(_))
        ));

        // ordinal_num: "we" + the binding's repr, verbatim.
        assert_eq!(
            l.ordinal_num_float_entry(&f(5.0, 1), "5.0").unwrap(),
            "we5.0"
        );
        assert_eq!(
            l.ordinal_num_float_entry(&f(1e16, 16), "1e+16").unwrap(),
            "we1e+16"
        );

        // year == cardinal, via the un-overridden trait default.
        assert_eq!(
            l.year_float_entry(&f(-3.0, 1)).unwrap(),
            "ngaphantsi kwe ntathu ichaphaza qanda"
        );
    }

    /// Inf/NaN string inputs are served natively per mode: `ValueError` on the
    /// cardinal/ordinal/year paths, a literal answer on `ordinal_num`.
    #[test]
    fn inf_nan_results_native() {
        let l = LangXh::new();
        for to in ["cardinal", "ordinal", "year"] {
            assert_value_err(l.inf_result(false, to).unwrap_err(), "Infinity");
            assert_value_err(l.inf_result(true, to).unwrap_err(), "Infinity");
            assert_value_err(l.nan_result(to).unwrap_err(), "NaN");
        }
        assert_eq!(l.inf_result(false, "ordinal_num").unwrap(), "weInfinity");
        assert_eq!(l.inf_result(true, "ordinal_num").unwrap(), "we-Infinity");
        assert_eq!(l.nan_result("ordinal_num").unwrap(), "weNaN");
    }

    /// `to_cheque` is `Num2Word_Base`'s, so unlike `to_currency` it raises for
    /// an unknown code (bug 5) and always uses the plural unit form.
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "IWAKA NA MBINI IKHULU NA AMASHUMI AMATHATHU ANA NE AND 56/100 IYURO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "IWAKA NA MBINI IKHULU NA AMASHUMI AMATHATHU ANA NE AND 56/100 IDOLA"
        );
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(msg)) => assert_eq!(
                    msg,
                    format!("Currency code \"{code}\" not implemented for \"Num2Word_XH\"")
                ),
                other => panic!("{code}: expected NotImplemented, got {other:?}"),
            }
        }
        // Checked against the live interpreter.
        assert_eq!(
            cheque("1234.56", "ZAR").unwrap(),
            "IWAKA NA MBINI IKHULU NA AMASHUMI AMATHATHU ANA NE AND 56/100 RANDI"
        );
        assert_eq!(cheque("-5.5", "USD").unwrap(), "MINUS NTLANU AND 50/100 IDOLA");
        assert_eq!(cheque("1.0", "USD").unwrap(), "NYE AND 00/100 IDOLA");
        assert_eq!(cheque("0.01", "USD").unwrap(), "IQANDA AND 01/100 IDOLA");
    }
}
