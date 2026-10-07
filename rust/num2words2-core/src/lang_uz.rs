//! Port of `lang_UZ.py` (Uzbek).
//!
//! Registry check: `__init__.py` maps `"uz"` to `lang_UZ.Num2Word_UZ()`, which
//! is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_UZ` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. `Num2Word_Base.
//! __init__` guards its card-table construction behind
//! `if any(hasattr(self, field) for field in [...])`, so for UZ `self.cards`
//! is **never built** and `self.MAXVAL` is **never set**. `to_cardinal` is
//! overridden outright and drives a recursive `_int_to_word`. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check** and no `splitnum`/`clean`/`merge` involvement at all.
//!
//! `setup()` only assigns `negword`/`pointword`/`ones`/`tens`/`hundred`/
//! `thousand`/`million`. Nothing else from the base is reconfigured, so
//! `is_title` stays `False` and `title()` is a no-op — note that the
//! overridden `to_cardinal` does not call `self.title()` anyway (the base's
//! does), so titling could never apply here even if it were enabled.
//!
//! Inherited from `Num2Word_Base` and left alone by UZ:
//!   * nothing relevant — UZ overrides `to_cardinal`, `to_ordinal`,
//!     `to_ordinal_num` and `to_year` itself.
//!
//! `to_year(self, val, longval=True)` ignores `longval` entirely and just
//! returns `self.to_cardinal(val)`, which is exactly the trait's default
//! `to_year`, so the integer hook is not re-implemented below. (The *float*
//! hook is: [`Lang::year_float_entry`]'s default would route whole floats to
//! the integer path, but UZ's `to_cardinal` keeps them in the `pointword`
//! branch — see the float section below.)
//!
//! # Float / Decimal cardinal path
//!
//! `Num2Word_UZ.to_cardinal` starts from `n = str(number).strip()` and branches
//! on `"." in n`. For integer input `str()` never yields a ".", so `to_cardinal`
//! below handles only the integer branch; the float/`pointword` branch is ported
//! into the [`LangUz::to_cardinal_float`] override, because the Rust core routes
//! integer cardinal (`&BigInt`) and float cardinal (`&FloatValue`) through
//! separate trait hooks. See that method for the sign, integer-part and
//! digit-by-digit fraction details.
//!
//! The `"." in str(number)` routing itself lives in the
//! [`Lang::cardinal_float_entry`] override:
//!
//! * a repr **with** a dot (every finite float with `|v| == 0` or
//!   `1e-4 <= |v| < 1e16`; any Decimal with a positive scale) takes the
//!   `pointword` branch — so whole values keep their tail: `5.0` ->
//!   "besh point zero", `Decimal("5.00")` -> "besh point zero zero";
//! * a repr **without** a dot and all digits (`Decimal("5")`, `Decimal("100")`)
//!   is `int(n)` -> the integer path;
//! * a repr without a dot that `int()` cannot parse — scientific notation
//!   (`str(1e16)` == "1e+16", `str(Decimal("1E+2"))` == "1E+2") or a
//!   non-finite value ("inf"/"nan") — raises **ValueError**, exactly like
//!   Python's `int("1e+16")`. Corpus-pinned for 1e+16, 1e+20, `Decimal("1E+2")`
//!   and `Decimal("1E+20")` across cardinal/ordinal/year.
//!
//! `to_ordinal` (cardinal + "-chi", no verify_ordinal) and `to_year`
//! (`to_cardinal`, `longval` ignored) inherit all of the above through their
//! own float entries; `to_ordinal_num` is `str(number) + "."` with no checks,
//! so floats keep their repr ("5.0.", "-0.0.", "1e+16.").
//!
//! Strings: `str_to_number` is Base's `Decimal(value)`, but a parse that
//! yields an *infinity* is intercepted and turned into the `ValueError` that
//! Python's `int("Infinity")` raises inside `to_cardinal` — the binding would
//! otherwise raise Base's OverflowError (`int(Decimal('Infinity'))`), which is
//! wrong for this self-contained class. See [`Lang::str_to_number`] below for
//! the off-corpus caveat (`to_ordinal_num("Infinity")`).
//!
//! # Currency
//!
//! `Num2Word_UZ` defines its own `CURRENCY_FORMS` (three codes: UZS, USD, EUR)
//! as a **class attribute on itself**, so it does not share — and is not
//! polluted by — the `Num2Word_EUR` dict that `Num2Word_EN.__init__` mutates in
//! place. Verified against the live interpreter: `CONVERTER_CLASSES["uz"].
//! CURRENCY_FORMS` has exactly those three keys, and `CURRENCY_ADJECTIVES` /
//! `CURRENCY_PRECISION` are both `{}` (inherited from `Num2Word_Base`, never
//! written). So `currency_adjective` stays `None` and `currency_precision`
//! stays 100 for *every* code — including JPY and KWD, which UZ has no entry
//! for and no precision override for. Both trait defaults already do that, so
//! neither is overridden below.
//!
//! `to_currency` is overridden outright by UZ and shares nothing with
//! `Num2Word_Base.to_currency`: no `parse_currency_parts`, no `divisor`, no
//! `pluralize`, no `CURRENCY_ADJECTIVES`. `to_cheque` is **not** overridden, so
//! it runs `Num2Word_Base.to_cheque` — which is what makes the two paths
//! disagree about unknown codes (quirks 7 and 8 below).
//!
//! `pluralize` is never reached: UZ's `to_currency` picks its form by hand
//! (`cr1[1] if left != 1 else cr1[0]`) and `Num2Word_Base.to_cheque` takes
//! `cr1[-1]` unconditionally. It therefore stays at the trait default, which
//! raises — exactly as `Num2Word_Base.pluralize` does.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the bare digits.** The
//!    final `else` is `return str(number)  # Fallback for very large numbers`.
//!    So `to_cardinal(10**9)` == "1000000000" (a numeral, not words) and
//!    `to_ordinal(10**9)` == "1000000000-chi". Verified against corpus rows
//!    for 10^9, 1234567890, 10^10, 10^12, 10^15, 10^18 and 10^21 — the value
//!    is unbounded, hence `BigInt` and `to_string()` rather than any cast.
//!    Negative inputs below -10^9 compose as "minus 1000000000": `to_cardinal`
//!    strips the sign, `_int_to_word` stringifies the magnitude, and the
//!    `negword` prefix is re-attached. (No corpus row covers that; see the
//!    report's `concerns`.)
//! 2. **Zero is the English "zero", not an Uzbek word.** `_int_to_word` does
//!    `return self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is the
//!    empty string (falsy), so the fallback always wins: `to_cardinal(0)` ==
//!    "zero" and `to_ordinal(0)` == "zero-chi". The `self.ones[0]` arm is
//!    dead code. Uzbek for zero is "nol".
//! 3. **The hundreds digit is always spelled out**, so 100 == "bir yuz"
//!    rather than a bare "yuz" ("hundreds_val" is 1..=9 and never suppressed).
//! 4. **`pointword` is the untranslated English "point"**, not an Uzbek word,
//!    and is used raw (UZ never titles). Reached only by the float path
//!    ([`LangUz::to_cardinal_float`]).
//! 5. **The ordinal suffix is applied to the whole cardinal with a hyphen and
//!    no agreement logic**: `to_ordinal(n) == to_cardinal(n) + "-chi"`. That
//!    means the sign leaks in too — `to_ordinal(-1)` == "minus bir-chi" — and
//!    the suffix lands on the digit fallback for large values (quirk 1).
//! 6. **`to_ordinal_num` is `str(number) + "."`**, a period rather than the
//!    "-chi"/"-inchi" abbreviation an Uzbek reader would expect, and it does
//!    not reject negatives: `to_ordinal_num(-1)` == "-1.". Note `Num2Word_Base.
//!    verify_ordinal` is never called by UZ, so no negative/float guard runs on
//!    any of the ordinal paths.
//! 7. **`to_currency` never raises for an unknown code.** It does
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a *default*, not a subscript — so GBP, JPY, KWD, BHD,
//!    INR, CNY and CHF all silently render as Uzbek som. The corpus pins this:
//!    `currency:JPY 12.34` is "o'n ikki so'm o'ttiz to'rt tiyin", not a
//!    `NotImplementedError` and not yen.
//! 8. **`to_cheque` *does* raise for those same codes**, because UZ does not
//!    override it and `Num2Word_Base.to_cheque` subscripts
//!    `self.CURRENCY_FORMS[currency]` inside a `try/except KeyError`. So
//!    `cheque:GBP` raises `NotImplementedError` while `currency:GBP` quietly
//!    prints som — the two entry points disagree about the same code. Both are
//!    in the corpus; both are reproduced.
//! 9. **`CURRENCY_PRECISION` is ignored by `to_currency` entirely.** UZ slices
//!    two decimal digits out of the string no matter the currency, so the
//!    3-decimal (KWD/BHD) and 0-decimal (JPY) conventions never apply:
//!    `currency:KWD 12.34` is 34 tiyin (not 340 fils) and `currency:JPY 12.34`
//!    shows a subunit at all (base would have rounded to a whole yen). The
//!    inherited `to_cheque` *does* read `CURRENCY_PRECISION`, but UZ's is `{}`,
//!    so it resolves to 100 for every code it accepts.
//! 10. **A float with zero cents drops the cents segment**, because `right` is
//!    an `int` and `if cents and right` makes 0 falsy. So `1.0` renders "bir
//!    euro" — identical to the int `1` — even though `base.to_currency` goes to
//!    great lengths to keep floats showing "... 00 cents". UZ's string-slicing
//!    rewrite loses the int/float distinction that `isinstance(val, int)`
//!    exists to preserve.
//! 11. **`cents=False` drops the segment outright** rather than falling back to
//!    the terse `"%02d"` digits: `_cents_terse` is never called, so
//!    `to_currency(12.34, cents=False)` is "o'n ikki euros", losing the .34.
//! 12. **A float extreme enough for `repr` to go scientific raises
//!    `ValueError`.** `str(1e16)` is "1e+16", which has no "." to split on, so
//!    `int(parts[0])` gets fed "1e+16" and dies. Verified: `to_currency(1e16)`
//!    raises `ValueError: invalid literal for int() with base 10: '1e+16'`.
//!    Reproduced here because `BigDecimal`'s `Display` happens to keep the
//!    scientific form for large magnitudes, so `BigInt::from_str` fails the
//!    same way. **Known gap at the other end of the range** — see the
//!    `str(val)` note below.
//! 13. **`adjective=True` is accepted and ignored** — UZ has no
//!    `CURRENCY_ADJECTIVES` and never calls `prefix_currency`.
//!
//! # Errors
//!
//! None of the four integer modes can raise for integer input: there is no
//! `MAXVAL` (so no `OverflowError`), no dict lookup (so no `KeyError`), and
//! every list index is bounded by the branch that guards it (so no
//! `IndexError`). `Result` is returned only to satisfy the trait.
//!
//! The currency surface has exactly two failure modes: `NotImplementedError`
//! from the inherited `to_cheque` on an unknown code (quirk 8), and
//! `ValueError` from `int()` on a scientific-notation float (quirk 12).
//! `to_currency` itself cannot raise `NotImplementedError` at all (quirk 7).
//!
//! # Known gap: `to_currency` depends on `str(val)`, which the boundary drops
//!
//! UZ is the awkward case for the `CurrencyValue` contract. `base.to_currency`
//! only ever does *arithmetic* on its Decimal, so passing `str(value)` across
//! the boundary and re-parsing it into a `BigDecimal` is lossless for it. UZ
//! instead re-stringifies the value (`parts = str(val).split(".")`), which
//! makes its output depend on the value's exact **textual** form — and
//! `BigDecimal::to_string()` is not Python's `str()`.
//!
//! They agree on every corpus row and on every ordinary magnitude, and they
//! coincidentally agree on large floats (both fail, quirk 12). They disagree
//! for a float whose `repr` goes scientific *below* 1e-4:
//!
//! | input | Python | here |
//! |---|---|---|
//! | `1e-05` (float) | `ValueError` — `str` is "1e-05" | "zero euros" — Display is "0.00001" |
//! | `Decimal("0.00001")` | "zero euros" — `str` is "0.00001" | "zero euros" |
//!
//! Note the two Python rows are the *same number* and differ only in type, so
//! this is not something the language file can repair: `CurrencyValue::Decimal`
//! carries neither the original string nor a float-vs-`Decimal` tag, and
//! reconstructing `repr(float)` here would be both the second
//! shortest-round-trip implementation `currency.rs` exists to avoid and wrong
//! for the `Decimal` row. No corpus row reaches it (the smallest float is
//! 0.01). Flagged rather than hacked around.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dollar / yevro with sent. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`. The trailing space is load-bearing: `to_cardinal` does
/// `ret = self.negword` and then `ret + self._int_to_word(...)` with no
/// separator of its own, so the space here is what separates "minus" from the
/// number. (The base's `to_cardinal` would `.strip()` it and re-add a space;
/// UZ's override does not, and the final `.strip()` only trims the ends.)
const NEGWORD: &str = "minus ";

