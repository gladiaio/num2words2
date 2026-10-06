//! Port of `lang_LT.py` (Lithuanian).
//!
//! Shape: **self-contained**. `Num2Word_LT` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` table
//! ends at 10^30; `maxval()` is 10^33 and larger values raise
//! `OverflowError` (see bug 4 below).
//!
//! `setup()` sets `negword = "minus"` and `pointword = "kablelis"`.
//!
//! Inherited from `Num2Word_Base` (unchanged by LT, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`
//!   * `to_year(value)        -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below.
//!
//! Unlike `lang_PL`, LT's `to_cardinal` strips the sign *before* the digits
//! ever reach `splitbyx`/`get_digits`, so negatives are safe everywhere:
//! `to_cardinal(-1)` == "minus vienas" and `to_ordinal(-1)` == "minus vienasas"
//! rather than the `ValueError` Polish produces.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following are all wrong-looking but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **`to_ordinal` just glues "as" onto the cardinal** for anything outside
//!    its small lookup table ("For other numbers, add 'as' suffix to the
//!    cardinal / This is a simplified implementation"). The results are not
//!    Lithuanian words, but they are the spec:
//!      * `to_ordinal(0)`   == "nulisas"  (0 is absent from the table)
//!      * `to_ordinal(21)`  == "dvidešimt vienasas"
//!      * `to_ordinal(200)` == "du šimtaias"
//!      * `to_ordinal(999)` == "devyni šimtai devyniasdešimt devynias"
//!      * `to_ordinal(10**11)` == "vienas šimtas milijardųas"
//!      * `to_ordinal(-1)`  == "minus vienasas" (sign kept, suffix appended)
//! 2. The ordinal table itself is internally inconsistent: `100` maps to the
//!    definite form "šimtasis" while `1000` maps to "tūkstantas" (the definite
//!    form would be "tūkstantasis"). Both kept verbatim.
//! 3. Typos in `THOUSANDS`, kept verbatim: index 7 is "sikstilijonas" (not
//!    "sekstilijonas") and index 10 is "naintilijonas" — an English "nine"
//!    transliteration where Lithuanian would use "nonilijonas". Hence
//!    `to_cardinal(10**21)` == "vienas sikstilijonas" and `to_cardinal(10**30)`
//!    == "vienas naintilijonas".
//! 4. Fixed (gladiaio/num2words2#159): `THOUSANDS` stops at index 10, so
//!    Python raised `KeyError` from 10^33 up (`to_cardinal(10**33)` →
//!    `KeyError: 11`). `maxval()` is now 10^33 and `_int2word` raises
//!    `OverflowError` for `abs(n) >= 10**33` first; 10^33 - 1 is the largest
//!    convertible value.
//! 5. **`ONES_FEMININE` is unreachable dead code** from every in-scope entry
//!    point — see [`LangLt::int2word`] for the full argument. The table and the
//!    branch are reproduced anyway so the control flow mirrors Python exactly.
//!
//! # Currency
//!
//! `Num2Word_LT` defines its own `CURRENCY_FORMS` **class attribute**, so it is
//! not one of the 16 classes that read the dict `Num2Word_EN.__init__` mutates
//! in place — the `lang_EUR.py` trap does not apply here. Verified against the
//! live interpreter: LT's runtime table is exactly its source literal, six codes
//! (LTL/EUR/USD/GBP/PLN/RUB), each with **three** forms per side.
//!
//! `pluralize` is LT's own Slavic-style three-way rule. `to_currency` is
//! replaced outright and is *not* equivalent to `Num2Word_Base.to_currency` —
//! see [`LangLt::to_currency`] for the four divergences. `to_cheque`,
//! `_money_verbose`, `_cents_verbose`, `_cents_terse`, `CURRENCY_PRECISION`
//! (empty) and `CURRENCY_ADJECTIVES` (empty) are all inherited unchanged, so the
//! trait defaults already mirror them and are deliberately left alone.
//!
//! # Error variants
//!
//! Lithuanian raises two exception types in scope: `OverflowError` for values
//! at or above 10^33 (bug 4), and `NotImplementedError` for an unknown
//! currency code, mapped to `N2WError::NotImplemented`. The table `KeyError`
//! ([`key_error`]) is unreachable from the public entry points since #159.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::ParsedNumber;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const ZERO: &str = "nulis";
const NEGWORD: &str = "minus";
const POINTWORD: &str = "kablelis";

/// `ONES_FEMININE`, keys 1..=9. Index 0 is absent in Python (guarded by
/// `n1 > 0`). Unreachable in practice — see bug 5 / [`LangLt::int2word`].
const ONES_FEMININE: [&str; 10] = [
    "", "viena", "dvi", "trys", "keturios", "penkios", "šešios", "septynios", "aštuonios",
    "devynios",
];

/// `ONES`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0` /
/// `n3 > 0`).
const ONES: [&str; 10] = [
    "", "vienas", "du", "trys", "keturi", "penki", "šeši", "septyni", "aštuoni", "devyni",
];

/// `TENS`, keys 0..=9 — the teens, indexed by the *units* digit when the tens
/// digit is 1. `TENS[0]` == "dešimt" (10).
const TENS: [&str; 10] = [
    "dešimt",
    "vienuolika",
    "dvylika",
    "trylika",
    "keturiolika",
    "penkiolika",
    "šešiolika",
    "septyniolika",
    "aštuoniolika",
    "devyniolika",
];

/// `TWENTIES`, keys 2..=9. Indices 0 and 1 are absent in Python (guarded by
/// `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "dvidešimt",
    "trisdešimt",
    "keturiasdešimt",
    "penkiasdešimt",
    "šešiasdešimt",
    "septyniasdešimt",
    "aštuoniasdešimt",
    "devyniasdešimt",
];

/// `HUNDRED`: (singular, plural). Selected by `n3 > 1`, so 100 → "vienas
/// šimtas" and 200 → "du šimtai".
const HUNDRED: [&str; 2] = ["šimtas", "šimtai"];

/// `THOUSANDS`: chunk index → the three plural forms (singular, nominative
/// plural, genitive plural). Keys 1..=10, i.e. up to 1000^10 == 10^30. Index 0
/// is absent in Python (guarded by `i > 0`); index 11 and beyond was the
/// `KeyError` of bug 4, now pre-empted by the MAXVAL check. Spellings at 7 and 10 are Python's typos (bug 3).
///
/// `static` rather than `const` so that `thousands_at` can hand out a
/// `&'static` borrow without relying on const promotion.
static THOUSANDS: [[&str; 3]; 11] = [
    ["", "", ""], // absent in Python
    ["tūkstantis", "tūkstančiai", "tūkstančių"],
    ["milijonas", "milijonai", "milijonų"],
    ["milijardas", "milijardai", "milijardų"],
    ["trilijonas", "trilijonai", "trilijonų"],
    ["kvadrilijonas", "kvadrilijonai", "kvadrilijonų"],
    ["kvintilijonas", "kvintilijonai", "kvintilijonų"],
    ["sikstilijonas", "sikstilijonai", "sikstilijonų"],
    ["septilijonas", "septilijonai", "septilijonų"],
    ["oktilijonas", "oktilijonai", "oktilijonų"],
    ["naintilijonas", "naintilijonai", "naintilijonų"],
];

