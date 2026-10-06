//! Port of `lang_ID.py` (Indonesian).
//!
//! Shape: **self-contained**. `Num2Word_ID` is unusual: it inherits from
//! *nothing* (`class Num2Word_ID:` — no `Num2Word_Base`), so there is no
//! inheritance chain to chase and no `cards`/`merge`/`splitnum` machinery at
//! all. Every method in scope is defined locally. Consequently `cards`,
//! `maxval` and `merge` stay at their trait defaults here and are never
//! reached; `MAXVAL` is enforced by hand inside [`Lang::to_cardinal`] instead
//! of by `base.rs`'s `default_to_cardinal`.
//!
//! The algorithm groups the decimal string into 3-digit blocks from the tail,
//! spells each block, and joins them with a scale word (`ribu`, `juta`, …)
//! drawn from `TENS_TO`.
//!
//! # Faithfully reproduced Python bugs
//!
//! All four verified against the interpreter; this is a port, not a rewrite.
//!
//! 1. **The overflow guard is one-sided.** `to_cardinal` tests
//!    `if number >= self.MAXVAL`, *not* `abs(number) >= MAXVAL`. A negative
//!    number can therefore sail past the guard and reach `join`, where the
//!    13th block needs `TENS_TO[36]` — a key that does not exist. So
//!    `to_cardinal(10**36)` raises `OverflowError`, but `to_cardinal(-10**36)`
//!    raises **`KeyError: 36`** from `lang_ID.py:184`. `-(10**35)` and
//!    `-(10**36)+1` (36 digits, 12 blocks, `TENS_TO[33]`) still work fine, so
//!    the crash begins at exactly 37 digits. Modelled in [`join`].
//! 2. **`errmsg_negord` / `errmsg_floatord` carry no format specifier**, yet
//!    `verify_ordinal` does `raise TypeError(self.errmsg_negord % value)`.
//!    `"Cannot treat negative number as ordinal" % -1` raises
//!    `TypeError: not all arguments converted during string formatting`
//!    *while building the message*, so the intended text is never seen. The
//!    raised type is `TypeError` either way — which is all the corpus records
//!    — but the message is the interpreter's, not the author's. Reproduced
//!    verbatim in [`negord_type_error`].
//! 3. `to_ordinal(0)` == `"kenol"` and `to_ordinal(2000)` == `"kedua ribu"`:
//!    the `"ke"` prefix is glued to the whole cardinal with no separator, so
//!    multi-word ordinals read as one run-on token. Only the literal string
//!    `"satu"` is special-cased to `"pertama"`, so `to_ordinal(1000000)` is
//!    `"kesatu juta"` rather than anything idiomatic. Corpus-confirmed.
//! 4. `str_to_number` returns a `Decimal` and `Num2Word_ID` has **neither**
//!    `to_cheque` nor `to_fraction` (it inherits from nothing), so Python
//!    raised `AttributeError` for both. The port raises NotImplementedError
//!    instead: ID has no fraction rules (#217) and no cheque rules (#223);
//!    "1/0" raises ZeroDivisionError like every other language.
//! 5. **`verify_ordinal` rejects floats and negatives with the same broken
//!    `%`-format TypeError as bug 2** — `errmsg_floatord` also carries no
//!    `%s`. So `to_ordinal(2.5)`, `to_ordinal(-3.0)` and
//!    `to_ordinal_num(-1.0)` are all
//!    `TypeError: not all arguments converted during string formatting`,
//!    while `to_ordinal(5.0)` (whole, positive) passes the check and renders
//!    `"kelima koma nol"` off the repr. `-0.0` passes too (`abs(-0.0) ==
//!    -0.0` is numerically true), hence `"kenol koma nol"` / `"ke--0.0"`.
//!
//! # The fractional path
//!
//! `split_by_koma`/`spell_float` implement the fractional path (`str(number)`
//! split on `"."`). `str(int)` never contains a `"."`, so for the *integer*
//! inputs [`Lang::to_cardinal`] handles, `n` always has length 1 and
//! `float_word` is always `""` — that method drops `join`'s `float_part`
//! argument rather than threading it through.
//!
//! Non-integer input arrives instead through [`Lang::to_cardinal_float`], which
//! `num2words(<float|Decimal>, lang="id")` reaches because `Num2Word_ID` has no
//! `to_cardinal_float` of its own and its `to_cardinal` handles floats inline.
//! That override spells `str(number)` verbatim (via [`spell_float`] for the
//! fraction), **not** through `base.float2tuple`, so ID keeps the repr's exact
//! digits where the shared float path would round f64 artefacts differently.
//!
//! Crucially, `to_cardinal` never routes a *whole* float/Decimal to the
//! integer path — there is no `int(value) == value` test anywhere. Every
//! float/Decimal goes through `str(number)`, so `1.0` is `"satu koma nol"`
//! and `Decimal("5.00")` is `"lima koma nol nol"`, while `Decimal("5")`
//! (whose str has no `"."`) is plain `"lima"` purely because `split_by_koma`
//! yields one part. [`Lang::cardinal_float_entry`] therefore forwards
//! straight to [`Lang::to_cardinal_float`] with **no** whole-value shortcut,
//! and `to_year`/`to_ordinal` inherit the same behaviour through it.
//!
//! `to_currency` is the one caller that *can* reach ID's float path, via
//! `to_cardinal(float(right))` on fractional cents. Rather than re-deriving
//! `split_by_koma`/`spell_float`, it delegates to the [`Lang::cardinal_from_decimal`]
//! hook, whose default routes through `floatpath.rs`. The two algorithms are
//! not the same code but do agree over the reachable domain — see
//! [`Lang::to_currency`] for why, and for the one input class where they would
//! not (which is intercepted there instead).
//!
//! # The currency surface
//!
//! `Num2Word_ID` does not inherit `Num2Word_Base`, so it has **none** of the
//! usual currency machinery: no `pluralize`, no `_money_verbose`, no
//! `_cents_verbose`/`_cents_terse`, no `CURRENCY_ADJECTIVES`, no
//! `CURRENCY_PRECISION`, and no `to_cheque`. It defines exactly one method,
//! `to_currency`, which is a self-contained reimplementation. The trait hooks
//! for everything it lacks are therefore left at their defaults — nothing in
//! this file can reach them.
//!
//! `CURRENCY_FORMS` **is** defined on the class but is *dead code*: the body of
//! `to_currency` never reads it, hardcoding `"rupiah"`/`"sen"` instead, so
//! Python printed rupiah for every code. The port accepts IDR only and raises
//! NotImplementedError for any other code (#219); the [`Lang::currency_forms`]
//! hook stays unimplemented.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

