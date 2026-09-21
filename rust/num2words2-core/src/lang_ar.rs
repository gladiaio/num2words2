//! Port of `lang_AR.py` (Arabic). Registry key `"ar"` → `Num2Word_AR`.
//!
//! Shape: **self-contained**. `Num2Word_AR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! populates `self.cards` and never uses `splitnum`/`clean`/`merge`. It sets
//! its own `MAXVAL = 10**51` and drives everything through
//! `convert` → `convert_to_arabic` → `process_arabic_group` over 3-digit
//! groups. Accordingly `cards`/`maxval`/`merge` stay at their trait defaults
//! here and the overflow check is open-coded in [`validate_number`].
//!
//! Inherited from `Num2Word_Base` but *overridden* by AR, so no base
//! behaviour survives in our four modes: `to_cardinal`, `to_ordinal`,
//! `to_ordinal_num` and `to_year` are all defined in `lang_AR.py` itself.
//!
//! # The Decimal precision bug (the big one)
//!
//! `convert_to_arabic` walks the number with **`decimal.Decimal`**, not ints:
//!
//! ```python
//! number_to_process = int(temp_number_dec % Decimal(str(1000)))   # may raise
//! temp_number       = int(temp_number_dec / Decimal(1000))        # rounds!
//! ```
//!
//! `Decimal.__truediv__` rounds to the context precision (default **28**
//! significant digits, ROUND_HALF_EVEN), so for numbers with more than 28
//! significant digits the "divide by 1000" step is *wrong*. `Decimal.__mod__`
//! separately raises `InvalidOperation` once the integer quotient exceeds the
//! precision; the module catches that and bumps the **global** context
//! precision to the current operand's digit count, after which everything is
//! exact again.
//!
//! The upshot is a three-digit-wide "danger zone". With a fresh context
//! (prec = 28):
//!
//! * ≤ 28 digits — division exact, `%` never raises. Normal output.
//! * 29–31 digits — `%` does *not* raise (so precision is never bumped) but
//!   the division **does** round. Output is silently wrong. For example
//!   `to_cardinal(10**29 - 1)` returns `"مائة أوكتيليوناً وتسعمائة وتسعة وتسعون"`
//!   ("one hundred octillion nine hundred ninety-nine") instead of the 29-nines
//!   reading — the first division rounds 99999…9 up to exactly `10**26`.
//! * ≥ 32 digits — `%` raises, precision is bumped to the full digit count,
//!   and the result is exact again.
//!
//! Dividing by 1000 only shifts the exponent, so rounding `n / 1000` to `p`
//! significant digits is identical to rounding `n` to `p` significant digits
//! and then dividing exactly. That identity is what lets [`convert`] reproduce
//! the whole thing in exact `BigInt` arithmetic via
//! [`round_half_even_sig`] — validated against the real interpreter on 42k+
//! values spanning the entire danger zone (0 mismatches).
//!
//! **Cross-call global state**: `decimal.getcontext().prec` is process-global
//! and the module never restores it. Converting one ≥ 32-digit number
//! permanently raises the precision, so a *later* `to_cardinal(10**29 - 1)`
//! in the same process returns the exact (different!) string. This port
//! implements the **fresh-process** semantics (prec = 28 at the start of every
//! conversion), which is what the frozen corpus records. See the module report
//! for the dispatcher implications.
//!
//! ## ⚠ The above describes a `lang_AR.py` that no longer exists
//!
//! `lang_AR.py`'s `convert_to_arabic` has now been rewritten **twice** since the
//! frozen corpus was generated, and the three loops in this file are three
//! different snapshots of it. Only the third is current:
//!
//! | loop | precision rule | status |
//! |---|---|---|
//! | [`convert`] (integer modes) | leaks globally; bumped only when `%` raises | **stale** |
//! | [`convert_groups_currency`] (currency) | `localcontext` around the `except` handler, so it reverts each iteration | **stale** |
//! | [`convert_groups_exact`] (float/Decimal) | current source: widened once, up front | correct |
//!
//! The current source has no `except InvalidOperation` handler at all:
//!
//! ```python
//! def convert_to_arabic(self):
//!     with decimal.localcontext() as ctx:
//!         digits = len(Decimal(self.number).as_tuple().digits)
//!         if digits > ctx.prec:
//!             ctx.prec = digits
//!         return self._convert_to_arabic_inner()
//! ```
//!
//! The precision is widened **once, to the whole operand's digit count**, for
//! the whole call — so the division never rounds, **there is no danger zone at
//! all**, and the two "wrong output" narratives above are simply no longer true
//! of Python. `getcontext().prec` is also restored on the way out, so the
//! process-global leak is gone too.
//!
//! **This is a live bug in the two stale loops, not a documentation nit.**
//! Measured against the pure-Python converter (`CONVERTER_CLASSES["ar"]`,
//! bypassing the dispatcher's Rust fast path), [`convert`] diverges on
//! 1/121 of 29-digit, 5/120 of 30-digit and **62/120 of 31-digit** operands;
//! 28-and-below and 32-and-above agree. Concretely,
//! `to_cardinal(1000000000000000019884624838656)` reads the thousands group as
//! 839 here and 838 in Python, and `to_currency(1e30)` does the same via
//! [`convert_groups_currency`]. Both are corpus-invisible: no `ar` corpus row
//! lands in the band with a non-zero tail (the `10**30` edge row is all zeros,
//! which rounds exactly), which is why the integer and currency suites still
//! pass 505/505.
//!
//! They are left untouched here because the integer and currency modes are out
//! of scope for this change — see the port report. Only the float/Decimal path
//! added in this change implements the current rule.
//!
//! # Faithfully reproduced Python bugs
//!
//! 1. **`out.lstrip(minus)` strips a character *set*, not a prefix.**
//!    `to_cardinal` does `bare = out.lstrip(minus)` with `minus = "سالب "`,
//!    so it strips any leading run of `{س, ا, ل, ب, ' '}`. For `-2` the
//!    output `"سالب اثنان"` loses its `ا` and becomes `"ثنان"`. Harmless in
//!    practice — `bare` is only used as a `_AR_STANDALONE_DUAL` key — but it
//!    *does* let negatives reach that table: `to_cardinal(-200)` yields
//!    `"سالب مئتان"` (rewritten), while `to_ordinal(-200)` yields the bare
//!    construct form `"مئتا"`. Modelled by [`py_lstrip_charset`].
//! 2. **`validate_number` only checks the upper bound**, never `abs`. So
//!    `to_cardinal(-(10**51))` sails past the overflow guard and then trips an
//!    `assert` deep inside (see 4).
//! 3. **`to_ordinal` never calls `validate_number`.** `to_ordinal(10**51)`
//!    therefore raises `AssertionError` where `to_cardinal(10**51)` raises the
//!    intended `OverflowError`.
//! 4. **Bare `assert`s guard the group tables**, so out-of-range groups raise
//!    `AssertionError` rather than the commented-out `OverflowError` the
//!    author intended (the dead `raise OverflowError` blocks are still in the
//!    source). See the error-variant note below.
//! 5. **Two dead branches** in `process_arabic_group` test `hundreds == 0`
//!    inside an `if tens > 0` block. `hundreds` is a *true* Decimal division
//!    (`group_number / 100`), so it is `0` only when `group_number == 0` —
//!    which forces `tens == 0` and skips the block entirely. Python's own
//!    comments say "Note: this never happens". Both branches are omitted here
//!    because they are unreachable; the surviving `elif`/`else` ordering is
//!    preserved exactly.
//! 6. **`isCurrencyNameFeminine` leaks into ordinals of 0 and negatives.**
//!    `to_ordinal`'s fallback sets the flag from `number < 100`, which is true
//!    for every negative. So `to_ordinal(-1)` returns the *feminine* `"إحدى"`
//!    and `to_ordinal(-999)` returns `"تسعمائة وتسع وتسعون"` (feminine `تسع`),
//!    while the cardinals use the masculine forms. Reproduced via the
//!    `fem_name` argument threaded through [`convert`].
//! 7. **Ordinals ≥ 1000 and ≤ 0 silently fall back to the cardinal form** and
//!    drop the sign (`to_ordinal(-1000)` == `"ألف"`), because the fallback
//!    converts `self.abs(number)` with no `minus` prefix.
//! 8. **`to_ordinal(2000)` == `"ألفا"` but `to_cardinal(2000)` == `"ألفان"`.**
//!    The construct-form → independent-form rewrite (`_AR_STANDALONE_DUAL`)
//!    lives only in `to_cardinal`; the ordinal fallback calls `convert`
//!    directly and never sees it.
//! 9. **Trailing spaces in two table entries** are shipped verbatim:
//!    `arabicAppendedTwos[9]` is `"أوكتيليونا "` and `arabicTwos[9]`/`[10]`
//!    are `"أوكتيليونان "` / `"نونيليونان "` — all with a trailing space.
//!    Callers `.strip()` the final result, so the space only survives when the
//!    entry is not the last token (`to_cardinal(2e30)` really does double-space:
//!    `"نونيليونان  و…"`).
//! 10. **`self.abs` rounds negative `Decimal`s but not positive ones.**
//!     `abs` is `number if number >= 0 else -number`, and `-number` is a
//!     `decimal` *context operation*: it rounds to 28 significant digits, while
//!     the positive arm hands the operand back untouched. So the two signs are
//!     not mirror images past 28 digits —
//!     `to_cardinal(Decimal("-646888273752320466851667632519.93"))` collapses
//!     to `6.468882737523204668516676325E+29`, whose `to_str` is the integral
//!     `"646888273752320466851667632500"`: the low digits *and* the whole
//!     fractional segment disappear, while its positive twin reads out in full.
//!     Floats are unaffected (`-x` is exact in f64). Only reachable from the
//!     float/Decimal path, so it is modelled in [`to_cardinal_float`].
//!
//! # Float/Decimal ordinals, string NaN, and the grammatical kwargs
//!
//! * **`to_ordinal`/`to_ordinal_num` truncate float input.** Python's
//!   `to_ordinal` begins with `number = int(number)`, so a fractional operand
//!   is truncated toward zero and read as an ordinary ordinal:
//!   `to_ordinal(2.5)` == `"الثاني"`, `to_ordinal(-1.5)` == `int → -1` → the
//!   feminine-leak fallback `"إحدى"` (bug 6), `-0.0` → `0` → `"صفر"`. No
//!   validation and no float grammar on this path. `to_ordinal_num` is
//!   `to_ordinal(value).strip()`, overriding base's echo-the-repr default.
//!   Hooks: `ordinal_float_entry` / `ordinal_num_float_entry`.
//! * **`num2words("NaN", lang="ar")` raises `decimal.InvalidOperation`.**
//!   `str_to_number` itself succeeds (`Decimal("NaN")`); the raise comes from
//!   `validate_number`'s `number >= self.MAXVAL` — an ordering comparison,
//!   which on NaN raises InvalidOperation
//!   (`"[<class 'decimal.InvalidOperation'>]"`). The binding intercepts
//!   `ParsedNumber::NaN` before any per-language hook, so the `str_to_number`
//!   override rewrites the parse result into that error. Trade-off documented
//!   at the override site.
//! * **kwargs**: `to_cardinal(number, case="nominative")` and
//!   `to_ordinal(number, gender="m", prefix="")` are the only signatures with
//!   extra parameters; `to_ordinal_num`/`to_year`/`to_currency` accept none
//!   beyond the standard ones, so their `*_kw` hooks stay at the trait
//!   defaults. An oblique `case` (`_AR_OBLIQUE` membership after `.lower()`)
//!   switches the standalone-dual rewrite to its ين column and runs the
//!   compound `_AR_DUAL_NOMINATIVE_TO_OBLIQUE` `str.replace` cascade; any
//!   other value — unknown strings, `None`, ints — silently means nominative.
//!   `gender` is masculine only for the exact string `"m"`; `prefix` only
//!   surfaces in the >= 1000 / <= 0 cardinal fallback. No kwarg value raises.
//!
//! # Error variants
//!
//! `to_cardinal`/`to_year` raise `OverflowError` → [`N2WError::Overflow`].
//!
//! The `assert` sites raise **`AssertionError`** → [`N2WError::Assertion`],
//! which the binding maps back to a real `PyAssertionError`. (An earlier
//! revision of this note said the variant did not exist and that the error rode
//! on `N2WError::Value` with the name spelled out in the message; `base.rs`
//! grew the dedicated variant since, and [`assertion_error`] uses it.)
//!
//! No corpus row exercises it, but the currency path does reach it and it is
//! verified against the live interpreter: `to_currency(10**51 - 1)` raises
//! AssertionError from `assert int(group_level) < len(self.arabicTwos)` in
//! both implementations.

use crate::base::{Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};

// ---------------------------------------------------------------------------
// Tables — transcribed verbatim from lang_AR.py.
// ---------------------------------------------------------------------------

/// `ARABIC_ONES` (module-level in Python; `to_cardinal` re-assigns
/// `self.arabicOnes = ARABIC_ONES` on every call). Index 0 is the empty string.
const ARABIC_ONES: [&str; 20] = [
    "",
    "واحد",
    "اثنان",
    "ثلاثة",
    "أربعة",
    "خمسة",
    "ستة",
    "سبعة",
    "ثمانية",
    "تسعة",
    "عشرة",
    "أحد عشر",
    "اثنا عشر",
    "ثلاثة عشر",
    "أربعة عشر",
    "خمسة عشر",
    "ستة عشر",
    "سبعة عشر",
    "ثمانية عشر",
    "تسعة عشر",
];

/// `self.arabicFeminineOnes`.
const ARABIC_FEMININE_ONES: [&str; 20] = [
    "",
    "إحدى",
    "اثنتان",
    "ثلاث",
    "أربع",
    "خمس",
    "ست",
    "سبع",
    "ثمان",
    "تسع",
    "عشر",
    "إحدى عشرة",
    "اثنتا عشرة",
    "ثلاث عشرة",
    "أربع عشرة",
    "خمس عشرة",
    "ست عشرة",
    "سبع عشرة",
    "ثماني عشرة",
    "تسع عشرة",
];

/// `self.arabicTens`, indexed by `tens/10 - 2` (so index 0 is twenty).
const ARABIC_TENS: [&str; 8] = [
    "عشرون",
    "ثلاثون",
    "أربعون",
    "خمسون",
    "ستون",
    "سبعون",
    "ثمانون",
    "تسعون",
];

/// `self.arabicHundreds`. Index 0 is the empty string.
const ARABIC_HUNDREDS: [&str; 10] = [
    "",
    "مائة",
    "مئتان",
    "ثلاثمائة",
    "أربعمائة",
    "خمسمائة",
    "ستمائة",
    "سبعمائة",
    "ثمانمائة",
    "تسعمائة",
];

