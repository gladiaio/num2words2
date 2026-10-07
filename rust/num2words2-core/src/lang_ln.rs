//! Port of `lang_LN.py` (Lingala).
//!
//! Shape: **self-contained**. `Num2Word_LN` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives a plain
//! recursive `_int_to_word`. Consequently `cards`/`maxval`/`merge` stay at
//! their trait defaults here, and there is **no overflow check** — see the
//! digit fallback below for what happens instead.
//!
//! Inherited from `Num2Word_Base` and left alone by LN:
//!   * nothing in the four in-scope modes — LN overrides `to_cardinal`,
//!     `to_ordinal` and `to_ordinal_num`, and its `to_year(val, longval=True)`
//!     ignores `longval` and just forwards to `to_cardinal`. The trait's
//!     default `to_year` already delegates through `&self`, so it picks up the
//!     `to_cardinal` override below and matches Python exactly; it is left
//!     unimplemented here rather than restated.
//!
//! LN carries **no cross-call mutable state** — no `_pending_ordinal`-style
//! handshake. `str_to_number` keeps Base's `Decimal(value)` semantics except
//! that an Infinity parse punts to the Python fallback (see the hook), so the
//! dispatcher's Rust fast path for plain ints is safe.
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, and are confirmed by the frozen corpus:
//!
//! 1. **The digit fallback.** `_int_to_word` handles the number only up to
//!    `< 1000000000`; its final `else` is `return str(number)`. So Lingala
//!    silently stops wording at 10^9 and hands back bare digits:
//!    `to_cardinal(10**9)` == "1000000000" (not an `OverflowError`), and
//!    `to_ordinal(10**9)` == "1000000000-e". This is why the value must stay a
//!    `BigInt` — the fallback is reached for arbitrarily large input
//!    (corpus covers 10^21) and `to_string()` must render it exactly.
//! 2. **No scale word above "milio".** There is no billion/trillion entry at
//!    all; 10^9 is simply where wording stops (see 1).
//! 3. **`to_ordinal` is cardinal + "-e", unconditionally.** No negative guard
//!    (Python's `errmsg_negord` is never consulted), so `to_ordinal(-1)` ==
//!    "minus moko-e", and the suffix lands on the *last word* with no space:
//!    `to_ordinal(11)` == "zómi moko-e". It also glues onto the digit
//!    fallback, hence "1000000000-e".
//! 4. **`to_ordinal_num` ignores the language entirely** and is `str(number)
//!    + "."`, so `to_ordinal_num(-1)` == "-1." and `to_ordinal_num(0)` == "0.".
//! 5. **`_int_to_word(0)` is `self.ones[0] if self.ones[0] else "zero"`.**
//!    `ones[0]` is `""` (falsy), so the branch is dead and the answer is
//!    always "zero" — never the empty string. Mirrored as a constant below.
//!
//! # Unreachable-but-preserved code
//!
//! `_int_to_word`'s `if number < 0` arm cannot fire from any of the four
//! in-scope modes: `to_cardinal` strips the "-" off the *string* before
//! calling `int()`, so `_int_to_word` only ever sees a non-negative value.
//! It is reproduced anyway ([`int_to_word`]) so the function matches its
//! Python counterpart line for line.
//!
//! # The currency surface
//!
//! `Num2Word_LN` declares its own `CURRENCY_FORMS` (CDF/USD/EUR) as a class
//! attribute and inherits `CURRENCY_ADJECTIVES = {}` / `CURRENCY_PRECISION =
//! {}` from `Num2Word_Base`. Verified against the live interpreter: the MRO is
//! exactly `Num2Word_LN -> Num2Word_Base -> object`, so LN never touches the
//! `Num2Word_EUR` dict that `Num2Word_EN.__init__` mutates in place, and none
//! of English's ~24 extra codes leak in. `CURRENCY_PRECISION` being empty means
//! **every** code runs at the default divisor 100 — LN has no 3-decimal
//! (KWD/BHD) or 0-decimal (JPY) path at all, and the corpus confirms it:
//! `currency:JPY` of `12.34` is "... ntuku mísáto mínei santimi", i.e. 34
//! subunits, not a rounded whole.
//!
//! `to_currency` is overridden outright and ignores `base.to_currency`
//! entirely — no `parse_currency_parts`, no `divisor`, no `pluralize`, no
//! `prefix_currency`. `to_cheque` is **not** overridden, so it comes from
//! `Num2Word_Base` (the trait default reproduces it) and reaches back into
//! LN's `to_cardinal` through the default `_money_verbose`.
//!
//! ## Further faithfully reproduced Python behaviour
//!
//! 6. **Unknown currency codes never raise from `to_currency`.** The lookup is
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a miss silently falls back to the *first inserted* entry, `CDF`. So
//!    `currency:GBP` prints "faranga"/"santimi" rather than raising. The
//!    inherited `to_cheque` subscripts `CURRENCY_FORMS[currency]` and converts
//!    the `KeyError` into `NotImplementedError`, so `cheque:GBP` *does* raise.
//!    Both halves are in the corpus and the split is deliberate — see the note
//!    on [`LangLn::currency_forms`].
//! 7. **Cents are sliced out of the decimal string, not computed.**
//!    `int(parts[1][:2].ljust(2, "0"))` truncates past two digits and pads
//!    short ones, so `0.5` is 50 subunits and `0.01` is 1. There is no
//!    rounding: `1.567` yields 56 subunits, not 57.
//! 8. **A float with zero cents drops the cents segment.** `if cents and
//!    right:` tests an `int`, and `1.0` gives `right == 0`, which is falsy. So
//!    `currency:EUR` of `1.0` is "moko euro" — the same output as the `int` 1,
//!    but arrived at through the string `"1.0"` rather than through Python's
//!    `isinstance(val, int)` branch, which LN never consults.
//! 9. **`cents=False` drops the segment outright.** `Num2Word_Base` would fall
//!    back to `_cents_terse` and print "34"; LN has no `else`, so the cents
//!    simply vanish.
//! 10. **Zero takes the plural.** `cr1[1] if left != 1 else cr1[0]` keys off
//!     `!= 1`, so `0` renders "zero euros".
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). No reliable Lingala cent noun was found, so USD and EUR raise
//! NotImplementedError. Examples in these docs that quote English nouns record
//! Python's output.

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

