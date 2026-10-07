//! Port of `lang_SA.py` (Sanskrit).
//!
//! Shape: **self-contained**. `Num2Word_SA` subclasses `Num2Word_Base` but its
//! `setup()` defines only `ones`/`tens`/`hundred`/`thousand`/`million` — none
//! of `high_numwords`/`mid_numwords`/`low_numwords`. So the `hasattr` guard in
//! `Num2Word_Base.__init__` never fires: `self.cards` is never built and
//! `self.MAXVAL` is **never set**. `to_cardinal` is overridden outright and
//! drives a recursive `_int_to_word`, so `cards`/`maxval`/`merge` stay at their
//! trait defaults here and there is **no overflow check** — no input of any
//! magnitude raises. (Touching `self.MAXVAL` would be an `AttributeError`, but
//! nothing in the four in-scope modes reaches it.)
//!
//! Inherited from `Num2Word_Base` unchanged: `to_cheque`, `pluralize`,
//! `_money_verbose`, `_cents_verbose`, `_cents_terse`, and the empty
//! `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION` dicts. SA overrides all four of
//! `to_cardinal`, `to_ordinal`, `to_ordinal_num` and `to_year`, plus
//! `to_currency` (wholesale — it never calls `super()`).
//!
//! # Currency
//!
//! `Num2Word_SA.to_currency` is a **complete replacement** for
//! `Num2Word_Base.to_currency`, not a refinement of it. It shares almost
//! nothing with the base implementation, and the differences are all
//! observable:
//!
//! * It never calls `pluralize`, `_money_verbose`, `_cents_verbose` or
//!   `_cents_terse`. Plurals are picked inline with `cr1[1] if left != 1 else
//!   cr1[0]`, and both number words come from `_int_to_word` directly. Those
//!   four hooks are therefore **unreachable** for this language and stay at
//!   their trait defaults (including `pluralize`, which raises).
//! * It never consults `CURRENCY_PRECISION`. There is no divisor, no
//!   `parse_currency_parts`, and no `ROUND_HALF_UP` quantize — see quirks 10
//!   and 11 below.
//! * An unknown currency code does **not** raise. It silently falls back to
//!   the first entry of `CURRENCY_FORMS` (quirk 12).
//! * Its `separator` default is `" "`, not `Num2Word_Base`'s `","` (quirk 14).
//!
//! `to_cheque` is the opposite story: SA does not define it at all, so
//! `Num2Word_Base.to_cheque` runs unchanged. That means the two entry points
//! disagree about unknown codes — cheque indexes `CURRENCY_FORMS[currency]`
//! directly and raises `NotImplementedError`, while currency falls back. Both
//! behaviours are pinned by the corpus and both are reproduced here.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, and each is pinned by the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the decimal digits.** The
//!    `elif` ladder stops at `number < 1000000000`; the final `else` is
//!    literally `return str(number)  # Fallback for very large numbers`. So
//!    `to_cardinal(10**9)` == `"1000000000"` and `to_cardinal(1234567890)` ==
//!    `"1234567890"` — digits, not words, with no error raised. This holds all
//!    the way up (corpus pins 10^21 → `"1000000000000000000000"`), which is why
//!    the value must stay a `BigInt`: it is unbounded on this path. See
//!    [`LangSa::int_to_word`].
//! 2. **Zero, minus and the decimal word (fixed, gladiaio/num2words2#154).**
//!    Python's `self.ones[0]` is `""`, so
//!    `return self.ones[0] if self.ones[0] else "zero"` always answered with
//!    the English "zero", and `negword`/`pointword` were the English
//!    `"minus "`/`"point"`. This port uses शून्यम् (neuter nominative, like
//!    एकम्), ऋण and दशमलव, the words the Nepali/Marathi/Hindi modules use.
//! 3. **The negative word is *not* stripped** by `to_cardinal` the way
//!    `Num2Word_Base` does it (base uses `"%s " % self.negword.strip()`; SA
//!    just concatenates `self.negword`). The trailing space in `"ऋण "` is
//!    what separates it from the number, so `to_cardinal(-1)` == `"ऋण एकम्"`.
//! 4. **Teens are compounds, not distinct words.** `tens[1]` is "दश" (ten) and
//!    11 renders as `"दश एकम्"` (ten one), because the `number < 100` branch
//!    has no teen special-case. Likewise 21 == `"विंशति एकम्"`.
//! 5. **Hundreds/thousands/millions always carry an explicit multiplier**, so
//!    100 == `"एकम् शतम्"` (one hundred), never bare `"शतम्"`.
//! 6. **`million` is "दशलक्षम्"** — literally *ten lakh*. Correct value
//!    (10^6), but the word is built on the Indian scale. Kept verbatim.
//! 7. **`to_ordinal` does not call `verify_ordinal`**, so negatives and zero
//!    are accepted rather than raising `TypeError`: `to_ordinal(0)` ==
//!    `"शून्यम्-मः"`, `to_ordinal(-1)` == `"ऋण एकम्-मः"`. The suffix is glued
//!    to the *whole* cardinal with no space, so it lands on the last word.
//! 8. **`to_ordinal_num` ignores the language entirely** and returns
//!    `str(number) + "."` — `"-1."` for -1. Note this is the raw input, so the
//!    minus sign survives.
//! 9. **`to_year` ignores its `longval` parameter** and is a bare alias for
//!    `to_cardinal`, so negative years get `"ऋण "` rather than an era
//!    suffix: `to_year(-44)` == `"ऋण चत्वारिंशत् चत्वारि"`.
//!
//! 10. **`to_currency` reads the cents off the decimal *string*, not the
//!     number.** It does `str(val).split(".")` and then
//!     `int(parts[1][:2].ljust(2, "0"))`. So the fraction is **truncated** to
//!     two digits and then right-padded, never rounded: `0.5` yields 50 cents
//!     (`"5"` → `"50"`), and a hypothetical `0.999` would yield 99, not 100.
//!     `CURRENCY_PRECISION` is never consulted, so this holds for *every* code
//!     — `to_currency(12.34, "KWD")` reports 34 subunits, not 340, and
//!     `to_currency(12.34, "JPY")` reports 34 subunits rather than rounding to
//!     a whole yen. The 3-decimal and 0-decimal corpus rows are byte-identical
//!     to the EUR ones apart from the unit names.
//! 11. **Cents of exactly zero are dropped, even from a float.** The guard is
//!     `if cents and right`, and `right` is a plain int, so `1.0` → `str` is
//!     `"1.0"` → `parts[1]` is `"0"` (truthy, so the `len(parts) > 1 and
//!     parts[1]` test passes) → `right = int("00") = 0` (falsy) → no cents
//!     segment. `to_currency(1.0, "EUR")` == `"एकम् euro"`, identical to the
//!     int `1`. This is the one language where the int/float split that
//!     `base.to_currency` fights so hard to preserve makes **no** difference to
//!     the output.
//! 12. **An unknown currency code silently becomes INR.** The lookup is
//!     `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`, and the
//!     class dict is written in the literal order INR, USD, EUR — so the
//!     default is the *INR* entry. `to_currency(1, "GBP")` == `"एकम् रूप्यकाणि"`:
//!     Sanskrit rupees for a British pound, no error. The inherited `to_cheque`
//!     uses `CURRENCY_FORMS[currency]` instead and *does* raise
//!     `NotImplementedError` for the same code.
//! 13. **INR's singular and plural are the same word** — `("रूप्यकाणि",
//!     "रूप्यकाणि")`, likewise `("पैसा", "पैसा")` — so the `left != 1` branch is
//!     invisible for INR and for every code that falls back to it. It is only
//!     observable on USD/EUR ("एकम् euro" vs "द्वे euros").
//! 14. **The unit words for USD and EUR are English**, not Sanskrit:
//!     `("dollar", "dollars")`, `("euro", "euros")`, `("cent", "cents")`. Only
//!     INR is transliterated, so `to_currency(12.34, "EUR")` mixes scripts:
//!     `"दश द्वे euros त्रिंशत् चत्वारि cents"`.
//!
//! # Errors
//!
//! The four integer modes and `to_currency` are total: the corpus has zero
//! `ok: false` rows for `cardinal`/`ordinal`/`ordinal_num`/`year`/`currency`,
//! and quirk 12 means even a bogus currency code succeeds.
//!
//! `to_cheque` is the only fallible entry point. `Num2Word_Base.to_cheque`
//! catches the `KeyError` from `CURRENCY_FORMS[currency]` and re-raises
//! `NotImplementedError('Currency code "%s" not implemented for "%s"')`, which
//! is what the corpus records for GBP/JPY/KWD/BHD/CNY/CHF. That message is
//! produced by [`crate::currency::default_to_cheque`] off `currency_forms()`
//! returning `None` and `lang_name()` returning `"Num2Word_SA"`.
//!
//! One path that no corpus row reaches can raise [`N2WError::Value`]:
//! `int(parts[0])` raises `ValueError` on a non-numeric token, and the only
//! non-numeric token Python can produce here is a `repr` in scientific
//! notation (`str(1e-05)` == `"1e-05"`, and `int("1e-05")` → `ValueError`).
//! `BigDecimal`'s `Display` switches to exponential form too, so
//! `BigInt::from_str` rejects those tokens and we surface `Value` — the exact
//! type Python raises.
//!
//! The two switchover *thresholds* do not line up, and they cannot be made to.
//! See `SCIENTIFIC-NOTATION GAP` on [`LangSa::to_currency`] — this is a
//! limitation of the `CurrencyValue` contract for this language, not a choice
//! made here. No corpus row is affected.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). Sanskrit has no attested dollar, euro or cent noun, so USD and
//! EUR raise NotImplementedError. Examples in these docs that quote English
//! nouns record Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`, in Sanskrit (Python: English `"minus "`, #154). Note the
/// trailing space — it is load-bearing (quirk 3).
const NEGWORD: &str = "ऋण ";

