//! Port of `lang_RM_SURSILV.py` (Romansh — Sursilvan idiom).
//!
//! Shape: **self-contained**. `Num2Word_RM_SURSILV` is a *bare* class — it
//! subclasses nothing at all (not even `Num2Word_Base`), and its `__init__` is
//! a single `pass`. There is no inheritance chain to chase: everything the
//! class can do is the six methods in the file. Consequently `cards` and
//! `merge` stay at their trait defaults and are never reached; `maxval()`
//! reports the ceiling below.
//!
//! The module header says "Based on lang_IT template from Filippo Costa", and
//! the skeleton matches `lang_IT`/`lang_LIJ`: `MINUS_PREFIX_WORD`, a size
//! dispatch in `to_cardinal`, and per-magnitude helpers. The phonology is
//! entirely Sursilvan, driven by string `.replace()` passes rather than by
//! table lookups.
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
//! too large.")` in Python when `len(str(number)) >= 66`, so the wall is at **10**65**
//! exactly (10**65 - 1, a 65-digit number, is the largest convertible value).
//! `to_cardinal` tests the sign *before* the size dispatch, so `-10**65`
//! raises too. Python raises `NotImplementedError`; fixed
//! (gladiaio/num2words2#159): the port raises `OverflowError` ("abs(v) must
//! be less than 10^65.").
//!
//! That bound also makes `EXPONENT_PREFIXES` provably safe: `len(exponent)`
//! peaks at 63 (a 65-digit number gives `predigits = 65 % 3 = 2`), and
//! `63 // 6 == 10` is exactly the last index of the 11-entry table. The
//! `IndexError` arm in [`exponent_length_to_string`] is therefore unreachable;
//! it is modelled rather than panicked on, for fidelity.
//!
//! # Python semantics that matter here
//!
//! * `predigits = length % 3 or 3` — Python's `or` returns the *right* operand
//!   when the left is falsy, so a length divisible by 3 yields 3, not 0.
//! * `set(exponent) != set("0")` asks "is the exponent something other than
//!   all zeros?". `exponent` is never empty here (`number >= 10**6` forces
//!   `len(exponent) >= 4`), so it reduces to "any char is not '0'".
//! * `multiplier == ["1"]` compares a *list of one-character strings*, so it
//!   is true only when `predigits == 1` and that digit is `1`.
//! * `str.replace` replaces **every** non-overlapping occurrence, left to
//!   right, and the ordering of the chained calls is load-bearing (see the
//!   `adapt_hundred` note below). Rust's `str::replace` has identical
//!   semantics.
//! * All strings in this language are pure ASCII, but slicing is still done
//!   by `chars()` here so the port cannot rot if a diacritic is ever added.
//!
//! # Faithfully reproduced oddities
//!
//! None of these are errors to be fixed — all are corpus-confirmed:
//!
//! 1. **The `"_"` sentinel.** `empty_if_zero` maps a zero unit to the literal
//!    `"_"`, which is then either rewritten (`"ventga_"` → `"vegn"`, i.e. 20)
//!    or deleted by [`phonetic_contraction`]. The marker exists purely so that
//!    "20" can be spelled `vegn` while "21" is built from the `ventga` stem
//!    (`ventga` + `in` → `ventgain` → `ventgin`). This means `to_cardinal(20)`
//!    == "vegn" but `to_cardinal(120)` == "tschienevegn" — the contraction has
//!    already happened in the recursive call.
//! 2. **`adapt_hundred`/`adapt_thousand` run over the *whole* string, not just
//!    the prefix.** So `to_cardinal(123456)` ==
//!    "tschienventgatreimelliquatertschientschuncontasis": the `treismelli` →
//!    `treimelli` rule fires on a boundary formed between the recursively-built
//!    "tschienventgatreis" and the literal "melli". This is the behaviour the
//!    corpus records, and it is *not* what `lang_LIJ` does (LIJ scopes its
//!    equivalent replaces to the prefix only).
//! 3. **`"eend"` must be replaced before `"ein"`.** Both rules are live and
//!    the chain order in the Python source is `eend`, then `ein`, then `eotg`.
//!    `to_cardinal(111)` == "tschienedendisch" depends on it.
//! 4. **`adapt_milliarda` pads with spaces then the caller strips.** The
//!    rules are written as `" in milliarda "` etc. — whitespace-anchored — so
//!    the function brackets the string with spaces to let the first and last
//!    words match, and `big_number_to_cardinal` calls `.strip()` on the
//!    result. Hence `to_cardinal(10**9)` == "ina milliarda" (article gender
//!    agreement) while `to_cardinal(10**15)` == "in billiarda" is untouched —
//!    the rule keys on "milliarda" literally, so *billiarda*, *trilliarda* and
//!    friends never get the feminine article.
//! 5. **`tens_to_cardinal`'s `else` arm is dead.** `STR_TENS` covers keys
//!    2..=9 and the only caller guarantees `20 <= number < 100`, so
//!    `CARDINAL_WORDS[tens][:-1] + "onta"` can never run. Ported anyway.
//! 6. **`to_ordinal` of a negative is not a real ordinal**: it prefixes
//!    "minus " and recurses, so `to_ordinal(-1)` == "minus emprem" rather than
//!    raising. This class has no `verify_ordinal` at all, and `to_ordinal(0)`
//!    happily returns "nulla".

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};

