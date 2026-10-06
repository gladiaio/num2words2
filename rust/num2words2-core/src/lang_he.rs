//! Port of `lang_HE.py` (Hebrew).
//!
//! Shape: **self-contained**. `Num2Word_HE` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards`. It overrides `to_cardinal` outright and drives the
//! module-level `int2word`/`chunk2word` pair over 3-digit chunks, so
//! `cards`/`merge` stay at their trait defaults here. `MAXVAL` *is* set (by
//! `setup()`, to 10**66), so the overflow check is real — see [`LangHe::maxval`].
//!
//! Inherited from `Num2Word_Base` unchanged, so the trait defaults are correct:
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`.
//!     Note it does *not* call `verify_ordinal`, so negatives pass through:
//!     `to_ordinal_num(-1)` == "-1".
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` → the default
//!     delegates through `&self` and picks up the `to_cardinal` override below,
//!     with `gender="f"`. Hebrew years are therefore identical to cardinals.
//!   * `verify_ordinal` → the float branch (`value == int(value)`) raises
//!     `TypeError` (`errmsg_floatord`), the negative branch `TypeError`
//!     (`errmsg_negord`). Reproduced inline in [`LangHe::to_ordinal`] (BigInt:
//!     only the negative branch can fire) and in the `ordinal_float_entry`
//!     override (float/Decimal: both). Note `abs(-0.0) == -0.0` is *True* in
//!     Python (value comparison, not sign bit), so `to_ordinal(-0.0)` is
//!     "האפס" — the entry therefore keys off the numeric sign of the
//!     converted int, not `FloatValue::is_negative`.
//!
//! # Grammatical kwargs
//!
//! Python signatures: `to_cardinal(value, gender="f", construct=False)`,
//! `to_ordinal(value, gender="m", definite=False, plural=False)`,
//! `to_currency(val, currency="ILS", cents=True, separator=AND,
//! adjective=False, prefer_singular=False, prefer_singular_cents=False)`.
//! Ported as `to_cardinal_kw` / `to_ordinal_kw` / `to_currency_kw`, plus
//! `to_cardinal_float_kw` for float/Decimal input (`gender` reaches only the
//! integer part; `construct` is dropped by `to_cardinal_float`):
//!
//!   * `gender` is only ever compared with `== "m"`, so *any* other value —
//!     "f", "x", even an explicit `None` — selects the feminine forms. An
//!     explicit `gender=None` is therefore **not** treated like the ordinal's
//!     "m" default.
//!   * `construct`/`definite`/`plural` are used arithmetically in `chunk2word`
//!     (`2 * plural`, `2 * (construct and i == 0)`), so a non-bool value would
//!     change indexes or raise (`2 * None` → TypeError). Only real bools are
//!     handled here; anything else returns NotImplemented so the dispatcher
//!     falls back to Python, which reproduces the exotic behaviour itself.
//!   * `prefer_singular` / `prefer_singular_cents` are accepted by HE's
//!     `to_currency` and then never read (the body forwards neither, and
//!     `pluralize`'s `prefer_singular` limb is unreachable — see
//!     [`LangHe::pluralize`]). Any value is a no-op, so `to_currency_kw`
//!     simply delegates to the plain `to_currency`.
//!   * `_money_verbose` / `_cents_verbose` → `self.to_cardinal(number)`, and
//!     `_cents_terse` → zero-padded digits. All three trait defaults already
//!     do this, and route through the `to_cardinal` override.
//!   * `to_cheque` → `currency::default_to_cheque`. `.upper()` is a no-op on
//!     Hebrew, so only the literal "AND"/"MINUS" are upper-case in the output.
//!   * `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION` are both `{}` — HE descends
//!     from `Num2Word_Base`, not `Num2Word_EUR`, so it inherits neither EUR's
//!     adjective table nor EN's mils precisions (verified against the live
//!     interpreter). Every code is therefore divisor 100, and `adjective=True`
//!     is a no-op. Trait defaults (`None` / 100) already say exactly that.
//!
//! # The currency surface
//!
//! `CURRENCY_FORMS` is `Num2Word_HE`'s own class attribute, so the
//! `lang_EUR.py` mutation trap does not apply: EN rewrites
//! `Num2Word_EUR.CURRENCY_FORMS` in place, but HE neither inherits from
//! `Num2Word_EUR` nor is touched by EN. The live table is exactly the three
//! codes in [`build_currency_forms`] — anything else is a `NotImplementedError`.
//!
//! `Num2Word_HE.to_currency` is the interesting part. It splits on
//! `isinstance(val, int)` and, for a true int, **casts to `float(val)` before
//! delegating to `Num2Word_Base.to_currency`**. That inverts Base's usual
//! int/float split: Base skips the cents segment for ints, but HE never lets
//! Base see an int, so `to_currency(0, "EUR")` renders "אפס אירוו אפס סנטים"
//! — with cents. Then it scrubs the now-redundant zero-cents text back out
//! with a list of hardcoded patterns. See [`strip_zero_cents`].
//!
//! `CURRENCY_GENDERS` and `__init__`'s `makaf` are both dead data in Python —
//! assigned and never read by any code path — so neither is ported. They are
//! called out here because a reviewer diffing against `lang_HE.py` will look
//! for them.
//!
//! # Hebrew-specific behaviour worth knowing
//!
//! * **Cardinals are feminine, ordinals are masculine.** `to_cardinal` defaults
//!   to `gender="f"` and `to_ordinal` to `gender="m"`. That is why 1 is "אחת"
//!   as a cardinal but the ordinal of 2 is "שני".
//! * **Gender flips above the units chunk.** `male = gender == "m" or i > 0`:
//!   any chunk above the last one (thousands, millions, …) is forced masculine
//!   regardless of the caller's gender. So 123456 cardinal has masculine
//!   "שלושה" in the thousands chunk but feminine "שש" in the units chunk.
//! * **The `cop` offset only applies below 11.** `cop = (…4 * ordinal…) * (n < 11)`
//!   keys off the *whole* number `n`, not the chunk, so true ordinal wordforms
//!   ("ראשון", "עשירי") appear only for n in 1..=10. For n >= 11 the ordinal is
//!   just the cardinal with a definite "ה" glued on: `to_ordinal(11)` ==
//!   "האחד עשר", literally "the eleven".
//! * **The "ו" (and) conjunction is applied per chunk, not once at the end.**
//!   `int2word` prefixes AND to `words[-1]` after *every* non-zero chunk, so a
//!   number can carry several: 99999 == "תשעים ותשעה אלף תשע מאות תשעים ותשע".
//! * **Hundreds are always feminine.** The `ONES[n3][0]` in the hundreds branch
//!   ignores gender entirely, hence ordinal 300 == "השלוש מאות".
//!
//! # Faithfully reproduced Python quirks
//!
//! 1. `chunk2word`'s construct-state hundreds test is `construct and n == 100`,
//!    comparing against the **whole number** `n` rather than the current chunk
//!    `x`. A construct-state 100100 would therefore miss the "מאת" form. Kept
//!    verbatim; unreachable from the four in-scope entry points because none of
//!    them passes `construct=True` (see the `construct` note in the report).
//! 2. `HUNDREDS[n3][1]` in that same branch would raise `KeyError` for n3 >= 4
//!    and `IndexError` for n3 == 2 or 3 (those tuples are 1-long). Modelled by
//!    [`hundreds`] rather than being tidied away — same reasoning as (1).
//! 3. `to_ordinal(0)` returns "האפס" ("the zero") rather than raising: the
//!    `n == 0` early-out in `int2word` prepends DEF when `ordinal` is set.
//!
//! # Bounds
//!
//! `MAXVAL` is 10**66 and `LARGE` tops out at key 20 ("ויגינטיליון"). The
//! largest permitted n has 66 digits → 22 chunks → max chunk index `i` == 21 →
//! `LARGE[20]`. The table therefore cannot be over-indexed while the MAXVAL
//! guard holds; [`large`] still returns `N2WError::Key` rather than panicking.

