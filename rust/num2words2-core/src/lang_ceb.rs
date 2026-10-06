//! Port of `lang_CEB.py` (Cebuano).
//!
//! Shape: **self-contained**. `Num2Word_CEB` subclasses `Num2Word_Base`, but
//! its `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`.
//! The `any(hasattr(self, field) for field in [...])` guard in
//! `Num2Word_Base.__init__` is therefore False, so Python never builds
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int_to_word` recursively. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check at all** — see bug 1 below for what happens instead.
//!
//! Inheritance chain: `Num2Word_CEB` -> `Num2Word_Base` -> `object`. Every
//! in-scope method (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`)
//! is overridden by CEB itself, so nothing is inherited from `base.py` for the
//! four supported modes. `title()` is inherited but inert: `is_title` stays
//! False (set by `Num2Word_Base.__init__`, never flipped by CEB), so the
//! `exclude_title` list CEB populates is dead weight. It is mirrored here for
//! fidelity but can never affect output.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, each confirmed against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the bare digits.** The
//!    final `return str(number)` has no word forms above `milyon`, so
//!    `to_cardinal(10**9)` == "1000000000" and `to_cardinal(1234567890)` ==
//!    "1234567890" — digits, not words. This is *not* an OverflowError; the
//!    call succeeds and returns a numeric string. It cascades: `to_ordinal`
//!    prefixes it blindly, so `to_ordinal(10**9)` == "ika-1000000000". This
//!    is why the port must keep `BigInt` all the way down rather than
//!    bounding the input — arbitrarily large values are *valid* input that
//!    simply round-trip to their decimal representation.
//! 2. **`to_ordinal` accepts negatives and emits nonsense.** CEB overrides
//!    `to_ordinal` without the `errmsg_negord` guard that `Num2Word_Base`
//!    applies, so `to_ordinal(-1)` == "ika-minus usa" (an "ordinal" built on
//!    a minus sign) rather than raising. Likewise `to_ordinal_num(-1)` ==
//!    "ika--1", with the doubled hyphen.
//! 3. **`to_ordinal` computes the cardinal it may discard.** `cardinal =
//!    self.to_cardinal(number)` runs *before* the `if number == 1` check that
//!    returns "una" and ignores it. Harmless (no side effects), but the
//!    evaluation order is preserved below so the two stay lockstep if either
//!    ever grows one.
//! 4. **No teens.** 11 is "napulo ug usa" ("ten and one"), not a dedicated
//!    form. Correct Cebuano is arguably "napulog usa"; the corpus freezes the
//!    spaced form, so that is what ships.
//! 5. **`gatos`/`libo`/`milyon` join with a bare space, but `ug` ("and") only
//!    appears below 1000.** So 1001 is "usa libo usa" (no "ug"), while 101 is
//!    "gatos ug usa". Asymmetric, and preserved verbatim.
//! 6. **A float whose repr goes exponential is a `ValueError`, not a number.**
//!    `to_cardinal` feeds the halves of `str(number)` straight to `int()`, and
//!    CPython's float repr switches to exponent form at `decpt > 16` or
//!    `decpt <= -4`. So `1e15` is fine ("1000000000000000 puntos siro", digits
//!    by bug 1) but one step up:
//!
//!    | input | `str()` | result |
//!    |---|---|---|
//!    | `1e16` | `'1e+16'` | `ValueError: invalid literal for int() with base 10: '1e+16'` |
//!    | `1e-05` | `'1e-05'` | `ValueError: ... : '1e-05'` |
//!    | `1.5e20` | `'1.5e+20'` | `ValueError: ... : 'e'` |
//!    | `Decimal("1E+2")` | `'1E+2'` | `ValueError: ... : '1E+2'` |
//!
//!    The `1.5e+20` row is the interesting one: the string *does* contain a
//!    ".", so it takes the fraction branch, `int("1")` succeeds, the digit
//!    loop emits "lima" for the '5' — and only then does `int('e')` blow up.
//!    Hence the message quotes `'e'`, the single offending character, not the
//!    whole token. All four are corpus-absent but live-interpreter confirmed.
//!    Contrast `Num2Word_Base`, which would cheerfully render `1e16` via
//!    `float2tuple`; CEB cannot reach that code.
//!
//! # Notes on unreachable Python paths
//!
//! * `_int_to_word` is never called with a negative: `to_cardinal` strips the
//!   sign from the *string* before recursing, and every internal recursion
//!   passes a `divmod` remainder. Were a negative to reach it, Python's
//!   `number < 10` would be True and `self.ones[number]` would silently
//!   negative-index the list (`ones[-1]` == "siyam"). Unreachable in scope, so
//!   not modelled; [`int_to_word`] below takes non-negative values only.
//!
//! # The float/Decimal path does not use `float2tuple`
//!
//! `Num2Word_CEB` overrides `to_cardinal` outright, so `Num2Word_Base`'s
//! `to_cardinal_float`/`float2tuple` never run for CEB — not the
//! `abs(value - pre) * 10**precision` binary arithmetic, not the `< 0.01`
//! rescue heuristic, and not `self.precision`. CEB's very first statement is
//!
//! ```python
//! n = str(number).strip()
//! ```
//!
//! and everything after it is string surgery on that repr: split on the first
//! ".", `int()` the left half, and map the right half **character by
//! character** through `self.ones`. So the port's entire job on this path is to
//! reproduce CPython's `str(float)` and `str(Decimal)` exactly — see
//! [`py_str_f64`] and [`py_str_decimal`]. Three consequences:
//!
//! 1. **The `precision=` kwarg is inert.** The dispatcher does set
//!    `converter.precision` (base's `__init__` gives CEB the attribute, so
//!    `hasattr` passes), but CEB never reads it. Confirmed live:
//!    `num2words(2.675, lang="ceb", precision=1)` and `precision=5` both give
//!    "duha puntos unom pito lima". [`LangCeb::to_cardinal_float`] therefore
//!    ignores `precision_override`, and ignores `FloatValue::precision` too.
//! 2. **The fraction is never rounded or padded** — it is the repr's digits
//!    verbatim. `1.005` -> "usa puntos siro siro lima" and `2.675` -> "duha
//!    puntos unom pito lima" come straight from `str()`, not from the f64
//!    artefact + heuristic dance. (Base happens to agree on both; it does not
//!    agree once the repr goes exponential — see bug 6.)
//! 3. **Trailing zeros survive.** `str(1.0)` is "1.0", so `to_cardinal(1.0)` is
//!    "usa puntos siro", and `str(Decimal("1.10"))` keeps its scale, giving
//!    "usa puntos usa siro".
//!
//! # Error variants
//!
//! None, for the four integer modes. For integer input there is no card table
//! to overflow, no dict lookup to miss, and no list index that a non-negative
//! value can push out of range.
//!
//! The float/Decimal path adds `Value` (Python `ValueError`) — see bug 6.
//!
//! The currency surface adds exactly two:
//!
//! * `to_cheque` raises `NotImplemented` for an unknown code. CEB inherits it
//!   unchanged from `Num2Word_Base`, which looks the code up with `[]` rather
//!   than `.get` — so it raises exactly where [`LangCeb::to_currency`] silently
//!   falls back to PISO. That asymmetry is real and the corpus pins it:
//!   `currency:GBP` yields "piso", `cheque:GBP` raises.
//! * `to_currency` raises `Value` (Python `ValueError`) when `str(abs(val))`
//!   comes out in exponent notation, because CEB feeds that string straight to
//!   `int()`. `num2words(1e16, lang="ceb", to="currency")` is
//!   `ValueError: invalid literal for int() with base 10: '1e+16'`, not a
//!   number. See [`LangCeb::to_currency`] for which side of that Rust can and
//!   cannot reproduce.
//!
//! Note that an unknown *currency code* never raises from `to_currency` — the
//! `CURRENCY_FORMS.get(currency, <first entry>)` fallback swallows it.
//!
//! # Currency: what CEB overrides and what it inherits
//!
//! `Num2Word_CEB` defines its own `to_currency` and `pluralize` and its own
//! three-entry `CURRENCY_FORMS`. It defines **no** `CURRENCY_PRECISION` and no
//! `CURRENCY_ADJECTIVES`, and it does not touch `Num2Word_EUR`'s shared table,
//! so the `lang_EUR`/`Num2Word_EN` mutation trap described in
//! PORTING_CURRENCY.md does not reach CEB. Verified against the live
//! interpreter: `CONVERTER_CLASSES["ceb"].CURRENCY_FORMS` is exactly the three
//! codes below, and both `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are
//! empty dicts.
//!
//! Because CEB's `to_currency` overrides `Num2Word_Base.to_currency` outright,
//! **none** of base's currency machinery runs: no `CURRENCY_PRECISION` lookup,
//! no zero-decimal (JPY) rounding preamble, no `parse_currency_parts`, no
//! `pluralize` call, no `adjective` handling. CEB reimplements the lot with
//! naive string slicing. That is why `currency_precision` is deliberately left
//! at its default here — overriding it would be dead code that misleads the
//! next reader into thinking JPY behaves differently. It does not.
//!
//! `to_cheque` is *not* overridden, so it comes from `Num2Word_Base` and does
//! consult `CURRENCY_PRECISION` (finding nothing, hence divisor 100 for every
//! code, including KWD/BHD) and `_money_verbose` (base's, hence CEB's
//! `to_cardinal`). The trait defaults reproduce it exactly, so it is not
//! overridden here either.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// Python: `self.negword = "minus "` — note the trailing space, which the
/// `.strip()` in `to_cardinal` makes invisible in practice.
const NEGWORD: &str = "minus ";
/// Python: `self.pointword = "puntos"`. Separates the integer part from the
/// per-digit fraction on the float/Decimal path.
const POINTWORD: &str = "puntos";

