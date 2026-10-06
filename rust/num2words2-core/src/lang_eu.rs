//! Port of `lang_EU.py` (Basque / Euskara).
//!
//! Shape: **self-contained**. `Num2Word_EU` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. `base.py`'s
//! `__init__` guards card construction behind
//! `any(hasattr(self, f) for f in ["high_numwords", "mid_numwords", "low_numwords"])`,
//! which is False here, so Python never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives the recursive
//! `_int_to_word` over magnitude bands. Consequently `cards`/`merge` stay at
//! their trait defaults, and Python has **no overflow check at all** — the
//! billions branch recurses without bound, so arbitrarily large values render
//! (10^18 == "bat mila milioi mila milioi", verified in the corpus). On a
//! large enough integer the port's recursion overflowed the native stack, so
//! it adds a ceiling (gladiaio/num2words2#203): `maxval` is 10^12, where
//! "mila" would repeat ("mila mila milioi"), and every mode raises
//! `OverflowError` from there.
//!
//! `setup()` overrides two inherited fields:
//!   * `negword  = "minus "`  — note the **trailing space**, which is
//!     significant: the overridden `to_cardinal` concatenates it raw rather
//!     than going through `base.py`'s `to_cardinal`, which would `.strip()` it
//!     and re-add a single space. Same result here, but the raw form is what
//!     this module actually executes, so [`Lang::negword`] returns it verbatim.
//!   * `pointword = "koma"` — reached on the float / Decimal path via the
//!     overridden `to_cardinal`'s `if "." in n` branch (see
//!     [`LangEu::to_cardinal_float`]).
//!
//! Inherited from `Num2Word_Base` but overridden by EU, so the trait defaults
//! are *not* used: `to_ordinal`, `to_ordinal_num`, `to_year`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every one of the following looks wrong and is
//! exactly what Python emits — each is confirmed against a frozen corpus row:
//!
//! 1. **"hamarta" — the doubled tens joiner.** `tens[3579]` already end in
//!    "…ta hamar" ("hogeita hamar" = 30, "berrogeita hamar" = 50, …). The
//!    `number < 100` branch then appends the literal `"ta "` before the unit
//!    regardless, producing "hamarta" instead of Basque's correct
//!    "hogeita hamaika"-style forms. So 99 == "laurogeita hamarta bederatzi"
//!    and 55 == "berrogeita hamarta bost". See [`int_to_word`].
//! 2. **Teens are built by prefix, not by table.** `11..=19` render as
//!    `"hama" + ones[n - 10]`, giving "hamabat" (11), "hamabi" (12),
//!    "hamalau" (14), "hamabederatzi" (19). Real Basque is "hamaika",
//!    "hamabi", "hamalau", "hemeretzi". Kept verbatim.
//! 3. **The billions branch has no `== 1` special case**, unlike the thousands
//!    and millions branches. So 10^9 == "bat mila milioi" (a leading bare
//!    "bat"), where the millions branch would have yielded "milioi bat".
//! 4. **`1_000_001` == "milioi bat bat"** — the millions branch emits the
//!    fused literal "milioi bat" for `million_val == 1`, then appends the
//!    remainder "bat", stuttering the unit word.
//! 5. **`to_ordinal`'s `if cardinal.endswith("t")` is dead code**: both arms
//!    return `cardinal + "garren"` identically. Reproduced as a single
//!    unconditional concatenation, which is byte-identical.
//! 6. **`to_ordinal` accepts negatives** rather than raising `errmsg_negord`
//!    like most modules: `to_ordinal(-1)` == "minus batgarren". The suffix
//!    lands on the last word, not the number as a whole.
//! 7. **`to_year` ignores its `longval` parameter entirely** and appends the
//!    ordinal-marker ". urtea" to a *cardinal*: 2000 == "bi mila. urtea".
//!
//! Bugs 8-13 live on the currency surface and are documented on
//! [`LangEu::to_currency`].
//!
//! # Currency
//!
//! `Num2Word_EU` overrides `to_currency` outright — see
//! [`LangEu::to_currency`], which reaches none of `Num2Word_Base`'s currency
//! machinery. It does **not** override `to_cheque`, `pluralize`,
//! `_money_verbose`, `_cents_verbose` or `_cents_terse`, and defines neither
//! `CURRENCY_ADJECTIVES` nor `CURRENCY_PRECISION` (both inherit the empty base
//! dicts, so `CURRENCY_PRECISION.get(code, 100)` is 100 for *every* code,
//! including JPY and KWD). Those all stay at their trait defaults, which
//! already mirror the base class:
//!
//! * `to_cheque` -> [`crate::currency::default_to_cheque`], which reproduces
//!   base's strict `CURRENCY_FORMS[currency]` lookup and its
//!   `Currency code "%s" not implemented for "%s"` NotImplementedError. Its
//!   `_money_verbose` call lands on EU's `to_cardinal`, so
//!   `cheque:EUR 1234.56` == "MILA BIEHUN ETA HOGEITA HAMARTA LAU AND 56/100
//!   EURO".
//! * `pluralize` keeps the default that raises — matching Python's abstract
//!   `raise NotImplementedError`. Neither EU's `to_currency` nor base's
//!   `to_cheque` calls it, so it stays unreachable.
//!
//! # Error variants
//!
//! One, and only on the cheque path: `to_cheque` with a code outside
//! {EUR, USD, GBP} raises `NotImplementedError`
//! ([`crate::base::N2WError::NotImplemented`]), pinned by six corpus rows
//! (JPY/KWD/BHD/INR/CNY/CHF).
//!
//! Everything else is infallible. Within the four integer modes every table
//! index is provably bounded (`ones[n]` only for `n < 10`, `ones[n - 10]` only
//! for `10 < n < 20`, `tens[n / 10]` only for `n < 100`, `ones[n / 100]` only
//! for `n < 1000`); the only raise is the port's 10^12 ceiling (#203). The
//! currency path cannot raise at all — an unknown code falls back to euros
//! (bug 9) instead of raising, which is precisely why `currency:JPY` succeeds
//! where `cheque:JPY` does not. The corpus agrees: every `eu` row in the four
//! integer modes and all 108 `currency:` rows have `"ok": true`.

