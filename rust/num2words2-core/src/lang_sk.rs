//! Port of `lang_SK.py` (Slovak). Registry key `"sk"` → `Num2Word_SK`
//! (verified in `num2words2/__init__.py`: `"sk": lang_SK.Num2Word_SK()`).
//!
//! Shape: **self-contained**. `Num2Word_SK` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. `cards`/`merge`
//! therefore stay at their trait defaults. The `THOUSANDS` table ends at
//! 10^30; `maxval()` is 10^33 and larger values raise `OverflowError` (see
//! bug 4 below).
//!
//! Inherited from `Num2Word_Base` (SK does not override either, so the trait
//! defaults are exactly right):
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`.
//!     Corpus confirms the raw form, negatives included: `-1` → `"-1"`.
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` → default
//!     delegates through `&self` and picks up the `to_cardinal` override
//!     below. Slovak has no era/century handling, so years are plain
//!     cardinals: `1999` → "tisíc deväťsto deväťdesiat deväť", `-44` →
//!     "mínus štyridsať štyri".
//!
//! `setup()` sets `negword = "mínus"` and `pointword = "celých"`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below is wrong-looking Slovak but
//! is exactly what Python emits, confirmed against the frozen corpus:
//!
//! 1. *(Fixed, #248.)* Python glued a bare "ý" onto the cardinal for any
//!    number outside its 29-entry table ("dvadsať päťý", "dvestoý",
//!    "milióný"). Compound ordinals are now built component by component,
//!    as in `lang_cs.rs`: 21 == "dvadsiaty prvý", 101 == "stý prvý", 200 ==
//!    "dvojstý", 1001 == "tisíci prvý", 2021 == "dvojtisíci dvadsiaty
//!    prvý", 10**6 == "miliónty", 0 == "nultý". Thousands with a multiplier
//!    above 9 stay cardinal before a lower part ("dvanásť tisíc prvý");
//!    round values the tables do not reach (10 000, 2·10^6, …) still take
//!    Python's cardinal + "ý" fallback.
//! 2. *(Fixed, #248.)* Negative ordinals are "mínus" + the ordinal
//!    (`to_ordinal(-1)` == "mínus prvý"), no longer "mínus jedený". They
//!    still do not raise.
//! 3. **Thousands are written detached and mis-pluralized.** `_int2word`
//!    joins chunk words with spaces, so 2000 == "dve tisíc" (idiomatic Slovak
//!    is "dvetisíc"), and `pluralize` keys off the whole 3-digit chunk rather
//!    than its last digit, so 123 selects form 2: `123456789` ==
//!    "sto dvadsať tri **miliónov** …" where Slovak wants "milióny".
//! 4. **`THOUSANDS` stops at key 10 (10^30).** A chunk index of 11 or more —
//!    i.e. any value >= 10^33 — was a `KeyError` in Python. Fixed
//!    (gladiaio/num2words2#159): `_int2word` raises `OverflowError` for
//!    `abs(n) >= 10**33` first, so [`LangSk::thousands_at`] never misses.
//! 5. **Typo in `THOUSANDS[10]`**: the plural form is "kvintillióny" with a
//!    doubled `l`, while the singular "kvintilión" and genitive
//!    "kvintiliónov" both have one. Kept verbatim.
//! 6. **`to_currency`'s int path mis-pluralized every count except 1 (fixed,
//!    #187).** Python picks `cr1[0]`/`cr1[1]` by hand instead of calling
//!    `self.pluralize`, so 0, 5+ and the teens all took the paucal (2-4) form
//!    — "päť eurá" — while the float path said "päť eur". The port uses
//!    `pluralize` on both paths. See [`LangSk::to_currency`].
//! 7. **`to_currency` has no gender agreement.** The count comes from
//!    `to_cardinal`, which only ever emits the masculine "jeden"/"dva", so a
//!    feminine unit reads wrong: `1 CZK` is "jeden koruna" (Slovak wants
//!    "jedna koruna") and `2 CZK` is "dva koruny" (wants "dve koruny").
//!    `_int2word`'s feminine forms are keyed off the `THOUSANDS` scale index
//!    only (see [`ONES`]) and never off the currency, so nothing here can
//!    reach them.
//!
//! # The currency surface
//!
//! `Num2Word_SK` declares its own `CURRENCY_FORMS` in the class body and
//! subclasses `Num2Word_Base`, **not** `Num2Word_EUR` — so the shared-class-dict
//! mutation that `Num2Word_EN.__init__` performs on `Num2Word_EUR.CURRENCY_FORMS`
//! (see PORTING_CURRENCY.md) cannot reach it: Base's own `CURRENCY_FORMS` is a
//! different, empty dict, and SK's class attribute shadows it outright. Here the
//! literal source *is* what runs. Verified against the live interpreter — SK
//! sees exactly three codes (CZK/EUR/USD) and nothing else, so every other code
//! raises `NotImplementedError`, the corpus-covered
//! GBP/JPY/KWD/BHD/INR/CNY/CHF included.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are Base's empty dicts (SK
//! adds neither), which settles several hooks by inheritance:
//!   * `currency_adjective` — always absent, so `adjective=True` never prefixes
//!     anything on either path. The trait default `None` is correct.
//!   * `currency_precision` — `.get(code, 100)` is 100 for every code, so the
//!     trait default 100 is correct. **SK has no 3-decimal and no 0-decimal
//!     currency**: `default_to_currency`'s `divisor == 1` (JPY) and
//!     `divisor == 1000` (KWD/BHD) branches are unreachable, because those
//!     codes are absent from `CURRENCY_FORMS` and raise first.
//!   * `money_verbose` / `cents_verbose` — both are `self.to_cardinal(n)`, which
//!     the defaults already do through `&self`, so they pick up SK's
//!     `to_cardinal` override.
//!   * `cents_terse` — Base's zero-padded `"%0*d"`, mirrored by
//!     `currency::default_cents_terse`. `cents=False` at 12.05 USD gives
//!     "dvanásť dolárov, 05 centov".
//!   * `to_cheque` — Base's, unoverridden. It reads `cr1[-1]` (the genitive
//!     plural), calls `_money_verbose` for the whole part and upper-cases the
//!     lot: `cheque:USD` of 1234.56 → "TISÍC DVESTO TRIDSAŤ ŠTYRI AND 56/100
//!     DOLÁROV". `currency::default_to_cheque` matches it field for field.
//!   * `cardinal_from_decimal` — left at the default per PORTING_CURRENCY.md
//!     ("the float/Decimal cardinal path is a later phase"). This is the one
//!     known gap, and unlike most languages SK's is **not** benign:
//!
//!     It backs `default_to_currency`'s fractional-cents branch, which Python
//!     reaches as `self.to_cardinal(float(right))` — i.e. through SK's
//!     `to_cardinal` *decimal* branch, not through `Num2Word_Base
//!     .to_cardinal_float`, which is what the default routes to. The two
//!     disagree whenever the fractional remainder has more than one digit,
//!     because SK reads it as a **whole number** while Base spells it
//!     **digit by digit**: `to_currency(3.14159, "USD")` is
//!     "tri doláre, štrnásť celých **sto päťdesiat deväť** centy" in Python but
//!     would be "...štrnásť celých **jeden päť deväť** centy" through the
//!     default. Verified against the live interpreter. No corpus row reaches
//!     it (every `arg` has at most two decimals, so `right` is always whole).
//!     Closing it means porting SK's `to_cardinal` decimal branch — the later
//!     float phase — not patching this hook.
//!
//! # Error variants
//!
//! `KeyError` (bug 4) is modelled by [`key_error`], but since #159 the MAXVAL
//! check makes it unreachable from the public entry points: too-large values
//! are an `OverflowError`. [`value_error`] models `int()` on a non-numeric
//! token: on the integer path the sign is stripped before the digit helpers, so
//! it never fires there, but the float path ([`LangSk::to_cardinal_float`])
//! *does* reach it — a repr/str with no "." (scientific notation, `inf`, `nan`)
//! or a fractional token carrying an `'e'` feeds `int()` a non-decimal token,
//! exactly as Python's `to_cardinal` does.
//!
//! The currency surface adds two more, both routed through `&self` so they keep
//! SK's semantics:
//!   * `NotImplementedError` for an unknown code, raised by
//!     `currency::default_to_currency`/`default_to_cheque` off
//!     [`Lang::currency_forms`] returning `None`. Message verified byte for
//!     byte: `Currency code "GBP" not implemented for "Num2Word_SK"`.
//!   * `OverflowError` — bug 4 is reachable *through* `to_currency`, since the
//!     int path calls `to_cardinal`: `to_currency(10**33, "EUR")` was
//!     `KeyError: 11` in Python and is now an OverflowError (#159).
//!     `to_currency(10**30)` is fine ("kvintilión eur").

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const ZERO: &str = "nula";
const NEGWORD: &str = "mínus";
const POINTWORD: &str = "celých";