/// `ZERO`.
const ZERO: &str = "nol";
/// `DECIMAL_SEPARATOR`. `spell_float` joins it between the integer words and
/// the fractional digits — e.g. `"nol koma lima"` for `0.5`.
const DECIMAL_SEPARATOR: &str = "koma";
/// `MINUS_SIGN`. Note `to_cardinal` emits `MINUS_SIGN + " "`, i.e. `"min "`.
/// (`to_currency` uses a hardcoded `"minus "` instead — different word, out of
/// scope, but it explains the `minus …` strings in the currency corpus rows.)
const MINUS_SIGN: &str = "min";

/// `BASE`, keys 0..=9. Key 0 maps to the **empty list**, not to a word — the
/// caller is expected to have handled zero already. `None` models that: it
/// contributes no words, exactly like Python's `[]` under `+`.
const BASE: [Option<&str>; 10] = [
    None, // BASE[0] == [] — contributes nothing when concatenated
    Some("satu"),
    Some("dua"),
    Some("tiga"),
    Some("empat"),
    Some("lima"),
    Some("enam"),
    Some("tujuh"),
    Some("delapan"),
    Some("sembilan"),
];

/// `TENS_TO`: power-of-ten exponent → scale word. Keys are 3..=33 step 3.
/// A key of 36 or more is a `KeyError` — see bug 1 in the module docs.
fn tens_to(exp: usize) -> Option<&'static str> {
    Some(match exp {
        3 => "ribu",
        6 => "juta",
        9 => "miliar",
        12 => "triliun",
        15 => "kuadriliun",
        18 => "kuantiliun",
        21 => "sekstiliun",
        24 => "septiliun",
        27 => "oktiliun",
        30 => "noniliun",
        33 => "desiliun",
        _ => return None,
    })
}

/// `MAXVAL` == 10**36.
fn maxval() -> BigInt {
    BigInt::from(10u8).pow(36)
}

/// `TypeError(self.errmsg_negord % value)` / `TypeError(self.errmsg_floatord
/// % value)`.
///
/// The message is NOT `errmsg_negord`/`errmsg_floatord`. Neither format
/// string has a `%s`, so the `%` operator raises before `TypeError` is ever
/// constructed — both `verify_ordinal` failure arms surface the identical
/// interpreter message. See bug 2 in the module docs.
fn negord_type_error() -> N2WError {
    N2WError::Type("not all arguments converted during string formatting".to_string())
}

