//! Port of `lang_YO.py` (Yoruba).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "biliọnu" (10^9; Wiktionary "biliọnu", spelled like miliọnu),
//! composed like the million arm, and raises `OverflowError` from 10^12, which
//! `maxval()` reports. The 10^12 word has a single source and is not used.
//! Where the notes below describe the digit fallback ("1000000000", "no
//! overflow check"), they describe Python; that arm is now unreachable.
//!
//! Shape: **self-contained**. `Num2Word_YO` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never enters the `if any(hasattr(...))` branch:
//! `self.cards` is never built and `self.MAXVAL` is never set. `to_cardinal`
//! is overridden outright and drives a recursive `_int_to_word`. Accordingly
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check** — no input can raise `OverflowError`, because
//! `_int_to_word` has a `str(number)` fallback instead of a ceiling (bug 1).
//!
//! `setup()` supplies `negword = "minus "` (note the trailing space) and
//! `pointword = "point"` — still English: the only decimal word found
//! ("ẹsẹ") comes from a single proposal dictionary (gladiaio/num2words2#154). Every in-scope entry point is overridden by
//! `Num2Word_YO`, so nothing is inherited from `Num2Word_Base` except the
//! attributes above:
//!   * `to_cardinal`    — overridden (see below)
//!   * `to_ordinal`     — overridden: `to_cardinal(n) + "-kẹta"`
//!   * `to_ordinal_num` — overridden: `str(n) + "."`
//!   * `to_year`        — overridden: `to_cardinal(val)`, ignoring `longval`
//!
//! No cross-call mutable state: `Num2Word_YO` defines no `str_to_number` and
//! stashes no flags between methods, so the stateless Rust path is faithful.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the bare digits.** The
//!    final `else` is `return str(number)`, so `to_cardinal(10**9)` ==
//!    `"1000000000"` — digits, not words — and `to_ordinal(10**9)` ==
//!    `"1000000000-kẹta"`. There is no `million`-and-up table (`setup` stops
//!    at `self.million`), and no exception is ever raised. The fallback keeps
//!    full precision for arbitrarily large input, hence the `BigInt`
//!    `to_string()` in [`LangYo::int_to_word`] — the corpus exercises 10^21.
//!    Because `to_cardinal` strips the sign *before* recursing, a large
//!    negative renders as `"minus 1000000000"`.
//!
//! 2. **Hundreds are formed as `ones[h] + " " + hundred`,** so 100 is
//!    `"ọkan ọgọrun"` ("one hundred") rather than a bare `"ọgọrun"`. Kept.
//!
//! 3. **`to_ordinal` appends a fixed `"-kẹta"` to the cardinal** — `kẹta` is
//!    the Yoruba ordinal for *third*, so every ordinal literally reads
//!    "<n>-third": `to_ordinal(1)` == `"ọkan-kẹta"`, `to_ordinal(0)` ==
//!    `"òdo-kẹta"`. It is a fixed suffix, not agreement, and unlike most
//!    modules it neither rejects 0 nor rejects negatives:
//!    `to_ordinal(-1)` == `"minus ọkan-kẹta"`. No exception path exists.
//!
//! 4. **`_int_to_word(0)` is written `self.ones[0] if self.ones[0] else "zero"`.**
//!    `ones[0]` is `""` — falsy — so Python always returned the English
//!    "zero". Fixed (gladiaio/num2words2#154): the constant [`ZERO`] is the
//!    Yoruba "òdo".
//!
//! 5. **The `number < 0` arm of `_int_to_word` is unreachable** from every
//!    in-scope entry point: `to_cardinal` strips the leading `"-"` from the
//!    *string* and prepends `negword` itself, so `_int_to_word` only ever
//!    sees a non-negative value. It is reproduced anyway (see
//!    [`LangYo::int_to_word`]) because it is part of the function's
//!    semantics, and it would double the negword if it ever were reached.
//!
//! # Currency
//!
//! `Num2Word_YO` overrides `to_currency` **wholesale** — it never reaches
//! `Num2Word_Base.to_currency`, so none of `parse_currency_parts`,
//! `pluralize`, `_cents_verbose`, `_cents_terse` or `CURRENCY_PRECISION`
//! participate. It re-derives the split itself from `str(val)`. See
//! [`LangYo::to_currency`].
//!
//! `to_cheque` is *not* overridden, so it comes from `Num2Word_Base` via the
//! trait default (`currency::default_to_cheque`), which needs only
//! [`Lang::currency_forms`] + [`Lang::lang_name`] + the default
//! `money_verbose` (= `to_cardinal`) from this file.
//!
//! `CURRENCY_FORMS` is defined directly on `Num2Word_YO` (NGN, USD, EUR) and
//! is **not** the shared `Num2Word_EUR` dict, so the `lang_EN` mutation trap
//! described in `PORTING_CURRENCY.md` does not apply here — Yoruba never sees
//! EN's ~24 extra codes. `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are
//! inherited from `Num2Word_Base` and are both `{}`, so the trait defaults
//! (`None` / `100`) are already correct and are not overridden. Verified
//! against the live interpreter.
//!
//! # Faithfully reproduced Python bugs / oddities — currency
//!
//! 6. **An unknown currency code silently falls back to NGN** rather than
//!    raising. `to_currency` looks the code up with
//!    `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`, and
//!    `values()[0]` is NGN because it is first in the dict literal. So
//!    `to_currency(1, currency="GBP")` == `"ọkan náírà"` — pounds rendered as
//!    naira. The corpus pins this for GBP/JPY/KWD/BHD/INR/CNY/CHF.
//!    `to_cheque` does **not** share the fallback: it indexes
//!    `CURRENCY_FORMS[currency]` and raises `NotImplementedError`. Hence
//!    [`Lang::currency_forms`] here reports the honest `None` for a missing
//!    code (so cheque raises) and the NGN fallback lives *only* inside
//!    [`LangYo::to_currency`], exactly mirroring the two Python call sites.
//!
//! 7. **Cents are truncated to 2 digits, never rounded.** The subunit comes
//!    from `int(parts[1][:2].ljust(2, "0"))` — a *string* slice of `str(val)`.
//!    So `2.675` -> 67 cents (not 68; there is no ROUND_HALF_UP anywhere),
//!    `1.005` -> 0 cents, and `0.0001` -> `"òdo euros"`. Conversely `1.5`
//!    -> `"5"` -> ljust -> `"50"` -> 50 cents, i.e. the slice is positional,
//!    not numeric.
//!
//! 8. **`CURRENCY_PRECISION` is ignored entirely.** JPY (0-decimal) and
//!    KWD/BHD (3-decimal) are not in `CURRENCY_FORMS` anyway, so they take
//!    the NGN fallback *and* the hardcoded 2-digit cent slice: `to_currency
//!    (12.34, currency="JPY")` == `"mẹwa méjì náírà ọgbọn mẹrin kóbò"` —
//!    yen with naira subunits. Both corpus-pinned.
//!
//! 9. **`adjective` is accepted and then ignored.** `Num2Word_YO.to_currency`
//!    never reads it (and `CURRENCY_ADJECTIVES` is empty regardless), so
//!    `adjective=True` changes nothing.
//!
//! 10. **`cents=False` drops the subunit clause entirely.** Python's guard is
//!    `if cents and right:`. In `Num2Word_Base` `cents=False` means "render
//!    the subunit as digits" (`_cents_terse`); here it means "omit". So
//!    `to_currency(12.34, cents=False)` == `"mẹwa méjì euros"`.
//!
//! 11. **A float whose `repr` is scientific crashes or silently mis-reads.**
//!    Because the split is done on `str(val)` rather than on the number,
//!    `str(1e16)` == `"1e+16"` reaches `int()` and raises `ValueError`, while
//!    `str(1.25e16)` == `"1.25e+16"` splits into `"1"` / `"25e+16"`, whose
//!    first two chars are `"25"` — so `1.25e16` renders as
//!    `"ọkan euro ogún marun cents"` (one euro, twenty-five cents). See
//!    [`py_str`] for how this is reproduced.
//!
//! 12. **The `separator` default is `" "`, not Base's `","`.** Python's
//!    signature is `to_currency(self, val, currency="NGN", cents=True,
//!    separator=" ", adjective=False)`, so the corpus — which never passes
//!    `separator=` — records `"mẹwa méjì euros ọgbọn mẹrin cents"` rather than
//!    `"...euros,ọgbọn..."`. Not a bug so much as a divergence from Base that
//!    is easy to lose: [`Lang::default_separator`] carries it, and
//!    `to_currency` receives `None` when the caller omitted the kwarg.
//!
//! # Float/Decimal cardinal
//!
//! `Num2Word_YO.to_cardinal` handles non-integers inline: it never calls
//! `Num2Word_Base.to_cardinal_float`, never runs `float2tuple`, and does **no
//! arithmetic at all** — the whole algorithm is string surgery on
//! `str(number)`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"): n = n[1:]; ret = self.negword
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     for digit in right:
//!         ret += self._int_to_word(int(digit)) + " "
//!     return ret.strip()
//! ```
//!
//! So [`LangYo::to_cardinal_float`] reconstructs the text Python saw —
//! `repr(float)` via the same [`py_str`] the currency port already uses, and
//! `str(Decimal)` via [`decimal_str_abs`] — then runs the string algorithm.
//! The base-path traps (banker's rounding, the `< 0.01` float2tuple rescue)
//! do not apply here: for Yoruba the decimal *string* is the spec, so `2.675`
//! reads its repr digits verbatim (`"méjì point mẹfa meje marun"`) and `1.005`
//! keeps its leading zeros (`"ọkan point òdo òdo marun"`).
//!
//! # Faithfully reproduced Python bugs / oddities — float cardinal
//!
//! 13. **A scientific-notation repr raises ValueError**, same family as
//!     bug 11: `str(1e-05)` == `"1e-05"` has no `"."`, so the whole literal
//!     hits `int()` — `invalid literal for int() with base 10: '1e-05'` —
//!     while `str(1.5e-05)` == `"1.5e-05"` *does* split, and the digit loop
//!     dies on the `'e'` (`... base 10: 'e'`). Decimals keep their own
//!     spelling: `Decimal("1E-7")` fails on `'1E-7'`, `Decimal("1.5E-7")` on
//!     `'E'` (capital, unpadded — Decimal's `__str__`, not float `repr`).
//!     All four messages verified against the live interpreter.
//!
//! 14. **The `precision=` kwarg is a no-op.** The dispatcher sets
//!     `converter.precision` (the attribute exists, inherited from Base), but
//!     `Num2Word_YO.to_cardinal` never reads it — the digit count comes from
//!     the text. Base-path languages honour it; Yoruba ignores it, so
//!     `precision_override` is deliberately discarded. Verified live:
//!     `to_cardinal(3.14)` with `precision=5` is still
//!     `"mẹta point ọkan mẹrin"`.
//!
//! 15. **A whole float keeps its fractional clause**: `repr(1.0)` is `"1.0"`,
//!     so Python renders `"ọkan point òdo"`, never bare `"ọkan"` (corpus rows
//!     `0.0`/`1.0` pin this). The dispatcher routes whole floats to the Python
//!     converter, so the Rust hook only ever *receives* non-whole values — but
//!     the reconstruction appends `repr`'s `".0"` anyway, so both sides agree
//!     even on a direct call.
//!
//! 16. **Float noise is spelled out digit by digit.** There is no rounding
//!     rescue anywhere, so `0.30000000000000004` becomes `"òdo point mẹta"`
//!     followed by fifteen `"zero"`s and a `"mẹrin"` — 17 fraction words,
//!     exactly the repr digits. Similarly the `>= 10**9` fallback (bug 1)
//!     applies to the integer part: `Decimal("98746251323029.99")` is
//!     `"98746251323029 point mẹsan mẹsan"` (corpus-pinned).
//!
//! # Error variants
//!
//! The four integer modes raise nothing: there is no overflow ceiling
//! (bug 1), no ordinal guard (bug 3), and no dict/list lookup that can miss —
//! every index into `ones`/`tens` is derived from `n // 10 % 10` and is in
//! range by construction. All four modes are total over the integers.
//!
//! The currency and float surfaces add exactly two:
//!
//! * `N2WError::Value` — `to_currency` or the float cardinal on a value whose
//!   `str()` is scientific notation, where Python's `int()` chokes on the
//!   literal (bugs 11 and 13).
//! * `N2WError::NotImplemented` — `to_cheque` with a code outside
//!   {NGN, USD, EUR}, raised by the inherited `Num2Word_Base.to_cheque`.
//!   `to_currency` never raises this (bug 6).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). No reliable Yoruba cent noun was found, so USD and EUR raise
//! NotImplementedError. Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
#[cfg(test)] // only the tests build BigDecimals
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `_int_to_word(0)`. Python writes `self.ones[0] if self.ones[0] else "zero"`,
/// and `ones[0]` is `""`, so this constant is the only reachable result.
const ZERO: &str = "òdo";