/// `self.arabicAppendedTwos` — the construct ("mudaf") dual forms.
///
/// Note index 9 `"أوكتيليونا "` carries a trailing space in the Python source;
/// preserved verbatim (bug 9).
const ARABIC_APPENDED_TWOS: [&str; 17] = [
    "مئتا",
    "ألفا",
    "مليونا",
    "مليارا",
    "تريليونا",
    "كوادريليونا",
    "كوينتليونا",
    "سكستيليونا",
    "سبتيليونا",
    "أوكتيليونا ",
    "نونيليونا",
    "ديسيليونا",
    "أندسيليونا",
    "دوديسيليونا",
    "تريديسيليونا",
    "كوادريسيليونا",
    "كوينتينيليونا",
];

/// `self.arabicTwos` — the independent dual forms.
///
/// Indices 9 and 10 (`"أوكتيليونان "`, `"نونيليونان "`) carry trailing spaces
/// in the Python source; preserved verbatim (bug 9).
const ARABIC_TWOS: [&str; 17] = [
    "مئتان",
    "ألفان",
    "مليونان",
    "ملياران",
    "تريليونان",
    "كوادريليونان",
    "كوينتليونان",
    "سكستيليونان",
    "سبتيليونان",
    "أوكتيليونان ",
    "نونيليونان ",
    "ديسيليونان",
    "أندسيليونان",
    "دوديسيليونان",
    "تريديسيليونان",
    "كوادريسيليونان",
    "كوينتينيليونان",
];

/// `self.arabicGroup` — singular group names. Index 0 is "hundred".
const ARABIC_GROUP: [&str; 17] = [
    "مائة",
    "ألف",
    "مليون",
    "مليار",
    "تريليون",
    "كوادريليون",
    "كوينتليون",
    "سكستيليون",
    "سبتيليون",
    "أوكتيليون",
    "نونيليون",
    "ديسيليون",
    "أندسيليون",
    "دوديسيليون",
    "تريديسيليون",
    "كوادريسيليون",
    "كوينتينيليون",
];

/// `self.arabicAppendedGroup` — accusative/tanwin group names.
const ARABIC_APPENDED_GROUP: [&str; 17] = [
    "",
    "ألفاً",
    "مليوناً",
    "ملياراً",
    "تريليوناً",
    "كوادريليوناً",
    "كوينتليوناً",
    "سكستيليوناً",
    "سبتيليوناً",
    "أوكتيليوناً",
    "نونيليوناً",
    "ديسيليوناً",
    "أندسيليوناً",
    "دوديسيليوناً",
    "تريديسيليوناً",
    "كوادريسيليوناً",
    "كوينتينيليوناً",
];

/// `self.arabicPluralGroups` — plural group names, used for counts 3..=10.
const ARABIC_PLURAL_GROUPS: [&str; 17] = [
    "",
    "آلاف",
    "ملايين",
    "مليارات",
    "تريليونات",
    "كوادريليونات",
    "كوينتليونات",
    "سكستيليونات",
    "سبتيليونات",
    "أوكتيليونات",
    "نونيليونات",
    "ديسيليونات",
    "أندسيليونات",
    "دوديسيليونات",
    "تريديسيليونات",
    "كوادريسيليونات",
    "كوينتينيليونات",
];

/// `_AR_ORDINALS_DEF`, keys 1..=19 as `(masculine, feminine)`.
/// Index 0 is absent in Python (guarded by the `1 <= number` test).
const AR_ORDINALS_DEF: [(&str, &str); 20] = [
    ("", ""), // absent in Python
    ("الأول", "الأولى"),
    ("الثاني", "الثانية"),
    ("الثالث", "الثالثة"),
    ("الرابع", "الرابعة"),
    ("الخامس", "الخامسة"),
    ("السادس", "السادسة"),
    ("السابع", "السابعة"),
    ("الثامن", "الثامنة"),
    ("التاسع", "التاسعة"),
    ("العاشر", "العاشرة"),
    ("الحادي عشر", "الحادية عشرة"),
    ("الثاني عشر", "الثانية عشرة"),
    ("الثالث عشر", "الثالثة عشرة"),
    ("الرابع عشر", "الرابعة عشرة"),
    ("الخامس عشر", "الخامسة عشرة"),
    ("السادس عشر", "السادسة عشرة"),
    ("السابع عشر", "السابعة عشرة"),
    ("الثامن عشر", "الثامنة عشرة"),
    ("التاسع عشر", "التاسعة عشرة"),
];

/// `_AR_TENS_DEF`, keyed 20..=90 step 10 → indexed here by `tens/10 - 2`.
const AR_TENS_DEF: [&str; 8] = [
    "العشرون",
    "الثلاثون",
    "الأربعون",
    "الخمسون",
    "الستون",
    "السبعون",
    "الثمانون",
    "التسعون",
];

/// `_AR_HUNDREDS_DEF`, keyed 100..=900 step 100 → indexed here by `h/100 - 1`.
const AR_HUNDREDS_DEF: [&str; 9] = [
    "المائة",
    "المئتان",
    "الثلاثمائة",
    "الأربعمائة",
    "الخمسمائة",
    "الستمائة",
    "السبعمائة",
    "الثمانمائة",
    "التسعمائة",
];

/// `_AR_AFTER_HUNDRED`, keyed 100..=900 step 100 → indexed here by `h/100 - 1`.
const AR_AFTER_HUNDRED: [&str; 9] = [
    "بعد المائة",
    "بعد المئتين",
    "بعد الثلاثمائة",
    "بعد الأربعمائة",
    "بعد الخمسمائة",
    "بعد الستمائة",
    "بعد السبعمائة",
    "بعد الثمانمائة",
    "بعد التسعمائة",
];

/// `_AR_STANDALONE_DUAL`: construct form → `(nominative, oblique)`.
///
/// The nominative arm is what the plain `to_cardinal` hook reaches (Python
/// defaults to `case="nominative"`, and `"nominative"` is *not* a member of
/// `_AR_OBLIQUE`, so `is_oblique` is `False`). The oblique arm is reachable
/// through `to_cardinal_kw(case=...)` — see [`is_oblique_case`].
const AR_STANDALONE_DUAL: [(&str, &str, &str); 6] = [
    ("مئتا", "مئتان", "مئتين"),
    ("ألفا", "ألفان", "ألفين"),
    ("مليونا", "مليونان", "مليونين"),
    ("مليارا", "ملياران", "مليارين"),
    ("تريليونا", "تريليونان", "تريليونين"),
    ("كوادريليونا", "كوادريليونان", "كوادريليونين"),
];

/// `_AR_DUAL_NOMINATIVE_TO_OBLIQUE` — the ان → ين dual-ending rewrites the
/// oblique cases apply to *compound* outputs (e.g. 205 → "مئتان وخمسة" →
/// "مئتين وخمسة"). Python iterates a list, longest stems first, so a shorter
/// pair never rewrites a substring of a longer word; order preserved verbatim.
/// Note the list stops at quadrillion — "كوينتليونان" and above are *not*
/// rewritten, exactly as in the source.
const AR_DUAL_NOMINATIVE_TO_OBLIQUE: [(&str, &str); 6] = [
    ("كوادريليونان", "كوادريليونين"),
    ("تريليونان", "تريليونين"),
    ("مليونان", "مليونين"),
    ("ملياران", "مليارين"),
    ("ألفان", "ألفين"),
    ("مئتان", "مئتين"),
];

/// `"صفر"` — the zero word returned by `convert_to_arabic`.
const ZERO_WORD: &str = "صفر";

/// The `minus` prefix built in `to_cardinal`. Note the trailing space.
const MINUS: &str = "سالب ";

/// The default `decimal` context precision Python starts a process with.
const DEFAULT_PREC: usize = 28;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// `Num2Word_AR.MAXVAL` = `10**51`.
fn maxval() -> BigInt {
    BigInt::from(10u8).pow(51)
}

/// Python's `AssertionError` from the bare `assert`s guarding the group
/// tables. See the module docs' "Error variants" note.
fn assertion_error(what: &str) -> N2WError {
    N2WError::Assertion(what.to_string())
}

/// Decimal digit count of a non-negative `BigInt`; `0` has one digit,
/// matching `len(Decimal(x).as_tuple().digits)`.
fn ndigits(x: &BigInt) -> usize {
    if x.is_zero() {
        1
    } else {
        x.to_string().len()
    }
}

/// Is `x` exactly `2 * 10**k` for some `k >= 0`? (Decimal string is a "2"
/// followed by zeros.) See the call site for why this stands in for Python's
/// `self.integer_value == 2 * (10**int(math.log10(self.integer_value)))`.
fn is_two_times_power_of_ten(x: &BigInt) -> bool {
    let s = x.to_string();
    let mut it = s.chars();
    it.next() == Some('2') && it.all(|c| c == '0')
}

/// Round a non-negative `BigInt` to `prec` significant decimal digits using
/// ROUND_HALF_EVEN — the default `decimal` rounding mode.
///
/// This is the crux of the Decimal-precision reproduction. Because
/// `n / 1000` merely shifts the exponent, `round(n/1000, prec)` equals
/// `round(n, prec) / 1000`, and `round(n, prec)` is exactly divisible by
/// 1000's worth of trailing zeros whenever rounding actually bites — so
/// [`convert`] can round here and then divide exactly.
fn round_half_even_sig(x: &BigInt, prec: usize) -> BigInt {
    let d = ndigits(x);
    if d <= prec {
        return x.clone();
    }
    let k = d - prec;
    let p = BigInt::from(10u8).pow(k as u32);
    let q = x / &p;
    let r = x - &q * &p;
    let half = &p / 2u8;
    // ROUND_HALF_EVEN: round up on >half, or on ==half with an odd quotient.
    let bump = r > half || (r == half && (&q % 2u8) != BigInt::from(0u8));
    if bump {
        (q + 1u8) * p
    } else {
        q * p
    }
}

/// Python's `str.lstrip(chars)` — strips a leading run of any character in
/// `chars`, **not** a prefix. `lstrip("")` strips nothing.
///
/// Reproduces bug 1. Indexes by `chars()`, never bytes: every character here
/// is multi-byte Arabic.
fn py_lstrip_charset(s: &str, chars: &str) -> String {
    if chars.is_empty() {
        return s.to_string();
    }
    let set: Vec<char> = chars.chars().collect();
    let mut it = s.char_indices();
    let mut cut = s.len();
    loop {
        match it.next() {
            Some((i, c)) => {
                if !set.contains(&c) {
                    cut = i;
                    break;
                }
            }
            None => break,
        }
    }
    s[cut..].to_string()
}

/// `Num2Word_AR.digit_feminine_status`.
///
/// `isCurrencyPartNameFeminine` is `True` for the whole life of the object and
/// nothing in our four modes ever changes it, so the `group_level == -1` arm
/// is hard-coded to the feminine table (its `else` is marked
/// "Note: this never happens" in Python).
fn digit_feminine_status(digit: usize, group_level: i32, fem_name: bool) -> &'static str {
    if group_level == -1 {
        ARABIC_FEMININE_ONES[digit]
    } else if group_level == 0 {
        if fem_name {
            ARABIC_FEMININE_ONES[digit]
        } else {
            ARABIC_ONES[digit]
        }
    } else {
        ARABIC_ONES[digit]
    }
}

/// `Num2Word_AR.process_arabic_group`.
///
/// `group_number` is 0..=999. `integer_value` is the full absolute value being
/// converted (Python reads `self.integer_value`), needed for the
/// `2 * 10**pow` test.
///
/// Python computes `hundreds = Decimal(group_number) / Decimal(100)` — a
/// *true* division — and then uses `int(hundreds)` (truncation) everywhere
/// except the two dead `hundreds == 0` tests. Since `group_number >= 0`,
/// `int(hundreds)` is plain floor division; see bug 5 for why the dead
/// branches are omitted.
fn process_arabic_group(
    group_number: u32,
    group_level: i32,
    integer_value: &BigInt,
    fem_name: bool,
) -> Result<String> {
    let tens = group_number % 100;
    let hundreds_int = (group_number / 100) as usize;
    let mut ret_val = String::new();

    if hundreds_int > 0 {
        if tens == 0 && hundreds_int == 2 {
            ret_val = ARABIC_APPENDED_TWOS[0].to_string();
        } else {
            ret_val = ARABIC_HUNDREDS[hundreds_int].to_string();
            if !ret_val.is_empty() && tens != 0 {
                ret_val.push_str(" و");
            }
        }
    }

    if tens > 0 {
        if tens < 20 {
            // Python: `assert int(group_level) < len(self.arabicTwos)`.
            if group_level >= ARABIC_TWOS.len() as i32 {
                return Err(assertion_error("int(group_level) < len(self.arabicTwos)"));
            }
            if tens == 2 && hundreds_int == 0 && group_level > 0 {
                // Python:
                //   pow = int(math.log10(self.integer_value))
                //   if (self.integer_value > 10 and pow % 3 == 0
                //           and self.integer_value == 2 * (10**pow)):
                //
                // `math.log10` routes a big int through a C double and is *not*
                // always `ndigits - 1`: log10(10**15 - 1) rounds up to exactly
                // 15.0, so `pow` overshoots for all-nines operands.
                //
                // That imprecision is provably inert here. The equality
                // `integer_value == 2 * 10**pow` can only hold when
                // `integer_value` is itself `2 * 10**k` (a "2" followed by
                // zeros), and for exactly those operands `int(math.log10(...))`
                // is exact for every k in 0..=51 (MAXVAL caps k at 51) — the
                // overshoot only ever hits values near 10**k from below, which
                // are never of that form. So testing the *shape* of
                // `integer_value` is equivalent to Python's float round-trip,
                // and avoids depending on f64 rounding. Verified equivalent on
                // 300k+ values, including every 10**k / 2*10**k boundary.
                if is_two_times_power_of_ten(integer_value)
                    && *integer_value > BigInt::from(10u8)
                    && (ndigits(integer_value) - 1) % 3 == 0
                {
                    ret_val = ARABIC_APPENDED_TWOS[group_level as usize].to_string();
                } else {
                    ret_val = ARABIC_TWOS[group_level as usize].to_string();
                }
            } else {
                // The two `hundreds == 0` branches Python tests first are dead
                // (bug 5); this preserves the surviving elif/else ordering.
                if tens == 1 && group_level > 0 {
                    ret_val.push_str(ARABIC_GROUP[group_level as usize]);
                } else {
                    ret_val.push_str(digit_feminine_status(tens as usize, group_level, fem_name));
                }
            }
        } else {
            // Python: `ones = tens % 10; tens = (tens / 10) - 2` — a true
            // Decimal division, then `int(tens)` truncates (e.g. 99 → 9.9 - 2
            // = 7.9 → 7). For non-negative `tens` that is floor division.
            let ones = tens % 10;
            let t = (tens / 10 - 2) as usize;
            if ones > 0 {
                ret_val.push_str(digit_feminine_status(ones as usize, group_level, fem_name));
            }
            if !ret_val.is_empty() && ones != 0 {
                ret_val.push_str(" و");
            }
            ret_val.push_str(ARABIC_TENS[t]);
        }
    }

    Ok(ret_val)
}

