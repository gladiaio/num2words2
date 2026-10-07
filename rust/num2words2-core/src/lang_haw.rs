//! Port of `lang_HAW.py` (Hawaiian).
//!
//! # Best-candidate words (gladiaio/num2words2#154)
//!
//! UNVERIFIED (#154): "'i'o 'ole" (minus) — best candidate: "negative" in
//!   Pukui-Elbert (wehe.hilo.hawaii.edu/?q=negative, "kaha ʻiʻo ʻole" = negative
//!   sign); no source reads a negative number aloud.
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "piliona" (10^9) and "kiliona" (10^12) (Pukui-Elbert via
//! wehe.hilo.hawaii.edu), composed like the million arm, and raises
//! `OverflowError` from 10^15, which `maxval()` reports. Where the notes below
//! describe the digit fallback ("1000000000", "no overflow check"), they
//! describe Python; that arm is now unreachable.
//!
//! Shape: **self-contained**. `Num2Word_HAW` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`, so Python never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives `_int_to_word`,
//! a plain recursive cascade over 10 / 100 / 1000 / 10^6 / 10^9 thresholds.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here,
//! and there is **no overflow check** — see the 10^9 fallback below.
//!
//! `setup()` assigns `negword = "minus "` (trailing space is load-bearing —
//! `to_cardinal` concatenates then `.strip()`s) and the English
//! `pointword = "point"`; the port says "kiko" (Pukui-Elbert, "kiko
//! kekimala" = decimal point), gladiaio/num2words2#154. The negword is the
//! best candidate "'i'o 'ole " (see UNVERIFIED above).
//!
//! Every method in scope is overridden by HAW, so nothing is inherited from
//! `Num2Word_Base` here except the class scaffolding:
//!   * `to_cardinal`    — overridden (see below)
//!   * `to_ordinal`     — overridden ("ka mua" / "ka lua" / "ka " + cardinal)
//!   * `to_ordinal_num` — overridden (`str(number) + "."`)
//!   * `to_year`        — overridden (`self.to_cardinal(val)`; the `longval`
//!     kwarg is accepted and then ignored entirely)
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following all look wrong but are exactly
//! what Python emits, each confirmed against a frozen corpus row:
//!
//! 1. **The 10^9 fallback returns digits, not words.** `_int_to_word`'s final
//!    `else` is `return str(number)  # Fallback for very large numbers`. So
//!    `to_cardinal(10**9)` == `"1000000000"` and `to_cardinal(10**21)` ==
//!    `"1000000000000000000000"` — bare digit strings, no Hawaiian at all, and
//!    **no** `OverflowError`. `kaukani`/`miliona` are the largest scale words
//!    defined; there is no billion word, so the module gives up silently.
//!    Corpus: `{"arg": "1000000000", "out": "1000000000"}`. See [`one_billion`].
//! 2. **Negatives leak the negword into ordinals.** `to_ordinal` prefixes a
//!    literal `"ka "` onto whatever `to_cardinal` returns, with no sign
//!    handling, so `to_ordinal(-1)` == `"ka 'i'o 'ole 'ekahi"`. Corpus confirms.
//!    Combined with (1), `to_ordinal(10**9)` == `"ka 1000000000"`.
//! 3. **Zero (fixed, gladiaio/num2words2#154).** The zero guard reads
//!    `return self.ones[0] if self.ones[0] else "zero"` with `ones[0] == ""`,
//!    so Python always answered the English "zero". The port says the
//!    Hawaiian "'ole" (ʻokina written as an apostrophe, as everywhere in this
//!    module): `to_cardinal(0)` == `"'ole"`, `to_ordinal(0)` == `"ka 'ole"`.
//! 4. **`_int_to_word`'s negative branch is unreachable.** `to_cardinal`
//!    strips the `"-"` from the *string* before calling `_int_to_word`, and no
//!    recursive call can go negative (`div`/`mod` of a non-negative). Kept
//!    below to mirror the source; see [`int_to_word`].
//! 5. **Hawaiian teens are compounds, not words.** 11 == `"'umi 'ekahi"`
//!    (literally "ten one") — there is no teens table. Not a bug, but it means
//!    the 10..99 branch has no special case and 11..19 fall straight out of
//!    the generic `TENS[t] + " " + ONES[o]` rule.
//!
//! # Orthography
//!
//! The source uses the **ASCII apostrophe** `'` (U+0027) where Hawaiian
//! orthography would want the ʻokina (U+02BB), and `ā` is U+0101 (LATIN SMALL
//! LETTER A WITH MACRON). Both verified by hexdump of `lang_HAW.py` and of the
//! corpus `out` fields. Reproduced byte for byte — do not "correct" the
//! apostrophes.
//!
//! # Errors
//!
//! None, for the four integer modes. For integer input every path in scope
//! returns `Ok`: there is no overflow check, no dict lookup that can miss, and
//! no list index that can go out of range (all indices are derived from `% 10`
//! / `// 100` of a value the branch has already bounded). The `Result` is
//! structural only. The currency surface below *can* fail — see
//! [`LangHaw::to_currency`] and the `to_cheque` note.
//!
//! # Currency (phase 2)
//!
//! `Num2Word_HAW` declares its **own** two-entry `CURRENCY_FORMS` and inherits
//! nothing from `lang_EUR`/`lang_EU`, so the "English mutates the shared class
//! dict" trap in `PORTING_CURRENCY.md` does not apply here. Verified against
//! the live interpreter:
//!
//! ```text
//! CURRENCY_FORMS      == {'USD': (('kālā','kālā'), ('keneka','keneka')),
//!                         'EUR': (('euro','euros'), ('cent','cents'))}   # USD first
//! CURRENCY_PRECISION  == {}     # -> divisor is always the default 100
//! CURRENCY_ADJECTIVES == {}     # -> `adjective=True` can never do anything
//! MRO                 == [Num2Word_HAW, Num2Word_Base, object]
//! ```
//!
//! `to_currency` is overridden outright and shares **no** code with
//! `Num2Word_Base.to_currency`, so `currency::default_to_currency` is bypassed
//! entirely. `to_cheque` is *not* overridden, so it comes from the base and
//! `currency::default_to_cheque` serves it unchanged.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). The euro has no reliable Hawaiian noun, so EUR raises
//! NotImplementedError. Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `setup`: `self.negword = "minus "`. The trailing space matters — see
/// [`LangHaw::to_cardinal`], which concatenates it then trims.
const NEGWORD: &str = "'i'o 'ole ";

/// `self.ones`. Index 0 is `""` and is never read as a word — see quirk 3.
const ONES: [&str; 10] = [
    "",
    "'ekahi",
    "'elua",
    "'ekolu",
    "'ehā",
    "'elima",
    "'eono",
    "'ehiku",
    "'ewalu",
    "'eiwa",
];

/// `self.tens`. Index 0 is `""` and is unreachable (the 10..99 branch only
/// runs for `number >= 10`, so `number // 10 >= 1`).
const TENS: [&str; 10] = [
    "",
    "'umi",
    "iwakālua",
    "kanakolu",
    "kanahā",
    "kanalima",
    "kanaono",
    "kanahiku",
    "kanawalu",
    "kanaiwa",
];

const HUNDRED: &str = "haneli";
const THOUSAND: &str = "kaukani";
const MILLION: &str = "miliona";

fn ten() -> BigInt {
    BigInt::from(10)
}
fn one_hundred() -> BigInt {
    BigInt::from(100)
}
fn one_thousand() -> BigInt {
    BigInt::from(1_000)
}
fn one_million() -> BigInt {
    BigInt::from(1_000_000)
}
/// The ceiling of the Hawaiian scale words. At or above this, `_int_to_word`
/// falls through to `str(number)` — quirk 1.
fn one_billion() -> BigInt {
    BigInt::from(1_000_000_000)
}

/// Port of `Num2Word_HAW._int_to_word`.
///
/// Mirrors the Python cascade branch for branch. Each numeric branch has
/// already bounded `number` below 10^9 before any `to_u64`, so the narrowing
/// conversions are provably lossless; the unbounded case returns before any
/// conversion happens. Recursion stays on `BigInt`, as Python's does on `int`.
fn int_to_word(number: &BigInt) -> String {
    // Python: `if number == 0: return self.ones[0] if self.ones[0] else "zero"`
    // `ones[0]` is "" (falsy), so this is unconditionally "zero". Quirk 3.
    if number.is_zero() {
        return "'ole".to_string();
    }

    // Python: `if number < 0: return self.negword + self._int_to_word(abs(number))`
    // Unreachable — to_cardinal strips the sign from the string first, and no
    // recursion below can produce a negative. Kept to mirror the source.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    if number < &ten() {
        // 1..=9, so the index is in range and never hits the "" at 0.
        let i = number.to_usize().expect("bounded < 10");
        return ONES[i].to_string();
    }

    if number < &one_hundred() {
        let n = number.to_usize().expect("bounded < 100");
        let tens_val = n / 10; // 1..=9
        let ones_val = n % 10; // 0..=9
        if ones_val == 0 {
            return TENS[tens_val].to_string();
        }
        return format!("{} {}", TENS[tens_val], ONES[ones_val]);
    }

    if number < &one_thousand() {
        let n = number.to_usize().expect("bounded < 1000");
        let hundreds_val = n / 100; // 1..=9
        let remainder = n % 100;
        // Note: Python indexes `self.ones[hundreds_val]` directly here rather
        // than recursing, which is why 100 is "'ekahi haneli" (one hundred)
        // and not bare "haneli".
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    if number < &one_million() {
        let n = number.to_u64().expect("bounded < 10^6");
        let thousands_val = n / 1_000; // 1..=999
        let remainder = n % 1_000;
        let mut result = format!(
            "{} {}",
            int_to_word(&BigInt::from(thousands_val)),
            THOUSAND
        );
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    if number < &one_billion() {
        let n = number.to_u64().expect("bounded < 10^9");
        let millions_val = n / 1_000_000; // 1..=999
        let remainder = n % 1_000_000;
        let mut result =
            format!("{} {}", int_to_word(&BigInt::from(millions_val)), MILLION);
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

    // Python: `else: return str(number)  # Fallback for very large numbers`.
    // No OverflowError, no words — just the decimal digits. Quirk 1.
    number.to_string()
}

/// The `ValueError` HAW's string-driven `to_cardinal` raises when `str(float)`
/// is not a plain decimal, or `None` when the repr is safe.
///
/// Python's `repr(float)` switches to exponent form at `abs(v) >= 1e16` and,
/// for nonzero magnitudes, below `1e-4`; non-finite values print "inf"/"nan".
/// HAW then feeds the text to `int()`:
///
/// * no "." in the repr (`"1e+16"`, `"inf"`, `"nan"`) — `int(n)` rejects the
///   whole literal;
/// * "." in the repr (`"1.5e+16"` splits into `"1"` / `"5e+16"`) — `int(left)`
///   succeeds and the per-character digit loop dies on `int("e")`.
///
/// Either way the observable exception is a ValueError; the literal in the
/// message is reconstructed to match which of the two `int()` calls raised.
/// The sign never appears in the literal: `to_cardinal` strips a leading "-"
/// from the string before either `int()` runs.
fn sci_float_value_error(f: f64) -> Option<N2WError> {
    if !f.is_finite() {
        let lit = if f.is_nan() { "nan" } else { "inf" };
        return Some(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            lit
        )));
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Rebuild Python's exponent-form repr from Rust's `{:e}` (both are
        // shortest-round-trip): "1e16" -> "1e+16", "1e-5" -> "1e-05".
        let s = format!("{:e}", a);
        let (m, e) = s.split_once('e').expect("{:e} always contains an 'e'");
        let exp: i32 = e.parse().expect("{:e} exponent is a plain integer");
        let lit = if m.contains('.') {
            // "1.5e+16": the fraction loop reaches the 'e' character first.
            "e".to_string()
        } else {
            format!("{}e{}{:02}", m, if exp < 0 { '-' } else { '+' }, exp.abs())
        };
        return Some(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            lit
        )));
    }
    None
}

