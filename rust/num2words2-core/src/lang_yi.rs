//! Port of `lang_YI.py` (Yiddish).
//!
//! Shape: **self-contained**. `Num2Word_YI` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives a recursive `_int_to_word`. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check at all** — `_int_to_word` falls back to `str(number)`
//! above 10**9 instead of raising (see bug 3 below).
//!
//! Nothing in the four in-scope modes can raise: `to_cardinal` strips the sign
//! from the *string* before `int()`, every table index is range-guarded by the
//! `< 10` / `< 100` / `< 1000` cascade, and the `else` arm swallows everything
//! else. So this port returns `Ok(..)` unconditionally and never constructs an
//! `N2WError`.
//!
//! Inherited from `Num2Word_Base` (YI overrides `to_year` but to the same
//! effect, and does not touch `verify_ordinal`):
//!   * `to_year(val, longval=True) -> self.to_cardinal(val)` — the `longval`
//!     parameter is accepted and ignored, so years are plain cardinals with no
//!     century splitting: `to_year(1900)` == `to_cardinal(1900)` ==
//!     "איינס טויזנט נײַן הונדערט", *not* "nineteen hundred".
//!   * `verify_ordinal` is **never called** — `to_ordinal` accepts negatives
//!     and simply suffixes the (sign-bearing) cardinal, so `to_ordinal(-1)` ==
//!     "מינוס איינס-טער" rather than raising TypeError.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below looks wrong and is exactly
//! what Python emits — all verified against the frozen corpus:
//!
//! 1. **Yiddish has no teens and no "un-" tens compounding.** `_int_to_word`
//!    handles 10..99 as `tens[n // 10] + " " + ones[n % 10]`, unconditionally.
//!    There is no `low_numwords`-style 11..19 table, so 11 renders as
//!    "צען איינס" (literally "ten one") and 12 as "צען צוויי". Real Yiddish
//!    would use elf/tsvelf and put the unit first ("ein un tsvantsik"); the
//!    library does neither. 21 == "צוואַנציק איינס".
//! 2. **Hundreds/thousands/millions take no multiplier elision.** The
//!    `< 1000` arm is `self.ones[hundreds_val] + " " + self.hundred` with no
//!    `if hundreds_val > 1` guard, so 100 == "איינס הונדערט" ("one hundred"),
//!    and by recursion 1000 == "איינס טויזנט" and 10**6 == "איינס מיליאָן".
//! 3. **Above 10**9 the converter gives up and returns the digits.** The final
//!    `else` arm of `_int_to_word` is literally `return str(number)`, so
//!    `to_cardinal(10**9)` == "1000000000" — a *digit string*, not words. It
//!    does not raise OverflowError; there is no `MAXVAL`. `to_ordinal(10**9)`
//!    therefore yields "1000000000-טער". Modelled by [`LangYi::int_to_word`].
//! 4. **Zero (fixed, gladiaio/num2words2#154).** Python's `_int_to_word`
//!    opens with `return self.ones[0] if self.ones[0] else "zero"` — `""` is
//!    falsy, so Python always answered with the *English* "zero". This port
//!    says the Yiddish "נול": `to_cardinal(0)` == "נול" and `to_ordinal(0)` ==
//!    "נול-טער". See [`ZERO`].
//! 5. **`negword` and `pointword` (fixed, #154).** Python has the English
//!    "minus " and "point"; this port uses "מינוס " and "פּונקט" (YIVO
//!    spelling, matching the pointed forms in the tables). `to_cardinal`
//!    prepends the negword raw rather than via `parse_minus`:
//!    `to_cardinal(-100)` == "מינוס איינס הונדערט".
//! 6. **`to_ordinal` is cardinal + a hyphenated suffix, with no agreement and
//!    no last-word inflection.** `return cardinal + "-טער"` — the suffix is
//!    glued to whatever came out, including the digit-string fallback (bug 3)
//!    and the zero word (bug 4).
//!
//! # Dead code in the source, reproduced anyway
//!
//! `_int_to_word`'s `if number < 0` arm is unreachable from all four integer
//! modes: `to_cardinal` slices the "-" off the *string* (`n = n[1:]`) before
//! `int(n)`, so `_int_to_word` only ever sees a non-negative value. It is
//! unreachable from `to_currency` too, which takes `abs(val)` before splitting.
//! It is kept in [`LangYi::int_to_word`] for fidelity — with the same ordering
//! quirk that `number == 0` is tested *before* `number < 0`.
//!
//! # Currency
//!
//! `Num2Word_YI` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base`'s version. It does not call `pluralize`, does
//! not consult `CURRENCY_PRECISION`, does not use `parse_currency_parts`, and
//! ignores its own `adjective` parameter. So `currency::default_to_currency` is
//! bypassed entirely and the divisor is hard-wired to 100 for every code.
//!
//! `to_cheque` is *not* overridden, so `Num2Word_Base.to_cheque` runs — which
//! means the two entry points disagree about unknown currency codes:
//!
//! | code       | `to_currency`            | `to_cheque`          |
//! |------------|--------------------------|----------------------|
//! | EUR / USD  | its own forms            | its own forms        |
//! | anything else | silently uses **EUR** | NotImplementedError  |
//!
//! That asymmetry is bug 7 below and is exactly what the corpus records.
//!
//! ## More faithfully reproduced Python bugs
//!
//! 7. **`to_currency` never raises for an unknown code.** The lookup is
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a *default*, not a `[]`. The default is the first value in
//!    a dict written `{"EUR": ..., "USD": ...}`, so every unrecognised code
//!    (GBP, JPY, KWD, INR, ...) renders in euros: `to_currency(1, "GBP")` ==
//!    "איינס אייראָ". Only `to_cheque`, inherited from the base, raises
//!    `NotImplementedError`. See [`FALLBACK_CODE`].
//! 8. **Cents are truncated to two digits and right-padded, never rounded.**
//!    `int(parts[1][:2].ljust(2, "0"))` slices the *decimal string*: `0.5`
//!    means **50** cents (not 5), `1.239` gives 23 cents (not 24), and `1.005`
//!    gives 0. Modelled arithmetically as `floor(frac * 100)`.
//! 9. **A zero cents value suppresses the whole segment**, because the guard is
//!    the truthiness test `if cents and right:`. So the float `1.0` prints
//!    "איינס אייראָ" with no cents — the `isinstance(val, int)` split that
//!    `Num2Word_Base.to_currency` relies on is absent here, and `1` and `1.0`
//!    coincidentally agree. `1.005` loses its cents the same way (bug 8).
//! 10. **Currency mixes scripts and pluralizes in English.** USD carries the
//!     ASCII `("dollar", "dollars")`/`("cent", "cents")` rather than Yiddish, so
//!     `to_currency(2, "USD")` == "צוויי dollars", and a negative prepends the
//!     `negword`: "מינוס צען צוויי dollars דרײַסיק פיר cents".
//! 11. **EUR's singular and plural are the same string** (`("אייראָ", "אייראָ")`,
//!     `("צענט", "צענט")`), so the `left != 1` test is a no-op for EUR — and for
//!     every code that falls back to it (bug 7).
//! 12. **`to_currency(0.0..1)` says "נול".** `left == 0` routes through
//!     `_int_to_word(0)` (bug 4): `to_currency(0.01)` == "נול אייראָ איינס
//!     צענט".
//!
//! ## Unreachable from Rust: the sci-notation `ValueError`
//!
//! `to_currency` does `int(str(abs(val)).split(".")[0])`. Python's `repr` of a
//! float switches to exponent form at `1e16` and below `1e-4`, and `int()`
//! rejects that spelling — so `to_currency(1e20)` raises **ValueError**
//! (`invalid literal for int() with base 10: '1e+20'`), as does `1e-5`. The
//! shim hands this core `str(value)` already parsed into a `BigDecimal`, so the
//! "1e+20" *spelling* — the sole cause of the crash — is gone before the port
//! can see it, and these inputs render normally here instead of raising. No
//! corpus row covers it; see the reported concerns.
//!
//! # The float / Decimal cardinal path
//!
//! `Num2Word_YI` does **not** define `to_cardinal_float`; it overrides
//! `to_cardinal` and handles non-integers *inline*, working entirely on the
//! *string* `str(number)`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"):
//!     n = n[1:]
//!     ret = self.negword
//! else:
//!     ret = ""
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     for digit in right:
//!         ret += self._int_to_word(int(digit)) + " "
//!     return ret.strip()
//! else:
//!     return (ret + self._int_to_word(int(n))).strip()
//! ```
//!
//! So the fraction is the literal digits of the repr, one word per character —
//! no `float2tuple`, no `10**precision`, no `< 0.01` rescue, no rounding and no
//! padding. The port therefore reproduces `str()` exactly ([`py_str_f64`] /
//! [`py_str_decimal`]) and runs the same string body ([`LangYi::cardinal_from_repr`]).
//! Because the base float2tuple is *not* on this path, [`to_cardinal_float`]'s
//! `precision_override` (the `precision=` kwarg) is **inert** — `to_cardinal`
//! has no such parameter and never reads `self.precision`, verified against the
//! live interpreter.
//!
//! [`to_cardinal_float`]: LangYi
//!
//! Three consequences, all matching the live interpreter and reproduced here:
//!
//! * **The f64 artefacts come from `repr`, not arithmetic.** `2.675` prints
//!   "זעקס זיבן פינף" (6 7 5) because `str(2.675) == "2.675"` (shortest
//!   round-trip), and `1.005` prints "נול נול פינף" because
//!   `str(1.005) == "1.005"`. There is no `674.9999…`→`675` heuristic here; the
//!   repr already carries the intended digits, so `py_str_f64` must be a
//!   byte-exact `repr` — including CPython's tie-to-**even** last digit, which
//!   Rust's shortest `{:e}` alone gets wrong (see [`shortest_repr_digits`]).
//! * **A Decimal keeps every written digit.** `str(Decimal("1.10")) == "1.10"`,
//!   so the trailing "נול" appears — something the float `1.1` could never
//!   express (`str(1.1) == "1.1"`). Handled by [`py_str_decimal`], not
//!   `BigDecimal`'s own `Display`, which normalises differently.
//! * **An exponent-form repr reaches `int()` and raises `ValueError`.** Unlike
//!   the `to_currency` case above — where the shim pre-parses away the spelling
//!   — the float path carries the raw `f64`, so `py_str_f64` reconstructs the
//!   real repr and the crash *is* reproducible: `to_cardinal_float(1e16)` and
//!   `(1e-5)` raise `ValueError: invalid literal for int() with base 10:
//!   '1e+16'` / `'1e-05'`, and `1.5e20` (which keeps a ".") dies on the single
//!   char `'e'` after already emitting the `5`. See bug 13 below.
//!
//! ## One more faithfully reproduced Python bug
//!
//! 13. **`_int_to_word(int(left))` inherits the `>= 10**9` digit-string
//!     fallback (bug 3).** The integer part of a large value comes back as bare
//!     digits: `to_cardinal_float(Decimal("98746251323029.99"))` ==
//!     "98746251323029 פּונקט נײַן נײַן", not words.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `_int_to_word(0)`. Python computes `self.ones[0] if self.ones[0] else "zero"`
/// and `self.ones[0]` is `""` — falsy — so Python's English literal is the
/// only reachable result; this port says "נול". See module bug 4.
const ZERO: &str = "נול";

