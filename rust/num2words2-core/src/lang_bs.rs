//! Port of `lang_BS.py` (Bosnian, Latin script).
//!
//! Shape: **self-contained**. `Num2Word_BS` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` probe in `Num2Word_Base.__init__` fails and Python never builds
//! `self.cards` nor sets `self.MAXVAL`. `to_cardinal` is overridden outright
//! and drives a hand-rolled `_int_to_word` recursion. Consequently
//! `cards`/`merge` stay at their trait defaults here. Python has **no
//! overflow check** — see bug 2 below for what it did instead; this port
//! raises `OverflowError` from 10^24 (`maxval`).
//!
//! Inherited from `Num2Word_Base` (unchanged by BS):
//!   * `setup()` sets `negword = "minus "` (note the **trailing space**) and
//!     `pointword = "zarez"`.
//!
//! Overridden by BS, all four in scope:
//!   * `to_cardinal` / `to_ordinal` / `to_ordinal_num` / `to_year`.
//!
//! # The float / Decimal path (phase 3)
//!
//! BS does **not** override `to_cardinal_float` — it never reaches it at all.
//! `num2words()` dispatches `to="cardinal"` straight to `Num2Word_BS
//! .to_cardinal(number)` with the raw `float`/`Decimal`, and that override's
//! very first statement is `n = str(number).strip()`. So `Num2Word_Base
//! .to_cardinal`'s `assert int(value) == value` guard — the thing that
//! delegates to `to_cardinal_float` for the other 26 languages — is dead code
//! for BS. See [`LangBs::to_cardinal_float`].
//!
//! The consequence is that **none of `float2tuple`'s binary arithmetic runs**.
//! BS never computes `abs(value - pre) * 10**precision`, so the f64 artefacts
//! `floatpath.rs` is built to preserve, and the `< 0.01` heuristic that
//! rescues them, simply do not arise. `2.675` is the string `"2.675"` and its
//! fractional digits are read off as characters. The two routes agree on
//! `2.675`/`1.005` precisely *because* that heuristic exists to recover the
//! `repr` digits that the string route gets for free.
//!
//! What BS needs instead is `str(number)` itself, byte for byte — see
//! [`py_repr_float`] and [`py_str_decimal`]. Both are reproduced here rather
//! than approximated, because BS's parse is exact-match brittle: a `repr` that
//! tips into exponent notation feeds `"e"` to `int()` and raises ValueError
//! (bug 9), and a `repr` that loses its `".0"` silently skips the whole
//! `zarez` segment.
//!
//! `self.precision` is never read on this path, so the `precision=` kwarg is a
//! **no-op** for BS — confirmed live: `precision=0/1/3/5` all return
//! `num2words(1.005, lang="bs")`'s plain `"jedan zarez nula nula pet"`.
//! `precision_override` is therefore accepted and ignored.
//!
//! `to_year(val, longval=True)` ignores `longval` entirely and just returns
//! `self.to_cardinal(val)`, which is exactly what `Num2Word_Base.to_year`
//! already does — so the trait default is left in place here (it dispatches
//! through `&self` and picks up the `to_cardinal` override below).
//!
//! No cross-call mutable state: every method is a pure function of its
//! argument. Nothing to teach the dispatcher to skip.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong Bosnian but
//! are exactly what Python emits, and each is confirmed against the frozen
//! corpus:
//!
//! 1. *(Fixed, #216.)* Python built 11..=19 as `ones[n - 10] + "aest"`,
//!    emitting non-words like "dvaaest" (12) and "četiriaest" (14). The port
//!    now uses the standard `-naest` teens from [`TEENS`] (jedanaest,
//!    dvanaest, trinaest, četrnaest, …, devetnaest), as `lang_hr.rs` does.
//!    The bug number is kept so the numbering below stays stable.
//! 2. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's final
//!    `else` of `_int_to_word` is `return str(number)`, so `to_cardinal(10**9)`
//!    was the *string* `"1000000000"` — even though `self.scale` lists
//!    `milijarda` and `bilion`, which nothing ever read. This port continues
//!    the scale with milijarda (10^9), bilion (10^12), bilijarda (10^15),
//!    trilion (10^18) and trilijarda (10^21) — the -ijarda words also appear
//!    in `lang_hr.rs`'s table — and raises `OverflowError` from 10^24
//!    (`maxval`). The feminine -ijarda words agree like `hiljada` and the
//!    masculine -ilion words like `milion`, bug 3's quirks included:
//!    10^9 == "milijarda", 2*10^9 == "dva milijarde", 10^12 == "bilion".
//! 3. **`hiljada` never agrees for 1 or for compound thousands.** Exactly
//!    1000 renders as bare "hiljada" with no leading "jedan" (real Bosnian:
//!    "hiljadu"), and the 2/3/4 -> "hiljade" rule keys off the *whole*
//!    thousands count rather than its last digit, so 21000 becomes
//!    "dvadeset jedan hiljada" (correct: "dvadeset jedna hiljada"). Likewise
//!    10^6 is bare "milion" with no "jedan".
//! 4. **A redundant branch.** The `hundreds_val == 3 or hundreds_val == 4`
//!    arm and the `else` arm compute the identical expression
//!    (`self.ones[hundreds_val] + "sto"`). Collapsed below — the two arms are
//!    textually the same expression, so this cannot change output — but noted
//!    here so a reader diffing against Python does not think a case was lost.
//! 5. **`to_ordinal` was barely an ordinal.** Only 1/2/3 got real ordinal
//!    words ("prvi"/"drugi"/"treći"); every other value was the cardinal with
//!    a "." glued on ("dvanaest.", "nula.", "minus jedan."). Fixed (#259) as
//!    in `lang_hr.rs`: "nulti", "dvanaesti", "dvadeset prvi", "dvjestoti",
//!    "hiljaditi", "milioniti", "minus treći". Other round thousands and
//!    millions ("dva hiljade.", "dva miliona.") still take the old "."
//!    fallback, as does the float path ("pet zarez nula.").
//! 6. **`to_currency` silently accepts any currency code; `to_cheque` does
//!    not.** `to_currency` does `CURRENCY_FORMS.get(currency,
//!    CURRENCY_FORMS["BAM"])`, so an unknown code is *not* an error — it is
//!    quietly billed in convertible marks. `num2words(2, lang="bs",
//!    to="currency", currency="JPY")` is "dva marke". The corpus pins this for
//!    GBP/JPY/KWD/BHD/INR/CNY/CHF, all of which come back as marks/feninga
//!    while `to_cheque` on the same codes raises `NotImplementedError`.
//! 7. **The unit form never agrees for 1.** `left == 1` takes `cr1[0]`, which
//!    is the bare nominative, so BAM gives "jedan marka" (correct Bosnian:
//!    "jedna marka" — *marka* is feminine). EUR/USD are masculine and land
//!    right by luck ("jedan euro", "jedan dolar").
//! 8. **Cents are truncated to two digits, not rounded, and `cents=False`
//!    drops them.** `int(parts[1][:2].ljust(2, "0"))` slices the *string*, so
//!    1.999 is "jedan euro devedeset devet centi" (99c, not a rounded 100).
//!    And `if cents and right` means `cents=False` omits the subunit segment
//!    entirely rather than rendering it tersely as `Num2Word_Base` would.
//! 9. **Any value whose `str()` uses exponent notation raises ValueError.**
//!    `to_cardinal` feeds the pieces of `str(number)` to `int()` with no
//!    guard, so as soon as `repr` switches to scientific form the parse blows
//!    up on the letter. Confirmed live, and note the *two distinct* messages
//!    depending on whether a "." survives to split on:
//!      * `1e16`, `1e-05`, `1e+100`, `1e-300`, `Decimal("1E+2")`,
//!        `Decimal("1E-7")` — no ".", so `int(n)` sees the whole literal:
//!        `ValueError: invalid literal for int() with base 10: '1e+16'`.
//!      * `1.5e-05`, `1.5e+16`, `Decimal("1.5E+3")` — the "." splits, `int(left)`
//!        succeeds, then the `for digit in right` loop hits the exponent
//!        letter: `ValueError: invalid literal for int() with base 10: 'e'`.
//!    `float("nan")`/`float("inf")`/`float("-inf")` land in the first shape
//!    ('nan' / 'inf' / 'inf' — the sign is stripped before `int()` sees it),
//!    as do `Decimal("NaN")`/`Decimal("Infinity")`. This is BS's *lower* and
//!    *upper* de facto ceiling on the float path. Reproduced in [`py_int`];
//!    only a `Decimal` whose whole part reaches 10^24 hits the `maxval`
//!    OverflowError first (bug 2, #147).
//!
//! # Currency (phase 2)
//!
//! `Num2Word_BS` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base`'s currency machinery:
//!
//!   * It never calls `pluralize`, `_money_verbose`, `_cents_verbose` or
//!     `_cents_terse` — the Slavic 1 / 2-4 / else form choice is open-coded
//!     twice, inline, once for the unit and once for the subunit.
//!   * It never reads `CURRENCY_PRECISION` (which is `{}` anyway, so every
//!     code is precision 100) nor `CURRENCY_ADJECTIVES` (also `{}`), so the
//!     `adjective=` argument is **silently ignored** and 3-decimal (KWD/BHD)
//!     and 0-decimal (JPY) currencies get the plain 2-decimal treatment.
//!   * It does not go anywhere near `parse_currency_parts`: it re-stringifies
//!     the value with `str(val)` and slices the decimal point out by hand.
//!
//! `CURRENCY_FORMS` is BS's own class dict — three codes, each with a full
//! three-form tuple. It is *not* the dict `Num2Word_EN.__init__` mutates (see
//! `PORTING_CURRENCY.md`); the live interpreter confirms BS sees exactly
//! `{BAM, EUR, USD}` and empty `CURRENCY_PRECISION`/`CURRENCY_ADJECTIVES`.
//!
//! `to_cheque` is **not** overridden, so it comes from `Num2Word_Base` via the
//! trait default: it indexes `CURRENCY_FORMS[currency]` directly and raises
//! `NotImplementedError` on a miss. That is why the two lookups differ and
//! must stay separate — see `LangBs::currency_forms` and bug 6.
//!
//! # Error variants
//!
//! The four integer modes raise only `OverflowError`, for `abs(n) >= 10^24`
//! (bug 2, #147): there is no table lookup that can miss and no `int()` of a
//! parsed token. `to_currency` shares that ceiling — its `.get(..., BAM)`
//! fallback cannot miss. The only error on this surface is the `NotImplementedError`
//! that `Num2Word_Base.to_cheque` raises for a code outside `CURRENCY_FORMS`,
//! and that is produced by `currency::default_to_cheque` from the `None` that
//! `LangBs::currency_forms` returns.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.negword`. The trailing space is significant: `to_cardinal` does a
/// bare `ret + word` concatenation with no separator and relies on it. The
/// final `.strip()` is what keeps it from leaking when the word is empty.
const NEGWORD: &str = "minus ";

