//! Port of `lang_MN.py` (Mongolian).
//!
//! Shape: **self-contained**. `Num2Word_MN` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`, so Python never populates `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives `_int2word` over
//! 3-digit chunks produced by `utils.splitbyx`. `cards`/`merge` therefore
//! stay at their trait defaults here. The `THOUSANDS` table (keys 1..=22)
//! ends at 10^66; Python raised `KeyError` for values >= 10^69. Fixed
//! (gladiaio/num2words2#159): `maxval()` is 10^69 and `_int2word` raises
//! `OverflowError` for `abs(n) >= 10**69` up front.
//!
//! `setup()` sets `negword = "хасах"`; everything else in the four in-scope
//! modes is defined locally, so nothing is inherited from `Num2Word_Base`
//! except the class scaffolding. All four modes (`to_cardinal`, `to_ordinal`,
//! `to_ordinal_num`, `to_year`) are overridden by MN itself.
//!
//! # The `all_suffixed` flag
//!
//! Mongolian numerals carry two forms: a *free/final* form ("арав", "хорь",
//! "зуу", "гурав") and an *attributive/suffixed* form ("арван", "хорин",
//! "зуун", "гурван"). `_int2word` normally emits the suffixed form everywhere
//! except on the **last** word of the **final** (i == 0) chunk, which takes the
//! free form. `all_suffixed=True` — used only by `to_year` and the currency
//! path — suppresses that exception, so every word keeps its suffixed form.
//!
//! `to_cardinal(1234)` == "нэг мянга хоёр зуун гучин дөрөв"  (free "дөрөв")
//! `to_year(1234)`     == "нэг мянга хоёр зуун гучин дөрвөн он" (suffixed)
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits:
//!
//! 1. **`all_suffixed` is dropped across the negative recursion.**
//!    `_int2word` line 180 reads
//!    `return " ".join([self.negword, self._int2word(abs(n))])` — it does
//!    *not* forward `all_suffixed`, so `to_cardinal(-1234, all_suffixed=True)`
//!    silently produces the *unsuffixed* "хасах нэг мянга хоёр зуун гучин
//!    дөрөв". Reachable through the `all_suffixed` kwarg (the kwargs corpus
//!    pins `to_cardinal(-5, all_suffixed=True)` == "хасах тав", not "таван")
//!    and inherited by the currency path; unreachable from `to_year`, which
//!    takes `abs()` and prefixes "МЭӨ " before calling.
//! 2. **`to_ordinal(0)` / `to_ordinal_num(0)` raise `IndexError`.**
//!    `_get_ordinal_suffix` tests `number_str[-1] != "0"`; for "0" that is
//!    false, so it falls to the `elif number_str[-2] != "0"` arm and indexes
//!    position -2 of a 1-character string. Zero is the *only* input that can
//!    trigger this: every other value ending in "0" has at least two
//!    characters, and no negative renders as "-0". See [`ordinal_suffix`].
//! 3. **The ordinal suffix only inspects the last two digits, and only the
//!    tens digit when the units digit is 0.** So 40 → "дөч дүгээр" (tens is
//!    4) but 100 → "зуу дугаар" (tens is 0, so the `elif` guard fails and the
//!    default "дугаар" survives — the hundreds digit is never consulted).
//! 4. **The "мянга" → "мянган" fixup is a literal last-word string compare.**
//!    `if all_suffixed and words and words[-1] == "мянга"`. It fires for
//!    `to_year(1000)` → "нэг мянган он" and `to_year(2000)` → "хоёр мянган
//!    он", but not for `to_year(1005)` (last word is "таван"). No other
//!    `THOUSANDS` entry gets an equivalent fixup.
//! 5. **`ONES[9][0]` is "ес" but `ONES[9][1]` is "есөн"** — an irregular pair
//!    (most others just append -н/-ан). Kept verbatim, as is `TWENTIES[8]`
//!    ("ная"/"наян") and `ONES[4]` ("дөрөв"/"дөрвөн", with the vowel dropping
//!    out of the stem).
//! 6. **`to_currency` on an `int` ignores the currency entirely.** The method
//!    opens with `if isinstance(val, int): return self.to_cardinal(val)` — no
//!    table lookup, no unit word, no `NotImplementedError`. So
//!    `to_currency(100, "BHD")` is "зуу" (a bare cardinal) even though BHD is
//!    absent from `CURRENCY_FORMS`, while `to_currency(12.34, "BHD")` raises.
//!    The corpus pins both halves of that split. See [`LangMn::to_currency`].
//! 7. **The 1/100 divisor is hardcoded, `CURRENCY_PRECISION` is never read.**
//!    `to_currency` computes `(Decimal(str(val)) * 100) % 1` and calls
//!    `parse_currency_parts` without a `divisor=`, taking its default of 100.
//!    `Num2Word_MN` leaves `CURRENCY_PRECISION` at Base's empty dict, so the
//!    two agree today at 100 for every code — but the hardcoding is what the
//!    source says, and it is why JPY renders сен at 1/100 here rather than
//!    taking `base.to_currency`'s zero-decimal shortcut. See [`DIVISOR`].
//! 8. **`cents=False` emits `str(right)`, not `_cents_terse(right)`.** So a
//!    sub-unit under ten prints unpadded — 0.04 gives "4" where Base gives
//!    "04". [`Lang::cents_terse`] is consequently dead code for Mongolian and
//!    is left at its trait default.
//! 9. **The cents segment is gated on `right > 0` alone.** MN never computes
//!    Base's `has_decimal` guard, so `1.0` renders "нэг евро" with no cents
//!    tail — the same output an `int` 1 would give by quirk 6, but reached by
//!    a different path.
//! 10. **`CURRENCY_ADJECTIVES["CZK"]` is `"Чехийн "` with a trailing space**,
//!    and `prefix_currency` joins with `"%s %s"` — so CZK renders the double
//!    space "Чехийн  крон". Reproduced verbatim; no corpus row covers CZK.
//! 11. **`CURRENCY_FORMS["KWD"]` is `"динaр"` with a LATIN SMALL LETTER A**
//!    (U+0061) where the Cyrillic а (U+0430) belongs. It survives `.upper()`
//!    as a Latin "A", which is why the cheque corpus expects "ДИНAР". The
//!    tables here were generated from the live interpreter rather than
//!    transcribed, so the mixed script is preserved byte for byte.
//!
//! # Error variants
//!
//! Mongolian raises two exception types that the four-way mapping in
//! PORTING.md (Overflow/Type/NotImplemented/ZeroDivision) does not cover:
//! `IndexError` (quirk 2 above) and `KeyError` (`THOUSANDS[i]` for i >= 23,
//! i.e. values >= 10^69). Both are Python *crashes* rather than deliberate
//! raises. The `IndexError` is reproduced (see [`index_error`]); the
//! `KeyError` ([`key_error`]) is unreachable from the public entry points
//! since the 10^69 MAXVAL check (#159) raises `OverflowError` first.
//!
//! # The float/`POINT_WORDS` branch of `to_cardinal`
//!
//! `Num2Word_MN` overrides **`to_cardinal`** (not `to_cardinal_float`) and
//! handles a non-integer inline. The algorithm is *not* Base's `float2tuple`
//! path: it reads the **string** `n = str(value).replace(",", ".")`, splits on
//! ".", and renders
//! `"%s, %s %s" % (_int2word(int(left)), POINT_WORDS[len(right)],
//! _int2word(int(right)))`, keyed on the *number of decimal places*. So MN
//! **cannot** inherit either [`Lang::to_cardinal_float`] or
//! [`Lang::cardinal_from_decimal`] from Base — the default emits "жаран долоо
//! (.) тав" (Base's untranslated `pointword`) where MN says "жаран долоо,
//! аравны тав". Both are overridden below, along with the full entry
//! [`Lang::cardinal_float_entry`], because MN's `to_cardinal` owns the
//! whole-value routing too (it never runs Base's `int(value) == value` test).
//!
//! Because the algorithm keys on `str(value)`, the f64 artefacts that
//! `floatpath.rs` reproduces do **not** apply: `str(2.675)` is the literal
//! "2.675", so `right == "675"` with no `674.9999…`/`< 0.01` rescue involved.
//! [`mn_value_str`] reconstructs Python's `str(value)`: the f64 arm via
//! [`python_float_str`] (`format!("{:.p$}", f)` — Python already supplied
//! `precision` as the repr's fractional-digit count — switching to `1e+16`
//! exponent form at the same thresholds CPython's repr uses), the Decimal arm
//! exactly via `strnum::python_decimal_str`, so a `Decimal("1.10")` keeps its
//! trailing zero and 98746251323029.99 keeps every digit (issue #603).
//! [`mn_to_cardinal_str`] then runs the Python method verbatim on that string.
//!
//! Python quirks in this branch, all reproduced:
//!
//! * **`int(right) == 0` returns before the `len(right) > 6` raise.** So
//!   `Decimal("1.00000000")` is "нэг" (all-zero fraction, integer early
//!   return) while `Decimal("1.10000000")` raises `NotImplementedError` (eight
//!   non-zero decimal places). The early return is `_int2word(int(float(n)))`,
//!   which re-casts through f64 and so is signed but lossy for huge integers.
//! * **`str(value)` with no "." hits the integer `else` and its `int(str)`
//!   raises `ValueError`** — `invalid literal for int() with base 10: '…'` —
//!   for every value whose repr is scientific or non-finite: floats >= 1e16
//!   (`str(1e16) == "1e+16"`), Decimals Python renders in E-notation
//!   (`Decimal("1e3")` prints "1E+3", `Decimal("1E-7")` prints "1E-7"), and
//!   `Decimal("Infinity")`/`"NaN"`. The corpus pins 1e+16/1e+20/1E+2/1E+20 and
//!   the "Infinity" strings. So MN's ceiling on the cardinal *float* path is
//!   1e16, while ints run to the 10^69 MAXVAL `OverflowError` — and
//!   `to_ordinal(1e+16)` still works, because it truncates the float without
//!   ever stringifying it (see below).
//!
//! # Float/Decimal entries for the other three modes
//!
//! `to_ordinal`/`to_ordinal_num` open with `number = int(value)` — plain
//! truncation toward zero, no string inspection — so `to_ordinal(2.5)` is
//! "хоёр дугаар", `to_ordinal(1e+16)` is "арван тунамал дугаар" (no
//! ValueError: the float is never stringified), and `to_ordinal(±0.0)`
//! truncates to 0 and hits the quirk-2 IndexError. [`mn_trunc_int`] ports the
//! `int()` cast, including `int(inf)` -> OverflowError / `int(nan)` ->
//! ValueError for completeness (the dispatcher filters non-finite floats
//! before the core is reached).
//!
//! `to_year` tests **`value < 0`** — a numeric comparison, unlike the
//! cardinal's `n.startswith("-")` — so `to_year(-0.0)` takes **no** "МЭӨ "
//! prefix; the "-" of "-0.0" then reaches the cardinal, whose `int(right)==0`
//! early return drops it: "тэг он". A genuinely negative value is `abs()`ed
//! *before* `to_cardinal(value, all_suffixed=True)`, so "МЭӨ нэг, аравны
//! таван он" carries no "хасах". And because `to_year` re-enters the
//! stringifying cardinal, `to_year(1e+16)` **does** raise the ValueError that
//! `to_ordinal(1e+16)` avoids.
//!
//! # String inputs and the `Infinity` interception
//!
//! MN does not override `str_to_number`; Base's `Decimal(value)` applies. But
//! the binding hardwires `ParsedNumber::Inf` to OverflowError — the base-
//! language outcome of `int(Decimal("Infinity"))` — before any language hook
//! runs, while MN's `to_cardinal` reaches `int("Infinity")` and raises
//! **ValueError** instead. The only interception point the core owns is
//! [`Lang::str_to_number`], so MN's override turns the Inf parse itself into
//! that ValueError. Known residual: a *non-cardinal* mode fed the string
//! "Infinity" would raise OverflowError in Python (via `int(Decimal)`), but
//! gets this ValueError here — no corpus row exercises that combination.
//!
//! # Grammatical kwargs
//!
//! `to_cardinal(value, all_suffixed=False)` is the only extended signature
//! (`to_ordinal`/`to_ordinal_num`/`to_year` take no extras; `to_currency`
//! adds nothing beyond the standard four). `to_cardinal_kw` and
//! `to_cardinal_float_kw` accept exactly `all_suffixed`, applying Python
//! truthiness (None/0/""/[] are falsy — the source only ever tests the flag).
//! `to_cardinal(-5, all_suffixed=True)` is "хасах тав": quirk 1 drops the
//! flag across the negative recursion.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, prefix_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, python_int_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const ZERO: &str = "тэг";