/// The `ValueError` for a `Decimal` whose `str()` is scientific notation
/// (`"1E+2"`, `"1E-7"`, `"1.5E+3"`). Mirrors [`sci_float_value_error`]'s two
/// shapes with Decimal's capital `E`: no "." — `int("1E+2")` rejects the whole
/// (sign-stripped) literal; with a "." — the digit loop dies on `int("E")`.
fn decimal_sci_value_error(s: &str) -> N2WError {
    let n = s.strip_prefix('-').unwrap_or(s);
    let lit = if n.contains('.') { "E" } else { n };
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        lit
    ))
}

/// The code whose forms `to_currency` falls back to for an unknown currency.
///
/// Python: `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
/// `list(values())[0]` is the **first-inserted** entry, and the class body
/// lists `USD` before `EUR`, so the fallback is USD's forms. Dicts have
/// preserved insertion order since CPython 3.7, and the live interpreter
/// confirms `list(CURRENCY_FORMS.keys()) == ['USD', 'EUR']`. A `HashMap` has no
/// order of its own, so the choice is pinned to a constant here rather than
/// left to iteration order.
const FALLBACK_CURRENCY: &str = "USD";

/// The separator the FFI bridge sends when the caller passed none.
///
/// `Num2Word_HAW.to_currency` defaults `separator=" "`, but the bridge cannot
/// express "caller omitted it": `num2words2/__init__.py` and
/// `bench/diff_test.py` both send `Num2Word_Base`'s default `","` literally.
/// The corpus was generated through the Python path with the separator omitted,
/// so every expected string uses `" "` — 54 of the 108 currency rows depend on
/// mapping this back. Right for the default call and for every caller who
/// passes anything other than `","`; wrong only for an explicit
/// `separator=","`, which Python renders `"'umi 'elua euros,kanakolu 'ehā
/// cents"` and this renders with a space. That case is indistinguishable from
/// the default at this boundary. Fixing it properly means teaching the binding
/// each language's own default; it cannot be fixed from this file. ~100 of the
/// 156 Python modules override this default, so the issue is systemic rather
/// than HAW-specific. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// HAW's own `to_currency` default, restored when [`SEPARATOR_UNSET`] arrives.
const SEPARATOR_DEFAULT: &str = " ";

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "piliona"), (12, "kiliona")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

/// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
/// entry point that hands over a caller-supplied integer goes through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

pub struct LangHaw {
    /// `CURRENCY_FORMS`, built once. Both entries carry exactly two unit forms
    /// and two subunit forms, matching Python's tuple arity — `to_currency`
    /// indexes `[0]`/`[1]` directly, so the arity is load-bearing.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl LangHaw {
    pub fn new() -> Self {
        // Built once and cached by the caller (`num2words2-py` holds this in a
        // OnceLock), never per call.
        let mut forms = HashMap::with_capacity(2);
        forms.insert(
            "USD",
            CurrencyForms::new(&["kālā", "kālā"], &["keneka", "keneka"]),
        );
        // EUR was English ("dollars", "cents", "euros"); no reliable Hawaiian euro noun was found, so
        // it raises NotImplementedError (#222).
        LangHaw { forms }
    }
}

impl Default for LangHaw {
    fn default() -> Self {
        LangHaw::new()
    }
}

impl Lang for LangHaw {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "USD"
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
        "kiko"
    }

    /// Port of `Num2Word_HAW.to_cardinal`.
    ///
    /// Python works on `str(number).strip()` and tests `n.startswith("-")`,
    /// stripping the sign lexically rather than arithmetically. For integer
    /// input that is exactly `is_negative()` + `abs()`, since `str(BigInt)`
    /// emits a leading `-` iff the value is negative and never emits `+`,
    /// whitespace, or a `.`. The float branch (`if "." in n`) is unreachable
    /// for integers and out of scope.
    ///
    /// The trailing `.strip()` is reproduced as `trim()`. It is a no-op for
    /// every reachable input: `int_to_word` never returns an empty or
    /// space-padded string, so `negword`'s trailing space always lands
    /// *between* "minus" and the first word rather than at an edge. Kept for
    /// fidelity — and because it is what would swallow the sign spacing if
    /// `int_to_word` ever did return "".
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (n, ret) = if value.is_negative() {
            (value.abs(), NEGWORD)
        } else {
            (value.clone(), "")
        };
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_HAW.to_ordinal`.
    ///
    /// Only 1 and 2 have suppletive forms; everything else is a bare `"ka "`
    /// prefix on the cardinal, including negatives and the 10^9 digit
    /// fallback. Quirk 2.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value == &BigInt::from(1) {
            return Ok("ka mua".to_string()); // first
        }
        if value == &BigInt::from(2) {
            return Ok("ka lua".to_string()); // second
        }
        Ok(format!("ka {}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_HAW.to_ordinal_num`: `str(number) + "."`.
    ///
    /// No sign handling and no suppletion — `to_ordinal_num(-1)` == `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_HAW.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The `longval=True` kwarg is accepted and ignored, so years get no
    /// century splitting: 1776 == "'ekahi kaukani 'ehiku haneli kanahiku
    /// 'eono" (one thousand seven hundred seventy six), and negative years get
    /// no BC marker — `to_year(-44)` == "'i'o 'ole kanahā 'ehā".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the float/Decimal branch of `Num2Word_HAW.to_cardinal`.
    ///
    /// HAW overrides `to_cardinal` and handles non-integers **inline** on the
    /// string `str(number)` — it never calls `base.float2tuple` and never
    /// reaches `Num2Word_Base.to_cardinal_float`. So the two traps in
    /// `PORTING_FLOAT.md` do **not** apply here: there is no `round_ties_even`
    /// and no `< 0.01` f64-artefact heuristic. `str(2.675)` is literally
    /// `"2.675"`, and each fractional **character** becomes its own digit word.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword          # "minus "
    /// else:
    ///     ret = ""
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
    /// The `precision=` kwarg is inert for HAW: `to_cardinal` takes no such
    /// parameter and reads the fractional length straight off `repr`, so the
    /// live interpreter returns the same string with or without it. Hence
    /// `precision_override` is deliberately ignored and `value.precision()`
    /// (the repr-derived length, `abs(Decimal(str(v)).as_tuple().exponent)`)
    /// drives the split.
    ///
    /// # Reconstructing `str(number)`
    ///
    /// * **Float** — `format!("{:.p}", value.abs())` with `p = precision`
    ///   yields exactly the repr's fractional digits: `precision` was derived
    ///   from that same shortest round-trip repr, so rounding the raw f64 to
    ///   `p` places reproduces it (`2.675` -> `"2.675"`, `1.005` -> `"1.005"`).
    /// * **Decimal** — the mantissa at scale `precision` gives the exact digits
    ///   with trailing zeros preserved (`Decimal("1.10")` -> `"1"`, `"10"`),
    ///   matching `str(Decimal)`.
    ///
    /// # Quirks reproduced
    ///
    /// * The 10^9 digit fallback (quirk 1) fires on the **integer part** too:
    ///   `Decimal("98746251323029.99")` -> `"98746251323029 point 'eiwa
    ///   'eiwa"` — the integer part is bare digits. Corpus-pinned.
    /// * `zero` (quirk 3) is emitted for every `0` digit, so `0.01` ->
    ///   `"'ole kiko 'ole 'ekahi"` and `1.005` -> `"'ekahi point zero zero
    ///   'elima"`.
    /// * A negative fraction keeps the negword and prints `int_to_word(0)`:
    ///   `-0.5` -> `"'i'o 'ole 'ole kiko 'elima"` (there is no `pre == 0` sign
    ///   rescue like the base path — the `"-"` is stripped lexically).
    ///
    /// # Errors
    ///
    /// `N2WError::Value` mirroring Python's `int()` `ValueError` on a
    /// non-digit fractional character — unreachable for ordinary decimal
    /// reprs; possible only for a scientific-notation `repr` (e.g. a Float so
    /// large that `str` is `"1e+16"`), which Python itself would raise
    /// `ValueError` on. That specific edge is out of corpus scope; see the
    /// port report's concerns.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Ignore the override — see doc comment; HAW's split length is the
        // repr-derived precision, full stop.
        let precision = value.precision();
        let is_negative = value.is_negative();

        // (int_left, frac_digits) reconstructed from `str(number)`.
        let (int_left, frac): (BigInt, String) = match value {
            FloatValue::Float { value: f, .. } => {
                // `{:.p}` always emits exactly `p` fractional digits and never
                // scientific notation, so split_once('.') is safe for p > 0.
                let s = format!("{:.*}", precision as usize, f.abs());
                match s.split_once('.') {
                    Some((i, fr)) => (
                        BigInt::from_str(i).unwrap_or_else(|_| BigInt::zero()),
                        fr.to_string(),
                    ),
                    None => (
                        BigInt::from_str(&s).unwrap_or_else(|_| BigInt::zero()),
                        String::new(),
                    ),
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                // str(Decimal) shows the exact digits; the mantissa at scale
                // `precision` is that digit string with trailing zeros kept.
                let (mant, _scale) =
                    d.abs().with_scale(precision as i64).as_bigint_and_exponent();
                let digits = mant.to_string(); // non-negative, no leading zeros
                let p = precision as usize;
                if digits.len() > p {
                    let (i, fr) = digits.split_at(digits.len() - p);
                    (
                        BigInt::from_str(i).unwrap_or_else(|_| BigInt::zero()),
                        fr.to_string(),
                    )
                } else {
                    // Integer part is 0; left-pad the fraction to `p` digits,
                    // e.g. Decimal("0.01") -> mant "1" -> frac "01".
                    let mut fr = "0".repeat(p - digits.len());
                    fr.push_str(&digits);
                    (BigInt::zero(), fr)
                }
            }
        };

        // Build `ret` exactly as Python concatenates, then `.strip()`.
        let mut ret = String::new();
        if is_negative {
            ret.push_str(NEGWORD); // "'i'o 'ole " — trailing space is load-bearing.
        }
        ret.push_str(&checked_int_to_word(&int_left)?);

        // Python emits the point + digit words only when `"." in n`, i.e. when
        // there is a fractional part (precision > 0). precision == 0 reproduces
        // the integer `else` branch (bare `int_to_word`, no "point").
        if precision > 0 {
            ret.push(' ');
            ret.push_str(self.pointword()); // "kiko"
            for ch in frac.chars() {
                ret.push(' ');
                let digit = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                ret.push_str(&checked_int_to_word(&BigInt::from(digit))?);
            }
        }

        Ok(ret.trim().to_string())
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// HAW routes on the string, not the value: `"." in str(number)` decides
    /// between the digit-word grammar and `int(n)`. So the base default's
    /// whole→int shortcut is wrong here — `str(5.0)` is `"5.0"` and must read
    /// `"'elima kiko 'ole"`, while the point-free `Decimal("5")` stays
    /// `"'elima"`, and an exponent-form repr raises the `int()` ValueError
    /// ([`sci_float_value_error`] / [`decimal_sci_value_error`] reconstruct
    /// which of the two `int()` calls fires).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                if let Some(err) = sci_float_value_error(*f) {
                    return Err(err);
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                let s = python_decimal_str(d);
                if s.contains('E') {
                    return Err(decimal_sci_value_error(&s));
                }
            }
        }
        if value.has_visible_point() {
            // `"." in n`: the digit-word grammar, whole values included.
            return self.to_cardinal_float(value, precision_override);
        }
        // Point-free and not scientific: an integer-valued Decimal — Python's
        // `return (ret + self._int_to_word(int(n))).strip()` arm. (A finite
        // float without a point is >= 1e16 and was rejected above.)
        let i = value
            .as_whole_int()
            .expect("a point-free, non-scientific value is whole");
        self.to_cardinal(&i)
    }

    /// `to_ordinal(float/Decimal)`. Python's `number == 1` / `number == 2`
    /// are *numeric* comparisons, so `1.0` is "ka mua" and `Decimal("2.00")`
    /// is "ka lua"; everything else — negative zero included — is `"ka "`
    /// glued to the string-routed cardinal: `to_ordinal(5.0)` ==
    /// `"ka 'elima kiko 'ole"`, `to_ordinal(-0.0)` == `"ka minus zero point
    /// zero"`. An exponent-form repr propagates the cardinal's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i == BigInt::from(1) {
                return Ok("ka mua".to_string());
            }
            if i == BigInt::from(2) {
                return Ok("ka lua".to_string());
            }
        }
        Ok(format!("ka {}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the raw repr,
    /// never an error: `to_ordinal_num(1e16)` == `"1e+16."` and
    /// `Decimal("5.00")` == `"5.00."`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: bare `self.to_cardinal(val)`, string routing
    /// included — `to_year(5.0)` == `"'elima kiko 'ole"`, `to_year(1e16)`
    /// raises ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`) and this
    /// override does not change what parses — `python_decimal_parse` still
    /// decides. It exists because `Decimal("Infinity")`/`Decimal("NaN")` *do*
    /// parse in Python, and HAW's `to_cardinal` then dies at `int("Infinity")`
    /// with `ValueError: invalid literal for int() with base 10: 'Infinity'`
    /// (str(Decimal) capitalises whatever the input case, and the sign of
    /// "-Infinity" is stripped before `int()` sees it). The bridge hard-wires
    /// the *base* integer-path errors for Inf/NaN parses, which HAW never
    /// produces, so the raise is surfaced here instead. The one input this
    /// would misserve is `to_ordinal_num("Infinity")` (Python: `"Infinity."`),
    /// which no corpus exercises.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    /// Used only for the `NotImplementedError` message, which `to_cheque`
    /// raises via `self.__class__.__name__`.
    fn lang_name(&self) -> &str {
        "Num2Word_HAW"
    }

    /// `CURRENCY_FORMS[code]`, the strict lookup.
    ///
    /// This is what `currency::default_to_cheque` consults, and it mirrors the
    /// base's `self.CURRENCY_FORMS[currency]` -> `KeyError` -> re-raised as
    /// `NotImplementedError`. Note that HAW's own `to_currency` deliberately
    /// does **not** go through here — it uses `.get(..., fallback)` and so never
    /// raises. Hence `to_cheque("GBP")` raises while `to_currency("GBP")`
    /// happily prints kālā; that asymmetry is real and the corpus pins it.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // `currency_precision` is intentionally NOT overridden: HAW inherits
    // `Num2Word_Base.CURRENCY_PRECISION`, which is `{}`, so every code takes
    // the `.get(code, 100)` default of 100 — the trait default. This is why
    // BHD/KWD (nominally 3-decimal) and JPY (nominally 0-decimal) behave
    // exactly like EUR here; see quirk 8.
    //
    // `currency_adjective` is NOT overridden: `CURRENCY_ADJECTIVES` is `{}`.
    //
    // `pluralize` / `cents_verbose` / `cents_terse` are NOT overridden and are
    // unreachable: HAW's `to_currency` does its own form selection, and
    // `default_to_cheque` takes `forms.unit.last()` and never pluralizes. The
    // trait default for `pluralize` raises NotImplemented, matching Python's
    // abstract `raise NotImplementedError` — correct, since nothing calls it.
    //
    // `money_verbose` is NOT overridden: base's `_money_verbose` is
    // `self.to_cardinal(number)`, which is exactly the trait default, and it
    // dispatches to HAW's overridden `to_cardinal`. `to_cheque` relies on this.

    /// Port of `Num2Word_HAW.to_currency` — a full override that shares no code
    /// with `Num2Word_Base.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="USD", cents=True, separator=" ", adjective=False):
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
    /// 6. **An unknown currency never raises.** `.get(currency, <USD forms>)`
    ///    silently substitutes USD, so `to_currency(1, "GBP")` is
    ///    `"'ekahi kālā"` — a pound rendered as a *dollar*, with no error. The
    ///    corpus pins this for GBP/JPY/KWD/BHD/INR/CNY/CHF, all of which come
    ///    out identical to USD.
    /// 7. **Cents are parsed lexically and truncated, never rounded.**
    ///    `parts[1][:2]` takes the first two fraction *characters*, so `0.5` ->
    ///    `"5"` -> `ljust` -> `"50"` -> 50 cents (correct), but `12.345` ->
    ///    `"34"` -> 34 cents, **truncating** the 5 that `ROUND_HALF_UP` would
    ///    carry to 35. The base's `Decimal.quantize` path is never reached.
    /// 8. **`CURRENCY_PRECISION` is ignored outright.** The divisor is hard-wired
    ///    to "first two fraction digits". JPY (a 0-decimal currency) still gets
    ///    cents — `to_currency(12.34, "JPY")` is `"'umi 'elua kālā kanakolu
    ///    'ehā keneka"` — and KWD/BHD get 2 mils, not 3. Corpus-confirmed.
    /// 9. **`adjective` is accepted and ignored.** The parameter is never read;
    ///    `CURRENCY_ADJECTIVES` is empty anyway, so even the base's behaviour
    ///    would be a no-op.
    /// 10. **Int vs float converge here, but only by accident.** Python branches
    ///    on `str(val)`, not `isinstance(val, int)`: `str(1)` is `"1"` (one
    ///    part -> `right = 0`) while `str(1.0)` is `"1.0"` (`parts[1] == "0"`,
    ///    truthy -> `right = int("00") == 0`). Both land on `right == 0`, which
    ///    is falsy, so both skip cents and print `"'ekahi euro"`. The two paths
    ///    are kept distinct below rather than collapsed, because the *reason*
    ///    they agree is arithmetic on `right`, not a shared branch.
    ///
    /// # Errors
    ///
    /// `N2WError::Value` where Python's `int()` raises `ValueError` on a
    /// non-numeric field — reachable only for a float whose `repr` is in
    /// scientific notation (see the module-level concern in the port report).
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

        // Restore HAW's own separator default. See [`SEPARATOR_UNSET`].
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

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

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — USD on a miss,
        // never an error. Quirk 6.
        let forms = self.forms.get(currency).unwrap_or_else(|| {
            self.forms
                .get(FALLBACK_CURRENCY)
                .expect("new() always inserts USD")
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

