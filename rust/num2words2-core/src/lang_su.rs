//! Port of `lang_SU.py` (Sundanese).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "miliar" (10^9) and "triliun" (10^12) (su.wikipedia "Républik
//! Indonésia"), composed like the million arm, and raises `OverflowError` from
//! 10^15, which `maxval()` reports. Where the notes below describe the digit
//! fallback ("1000000000", "no overflow check"), they describe Python; that
//! arm is now unreachable.
//!
//! Registry check: `__init__.py` maps `"su" -> lang_SU.Num2Word_SU()`, so this
//! is the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_SU` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `__init__` never builds `self.cards` and never sets `self.MAXVAL`.
//! `to_cardinal` is overridden outright and drives a private `_int_to_word`
//! recursion. `cards`/`maxval`/`merge` therefore stay at their trait defaults
//! here and are never consulted. There is **no overflow check** at all — see
//! bug 3 below for what happens instead.
//!
//! Inherited from `Num2Word_Base` and left alone by SU:
//!   * `is_title = False`, `exclude_title = []` → `title()` is the identity,
//!     so the trait defaults are correct.
//!
//! Everything in scope (`to_cardinal`, `to_ordinal`, `to_ordinal_num`,
//! `to_year`) is overridden by `Num2Word_SU` itself, so no base-class
//! behaviour leaks into the four modes.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. Each of the following looks wrong and is
//! nevertheless exactly what Python emits (verified against the frozen corpus):
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** `_int_to_word` opens with
//!    `return self.ones[0] if self.ones[0] else "zero"` and `ones[0]` is the
//!    empty string, so Python always said the English "zero". The port says
//!    the Sundanese "nol". The decimal word is still the English "point":
//!    CLDR's Sundanese rules say "titik", Indonesian usage "koma", and no
//!    Sundanese source settles it.
//! 2. **The teens are built compositionally and come out wrong.** 11..19 go
//!    through the generic `tens[1] + " " + ones[n]` path, yielding
//!    "sapuluh hiji" (lit. "ten one") for 11 and "sapuluh dua" for 12. Real
//!    Sundanese uses "sabelas"/"dua belas". Likewise 100 is `ones[1] + " " +
//!    "ratus"` == "hiji ratus" rather than "saratus", and 1000 is "hiji rebu"
//!    rather than "sarebu". Preserved verbatim.
//! 3. **Numbers >= 10^9 are not converted at all.** `_int_to_word`'s final
//!    `else` is `return str(number)`, so `to_cardinal(10**9)` == "1000000000"
//!    — a bare digit string, no words, no `OverflowError`. This is the only
//!    unbounded path, which is why the recursion below stays on `BigInt`.
//!    It also leaks into the other modes: `to_ordinal(10**9)` ==
//!    "1000000000-na".
//! 4. **`to_ordinal_num` appends a period, not an ordinal marker.**
//!    `str(number) + "."` gives "1." — European ordinal-abbreviation style,
//!    unrelated to Sundanese. It is also the only mode that keeps a negative
//!    sign in numeric form: `to_ordinal_num(-1)` == "-1.".
//! 5. **`to_ordinal` is cardinal + "-na" with no linguistic agreement**, and
//!    the suffix binds to the whole phrase, sign included:
//!    `to_ordinal(-1)` == "minus hiji-na".
//! 6. `negword` is `"minus "` — with a **trailing space** baked into the
//!    attribute. `to_cardinal` concatenates it raw and relies on the final
//!    `.strip()` to tidy up. [`LangSu::negword`] returns the attribute
//!    verbatim rather than a trimmed copy.
//!
//! # Dead code in the Python, modelled but unreachable
//!
//! `_int_to_word` has a `if number < 0:` branch returning
//! `negword + _int_to_word(abs(number))`. `to_cardinal` strips the "-" from
//! the *string* before calling `int()`, so the callee never sees a negative.
//! It is reproduced in [`LangSu::int_to_word`] for fidelity but cannot fire
//! from any of the four in-scope modes.
//!
//! # Errors
//!
//! None on the integer modes. Every in-scope path is total: the tables are
//! indexed only with values the range checks have already bounded to 0..=9,
//! and the >= 10^9 fallback swallows what would elsewhere be an overflow. No
//! `N2WError` is ever returned for integer input. The currency surface *can*
//! error — see the currency section below.
//!
//! # The currency surface
//!
//! `Num2Word_SU` declares its own `CURRENCY_FORMS` class attribute with
//! exactly three entries, in this insertion order: **IDR, USD, EUR**. It is a
//! fresh dict on `Num2Word_SU`, *not* the shared `Num2Word_EUR` dict that
//! `Num2Word_EN.__init__` mutates at import time, so none of the ~24 codes EN
//! injects (AUD/JPY/KWD/...) are visible here and the EUR/GBP rewrite trap
//! documented in `PORTING_CURRENCY.md` does not apply. Verified against the
//! live interpreter, not the source literal.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are inherited from
//! `Num2Word_Base` and are both `{}`, so `currency_precision` is 100 for every
//! code (KWD/BHD are *not* 3-decimal here) and `currency_adjective` is always
//! `None`. Both trait defaults already say exactly that, so neither is
//! overridden.
//!
//! The two halves of the surface behave completely differently, and the split
//! is the single most important thing about this file:
//!
//! * **`to_currency` is overridden outright** and never raises for an unknown
//!   code. It does `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS
//!   .values())[0])` — an unknown code silently falls back to `values()[0]`,
//!   which is **IDR**. So `to_currency(1, "GBP")` is "hiji rupiah", and
//!   `to_currency(12.34, "JPY")` is "sapuluh dua rupiah tilu puluh opat sen".
//!   Every currency code in the world resolves; nothing is ever
//!   NotImplementedError.
//! * **`to_cheque` is *not* overridden**, so it comes from `Num2Word_Base`,
//!   which does `self.CURRENCY_FORMS[currency]` inside a `try` and converts
//!   the `KeyError` into `NotImplementedError`. There is no IDR fallback on
//!   this path. `cheque:GBP` therefore raises where `currency:GBP` happily
//!   answers "rupiah". [`LangSu::currency_forms`] returns `None` for unknown
//!   codes to drive exactly that, and [`LangSu::lang_name`] supplies
//!   "Num2Word_SU" for the message.
//!
//! `to_currency` also ignores `adjective` entirely (the parameter is accepted
//! and never read) and never consults `pluralize`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse` or `CURRENCY_PRECISION` — it hand-rolls
//! its own singular/plural pick and its own cents extraction. `to_cheque`, by
//! contrast, goes through `_money_verbose` -> `to_cardinal`, which the trait
//! default already routes correctly.
//!
//! # `to_currency` does string surgery on `str(val)`, not arithmetic
//!
//! This is the crux of the port. Python's `to_currency` never divides:
//!
//! ```python
//! parts = str(val).split(".")
//! left = int(parts[0]) if parts[0] else 0
//! right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
//! ```
//!
//! Consequences, all reproduced:
//!
//! 7. **Cents are truncated to two digits, never rounded.** `12.345` and
//!    `12.999` give 34 and 99 cents; `0.001` gives `"001"[:2] == "00"` -> 0
//!    cents, i.e. plain "nol euros".
//! 8. **`ljust` scales a short fraction.** `0.5` -> `"5".ljust(2,"0")` ==
//!    `"50"` -> fifty cents. Correct here, but arrived at by string padding.
//! 9. **An `int` skips cents structurally, not by an `isinstance` check.**
//!    `str(5)` has no ".", so `len(parts) == 1` and `right` stays 0. A float
//!    `1.0` still yields `parts[1] == "0"` -> `"00"` -> 0, which is *falsy*,
//!    so `1.0` also prints no cents ("hiji euro"). Same output, different
//!    route than `Num2Word_Base`.
//! 10. **Singular/plural are decided independently for units and subunits**,
//!    so `1.5` -> "hiji euro lima puluh cents": singular unit, plural subunit.
//! 11. **Bug 3 leaks into currency**: `to_currency(10**9, "EUR")` ==
//!    "1000000000 euros".
//! 12. **`negword`'s trailing space carries the separation**: the result is
//!    `"minus " + result`, then `.strip()`. `-0.5` -> "minus zero euros lima
//!    puluh cents".
//!
//! Because the Python reads `str(val)`, [`python_str`] reconstructs that exact
//! string from the `BigDecimal` rather than using `BigDecimal`'s own
//! `Display`, which is *not* the same function: bigdecimal renders `2.5e+20`
//! as "25e+19" and `0.0` as "0", and flips to exponential form on its own
//! thresholds. See [`python_str`] for the one regime that cannot be recovered.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dolar / euro with sen. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.negword`. The trailing space is part of the Python attribute.
const NEGWORD: &str = "minus ";
/// `self.pointword`.
const POINTWORD: &str = "point";

