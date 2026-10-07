//! Faithful port of num2words2's `lang_RM_SURMIRAN.py` (Rumantsch Surmiran).
//!
//! # Shape: self-contained
//!
//! `Num2Word_RM_SURMIRAN` is a **plain class with no base class at all** — it
//! does not subclass `Num2Word_Base`, `Num2Word_EU` or anything else, and its
//! `__init__` is a bare `pass`. It defines its own `to_cardinal` from scratch,
//! so `cards`/`maxval`/`merge` (and therefore `splitnum`/`clean`) are never
//! touched. They stay at their trait defaults here, exactly as in the Python.
//!
//! The module was "Based on lang_IT template from Filippo Costa" (see the
//! Python header), and the structure mirrors `lang_it.rs` closely:
//! `tens_to_cardinal` / `hundreds_to_cardinal` / `thousands_to_cardinal` /
//! `big_number_to_cardinal` dispatched from `to_cardinal` by magnitude.
//!
//! Sign handling goes through `MINUS_PREFIX_WORD` (`"minus "`), not `negword`:
//! the class has no `negword` attribute whatsoever, so the trait's default
//! `negword` is left alone and is unreachable for the four in-scope modes.
//!
//! The float/Decimal cardinal path is ported below (`float_to_words` /
//! `FLOAT_INFIX_WORD`, exposed through the `to_cardinal_float` trait hook). The
//! Python class overrides `to_cardinal` and handles floats inline — it has no
//! base class, so it never touches `base.float2tuple` / `to_cardinal_float` /
//! `pointword`; it reads the fractional digits straight off `str(value)`.
//!
//! The class defines no `to_currency` / `to_cheque` / `to_fraction` at all;
//! Python raised `AttributeError` for every one of those, for the same reason
//! as bug 1 below (fixed, #157).
//!
//! # Faithfully reproduced Python bugs and quirks
//!
//! This is a port, not a rewrite. Every item below is wrong-looking but is
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. ~~**`to_year` and `to_ordinal_num` do not exist.**~~ Fixed
//!    (gladiaio/num2words2#157). Because the class inherits from nothing,
//!    Python raised `AttributeError` for every call of either mode, every
//!    string input and every currency/cheque/fraction; a `Decimal` crashed in
//!    the integer ladder. The port now follows `lang_rm` (see its module
//!    docs): strings use the shared parse, a `Decimal` reads like the int or
//!    the float, `to_year` is the cardinal as in `Num2Word_Base`, and
//!    `to_ordinal_num`, currency, cheques and fractions raise a
//!    `NotImplementedError` naming the language — no ordinal-numeral marker
//!    or currency wording is attested to port.
//!
//! 2. **`EXPONENT_PREFIXES[0]` is `ZERO` ("nolla"), not `""`.** An
//!    `exponent_length` of 3 would index it and produce the nonsense
//!    "nollailliarda". Unreachable: `big_number_to_cardinal` only runs for
//!    `number >= 10**6` (`length >= 7`), and `predigits = length % 3 or 3`
//!    is in `{1,2,3}`, so `exponent_length = length - predigits` is a multiple
//!    of 3 that is always `>= 6`. Kept verbatim anyway.
//!
//! 3. **The empty-`exponent` path would hit `int("")` → `ValueError`.**
//!    `set(exponent) != set("0")` is *true* for an empty exponent (`set()` vs
//!    `{"0"}`), which would fall into the `int("".join(exponent))` branch.
//!    Unreachable for the same reason as bug 2. Modelled as `N2WError::Value`
//!    rather than a panic, for fidelity.
//!
//! 4. **`tens_to_cardinal`'s fallback branch is dead code.**
//!    `CARDINAL_WORDS[tens][:-1] + "anta"` only runs when `tens not in
//!    STR_TENS`, but `STR_TENS` covers 2..=9 and the method is only called for
//!    `20 <= number < 100`. Ported as-is (with character-based slicing).
//!
//! 5. **`to_ordinal` of a negative is not a real ordinal**: it prefixes
//!    "minus " and recurses, so `to_ordinal(-1)` == "minus amprem" rather than
//!    raising. Unlike most `Num2Word_Base` descendants there is no
//!    `verify_ordinal` call anywhere — the class does not have one.
//!
//! 6. **`adapt_thousand` rewrites digits that came from the *multiplier*.**
//!    Its `.replace("treismella", "tremella")` is applied to the fully
//!    assembled string, so the "treis" of an embedded 123 is contracted too:
//!    `to_cardinal(123456)` builds "tschentvantgatreismella…" and the replace
//!    turns it into "tschentvantga**tremella**quattertschenttschuncantaseis".
//!    That is the corpus-confirmed output, and it is what the rule intends
//!    (123 000 → "tschentvantgatremella"), but it is worth noting that the
//!    rule is positional-blind.
//!
//! 7. **Numbers with 66 or more digits raise**, checked *before* anything else
//!    in `big_number_to_cardinal`. `10**64` (65 digits) works → "diesch
//!    decilliardas"; `10**65` raises. Python raises `NotImplementedError`
//!    ("The given number is too large."). Fixed (gladiaio/num2words2#159):
//!    the port raises `OverflowError` ("abs(v) must be less than 10^65.") and
//!    `maxval()` reports 10^65.
//!
//! # Python semantics that matter here
//!
//! * `str.replace` replaces **every** occurrence, not just the first; Rust's
//!   `str::replace` matches that, so the rewrite chains map across directly.
//!   The *order* of the chained replaces is load-bearing in three places:
//!   - `phonetic_contraction`: `"aen"` → `"egn"` must run before the `"_"`
//!     erasure, and `"tga_"` → `"tg"` before it too (that is how 20 becomes
//!     "vantg" rather than "vantga").
//!   - `adapt_hundred`: `"aendesch"` → `"adendesch"` must run **before**
//!     `"aen"` → `"adegn"`, else 111 would become "tschentadegndesch"
//!     instead of "tschentadendesch".
//!   - `adapt_thousand`: likewise, and its trailing `"aa"` → `"a"` mops up
//!     what the earlier rules left (1011 → "mellaadendesch" → "melladendesch").
//! * All vocabulary in this module is pure ASCII (no accents), but `[:-1]` is
//!   still ported through a character-based helper rather than byte slicing,
//!   so the dead branch in bug 4 stays correct in principle.
//! * `Python`'s `strip()` removes leading/trailing whitespace; `trim()` matches.
//! * `number % 1 != 0` in `to_ordinal` is the float guard. Input is integral
//!   here, so it is always false and the float branch is dropped as out of
//!   scope.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};

