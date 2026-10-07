//! Port of `lang_NN.py` (Norwegian Nynorsk).
//!
//! Shape: **self-contained**. `Num2Word_NN` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the guard in
//! `Num2Word_Base.__init__` never fires: `self.cards` is never built and
//! `self.MAXVAL` is never assigned. `to_cardinal` is overridden outright and
//! drives a private `_int_to_word` recursion. Consequently `cards`/`merge`
//! stay at their trait defaults here. Python has **no overflow check** —
//! arbitrarily large input was silently degraded (see bug 5 below); this
//! port raises `OverflowError` from 10^21 (`maxval`).
//!
//! `Num2Word_NN` overrides all four in-scope methods (`to_cardinal`,
//! `to_ordinal`, `to_ordinal_num`, `to_year`), so nothing is inherited from
//! `Num2Word_Base` for the ported surface. In particular `title()` is never
//! reached: base's `to_cardinal` applies it, but NN's override does not, and
//! `is_title` is `False` anyway.
//!
//! `setup()` also sets `pointword = "point"` (this port: Nynorsk "komma",
//! gladiaio/num2words2#154 — Norwegian writes a decimal comma), used only by
//! the float path
//! (`"." in n`). `Num2Word_NN` overrides `to_cardinal` outright and handles
//! non-integer input inline off `str(number)`, so it never reaches
//! `base.float2tuple`/`to_cardinal_float`; [`LangNn::to_cardinal_float`] ports
//! that inline branch directly. It is surfaced via the `pointword()` trait
//! method for the same reason.
//!
//! Because everything keys on `str(number)`, a float/Decimal takes the string
//! route for *every* value, whole ones included: `to_cardinal(5.0)` is
//! "fem komma null", never "fem" — Base's `int(value) == value` whole-value
//! shortcut does not exist here. [`LangNn::cardinal_float_entry`] pins that
//! full routing (and `year_float_entry` inherits it, matching `to_year` ==
//! `to_cardinal`); [`LangNn::ordinal_float_entry`] rides the same path and
//! appends "-de" (`to_ordinal(5.0)` == "fem komma null-de");
//! [`LangNn::ordinal_num_float_entry`] is `str(number) + "."`, so "5.0." and
//! even "1e+16." are real outputs. And because the route is the string form,
//! a value whose `str()` is *exponent notation* crashes `int()` — a Python
//! bug reproduced faithfully (see bug 9 below).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong for Norwegian
//! but is exactly what Python emits, and each is pinned by the frozen corpus:
//!
//! 1. **Zero (fixed, #154).** Python's `_int_to_word` opens with
//!    `return self.ones[0] if self.ones[0] else "zero"`. `ones[0]` is `""` —
//!    always falsy — so in Python every zero renders as the English "zero".
//!    This port says the Nynorsk "null": `0 -> "null"`,
//!    `to_ordinal(0) -> "null"`.
//! 2. **Teens (fixed, #216).** Python's `< 100` branch was a plain
//!    tens/ones split with no 11..19 case, so 11 was "ti ein" and 12 "ti to".
//!    This port uses the Nynorsk teens from [`TEENS`] (elleve, tolv, tretten,
//!    …, nitten), matching `lang_nb.rs`: `12345 -> "tolv tusen tre hundre
//!    førti fem"`.
//! 3. **No "hundre"/"tusen" elision and no compounding.** Norwegian writes
//!    "hundre" for 100 and compounds ("tjueein"); this module always prefixes
//!    the multiplier and space-separates every token, so `100 -> "ein hundre"`
//!    and `1000 -> "ein tusen"`.
//! 4. **"million" never pluralises.** The `< 10^9` branch appends the bare
//!    singular, so `10^7 -> "ti million"`, not "ti millionar".
//! 5. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's final
//!    `else` is `return str(number)  # Fallback for very large numbers`, so
//!    `10^9 -> "1000000000"`. This port continues the `million` composition
//!    with the Nynorsk long-scale words milliard (10^9), billion (10^12),
//!    billiard (10^15) and trillion (10^18) — all in Nynorskordboka
//!    (ordbokene.no), which has no "trilliard" — so `10^9 -> "ein milliard"`
//!    (never pluralised, like bug 4), and raises `OverflowError` from 10^21
//!    (`maxval`). See [`int_to_word`].
//! 6. ~~**The ordinal was cardinal + "-de", unconditionally.**~~ No stem
//!    changes and no agreement: 11 read "elleve-de", 1234567 ended
//!    "...seksti sju-de". Fixed (#259): the last word takes its Nynorsk
//!    ordinal ("ellevte", "sjuande", "trettande", "tjue første", "to
//!    hundrede", "millionte"), and a bare scale word drops its "ein"
//!    ("hundrede", "tusende"). 0 has no attested ordinal and stays "null".
//!    Negatives are *not* rejected (base's `errmsg_negord` guard is
//!    bypassed by the override): `to_ordinal(-1) -> "minus første"`. The
//!    float/Decimal path still appends "-de" to the decimal reading
//!    ("fem komma null-de").
//! 7. **An unknown currency code silently becomes kroner.** `to_currency`
//!    looks the code up with `CURRENCY_FORMS.get(currency, <first value>)`
//!    instead of indexing, so anything outside NOK/USD/EUR renders with NOK's
//!    forms rather than raising: `to_currency(12.34, "JPY")` is
//!    "tolv kroner tretti fire øre". The inherited `to_cheque` indexes the
//!    same dict and *does* raise for those codes, so the two entry points
//!    disagree about which currencies exist. Both halves are corpus-pinned.
//! 8. **Currency precision is hardcoded to two decimals.** `to_currency`
//!    never reads `CURRENCY_PRECISION` (which is `{}` anyway), so the
//!    zero-decimal and three-decimal currencies are wrong in both directions:
//!    JPY grows a subunit and KWD/BHD are quantised to /100, not /1000.
//! 9. **Values whose `str()` uses exponent notation raise ValueError.** A
//!    float at or above 1e16 reprs as `'1e+16'`; a Decimal whose exponent is
//!    positive prints as `'1E+2'`/`'1E+20'` (and one whose adjusted exponent
//!    drops below -6 as `'1E-7'`). None of those contain a "." — the whole
//!    token hits `int()` and dies: `ValueError: invalid literal for int()
//!    with base 10: '1e+16'`. Ones that *do* keep a "." (`'1.5e-05'`) split
//!    and then die in the digit loop on the exponent marker (`int('e')`,
//!    uppercase `int('E')` for Decimals). Corpus-pinned for 1e+16, 1e+20,
//!    1E+2 and 1E+20 across cardinal/ordinal/year. `to_ordinal_num` never
//!    calls `int()`, so it *succeeds* on the same inputs ("1e+16.").
//! 10. **`"Infinity"`/`"NaN"` strings parse but blow up later.** Base's
//!    `str_to_number` (`Decimal(value)`) accepts them; the failure is NN's
//!    `int("Infinity")` inside `to_cardinal` (ValueError; "-Infinity" loses
//!    its sign first, so the message always quotes 'Infinity') — while
//!    `to_ordinal_num("Infinity")` is *answered*, with "Infinity.". The Rust
//!    dispatcher hard-codes Base semantics for non-finite parses
//!    (OverflowError), which NN never executes, so [`LangNn::str_to_number`]
//!    punts them back to the pure-Python original via NotImplemented; the
//!    fallback then raises (or returns) the byte-exact original per mode.
//!
//! # Dead code reproduced
//!
//! `_int_to_word`'s `number < 0` branch (`negword + _int_to_word(abs(n))`) is
//! unreachable: `to_cardinal` strips the sign textually before calling it, and
//! every recursive call passes a positive value. It is mirrored in
//! [`int_to_word`] to keep the structure aligned with the source, but it can
//! never fire. Note it would double the negword if it ever did.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dollar / euro with cent (same form in the
//! plural). Examples in these docs that quote English nouns record Python's
//! output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `setup()`: note the trailing space — NN overrides base's `"(-) "`.
const NEGWORD: &str = "minus ";
/// Python's `"point"`; the Nynorsk decimal comma "komma" here (#154).
const POINTWORD: &str = "komma";