/// `self.negword`, with a trailing space, and used verbatim (not via
/// `parse_minus`, which would `.strip()` it). See module bug 5.
const NEGWORD: &str = "מינוס ";

/// `self.pointword`. Python's English "point"; Yiddish here. See bug 5.
const POINTWORD: &str = "פּונקט";

/// `self.ones`; index 0 is `""` in Python and is never reached as a word —
/// the `number == 0` guard fires first. See module bug 4.
const ONES: [&str; 10] = [
    "",
    "איינס",
    "צוויי",
    "דרײַ",
    "פיר",
    "פינף",
    "זעקס",
    "זיבן",
    "אַכט",
    "נײַן",
];

/// `self.tens`; index 0 is `""` and unreachable (`number < 10` handles it).
/// Index 1 is "צען" (ten) — Yiddish here has no separate teens table, so this
/// entry doubles as the tens-digit word for 10..19. See module bug 1.
const TENS: [&str; 10] = [
    "",
    "צען",
    "צוואַנציק",
    "דרײַסיק",
    "פערציק",
    "פופציק",
    "זעכציק",
    "זיבעציק",
    "אַכציק",
    "נײַנציק",
];

/// `self.hundred`.
const HUNDRED: &str = "הונדערט";
/// `self.thousand`.
const THOUSAND: &str = "טויזנט";
/// `self.million`.
const MILLION: &str = "מיליאָן";

