//! Port of `lang_RM_PUTER.py` (Romansh, Puter idiom).
//!
//! Shape: **self-contained**. `Num2Word_RM_PUTER` subclasses **nothing** — it
//! is a bare `class Num2Word_RM_PUTER:` with `def __init__(self): pass`. There
//! is no `Num2Word_Base` in its ancestry at all, which drives three facts the
//! rest of this file depends on:
//!
//!   * `self.cards` / `self.MAXVAL` / `merge` never exist, so `cards` and
//!     `merge` stay at their trait defaults and are never reached.
//!   * The only ceiling is `big_number_to_cardinal`'s explicit
//!     `raise NotImplementedError("The given number is too large.")` at
//!     >= 66 digits — an `OverflowError` in the port since
//!     gladiaio/num2words2#159, and `maxval()` reports 10^65. See "Ceiling".
//!   * **`to_ordinal_num` and `to_year` do not exist** in Python, so calling
//!     either raised `AttributeError`. Fixed — see "Missing methods" below.
//!
//! The module header says "Based on lang_IT template from Filippo Costa", and
//! the structure (phonetic contraction + `adapt_*` surface rules +
//! `EXPONENT_PREFIXES`) mirrors `lang_IT.py` closely. It does **not** share
//! IT's class hierarchy, though, so none of IT's inherited methods come along.
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
//! This is a port, not a rewrite. All of the following are verified against
//! the interpreter and are preserved verbatim:
//!
//! 1. **`adapt_thousand` handles "eot" where `adapt_hundred` handles "eoch".**
//!    `adapt_hundred` rewrites `"eoch"` → `"edoch"`, but `adapt_thousand`
//!    rewrites `"eot"` → `"edot"` — and **no word in this language starts with
//!    "ot"** ("eight" is `"och"`). The rule is inherited dead from the Italian
//!    template ("otto"), so the thousands path never gets its `ed` liaison:
//!      * `to_cardinal(108)`  == `"tschientedoch"`  (hundreds: liaison fires)
//!      * `to_cardinal(1008)` == `"millieoch"`      (thousands: liaison does
//!        **not** fire — this looks like a typo for "eoch" but is what ships)
//!      * `to_cardinal(2008)` == `"duamillieoch"`, `to_cardinal(10008)` ==
//!        `"deschmillieoch"`, all likewise un-liaised.
//!    `adapt_milliard` covers `" e och"` → `" ed och"` correctly, so
//!    `to_cardinal(1000008)` == `"ün milliun ed och"`. Only the thousands rung
//!    is affected. Not fixed here — see [`adapt_thousand`].
//!
//! 2. **`adapt_thousand`'s collective-plural rules are substring rules, not
//!    multiplier rules.** `"traismilli"` → `"trajamilli"` fires wherever that
//!    substring appears, including when "trais" is the *units digit of the
//!    thousands multiplier* rather than the multiplier itself:
//!    `to_cardinal(123456)` == `"tschientvainchatrajamilli..."` — the "trais"
//!    of "tschientvainchatrais" (123) is rewritten to "traja". This is
//!    reproduced exactly (and the corpus confirms it).
//!
//! 3. **`tens_to_cardinal`'s `else` branch is unreachable.** It is only called
//!    for `20 <= number < 100`, so `tens` is always in 2..=9 and always a
//!    `STR_TENS` key; `CARDINAL_WORDS[tens][:-1] + "aunta"` never runs. Ported
//!    verbatim anyway.
//!
//! 4. **`STR_TENS[2]` is `"vaincha"`, not `"vainch"`.** The trailing "a" is a
//!    scaffold: `phonetic_contraction` restores the surface form via the
//!    `"vaincha_"` → `"vainch"` rule, where `_` is the empty-units marker.
//!    Rule *order* is load-bearing — `"aün"` → `"ün"` runs first, so
//!    `"vainchaün"` (21) contracts to `"vainchün"` and the `"vaincha_"` rule
//!    never sees it. See [`phonetic_contraction`].
//!
//! 5. **The "e" liaison distribution in `hundreds_to_cardinal` is irregular**:
//!    `tens in 1..=13` or `tens in {15, 16, 20, 30}`. 14 is excluded, 17/18/19
//!    are excluded. The Python comment says "distribution may seem unusual but
//!    it was reviewed by a native speaker", so it is deliberate, not a bug:
//!    `to_cardinal(116)` == `"tschientesaidesch"` but `to_cardinal(118)` ==
//!    `"tschientdischdoch"`. Ported as-is.
//!
//! # Ceiling
//!
//! `big_number_to_cardinal` raises at `len(str(n)) >= 66` (`OverflowError`
//! since #159; Python raises `NotImplementedError`)
//! — i.e. the largest representable value is 65 digits. That bound is exactly
//! what keeps `EXPONENT_PREFIXES` (11 entries, 0..=10) in range: 65 digits →
//! `predigits = 2` → `exponent_length = 63` → `63 // 6 == 10`, the last index.
//! So the `IndexError` modelled in [`exponent_length_to_string`] is
//! unreachable. `to_cardinal(10**64)` == `"ün decilliard"`;
//! `to_cardinal(10**65)` raises `OverflowError`. Values are `BigInt`
//! throughout — 65 digits does not fit a `u128`.
//!
//! # Python semantics that matter here
//!
//! Strings genuinely contain "ü", so **all indexing and slicing is
//! character-based, never byte-based**: `to_ordinal` inspects `cardinal[-1]`
//! and may drop it with `cardinal[:-1]`, and a byte-wise `[..len-1]` would
//! split "ü" in half. See [`drop_last_char`] and the `to_ordinal` impl.
//!
//! # The float / Decimal cardinal path
//!
//! `to_cardinal` dispatches non-integer input inline. A **float** hits the
//! `isinstance(number, float)` branch → `float_to_words`, spelled out in
//! [`LangRmPuter::float_to_words`]: `to_cardinal(int(v))`, then `" comma "`,
//! then each fractional digit of `str(v)` spelled separately. A **Decimal** is
//! not a float, so it skips that branch and falls through the integer dispatch,
//! where it crashes (`TypeError`/`ValueError`) or, for a whole number
//! `>= 1e6`, succeeds — see [`LangRmPuter::decimal_to_cardinal`]. Both are
//! reached through the overridden `to_cardinal_float` at the bottom of the file
//! (the generic `base.rs` float path would double the spaces around " comma ").
//!
//! # Still out of scope
//!
//! The `number % 1 != 0` branch of `to_ordinal` (ordinal of a non-integer) is
//! not modelled — this phase is cardinal-only, and no `ordinal` float corpus
//! rows exist for this language. Currency/cheque/fraction do not exist on this
//! class at all; they raise `NotImplementedError` (#157).

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive};