/// `self.pointword`, in Sanskrit (Python: English `"point"`, #154).
const POINTWORD: &str = "दशमलव";

/// Emitted for 0 because `self.ones[0]` is the empty string (quirk 2).
/// Python says the English "zero"; this port the Sanskrit शून्यम्.
const ZERO_WORD: &str = "शून्यम्";

/// `self.ones`, keys 0..=9. Index 0 is `""` in Python and never reached as a
/// word: the `number == 0` guard fires first, and the hundreds branch only
/// indexes it with 1..=9.
const ONES: [&str; 10] = [
    "",
    "एकम्",
    "द्वे",
    "त्रीणि",
    "चत्वारि",
    "पञ्च",
    "षट्",
    "सप्त",
    "अष्ट",
    "नव",
];

/// `self.tens`, keys 0..=9. Index 0 is `""` and unreachable (the `number < 100`
/// branch is only entered for `number >= 10`, so `number // 10 >= 1`).
const TENS: [&str; 10] = [
    "",
    "दश",
    "विंशति",
    "त्रिंशत्",
    "चत्वारिंशत्",
    "पञ्चाशत्",
    "षष्टि",
    "सप्तति",
    "अशीति",
    "नवति",
];

const HUNDRED: &str = "शतम्";
const THOUSAND: &str = "सहस्रम्";
/// `self.million` — "ten lakh" by etymology, 10^6 by value (quirk 6).
const MILLION: &str = "दशलक्षम्";

