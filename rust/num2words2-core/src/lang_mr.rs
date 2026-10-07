//! Port of `lang_MR.py` (Marathi).
//!
//! Shape: **self-contained**. `Num2Word_MR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. Every in-scope mode is
//! overridden outright and driven by `_int_to_word`, a plain recursive
//! descent over the Indian scale system (हजार / लाख / कोटी / अब्ज / खर्व).
//! Consequently `cards`/`merge` stay at their trait defaults here.
//!
//! **Python has no overflow check and no ceiling.** Because `MAXVAL` is never
//! set, the `value >= self.MAXVAL` guard in `Num2Word_Base.to_cardinal` is
//! never reached (the override skips it), and the top `खर्व` branch recurses
//! on `number // 10**11`, which re-enters the same branch for large enough
//! inputs. So in Python `10**22` is "एक खर्व खर्व" rather than an
//! `OverflowError`. On a large enough integer the port's recursion
//! overflowed the native stack, so it adds a ceiling
//! (gladiaio/num2words2#203): `maxval` is 10^22, where the multiplier would
//! itself need `खर्व`, and every mode raises `OverflowError` from there.
//! Below it none of the four in-scope modes can raise: every list index is
//! guarded by the range check that selects the branch, so there is no
//! `IndexError`/`KeyError` path here.
//!
//! Inherited from `Num2Word_Base`:
//!   * `setup()` sets only `negword = "ऋण "` (note the **trailing space**)
//!     and `pointword = "दशांश"`. Nothing else is touched, so `is_title`
//!     stays `False` — and MR's `to_cardinal` never calls `self.title`
//!     anyway.
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. The following all look wrong but are
//! exactly what Python emits, and every one is confirmed against the frozen
//! corpus:
//!
//! 1. **Tens and units (fixed, gladiaio/num2words2#247).** Python glued the
//!    unit onto "वीस" for 21..29 only ("एकवीस", "पाचवीस", "चारवीस") and
//!    emitted every other decade as two space-separated words: 31 was
//!    "तीस एक" (lit. "thirty one"), 42 "चाळीस दोन", 98 "नव्वद आठ", with 99
//!    alone hardcoded to "नव्याण्णव". The 20s compounds were malformed too:
//!    22 "दोनवीस" (Marathi बावीस), 23 "तीनवीस" (तेवीस). The port reads
//!    [`BELOW_HUNDRED`]: 31 is "एकतीस", 42 "बेचाळीस", 23 "तेवीस".
//!
//!    Items 2-4 of the old list (the malformed 20s, Python's dead
//!    `number == 21` arm and the lone 99 special case) went with it.
//! 5. **`to_ordinal` suffixes the raw cardinal**, so negatives and zero
//!    produce nonsense rather than raising: `to_ordinal(0)` == "शून्यवा",
//!    `to_ordinal(-21)` == "ऋण एकवीसवा", `to_ordinal(-1000)` ==
//!    "ऋण एक हजारवा". `Num2Word_Base.verify_ordinal` (which would raise
//!    `TypeError` on a negative) is never called.
//! 6. **`to_ordinal_num` mixed numeral systems (fixed, #224).** Python gave
//!    Devanagari digits for 1..4 ("१ला", "२रा", "३रा", "४था") and ASCII
//!    `str(number) + "वा"` for everything else ("5वा", "10वा", "-1वा"). The
//!    port writes ASCII digits throughout, the script of every other number:
//!    "1ला", "2रा", "3रा", "4था". 2 and 3 share the suffix "रा".
//! 7. **`to_year` ignores its `longval` parameter** and just prefixes the
//!    cardinal, so there is no two-digit-pair year reading: 1905 is
//!    "सन एक हजार नऊशे पाच", not "nineteen oh five". `to_year(0)` is
//!    "सन शून्य" (0 is not `< 0`).
//!
//! 8. **`to_currency` falls back to rupees for every unknown code** rather
//!    than raising, because it looks the code up with
//!    `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["INR"])`. The inherited
//!    `to_cheque` uses a strict `CURRENCY_FORMS[currency]` subscript instead,
//!    so the *same* code (JPY, CHF, ...) renders happily as रुपये through
//!    `to_currency` but raises NotImplementedError through `to_cheque`. Both
//!    halves of that contradiction are in the corpus.
//!
//! # Currency shape
//!
//! `to_currency` is overridden outright and shares nothing with
//! `Num2Word_Base`'s currency machinery — see [`LangMr::to_currency`].
//! `to_cheque` is *not* overridden, so it comes from `Num2Word_Base` and needs
//! only `lang_name` + `currency_forms` + the default `money_verbose` (which
//! routes to MR's `to_cardinal`) to reproduce
//! "एक हजार दोनशे चौतीस AND 56/100 युरो". `.upper()` is a no-op on
//! Devanagari — it is caseless — so only the literal "AND" looks upper-cased.
//! `pluralize` is never reached from either path and correctly keeps the
//! trait's raising default (`Num2Word_Base.pluralize` raises
//! NotImplementedError too).
//!
//! Note also that the `number < 0` arm of `_int_to_word` (which would return
//! `negword + _int_to_word(abs(n))` with **no** `.strip()`) is unreachable
//! from every in-scope entry point: `to_cardinal` detaches the sign before
//! calling in, and `to_ordinal`/`to_year` both route through `to_cardinal`.
//! It is reproduced in [`int_to_word`] for fidelity regardless.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword`, set by `setup()`. The trailing space is load-bearing:
/// `to_cardinal` concatenates it directly (`ret + word`) rather than joining
/// with a separator, unlike `Num2Word_Base`, which does `negword.strip() + " "`.
const NEGWORD: &str = "ऋण ";

