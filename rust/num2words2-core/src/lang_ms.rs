//! Port of `lang_MS.py` (Malay).
//!
//! Registry check: `__init__.py` maps `"ms"` → `lang_MS.Num2Word_MS()`, so this
//! is the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_MS` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. `Num2Word_Base.
//! __init__` only builds `self.cards` / sets `self.MAXVAL` when one of those
//! three attributes exists (`if any(hasattr(self, field) for field in ...)`),
//! so for MS **neither is ever created**. `to_cardinal` is overridden outright
//! and drives `_int_to_word` by repeated division. Consequently `cards`/
//! `maxval`/`merge` stay at their trait defaults here, and there is **no
//! overflow check at all** — MS is unbounded, which is why 10^21 converts fine
//! (`"satu bilion trilion"`).
//!
//! Dead code in the Python source, deliberately not ported:
//!   * `Num2Word_MS._setup` calls `super()._setup()`, but `Num2Word_Base` has
//!     no `_setup` — only `setup` (a `pass`), which is what `__init__` calls.
//!     `_setup` is therefore never invoked; had it been, it would raise
//!     `AttributeError`. Unreachable, so nothing to reproduce.
//!
//! Inherited from `Num2Word_Base` but irrelevant here: `title()` is identity
//! because `is_title` stays `False`, and MS's `to_cardinal` never calls it
//! anyway.
//!
//! # Currency
//!
//! `Num2Word_MS` declares `CURRENCY_FORMS` in its **own** class body, so it is
//! *not* the `Num2Word_EUR` dict that `Num2Word_EN.__init__` mutates in place
//! at import time — none of EN's EUR/GBP rewrites or its ~24 extra ISO codes
//! reach MS. Confirmed against the live interpreter: the seven codes in
//! [`build_currency_forms`] are the whole table, which is why JPY/KWD/BHD/INR/
//! CNY/CHF legitimately raise `NotImplementedError`. Note EUR keeps MS's own
//! `("euro", "euro")` rather than EN's `("euro", "euros")`.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` — EN *rebinds*
//! `CURRENCY_PRECISION` on `self` instead of mutating the class dict, so its
//! 3-decimal entries do not leak here either — so the trait defaults (100 and
//! `None`) already match and are not overridden.
//!
//! `to_currency` is overridden wholesale. `to_cheque` is inherited from
//! `Num2Word_Base` unchanged, and the trait default already mirrors it.
//!
//! # More faithfully reproduced Python bugs (currency)
//!
//! 6. ~~**`parse_currency_parts(n)` is called bare**~~, so in Python
//!    `is_int_with_cents` kept its `True` default and an `int` was read as
//!    *minor* units: `to_currency(42)` was "kosong ringgit empat puluh dua
//!    sen" while `42.0` said "empat puluh dua ringgit" — the old upstream
//!    convention savoirfairelinux/num2words#426 removed everywhere else.
//!    Fixed (gladiaio/num2words2#171, as #161 for LIJ): an `int` is a count of
//!    units, so `100` is "seratus euro" and int, float and str agree.
//! 7. **There is no `has_decimal` guard.** The cents segment is gated on
//!    `right > 0` alone, so a whole float prints no subunit: `1.0` is "satu
//!    euro" where `Num2Word_Base` would append a zero-cents segment.
//!    `Decimal("5.00")` likewise renders "lima ringgit". The `has_decimal` flag
//!    the shim computes is therefore ignored here.
//! 8. **The signature is `to_currency(n, currency="MYR")`** — no `cents`, no
//!    `separator`, no `adjective`. All three are ignored; see the note on
//!    [`LangMs::to_currency`].
//! 9. **The fractional-cents branch can only ever say "kosong"** — the actual
//!    fraction is unreachable. See [`FRACTIONAL_CENTS_WORD`].
//! 10. **`pluralize` is never called.** MS reads `cr_major[0]` / `cr_minor[0]`
//!     directly, which is why it never trips Base's abstract `pluralize`. Every
//!     entry's two forms are identical anyway — Malay does not inflect these
//!     nouns for number — but the arity of 2 is kept as the ported data.
//! 11. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** The
//!     divisor is the hardcoded `100` in the `has_fractional_cents` test, and
//!     `parse_currency_parts` is called with no `divisor=`, so currency.py's
//!     `100` default stands. MS therefore has no 3-decimal or 0-decimal
//!     behaviour even if a caller names such a code — moot in practice, since
//!     none of them are in its table.
//! 12. **`except BaseException` swallows decimal's context limit** and hands
//!     back the bare number. See [`LangMs::to_currency`] for the derivation of
//!     the exact 10^26 threshold and for what it means that this beats the
//!     `NotImplementedError`.
//!
//! # The one deliberate gap
//!
//! Bug 12 is reproduced exactly on the int path, where `str(n)` is precisely
//! BigInt's `Display`. On the float/Decimal path `str(n)` is `repr(float)` /
//! Decimal's scientific form, which no longer exists by the time the core sees
//! a `BigDecimal` — so for `|value| >= 10**26` `to_currency` returns
//! `NotImplemented` and lets `num2words()`'s `except NotImplementedError: pass`
//! hand the call back to Python, which produces the real answer. See
//! [`LangMs::to_currency`]. Nothing else in this file gives up.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following look wrong but are exactly what
//! Python emits, and each is pinned by a `bench/corpus.jsonl` row:
//!
//! 1. ~~**`to_ordinal` of a negative silently returns a *positive* ordinal**~~
//!    Fixed (gladiaio/num2words2#155). `_int_to_ordinal` guards with
//!    `if n <= 10: return self.ordinals[n]`, which is true for every negative
//!    `n`, so Python wrapped around (`to_ordinal(-1)` == "kesepuluh",
//!    `to_ordinal(-11)` == "") and raised `IndexError` from `-12` down. The
//!    port raises Base's `errmsg_negord` `TypeError` for every negative, like
//!    most languages, on the int and the truncating float path alike.
//! 2. (merged into 1.)
//! 3. **`to_ordinal_num` does no sign handling**: it is literally
//!    `"ke-" + str(n)`, so `to_ordinal_num(-1)` == `"ke--1"` (double hyphen).
//! 4. **`to_year`'s three branches are identical** — the `n < 1000` /
//!    `n < 2000` / `else` ladder every arm of which returns
//!    `self._int_to_cardinal(n)`. There is no era suffix and no year-pairing
//!    ("nineteen ninety-nine"); 1999 spells out in full as a plain cardinal.
//!    Collapsed to a straight delegation here.
//! 5. `ones[8]` is `"lapan"`, not the fuller `"delapan"`. Preserved verbatim
//!    (corpus pins `"lapan puluh"` for 80).
//!
//! # Error variants
//!
//! Negative ordinals raise `TypeError` (bug 1). `to_cardinal`/`to_ordinal_num`/`to_year`
//! cannot fail for integer input: every table index `_int_to_word` computes is
//! provably in range (see the safety notes on [`int_to_word`]), so the
//! `except BaseException` fallbacks in the Python `to_cardinal` /
//! `to_ordinal_num` are unreachable and are not modelled.
//!
//! # No cross-call mutable state
//!
//! `Num2Word_MS` stashes no flags between methods (no `_pending_ordinal`-style
//! handshake as in `lang_ES`). Every method is a pure function of its argument,
//! so the stateless Rust path is faithful.

