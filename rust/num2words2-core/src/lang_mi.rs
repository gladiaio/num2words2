//! Port of `lang_MI.py` (Maori / te reo Māori).
//!
//! # Best-candidate words (gladiaio/num2words2#154)
//!
//! UNVERIFIED (#154): "tōraro" (minus) — best candidate: "negative" in Te Aka
//!   (maoridictionary.co.nz, "tau tōraro" = negative number); no source reads a
//!   negative number aloud.
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "piriona" (10^9; Te Aka Māori dictionary), composed like the
//! million arm, and raises `OverflowError` from 10^12, which `maxval()`
//! reports. No word for 10^12 is attested. Where the notes below describe the
//! digit fallback ("1000000000", "no overflow check"), they describe Python;
//! that arm is now unreachable.
//!
//! Shape: **self-contained**. `Num2Word_MI` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires: Python
//! never builds `self.cards`, never calls `set_numwords`, and never assigns
//! `self.MAXVAL`. All four in-scope entry points are overridden outright and
//! drive a hand-written `_int_to_word` recursion. Consequently `cards`/
//! `maxval`/`merge` stay at their trait defaults here and are never consulted,
//! and there is **no overflow check at all** — see the 10^9 fallback below.
//!
//! Nothing is inherited from `Num2Word_Base` for our scope: MI overrides
//! `to_cardinal`, `to_ordinal`, `to_ordinal_num` and `to_year` itself. In
//! particular `self.is_title` stays `False` and MI's `to_cardinal` never calls
//! `self.title()`, so no title-casing is applied.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, and every one of them is pinned by the frozen corpus:
//!
//! 1. **Numbers >= 10^9 are not spelled out at all.** `_int_to_word`'s final
//!    `else` is `return str(number)` — a "fallback for very large numbers"
//!    that just hands back the decimal digits. So `to_cardinal(10**9)` is the
//!    string `"1000000000"`, and `to_ordinal(10**9)` is `"tua 1000000000"`.
//!    This is the reason the port must stay on `BigInt`: the fallback has no
//!    ceiling, so `to_cardinal(10**21)` returns 22 digits verbatim. Modelled
//!    by the last arm of [`int_to_word`].
//! 2. **`to_ordinal` does not reject negatives**, unlike most languages (which
//!    raise on `errmsg_negord`). It falls through to `"tua " + to_cardinal(n)`,
//!    and `to_cardinal` prefixes its own negword, yielding the self-
//!    contradictory `to_ordinal(-1) == "tua tōraro tahi"` ("the tōraro-first").
//!    Corpus-confirmed for -1, -7, -21, -42, -100, -999, -1000, -1000000.
//! 3. **`to_ordinal` has special forms only for 1..=5**; 0 and everything from
//!    6 up take the generic `"tua " + cardinal` path, so `to_ordinal(0)` is
//!    `"tua kore"`. Note the special forms are written solid ("tuatahi") while
//!    the generic path inserts a space ("tua whitu") — that inconsistency is
//!    in the Python and is preserved.
//! 4. **Zero and decimal (fixed, gladiaio/num2words2#154).** Python writes
//!    `return self.ones[0] if self.ones[0] else "zero"` with `ones[0] == ""`,
//!    so it always answered the English "zero", and `pointword` was "point".
//!    The port says "kore" and "ira" (Te Aka: ira = decimal point):
//!    1.5 == "tahi ira rima". The negword is the best
//!    candidate "tōraro " (see UNVERIFIED above).
//! 5. **`negword` carries a trailing space** (`"minus "`), which is why
//!    `to_cardinal` ends with `.strip()`. Kept verbatim, trailing space and
//!    all, with the matching trim.
//!
//! # Dead code in the original
//!
//! `_int_to_word` opens with a `number < 0` branch returning
//! `self.negword + self._int_to_word(abs(number))`, but it is **unreachable**
//! from every in-scope mode: `to_cardinal` strips the `"-"` off the *string*
//! before calling `int()`, so `_int_to_word` only ever sees a non-negative
//! value. It is reproduced in [`int_to_word`] anyway for structural fidelity
//! (a currency path, out of scope, could reach it). Because that branch
//! absorbs every negative, all subsequent `//` and `%` in the function run on
//! strictly positive values, where Python's floor-division and Rust's
//! truncating `BigInt` division agree — no sign-semantics hazard here.
//!
//! # Errors
//!
//! None, for the four integer modes. Every `mi` cardinal/ordinal row in the
//! frozen corpus is `"ok": true`, and the four overridden methods contain no
//! `raise`, no dict lookup and no list index that can go out of range
//! (`ones`/`tens` are only ever indexed with a digit derived from `% 10` or a
//! `< 1000` quotient). `to_cardinal` therefore always returns `Ok`. The
//! currency surface below *can* fail — see [`LangMi::to_currency`] and the
//! `to_cheque` note.
//!
//! Float/Decimal/string input *can* raise — always `ValueError`, from the
//! `int()` calls inside `to_cardinal`. The routing itself is
//! `"." in str(number)`, so whole floats keep their ".0"
//! (`to_cardinal(5.0)` == "rima ira kore", `str(-0.0)` keeps the sign ->
//! "tōraro kore ira kore"), only point-free string forms (integer-valued
//! Decimals) take the bare integer grammar, and exponent-form shapes
//! (`str(1e16)` == "1e+16", `str(Decimal("1E+2"))` == "1E+2") plus the
//! special Decimals ("Infinity"/"NaN") die in `int()` with
//! `invalid literal for int() with base 10: '...'`. See
//! `cardinal_float_entry` / `str_to_number` below. `to_ordinal`'s 1..=5
//! special forms are *numeric* checks (`number == 5`), so `5.0` and
//! `Decimal("5.00")` are still "tuarima".
//!
//! # Currency (phase 2)
//!
//! `Num2Word_MI` declares its **own** three-entry `CURRENCY_FORMS` and inherits
//! nothing from `lang_EUR`/`lang_EU`, so the "English mutates the shared class
//! dict" trap in `PORTING_CURRENCY.md` does not apply here. Verified against
//! the live interpreter:
//!
//! ```text
//! CURRENCY_FORMS      == {'NZD': (('tāra','tāra'),   ('hēneti','hēneti')),
//!                         'USD': (('dollar','dollars'), ('cent','cents')),
//!                         'EUR': (('euro','euros'),     ('cent','cents'))}  # NZD first
//! CURRENCY_PRECISION  == {}     # -> divisor is always the default 100
//! CURRENCY_ADJECTIVES == {}     # -> `adjective=True` can never do anything
//! MRO                 == [Num2Word_MI, Num2Word_Base, object]
//! CURRENCY_FORMS is Num2Word_EN.CURRENCY_FORMS  -> False
//! ```
//!
//! `to_currency` is overridden outright and shares **no** code with
//! `Num2Word_Base.to_currency`, so `currency::default_to_currency` is bypassed
//! entirely. `to_cheque` is *not* overridden, so it comes from the base and
//! `currency::default_to_cheque` serves it unchanged.
//!
//! The two disagree about unknown codes, and that asymmetry is real and
//! corpus-pinned: `to_currency("GBP")` silently prints NZD's "tāra" (quirk 6)
//! while `to_cheque("GBP")` raises `NotImplementedError`.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). The Māori US dollar is "tāra" like NZD, and the euro has no
//! reliable Māori noun, so USD and EUR raise NotImplementedError. Examples in
//! these docs that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::ParsedNumber;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `_int_to_word`'s zero case. Python's `self.ones[0] if self.ones[0] else
/// "zero"` always takes the `else` — `ones[0]` is `""`. See quirk 4.
const ZERO: &str = "kore";