/// `self.pointword`, set by `setup()`. Reached only on the float/Decimal
/// path, where `Num2Word_MR.to_cardinal` splices it between the integer and
/// the digit-by-digit fraction (see [`cardinal_from_str`]). `is_title` is
/// `False`, and MR uses `self.pointword` directly rather than `self.title(...)`,
/// so it is emitted verbatim.
const POINTWORD: &str = "दशांश";

const ZERO_WORD: &str = "शून्य";

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247).
///
/// Python had `ones`/`teens`/`tens`, glued a unit onto वीस for the 20s only
/// ("तीनवीस" for 23, Marathi तेवीस) and joined every other decade with a
/// space ("तीस एक" for 31), hardcoding only 99. Marathi has its own word for
/// every number below a hundred. Index 0 is never read (`number == 0`
/// returns first). 21..=98 follow the majority of four 1-100 lists
/// (superprof.co.in, yugmarathi.com, mahasarav.com, helpdiva.com; Wiktionary
/// for सव्वीस); 0..=20, the round tens and 99 नव्याण्णव are the module's own
/// words (the lists also spell 99 नव्व्याण्णव).
const BELOW_HUNDRED: [&str; 100] = [
    "", "एक", "दोन", "तीन", "चार", "पाच", "सहा", "सात", "आठ", "नऊ", // 0..9
    "दहा", "अकरा", "बारा", "तेरा", "चौदा", "पंधरा", "सोळा", "सतरा", "अठरा", "एकोणीस", // 10..19
    "वीस", "एकवीस", "बावीस", "तेवीस", "चोवीस", "पंचवीस", "सव्वीस", "सत्तावीस", "अठ्ठावीस", "एकोणतीस", // 20..29
    "तीस", "एकतीस", "बत्तीस", "तेहतीस", "चौतीस", "पस्तीस", "छत्तीस", "सदतीस", "अडतीस", "एकोणचाळीस", // 30..39
    "चाळीस", "एक्केचाळीस", "बेचाळीस", "त्रेचाळीस", "चव्वेचाळीस", "पंचेचाळीस", "सेहेचाळीस", "सत्तेचाळीस", "अठ्ठेचाळीस", "एकोणपन्नास", // 40..49
    "पन्नास", "एक्कावन्न", "बावन्न", "त्रेपन्न", "चोपन्न", "पंचावन्न", "छप्पन्न", "सत्तावन्न", "अठ्ठावन्न", "एकोणसाठ", // 50..59
    "साठ", "एकसष्ट", "बासष्ट", "त्रेसष्ट", "चौसष्ट", "पासष्ट", "सहासष्ट", "सदुसष्ट", "अडुसष्ट", "एकोणसत्तर", // 60..69
    "सत्तर", "एक्काहत्तर", "बाहत्तर", "त्र्याहत्तर", "चौऱ्याहत्तर", "पंच्याहत्तर", "शहात्तर", "सत्याहत्तर", "अठ्ठ्याहत्तर", "एकोणऐंशी", // 70..79
    "ऐंशी", "एक्क्याऐंशी", "ब्याऐंशी", "त्र्याऐंशी", "चौऱ्याऐंशी", "पंच्याऐंशी", "शहाऐंशी", "सत्त्याऐंशी", "अठ्ठ्याऐंशी", "एकोणनव्वद", // 80..89
    "नव्वद", "एक्क्याण्णव", "ब्याण्णव", "त्र्याण्णव", "चौऱ्याण्णव", "पंच्याण्णव", "शहाण्णव", "सत्त्याण्णव", "अठ्ठ्याण्णव", "नव्याण्णव", // 90..99
];

/// Suffix appended by the `< 1000` branch: `ones[number // 100] + "शे"`.
const HUNDRED_SUFFIX: &str = "शे";

/// Generic ordinal suffix appended to the cardinal for anything above 10.
const ORDINAL_SUFFIX: &str = "वा";

