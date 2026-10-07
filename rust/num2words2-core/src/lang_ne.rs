//! Port of `lang_NE.py` (Nepali).
//!
//! Shape: **self-contained**. `Num2Word_NE` subclasses `Num2Word_Base` but its
//! `setup()` defines none of `high_numwords`/`mid_numwords`/`low_numwords`, so
//! the `hasattr` guard in `Num2Word_Base.__init__` never fires: Python builds
//! **no** `self.cards` and never sets `self.MAXVAL` (verified:
//! `hasattr(c, "cards") == False`, `hasattr(c, "MAXVAL") == False`).
//! `to_cardinal` is overridden outright and drives a private `_int_to_word`
//! recursion. Consequently `cards`/`maxval`/`merge` stay at their trait
//! defaults here, and there is **no overflow check at all** — arbitrarily large
//! input is accepted and silently degrades (see bug 1 below).
//!
//! Numbers use the South Asian scale: `हजार` (thousand, 10^3), `लाख` (lakh,
//! 10^5), `करोड` (crore, 10^7). So 10^6 is "दस लाख" (ten lakh), not "one
//! million".
//!
//! Inherited from `Num2Word_Base` — nothing relevant. NE overrides all four
//! in-scope methods (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`),
//! so no base method is reached on any in-scope path. In particular
//! `verify_ordinal` is never called (neither by NE nor by the `num2words()`
//! dispatcher), which is why negative ordinals return a string instead of
//! raising `TypeError` (see bug 3).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong-looking but
//! are exactly what Python emits, verified against the interpreter:
//!
//! 1. **The 10^9 cliff.** `_int_to_word`'s final `else` is
//!    `return str(number)  # Fallback for very large numbers` — so every value
//!    `>= 1_000_000_000` comes back as **bare ASCII digits**, not words:
//!    `to_cardinal(10**9) == "1000000000"` and
//!    `to_cardinal(10**21) == "1000000000000000000000"`. The sign is stripped
//!    by `to_cardinal` before `_int_to_word` runs, so a negative large value
//!    mixes scripts: `to_cardinal(-10**9) == "ऋण 1000000000"`. Modelled by the
//!    early `return number.to_string()` in [`LangNe::int_to_word`].
//!    Note the ceiling is 10^9 even though the largest scale word is `करोड`
//!    (10^7), so only 10^7..10^9 ever uses `करोड` — i.e. at most "99 करोड".
//! 2. **Ordinal suffix on digits.** `to_ordinal` appends "औं" to whatever
//!    `to_cardinal` returned, with no separator and no re-check, so bug 1
//!    leaks through: `to_ordinal(10**9) == "1000000000औं"` — a Devanagari
//!    suffix glued to an ASCII numeral.
//! 3. **Negative ordinals do not raise.** `to_ordinal(-1) == "ऋण एकऔं"`.
//!    Most languages route through `verify_ordinal` and raise `TypeError` for
//!    negatives; NE never calls it.
//! 4. **`to_year` is not year-formatted.** It is a plain cardinal with a hard
//!    coded ASCII `"BC "` prefix for negatives (`to_year(-500) == "BC पाँच
//!    सय"`) — no Nepali era word, and no pair-wise "nineteen eighty-four"
//!    style. The positive branch is literally `return "" + self.to_cardinal(val)`;
//!    the empty-string concat is a no-op, reproduced as a plain delegation.
//!    Its `longval=True` parameter is accepted and never read.
//! 5. **No teen/ten compounding.** `tens` are joined to `ones` with a plain
//!    space rather than the fused Nepali forms, so 21 is "बीस एक"
//!    (lit. "twenty one") rather than "एक्काइस", and 99 is "नब्बे नौ". This
//!    propagates everywhere, e.g. 123456789 == "बाह्र करोड तीस चार लाख पचास छ
//!    हजार सात सय असी नौ". Kept verbatim.
//!
//! # Float/Decimal routing (the `*_float_entry` hooks)
//!
//! `to_cardinal` is one method over `n = str(number).strip()`, whatever the
//! type, so there is **no whole-value shortcut**: `str(5.0)` is "5.0" and the
//! ".0" is read digit by digit — `num2words(5.0, lang="ne")` is "पाँच दशमलव
//! शून्य", never "पाँच". [`Lang::cardinal_float_entry`] bypasses the trait
//! default's `as_whole_int()` fast path accordingly. Reproducing `str(number)`
//! byte for byte is what the port pivots on: [`py_float_repr`] for floats
//! (CPython's `repr`, exponent notation included) and
//! [`crate::strnum::python_decimal_str`] for Decimals.
//!
//! Three string-basis consequences, all corpus-pinned:
//!
//! * **The sign is the string's leading "-"** — the sign *bit*, not `< 0` —
//!   so `-0.0` (and `Decimal("-0.0")`, which the binding hands over as the
//!   float -0.0) keeps the negword: "ऋण शून्य दशमलव शून्य".
//! * **Exponent-form repr raises `ValueError`.** `str(1e16)` is "1e+16",
//!   which has no "." — so the whole literal goes to `int()`:
//!   `ValueError: invalid literal for int() with base 10: '1e+16'`. The same
//!   for `Decimal("1E+2")` ("1E+2") and would-be "inf"/"nan" (the shim keeps
//!   non-finite floats on the Python side). A dotted e-form like
//!   `str(1.5e16)` == "1.5e+16" splits at the dot and dies on `int('e')`
//!   instead. See [`py_int`] and [`LangNe::cardinal_from_str`].
//! * **The 10^9 cliff (bug 1) leaks into the integer part**:
//!   `1000000000.0` == "1000000000 दशमलव शून्य" — ASCII digits before the
//!   point.
//!
//! The other modes follow `to_cardinal`'s lead:
//!
//! * `to_ordinal` — `if number == 1: return "पहिलो"` etc. is a *numeric*
//!   equality, so `1.0` and `Decimal("1.00")` take the special forms with no
//!   "दशमलव" tail; everything else is the cardinal with "औं" glued on
//!   ("पाँच दशमलव शून्यऔं"), ValueErrors propagating (1e16 raises before the
//!   suffix is reached).
//! * `to_ordinal_num` — `str(number) + "."` unconditionally: "5.0.",
//!   "1e+16." (no error!), "-0.0.".
//! * `to_year` — `if val < 0` is numeric (`-0.0` is *not* < 0, so it skips
//!   "BC" and renders through the string path, negword and all:
//!   "ऋण शून्य दशमलव शून्य"), otherwise `"BC " + to_cardinal(abs(val))`.
//!
//! `num2words("Infinity", lang="ne")` is a `ValueError`, not base's
//! OverflowError: `Decimal("Infinity")` parses, `str()` is "Infinity", no "."
//! — `int("Infinity")` raises. The shared Inf sentinel maps to OverflowError
//! in the binding, so [`Lang::str_to_number`] raises the ValueError itself
//! (same route as the RU port; see the override for the mode caveat).
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_NE` holds no flag that one method sets and another consumes
//! (no `_pending_ordinal`-style handshake as in `lang_ES`), and no NE method
//! ever touches `self.precision` (only the float branch of `Num2Word_Base`
//! does, which NE never reaches — its float handling is inline in
//! `to_cardinal`). This port is safely stateless.
//!
//! # Error variants
//!
//! **Integer modes: none reachable.** Every in-scope integer input returns
//! `Ok`; there is no overflow check, no table lookup that can miss, and no
//! `int()` of a bad token. All 305 in-scope corpus rows are `ok: true`.
//!
//! **Float/Decimal/string modes**: `ValueError` when `str(number)` is not
//! plain digits — exponent-form repr ("1e+16", "1E+2") and "Infinity" (see
//! the routing section above). `to_ordinal_num` never raises.
//!
//! **Currency**: `to_currency` cannot raise for an unknown code (bug 6 below)
//! but *can* raise `ValueError` on an exponent-notation float (bug 8);
//! `to_cheque` raises `NotImplementedError` for any code outside the table.
//!
//! # Currency surface (phase 2)
//!
//! `Num2Word_NE` declares its **own** `CURRENCY_FORMS` class attribute
//! (verified: `"CURRENCY_FORMS" in Num2Word_NE.__dict__ == True`), so the
//! `Num2Word_EN.__init__` mutation that rewrites `Num2Word_EUR`'s shared dict
//! never reaches it — NE sees exactly the three literals in `lang_NE.py`, and
//! none of EN's ~24 extra codes. It defines neither `CURRENCY_ADJECTIVES` nor
//! `CURRENCY_PRECISION`, inheriting `{}` for both from `Num2Word_Base`
//! (verified), so `currency_precision` stays at the trait default of 100 for
//! *every* code — including KWD/BHD (which are not in the table anyway) — and
//! `currency_adjective` stays `None`.
//!
//! `to_currency` is overridden outright and calls **none** of the base
//! currency machinery: no `parse_currency_parts`, no `pluralize`, no
//! `_money_verbose`/`_cents_verbose`/`_cents_terse`, no `currency_precision`.
//! Those hooks are therefore left at their trait defaults — `pluralize`'s
//! default (raise) is the faithful mirror of Python's abstract `pluralize`,
//! which NE never implements and never reaches.
//!
//! `to_cheque` is **not** overridden, so `Num2Word_Base.to_cheque` runs
//! verbatim: strict `CURRENCY_FORMS[currency]` lookup, divisor 100,
//! `_money_verbose` → `self.to_cardinal`, `cr1[-1]` for the unit, `.upper()`.
//! That is exactly `currency::default_to_cheque`, so this port only supplies
//! `lang_name` + `currency_forms` and lets the default drive.
//!
//! # Further faithfully reproduced Python bugs (currency)
//!
//! 6. **An unknown currency code silently becomes NPR.**
//!    `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["NPR"])` — no
//!    `NotImplementedError`, unlike every other language. So
//!    `to_currency(12.34, currency="JPY")` returns "बाह्र रुपैयाँ तीस चार पैसा":
//!    Japanese yen rendered as Nepali rupees. The corpus pins this — its
//!    `currency:GBP` / `currency:JPY` / `currency:KWD` / `currency:BHD` /
//!    `currency:INR` / `currency:CNY` / `currency:CHF` rows are all `ok: true`
//!    with रुपैयाँ, while the matching `cheque:*` rows raise
//!    `NotImplementedError` — the two lookups disagree by design.
//! 7. **`CURRENCY_PRECISION` is never consulted.** JPY (a 0-decimal currency)
//!    gets a subunit it does not have, and KWD/BHD get two digits of cents
//!    instead of three mils. Base's zero-decimal rounding never runs.
//!    (Moot in practice, since bug 6 turns all three into NPR first.)
//! 8. **The split is a string operation, so exponent-notation floats crash.**
//!    `str(val).split(".")` then `int()` — see [`split_currency`].
//! 9. **`cents=False` omits the cents rather than making them terse.** The
//!    base class would switch to `_cents_terse` (digits); NE's `cents and
//!    right` guard suppresses the whole segment, so `_cents_terse` is
//!    unreachable and the trait default is left alone.
//!    (`to_currency(12.34, cents=False) == "बाह्र euros"`, verified.)
//! 10. **`adjective` is accepted and never read.** `CURRENCY_ADJECTIVES` is
//!    empty for NE anyway, so base would have ignored it too.
//! 11. **`negword` is used raw, not stripped.** `self.negword + result` where
//!    base does `"%s " % self.negword.strip()`. Both give "ऋण " because the
//!    literal already ends in exactly one space.
//! 12. **The 10^9 cliff (bug 1) leaks into money.** `to_currency` renders both
//!    halves through `_int_to_word`, not `to_cardinal`, so
//!    `to_currency(10**9, currency="EUR") == "1000000000 euros"` — ASCII
//!    digits with an English unit (verified).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use डलर / युरो with सेन्ट. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `setup()`: `self.negword`. Note the **trailing space** — it is load-bearing,
/// since `to_cardinal` concatenates it directly onto the word with no
/// separator of its own.
const NEGWORD: &str = "ऋण ";