/// `self.ones`. Index 0 is `""`; see bug 1 — it is only ever read at index 0
/// through the dead conditional, and at 1..=9 for real digits.
const ONES: [&str; 10] = [
    "", "ein", "to", "tre", "fire", "fem", "seks", "sju", "åtte", "ni",
];

/// 11..=19, which Python lacked (bug 2, fixed in #216).
const TEENS: [&str; 9] = [
    "elleve", "tolv", "tretten", "fjorten", "femten", "seksten", "sytten", "atten", "nitten",
];

/// `self.tens`. Index 0 is `""` and unreachable (the branch requires n >= 10).
const TENS: [&str; 10] = [
    "", "ti", "tjue", "tretti", "førti", "femti", "seksti", "sytti", "åtti", "nitti",
];

const HUNDRED: &str = "hundre";
const THOUSAND: &str = "tusen";
const MILLION: &str = "million";

/// What `self.ones[0] if self.ones[0] else "zero"` evaluates to: English
/// "zero" in Python, Nynorsk "null" here (#154).
const ZERO_WORD: &str = "null";

/// The ceiling of Python's worded range; at or above it Python returned
/// `str(number)` (bug 5) and [`HIGH_SCALES`] take over here.
const FALLBACK_AT: u64 = 1_000_000_000;