/// The `"zero"` fallback in `_int_to_word`. Python writes
/// `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""`, so this is
/// the only reachable result for 0.
const ZERO: &str = "nol";

/// `self.ones`. Index 0 is `""` (see bug 1) and is never used as a word.
const ONES: [&str; 10] = [
    "", "hiji", "dua", "tilu", "opat", "lima", "genep", "tujuh", "dalapan", "salapan",
];

/// `self.tens`. Index 0 is `""`; unreachable because the `< 100` branch is
/// only entered when `number >= 10`, so `number // 10 >= 1`.
const TENS: [&str; 10] = [
    "",
    "sapuluh",
    "dua puluh",
    "tilu puluh",
    "opat puluh",
    "lima puluh",
    "genep puluh",
    "tujuh puluh",
    "dalapan puluh",
    "salapan puluh",
];

/// `self.hundred`.
const HUNDRED: &str = "ratus";
/// `self.thousand`.
const THOUSAND: &str = "rebu";
/// `self.million`.
const MILLION: &str = "juta";

/// Python's `CURRENCY_FORMS` **insertion order**. Load-bearing: `to_currency`
/// falls back to `list(self.CURRENCY_FORMS.values())[0]`, and dicts preserve
/// insertion order, so `[0]` is whatever was written first in the class body —
/// "IDR". (Note `pprint` sorts keys, so dumping the dict makes EUR look first;
/// `list(...keys())` confirms IDR/USD/EUR.)
const CURRENCY_ORDER: [&str; 3] = ["IDR", "USD", "EUR"];