/// `BASE[int(c)]` for a single digit character.
///
/// Python would raise `KeyError`/`ValueError` on a non-digit, but every caller
/// feeds this a character taken from `str(abs(int))`, so it is always 0..=9.
fn base_of(c: char) -> Option<&'static str> {
    BASE[c.to_digit(10).expect("digit from str(int)") as usize]
}

/// Port of `Num2Word_ID.ratus`. `number` is a single character.
fn ratus(c: char) -> Vec<String> {
    match c {
        '1' => vec!["seratus".to_string()],
        '0' => vec![],
        _ => {
            let mut v: Vec<String> = base_of(c).into_iter().map(str::to_string).collect();
            v.push("ratus".to_string());
            v
        }
    }
}

/// Port of `Num2Word_ID.puluh`. `number` is a two-character slice.
///
/// The `number[0] == "0"` arm returns `BASE[int(number[1])]`, which is `[]`
/// for "00" — that empty spelling is what makes `join` skip a block's scale
/// word entirely (e.g. the `"000"` block of 1_000_000 emits no `"ribu"`).
fn puluh(pair: &[char]) -> Vec<String> {
    let (a, b) = (pair[0], pair[1]);
    if a == '1' {
        match b {
            '0' => vec!["sepuluh".to_string()],
            '1' => vec!["sebelas".to_string()],
            _ => {
                let mut v: Vec<String> = base_of(b).into_iter().map(str::to_string).collect();
                v.push("belas".to_string());
                v
            }
        }
    } else if a == '0' {
        base_of(b).into_iter().map(str::to_string).collect()
    } else {
        let mut v: Vec<String> = base_of(a).into_iter().map(str::to_string).collect();
        v.push("puluh".to_string());
        v.extend(base_of(b).into_iter().map(str::to_string));
        v
    }
}

/// Port of `Num2Word_ID.spell_float`: spell the fractional digit string.
///
/// Python builds `" ".join(["", DECIMAL_SEPARATOR] + word_list)`, so the
/// result carries a leading space and the `"koma"` separator, e.g. `"5"` →
/// `" koma lima"` and `"01"` → `" koma nol satu"`. `to_cardinal_float`
/// concatenates it straight onto the integer words, exactly as Python's `join`
/// appends its `float_part` (`" ".join(word_list) + float_part`).
///
/// Each digit maps to its `BASE` word, with `'0'` → `ZERO` ("nol"). Unlike the
/// integer path this walks the fractional part `str(number)` produced, but the
/// caller has already rejected any non-digit (an exponent `'e'`, etc.), so
/// `base_of` here only ever sees `'1'..'9'`.
fn spell_float(frac: &str) -> String {
    // ["", DECIMAL_SEPARATOR] + word_list, then " ".join(...).
    let mut parts: Vec<String> = vec![String::new(), DECIMAL_SEPARATOR.to_string()];
    for c in frac.chars() {
        if c == '0' {
            // Python: word_list += [self.ZERO]
            parts.push(ZERO.to_string());
        } else {
            // Python: word_list += self.BASE[int(n)] — a single-word list.
            parts.extend(base_of(c).map(str::to_string));
        }
    }
    parts.join(" ")
}

/// Port of `Num2Word_ID.split_by_3`: group the digit string into 3-char blocks
/// from the tail. `'1234567'` → `["1", "234", "567"]`.
///
/// Python returns a tuple of 1-tuples and `spell` later grows each into a
/// 2-tuple; the nesting carries no information (unlike `base.rs`'s `Node`, it
/// is never branched on), so a flat `Vec<String>` is equivalent.
///
/// Note the `length < 3` short-circuit returns the string whole — so a 1- or
/// 2-digit input yields one block that is NOT 3 characters wide. `spell`
/// handles that via its first-block length check.
fn split_by_3(number: &str) -> Vec<String> {
    let chars: Vec<char> = number.chars().collect();
    let length = chars.len();
    let mut blocks: Vec<String> = Vec::new();

    if length < 3 {
        blocks.push(number.to_string());
    } else {
        let len_of_first_block = length % 3;
        if len_of_first_block > 0 {
            blocks.push(chars[0..len_of_first_block].iter().collect());
        }
        let mut i = len_of_first_block;
        while i < length {
            // (length - len_of_first_block) is a multiple of 3, so every
            // block from here on is exactly 3 chars — `spell` relies on it.
            blocks.push(chars[i..(i + 3).min(length)].iter().collect());
            i += 3;
        }
    }
    blocks
}

