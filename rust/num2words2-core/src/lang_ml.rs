//! Port of `lang_ML.py` (Malayalam).
//!
//! Registry check: `CONVERTER_CLASSES["ml"]` is `lang_ML.Num2Word_ML()`, which
//! is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_ML` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` skips the `self.cards = OrderedDict()` block
//! entirely: Python never builds `cards`, never calls `set_numwords`, and
//! never sets `MAXVAL`. `to_cardinal` is overridden outright and drives a
//! recursive `_int_to_word` over the Indian lakh/crore scale. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! no overflow check in Python; the port's `maxval` is 10^14 (bug 4).
//!
//! ML overrides all four of `to_cardinal`, `to_ordinal`, `to_ordinal_num` and
//! `to_year`, plus `to_currency`. Inherited from `Num2Word_Base` and left
//! untouched: `to_cheque`, `_money_verbose` and `CURRENCY_PRECISION`.
//!
//! # The currency surface
//!
//! `Num2Word_ML` defines `CURRENCY_FORMS` for INR/USD/EUR only, and overrides
//! `to_currency` with a hand-rolled implementation that shares almost nothing
//! with `Num2Word_Base.to_currency`. It never calls `pluralize`,
//! `_money_verbose`, `_cents_verbose`, `_cents_terse` or `parse_currency_parts`,
//! and it never consults `CURRENCY_PRECISION` — so those hooks stay at their
//! trait defaults here and [`LangMl::to_currency`] overrides the whole path.
//!
//! `to_cheque` is *not* overridden, so it runs `Num2Word_Base.to_cheque`
//! ([`crate::currency::default_to_cheque`]) unchanged. That reaches
//! `CURRENCY_FORMS` through `currency_forms()` and `CURRENCY_PRECISION` through
//! `currency_precision()`. ML declares no `CURRENCY_PRECISION` and the base's
//! is `{}`, so `.get(code, 100)` is always 100 — the trait default — and the
//! divisor is 100 even for KWD/BHD/JPY. The two entry points therefore disagree
//! about unknown codes, which is bug 7 below.
//!
//! `CURRENCY_ADJECTIVES` is `{}` (base) and ML's `to_currency` ignores its
//! `adjective` argument outright, so `currency_adjective` stays unimplemented.
//!
//! No cross-call mutable state: `setup()` assigns plain tables once at
//! construction and no method stashes a flag for another to consume. The
//! stateless Rust path is safe for the dispatcher's fast path.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong and is exactly
//! what Python emits — each is pinned by a row in `bench/corpus.jsonl`.
//!
//! 1. **Ordinals concatenate the "-ാം" suffix onto a cardinal that still ends
//!    in a chandrakkala (U+0D4D), producing non-words.** Malayalam ordinals
//!    are meant to elide that virama, and the five hardcoded special forms
//!    (1..=5) correctly do — `to_ordinal(1)` is "ഒന്നാം". But every other
//!    number takes the `cardinal + "ാം"` branch verbatim, so `to_ordinal(7)`
//!    is "ഏഴ്ാം" (chandrakkala then vowel sign AA) rather than "ഏഴാം", and
//!    `to_ordinal(1000)` is "ഒന്ന് ആയിരംാം" (anusvara then vowel sign AA).
//!    Note `to_ordinal(1)` != `to_cardinal(1) + "ാം"`: the special form is
//!    "ഒന്നാം" while the generic path would give "ഒന്ന്ാം". Kept verbatim.
//! 2. **`to_ordinal(0)` == "പൂജ്യംാം"** and **`to_ordinal(n)` for negative n
//!    prefixes the negword**, e.g. `to_ordinal(-1)` == "മൈനസ് ഒന്ന്ാം".
//!    Python has no `errmsg_negord` guard here because `to_ordinal` is
//!    overridden and never calls the base's checks. No error is raised.
//! 3. **Hundreds/thousands are not idiomatic**: the code always writes the
//!    multiplier, so 100 is "ഒന്ന് നൂറ്" ("one hundred") and 1000 is
//!    "ഒന്ന് ആയിരം" ("one thousand") where Malayalam would say "നൂറ്" /
//!    "ആയിരം" alone. Two parts are fixed (gladiaio/num2words2#247): tens
//!    composed as bare juxtaposition — 21 was "ഇരുപത് ഒന്ന്", now the fused
//!    "ഇരുപത്തിയൊന്ന്" ([`BELOW_HUNDRED`]) — and a lakh/crore multiplier of
//!    one takes the attributive form: "ഒരു ലക്ഷം", not "ഒന്ന് ലക്ഷം".
//! 4. **Past 10^9 Python returned the decimal digits (fixed,
//!    gladiaio/num2words2#147).** The final `else` of `_int_to_word` was
//!    `return str(number)`, so 10^9 was "1000000000". Malayalam has no word
//!    above കോടി in standard use; the port spells the crore count, as
//!    "നൂറ് കോടി" (hundred crore) and "ലക്ഷം കോടി" are used, and raises
//!    `OverflowError` from 10^14, where that count would itself need കോടി.
//! 5. **`to_year` emits an ASCII "BC " prefix** for negative years
//!    (`to_year(-44)` == "BC നാല്പത് നാല്") and applies no era word at all
//!    for positive ones — the `else` arm is a no-op `"" + self.to_cardinal(val)`.
//!    `abs(val)` is taken first, so the negword never appears alongside "BC ".
//! 6. **`to_ordinal_num` is a bare `str(number) + "."`**, giving "5." and,
//!    for negatives, "-5.". No Malayalam is involved and no sign handling.
//! 7. **An unknown currency code silently becomes rupees in `to_currency`,
//!    but raises in `to_cheque`.** `to_currency` does
//!    `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`, so
//!    `to_currency(2, "GBP")` == "രണ്ട് രൂപ" and `to_currency(12.34, "JPY")`
//!    == "പന്ത്രണ്ട് രൂപ മുപ്പത് നാല് പൈസ" — pounds and yen rendered as
//!    rupees and paisa, with no error. `to_cheque` is the inherited base
//!    version, which subscripts `CURRENCY_FORMS[currency]` and turns the
//!    KeyError into `NotImplementedError`. Both halves are corpus-pinned
//!    (`currency:GBP` succeeds, `cheque:GBP` raises), so the asymmetry is real
//!    and must survive: `currency_forms()` reports the strict truth for
//!    `to_cheque`, and the INR fallback lives inside [`LangMl::to_currency`].
//! 8. **`CURRENCY_PRECISION` is never consulted by `to_currency`**, so the
//!    subunit divisor is hardwired to the two-digit `[:2]` slice below. JPY (a
//!    0-decimal currency) still renders paisa, and KWD/BHD (3-decimal) are
//!    truncated to 2 — `to_currency(1234.56, "KWD")` ==
//!    "... രൂപ അമ്പത് ആറ് പൈസ", not 560 fils. Corpus-pinned for both.
//! 9. **`cents=False` drops the subunits entirely** rather than falling back to
//!    the terse digit form. `if cents and right:` guards the whole segment, so
//!    `to_currency(12.34, "EUR", cents=False)` == "പന്ത്രണ്ട് euros" where the
//!    base class would append "34". The same guard means a zero subunit is
//!    always omitted, so a *float* with no cents loses its cents segment too:
//!    `to_currency(1.0, "EUR")` == "ഒന്ന് euro", identical to the int `1`.
//!    This is the one place ML collapses the int/float distinction that
//!    `Num2Word_Base.to_currency` is careful to keep — see
//!    [`LangMl::split_parts`].
//! 10. **`to_currency` re-stringifies the value and parses the text**, so a
//!     float whose `repr` is scientific derails it: `1e16` raises `ValueError`
//!     from `int("1e+16")`, and `1.23e16` returns a *wrong answer* rather than
//!     an error — "ഒന്ന് രൂപ ഇരുപത് മൂന്ന് പൈസ" (one rupee, 23 paisa) for
//!     12,300,000,000,000,000. See [`LangMl::scientific_parts`].
//!
//! # Dead code preserved
//!
//! `_int_to_word`'s `number < 0` branch is unreachable from every in-scope
//! entry point: `to_cardinal` strips the minus from the *string* before
//! calling `int()`, so `_int_to_word` only ever receives a non-negative value.
//! It is reproduced in [`LangMl::int_to_word`] anyway so the function matches
//! its Python counterpart line for line.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use ഡോളർ / യൂറോ with സെന്റ്. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::str::FromStr;

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247).
///
/// Python had `ones`/`teens`/`tens` and juxtaposed a ten and a unit
/// ("ഇരുപത് ഒന്ന്" for 21). Malayalam composes 21..=99 from the ten's
/// combining form (ഇരുപത്തി-, മുപ്പത്തി-, ..., തൊണ്ണൂറ്റി-) and the unit,
/// written as one word, with a glide യ before a vowel: 21 ഇരുപത്തിയൊന്ന്,
/// 23 ഇരുപത്തിമൂന്ന്, 25 ഇരുപത്തിയഞ്ച് (Omniglot, Wikipedia "Malayalam
/// numerals", learnentry.com). 1..=20 and the round tens are the module's own
/// words. Index 0 is never read.
const BELOW_HUNDRED: [&str; 100] = [
    "", "ഒന്ന്", "രണ്ട്", "മൂന്ന്", "നാല്", "അഞ്ച്", "ആറ്", "ഏഴ്", "എട്ട്", "ഒൻപത്", // 0..9
    "പത്ത്", "പതിനൊന്ന്", "പന്ത്രണ്ട്", "പതിമൂന്ന്", "പതിനാല്", "പതിനഞ്ച്", "പതിനാറ്", "പതിനേഴ്", "പതിനെട്ട്", "പത്തൊൻപത്", // 10..19
    "ഇരുപത്", "ഇരുപത്തിയൊന്ന്", "ഇരുപത്തിരണ്ട്", "ഇരുപത്തിമൂന്ന്", "ഇരുപത്തിനാല്", "ഇരുപത്തിയഞ്ച്", "ഇരുപത്തിയാറ്", "ഇരുപത്തിയേഴ്", "ഇരുപത്തിയെട്ട്", "ഇരുപത്തിയൊൻപത്", // 20..29
    "മുപ്പത്", "മുപ്പത്തിയൊന്ന്", "മുപ്പത്തിരണ്ട്", "മുപ്പത്തിമൂന്ന്", "മുപ്പത്തിനാല്", "മുപ്പത്തിയഞ്ച്", "മുപ്പത്തിയാറ്", "മുപ്പത്തിയേഴ്", "മുപ്പത്തിയെട്ട്", "മുപ്പത്തിയൊൻപത്", // 30..39
    "നാല്പത്", "നാല്പത്തിയൊന്ന്", "നാല്പത്തിരണ്ട്", "നാല്പത്തിമൂന്ന്", "നാല്പത്തിനാല്", "നാല്പത്തിയഞ്ച്", "നാല്പത്തിയാറ്", "നാല്പത്തിയേഴ്", "നാല്പത്തിയെട്ട്", "നാല്പത്തിയൊൻപത്", // 40..49
    "അമ്പത്", "അമ്പത്തിയൊന്ന്", "അമ്പത്തിരണ്ട്", "അമ്പത്തിമൂന്ന്", "അമ്പത്തിനാല്", "അമ്പത്തിയഞ്ച്", "അമ്പത്തിയാറ്", "അമ്പത്തിയേഴ്", "അമ്പത്തിയെട്ട്", "അമ്പത്തിയൊൻപത്", // 50..59
    "അറുപത്", "അറുപത്തിയൊന്ന്", "അറുപത്തിരണ്ട്", "അറുപത്തിമൂന്ന്", "അറുപത്തിനാല്", "അറുപത്തിയഞ്ച്", "അറുപത്തിയാറ്", "അറുപത്തിയേഴ്", "അറുപത്തിയെട്ട്", "അറുപത്തിയൊൻപത്", // 60..69
    "എഴുപത്", "എഴുപത്തിയൊന്ന്", "എഴുപത്തിരണ്ട്", "എഴുപത്തിമൂന്ന്", "എഴുപത്തിനാല്", "എഴുപത്തിയഞ്ച്", "എഴുപത്തിയാറ്", "എഴുപത്തിയേഴ്", "എഴുപത്തിയെട്ട്", "എഴുപത്തിയൊൻപത്", // 70..79
    "എൺപത്", "എൺപത്തിയൊന്ന്", "എൺപത്തിരണ്ട്", "എൺപത്തിമൂന്ന്", "എൺപത്തിനാല്", "എൺപത്തിയഞ്ച്", "എൺപത്തിയാറ്", "എൺപത്തിയേഴ്", "എൺപത്തിയെട്ട്", "എൺപത്തിയൊൻപത്", // 80..89
    "തൊണ്ണൂറ്", "തൊണ്ണൂറ്റിയൊന്ന്", "തൊണ്ണൂറ്റിരണ്ട്", "തൊണ്ണൂറ്റിമൂന്ന്", "തൊണ്ണൂറ്റിനാല്", "തൊണ്ണൂറ്റിയഞ്ച്", "തൊണ്ണൂറ്റിയാറ്", "തൊണ്ണൂറ്റിയേഴ്", "തൊണ്ണൂറ്റിയെട്ട്", "തൊണ്ണൂറ്റിയൊൻപത്", // 90..99
];