use crate::base::{KwVal, Kwargs, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float_by, float_repr_precision, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

const ZERO: &str = "אפס";
const AND: &str = "ו";
const DEF: &str = "ה";
const NEGWORD: &str = "מינוס";
#[allow(dead_code)] // kept from the port; currently unreferenced (#246)
const POINTWORD: &str = "נקודה";

/// `THOUSANDS[1][0]`.
const THOUSANDS1: &str = "אלף";
/// `THOUSANDS[2][0]`.
const THOUSANDS2: &str = "אלפיים";
/// `THOUSANDS[3]`, two forms: absolute, construct.
const THOUSANDS3: [&str; 2] = ["אלפים", "אלפי"];

const ONES: [[&str; 8]; 10] = [
    ["", "", "", "", "", "", "", ""], // 0: absent in Python (never indexed)
    ["אחת", "אחד", "אחת", "אחד", "ראשונה", "ראשון", "ראשונות", "ראשונים"],
    ["שתיים", "שניים", "שתי", "שני", "שנייה", "שני", "שניות", "שניים"],
    ["שלוש", "שלושה", "שלוש", "שלושת", "שלישית", "שלישי", "שלישיות", "שלישיים"],
    ["ארבע", "ארבעה", "ארבע", "ארבעת", "רביעית", "רביעי", "רביעיות", "רביעיים"],
    ["חמש", "חמישה", "חמש", "חמשת", "חמישית", "חמישי", "חמישיות", "חמישיים"],
    ["שש", "שישה", "שש", "ששת", "שישית", "שישי", "שישיות", "שישיים"],
    ["שבע", "שבעה", "שבע", "שבעת", "שביעית", "שביעי", "שביעיות", "שביעיים"],
    ["שמונה", "שמונה", "שמונה", "שמונת", "שמינית", "שמיני", "שמיניות", "שמיניים"],
    ["תשע", "תשעה", "תשע", "תשעת", "תשיעית", "תשיעי", "תשיעיות", "תשיעיים"],
];

/// `TENS[0]`, eight forms.
const TENS0: [&str; 8] = ["עשר", "עשרה", "עשר", "עשרת", "עשירית", "עשירי", "עשיריות", "עשיריים"];
/// `TENS[1]`, two forms (feminine, masculine).
const TENS1: [&str; 2] = ["עשרה", "עשר"];
/// `TENS[2]`, two forms (feminine, masculine).
const TENS2: [&str; 2] = ["שתים עשרה", "שנים עשר"];

const TWENTIES: [&str; 10] = [
    "", "", // 0, 1: absent in Python (guarded by `n2 > 1`)
    "עשרים",
    "שלושים",
    "ארבעים",
    "חמישים",
    "שישים",
    "שבעים",
    "שמונים",
    "תשעים",
];

const LARGE: [(&str, &str); 21] = [
    ("", ""), // 0: absent in Python
    ("מיליון", "מיליוני"),
    ("מיליארד", "מיליארדי"),
    ("טריליון", "טריליוני"),
    ("קוודריליון", "קוודריליוני"),
    ("קווינטיליון", "קווינטיליוני"),
    ("סקסטיליון", "סקסטיליוני"),
    ("ספטיליון", "ספטיליוני"),
    ("אוקטיליון", "אוקטיליוני"),
    ("נוניליון", "נוניליוני"),
    ("דסיליון", "דסיליוני"),
    ("אונדסיליון", "אונדסיליוני"),
    ("דואודסיליון", "דואודסיליוני"),
    ("טרדסיליון", "טרדסיליוני"),
    ("קווטואורדסיליון", "קווטואורדסיליוני"),
    ("קווינדסיליון", "קווינדסיליוני"),
    ("סקסדסיליון", "סקסדסיליוני"),
    ("ספטנדסיליון", "ספטנדסיליוני"),
    ("אוקטודסיליון", "אוקטודסיליוני"),
    ("נובמדסיליון", "נובמדסיליוני"),
    ("ויגינטיליון", "ויגינטיליוני"),
];

/// `HUNDREDS = {1: ("מאה", "מאת"), 2: ("מאתיים",), 3: ("מאות",)}`.
///
/// Modelled as a lookup rather than an array because the ragged tuple lengths
/// are load-bearing: Python raises `KeyError` for an absent key and
/// `IndexError` for an out-of-range form, and `chunk2word` has a branch
/// (`construct and n == 100`) that can reach both. See quirk (2) in the module
/// docs.
fn hundreds(key: u32, idx: usize) -> Result<&'static str> {
    let row: &[&str] = match key {
        1 => &["מאה", "מאת"],
        2 => &["מאתיים"],
        3 => &["מאות"],
        _ => return Err(N2WError::Key(format!("{}", key))),
    };
    row.get(idx)
        .copied()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// `THOUSANDS[key][0]` for the small keys. Reached only with `key` in 1..=2
/// (guarded by `n1 != 0 && n1 <= 2`); key 3 is spelled out for completeness.
fn thousands_small(key: u32) -> Result<&'static str> {
    match key {
        1 => Ok(THOUSANDS1),
        2 => Ok(THOUSANDS2),
        3 => Ok(THOUSANDS3[0]),
        _ => Err(N2WError::Key(format!("{}", key))),
    }
}