/// `self.__class__.__name__`, for the NotImplementedError `to_cheque` raises.
const LANG_NAME: &str = "Num2Word_CEB";

/// The key of the first entry in Python's `CURRENCY_FORMS` dict literal.
///
/// `to_currency`'s fallback is `list(self.CURRENCY_FORMS.values())[0]`, i.e.
/// *insertion order*, which for the literal in `lang_CEB.py` (PHP, USD, EUR)
/// means PHP. A `HashMap` cannot express that ordering, so the fallback is
/// resolved once in [`LangCeb::new`] and stored rather than recovered by
/// iterating the map — whose order is randomised per process.
const FALLBACK_CURRENCY: &str = "PHP";

/// Python's `_int_to_word` special-cases 0 rather than reading `ones[0]`
/// (which is the empty string).
const ZERO_WORD: &str = "siro";
const HUNDRED: &str = "gatos";
const THOUSAND: &str = "libo";
const MILLION: &str = "milyon";

/// Python: `self.ones`. Index 0 is "" and is never read — `_int_to_word`
/// returns "siro" for 0 before it can be reached, and `h`/`o`/`t` indices are
/// only taken when non-zero.
const ONES: [&str; 10] = [
    "", "usa", "duha", "tulo", "upat", "lima", "unom", "pito", "walo", "siyam",
];

/// Python: `self.tens`. Index 0 is likewise unreachable (the `< 100` branch
/// only runs for `number >= 10`, so `t >= 1`).
const TENS: [&str; 10] = [
    "",
    "napulo",
    "kawhaan",
    "katloan",
    "kap-atan",
    "kalim-an",
    "kan-uman",
    "kapitoan",
    "kawaloan",
    "kasiyaman",
];

/// Narrow a `BigInt` to a list index.
///
/// Every call site has already proven the value is in `0..=9` (it is either a
/// number known to be `< 10`, or a `divmod(_, 10)` quotient of a number known
/// to be `< 100`), so the conversion cannot fail. The `expect` documents that
/// proof rather than guarding a real case.
fn idx(n: &BigInt) -> usize {
    n.to_usize()
        .expect("caller proved 0 <= n < 10 before indexing")
}

