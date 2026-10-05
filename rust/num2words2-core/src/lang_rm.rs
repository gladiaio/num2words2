//! Port of `lang_RM.py` (Romansh — Rumantsch Grischun, the pan-regional
//! standard; the five idiom-specific variants live under `rm_puter`,
//! `rm_surmiran`, `rm_sursilv`, `rm_sutsilv`, `rm_vallader`).
//!
//! Registry check: `CONVERTER_CLASSES["rm"] == lang_RM.Num2Word_RM()`, so this
//! file ports `Num2Word_RM` — the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_RM` is derived from the `lang_IT`
//! template but, unlike `Num2Word_IT`, it subclasses **nothing at all** —
//! `Num2Word_RM.__mro__` is literally `(Num2Word_RM, object)`. It defines its
//! own `to_cardinal` driving `tens_/hundreds_/thousands_/big_number_to_cardinal`
//! and never touches cards/merge/splitnum. `cards`/`merge` therefore stay at
//! their trait defaults. The only ceiling is the 66-digit guard in
//! `big_number_to_cardinal`, which Python raises as `NotImplementedError`.
//! Fixed (gladiaio/num2words2#159): the port raises `OverflowError` ("abs(v)
//! must be less than 10^65.") and `maxval()` reports 10^65.
//!
//! # The empty base class is load-bearing — `to_ordinal_num` and `to_year`
//!
//! Because `Num2Word_RM` inherits from `object` rather than `Num2Word_Base`, it
//! has **no `to_ordinal_num` and no `to_year` at all**. `hasattr(c,
//! "to_ordinal_num")` and `hasattr(c, "to_year")` are both `False`, so the
//! dispatcher's `getattr(converter, "to_{}".format(to))` blows up with
//! `AttributeError` before any conversion runs. The corpus agrees: **all 90
//! `ordinal_num` rows and all 35 `year` rows are `err: "AttributeError"`** —
//! there is not one successful row for either mode. (The same emptiness is why
//! every `currency`/`cheque`/`fraction` row also errors, but those are out of
//! scope.)
//!
//! This is *not* the `lang_IT` situation, where `to_ordinal_num` exists and
//! dies later on a missing `errmsg_negord`. Here the method never existed, so
//! the failure is unconditional — it fires for `0` and `1` just as it does for
//! `-1`. Both trait defaults are overridden below to return the error for
//! **every** input rather than inheriting `base.rs`'s working implementations,
//! which would otherwise silently invent behaviour Python does not have.
//!
//! `base.rs` has no `N2WError::Attribute` variant, so — following the
//! precedent set by `lang_it.rs` — this is emitted as `N2WError::Type`
//! carrying a message that names `AttributeError` explicitly, letting the
//! integration layer remap it. See [`attribute_error`] and the port report's
//! `concerns`.
//!
//! # Faithfully reproduced Python quirks
//!
//! 1. **`tens_to_cardinal`'s `else` branch is dead code.** `STR_TENS` covers
//!    keys 2..=9 and the method is only ever reached from `to_cardinal` with
//!    `20 <= number < 100`, so `tens` is always 2..=9 and
//!    `CARDINAL_WORDS[tens][:-1] + "anta"` can never run. Kept verbatim anyway
//!    (see [`LangRm::tens_to_cardinal`]).
//! 2. **The `"_"` sentinel.** `empty_if_zero` maps "nulla" to `"_"` — a marker
//!    for "no following unit" — which `phonetic_contraction` then consumes via
//!    the `"tga_"` → `"tg"` rule (turning "ventga" into "ventg") before
//!    stripping any leftover `"_"`. The rule order is load-bearing and is
//!    preserved exactly.
//! 3. **The "e/ed" infix distribution looks arbitrary but is intentional.**
//!    `hundreds_to_cardinal` inserts "e" when `0 < tens <= 13` *or* `tens` is
//!    one of 15, 16, 20, 30 — note 14 and 17..19 are excluded. Python's comment
//!    says "distribution may seem unusual but it was reviewed by a native
//!    speaker". Reproduced as-is, gap and all.
//! 4. **`thousands_to_cardinal`'s infix condition is `hundreds <= 100`, not
//!    `< 100`.** So 1100 → "millietschient" (with "e") but 1101 →
//!    "millitschientedin" (no "e"). Verified against the corpus.
//! 5. **The `int("")` → `ValueError` path in `big_number_to_cardinal` is
//!    unreachable.** `exponent` is empty only when `length <= 3`, but the
//!    method is only called for `number >= 10**6` (`length >= 7`), so
//!    `exponent` is always at least 4 digits. Modelled as `N2WError::Value`
//!    rather than a panic, for fidelity.
//! 6. **`EXPONENT_PREFIXES[0]` ("nulla") is never used** — the smallest
//!    reachable `exponent_length` is 6, giving index 1 ("m"). The largest is 63
//!    (from a 65-digit input), giving index 10 ("dec"), which is exactly the
//!    last entry. So the 66-digit guard makes the table lookup provably
//!    in-range; it is still bounds-checked here rather than panicking.
//!
//! # Python semantics that matter here
//!
//! Every numword in this module is pure ASCII (Rumantsch Grischun needs no
//! diacritics for these forms), so byte and char indexing would coincide — but
//! `cardinal[-1]` still goes through `chars().last()` rather than a byte peek,
//! so the code stays correct if a form ever gains an accent.
//!
//! `str.replace` in Python is non-overlapping and left-to-right, which is
//! exactly Rust's `str::replace`, so the contraction/adaptation chains port
//! one-to-one. `set(exponent) != set("0")` is Python's idiom for "does the
//! exponent's value differ from zero", true for an empty string too — see
//! quirk 5.
//!
//! Negatives never reach the arithmetic: `to_cardinal`/`to_ordinal` strip the
//! sign and recurse first, so Rust's truncating `%` and Python's floor `%`
//! cannot disagree here.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};