/// Port of `Num2Word_ID.spell`: pair each block with its word list.
///
/// The first block is special-cased on width (1, 2 or 3+ chars) because
/// `split_by_3` may hand back a short leading block; every later block is
/// unconditionally treated as 3 chars.
fn spell(blocks: Vec<String>) -> Vec<(String, Vec<String>)> {
    let mut word_blocks: Vec<(String, Vec<String>)> = Vec::new();

    let first = &blocks[0];
    let fchars: Vec<char> = first.chars().collect();
    let spelling: Vec<String> = if fchars.len() == 1 {
        if fchars[0] == '0' {
            vec![ZERO.to_string()]
        } else {
            base_of(fchars[0]).into_iter().map(str::to_string).collect()
        }
    } else if fchars.len() == 2 {
        puluh(&fchars)
    } else {
        let mut v = ratus(fchars[0]);
        v.extend(puluh(&fchars[1..3]));
        v
    };
    word_blocks.push((first.clone(), spelling));

    for block in blocks.iter().skip(1) {
        let bchars: Vec<char> = block.chars().collect();
        let mut v = ratus(bchars[0]);
        v.extend(puluh(&bchars[1..3]));
        word_blocks.push((block.clone(), v));
    }

    word_blocks
}

/// Port of `Num2Word_ID.join`.
///
/// Two subtleties are load-bearing:
///
/// * The `"seribu"` special case fires only when there are exactly **two**
///   blocks and the first block's *digit string* is `"1"` — i.e. only for
///   1000..=1999. 10_000 leads with the block `"10"` and 100_000 with `"100"`,
///   neither of which equals `"1"`, so they stay `"sepuluh ribu"` /
///   `"seratus ribu"`. 1_000_000 has three blocks, hence `"satu juta"`.
/// * `if not word_blocks[i][1]: continue` runs *after* the (no-op) extend, and
///   skips **both** the scale word and the `i == length` break. That is what
///   makes an all-zero block swallow its own scale word: 1_000_001 →
///   `"satu juta satu"` with no `"ribu"`.
///
/// Python's `float_part` argument is always `""` for integer input, so it is
/// not modelled — see the module docs.
fn join(word_blocks: &[(String, Vec<String>)]) -> Result<String> {
    let mut word_list: Vec<String> = Vec::new();
    let length = word_blocks.len() - 1;
    let mut start = 0usize;

    if length == 1 && word_blocks[0].0 == "1" {
        word_list.push("seribu".to_string());
        start = 1;
    }

    for i in start..=length {
        word_list.extend(word_blocks[i].1.iter().cloned());
        if word_blocks[i].1.is_empty() {
            continue;
        }
        if i == length {
            break;
        }
        // TENS_TO[(length - i) * 3]: KeyError past 33 — reachable only for
        // negatives with >= 37 digits, which the one-sided MAXVAL guard in
        // to_cardinal lets through. See bug 1 in the module docs.
        let key = (length - i) * 3;
        let scale = tens_to(key).ok_or_else(|| N2WError::Key(key.to_string()))?;
        word_list.push(scale.to_string());
    }

    Ok(word_list.join(" "))
}

/// Port of `Num2Word_ID.verify_ordinal`, integer path.
///
/// `if not value == int(value)` (the float check) is vacuously true for
/// integers and so is not modelled. The negative check raises `TypeError` —
/// but see bug 2 for *which* TypeError.
fn verify_ordinal(value: &BigInt) -> Result<()> {
    if value.is_negative() {
        return Err(negord_type_error());
    }
    Ok(())
}

