//! Port of `lang_RM_SUTSILV.py` (Romansh — Sutsilvan idiom).
//!
//! Key `"rm_sutsilv"` in `CONVERTER_CLASSES` resolves to
//! `lang_RM_SUTSILV.Num2Word_RM_SUTSILV` — verified, no aliasing surprise.
//!
//! Shape: **self-contained**. `Num2Word_RM_SUTSILV` is a *bare* class — it
//! subclasses nothing (not even `Num2Word_Base`) and its `__init__` is `pass`.
//! It therefore defines exactly five public methods: `float_to_words`,
//! `tens_to_cardinal`, `hundreds_to_cardinal`, `thousands_to_cardinal`,
//! `big_number_to_cardinal`, `to_cardinal` and `to_ordinal`. `cards` and
//! `merge` stay at their trait defaults and are never reached; `maxval()`
//! reports the ceiling below.
//!
//! # Missing methods (fixed, gladiaio/num2words2#157)
//!
//! The class has no base, so in Python `to_ordinal_num`, `to_year`,
//! `to_currency`, `to_cheque`, `to_fraction` and `str_to_number` simply did
//! not exist: those modes, and every string input, raised `AttributeError`
//! before any conversion ran, and a `Decimal` crashed in the integer ladder.
//! The port now follows `lang_rm` (see its module docs): strings use the
//! shared `Decimal(value)` parse, a `Decimal` reads like the int (whole) or
//! the float (fractional, exact digits), `to_year` is the cardinal as in
//! `Num2Word_Base`, and `to_ordinal_num`, currency, cheques and fractions
//! raise a `NotImplementedError` naming the language — no ordinal-numeral
//! marker or currency wording is attested to port.
//!
//! # Ceiling
//!
//! `big_number_to_cardinal` raises `NotImplementedError("The given number is
//! too large.")` once `len(str(number)) >= 66`, i.e. at **10**65** exactly
//! (10**65 - 1 is the largest convertible value). `to_cardinal` tests the sign
//! *first*, so -10**65 raises too. Python raises `NotImplementedError`;
//! fixed (gladiaio/num2words2#159): the port raises `OverflowError` ("abs(v)
//! must be less than 10^65.").
//!
//! # Faithfully reproduced Python oddities
//!
//! All preserved verbatim; each is corpus-confirmed unless noted.
//!
//! 1. **`egn`/`egna` gender agreement fires only for `miliarda`, never for
//!    `biliarda`/`triliarda`/…** `adapt_milliarda` matches the literal
//!    `" egn miliarda "`, and the higher -iarda terms are `"biliarda"`,
//!    `"triliarda"`, … which do not contain `" miliarda "`. So 10**9 →
//!    "egn**a** miliarda" but 10**15 → "egn biliarda" and 10**21 → "egn
//!    triliarda". Corpus-confirmed on all three.
//! 2. **`thousands_to_cardinal`'s "a" infix keys off `hundreds <= 100`, so it
//!    switches off again at 101.** 1100 → "meli**a**tschient" (remainder 100)
//!    but 1101 → "melitschientadegn" (remainder 101, no infix). Both in the
//!    corpus.
//! 3. **`"dus miliardas"` → `"duas miliardas"` is an unanchored substring
//!    replace.** Unlike the `" egn miliarda "` rule it carries no surrounding
//!    spaces, so it also rewrites a "dus" that is merely the *tail* of a longer
//!    numeral: 402 * 10**9 → "quatertschienta**duas** miliardas" (from
//!    "quatertschientadus miliardas"). Not in the corpus — traced by hand.
//! 4. **`EXPONENT_PREFIXES[0]` is `ZERO` = "nola"**, which would yield
//!    "nolailiùn". Dead: `exponent_length` is always >= 6 (see
//!    [`exponent_length_to_string`]), so the index is always >= 1.
//! 5. **`tens_to_cardinal`'s `else` branch is dead.** `CARDINAL_WORDS[tens][:-1]
//!    + "ànta"` only runs when `tens not in STR_TENS`, but its sole caller is
//!    guarded by `20 <= number < 100`, so `tens` is always 2..=9 — exactly
//!    `STR_TENS`'s key set. Ported anyway, char-safe.
//! 6. `omitt_if_zero` / `empty_if_zero` compare the *rendered string* against
//!    "nola" rather than testing the number, and `"_"` is used as an in-band
//!    "no unit" marker that `phonetic_contraction` strips last. Kept as-is:
//!    the `"veintga_"` → `"veintg"` rule depends on the marker still being
//!    present at that point.
//!
//! Method names `omitt_if_zero` (double "t") and `adapt_milliarda` are Python's
//! spellings, kept so the port greps back to its origin.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};