/// `self.negword` — note the trailing space; `to_cardinal` concatenates it
/// directly onto the worded magnitude and `.strip()`s the result.
const NEGWORD: &str = "minus ";

/// `self.ones`. Index 0 is `""` in Python and is only ever reached through
/// the dead `_int_to_word(0)` branch documented above.
const ONES: [&str; 10] = [
    "", "moko", "míbalé", "mísáto", "mínei", "mítáno", "motóbá", "sambo", "mwambe", "libwá",
];

/// `self.tens`. Index 0 is `""` and is unreachable (the `< 100` arm is only
/// entered for `number >= 10`, so `tens_val >= 1`).
const TENS: [&str; 10] = [
    "",
    "zómi",
    "ntuku míbalé",
    "ntuku mísáto",
    "ntuku mínei",
    "ntuku mítáno",
    "ntuku motóbá",
    "ntuku sambo",
    "ntuku mwambe",
    "ntuku libwá",
];

const HUNDRED: &str = "nkama";
const THOUSAND: &str = "nkóto";
const MILLION: &str = "milio";

/// `self.pointword`, interpolated raw between the integer part and the digits
/// on the float path: `... + " " + self.pointword + " "`. LN never calls
/// `self.title()`, so it is emitted verbatim (mirrors [`LangLn::pointword`]).
const POINTWORD: &str = "point";

/// `Num2Word_LN.to_currency`'s own default `separator=" "`, confirmed against
/// the interpreter: `Num2Word_LN.to_currency.__defaults__` is
/// `("CDF", True, " ", False)`.
///
/// Unlike `Num2Word_Base`, whose format string writes `separator` *followed by
/// a literal space*, LN writes `result += separator + cents_str + ...`. The
/// separator is therefore the entire gap, and LN's `" "` is what keeps the
/// default output comma-free ("moko euro moko cent").
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default `separator=","`.
///
/// Both callers of the Rust core — `__init__.py`'s fast path and
/// `bench/diff_test.py` — send `kwargs.get("separator", ",")`, i.e. *base's*
/// default rather than the per-language one. The trait signature has no way to
/// express a default argument, so "caller said nothing" and "caller explicitly
/// asked for `,`" arrive identically. `to_currency` reads this value as the
/// former and substitutes [`DEFAULT_SEPARATOR`]; see the note on
/// [`LangLn::to_currency`]. `lang_br.rs` and `lang_es.rs` resolve the same
/// conflict the same way.
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// The value `_int_to_word(0)` returns. Python writes
/// `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""`, so the
/// conditional always takes the `else`.
const ZERO_WORD: &str = "zero";

