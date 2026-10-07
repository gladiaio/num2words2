//! Port of `lang_TK.py` (Turkmen).
//!
//! # Best-candidate words (gladiaio/num2words2#154)
//!
//! UNVERIFIED (#154): "otur" (decimal) — best candidate: the name of the
//!   comma, which Turkmen writes as decimal separator (enedilim.com); the
//!   standard reading is a fraction ("bitin ... -dan"), which the digit-by-digit
//!   path cannot produce.
//! UNVERIFIED (#154): "minus" is kept: it is Turkmen usage ("minus san",
//!   negative number).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "milliard" (10^9) and "trillion" (10^12) (Wiktionary;
//! business.com.tm, ashgabat.in), composed like the million arm, and raises
//! `OverflowError` from 10^15, which `maxval()` reports. Where the notes below
//! describe the digit fallback ("1000000000", "no overflow check"), they
//! describe Python; that arm is now unreachable.
//!
//! Shape: **self-contained**. `Num2Word_TK` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`/`merge`, so Python never builds `self.cards` and never
//! sets `MAXVAL`. `to_cardinal` is overridden outright and drives a plain
//! recursive `_int_to_word`. Consequently `cards`/`maxval`/`merge` stay at
//! their trait defaults here, and there is **no overflow check** — see the
//! 10^9 fallback below, which returns digits rather than raising.
//!
//! All four in-scope methods are overridden by the Python class, so nothing is
//! inherited from `Num2Word_Base` on the integer path:
//!   * `to_cardinal(number)`   → sign-strip, then `_int_to_word`
//!   * `to_ordinal(number)`    → `to_cardinal(number)` + a harmonised ending
//!     (upstream glued a fixed "-nji"; fixed, see quirk 4)
//!   * `to_ordinal_num(number)`→ `str(number) + "."`  (base returns `value`
//!     *unchanged*; TK's override appends a period, hence "0." not "0")
//!   * `to_year(val, longval=True)` → `self.to_cardinal(val)`, discarding
//!     base.py's era/"hundred"-style year logic entirely. Turkmen years are
//!     therefore just cardinals: 1905 → "bir müň dokuz ýüz bäş".
//!
//! No cross-call mutable state: `setup()` only assigns constant tables, and no
//! method stashes a flag for another to consume. Python's `str_to_number` is
//! not overridden, so there is no `_pending_ordinal`-style handshake to
//! preserve; the Rust-side [`Lang::str_to_number`] override below exists only
//! to surface the `int("Infinity")` ValueError the dispatcher would otherwise
//! misreport (see the float section).
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits:
//!
//! 1. **`_int_to_word` gives up at 10^9** and returns `str(number)` — the bare
//!    decimal digits — because the `elif` chain in Python ends at
//!    `number < 1000000000` with a `# Fallback for very large numbers` else.
//!    So `to_cardinal(10**9)` == "1000000000" (a numeral, not words), and
//!    `to_ordinal(10**9)` == "1000000000-nji". Corpus confirms this all the way
//!    up to 10^21. There is no `milliard`/`billion` word in `setup()` to reach
//!    for, and no OverflowError is raised. See [`int_to_word`].
//! 2. **Zero (fixed, gladiaio/num2words2#154).** `ones[0]` is the empty
//!    string, so Python's `return self.ones[0] if self.ones[0] else "zero"`
//!    always said the English "zero". The port says "nol": `to_cardinal(0)`
//!    == "nol", `to_ordinal(0)` == "nolunjy". The decimal word is the best
//!    candidate "otur" (see UNVERIFIED above).
//! 3. **Hundreds always carry an explicit "bir"**: `_int_to_word` builds
//!    `self.ones[hundreds_val] + " " + self.hundred` with no `> 1` guard, so
//!    100 → "bir ýüz", never the idiomatic bare "ýüz".
//! 4. **Fixed (gladiaio/num2words2#148): `to_ordinal` was suffix-only** — it
//!    appended "-nji" to the cardinal with no harmony ("bir-nji", "on-nji").
//!    The ending is now harmonised on the last word ("birinji", "onunjy",
//!    "altynjy", "dördünji", "bir ýüz ýigrimi üçünji"); see [`join_ordinal`].
//!    It still ordinalises negatives ("minus birinji") and the 10^9 numeral
//!    fallback ("1000000000-nji"), where most ports raise. Unlike `lang_PL`, **nothing on
//!    TK's integer surface raises** — no Index/Key/Value crash sites exist
//!    there. (The currency surface does raise; see quirk 6 and `to_currency`'s
//!    Errors section.)
//! 5. **`negword` is "minus " with a trailing space**, and the sign is stripped
//!    *textually* (`n.startswith("-")`) before `int()`, so the recursive
//!    `number < 0` arm of `_int_to_word` is unreachable from `to_cardinal`.
//!    The final `.strip()` cleans up the trailing space when the word part is
//!    empty — it never is here, but the strip is reproduced regardless.
//!
//! # The float / Decimal path
//!
//! `Num2Word_TK` does **not** define `to_cardinal_float`; its overridden
//! `to_cardinal` handles non-integers inline by working on `str(number)`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"): n = n[1:]; ret = self.negword
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     for digit in right: ret += self._int_to_word(int(digit)) + " "
//!     return ret.strip()
//! ```
//!
//! This reads the fractional **digits straight out of the decimal repr** rather
//! than reconstructing them from `base.float2tuple`'s binary arithmetic, and it
//! runs for *every* float/Decimal, whole values included: `str(5.0)` is
//! `"5.0"`, so `to_cardinal(5.0)` == "bäş otur nol", never the integer
//! path's bare "bäş" — the base's whole-value routing is therefore overridden
//! at [`Lang::cardinal_float_entry`]. `str(number)` is reconstructed exactly:
//!   * float: [`python_float_repr`] — CPython's shortest-round-trip repr,
//!     including the exponent form at `|v| >= 1e16` ("1e+16"), the ".0" tail
//!     of whole values, "-0.0"'s sign bit, and "inf"/"nan". Differentially
//!     tested against CPython on 300k doubles in the sibling ports
//!     (`lang_bm`, `lang_ki`): 0 mismatches.
//!   * Decimal: [`crate::strnum::python_decimal_str`] — the spec's
//!     to-scientific-string, trailing zeros preserved ("1.10"), capital-E
//!     exponent form for a positive exponent (`Decimal("1E+2")` → "1E+2").
//!
//! **Exponent-form reprs raise `ValueError`**: "1e+16"/"1E+2" contain no ".",
//! so TK's `int(n)` chokes — `invalid literal for int() with base 10: '1e+16'`
//! — exactly as the wholefloat corpus pins (`cardinal 1e+16` → ValueError,
//! `Decimal("1E+20")` → ValueError, string `"1e3"` → ValueError). The other
//! three modes follow `to_cardinal`: `to_ordinal(float)` is the cardinal plus
//! the harmonised ending ("bäş otur nolunjy"), `to_year(float)` is the cardinal, and both
//! propagate the ValueError; `to_ordinal_num(float)` is `str(number) + "."`
//! and never raises ("1e+16.").
//!
//! String input `"Infinity"`/`"-Infinity"` parses to `Decimal("Infinity")` in
//! Python (base `str_to_number`) and dies later in TK's `int("Infinity")` with
//! `ValueError`. The Rust dispatcher hard-codes `ParsedNumber::Inf` →
//! OverflowError before any language hook runs, so [`Lang::str_to_number`] is
//! overridden to surface the ValueError at parse time. Known divergence,
//! documented rather than fixable from this file: Python's
//! `to_ordinal_num(Decimal("Infinity"))` returns "Infinity." *without*
//! raising; with the early raise this port reports ValueError there too. No
//! corpus row exercises it — the Infinity rows are all `to="cardinal"`.
//!
//! 12. **`precision=` (issue #580) has no effect on TK floats.** TK.to_cardinal
//!     reads `str(number)` and never consults `self.precision`, so the kwarg —
//!     which only rebinds `converter.precision` — changes nothing. Verified:
//!     `num2words(2.675, lang='tk', precision=1)` still yields all three
//!     fractional digits. `precision_override` is therefore accepted and
//!     ignored in the hook below.
//!
//! # The currency surface
//!
//! `Num2Word_TK` overrides `to_currency` **outright** and inherits
//! `to_cheque`/`_money_verbose`/`_cents_*` from `Num2Word_Base` untouched (the
//! live interpreter confirms `to_cheque.__qualname__ == "Num2Word_Base.to_cheque"`).
//! The two halves therefore behave very differently, and the differences are
//! Python's, not this port's:
//!
//! 6. **`to_currency` never raises for an unknown code.** Python reaches for
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a default, not a subscript — so `GBP`/`JPY`/`KWD`/`BHD`/
//!    `INR`/`CNY`/`CHF` silently render as Turkmen manat/teňňe rather than
//!    raising NotImplementedError. `to_cheque`, inherited from the base, uses a
//!    plain `self.CURRENCY_FORMS[currency]` subscript and *does* raise for those
//!    same codes. The corpus pins both halves: `currency:JPY 12.34` →
//!    "on iki manat otuz dört teňňe", `cheque:JPY 1234.56` → NotImplementedError.
//!    See [`FALLBACK_CURRENCY`].
//! 7. **Cents come from string slicing, not arithmetic.** Python does
//!    `int(parts[1][:2].ljust(2, "0"))` on `str(val)`, so `0.5` → "5" → "50" →
//!    50 cents, and `1.005` → "005"[:2] → "00" → 0 cents (truncation, never
//!    rounding). Sliced by `chars()` per the porting contract.
//! 8. **`CURRENCY_PRECISION` is ignored by `to_currency`.** TK's override never
//!    consults it, and TK inherits the base's empty `{}` anyway, so the
//!    3-decimal (KWD/BHD) and 0-decimal (JPY) special-casing in
//!    `Num2Word_Base.to_currency` is unreachable here — all three fall back to
//!    TMT forms with a hard-coded 2-digit cent slice. `currency_precision` is
//!    left at the trait default (100), which is what the inherited `to_cheque`
//!    reads.
//! 9. **`adjective` is accepted and never read.** TK's signature declares it but
//!    the body has no `CURRENCY_ADJECTIVES` lookup; the class inherits the
//!    base's empty `{}` regardless.
//! 10. **`cents=False` drops the cents segment entirely** rather than switching
//!    to the terse `%02d` form: the guard is `if cents and right:`, with no
//!    `else`. So `to_currency(12.34, cents=False)` == "on iki euros", and
//!    `_cents_terse` is dead code for this class.
//! 11. **A float with zero cents prints no cents** — `1.0` → "bir euro", not
//!    "bir euro zero cents" — because `right` is 0 and `if cents and right:` is
//!    falsy. This lands in the same place as the true-int path (`1` → "bir
//!    euro"), but by different arithmetic, so the two are kept distinct below.
//!
//! `pluralize` is deliberately **not** overridden: it is abstract in
//! `Num2Word_Base` (raises NotImplementedError), and neither TK's `to_currency`
//! nor the inherited `to_cheque` ever calls it — TK picks its forms with an
//! inline `cr1[1] if left != 1 else cr1[0]`.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). No reliable Turkmen cent noun was found, so USD and EUR raise
//! NotImplementedError. Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.ones`. Index 0 is the empty string, exactly as in Python — the
/// `_int_to_word(0)` guard depends on its falsiness (see quirk 2).
const ONES: [&str; 10] = [
    "", "bir", "iki", "üç", "dört", "bäş", "alty", "ýedi", "sekiz", "dokuz",
];