/// `MINUS_PREFIX_WORD` — note the trailing space (concatenated raw; this class
/// has no `negword`).
const MINUS_PREFIX_WORD: &str = "minus ";

/// `FLOAT_INFIX_WORD` — the separator `float_to_words` puts between the integer
/// prefix and the digit-by-digit fractional postfix. Both surrounding spaces
/// are part of the literal (Python: `prefix + " coma " + postfix`).
const FLOAT_INFIX_WORD: &str = " coma ";

const ZERO: &str = "nola";

const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "egn",
    "dus",
    "tres",
    "quater",
    "tschentg",
    "sis",
    "seat",
    "otg",
    "nov",
    "diesch",
    "endesch",
    "dudesch",
    "tredesch",
    "quitordesch",
    "quendesch",
    "sedesch",
    "gisseat",
    "schotg",
    "schenev",
];

const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "amprem",
    "savund",
    "tearz",
    "quart",
    "tschentgavel",
    "sisavel",
    "seatavel",
    "otgavel",
    "novavel",
    "dieschavel",
    "endeschavel",
    "dudeschavel",
    "tredeschavel",
    "quitordeschavel",
    "quendeschavel",
    "sedeschavel",
    "gisseatavel",
    "schotgavel",
    "schenevavel",
    "veintgavel",
];

/// Python's `STR_TENS` dict, keys 2..=9. "20" is stored as "veintga"; the
/// surface form "veintg" is restored by [`phonetic_contraction`].
fn str_tens(tens: usize) -> Option<&'static str> {
    Some(match tens {
        2 => "veintga",
        3 => "trainta",
        4 => "curànta",
        5 => "tschuncànta",
        6 => "sissànta",
        7 => "satànta",
        8 => "otgànta",
        9 => "novànta",
        _ => return None,
    })
}

/// Prefixes for extremely big numbers. Index 0 is `ZERO` and is unreachable —
/// see module docs (4).
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

// Utils
// =====

/// Port of `phonetic_contraction`. `_` marks "empty", i.e. no following unit.
///
/// Order is load-bearing: `"veintga_"` → `"veintg"` must run *before* the
/// blanket `"_"` → `""`, and the `aegn`/`aotg` contractions must run before
/// both (21 = "veintga"+"egn" → "veintgegn", never "veintgaegn").
fn phonetic_contraction(string: &str) -> String {
    string
        .replace("aegn", "egn") // ex. "traintaegn" -> "traintegn"
        .replace("aotg", "otg") // ex. "curàntaotg" -> "curàntotg"
        .replace("veintga_", "veintg") // ex. "veintga" -> "veintg"
        .replace('_', "")
}

/// Port of `adapt_hundred`: dus/du, tres/tre, and a/ad phonotactic adaptation.
fn adapt_hundred(string: &str) -> String {
    string
        .replace("dustschient", "dutschient")
        .replace("trestschient", "tretschient")
        .replace("aend", "adend")
        .replace("aegn", "adegn")
        .replace("aotg", "adotg")
}

/// Port of `adapt_thousand`: dus/du, tres/tre, and a/ad phonotactic adaptation.
fn adapt_thousand(string: &str) -> String {
    string
        .replace("dusmeli", "dumeli")
        .replace("tresmeli", "tremeli")
        .replace("aend", "adend")
        .replace("aegn", "adegn")
        .replace("aotg", "adotg")
}

/// Port of `adapt_milliarda`: article gender agreement + a/ad adaptation.
///
/// Pads with a space on each side so the anchored `" egn miliarda "` rule can
/// match at string edges. The caller strips the padding. See module docs (1)
/// and (3) for the two quirks this function carries.
fn adapt_milliarda(string: &str) -> String {
    let padded = format!(" {} ", string);
    padded
        .replace(" egn miliarda ", " egna miliarda ")
        .replace("dus miliardas", "duas miliardas")
        .replace(" a end", " ad end")
        .replace(" a egn", " ad egn")
        .replace(" a otg", " ad otg")
}