/// `to_ordinal`'s irregular forms for 1..=10. Python tests `number == k`
/// with plain `==`, which is *numeric*, so these fire for equal
/// floats/Decimals too: `to_ordinal(1.0)` is "पहिला" and
/// `to_ordinal(Decimal("5.00"))` is "पाचवा" — the ".0" tail vanishes
/// entirely. Shared by [`LangMr::to_ordinal`] (int path) and
/// [`LangMr::ordinal_float_entry`] (float/Decimal path).
const ORDINAL_SPECIALS: [(u64, &str); 10] = [
    (1, "पहिला"),
    (2, "दुसरा"),
    (3, "तिसरा"),
    (4, "चौथा"),
    (5, "पाचवा"),
    (6, "सहावा"),
    (7, "सातवा"),
    (8, "आठवा"),
    (9, "नववा"),
    (10, "दहावा"),
];

const YEAR_PREFIX: &str = "सन ";
const YEAR_BC_PREFIX: &str = "इसवीसन पूर्व ";

/// `Num2Word_MR.to_currency`'s own default `separator=" आणि "`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot simply be a parameter default.
const SEPARATOR_DEFAULT: &str = " आणि ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_MR.to_currency` declares `separator=" आणि "`, but the `Lang` trait
/// has no per-language defaults: `__init__.py`'s fast path (and
/// `bench/diff_test.py`) substitute `kwargs.get("separator", ",")` —
/// **`Num2Word_Base`'s** default — before the value ever reaches Rust. By then
/// "caller omitted separator" and "caller explicitly passed a comma" are the
/// same string, and the information needed to tell them apart no longer exists
/// on this side of the boundary.
///
/// So `,` is read back as the unset sentinel and MR's own default restored.
/// This is the only reading that matches the oracle: every float row of the
/// `mr` currency corpus was generated by `num2words(v, lang="mr",
/// to="currency", currency=c)` with no `separator=`, and each one expects
/// " आणि " (e.g. `12.34` -> "बारा युरो आणि चौतीस सेंट्स").
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " आणि " here where Python would give ",". Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report.
/// `lang_ca.rs` and `lang_es.rs` resolve the identical conflict the same way.
const SEPARATOR_UNSET: &str = ",";

/// The fallback code for `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["INR"])`.
const CURRENCY_FALLBACK: &str = "INR";

fn bi(n: u64) -> BigInt {
    BigInt::from(n)
}

/// Narrow a `BigInt` the caller has already range-checked below 1000.
///
/// Every call site sits inside a `number < 100` or `number < 1000` guard and
/// passes either the value itself or a quotient of it, so the conversion
/// cannot fail.
fn small(n: &BigInt) -> usize {
    n.to_usize()
        .expect("caller guarantees this value is range-checked below 1000")
}

/// One Indian-scale group: `_int_to_word(number // divisor) + " " + word`,
/// then `" " + _int_to_word(remainder)` when the remainder is non-zero.
///
/// Shared by the हजार / लाख / कोटी / अब्ज / खर्व branches, which are
/// character-for-character identical in Python apart from the two constants.
fn scale(number: &BigInt, divisor: u64, word: &str) -> String {
    let (div, rem) = number.div_mod_floor(&bi(divisor));
    let mut result = format!("{} {}", int_to_word(&div), word);
    if !rem.is_zero() {
        result.push(' ');
        result.push_str(&int_to_word(&rem));
    }
    result
}

/// Port of `Num2Word_MR._int_to_word`.
///
/// Python checks `number == 0` first, then the tables are (re)declared, then
/// the `number < 0` arm runs — so every branch below the sign check operates
/// on a non-negative value and Python's floor `//`/`%` coincide with
/// truncating division. `div_mod_floor` matches Python exactly either way.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Unreachable from to_cardinal/to_ordinal/to_year (all detach the sign
    // first), but reproduced as written — note there is no .strip() here.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    if number < &bi(100) {
        return BELOW_HUNDRED[small(number)].to_string();
    }

    if number < &bi(1000) {
        let (div, rem) = number.div_mod_floor(&bi(100));
        let mut result = format!("{}{}", BELOW_HUNDRED[small(&div)], HUNDRED_SUFFIX);
        if !rem.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&rem));
        }
        return result;
    }

    if number < &bi(100_000) {
        // Indian lakh system: thousands run 1_000..99_999.
        return scale(number, 1_000, "हजार");
    }

    if number < &bi(10_000_000) {
        return scale(number, 100_000, "लाख");
    }

    if number < &bi(1_000_000_000) {
        // "Less than 100 crore" — the quotient here reaches 99.
        return scale(number, 10_000_000, "कोटी");
    }

    if number < &bi(100_000_000_000) {
        return scale(number, 1_000_000_000, "अब्ज");
    }

    // Kharab, and the only unbounded branch: 10**22 // 10**11 == 10**11
    // lands right back here, yielding "एक खर्व खर्व".
    scale(number, 100_000_000_000, "खर्व")
}