/// `Num2Word_AR.convert` → `convert_to_arabic`, for a non-negative integer.
///
/// Returns Python's `formatted_number` *unstripped*. In our four modes
/// `arabicPrefixText`, `arabicSuffixText`, `currency_unit` and
/// `currency_subunit` are all empty and `_decimalValue` is always 0 (integer
/// input), so every currency/decimal branch of `convert_to_arabic` appends the
/// empty string and `formatted_number` reduces to `ret_val`.
///
/// `fem_name` is Python's `self.isCurrencyNameFeminine` (bug 6).
///
/// `prec` starts at [`DEFAULT_PREC`] on every call — the fresh-process
/// semantics; see the module docs on the global-precision leak.
fn convert(n: &BigInt, fem_name: bool) -> Result<String> {
    debug_assert!(!n.is_negative());
    let integer_value = n.clone();

    // `convert_to_arabic`: `if temp_number == Decimal(0): return "صفر"`.
    if n.is_zero() {
        return Ok(ZERO_WORD.to_string());
    }

    let thousand = BigInt::from(1000u16);
    let mut ret_val = String::new();
    let mut group: i32 = 0;
    let mut prec = DEFAULT_PREC;
    let mut temp = n.clone();

    while temp > BigInt::zero() {
        // `int(temp_number_dec % Decimal(str(1000)))` raises InvalidOperation
        // when the integer quotient has more digits than the context
        // precision; the handler bumps the *global* precision to this
        // operand's digit count and retries (which then always succeeds).
        let q_int = &temp / &thousand;
        if ndigits(&q_int) > prec {
            prec = ndigits(&temp);
        }
        let number_to_process = (&temp % &thousand).to_u32().expect("0..=999");

        // `temp_number = int(temp_number_dec / Decimal(1000))` — rounds to
        // `prec` significant digits, then truncates toward zero.
        temp = round_half_even_sig(&temp, prec) / &thousand;

        // Python passes `Decimal(floor(temp_number))` as `remaining_number`,
        // which is only read by a dead branch (bug 5), so it is not threaded
        // through here.
        let group_description =
            process_arabic_group(number_to_process, group, &integer_value, fem_name)?;

        if !group_description.is_empty() {
            if group > 0 {
                if !ret_val.is_empty() {
                    ret_val = format!("و{}", ret_val);
                }
                if number_to_process != 2 && number_to_process != 1 {
                    // Python: `assert group < len(self.arabicGroup)`.
                    if group >= ARABIC_GROUP.len() as i32 {
                        return Err(assertion_error("group < len(self.arabicGroup)"));
                    }
                    if number_to_process % 100 != 1 {
                        if (3..=10).contains(&number_to_process) {
                            ret_val =
                                format!("{} {}", ARABIC_PLURAL_GROUPS[group as usize], ret_val);
                        } else if !ret_val.is_empty() {
                            ret_val =
                                format!("{} {}", ARABIC_APPENDED_GROUP[group as usize], ret_val);
                        } else {
                            ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                        }
                    } else {
                        ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                    }
                }
            }
            ret_val = format!("{} {}", group_description, ret_val);
        }
        group += 1;
    }

    Ok(ret_val)
}

/// `Num2Word_AR.validate_number`.
///
/// Note it compares `number >= MAXVAL` on the *signed* value — it never takes
/// an absolute value, so no negative input is ever rejected (bug 2).
fn validate_number(number: &BigInt) -> Result<()> {
    let mx = maxval();
    if *number >= mx {
        return Err(N2WError::Overflow(format!(
            "abs({}) must be less than {}.",
            number, mx
        )));
    }
    Ok(())
}

/// `Num2Word_AR.to_cardinal(number, case=...)` for an integer operand.
/// `is_oblique` is Python's `isinstance(case, str) and case.lower() in
/// _AR_OBLIQUE` (see [`is_oblique_case`]); the plain hook passes `false`.
fn cardinal_int_case(value: &BigInt, is_oblique: bool) -> Result<String> {
    // Python sets `self.isCurrencyNameFeminine = False` first, then
    // validates — the flag is masculine for every cardinal.
    validate_number(value)?;

    let minus = if value.is_negative() { MINUS } else { "" };
    let abs = if value.is_negative() {
        -value
    } else {
        value.clone()
    };
    let out = format!("{}{}", minus, convert(&abs, false)?.trim());
    Ok(finish_cardinal_case(out, minus, is_oblique))
}

/// `Num2Word_AR.to_ordinal(number, gender="m", prefix="")`.
///
/// `feminine` is Python's `gender_idx` — `False` selects column 0 (the `"m"`
/// default), `True` column 1. Python computes it as `0 if gender == "m" else
/// 1`, so *any* non-`"m"` value (including `None`, ints, `"M"`) is feminine;
/// [`LangAr::to_ordinal_kw`] preserves that.
///
/// `prefix` is Python's `prefix=` kwarg **already `str.format`-ted** (see
/// [`kwval_display`]). It only matters in the fallback branch, which sets
/// `self.arabicPrefixText = prefix` before `convert`; `convert_to_arabic`
/// prepends `"{} ".format(prefix)` when `prefix != ""` — but its `== 0` early
/// return fires *before* that, so `to_ordinal(0, prefix="xx")` is a bare
/// `"صفر"`. The 1..=999 table branches never see the prefix at all, and the
/// 100..=999 recursion passes `gender` but leaves `prefix` at its `""`
/// default, exactly as `self.to_ordinal(remainder, gender=gender)` does.
/// `Num2Word_Base.verify_ordinal`, which `Num2Word_AR.to_ordinal` did not
/// call — ported from savoirfairelinux/num2words#672.
///
/// Without it a negative index runs straight into `arabicOrdinal[number]`,
/// which in Python indexes the list *backwards*: `to_ordinal(-1)` returned
/// "إحدى" (the feminine "one") and `to_ordinal(-100)` returned "مائة"
/// ("hundred"). Both are silently wrong rather than an error, which is worse
/// than the crash upstream saw, so this is a behaviour fix rather than a
/// crash fix here.
///
/// This is the integer arm; [`verify_ordinal_float`] adds Base's
/// non-integer check ahead of it.
fn verify_ordinal(number: &BigInt) -> Result<()> {
    if number.is_negative() {
        // Base's `errmsg_negord`, verbatim — the wording every other
        // language in this crate already raises.
        return Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            number
        )));
    }
    Ok(())
}

/// `verify_ordinal` for a float/Decimal operand.
///
/// Base checks `int(value) == value` first, so a fractional operand raises
/// before the sign is looked at. This replaces AR's previous silent
/// truncation: `to_ordinal(2.5)` used to return "الثاني", and now raises,
/// which is what upstream does once `verify_ordinal` is called.
fn verify_ordinal_float(value: &FloatValue) -> Result<BigInt> {
    // Base writes `if not int(value) == value`, and it is the `int()` call
    // itself that raises for inf/nan — OverflowError and ValueError
    // respectively — *before* any comparison happens. Running the same
    // conversion first keeps those two errors as they were; only finite
    // fractional values reach the TypeError below.
    float_int_trunc(value)?;
    match value.as_whole_int() {
        Some(i) => {
            verify_ordinal(&i)?;
            Ok(i)
        }
        None => Err(N2WError::Type(format!(
            "Cannot treat float {} as ordinal.",
            ar_py_num_str(value)
        ))),
    }
}

/// `"%s" % value` for the `errmsg_floatord` message.
///
/// Only *fractional* values reach it — a whole float or Decimal is accepted
/// by `as_whole_int` above and never formatted here — so Rust's `Display`
/// and Python's `str()` agree on every input that can get this far
/// (`2.5` -> "2.5", `Decimal("2.50")` -> "2.50").
fn ar_py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => value.to_string(),
    }
}

fn to_ordinal_impl(number: &BigInt, feminine: bool, prefix: &str) -> Result<String> {
    let pick = |pair: &(&'static str, &'static str)| if feminine { pair.1 } else { pair.0 };
    let one = BigInt::from(1u8);
    let nineteen = BigInt::from(19u8);
    let twenty = BigInt::from(20u8);
    let ninety_nine = BigInt::from(99u8);
    let hundred = BigInt::from(100u8);
    let nine_ninety_nine = BigInt::from(999u16);

    if *number >= one && *number <= nineteen {
        let i = number.to_usize().expect("1..=19");
        return Ok(pick(&AR_ORDINALS_DEF[i]).to_string());
    }
    if *number >= twenty && *number <= ninety_nine {
        let n = number.to_u32().expect("20..=99");
        let tens = (n / 10) * 10;
        let ones = n % 10;
        if ones == 0 {
            return Ok(AR_TENS_DEF[(tens / 10 - 2) as usize].to_string());
        }
        let ones_form = pick(&AR_ORDINALS_DEF[ones as usize]);
        return Ok(format!("{} و{}", ones_form, AR_TENS_DEF[(tens / 10 - 2) as usize]));
    }
    if *number >= hundred && *number <= nine_ninety_nine {
        let n = number.to_u32().expect("100..=999");
        let hundreds = (n / 100) * 100;
        let remainder = n % 100;
        if remainder == 0 {
            return Ok(AR_HUNDREDS_DEF[(hundreds / 100 - 1) as usize].to_string());
        }
        // Python recurses: `inner = self.to_ordinal(remainder, gender=gender)`
        // — gender rides along, prefix stays at its "" default.
        let inner = to_ordinal_impl(&BigInt::from(remainder), feminine, "")?;
        return Ok(format!(
            "{} {}",
            inner,
            AR_AFTER_HUNDRED[(hundreds / 100 - 1) as usize]
        ));
    }

    // Fallback: >= 1000, or 0/negative. Note `to_ordinal` never calls
    // `validate_number` (bug 3) and drops the sign (bug 7). `gender` is
    // ignored here — the feminine leak comes from `isCurrencyNameFeminine`
    // (bug 6), not from the kwarg.
    let fem_name = *number < hundred; // true for 0 and every negative (bug 6)
    let abs = if number.is_negative() {
        -number
    } else {
        number.clone()
    };
    let conv = convert(&abs, fem_name)?;
    // `convert_to_arabic` prepends `"{} ".format(arabicPrefixText)` when it is
    // non-empty — but returns "صفر" for zero *before* reaching that append,
    // and `convert` already reproduces the early return, so only guard the
    // non-zero shape here.
    if !prefix.is_empty() && !abs.is_zero() {
        return Ok(format!("{} {}", prefix, conv).trim().to_string());
    }
    Ok(conv.trim().to_string())
}

/// Python's `int(number)` on a float/Decimal operand — the first line of
/// `to_ordinal` (`number = int(number)`), truncating toward zero.
///
/// `int(float('inf'))` raises **OverflowError** and `int(float('nan'))`
/// raises **ValueError**, same as the [`to_str_float_checked`] guard.
/// Decimal Inf/NaN never reach here: the binding intercepts
/// `ParsedNumber::Inf`/`NaN` before any per-language hook (and for AR the
/// NaN case is re-routed in [`LangAr::str_to_number`]).
fn float_int_trunc(value: &FloatValue) -> Result<BigInt> {
    match value {
        FloatValue::Float { value: x, .. } => {
            if x.is_nan() {
                return Err(N2WError::Value("cannot convert float NaN to integer".into()));
            }
            if x.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
            Ok(BigInt::from_f64(x.trunc()).expect("finite f64"))
        }
        // `int(Decimal)` truncates toward zero; `with_scale(0)` drops the
        // fractional digits the same way (BigInt division truncates).
        FloatValue::Decimal { value: d, .. } => Ok(d.with_scale(0).as_bigint_and_exponent().0),
    }
}

/// `is_oblique = isinstance(case, str) and case.lower() in self._AR_OBLIQUE`.
///
/// Any non-string value — `None`, a bool, an int, a list — fails the
/// `isinstance` test and silently means nominative; nothing ever raises on a
/// bad `case`. The membership set `_AR_OBLIQUE` is transcribed inline; its
/// nominative sibling `_AR_NOMINATIVE` is dead code in Python (nothing tests
/// membership in it) and is omitted here.
fn is_oblique_case(case: Option<&KwVal>) -> bool {
    match case {
        Some(KwVal::Str(s)) => {
            let l = s.to_lowercase();
            matches!(
                l.as_str(),
                "a" | "accusative"
                    | "نصب"
                    | "منصوب"
                    | "g"
                    | "genitive"
                    | "جر"
                    | "مجرور"
                    | "o"
                    | "oblique"
            )
        }
        _ => false,
    }
}

/// The tail of `Num2Word_AR.to_cardinal`, shared by the int and float paths:
/// the `_AR_STANDALONE_DUAL` rewrite (bug 1 lets negatives reach it via the
/// charset `lstrip`) and, for the oblique cases, the compound ان → ين
/// `str.replace` cascade.
fn finish_cardinal_case(out: String, minus: &str, is_oblique: bool) -> String {
    // `bare = out.lstrip(minus).strip()` — a character-set strip (bug 1).
    let bare = py_lstrip_charset(&out, minus).trim().to_string();
    for (construct, nominative, oblique) in AR_STANDALONE_DUAL.iter() {
        if bare == *construct {
            // `out = (minus + (obl if is_oblique else nom)).strip()`
            let form = if is_oblique { oblique } else { nominative };
            return format!("{}{}", minus, form).trim().to_string();
        }
    }
    if is_oblique {
        // `for nom, obl in _AR_DUAL_NOMINATIVE_TO_OBLIQUE: out = out.replace(...)`
        // — replaces every occurrence, list order (longest stems first).
        let mut out = out;
        for (nom, obl) in AR_DUAL_NOMINATIVE_TO_OBLIQUE.iter() {
            out = out.replace(nom, obl);
        }
        return out;
    }
    out
}

/// `"{}".format(value)` for a kwarg that Python only ever interpolates into a
/// string — the `prefix=` parameter. A non-str prefix does not raise: it is
/// formatted (`str(None)` == `"None"`, `str(True)` == `"True"`) and, being
/// `!= ""`, always takes the prefixed branch. Lists render in Python's repr
/// shape, which is what `format` produces for them.
fn kwval_display(v: &KwVal) -> String {
    match v {
        KwVal::Str(s) => s.clone(),
        KwVal::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        KwVal::Int(i) => i.to_string(),
        KwVal::None => "None".to_string(),
        KwVal::List(l) => {
            let items: Vec<String> = l.iter().map(|s| format!("'{}'", s)).collect();
            format!("[{}]", items.join(", "))
        }
    }
}

