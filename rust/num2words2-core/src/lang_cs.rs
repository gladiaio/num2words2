//! Port of `lang_CS.py` (Czech).
//!
//! Registry check: `CONVERTER_CLASSES["cs"]` is `lang_CS.Num2Word_CS()`, and
//! the legacy alias `"cz"` maps to the same class. This file ports
//! `Num2Word_CS`.
//!
//! Shape: **self-contained**. `Num2Word_CS` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! populates `self.cards` and never sets a meaningful `MAXVAL`. `to_cardinal`
//! is overridden outright and drives `_int2word` over 3-digit chunks, so
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` table
//! (keys 1..=10) ends at 10^30; `maxval()` is 10^33 and larger values raise
//! `OverflowError` up front (gladiaio/num2words2#159) — see below.
//!
//! `setup()` sets `negword = "mínus"` and `pointword = "čárka"`.
//!
//! Inherited from `Num2Word_Base` and left unchanged by CS, so the trait
//! defaults are already correct:
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`.
//!     Verified: `ordinal_num(-1)` == "-1", `ordinal_num(0)` == "0".
//!   * `to_year(value) -> self.to_cardinal(value)` → the default delegates
//!     through `&self` and so picks up the `to_cardinal` override below.
//!     Verified: `year(2000)` == "dva tisíce", `year(-500)` == "mínus pět set".
//!     CS has no era/"BC" handling — a negative year just gets "mínus".
//!
//! Out of scope per PORTING.md: the `"." in n` decimal branch of
//! `to_cardinal`, which is unreachable for integer input because `str(int)`
//! never contains a `.` or `,`. (It *is* reachable from the fractional-cents
//! currency path — see the `cardinal_from_decimal` note under "currency".)
//!
//! # The currency surface
//!
//! `Num2Word_CS` declares its own `CURRENCY_FORMS` in the class body (CZK /
//! EUR / USD only) and subclasses `Num2Word_Base`, **not** `Num2Word_EUR` — so
//! the shared-class-dict mutation that `Num2Word_EN.__init__` performs on
//! `Num2Word_EUR.CURRENCY_FORMS` cannot reach it. Verified against the live
//! interpreter: CS sees exactly three codes, and every other code
//! (GBP/JPY/KWD/BHD/INR/CNY/CHF, all corpus-covered) raises
//! `NotImplementedError`. There is consequently **no 3-decimal and no
//! 0-decimal currency here**: `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION`
//! are both left at Base's empty dict, so the divisor is 100 for every code
//! CS knows and `adjective=True` is a no-op.
//!
//! CS overrides `to_currency` to intercept **`isinstance(val, int)`** and
//! delegates everything else (floats, Decimals, and ints with an unknown code)
//! to `Num2Word_Base.to_currency`. `to_cheque`, `_money_verbose`,
//! `_cents_verbose` and `_cents_terse` are inherited untouched, so the trait
//! defaults already mirror them — the two `_verbose` hooks route through
//! `self.to_cardinal`, which picks up the CS override.
//!
//! # Faithfully reproduced Python bugs
//!
//! `to_ordinal` was the interesting one. Python carries a hardcoded lookup
//! for 1..=20, the round tens 30..=90, 100 and 1000, and glues `"ý"` onto
//! the cardinal for everything else ("dvacet jednaý" for 21, "dvě stěý" for
//! 200, "nulaý" for 0). Fixed (gladiaio/num2words2#216): every component of
//! a compound ordinal is now itself an ordinal, as in standard Czech:
//!
//! | input | output                     |
//! |-------|----------------------------|
//! | 0     | `nultý`                    |
//! | 21    | `dvacátý první`            |
//! | 101   | `stý první`                |
//! | 200   | `dvoustý`                  |
//! | 2021  | `dvoutisící dvacátý první` |
//! | 10^6  | `miliontý`                 |
//! | -1    | `mínus první`              |
//!
//! Thousands with a multiplier above 9 keep that multiplier as a cardinal
//! when a lower part follows ("dvanáct tisíc první"). Round values the
//! tables do not reach (10 000, 2·10^6, …) still take Python's
//! `to_cardinal(n) + "ý"` fallback; that remains a known gap.
//!
//! Two further quirks worth naming:
//!
//! 1. `THOUSANDS[10]` is `("quintillion", "quintilliony", "quintillionů")` —
//!    English-looking stems in an otherwise Czech table (the surrounding
//!    entries are "kvadrilion"/"kvadriliarda"). Kept verbatim.
//! 2. The feminine-"dvě" rule at `i in [3, 5, 7]` (miliarda/biliarda/
//!    triliarda) is applied but the matching rule for `i in [3,5,7]` with
//!    n1 == 1 is absent, so 10^9 relies on the `x == 1` suppression instead.
//!    That happens to produce the right answer ("miliarda"), so it is only
//!    an asymmetry, not a visible bug.
//!
//! # Error variants
//!
//! `THOUSANDS` has keys 1..=10, i.e. chunk indices up to 10^30. `_int2word`
//! looks up `THOUSANDS[i]` for every non-zero chunk, so a value with a
//! non-zero chunk at index >= 11 — anything from 10^33 up, given a non-zero
//! leading group — raised `KeyError` in Python. Fixed
//! (gladiaio/num2words2#159): `_int2word` rejects `abs(n) >= 10**33` with
//! `OverflowError` first, so the `N2WError::Key` arm of [`thousands_at`] is
//! unreachable from the public entry points.
//!
//! No `ValueError`/`IndexError` path is reachable here: `to_cardinal` strips
//! the sign from the *string* before calling `_int2word`, so the `"-"`-into-
//! `int()` hazard that breaks `lang_PL.to_ordinal` cannot fire in Czech.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