/// `MINUS_PREFIX_WORD` — note the trailing space (concatenated raw; this class
/// has no `negword` because `Num2Word_Base.setup` never runs).
const MINUS_PREFIX_WORD: &str = "minus ";

/// `FLOAT_INFIX_WORD` — the separator between integer and fractional words in
/// `float_to_words`. Note the surrounding spaces (concatenated raw as
/// `prefix + FLOAT_INFIX_WORD + postfix`).
const FLOAT_INFIX_WORD: &str = " comma ";

const ZERO: &str = "nulla";

const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "in",
    "dus",
    "treis",
    "quater",
    "tschun",
    "sis",
    "siat",
    "otg",
    "nov",
    "diesch",
    "endisch",
    "dudisch",
    "tredisch",
    "quitordisch",
    "quendisch",
    "sedisch",
    "gissiat",
    "schotg",
    "scheniv",
];

/// 21 entries — index 20 ("vegnavel") is live, since `to_ordinal` tests
/// `number <= 20` inclusively.
const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "emprem",
    "secund",
    "tierz",
    "quart",
    "tschunavel",
    "sisavel",
    "siatavel",
    "otgavel",
    "novavel",
    "dieschavel",
    "endischavel",
    "dudischavel",
    "tredischavel",
    "quitordischavel",
    "quendischavel",
    "sedischavel",
    "gissiatavel",
    "schotgavel",
    "schenivavel",
    "vegnavel",
];

/// `STR_TENS` — a dict with keys 2..=9 in Python. Returns `None` for 0/1,
/// mirroring the missing keys (the `else` arm they select is dead; see module
/// docs (5)).
///
/// The Python comment reads: `"20" = "vegn"` / surface form is restaured
/// during phonetic adaptation phase — i.e. `ventga` is the underlying stem and
/// [`phonetic_contraction`] rewrites the bare form.
fn str_tens(tens: u32) -> Option<&'static str> {
    match tens {
        2 => Some("ventga"),
        3 => Some("trenta"),
        4 => Some("curonta"),
        5 => Some("tschunconta"),
        6 => Some("sissonta"),
        7 => Some("siatonta"),
        8 => Some("otgonta"),
        9 => Some("navonta"),
        _ => None,
    }
}

/// `EXPONENT_PREFIXES` — 11 entries, indexed by `len(exponent) // 6`.
///
/// Index 0 ("nulla") is unreachable. `predigits` is congruent to `length`
/// mod 3 (it is `length % 3`, or 3 when that is 0), so `len(exponent) =
/// length - predigits` is always a multiple of 3; and `big_number_to_cardinal`
/// only runs for `number >= 10**6`, i.e. `length >= 7`, which floors
/// `len(exponent)` at 6. So the live index range is 1..=10 — "m" through
/// "dec". See [`exponent_length_to_string`].
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

/// Python `s[:-n]`, counting **characters** not bytes. Python clamps rather
/// than panicking when `n > len(s)`.
fn drop_last(s: &str, n: usize) -> String {
    let total = s.chars().count();
    s.chars().take(total.saturating_sub(n)).collect()
}

// Utils
// =====

/// `_` is a marker for "empty", i.e. no following unit.
///
/// Order is load-bearing and matches the Python chain exactly.
fn phonetic_contraction(string: &str) -> String {
    string
        .replace("ain", "in") // ex. "trentain" -> "trentin"
        .replace("aotg", "otg") // ex. "curontaotg" -> "curantotg"
        .replace("ventga_", "vegn") // ex. "ventga" -> "vegn"
        .replace('_', "")
}

