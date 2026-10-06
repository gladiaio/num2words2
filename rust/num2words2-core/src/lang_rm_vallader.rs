//! Port of `lang_RM_VALLADER.py` (Romansh — Vallader idiom).
//!
//! Shape: **self-contained**. `Num2Word_RM_VALLADER` is a **bare class** —
//! `class Num2Word_RM_VALLADER:` with no base at all, not even
//! `Num2Word_Base`. Its `__init__` is a bare `pass`. So there is no
//! inheritance chain to chase: everything the converter can do is the six
//! methods in that one file, and everything it *cannot* do raised
//! `AttributeError` at lookup time in Python (fixed, #157 — see below).
//!
//! Consequences the port depends on:
//!
//!   * `self.cards` / `self.MAXVAL` are never built, and no
//!     `high_numwords`/`mid_numwords`/`low_numwords` exist. `cards` and
//!     `merge` therefore stay at their trait defaults and are never reached.
//!   * `self.negword` / `self.pointword` are never set; the class uses its own
//!     `MINUS_PREFIX_WORD = "minus "` and `FLOAT_INFIX_WORD = " comma "`.
//!   * The only ceiling is `big_number_to_cardinal`'s explicit guard at
//!     `len(str(number)) >= 66` — `NotImplementedError` in Python, an
//!     `OverflowError` here since gladiaio/num2words2#159, with `maxval()`
//!     reporting 10^65. See "Ceiling".
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
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, verified line-by-line against the frozen corpus:
//!
//! 1. **`to_ordinal` of a big number suffixes the plural.** `to_ordinal` just
//!    glues a suffix onto `to_cardinal`'s output, with no awareness of the
//!    "milliuns"/"milliards" plural or the spaces inside it. Hence
//!    `to_ordinal(10**7)` == "desch milliunsavel" and `to_ordinal(10**10)` ==
//!    "desch milliardsavel" — the suffix lands on the plural "s". Likewise
//!    `to_ordinal(1000001)` == "ün milliun ed ünavel". Corpus-confirmed.
//!
//! 2. **`to_ordinal` of a negative is not a real ordinal**: it prefixes
//!    "minus " and recurses, so `to_ordinal(-1)` == "minus prüm" rather than
//!    raising. There is no `verify_ordinal` in this class at all.
//!
//! 3. **`adapt_thousand` is applied to the whole assembled string**, not just
//!    the multiplier, so its "traismilli" -> "trajamilli" rule fires on a
//!    "trais" that is merely the *units digit* of the thousands multiplier.
//!    `to_cardinal(123456)`: thousands 123 -> "tschientvainchetrais", + "milli"
//!    -> "tschientvainchetraismilli", and `adapt_thousand` rewrites the
//!    embedded "traismilli" -> "tschientvainchetrajamilli...". Corpus-
//!    confirmed ("tschientvainchetrajamilliquattertschienttschinquantases").
//!    The same latent aliasing exists for "duosmilli".
//!
//! 4. **`tens_to_cardinal`'s `else` branch is unreachable.** It is only called
//!    for `20 <= number < 100`, so `tens` is 2..9 and `STR_TENS` covers every
//!    one of those keys. The fallback `CARDINAL_WORDS[tens][:-1] + "anta"` can
//!    never run. Kept verbatim anyway — see [`tens_to_cardinal`].
//!
//! 5. **`exponent_length_to_string`'s index-0 ("nolla") branch is
//!    unreachable.** `EXPONENT_PREFIXES[0]` is `ZERO`, which would compose the
//!    nonsense "nollailliard". Reaching it needs `exponent_length == 3`, i.e.
//!    `len(digits) <= 6`, but `big_number_to_cardinal` only runs for
//!    `number >= 10**6` (`len(digits) >= 7`, hence `exponent_length >= 6`).
//!
//! 6. **The empty-`exponent` path (`int("")` -> `ValueError`) is
//!    unreachable.** `set(exponent) != set("0")` is *true* for an empty
//!    `exponent` (`set()` != `{"0"}`), which would fall into the branch that
//!    calls `int("".join(exponent))`. But `exponent` has length
//!    `len(digits) - predigits >= 7 - 3 == 4`, so it is never empty. Modelled
//!    as `N2WError::Value` rather than a panic, for fidelity — see
//!    [`parse_int`].
//!
//! # Ceiling
//!
//! `big_number_to_cardinal` raises (`OverflowError` since #159; Python's
//! `NotImplementedError("The given number is too large.")`) for
//! `len(str(number)) >= 66`, i.e. from 10^65. That guard exactly protects
//! `EXPONENT_PREFIXES` (11 entries, indices 0..=10): with `length <= 65`,
//! `predigits = length % 3 or 3` gives `exponent_length = length - predigits`
//! of at most 63, and `63 // 6 == 10` — the last valid index. So the
//! `IndexError` that would otherwise follow is unreachable; [`exponent_prefix`]
//! models it defensively regardless.
//!
//! # Sign / division semantics
//!
//! `to_cardinal` and `to_ordinal` both strip the sign first and recurse on
//! `-number`, so every helper below is only ever reached with a **non-negative**
//! value. Rust's truncating `/` and `%` on `BigInt` therefore agree with
//! Python's flooring `//` and `%` throughout; no divergence is possible.
//!
//! # The float/Decimal cardinal path
//!
//! `to_cardinal_float` (below) ports the `isinstance(number, float)` branch of
//! `to_cardinal` — i.e. `float_to_words` — and the `Decimal` fall-through. This
//! class **does not** inherit `Num2Word_Base`, so it never uses `float2tuple`
//! or the `< 0.01` heuristic: the fractional digits come straight from
//! `str(float_number).split('.')[1]`, the shortest round-trip repr. See
//! [`LangRmVallader::float_to_words`] and [`LangRmVallader::decimal_to_cardinal`].
//!
//! # Still out of scope
//!
//! The `number % 1 != 0` branch of `to_ordinal` (`float_to_words(..,
//! ordinal=True)`) is an *ordinal* float path with no trait hook here, so it is
//! not ported. Integer input only for `to_ordinal`.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};