/// `self.tens`. Index 0 is the empty string but is never reached: the
/// `number < 100` arm only runs for `number >= 10`, so `tens_val >= 1`.
const TENS: [&str; 10] = [
    "", "on", "ýigrimi", "otuz", "kyrk", "elli", "altmyş", "ýetmiş", "segsen", "togsan",
];

const HUNDRED: &str = "ýüz";
const THOUSAND: &str = "müň";
const MILLION: &str = "million";

/// `self.negword` — note the trailing space, which Python relies on for
/// "minus bir" and then trims off any dangling remainder with `.strip()`.
const NEGWORD: &str = "minus ";

/// Attach the ordinal ending to the **last word** of a spelled cardinal
/// (gladiaio/num2words2#148). Upstream glued a fixed "-nji" onto the whole
/// cardinal ("bir-nji", "on-nji"). Turkmen ordinals take *-(V)njy/-(V)nji*
/// (enedilim.com, "Sanlar"; Wiktionary's Turkmen number table):
///
/// * the suffix vowel follows backness — *-njy* after a back stem (a o u y),
///   *-nji* after a front one (ä e i ö ü): altynjy, ýedinji;
/// * after a consonant a linking vowel is inserted — *y*/*i*, rounded to
///   *u*/*ü* only in a **one-syllable** stem whose vowel is rounded: onunjy,
///   üçünji, ýüzünji, but dokuzynjy, otuzynjy, millionynjy;
/// * dört voices its final t: dördünji.
///
/// 123rd is therefore "bir ýüz ýigrimi üçünji". A cardinal that is not a word
/// at all — the digit fallback above 10^9 — keeps a hyphen ("1000000000-nji").
fn join_ordinal(cardinal: &str) -> String {
    let (head, stem) = match cardinal.rfind(' ') {
        Some(i) => cardinal.split_at(i + 1),
        None => ("", cardinal),
    };
    let vowels: Vec<char> = stem
        .chars()
        .filter(|c| "aouyäeiöü".contains(*c))
        .collect();
    let last = match vowels.last() {
        Some(v) => *v,
        None => return format!("{}-nji", cardinal),
    };
    let back = "aouy".contains(last);
    let ending = if back { "njy" } else { "nji" };
    if stem.ends_with(|c: char| "aouyäeiöü".contains(c)) {
        return format!("{}{}{}", head, stem, ending);
    }
    let rounded = vowels.len() == 1 && "ouöü".contains(last);
    let link = match (back, rounded) {
        (true, true) => 'u',
        (true, false) => 'y',
        (false, true) => 'ü',
        (false, false) => 'i',
    };
    let stem = if stem == "dört" { "dörd" } else { stem };
    format!("{}{}{}{}", head, stem, link, ending)
}

