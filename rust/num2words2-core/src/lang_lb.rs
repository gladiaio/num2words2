//! Port of `lang_LB.py` (Luxembourgish).
//!
//! Shape: **self-contained**. `Num2Word_LB` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, and
//! `Num2Word_Base` declares none at class level either. The `hasattr` guard in
//! `Num2Word_Base.__init__` therefore never fires, so Python never builds
//! `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden outright
//! and drives `_int_to_word`, a plain recursive descent over the
//! ones/tens/hundred/thousand/million scale.
//!
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here, and
//! there is **no overflow check at all** — see bug 5 below for what happens
//! instead of an `OverflowError`.
//!
//! `setup()` only assigns `negword`/`pointword`/`ones`/`tens`/`hundred`/
//! `thousand`/`million`; those are the whole of the language data.
//!
//! Inherited from `Num2Word_Base` — but note LB overrides `to_year` itself:
//!   * `to_year(val, longval=True) -> self.to_cardinal(val)` — LB's own
//!     override, which discards `longval` and ignores the elaborate
//!     century/"hundred"-splitting logic in `Num2Word_Base.to_year`. Years are
//!     therefore plain cardinals: `to_year(1999)` == "eent dausend néng honnert
//!     nongzeg néng", **not** a "nineteen ninety-nine"-style reading.
//!   * `to_ordinal_num` is overridden by LB as `str(number) + "."`.
//!
//! No cross-call mutable state: every method here is a pure function of its
//! argument. There is no `str_to_number`/`_pending_ordinal`-style handshake.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below looks wrong and is exactly
//! what Python emits — each one is confirmed against the frozen corpus.
//!
//! 1. **Zero and the decimal word (fixed, gladiaio/num2words2#154).**
//!    Python's `_int_to_word` opens with
//!    `return self.ones[0] if self.ones[0] else "zero"`. `self.ones[0]` is the
//!    empty string — a placeholder so that `ones[n]` can be indexed by digit —
//!    and `""` is falsy, so Python always answered with the English "zero";
//!    its `pointword` is the English "point". This port says the
//!    Luxembourgish "null" and "Komma" (a decimal comma, capitalised as a noun
//!    like "Millioun"): `to_cardinal(0)` == "null", `1.5` == "eent Komma
//!    fënnef". Modelled by the unconditional [`ZERO`] return in
//!    [`LangLb::int_to_word`].
//! 2. **There are no teens.** `tens[1]` is "zéng" (10) and 11..=19 are built by
//!    the generic `tens[n/10] + " " + ones[n%10]` rule, so `to_cardinal(11)` ==
//!    "zéng eent" ("ten one") rather than "eelef", and `to_cardinal(19)` ==
//!    "zéng néng". This propagates: `to_cardinal(12345)` == "zéng zwou dausend
//!    dräi honnert véierzeg fënnef", because the thousands part 12 is itself
//!    rendered "zéng zwou".
//! 3. **Hundreds/thousands/millions are never elided or joined.** Python emits
//!    `ones[h] + " " + hundred` unconditionally, so 100 == "eent honnert"
//!    ("one hundred", where idiomatic Luxembourgish is "honnert"), 1000 ==
//!    "eent dausend", 10^6 == "eent Millioun". Every scale word is
//!    space-separated rather than compounded, which is not how Luxembourgish
//!    writes numbers.
//! 4. **"zwou" is the feminine form of 2**, used unconditionally — so 2, 200
//!    ("zwou honnert") and 2000 ("zwou dausend") all take the feminine, and the
//!    masculine "zwee" never appears. Likewise "eent" is the neuter/counting
//!    form used even attributively ("eent honnert").
//! 5. **Values >= 10^9 silently degrade to digits** instead of raising.
//!    `_int_to_word`'s final `else` is `return str(number)  # Fallback for very
//!    large numbers`. So `to_cardinal(10**9)` == "1000000000" and
//!    `to_cardinal(10**21)` == "1000000000000000000000" — a *successful* call
//!    returning an unconverted numeral, not an `OverflowError`. "Milliard" and
//!    everything above it are simply absent from the language data. The scale
//!    words stop at `million` = "Millioun", so 999999999 is the largest value
//!    that produces words. This also means the fallback composes with the sign:
//!    `to_cardinal(-10**12)` == "minus 1000000000000".
//! 6. **`to_ordinal` is a blind suffix append**, `to_cardinal(n) + "-ten"`,
//!    with no stem change, no agreement and no special-casing whatsoever. It
//!    inherits every quirk above and adds its own:
//!      * `to_ordinal(0)` == "null-ten".
//!      * `to_ordinal(-1)` == "minus eent-ten" — negatives are cheerfully
//!        ordinalised, where most modules raise
//!        "Cannot treat negative num %s as ordinal".
//!      * `to_ordinal(10**12)` == "1000000000000-ten" — bug 5 leaks through,
//!        yielding digits with a Luxembourgish suffix glued on.
//!      * The suffix attaches to the *last word only* with no separator, so
//!        `to_ordinal(999)` == "néng honnert nongzeg néng-ten".
//!    Real Luxembourgish ordinals are "éischten", "zweeten", "drëtten", …;
//!    none of that is implemented.
//! 7. **`to_ordinal_num` ignores the sign**: `str(number) + "."` gives
//!    `to_ordinal_num(-1)` == "-1.", a negative ordinal numeral.
//!
//! # Currency
//!
//! `Num2Word_LB` declares its own `CURRENCY_FORMS` — only `EUR` and `USD` —
//! and overrides `to_currency` wholesale. It subclasses `Num2Word_Base`, **not**
//! `Num2Word_EUR`, so the class dict that `Num2Word_EN.__init__` mutates in
//! place (rewriting EUR to `("euro", "euros")` and adding ~24 codes) is a
//! *different* dict and never reaches here. The literal in `lang_LB.py` is what
//! runs, verified against the live interpreter:
//!
//! ```text
//! {'EUR': (('euro', 'euro'), ('cent', 'cents')),
//!  'USD': (('dollar', 'dollars'), ('cent', 'cents'))}
//! ```
//!
//! Hence `to_currency(2)` == "zwou euro", not "zwou euros" — EUR's plural form
//! is spelled identically to its singular in LB's own table.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited from
//! `Num2Word_Base` and are both `{}`, so `currency_adjective` stays `None` and
//! `currency_precision` stays at the trait's default 100 for *every* code.
//! `pluralize`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are all
//! inherited untouched; LB's `to_currency` calls **none** of them, so only
//! `_money_verbose` is ever reached, and only via the inherited `to_cheque`.
//!
//! `to_cheque` is **not** overridden by LB, so `Num2Word_Base.to_cheque` runs
//! unchanged and the trait default in `currency.rs` serves it. It needs nothing
//! from this file beyond [`Lang::lang_name`] and [`Lang::currency_forms`].
//!
//! # Faithfully reproduced Python bugs, currency edition
//!
//! 8. **An unknown currency code silently becomes EUR.** `to_currency` looks up
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with the *first dict value* as its default, not a `[]`. So
//!    `to_currency(0.5, currency="JPY")` == "null euro fofzeg cents" and
//!    `to_currency(12.34, currency="KWD")` == "zéng zwou euro drësseg véier
//!    cents": every unimplemented code renders as euro, and no code ever raises
//!    from `to_currency`. The corpus records this for GBP, JPY, KWD, BHD, INR,
//!    CNY and CHF alike. Python's reliance on dict insertion order to mean
//!    "EUR" is modelled by the explicit [`LangLb::fallback_forms`] field.
//! 9. **`to_cheque` disagrees with `to_currency` about the same code.** The
//!    inherited `to_cheque` does a strict `self.CURRENCY_FORMS[currency]` inside
//!    a `try/except KeyError` and re-raises `NotImplementedError`. So `GBP`
//!    *raises* on the cheque surface while rendering happily as euro on the
//!    currency surface. Both behaviours are real and both are in the corpus.
//! 10. **Cents are truncated to two digits, never rounded.** `parts[1][:2]`
//!    slices the fractional *string*: `1.005` -> parts[1] "005" -> "00" -> 0
//!    cents -> "eent euro", and `12.345` -> "34" -> "drësseg véier cents".
//!    Nothing here rounds, so 1.005 loses its half-cent instead of becoming 1
//!    cent.
//! 11. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** The
//!    divisor is hard-wired to 100 by the `[:2].ljust(2, "0")` slice. JPY (a
//!    zero-decimal currency) therefore gets cents, and KWD/BHD (three-decimal)
//!    get two — though bug 8 means both render as euro anyway.
//! 12. **`adjective` is accepted and completely ignored.** LB's signature takes
//!    it, then never reads it. Modelled by the `_adjective` binding.
//! 13. **`cents=False` drops the cents segment entirely.** `if cents and right`
//!    gates the whole clause, where `Num2Word_Base` would fall through to
//!    `_cents_terse` and emit digits. `to_currency(12.34, cents=False)` ==
//!    "zéng zwou euro" — the 34 cents are gone, not abbreviated.
//! 14. **Zero cents suppress the segment even for a float.** `right` is `0`,
//!    which is falsy, so `1.0` renders "eent euro" with no "null cents" tail —
//!    the same string a true `int` 1 produces. LB never calls
//!    `isinstance(val, int)`, so unlike every `Num2Word_Base` descendant the
//!    int/float split is invisible here. [`CurrencyValue`]'s two arms are still
//!    handled separately because the *stringification* differs.
//! 15. **The singular/plural choice reads the integer part only.** `cr1[1] if
//!    left != 1 else cr1[0]`, so `1.5` takes the singular ("eent euro"-style
//!    agreement) while the cents clause tests `right != 1` independently.
//!
//! # Float/Decimal entry routing (the `"." in str(number)` rule)
//!
//! `to_cardinal` never asks whether the value is *whole* — it asks whether
//! the *string* shows a point. That routing is reproduced by
//! [`Lang::cardinal_float_entry`] here (and, through it, the ordinal/year
//! entries), with these pinned consequences:
//!
//! 16. **Whole floats keep their ".0" tail.** `str(5.0)` == "5.0" has a
//!     point, so `to_cardinal(5.0)` == "fënnef Komma null", *not* "fënnef".
//!     Likewise `Decimal("5.00")` == "fënnef Komma null null" — every
//!     fractional character is spelled via `_int_to_word(int(digit))`,
//!     trailing zeros included (each '0' is "null", bug 1).
//! 17. **`-0.0` renders the negword.** The sign is read off the string
//!     (`str(-0.0)` == "-0.0" starts with "-"), so `to_cardinal(-0.0)` ==
//!     "minus null Komma null". A `< 0` test would miss it.
//! 18. **Scientific notation is a `ValueError`.** `str(1e16)` == "1e+16" has
//!     no point, so Python runs `int("1e+16")` and dies: `invalid literal
//!     for int() with base 10: '1e+16'`. Same for `1e+20`, tiny floats
//!     (`str(1e-05)` == "1e-05") and non-canonical Decimals
//!     (`str(Decimal("1E+2"))` == "1E+2"). The sign is stripped *before* the
//!     `int()` (`n = n[1:]`), so the message never shows a minus.
//! 19. **Point-less integral Decimals take the integer grammar.**
//!     `Decimal("100")` stringifies as "100" — no point — and `int("100")`
//!     succeeds, so it renders "eent honnert" like the int would.
//! 20. `to_ordinal(float)` is `to_cardinal(float) + "-ten"` ("fënnef Komma
//!     null-ten"); `to_ordinal_num(float)` is `str(number) + "."` ("5.0.",
//!     "1e+16." — nothing is parsed, so no ValueError there); `to_year(float)`
//!     is `to_cardinal(float)` (the trait default already routes through the
//!     override).
//! 21. **`"Infinity"`/`"NaN"` strings fall back to Python.**
//!     `Decimal("Infinity")` parses fine and dies later in `int("Infinity")`
//!     (ValueError) — but only on the modes that parse;
//!     `to_ordinal_num("Infinity")` happily returns "Infinity.".
//!     `ParsedNumber` cannot carry Inf/NaN into that split, so
//!     `str_to_number` returns NotImplemented and the shim reruns the
//!     original pure-Python path, reproducing every mode exactly.
//!
//! # Error variants
//!
//! For *integer* input the module cannot raise: there is no overflow check
//! (bug 5 replaces it with a digit fallback), no dict lookup and no `int()`
//! of a hostile token. `to_currency` cannot raise either, because bug 8 turns
//! every missing code into EUR. The errors that do exist:
//!
//! * The `NotImplementedError` that the *inherited* `to_cheque` raises for a
//!   code outside `{EUR, USD}` — `currency.rs`'s `default_to_cheque` already
//!   produces it from [`Lang::currency_forms`] returning `None`.
//! * The `ValueError` of quirk 18 above, on the float/Decimal entries only.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `_int_to_word`'s zero case. Python writes
/// `self.ones[0] if self.ones[0] else "zero"`, but `ones[0]` is `""` (falsy),
/// so Python's English word is the only reachable result; this port says
/// "null". See module bug 1.
const ZERO: &str = "null";