/// `LARGE[key][idx]`. `key` 0 is absent in the Python dict.
fn large(key: usize, idx: usize) -> Result<&'static str> {
    if key == 0 || key >= LARGE.len() {
        return Err(N2WError::Key(format!("{}", key)));
    }
    let (abs, construct) = LARGE[key];
    match idx {
        0 => Ok(abs),
        1 => Ok(construct),
        _ => Err(N2WError::Index("tuple index out of range".into())),
    }
}

/// Index of the last element, or Python's `IndexError` on an empty list.
///
/// Python writes `words[-1]`, which throws on an empty list. Every call site
/// here is provably non-empty (a chunk with `x >= 11` always appends at least
/// one word first), but the check keeps a Rust panic off the table.
fn last_index(words: &[String]) -> Result<usize> {
    words
        .len()
        .checked_sub(1)
        .ok_or_else(|| N2WError::Index("list index out of range".into()))
}

/// `utils.splitbyx(s, 3)` — split a digit string into 3-digit chunks, the
/// leftmost possibly short.
///
/// `s` is always the decimal form of a non-negative integer here (`int2word`
/// is only ever handed `abs(value)`), so every slice parses and each chunk
/// fits in a `u32`.
fn splitbyx3(s: &str) -> Vec<u32> {
    let length = s.len();
    let x = 3usize;
    let mut out = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(s[..start].parse::<u32>().unwrap_or(0));
        }
        let mut i = start;
        while i < length {
            let end = (i + x).min(length);
            out.push(s[i..end].parse::<u32>().unwrap_or(0));
            i += x;
        }
    } else {
        out.push(s.parse::<u32>().unwrap_or(0));
    }
    out
}

/// `utils.get_digits(x)` → `(n1, n2, n3)` = (units, tens, hundreds).
///
/// Python formats `"%03d" % n`, takes the last 3 characters and reverses them.
/// `x` is always a chunk in 0..=999 here, so `"%03d"` is exactly 3 digits and
/// the slice is a no-op — plain arithmetic is equivalent.
fn get_digits(x: u32) -> (u32, u32, u32) {
    (x % 10, (x / 10) % 10, (x / 100) % 10)
}

/// Python's module-level `chunk2word`.
///
/// `n` is the whole number; `i` the chunk index counting down from the top
/// (0 == units chunk); `x` the chunk value in 0..=999. `gender_m` is Python's
/// `gender == "m"`.
#[allow(clippy::too_many_arguments)]
fn chunk2word(
    n: &BigInt,
    i: usize,
    x: u32,
    gender_m: bool,
    construct: bool,
    ordinal: bool,
    plural: bool,
) -> Result<Vec<String>> {
    let mut words: Vec<String> = Vec::new();
    let (n1, n2, n3) = get_digits(x);

    if n3 > 0 {
        if construct && n == &BigInt::from(100) {
            // Quirk (1)+(2): compares the whole `n`, and can raise Key/Index.
            words.push(hundreds(n3, 1)?.to_string());
        } else if n3 <= 2 {
            words.push(hundreds(n3, 0)?.to_string());
        } else {
            // Hundreds always take the feminine ONES form, whatever the gender.
            words.push(format!("{} {}", ONES[n3 as usize][0], hundreds(3, 0)?));
        }
    }

    if n2 > 1 {
        words.push(TWENTIES[n2 as usize].to_string());
    }

    if i == 0 || x >= 11 {
        // Python: `male = gender == "m" or i > 0` — a bool used as an index.
        let male = usize::from(gender_m || i > 0);
        // Python: `cop = (2 * (construct and i == 0) + 4 * ordinal + 2 * plural) * (n < 11)`
        // Note the `n < 11` keys off the whole number, not the chunk.
        let cop = (2 * usize::from(construct && i == 0)
            + 4 * usize::from(ordinal)
            + 2 * usize::from(plural))
            * usize::from(n < &BigInt::from(11));
        if n2 == 1 {
            if n1 == 0 {
                // Python indexes TENS[n1], i.e. the literal key 0 in this branch.
                words.push(TENS0[male + cop].to_string());
            } else if n1 == 2 {
                // TENS[2] has only 2 forms; Python indexes it with `male` alone.
                words.push(TENS2[male].to_string());
            } else {
                words.push(format!("{} {}", ONES[n1 as usize][male], TENS1[male]));
            }
        } else if n1 > 0 {
            words.push(ONES[n1 as usize][male + cop].to_string());
        }
    }

    // Python: `construct_last = construct and (n % 1000**i == 0)` — evaluates
    // to the bool `False` whenever `construct` is False, which is every call
    // reachable from the four in-scope entry points.
    let construct_last =
        construct && (n % BigInt::from(1000u32).pow(i as u32)).is_zero();
    let cl = usize::from(construct_last);

    if i == 1 {
        if x >= 11 {
            let last = last_index(&words)?;
            words[last].push(' ');
            words[last].push_str(THOUSANDS1);
        } else if n1 == 0 {
            // x can only be 10 here (x != 0, x < 11, n1 == 0).
            words.push(format!("{} {}", TENS0[3], THOUSANDS3[cl]));
        } else if n1 <= 2 {
            words.push(thousands_small(n1)?.to_string());
        } else {
            words.push(format!("{} {}", ONES[n1 as usize][3], THOUSANDS3[cl]));
        }
    } else if i > 1 {
        if x >= 11 {
            let suffix = large(i - 1, cl)?;
            let last = last_index(&words)?;
            words[last].push(' ');
            words[last].push_str(suffix);
        } else if n1 == 0 {
            words.push(format!("{} {}", TENS0[1 + 2 * cl], large(i - 1, cl)?));
        } else if n1 == 1 {
            words.push(large(i - 1, 0)?.to_string());
        } else {
            // `x == 2` <=> `n1 == 2` here (x < 11 and n1 >= 2 forces x == n1).
            let idx = 1 + 2 * usize::from(construct_last || x == 2);
            words.push(format!("{} {}", ONES[n1 as usize][idx], large(i - 1, cl)?));
        }
    }

    Ok(words)
}