/// Narrow a `BigInt` to a table index.
///
/// Every call site has already bounded the value by an explicit `<`
/// comparison (`< 10`, or a quotient of a value `< 1000`), so the value is in
/// `0..=9` and the conversion cannot fail. `unwrap_or(0)` keeps the function
/// total without masking a real bug: a hypothetical out-of-range value would
/// hit `ONES[0]`/`TENS[0]` == `""`, the same thing Python's list would return
/// for index 0.
fn idx(n: &BigInt) -> usize {
    n.to_usize().unwrap_or(0)
}

/// Python's `Num2Word_LN._int_to_word`.
///
/// Recursive, and deliberately stops at 10^9 — past that it returns the
/// decimal digits (bug 1 in the module docs).
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Unreachable from the four in-scope modes (see module docs); preserved
    // to mirror the Python function exactly.
    if n.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&n.abs()));
    }

    let ten = BigInt::from(10u32);
    let hundred = BigInt::from(100u32);
    let thousand = BigInt::from(1000u32);
    let million = BigInt::from(1_000_000u32);
    let billion = BigInt::from(1_000_000_000u32);

    if n < &ten {
        return ONES[idx(n)].to_string();
    }

    if n < &hundred {
        // Python: number // 10, number % 10. Operands are positive here, so
        // floor- and trunc-division agree.
        let (tens_val, ones_val) = n.div_rem(&ten);
        if ones_val.is_zero() {
            return TENS[idx(&tens_val)].to_string();
        }
        return format!("{} {}", TENS[idx(&tens_val)], ONES[idx(&ones_val)]);
    }

    if n < &thousand {
        // Note: hundreds uses `self.ones[hundreds_val]` directly — not a
        // recursive call — so 100 is "moko nkama".
        let (hundreds_val, remainder) = n.div_rem(&hundred);
        let mut result = format!("{} {}", ONES[idx(&hundreds_val)], HUNDRED);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if n < &million {
        let (thousands_val, remainder) = n.div_rem(&thousand);
        let mut result = format!("{} {}", int_to_word(&thousands_val), THOUSAND);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if n < &billion {
        let (millions_val, remainder) = n.div_rem(&million);
        let mut result = format!("{} {}", int_to_word(&millions_val), MILLION);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    // `return str(number)` — the fallback for "very large numbers".
    n.to_string()
}

/// Python's `int(s)` on the integer fragment `parts[0]`/`n` of `str(number)`.
///
/// `int()` accepts only a plain optionally-signed digit run, so a fragment in
/// exponent notation (`"1e+16"`) raises `ValueError`. The reconstructed strings
/// never carry an exponent (see [`float_to_str`]), so this is unreachable for
/// every corpus row — but it is reproduced verbatim, including Python's message,
/// for the scientific-notation float that `str()` would produce (see the port
/// report).
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Python's `int(digit)` on a *single* character of the fractional string,
/// inside `Num2Word_LN.to_cardinal`'s float branch (`_int_to_word(int(digit))`).
///
/// `int('5')` -> 5; `int('e')` raises `ValueError` with the char quoted. Our
/// reconstructed fractional part is always ASCII `0-9`, so the error arm only
/// fires for the exponent-notation repr LN itself chokes on.
fn parse_digit(ch: char) -> Result<BigInt> {
    match ch.to_digit(10) {
        Some(d) => Ok(BigInt::from(d)),
        None => Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            ch
        ))),
    }
}