/// `setup()`: `self.pointword`. Consumed by [`LangNe::to_cardinal_float`] — the
/// separator word between the integer part and the spelled-out fraction digits.
const POINTWORD: &str = "दशमलव";

/// `setup()`: `self.zero`.
const ZERO: &str = "शून्य";

/// `setup()`: `self.ones`. Index 0 is `""` in Python and is never reachable —
/// `_int_to_word` returns `self.zero` for 0 before any `ones` lookup, and the
/// hundreds branch indexes `number // 100`, which is >= 1 there. Kept verbatim.
const ONES: [&str; 10] = [
    "", "एक", "दुई", "तीन", "चार", "पाँच", "छ", "सात", "आठ", "नौ",
];

/// `setup()`: `self.tens`. Indices 0 and 1 are unreachable — the branch that
/// reads this table is guarded by `20 <= number < 100`, so `number // 10` is
/// always 2..=9. (`tens[1]` == "दस" duplicates `teens[0]`; 10 is served by the
/// teens branch.) Kept verbatim.
const TENS: [&str; 10] = [
    "", "दस", "बीस", "तीस", "चालीस", "पचास", "साठी", "सत्तरी", "असी", "नब्बे",
];

/// `setup()`: `self.teens`, indexed by `number - 10` for `10 <= number < 20`.
const TEENS: [&str; 10] = [
    "दस", "एघार", "बाह्र", "तेह्र", "चौध", "पन्ध्र", "सोह्र", "सत्र", "अठार", "उन्नाइस",
];

