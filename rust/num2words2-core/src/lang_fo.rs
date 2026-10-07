//! Port of `lang_FO.py` (Faroese).
//!
//! Shape: **self-contained**. `Num2Word_FO` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` guard in `Num2Word_Base.__init__` never fires: Python builds no
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives a plain recursive `_int_to_word`. Consequently
//! `cards`/`merge` stay at their trait defaults here. Python has **no
//! overflow check** (see bug 3 below for what it did past 10^9); this port
//! raises `OverflowError` from 10^24 (`maxval`).
//!
//! All four in-scope modes are overridden by Python, so nothing here falls
//! through to a `Num2Word_Base` default:
//!   * `to_cardinal(number)`  → strips the sign off `str(number)`, recurses.
//!   * `to_ordinal(number)`   → `to_cardinal(number) + "-ti"`, unconditionally.
//!   * `to_ordinal_num(number)` → `str(number) + "."`.
//!   * `to_year(val, longval=True)` → `to_cardinal(val)`; `longval` is accepted
//!     and then ignored, so years get no century split and no era suffix.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is wrong — as Faroese and
//! sometimes as arithmetic — but is exactly what Python emits, verified
//! against the frozen corpus:
//!
//! 1. **There are no teens.** `self.tens[1]` is "tíggju" (= 10) and 11..=19 are
//!    built by the generic `tens[n/10] + " " + ones[n%10]` rule, so
//!    `to_cardinal(11)` == "tíggju ein" — literally "ten one". Real Faroese is
//!    "ellivu". Likewise 12 → "tíggju tvey" (not "tólv"), 13 → "tíggju trý"
//!    (not "trettan"), ... 19 → "tíggju níggju" (not "nítjan"). This leaks
//!    upward: `to_cardinal(12345)` == "tíggju tvey túsund trý hundrað fjøruti
//!    fimm", i.e. the thousands part reads "ten two thousand".
//! 2. **No copula anywhere.** Faroese joins the last two elements with "og"
//!    ("hundrað og ein"); this module joins every fragment with a bare space,
//!    so 101 → "ein hundrað ein". Numerals are also never inflected for gender
//!    ("tvey"/"trý" are the neuter forms, used for everything).
//! 3. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` chain stops at `million`, and the final `else` is
//!    `return str(number)`, so `to_cardinal(10**9)` was the digit string
//!    "1000000000". This port continues the same `N <scale>` composition with
//!    the Faroese long-scale words milliard (10^9), billión (10^12), billiard
//!    (10^15), trillión (10^18) and trilliard (10^21) — fo.wikipedia "Langi
//!    og stutti talstigin fyri stór tøl", Omniglot "Numbers in Faroese" — and
//!    raises `OverflowError` from 10^24 (`maxval`).
//! 4. **`to_ordinal` glues "-ti" onto whatever the cardinal returned**
//!    without inspecting it. Python's 10^9 fallback made that
//!    "1000000000-ti"; since #147 it is "ein milliard-ti".
//! 5. **`to_ordinal` accepts negatives.** Python never calls
//!    `Num2Word_Base.verify_ordinal`, which would have raised `TypeError` for
//!    negative input. So `to_ordinal(-1)` == "minus ein-ti" rather than
//!    raising, and `to_ordinal_num(-1)` == "-1." (`str(-1) + "."`).
//! 6. **"-ti" is not an ordinal suffix**, it is the tens suffix ("fimmti" =
//!    50). Every ordinal is formed this way regardless of the stem, so
//!    `to_ordinal(1)` == "ein-ti" where Faroese wants "fyrsti". The hyphen is
//!    literal and always present.
//! 7. **Zero and the decimal word (fixed, gladiaio/num2words2#154).**
//!    Python's `_int_to_word(0)` reads
//!    `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""` — falsy —
//!    so Python's 0 is always the English "zero", and its `pointword` is the
//!    English "point". This port uses the Faroese "null" and "komma" (Faroese
//!    writes a decimal comma): 0 → "null", 1.5 → "ein komma fimm".
//!
//! # Error variants
//!
//! The four *integer* modes raise only `OverflowError`, at `maxval` 10^24
//! (#147): every table index is bounded by its enclosing branch, and
//! negatives are handled rather than rejected.
//!
//! The **float/Decimal** path *can* raise, and only one way: `to_cardinal`
//! re-parses the sign-stripped `str(number)` with `int()`, so any point-less
//! form that is not an integer literal — `repr(1e16)` == "1e+16",
//! `str(Decimal("1E+2"))` == "1E+2", "inf"/"nan"/"Infinity" — raises
//! `ValueError: invalid literal for int() with base 10: '...'`. Values whose
//! string *does* show a point raise only when their integer part reaches
//! `maxval` (bug 3, #147). Routing is pinned by the
//! wholefloat corpus: `13x ValueError` rows for 1e+16/1e+20/1E+2/1E+20 across
//! cardinal/ordinal/year, and `ok` rows for every pointed form including
//! "-0.0" -> "minus null komma null" (see [`route_by_str`]).
//!
//! `to_currency` raises only the same `maxval` OverflowError — see bug 8
//! below, it has no `NotImplementedError` path at all. The **only** raising surface in this
//! module is `to_cheque`, which `Num2Word_FO` does *not* override: the
//! inherited `Num2Word_Base.to_cheque` subscripts `CURRENCY_FORMS[currency]`
//! and converts the `KeyError` into `NotImplementedError`. So `cheque:GBP`
//! raises while `currency:GBP` happily prints krónur.
//!
//! # Currency surface
//!
//! `Num2Word_FO` overrides `to_currency` **wholesale** — it never calls
//! `Num2Word_Base.to_currency`, `parse_currency_parts`, `pluralize`,
//! `_cents_verbose` or `_cents_terse`. Consequently:
//!
//! * `pluralize` is never reached and stays at its (raising) trait default,
//!   exactly as `Num2Word_FO` leaves `Num2Word_Base.pluralize` abstract.
//! * `CURRENCY_PRECISION` is `{}` (inherited from `Num2Word_Base`), so
//!   `.get(code, 100)` is always 100 — `currency_precision` stays at the trait
//!   default. FO's `to_currency` does not consult it anyway (bug 9).
//! * `CURRENCY_ADJECTIVES` is `{}` and `adjective` is accepted-then-ignored
//!   (bug 10), so `currency_adjective` stays at its `None` default.
//! * `money_verbose` stays at its default (`self.to_cardinal`), which is what
//!   `to_cheque` needs — `Num2Word_FO` does not override `_money_verbose`.
//!
//! ## Further faithfully reproduced Python bugs
//!
//! 8. **An unknown currency code silently becomes DKK.** FO's `to_currency`
//!    reads `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a *default*, not a subscript. `CURRENCY_FORMS` is
//!    declared `{"DKK": ..., "USD": ..., "EUR": ...}` and dicts preserve
//!    insertion order, so `values()[0]` is DKK's pair. Every unrecognised code
//!    is therefore rendered in Faroese krónur under its own name:
//!    `to_currency(1.0, "GBP")` == "ein króna", `to_currency(12.34, "JPY")` ==
//!    "tíggju tvey krónur tríati fýra oyru". The corpus pins GBP/JPY/KWD/BHD/
//!    INR/CNY/CHF to exactly that. Modelled by [`FALLBACK_CURRENCY`].
//! 9. **`CURRENCY_PRECISION` is ignored, so every currency splits at 2
//!    decimals.** The cents are cut out of the *digit string* with
//!    `parts[1][:2]`, a hard-coded 2. 3-decimal codes (KWD/BHD) and 0-decimal
//!    ones (JPY) get cent-style treatment like everything else — and since
//!    bug 8 has already rewritten them to DKK, `currency:JPY` for 12.34 is
//!    "tíggju tvey krónur tríati fýra oyru" rather than the ¥12 a JPY-aware
//!    path would give.
//! 10. **`adjective` is dead.** The parameter is declared and never read, so
//!    `adjective=True` changes nothing (there are no `CURRENCY_ADJECTIVES` to
//!    apply either).
//! 11. **Cents are dropped whenever they round to zero**, `int` or `float`
//!    alike. The guard is `if cents and right:` — a plain truthiness test on
//!    the cent count, *not* `Num2Word_Base`'s `isinstance(val, int)` check. So
//!    FO is one of the rare languages where `1` and `1.0` agree: both are "ein
//!    euro", never "ein euro null cents". `0.001` likewise truncates to zero
//!    cents and prints "null euros".
//! 12. **The cent digits are truncated, never rounded**, and are read
//!    left-to-right off the string: `12.999` → `"999"[:2]` → 99 cents, and
//!    `0.5` → `"5".ljust(2, "0")` → "50" → 50 cents. So a trailing digit is
//!    dropped rather than rounded up (`.999` does not carry to the next unit).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dollari/dollarar and evra/evrur with sent. The
//! numeral does not yet agree with the feminine "evra" ("tvey evrur" for "tvær
//! evrur"). Examples in these docs that quote English nouns record Python's
//! output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The scales above `million` that Python lacks (gladiaio/num2words2#147),
/// largest first, as `(power of ten, word)`: the Faroese long scale per
/// fo.wikipedia "Langi og stutti talstigin fyri stór tøl" and Omniglot.
/// Composed like `million` — multiplier, space, uninflected word.
const HIGH_SCALES: [(u32, &str); 5] = [
    (21, "trilliard"),
    (18, "trillión"),
    (15, "billiard"),
    (12, "billión"),
    (9, "milliard"),
];