/// `ONES`: keys 1..=9, each `(masculine, feminine, scales_using_feminine)`.
///
/// Only key 2 carries a non-empty set: `{1, 3, 5, 7, 9}` are the chunk indices
/// whose `THOUSANDS` noun is grammatically feminine (tisíc, miliarda,
/// biliarda, triliarda, kvadriliarda), where Slovak wants "dve" over "dva".
/// Index 0 is absent in Python and unreachable (guarded by `n1 > 0`).
const ONES: [(&str, &str, &[usize]); 10] = [
    ("", "", &[]), // absent in Python
    ("jeden", "jeden", &[]),
    ("dva", "dve", &[1, 3, 5, 7, 9]),
    ("tri", "tri", &[]),
    ("štyri", "štyri", &[]),
    ("päť", "päť", &[]),
    ("šesť", "šesť", &[]),
    ("sedem", "sedem", &[]),
    ("osem", "osem", &[]),
    ("deväť", "deväť", &[]),
];

/// `TENS`: keys 0..=9 — the 10..=19 teens, keyed by the *units* digit.
const TENS: [&str; 10] = [
    "desať",
    "jedenásť",
    "dvanásť",
    "trinásť",
    "štrnásť",
    "pätnásť",
    "šestnásť",
    "sedemnásť",
    "osemnásť",
    "devätnásť",
];

/// `TWENTIES`: keys 2..=9. Indices 0/1 are absent in Python (guarded `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "dvadsať",
    "tridsať",
    "štyridsať",
    "päťdesiat",
    "šesťdesiat",
    "sedemdesiat",
    "osemdesiat",
    "deväťdesiat",
];