/// `setup()`: `self.hundred` (10^2).
const HUNDRED: &str = "सय";
/// `setup()`: `self.thousand` (10^3).
const THOUSAND: &str = "हजार";
/// `setup()`: `self.lakh` (10^5).
const LAKH: &str = "लाख";
/// `setup()`: `self.crore` (10^7).
const CRORE: &str = "करोड";

/// The exclusive ceiling of `_int_to_word`'s word-producing range. At or above
/// this, Python falls through to `str(number)` — see bug 1 in the module docs.
const FALLBACK_LIMIT: u64 = 1_000_000_000;

/// `self.__class__.__name__`, quoted in `to_cheque`'s NotImplementedError.
const LANG_NAME: &str = "Num2Word_NE";

/// `Num2Word_NE.to_currency`'s own default `separator=" "` — note it is a bare
/// space, not base's `","`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot simply be a parameter default.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `num2words.__init__`'s Rust fast path calls
/// `_RUST.to_currency(..., kwargs.get("separator", ","), ...)` — it fills in
/// **base's** default, not the language's, before the value ever reaches Rust.
/// By then "caller omitted separator" and "caller explicitly passed a comma"
/// are the same string, and the information needed to tell them apart no
/// longer exists on this side of the boundary.
///
/// So `,` is read back as the unset sentinel and NE's own default restored.
/// This is the only reading that matches the oracle: every float row of the
/// `ne` currency corpus was generated by `num2words(v, lang="ne",
/// to="currency", currency=c)` with no `separator=`, and every one expects a
/// space (`"बाह्र euros तीस चार cents"`).
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets `" "` here where Python gives `","`. Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report. This
/// mirrors the convention `lang_ca.rs` / `lang_as.rs` / `lang_bo.rs`
/// established for the same problem.
const SEPARATOR_UNSET: &str = ",";