use crate::base::{check_maxval, pow10_big, Lang as LangTrait, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::str::FromStr;

/// `ones`. Index 0 is the empty string, as in Python — it is only ever
/// selected when the caller has already excluded that case (e.g. the
/// `one_val == 0` arm of the tens branch returns `tens[..]` alone).
const ONES: [&str; 10] = [
    "",
    "bat",
    "bi",
    "hiru",
    "lau",
    "bost",
    "sei",
    "zazpi",
    "zortzi",
    "bederatzi",
];

/// `tens`. Basque counts in twenties, hence "hogeita hamar" (20 + 10) for 30.
/// Note that indices 3, 5, 7 and 9 already end in "hamar"; the `number < 100`
/// branch appends "ta " to these anyway, which is bug 1 in the module docs.
const TENS: [&str; 10] = [
    "",
    "hamar",
    "hogei",
    "hogeita hamar",
    "berrogei",
    "berrogeita hamar",
    "hirurogei",
    "hirurogeita hamar",
    "laurogei",
    "laurogeita hamar",
];

const ZERO_WORD: &str = "zero";
const NEGWORD: &str = "minus ";
const POINTWORD: &str = "koma";

/// `Num2Word_EU.to_currency`'s own default `separator=" eta "`.
const SEPARATOR_DEFAULT: &str = " eta ";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_EU.to_currency` declares `separator=" eta "`, but the `Lang` trait
/// carries no per-language parameter defaults, and both `__init__.py`'s fast
/// path and `bench/diff_test.py` substitute `kwargs.get("separator", ",")` —
/// **`Num2Word_Base`'s** default, not EU's — before the value crosses the
/// boundary. By then "caller omitted separator" and "caller explicitly passed a
/// comma" are the same `&str` and the information needed to tell them apart is
/// already gone.
///
/// So `,` is read back as the unset sentinel and EU's own default restored.
/// This is the only reading the oracle supports: every float row of the `eu`
/// currency corpus was generated by `num2words(v, lang="eu", to="currency",
/// currency=c)` with no `separator=`, and all 54 of them expect " eta "
/// ("hamabi euro eta hogeita hamarta lau zentimo").
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " eta " where Python would give ",". Expressing that
/// case needs `Option<&str>` in the trait signature, which lives in `base.rs`
/// — outside this port's remit. Flagged in the port report. `lang_ca.rs` and
/// `lang_es.rs` resolve the identical conflict the same way.
const SEPARATOR_UNSET: &str = ",";

/// `Num2Word_EU._split_currency`, for a value already made non-negative.
///
/// Python works on the *string*:
///
/// ```python
/// parts = str(n).split(".")
/// left = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// Taking the first two fractional digits and right-padding to two is exactly
/// `trunc(frac * 100)` for any plain decimal string — `"5" -> "50" -> 50` is
/// `trunc(0.5 * 100)`, `"567" -> "56"` is `trunc(0.567 * 100)` — so this
/// computes the split arithmetically rather than round-tripping the value back
/// through `BigDecimal`'s `Display`. Identical results, one less dependency on
/// a formatting impl we do not control.
///
/// Note the hard-coded 2: `_split_currency` never consults
/// `CURRENCY_PRECISION`, so 3-decimal (KWD/BHD) and 0-decimal (JPY) codes are
/// split at two digits like everything else. See [`LangEu::to_currency`].
fn split_currency(value: &BigDecimal) -> (BigInt, BigInt) {
    // `with_scale(0)` truncates toward zero; the caller has already applied
    // `abs()`, so truncation and Python's `int()`/floor agree.
    let integer = value.with_scale(0);
    let fraction = value - &integer;
    let cents = (fraction * BigDecimal::from(100)).with_scale(0);
    (
        integer.as_bigint_and_exponent().0,
        cents.as_bigint_and_exponent().0,
    )
}

/// Index a bounded table exactly as Python's `ones[i]` / `tens[i]` would.
///
/// Every call site has already proven `0 <= i < 10` (see the module docs'
/// "Error variants" note), so the `to_usize` cannot fail and the bounds check
/// cannot trip. Python would negative-index on a negative argument, but
/// `to_cardinal` strips the sign before calling [`int_to_word`], so no
/// negative ever reaches here.
fn small(table: &[&str; 10], i: &BigInt) -> String {
    table[i.to_usize().expect("caller proved 0 <= i < 10")].to_string()
}

/// `Num2Word_EU._int_to_word`.
///
/// Recursive over magnitude bands. The billions band recurses on
/// `number / 10^9` with no ceiling, so this is unbounded in `BigInt` —
/// deliberately, matching Python.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1_000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);

    if number < &ten {
        small(&ONES, number)
    } else if number == &ten {
        "hamar".to_string()
    } else if number < &BigInt::from(20) {
        // Bug 2: teens by prefix, not by table.
        format!("hama{}", small(&ONES, &(number - &ten)))
    } else if number < &hundred {
        let (ten_val, one_val) = number.div_mod_floor(&ten);
        if one_val.is_zero() {
            small(&TENS, &ten_val)
        } else {
            // Bug 1: "ta " is appended even when tens[ten_val] already ends
            // in "hamar", yielding "hamarta".
            format!("{}ta {}", small(&TENS, &ten_val), small(&ONES, &one_val))
        }
    } else if number < &thousand {
        let (hundred_val, remainder) = number.div_mod_floor(&hundred);
        let mut result = if hundred_val.is_one() {
            "ehun".to_string()
        } else {
            format!("{}ehun", small(&ONES, &hundred_val))
        };
        if !remainder.is_zero() {
            result.push_str(" eta ");
            result.push_str(&int_to_word(&remainder));
        }
        result
    } else if number < &million {
        let (thousand_val, remainder) = number.div_mod_floor(&thousand);
        let mut result = if thousand_val.is_one() {
            "mila".to_string()
        } else {
            format!("{} mila", int_to_word(&thousand_val))
        };
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        result
    } else if number < &billion {
        let (million_val, remainder) = number.div_mod_floor(&million);
        // Bug 4: the `== 1` arm fuses "milioi bat"; a nonzero remainder then
        // appends after it ("milioi bat bat" for 1_000_001).
        let mut result = if million_val.is_one() {
            "milioi bat".to_string()
        } else {
            format!("{} milioi", int_to_word(&million_val))
        };
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        result
    } else {
        let (billion_val, remainder) = number.div_mod_floor(&billion);
        // Bug 3: no `== 1` special case here, so 10^9 == "bat mila milioi".
        let mut result = format!("{} mila milioi", int_to_word(&billion_val));
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        result
    }
}

// ---- float / Decimal cardinal path -------------------------------------
//
// `Num2Word_EU` does **not** override `to_cardinal_float`; it overrides
// `to_cardinal`, which handles non-integers inline off `str(number)`:
//
// ```python
// def to_cardinal(self, number):
//     n = str(number).strip()
//     if n.startswith("-"):
//         n = n[1:]
//         ret = self.negword          # "minus "
//     else:
//         ret = ""
//     if "." in n:
//         left, right = n.split(".", 1)
//         ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//         ret += " ".join(self._int_to_word(int(d)) for d in right)
//         return ret
//     else:
//         return ret + self._int_to_word(int(n))
// ```
//
// `num2words(x, lang="eu", to="cardinal")` calls this overridden method
// directly (base.py line 590), so `Num2Word_Base.to_cardinal_float` — with its
// `float2tuple`, `10**precision` and `< 0.01` rescue — is **dead code** for EU.
// The f64 artefacts it exists to preserve never arise: `str()` is applied to
// the double directly, so the only rounding is inside `str(float)` itself,
// which is round-trip-shortest (banker's ties). Reproducing it therefore means
// reproducing CPython's `str(float)` and `str(Decimal)`, not the float path.
//
// The helpers below (`shortest_repr_digits`, `py_str_f64`, `py_str_decimal`,
// `py_int`, `py_int_digit`) are the verified ports from `lang_ceb.rs`, which
// solves the identical problem. `precision`/`precision_override` are unread:
// EU's `to_cardinal` takes no `precision` argument at all.

/// The shortest round-trip decimal digits of `a` (finite, non-negative), plus
/// CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// `{:e}` and CPython's `repr` are both "shortest string that reads back as the
/// same double" and agree on digit count always and on the digits almost
/// always; they part on an exact tie, where CPython breaks to **even** and
/// Rust's `flt2dec` shortest breaks **away from zero**. Re-rounding the printed
/// digit count with `{:.*e}` (round-half-to-even) and keeping it only when it
/// still round-trips recovers CPython's choice. Verified in `lang_ceb.rs`
/// against 777k doubles with zero mismatches.
fn shortest_repr_digits(a: f64) -> (String, i32) {
    let split = |s: &str| -> (String, i32) {
        let (mant, exp) = s.split_once('e').expect("{:e} always emits an 'e'");
        (
            mant.chars().filter(|c| *c != '.').collect(),
            exp.parse::<i32>().expect("{:e} exponent is an integer") + 1,
        )
    };

    let shortest = format!("{:e}", a);
    let (digits, decpt) = split(&shortest);

    let ties_even = format!("{:.*e}", digits.len() - 1, a);
    if ties_even.parse::<f64>() == Ok(a) {
        return split(&ties_even);
    }
    (digits, decpt)
}

/// CPython's `str(float)` / `repr(float)` (`PyOS_double_to_string(v, 'r', 0,
/// Py_DTSF_ADD_DOT_0)`): shortest digits, exponent form when
/// `decpt <= -4 || decpt > 16`, else positional with a trailing `.0` when
/// nothing follows the point. The sign comes from the sign *bit* so
/// `str(-0.0)` is "-0.0".
fn py_str_f64(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits.clone()
        };
        let exp = decpt - 1;
        let (esign, eabs) = if exp < 0 {
            ("-", -(exp as i64))
        } else {
            ("+", exp as i64)
        };
        return format!("{}{}e{}{:0>2}", sign, mantissa, esign, eabs);
    }
    if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndig {
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndig) as usize))
    } else {
        format!(
            "{}{}.{}",
            sign,
            &digits[..decpt as usize],
            &digits[decpt as usize..]
        )
    }
}