// Globals — mirroring the module-level constants in lang_RM_VALLADER.py.
// -------------------------------------------------------------------------

const ZERO: &str = "nolla";

const MINUS_PREFIX_WORD: &str = "minus ";

/// `FLOAT_INFIX_WORD` — the class attribute glued between the integer prefix
/// and the digit-by-digit fractional part. Note the surrounding spaces:
/// `float_to_words` returns `prefix + " comma " + postfix`.
const FLOAT_INFIX_WORD: &str = " comma ";

/// `CARDINAL_WORDS` — 20 entries, indices 0..=19.
const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "ün",
    "duos",
    "trais",
    "quatter",
    "tschinch",
    "ses",
    "set",
    "ot",
    "nouv",
    "desch",
    "ündesch",
    "dudesch",
    "traidesch",
    "quattordesch",
    "quindesch",
    "saidesch",
    "deschset",
    "deschdot",
    "deschnouv",
];

/// `ORDINAL_WORDS` — 21 entries, indices 0..=20 (one longer than
/// `CARDINAL_WORDS`, which is why `to_ordinal` tests `number <= 20` while
/// `to_cardinal` tests `number < 20`).
const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "prüm",
    "seguond",
    "terz",
    "quart",
    "tschinchavel",
    "sesavel",
    "settavel",
    "ottavel",
    "nouvavel",
    "deschavel",
    "ündeschavel",
    "dudeschavel",
    "traideschavel",
    "quattordeschavel",
    "quindeschavel",
    "saideschavel",
    "deschsettavel",
    "deschdottavel",
    "deschnouvavel",
    "vainchavel",
];

/// `STR_TENS` — note key 2 is "vainche", not "vainch". The trailing "e" is a
/// join vowel; the bare surface form "vainch" is restored by
/// [`phonetic_contraction`] via the "vainche_" rule.
fn str_tens(tens: &BigInt) -> Option<&'static str> {
    match tens.to_u32()? {
        2 => Some("vainche"),
        3 => Some("trenta"),
        4 => Some("quaranta"),
        5 => Some("tschinquanta"),
        6 => Some("sesanta"),
        7 => Some("settanta"),
        8 => Some("ottanta"),
        9 => Some("novanta"),
        _ => None,
    }
}

/// `EXPONENT_PREFIXES` — 11 entries, indices 0..=10. Index 0 is `ZERO`; see
/// module quirk 5 for why that entry is unreachable.
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

// Utils
// =====