/// `to_ordinal`'s suffix, glued on with a literal hyphen. See module bug 6.
const ORDINAL_SUFFIX: &str = "-טער";

/// The ceiling of `_int_to_word`'s word-producing cascade. At or above this,
/// Python returns `str(number)`. See module bug 3.
const FALLBACK_LIMIT: u64 = 1_000_000_000;

/// `self.__class__.__name__`, for the NotImplementedError `to_cheque` raises.
const LANG_NAME: &str = "Num2Word_YI";

/// `CURRENCY_FORMS["EUR"]`'s unit word. Python stores it in *both* the singular
/// and the plural slot, so `left != 1` cannot change the output. See bug 11.
const EURO: &str = "אייראָ";
/// `CURRENCY_FORMS["EUR"]`'s subunit word — likewise identical in both slots.
const EURO_CENT: &str = "צענט";

/// The code that `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// `CURRENCY_FORMS` is the dict literal `{"EUR": ..., "USD": ...}` and Python
/// dicts preserve insertion order, so the first value is always EUR's. That
/// value is `to_currency`'s `.get(currency, <default>)` fallback, which is why
/// an unknown code renders in euros rather than raising. See module bug 7.
const FALLBACK_CODE: &str = "EUR";

/// `Num2Word_YI.to_currency`'s declared default `separator=" "`, which
/// *overrides* `Num2Word_Base.to_currency`'s `separator=","`.
///
/// Neither caller into this core can express "the caller omitted `separator`":
/// `__init__.py` sends `kwargs.get("separator", ",")` and `bench/diff_test.py`
/// hardcodes `","`. Both substitute the **base** default, so YI's own default
/// is erased before the value reaches Rust. The corpus was frozen from the pure
/// Python path, where the omitted kwarg correctly resolves to `" "`:
///
/// ```text
/// {"lang": "yi", "to": "currency:EUR", "arg": "12.34",
///  "out": "צען צוויי אייראָ דרײַסיק פיר צענט"}
/// ```
///
/// — a space, not a comma. So [`separator_for`] reads an incoming `","` as
/// "unsupplied" and restores YI's default. See the reported concerns: this is a
/// boundary defect shared by every language whose `to_currency` overrides the
/// separator default (DE `" und"`, ES `" y"`, BG `" и"`, ~50 more).
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s `separator=","`, which both call sites send
/// verbatim when the caller supplies nothing. See [`DEFAULT_SEPARATOR`].
const BASE_SEPARATOR_DEFAULT: &str = ",";