/// Python `KeyError` for `THOUSANDS[i]` past index 10. Unreachable from the
/// public entry points since the 10^33 MAXVAL check (bug 4, #159).
fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// `THOUSANDS[i]`, raising `KeyError` past 10 (unreachable, see bug 4).
fn thousands_at(i: usize) -> Result<&'static [&'static str; 3]> {
    THOUSANDS
        .get(i)
        .filter(|_| i >= 1)
        .ok_or_else(|| key_error(i.to_string()))
}

/// Port of `utils.splitbyx(n, x)` with `x == 3` and `format_int=True`.
///
/// `n` is always the decimal string of a **non-negative** `BigInt` here:
/// `to_cardinal` strips the sign via `parse_minus` before `_int2word` runs, so
/// (unlike Polish) no `"-"` can reach the chunk parser and `int()` never fails.
///
/// Each chunk is at most 3 digits, hence `<= 999`, so `u32` is provably wide
/// enough — the *value* stays `BigInt`, only the chunks narrow.
fn splitbyx(n: &str) -> Vec<u32> {
    const X: usize = 3;
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    // Every char is an ASCII digit and every slice is <= 3 long, so parse
    // cannot fail and cannot exceed 999.
    let parse = |s: &[char]| -> u32 {
        s.iter()
            .collect::<String>()
            .parse::<u32>()
            .expect("splitbyx slice is always <= 3 ASCII digits")
    };

    let mut out: Vec<u32> = Vec::new();
    if length > X {
        let start = length % X;
        if start > 0 {
            out.push(parse(&chars[0..start]));
        }
        let mut i = start;
        while i < length {
            out.push(parse(&chars[i..(i + X).min(length)]));
            i += X;
        }
    } else {
        out.push(parse(&chars));
    }
    out
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Callers only ever pass a 3-digit chunk (0..=999), for which `"%03d"` yields
/// exactly three characters, so the `[-3:]` slice is total and no digit can be
/// dropped.
fn get_digits(n: u32) -> [usize; 3] {
    let s = format!("{:03}", n); // "%03d" % n
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).expect("chunk digits are ASCII") as usize;
    }
    a
}

/// The `ordinals` dict rebuilt on every `Num2Word_LT.to_ordinal` call.
///
/// Keys: 1..=20, then the round tens 30..=90, then 100 and 1000. Anything else
/// (including 0 and every negative) misses and falls through to the
/// cardinal-plus-"as" path — bug 1.
///
/// `to_u32` returns `None` for negatives and for anything above `u32::MAX`,
/// which is exactly the "not in ordinals" answer Python gives for those.
fn ordinal_lookup(n: &BigInt) -> Option<&'static str> {
    Some(match n.to_u32()? {
        1 => "pirmas",
        2 => "antras",
        3 => "trečias",
        4 => "ketvirtas",
        5 => "penktas",
        6 => "šeštas",
        7 => "septintas",
        8 => "aštuntas",
        9 => "devintas",
        10 => "dešimtas",
        11 => "vienuoliktas",
        12 => "dvyliktas",
        13 => "tryliktas",
        14 => "keturioliktas",
        15 => "penkioliktas",
        16 => "šešioliktas",
        17 => "septynioliktas",
        18 => "aštuonioliktas",
        19 => "devynioliktas",
        20 => "dvidešimtas",
        30 => "trisdešimtas",
        40 => "keturiasdešimtas",
        50 => "penkiasdešimtas",
        60 => "šešiasdešimtas",
        70 => "septyniasdešimtas",
        80 => "aštuoniasdešimtas",
        90 => "devyniasdešimtas",
        100 => "šimtasis",
        1000 => "tūkstantas",
        _ => return None,
    })
}

/// `Num2Word_LT.CURRENCY_FORMS`, verbatim.
///
/// LT declares this as its own class attribute, so — unlike the 16 classes that
/// inherit `Num2Word_EUR`'s dict — nothing mutates it at import time and the
/// source literal *is* the runtime table. Confirmed by dumping
/// `CONVERTER_CLASSES["lt"].CURRENCY_FORMS` from the live interpreter.
///
/// Every entry carries **three** forms per side (singular, nominative plural,
/// genitive plural) because [`LangLt::pluralize`] indexes `forms[2]`. Dropping
/// the third form would turn every "nulis eurų" into an IndexError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // `GENERIC_CENTS` in Python — one shared tuple reused by four codes.
    const GENERIC_CENTS: [&str; 3] = ["centas", "centai", "centų"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "LTL",
        CurrencyForms::new(&["litas", "litai", "litų"], &GENERIC_CENTS),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(&["euras", "eurai", "eurų"], &GENERIC_CENTS),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["doleris", "doleriai", "dolerių"], &GENERIC_CENTS),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(
            &["svaras sterlingų", "svarai sterlingų", "svarų sterlingų"],
            &["pensas", "pensai", "pensų"],
        ),
    );
    m.insert(
        "PLN",
        CurrencyForms::new(
            &["zlotas", "zlotai", "zlotų"],
            &["grašis", "grašiai", "grašių"],
        ),
    );
    m.insert(
        "RUB",
        CurrencyForms::new(
            &["rublis", "rubliai", "rublių"],
            &["kapeika", "kapeikos", "kapeikų"],
        ),
    );
    m
}

/// Python's `right` inside `Num2Word_LT.to_currency`.
///
/// `parse_currency_parts` returns an `int` when `keep_precision` is off and a
/// `Decimal` when it is on, and LT branches on `isinstance(right, Decimal)` to
/// choose between `to_cardinal` and `to_cardinal_float`. Rust's
/// `parse_currency_parts` always hands back a `BigDecimal`, so the Python type
/// is carried explicitly rather than re-derived — collapsing the two would send
/// whole cents down the float path and print "trisdešimt keturi kablelis nulis".
enum Cents {
    /// Python `int`.
    Whole(BigInt),
    /// Python `Decimal`, with a non-zero fractional part.
    Fractional(BigDecimal),
}

/// Port of `utils.get_digits(n)` for an arbitrary-width `BigInt`.
///
/// ```python
/// [int(x) for x in reversed(list(("%03d" % n)[-3:]))]
/// ```
///
/// `%03d` zero-pads to width 3 and only the last three characters survive the
/// slice, so for any magnitude this is "the last three digits, units-first" —
/// `get_digits(1234)` is `[4, 3, 2]`, not a five-element list. That is why
/// `pluralize(1234, ...)` agrees with `pluralize(234, ...)`.
///
/// The sign occupies part of the `%03d` field, so `"%03d" % -5` is `"-05"` and
/// Python then dies on `int("-")` with a `ValueError`. Reproduced rather than
/// smoothed over, though it is unreachable: every caller passes `abs(...)` or a
/// subunit count. `n` is a full `BigInt` (not the `u32` chunk that
/// [`splitbyx`] yields) because `to_currency` pluralizes `left`, which is
/// unbounded.
fn get_digits_big(n: &BigInt) -> Result<[usize; 3]> {
    let digits = n.abs().to_string();
    let s = if n.is_negative() {
        format!("-{:0>2}", digits)
    } else {
        format!("{:0>3}", digits)
    };
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).ok_or_else(|| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
        })? as usize;
    }
    Ok(a)
}