const ZERO: &str = "nulla";

/// `MINUS_PREFIX_WORD`. The trailing space is the separator.
const MINUS_PREFIX_WORD: &str = "minus ";

const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "in",
    "dus",
    "trais",
    "quatter",
    "tschintg",
    "sis",
    "set",
    "otg",
    "nov",
    "diesch",
    "indesch",
    "dudesch",
    "tredesch",
    "quattordesch",
    "quindesch",
    "sedesch",
    "deschset",
    "deschdotg",
    "deschnov",
];

/// `ORDINAL_WORDS`, indices 0..=20 — note it runs one entry *longer* than
/// `CARDINAL_WORDS` (it includes "ventgavel" for 20), which is why
/// `to_ordinal`'s table branch is `number <= 20` while `to_cardinal`'s is
/// `number < 20`.
const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "emprim",
    "segund",
    "terz",
    "quart",
    "tschintgavel",
    "sisavel",
    "settavel",
    "otgavel",
    "novavel",
    "dieschavel",
    "indeschavel",
    "dudeschavel",
    "tredeschavel",
    "quattordeschavel",
    "quindeschavel",
    "sedeschavel",
    "deschsettavel",
    "deschdotgavel",
    "deschnovavel",
    "ventgavel",
];

/// `STR_TENS`, keys 2..=9. Index 0 and 1 are absent in Python; the `""` slots
/// here are never read (`tens_to_cardinal` only sees 2..=9).
///
/// Python's comment notes `"20" = "ventg"`: the table stores "ventga" and the
/// trailing "a" is removed by `phonetic_contraction`'s `"tga_"` rule when no
/// unit follows.
const STR_TENS: [&str; 10] = [
    "",
    "",
    "ventga",
    "trenta",
    "quaranta",
    "tschuncanta",
    "sessanta",
    "settanta",
    "otganta",
    "novanta",
];

/// `EXPONENT_PREFIXES`, used for extremely big numbers. Index 0 is unreachable
/// (see module quirk 6).
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

/// `phonetic_contraction`. `_` is a marker for "empty", i.e. no following unit.
///
/// The four rules run in this exact order and each is a whole-string pass, so
/// e.g. "ventgain" → "ventgin" (rule 1) and "ventga_" → "ventg" (rule 3).
/// Reordering them changes the output: rule 3 must see the `_` still in place.
fn phonetic_contraction(string: &str) -> String {
    string
        .replace("ain", "in") // ex. "trentain" -> "trentin"
        .replace("aotg", "otg") // ex. "quarantaotg" -> "quarantotg"
        .replace("tga_", "tg") // ex. "ventga" -> "ventg"
        .replace('_', "")
}

/// `adapt_hundred`: collective plural + e/ed phonotactic adaptation.
fn adapt_hundred(string: &str) -> String {
    string
        .replace("dustschient", "duatschient")
        .replace("traistschient", "traitschient")
        .replace("ein", "edin")
        .replace("eotg", "edotg")
}

/// `adapt_thousand`: collective plural + e/ed phonotactic adaptation.
fn adapt_thousand(string: &str) -> String {
    string
        .replace("dusmilli", "duamilli")
        .replace("traismilli", "traimilli")
        .replace("ein", "edin")
        .replace("eotg", "edotg")
}

/// `adapt_milliarda`: article gender agreement + e/ed phonotactic adaptation.
///
/// Python pads the string with a space on each side *before* replacing, so the
/// rules can anchor on word boundaries (" in milliarda " only matches a
/// standalone "in"). The padding is left in place — `big_number_to_cardinal`
/// strips it afterwards.
fn adapt_milliarda(string: &str) -> String {
    let padded = format!(" {} ", string);
    padded
        .replace(" in milliarda ", " ina milliarda ")
        .replace("dus milliardas", "duas milliardas")
        .replace(" e in", " ed in")
        .replace(" e otg", " ed otg")
}