fn key_error(msg: impl Into<String>) -> N2WError {
    N2WError::Key(msg.into())
}

const ZERO_WORD: &str = "nula";
const NEGWORD: &str = "mínus";
const POINTWORD: &str = "čárka";

/// `ONES`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0`).
const ONES: [&str; 10] = [
    "", "jedna", "dva", "tři", "čtyři", "pět", "šest", "sedm", "osm", "devět",
];

/// `TENS`, keys 0..=9 — the 10..=19 teens, indexed by the *units* digit.
const TENS: [&str; 10] = [
    "deset",
    "jedenáct",
    "dvanáct",
    "třináct",
    "čtrnáct",
    "patnáct",
    "šestnáct",
    "sedmnáct",
    "osmnáct",
    "devatenáct",
];

/// `TWENTIES`, keys 2..=9. Indices 0 and 1 are absent in Python
/// (guarded by `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "dvacet",
    "třicet",
    "čtyřicet",
    "padesát",
    "šedesát",
    "sedmdesát",
    "osmdesát",
    "devadesát",
];

/// `HUNDREDS`, keys 1..=9. Index 0 is absent in Python (guarded by `n3 > 0`).
/// Several entries are two words ("dvě stě", "pět set").
const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dvě stě",
    "tři sta",
    "čtyři sta",
    "pět set",
    "šest set",
    "sedm set",
    "osm set",
    "devět set",
];

/// `THOUSANDS`, keys 1..=10 → (singular, paucal, genitive-plural).
/// Index 0 is a placeholder: Python has no key 0 and the call site guards it
/// with `if i > 0`. Key 10 ships English-looking "quintillion" stems; kept.
const THOUSANDS: [(&str, &str, &str); 11] = [
    ("", "", ""),                                          // absent in Python
    ("tisíc", "tisíce", "tisíc"),                          // 10^3
    ("milion", "miliony", "milionů"),                      // 10^6
    ("miliarda", "miliardy", "miliard"),                   // 10^9
    ("bilion", "biliony", "bilionů"),                      // 10^12
    ("biliarda", "biliardy", "biliard"),                   // 10^15
    ("trilion", "triliony", "trilionů"),                   // 10^18
    ("triliarda", "triliardy", "triliard"),                // 10^21
    ("kvadrilion", "kvadriliony", "kvadrilionů"),          // 10^24
    ("kvadriliarda", "kvadriliardy", "kvadriliard"),       // 10^27
    ("quintillion", "quintilliony", "quintillionů"),       // 10^30
];

/// `THOUSANDS[i]`. Keys 1..=10 only; `i >= 11` would be a `KeyError`, kept
/// unreachable by the 10^33 MAXVAL check (#159). `i == 0` is unreachable (the call site guards with `if i > 0`) but
/// is reported as a `KeyError` too, matching what Python would do.
fn thousands_at(i: usize) -> Result<(&'static str, &'static str, &'static str)> {
    THOUSANDS
        .get(i)
        .filter(|_| i >= 1)
        .copied()
        .ok_or_else(|| key_error(i.to_string()))
}