/// The scales above `million` that Python lacks (gladiaio/num2words2#147),
/// largest first, as `(power of ten, word)`. Nynorsk long scale, all in
/// Nynorskordboka (ordbokene.no). Never pluralised, like `million` (bug 4).
const HIGH_SCALES: [(u32, &str); 4] = [
    (18, "trillion"),
    (15, "billiard"),
    (12, "billion"),
    (9, "milliard"),
];

/// The ordinal of one cardinal word (#259): "første", "sjuande",
/// "trettande", "tjuande", "hundrede", "tusende", "millionte". `None` for
/// "null".
fn ordinal_word(word: &str) -> Option<String> {
    Some(match word {
        "ein" => "første".to_string(),
        "to" => "andre".to_string(),
        "tre" => "tredje".to_string(),
        "fire" => "fjerde".to_string(),
        "fem" => "femte".to_string(),
        "seks" => "sjette".to_string(),
        "elleve" => "ellevte".to_string(),
        "tolv" => "tolvte".to_string(),
        HUNDRED => "hundrede".to_string(),
        THOUSAND => "tusende".to_string(),
        // sju, åtte, ni, ti, tretten..nitten ("trettande"), tjue..nitti.
        w if ["sju", "åtte", "ni", "ti"].contains(&w) => format!("{}ande", w.trim_end_matches('e')),
        w if TEENS.contains(&w) => format!("{}ande", w.trim_end_matches("en")),
        w if TENS[2..].contains(&w) => format!("{}ande", w.trim_end_matches('e')),
        w if w == MILLION || HIGH_SCALES.iter().any(|(_, s)| *s == w) => format!("{}te", w),
        _ => return None,
    })
}

/// The exclusive ceiling: Nynorskordboka has no word for 10^21 (#147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(21))
}

/// Python's `_int_to_word`.
///
/// Splits the bounded case out into [`int_to_word_small`]: below `FALLBACK_AT`
/// the value provably fits in a u64. At or above it Python printed the digits
/// (bug 5); here the [`HIGH_SCALES`] word it, and the `maxval` check raises
/// `OverflowError` past them (#147).
fn int_to_word(n: &BigInt) -> Result<String> {
    check_maxval(n, maxval_ceiling())?;
    if n.is_zero() {
        return Ok(ZERO_WORD.to_string()); // bug 1
    }
    if n.is_negative() {
        // Unreachable — see "Dead code reproduced" in the module docs.
        return Ok(format!("{}{}", NEGWORD, int_to_word(&n.abs())?));
    }
    if let Some(v) = n.to_u64() {
        if v < FALLBACK_AT {
            return Ok(int_to_word_small(v));
        }
    }
    // bug 5: Python's "Fallback for very large numbers" — words instead.
    for &(exp, word) in HIGH_SCALES.iter() {
        let scale = pow10_big(exp);
        if n >= &scale {
            let head = int_to_word(&(n / &scale))?;
            let remainder = n % &scale;
            let mut out = format!("{} {}", head, word);
            if !remainder.is_zero() {
                out.push(' ');
                out.push_str(&int_to_word(&remainder)?);
            }
            return Ok(out);
        }
    }
    unreachable!("every value >= 10^9 and below maxval has a HIGH_SCALES entry")
}