// ---- float / Decimal path -------------------------------------------------
//
// `Num2Word_MR.to_cardinal` does NOT use `Num2Word_Base.to_cardinal_float`.
// It overrides `to_cardinal` and handles non-integers *inline* off
// `str(number)`:
//
// ```python
// n = str(number).strip()
// if n.startswith("-"):
//     n = n[1:]; ret = self.negword
// else:
//     ret = ""
// if "." in n:
//     left, right = n.split(".", 1)
//     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//     ret += " ".join(self._int_to_word(int(d)) for d in right)
//     return ret.strip()
// else:
//     return (ret + self._int_to_word(int(n))).strip()
// ```
//
// So the fractional part is the *literal digit string* of `str(number)`
// spoken one digit at a time — never a `post` integer, never zero-padded to a
// precision, and the `base.float2tuple` `< 0.01` heuristic never runs. The
// entire specification is therefore Python's `repr(float)` / `str(Decimal)`,
// reproduced by [`py_float_repr`] / [`py_decimal_str`]. Two behaviours fall
// straight out of that and are exactly what Python does, but which the
// inherited base path gets wrong (which is why MR needs its own override):
//
//   * **negative zero keeps its sign.** `str(-0.0)` == "-0.0" and
//     `str(Decimal("-0.00"))` == "-0.00" both start with "-", so MR prepends
//     `negword` even though the value is not `< 0`. Base drops it (its guard
//     is `value < 0 and pre == 0`).
//   * **scientific notation raises `ValueError`.** For `abs(f) >= 1e16` or a
//     nonzero `abs(f) < 1e-4`, `repr(float)` switches to exponent form; MR
//     then feeds a token containing `e`/`E`/`+` to `int()` (either `int(n)`
//     with no dot, or `int(d)` mid-fraction) and Python raises. Base renders
//     it instead.
//
// `precision` (and the `precision=` kwarg) is set on the converter by
// `__init__.py` but MR's `to_cardinal` never reads `self.precision`, so it is
// ignored here too — `precision_override` is dropped in `to_cardinal_float`.
//
// These helpers mirror the already-verified `lang_as.rs` / `lang_gu.rs`
// (Assamese / Gujarati) versions verbatim; those languages share MR's exact
// `str(number)`-driven shape.