/// `Num2Word_CS.to_ordinal`'s hardcoded table: 1..=20, round tens, 100, 1000.
/// [`ordinal_below_1000`] composes the rest from it (#216).
const ORDINALS: [(u16, &str); 29] = [
    (1, "první"),
    (2, "druhý"),
    (3, "třetí"),
    (4, "čtvrtý"),
    (5, "pátý"),
    (6, "šestý"),
    (7, "sedmý"),
    (8, "osmý"),
    (9, "devátý"),
    (10, "desátý"),
    (11, "jedenáctý"),
    (12, "dvanáctý"),
    (13, "třináctý"),
    (14, "čtrnáctý"),
    (15, "patnáctý"),
    (16, "šestnáctý"),
    (17, "sedmnáctý"),
    (18, "osmnáctý"),
    (19, "devatenáctý"),
    (20, "dvacátý"),
    (30, "třicátý"),
    (40, "čtyřicátý"),
    (50, "padesátý"),
    (60, "šedesátý"),
    (70, "sedmdesátý"),
    (80, "osmdesátý"),
    (90, "devadesátý"),
    (100, "stý"),
    (1000, "tisící"),
];

/// Ordinal hundreds 100..=900 (#216). Index 0 is unused.
const HUNDREDS_ORD: [&str; 10] = [
    "", "stý", "dvoustý", "třístý", "čtyřstý", "pětistý", "šestistý", "sedmistý", "osmistý",
    "devítistý",
];

/// Combining prefix for 2..=9 thousand ("dvoutisící", "pětitisící"). Index
/// 0 and 1 are unused.
const MULT_PREFIX: [&str; 10] = [
    "", "", "dvou", "tří", "čtyř", "pěti", "šesti", "sedmi", "osmi", "devíti",
];

fn ordinal_table(n: u64) -> Option<&'static str> {
    ORDINALS.iter().find(|(k, _)| u64::from(*k) == n).map(|(_, w)| *w)
}

/// Ordinal of `1..=999` with every component ordinal: 21 → "dvacátý první",
/// 101 → "stý první", 345 → "třístý čtyřicátý pátý" (#216).
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

/// Port of `utils.splitbyx(n, x)` with `format_int=True`, specialised to the
/// only way CS calls it: `splitbyx(str(n), 3)` where `n` is a **non-negative**
/// int (`to_cardinal` strips the sign textually first). Every chunk is
/// therefore a run of ASCII digits and `int()` cannot fail — unlike in
/// `lang_PL`, where a surviving `"-"` makes this fallible.
///
/// Always yields at least one chunk. Chunks are in most-significant-first
/// order; the head chunk is 1 or 2 digits when `len % 3 != 0`.
fn splitbyx(n: &str, x: usize) -> Vec<BigInt> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let parse = |i: usize, j: usize| -> BigInt {
        let s: String = chars[i..j.min(length)].iter().collect();
        BigInt::parse_bytes(s.as_bytes(), 10).unwrap_or_else(BigInt::zero)
    };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse(0, start));
        }
        let mut i = start;
        while i < length {
            out.push(parse(i, i + x));
            i += x;
        }
    } else {
        out.push(parse(0, length));
    }
    out
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Infallible here: `n` is always a chunk produced by [`splitbyx`] from a
/// non-negative decimal string, so `0 <= n <= 999` and the `"%03d"` field is
/// exactly 3 digits with no sign to trip `int()`.
fn get_digits(n: &BigInt) -> [usize; 3] {
    let s = format!("{:0>3}", n.to_string());
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).unwrap_or(0) as usize;
    }
    a
}