/// Apply surface modifications: collective plural, e/ed phonotactic
/// adaptation. Runs over the whole string — see module docs (2) and (3).
fn adapt_hundred(string: &str) -> String {
    string
        .replace("dustschien", "duatschien")
        .replace("treistschien", "treitschien")
        .replace("eend", "edend")
        .replace("ein", "edin")
        .replace("eotg", "edotg")
}

/// Apply surface modifications: collective plural, e/ed phonotactic
/// adaptation.
fn adapt_thousand(string: &str) -> String {
    string
        .replace("dusmelli", "duamelli")
        .replace("treismelli", "treimelli")
        .replace("eend", "edend")
        .replace("ein", "edin")
        .replace("eotg", "edotg")
}

/// Apply surface modifications: article gender agreement, e/ed phonotactic
/// adaptation.
///
/// Brackets the string with spaces so the whitespace-anchored rules can match
/// at the edges; `big_number_to_cardinal` strips the padding afterwards.
fn adapt_milliarda(string: &str) -> String {
    let string = format!(" {} ", string);
    string
        .replace(" in milliarda ", " ina milliarda ")
        .replace("dus milliardas", "duas milliardas")
        .replace(" e in", " ed in")
        .replace(" e otg", " ed otg")
}

/// We always assume `exponent` to be a multiple of 3. If that's not true, then
/// [`big_number_to_cardinal`] did something wrong.
///
/// The `Index` arm is unreachable given the `length >= 66` guard (see module
/// docs, "Ceiling") but is modelled rather than panicked on, for fidelity:
/// Python would raise `IndexError: list index out of range`.
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let prefix = EXPONENT_PREFIXES
        .get(exponent_length / 6)
        .ok_or_else(|| N2WError::Index("list index out of range".to_string()))?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}illiun", prefix))
    } else {
        Ok(format!("{}illiarda", prefix))
    }
}

/// `"" if number_to_string == ZERO else number_to_string`
fn omitt_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        String::new()
    } else {
        number_to_string.to_string()
    }
}

/// `"_" if number_to_string == ZERO else number_to_string` — the sentinel that
/// [`phonetic_contraction`] later consumes. See module docs (1).
fn empty_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        "_".to_string()
    } else {
        number_to_string.to_string()
    }
}

// Main class
// ==========

/// Port of `tens_to_cardinal`. Callers guarantee `20 <= number < 100`.
fn tens_to_cardinal(number: u32) -> String {
    debug_assert!((20..100).contains(&number));
    let tens = number / 10;
    let units = number % 10;
    let prefix = match str_tens(tens) {
        Some(w) => w.to_string(),
        // Dead: tens is 2..=9 here and STR_TENS covers all of those. See
        // module docs (5).
        None => drop_last(CARDINAL_WORDS[tens as usize], 1) + "onta",
    };
    // we keep track of 0 using '_' -- removed in phonetic_contraction
    let postfix = empty_if_zero(CARDINAL_WORDS[units as usize]);
    phonetic_contraction(&(prefix + &postfix))
}

/// Port of `hundreds_to_cardinal`. Callers guarantee `100 <= number < 1000`.
fn hundreds_to_cardinal(number: u32) -> Result<String> {
    debug_assert!((100..1000).contains(&number));
    let hundreds = number / 100;
    let tens = number % 100;
    let prefix = if hundreds != 1 {
        CARDINAL_WORDS[hundreds as usize].to_string() + "tschien"
    } else {
        "tschien".to_string()
    };
    let postfix = omitt_if_zero(&to_cardinal_impl(&BigInt::from(tens))?);
    // "e/ed" is inserted if tens <= 13 or = 15, 16, 20, 30
    // distribution may seem unusual but it was reviewed by a native speaker
    let infix = if (tens > 0 && tens <= 13) || matches!(tens, 15 | 16 | 20 | 30) {
        "e"
    } else {
        ""
    };
    Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
}

/// Port of `thousands_to_cardinal`. Callers guarantee `1000 <= number < 10**6`.
fn thousands_to_cardinal(number: u32) -> Result<String> {
    debug_assert!((1000..1_000_000).contains(&number));
    let thousands = number / 1000;
    let hundreds = number % 1000;
    let prefix = if thousands != 1 {
        to_cardinal_impl(&BigInt::from(thousands))? + "melli"
    } else {
        "melli".to_string()
    };
    let postfix = omitt_if_zero(&to_cardinal_impl(&BigInt::from(hundreds))?);
    // "e/ed" is inserted if tens <= 100
    let infix = if hundreds <= 100 && !postfix.is_empty() {
        "e"
    } else {
        ""
    };
    Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
}

