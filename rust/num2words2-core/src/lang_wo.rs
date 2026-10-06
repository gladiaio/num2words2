//! Port of `lang_WO.py` (Wolof).
//!
//! Registry check: `__init__.py` maps `"wo"` → `lang_WO.Num2Word_WO()`, which is
//! the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_WO` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the `any(hasattr(...))`
//! guard in `Num2Word_Base.__init__` never fires: Python never builds
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int_to_word` recursively. Consequently `cards`/`maxval`/
//! `merge` stay at their trait defaults here, and there is **no overflow check** —
//! `_int_to_word` silently degrades to `str(number)` above 10^9 instead (see
//! below). `Num2Word_Base.splitnum`/`clean`/`merge` are unreachable.
//!
//! Inherited from `Num2Word_Base` but overridden by WO, so the trait defaults are
//! *not* used: `to_ordinal`, `to_ordinal_num`, `to_year` all have WO bodies.
//! Notably WO's `to_ordinal` never calls `verify_ordinal`, so the base class's
//! "Cannot treat negative num %s as ordinal." TypeError is unreachable — see
//! bug 2 below.
//!
//! # The float/Decimal path
//!
//! `Num2Word_WO` does **not** override `to_cardinal_float`; `Num2Word_Base`'s
//! never runs either. WO overrides `to_cardinal` and handles non-integers
//! inline via `str(number)`, *before* the base class gets a look in. So none of
//! `floatpath.rs` applies here — **no `float2tuple`, no `10**precision`
//! scaling, no banker's rounding, and none of the f64 artefacts that path
//! exists to preserve**. WO's whole float path is
//!
//! ```python
//! n = str(number).strip()
//! ```
//!
//! and then pure string surgery on the result: split at the first ".", `int()`
//! the left, and map each *character* of the right through `_int_to_word`.
//! Consequences, opposite to every language that inherits base's path:
//!
//! * **The artefact cases are trivially right.** `str(2.675)` is `"2.675"`
//!   (`repr` is shortest-round-trip), so the fraction digits are `6 7 5` by
//!   construction — WO never scales, so there is nothing to rescue.
//! * **`str()` is the whole spec**, so this port lives or dies on reproducing
//!   `repr(float)` and `str(Decimal)` byte for byte — see
//!   [`python_float_repr`] and [`python_decimal_str`], ported unchanged from the
//!   sibling `lang_bm.rs`, which is the same shape.
//! * **The fraction digits go through `_int_to_word`, not a bare `ones[]`
//!   lookup** (this is where WO differs from BM). `_int_to_word(0)` is `"zero"`
//!   (bug 3), so `0.01` → `"zero point zero benn"`, not `"... <empty> benn"`.
//! * **Trailing zeros are significant** — they are characters, not a computed
//!   remainder: `Decimal("1.10")` → `"benn point benn zero"`.
//! * **Exponent notation raises `ValueError`**, since `int()` chokes on the
//!   literal — the same hole [`parse_int`] documents for currency. `1e16` →
//!   `"1e+16"` → no `"."` → `int("1e+16")` raises quoting the whole literal;
//!   `1.5e16` → `"1.5e+16"` → the `"."` branch → `int("e")` raises quoting just
//!   the offending *character*. `Decimal("1E+16")` fails identically, capital E
//!   and all. All verified against the live interpreter.
//!
//! `precision=` (issue #580) is threaded in by `__init__.py` as
//! `converter.precision`, which WO's `to_cardinal` never reads — so
//! `precision_override` is accepted and **ignored**. Confirmed:
//! `num2words(2.675, lang="wo", precision=1)` is still the full fraction.
//!
//! # Currency
//!
//! `Num2Word_WO` declares its **own** `CURRENCY_FORMS` class attribute (XOF, USD,
//! EUR), so it is untouched by the `Num2Word_EN.__init__` mutation that rewrites
//! `Num2Word_EUR.CURRENCY_FORMS` in place. Verified against the live interpreter:
//!
//! ```text
//! {'EUR': (('euro', 'euros'), ('cent', 'cents')),
//!  'USD': (('dollar', 'dollars'), ('cent', 'cents')),
//!  'XOF': (('dërëm', 'dërëm'), ('santim', 'santim'))}
//! ```
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited from
//! `Num2Word_Base` and are `{}`, so `currency_adjective` stays `None` and
//! `currency_precision` stays 100 for *every* code — including JPY, KWD and BHD.
//! WO's `to_currency` never consults `CURRENCY_PRECISION` at all, so the
//! 0-decimal / 3-decimal special cases in `base.to_currency` are unreachable
//! here. The corpus confirms: `currency:JPY 12.34` → "fukk ñaar dërëm ñett-fukk
//! ñeent santim", i.e. cents are shown for a nominally 0-decimal currency.
//!
//! `pluralize`, `_cents_verbose` and `_cents_terse` are never reached: WO
//! overrides `to_currency` wholesale and picks its plural form by hand, and
//! `to_cheque` (inherited unchanged from `Num2Word_Base`) uses neither. So
//! `pluralize` keeps the trait default that raises, exactly as `Num2Word_WO`
//! inherits `Num2Word_Base.pluralize`'s bare `raise NotImplementedError`.
//!
//! `to_cheque` is **not** overridden by WO, so the trait default
//! (`currency::default_to_cheque`, the port of `Num2Word_Base.to_cheque`) serves
//! it. It looks the code up strictly — `self.CURRENCY_FORMS[currency]` — so
//! `currency_forms` below returns `None` for an unknown code and the cheque path
//! raises NotImplementedError, while `to_currency` silently falls back to XOF
//! (bug 8). That asymmetry is real and the corpus pins both halves of it.
//!
//! # Further faithfully reproduced Python bugs / oddities (currency)
//!
//! 7. **`to_currency` parses `str(val)`, not the number.** It never uses
//!    `parse_currency_parts`; it splits the *decimal rendering* on "." and takes
//!    `parts[1][:2]` — a **truncation to two digits, not a rounding**. So
//!    `1.005` → "benn euro" (cents truncate to "00", and `if cents and right`
//!    then drops the segment entirely) and `1.567` → "benn euro juróom-fukk
//!    juróom-benn cents" (56, not 57).
//! 8. **An unknown currency code never raises.**
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    falls back to the **first value in dict-literal order**, which is XOF. So
//!    `to_currency(1, currency="ZZZ")` == "benn dërëm" rather than
//!    NotImplementedError. The corpus pins this for GBP/JPY/KWD/BHD/INR/CNY/CHF,
//!    all of which come out as dërëm/santim. This is why the fallback entry is
//!    stored in its own field: a `HashMap` has no insertion order to fall back
//!    through.
//! 9. **`cents=False` drops the cents segment instead of rendering digits.**
//!    `if cents and right:` — base's `cents=False` means "terse digits"
//!    (`_cents_terse`), but WO reads it as "no cents at all":
//!    `to_currency(12.34, cents=False)` == "fukk ñaar euros".
//! 10. **`adjective` is accepted and completely ignored.** No
//!    `CURRENCY_ADJECTIVES` lookup, no `prefix_currency` call.
//! 11. **A float with zero cents renders no cents.** `str(1.0)` → parts
//!    `["1", "0"]` → `right = int("0".ljust(2, "0"))` = 0 → falsy → segment
//!    skipped. So `to_currency(1.0)` == "benn euro", *not* "benn euro zero
//!    cents". This is the one place WO does **not** honour the int/float split
//!    that `base.to_currency` makes — but the split still reaches WO intact and
//!    must not be collapsed, because `str(1)` == "1" (one part) and `str(1.0)`
//!    == "1.0" (two parts) take different branches to get there.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, and are confirmed against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9.** The final `else` returns
//!    `str(number)` — bare digits, no words. So `to_cardinal(10**9)` ==
//!    "1000000000" and `to_cardinal(10**21)` == "1000000000000000000000". Wolof
//!    has no billion word in the table (`self.million = "tamndareet"` is the
//!    highest), and rather than raising OverflowError the module emits the
//!    numeral. This is why the value is **not bounded** and must stay a BigInt:
//!    the fallback has to render arbitrarily large inputs verbatim.
//! 2. **`to_ordinal` has no negative/zero guard.** It is a blind
//!    `to_cardinal(number) + "-eel"`, so `to_ordinal(0)` == "zero-eel" and
//!    `to_ordinal(-1)` == "minus benn-eel" — the suffix lands on the *last word*
//!    of a multi-word cardinal, e.g. `to_ordinal(100)` == "benn téeméer-eel".
//!    Combined with bug 1, `to_ordinal(10**9)` == "1000000000-eel".
//! 3. **`_int_to_word(0)` is a tautology.** Python writes
//!    `return self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""`,
//!    which is falsy, so the branch unconditionally yields "zero" and the
//!    `self.ones[0]` arm is dead code.
//! 4. **`_int_to_word`'s `number < 0` arm is unreachable.** `to_cardinal`
//!    detaches the sign from the *string* before calling `int()`, and no
//!    recursive call ever passes a negative. Mirrored anyway, since Python has
//!    it. (It would double the space: negword is "minus " *with* a trailing
//!    space, and this arm concatenates without `.strip()`.)
//! 5. **`to_ordinal_num` produces no words and keeps the sign.**
//!    `str(number) + "."` — so `to_ordinal_num(-1)` == "-1." and
//!    `to_ordinal_num(0)` == "0.".
//! 6. **Hundreds always carry an explicit "benn".** `self.ones[hundreds_val] +
//!    " " + self.hundred` has no `hundreds_val == 1` special case, so 100 is
//!    "benn téeméer" ("one hundred"), never bare "téeméer". Same for thousands
//!    and millions via the recursion: 1000 == "benn junni".
//!
//! No cross-call mutable state: `setup()` only assigns constant tables, and no
//! method sets a flag that another consumes. The Rust path being stateless is
//! safe here.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.negword` — note the trailing space, which is load-bearing: `to_cardinal`
/// concatenates it directly onto the magnitude with no separator.
const NEGWORD: &str = "minus ";