/// `HUNDREDS`: keys 1..=9. Index 0 is absent in Python (guarded `n3 > 0`).
const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvesto",
    "tristo",
    "štyristo",
    "päťsto",
    "šesťsto",
    "sedemsto",
    "osemsto",
    "deväťsto",
];

/// `THOUSANDS`: chunk index → the three plural forms (1 / 2..4 / 5+).
///
/// Keys 1..=10, i.e. 1000^1 (10^3) through 1000^10 (10^30). Index 0 is a
/// placeholder — Python has no key 0 and never looks one up (guarded `i > 0`).
/// A chunk index of 11 or more is a `KeyError`; see bug 4 in the module docs.
///
/// Note `THOUSANDS[10].1` == "kvintillióny" — a doubled `l` that the singular
/// and genitive forms lack. Python's typo, preserved (bug 5).
const THOUSANDS: [[&str; 3]; 11] = [
    ["", "", ""], // absent in Python
    ["tisíc", "tisíc", "tisíc"],                            // 10^3
    ["milión", "milióny", "miliónov"],                      // 10^6
    ["miliarda", "miliardy", "miliárd"],                    // 10^9
    ["bilión", "bilióny", "biliónov"],                      // 10^12
    ["biliarda", "biliardy", "biliárd"],                    // 10^15
    ["trilión", "trilióny", "triliónov"],                   // 10^18
    ["triliarda", "triliardy", "triliárd"],                 // 10^21
    ["kvadrilión", "kvadrilióny", "kvadriliónov"],          // 10^24
    ["kvadriliarda", "kvadriliardy", "kvadriliárd"],        // 10^27
    ["kvintilión", "kvintillióny", "kvintiliónov"],         // 10^30 — sic
];

/// The `ordinals` dict local to `Num2Word_SK.to_ordinal`.
///
/// 29 entries: 1..=20, the round tens 30..=90, 100 and 1000.
/// [`ordinal_below_1000`] composes the rest from it (#248). Keys are `u32` because
/// the table's own domain is tiny; the *input* stays `BigInt` and is compared
/// by value, so a huge or negative argument simply misses every entry — which
/// is exactly Python's `num in ordinals`.
const ORDINALS: [(u32, &str); 29] = [
    (1, "prvý"),
    (2, "druhý"),
    (3, "tretí"),
    (4, "štvrtý"),
    (5, "piaty"),
    (6, "šiesty"),
    (7, "siedmy"),
    (8, "ôsmy"),
    (9, "deviaty"),
    (10, "desiaty"),
    (11, "jedenásty"),
    (12, "dvanásty"),
    (13, "trinásty"),
    (14, "štrnásty"),
    (15, "pätnásty"),
    (16, "šestnásty"),
    (17, "sedemnásty"),
    (18, "osemnásty"),
    (19, "devätnásty"),
    (20, "dvadsiaty"),
    (30, "tridsiaty"),
    (40, "štyridsiaty"),
    (50, "päťdesiaty"),
    (60, "šesťdesiaty"),
    (70, "sedemdesiaty"),
    (80, "osemdesiaty"),
    (90, "deväťdesiaty"),
    (100, "stý"),
    (1000, "tisíci"),
];

/// Ordinal hundreds 100..=900 (#248). Index 0 is unused.
const HUNDREDS_ORD: [&str; 10] = [
    "", "stý", "dvojstý", "trojstý", "štvorstý", "päťstý", "šesťstý", "sedemstý", "osemstý",
    "deväťstý",
];

/// Combining prefix for 2..=9 thousand ("dvojtisíci"). Index 0 and 1 unused.
const MULT_PREFIX: [&str; 10] = [
    "", "", "dvoj", "troj", "štvor", "päť", "šesť", "sedem", "osem", "deväť",
];

fn ordinal_table(n: u64) -> Option<&'static str> {
    ORDINALS.iter().find(|(k, _)| u64::from(*k) == n).map(|(_, w)| *w)
}

/// Ordinal of `1..=999` with every component ordinal: 21 → "dvadsiaty prvý",
/// 101 → "stý prvý" (#248).
fn ordinal_below_1000(n: u64) -> String {
    debug_assert!(n > 0 && n < 1000);
    let mut parts: Vec<&str> = Vec::new();
    if n >= 100 {
        parts.push(HUNDREDS_ORD[(n / 100) as usize]);
    }
    let r = n % 100;
    if r != 0 {
        match ordinal_table(r) {
            Some(w) => parts.push(w),
            None => {
                parts.push(ordinal_table(r / 10 * 10).expect("round tens are tabled"));
                parts.push(ordinal_table(r % 10).expect("units are tabled"));
            }
        }
    }
    parts.join(" ")
}

// --- Python exception encoding -------------------------------------------
//
// Encode the Python exception name in the message so a later phase can remap
// without re-deriving the semantics.

/// Python `KeyError` — the missing `THOUSANDS` entry past 10^30 (bug 4).
fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// Python `ValueError` from `int()` on a non-numeric token. On the integer
/// path it is unreachable (the sign never reaches the digit helpers), but the
/// float path reaches it: a repr/str with no "." (scientific notation, `inf`,
/// `nan`) or a fractional token carrying an `'e'` feeds `int()` a non-decimal
/// token, exactly as Python's `Num2Word_SK.to_cardinal` does. Also kept so
/// [`splitbyx`]/[`get_digits`] mirror `utils.py` rather than assuming clean
/// input.
fn value_error(msg: String) -> N2WError {
    N2WError::Value(msg)
}