use crate::base::{
    negord_error, py_num_str, strictly_negative, verify_ordinal, Lang, N2WError, Result,
};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.ones`. Index 0 is the empty-string filler, exactly as in Python.
const ONES: [&str; 10] = [
    "", "satu", "dua", "tiga", "empat", "lima", "enam", "tujuh", "lapan", "sembilan",
];

/// `self.tens`. Index 0 and 1 are unused by `_int_to_word` (10 is special-cased
/// to the literal "sepuluh"), but are kept so the indices line up with Python.
const TENS: [&str; 10] = [
    "",
    "sepuluh",
    "dua puluh",
    "tiga puluh",
    "empat puluh",
    "lima puluh",
    "enam puluh",
    "tujuh puluh",
    "lapan puluh",
    "sembilan puluh",
];

/// `self.teens`, a dict keyed 11..=19 in Python. Stored densely; index with
/// `n - 11`.
const TEENS: [&str; 9] = [
    "sebelas",
    "dua belas",
    "tiga belas",
    "empat belas",
    "lima belas",
    "enam belas",
    "tujuh belas",
    "lapan belas",
    "sembilan belas",
];

/// `self.ordinals`, 11 entries (indices 0..=10). Index 0 is the empty-string
/// filler that bug 1 above can actually return.
const ORDINALS: [&str; 11] = [
    "",
    "pertama",
    "kedua",
    "ketiga",
    "keempat",
    "kelima",
    "keenam",
    "ketujuh",
    "kelapan",
    "kesembilan",
    "kesepuluh",
];

const ZERO_WORD: &str = "kosong";

/// Python's `self.to_cardinal(right / 100.0)` in the fractional-cents branch —
/// which is *always* `"kosong"`, however many fractional cents there were.
///
/// The constant looks arbitrary, so here is the derivation. `right` is an `int`
/// at that point: MS calls `parse_currency_parts` bare, so `keep_precision`
/// stays `False` and the subunit comes back whole rather than as a `Decimal`
/// (this is what makes MS's branch differ from `Num2Word_Base`'s, which does
/// keep precision and really does render "one point one cents"). Since
/// `right = int(fraction * 100)` with `fraction < 1`, it is bounded to
/// `0..=99`, and the branch is guarded by `right > 0` — so `right / 100.0` is
/// a float strictly between 0 and 1. Feeding that to `Num2Word_MS.to_cardinal`:
///
/// ```text
/// to_cardinal(0.68) -> _int_to_cardinal(0.68) -> _int_to_word(0.68)
///     every `n >= SCALE` test is False, `10 < n < 20` is False,
///     `n >= 10` is False, `n > 0` is True  ->  self.ones[0.68]
///     -> TypeError: list indices must be integers or slices, not float
/// ```
///
/// `to_cardinal`'s own `except BaseException` catches that and retries as
/// `self._int_to_cardinal(int(0.68))` == `_int_to_cardinal(0)` == "kosong".
/// The retry cannot raise a second time, so the fallback always wins and the
/// fraction never reaches the output. Checked against the live interpreter for
/// every `right` in `1..=99`; e.g. `to_currency(2.675, "MYR")` is
/// "dua ringgit kosong sen", not "...enam puluh lapan sen".
const FRACTIONAL_CENTS_WORD: &str = ZERO_WORD;

/// `Num2Word_MS.CURRENCY_FORMS`, verbatim from MS's own class body.
///
/// Each entry carries two identical forms because Malay does not inflect these
/// nouns for number. The arity of 2 mirrors the Python tuples and is kept even
/// though `to_currency` only ever reads index 0 (`cr_major[0]` / `cr_minor[0]`)
/// and the inherited `to_cheque` only ever reads index -1.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const DOLAR: [&str; 2] = ["dolar", "dolar"];
    const SEN: [&str; 2] = ["sen", "sen"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("MYR", CurrencyForms::new(&["ringgit", "ringgit"], &SEN));
    m.insert("SGD", CurrencyForms::new(&DOLAR, &SEN));
    m.insert("USD", CurrencyForms::new(&DOLAR, &SEN));
    // "euro", not EN's "euros": this table is MS's own, so the
    // Num2Word_EN.__init__ mutation of Num2Word_EUR's shared dict misses it.
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &SEN));
    m.insert("GBP", CurrencyForms::new(&["paun", "paun"], &["peni", "peni"]));
    m.insert("IDR", CurrencyForms::new(&["rupiah", "rupiah"], &SEN));
    m.insert("BND", CurrencyForms::new(&DOLAR, &SEN));
    m
}

fn index_error(i: &BigInt) -> N2WError {
    N2WError::Index(format!("list index out of range (ordinals[{}])", i))
}

/// Python's `self.ordinals[n]`. `int_to_ordinal` rejects negatives first
/// (#155), so callers only reach this with `0 <= n <= 10`.
fn ordinal_at(n: &BigInt) -> Result<&'static str> {
    n.to_usize()
        .and_then(|i| ORDINALS.get(i).copied())
        .ok_or_else(|| index_error(n))
}