/// Port of `big_number_to_cardinal`. Callers guarantee `number >= 10**6`, and
/// `to_cardinal` has already stripped any sign, so `to_string()` never carries
/// a `-`.
///
/// `number` is unbounded up to 10**65, so it stays a `BigInt` and is split via
/// its decimal string exactly as Python does with `str(number)`.
fn big_number_to_cardinal(number: &BigInt) -> Result<String> {
    debug_assert!(!number.is_negative());
    let digits: Vec<char> = number.to_string().chars().collect();
    let length = digits.len();
    check_maxval(number, maxval_ceiling())?;
    // This is how many digits come before the "illion" term.
    //   tschien milliardas => 3
    //   diesch milliuns => 2
    //   ina milliarda => 1
    // Python's `length % 3 or 3`: a multiple of 3 yields 3, not 0.
    let predigits = if length % 3 != 0 { length % 3 } else { 3 };
    let multiplier: String = digits[..predigits].iter().collect();
    let exponent: String = digits[predigits..].iter().collect();
    let mut infix = exponent_length_to_string(exponent.len())?;

    // Python compares the *list* `multiplier` against `["1"]`, which is true
    // only when predigits == 1 and that digit is '1'.
    let prefix = if multiplier == "1" {
        "in ".to_string()
    } else {
        // At most 3 digits, so the u32 parse is provably in range.
        let m: u32 = multiplier.parse().expect("<= 3 decimal digits");
        let p = to_cardinal_impl(&BigInt::from(m))?;
        // Plural form
        infix = format!(" {}s", infix);
        p
    };

    // Read as: Does the value of exponent equal 0?
    // `exponent` is never empty here (length >= 7 and predigits <= 3), so
    // `set(exponent) != set("0")` reduces to "any digit is not '0'".
    let postfix = if exponent.chars().any(|c| c != '0') {
        let p = to_cardinal_impl(&exponent.parse::<BigInt>().expect("decimal digits"))?;
        // we introduce "e" if 3-digits gap before next value
        if exponent.starts_with("000") {
            infix.push_str(" e ");
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
/// The `isinstance(number, float)` branch is dead for integer input and is
/// omitted along with `float_to_words`. Note the sign test comes *first*, so
/// `-10**65` raises `OverflowError` just like `+10**65`.
fn to_cardinal_impl(number: &BigInt) -> Result<String> {
    if number.is_negative() {
        return Ok(MINUS_PREFIX_WORD.to_string() + &to_cardinal_impl(&(-number))?);
    }
    // Each branch below 10**6 is bounded by its own guard, so the narrowing to
    // u32/usize is provably lossless. Above that we stay in BigInt.
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
/// The `number % 1 != 0` float branch is dead for integer input. This class
/// has no `verify_ordinal`, so 0 and negatives are accepted (see module docs
/// (6)).
///
/// `cardinal[-1]` would raise `IndexError` on an empty string, but every value
/// reaching that line is `> 20` and so has a non-empty cardinal.
fn to_ordinal_impl(number: &BigInt) -> Result<String> {
    if number.is_negative() {
        return Ok(MINUS_PREFIX_WORD.to_string() + &to_ordinal_impl(&(-number))?);
    }
    if *number <= BigInt::from(20u32) {
        return Ok(ORDINAL_WORDS[number.to_usize().expect("<= 20")].to_string());
    }
    let cardinal = to_cardinal_impl(number)?;
    // Python `cardinal[-1] == 'a'` — a *character* test, not a byte test.
    let suffix = if cardinal.ends_with('a') {
        "vel"
    } else {
        "avel"
    };
    Ok(cardinal + suffix)
}

// Float / Decimal cardinal path
// =============================

/// Whether Python's `str(f)` — the shortest round-trip repr — contains a `.`.
///
/// repr picks exponent form (no point) for finite non-zero magnitudes below
/// `1e-4` ("5e-05") or at/above `1e16` ("1e+16", "1e+20"); every other finite
/// float prints with a point ("0.0", "-0.0", "21.0", "2.675").
/// `float_to_words` does `str(float_number).split('.')[1]`, so a pointless
/// repr is Python's `IndexError: list index out of range`.
fn float_repr_has_point(f: f64) -> bool {
    f.is_finite() && (f == 0.0 || (f.abs() >= 1e-4 && f.abs() < 1e16))
}

/// Python's `int(f)` — truncate toward zero, exact at any magnitude (every
/// whole f64 is exactly representable as a BigInt). `int(inf)` is Python's
/// OverflowError, `int(nan)` its ValueError; neither is reachable from the
/// corpus but both are modelled.
fn f64_trunc_to_bigint(f: f64) -> Result<BigInt> {
    if f.is_nan() {
        return Err(N2WError::Value(
            "cannot convert float NaN to integer".into(),
        ));
    }
    num_traits::FromPrimitive::from_f64(f.trunc())
        .ok_or_else(|| N2WError::Overflow("cannot convert float infinity to integer".into()))
}

/// `Num2Word_RM_SURSILV.float_to_words` (cardinal arm, `ordinal=False`) applied
/// to a **non-negative** f64 — the caller strips the sign.
///
/// ```python
/// def float_to_words(self, float_number, ordinal=False):
///     prefix = self.to_cardinal(int(float_number))
///     float_part = str(float_number).split('.')[1]
///     postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
///     return prefix + Num2Word_RM_SURSILV.FLOAT_INFIX_WORD + postfix
/// ```
///
/// Unlike `lang_IT` (whose `to_cardinal` gates on `int(number) != number`),
/// this class gates the float branch on `isinstance(number, float)`, so an
/// **integer-valued** float still enters `float_to_words`: `1.0` →
/// "in comma nulla", `0.0` → "nulla comma nulla". There is no `int(number)`
/// short-circuit here.
///
/// This never touches `base.float2tuple`: the fractional digits come straight
/// from `str(float_number)`, one character at a time, so `2.675` renders
/// "sis siat tschun" (6 7 5) because `str(2.675)` is literally `"2.675"` — no
/// `< 0.01` artefact heuristic, no banker's rounding.
///
/// `precision` is the repr-derived fractional-digit count. In the normal range
/// it equals `len(str(float_number).split('.')[1])`, so a fixed format to that
/// many places reproduces the exact repr digits (verified byte-for-byte against
/// the interpreter over the corpus float set; `{:.N}` rounds the exact binary
/// value half-to-even, agreeing with CPython where a shortest-repr
/// reconstruction would not). Exponent-notation reprs (`|x|` very large/small)
/// are out of corpus scope — see the port report.
fn float_body_f64(x: f64, precision: u32) -> Result<String> {
    // prefix = to_cardinal(int(float_number)); `int()` truncates toward zero.
    // Computed *before* the point probe, exactly as Python does — for a huge
    // float the maxval OverflowError (#159) beats the IndexError below.
    let pre = f64_trunc_to_bigint(x)?;
    let prefix = to_cardinal_impl(&pre)?;

    // Python: `str(float_number).split('.')[1]` with no '.' → IndexError.
    // This must be an explicit repr test: the binding's `precision` for
    // `1e+16` is 16 (`abs(Decimal("1e+16").as_tuple().exponent)`), so the
    // `{:.16}` reconstruction *would* carry a point Python's repr does not
    // have. Corpus-confirmed IndexError for `1e+16` and `1e+20`.
    if !float_repr_has_point(x) {
        return Err(N2WError::Index("list index out of range".into()));
    }

    // Reconstruct `str(float_number)`: shortest round-trip == fixed format to
    // the repr-derived precision.
    let s = format!("{:.*}", precision as usize, x);
    let frac = s.split_once('.').map(|(_, f)| f).unwrap_or("");

    // postfix = " ".join(to_cardinal(int(c)) for c in float_part)
    let mut parts: Vec<String> = Vec::new();
    for c in frac.chars() {
        // Python `int(c)`: a non-digit would raise ValueError (out of scope).
        let d = c
            .to_digit(10)
            .ok_or_else(|| N2WError::Value("invalid literal for int() with base 10".into()))?;
        parts.push(to_cardinal_impl(&BigInt::from(d))?);
    }

    Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, parts.join(" ")))
}

/// Reconstruct the fixed-point `str(Decimal)` of a **non-negative** value.
///
/// Matches Python's `str(Decimal)` over the fixed-point range (exponent `<= 0`,
/// adjusted exponent `>= -6`) — which covers the whole corpus. Decimals that
/// Python would render in scientific notation would differ, but only in the
/// text of the resulting `ValueError`'s bad literal, never in its *type*. See
/// the port report.
fn decimal_fixed_str(d: &BigDecimal, precision: u32) -> String {
    let scaled = d.with_scale(precision as i64);
    let coeff = scaled.as_bigint_and_exponent().0; // >= 0
    if precision == 0 {
        return coeff.to_string();
    }
    let divisor = BigInt::from(10).pow(precision);
    let int_part = &coeff / &divisor;
    let frac_val = &coeff % &divisor;
    let frac_raw = frac_val.to_string();
    let frac_str = format!(
        "{}{}",
        "0".repeat((precision as usize).saturating_sub(frac_raw.len())),
        frac_raw
    );
    format!("{}.{}", int_part, frac_str)
}

/// Python's `to_cardinal(<Decimal>)`.
///
/// RM's `to_cardinal` gates the float branch on `isinstance(number, float)`.
/// A `Decimal` is **not** a `float`, so it skips `float_to_words` and falls
/// through to the integer size-dispatch, which crashes there:
///
/// * `abs(d) < 10**6` → every branch ends up indexing a list with the Decimal
///   (`CARDINAL_WORDS[number]`, directly or after one recursion) → **TypeError**
///   (`list indices must be integers or slices, not decimal.Decimal`). The four
///   sub-ranges (`<20`, `<100`, `<1000`, `<10**6`) all reduce to this.
/// * `abs(d) >= 10**6` → `big_number_to_cardinal` splits `str(number)` — which
///   carries the '.' — and, since the integer part has `>= 7` digits while
///   `predigits <= 3`, the '.' always lands in `digits[predigits:]`; the branch
///   then calls `int("...<frac>...")` → **ValueError**. A magnitude `>=
///   10**65` raises **OverflowError** first (#159); a `str(number)` of 66+
///   characters below that still raises **NotImplementedError**.
///
/// A negative Decimal prepends "minus " *after* the recursive call, which
/// raises, so the sign never reaches the output — take abs and reproduce the
/// crash. (An integer-valued Decimal `>= 10**6` whose `str` has no '.' would
/// instead spell out; that value never reaches this path from the corpus, but
/// it is reproduced for fidelity.)
fn decimal_to_cardinal(value: &BigDecimal, precision: u32) -> Result<String> {
    let d = value.abs();
    if d < BigDecimal::from(1_000_000i64) {
        return Err(N2WError::Type(
            "list indices must be integers or slices, not decimal.Decimal".into(),
        ));
    }

    // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
    // checked before Python's `len(str(number)) >= 66` string guard.
    check_maxval(&d.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
    // >= 10**6: reproduce big_number_to_cardinal on str(number).
    let s = decimal_fixed_str(&d, precision);
    let length = s.chars().count();
    // Only reachable below maxval when the fraction digits push str(number)
    // past 65 characters; magnitudes past maxval were rejected above (#159).
    if length >= 66 {
        return Err(N2WError::NotImplemented(
            "The given number is too large.".into(),
        ));
    }

    if !s.contains('.') {
        // Integer-valued Decimal (exponent >= 0): str has no '.', so
        // big_number_to_cardinal spells it out rather than crashing. Python
        // wraps the successful body in "minus " for a negative input.
        let n: BigInt = s.parse().expect("all-digit decimal string");
        let body = big_number_to_cardinal(&n)?;
        return Ok(if value.is_negative() {
            format!("{}{}", MINUS_PREFIX_WORD, body)
        } else {
            body
        });
    }

    // The '.' is inside digits[predigits:] (predigits = length % 3 or 3), so
    // int(exponent_str) raises ValueError with that slice as the bad literal.
    let predigits = if length % 3 != 0 { length % 3 } else { 3 };
    let exponent_str: String = s.chars().skip(predigits).collect();
    Err(N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        exponent_str
    )))
}

/// `Num2Word_RM_SURSILV.to_ordinal` for a `float` argument:
///
/// ```python
/// if number < 0:  return MINUS_PREFIX_WORD + self.to_ordinal(-number)
/// elif number % 1 != 0:  return self.float_to_words(number, ordinal=True)
/// elif number <= 20:  return ORDINAL_WORDS[number]   # float index -> TypeError
/// else: cardinal = self.to_cardinal(number)  # float branch, "... comma nulla"
/// ```
///
/// Corpus-confirmed quirks, all reproduced:
///   * a *fractional* float works: `2.5` -> "secund comma tschun"
///     (ordinal prefix over `int(2.5)`, cardinal digit words after);
///   * a *whole* float `<= 20` (incl. `-0.0`/`0.0`: `-0.0 < 0` is False,
///     `-0.0 % 1 == 0`) dies on `ORDINAL_WORDS[<float>]` -> TypeError;
///   * a whole float `> 20` renders its float *cardinal* ("ventgin comma
///     nulla") and then gets the ordinal suffix: "ventgin comma nullavel";
///   * `1e+16`/`1e+20` (whole, > 20) reach the cardinal float branch and
///     die on the pointless repr -> IndexError.
fn float_ordinal_f64(f: f64, precision: u32) -> Result<String> {
    if f < 0.0 {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            float_ordinal_f64(-f, precision)?
        ));
    }
    if f.fract() != 0.0 {
        // float_to_words(number, ordinal=True): the prefix is the ordinal of
        // int(number); the digit words stay *cardinal*.
        let prefix = to_ordinal_impl(&f64_trunc_to_bigint(f)?)?;
        if !float_repr_has_point(f) {
            return Err(N2WError::Index("list index out of range".into()));
        }
        let s = format!("{:.*}", precision as usize, f);
        let frac = s.split_once('.').map(|(_, x)| x).unwrap_or("");
        let mut parts: Vec<String> = Vec::new();
        for c in frac.chars() {
            let d = c
                .to_digit(10)
                .ok_or_else(|| N2WError::Value("invalid literal for int() with base 10".into()))?;
            parts.push(to_cardinal_impl(&BigInt::from(d))?);
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
    // "... comma nulla" here, so the 'a' rule always fires; the "avel" arm is
    // kept for shape parity with the integer path.
    let cardinal = float_body_f64(f, precision)?;
    let suffix = if cardinal.ends_with('a') { "vel" } else { "avel" };
    Ok(cardinal + suffix)
}

/// `Num2Word_RM_SURSILV.to_ordinal` for a `Decimal` argument. `Decimal % 1`
/// works, so a *fractional* Decimal takes the `float_to_words(ordinal=True)`
/// branch and renders (reading `str(Decimal)`); a whole-valued one falls into
/// the integer branches: `<= 20` dies on `ORDINAL_WORDS[<Decimal>]`
/// (TypeError), `> 20` re-enters `to_cardinal(Decimal)` — TypeError below
/// `10**6`, str-splitting above (ValueError when the '.' survives).
/// `Decimal("-0.0") < 0` is False, so it is *not* minus-prefixed — it crashes
/// in the table branch like `0.0` does.
fn decimal_ordinal(value: &BigDecimal, precision: u32) -> Result<String> {
    if value.is_negative() {
        return Ok(format!(
            "{}{}",
            MINUS_PREFIX_WORD,
            decimal_ordinal(&value.abs(), precision)?
        ));
    }
    if !value.is_integer() {
        // float_to_words(number, ordinal=True) over str(Decimal).
        let pre = value.with_scale(0).as_bigint_and_exponent().0;
        let prefix = to_ordinal_impl(&pre)?;
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
            parts.push(to_cardinal_impl(&BigInt::from(d))?);
        }
        return Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, parts.join(" ")));
    }
    if value <= &BigDecimal::from(20) {
        return Err(N2WError::Type(
            "list indices must be integers or slices, not decimal.Decimal".into(),
        ));
    }
    let cardinal = decimal_to_cardinal(value, precision)?;
    let suffix = if cardinal.ends_with('a') { "vel" } else { "avel" };
    Ok(cardinal + suffix)
}