/// The worded range: `0 < n < 10^9`.
fn int_to_word_small(n: u64) -> String {
    debug_assert!(n > 0 && n < FALLBACK_AT);
    if n < 10 {
        ONES[n as usize].to_string()
    } else if (11..20).contains(&n) {
        // bug 2 (fixed, #216): Python had no teens and said "ti ein".
        TEENS[(n - 11) as usize].to_string()
    } else if n < 100 {
        let (t, o) = ((n / 10) as usize, (n % 10) as usize);
        if o == 0 {
            TENS[t].to_string()
        } else {
            format!("{} {}", TENS[t], ONES[o])
        }
    } else if n < 1_000 {
        // Python indexes `self.ones[hundreds_val]` directly here rather than
        // recursing; identical for 1..=9, kept literal for fidelity.
        scale(ONES[(n / 100) as usize], HUNDRED, n % 100)
    } else if n < 1_000_000 {
        scale(&int_to_word_small(n / 1_000), THOUSAND, n % 1_000)
    } else {
        scale(&int_to_word_small(n / 1_000_000), MILLION, n % 1_000_000)
    }
}

/// The shared `head + " " + word [+ " " + rest]` tail of the three scale
/// branches. Python guards the remainder with `if remainder:`, so a zero
/// remainder appends nothing and never recurses into the zero case.
fn scale(head: &str, word: &str, remainder: u64) -> String {
    let mut out = format!("{} {}", head, word);
    if remainder != 0 {
        out.push(' ');
        out.push_str(&int_to_word_small(remainder));
    }
    out
}

/// `CURRENCY_FORMS`, in Python's insertion order — the order is load-bearing,
/// see [`FALLBACK_INDEX`].
///
/// `Num2Word_NN` declares this dict itself and inherits from `Num2Word_Base`,
/// **not** `Num2Word_EUR`, so the shared-class-dict mutation that
/// `Num2Word_EN.__init__` performs on `Num2Word_EUR.CURRENCY_FORMS` never
/// reaches it. Confirmed against the live interpreter: the runtime table is
/// exactly these three entries, and `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION`
/// are both `{}` — hence no `currency_adjective`/`currency_precision` override
/// below (precision stays 100 for *every* code, including JPY and KWD).
///
/// Note NOK's subunit is `("øre", "øre")`: identical singular and plural, which
/// is correct Nynorsk and makes the `right != 1` test invisible for that entry.
const FORMS: [(&str, [&str; 2], [&str; 2]); 3] = [
    ("NOK", ["krone", "kroner"], ["øre", "øre"]),
    ("USD", ["dollar", "dollar"], ["cent", "cent"]),
    ("EUR", ["euro", "euro"], ["cent", "cent"]),
];

/// `list(self.CURRENCY_FORMS.values())[0]` — the `.get()` default in
/// `to_currency` (bug 7). Python dicts iterate in insertion order, and NOK is
/// declared first, so this is always the NOK entry.
const FALLBACK_INDEX: usize = 0;

/// NN's own `to_currency` signature default (`separator=" "`).
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s signature default. The FFI boundary cannot
/// express per-language default arguments, so the Python shim and the diff
/// harness both pass this literal to mean "caller did not supply one"; see the
/// `separator` handling in [`LangNn::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// Python's `int(s)` failing on a non-decimal literal.
fn int_value_error(literal: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        literal
    ))
}