/// `self.pointword`: English "point" in Python, "Komma" here (bug 1, #154).
const POINTWORD: &str = "Komma";

/// `self.negword`. The trailing space is Python's and is load-bearing:
/// `to_cardinal` builds `negword + word` with no separator of its own.
const NEGWORD: &str = "minus ";

/// `self.ones`, indexed by digit. Index 0 is `""` — a positional placeholder,
/// never emitted: 0 is intercepted by the `ZERO` branch, and the hundreds arm
/// only reaches `ones[h]` for `h` in 1..=9.
const ONES: [&str; 10] = [
    "", "eent", "zwou", "dräi", "véier", "fënnef", "sechs", "siwen", "aacht", "néng",
];

/// `self.tens`, indexed by tens digit. Index 0 is `""` and is unreachable:
/// `tens[t]` is only read when `number >= 10`, i.e. `t` in 1..=9.
///
/// Note `tens[1]` == "zéng" is plain 10; there is no teens table, which is the
/// root of module bug 2.
const TENS: [&str; 10] = [
    "",
    "zéng",
    "zwanzeg",
    "drësseg",
    "véierzeg",
    "fofzeg",
    "sechzeg",
    "siwwenzeg",
    "achtzeg",
    "nongzeg",
];

/// `self.hundred`.
const HUNDRED: &str = "honnert";
/// `self.thousand`.
const THOUSAND: &str = "dausend";
/// `self.million`. Capitalised in the Python source (German-style noun
/// capitalisation) and emitted mid-sentence as-is: "eent Millioun".
const MILLION: &str = "Millioun";

