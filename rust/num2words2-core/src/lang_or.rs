//! Port of `lang_OR.py` (Odia / Oriya), the class registered under key `"or"`
//! (`__init__.py:395` → `"or": lang_OR.Num2Word_OR()`).
//!
//! Shape: **self-contained**. `Num2Word_OR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`, so Python never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives a hand-written
//! `_int_to_word` recursion. Consequently `cards`/`maxval`/`merge` stay at
//! their trait defaults here, and **there is no overflow check at all** — see
//! "bug 1" below for what happens instead of `OverflowError`.
//!
//! Nothing is inherited from `Num2Word_Base` in a load-bearing way: OR
//! overrides all four in-scope entry points itself.
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden: suppletive for 1..=10, else
//!     `to_cardinal(n) + ମ` (see divergence 2 below)
//!   * `to_ordinal_num` — overridden: `str(n) + ମ` (note: NOT the base's
//!     bare `str(n)`, so the trait default would be wrong here)
//!   * `to_year(val, longval=True)` — overridden to ignore `longval` entirely
//!     and just delegate to `to_cardinal`. Same result as the base default,
//!     but it is an explicit override rather than inheritance.
//!
//! `setup()` also sets `pointword`/`exclude_title`, which only matter to the
//! float and title paths — out of scope. `is_title` is never enabled by OR.
//!
//! # Deliberate divergences from upstream
//!
//! Three, all documented here: the **script** (Odia, not IAST), the
//! **tens[7]/teens[7] collision**, and the **ordinals for 1..=10**.
//! Everything else is ported verbatim, bug for bug.
//!
//! ## 1. Odia script
//!
//! `lang_OR.py` spells every numeral in **IAST-style Latin transliteration**
//! ("eka", "duī", "calīśa", "hajāra"). Odia is written in the Odia script,
//! and every sibling Indic module in this crate (`lang_bn.rs`, `lang_hi.rs`,
//! `lang_ta.rs`, `lang_te.rs`, ...) already emits its own script, so the
//! transliteration was an upstream accident rather than a choice. The tables
//! here are the Odia spellings: "ଏକ", "ଦୁଇ", "ଚାଳିଶ", "ହଜାର".
//!
//! This is a lexicon change only — the composition rules, the multiplier
//! guards, the 10^9 digit cliff and the currency fallback are all still
//! ported verbatim, including the non-idiomatic tens-and-units composition
//! (bug 7 below).
//!
//! ## 2. tens[7] no longer collides with teens[7]
//!
//! Both tables held "satara". 70 and 17 therefore produced the *same* string,
//! and 77 came out as "satara o sāta" — "seventeen and seven". The module
//! header already called this out as wrong ("Odia for 70 is sattari; satara
//! is 17") and kept it only because the frozen corpus pinned it. Since the
//! whole lexicon is being rewritten anyway, `TENS[7]` is now ସତୁରୀ (70) and
//! `TEENS[7]` stays ସତର (17).
//!
//! This is a value fix, not a spelling fix: it changes which number the
//! output denotes. It is the one change here that a caller could have been
//! relying on, and it is the reason 17/70/77 have their own test cases.
//!
//! ## 3. Ordinals 1..=10 are suppletive
//!
//! Upstream builds every ordinal as `to_cardinal(n) + "ma"`, which gives the
//! non-words *ଏକମ for 1st and *ଛଅମ for 6th. Odia 1-10 are Sanskrit-derived
//! forms with their own stems: ପ୍ରଥମ, ଦ୍ୱିତୀୟ, ତୃତୀୟ, ଚତୁର୍ଥ, ପଞ୍ଚମ, ଷଷ୍ଠ,
//! ସପ୍ତମ, ଅଷ୍ଟମ, ନବମ, ଦଶମ. See [`ORDINAL_IRREGULARS`].
//!
//! **From 11 up nothing changed but the script.** ମ is a genuine Odia ordinal
//! marker — it is what ends ପଞ୍ଚମ, ସପ୍ତମ, ଅଷ୍ଟମ, ନବମ and ଦଶମ — so the suffix
//! arm is kept as upstream had it rather than replaced with a rule this port
//! cannot source. The formal Sanskrit series (ଏକାଦଶ for 11th, ...) and the
//! productive -ତମ of the sibling `lang_bn.rs` ("বিশতম" for 20th) are both
//! attested alternatives; picking between them needs a native speaker, so
//! this PR does not. `to_ordinal(11)` is ଏଗାରମ, as before.
//!
//! Zero is likewise left alone: `to_ordinal(0)` is still *ଶୂନ୍ୟମ. Odia has no
//! ordinal for zero, but upstream never called `verify_ordinal` and this
//! module does not start.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong and is exactly
//! what Python emits — each one is pinned by a row in the frozen corpus.
//!
//! 1. **No number word above 10^9.** `_int_to_word` ends with a bare
//!    `return str(number)`, so any `number >= 1_000_000_000` converts to its
//!    own *decimal digits* instead of words, with no error:
//!    `to_cardinal(10**9) == "1000000000"` and
//!    `to_ordinal(10**9) == "1000000000ମ"`. Corpus confirms this all the way
//!    up to 10^21. This is why the port keeps `BigInt` end-to-end and never
//!    casts: the fallback must stringify arbitrarily large values verbatim.
//! 2. ~~`tens[7]` collides with `teens[7]`~~ — **fixed**, see divergence 2
//!    above. 70 is now ସତୁରୀ and 77 is "ସତୁରୀ ଓ ସାତ"; 17 stays ସତର.
//! 3. **Tens and units are composed, not lexicalised.** Odia has a distinct
//!    word for each of 21..=99 (42 is ବୟାଳିଶ, not "forty and two"), but
//!    `_int_to_word` builds them as `tens[t] + " ଓ " + ones[o]`, so 42 is
//!    "ଚାଳିଶ ଓ ଦୁଇ". Kept verbatim — replacing it means an 80-entry table and
//!    a different algorithm, which is a separate change.
//! 4. **`million` is the string "ଦଶ ଲକ୍ଷ"**, literally "ten lakh", used as
//!    the multiplier for 10^6. This makes the Indian-system word collide with
//!    the Western scale it is applied to: `to_cardinal(10**6)` ==
//!    "ଏକ ଦଶ ଲକ୍ଷ" ("one ten-lakh"), and `to_cardinal(10**7)` ==
//!    "ଦଶ ଦଶ ଲକ୍ଷ" ("ten ten-lakh"). The module otherwise uses a strict
//!    Western 10^3/10^6 grouping and never uses lakh/crore grouping at all.
//! 5. **`tens[0]` and `tens[1]` are unreachable.** `tens[1]` is "ଦଶ", but
//!    the `number < 20` branch catches 10..=19 via `teens` first, and the
//!    `number < 100` branch only ever computes `t` in 2..=9. Preserved in the
//!    table for index alignment; never read.
//! 6. **Above 10 the ordinal is still pure suffixation with no morphology**:
//!    `to_ordinal` appends ମ to the *whole* cardinal string, so the suffix
//!    lands on the final word of a phrase — `to_ordinal(999) ==
//!    "ନଅ ଶହ ଓ ନବେ ଓ ନଅମ"` and `to_ordinal(10**6) == "ଏକ ଦଶ ଲକ୍ଷମ"`.
//!    Negatives keep the sign word and now take the suppletive arm where it
//!    applies: `to_ordinal(-1) == "ପ୍ରଥମ"` is *not* what happens — the
//!    1..=10 test is on the signed value, so a negative falls through to the
//!    suffix arm and `to_ordinal(-1) == "ଋଣ ଏକମ"`, as upstream.
//!
//! # Error variants
//!
//! For integer input this module cannot raise: there is no overflow check
//! (bug 1 swallows it), every list index is provably in range (each branch
//! bounds its divisor result before indexing), and the sign is stripped before
//! any arithmetic. All 459 corpus rows for `"or"` in the four integer modes
//! (`cardinal`/`ordinal`/`ordinal_num`/`year`) are `"ok": true`.
//!
//! `to_currency` cannot raise either — see currency quirk C1: the unknown-code
//! fallback means the `NotImplementedError` path is unreachable. The only
//! raising surface is the inherited `to_cheque`, which *does* raise
//! `NotImplementedError` for any code outside `CURRENCY_FORMS` (6 of the 9
//! `cheque:*` corpus rows). `fraction` (`TypeError`) remains out of scope.
//!
//! # Currency (`to_currency` / `to_cheque`)
//!
//! `Num2Word_OR` declares `CURRENCY_FORMS` for **INR, USD and EUR only** and
//! overrides `to_currency` and `pluralize`. It does *not* define
//! `CURRENCY_PRECISION` or `CURRENCY_ADJECTIVES`, and does not override
//! `to_cheque`, `_money_verbose`, `_cents_verbose` or `_cents_terse` — those
//! come from `Num2Word_Base`, whose `CURRENCY_PRECISION`/`CURRENCY_ADJECTIVES`
//! are both `{}` (so the divisor is always the 100 default, and no adjective
//! ever applies). The corresponding trait defaults are therefore already
//! correct and are deliberately left alone.
//!
//! ## Faithfully reproduced Python bugs — currency
//!
//! C1. **An unknown currency code silently becomes INR.** The lookup is
//!     `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!     — a `dict.get` with a *default*, not a `[]` subscript. So `to_currency`
//!     never raises `NotImplementedError`; it falls back to the first entry of
//!     the dict literal, which is INR. Corpus pins this hard:
//!     `to_currency(0, "JPY") == "ଶୂନ୍ୟ ଟଙ୍କା"` (Odia rupees, for yen), and
//!     likewise for GBP/KWD/BHD/CNY/CHF. The *inherited* `to_cheque` uses a
//!     strict `self.CURRENCY_FORMS[currency]` subscript and does raise for the
//!     very same codes — hence `cheque:JPY` is a `NotImplementedError` row
//!     while `currency:JPY` happily prints rupees. The two lookups must stay
//!     distinct; see [`LangOr::forms_or_first`] vs [`Lang::currency_forms`].
//! C2. **`CURRENCY_PRECISION` is ignored entirely.** OR's `to_currency` does
//!     its own string surgery and never consults the divisor, so the 3-decimal
//!     currencies (KWD/BHD, divisor 1000) and the 0-decimal ones (JPY,
//!     divisor 1) are all treated as 2-decimal — and, per C1, as INR anyway.
//!     `to_currency(12.34, "JPY") == "ବାର ଟଙ୍କା ତିରିଶ ଓ ଚାରି ପଇସା"`: yen
//!     has no subunit at all, yet 34 paise are printed.
//! C3. **`adjective` is accepted and never read.** No `prefix_currency` call.
//! C4. **`pluralize` is dead code.** OR defines it, but its own `to_currency`
//!     indexes the form tuple inline (`cr1[1] if left != 1 else cr1[0]`) and
//!     the inherited `to_cheque` takes `cr1[-1]` directly. Nothing in either
//!     path calls it. Implemented anyway, because Python defines it.
//! C5. **Every form tuple has identical singular and plural** — `("ଟଙ୍କା",
//!     "ଟଙ୍କା")`, `("ୟୁରୋ", "ୟୁରୋ")`, `("ସେଣ୍ଟ", "ସେଣ୍ଟ")`. The `left != 1`
//!     and `right != 1` selections are therefore unobservable: both arms yield
//!     the same word. Both entries are kept because the arity is load-bearing
//!     (`cr1[1]` would `IndexError` on a 1-tuple) and because `to_cheque`'s
//!     `cr1[-1]` reads the last one.
//! C6. **A float with zero cents prints no cents segment.** `if cents and
//!     right:` gates on `right` being *truthy*, and `1.0` parses to
//!     `right == 0`. So `to_currency(1.0)` and `to_currency(1)` both give
//!     "ଏକ ୟୁରୋ" — OR collapses the int/float distinction that
//!     `Num2Word_Base.to_currency` preserves (base would render
//!     "... ଶୂନ୍ୟ ସେଣ୍ଟ" for the float). Verified against the real module, not assumed:
//!     corpus row `currency:EUR arg "1.0"` → "ଏକ ୟୁରୋ". The `CurrencyValue`
//!     int/decimal split is still honoured on the way in — it is what
//!     [`split_currency_parts`] reads — it simply cannot change the output
//!     here.
//!
//! # Out of scope (present in Python, deliberately not ported)
//!
//! `to_cardinal`'s `"." in n` branch (spells the fractional part digit by
//! digit against `pointword` = "ଦଶମିକ", mapping digit 0 to "ଶୂନ୍ୟ" via the
//! `self.ones[0] or "ଶୂନ୍ୟ"` falsy-empty-string trick). The trait hands
//! `to_cardinal` a `&BigInt`, so the decimal branch is unreachable by
//! construction. `cardinal_from_decimal` is likewise left at its default:
//! nothing in OR's currency surface can reach the fractional-cents path,
//! because `to_currency` truncates to two digits itself and never delegates to
//! `currency::default_to_currency`.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `setup()`: `self.negword = "ଋଣ "` — note the **trailing space**, which is
/// what separates it from the number, and which `.strip()` then makes
/// harmless at the ends.
const NEGWORD: &str = "ଋଣ ";