// Globals
// -------

const ZERO: &str = "nolla";

const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "en",
    "dus",
    "treis",
    "quatter",
    "tschintg",
    "seis",
    "set",
    "otg",
    "nov",
    "diesch",
    "endesch",
    "dodesch",
    "tredesch",
    "quittordesch",
    "quindesch",
    "sedesch",
    "dischset",
    "dischdotg",
    "dischnov",
];

const ORDINAL_WORDS: [&str; 21] = [
    ZERO,
    "amprem",
    "sagond",
    "terz",
    "quart",
    "tschintgavel",
    "seisavel",
    "settavel",
    "otgavel",
    "novavel",
    "dieschavel",
    "endeschavel",
    "dodeschavel",
    "tredeschavel",
    "quittordeschavel",
    "quindeschavel",
    "sedeschavel",
    "dischsettavel",
    "dischdotgavel",
    "dischnovavel",
    "vantgavel",
];

/// Python's `STR_TENS` dict. Note "20" alone is "ventg" per the source comment,
/// but the table stores "vantga" — the trailing "a" is what
/// `phonetic_contraction`'s `"tga_"` → `"tg"` rule consumes.
fn str_tens(tens: usize) -> Option<&'static str> {
    match tens {
        2 => Some("vantga"),
        3 => Some("trenta"),
        4 => Some("curanta"),
        5 => Some("tschuncanta"),
        6 => Some("sessanta"),
        7 => Some("settanta"),
        8 => Some("otganta"),
        9 => Some("novanta"),
        _ => None,
    }
}

/// These prefixes are used for extremely big numbers.
///
/// Index 0 is `ZERO` — see module bug 2.
const EXPONENT_PREFIXES: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