/// `self.pointword`. Float path only — unreachable for integer input.
const POINTWORD: &str = "point";

/// `self.ones`. Index 0 is `""`; see quirk 2 — it is only ever read by the
/// dead `if self.ones[0]` test, never as a word.
const ONES: [&str; 10] = [
    "", "bir", "ikki", "uch", "to'rt", "besh", "olti", "yetti", "sakkiz", "to'qqiz",
];

/// `self.tens`. Index 0 is `""` and is unreachable: the tens branch is guarded
/// by `10 <= number < 100`, so `number // 10` is 1..=9.
const TENS: [&str; 10] = [
    "", "o'n", "yigirma", "o'ttiz", "qirq", "ellik", "oltmish", "yetmish", "sakson", "to'qson",
];

const HUNDRED: &str = "yuz";
const THOUSAND: &str = "ming";
const MILLION: &str = "million";

/// The "zero" literal from `_int_to_word`'s falsy-`ones[0]` fallback.
const ZERO_WORD: &str = "zero";

/// The `-chi` suffix `to_ordinal` glues onto the cardinal.
const ORDINAL_SUFFIX: &str = "-chi";

/// `Num2Word_UZ.to_currency`'s own default `separator=" "`. Confirmed against
/// the interpreter: `Num2Word_UZ.to_currency.__defaults__` is
/// `("UZS", True, " ", False)`.
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default `separator=","`, which the pyo3
/// binding substitutes when the Python caller omits the kwarg.
///
/// `__init__.py`'s Rust fast path sends `kwargs.get("separator", ",")` — the
/// **base** default, not UZ's — so by the time the trait sees a separator, a
/// caller who said nothing is indistinguishable from one who explicitly asked
/// for `","`. [`LangUz::to_currency`] reads this value back as the "unset"
/// sentinel it is; see the note there.
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// `self.__class__.__name__`, for the `to_cheque` NotImplementedError message.
const LANG_NAME: &str = "Num2Word_UZ";