/// Python's module-level `int2word`.
///
/// The three `assert`s at the top of the Python function
/// (`n == int(n)`, `not construct or not ordinal`,
/// `ordinal or (not definite and not plural)`) are all satisfied by every
/// in-scope call site, so they are documented rather than modelled.
fn int2word(
    n: &BigInt,
    gender_m: bool,
    construct: bool,
    ordinal: bool,
    definite: bool,
    plural: bool,
) -> Result<String> {
    if n >= maxval_ref() {
        return Err(N2WError::Overflow(format!(
            "abs({}) must be less than {}.",
            n,
            maxval_ref()
        )));
    }

    if n.is_zero() {
        // Quirk (3): the ordinal of zero is "האפס", not an error.
        return Ok(if ordinal {
            format!("{}{}", DEF, ZERO)
        } else {
            ZERO.to_string()
        });
    }

    let mut words: Vec<String> = Vec::new();
    let chunks = splitbyx3(&n.to_string());
    let mut i = chunks.len();
    for x in chunks {
        i -= 1;

        if x == 0 {
            continue;
        }

        words.extend(chunk2word(n, i, x, gender_m, construct, ordinal, plural)?);

        // The AND conjunction is applied once per non-zero chunk, to whatever
        // is currently last — not once at the very end.
        if words.len() > 1 {
            let last = last_index(&words)?;
            words[last].insert_str(0, AND);
        }
    }

    if ordinal && (n >= &BigInt::from(11) || definite) {
        if words.is_empty() {
            return Err(N2WError::Index("list index out of range".into()));
        }
        words[0].insert_str(0, DEF);
    }

    Ok(words.join(" "))
}

/// Python `str(float)`, for `verify_ordinal`'s error messages only.
///
/// The corpora record only the exception *type*; the message just has to
/// match `errmsg_floatord`/`errmsg_negord`'s `%s` formatting closely.
/// Python repr rules: whole values keep a trailing ".0" ("-1000000.0",
/// "-0.0"); |v| >= 1e16 or 0 < |v| < 1e-4 switch to exponent form with an
/// explicit sign and >= 2 exponent digits ("1e+16", "1e-05").
fn py_float_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    let a = f.abs();
    if a != 0.0 && !(1e-4..1e16).contains(&a) {
        // Rust's `{:e}` is shortest-round-trip like Python's repr, but writes
        // "1e16" where Python writes "1e+16" (signed, zero-padded to 2).
        let s = format!("{:e}", f);
        if let Some(pos) = s.find('e') {
            let exp: i32 = s[pos + 1..].parse().unwrap_or(0);
            let sign = if exp < 0 { '-' } else { '+' };
            return format!("{}e{}{:02}", &s[..pos], sign, exp.abs());
        }
        s
    } else if f.fract() == 0.0 {
        // `{:.1}` keeps the ".0" and the sign of -0.0.
        format!("{:.1}", f)
    } else {
        format!("{}", f)
    }
}