/// The exclusive ceiling: 10^24 (kvadrillión) is past the scale table (#147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(24))
}

/// `self.negword`. Note the **trailing space**, which is part of the literal
/// in Python (`"minus "`) and differs from the `Num2Word_Base` convention.
/// `to_cardinal` concatenates it raw and lets the final `.strip()` tidy up.
const NEGWORD: &str = "minus ";

/// `self.pointword`. English "point" in Python; the Faroese "komma" here
/// (bug 7, #154).
const POINTWORD: &str = "komma";

/// What `_int_to_word(0)` returns: English "zero" in Python, Faroese "null"
/// here (bug 7, #154).
const ZERO_WORD: &str = "null";

/// `self.ones`. Index 0 is the empty string — see bug 7.
const ONES: [&str; 10] = [
    "", "ein", "tvey", "trý", "fýra", "fimm", "seks", "sjey", "átta", "níggju",
];

/// `self.tens`. Index 1 is "tíggju" (10) and there is no teens table — see
/// bug 1.
const TENS: [&str; 10] = [
    "", "tíggju", "tjúgu", "tríati", "fjøruti", "fimmti", "seksti", "sjeyti", "áttati", "níti",
];

const HUNDRED: &str = "hundrað";
const THOUSAND: &str = "túsund";
const MILLION: &str = "millión";