/// Port of `Num2Word_ID.verify_ordinal`, float/Decimal path.
///
/// Both checks raise the same interpreter TypeError (bug 2 / bug 5):
///
/// * `if not value == int(value)` — `int()` truncates, so any fractional
///   value fails here first (`2.5`, `-1.5`, `Decimal("1.50")`). NaN/inf
///   cannot reach this: the dispatcher keeps non-finite floats on the Python
///   side, and the shim never builds a non-finite `FloatValue`.
/// * `if not abs(value) == value` — a *numeric* comparison, so `-0.0` passes
///   (`abs(-0.0) == -0.0` is True: IEEE zeros compare equal) and only
///   genuinely negative values fail. That is what makes
///   `to_ordinal(-0.0)` == `"kenol koma nol"` and
///   `to_ordinal_num(-0.0)` == `"ke--0.0"` while `to_ordinal(-1.0)` raises.
fn verify_ordinal_float(value: &FloatValue) -> Result<()> {
    if value.as_whole_int().is_none() {
        return Err(negord_type_error());
    }
    // Python `value < 0`, NOT the sign bit: `FloatValue::is_negative()` is
    // sign-bit aware and would wrongly reject -0.0.
    let negative = match value {
        FloatValue::Float { value: f, .. } => *f < 0.0,
        FloatValue::Decimal { value: d, .. } => d.is_negative(),
    };
    if negative {
        return Err(negord_type_error());
    }
    Ok(())
}

pub struct LangId;

impl Default for LangId {
    fn default() -> Self {
        Self::new()
    }
}

impl LangId {
    pub fn new() -> Self {
        LangId
    }
}