/// Port of `Num2Word_CEB.CURRENCY_FORMS`, verbatim and complete — three codes.
///
/// CEB declares this in its own class body and subclasses `Num2Word_Base`
/// directly, so it neither reads nor is polluted by the shared
/// `Num2Word_EUR.CURRENCY_FORMS` dict that `Num2Word_EN.__init__` mutates.
/// The `lang_EUR` trap in PORTING_CURRENCY.md therefore does not apply: there
/// is no inherited EUR/GBP entry to be rewritten, and none of EN's ~24 added
/// codes (JPY, KWD, BHD, ...) are visible here. Confirmed against the live
/// interpreter — `CONVERTER_CLASSES["ceb"].CURRENCY_FORMS` has exactly these
/// keys, in this order.
///
/// Every entry carries two forms, and both are identical: Cebuano does not
/// inflect these nouns for number. The duplication is not a typo to be
/// collapsed — `to_currency` indexes `cr1[1]` for any count != 1, so the second
/// slot must exist.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const SENTIMO: [&str; 2] = ["sentimo", "sentimo"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("PHP", CurrencyForms::new(&["piso", "piso"], &SENTIMO));
    m.insert("USD", CurrencyForms::new(&["dolyar", "dolyar"], &SENTIMO));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &SENTIMO));
    m
}