// ---------------------------------------------------------------------------
// Currency
// ---------------------------------------------------------------------------
//
// `Num2Word_AR.CURRENCY_FORMS` is `Num2Word_Base`'s empty dict — AR subclasses
// Base directly, so none of `lang_EUR.py`'s table (nor English's in-place
// mutation of it) reaches here. Verified against the live interpreter:
// `CONVERTER_CLASSES["ar"].CURRENCY_FORMS == {}`.
//
// `to_currency` ignores it entirely and selects one of the module-level
// `CURRENCY_*` pairs via `set_currency_prefer`. `to_cheque` does *not*, so it
// hits the empty dict and raises NotImplementedError for every code — see the
// `lang_name` note below.

/// `self.currency_unit` / `self.currency_subunit` / `self.partPrecision`, i.e.
/// everything `set_currency_prefer` assigns.
struct CurrencyPrefs {
    /// 4 forms: [singular, dual, plural(3..=10), accusative(11..=99)].
    unit: &'static [&'static str; 4],
    subunit: &'static [&'static str; 4],
    part_precision: usize,
}

// The module-level `CURRENCY_*` tables, transcribed verbatim. Each is
// `[unit_forms, subunit_forms]`; the arity (4) is load-bearing because
// `convert_to_arabic` indexes 0..=3 by the `% 100` remainder.
const CURRENCY_SR: ([&str; 4], [&str; 4]) = (
    ["ريال", "ريالان", "ريالات", "ريالاً"],
    ["هللة", "هللتان", "هللات", "هللة"],
);
const CURRENCY_EGP: ([&str; 4], [&str; 4]) = (
    ["جنيه", "جنيهان", "جنيهات", "جنيهاً"],
    ["قرش", "قرشان", "قروش", "قرش"],
);
const CURRENCY_KWD: ([&str; 4], [&str; 4]) = (
    ["دينار", "ديناران", "دينارات", "ديناراً"],
    ["فلس", "فلسان", "فلس", "فلس"],
);
const CURRENCY_LBP: ([&str; 4], [&str; 4]) = (
    ["ليرة", "ليرتان", "ليرات", "ليرة"],
    ["قرش", "قرشان", "قروش", "قرش"],
);
const CURRENCY_YER: ([&str; 4], [&str; 4]) = (
    ["ريال", "ريالان", "ريالات", "ريالاً"],
    ["فلس", "فلسان", "فلس", "فلس"],
);
const CURRENCY_USD: ([&str; 4], [&str; 4]) = (
    ["دولار", "دولارين", "دولارات", "دولاراً"],
    ["سنت", "سنتان", "سنتا", "سنتاٌ"],
);
const CURRENCY_TND: ([&str; 4], [&str; 4]) = (
    ["دينار", "ديناران", "دينارات", "ديناراً"],
    ["مليماً", "ميلمان", "مليمات", "مليم"],
);

/// `Num2Word_AR.set_currency_prefer`.
///
/// An `if/elif` chain over seven codes with an unconditional `else` — there is
/// no lookup table and no KeyError, so **every** unknown code (EUR, USD's
/// neighbours, "XXX", …) silently falls back to Saudi riyals rather than
/// raising NotImplementedError. That is why the corpus records
/// `currency:EUR "2"` as `"اثنان ريالان"` (two *riyals*) and why JPY/BHD/KWD
/// are not treated as 0- or 3-decimal currencies here: `CURRENCY_PRECISION`
/// never enters AR's currency path at all.
///
/// Note KWD sets `partPrecision = 2`, not 3, even though the dinar is a
/// 3-decimal currency and `CURRENCY_TND` (also dinars) gets 3. Preserved
/// verbatim: it is Python's behaviour.
///
/// Returns borrowed `'static` data, so the "build tables once, never per call"
/// rule is satisfied by construction — this is a match over consts, not a
/// table build.
fn set_currency_prefer(currency: &str) -> CurrencyPrefs {
    let (unit, subunit, part_precision) = match currency {
        "TND" => (&CURRENCY_TND.0, &CURRENCY_TND.1, 3),
        "EGP" => (&CURRENCY_EGP.0, &CURRENCY_EGP.1, 2),
        "KWD" => (&CURRENCY_KWD.0, &CURRENCY_KWD.1, 2),
        "LBP" => (&CURRENCY_LBP.0, &CURRENCY_LBP.1, 2),
        "YER" => (&CURRENCY_YER.0, &CURRENCY_YER.1, 2),
        "USD" => (&CURRENCY_USD.0, &CURRENCY_USD.1, 2),
        _ => (&CURRENCY_SR.0, &CURRENCY_SR.1, 2),
    };
    CurrencyPrefs {
        unit,
        subunit,
        part_precision,
    }
}

/// `Num2Word_AR.to_str` for a Python `int`.
///
/// `integer = int(number); if integer == number: return str(integer)` — for an
/// `int` operand that test is always true, so `to_str` is just `str(n)` and no
/// float arithmetic is involved.
fn to_str_int(n: &BigInt) -> String {
    n.to_string()
}

/// `Num2Word_AR.to_str` for a Python `float`:
///
/// ```python
/// integer = int(number)
/// if integer == number:
///     return str(integer)
/// decimal = round((number - integer) * 10**9)
/// return str(integer) + "." + "{:09d}".format(decimal).rstrip("0")
/// ```
///
/// Deliberately done in `f64`, not on the `BigDecimal`. `CurrencyValue::Decimal`
/// is parsed from `str(value)` — the float's shortest round-tripping repr — so
/// re-parsing it recovers the *identical* f64 and this reproduces Python's
/// arithmetic bit for bit. Using the exact decimal instead would diverge:
/// `to_str(1e40)` must yield the float's true integer value
/// `10000000000000000303786028427003666890752`, not `10**40`.
///
/// Two quirks ride along and are relied on by the oracle:
///
/// * `"{:09d}"` is a *minimum* width, so a `decimal` that carries to
///   1_000_000_000 prints ten digits and `.rstrip("0")` leaves `"1"` —
///   `to_str(2.9999999999)` is `"2.1"`, not `"3"`.
/// * `.rstrip("0")` can empty the fraction entirely, yielding a trailing dot:
///   `to_str(1.0000000001)` is `"1."` and `to_str(1e-10)` is `"0."`
///   (which `Decimal("0.")` then reads back as zero → "صفر").
fn to_str_float(x: f64) -> String {
    let integer = x.trunc();
    // `int(float)` is exact in Python; `BigInt::from_f64` after `trunc` matches.
    let int_big = BigInt::from_f64(integer).unwrap_or_else(BigInt::zero);
    if integer == x {
        return int_big.to_string();
    }
    // `number - integer` is exact (both are f64 and the difference needs no
    // more bits than `number` already had); `* 10**9` is a plain f64 multiply
    // because Python widens the int operand to float.
    let decimal = py_round_half_even_f64((x - integer) * 1e9);
    let padded = format!("{:0>9}", decimal);
    format!("{}.{}", int_big, padded.trim_end_matches('0'))
}

/// Python's 1-argument `round()` on a non-negative float: nearest, ties to
/// even. `f64::round` is ties-away-from-zero, so it cannot be used.
fn py_round_half_even_f64(v: f64) -> i64 {
    let fl = v.floor();
    let base = fl as i64;
    let frac = v - fl;
    if frac > 0.5 {
        base + 1
    } else if frac < 0.5 {
        base
    } else if base % 2 == 0 {
        base
    } else {
        base + 1
    }
}

/// `Num2Word_AR.decimal_value`: right-pad the fraction with zeros to
/// `partPrecision`, then truncate to `partPrecision` digits.
///
/// Python's guard is `if self.partPrecision is not len(decimal_part)` — an
/// identity test on two small ints, which CPython interns, so it behaves as
/// `!=`. `len()` here is at most 10 (`to_str` caps the fraction), far inside
/// the interned range, so the `is`/`==` distinction can never be observed.
fn decimal_value(decimal_part: &str, part_precision: usize) -> String {
    let n = decimal_part.chars().count();
    if part_precision != n {
        let mut b = decimal_part.to_string();
        // `for i in range(0, self.partPrecision - decimal_part_length)` — an
        // empty range when the fraction is already longer than the precision.
        for _ in 0..part_precision.saturating_sub(n) {
            b.push('0');
        }
        let len = b.chars().count();
        let dec = if len <= part_precision {
            len
        } else {
            part_precision
        };
        b.chars().take(dec).collect()
    } else {
        decimal_part.to_string()
    }
}

/// `Decimal(self.number) == Decimal(0)`, the "صفر" test in `convert_to_arabic`.
///
/// Must be asked of the *whole* `to_str` string rather than of
/// `integer_value`: `to_str(0.001)` is `"0.001"`, whose integer part is 0 yet
/// whose Decimal is non-zero, so Python does **not** return "صفر" (it returns
/// the empty string). `to_str` never emits a sign or an exponent, so "all the
/// digits are 0" is the whole test.
fn number_str_is_zero(number: &str) -> bool {
    number.chars().all(|c| c == '0' || c == '.')
}

/// The group loop of `convert_to_arabic`, for the currency path.
///
/// # Why this is not `convert`'s loop
///
/// `lang_AR.py` was changed after the frozen corpus was generated: the
/// `decimal.InvalidOperation` handler now widens the precision inside a
/// `with decimal.localcontext() as ctx:` block, which **restores** the previous
/// precision on exit. The old code assigned `decimal.getcontext().prec`
/// directly, so one widening leaked into every later iteration (and every later
/// call in the process). [`convert`] still implements the old, leaking
/// behaviour; this loop implements what the current source does. See the
/// module-level "Decimal precision" note and the port report — the two are
/// deliberately, and visibly, out of step.
///
/// The observable difference: precision reverts to 28 each iteration, so once
/// the running value drops to 29..=31 digits the divide-by-1000 silently rounds
/// again instead of staying exact. `to_currency(10**51 - 1)` degrades until
/// group 17 is reached and trips `assert int(group_level) < len(arabicTwos)`
/// (AssertionError); under the leaking rule it would return a string.
///
/// # Why `integer_value` and not the fractional Decimal
///
/// Python loops over `Decimal(self.number)`, fraction included. For `x >= 0`,
/// `int(x % 1000) == floor(x) % 1000` and `int(x / 1000) == floor(x) // 1000`,
/// so the fraction is truncated away on the first iteration and the loop is
/// equivalent to one over `integer_value`. The precision rules cannot perturb
/// that: a `to_str` string only carries a fraction when the float was below
/// 2**53 (anything larger is integral), capping it at 16 + 9 = 25 significant
/// digits — inside prec 28, so the first iteration is always exact.
///
/// One shape difference is inert: for `0 < x < 1` Python runs one iteration
/// (`temp_number > 0`) with `number_to_process == 0`, while this loop runs
/// none. That iteration produces an empty `group_description`, which is
/// skipped, so `ret_val` is identical either way.
fn convert_groups_currency(integer_value: &BigInt, fem_name: bool) -> Result<String> {
    let thousand = BigInt::from(1000u16);
    let mut ret_val = String::new();
    let mut group: i32 = 0;
    let mut temp = integer_value.clone();

    while temp > BigInt::zero() {
        // `int(temp_number_dec % Decimal(str(1000)))` raises InvalidOperation
        // when the integer quotient has more digits than the context
        // precision. `%` itself is exact whenever it does not raise.
        let q_int = &temp / &thousand;
        let prec = if ndigits(&q_int) > DEFAULT_PREC {
            // `ctx.prec = len(temp_number_dec.as_tuple().digits)` — scoped to
            // this iteration by `localcontext`, hence recomputed rather than
            // carried.
            ndigits(&temp)
        } else {
            DEFAULT_PREC
        };
        let number_to_process = (&temp % &thousand).to_u32().expect("0..=999");

        // `temp_number = int(temp_number_dec / Decimal(1000))` — rounds to
        // `prec` significant digits, then truncates toward zero.
        temp = round_half_even_sig(&temp, prec) / &thousand;

        let group_description =
            process_arabic_group(number_to_process, group, integer_value, fem_name)?;

        if !group_description.is_empty() {
            if group > 0 {
                if !ret_val.is_empty() {
                    ret_val = format!("و{}", ret_val);
                }
                if number_to_process != 2 && number_to_process != 1 {
                    // Python: `assert group < len(self.arabicGroup)`.
                    if group >= ARABIC_GROUP.len() as i32 {
                        return Err(assertion_error("group < len(self.arabicGroup)"));
                    }
                    if number_to_process % 100 != 1 {
                        if (3..=10).contains(&number_to_process) {
                            ret_val =
                                format!("{} {}", ARABIC_PLURAL_GROUPS[group as usize], ret_val);
                        } else if !ret_val.is_empty() {
                            ret_val =
                                format!("{} {}", ARABIC_APPENDED_GROUP[group as usize], ret_val);
                        } else {
                            ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                        }
                    } else {
                        ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                    }
                }
            }
            ret_val = format!("{} {}", group_description, ret_val);
        }
        group += 1;
    }

    Ok(ret_val)
}