/// Reproduce Python's `str(val)` for the `Decimal` arm of [`CurrencyValue`].
///
/// `to_currency` slices the decimal *string*, so the port has to rebuild the
/// same string. The shim sends `str(value)` across and `BigDecimal::from_str`
/// parses it without normalising, so `(int_val, scale)` still holds the
/// original digits and decimal position and this inverts the parse exactly for
/// every value Python renders in fixed notation — the whole corpus, and every
/// realistic input.
///
/// `scale < 0` can only come from a string that carried an exponent (fixed
/// notation never yields a negative scale), so that branch rebuilds Python's
/// float repr — `1e+16`, `1.5e+16`, `2.5e+20` — and lets the caller's surgery
/// produce whatever Python produces from it, ValueError included.
///
/// # Known gap
///
/// The notation Python chose is *not* always recoverable. `str(1e-05)` is
/// `'1e-05'` but `str(Decimal('0.00001'))` is `'0.00001'`; both parse to the
/// identical `BigDecimal(1, scale=5)`, and Python's two answers differ
/// (ValueError vs "nol euros"). Same for a 17-significant-digit float at
/// `e+16` (scale lands on 0). This function takes the fixed reading in both
/// cases, which is right for `Decimal` input and wrong for `float` input.
/// Recovering it needs the original string, which `CurrencyValue::Decimal`
/// does not carry. Flagged in the port report; no corpus row reaches it.
fn python_str(d: &BigDecimal) -> String {
    let (int_val, scale) = d.as_bigint_and_exponent();
    // Callers abs() first, so int_val is already non-negative; abs() here is
    // belt and braces. (Python's `abs(val)` cannot produce "-0.0" either: the
    // `val < 0` guard is false for -0.0, so Python keeps the string "-0.0" and
    // then reads int("-0") == 0. BigDecimal drops the sign of zero at parse
    // time, so this returns "0.0" — a different string, the same 0/0 split,
    // and the same "nol euros".)
    let digits = int_val.abs().to_string();

    if scale < 0 {
        // Exponential. Python writes the mantissa with one digit before the
        // point and pads the exponent to two places: 1e+16, 1.25e+16.
        let exp = digits.len() as i64 - 1 - scale;
        let mut mantissa = String::from(&digits[..1]);
        if digits.len() > 1 {
            mantissa.push('.');
            mantissa.push_str(&digits[1..]);
        }
        return format!("{}e+{:02}", mantissa, exp);
    }

    let scale = scale as usize;
    if scale == 0 {
        return digits;
    }
    if digits.len() > scale {
        let point = digits.len() - scale;
        format!("{}.{}", &digits[..point], &digits[point..])
    } else {
        // Python always writes a leading zero: 0.5, never .5.
        format!("0.{}{}", "0".repeat(scale - digits.len()), digits)
    }
}