/// The form `Num2Word_CS.pluralize(n, forms)` selects — the Czech 1 / 2-4 / 5+
/// split, as an index into `forms`.
///
/// Python: `if n == 1: 0; elif 5 > n % 10 > 1 and (n % 100 < 10 or n % 100 > 20): 1; else: 2`.
/// The chained comparison means `n % 10 < 5 and n % 10 > 1`.
///
/// `n` is non-negative on every path that reaches here — `_int2word` passes a
/// chunk in 0..=999, and the currency path passes unit/subunit counts that
/// `parse_currency_parts` has already taken the absolute value of — so `%` and
/// `mod_floor` agree (no negative-modulo divergence between Python and Rust).
fn plural_form_index(n: &BigInt) -> usize {
    if n.is_one() {
        return 0;
    }
    let m10 = n.mod_floor(&BigInt::from(10));
    let m100 = n.mod_floor(&BigInt::from(100));
    let paucal = m10 < BigInt::from(5)
        && m10 > BigInt::one()
        && (m100 < BigInt::from(10) || m100 > BigInt::from(20));
    if paucal {
        1
    } else {
        2
    }
}

/// `pluralize` against a fixed 3-tuple — the shape `_int2word` needs for
/// `THOUSANDS[i]`. The trait's `pluralize` handles the `CURRENCY_FORMS` shape.
fn pluralize<'a>(n: &BigInt, forms: (&'a str, &'a str, &'a str)) -> &'a str {
    match plural_form_index(n) {
        0 => forms.0,
        1 => forms.1,
        _ => forms.2,
    }
}

/// `Num2Word_CS.CURRENCY_FORMS`, transcribed from the class body.
///
/// CS descends from `Num2Word_Base`, not `Num2Word_EUR`, so nothing here is
/// touched by the `Num2Word_EN.__init__` mutation described in
/// PORTING_CURRENCY.md — the literal source *is* what runs. Confirmed against
/// the live interpreter: these three codes and no others.
///
/// All six tuples carry three forms, and the arity is load-bearing:
/// `pluralize` indexes 0/1/2, and `to_cheque` takes `cr1[-1]`.
///
/// Two entries are deliberately degenerate, per the Python comments:
///   * EUR's unit is `("euro", "euro", "euro")` — euro does not decline in
///     Czech, so all three forms collide.
///   * EUR's subunit is `("centů", "centů", "centů")` — always the genitive
///     plural. That is why `0.01 EUR` reads "nula euro, jedna centů": the
///     singular slot holds a plural word.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "CZK",
        CurrencyForms::new(
            &["koruna", "koruny", "korun"],
            &["haléř", "haléře", "haléřů"],
        ),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(
            // Euro doesn't decline in Czech.
            &["euro", "euro", "euro"],
            // Cents always in genitive plural.
            &["centů", "centů", "centů"],
        ),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["dolar", "dolary", "dolarů"], &["cent", "centy", "centů"]),
    );
    m
}

/// Reproduce CPython's `repr(float)` for a finite-or-not `f64`.
///
/// `Num2Word_CS.to_cardinal` starts with `n = str(number)` and then *string-
/// splits* on `.` — it never calls `float2tuple`, so the f64-artefact heuristic
/// in `floatpath` is irrelevant here. What matters is that `str(number)` for a
/// Python float is byte-identical to `repr(number)` (shortest round-trip), and
/// the CS algorithm reads the individual characters of that string. So the port
/// must rebuild that exact string.
///
/// CPython uses David Gay's shortest `dtoa` (mode 0): a digit string plus a
/// decimal-point position `decpt`. It then prints fixed notation iff
/// `-4 < decpt <= 16` and scientific otherwise, appending `.0` in fixed
/// notation when nothing follows the point (`Py_DTSF_ADD_DOT_0`) and padding the
/// scientific exponent to at least two digits (`1e-05`, `1e+16`). Rust's `{:e}`
/// emits the same shortest digit string with `decpt == exp + 1`, so the two
/// reconstruct identically. Verified digit-for-digit against the interpreter.
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
/// `Num2Word_CS.to_cardinal` sees `n = str(number)` for a `Decimal` too, and
/// again string-splits it. `str(Decimal)` is `_pydecimal.__str__`'s
/// to-scientific-string: it keeps the coefficient's trailing zeros (they live
/// in the exponent, not the digits), so `Decimal('1.10')` prints `"1.10"`, and
/// it switches to `E`-notation exactly when `exp > 0` or the adjusted exponent
/// `< -6`. `BigDecimal::as_bigint_and_exponent` gives `(coefficient, scale)`
/// with `value == coefficient * 10^-scale`, i.e. Python's `_exp == -scale` and
/// `_int == str(abs(coefficient))`, so the same algorithm reconstructs it.
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