/// CPython's `Decimal.__str__` (`to-scientific-string`), ported from
/// `_pydecimal.Decimal.__str__`. `BigDecimal`'s own `Display` disagrees on
/// `1E+2`/`1.10`/`0.0`/negative-zero, so digits + exponent are read off
/// `as_bigint_and_exponent()` and reassembled by CPython's rule.
fn py_str_decimal(value: &BigDecimal) -> String {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    let exp = -scale;
    let sign = if coefficient.is_negative() { "-" } else { "" };
    let int_digits = coefficient.abs().to_string();
    let ndig = int_digits.len() as i64;

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
        (
            int_digits[..dotplace as usize].to_string(),
            format!(".{}", &int_digits[dotplace as usize..]),
        )
    };

    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

/// Python's `int(s)` on a full string. `ValueError` for a non-integer literal
/// (e.g. the `"1e+16"` / `"1E+2"` that exponent-form reprs produce), quoting
/// the whole offending string exactly as CPython does.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `int(ch)` for a single fraction character. `ValueError` on a
/// non-digit (an `'e'`/`'E'`/`'+'` from a scientific-coefficient repr), quoting
/// that one character.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10).map(|d| d as usize).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

/// `Num2Word_EU.to_cardinal` operating on `str(number)`, the form Python's own
/// code works in. Reproduces the exact body: the sign is peeled off the string
/// (`negword` carries its trailing space, so it concatenates raw), the integer
/// part goes through `_int_to_word(int(left))`, and each fraction *character*
/// is rendered by `_int_to_word(int(d))` — so a `0` digit becomes "zero", not
/// "". No trailing `.strip()`: Python returns `ret` verbatim.
fn cardinal_from_repr(n: &str) -> Result<String> {
    // n = str(number).strip()
    let n = n.trim();

    // if n.startswith("-"): n = n[1:]; ret = negword  else: ret = ""
    let (mut ret, n) = match n.strip_prefix('-') {
        Some(rest) => (NEGWORD.to_string(), rest),
        None => (String::new(), n),
    };

    match n.split_once('.') {
        Some((left, right)) => {
            // ret += _int_to_word(int(left)) + " " + pointword + " "
            ret.push_str(&checked_int_to_word(&py_int(left)?)?);
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // ret += " ".join(_int_to_word(int(d)) for d in right)
            let mut first = true;
            for d in right.chars() {
                if !first {
                    ret.push(' ');
                }
                first = false;
                let digit = py_int_digit(d)?;
                ret.push_str(&checked_int_to_word(&BigInt::from(digit))?);
            }
            Ok(ret)
        }
        // return ret + _int_to_word(int(n))
        None => {
            ret.push_str(&checked_int_to_word(&py_int(n)?)?);
            Ok(ret)
        }
    }
}