/// `exponent_length_to_string`.
///
/// `exponent_length` is always a multiple of 3 (it is `length - (length % 3 or
/// 3)`). Python's comment: "If it's not true, then
/// `Num2Word_RM.big_number_to_cardinal` did something wrong."
///
/// The `// 6` split is what alternates the long-scale "-illiun" / "-illiarda"
/// pair: 10^6 "milliun", 10^9 "milliarda", 10^12 "billiun", 10^15 "billiarda".
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let idx = exponent_length / 6;
    // Provably in range given the 66-digit guard (module quirk 6); bounds
    // checked rather than panicking so a future guard change surfaces as the
    // IndexError Python would raise, not a Rust panic.
    let prefix = EXPONENT_PREFIXES
        .get(idx)
        .ok_or_else(|| N2WError::Index("list index out of range".to_string()))?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}illiun", prefix))
    } else {
        Ok(format!("{}illiarda", prefix))
    }
}

/// `omitt_if_zero` (Python's spelling of "omit" — kept as the doc name only).
fn omitt_if_zero(number_to_string: &str) -> &str {
    if number_to_string == ZERO {
        ""
    } else {
        number_to_string
    }
}

/// `empty_if_zero`: yields the `_` sentinel that `phonetic_contraction` eats.
fn empty_if_zero(number_to_string: &str) -> &str {
    if number_to_string == ZERO {
        "_"
    } else {
        number_to_string
    }
}

/// Python raised `AttributeError`, which `base.rs` cannot express. See the
/// module docs: emitted as `N2WError::Type` with a message naming the real
/// type, so the integration layer can remap it. Mirrors `lang_it.rs`.
fn attribute_error(msg: &str) -> N2WError {
    N2WError::Attribute(msg.to_string())
}

/// Python raised `ValueError` (`int("")`). Unreachable in practice — see
/// module quirk 5 — but modelled rather than panicked on.
fn value_error(msg: &str) -> N2WError {
    N2WError::Value(msg.to_string())
}

/// Whether Python's `str(f)` — the shortest round-trip repr — contains a `.`.
///
/// repr picks exponent form (no point) for finite non-zero magnitudes below
/// `1e-4` ("5e-05") or at/above `1e16` ("1e+16", "1e+20"); every other finite
/// float prints with a point ("0.0", "-0.0", "21.0", "2.675"). `float_to_words`
/// does `str(float_number).split('.')[1]`, so a pointless repr is Python's
/// `IndexError: list index out of range` — the corpus records exactly that for
/// `1e+16` and `1e+20`.
fn float_repr_has_point(f: f64) -> bool {
    f.is_finite() && (f == 0.0 || (f.abs() >= 1e-4 && f.abs() < 1e16))
}

/// Python's `int(f)` — truncate toward zero, exact at any magnitude (every
/// whole f64 is exactly representable as a BigInt). `int(inf)` is Python's
/// OverflowError, `int(nan)` its ValueError; neither is reachable from the
/// corpus but both are modelled.
fn float_trunc_int(f: f64) -> Result<BigInt> {
    if f.is_nan() {
        return Err(value_error("cannot convert float NaN to integer"));
    }
    num_traits::FromPrimitive::from_f64(f.trunc()).ok_or_else(|| {
        N2WError::Overflow("cannot convert float infinity to integer".to_string())
    })
}

pub struct LangRm;

impl LangRm {
    pub fn new() -> Self {
        LangRm
    }

    /// `tens_to_cardinal`. Only ever called with `20 <= number < 100`, so
    /// `tens` is 2..=9 and every lookup is in range.
    fn tens_to_cardinal(&self, number: u32) -> String {
        let tens = (number / 10) as usize;
        let units = (number % 10) as usize;
        // Python: `if tens in STR_TENS: prefix = STR_TENS[tens]` else
        // `CARDINAL_WORDS[tens][:-1] + "anta"`. STR_TENS covers 2..=9 and
        // `tens` can only be 2..=9 here, so the else branch is dead code
        // (module quirk 1) — kept for shape.
        let prefix = if (2..=9).contains(&tens) {
            STR_TENS[tens].to_string()
        } else {
            let base: String = {
                let w = CARDINAL_WORDS[tens];
                let n = w.chars().count();
                w.chars().take(n.saturating_sub(1)).collect()
            };
            format!("{}anta", base)
        };
        // We keep track of 0 using '_' -- removed in phonetic_contraction.
        let postfix = empty_if_zero(CARDINAL_WORDS[units]);
        phonetic_contraction(&format!("{}{}", prefix, postfix))
    }