/// Reconstruct `str(number)` for a `FloatValue::Float`, sign included.
///
/// CPython reprs a float in fixed notation only while its decimal exponent is
/// in `[-4, 16)`; outside that it switches to exponent form (`repr(1e16) ==
/// '1e+16'`, `repr(1e-05) == '1e-05'`), which NN's `int()` then chokes on
/// (bug 9). Inside the fixed window, `format!("{:.p}")` with the repr-derived
/// `precision` reproduces `str()` byte for byte — correctly rounding the
/// binary value to the repr's own digit count re-derives exactly those digits
/// (shortest-round-trip is unique at its own length), and it restores the
/// trailing ".0" that Rust's plain `{}` drops from whole floats. The sign
/// comes from the sign *bit*, so `-0.0` prints "-0.0" and earns the negword.
///
/// In the exponent windows the shortest digits are recovered from Rust's `{}`
/// (also shortest-round-trip, hence the same digits as Python's repr) and
/// re-assembled Python-style: one leading digit, "." only if more digits
/// follow, and a signed, two-digit-minimum exponent (`1e+16`, `1.5e-05`).
/// Every f64 at or above 1e16 is whole, so the high side is all integer
/// digits. Non-finite values print as Python does ('inf'/'nan' — note
/// CPython drops NaN's sign bit); they are binding-unreachable but kept so
/// the surface matches `str(float)` everywhere. (Same reconstruction as the
/// XH port's, which the corpus pins on the identical code shape.)
fn py_float_str(value: f64, precision: u32) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let abs = value.abs();
    if abs.is_infinite() {
        return format!("{}inf", sign);
    }
    if abs != 0.0 && abs < 1e-4 {
        // '1e-05' side. Rust `{}` is always positional: "0." + zeros + digits.
        let s = format!("{}", abs);
        let frac = &s[2..];
        let zeros = frac.bytes().take_while(|&b| b == b'0').count();
        let sig = &frac[zeros..];
        let coeff = if sig.len() == 1 {
            sig.to_string()
        } else {
            format!("{}.{}", &sig[..1], &sig[1..])
        };
        return format!("{}{}e-{:02}", sign, coeff, zeros + 1);
    }
    if abs >= 1e16 {
        // '1e+16' side: at this magnitude `{}` is all integer digits.
        let s = format!("{}", abs);
        let sig = s.trim_end_matches('0');
        let coeff = if sig.len() == 1 {
            sig.to_string()
        } else {
            format!("{}.{}", &sig[..1], &sig[1..])
        };
        return format!("{}{}e+{:02}", sign, coeff, s.len() - 1);
    }
    format!("{:.*}", precision as usize, value)
}

/// Python's `Num2Word_NN.to_cardinal` applied to `str(number)` — the shared
/// body behind the integer, float and Decimal entries:
///
/// ```text
/// n = str(number).strip()
/// if n.startswith("-"): n = n[1:]; ret = self.negword
/// else:                 ret = ""
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
/// `int(...)` on a non-numeric token raises ValueError, and exponent-notation
/// `str()`s genuinely reach it (bug 9): `'1e+16'`/`'1E+2'` have no "." so the
/// whole token hits the integer parse, while `'1.5e-05'` splits and then dies
/// in the digit loop on the exponent marker — `int('e')` (uppercase `int('E')`
/// for a Decimal). The error escapes before the negword is applied, matching
/// Python's evaluation order. Both messages mirror CPython's byte for byte,
/// including which token they quote.
fn string_to_words(n: &str) -> Result<String> {
    // `n = str(number).strip()` — no `str()` form can carry edge whitespace,
    // so the strip is a no-op, mirrored via the input contract.
    //
    // `if n.startswith("-"): n = n[1:]; ret = self.negword`
    let (n, ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD),
        None => (n, ""),
    };

    match n.split_once('.') {
        Some((left, right)) => {
            // `ret += _int_to_word(int(left)) + " " + self.pointword + " "`
            let left_int = BigInt::from_str(left).map_err(|_| int_value_error(left))?;
            let mut out = format!("{}{} {} ", ret, int_to_word(&left_int)?, POINTWORD);
            // `for digit in right: ret += _int_to_word(int(digit)) + " "`
            for ch in right.chars() {
                let d = ch
                    .to_digit(10)
                    .ok_or_else(|| int_value_error(&ch.to_string()))?;
                out.push_str(&int_to_word(&BigInt::from(d))?);
                out.push(' ');
            }
            // `return ret.strip()`
            Ok(out.trim().to_string())
        }
        None => {
            // `return (ret + _int_to_word(int(n))).strip()`
            let n_int = BigInt::from_str(n).map_err(|_| int_value_error(n))?;
            Ok(format!("{}{}", ret, int_to_word(&n_int)?).trim().to_string())
        }
    }
}