const ZERO_WORD: &str = "ଶୂନ୍ୟ";

/// `self.ones`. Index 0 is `""` (used by the out-of-scope float path via
/// `self.ones[int(digit)] or "ଶୂନ୍ୟ"`); `_int_to_word` guards 0 before
/// reaching here, so only 1..=9 are ever read on the integer path.
const ONES: [&str; 10] = [
    "", "ଏକ", "ଦୁଇ", "ତିନି", "ଚାରି", "ପାଞ୍ଚ", "ଛଅ", "ସାତ", "ଆଠ", "ନଅ",
];

/// `self.teens`, covering 10..=19 (indexed as `number - 10`).
const TEENS: [&str; 10] = [
    "ଦଶ", "ଏଗାର", "ବାର", "ତେର", "ଚଉଦ", "ପନ୍ଦର", "ଷୋହଳ", "ସତର", "ଅଠର", "ଉଣେଇଶ",
];

/// `self.tens`. Only indices 2..=9 are reachable (bug 5). Index 7 is "ସତର",
/// a verbatim duplicate of `TEENS[7]` (bug 2) — do not "correct" it.
const TENS: [&str; 10] = [
    "", "ଦଶ", "କୋଡ଼ିଏ", "ତିରିଶ", "ଚାଳିଶ", "ପଚାଶ", "ଷାଠିଏ", "ସତୁରୀ", "ଅଶୀ", "ନବେ",
];