/// The Python class name, for `Num2Word_Base.to_cheque`'s NotImplementedError
/// message (`self.__class__.__name__`).
const LANG_NAME: &str = "Num2Word_FO";

/// `Num2Word_FO.to_currency`'s own default `separator=" "` — a bare space.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_FO.to_currency` declares `separator=" "`, but the `Lang` trait
/// carries no per-language parameter defaults, and both `__init__.py`'s
/// currency fast path and `bench/diff_test.py` substitute
/// `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default, not FO's —
/// before the value crosses the boundary. By then "caller omitted separator"
/// and "caller explicitly passed a comma" are the same `&str`, and the
/// information needed to tell them apart no longer exists on this side.
///
/// So `,` is read back as the unset sentinel and FO's own default restored.
/// This is the only reading the oracle supports: every float row of the `fo`
/// currency corpus was generated by `num2words(v, lang="fo", to="currency",
/// currency=c)` with no `separator=`, and all 54 of them expect a bare space
/// ("tíggju tvey euros tríati fýra cents", not "...euros,tríati fýra cents").
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " " where Python would give ",". Expressing that case
/// needs `Option<&str>` in the trait signature, which lives in `base.rs` —
/// outside this port's remit. Flagged in the port report. `lang_bo.rs`,
/// `lang_ca.rs`, `lang_es.rs` and `lang_eu.rs` resolve the identical conflict
/// the same way.
const SEPARATOR_UNSET: &str = ",";