/// Port of `exponent_length_to_string`.
///
/// `exponent_length` is always a positive multiple of 3: its only caller
/// derives it as `length - (length % 3 or 3)` with `7 <= length <= 65`, giving
/// 6, 9, … 63. So the index is 1..=10 and the `ZERO` slot never fires. The
/// bounds check nevertheless returns `N2WError::Index` (what Python would raise)
/// rather than panicking.
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let prefix = EXPONENT_PREFIXES.get(exponent_length / 6).ok_or_else(|| {
        N2WError::Index(format!(
            "list index out of range: EXPONENT_PREFIXES[{}]",
            exponent_length / 6
        ))
    })?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}iliùn", prefix))
    } else {
        Ok(format!("{}iliarda", prefix))
    }
}

/// Port of `omitt_if_zero` (Python's spelling, double "t").
fn omitt_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        String::new()
    } else {
        number_to_string.to_string()
    }
}

/// Port of `empty_if_zero`. Emits the in-band `_` marker, not an empty string.
fn empty_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        "_".to_string()
    } else {
        number_to_string.to_string()
    }
}

/// Python `s[:-n]`, counting **characters** not bytes (the tables carry à/ù).
fn drop_last(s: &str, n: usize) -> String {
    let total = s.chars().count();
    s.chars().take(total.saturating_sub(n)).collect()
}

/// Parse an all-digits decimal slice, mirroring Python's `int("".join(...))`.
/// Leading zeros are accepted (e.g. `int("000001") == 1`).
fn parse_digits(s: &str) -> BigInt {
    BigInt::parse_bytes(s.as_bytes(), 10).expect("slice of str(BigInt) is all decimal digits")
}

// Main class
// ==========

/// Port of `tens_to_cardinal`. Caller guarantees `20 <= number < 100`.
fn tens_to_cardinal(number: u32) -> String {
    let tens = (number / 10) as usize;
    let units = (number % 10) as usize;
    let prefix = match str_tens(tens) {
        Some(p) => p.to_string(),
        // Dead branch — see module docs (5).
        None => drop_last(CARDINAL_WORDS[tens], 1) + "ànta",
    };
    // We keep track of 0 using '_' -- removed in phonetic_contraction
    let postfix = empty_if_zero(CARDINAL_WORDS[units]);
    phonetic_contraction(&(prefix + &postfix))
}

/// Port of `hundreds_to_cardinal`. Caller guarantees `100 <= number < 1000`.
fn hundreds_to_cardinal(number: u32) -> Result<String> {
    let hundreds = (number / 100) as usize;
    let tens = number % 100;
    let mut prefix = "tschient".to_string();
    if hundreds != 1 {
        prefix = format!("{}{}", CARDINAL_WORDS[hundreds], prefix);
    }
    let postfix = omitt_if_zero(&to_cardinal(&BigInt::from(tens))?);
    // "a/ad" is inserted if tens <= 13 or = 15, 16, 20, 30.
    // The distribution may seem unusual but the Python comment records that it
    // was reviewed by a native speaker. Note 14 is excluded while 15/16 are in.
    let infix = if (tens > 0 && tens <= 13) || matches!(tens, 15 | 16 | 20 | 30) {
        "a"
    } else {
        ""
    };
    Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
}

/// Port of `thousands_to_cardinal`. Caller guarantees `1000 <= number < 10**6`.
fn thousands_to_cardinal(number: u32) -> Result<String> {
    let thousands = number / 1000;
    let hundreds = number % 1000;
    let prefix = if thousands != 1 {
        format!("{}meli", to_cardinal(&BigInt::from(thousands))?)
    } else {
        "meli".to_string()
    };
    let postfix = omitt_if_zero(&to_cardinal(&BigInt::from(hundreds))?);
    // "a/ad" is inserted if the remainder is <= 100 — so it fires for 1100 and
    // stops again at 1101. See module docs (2).
    let infix = if hundreds <= 100 && !postfix.is_empty() {
        "a"
    } else {
        ""
    };
    Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
}