/// `setup()`: `self.negword = "minus "` — the trailing space is load-bearing.
/// `to_cardinal` concatenates it directly (`ret + word`) rather than joining,
/// then `.strip()`s the result.
const NEGWORD: &str = "minus ";

/// `self.ones`, indices 0..=9. Index 0 is `""` (see [`ZERO`] and bug 4); it is
/// never selected by the `number < 10` arm, which is guarded by `number != 0`.
const ONES: [&str; 10] = [
    "", "ọkan", "méjì", "mẹta", "mẹrin", "marun", "mẹfa", "meje", "mẹjọ", "mẹsan",
];

/// `self.tens`, indices 0..=9, keyed by the tens digit. Index 0 is `""` and is
/// unreachable: the `number < 100` arm requires `number >= 10`, so
/// `number // 10 >= 1`. Note index 1 (`"mẹwa"`, ten) is reused for the teens,
/// so 11 is `"mẹwa ọkan"` — literally "ten one", with no dedicated teen words.
const TENS: [&str; 10] = [
    "",
    "mẹwa",
    "ogún",
    "ọgbọn",
    "ogójì",
    "àádọta",
    "ọgọta",
    "àádọrin",
    "ọgọrin",
    "àádọrún",
];

/// `self.hundred`.
const HUNDRED: &str = "ọgọrun";
/// `self.thousand`.
const THOUSAND: &str = "ẹgbẹrun";
/// `self.million`.
const MILLION: &str = "miliọnu";