/// `to_currency`'s fallback code — see bug 8.
///
/// Python spells it `list(self.CURRENCY_FORMS.values())[0]`, i.e. "whichever
/// entry was inserted first". `CURRENCY_FORMS` is a dict literal ordered
/// `DKK, USD, EUR` and dicts preserve insertion order, so that is DKK. Pinning
/// the *key* rather than reproducing a positional lookup keeps the intent
/// legible; a `HashMap` has no insertion order to index into anyway.
///
/// Verified against the live interpreter rather than read off the source:
/// `list(CONVERTER_CLASSES["fo"].CURRENCY_FORMS.keys())` == `['DKK', 'USD',
/// 'EUR']`. Nothing mutates this dict — `Num2Word_FO` declares its own
/// `CURRENCY_FORMS` and inherits from `Num2Word_Base`, not `Num2Word_EUR`, so
/// the `Num2Word_EN.__init__` shared-dict rewrite documented in
/// PORTING_CURRENCY.md does not reach it.
const FALLBACK_CURRENCY: &str = "DKK";

/// The hard-coded 2 in `parts[1][:2].ljust(2, "0")` — see bug 9.
const CENT_DIVISOR: i64 = 100;

/// Python's `int(s)` failure, message verbatim from CPython. `s` is the
/// point-less `str(number)` slice `to_cardinal` feeds it — sign already
/// stripped, because the leading "-" is peeled off the string before `int()`
/// ever runs.
fn int_value_error(s: &str) -> N2WError {
    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
}

/// Python `repr(float)` for the scientific range, on an already-`abs()`ed
/// finite f64 (`a >= 1e16`, or `0 < a < 1e-4`).
///
/// Rust's `{:e}` yields the shortest round-trip mantissa — the same digits
/// CPython's repr picks — but lays the exponent out as `1e16` / `1e-5` where
/// Python writes `1e+16` / `1e-05`: always a sign, zero-padded to two digits.
/// Only the exponent dressing needs fixing up. (Same helper as `lang_ky.rs`,
/// whose Python module is the same copy-paste family.)
fn py_float_sci_repr(a: f64) -> String {
    let sci = format!("{:e}", a); // "1e16", "1.5e20", "1e-5"
    let (mant, exp_str) = sci.split_once('e').expect("{:e} always emits an 'e'");
    let exp: i64 = exp_str.parse().expect("{:e} exponent is a base-10 integer");
    format!(
        "{}e{}{:02}",
        mant,
        if exp < 0 { '-' } else { '+' },
        exp.abs()
    )
}

/// Where `str(number)` sends the value in `to_cardinal` (sign already peeled).
enum StrRoute {
    /// `"." in n` — the decimal grammar spells every fractional character.
    Point,
    /// No "." and `int(n)` succeeds — `_int_to_word` runs on this magnitude.
    WholeDigits(BigInt),
}

/// Reproduce Python's `"." in str(number)` routing, including the `int(n)`
/// `ValueError` for the point-less forms that are not integer literals.
///
/// * **Float** — repr is fixed-notation (always with a ".") for every finite
///   value outside the scientific range, so the only no-point floats are the
///   scientific ones (`|v| >= 1e16`, `0 < |v| < 1e-4`) and inf/nan — all of
///   which make `int()` raise. `FloatValue::has_visible_point` is *not* used
///   here: it reports `true` for tiny scientific floats (`1e-05`), whose repr
///   has no point and must raise.
/// * **Decimal** — `str(Decimal)` is reconstructed by `python_decimal_str`;
///   a point routes to the decimal grammar, plain digits (canonical integral
///   Decimals like "100") parse, and everything else ("1E+2", "1E+20") is the
///   `int()` ValueError.
fn route_by_str(v: &FloatValue) -> Result<StrRoute> {
    match v {
        FloatValue::Float { value, .. } => {
            let a = value.abs();
            if !a.is_finite() {
                // str(float("inf")) == "inf", str(float("nan")) == "nan" —
                // no ".", so int() raises. The "-" of "-inf" is stripped by
                // the string-sign peel before int() ever sees it.
                return Err(int_value_error(if a.is_nan() { "nan" } else { "inf" }));
            }
            if a >= 1e16 || (a > 0.0 && a < 1e-4) {
                // repr picked exponent form: no "." in n -> int(n) raises.
                return Err(int_value_error(&py_float_sci_repr(a)));
            }
            // Every other finite float reprs with a point ("5.0", "0.0001").
            Ok(StrRoute::Point)
        }
        FloatValue::Decimal { value, .. } => {
            // abs() first: Python strips the string sign before this test.
            let s = python_decimal_str(&value.abs());
            if s.contains('.') {
                Ok(StrRoute::Point)
            } else if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
                Ok(StrRoute::WholeDigits(s.parse().expect("digits only")))
            } else {
                // "1E+2", "1E+20", ... — int() raises.
                Err(int_value_error(&s))
            }
        }
    }
}