    /// `hundreds_to_cardinal`. Only ever called with `100 <= number < 1000`.
    fn hundreds_to_cardinal(&self, number: u32) -> Result<String> {
        let hundreds = (number / 100) as usize;
        let tens = number % 100;
        let prefix = if hundreds != 1 {
            format!("{}tschient", CARDINAL_WORDS[hundreds])
        } else {
            "tschient".to_string()
        };
        let inner = self.cardinal(&BigInt::from(tens))?;
        let postfix = omitt_if_zero(&inner);
        // "e/ed" is inserted if tens <= 13 or = 15, 16, 20, 30. Distribution
        // may seem unusual but it was reviewed by a native speaker — note the
        // deliberate gaps at 14 and 17..19 (module quirk 3).
        let infix = if (tens > 0 && tens <= 13) || matches!(tens, 15 | 16 | 20 | 30) {
            "e"
        } else {
            ""
        };
        Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// `thousands_to_cardinal`. Only ever called with `1000 <= number < 10**6`.
    fn thousands_to_cardinal(&self, number: u32) -> Result<String> {
        let thousands = number / 1000;
        let hundreds = number % 1000;
        let prefix = if thousands != 1 {
            format!("{}milli", self.cardinal(&BigInt::from(thousands))?)
        } else {
            "milli".to_string()
        };
        let inner = self.cardinal(&BigInt::from(hundreds))?;
        let postfix = omitt_if_zero(&inner);
        // Python's comment says "e/ed is inserted if tens <= 100"; the code
        // tests `hundreds <= 100`, inclusive — hence 1100 keeps the "e" but
        // 1101 does not (module quirk 4).
        let infix = if hundreds <= 100 && !postfix.is_empty() {
            "e"
        } else {
            ""
        };
        Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// `big_number_to_cardinal`. Only ever called with `number >= 10**6`, and
    /// `number` is genuinely unbounded here (`BigInt`, never a fixed-width
    /// cast) up to the maxval (10^65) guard.
    fn big_number_to_cardinal(&self, number: &BigInt) -> Result<String> {
        // Python: digits = [c for c in str(number)]. `number` is known
        // non-negative at this point, so no '-' leaks into the digit list.
        let digits: Vec<char> = number.to_string().chars().collect();
        let length = digits.len();
        check_maxval(number, maxval_ceiling())?;
        // This is how many digits come before the "illion" term.
        //   tschient milliardas => 3
        //   diesch milliuns => 2
        //   ina milliarda => 1
        // `length % 3 or 3`: Python's `or` yields 3 when the modulo is 0.
        let predigits = if length % 3 != 0 { length % 3 } else { 3 };
        let multiplier: String = digits[..predigits].iter().collect();
        let exponent: String = digits[predigits..].iter().collect();
        let mut infix = exponent_length_to_string(exponent.len())?;

        // Python compares the *list* `multiplier == ["1"]`, i.e. exactly one
        // digit and it is '1' — equivalent to this string compare.
        let prefix = if multiplier == "1" {
            "in ".to_string()
        } else {
            let p = self.cardinal(
                &multiplier
                    .parse::<BigInt>()
                    .map_err(|_| value_error("invalid literal for int() with base 10"))?,
            )?;
            // Plural form: "milliun" -> " milliuns", "milliarda" ->
            // " milliardas". Unlike lang_IT this only appends an "s"; it does
            // not drop the final vowel first.
            infix = format!(" {}s", infix);
            p
        };

        // Python: `if set(exponent) != set("0")`, read as "does the value of
        // exponent equal 0?". set("0") == {'0'}, so the two are equal exactly
        // when `exponent` is non-empty and every char is '0'. An empty
        // `exponent` takes the *postfix* branch and feeds "" to int() —
        // unreachable (module quirk 5) but modelled as ValueError.
        let exponent_is_zero = !exponent.is_empty() && exponent.chars().all(|c| c == '0');
        let postfix = if !exponent_is_zero {
            let p = self.cardinal(
                &exponent
                    .parse::<BigInt>()
                    .map_err(|_| value_error("invalid literal for int() with base 10: ''"))?,
            )?;
            // We introduce "e" if 3-digits gap before next value, i.e. the
            // next group down is empty: 10**6+1 -> "in milliun ed in" (via
            // adapt_milliarda), but 1234567 -> "in milliun duatschient...".
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

    /// `Num2Word_RM.to_cardinal`, integer path only.
    fn cardinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // Python: MINUS_PREFIX_WORD + self.to_cardinal(-number).
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, self.cardinal(&-number)?));
        }
        // Python's `elif isinstance(number, float)` branch is out of scope:
        // integer input only.
        if number < &BigInt::from(20) {
            // Safe: 0 <= number < 20.
            Ok(CARDINAL_WORDS[number.to_usize().expect("0 <= number < 20")].to_string())
        } else if number < &BigInt::from(100) {
            Ok(self.tens_to_cardinal(number.to_u32().expect("20 <= number < 100")))
        } else if number < &BigInt::from(1000) {
            self.hundreds_to_cardinal(number.to_u32().expect("100 <= number < 1000"))
        } else if number < &BigInt::from(1_000_000) {
            self.thousands_to_cardinal(number.to_u32().expect("1000 <= number < 10**6"))
        } else {
            self.big_number_to_cardinal(number)
        }
    }