/// Python `str(value)` of a float-or-Decimal input, for error messages.
fn float_value_str(v: &crate::floatpath::FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => py_float_str(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// `gender` kwarg → Python's `gender == "m"`.
///
/// Absent means the signature default (`default_m`); a present value of any
/// other shape — "f", "x", `None`, a non-string — compares unequal to "m"
/// and selects the feminine forms. See the module docs.
fn kw_gender_m(kw: &Kwargs, default_m: bool) -> bool {
    match kw.get("gender") {
        None => default_m,
        Some(KwVal::Str(s)) => s == "m",
        Some(_) => false,
    }
}

/// A bool-only kwarg (`construct`/`definite`/`plural`): absent → `false`,
/// a real bool → itself, anything else → `None` (fall back to Python, which
/// reproduces the arithmetic-on-non-bool behaviour — see the module docs).
fn kw_flag(kw: &Kwargs, key: &str) -> Option<bool> {
    match kw.get(key) {
        None => Some(false),
        Some(KwVal::Bool(b)) => Some(*b),
        Some(_) => None,
    }
}

/// `MAXVAL = int("1" + "0" * 66)` == 10**66, installed by `setup()`.
fn maxval_ref() -> &'static BigInt {
    static MAXVAL: OnceLock<BigInt> = OnceLock::new();
    MAXVAL.get_or_init(|| BigInt::from(10u32).pow(66))
}

/// `Num2Word_HE.CURRENCY_FORMS`, verbatim.
///
/// HE's own class attribute, shadowing `Num2Word_Base`'s empty dict. Confirmed
/// against the live `CONVERTER_CLASSES["he"].CURRENCY_FORMS`: exactly these
/// three codes, no EN/EUR leakage.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 2] = ["סנט", "סנטים"];
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "ILS",
        CurrencyForms::new(&["שקל", "שקלים"], &["אגורה", "אגורות"]),
    );
    m.insert("EUR", CurrencyForms::new(&["אירו", "אירו"], &CENTS));
    m.insert("USD", CurrencyForms::new(&["דולר", "דולרים"], &CENTS));
    m
}

/// `Num2Word_HE.to_currency`'s `zero_patterns` list, verbatim and in order
/// (including the duplicated "нула стотинки").
///
/// Copied wholesale from some upstream multi-language sweep: only
/// "אפס אגורות" — ILS's zero cents — can ever occur in a Hebrew result, since
/// every other piece of `result` comes from HE's own Hebrew wordlists. The
/// rest are dead weight, kept so the loop matches Python element for element.
const ZERO_PATTERNS: [&str; 21] = [
    "zero cent",
    "nul cent",
    "null cent",
    "sıfır kuruş",
    "אפס אגורות",
    "zero sen",
    "ศูนย์สตางค์",
    "không xu",
    "शून्य पैसे",
    "শূন্য পয়সা",
    "nula lipa",
    "нула пара",
    "ноль копеек",
    "нула стотинки",
    "零分",
    "ዜሮ ሳንቲም",
    "صفر",
    "sero sent",
    "dim ceiniog",
    "ნულოვანი თეთრი",
    "нула стотинки",
];

/// The alternation inside `to_currency`'s connecting-word regex, verbatim and
/// in order (duplicates and all — "და" appears four times).
///
/// Note Hebrew's own conjunction "ו" (U+05D5) is *not* in the list; the Arabic
/// "و" (U+0648) that is there only looks like it. That omission is why the
/// default `separator="ו"` survives the scrub and leaves a dangling "ו" — see
/// [`strip_zero_cents`].
const CONJUNCTIONS: [&str; 24] = [
    "and", "və", "և", "და", "ir", "და", "და", "و", "و", "与", "ja", "और", "এবং", "i", "и", "и",
    "と", "그리고", "และ", "và", "dan", "a", "e", "და",
];

/// Python's *simple* case fold for one `char`.
///
/// `char::to_lowercase` can expand one char into several ("İ" → "i̇"), which
/// Python's `re` does not do when matching a literal under `re.IGNORECASE`. An
/// expansion is therefore treated as "no simple lowercase" and the char is
/// compared as-is, which keeps the comparison 1:1 and — crucially — keeps
/// haystack byte offsets valid.
fn simple_lower(c: char) -> char {
    let mut it = c.to_lowercase();
    match (it.next(), it.next()) {
        (Some(l), None) => l,
        _ => c,
    }
}

/// Case-insensitive literal match of `needle` at byte offset `at`.
/// Returns the number of *haystack* bytes consumed, or `None`.
fn match_ci_at(hay: &str, at: usize, needle: &str) -> Option<usize> {
    let mut h = hay[at..].chars();
    let mut consumed = 0usize;
    for nc in needle.chars() {
        let hc = h.next()?;
        if simple_lower(hc) != simple_lower(nc) {
            return None;
        }
        consumed += hc.len_utf8();
    }
    Some(consumed)
}

/// Leftmost case-insensitive occurrence of `needle` at or after `from`, as
/// `(start, end)` byte offsets.
fn find_ci(hay: &str, needle: &str, from: usize) -> Option<(usize, usize)> {
    let mut i = from;
    while i <= hay.len() {
        if hay.is_char_boundary(i) {
            if let Some(len) = match_ci_at(hay, i, needle) {
                return Some((i, i + len));
            }
        }
        i += 1;
    }
    None
}

/// Greedy `\s+` at `at`: the offset just past the run, or `None` if there is
/// no whitespace there at all.
fn skip_ws1(hay: &str, at: usize) -> Option<usize> {
    let mut i = at;
    for c in hay[at..].chars() {
        if !c.is_whitespace() {
            break;
        }
        i += c.len_utf8();
    }
    if i > at {
        Some(i)
    } else {
        None
    }
}

/// `re.sub(re.escape(needle), "", hay, flags=re.IGNORECASE)`.
fn remove_all_ci(hay: &str, needle: &str) -> String {
    if needle.is_empty() {
        return hay.to_string();
    }
    let mut out = String::with_capacity(hay.len());
    let mut i = 0usize;
    while let Some((s, e)) = find_ci(hay, needle, i) {
        out.push_str(&hay[i..s]);
        i = e;
    }
    out.push_str(&hay[i..]);
    out
}

/// One match of `\s+(<conj>)\s+<pattern>` starting at `at`; returns its end.
///
/// Both `\s+` are greedy and need no backtracking: no conjunction and no zero
/// pattern begins with whitespace, so the maximal run is always the right one.
/// Alternatives are tried in Python's source order, and a conjunction that
/// matches but is not followed by `\s+<pattern>` falls through to the next —
/// which is what `re`'s backtracking does.
fn match_conjoined(hay: &str, at: usize, pattern: &str) -> Option<usize> {
    let a = skip_ws1(hay, at)?;
    for conj in CONJUNCTIONS {
        let Some(clen) = match_ci_at(hay, a, conj) else {
            continue;
        };
        let Some(b) = skip_ws1(hay, a + clen) else {
            continue;
        };
        if let Some(plen) = match_ci_at(hay, b, pattern) {
            return Some(b + plen);
        }
    }
    None
}