/// Shortest round-trip decimal digits of `a` and its decimal point position,
/// matching CPython/Gay's dtoa. Returns `(digits, decpt)` where the value is
/// `0.<digits> * 10**decpt`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with dtoa on the digit
/// *count* and almost always the digits; it disagrees only on **exact ties**,
/// where dtoa picks the candidate with an even last digit while Rust rounds
/// half up. The block below detects that tie with no bignum (write
/// `a = m * 2**e` with `m` odd, `q = digits.len() - decpt`; the tie is
/// `e + q + 1 == 0`, plus `5**-q | m` when `q < 0`) and steps to the even
/// neighbour, exactly as `lang_as.rs` documents and fuzzes.
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let mut digits: Vec<u8> = mant.bytes().filter(|c| *c != b'.').collect();
    let mut decpt: i32 = exp.parse::<i32>().expect("{:e} exponent is an integer") + 1;

    // Decompose a == m * 2**e exactly, then reduce m to odd.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mut m, mut e) = if biased == 0 {
        (frac, -1074i32) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };
    if m == 0 {
        // a == 0.0: dtoa reports digits "0", decpt 1. No tie to break.
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }
    let z = m.trailing_zeros() as i32;
    m >>= z;
    e += z;

    let q = digits.len() as i32 - decpt;
    let mut tie = e + q + 1 == 0;
    if tie && q < 0 {
        let r = -q as u32;
        tie = r <= 22 && m % 5u64.pow(r) == 0;
    }
    if !tie {
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }

    let last = digits[digits.len() - 1] - b'0';
    if last % 2 == 1 {
        if m % 4 == 1 {
            // k even: Python wants k, Rust gave k+1. Last digit is odd (hence
            // non-zero), so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k odd: Python wants k+1, Rust gave k. Carry like dtoa's roundoff.
            let mut i = digits.len();
            loop {
                if i == 0 {
                    digits.insert(0, b'1');
                    decpt += 1;
                    break;
                }
                i -= 1;
                if digits[i] == b'9' {
                    digits[i] = b'0';
                } else {
                    digits[i] += 1;
                    break;
                }
            }
        }
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), CPython's
/// `format_float_short(..., 'r', ...)`. Rust's own `{}` cannot stand in: it
/// never switches to exponent notation and prints `1`, not `1.0`, for integral
/// floats. Rules from `format_float_short`: exponent notation iff
/// `decpt <= -4 || decpt > 16`; the exponent is `%+.02d`; a trailing `.0` is
/// appended to an otherwise-integral fixed result but never in exponent form;
/// `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and the sign is
    // stripped textually into a negword.
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_digits(value.abs());
    let ndigits = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let exp = decpt - 1;
        let mut mant = String::from(&digits[..1]);
        if digits.len() > 1 {
            mant.push('.');
            mant.push_str(&digits[1..]);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - ndigits) as usize)
        )
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(Decimal)` — `Decimal.__str__` with the default context.
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`; the shim builds the value with
/// `BigDecimal::from_str(str(value))`, which preserves trailing zeros and
/// negative exponents (so `"1.10"` round-trips as `(110, 2)`). The
/// `leftdigits > -6` guard keeps every `"0".repeat(...)` bounded.
///
/// Caveat (same as `lang_as.rs`): a `BigDecimal` cannot represent `-0.0`, so a
/// fixed-notation negative zero loses its negword here. Beyond the exponent
/// threshold `int()` raises `ValueError` with a message that agrees either way.
/// Flagged in the port report.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // i128 so that `-scale` cannot overflow for a pathological i64::MIN scale.
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string(); // Decimal._int
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let d = dotplace as usize;
        (int_digits[..d].to_string(), format!(".{}", &int_digits[d..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — signed, but not zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)` for the ASCII fragments [`cardinal_from_str`] hands it.
/// Ports the underscore rule (`int("1_0") == 10`, `int("1_")` raises) and the
/// error message (`repr(s)` of the original argument).
fn py_int(s: &str) -> Result<BigInt> {
    let err = || {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    };
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body.chars().all(|c| c.is_ascii_digit() || c == '_')
    {
        return Err(err());
    }
    let digits: String = body.chars().filter(|c| *c != '_').collect();
    let n: BigInt = digits.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

/// The body of `Num2Word_MR.to_cardinal` for a stringified number `n`, minus
/// the outer `.strip()` (applied by the caller). The sign is stripped
/// *textually* (so "-0.0" keeps its `negword`), `split(".", 1)` caps at one
/// split (a second dot detonates in the digit loop), and `int(left)` runs
/// before the digit generator so a malformed left half is the first to raise.
fn cardinal_from_str(number: &str) -> Result<String> {
    let n = number.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        // ret = self.negword — "ऋण " already carries its own trailing space.
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    let Some(dot) = n.find('.') else {
        ret.push_str(&checked_int_to_word(&py_int(n)?)?);
        return Ok(ret);
    };

    // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
    let (left, right) = (&n[..dot], &n[dot + 1..]);
    ret.push_str(&checked_int_to_word(&py_int(left)?)?);
    ret.push(' ');
    ret.push_str(POINTWORD);
    ret.push(' ');

    // " ".join(self._int_to_word(int(d)) for d in right) — iterate *characters*.
    let mut first = true;
    for d in right.chars() {
        if !first {
            ret.push(' ');
        }
        first = false;
        let mut buf = [0u8; 4];
        ret.push_str(&checked_int_to_word(&py_int(d.encode_utf8(&mut buf))?)?);
    }
    Ok(ret)
}

/// Marathi (`mr`).
///
/// Stateless apart from the immutable `CURRENCY_FORMS` table — Python's
/// `Num2Word_MR` carries no cross-call flags (`setup` assigns two string
/// constants and nothing mutates them afterwards, and `CURRENCY_FORMS` is a
/// class attribute that is only ever read).
pub struct LangMr {
    /// `Num2Word_MR.CURRENCY_FORMS`, built once. `Num2Word_Base` declares an
    /// empty dict and MR replaces it outright — there is no `__init__` merge
    /// step in this class, and MR inherits neither `CURRENCY_ADJECTIVES` nor
    /// `CURRENCY_PRECISION` overrides (both stay `{}`), so the trait defaults
    /// for `currency_adjective` (None) and `currency_precision` (100) are
    /// already correct and are deliberately not overridden.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangMr {
    pub fn new() -> Self {
        // Exactly the four codes Num2Word_MR declares, each a 2-tuple of
        // 2-tuples. Note USD/EUR/GBP repeat the singular as the plural
        // ("डॉलर"/"डॉलर", "पेन्स"/"पेन्स"); only INR actually inflects
        // (रुपया -> रुपये, पैसा -> पैसे). The arity is load-bearing:
        // `to_currency` indexes `cr1[1]`/`cr2[1]` for the plural.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "INR",
            CurrencyForms::new(&["रुपया", "रुपये"], &["पैसा", "पैसे"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["डॉलर", "डॉलर"], &["सेंट", "सेंट्स"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["युरो", "युरो"], &["सेंट", "सेंट्स"]),
        );
        currency_forms.insert(
            "GBP",
            CurrencyForms::new(&["पाउंड", "पाउंड"], &["पेन्स", "पेन्स"]),
        );
        LangMr { currency_forms }
    }
}

impl Default for LangMr {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// खर्व (10^11), so from 10^22 its multiplier would itself need खर्व and the
/// word would stack ("एक खर्व खर्व").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(22))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl Lang for LangMr {
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

    /// `to_ordinal(float/Decimal)`. Python's `to_ordinal` opens with ten
    /// `number == k` checks, and `==` is numeric, so whole floats/Decimals
    /// take the irregular words: `to_ordinal(1.0)` == "पहिला",
    /// `to_ordinal(Decimal("5.00"))` == "पाचवा" — the fractional tail
    /// vanishes entirely. Everything else falls through to
    /// `to_cardinal(number) + "वा"`, i.e. the `str(number)` grammar with the
    /// suffix glued on: 11.0 → "अकरा दशांश शून्यवा", 0.5 →
    /// "शून्य दशांश पाचवा". -0.0 equals no special (and 0 has none anyway)
    /// and keeps its textual negword: "ऋण शून्य दशांश शून्यवा"; -3.0 is
    /// likewise "ऋण तीन दशांश शून्यवा" (oddity 5: no negative guard).
    /// Exponent-form inputs (1e16, `Decimal("1E+2")`) miss every `==` and
    /// then raise `int()`'s ValueError from inside `to_cardinal`.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            for (k, word) in ORDINAL_SPECIALS {
                if i == bi(k) {
                    return Ok(word.to_string());
                }
            }
        }
        Ok(format!(
            "{}{}",
            self.to_cardinal_float(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`. Same numeric `==` on 1..=4, so 1.0 →
    /// "1ला", 2.0 → "2रा", 3.0 → "3रा", `Decimal("4.00")` → "4था"; everything
    /// else is `str(number) + "वा"` (oddity 6). `repr_str` is the binding's Python
    /// `str(value)`, exactly the string Python concatenates — `str()` never
    /// raises, so 5.0 keeps its tail ("5.0वा") and exponent forms pass
    /// through unharmed: "1e+16वा", "1E+2वा", "-0.0वा".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i == bi(1) {
                return Ok("1ला".to_string());
            }
            if i == bi(2) {
                return Ok("2रा".to_string());
            }
            if i == bi(3) {
                return Ok("3रा".to_string());
            }
            if i == bi(4) {
                return Ok("4था".to_string());
            }
        }
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`. Python: `if val < 0:` — a *numeric* test,
    /// not a sign-bit one, so -0.0 takes the "सन" arm and keeps the negword
    /// its `to_cardinal` derives textually: "सन ऋण शून्य दशांश शून्य"
    /// (`FloatValue::is_negative()` would lie here — it is sign-bit aware by
    /// design). True negatives are `abs()`ed *before* `to_cardinal`, so the
    /// BC arm never shows a negword: `to_year(-1.5)` ==
    /// "इसवीसन पूर्व एक दशांश पाच". Exponent-form ValueErrors propagate from
    /// `to_cardinal` in both arms.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let below_zero = match value {
            // `-0.0 < 0` is False in Python.
            FloatValue::Float { value, .. } => *value < 0.0,
            // A BigInt mantissa carries no negative zero, so is_negative()
            // is already the numeric test on this arm.
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if below_zero {
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
            Ok(format!(
                "{}{}",
                YEAR_BC_PREFIX,
                self.to_cardinal_float(&abs, None)?
            ))
        } else {
            Ok(format!(
                "{}{}",
                YEAR_PREFIX,
                self.to_cardinal_float(value, None)?
            ))
        }
    }

    /// MR inherits Base's `str_to_number` (`Decimal(value)`), which parses
    /// "Infinity"/"-Infinity" *successfully* — the ValueError Python shows
    /// for `num2words("Infinity", lang="mr")` happens later, inside MR's
    /// `to_cardinal` (`int("Infinity")` → `ValueError: invalid literal for
    /// int() with base 10: 'Infinity'`; "-Infinity" strips its sign
    /// textually first and reports 'Infinity' too). The Rust dispatcher
    /// hard-codes Base's `int(Decimal("Infinity"))` semantics
    /// (`OverflowError`) for `ParsedNumber::Inf`, which MR never executes,
    /// so Inf parses punt to the Python fallback instead: it runs the
    /// original converter and reproduces every mode byte for byte
    /// (cardinal/ordinal/year raise the ValueError; `to_ordinal_num` happily
    /// returns "Infinityवा"). NaN stays on the dispatcher path — its
    /// hard-coded ValueError already matches the type MR raises from
    /// `int("NaN")`. Same shape as `lang_be.rs` / `lang_as.rs`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity` per mode. MR's `to_cardinal`
    /// feeds the sign-stripped `str(number)` to `int()`, so `int("Infinity")`
    /// raises `ValueError` for cardinal/ordinal/year ("-Infinity" strips its
    /// sign textually first, so the message names 'Infinity' either way).
    /// `to_ordinal_num` never parses — it is `str(number) + "वा"` — so it
    /// succeeds: "Infinityवा" / "-Infinityवा". Replaces the base default
    /// (`int(Decimal Inf)` → OverflowError), which MR never executes.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}Infinity{}",
                if negative { "-" } else { "" },
                ORDINAL_SUFFIX
            )),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".to_string(),
            )),
        }
    }

    /// `Decimal('NaN')` per mode. cardinal/ordinal die in `int("NaN")` →
    /// `ValueError`; `to_ordinal_num` returns "NaNवा" unparsed; but `to_year`
    /// opens with `if val < 0:` — and `Decimal('NaN') < 0` raises
    /// `decimal.InvalidOperation` *before* `to_cardinal` is ever reached.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!("NaN{}", ORDINAL_SUFFIX)),
            "year" => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".to_string(),
            }),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".to_string(),
            )),
        }
    }

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
        " आणि "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "दशांश"
    }

    /// Port of `Num2Word_MR.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).strip()` and tests `n.startswith("-")`,
    /// which for an integer is exactly a sign test. The `"." in n` branch
    /// (float input) is out of scope: `str(int)` never contains a dot.
    ///
    /// The final `.strip()` is reproduced but cannot bite — `_int_to_word`
    /// never returns an empty or space-padded string, so the only whitespace
    /// in play is the single space inside `negword`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_MR.to_ordinal`.
    ///
    /// 1..=10 are irregular; everything else — including 0 and every
    /// negative — is the cardinal with "वा" glued on. Never raises.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        for (k, word) in ORDINAL_SPECIALS {
            if value == &bi(k) {
                return Ok(word.to_string());
            }
        }
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_MR.to_ordinal_num`.
    ///
    /// ASCII digits throughout — irregular suffixes for 1..=4, `str(number) +
    /// "वा"` for everything else (including negatives: `to_ordinal_num(-1)` ==
    /// "-1वा"). See oddity 6.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        if value == &bi(1) {
            return Ok("1ला".to_string());
        }
        if value == &bi(2) {
            return Ok("2रा".to_string());
        }
        if value == &bi(3) {
            return Ok("3रा".to_string());
        }
        if value == &bi(4) {
            return Ok("4था".to_string());
        }
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_MR.to_year`. The `longval=True` parameter is
    /// accepted and then ignored by Python, so the trait's arity matches.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!(
                "{}{}",
                YEAR_BC_PREFIX,
                self.to_cardinal(&value.abs())?
            ))
        } else {
            Ok(format!("{}{}", YEAR_PREFIX, self.to_cardinal(value)?))
        }
    }

    /// Port of `Num2Word_MR.to_cardinal`'s non-integer branch.
    ///
    /// MR does not use `Num2Word_Base.to_cardinal_float`; its overridden
    /// `to_cardinal` reads `str(number)` and speaks the fractional digits one
    /// at a time. So the raw f64 is turned back into `repr(float)` (or the
    /// Decimal into `str(Decimal)`) and fed to [`cardinal_from_str`] verbatim.
    /// The `precision=` kwarg (`precision_override`) is set on the converter by
    /// `__init__.py` but `to_cardinal` never reads `self.precision`, so it is
    /// ignored, exactly as Python ignores it.
    ///
    /// Python wraps both return arms in `.strip()`; it is a no-op on every
    /// reachable output (the string always begins and ends with a word) but is
    /// applied here for fidelity.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            // Python's str(float); the raw f64 crossed the boundary so repr()
            // can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        Ok(cardinal_from_str(&n)?.trim().to_string())
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for the NotImplementedError message that
    /// the inherited `to_cheque` raises on an unknown code.
    fn lang_name(&self) -> &str {
        "Num2Word_MR"
    }

    /// `CURRENCY_FORMS[code]`.
    ///
    /// **Only the inherited `to_cheque` reaches this.** MR's own
    /// `to_currency` overrides the lookup with a `.get(currency, ...INR)`
    /// fallback and never consults this hook — see [`LangMr::to_currency`].
    /// `to_cheque` keeps the strict `self.CURRENCY_FORMS[currency]` subscript,
    /// so `None` here reproduces the `KeyError` that Python catches and
    /// re-raises as NotImplementedError.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_MR.to_currency`.
    ///
    /// MR replaces `Num2Word_Base.to_currency` wholesale with a string-slicing
    /// implementation, so **none** of the base currency machinery participates:
    /// no `parse_currency_parts`, no `pluralize`, no `_money_verbose` /
    /// `_cents_verbose` / `_cents_terse`, no `CURRENCY_PRECISION`, and no
    /// `adjective` support. Four consequences, all confirmed against the
    /// corpus:
    ///
    /// 1. **Unknown codes never raise.** `CURRENCY_FORMS.get(currency,
    ///    CURRENCY_FORMS["INR"])` silently falls back to rupees, so
    ///    `to_currency(2, "CHF")` is "दोन रुपये" and JPY/KWD/BHD/CNY/CHF all
    ///    render in रुपये/पैसे. This is why the `mr` currency corpus has zero
    ///    NotImplementedError rows while the cheque corpus has five.
    /// 2. **Precision is hardcoded to two decimal places** by the `[:2]`
    ///    slice, so the 3-decimal (KWD/BHD) and 0-decimal (JPY) currencies get
    ///    no special handling: `12.34 KWD` is "बारा रुपये आणि चौतीस पैसे",
    ///    not mils, and `12.34 JPY` still shows a cents segment.
    /// 3. **`adjective` is accepted and then ignored** — MR declares the
    ///    parameter but never reads it, and defines no `CURRENCY_ADJECTIVES`.
    /// 4. **Cents are dropped whenever `right == 0`**, including for floats.
    ///    `1.0` -> parts[1] is "0" -> `int("00")` -> 0 -> falsy -> "एक रुपया",
    ///    *not* "एक रुपया आणि शून्य पैसे" as `Num2Word_Base` would give. So
    ///    MR reaches the base's "int skips cents" outcome by a different
    ///    route: the `Int`/`Decimal` split still matters lexically (an int has
    ///    no "." at all, a float may carry non-zero cents), but for a
    ///    whole-valued float the two coincide.
    ///
    /// Python computes `parts = str(val).split(".")` *after* `val = abs(val)`,
    /// so the sign is detached before stringification. `CurrencyValue` carries
    /// the value parsed from that same `str(value)`, and `BigDecimal`'s
    /// `Display` round-trips it (verified: "1.0" stays "1.0", "0.5" stays
    /// "0.5" — the scale is preserved, so `int("00")` vs `int("50")` comes out
    /// right). See the port report for the one lexical form where this
    /// round-trip diverges.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore MR's own `separator=" आणि "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };
        // Declared by Python and never read. Named for the reader's benefit.
        let _ = adjective;

        // `if val < 0: is_negative = True; val = abs(val)`. BigDecimal's
        // is_negative() is false for "-0.0" (the sign is not carried on a zero
        // int_val), matching Python's `-0.0 < 0` being False.
        let is_negative = val.is_negative();

        // `str(val)` — the value stringified *after* the abs().
        let s = match val {
            CurrencyValue::Int(v) => {
                if is_negative {
                    v.abs().to_string()
                } else {
                    v.to_string()
                }
            }
            CurrencyValue::Decimal { value: d, .. } => {
                if is_negative {
                    d.abs().to_string()
                } else {
                    d.to_string()
                }
            }
        };

        // `parts = str(val).split(".")` — split on every dot, as Python does.
        let parts: Vec<&str> = s.split('.').collect();

        // `left = int(parts[0]) if parts[0] else 0`. A non-numeric parts[0]
        // is a ValueError out of int(); str() of a float big enough to render
        // in exponent form ("1e+16") lands here and Python raises ValueError
        // too, so the variant matches rather than approximates.
        let left = if parts[0].is_empty() {
            BigInt::zero()
        } else {
            parts[0]
                .parse::<BigInt>()
                .map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // — a *truncating* two-character slice, not a rounding quantize, so
        // 12.349 gives 34 and 12.999 gives 99 (never 100). Sliced by chars().
        let right = if parts.len() > 1 && !parts[1].is_empty() {
            let mut frac: String = parts[1].chars().take(2).collect();
            while frac.chars().count() < 2 {
                frac.push('0');
            }
            frac.parse::<BigInt>()
                .map_err(|e| N2WError::Value(e.to_string()))?
        } else {
            BigInt::zero()
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`
        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            None => self
                .currency_forms
                .get(CURRENCY_FALLBACK)
                .expect("INR is inserted by LangMr::new"),
        };
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = bi(1);

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let left_str = checked_int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is falsy at 0, which is how a whole
        // float loses its cents segment.
        if cents && !right.is_zero() {
            let cents_str = checked_int_to_word(&right)?;
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — negword already carries its own
        // trailing space ("ऋण "), so this is the interior separator too.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's `.strip()`. Nothing here is ever space-padded (negword's
        // space lands mid-string), so this cannot bite — reproduced anyway.
        Ok(result.trim().to_string())
    }
}