    /// `Num2Word_RM.to_ordinal`, integer path only.
    ///
    /// Note there is no `verify_ordinal` call anywhere in this class (there is
    /// no base class to inherit one from), so a negative ordinal does not
    /// raise: `to_ordinal(-1)` == "minus emprim".
    fn ordinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, self.ordinal(&-number)?));
        }
        // Python's `elif number % 1 != 0` float branch cannot fire on an int.

        // Table branch is `<= 20`, not `< 20`: ORDINAL_WORDS has 21 entries.
        if number <= &BigInt::from(20) {
            return Ok(ORDINAL_WORDS[number.to_usize().expect("0 <= number <= 20")].to_string());
        }

        let cardinal = self.cardinal(number)?;
        // `cardinal[-1] == 'a'` — last *char*. `cardinal` is non-empty here
        // (number > 20), so Python's IndexError on `[-1]` is unreachable.
        let suffix = if cardinal.chars().last() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            // "settantaset" -> "settantasettavel" (doubles the t).
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }

    /// `Num2Word_RM.float_to_words` (cardinal path) plus the sign handling
    /// `to_cardinal` performs first. Python's `to_cardinal(number)` strips a
    /// leading minus and recurses, then the `isinstance(number, float)` branch
    /// calls `float_to_words`, which reads the fractional digits from
    /// `str(float_number)` — the shortest-round-trip repr — and **not** from
    /// `base.float2tuple`. So the base float path's binary-artefact `< 0.01`
    /// heuristic never runs here: the digits are exactly what `repr(float)`
    /// shows. `2.675` -> `"675"` because `str(2.675) == "2.675"`, with no
    /// float2tuple detour.
    ///
    /// `precision` is `abs(Decimal(str(f)).as_tuple().exponent)`, i.e. the
    /// count of fractional digits in `repr(f)`. Formatting `f` to exactly that
    /// many places reproduces `str(f).split('.')[1]` — including the trailing
    /// `".0"` that Rust's own `{}` Display drops for whole floats (`str(1.0)`
    /// is `"1.0"`, so `"1.0" -> "0" -> "nulla"`).
    fn float_cardinal(&self, f: f64, precision: u32) -> Result<String> {
        if f < 0.0 {
            // Python: MINUS_PREFIX_WORD + self.to_cardinal(-number).
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.float_cardinal(-f, precision)?
            ));
        }
        // prefix = self.to_cardinal(int(float_number)); int() truncates to 0.
        // Computed *before* the split, exactly as Python does — for a huge
        // float the maxval OverflowError (#159) beats the IndexError below.
        let prefix = self.cardinal(&float_trunc_int(f)?)?;
        // float_part = str(float_number).split('.')[1]. A repr in exponent
        // form ("1e+16") has no '.' — Python's IndexError, corpus-confirmed.
        if !float_repr_has_point(f) {
            return Err(N2WError::Index("list index out of range".to_string()));
        }
        let formatted = format!("{:.*}", precision as usize, f);
        let float_part = formatted
            .split_once('.')
            .map(|(_, frac)| frac)
            .unwrap_or("");
        // postfix = " ".join(self.to_cardinal(int(c)) for c in float_part).
        let mut parts = Vec::with_capacity(float_part.len());
        for c in float_part.chars() {
            // int(c) — a non-digit char (only reachable via repr's exponent
            // notation, out of scope) is Python's ValueError.
            let d = c.to_digit(10).ok_or_else(|| {
                value_error(&format!("invalid literal for int() with base 10: '{}'", c))
            })?;
            parts.push(self.cardinal(&BigInt::from(d))?);
        }
        // prefix + FLOAT_INFIX_WORD (" comma ") + postfix.
        Ok(format!("{} comma {}", prefix, parts.join(" ")))
    }

    /// `Num2Word_RM.to_cardinal` for `Decimal` input. Romansh has no Decimal
    /// branch: `isinstance(number, float)` is `False` for a `Decimal`, so it
    /// falls through to the integer branches and crashes. Below `10**6` every
    /// branch bottoms out at `CARDINAL_WORDS[<Decimal>]`, a list indexed by a
    /// non-int -> `TypeError` (raised for *every* such value, integer-valued or
    /// not, and before anything is emitted). At or above `10**6`,
    /// `big_number_to_cardinal` reads `str(number)` and feeds the fractional
    /// slice to `int()` -> `ValueError`; an all-integer Decimal such as
    /// `Decimal("1000000")` has no '.', so it succeeds ("in milliun"), while
    /// `Decimal("1000000.00")` keeps its trailing-zero '.' and errors. All
    /// reproduced — see the port report's `concerns`.
    fn decimal_cardinal(&self, value: &BigDecimal, precision: u32) -> Result<String> {
        if value.is_negative() {
            // Python: MINUS_PREFIX_WORD + self.to_cardinal(-number).
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.decimal_cardinal(&value.abs(), precision)?
            ));
        }
        // number < 10**6: `CARDINAL_WORDS[number]` (a list indexed by a
        // Decimal) raises TypeError before any output, for every such value.
        if value < &BigDecimal::from(1_000_000i64) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
        // checked before Python's `len(str(number)) >= 66` string guard.
        check_maxval(&value.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        // number >= 10**6: big_number_to_cardinal(number) over str(number).
        self.big_number_decimal(value, precision)
    }

    /// `str(number)` for a non-negative `Decimal`, in Python's plain (non-`E`)
    /// form — always used here because reachable values are `>= 10**6` with a
    /// non-positive exponent. Integer digits, then a point and exactly
    /// `precision` fractional digits, trailing zeros kept: `Decimal("1000000")`
    /// -> `"1000000"`, `Decimal("1000000.00")` -> `"1000000.00"`.
    fn decimal_str(&self, value: &BigDecimal, precision: u32) -> String {
        let (mantissa, _scale) = value.with_scale(precision as i64).as_bigint_and_exponent();
        let digits = mantissa.abs().to_string();
        if precision == 0 {
            return digits;
        }
        let p = precision as usize;
        // Values reaching here are >= 10**6, so the integer part is never
        // empty; the pad is defensive so the split point is always valid.
        let padded = if digits.len() <= p {
            format!("{}{}", "0".repeat(p + 1 - digits.len()), digits)
        } else {
            digits
        };
        let split = padded.len() - p;
        format!("{}.{}", &padded[..split], &padded[split..])
    }

    /// `Num2Word_RM.big_number_to_cardinal` reading `str(number)` for a
    /// `Decimal`. Same shape as the integer [`LangRm::big_number_to_cardinal`],
    /// but the digit list can carry a '.', which then reaches `int()` and
    /// raises `ValueError`. Kept separate so the corpus-verified integer method
    /// is left untouched.
    fn big_number_decimal(&self, value: &BigDecimal, precision: u32) -> Result<String> {
        let s = self.decimal_str(value, precision);
        let digits: Vec<char> = s.chars().collect();
        let length = digits.len();
        // Only reachable below maxval when the fraction digits push str(number)
        // past 65 characters; magnitudes past maxval were rejected above (#159).
        if length >= 66 {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        let predigits = if length % 3 != 0 { length % 3 } else { 3 };
        let multiplier: String = digits[..predigits].iter().collect();
        let exponent: String = digits[predigits..].iter().collect();
        let mut infix = exponent_length_to_string(exponent.len())?;

        // For a Decimal >= 10**6 the integer part is >= 7 digits, so the
        // 1..=3-char multiplier is always pure digits and parses cleanly.
        let prefix = if multiplier == "1" {
            "in ".to_string()
        } else {
            let m = multiplier.parse::<BigInt>().map_err(|_| {
                value_error(&format!(
                    "invalid literal for int() with base 10: '{}'",
                    multiplier
                ))
            })?;
            let p = self.cardinal(&m)?;
            infix = format!(" {}s", infix);
            p
        };

        // set(exponent) != set("0"): true unless exponent is a non-empty
        // all-'0' run (the integer-valued Decimal case). When a '.' survives in
        // the exponent slice, int() below is Python's ValueError.
        let exponent_is_zero = !exponent.is_empty() && exponent.chars().all(|c| c == '0');
        let postfix = if !exponent_is_zero {
            let e = exponent.parse::<BigInt>().map_err(|_| {
                value_error(&format!(
                    "invalid literal for int() with base 10: '{}'",
                    exponent
                ))
            })?;
            let p = self.cardinal(&e)?;
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

    /// `Num2Word_RM.to_ordinal` for a `float` argument:
    ///
    /// ```python
    /// if number < 0:  return MINUS_PREFIX_WORD + self.to_ordinal(-number)
    /// elif number % 1 != 0:  return self.float_to_words(number, ordinal=True)
    /// elif number <= 20:  return ORDINAL_WORDS[number]   # float index -> TypeError
    /// else: cardinal = self.to_cardinal(number)  # float branch, "... comma nulla"
    /// ```
    ///
    /// Corpus-confirmed quirks, all reproduced:
    ///   * a *fractional* float works: `2.5` -> "segund comma tschintg"
    ///     (ordinal prefix over `int(2.5)`, cardinal digit words after);
    ///   * a *whole* float `<= 20` (incl. `-0.0`/`0.0`: `-0.0 < 0` is False,
    ///     `-0.0 % 1 == 0`) dies on `ORDINAL_WORDS[<float>]` -> TypeError;
    ///   * a whole float `> 20` renders its float *cardinal* ("ventgin comma
    ///     nulla") and then gets the ordinal suffix: "ventgin comma nullavel";
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
            let prefix = self.ordinal(&float_trunc_int(f)?)?;
            if !float_repr_has_point(f) {
                return Err(N2WError::Index("list index out of range".to_string()));
            }
            let formatted = format!("{:.*}", precision as usize, f);
            let float_part = formatted
                .split_once('.')
                .map(|(_, frac)| frac)
                .unwrap_or("");
            let mut parts = Vec::with_capacity(float_part.len());
            for c in float_part.chars() {
                let d = c.to_digit(10).ok_or_else(|| {
                    value_error(&format!("invalid literal for int() with base 10: '{}'", c))
                })?;
                parts.push(self.cardinal(&BigInt::from(d))?);
            }
            return Ok(format!("{} comma {}", prefix, parts.join(" ")));
        }
        if f <= 20.0 {
            // ORDINAL_WORDS[<float>] — a float is not a valid list index.
            return Err(N2WError::Type(
                "list indices must be integers or slices, not float".to_string(),
            ));
        }
        // cardinal = self.to_cardinal(number) — the float branch again.
        let cardinal = self.float_cardinal(f, precision)?;
        // Ends in "... comma nulla" here, so the 'a' rule always fires; the
        // other arms are kept for shape parity with the integer path.
        let suffix = if cardinal.chars().last() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }

    /// `Num2Word_RM.to_ordinal` for a `Decimal` argument. `Decimal % 1` works,
    /// so a *fractional* Decimal takes the `float_to_words(ordinal=True)`
    /// branch and renders (reading `str(Decimal)`); a whole-valued one falls
    /// into the integer branches: `<= 20` dies on `ORDINAL_WORDS[<Decimal>]`
    /// (TypeError), `> 20` re-enters `to_cardinal(Decimal)` — TypeError below
    /// `10**6`, str-splitting above (ValueError for `Decimal("1E+20")`, whose
    /// str is scientific). `Decimal("-0.0") < 0` is False, so it is *not*
    /// minus-prefixed — it crashes in the table branch like `0.0` does.
    fn decimal_ordinal(&self, value: &BigDecimal, precision: u32) -> Result<String> {
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.decimal_ordinal(&value.abs(), precision)?
            ));
        }
        if !value.is_integer() {
            // float_to_words(number, ordinal=True) over str(Decimal).
            let pre = value.with_scale(0).as_bigint_and_exponent().0;
            let prefix = self.ordinal(&pre)?;
            let s = crate::strnum::python_decimal_str(value);
            let float_part = match s.split_once('.') {
                Some((_, frac)) => frac.to_string(),
                // Scientific repr with no '.' (e.g. Decimal("5E-7")).
                None => return Err(N2WError::Index("list index out of range".to_string())),
            };
            let mut parts = Vec::with_capacity(float_part.len());
            for c in float_part.chars() {
                // int(c) — an 'E'/'+' from a scientific repr is ValueError.
                let d = c.to_digit(10).ok_or_else(|| {
                    value_error(&format!("invalid literal for int() with base 10: '{}'", c))
                })?;
                parts.push(self.cardinal(&BigInt::from(d))?);
            }
            return Ok(format!("{} comma {}", prefix, parts.join(" ")));
        }
        if value <= &BigDecimal::from(20) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        let cardinal = self.decimal_cardinal(value, precision)?;
        let suffix = if cardinal.chars().last() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }
}