/// Python's `_int_to_word`. `n` must be non-negative (`_int_to_cardinal` strips
/// the sign before calling).
///
/// # Why no index can go out of range
///
/// Each scale strips its magnitude off `n` before the next test, so by the time
/// a table is indexed the value is provably small:
///   * `ones[hundreds]`: reached only with `n < 1000`, so `hundreds` is 1..=9.
///   * `tens[tens_val]`: reached only with `20 <= n < 100`, so `tens_val` is
///     2..=9.
///   * `teens[n - 11]`: guarded by `10 < n < 20`.
///   * `ones[n % 10]`: a single digit by construction.
/// The trillions arm recurses on `n / 10^12`, shedding 12 digits per level, so
/// recursion depth is ~digits/12 — 10^606 bottoms out in ~50 frames. The
/// billions/millions/thousands arms recurse on values `< 1000`, which terminate
/// immediately.
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO_WORD.to_string();
    }

    let trillion = BigInt::from(1_000_000_000_000u64);
    let billion = BigInt::from(1_000_000_000u64);
    let million = BigInt::from(1_000_000u64);
    let thousand = BigInt::from(1_000u64);
    let hundred = BigInt::from(100u64);

    let mut n = n.clone();
    let mut parts: Vec<String> = Vec::new();

    // Each block mirrors one `if n >= SCALE:` in the Python, including the
    // `== 1` special cases ("satu trilion" but "seribu", not "satu ribu").
    for (scale, one_form, plural_suffix) in [
        (&trillion, "satu trilion", " trilion"),
        (&billion, "satu bilion", " bilion"),
        (&million, "satu juta", " juta"),
        (&thousand, "seribu", " ribu"),
    ] {
        if n >= *scale {
            let (div, rem) = n.div_rem(scale);
            if div.is_one() {
                parts.push(one_form.to_string());
            } else {
                parts.push(format!("{}{}", int_to_word(&div), plural_suffix));
            }
            n = rem;
        }
    }

    // Hundreds. Note Python uses `self.ones[hundreds]` here, NOT a recursive
    // call — which is fine only because `hundreds` is a single digit.
    if n >= hundred {
        let (div, rem) = n.div_rem(&hundred);
        if div.is_one() {
            parts.push("seratus".to_string());
        } else {
            parts.push(format!("{} ratus", ONES[div.to_usize().unwrap_or(0)]));
        }
        n = rem;
    }

    // `n` is now 0..=99, so a u32 view is exact.
    let small = n.to_u32().unwrap_or(0);

    if small > 10 && small < 20 {
        parts.push(TEENS[(small - 11) as usize].to_string());
    } else {
        let mut small = small;
        if small >= 10 {
            if small == 10 {
                parts.push("sepuluh".to_string());
            } else {
                parts.push(TENS[(small / 10) as usize].to_string());
            }
            small %= 10;
        }
        if small > 0 {
            parts.push(ONES[small as usize].to_string());
        }
    }

    parts.join(" ")
}

/// Python's `_int_to_cardinal`.
///
/// `negword` is "negatif " *with* a trailing space and is concatenated raw
/// (`self.negword + self._int_to_word(-n)`) — not trimmed-then-spaced as
/// `Num2Word_Base.to_cardinal` would do. Same result, but MS never routes
/// through the base method.
fn int_to_cardinal(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO_WORD.to_string();
    }
    if n.is_negative() {
        return format!("{}{}", "negatif ", int_to_word(&(-n)));
    }
    int_to_word(n)
}

/// `int(value)` — truncation toward zero — for both `FloatValue` arms.
fn trunc_toward_zero(value: &FloatValue) -> Result<BigInt> {
    match value {
        FloatValue::Float { value, .. } => BigInt::from_f64(value.trunc()).ok_or_else(|| {
            N2WError::Value(format!("cannot convert non-finite float {} to int", value))
        }),
        FloatValue::Decimal { value, .. } => Ok(value.with_scale(0).as_bigint_and_exponent().0),
    }
}

/// The `TypeError` a Python `list` raises when indexed with a float/Decimal —
/// `self.ones[…]` / `self.tens[…]` inside `_int_to_word`.
fn list_index_type_error() -> N2WError {
    N2WError::Type("list indices must be integers or slices, not float".to_string())
}

/// `Num2Word_MS._int_to_word(n)` fed a float/Decimal `n >= 0` — the walk
/// `to_year` performs with **no** `except BaseException` to rescue it.
///
/// `n` is `(whole, frac)`: the truncated magnitude plus a has-fraction flag
/// (the fraction survives every `%=` untouched, so its exact digits never
/// matter — only whether it exists).
///
/// Outcomes, mirroring the interpreter:
///   * scale arms (`trilion`/`bilion`/`juta`/`ribu`): `//` yields a whole
///     count; `== 1` compares numerically, so they never raise and recurse
///     on the whole count.
///   * hundreds: `self.ones[hundreds]` is a **list** index → TypeError for
///     any non-1 count; `hundreds == 1` short-circuits to "seratus".
///   * teens (`10 < n < 20`): a **dict** lookup — a whole value hash-matches
///     its int key ("sebelas"), a fractional one is a KeyError.
///   * `n == 10` → "sepuluh"; any other `n >= 10` → `self.tens[n // 10]`,
///     a list index → TypeError.
///   * a final `n > 0` residue → `self.ones[n]`, a list index → TypeError.
fn ms_word_numeric(whole: &BigInt, frac: bool) -> Result<String> {
    let trillion = BigInt::from(1_000_000_000_000u64);
    let billion = BigInt::from(1_000_000_000u64);
    let million = BigInt::from(1_000_000u64);
    let thousand = BigInt::from(1_000u64);
    let hundred = BigInt::from(100u64);

    let mut w = whole.clone();
    let mut parts: Vec<String> = Vec::new();

    for (scale, one_form, plural_suffix) in [
        (&trillion, "satu trilion", " trilion"),
        (&billion, "satu bilion", " bilion"),
        (&million, "satu juta", " juta"),
        (&thousand, "seribu", " ribu"),
    ] {
        if &w >= scale {
            let (div, rem) = w.div_rem(scale);
            if div.is_one() {
                parts.push(one_form.to_string());
            } else {
                // The count is a whole float/Decimal; recurse without frac.
                parts.push(format!("{}{}", ms_word_numeric(&div, false)?, plural_suffix));
            }
            w = rem;
        }
    }

    if w >= hundred {
        let (div, rem) = w.div_rem(&hundred);
        if div.is_one() {
            parts.push("seratus".to_string());
        } else {
            // self.ones[hundreds] with a float/Decimal index.
            return Err(list_index_type_error());
        }
        w = rem;
    }

    // `w` is now 0..=99 (plus the fraction, if any).
    let small = w.to_u32().unwrap_or(0);
    if (11..=19).contains(&small) || (small == 10 && frac) {
        // `10 < n < 20` → self.teens[n]: whole hash-matches, fractional is
        // a KeyError (uncaught here, unlike to_ordinal's retry).
        if frac {
            return Err(N2WError::Key(format!("{}.…", small)));
        }
        parts.push(TEENS[(small - 11) as usize].to_string());
    } else {
        let mut rest = small;
        if small >= 10 {
            if small == 10 && !frac {
                parts.push("sepuluh".to_string());
            } else {
                // self.tens[n // 10] with a float/Decimal index.
                return Err(list_index_type_error());
            }
            rest %= 10;
        }
        if rest > 0 || frac {
            // self.ones[n] with a float/Decimal index.
            return Err(list_index_type_error());
        }
    }

    Ok(parts.join(" "))
}

/// `Num2Word_MS._int_to_cardinal(n)` fed a float/Decimal — the entry
/// `to_year` uses, with no exception net.
fn ms_cardinal_numeric(value: &FloatValue) -> Result<String> {
    let (neg, frac) = match value {
        FloatValue::Float { value, .. } => (*value < 0.0, value.fract() != 0.0),
        FloatValue::Decimal { value, .. } => (value.is_negative(), !value.is_integer()),
    };
    let whole = trunc_toward_zero(value)?.abs();

    // `if n == 0: return "kosong"` — numeric equality, so -0.0 lands here.
    if whole.is_zero() && !frac {
        return Ok(ZERO_WORD.to_string());
    }
    if neg {
        return Ok(format!("{}{}", "negatif ", ms_word_numeric(&whole, frac)?));
    }
    ms_word_numeric(&whole, frac)
}