/// Python's `int(s)`. Only reachable with an all-digit `s`; the `ValueError`
/// arm exists for the unreachable `int("")` of module quirk 6.
fn parse_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
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

/// Python's `int(f)` — truncate toward zero, exact at any magnitude (every
/// whole f64 is exactly representable as a BigInt). `int(inf)` is Python's
/// OverflowError, `int(nan)` its ValueError; neither is reachable from the
/// corpus but both are modelled.
fn f64_trunc_to_bigint(f: f64) -> Result<BigInt> {
    if f.is_nan() {
        return Err(N2WError::Value(
            "cannot convert float NaN to integer".to_string(),
        ));
    }
    num_traits::FromPrimitive::from_f64(f.trunc()).ok_or_else(|| {
        N2WError::Overflow("cannot convert float infinity to integer".to_string())
    })
}

/// `CARDINAL_WORDS[i]`. Python would raise `IndexError` out of range (and wrap
/// for a negative index, but no caller can produce one — every helper sees a
/// non-negative value). Out of range is unreachable from the four ported modes.
fn card_word(i: &BigInt) -> Result<&'static str> {
    i.to_usize()
        .and_then(|i| CARDINAL_WORDS.get(i).copied())
        .ok_or_else(|| N2WError::Index("list index out of range".to_string()))
}

/// `ORDINAL_WORDS[i]`. Same reasoning as [`card_word`].
fn ord_word(i: &BigInt) -> Result<&'static str> {
    i.to_usize()
        .and_then(|i| ORDINAL_WORDS.get(i).copied())
        .ok_or_else(|| N2WError::Index("list index out of range".to_string()))
}

/// `EXPONENT_PREFIXES[i]`. Unreachable out of range — see module "Ceiling".
fn exponent_prefix(i: usize) -> Result<&'static str> {
    EXPONENT_PREFIXES
        .get(i)
        .copied()
        .ok_or_else(|| N2WError::Index("list index out of range".to_string()))
}

/// `_` is a marker for "empty", i.e. no following unit.
///
/// The rule order is load-bearing and matches Python's chained `.replace`
/// calls exactly: each rule sees the previous rule's output. In particular
/// "eün" fires before "vainche_", so "vaincheün" contracts to "vainchün"
/// rather than surviving to the "vainche_" rule.
fn phonetic_contraction(s: &str) -> String {
    s.replace("aün", "ün") // ex. "trentaün" -> "trentün"
        .replace("eün", "ün") // ex. "vaincheün" -> "vainchün"
        .replace("aot", "ot") // ex. "quarantaot" -> "quarantot"
        .replace("eot", "ot") // ex. "vaincheot" -> "vainchot"
        .replace("vainche_", "vainch") // ex. "vainche" -> "vainch"
        .replace('_', "")
}

/// Surface modifications: collective plural + e/ed phonotactic adaptation.
fn adapt_hundred(s: &str) -> String {
    s.replace("duostschient", "duatschient")
        .replace("traistschient", "trajatschient")
        .replace("eün", "edün")
        .replace("eot", "edot")
}

/// Surface modifications: collective plural + e/ed phonotactic adaptation.
/// Applied to the whole assembled string — see module quirk 3.
fn adapt_thousand(s: &str) -> String {
    s.replace("duosmilli", "duamilli")
        .replace("traismilli", "trajamilli")
        .replace("eün", "edün")
        .replace("eot", "edot")
}

/// Surface modifications: article gender agreement + e/ed phonotactic
/// adaptation. Python pads with a space on each side so the " e ün" / " e ot"
/// patterns can match at the string edges; `big_number_to_cardinal` strips the
/// padding back off afterwards.
fn adapt_milliard(s: &str) -> String {
    let s = format!(" {} ", s);
    s.replace(" e ün", " ed ün").replace(" e ot", " ed ot")
}

/// We always assume `exponent` to be a multiple of 3. If that is not true then
/// `big_number_to_cardinal` did something wrong.
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let prefix = exponent_prefix(exponent_length / 6)?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}illiun", prefix))
    } else {
        Ok(format!("{}illiard", prefix))
    }
}

fn omitt_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        String::new()
    } else {
        number_to_string.to_string()
    }
}

fn empty_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        "_".to_string()
    } else {
        number_to_string.to_string()
    }
}