// Globals — mirroring the Python module-level tables.

const ZERO: &str = "nolla";

const MINUS_PREFIX_WORD: &str = "minus ";

/// `FLOAT_INFIX_WORD` — the separator `float_to_words` puts between the
/// integer part and the spelled-out fractional digits. Note the surrounding
/// spaces are part of the constant: the result is `prefix + " comma " +
/// postfix`, so exactly one space falls on each side. (The generic
/// `pointword()` route in `base.rs` would title-case this and *also* join it
/// with spaces, yielding `nolla  comma  tschinch` with doubled spaces — which
/// is why this class needs its own `to_cardinal_float`.)
const FLOAT_INFIX_WORD: &str = " comma ";

/// `CARDINAL_WORDS`, indices 0..=19.
const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "ün",
    "duos",
    "trais",
    "quatter",
    "tschinch",
    "ses",
    "set",
    "och",
    "nouv",
    "desch",
    "ündesch",
    "dudesch",
    "tredesch",
    "quattordesch",
    "quindesch",
    "saidesch",
    "dischset",
    "dischdoch",
    "dischnouv",
];

/// `ORDINAL_WORDS`, indices 0..=20 (note it runs one further than
/// `CARDINAL_WORDS`, which is why `to_ordinal` guards with `<= 20`).
const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "prüm",
    "seguond",
    "terz",
    "quart",
    "tschinchevel",
    "sesevel",
    "settevel",
    "ochevel",
    "nouvevel",
    "deschevel",
    "ündeschevel",
    "dudeschevel",
    "tredeschevel",
    "quattordeschevel",
    "quindeschevel",
    "saideschevel",
    "dischsettevel",
    "dischdochevel",
    "dischnouvevel",
    "vainchevel",
];

/// `EXPONENT_PREFIXES`, indices 0..=10. Index 0 is `ZERO` and unreachable
/// (`exponent_length // 6 == 0` would need `exponent_length < 6`, but
/// `big_number_to_cardinal` only runs for >= 10**6, giving >= 6).
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "och", "nov", "dec",
];

/// `STR_TENS`, a dict with keys 2..=9. Returns `None` for a missing key, which
/// is how the (unreachable) `else` branch of `tens_to_cardinal` is selected —
/// Python tests `if tens in STR_TENS`, so a miss is not a `KeyError`.
fn str_tens(tens: usize) -> Option<&'static str> {
    match tens {
        2 => Some("vaincha"), // surface form restored by phonetic_contraction
        3 => Some("trenta"),
        4 => Some("quaraunta"),
        5 => Some("tschinquaunta"),
        6 => Some("sesaunta"),
        7 => Some("settaunta"),
        8 => Some("ochaunta"),
        9 => Some("nonaunta"),
        _ => None,
    }
}