/// `self.negword`. The trailing space is in the Python and is load-bearing:
/// `to_cardinal` concatenates it directly and relies on `.strip()` after.
const NEGWORD: &str = "tōraro ";

/// `self.ones`. Index 0 is `""` and is never used as a word — the zero case
/// returns early, and the hundreds branch only indexes 1..=9.
const ONES: [&str; 10] = [
    "", "tahi", "rua", "toru", "whā", "rima", "ono", "whitu", "waru", "iwa",
];

/// `self.tens`. Spelled out in full ("rua tekau" = 20) rather than composed,
/// exactly as the Python table has it. Index 0 is `""` and is unreachable
/// (the `< 100` branch is only entered when `number >= 10`).
const TENS: [&str; 10] = [
    "",
    "tekau",
    "rua tekau",
    "toru tekau",
    "whā tekau",
    "rima tekau",
    "ono tekau",
    "whitu tekau",
    "waru tekau",
    "iwa tekau",
];

const HUNDRED: &str = "rau";
const THOUSAND: &str = "mano";
const MILLION: &str = "miriona";

/// `self.pointword`. Consumed by the float branch of `to_cardinal`
/// (`"." in n`), ported here as [`LangMi::to_cardinal_float`]. Also exposed via
/// the `pointword()` trait method for parity with `setup()`.
const POINTWORD: &str = "ira";