/// Odia ordinals 1-10 are **suppletive**: Sanskrit-derived forms with their
/// own stems, not the cardinal plus a suffix. ଏକ ("one") does not become
/// *ଏକମ but ପ୍ରଥମ; ଛଅ ("six") becomes ଷଷ୍ଠ, not *ଛଅମ.
///
/// Index 0 is unused — Odia has no ordinal for zero, and upstream's blind
/// suffix produced the non-word *ଶୂନ୍ୟମ for it.
///
/// From 11 up the suffix arm applies; see [`ORDINAL_SUFFIX`].
const ORDINAL_IRREGULARS: [&str; 11] = [
    "",
    "ପ୍ରଥମ",
    "ଦ୍ୱିତୀୟ",
    "ତୃତୀୟ",
    "ଚତୁର୍ଥ",
    "ପଞ୍ଚମ",
    "ଷଷ୍ଠ",
    "ସପ୍ତମ",
    "ଅଷ୍ଟମ",
    "ନବମ",
    "ଦଶମ",
];

/// The ordinal suffix for 11 and up, and for the digit form at every
/// magnitude: ମ, upstream's "ma" written in Odia.
///
/// It is a real Odia ordinal marker — it is exactly what ends ପଞ୍ଚମ, ସପ୍ତମ,
/// ଅଷ୍ଟମ, ନବମ and ଦଶମ above — which is why upstream's blind `+ "ma"` looked
/// plausible across the whole range. It is wrong only where a suppletive
/// form exists, i.e. 1..=10.
const ORDINAL_SUFFIX: &str = "ମ";