/// `Num2Word_AR.convert` → `extract_integer_and_decimal_parts` →
/// `convert_to_arabic`, for the currency path.
///
/// `number` is the `to_str` output. Returns `formatted_number` **unstripped** —
/// `to_currency` does not `.strip()` its result, which is why the corpus
/// records `to_currency(0.01)` as `" وإحدى هللة"` with a leading space.
fn convert_currency(number: &str, prefs: &CurrencyPrefs) -> Result<String> {
    // --- extract_integer_and_decimal_parts ---
    // `splits = re.split("\\.", str(self.number))`. `to_str` emits at most one
    // dot, so a 2-way split is exact.
    let mut splits = number.splitn(2, '.');
    let int_str = splits.next().unwrap_or("0");
    let frac = splits.next();
    let integer_value = BigInt::parse_bytes(int_str.as_bytes(), 10).ok_or_else(|| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            int_str
        ))
    })?;
    // `int(self.decimal_value(splits[1]))`. The result is at most
    // `partPrecision` (<= 3) digits, so it always fits.
    let decimal_value_num: u32 = match frac {
        Some(f) => decimal_value(f, prefs.part_precision)
            .parse()
            .map_err(|_| N2WError::Value("invalid literal for int() with base 10".into()))?,
        None => 0,
    };

    // --- convert_to_arabic ---
    if number_str_is_zero(number) {
        return Ok(ZERO_WORD.to_string());
    }

    // `self.isCurrencyNameFeminine = False` is set by `to_currency` before it
    // converts, so the masculine `arabicOnes` table is used at group_level 0.
    let fem_name = false;

    // Computed before the loop, exactly as Python does. group_level -1 routes
    // `digit_feminine_status` to `arabicFeminineOnes` unconditionally
    // (`isCurrencyPartNameFeminine` is True for the object's whole life), which
    // is why the subunit count is feminine — "أربع وثلاثون هللة" — while the
    // unit count above it is masculine.
    let decimal_string = process_arabic_group(decimal_value_num, -1, &integer_value, fem_name)?;

    let ret_val = convert_groups_currency(&integer_value, fem_name)?;

    // `formatted_number`: `arabicPrefixText`/`arabicSuffixText` are both "" in
    // currency mode, so their two branches contribute nothing.
    let mut formatted = ret_val;

    if !integer_value.is_zero() {
        let remaining100 = (&integer_value % 100u8).to_u32().expect("0..=99");
        if remaining100 == 0 {
            formatted.push_str(prefs.unit[0]);
        } else if remaining100 == 1 {
            formatted.push_str(prefs.unit[0]);
        } else if remaining100 == 2 {
            // Only a bare 2 takes the dual; 102, 1002, … take the singular.
            if integer_value == BigInt::from(2u8) {
                formatted.push_str(prefs.unit[1]);
            } else {
                formatted.push_str(prefs.unit[0]);
            }
        } else if (3..=10).contains(&remaining100) {
            formatted.push_str(prefs.unit[2]);
        } else if (11..=99).contains(&remaining100) {
            formatted.push_str(prefs.unit[3]);
        }
    }

    if decimal_value_num != 0 {
        // `formatted_number.rstrip()` before the separator, so the group loop's
        // trailing space does not double up.
        formatted = formatted.trim_end().to_string();
        // `self.separator` is "و" for the whole of `to_currency`, so the
        // `" {} ".format(self.separator)` arm is dead and the `separator=`
        // kwarg has no effect on this language whatsoever.
        formatted.push_str(" و");
        formatted.push_str(&decimal_string);

        formatted.push(' ');
        let remaining100 = decimal_value_num % 100;
        if remaining100 == 0 {
            formatted.push_str(prefs.subunit[0]);
        } else if remaining100 == 1 {
            formatted.push_str(prefs.subunit[0]);
        } else if remaining100 == 2 {
            formatted.push_str(prefs.subunit[1]);
        } else if (3..=10).contains(&remaining100) {
            formatted.push_str(prefs.subunit[2]);
        } else if (11..=99).contains(&remaining100) {
            formatted.push_str(prefs.subunit[3]);
        }
    }

    Ok(formatted)
}

/// `Num2Word_AR.validate_number` for the float branch of `to_currency`.
///
/// Python compares `number >= self.MAXVAL` with `number` an f64 and `MAXVAL`
/// the int `10**51`; CPython does that comparison **exactly**. It cannot be
/// done in f64 — `10**51` is not representable, and `float(1e51)` is in fact
/// *below* it (999999999999999993220948674361627976461708441944064), so
/// `to_currency(1e51)` does not overflow. So compare the float's exact integer
/// value instead: anything that could reach `10**51` is far above 2**53 and
/// therefore integral, making truncation lossless.
///
/// Like the integer branch this only tests the upper bound, never `abs`, so
/// negatives always pass (bug 2).
fn validate_number_float(x: f64) -> bool {
    if !x.is_finite() {
        return x > 0.0;
    }
    if x < 4.5e15 {
        // Below 2**52 — nowhere near MAXVAL, and the only range where a float
        // can carry a fraction at all.
        return false;
    }
    BigInt::from_f64(x.trunc())
        .map(|i| i >= maxval())
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Float / Decimal cardinal path
// ---------------------------------------------------------------------------
//
// AR never reaches `Num2Word_Base.to_cardinal_float`. `lang_AR.py` overrides
// `to_cardinal` outright, so base's
// `if int(value) != value: return self.to_cardinal_float(value)` never runs and
// **`float2tuple` is never called**. Everything `floatpath.rs` exists for — the
// `abs(round(post) - post) < 0.01` heuristic, `pointword`, spelling the
// fraction out digit by digit — is irrelevant here, and the generated
// `pointword()` on the impl below is inert: nothing reads it.
//
// What AR does instead is run the *same*
// `convert` → `to_str` → `extract_integer_and_decimal_parts` →
// `convert_to_arabic` pipeline the currency path uses, with `to_cardinal`'s own
// state:
//
//   * `self.currency_unit = self.currency_subunit = ("", "", "", "")`, so every
//     unit/subunit append contributes the empty string, and
//   * `self.separator = "،"` (Arabic comma U+060C) rather than `to_currency`'s
//     "و", which takes the `" {} ".format(self.separator)` arm instead.
//
// So `0.5` is `"، خمسون"`: no integer part, a comma, then the fraction read as
// a whole *two-digit number* ("fifty") — not as digits, and with no pointword
// anywhere. `partPrecision` is pinned to 2, so the fraction is zero-padded or
// **truncated** to exactly two digits: `12.345` and `12.34` are the same
// string, and the third decimal is dropped, never rounded.
//
// The float-artefact trap still bites, just somewhere else: `to_str` does
// `round((number - integer) * 10**9)` in f64, so `2.675` arrives as
// `674999999.9999998` and is rescued by `round()` to `675000000` — the same
// rescue base.py spells `< 0.01`. That already lives in [`to_str_float`] /
// [`py_round_half_even_f64`], shared verbatim with the currency path.

/// Python's `str(float)` (== `repr`) for the `errmsg_toobig` interpolation.
///
/// Only ever called on a value that failed [`validate_number_float`], i.e.
/// `x >= 10**51` or `+inf`. Python's repr switches to exponent form at
/// `exp >= 16`, so every such value prints scientific — which is exactly the
/// case Rust's `{}` gets wrong (it renders `1e52` as 53 literal digits).
/// `{:e}` is shortest-round-trip like repr but omits the `+` on a positive
/// exponent, so the sign is re-inserted here. `inf` matches as-is.
fn py_repr_f64_big(x: f64) -> String {
    if !x.is_finite() {
        // repr(float('inf')) == 'inf'; Rust agrees.
        return format!("{}", x);
    }
    let s = format!("{:e}", x);
    match s.split_once('e') {
        Some((mant, exp)) if !exp.starts_with('-') => format!("{}e+{}", mant, exp),
        _ => s,
    }
}

/// Python's `str(Decimal)` — the decimal spec's `to-scientific-string`.
///
/// Needed because `errmsg_toobig` interpolates the operand with `%s`, and
/// `BigDecimal`'s own `Display` is a different notation entirely: it normalises
/// the coefficient to an integer and keeps a lowercase `e`, rendering
/// `Decimal("1.5E+51")` as `15e+50` and `Decimal("123E+51")` as `123e+51` where
/// Python says `1.5E+51` and `1.23E+53`.
///
/// The spec's rule: use plain notation when `exponent <= 0` and
/// `adjusted >= -6`, otherwise scientific with the point after the first digit
/// and a signed exponent. `BigDecimal::as_bigint_and_exponent` returns
/// `(coefficient, scale)` for `coefficient * 10^-scale`, so Python's `exponent`
/// is `-scale` and its `digits` are the coefficient's — the same pair Python's
/// `as_tuple()` carries, since both keep the literal's trailing zeros
/// (`"1.10"` stays coefficient 110, exponent -2 on both sides).
///
/// Only the `>= 10**51` slice is reachable here (nothing else overflows), but
/// the general rule is cheaper to verify than a special case.
fn py_str_decimal(d: &BigDecimal) -> String {
    let (mant, scale) = d.as_bigint_and_exponent();
    let exponent = -scale;
    let neg = mant.is_negative();
    let digits = if neg { (-&mant).to_string() } else { mant.to_string() };
    let ndig = digits.len() as i64;
    let adjusted = exponent + ndig - 1;
    let sign = if neg { "-" } else { "" };

    if exponent <= 0 && adjusted >= -6 {
        if exponent == 0 {
            return format!("{}{}", sign, digits);
        }
        let point = ndig + exponent; // digits left of the '.'
        if point > 0 {
            let p = point as usize;
            format!("{}{}.{}", sign, &digits[..p], &digits[p..])
        } else {
            format!("{}0.{}{}", sign, "0".repeat((-point) as usize), digits)
        }
    } else {
        let mut m = digits[..1].to_string();
        if ndig > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        let e = if adjusted >= 0 {
            format!("+{}", adjusted)
        } else {
            format!("-{}", -adjusted)
        };
        format!("{}{}E{}", sign, m, e)
    }
}

/// Python's `round()` on a **non-negative** `BigDecimal` → `int`: nearest,
/// ties to even.
///
/// The `Decimal` sibling of [`py_round_half_even_f64`], and the same trap:
/// `round(Decimal("0.5"))` is `0` in Python, and anything ties-away would say
/// `1`. Only the non-negative case is modelled because `to_cardinal` applies
/// `self.abs` before `convert` ever sees the value.
///
/// Context precision does not enter: `round-to-integral` is exempt from it in
/// the decimal spec, and the operand here is `< 10**9` anyway (a fraction
/// times `10**9`), far inside prec 28.
fn py_round_half_even_dec(v: &BigDecimal) -> BigInt {
    let (mant, exp) = v.as_bigint_and_exponent();
    if exp <= 0 {
        // Already integral: value == mant * 10**(-exp).
        return mant * BigInt::from(10u8).pow((-exp) as u32);
    }
    let p = BigInt::from(10u8).pow(exp as u32);
    let q = &mant / &p;
    let r = &mant - &q * &p;
    let half = &p / 2u8; // exp >= 1, so p is even and this is exact
    let bump = r > half || (r == half && (&q % 2u8) != BigInt::zero());
    if bump {
        q + 1u8
    } else {
        q
    }
}

/// Round a `BigDecimal` to `prec` significant decimal digits, ROUND_HALF_EVEN.
///
/// Every `decimal` *arithmetic* operation (unlike `round()` above) computes the
/// exact result and then rounds it to the context precision — 28 by default.
/// That is observable in `to_str`: for a `Decimal` with 29 significant digits
/// like `0.00000000050000000000000000000000000001`, the `number - integer`
/// subtraction alone drops the trailing `1`, leaving exactly `5E-10`, so
/// `* 10**9` gives `0.5` and `round()` — ties to even — gives `0` rather than
/// the `1` the exact value would produce. `to_cardinal` of that Decimal really
/// is `"صفر"` while the 28-digit sibling is `""`.
///
/// Trailing-zero bookkeeping need not match Python's coefficient exactly: this
/// is value-based, and rounding a trailing zero away is exact, so counting
/// digits slightly differently than `as_tuple().digits` cannot change a result.
fn dec_round_prec(x: &BigDecimal, prec: usize) -> BigDecimal {
    let (mant, exp) = x.as_bigint_and_exponent();
    let neg = mant.is_negative();
    let m = if neg { -&mant } else { mant.clone() };
    let d = ndigits(&m);
    if d <= prec {
        return x.clone();
    }
    let k = d - prec;
    let p = BigInt::from(10u8).pow(k as u32);
    let q = &m / &p;
    let r = &m - &q * &p;
    let half = &p / 2u8;
    let bump = r > half || (r == half && (&q % 2u8) != BigInt::zero());
    let q = if bump { q + 1u8 } else { q };
    let q = if neg { -q } else { q };
    // The mantissa lost `k` digits, so the value keeps its scale by shedding
    // `k` from the exponent. A carry (999.. -> 100..) leaves one extra digit
    // versus Python's renormalised coefficient; it is a trailing zero, hence
    // value-neutral — see the note above.
    BigDecimal::new(q, exp - k as i64)
}

/// `Num2Word_AR.to_str` for a Python `Decimal`:
///
/// ```python
/// integer = int(number)
/// if integer == number:
///     return str(integer)
/// decimal = round((number - integer) * 10**9)
/// return str(integer) + "." + "{:09d}".format(decimal).rstrip("0")
/// ```
///
/// Same source line as [`to_str_float`], but every operator dispatches to
/// `Decimal`, not `float`: exact arbitrary precision, rounded to prec 28 per
/// operation. Casting to f64 first would defeat the whole point of the Decimal
/// arm — issue #603's `98746251323029.99` is representable exactly here and
/// only approximately as a double.
///
/// The two `to_str` quirks the float arm documents ride along unchanged, since
/// they live in the shared formatting tail rather than in the arithmetic:
/// `"{:09d}"` is a *minimum* width, so `Decimal("2.9999999999")` carries to
/// 1_000_000_000, prints ten digits, and `.rstrip("0")` leaves `"1"` — giving
/// `"2.1"`, not `"3"`. And `.rstrip("0")` can empty the fraction entirely,
/// leaving a trailing dot (`Decimal("1.0000000001")` → `"1."`).
fn to_str_decimal(x: &BigDecimal) -> String {
    // `integer = int(number)` — truncates toward zero.
    let integer = x.with_scale(0).as_bigint_and_exponent().0;
    // `integer == number` compares an int to a Decimal, i.e. numerically:
    // Decimal("1.00") == 1 is True, so `to_str(Decimal("200.00"))` is "200"
    // and the value re-enters the pure-integer path.
    if BigDecimal::from(integer.clone()) == *x {
        return integer.to_string();
    }
    let frac = dec_round_prec(&(x - BigDecimal::from(integer.clone())), DEFAULT_PREC);
    let scaled = dec_round_prec(
        &(frac * BigDecimal::from(BigInt::from(10u8).pow(9))),
        DEFAULT_PREC,
    );
    let decimal = py_round_half_even_dec(&scaled);
    let padded = format!("{:0>9}", decimal);
    format!("{}.{}", integer, padded.trim_end_matches('0'))
}

/// [`to_str_float`] plus the `int(number)` guard Python hits on its first line.
///
/// `int(float('inf'))` raises **OverflowError** and `int(float('nan'))` raises
/// **ValueError** — two different exception types, neither of them the
/// `errmsg_toobig` OverflowError. So `to_cardinal(-inf)` fails with
/// `"cannot convert float infinity to integer"` (it slips past
/// `validate_number`, which only tests the *upper* bound — bug 2 — and then
/// trips on `self.abs(-inf)` → `to_str(inf)`), while `to_cardinal(+inf)` fails
/// with `errmsg_toobig` from `validate_number` itself.
///
/// [`to_str_float`] cannot answer for these: `BigInt::from_f64(inf)` is `None`
/// and its `unwrap_or_else(BigInt::zero)` would silently report "صفر". The
/// guard lives here rather than there so the verified currency path keeps its
/// exact current behaviour.
fn to_str_float_checked(x: f64) -> Result<String> {
    if x.is_nan() {
        return Err(N2WError::Value("cannot convert float NaN to integer".into()));
    }
    if x.is_infinite() {
        return Err(N2WError::Overflow(
            "cannot convert float infinity to integer".into(),
        ));
    }
    Ok(to_str_float(x))
}

/// The group loop of `convert_to_arabic`, for the cardinal float/Decimal path.
///
/// # Why this is a third copy of the same loop
///
/// The three copies differ *only* in when `decimal`'s context precision makes
/// `int(temp_number_dec / Decimal(1000))` round, and they are three snapshots
/// of a method that has now been rewritten twice:
///
/// * [`convert`] — the original: `decimal.getcontext().prec` assigned inside an
///   `except InvalidOperation` handler and never restored, so one widening
///   leaked into every later iteration *and every later call in the process*.
/// * [`convert_groups_currency`] — the first fix: the same handler, but wrapped
///   in `with decimal.localcontext()`, so the widening reverted each iteration.
/// * this one — the current source, which has no handler at all:
///
/// ```python
/// with decimal.localcontext() as ctx:
///     digits = len(Decimal(self.number).as_tuple().digits)
///     if digits > ctx.prec:
///         ctx.prec = digits
///     return self._convert_to_arabic_inner()
/// ```
///
/// The precision is now widened **once, up front, to the whole operand's digit
/// count**, and the `with` covers the entire loop. That makes every step exact,
/// so there is no rounding to model and no danger band: dividing by 1000 only
/// shifts the exponent, so the quotient never needs more significant digits
/// than `temp` already had, and `int()` only removes digits. The running value
/// can therefore never exceed `ctx.prec` significant digits. This loop is plain
/// floor division on `BigInt`.
///
/// Verified against the live interpreter at the exact spot the other two rules
/// diverge: `to_cardinal(1e30)`, whose `to_str` is the 31-digit
/// `1000000000000000019884624838656` — squarely inside the old 29..=31 band —
/// returns the same string as `to_cardinal(1000000000000000019884624838656)`,
/// and `getcontext().prec` is still 28 afterwards. The other two rules would
/// round it to `…839000` and misread the thousands group. The three are
/// therefore deliberately out of step; see the module docs and the port report.
///
/// # Why `integer_value` and not the fractional Decimal
///
/// Python loops over `Decimal(self.number)`, fraction included. For `x >= 0`,
/// `int(x % 1000) == floor(x) % 1000` and `int(x / 1000) == floor(x) // 1000`,
/// so the fraction is truncated away on the first iteration and the loop is
/// equivalent to one over `integer_value`. One shape difference is inert: for
/// `0 < x < 1` Python runs one iteration (`temp_number > 0`) with
/// `number_to_process == 0`, while this loop runs none — that iteration yields
/// an empty `group_description`, which is skipped, so `ret_val` is identical.
fn convert_groups_exact(integer_value: &BigInt, fem_name: bool) -> Result<String> {
    let thousand = BigInt::from(1000u16);
    let mut ret_val = String::new();
    let mut group: i32 = 0;
    let mut temp = integer_value.clone();

    while temp > BigInt::zero() {
        let number_to_process = (&temp % &thousand).to_u32().expect("0..=999");
        temp /= &thousand;

        let group_description =
            process_arabic_group(number_to_process, group, integer_value, fem_name)?;

        if !group_description.is_empty() {
            if group > 0 {
                if !ret_val.is_empty() {
                    ret_val = format!("و{}", ret_val);
                }
                if number_to_process != 2 && number_to_process != 1 {
                    // Python: `assert group < len(self.arabicGroup)`.
                    if group >= ARABIC_GROUP.len() as i32 {
                        return Err(assertion_error("group < len(self.arabicGroup)"));
                    }
                    if number_to_process % 100 != 1 {
                        if (3..=10).contains(&number_to_process) {
                            ret_val =
                                format!("{} {}", ARABIC_PLURAL_GROUPS[group as usize], ret_val);
                        } else if !ret_val.is_empty() {
                            ret_val =
                                format!("{} {}", ARABIC_APPENDED_GROUP[group as usize], ret_val);
                        } else {
                            ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                        }
                    } else {
                        ret_val = format!("{} {}", ARABIC_GROUP[group as usize], ret_val);
                    }
                }
            }
            ret_val = format!("{} {}", group_description, ret_val);
        }
        group += 1;
    }

    Ok(ret_val)
}

/// `extract_integer_and_decimal_parts` + `convert_to_arabic`, for the cardinal
/// float/Decimal path. `number` is the [`to_str_float`]/[`to_str_decimal`]
/// output; the result is **unstripped** (`to_cardinal` strips it).
///
/// The sibling of [`convert_currency`] with `to_cardinal`'s state baked in:
/// `partPrecision = 2`, `isCurrencyNameFeminine = False`, `separator = "،"`,
/// and both currency tuples `("", "", "", "")`. Because the unit and subunit
/// forms are empty, both `% 100` cascades in `convert_to_arabic` append the
/// empty string for every branch, so they collapse out entirely — only the
/// `formatted_number += " "` before the (empty) subunit survives, as a trailing
/// space that `to_cardinal`'s `.strip()` removes.
fn convert_cardinal_float(number: &str) -> Result<String> {
    // --- extract_integer_and_decimal_parts ---
    // `splits = re.split("\\.", str(self.number))`; `to_str` emits at most one
    // dot, so a 2-way split is exact. The fraction can be empty ("1." from
    // to_str(1.0000000001)), which `decimal_value` pads to "00" -> 0.
    let mut splits = number.splitn(2, '.');
    let int_str = splits.next().unwrap_or("0");
    let frac = splits.next();
    let integer_value = BigInt::parse_bytes(int_str.as_bytes(), 10).ok_or_else(|| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            int_str
        ))
    })?;
    // `self.partPrecision = 2`, pinned by `to_cardinal` on every call — the
    // `precision=` kwarg cannot reach it (see `to_cardinal_float` below).
    let decimal_value_num: u32 = match frac {
        Some(f) => decimal_value(f, 2)
            .parse()
            .map_err(|_| N2WError::Value("invalid literal for int() with base 10".into()))?,
        None => 0,
    };

    // --- convert_to_arabic ---
    // Asked of the whole `to_str` string, not of `integer_value`: `0.001` has
    // integer part 0 yet a non-zero Decimal, so Python does *not* say "صفر" —
    // it returns "". Whereas `to_str(1e-10)` is literally "0.", which is zero.
    if number_str_is_zero(number) {
        return Ok(ZERO_WORD.to_string());
    }

    // `self.isCurrencyNameFeminine = False` — masculine `arabicOnes` at
    // group_level 0. The fraction below is still read feminine: it goes in at
    // group_level -1, where `digit_feminine_status` consults
    // `isCurrencyPartNameFeminine` (True for the object's whole life) instead.
    // Hence "اثنا عشر ، أربع وثلاثون" for 12.34 — masculine 12, feminine 34.
    let fem_name = false;

    // Computed before the loop, exactly as Python does.
    let decimal_string = process_arabic_group(decimal_value_num, -1, &integer_value, fem_name)?;

    let ret_val = convert_groups_exact(&integer_value, fem_name)?;

    // `formatted_number`: `arabicPrefixText`/`arabicSuffixText` are "" here, and
    // so is every `currency_unit`/`currency_subunit` form, so the only branches
    // left with an effect are the separator and the space before the subunit.
    let mut formatted = ret_val;

    if decimal_value_num != 0 {
        // `formatted_number.rstrip()` before the separator, so the group loop's
        // trailing space does not double up (issue #53).
        formatted = formatted.trim_end().to_string();
        // `self.separator` is "،", not "و", so this takes the
        // `" {} ".format(self.separator)` arm — the space on *both* sides is
        // why "واحد ، خمسون" has one around the comma and `0.5` alone yields a
        // leading space that `.strip()` then removes.
        formatted.push_str(" ، ");
        formatted.push_str(&decimal_string);
        formatted.push(' ');
    }

    Ok(formatted)
}