/// Python's `int(s)` for the digit-string shapes this module produces.
/// `int("024")` == 24; `int("-")` and `int("")` raise `ValueError`.
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10)
        .ok_or_else(|| value_error(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
///
/// Splits a digit string into chunks of `x` from the right, so the *first*
/// chunk is the short one: `"2024"` → `[2, 24]`, `"1000000"` → `[1, 0, 0]`.
/// Indexes by `chars()`, never bytes — the caller only ever passes ASCII
/// digits here, but the shared helper's contract is character-based.
fn splitbyx(n: &str, x: usize) -> Result<Vec<BigInt>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let slice = |i: usize, j: usize| -> String { chars[i..j.min(length)].iter().collect() };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse_int(&slice(0, start))?);
        }
        let mut i = start;
        while i < length {
            out.push(parse_int(&slice(i, i + x))?);
            i += x;
        }
    } else {
        out.push(parse_int(n)?);
    }
    Ok(out)
}

/// Python's `"%03d" % n`: field width 3 **including** any sign.
/// `1` → `"001"`, `24` → `"024"`, `123` → `"123"`, `1234` → `"1234"`.
fn fmt_03(n: &BigInt) -> String {
    let mag = n.abs().to_string();
    if n.is_negative() {
        let pad = 3usize.saturating_sub(1 + mag.len());
        format!("-{}{}", "0".repeat(pad), mag)
    } else {
        let pad = 3usize.saturating_sub(mag.len());
        format!("{}{}", "0".repeat(pad), mag)
    }
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Callers only ever pass a chunk in 0..=999, so the `[-3:]` slice is the
/// whole formatted string and no digit is ever dropped.
fn get_digits(n: &BigInt) -> Result<[usize; 3]> {
    let s = fmt_03(n);
    let chars: Vec<char> = s.chars().collect();
    // fmt_03 always yields >= 3 chars, so the [-3:] slice is total.
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).ok_or_else(|| {
            value_error(format!("invalid literal for int() with base 10: '{}'", c))
        })? as usize;
    }
    Ok(a)
}

/// The form index `Num2Word_SK.pluralize` selects:
///
/// ```python
/// if n == 1:      form = 0
/// elif 0 < n < 5: form = 1
/// else:           form = 2
/// ```
///
/// Unlike most Slavic siblings (`lang_CS` keys off `n % 10` / `n % 100`), SK
/// tests the number *whole*, which is bug 3: a 3-digit chunk of 123 is not
/// `< 5`, so it takes form 2 and 123 millions reads "miliónov" where Slovak
/// wants "milióny".
///
/// `n` is non-negative on every path that reaches here — `_int2word` passes a
/// chunk in 0..=999, and the currency path passes unit/subunit counts that
/// `parse_currency_parts`/`abs()` have already made positive — so the
/// `0 < n` guard only ever excludes `n == 0` (which takes form 2).
///
/// Note: [`LangSk::pluralize`] spells the same rule out inline against a fixed
/// 3-tuple, the shape `_int2word` needs for `THOUSANDS[i]`; that method is
/// verified against the integer corpus and is deliberately left untouched here.
/// The two must stay in agreement.
fn plural_form_index(n: &BigInt) -> usize {
    if n.is_one() {
        0
    } else if n.is_positive() && n < &BigInt::from(5) {
        1
    } else {
        2
    }
}

/// `Num2Word_SK.CURRENCY_FORMS`, transcribed from the class body.
///
/// Three codes, no more: SK descends from `Num2Word_Base`, so it inherits none
/// of the ~24 extra codes `Num2Word_EN.__init__` writes into
/// `Num2Word_EUR.CURRENCY_FORMS`. Confirmed against the live interpreter.
///
/// All six tuples carry three forms — nominative singular / paucal (2-4) /
/// genitive plural — and the arity is load-bearing: `pluralize` indexes 0/1/2
/// and `to_cheque` takes `cr1[-1]`.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 3] = ["cent", "centy", "centov"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "eurá", "eur"], &CENTS),
    );
    m.insert(
        "CZK",
        CurrencyForms::new(
            &["koruna", "koruny", "korún"],
            &["halier", "haliere", "halierov"],
        ),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["dolár", "doláre", "dolárov"], &CENTS),
    );
    m
}