/// `self.pointword`. Live on the float path, where WO interpolates it raw
/// (with a space on either side) between the integral part and the spelled-out
/// fraction digits: `int(left) + " " + pointword + " " + digits…`.
const POINTWORD: &str = "point";

/// `self.ones`. Index 0 is `""` and is only ever reached via the dead
/// `ones[0]` arm of the zero check (bug 3).
const ONES: [&str; 10] = [
    "",
    "benn",
    "ñaar",
    "ñett",
    "ñeent",
    "juróom",
    "juróom-benn",
    "juróom-ñaar",
    "juróom-ñett",
    "juróom-ñeent",
];

/// `self.tens`. Index 0 is `""`, guarded by the `number < 10` branch above it.
const TENS: [&str; 10] = [
    "",
    "fukk",
    "ñaar-fukk",
    "ñett-fukk",
    "ñeent-fukk",
    "juróom-fukk",
    "juróom-benn-fukk",
    "juróom-ñaar-fukk",
    "juróom-ñett-fukk",
    "juróom-ñeent-fukk",
];

const HUNDRED: &str = "téeméer";
const THOUSAND: &str = "junni";
const MILLION: &str = "tamndareet";

/// The value at which `_int_to_word` gives up and returns `str(number)` (bug 1).
const BILLION: u32 = 1_000_000_000;