/// Resolve the separator YI's Python signature would have applied.
///
/// An explicit `separator=","` is byte-identical to an omitted one by the time
/// it arrives here, so it renders as `" "` — the one input this cannot get
/// right, and one no implementation behind this ABI could. Every other explicit
/// separator passes through untouched and stays faithful.
fn separator_for(passed: &str) -> &str {
    if passed == BASE_SEPARATOR_DEFAULT {
        DEFAULT_SEPARATOR
    } else {
        passed
    }
}

// ---- the str()-based float path -------------------------------------------
//
// `Num2Word_YI.to_cardinal` runs on `str(number)`, so the float path is pure
// string work. These helpers reproduce CPython's `str()` byte-for-byte and its
// `int()` failure modes; they are numeric formatters and carry no Yiddish word,
// so they mirror the verified `lang_ceb` helpers exactly.

/// Python's `int(s)` for a whole token. The error message quotes the offending
/// literal exactly as CPython does, so an exponent-form repr (`"1e+16"`,
/// `"1E-7"`) surfaces the identical `ValueError` text.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `int(ch)` for a *single* character, as the fraction loop calls it.
///
/// The message quotes the one offending character, matching CPython — that is
/// why `1.5e+20` reports `'e'` (after already emitting the `5`) rather than
/// `'5e+20'`. `char::to_digit(10)` is ASCII-only, but no `str(float)` /
/// `str(Decimal)` can emit a non-ASCII digit, so the two agree everywhere the
/// fraction loop can actually reach.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10).map(|d| d as usize).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