// --- Python exception encoding -------------------------------------------

/// Python raised `IndexError` (a list index out of range). Every call site
/// below is provably unreachable given `to_cardinal`'s range dispatch; these
/// exist so an out-of-range access reproduces Python's exception type instead
/// of panicking.
fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

/// Python raised `ValueError` — `int("")`. Unreachable: `big_number_to_cardinal`
/// only runs for >= 10**6 (>= 7 digits), so `exponent` is never empty.
fn value_error(s: &str) -> N2WError {
    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
}

/// Python's `int(s)` for the all-digit strings this module builds. Leading
/// zeros are fine (`int("000001") == 1`); only `""` can fail here.
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10).ok_or_else(|| value_error(s))
}

/// Whether Python's `str(f)` — the shortest round-trip repr — contains a `.`.
///
/// repr picks exponent form (no point) for finite non-zero magnitudes below
/// `1e-4` ("5e-05") or at/above `1e16` ("1e+16", "1e+20"); every other finite
/// float prints with a point ("0.0", "21.0"). `float_to_words` does
/// `str(float_number).split('.')[1]`, so a pointless repr is Python's
/// `IndexError: list index out of range` — corpus-confirmed for `1e+16` and
/// `1e+20`. This must be an explicit repr test: the binding's `precision`
/// for `1e+16` is 16 (`abs(Decimal("1e+16").as_tuple().exponent)`), so the
/// `{:.16}` reconstruction *would* carry a point Python's repr does not have.
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

// --- Character-safe slicing ----------------------------------------------

/// Python's `s[:-1]` — drop the last **character**. Byte slicing would split
/// the "ü" in e.g. "vainchün".
fn drop_last_char(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    chars[..chars.len() - 1].iter().collect()
}

/// `CARDINAL_WORDS[i]` with Python's `IndexError` on a miss.
///
/// Note: a *negative* Python index would wrap rather than raise, but no
/// negative ever reaches here — `to_cardinal` strips the sign before
/// dispatching on magnitude.
fn cardinal_word(i: usize) -> Result<&'static str> {
    CARDINAL_WORDS
        .get(i)
        .copied()
        .ok_or_else(|| index_error("list index out of range"))
}

/// `ORDINAL_WORDS[i]` with Python's `IndexError` on a miss.
fn ordinal_word(i: usize) -> Result<&'static str> {
    ORDINAL_WORDS
        .get(i)
        .copied()
        .ok_or_else(|| index_error("list index out of range"))
}

/// A `BigInt` used as a list index. Callers have already bounded the value
/// (`< 20`, `< 100`, `< 1000`), so `None` is unreachable; it maps to the
/// `IndexError` Python would raise.
fn as_index(n: &BigInt) -> Result<usize> {
    n.to_usize()
        .ok_or_else(|| index_error("list index out of range"))
}

// --- Utils (module-level functions in Python) -----------------------------

/// Port of `phonetic_contraction`. `_` marks "no following unit".
///
/// The four rules run **in this order** and the order is load-bearing:
/// `"aün"` → `"ün"` fires before `"vaincha_"` → `"vainch"`, so 21
/// ("vainchaün") contracts via the first rule and never reaches the third.
///
/// * 31: "trentaün"    → "trentün"
/// * 88: "ochauntaoch" → "ochauntoch"
/// * 20: "vaincha_"    → "vainch"
/// * 30: "trenta_"     → "trenta"
fn phonetic_contraction(s: &str) -> String {
    s.replace("aün", "ün")
        .replace("aoch", "och")
        .replace("vaincha_", "vainch")
        .replace("_", "")
}

/// Port of `adapt_hundred`: collective plural + e/ed phonotactic adaptation.
fn adapt_hundred(s: &str) -> String {
    s.replace("duostschient", "duatschient")
        .replace("traistschient", "trajatschient")
        .replace("eün", "edün")
        .replace("eoch", "edoch")
}

/// Port of `adapt_thousand`: collective plural + e/ed phonotactic adaptation.
///
/// The last rule is `"eot"` → `"edot"`, **not** `"eoch"` → `"edoch"`. No word
/// here begins with "ot", so it is dead — inherited from the Italian template
/// ("otto"). Consequence: `to_cardinal(1008)` == `"millieoch"` while
/// `to_cardinal(108)` == `"tschientedoch"`. See module quirk 1; not fixed.
fn adapt_thousand(s: &str) -> String {
    s.replace("duosmilli", "duamilli")
        .replace("traismilli", "trajamilli")
        .replace("eün", "edün")
        .replace("eot", "edot")
}