/// Ordinal words for 0..=20, the round tens, 100, 1000 and 10^6 (#259), as
/// in `lang_hr.rs` but with the Bosnian "hiljaditi", "milioniti".
fn ordinal_word(n: u64) -> Option<&'static str> {
    Some(match n {
        0 => "nulti",
        1 => "prvi",
        2 => "drugi",
        3 => "treći",
        4 => "četvrti",
        5 => "peti",
        6 => "šesti",
        7 => "sedmi",
        8 => "osmi",
        9 => "deveti",
        10 => "deseti",
        11 => "jedanaesti",
        12 => "dvanaesti",
        13 => "trinaesti",
        14 => "četrnaesti",
        15 => "petnaesti",
        16 => "šesnaesti",
        17 => "sedamnaesti",
        18 => "osamnaesti",
        19 => "devetnaesti",
        20 => "dvadeseti",
        30 => "trideseti",
        40 => "četrdeseti",
        50 => "pedeseti",
        60 => "šezdeseti",
        70 => "sedamdeseti",
        80 => "osamdeseti",
        90 => "devedeseti",
        100 => "stoti",
        1000 => "hiljaditi",
        1_000_000 => "milioniti",
        _ => return None,
    })
}

/// Ordinal hundreds 100..=900 (#259). Index 0 is unused.
const HUNDREDS_ORD: [&str; 10] = [
    "", "stoti", "dvjestoti", "tristoti", "četiristoti", "petstoti", "šeststoti", "sedamstoti",
    "osamstoti", "devetstoti",
];

/// `self.pointword`. `to_cardinal` splices it in raw (`+ " " + self.pointword
/// + " "`), *not* through `self.title(...)` the way
/// `Num2Word_Base.to_cardinal_float` does — BS never reaches that method. The
/// distinction is invisible here only because `is_title()` is false for BS.
const POINTWORD: &str = "zarez";

/// `self.ones`. Index 0 is `""` and is never indexed by the in-scope paths
/// (every caller guards with a non-zero digit first).
const ONES: [&str; 10] = [
    "", "jedan", "dva", "tri", "četiri", "pet", "šest", "sedam", "osam", "devet",
];

/// 11..=19. Python glued `"aest"` onto `self.ones` (bug 1, fixed in #216);
/// these are the standard Bosnian teens.
const TEENS: [&str; 9] = [
    "jedanaest",
    "dvanaest",
    "trinaest",
    "četrnaest",
    "petnaest",
    "šesnaest",
    "sedamnaest",
    "osamnaest",
    "devetnaest",
];

/// `self.tens`. Index 0 is `""`; index 1 ("deset") doubles as the word for a
/// bare 10, which `_int_to_word` special-cases before the teens branch.
const TENS: [&str; 10] = [
    "",
    "deset",
    "dvadeset",
    "trideset",
    "četrdeset",
    "pedeset",
    "šezdeset",
    "sedamdeset",
    "osamdeset",
    "devedeset",
];

/// 10^9 — where Python's `_int_to_word` gave up and returned digits, and
/// where [`HIGH_SCALES`] take over (#147).
const FALLBACK_THRESHOLD: u64 = 1_000_000_000;

/// The scales above `milion` that Python lacks (gladiaio/num2words2#147),
/// largest first, as `(power of ten, singular, paucal, plural)`. Bosnian
/// long scale, per Radio Sarajevo's table of large numbers
/// (radiosarajevo.ba/…/371905), bs.wikipedia "Milijarda"/"Bilion" and the
/// -ijarda rows of `lang_hr.rs`. Feminine -ijarda words take
/// `hiljada`'s 1 / 2-4 / 5+ forms; masculine -ilion words take `milion`'s
/// bare-1 / `-a` forms (bug 3's whole-count test included).
const HIGH_SCALES: [(u32, &str, &str, &str); 5] = [
    (21, "trilijarda", "trilijarde", "trilijardi"),
    (18, "trilion", "triliona", "triliona"),
    (15, "bilijarda", "bilijarde", "bilijardi"),
    (12, "bilion", "biliona", "biliona"),
    (9, "milijarda", "milijarde", "milijardi"),
];

/// The exclusive ceiling: 10^24 would need a kvadrilion, which the sources
/// above do not agree on (#147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(24))
}

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_BS.to_currency` declares `separator=" "` in its own signature,
/// but the `Lang` trait takes the separator as a plain argument, and both
/// `num2words2/__init__.py`'s Rust fast path and `bench/diff_test.py`
/// substitute `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default,
/// not BS's — before the value ever reaches Rust. By the time we see it,
/// "caller omitted separator" and "caller asked for a comma" are the same
/// string and the information is gone.
///
/// Every currency row in the frozen corpus is generated by
/// `num2words(v, lang="bs", to="currency", currency=c)` with no `separator=`,
/// so Python renders them with BS's " " ("dvanaest eura trideset četiri
/// centa"), while `diff_test.py` feeds this core a ",". Mapping "," back to
/// " " restores BS's default and reproduces the corpus. The one input it gets
/// wrong is an *explicit* `separator=","`, which Python renders as
/// "dvanaest eura,trideset četiri centa" (no space — BS concatenates the
/// separator raw) and which we render as " ". Fixing that properly means
/// teaching the shim to pass the converter's own default; that is out of
/// scope here (`__init__.py` is off-limits and shared by ~150 languages), so
/// the narrower divergence is the deliberate choice. `lang_ca.rs` and
/// `lang_es.rs` hit the identical trap and resolve it the same way.
const SEPARATOR_UNSET: &str = ",";