/// Narrow a `BigInt` that the caller has already bounded to 0..=9 into a
/// list index.
///
/// Every call site is inside a branch that has proven `number < 10`, or takes
/// `number / 10` (resp. `/ 100`) under `number < 100` (resp. `< 1000`), so the
/// value always fits. The `expect` documents that invariant rather than
/// guarding a reachable path.
fn digit(n: &BigInt) -> usize {
    n.to_usize()
        .expect("callers bound this to 0..=9 before indexing ONES/TENS")
}

/// `Num2Word_UZ._int_to_word`.
///
/// Recursion is shallow and bounded: the thousands branch recurses on
/// `number // 1000 < 1000` and the millions branch on `number // 10**6 < 1000`,
/// so neither can re-enter a branch at or above its own magnitude, and the
/// 10^9 fallback is only ever reached at the top level.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        // Python: `self.ones[0] if self.ones[0] else "zero"`. ones[0] == ""
        // is falsy, so this is unconditionally "zero" (quirk 2).
        return ZERO_WORD.to_string();
    }

    if number.is_negative() {
        // Faithful, but dead from `to_cardinal`, which strips the sign before
        // calling in. Reachable in Python only via a direct `_int_to_word`
        // call (or `to_currency`, out of scope).
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    let ten = BigInt::from(10u32);
    let hundred = BigInt::from(100u32);
    let thousand = BigInt::from(1_000u32);
    let million = BigInt::from(1_000_000u32);
    let billion = BigInt::from(1_000_000_000u32);

    if number < &ten {
        return ONES[digit(number)].to_string();
    }

    if number < &hundred {
        // Python `//` and `%`; operands are non-negative here, so floor and
        // truncating division agree. div_mod_floor matches Python regardless.
        let (tens_val, ones_val) = number.div_mod_floor(&ten);
        if ones_val.is_zero() {
            return TENS[digit(&tens_val)].to_string();
        }
        return format!("{} {}", TENS[digit(&tens_val)], ONES[digit(&ones_val)]);
    }

    if number < &thousand {
        let (hundreds_val, remainder) = number.div_mod_floor(&hundred);
        // hundreds_val is 1..=9, so ONES[..] is never the empty string:
        // 100 -> "bir yuz", never a bare "yuz" (quirk 3).
        let mut result = format!("{} {}", ONES[digit(&hundreds_val)], HUNDRED);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if number < &million {
        let (thousands_val, remainder) = number.div_mod_floor(&thousand);
        let mut result = format!("{} {}", int_to_word(&thousands_val), THOUSAND);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if number < &billion {
        let (millions_val, remainder) = number.div_mod_floor(&million);
        let mut result = format!("{} {}", int_to_word(&millions_val), MILLION);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    // `return str(number)  # Fallback for very large numbers` (quirk 1).
    number.to_string()
}

/// Python `str(float)`/`repr(float)` — needed only for the reprs that carry
/// **no** decimal point (scientific notation and non-finite values), whose
/// text lands verbatim in the `int()` ValueError message
/// (`invalid literal for int() with base 10: '1e+16'`). Dotted fixed-form
/// reprs never come through here: [`Lang::cardinal_float_entry`] routes them
/// into the float grammar without stringifying.
fn py_float_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        };
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Python exponent form: mantissa, 'e', sign, two-digit-minimum exponent.
        let s = format!("{:e}", f);
        if let Some((m, e)) = s.split_once('e') {
            let (sign, digits) = match e.strip_prefix('-') {
                Some(d) => ("-", d.to_string()),
                None => ("+", e.to_string()),
            };
            let digits = if digits.len() < 2 {
                format!("0{}", digits)
            } else {
                digits
            };
            return format!("{}e{}{}", m, sign, digits);
        }
        s
    } else if f.fract() == 0.0 {
        // repr keeps the trailing ".0" that Rust's `{}` would drop.
        format!("{:.1}", f)
    } else {
        format!("{}", f)
    }
}