/// `to_ordinal`'s fixed suffix — see bug 3.
const ORDINAL_SUFFIX: &str = "-kẹta";

/// The ceiling of `_int_to_word`'s word-forming branches. At or above this,
/// Python returns `str(number)` verbatim (bug 1).
const FALLBACK_THRESHOLD: u64 = 1_000_000_000;

/// The code `list(CURRENCY_FORMS.values())[0]` resolves to — the first key in
/// `Num2Word_YO`'s dict literal, which Python dicts preserve in insertion
/// order. `to_currency` uses it as the fallback for an unknown code (bug 6).
const FALLBACK_CURRENCY: &str = "NGN";

/// `Num2Word_YO.CURRENCY_FORMS`, in the dict literal's insertion order.
///
/// Defined directly on `Num2Word_YO`, so — unlike the 16 classes that read the
/// shared `Num2Word_EUR` dict — this table is **not** touched by
/// `Num2Word_EN.__init__`'s in-place mutation, and Yoruba never sees EN's ~24
/// extra codes. Verified against the live interpreter: `CURRENCY_FORMS.keys()`
/// is exactly `['NGN', 'USD', 'EUR']`.
///
/// The order is load-bearing: `to_currency`'s fallback is
/// `list(CURRENCY_FORMS.values())[0]`, and Python dicts preserve insertion
/// order, so NGN — declared first — is what an unknown code resolves to
/// (bug 6). [`FALLBACK_CURRENCY`] pins that as a named lookup rather than
/// relying on `HashMap` iteration order, which is arbitrary.
///
/// Every entry carries exactly two forms, matching the Python tuples, so the
/// `cr1[1]`/`cr2[1]` indexing below can never raise IndexError. NGN's two unit
/// forms are deliberately identical (`("náírà", "náírà")`) — Yoruba does not
/// inflect the noun for number here — which is why "ọkan náírà" and "méjì
/// náírà" share a word where USD alternates dollar/dollars.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("NGN", CurrencyForms::new(&["náírà", "náírà"], &["kóbò", "kóbò"]));
    // USD/EUR were English ("dollars", "cents"); no reliable Yoruba subunit
    // noun was found, so they raise NotImplementedError (#222).
    m
}

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 1] = [(9, "biliọnu")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

pub struct LangYo {
    /// Built once in [`LangYo::new`] and only ever read. `to_currency` runs on
    /// every call, and rebuilding this table per call is what made an earlier
    /// revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

/// Python's `int(s)` for a `str`, reproducing its `ValueError`.
///
/// `Num2Word_YO.to_currency` feeds `int()` slices of `str(val)`, which for a
/// scientific-notation `repr` are not numeric at all (bug 11). The exception
/// type is the observable part — `bench/diff_test.py` compares
/// `type(e).__name__` — and `N2WError::Value` is what the generated binding
/// maps to `PyValueError`.
///
/// Python additionally accepts `_` separators and non-ASCII decimal digits.
/// Neither can occur here: every string handed to this function is produced by
/// [`py_str`] out of a `BigInt`'s ASCII digits, so the narrower grammar is
/// exact for all reachable input.
fn py_int(s: &str) -> Result<BigInt> {
    let trimmed = s.trim_matches(|c: char| c.is_ascii_whitespace());
    let digits = trimmed
        .strip_prefix(['+', '-'])
        .unwrap_or(trimmed);
    if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
        if let Ok(v) = BigInt::from_str(trimmed) {
            return Ok(v);
        }
    }
    // Python: ValueError: invalid literal for int() with base 10: '1e+16'
    Err(N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        s
    )))
}