/// Port of `big_number_to_cardinal`. Caller guarantees `number >= 10**6`.
///
/// `number` is positive here (`to_cardinal` peels the sign first), so
/// `to_string()` never carries a '-' or a leading zero.
fn big_number_to_cardinal(number: &BigInt) -> Result<String> {
    let s = number.to_string();
    let digits: Vec<char> = s.chars().collect();
    let length = digits.len();
    check_maxval(number, maxval_ceiling())?;
    // This is how many digits come before the "illion" term.
    //   tschient miliardas => 3
    //   diesch miliùns => 2
    //   egna miliarda => 1
    let predigits = if length % 3 != 0 { length % 3 } else { 3 };
    let multiplier: String = digits[..predigits].iter().collect();
    let exponent: String = digits[predigits..].iter().collect();

    let mut infix = exponent_length_to_string(exponent.chars().count())?;
    // Python compares the digit *list* to ["1"], i.e. predigits == 1 and that
    // digit is '1'. The joined-string compare is equivalent.
    let prefix = if multiplier == "1" {
        "egn ".to_string()
    } else {
        let p = to_cardinal(&parse_digits(&multiplier))?;
        // Plural form
        infix = format!(" {}s", infix);
        p
    };

    // Python reads `set(exponent) != set("0")` — "does the value of exponent
    // equal 0?". `exponent` is never empty (length >= 6), so testing for any
    // non-'0' digit is equivalent.
    let postfix = if exponent.chars().any(|c| c != '0') {
        let p = to_cardinal(&parse_digits(&exponent))?;
        // we introduce "a" if 3-digits gap before next value
        if exponent.starts_with("000") {
            infix.push_str(" a ");
        } else {
            infix.push(' ');
        }
        p
    } else {
        String::new()
    };

    Ok(adapt_milliarda(&format!("{}{}{}", prefix, infix, postfix))
        .trim()
        .to_string())
}

/// Port of `to_cardinal`.
///
/// The sign test comes *first*, before the size dispatch — so -10**65 raises
/// `OverflowError` exactly like +10**65. The `isinstance(number, float)`
/// branch is dead for integer input; the float/Decimal input arrives instead
/// through [`Lang::to_cardinal_float`], which ports `float_to_words`
/// ([`rm_float_f64`]) and the Decimal crash path ([`rm_decimal`]).
fn to_cardinal(number: &BigInt) -> Result<String> {
    if number.is_negative() {
        return Ok(format!("{}{}", MINUS_PREFIX_WORD, to_cardinal(&-number)?));
    }
    // Each branch below 10**6 is bounded by its own guard, so the narrowing to
    // u32 is provably lossless. Above that we stay in BigInt.
    if *number < BigInt::from(20u32) {
        Ok(CARDINAL_WORDS[number.to_usize().expect("< 20")].to_string())
    } else if *number < BigInt::from(100u32) {
        Ok(tens_to_cardinal(number.to_u32().expect("< 100")))
    } else if *number < BigInt::from(1000u32) {
        hundreds_to_cardinal(number.to_u32().expect("< 1000"))
    } else if *number < BigInt::from(1_000_000u32) {
        thousands_to_cardinal(number.to_u32().expect("< 10**6"))
    } else {
        big_number_to_cardinal(number)
    }
}

/// Port of `to_ordinal`.
///
/// The `number % 1 != 0` float branch is dead for integer input. Unlike most
/// languages RM_SUTSILV never calls `verify_ordinal` (it has no base class to
/// inherit one from), so 0 → "nola" and -1 → "minus amprem" rather than
/// raising.
fn to_ordinal(number: &BigInt) -> Result<String> {
    if number.is_negative() {
        return Ok(format!("{}{}", MINUS_PREFIX_WORD, to_ordinal(&-number)?));
    }
    if *number <= BigInt::from(20u32) {
        return Ok(ORDINAL_WORDS[number.to_usize().expect("<= 20")].to_string());
    }
    let cardinal = to_cardinal(number)?;
    // Python `cardinal[-1]` — last *character*. `number > 20` here, so the
    // cardinal is never empty and Python's IndexError is unreachable.
    let suffix = if cardinal.chars().next_back() == Some('a') {
        "vel"
    } else {
        "avel"
    };
    Ok(cardinal + suffix)
}

/// `int(f)` — truncate toward zero, exact for arbitrarily large magnitudes.
///
/// `f.trunc() as i128` would overflow past 2**127; formatting the truncated
/// f64 with zero fractional places and parsing that keeps every digit the
/// double actually holds (`int(1e20) == 100000000000000000000`).
fn f64_trunc_to_bigint(f: f64) -> Result<BigInt> {
    format!("{:.0}", f.trunc())
        .parse::<BigInt>()
        .map_err(|e| N2WError::Value(e.to_string()))
}