/// The ceiling of the `_int_to_word` word ladder. At or above this, Python
/// returns `str(number)` (quirk 1).
const FALLBACK_LIMIT: u64 = 1_000_000_000;

/// `Num2Word_SA.to_currency`'s own `separator=" "` default.
///
/// SA narrows `Num2Word_Base`'s `","`, and every currency corpus row is
/// generated by `num2words(v, lang="sa", to="currency", currency=C)` with **no**
/// separator — so `" "` is what each expected string carries ("... euros त्रिंशत्
/// चत्वारि cents", not "... euros,त्रिंशत् ...").
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s `separator=","` default, used here as the
/// "caller did not pass one" sentinel — see [`LangSa::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// The key whose value is `list(CURRENCY_FORMS.values())[0]` — the fallback an
/// unknown code lands on (quirk 12). Python's dict literal is written INR,
/// USD, EUR, and dicts have preserved insertion order since 3.7, so the first
/// value is INR's.
const FALLBACK_CURRENCY: &str = "INR";

pub struct LangSa {
    /// `Num2Word_SA.CURRENCY_FORMS`, built once in [`LangSa::new`].
    ///
    /// A `HashMap` is enough for lookup, but it cannot answer "the first
    /// value", so `fallback_forms` caches that separately rather than this
    /// being an ordered map — see [`FALLBACK_CURRENCY`].
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(CURRENCY_FORMS.values())[0]`, resolved once. A clone of the INR
    /// entry: it is two short strings, and holding it by value keeps the
    /// struct free of a self-referential borrow.
    fallback_forms: CurrencyForms,
}

impl Default for LangSa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSa {
    pub fn new() -> Self {
        // Python's class-level dict literal, in source order. Note quirk 14:
        // only INR is in Devanagari; USD and EUR carry English words.
        let mut currency_forms = HashMap::with_capacity(3);
        currency_forms.insert(
            "INR",
            CurrencyForms::new(&["रूप्यकाणि", "रूप्यकाणि"], &["पैसा", "पैसा"]),
        );
        // USD/EUR were English ("dollars", "cents", "euros"); Sanskrit has no attested dollar, euro or cent noun, so
        // they raise NotImplementedError (#222).

        let fallback_forms = currency_forms[FALLBACK_CURRENCY].clone();

        LangSa {
            currency_forms,
            fallback_forms,
        }
    }

    /// Port of `Num2Word_SA._int_to_word`.
    ///
    /// The negative branch (`negword + _int_to_word(abs(number))`) is
    /// reproduced for fidelity but is **unreachable from every mode**:
    /// `to_cardinal` strips the sign textually before calling in, so `int(n)`
    /// is always non-negative, and `to_currency` — the only other caller —
    /// takes `abs` before it stringifies. Nothing ever hands this a negative.
    fn int_to_word(&self, number: &BigInt) -> String {
        // Python: `self.ones[0] if self.ones[0] else "zero"` — ones[0] is ""
        // (falsy), so this is unconditionally the zero word.
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // The `else` arm of Python's ladder: values >= 10^9 stringify. This is
        // the only unbounded path, and it is exactly why `number` is a BigInt.
        let limit = BigInt::from(FALLBACK_LIMIT);
        if *number >= limit {
            return number.to_string();
        }

        // Proven bounded: 0 < number < 10^9, so u64 is safe here (and only
        // here). Everything below mirrors Python's `//` and `%`, which agree
        // with Rust's on non-negative operands.
        let n = number
            .to_u64()
            .expect("0 < number < 10^9 was just proven, so u64 conversion is total");
        self.int_to_word_small(n)
    }

    /// The bounded tail of `_int_to_word`, for `0 < n < 10^9`.
    fn int_to_word_small(&self, n: u64) -> String {
        if n < 10 {
            return ONES[n as usize].to_string();
        }

        if n < 100 {
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                return TENS[tens_val].to_string();
            }
            // No teen special-case: 11 -> "दश एकम्" (quirk 4).
            return format!("{} {}", TENS[tens_val], ONES[ones_val]);
        }

        if n < 1_000 {
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            // Always an explicit multiplier: 100 -> "एकम् शतम्" (quirk 5).
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            return result;
        }

        if n < 1_000_000 {
            let thousands_val = n / 1_000;
            let remainder = n % 1_000;
            let mut result = format!("{} {}", self.int_to_word_small(thousands_val), THOUSAND);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word_small(remainder));
            }
            return result;
        }

        // n < 10^9 is guaranteed by the caller, so this is the last arm.
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

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. `Num2Word_SA.to_cardinal` is driven
/// entirely by this string — `str(1e16) == "1e+16"` has no `"."`, so `int()`
/// raises `ValueError` there. Mirrors `lang_sk.rs`'s `python_repr_f64`.
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
/// `< -6` — so `Decimal("1E+2")` prints `"1E+2"` and feeds `int()` a
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