/// `setup()`: `self.negword = "хасах"`.
const NEGWORD: &str = "хасах";

/// `to_year`'s BCE prefix. The trailing space is part of the literal.
const BCE_PREFIX: &str = "МЭӨ ";

/// `to_year`'s trailing unit word.
const YEAR_WORD: &str = "он";

/// `ONES`: (free form, suffixed form). Index 0 is absent in Python — every
/// read is guarded by `n1 > 0` or `n3 > 1`, so the placeholder is never used.
const ONES: [(&str, &str); 10] = [
    ("", ""),
    ("нэг", "нэг"),
    ("хоёр", "хоёр"),
    ("гурав", "гурван"),
    ("дөрөв", "дөрвөн"),
    ("тав", "таван"),
    ("зургаа", "зургаан"),
    ("долоо", "долоон"),
    ("найм", "найман"),
    ("ес", "есөн"),
];

/// `TEN`: (free, suffixed). Used when the tens digit is exactly 1.
const TEN: (&str, &str) = ("арав", "арван");

/// `TWENTIES`: (free, suffixed), keys 2..=9 in Python. Indices 0 and 1 are
/// absent there; reads are guarded by `n2 > 1`.
const TWENTIES: [(&str, &str); 10] = [
    ("", ""),
    ("", ""),
    ("хорь", "хорин"),
    ("гуч", "гучин"),
    ("дөч", "дөчин"),
    ("тавь", "тавин"),
    ("жар", "жаран"),
    ("дал", "далан"),
    ("ная", "наян"),
    ("ер", "ерэн"),
];

/// `HUNDRED`: (free, suffixed).
const HUNDRED: (&str, &str) = ("зуу", "зуун");

/// `THOUSANDS`: chunk index → scale word, keys 1..=22 in Python (10^3 through
/// 10^66). Index 0 is a placeholder — `THOUSANDS[i]` is only read under
/// `if i > 0`. A chunk index of 23 or more (any value >= 10^69) was a
/// `KeyError` in Python; the MAXVAL check (#159) now rejects those values
/// with `OverflowError` before the lookup.
const THOUSANDS: [&str; 23] = [
    "",                       // unused (i == 0 never reaches the lookup)
    "мянга",                  // 10^3
    "сая",                    // 10^6
    "тэрбум",                 // 10^9
    "их наяд",                // 10^12
    "тунамал",                // 10^15
    "их ингүүмэл",            // 10^18
    "ялгаруулагч",            // 10^21
    "их өөр дээр",            // 10^24
    "хязгаар үзэгдэл",        // 10^27
    "их шалтгааны зүйл",      // 10^30
    "эрхт",                   // 10^33
    "их сайтар хүргэсэн",     // 10^36
    "живэх тоосон билэг",     // 10^39
    "их билэг тэмдэг",        // 10^42
    "тохио мэдэхүй",          // 10^45
    "их тийн болсон",         // 10^48
    "асрахуй",                // 10^51
    "их нигүүлсэнгүй",        // 10^54
    "тоолшгүй",               // 10^57
    "өгүүлшгүй",              // 10^60
    "үлэшгүй",                // 10^63
    "сэтгэшгүй",              // 10^66
];

/// The `THOUSANDS[1]` value, and the `words[-1]` needle for the `all_suffixed`
/// fixup in `_int2word`.
const THOUSAND_FREE: &str = "мянга";

/// What `THOUSAND_FREE` becomes when `all_suffixed` is set and it lands last.
const THOUSAND_SUFFIXED: &str = "мянган";

/// The default ordinal suffix.
const ORD_DUGAAR: &str = "дугаар";

/// The ordinal suffix for numbers whose significant final digit is 1, 4 or 9
/// (units digit), or 4 or 9 (tens digit, when the units digit is 0).
const ORD_DUGEER: &str = "дүгээр";