/// `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["NPR"])` — the code every
/// unknown currency silently degrades to. See bug 6 in the module docs.
const FALLBACK_CURRENCY: &str = "NPR";

/// Python's `str(float)` — which for a float is `repr(float)`.
///
/// This string *is* NE's input: `to_cardinal` starts with `n = str(number)`
/// and only ever looks at the text again, so `repr` must be reproduced rather
/// than the raw f64's binary arithmetic (same situation as `lang_az.rs`,
/// whose verified implementation this duplicates — each module carries its
/// own copy by convention).
///
/// Reproduces CPython's `format_float_short(d, 'r', ..., ADD_DOT_0)`:
///
///   * Digit *generation* is delegated to Rust's own shortest-round-trip
///     formatter, the same contract as Python's `repr`.
///   * The notation switch is CPython's `if (decpt <= -4 || decpt > 16)
///     use_exp = 1;` — the "1e+16" cliff NE's `int()` then falls off.
///   * The exponent is `sprintf("%+.02d")`: always signed, zero-padded to two
///     digits, hence "1e+16" and "1e-05".
///   * `Py_DTSF_ADD_DOT_0` is why `repr(1.0)` is "1.0" and not "1".
fn py_float_repr(value: f64) -> String {
    // The shim keeps non-finite floats on the Python side, but the spellings
    // are kept for completeness (str(float('inf')) == "inf").
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    // `{:e}` is Rust's shortest round-trip in scientific form, yielding the
    // `digits` / `decpt` pair `_Py_dg_dtoa` hands `format_float_short`.
    let sci = format!("{:e}", value);
    let (mantissa, exponent) = match sci.split_once('e') {
        Some(pair) => pair,
        None => return sci, // unreachable: LowerExp always emits an 'e'.
    };
    let decpt = exponent.parse::<i32>().unwrap_or(0) + 1;
    let ndigits = mantissa.chars().filter(char::is_ascii_digit).count() as i32;

    if decpt <= -4 || decpt > 16 {
        // Exponent form — the string NE's `int()` chokes on, built out in
        // full because the ValueError quotes it verbatim.
        let (sign, mantissa) = match mantissa.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", mantissa),
        };
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let mut out = String::from(sign);
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            // No ADD_DOT_0 here: repr(1e21) is "1e+21", not "1.0e+21".
            out.push('.');
            out.push_str(&digits[1..]);
        }
        let exp = decpt - 1;
        out.push_str(&format!(
            "e{}{:02}",
            if exp < 0 { '-' } else { '+' },
            (exp as i64).abs()
        ));
        out
    } else {
        // Fixed form. The fraction is `ndigits - decpt` wide, floored at 1 by
        // ADD_DOT_0 (repr(1e15) is "1000000000000000.0"). Re-rendering with
        // `{:.n$}` instead of splicing the `{:e}` digits back together keeps
        // exact ties on CPython's round-to-even side (see lang_az.rs).
        let frac = std::cmp::max(ndigits - decpt, 1) as usize;
        format!("{:.*}", frac, value)
    }
}

/// Python's `int(str)` as `to_cardinal` uses it — on a slice of
/// `str(number)`, so the accepted alphabet is plain ASCII digits (any sign
/// was already stripped by the caller); anything else raises ValueError
/// naming the *whole* literal: '1e+16', '1E+2', 'inf', 'Infinity'.
fn py_int(s: &str) -> Result<BigInt> {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
        return Ok(s.parse().expect("all-ASCII-digit string parses"));
    }
    Err(N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        s
    )))
}