/// Python's `int(s)` on the fragments `to_currency` slices out.
///
/// Only reachable with a non-numeric fragment when the value arrived in
/// exponential notation — `int("1e+16")` and `int("5e")` both raise
/// ValueError, which is [`N2WError::Value`], with Python's exact message.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Sign-free `str(value)` for a *float* whose repr shows **no** decimal point
/// — i.e. `FloatValue::has_visible_point()` said no. For a finite f64 that
/// means "whole and `|v| >= 1e16`", where CPython's repr switches to exponent
/// form (`"1e+16"`); non-finite values print `"inf"`/`"nan"` (the sign having
/// been peeled off by the caller, exactly like Python's `n[1:]`).
///
/// Every output of this function contains a non-digit (`e`/`.`/`inf`/`nan`)
/// and exists only to be fed to `int(n)`, which raises the same ValueError for
/// either the exact-expansion or shortest-repr spelling; for the corpus-pinned
/// cases (1e+16, 1e+20) the two coincide exactly.
fn float_no_point_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return "inf".to_string();
    }
    // Finite without a visible point => whole and |v| >= 1e16, so this BigInt
    // conversion is exact.
    let digits = BigInt::from_f64(v.abs())
        .expect("finite whole f64 converts exactly")
        .to_string();
    let exp = digits.len() - 1; // >= 16, so always two+ exponent digits
    let mant = digits.trim_end_matches('0');
    let mant = if mant.len() <= 1 {
        // Power of ten: bare leading digit, no ".".
        digits[..1].to_string()
    } else {
        format!("{}.{}", &mant[..1], &mant[1..])
    };
    format!("{}e+{}", mant, exp)
}

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "miliar"), (12, "triliun")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

pub struct LangSu {
    /// `Num2Word_SU.CURRENCY_FORMS`. Built once here rather than per call —
    /// the registry holds `LangSu` in a `OnceLock`, so this is constructed
    /// exactly once per process.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the IDR entry, precomputed.
    ///
    /// Python re-evaluates this on every `to_currency` call (it is the default
    /// argument of `.get`, so it is built eagerly whether or not the code is
    /// found). It is a pure expression over a dict that is never mutated, so
    /// hoisting it is observationally identical and drops an allocation from
    /// the hot path.
    currency_fallback: CurrencyForms,
}

impl Default for LangSu {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSu {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        let mut currency_forms = HashMap::with_capacity(CURRENCY_ORDER.len());
        // Arity is load-bearing: to_currency indexes cr1[1]/cr2[1] for the
        // plural, so both forms must be present even where they are equal
        // (IDR's "rupiah"/"rupiah" — Sundanese does not mark plural, but the
        // tuple still has two slots).
        currency_forms.insert(
            "IDR",
            CurrencyForms::new(&["rupiah", "rupiah"], &["sen", "sen"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dolar", "dolar"], &["sen", "sen"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euro"], &["sen", "sen"]),
        );
        let currency_fallback = currency_forms[CURRENCY_ORDER[0]].clone();
        LangSu {
            currency_forms,
            currency_fallback,
        }
    }