pub struct LangFo {
    /// `Num2Word_FO.CURRENCY_FORMS`.
    ///
    /// Built once in [`LangFo::new`] and stored, never per call: the binding
    /// holds the converter in a `OnceLock`, so this table is constructed
    /// exactly once per process.
    ///
    /// Each entry is a 2-tuple of 2-tuples, `((unit_sg, unit_pl), (sub_sg,
    /// sub_pl))`. The arity is load-bearing: `to_currency` indexes `[1]` for
    /// the plural and `to_cheque` takes `cr1[-1]`, so both forms must be
    /// present and distinct.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangFo {
    pub fn new() -> Self {
        // Insertion order is irrelevant to a HashMap; FALLBACK_CURRENCY
        // captures the one place Python's ordering was observable.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("DKK", &["króna", "krónur"][..], &["oyra", "oyru"][..]),
            ("USD", &["dollari", "dollarar"][..], &["sent", "sent"][..]),
            ("EUR", &["evra", "evrur"][..], &["sent", "sent"][..]),
        ]
        .into_iter()
        .map(|(k, u, s)| (k, CurrencyForms::new(u, s)))
        .collect();
        LangFo { currency_forms }
    }

    /// The `(left, right)` split at the head of `Num2Word_FO.to_currency`, for
    /// a value already made non-negative.
    ///
    /// Python works on the *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Taking the first two fractional digits and right-padding to two is
    /// exactly `trunc(frac * 100)` for any plain decimal string — `"5"` →
    /// `"50"` → 50 is `trunc(0.5 * 100)`, `"345"` → `"34"` is
    /// `trunc(0.345 * 100)` — so this computes the split arithmetically rather
    /// than round-tripping the value back through `BigDecimal`'s `Display`.
    /// Identical results, one less dependency on a formatting impl we do not
    /// control. (`lang_eu.rs` ports the same idiom the same way.)
    ///
    /// A true `int` has no "." in `str(val)`, so `len(parts) > 1` is false and
    /// `right` is 0 — which is also what `frac == 0` gives here. FO therefore
    /// needs no `isinstance(val, int)` branch: bug 11 means the int/float
    /// distinction is invisible in the output.
    fn split_currency(val: &CurrencyValue) -> (BigInt, BigInt) {
        match val {
            CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let abs = d.abs();
                // int(parts[0]) — truncation, and abs is non-negative, so this
                // is floor as well.
                let left = abs.with_scale(0).as_bigint_and_exponent().0;
                let frac = &abs - BigDecimal::from(left.clone());
                let right = (frac * BigDecimal::from(CENT_DIVISOR))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                (left, right)
            }
        }
    }

    /// Port of `Num2Word_FO._int_to_word`.
    ///
    /// Raises only `OverflowError`, at `maxval` (#147); Python instead
    /// stringified everything from 10^9 (bug 3). Every
    /// `ONES`/`TENS` index is bounded by the enclosing range check (a value
    /// `< 100` divided by 10 is `< 10`), so the `to_usize` casts below are
    /// proven safe rather than assumed — the BigInt is only narrowed once the
    /// branch has established it fits.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        // Python: `return self.ones[0] if self.ones[0] else "zero"`.
        // ONES[0] is "" → falsy → the fallback always wins. The condition is
        // kept verbatim rather than folded to ZERO_WORD to document the dead arm.
        if number.is_zero() {
            return Ok(if ONES[0].is_empty() {
                ZERO_WORD.to_string()
            } else {
                ONES[0].to_string()
            });
        }

        // Unreachable from `to_cardinal`, which strips the sign before calling
        // in. Kept because Python has it: it would double the negword prefix.
        if number.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&number.abs())?));
        }

        if *number < BigInt::from(10) {
            // 1..=9; ONES[0] is unreachable (the is_zero arm caught it).
            return Ok(ONES[number.to_usize().expect("< 10")].to_string());
        }

        if *number < BigInt::from(100) {
            let (tens_val, ones_val) = number.div_mod_floor(&BigInt::from(10));
            let tens_word = TENS[tens_val.to_usize().expect("< 10")];
            if ones_val.is_zero() {
                return Ok(tens_word.to_string());
            }
            return Ok(format!("{} {}", tens_word, ONES[ones_val.to_usize().expect("< 10")]));
        }

        if *number < BigInt::from(1000) {
            let (hundreds_val, remainder) = number.div_mod_floor(&BigInt::from(100));
            // Python indexes `self.ones` directly here rather than recursing,
            // which is why 100 is "ein hundrað" and not "hundrað".
            let mut result = format!("{} {}", ONES[hundreds_val.to_usize().expect("< 10")], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        if *number < BigInt::from(1_000_000) {
            let (thousands_val, remainder) = number.div_mod_floor(&BigInt::from(1000));
            let mut result = format!("{} {}", self.int_to_word(&thousands_val)?, THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        if *number < BigInt::from(1_000_000_000) {
            let (millions_val, remainder) = number.div_mod_floor(&BigInt::from(1_000_000));
            let mut result = format!("{} {}", self.int_to_word(&millions_val)?, MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        // Python: `return str(number)  # Fallback for very large numbers` —
        // bug 3. The scales above million instead, same composition (#147);
        // the maxval check above keeps the multiplier below 1000.
        for &(exp, word) in HIGH_SCALES.iter() {
            let scale = pow10_big(exp);
            if *number >= scale {
                let (scale_val, remainder) = number.div_mod_floor(&scale);
                let mut result = format!("{} {}", self.int_to_word(&scale_val)?, word);
                if !remainder.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&remainder)?);
                }
                return Ok(result);
            }
        }
        unreachable!("every value >= 10^9 and below maxval has a HIGH_SCALES entry")
    }
}

impl Default for LangFo {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangFo {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "DKK"
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


    /// Port of `Num2Word_FO.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input, peels a leading "-" off the *string*, and
    /// re-parses with `int()`; `str(int)` never contains ".", so integers
    /// always take the `else` branch. Peeling the sign textually and re-parsing
    /// is exactly `abs()` for every BigInt, so that is what we do.
    ///
    /// The trailing `.strip()` is what removes the space inside `negword`
    /// ("minus ") from a bare-negword result; for the shapes reachable here it
    /// only ever trims the seam, since `int_to_word` never returns padding.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_FO.to_ordinal`: cardinal + a literal "-ti", with no
    /// `verify_ordinal` call and no inspection of what came back. See bugs
    /// 4, 5 and 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-ti", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_FO.to_ordinal_num`: `str(number) + "."`. Note this
    /// overrides `Num2Word_Base.to_ordinal_num` (which returns the value
    /// untouched), and that it keeps the minus sign: -1 → "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_FO.to_year`. Python overrides the base method but the
    /// body is just `return self.to_cardinal(val)`; the `longval=True`
    /// parameter is accepted and never read. So 1900 → "ein túsund níggju
    /// hundrað" (not a century split) and -44 → "minus fjøruti fýra" (not
    /// "fjøruti fýra f.Kr."). Stated explicitly rather than left to the trait
    /// default, since Python does override it.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the float/Decimal path of `Num2Word_FO.to_cardinal`.
    ///
    /// FO does **not** inherit `Num2Word_Base.to_cardinal_float`/`float2tuple`.
    /// It overrides `to_cardinal(number)`, which operates on `str(number)`:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// ```
    ///
    /// The fractional part is read **digit by digit off the decimal string**,
    /// so the `float2tuple` f64-artefact heuristic never applies here: `2.675`
    /// becomes `str` `"2.675"` → `"6 7 5"` directly, with no `674.999…` to
    /// rescue. Reproduce that by rebuilding the repr string, not by arithmetic
    /// on the raw double (which *would* reintroduce the artefact).
    ///
    /// * **Float** — Rust's `{}` drops the `.0` on whole floats (`1.0` → "1"),
    ///   so the repr is reconstructed with `{:.precision$}` on the absolute
    ///   value. `precision` is the repr-derived fractional length (== Python's
    ///   `abs(Decimal(str(v)).as_tuple().exponent)`), and formatting the double
    ///   back to that many places recovers exactly the shortest-round-trip
    ///   digits.
    /// * **Decimal** — reconstructed arithmetically (exact): `int(left)` is the
    ///   truncated integer part, and the fractional digits are
    ///   `(abs - left) * 10**precision` left-padded to `precision`. This keeps
    ///   trailing zeros the way `str(Decimal("1.10"))` does ("1.10" → "10").
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is **inert**
    /// for FO: `to_cardinal` takes no such parameter, so the kwarg is dropped
    /// before it can matter. Verified live:
    /// `num2words(0.5, lang="fo", precision=3)` == "null komma fimm".
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Rebuild `n` (already sign-stripped) as "<left>.<right>" with exactly
        // `precision` fractional digits — or no "." when precision is 0, which
        // mirrors Python's `if "." in n` guard falling through to the integer
        // branch. `neg` mirrors `str(number).startswith("-")`.
        let (neg, left, right_str) = match value {
            FloatValue::Float { value, precision } => {
                // Sign *bit*, not `< 0.0`: repr(-0.0) is "-0.0", so Python's
                // startswith("-") is true and FO answers "minus null komma
                // null". A `< 0.0` test would drop the minus.
                let neg = value.is_sign_negative();
                let prec = *precision as usize;
                let s = format!("{:.p$}", value.abs(), p = prec);
                match s.split_once('.') {
                    Some((l, r)) => (neg, l.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero()), r.to_string()),
                    None => (neg, s.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero()), String::new()),
                }
            }
            FloatValue::Decimal { value, precision } => {
                let neg = value.is_negative();
                let abs = value.abs();
                // int(left) — truncate the absolute value toward zero.
                let left = abs.with_scale(0).as_bigint_and_exponent().0;
                if *precision == 0 {
                    (neg, left, String::new())
                } else {
                    let frac = &abs - BigDecimal::from(left.clone());
                    let scale = BigInt::from(10).pow(*precision);
                    let right = (frac * BigDecimal::from(scale))
                        .with_scale(0)
                        .as_bigint_and_exponent()
                        .0;
                    // Left-pad to `precision` digits (right is non-negative and
                    // < 10**precision, so this never truncates). Manual pad
                    // rather than a numeric fill so BigInt's Display is used
                    // verbatim.
                    let rs = right.to_string();
                    let pad = (*precision as usize).saturating_sub(rs.len());
                    (neg, left, format!("{}{}", "0".repeat(pad), rs))
                }
            }
        };

        // ret += _int_to_word(int(left)) [+ " komma " + per-digit words];
        // strip. `pointword` is used raw — FO applies no title() here.
        let mut tokens: Vec<String> = vec![self.int_to_word(&left)?];
        if !right_str.is_empty() {
            tokens.push(POINTWORD.to_string());
            for ch in right_str.chars() {
                // Each char is one decimal digit; int(digit) ∈ 0..=9.
                let d = ch.to_digit(10).unwrap_or(0);
                tokens.push(self.int_to_word(&BigInt::from(d))?);
            }
        }
        let joined = tokens.join(" ");
        // negword ("minus ") is prepended raw, then the whole thing stripped —
        // exactly Python's `ret = self.negword + ...; return ret.strip()`.
        let result = if neg { format!("{}{}", NEGWORD, joined) } else { joined };
        Ok(result.trim().to_string())
    }

    /// `to_cardinal(float/Decimal)` — the full entry, routing on
    /// `"." in str(number)` rather than on whole-ness (Base's
    /// `int(value) == value` test never runs; FO overrides `to_cardinal`
    /// outright).
    ///
    /// A whole float therefore keeps its ".0" tail (`5.0` -> "fimm komma
    /// null", `-0.0` -> "minus null komma null"), a point-less integral
    /// Decimal takes the integer grammar (`Decimal("100")` -> "ein hundrað"),
    /// and a point-less non-integer form is Python's `int()` ValueError
    /// (`1e+16`, `Decimal("1E+2")`).
    ///
    /// `precision_override` is threaded through untouched; FO's grammar never
    /// reads a precision, so it is inert either way (see `to_cardinal_float`).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        match route_by_str(value)? {
            StrRoute::Point => self.to_cardinal_float(value, precision_override),
            StrRoute::WholeDigits(abs_int) => {
                // `(ret + self._int_to_word(int(n))).strip()`, with the sign
                // already peeled off the string into `ret = self.negword`.
                let body = self.int_to_word(&abs_int)?;
                if value.is_negative() {
                    Ok(format!("{}{}", NEGWORD, body).trim().to_string())
                } else {
                    Ok(body)
                }
            }
        }
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-ti"` — the
    /// same blind suffix as the integer path (bugs 4/5/6), so `5.0` ==
    /// "fimm komma null-ti" and the ValueError of `1e+16` propagates
    /// unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-ti", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. Purely textual —
    /// nothing is parsed, so even the scientific forms succeed: `1e+16` ==
    /// "1e+16." and `Decimal("1E+2")` == "1E+2.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    // `year_float_entry` is deliberately NOT overridden: FO's `to_year` is
    // `return self.to_cardinal(val)`, and the trait default routes through
    // the overridden `cardinal_float_entry` above — so `to_year(5.0)` ==
    // "fimm komma null" and `to_year(1e+16)` raises ValueError, as the
    // corpus pins.

    /// `converter.str_to_number` — Base's `Decimal(value)`, which FO does not
    /// override. `Decimal("Infinity")`/`"NaN"` parse fine; the ValueError FO
    /// shows happens *later*, inside its own `to_cardinal`, which is now
    /// served natively by [`Lang::inf_result`] / [`Lang::nan_result`] rather
    /// than by a Python fallback.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `-Infinity` per mode. FO's `to_cardinal`
    /// re-parses the sign-stripped `str(number)` with `int()`, so
    /// `int("Infinity")` raises `ValueError` for cardinal/ordinal/year (the
    /// leading "-" of "-Infinity" is peeled off the string first, so the
    /// message names 'Infinity' either way). `to_ordinal_num` never parses —
    /// it is `str(number) + "."` — so it succeeds: "Infinity." / "-Infinity.".
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!("{}Infinity.", if negative { "-" } else { "" })),
            _ => Err(int_value_error("Infinity")),
        }
    }

    /// `Decimal('NaN')` per mode. `int("NaN")` raises `ValueError` for
    /// cardinal/ordinal/year; `to_ordinal_num` returns "NaN." unparsed.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaN.".to_string()),
            _ => Err(int_value_error("NaN")),
        }
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `CURRENCY_FORMS[code]`, **strictly** — `None` for anything else.
    ///
    /// Deliberately *not* the DKK-defaulting lookup FO's `to_currency` uses
    /// (bug 8). This hook exists to serve the inherited
    /// `Num2Word_Base.to_cheque`, which subscripts
    /// `self.CURRENCY_FORMS[currency]` and turns the `KeyError` into
    /// `NotImplementedError`. The fallback belongs to `to_currency` alone and
    /// lives there; leaking it here would make `cheque:GBP` return
    /// "... KRÓNUR" instead of raising, contradicting the corpus.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_FO.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="DKK", cents=True,
    ///                 separator=" ", adjective=False):
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
    /// A complete override: `Num2Word_Base.to_currency` is never called, so
    /// none of `parse_currency_parts`, `pluralize`, `_cents_verbose`,
    /// `_cents_terse`, `CURRENCY_PRECISION` or `CURRENCY_ADJECTIVES` is
    /// reachable from here. Infallible — there is no `NotImplementedError`
    /// path (bug 8) and no table index that can escape its range.
    ///
    /// Note the plural is selected by `left != 1` / `right != 1` directly
    /// rather than by `pluralize`, which `Num2Word_FO` leaves abstract.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // declared by Python, never read — bug 10.
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Python: `if val < 0: is_negative = True; val = abs(val)`. The abs is
        // folded into split_currency, which every later step reads through.
        let is_negative = val.is_negative();
        let (left, right) = Self::split_currency(val);

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — bug 8. Python
        // evaluates the default eagerly on every call, but it cannot fail:
        // DKK is always present.
        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            None => &self.currency_forms[FALLBACK_CURRENCY],
        };

        let one = BigInt::one();
        let left_str = self.int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left != one { &forms.unit[1] } else { &forms.unit[0] }
        );

        // `if cents and right:` — a truthiness test on the cent count, not on
        // the type of `val`. See bug 11.
        if cents && !right.is_zero() {
            let cents_str = self.int_to_word(&right)?;
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            });
        }

        if is_negative {
            // negword carries a trailing space ("minus "), so this is the seam
            // the final strip() does *not* touch.
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