/// Reconstruct Python's `str(number)` for the value that reached
/// `Num2Word_LT.to_cardinal`.
///
/// LT's float path is `str(number).split(".")`, **not** `base.float2tuple`, so
/// the load-bearing f64 artefacts of the base path are irrelevant here: LT reads
/// the decimal *digits*, so `2.675` yields "675" (the repr digits) and `1.005`
/// yields "005" (two counted leading zeros, then 5). This helper produces the
/// exact string Python's `str()` would, sign included, for both arms:
///
/// * **Float** — Python's `str(float)` is the shortest round-trip repr, always
///   carrying a fractional part (`str(1.0) == "1.0"`). `precision` is the number
///   of fractional digits in that repr (`abs(Decimal(repr).exponent)`), so
///   `format!("{:.*}")` at that width reproduces the same digits — and, unlike
///   Rust's bare `{}`, keeps the trailing `.0` and the leading `-`.
/// * **Decimal** — Python's `str(Decimal)` preserves the exact scale, trailing
///   zeros and all (`str(Decimal("1.10")) == "1.10"`). `precision` is that scale;
///   `with_scale(precision)` re-expands so a `BigDecimal` that dropped trailing
///   zeros is restored to the intended width before the digits are read off.
/// `int(number)`'s truncation toward zero, for the finite float and Decimal
/// arms. `None` only for a non-finite float (Python's `int()` raises there —
/// see the callers for which exception).
fn trunc_to_bigint(v: &FloatValue) -> Option<BigInt> {
    match v {
        FloatValue::Float { value, .. } => {
            if !value.is_finite() {
                return None;
            }
            BigInt::from_f64(value.trunc())
        }
        FloatValue::Decimal { value, .. } => {
            // `with_scale(0)` truncates toward zero — exactly `int(Decimal)`.
            Some(value.with_scale(0).as_bigint_and_exponent().0)
        }
    }
}

/// `int(n)`'s ValueError for the no-`"."` branch of `to_cardinal`, reached
/// when `str(number)` came out in exponent form (`1e+16`, `1E+2`) or as
/// `inf`/`nan`/`Infinity`. Message shape matches CPython's; the corpus checks
/// exception types.
fn int_value_error(literal: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        literal
    ))
}

/// `str(number)` for a float carrying no visible point: exponent form for
/// finite values, `inf`/`nan` otherwise. Only feeds the ValueError message.
fn float_no_point_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let s = format!("{:e}", f);
    match s.split_once('e') {
        Some((m, e)) if !e.starts_with('-') => format!("{}e+{:0>2}", m, e),
        Some((m, e)) => format!("{}e-{:0>2}", m, &e[1..]),
        None => s,
    }
}

fn reconstructed_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => {
            format!("{:.*}", *precision as usize, value)
        }
        FloatValue::Decimal { value, precision } => {
            let neg = value.is_negative();
            let abs = value.abs();
            let body = if *precision == 0 {
                // Integer-valued Decimal (exponent 0): no ".", e.g. str("5").
                abs.with_scale(0).as_bigint_and_exponent().0.to_string()
            } else {
                let sc = *precision as usize;
                // coeff = abs(value) * 10**precision, as an integer.
                let coeff = abs
                    .with_scale(*precision as i64)
                    .as_bigint_and_exponent()
                    .0;
                let digits = coeff.to_string();
                // Left-pad to at least one integer digit before the point.
                let padded = if digits.len() < sc + 1 {
                    format!("{}{}", "0".repeat(sc + 1 - digits.len()), digits)
                } else {
                    digits
                };
                let cut = padded.len() - sc;
                format!("{}.{}", &padded[..cut], &padded[cut..])
            };
            if neg {
                format!("-{}", body)
            } else {
                body
            }
        }
    }
}

pub struct LangLt {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangLt {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLt {
    pub fn new() -> Self {
        LangLt {
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // ever read this table; rebuilding it on each call is what made an
            // earlier revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_LT.pluralize`.
    ///
    /// ```python
    /// n1, n2, n3 = get_digits(n)
    /// if n2 == 1 or n1 == 0 or n == 0: return forms[2]
    /// elif n1 == 1:                    return forms[0]
    /// else:                            return forms[1]
    /// ```
    /// `n` is always a 3-digit chunk here, so the `n == 0` test is redundant
    /// (`n1 == 0` already covers it) — kept for structural fidelity.
    fn pluralize(&self, n: u32, forms: &[&str; 3]) -> String {
        let [n1, n2, _n3] = get_digits(n);
        if n2 == 1 || n1 == 0 || n == 0 {
            forms[2].to_string()
        } else if n1 == 1 {
            forms[0].to_string()
        } else {
            forms[1].to_string()
        }
    }

    /// Port of `Num2Word_LT._int2word(n, feminine=False)`.
    ///
    /// # Bug 5: the `ONES_FEMININE` branch is unreachable
    ///
    /// Python guards it with:
    ///
    /// ```python
    /// if (i == 1 or feminine and i == 0) and n < 1000:
    /// ```
    ///
    /// `and` binds tighter than `or`, so this parses as
    /// `((i == 1) or (feminine and (i == 0))) and (n < 1000)`. Both disjuncts
    /// are then dead in every in-scope call:
    ///
    /// * `i == 1` means a thousands chunk exists, which forces `n >= 1000` and
    ///   contradicts the `n < 1000` conjunct. So `i == 1` can never win.
    /// * `feminine and i == 0` needs `feminine == true`, but no caller anywhere
    ///   in the class passes it. `to_cardinal` calls `_int2word(int(n))` and
    ///   leaves the default `feminine=False`; `to_currency` (now ported, see
    ///   below) only ever reaches `_int2word` through `to_cardinal`, so it does
    ///   not pass it either. Nothing else calls `_int2word`.
    ///
    /// Verified against the interpreter: `to_cardinal(21000)` ==
    /// "dvidešimt vienas tūkstantis", using masculine "vienas" where the
    /// feminine "viena" was evidently intended. The parameter and the branch are
    /// kept so the control flow matches Python one-to-one; if a future caller
    /// ever passes `feminine=true` with `n < 1000`, this reproduces what Python
    /// would then do.
    fn int2word(&self, n: &BigInt, feminine: bool) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string());
        let mut i = chunks.len();
        let thousand = BigInt::from(1000);

        for x in chunks {
            i -= 1;

            if x == 0 {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                words.push(ONES[n3].to_string());
                if n3 > 1 {
                    words.push(HUNDRED[1].to_string());
                } else {
                    words.push(HUNDRED[0].to_string());
                }
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1].to_string());
            } else if n1 > 0 {
                // See the bug-5 note above: this condition is never true.
                if (i == 1 || (feminine && i == 0)) && *n < thousand {
                    words.push(ONES_FEMININE[n1].to_string());
                } else {
                    words.push(ONES[n1].to_string());
                }
            }

            if i > 0 {
                // THOUSANDS[i] — KeyError past index 10, pre-empted by the
                // MAXVAL check (bug 4).
                let forms = thousands_at(i)?;
                words.push(self.pluralize(x, forms));
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

impl Lang for LangLt {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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
        ""
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "kablelis"
    }

    /// Port of `Num2Word_LT.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input, swaps `","` for `"."`, then splits the
    /// sign off with `parse_minus`. `str(int)` never contains a separator, so
    /// integers always take the `else` branch:
    /// `"%s%s" % (base_str, self._int2word(int(n)))`, where `base_str` is
    /// `"minus "` (`negword.strip()` plus a space) for negatives and `""`
    /// otherwise. The float branch (`pointword`, leading-zero padding) is out
    /// of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // base_str = "minus ", n = str(value)[1:] → int(n) == abs(value)
            let words = self.int2word(&value.abs(), false)?;
            Ok(format!("{} {}", NEGWORD, words))
        } else {
            self.int2word(value, false)
        }
    }