/// `int(token)` as Python does it for the tokens `Num2Word_CS.to_cardinal`
/// feeds to `int()`/`_int2word`: a run of ASCII decimal digits. A non-digit
/// character (an `'e'`/`'E'` from scientific notation, `'inf'`, `'nan'`) is
/// where Python's `int()` raises `ValueError`, which the CS float path never
/// guards — so this returns the same `ValueError` type.
fn parse_int_token(token: &str) -> Result<BigInt> {
    BigInt::parse_bytes(token.as_bytes(), 10)
        .filter(|_| !token.is_empty() && token.chars().all(|c| c.is_ascii_digit()))
        .ok_or_else(|| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                token
            ))
        })
}

pub struct LangCs {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangCs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCs {
    pub fn new() -> Self {
        LangCs {
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // ever read this table, and rebuilding it on each call is what made
            // an earlier revision of this port slower than the Python it
            // replaces.
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_CS._int2word(n)`. `n` is non-negative.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ZERO_WORD.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3);
        // Python: `i = len(chunks)` then `i -= 1` at the top of each pass, so
        // the leading chunk sees `i == len - 1`. splitbyx yields >= 1 chunk,
        // so this cannot underflow.
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                words.push(HUNDREDS[n3].to_string());
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                // Teens: TENS is indexed by the units digit, not the tens.
                words.push(TENS[n1].to_string());
            } else if n1 > 0 && !(i > 0 && x.is_one()) {
                // The `not (i > 0 and x == 1)` guard suppresses a bare "jedna"
                // before a scale word: 1_000_000 is "milion", not "jedna milion".
                if n1 == 2 && (i == 3 || i == 5 || i == 7) {
                    // Feminine form before miliarda / biliarda / triliarda.
                    words.push("dvě".to_string());
                } else {
                    words.push(ONES[n1].to_string());
                }
            }

            if i > 0 {
                // KeyError for i >= 11 (>= 10^33 with a non-zero leading chunk).
                words.push(pluralize(x, thousands_at(i)?).to_string());
            }
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

impl Lang for LangCs {
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
        "čárka"
    }

    /// Port of `Num2Word_CS.to_cardinal`, integer path only.
    ///
    /// Python builds `n = str(number).replace(",", ".")` and branches on
    /// `"." in n`. For integer input there is never a `.` or `,`, so only the
    /// `else` arm is reachable: strip a leading `"-"`, convert the magnitude,
    /// and prefix `negword + " "`.
    ///
    /// Note the negword is used **unstripped** (`self.negword + " " + result`),
    /// not via `Num2Word_Base`'s `"%s " % self.negword.strip()`. "mínus" has no
    /// surrounding whitespace, so both spell "mínus jedna" — but the override
    /// means the base's `parse_minus` spacing convention does not apply here.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let result = self.int2word(&value.abs())?;
            Ok(format!("{} {}", NEGWORD, result))
        } else {
            self.int2word(value)
        }
    }

    /// Port of `Num2Word_CS.to_ordinal`.
    ///
    /// `int(number)` inside Python's `try` always succeeds for integer input,
    /// so the `except (ValueError, TypeError): return str(number)` fallback is
    /// unreachable and is not modelled.
    ///
    /// Python's table covers 1..=20, the round tens 30..=90, 100 and 1000
    /// and glues "ý" onto the cardinal for the rest. Compounds are now built
    /// component by component (#216); see the module header for the rules
    /// and the remaining fallback.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{} {}", NEGWORD, self.to_ordinal(&value.abs())?));
        }
        let n = match value.to_u64() {
            Some(n) => n,
            None => return Ok(self.to_cardinal(value)? + "ý"),
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
            1 => Some("tisící".to_string()),
            2..=9 => Some(format!("{}tisící", MULT_PREFIX[thousands as usize])),
            _ => None,
        };
        if low == 0 {
            return Ok(match (high_ord, n) {
                (Some(w), _) => w,
                (None, 1_000_000) => "miliontý".to_string(),
                (None, 1_000_000_000) => "miliardtý".to_string(),
                // Known gap: no standard form tabled for this round value.
                (None, _) => self.to_cardinal(value)? + "ý",
            });
        }
        let head = match high_ord {
            Some(w) => w,
            None => self.to_cardinal(&BigInt::from(high))?,
        };
        Ok(format!("{} {}", head, ordinal_below_1000(low)))
    }

    /// `to_ordinal(float/Decimal)`: Python's first line is `num =
    /// int(number)` — truncation toward zero — so `2.5` → "druhý",
    /// `-1.5` → "mínus první", `Decimal("1E+2")` → "stý", and the huge
    /// e-form floats that make the *cardinal* path raise ValueError
    /// (`1e+16`) convert cleanly here ("deset biliardý", a round value still on the
    /// cardinal + "ý" fallback). The
    /// `except (ValueError, TypeError)` rescue is unreachable for a
    /// finite float/Decimal; `int(inf)`'s OverflowError and `int(nan)`'s
    /// ValueError are modelled for completeness.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let num = match value {
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
                <BigInt as num_traits::FromPrimitive>::from_f64(value.trunc())
                    .ok_or_else(|| N2WError::Value("cannot convert float to integer".into()))?
            }
            FloatValue::Decimal { value, .. } => {
                // int(Decimal) truncates toward zero.
                value.with_scale(0).as_bigint_and_exponent().0
            }
        };
        self.to_ordinal(&num)
    }