pub struct LangEu {
    /// `Num2Word_EU.CURRENCY_FORMS`.
    ///
    /// Built once in [`LangEu::new`] and stored, never per call: the binding
    /// holds the converter in a `OnceLock`, so this table is constructed exactly
    /// once per process.
    ///
    /// Both sides of every pair are the same word — Basque does not inflect
    /// these for number — but the 2-tuple arity is Python's and is kept as-is.
    /// Nothing here indexes past `[0]` on the currency path; `to_cheque` takes
    /// `cr1[-1]`, which for these entries is the same string as `cr1[0]`.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangEu {
    pub fn new() -> Self {
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("EUR", &["euro", "euro"][..], &["zentimo", "zentimo"][..]),
            ("USD", &["dolar", "dolar"][..], &["zentabo", "zentabo"][..]),
            ("GBP", &["libera", "libera"][..], &["penike", "penike"][..]),
        ]
        .into_iter()
        .map(|(k, u, s)| (k, CurrencyForms::new(u, s)))
        .collect();
        LangEu { currency_forms }
    }
}

impl Default for LangEu {
    fn default() -> Self {
        LangEu::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// "mila milioi" (10^9), so from 10^12 the module would stack "mila" ("mila
/// mila milioi").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl LangTrait for LangEu {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }


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
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "EUR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " eta "
    }