pub struct LangRmSursilv;

impl LangRmSursilv {
    pub fn new() -> Self {
        LangRmSursilv
    }
}

impl Default for LangRmSursilv {
    fn default() -> Self {
        LangRmSursilv::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRmSursilv {
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
        Err(crate::lang_rm::unsupported("rm_sursilv", "to='currency'"))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sursilv", "to='cheque'"))
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        to_cardinal_impl(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        to_ordinal_impl(value)
    }

    /// Python's class had no `to_ordinal_num` (AttributeError for every
    /// input), and no ordinal-numeral marker is attested for Romansh in the
    /// module or its tests, so the port raises a clear NotImplementedError
    /// rather than inventing one (#157).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sursilv", "to='ordinal_num'"))
    }

    /// Python's class had no `to_year` (AttributeError for every input).
    /// `Num2Word_Base.to_year` is the cardinal, which is what the port
    /// returns (#157).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `Num2Word_RM_SURSILV.to_cardinal(<float | Decimal>)` — the float/Decimal
    /// cardinal path.
    ///
    /// This class has no base, so it inherits no `Num2Word_Base.to_cardinal_float`;
    /// instead its overridden `to_cardinal` handles non-integers inline:
    ///
    /// ```python
    /// def to_cardinal(self, number):
    ///     if number < 0:
    ///         string = MINUS_PREFIX_WORD + self.to_cardinal(-number)
    ///     elif isinstance(number, float):
    ///         string = self.float_to_words(number)
    ///     elif number < 20: ...   # a Decimal reaches these and crashes
    /// ```
    ///
    /// * **float** → the sign is stripped (recursing prepends "minus "), then
    ///   `float_to_words` renders it. Integer-valued floats are included (the
    ///   gate is `isinstance(number, float)`, not `int(number) != number`).
    /// * **Decimal** → not a `float`, so in Python it fell through to the
    ///   integer dispatch and crashed there; it now reads like the int or the
    ///   float (#157).
    ///
    /// `precision_override` (the `precision=` kwarg) is ignored, exactly as
    /// Python ignores it here: the bare class never sets `self.precision`, so
    /// the dispatcher's `hasattr(converter, "precision")` guard is false and
    /// `float_to_words` reads `str(number)` regardless (confirmed live:
    /// `num2words(2.675, lang="rm_sursilv", precision=1)` is unchanged).
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
            FloatValue::Float { value, precision } => {
                if *value < 0.0 {
                    // `number < 0` → MINUS_PREFIX_WORD + to_cardinal(-number),
                    // where -number re-enters the float branch.
                    Ok(format!(
                        "{}{}",
                        MINUS_PREFIX_WORD,
                        float_body_f64(-value, *precision)?
                    ))
                } else {
                    float_body_f64(*value, *precision)
                }
            }
            FloatValue::Decimal { value, precision } => {
                decimal_to_cardinal(value, *precision)
            }
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing. The gate is
    /// `isinstance(number, float)`, **not** `int(number) == number`, so a
    /// whole-valued float still renders through `float_to_words`
    /// (`1.0` -> "in comma nulla") and a whole-valued Decimal crashed
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
            FloatValue::Float { value, precision } => {
                if *value < 0.0 {
                    Ok(format!(
                        "{}{}",
                        MINUS_PREFIX_WORD,
                        float_body_f64(-value, *precision)?
                    ))
                } else {
                    float_body_f64(*value, *precision)
                }
            }
            FloatValue::Decimal { value, precision } => {
                decimal_to_cardinal(value, *precision)
            }
        }
    }

    /// `to_ordinal(float/Decimal)` — see [`float_ordinal_f64`] /
    /// [`decimal_ordinal`] for the branch-by-branch mapping.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // A whole Decimal crashed on `ORDINAL_WORDS[<Decimal>]` in Python; it
        // now reads like the int (#157).
        if let FloatValue::Decimal { value: d, .. } = value {
            if d.is_integer() {
                return self.to_ordinal(&d.with_scale(0).as_bigint_and_exponent().0);
            }
        }
        match value {
            FloatValue::Float { value, precision } => float_ordinal_f64(*value, *precision),
            FloatValue::Decimal { value, precision } => decimal_ordinal(value, *precision),
        }
    }

    /// See [`Lang::to_ordinal_num`] above (#157).
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sursilv", "to='ordinal_num'"))
    }

    /// `to_year(float/Decimal)`: a whole value is the integer year; a
    /// fractional one is a `TypeError`, as in `lang_en` (#157).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_year(&i),
            None => Err(crate::base::year_float_error(value)),
        }
    }


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

    // `str_to_number`: the trait default, the shared `Decimal(value)` parse.
    // Python's class had none, so every string raised AttributeError (#157).

    /// Python's class had no `to_fraction` (AttributeError for every n/d).
    /// Base's generic reading ("<n> <ordinal>s") is not Romansh, so this is a
    /// clear NotImplementedError instead (#157).
    fn to_fraction(&self, _numerator: &BigInt, _denominator: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_sursilv", "fractions"))
    }
}