/// `re.sub(r"\s+(<conj>)\s+" + re.escape(pattern), "", hay, flags=re.IGNORECASE)`.
fn remove_conjoined(hay: &str, pattern: &str) -> String {
    let mut out = String::with_capacity(hay.len());
    let mut i = 0usize;
    let mut cut = 0usize;
    while i < hay.len() {
        if !hay.is_char_boundary(i) {
            i += 1;
            continue;
        }
        // `match_conjoined` needs >= 1 whitespace char, so `end > i` always
        // and this cannot spin.
        if let Some(end) = match_conjoined(hay, i, pattern) {
            out.push_str(&hay[cut..i]);
            cut = end;
            i = end;
            continue;
        }
        i += 1;
    }
    out.push_str(&hay[cut..]);
    out
}

/// The zero-cents scrub `Num2Word_HE.to_currency` runs over the **int** result.
///
/// ```python
/// for pattern in zero_patterns:
///     if pattern in result.lower():
///         result = re.sub(r"\s+(and|...)\s+" + re.escape(pattern), "", result, flags=re.IGNORECASE)
///         result = re.sub(re.escape(pattern), "", result, flags=re.IGNORECASE)
///         result = " ".join(result.split())
/// return result.strip()
/// ```
///
/// The whitespace normalisation sits *inside* the `if`, so it only runs for a
/// pattern that actually hit; `.strip()` always runs. Both are reproduced.
///
/// **Python bug, reproduced**: for ILS the conjunction sub never fires, because
/// HE's default separator "ו" is glued to the preceding word ("שקליםו") and is
/// not in `CONJUNCTIONS` anyway. So only the literal sub runs and the separator
/// is orphaned: `to_currency(100, "ILS")` == "מאה שקליםו" — "a hundred shekels
/// and". Verified against the live interpreter.
///
/// The containment guard uses `find_ci` (simple per-char fold) where Python
/// uses `str.lower()` (full fold). They can only disagree on chars whose
/// lowercase expands, none of which occur in `ZERO_PATTERNS` or in any string
/// HE can build.
fn strip_zero_cents(result: &str) -> String {
    let mut result = result.to_string();
    for pattern in ZERO_PATTERNS {
        if find_ci(&result, pattern, 0).is_some() {
            result = remove_conjoined(&result, pattern);
            result = remove_all_ci(&result, pattern);
            result = result.split_whitespace().collect::<Vec<_>>().join(" ");
        }
    }
    result.trim().to_string()
}

pub struct LangHe {
    /// Python's `Num2Word_HE.__init__(self, makaf="-")`. Assigned there and
    /// read by nothing in the whole package (`grep -rn makaf num2words2/`
    /// finds only the two lines of `__init__`), currency included. Stored for
    /// parity with the Python constructor and otherwise unused.
    #[allow(dead_code)]
    makaf: String,
    /// Built once, here. `to_currency`/`to_cheque` only ever read this table,
    /// and rebuilding it per call is what made an earlier revision of the port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangHe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangHe {
    pub fn new() -> Self {
        LangHe {
            makaf: "-".to_string(),
            currency_forms: build_currency_forms(),
        }
    }
}