/// The ordinal suffix appended by `to_ordinal`. See module bug 6.
const ORDINAL_SUFFIX: &str = "-ten";

/// `Num2Word_LB.__name__`, for the `NotImplementedError` the inherited
/// `to_cheque` raises.
const LANG_NAME: &str = "Num2Word_LB";

/// The key `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// Python's dict preserves insertion order and `lang_LB.py` writes `EUR` first,
/// so the `.get(currency, <first value>)` default in `to_currency` is the EUR
/// entry. Naming the code makes that ordering dependency explicit instead of
/// leaving it to a `HashMap`, which has no order to depend on. See bug 8.
const FALLBACK_CODE: &str = "EUR";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_LB.to_currency` declares `separator=" "` in **its own** signature,
/// overriding `Num2Word_Base`'s `separator=","`. But the `Lang` trait takes the
/// separator as a plain argument, and both callers — `num2words2/__init__.py`'s
/// fast path and `bench/diff_test.py` — substitute `kwargs.get("separator",
/// ",")`, i.e. *base's* default, before the value crosses the boundary. By the
/// time it arrives, "caller omitted separator" and "caller explicitly asked for
/// a comma" are the same four bytes and cannot be told apart here.
///
/// The corpus settles which way to resolve the ambiguity: all 54 of its
/// cents-bearing LB rows come from `num2words(v, lang="lb", to="currency",
/// currency=c)` with no `separator=`, and every one expects a bare space
/// ("zéng zwou euro drësseg véier cents"). So a received `","` is read as
/// "unset" and LB's own default restored.
///
/// This is right for the overwhelmingly common caller (no `separator=`) and for
/// every caller who passes anything other than `","`, and wrong only for an
/// explicit `separator=","`, which Python renders "zéng zwou euro,drësseg véier
/// cents" and this renders with a space. Fixing that properly means teaching
/// the binding each language's own default; it cannot be fixed from this file.
/// Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// LB's own `to_currency` default, restored when [`SEPARATOR_UNSET`] arrives.
const SEPARATOR_DEFAULT: &str = " ";

/// Narrow a `BigInt` to a table index.
///
/// Only ever called on a value proven to be in 0..=9 by the enclosing range
/// check (`number < 10`, or a quotient/remainder of a division by 10 or 100
/// under a matching bound), so the conversion cannot fail.
fn idx(n: &BigInt) -> usize {
    debug_assert!(!n.is_negative() && n <= &BigInt::from(9), "digit out of range");
    n.to_usize().expect("digit index is bounded by construction")
}

/// Python's
///
/// ```python
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// evaluated on the already-absolute value, returning `(left, right)`.
///
/// `to_currency` is string-driven: it never does arithmetic on the fraction, it
/// *slices the decimal text*. The `CurrencyValue` we get was parsed from exactly
/// the `str(number)` the Python shim produced, and `BigDecimal` keeps the
/// literal's mantissa and scale without normalising (`"1.0"` stays mantissa 10 /
/// scale 1, `"100"` stays mantissa 100 / scale 0), so the digits Python sliced
/// are recoverable from `as_bigint_and_exponent` and the slice is reproduced
/// arithmetically:
///
/// * `parts[0]` is `mantissa / 10^scale`.
/// * `parts[1]` is `mantissa % 10^scale`, left-padded with zeros to `scale`
///   digits — that padding is why `0.01` yields "01" and not "1".
/// * `parts[1][:2].ljust(2, "0")` keeps the leading two of those `scale` digits,
///   which for `scale >= 2` is a floor-divide by `10^(scale - 2)` and for
///   `scale == 1` is a multiply by 10 (the `ljust` appending the missing digit).
///   Doing it as division avoids formatting a string only to re-parse it, and
///   removes a parse error path that could not fire.
///
/// Truncation, not rounding, is the point — see module bug 10.
///
/// An `Int` never has a `"."` in its `str()`, so it takes `right = 0` with no
/// fraction handling at all. That is the *only* way the int/float split shows
/// up here (bug 14), and it agrees with the `Decimal` arm for whole values.
fn split_str_parts(val: &CurrencyValue) -> (BigInt, BigInt) {
    match val {
        // `str(5)` == "5" -> parts == ["5"] -> the `len(parts) > 1` guard fails.
        CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
        CurrencyValue::Decimal { value: d, .. } => {
            // Python does `val = abs(val)` *before* `str(val)`, so the sign is
            // gone by the time the text is split.
            let (mantissa, scale) = d.abs().as_bigint_and_exponent();

            if scale <= 0 {
                // No fractional digits in the literal, so no "." and no
                // `parts[1]`. `str()` writes this as plain digits, e.g.
                // Decimal("100") -> "100" -> left = 100.
                let pow = BigInt::from(10).pow((-scale) as u32);
                return (mantissa * pow, BigInt::zero());
            }

            let pow = BigInt::from(10).pow(scale as u32);
            // Non-negative dividend, so `div_rem` and Python's `//`/`%` agree.
            let (left, frac) = mantissa.div_rem(&pow);
            let right = if scale >= 2 {
                // Keep the first two of `scale` zero-padded digits.
                frac / BigInt::from(10).pow((scale - 2) as u32)
            } else {
                // scale == 1: one digit, and `ljust(2, "0")` appends the second.
                frac * BigInt::from(10)
            };
            (left, right)
        }
    }
}

/// Python's `int(s)` failure, message verbatim from CPython. The `s` is the
/// point-less `str(number)` slice LB feeds it — sign already stripped, because
/// `to_cardinal` peels the "-" off the string (`n = n[1:]`) before `int()`.
fn int_value_error(s: &str) -> N2WError {
    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
}

/// Python `repr(float)` for the scientific range, on an already-`abs()`ed
/// finite f64 (`a >= 1e16`, or `0 < a < 1e-4`).
///
/// Rust's `{:e}` yields the shortest round-trip mantissa — the same digits
/// CPython's repr picks — but lays the exponent out as `1e16` / `1e-5` where
/// Python writes `1e+16` / `1e-05`: always a sign, zero-padded to two digits.
/// Only the exponent dressing needs fixing up.
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

pub struct LangLb {
    /// `Num2Word_LB.CURRENCY_FORMS`, built once in [`LangLb::new`].
    ///
    /// The registry holds each language in a `OnceLock` and hands out a
    /// `&'static dyn Lang`, so `new()` runs at most once per process and every
    /// `to_currency` call reads this table rather than rebuilding it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the [`FALLBACK_CODE`] entry,
    /// which `to_currency` substitutes for any code it does not know (bug 8).
    ///
    /// Held separately rather than looked up through `currency_forms` so the
    /// hot path has no `unwrap`/`expect` and no way to panic across the pyo3
    /// boundary: `new()` is the single place that has to keep the two in step.
    fallback_forms: CurrencyForms,
}

impl Default for LangLb {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLb {
    pub fn new() -> Self {
        // `lang_LB.py`'s class-level literal, in source order. LB subclasses
        // Num2Word_Base and declares this itself, so `Num2Word_EN.__init__`'s
        // in-place mutation of `Num2Word_EUR.CURRENCY_FORMS` — which rewrites
        // EUR to ("euro", "euros") and injects ~24 extra codes — does not touch
        // it. Confirmed against the live interpreter: EUR's plural really is
        // "euro", and GBP/JPY/KWD/... really are absent.
        let mut currency_forms = HashMap::with_capacity(2);
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euro"], &["cent", "cents"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );

        let fallback_forms = currency_forms
            .get(FALLBACK_CODE)
            .cloned()
            .expect("FALLBACK_CODE was just inserted above");

        LangLb {
            currency_forms,
            fallback_forms,
        }
    }

    /// Python's `_int_to_word`.
    ///
    /// Infallible: the `else` arm swallows everything at or above 10^9 by
    /// returning `str(number)` (module bug 5), so there is no error path.
    fn int_to_word(&self, number: &BigInt) -> String {
        // Python: `if number == 0: return self.ones[0] if self.ones[0] else "zero"`.
        // `ones[0]` is "" (falsy) so the ternary is constant — see bug 1.
        if number.is_zero() {
            return ZERO.to_string();
        }

        // Python: `if number < 0: return self.negword + self._int_to_word(abs(number))`.
        //
        // Unreachable from any in-scope entry point — `to_cardinal` strips the
        // sign from the *string* before parsing, so `_int_to_word` only ever
        // sees a non-negative value. Reproduced for fidelity with the source.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1_000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        if number < &ten {
            return ONES[idx(number)].to_string();
        }

        if number < &hundred {
            // Positive dividend, so floor- and trunc-division agree; this
            // matches Python's `//` and `%` exactly.
            let (tens_val, ones_val) = number.div_mod_floor(&ten);
            if ones_val.is_zero() {
                return TENS[idx(&tens_val)].to_string();
            }
            return format!("{} {}", TENS[idx(&tens_val)], ONES[idx(&ones_val)]);
        }

        if number < &thousand {
            let (hundreds_val, remainder) = number.div_mod_floor(&hundred);
            // Python indexes `ones[hundreds_val]` directly rather than
            // recursing, and never elides the "eent" for 100 — see bug 3.
            let mut result = format!("{} {}", ONES[idx(&hundreds_val)], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &million {
            let (thousands_val, remainder) = number.div_mod_floor(&thousand);
            // `thousands_val` < 1000 here, so this recursion lands in one of
            // the arms above and never re-enters the digit fallback.
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &billion {
            let (millions_val, remainder) = number.div_mod_floor(&million);
            // `millions_val` < 1000 here — same reasoning as above.
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        // Python: `return str(number)  # Fallback for very large numbers`.
        // A successful call returning bare digits — see bug 5.
        number.to_string()
    }
}

impl Lang for LangLb {
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

    /// `self.pointword`. Unreachable from the integer paths: Python only
    /// consults it in `to_cardinal`'s `"." in n` branch, and an integer's
    /// rendering never contains a point.
    fn pointword(&self) -> &str {
        POINTWORD
    }


    /// Python's `to_cardinal`.
    ///
    /// The original is string-driven: `n = str(number).strip()`, then it peels
    /// a leading "-" off the *text* and re-parses the rest with `int(n)`. For
    /// `BigInt` input that round-trip is exactly `value.abs()`, and the
    /// `.strip()` is a no-op (an integer's decimal rendering carries no
    /// whitespace), so both are folded away here.
    ///
    /// The `"." in n` branch handles float input and is out of scope: an
    /// integer's rendering never contains a point.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };

        // Python's trailing `.strip()`. A no-op in practice — `_int_to_word`
        // never returns "" (0 yields "null") nor pads its output — but kept so
        // the port has no behaviour of its own.
        Ok(format!("{}{}", ret, self.int_to_word(&value.abs()))
            .trim()
            .to_string())
    }

    /// Python's `to_ordinal`: `self.to_cardinal(number) + "-ten"`, unguarded.
    /// Accepts 0 and negatives and digit-fallback output alike — see bug 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`, sign included — bug 7.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)`: `return self.to_cardinal(val)`.
    ///
    /// `longval` is accepted and discarded, and `Num2Word_Base.to_year`'s
    /// century logic is bypassed entirely. This happens to coincide with the
    /// trait default, but is spelled out because it is a real override in the
    /// source, not an inheritance.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// LB's float/Decimal path. **`Num2Word_LB` does not use the base
    /// `to_cardinal_float`/`float2tuple` machinery at all** — it overrides
    /// `to_cardinal` and handles a non-integer inline by slicing the *string*
    /// form of the value:
    ///
    /// ```python
    /// def to_cardinal(self, number):
    ///     n = str(number).strip()
    ///     if n.startswith("-"):
    ///         n = n[1:]
    ///         ret = self.negword          # "minus "
    ///     else:
    ///         ret = ""
    ///     if "." in n:
    ///         left, right = n.split(".", 1)
    ///         ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///         for digit in right:
    ///             ret += self._int_to_word(int(digit)) + " "
    ///         return ret.strip()
    ///     else:
    ///         return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// The base `default_to_cardinal_float` happens to agree with this for
    /// every value in the frozen corpus — but it derives the fractional digits
    /// from `float2tuple`'s **lossy** `abs(value - pre) * 10**precision`, and
    /// that parts ways with LB's *string* slice in two places:
    ///
    /// * **Sign of zero.** `str(-0.0)` is `"-0.0"`, which starts with `"-"`, so
    ///   LB emits `"minus null Komma null"`; the base path keys the sign off
    ///   `value < 0` (false for `-0.0`) and drops the minus.
    /// * **Large-magnitude floats.** For `588758963.044982` the f64 subtraction
    ///   loses low bits and `abs(v-pre)*1e6` lands ~0.1 off an integer, so the
    ///   base heuristic *floors* to `44981` — last digit `eent`. But
    ///   `repr(588758963.044982)` is the exact shortest round-trip
    ///   `"588758963.044982"`, so LB reads `...044982` — last digit `zwou`. LB
    ///   never does that arithmetic; it slices `str(number)`.
    ///
    /// So this override reconstructs LB's `str(number)` rather than reusing
    /// `float2tuple` for the Float arm. `{:.precision}` on the f64 reproduces
    /// Python's repr fractional digits (both round half-to-even at the last
    /// place, and `precision` is itself derived from `repr` on the Python side).
    /// The Decimal arm keeps `float2tuple`, whose Decimal branch is *exact*
    /// arbitrary precision and therefore already equals `str(Decimal)`.
    ///
    /// The sign is LB's own string-sign rule: `is_sign_negative()` for a float
    /// (true for `-0.0`) and `is_negative()` for a Decimal.
    ///
    /// `precision_override` is discarded: LB's `to_cardinal` takes no
    /// `precision` kwarg and never reads `self.precision`, so `num2words(...,
    /// precision=k)` has no effect on LB (verified against the live
    /// interpreter). See the port report for what this does *not* reproduce:
    /// scientific-notation floats (`1e16`, `1e-5`), non-canonical Decimals
    /// (`Decimal("1E+2")`) and inf/nan, which raise `ValueError` in Python
    /// (`int("1e+16")`) but are absent from the corpus and unreachable through
    /// the real `num2words` dispatch (which routes floats to the Python
    /// converter, never here).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let precision = value.precision() as usize;

        // Reproduce LB's `n = str(number)`, split into the integer magnitude
        // (`int(left)`) and the `precision`-digit fractional string it walks in
        // `for digit in right`. The sign is taken separately from LB's
        // `n.startswith("-")` and prepended once as `negword`.
        let (negative, left, frac): (bool, BigInt, String) = match value {
            FloatValue::Float { value, .. } => {
                // `str(number).startswith("-")` — true for -0.0 as well.
                let negative = value.is_sign_negative();
                // `{:.precision}` on the magnitude reproduces the fractional
                // digits of Python's repr for every non-scientific float.
                let s = format!("{:.*}", precision, value.abs());
                let (left_str, frac) = match s.split_once('.') {
                    Some((l, f)) => (l, f.to_string()),
                    None => (s.as_str(), String::new()),
                };
                let left = left_str.parse::<BigInt>().map_err(|_| {
                    // Unreachable: `{:.*}` of a finite f64 yields only ASCII
                    // digits before the point. An error, not a panic, keeps the
                    // pyo3 boundary safe.
                    N2WError::Value(format!("non-numeric integer part {:?}", left_str))
                })?;
                (negative, left, frac)
            }
            FloatValue::Decimal { value: d, .. } => {
                // `str(Decimal)` is exact; `float2tuple`'s Decimal branch is
                // exact arbitrary precision, so its digits match. `pre` carries
                // the sign of the integer part; LB uses only the magnitude.
                let negative = d.is_negative();
                let (pre, post) = float2tuple(value);
                let post_str = post.to_string();
                // Zero-pad on the left to `precision` — that padding is why
                // `0.01` -> post 1 -> "01" and iterates as digits 0 then 1.
                let frac = format!(
                    "{}{}",
                    "0".repeat(precision.saturating_sub(post_str.len())),
                    post_str
                );
                (negative, pre.abs(), frac)
            }
        };

        let mut ret = String::new();
        if negative {
            // `self.negword` is "minus " — trailing space is load-bearing, it
            // is concatenated straight onto `_int_to_word(int(left))`.
            ret.push_str(NEGWORD);
        }
        // `self._int_to_word(int(left))` — the integer part, unsigned.
        ret.push_str(&self.int_to_word(&left));

        // Python's `"." in n` branch. For a normal (non-scientific) float this
        // is precision >= 1, and for a canonical Decimal it is scale >= 1; the
        // else branch (a whole-valued Decimal such as `Decimal("100")`, which
        // stringifies without a ".") emits no pointword, just the integer.
        if precision > 0 {
            ret.push(' ');
            // `self.pointword` == "Komma", emitted verbatim (LB never titles it;
            // `is_title` is false).
            ret.push_str(self.pointword());
            ret.push(' ');

            // `for digit in right: ret += self._int_to_word(int(digit)) + " "`.
            for ch in frac.chars().take(precision) {
                let d = ch.to_digit(10).ok_or_else(|| {
                    // Unreachable: `frac` is `precision` ASCII digits by
                    // construction. Error, not panic, for boundary safety.
                    N2WError::Value(format!("non-digit {:?} in fractional part", ch))
                })?;
                ret.push_str(&self.int_to_word(&BigInt::from(d)));
                ret.push(' ');
            }
        }

        // Python's `.strip()`.
        Ok(ret.trim().to_string())
    }

    /// `to_cardinal(float/Decimal)` — the full entry, routing on
    /// `"." in str(number)` rather than on whole-ness (module quirks 16-19).
    ///
    /// A whole float therefore keeps its ".0" tail (`5.0` -> "fënnef Komma
    /// null", `-0.0` -> "minus null Komma null"), a point-less integral
    /// Decimal takes the integer grammar (`Decimal("100")` -> "eent
    /// honnert"), and a point-less non-integer form is Python's `int()`
    /// ValueError (`1e+16`, `Decimal("1E+2")`).
    ///
    /// `precision_override` is threaded through untouched; LB's grammar never
    /// reads a precision, so it is inert either way (see `to_cardinal_float`).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        match route_by_str(value)? {
            StrRoute::Point => self.to_cardinal_float(value, precision_override),
            StrRoute::WholeDigits(abs_int) => {
                // Python: `return (ret + self._int_to_word(int(n))).strip()`,
                // where `ret` is negword iff the string carried a "-".
                let ret = if value.is_negative() { NEGWORD } else { "" };
                Ok(format!("{}{}", ret, self.int_to_word(&abs_int))
                    .trim()
                    .to_string())
            }
        }
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-ten"` —
    /// the same blind suffix as the integer path (bug 6), so `5.0` ==
    /// "fënnef Komma null-ten" and the ValueError of `1e+16` propagates
    /// unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. Purely textual —
    /// nothing is parsed, so even the scientific forms succeed: `1e+16` ==
    /// "1e+16.", `Decimal("1E+2")` == "1E+2.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    // `year_float_entry` is deliberately NOT overridden: LB's `to_year` is
    // `self.to_cardinal(val)`, and the trait default routes through the
    // overridden `cardinal_float_entry` above — so `to_year(5.0)` == "fënnef
    // Komma null" and `to_year(1e+16)` raises ValueError, as the corpus pins.

    /// `converter.str_to_number` — Base's `Decimal(value)`, which LB does not
    /// override. Inf/NaN parse fine here; the per-mode ValueError comes later
    /// (`to_cardinal` dies in `int("Infinity")`), which the binding serves via
    /// [`Lang::inf_result`] / [`Lang::nan_result`] below — no Python fallback.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `Decimal('-Infinity')`. LB's `to_cardinal`
    /// stringifies (`str(Decimal('Infinity'))` == "Infinity"), finds no "."
    /// and runs `int("Infinity")` → **ValueError**. The leading "-" of
    /// `-Infinity` is peeled before `int()`, so the message always quotes the
    /// unsigned "Infinity". `to_ordinal` = cardinal + "-ten" raises the same;
    /// only `to_ordinal_num` (pure `str(number) + "."`) succeeds, echoing the
    /// repr. See module quirk 21.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}Infinity.",
                if negative { "-" } else { "" }
            )),
            _ => Err(int_value_error("Infinity")),
        }
    }

    /// `Decimal('NaN')`. Same shape as [`Lang::inf_result`]: `int("NaN")` is a
    /// **ValueError**, except `to_ordinal_num` echoes "NaN.".
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

    /// `Num2Word_LB.CURRENCY_FORMS[code]` — the **strict** subscript.
    ///
    /// This is what the inherited `to_cheque` performs, and returning `None`
    /// for an unknown code is what makes `default_to_cheque` raise
    /// `NotImplementedError('Currency code "GBP" not implemented for
    /// "Num2Word_LB"')`, matching Python byte for byte.
    ///
    /// LB's own `to_currency` deliberately does **not** route through here: it
    /// uses `.get(code, <first value>)` and so never fails (bug 8). Keeping this
    /// hook strict is what lets the two surfaces disagree exactly as they do in
    /// Python (bug 9).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_LB.to_currency`:
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True, separator=" ",
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
    /// A complete override: it shares nothing with `Num2Word_Base.to_currency`
    /// and reaches none of `pluralize`, `_money_verbose`, `_cents_verbose`,
    /// `_cents_terse`, `CURRENCY_PRECISION` or `CURRENCY_ADJECTIVES`. So
    /// `currency::default_to_currency` — with its zero-decimal JPY handling,
    /// its `isinstance(val, int)` branch and its ROUND_HALF_UP quantize — must
    /// not be delegated to; every one of those would change the output.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Python binds `adjective` and never reads it again — see bug 12.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore LB's own `separator=" "` default; see `SEPARATOR_UNSET`.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Python: `if val < 0: is_negative = True; val = abs(val)`. The `abs`
        // is folded into `split_str_parts`, which is the only reader of `val`.
        let is_negative = val.is_negative();
        let (left, right) = split_str_parts(val);

        // Python: `.get(currency, list(self.CURRENCY_FORMS.values())[0])` —
        // an unknown code becomes EUR rather than raising (bug 8).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        // Python indexes `cr1[1]`/`cr1[0]` and `cr2[1]`/`cr2[0]` directly. Both
        // LB entries carry exactly two forms, so the indices are in range by
        // construction — `new()` is the only writer.
        let one = BigInt::one();
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };

        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `cents and right`: `right == 0` is falsy in Python, so zero cents
        // suppress the clause even for a float (bug 14), and `cents=False`
        // suppresses it outright rather than falling back to terse digits
        // (bug 13).
        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        if is_negative {
            // `self.negword` is "minus " — the trailing space is the only gap
            // between the sign and the number.
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's `.strip()`. A no-op for every reachable value (nothing here
        // pads either end), but kept so the port adds no behaviour of its own.
        Ok(result.trim().to_string())
    }
}