/// The special forms opening Python's `to_ordinal`. The comparisons are
/// `number == 1` etc — *numeric* equality, so 1.0 and Decimal("1.00") hit
/// "पहिलो" too; the float entry funnels through `as_whole_int` first.
fn special_ordinal(value: &BigInt) -> Option<&'static str> {
    match value.to_i64() {
        Some(1) => Some("पहिलो"),  // first
        Some(2) => Some("दोस्रो"), // second
        Some(3) => Some("तेस्रो"), // third
        Some(4) => Some("चौथो"),  // fourth
        _ => None,
    }
}

/// The `str(val).split(".")` half of `to_currency`, which is a **string**
/// operation, not an arithmetic one:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `val` is already non-negative here (`to_currency` took `abs` first).
///
/// Three consequences, all corpus-pinned:
///
/// * `right` left-justifies the *first two fraction digits*, so `0.5` → `"5"`
///   → `"50"` → 50 paisa, while `0.05` → `"05"` → 5. It is a digit slice, not
///   a multiplication, and it silently truncates a third decimal (`12.345` →
///   34 cents, no rounding — verified).
/// * a fraction of `"0"` (i.e. the float `1.0`) yields `right == 0`, which
///   `to_currency` treats as falsy and drops the cents clause entirely. This
///   is where NE **departs from base**: base renders "one euro, zero cents"
///   for `1.0` because a float always shows cents, while NE prints just
///   "एक euro". The corpus pins it (`{"arg": "1.0", "out": "एक euro"}`), so the
///   `Int`/`Decimal` split still matters for a different reason — see
///   `to_currency`.
/// * no `.` in `str(val)` (a true `int`) leaves `right` at 0 for the same
///   reason, which is how NE arrives at base's `isinstance(val, int)`
///   behaviour the long way round.
///
/// # The exponent-notation hole
///
/// `str(float)` switches to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal. Verified against
/// the interpreter:
///
/// ```text
/// to_currency(1e16)    -> ValueError: invalid literal for int() with base 10: '1e+16'
/// to_currency(1.5e16)  -> ValueError: invalid literal for int() with base 10: '5e'
/// to_currency(1e-05)   -> ValueError: invalid literal for int() with base 10: '1e-05'
/// ```
///
/// A negative `BigDecimal` scale is exactly the "the source string used `e+`
/// notation *and* the exponent outran the digits" signal (a plain digit string
/// parses to scale 0, never below), so that arm is reproduced.
///
/// Two residual holes are **not** reproducible here and are flagged in the
/// port report, both because the discriminator is the original string, which
/// the `CurrencyValue` boundary does not carry:
///
/// * the `e-` side: `1e-05` and `Decimal("0.00001")` parse to the *same*
///   `BigDecimal` (digits 1, scale 5), yet Python raises `ValueError` for the
///   first and returns "शून्य euros" for the second.
/// * a 17-significant-digit `e+` float lands back on scale 0:
///   `str(1.2345678901234568e+16)` splits to `["1", "2345678901234568e+16"]`,
///   so Python takes `left = 1`, `right = 23` and returns
///   "एक euro बीस तीन cents" (verified) where this port reads the value as the
///   integer 12345678901234568.
///
/// No corpus row reaches any of the three (`ne`'s largest float is 1234.56).
fn split_currency(val: &BigDecimal) -> Result<(BigInt, BigInt)> {
    // value == digits * 10^-scale
    let (digits, scale) = val.as_bigint_and_exponent();

    if scale < 0 {
        // str(val) would be "1e+16"-shaped; int() on it raises. Python's
        // message quotes the exact offending literal, which is unrecoverable
        // from the parsed value — the exception *type* is what callers observe.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            val
        )));
    }

    // No "." in str(val): parts == [str(val)], so `right` stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    // `val` is non-negative, so this is a bare ASCII digit string.
    let s = digits.abs().to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    // `int(parts[0]) if parts[0] else 0` — the guard is unreachable (str() of
    // a non-negative number never yields an empty integer part), but the
    // fallback is kept rather than panicking.
    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