pub struct LangWo {
    /// `Num2Word_WO.CURRENCY_FORMS`, built once. The registry caches the
    /// instance in a `OnceLock`, so `new()` runs at most once per process;
    /// rebuilding this per call is what made an earlier port 10x slower than
    /// the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the XOF entry, i.e. the first
    /// value in the class-body dict literal's insertion order. `to_currency`
    /// falls back to it for any unknown code (bug 8). Held separately because a
    /// `HashMap` has no insertion order to recover it from.
    currency_forms_fallback: CurrencyForms,
}

impl LangWo {
    pub fn new() -> Self {
        // Insertion order of the class-body literal is XOF, USD, EUR; only the
        // first entry's identity matters (it is the .get() default), and that is
        // captured in `currency_forms_fallback` rather than left to the map.
        let xof = CurrencyForms::new(&["dërëm", "dërëm"], &["santim", "santim"]);
        let mut currency_forms = HashMap::new();
        currency_forms.insert("XOF", xof.clone());
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]),
        );
        LangWo {
            currency_forms,
            currency_forms_fallback: xof,
        }
    }
}

impl Default for LangWo {
    fn default() -> Self {
        Self::new()
    }
}

/// Python's `_int_to_word`, verbatim including the `str(number)` fallback.
fn int_to_word(number: &BigInt) -> String {
    // `if number == 0: return self.ones[0] if self.ones[0] else "zero"`.
    // ones[0] is "" (falsy), so this is unconditionally "zero" (bug 3).
    if number.is_zero() {
        return "zero".to_string();
    }

    // Unreachable from to_cardinal/to_ordinal/to_year (bug 4) — mirrored anyway.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    // `else: return str(number)` — the very-large-number fallback (bug 1). This
    // must render the full BigInt; the value is NOT bounded here.
    if number >= &BigInt::from(BILLION) {
        return number.to_string();
    }

    // Below 10^9 the value provably fits a u32 (999_999_999 < u32::MAX), so the
    // rest of the recursion can use machine arithmetic without changing
    // behaviour. Python's `//` and `%` agree with Rust's `/` and `%` here
    // because both operands are non-negative.
    int_to_word_small(number.to_u32().expect("checked < 10^9 and >= 0 above"))
}