    /// Port of `Num2Word_SU._int_to_word`.
    ///
    /// Total: no branch can panic. Each table index is derived from a value
    /// the enclosing range check has already pinned to 0..=9, and the final
    /// `else` absorbs everything from 10^9 up as a digit string (bug 3), so
    /// there is no upper bound to overflow. The recursion stays on `BigInt`
    /// precisely because that last branch accepts arbitrarily large input.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            // Python: `self.ones[0] if self.ones[0] else "zero"` — ones[0] is
            // "", so this always takes the fallback.
            return ZERO.to_string();
        }

        if number.is_negative() {
            // Dead code in Python: to_cardinal removes the sign from the
            // string before int() ever runs. Reproduced for fidelity.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        if *number < BigInt::from(10) {
            // Bounded to 1..=9 by the checks above.
            return ONES[number.to_usize().expect("0 < number < 10")].to_string();
        }

        if *number < BigInt::from(100) {
            let (tens_val, ones_val) = number.div_rem(&BigInt::from(10));
            let tens_val = tens_val.to_usize().expect("10 <= number < 100");
            let ones_val = ones_val.to_usize().expect("remainder mod 10");
            if ones_val == 0 {
                return TENS[tens_val].to_string();
            }
            // Builds the teens as "sapuluh hiji" etc. — see bug 2.
            return format!("{} {}", TENS[tens_val], ONES[ones_val]);
        }

        if *number < BigInt::from(1000) {
            let (hundreds_val, remainder) = number.div_rem(&BigInt::from(100));
            let hundreds_val = hundreds_val.to_usize().expect("100 <= number < 1000");
            // Python indexes `ones` directly here rather than recursing, so
            // 100 == "hiji ratus" (bug 2).
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000) {
            let (thousands_val, remainder) = number.div_rem(&BigInt::from(1000));
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000_000) {
            let (millions_val, remainder) = number.div_rem(&BigInt::from(1_000_000));
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        // Scale words above a million (gladiaio/num2words2#147), composed like
        // the million arm. Every entry point rejects values at or above
        // `maxval_ceiling()` first, so the top quotient is always below 1000.
        for &(exp, word) in SCALES.iter().rev() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let mut result = format!("{} {}", self.int_to_word(&(number / &scale)), word);
                let rest = number % &scale;
                if !rest.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&rest));
                }
                return result;
            }
        }

        // Python: `return str(number)  # Fallback for very large numbers`.
        // No words, no OverflowError — just the digits (bug 3).
        number.to_string()
    }
}

impl Lang for LangSu {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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