const MINUS_PREFIX_WORD: &str = "minus ";

/// Python's `Num2Word_RM_SURMIRAN.FLOAT_INFIX_WORD`. The float path joins the
/// integer and fractional readings with this, in place of a `pointword`.
const FLOAT_INFIX_WORD: &str = " comma ";

// Utils
// =====

/// Python's `phonetic_contraction`.
///
/// `_` is a marker for "empty", i.e. no following unit. Two forms of 1 exist
/// (cardinal "en" and pronoun "egn") — "egn" is used within complex numbers.
fn phonetic_contraction(string: &str) -> String {
    string
        .replace("aen", "egn") // ex. "trentaen" -> "trentegn"
        .replace("aotg", "otg") // ex. "curantaotg" -> "curantotg"
        .replace("tga_", "tg") // ex. "vantga" -> "vantg"
        .replace('_', "")
}

/// Python's `adapt_hundred`: 2: dus -> du, 3: treis -> tre, plus the a/ad
/// phonotactic adaptation.
fn adapt_hundred(string: &str) -> String {
    string
        .replace("dustschent", "dutschent")
        .replace("treistschent", "tretschent")
        .replace("aendesch", "adendesch")
        .replace("aen", "adegn")
        .replace("aotg", "adotg")
}

/// Python's `adapt_thousand`: 2: dus -> du, 3: treis -> tre, plus the a/ad
/// phonotactic adaptation. See module bug 6 on the positional blindness.
fn adapt_thousand(string: &str) -> String {
    string
        .replace("dusmella", "dumella")
        .replace("treismella", "tremella")
        .replace("aendesch", "adendesch")
        .replace("aaen", "adegn")
        .replace("aaotg", "adotg")
        .replace("aa", "a")
}

/// Python's `adapt_milliarda`: article gender agreement and the e/ed
/// phonotactic adaptation.
///
/// Python pads with a space on each side so the rules can anchor on word
/// boundaries; the caller strips the padding back off.
fn adapt_milliarda(string: &str) -> String {
    let string = format!(" {} ", string);
    string
        .replace(" en milliarda ", " ena milliarda ")
        .replace(" a endesch", " ad endesch")
        .replace(" a en", " ad egn")
        .replace(" a otg", " ad otg")
}

/// Python's `exponent_length_to_string`.
///
/// We always assume `exponent` to be a multiple of 3. If that's not true, then
/// `big_number_to_cardinal` did something wrong.
///
/// The `EXPONENT_PREFIXES` index is provably in range: `exponent_length <= 63`
/// (the 66-digit guard caps `length` at 65, and `predigits >= 1`), so
/// `exponent_length // 6 <= 10` and the table has 11 entries. The bounds check
/// is kept and mapped to `N2WError::Index` rather than allowed to panic.
fn exponent_length_to_string(exponent_length: usize) -> Result<String> {
    let prefix = EXPONENT_PREFIXES
        .get(exponent_length / 6)
        .ok_or_else(|| N2WError::Index(format!("list index out of range: {}", exponent_length / 6)))?;
    if exponent_length % 6 == 0 {
        Ok(format!("{}illiun", prefix))
    } else {
        Ok(format!("{}illiarda", prefix))
    }
}

/// Python's `omitt_if_zero` (the typo in the name is upstream's).
fn omitt_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        String::new()
    } else {
        number_to_string.to_string()
    }
}

/// Python's `empty_if_zero`: `_` marks "no following unit" for
/// [`phonetic_contraction`] to erase.
fn empty_if_zero(number_to_string: &str) -> String {
    if number_to_string == ZERO {
        "_".to_string()
    } else {
        number_to_string.to_string()
    }
}

/// Python's `s[:-1]`, by character rather than byte. See module bug 4 — the
/// only caller is dead code, but the semantics are preserved regardless.
fn drop_last_char(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.pop();
    chars.into_iter().collect()
}

// Main class
// ==========

pub struct LangRmSurmiran;

impl LangRmSurmiran {
    pub fn new() -> Self {
        LangRmSurmiran
    }