/// Reproduce CPython's `repr(float)` for a finite-or-not `f64`.
///
/// `Num2Word_SK.to_cardinal` starts with `n = str(number)` and then *string-
/// splits* on `.` — it never calls `float2tuple`, so the f64-artefact heuristic
/// in `floatpath` is irrelevant here. What matters is that `str(number)` for a
/// Python float is byte-identical to `repr(number)` (shortest round-trip), and
/// the SK algorithm reads the characters of that string. So the port must
/// rebuild that exact string.
///
/// CPython uses David Gay's shortest `dtoa` (mode 0): a digit string plus a
/// decimal-point position `decpt`. It prints fixed notation iff
/// `-4 < decpt <= 16` and scientific otherwise, appending `.0` in fixed
/// notation when nothing follows the point (`Py_DTSF_ADD_DOT_0`) and padding the
/// scientific exponent to at least two digits (`1e-05`, `1e+16`). Rust's `{:e}`
/// emits the same shortest digit string with `decpt == exp + 1`, so the two
/// reconstruct identically. Mirrors `lang_cs.rs`'s `python_repr_f64`.
fn python_repr_f64(f: f64) -> String {
    if f.is_nan() {
        // repr(float('nan')) == 'nan', sign dropped. Feeds the no-'.' branch,
        // where int('nan') raises ValueError — reproduced downstream.
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
        // Scientific: first digit, optional ".rest", then "e±NN".
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
        // 0.00…digits
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndigits {
        // digits, trailing zeros, then the ADD_DOT_0 ".0".
        format!("{}{}.0", digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        // digits[:decpt] "." digits[decpt:]
        let dp = decpt as usize;
        format!("{}.{}", &digits[..dp], &digits[dp..])
    };

    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Reproduce CPython's `str(Decimal)` for the value carried on
/// `FloatValue::Decimal`.
///
/// `Num2Word_SK.to_cardinal` sees `n = str(number)` for a `Decimal` too, and
/// again string-splits it. `str(Decimal)` is `_pydecimal.__str__`'s
/// to-scientific-string: it keeps the coefficient's trailing zeros (they live
/// in the exponent, not the digits), so `Decimal('1.10')` prints `"1.10"`, and
/// it switches to `E`-notation exactly when `exp > 0` or the adjusted exponent
/// `< -6`. `BigDecimal::as_bigint_and_exponent` gives `(coefficient, scale)`
/// with `value == coefficient * 10^-scale`, i.e. Python's `_exp == -scale` and
/// `_int == str(abs(coefficient))`, so the same algorithm reconstructs it.
/// Preserving trailing zeros is load-bearing: `"1.10"` must stay `"1.10"` (SK
/// reads the fraction as the whole number `10` → "desať"), never `"1.1"`.
/// Mirrors `lang_cs.rs`'s `python_str_decimal`.
///
/// Default context is non-engineering with `capitals == 1`, hence upper-case
/// `E` and an unpadded `%+d` exponent (`1E-7`, not `1e-07`).
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

pub struct LangSk {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangSk {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSk {
    pub fn new() -> Self {
        LangSk {
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // ever read this table, and rebuilding it on each call is what made
            // an earlier revision of this port slower than the Python it
            // replaces. `lib.rs` holds the instance in a `OnceLock`, so this
            // runs at most once per process.
            currency_forms: build_currency_forms(),
        }
    }

    /// `THOUSANDS[i]`, raising `KeyError` past 10 — unreachable since the
    /// MAXVAL check (bug 4, #159).
    fn thousands_at(&self, i: usize) -> Result<&'static [&'static str; 3]> {
        if (1..=10).contains(&i) {
            Ok(&THOUSANDS[i])
        } else {
            Err(key_error(i.to_string()))
        }
    }

    /// Port of `Num2Word_SK.pluralize`.
    ///
    /// ```python
    /// if n == 1:      form = 0
    /// elif 0 < n < 5: form = 1
    /// else:           form = 2
    /// ```
    /// `n` is the whole 3-digit chunk, not its last digit — which is why 123
    /// millions comes out as "miliónov" (form 2) rather than "milióny"
    /// (bug 3). `n` is never 0 here (`_int2word` skips zero chunks) and never
    /// negative (`to_cardinal` strips the sign first).
    fn pluralize(&self, n: &BigInt, forms: &[&'static str; 3]) -> &'static str {
        let form = if n.is_one() {
            0usize
        } else if n.is_positive() && n < &BigInt::from(5) {
            1
        } else {
            2
        };
        forms[form]
    }

    /// Port of `Num2Word_SK._int2word`. Only ever called with a non-negative
    /// `n` — `to_cardinal` removes the sign before delegating here.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3)?;
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x)?;

            // Python builds a per-chunk list and joins it, then joins the
            // chunk strings again with " ". Both joins use a single space, so
            // the nesting is invisible in the output — but it is why an empty
            // chunk would contribute an empty string rather than being
            // skipped. Zero chunks `continue` above, so that never happens.
            let mut word_chunk: Vec<&'static str> = Vec::new();

            if n3 > 0 {
                word_chunk.push(HUNDREDS[n3]);
            }

            if n2 > 1 {
                word_chunk.push(TWENTIES[n2]);
            }

            if n2 == 1 {
                word_chunk.push(TENS[n1]);
            } else if n1 > 0 && !(i > 0 && x.is_one()) {
                // A leading "jeden" is suppressed on scale chunks, so 1000 is
                // "tisíc" and 10^6 is "milión" — not "jeden tisíc".
                if n2 == 0 && n3 == 0 && ONES[n1].2.contains(&i) {
                    word_chunk.push(ONES[n1].1);
                } else {
                    word_chunk.push(ONES[n1].0);
                }
            }

            if i > 0 {
                word_chunk.push(self.pluralize(x, self.thousands_at(i)?));
            }

            words.push(word_chunk.join(" "));
        }

        Ok(words.join(" "))
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangSk {
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

    /// `to_ordinal(float/Decimal)` — **not** the cardinal float grammar.
    ///
    /// ```text
    /// try:
    ///     num = int(number)
    /// except (ValueError, TypeError):
    ///     return str(number)
    /// ```
    ///
    /// `int()` truncates toward zero, so `to_ordinal(2.5)` == "druhý",
    /// `to_ordinal(-1.5)` == "mínus prvý" (int is -1), and `-0.0` -> 0 ->
    /// "nultý" with **no** minus. Exponent forms convert fine — `int(1e16)`
    /// == 10**16, whose cardinal + "ý" is "desať biliárdý", and
    /// `Decimal("1E+2")` -> 100 -> "stý" — unlike the cardinal path, where
    /// the string algorithm raises ValueError on them. The `except` guard
    /// never fires for a real float/Decimal (int() succeeds), so it is not
    /// modelled here.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let num = match value {
            // Truncation toward zero; every whole f64 is exactly
            // representable, and NaN/±inf floats never reach this entry
            // (the dispatcher keeps them on the Python side).
            FloatValue::Float { value, .. } => {
                BigInt::from_f64(value.trunc()).unwrap_or_else(BigInt::zero)
            }
            // `int(Decimal)` — with_scale(0) truncates toward zero, same as
            // `floatpath::float2tuple`'s `pre`.
            FloatValue::Decimal { value, .. } => {
                value.with_scale(0).as_bigint_and_exponent().0
            }
        };
        self.to_ordinal(&num)
    }

    // to_ordinal_num / to_year float entries stay at the trait defaults:
    // SK overrides neither method, so Base's to_ordinal_num echoes
    // str(number) (the default hook) and Base's to_year routes through
    // to_cardinal (the default hook lands in cardinal_float_entry above).

    /// Base's `str_to_number` parses "Infinity" *successfully*; SK's
    /// ValueError comes later, from `int("Infinity")` inside `to_cardinal`
    /// (no "." in the string, so the else arm feeds the whole token to
    /// int()). The Rust dispatcher hard-codes Base's OverflowError for
    /// `ParsedNumber::Inf`, which SK never executes — so Inf parses punt to
    /// the Python fallback, which reproduces every mode byte for byte. NaN
    /// stays on the dispatcher path (its hard-coded ValueError matches
    /// `int("NaN")` on the cardinal path). Same shape as `lang_as.rs`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity` from a string arg. SK's failure mode
    /// depends on `to` — verified live against the pure-Python oracle:
    ///
    /// * `to_cardinal` / `to_year` reach `int(str(number))` == `int("Infinity")`
    ///   through the else arm (no "." in the string) → `ValueError`. `-Infinity`
    ///   strips its sign first, so the message quotes `'Infinity'`.
    /// * `to_ordinal` does `int(number)` == `int(Decimal('Infinity'))` first,
    ///   whose `OverflowError` the `except (ValueError, TypeError)` does **not**
    ///   catch → `OverflowError` (Base's default here).
    /// * `to_ordinal_num` returns the `Decimal` object unchanged (untested, and
    ///   not representable as a string) — left on Base's `OverflowError`.
    ///
    /// NaN is *not* overridden: Base's `nan_result` already yields `ValueError`,
    /// which matches `int("NaN")` on the corpus-covered cardinal path.
    fn inf_result(&self, _negative: bool, to: &str) -> Result<String> {
        match to {
            "cardinal" | "year" => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            // ordinal (OverflowError) and the un-representable ordinal_num keep
            // Base's OverflowError.
            _ => Err(N2WError::Overflow(
                "cannot convert Infinity to integer".into(),
            )),
        }
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "EUR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "celých"
    }

    /// Port of `Num2Word_SK.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and looks for `"."` after a `","` → `"."`
    /// swap; `str(int)` contains neither, so integers always take the `else`
    /// branch. The float branch (`pointword`, leading-zero padding) is out of
    /// scope.
    ///
    /// Python's `int(str(value)[1:])` for the negative case is exactly
    /// `value.abs()` — `BigInt` never stringifies as `"-0"`, so there is no
    /// edge case where the two diverge.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let result = self.int2word(&value.abs())?;
            Ok(format!("{} {}", NEGWORD, result))
        } else {
            self.int2word(value)
        }
    }

    /// Port of `Num2Word_SK.to_ordinal`.
    ///
    /// The `try: int(number) except (ValueError, TypeError): return str(number)`
    /// guard cannot fire for an integer argument and is not modelled.
    ///
    /// Table hit → the proper Slovak ordinal. Python glued "ý" onto the
    /// cardinal for the rest; compounds are now built component by
    /// component (#248), see the module header for the remaining fallback.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{} {}", NEGWORD, self.to_ordinal(&value.abs())?));
        }
        let n = match value.to_u64() {
            Some(n) => n,
            None => return Ok(format!("{}ý", self.to_cardinal(value)?)),
        };
        if n == 0 {
            return Ok("nultý".to_string());
        }
        if let Some(w) = ordinal_table(n) {
            return Ok(w.to_string());
        }
        if n < 1000 {
            return Ok(ordinal_below_1000(n));
        }
        let (high, low) = (n / 1000 * 1000, n % 1000);
        let thousands = n / 1000;
        let high_ord = match thousands {
            1 => Some("tisíci".to_string()),
            2..=9 => Some(format!("{}tisíci", MULT_PREFIX[thousands as usize])),
            _ => None,
        };
        if low == 0 {
            return Ok(match (high_ord, n) {
                (Some(w), _) => w,
                (None, 1_000_000) => "miliónty".to_string(),
                (None, 1_000_000_000) => "miliardtý".to_string(),
                // Known gap: no standard form tabled for this round value.
                (None, _) => format!("{}ý", self.to_cardinal(value)?),
            });
        }
        let head = match high_ord {
            Some(w) => w,
            None => self.to_cardinal(&BigInt::from(high))?,
        };
        Ok(format!("{} {}", head, ordinal_below_1000(low)))
    }

    /// Port of the non-integer path of `Num2Word_SK.to_cardinal`.
    ///
    /// SK overrides `to_cardinal` outright and handles floats/Decimals **inline
    /// off the string form** — `n = str(number)` — never through
    /// `Num2Word_Base.to_cardinal_float` and never through `float2tuple`. So this
    /// override does NOT call the shared `floatpath` code: it rebuilds
    /// `n = str(number)` exactly (`python_repr_f64` for a float,
    /// `python_str_decimal` for a Decimal) and runs the Python algorithm:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count + self._int2word(int(right))
    ///     result = "%s %s %s" % (self._int2word(int(left)), self.pointword, decimal_part)
    ///     if is_negative: result = self.negword + " " + result
    ///     return result
    /// else:  # no "." — scientific/inf/nan, or an integral Decimal str
    ///     is_negative = n.startswith("-")
    ///     if is_negative: return self.negword + " " + self._int2word(int(n[1:]))
    ///     else:           return self._int2word(int(n))
    /// ```
    ///
    /// Consequences reproduced verbatim:
    ///   * **The f64-artefact trap does not apply.** SK reads the shortest-
    ///     round-trip repr, not `abs(value-pre)*10**precision`: `2.675` reads
    ///     "675" straight off `"2.675"` → "dva celých šesťsto sedemdesiat päť",
    ///     `1.005` reads "005" off `"1.005"` → "jeden celých nula nula päť".
    ///   * **The fraction is spelled as a whole number, not digit by digit.**
    ///     `int(right)` runs through `_int2word`, so `Decimal("1.10")` →
    ///     "jeden celých desať" (ten), not "…jeden nula". Each leading zero of
    ///     `right` prepends a bare "nula "; SK does **not** rstrip trailing zeros
    ///     (unlike CS), so a float `1.0` (repr "1.0") → "jeden celých nula"
    ///     and `Decimal("1.00")` → "jeden celých nula nula": one "nula" per
    ///     digit written (Python added one more, gladiaio/num2words2#237).
    ///   * `precision` / `precision_override` are **ignored**. SK's Python
    ///     `to_cardinal` takes no `precision` kwarg and never reads
    ///     `self.precision`, so the `precision=` override the dispatcher stashes
    ///     is dead for SK. The parameter is accepted and dropped.
    ///   * A repr/str with **no "."** (scientific notation, `inf`, `nan`) feeds a
    ///     non-decimal token to `int()`, which raises **ValueError** via
    ///     [`parse_int`]. An integral Decimal like `Decimal('5')` prints `"5"`
    ///     and the else arm returns "päť"; `Decimal('-3')` returns "mínus tri".
    ///   * A "." repr that also carries an `'e'` (`1.5e-05` → `"1.5e-05"`) enters
    ///     the decimal arm and hits `int("5e-05")` → ValueError, same as Python.
    ///
    /// negword is used **unstripped** (`self.negword + " " + result`), exactly as
    /// the Python source; "mínus" has no surrounding whitespace so the bytes
    /// match. `is_negative` keys off the reconstructed string's leading '-', so a
    /// float `-0.0` (repr "-0.0") reads "mínus …" just as Python does.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).replace(",", ".") — a float repr / Decimal str never
        // carries a comma, so the replace is a no-op, kept for faithfulness.
        let n_raw = match value {
            FloatValue::Float { value, .. } => python_repr_f64(*value),
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
        };
        let n = n_raw.replace(',', ".");

        if n.contains('.') {
            let is_negative = n.starts_with('-');
            // abs_n = n[1:] if is_negative else n. '-' is one ASCII byte, so
            // slicing at 1 lands on a char boundary.
            let abs_n: &str = if is_negative { &n[1..] } else { &n };

            // left, right = abs_n.split(".")
            let (left, right) = match abs_n.split_once('.') {
                Some(pair) => pair,
                // Unreachable: `n` contains '.', and it is never the leading char.
                None => {
                    return Err(value_error(format!(
                        "invalid literal for int() with base 10: '{}'",
                        abs_n
                    )))
                }
            };

            // leading_zero_count = len(right) - len(right.lstrip("0")) — the
            // count of leading '0' chars. SK does not rstrip the fraction.
            // The final int(right) word already says one zero, so an all-zero
            // fraction gets len - 1 leading zeros: exactly the digits written
            // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
            let leading_zero_count = right.chars().take_while(|c| *c == '0').count()
                .min(right.len().saturating_sub(1));

            // decimal_part = (ZERO[0] + " ") * leading_zero_count
            //                + self._int2word(int(right))
            // int(right) is evaluated and _int2word run here, before int(left)
            // below — matching Python's statement order.
            let right_int = parse_int(right)?;
            let right_word = self.int2word(&right_int)?;
            let mut decimal_part = String::new();
            for _ in 0..leading_zero_count {
                decimal_part.push_str(ZERO);
                decimal_part.push(' ');
            }
            decimal_part.push_str(&right_word);

            // result = "%s %s %s" % (self._int2word(int(left)), pointword, decimal_part)
            let left_int = parse_int(left)?;
            let result = format!(
                "{} {} {}",
                self.int2word(&left_int)?,
                POINTWORD,
                decimal_part
            );

            if is_negative {
                Ok(format!("{} {}", NEGWORD, result))
            } else {
                Ok(result)
            }
        } else {
            // else arm: no "." — reachable only for scientific notation / inf /
            // nan (all raise ValueError through parse_int) or an integral
            // Decimal str like "5".
            let is_negative = n.starts_with('-');
            if is_negative {
                let abs_n = &n[1..];
                let m = parse_int(abs_n)?;
                Ok(format!("{} {}", NEGWORD, self.int2word(&m)?))
            } else {
                let m = parse_int(&n)?;
                self.int2word(&m)
            }
        }
    }

    // to_ordinal_num: SK does not override Num2Word_Base.to_ordinal_num, which
    // returns the value unchanged → the trait default is correct.
    //
    // to_year: SK does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal → the trait default is correct.

    // ---- currency -------------------------------------------------------
    //
    // SK supplies `CURRENCY_FORMS`, `pluralize` and an int-only `to_currency`.
    // Everything else on the currency path is `Num2Word_Base`'s and is left at
    // the trait default on purpose — see "The currency surface" in the module
    // docs for why each one is already right.

    fn lang_name(&self) -> &str {
        "Num2Word_SK"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_SK.pluralize(n, forms)`.
    ///
    /// Python indexes the tuple directly, so a form list shorter than the
    /// selected index raises IndexError. Every SK entry has three forms, so
    /// that is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    ///
    /// Python only reached this from `Num2Word_Base.to_currency`'s float path;
    /// the port's int path uses it too (bug 6, #187).
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(plural_form_index(n))
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_SK.to_currency`.
    ///
    /// SK intercepts **`isinstance(val, int)`** — a true Python `int`, never a
    /// whole float — and hands everything else to `Num2Word_Base.to_currency`.
    /// The split is why `currency:EUR` of `1` is "jeden euro" (no cents) while
    /// `1.0` is "jeden euro, nula centov".
    ///
    /// # The int path's plural bug (bug 6, fixed in #187)
    ///
    /// Python picks the currency word by hand here instead of calling
    /// `self.pluralize`:
    ///
    /// ```python
    /// if abs_val == 1:
    ///     currency_str = cr1[0]
    /// else:
    ///     currency_str = cr1[1] if len(cr1) > 1 else cr1[0]
    /// ```
    ///
    /// So every count other than 1 took `cr1[1]`, the paucal (2-4) form,
    /// including 0, 5+ and the teens: `5` was "päť eurá" and `100 USD` "sto
    /// doláre", while the float path said "päť eur, nula centov". The port
    /// calls [`Lang::pluralize`] like the float path, so `5` is "päť eur"
    /// and `0` "nula eur".
    ///
    /// `abs(val)` is taken before the comparison, so `-1` is singular too:
    /// "mínus jeden euro".
    ///
    /// Bug 4 reaches through here: `to_cardinal` is called unguarded, so
    /// `to_currency(10**33, "EUR")` propagates its MAXVAL OverflowError (#159).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        if let CurrencyValue::Int(v) = val {
            // Python: `try: cr1, cr2 = self.CURRENCY_FORMS[currency]
            //          except (KeyError, AttributeError): return super()...`
            // The AttributeError arm is dead — `CURRENCY_FORMS` always exists —
            // so only a missing code falls through, and Base then hits the same
            // KeyError and raises NotImplementedError. Delegating is a longer
            // route to the same error, but it is the route Python takes.
            if let Some(forms) = self.currency_forms.get(currency) {
                // `self.negword`, used *unstripped* — unlike Base, which does
                // `"%s " % self.negword.strip()`. "mínus" has no surrounding
                // whitespace, so the two agree, but the shapes differ.
                let minus_str = if v.is_negative() { NEGWORD } else { "" };
                let abs_val = v.abs();
                let money_str = self.to_cardinal(&abs_val)?;

                // `self.pluralize`, as the float path does — Python's
                // hand-picked `cr1[1]` gave "päť eurá" (bug 6, #187).
                let currency_str = Lang::pluralize(self, &abs_val, &forms.unit)?;

                // Python: ("%s %s %s" % (minus_str, money_str, currency_str))
                //             .strip()
                // The `.strip()` exists to eat the leading space that an empty
                // `minus_str` leaves behind; neither of the other two fields can
                // carry outer whitespace, so `.trim()` is equivalent.
                //
                // `cents`, `separator` and `adjective` are all ignored on this
                // path — Python drops them too. `adjective=True` is a no-op for
                // SK either way, since `CURRENCY_ADJECTIVES` is empty.
                return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                    .trim()
                    .to_string());
            }
        }

        // Floats and Decimals — plus ints whose code SK does not know — go to
        // `Num2Word_Base.to_currency`, which *does* use `pluralize`.
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }
}