/// The ceiling of `_int_to_word`'s word-producing branches. At or above this,
/// Python falls through to `str(number)` (quirk 1).
const MILLIARD: u64 = 1_000_000_000;

/// Python's `_int_to_word` for `1 <= n < 10^9`.
///
/// Split out from [`int_to_word`] so the arithmetic can run on `u64`: every
/// caller has already proven the bound, and Python's `//` and `%` agree with
/// Rust's on non-negative operands, so there is no floor-division divergence
/// to worry about here.
///
/// The `remainder`/`div` recursions in Python route back through the full
/// `_int_to_word`, but each is guarded by `if remainder:` or is a quotient
/// `>= 1`, so the zero and negative arms are never re-entered — recursing into
/// this function instead is equivalent.
fn int_to_word_small(n: u64) -> String {
    debug_assert!(n >= 1 && n < MILLIARD);

    if n < 10 {
        // self.ones[number]
        return ONES[n as usize].to_string();
    }

    if n < 100 {
        let tens_val = (n / 10) as usize;
        let ones_val = (n % 10) as usize;
        if ones_val == 0 {
            return TENS[tens_val].to_string();
        }
        return format!("{} {}", TENS[tens_val], ONES[ones_val]);
    }

    if n < 1_000 {
        let hundreds_val = (n / 100) as usize;
        let remainder = n % 100;
        // No `hundreds_val > 1` guard in Python: 100 → "bir ýüz" (quirk 3).
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        return result;
    }

    if n < 1_000_000 {
        let thousands_val = n / 1_000;
        let remainder = n % 1_000;
        let mut result = format!("{} {}", int_to_word_small(thousands_val), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        return result;
    }

    // n < 1_000_000_000
    let millions_val = n / 1_000_000;
    let remainder = n % 1_000_000;
    let mut result = format!("{} {}", int_to_word_small(millions_val), MILLION);
    if remainder != 0 {
        result.push(' ');
        result.push_str(&int_to_word_small(remainder));
    }
    result
}

/// Python's `_int_to_word`, entry point.
///
/// Mirrors the Python arm-for-arm, including the two arms that are dead on the
/// integer path but kept for fidelity:
///   * `number == 0` → "zero" (quirk 2) — live, reached by `to_cardinal(0)`.
///   * `number < 0` → `negword + _int_to_word(abs(number))` — **unreachable**
///     from any in-scope caller, since `to_cardinal` strips the sign textually
///     before `int()` and every recursion passes a non-negative quotient or a
///     non-zero remainder. Reproduced anyway so the shape matches the source.
///   * the `else` fallback → `str(number)` for `number >= 10^9` (quirk 1).
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        // `self.ones[0] if self.ones[0] else "zero"` — ONES[0] is "", falsy.
        return "nol".to_string();
    }

    if n.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&n.abs()));
    }

    // Scale words above a million (gladiaio/num2words2#147), composed like
    // the million arm. Every entry point rejects values at or above
    // `maxval_ceiling()` first, so the top quotient is always below 1000.
    for &(exp, word) in SCALES.iter().rev() {
        let scale = pow10_big(exp);
        if n >= &scale {
            let mut result = format!("{} {}", int_to_word(&(n / &scale)), word);
            let rest = n % &scale;
            if !rest.is_zero() {
                result.push(' ');
                result.push_str(&int_to_word(&rest));
            }
            return result;
        }
    }

    // Below 10^9 the value fits a u64 comfortably; at or above it, Python
    // stops producing words and just prints the digits, so BigInt values of
    // any size (the corpus goes to 10^21) land in the fallback unharmed.
    match n.to_u64() {
        Some(v) if v < MILLIARD => int_to_word_small(v),
        _ => n.to_string(),
    }
}