    /// Port of `Num2Word_LT.to_ordinal`.
    ///
    /// ```python
    /// try: num = int(number)
    /// except (ValueError, TypeError): return str(number)
    /// if num in ordinals: return ordinals[num]
    /// cardinal = self.to_cardinal(num)
    /// return cardinal + "as"
    /// ```
    ///
    /// The `int()` guard cannot fire for integral input, so it is not modelled.
    /// Everything outside the table gets the cardinal with "as" glued on
    /// (bug 1), and an `OverflowError` from `to_cardinal` propagates
    /// unchanged (bug 4).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(word) = ordinal_lookup(value) {
            return Ok(word.to_string());
        }
        Ok(format!("{}as", self.to_cardinal(value)?))
    }

    /// Port of the **float/Decimal arm of `Num2Word_LT.to_cardinal`**.
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// base_str, n = self.parse_minus(n)
    /// if "." in n:
    ///     left, right = n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count \
    ///                    + self._int2word(int(right))
    ///     return "%s%s %s %s" % (base_str, self._int2word(int(left)),
    ///                            self.pointword, decimal_part)
    /// else:
    ///     return "%s%s" % (base_str, self._int2word(int(n)))
    /// ```
    ///
    /// # Why this is NOT the base float path
    ///
    /// `Num2Word_LT` overrides `to_cardinal` and handles non-integers *inline
    /// from the string* rather than through `base.float2tuple`. Two consequences
    /// the base path would get wrong:
    ///
    /// * **No f64 heuristic.** LT reads the repr digits directly, so `2.675`
    ///   becomes "šeši šimtai septyniasdešimt penki" (675 whole, from
    ///   `int("675")`), not the base path's `.< 0.01`-rescued split. `1.005`
    ///   becomes "nulis nulis penki": `right == "005"`, two counted leading
    ///   zeros, then `int("005") == 5`.
    /// * **Trailing-zero fidelity for Decimals.** `str(Decimal("1.10")) ==
    ///   "1.10"`, so `right == "10"` and the fraction is "dešimt" (10), whereas a
    ///   `float("1.10")` would collapse to "1.1". [`reconstructed_str`] restores
    ///   the exact scale before the digits are read.
    ///
    /// `precision=` is accepted and ignored: `Num2Word_LT.to_cardinal` takes no
    /// precision kwarg and never reads `self.precision`, so a caller-supplied
    /// precision has no effect (verified against the interpreter:
    /// `num2words(1.5, lang="lt", precision=5)` is still "vienas kablelis
    /// penki"). `int2word` raises `OverflowError` from 10^33 (bug 4), and a
    /// leading-zero fractional digit string parses fine (`int("005") == 5`).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let s = reconstructed_str(value);

        // base_str, n = self.parse_minus(n) — negword.strip() == "minus".
        let (base_str, n) = match s.strip_prefix('-') {
            Some(rest) => (format!("{} ", NEGWORD), rest.to_string()),
            None => (String::new(), s),
        };

        // int() over a digit substring: always valid here (str/Decimal reprs),
        // but mapped to ValueError rather than panicking, mirroring int().
        let to_int = |lit: &str| -> Result<BigInt> {
            BigInt::parse_bytes(lit.as_bytes(), 10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", lit))
            })
        };

        if let Some(dot) = n.find('.') {
            let left = &n[..dot];
            let right = &n[dot + 1..];
            // leading_zero_count = len(right) - len(right.lstrip("0"))
            // The final int(right) word already says one zero, so an all-zero
            // fraction gets len - 1 leading zeros: exactly the digits written
            // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
            let leading_zero_count = (right.len() - right.trim_start_matches('0').len())
                .min(right.len().saturating_sub(1));
            // decimal_part = (ZERO[0] + " ") * leading_zero_count
            //                + self._int2word(int(right))
            let decimal_part = format!(
                "{}{}",
                format!("{} ", ZERO).repeat(leading_zero_count),
                self.int2word(&to_int(right)?, false)?
            );
            // "%s%s %s %s" % (base_str, _int2word(int(left)), pointword, dec)
            Ok(format!(
                "{}{} {} {}",
                base_str,
                self.int2word(&to_int(left)?, false)?,
                POINTWORD,
                decimal_part
            ))
        } else {
            // "%s%s" % (base_str, self._int2word(int(n)))
            Ok(format!("{}{}", base_str, self.int2word(&to_int(&n)?, false)?))
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing — Python's gate is
    /// `"." in str(number)`, NOT the base default's `int(value) == value`:
    ///
    /// * a **visible point** (any finite float below 1e16, or a Decimal with
    ///   positive scale) takes the fractional branch even for whole values —
    ///   `5.0` -> "penki kablelis nulis" (one "nulis" per
    ///   fractional digit written; Python added one more, #237),
    ///   `Decimal("5.00")` -> two "nulis".
    /// * **no point** funnels the whole string into `int(n)`: plain digit
    ///   Decimals reach the integer path, while exponent forms (`str(1e16) ==
    ///   "1e+16"`, `str(Decimal("1E+2")) == "1E+2"`) and inf/nan raise
    ///   **ValueError**, not the base default's OverflowError.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        match value {
            FloatValue::Float { value, .. } => Err(int_value_error(&float_no_point_str(*value))),
            FloatValue::Decimal { value, .. } => {
                let s = crate::strnum::python_decimal_str(value);
                match crate::strnum::python_int_parse(&s) {
                    Some(i) => self.to_cardinal(&i),
                    None => Err(int_value_error(&s)),
                }
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python:
    ///
    /// ```python
    /// try: num = int(number)
    /// except (ValueError, TypeError): return str(number)
    /// ```
    ///
    /// `int()` **truncates**: `to_ordinal(2.5)` == `to_ordinal(2)` ==
    /// "antras", `to_ordinal(-0.0)` == "nulisas" (int(-0.0) is 0, sign gone),
    /// and `to_ordinal(1e16)` succeeds — int(float) never raises ValueError
    /// for a finite value, so the ordinal table + "as" suffix run on the
    /// truncated integer. `int(nan)` raises ValueError, which the except arm
    /// converts to `str(number)` == "nan"; `int(inf)` raises OverflowError,
    /// which is *not* in the except tuple and propagates.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let FloatValue::Float { value: f, .. } = value {
            if f.is_nan() {
                return Ok("nan".to_string());
            }
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
        }
        let num = trunc_to_bigint(value).expect("finite after the guards above");
        self.to_ordinal(&num)
    }

    // year_float_entry is deliberately left at the trait default: Python's
    // Num2Word_Base.to_year is `self.to_cardinal(value)` and LT does not
    // override it, so the default's delegation to cardinal_float_entry —
    // which now carries LT's own str-based routing — is exactly the port.

    /// `converter.str_to_number` is Base's `Decimal(value)` (LT doesn't
    /// override it), but `Decimal("Infinity")` then hits LT's `to_cardinal`,
    /// where `str(number)` has no "." and `int("Infinity")` raises
    /// **ValueError** — not the OverflowError the binding's generic Inf arm
    /// would produce. NaN already maps to ValueError there.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(int_value_error(if negative {
                "-Infinity"
            } else {
                "Infinity"
            })),
            other => Ok(other),
        }
    }

    // to_ordinal_num is inherited unchanged from Num2Word_Base: the trait
    // default (`value.to_string()`) already matches, so LT deliberately does
    // not override it.

    // ---- currency -------------------------------------------------------
    //
    // LT overrides exactly two things Python-side: `CURRENCY_FORMS` and
    // `pluralize`, plus `to_currency` wholesale. Everything else on the
    // currency path — `to_cheque`, `_money_verbose`, `_cents_verbose`,
    // `_cents_terse`, `CURRENCY_PRECISION` (Base's empty dict, hence a flat
    // divisor of 100) and `CURRENCY_ADJECTIVES` (also empty) — is inherited
    // unchanged, so the trait defaults already mirror Python and are left
    // alone. In particular `currency_precision` must NOT be overridden: LT has
    // no 3-decimal or 0-decimal code, and `to_cheque` reads the default 100 to
    // build its `NN/100` fraction.

    fn lang_name(&self) -> &str {
        "Num2Word_LT"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_LT.pluralize`.
    ///
    /// ```python
    /// n1, n2, n3 = get_digits(n)
    /// if n2 == 1 or n1 == 0 or n == 0: return forms[2]
    /// elif n1 == 1:                    return forms[0]
    /// else:                            return forms[1]
    /// ```
    ///
    /// The Lithuanian rule keys off the last two digits: the teens and any
    /// multiple of ten take the genitive plural (`forms[2]`, "eurų"), something
    /// ending in a bare 1 takes the singular (`forms[0]`, "euras"), everything
    /// else the nominative plural (`forms[1]`, "eurai"). Hence "vienas šimtas
    /// eurų" (100 → n1 == 0) and "dvylika eurų" (12 → n2 == 1).
    ///
    /// The `n == 0` disjunct is dead: `get_digits(0)` already yields `n1 == 0`,
    /// which fires first. Kept for structural fidelity.
    ///
    /// Python indexes the tuple directly, so a table entry with fewer than
    /// three forms would raise IndexError. Every LT entry has three, so this is
    /// unreachable — but it is mapped rather than panicking so the exception
    /// type survives if the table ever changes.
    ///
    /// This mirrors the private [`LangLt::pluralize`] used by `int2word`, which
    /// stays on `u32` chunks to keep that hot path free of `BigInt`
    /// allocations. Reached here via `Lang::pluralize` (the inherent method
    /// shadows the trait one for `self.pluralize(..)`).
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let [n1, n2, _n3] = get_digits_big(n)?;
        let idx = if n2 == 1 || n1 == 0 || n.is_zero() {
            2
        } else if n1 == 1 {
            0
        } else {
            1
        };
        forms
            .get(idx)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_LT.to_currency`.
    ///
    /// LT *replaces* `Num2Word_Base.to_currency` rather than extending it, and
    /// the replacement is not equivalent. Four divergences, all deliberate:
    ///
    /// 1. **The divisor is hardcoded to 100.** `parse_currency_parts` is called
    ///    without `divisor=`, and `CURRENCY_PRECISION` is never consulted, so
    ///    the 3-decimal / 0-decimal machinery in `default_to_currency` is
    ///    bypassed entirely. Moot in practice — LT's table has no KWD/BHD/JPY
    ///    entry, so those raise NotImplementedError long before a divisor could
    ///    matter — but it is why this cannot delegate to `default_to_currency`.
    /// 2. **The cents gate is `right > 0 or is_float`**, where `is_float` means
    ///    only "the caller did not pass an `int`". Base instead computes
    ///    `has_decimal = isinstance(val, float) or "." in str(val)`. The two
    ///    agree on floats, but they split on `Decimal("5")`: Base prints "penki
    ///    eurai", LT prints **"penki eurai, nulis centų"**. Verified against the
    ///    interpreter. So `CurrencyValue::Decimal`'s `has_decimal` flag is
    ///    ignored here *on purpose* — using it would be a silent behaviour
    ///    change.
    /// 3. **`adjective` is accepted and then ignored.** LT never reads
    ///    `CURRENCY_ADJECTIVES` (Base's empty dict anyway), so
    ///    `to_currency(2, adjective=True)` is just "du eurai".
    /// 4. **`money_str` is `self.to_cardinal(left)`**, not
    ///    `self._money_verbose(left, currency)`. Identical result — LT does not
    ///    override `_money_verbose` — but kept literal.
    ///
    /// The separator is Python's `separator=""` default, resolved through
    /// `default_separator()`. The empty string is not a sentinel: LT's own body
    /// does `sep = separator if separator else ", "`, a truthiness test on the
    /// value, which is ported as-is.
    ///
    /// # Preserved quirk: no space after an explicit separator
    ///
    /// The format string is `"%s%s %s%s%s %s"` — Base's is `"%s%s %s%s %s %s"`.
    /// LT dropped the space because its default `", "` already carries one, so
    /// any caller-supplied separator collides with the cents:
    /// `to_currency(12.34, separator=" ir")` yields
    /// `"dvylika eurų irtrisdešimt keturi centai"`. Verified against the
    /// interpreter; reproduced byte for byte.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Trait hands us None when the caller omitted `separator=`; resolve it
        // to LT's own default ("") before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        // `if isinstance(val, int): ... else: ...`
        let (left, right, is_negative, is_float) = match val {
            CurrencyValue::Int(v) => (v.abs(), Cents::Whole(BigInt::zero()), v.is_negative(), false),
            CurrencyValue::Decimal { value, .. } => {
                // has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0.
                // Decimal's `%` keeps the dividend's sign, but the test is
                // against 0 and Decimal("-0.00") == 0, so it is sign-agnostic —
                // truncating toward zero reproduces it for negatives too.
                let scaled = value * BigDecimal::from(100);
                let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

                let (l, r, neg) = parse_currency_parts(val, false, has_fractional_cents, 100);
                // keep_precision is exactly `isinstance(right, Decimal)`.
                let r = if has_fractional_cents {
                    Cents::Fractional(r)
                } else {
                    Cents::Whole(r.as_bigint_and_exponent().0)
                };
                (l, r, neg, true)
            }
        };

        // `try: cr1, cr2 = self.CURRENCY_FORMS[currency] except KeyError: raise
        // NotImplementedError(...)`. The KeyError is swallowed, so the variant
        // is NotImplemented, not Key.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let minus_str = if is_negative {
            format!("{} ", NEGWORD)
        } else {
            String::new()
        };
        let money_str = self.to_cardinal(&left)?;

        // `if right > 0 or is_float:` — see divergence 2.
        let right_is_positive = match &right {
            Cents::Whole(r) => r.is_positive(),
            Cents::Fractional(r) => r.is_positive(),
        };
        if !(right_is_positive || is_float) {
            return Ok(format!(
                "{}{} {}",
                minus_str,
                money_str,
                Lang::pluralize(self, &left, cr1)?
            ));
        }

        // `if right == 0: ... else: ...`
        let right_is_zero = match &right {
            Cents::Whole(r) => r.is_zero(),
            Cents::Fractional(r) => r.is_zero(),
        };
        let cents_str = if right_is_zero {
            if cents {
                self.to_cardinal(&BigInt::zero())?
            } else {
                "0".to_string()
            }
        } else {
            match &right {
                // `self.to_cardinal_float(float(right)) if cents else
                // str(float(right))` — fractional cents, e.g. 34.5.
                Cents::Fractional(r) => {
                    if cents {
                        // Base's to_cardinal_float via the float cast: exactly
                        // what `cardinal_from_decimal` defaults to. Left at the
                        // default per the port contract.
                        self.cardinal_from_decimal(r)?
                    } else {
                        let f = r.to_f64().ok_or_else(|| {
                            N2WError::Value(format!("cannot represent {} as f64", r))
                        })?;
                        format!("{}", f)
                    }
                }
                // `self.to_cardinal(right) if cents else str(right)`.
                Cents::Whole(r) => {
                    if cents {
                        self.to_cardinal(r)?
                    } else {
                        r.to_string()
                    }
                }
            }
        };

        // `self.pluralize(right, cr2)` — `right` may still be a Decimal here.
        // `get_digits` renders it with `%03d`, which truncates, so the plural
        // rule only ever sees `int(right)`: pluralize(Decimal("34.5")) ==
        // pluralize(34) -> "centai". The `n == 0` disjunct cannot tell the two
        // apart either, because a truncated-to-zero Decimal also has n1 == 0
        // and takes the same `forms[2]` branch.
        let right_trunc = match &right {
            Cents::Whole(r) => r.clone(),
            Cents::Fractional(r) => r.with_scale(0).as_bigint_and_exponent().0,
        };

        // `sep = separator if separator else ", "` — a truthiness test on the
        // ported default `""`, not a sentinel.
        let sep = if separator.is_empty() { ", " } else { separator };

        // `"%s%s %s%s%s %s"` — note the missing space after `sep`.
        Ok(format!(
            "{}{} {}{}{} {}",
            minus_str,
            money_str,
            Lang::pluralize(self, &left, cr1)?,
            sep,
            cents_str,
            Lang::pluralize(self, &right_trunc, cr2)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// A Python `int` arg (corpus `arg` with no dot).
    fn int(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, true, false, false).unwrap()
    }

    /// A Python `float` arg. `has_decimal` is always true for a float repr —
    /// and LT ignores the flag entirely (it keys on `isinstance(val, int)`),
    /// which `decimal_without_dot_still_shows_cents` pins.
    fn float(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, false, true, true).unwrap()
    }

    fn cur(lt: &LangLt, code: &str, v: &CurrencyValue) -> Result<String> {
        lt.to_currency(v, code, true, None, false)
    }

    /// Every succeeding `"lang": "lt", "to": "currency:*"` row of the frozen
    /// corpus, verbatim.
    #[test]
    fn currency_corpus_rows() {
        let lt = LangLt::new();
        let rows: Vec<(&str, CurrencyValue, &str)> = vec![
            ("EUR", int("0"), "nulis eurų"),
            ("EUR", int("1"), "vienas euras"),
            ("EUR", int("2"), "du eurai"),
            ("EUR", int("100"), "vienas šimtas eurų"),
            ("EUR", float("12.34"), "dvylika eurų, trisdešimt keturi centai"),
            ("EUR", float("0.01"), "nulis eurų, vienas centas"),
            ("EUR", float("1.0"), "vienas euras, nulis centų"),
            ("EUR", float("99.99"), "devyniasdešimt devyni eurai, devyniasdešimt devyni centai"),
            ("EUR", float("1234.56"), "vienas tūkstantis du šimtai trisdešimt keturi eurai, penkiasdešimt šeši centai"),
            ("EUR", float("-12.34"), "minus dvylika eurų, trisdešimt keturi centai"),
            ("EUR", int("1000000"), "vienas milijonas eurų"),
            ("EUR", float("0.5"), "nulis eurų, penkiasdešimt centų"),
            ("USD", int("0"), "nulis dolerių"),
            ("USD", int("1"), "vienas doleris"),
            ("USD", int("2"), "du doleriai"),
            ("USD", int("100"), "vienas šimtas dolerių"),
            ("USD", float("12.34"), "dvylika dolerių, trisdešimt keturi centai"),
            ("USD", float("0.01"), "nulis dolerių, vienas centas"),
            ("USD", float("1.0"), "vienas doleris, nulis centų"),
            ("USD", float("99.99"), "devyniasdešimt devyni doleriai, devyniasdešimt devyni centai"),
            ("USD", float("1234.56"), "vienas tūkstantis du šimtai trisdešimt keturi doleriai, penkiasdešimt šeši centai"),
            ("USD", float("-12.34"), "minus dvylika dolerių, trisdešimt keturi centai"),
            ("USD", int("1000000"), "vienas milijonas dolerių"),
            ("USD", float("0.5"), "nulis dolerių, penkiasdešimt centų"),
            ("GBP", int("0"), "nulis svarų sterlingų"),
            ("GBP", int("1"), "vienas svaras sterlingų"),
            ("GBP", int("2"), "du svarai sterlingų"),
            ("GBP", int("100"), "vienas šimtas svarų sterlingų"),
            ("GBP", float("12.34"), "dvylika svarų sterlingų, trisdešimt keturi pensai"),
            ("GBP", float("0.01"), "nulis svarų sterlingų, vienas pensas"),
            ("GBP", float("1.0"), "vienas svaras sterlingų, nulis pensų"),
            ("GBP", float("99.99"), "devyniasdešimt devyni svarai sterlingų, devyniasdešimt devyni pensai"),
            ("GBP", float("1234.56"), "vienas tūkstantis du šimtai trisdešimt keturi svarai sterlingų, penkiasdešimt šeši pensai"),
            ("GBP", float("-12.34"), "minus dvylika svarų sterlingų, trisdešimt keturi pensai"),
            ("GBP", int("1000000"), "vienas milijonas svarų sterlingų"),
            ("GBP", float("0.5"), "nulis svarų sterlingų, penkiasdešimt pensų"),
        ];
        for (code, val, want) in rows {
            assert_eq!(cur(&lt, code, &val).unwrap(), want, "currency:{} {:?}", code, val);
        }
    }

    /// The corpus rows LT's table has no entry for. All 72 are
    /// NotImplementedError — including JPY/KWD/BHD, which never reach the
    /// 0-/3-decimal paths because the lookup fails first.
    #[test]
    fn currency_corpus_notimplemented_rows() {
        let lt = LangLt::new();
        let vals = [
            int("0"), int("1"), int("2"), int("100"), float("12.34"), float("0.01"),
            float("1.0"), float("99.99"), float("1234.56"), float("-12.34"),
            int("1000000"), float("0.5"),
        ];
        for code in ["JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for val in &vals {
                match cur(&lt, code, val) {
                    Err(N2WError::NotImplemented(m)) => assert_eq!(
                        m,
                        format!("Currency code \"{}\" not implemented for \"Num2Word_LT\"", code)
                    ),
                    other => panic!("currency:{} {:?} -> {:?}", code, val, other),
                }
            }
        }
    }

    /// Every `"lang": "lt", "to": "cheque:*"` row of the frozen corpus.
    /// `to_cheque` is inherited from `Num2Word_Base`; these pin that the
    /// default is in fact the right behaviour for LT — note the unit is the
    /// *third* (genitive plural) form, since Base takes `cr1[-1]`.
    #[test]
    fn cheque_corpus_rows() {
        let lt = LangLt::new();
        let v = BigDecimal::from_str("1234.56").unwrap();
        for (code, want) in [
            ("EUR", "VIENAS TŪKSTANTIS DU ŠIMTAI TRISDEŠIMT KETURI AND 56/100 EURŲ"),
            ("USD", "VIENAS TŪKSTANTIS DU ŠIMTAI TRISDEŠIMT KETURI AND 56/100 DOLERIŲ"),
            ("GBP", "VIENAS TŪKSTANTIS DU ŠIMTAI TRISDEŠIMT KETURI AND 56/100 SVARŲ STERLINGŲ"),
        ] {
            assert_eq!(lt.to_cheque(&v, code).unwrap(), want, "cheque:{}", code);
        }
        for code in ["JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match lt.to_cheque(&v, code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_LT\"", code)
                ),
                other => panic!("cheque:{} -> {:?}", code, other),
            }
        }
    }

    /// The three codes the corpus never exercises. Values from the live
    /// interpreter.
    #[test]
    fn currency_codes_absent_from_corpus() {
        let lt = LangLt::new();
        let rows: Vec<(&str, CurrencyValue, &str)> = vec![
            ("LTL", int("1"), "vienas litas"),
            ("LTL", int("2"), "du litai"),
            ("LTL", int("0"), "nulis litų"),
            ("LTL", float("1.0"), "vienas litas, nulis centų"),
            ("LTL", float("12.34"), "dvylika litų, trisdešimt keturi centai"),
            ("PLN", int("1"), "vienas zlotas"),
            ("PLN", int("2"), "du zlotai"),
            ("PLN", int("0"), "nulis zlotų"),
            ("PLN", float("1.0"), "vienas zlotas, nulis grašių"),
            ("PLN", float("12.34"), "dvylika zlotų, trisdešimt keturi grašiai"),
            ("RUB", int("1"), "vienas rublis"),
            ("RUB", int("2"), "du rubliai"),
            ("RUB", int("0"), "nulis rublių"),
            ("RUB", float("1.0"), "vienas rublis, nulis kapeikų"),
            ("RUB", float("12.34"), "dvylika rublių, trisdešimt keturi kapeikos"),
        ];
        for (code, val, want) in rows {
            assert_eq!(cur(&lt, code, &val).unwrap(), want, "currency:{} {:?}", code, val);
        }
    }

    /// Divergence 2: LT's cents gate is `right > 0 or is_float`, where
    /// `is_float` means only "not a Python int" — so `Decimal("5")`, whose
    /// `has_decimal` is false, still prints cents. `Num2Word_Base` would say
    /// "penki eurai". Both values from the live interpreter.
    #[test]
    fn decimal_without_dot_still_shows_cents() {
        let lt = LangLt::new();
        let d5 = CurrencyValue::parse("5", false, false, false).unwrap();
        assert_eq!(cur(&lt, "EUR", &d5).unwrap(), "penki eurai, nulis centų");
        // ...while a true int takes the no-cents branch.
        assert_eq!(cur(&lt, "EUR", &int("5")).unwrap(), "penki eurai");
    }

    /// Fractional cents: `right` stays a `Decimal` and goes through
    /// `to_cardinal_float`, while `pluralize` only ever sees `int(right)`.
    /// Values from the live interpreter.
    #[test]
    fn fractional_cents() {
        let lt = LangLt::new();
        // right == Decimal("34.500") -> "trisdešimt keturi kablelis penki",
        // pluralized as 34 -> "centai".
        assert_eq!(
            cur(&lt, "EUR", &float("12.345")).unwrap(),
            "dvylika eurų, trisdešimt keturi kablelis penki centai"
        );
        // right == Decimal("1.100") -> pluralized as 1 -> "centas" (singular).
        assert_eq!(
            cur(&lt, "EUR", &float("1.011")).unwrap(),
            "vienas euras, vienas kablelis vienas centas"
        );
        // right == Decimal("0.500") -> truncates to 0 -> n1 == 0 -> "centų".
        assert_eq!(
            cur(&lt, "EUR", &float("0.005")).unwrap(),
            "nulis eurų, nulis kablelis penki centų"
        );
    }

    /// `cents=False` takes the terse branch — `str(right)`, not
    /// `_cents_terse`, so there is no zero padding ("0", never "00").
    #[test]
    fn cents_false() {
        let lt = LangLt::new();
        let terse = |v: &CurrencyValue| lt.to_currency(v, "EUR", false, None, false).unwrap();
        assert_eq!(terse(&float("12.34")), "dvylika eurų, 34 centai");
        assert_eq!(terse(&float("1.0")), "vienas euras, 0 centų");
        // str(float(Decimal("34.500"))) == "34.5"
        assert_eq!(terse(&float("12.345")), "dvylika eurų, 34.5 centai");
    }

    /// Preserved quirk: LT's format string has no space after the separator,
    /// because its default `", "` already carries one. Verified against the
    /// interpreter.
    #[test]
    fn explicit_separator_has_no_trailing_space() {
        let lt = LangLt::new();
        assert_eq!(
            lt.to_currency(&float("12.34"), "EUR", true, Some(" ir"), false).unwrap(),
            "dvylika eurų irtrisdešimt keturi centai"
        );
        assert_eq!(
            lt.to_currency(&float("12.34"), "EUR", true, Some(","), false).unwrap(),
            "dvylika eurų,trisdešimt keturi centai"
        );
        // separator="" is Python's own default, and its truthiness test
        // restores ", " — it is a value, not a sentinel.
        assert_eq!(
            lt.to_currency(&float("12.34"), "EUR", true, Some(""), false).unwrap(),
            "dvylika eurų, trisdešimt keturi centai"
        );
    }

    /// Divergence 3: `adjective` is accepted and ignored (LT never reads
    /// CURRENCY_ADJECTIVES, which is Base's empty dict).
    #[test]
    fn adjective_is_ignored() {
        let lt = LangLt::new();
        assert_eq!(
            lt.to_currency(&float("12.34"), "EUR", true, None, true).unwrap(),
            "dvylika eurų, trisdešimt keturi centai"
        );
        assert_eq!(lt.to_currency(&int("2"), "EUR", true, None, true).unwrap(), "du eurai");
    }

    /// `get_digits` keeps only the last three digits, so the plural rule is
    /// driven by them alone regardless of magnitude.
    #[test]
    fn pluralize_reads_last_three_digits() {
        let lt = LangLt::new();
        let forms: Vec<String> = ["euras", "eurai", "eurų"].iter().map(|s| s.to_string()).collect();
        let p = |n: i64| Lang::pluralize(&lt, &BigInt::from(n), &forms).unwrap();
        assert_eq!(p(1), "euras");
        assert_eq!(p(21), "euras"); // n1 == 1
        assert_eq!(p(11), "eurų"); // n2 == 1 (teens)
        assert_eq!(p(12), "eurų");
        assert_eq!(p(2), "eurai");
        assert_eq!(p(1234), "eurai"); // reads "234"
        assert_eq!(p(0), "eurų");
        assert_eq!(p(100), "eurų"); // n1 == 0
        assert_eq!(p(1000000), "eurų");
        // Beyond u32/u64 — pluralize must not narrow the value.
        assert_eq!(
            Lang::pluralize(&lt, &BigInt::from_str("100000000000000000000001").unwrap(), &forms).unwrap(),
            "euras"
        );
    }

    /// Bug reproduction, not endorsement: `"%03d" % -5` is "-05", and Python
    /// then dies on `int("-")` with a ValueError. Unreachable from every real
    /// caller (all pass abs()), pinned so the variant does not drift.
    #[test]
    fn pluralize_negative_is_a_value_error() {
        let lt = LangLt::new();
        let forms: Vec<String> = ["euras", "eurai", "eurų"].iter().map(|s| s.to_string()).collect();
        assert!(matches!(
            Lang::pluralize(&lt, &BigInt::from(-5), &forms),
            Err(N2WError::Value(_))
        ));
    }

    /// Build a `float`-arm value the way the py binding does: raw f64 plus the
    /// repr-derived precision `abs(Decimal(repr(v)).as_tuple().exponent)`.
    fn flt(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    /// Build a `Decimal`-arm value: BigDecimal parsed from `str(Decimal)`, with
    /// precision == the scale (`abs(exponent)`).
    fn dec(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// Every `"lang": "lt", "to": "cardinal"` corpus row whose `arg` has a dot
    /// (float input), verbatim.
    #[test]
    fn cardinal_float_corpus_rows() {
        let lt = LangLt::new();
        let rows: Vec<(FloatValue, &str)> = vec![
            (flt(0.0, 1), "nulis kablelis nulis"), // #237
            (flt(0.5, 1), "nulis kablelis penki"),
            (flt(1.0, 1), "vienas kablelis nulis"),
            (flt(1.5, 1), "vienas kablelis penki"),
            (flt(2.25, 2), "du kablelis dvidešimt penki"),
            (flt(3.14, 2), "trys kablelis keturiolika"),
            (flt(0.01, 2), "nulis kablelis nulis vienas"),
            (flt(0.1, 1), "nulis kablelis vienas"),
            (flt(0.99, 2), "nulis kablelis devyniasdešimt devyni"),
            (flt(1.01, 2), "vienas kablelis nulis vienas"),
            (flt(12.34, 2), "dvylika kablelis trisdešimt keturi"),
            (flt(99.99, 2), "devyniasdešimt devyni kablelis devyniasdešimt devyni"),
            (flt(100.5, 1), "vienas šimtas kablelis penki"),
            (flt(1234.56, 2), "vienas tūkstantis du šimtai trisdešimt keturi kablelis penkiasdešimt šeši"),
            (flt(-0.5, 1), "minus nulis kablelis penki"),
            (flt(-1.5, 1), "minus vienas kablelis penki"),
            (flt(-12.34, 2), "minus dvylika kablelis trisdešimt keturi"),
            // f64-artefact cases: LT reads the repr digits, so no float2tuple
            // rescue is involved — "005" -> 5, "675" -> 675.
            (flt(1.005, 3), "vienas kablelis nulis nulis penki"),
            (flt(2.675, 3), "du kablelis šeši šimtai septyniasdešimt penki"),
        ];
        for (v, want) in rows {
            assert_eq!(lt.to_cardinal_float(&v, None).unwrap(), want, "cardinal {:?}", v);
        }
    }

    /// Every `"lang": "lt", "to": "cardinal_dec"` corpus row (Decimal input),
    /// verbatim — the #603 precision path a float cast would round away.
    #[test]
    fn cardinal_decimal_corpus_rows() {
        let lt = LangLt::new();
        let rows: Vec<(FloatValue, &str)> = vec![
            (dec("0.01", 2), "nulis kablelis nulis vienas"),
            // Trailing zero preserved: right == "10" -> "dešimt", not "1.1".
            (dec("1.10", 2), "vienas kablelis dešimt"),
            (dec("12.345", 3), "dvylika kablelis trys šimtai keturiasdešimt penki"),
            (dec("98746251323029.99", 2),
             "devyniasdešimt aštuoni trilijonai septyni šimtai keturiasdešimt šeši \
              milijardai du šimtai penkiasdešimt vienas milijonas trys šimtai \
              dvidešimt trys tūkstančiai dvidešimt devyni kablelis devyniasdešimt devyni"),
            (dec("0.001", 3), "nulis kablelis nulis nulis vienas"),
        ];
        for (v, want) in rows {
            assert_eq!(lt.to_cardinal_float(&v, None).unwrap(), want, "cardinal_dec {:?}", v);
        }
    }

    /// Decimal edge cases beyond the corpus, from the live interpreter:
    /// an integer-valued Decimal (no dot), a scaled zero, and trailing zeros
    /// that make a three-digit fraction.
    #[test]
    fn cardinal_decimal_edges() {
        let lt = LangLt::new();
        // str(Decimal("5")) == "5" -> no "." -> integer branch.
        assert_eq!(lt.to_cardinal_float(&dec("5", 0), None).unwrap(), "penki");
        // str(Decimal("5.00")) == "5.00" -> right "00" -> one zero per digit
        // written (#237: Python said three).
        assert_eq!(
            lt.to_cardinal_float(&dec("5.00", 2), None).unwrap(),
            "penki kablelis nulis nulis"
        );
        // str(Decimal("1.100")) == "1.100" -> right "100" -> int("100") == 100.
        assert_eq!(
            lt.to_cardinal_float(&dec("1.100", 3), None).unwrap(),
            "vienas kablelis vienas šimtas"
        );
        // Negative integer-valued Decimal keeps the minus.
        assert_eq!(lt.to_cardinal_float(&dec("-1.5", 1), None).unwrap(), "minus vienas kablelis penki");
    }

    /// `precision=` is ignored by LT (its to_cardinal never reads
    /// self.precision), verified against the interpreter.
    #[test]
    fn precision_override_is_ignored() {
        let lt = LangLt::new();
        for p in [Some(0u32), Some(1), Some(5), None] {
            assert_eq!(
                lt.to_cardinal_float(&flt(1.5, 1), p).unwrap(),
                "vienas kablelis penki",
                "precision override {:?}", p
            );
        }
        // 2.675 with precision=1 is still the full repr-digit split.
        assert_eq!(
            lt.to_cardinal_float(&flt(2.675, 3), Some(1)).unwrap(),
            "du kablelis šeši šimtai septyniasdešimt penki"
        );
    }

    /// Non-corpus floats with messy shortest-round-trip reprs, from the live
    /// interpreter. Confirms `format!("{:.*}")` reproduces Python's `str(float)`
    /// digit-for-digit even at repr's full width — 0.1+0.2 spills 17 fractional
    /// digits, whose `int(...)` is a 17-digit whole number ("...kvadrilijonų...").
    #[test]
    fn cardinal_float_messy_reprs() {
        let lt = LangLt::new();
        let rows: Vec<(FloatValue, &str)> = vec![
            (flt(10.0, 1), "dešimt kablelis nulis"),
            (flt(0.25, 2), "nulis kablelis dvidešimt penki"),
            (flt(123.456, 3), "vienas šimtas dvidešimt trys kablelis keturi šimtai penkiasdešimt šeši"),
            (flt(1000000.001, 3), "vienas milijonas kablelis nulis nulis vienas"),
            (flt(3.0, 1), "trys kablelis nulis"),
            (flt(0.0001, 4), "nulis kablelis nulis nulis nulis vienas"),
            (flt(-0.01, 2), "minus nulis kablelis nulis vienas"),
            // 0.1 + 0.2 == 0.30000000000000004 in IEEE-754 (precision 17).
            (flt(0.1 + 0.2, 17), "nulis kablelis trisdešimt kvadrilijonų keturi"),
        ];
        for (v, want) in rows {
            assert_eq!(lt.to_cardinal_float(&v, None).unwrap(), want, "cardinal {:?}", v);
        }
    }
}