/// The shortest round-trip decimal digits of `a` (finite, non-negative), plus
/// CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// Rust's `{:e}` and CPython's `repr` are both "shortest string that reads back
/// as the same double" and agree on the digit count always and the digits
/// themselves almost always — they part company only on an **exact tie**, where
/// CPython breaks to **even** and Rust's shortest `{:e}` breaks **away from
/// zero** (e.g. the double `670352580196876.25` reprs `...876.2`, not `.3`).
/// Since YI's whole fraction *is* the repr string, a wrong last digit is a
/// wrong word. Rust's fixed-precision `{:.*e}` rounds half-to-even, so
/// re-emitting the same significant-digit count applies CPython's tie rule; it
/// is kept only if it still round-trips, otherwise we fall back to the shortest
/// digits rather than emit a string that reads back as a different double.
fn shortest_repr_digits(a: f64) -> (String, i32) {
    let split = |s: &str| -> (String, i32) {
        let (mant, exp) = s.split_once('e').expect("{:e} always emits an 'e'");
        (
            mant.chars().filter(|c| *c != '.').collect(),
            // `{:e}` normalises to exactly one digit before the point, so the
            // decimal point sits one place left of where CPython counts it.
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

/// CPython's `str(float)` / `repr(float)` — where YI's float path begins.
///
/// `PyOS_double_to_string(v, 'r', 0, Py_DTSF_ADD_DOT_0)`: shortest round-trip
/// digits, then switch to exponent form when `decpt <= -4 || decpt > 16`
/// (`str(1e15) == "1000000000000000.0"` but `str(1e16) == "1e+16"`;
/// `str(0.0001) == "0.0001"` but `str(1e-05) == "1e-05"`), formatting the
/// exponent `%+.02d`; otherwise print positionally and append `.0` if nothing
/// follows the point (the reason `1.0` is "איינס פּונקט נול", not "איינס").
fn py_str_f64(v: f64) -> String {
    // Unreachable from the shim (which derives `precision` from a finite
    // Decimal repr), but handled so this is a faithful `str()` rather than a
    // faithful-in-context one; `cardinal_from_repr` would then raise on
    // `int("inf")`/`int("nan")`, exactly as Python does.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    // Sign from the sign *bit*, not `v < 0.0`, so `str(-0.0) == "-0.0"` and the
    // leading-"-" branch fires: -0.0 renders "מינוס נול פּונקט נול".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            // No ADD_DOT_0 in exponent form: `str(1e16)` is "1e+16".
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
        // "0." + leading zeros + digits: 0.5 -> "0.5", 1e-4 -> "0.0001".
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndig {
        // Nothing after the point, so Py_DTSF_ADD_DOT_0 supplies ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndig) as usize))
    } else {
        format!("{}{}.{}", sign, &digits[..decpt as usize], &digits[decpt as usize..])
    }
}

/// CPython's `Decimal.__str__` (`to-scientific-string`), ported from
/// `_pydecimal.Decimal.__str__`. `BigDecimal`'s own `Display` disagrees on
/// `1E+2` (would give `100`), `0.0` (would give `0`) and the exponent's case,
/// each of which would change YI's output or its exception, so the digits and
/// exponent are read off `as_bigint_and_exponent()` and reassembled by Python's
/// rule. `BigDecimal::from_str` keeps the written scale rather than normalising
/// (`"1.10"` stays coefficient 110 / scale 2), which is what makes the trailing
/// "נול" appear, and `(coefficient, -scale)` is exactly Python's `(_int, _exp)`.
fn py_str_decimal(value: &BigDecimal) -> String {
    // BigDecimal stores value = coefficient * 10^-scale, so Python's `_exp`
    // (which counts the other way) is the negated scale.
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

    // `['e', 'E'][context.capitals]`, and the default context has capitals=1.
    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

pub struct LangYi {
    /// `CURRENCY_FORMS`, built once. `get_lang` keeps the `LangYi` in a
    /// `OnceLock` and hands out `&'static` refs, so this table is constructed
    /// exactly once per process rather than per call.
    forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the entry `to_currency` falls
    /// back to for an unknown code (module bug 7). Cloned out of `forms` rather
    /// than re-spelled so the two cannot drift apart.
    fallback_forms: CurrencyForms,
}

impl LangYi {
    pub fn new() -> Self {
        let mut forms = HashMap::new();
        // Both EUR slots hold the same word; the arity (2) still matters
        // because to_currency indexes cr1[0]/cr1[1] directly. See bug 11.
        forms.insert("EUR", CurrencyForms::new(&[EURO, EURO], &[EURO_CENT, EURO_CENT]));
        // USD is ASCII English, not Yiddish — see module bug 10.
        forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        let fallback_forms = forms[FALLBACK_CODE].clone();
        LangYi { forms, fallback_forms }
    }

    /// Python's `_int_to_word`.
    ///
    /// The `< 0` arm is dead from every in-scope caller (see module docs) but
    /// is reproduced, including the quirk that `== 0` is tested first.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO.to_string();
        }
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }
        // Python's final `else: return str(number)`. Hoisted above the u64
        // narrowing because it is what makes that narrowing sound.
        if number >= &BigInt::from(FALLBACK_LIMIT) {
            return number.to_string();
        }
        // Proven bounded: 0 < number < 10**9, so the value fits u64 and the
        // cascade below can use fixed-width arithmetic without truncating.
        let n = number
            .to_u64()
            .expect("0 < number < 10**9 always fits in u64");
        self.int_to_word_small(n)
    }

    /// The word-producing cascade of `_int_to_word`, for `0 < n < 10**9`.
    ///
    /// Recurses on remainders only (never on 0 — Python guards each with
    /// `if remainder:`), so `n == 0` cannot arrive here from `int_to_word`.
    /// The `n == 0` guard is kept anyway to mirror Python's own entry check.
    fn int_to_word_small(&self, n: u64) -> String {
        if n == 0 {
            return ZERO.to_string();
        }
        if n < 10 {
            ONES[n as usize].to_string()
        } else if n < 100 {
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                // No teens table: 10 => "צען", 20 => "צוואַנציק". Bug 1.
                TENS[tens_val].to_string()
            } else {
                // 11 => "צען איינס", 21 => "צוואַנציק איינס". Bug 1.
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            }
        } else if n < 1000 {
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            // Direct ONES index, not a recursive call — and no elision for
            // hundreds_val == 1, so 100 => "איינס הונדערט". Bug 2.
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            result
        } else if n < 1_000_000 {
            let thousands_val = n / 1000;
            let remainder = n % 1000;
            let mut result = format!("{} {}", self.int_to_word_small(thousands_val), THOUSAND);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            result
        } else {
            // n < 10**9 — the caller already handled the str(number) fallback.
            let millions_val = n / 1_000_000;
            let remainder = n % 1_000_000;
            let mut result = format!("{} {}", self.int_to_word_small(millions_val), MILLION);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            result
        }
    }

    /// Python's `Num2Word_YI.to_cardinal` operating on `str(number)` — the
    /// form its own code works in for a non-integer. `n` is already the repr
    /// ([`py_str_f64`] / [`py_str_decimal`] produced it):
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
    /// The sign is stripped from the *string* before the dot check, so both
    /// branches carry the raw `negword` ("מינוס ", not via `parse_minus`).
    /// `split(".", 1)` splits on the first dot only; `int(left)` / `int(digit)`
    /// raise `ValueError` on an exponent-form repr (`"1e+16"`) or a stray `'e'`.
    /// `int(left)` inherits `int_to_word`'s `>= 10**9` digit-string fallback
    /// (bug 13). `self.pointword` is used raw — no `title`, matching the inline
    /// body (unlike `Num2Word_Base.to_cardinal_float`, which title-cases it).
    fn cardinal_from_repr(&self, n: &str) -> Result<String> {
        // n = str(number).strip(). Python strips its own whitespace set and
        // Rust trims Unicode's; no repr contains either.
        let n = n.trim();

        // if n.startswith("-"): n = n[1:]; ret = self.negword; else: ret = ""
        let (n, ret) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD),
            None => (n, ""),
        };

        match n.split_once('.') {
            // if "." in n:
            Some((left, right)) => {
                // ret += int_to_word(int(left)) + " " + pointword + " "
                let mut out = String::from(ret);
                out.push_str(&self.int_to_word(&py_int(left)?));
                out.push(' ');
                out.push_str(self.pointword());
                out.push(' ');
                // for digit in right: ret += int_to_word(int(digit)) + " "
                for digit in right.chars() {
                    out.push_str(&self.int_to_word(&BigInt::from(py_int_digit(digit)?)));
                    out.push(' ');
                }
                // return ret.strip()
                Ok(out.trim().to_string())
            }
            // else: return (ret + int_to_word(int(n))).strip()
            None => Ok(format!("{}{}", ret, self.int_to_word(&py_int(n)?))
                .trim()
                .to_string()),
        }
    }
}