/// `POINT_WORDS`: the connective between whole and fractional part, keyed on
/// the **number of decimal places** (`len(right)`), 1..=6 in Python. Index 0 is
/// a placeholder — the fractional branch is only reached with 1..=6 digits (a
/// 0-digit fraction hits the integer `else`, and >6 raises). This is what makes
/// the MN float path irreducible to Base's `pointword`: Base emits one
/// `pointword` then each digit; MN emits *one* scale word chosen by digit count
/// and the fraction as a whole number. So 0.5 is "аравны тав" (tenths + five),
/// not "(.) тав".
const POINT_WORDS: [&str; 7] = [
    "",              // 0 unused
    "аравны",        // 1  tenths
    "зууны",         // 2  hundredths
    "мянганы",       // 3  thousandths
    "арван мянганы", // 4  ten-thousandths
    "зуун мянганы",  // 5  hundred-thousandths
    "саяны",         // 6  millionths
];

// These mirror crashes in lang_MN.py, not deliberate raises: the exception
// *type* is observable behaviour a caller may catch, so parity requires
// reproducing it rather than tidying it into a TypeError. The ordinal(0)
// IndexError is now fixed upstream (PR #661), so this helper is retained for
// its doc reference and any future faithfully-ported crash.
#[allow(dead_code)]
fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

fn key_error(key: usize) -> N2WError {
    N2WError::Key(key.to_string())
}

/// `THOUSANDS[i]`. Python raises `KeyError: i` for i > 22 (unreachable here
/// since the MAXVAL check, #159).
fn thousands(i: usize) -> Result<&'static str> {
    THOUSANDS.get(i).copied().ok_or_else(|| key_error(i))
}

/// Port of `utils.splitbyx(n, 3)` with `format_int=True`, for the
/// non-negative digit strings `_int2word` feeds it.
///
/// `_int2word` strips the sign before calling (`if n < 0` returns early), so
/// unlike some sibling modules this never sees a "-" and never trips the
/// `int("-")` ValueError that e.g. lang_PL does. `s` is guaranteed to be pure
/// ASCII digits from `BigInt::to_string()` on a non-negative value, so byte
/// indexing is safe here.
///
/// Chunks are at most 3 digits, hence always in 0..=999 — `u16` is provably
/// sufficient and no BigInt survives past this point.
fn splitbyx(s: &str) -> Vec<u16> {
    let length = s.len();
    let mut out: Vec<u16> = Vec::new();
    if length > 3 {
        let start = length % 3;
        if start > 0 {
            out.push(s[..start].parse().expect("ascii digits"));
        }
        let mut i = start;
        while i < length {
            out.push(s[i..i + 3].parse().expect("ascii digits"));
            i += 3;
        }
    } else {
        out.push(s.parse().expect("ascii digits"));
    }
    out
}

/// Port of `utils.get_digits(n)`.
///
/// Python builds `("%03d" % n)[-3:]`, reverses it, and unpacks to
/// `n1, n2, n3` — so n1 is the **units** digit, n2 the tens, n3 the hundreds.
/// Arithmetic gives the identical result for the 0..=999 chunks produced by
/// [`splitbyx`].
fn get_digits(x: u16) -> (u16, u16, u16) {
    (x % 10, (x / 10) % 10, (x / 100) % 10)
}

/// Port of `Num2Word_MN._int2word`.
fn int2word(n: &BigInt, all_suffixed: bool) -> Result<String> {
    check_maxval(n, maxval_ceiling())?;
    if n.is_negative() {
        // Python: `" ".join([self.negword, self._int2word(abs(n))])`.
        // `all_suffixed` is NOT forwarded — see quirk 1 in the module docs.
        return Ok(format!("{} {}", NEGWORD, int2word(&n.abs(), false)?));
    }

    if n.is_zero() {
        return Ok(ZERO.to_string());
    }

    let mut words: Vec<&'static str> = Vec::new();
    let chunks = splitbyx(&n.to_string());

    // Python: `i = len(chunks)`, then `i -= 1` at the top of each iteration —
    // so the first chunk sees i == len-1 and the last sees i == 0.
    let mut i = chunks.len();

    for x in chunks {
        i -= 1;

        if x == 0 {
            // Skips the THOUSANDS lookup too, so an all-zero chunk can never
            // raise KeyError on its own.
            continue;
        }

        let (n1, n2, n3) = get_digits(x);

        let (mut use_suffix1, mut use_suffix2, mut use_suffix3) = (true, true, true);
        if !all_suffixed && i == 0 {
            // Only the final chunk's *last* emitted word takes the free form.
            if n1 == 0 && n2 == 0 {
                use_suffix3 = false;
            } else if n1 == 0 {
                use_suffix2 = false;
            } else {
                use_suffix1 = false;
            }
        }

        if n3 > 0 {
            if n3 > 1 {
                words.push(ONES[n3 as usize].1);
            }
            words.push(if use_suffix3 { HUNDRED.1 } else { HUNDRED.0 });
        }

        if n2 == 1 {
            words.push(if use_suffix2 { TEN.1 } else { TEN.0 });
        } else if n2 > 1 {
            let t = TWENTIES[n2 as usize];
            words.push(if use_suffix2 { t.1 } else { t.0 });
        }

        if n1 > 0 {
            let o = ONES[n1 as usize];
            words.push(if use_suffix1 { o.1 } else { o.0 });
        }

        if i > 0 {
            words.push(thousands(i)?);
        }
    }

    // Python: `if all_suffixed and words and words[-1] == "мянга":
    //              words[-1] = "мянган"`.
    // A literal last-word compare; see quirk 4 in the module docs. The `words`
    // emptiness guard is dead code in Python (n == 0 returned early and every
    // other n has at least one non-zero chunk), but is kept for fidelity.
    if all_suffixed {
        if let Some(last) = words.last_mut() {
            if *last == THOUSAND_FREE {
                *last = THOUSAND_SUFFIXED;
            }
        }
    }

    Ok(words.join(" "))
}

/// Port of `Num2Word_MN._get_ordinal_suffix`.
///
/// Reproduces the `number_str[-2]` IndexError on "0" (quirk 2) and the
/// two-digit-only inspection window (quirk 3).
fn ordinal_suffix(number: &BigInt) -> Result<&'static str> {
    let number_str = number.to_string();
    // ASCII digits plus a possible leading '-', but index by chars() anyway
    // per the porting contract — never assume byte offsets line up.
    let chars: Vec<char> = number_str.chars().collect();

    let mut suffix = ORD_DUGAAR;

    // `number_str` is never empty: BigInt::to_string() always yields at least
    // one character, so [-1] cannot itself raise.
    let last = chars[chars.len() - 1];

    if last != '0' {
        if last == '1' || last == '4' || last == '9' {
            suffix = ORD_DUGEER;
        }
    } else if chars.len() > 1 {
        // PR savoirfairelinux/num2words#661: Python's `elif number_str[-2]`
        // indexed out of range on the single-character string "0"; guard the
        // tens-digit inspection with `len(number_str) > 1` so to_ordinal(0)
        // falls through to the default "дугаар" suffix instead of crashing.
        let second_last = chars[chars.len() - 2];
        if second_last != '0' && (second_last == '4' || second_last == '9') {
            suffix = ORD_DUGEER;
        }
        // Note the nesting: when the tens digit IS '0' the default "дугаар"
        // survives untouched — the hundreds digit is never consulted, so
        // to_ordinal(100) == "зуу дугаар" and to_ordinal(1000) ==
        // "нэг мянга дугаар".
    }

    Ok(suffix)
}