    /// `self.negword` verbatim, trailing space included (bug 6).
    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "point"
    }

    /// Port of `Num2Word_SU.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).strip()`, peels a leading "-" into
    /// `ret = self.negword`, then branches on whether `n` contains ".".
    /// `str(int)` never does, so integers always take the `else` branch:
    /// `(ret + self._int_to_word(int(n))).strip()`. Stripping the "-" from
    /// the string and re-parsing is exactly `abs()`. The float branch
    /// (pointword + per-digit decimals) is out of scope.
    ///
    /// The trailing `.strip()` is what removes `negword`'s built-in trailing
    /// space when `_int_to_word` returns "" — which it never does — and is
    /// otherwise a no-op, since no `_int_to_word` branch pads its result.
    /// Kept anyway to match the Python exactly.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.checked_int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_SU.to_ordinal`: cardinal + "-na", sign and all
    /// (bug 5). Never errors, because `to_cardinal` never does.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-na", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_SU.to_ordinal_num`: `str(number) + "."` (bug 4).
    /// Note this overrides the base default (`return value`) and is the one
    /// mode that keeps the digits and the minus sign: -1 → "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_SU.to_year`. Python's signature is
    /// `to_year(self, val, longval=True)`, but `longval` is accepted and
    /// ignored — the body is just `return self.to_cardinal(val)`, so years
    /// get no century/teen treatment: 1999 → "hiji rebu salapan ratus
    /// salapan puluh salapan".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float / Decimal entry routing --------------------------------

    /// `to_cardinal(float/Decimal)` FULL routing.
    ///
    /// Python's `to_cardinal` is string-driven: `"." in str(number)` picks the
    /// decimal grammar, and `str(5.0)` is `"5.0"`, so **whole floats keep
    /// their ".0" tail** ("lima point nol") instead of taking Base's
    /// whole-value integer route. Without a visible point the sign-free string
    /// lands in `int(n)`:
    ///   * `Decimal("5")` -> `"5"` -> the integer path ("lima");
    ///   * repr-exponential floats (`str(1e16)` == `"1e+16"`) and exponential
    ///     Decimals (`str(Decimal("1E+2"))` == `"1E+2"`) -> `int()` raises
    ///     ValueError, type and message reproduced by [`py_int`].
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        // Sign handling matches the textual `n.startswith("-")` peel: the
        // reconstructed string below is sign-free, exactly like Python's
        // `n[1:]` (is_negative() is sign-bit aware for the Float arm).
        let is_negative = value.is_negative();
        let text = match value {
            FloatValue::Float { value, .. } => float_no_point_repr(*value),
            FloatValue::Decimal { value, .. } => python_decimal_str(&value.abs()),
        };
        let body = self.checked_int_to_word(&py_int(&text)?)?;
        let out = if is_negative {
            format!("{}{}", NEGWORD, body)
        } else {
            body
        };
        Ok(out.trim().to_string())
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + "-na"` with no type guard, so floats get
    /// the full decimal phrase plus the suffix ("lima point nol-na") and the
    /// exponential-form ValueError propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-na", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, no `int()`
    /// anywhere — so it succeeds where the other modes raise ("1e+16.") and
    /// "-0.0" keeps its textual minus ("-0.0."). `repr_str` is the
    /// Python-side `str(number)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is `return self.to_cardinal(val)`,
    /// so the full float routing above applies verbatim, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which SU does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError. The
    /// binding otherwise maps `ParsedNumber::Inf` to the base integer path's
    /// OverflowError before any SU code runs, so the ValueError must be
    /// raised here. (NaN needs no interception: the binding's ValueError
    /// already matches `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Port of the float/Decimal branch of `Num2Word_SU.to_cardinal`.
    ///
    /// SU overrides `to_cardinal` outright and handles non-integers *inline*
    /// by doing string surgery on `str(number)`. It never calls the base
    /// `to_cardinal_float`, never builds `float2tuple`, and never reads
    /// `self.precision`. So **none** of the `float2tuple` rounding artefacts
    /// apply here — the fractional digits come straight off the repr string,
    /// one `_int_to_word` per digit character:
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
    /// `precision_override` is **ignored**: the dispatcher's `precision=`
    /// kwarg writes onto `self.precision`, which this code path never consults,
    /// so it has no effect on SU's float output (verified below).
    ///
    /// Consequences reproduced:
    ///
    /// * `1.005` → "hiji point nol nol lima" and `2.675` → "dua point genep
    ///   tujuh lima": the repr digits are taken verbatim, so the f64 artefacts
    ///   (`674.9999…`) never arise — there is no `abs(value-pre)*10**p` here.
    /// * Trailing repr zeros survive: `str(1.0)` == "1.0" → "hiji point nol",
    ///   and the Decimal `1.10` → "hiji point hiji nol".
    /// * Bug 3 leaks in: a `left` ≥ 10^9 is emitted as bare digits, so the
    ///   Decimal `98746251323029.99` → "98746251323029 point salapan salapan".
    /// * The sign is peeled off `str(number)` exactly as Python does, so
    ///   `str(-0.0)` == "-0.0" would yield "minus nol point nol" (Rust's
    ///   fixed formatting preserves the negative-zero sign, matching repr).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Reconstruct Python's `str(number)` for the two input arms.
        let s = match value {
            // str(float): fixed-notation repr with `precision` fractional
            // digits. `precision` is abs(Decimal(repr(f)).as_tuple().exponent)
            // — exactly the count of fractional digits repr shows — so
            // formatting the raw f64 to that many places reproduces the repr
            // (the shortest round-trip decimal rounds to itself), sign and all.
            FloatValue::Float { value: f, precision } => {
                format!("{:.*}", *precision as usize, f)
            }
            // str(Decimal): the exact digits and scale, rebuilt from the
            // BigDecimal by `python_str` (which drops the sign); re-attach it.
            FloatValue::Decimal { value: d, .. } => {
                let body = python_str(d);
                if d.is_negative() {
                    format!("-{}", body)
                } else {
                    body
                }
            }
        };

        // Python: `n = str(number).strip()`, then peel a leading "-" into
        // `ret = self.negword` (which carries its own trailing space, bug 6).
        let n = s.trim();
        let (is_neg, n) = match n.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, n),
        };
        let mut ret = if is_neg {
            NEGWORD.to_string()
        } else {
            String::new()
        };

        match n.split_once('.') {
            Some((left, right)) => {
                // ret += _int_to_word(int(left)) + " " + pointword + " "
                ret.push_str(&self.checked_int_to_word(&py_int(left)?)?);
                ret.push(' ');
                ret.push_str(POINTWORD);
                ret.push(' ');
                // for digit in right: ret += _int_to_word(int(digit)) + " "
                for ch in right.chars() {
                    ret.push_str(&self.checked_int_to_word(&py_int(&ch.to_string())?)?);
                    ret.push(' ');
                }
                Ok(ret.trim().to_string())
            }
            None => {
                // No "." in str(number). For a float this is unreachable in
                // the fixed-magnitude range (repr always shows ".0"); it is
                // reached only when str() chose exponential notation, where
                // Python's own `int(n)` raises ValueError — which py_int
                // reproduces (N2WError::Value). Integer Decimals (scale 0,
                // e.g. Decimal("5")) also land here and convert normally.
                Ok(format!("{}{}", ret, self.checked_int_to_word(&py_int(n)?)?)
                    .trim()
                    .to_string())
            }
        }
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError.
    fn lang_name(&self) -> &str {
        "Num2Word_SU"
    }

    /// `CURRENCY_FORMS[code]` — a plain lookup with **no** IDR fallback.
    ///
    /// This is the `to_cheque` view of the table (`self.CURRENCY_FORMS[currency]`
    /// -> KeyError -> NotImplementedError). `to_currency` does *not* go through
    /// here; it overrides the whole method and applies its own `.get(..., IDR)`
    /// fallback. Returning `None` for an unknown code is what makes
    /// `cheque:GBP` raise while `currency:GBP` answers in rupiah.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_SU.to_currency`.
    ///
    /// Mirrors the Python line for line: sign check, `str(val).split(".")`,
    /// a two-character truncating slice of the fraction, then a hand-rolled
    /// singular/plural pick per segment.
    ///
    /// `adjective` is accepted and ignored, exactly as in Python — SU never
    /// reads it and has no `CURRENCY_ADJECTIVES`, so `adjective=True` changes
    /// nothing. `pluralize` / `_money_verbose` / `_cents_verbose` /
    /// `_cents_terse` / `CURRENCY_PRECISION` are likewise all bypassed.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Python: `if val < 0: is_negative = True; val = abs(val)`.
        // abs() of a non-negative is the identity, so the guard only matters
        // for the flag.
        let is_negative = val.is_negative();
        let s = match val {
            // str(int) never contains ".", which is *why* ints skip cents
            // (quirk 9) — no isinstance check is involved.
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => python_str(&d.abs()),
        };

        // Python's str.split(".") with no maxsplit. Only [0] and [1] are read;
        // a BigDecimal can never round-trip to more than one ".".
        let parts: Vec<&str> = s.split('.').collect();

        let left = if parts[0].is_empty() {
            BigInt::zero()
        } else {
            py_int(parts[0])?
        };

        let right = if parts.len() > 1 && !parts[1].is_empty() {
            // `parts[1][:2].ljust(2, "0")` — truncate to two characters (no
            // rounding, quirk 7), then right-pad with "0" (quirk 8). Sliced by
            // chars, not bytes.
            let mut frac: String = parts[1].chars().take(2).collect();
            while frac.chars().count() < 2 {
                frac.push('0');
            }
            py_int(&frac)?
        } else {
            BigInt::zero()
        };

        // `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        // — unknown codes silently become IDR rather than raising.
        let cr = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);

        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            if left.is_one() { &cr.unit[0] } else { &cr.unit[1] }
        );

        // `if cents and right:` — `right` is an int, so this is `right != 0`.
        // That is the test 1.0 fails ("00" -> 0) and 0.01 passes ("01" -> 1).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(if right.is_one() {
                &cr.subunit[0]
            } else {
                &cr.subunit[1]
            });
        }

        if is_negative {
            // negword carries its own trailing space (quirk 6/12).
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