/// Python's `int(s)` on a fragment of `str(number)`: only an optionally
/// signed ASCII digit run parses; anything with an `e`/`E` (exponent form,
/// "Infinity") raises `ValueError` with the literal quoted — exactly the
/// error `to_cardinal(1e16)` shows.
fn py_int(s: &str) -> Result<BigInt> {
    let err = || N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s));
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit()) {
        return Err(err());
    }
    let n: BigInt = body.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

impl Lang for LangSa {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "INR"
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

    /// Port of `Num2Word_SA.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and splits on `"."`; `str(int)` never
    /// contains one, so integers always take the `else` branch. The float
    /// branch (`pointword`, per-digit decimals) is out of scope.
    ///
    /// The sign is handled *textually* — `n[1:]` on the decimal string — not
    /// arithmetically, and `self.negword` goes in raw (trailing space intact,
    /// no `.strip()` as the base class does). The trailing `.strip()` on the
    /// joined result is a no-op for every in-scope input, since `_int_to_word`
    /// never returns a space-padded string; it is kept for fidelity.
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

    /// Port of `Num2Word_SA.to_ordinal`: the cardinal with `"-मः"` glued on.
    ///
    /// No `verify_ordinal` call, so 0 and negatives pass through (quirk 7).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}-मः", cardinal))
    }

    /// Port of `Num2Word_SA.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Language-independent, and the sign survives: -1 -> "-1." (quirk 8).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_SA.to_year`: a bare alias for `to_cardinal`.
    ///
    /// The `longval=True` parameter is accepted and ignored, and there is no
    /// BC/era handling — negative years just get "ऋण " (quirk 9).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / `Decimal` cardinal path — the `"."` branch of
    /// `Num2Word_SA.to_cardinal`.
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
    /// SA overrides `to_cardinal`, **not** `to_cardinal_float`, and the whole
    /// method turns on `str(number)`. So this path shares nothing with
    /// `base.float2tuple`: there is no `abs(value - pre) * 10**precision`
    /// arithmetic, no `round()`, and no `< 0.01` rescue heuristic. Python's own
    /// `repr(float)` already yields the clean shortest-round-trip digits
    /// (`str(2.675) == "2.675"`, `str(1.005) == "1.005"`), so the f64-artefact
    /// and banker's-rounding traps that bite the base path never arise here.
    ///
    /// Two consequences of being string-driven:
    ///
    /// * **The `precision=` kwarg is ignored.** `Num2Word_SA.to_cardinal` takes
    ///   only `number`; the kwarg never reaches this code in Python and changes
    ///   nothing there (`num2words(2.675, lang="sa", precision=1)` still emits
    ///   all three fractional digits). `precision_override` is dropped to match.
    /// * **A `Decimal`'s scale is load-bearing.** `str(Decimal("1.10"))` keeps
    ///   its trailing zero, so `1.10` renders as `"एकम् दशमलव एकम् शून्यम्"`, not
    ///   `"... एकम्"`. The fractional digits are rebuilt from the mantissa and
    ///   the scale, never from a normalised value.
    ///
    /// `pointword` is not run through `title()` (the base `to_cardinal_float`
    /// does; SA's inline code does not), which is invisible anyway since SA is
    /// not a titlecased language.
    ///
    /// # Scientific-notation gap (no corpus row, unfixable here)
    ///
    /// Python branches on `"." in str(number)`. For a float whose `repr` is in
    /// exponent form (`str(1e-05) == "1e-05"`, `str(1e16) == "1e+16"` — neither
    /// contains a `"."`), Python takes the *integer* branch and then
    /// `int("1e-05")` raises `ValueError`. Rust `f64` reprs that reach this file
    /// as a `precision >= 1` value are always in plain decimal notation, so this
    /// path is not reproduced; it matches no corpus row. Mirrors the same
    /// boundary limitation documented on `to_currency`.
    ///
    /// # Signed-zero `Decimal` gap (no corpus row, unfixable here)
    ///
    /// The float arm keeps `-0.0` negative — `str(-0.0) == "-0.0"` starts with
    /// `"-"`, and `f64::is_sign_negative()` reports it — so `-0.0` renders as
    /// `"ऋण शून्यम् दशमलव शून्यम्"`, matching Python. The `Decimal` arm cannot:
    /// the py binding parses `str(Decimal("-0.0"))` into a `BigDecimal` whose
    /// mantissa is a plain `BigInt` with no negative zero, so `is_negative()`
    /// returns `false` and the `"ऋण "` is dropped. Recovering it would need
    /// the original `str(value)`, which the boundary has already discarded —
    /// the same information-loss class as the scientific-notation gap above,
    /// and equally out of scope for this file.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() — repr(float) / str(Decimal), exponent forms
        // included: `str(1e16)` is "1e+16" (no "."), so the else branch feeds
        // it to int() and raises ValueError, exactly as Python.
        let n_full = match value {
            FloatValue::Float { value, .. } => python_repr_f64(*value),
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
        };
        let n_full = n_full.trim();

        // if n.startswith("-"): n = n[1:]; ret = self.negword  else: ret = ""
        let (n, neg) = match n_full.strip_prefix('-') {
            Some(rest) => (rest, true),
            None => (n_full, false),
        };

        // `ret = self.negword` ("ऋण ", trailing space intact — quirk 3) or "".
        let mut ret = String::new();
        if neg {
            ret.push_str(NEGWORD);
        }

        if let Some((left, right)) = n.split_once('.') {
            // `ret += _int_to_word(int(left)) + " " + self.pointword + " "`
            ret.push_str(&self.int_to_word(&py_int(left)?));
            ret.push(' ');
            ret.push_str(self.pointword());
            ret.push(' ');
            // `for digit in right: ret += _int_to_word(int(digit)) + " "` —
            // `int('e')` (the exponent of "1.5e+16") raises ValueError.
            for ch in right.chars() {
                let mut buf = [0u8; 4];
                let d = py_int(ch.encode_utf8(&mut buf))?;
                ret.push_str(&self.int_to_word(&d));
                ret.push(' ');
            }
            Ok(ret.trim().to_string())
        } else {
            // The integer branch: `(ret + _int_to_word(int(n))).strip()` —
            // exponent forms ("1e+16", "1E+2") raise ValueError here.
            ret.push_str(&self.int_to_word(&py_int(n)?));
            Ok(ret.trim().to_string())
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "पञ्च दशमलव शून्यम्", `-0.0` ->
    /// "ऋण शून्यम् दशमलव शून्यम्", `Decimal("5.00")` -> "पञ्च दशमलव शून्यम् शून्यम्".
    /// The base default's whole-value integer shortcut must not fire here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-मः"` — the
    /// cardinal being the string algorithm above, suffix glued on raw.
    /// Exponent forms raise ValueError before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-मः", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is
    /// the binding's Python `str(value)`, so exponent forms echo verbatim:
    /// `to_ordinal_num(1e16)` == "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: SA's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// Base's `str_to_number` parses "Infinity" *successfully*; SA's
    /// ValueError comes later, from `int("Infinity")` inside `to_cardinal`.
    /// The non-finite sentinels pass straight through, and the mode-aware
    /// [`inf_result`](Self::inf_result) reproduces SA's ValueError natively
    /// (the base default's OverflowError would be wrong). NaN is left to the
    /// base `nan_result` default, whose ValueError already matches
    /// `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity`. SA's `to_cardinal` strips a
    /// leading "-" off `str(number)` and feeds the rest to `int()`, so
    /// `int("Infinity")` raises `ValueError: invalid literal for int() with
    /// base 10: 'Infinity'` (both signs quote 'Infinity'). `to_ordinal` glues
    /// its suffix on *after* that int(), so it raises the same way, and
    /// `to_year` == `to_cardinal`. `to_ordinal_num` is `str(number) + "."`
    /// and never calls `int()`, so it succeeds with "Infinity." /
    /// "-Infinity.". The base default would raise OverflowError instead.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        if to == "ordinal_num" {
            return Ok(format!("{}Infinity.", if negative { "-" } else { "" }));
        }
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'Infinity'".into(),
        ))
    }

    // ---- currency ------------------------------------------------------
    //
    // `pluralize`, `money_verbose`, `cents_verbose`, `cents_terse`,
    // `currency_adjective` and `currency_precision` are all left at their
    // trait defaults, and each for a reason:
    //
    // * `pluralize` / `cents_verbose` / `cents_terse` are **unreachable**.
    //   `to_currency` below inlines the plural choice and calls `int_to_word`
    //   directly, and `to_cheque` does not use them. The default `pluralize`
    //   raises, which is exactly what `Num2Word_Base.pluralize` does — SA
    //   never overrides it and never calls it.
    // * `money_verbose` is reached, but only from the inherited `to_cheque`.
    //   The default (`self.to_cardinal(n)`) *is* `Num2Word_Base._money_verbose`
    //   verbatim, and it dispatches to SA's `to_cardinal` override.
    // * `currency_adjective` → `None` matches `CURRENCY_ADJECTIVES = {}`
    //   (and `to_currency` ignores `adjective` regardless).
    // * `currency_precision` → 100 matches `CURRENCY_PRECISION = {}`, i.e.
    //   `.get(code, 100)`. Only `to_cheque` reads it; `to_currency` never does
    //   (quirk 10), which is why KWD/BHD/JPY behave like EUR here.

    fn lang_name(&self) -> &str {
        "Num2Word_SA"
    }

    /// `CURRENCY_FORMS[code]` — the **strict** lookup, with no fallback.
    ///
    /// This models the subscript in `Num2Word_Base.to_cheque`, whose `KeyError`
    /// becomes the `NotImplementedError` the corpus records for
    /// GBP/JPY/KWD/BHD/CNY/CHF. `to_currency` deliberately does *not* route
    /// through here: it uses `.get(code, <first value>)` and must never raise
    /// (quirk 12).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_SA.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="INR", cents=True,
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
    /// This shares no code with `default_to_currency` and must not delegate to
    /// it: there is no `parse_currency_parts`, no divisor, no rounding, and no
    /// `NotImplementedError`. Everything is derived from the decimal *string*
    /// (quirk 10).
    ///
    /// The `Int` / `Decimal` split still matters for `str(val)` — `str(1)` is
    /// `"1"` (no `"."`, so `right` is 0 by the `len(parts) > 1` test) while
    /// `str(1.0)` is `"1.0"` (a `"."`, so `right` is 0 by `int("00") == 0`
    /// instead). Both land on `right = 0` and produce the same output, which
    /// makes SA the one language where collapsing the two would be
    /// *unobservable* — reproduced faithfully anyway (quirk 11).
    ///
    /// # SCIENTIFIC-NOTATION GAP (known, unfixable here, no corpus row hit)
    ///
    /// Because SA re-derives everything from `str(val)`, it is the rare
    /// language where the *notation* of the repr — not just its value — is
    /// load-bearing. The `CurrencyValue` contract hands us a `BigDecimal`
    /// parsed from that repr, which preserves the value and the scale but
    /// **discards the notation**. `BigDecimal`'s `Display` then re-derives a
    /// string using its own thresholds (exponential when
    /// `scale - ndigits > 5`), and CPython's `repr(float)` uses different ones
    /// (exponential when `decpt <= -4 or decpt > 16`). Where they disagree, so
    /// do we:
    ///
    /// | value | Python `str` | Python result | here |
    /// |---|---|---|---|
    /// | `1e-05` | `"1e-05"` | `ValueError` | `"शून्यम् euros"` |
    /// | `1e-06` | `"1e-06"` | `ValueError` | `"शून्यम् euros"` |
    /// | `1.5e-05` | `"1.5e-05"` | `ValueError` | `"शून्यम् euros"` |
    /// | `12345678901234567.0` | `"1.2345678901234568e+16"` | `"एकम् euro विंशति त्रीणि cents"` | `"12345678901234568 euros"` |
    ///
    /// That last row is not a rounding difference, it is Python splitting
    /// `"1.2345678901234568e+16"` on `"."` and reading `left = int("1")`,
    /// `right = int("23")` — so 12.3 quadrillion renders as **one euro and
    /// twenty-three cents**. Reproducing it would require the raw repr.
    ///
    /// This is **not repairable in this file**, and not by reimplementing
    /// CPython's repr thresholds either: `1e-05` (a float) and
    /// `Decimal("0.00001")` parse to the *same* `BigDecimal`, yet Python
    /// raises `ValueError` for the first and returns `"शून्यम् euros"` for the
    /// second — `str(float)` and `str(Decimal)` use different rules. One
    /// `BigDecimal`, two correct answers: the information needed to choose has
    /// already been destroyed at the boundary. Closing this would mean
    /// `CurrencyValue::Decimal` carrying the original `str(value)` alongside
    /// the parsed number, which is a `currency.rs` change and out of scope
    /// here. Every value in the ordinary money range is unaffected, and all
    /// 108 currency + 9 cheque corpus rows pass.
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
        // Python accepts `adjective` and never reads it: SA has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency.
        let _ = adjective;

        // The trait passes whatever the caller gave, and the dispatcher's
        // "caller said nothing" value is `Num2Word_Base`'s `","` — not SA's
        // own `" "`. Treat it as the sentinel it is.
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`, then `str(val)`.
        // The abs() happens *first*, so the string never carries a sign — and
        // it is conditional in Python, so it stays conditional here.
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

        // `str(val).split(".")`: take parts[0] and parts[1]. Splitting on all
        // dots rather than just the first matters only for input Python could
        // never produce, but it is what `.split(".")` does.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads, so "5" -> "50" (0.5 is 50 cents)
        // and "01" -> "01" (0.01 is 1 cent). Sliced by chars, not bytes.
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

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code becomes INR rather than an error (quirk 12).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::one();

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            self.int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 11).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `self.negword + result` — raw, keeping the trailing space of
        // "ऋण ". Base would strip it and re-add one; SA does not (quirk 3).

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input — `int_to_word`
        // never returns padding and no currency form is blank — but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}