    /// Python's `tens_to_cardinal`. Only called for `20 <= number < 100`, so
    /// `number` is provably bounded and the `usize` narrowing is safe.
    fn tens_to_cardinal(&self, number: u64) -> String {
        let tens = (number / 10) as usize;
        let units = (number % 10) as usize;
        let prefix = match str_tens(tens) {
            Some(p) => p.to_string(),
            // Dead code — see module bug 4.
            None => format!("{}anta", drop_last_char(CARDINAL_WORDS[tens])),
        };
        // we keep track of 0 using '_' -- removed in phonetic_contraction
        let postfix = empty_if_zero(CARDINAL_WORDS[units]);
        phonetic_contraction(&format!("{}{}", prefix, postfix))
    }

    /// Python's `hundreds_to_cardinal`. Only called for `100 <= number < 1000`.
    fn hundreds_to_cardinal(&self, number: u64) -> Result<String> {
        let hundreds = (number / 100) as usize;
        let tens = number % 100;
        let mut prefix = "tschent".to_string();
        if hundreds != 1 {
            prefix = format!("{}tschent", CARDINAL_WORDS[hundreds]);
        }
        let postfix = omitt_if_zero(&self.cardinal(&BigInt::from(tens))?);
        // "a/ad" is inserted if tens <= 13 or = 15, 16, 20, 30
        // distribution may seem unusual but it was reviewed by a native speaker
        // surmiran's "and" is normally "e/ed", but in numbers, "a/ad" is used
        let infix = if (tens > 0 && tens <= 13) || matches!(tens, 15 | 16 | 20 | 30) {
            "a"
        } else {
            ""
        };
        Ok(adapt_hundred(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// Python's `thousands_to_cardinal`. Only called for
    /// `1000 <= number < 1_000_000`.
    fn thousands_to_cardinal(&self, number: u64) -> Result<String> {
        let thousands = number / 1000;
        let hundreds = number % 1000;
        let mut prefix = "mella".to_string();
        if thousands != 1 {
            prefix = format!("{}mella", self.cardinal(&BigInt::from(thousands))?);
        }
        let postfix = omitt_if_zero(&self.cardinal(&BigInt::from(hundreds))?);
        // "a/ad" is inserted if tens <= 100
        let infix = if hundreds <= 100 && !postfix.is_empty() {
            "a"
        } else {
            ""
        };
        Ok(adapt_thousand(&format!("{}{}{}", prefix, infix, postfix)))
    }

    /// Python's `big_number_to_cardinal`. Called for `number >= 10**6`; the
    /// value is unbounded, so it stays a `BigInt` throughout.
    fn big_number_to_cardinal(&self, number: &BigInt) -> Result<String> {
        let digits: Vec<char> = number.to_string().chars().collect();
        let length = digits.len();
        check_maxval(number, maxval_ceiling())?;
        // This is how many digits come before the "illion" term.
        //   tschent milliardas => 3
        //   diesch milliuns => 2
        //   ena milliarda => 1
        let predigits = if length % 3 == 0 { 3 } else { length % 3 };
        let multiplier: Vec<char> = digits[..predigits].to_vec();
        let exponent: Vec<char> = digits[predigits..].to_vec();
        let mut infix = exponent_length_to_string(exponent.len())?;
        let prefix;
        if multiplier == vec!['1'] {
            prefix = "en ".to_string();
        } else {
            let m: String = multiplier.iter().collect();
            prefix = self.cardinal(&parse_int(&m)?)?;
            // Plural form
            infix = format!(" {}s", infix);
        }
        // Read as: Does the value of exponent equal 0?
        //
        // Python compares `set(exponent) != set("0")`, which is true when the
        // exponent is empty as well as when any digit is non-zero.
        let postfix;
        if exponent.is_empty() || exponent.iter().any(|c| *c != '0') {
            let exponent_str: String = exponent.iter().collect();
            // int("") raises ValueError — unreachable, see module bug 3.
            postfix = self.cardinal(&parse_int(&exponent_str)?)?;
            // we introduce "e" if 3-digits gap before next value
            if exponent_str.starts_with("000") {
                infix.push_str(" a ");
            } else {
                infix.push(' ');
            }
        } else {
            postfix = String::new();
        }
        Ok(adapt_milliarda(&format!("{}{}{}", prefix, infix, postfix))
            .trim()
            .to_string())
    }

    /// Python's `to_cardinal`. The float branch is out of scope.
    fn cardinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.cardinal(&(-number))?
            ));
        }
        // Every branch below 10**6 is bounded, so the u64 narrowing is proven
        // safe; only big_number_to_cardinal sees arbitrary magnitudes.
        if number < &BigInt::from(20) {
            let n = number.to_usize().expect("bounded by 20");
            Ok(CARDINAL_WORDS[n].to_string())
        } else if number < &BigInt::from(100) {
            Ok(self.tens_to_cardinal(number.to_u64().expect("bounded by 100")))
        } else if number < &BigInt::from(1000) {
            self.hundreds_to_cardinal(number.to_u64().expect("bounded by 1000"))
        } else if number < &BigInt::from(1_000_000) {
            self.thousands_to_cardinal(number.to_u64().expect("bounded by 10**6"))
        } else {
            self.big_number_to_cardinal(number)
        }
    }

    /// Python's `to_ordinal`. The float branch (`number % 1 != 0`) is
    /// unreachable for integral input and is out of scope.
    fn ordinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // Not a real ordinal — see module bug 5.
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.ordinal(&(-number))?
            ));
        }
        if number <= &BigInt::from(20) {
            let n = number.to_usize().expect("bounded by 20");
            return Ok(ORDINAL_WORDS[n].to_string());
        }
        let cardinal = self.cardinal(number)?;
        // `cardinal[-1]` on an empty string would be an IndexError; the
        // string is never empty for number > 20.
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }

    /// Python's `float_to_words` (cardinal branch, `ordinal=False`):
    ///
    /// ```python
    /// prefix = self.to_cardinal(int(float_number))
    /// float_part = str(float_number).split('.')[1]
    /// postfix = " ".join(self.to_cardinal(int(c)) for c in float_part)
    /// return prefix + FLOAT_INFIX_WORD + postfix
    /// ```
    ///
    /// This class does **not** use `base.float2tuple` and its `< 0.01`
    /// artefact heuristic — it reads the digits straight off `str(value)`. So
    /// `2.675` reads its literal repr digits (`675`), never the
    /// `674.9999999998`-style artefact the base path deliberately preserves.
    ///
    /// `value` is already non-negative: [`to_cardinal_float`] strips the sign
    /// via `MINUS_PREFIX_WORD`, exactly as `to_cardinal(number < 0)` recurses
    /// on `-number` before hitting the float branch.
    ///
    /// `precision` is the repr-derived fractional-digit count. `str(value)` is
    /// shortest-round-trip, and formatting to that precision reproduces it for
    /// normal-range values (e.g. `1.0` -> `"1.0"`, so `split('.')[1]` == `"0"`).
    fn float_to_words(&self, value: f64, precision: u32) -> Result<String> {
        let s = format!("{:.*}", precision as usize, value);
        let mut parts = s.splitn(2, '.');
        let int_str = parts.next().unwrap_or("");
        // int(float_number) truncates toward zero; for value >= 0 that is the
        // integer part of the repr string. Computed *before* the point probe,
        // exactly as Python does — for a huge float the maxval
        // OverflowError (#159) beats the IndexError below.
        let prefix = self.cardinal(&parse_int(int_str)?)?;
        // Python indexes `str(value).split('.')[1]`; an absent point (a float
        // whose repr carries no '.', i.e. exponent form) raises IndexError.
        // This must be an explicit repr test rather than a probe of the
        // reconstruction: the binding's `precision` for `1e+16` is 16
        // (`abs(Decimal("1e+16").as_tuple().exponent)`), so `{:.16}` *would*
        // carry a point Python's repr does not have. Corpus-confirmed
        // IndexError for `1e+16` and `1e+20`.
        if !float_repr_has_point(value) {
            return Err(N2WError::Index("list index out of range".to_string()));
        }
        let float_part = parts
            .next()
            .ok_or_else(|| N2WError::Index("list index out of range".to_string()))?;
        let mut words: Vec<String> = Vec::new();
        for c in float_part.chars() {
            // int(c): a non-digit would raise ValueError. Formatted fractional
            // digits are always 0-9, so this cannot fire here, but the mapping
            // is kept faithful.
            let d = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
            })?;
            words.push(self.cardinal(&BigInt::from(d))?);
        }
        Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, words.join(" ")))
    }

    /// Python's `to_cardinal` for a `Decimal` argument.
    ///
    /// The class only special-cases `isinstance(number, float)`, never
    /// `Decimal`, so a `Decimal` falls through the magnitude ladder as if it
    /// were an int — but it cannot index a list, so every path below `10**6`
    /// crashes:
    ///
    /// * `number < 20` does `CARDINAL_WORDS[number]` -> **TypeError** (a
    ///   `Decimal` is not a valid list index, integral value or not).
    /// * `20 <= number < 100` reaches `CARDINAL_WORDS[units]` in
    ///   `tens_to_cardinal` -> TypeError.
    /// * `100 <= number < 1000` reaches `CARDINAL_WORDS[hundreds]` (or recurses
    ///   into the tens case) -> TypeError.
    /// * `1000 <= number < 10**6` recurses through the hundreds/tens cases ->
    ///   TypeError.
    ///
    /// So every value below `10**6` yields the same `TypeError`, with the same
    /// message Python's list-indexing raises. (The corpus compares exception
    /// type only, and the four `< 20` decimal rows all land here.)
    ///
    /// At `>= 10**6` it enters `big_number_to_cardinal`, which slices
    /// `str(number)` — and `str` keeps the decimal point, so `int(exponent_str)`
    /// raises **ValueError** for any non-integral value (the point is always in
    /// the exponent slice, since the integer part already has >= 7 digits while
    /// `predigits <= 3`). That is the `98746251323029.99` corpus row.
    fn cardinal_decimal(&self, value: &BigDecimal) -> Result<String> {
        if value.is_negative() {
            // Python: number < 0 -> MINUS_PREFIX_WORD + to_cardinal(-number).
            // The recursion raises before the concatenation, so the sign word
            // never actually appears — the exception type is what propagates.
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.cardinal_decimal(&value.abs())?
            ));
        }
        if value < &BigDecimal::from(1_000_000i64) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        // gladiaio/num2words2#159: a magnitude past maxval is an OverflowError,
        // checked before Python's `len(str(number)) >= 66` string guard.
        check_maxval(&value.with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        // number >= 10**6 -> big_number_to_cardinal(Decimal), over str(number).
        // `python_decimal_str` reproduces Python's `str(Decimal)` exactly,
        // including the scientific form: `str(Decimal("1E+20"))` is `"1E+20"`,
        // whose multiplier slice "1E" fails int() with ValueError — a plain
        // digit expansion would instead succeed and diverge from the corpus.
        let s = crate::strnum::python_decimal_str(value);
        self.big_number_to_cardinal_str(&s)
    }

    /// Python's `big_number_to_cardinal` operating on an already-materialised
    /// `str(number)` — the `Decimal` entry point, where that string may carry a
    /// decimal point. Byte-for-byte the same logic as
    /// [`Self::big_number_to_cardinal`]; only the source of the digit string
    /// differs (`str(int)` vs `str(Decimal)`), so it is duplicated rather than
    /// letting the integer path grow a second caller.
    fn big_number_to_cardinal_str(&self, s: &str) -> Result<String> {
        let digits: Vec<char> = s.chars().collect();
        let length = digits.len();
        // Only reachable below maxval when the fraction digits push str(number)
        // past 65 characters; magnitudes past maxval were rejected above (#159).
        if length >= 66 {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        let predigits = if length % 3 == 0 { 3 } else { length % 3 };
        let multiplier: Vec<char> = digits[..predigits].to_vec();
        let exponent: Vec<char> = digits[predigits..].to_vec();
        let mut infix = exponent_length_to_string(exponent.len())?;
        let prefix;
        if multiplier == vec!['1'] {
            prefix = "en ".to_string();
        } else {
            // int("".join(multiplier)): a decimal point never lands in the
            // multiplier here (integer part has >= 7 digits, predigits <= 3),
            // so this is pure digits and parses cleanly.
            let m: String = multiplier.iter().collect();
            prefix = self.cardinal(&parse_int(&m)?)?;
            infix = format!(" {}s", infix);
        }
        let postfix;
        if exponent.is_empty() || exponent.iter().any(|c| *c != '0') {
            let exponent_str: String = exponent.iter().collect();
            // int(exponent_str): for a Decimal the point sits in this slice,
            // so parse_int raises ValueError — the corpus behaviour.
            postfix = self.cardinal(&parse_int(&exponent_str)?)?;
            if exponent_str.starts_with("000") {
                infix.push_str(" a ");
            } else {
                infix.push(' ');
            }
        } else {
            postfix = String::new();
        }
        Ok(adapt_milliarda(&format!("{}{}{}", prefix, infix, postfix))
            .trim()
            .to_string())
    }

    /// Python's `to_ordinal` for a `float` argument:
    ///
    /// ```python
    /// if number < 0:  return MINUS_PREFIX_WORD + self.to_ordinal(-number)
    /// elif number % 1 != 0:  return self.float_to_words(number, ordinal=True)
    /// elif number <= 20:  return ORDINAL_WORDS[number]   # float index -> TypeError
    /// else: cardinal = self.to_cardinal(number)  # float branch, "... comma nolla"
    /// ```
    ///
    /// Corpus-confirmed quirks, all reproduced:
    ///   * a *fractional* float works: `2.5` -> "sagond comma tschintg"
    ///     (ordinal prefix over `int(2.5)`, cardinal digit words after);
    ///   * a *whole* float `<= 20` (incl. `-0.0`/`0.0`) dies on
    ///     `ORDINAL_WORDS[<float>]` -> TypeError;
    ///   * a whole float `> 20` renders its float *cardinal* ("vantgegn comma
    ///     nolla") and then gets the ordinal suffix: "vantgegn comma nollavel";
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
            let s = format!("{:.*}", precision as usize, f);
            let mut split = s.splitn(2, '.');
            let int_str = split.next().unwrap_or("");
            let prefix = self.ordinal(&parse_int(int_str)?)?;
            if !float_repr_has_point(f) {
                return Err(N2WError::Index("list index out of range".to_string()));
            }
            let float_part = split
                .next()
                .ok_or_else(|| N2WError::Index("list index out of range".to_string()))?;
            let mut words: Vec<String> = Vec::new();
            for c in float_part.chars() {
                let d = c.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
                })?;
                words.push(self.cardinal(&BigInt::from(d))?);
            }
            return Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, words.join(" ")));
        }
        if f <= 20.0 {
            // ORDINAL_WORDS[<float>] — a float is not a valid list index.
            return Err(N2WError::Type(
                "list indices must be integers or slices, not float".to_string(),
            ));
        }
        // cardinal = self.to_cardinal(number) — the float branch again. Ends in
        // "... comma nolla" here, so the 'a' rule always fires; the other arms
        // are kept for shape parity with the integer path.
        let cardinal = self.float_to_words(f, precision)?;
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }

    /// Python's `to_ordinal` for a `Decimal` argument. `Decimal % 1` works,
    /// so a *fractional* Decimal takes the `float_to_words(ordinal=True)`
    /// branch and renders (reading `str(Decimal)`); a whole-valued one falls
    /// into the integer branches: `<= 20` dies on `ORDINAL_WORDS[<Decimal>]`
    /// (TypeError), `> 20` re-enters `to_cardinal(Decimal)` — TypeError below
    /// `10**6`, str-splitting above (ValueError for `Decimal("1E+20")`, whose
    /// str is scientific). `Decimal("-0.0") < 0` is False, so it is *not*
    /// minus-prefixed — it crashes in the table branch like `0.0` does.
    fn decimal_ordinal(&self, value: &BigDecimal) -> Result<String> {
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.decimal_ordinal(&value.abs())?
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
            let mut words: Vec<String> = Vec::new();
            for c in float_part.chars() {
                // int(c) — an 'E'/'+' from a scientific repr is ValueError.
                let d = c.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
                })?;
                words.push(self.cardinal(&BigInt::from(d))?);
            }
            return Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, words.join(" ")));
        }
        if value <= &BigDecimal::from(20) {
            return Err(N2WError::Type(
                "list indices must be integers or slices, not decimal.Decimal".to_string(),
            ));
        }
        let cardinal = self.cardinal_decimal(value)?;
        let suffix = if cardinal.chars().next_back() == Some('a') {
            "vel"
        } else if cardinal.ends_with("set") {
            "tavel"
        } else {
            "avel"
        };
        Ok(format!("{}{}", cardinal, suffix))
    }
}