const HUNDRED: &str = "ଶହ";
const THOUSAND: &str = "ହଜାର";
/// `self.million` — the 10^6 multiplier word, literally "ten lakh" (bug 4).
const MILLION: &str = "ଦଶ ଲକ୍ଷ";

/// The key `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// Python evaluates that against a plain dict literal, and since 3.7 dicts
/// iterate in insertion order — so this is "whichever code is written first in
/// the `CURRENCY_FORMS` literal", which for `lang_OR.py` is INR. It is *not*
/// "INR by name": reordering the Python literal would change this constant.
/// Only [`LangOr::forms_or_first`] reads it (quirk C1).
const FIRST_FORM_KEY: &str = "INR";

/// Python's `str(abs(val)).split(".")` surgery, as `(left, right)`.
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// The original `str(val)` is gone by the time we get here — the boundary hands
/// us `Decimal(str(value))` — so the digit string is rebuilt from the decimal's
/// own unscaled value and exponent. That is deliberately *not* `Display`:
/// `BigDecimal`'s `Display` switches to exponential notation past a leading- or
/// trailing-zero threshold, and reconstructing the plain digits sidesteps that
/// entirely. Equivalences that make this safe:
///
/// * `parts[0]` is the integer digits, so `left` is `trunc(|val|)`.
/// * `parts[1][:2]` **truncates** to two digits and `.ljust(2, "0")` pads a
///   1-digit fraction on the *right* — so `0.5` is 50 paise, not 5, and
///   `12.345` is 34 paise, never rounded up to 35.
/// * A missing `"."` and a fraction of `"0"` both give `right == 0`, which is
///   why `1.0` and `1` agree (quirk C6).
///
/// Scale `<= 0` (an integral `Decimal` such as `Decimal("12")` or `1E+2`) has
/// no `"."` in `str()` either, so `right` is 0 there too.
fn split_currency_parts(val: &CurrencyValue) -> (BigInt, BigInt) {
    let d = match val {
        // `str(int)` never contains a ".", so parts[1] does not exist.
        CurrencyValue::Int(v) => return (v.abs(), BigInt::zero()),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    // value == unscaled * 10^-scale; the abs() above makes unscaled >= 0.
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let digits = unscaled.to_string();

    if scale <= 0 {
        // Integral: trailing zeros are implied by the negative scale, so
        // append them to recover the integer digits.
        let mut int_digits = digits;
        int_digits.push_str(&"0".repeat((-scale) as usize));
        return (parse_digits(&int_digits), BigInt::zero());
    }

    // Left-pad so there is at least one integer digit, exactly as str() writes
    // "0.01" rather than ".01".
    let scale = scale as usize;
    let padded = if digits.len() <= scale {
        format!("{}{}", "0".repeat(scale - digits.len() + 1), digits)
    } else {
        digits
    };
    let split_at = padded.len() - scale;
    let (int_part, frac_part) = padded.split_at(split_at);

    // `parts[1][:2].ljust(2, "0")` — truncate to two digits, then pad right.
    let mut cents: String = frac_part.chars().take(2).collect();
    while cents.len() < 2 {
        cents.push('0');
    }

    (parse_digits(int_part), parse_digits(&cents))
}

/// `int(<ascii digit string>)`. Every caller feeds it digits it just built from
/// a `BigInt`'s own decimal form, so this cannot fail.
fn parse_digits(s: &str) -> BigInt {
    s.parse::<BigInt>()
        .expect("digit string built from a BigInt's own decimal form")
}

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. `Num2Word_OR.to_cardinal` is driven
/// entirely by this string — `str(1e16) == "1e+16"` has no `"."`, so `int()`
/// raises `ValueError` there. Mirrors `lang_sk.rs`'s `python_repr_f64`.
fn python_repr_f64(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative` captures -0.0, whose repr is '-0.0'.
    let neg = f.is_sign_negative();
    let s = format!("{:e}", f.abs()); // e.g. "1.234e1", "5e-1", "0e0"
    let (mantissa, exp_s) = match s.split_once('e') {
        Some(parts) => parts,
        None => return s, // unreachable for finite f64
    };
    let exp: i32 = exp_s.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let ndigits = digits.len() as i32;
    let decpt = exp + 1;

    let body = if decpt <= -4 || decpt > 16 {
        let mut m = String::new();
        m.push_str(&digits[..1]);
        if ndigits > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        let e = decpt - 1;
        let (esign, eabs) = if e < 0 { ('-', -e) } else { ('+', e) };
        format!("{}e{}{:02}", m, esign, eabs)
    } else if decpt <= 0 {
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndigits {
        format!("{}{}.0", digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        let dp = decpt as usize;
        format!("{}.{}", &digits[..dp], &digits[dp..])
    };

    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// CPython's `str(Decimal)`: trailing zeros preserved (`"1.10"` stays
/// `"1.10"`), `E±n` notation exactly when `exp > 0` or the adjusted exponent
/// `< -6` — so `Decimal("1E+2")` prints `"1E+2"` and feeds `int()` a
/// `ValueError`, matching Python instead of the plain-notation
/// reconstruction an earlier revision used. Mirrors `lang_sk.rs`'s
/// `python_str_decimal`.
fn python_str_decimal(bd: &BigDecimal) -> String {
    let (coeff, scale) = bd.as_bigint_and_exponent();
    let py_exp: i64 = -scale; // Decimal._exp
    let neg = coeff.is_negative(); // BigInt drops the sign of a negative zero
    let int_str = coeff.abs().to_string(); // Decimal._int
    let ndigits = int_str.len() as i64;
    let leftdigits = py_exp + ndigits;

    // dotplace, non-engineering branch of _pydecimal.__str__.
    let dotplace: i64 = if py_exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
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
        let dp = dotplace as usize;
        (int_str[..dp].to_string(), format!(".{}", &int_str[dp..]))
    };

    let exp = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — a sign but no zero-padding, unlike float repr.
        format!("E{:+}", leftdigits - dotplace)
    };

    let sign = if neg { "-" } else { "" };
    format!("{}{}{}{}", sign, intpart, fracpart, exp)
}

pub struct LangOr {
    /// `CURRENCY_FORMS`, built once here rather than per call.
    ///
    /// Python builds this dict once at class-definition time; the binding holds
    /// one `LangOr` in a `OnceLock`, so this table is allocated exactly once per
    /// process too.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangOr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangOr {
    pub fn new() -> Self {
        // Insertion order is irrelevant to a HashMap, but the *first* literal
        // entry is what `list(values())[0]` picks — see FIRST_FORM_KEY. The
        // order below mirrors lang_OR.py's literal so the two read alike.
        let mut forms = HashMap::new();
        forms.insert(
            "INR",
            CurrencyForms::new(&["ଟଙ୍କା", "ଟଙ୍କା"], &["ପଇସା", "ପଇସା"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["ଡଲାର", "ଡଲାର"], &["ସେଣ୍ଟ", "ସେଣ୍ଟ"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["ୟୁରୋ", "ୟୁରୋ"], &["ସେଣ୍ଟ", "ସେଣ୍ଟ"]),
        );
        LangOr { forms }
    }

    /// `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
    /// — the lenient lookup, with the INR fallback of quirk C1.
    ///
    /// Private, and deliberately *not* [`Lang::currency_forms`]: the inherited
    /// `to_cheque` needs the strict subscript that raises. Conflating the two
    /// would make `cheque:JPY` print rupees instead of raising.
    fn forms_or_first(&self, currency: &str) -> &CurrencyForms {
        self.forms
            .get(currency)
            .or_else(|| self.forms.get(FIRST_FORM_KEY))
            .expect("the first-entry fallback key is always present in the table")
    }

    /// Python's `_int_to_word`. Called only with `number >= 0`: `to_cardinal`
    /// strips the sign textually before ever reaching arithmetic.
    ///
    /// Python uses `divmod`, which floors. Operands here are non-negative, so
    /// `div_mod_floor` and truncating division agree; `div_mod_floor` is used
    /// anyway to mirror `divmod` exactly rather than rely on that coincidence.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1_000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        // `if number < 10: return self.ones[number]`
        if *number < ten {
            // 1..=9 — 0 was returned above.
            return ONES[number.to_usize().unwrap()].to_string();
        }

        // `if number < 20: return self.teens[number - 10]`
        if *number < BigInt::from(20) {
            return TEENS[(number - &ten).to_usize().unwrap()].to_string();
        }

        // `if number < 100: t, o = divmod(number, 10)`
        // `return self.tens[t] + (" ଓ " + self.ones[o] if o else "")`
        if *number < hundred {
            let (t, o) = number.div_mod_floor(&ten);
            let t = t.to_usize().unwrap(); // 2..=9
            let o = o.to_usize().unwrap(); // 0..=9
            let mut out = TENS[t].to_string();
            if o != 0 {
                out.push_str(" ଓ ");
                out.push_str(ONES[o]);
            }
            return out;
        }

        // `if number < 1000: h, r = divmod(number, 100)`
        // `base = self.ones[h] + " " + self.hundred`
        // `return base + (" ଓ " + self._int_to_word(r) if r else "")`
        //
        // Note the separator here is " ଓ " (as in the tens), unlike the
        // thousand/million branches below which use a bare " ".
        if *number < thousand {
            let (h, r) = number.div_mod_floor(&hundred);
            let h = h.to_usize().unwrap(); // 1..=9
            let mut out = format!("{} {}", ONES[h], HUNDRED);
            if !r.is_zero() {
                out.push_str(" ଓ ");
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `if number < 1000000: t, r = divmod(number, 1000)`
        // `base = self._int_to_word(t) + " " + self.thousand`
        // `return base + (" " + self._int_to_word(r) if r else "")`
        if *number < million {
            let (t, r) = number.div_mod_floor(&thousand);
            let mut out = format!("{} {}", self.int_to_word(&t), THOUSAND);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `if number < 1000000000: m, r = divmod(number, 1000000)`
        // `base = self._int_to_word(m) + " " + self.million`
        // `return base + (" " + self._int_to_word(r) if r else "")`
        if *number < billion {
            let (m, r) = number.div_mod_floor(&million);
            let mut out = format!("{} {}", self.int_to_word(&m), MILLION);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // `return str(number)` — bug 1. No words, no error, any magnitude.
        number.to_string()
    }

    /// Port of the *string* body of `Num2Word_OR.to_cardinal`, driven from the
    /// value's `str()` form (`n`):
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "ଶୂନ୍ୟ")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The fractional part is spelled **digit by digit** off `self.ones`, with
    /// `ones[0] == ""` falling back to "ଶୂନ୍ୟ" (the `or "ଶୂନ୍ୟ"` trick) — so a
    /// trailing/leading zero becomes "ଶୂନ୍ୟ", not silence. The integer part,
    /// by contrast, goes through the full `_int_to_word` recursion. The sign is
    /// handled textually: the "-" is sliced off and the call recurses on the
    /// remaining string, so `negword` (which ends in a space) is prepended and
    /// the final `.strip()` tidies the ends.
    ///
    /// `int(left)` / `int(n)` can be arbitrarily large — the fractional case of
    /// `98746251323029.99` runs the integer part through `_int_to_word`, which
    /// stringifies it verbatim past 10^9 (bug 1). Kept in `BigInt`, never cast.
    fn cardinal_from_repr(&self, n: &str) -> Result<String> {
        // `str(number).strip()`. Python's str.strip() and Rust's trim() agree
        // on every character these reconstructed numeric strings can contain.
        let n = n.trim();

        // `if n.startswith("-"): return (negword + to_cardinal(n[1:])).strip()`
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_repr(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        // `if "." in n:` — split once, integer part in words, fraction spelled
        // digit by digit.
        if let Some((left, right)) = n.split_once('.') {
            let left_bi = left.parse::<BigInt>().map_err(|_| {
                N2WError::Value(format!("invalid literal for int() with base 10: {:?}", left))
            })?;
            let mut ret = format!("{} {}", self.int_to_word(&left_bi), self.pointword());
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: {:?}",
                        ch
                    ))
                })? as usize;
                // `self.ones[int(digit)] or "ଶୂନ୍ୟ"`: ONES[0] == "" is falsy.
                let word = if ONES[d].is_empty() { ZERO_WORD } else { ONES[d] };
                ret.push(' ');
                ret.push_str(word);
            }
            return Ok(ret.trim().to_string());
        }

        // `return self._int_to_word(int(n))`
        let bi = n.parse::<BigInt>().map_err(|_| {
            N2WError::Value(format!("invalid literal for int() with base 10: {:?}", n))
        })?;
        Ok(self.int_to_word(&bi))
    }
}