pub struct LangCeb {
    /// Python: `self.exclude_title = ["minus", "puntos", "ug"]`. Inert — see
    /// the module docs; `is_title` is never True for CEB.
    exclude_title: Vec<String>,
    /// Built once here rather than per call. `to_currency` and `to_cheque`
    /// only ever read this table, and rebuilding it on each call is what made
    /// an earlier revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangCeb {
    pub fn new() -> Self {
        LangCeb {
            exclude_title: ["minus", "puntos", "ug"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            currency_forms: build_currency_forms(),
        }
    }
}

/// Python's `int(s)`, raising `ValueError` with CPython's exact message.
///
/// CEB reaches `int()` with whatever `str(abs(val))` produced, which is not
/// always digits — see [`LangCeb::to_currency`]. CPython's message is
/// `invalid literal for int() with base 10: '<s>'`, quoted exactly.
///
/// The accepted grammars differ in ways that cannot bite here: Python's `int`
/// also takes surrounding whitespace, a `+` sign and `_` separators, none of
/// which `BigInt`/`BigDecimal` `Display` can emit.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Python's `int(ch)` for a *single* character, as the fraction loop calls it.
///
/// The message quotes the one offending character, matching CPython — that is
/// why `1.5e+20` reports `'e'` rather than `'5e+20'` (bug 6).
///
/// One unreachable divergence: CPython's `int()` accepts any Unicode `Nd`
/// codepoint (`int('٥') == 5`), while `char::to_digit(10)` is ASCII-only. No
/// `str(float)` or `str(Decimal)` can emit a non-ASCII digit, so the two agree
/// on every string that reaches here.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10).map(|d| d as usize).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

/// The shortest round-trip decimal digits of `a` (which must be finite and
/// non-negative), plus CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// # Why `{:e}` alone is not enough
///
/// Rust's `{:e}` and CPython's `repr` are both "shortest string that reads back
/// as the same double", and they agree on the digit *count* always and on the
/// digits themselves almost always. They part company on an **exact tie**,
/// where two equally-short decimals are equidistant from the true value and
/// both round-trip: CPython's `_Py_dg_dtoa` breaks the tie to **even**, Rust's
/// `flt2dec` shortest breaks it **away from zero**. The double whose exact
/// value is `670352580196876.25` sits precisely between `...876.2` and
/// `...876.3`, so `repr` says `.2` and `{:e}` says `.3`.
///
/// This is the same banker's-rounding trap `floatpath.rs` documents for
/// `round()`, relocated into the formatter — and it bites here for the same
/// reason: CEB's whole float path *is* the repr string, so a wrong last digit
/// is a wrong word.
///
/// # The fix
///
/// Rust's *fixed-precision* `{:.*e}` is correctly rounded half-to-even, so
/// re-emitting the same number of significant digits through it applies
/// CPython's tie rule. It is kept only if it still round-trips, so a
/// hypothetical asymmetric rounding interval (near a power of two, where the
/// nearer decimal can sit outside the interval) falls back to the shortest
/// digits rather than silently producing a string that reads back as a
/// different double.
///
/// Measured against CPython over 777,014 doubles — 430k random bit patterns
/// plus denormals, powers of two, the `1e16`/`1e-4` format boundaries and
/// tie-prone few-significant-bit values — this reproduces `repr` with zero
/// mismatches. `{:e}` alone missed 21 of the first 75,034.
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

/// CPython's `str(float)` / `repr(float)`, which is where CEB's float path
/// begins and ends.
///
/// `PyOS_double_to_string(v, 'r', 0, Py_DTSF_ADD_DOT_0)` in `pystrtod.c`:
/// take the shortest round-trip digits, then
///
/// * switch to exponent form when `decpt <= -4 || decpt > 16` — hence
///   `str(1e15) == "1000000000000000.0"` but `str(1e16) == "1e+16"`, and
///   `str(0.0001) == "0.0001"` but `str(1e-05) == "1e-05"`. (CPython moved this
///   threshold down from 1e17 so that `repr(2e16+8)` stops claiming
///   `20000000000000010.0` for a value that is really `...008.0`.)
/// * format that exponent `%+.02d` — signed, at least two digits, so `1e+16`
///   and `1e-05` but `5e-324`.
/// * otherwise print positionally and append `.0` if nothing follows the point
///   (`Py_DTSF_ADD_DOT_0`), which is the whole reason `1.0` is "usa puntos
///   siro" and not "usa".
fn py_str_f64(v: f64) -> String {
    // Unreachable from the shim, which computes `precision` as
    // `abs(Decimal(str(value)).as_tuple().exponent)` and would raise on the
    // 'F'/'n' exponent of a non-finite Decimal long before Rust is called.
    // Handled anyway so the helper is a faithful `str()` rather than a
    // faithful-in-context one; CEB would go on to raise `ValueError` on
    // `int("inf")`, exactly as Python does.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    // The sign is taken from the sign *bit*, not from `v < 0.0`, so that
    // `str(-0.0)` is "-0.0" and CEB's `startswith("-")` fires: -0.0 renders
    // "minus siro puntos siro".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            // No ADD_DOT_0 in exponent form: `str(1e16)` is "1e+16", not
            // "1.0e+16".
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

/// CPython's `Decimal.__str__` (the spec's `to-scientific-string`), ported from
/// `_pydecimal.Decimal.__str__`.
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # positional
/// else:
///     dotplace = 1                   # scientific
/// ```
///
/// # Why this cannot just be `BigDecimal`'s `Display`
///
/// The two disagree in three places, each of which would change CEB's output or
/// its exception:
///
/// | value | `Decimal.__str__` | `BigDecimal` `Display` |
/// |---|---|---|
/// | `1E+2` | `1E+2` (CEB: `ValueError`) | `100` (CEB: "gatos") |
/// | `0.0` | `0.0` (CEB: "siro puntos siro") | `0` (CEB: "siro") |
/// | `1E+16` | `1E+16` | `1e+16` — lowercase |
///
/// So the digits and exponent are read off `as_bigint_and_exponent()` and
/// reassembled by Python's rule instead. That pairing is exact:
/// `BigDecimal::from_str` keeps the written scale rather than normalising
/// (`"1.10"` stays coefficient 110 / scale 2, which is what makes the trailing
/// "siro" appear), and `(coefficient, -scale)` is precisely Python's
/// `(_int, _exp)` — including for values Python itself cannot tell apart, since
/// `Decimal("1E-7")` and `Decimal("0.0000001")` *are* the same object and both
/// stringify "1E-7".
///
/// Verified against CPython over 40,029 Decimals — random coefficients up to 25
/// digits crossed with exponents in ±35, plus the positional/scientific
/// boundary cases. The only mismatches were negative zero; see
/// [`LangCeb::to_cardinal_float`].
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

/// Port of `Num2Word_CEB.to_cardinal` operating on the repr, which is the form
/// Python's own code works in:
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     return (self.negword + self.to_cardinal(n[1:])).strip()
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "siro")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// The recursion really is string-level in Python — `to_cardinal(n[1:])` hands
/// a `str` back to the same method, whose `str(number)` is then a no-op — so it
/// is reproduced as such rather than folded into an `abs()`. One level deep in
/// practice: no repr yields a second leading "-".
///
/// Two details that look like slips but are Python's:
///
/// * `split(".", 1)` splits on the *first* dot only, so a second dot would land
///   in `right` and reach `int()` as a character. Unreachable from a repr.
/// * `self.ones[0]` is `""`, which is falsy, so `or "siro"` is what turns a
///   fraction digit 0 into a word. That is a different mechanism from
///   `_int_to_word`'s explicit `if number == 0` — same output, and both are
///   needed.
fn cardinal_from_repr(n: &str) -> Result<String> {
    // n = str(number).strip(). Python strips its own whitespace set and Rust
    // trims Unicode's; no repr contains either.
    let n = n.trim();

    // if n.startswith("-"): return (self.negword + self.to_cardinal(n[1:])).strip()
    if let Some(rest) = n.strip_prefix('-') {
        let inner = cardinal_from_repr(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }

    let (left, right) = match n.split_once('.') {
        Some(halves) => halves,
        // return self._int_to_word(int(n))
        None => return Ok(int_to_word(&py_int(n)?)),
    };

    // ret = self._int_to_word(int(left)) + " " + self.pointword
    //
    // `int(left)` is the whole integer part, so bug 1 applies: at 1e9 and above
    // it comes back as bare digits ("98746251323029 puntos siyam siyam").
    let mut ret = int_to_word(&py_int(left)?);
    ret.push(' ');
    ret.push_str(POINTWORD);

    // for digit in right: ret += " " + (self.ones[int(digit)] or "siro")
    //
    // Per *character*, so there is no grouping, no rounding and no padding: the
    // fraction is however many digits the repr had.
    for digit in right.chars() {
        let word = ONES[py_int_digit(digit)?];
        ret.push(' ');
        ret.push_str(if word.is_empty() { ZERO_WORD } else { word });
    }
    // Cosmetic: `_int_to_word` never returns "", so nothing can be trimmed.
    Ok(ret.trim().to_string())
}

/// Python's `cr[1] if n != 1 else cr[0]`, as `to_currency` inlines it.
///
/// Deliberately *not* routed through [`LangCeb::pluralize`]: CEB's `pluralize`
/// takes `forms[-1]`, this takes `forms[1]`. Identical for CEB's two-form
/// entries, but they are different functions and only this one is on the
/// `to_currency` path. Out-of-range indexing is `IndexError` in Python;
/// unreachable for the shipped table, mapped rather than panicking so the
/// exception type survives if the table ever changes.
fn pick_form(forms: &[String], n: &BigInt) -> Result<String> {
    let i = if n.is_one() { 0 } else { 1 };
    forms
        .get(i)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

impl Default for LangCeb {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of `Num2Word_CEB._int_to_word`. `number` must be non-negative (see the
/// module docs: no in-scope caller can pass a negative).
///
/// Python uses `divmod`, i.e. floor division; `div_mod_floor` matches it
/// exactly. For the non-negative operands here it coincides with truncating
/// division, but the faithful primitive is used regardless.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);

    // if number < 10: return self.ones[number]
    if number < &ten {
        return ONES[idx(number)].to_string();
    }

    // if number < 100:
    //     t, o = divmod(number, 10)
    //     return self.tens[t] + (" ug " + self.ones[o] if o else "")
    if number < &hundred {
        let (t, o) = number.div_mod_floor(&ten);
        let mut out = TENS[idx(&t)].to_string();
        if !o.is_zero() {
            out.push_str(" ug ");
            out.push_str(ONES[idx(&o)]);
        }
        return out;
    }

    // if number < 1000:
    //     h, r = divmod(number, 100)
    //     base = (self.ones[h] + " " if h > 1 else "") + self.hundred
    //     return base + (" ug " + self._int_to_word(r) if r else "")
    //
    // Note `h > 1`, not `h >= 1`: 100 is "gatos", never "usa gatos".
    if number < &thousand {
        let (h, r) = number.div_mod_floor(&hundred);
        let mut out = String::new();
        if h > BigInt::one() {
            out.push_str(ONES[idx(&h)]);
            out.push(' ');
        }
        out.push_str(HUNDRED);
        if !r.is_zero() {
            out.push_str(" ug ");
            out.push_str(&int_to_word(&r));
        }
        return out;
    }

    // if number < 1000000:
    //     t, r = divmod(number, 1000)
    //     base = self._int_to_word(t) + " " + self.thousand
    //     return base + (" " + self._int_to_word(r) if r else "")
    //
    // Unlike the hundreds branch there is no `> 1` test, so 1000 is
    // "usa libo" (with the "usa"), and the remainder joins with a plain
    // space rather than " ug ".
    if number < &million {
        let (t, r) = number.div_mod_floor(&thousand);
        let mut out = int_to_word(&t);
        out.push(' ');
        out.push_str(THOUSAND);
        if !r.is_zero() {
            out.push(' ');
            out.push_str(&int_to_word(&r));
        }
        return out;
    }

    // if number < 1000000000:
    //     m, r = divmod(number, 1000000)
    //     base = self._int_to_word(m) + " " + self.million
    //     return base + (" " + self._int_to_word(r) if r else "")
    if number < &billion {
        let (m, r) = number.div_mod_floor(&million);
        let mut out = int_to_word(&m);
        out.push(' ');
        out.push_str(MILLION);
        if !r.is_zero() {
            out.push(' ');
            out.push_str(&int_to_word(&r));
        }
        return out;
    }

    // return str(number)  -- bug 1: no words exist above `milyon`.
    number.to_string()
}

impl Lang for LangCeb {

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

    /// `to_ordinal(float/Decimal)`: Python computes the (float-grammar)
    /// cardinal *first*, then short-circuits on `number == 1` — a numeric
    /// comparison, so `1.0` and `Decimal("1.00")` both yield "una" while
    /// every other value gets the "ika-" prefix on the spelled-out repr:
    /// `to_ordinal(5.0)` == "ika-lima puntos siro", `to_ordinal(-1.5)` ==
    /// "ika-minus usa puntos lima" (bug: the prefix lands *before* the
    /// negword). Exponent reprs raise the cardinal's ValueError first —
    /// order preserved.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.to_cardinal_float(value, None)?;
        if value.as_whole_int() == Some(BigInt::from(1)) {
            return Ok("una".to_string());
        }
        Ok(format!("ika-{}", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ika-" + str(number)` — the repr
    /// verbatim, so a negative doubles the dash: `-0.0` → "ika--0.0",
    /// `Decimal("5.00")` → "ika-5.00", `1e16` → "ika-1e+16".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("ika-{}", repr_str))
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, except
    /// that an Infinity parse is surfaced as the ValueError CEB's own
    /// `to_cardinal` raises one step later: `str(Decimal("Infinity"))` has no
    /// "." and `int("Infinity")` chokes on the literal. The dispatcher's
    /// default maps `ParsedNumber::Inf` to base's OverflowError, which CEB
    /// can never raise. NaN keeps the default routing (ValueError either way).
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
        "PHP"
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
        "puntos"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_CEB.to_cardinal`, integer path only.
    ///
    /// Python works on `n = str(number).strip()` and tests `n.startswith("-")`,
    /// then recurses on the *string* tail `n[1:]` — which is just the absolute
    /// value's digits, so `value.abs()` is equivalent. The float branch
    /// (`"." in n`) is unreachable for `str(int)`.
    ///
    /// The `.strip()` on the negative result is faithful but cosmetic: it can
    /// only bite if the inner cardinal were empty, and `_int_to_word` never
    /// returns "" for a value >= 1.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // Python: (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(int_to_word(value))
    }

    /// Port of `Num2Word_CEB.to_cardinal` for float and Decimal input — the
    /// same method [`LangCeb::to_cardinal`] ports, reached with a value whose
    /// `str()` may contain a ".".
    ///
    /// Note what is *not* here: no `float2tuple`, no `10**precision`, no
    /// `< 0.01` rescue, no `round()`. CEB overrides `to_cardinal` outright, so
    /// `Num2Word_Base.to_cardinal_float` is dead code for this language and the
    /// f64 artefacts it exists to preserve never arise — `str()` is applied to
    /// the double directly. The banker's-rounding trap still applies, just one
    /// level down, inside [`shortest_repr_digits`].
    ///
    /// `precision_override` is accepted and ignored, and so is
    /// `FloatValue::precision`. The `precision=` kwarg mutates
    /// `converter.precision`, which CEB never reads; the live interpreter
    /// agrees that `precision=1`, `precision=2` and `precision=5` all give
    /// "duha puntos unom pito lima" for 2.675.
    ///
    /// # The one input this cannot reproduce
    ///
    /// A **negative-zero Decimal**. `str(Decimal("-0.0"))` is "-0.0", so Python
    /// takes the `startswith("-")` branch and answers "minus siro puntos siro";
    /// this returns "siro puntos siro". The sign is lost at the `FloatValue`
    /// boundary, not here: `BigDecimal` holds a `BigInt` coefficient, and
    /// `BigInt` has no negative zero — `floatpath::FloatValue::is_negative` has
    /// the identical blind spot. Recovering it needs a signedness bit the enum
    /// does not carry. `-0.0` as a *float* is unaffected: f64 keeps the sign
    /// bit, and [`py_str_f64`] reads it. Flagged in the port report; no corpus
    /// row exercises it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        // n = str(number) — the float and Decimal arms are not interchangeable
        // (issue #603), and CEB makes the split visible in a way base does not:
        // `str` of a Decimal keeps every written digit, so Decimal("1.10") ends
        // in "siro" where the float 1.10 could not.
        let n = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        cardinal_from_repr(&n)
    }

    /// Port of `Num2Word_CEB.to_ordinal`.
    ///
    /// Never raises — no negative guard (bug 2), and the cardinal is computed
    /// before the `number == 1` short-circuit that discards it (bug 3). Order
    /// preserved deliberately.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        if value.is_one() {
            return Ok("una".to_string());
        }
        Ok(format!("ika-{}", cardinal))
    }

    /// Port of `Num2Word_CEB.to_ordinal_num`: `"ika-" + str(number)`.
    ///
    /// No sign handling, hence "ika--1" for -1 (bug 2).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ika-{}", value))
    }

    /// Port of `Num2Word_CEB.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The `longval` parameter is accepted and ignored by Python. Years get no
    /// pair-wise reading ("usa libo siyam gatos ug kasiyaman ug siyam" for
    /// 1999, not a "nineteen ninety-nine" style split). This matches the trait
    /// default, but is spelled out because CEB states it explicitly.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency -------------------------------------------------------
    //
    // CEB overrides `to_currency` and `pluralize` and supplies its own
    // `CURRENCY_FORMS`. It defines no `CURRENCY_PRECISION` and no
    // `CURRENCY_ADJECTIVES` (both are `{}` on the live class), so
    // `currency_precision` and `currency_adjective` are left at their trait
    // defaults — 100 for every code and `None` respectively. Overriding them
    // would be dead code that misleads the next reader into thinking JPY or
    // KWD behave differently here. They do not: KWD is not even a known code.
    //
    // `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
    // inherited from `Num2Word_Base` unchanged, and the trait defaults already
    // mirror them, so they are not overridden either. `default_to_cheque`
    // reaches CEB's `to_cardinal` through the default `money_verbose`, and
    // raises `NotImplemented` via `currency_forms` returning `None`.

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `CURRENCY_FORMS[code]`, honestly reporting a miss as `None`.
    ///
    /// The PISO fallback that CEB's `to_currency` applies is deliberately *not*
    /// baked in here. `to_cheque` reads this same table and must raise
    /// `NotImplemented` for an unknown code, so folding the fallback in would
    /// turn `cheque:GBP` from a raise into "... PISO". The fallback belongs to
    /// `to_currency` alone; see there.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_CEB.pluralize`.
    ///
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Total — the empty guard means it cannot `IndexError`, unlike the
    /// `Num2Word_EUR.pluralize` most languages inherit. Note `forms[-1]`, the
    /// *last* form, not `forms[1]`.
    ///
    /// Dead code in practice: CEB's `to_currency` inlines its own plural choice
    /// (see `pick_form`) and `to_cheque` takes `cr1[-1]` directly, so nothing
    /// in scope calls this. Implemented anyway because the trait default raises
    /// `NotImplemented`, which would be wrong if anything ever did reach it.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Port of `Num2Word_CEB.to_currency`, which overrides
    /// `Num2Word_Base.to_currency` outright.
    ///
    /// ```python
    /// is_negative = val < 0
    /// val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    /// result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// None of base's currency machinery runs: no `CURRENCY_PRECISION` lookup,
    /// no zero-decimal (JPY) rounding preamble, no `parse_currency_parts`, no
    /// `pluralize` call, no `has_decimal` guard, no `_cents_terse`. CEB
    /// reimplements the lot by slicing a decimal string. Four consequences,
    /// all corpus-confirmed:
    ///
    /// 1. **Cents truncate, they do not round.** `parts[1][:2]` discards the
    ///    third decimal onward, where base quantizes ROUND_HALF_UP. So 2.675
    ///    is "kan-uman ug pito sentimo" (67, not 68) and 1.005 is bare "usa
    ///    piso" — the half-cent vanishes rather than rounding up to 01.
    /// 2. **An unknown code cannot raise.** `.get(currency, <first value>)`
    ///    falls back to the *insertion-order first* entry, PHP. Hence
    ///    `currency:JPY` -> "piso", not `NotImplementedError` and not yen.
    ///    The `FALLBACK_CURRENCY` constant resolves that ordering, which a
    ///    `HashMap` cannot express.
    /// 3. **`cents=False` drops the segment entirely** rather than falling back
    ///    to the terse "56" form base would emit.
    /// 4. **`adjective` is accepted and ignored.** CEB has no
    ///    `CURRENCY_ADJECTIVES`, and unlike base it never consults them.
    ///
    /// `has_decimal` is genuinely irrelevant here, so it is not consulted: CEB
    /// gates the cents segment on `right` being truthy, and `str(Decimal("5"))`
    /// and `str(Decimal("5.00"))` both yield `right == 0`. The int/Decimal
    /// split still matters for *stringification*, not for the guard.
    ///
    /// # The exponent-notation hazard
    ///
    /// `int(parts[0])` is fed raw string output, so any `str(abs(val))` in
    /// exponent form is a `ValueError`, not a number. Python raises on `1e16`,
    /// `1e21`, `1e-05` and `Decimal("1E+2")` alike.
    ///
    /// This side sees only the parsed `BigDecimal`, never Python's original
    /// `str(val)`, so the reproduction is exact in the large-magnitude regime
    /// and lossy in the small one:
    ///
    /// * **Exact for floats >= 1e16.** `BigDecimal`'s `Display` switches to
    ///   `1e+NN` at scale <= -16, and CPython's float repr switches at exactly
    ///   1e16 — the thresholds coincide, so `1e16`/`1e21` raise here with
    ///   byte-identical messages. Below 1e16 both render plain and agree: `1e15`
    ///   is `str()`-ed to "1000000000000000.0", so `left` is 10^15 and bug 1
    ///   hands back the bare digits — "1000000000000000 piso", not words.
    /// * **Diverges for floats < 1e-4 and for `Decimal("1E+n")`.**
    ///   `str(1e-05)` is `'1e-05'` (Python: `ValueError`) but
    ///   `BigDecimal::from_str("1e-05")` is scale 5, indistinguishable from
    ///   `Decimal("0.00001")` (Python: "siro piso"), and this returns the
    ///   latter for both. Likewise `Decimal("1E+2")` is scale -2, which
    ///   `Display` renders "100", so this returns "gatos piso" where Python
    ///   raises. Recovering these needs the pre-parse string, which the
    ///   `CurrencyValue` boundary discards. Flagged in the port report; no
    ///   corpus row exercises either, and both are sub-cent or exotic-Decimal
    ///   inputs rather than money.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // CEB's signature takes `adjective` and never reads it (quirk 4).
        let _ = adjective;
        // `separator=" "` in Python's signature; `None` means the caller
        // omitted the kwarg, so the language default applies.
        let separator = separator.unwrap_or(self.default_separator());

        // is_negative = val < 0; val = abs(val); parts = str(val).split(".")
        //
        // Python stringifies *after* abs(), so the sign never reaches the
        // string and `parts[0]` is always unsigned. Both `Display` impls match
        // CPython's `str()` for every plain-decimal input; see the exponent
        // note above for where that stops holding.
        let (is_negative, s) = match val {
            CurrencyValue::Int(v) => (v.is_negative(), v.abs().to_string()),
            CurrencyValue::Decimal { value, .. } => (value.is_negative(), value.abs().to_string()),
        };
        let parts: Vec<&str> = s.split('.').collect();

        // left = int(parts[0]) if parts[0] else 0
        let left = if parts[0].is_empty() {
            BigInt::zero()
        } else {
            py_int(parts[0])?
        };

        // right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
        //
        // Slicing then right-padding to two *characters* is what truncates
        // rather than rounds (quirk 1): "5" -> "50", "999" -> "99", "0" -> "00".
        let right = match parts.get(1) {
            Some(frac) if !frac.is_empty() => {
                let mut cents_digits: String = frac.chars().take(2).collect();
                while cents_digits.chars().count() < 2 {
                    cents_digits.push('0');
                }
                py_int(&cents_digits)?
            }
            _ => BigInt::zero(),
        };

        // cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
        let forms = self.currency_forms(currency).unwrap_or_else(|| {
            self.currency_forms
                .get(FALLBACK_CURRENCY)
                .expect("FALLBACK_CURRENCY is inserted by build_currency_forms")
        });

        // result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
        let mut result = int_to_word(&left);
        result.push(' ');
        result.push_str(&pick_form(&forms.unit, &left)?);

        // if cents and right:
        //     result += separator + self._int_to_word(right) + " " + (...)
        //
        // `right` is a Python int here, so 0 is falsy and a whole-unit amount
        // prints no cents at all — this is why 1.0 is "usa euro".
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(&pick_form(&forms.subunit, &right)?);
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn f(v: f64, precision: u32) -> Result<String> {
        LangCeb::new().to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
    }

    fn d(s: &str, precision: u32) -> Result<String> {
        LangCeb::new().to_cardinal_float(
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
        assert_eq!(f(0.0, 1).unwrap(), "siro puntos siro");
        assert_eq!(f(0.5, 1).unwrap(), "siro puntos lima");
        assert_eq!(f(1.0, 1).unwrap(), "usa puntos siro");
        assert_eq!(f(1.5, 1).unwrap(), "usa puntos lima");
        assert_eq!(f(2.25, 2).unwrap(), "duha puntos duha lima");
        assert_eq!(f(3.14, 2).unwrap(), "tulo puntos usa upat");
        assert_eq!(f(0.01, 2).unwrap(), "siro puntos siro usa");
        assert_eq!(f(0.1, 1).unwrap(), "siro puntos usa");
        assert_eq!(f(0.99, 2).unwrap(), "siro puntos siyam siyam");
        assert_eq!(f(1.01, 2).unwrap(), "usa puntos siro usa");
        assert_eq!(f(12.34, 2).unwrap(), "napulo ug duha puntos tulo upat");
        assert_eq!(f(99.99, 2).unwrap(), "kasiyaman ug siyam puntos siyam siyam");
        assert_eq!(f(100.5, 1).unwrap(), "gatos puntos lima");
        assert_eq!(
            f(1234.56, 2).unwrap(),
            "usa libo duha gatos ug katloan ug upat puntos lima unom"
        );
        assert_eq!(f(-0.5, 1).unwrap(), "minus siro puntos lima");
        assert_eq!(f(-1.5, 1).unwrap(), "minus usa puntos lima");
        assert_eq!(f(-12.34, 2).unwrap(), "minus napulo ug duha puntos tulo upat");
        assert_eq!(f(1.005, 3).unwrap(), "usa puntos siro siro lima");
        assert_eq!(f(2.675, 3).unwrap(), "duha puntos unom pito lima");
    }

    /// Every `"to": "cardinal_dec"` row in bench/corpus.jsonl.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(d("0.01", 2).unwrap(), "siro puntos siro usa");
        // The trailing "siro" is the point of this row: Decimal("1.10") keeps
        // its written scale, which the float 1.1 could never express.
        assert_eq!(d("1.10", 2).unwrap(), "usa puntos usa siro");
        assert_eq!(d("12.345", 3).unwrap(), "napulo ug duha puntos tulo upat lima");
        // Issue #603: the exact-Decimal arm at trillion scale. The integer part
        // exceeds 10^9, so bug 1 hands back bare digits.
        assert_eq!(
            d("98746251323029.99", 2).unwrap(),
            "98746251323029 puntos siyam siyam"
        );
        assert_eq!(d("0.001", 3).unwrap(), "siro puntos siro siro usa");
    }

    /// The f64-artefact cases. CEB reads `str()`, not `float2tuple`, so the
    /// repr's digits are used verbatim — no `10**precision`, no `< 0.01` rescue.
    #[test]
    fn f64_artefacts_come_from_repr_not_float2tuple() {
        assert_eq!(f(1.005, 3).unwrap(), "usa puntos siro siro lima");
        assert_eq!(f(2.675, 3).unwrap(), "duha puntos unom pito lima");
        // repr keeps all 17 fractional digits of 0.1+0.2 ("0.30000000000000004")
        // and every one of them becomes a word — no rounding, no truncation.
        // Built rather than spelled out so the zero count cannot drift.
        let expected = format!("siro puntos tulo {} upat", ["siro"; 15].join(" "));
        assert_eq!(f(0.1 + 0.2, 17).unwrap(), expected);
    }

    /// `precision=` sets `converter.precision`, which CEB never reads.
    #[test]
    fn precision_override_is_inert() {
        let l = LangCeb::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        for over in [None, Some(0), Some(1), Some(2), Some(5), Some(17)] {
            assert_eq!(
                l.to_cardinal_float(&v, over).unwrap(),
                "duha puntos unom pito lima"
            );
        }
    }

    /// Bug 6: a repr in exponent form reaches `int()` and raises.
    #[test]
    fn exponent_form_repr_raises_value_error() {
        // 1e15 still prints positionally, so it survives (as digits, by bug 1).
        assert_eq!(f(1e15, 1).unwrap(), "1000000000000000 puntos siro");
        // One decade up, decpt > 16 and repr flips.
        assert_eq!(
            value_err(f(1e16, 16)),
            "invalid literal for int() with base 10: '1e+16'"
        );
        assert_eq!(
            value_err(f(1e21, 21)),
            "invalid literal for int() with base 10: '1e+21'"
        );
        // 1e-4 prints positionally; 1e-5 does not.
        assert_eq!(f(1e-4, 4).unwrap(), "siro puntos siro siro siro usa");
        assert_eq!(
            value_err(f(1e-5, 5)),
            "invalid literal for int() with base 10: '1e-05'"
        );
        // '1.5e+20' *does* contain a ".", so it takes the fraction branch and
        // dies on the single char 'e' after already emitting "lima" for the 5.
        assert_eq!(value_err(f(1.5e20, 19)), "invalid literal for int() with base 10: 'e'");
        // Decimal's own scientific form raises the same way.
        assert_eq!(
            value_err(d("1E+2", 2)),
            "invalid literal for int() with base 10: '1E+2'"
        );
        assert_eq!(
            value_err(d("1E-7", 7)),
            "invalid literal for int() with base 10: '1E-7'"
        );
    }

    /// A Decimal with no fractional part never reaches the pointword branch.
    #[test]
    fn integral_decimal_takes_the_int_branch() {
        assert_eq!(d("100", 0).unwrap(), "gatos");
        assert_eq!(d("1", 0).unwrap(), "usa");
        assert_eq!(d("-1", 0).unwrap(), "minus usa");
        // ...but Decimal("5.00") does, and prints both zeros.
        assert_eq!(d("5.00", 2).unwrap(), "lima puntos siro siro");
        assert_eq!(d("0.0", 1).unwrap(), "siro puntos siro");
    }

    /// `str(-0.0) == "-0.0"`, so the sign bit alone produces "minus".
    #[test]
    fn negative_zero_float_keeps_its_sign() {
        assert_eq!(f(-0.0, 1).unwrap(), "minus siro puntos siro");
        assert_eq!(f(0.0, 1).unwrap(), "siro puntos siro");
    }

    /// [`py_str_f64`] against CPython's `repr`, including the tie-to-even cases
    /// that Rust's `{:e}` alone gets wrong.
    #[test]
    fn py_str_f64_matches_cpython_repr() {
        assert_eq!(py_str_f64(1.0), "1.0");
        assert_eq!(py_str_f64(-0.0), "-0.0");
        assert_eq!(py_str_f64(0.0), "0.0");
        assert_eq!(py_str_f64(100.0), "100.0");
        assert_eq!(py_str_f64(0.5), "0.5");
        assert_eq!(py_str_f64(2.675), "2.675");
        assert_eq!(py_str_f64(1e15), "1000000000000000.0");
        assert_eq!(py_str_f64(1e16), "1e+16");
        assert_eq!(py_str_f64(1e-4), "0.0001");
        assert_eq!(py_str_f64(1e-5), "1e-05");
        assert_eq!(py_str_f64(1.5e20), "1.5e+20");
        assert_eq!(py_str_f64(5e-324), "5e-324");
        assert_eq!(py_str_f64(1e100), "1e+100");
        assert_eq!(py_str_f64(2e16 + 8.0), "2.000000000000001e+16");
        assert_eq!(py_str_f64(f64::INFINITY), "inf");
        assert_eq!(py_str_f64(f64::NEG_INFINITY), "-inf");
        assert_eq!(py_str_f64(f64::NAN), "nan");
        // Exact ties. The doubles below are `...25` / `...125` exactly, so both
        // neighbours round-trip; CPython picks the even last digit and Rust's
        // shortest `{:e}` would pick the odd one.
        assert_eq!(py_str_f64(670352580196876.25), "670352580196876.2");
        assert_eq!(py_str_f64(161834668665500.125), "161834668665500.12");
        assert_eq!(py_str_f64(9930461221140.8125), "9930461221140.812");
        assert_eq!(py_str_f64(-254209913874244.625), "-254209913874244.62");
    }

    /// [`py_str_decimal`] against CPython's `Decimal.__str__`, especially where
    /// `BigDecimal`'s own `Display` disagrees.
    #[test]
    fn py_str_decimal_matches_cpython_str() {
        let s = |x: &str| py_str_decimal(&BigDecimal::from_str(x).unwrap());
        assert_eq!(s("1.10"), "1.10");
        assert_eq!(s("98746251323029.99"), "98746251323029.99");
        assert_eq!(s("100"), "100");
        assert_eq!(s("100.00"), "100.00");
        // Display would say "0"; Python says "0.0", and CEB's output depends
        // on the difference.
        assert_eq!(s("0.0"), "0.0");
        assert_eq!(s("0.00"), "0.00");
        // Display would say "100"; Python keeps the exponent, and CEB raises.
        assert_eq!(s("1E+2"), "1E+2");
        assert_eq!(s("0E+2"), "0E+2");
        // Display would say "1e+16" — lowercase.
        assert_eq!(s("1E+16"), "1E+16");
        // The positional/scientific boundary: exponent <= 0 and adjusted > -6.
        assert_eq!(s("0.000001"), "0.000001");
        assert_eq!(s("0.0000001"), "1E-7");
        // Decimal("1E-7") and Decimal("0.0000001") are the same object.
        assert_eq!(s("1E-7"), "1E-7");
        assert_eq!(s("-0.5"), "-0.5");
    }
}