/// The ValueError Python's `int(n)` raises on a non-numeric string — the
/// failure mode of `to_cardinal`'s dot-less branch for scientific reprs.
/// Python strips a leading "-" from `n` before calling `int()`, so the
/// quoted literal never carries the sign.
fn int_value_error(repr_no_dot: &str) -> N2WError {
    let unsigned = repr_no_dot.strip_prefix('-').unwrap_or(repr_no_dot);
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        unsigned
    ))
}

pub struct LangUz {
    /// `Num2Word_UZ.CURRENCY_FORMS`. Built once in [`LangUz::new`] and only
    /// read afterwards: the generated registry parks each language in a
    /// `OnceLock` and calls `new` through `get_or_init`, so this is
    /// constructed once per process, not once per conversion.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the default `to_currency`'s
    /// `.get` falls back to for an unknown code (quirk 7).
    ///
    /// Python re-evaluates that expression on every call, and under CPython's
    /// insertion-ordered dicts it resolves to the **first key in the class
    /// body**, which for UZ is `UZS`. A `HashMap` has no first element, so the
    /// choice is pinned here rather than left to iteration order. Confirmed
    /// against the interpreter — `list(CURRENCY_FORMS.values())[0]` is
    /// `(("so'm", "so'm"), ("tiyin", "tiyin"))` — and against the corpus, where
    /// `currency:GBP 1` is "bir so'm". (Note `pprint` sorts dict keys and
    /// displays EUR first; that is a display artefact, not the real order.)
    fallback_forms: CurrencyForms,
}