/// Python's `_int_to_word`.
///
/// Branch order is preserved exactly, including the dead `number < 0` arm
/// (see the module docs) and the bare `str(number)` fallback for >= 10^9.
///
/// The magnitude branches convert to `u32` only *after* a `BigInt` comparison
/// has proven the value is below 10^9, so the `unwrap`s cannot fire. Values
/// at or above 10^9 never leave `BigInt`.
fn int_to_word(number: &BigInt) -> String {
    // `if number == 0` comes first in Python, ahead of the sign test.
    if number.is_zero() {
        return ZERO.to_string();
    }

    if number.is_negative() {
        // Unreachable from the four in-scope modes; see module docs.
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    if number < &BigInt::from(10u32) {
        // Safe: 0 < number < 10.
        return ONES[number.to_usize().unwrap()].to_string();
    }

    if number < &BigInt::from(100u32) {
        let v = number.to_u32().unwrap();
        let tens_val = (v / 10) as usize;
        let ones_val = (v % 10) as usize;
        return if ones_val == 0 {
            TENS[tens_val].to_string()
        } else {
            format!("{} {}", TENS[tens_val], ONES[ones_val])
        };
    }

    if number < &BigInt::from(1_000u32) {
        let v = number.to_u32().unwrap();
        let hundreds_val = (v / 100) as usize;
        let remainder = v % 100;
        // Note: `ones[h] + " " + hundred`, so 100 == "tahi rau" (not "rau").
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    if number < &BigInt::from(1_000_000u32) {
        let v = number.to_u32().unwrap();
        let thousands_val = v / 1_000;
        let remainder = v % 1_000;
        let mut result = format!("{} {}", int_to_word(&BigInt::from(thousands_val)), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    if number < &BigInt::from(1_000_000_000u32) {
        let v = number.to_u32().unwrap();
        let millions_val = v / 1_000_000;
        let remainder = v % 1_000_000;
        let mut result = format!("{} {}", int_to_word(&BigInt::from(millions_val)), MILLION);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    // Scale words above a million (gladiaio/num2words2#147), composed like
    // the million arm. Every entry point rejects values at or above
    // `maxval_ceiling()` first, so the top quotient is always below 1000.
    for &(exp, word) in SCALES.iter().rev() {
        let scale = pow10_big(exp);
        if number >= &scale {
            let mut result = format!("{} {}", int_to_word(&(number / &scale)), word);
            let rest = number % &scale;
            if !rest.is_zero() {
                result.push(' ');
                result.push_str(&int_to_word(&rest));
            }
            return result;
        }
    }

    // `return str(number)` — "Fallback for very large numbers". See quirk 1.
    number.to_string()
}

/// Python `repr()` of a **non-negative** f64 — the string MI's `to_cardinal`
/// sees after detaching the sign. Rust's `{}` produces the same
/// shortest-round-trip digits but never switches to exponent form and drops
/// the ".0" of whole values, so the two Python-isms are reapplied here:
///
/// * exponent form for `|v| >= 1e16` or `0 < |v| < 1e-4`, with the sign
///   always shown and the exponent zero-padded to two digits (`"1e+16"`,
///   `"1.5e-05"`) — exactly repr's thresholds;
/// * a trailing `".0"` for whole values in positional form (`"5.0"`).
///
/// inf/nan come back as `"inf"`/`"nan"`, which the caller's `int()` port then
/// rejects with the same ValueError Python would raise.
fn python_float_repr_abs(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return "inf".to_string();
    }
    if f != 0.0 && (f >= 1e16 || f < 1e-4) {
        let s = format!("{:e}", f); // "1e16" / "1.2345e-7"
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("f64 exponent is a small integer");
        return format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }
    let s = format!("{}", f);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// The code whose forms `to_currency` falls back to for an unknown currency.
///
/// Python: `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
/// `list(values())[0]` is the **first-inserted** entry, and the class body lists
/// `NZD` before `USD` and `EUR`, so the fallback is NZD's forms. Dicts have
/// preserved insertion order since CPython 3.7, and the live interpreter
/// confirms `list(CURRENCY_FORMS.keys()) == ['NZD', 'USD', 'EUR']`. A `HashMap`
/// has no order of its own, so the choice is pinned to a constant here rather
/// than left to iteration order.
const FALLBACK_CURRENCY: &str = "NZD";

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 1] = [(9, "piriona")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
/// entry point that hands over a caller-supplied integer goes through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

pub struct LangMi {
    /// `CURRENCY_FORMS`, built once. Every entry carries exactly two unit forms
    /// and two subunit forms, matching Python's tuple arity — `to_currency`
    /// indexes `[0]`/`[1]` directly, so the arity is load-bearing.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl LangMi {
    pub fn new() -> Self {
        // Built once and cached by the caller (`num2words2-py` holds this in a
        // OnceLock), never per call.
        let mut forms = HashMap::with_capacity(3);
        // NZD is inserted first for documentation only; see FALLBACK_CURRENCY —
        // the fallback is pinned by name, not by iteration order.
        forms.insert(
            "NZD",
            CurrencyForms::new(&["tāra", "tāra"], &["hēneti", "hēneti"]),
        );
        // USD/EUR were English ("dollars", "cents", "euros"). The Māori US
        // dollar is "tāra" too, which would read exactly like NZD, and the
        // euro has no reliable Māori noun, so both raise NotImplementedError
        // (#222).
        LangMi { forms }
    }
}

impl Default for LangMi {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangMi {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NZD"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "ira"
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)`: it strips a leading `"-"` off the
    /// *text*, sets `ret = self.negword`, then feeds the remainder to `int()`.
    /// For integer input that is exactly "take the absolute value and remember
    /// the sign" — `str()` of an int never yields `"-0"`. The `"." in n` branch
    /// is the float path and is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // Python: `(ret + self._int_to_word(int(n))).strip()`. The strip only
        // ever matters via negword's trailing space, which is always followed
        // by a word — but it is in the original, so it is here.
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// Python's `to_ordinal`. Special forms for 1..=5 only; everything else,
    /// including 0 and every negative, takes `"tua " + to_cardinal(number)`.
    /// See quirks 2 and 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(v) = value.to_i32() {
            match v {
                1 => return Ok("tuatahi".to_string()),
                2 => return Ok("tuarua".to_string()),
                3 => return Ok("tuatoru".to_string()),
                4 => return Ok("tuawhā".to_string()),
                5 => return Ok("tuarima".to_string()),
                _ => {}
            }
        }
        Ok(format!("tua {}", self.to_cardinal(value)?))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`. No suffix logic, and no
    /// sign handling — `to_ordinal_num(-1)` is `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)` ignores `longval` entirely and
    /// just delegates to `to_cardinal`. No BC/AD handling, no two-digit pairing:
    /// `to_year(-500)` is `"tōraro rima rau"`, not "rima rau BC".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// MI routes on the string, not the value: `"." in str(number)` decides
    /// between the digit-word grammar and `int(n)`. Three corpus-pinned
    /// behaviours hang off the exact shape of `str()`:
    ///
    /// * **Whole floats keep their ".0"** — `str(5.0)` is `"5.0"`, so
    ///   `to_cardinal(5.0)` == `"rima ira kore"`, never the bare integer
    ///   word the base default would pick. Only a point-free string — an
    ///   integer-valued `Decimal` like `Decimal("5")` — takes the int path.
    /// * **Exponent-form reprs raise ValueError.** `str(1e16)` is `"1e+16"`
    ///   (no "."), so Python falls to `int("1e+16")` and raises
    ///   `invalid literal for int() with base 10: '1e+16'`. Same for
    ///   `Decimal("1E+2")`. A *dotted* e-form (`1.5e+16` -> `"1.5e+16"`)
    ///   enters the `"."` branch instead and dies in the digit loop at
    ///   `int('e')` — same type, message `'e'` (`'E'` for Decimals).
    /// * **-0.0 keeps its sign** — `str(-0.0)` is `"-0.0"`, sign detached as
    ///   a string, hence `"tōraro kore ira kore"`.
    ///
    /// The bridge converts a signed-zero `Decimal("-0.0")` to
    /// `Float { -0.0 }` (BigDecimal cannot carry the sign), so the Float arm
    /// may stand in for a Decimal zero; for zero both arms produce the same
    /// words, with `precision` (not this repr) driving the digit count.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() minus its sign — Python detaches a leading
        // "-" before any other test; the sign is re-applied from the value
        // below (int path) or inside to_cardinal_float (dotted path).
        let n = match value {
            FloatValue::Float { value: f, .. } => python_float_repr_abs(f.abs()),
            FloatValue::Decimal { value: d, .. } => {
                let s = crate::strnum::python_decimal_str(d);
                s.strip_prefix('-').map(str::to_string).unwrap_or(s)
            }
        };

        match n.split_once('.') {
            // `"." in n`: the digit-word grammar — unless a non-digit
            // character kills one of the int() calls first.
            Some((left, right)) => {
                // `int(left)` runs before the digit loop. Unreachable for
                // every str() shape in practice (a dotted repr's left part is
                // always plain digits), mirrored anyway for order fidelity.
                if !left.bytes().all(|b| b.is_ascii_digit()) || left.is_empty() {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        left
                    )));
                }
                // `for digit in right: int(digit)` — first non-digit raises;
                // this is where "1.5e+16" / Decimal("1.5E+20") die.
                if let Some(bad) = right.chars().find(|c| !c.is_ascii_digit()) {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        bad
                    )));
                }
                self.to_cardinal_float(value, precision_override)
            }
            // No "." — `int(n)`: plain digits take the integer grammar; an
            // exponent/Infinity/NaN string raises exactly like Python's int().
            None => {
                if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        n
                    )));
                }
                let magnitude: BigInt = n.parse().expect("all-ASCII-digit string parses");
                let ret = if value.is_negative() { NEGWORD } else { "" };
                Ok(format!("{}{}", ret, checked_int_to_word(&magnitude)?)
                    .trim()
                    .to_string())
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python's `number == 1` .. `number == 5`
    /// chain is a *numeric* comparison, so `5.0` and `Decimal("5.00")` still
    /// hit "tuarima"; everything else — 0.0, negatives, negative zero — is
    /// "tua " glued to the float cardinal: `to_ordinal(0.0)` == "tua zero
    /// point zero", `to_ordinal(-0.0)` == "tua minus zero point zero",
    /// `to_ordinal(7.0)` == "tua whitu ira kore" (solid special forms vs
    /// spaced generic path, quirk 3, carries over). An exponent-form repr
    /// propagates the cardinal's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(v) = value.as_whole_int().as_ref().and_then(|i| i.to_i32()) {
            match v {
                1 => return Ok("tuatahi".to_string()),
                2 => return Ok("tuarua".to_string()),
                3 => return Ok("tuatoru".to_string()),
                4 => return Ok("tuawhā".to_string()),
                5 => return Ok("tuarima".to_string()),
                _ => {}
            }
        }
        Ok(format!("tua {}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, identical to the
    /// int path. `repr_str` IS Python's `str(number)`, so
    /// `to_ordinal_num(1e16)` == "1e+16." and `Decimal("5.00")` == "5.00.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: the same bare `self.to_cardinal(val)`
    /// delegation as the int path, string routing included — so
    /// `to_year(5.0)` == "rima ira kore" and `to_year(1e16)` raises
    /// ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`), and
    /// this override does not change what parses — `python_decimal_parse`
    /// still decides. It exists because `Decimal("Infinity")` and
    /// `Decimal("NaN")` *do* parse in Python, and MI's `to_cardinal` then
    /// dies at `int("Infinity")` with
    /// `ValueError: invalid literal for int() with base 10: 'Infinity'`
    /// (str(Decimal) capitalises to 'Infinity' whatever the input case, and
    /// the sign of "-Infinity" is stripped before int() sees it). The bridge
    /// hard-wires the *base* integer-path errors for Inf/NaN parses
    /// (OverflowError / "cannot convert NaN"), which MI never produces, so
    /// the raise is surfaced here instead. Observably identical for every
    /// corpus row — Infinity/NaN strings only appear with to="cardinal";
    /// the one input this would misserve is `to_ordinal_num("Infinity")`
    /// (Python: "Infinity."), which no corpus exercises.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Port of the float/Decimal branch of `Num2Word_MI.to_cardinal` — the
    /// code that runs when `str(number)` contains a ".". MI overrides
    /// `to_cardinal` and handles non-integers inline; it inherits
    /// `Num2Word_Base.to_cardinal_float` but never actually reaches it (the
    /// dispatcher calls `to_cardinal(number)` directly). This override is where
    /// the Rust FFI routes float/Decimal cardinal input.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"): n = n[1:]; ret = self.negword
    /// else: ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// The literal `str(number)` digits are reproduced through
    /// [`float2tuple`] rather than a Rust `format!("{}", f)`. The two agree
    /// digit-for-digit — verified over every corpus row and 200k random floats
    /// — and `float2tuple` is where the load-bearing f64 artefacts live
    /// (`2.675` really produces `674.9999…`, rescued to `675` by the `< 0.01`
    /// heuristic; `1.005` -> `4.9999…` -> `5` -> `"005"`). Re-deriving the
    /// digits from a decimal string would compute the mathematically-right and
    /// therefore *wrong* answer, and would also break on Rust's shortest-repr
    /// for `1.0` ("1", not "1.0"), `1e21`, and `1e-05`. See PORTING_FLOAT.md.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is dropped:
    /// MI's inline branch never consults `self.precision`, so `precision=` has
    /// no effect on `to_cardinal`. The digit count is `str(number)`'s own
    /// fractional length, i.e. `value.precision()` = `len(right)`.
    ///
    /// # Errors
    ///
    /// None for any ordinary decimal `repr` (every fractional character is a
    /// digit). The `N2WError::Value` arm is dead in practice; it mirrors the
    /// `ValueError` Python's `int()` would raise, which is reachable only for a
    /// value whose `repr` is scientific notation — see the port report's
    /// concerns. The base default would not reproduce that raise either, so
    /// this is not a regression.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // len(right) in the Python source: the count of fractional digits
        // str(number) shows. Derived from repr on the FFI side.
        let precision = value.precision();
        // Python tests `str(number).startswith("-")`; abs() of the value is
        // equivalent for every ordinary decimal repr.
        let is_negative = value.is_negative();
        let (pre, post) = float2tuple(value);
        // int(left): |value|'s integer part. `float2tuple`'s `pre` is
        // int(value) (signed, truncated toward zero); MI strips the sign off
        // the string first, so the word comes from the magnitude and negword
        // is prepended below.
        let left = pre.abs();

        // No "." in str(number) — an integer-valued Decimal like Decimal("5")
        // (precision 0) — takes Python's `return self._int_to_word(int(n))`
        // arm: the bare integer word, no pointword. A plain float always has a
        // "." in its repr, so precision >= 1 there.
        if precision == 0 {
            let mut ret = checked_int_to_word(&left)?;
            if is_negative {
                ret = format!("{}{}", NEGWORD, ret);
            }
            return Ok(ret.trim().to_string());
        }

        // right = str(number)'s fractional part = post, left-padded to
        // precision (`"0" * (precision - len(post)) + post`).
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        // ret = _int_to_word(int(left)) + " " + self.pointword
        let mut ret = format!("{} {}", checked_int_to_word(&left)?, POINTWORD);
        // for digit in right: ret += " " + self._int_to_word(int(digit))
        for ch in post_str.chars().take(precision as usize) {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("non-digit {:?} in fractional part", ch))
            })?;
            ret.push(' ');
            ret.push_str(&checked_int_to_word(&BigInt::from(d))?);
        }

        // Python: `ret = self.negword + ...` was set up front; negword keeps
        // its trailing space and the whole thing is `.strip()`ped at the end.
        if is_negative {
            ret = format!("{}{}", NEGWORD, ret);
        }
        Ok(ret.trim().to_string())
    }

    // ---- currency ----------------------------------------------------

    /// Used only for the `NotImplementedError` message, which `to_cheque`
    /// raises via `self.__class__.__name__`.
    fn lang_name(&self) -> &str {
        "Num2Word_MI"
    }

    /// `CURRENCY_FORMS[code]`, the strict lookup.
    ///
    /// This is what `currency::default_to_cheque` consults, and it mirrors the
    /// base's `self.CURRENCY_FORMS[currency]` -> `KeyError` -> re-raised as
    /// `NotImplementedError`. MI's own `to_currency` deliberately does **not**
    /// go through here — it uses `.get(..., fallback)` and so never raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // `currency_precision` is intentionally NOT overridden: MI inherits
    // `Num2Word_Base.CURRENCY_PRECISION`, which is `{}` (EN *rebinds* rather
    // than mutates it, so nothing leaks in), so every code takes the
    // `.get(code, 100)` default of 100 — the trait default. This is why
    // BHD/KWD (nominally 3-decimal) and JPY (nominally 0-decimal) behave
    // exactly like EUR here; see quirk 8.
    //
    // `currency_adjective` is NOT overridden: `CURRENCY_ADJECTIVES` is `{}`.
    //
    // `pluralize` / `cents_verbose` / `cents_terse` are NOT overridden and are
    // unreachable: MI's `to_currency` does its own form selection, and
    // `default_to_cheque` takes `forms.unit.last()` and never pluralizes. The
    // trait default for `pluralize` raises NotImplemented, matching Python's
    // abstract `raise NotImplementedError` — correct, since nothing calls it.
    //
    // `money_verbose` is NOT overridden: base's `_money_verbose` is
    // `self.to_cardinal(number)`, which is exactly the trait default, and it
    // dispatches to MI's overridden `to_cardinal`. `to_cheque` relies on this:
    // `cheque:EUR 1234.56` -> "TAHI MANO RUA RAU TORU TEKAU WHĀ AND 56/100
    // EUROS", where the words come from MI and the "AND 56/100" from the base.
    //
    // `to_cheque` is NOT overridden — MI does not define it, so
    // `currency::default_to_cheque` serves it unchanged.

    /// Port of `Num2Word_MI.to_currency` — a full override that shares no code
    /// with `Num2Word_Base.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="NZD", cents=True, separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// # Quirks reproduced (each pinned by a corpus row)
    ///
    /// 6. **An unknown currency never raises.** `.get(currency, <NZD forms>)`
    ///    silently substitutes NZD, so `to_currency(1, "GBP")` is `"tahi tāra"`
    ///    — a pound rendered as a New Zealand *dollar*, with no error. The
    ///    corpus pins this for GBP/JPY/KWD/BHD/INR/CNY/CHF, all seven of which
    ///    come out identical to NZD.
    /// 7. **Cents are parsed lexically and truncated, never rounded.**
    ///    `parts[1][:2]` takes the first two fraction *characters*, so `0.5` ->
    ///    `"5"` -> `ljust` -> `"50"` -> 50 cents (correct), but `12.345` ->
    ///    `"34"` -> 34 cents, **truncating** the 5 that `ROUND_HALF_UP` would
    ///    carry to 35. The base's `Decimal.quantize` path is never reached.
    /// 8. **`CURRENCY_PRECISION` is ignored outright.** The divisor is hard-wired
    ///    to "first two fraction digits". JPY (a 0-decimal currency) still gets
    ///    cents — `to_currency(12.34, "JPY")` is `"tekau rua tāra toru tekau whā
    ///    hēneti"` — and KWD/BHD get 2 hēneti, not 3 mils. Corpus-confirmed.
    /// 9. **`adjective` is accepted and ignored.** The parameter is never read;
    ///    `CURRENCY_ADJECTIVES` is empty anyway, so even the base's behaviour
    ///    would be a no-op.
    /// 10. **Int vs float converge here, but only by accident.** Python branches
    ///    on `str(val)`, not `isinstance(val, int)`: `str(1)` is `"1"` (one
    ///    part -> `right = 0`) while `str(1.0)` is `"1.0"` (`parts[1] == "0"`,
    ///    truthy -> `right = int("00") == 0`). Both land on `right == 0`, which
    ///    is falsy, so both skip cents and print `"tahi euro"`. The two paths
    ///    are kept distinct below rather than collapsed, because the *reason*
    ///    they agree is arithmetic on `right`, not a shared branch. Note this
    ///    diverges from `Num2Word_Base.to_currency`, where a float `1.0` *does*
    ///    render a zero-cents segment.
    /// 11. **The `>= 10^9` digit fallback reaches the currency path.** `left` is
    ///    fed to `_int_to_word` unbounded, so `to_currency(10**9, "EUR")` is
    ///    `"1000000000 euros"` — bare digits and a plural. Verified live.
    ///
    /// # Errors
    ///
    /// `N2WError::Value` where Python's `int()` raises `ValueError` on a
    /// non-numeric field — reachable only for a float whose `repr` is in
    /// scientific notation (see the port report's concerns).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let _ = adjective; // Python names the parameter and never reads it. Quirk 9.

        // MI's Python signature defaults `separator=" "`, but the FFI bridge
        // cannot express "caller omitted it" and always sends `Num2Word_Base`'s
        // default `","` (see `num2words2/__init__.py` and `bench/diff_test.py`,
        // both of which hard-code `","`). The corpus was generated through the
        // Python path with the separator omitted, so every expected string uses
        // `" "`. Mapping `","` back to `" "` reproduces the default call (63 of
        // the 108 currency rows depend on it) and passes every other separator
        // through untouched. The one call this cannot serve is an *explicit*
        // `separator=","`, which is indistinguishable from the default at this
        // boundary; it renders `" "`. Flagged in the port report. Matches the
        // convention already established by the sibling `lang_haw.rs`.
        let separator = if separator == "," { " " } else { separator };

        // Python: `if val < 0: is_negative = True; val = abs(val)` — note the
        // abs() happens *before* str(), so the sign never reaches the split.
        let is_negative = val.is_negative();
        let s = match val {
            // `str(int)` — no ".", so `parts` has length 1 and right stays 0.
            CurrencyValue::Int(i) => {
                if is_negative { i.abs() } else { i.clone() }.to_string()
            }
            // `str(float)`. The Python shim already stringified the float with
            // repr() and the core parsed that, so `BigDecimal::to_string` here
            // reproduces `str(val)` exactly for every ordinary decimal repr —
            // crucially preserving the trailing ".0" of `1.0`, which is what
            // makes quirk 10 work out.
            CurrencyValue::Decimal { value: d, .. } => {
                if is_negative { d.abs() } else { d.clone() }.to_string()
            }
        };

        // Python: `parts = str(val).split(".")`. Splitting on every "." (not
        // just the first) so that `parts[1]` is the field *between* the first
        // and second dot, exactly as Python's list indexing would see it.
        let mut parts = s.split('.');
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(p0).map_err(|_| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", p0))
            })?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // Sliced by chars(), per the porting contract. Quirk 7.
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut f: String = frac.chars().take(2).collect();
                while f.chars().count() < 2 {
                    f.push('0'); // ljust(2, "0")
                }
                BigInt::from_str(&f).map_err(|_| {
                    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", f))
                })?
            }
            _ => BigInt::zero(),
        };

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — NZD on a miss,
        // never an error. Quirk 6.
        let forms = self.forms.get(currency).unwrap_or_else(|| {
            self.forms
                .get(FALLBACK_CURRENCY)
                .expect("new() always inserts NZD")
        });
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::from(1);
        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            checked_int_to_word(&left)?,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is falsy at 0, so a float with zero
        // cents (1.0) prints no cents segment. Quirk 10.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                checked_int_to_word(&right)?,
                if right != one { &cr2[1] } else { &cr2[0] }
            ));
        }

        // `if is_negative: result = self.negword + result` — "minus " (with its
        // trailing space) is prepended, then the whole thing is stripped.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}