/// `Num2Word_BS.CURRENCY_FORMS`'s fallback key. `to_currency` does
/// `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["BAM"])`, so BAM is
/// what every unrecognised code silently becomes — see bug 6.
const FALLBACK_CURRENCY: &str = "BAM";

// ---- str(number): the only thing BS's float path looks at ---------------

/// Python's `int(s)`, and the `ValueError` it raises when `s` is not a
/// numeral — bug 9's engine.
///
/// The message is CPython's verbatim: `invalid literal for int() with base
/// 10: '<s>'`, quoting the *original* argument (not the whitespace-trimmed
/// one). `repr()` of a str would switch to double quotes if `s` contained an
/// apostrophe; every `s` reachable from a `repr(float)` or `str(Decimal)` is
/// drawn from `[0-9eE+.-]` or is one of `nan`/`inf`/`NaN`/`Infinity`, so the
/// plain single-quote form is exact here.
///
/// `int()` tolerates surrounding whitespace, hence the `trim()`; it also
/// tolerates `_` separators and non-ASCII decimal digits, which `BigInt`'s
/// parser does not. Neither is reachable — nothing upstream of this can
/// produce a string BS did not just generate itself — so the divergence is
/// noted rather than coded around.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s.trim()).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Drop trailing zeros from `q`, compensating in `e`, so that `q * 10^e` is
/// unchanged but `q` is the canonical (shortest) coefficient.
///
/// Needed because rounding can carry: rounding `999` to one significant digit
/// yields `q = 10`, which denotes the same value as `q = 1` one exponent up.
fn strip_trailing_zeros(mut q: BigInt, mut e: i32) -> (BigInt, i32) {
    let ten = BigInt::from(10u32);
    while !q.is_zero() {
        let (div, rem) = q.div_rem(&ten);
        if !rem.is_zero() {
            break;
        }
        q = div;
        e += 1;
    }
    (q, e)
}

/// The shortest round-trip decimal digits of a finite `f`, as `(negative,
/// digits, decpt)` with `f == ±0.<digits> * 10^decpt`.
///
/// # Why this is not `format!("{:e}", f)`
///
/// Rust's `{}` / `{:e}` and CPython's `_Py_dg_dtoa(f, mode=0, ...)` — what
/// `repr` calls — both produce *a* shortest digit string that round-trips, but
/// **not the same one**. When two equally short candidates both round-trip,
/// Gay's dtoa returns the one nearest the true value, breaking an exact tie
/// half-to-even; Rust's `flt2dec` explicitly does not promise the nearest.
/// They diverge on real inputs:
///
/// | f (exact value)              | Python repr         | Rust `{}`           |
/// |------------------------------|---------------------|---------------------|
/// | `953749603507345.25`         | `953749603507345.2` | `953749603507345.3` |
/// | `14325669370808.5625`        | `14325669370808.562`| `14325669370808.563`|
///
/// Both of those are exact ties, and dtoa rounds to the even digit. A 40k-case
/// differential fuzz against the live interpreter hits this about 1 time in
/// 5000 — often enough to matter, rarely enough to sail through any hand-
/// written test. BS reads these digits out one word at a time, so a wrong last
/// digit is a wrong word ("jedan tri" for "jedan dva").
///
/// # What this does instead
///
/// Reproduces dtoa mode 0 from first principles, in exact arithmetic:
///
///  1. Decompose `|f|` into its exact decimal expansion `c * 10^-k`. Every
///     finite double is exactly `mant * 2^exp2`, and `2^-k == 5^k * 10^-k`, so
///     the expansion is finite and `c` is a plain integer.
///  2. For `n` = 1, 2, ... 17, round `c` to `n` significant digits with
///     round-half-**even** (Python's rule, and `dtoa`'s) and test whether the
///     result parses back to `f`. Rust's `f64: FromStr` is correctly rounded,
///     as is CPython's `float()`, so the two agree on the test.
///  3. The first `n` that round-trips wins. That is the definition of dtoa
///     mode 0's output, ties included.
///
/// 17 always terminates the loop (every double round-trips at 17 significant
/// digits), and `n = c`'s own digit count would round exactly, so the fallback
/// is unreachable. The winning `q` can never end in `0`: if it did, `n - 1`
/// would have produced the identical value and been accepted first.
fn shortest_digits(f: f64) -> (bool, String, i32) {
    let neg = f.is_sign_negative();
    let a = f.abs();
    if a == 0.0 {
        // dtoa returns "0" with decpt 1; repr(-0.0) keeps the sign bit.
        return (neg, "0".to_string(), 1);
    }

    // |a| == mant * 2^exp2, exactly.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mant, exp2) = if biased == 0 {
        (frac, -1074) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };

    // |a| == c * 10^-k, exactly.
    let (c, k) = if exp2 >= 0 {
        (BigInt::from(mant) << (exp2 as usize), 0i32)
    } else {
        let k = (-exp2) as u32;
        (BigInt::from(mant) * BigInt::from(5u32).pow(k), k as i32)
    };
    let c_digits = c.to_string().len() as i32;

    for n in 1..=17i32 {
        let drop = c_digits - n;
        let (q, e) = if drop <= 0 {
            // Fewer digits than asked for: the rounding is exact.
            (c.clone(), -k)
        } else {
            let pow = BigInt::from(10u32).pow(drop as u32);
            let (mut q, rem) = c.div_rem(&pow);
            // Round half to even — the tie rule that makes 953749603507345.25
            // render as "...45.2" and not "...45.3".
            let two_rem = &rem * 2u32;
            if two_rem > pow || (two_rem == pow && q.is_odd()) {
                q += 1u32;
            }
            (q, drop - k)
        };
        let (q, e) = strip_trailing_zeros(q, e);

        // Round-trip test against the original double.
        if format!("{}e{}", q, e).parse::<f64>() == Ok(a) {
            let digits = q.to_string();
            let decpt = digits.len() as i32 + e;
            return (neg, digits, decpt);
        }
    }

    // Unreachable: 17 significant digits round-trip for every finite double.
    // Fall back on the exact expansion rather than panicking.
    let (q, e) = strip_trailing_zeros(c, -k);
    let digits = q.to_string();
    let decpt = digits.len() as i32 + e;
    (neg, digits, decpt)
}