pub struct LangNe {
    /// `Num2Word_NE.CURRENCY_FORMS`, built once in [`LangNe::new`].
    ///
    /// The binding holds each `LangNe` in a `OnceLock`, so this table is
    /// constructed exactly once per process rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangNe {
    pub fn new() -> Self {
        // Num2Word_NE.CURRENCY_FORMS verbatim — three codes, nothing inherited.
        // NPR carries two identical forms because Nepali does not inflect these
        // nouns for number; the arity stays at Python's 2 regardless, since
        // `to_currency` indexes `cr1[1]` and base's `to_cheque` reads `cr1[-1]`.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "NPR",
            CurrencyForms::new(&["रुपैयाँ", "रुपैयाँ"], &["पैसा", "पैसा"]),
        );
        currency_forms.insert("USD", CurrencyForms::new(&["डलर", "डलर"], &["सेन्ट", "सेन्ट"]));
        currency_forms.insert("EUR", CurrencyForms::new(&["युरो", "युरो"], &["सेन्ट", "सेन्ट"]));
        LangNe { currency_forms }
    }

    /// Python's `_int_to_word`.
    ///
    /// Kept on `BigInt` because input is unbounded. The `number == 0` and
    /// `number < 0` guards come first, exactly as in Python, so by the time we
    /// reach the numeric ladder the value is >= 1 and we can hand off to
    /// [`LangNe::small_to_word`] on a plain `u64`.
    ///
    /// The `to_u64()` miss and the `>= FALLBACK_LIMIT` hit collapse into the
    /// same arm: both are Python's `else: return str(number)`. A value too big
    /// for `u64` is necessarily >= 10^9, so this is exact, not an approximation.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO.to_string();
        }
        if number.is_negative() {
            // Unreachable from any in-scope entry point: `to_cardinal` strips
            // the sign before calling in, `to_year` passes `abs(val)`, and
            // every recursive call passes a non-zero positive remainder. Python
            // only reaches it via `to_currency` (out of scope). Ported anyway
            // for structural fidelity — note it would emit a *doubled* negword
            // if it ever were reached through `to_cardinal`.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // Python: `else: return str(number)` — the "very large numbers" fallback.
        match number.to_u64() {
            Some(n) if n < FALLBACK_LIMIT => self.small_to_word(n),
            _ => number.to_string(),
        }
    }

    /// The word-producing ladder of `_int_to_word`, on the proven domain
    /// `1 <= n < 1_000_000_000`.
    ///
    /// Every recursive call passes a strictly smaller, non-zero, positive value
    /// (each is guarded by `if remainder:` in Python / `!= 0` here), so the
    /// invariant holds throughout and no index can go out of range.
    fn small_to_word(&self, n: u64) -> String {
        if n < 10 {
            ONES[n as usize].to_string()
        } else if n < 20 {
            TEENS[(n - 10) as usize].to_string()
        } else if n < 100 {
            let mut result = TENS[(n / 10) as usize].to_string();
            if n % 10 != 0 {
                result.push(' ');
                result.push_str(ONES[(n % 10) as usize]);
            }
            result
        } else if n < 1_000 {
            // Note: the hundreds digit reads `ones` directly rather than
            // recursing, so 100 == "एक सय".
            let mut result = format!("{} {}", ONES[(n / 100) as usize], HUNDRED);
            let remainder = n % 100;
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        } else if n < 100_000 {
            // Thousands run up to the lakh, so `n / 1000` is 1..=99.
            let mut result = format!("{} {}", self.small_to_word(n / 1_000), THOUSAND);
            let remainder = n % 1_000;
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        } else if n < 10_000_000 {
            // Lakh: `n / 100_000` is 1..=99, so 10^6 == "दस लाख".
            let mut result = format!("{} {}", self.small_to_word(n / 100_000), LAKH);
            let remainder = n % 100_000;
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        } else {
            // Crore: `n < 1_000_000_000` by invariant, so `n / 10_000_000` is
            // 1..=99 — "99 करोड" is the largest expressible magnitude.
            let mut result = format!("{} {}", self.small_to_word(n / 10_000_000), CRORE);
            let remainder = n % 10_000_000;
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        }
    }

    /// The body of Python's `to_cardinal`, given `n = str(number).strip()`:
    ///
    /// ```text
    /// if n.startswith("-"): n = n[1:]; ret = self.negword
    /// else:                 ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     ret += " ".join(self._int_to_word(int(d)) for d in right)
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// Shared by the float/Decimal entries (the integer entry keeps its own
    /// sign-and-abs shortcut, which is the identity for `str(int)`). The sign
    /// is the string's leading "-" — so -0.0 *does* keep the negword — and
    /// the fraction is read digit by digit, trailing zeros and all. `int()`
    /// of a non-digit slice — exponent-form repr, "inf", "Infinity" — raises
    /// ValueError quoting the exact literal ([`py_int`]); a *dotted* e-form
    /// like "1.5e+16" instead dies per-character on `int('e')` after the "5"
    /// was already accepted (the half-built join is discarded on the raise,
    /// so the order is unobservable).
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        let (ret, n) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", n),
        };
        if let Some((left, right)) = n.split_once('.') {
            // `n.split(".", 1)` — str() output has at most one dot, so
            // split_once is exact.
            let left_word = self.int_to_word(&py_int(left)?);
            let mut digit_words: Vec<String> = Vec::with_capacity(right.len());
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                digit_words.push(self.int_to_word(&BigInt::from(d)));
            }
            // ret + left + " " + pointword + " " + " ".join(digits), stripped.
            Ok(
                format!("{}{} {} {}", ret, left_word, POINTWORD, digit_words.join(" "))
                    .trim()
                    .to_string(),
            )
        } else {
            // `(ret + self._int_to_word(int(n))).strip()`.
            Ok(format!("{}{}", ret, self.int_to_word(&py_int(n)?))
                .trim()
                .to_string())
        }
    }
}