/// Reconstruct Python's `str(value)` for the value the shim stringified.
///
/// # Why this exists
///
/// `Num2Word_YO.to_currency` does not do arithmetic on the number — it does
/// `str(val).split(".")` and slices the *text*. Every one of its quirks (the
/// 2-char truncation of bug 7, the `ValueError` and the 25-cents misread of
/// bug 11) is a property of that text. So the faithful port reconstructs the
/// text and then runs Python's algorithm on it, rather than re-deriving the
/// split numerically and hoping the edge cases agree.
///
/// # Why it is not a `repr(float)` reimplementation
///
/// `currency.rs` deliberately keeps shortest-round-trip float formatting on
/// the Python side, and this does not bring it back. The *digits* arrive
/// already chosen: the shim sent `str(value)`, and `bigdecimal`'s parser
/// stores them verbatim as `(int_val, scale)` without normalising, so
/// `as_bigint_and_exponent()` hands back exactly the digits `repr` produced.
/// All that is recovered here is the **layout** — where `repr` put the point,
/// and whether it chose exponential form — which is a plain range check on the
/// decimal exponent, not a formatting algorithm.
///
/// `decpt` is the decimal point's position relative to the digit string
/// (`value == 0.<digits> * 10^decpt`). CPython's `float_repr` emits fixed
/// notation for `-4 < decpt <= 16` and exponential outside it, which is why
/// `str(1e15)` is `"1000000000000000.0"` but `str(1e16)` is `"1e+16"`, and
/// `str(1e-4)` is `"0.0001"` but `str(1e-5)` is `"1e-05"`. Verified against
/// the interpreter across that whole boundary.
///
/// # Limits
///
/// `digits` must be the **absolute** value: Python evaluates `val = abs(val)`
/// before `str(val)`, so a negative never reaches the text. (`-0.0` is the one
/// exception — `-0.0 < 0` is `False`, so Python stringifies `"-0.0"` and
/// `int("-0")` yields `0`. `bigdecimal` has no signed zero, so this returns
/// `"0.0"` and `int("0")` yields the same `0`. Indistinguishable in output.)
///
/// The layout rule is `repr(float)`'s, and it is **exact for every `float` and
/// `int`** — the two types the corpus exercises. Fuzzed against the live
/// interpreter over ~125k `to_currency`/`to_cheque` calls, including a sweep of
/// `{1.0, 1.25, 9.99, 5.0, 1.5} x 10^{-12..23}` that straddles both
/// notation boundaries in both directions: zero mismatches.
///
/// `decimal.Decimal` — which `num2words()` also accepts (`__init__.py` admits
/// `(int, float, Decimal)` and sends `str(number)`) — formats itself by
/// *different* rules, and there the reconstruction can diverge. `str(Decimal)`
/// preserves the literal's own notation; `repr(float)` imposes the decpt rule
/// above. Three known divergences, all verified against the interpreter:
///
/// | input | Python | this port |
/// |---|---|---|
/// | `Decimal("0.00001")` | `"òdo euros"` | ValueError |
/// | `Decimal("1e-5")`    | `"òdo euros"` | ValueError |
/// | `Decimal("1E+2")`    | ValueError     | `"ọkan ọgọrun euros"` |
///
/// The first two are **not fixable here, even in principle**:
/// `Decimal("0.00001")` and the float `1e-05` both parse to exactly
/// `(digits: 1, scale: 5)`, yet Python renders one `"òdo euros"` and raises on
/// the other. The distinguishing information is the *spelling*, which
/// `CurrencyValue::parse` consumes before this file is reached. The fix belongs
/// at the boundary — `CurrencyValue::Decimal` would have to carry the original
/// `str(val)` alongside the parsed number, the same way `has_decimal` already
/// crosses because it too cannot be recovered from the value. That is a
/// `currency.rs` + binding change, out of scope for a single-language port.
///
/// `Decimal("1E+2")` *is* separable (a float repr can never yield `scale < 0`
/// with `decpt <= 16`), but only via a Decimal-specific layout heuristic that
/// no corpus row pins — and it would not rescue the two cases above. Guessing
/// there would trade a flagged, understood gap for an unflagged one, so it is
/// deliberately not attempted. Reported as a concern instead.
fn py_str(digits: &BigInt, scale: i64) -> String {
    debug_assert!(!digits.is_negative(), "py_str expects abs(value)'s digits");
    let ds = digits.to_string();
    // `ds` is BigInt::to_string of a non-negative — ASCII digits only, so the
    // byte slicing below is char-safe by construction.
    let decpt = ds.len() as i64 - scale;

    if decpt > 16 || decpt < -3 {
        // Exponential: one digit, then the rest, then a >=2-digit exponent.
        // repr never emits a trailing-zero mantissa, and `ds` came from repr.
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
        // Integral: repr appends ".0" (Py_DTSF_ADD_DOT_0). 1e15, 100.0, ...
        format!("{}{}.0", ds, "0".repeat(decpt as usize - ds.len()))
    } else {
        format!("{}.{}", &ds[..decpt as usize], &ds[decpt as usize..])
    }
}

/// `repr(abs(f))` for the float arm of the cardinal path.
///
/// `Num2Word_YO.to_cardinal` runs on `str(number)`, so the port needs the
/// exact text Python saw. The digits come from Rust's `{}` formatting —
/// shortest-round-trip, the same digit choice as CPython's `repr` — and the
/// *layout* (where the point goes, whether repr chose exponential form, the
/// `".0"` suffix on integral values) is recovered by the same [`py_str`] the
/// currency port already fuzzed against the interpreter.
///
/// `{}` never prints exponential form, so its output is normalised to
/// `(digits, scale)` first: an integral rendering like `"15000000000000000"`
/// carries positional trailing zeros that repr's mantissa would not
/// (`repr(1.5e16)` == `"1.5e+16"`), hence the trailing-zero strip. A
/// fractional rendering can never end in `'0'` — a shorter string would
/// round-trip to the same value, contradicting shortest-form — so the strip
/// only ever fires on the integral case.
///
/// Non-finite guards are defensive: the dispatcher's whole-float check
/// swallows `inf`/`nan` before Rust is reached (`int(inf)` raises in the
/// guard itself). Python's spellings are `"inf"`/`"nan"`, which then fail
/// `int()` exactly like a scientific literal (bug 13's family).
fn float_repr_abs(f: f64) -> String {
    let a = f.abs();
    if a.is_nan() {
        return "nan".to_string();
    }
    if a.is_infinite() {
        return "inf".to_string();
    }
    let s = format!("{}", a);
    let (digit_text, mut scale) = match s.split_once('.') {
        Some((int_part, frac)) => (format!("{}{}", int_part, frac), frac.len() as i64),
        None => (s, 0),
    };
    let mut digits = BigInt::from_str(&digit_text)
        .expect("f64 Display of a finite value is ASCII digits and an optional point");
    if digits.is_zero() {
        // py_str(0, 0) -> "0.0", matching repr(0.0).
        return py_str(&digits, 0);
    }
    let ten = BigInt::from(10u8);
    while (&digits % &ten).is_zero() {
        digits = &digits / &ten;
        scale -= 1;
    }
    py_str(&digits, scale)
}