/// Whether Python's `str(f)` — the shortest round-trip repr — contains a `.`.
///
/// repr picks exponent form (no point) for finite non-zero magnitudes below
/// `1e-4` ("5e-05") or at/above `1e16` ("1e+16", "1e+20"); every other finite
/// float prints with a point ("0.0", "-0.0", "21.0", "2.675").
/// `float_to_words` does `str(float_number).split('.')[1]`, so a pointless
/// repr is Python's `IndexError` — corpus-confirmed for `1e+16` and `1e+20`.
/// This must be an explicit repr test: the binding's `precision` for `1e+16`
/// is 16 (`abs(Decimal("1e+16").as_tuple().exponent)`), so the `{:.16}`
/// reconstruction *would* carry a point Python's repr does not have.
fn float_repr_has_point(f: f64) -> bool {
    f.is_finite() && (f == 0.0 || (f.abs() >= 1e-4 && f.abs() < 1e16))
}

/// Port of `float_to_words` for `FloatValue::Float`, the cardinal float path.
///
/// `Num2Word_RM_SUTSILV.to_cardinal(float)` peels the sign (`"minus " +
/// to_cardinal(-number)`) then calls `float_to_words`, which reads the *string*
/// repr — `prefix = to_cardinal(int(number))`, `float_part =
/// str(number).split('.')[1]`, then one cardinal digit word per fractional
/// character joined by spaces, glued with `" coma "`. Because it keys on
/// `str(number)` rather than `base.float2tuple`, the f64 artefacts that
/// `floatpath.rs` reproduces never appear: `str(2.675)` is the literal
/// "2.675", so the fractional part is "675" with no `674.9999…`/`< 0.01`
/// rescue involved. `format!("{:.p$}", f)` reconstructs that repr exactly —
/// Python already supplied `precision` as the repr's fractional-digit count,
/// and formatting an f64 to that many places recovers the shortest
/// round-trip digits (verified on 0.5/2.675/1.005/0.01/99.99/1234.56).
///
/// `precision_override` is deliberately not honoured: `Num2Word_RM_SUTSILV`
/// defines no `precision` attribute, so the dispatcher's
/// `hasattr(converter, "precision")` guard is False and `precision=` is popped
/// but never applied. See the port report.
fn rm_float_f64(value: f64, precision: u32) -> Result<String> {
    // Python: `if number < 0: return MINUS_PREFIX_WORD + to_cardinal(-number)`.
    // `-0.0 < 0` is False in both languages, so a signed zero falls through to
    // float_to_words rather than gaining a "minus".
    if value < 0.0 {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            rm_float_f64(-value, precision)?
        ));
    }

    // prefix = to_cardinal(int(number)) — int() truncates toward zero.
    // Computed *before* the point probe, exactly as Python does.
    let prefix = to_cardinal(&f64_trunc_to_bigint(value)?)?;

    // float_part = str(number).split('.')[1]: a repr in exponent form
    // ("1e+16", "1e+20") has no '.' — Python's IndexError, corpus-confirmed.
    // The `{:.p$}` reconstruction below cannot detect this (`precision` for
    // `1e+16` is 16, which *would* print a point), hence the explicit test.
    if !float_repr_has_point(value) {
        return Err(N2WError::Index("list index out of range".into()));
    }

    // float_part = str(number).split('.')[1].
    let s = format!("{:.*}", precision as usize, value);
    let right = match s.split_once('.') {
        Some((_, r)) => r,
        None => {
            // No '.': Python's `str(number)` had none either — only floats whose
            // repr is scientific ("1e+16", "1e-05"). There `str(number)
            // .split('.')[1]` raises IndexError. `precision == 0` is the sole
            // way to reach this from an f64 (a scientific repr still yields a
            // '.' after `{:.p$}` formatting — see the port report). Reproduce
            // the IndexError.
            return Err(N2WError::Index("list index out of range".into()));
        }
    };

    // postfix = " ".join([to_cardinal(int(c)) for c in float_part]).
    let mut parts: Vec<String> = Vec::with_capacity(right.chars().count());
    for c in right.chars() {
        // Every char is a decimal digit produced by `{:.p$}`, so `int(c)` never
        // raises; the branch mirrors Python's `int(c)` regardless.
        let d = c.to_digit(10).ok_or_else(|| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
        })?;
        parts.push(to_cardinal(&BigInt::from(d))?);
    }

    Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, parts.join(" ")))
}