    /// `str_to_number` stays Base's `Decimal(value)`, but CS's `to_cardinal`
    /// then runs `int()` over the dot-free `str()` form, so `"Infinity"`
    /// raises **ValueError** (`int('Infinity')`) rather than the shared Inf
    /// sentinel's OverflowError.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Port of the non-integer path of `Num2Word_CS.to_cardinal`.
    ///
    /// CS overrides `to_cardinal` outright and handles floats/Decimals **inline
    /// off the string form**, never through `Num2Word_Base.to_cardinal_float`
    /// and never through `float2tuple`. So this override does NOT call the
    /// shared `floatpath` code: it rebuilds `n = str(number)` exactly
    /// (`python_repr_f64` for a float, `python_str_decimal` for a Decimal — the
    /// same two reconstructors the currency notes describe) and then runs the
    /// Python algorithm character by character:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     right = right.rstrip("0") or "0"     # '1.50' and 1.50 → 'pět'; '1.00' → 'nula'
    ///     decimal_part = " ".join(ZERO[0] if d == "0" else ONES[int(d)][0] for d in right)
    ///     result = "%s %s %s" % (self._int2word(int(left)), self.pointword, decimal_part)
    ///     if is_negative: result = self.negword + " " + result
    ///     return result
    /// else:
    ///     is_negative = n.startswith("-")
    ///     if is_negative: return self.negword + " " + self._int2word(int(n[1:]))
    ///     else:           return self._int2word(int(n))
    /// ```
    ///
    /// Consequences reproduced verbatim:
    ///   * The **f64-artefact trap does not apply here**: CS reads the shortest-
    ///     round-trip repr string, not `abs(value-pre)*10**precision`. `2.675`
    ///     reads its digits straight off `"2.675"` → "dva čárka šest sedm pět";
    ///     `1.005` off `"1.005"` → "jedna čárka nula nula pět".
    ///   * `precision` / `precision_override` are **ignored**. CS's Python
    ///     `to_cardinal` takes no `precision` kwarg and never reads
    ///     `self.precision`, so the `precision=` override the dispatcher stashes
    ///     on the converter is dead for CS. The parameter is accepted and
    ///     dropped.
    ///   * Trailing zeros are stripped from the fraction (issue #75), an all-
    ///     zero fraction collapses to a single "nula": `Decimal('1.10')` →
    ///     "jedna čárka jedna", `1.0` → "jedna čárka nula".
    ///   * A repr/str with **no "."** (scientific notation, `inf`, `nan`) takes
    ///     the `else` arm and feeds a non-decimal token to `int()`, which raises
    ///     **ValueError** — reproduced via [`parse_int_token`]. `1e-05`, `1e+16`,
    ///     `Decimal('1E-7')`, `inf`, `nan` all raise, matching the interpreter.
    ///     An integral Decimal like `Decimal('5')` prints `"5"` and the else arm
    ///     returns "pět"; `Decimal('-3')` returns "mínus tři".
    ///   * A "." repr that also carries an `'e'` (e.g. `1.5e-05` → `"1.5e-05"`)
    ///     enters the decimal arm and hits the `'e'` while spelling the
    ///     fractional digits → `int('e')` ValueError, same as Python.
    ///
    /// negword is used **unstripped** (`self.negword + " " + result`), exactly
    /// as the Python source does; "mínus" has no surrounding whitespace so the
    /// bytes are identical.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).replace(",", ".")
        // A float's repr and a Decimal's str never contain a comma, so the
        // replace is a no-op here; kept for faithfulness to the Python line.
        let n_raw = match value {
            FloatValue::Float { value, .. } => python_repr_f64(*value),
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
        };
        let n = n_raw.replace(',', ".");