/// CPython's `repr(float)` / `str(float)`, which is what TK's `to_cardinal`
/// runs on: `n = str(number).strip()`. TK's float branch is a *string*
/// algorithm, so the exact repr — not the numeric value — drives the output
/// (and the ValueErrors).
///
/// # 1. The digits
///
/// `{:e}` is Rust's shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the
/// significant digits and the decimal-point position fall straight out. A rare
/// tie can leave `{:e}`'s final digit one off the value CPython's dtoa would
/// pick; re-formatting with `{:.*}` at the known digit count repairs it. This
/// exact function is differentially tested against CPython on 300k doubles in
/// the sibling ports (`lang_bm`, `lang_ki`): 0 mismatches with the repair.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`
/// (`format_float_short`, format code `'r'`), pads the exponent to two digits,
/// and appends `.0` to anything that would otherwise look like an integer.
/// Rust's `{}` does none of this, so both `1e16` and `1.0` would come out
/// wrong in opposite directions. Both matter to TK: `str(1.0)` is `"1.0"` →
/// "bir otur nol", and `str(1e16)` is `"1e+16"` → `int("1e+16")` raises
/// `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this: for an exponent-form repr it is the *exponent*
/// (`abs(Decimal(str(v)).as_tuple().exponent)`), not a digit count — `1e16`
/// arrives with `precision == 16`, and `format!("{:.16}", 1e16)` would emit
/// sixteen spurious fractional zeros where Python raises.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). TK feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and TK renders that
    // "minus nol otur nol".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `decpt` is CPython's: the value is `0.<digits> * 10**decpt`.
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').expect("LowerExp always emits an 'e'");
    let exp: i32 = exp.parse().expect("LowerExp emits an integer exponent");
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut decpt = exp + 1;

    // Tie repair — see the doc comment. Only reachable when the shortest form
    // has fractional digits; `a == 0.0` is excluded because `{:e}` reports it
    // as "0e0" and there is nothing to round.
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