impl Default for LangYi {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangYi {

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

    /// `to_ordinal(float/Decimal)`. YI's `to_ordinal` is
    /// `self.to_cardinal(number) + "-טער"` for *every* input, so the float
    /// entry is the float cardinal plus the suffix. An exponent-form Decimal
    /// repr ("1E+2") still dies in `int()` with ValueError inside the
    /// cardinal, before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr the
    /// binding computed, dot appended, sign and exponent form included
    /// ("-0.0.", "1e+16.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside YI's `int("Infinity")` (`to_cardinal` reads
    /// `str(number)`, strips the sign, finds no "." and calls `int()`).
    /// The binding otherwise hard-codes `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any YI code runs, so the
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
        "EUR"
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

    // cards / maxval / merge stay at their trait defaults: Python never builds
    // self.cards for YI (no *_numwords attributes), so splitnum/clean/merge are
    // unreachable and MAXVAL does not exist. There is no overflow check.

    /// Python's `to_cardinal`, integer path.
    ///
    /// Python works on `str(number)`: it strips a leading "-" from the *string*
    /// and stashes `negword`, so `_int_to_word` only ever sees a non-negative
    /// value. The float branch (`"." in n`) is out of scope. The trailing
    /// `.strip()` is a no-op for integer input — `_int_to_word` never returns
    /// leading or trailing whitespace — but is reproduced verbatim.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Python's `to_cardinal`, float/Decimal path — the `"." in n` branch of
    /// the same method, reached when the dispatcher hands over a non-integer.
    /// There is no separate `to_cardinal_float` in `Num2Word_YI`; the body is
    /// the inline string walk ported in [`LangYi::cardinal_from_repr`], fed by
    /// an exact `str(number)` (see the module docs).
    ///
    /// `precision_override` (the `precision=` kwarg) is deliberately ignored:
    /// `num2words(..., precision=N)` sets `converter.precision`, but
    /// `Num2Word_YI.to_cardinal` takes no such parameter and never reads
    /// `self.precision`. Verified inert against the live interpreter —
    /// `precision=3` leaves `0.5` as "נול פּונקט פינף", not padded. The
    /// `FloatValue`'s own `precision` field is likewise unread: the repr
    /// string carries the digit count by construction.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number) — the float and Decimal arms are not interchangeable
        // (issue #603): `str` of a Decimal keeps every written digit, so
        // Decimal("1.10") ends in a trailing "נול" the float 1.1 could never
        // express, and Decimal("98746251323029.99") keeps its trillion-scale
        // integer part exact where a float cast would have rounded it.
        let n = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        self.cardinal_from_repr(&n)
    }

    /// Python's `to_ordinal`: `self.to_cardinal(number) + "-טער"`.
    ///
    /// No `verify_ordinal` call, so negatives pass through; no last-word
    /// inflection, so the suffix lands on the digit-string fallback and on the
    /// zero word too. Bugs 3, 4 and 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`.
    ///
    /// Overrides `Num2Word_Base.to_ordinal_num` (which returns the value
    /// unchanged), so the trait default is *not* correct here — the trailing
    /// dot is required. The sign is kept: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)`: `return self.to_cardinal(val)`.
    ///
    /// `longval` is accepted and ignored — no century splitting, no "hundred"
    /// idiom. Same result as the trait default; spelled out because YI does
    /// physically override the method.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `CURRENCY_FORMS[code]`, with **no** fallback.
    ///
    /// This is the strict lookup, and it is what the inherited `to_cheque`
    /// needs: `Num2Word_Base.to_cheque` does `self.CURRENCY_FORMS[currency]`
    /// and converts the `KeyError` into `NotImplementedError`, so returning
    /// `None` here makes `currency::default_to_cheque` raise the identical
    /// message for GBP/JPY/KWD/BHD/INR/CNY/CHF.
    ///
    /// `to_currency` deliberately does *not* route through this — it applies
    /// the EUR fallback instead. That asymmetry is module bug 7.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // currency_precision stays at the trait default of 100: Num2Word_YI never
    // defines CURRENCY_PRECISION and Num2Word_Base's is `{}`, so
    // `.get(code, 100)` is 100 for every code — including the ones that are
    // 3-decimal (KWD/BHD) or 0-decimal (JPY) elsewhere. to_cheque therefore
    // prints "56/100" for KWD, and to_currency ignores precision outright.
    //
    // currency_adjective stays at the default (None): CURRENCY_ADJECTIVES is
    // `{}`, and to_currency ignores its `adjective` argument regardless.
    //
    // pluralize stays at the default (raises NotImplementedError), exactly as
    // Num2Word_Base.pluralize does — Num2Word_YI never overrides it and
    // nothing reachable calls it: to_currency inlines its own
    // `cr1[1] if left != 1 else cr1[0]`, and to_cheque takes `cr1[-1]`
    // unconditionally.
    //
    // money_verbose / cents_verbose / cents_terse stay at their defaults
    // (`self.to_cardinal(number)`), which is what Num2Word_Base defines.
    // to_cheque reaches money_verbose; to_currency calls _int_to_word directly
    // and reaches none of them.