/// Python's `str(Decimal)` — the spec's *to-scientific-string* — for the
/// **absolute** value, from `BigDecimal`'s verbatim `(digits, scale)`.
///
/// This is *not* [`py_str`]: `Decimal.__str__` follows different rules from
/// float `repr`. Fixed notation applies iff `exponent <= 0` and the adjusted
/// exponent (`exponent + len(digits) - 1`) is `>= -6`; outside that the form
/// is exponential with a **capital `E`**, an explicit sign, and **no zero
/// padding** — `str(Decimal("1e-7"))` is `"1E-7"`, where a float repr would
/// be `"1e-07"`. Trailing coefficient zeros survive (`Decimal("1.10")` ->
/// `"1.10"`, `Decimal("1.50E-7")` -> `"1.50E-7"`) because `bigdecimal`
/// stores the parsed digits unnormalised. All spellings verified against the
/// live interpreter.
///
/// The dispatcher only forwards non-whole Decimals (`exponent < 0`), so the
/// `exponent > 0` positive-exponent arm (`"1E+2"`) is defensive — reachable
/// only by calling the core directly.
fn decimal_str_abs(digits: &BigInt, scale: i64) -> String {
    debug_assert!(!digits.is_negative(), "decimal_str_abs expects abs(value)'s digits");
    let ds = digits.to_string();
    let exponent = -scale;
    let adjusted = exponent + ds.len() as i64 - 1;

    if exponent <= 0 && adjusted >= -6 {
        if exponent == 0 {
            return ds;
        }
        // Digits left of the point; exponent < 0 here so point < ds.len().
        let point = ds.len() as i64 + exponent;
        if point <= 0 {
            // Decimal("0.01") -> digits "1", point -1 -> "0.01".
            format!("0.{}{}", "0".repeat((-point) as usize), ds)
        } else {
            // ds is ASCII digits, so byte slicing is char-safe.
            format!("{}.{}", &ds[..point as usize], &ds[point as usize..])
        }
    } else {
        let mantissa = if ds.len() > 1 {
            format!("{}.{}", &ds[..1], &ds[1..])
        } else {
            ds
        };
        let sign = if adjusted < 0 { '-' } else { '+' };
        format!("{}E{}{}", mantissa, sign, adjusted.abs())
    }
}

impl Default for LangYo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangYo {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        LangYo {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_YO._int_to_word`.
    ///
    /// The Python branches only ever compare `number` against 10, 100, 1000,
    /// 10^6 and 10^9, so once the value is known to be below `10**9` it fits
    /// in a `u64` and the arithmetic can be done there — see
    /// [`LangYo::small_to_word`]. Anything at or above `10**9` (including
    /// values far beyond `u64::MAX`, which `to_u64` reports as `None`) takes
    /// the `str(number)` fallback with full `BigInt` precision.
    fn int_to_word(&self, number: &BigInt) -> String {
        // Python: `if number == 0: return self.ones[0] if self.ones[0] else "zero"`
        if number.is_zero() {
            return ZERO.to_string();
        }

        // Python: `if number < 0: return self.negword + self._int_to_word(abs(number))`
        //
        // Unreachable from to_cardinal/to_ordinal/to_year — all of them route
        // through to_cardinal, which strips the sign from the *string* first
        // (bug 5). Reproduced for fidelity, not because a test can hit it.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // Scale words above a million (gladiaio/num2words2#147), composed like
        // the million arm. Every entry point rejects values at or above
        // `maxval_ceiling()` first, so the top quotient is always below 1000.
        for &(exp, word) in SCALES.iter().rev() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let mut result = format!("{} {}", self.int_to_word(&(number / &scale)), word);
                let rest = number % &scale;
                if !rest.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&rest));
                }
                return result;
            }
        }

        match number.to_u64() {
            Some(n) if n < FALLBACK_THRESHOLD => self.small_to_word(n),
            // Python: `else: return str(number)  # Fallback for very large numbers`
            _ => number.to_string(),
        }
    }

    /// The word-forming half of `_int_to_word`, for `1 <= n < 10**9`.
    ///
    /// Every recursive call shrinks `n`, and each argument stays inside the
    /// same range, so the `u64` domain holds throughout.
    fn small_to_word(&self, n: u64) -> String {
        debug_assert!(n > 0 && n < FALLBACK_THRESHOLD);

        if n < 10 {
            // Python: `return self.ones[number]`
            ONES[n as usize].to_string()
        } else if n < 100 {
            // Python: tens_val = number // 10; ones_val = number % 10
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                TENS[tens_val].to_string()
            } else {
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            }
        } else if n < 1000 {
            // Python: result = self.ones[hundreds_val] + " " + self.hundred
            // Note: no bare-hundred special case — 100 is "ọkan ọgọrun" (bug 2).
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        } else if n < 1_000_000 {
            // Python: result = self._int_to_word(thousands_val) + " " + self.thousand
            let thousands_val = n / 1000;
            let remainder = n % 1000;
            let mut result = format!("{} {}", self.small_to_word(thousands_val), THOUSAND);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        } else {
            // n < 10**9 — Python: result = self._int_to_word(millions_val) + " " + self.million
            let millions_val = n / 1_000_000;
            let remainder = n % 1_000_000;
            let mut result = format!("{} {}", self.small_to_word(millions_val), MILLION);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            result
        }
    }
}

impl Lang for LangYo {
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

    /// `to_ordinal(float/Decimal)`. YO's `to_ordinal` is
    /// `self.to_cardinal(number) + "-kẹta"` for *every* input, so the float
    /// entry is the float cardinal plus the suffix — "marun point òdo-kẹta".
    /// An exponent-form Decimal repr ("1E+2") still dies in `int()` with
    /// ValueError inside the cardinal, before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr the
    /// binding computed, dot appended, sign and exponent form included
    /// ("-0.0.", "1e+16.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside YO's `int("Infinity")` (`to_cardinal` reads
    /// `str(number)`, strips the sign, finds no "." and calls `int()`).
    /// The binding otherwise hard-codes `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any YO code runs, so the
    /// ValueError must be raised here. The sign is sliced off before
    /// `int()` sees it, so both signs quote the same literal.
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
        "NGN"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    /// `setup()`: `"minus "`, trailing space included.
    fn negword(&self) -> &str {
        NEGWORD
    }