/// The `1 ..= 999_999_999` arm of `_int_to_word`.
///
/// Every recursive call below shrinks into a lower bracket (`thousands_val` and
/// `millions_val` are both <= 999_999 / 999 respectively), so recursion always
/// terminates and never re-enters the BigInt fallback.
fn int_to_word_small(number: u32) -> String {
    if number < 10 {
        // `return self.ones[number]`
        ONES[number as usize].to_string()
    } else if number < 100 {
        let tens_val = (number / 10) as usize;
        let ones_val = (number % 10) as usize;
        if ones_val == 0 {
            TENS[tens_val].to_string()
        } else {
            format!("{} {}", TENS[tens_val], ONES[ones_val])
        }
    } else if number < 1_000 {
        // `result = self.ones[hundreds_val] + " " + self.hundred` — the "benn"
        // in "benn téeméer" comes from here (bug 6).
        let hundreds_val = (number / 100) as usize;
        let remainder = number % 100;
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else if number < 1_000_000 {
        let thousands_val = number / 1_000;
        let remainder = number % 1_000;
        let mut result = format!("{} {}", int_to_word_small(thousands_val), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else {
        // number < 1_000_000_000, guaranteed by the caller.
        let millions_val = number / 1_000_000;
        let remainder = number % 1_000_000;
        let mut result = format!("{} {}", int_to_word_small(millions_val), MILLION);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    }
}

/// Python's `int(s)` on a fragment of a `str()` rendering.
///
/// Two callers: `to_currency` (where the fragments are always plain digit
/// strings, so it never raises) and the float path's [`cardinal_from_str`]
/// (where it *is* reachable — `str(1e16)` is `"1e+16"`, and `int("1e+16")`
/// raises `ValueError`). The message quotes the offending literal verbatim,
/// exactly as CPython's does, so the failure keeps its type and text rather
/// than being unwrapped into a panic.
fn parse_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// CPython's `repr(float)` — which for a `float` is also `str(float)`, and
/// therefore the entire input to WO's float path.
///
/// Ported unchanged from the sibling `lang_bm.rs` (same self-contained
/// `str(number)` shape), where it is documented in full and differentially
/// tested against CPython on 300k doubles: 678 mismatches without the tie-repair
/// step, 0 with it. Two halves.
///
/// **The digits.** `repr` is shortest-round-trip, and so is Rust's `{:e}` — but
/// on an exact tie they disagree (CPython breaks to even; Rust's shortest
/// formatter does neither consistently, ~1 double in 10,000). So the fractional
/// *count* is taken from `{:e}` (a tie cannot change it — a carry would yield a
/// shorter form the algorithm would have found first) and the digits are
/// re-derived through Rust's exact `{:.n$}`, which **is** round-half-to-even.
/// `670352580196876.25` is such a tie: CPython prints `670352580196876.2`, and
/// WO must then say `ñaar`, not `ñett`.
///
/// **The placement.** CPython uses exponent notation iff `decpt <= -4 ||
/// decpt > 16`, pads the exponent to two digits, and appends `.0` to anything
/// that would otherwise look integral. Rust's `{}` does none of this. Both
/// matter: `str(1.0)` is `"1.0"` and `str(1e16)` is `"1e+16"` (→ ValueError).
/// The `precision` on `FloatValue::Float` is deliberately not used to shortcut
/// this — for an exponent-form repr it is the exponent, not a digit count.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). WO feeds these straight to int(),
    // which rejects them like any other bad literal ("nan"/"inf"/"-inf").
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `{:e}` is shortest-round-trip in `<d>[.<ddd>]e<exp>` form. `decpt` is
    // CPython's: the value is `0.<digits> * 10**decpt`.
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').expect("LowerExp always emits an 'e'");
    let exp: i32 = exp.parse().expect("LowerExp emits an integer exponent");
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut decpt = exp + 1;

    // Tie repair — see the doc comment. Only reachable when the shortest form
    // has fractional digits; `a == 0.0` is excluded (`{:e}` reports it "0e0").
    let frac_digits = digits.chars().count() as i32 - decpt;
    if frac_digits > 0 && a != 0.0 {
        let t = format!("{:.*}", frac_digits as usize, a);
        let (ip, fp) = t.split_once('.').expect("frac_digits > 0 forces a point");
        let all = format!("{}{}", ip, fp);
        let trimmed = all.trim_start_matches('0');
        if !trimmed.is_empty() {
            let lead = all.chars().count() - trimmed.chars().count();
            digits = trimmed.to_string();
            decpt = ip.chars().count() as i32 - lead as i32;
        }
    }

    let n = digits.chars().count() as i32;

    if decpt <= -4 || decpt > 16 {
        // CPython: mantissa, then "e", then "%+.02d" of decpt-1.
        let e = decpt - 1;
        let mut out = String::from(sign);
        let mut it = digits.chars();
        out.push(it.next().expect("a finite double has at least one digit"));
        if n > 1 {
            out.push('.');
            out.push_str(it.as_str());
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (e as i64).abs()));
        out
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= n {
        // Py_DTSF_ADD_DOT_0: an integral value still reprs with a ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - n) as usize))
    } else {
        let k = decpt as usize;
        format!(
            "{}{}.{}",
            sign,
            digits.chars().take(k).collect::<String>(),
            digits.chars().skip(k).collect::<String>()
        )
    }
}

/// CPython's `str(Decimal)` — the spec's to-scientific-string, ported unchanged
/// from `lang_bm.rs` (transcribed from `_pydecimal.Decimal.__str__`).
///
/// `BigDecimal` is the same `(unscaled, scale)` pair as Python's `(_int, _exp)`
/// with `_exp == -scale`, and `from_str` preserves the scale as written, so
/// `Decimal("1.10")`'s trailing zero survives the crossing (→ `"benn point benn
/// zero"`). This reads `as_bigint_and_exponent()` rather than `BigDecimal`'s own
/// `Display`, which is **not** `str(Decimal)`: Display renders `Decimal("0.00")`
/// as `"0"`, losing the two digits WO would have spoken. The capital `E` and the
/// unpadded exponent are Python's too — `str` gives `"1E+16"`.
///
/// The negative-zero hole (`Decimal("-0.0")` loses its sign because `BigInt`
/// has no negative zero) is the same one `lang_bm.rs` documents; flagged in the
/// port report. Not in the corpus.
fn python_decimal_str(d: &BigDecimal) -> String {
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let sign = if unscaled.is_negative() { "-" } else { "" };
    let int_digits = unscaled.abs().to_string();
    let exp: i64 = -scale;
    let ndig = int_digits.chars().count() as i64;
    let leftdigits = exp + ndig;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= ndig {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - ndig) as usize)),
            String::new(),
        )
    } else {
        let k = dotplace as usize;
        (
            int_digits.chars().take(k).collect::<String>(),
            format!(".{}", int_digits.chars().skip(k).collect::<String>()),
        )
    };

    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}