    /// Python's `Num2Word_YI.to_currency` — a wholesale override that shares
    /// no code with `Num2Word_Base.to_currency`.
    ///
    /// ```python
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    /// left_str = self._int_to_word(left)
    /// result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     cents_str = self._int_to_word(right)
    ///     result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// `adjective` is in the signature and never read — hence `_adjective`.
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
        let is_negative = val.is_negative();

        // Python takes abs() *first*, then splits str(val) on ".".
        let (left, right) = match val {
            // `str(int)` never contains a ".", so `len(parts) > 1` is False
            // and `right` stays 0. An int can never print cents — though here
            // that is incidental rather than the deliberate
            // `isinstance(val, int)` branch the base class uses (bug 9).
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let v = d.abs();
                // `int(parts[0])` — the digits left of the point, i.e. floor
                // (v is already non-negative), matching with_scale(0)'s
                // truncation.
                let integer = v.with_scale(0);
                let frac = &v - &integer;
                // `int(parts[1][:2].ljust(2, "0"))` — two decimal digits,
                // padded on the right and truncated beyond, which is exactly
                // floor(frac * 100): "5" -> 50, "239" -> 23, "005" -> 0.
                // Truncation, never rounding. See module bug 8.
                let right = (frac * BigDecimal::from(100)).with_scale(0);
                (
                    integer.as_bigint_and_exponent().0,
                    right.as_bigint_and_exponent().0,
                )
            }
        };

        // `.get(currency, <first value>)` — an unknown code borrows EUR's
        // forms rather than raising. See module bug 7.
        let forms = self.forms.get(currency).unwrap_or(&self.fallback_forms);

        let one = BigInt::one();
        let mut result = format!(
            "{} {}",
            self.int_to_word(&left),
            if left == one { &forms.unit[0] } else { &forms.unit[1] }
        );

        // `if cents and right:` — truthiness, so right == 0 drops the segment
        // even for a float. This is why 1.0 has no cents. See module bug 9.
        if cents && !right.is_zero() {
            result.push_str(separator_for(separator));
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right == one {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            });
        }

        if is_negative {
            // The raw negword ("מינוס ", trailing space), not parse_minus.

            result = format!("{}{}", NEGWORD, result);
        }
        // `.strip()` is a no-op — _int_to_word never returns padding and no
        // form is empty — but is reproduced verbatim.
        Ok(result.trim().to_string())
    }

    // to_cheque is NOT overridden: Num2Word_YI inherits
    // Num2Word_Base.to_cheque, which currency::default_to_cheque already
    // ports. It reaches YI only through lang_name / currency_forms /
    // currency_precision / money_verbose above, and needs no help:
    //   to_cheque(1234.56, "EUR") -> whole=1234, sub=56, divisor=100
    //     -> "<to_cardinal(1234)> AND 56/100 אייראָ".upper()
    // Rust's str::to_uppercase leaves Hebrew-script text alone exactly as
    // Python's str.upper() does, so only the ASCII forms shift:
    // "dollars" -> "DOLLARS", and the literal "and" -> "AND".
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(v: f64, precision: u32) -> Result<String> {
        LangYi::new().to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
    }

    fn d(s: &str, precision: u32) -> Result<String> {
        LangYi::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision,
            },
            None,
        )
    }

    fn value_err(r: Result<String>) -> String {
        match r {
            Err(N2WError::Value(m)) => m,
            other => panic!("expected ValueError, got {:?}", other),
        }
    }

    /// Every `"to": "cardinal"` row in bench/corpus.jsonl whose arg is a float.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(f(0.0, 1).unwrap(), "נול פּונקט נול");
        assert_eq!(f(0.5, 1).unwrap(), "נול פּונקט פינף");
        assert_eq!(f(1.0, 1).unwrap(), "איינס פּונקט נול");
        assert_eq!(f(1.5, 1).unwrap(), "איינס פּונקט פינף");
        assert_eq!(f(2.25, 2).unwrap(), "צוויי פּונקט צוויי פינף");
        assert_eq!(f(3.14, 2).unwrap(), "דרײַ פּונקט איינס פיר");
        assert_eq!(f(0.01, 2).unwrap(), "נול פּונקט נול איינס");
        assert_eq!(f(0.1, 1).unwrap(), "נול פּונקט איינס");
        assert_eq!(f(0.99, 2).unwrap(), "נול פּונקט נײַן נײַן");
        assert_eq!(f(1.01, 2).unwrap(), "איינס פּונקט נול איינס");
        assert_eq!(f(12.34, 2).unwrap(), "צען צוויי פּונקט דרײַ פיר");
        assert_eq!(f(99.99, 2).unwrap(), "נײַנציק נײַן פּונקט נײַן נײַן");
        assert_eq!(f(100.5, 1).unwrap(), "איינס הונדערט פּונקט פינף");
        assert_eq!(
            f(1234.56, 2).unwrap(),
            "איינס טויזנט צוויי הונדערט דרײַסיק פיר פּונקט פינף זעקס"
        );
        assert_eq!(f(-0.5, 1).unwrap(), "מינוס נול פּונקט פינף");
        assert_eq!(f(-1.5, 1).unwrap(), "מינוס איינס פּונקט פינף");
        assert_eq!(f(-12.34, 2).unwrap(), "מינוס צען צוויי פּונקט דרײַ פיר");
        assert_eq!(f(1.005, 3).unwrap(), "איינס פּונקט נול נול פינף");
        assert_eq!(f(2.675, 3).unwrap(), "צוויי פּונקט זעקס זיבן פינף");
    }

    /// Every `"to": "cardinal_dec"` row in bench/corpus.jsonl.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(d("0.01", 2).unwrap(), "נול פּונקט נול איינס");
        // The trailing "נול" is the point of this row: Decimal("1.10") keeps
        // its written scale, which the float 1.1 could never express.
        assert_eq!(d("1.10", 2).unwrap(), "איינס פּונקט איינס נול");
        assert_eq!(d("12.345", 3).unwrap(), "צען צוויי פּונקט דרײַ פיר פינף");
        // Issue #603: the exact-Decimal arm at trillion scale. The integer
        // part exceeds 10**9, so bug 3 hands back bare digits (bug 13).
        assert_eq!(
            d("98746251323029.99", 2).unwrap(),
            "98746251323029 פּונקט נײַן נײַן"
        );
        assert_eq!(d("0.001", 3).unwrap(), "נול פּונקט נול נול איינס");
    }

    /// Non-corpus rows verified against the live interpreter.
    #[test]
    fn live_interpreter_rows() {
        assert_eq!(
            f(1234567.89, 2).unwrap(),
            "איינס מיליאָן צוויי הונדערט דרײַסיק פיר טויזנט פינף הונדערט זעכציק זיבן פּונקט אַכט נײַן"
        );
        // Integer part >= 10**9: the digit-string fallback (bugs 3/13).
        assert_eq!(f(1500000000.25, 2).unwrap(), "1500000000 פּונקט צוויי פינף");
        // str(-0.0) == "-0.0": the sign bit survives repr, so negword fires.
        assert_eq!(f(-0.0, 1).unwrap(), "מינוס נול פּונקט נול");
        // A negative Decimal takes the same string-level "-" strip.
        assert_eq!(d("-12.34", 2).unwrap(), "מינוס צען צוויי פּונקט דרײַ פיר");
    }

    /// The f64-artefact cases. YI reads `str()`, not `float2tuple`, so the
    /// repr's digits are used verbatim — no `10**precision`, no `< 0.01`
    /// rescue, no banker's rounding anywhere on this path.
    #[test]
    fn f64_artefacts_come_from_repr_not_float2tuple() {
        assert_eq!(f(1.005, 3).unwrap(), "איינס פּונקט נול נול פינף");
        assert_eq!(f(2.675, 3).unwrap(), "צוויי פּונקט זעקס זיבן פינף");
        // repr keeps all 17 fractional digits of 0.1+0.2 ("0.30000000000000004")
        // and every one of them becomes a word — no rounding, no truncation.
        // Built rather than spelled out so the zero count cannot drift.
        let expected = format!("נול פּונקט דרײַ {} פיר", ["נול"; 15].join(" "));
        assert_eq!(f(0.1 + 0.2, 17).unwrap(), expected);
    }

    /// `precision=` sets `converter.precision`, which YI never reads.
    #[test]
    fn precision_override_is_inert() {
        let l = LangYi::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        for over in [None, Some(0), Some(1), Some(2), Some(5), Some(17)] {
            assert_eq!(
                l.to_cardinal_float(&v, over).unwrap(),
                "צוויי פּונקט זעקס זיבן פינף"
            );
        }
    }

    /// Bug 13's companion: a repr in exponent form reaches `int()` and raises,
    /// with CPython's exact message. Verified against the live interpreter.
    #[test]
    fn exponent_form_repr_raises_value_error() {
        // 1e15 still prints positionally, so it survives (as digits, by bug 3).
        assert_eq!(f(1e15, 1).unwrap(), "1000000000000000 פּונקט נול");
        // One decade up, decpt > 16 and repr flips to "1e+16" — no "." at all,
        // so the whole token hits int().
        assert_eq!(
            value_err(f(1e16, 0)),
            "invalid literal for int() with base 10: '1e+16'"
        );
        // The "-" is stripped from the *string* first, so the message quotes
        // the unsigned literal.
        assert_eq!(
            value_err(f(-1e16, 0)),
            "invalid literal for int() with base 10: '1e+16'"
        );
        assert_eq!(
            value_err(f(1e-5, 0)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        // "1.5e+20" keeps a ".", so the crash comes from the fraction loop on
        // the single char 'e' — after the '5' was already emitted.
        assert_eq!(
            value_err(f(1.5e20, 0)),
            "invalid literal for int() with base 10: 'e'"
        );
        // Decimal.__str__ goes scientific at leftdigits <= -6: "1E-7".
        assert_eq!(
            value_err(d("1E-7", 7)),
            "invalid literal for int() with base 10: '1E-7'"
        );
    }
}