/// `int(float(n))` — Python's `int()` truncates toward zero. Reproduced by
/// truncating the f64 then formatting with 0 fractional digits (`{:.0}` never
/// uses scientific notation, so this survives large magnitudes). `f.trunc()`
/// yields an integer-valued double, so the format is exact rather than rounded.
fn f64_trunc_to_bigint(f: f64) -> Result<BigInt> {
    format!("{:.0}", f.trunc())
        .parse::<BigInt>()
        .map_err(|e| N2WError::Value(e.to_string()))
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the count of
/// fractional digits in Python's shortest round-trip `repr`. Rust's `{}` for
/// f64 is the same shortest round-trip contract, so counting the digits after
/// the point matches. Mirrors the private `floatpath::float_repr_precision`,
/// reimplemented here because that one is not `pub`.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Python's `str(value)` for the f64 arm — `repr(float)`: shortest
/// round-trip digits, fixed-point with at least one fractional digit,
/// switching to exponent form at CPython's thresholds (decimal exponent
/// >= 16 or < -4: `str(1e16)` is "1e+16", `str(0.00001)` is "1e-05", but
/// `str(0.0001)` is "0.0001").
fn python_float_str(f: f64, precision: u32) -> String {
    if f.is_nan() {
        // str(float("nan")) is "nan" whatever the sign bit says.
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }
    if f != 0.0 {
        // Rust {:e} is shortest-round-trip "d[.ddd]e<exp>", the same digit
        // string CPython's repr computes. CPython picks exponent form iff
        // exp >= 16 or exp < -4, then always signs the exponent and pads it
        // to at least two digits ("1e+16", "1e-05").
        let sci = format!("{:e}", f);
        let (mant, exp) = sci.split_once('e').expect("LowerExp always has an e");
        let exp: i32 = exp.parse().expect("exponent is an integer");
        if !(-4..16).contains(&exp) {
            return format!(
                "{}e{}{:02}",
                mant,
                if exp < 0 { "-" } else { "+" },
                exp.abs()
            );
        }
    }
    // Fixed-point. `precision` is Python's fractional-digit count of this
    // very repr (`abs(Decimal(str(number)).as_tuple().exponent)`, computed by
    // the binding), so formatting to that many places recovers the repr
    // exactly. repr never prints fewer than one fractional digit ("21.0",
    // "-0.0"), hence the max(1) backstop.
    format!("{:.*}", precision.max(1) as usize, f)
}

/// Python's `n = str(value)` — the string `Num2Word_MN.to_cardinal` routes
/// on. The Decimal arm is `strnum::python_decimal_str`, byte-exact including
/// E-notation ("1E+3", "1E-7") and preserved trailing zeros ("5.00").
fn mn_value_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => python_float_str(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Port of `Num2Word_MN.to_cardinal(value, all_suffixed=...)` — the whole
/// method, both arms, running on the reconstructed `str(value)`.
fn mn_to_cardinal_str(n: &str, all_suffixed: bool) -> Result<String> {
    // Python: `n = str(value).replace(",", ".")` — no reconstructed repr ever
    // contains a comma, so the replace is a no-op here.
    if n.contains('.') {
        // is_negative = n.startswith("-"); the sign-bit test upstream matches
        // this for a signed zero too (str(-0.0) == "-0.0"), though the sign
        // is then dropped by the int(right)==0 branch, exactly as in Python.
        let is_negative = n.starts_with('-');
        let abs_n = if is_negative { &n[1..] } else { n };
        let (left, right) = abs_n.split_once('.').expect("checked: contains '.'");

        // `if int(right) == 0:` runs BEFORE the `len(right) > 6` raise, so
        // Decimal("1.00000000") is "нэг" rather than NotImplementedError.
        // `right` is pure ASCII digits from a repr, so the parse cannot fail.
        let right_int = right
            .parse::<BigInt>()
            .map_err(|e| N2WError::Value(e.to_string()))?;
        if right_int.is_zero() {
            // `return self._int2word(int(float(n)), all_suffixed=...)` —
            // re-cast through f64: signed, truncated toward zero, and lossy
            // for huge integers, exactly as in Python.
            let f: f64 = n
                .parse()
                .map_err(|e: std::num::ParseFloatError| N2WError::Value(e.to_string()))?;
            return int2word(&f64_trunc_to_bigint(f)?, all_suffixed);
        }

        // `fractional_length = len(right); if fractional_length > 6: raise
        // NotImplementedError()` — empty message, matching the bare raise.
        if right.len() > 6 {
            return Err(N2WError::NotImplemented(String::new()));
        }

        let left_int = left
            .parse::<BigInt>()
            .map_err(|e| N2WError::Value(e.to_string()))?;
        let body = format!(
            "{}, {} {}",
            int2word(&left_int, all_suffixed)?,
            POINT_WORDS[right.len()],
            int2word(&right_int, all_suffixed)?,
        );
        return Ok(if is_negative {
            // Python: `self.negword + " " + result`.
            format!("{} {}", NEGWORD, body)
        } else {
            body
        });
    }
    // `else: return self._int2word(int(n), ...)`. int(str) accepts an
    // optional sign plus digits — scientific reprs ("1e+16", "1E+2") and
    // non-finite ones ("inf", "Infinity", "nan") raise the ValueError the
    // corpus pins, with Python's exact message.
    match python_int_parse(n) {
        Some(i) => int2word(&i, all_suffixed),
        None => Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            n
        ))),
    }
}

/// `Num2Word_MN.to_cardinal` on a float/Decimal, `all_suffixed` threaded.
fn mn_to_cardinal(v: &FloatValue, all_suffixed: bool) -> Result<String> {
    mn_to_cardinal_str(&mn_value_str(v), all_suffixed)
}

/// Python's `int(value)` on a float/Decimal — the opening line of MN's
/// `to_ordinal`/`to_ordinal_num`. Truncates toward zero; `int(inf)` raises
/// OverflowError and `int(nan)` ValueError (unreachable from the dispatcher,
/// which keeps non-finite floats on the Python side, but ported anyway).
fn mn_trunc_int(v: &FloatValue) -> Result<BigInt> {
    match v {
        FloatValue::Float { value, .. } => {
            if value.is_nan() {
                return Err(N2WError::Value(
                    "cannot convert float NaN to integer".into(),
                ));
            }
            if value.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
            f64_trunc_to_bigint(*value)
        }
        // `with_scale(0)` truncates toward zero, as in floatpath.rs.
        FloatValue::Decimal { value, .. } => Ok(value.with_scale(0).as_bigint_and_exponent().0),
    }
}

/// Python truthiness for the `all_suffixed` kwarg: the source only ever
/// tests the flag (`if all_suffixed`, `if (not all_suffixed) and i == 0`), so
/// an explicit None — which Python treats as falsy — behaves like the
/// default, as do 0/""/[].
fn kw_truthy(v: Option<&KwVal>) -> bool {
    match v {
        None | Some(KwVal::None) => false,
        Some(KwVal::Bool(b)) => *b,
        Some(KwVal::Int(i)) => *i != 0,
        Some(KwVal::Str(s)) => !s.is_empty(),
        Some(KwVal::List(l)) => !l.is_empty(),
    }
}

/// The sub-unit divisor `to_currency` uses for **every** code.
///
/// `Num2Word_MN.to_currency` hardcodes it twice — `(decimal_val * 100) % 1`
/// and the `divisor=100` default it lets `parse_currency_parts` supply — and
/// never consults `CURRENCY_PRECISION`. See quirk 7 in the module docs.
const DIVISOR: i64 = 100;