/// Port of `to_cardinal` for `FloatValue::Decimal`.
///
/// `Num2Word_RM_SUTSILV` has no `to_cardinal_float`; a `Decimal` simply flows
/// through the integer `to_cardinal`, where it (almost always) crashes because
/// the number never becomes an `int`:
///
/// * `number < 0` → `"minus " + to_cardinal(-number)`; the inner call raises,
///   so the sign is handled here purely by propagating that error.
/// * `abs(number) < 1_000_000` → every branch eventually does
///   `CARDINAL_WORDS[<Decimal>]`, and a `Decimal` is not a valid list index →
///   **TypeError** (`list indices must be integers … not decimal.Decimal`).
///   Corpus-confirmed on 0.01, 1.10, 12.345, 0.001.
/// * `abs(number) >= 1_000_000` → `big_number_to_cardinal`, which splits
///   `str(number)`:
///   * `precision == 0` (exponent 0, i.e. `str` is all digits) → the `int(...)`
///     recursions succeed and it renders exactly like the integer cardinal.
///     So `Decimal("2000000")` → "dus miliùns", but `Decimal("2000000.00")`
///     (precision 2, `str` keeps the dot) → ValueError. Confirmed on the live
///     interpreter.
///   * `precision > 0` → `str(number)` carries a non-digit ('.' or 'E'), so an
///     `int()` on a fragment raises **ValueError** — unless the magnitude is
///     `>= 10**65` (OverflowError, #159) or `len(str(number)) >= 66` (still
///     NotImplementedError) first. Corpus-confirmed
///     on 98746251323029.99 (ValueError).
fn rm_decimal(value: &BigDecimal, precision: u32) -> Result<String> {
    // `number < 0` → "minus " + to_cardinal(-number). The recursion raises for
    // every in-range magnitude, so the prefix is only ever prepended to a
    // successful big all-digit Decimal (e.g. Decimal("-2000000")).
    if value.is_negative() {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            rm_decimal(&(-value.clone()), precision)?
        ));
    }

    // abs(number) < 1_000_000: list-indexed by a Decimal somewhere → TypeError.
    if *value < BigDecimal::from(1_000_000) {
        return Err(N2WError::Type(
            "list indices must be integers or slices, not decimal.Decimal".into(),
        ));
    }

    // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
    // checked before Python's `len(str(number)) >= 66` string guard.
    check_maxval(&value.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
    // abs(number) >= 1_000_000 → big_number_to_cardinal(str(number)).
    if precision == 0 {
        // str(number) is all digits: identical to the integer cardinal (and its
        // own maxval guard lives in `big_number_to_cardinal`).
        let bigint = value.with_scale(0).as_bigint_and_exponent().0;
        return to_cardinal(&bigint);
    }

    // precision > 0: str(number) has a '.'/'E', so int() on a fragment raises
    // ValueError — after the maxval check above and the `len(str(number)) >= 66`
    // NotImplementedError check below.
    let int_digits = value
        .with_scale(0)
        .as_bigint_and_exponent()
        .0
        .to_string()
        .len();
    let length = int_digits + 1 + precision as usize; // int part + '.' + fraction
    // Only reachable below maxval when the fraction digits push str(number)
    // past 65 characters; magnitudes past maxval were rejected above (#159).
    if length >= 66 {
        return Err(N2WError::NotImplemented(
            "The given number is too large.".into(),
        ));
    }
    Err(N2WError::Value(
        "invalid literal for int() with base 10".into(),
    ))
}