/// Port of `adapt_milliard`: article gender agreement + e/ed adaptation.
///
/// Python pads with a space on each side so the `" e ün"` / `" e och"` rules
/// can match at a word boundary; the caller then `.strip()`s the padding back
/// off. Both are reproduced (see `big_number_to_cardinal`'s trailing `trim`).
fn adapt_milliard(s: &str) -> String {
    let padded = format!(" {} ", s);
    padded.replace(" e ün", " ed ün").replace(" e och", " ed och")
}

/// Port of `exponent_length_to_string`.
///
/// `exponent_length` is always a multiple of 3 (guaranteed by
/// `big_number_to_cardinal`'s `predigits` split). `// 6 == 0` and `% 6 == 0`
/// therefore pick "illiun" for 6/12/18... and "illiard" for 9/15/21...
///
/// The `IndexError` is unreachable — see the module docs' "Ceiling".
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let prefix = EXPONENT_PREFIXES
        .get(exponent_length / 6)
        .copied()
        .ok_or_else(|| index_error("list index out of range"))?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}illiun", prefix))
    } else {
        Ok(format!("{}illiard", prefix))
    }
}

/// Port of `omitt_if_zero` (sic — one "t", two "t"s; Python's spelling).
fn omitt_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        String::new()
    } else {
        number_to_string.to_string()
    }
}

/// Port of `empty_if_zero`: the `_` marker consumed by `phonetic_contraction`.
fn empty_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        "_".to_string()
    } else {
        number_to_string.to_string()
    }
}

// --- Main class -----------------------------------------------------------

pub struct LangRmPuter;

impl Default for LangRmPuter {
    fn default() -> Self {
        Self::new()
    }
}

impl LangRmPuter {
    pub fn new() -> Self {
        LangRmPuter
    }