impl Lang for LangId {

    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        // Python class attribute MAXVAL (self-contained converter).
        Some(num_bigint::BigInt::from(10u32).pow(36))
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "IDR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        MINUS_SIGN
    }

    fn pointword(&self) -> &str {
        // DECIMAL_SEPARATOR; unused on the integer path.
        "koma"
    }

    /// Port of `Num2Word_ID.to_cardinal`, integer path only.
    ///
    /// The guard is `number >= MAXVAL` — **not** `abs(number) >= MAXVAL`, so
    /// large negatives fall through to a `KeyError` in [`join`] instead. This
    /// asymmetry is bug 1; it is reproduced, not repaired.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let max = maxval();
        if value >= &max {
            // errmsg_toobig % (number, MAXVAL) — `number` is the signed input,
            // interpolated inside the literal text "abs(...)". Only reachable
            // for positive values, so the sign never shows.
            return Err(N2WError::Overflow(format!(
                "Number is too large to convert to words (abs({}) > {}).",
                value, max
            )));
        }

        let minus = if value.is_negative() {
            format!("{} ", MINUS_SIGN)
        } else {
            String::new()
        };

        // split_by_koma(abs(number)) — str(int) never contains ".", so this is
        // always a single element and float_word stays "".
        let digits = value.abs().to_string();
        Ok(format!("{}{}", minus, join(&spell(split_by_3(&digits)))?))
    }

    /// Port of `Num2Word_ID.to_ordinal`.
    ///
    /// The `"satu"` → `"pertama"` test compares the *whole* cardinal string,
    /// so it fires for 1 alone; everything else gets a bare `"ke"` prefix with
    /// no separator (`0` → `"kenol"`, `2000` → `"kedua ribu"`).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        let out_word = self.to_cardinal(value)?;
        if out_word == "satu" {
            return Ok("pertama".to_string());
        }
        Ok(format!("ke{}", out_word))
    }

    /// Port of `Num2Word_ID.to_ordinal_num`: `"ke-" + str(number)`.
    /// Overrides the trait default, which would return the bare digits.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        Ok(format!("ke-{}", value))
    }

    /// Port of `Num2Word_ID.to_year`: delegates straight to `to_cardinal`
    /// with no era/century handling, so negatives read `"min lima ratus"`
    /// rather than anything BC-flavoured. Matches the trait default, but is
    /// spelled out because `lang_ID.py` defines it explicitly.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_ID.to_cardinal` for float / Decimal input.
    ///
    /// `Num2Word_ID` has **no** `to_cardinal_float`; its plain `to_cardinal`
    /// is what the dispatcher calls when `num2words(<float|Decimal>, lang="id")`
    /// skips the integer Rust fast path. That method spells the number straight
    /// off `str(number)` — it splits on `"."`, spells the integer part through
    /// the same `split_by_3`/`spell`/`join` machinery as the integer path, and
    /// spells each fractional *character* via [`spell_float`]. It never touches
    /// `base.float2tuple`, so the f64-artefact rounding heuristic does **not**
    /// apply here: the digits come verbatim from the repr. That is the whole
    /// reason ID cannot inherit `default_to_cardinal_float` — e.g. `1.1 * 1.1`
    /// has repr `1.2100000000000002`, and ID ends it `"… nol dua"` (repr's last
    /// digit) whereas float2tuple's floor branch would end it `"… nol satu"`.
    ///
    /// `Num2Word_ID` defines no `precision` attribute, so the dispatcher's
    /// `precision=` handling (`if hasattr(converter, "precision")`) is a no-op
    /// for it; `precision_override` is therefore ignored.
    ///
    /// The reconstruction of `str(number)`:
    /// * float — Rust's `{:?}` is shortest-round-trip *with* a decimal point,
    ///   matching Python's `repr`/`str` over the whole non-exponent range
    ///   (including the exact `1e-4` / `1e16` switch-to-exponent boundaries).
    /// * Decimal — [`python_decimal_str`] is `Decimal.__str__` verbatim:
    ///   trailing zeros kept (`Decimal("1.10")` → `"1.10"` → `"… satu nol"`)
    ///   and positive exponents in scientific form (`Decimal("1E+2")` →
    ///   `"1E+2"` → ValueError from `int('E')`).
    ///
    /// **Exponent / special reprs.** Once `|x| >= 1e16` or `< 1e-4`, `str(float)`
    /// switches to exponent form (`"1e+16"`, `"1e-05"`); Python then feeds the
    /// `'e'` to `int()` inside `spell`/`spell_float` and raises
    /// `ValueError: invalid literal for int() with base 10: 'e'` *before* any
    /// words are produced. `1e16` and `1e-5` are ordinary reachable inputs, so
    /// this is reproduced (as an [`N2WError::Value`]) rather than left to panic
    /// in `base_of`. Because the mantissa is always digits then `'e'`, the
    /// offending character is invariably `'e'` for floats; the scan below simply
    /// reports the first non-digit it finds, which also covers a `Decimal` that
    /// stringifies with `'E'` and the pathological `inf` case (`str(inf)` →
    /// `"inf"` → `int('i')`).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `if number >= self.MAXVAL: raise OverflowError(errmsg_toobig % ...)`.
        // Same one-sided guard as the integer path (bug 1): it tests the signed
        // value, not abs, and is only reachable for values >= 10**36 — none in
        // the corpus. The message interpolates the signed `number` inside the
        // literal "abs(...)" text, exactly as Python does.
        let max = maxval();
        let over = match value {
            FloatValue::Float { value: f, .. } => *f >= 1e36,
            FloatValue::Decimal { value: d, .. } => *d >= BigDecimal::from(max.clone()),
        };
        if over {
            let signed = match value {
                FloatValue::Float { value: f, .. } => format!("{:?}", f),
                FloatValue::Decimal { value: d, .. } => format!("{}", d),
            };
            return Err(N2WError::Overflow(format!(
                "Number is too large to convert to words (abs({}) > {}).",
                signed, max
            )));
        }

        // minus = MINUS_SIGN + " " if number < 0. (to_cardinal uses "min ",
        // NOT the "minus " that to_currency hardcodes.) Python's `< 0`, not
        // the sign bit: `-0.0 < 0` is False, so -0.0 renders "nol koma nol"
        // with no negword — `FloatValue::is_negative()` would get this wrong.
        // (A BigDecimal cannot carry -0 at all; the shim already converts
        // Decimal("-0.0") to the Float arm.)
        let minus = match value {
            FloatValue::Float { value: f, .. } if *f < 0.0 => format!("{} ", MINUS_SIGN),
            FloatValue::Decimal { value: d, .. } if d.is_negative() => {
                format!("{} ", MINUS_SIGN)
            }
            _ => String::new(),
        };

        // n = split_by_koma(abs(number)) == str(abs(number)).split(".").
        let s = match value {
            FloatValue::Float { value: f, .. } => format!("{:?}", f.abs()),
            FloatValue::Decimal { value: d, .. } => {
                // str(abs(Decimal)) == str(Decimal) minus the leading sign.
                // python_decimal_str is Python's exact Decimal.__str__: it
                // keeps the stored scale (str(Decimal("0.00")) == "0.00" ->
                // "nol koma nol nol", str(Decimal("1.10")) == "1.10" ->
                // "… satu nol") AND reproduces the scientific form for a
                // positive exponent (str(Decimal("1e3")) == "1E+3"), whose
                // 'E' the scan below turns into Python's ValueError.
                // BigDecimal's own Display would expand "1E+3" to "1000"
                // and silently spell "seribu" where Python raises.
                let disp = python_decimal_str(d);
                disp.strip_prefix('-').unwrap_or(&disp).to_string()
            }
        };

        // Exponent form ('e'/'E') or a special value ('inf'/'nan') puts a
        // non-digit in the string; Python's int() dies on the first such
        // character. Report it here rather than panicking in base_of. For every
        // reachable float this character is 'e'.
        if let Some(bad) = s.chars().find(|c| *c != '.' && !c.is_ascii_digit()) {
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                bad
            )));
        }

        // if len(n) == 2: float_word = spell_float(n[1]). str(number) has at
        // most one ".", so split_once mirrors split(".") exactly here.
        let (int_part, float_word) = match s.split_once('.') {
            Some((int_part, frac)) => (int_part, spell_float(frac)),
            None => (s.as_str(), String::new()),
        };

        // minus + join(spell(split_by_3(n[0])), float_word). `join` returns the
        // integer words; Python appends float_part to them, so concatenating
        // float_word here reproduces `" ".join(word_list) + float_part`.
        Ok(format!(
            "{}{}{}",
            minus,
            join(&spell(split_by_3(int_part)))?,
            float_word
        ))
    }

    // ---- float/Decimal routing -------------------------------------------

    /// `to_cardinal(float/Decimal)`, full routing. `lang_ID.py` has exactly
    /// one `to_cardinal` and it spells `str(number)` — there is no
    /// `int(value) == value` shortcut anywhere, so whole values keep their
    /// visible decimals: `1.0` → `"satu koma nol"`, `Decimal("5.00")` →
    /// `"lima koma nol nol"`. The trait default's whole→integer routing
    /// would drop the `"koma nol"` tail, so this forwards everything to
    /// [`Lang::to_cardinal_float`] unconditionally. (A pointless int-path
    /// detour is also avoided for `Decimal("5")`: its str has no `"."`, so
    /// the float grammar produces the identical `"lima"` by itself.)
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` (TypeError for
    /// fractional or negative input — bug 5), then the same
    /// `"ke" + to_cardinal(number)` gluing as the integer path. A whole
    /// positive float keeps its repr decimals, so `5.0` is
    /// `"kelima koma nol"` and `1.0` is `"kesatu koma nol"` — the
    /// `"pertama"` special case compares the *whole* cardinal string and
    /// only `Decimal("1")` (str `"1"` → `"satu"`) can still hit it here.
    /// `1e16` passes verification (it *is* whole) and then dies in
    /// `to_cardinal` with the `int('e')` ValueError, exactly like Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        verify_ordinal_float(value)?;
        let out_word = self.to_cardinal_float(value, None)?;
        if out_word == "satu" {
            return Ok("pertama".to_string());
        }
        Ok(format!("ke{}", out_word))
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal`, then
    /// `"ke-" + str(number)`. `repr_str` is Python's `str(value)` computed
    /// by the binding, so the exponent forms survive verbatim
    /// (`"ke-1e+16"`, `"ke-1E+2"`), `Decimal("5.00")` keeps its trailing
    /// zeros (`"ke-5.00"`) and `-0.0` — which passes verification — yields
    /// the double-dash `"ke--0.0"`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        verify_ordinal_float(value)?;
        Ok(format!("ke-{}", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is a bare `to_cardinal` call, so
    /// floats keep their decimals (`1999.0` → `"… koma nol"`) and no
    /// ordinal-style verification runs — `-1.5` is `"min satu koma lima"`,
    /// not a TypeError. Matches what the trait default reaches through this
    /// file's `cardinal_float_entry`, but is spelled out because
    /// `lang_ID.py` defines `to_year` explicitly.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    // ---- string inputs ----------------------------------------------------

    /// `str_to_number` is `Decimal(value)` — the base default — so the
    /// default parse stands, with two adjustments for where the *next* step
    /// diverges from what the shim hardcodes for special values:
    ///
    /// * `"NaN"` — `Decimal("NaN")` parses fine; the first thing
    ///   `to_cardinal` then does is `number >= self.MAXVAL`, and an ordering
    ///   comparison on a NaN Decimal raises `decimal.InvalidOperation`
    ///   (CPython's `_decimal` message is the bare signal-class repr).
    ///   Raising it here is observably identical for the reachable corpus
    ///   (the dispatcher's InvalidOperation catch only reroutes strings
    ///   containing a digit, and `"NaN"` has none).
    /// * `"-Infinity"` — sails past the one-sided `>= MAXVAL` guard
    ///   (bug 1: -inf is not >= 10**36), takes the minus branch, and then
    ///   `str(abs(number))` == `"Infinity"` reaches `puluh("In")` where
    ///   `int('I')` raises ValueError. The shim's generic Inf arm would
    ///   raise OverflowError instead, so the ValueError is produced here.
    ///   *Positive* `"Infinity"` really is `>= MAXVAL` → OverflowError,
    ///   which the shim's Inf arm already delivers — left untouched.
    ///
    /// Both rewrites are keyed to `to_cardinal` (the only mode the corpus
    /// exercises for these strings); `to_ordinal_num("Infinity")` would
    /// raise a differently-worded OverflowError in Python (`int(inf)`), but
    /// the type still matches.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s) {
            Ok(ParsedNumber::NaN) => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            Ok(ParsedNumber::Inf { negative: true }) => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'I'".into(),
            )),
            // Successful parses are served natively.
            other => other,
        }
    }

    // ---- fractions ---------------------------------------------------------

    // `Num2Word_ID` had no `to_fraction` (AttributeError). ID has no fraction
    // rules, so the trait default raises NotImplementedError (#217).

    // ---- currency -------------------------------------------------------
    //
    // Only `lang_name`, `to_currency` and `to_cheque` are overridden. Every
    // other currency hook stays at its trait default because `Num2Word_ID`
    // defines no counterpart and — since `to_currency`/`to_cheque` are
    // wholesale overrides that never delegate to `currency::default_*` — none
    // of them is reachable. See the module docs for why `currency_forms` in
    // particular is left unimplemented despite the class carrying a
    // `CURRENCY_FORMS` table.

    /// Class name. Only ever surfaced through `cardinal_from_decimal`'s default
    /// message (see [`Lang::to_currency`]); ID reaches no other message that
    /// names the class, because it raises NotImplementedError for no currency
    /// code at all.
    fn lang_name(&self) -> &str {
        "Num2Word_ID"
    }

    /// Port of `Num2Word_ID.to_currency`.
    ///
    /// A self-contained reimplementation that shares nothing with
    /// `Num2Word_Base.to_currency`, so `currency::default_to_currency` is
    /// deliberately not delegated to:
    ///
    /// 1. **The unit is always `"rupiah"`.** Python printed rupiah whatever
    ///    `currency` said (`USD 12.34` was `"dua belas rupiah tiga puluh empat
    ///    sen"`) and never raised; the port accepts only IDR and raises
    ///    NotImplementedError for every other code (#219).
    /// 2. **`currency == "IDR"` suppresses the cents segment entirely**, on the
    ///    stated reasoning that the rupiah has no practical subunit. So
    ///    `IDR 12.34` is `"dua belas rupiah"`. With only IDR accepted, the
    ///    Python "rupiah … sen" branch for other codes is gone.
    /// 3. **The negative word is `"minus "`, not `MINUS_SIGN`** (`"min "`),
    ///    which `to_cardinal` uses. `to_currency` hardcodes its own.
    /// 4. **`separator`, `adjective` and `cents` are ignored.**
    ///
    /// `has_decimal` is unused — ID never branches on it, so `Decimal("5")` and
    /// `Decimal("5.00")` both give `"lima rupiah"` where Base would split them.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Every branch says "rupiah" (#219): any other code would print the
        // wrong currency, so it raises like an unknown code in base.
        if currency != "IDR" {
            return Err(crate::currency::unknown_currency(self, currency));
        }
        // has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0.
        // Hardcoded 100, not currency_precision() — see the doc comment.
        // `str(int) * 100` can never have a remainder, so ints are always
        // False. It only decides the rounding of the whole part below.
        let has_fractional_cents = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => {
                let scaled = value * BigDecimal::from(100);
                // Decimal's `%` truncates toward zero, as with_scale(0) does;
                // the `!= 0` test is insensitive to the tie rule either way.
                &scaled - scaled.with_scale(0) != BigDecimal::zero()
            }
        };

        // is_int_with_cents=False, keep_precision=has_fractional_cents,
        // divisor defaulted to 100 on the Python side.
        let (left, _right, is_negative) =
            parse_currency_parts(val, false, has_fractional_cents, 100);

        let minus_str = if is_negative { "minus " } else { "" };
        let money_str = self.to_cardinal(&left)?;

        // `currency == "IDR"` suppresses the cents segment: the rupiah has no
        // practical subunit, so whole rupiah are read for every input.
        Ok(format!("{}{} rupiah", minus_str, money_str))
    }

    // `Num2Word_ID` had no `to_cheque` (AttributeError).
    // No cheque rules, so NotImplementedError ("lang='id' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}