/// `Num2Word_RM_SUTSILV.to_ordinal` for a `float` argument:
///
/// ```python
/// if number < 0:  return MINUS_PREFIX_WORD + self.to_ordinal(-number)
/// elif number % 1 != 0:  return self.float_to_words(number, ordinal=True)
/// elif number <= 20:  return ORDINAL_WORDS[number]   # float index -> TypeError
/// else: cardinal = self.to_cardinal(number)  # float branch, "... coma nola"
/// ```
///
/// Corpus-confirmed quirks, all reproduced:
///   * a *fractional* float works: `2.5` -> "savund coma tschentg"
///     (ordinal prefix over `int(2.5)`, cardinal digit words after);
///   * a *whole* float `<= 20` (incl. `-0.0`/`0.0`: `-0.0 < 0` is False,
///     `-0.0 % 1 == 0`) dies on `ORDINAL_WORDS[<float>]` -> TypeError;
///   * a whole float `> 20` renders its float *cardinal* ("veintgegn coma
///     nola") and then gets the ordinal suffix: "veintgegn coma nolavel";
///   * `1e+16`/`1e+20` (whole, > 20) reach the cardinal float branch and
///     die on the pointless repr -> IndexError.
fn rm_float_ordinal_f64(f: f64, precision: u32) -> Result<String> {
    if f < 0.0 {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            rm_float_ordinal_f64(-f, precision)?
        ));
    }
    if f.fract() != 0.0 {
        // float_to_words(number, ordinal=True): the prefix is the ordinal of
        // int(number); the digit words stay *cardinal*.
        let prefix = to_ordinal(&f64_trunc_to_bigint(f)?)?;
        if !float_repr_has_point(f) {
            return Err(N2WError::Index("list index out of range".into()));
        }
        let s = format!("{:.*}", precision as usize, f);
        let frac = s.split_once('.').map(|(_, x)| x).unwrap_or("");
        let mut parts: Vec<String> = Vec::new();
        for c in frac.chars() {
            let d = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
            })?;
            parts.push(to_cardinal(&BigInt::from(d))?);
        }
        return Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, parts.join(" ")));
    }
    if f <= 20.0 {
        // ORDINAL_WORDS[<float>] — a float is not a valid list index.
        return Err(N2WError::Type(
            "list indices must be integers or slices, not float".into(),
        ));
    }
    // cardinal = self.to_cardinal(number) — the float branch again. Ends in
    // "... coma nola" here, so the 'a' rule always fires; the "avel" arm is
    // kept for shape parity with the integer path.
    let cardinal = rm_float_f64(f, precision)?;
    let suffix = if cardinal.ends_with('a') { "vel" } else { "avel" };
    Ok(cardinal + suffix)
}

/// `Num2Word_RM_SUTSILV.to_ordinal` for a `Decimal` argument. `Decimal % 1`
/// works, so a *fractional* Decimal takes the `float_to_words(ordinal=True)`
/// branch and renders (reading `str(Decimal)`); a whole-valued one falls into
/// the integer branches: `<= 20` dies on `ORDINAL_WORDS[<Decimal>]`
/// (TypeError), `> 20` re-enters `to_cardinal(Decimal)` — TypeError below
/// `10**6`, str-splitting above (ValueError when the str carries a non-digit).
/// `Decimal("-0.0") < 0` is False, so it is *not* minus-prefixed — it crashes
/// in the table branch like `0.0` does.
fn rm_decimal_ordinal(value: &BigDecimal, precision: u32) -> Result<String> {
    if value.is_negative() {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            rm_decimal_ordinal(&(-value.clone()), precision)?
        ));
    }
    if !value.is_integer() {
        // float_to_words(number, ordinal=True) over str(Decimal).
        let pre = value.with_scale(0).as_bigint_and_exponent().0;
        let prefix = to_ordinal(&pre)?;
        let s = crate::strnum::python_decimal_str(value);
        let frac = match s.split_once('.') {
            Some((_, f)) => f.to_string(),
            // Scientific repr with no '.' (e.g. Decimal("5E-7")).
            None => return Err(N2WError::Index("list index out of range".into())),
        };
        let mut parts: Vec<String> = Vec::new();
        for c in frac.chars() {
            // int(c) — an 'E'/'+' from a scientific repr is ValueError.
            let d = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
            })?;
            parts.push(to_cardinal(&BigInt::from(d))?);
        }
        return Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, parts.join(" ")));
    }
    if *value <= BigDecimal::from(20) {
        return Err(N2WError::Type(
            "list indices must be integers or slices, not decimal.Decimal".into(),
        ));
    }
    let cardinal = rm_decimal(value, precision)?;
    let suffix = if cardinal.ends_with('a') { "vel" } else { "avel" };
    Ok(cardinal + suffix)
}

pub struct LangRmSutsilv;

impl LangRmSutsilv {
    pub fn new() -> Self {
        LangRmSutsilv
    }
}

impl Default for LangRmSutsilv {
    fn default() -> Self {
        LangRmSutsilv::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRmSutsilv {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    // The class defines no to_currency (Python raised AttributeError on the
    // lookup) and there is no Romansh currency wording to port: a clear
    // NotImplementedError instead (gladiaio/num2words2#157).
    fn to_currency(
        &self,
        _val: &crate::currency::CurrencyValue,
        _currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sutsilv", "to='currency'"))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sutsilv", "to='cheque'"))
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        to_cardinal(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        to_ordinal(value)
    }

    /// Python's class had no `to_ordinal_num` (AttributeError for every
    /// input), and no ordinal-numeral marker is attested for Romansh in the
    /// module or its tests, so the port raises a clear NotImplementedError
    /// rather than inventing one (#157).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sutsilv", "to='ordinal_num'"))
    }