/// `str(number)` for whatever the Python dispatcher handed the converter.
///
/// The `FloatValue` split is exactly Python's `isinstance(value, Decimal)`: the
/// two arms stringify by different rules and must not be collapsed.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// `Num2Word_WO.to_cardinal`, driven by the `str()` rendering rather than the
/// value — the non-integer path.
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     n = n[1:]
///     ret = self.negword
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
/// Points of order, all corpus-pinned:
///
/// * Unlike the sibling `lang_bm.rs`, WO does **not** recurse on the sign: it
///   strips one leading "-", sets `ret = negword` ("minus ", trailing space
///   load-bearing), and carries `ret` as a prefix through both branches.
/// * Each fraction character goes through `_int_to_word(int(digit))`, so digit
///   `0` becomes `"zero"` (bug 3), not the empty `ones[0]`. Every digit 0-9
///   maps into the `1..=999_999_999` machinery, which for a single digit just
///   returns the `ones`/`zero` word.
/// * `int(left)` runs **before** the fraction loop, so a bad left literal (e.g.
///   from exponent notation) raises first. `int(digit)` is per-character, so a
///   malformed fraction quotes one character (`'e'`) where a malformed whole `n`
///   quotes the entire literal (`'1e+16'`).
/// * The trailing `.strip()` is reproduced via `trim()`; it is a no-op in
///   practice (negword's space is absorbed by the following word, and the
///   fraction loop's trailing space is what `strip` removes).
fn cardinal_from_str(n: &str) -> Result<String> {
    // n = str(number).strip()
    let n = n.trim();

    // if n.startswith("-"): n = n[1:]; ret = negword  else: ret = ""
    let (ret_prefix, n): (&str, &str) = match n.strip_prefix('-') {
        Some(rest) => (NEGWORD, rest),
        None => ("", n),
    };

    if let Some((left, right)) = n.split_once('.') {
        // ret += int_to_word(int(left)) + " " + pointword + " "
        // `int(left)` is evaluated first — a bad literal raises before the loop.
        let mut ret = format!(
            "{}{} {} ",
            ret_prefix,
            int_to_word(&parse_int(left)?),
            POINTWORD
        );
        // for digit in right: ret += int_to_word(int(digit)) + " "
        for ch in right.chars() {
            // int(digit) per character: a non-digit raises ValueError quoting it.
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    ch
                ))
            })?;
            ret.push_str(&int_to_word(&BigInt::from(d)));
            ret.push(' ');
        }
        // return ret.strip()
        Ok(ret.trim().to_string())
    } else {
        // return (ret + int_to_word(int(n))).strip()
        Ok(format!("{}{}", ret_prefix, int_to_word(&parse_int(n)?))
            .trim()
            .to_string())
    }
}