impl Default for LangRm {
    fn default() -> Self {
        LangRm::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRm {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    // Num2Word_RM and its variants define no to_currency / to_cheque
    // at all, so Python raises AttributeError on attribute lookup —
    // not the NotImplementedError the trait default would give.
    fn to_currency(
        &self,
        _val: &crate::currency::CurrencyValue,
        _currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        Err(N2WError::Attribute(format!(
            "'{}' object has no attribute 'to_currency'",
            "Num2Word_RM",
        )))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(N2WError::Attribute(format!(
            "'{}' object has no attribute 'to_cheque'",
            "Num2Word_RM",
        )))
    }

    // cards/maxval/merge stay at their trait defaults: Python never builds
    // self.cards for this class (it has no base class at all), so
    // splitnum/clean/merge are unreachable and there is no MAXVAL overflow
    // check. See the module docs.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.ordinal(value)
    }

    /// `Decimal('-0.0')`. A float `-0.0` renders through `float_to_words`
    /// ("nulla comma nulla"), but a `Decimal('-0.0')` is not a `float`:
    /// `to_cardinal` falls into the integer ladder and `CARDINAL_WORDS[<Decimal>]`
    /// raises **TypeError** (`Decimal('-0.0') < 0` is False, so it is not even
    /// minus-prefixed — it crashes in the `< 20` table branch). BigDecimal
    /// cannot carry the sign, so this is served here rather than by the
    /// `Float{-0.0}` demotion. The other modes coincide with that demotion —
    /// `ordinal` TypeErrors either way, `ordinal_num`/`year` AttributeError —
    /// so they return `None`.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            "cardinal" => Some(Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ))),
            _ => None,
        }
    }

    /// **Does not exist on `Num2Word_RM`.** The class has no base, so the
    /// dispatcher's `getattr(converter, "to_ordinal_num")` raises
    /// `AttributeError` for *every* input — the corpus has 90 such rows and
    /// zero successes. Overriding the trait default (which would return the
    /// digits) is what keeps that parity.
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'to_ordinal_num'",
        ))
    }

    /// **Does not exist on `Num2Word_RM`.** Same reasoning as
    /// [`Lang::to_ordinal_num`] above: `AttributeError` for every input, 35
    /// corpus rows, zero successes. The trait default would have delegated to
    /// `to_cardinal` and invented a year reading Python does not have.
    fn to_year(&self, _value: &BigInt) -> Result<String> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'to_year'",
        ))
    }

    /// **`Num2Word_RM` has no `to_cardinal_float`** (no base class), so the
    /// dispatcher hands floats and Decimals straight to `to_cardinal`. This
    /// override reproduces both arms: a `float` renders through
    /// `float_to_words` ([`LangRm::float_cardinal`]); a `Decimal` falls into
    /// the integer branches and crashes ([`LangRm::decimal_cardinal`]).
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored:
    /// `num2words` only applies it when `hasattr(converter, "precision")`, and
    /// this class has no `precision` attribute — verified against the live
    /// interpreter (`precision=1` on `2.675` still yields the full repr).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.float_cardinal(*value, *precision),
            FloatValue::Decimal { value, precision } => self.decimal_cardinal(value, *precision),
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing. The gate is
    /// `isinstance(number, float)`, **not** `int(number) == number`, so a
    /// whole-valued float still renders through `float_to_words`
    /// (`1.0` -> "in comma nulla") and a whole-valued Decimal still crashes
    /// through the integer ladder (`Decimal("5.0")` -> TypeError). The base
    /// default's whole-value -> int-path shortcut is exactly wrong here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.float_cardinal(*value, *precision),
            FloatValue::Decimal { value, precision } => self.decimal_cardinal(value, *precision),
        }
    }

    /// `to_ordinal(float/Decimal)` — see [`LangRm::float_ordinal`] /
    /// [`LangRm::decimal_ordinal`] for the branch-by-branch mapping.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.float_ordinal(*value, *precision),
            FloatValue::Decimal { value, precision } => self.decimal_ordinal(value, *precision),
        }
    }

    /// **Does not exist on `Num2Word_RM`** — same AttributeError as the
    /// integer [`Lang::to_ordinal_num`] override; the float/Decimal entry
    /// would otherwise echo the repr.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'to_ordinal_num'",
        ))
    }

    /// **Does not exist on `Num2Word_RM`** — same AttributeError as the
    /// integer [`Lang::to_year`] override.
    fn year_float_entry(&self, _value: &FloatValue) -> Result<String> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'to_year'",
        ))
    }


    /// **Does not exist on `Num2Word_RM`.** The dispatcher does
    /// `converter.str_to_number(value)` for every string input, and this
    /// bare class has no such attribute — so *every* `num2words("...")`
    /// call raises AttributeError before any parsing ("5", "1.5", "abc",
    /// "Infinity" alike). Corpus: all 78 string rows are AttributeError.
    fn str_to_number(&self, _s: &str) -> Result<crate::strnum::ParsedNumber> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'str_to_number'",
        ))
    }

    /// **Does not exist on `Num2Word_RM`.** `to_fraction` is a
    /// `Num2Word_Base` method (issue #584) and this class has no base, so
    /// the attribute lookup fails for every n/d — including `1/0`, where
    /// Python never reaches the ZeroDivision check. Corpus: all 25
    /// fraction2 rows are AttributeError.
    fn to_fraction(&self, _numerator: &BigInt, _denominator: &BigInt) -> Result<String> {
        Err(attribute_error(
            "'Num2Word_RM' object has no attribute 'to_fraction'",
        ))
    }
}