// Main class
// ==========

pub struct LangRmVallader;

impl LangRmVallader {
    pub fn new() -> Self {
        LangRmVallader
    }

    /// `20 <= number < 100`.
    fn tens_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let tens = number / BigInt::from(10);
        let units = number % BigInt::from(10);
        let prefix = match str_tens(&tens) {
            Some(p) => p.to_string(),
            None => {
                // Unreachable: `tens` is 2..9 here and STR_TENS covers all of
                // those. Kept verbatim — see module quirk 4. Python's `[:-1]`
                // drops the last *character*, so index by chars, not bytes.
                let word = card_word(&tens)?;
                let mut chars: Vec<char> = word.chars().collect();
                chars.pop();
                format!("{}anta", chars.into_iter().collect::<String>())
            }
        };
        // we keep track of 0 using '_' -- removed in phonetic_contraction
        let postfix = empty_if_zero(card_word(&units)?);
        Ok(phonetic_contraction(&format!("{}{}", prefix, postfix)))
    }

    /// `100 <= number < 1000`.
    fn hundreds_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let hundreds = number / BigInt::from(100);
        let tens = number % BigInt::from(100);
        let mut prefix = "tschient".to_string();
        if hundreds != BigInt::from(1) {
            prefix = format!("{}tschient", card_word(&hundreds)?);
        }
        let postfix = omitt_if_zero(&self.to_cardinal(&tens)?);
        // "e/ed" is inserted if tens <= 13 or = 15, 16, 20, 30
        // distribution may seem unusual but it was reviewed by a native speaker
        let mut infix = "";
        let t = tens.to_u32().unwrap_or(u32::MAX);
        if (t > 0 && t <= 13) || matches!(t, 15 | 16 | 20 | 30) {
            infix = "e";
        }
        Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// `1000 <= number < 1_000_000`.
    fn thousands_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let thousands = number / BigInt::from(1000);
        let hundreds = number % BigInt::from(1000);
        let mut prefix = "milli".to_string();
        if thousands != BigInt::from(1) {
            prefix = format!("{}milli", self.to_cardinal(&thousands)?);
        }
        let postfix = omitt_if_zero(&self.to_cardinal(&hundreds)?);
        // "e/ed" is inserted if tens <= 100
        let mut infix = "";
        if hundreds <= BigInt::from(100) && !postfix.is_empty() {
            infix = "e";
        }
        Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// `number >= 1_000_000`.
    fn big_number_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let digits: Vec<char> = number.to_string().chars().collect();
        let length = digits.len();
        check_maxval(number, maxval_ceiling())?;
        // This is how many digits come before the "illion" term.
        //   tschient milliards => 3
        //   desch milliuns => 2
        //   ün milliard => 1
        // Python: `length % 3 or 3` — 0 falls back to 3.
        let predigits = if length % 3 == 0 { 3 } else { length % 3 };
        let multiplier: String = digits[..predigits].iter().collect();
        let exponent: String = digits[predigits..].iter().collect();
        let mut infix = exponent_length_to_string(exponent.chars().count())?;
        let prefix;
        // Python compares the *list* `digits[:predigits] == ["1"]`, i.e. true
        // only when predigits == 1 and that digit is "1". The string compare
        // below is equivalent.
        if multiplier == "1" {
            prefix = "ün ".to_string();
        } else {
            prefix = self.to_cardinal(&parse_int(&multiplier)?)?;
            // Plural form
            infix = format!(" {}s", infix);
        }
        // Read as: Does the value of exponent equal 0?
        // Python: `set(exponent) != set("0")`. That is false only when every
        // char is "0" AND exponent is non-empty — an empty exponent yields
        // `set() != {"0"}` == true and falls into the `int("")` arm. See
        // module quirk 6; unreachable, but modelled exactly.
        let exponent_is_all_zero = !exponent.is_empty() && exponent.chars().all(|c| c == '0');
        let postfix;
        if !exponent_is_all_zero {
            let exponent_str = &exponent;
            postfix = self.to_cardinal(&parse_int(exponent_str)?)?;
            // we introduce "e" if 3-digits gap before next value
            if exponent_str.starts_with("000") {
                infix.push_str(" e ");
            } else {
                infix.push(' ');
            }
        } else {
            postfix = String::new();
        }
        Ok(adapt_milliard(&format!("{}{}{}", prefix, infix, postfix))
            .trim()
            .to_string())
    }

    /// Python's `float_to_words(float_number, ordinal=False)` for the cardinal
    /// (non-ordinal) case. `float_number` is already **non-negative** here — the
    /// `number < 0` branch of `to_cardinal` strips the sign and recurses before
    /// ever reaching the `isinstance(number, float)` branch, so the helper only
    /// ever sees a positive value (see [`float_to_cardinal`]).
    ///
    /// ```python
    /// prefix = self.to_cardinal(int(float_number))
    /// float_part = str(float_number).split('.')[1]
    /// postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
    /// return prefix + " comma " + postfix
    /// ```
    ///
    /// The fractional digits come from `str(float_number)` — the shortest
    /// round-trip repr — **not** from `base.float2tuple`. This class does not
    /// inherit `Num2Word_Base` and never touches `float2tuple`; there is no
    /// `< 0.01` heuristic in play. `precision` (computed Python-side as
    /// `abs(Decimal(str(value)).as_tuple().exponent)`) is exactly the number of
    /// fractional digits in that repr, so `format!("{:.*}", precision, v)`
    /// reproduces `str(v)`'s fractional part digit-for-digit, including leading
    /// zeros (`0.01` -> "01", `1.005` -> "005"). Rust's bare `{}` Display could
    /// not be used: it drops the trailing ".0" (`1.0` -> "1") and would make
    /// `split('.')` miss the fraction entirely.
    fn float_to_words(&self, float_number: f64, precision: u32) -> Result<String> {
        // prefix = self.to_cardinal(int(float_number)) — int() truncates toward
        // zero; float_number >= 0 so this is the floor. Computed *before* the
        // point probe, exactly as Python does — for a huge float the maxval
        // OverflowError (#159) beats the IndexError below.
        let pre = f64_trunc_to_bigint(float_number)?;
        let prefix = self.to_cardinal(&pre)?;

        // float_part = str(float_number).split('.')[1]: a repr in exponent
        // form ("1e+16", "1e+20") has no '.' — Python's IndexError,
        // corpus-confirmed. The `{:.p$}` reconstruction below cannot detect
        // this (`precision` for `1e+16` is 16, which *would* print a point),
        // hence the explicit test.
        if !float_repr_has_point(float_number) {
            return Err(N2WError::Index("list index out of range".to_string()));
        }
        let repr = format!("{:.*}", precision as usize, float_number);
        let float_part = repr.split_once('.').map(|(_, f)| f).unwrap_or("");

        // postfix = " ".join(self.to_cardinal(int(c)) for c in float_part)
        let mut words: Vec<String> = Vec::with_capacity(float_part.len());
        for c in float_part.chars() {
            // int(c) — the character is always a digit here; the ValueError arm
            // mirrors Python's int() on a non-digit and is unreachable.
            let d = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    c
                ))
            })?;
            words.push(self.to_cardinal(&BigInt::from(d))?);
        }

        Ok(format!(
            "{}{}{}",
            prefix,
            FLOAT_INFIX_WORD,
            words.join(" ")
        ))
    }

    /// The `float` arm of `to_cardinal`. Reproduces the leading `number < 0`
    /// branch, which prepends "minus " for **any** negative float and recurses
    /// on `-number` — so the sign lands even when the integer part is non-zero
    /// (`-12.34` -> "minus dudesch comma trais quatter"). This is unlike the
    /// `Num2Word_Base` float path, which only re-adds the sign when the integer
    /// part is zero.
    fn float_to_cardinal(&self, value: f64, precision: u32) -> Result<String> {
        if value < 0.0 {
            // string = MINUS_PREFIX_WORD + self.to_cardinal(-number); the
            // positive value then takes the isinstance(float) branch.
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.float_to_words(-value, precision)?
            ));
        }
        self.float_to_words(value, precision)
    }

    /// The `Decimal` arm. A `Decimal` is **not** a `float`, so it never reaches
    /// `float_to_words`; it falls straight through `to_cardinal`'s integer
    /// dispatch, where the branches index `CARDINAL_WORDS`/recurse with a
    /// `Decimal` that has no `__index__`, or feed a `.`-bearing string to
    /// `int()`. The corpus records every `cardinal_dec` row as an error:
    ///
    ///   * `d < 1_000_000` -> `CARDINAL_WORDS[Decimal]` (directly, or after the
    ///     tens/hundreds/thousands helpers recurse down to it) -> **TypeError**.
    ///   * `d >= 1_000_000` and fractional -> `big_number_to_cardinal` splits
    ///     `str(d)` (which contains a '.') and calls `int("".join(exponent))`
    ///     on a fractional substring -> **ValueError**. The '.' is always in
    ///     the exponent portion (the integer part has >= 7 digits, `predigits`
    ///     is at most 3), so this arm is reached unconditionally.
    ///   * `d >= 1_000_000` and integral -> a genuine cardinal, via the same
    ///     digit path the integer `big_number_to_cardinal` uses. No corpus row
    ///     exercises this, but it is reproduced for fidelity.
    fn decimal_to_cardinal(&self, value: &BigDecimal) -> Result<String> {
        // number < 0: MINUS_PREFIX_WORD + to_cardinal(-number). The inner call
        // still raises for these inputs, so the "minus " is never emitted — the
        // error propagates, exactly as Python raises before completing the
        // concatenation.
        if value.is_negative() {
            let inner = self.decimal_to_cardinal(&(-value.clone()))?;
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, inner));
        }

        // Every dispatch branch below 1_000_000 bottoms out in a
        // `CARDINAL_WORDS[Decimal]` (or `STR_TENS`/recursion that reaches one),
        // and a Decimal is not a valid list index -> TypeError.
        if *value < BigDecimal::from(1_000_000i64) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal"
                    .to_string(),
            ));
        }

        // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
        // checked before Python's `len(str(number)) >= 66` string guard.
        check_maxval(&value.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        // number >= 1_000_000: big_number_to_cardinal(number). It works on
        // digits = list(str(number)); str(Decimal) keeps the '.'.
        // `python_decimal_str` reproduces Python's `str(Decimal)` exactly,
        // including the scientific form: `str(Decimal("1E+20"))` is `"1E+20"`,
        // which has no '.' but fails `int()` all the same (ValueError) — a
        // plain digit expansion would instead succeed and diverge from the
        // corpus.
        let s = crate::strnum::python_decimal_str(value);
        let digits: Vec<char> = s.chars().collect();
        let length = digits.len();
        // Only reachable below maxval when the fraction digits push str(number)
        // past 65 characters; magnitudes past maxval were rejected above (#159).
        if length >= 66 {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        if s.contains('.') {
            // exponent = digits[predigits:]; int("".join(exponent)) fails on the
            // '.' -> ValueError. predigits = length % 3 or 3.
            let predigits = if length % 3 == 0 { 3 } else { length % 3 };
            let exponent: String = digits[predigits..].iter().collect();
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                exponent
            )));
        }
        // Integral Decimal >= 1_000_000: str(number) is all digits, so this is
        // the very same computation the integer path performs. (A scientific
        // `str(Decimal)` such as "1E+20" also lands here — no '.' — and fails
        // the int() below with ValueError, matching Python's int("1E") crash.)
        let n = parse_int(&s)?;
        self.big_number_to_cardinal(&n)
    }

    /// `Num2Word_RM_VALLADER.to_ordinal` for a `float` argument:
    ///
    /// ```python
    /// if number < 0:  return MINUS_PREFIX_WORD + self.to_ordinal(-number)
    /// elif number % 1 != 0:  return self.float_to_words(number, ordinal=True)
    /// elif number <= 20:  return ORDINAL_WORDS[number]   # float index -> TypeError
    /// else: cardinal = self.to_cardinal(number)  # float branch, "... comma nolla"
    /// ```
    ///
    /// Corpus-confirmed quirks, all reproduced:
    ///   * a *fractional* float works: `2.5` -> "seguond comma tschinch"
    ///     (ordinal prefix over `int(2.5)`, cardinal digit words after);
    ///   * a *whole* float `<= 20` (incl. `-0.0`/`0.0`: `-0.0 < 0` is False,
    ///     `-0.0 % 1 == 0`) dies on `ORDINAL_WORDS[<float>]` -> TypeError;
    ///   * a whole float `> 20` renders its float *cardinal* ("vainchün comma
    ///     nolla") and then gets the ordinal suffix: "vainchün comma nollavel";
    ///   * `1e+16`/`1e+20` (whole, > 20) reach the cardinal float branch and
    ///     die on the pointless repr -> IndexError.
    fn float_ordinal(&self, f: f64, precision: u32) -> Result<String> {
        if f < 0.0 {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.float_ordinal(-f, precision)?
            ));
        }
        if f.fract() != 0.0 {
            // float_to_words(number, ordinal=True): the prefix is the ordinal
            // of int(number); the digit words stay *cardinal*.
            let prefix = self.to_ordinal(&f64_trunc_to_bigint(f)?)?;
            if !float_repr_has_point(f) {
                return Err(N2WError::Index("list index out of range".to_string()));
            }
            let repr = format!("{:.*}", precision as usize, f);
            let float_part = repr.split_once('.').map(|(_, x)| x).unwrap_or("");
            let mut words: Vec<String> = Vec::with_capacity(float_part.len());
            for c in float_part.chars() {
                let d = c.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        c
                    ))
                })?;
                words.push(self.to_cardinal(&BigInt::from(d))?);
            }
            return Ok(format!(
                "{}{}{}",
                prefix,
                FLOAT_INFIX_WORD,
                words.join(" ")
            ));
        }
        if f <= 20.0 {
            // ORDINAL_WORDS[<float>] — a float is not a valid list index.
            return Err(N2WError::Type(
                "list indices must be integers or slices, not float".to_string(),
            ));
        }
        // cardinal = self.to_cardinal(number) — the float branch again. Ends
        // in "... comma nolla" here, so the 'a' rule always fires; the other
        // arms ("set"/"ot" -> "tavel", else "avel") are kept for shape parity
        // with the integer path.
        let cardinal = self.float_to_cardinal(f, precision)?;
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") || cardinal.ends_with("ot") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }

    /// `Num2Word_RM_VALLADER.to_ordinal` for a `Decimal` argument. `Decimal %
    /// 1` works, so a *fractional* Decimal takes the
    /// `float_to_words(ordinal=True)` branch and renders (reading
    /// `str(Decimal)`); a whole-valued one falls into the integer branches:
    /// `<= 20` dies on `ORDINAL_WORDS[<Decimal>]` (TypeError), `> 20`
    /// re-enters `to_cardinal(Decimal)` — TypeError below `10**6`,
    /// str-splitting above (ValueError for `Decimal("1E+20")`, whose str is
    /// scientific). `Decimal("-0.0") < 0` is False, so it is *not*
    /// minus-prefixed — it crashes in the table branch like `0.0` does.
    fn decimal_ordinal(&self, value: &BigDecimal) -> Result<String> {
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.decimal_ordinal(&(-value.clone()))?
            ));
        }
        if !value.is_integer() {
            // float_to_words(number, ordinal=True) over str(Decimal).
            let pre = value.with_scale(0).as_bigint_and_exponent().0;
            let prefix = self.to_ordinal(&pre)?;
            let s = crate::strnum::python_decimal_str(value);
            let float_part = match s.split_once('.') {
                Some((_, frac)) => frac.to_string(),
                // Scientific repr with no '.' (e.g. Decimal("5E-7")).
                None => return Err(N2WError::Index("list index out of range".to_string())),
            };
            let mut words: Vec<String> = Vec::with_capacity(float_part.len());
            for c in float_part.chars() {
                // int(c) — an 'E'/'+' from a scientific repr is ValueError.
                let d = c.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        c
                    ))
                })?;
                words.push(self.to_cardinal(&BigInt::from(d))?);
            }
            return Ok(format!(
                "{}{}{}",
                prefix,
                FLOAT_INFIX_WORD,
                words.join(" ")
            ));
        }
        if *value <= BigDecimal::from(20) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        let cardinal = self.decimal_to_cardinal(value)?;
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") || cardinal.ends_with("ot") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }
}