impl Lang for LangWo {

    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("comma nulla"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`. WO's `to_ordinal` is
    /// `self.to_cardinal(number) + "-eel"` for *every* input, so the float
    /// entry is the float cardinal plus the suffix — "juróom point zero-eel".
    /// An exponent-form Decimal repr ("1E+2") still dies in `int()` with
    /// ValueError inside the cardinal, before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-eel", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr the
    /// binding computed, dot appended, sign and exponent form included
    /// ("-0.0.", "1e+16.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside WO's `int("Infinity")` (`to_cardinal` reads
    /// `str(number)`, strips the sign, finds no "." and calls `int()`).
    /// The binding otherwise hard-codes `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any WO code runs, so the
    /// ValueError must be raised here. The sign is sliced off before
    /// `int()` sees it, so both signs quote the same literal.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "XOF"
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
        "point"
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     ...                      # float branch, dead for integer input
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    /// The sign is detached from the *string*, so `_int_to_word` only ever sees a
    /// magnitude — which is why its `number < 0` arm is dead (bug 4). The
    /// `"." in n` branch cannot fire for integer input.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let n = value.to_string();
        let (ret, magnitude_str) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", n.as_str()),
        };
        // `int(n)` on a sign-stripped decimal rendering of a BigInt — always
        // valid, so this cannot raise the ValueError that bites other modules.
        let magnitude: BigInt = magnitude_str
            .parse()
            .expect("decimal rendering of a BigInt minus its sign is always parseable");