/// `Num2Word_MN.CURRENCY_FORMS`, 36 codes.
///
/// MN declares this as its own class attribute, so it is **not** the shared
/// `Num2Word_EUR` dict that `Num2Word_EN.__init__` mutates in place: the
/// lang_EUR trap in PORTING_CURRENCY.md does not apply here. Verified against
/// the live interpreter — MN's EUR is `("евро",)`, not English's
/// `("euro", "euros")`, and EN's ~24 added codes (BHD, OMR, ...) are absent.
///
/// Every entry is a **1-tuple** on both sides. That arity is load-bearing:
/// `pluralize` returns `form[0]` unconditionally, so a second form would be
/// dead — but adding one would still change nothing, and dropping the tuple
/// shape would. Kept at exactly the arity Python has.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AED", CurrencyForms::new(&["дирхам"], &["филс"]));
    m.insert("AUD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("BGN", CurrencyForms::new(&["лев"], &["стотинка"]));
    m.insert("CAD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("CHF", CurrencyForms::new(&["франк"], &["раппен"]));
    m.insert("CNY", CurrencyForms::new(&["юань"], &["фэнь"]));
    m.insert("CZK", CurrencyForms::new(&["крон"], &["галерж"]));
    m.insert("DKK", CurrencyForms::new(&["крон"], &["өре"]));
    m.insert("EGP", CurrencyForms::new(&["фунт"], &["пиастр"]));
    m.insert("EUR", CurrencyForms::new(&["евро"], &["цент"]));
    m.insert("GBP", CurrencyForms::new(&["фунт стерлинг"], &["пенс"]));
    m.insert("HKD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("HUF", CurrencyForms::new(&["форинт"], &["филлер"]));
    m.insert("IDR", CurrencyForms::new(&["рупи"], &["сен"]));
    m.insert("INR", CurrencyForms::new(&["рупи"], &["пайса"]));
    m.insert("JPY", CurrencyForms::new(&["иен"], &["сен"]));
    m.insert("KPW", CurrencyForms::new(&["вон"], &["чон"]));
    m.insert("KRW", CurrencyForms::new(&["вон"], &["чон"]));
    // "динaр" carries a LATIN "a" (U+0061), not Cyrillic "а" — quirk 11.
    m.insert("KWD", CurrencyForms::new(&["динaр"], &["филс"]));
    m.insert("KZT", CurrencyForms::new(&["тенге"], &["тийн"]));
    m.insert("MNT", CurrencyForms::new(&["төгрөг"], &["мөнгө"]));
    m.insert("MYR", CurrencyForms::new(&["ринггит"], &["сен"]));
    m.insert("NOK", CurrencyForms::new(&["крон"], &["өре"]));
    m.insert("NPR", CurrencyForms::new(&["рупи"], &["пайса"]));
    m.insert("NZD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("PLN", CurrencyForms::new(&["злот"], &["грош"]));
    m.insert("RUB", CurrencyForms::new(&["рубль"], &["копейк"]));
    m.insert("SEK", CurrencyForms::new(&["крон"], &["өре"]));
    m.insert("SGD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("THB", CurrencyForms::new(&["бат"], &["сатанг"]));
    m.insert("TRY", CurrencyForms::new(&["лира"], &["куруш"]));
    m.insert("TWD", CurrencyForms::new(&["доллар"], &["сентао"]));
    m.insert("UAH", CurrencyForms::new(&["гривн"], &["копейк"]));
    m.insert("USD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("VND", CurrencyForms::new(&["донг"], &["су"]));
    m.insert("ZAR", CurrencyForms::new(&["ранд"], &["сента"]));
    m
}

/// `Num2Word_MN.CURRENCY_ADJECTIVES`, 16 codes.
///
/// Note "Чехийн " — the trailing space is in the Python source and produces a
/// double-spaced "Чехийн  крон" once `prefix_currency` joins with `"%s %s"`.
/// See quirk 10.
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Австралийн"),
        ("CAD", "Канадын"),
        ("CZK", "Чехийн "),
        ("DKK", "Данийн"),
        ("HKD", "Хонконг"),
        ("IDR", "Индонезийн"),
        ("INR", "Энэтхэгийн"),
        ("KPW", "БНАСАУ-ын"),
        ("KRW", "БНСУ-ын"),
        ("NOK", "Норвегийн"),
        ("NPR", "Непалын"),
        ("NZD", "Шинэ Зеландын"),
        ("SEK", "Шведийн"),
        ("SGD", "Сингапур"),
        ("TWD", "Тайванийн"),
        ("USD", "Америк"),
    ]
    .into_iter()
    .collect()
}

pub struct LangMn {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangMn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangMn {
    pub fn new() -> Self {
        LangMn {
            // Built once here, never per call. `to_currency` and `to_cheque`
            // only ever read these; rebuilding 36 entries per conversion is
            // what made an earlier revision of this port slower than the
            // Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^66, so 10^69 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(69))
}

impl Lang for LangMn {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// `self.pointword`, read from the live Python instance.
    /// Unused by the four integer modes, so phase 1 never needed
    /// it — the float path is its first caller.
    fn pointword(&self) -> &str {
        "(.)"
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MNT"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    /// `setup()`: `self.negword = "хасах"`.
    ///
    /// Unused by the code below — `int2word` reaches for [`NEGWORD`] directly,
    /// mirroring `_int2word`'s `self.negword` — but overridden so the trait
    /// default "(-) " never leaks out of this language.
    fn negword(&self) -> &str {
        NEGWORD
    }

    /// Port of `to_cardinal(value)` for integral input.
    ///
    /// Python does `n = str(value).replace(",", ".")` and branches on whether
    /// "." is present. An integer never stringifies with a ".", so control
    /// always lands in the `else` arm: `self._int2word(int(n))`. The
    /// `POINT_WORDS` fractional branch above it is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        int2word(value, false)
    }

    /// Port of `to_ordinal(value)`:
    /// `"%s %s" % (self.to_cardinal(number), self._get_ordinal_suffix(number))`.
    ///
    /// Python evaluates the cardinal first, so when both would raise (they
    /// never do for the same input) the cardinal's exception wins. Order is
    /// preserved here regardless.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        let suffix = ordinal_suffix(value)?;
        Ok(format!("{} {}", cardinal, suffix))
    }

    /// Port of `to_ordinal_num(value)`:
    /// `"%s %s" % (number, self._get_ordinal_suffix(number))`.
    ///
    /// `number` is `int(value)`, so it renders as plain decimal digits — the
    /// sign is kept, giving "-1 дүгээр".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        let suffix = ordinal_suffix(value)?;
        Ok(format!("{} {}", value, suffix))
    }

    /// Port of `to_year(value)`.
    ///
    /// Negatives are turned positive and prefixed "МЭӨ " *before* the cardinal
    /// call, so `_int2word`'s negative branch — and with it the dropped
    /// `all_suffixed` of quirk 1 — is unreachable from here.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut prefix = "";
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            prefix = BCE_PREFIX;
        }
        // Python: `self.to_cardinal(value, all_suffixed=True)`, which for an
        // integer reduces to `_int2word(value, all_suffixed=True)`.
        Ok(format!("{}{} {}", prefix, int2word(&v, true)?, YEAR_WORD))
    }

    // ---- currency -------------------------------------------------------
    //
    // MN overrides `to_currency`, `_money_verbose`, `_cents_verbose` and
    // `pluralize`, and inherits `to_cheque` and `_cents_terse` from
    // `Num2Word_Base`. `CURRENCY_PRECISION` is left at Base's empty dict, so
    // `currency_precision()` stays at its trait default of 100 for every code
    // — which is also what `default_to_cheque` reads.

    fn lang_name(&self) -> &str {
        "Num2Word_MN"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// MN's own signature default: `to_currency(..., adjective=True)` —
    /// the only language in the library that defaults it on.
    fn default_adjective(&self) -> bool {
        true
    }

    /// `Num2Word_MN.pluralize`: `return form[0]` — the count is ignored.
    ///
    /// Mongolian does not inflect the noun after a numeral, so every
    /// `CURRENCY_FORMS` entry is a 1-tuple and this always yields the only
    /// form. Python would raise IndexError on an empty tuple; that is
    /// unreachable with MN's table but mapped rather than panicked so the
    /// exception type survives if the table ever changes.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .first()
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `_money_verbose`: `self.to_cardinal(number, all_suffixed=True)`.
    ///
    /// `number` is always a non-negative int here (`parse_currency_parts`
    /// strips the sign, and `to_cheque` passes `int(abs_val)`), so the dropped
    /// `all_suffixed` of quirk 1 is unreachable through this hook.
    fn money_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        int2word(number, true)
    }

    /// `_cents_verbose`: `self.to_cardinal(number, all_suffixed=True)`.
    fn cents_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        int2word(number, true)
    }

    /// Port of `Num2Word_MN.to_cardinal(float/Decimal)` — the **full** entry.
    ///
    /// MN overrides `to_cardinal` outright, so the whole-value routing is its
    /// own too: Base's `int(value) == value` test never runs. `str(value)`
    /// decides everything — "5.00" takes the fraction branch and its
    /// `int(right)==0` early return ("тав"), while "1e+16"/"1E+2"/"Infinity"
    /// (no ".") fall to `int(str)` and raise ValueError. See the module docs.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored,
    /// as in Python: MN's `to_cardinal` takes no `precision` argument and
    /// never reads `self.precision`, so the dispatcher's
    /// `converter.precision = …` mutation is a no-op for it. Verified against
    /// the live interpreter (`precision=1` and `precision=4` both leave 2.675
    /// unchanged).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        mn_to_cardinal(value, false)
    }

    /// MN's raw float grammar is the same routine as the full entry — Python
    /// has a single `to_cardinal` handling both — kept overridden so nothing
    /// reaches Base's `float2tuple` dialect ("жаран долоо (.) тав" instead of
    /// "жаран долоо, аравны тав").
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        mn_to_cardinal(value, false)
    }

    /// Port of `to_ordinal(float/Decimal)`: `number = int(value)` — plain
    /// truncation toward zero, then the integer body. So 2.5 -> "хоёр
    /// дугаар", 1e+16 -> "арван тунамал дугаар" (the float is never
    /// stringified, so no ValueError here), and ±0.0 -> 0 -> the quirk-2
    /// IndexError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let n = mn_trunc_int(value)?;
        self.to_ordinal(&n)
    }

    /// Port of `to_ordinal_num(float/Decimal)`: `number = int(value)`, then
    /// `"%s %s" % (number, suffix)` — so "-1.5" renders "-1 дүгээр" and
    /// Decimal("1E+20") renders all 21 digits.
    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        let n = mn_trunc_int(value)?;
        self.to_ordinal_num(&n)
    }

    /// Port of `to_year(float/Decimal)`.
    ///
    /// `if value < 0` is a numeric comparison — NOT the cardinal's
    /// `startswith("-")` — so -0.0 takes no "МЭӨ " prefix and its stray "-"
    /// is later dropped by the `int(right)==0` early return ("тэг он"). A
    /// negative value is `abs()`ed *before* `to_cardinal(value,
    /// all_suffixed=True)`, so the cardinal never sees the sign. Because this
    /// re-enters the stringifying cardinal, `to_year(1e+16)` raises the
    /// ValueError that `to_ordinal(1e+16)` avoids.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let lt_zero = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            // BigDecimal has no signed zero, so is_negative() is exactly < 0.
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        let (v, prefix) = if lt_zero {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: value.abs(),
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: value.abs(),
                    precision: *precision,
                },
            };
            (abs, BCE_PREFIX)
        } else {
            (value.clone(), "")
        };
        Ok(format!("{}{} {}", prefix, mn_to_cardinal(&v, true)?, YEAR_WORD))
    }

    /// Base `str_to_number` (`Decimal(value)`) with the Inf parse turned into
    /// the ValueError MN's `to_cardinal` would raise on `int("Infinity")`.
    ///
    /// The binding hardwires `ParsedNumber::Inf` to OverflowError — correct
    /// for base languages, whose `int(Decimal("Infinity"))` raises it — before
    /// any other language hook runs, so this parse-time interception is the
    /// only way to surface MN's ValueError. Residual divergence: a
    /// non-cardinal mode fed "Infinity" would raise OverflowError in Python
    /// but ValueError here; no corpus row exercises that combination.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}Infinity'",
                if negative { "-" } else { "" },
            ))),
            other => Ok(other),
        }
    }

    /// `to_cardinal(value, all_suffixed=False)` — the one extended signature
    /// in `Num2Word_MN` (ordinal/ordinal_num/year take no extras, and
    /// to_currency adds nothing beyond the standard four).
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["all_suffixed"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        // Integer input always lands in `_int2word(int(n), all_suffixed)`.
        // -5 with all_suffixed=True is still "хасах тав": quirk 1 drops the
        // flag across the negative recursion inside int2word.
        int2word(value, kw_truthy(kw.get("all_suffixed")))
    }

    /// The float/Decimal cardinal with kwargs: same single `all_suffixed`
    /// flag, threaded through both branches of the string algorithm exactly
    /// as Python's `to_cardinal(value, all_suffixed=...)` does.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["all_suffixed"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        mn_to_cardinal(value, kw_truthy(kw.get("all_suffixed")))
    }

    /// Port of the currency path's `self.to_cardinal(float(right))`.
    ///
    /// The fractional-cents branch of `Num2Word_MN.to_currency` casts the
    /// Decimal sub-unit to a **float** first (`float(right)`), then runs the
    /// float branch of `to_cardinal`. So this renders through the f64 arm — not
    /// the exact Decimal arm — deriving the precision from the resulting float's
    /// repr, exactly as `to_cardinal(67.5)` would. Reproduces the correct
    /// dialect ("жаран долоо, аравны тав") in-core, so the currency path no
    /// longer needs to defer to the Python converter for fractional cents.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        let precision = float_repr_precision(f);
        mn_to_cardinal(&FloatValue::Float { value: f, precision }, false)
    }

    /// Port of `Num2Word_MN.to_currency`.
    ///
    /// MN replaces `Num2Word_Base.to_currency` wholesale rather than extending
    /// it, and the two differ in ways the corpus pins:
    ///
    /// * an `int` returns a bare cardinal with no unit word and no table
    ///   lookup (quirk 6) — so an unknown code cannot raise on that path;
    /// * the divisor is hardcoded to 100 (quirk 7);
    /// * the cents segment is gated on `right > 0` alone, with no `has_decimal`
    ///   guard (quirk 9);
    /// * `cents=False` emits `str(right)`, not `_cents_terse` (quirk 8).
    ///
    /// # The `adjective` parameter is deliberately ignored
    ///
    /// `Num2Word_MN.to_currency` is the **only** converter in the library whose
    /// `adjective` parameter defaults to `True` (all 148 others default to
    /// `False`, matching Base). The shim resolves that default on the Python
    /// side with a hardcoded `kwargs.get("adjective", False)` — Base's default,
    /// not the language's — so by the time the core is called, "caller omitted
    /// `adjective`" (Python: apply it) and "caller passed `adjective=False`"
    /// (Python: skip it) both arrive here as `false` and are indistinguishable.
    ///
    /// `separator` has the same problem and solves it with `Option<&str>` plus
    /// [`Lang::default_separator`]; there is no `default_adjective()` hook, and
    /// `base.rs` is out of bounds for this change. So this body applies MN's
    /// `adjective=True` default unconditionally, which reproduces Python for
    /// every call that omits the kwarg — i.e. the corpus, and every route the
    /// shim actually takes. `num2words(12.34, lang="mn", to="currency",
    /// currency="USD")` is "арван хоёр Америк доллар, гучин дөрвөн цент" in
    /// Python and here; honouring the incoming `false` would drop "Америк" and
    /// fail 14 corpus rows (the USD and INR floats).
    ///
    /// The residual divergence is an **explicit** `adjective=False`, which
    /// Python honours and this cannot see. Flagged rather than papered over;
    /// the fix is a `default_adjective()` hook mirroring `default_separator()`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        let d = match val {
            // `if isinstance(val, int): return self.to_cardinal(val)` — the
            // whole method, for an int. The sign is kept (to_cardinal(-5) is
            // "хасах тав"), `all_suffixed` is not passed, and `currency` is
            // never looked at. Quirk 6.
            CurrencyValue::Int(v) => return int2word(v, false),
            CurrencyValue::Decimal { value, .. } => value,
        };

        // MN's own `separator=","` default, applied when the caller omitted
        // the kwarg. Same value as Base's here, but read through the hook
        // rather than hardcoded.
        let separator = separator.unwrap_or(self.default_separator());

        // `has_fractional_cents = (decimal_val * 100) % 1 != 0`. Decimal's `%`
        // truncates toward zero, and so does `with_scale(0)`, so the two agree
        // on negatives: -12.34 * 100 is -1234.00, remainder -0.00, which is 0.
        let scaled = d * BigDecimal::from(DIVISOR);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        // `parse_currency_parts(val, is_int_with_cents=False,
        //                       keep_precision=has_fractional_cents)`
        // — divisor comes from the callee's default of 100.
        let (left, right, is_negative) =
            parse_currency_parts(val, false, has_fractional_cents, DIVISOR);

        // The lookup happens *after* parsing, exactly as in Python. Only
        // reachable for a float, so an unknown code raises here but not on the
        // int path above.
        let forms = self.currency_forms(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        let mut cr1 = forms.unit.clone();
        let cr2 = forms.subunit.clone();
        // `if adjective and currency in self.CURRENCY_ADJECTIVES`. The param
        // arrives already resolved: the binding substitutes MN's own default
        // (True, via default_adjective) when the caller omitted the kwarg,
        // so an explicit adjective=False is honoured — test_mn passes one.
        if adjective {
            if let Some(adj) = self.currency_adjective(currency) {
                cr1 = prefix_currency(adj, &cr1);
            }
        }

        // `"%s " % self.negword.strip()`. NEGWORD has no padding to strip.
        let minus_str = if is_negative {
            format!("{} ", NEGWORD)
        } else {
            String::new()
        };
        let money_str = self.money_verbose(&left, currency)?;

        // `if right > 0:` — `right` is non-negative (parse_currency_parts took
        // abs first), so this is purely "are there any sub-units". No
        // `has_decimal` guard, hence 1.0 -> "нэг евро". Quirk 9.
        if right <= BigDecimal::zero() {
            return Ok(format!(
                "{}{} {}",
                minus_str,
                money_str,
                self.pluralize(&left, &cr1)?
            ));
        }

        // `isinstance(right, Decimal)` is exactly `keep_precision`: Python's
        // parse_currency_parts returns `int(fraction * divisor)` when it is
        // false and a raw Decimal when true.
        let cents_str = if has_fractional_cents {
            // Python: `self.to_cardinal(float(right)) if cents else
            //          str(float(right))`, i.e. the float/POINT_WORDS branch.
            // Out of scope per PORTING_CURRENCY.md. Both arms raise
            // NotImplemented, which `num2words()` catches and falls back to the
            // Python converter for. NB `cardinal_from_decimal` is overridden
            // above precisely so this does NOT reach Base's float default,
            // which would emit the wrong dialect instead of deferring.
            if cents {
                self.cardinal_from_decimal(&right)?
            } else {
                return Err(N2WError::NotImplemented(format!(
                    "str(float(...)) repr of fractional sub-units not \
                     implemented for {} ({})",
                    self.lang_name(),
                    right
                )));
            }
        } else {
            // `right` went through `.with_scale(0)`, so exponent 0 and the
            // mantissa is the value.
            let right_int = right.as_bigint_and_exponent().0;
            if cents {
                self.cents_verbose(&right_int, currency)?
            } else {
                // `str(right)` on a Python int — NOT `_cents_terse`, so no
                // zero padding. Quirk 8.
                right_int.to_string()
            }
        };

        // `self.pluralize(right, cr2)` — MN passes `right` itself, which is a
        // Decimal on the fractional path. `pluralize` ignores its count
        // entirely, so the BigInt/BigDecimal split cannot change the output.
        //
        // Note for whoever implements `cardinal_from_decimal`: this line is
        // currently unreachable on the fractional path (the `?` above bails
        // first). Once it is reachable, `as_bigint_and_exponent().0` yields the
        // *mantissa*, not the value — Decimal("1.100") gives 1100. Harmless
        // while `pluralize` discards the count, but it is a trap if that ever
        // changes.
        let right_int = right.as_bigint_and_exponent().0;
        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str,
            money_str,
            self.pluralize(&left, &cr1)?,
            separator,
            cents_str,
            self.pluralize(&right_int, &cr2)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn f(v: f64, p: u32) -> Result<String> {
        LangMn::new().to_cardinal_float(&FloatValue::Float { value: v, precision: p }, None)
    }

    fn dec(s: &str, p: u32) -> Result<String> {
        LangMn::new().to_cardinal_float(
            &FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: p },
            None,
        )
    }

    /// Every `to: "cardinal"` corpus row whose `arg` has a dot (float input).
    /// `precision` is Python's `abs(Decimal(str(value)).as_tuple().exponent)`.
    #[test]
    fn float_cardinal_corpus() {
        assert_eq!(f(0.0, 1).unwrap(), "тэг");
        assert_eq!(f(0.5, 1).unwrap(), "тэг, аравны тав");
        assert_eq!(f(1.0, 1).unwrap(), "нэг");
        assert_eq!(f(1.5, 1).unwrap(), "нэг, аравны тав");
        assert_eq!(f(2.25, 2).unwrap(), "хоёр, зууны хорин тав");
        assert_eq!(f(3.14, 2).unwrap(), "гурав, зууны арван дөрөв");
        assert_eq!(f(0.01, 2).unwrap(), "тэг, зууны нэг");
        assert_eq!(f(0.1, 1).unwrap(), "тэг, аравны нэг");
        assert_eq!(f(0.99, 2).unwrap(), "тэг, зууны ерэн ес");
        assert_eq!(f(1.01, 2).unwrap(), "нэг, зууны нэг");
        assert_eq!(f(12.34, 2).unwrap(), "арван хоёр, зууны гучин дөрөв");
        assert_eq!(f(99.99, 2).unwrap(), "ерэн ес, зууны ерэн ес");
        assert_eq!(f(100.5, 1).unwrap(), "зуу, аравны тав");
        assert_eq!(
            f(1234.56, 2).unwrap(),
            "нэг мянга хоёр зуун гучин дөрөв, зууны тавин зургаа"
        );
        assert_eq!(f(-0.5, 1).unwrap(), "хасах тэг, аравны тав");
        assert_eq!(f(-1.5, 1).unwrap(), "хасах нэг, аравны тав");
        assert_eq!(f(-12.34, 2).unwrap(), "хасах арван хоёр, зууны гучин дөрөв");
        // The f64-artefact cases: MN reads str(value), not float2tuple, so
        // 1.005 -> "005" -> 5 and 2.675 -> "675" -> 675 fall out of the repr.
        assert_eq!(f(1.005, 3).unwrap(), "нэг, мянганы тав");
        assert_eq!(
            f(2.675, 3).unwrap(),
            "хоёр, мянганы зургаан зуун далан тав"
        );
    }

    /// Every `to: "cardinal_dec"` corpus row (Decimal input).
    #[test]
    fn decimal_cardinal_corpus() {
        assert_eq!(dec("0.01", 2).unwrap(), "тэг, зууны нэг");
        // The trailing zero survives: str(Decimal("1.10")) == "1.10", right="10".
        assert_eq!(dec("1.10", 2).unwrap(), "нэг, зууны арав");
        assert_eq!(
            dec("12.345", 3).unwrap(),
            "арван хоёр, мянганы гурван зуун дөчин тав"
        );
        // #603: full precision, no f64 rounding at trillion scale.
        assert_eq!(
            dec("98746251323029.99", 2).unwrap(),
            "ерэн найман их наяд долоон зуун дөчин зургаан тэрбум хоёр зуун \
             тавин нэг сая гурван зуун хорин гурван мянга хорин ес, зууны ерэн ес"
        );
        assert_eq!(dec("0.001", 3).unwrap(), "тэг, мянганы нэг");
    }

    /// Branch ordering and boundaries confirmed against the live interpreter.
    #[test]
    fn edge_cases() {
        // int(right)==0 early return runs BEFORE the >6 raise.
        assert_eq!(dec("1.00000000", 8).unwrap(), "нэг");
        assert_eq!(dec("1.000000", 6).unwrap(), "нэг");
        assert_eq!(dec("12.000", 3).unwrap(), "арван хоёр");
        // Non-zero fraction beyond six places raises NotImplementedError().
        assert!(matches!(
            dec("1.10000000", 8),
            Err(N2WError::NotImplemented(m)) if m.is_empty()
        ));
        assert!(matches!(
            f(1.1234567, 7),
            Err(N2WError::NotImplemented(m)) if m.is_empty()
        ));
        // Signed zero: str(-0.0)=="-0.0" -> is_negative, but the zero-fraction
        // early return drops the sign, so "тэг".
        assert_eq!(f(-0.0, 1).unwrap(), "тэг");
        // cardinal_from_decimal casts to float first (currency fractional cents).
        assert_eq!(
            LangMn::new()
                .cardinal_from_decimal(&BigDecimal::from_str("67.5").unwrap())
                .unwrap(),
            "жаран долоо, аравны тав"
        );
    }

    fn fv(v: f64, p: u32) -> FloatValue {
        FloatValue::Float { value: v, precision: p }
    }

    fn dv(s: &str, p: u32) -> FloatValue {
        FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: p }
    }

    /// The full cardinal entry: whole-value routing is MN's own — `str(value)`
    /// with no "." goes to int(str), which raises ValueError on scientific
    /// reprs (the binding derives precision as abs(Decimal(str).exponent),
    /// hence 16/20/2 here — the e-form branch never reads it).
    #[test]
    fn cardinal_entry_corpus() {
        let l = LangMn::new();
        assert!(matches!(
            l.cardinal_float_entry(&fv(1e16, 16), None),
            Err(N2WError::Value(m)) if m == "invalid literal for int() with base 10: '1e+16'"
        ));
        assert!(matches!(
            l.cardinal_float_entry(&fv(1e20, 20), None),
            Err(N2WError::Value(m)) if m == "invalid literal for int() with base 10: '1e+20'"
        ));
        // #211: str(Decimal) is written out ("100"), so it reads.
        assert_eq!(
            l.cardinal_float_entry(&dv("1E+2", 2), None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );
        assert_eq!(
            format!("{:?}", l.cardinal_float_entry(&dv("1E+20", 20), None)),
            format!("{:?}", l.to_cardinal(&BigInt::from(10).pow(20)))
        );
        // Whole values still route through the fraction branch's early return
        // (visible ".") or the int arm (no ".").
        assert_eq!(l.cardinal_float_entry(&fv(5.0, 1), None).unwrap(), "тав");
        assert_eq!(l.cardinal_float_entry(&dv("5.00", 2), None).unwrap(), "тав");
        assert_eq!(l.cardinal_float_entry(&dv("100", 0), None).unwrap(), "зуу");
        assert_eq!(l.cardinal_float_entry(&fv(-0.0, 1), None).unwrap(), "тэг");
        assert_eq!(
            l.cardinal_float_entry(&dv("12345.000", 3), None).unwrap(),
            "арван хоёр мянга гурван зуун дөчин тав"
        );
    }

    /// `to_ordinal(float)`: int(value) truncation, then the integer body —
    /// so 1e+16 works here (never stringified) and ±0.0 is the IndexError.
    #[test]
    fn ordinal_entry_corpus() {
        let l = LangMn::new();
        assert_eq!(l.ordinal_float_entry(&fv(2.5, 1)).unwrap(), "хоёр дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(-1.5, 1)).unwrap(), "хасах нэг дүгээр");
        assert_eq!(l.ordinal_float_entry(&fv(3.25, 2)).unwrap(), "гурав дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(100.0, 1)).unwrap(), "зуу дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(-1000.0, 1)).unwrap(), "хасах нэг мянга дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(1e16, 16)).unwrap(), "арван тунамал дугаар");
        assert_eq!(
            l.ordinal_float_entry(&dv("1E+20", 20)).unwrap(),
            "зуун их ингүүмэл дугаар"
        );
        assert_eq!(l.ordinal_float_entry(&dv("5.00", 2)).unwrap(), "тав дугаар");
        // PR #661: to_ordinal(0) no longer raises IndexError; whole/truncated
        // zero floats now ordinalise to "тэг дугаар".
        assert_eq!(l.ordinal_float_entry(&fv(0.0, 1)).unwrap(), "тэг дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(-0.0, 1)).unwrap(), "тэг дугаар");
        assert_eq!(l.ordinal_float_entry(&fv(0.5, 1)).unwrap(), "тэг дугаар");
        assert_eq!(l.ordinal_float_entry(&dv("0", 0)).unwrap(), "тэг дугаар");
    }

    /// `to_ordinal_num(float)`: int(value), rendered as digits + suffix.
    #[test]
    fn ordinal_num_entry_corpus() {
        let l = LangMn::new();
        assert_eq!(
            l.ordinal_num_float_entry(&fv(-1000000.0, 1), "-1000000.0").unwrap(),
            "-1000000 дугаар"
        );
        assert_eq!(l.ordinal_num_float_entry(&fv(-1.5, 1), "-1.5").unwrap(), "-1 дүгээр");
        assert_eq!(
            l.ordinal_num_float_entry(&fv(1e16, 16), "1e+16").unwrap(),
            "10000000000000000 дугаар"
        );
        assert_eq!(
            l.ordinal_num_float_entry(&dv("1E+20", 20), "1E+20").unwrap(),
            "100000000000000000000 дугаар"
        );
        assert_eq!(l.ordinal_num_float_entry(&dv("5.00", 2), "5.00").unwrap(), "5 дугаар");
        // PR #661: to_ordinal_num(0) no longer raises; int(0.5)=int(-0.0)=0.
        assert_eq!(l.ordinal_num_float_entry(&fv(0.5, 1), "0.5").unwrap(), "0 дугаар");
        assert_eq!(l.ordinal_num_float_entry(&fv(-0.0, 1), "-0.0").unwrap(), "0 дугаар");
    }

    /// `to_year(float)`: `value < 0` (numeric — -0.0 takes no prefix), abs()
    /// before the all_suffixed cardinal, " он" appended.
    #[test]
    fn year_entry_corpus() {
        let l = LangMn::new();
        assert_eq!(l.year_float_entry(&fv(-1000.0, 1)).unwrap(), "МЭӨ нэг мянган он");
        assert_eq!(l.year_float_entry(&fv(-1000000.0, 1)).unwrap(), "МЭӨ нэг сая он");
        assert_eq!(l.year_float_entry(&fv(-0.0, 1)).unwrap(), "тэг он");
        assert_eq!(l.year_float_entry(&fv(0.5, 1)).unwrap(), "тэг, аравны таван он");
        assert_eq!(l.year_float_entry(&fv(-1.5, 1)).unwrap(), "МЭӨ нэг, аравны таван он");
        assert_eq!(l.year_float_entry(&fv(3.25, 2)).unwrap(), "гурван, зууны хорин таван он");
        assert_eq!(l.year_float_entry(&fv(99.0, 1)).unwrap(), "ерэн есөн он");
        assert_eq!(l.year_float_entry(&fv(1234.0, 1)).unwrap(), "нэг мянга хоёр зуун гучин дөрвөн он");
        assert_eq!(l.year_float_entry(&dv("5.00", 2)).unwrap(), "таван он");
        assert_eq!(l.year_float_entry(&dv("100", 0)).unwrap(), "зуун он");
        assert_eq!(l.year_float_entry(&dv("1.50", 2)).unwrap(), "нэг, зууны тавин он");
        // to_year re-enters the stringifying cardinal, so e-form raises.
        assert!(matches!(
            l.year_float_entry(&fv(1e16, 16)),
            Err(N2WError::Value(m)) if m == "invalid literal for int() with base 10: '1e+16'"
        ));
        assert_eq!(
            l.year_float_entry(&dv("1E+2", 2)).unwrap(),
            l.to_year(&BigInt::from(100)).unwrap()
        );
    }

    /// The `all_suffixed` kwarg, and the quirk-1 drop across the negative
    /// recursion (-5 stays "хасах тав" even when suffixed).
    #[test]
    fn all_suffixed_kwarg() {
        let l = LangMn::new();
        let t = Kwargs(vec![("all_suffixed".into(), KwVal::Bool(true))]);
        let f_ = Kwargs(vec![("all_suffixed".into(), KwVal::Bool(false))]);
        let none = Kwargs(vec![("all_suffixed".into(), KwVal::None)]);
        let bad = Kwargs(vec![("gender".into(), KwVal::Str("f".into()))]);
        assert_eq!(l.to_cardinal_kw(&BigInt::from(3), &t).unwrap(), "гурван");
        assert_eq!(l.to_cardinal_kw(&BigInt::from(100), &t).unwrap(), "зуун");
        assert_eq!(
            l.to_cardinal_kw(&BigInt::from(1234), &t).unwrap(),
            "нэг мянга хоёр зуун гучин дөрвөн"
        );
        assert_eq!(l.to_cardinal_kw(&BigInt::from(-5), &t).unwrap(), "хасах тав");
        assert_eq!(l.to_cardinal_kw(&BigInt::from(1234), &f_).unwrap(), "нэг мянга хоёр зуун гучин дөрөв");
        // all_suffixed=None behaves like the default (falsy).
        assert_eq!(l.to_cardinal_kw(&BigInt::from(3), &none).unwrap(), "гурав");
        assert!(matches!(
            l.to_cardinal_kw(&BigInt::from(3), &bad),
            Err(N2WError::Fallback(_))
        ));
        // Floats accept the flag too: Python's to_cardinal threads it into
        // both _int2word calls of the fraction branch.
        assert_eq!(
            l.to_cardinal_float_kw(&fv(1.5, 1), None, &t).unwrap(),
            "нэг, аравны таван"
        );
    }

    /// The Inf parse becomes the ValueError MN's `int("Infinity")` raises;
    /// everything else passes through Base's Decimal(value).
    #[test]
    fn str_to_number_infinity() {
        let l = LangMn::new();
        assert!(matches!(
            l.str_to_number("Infinity"),
            Err(N2WError::Value(m)) if m == "invalid literal for int() with base 10: 'Infinity'"
        ));
        assert!(matches!(
            l.str_to_number("-Infinity"),
            Err(N2WError::Value(m)) if m == "invalid literal for int() with base 10: '-Infinity'"
        ));
        assert!(matches!(l.str_to_number("1e3"), Ok(ParsedNumber::Dec(_))));
        assert!(matches!(l.str_to_number("NaN"), Ok(ParsedNumber::NaN)));
        // "1e3" parses to Decimal('1E+3'), written out as "1000" (#211).
        assert_eq!(
            l.cardinal_float_entry(&dv("1E+3", 3), None).unwrap(),
            l.to_cardinal(&BigInt::from(1000)).unwrap()
        );
    }
}