/// The shared body of `to_cardinal_float` / `to_cardinal_float_kw`:
/// `Num2Word_AR.to_cardinal(number, case=...)` reached with a **float** or
/// **Decimal** operand. See the hook docs on `LangAr::to_cardinal_float` for
/// the routing and state notes; `is_oblique` is [`is_oblique_case`]'s verdict
/// (`false` for the kwarg-less hook).
fn cardinal_float_case(value: &FloatValue, is_oblique: bool) -> Result<String> {
    // `number = self.validate_number(number)`, then `self.abs(number)`,
    // then `self.convert(...)` -> `to_str`. Both operand kinds take the
    // same shape; only the arithmetic differs.
    let (number, minus) = match value {
        FloatValue::Float { value: x, .. } => {
            // Upper bound only, on the *signed* value (bug 2), so -inf and
            // every other negative sails past and fails later — or not at
            // all.
            if validate_number_float(*x) {
                return Err(N2WError::Overflow(format!(
                    "abs({}) must be less than {}.",
                    py_repr_f64_big(*x),
                    maxval()
                )));
            }
            let minus = *x < 0.0;
            // `self.abs`: `number if number >= 0 else -number`. Note -0.0
            // takes the `>= 0` arm and stays -0.0, exactly as in Python;
            // `to_str` then reports "0" for it either way.
            let a = if *x >= 0.0 { *x } else { -*x };
            (to_str_float_checked(a)?, minus)
        }
        FloatValue::Decimal { value: d, .. } => {
            // Python compares Decimal >= int exactly. Never collapse this
            // to the f64 arm: issue #603's `98746251323029.99` is exact
            // here and only approximate as a double.
            if *d >= BigDecimal::from(maxval()) {
                return Err(N2WError::Overflow(format!(
                    "abs({}) must be less than {}.",
                    py_str_decimal(d),
                    maxval()
                )));
            }
            let minus = d.is_negative();
            // `self.abs`: `number if number >= 0 else -number`. The two
            // arms are NOT symmetric for a Decimal (bug 10): the positive
            // arm returns the operand untouched, while `-number` is a
            // `decimal` *context operation* and rounds its result to 28
            // significant digits. So a 32-digit negative loses its tail —
            // `Decimal("-646888273752320466851667632519.93")` becomes
            // `6.468882737523204668516676325E+29`, whose `to_str` is the
            // integral "646888273752320466851667632500": low digits gone,
            // fraction gone, no "، ثلاث وتسعون" segment. Its positive twin
            // reads out exactly. Verified against the live interpreter.
            let a = if d.is_negative() {
                dec_round_prec(&(-d), DEFAULT_PREC)
            } else {
                d.clone()
            };
            (to_str_decimal(&a), minus)
        }
    };

    let minus = if minus { MINUS } else { "" };
    let out = format!("{}{}", minus, convert_cardinal_float(&number)?.trim());

    // The `_AR_STANDALONE_DUAL` tail (bug 1: the character-set `lstrip` is
    // what lets a *negative* reach the table at all — "سالب مئتا" loses its
    // leading {س,ا,ل,ب,' '} run and becomes the key "مئتا", so
    // `to_cardinal(-200.0)` is "سالب مئتان"). A float only gets here when
    // `to_str` produced no fraction, so `200.5` stays the unrewritten
    // "مئتا ، خمسون".
    Ok(finish_cardinal_case(out, minus, is_oblique))
}

// ---------------------------------------------------------------------------
// Lang
// ---------------------------------------------------------------------------

pub struct LangAr;

impl LangAr {
    pub fn new() -> Self {
        LangAr
    }
}

impl Default for LangAr {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangAr {

    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        // Python class attribute MAXVAL (self-contained converter).
        Some(num_bigint::BigInt::from(10u32).pow(51))
    }
    /// `self.pointword`, read from the live Python instance.
    /// Unused by the four integer modes, so phase 1 never needed it —
    /// the float path is the first caller.
    fn pointword(&self) -> &str {
        "(.)"
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "SR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    // `cards`/`maxval`/`merge` stay at their trait defaults: AR is
    // self-contained and Python never builds `self.cards` for it. The
    // overflow ceiling is `validate_number`, not `maxval()`.

    /// `Num2Word_AR.to_cardinal(number, case="nominative")` at the default
    /// case: `"nominative"` is not a member of `_AR_OBLIQUE`, so `is_oblique`
    /// is `False` and only the nominative arm of `_AR_STANDALONE_DUAL` fires.
    /// The `case=` kwarg itself arrives through [`LangAr::to_cardinal_kw`].
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        cardinal_int_case(value, false)
    }