impl LangUz {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::new();
        // Insertion order is irrelevant to a HashMap but the *first* Python key
        // is not — see `fallback_forms`. Listed in class-body order so the two
        // can be compared by eye. Arity is load-bearing: `to_currency` indexes
        // `cr1[0]`/`cr1[1]` and `cr2[0]`/`cr2[1]`, and the inherited
        // `to_cheque` takes `cr1[-1]`, so both forms of both tuples must
        // survive verbatim — including UZS's, where the two are identical
        // ("so'm" has no distinct plural).
        currency_forms.insert(
            "UZS",
            CurrencyForms::new(&["so'm", "so'm"], &["tiyin", "tiyin"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollar"], &["sent", "sent"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["yevro", "yevro"], &["sent", "sent"]),
        );
        let fallback_forms = currency_forms
            .get("UZS")
            .expect("CURRENCY_FORMS[\"UZS\"] is inserted directly above")
            .clone();
        LangUz {
            currency_forms,
            fallback_forms,
        }
    }
}

impl Default for LangUz {
    fn default() -> Self {
        LangUz::new()
    }
}

impl Lang for LangUz {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "UZS"
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

    /// `Num2Word_UZ.to_cardinal`.
    ///
    /// Python works on `str(number).strip()`, peels a leading "-" into `ret`,
    /// and — for integer input, where `"." in n` is always false — returns
    /// `(ret + self._int_to_word(int(n))).strip()`. `int(n)` is therefore the
    /// magnitude, and `_int_to_word`'s own negative branch is never taken from
    /// here.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // The trailing `.strip()`. `int_to_word` never returns an empty string
        // (0 yields "zero"), so this only ever no-ops, but it is what Python
        // does.
        Ok(format!("{}{}", ret, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// `Num2Word_UZ.to_ordinal`: the cardinal with "-chi" glued on (quirk 5).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// `Num2Word_UZ.to_ordinal_num`: `str(number) + "."` (quirk 6). No
    /// `verify_ordinal` call, so negatives pass straight through.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    // to_year is `self.to_cardinal(val)` (longval ignored) == the trait
    // default, which dispatches through `&self` and picks up the override
    // above. Not re-implemented.

    /// `Num2Word_UZ.to_cardinal`'s float/`pointword` branch, split out of the
    /// integer `to_cardinal` because the Rust core routes `&BigInt` and
    /// `&FloatValue` through separate trait hooks.
    ///
    /// Python:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword         # "minus "
    /// else:
    ///     ret = ""
    /// if "." in n:                              # this branch
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// ```
    ///
    /// Three faithful choices, all confirmed against the corpus and the live
    /// interpreter:
    ///
    /// * **The fraction is spelled digit by digit** (`for digit in right`), each
    ///   through `_int_to_word` — so `.34` is "uch to'rt" (three four), never
    ///   "o'ttiz to'rt" (thirty-four). This happens to coincide with
    ///   `Num2Word_Base.to_cardinal_float`'s per-digit spelling, but the sign
    ///   and integer-part handling below do not, so the default is still wrong.
    /// * **The integer part is `_int_to_word(int(left))`**, i.e. the *absolute*
    ///   value's digits — Python strips the "-" before splitting — so this uses
    ///   `pre.abs()`, not the signed `pre` that `Num2Word_Base` feeds to
    ///   `to_cardinal`. `_int_to_word` (not `to_cardinal`) also means the digit
    ///   fallback (quirk 1) applies: `str(98746251323029.99)` → integer part
    ///   "98746251323029".
    /// * **The sign follows `str(number).startswith("-")`**, i.e. the IEEE sign
    ///   bit, so `-0.0` (str "-0.0") is negative and renders "minus zero point
    ///   zero" even though `-0.0 < 0.0` is false. That is `is_sign_negative()`
    ///   on the raw f64 — and it is the one negword divergence from
    ///   `Num2Word_Base`, which keys on `value < 0` and so drops the sign for
    ///   `-0.0`. The negword is always prepended for a negative here (Python
    ///   sets `ret = self.negword` up front), not only when the integer part is
    ///   zero.
    ///
    /// `right` (Python's fractional digit string) is reconstructed from
    /// `float2tuple`'s `post`, zero-padded to `precision`. `precision` is by
    /// construction `len(right)` — `abs(Decimal(str(f)).as_tuple().exponent)`
    /// for floats, the Decimal scale otherwise — so the padded string equals
    /// `right`. Keeping `float2tuple`'s f64 arithmetic (rather than re-deriving
    /// from a decimal string) preserves the load-bearing `< 0.01` rescue
    /// heuristic: `2.675` → `674.999…` → `675` → "olti yetti besh", exactly as
    /// the corpus pins it.
    ///
    /// `precision=` (issue #580 → `precision_override`) is **inert** for UZ:
    /// `Num2Word_UZ.to_cardinal(self, number)` takes no `precision` argument and
    /// never reads `self.precision`. Confirmed live — `num2words(2.675, 'uz',
    /// precision=2)` and `precision=0` both give "ikki point olti yetti besh".
    /// The argument is accepted and ignored.
    ///
    /// # Scientific-notation reprs never reach this method
    ///
    /// `str(1e16)` is "1e+16" (no "." → `int("1e+16")` → ValueError) and
    /// `str(1e-05)` is "1e-05"; Python's `int()` dies either way. That
    /// routing now lives in [`Lang::cardinal_float_entry`], which reproduces
    /// Python's repr boundary (`abs(value) >= 1e16` or `0 < abs(value) <
    /// 1e-4` goes scientific) and raises the ValueError *before* this float
    /// grammar runs — so every value that arrives here has a dotted repr.
    ///
    /// # Known off-corpus gap (flagged, not repaired)
    ///
    /// * **Large-magnitude fractional digits.** For a float big enough that
    ///   `(value - trunc(value)) * 10**precision` loses precision (e.g.
    ///   `959926189324.9778`), `str()`'s shortest-round-trip digits ("…9778")
    ///   and `float2tuple`'s arithmetic ("…9777") differ in the last digit.
    ///   `float2tuple` matches `Num2Word_Base` here, not `str()`; no corpus
    ///   float exceeds 1234.56, so it is not exercised.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // precision is `len(right)` by construction; UZ ignores precision=.
        let precision = value.precision() as usize;

        // (pre, post): pre = int(number) truncated toward zero (signed);
        // post = the fractional digits as a non-negative integer.
        let (pre, post) = float2tuple(value);

        // `str(number).startswith("-")` == the IEEE sign bit, so -0.0 is
        // negative. The Decimal arm keeps BigDecimal's own sign.
        let is_negative = match value {
            FloatValue::Float { value, .. } => value.is_sign_negative(),
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };

        // Build the words and join with single spaces — Python concatenates
        // "minus " / word / " point " / "<digit> " fragments and `.strip()`s
        // the trailing space, which is exactly a single-space join.
        let mut words: Vec<String> = Vec::new();
        if is_negative {
            // `ret = self.negword` ("minus "); after the join it is just "minus".
            words.push(NEGWORD.trim().to_string());
        }
        // `_int_to_word(int(left))` on the absolute integer part.
        words.push(int_to_word(&pre.abs()));

        if precision > 0 {
            // `self.pointword`, used raw (UZ never titles; is_title is false).
            // POINTWORD == self.pointword() ("point") for UZ.
            words.push(POINTWORD.to_string());

            // right = post, zero-padded on the left to `precision` digits. post
            // < 10**precision, so this is exactly `precision` ASCII digits.
            let post_digits = post.to_string();
            let right = format!(
                "{}{}",
                "0".repeat(precision.saturating_sub(post_digits.len())),
                post_digits
            );
            // `for digit in right: _int_to_word(int(digit))`.
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("non-digit {:?} in fractional part", ch))
                })?;
                words.push(int_to_word(&BigInt::from(d)));
            }
        }

        Ok(words.join(" "))
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands a float/Decimal straight to the converter
    // method, and UZ's own `to_cardinal` decides where it lands by looking at
    // `str(number)` — NOT by base's `int(value) == value` assert. The trait
    // default (whole -> int path) is therefore wrong for every whole float:
    // `to_cardinal(5.0)` is "besh point zero", never "besh".

    /// `Num2Word_UZ.to_cardinal`'s `"." in str(number)` routing, whole values
    /// included:
    ///
    /// * dotted repr -> the `pointword` branch ([`Self::to_cardinal_float`]),
    ///   so 5.0 -> "besh point zero" and -0.0 -> "minus zero point zero";
    /// * dot-less digits (`Decimal("5")`, `Decimal("-3")`) -> `int(n)` ->
    ///   the integer path, identical to [`Self::to_cardinal`];
    /// * dot-less non-digits — scientific reprs ("1e+16", "1E+2") and
    ///   non-finite values ("inf", "nan") — -> `int(n)` raises **ValueError**.
    ///
    /// The float repr boundary is CPython's: fixed form iff the value is zero
    /// or `1e-4 <= |v| < 1e16` (both literals exact in f64; `1e-4` parses to
    /// the nearest double *above* decimal 1e-4, matching repr's decision bit
    /// for bit). Every finite fixed-form float shows a ".", so the boundary
    /// test doubles as the `"." in n` test without building the repr.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                let a = f.abs();
                if f.is_finite() && (*f == 0.0 || (a >= 1e-4 && a < 1e16)) {
                    self.to_cardinal_float(value, precision_override)
                } else {
                    Err(int_value_error(&py_float_str(*f)))
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                // `str(Decimal)` — python_decimal_str is byte-exact, including
                // the plain/E±n selection that decides this branch.
                let s = python_decimal_str(d);
                if s.contains('.') {
                    self.to_cardinal_float(value, precision_override)
                } else if s.contains('E') {
                    // "1E+2", "1E+20", "1E-7": no dot, int() dies.
                    Err(int_value_error(&s))
                } else {
                    // Plain integer string -> scale 0 -> exact whole value.
                    let i = value
                        .as_whole_int()
                        .expect("dot-less, E-less Decimal repr is integral");
                    self.to_cardinal(&i)
                }
            }
        }
    }

    /// `Num2Word_UZ.to_ordinal(float/Decimal)`: the cardinal with "-chi"
    /// glued on (quirk 5) — **no** `verify_ordinal`, so negatives and
    /// fractions pass straight through ("minus bir point besh-chi") and the
    /// scientific-repr ValueError propagates before the suffix is reached.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// `Num2Word_UZ.to_ordinal_num(float/Decimal)`: `str(number) + "."` with
    /// no checks (quirk 6), so the repr survives verbatim — "5.0.", "-0.0.",
    /// "1e+16.", "5.00.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `Num2Word_UZ.to_year(float/Decimal)`: `self.to_cardinal(val)` with
    /// `longval` ignored — i.e. exactly the cardinal entry above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    // ---- string inputs ---------------------------------------------------

    /// `converter.str_to_number` is Base's `Decimal(value)` — UZ does not
    /// override it — but an *infinity* parse must not surface as Base's
    /// OverflowError (`int(Decimal('Infinity'))`): UZ's `to_cardinal` never
    /// calls `int()` on the Decimal, it calls `int(str(number))`, and
    /// `int("Infinity")` raises **ValueError**. Corpus-pinned:
    /// `num2words("Infinity", lang="uz")` / `"-Infinity"` are ValueError.
    /// The message quotes the sign-stripped literal, as `to_cardinal` peels
    /// a leading "-" before `int()`.
    ///
    /// Off-corpus caveat, flagged not repaired: Python's
    /// `to_ordinal_num(Decimal('Infinity'))` would return "Infinity." (no
    /// int() involved), which this early raise cannot reproduce — the parse
    /// result routes through one shared entry. No corpus row reaches it
    /// (only `to="cardinal"` Infinity rows exist).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ------------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `Num2Word_UZ.CURRENCY_FORMS[code]`, as a *subscript* — a miss is `None`.
    ///
    /// This is only reached from the inherited `to_cheque`, which is what makes
    /// `cheque:GBP` raise `NotImplementedError` (quirk 8). `to_currency` below
    /// deliberately does **not** route through here: it uses `.get` with a
    /// fallback and so must still see a miss as som, not as an error (quirk 7).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // currency_adjective: CURRENCY_ADJECTIVES is {} -> always None == default.
    // currency_precision: CURRENCY_PRECISION is {} -> .get(code, 100) is always
    //   100 == default. Not overridden; see quirk 9.
    // pluralize: unreachable from either currency path, so the raising default
    //   stands, matching Num2Word_Base.pluralize.
    // money_verbose / cents_verbose / cents_terse: UZ inherits all three from
    //   Num2Word_Base. Only _money_verbose is ever called (by to_cheque), and
    //   the trait default already forwards to self.to_cardinal, which resolves
    //   to UZ's override above.
    // to_cheque: not overridden by UZ, so the trait default -- a port of
    //   Num2Word_Base.to_cheque -- is exactly right. It reads currency_forms
    //   (subscript semantics, above), currency_precision (100) and
    //   money_verbose, all of which now resolve correctly for UZ.

    /// Port of `Num2Word_UZ.to_currency`.
    ///
    /// UZ ignores `base.to_currency` wholesale and slices the decimal *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Because the branch is on the *text* rather than on `isinstance(val,
    /// int)`, the int/float distinction collapses on its own: `1` and `1.0`
    /// both reach `right == 0` and both print "bir euro" (quirk 10). The
    /// `CurrencyValue` split is still honoured — it decides whether `str(val)`
    /// has a "." at all — it simply stops mattering downstream.
    ///
    /// # The `separator` argument is not what Python's default would be
    ///
    /// UZ's own signature defaults to `separator=" "`, but the trait cannot
    /// carry a per-language default argument: `__init__.py` resolves it before
    /// the call and substitutes `Num2Word_Base`'s `","`. The frozen corpus was
    /// generated through Python with no `separator=` kwarg — so UZ's `" "`
    /// applied, and `currency:EUR 12.34` is "o'n ikki euros o'ttiz to'rt cents"
    /// with a space. `","` is therefore read back as the "unset" sentinel it
    /// is. Exact for a caller who omits `separator` and for one who passes
    /// anything other than `","`; wrong only for an explicit `separator=","`,
    /// which Python renders as "o'n ikki euros,o'ttiz to'rt cents" (no space —
    /// the separator replaces it) and this renders with a space. `lang_br.rs`
    /// and `lang_bs.rs` take the same approach for the same reason. See
    /// `concerns`.
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
        // Python accepts `adjective` and never reads it (quirk 13).
        let _ = adjective;

        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` runs *before*
        // `str(val)`, so the string never carries a sign. The abs() is
        // conditional in Python, so it stays conditional here.
        let is_negative = val.is_negative();
        let s = match val {
            CurrencyValue::Int(v) => {
                if is_negative {
                    v.abs().to_string()
                } else {
                    v.to_string()
                }
            }
            CurrencyValue::Decimal { value: d, .. } => {
                if is_negative {
                    d.abs().to_string()
                } else {
                    d.to_string()
                }
            }
        };

        // `str(val).split(".")`. Python splits on every dot and then reads only
        // parts[0] and parts[1], so any trailing fragment is ignored; taking
        // the first two off the iterator is the same thing.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`. Evaluated before `right`, so a
        // scientific-notation string raises here first (quirk 12). ValueError
        // is what Python's int() raises, hence N2WError::Value.
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads, so "5" -> "50" (0.5 is 50 cents,
        // not 5) and "01" -> "01" (0.01 is 1). Sliced by chars, not bytes.
        // A whole float gives "0" -> "00" -> 0, which is falsy (quirk 10).
        let right = match part1 {
            Some(f) if !f.is_empty() => {
                let mut two: String = f.chars().take(2).collect();
                while two.chars().count() < 2 {
                    two.push('0');
                }
                BigInt::from_str(&two).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — quirk 7.
        // Deliberately not `self.currency_forms(currency)`: that hook has
        // subscript semantics for to_cheque's sake.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::one();

        // `left_str = self._int_to_word(left)` — `_int_to_word`, not
        // `to_cardinal`, so none of to_cardinal's str()/strip() wrapper runs.
        // `left` is already non-negative, so int_to_word's negative branch is
        // unreachable from here.
        //
        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`. Note
        // 0 takes the *plural*: "zero euros".
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 10). `cents=False`
        // drops it too, with no terse fallback (quirk 11).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — raw, so NEGWORD's trailing space
        // is what separates it from the number.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input, but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    fn f(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn d(s: &str) -> FloatValue {
        // `precision` mirrors what the shim computes
        // (`abs(Decimal(str(value)).as_tuple().exponent)`); by construction it
        // is the digit count after the dot.
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        FloatValue::Decimal { value, precision }
    }

    fn go(v: &FloatValue) -> String {
        LangUz::new().to_cardinal_float(v, None).unwrap()
    }

    /// Every `"to": "cardinal"` row with a dot in `arg` that the frozen corpus
    /// records for uz, verbatim. `precision` is the fractional digit count of
    /// the arg text, which for these shortest-repr floats equals
    /// `abs(Decimal(str(f)).as_tuple().exponent)`.
    #[test]
    fn corpus_cardinal_float_rows() {
        for (value, precision, out) in [
            (0.0, 1, "zero point zero"),
            (0.5, 1, "zero point besh"),
            (1.0, 1, "bir point zero"),
            (1.5, 1, "bir point besh"),
            (2.25, 2, "ikki point ikki besh"),
            (3.14, 2, "uch point bir to'rt"),
            (0.01, 2, "zero point zero bir"),
            (0.1, 1, "zero point bir"),
            (0.99, 2, "zero point to'qqiz to'qqiz"),
            (1.01, 2, "bir point zero bir"),
            (12.34, 2, "o'n ikki point uch to'rt"),
            (99.99, 2, "to'qson to'qqiz point to'qqiz to'qqiz"),
            (100.5, 1, "bir yuz point besh"),
            (1234.56, 2, "bir ming ikki yuz o'ttiz to'rt point besh olti"),
            (-0.5, 1, "minus zero point besh"),
            (-1.5, 1, "minus bir point besh"),
            (-12.34, 2, "minus o'n ikki point uch to'rt"),
            // The f64-artefact rows: 1.005 -> 4.99999999999989 and
            // 2.675 -> 674.9999999999998, both rescued by float2tuple's
            // `< 0.01` heuristic, exactly as in Python.
            (1.005, 3, "bir point zero zero besh"),
            (2.675, 3, "ikki point olti yetti besh"),
        ] {
            assert_eq!(go(&f(value, precision)), out, "float {}", value);
        }
    }

    /// Every `"to": "cardinal_dec"` row the frozen corpus records for uz,
    /// verbatim — including the trillion-scale one that pins both the exact
    /// Decimal arm (no f64 rounding of .99) and `_int_to_word`'s bare-digit
    /// fallback at >= 10^9 (quirk 1).
    #[test]
    fn corpus_cardinal_dec_rows() {
        for (arg, out) in [
            ("0.01", "zero point zero bir"),
            ("1.10", "bir point bir zero"),
            ("12.345", "o'n ikki point uch to'rt besh"),
            ("98746251323029.99", "98746251323029 point to'qqiz to'qqiz"),
            ("0.001", "zero point zero zero bir"),
        ] {
            assert_eq!(go(&d(arg)), out, "decimal {}", arg);
        }
    }

    /// Off-corpus cases traced against the pure-Python converter
    /// (`CONVERTER_CLASSES['uz'].to_cardinal(...)`, bypassing the dispatcher's
    /// Rust fast path).
    #[test]
    fn traced_against_pure_python() {
        // str(-0.0) is "-0.0", so the sign survives: the IEEE sign bit, not
        // `< 0` (which -0.0 fails), decides the negword.
        assert_eq!(go(&f(-0.0, 1)), "minus zero point zero");
        // Integer part past 10^9: _int_to_word's digit fallback (quirk 1)
        // applies to the float path's integer part too.
        assert_eq!(go(&f(1234567890.5, 1)), "1234567890 point besh");
        // Large magnitude where float2tuple takes the floor branch
        // (67.1875 is not within 0.01 of an integer) and still agrees with
        // Python's str()-derived digits.
        assert_eq!(
            go(&f(123456789012345.67, 2)),
            "123456789012345 point olti yetti"
        );
    }

    /// `precision=` is inert for UZ: `Num2Word_UZ.to_cardinal` takes no such
    /// argument and never reads `self.precision`. Confirmed live —
    /// `num2words(2.675, 'uz', precision=0)` == `precision=2` == no kwarg.
    #[test]
    fn precision_override_is_ignored() {
        let uz = LangUz::new();
        let v = f(2.675, 3);
        let expect = "ikki point olti yetti besh";
        assert_eq!(uz.to_cardinal_float(&v, None).unwrap(), expect);
        assert_eq!(uz.to_cardinal_float(&v, Some(2)).unwrap(), expect);
        assert_eq!(uz.to_cardinal_float(&v, Some(0)).unwrap(), expect);
    }
}