    /// `setup()`: `"point"`. Consumed by the float branch of `to_cardinal`
    /// ([`LangYo::to_cardinal_float`] here).
    fn pointword(&self) -> &str {
        "point"
    }

    /// Port of `Num2Word_YO.to_cardinal`, integer path only.
    ///
    /// Python works on `str(number).strip()`: it peels a leading `"-"` off the
    /// *string* and sets `ret = self.negword`, then checks for `"."`.
    /// `str(int)` never contains a `"."`, so integers always take the `else`
    /// branch — `(ret + self._int_to_word(int(n))).strip()`. The float branch
    /// (`pointword`, digit-by-digit decimals) lives in
    /// [`LangYo::to_cardinal_float`].
    ///
    /// The trailing `.strip()` is what removes `negword`'s trailing space from
    /// the join, and it can never eat anything else: `_int_to_word` returns a
    /// non-empty, unpadded string for every input.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        let words = self.checked_int_to_word(&n)?;
        Ok(format!("{}{}", ret, words).trim().to_string())
    }

    /// Port of `Num2Word_YO.to_ordinal`: `self.to_cardinal(number) + "-kẹta"`.
    ///
    /// A fixed suffix with no guards — 0 and negatives both pass through
    /// (bug 3), and a value >= 10^9 yields digits + suffix (bug 1).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_YO.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Not the trait default (`value.to_string()`), which omits the dot.
    /// Negatives keep their sign: `to_ordinal_num(-1)` == `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_YO.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The Python signature takes `longval=True` but never reads it; there is
    /// no era/century handling, so a year is just a cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_YO.to_cardinal`, float/Decimal branch.
    ///
    /// Python never reaches `Num2Word_Base.to_cardinal_float` — its own
    /// `to_cardinal` works on `str(number).strip()`: peel a leading `"-"`
    /// (setting `ret = self.negword`), split at the first `"."`, spell the
    /// left side through `_int_to_word(int(left))`, then each fraction
    /// *character* through `_int_to_word(int(digit))`, and `.strip()` the
    /// join. No `float2tuple`, no rounding, no banker's-tie anywhere — the
    /// decimal text **is** the spec, so this reconstructs it
    /// ([`float_repr_abs`] / [`decimal_str_abs`]) and mirrors the string
    /// algorithm step for step.
    ///
    /// `int()` is where scientific notation dies (bug 13): a dotless literal
    /// (`"1e-05"`, `"1E-7"`, `"inf"`) fails wholesale in the `else` branch,
    /// while a dotted one (`"1.5e-05"`) fails on the first non-digit char of
    /// the fraction loop (`'e'` / `'E'`). [`py_int`] carries the exact
    /// message. Error order matches Python: `int(left)` is evaluated before
    /// any fraction digit.
    ///
    /// `precision_override` is deliberately ignored (bug 14): the dispatcher
    /// sets `converter.precision`, but this method never reads it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // Bug 14: Num2Word_YO.to_cardinal derives everything from the text;
        // self.precision (and therefore precision=) is never consulted.
        let _ = precision_override;

        // Python: `n = str(number).strip()` then `n.startswith("-")`.
        // For floats the sign test must see -0.0 (repr "-0.0" starts with
        // "-"), so it is sign-bit-based, not `< 0.0`; NaN is excluded because
        // CPython spells every NaN "nan", never "-nan". Both cases are
        // dispatcher-unreachable (whole / non-finite) — defensive parity only.
        let (is_negative, n) = match value {
            FloatValue::Float { value: f, .. } => {
                (f.is_sign_negative() && !f.is_nan(), float_repr_abs(*f))
            }
            FloatValue::Decimal { value: d, .. } => {
                let (digits, scale) = d.abs().as_bigint_and_exponent();
                (d.is_negative(), decimal_str_abs(&digits, scale))
            }
        };

        // Python: `ret = self.negword` — trailing space included; the final
        // `.strip()` (here `trim()`) is what tidies the join.
        let ret = if is_negative { NEGWORD } else { "" };

        match n.split_once('.') {
            // Python: `left, right = n.split(".", 1)` — repr/str(Decimal)
            // contain at most one ".", so split_once is exact.
            Some((left, right)) => {
                // Python: `ret += self._int_to_word(int(left)) + " " +
                //          self.pointword + " "`
                let mut out = String::from(ret);
                out.push_str(&self.checked_int_to_word(&py_int(left)?)?);
                out.push(' ');
                out.push_str(self.pointword());
                out.push(' ');
                // Python: `for digit in right: ret += self._int_to_word(
                //          int(digit)) + " "` — a '0' char renders "òdo",
                //          which is how 1.005 keeps its leading zeros.
                for ch in right.chars() {
                    out.push_str(&self.checked_int_to_word(&py_int(&ch.to_string())?)?);
                    out.push(' ');
                }
                // Python: `return ret.strip()`.
                Ok(out.trim().to_string())
            }
            // Python's `else` — a dotless text (scientific repr, inf/nan).
            // `int(n)` raises ValueError with the full literal (bug 13).
            None => Ok(format!("{}{}", ret, self.checked_int_to_word(&py_int(&n)?)?)
                .trim()
                .to_string()),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_YO` overrides `to_currency` wholesale and defines
    // `CURRENCY_FORMS`; everything else on the currency surface is inherited
    // from `Num2Word_Base` unchanged, so only these three hooks are overridden
    // and the trait defaults cover the rest:
    //
    //   * `currency_adjective`  — `CURRENCY_ADJECTIVES` is `{}` (default None)
    //   * `currency_precision`  — `CURRENCY_PRECISION` is `{}` (default 100),
    //                             and `to_currency` ignores it anyway (bug 8)
    //   * `pluralize`           — never reached; `to_currency` inlines
    //                             `cr1[1] if left != 1 else cr1[0]` and
    //                             `to_cheque` takes `cr1[-1]` directly. Left at
    //                             the raising default, exactly as Python leaves
    //                             it abstract.
    //   * `money_verbose`       — default (`to_cardinal`), which is what
    //                             `Num2Word_Base.to_cheque` calls.
    //   * `_cents_verbose`/`_cents_terse` — unreachable (bug 10).
    //   * `to_cheque`           — not overridden in Python; the trait default
    //                             (`currency::default_to_cheque`) is the port.
    // Both verified against the live interpreter.

    fn lang_name(&self) -> &str {
        "Num2Word_YO"
    }

    /// `CURRENCY_FORMS[code]` — an honest `None` for a code outside
    /// {NGN, USD, EUR}.
    ///
    /// This is the *strict* lookup, and it is the one `to_cheque` consumes:
    /// `Num2Word_Base.to_cheque` subscripts `self.CURRENCY_FORMS[currency]` and
    /// converts the `KeyError` into `NotImplementedError`, which the corpus
    /// pins for GBP/JPY/KWD/BHD/INR/CNY/CHF.
    ///
    /// `to_currency`'s NGN fallback (bug 6) deliberately does **not** live
    /// here — it is a `.get(currency, ...)` at that one Python call site, not a
    /// property of the table. Folding it in would make `to_cheque` return
    /// naira for GBP instead of raising.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_YO.to_currency` — a wholesale override that never
    /// reaches `Num2Word_Base.to_currency`.
    ///
    /// Python re-derives the split from the *text* of `str(val)` rather than
    /// from the number, so `parse_currency_parts`, `pluralize`,
    /// `_cents_verbose`, `_cents_terse` and `CURRENCY_PRECISION` all sit unused.
    /// This mirrors the text algorithm step for step (see [`py_str`] for how
    /// `str(val)` is recovered):
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Consequences kept verbatim: cents are **truncated** to two digits with
    /// no rounding anywhere (2.675 -> 67, not 68 — bug 7); the slice is
    /// positional, so 1.5 -> `"5"` -> `"50"` -> 50 cents; `CURRENCY_PRECISION`
    /// is ignored, so JPY takes naira and 2-digit cents (bug 8); `adjective` is
    /// read and discarded (bug 9); and `cents=False` **omits** the subunit
    /// clause rather than rendering it as digits (bug 10).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Bug 9: `Num2Word_YO.to_currency` accepts `adjective` and never reads
        // it. `CURRENCY_ADJECTIVES` is empty regardless, so there is nothing to
        // prefix even if it did.
        let _ = adjective;
        // `None` = the Python caller omitted `separator=`, so the method's own
        // default applies. That default is `" "` (not Base's `","`), and the
        // trait resolves it through `default_separator` below.
        let separator = separator.unwrap_or(self.default_separator());

        // Python: `if val < 0: is_negative = True; val = abs(val)`.
        // `-0.0 < 0` is False, and BigDecimal has no signed zero, so both sides
        // agree that -0.0 is non-negative.
        let is_negative = val.is_negative();

        // Python: `parts = str(val).split(".")`, evaluated *after* the abs().
        let text = match val {
            // `str(int)` never contains a "." — the cents clause is dead for a
            // true int, which is why 100 is "ọkan ọgọrun euros" and not
            // "... zero cents".
            CurrencyValue::Int(v) => v.abs().to_string(),
            // `has_decimal` is not consulted: Python's guard here is the
            // truthiness of `right`, not the presence of a point. That is why
            // `Decimal("5.00")` renders "marun euros" with no cents clause even
            // though its text *does* have a point.
            CurrencyValue::Decimal { value, .. } => {
                let (digits, scale) = value.abs().as_bigint_and_exponent();
                py_str(&digits, scale)
            }
        };

        let mut parts = text.split('.');
        // `str.split` always yields at least one element.
        let p0 = parts.next().unwrap_or("");
        // `None` here is Python's `len(parts) > 1` being False.
        let p1 = parts.next();

        // Python: `left = int(parts[0]) if parts[0] else 0`.
        // `int()` is where a scientific-notation repr raises ValueError (bug 11).
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            py_int(p0)?
        };

        // Python: `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                // `[:2]` then `.ljust(2, "0")` — a *character* slice, padded on
                // the right. Always yields exactly two chars, so `right` lands
                // in 0..=99 for every reachable input, including the "25e+16"
                // misread of bug 11.
                let mut head: String = frac.chars().take(2).collect();
                while head.chars().count() < 2 {
                    head.push('0');
                }
                py_int(&head)?
            }
            _ => BigInt::zero(),
        };

        // Python: `cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        //
        // Bug 6: an unknown code silently falls back to the dict's first value
        // — NGN — instead of raising. `.expect` is sound because
        // FALLBACK_CURRENCY is a key this file's own table always inserts.
        let forms = self.currency_forms.get(currency).unwrap_or_else(|| {
            self.currency_forms
                .get(FALLBACK_CURRENCY)
                .expect("build_currency_forms always inserts the fallback code")
        });

        // Python indexes the tuples directly: `cr1[1] if left != 1 else cr1[0]`.
        // Every entry has two forms, so the IndexError arm is unreachable — but
        // it is mapped rather than panicked so the exception *type* survives if
        // the table ever changes.
        let pick = |tuple: &[String], plural: bool| -> Result<String> {
            tuple
                .get(usize::from(plural))
                .cloned()
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
        };

        // Python: `result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            pick(&forms.unit, !left.is_one())?
        );

        // Python: `if cents and right:` — bug 10, `cents=False` omits the
        // clause entirely rather than rendering `_cents_terse` digits.
        if cents && !right.is_zero() {
            // Python: `result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])`
            result.push_str(separator);
            result.push_str(&self.checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(&pick(&forms.subunit, !right.is_one())?);
        }

        // Python: `if is_negative: result = self.negword + result`
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // Python: `return result.strip()`. This is what absorbs NEGWORD's
        // trailing space into the join.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    /// The shim's precision: `abs(Decimal(str(value)).as_tuple().exponent)`.
    /// LangYo ignores it (bug 14), but the tests pass the real value anyway
    /// so the call shape matches production.
    fn card_f(v: f64, precision: u32) -> Result<String> {
        LangYo::new().to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
    }

    fn card_d(s: &str) -> Result<String> {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.max(0) as u32;
        LangYo::new().to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
    }

    /// Every float `cardinal` corpus row with a "." in `arg`, plus the live
    /// interpreter's outputs for the trap values. Expected strings are pinned
    /// from `bench/corpus.jsonl` and the pure-Python converter (Rust fast
    /// path bypassed).
    #[test]
    fn corpus_float_rows() {
        for (v, p, want) in [
            (0.0, 1, "òdo point òdo"),
            (0.5, 1, "òdo point marun"),
            (1.0, 1, "ọkan point òdo"),
            (1.5, 1, "ọkan point marun"),
            (2.25, 2, "méjì point méjì marun"),
            (3.14, 2, "mẹta point ọkan mẹrin"),
            (0.01, 2, "òdo point òdo ọkan"),
            (0.1, 1, "òdo point ọkan"),
            (0.99, 2, "òdo point mẹsan mẹsan"),
            (1.01, 2, "ọkan point òdo ọkan"),
            (12.34, 2, "mẹwa méjì point mẹta mẹrin"),
            (99.99, 2, "àádọrún mẹsan point mẹsan mẹsan"),
            (100.5, 1, "ọkan ọgọrun point marun"),
            (
                1234.56,
                2,
                "ọkan ẹgbẹrun méjì ọgọrun ọgbọn mẹrin point marun mẹfa",
            ),
            (-0.5, 1, "minus òdo point marun"),
            (-1.5, 1, "minus ọkan point marun"),
            (-12.34, 2, "minus mẹwa méjì point mẹta mẹrin"),
            // The f64-artefact values: for yo the repr digits are the spec —
            // no float2tuple, no rounding, leading zeros kept.
            (1.005, 3, "ọkan point òdo òdo marun"),
            (2.675, 3, "méjì point mẹfa meje marun"),
            (0.0001, 4, "òdo point òdo òdo òdo ọkan"),
            // Float noise spelled digit by digit (bug 16).
            (
                0.30000000000000004,
                17,
                "òdo point mẹta òdo òdo òdo òdo òdo òdo òdo òdo \
                 òdo òdo òdo òdo òdo òdo òdo mẹrin",
            ),
            (
                123456789.12345679,
                8,
                "ọkan ọgọrun ogún mẹta miliọnu mẹrin ọgọrun àádọta mẹfa \
                 ẹgbẹrun meje ọgọrun ọgọrin mẹsan point ọkan méjì mẹta \
                 mẹrin marun mẹfa meje mẹsan",
            ),
        ] {
            assert_eq!(card_f(v, p).unwrap(), want, "value {}", v);
        }
    }

    /// Every `cardinal_dec` corpus row, plus live-interpreter extras.
    #[test]
    fn corpus_decimal_rows() {
        for (s, want) in [
            ("0.01", "òdo point òdo ọkan"),
            ("1.10", "ọkan point ọkan òdo"),
            ("12.345", "mẹwa méjì point mẹta mẹrin marun"),
            ("0.001", "òdo point òdo òdo ọkan"),
            ("-12.345", "minus mẹwa méjì point mẹta mẹrin marun"),
        ] {
            assert_eq!(card_d(s).unwrap(), want, "value {}", s);
        }
        // Trillion-scale integer parts are past the 10^12 ceiling (#147):
        // OverflowError, where Python printed the digits.
        assert!(matches!(
            card_d("98746251323029.99"),
            Err(N2WError::Overflow(_))
        ));
    }

    /// Bug 13: scientific-notation spellings raise ValueError with Python's
    /// exact message — dotless literals fail wholesale, dotted ones fail on
    /// the first non-digit fraction char. Decimal keeps its capital-E,
    /// unpadded spelling.
    #[test]
    fn scientific_notation_value_errors() {
        let msg = |r: Result<String>| match r {
            Err(N2WError::Value(m)) => m,
            other => panic!("expected ValueError, got {:?}", other),
        };
        assert_eq!(
            msg(card_f(1e-05, 5)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        assert_eq!(
            msg(card_f(-1e-05, 5)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        assert_eq!(
            msg(card_f(1.5e-05, 6)),
            "invalid literal for int() with base 10: 'e'"
        );
        assert_eq!(
            msg(card_d("1E-7")),
            "invalid literal for int() with base 10: '1E-7'"
        );
        assert_eq!(
            msg(card_d("1.5E-7")),
            "invalid literal for int() with base 10: 'E'"
        );
        assert_eq!(
            msg(card_d("1.50E-7")),
            "invalid literal for int() with base 10: 'E'"
        );
    }

    /// Bug 14: precision= is a no-op — Python sets converter.precision but
    /// Num2Word_YO.to_cardinal never reads it. Verified live:
    /// num2words(3.14, lang='yo', precision=5) == 'mẹta point ọkan mẹrin'.
    #[test]
    fn precision_override_ignored() {
        let got = LangYo::new()
            .to_cardinal_float(&FloatValue::Float { value: 3.14, precision: 2 }, Some(5))
            .unwrap();
        assert_eq!(got, "mẹta point ọkan mẹrin");
    }

    /// The text reconstructions match CPython spellings on the layout
    /// boundaries (verified against the interpreter).
    #[test]
    fn text_reconstruction_spellings() {
        for (f, want) in [
            (0.0001_f64, "0.0001"),
            (1e-05, "1e-05"),
            (1.5e-05, "1.5e-05"),
            (5e-324, "5e-324"),
            (1.0, "1.0"),
            (0.0, "0.0"),
            (1e15, "1000000000000000.0"),
            (1.5e16, "1.5e+16"),
            (9007199254740992.0, "9007199254740992.0"),
            (1e21, "1e+21"),
            (f64::INFINITY, "inf"),
            (f64::NAN, "nan"),
        ] {
            assert_eq!(float_repr_abs(f), want, "repr of {}", f);
        }
        for (s, want) in [
            ("0.00", "0.00"),
            ("1.10", "1.10"),
            ("0.001", "0.001"),
            ("1E-7", "1E-7"),
            ("1.5E-7", "1.5E-7"),
            ("1.50E-7", "1.50E-7"),
            ("98746251323029.99", "98746251323029.99"),
            ("1E+2", "1E+2"),
        ] {
            let (digits, scale) = BigDecimal::from_str(s)
                .unwrap()
                .abs()
                .as_bigint_and_exponent();
            assert_eq!(decimal_str_abs(&digits, scale), want, "str of {}", s);
        }
    }
}