/// Python's `repr(float)` — which on Python 3 is exactly `str(float)`, and so
/// is the whole of BS's float input.
///
/// Reproduces CPython's `format_float_short(v, 'r', ...)`:
///
/// ```c
/// case 'r':
///     /* convert to exponential format at 1e16. */
///     if (decpt <= -4 || decpt > 16)
///         use_exp = 1;
/// ```
///
/// plus `Py_DTSF_ADD_DOT_0`, which appends the `".0"` that keeps `repr` from
/// ever looking like an int — load-bearing for BS, because the `".0"` is the
/// only reason `1e15` renders as `"bilijarda zarez nula"` instead of
/// taking the dotless branch. The exponent is `"%+.02d"`: always signed, and
/// zero-padded to two digits (`1e-05`, not `1e-5`).
///
/// `nan` carries no sign in CPython (`repr(float("-nan")) == "nan"`);
/// `inf` does. Both are dead ends — see bug 9 — but the sign still decides
/// whether `to_cardinal` peels a `negword` off first.
fn py_repr_float(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    let (neg, digits, decpt) = shortest_digits(f);
    let sign = if neg { "-" } else { "" };
    let len = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        // Exponential form. No ".0" is added here: repr(1e16) is "1e+16".
        let exp = decpt - 1;
        let mut mant = String::new();
        let mut chars = digits.chars();
        mant.push(chars.next().expect("dtoa never returns an empty digit string"));
        let rest = chars.as_str();
        if !rest.is_empty() {
            mant.push('.');
            mant.push_str(rest);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.unsigned_abs()
        )
    } else if decpt <= 0 {
        // 0.0000<digits> — repr(0.5) == "0.5", repr(0.0001) == "0.0001".
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt < len {
        // <digits> with the point spliced in — repr(12.34) == "12.34".
        let cut = decpt as usize;
        format!("{}{}.{}", sign, &digits[..cut], &digits[cut..])
    } else {
        // Integral: pad right with zeros, then ADD_DOT_0. Shortest-repr digits
        // never carry trailing zeros, so this branch *is* "value is integral".
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - len) as usize)
        )
    }
}

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__` with the default
/// context (`capitals = 1`, hence the upper-case `E`), non-engineering.
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
/// `BigDecimal` stores `value == coeff * 10^-scale`, so `_exp` is `-scale` and
/// `_int` is `str(abs(coeff))` — the same correspondence
/// [`LangBs::split_value`] already relies on for currency, and the reason
/// `Decimal("1.10")` keeps its trailing zero ("jedan zarez jedan nula") rather
/// than normalising to `1.1`.
///
/// Unlike `repr(float)`'s `"%+.02d"`, Decimal's exponent is a bare `"%+d"` —
/// `1E+2`, not `1E+02`.
fn py_str_decimal(d: &BigDecimal) -> String {
    let (coeff, scale) = d.as_bigint_and_exponent();
    let sign = if coeff.is_negative() { "-" } else { "" };
    // `self._int`: the coefficient's digits, unsigned. BigInt renders 0 as the
    // single "0" that Decimal's digits tuple also carries for a zero.
    let int_digits = coeff.abs().to_string();
    let len = int_digits.len() as i64;
    let exp = -scale;
    let leftdigits = exp + len;

    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    // Both branches below are bounded: in the plain arm `exp <= 0` forces
    // `dotplace <= len` and `-dotplace < 6`; in the scientific arm `dotplace`
    // is 1. So neither `repeat` can be handed a large count, however extreme
    // the scale.
    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let cut = dotplace as usize;
        (
            int_digits[..cut].to_string(),
            format!(".{}", &int_digits[cut..]),
        )
    };

    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        // Rust's `{:+}` on an integer is Python's "%+d".
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}

pub struct LangBs {
    /// `Num2Word_BS.CURRENCY_FORMS`, built once here rather than per call.
    ///
    /// BS's own class dict, untouched by `Num2Word_EN.__init__`'s mutation of
    /// the `Num2Word_EUR` dict — so exactly three codes, no AUD/JPY/KWD/…
    /// leakage. Each entry keeps all **three** forms: `to_currency` indexes
    /// `cr1[2]` / `cr2[2]` on the "else" arm, so dropping the third would
    /// panic rather than merely reword.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `self.CURRENCY_FORMS["BAM"]`, kept alongside the map purely so the
    /// `.get(currency, <default>)` fallback in `to_currency` is a borrow with
    /// no panic path. Python evaluates that default eagerly on every call and
    /// would `KeyError` if BAM were absent; here it cannot be.
    bam: CurrencyForms,
}

impl Default for LangBs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangBs {
    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "BAM": (("marka", "marke", "maraka"),
        //             ("feninga", "feninga", "feninga")),
        //     "EUR": (("euro", "eura", "eura"), ("cent", "centa", "centi")),
        //     "USD": (("dolar", "dolara", "dolara"), ("cent", "centa", "centi")),
        // }
        // Verified against the live interpreter, not the source literal.
        let bam = CurrencyForms::new(
            &["marka", "marke", "maraka"],
            &["feninga", "feninga", "feninga"],
        );
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (FALLBACK_CURRENCY, bam.clone()),
            (
                "EUR",
                CurrencyForms::new(&["euro", "eura", "eura"], &["cent", "centa", "centi"]),
            ),
            (
                "USD",
                CurrencyForms::new(&["dolar", "dolara", "dolara"], &["cent", "centa", "centi"]),
            ),
        ]
        .into_iter()
        .collect();

        LangBs { currency_forms, bam }
    }

    /// The Slavic count-agreement selector `to_currency` open-codes twice.
    ///
    /// ```python
    /// if left == 1:
    ///     currency_word = cr1[0]
    /// elif left % 10 in [2, 3, 4] and left % 100 not in [12, 13, 14]:
    ///     currency_word = cr1[1]
    /// else:
    ///     currency_word = cr1[2]
    /// ```
    ///
    /// Identical for `right`/`cr2`. `n` is always non-negative at both call
    /// sites (`to_currency` has already applied `abs`), so `mod_floor` and
    /// Python's `%` agree and the `to_u32` conversions cannot fail — the
    /// residues are bounded by 10 and 100 respectively.
    ///
    /// Indexing `forms[0..=2]` directly is sound because every entry in
    /// `LangBs::currency_forms` is built here with exactly three forms and
    /// the table is never mutated. (Python would `IndexError` on a 2-form
    /// entry; BS has none.)
    fn select_form<'a>(n: &BigInt, forms: &'a [String]) -> &'a str {
        if n == &BigInt::from(1u32) {
            return forms[0].as_str();
        }
        let m10 = n.mod_floor(&BigInt::from(10u32)).to_u32().unwrap_or(0);
        let m100 = n.mod_floor(&BigInt::from(100u32)).to_u32().unwrap_or(0);
        if matches!(m10, 2 | 3 | 4) && !matches!(m100, 12 | 13 | 14) {
            forms[1].as_str()
        } else {
            forms[2].as_str()
        }
    }

    /// Port of `to_currency`'s hand-rolled value decomposition.
    ///
    /// ```python
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Returns `(left, right, is_negative)` with `right` in `0..=99`.
    ///
    /// This is *not* `parse_currency_parts` — BS never calls it. Notably the
    /// `Int`/`Decimal` split does **no** work here beyond "an int's `str()`
    /// has no dot": BS skips the cents segment on `if cents and right`, not on
    /// `isinstance(val, int)`. That is why `1.0` prints "jedan euro" with no
    /// cents even though it is a float — `str(1.0)` is "1.0", `parts[1]` is
    /// the truthy `"0"`, and `right` computes to a falsy 0. The two paths
    /// still must not be collapsed: `Int` must never grow a fraction.
    ///
    /// `parts[0]` is never empty for either variant (`str` always emits a
    /// leading "0"), so Python's `if parts[0] else 0` guard is unreachable.
    fn split_value(val: &CurrencyValue) -> (BigInt, u32, bool) {
        match val {
            // str(int) has no ".", so parts == [digits] and right stays 0.
            CurrencyValue::Int(v) => (v.abs(), 0, v.is_negative()),
            CurrencyValue::Decimal { value: d, .. } => {
                let is_negative = d.is_negative();
                // `value == digits * 10^-scale`, with `digits`/`scale` taken
                // verbatim from the `str(value)` the shim parsed — so
                // reconstructing the plain-notation string from them
                // reproduces Python's `str(abs(val))` exactly. `abs` before
                // `str` is just the sign of `digits`.
                let (digits, scale) = d.as_bigint_and_exponent();
                let digits = digits.abs();

                if scale <= 0 {
                    // No "." in the string: the value is integral as written.
                    // scale == 0 is the ordinary case (Decimal("100")).
                    // scale < 0 means exponent notation — see the module
                    // concerns; Python takes a different (broken) route there.
                    let pow = BigInt::from(10u32).pow((-scale) as u32);
                    return (digits * pow, 0, is_negative);
                }

                let pow = BigInt::from(10u32).pow(scale as u32);
                let (left, frac) = digits.div_rem(&pow);
                // `parts[1]` is `frac` zero-padded to `scale` digits;
                // `int(parts[1][:2].ljust(2, "0"))` is then exactly "the first
                // two fractional digits as a 2-digit number", i.e. floor
                // division that keeps leading zeros and pads short fractions
                // on the right. 0.5 -> frac 5, scale 1 -> 5*100/10 = 50.
                // 1.999 -> frac 999, scale 3 -> 999*100/1000 = 99 (bug 8).
                // 0.01 -> frac 1, scale 2 -> 1*100/100 = 1.
                let right = (frac * BigInt::from(100u32) / &pow).to_u32().unwrap_or(0);
                (left, right, is_negative)
            }
        }
    }

    /// The body of `Num2Word_BS.to_cardinal`, driven off `str(number)`.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword          # "minus " — trailing space
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// Only the float/Decimal path routes through here: `Lang::to_cardinal`
    /// takes a `&BigInt`, whose `str()` can never contain a ".", so it
    /// short-circuits to the `else` branch and keeps its `abs()` shortcut.
    ///
    /// Three details that look incidental and are not:
    ///
    ///   * `startswith("-")` strips exactly **one** sign, and it is stripped
    ///     from the *string*. `_int_to_word` therefore only ever sees a
    ///     non-negative `left`, so its own `negword` arm stays unreachable and
    ///     "minus " is never doubled.
    ///   * The trailing `.strip()` is what removes the space the digit loop
    ///     leaves behind. It also trims `negword`'s trailing space — but only
    ///     if nothing follows it, which cannot happen.
    ///   * `int(left)` is evaluated before the loop, so on a string like
    ///     "1.5e-05" the ValueError names `'e'`, not the whole literal.
    ///     Ordering is load-bearing for bug 9's two message shapes.
    fn cardinal_from_str(&self, s: &str) -> Result<String> {
        let n = s.trim();
        let (n, mut ret) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD.to_string()),
            None => (n, String::new()),
        };

        match n.split_once('.') {
            // `"." in n` -> `n.split(".", 1)`, which yields exactly two parts.
            Some((left, right)) => {
                ret.push_str(&self.int_to_word(&py_int(left)?)?);
                ret.push(' ');
                ret.push_str(POINTWORD);
                ret.push(' ');
                // `for digit in right` iterates *characters*.
                for digit in right.chars() {
                    ret.push_str(&self.int_to_word(&py_int(&digit.to_string())?)?);
                    ret.push(' ');
                }
                Ok(ret.trim().to_string())
            }
            None => {
                ret.push_str(&self.int_to_word(&py_int(n)?)?);
                Ok(ret.trim().to_string())
            }
        }
    }

    /// Port of `Num2Word_BS._int_to_word`, the BigInt-level dispatch.
    ///
    /// Python's branch order is `== 0`, `< 0`, `< 10`, `== 10`, `< 20`,
    /// `< 100`, `< 1000`, `< 1000000`, `< 1000000000`, else `str(number)`.
    /// Everything from `< 10` through `< 1000000000` is bounded by 10^9 and so
    /// fits a `u64`; that whole span is delegated to [`Self::int_to_word_small`]
    /// after this function has peeled off zero and the sign. Python's
    /// `str(number)` fallback is replaced by the [`HIGH_SCALES`] and the
    /// `maxval` check (bug 2, #147).
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_zero() {
            return Ok("nula".to_string());
        }

        if number.is_negative() {
            // Python: `return self.negword + self._int_to_word(abs(number))`.
            //
            // Dead code for all four in-scope modes: `to_cardinal` strips the
            // "-" from the *string* before calling `int()`, so `_int_to_word`
            // only ever sees a non-negative value. Reached in Python only via
            // `to_currency`, which is out of scope. Ported anyway for fidelity;
            // note it would yield a doubled "minus " if it ever were reached
            // through `to_cardinal`.
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&number.abs())?));
        }

        if let Some(n) = number.to_u64() {
            // Bounded by the `< 1000000000` guard, so `to_u64` cannot fail
            // for any value this arm accepts.
            if n < FALLBACK_THRESHOLD {
                return Ok(self.int_to_word_small(n));
            }
        }
        // Python: `else: return str(number)` — bug 2. The scales above
        // milion instead (#147); the maxval check keeps the count below 1000.
        for &(exp, one, few, many) in HIGH_SCALES.iter() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let (count, remainder) = number.div_mod_floor(&scale);
                let mut result = if count.is_one() {
                    one.to_string()
                } else if count < BigInt::from(5) {
                    format!("{} {}", self.int_to_word(&count)?, few)
                } else {
                    format!("{} {}", self.int_to_word(&count)?, many)
                };
                if !remainder.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&remainder)?);
                }
                return Ok(result);
            }
        }
        unreachable!("every value >= 10^9 and below maxval has a HIGH_SCALES entry")
    }

    /// The `0 < number < 10^9` span of `_int_to_word`.
    ///
    /// Every recursive call passes a strictly smaller non-negative remainder,
    /// so the `< 0` and `>= 10^9` arms of [`Self::int_to_word`] are
    /// unreachable from here and recursion terminates.
    fn int_to_word_small(&self, number: u64) -> String {
        // `number == 0` and `number < 0` are handled by the caller.
        if number < 10 {
            return ONES[number as usize].to_string();
        }

        if number == 10 {
            return TENS[1].to_string();
        }

        if number < 20 {
            // Bug 1 (fixed, #216): Python's `ones[n - 10] + "aest"` gave
            // "dvaaest"; use the real teens.
            return TEENS[(number - 11) as usize].to_string();
        }

        if number < 100 {
            let tens_val = (number / 10) as usize;
            let ones_val = (number % 10) as usize;
            return if ones_val == 0 {
                TENS[tens_val].to_string()
            } else {
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            };
        }

        if number < 1000 {
            let hundreds_val = (number / 100) as usize;
            let remainder = number % 100;
            let mut result = match hundreds_val {
                1 => "sto".to_string(),
                2 => "dvjesto".to_string(),
                // Bug 4: Python spells out a `3 or 4` arm and an `else` arm
                // that are the same expression. Collapsed; output identical.
                _ => format!("{}sto", ONES[hundreds_val]),
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            return result;
        }

        if number < 1_000_000 {
            let thousands_val = number / 1000;
            let remainder = number % 1000;
            // Bug 3: bare "hiljada" for 1 (no "jedan"), and the 2/3/4 test
            // looks at the whole count, not its last digit.
            let mut result = if thousands_val == 1 {
                "hiljada".to_string()
            } else if thousands_val < 5 {
                format!("{} hiljade", self.int_to_word_small(thousands_val))
            } else {
                format!("{} hiljada", self.int_to_word_small(thousands_val))
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            return result;
        }

        // number < 1_000_000_000, guaranteed by the caller.
        let millions_val = number / 1_000_000;
        let remainder = number % 1_000_000;
        // Bug 3 again: bare "milion" for 1, and "miliona" for every other
        // count (so 2 gives "dva miliona", correct here only by luck).
        let mut result = if millions_val == 1 {
            "milion".to_string()
        } else {
            format!("{} miliona", self.int_to_word_small(millions_val))
        };
        if remainder != 0 {
            result.push(' ');
            result.push_str(&self.int_to_word_small(remainder));
        }
        result
    }
}

impl Lang for LangBs {
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

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` compares the raw
    /// value against 1/2/3 *before* any stringification — `1.0 == 1` and
    /// `Decimal("1.00") == 1` are both True, so those get the real ordinal
    /// words — and everything else is the (float-grammar) cardinal plus ".":
    /// `to_ordinal(5.0)` == "pet zarez nula.", `to_ordinal(-1.5)` ==
    /// "minus jedan zarez pet.". No verify_ordinal guard anywhere.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i == BigInt::from(1) {
                return Ok("prvi".to_string());
            }
            if i == BigInt::from(2) {
                return Ok("drugi".to_string());
            }
            if i == BigInt::from(3) {
                return Ok("treći".to_string());
            }
        }
        Ok(format!("{}.", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr
    /// verbatim, sign and trailing zeros included: `-0.0` → "-0.0.",
    /// `Decimal("5.00")` → "5.00.", `1e16` → "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, except
    /// that an Infinity parse is surfaced as the ValueError BS's own
    /// `to_cardinal` raises one step later: `str(Decimal("Infinity"))` has no
    /// "." and `int("Infinity")` chokes on the literal. The dispatcher's
    /// default maps `ParsedNumber::Inf` to base's OverflowError, which BS can
    /// never raise. NaN keeps the default routing (ValueError either way).
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
        "BAM"
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
        "zarez"
    }

    /// Port of `Num2Word_BS.to_cardinal`, integer path only.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword          # "minus " — trailing space
    /// else:
    ///     ret = ""
    /// if "." in n: ...                # float path, out of scope
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// `str(int)` never contains a ".", so integers always take the `else`
    /// branch. Stripping the leading "-" from the string and re-parsing is
    /// exactly `abs()` for an integer, so `value.abs()` is used instead.
    ///
    /// The trailing `.strip()` is load-bearing only in that it trims the
    /// trailing space of `negword` when `_int_to_word` returns "" — which it
    /// never does for a non-negative input. It is kept for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_BS.to_ordinal`, with real ordinal words (#259).
    ///
    /// Python had words for 1/2/3 only and glued "." onto every other
    /// cardinal ("dvanaest."). As in `lang_hr.rs`, the table covers
    /// 0..=20, the round tens, 100 and 1000, and a compound inflects its
    /// last word: "dvadeset prvi", "sto dvanaesti", "dvjestoti". See bug 5
    /// for the round values still on the old fallback.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.to_ordinal(&value.abs())?));
        }
        if let Some(word) = value.to_u64().and_then(ordinal_word) {
            return Ok(word.to_string());
        }
        let cardinal = self.to_cardinal(value)?;
        if let Some(n) = value.to_u64() {
            if let Some(word) = crate::compound_ordinal::last_word_ordinal(
                n,
                &cardinal,
                ordinal_word,
                Some(&HUNDREDS_ORD),
            ) {
                return Ok(word);
            }
        }
        Ok(format!("{}.", cardinal))
    }

    /// Port of `Num2Word_BS.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Note this overrides `Num2Word_Base.to_ordinal_num` (which returns the
    /// value untouched), and that the minus sign survives: -1 -> "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// The float/Decimal cardinal path.
    ///
    /// **BS has no `to_cardinal_float`.** It does not override the method, and
    /// it does not inherit its way into it either: `num2words()` calls
    /// `converter.to_cardinal(number)` with the `float`/`Decimal` still
    /// intact, and `Num2Word_BS.to_cardinal` — which does *not* chain up to
    /// `Num2Word_Base.to_cardinal` — stringifies it on the first line. The
    /// `assert int(value) == value` / `except -> self.to_cardinal_float(value)`
    /// hand-off in the base class is simply never executed for BS.
    ///
    /// This hook exists because Rust's `Lang::to_cardinal` is typed
    /// `&BigInt` and so cannot carry a non-integer. Overriding it here — with
    /// the *same* string body the integer path uses — reunites the two halves
    /// of the one Python method. `floatpath::default_to_cardinal_float` is
    /// deliberately not called: it would run `float2tuple`, which BS never
    /// does.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is accepted
    /// and **ignored**. `num2words()` applies it by assigning
    /// `converter.precision`, and BS reads that attribute nowhere: its
    /// fractional digit count is `len(str(number).split(".")[1])`, fixed by
    /// `repr`. Verified live — `precision=0/1/3/5` on `1.005` all return
    /// "jedan zarez nula nula pet".
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `str(number)`. The `precision` FloatValue carries is likewise unused:
        // it is derived from this same repr on the Python side, and BS reads
        // the repr directly.
        let s = match value {
            FloatValue::Float { value, .. } => py_repr_float(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        self.cardinal_from_str(&s)
    }

    // to_year: `Num2Word_BS.to_year(val, longval=True)` ignores `longval` and
    // returns `self.to_cardinal(val)` — identical to `Num2Word_Base.to_year`.
    // The trait default already delegates to to_cardinal through &self, so it
    // is correct as-is.

    // ---- currency ------------------------------------------------------

    /// `self.__class__.__name__`, for `Num2Word_Base.to_cheque`'s
    /// `'Currency code "%s" not implemented for "%s"'`.
    fn lang_name(&self) -> &str {
        "Num2Word_BS"
    }

    /// `self.CURRENCY_FORMS[code]` — the **strict** lookup.
    ///
    /// This hook models Python's *subscript*, which is what
    /// `Num2Word_Base.to_cheque` uses (`self.CURRENCY_FORMS[currency]` inside
    /// a `try/except KeyError`). Returning `None` for an unknown code is what
    /// makes `currency::default_to_cheque` raise the `NotImplementedError`
    /// the corpus pins for cheque:GBP/JPY/KWD/BHD/INR/CNY/CHF.
    ///
    /// `LangBs::to_currency` deliberately does **not** call this: BS's own
    /// `to_currency` uses `.get(currency, self.CURRENCY_FORMS["BAM"])`, which
    /// never fails (bug 6). The asymmetry is Python's, not ours.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // currency_precision: `CURRENCY_PRECISION` is `{}` (confirmed live), so
    // `.get(code, 100)` is 100 for every code — the trait default. BS's own
    // to_currency ignores it entirely; only to_cheque reads it, and 100 is
    // what gives cheque's "56/100".
    //
    // currency_adjective: `CURRENCY_ADJECTIVES` is `{}` too, so the default
    // `None` is right. BS's to_currency never consults it in any case.
    //
    // pluralize / money_verbose / cents_verbose / cents_terse: BS's
    // to_currency open-codes its form choice and never calls any of them.
    // to_cheque does reach `_money_verbose`, whose base implementation is
    // `self.to_cardinal(number)` — exactly the trait default, which dispatches
    // through `&self` and so picks up BS's to_cardinal override. `pluralize`
    // stays at the default that raises, mirroring the abstract
    // `Num2Word_Base.pluralize`; nothing on this surface reaches it.
    //
    // to_cheque: not overridden by BS, so `currency::default_to_cheque` (the
    // port of `Num2Word_Base.to_cheque`) is correct as-is. Traced below.

    /// Port of `Num2Word_BS.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="BAM", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["BAM"])
    ///     left_str = self._int_to_word(left)
    ///     ...
    ///     result = left_str + " " + currency_word
    ///     if cents and right:
    ///         ...
    ///         result += separator + cents_str + " " + cents_word
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// `adjective` is accepted and dropped on the floor, exactly as Python
    /// does — `CURRENCY_ADJECTIVES` is never read on this path, so
    /// `adjective=True` changes nothing.
    ///
    /// `_int_to_word` is called with the already-`abs`'d `left`, so its
    /// negative arm stays unreachable here and "minus " is prepended exactly
    /// once, at the end.
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
        // Restore BS's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            " "
        } else {
            separator
        };

        let (left, right, is_negative) = Self::split_value(val);

        // `.get(currency, self.CURRENCY_FORMS["BAM"])` — bug 6. Note this
        // bypasses the `currency_forms` hook on purpose; that one is strict.
        let forms = self.currency_forms.get(currency).unwrap_or(&self.bam);

        let left_str = self.int_to_word(&left)?;
        let currency_word = Self::select_form(&left, &forms.unit);
        let mut result = format!("{} {}", left_str, currency_word);

        // `if cents and right:` — a zero `right` suppresses the segment, which
        // is why the float 1.0 renders as bare "jedan euro" (bug 8 covers the
        // `cents=False` half of this branch).
        if cents && right != 0 {
            let right_big = BigInt::from(right);
            let cents_str = self.int_to_word(&right_big)?;
            let cents_word = Self::select_form(&right_big, &forms.subunit);
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(cents_word);
        }

        if is_negative {
            // `self.negword` — "minus ", trailing space included.
            result.insert_str(0, NEGWORD);
        }

        // `.strip()`. A no-op for every reachable input (`_int_to_word` never
        // returns "", so nothing pads the ends), kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn f(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn d(s: &str) -> FloatValue {
        // `precision` mirrors what the shim computes
        // (`abs(Decimal(str(value)).as_tuple().exponent)`); BS ignores it.
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        FloatValue::Decimal { value, precision }
    }

    fn go(v: &FloatValue) -> String {
        LangBs::new().to_cardinal_float(v, None).unwrap()
    }

    fn err(v: &FloatValue) -> String {
        match LangBs::new().to_cardinal_float(v, None) {
            Err(N2WError::Value(m)) => m,
            other => panic!("expected N2WError::Value, got {:?}", other.map(|s| s)),
        }
    }

    /// Every `"to": "cardinal"` row with a dot in `arg` that the frozen corpus
    /// records for bs, verbatim.
    #[test]
    fn corpus_cardinal_float_rows() {
        assert_eq!(go(&f(0.0, 1)), "nula zarez nula");
        assert_eq!(go(&f(0.5, 1)), "nula zarez pet");
        assert_eq!(go(&f(1.0, 1)), "jedan zarez nula");
        assert_eq!(go(&f(1.5, 1)), "jedan zarez pet");
        assert_eq!(go(&f(2.25, 2)), "dva zarez dva pet");
        assert_eq!(go(&f(3.14, 2)), "tri zarez jedan četiri");
        assert_eq!(go(&f(0.01, 2)), "nula zarez nula jedan");
        assert_eq!(go(&f(0.1, 1)), "nula zarez jedan");
        assert_eq!(go(&f(0.99, 2)), "nula zarez devet devet");
        assert_eq!(go(&f(1.01, 2)), "jedan zarez nula jedan");
        assert_eq!(go(&f(12.34, 2)), "dvanaest zarez tri četiri");
        assert_eq!(go(&f(99.99, 2)), "devedeset devet zarez devet devet");
        assert_eq!(go(&f(100.5, 1)), "sto zarez pet");
        assert_eq!(
            go(&f(1234.56, 2)),
            "hiljada dvjesto trideset četiri zarez pet šest"
        );
        assert_eq!(go(&f(-0.5, 1)), "minus nula zarez pet");
        assert_eq!(go(&f(-1.5, 1)), "minus jedan zarez pet");
        assert_eq!(go(&f(-12.34, 2)), "minus dvanaest zarez tri četiri");
        assert_eq!(go(&f(1.005, 3)), "jedan zarez nula nula pet");
        assert_eq!(go(&f(2.675, 3)), "dva zarez šest sedam pet");
    }

    /// Every `"to": "cardinal_dec"` row the frozen corpus records for bs.
    #[test]
    fn corpus_cardinal_dec_rows() {
        assert_eq!(go(&d("0.01")), "nula zarez nula jedan");
        // Trailing zero preserved: str(Decimal("1.10")) is "1.10", not "1.1".
        assert_eq!(go(&d("1.10")), "jedan zarez jedan nula");
        assert_eq!(go(&d("12.345")), "dvanaest zarez tri četiri pet");
        // Issue #603's value. The integer part clears 10^9, which Python
        // left numeric (bug 2); spelled out since #147.
        assert_eq!(
            go(&d("98746251323029.99")),
            "devedeset osam biliona sedamsto četrdeset šest milijardi dvjesto \
             pedeset jedan miliona tristo dvadeset tri hiljada dvadeset devet \
             zarez devet devet"
        );
        assert_eq!(go(&d("0.001")), "nula zarez nula nula jedan");
    }

    /// The f64-artefact pair. BS reaches them by a different road than
    /// `floatpath.rs` does — `str()`, not `abs(v-pre)*10**precision` — and
    /// arrives at the same place.
    #[test]
    fn float_artefact_cases_go_through_repr_not_float2tuple() {
        assert_eq!(py_repr_float(2.675), "2.675");
        assert_eq!(py_repr_float(1.005), "1.005");
        // The artefact float2tuple has to repair, for contrast: this is what
        // `abs(2.675 - 2.0) * 10f64.powi(3)` actually evaluates to.
        assert_eq!((2.675f64 - 2.0).abs() * 1000.0, 674.9999999999998);
        assert_eq!(go(&f(2.675, 3)), "dva zarez šest sedam pet");
    }

    /// `-0.0` keeps its sign through `repr`, so the negword survives even
    /// though the value is zero. BS keys off the *string* (`n.startswith("-")`),
    /// not a numeric `val < 0`, which is why a value equal to zero can still
    /// come out negative here — and why `to_currency`, which does compare
    /// numerically, does not have this behaviour.
    #[test]
    fn negative_zero_float() {
        assert_eq!(py_repr_float(-0.0), "-0.0");
        assert_eq!(go(&f(-0.0, 1)), "minus nula zarez nula");
    }

    /// The correctly-rounded-shortest cases that Rust's own float formatting
    /// gets "wrong" (differently). Each exact value below is an exact tie
    /// between two 16/17-digit candidates that both round-trip; CPython's dtoa
    /// breaks the tie to the even digit, `format!("{}", f)` does not.
    ///
    /// Found by differential fuzzing, not by inspection. Regression-pinned
    /// because switching `shortest_digits` back to `{:e}` still passes every
    /// other test in this module.
    #[test]
    fn shortest_digits_breaks_exact_ties_to_even_like_dtoa() {
        // exact: 953749603507345.25 -> "...45.2", not Rust's "...45.3"
        assert_eq!(py_repr_float(953749603507345.2), "953749603507345.2");
        assert_eq!(format!("{}", 953749603507345.2f64), "953749603507345.3");
        // exact: 14325669370808.5625 -> "...08.562", not "...08.563"
        assert_eq!(py_repr_float(14325669370808.562), "14325669370808.562");
        assert_eq!(format!("{}", 14325669370808.562f64), "14325669370808.563");
        // exact: -91334146377350.125 -> "...50.12", not "...50.13"
        assert_eq!(py_repr_float(-91334146377350.12), "-91334146377350.12");
        assert_eq!(format!("{}", -91334146377350.12f64), "-91334146377350.13");

        assert_eq!(
            go(&f(953749603507345.2, 1)),
            "devetsto pedeset tri biliona sedamsto četrdeset devet milijardi \
             šeststo tri miliona petsto sedam hiljada tristo četrdeset pet \
             zarez dva"
        );
        assert_eq!(
            go(&f(-91334146377350.12, 2)),
            "minus devedeset jedan biliona tristo trideset četiri milijardi \
             sto četrdeset šest miliona tristo sedamdeset sedam hiljada \
             tristo pedeset zarez jedan dva"
        );
    }

    /// dtoa mode 0 is *shortest*, not *closest*: the double nearest 1e23 is
    /// 99999999999999991611392, and one digit round-trips, so repr stops there.
    /// (Gay's own mode 0 vs mode 1 example.)
    #[test]
    fn shortest_not_closest() {
        assert_eq!(py_repr_float(1e23), "1e+23");
        assert_eq!(py_repr_float(1e22), "1e+22");
        // Subnormal floor and normal ceiling still round-trip.
        assert_eq!(py_repr_float(5e-324), "5e-324");
        assert_eq!(py_repr_float(f64::MAX), "1.7976931348623157e+308");
    }

    /// **Known gap, not fixable in this file.** `FloatValue::Decimal` carries a
    /// `BigDecimal`, whose coefficient is a `BigInt` — and `BigInt` has no
    /// negative zero (`Sign::NoSign`). The pyo3 binding builds it with
    /// `BigDecimal::from_str(decimal_str)`, so the minus in `str(Decimal(
    /// "-0.00")) == "-0.00"` is already gone before this file sees the value.
    ///
    /// Python prints "minus nula zarez nula nula" because BS tests
    /// `n.startswith("-")` on the string; we print "nula zarez nula nula".
    /// The scale survives (that is why the "nula nula" is right), only the sign
    /// of a zero is lost. Affects `Decimal("-0")`, `Decimal("-0.00")` and
    /// friends — and *only* those: any nonzero negative Decimal keeps its sign.
    ///
    /// This test documents the divergence rather than asserting the Python
    /// answer, so it fails loudly if `FloatValue` ever grows the sign.
    #[test]
    fn negative_zero_decimal_loses_its_sign_upstream() {
        assert!(!BigDecimal::from_str("-0.00").unwrap().is_negative());
        // Python: "minus nula zarez nula nula".
        assert_eq!(go(&d("-0.00")), "nula zarez nula nula");
        // Nonzero negatives are unaffected — the sign lives in the coefficient.
        assert_eq!(go(&d("-0.5")), "minus nula zarez pet");
        assert_eq!(go(&d("-0.01")), "minus nula zarez nula jedan");
    }

    /// The ".0" that `Py_DTSF_ADD_DOT_0` appends is the only reason this has a
    /// "zarez" segment at all — and 10^15 is the largest value that still gets
    /// one, since 10^16 tips into exponent notation.
    #[test]
    fn add_dot_0_keeps_integral_floats_on_the_fraction_branch() {
        assert_eq!(py_repr_float(1e15), "1000000000000000.0");
        assert_eq!(go(&f(1e15, 1)), "bilijarda zarez nula");
        assert_eq!(py_repr_float(100.0), "100.0");
        assert_eq!(go(&f(100.0, 1)), "sto zarez nula");
    }

    /// Bug 2 (fixed, #147) through the float path: the left half is words
    /// from 10^9 up, and past maxval it is an OverflowError.
    #[test]
    fn large_values_are_words_on_the_left() {
        assert_eq!(go(&f(1000000000.5, 1)), "milijarda zarez pet");
        assert_eq!(
            go(&f(1234567890.5, 1)),
            "milijarda dvjesto trideset četiri miliona petsto šezdeset sedam \
             hiljada osamsto devedeset zarez pet"
        );
        assert_eq!(go(&f(-1000000000.5, 1)), "minus milijarda zarez pet");
        // Just under 10^9: still words.
        assert_eq!(
            go(&f(123456789.25, 2)),
            "sto dvadeset tri miliona četiristo pedeset šest hiljada \
             sedamsto osamdeset devet zarez dva pet"
        );
        // Decimal keeps full precision past f64's reach (issue #603).
        assert_eq!(
            go(&d("123456789012345678901.5")),
            "sto dvadeset tri triliona četiristo pedeset šest bilijardi sedamsto \
             osamdeset devet biliona dvanaest milijardi tristo četrdeset pet \
             miliona šeststo sedamdeset osam hiljada devetsto jedan zarez pet"
        );
        assert!(matches!(
            LangBs::new().to_cardinal_float(&d("1000000000000000000000000.5"), None),
            Err(N2WError::Overflow(_))
        ));
    }

    /// Bug 9, message shape 1: no "." survives, so `int(n)` sees the literal.
    #[test]
    fn exponent_notation_without_a_dot_raises_valueerror_naming_the_literal() {
        assert_eq!(py_repr_float(1e16), "1e+16");
        assert_eq!(
            err(&f(1e16, 16)),
            "invalid literal for int() with base 10: '1e+16'"
        );
        assert_eq!(py_repr_float(1e-5), "1e-05");
        assert_eq!(
            err(&f(1e-5, 5)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        assert_eq!(
            err(&f(1e100, 0)),
            "invalid literal for int() with base 10: '1e+100'"
        );
        assert_eq!(
            err(&f(1e-300, 300)),
            "invalid literal for int() with base 10: '1e-300'"
        );
        // Decimal's own scientific form: bare "%+d" exponent, upper-case E.
        assert_eq!(py_str_decimal(&BigDecimal::from_str("1E+2").unwrap()), "1E+2");
        assert_eq!(
            err(&d("1E+2")),
            "invalid literal for int() with base 10: '1E+2'"
        );
        assert_eq!(py_str_decimal(&BigDecimal::from_str("1e-7").unwrap()), "1E-7");
        assert_eq!(
            err(&d("1e-7")),
            "invalid literal for int() with base 10: '1E-7'"
        );
    }

    /// Bug 9, message shape 2: the "." splits, `int(left)` succeeds, and the
    /// digit loop trips on the exponent letter.
    #[test]
    fn exponent_notation_with_a_dot_raises_valueerror_naming_the_letter() {
        assert_eq!(py_repr_float(1.5e-5), "1.5e-05");
        assert_eq!(err(&f(1.5e-5, 6)), "invalid literal for int() with base 10: 'e'");
        assert_eq!(py_repr_float(1.5e16), "1.5e+16");
        assert_eq!(err(&f(1.5e16, 0)), "invalid literal for int() with base 10: 'e'");
        assert_eq!(
            py_str_decimal(&BigDecimal::from_str("1.5E+3").unwrap()),
            "1.5E+3"
        );
        assert_eq!(err(&d("1.5E+3")), "invalid literal for int() with base 10: 'E'");
    }

    /// nan/inf reach `int()` as words. The sign is peeled off first, so -inf
    /// and +inf produce the *same* message.
    #[test]
    fn nan_and_inf() {
        assert_eq!(py_repr_float(f64::NAN), "nan");
        assert_eq!(py_repr_float(-f64::NAN), "nan");
        assert_eq!(py_repr_float(f64::INFINITY), "inf");
        assert_eq!(py_repr_float(f64::NEG_INFINITY), "-inf");
        assert_eq!(
            err(&f(f64::NAN, 0)),
            "invalid literal for int() with base 10: 'nan'"
        );
        assert_eq!(
            err(&f(f64::INFINITY, 0)),
            "invalid literal for int() with base 10: 'inf'"
        );
        assert_eq!(
            err(&f(f64::NEG_INFINITY, 0)),
            "invalid literal for int() with base 10: 'inf'"
        );
    }

    /// The last positional value before the `decpt <= -4` cliff, and the first
    /// one past it.
    #[test]
    fn small_positional_boundary() {
        assert_eq!(py_repr_float(1e-4), "0.0001");
        assert_eq!(go(&f(1e-4, 4)), "nula zarez nula nula nula jedan");
        // Decimal's cliff is elsewhere: adjusted >= -6, so 1e-6 stays plain.
        assert_eq!(go(&d("1e-6")), "nula zarez nula nula nula nula nula jedan");
    }

    /// Decimals that carry an exact scale a float could not.
    #[test]
    fn decimal_scale_is_preserved() {
        assert_eq!(py_str_decimal(&BigDecimal::from_str("5.00").unwrap()), "5.00");
        assert_eq!(go(&d("5.00")), "pet zarez nula nula");
        assert_eq!(go(&d("0.0")), "nula zarez nula");
        assert_eq!(go(&d("-0.5")), "minus nula zarez pet");
        assert_eq!(go(&d("-12.34")), "minus dvanaest zarez tri četiri");
        // No fractional part at all: the dotless branch, no "zarez".
        assert_eq!(py_str_decimal(&BigDecimal::from_str("100").unwrap()), "100");
        assert_eq!(go(&d("100")), "sto");
        assert_eq!(go(&d("2.675")), "dva zarez šest sedam pet");
        assert_eq!(go(&d("1.005")), "jedan zarez nula nula pet");
    }

    /// `precision=` is inert for BS: it is applied by assigning
    /// `converter.precision`, which `to_cardinal` never reads.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangBs::new();
        let v = f(1.005, 3);
        for p in [None, Some(0), Some(1), Some(3), Some(5)] {
            assert_eq!(
                l.to_cardinal_float(&v, p).unwrap(),
                "jedan zarez nula nula pet",
                "precision={:?} must not change BS's output",
                p
            );
        }
        // The FloatValue's own `precision` field is equally inert — a bogus
        // one changes nothing, because the digits come from the repr.
        assert_eq!(go(&f(12.34, 99)), "dvanaest zarez tri četiri");
    }

    /// The integer path must stay on the dotless branch: `str(int)` has no ".".
    #[test]
    fn integers_are_unaffected() {
        let l = LangBs::new();
        assert_eq!(l.to_cardinal(&BigInt::from(12)).unwrap(), "dvanaest");
        assert_eq!(l.to_cardinal(&BigInt::from(0)).unwrap(), "nula");
        assert_eq!(l.to_cardinal(&BigInt::from(-12)).unwrap(), "minus dvanaest");
    }
}