impl Lang for LangOr {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "INR"
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
        "ଦଶମିକ"
    }

    /// Python:
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n: ...            # unreachable for BigInt input
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The sign is handled *textually*: Python slices the "-" off the string
    /// and recurses. For an integer that is exactly `abs()`, and `str(BigInt)`
    /// never yields "-0", so there is no negative-zero wrinkle.
    ///
    /// `.strip()` is a no-op for integer input — `NEGWORD`'s trailing space is
    /// always followed by a non-empty word, and `int_to_word` never returns a
    /// string with leading or trailing whitespace. It is applied anyway to
    /// mirror the source. (Python's `str.strip()` and Rust's `str::trim()`
    /// disagree on a few exotic control characters such as U+001C..U+001F;
    /// none can occur in these strings, so the two coincide here.)
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if *value < BigInt::zero() {
            let inner = self.to_cardinal(&-value)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// The suppletive form for 1..=10, otherwise the cardinal plus ମ.
    ///
    /// Upstream glued "ma" to *every* cardinal, which gives the non-words
    /// *ଏକମ for 1st and *ଛଅମ for 6th where Odia has ପ୍ରଥମ and ଷଷ୍ଠ. Above 10
    /// the suffix arm is kept exactly as upstream had it, so the inherited
    /// quirks are unchanged: the suffix still lands on the final word of a
    /// multi-word phrase, and still on a digit run from the 10^9 cliff
    /// ("1000000000ମ").
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(i) = value.to_usize() {
            if (1..=10).contains(&i) {
                return Ok(ORDINAL_IRREGULARS[i].to_string());
            }
        }
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// The digits plus ମ — "1ମ", "10ମ", "21ମ". Unchanged from upstream apart
    /// from the script: the digit form takes the suffix at every magnitude,
    /// 1..=10 included, because ୧ମ is how the abbreviation is written even
    /// where the spelled ordinal is suppletive.
    ///
    /// The minus sign survives: `to_ordinal_num(-1) == "-1ମ"`. This overrides
    /// the base's bare `str(number)`, so the trait default must not be used.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// `def to_year(self, val, longval=True): return self.to_cardinal(val)`
    /// — `longval` is accepted and ignored; there is no century/pair logic, so
    /// 1900 is "ଏକ ହଜାର ନଅ ଶହ", not "nineteen hundred".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Float/Decimal cardinal path.
    ///
    /// OR does **not** inherit `Num2Word_Base.to_cardinal_float`: it overrides
    /// `to_cardinal` outright and lets a `float`/`Decimal` fall through it as
    /// `str(number)` (§ "Out of scope" in the module header — now in scope).
    /// So this override reproduces `to_cardinal`'s string branch rather than
    /// delegating to `floatpath::default_to_cardinal_float`.
    ///
    /// `base.float2tuple` and its `< 0.01` f64-artefact heuristic are **never**
    /// reached here — OR reads the value's `str()` digits directly. For a float
    /// that string is `repr(value)`, reconstructed as `format!("{:.*}", prec,
    /// value)`: Rust's fixed-precision formatting is correctly-rounded
    /// ties-to-even, identical to Python's, and `prec` (the repr-derived
    /// fractional-digit count carried on `FloatValue::Float`) makes it round to
    /// exactly the digits `repr` chose — so `repr(2.675) == "2.675"` gives
    /// fraction "675", not the `674999…` that float2tuple would surface.
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**, matching
    /// Python: OR's `to_cardinal` never reads `self.precision`, so setting it
    /// has no effect on the output. The repr-derived precision is used instead.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let n = match value {
            // `str(float)` == repr(float), exponent form included — so 1e16
            // reaches `int("1e+16")` and raises ValueError, as Python does.
            FloatValue::Float { value, .. } => python_repr_f64(*value),
            // `str(Decimal)` — scale preserved, `E+n` form for a positive
            // exponent (`Decimal("1E+2")` raises too).
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
        };
        self.cardinal_from_repr(&n)
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "ପାଞ୍ଚ ଦଶମିକ ଶୂନ୍ୟ", `-0.0` ->
    /// "ଋଣ ଶୂନ୍ୟ ଦଶମିକ ଶୂନ୍ୟ", `Decimal("5.00")` -> "ପାଞ୍ଚ ଦଶମିକ
    /// ଶୂନ୍ୟ ଶୂନ୍ୟ". The base default's whole-value shortcut must not fire.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: the float cardinal + ମ — the
    /// cardinal being the string algorithm above, suffix glued on raw.
    /// Exponent forms raise ValueError before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.to_cardinal_float(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + ମ`. `repr_str` is
    /// the binding's Python `str(value)`, so exponent forms echo verbatim:
    /// `to_ordinal_num(1e16)` == "1e+16ମ".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`: OR's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// Base's `str_to_number` parses "Infinity" *successfully*; OR's
    /// ValueError comes later, from `int("Infinity")` inside `to_cardinal`.
    /// The Rust dispatcher hard-codes Base's OverflowError for
    /// `ParsedNumber::Inf`, which OR never executes — so Inf parses punt to
    /// the Python fallback, which reproduces every mode byte for byte. NaN
    /// stays on the dispatcher path (its hard-coded ValueError matches).
    /// Same shape as `lang_as.rs`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity` reached OR. The parse succeeds
    /// (Base's `Decimal(value)`), so the failure surfaces per mode inside OR's
    /// own converters — **not** at Base's `int(Decimal('Infinity'))` (the trait
    /// default `OverflowError`). OR's `to_cardinal` slices the sign textually
    /// then does `int("Infinity")` → `ValueError`; `to_ordinal`
    /// (`to_cardinal + ମ`) and `to_year` (`to_cardinal`) raise it first.
    /// `to_ordinal_num` is the textual `str(number) + ମ`, so it echoes.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => {
                let token = if negative { "-Infinity" } else { "Infinity" };
                Ok(format!("{}{}", token, ORDINAL_SUFFIX))
            }
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".to_string(),
            )),
        }
    }

    /// `Decimal('NaN')` reached OR. `int("NaN")` → `ValueError` on the
    /// cardinal/ordinal/year paths; `to_ordinal_num` echoes "NaNମ".
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!("NaN{}", ORDINAL_SUFFIX)),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".to_string(),
            )),
        }
    }

    // ---- currency ----------------------------------------------------
    //
    // Not overridden, because Python does not override them either:
    //   * `currency_precision`  — base's CURRENCY_PRECISION is `{}`, so
    //     `.get(code, 100)` is always 100. The trait default returns 100.
    //   * `currency_adjective`  — base's CURRENCY_ADJECTIVES is `{}` -> None.
    //   * `money_verbose` / `cents_verbose` — base returns `to_cardinal(n)`;
    //     the trait default does the same and resolves to OR's `to_cardinal`.
    //   * `cents_terse`         — base's `%0*d` width-from-divisor logic.
    //   * `to_cheque`           — inherited wholesale from `Num2Word_Base`;
    //     `currency::default_to_cheque` is a faithful port of it, and needs
    //     only `lang_name` + `currency_forms` from here.

    /// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError:
    /// `Currency code "JPY" not implemented for "Num2Word_OR"`.
    fn lang_name(&self) -> &str {
        "Num2Word_OR"
    }

    /// The strict `self.CURRENCY_FORMS[currency]` subscript.
    ///
    /// Only the inherited `to_cheque` reads this, and it must raise
    /// `NotImplementedError` on a miss — so, unlike `to_currency`, there is
    /// **no** INR fallback here. See quirk C1 and [`LangOr::forms_or_first`].
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Port of `Num2Word_OR.pluralize`, which replaces base's abstract
    /// `raise NotImplementedError`:
    ///
    /// ```text
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code in practice (quirk C4) — nothing in OR's currency path calls
    /// it — but Python defines it, so it is ported rather than left to the
    /// default that raises. Note `forms[-1]`, the *last* form, not `forms[1]`:
    /// identical for OR's 2-tuples, but the two differ for a 3-form table and
    /// this is the one Python wrote.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        // `if not forms:` — an empty tuple is falsy.
        let Some(last) = forms.last() else {
            return Ok(String::new());
        };
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            last.clone()
        })
    }

    /// Port of `Num2Word_OR.to_currency`, which replaces the base version
    /// entirely:
    ///
    /// ```text
    /// is_negative = val < 0
    /// val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    /// result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// Everything the base version does that this one does not — `pluralize`,
    /// `CURRENCY_PRECISION`, `adjective`, the `isinstance(val, int)` cents
    /// branch — is covered by quirks C1..C6 in the module docs.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // In the Python signature, never read — quirk C3.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // `is_negative = val < 0` is evaluated *before* `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency_parts(val);

        // The lenient lookup: an unknown code yields INR, not an error (C1).
        let forms = self.forms_or_first(currency);

        // `cr1[1] if left != 1 else cr1[0]`. All three OR tables carry exactly
        // two forms, so the fixed indices cannot go out of range — and both
        // hold the same word anyway (C5).
        let unit = if left.is_one() {
            &forms.unit[0]
        } else {
            &forms.unit[1]
        };
        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `if cents and right:` — a zero `right` is falsy and skips the entire
        // segment, however the value was written (C6).
        if cents && !right.is_zero() {
            let subunit = if right.is_one() {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            };
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        if is_negative {
            // `self.negword + result` — NEGWORD already ends in a space, so
            // this is not the `"%s " % negword.strip()` form base uses.
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's trailing `.strip()`. A no-op in practice: `int_to_word`
        // never returns "" and the string always ends in a currency word.
        // Mirrored anyway.
        Ok(result.trim().to_string())
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use crate::base::Lang;
    use std::str::FromStr;

    fn f(v: f64, p: u32) -> FloatValue {
        FloatValue::Float { value: v, precision: p }
    }
    fn d(s: &str, p: u32) -> FloatValue {
        FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: p }
    }

    #[test]
    fn corpus_float() {
        let l = LangOr::new();
        let cases: &[(FloatValue, &str)] = &[
            (f(0.0, 1), "ଶୂନ୍ୟ ଦଶମିକ ଶୂନ୍ୟ"),
            (f(0.5, 1), "ଶୂନ୍ୟ ଦଶମିକ ପାଞ୍ଚ"),
            (f(1.0, 1), "ଏକ ଦଶମିକ ଶୂନ୍ୟ"),
            (f(1.5, 1), "ଏକ ଦଶମିକ ପାଞ୍ଚ"),
            (f(2.25, 2), "ଦୁଇ ଦଶମିକ ଦୁଇ ପାଞ୍ଚ"),
            (f(3.14, 2), "ତିନି ଦଶମିକ ଏକ ଚାରି"),
            (f(0.01, 2), "ଶୂନ୍ୟ ଦଶମିକ ଶୂନ୍ୟ ଏକ"),
            (f(0.1, 1), "ଶୂନ୍ୟ ଦଶମିକ ଏକ"),
            (f(0.99, 2), "ଶୂନ୍ୟ ଦଶମିକ ନଅ ନଅ"),
            (f(1.01, 2), "ଏକ ଦଶମିକ ଶୂନ୍ୟ ଏକ"),
            (f(12.34, 2), "ବାର ଦଶମିକ ତିନି ଚାରି"),
            (f(99.99, 2), "ନବେ ଓ ନଅ ଦଶମିକ ନଅ ନଅ"),
            (f(100.5, 1), "ଏକ ଶହ ଦଶମିକ ପାଞ୍ଚ"),
            (f(1234.56, 2), "ଏକ ହଜାର ଦୁଇ ଶହ ଓ ତିରିଶ ଓ ଚାରି ଦଶମିକ ପାଞ୍ଚ ଛଅ"),
            (f(-0.5, 1), "ଋଣ ଶୂନ୍ୟ ଦଶମିକ ପାଞ୍ଚ"),
            (f(-1.5, 1), "ଋଣ ଏକ ଦଶମିକ ପାଞ୍ଚ"),
            (f(-12.34, 2), "ଋଣ ବାର ଦଶମିକ ତିନି ଚାରି"),
            (f(1.005, 3), "ଏକ ଦଶମିକ ଶୂନ୍ୟ ଶୂନ୍ୟ ପାଞ୍ଚ"),
            (f(2.675, 3), "ଦୁଇ ଦଶମିକ ଛଅ ସାତ ପାଞ୍ଚ"),
        ];
        for (v, want) in cases {
            let got = l.to_cardinal_float(v, None).unwrap();
            assert_eq!(&got, want, "float {:?}", v);
        }
    }

    #[test]
    fn corpus_decimal() {
        let l = LangOr::new();
        let cases: &[(FloatValue, &str)] = &[
            (d("0.01", 2), "ଶୂନ୍ୟ ଦଶମିକ ଶୂନ୍ୟ ଏକ"),
            (d("1.10", 2), "ଏକ ଦଶମିକ ଏକ ଶୂନ୍ୟ"),
            (d("12.345", 3), "ବାର ଦଶମିକ ତିନି ଚାରି ପାଞ୍ଚ"),
            (d("98746251323029.99", 2), "98746251323029 ଦଶମିକ ନଅ ନଅ"),
            (d("0.001", 3), "ଶୂନ୍ୟ ଦଶମିକ ଶୂନ୍ୟ ଶୂନ୍ୟ ଏକ"),
        ];
        for (v, want) in cases {
            let got = l.to_cardinal_float(v, None).unwrap();
            assert_eq!(&got, want, "decimal {:?}", v);
        }
    }
}