/// Whether Python's `str(f)` — the shortest round-trip repr — contains a `.`.
///
/// repr picks exponent form (no point) for finite non-zero magnitudes below
/// `1e-4` ("5e-05") or at/above `1e16` ("1e+16", "1e+20"); every other finite
/// float prints with a point ("0.0", "-0.0", "21.0", "2.675").
fn float_repr_has_point(f: f64) -> bool {
    f.is_finite() && (f == 0.0 || (f.abs() >= 1e-4 && f.abs() < 1e16))
}

/// Python's `int(s)` over a digit string. Only `""` can fail here (the string
/// is otherwise built from `str(number)`), which is module bug 3.
fn parse_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>()
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

impl Default for LangRmSurmiran {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangRmSurmiran {
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
        Err(crate::lang_rm::unsupported("rm_surmiran", "to='currency'"))
    }

    fn to_cheque(&self, _val: &bigdecimal::BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_surmiran", "to='cheque'"))
    }

    // cards/merge stay at their trait defaults: the Python class has no base
    // class and never builds self.cards, so splitnum/clean/merge are
    // unreachable. The magnitude guard is the 66-digit one, an OverflowError
    // since #159, reported by maxval() above. See the module docs.
    //
    // negword likewise stays at its default: the class has no negword
    // attribute at all and signs go through MINUS_PREFIX_WORD.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.ordinal(value)
    }

    /// Python's class had no `to_ordinal_num` (AttributeError for every
    /// input), and no ordinal-numeral marker is attested for Romansh in the
    /// module or its tests, so the port raises a clear NotImplementedError
    /// rather than inventing one (#157).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("rm_surmiran", "to='ordinal_num'"))
    }

    /// Python's class had no `to_year` (AttributeError for every input).
    /// `Num2Word_Base.to_year` is the cardinal, which is what the port
    /// returns (#157).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal cardinal path.
    ///
    /// The Python class overrides `to_cardinal` and handles floats inline via
    /// `float_to_words` — it does **not** use `Num2Word_Base.to_cardinal_float`
    /// (it has no base class), no `float2tuple`, and no `pointword`. This
    /// override reproduces that inline float branch, and the `Decimal`
    /// fall-through (see [`Self::cardinal_decimal`]).
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored:
    /// the dispatcher only applies it when `hasattr(converter, "precision")`,
    /// and this class defines no such attribute, so the kwarg is a no-op here.
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
                    // to_cardinal: number < 0 -> MINUS_PREFIX_WORD + to_cardinal(-number).
                    Ok(format!(
                        "{}{}",
                        MINUS_PREFIX_WORD,
                        self.float_to_words(-*value, *precision)?
                    ))
                } else {
                    self.float_to_words(*value, *precision)
                }
            }
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value),
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing. The gate is
    /// `isinstance(number, float)`, **not** `int(number) == number`, so a
    /// whole-valued float still renders through `float_to_words`
    /// (`1.0` -> "en comma nolla") and a whole-valued Decimal crashed
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
                        self.float_to_words(-*value, *precision)?
                    ))
                } else {
                    self.float_to_words(*value, *precision)
                }
            }
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value),
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

    /// `to_ordinal(float/Decimal)` — see [`LangRmSurmiran::float_ordinal`] /
    /// [`LangRmSurmiran::decimal_ordinal`] for the branch-by-branch mapping.
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
        Err(crate::lang_rm::unsupported("rm_surmiran", "to='ordinal_num'"))
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
        Err(crate::lang_rm::unsupported("rm_surmiran", "to='fraction'"))
    }
}