        // Python's trailing `.strip()`: a no-op in practice (negword's trailing
        // space is consumed by the word that follows, and _int_to_word never
        // returns a padded string), but reproduced for fidelity.
        Ok(format!("{}{}", ret, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Python: `return self.to_cardinal(number) + "-eel"` — no verify_ordinal, no
    /// negative guard, no zero guard (bug 2). The suffix is glued to whatever the
    /// cardinal ends with, so it attaches to the final word of a phrase
    /// ("benn téeméer-eel") and even to bare digits ("1000000000-eel").
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-eel", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "."` — no words at all, and the sign
    /// survives: `to_ordinal_num(-1)` == "-1." (bug 5).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`
    /// — `longval` is accepted and ignored, there is no two-digit-pair year
    /// idiom, and negative years get no "BC" treatment, just the negword:
    /// `to_year(-500)` == "minus juróom téeméer".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// WO reaches non-integers through its own `to_cardinal` override, not
    /// through `Num2Word_Base.to_cardinal_float`, so this hook exists only to
    /// intercept the base implementation and run WO's string surgery instead.
    /// There is no `float2tuple` here and no rounding of any kind — see the
    /// module docs and [`cardinal_from_str`].
    ///
    /// `precision_override` (the `precision=` kwarg) is assigned by
    /// `__init__.py` as `converter.precision`, which WO's `to_cardinal` never
    /// reads, so it is accepted and **ignored** — matching the live interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        cardinal_from_str(&python_str(value))
    }

    // ---- currency ---------------------------------------------------------

    /// `self.__class__.__name__`, for the NotImplementedError `to_cheque` raises.
    fn lang_name(&self) -> &str {
        "Num2Word_WO"
    }

    /// `self.CURRENCY_FORMS[currency]` — a **strict** lookup, matching the
    /// `try: cr1, _cr2 = self.CURRENCY_FORMS[currency] except KeyError:` in
    /// `Num2Word_Base.to_cheque`, which is the only caller of this hook here.
    /// `to_currency` deliberately does *not* go through it: it uses `.get()`
    /// with an XOF default and never raises (bug 8).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_WO.to_currency`:
    ///
    /// ```python
    /// def to_currency(self, val, currency="XOF", cents=True, separator=" ",
    ///                 adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
    ///
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///
    ///     if is_negative:
    ///         result = self.negword + result
    ///
    ///     return result.strip()
    /// ```
    ///
    /// This shares nothing with `base.to_currency` — no `parse_currency_parts`,
    /// no `pluralize`, no `CURRENCY_PRECISION`, no NotImplementedError. It is a
    /// string-level reimplementation, and the `str(val)` it splits is what makes
    /// the Int/Decimal distinction reach here: `str(1)` == "1" has one part
    /// (`right` = 0), `str(1.0)` == "1.0" has two — which then *also* yields
    /// `right` = 0 via `int("0".ljust(2, "0"))`, so both print "benn euro"
    /// (bug 11). They agree, but by different routes, and only for zero cents.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // accepted and ignored by Python (bug 10)
    ) -> Result<String> {
        // Trait hands None when the caller omitted separator=;
        // resolve to this language's own default.
        let separator = separator.unwrap_or(self.default_separator());
        // `if val < 0: is_negative = True; val = abs(val)` — note the abs()
        // lands *before* the str(), so no part can carry a "-".
        let is_negative = val.is_negative();

        // `str(val)`. The Decimal arm was parsed from Python's own `str(value)`
        // on the shim side, so rendering it back reproduces the literal for
        // every plain decimal string — see `concerns` for the exponent case.
        let s = match val {
            CurrencyValue::Int(i) => i.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `parts = str(val).split(".")`, then parts[0] / parts[1]. Python splits
        // on every ".", so parts[1] stops at a second one; a decimal rendering
        // never has two, making `split` and a 2-way split equivalent.
        let mut parts = s.split('.');
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            parse_int(p0)?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // `[:2]` truncates to two digits — it does not round (bug 7).
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut padded: String = frac.chars().take(2).collect();
                while padded.chars().count() < 2 {
                    padded.push('0'); // ljust(2, "0")
                }
                parse_int(&padded)?
            }
            _ => BigInt::zero(),
        };