impl Default for LangRmVallader {
    fn default() -> Self {
        LangRmVallader::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRmVallader {
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
        Err(crate::lang_rm::unsupported("rm_vallader", "to='currency'"))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_vallader", "to='cheque'"))
    }

    /// `cards` / `maxval` / `merge` stay at their trait defaults: this class
    /// builds no card table and never calls `merge`. See module docs.
    fn to_cardinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // Python recurses on `-number`, so every helper below only ever
            // sees a non-negative value.
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.to_cardinal(&(-number))?
            ));
        }
        // The `isinstance(number, float)` branch sits here in Python; integer
        // input only, so it is skipped.
        if *number < BigInt::from(20) {
            Ok(card_word(number)?.to_string())
        } else if *number < BigInt::from(100) {
            self.tens_to_cardinal(number)
        } else if *number < BigInt::from(1000) {
            self.hundreds_to_cardinal(number)
        } else if *number < BigInt::from(1_000_000) {
            self.thousands_to_cardinal(number)
        } else {
            self.big_number_to_cardinal(number)
        }
    }

    fn to_ordinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // Not a real ordinal — see module quirk 2.
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.to_ordinal(&(-number))?
            ));
        }
        // The `number % 1 != 0` float branch sits here in Python; always false
        // for integer input.
        if *number <= BigInt::from(20) {
            return Ok(ord_word(number)?.to_string());
        }
        let cardinal = self.to_cardinal(number)?;
        // Python's `cardinal[-1]` indexes the last *character*. (It would
        // raise IndexError on an empty string, but `to_cardinal` never returns
        // one for number > 20.)
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") || cardinal.ends_with("ot") {
            "tavel"
        } else {
            "avel"
        };
        Ok(cardinal + suffix)
    }

    /// Python's class had no `to_ordinal_num` (AttributeError for every
    /// input), and no ordinal-numeral marker is attested for Romansh in the
    /// module or its tests, so the port raises a clear NotImplementedError
    /// rather than inventing one (#157).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_vallader", "to='ordinal_num'"))
    }

    /// Python's class had no `to_year` (AttributeError for every input).
    /// `Num2Word_Base.to_year` is the cardinal, which is what the port
    /// returns (#157).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal cardinal path. This class handles non-integers inline
    /// inside `to_cardinal` (there is no separate `to_cardinal_float` method and
    /// no `self.precision` attribute), so:
    ///
    ///   * `FloatValue::Float` -> the `isinstance(number, float)` branch, i.e.
    ///     `float_to_words` (see [`float_to_cardinal`]).
    ///   * `FloatValue::Decimal` -> in Python the integer dispatch, which
    ///     errored for every non-integer Decimal; now read like the int or the
    ///     float (#157).
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**: the class is
    /// a bare object with no `precision` attribute, so the dispatcher's
    /// `hasattr(converter, "precision")` guard is false and the kwarg is a
    /// no-op. `float_to_words` reads the digits from `str(value)` regardless.
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
                self.float_to_cardinal(*value, *precision)
            }
            FloatValue::Decimal { value, .. } => self.decimal_to_cardinal(value),
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

    /// Full `to_cardinal(float/Decimal)` routing. The gate is
    /// `isinstance(number, float)`, **not** `int(number) == number`, so a
    /// whole-valued float still renders through `float_to_words`
    /// (`1.0` -> "ün comma nolla") and a whole-valued Decimal crashed
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
                self.float_to_cardinal(*value, *precision)
            }
            FloatValue::Decimal { value, .. } => self.decimal_to_cardinal(value),
        }
    }

    /// `to_ordinal(float/Decimal)` — see [`LangRmVallader::float_ordinal`] /
    /// [`LangRmVallader::decimal_ordinal`] for the branch-by-branch mapping.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // A whole Decimal crashed on `ORDINAL_WORDS[<Decimal>]` in Python; it
        // now reads like the int (#157).
        if let FloatValue::Decimal { value: d, .. } = value {
            if d.is_integer() {
                return self.to_ordinal(&d.with_scale(0).as_bigint_and_exponent().0);
            }
        }
        match value {
            FloatValue::Float { value, precision } => self.float_ordinal(*value, *precision),
            FloatValue::Decimal { value, .. } => self.decimal_ordinal(value),
        }
    }

    /// See [`Lang::to_ordinal_num`] above (#157).
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_vallader", "to='ordinal_num'"))
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

    /// Python's class had no `to_fraction` (AttributeError for every n/d).
    /// Base's generic reading ("<n> <ordinal>s") is not Romansh, so this is a
    /// clear NotImplementedError instead (#157).
    fn to_fraction(&self, _numerator: &BigInt, _denominator: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_vallader", "to='fraction'"))
    }
}