    /// Python's `self.to_cardinal`, as an inherent method so the internal
    /// recursion (`hundreds_to_cardinal` → `to_cardinal` → ...) resolves
    /// exactly as Python's does. No subclass exists, so dynamic dispatch and
    /// this direct call are equivalent.
    fn card(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // Python: MINUS_PREFIX_WORD + self.to_cardinal(-number)
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, self.card(&-number)?));
        }
        // `card` is the integer path: Python's `isinstance(number, float)`
        // branch is handled up in `to_cardinal_float` → `float_to_words`, which
        // then calls back here with the integer part.
        if number < &BigInt::from(20) {
            Ok(cardinal_word(as_index(number)?)?.to_string())
        } else if number < &BigInt::from(100) {
            self.tens_to_cardinal(number)
        } else if number < &BigInt::from(1000) {
            self.hundreds_to_cardinal(number)
        } else if number < &BigInt::from(1_000_000) {
            self.thousands_to_cardinal(number)
        } else {
            self.big_number_to_cardinal(number)
        }
    }

    /// Port of `tens_to_cardinal`. Reached only for `20 <= number < 100`.
    fn tens_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let tens = number.div_floor(&BigInt::from(10));
        let units = number.mod_floor(&BigInt::from(10));
        let tens_i = as_index(&tens)?;
        let units_i = as_index(&units)?;

        let prefix = match str_tens(tens_i) {
            Some(p) => p.to_string(),
            // Unreachable: tens is always 2..=9 here. See module quirk 3.
            None => format!("{}aunta", drop_last_char(cardinal_word(tens_i)?)),
        };
        // 0 is tracked with '_' and removed in phonetic_contraction.
        let postfix = empty_if_zero(cardinal_word(units_i)?);
        Ok(phonetic_contraction(&format!("{}{}", prefix, postfix)))
    }

    /// Port of `hundreds_to_cardinal`. Reached only for `100 <= number < 1000`.
    fn hundreds_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let hundreds = number.div_floor(&BigInt::from(100));
        let tens = number.mod_floor(&BigInt::from(100));

        let mut prefix = "tschient".to_string();
        if hundreds != BigInt::from(1) {
            prefix = format!("{}tschient", cardinal_word(as_index(&hundreds)?)?);
        }
        let postfix = omitt_if_zero(&self.card(&tens)?);

        // "e/ed" is inserted if tens <= 13 or = 15, 16, 20, 30.
        // Irregular on purpose — see module quirk 5.
        let mut infix = "";
        let t = as_index(&tens)?;
        if (t > 0 && t <= 13) || matches!(t, 15 | 16 | 20 | 30) {
            infix = "e";
        }
        Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// Port of `thousands_to_cardinal`. Reached only for
    /// `1000 <= number < 1_000_000`.
    fn thousands_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let thousands = number.div_floor(&BigInt::from(1000));
        let hundreds = number.mod_floor(&BigInt::from(1000));

        let mut prefix = "milli".to_string();
        if thousands != BigInt::from(1) {
            prefix = format!("{}milli", self.card(&thousands)?);
        }
        let postfix = omitt_if_zero(&self.card(&hundreds)?);

        // Python comment says "e/ed is inserted if tens <= 100"; the code
        // actually tests the whole `hundreds` remainder, which is what runs.
        let mut infix = "";
        if hundreds <= BigInt::from(100) && !postfix.is_empty() {
            infix = "e";
        }
        Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// Port of `big_number_to_cardinal`. Reached only for `number >= 10**6`,
    /// so `digits` is at least 7 long and never carries a sign.
    fn big_number_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let digits: Vec<char> = number.to_string().chars().collect();
        let length = digits.len();
        check_maxval(number, maxval_ceiling())?;

        // How many digits come before the "illion" term:
        //   tschient milliards => 3 / desch milliuns => 2 / ün milliard => 1
        // Python: `length % 3 or 3` — 0 is falsy, so a multiple of 3 gives 3.
        let predigits = if length % 3 == 0 { 3 } else { length % 3 };
        let multiplier: Vec<char> = digits[..predigits].to_vec();
        let exponent: Vec<char> = digits[predigits..].to_vec();

        let mut infix = exponent_length_to_string(exponent.len())?;
        let prefix = if multiplier == vec!['1'] {
            "ün ".to_string()
        } else {
            let m: String = multiplier.iter().collect();
            let p = self.card(&parse_int(&m)?)?;
            // Plural form
            infix = format!(" {}s", infix);
            p
        };

        // Python: `if set(exponent) != set("0")` — i.e. "is the exponent
        // non-zero?". An *empty* exponent would also take this branch (set()
        // != {"0"}) and die in int(""); it can't happen, but the shape is
        // preserved rather than simplified to `any(!= '0')`.
        let exponent_is_zero = !exponent.is_empty() && exponent.iter().all(|&c| c == '0');
        let postfix = if !exponent_is_zero {
            let exponent_str: String = exponent.iter().collect();
            let p = self.card(&parse_int(&exponent_str)?)?;
            // "e" is introduced if there is a 3-digit gap before the next value
            if exponent_str.starts_with("000") {
                infix.push_str(" e ");
            } else {
                infix.push(' ');
            }
            p
        } else {
            String::new()
        };

        // adapt_milliard pads with spaces; Python strips them back off here.
        Ok(adapt_milliard(&format!("{}{}{}", prefix, infix, postfix))
            .trim()
            .to_string())
    }

    /// Port of `float_to_words` for the **cardinal** case (`ordinal=False`).
    ///
    /// ```python
    /// prefix = self.to_cardinal(int(float_number))
    /// float_part = str(float_number).split('.')[1]
    /// postfix = " ".join(self.to_cardinal(int(c)) for c in float_part)
    /// return prefix + " comma " + postfix
    /// ```
    ///
    /// Two Python facts drive the shape:
    ///
    ///   * The sign never reaches `float_to_words`. `to_cardinal` peels it off
    ///     first (`if number < 0: MINUS_PREFIX_WORD + self.to_cardinal(-number)`),
    ///     so `float_to_words` only ever sees a non-negative float. We mirror
    ///     that by prefixing `"minus "` around the absolute-value body rather
    ///     than by signing the parts. `int(-0.5)` would carry no minus anyway,
    ///     so this is the only way the sign survives — exactly as in Python.
    ///
    ///   * `str(float_number).split('.')[1]` is the shortest-round-trip repr's
    ///     fractional digits. `precision` (computed Python-side as
    ///     `abs(Decimal(str(v)).as_tuple().exponent)`) is *by definition* the
    ///     length of that fraction, so `format!("{:.precision$}", a)` — which
    ///     rounds `a` to exactly its own repr length, i.e. does no real
    ///     rounding — reproduces those digits byte for byte. We deliberately do
    ///     **not** reimplement `repr(float)`; we reuse the precision it derived.
    ///
    /// Out of range: if `str(v)` were in exponent form (`abs(v) >= 1e16` or a
    /// very small magnitude), Python's `split('.')[1]` raises `IndexError`.
    /// That is unreachable for every value the corpus/harness feeds here
    /// (max `1234.56`); see the porting report's `concerns`.
    fn float_to_words(&self, value: f64, precision: u32) -> Result<String> {
        let negative = value < 0.0;
        let a = value.abs();

        // prefix = self.to_cardinal(int(a)) — int() truncates toward zero; a >= 0.
        // Computed *before* the split, exactly as Python does — for a huge
        // float the maxval OverflowError (#159) beats the IndexError below.
        let pre = f64_trunc_to_bigint(a)?;
        let prefix = self.card(&pre)?;

        // float_part = str(a).split('.')[1]. A repr in exponent form ("1e+16")
        // has no '.' — Python's IndexError, corpus-confirmed.
        if !float_repr_has_point(a) {
            return Err(index_error("list index out of range"));
        }

        // float_part = str(a).split('.')[1], rebuilt at the repr precision.
        let formatted = format!("{:.*}", precision as usize, a);
        let float_part = formatted.split_once('.').map(|(_, f)| f).unwrap_or("");

        // postfix = " ".join(self.to_cardinal(int(c)) for c in float_part)
        let mut parts: Vec<String> = Vec::with_capacity(float_part.len());
        for ch in float_part.chars() {
            // Every char here is a decimal digit; int(c) never fails. Mapping a
            // miss to Python's ValueError keeps parity if that ever changed.
            let d = ch
                .to_digit(10)
                .ok_or_else(|| value_error(&ch.to_string()))?;
            parts.push(self.card(&BigInt::from(d))?);
        }
        let postfix = parts.join(" ");

        let body = format!("{}{}{}", prefix, FLOAT_INFIX_WORD, postfix);
        if negative {
            Ok(format!("{}{}", MINUS_PREFIX_WORD, body))
        } else {
            Ok(body)
        }
    }

    /// Reproduce `to_cardinal(Decimal)`. A `Decimal` is **not** a `float`, so it
    /// never takes the `isinstance(number, float)` branch; it flows through the
    /// integer-magnitude dispatch and crashes — the corpus records every
    /// `cardinal_dec` row for this language as an error.
    ///
    /// Behaviour, verified against the live interpreter (sign is peeled off
    /// first, so everything below is in terms of `a = |value|`):
    ///
    ///   * `a < 1_000_000` → **`TypeError`**. Every branch below a million
    ///     eventually evaluates `CARDINAL_WORDS[<Decimal>]` (directly, or after
    ///     recursing down to a value `< 20`), and a `Decimal` is not a valid
    ///     list index. True even for whole-valued Decimals like `Decimal("100")`.
    ///
    ///   * `a >= 1_000_000` → `big_number_to_cardinal(str(a))`:
    ///       - `a >= 10**65` → **`OverflowError`** (#159, checked first); a
    ///         longer-than-65-char `str(a)` below that → `NotImplementedError`.
    ///       - otherwise, if `a` has a fractional part (`precision > 0`), `str(a)`
    ///         contains a `.`; since `a >= 1e6` the integer part is at least 7
    ///         digits and `predigits <= 3`, so the `.` always lands in the
    ///         `exponent` slice and `int(exponent)` is the first `int()` to fail
    ///         → **`ValueError`** naming that exact substring.
    ///       - a whole number (`precision == 0`) has an all-digit `str(a)` and
    ///         succeeds, producing the same spelling as the integer path.
    ///
    /// The exotic exponent-notation Decimal (`Decimal("1E+7")`, positive
    /// `as_tuple().exponent`) is not reachable through the harness and is not
    /// modelled exactly — see the porting report's `concerns`.
    fn decimal_to_cardinal(&self, value: &BigDecimal, precision: u32) -> Result<String> {
        let negative = value.is_negative();
        let a = value.abs();

        if a < BigDecimal::from(1_000_000i64) {
            // CARDINAL_WORDS[<decimal.Decimal>] — list index that is not an int.
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }

        // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
        // checked before Python's `len(str(number)) >= 66` string guard.
        check_maxval(&a.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        if precision == 0 {
            // str(a) is all digits: big_number_to_cardinal succeeds and matches
            // the integer path. card() also owns the `>= 66 digits` ceiling.
            let int_part = a.with_scale(0).as_bigint_and_exponent().0;
            let body = self.card(&int_part)?;
            return Ok(if negative {
                format!("{}{}", MINUS_PREFIX_WORD, body)
            } else {
                body
            });
        }

        // precision > 0: rebuild str(a) = "<int_part>.<frac>" so the failing
        // substring in the ValueError matches Python's byte for byte.
        let (coeff, _exp) = a.with_scale(precision as i64).as_bigint_and_exponent();
        let modulus = BigInt::from(10).pow(precision);
        let int_str = (&coeff / &modulus).to_string();
        let frac_raw = (&coeff % &modulus).to_string();
        let frac_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(frac_raw.len())),
            frac_raw
        );
        let s = format!("{}.{}", int_str, frac_str);

        // big_number_to_cardinal: the `length >= 66` guard runs before any int().
        let length = s.chars().count();
        // Only reachable below maxval when the fraction digits push str(number)
        // past 65 characters; magnitudes past maxval were rejected above (#159).
        if length >= 66 {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        // predigits = length % 3 or 3; exponent = digits[predigits:]. int() of
        // that slice is the crash site (it holds the '.').
        let predigits = if length % 3 == 0 { 3 } else { length % 3 };
        let exponent: String = s.chars().skip(predigits).collect();
        Err(value_error(&exponent))
    }

    /// `Num2Word_RM_PUTER.to_ordinal` for a `float` argument:
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
    ///     nolla") and then takes the ordinal suffix — ends in 'a', so the 'a'
    ///     is dropped and "evel" appended: "vainchün comma nollevel";
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
                return Err(index_error("list index out of range"));
            }
            let formatted = format!("{:.*}", precision as usize, f);
            let float_part = formatted.split_once('.').map(|(_, x)| x).unwrap_or("");
            let mut parts: Vec<String> = Vec::with_capacity(float_part.len());
            for ch in float_part.chars() {
                let d = ch
                    .to_digit(10)
                    .ok_or_else(|| value_error(&ch.to_string()))?;
                parts.push(self.card(&BigInt::from(d))?);
            }
            return Ok(format!(
                "{}{}{}",
                prefix,
                FLOAT_INFIX_WORD,
                parts.join(" ")
            ));
        }
        if f <= 20.0 {
            // ORDINAL_WORDS[<float>] — a float is not a valid list index.
            return Err(N2WError::Type(
                "list indices must be integers or slices, not float".to_string(),
            ));
        }
        // cardinal = self.to_cardinal(number) — the float branch again. Ends in
        // "... comma nolla" here, so the 'a' rule (drop 'a', add "evel") always
        // fires; the other arms are kept for shape parity.
        let cardinal = self.float_to_words(f, precision)?;
        let last = cardinal
            .chars()
            .next_back()
            .ok_or_else(|| index_error("string index out of range"))?;
        if last == 'a' {
            Ok(format!("{}evel", drop_last_char(&cardinal)))
        } else if cardinal.ends_with("set") {
            Ok(format!("{}tevel", cardinal))
        } else {
            Ok(format!("{}evel", cardinal))
        }
    }

    /// `Num2Word_RM_PUTER.to_ordinal` for a `Decimal` argument. `Decimal % 1`
    /// works, so a *fractional* Decimal takes the `float_to_words(ordinal=True)`
    /// branch and renders (reading `str(Decimal)`); a whole-valued one falls
    /// into the integer branches: `<= 20` dies on `ORDINAL_WORDS[<Decimal>]`
    /// (TypeError), `> 20` re-enters `to_cardinal(Decimal)` — TypeError below
    /// `10**6`, str-splitting above (ValueError for a fractional or
    /// scientific-repr Decimal). `Decimal("-0.0") < 0` is False, so it is *not*
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
            let prefix = self.to_ordinal(&pre)?;
            let s = crate::strnum::python_decimal_str(value);
            let float_part = match s.split_once('.') {
                Some((_, frac)) => frac.to_string(),
                // Scientific repr with no '.' (e.g. Decimal("5E-7")).
                None => return Err(index_error("list index out of range")),
            };
            let mut parts: Vec<String> = Vec::with_capacity(float_part.len());
            for c in float_part.chars() {
                // int(c) — an 'E'/'+' from a scientific repr is ValueError.
                let d = c.to_digit(10).ok_or_else(|| value_error(&c.to_string()))?;
                parts.push(self.card(&BigInt::from(d))?);
            }
            return Ok(format!(
                "{}{}{}",
                prefix,
                FLOAT_INFIX_WORD,
                parts.join(" ")
            ));
        }
        if value <= &BigDecimal::from(20) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        let cardinal = self.decimal_to_cardinal(value, precision)?;
        let last = cardinal
            .chars()
            .next_back()
            .ok_or_else(|| index_error("string index out of range"))?;
        if last == 'a' {
            Ok(format!("{}evel", drop_last_char(&cardinal)))
        } else if cardinal.ends_with("set") {
            Ok(format!("{}tevel", cardinal))
        } else {
            Ok(format!("{}evel", cardinal))
        }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRmPuter {
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
        Err(crate::lang_rm::unsupported("rm_puter", "to='currency'"))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_puter", "to='cheque'"))
    }

    /// `MINUS_PREFIX_WORD` is a class attribute used directly by
    /// `to_cardinal`/`to_ordinal`; `self.negword` never exists on this class.
    /// Exposed here only for completeness — nothing in this file reads it.
    fn negword(&self) -> &str {
        MINUS_PREFIX_WORD
    }

    /// `FLOAT_INFIX_WORD`. This class overrides `to_cardinal_float` (below) and
    /// never routes through `base.rs`'s generic pointword path, so nothing in
    /// this file actually reads `pointword`; it is kept only so the trait's
    /// default float path — were it ever reached — would use the right word.
    fn pointword(&self) -> &str {
        FLOAT_INFIX_WORD
    }

    /// Port of the float/Decimal cardinal path.
    ///
    /// This class handles non-integers *inside* `to_cardinal` (the
    /// `isinstance(number, float)` branch → `float_to_words`), so the generic
    /// `base.py` `to_cardinal_float` never runs for it and the trait default
    /// (which doubles the spaces around " comma ") is wrong here. We dispatch
    /// the two `FloatValue` arms to the faithful reproductions instead.
    ///
    /// `precision_override` is ignored on purpose: `num2words`'s `precision=`
    /// kwarg only takes effect when `hasattr(converter, "precision")`, and this
    /// bare class has no `precision` attribute — nor does `float_to_words` read
    /// one. It slices `str(float_number)` directly.
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
            FloatValue::Float { value, precision } => self.float_to_words(*value, *precision),
            FloatValue::Decimal { value, precision } => {
                self.decimal_to_cardinal(value, *precision)
            }
        }
    }

    /// Port of `Num2Word_RM_PUTER.to_cardinal`, integer path only.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.card(value)
    }

    /// Port of `Num2Word_RM_PUTER.to_ordinal`.
    ///
    /// `number % 1 != 0` is always false for an `int`, so the float branch is
    /// not modelled. Negatives recurse behind "minus " and are *not* rejected
    /// (there is no `verify_ordinal` on this class): `to_ordinal(-1)` ==
    /// "minus prüm".
    ///
    /// The suffix rules read the **last character** of the cardinal:
    ///   * ends in "a" → drop it, add "evel"   (30 "trenta"  → "trentevel")
    ///   * ends in "set" → add "tevel"         (77 "settauntaset" →
    ///                                          "settauntasettevel")
    ///   * otherwise    → add "evel"           (21 "vainchün" → "vainchünevel")
    ///
    /// Note the "a" test runs first, so a cardinal ending in "a" never reaches
    /// the "set" test (they are mutually exclusive anyway).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, self.to_ordinal(&-value)?));
        }
        if value <= &BigInt::from(20) {
            return Ok(ordinal_word(as_index(value)?)?.to_string());
        }

        let cardinal = self.card(value)?;
        // Python's `cardinal[-1]`: IndexError on an empty string. Unreachable
        // — to_cardinal never returns "" for value > 20.
        let last = cardinal
            .chars()
            .next_back()
            .ok_or_else(|| index_error("string index out of range"))?;

        if last == 'a' {
            Ok(format!("{}evel", drop_last_char(&cardinal)))
        } else if cardinal.ends_with("set") {
            Ok(format!("{}tevel", cardinal))
        } else {
            Ok(format!("{}evel", cardinal))
        }
    }

    /// Python's class had no `to_ordinal_num` (AttributeError for every
    /// input), and no ordinal-numeral marker is attested for Romansh in the
    /// module or its tests, so the port raises a clear NotImplementedError
    /// rather than inventing one (#157).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_puter", "to='ordinal_num'"))
    }

    /// Python's class had no `to_year` (AttributeError for every input).
    /// `Num2Word_Base.to_year` is the cardinal, which is what the port
    /// returns (#157).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
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
            FloatValue::Float { value, precision } => self.float_to_words(*value, *precision),
            FloatValue::Decimal { value, precision } => {
                self.decimal_to_cardinal(value, *precision)
            }
        }
    }

    /// `to_ordinal(float/Decimal)` — see [`LangRmPuter::float_ordinal`] /
    /// [`LangRmPuter::decimal_ordinal`] for the branch-by-branch mapping.
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
            FloatValue::Decimal { value, precision } => self.decimal_ordinal(value, *precision),
        }
    }

    /// See [`Lang::to_ordinal_num`] above (#157).
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_puter", "to='ordinal_num'"))
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
        Err(crate::lang_rm::unsupported("rm_puter", "to='fraction'"))
    }
}