    /// `Num2Word_AR.to_cardinal(number, case=...)` — the only kwarg the
    /// signature accepts. A `case` of any non-string type (None, bool, int,
    /// list) fails Python's `isinstance(case, str)` test and silently means
    /// nominative; an unknown *string* ("bogus") is simply not in
    /// `_AR_OBLIQUE` and means nominative too. Nothing raises on `case`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        cardinal_int_case(value, is_oblique_case(kw.get("case")))
    }

    /// `Num2Word_AR.to_ordinal(number, gender="m", prefix="")` at the kwarg
    /// defaults (masculine, no prefix).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        to_ordinal_impl(value, false, "")
    }

    /// `Num2Word_AR.to_ordinal(number, gender=..., prefix=...)`.
    ///
    /// `gender_idx = 0 if gender == "m" else 1` — only the exact string
    /// `"m"` is masculine; `"M"`, `"f"`, `None`, ints, anything else compares
    /// unequal and selects the feminine column. `prefix` is interpolated with
    /// `"{} ".format(prefix)` in the >= 1000 / <= 0 fallback only (and even
    /// there, zero's early return beats it); no value of either kwarg raises.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender", "prefix"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let feminine = match kw.get("gender") {
            // Absent -> the "m" default -> masculine.
            None => false,
            // Only the exact string "m" is masculine.
            Some(KwVal::Str(s)) => s != "m",
            // None/bool/int/list all compare unequal to "m" -> feminine.
            Some(_) => true,
        };
        // verify_ordinal runs before the kwargs are read, matching Base:
        // to_ordinal(-1, gender="f") is TypeError, not a feminine "one".
        verify_ordinal(value)?;
        let prefix = kw.get("prefix").map(kwval_display).unwrap_or_default();
        to_ordinal_impl(value, feminine, &prefix)
    }

    /// `Num2Word_AR.to_ordinal_num(value)` == `self.to_ordinal(value).strip()`.
    ///
    /// `to_ordinal` already returns stripped text, so this is identical to
    /// `to_ordinal` for every input — as the frozen corpus confirms. Note the
    /// signature takes no kwargs, so `to_ordinal_num_kw` stays at the trait
    /// default (any kwarg → NotImplemented → Python's original TypeError).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        Ok(to_ordinal_impl(value, false, "")?.trim().to_string())
    }

    /// `to_ordinal(float/Decimal)`.
    ///
    /// Upstream's first line used to be `number = int(number)`, so a
    /// fractional operand was *truncated toward zero* and read as an ordinary
    /// ordinal — `to_ordinal(2.5)` was `"الثاني"` and `to_ordinal(-1.5)`
    /// truncated to -1 and hit the feminine-leak fallback `"إحدى"`.
    /// savoirfairelinux/num2words#672 puts `verify_ordinal` ahead of that, so
    /// both now raise `TypeError`. A *whole* float is still fine: `-0.0`
    /// verifies as 0 and renders `"صفر"`.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        to_ordinal_impl(&verify_ordinal_float(value)?, false, "")
    }

    /// `to_ordinal_num(float/Decimal)` == `to_ordinal(value).strip()` — AR
    /// overrides base's echo-the-repr default outright, so floats and
    /// Decimals get the same `int()`-truncated ordinal as [`Self::ordinal_float_entry`].
    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        Ok(to_ordinal_impl(&verify_ordinal_float(value)?, false, "")?
            .trim()
            .to_string())
    }

    /// `converter.str_to_number` — base's `Decimal(value)`, except that a NaN
    /// parse is converted to the error AR's *first use* of the value raises.
    ///
    /// Python's `str_to_number("NaN")` succeeds (`Decimal("NaN")`), and the
    /// dispatcher then calls `to_cardinal`, whose `validate_number` runs
    /// `number >= self.MAXVAL` — an ordering comparison, which on a NaN
    /// Decimal raises `decimal.InvalidOperation` (str
    /// `"[<class 'decimal.InvalidOperation'>]"`). The binding intercepts
    /// `ParsedNumber::NaN` generically as base's ValueError before any
    /// per-language hook runs, so the rewrite has to happen here.
    ///
    /// Deliberate trade-off, corpus-first: the only NaN rows for `ar` are
    /// `to=cardinal` (and year/currency would validate identically), but
    /// `to_ordinal(Decimal("NaN"))` in Python raises ValueError from
    /// `int(number)` instead — that non-corpus shape becomes InvalidOperation
    /// here. Infinity is left on the binding's generic path: both sides raise
    /// OverflowError (different message, same type — the corpus records only
    /// the type).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    /// `Num2Word_AR.to_year(value)` — validates, then delegates to
    /// `to_cardinal` (which validates again). No year-specific formatting.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        validate_number(value)?;
        self.to_cardinal(value)
    }

    /// `Num2Word_AR.to_cardinal(number, case="nominative")` reached with a
    /// **float** or **Decimal** operand.
    ///
    /// Not an override of `Num2Word_Base.to_cardinal_float` — AR has no such
    /// method. `lang_AR.py` replaces `to_cardinal` wholesale, so base's
    /// `if int(value) != value: return self.to_cardinal_float(value)` never
    /// runs and float and int input land in the *same* method. This hook is
    /// simply where the trait routes non-integral input; the body is
    /// `to_cardinal`'s, re-typed for the two operand kinds.
    ///
    /// # `precision_override` is deliberately ignored
    ///
    /// It carries the dispatcher's `precision=` kwarg (issue #580), which
    /// reaches a converter two ways, and AR is deaf to both:
    ///
    /// * as a kwarg — `to_cardinal(number, case="nominative")` has no
    ///   `precision` parameter, and the dispatcher pops `precision` out of
    ///   `kwargs` before the call anyway;
    /// * as `converter.precision` — the dispatcher does set that attribute
    ///   (AR inherits one from `Num2Word_Base`, so its `hasattr` guard passes),
    ///   but nothing in AR ever reads it. The fractional width is
    ///   `self.partPrecision`, which `to_cardinal` re-pins to 2 on entry.
    ///
    /// Confirmed against the live interpreter:
    /// `num2words(12.345, lang="ar", precision=p)` is `"اثنا عشر ، أربع وثلاثون"`
    /// for every `p` in 1, 3, 5 and for no `precision=` at all.
    ///
    /// # State
    ///
    /// `to_cardinal` re-assigns every field it reads —
    /// `isCurrencyNameFeminine`, `separator`, `currency_unit`,
    /// `currency_subunit`, `arabicPrefixText`, `arabicSuffixText`,
    /// `arabicOnes`, `partPrecision` — on entry, so nothing `to_currency`
    /// leaves behind (it rebinds `partPrecision` and the currency tuples) can
    /// reach it. A stateless port is faithful.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        cardinal_float_case(value, false)
    }

    /// `Num2Word_AR.to_cardinal(number, case=...)` with a float/Decimal
    /// operand — the same body as [`Self::to_cardinal_float`] (see
    /// [`cardinal_float_case`]) with the oblique rewrites live.
    /// `precision_override` stays inert for the reasons documented above.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        cardinal_float_case(value, is_oblique_case(kw.get("case")))
    }

    // ---- currency ------------------------------------------------------

    /// Only reached through `to_cheque`. `Num2Word_AR.CURRENCY_FORMS` is
    /// Base's empty dict, so `currency_forms` stays at its `None` default and
    /// the trait's `to_cheque` raises
    /// `Currency code "X" not implemented for "Num2Word_AR"` for every code —
    /// which is exactly what the corpus records for all nine cheque rows.
    ///
    /// `currency_adjective`, `currency_precision`, `pluralize`,
    /// `money_verbose`, `cents_verbose`, `cents_terse` and
    /// `cardinal_from_decimal` are all left at their defaults too: AR's
    /// `to_currency` is self-contained and calls none of them, and
    /// `to_cheque` raises before it can reach `currency_precision`.
    fn lang_name(&self) -> &str {
        "Num2Word_AR"
    }

    /// `Num2Word_AR.to_currency(n, currency="SR", cents=True, separator=",",
    /// adjective=False)`.
    ///
    /// A wholesale replacement — it never calls `super()`, so no part of
    /// `Num2Word_Base.to_currency` runs and `currency::default_to_currency` is
    /// bypassed. Consequences worth spelling out, all corpus-confirmed:
    ///
    /// * **`cents`, `separator` and `adjective` are dead parameters.** The body
    ///   reads none of them; it pins `self.separator = "و"` itself. So the
    ///   generated `default_separator` (",") is inert for this language, and
    ///   `to_currency(x, separator=" و ")` changes nothing.
    /// * **No currency code ever raises.** `set_currency_prefer`'s `else`
    ///   falls back to Saudi riyals, so `currency:EUR` renders riyals rather
    ///   than raising NotImplementedError.
    /// * **`CURRENCY_PRECISION` is not consulted**, so JPY is not treated as a
    ///   0-decimal currency and BHD/KWD are not 3-decimal ones — only TND gets
    ///   `partPrecision = 3`, from `set_currency_prefer`.
    /// * **The int/float split is `to_str`'s, not `isinstance(val, int)`'s.**
    ///   Base skips the cents segment for ints; AR instead skips it whenever
    ///   `int(number) == number`, which catches `1.0` as well as `1`. So
    ///   `has_decimal` is not consulted here — `to_str` subsumes it.
    /// * The result is **not** stripped, so a sub-unit-only amount keeps its
    ///   leading space: `to_currency(0.01)` is `" وإحدى هللة"`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        let prefs = set_currency_prefer(currency);

        // `self.isCurrencyNameFeminine = False`, `arabicPrefixText = ""`,
        // `arabicSuffixText = ""`, `self.separator = "و"` — all constant for
        // this path, so they are folded into the callees rather than stored.
        //
        // Note `set_currency_prefer` also rebinds `self.partPrecision`, which
        // `to_cardinal`/`to_ordinal` reset on their own way in — the Python
        // instance is shared and mutable, but no currency state can leak into
        // the four integer modes, and `to_currency` never reads state it did
        // not just write. Hence a stateless port is faithful.
        let (number, minus) = match val {
            CurrencyValue::Int(v) => {
                // `n = self.validate_number(n)` — on the *signed* value.
                validate_number(v)?;
                let neg = v.is_negative();
                let n = if neg { -v } else { v.clone() };
                (to_str_int(&n), neg)
            }
            CurrencyValue::Decimal { value, .. } => {
                // The float Python started from: `CurrencyValue::Decimal` is
                // parsed from `str(value)`, so re-parsing the exact decimal
                // recovers the identical bit pattern.
                let x: f64 = value.to_string().parse().map_err(|_| {
                    N2WError::Value(format!("could not convert string to float: '{}'", value))
                })?;
                if validate_number_float(x) {
                    return Err(N2WError::Overflow(format!(
                        "abs({}) must be less than {}.",
                        value,
                        maxval()
                    )));
                }
                let neg = x < 0.0;
                let n = if neg { -x } else { x };
                (to_str_float(n), neg)
            }
        };

        let result = convert_currency(&number, &prefs)?;
        // `if minus: return minus + result` — no strip, so a negative
        // sub-unit-only amount doubles the space: `-0.01` → "سالب  وإحدى هللة".
        if minus {
            return Ok(format!("{}{}", MINUS, result));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// The dispatcher hands floats over as the raw f64 (`precision` is derived
    /// from `repr` on the Python side and AR never reads it).
    fn f(x: f64) -> FloatValue {
        FloatValue::Float { value: x, precision: 0 }
    }
    /// `FloatValue::Decimal` is built from `str(value)`, so parsing the same
    /// string here reproduces what the binding constructs.
    fn d(s: &str) -> FloatValue {
        FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: 0 }
    }
    fn card(v: FloatValue) -> String {
        LangAr::new().to_cardinal_float(&v, None).unwrap()
    }

    /// Every `"lang": "ar", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        for (arg, out) in [
            ("0.0", "صفر"),
            ("0.5", "، خمسون"),
            ("1.0", "واحد"),
            ("1.5", "واحد ، خمسون"),
            ("2.25", "اثنان ، خمس وعشرون"),
            ("3.14", "ثلاثة ، أربع عشرة"),
            ("0.01", "، إحدى"),
            ("0.1", "، عشر"),
            ("0.99", "، تسع وتسعون"),
            ("1.01", "واحد ، إحدى"),
            ("12.34", "اثنا عشر ، أربع وثلاثون"),
            ("99.99", "تسعة وتسعون ، تسع وتسعون"),
            ("100.5", "مائة ، خمسون"),
            ("1234.56", "ألف ومئتان وأربعة وثلاثون ، ست وخمسون"),
            ("-0.5", "سالب ، خمسون"),
            ("-1.5", "سالب واحد ، خمسون"),
            ("-12.34", "سالب اثنا عشر ، أربع وثلاثون"),
            ("1.005", "واحد"),
            ("2.675", "اثنان ، سبع وستون"),
        ] {
            assert_eq!(card(f(arg.parse().unwrap())), out, "float {}", arg);
        }
    }

    /// Every `"lang": "ar", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        for (arg, out) in [
            ("0.01", "، إحدى"),
            ("1.10", "واحد ، عشر"),
            ("12.345", "اثنا عشر ، أربع وثلاثون"),
            // Issue #603: exact at trillion scale. A float() cast would round.
            (
                "98746251323029.99",
                "ثمانية وتسعون تريليوناً وسبعمائة وستة وأربعون ملياراً ومئتان وواحد وخمسون مليوناً وثلاثمائة وثلاثة وعشرون ألفاً وتسعة وعشرون ، تسع وتسعون",
            ),
            ("0.001", ""),
        ] {
            assert_eq!(card(d(arg)), out, "Decimal {}", arg);
        }
    }

    /// `partPrecision` is pinned to 2, so a third decimal is *truncated*, never
    /// rounded, and `precision=` cannot change that (see `to_cardinal_float`).
    #[test]
    fn fraction_is_truncated_to_two_digits_not_rounded() {
        assert_eq!(card(f(12.345)), card(f(12.34)));
        assert_eq!(card(d("12.999")), card(d("12.99")));
        for p in [None, Some(1), Some(3), Some(5)] {
            let got = LangAr::new().to_cardinal_float(&f(12.345), p).unwrap();
            assert_eq!(got, "اثنا عشر ، أربع وثلاثون", "precision={:?} must be inert", p);
        }
    }

    /// The f64 artefacts `to_str`'s banker's `round()` rescues — AR's analogue
    /// of base.py's `< 0.01` heuristic. `2.675 - 2` is `0.6749999999999998`, so
    /// `* 1e9` is `674999999.9999998`; `round()` pulls it back to `675000000`,
    /// and `partPrecision` then truncates "675" to "67".
    #[test]
    fn float_artefacts_are_reproduced_not_repaired() {
        assert_eq!(card(f(2.675)), "اثنان ، سبع وستون"); // 67, not 68
        // 1.005 -> "005" -> truncated to "00" -> the fraction vanishes entirely.
        assert_eq!(card(f(1.005)), "واحد");
        // "{:09d}" is a *minimum* width: this carries to 1_000_000_000, prints
        // ten digits, and rstrip("0") leaves "1" -> "2.1", not "3".
        assert_eq!(card(f(2.9999999999)), "اثنان ، عشر");
        // rstrip("0") can empty the fraction, leaving a trailing dot: "1."
        assert_eq!(card(f(1.0000000001)), "واحد");
        // ... and "0.", which Decimal("0.") reads back as zero.
        assert_eq!(card(f(1e-10)), "صفر");
        // Banker's rounding, the other trap: round(0.5) == 0, not 1.
        assert_eq!(card(d("0.0000000005000000000000000000000000001")), "");
    }

    /// `0.001` has integer part 0 but a non-zero Decimal, so `convert_to_arabic`
    /// does *not* say "صفر" — it returns the empty string.
    #[test]
    fn zero_word_is_asked_of_the_whole_to_str_string() {
        assert_eq!(card(f(0.0)), "صفر");
        assert_eq!(card(f(-0.0)), "صفر"); // self.abs keeps -0.0; to_str says "0"
        assert_eq!(card(d("0")), "صفر");
        assert_eq!(card(d("-0")), "صفر");
        assert_eq!(card(d("0.00")), "صفر");
        assert_eq!(card(f(0.001)), "");
        assert_eq!(card(d("0.001")), "");
    }

    /// Bug 1: `out.lstrip(minus)` strips a character *set*, so a negative loses
    /// its leading {س,ا,ل,ب,' '} run and can reach `_AR_STANDALONE_DUAL`.
    #[test]
    fn standalone_dual_rewrite_reaches_negatives() {
        assert_eq!(card(f(200.0)), "مئتان");
        assert_eq!(card(f(-200.0)), "سالب مئتان");
        assert_eq!(card(f(2000.0)), "ألفان");
        assert_eq!(card(f(-2000.0)), "سالب ألفان");
        assert_eq!(card(d("200.00")), "مئتان");
        assert_eq!(card(d("-200.00")), "سالب مئتان");
        // A fraction blocks it: `bare` is no longer the bare construct form.
        assert_eq!(card(f(200.5)), "مئتا ، خمسون");
        assert_eq!(card(f(2000.5)), "ألفا ، خمسون");
        // Only the six keys are rewritten; 2e30's "نونيليونا" is not one.
        assert_eq!(card(d("2E+30")), "نونيليونا");
    }

    /// Bug 10: `self.abs` is asymmetric for a Decimal. `-number` is a `decimal`
    /// *context operation* and rounds to 28 significant digits; the positive
    /// arm returns the operand untouched.
    #[test]
    fn abs_rounds_negative_decimals_to_28_significant_digits() {
        // 32 significant digits: the negative loses its tail *and* its fraction.
        let neg = card(d("-646888273752320466851667632519.93"));
        assert!(neg.ends_with("وخمسمائة"), "tail should be 500, got {:?}", neg);
        assert!(!neg.contains('،'), "fraction must vanish, got {:?}", neg);
        // The positive twin reads out exactly: ...519 , 93.
        let pos = card(d("646888273752320466851667632519.93"));
        assert!(pos.ends_with("وتسعة عشر ، ثلاث وتسعون"), "got {:?}", pos);
        // At <= 28 significant digits the two arms agree again.
        assert_eq!(
            card(d("-12345678901234567890123456.78")).trim_start_matches("سالب ").to_string(),
            card(d("12345678901234567890123456.78")),
        );
    }

    /// The current `convert_to_arabic` widens the precision once, up front, to
    /// the whole operand's digit count, so the group loop never rounds — see
    /// [`convert_groups_exact`]. `1e30`'s `to_str` is the 31-digit
    /// `1000000000000000019884624838656`, inside the *old* danger band.
    #[test]
    fn group_loop_is_exact_across_the_old_danger_band() {
        let via_float = card(f(1e30));
        let via_decimal = card(d("1000000000000000019884624838656"));
        assert_eq!(via_float, via_decimal);
        // ...838 thousand, not the ...839 the two stale precision rules give.
        assert!(via_float.contains("وثمانمائة وثمانية وثلاثون ألفاً"), "got {:?}", via_float);
        // Bug 9: arabicTwos[10] ships a trailing space, so 2e30 double-spaces.
        assert!(card(f(2e30)).starts_with("نونيليونان  و"));
    }

    /// Bug 2: `validate_number` only tests the *upper* bound, so negatives never
    /// overflow — they trip a bare `assert` deep inside instead (bug 4).
    #[test]
    fn bounds_and_error_variants() {
        let l = LangAr::new();
        // float(1e51) is *below* 10**51, so it does not overflow.
        assert!(l.to_cardinal_float(&f(1e51), None).is_ok());
        match l.to_cardinal_float(&f(1e52), None) {
            Err(N2WError::Overflow(m)) => assert_eq!(
                m,
                "abs(1e+52) must be less than 1000000000000000000000000000000000000000000000000000."
            ),
            o => panic!("expected Overflow, got {:?}", o),
        }
        // str(Decimal) keeps the uppercase E and the spec's coefficient form.
        match l.to_cardinal_float(&d("1E+51"), None) {
            Err(N2WError::Overflow(m)) => assert_eq!(
                m,
                "abs(1E+51) must be less than 1000000000000000000000000000000000000000000000000000."
            ),
            o => panic!("expected Overflow, got {:?}", o),
        }
        // Negative: past validate_number, then AssertionError — not Overflow.
        assert!(matches!(
            l.to_cardinal_float(&f(-1e52), None),
            Err(N2WError::Assertion(_))
        ));
        assert!(matches!(
            l.to_cardinal_float(&d("-1000000000000000000000000000000000000000000000000000"), None),
            Err(N2WError::Assertion(_))
        ));
        // +inf overflows via validate_number; -inf slips past it and dies in
        // `int(number)` inside to_str, with a different message entirely.
        match l.to_cardinal_float(&f(f64::INFINITY), None) {
            Err(N2WError::Overflow(m)) => assert!(m.starts_with("abs(inf) must be less than")),
            o => panic!("expected Overflow, got {:?}", o),
        }
        match l.to_cardinal_float(&f(f64::NEG_INFINITY), None) {
            Err(N2WError::Overflow(m)) => assert_eq!(m, "cannot convert float infinity to integer"),
            o => panic!("expected Overflow, got {:?}", o),
        }
        match l.to_cardinal_float(&f(f64::NAN), None) {
            Err(N2WError::Value(m)) => assert_eq!(m, "cannot convert float NaN to integer"),
            o => panic!("expected ValueError, got {:?}", o),
        }
    }

    /// `py_str_decimal` against Python's `str(Decimal)` on both notations.
    #[test]
    fn py_str_decimal_matches_python() {
        for (input, want) in [
            ("1E+51", "1E+51"),
            ("1.5E+51", "1.5E+51"),
            ("123E+51", "1.23E+53"),
            ("9.99E+51", "9.99E+51"),
            ("1.10", "1.10"),
            ("0.5", "0.5"),
            ("12.345", "12.345"),
        ] {
            assert_eq!(py_str_decimal(&BigDecimal::from_str(input).unwrap()), want, "input {}", input);
        }
    }

    /// The fraction is read feminine (group_level -1 →
    /// `isCurrencyPartNameFeminine`, True for the object's whole life) while the
    /// integer above it is masculine (`isCurrencyNameFeminine = False`).
    #[test]
    fn fraction_is_feminine_integer_is_masculine() {
        // 12 masculine "اثنا عشر"; 34 feminine "أربع وثلاثون".
        assert_eq!(card(f(12.34)), "اثنا عشر ، أربع وثلاثون");
        // 2 masculine "اثنان"; 0.02's 2 feminine "اثنتان".
        assert_eq!(card(f(2.02)), "اثنان ، اثنتان");
    }

    fn ord_f(v: FloatValue) -> String {
        LangAr::new().ordinal_float_entry(&v).unwrap()
    }
    fn ordnum_f(v: FloatValue, repr: &str) -> String {
        LangAr::new().ordinal_num_float_entry(&v, repr).unwrap()
    }
    fn kws(pairs: &[(&str, KwVal)]) -> Kwargs {
        Kwargs(pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
    }

    /// `to_ordinal` floats/Decimals: `verify_ordinal` first (ported from
    /// savoirfairelinux/num2words#672), then the ordinary ordinal
    /// tables/fallback for the whole, non-negative remainder.
    ///
    /// The negative and fractional rows this used to assert now live in
    /// [`ordinal_rejects_negative_and_fractional`] — they raise `TypeError`
    /// rather than truncating.
    #[test]
    fn ordinal_float_entry_truncates_like_int() {
        for (arg, out) in [
            (-0.0, "صفر"),
            (0.0, "صفر"),
            (1.0, "الأول"),
            (12.0, "الثاني عشر"),
            (20.0, "العشرون"),
            (21.0, "الأول والعشرون"),
            (42.0, "الثاني والأربعون"),
            (100.0, "المائة"),
            (101.0, "الأول بعد المائة"),
            (1234.0, "ألف ومئتان وأربعة وثلاثون"),
            (1e16, "عشرة كوادريليونات"),
            (1e20, "مائة كوينتليون"),
        ] {
            assert_eq!(ord_f(f(arg)), out, "ordinal {}", arg);
            assert_eq!(ordnum_f(f(arg), "x"), out, "ordinal_num {}", arg);
        }
        for (arg, out) in [
            ("0", "صفر"),
            ("5", "الخامس"),
            ("5.00", "الخامس"),
            ("1E+2", "المائة"),
            ("12345.000", "اثنا عشر ألفاً وثلاثمائة وخمسة وأربعون"),
            ("1E+20", "مائة كوينتليون"),
            ("-0.0", "صفر"),
        ] {
            assert_eq!(ord_f(d(arg)), out, "ordinal Decimal {}", arg);
            assert_eq!(ordnum_f(d(arg), "x"), out, "ordinal_num Decimal {}", arg);
        }
        // int(inf)/int(nan), same errors as to_str's guard.
        assert!(matches!(
            LangAr::new().ordinal_float_entry(&f(f64::INFINITY)),
            Err(N2WError::Overflow(_))
        ));
        assert!(matches!(
            LangAr::new().ordinal_float_entry(&f(f64::NAN)),
            Err(N2WError::Value(_))
        ));
    }

    /// `case=` kwarg on to_cardinal: oblique aliases switch the dual endings;
    /// anything else — unknown strings, None, ints — silently means
    /// nominative. All expectations are kwargs-corpus rows or interpreter-verified.
    #[test]
    fn cardinal_case_kwarg() {
        let l = LangAr::new();
        let n = |v: i64| BigInt::from(v);
        for case in ["a", "accusative", "نصب", "منصوب", "g", "genitive", "جر", "مجرور", "o", "oblique"] {
            let kw = kws(&[("case", KwVal::Str(case.into()))]);
            assert_eq!(
                l.to_cardinal_kw(&n(1234), &kw).unwrap(),
                "ألف ومئتين وأربعة وثلاثون",
                "case={}",
                case
            );
            // Standalone dual takes the ين column.
            assert_eq!(l.to_cardinal_kw(&n(200), &kw).unwrap(), "مئتين");
            assert_eq!(l.to_cardinal_kw(&n(-200), &kw).unwrap(), "سالب مئتين");
            // "اثنان" is not in the rewrite list — stays nominative-shaped.
            assert_eq!(l.to_cardinal_kw(&n(2), &kw).unwrap(), "اثنان");
        }
        // Upper-case goes through str.lower() first.
        let kw = kws(&[("case", KwVal::Str("ACCUSATIVE".into()))]);
        assert_eq!(l.to_cardinal_kw(&n(200), &kw).unwrap(), "مئتين");
        // Nominative / unknown / non-str: no rewrite, no raise.
        for v in [
            KwVal::Str("nominative".into()),
            KwVal::Str("bogus".into()),
            KwVal::None,
            KwVal::Int(3),
            KwVal::Bool(true),
        ] {
            let kw = kws(&[("case", v)]);
            assert_eq!(l.to_cardinal_kw(&n(1234), &kw).unwrap(), "ألف ومئتان وأربعة وثلاثون");
            assert_eq!(l.to_cardinal_kw(&n(200), &kw).unwrap(), "مئتان");
        }
        // Unknown kwarg -> Fallback (decline signal) -> Python's TypeError.
        let kw = kws(&[("plural", KwVal::Bool(true))]);
        assert!(matches!(
            l.to_cardinal_kw(&n(5), &kw),
            Err(N2WError::Fallback(_))
        ));
        // Float operand, same kwarg (interpreter-verified).
        let kw = kws(&[("case", KwVal::Str("a".into()))]);
        assert_eq!(l.to_cardinal_float_kw(&f(200.0), None, &kw).unwrap(), "مئتين");
        assert_eq!(
            l.to_cardinal_float_kw(&f(1234.5), None, &kw).unwrap(),
            "ألف ومئتين وأربعة وثلاثون ، خمسون"
        );
        // Fraction blocks the standalone-dual key, and "مئتا" (construct) has
        // no "مئتان" substring to rewrite.
        assert_eq!(l.to_cardinal_float_kw(&f(200.5), None, &kw).unwrap(), "مئتا ، خمسون");
    }

    /// `gender=`/`prefix=` kwargs on to_ordinal. gender is masculine only for
    /// the exact "m"; prefix surfaces only in the fallback, and zero's early
    /// return beats it.
    #[test]
    fn ordinal_gender_and_prefix_kwargs() {
        let l = LangAr::new();
        let n = |v: i64| BigInt::from(v);
        let fem = kws(&[("gender", KwVal::Str("f".into()))]);
        assert_eq!(l.to_ordinal_kw(&n(1), &fem).unwrap(), "الأولى");
        assert_eq!(l.to_ordinal_kw(&n(11), &fem).unwrap(), "الحادية عشرة");
        assert_eq!(l.to_ordinal_kw(&n(21), &fem).unwrap(), "الأولى والعشرون");
        assert_eq!(l.to_ordinal_kw(&n(100), &fem).unwrap(), "المائة");
        // A negative is rejected before the kwargs are read (#672); it used
        // to fall through to the gender-ignoring fallback and return "خمس".
        assert!(matches!(
            l.to_ordinal_kw(&n(-5), &fem),
            Err(N2WError::Type(_))
        ));
        assert_eq!(l.to_ordinal_kw(&n(1234), &fem).unwrap(), "ألف ومئتان وأربعة وثلاثون");
        // Any non-"m" gender is feminine — "M", None, ints included.
        for v in [KwVal::Str("M".into()), KwVal::None, KwVal::Int(0)] {
            let kw = kws(&[("gender", v)]);
            assert_eq!(l.to_ordinal_kw(&n(1), &kw).unwrap(), "الأولى");
        }
        // prefix: fallback only ("xx ألفا"), zero early-returns bare "صفر",
        // table branches never see it.
        let px = kws(&[("prefix", KwVal::Str("xx".into()))]);
        assert_eq!(l.to_ordinal_kw(&n(2000), &px).unwrap(), "xx ألفا");
        assert_eq!(l.to_ordinal_kw(&n(0), &px).unwrap(), "صفر");
        assert_eq!(l.to_ordinal_kw(&n(5), &px).unwrap(), "الخامس");
        // Unknown kwarg falls back to Python (Fallback decline signal).
        let kw = kws(&[("case", KwVal::Str("a".into()))]);
        assert!(matches!(
            l.to_ordinal_kw(&n(5), &kw),
            Err(N2WError::Fallback(_))
        ));
    }

    /// `num2words("NaN", lang="ar")`: str_to_number succeeds in Python and the
    /// InvalidOperation comes from validate_number's `>=` on the NaN Decimal;
    /// the override surfaces it from str_to_number because the binding
    /// intercepts ParsedNumber::NaN before any per-language hook.
    #[test]
    fn str_to_number_nan_raises_invalid_operation() {
        let l = LangAr::new();
        match l.str_to_number("NaN") {
            Err(N2WError::Custom { module, class, msg }) => {
                assert_eq!(module, "decimal");
                assert_eq!(class, "InvalidOperation");
                assert_eq!(msg, "[<class 'decimal.InvalidOperation'>]");
            }
            o => panic!("expected InvalidOperation, got {:?}", o),
        }
        // Infinity keeps the binding's generic OverflowError path.
        assert!(matches!(
            l.str_to_number("Infinity"),
            Ok(ParsedNumber::Inf { negative: false })
        ));
        // Ordinary parses are untouched.
        assert!(matches!(l.str_to_number("1.5"), Ok(ParsedNumber::Dec(_))));
        assert!(l.str_to_number("abc").is_err());
    }
}