        if n.contains('.') {
            let is_negative = n.starts_with('-');
            // abs_n = n[1:] if is_negative else n. The '-' is 1 ASCII byte, so
            // slicing at 1 lands on a char boundary; the '.' (never the first
            // char of a repr) survives, so `split_once` below always matches.
            let abs_n: &str = if is_negative { &n[1..] } else { &n };

            // left, right = abs_n.split(".")
            let (left, right) = match abs_n.split_once('.') {
                Some(pair) => pair,
                // Unreachable: `n` contains '.', and it is not the leading char.
                None => {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        abs_n
                    )))
                }
            };

            // right = right.rstrip("0") or "0"
            let stripped = right.trim_end_matches('0');
            let right_eff = if stripped.is_empty() { "0" } else { stripped };

            // Say each fractional digit individually. A non-digit char (the 'e'
            // of a scientific repr) is where Python's `int(digit)` raises
            // ValueError; reproduce the type, not a rescue.
            let mut decimal_parts: Vec<String> = Vec::with_capacity(right_eff.len());
            for ch in right_eff.chars() {
                if ch == '0' {
                    decimal_parts.push(ZERO_WORD.to_string());
                } else {
                    let d = ch.to_digit(10).ok_or_else(|| {
                        N2WError::Value(format!(
                            "invalid literal for int() with base 10: '{}'",
                            ch
                        ))
                    })? as usize;
                    // d is 1..=9 here (0 is handled above); ONES is indexed 0..=9.
                    decimal_parts.push(ONES[d].to_string());
                }
            }
            let decimal_part = decimal_parts.join(" ");

            // result = "%s %s %s" % (self._int2word(int(left)), pointword, decimal_part)
            let left_int = parse_int_token(left)?;
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
            // else arm: no "." — int(n) via _int2word, sign handled textually.
            // Reachable only for scientific notation / inf / nan (all raise
            // ValueError through parse_int_token) or an integral Decimal str.
            let is_negative = n.starts_with('-');
            if is_negative {
                let abs_n = &n[1..];
                let m = parse_int_token(abs_n)?;
                Ok(format!("{} {}", NEGWORD, self.int2word(&m)?))
            } else {
                let m = parse_int_token(&n)?;
                self.int2word(&m)
            }
        }
    }

    // to_ordinal_num: CS does not override Num2Word_Base.to_ordinal_num, which
    // returns the value unchanged → the trait default `value.to_string()` is
    // correct ("-1" → "-1", "0" → "0").
    //
    // to_year: CS does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal → the trait default is correct. No era handling in CS, so
    // year(-500) == "mínus pět set".

    // ---- currency -------------------------------------------------------
    //
    // CS supplies `CURRENCY_FORMS`, `pluralize` and an int-only `to_currency`.
    // Everything else on the currency path is `Num2Word_Base`'s, so the trait
    // defaults are already right and are deliberately not overridden:
    //
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is Base's empty dict, so
    //     `adjective=True` never prefixes anything. Default `None` is correct.
    //   * `currency_precision` — `CURRENCY_PRECISION` is Base's empty dict, so
    //     `.get(code, 100)` is 100 for every code. Default 100 is correct, and
    //     the `divisor == 1` (JPY) and `divisor == 1000` (KWD/BHD) branches of
    //     `default_to_currency` are unreachable for CS: those codes are absent
    //     from `CURRENCY_FORMS` and raise NotImplementedError first.
    //   * `money_verbose` / `cents_verbose` — both are `self.to_cardinal(n)`,
    //     which the defaults already do through `&self`, so they pick up the CS
    //     `to_cardinal` override.
    //   * `cents_terse` — Base's zero-padded `"%0*d"`, which
    //     `currency::default_cents_terse` mirrors. `cents=False` at 12.05 USD
    //     gives "dvanáct dolarů, 05 centů".
    //   * `to_cheque` — Base's. It reads `cr1[-1]` (the genitive plural), calls
    //     `_money_verbose` for the whole part, and upper-cases the lot:
    //     `cheque:USD` of 1234.56 → "TISÍC DVĚ STĚ TŘICET ČTYŘI AND 56/100
    //     DOLARŮ". Note EUR's degenerate forms make its cheque unit "EURO".
    //   * `cardinal_from_decimal` — left at the default per PORTING_CURRENCY.md
    //     ("the float/Decimal cardinal path is a later phase"). This is the one
    //     known gap, and it is worth stating precisely.
    //
    //     It backs `default_to_currency`'s fractional-cents branch, which Python
    //     reaches as `self.to_cardinal(float(right))` — i.e. through the CS
    //     `to_cardinal` *decimal* branch, which Rust does not implement yet, not
    //     through `Num2Word_Base.to_cardinal_float`, which is what the default
    //     routes to. The two nevertheless agree on everything reachable here,
    //     because both ultimately emit the digits of `repr(float(right))` one
    //     word at a time: CS splits the repr on "." and `rstrip("0")`s a tail
    //     that a repr never has, while Base re-derives the same digits from a
    //     repr-derived precision. Diffed against the live interpreter over 57
    //     fractional-cents cases (1.011, 2.675, 0.005, 3.14159, 123.4567, the
    //     Decimal forms of same, x CZK/EUR/USD): all agree.
    //
    //     The one divergence is `abs(value) < 1e-6` with a non-zero
    //     fractional-cents remainder. There `right < 1e-4`, so `repr(float)`
    //     switches to exponent notation ("9.9e-05"), CS's `"." in n` test fails,
    //     and `int("9.9e-05")` raises **ValueError**. Rust's `{}` for f64 never
    //     goes exponential in that range, so `float_repr_precision` sees
    //     "0.000099" and returns a string where Python raises. Verified:
    //     `to_currency(1e-07, "USD")` is ValueError in Python, "nula dolarů,
    //     nula čárka ... centy" here. No corpus row reaches it. Fixing it means
    //     porting CS's `to_cardinal` decimal branch — the later float phase —
    //     not patching this hook.

    fn lang_name(&self) -> &str {
        "Num2Word_CS"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_CS.pluralize(n, forms)`.
    ///
    /// Python indexes the tuple directly, so a form list shorter than the
    /// selected index raises IndexError. Every CS entry has three forms, so
    /// that is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    ///
    /// Reached only from `Num2Word_Base.to_currency`'s float path; CS's own
    /// int path pointedly does *not* call this (see `to_currency`).
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(plural_form_index(n))
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_CS.to_currency`.
    ///
    /// CS intercepts **`isinstance(val, int)`** — a true Python `int`, never a
    /// whole float — and hands everything else to `Num2Word_Base.to_currency`.
    /// The split is why `currency:USD` of `1` is "jedna dolar" (no cents) while
    /// `1.0` is "jedna dolar, nula centů".
    ///
    /// # The int path's plural bug, reproduced
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
    /// So *every* count other than 1 takes `cr1[1]`, the paucal (2-4) form,
    /// including 0, 5+ and the teens — where `pluralize` would correctly pick
    /// `cr1[2]`. The corpus locks the wrong answers in: `currency:USD` of `0`
    /// is "nula dolary", of `100` is "sto dolary", of `1000000` is
    /// "milion dolary" — all should be "dolarů", and the float path one line
    /// below *does* say "dolarů" for the same magnitudes ("nula dolarů,
    /// padesát centů" at 0.5). EUR hides the bug entirely because its three
    /// forms are identical. Do not "fix" this into a `pluralize` call.
    ///
    /// `abs(val)` is taken before the comparison, so `-1` is singular too:
    /// "mínus jedna dolar".
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

                let currency_str = if abs_val.is_one() {
                    forms.unit.first().cloned()
                } else {
                    forms.unit.get(1).or_else(|| forms.unit.first()).cloned()
                }
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

                // Python: ("%s %s %s" % (minus_str, money_str, currency_str))
                //             .strip()
                // The `.strip()` exists to eat the leading space that an empty
                // `minus_str` leaves behind; neither of the other two fields can
                // carry outer whitespace, so `.trim()` is equivalent.
                return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                    .trim()
                    .to_string());
            }
        }

        // Floats and Decimals — plus ints whose code CS does not know — go to
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