/// Python's `_int_to_ordinal`.
fn int_to_ordinal(n: &BigInt) -> Result<String> {
    if n.is_zero() {
        return Ok(ZERO_WORD.to_string());
    }
    // `n <= 10` is true for every negative too, which wrapped around in
    // Python (bug 1, #155): reject negatives like Base's verify_ordinal.
    verify_ordinal(n)?;
    if *n <= BigInt::from(10) {
        return ordinal_at(n).map(|s| s.to_string());
    }
    Ok(format!("ke-{}", int_to_cardinal(n)))
}

pub struct LangMs {
    /// Built once here, never per call — `to_currency` and the inherited
    /// `to_cheque` only ever read it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `10**26`: the value at which `(Decimal(str(n)) * 100) % 1` overruns
    /// decimal's default 28-digit context. See [`LangMs::to_currency`].
    decimal_ctx_limit: BigInt,
    /// The same limit against the already-scaled `value * 100`, i.e. `10**28`
    /// — which is the quantity Python's `% 1` actually chokes on. Kept as a
    /// second field so neither currency path builds a bound per call.
    decimal_ctx_scaled_limit: BigDecimal,
}

impl LangMs {
    pub fn new() -> Self {
        let limit = BigInt::from(10u8).pow(26);
        LangMs {
            currency_forms: build_currency_forms(),
            decimal_ctx_scaled_limit: BigDecimal::from(&limit * 100),
            decimal_ctx_limit: limit,
        }
    }
}

impl Default for LangMs {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangMs {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MYR"
    }

    fn negword(&self) -> &str {
        "negatif "
    }

    fn pointword(&self) -> &str {
        "titik"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(int_to_cardinal(value))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        int_to_ordinal(value)
    }