impl Default for LangNe {
    fn default() -> Self {
        LangNe::new()
    }
}

impl Lang for LangNe {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NPR"
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
        "दशमलव"
    }

    /// Python's `to_cardinal`.
    ///
    /// Python works on `n = str(number).strip()` and tests `n.startswith("-")`,
    /// then re-parses with `int(n[1:])`. For integer input that round-trip is
    /// the identity, so testing the sign of the `BigInt` and taking `abs()` is
    /// equivalent. The `"." in n` branch is the float/Decimal path, ported in
    /// [`LangNe::to_cardinal_float`] (the Rust dispatcher routes non-integer
    /// input there rather than through this integer entry point).
    ///
    /// The trailing `.strip()` is reproduced with `trim()`. The two differ in
    /// general (Rust trims Unicode whitespace, Python `str.strip()` trims
    /// `str.isspace()` chars), but the result here always begins and ends with
    /// a Devanagari word or an ASCII digit and can never be empty, so they
    /// agree. Its only real effect is on the positive branch, where it is a
    /// no-op; on the negative branch `negword`'s trailing space becomes the
    /// separator and survives.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&n)).trim().to_string())
    }

    /// Python's `to_cardinal`, **float/Decimal arm**.
    ///
    /// `Num2Word_NE` does **not** override `to_cardinal_float`; it overrides
    /// `to_cardinal` and handles non-integers inline, on the *string* form of
    /// the number — see [`LangNe::cardinal_from_str`] for the ported body and
    /// the module docs' routing section for the consequences (sign bit via
    /// the leading "-", per-digit fraction, exponent-form ValueError, the
    /// 10^9 cliff leaking ASCII digits into the integer part).
    ///
    /// `str(number)` is rebuilt exactly: [`py_float_repr`] for floats (so
    /// `2.675` spells the repr digits `छ सात पाँच`, and `1e16` becomes
    /// "1e+16" — the string `int()` then rejects) and
    /// [`crate::strnum::python_decimal_str`] for Decimals (so
    /// `Decimal("1.10")` keeps its trailing zero and `Decimal("1E+2")` stays
    /// in exponent form and raises). `Decimal("-0.0")` never reaches the
    /// Decimal arm — the binding hands it over as the float -0.0, whose repr
    /// "-0.0" carries the sign Python sees.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is
    /// **ignored**: NE never reads `self.precision`, so `converter.precision
    /// = …` in the dispatcher is a no-op here (verified against the
    /// interpreter). The fractional digit count comes solely from
    /// `str(number)`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>, // ignored: NE never reads self.precision.
    ) -> Result<String> {
        let s = match value {
            FloatValue::Float { value: f, .. } => py_float_repr(*f),
            FloatValue::Decimal { value: d, .. } => python_decimal_str(d),
        };
        self.cardinal_from_str(&s)
    }

    /// `to_cardinal(float/Decimal)` — the full entry, whole values included.
    ///
    /// NE's `to_cardinal` is one method over `str(number)`: a whole float
    /// keeps its ".0" (`num2words(5.0, lang="ne")` == "पाँच दशमलव शून्य"), so
    /// the trait default's `as_whole_int()` shortcut is wrong here and
    /// bypassed.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `if number == 1: ...` — *numeric*
    /// equality, so 1.0 and Decimal("1.00") take the special forms with no
    /// "दशमलव" tail; everything else (whole or not) is the string-driven
    /// cardinal with "औं" glued on ("पाँच दशमलव शून्यऔं"), ValueErrors
    /// propagating — 1e16 raises inside the cardinal before the suffix is
    /// reached. `as_whole_int` is `None` for a fractional value, which can
    /// equal none of 1..=4, so skipping the specials is exact.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if let Some(s) = special_ordinal(&i) {
                return Ok(s.to_string());
            }
        }
        Ok(format!("{}औं", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — no whole-value
    /// check, no words, no error path: "5.0.", "1e+16.", "-0.0.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `if val < 0` — a numeric `<`, not the sign
    /// bit, so -0.0 skips the "BC" arm and renders through `to_cardinal`,
    /// whose *string* test does see the "-": `to_year(-0.0)` == "ऋण शून्य
    /// दशमलव शून्य". Negatives render `"BC " + to_cardinal(abs(val))` — abs
    /// strips the sign before str(), so no negword survives under the "BC".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let lt_zero = match value {
            FloatValue::Float { value: f, .. } => *f < 0.0,
            FloatValue::Decimal { value: d, .. } => d.is_negative(),
        };
        if !lt_zero {
            return self.to_cardinal_float(value, None);
        }
        let abs = match value {
            FloatValue::Float { value: f, precision } => FloatValue::Float {
                value: f.abs(),
                precision: *precision,
            },
            FloatValue::Decimal { value: d, precision } => FloatValue::Decimal {
                value: d.abs(),
                precision: *precision,
            },
        };
        Ok(format!("BC {}", self.to_cardinal_float(&abs, None)?))
    }

    /// Python's `to_ordinal`. Special forms for 1..=4, otherwise the cardinal
    /// with "औं" glued on — including onto ASCII digits above the 10^9 cliff
    /// and onto negatives (bugs 2 and 3).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(s) = special_ordinal(value) {
            return Ok(s.to_string());
        }
        Ok(format!("{}औं", self.to_cardinal(value)?))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."` — digits, not words, and
    /// the minus sign is kept (`to_ordinal_num(-1) == "-1."`).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year`. See bug 4: an ASCII "BC " prefix over `abs(val)`,
    /// otherwise a plain cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("BC {}", self.to_cardinal(&value.abs())?))
        } else {
            self.to_cardinal(value)
        }
    }

    /// Base `str_to_number` (`Decimal(value)`), with one NE-visible fixup:
    /// `Decimal("Infinity")` parses fine in Python, and NE's `to_cardinal`
    /// then finds no "." in `str()` == "Infinity", so `int("Infinity")`
    /// raises `ValueError: invalid literal for int() with base 10:
    /// 'Infinity'` — not the OverflowError of base's
    /// `int(Decimal('Infinity'))`. "-Infinity" strips its leading sign first
    /// and names the same 'Infinity'. The shared Inf sentinel maps to
    /// OverflowError in the binding, so the ValueError is raised here
    /// instead.
    ///
    /// (For to="ordinal"/"year" Python reaches the same ValueError through
    /// the cardinal; to="ordinal_num" would return "Infinity." — a success —
    /// but only cardinal rows exist in the corpus and the mode is not
    /// visible from this hook. Same trade the RU port makes.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// The **strict** `CURRENCY_FORMS[currency]` lookup, as base's `to_cheque`
    /// performs it — a miss becomes `NotImplementedError`. `to_currency` does
    /// *not* come through here: it reads `self.currency_forms` directly so it
    /// can fall back to NPR (bug 6). The two lookups genuinely disagree in
    /// Python and the corpus pins both halves.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_NE.to_currency`.
    ///
    /// ```text
    /// def to_currency(self, val, currency="NPR", cents=True,
    ///                 separator=" ", adjective=False):
    /// ```
    ///
    /// Note it shares nothing with `Num2Word_Base.to_currency` beyond the
    /// signature: it re-derives left/right by string-splitting `str(val)`
    /// ([`split_currency`]), picks the plural form with a bare `!= 1` test
    /// instead of `pluralize`, and renders through `_int_to_word` rather than
    /// `to_cardinal` — so the sign is applied exactly once, at the very end,
    /// via the raw `negword` (which already carries its trailing space).
    ///
    /// `currency`'s own default is `"NPR"`, but the binding always passes a
    /// code explicitly (`kwargs.get("currency", "EUR")` fills in base's
    /// default when the caller omits one) — another place the shim substitutes
    /// base's default for the language's. Unobservable in the corpus, which
    /// always names a code; flagged in the port report alongside `separator`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // bug 10: accepted, never read.
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore NE's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`.
        //
        // For `-0.0` Python's guard is False (so `val` keeps its sign) and
        // `str(-0.0).split(".")` gives `["-0", "0"]` → `int("-0") == 0` →
        // "शून्य euros". `BigDecimal` has no negative zero, so `is_negative()`
        // is False and the split yields (0, 0) — the same answer by a
        // different route.
        let is_negative = val.is_negative();
        let (left, right) = match val {
            // str(int) has no ".", so parts has length 1 and `right` stays 0 —
            // the cents clause is dropped. This is the `isinstance(val, int)`
            // split base.py makes explicitly, arrived at the long way round.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => split_currency(&d.abs())?,
        };

        // Bug 6: `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["NPR"])` —
        // unknown code degrades to NPR with no error. "NPR" is inserted by
        // `new`, so the fallback cannot miss (Python would KeyError if it
        // could, since it subscripts rather than `.get`s the default).
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CURRENCY))
            .expect("CURRENCY_FORMS always carries the NPR fallback entry");
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::one();
        // `self._int_to_word(left)`, not `to_cardinal` — so bug 1's 10^9 cliff
        // surfaces here as ASCII digits (bug 12).
        let left_str = self.int_to_word(&left);
        let mut result = format!(
            "{} {}",
            left_str,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — a zero `right` is falsy, so `1.0` shows no
        // cents (bug 9 covers the `cents=False` half of this guard).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // Bug 11: raw `negword`, whose trailing space is the separator.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()` — a no-op in practice: `_int_to_word` never returns
        // "" for a non-negative input (0 is `self.zero`), and every currency
        // form in the table is non-empty, so the result never begins or ends
        // with whitespace. Kept for fidelity.
        Ok(result.trim().to_string())
    }
}