/// `str(number)` for whatever the Python dispatcher handed the converter. The
/// `FloatValue` split is exactly Python's `isinstance(value, Decimal)`: the
/// two arms stringify by different rules and must not be collapsed.
/// `str(Decimal)` is the spec algorithm in [`crate::strnum::python_decimal_str`]
/// — trailing zeros preserved (`Decimal("1.10")` → "1.10"), capital-E
/// exponent form for a positive exponent (`Decimal("1E+2")` → "1E+2", which
/// TK's `int()` then rejects with ValueError).
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// Python's `Num2Word_TK.to_cardinal` for the string `n = str(number).strip()`.
///
/// Faithfully reproduces the sign-strip, `split(".", 1)`, `int()`-per-field
/// logic. `int()` on a non-numeric field raises `ValueError`, mapped to
/// [`N2WError::Value`] — live for every exponent-form repr the reconstruction
/// produces ("1e+16", "1E+2", "inf", "nan"): `int(left)` runs first and quotes
/// the whole literal, while a bad *fraction* character quotes the single char
/// ('E'), exactly as Python's per-digit `int()` does.
fn tk_cardinal_from_str(n: &str, negword: &str, pointword: &str) -> Result<String> {
    // Python: `str(number).strip()`.
    let n = n.trim();
    // Python: `if n.startswith("-"): n = n[1:]; ret = self.negword`.
    let (n, mut ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, negword.to_string()),
        None => (n, String::new()),
    };

    // `self._int_to_word(int(field))`, raising ValueError where int() would.
    let int_word = |field: &str| -> Result<String> {
        let val = BigInt::from_str(field).map_err(|_| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                field
            ))
        })?;
        Ok(checked_int_to_word(&val)?)
    };

    // Python: `if "." in n:` — split on the *first* dot only (`split(".", 1)`).
    match n.find('.') {
        Some(dot) => {
            let left = &n[..dot];
            let right = &n[dot + 1..];
            ret.push_str(&int_word(left)?);
            ret.push(' ');
            ret.push_str(pointword);
            ret.push(' ');
            // Python: `for digit in right: ret += self._int_to_word(int(digit))`
            // — one character at a time (indexed by chars per the contract).
            for ch in right.chars() {
                ret.push_str(&int_word(&ch.to_string())?);
                ret.push(' ');
            }
            Ok(ret.trim().to_string())
        }
        None => {
            ret.push_str(&int_word(n)?);
            Ok(ret.trim().to_string())
        }
    }
}

/// The code whose forms `to_currency` falls back to for an unknown currency.
///
/// Python: `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
/// `list(values())[0]` is the **first-inserted** entry. The class body lists
/// `TMT` before `USD` and `EUR`, and dicts have preserved insertion order since
/// CPython 3.7 — the live interpreter confirms
/// `list(CURRENCY_FORMS.keys()) == ['TMT', 'USD', 'EUR']`. A `HashMap` has no
/// order of its own, so the choice is pinned to a constant here rather than
/// left to iteration order. Quirk 6.
const FALLBACK_CURRENCY: &str = "TMT";

/// The separator the FFI bridge sends when the caller passed none.
///
/// `Num2Word_TK.to_currency` defaults `separator=" "`, but the bridge cannot
/// express "caller omitted it": `num2words2/__init__.py` sends
/// `kwargs.get("separator", ",")` and `bench/diff_test.py` hard-codes `","`, so
/// `Num2Word_Base`'s default arrives literally in both cases. The corpus was
/// generated through the pure-Python path with the separator omitted, so every
/// expected string uses `" "` — 54 of the 108 currency rows depend on mapping
/// this back. Right for the default call and for every caller who passes
/// anything other than `","`; wrong only for an explicit `separator=","`, which
/// Python renders `"on iki euros,otuz dört cents"` and this renders with a
/// space. That case is indistinguishable from the default at this boundary.
/// Fixing it properly means teaching the binding each language's own default;
/// it cannot be fixed from this file. ~100 of the 156 Python modules override
/// this default, so the issue is systemic rather than TK-specific, and
/// `lang_haw.rs` resolves it the same way. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// TK's own `to_currency` default, restored when [`SEPARATOR_UNSET`] arrives.
const SEPARATOR_DEFAULT: &str = " ";

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "milliard"), (12, "trillion")];

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

pub struct LangTk {
    /// `CURRENCY_FORMS`, built once. Every entry carries exactly two unit forms
    /// and two subunit forms, matching Python's tuple arity — `to_currency`
    /// indexes `[0]`/`[1]` directly and the inherited `to_cheque` takes
    /// `cr1[-1]`, so the arity is load-bearing.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl LangTk {
    pub fn new() -> Self {
        // Built once and cached by the caller (`num2words2-py` holds this in a
        // OnceLock), never per call.
        let mut forms = HashMap::with_capacity(3);
        // Insertion order is irrelevant to a HashMap; FALLBACK_CURRENCY pins
        // what Python's `list(values())[0]` resolves to. Listed in the class
        // body's order regardless, so the two read the same.
        forms.insert(
            "TMT",
            CurrencyForms::new(&["manat", "manat"], &["teňňe", "teňňe"]),
        );
        // USD/EUR were English ("dollars", "cents", "euros"); no reliable Turkmen cent noun was found, so
        // they raise NotImplementedError (#222).
        LangTk { forms }
    }
}