/// `forms[1] if n != 1 else forms[0]` — the inline plural test NN's
/// `to_currency` uses in place of `pluralize` (which it never calls).
///
/// Every NN entry carries exactly two forms, so the `Index` arm is
/// unreachable; it exists so a malformed table cannot panic.
fn pick(forms: &[String], n: &BigInt) -> Result<String> {
    let idx = if n.is_one() { 0 } else { 1 };
    forms
        .get(idx)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

pub struct LangNn {
    /// Built once in [`LangNn::new`]; the registry caches the struct in a
    /// `OnceLock`, so this table is constructed exactly once per process.
    forms: HashMap<&'static str, CurrencyForms>,
    /// The `CURRENCY_FORMS.get(currency, <default>)` fallback, resolved once.
    fallback: CurrencyForms,
}

impl LangNn {
    pub fn new() -> Self {
        let forms = FORMS
            .iter()
            .map(|(code, unit, subunit)| (*code, CurrencyForms::new(unit, subunit)))
            .collect();
        let (_, unit, subunit) = &FORMS[FALLBACK_INDEX];
        LangNn {
            forms,
            fallback: CurrencyForms::new(unit, subunit),
        }
    }
}

impl Default for LangNn {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangNn {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NOK"
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
        POINTWORD
    }


    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python works on `str(number).strip()` and strips a leading "-"
        // textually before `int()`-ing the rest; for integer input that is
        // exactly a sign test plus abs().
        let (sign, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        // The trailing `.strip()` is a no-op for integer input (negword's
        // trailing space is always followed by a non-empty word, and
        // `_int_to_word` never returns "" — zero yields "null"). Mirrored
        // anyway to match the source line for line.
        Ok(format!("{}{}", sign, int_to_word(&magnitude)?)
            .trim()
            .to_string())
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // bug 6 (#259): the last word takes its ordinal form; negatives are
        // still accepted, as Python accepted them.
        let cardinal = self.to_cardinal(value)?;
        let (head, last) = match cardinal.rsplit_once(' ') {
            Some((h, l)) => (Some(h), l),
            None => (None, cardinal.as_str()),
        };
        let word = match ordinal_word(last) {
            Some(w) => w,
            // "null" has no attested Nynorsk ordinal; it stays as is.
            None => return Ok(cardinal),
        };
        Ok(match head {
            // A bare scale word drops its "ein": "hundrede", "millionte".
            Some("ein") if [HUNDRED, THOUSAND, MILLION].contains(&last)
                || HIGH_SCALES.iter().any(|(_, w)| *w == last) => word,
            Some(h) => format!("{} {}", h, word),
            None => word,
        })
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // `str(number) + "."` — the sign survives: -1 -> "-1.".
        Ok(format!("{}.", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // `to_year(val, longval=True)` ignores longval entirely and delegates.
        // No BC/AD handling, so -500 -> "minus fem hundre".
        self.to_cardinal(value)
    }

    /// Python's `to_cardinal` float/Decimal branch. NN overrides `to_cardinal`
    /// outright, so it never reaches `base.float2tuple`/`to_cardinal_float`:
    /// there is no rounding, no `< 0.01` heuristic, and `self.precision` is
    /// never read. It just does `n = str(number)`, splits on `"."`, words the
    /// integer part with `_int_to_word`, and words each fractional digit —
    /// see [`string_to_words`].
    ///
    /// Because there is no `float2tuple`, the f64-artefact trap does *not*
    /// apply here: `str(2.675)` is `"2.675"`, so the digits are `6 7 5`
    /// (`seks sju fem`) straight off the repr — the same 675 the artefact
    /// heuristic rescues elsewhere, but reached by a different route.
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**, matching
    /// Python: `num2words` pops `precision=` and only reapplies it when the
    /// converter carries a `.precision` attribute. `Num2Word_NN` never sets
    /// one (it defines no numword tables, so `Num2Word_Base.__init__`'s
    /// `self.precision = 2` line is never reached), so the kwarg is dropped.
    ///
    /// Reconstructing `str(number)`:
    /// * **Float** — [`py_float_str`]: `format!("{:.p}", value)` inside repr's
    ///   fixed window (byte-identical to `str(value)`, sign and negative zero
    ///   included; verified against every NN float corpus row), Python-style
    ///   `1e+16`/`1.5e-05` exponent form outside it. The exponent forms
    ///   contain no "." — they exist so the same inputs raise the same
    ///   `ValueError`s Python does (bug 9).
    /// * **Decimal** — [`python_decimal_str`], CPython's `Decimal.__str__`:
    ///   scale is preserved, so `"1.10"` keeps its trailing zero
    ///   (`ein point ein zero`), the #603 value `98746251323029.99` never
    ///   float-casts, and the exponent-form strings (`"1E+2"`, `"1E+20"`,
    ///   `"1E-7"`) come out exactly as Python prints them — and then fail
    ///   `int()` exactly as Python does.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let n = match value {
            FloatValue::Float { value, precision } => py_float_str(*value, *precision),
            FloatValue::Decimal { value, .. } => python_decimal_str(value),
        };
        string_to_words(&n)
    }

    // ---- float/Decimal routing ----------------------------------------

    /// `to_cardinal(float/Decimal)` full routing. Python's `to_cardinal` keys
    /// on `str(number)`, so *every* float/Decimal takes the string route —
    /// whole values included: 5.0 keeps its ".0" tail ("fem komma null"),
    /// `Decimal("5.00")` speaks both zeros, and only a point-less string
    /// (`Decimal("5")` -> "5") lands on the integer branch, via the same
    /// `int(n)` call. Base's whole-value shortcut never runs.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`. `Num2Word_NN.to_ordinal` is
    /// `self.to_cardinal(number) + "-de"` with **no type guard**, so a float
    /// or Decimal rides the same string route as `to_cardinal` and then takes
    /// the suffix: `to_ordinal(5.0)` == "fem komma null-de", `to_ordinal(-0.0)`
    /// == "minus null komma null-de", and a whole `Decimal("100")` ==
    /// "ein hundre-de". An exponent-notation repr (`str(1e16)` == "1e+16",
    /// `str(Decimal("1E+2"))` == "1E+2") makes the inner `int()` raise its
    /// ValueError *before* Python's `+ "-de"` runs — the `?` reproduces that
    /// ordering.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-de", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` never casts to
    /// int, so it *succeeds* on every float and Decimal — "5.0.", "-0.0.",
    /// "1e+16." and "1E+2." are all real Python outputs. `repr_str` is the
    /// binding's Python `str(value)`, exactly the string Python concatenates.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    // `year_float_entry` is deliberately NOT overridden: the trait default
    // routes to `cardinal_float_entry`, and Python's `to_year` is a plain
    // `return self.to_cardinal(val)` — identical by construction.

    /// `converter.str_to_number` — NN inherits `Num2Word_Base`'s
    /// `Decimal(value)`, so `"Infinity"`/`"NaN"` strings parse *successfully*
    /// and only blow up later, inside `to_cardinal`'s `int("Infinity")` /
    /// `int("NaN")`. The non-finite sentinels pass straight through here and
    /// the mode-aware [`inf_result`](Self::inf_result) / [`nan_result`](Self::nan_result)
    /// hooks reproduce the exact per-mode Python behaviour natively — no
    /// fallback. Finite parses are returned unchanged. See bug 10.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `-Infinity`. `to_cardinal` strips a leading
    /// "-" and then `int("Infinity")` raises `ValueError: invalid literal for
    /// int() with base 10: 'Infinity'` (both signs quote 'Infinity', the sign
    /// having been stripped); `to_ordinal` appends "-de" *after* that int(),
    /// so it raises identically, and `to_year` == `to_cardinal`.
    /// `to_ordinal_num` is `str(number) + "."` and never calls `int()`, so it
    /// succeeds with "Infinity." / "-Infinity." (bug 9).
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        if to == "ordinal_num" {
            return Ok(format!("{}Infinity.", if negative { "-" } else { "" }));
        }
        Err(int_value_error("Infinity"))
    }

    /// `Decimal('NaN')`. `str(Decimal('NaN'))` is "NaN"; `int("NaN")` raises
    /// `ValueError: invalid literal for int() with base 10: 'NaN'` on the
    /// cardinal/ordinal/year paths, while `to_ordinal_num` echoes "NaN.".
    fn nan_result(&self, to: &str) -> Result<String> {
        if to == "ordinal_num" {
            return Ok("NaN.".into());
        }
        Err(int_value_error("NaN"))
    }

    // ---- currency ----------------------------------------------------
    //
    // `Num2Word_NN` overrides `to_currency` outright and inherits `to_cheque`
    // from `Num2Word_Base`. The two disagree about unknown codes, and that
    // asymmetry is real (bug 7): `to_currency` does
    // `CURRENCY_FORMS.get(currency, <first value>)` and silently renders an
    // unknown code as *kroner*, while `to_cheque` does
    // `CURRENCY_FORMS[currency]` and turns the KeyError into a
    // NotImplementedError. The corpus pins both:
    //
    //     currency:JPY 12.34 -> "tolv kroner tretti fire øre"
    //     cheque:JPY   1234.56 -> NotImplementedError
    //
    // So `currency_forms` below reports only the three real codes (which is
    // what drives the inherited `to_cheque` to raise), and `to_currency`
    // applies the fallback itself rather than going through the hook.
    //
    // Not overridden, deliberately:
    // * `currency_adjective` — `CURRENCY_ADJECTIVES` is `{}`, and NN's
    //   `to_currency` ignores the `adjective` argument outright anyway.
    // * `currency_precision` — `CURRENCY_PRECISION` is `{}`, so every code is
    //   100. NN's `to_currency` never consults it at all (it hardcodes two
    //   decimal places), which is why JPY renders øre and KWD renders /100.
    // * `pluralize` — NN never calls it (it indexes the forms inline) and
    //   defines none, so the trait default (NotImplementedError, matching
    //   `Num2Word_Base.pluralize`) is correct and unreachable.
    // * `money_verbose` — base's default delegates to `to_cardinal`, which is
    //   what `Num2Word_NN` inherits; the trait default already does this.
    // * `cents_verbose` / `cents_terse` / `cardinal_from_decimal` — NN's
    //   `to_currency` reaches none of them (`cents=False` drops the segment
    //   entirely rather than falling back to terse digits).

    fn lang_name(&self) -> &str {
        "Num2Word_NN"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        // Deliberately *not* the fallback lookup — this hook backs the
        // inherited `to_cheque`, whose `CURRENCY_FORMS[currency]` raises.
        self.forms.get(code)
    }

    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // NN's signature takes `adjective=False` and then never reads it.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Python's `separator=" "` default cannot survive the FFI boundary:
        // the shim and the diff harness both pass base's "," when the caller
        // supplied nothing, so "," is the only available "unset" signal. Map
        // it back to NN's own default. This matches the Python converter for
        // every call except an explicit `separator=","`, which is
        // indistinguishable from the default here — see `concerns`.
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")`, evaluated *after* the abs().
        //
        // The Decimal arm carries `str(value)` as parsed on the Python side,
        // and BigDecimal's Display round-trips it (scale is preserved, so
        // "1.0" stays "1.0" and keeps its empty-cents part). The Int arm has
        // no ".", so `parts` has length 1 and cents are skipped — but note
        // that is *not* the int/float distinction base.to_currency draws:
        // NN reaches the same result for 1 and 1.0 because "1.0" yields
        // parts[1] == "0" -> right == 0 -> the falsy `and right` test.
        let text = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };
        let parts: Vec<&str> = text.split('.').collect();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if parts[0].is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(parts[0]).map_err(|_| int_value_error(parts[0]))?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // Truncation, not rounding: 12.999 -> "99" -> nitti ni øre, and
        // 0.001 -> "00" -> 0 -> no cents segment at all.
        let right = match parts.get(1) {
            Some(frac) if !frac.is_empty() => {
                let mut digits: String = frac.chars().take(2).collect();
                while digits.chars().count() < 2 {
                    digits.push('0');
                }
                BigInt::from_str(&digits).map_err(|_| int_value_error(&digits))?
            }
            _ => BigInt::zero(),
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(...values())[0])`
        let forms = self.forms.get(currency).unwrap_or(&self.fallback);

        let mut result = format!("{} {}", int_to_word(&left)?, pick(&forms.unit, &left)?);

        // `if cents and right:` — a zero `right` drops the segment, and so
        // does `cents=False` (NN has no terse-digits branch).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                int_to_word(&right)?,
                pick(&forms.subunit, &right)?
            ));
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