#[cfg(test)]
mod entry_routing_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    fn fv(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn dv(s: &str) -> FloatValue {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.max(0) as u32;
        FloatValue::Decimal { value, precision }
    }

    /// `"." in str(number)` routing — corpus_wholefloat rows, verbatim.
    #[test]
    fn corpus_cardinal_entry() {
        let lb = LangLb::new();
        // Whole floats keep their ".0" tail.
        assert_eq!(
            lb.cardinal_float_entry(&fv(5.0, 1), None).unwrap(),
            "fënnef Komma null"
        );
        assert_eq!(
            lb.cardinal_float_entry(&fv(0.0, 1), None).unwrap(),
            "null Komma null"
        );
        assert_eq!(
            lb.cardinal_float_entry(&fv(-1000000.0, 1), None).unwrap(),
            "minus eent Millioun Komma null"
        );
        // -0.0: the sign lives in the *string*, so the negword survives.
        assert_eq!(
            lb.cardinal_float_entry(&fv(-0.0, 1), None).unwrap(),
            "minus null Komma null"
        );
        // The >= 10^9 digit fallback composes with the ".0" tail (bug 5).
        assert_eq!(
            lb.cardinal_float_entry(&fv(1e9, 1), None).unwrap(),
            "1000000000 Komma null"
        );
        // Decimals: trailing zeros of the literal are all spelled "null".
        assert_eq!(
            lb.cardinal_float_entry(&dv("5.00"), None).unwrap(),
            "fënnef Komma null null"
        );
        assert_eq!(
            lb.cardinal_float_entry(&dv("12345.000"), None).unwrap(),
            "zéng zwou dausend dräi honnert véierzeg fënnef Komma null null null"
        );
        // Point-less integral Decimals take the integer grammar.
        assert_eq!(lb.cardinal_float_entry(&dv("0"), None).unwrap(), "null");
        assert_eq!(
            lb.cardinal_float_entry(&dv("100"), None).unwrap(),
            "eent honnert"
        );
    }

    /// Scientific repr has no "." -> `int(n)` ValueError, message verbatim.
    #[test]
    fn corpus_cardinal_entry_scientific_is_valueerror() {
        let lb = LangLb::new();
        for (v, repr) in [(1e16, "1e+16"), (1e20, "1e+20"), (1e-5, "1e-05")] {
            match lb.cardinal_float_entry(&fv(v, 0), None) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", repr)
                ),
                other => panic!("{} expected ValueError, got {:?}", v, other),
            }
        }
        // Negative: the string sign is stripped before int(), so no minus.
        match lb.cardinal_float_entry(&fv(-1e16, 0), None) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // #211: str(Decimal) is written out ("100"), so these read.
        for (s, n) in [("1E+2", 2u32), ("1E+20", 20), ("1E+3", 3)] {
            assert_eq!(
                lb.cardinal_float_entry(&dv(s), None).unwrap(),
                lb.to_cardinal(&BigInt::from(10).pow(n)).unwrap(),
                "{}", s
            );
        }
    }

    /// `to_ordinal(float)` = cardinal + "-ten"; errors propagate unchanged.
    #[test]
    fn corpus_ordinal_entry() {
        let lb = LangLb::new();
        assert_eq!(
            lb.ordinal_float_entry(&fv(5.0, 1)).unwrap(),
            "fënnef Komma null-ten"
        );
        assert_eq!(
            lb.ordinal_float_entry(&fv(-0.0, 1)).unwrap(),
            "minus null Komma null-ten"
        );
        assert_eq!(
            lb.ordinal_float_entry(&fv(3.25, 2)).unwrap(),
            "dräi Komma zwou fënnef-ten"
        );
        assert_eq!(lb.ordinal_float_entry(&dv("0")).unwrap(), "null-ten");
        assert_eq!(
            lb.ordinal_float_entry(&dv("5.00")).unwrap(),
            "fënnef Komma null null-ten"
        );
        assert!(matches!(
            lb.ordinal_float_entry(&fv(1e16, 0)),
            Err(N2WError::Value(_))
        ));
    }

    /// `to_ordinal_num(float)` = `str(number) + "."` — nothing parses, so the
    /// scientific forms succeed here.
    #[test]
    fn corpus_ordinal_num_entry() {
        let lb = LangLb::new();
        assert_eq!(
            lb.ordinal_num_float_entry(&fv(5.0, 1), "5.0").unwrap(),
            "5.0."
        );
        assert_eq!(
            lb.ordinal_num_float_entry(&fv(-0.0, 1), "-0.0").unwrap(),
            "-0.0."
        );
        assert_eq!(
            lb.ordinal_num_float_entry(&fv(1e16, 0), "1e+16").unwrap(),
            "1e+16."
        );
        assert_eq!(
            lb.ordinal_num_float_entry(&dv("1E+2"), "1E+2").unwrap(),
            "1E+2."
        );
        assert_eq!(
            lb.ordinal_num_float_entry(&dv("5.00"), "5.00").unwrap(),
            "5.00."
        );
    }

    /// `to_year(float)` — the trait default routes through the overridden
    /// `cardinal_float_entry`, giving LB's plain-cardinal years.
    #[test]
    fn corpus_year_entry() {
        let lb = LangLb::new();
        assert_eq!(
            lb.year_float_entry(&fv(5.0, 1)).unwrap(),
            "fënnef Komma null"
        );
        assert_eq!(
            lb.year_float_entry(&fv(-0.0, 1)).unwrap(),
            "minus null Komma null"
        );
        assert!(matches!(
            lb.year_float_entry(&fv(1e20, 0)),
            Err(N2WError::Value(_))
        ));
    }

    /// "Infinity"/"NaN" strings parse to `ParsedNumber::Inf`/`NaN` and are
    /// served natively: the int modes raise ValueError, `ordinal_num` echoes
    /// the repr with a trailing ".".
    #[test]
    fn str_to_number_inf_nan_native() {
        let lb = LangLb::new();
        assert!(matches!(
            lb.str_to_number("Infinity"),
            Ok(ParsedNumber::Inf { negative: false })
        ));
        assert!(matches!(
            lb.str_to_number("-Infinity"),
            Ok(ParsedNumber::Inf { negative: true })
        ));
        assert!(matches!(lb.str_to_number("NaN"), Ok(ParsedNumber::NaN)));

        // The int modes raise ValueError (int("Infinity") / int("NaN")).
        for to in ["cardinal", "ordinal", "year"] {
            assert!(matches!(lb.inf_result(false, to), Err(N2WError::Value(_))));
            assert!(matches!(lb.inf_result(true, to), Err(N2WError::Value(_))));
            assert!(matches!(lb.nan_result(to), Err(N2WError::Value(_))));
        }
        // ordinal_num is pure string formatting — no int(), so it succeeds.
        assert_eq!(lb.inf_result(false, "ordinal_num").unwrap(), "Infinity.");
        assert_eq!(lb.inf_result(true, "ordinal_num").unwrap(), "-Infinity.");
        assert_eq!(lb.nan_result("ordinal_num").unwrap(), "NaN.");

        // Everything else keeps Base's Decimal(value) semantics.
        assert!(matches!(lb.str_to_number("5"), Ok(ParsedNumber::Dec(_))));
        assert!(matches!(lb.str_to_number("1e3"), Ok(ParsedNumber::Dec(_))));
    }
}