    /// `setup()` sets `self.negword = "minus "`, trailing space included.
    fn negword(&self) -> &str {
        NEGWORD
    }

    /// `setup()` sets `self.pointword = "koma"`. Used by the float / Decimal
    /// path in [`LangEu::to_cardinal_float`] via [`cardinal_from_repr`].
    fn pointword(&self) -> &str {
        "koma"
    }

    /// `Num2Word_EU.to_cardinal`.
    ///
    /// Python works on `str(number).strip()` and hand-strips a leading "-",
    /// then re-parses with `int()`. For integer input that is exactly a sign
    /// split, so we branch on the sign directly. The `"." in n` float branch
    /// is out of scope.
    ///
    /// `negword` is concatenated **raw** (it already carries its trailing
    /// space) — this does *not* route through `base.py`'s `to_cardinal`, so no
    /// trim/re-space happens.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("{}{}", NEGWORD, checked_int_to_word(&value.abs())?))
        } else {
            Ok(checked_int_to_word(value)?)
        }
    }

    /// `Num2Word_EU.to_cardinal` reached with a float / Decimal, i.e. its
    /// `if "." in n` branch. EU overrides `to_cardinal` outright, so
    /// `Num2Word_Base.to_cardinal_float` (float2tuple / `10**precision` /
    /// `< 0.01` rescue) is never inherited: the value's `str()` is consumed
    /// directly by [`cardinal_from_repr`]. `precision_override` and
    /// `FloatValue::precision` are unread — EU's `to_cardinal` takes no
    /// `precision` argument.
    ///
    /// The Float and Decimal arms stay distinct (issue #603): `str(Decimal)`
    /// keeps every written digit, so `Decimal("1.10")` ends in "zero" where the
    /// float `1.10` could not. One input this cannot reproduce is a
    /// negative-zero **Decimal**: `str(Decimal("-0.0"))` is "-0.0" (Python:
    /// "minus zero koma zero"), but `BigInt` has no negative zero so the sign is
    /// already gone at the `FloatValue` boundary — the same blind spot
    /// `FloatValue::is_negative` and `lang_ceb.rs` carry. A `-0.0` *float* keeps
    /// its sign bit and renders "minus zero koma zero" correctly.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        cardinal_from_repr(&n)
    }

    /// `Num2Word_EU.to_ordinal`: `cardinal + "garren"`.
    ///
    /// Bug 5: Python branches on `cardinal.endswith("t")` but both arms are
    /// identical, so the condition is dead. Bug 6: negatives are not rejected.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}garren", self.to_cardinal(value)?))
    }

    /// `Num2Word_EU.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Operates on the *original* input, sign included: -1 == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `Num2Word_EU.to_year`: `to_cardinal(val) + ". urtea"`.
    ///
    /// Bug 7: `longval` is accepted and ignored, and the ordinal marker "."
    /// is glued onto a cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}. urtea", self.to_cardinal(value)?))
    }

    // ---- float / Decimal entry routing --------------------------------
    //
    // `to_ordinal` / `to_ordinal_num` / `to_year` have no type guard in
    // Python, so a float or Decimal flows through them exactly as an int
    // does: the suffix wraps the *full* decimal phrase (or, for
    // `to_ordinal_num`, the raw `str(number)`).

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "garren"` —
    /// "bost koma zerogarren" for 5.0 (the dead `endswith("t")` branch picks
    /// the same suffix either way); the exponential-form ValueError from the
    /// cardinal propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}garren", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — no `int()`, so
    /// it succeeds where the other modes raise ("1e+16.") and "-0.0" keeps
    /// its textual minus ("-0.0."). `repr_str` is Python's `str(number)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_cardinal(val) + ". urtea"` — the full
    /// float cardinal plus the year marker, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}. urtea", self.cardinal_float_entry(value, None)?))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which EU does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError; the
    /// binding's shared Inf sentinel would otherwise raise OverflowError.
    /// (NaN needs no interception: the binding's ValueError already matches
    /// `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ------------------------------------------------------

    /// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError.
    fn lang_name(&self) -> &str {
        "Num2Word_EU"
    }

    /// `CURRENCY_FORMS[code]` — a *strict* lookup, which is what `to_cheque`
    /// wants (`KeyError` -> `NotImplementedError`).
    ///
    /// [`LangEu::to_currency`] deliberately does **not** route through this: it
    /// does `.get(currency, CURRENCY_FORMS["EUR"])` and never raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EU.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True,
    ///                 separator=" eta ", adjective=False):
    ///     try:
    ///         left, right, is_negative = self.parse_currency(val)
    ///     except AttributeError:
    ///         is_negative = False
    ///         if val < 0:
    ///             is_negative = True
    ///             val = abs(val)
    ///         left, right = self._split_currency(val)
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["EUR"])
    ///     left_str = self._int_to_word(int(left))
    ///     cents_str = self._int_to_word(int(right)) if cents and right else ""
    ///     result = left_str + " " + cr1[0]
    ///     if cents_str:
    ///         result += separator + cents_str + " " + cr2[0]
    ///     return self.negword + result if is_negative else result
    /// ```
    ///
    /// This override bypasses `Num2Word_Base.to_currency` completely, so none
    /// of [`crate::currency::default_to_currency`]'s machinery runs: no
    /// `parse_currency_parts`, no ROUND_HALF_UP quantize, no `pluralize`, no
    /// `CURRENCY_PRECISION`, no NotImplementedError. Hence the hand-rolled body
    /// rather than a delegation.
    ///
    /// # Faithfully reproduced Python bugs
    ///
    /// 8. **The `try` block is dead code.** `parse_currency` is defined nowhere
    ///    in the package (only `parse_currency_parts`, a module-level function
    ///    in `currency.py`, exists), so `self.parse_currency` raises
    ///    `AttributeError` on *attribute lookup*, every single call, and the
    ///    `except AttributeError` fallback is the only path that ever executes.
    ///    The author evidently meant `parse_currency_parts`. Ported as the
    ///    fallback alone — the try arm is unreachable and has no observable
    ///    effect. (`lang_AS`, `lang_BA` and `lang_BO` carry the same copy-paste.)
    /// 9. **An unknown currency code silently becomes euros.** `.get(currency,
    ///    self.CURRENCY_FORMS["EUR"])` means `to_currency(1, currency="JPY")`
    ///    renders "bat euro", not a NotImplementedError — the *opposite* of
    ///    every other module and of this class's own inherited `to_cheque`,
    ///    which raises for the same code. The corpus pins it: all 6 unknown
    ///    codes (JPY/KWD/BHD/INR/CNY/CHF) return euro strings under
    ///    `to="currency"` and raise under `to="cheque"`.
    /// 10. **`CURRENCY_PRECISION` is ignored.** `_split_currency` hard-codes two
    ///    fractional digits, so KWD/BHD (1000 subunits) split at 2 digits —
    ///    `12.34` KWD is 34 subunits, not 340 — and JPY (which has no subunit at
    ///    all, and which `Num2Word_Base.to_currency` would have rounded to a
    ///    whole "hamabi euro") still grows a cents segment.
    /// 11. **A whole float drops its cents.** `cents_str` is gated on `right`
    ///    being *truthy*, so `1.0` yields "bat euro" where
    ///    `Num2Word_Base.to_currency` would give "bat euro, zero zentimo". The
    ///    int/float distinction the trait carefully preserves is therefore
    ///    invisible in EU's *output* — both arms reach `right == 0` — but the
    ///    two are still routed separately below, because they reach it by
    ///    different code and diverge at the extremes (`str(10**20)` splits fine
    ///    as an int; `1e+20` as a float is a `ValueError` — see `concerns`).
    /// 12. **`adjective` is accepted and never read.** The class defines no
    ///    `CURRENCY_ADJECTIVES` either, so the flag is inert.
    /// 13. **`negword` is concatenated raw**, keeping its trailing space, and is
    ///    prepended to the *whole* phrase after the cents segment is appended.
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
        // Bug 12: Python takes `adjective` and never looks at it.
        let _ = adjective;
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Bug 8: only the `except AttributeError` arm is reachable.
        let (left, right, is_negative) = match val {
            // `str(int)` never contains ".", so `_split_currency` returns
            // `(the int, 0)` — the cents segment can never appear for an int.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero(), v.is_negative()),
            CurrencyValue::Decimal { value: d, .. } => {
                // Python: `if val < 0: is_negative = True; val = abs(val)`.
                // `-0.0 < 0` is False, and `BigDecimal("-0.0")` is `Zero`, so
                // both sides agree that negative zero is not negative.
                let is_negative = d.is_negative();
                let (left, right) = split_currency(&d.abs());
                (left, right, is_negative)
            }
        };

        // Bug 9: unknown code -> euros. Python evaluates the
        // `self.CURRENCY_FORMS["EUR"]` default eagerly on every call; it is a
        // class-level literal containing "EUR", so it cannot KeyError.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or_else(|| &self.currency_forms["EUR"]);

        let left_str = checked_int_to_word(&left)?;
        // `cents and right` — `right == 0` is falsy in Python (bug 11).
        let cents_str = if cents && !right.is_zero() {
            checked_int_to_word(&right)?
        } else {
            String::new()
        };

        let mut result = format!("{} {}", left_str, forms.unit[0]);
        if !cents_str.is_empty() {
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        // Bug 13: raw `negword`, trailing space included, around the whole
        // phrase.
        if is_negative {
            Ok(format!("{}{}", NEGWORD, result))
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;

    fn ff(v: f64, p: u32) -> String {
        let l = LangEu::new();
        match l.to_cardinal_float(&FloatValue::Float { value: v, precision: p }, None) {
            Ok(s) => s,
            Err(e) => format!("ERR:{:?}", e),
        }
    }
    fn dd(s: &str) -> String {
        let l = LangEu::new();
        let bd = BigDecimal::from_str(s).unwrap();
        let p = bd.as_bigint_and_exponent().1.max(0) as u32;
        match l.to_cardinal_float(&FloatValue::Decimal { value: bd, precision: p }, None) {
            Ok(s) => s,
            Err(e) => format!("ERR:{:?}", e),
        }
    }

    #[test]
    fn corpus_float() {
        assert_eq!(ff(0.0, 1), "zero koma zero");
        assert_eq!(ff(0.5, 1), "zero koma bost");
        assert_eq!(ff(1.0, 1), "bat koma zero");
        assert_eq!(ff(1.5, 1), "bat koma bost");
        assert_eq!(ff(2.25, 2), "bi koma bi bost");
        assert_eq!(ff(3.14, 2), "hiru koma bat lau");
        assert_eq!(ff(0.01, 2), "zero koma zero bat");
        assert_eq!(ff(0.1, 1), "zero koma bat");
        assert_eq!(ff(0.99, 2), "zero koma bederatzi bederatzi");
        assert_eq!(ff(1.01, 2), "bat koma zero bat");
        assert_eq!(ff(12.34, 2), "hamabi koma hiru lau");
        assert_eq!(ff(99.99, 2), "laurogeita hamarta bederatzi koma bederatzi bederatzi");
        assert_eq!(ff(100.5, 1), "ehun koma bost");
        assert_eq!(ff(1234.56, 2), "mila biehun eta hogeita hamarta lau koma bost sei");
        assert_eq!(ff(-0.5, 1), "minus zero koma bost");
        assert_eq!(ff(-1.5, 1), "minus bat koma bost");
        assert_eq!(ff(-12.34, 2), "minus hamabi koma hiru lau");
        assert_eq!(ff(1.005, 3), "bat koma zero zero bost");
        assert_eq!(ff(2.675, 3), "bi koma sei zazpi bost");
        assert_eq!(ff(1000000.5, 1), "milioi bat koma bost");
    }

    #[test]
    fn live_extra() {
        // Billions branch, bug 3: no `== 1` special case.
        assert_eq!(ff(1000000000.5, 1), "bat mila milioi koma bost");
        assert_eq!(ff(0.25, 2), "zero koma bi bost");
        assert_eq!(ff(10.101, 3), "hamar koma bat zero bat");
        assert_eq!(ff(55.5, 1), "berrogeita hamarta bost koma bost");
        assert_eq!(ff(-1234.5, 1), "minus mila biehun eta hogeita hamarta lau koma bost");
        // -0.0 float keeps its sign bit -> "minus zero koma zero".
        assert_eq!(ff(-0.0, 1), "minus zero koma zero");
        // Decimal scientific form with no dot: int("1E+2") -> ValueError.
        assert_eq!(dd("1E+2"), "ERR:Value(\"invalid literal for int() with base 10: '1E+2'\")");
    }

    #[test]
    fn corpus_dec() {
        assert_eq!(dd("0.01"), "zero koma zero bat");
        assert_eq!(dd("1.10"), "bat koma bat zero");
        assert_eq!(dd("12.345"), "hamabi koma hiru lau bost");
        assert_eq!(dd("0.001"), "zero koma zero zero bat");
        assert_eq!(
            dd("98746251329.99"),
            "laurogeita hamarta zortzi mila milioi zazpiehun eta berrogeita sei milioi biehun eta berrogeita hamarta bat mila hiruehun eta hogeita bederatzi koma bederatzi bederatzi"
        );
    }
}