        // `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_forms_fallback);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`.
        // Every WO entry carries exactly two forms, so the [1] index is safe.
        let one = BigInt::one();
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so this is `right != 0`.
        // Note `cents=False` skips the segment rather than emitting digits the
        // way base's `_cents_terse` would (bug 9).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — negword is "minus " *with* its
        // trailing space, concatenated raw, which is what supplies the gap.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// `num2words(v, lang="wo")` for a float v.
    fn f(value: f64) -> String {
        let precision = {
            // Mirror the shim: precision = abs(exponent of Decimal(str(value))).
            // Not read by WO's path, but supplied as the binding would.
            let s = python_float_repr(value);
            match s.split_once('.') {
                Some((_, frac))
                    if !frac.contains(|ch: char| matches!(ch, 'e' | 'E' | '+' | '-')) =>
                {
                    frac.chars().count() as u32
                }
                _ => 0,
            }
        };
        LangWo::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// `num2words(Decimal(s), lang="wo")`.
    fn dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = (-value.as_bigint_and_exponent().1).unsigned_abs() as u32;
        LangWo::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `cardinal` corpus row with a dot in `arg` (float input).
    #[test]
    fn corpus_float() {
        assert_eq!(f(0.0), "zero point zero");
        assert_eq!(f(0.5), "zero point juróom");
        assert_eq!(f(1.0), "benn point zero");
        assert_eq!(f(1.5), "benn point juróom");
        assert_eq!(f(2.25), "ñaar point ñaar juróom");
        assert_eq!(f(3.14), "ñett point benn ñeent");
        assert_eq!(f(0.01), "zero point zero benn");
        assert_eq!(f(0.1), "zero point benn");
        assert_eq!(f(0.99), "zero point juróom-ñeent juróom-ñeent");
        assert_eq!(f(1.01), "benn point zero benn");
        assert_eq!(f(12.34), "fukk ñaar point ñett ñeent");
        assert_eq!(
            f(99.99),
            "juróom-ñeent-fukk juróom-ñeent point juróom-ñeent juróom-ñeent"
        );
        assert_eq!(f(100.5), "benn téeméer point juróom");
        assert_eq!(
            f(1234.56),
            "benn junni ñaar téeméer ñett-fukk ñeent point juróom juróom-benn"
        );
        assert_eq!(f(-0.5), "minus zero point juróom");
        assert_eq!(f(-1.5), "minus benn point juróom");
        assert_eq!(f(-12.34), "minus fukk ñaar point ñett ñeent");
        // The f64-artefact cases: repr is shortest-round-trip, so WO's string
        // path gets "1.005"/"2.675" for free — no rescue heuristic needed.
        assert_eq!(f(1.005), "benn point zero zero juróom");
        assert_eq!(f(2.675), "ñaar point juróom-benn juróom-ñaar juróom");
    }

    /// Every `cardinal_dec` corpus row (Decimal input) — trailing zeros and the
    /// >10^9 bare-digit fallback in the integral part both exercised.
    #[test]
    fn corpus_decimal() {
        assert_eq!(dec("0.01"), "zero point zero benn");
        assert_eq!(dec("1.10"), "benn point benn zero");
        assert_eq!(dec("12.345"), "fukk ñaar point ñett ñeent juróom");
        assert_eq!(
            dec("98746251323029.99"),
            "98746251323029 point juróom-ñeent juróom-ñeent"
        );
        assert_eq!(dec("0.001"), "zero point zero zero benn");
    }

    /// Not corpus rows; captured from the live interpreter.
    #[test]
    fn float_edges() {
        // -0.0 keeps its sign bit, so the negword survives.
        assert_eq!(f(-0.0), "minus zero point zero");
        // A tie CPython breaks to even: repr is "670352580196876.2", so ñaar.
        assert_eq!(f(670352580196876.25), "670352580196876 point ñaar");
        // Decimal with no fractional part takes the else branch.
        assert_eq!(dec("5"), "juróom");
        assert_eq!(dec("-5"), "minus juróom");
    }

    /// Exponent notation makes `int()` choke — the failure keeps ValueError's
    /// type and message. `str(1e16)` == "1e+16" (no dot) quotes the whole
    /// literal; `str(1.5e16)` == "1.5e+16" (dotted) quotes the char 'e'.
    #[test]
    fn exponent_literals_raise_value_error() {
        let whole = LangWo::new().to_cardinal_float(
            &FloatValue::Float {
                value: 1e16,
                precision: 16,
            },
            None,
        );
        match whole {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        let dotted = LangWo::new().to_cardinal_float(
            &FloatValue::Float {
                value: 1.5e16,
                precision: 16,
            },
            None,
        );
        match dotted {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'e'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // str(Decimal("1E+16")) == "1E+16": capital E, whole literal quoted.
        let d = LangWo::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str("1E+16").unwrap(),
                precision: 0,
            },
            None,
        );
        match d {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1E+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
    }

    /// `precision=` is ignored, exactly as the live interpreter shows.
    #[test]
    fn precision_override_ignored() {
        let full = LangWo::new()
            .to_cardinal_float(
                &FloatValue::Float {
                    value: 2.675,
                    precision: 3,
                },
                Some(1),
            )
            .unwrap();
        assert_eq!(full, "ñaar point juróom-benn juróom-ñaar juróom");
    }
}