const NEGWORD: &str = "മൈനസ് ";
const HUNDRED: &str = "നൂറ്";
const THOUSAND: &str = "ആയിരം";
const LAKH: &str = "ലക്ഷം";
const CRORE: &str = "കോടി";
const ZERO_WORD: &str = "പൂജ്യം";

/// The attributive "one" before ലക്ഷം/കോടി (#247): ഒരു ലക്ഷം, ഒരു കോടി,
/// where Python wrote the counting form ("ഒന്ന് ലക്ഷം").
const ONE_ATTRIBUTIVE: &str = "ഒരു";

/// The generic ordinal suffix: U+0D3E MALAYALAM VOWEL SIGN AA followed by
/// U+0D02 MALAYALAM SIGN ANUSVARA. Appended raw to the cardinal — see bug 1.
const ORD_SUFFIX: &str = "ാം";

// ---------------------------------------------------------------------------
// The float / Decimal cardinal path.
//
// `Num2Word_ML.to_cardinal` handles non-integers *inline* off `str(number)` —
// it never routes through `Num2Word_Base.to_cardinal_float`, so ML overrides
// `to_cardinal_float` (see `route` in the port report):
//
// ```python
// n = str(number).strip()
// if n.startswith("-"):
//     n = n[1:]; ret = self.negword
// else:
//     ret = ""
// if "." in n:
//     left, right = n.split(".", 1)
//     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//     ret += " ".join(self._int_to_word(int(d)) for d in right)
//     return ret.strip()
// else:
//     return (ret + self._int_to_word(int(n))).strip()
// ```
//
// So the fractional part is the *literal digit string* of `str(number)`
// spoken digit-by-digit — never a `post` integer, never zero-padded to a
// precision, and the f64-artefact `< 0.01` heuristic never runs (`2.675` is
// spoken "6 7 5", not repaired to 675). The entire specification is therefore
// Python's `repr(float)` / `str(Decimal)`, reproduced exactly by
// [`py_float_repr`] / [`py_decimal_str`]. `precision` (and the `precision=`
// kwarg) is set on the converter by `__init__.py` but ML's `to_cardinal`
// never reads `self.precision`, so it is ignored here too.
//
// The four pure helpers below (`shortest_digits`, `py_float_repr`,
// `py_decimal_str`, `py_int`) mirror the already-verified `lang_as.rs` /
// `lang_gu.rs` twins verbatim — the Python `to_cardinal` bodies are identical
// across all three; only the language tables and `_int_to_word` differ.