    /// Python's class had no `to_year` (AttributeError for every input).
    /// `Num2Word_Base.to_year` is the cardinal, which is what the port
    /// returns (#157).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The cardinal float/Decimal path. `Num2Word_RM_SUTSILV` has no
    /// `to_cardinal_float`; both arms reproduce what its integer `to_cardinal`
    /// does when handed a `float` (→ `float_to_words`, see [`rm_float_f64`]) or
    /// a `Decimal` (crashed through the int dispatch in Python; now read like
    /// the int or the float, #157).
    ///
    /// `precision_override` is ignored on purpose: the class defines no
    /// `precision` attribute, so the dispatcher never applies `precision=`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // A Decimal crashed in the integer ladder in Python; it now reads like
        // the int (whole) or the float (fractional, exact digits) (#157).
        if let FloatValue::Decimal { .. } = value {
            return crate::lang_rm::decimal_cardinal(self, value, MINUS_PREFIX_WORD, FLOAT_INFIX_WORD);
        }
        match value {
            FloatValue::Float { value, precision } => rm_float_f64(*value, *precision),
            FloatValue::Decimal { value, precision } => rm_decimal(value, *precision),
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing. The gate is
    /// `isinstance(number, float)`, **not** `int(number) == number`, so a
    /// whole-valued float still renders through `float_to_words`
    /// (`1.0` -> "egn coma nola") and a whole-valued Decimal crashed
    /// through the integer ladder (`Decimal("5.0")` -> TypeError) in Python —
    /// now it reads like the int (#157). The base
    /// default's whole-value -> int-path shortcut is exactly wrong here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // A Decimal crashed in the integer ladder in Python; it now reads like
        // the int (whole) or the float (fractional, exact digits) (#157).
        if let FloatValue::Decimal { .. } = value {
            return crate::lang_rm::decimal_cardinal(self, value, MINUS_PREFIX_WORD, FLOAT_INFIX_WORD);
        }
        match value {
            FloatValue::Float { value, precision } => rm_float_f64(*value, *precision),
            FloatValue::Decimal { value, precision } => rm_decimal(value, *precision),
        }
    }

    /// `to_ordinal(float/Decimal)` — see [`rm_float_ordinal_f64`] /
    /// [`rm_decimal_ordinal`] for the branch-by-branch mapping.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // A whole Decimal crashed on `ORDINAL_WORDS[<Decimal>]` in Python; it
        // now reads like the int (#157).
        if let FloatValue::Decimal { value: d, .. } = value {
            if d.is_integer() {
                return self.to_ordinal(&d.with_scale(0).as_bigint_and_exponent().0);
            }
        }
        match value {
            FloatValue::Float { value, precision } => rm_float_ordinal_f64(*value, *precision),
            FloatValue::Decimal { value, precision } => rm_decimal_ordinal(value, *precision),
        }
    }

    /// See [`Lang::to_ordinal_num`] above (#157).
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sutsilv", "to='ordinal_num'"))
    }

    /// `to_year(float/Decimal)`: a whole value is the integer year; a
    /// fractional one is a `TypeError`, as in `lang_en` (#157).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_year(&i),
            None => Err(crate::base::year_float_error(value)),
        }
    }


    // `str_to_number`: the trait default, the shared `Decimal(value)` parse.
    // Python's class had none, so every string raised AttributeError (#157).

    /// `Decimal('-0.0')` is a whole Decimal, so it reads like the int zero
    /// (#157; Python crashed with TypeError in the integer ladder). Served
    /// here because the `Float{-0.0}` demotion would take the float reading.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            "cardinal" | "year" => Some(self.to_cardinal(&BigInt::from(0))),
            "ordinal" => Some(self.to_ordinal(&BigInt::from(0))),
            _ => None,
        }
    }

    /// Python's class had no `to_fraction` (AttributeError for every n/d).
    /// Base's generic reading ("<n> <ordinal>s") is not Romansh, so this is a
    /// clear NotImplementedError instead (#157).
    fn to_fraction(&self, _numerator: &BigInt, _denominator: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sutsilv", "fractions"))
    }
}