impl Lang for LangHe {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ILS"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        "ו"
    }

    fn maxval(&self) -> &BigInt {
        maxval_ref()
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "נקודה"
    }

    /// `Num2Word_HE.to_cardinal(value, gender="f", construct=False)`.
    ///
    /// Note this override does *not* call `self.title()` (the base's version
    /// does); `is_title` is False for Hebrew either way.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = format!("{} ", NEGWORD.trim());
        }

        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v,
                self.maxval()
            )));
        }

        Ok(format!(
            "{}{}",
            out,
            int2word(&v, false, false, false, false, false)?
        ))
    }

    /// `Num2Word_HE.to_ordinal(value, gender="m", definite=False, plural=False)`.
    ///
    /// The `verify_ordinal` call is inlined: its float branch cannot fire on a
    /// `BigInt`, and its negative branch raises `TypeError` with
    /// `errmsg_negord`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }

        if value >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                value,
                self.maxval()
            )));
        }

        int2word(value, true, false, true, false, false)
    }

    /// `Num2Word_HE.to_cardinal_float(value, gender="f")`.
    ///
    /// HE overrides `to_cardinal_float` and, unlike `Num2Word_Base`, **always
    /// float-casts** (`float_value = float(value)`) before `float2tuple`. Two
    /// consequences, both load-bearing and both verified against the live
    /// interpreter:
    ///
    ///  1. **Decimal input is routed through the *float* branch**, so it picks
    ///     up the binary-f64 rounding that base.py's #603 fix deliberately
    ///     avoids. `Decimal("98746251323029.99")` → `float` `…029.98` →
    ///     "…עשרים ותשע נקודה תשע שמונה" (…29 point nine **eight**), and
    ///     `Decimal("1.10")` → `float` `1.1` → precision 1 → "אחת נקודה אחת"
    ///     (one point one), *not* the exact-Decimal "…אחת אפס". The inherited
    ///     `Num2Word_Base.to_cardinal_float` (the trait default) would keep the
    ///     Decimal exact and get both wrong — that is why HE overrides here.
    ///  2. **`precision=` is ignored.** `float2tuple` recomputes
    ///     `self.precision = abs(Decimal(str(float_value)).as_tuple().exponent)`,
    ///     clobbering the override num2words installs on `self.precision`. Live:
    ///     `num2words(1.5, lang="he", precision=4)` == "אחת נקודה חמש" (precision
    ///     1). So `precision_override` is deliberately dropped.
    ///
    /// For *float* input HE's method is byte-identical to base's (`gender="f"`
    /// is the default and `to_cardinal(pre)` already renders feminine), so the
    /// shared `default_to_cardinal_float` is reused directly. For *Decimal*
    /// input the float-cast + repr-precision is exactly what
    /// `cardinal_from_bigdecimal` performs.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { .. } => {
                crate::floatpath::default_to_cardinal_float(self, value, None)
            }
            FloatValue::Decimal { value: d, .. } => {
                crate::floatpath::cardinal_from_bigdecimal(self, d)
            }
        }
    }

    /// `to_ordinal(float/Decimal)` — `Num2Word_HE.to_ordinal` starts with
    /// `Num2Word_Base.verify_ordinal`, so the float path is *not* routed like
    /// the cardinal (the trait default): a non-whole value raises `TypeError`
    /// (`errmsg_floatord`), a negative whole value raises `TypeError`
    /// (`errmsg_negord`), and a non-negative whole value takes the ordinal
    /// int path — `to_ordinal(5.0)` == "חמישי", `to_ordinal(11.0)` ==
    /// "האחד עשר".
    ///
    /// Ordering quirks reproduced from `verify_ordinal`:
    ///  * The float check (`value == int(value)`) runs first, so `-1.5` gets
    ///    the *floatord* message, not negord.
    ///  * The negative check is `abs(value) == value` — a value comparison,
    ///    so `-0.0` passes (`abs(-0.0) == -0.0` is True) and renders "האפס".
    ///    That is why this keys off the converted int's sign rather than
    ///    `FloatValue::is_negative` (which is sign-bit aware).
    ///  * `int(inf)`/`int(nan)` inside the first check raise
    ///    OverflowError/ValueError before any comparison; modelled up front
    ///    for completeness (the dispatcher keeps non-finite floats on the
    ///    Python side, so this arm is belt-and-braces).
    ///
    /// After verify_ordinal, `Num2Word_HE.to_ordinal` checks
    /// `value >= self.MAXVAL` (OverflowError, `errmsg_toobig` formatted with
    /// the *original* value) and calls
    /// `int2word(int(value), gender="m", ordinal=True)`.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let FloatValue::Float { value: f, .. } = value {
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
            if f.is_nan() {
                return Err(N2WError::Value(
                    "cannot convert float NaN to integer".into(),
                ));
            }
        }
        let Some(n) = value.as_whole_int() else {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                float_value_str(value)
            )));
        };
        if n.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                float_value_str(value)
            )));
        }
        if &n >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                float_value_str(value),
                self.maxval()
            )));
        }
        int2word(&n, true, false, true, false, false)
    }

    // to_ordinal_num: inherited from Num2Word_Base (returns the value
    // unchanged, no verify_ordinal) → trait default `Ok(value.to_string())`.
    // to_year: inherited from Num2Word_Base (delegates to to_cardinal) → trait
    // default, which routes through the to_cardinal override above.

    // ---- grammatical kwargs ----------------------------------------------

    /// `to_cardinal(value, gender="f", construct=False)` with kwargs.
    ///
    /// Same body as [`LangHe::to_cardinal`], with `gender`/`construct`
    /// forwarded into `int2word`. Corpus-verified quirks: `construct=True`
    /// only changes the units chunk when the *whole* number is below 11
    /// (`cop`'s `n < 11` factor) or exactly 100 ("מאת"), so
    /// `to_cardinal(1234, construct=True)` equals the plain cardinal.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender", "construct"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender_m = kw_gender_m(kw, false);
        let Some(construct) = kw_flag(kw, "construct") else {
            return Err(N2WError::Fallback("kwargs".into()));
        };

        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = format!("{} ", NEGWORD.trim());
        }

        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v,
                self.maxval()
            )));
        }

        Ok(format!(
            "{}{}",
            out,
            int2word(&v, gender_m, construct, false, false, false)?
        ))
    }

    /// `to_cardinal(float/Decimal, gender="f", construct=False)`.
    ///
    /// A whole value passes `int(value) == value` and takes the integer path
    /// with both kwargs. Any other value goes to `to_cardinal_float(value,
    /// gender=gender)` (`construct` is dropped there), where only the integer
    /// part is gendered — the digits after the point are `to_cardinal(curr)`,
    /// feminine. Same float cast as the kwarg-free [`LangHe::to_cardinal_float`],
    /// and the same ignored `precision=`.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["gender", "construct"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        if let Some(i) = value.as_whole_int() {
            return self.to_cardinal_kw(&i, kw);
        }
        let gender = Kwargs(kw.0.iter().filter(|(k, _)| k == "gender").cloned().collect());
        let fv = match value {
            FloatValue::Float { .. } => value.clone(),
            FloatValue::Decimal { value: d, .. } => {
                let f = d.to_f64().ok_or_else(|| {
                    N2WError::Value(format!("cannot represent {} as f64", d))
                })?;
                FloatValue::Float { value: f, precision: float_repr_precision(f) }
            }
        };
        default_to_cardinal_float_by(self, &fv, None, |pre| self.to_cardinal_kw(pre, &gender))
    }

    /// `to_ordinal(value, gender="m", definite=False, plural=False)` with
    /// kwargs.
    ///
    /// `definite=True` forces the "ה" prefix below 11 (11 and up already get
    /// it); `plural=True` selects the plural ordinal forms, but only for
    /// n in 1..=10 (`cop`'s `n < 11` factor) — `to_ordinal(11, plural=True)`
    /// is still "האחד עשר". Negatives raise `TypeError` via `verify_ordinal`,
    /// exactly as in the plain [`LangHe::to_ordinal`].
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender", "definite", "plural"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender_m = kw_gender_m(kw, true);
        let (Some(definite), Some(plural)) = (kw_flag(kw, "definite"), kw_flag(kw, "plural"))
        else {
            return Err(N2WError::Fallback("kwargs".into()));
        };

        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }

        if value >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                value,
                self.maxval()
            )));
        }

        int2word(value, gender_m, false, true, definite, plural)
    }

    /// `to_currency(..., prefer_singular=False, prefer_singular_cents=False)`
    /// with kwargs.
    ///
    /// Both kwargs are accepted by `Num2Word_HE.to_currency` and then never
    /// read — the body forwards neither to `super().to_currency` nor to
    /// `pluralize` (whose `prefer_singular` limb is unreachable, see
    /// [`LangHe::pluralize`]). Any value, of any type, is a no-op in Python,
    /// so this delegates unconditionally to the plain `to_currency`.
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["prefer_singular", "prefer_singular_cents"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.to_currency(val, currency, cents, separator, adjective)
    }

    // ---- currency -------------------------------------------------------
    //
    // HE overrides only `CURRENCY_FORMS`, `pluralize` and `to_currency`.
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` come
    // straight from `Num2Word_Base`, and `CURRENCY_ADJECTIVES` /
    // `CURRENCY_PRECISION` are both empty — the trait defaults already mirror
    // all of that, so they are deliberately not overridden here.

    fn lang_name(&self) -> &str {
        "Num2Word_HE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_HE.pluralize(n, forms, currency=None, prefer_singular=False)`.
    ///
    /// ```python
    /// form = 1
    /// if n == 1 or prefer_singular and (abs(n) >= 11 or n == 0 or currency != "ILS"):
    ///     form = 0
    /// return forms[form]
    /// ```
    ///
    /// Every call reaching here comes from `Num2Word_Base.to_currency` /
    /// `to_cheque`, which pass `self.pluralize(n, forms)` positionally — so
    /// `currency` stays `None` and `prefer_singular` stays `False`. `False and
    /// (...)` short-circuits, collapsing the rule to `forms[0 if n == 1 else 1]`
    /// and making the whole `currency`/`prefer_singular` limb unreachable. HE's
    /// own `to_currency` accepts `prefer_singular`/`prefer_singular_cents`
    /// kwargs but never forwards them, so they are dead too — that is why the
    /// trait's two-argument `pluralize` is a faithful signature here.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise `IndexError`. All three HE entries have two forms, so this
    /// is unreachable; mapped to `Index` rather than panicking anyway.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_HE.to_currency`.
    ///
    /// Two paths, and the split is load-bearing:
    ///
    /// * **float/Decimal** → straight through to `Num2Word_Base.to_currency`.
    /// * **int** → `super().to_currency(float(val), ...)`, i.e. cast to a float
    ///   *first*. Base's `isinstance(val, int)` early-out — the one that skips
    ///   the cents segment — therefore never fires for HE, and `has_decimal` is
    ///   `True` via `isinstance(val, float)`. The int renders *with* cents, and
    ///   the redundant zero-cents text is then scrubbed by
    ///   [`strip_zero_cents`].
    ///
    /// The asymmetry is observable: `to_currency(1, "ILS")` == "אחת שקלו"
    /// (scrubbed) but `to_currency(1.0, "ILS")` == "אחת שקלו אפס אגורות"
    /// (not scrubbed — floats never reach the scrub).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us `None` when the caller omitted `separator=`; the
        // resolution normally done by the default body has to happen here
        // because we are replacing that body. `default_separator` is already
        // generated from HE's live signature (`separator=AND`, i.e. "ו").
        let separator = separator.unwrap_or(self.default_separator());

        let CurrencyValue::Int(v) = val else {
            // `return super().to_currency(val, ...)` — no scrub for floats.
            return crate::currency::default_to_currency(
                self, val, currency, cents, separator, adjective,
            );
        };

        // Python's `float(val)`, modelled as an *exact* widening.
        //
        // Faithful for every int f64 represents exactly (|v| <= 2**53), which
        // is every corpus row and every plausible money amount. Beyond that
        // the cast is lossy in Python and this deliberately is not, so three
        // divergences remain — all measured against the live interpreter, none
        // fixable from this file:
        //
        //  * |v| > 2**53: Python rounds through the f64. `to_currency(2**53+1)`
        //    says "...תשעים ושתיים" (…992); this says …993. Reproducing it
        //    needs f64 rounding *plus* Python's `repr` shortest-round-trip (it
        //    is `Decimal(str(float(v)))`, not the exact binary value), which is
        //    the second formatter PORTING_CURRENCY.md exists to avoid.
        //  * |v| >= 10**26: Python raises `decimal.InvalidOperation` —
        //    `quantize` needs digits(v)+2 > the default 28-digit context. Not
        //    an HE bug: `currency.rs`'s `round_half_up` models no context
        //    limit, so every language already diverges here on the *float*
        //    path (`en` raises InvalidOperation for 1e26 too). HE is only
        //    unusual in reaching it from an *int*, because of this cast — `en`
        //    renders 10**26 fine via Base's int early-out. No `N2WError`
        //    variant fits a stdlib `decimal.InvalidOperation`, and the fix
        //    belongs in `currency.rs`, not here.
        //  * |v| >= ~1.8e308: Python's `float()` itself raises
        //    OverflowError("int too large to convert to float"). Masked in
        //    practice — MAXVAL is 10**66, so `to_cardinal` already raises
        //    OverflowError (different message, same type) long before.
        let as_float = CurrencyValue::Decimal {
            value: BigDecimal::from(v.clone()),
            has_decimal: true,
            is_float: true,
        };
        let result = crate::currency::default_to_currency(
            self, &as_float, currency, cents, separator, adjective,
        )?;
        Ok(strip_zero_cents(&result))
    }
}