/// Shortest round-trip decimal digits of `a` and its decimal point position,
/// matching CPython/Gay's dtoa. Returns `(digits, decpt)` where the value is
/// `0.<digits> * 10**decpt`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with dtoa on the digit
/// *count* and almost always the digits; it disagrees only on **exact ties**,
/// where dtoa picks the candidate with an even last digit while Rust rounds
/// half up. The block below detects that tie with no bignum (write
/// `a = m * 2**e` with `m` odd, `q = digits.len() - decpt`; the tie is
/// `e + q + 1 == 0`, plus `5**-q | m` when `q < 0`) and steps to the even
/// neighbour, exactly as `lang_as.rs` documents and fuzzes.
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let mut digits: Vec<u8> = mant.bytes().filter(|c| *c != b'.').collect();
    let mut decpt: i32 = exp.parse::<i32>().expect("{:e} exponent is an integer") + 1;

    // Decompose a == m * 2**e exactly, then reduce m to odd.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mut m, mut e) = if biased == 0 {
        (frac, -1074i32) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };
    if m == 0 {
        // a == 0.0: dtoa reports digits "0", decpt 1. No tie to break.
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }
    let z = m.trailing_zeros() as i32;
    m >>= z;
    e += z;

    let q = digits.len() as i32 - decpt;
    let mut tie = e + q + 1 == 0;
    if tie && q < 0 {
        let r = -q as u32;
        tie = r <= 22 && m % 5u64.pow(r) == 0;
    }
    if !tie {
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }

    let last = digits[digits.len() - 1] - b'0';
    if last % 2 == 1 {
        if m % 4 == 1 {
            // k even: Python wants k, Rust gave k+1. Last digit is odd (hence
            // non-zero), so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k odd: Python wants k+1, Rust gave k. Carry like dtoa's roundoff.
            let mut i = digits.len();
            loop {
                if i == 0 {
                    digits.insert(0, b'1');
                    decpt += 1;
                    break;
                }
                i -= 1;
                if digits[i] == b'9' {
                    digits[i] = b'0';
                } else {
                    digits[i] += 1;
                    break;
                }
            }
        }
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), CPython's
/// `format_float_short(..., 'r', ...)`. Rust's own `{}` cannot stand in: it
/// never switches to exponent notation and prints `1`, not `1.0`, for integral
/// floats. Rules from `format_float_short`: exponent notation iff
/// `decpt <= -4 || decpt > 16`; the exponent is `%+.02d`; a trailing `.0` is
/// appended to an otherwise-integral fixed result but never in exponent form;
/// `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and the sign is
    // stripped textually into a negword.
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_digits(value.abs());
    let ndigits = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let exp = decpt - 1;
        let mut mant = String::from(&digits[..1]);
        if digits.len() > 1 {
            mant.push('.');
            mant.push_str(&digits[1..]);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - ndigits) as usize)
        )
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(Decimal)` — `Decimal.__str__` with the default context.
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`; the shim builds the value with
/// `BigDecimal::from_str(str(value))`, which preserves trailing zeros and
/// negative exponents (so `"1.10"` round-trips as `(110, 2)`). The
/// `leftdigits > -6` guard keeps every `"0".repeat(...)` bounded.
///
/// Caveat (same as `lang_as.rs`): a `BigDecimal` cannot represent `-0.0`, so a
/// fixed-notation negative zero loses its negword here. Beyond the exponent
/// threshold `int()` raises `ValueError` with a message that agrees either way.
/// Flagged in the port report.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // i128 so that `-scale` cannot overflow for a pathological i64::MIN scale.
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string(); // Decimal._int
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let d = dotplace as usize;
        (int_digits[..d].to_string(), format!(".{}", &int_digits[d..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — signed, but not zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)` for the ASCII fragments [`LangMl::cardinal_from_str`]
/// hands it. Ports the underscore rule (`int("1_0") == 10`, `int("1_")`
/// raises) and the error message (`repr(s)` of the original argument).
fn py_int(s: &str) -> Result<BigInt> {
    let err = || {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    };
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body.chars().all(|c| c.is_ascii_digit() || c == '_')
    {
        return Err(err());
    }
    let digits: String = body.chars().filter(|c| *c != '_').collect();
    let n: BigInt = digits.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

pub struct LangMl {
    /// `CURRENCY_FORMS`, built once. The registry holds `LangMl` in a
    /// `OnceLock`, so this table is constructed exactly once per process;
    /// rebuilding it per call is what made an earlier port 10x slower than
    /// the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangMl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangMl {
    pub fn new() -> Self {
        // Python's arity, kept exactly: two unit forms and two subunit forms
        // per code. `to_currency` indexes [0] for the singular and [1] for the
        // plural, so both slots must exist even where INR repeats itself.
        let mut currency_forms = HashMap::new();
        currency_forms.insert("INR", CurrencyForms::new(&["രൂപ", "രൂപ"], &["പൈസ", "പൈസ"]));
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["ഡോളർ", "ഡോളർ"], &["സെന്റ്", "സെന്റ്"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["യൂറോ", "യൂറോ"], &["സെന്റ്", "സെന്റ്"]),
        );
        LangMl { currency_forms }
    }

    /// `int_to_word` behind [`maxval_ceiling`], checked before the first
    /// recursive step. Every entry point that hands over a caller-supplied
    /// integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    /// The multiplier in front of ലക്ഷം/കോടി: [`ONE_ATTRIBUTIVE`] for one,
    /// the plain cardinal otherwise.
    fn multiplier(&self, n: &BigInt) -> String {
        if n.is_one() {
            ONE_ATTRIBUTIVE.to_string()
        } else {
            self.int_to_word(n)
        }
    }

    /// Python's `_int_to_word`.
    ///
    /// Scale boundaries, in Python's exact order: 10, 20, 100, 1000, 100000
    /// (lakh), 10000000 (crore), 1000000000. Past the last one the digits are
    /// returned verbatim (bug 4), which is why the signature returns `String`
    /// rather than `Result` — this function cannot fail.
    ///
    /// Every division here runs on a non-negative value (the `is_negative`
    /// arm recurses on `abs` first), so Python's floor `//` and `%` agree with
    /// `div_floor`/`mod_floor` — no sign skew to reproduce.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // Unreachable from to_cardinal/to_ordinal/to_year — kept for parity.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let lakh = BigInt::from(100_000);
        let crore = BigInt::from(10_000_000);

        if number < &hundred {
            return BELOW_HUNDRED[number.to_usize().unwrap()].to_string();
        }

        if number < &thousand {
            // Note: Python indexes `self.ones` directly here rather than
            // recursing, so the multiplier is always a bare 1..=9 word.
            let (div, rem) = number.div_mod_floor(&hundred);
            let mut result = format!("{} {}", BELOW_HUNDRED[div.to_usize().unwrap()], HUNDRED);
            if !rem.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&rem));
            }
            return result;
        }

        if number < &lakh {
            let (div, rem) = number.div_mod_floor(&thousand);
            let mut result = format!("{} {}", self.int_to_word(&div), THOUSAND);
            if !rem.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&rem));
            }
            return result;
        }

        if number < &crore {
            let (div, rem) = number.div_mod_floor(&lakh);
            let mut result = format!("{} {}", self.multiplier(&div), LAKH);
            if !rem.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&rem));
            }
            return result;
        }

        // Python stopped at 10^9 and returned `str(number)` (bug 4, fixed):
        // the crore count is spelled out up to the 10^14 ceiling.
        {
            let (div, rem) = number.div_mod_floor(&crore);
            let mut result = format!("{} {}", self.multiplier(&div), CRORE);
            if !rem.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&rem));
            }
            result
        }
    }

    /// Python's
    ///
    /// ```text
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// evaluated *after* `val = abs(val)`, hence the `abs` on the digits.
    ///
    /// `right` is a plain two-digit slice of the fractional text: no rounding,
    /// no `CURRENCY_PRECISION`, no `parse_currency_parts`. `0.5` therefore
    /// yields 50 (`"5".ljust(2, "0") == "50"`) while `0.001` yields 0
    /// (`"00"`), and any third decimal is discarded outright.
    ///
    /// Reconstructing `str(val)` from a `BigDecimal` is exact for every
    /// non-scientific `repr`, because `BigDecimal::from_str` preserves the
    /// source scale (`"1.0"` keeps scale 1) and the shim hands us `str(value)`
    /// verbatim. The scientific cases are split out into [`Self::scientific_parts`].
    fn split_parts(&self, val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
        match val {
            // `str(int)` never contains a ".", so `parts` has a single element
            // and `right` stays 0 — a true int can never render paisa. Note the
            // float `1.0` reaches the Decimal arm and *also* ends up with
            // right == 0, via `"0".ljust(2, "0")` (bug 9): unlike the base
            // class, ML renders both as "ഒന്ന് രൂപ".
            CurrencyValue::Int(v) => Ok((v.abs(), BigInt::zero())),
            CurrencyValue::Decimal { value: d, .. } => {
                let (digits, scale) = d.as_bigint_and_exponent();
                // abs(val) first, then str() — mirrors Python's statement order.
                let m = digits.abs().to_string();
                if scale < 0 {
                    Self::scientific_parts(&m, scale)
                } else {
                    Ok(Self::plain_parts(&m, scale as usize))
                }
            }
        }
    }

    /// `str(val)` in plain notation: `scale` fractional digits and at least one
    /// integer digit.
    ///
    /// `m` comes from `BigInt::to_string()` and the padding is ASCII `'0'`, so
    /// every byte index below is also a char index — no UTF-8 hazard, unlike
    /// the Malayalam tables above.
    fn plain_parts(m: &str, scale: usize) -> (BigInt, BigInt) {
        if scale == 0 {
            // No "." in str(val) — e.g. Decimal("100"), whose repr has no
            // fractional part at all. parts == [m], so right stays 0.
            return (BigInt::from_str(m).unwrap(), BigInt::zero());
        }
        // Python renders "0.01", never ".01", so ensure an integer digit
        // exists before slicing the fraction off the tail.
        let padded = if m.len() < scale + 1 {
            format!("{}{}", "0".repeat(scale + 1 - m.len()), m)
        } else {
            m.to_string()
        };
        let (int_part, frac_part) = padded.split_at(padded.len() - scale);
        // right = int(parts[1][:2].ljust(2, "0"))
        let mut frac2: String = frac_part.chars().take(2).collect();
        while frac2.len() < 2 {
            frac2.push('0');
        }
        // Both unwraps are infallible: `int_part` is a non-empty run of ASCII
        // digits (the padding guarantees at least one) and `frac2` is exactly
        // two ASCII digits.
        (
            BigInt::from_str(int_part).unwrap(),
            BigInt::from_str(&frac2).unwrap(),
        )
    }

    /// `str(val)` in scientific notation — see bug 10.
    ///
    /// A negative `scale` is the tell. `BigDecimal::from_str` only yields one
    /// when the source string carried an exponent, and the source string is the
    /// shim's `str(value)`, so negative scale means Python's `str(val)` was
    /// scientific too. That happens for floats with `abs(val) >= 1e16` and for
    /// an explicit `Decimal("1E+2")`.
    ///
    /// Both `repr(float)` and `Decimal.__str__` put exactly one digit before
    /// the ".", so the text is `m[0] ("." m[1:])? "e+" exp`. Running that
    /// through the split/int logic collapses to a test on `k = m.len()`:
    ///
    /// * `k == 1` — no ".", so `int("1e+16")` raises ValueError.
    /// * `k == 2` — `parts[1][:2]` is `m[1] + "e"`, so `int("5e")` raises.
    /// * `k >= 3` — `parts[1][:2]` is `m[1..3]`, two real digits, so Python
    ///   returns a nonsense answer instead of raising: `1.23e16` gives
    ///   left = 1, right = 23.
    ///
    /// Verified against CPython for 1e16, 1.5e16, 1.23e16, 1.234e16 and
    /// `Decimal("1.50E+16")` (which gives right = 50 — trailing mantissa zeros
    /// survive, and `from_str` preserves them, so the rule still holds).
    ///
    /// The mirror-image case is *not* detectable: `str(1e-05)` is `'1e-05'`
    /// and raises in Python, but it parses to scale +5 — identical to
    /// `Decimal("0.00001")`, which does not raise. See the port's `concerns`.
    fn scientific_parts(m: &str, scale: i64) -> Result<(BigInt, BigInt)> {
        let k = m.len();
        // exp = -scale + (k - 1), reconstructed only for the error text.
        let exp = -scale + (k as i64 - 1);
        if k == 1 {
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}e+{}'",
                m, exp
            )));
        }
        if k == 2 {
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}e'",
                &m[1..2]
            )));
        }
        // ASCII digits throughout — byte slicing is char slicing here.
        Ok((
            BigInt::from_str(&m[0..1]).unwrap(),
            BigInt::from_str(&m[1..3]).unwrap(),
        ))
    }

    /// The body of `Num2Word_ML.to_cardinal` for a stringified number `n`,
    /// minus the outer `.strip()` (applied by the caller). The sign is stripped
    /// *textually*, `split(".", 1)` caps at one split (a second dot detonates in
    /// the digit loop), and `int(left)` runs before the digit generator so a
    /// malformed left half is the first thing to raise.
    ///
    /// A method rather than a free function (unlike the `lang_gu.rs` twin) only
    /// because ML's `_int_to_word` is a method — it takes `&self` even though
    /// it never touches a field.
    fn cardinal_from_str(&self, number: &str) -> Result<String> {
        let n = number.trim();
        let (n, mut ret) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD.to_string()),
            None => (n, String::new()),
        };

        let Some(dot) = n.find('.') else {
            ret.push_str(&self.checked_int_to_word(&py_int(n)?)?);
            return Ok(ret);
        };

        // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
        let (left, right) = (&n[..dot], &n[dot + 1..]);
        ret.push_str(&self.checked_int_to_word(&py_int(left)?)?);
        ret.push(' ');
        // Python appends `self.pointword` verbatim — no `title()`, unlike the
        // base `to_cardinal_float`.
        ret.push_str(self.pointword());
        ret.push(' ');

        // " ".join(self._int_to_word(int(d)) for d in right) — iterate *chars*.
        let mut first = true;
        for d in right.chars() {
            if !first {
                ret.push(' ');
            }
            first = false;
            let mut buf = [0u8; 4];
            ret.push_str(&self.int_to_word(&py_int(d.encode_utf8(&mut buf))?));
        }
        Ok(ret)
    }
}