    /// Python: `return "ke-" + str(n)`. No sign handling — see bug 3.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ke-{}", value))
    }

    /// Python's `to_year` branches on `n < 1000` / `n < 2000` / else and returns
    /// `self._int_to_cardinal(n)` in all three — see bug 4.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(int_to_cardinal(value))
    }

    /// `to_ordinal(float/Decimal)`.
    ///
    /// Python's `_int_to_ordinal(n)` indexes `self.ordinals` (a list) with the
    /// raw float/Decimal — TypeError — or reaches `_int_to_word`, which dies
    /// the same way on any fractional residue; `to_ordinal`'s bare
    /// `except BaseException` then retries as `_int_to_ordinal(int(n))`. The
    /// only non-raising first passes (whole teens like `11.0`, whole scale
    /// counts like `1e6`) produce the identical words the retry would — so the
    /// observable result is `_int_to_ordinal(int(n))`, truncation and all.
    /// Negatives (`-0.5` included) raise `errmsg_negord` (bug 1, #155).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if strictly_negative(value) {
            return Err(negord_error(py_num_str(value)));
        }
        int_to_ordinal(&trunc_toward_zero(value)?)
    }

    /// `to_ordinal_num(float/Decimal)`: `"ke-" + str(n)`, purely lexical —
    /// "ke-5.0", "ke--17", "ke-1E+2".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("ke-{}", repr_str))
    }

    /// `to_year(float/Decimal)` — `self._int_to_cardinal(n)` on all three
    /// branches, with **no** try/except: the `list` indexes inside
    /// `_int_to_word` raise TypeError for most values (`5.0`, `Decimal("5")`,
    /// `-21.0`, …) while dict hits and `== 1` scale counts survive
    /// (`10.0` → "sepuluh", `11.0` → "sebelas", `100.0` → "seratus",
    /// `1e+16` → "sepuluh ribu trilion"). All corpus-pinned.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        ms_cardinal_numeric(value)
    }

    /// The float/Decimal cardinal path — **not** `Num2Word_Base`'s.
    ///
    /// MS overrides `to_cardinal` and *never* delegates to
    /// `Num2Word_Base.to_cardinal_float`, so the inherited float path
    /// (pointword + a word per fractional digit) is the wrong shape for MS. In
    /// Python a non-integer flows straight into
    /// `to_cardinal(n)` -> `_int_to_cardinal(n)` -> `_int_to_word(n)`, which
    /// indexes `self.ones` / `self.tens` (Python `list`s) or `self.teens`
    /// (a `dict`) with the raw float/Decimal. A `list` index that is not an
    /// `int` raises `TypeError`; a fractional value misses the `teens` `dict`
    /// and raises `KeyError`. Either way MS's bare `except BaseException`
    /// retries as `self._int_to_cardinal(int(n))`, and that retry cannot raise
    /// a second time — so the observable result is *always*
    ///
    /// ```text
    /// _int_to_cardinal(int(value))
    /// ```
    ///
    /// i.e. the integer part (truncated toward zero) spelled as a plain
    /// cardinal, with **no** `pointword` and **no** fractional digits. This is
    /// bug-for-bug the Python behaviour, verified against the live interpreter
    /// over 40,000 random float/Decimal inputs (0 mismatches) and pinned by the
    /// corpus: `0.5` -> "kosong", `12.34` -> "dua belas", `-12.34` ->
    /// "negatif dua belas", `Decimal("98746251323029.99")` -> the full 14-digit
    /// integer with the `.99` dropped. Note `int(-0.5) == 0`, so the sign is
    /// silently lost (`-0.5` -> "kosong"), unlike the base path which would
    /// re-prepend the negword.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored:
    /// MS's `to_cardinal(self, n)` takes no such parameter and never reads
    /// `self.precision`, so `num2words(v, lang="ms", precision=5)` is
    /// byte-for-byte `num2words(v, lang="ms")`. Confirmed against the live
    /// interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let int_part = match value {
            // `int(float)` truncates toward zero. `.trunc()` first so the value
            // is already integral, making the BigInt conversion exact at any
            // magnitude — Python's `int(1e21)` is the exact integer of that
            // double, not a saturated `i128`.
            FloatValue::Float { value, .. } => {
                BigInt::from_f64(value.trunc()).ok_or_else(|| {
                    // Unreachable from the corpus: only NaN/±inf return None
                    // here, and `int(float('inf'))` raises in Python too. Kept
                    // as a loud error rather than a panic or a fabricated 0.
                    N2WError::Value(format!(
                        "cannot convert non-finite float {} to int",
                        value
                    ))
                })?
            }
            // `int(Decimal)` truncates toward zero; `with_scale(0)` does exactly
            // that — the same op currency.rs and floatpath.rs use for the
            // Decimal `int()`.
            FloatValue::Decimal { value, .. } => {
                value.with_scale(0).as_bigint_and_exponent().0
            }
        };
        Ok(int_to_cardinal(&int_part))
    }

    // ---- currency -------------------------------------------------------
    //
    // MS overrides `to_currency` wholesale and inherits `to_cheque` from
    // `Num2Word_Base` unchanged, so only the class name, the forms table and
    // `to_currency` itself are language-specific here.
    //
    // Deliberately NOT overridden, because the trait defaults already match:
    //   * `currency_precision` — MS's CURRENCY_PRECISION is `{}`, so
    //     `.get(code, 100)` is 100 for every code, which is the default.
    //   * `currency_adjective` — CURRENCY_ADJECTIVES is `{}` -> None.
    //   * `pluralize` — MS never calls it (bug 10); Base's is abstract and the
    //     default raises NotImplemented, which is faithful if ever reached.
    //   * `money_verbose` — Base's `_money_verbose` is `self.to_cardinal(n)`,
    //     and the default routes to `to_cardinal` -> `int_to_cardinal`. That is
    //     exactly what the inherited `to_cheque` needs.
    //   * `cents_verbose` / `cents_terse` — reachable only from
    //     `default_to_currency`, which MS's own `to_currency` never calls.
    //   * `cardinal_from_decimal` — likewise unreachable: MS's fractional-cents
    //     branch resolves through [`FRACTIONAL_CENTS_WORD`] instead.

    fn lang_name(&self) -> &str {
        "Num2Word_MS"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_MS.to_currency(n, currency="MYR")`.
    ///
    /// `cents`, `separator` and `adjective` do not exist on the Python
    /// signature, so they are ignored (bug 8). `has_decimal` is ignored too:
    /// the cents segment is gated on `right > 0` alone (bug 7).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Python opens the method body with
        //
        //     decimal_val = Decimal(str(n))
        //     has_fractional_cents = (decimal_val * 100) % 1 != 0
        //
        // inside a `try:` whose `except BaseException` hands back
        // `str(n) + " " + currency`. For |n| >= 10**26 that second line
        // *raises*: `decimal_val * 100` is then >= 10**28, and decimal's
        // default context precision is 28, so the `% 1` remainder — whose
        // integer quotient would need 29+ digits — signals InvalidOperation.
        // The handler swallows it and MS returns the bare number:
        //
        //     to_currency(10**26 - 1, "MYR") == "sembilan puluh sembilan ..."
        //     to_currency(10**26,     "MYR") == "100000000000000000000000000 MYR"
        //
        // The threshold is exact for ints: below it `n * 100` needs at most 28
        // digits and is computed exactly, at or above it the product is >=
        // 10**28 whether or not it was rounded to 28 significant digits.
        //
        // This runs *before* the CURRENCY_FORMS lookup because Python's does
        // too — the raise happens on the first lines of the `try`, so the
        // fallback beats the NotImplementedError rather than the other way
        // round:
        //
        //     to_currency(10**30, "JPY") == "1000000000000000000000000000000 JPY"
        //
        // Reproduced exactly here on the int path, where `str(n)` is precisely
        // BigInt's Display, sign included. The float/Decimal path hits the same
        // limit but cannot reconstruct `str(n)`; it is handled a few lines
        // down.
        if let CurrencyValue::Int(v) = val {
            if v.abs() >= self.decimal_ctx_limit {
                return Ok(format!("{} {}", v, currency));
            }
        }

        // `(decimal_val * 100) % 1 != 0`. An int can never have fractional
        // cents — the product is integral — which is also why the arm above is
        // the only place the context limit can bite the int path.
        let has_fractional_cents = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => {
                let scaled = value * BigDecimal::from(100);

                // The float/Decimal side of the same context limit: `% 1`
                // signals once |decimal_val * 100| >= 10**28. Python's
                // `except BaseException` then returns `str(n) + " " + currency`
                // exactly as it does for ints — but `str(n)` here is
                // `repr(float)` ("1e+26") or Decimal's own scientific form
                // ("1E+26"), and *neither is recoverable* from the BigDecimal
                // the shim parsed. Reproducing Python's shortest-round-trip
                // float repr in Rust is the second implementation of it that
                // the porting contract exists to prevent.
                //
                // So the core reports that it cannot serve this input rather
                // than fabricating a different one. num2words()'s currency fast
                // path is `except NotImplementedError: pass`, so the dispatcher
                // falls through to the Python converter and the caller still
                // gets the exact "1e+26 MYR". A direct core caller gets a loud
                // error rather than silently plausible words ("seratus trilion
                // trilion ringgit"), which is what returning a value here would
                // mean. This is the one place MS's Rust surface is deliberately
                // narrower than Python's — see the port report's `concerns`.
                if scaled.abs() >= self.decimal_ctx_scaled_limit {
                    return Err(N2WError::NotImplemented(format!(
                        "Num2Word_MS.to_currency falls back to str(value) for \
                         |value| >= 10**26, which this core cannot reconstruct \
                         (currency {:?})",
                        currency
                    )));
                }

                // with_scale(0) truncates toward zero, matching Decimal's
                // sign-of-dividend `%`; either way this only tests != 0.
                &scaled - scaled.with_scale(0) != BigDecimal::zero()
            }
        };

        // Python calls `parse_currency_parts(n)` bare, so every default in
        // currency.py stands except is_int_with_cents: Python's True read an
        // int as cents (bug 6, #171); it is a count of units here, like 42.0.
        // keep_precision=False (bug 9), divisor=100 (bug 11). Note
        // `keep_precision` is False even when has_fractional_cents is True —
        // MS computes that flag for its own branch below and never forwards
        // it, unlike Num2Word_Base.
        let (left, right, is_negative) = parse_currency_parts(val, false, false, 100);

        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        let mut result: Vec<String> = Vec::new();

        // Python appends `self.negword.strip()`: the trailing space is dropped
        // here and put back by the `" ".join(result)` below. `left` is already
        // the absolute value (parse_currency_parts abs()es it), so
        // `int_to_cardinal` cannot prepend a second negword.
        if is_negative {
            result.push(self.negword().trim().to_string());
        }
        result.push(int_to_cardinal(&left));
        result.push(forms.unit[0].clone());

        // Python: `if right > 0`. This is the *only* cents guard, so a whole
        // float (1.0) or a scaled Decimal ("5.00") prints no subunit (bug 7).
        //
        // Safe: keep_precision=False leaves `cents` at scale 0, so the
        // coefficient is the value.
        let right = right.as_bigint_and_exponent().0;
        if right.is_positive() {
            result.push(if has_fractional_cents {
                FRACTIONAL_CENTS_WORD.to_string()
            } else {
                int_to_cardinal(&right)
            });
            result.push(forms.subunit[0].clone());
        }

        Ok(result.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn n(s: &str) -> BigInt {
        s.parse().unwrap()
    }

    /// Drives the same entry point the corpus harness does. `arg` is Python's
    /// `repr(value)`, so "100" is an int and "12.34" a float — a distinction
    /// `to_currency` branches on and `str()` would erase.
    fn cur(arg: &str, currency: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let val = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangMs::new().to_currency(&val, currency, true, None, false)
    }

    fn cheque(arg: &str, currency: &str) -> Result<String> {
        LangMs::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), currency)
    }

    fn card(s: &str) -> String {
        LangMs::new().to_cardinal(&n(s)).unwrap()
    }

    fn ord(s: &str) -> Result<String> {
        LangMs::new().to_ordinal(&n(s))
    }

    fn year(s: &str) -> String {
        LangMs::new().to_year(&n(s)).unwrap()
    }

    fn ord_num(s: &str) -> String {
        LangMs::new().to_ordinal_num(&n(s)).unwrap()
    }

    #[test]
    fn cardinals_from_corpus() {
        assert_eq!(card("0"), "kosong");
        assert_eq!(card("1"), "satu");
        assert_eq!(card("9"), "sembilan");
        assert_eq!(card("10"), "sepuluh");
        assert_eq!(card("11"), "sebelas");
        assert_eq!(card("12"), "dua belas");
        assert_eq!(card("19"), "sembilan belas");
        assert_eq!(card("20"), "dua puluh");
        assert_eq!(card("21"), "dua puluh satu");
        assert_eq!(card("80"), "lapan puluh");
        assert_eq!(card("99"), "sembilan puluh sembilan");
        assert_eq!(card("100"), "seratus");
        assert_eq!(card("101"), "seratus satu");
        assert_eq!(card("111"), "seratus sebelas");
        assert_eq!(card("200"), "dua ratus");
        assert_eq!(card("999"), "sembilan ratus sembilan puluh sembilan");
        assert_eq!(card("1000"), "seribu");
        assert_eq!(card("1234"), "seribu dua ratus tiga puluh empat");
        assert_eq!(card("2000"), "dua ribu");
        assert_eq!(card("10000"), "sepuluh ribu");
        assert_eq!(card("12345"), "dua belas ribu tiga ratus empat puluh lima");
        assert_eq!(card("100000"), "seratus ribu");
        assert_eq!(
            card("123456"),
            "seratus dua puluh tiga ribu empat ratus lima puluh enam"
        );
        assert_eq!(card("1000000"), "satu juta");
        assert_eq!(card("1000001"), "satu juta satu");
        assert_eq!(
            card("1234567"),
            "satu juta dua ratus tiga puluh empat ribu lima ratus enam puluh tujuh"
        );
        assert_eq!(card("1000000000"), "satu bilion");
        assert_eq!(
            card("123456789"),
            "seratus dua puluh tiga juta empat ratus lima puluh enam ribu tujuh ratus lapan puluh sembilan"
        );
        assert_eq!(card("1000000000000"), "satu trilion");
    }

    /// The trillions arm recurses, so beyond 10^12 the scale name stacks.
    #[test]
    fn unbounded_trillions_stacking() {
        assert_eq!(card("1000000000000000"), "seribu trilion");
        assert_eq!(card("1000000000000000000"), "satu juta trilion");
        assert_eq!(card("1000000000000000000000"), "satu bilion trilion");
    }

    #[test]
    fn negative_cardinals() {
        assert_eq!(card("-1"), "negatif satu");
        assert_eq!(card("-7"), "negatif tujuh");
        assert_eq!(card("-21"), "negatif dua puluh satu");
        assert_eq!(card("-100"), "negatif seratus");
        assert_eq!(card("-1000"), "negatif seribu");
        assert_eq!(card("-1000000"), "negatif satu juta");
    }

    #[test]
    fn ordinals_from_corpus() {
        assert_eq!(ord("0").unwrap(), "kosong");
        assert_eq!(ord("1").unwrap(), "pertama");
        assert_eq!(ord("10").unwrap(), "kesepuluh");
        assert_eq!(ord("11").unwrap(), "ke-sebelas");
        assert_eq!(ord("21").unwrap(), "ke-dua puluh satu");
        assert_eq!(ord("100").unwrap(), "ke-seratus");
        assert_eq!(ord("1000000").unwrap(), "ke-satu juta");
    }

    /// Bug 1 (fixed, #155): negatives used to wrap to a positive ordinal
    /// or raise IndexError; they now raise Base's `errmsg_negord` TypeError.
    #[test]
    fn negative_ordinals_raise_type_error() {
        for v in ["-1", "-7", "-11", "-12", "-21", "-42", "-1000000"] {
            assert!(matches!(ord(v), Err(N2WError::Type(_))), "{v}");
        }
    }

    /// Bug 3: no sign handling, hence the double hyphen.
    #[test]
    fn ordinal_num_is_raw_concat() {
        assert_eq!(ord_num("0"), "ke-0");
        assert_eq!(ord_num("1"), "ke-1");
        assert_eq!(ord_num("1234567890"), "ke-1234567890");
        assert_eq!(ord_num("-1"), "ke--1");
        assert_eq!(ord_num("-1000000"), "ke--1000000");
    }

    /// Bug 4: to_year is a plain cardinal — no pairing, no era suffix.
    #[test]
    fn years_are_plain_cardinals() {
        assert_eq!(year("1"), "satu");
        assert_eq!(year("999"), "sembilan ratus sembilan puluh sembilan");
        assert_eq!(year("1000"), "seribu");
        assert_eq!(year("1492"), "seribu empat ratus sembilan puluh dua");
        assert_eq!(year("1999"), "seribu sembilan ratus sembilan puluh sembilan");
        assert_eq!(year("2024"), "dua ribu dua puluh empat");
        assert_eq!(year("2100"), "dua ribu seratus");
        assert_eq!(year("-44"), "negatif empat puluh empat");
        assert_eq!(year("-500"), "negatif lima ratus");
    }

    // ---- currency -------------------------------------------------------

    /// Frozen-corpus rows — all 36 that MS's table serves; int rows corrected
    /// for #171.
    #[test]
    fn corpus_currency() {
        // Bug 7 is visible here: the float 1.0 prints no cents. Int rows are
        // corrected for bug 6 (#171): an int is units, not cents.
        for (arg, want) in [
            ("0", "kosong euro"),
            ("1", "satu euro"),
            ("2", "dua euro"),
            ("100", "seratus euro"),
            ("12.34", "dua belas euro tiga puluh empat sen"),
            ("0.01", "kosong euro satu sen"),
            ("1.0", "satu euro"),
            ("99.99", "sembilan puluh sembilan euro sembilan puluh sembilan sen"),
            ("1234.56", "seribu dua ratus tiga puluh empat euro lima puluh enam sen"),
            ("-12.34", "negatif dua belas euro tiga puluh empat sen"),
            ("1000000", "satu juta euro"),
            ("0.5", "kosong euro lima puluh sen"),
        ] {
            assert_eq!(cur(arg, "EUR").unwrap(), want, "EUR {}", arg);
        }
        for (arg, want) in [
            ("0", "kosong dolar"),
            ("1", "satu dolar"),
            ("2", "dua dolar"),
            ("100", "seratus dolar"),
            ("12.34", "dua belas dolar tiga puluh empat sen"),
            ("0.01", "kosong dolar satu sen"),
            ("1.0", "satu dolar"),
            ("99.99", "sembilan puluh sembilan dolar sembilan puluh sembilan sen"),
            ("1234.56", "seribu dua ratus tiga puluh empat dolar lima puluh enam sen"),
            ("-12.34", "negatif dua belas dolar tiga puluh empat sen"),
            ("1000000", "satu juta dolar"),
            ("0.5", "kosong dolar lima puluh sen"),
        ] {
            assert_eq!(cur(arg, "USD").unwrap(), want, "USD {}", arg);
        }
        for (arg, want) in [
            ("0", "kosong paun"),
            ("1", "satu paun"),
            ("2", "dua paun"),
            ("100", "seratus paun"),
            ("12.34", "dua belas paun tiga puluh empat peni"),
            ("0.01", "kosong paun satu peni"),
            ("1.0", "satu paun"),
            ("99.99", "sembilan puluh sembilan paun sembilan puluh sembilan peni"),
            ("1234.56", "seribu dua ratus tiga puluh empat paun lima puluh enam peni"),
            ("-12.34", "negatif dua belas paun tiga puluh empat peni"),
            ("1000000", "satu juta paun"),
            ("0.5", "kosong paun lima puluh peni"),
        ] {
            assert_eq!(cur(arg, "GBP").unwrap(), want, "GBP {}", arg);
        }
    }

    /// The three codes the corpus never exercises but MS's table carries.
    #[test]
    fn currency_untested_codes() {
        assert_eq!(cur("12.34", "MYR").unwrap(), "dua belas ringgit tiga puluh empat sen");
        assert_eq!(cur("12.34", "SGD").unwrap(), "dua belas dolar tiga puluh empat sen");
        assert_eq!(cur("12.34", "BND").unwrap(), "dua belas dolar tiga puluh empat sen");
        assert_eq!(cur("12.34", "IDR").unwrap(), "dua belas rupiah tiga puluh empat sen");
    }

    /// MS's table has seven codes and nothing else; the other six the corpus
    /// asks for are NotImplementedError rows.
    #[test]
    fn corpus_currency_not_implemented() {
        for code in ["JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cur("12.34", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_MS\"", code)
                ),
                other => panic!("{}: expected NotImplemented, got {:?}", code, other),
            }
            assert!(matches!(cur("100", code), Err(N2WError::NotImplemented(_))));
        }
    }

    /// Negative ints are units (bug 6 fixed, #171); the sign is said once.
    #[test]
    fn currency_negative_ints() {
        assert_eq!(cur("-1", "MYR").unwrap(), "negatif satu ringgit");
        assert_eq!(cur("-100", "MYR").unwrap(), "negatif seratus ringgit");
        assert_eq!(cur("-101", "MYR").unwrap(), "negatif seratus satu ringgit");
        assert_eq!(cur("-1000000", "MYR").unwrap(), "negatif satu juta ringgit");
    }

    /// Bug 9: the fractional-cents branch always says "kosong".
    #[test]
    fn currency_fractional_cents_are_always_kosong() {
        assert_eq!(cur("1.011", "MYR").unwrap(), "satu ringgit kosong sen");
        assert_eq!(cur("1.005", "MYR").unwrap(), "satu ringgit kosong sen");
        assert_eq!(cur("1.234", "MYR").unwrap(), "satu ringgit kosong sen");
        // ROUND_HALF_UP takes 2.675 to 2.68, so right is 68 — and still prints
        // "kosong" rather than "enam puluh lapan".
        assert_eq!(cur("2.675", "MYR").unwrap(), "dua ringgit kosong sen");
        assert_eq!(cur("0.009", "MYR").unwrap(), "kosong ringgit kosong sen");
        assert_eq!(cur("-1.011", "MYR").unwrap(), "negatif satu ringgit kosong sen");
        // ...but only when `right > 0` survives the quantize: 0.001 rounds to
        // 0.00 and 1.999 to 2.00, so both skip the segment entirely.
        assert_eq!(cur("0.001", "MYR").unwrap(), "kosong ringgit");
        assert_eq!(cur("1.999", "MYR").unwrap(), "dua ringgit");
        assert_eq!(cur("0.999", "MYR").unwrap(), "satu ringgit");
    }

    /// Bug 7: no has_decimal guard. Decimal("5.00") and the float 0.0 both
    /// skip the cents segment where Num2Word_Base would print one.
    #[test]
    fn currency_no_has_decimal_guard() {
        for arg in ["5", "5.00", "5.0"] {
            let val = CurrencyValue::parse(arg, false, true, true).unwrap();
            assert_eq!(
                LangMs::new().to_currency(&val, "MYR", true, None, false).unwrap(),
                "lima ringgit",
                "Decimal({})",
                arg
            );
        }
        assert_eq!(cur("0.0", "MYR").unwrap(), "kosong ringgit");
        assert_eq!(cur("-0.0", "MYR").unwrap(), "kosong ringgit");
    }

    /// Bug 12: at 10**26 decimal's context blows up and `except BaseException`
    /// returns the bare number — and it beats the NotImplementedError, because
    /// Python raises before it ever looks the currency code up.
    #[test]
    fn currency_decimal_context_limit_on_ints() {
        // 10**26 - 1: still inside the context, so still words.
        assert!(cur("99999999999999999999999999", "MYR").unwrap().starts_with("sembilan puluh"));
        assert_eq!(
            cur("100000000000000000000000000", "MYR").unwrap(),
            "100000000000000000000000000 MYR"
        );
        assert_eq!(
            cur("-100000000000000000000000000", "MYR").unwrap(),
            "-100000000000000000000000000 MYR"
        );
        // Beats the NotImplementedError an unknown code would otherwise raise.
        assert_eq!(
            cur("1000000000000000000000000000000", "JPY").unwrap(),
            "1000000000000000000000000000000 JPY"
        );
        // ...but only past the threshold.
        assert!(matches!(
            cur("10000000000000000000000000", "JPY"),
            Err(N2WError::NotImplemented(_))
        ));
    }

    /// The float/Decimal side of bug 12 is the one input the core declines
    /// rather than answers: `str(value)` is gone by the time it gets a
    /// BigDecimal, so it defers to Python through the dispatcher's
    /// `except NotImplementedError: pass` instead of emitting words Python
    /// would never emit. Below the threshold it answers normally.
    #[test]
    fn currency_decimal_context_limit_defers_on_floats() {
        for arg in ["100000000000000000000000000", "1e26", "1e30", "1e300"] {
            let val = CurrencyValue::parse(arg, false, true, true).unwrap();
            assert!(
                matches!(
                    LangMs::new().to_currency(&val, "MYR", true, None, false),
                    Err(N2WError::NotImplemented(_))
                ),
                "float {} should defer to Python, not answer",
                arg
            );
        }
        // Just under the limit the float path answers for itself.
        let val = CurrencyValue::parse("99999999999999999999999999.99", false, true, true).unwrap();
        assert!(LangMs::new()
            .to_currency(&val, "MYR", true, None, false)
            .unwrap()
            .starts_with("sembilan puluh sembilan trilion"));
    }

    /// Bug 8: `cents`, `separator` and `adjective` are not parameters of MS's
    /// Python `to_currency`, so no value of them can change the output.
    #[test]
    fn currency_ignores_base_kwargs() {
        let val = CurrencyValue::parse("12.34", false, true, true).unwrap();
        let want = "dua belas ringgit tiga puluh empat sen";
        let ms = LangMs::new();
        for cents in [true, false] {
            for adjective in [true, false] {
                for separator in [None, Some(" dan"), Some(",")] {
                    assert_eq!(
                        ms.to_currency(&val, "MYR", cents, separator, adjective).unwrap(),
                        want
                    );
                }
            }
        }
    }

    // ---- cheque ---------------------------------------------------------

    /// `to_cheque` is Base's, untouched: `_money_verbose` -> MS's to_cardinal,
    /// the plural (index -1) unit form, upper-cased.
    #[test]
    fn corpus_cheque() {
        for (code, want) in [
            ("EUR", "SERIBU DUA RATUS TIGA PULUH EMPAT AND 56/100 EURO"),
            ("USD", "SERIBU DUA RATUS TIGA PULUH EMPAT AND 56/100 DOLAR"),
            ("GBP", "SERIBU DUA RATUS TIGA PULUH EMPAT AND 56/100 PAUN"),
        ] {
            assert_eq!(cheque("1234.56", code).unwrap(), want, "cheque {}", code);
        }
        for code in ["JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_MS\"", code)
                ),
                other => panic!("{}: expected NotImplemented, got {:?}", code, other),
            }
        }
    }

    #[test]
    fn cheque_extras() {
        assert_eq!(cheque("1.05", "MYR").unwrap(), "SATU AND 05/100 RINGGIT");
        assert_eq!(
            cheque("-1234.56", "MYR").unwrap(),
            "MINUS SERIBU DUA RATUS TIGA PULUH EMPAT AND 56/100 RINGGIT"
        );
        assert_eq!(cheque("0", "MYR").unwrap(), "KOSONG AND 00/100 RINGGIT");
    }
}

// ---- float / Decimal cardinal path --------------------------------------
//
// MS overrides `to_cardinal` and never routes through
// `Num2Word_Base.to_cardinal_float`; a float/Decimal always resolves to
// `_int_to_cardinal(int(value))` (integer part, truncated toward zero, spelled
// as a plain cardinal — no pointword, no fractional digits). See
// `LangMs::to_cardinal_float`.
#[cfg(test)]
mod float_tests {
    use super::*;
    use std::str::FromStr;

    /// Drive the float arm the way the binding does. `precision` is the value
    /// the live interpreter reports (`abs(Decimal(repr(v)).as_tuple().
    /// exponent)`); MS ignores it, but it is carried faithfully anyway.
    fn f(value: f64, precision: u32) -> String {
        LangMs::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// Drive the Decimal arm — exact arbitrary precision, never an f64 cast.
    fn d(s: &str, precision: u32) -> String {
        LangMs::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    /// Every `"to": "cardinal"` corpus row for `ms` whose `arg` has a dot,
    /// verbatim. The fraction is always dropped; only the integer part speaks.
    #[test]
    fn corpus_cardinal_float() {
        let rows: &[(f64, u32, &str)] = &[
            (0.0, 1, "kosong"),
            (0.5, 1, "kosong"),
            (1.0, 1, "satu"),
            (1.5, 1, "satu"),
            (2.25, 2, "dua"),
            (3.14, 2, "tiga"),
            (0.01, 2, "kosong"),
            (0.1, 1, "kosong"),
            (0.99, 2, "kosong"),
            (1.01, 2, "satu"),
            (12.34, 2, "dua belas"),
            (99.99, 2, "sembilan puluh sembilan"),
            (100.5, 1, "seratus"),
            (1234.56, 2, "seribu dua ratus tiga puluh empat"),
            // int(-0.5) == 0, so the sign is lost — no negword (unlike the base
            // float path). int(-1.5) == -1, so that one keeps it.
            (-0.5, 1, "kosong"),
            (-1.5, 1, "negatif satu"),
            (-12.34, 2, "negatif dua belas"),
            // The two f64-artefact cases; irrelevant here since only int() is
            // used, but pinned regardless.
            (1.005, 3, "satu"),
            (2.675, 3, "dua"),
        ];
        for (v, p, want) in rows {
            assert_eq!(f(*v, *p), *want, "float {}", v);
        }
    }

    /// Every `"to": "cardinal_dec"` corpus row for `ms`, verbatim.
    #[test]
    fn corpus_cardinal_dec() {
        assert_eq!(d("0.01", 2), "kosong");
        assert_eq!(d("1.10", 2), "satu");
        assert_eq!(d("12.345", 3), "dua belas");
        // The trillion-scale exact-Decimal case (issue #603): `.99` truncated,
        // full 14-digit integer spelled.
        assert_eq!(
            d("98746251323029.99", 2),
            "sembilan puluh lapan trilion tujuh ratus empat puluh enam bilion \
             dua ratus lima puluh satu juta tiga ratus dua puluh tiga ribu \
             dua puluh sembilan"
        );
        assert_eq!(d("0.001", 3), "kosong");
    }

    /// Extra sign / magnitude coverage beyond the corpus.
    #[test]
    fn extra_float_and_decimal() {
        assert_eq!(f(-0.0, 1), "kosong");
        assert_eq!(d("-0.5", 1), "kosong");
        assert_eq!(d("-12.34", 2), "negatif dua belas");
        assert_eq!(d("5.00", 2), "lima");
        assert_eq!(d("1000.00", 2), "seribu");
        assert_eq!(d("-98746251323029.99", 2).split(' ').next().unwrap(), "negatif");
        // Integer-valued teen Decimals: Python's dict lookup succeeds without
        // the fallback, but the answer is identical either way.
        assert_eq!(d("12.00", 2), "dua belas");
        assert_eq!(d("19.99", 2), "sembilan belas");
    }

    /// The `precision=` kwarg cannot change MS output — its `to_cardinal` never
    /// reads `self.precision`.
    #[test]
    fn precision_override_is_ignored() {
        let ms = LangMs::new();
        for p in [None, Some(0), Some(1), Some(5)] {
            assert_eq!(
                ms.to_cardinal_float(&FloatValue::Float { value: 12.34, precision: 2 }, p)
                    .unwrap(),
                "dua belas"
            );
            assert_eq!(
                ms.to_cardinal_float(
                    &FloatValue::Decimal {
                        value: BigDecimal::from_str("12.345").unwrap(),
                        precision: 3
                    },
                    p
                )
                .unwrap(),
                "dua belas"
            );
        }
    }
}