/// Reconstruct Python's `str(number)` from a `FloatValue`.
///
/// `Num2Word_LN.to_cardinal` does no float arithmetic — it `str()`s the input
/// and splits on `"."`. So the float path must reproduce `str` exactly, **not**
/// `base.float2tuple`: `str(float)` is `repr` (shortest round-trip, exponent
/// form at `|v| >= 1e16`) and `str(Decimal)` is `_pydecimal.__str__` (scale
/// preserved, `E+n` form for a positive exponent). The exponent forms are
/// load-bearing: `str(1e16) == "1e+16"` has no `"."`, so Python's `int()`
/// raises `ValueError` — the digit-perfect fixed-notation reconstruction the
/// previous revision used silently *succeeded* there. Helpers mirror
/// `lang_sk.rs`/`lang_cs.rs`.
fn float_to_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_repr_f64(*value),
        FloatValue::Decimal { value, .. } => python_str_decimal(value),
    }
}

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. Mirrors `lang_sk.rs`'s
/// `python_repr_f64`.
fn python_repr_f64(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative` captures -0.0, whose repr is '-0.0'.
    let neg = f.is_sign_negative();
    let s = format!("{:e}", f.abs()); // e.g. "1.234e1", "5e-1", "0e0"
    let (mantissa, exp_s) = match s.split_once('e') {
        Some(parts) => parts,
        None => return s, // unreachable for finite f64
    };
    let exp: i32 = exp_s.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let ndigits = digits.len() as i32;
    let decpt = exp + 1;

    let body = if decpt <= -4 || decpt > 16 {
        // Scientific: first digit, optional ".rest", then "e±NN".
        let mut m = String::new();
        m.push_str(&digits[..1]);
        if ndigits > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        let e = decpt - 1;
        let (esign, eabs) = if e < 0 { ('-', -e) } else { ('+', e) };
        format!("{}e{}{:02}", m, esign, eabs)
    } else if decpt <= 0 {
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndigits {
        format!("{}{}.0", digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        let dp = decpt as usize;
        format!("{}.{}", &digits[..dp], &digits[dp..])
    };

    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// CPython's `str(Decimal)`: trailing zeros preserved (`"1.10"` stays
/// `"1.10"`), `E±n` notation exactly when `exp > 0` or the adjusted exponent
/// `< -6` — so `Decimal("1E+3")` prints `"1E+3"` and feeds `int()` a
/// `ValueError`. Mirrors `lang_sk.rs`'s `python_str_decimal`.
fn python_str_decimal(bd: &BigDecimal) -> String {
    let (coeff, scale) = bd.as_bigint_and_exponent();
    let py_exp: i64 = -scale; // Decimal._exp
    let neg = coeff.is_negative(); // BigInt drops the sign of a negative zero
    let int_str = coeff.abs().to_string(); // Decimal._int
    let ndigits = int_str.len() as i64;
    let leftdigits = py_exp + ndigits;

    // dotplace, non-engineering branch of _pydecimal.__str__.
    let dotplace: i64 = if py_exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_str),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", int_str, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let dp = dotplace as usize;
        (int_str[..dp].to_string(), format!(".{}", &int_str[dp..]))
    };

    let exp = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — a sign but no zero-padding, unlike float repr.
        format!("E{:+}", leftdigits - dotplace)
    };

    let sign = if neg { "-" } else { "" };
    format!("{}{}{}{}", sign, intpart, fracpart, exp)
}

/// Port of `Num2Word_LN.to_cardinal` operating on the reconstructed
/// `str(number)`:
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
/// Note the fraction uses `_int_to_word(int(digit))` — the full recursive
/// helper, so a `0` digit becomes `"zero"` (via [`int_to_word`]'s zero arm), not
/// the empty `ones[0]`. LN sets `ret = negword` up front rather than recursing
/// (unlike FF/HA), so this mirrors that structure exactly.
fn cardinal_from_str(s: &str) -> Result<String> {
    // n = str(number).strip()
    let n_full = s.trim();
    // if n.startswith("-"): n = n[1:]; ret = self.negword  else: ret = ""
    let (n, ret) = match n_full.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD),
        None => (n_full, ""),
    };

    if let Some((left, right)) = n.split_once('.') {
        // ret += self._int_to_word(int(left)) + " " + self.pointword + " "
        let left_int = parse_int(left)?;
        let mut out = String::from(ret);
        out.push_str(&int_to_word(&left_int));
        out.push(' ');
        out.push_str(POINTWORD);
        out.push(' ');
        // for digit in right: ret += self._int_to_word(int(digit)) + " "
        for ch in right.chars() {
            let d = parse_digit(ch)?;
            out.push_str(&int_to_word(&d));
            out.push(' ');
        }
        // return ret.strip()
        Ok(out.trim().to_string())
    } else {
        // return (ret + self._int_to_word(int(n))).strip()
        let ni = parse_int(n)?;
        Ok(format!("{}{}", ret, int_to_word(&ni)).trim().to_string())
    }
}

pub struct LangLn {
    /// `Num2Word_LN.CURRENCY_FORMS`. Built once in [`LangLn::new`] and only
    /// read afterwards — the generated registry parks each language in a
    /// `OnceLock` and calls `new` through `get_or_init`, so this table is
    /// constructed once per process rather than once per conversion.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — what `.get`'s default falls
    /// back to for an unknown code (quirk 6).
    ///
    /// Python re-evaluates that expression on every call, and under CPython's
    /// insertion-ordered dicts it resolves to the *first inserted* entry:
    /// `CDF`. A `HashMap` has no first element, so the choice is pinned here
    /// rather than left to iteration order.
    fallback_forms: CurrencyForms,
}

impl Default for LangLn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLn {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::new();
        // Insertion order mirrors the Python literal; `fallback_forms` below
        // depends on CDF being the first entry. Arity is load-bearing:
        // `to_currency` indexes `cr1[0]`/`cr1[1]` and `cr2[0]`/`cr2[1]`, so
        // both forms of both tuples must survive verbatim — including CDF's,
        // where singular and plural are deliberately identical.
        currency_forms.insert(
            "CDF",
            CurrencyForms::new(&["faranga", "faranga"], &["santimi", "santimi"]),
        );
        // USD/EUR were English ("dollars", "cents", "euros"); no reliable Lingala cent noun was found, so
        // they raise NotImplementedError (#222).
        let fallback_forms = currency_forms
            .get("CDF")
            .expect("CURRENCY_FORMS[\"CDF\"] is inserted directly above")
            .clone();
        LangLn {
            currency_forms,
            fallback_forms,
        }
    }
}

impl Lang for LangLn {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "CDF"
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
    /// ...
    /// return (ret + self._int_to_word(int(n))).strip()
    /// ```
    /// For integer input the `str()`/`int()` round-trip is an identity and the
    /// `"." in n` branch is unreachable, so the observable behaviour reduces
    /// to: strip the sign, word the magnitude, prefix `negword`. The trailing
    /// `.strip()` is a no-op on every reachable value (nothing produces
    /// leading or trailing whitespace) but is kept for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Python: `return self.to_cardinal(number) + "-e"`. No negative guard, no
    /// separator — the suffix fuses onto the final word (and onto the digit
    /// fallback for values >= 10^9).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-e", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "."` — language-independent, and keeps
    /// the minus sign for negatives.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    // to_year is intentionally not overridden: Python's
    // `to_year(self, val, longval=True)` ignores `longval` and returns
    // `self.to_cardinal(val)`, which is precisely what the trait default does.

    /// The float/Decimal branch of `Num2Word_LN.to_cardinal`.
    ///
    /// LN does **not** override `to_cardinal_float`; it overrides `to_cardinal`
    /// and handles non-integers inline via `str(number).split(".", 1)`. So the
    /// inherited `default_to_cardinal_float` (which drives `base.float2tuple`)
    /// is the wrong engine — it re-derives the fraction through binary
    /// arithmetic and can drop a low-order digit at large magnitude. This
    /// override rebuilds `str(number)` and runs LN's own string algorithm.
    ///
    /// `precision_override` is ignored: `Num2Word_LN.to_cardinal(self, number)`
    /// takes no `precision=` parameter and never reads `self.precision`, so the
    /// kwarg has no effect in Python either — verified live:
    /// `num2words(2.675, lang="ln", precision=1)` is unchanged. The
    /// reconstructed string uses the value's repr-derived precision, exactly as
    /// `str()` does.
    ///
    /// Faithfully reproduced quirks:
    ///   * `1.0` (float) -> `"moko point zero"`: `str(1.0)` is `"1.0"`, so the
    ///     `"."` branch fires and the trailing `"0"` digit -> `"zero"`.
    ///   * `Decimal("1.10")` -> `"moko point moko zero"`: the trailing zero is a
    ///     real fractional digit (unlike the float `1.1`).
    ///   * `Decimal("98746251323029.99")` -> `"98746251323029 point libwá
    ///     libwá"`: the >=10^9 integer part falls off `int_to_word`'s cliff to
    ///     bare digits (issue #603 value), but the fraction is still spelled.
    ///   * A negative with a zero integer part keeps its sign because the sign
    ///     lives in the *string* (`"-0.5"`), not in a truncated int:
    ///     `-0.5` -> `"minus zero point mítáno"`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        cardinal_from_str(&float_to_str(value))
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "mítáno point zero", `-0.0` ->
    /// "minus zero point zero", `Decimal("5.00")` -> "mítáno point zero zero".
    /// The base default's whole-value integer shortcut must not fire here.
    /// Exponent-form values (`1e16`, `Decimal("1E+2")`) raise `int()`'s
    /// ValueError from inside the string algorithm, exactly as Python.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-e"`, where
    /// that cardinal is the `str(number)` algorithm — so "mítáno point
    /// zero-e" for 5.0, and the exponent forms raise ValueError before the
    /// suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-e", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, verbatim.
    /// `repr_str` is the binding's Python `str(value)`, exactly the string
    /// Python concatenates — so even exponent forms come out fine:
    /// `to_ordinal_num(1e16)` == "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: LN's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// LN inherits Base's `str_to_number` (`Decimal(value)`), which parses
    /// "Infinity"/"-Infinity"/"NaN" *successfully*. The mode-dependent outcome
    /// is served natively by `inf_result` / `nan_result` below, so nothing
    /// punts to the Python fallback.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity` per mode. LN's converters read
    /// `str(number)`:
    ///
    /// * `to_cardinal` (also `to_ordinal` = cardinal + "-e", and `to_year` =
    ///   cardinal) strip the sign then `int("Infinity")` → `ValueError` — not
    ///   the base OverflowError.
    /// * `to_ordinal_num` is `str(number) + "."`; nothing is parsed, so it
    ///   yields "Infinity." / "-Infinity.".
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}.",
                if negative { "-Infinity" } else { "Infinity" }
            )),
            // Sign is peeled off the string before `int()`, so the message
            // quotes the bare token.
            _ => Err(parse_int("Infinity").unwrap_err()),
        }
    }

    /// `Decimal('NaN')` per mode. The parsing modes `int("NaN")` → ValueError;
    /// `to_ordinal_num` returns "NaN." unparsed.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaN.".to_string()),
            _ => Err(parse_int("NaN").unwrap_err()),
        }
    }

    // ---- currency ------------------------------------------------------
    //
    // Only the three hooks below are overridden. LN defines no
    // CURRENCY_ADJECTIVES and no CURRENCY_PRECISION, and leaves `pluralize`,
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` at
    // `Num2Word_Base`, whose behaviour the trait defaults already reproduce:
    //
    //   * `currency_adjective`  -> None; LN's CURRENCY_ADJECTIVES is `{}` and
    //     `to_currency` never calls `prefix_currency` anyway.
    //   * `currency_precision`  -> 100; LN's CURRENCY_PRECISION is `{}`, so
    //     `.get(code, 100)` is 100 for every code. Only `to_cheque` reads it
    //     (LN's `to_currency` hardcodes two digits instead).
    //   * `pluralize`           -> the default raises NotImplementedError, and
    //     that is correct: LN inherits the abstract `Num2Word_Base.pluralize`
    //     and nothing reachable calls it. `to_currency` is overridden and picks
    //     forms by hand; base's `to_cheque` never pluralizes.
    //   * `money_verbose`       -> `self.to_cardinal(number)`, which routes
    //     through LN's `to_cardinal` override. This is what gives `to_cheque`
    //     its Lingala words.
    //   * `cents_verbose` / `cents_terse` -> unreachable; LN's `to_currency`
    //     replaces the only code path that would call them.

    /// `self.__class__.__name__`, for the inherited `to_cheque`'s
    /// `NotImplementedError` message. Verified verbatim against the
    /// interpreter: `Currency code "GBP" not implemented for "Num2Word_LN"`.
    fn lang_name(&self) -> &str {
        "Num2Word_LN"
    }

    /// `CURRENCY_FORMS[code]` — a strict lookup that misses for anything but
    /// CDF/USD/EUR.
    ///
    /// Deliberately **not** the `.get(code, <CDF>)` fallback of quirk 6: that
    /// fallback is local to `to_currency`. The inherited `to_cheque`
    /// subscripts `CURRENCY_FORMS[currency]` and turns the `KeyError` into
    /// `NotImplementedError`, so it must still see a miss here — that is
    /// exactly what makes `cheque:GBP` raise while `currency:GBP` quietly
    /// prints faranga. Folding the fallback in here would silence the six
    /// `NotImplementedError` cheque rows in the corpus.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_LN.to_currency`.
    ///
    /// LN ignores `base.to_currency` wholesale — no `parse_currency_parts`, no
    /// `divisor`, no `pluralize`. It slices the decimal *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// # The `separator` argument is not what Python's default would be
    ///
    /// LN's own signature defaults to `separator=" "`, but the trait cannot
    /// carry a per-language default argument: the dispatcher resolves it
    /// before the call and passes `Num2Word_Base`'s `","`. The frozen corpus
    /// was generated through Python — where LN's `" "` applies — so `","` is
    /// read back as the "unset" sentinel it is. This is exact both for a caller
    /// who omits `separator` and for one who passes anything other than `","`,
    /// and wrong only for an explicit `separator=","`, a case the bridge cannot
    /// distinguish from the default anyway. See `concerns`; `lang_br.rs` (whose
    /// Python `to_currency` is byte-identical to LN's) and `lang_es.rs` take
    /// the same approach for the same reason.
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
        // Python accepts `adjective` and never reads it: LN has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency, so
        // `adjective=True` is byte-identical to the plain call.
        let _ = adjective;

        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` runs *before*
        // `str(val)`, so the string never carries a sign. The abs() is
        // conditional in Python, so it stays conditional here.
        //
        // The Int/Decimal split is preserved even though LN, uniquely, cannot
        // observe it in the output: it never calls `isinstance(val, int)`, and
        // an int `1` ("1" -> no cents) and a float `1.0` ("1.0" -> right == 0,
        // falsy -> no cents) converge on "moko euro" by different routes. The
        // branch still matters because `str()` differs between the two.
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

        // `str(val).split(".")` splits on every dot; Python then reads only
        // parts[0] and parts[1], so any trailing fragment is ignored either
        // way.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`.
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads, so "5" -> "50" (0.5 is 50
        // subunits) and "01" -> "01" (0.01 is 1). Sliced by chars, not bytes.
        // A whole float gives "0" -> "00" -> 0, which is falsy (quirk 8).
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

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — quirk 6.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::one();

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`. Note that this
        // is `self._int_to_word(left)`, *not* `self.to_cardinal(left)` — so the
        // 10^9 digit fallback applies here too. Zero takes the plural
        // ("zero euros", quirk 10).
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 8). `cents=False`
        // drops it too, with no terse fallback (quirk 9).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — raw, keeping the trailing space of
        // "minus ".
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input, but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }

    // to_cheque is intentionally not overridden. `Num2Word_LN` does not define
    // it, so Python uses `Num2Word_Base.to_cheque`, which the trait default
    // (`currency::default_to_cheque`) reproduces: strict CURRENCY_FORMS lookup
    // -> NotImplementedError, divisor 100 from the empty CURRENCY_PRECISION,
    // `int(abs_val)` whole part, `"%0*d/%d"` fraction, `_money_verbose` (->
    // LN's to_cardinal) for the words, `cr1[-1]` for the always-plural unit,
    // and `.upper()` over the lot. Traced against the interpreter for
    // cheque:EUR/USD/GBP at 1234.56.
}