/// The exclusive ceiling (gladiaio/num2words2#147). The scale stops at
/// കോടി (10^7); above it the crore count is spelled out (the everyday
/// "hundred crore", "lakh crore") until that count
/// would itself need കോടി at 10^14. Python returned the digits from
/// 10^9 up; this raises `OverflowError` from 10^14 instead.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(14))
}

impl Lang for LangMl {
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
        "പോയിന്റ്"
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)`: it peels a leading "-" off the
    /// *text*, then `int()`s the remainder. For integer input the "." branch
    /// is unreachable, so this reduces to sign-prefix + `_int_to_word(abs)`.
    /// The trailing `.strip()` is kept for fidelity but is a no-op here —
    /// `NEGWORD`'s trailing space is always followed by a word.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.checked_int_to_word(&n)?).trim().to_string())
    }

    /// Python's `to_ordinal`: five hardcoded special forms, then the generic
    /// `cardinal + "ാം"` fallback. See bugs 1 and 2.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(v) = value.to_i32() {
            match v {
                1 => return Ok("ഒന്നാം".to_string()),
                2 => return Ok("രണ്ടാം".to_string()),
                3 => return Ok("മൂന്നാം".to_string()),
                4 => return Ok("നാലാം".to_string()),
                5 => return Ok("അഞ്ചാം".to_string()),
                _ => {}
            }
        }
        Ok(format!("{}{}", self.to_cardinal(value)?, ORD_SUFFIX))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`. See bug 6.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)`. `longval` is accepted and then
    /// ignored by the original, so it has no counterpart here. See bug 5.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("BC {}", self.to_cardinal(&value.abs())?))
        } else {
            // Python: `return "" + self.to_cardinal(val)`.
            self.to_cardinal(value)
        }
    }

    /// `to_ordinal(float/Decimal)`: the `number == 1 ... == 5` chain matches
    /// numerically, so whole 1..=5 (1.0, Decimal("5.00")) take the special
    /// forms; everything else is `to_cardinal(number) + "ാം"` off the value's
    /// own str() grammar ("ഏഴ് പോയിന്റ് പൂജ്യംാം" for 7.0).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i.is_positive() && i <= BigInt::from(5) {
                return self.to_ordinal(&i);
            }
        }
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORD_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — Python's str,
    /// handed in as `repr_str` ("5.0.", "-0.0.", "1E+2.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `if val < 0` (numeric) → `"BC " +
    /// to_cardinal(abs(val))`, keeping the float grammar.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if negative {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: value.abs(),
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: value.abs(),
                    precision: *precision,
                },
            };
            Ok(format!("BC {}", self.cardinal_float_entry(&abs, None)?))
        } else {
            self.cardinal_float_entry(value, None)
        }
    }

    /// `str_to_number` stays Base's `Decimal(value)`, but ML's `to_cardinal`
    /// then runs `int()` over the dot-free `str()` form, so `"Infinity"`
    /// raises **ValueError** (`int('Infinity')`) rather than the shared Inf
    /// sentinel's OverflowError.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Python's `Num2Word_ML.to_cardinal` reached with a `float`/`Decimal`.
    ///
    /// ML overrides `to_cardinal` (not `to_cardinal_float`) and handles the
    /// non-integer inline off `str(number)` — see the module comment above the
    /// helpers. `precision_override` (the `precision=` kwarg) is threaded by the
    /// base but ML's `to_cardinal` never reads `self.precision`, so it is
    /// ignored, exactly as Python ignores it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            // Python's str(float); the raw f64 crossed the boundary so repr()
            // can be reproduced from the bits — no f64-artefact repair.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        Ok(self.cardinal_from_str(&n)?.trim().to_string())
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError.
    fn lang_name(&self) -> &str {
        "Num2Word_ML"
    }

    /// `CURRENCY_FORMS[code]` — the *strict* subscript, which is what the
    /// inherited `to_cheque` performs. Returning `None` for an unknown code is
    /// what turns `cheque:GBP` into NotImplementedError. `to_currency` does not
    /// route through here; it applies Python's INR fallback itself (bug 7).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // `currency_precision`, `currency_adjective`, `pluralize`,
    // `money_verbose`, `cents_verbose`, `cents_terse` and `to_cheque` are all
    // left at their trait defaults: ML declares no CURRENCY_PRECISION and no
    // CURRENCY_ADJECTIVES, overrides none of the verbose/pluralize hooks, and
    // inherits `Num2Word_Base.to_cheque` verbatim. The default `money_verbose`
    // delegates to `to_cardinal`, which is overridden above — so
    // `default_to_cheque` picks up the Malayalam words for free.

    /// Python's `Num2Word_ML.to_currency`.
    ///
    /// A wholesale override that shares nothing with the base implementation:
    /// no `parse_currency_parts`, no `pluralize`, no rounding and no
    /// `CURRENCY_PRECISION`. It stringifies the value, slices two characters of
    /// fractional text, and picks singular/plural by comparing against 1.
    ///
    /// The trailing `.strip()` is kept for fidelity but is provably a no-op:
    /// `_int_to_word` never returns an empty string (0 yields "പൂജ്യം"), so the
    /// result always starts with a Malayalam letter, and `negword`'s trailing
    /// space is always followed by a word.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Python accepts `adjective` and then never reads it — ML has no
        // CURRENCY_ADJECTIVES, so `to_currency(12.34, "EUR", adjective=True)`
        // is byte-identical to the plain call.
        _adjective: bool,
    ) -> Result<String> {
        // Trait hands None when the caller omitted separator=;
        // resolve to this language's own default.
        let separator = separator.unwrap_or(self.default_separator());
        // `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])` — an
        // unknown code silently becomes rupees rather than raising (bug 7).
        // `new()` always installs INR, so the fallback cannot miss.
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get("INR"))
            .expect("CURRENCY_FORMS[\"INR\"] is installed by LangMl::new");

        // Python takes `is_negative` from the original value, before `abs()`.
        let is_negative = val.is_negative();
        let (left, right) = self.split_parts(val)?;

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            if left.is_one() {
                &forms.unit[0]
            } else {
                &forms.unit[1]
            }
        );

        // `if cents and right:` — a falsy (zero) `right` drops the segment, and
        // so does `cents=False`, with no terse fallback (bug 9).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right.is_one() {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            });
        }

        if is_negative {
            // negword carries its own trailing space: "മൈനസ് ".
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