impl Default for LangTk {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangTk {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "TMT"
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
        "otur"
    }

    /// Python's `to_cardinal`.
    ///
    /// The Python works on `str(number)`: it strips a leading "-" off the
    /// *text*, then `int()`s what remains. For integer input that is exactly
    /// "take the absolute value and remember the sign", with no "." branch to
    /// consider (the float path is out of scope), so this is a faithful
    /// rendering rather than a shortcut.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // Python: `(ret + self._int_to_word(int(n))).strip()`. The strip only
        // matters if the word part were empty; it never is, but `trim()`
        // matches Python's `str.strip()` (both strip Unicode whitespace) and
        // the word tables contain no leading/trailing spaces.
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// Python's `to_ordinal`: the cardinal with the harmonised ending on its
    /// last word (quirk 4, fixed — see [`join_ordinal`]).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(join_ordinal(&self.to_cardinal(value)?))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."` — note this overrides
    /// base.py's `return value`, so the period is TK's own.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)`: `return self.to_cardinal(val)`.
    /// The `longval` flag is accepted and ignored. Stated explicitly rather
    /// than left to the trait default, because TK's override deliberately
    /// discards base.py's year-specific formatting.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_cardinal(float/Decimal)` — the **full** routing, whole values
    /// included. TK's `to_cardinal` reads `str(number)`, so a whole-valued
    /// float keeps its ".0" tail ("bäş otur nol") and an exponent-form repr
    /// raises ValueError; the base default's whole → integer-path route would
    /// get both wrong. See the module docs' float section.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// Python's `Num2Word_TK.to_cardinal` for **non-integer** input.
    ///
    /// TK has no `to_cardinal_float`; its `to_cardinal` handles floats and
    /// Decimals inline over `str(number)` (see the module note). This hook
    /// reconstructs that string exactly — [`python_float_repr`] /
    /// [`crate::strnum::python_decimal_str`] — and runs the identical
    /// algorithm, so exponent-form reprs die in the same `int()` as Python's.
    ///
    /// `precision_override` (#580) is accepted and ignored: TK reads the repr
    /// directly and never consults `self.precision`, so the kwarg has no effect
    /// (quirk 12, verified against the live interpreter).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        tk_cardinal_from_str(&python_str(value), self.negword(), self.pointword())
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number)` + the ordinal ending with no type check, so
    /// floats get the full decimal grammar plus the ending and the
    /// cardinal's ValueError on exponent-form reprs propagates before the
    /// suffix is appended (`to_ordinal(1e16)` → ValueError).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(join_ordinal(&self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, same as the
    /// integer overload — floats are *not* an error here, so
    /// `to_ordinal_num(1e16)` == "1e+16." while `to_ordinal(1e16)` raises.
    /// `repr_str` is Python's own `str(number)`, supplied by the binding.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is a bare `to_cardinal` alias, so
    /// floats route through the same string algorithm (and raise the same
    /// ValueErrors). Identical to the trait default — which also calls
    /// `cardinal_float_entry` — but spelled out because Python spells it out.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — base's `Decimal(value)`, with one
    /// deliberate deviation: `Decimal("Infinity")` parses fine in Python and
    /// the ValueError only fires later, inside TK's `int("Infinity")`. The
    /// Rust dispatcher hard-codes `ParsedNumber::Inf` → OverflowError (the
    /// base-class `int(Decimal('Infinity'))` behaviour) before any language
    /// hook runs, so the ValueError is surfaced here at parse time instead —
    /// same observable type for every mode that reaches `to_cardinal`.
    /// Known divergence (`to_ordinal_num("Infinity")`) documented in the
    /// module docs. NaN is left alone: the dispatcher already reports
    /// ValueError for it, matching Python's `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                // str(Decimal("Infinity")) is "Infinity"; the sign is sliced
                // off (n[1:]) before int() sees it, so both signs quote the
                // same literal.
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for the NotImplementedError message the
    /// inherited `to_cheque` raises.
    fn lang_name(&self) -> &str {
        "Num2Word_TK"
    }

    /// `CURRENCY_FORMS[code]` — the *subscript* form.
    ///
    /// Only the inherited `to_cheque` reads this hook, and it must return
    /// `None` for a missing code so the NotImplementedError fires. TK's own
    /// `to_currency` deliberately does **not** route through here: it uses
    /// `.get(code, <first entry>)` and never raises (quirk 6).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Python's `Num2Word_TK.to_currency`.
    ///
    /// Note this reimplements the whole method rather than delegating to
    /// `currency::default_to_currency`: TK's override shares almost nothing
    /// with the base — no `parse_currency_parts`, no `pluralize`, no
    /// `CURRENCY_PRECISION`, no `_money_verbose`, and no NotImplementedError.
    /// It calls `self._int_to_word` directly (not `self.to_cardinal`), which
    /// for the non-negative values reaching it is equivalent, but is
    /// reproduced literally here.
    ///
    /// # Errors
    ///
    /// `N2WError::Value` where Python's `int()` raises `ValueError` on a
    /// non-numeric field — reachable only for a float whose `repr` is in
    /// scientific notation (see the concern in the port report).
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

        // Restore TK's own separator default. See [`SEPARATOR_UNSET`].
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Python: `if val < 0: is_negative = True; val = abs(val)` — the abs()
        // happens *before* str(), so the sign never reaches the split.
        let is_negative = val.is_negative();
        let s = match val {
            // `str(int)` — no ".", so `parts` has length 1 and right stays 0.
            CurrencyValue::Int(i) => if is_negative { i.abs() } else { i.clone() }.to_string(),
            // `str(float)`. The Python shim already stringified the float with
            // repr() and the core parsed that, so `BigDecimal::to_string` here
            // reproduces `str(val)` exactly for every ordinary decimal repr —
            // crucially preserving the trailing ".0" of `1.0`, which is what
            // makes quirk 11 work out.
            CurrencyValue::Decimal { value: d, .. } => if is_negative { d.abs() } else { d.clone() }.to_string(),
        };

        // Python: `parts = str(val).split(".")`. Splitting on every "." (not
        // just the first) so `parts[1]` is the field *between* the first and
        // second dot, exactly as Python's list indexing would see it.
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

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — TMT on a miss,
        // never an error. Quirk 6.
        let forms = self.forms.get(currency).unwrap_or_else(|| {
            self.forms
                .get(FALLBACK_CURRENCY)
                .expect("new() always inserts TMT")
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
        // cents (1.0) prints no cents segment (quirk 11), and `cents=False`
        // drops the segment outright rather than going terse (quirk 10).
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

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;

    fn card_float(v: f64, precision: u32) -> String {
        // `precision` is what the binding passes: Python's
        // `abs(Decimal(repr(v)).as_tuple().exponent)`. Rust's `{}` would render
        // `1.0` as "1" (losing the ".0"), so it cannot be derived here — it is
        // supplied per case exactly as the harness computes it.
        LangTk::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn card_dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.max(0) as u32;
        LangTk::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    #[test]
    fn corpus_floats() {
        // Every "cardinal" row with a float arg from bench/corpus.jsonl, with
        // the precision the binding derives from Python's repr.
        let cases: &[(f64, u32, &str)] = &[
            (0.0, 1, "nol otur nol"),
            (0.5, 1, "nol otur bäş"),
            (1.0, 1, "bir otur nol"),
            (1.5, 1, "bir otur bäş"),
            (2.25, 2, "iki otur iki bäş"),
            (3.14, 2, "üç otur bir dört"),
            (0.01, 2, "nol otur nol bir"),
            (0.1, 1, "nol otur bir"),
            (0.99, 2, "nol otur dokuz dokuz"),
            (1.01, 2, "bir otur nol bir"),
            (12.34, 2, "on iki otur üç dört"),
            (99.99, 2, "togsan dokuz otur dokuz dokuz"),
            (100.5, 1, "bir ýüz otur bäş"),
            (1234.56, 2, "bir müň iki ýüz otuz dört otur bäş alty"),
            (-0.5, 1, "minus nol otur bäş"),
            (-1.5, 1, "minus bir otur bäş"),
            (-12.34, 2, "minus on iki otur üç dört"),
            (1.005, 3, "bir otur nol nol bäş"),
            (2.675, 3, "iki otur alty ýedi bäş"),
        ];
        for (v, p, want) in cases {
            assert_eq!(&card_float(*v, *p), want, "float {}", v);
        }
    }

    #[test]
    fn corpus_decimals() {
        // Every "cardinal_dec" row from bench/corpus.jsonl.
        let cases: &[(&str, &str)] = &[
            ("0.01", "nol otur nol bir"),
            ("1.10", "bir otur bir nol"),
            ("12.345", "on iki otur üç dört bäş"),
            ("98746251323029.99", "togsan sekiz trillion ýedi ýüz kyrk alty milliard iki ýüz elli bir million üç ýüz ýigrimi üç müň ýigrimi dokuz otur dokuz dokuz"),
            ("0.001", "nol otur nol nol bir"),
        ];
        for (s, want) in cases {
            assert_eq!(&card_dec(s), want, "decimal {}", s);
        }
    }

    #[test]
    fn precision_override_ignored() {
        // TK reads str(number); #580's precision= never changes the digits.
        let got = LangTk::new()
            .to_cardinal_float(
                &FloatValue::Float { value: 2.675, precision: 3 },
                Some(1),
            )
            .unwrap();
        assert_eq!(got, "iki otur alty ýedi bäş");
    }

    fn fv_f(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn fv_d(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// The wholefloat-corpus routing rows: every float/Decimal — whole values
    /// included — takes the string algorithm, so ".0" tails are spoken and
    /// exponent-form reprs raise ValueError from `int()`.
    #[test]
    fn corpus_float_entry_rows() {
        let l = LangTk::new();
        // Whole floats keep their ".0" (str(5.0) == "5.0").
        assert_eq!(
            l.cardinal_float_entry(&fv_f(5.0, 1), None).unwrap(),
            "bäş otur nol"
        );
        // str(-0.0) == "-0.0": the sign bit alone earns the negword.
        assert_eq!(
            l.cardinal_float_entry(&fv_f(-0.0, 1), None).unwrap(),
            "minus nol otur nol"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv_f(1234.0, 1), None).unwrap(),
            "bir müň iki ýüz otuz dört otur nol"
        );
        // Above 10^9 the integer field degrades to bare digits (quirk 1).
        assert_eq!(
            l.cardinal_float_entry(&fv_f(1e9, 1), None).unwrap(),
            "bir milliard otur nol"
        );
        // Decimal without a visible point takes the integer words...
        assert_eq!(l.cardinal_float_entry(&fv_d("5", 0), None).unwrap(), "bäş");
        // ...while trailing zeros survive str(Decimal).
        assert_eq!(
            l.cardinal_float_entry(&fv_d("5.00", 2), None).unwrap(),
            "bäş otur nol nol"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv_d("12345.000", 3), None).unwrap(),
            "on iki müň üç ýüz kyrk bäş otur nol nol nol"
        );
        // Exponent-form float reprs: str(1e16) == "1e+16" — no ".", so
        // int() raises ValueError (the dispatcher never sends these here,
        // #211). str(Decimal) is written out ("100"), so Decimals read.
        for v in [
            l.cardinal_float_entry(&fv_f(1e16, 16), None),
            l.cardinal_float_entry(&fv_f(1e20, 20), None),
        ] {
            assert!(matches!(v, Err(N2WError::Value(_))), "{v:?}");
        }
        assert_eq!(
            l.cardinal_float_entry(&fv_d("1E+2", 0), None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );
    }

    /// `to_ordinal` on floats: cardinal + ending, ValueErrors propagating;
    /// `to_ordinal_num` is repr + "." and never raises; `to_year` follows the
    /// cardinal.
    #[test]
    fn corpus_ordinal_entry_rows() {
        let l = LangTk::new();
        assert_eq!(
            l.ordinal_float_entry(&fv_f(1.0, 1)).unwrap(),
            "bir otur nolunjy"
        );
        assert_eq!(
            l.ordinal_float_entry(&fv_f(-0.0, 1)).unwrap(),
            "minus nol otur nolunjy"
        );
        assert_eq!(l.ordinal_float_entry(&fv_d("0", 0)).unwrap(), "nolunjy");
        assert_eq!(l.ordinal_float_entry(&fv_d("5", 0)).unwrap(), "bäşinji");
        assert_eq!(
            l.ordinal_float_entry(&fv_d("100", 0)).unwrap(),
            "bir ýüzünji"
        );
        assert_eq!(
            l.ordinal_float_entry(&fv_f(3.25, 2)).unwrap(),
            "üç otur iki bäşinji"
        );
        assert!(matches!(
            l.ordinal_float_entry(&fv_f(1e16, 16)),
            Err(N2WError::Value(_))
        ));
        assert_eq!(
            l.ordinal_num_float_entry(&fv_f(1e16, 16), "1e+16").unwrap(),
            "1e+16."
        );
        assert_eq!(
            l.ordinal_num_float_entry(&fv_d("5.00", 2), "5.00").unwrap(),
            "5.00."
        );
        assert_eq!(
            l.year_float_entry(&fv_f(5.0, 1)).unwrap(),
            "bäş otur nol"
        );
        assert_eq!(
            l.year_float_entry(&fv_d("1E+2", 0)).unwrap(),
            l.to_year(&BigInt::from(100)).unwrap()
        );
    }

    /// String inputs: "1e3" parses to Decimal('1E+3'), read as 1000 (#211); "Infinity" is intercepted in str_to_number
    /// (the dispatcher would otherwise report the base class's OverflowError).
    #[test]
    fn corpus_string_rows() {
        let l = LangTk::new();
        let parsed = l.str_to_number("1e3").unwrap();
        match parsed {
            ParsedNumber::Dec(d) => {
                // #211: str(Decimal('1E+3')) is written out as "1000".
                let fv = FloatValue::Decimal { value: d, precision: 0 };
                assert_eq!(
                    l.cardinal_float_entry(&fv, None).unwrap(),
                    l.to_cardinal(&BigInt::from(1000)).unwrap()
                );
            }
            other => panic!("expected Dec, got {other:?}"),
        }
        assert!(matches!(
            l.str_to_number("Infinity"),
            Err(N2WError::Value(_))
        ));
        assert!(matches!(
            l.str_to_number("-Infinity"),
            Err(N2WError::Value(_))
        ));
        // NaN stays a ParsedNumber::NaN — the dispatcher's ValueError already
        // matches Python's int("NaN") type.
        assert!(matches!(l.str_to_number("NaN"), Ok(ParsedNumber::NaN)));
    }

    /// [`python_float_repr`] against CPython ground truth for the corpus range.
    #[test]
    fn float_repr_matches_cpython() {
        for (v, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.0, "1.0"),
            (0.5, "0.5"),
            (1234.0, "1234.0"),
            (1000000000000000.0, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1e20, "1e+20"),
            (2.675, "2.675"),
            (1.005, "1.005"),
            (123456789.5, "123456789.5"),
        ] {
            assert_eq!(python_float_repr(v), want);
        }
    }
}
