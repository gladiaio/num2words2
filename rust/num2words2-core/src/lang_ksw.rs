//! S'gaw Karen (`ksw`), in the S'gaw Karen script (Myanmar block).
//!
//! `ksw` is one of the languages added by this project (upstream `num2words`
//! has no `lang_KSW.py`), so there is no upstream output to stay byte-for-byte
//! with. Python's `lang_KSW.py` spelled every numeral in an ad-hoc Latin
//! transliteration ("lwisi di khi" for 42) and fell back to digits from 10^9.
//! gladiaio/num2words2#143 replaced it with the script spellings that two
//! independent sources agree on; #262 fills in the rest on best evidence
//! (see the UNVERIFIED notes below), never mixing scripts.
//!
//! # Lexicon (sources)
//!
//! Omniglot, *Numbers in Sgaw Karen*
//! (<https://www.omniglot.com/language/numbers/sgawkaren.htm>) and the English
//! Wiktionary category *S'gaw Karen numerals*
//! (<https://en.wiktionary.org/wiki/Category:S%27gaw_Karen_numerals>):
//!
//! | n | word | | n | word |
//! |---|---|---|---|---|
//! | 1 | တ (final: တၢ) | | 10 | တဆံ |
//! | 2 | ခံ | | 11 | တဆံတၢ |
//! | 3 | သၢ | | 12 | တဆံခံ |
//! | 4 | လွံၢ် | | 20 | ခံဆံ |
//! | 5 | ယဲၢ် | | 40 | လွံၢ်ဆံ |
//! | 6 | ဃု | | 100 | တကယၤ |
//! | 7 | နွံ | | 1000 | တကထိ |
//! | 8 | ဃိး | | 10^4 | တကလး |
//! | 9 | ခွံ | | 10^5 | တကလီၢ် |
//!
//! Composition, as in Gilmore, *A Grammar of the Sgaw Karen* (1898) §88:
//! place values high to low, each `digit + place word` (the multiplier "one"
//! is တ: တကယၤ, တကထိ; 200 is ခံကယၤ), tens and units together
//! (`digit + ဆံ + unit`). A unit "one" after anything else takes the form
//! တၢ, as in 11 တဆံတၢ. The places are written as one word, without spaces
//! (#263), as the S'gaw Karen Common Bible (KSWC, 1992) writes every count:
//! Genesis 5:6 တကယၤယဲၢ် (105), 5:3 တကယၤသၢဆံ (130), 5:18 တကယၤဃုဆံခံ
//! (162), 50:26 တကယၤတဆံ (110), and the counts below. So 101 is တကယၤတၢ.
//! Omniglot writes 50 with ဟ although its 5 is ယဲၢ် and its transliteration
//! of 50 is "ye hsee";
//! this module uses the regular ယဲၢ်ဆံ, as Gilmore's 52 does.
//!
//! # Ten thousand and a hundred thousand (#262)
//!
//! ကလး (10^4) and ကလီၢ် (10^5) are on Wiktionary (*ကလး*, *ကလီၢ်*; ကလး is
//! filed as a classifier, as is the ကထိ this module already used for 1000),
//! and the S'gaw Karen Common Bible (KSWC, bible.com) uses both as place
//! words, with values the English text pins down: Psalm 91:7 pairs တကထိ
//! (a thousand) with တကလး (ten thousand); Numbers 1:46 counts 603,550 as
//! ဃုကလီၢ်သၢကထိယဲၢ်ကယၤယဲၢ်ဆံ and Numbers 26:51 counts 601,730 as
//! ဃုကလီၢ်တကထိနွံကယၤသၢဆံ. They compose like the lower places
//! (`digit + place word`, an empty place skipped, multiplier one တ).
//!
//! # A million and up (#262)
//!
//! ကကွဲၢ် is the KSWC's million, by majority of the verses that need one:
//! 1 Chronicles 22:14 has တကကွဲၢ် for "a thousand thousand" talents, and
//! Revelation 9:16 counts 200,000,000 as ကကွဲၢ်ခံကယၤ (also Daniel 7:10 and
//! Revelation 5:11 for "thousands of thousands"). The one dissent is
//! 2 Chronicles 14:9 (ကလီၢ်တကယၤ, "a hundred hundred-thousands"); 1
//! Chronicles 21:5 likewise counts 1,100,000 as ဆံတကလီၢ်. 1-9 million are
//! `digit + ကကွဲၢ်` like the lower places (တကကွဲၢ်).
//!
//! UNVERIFIED (#262): a count of ten million or more — best candidate:
//!   Revelation 9:16's noun-first ကကွဲၢ်ခံကယၤ, so 10^7 is ကကွဲၢ်တဆံ. The
//!   count is written as one word and a space separates it from the rest
//!   (otherwise 205,000,000 and 200,000,005 would read alike); that space
//!   is this module's choice. `maxval` is 10^12.
//!
//! # Zero, minus, decimals (#262)
//!
//! No Karen source reads these; the best candidates are the Burmese/Pali
//! words S'gaw Karen borrows for technical vocabulary, written as in
//! Burmese:
//!
//! UNVERIFIED (#262): "သုည" (zero) — best candidate: the Burmese/Pali zero
//!   (2 of 5 models; one other offered an English loan).
//! UNVERIFIED (#262): "အနုတ်" (minus, before the number) — best candidate:
//!   the Burmese minus (1 of 5 models; the rest unsure).
//! UNVERIFIED (#262): "ဒသမ" (decimal point) — best candidate: the Burmese
//!   decimal point (1 of 5 models; the rest unsure). The digits after it are
//!   read one by one, space-separated: 1.05 is "တ ဒသမ သုည ယဲၢ်".
//!
//! # Ordinals (#262)
//!
//! The KSWC forms ordinals with a classifier frame, `cardinal + CL + တ + CL`
//! ("N-CL one-CL"): Genesis 1:8 မုၢ်ခံနံၤတနံၤ (the second day), Genesis
//! 2:13-14 ခံဘိတဘိ / သၢဘိတဘိ / လွံၢ်ဘိတဘိ (second..fourth river),
//! Revelation 4:7 ခံဒုတဒု (second beast), Revelation 21:20 ယဲၢ်ဖျၢၣ်တဖျၢၣ်
//! to တဆံခံဖျၢၣ်တဖျၢၣ် (fifth..twelfth stone; eleventh is
//! တဆံတၢဖျၢၣ်တဖျၢၣ်, with the final-form unit). "First" is suppletive:
//! အခီၣ်ထံး + တ + CL (Genesis 2:11 အခီၣ်ထံးတဘိ; Genesis 1:5
//! အခီၣ်ထံးကတၢၢ်တနံၤ; Revelation 4:7 has အဆိကတၢၢ်တဒု).
//!
//! UNVERIFIED (#262): "ခါ" as the classifier of a bare ordinal — best
//!   candidate: Wiktionary's "generic classifier; classifier for abstract
//!   nouns". So 2nd is ခံခါတခါ, 1st အခီၣ်ထံးတခါ, 11th တဆံတၢခါတခါ, and
//!   `to='ordinal_num'` puts the digits in the same frame (2ခါတခါ). Negative
//!   ordinals raise `TypeError`, as in the base class.
//!
//! # Currency (#262)
//!
//! UNVERIFIED (#262): "ကၠး" (kyat) and "ပၠး" (pya) for MMK, the default —
//!   best candidate: Burmese ကျပ်/ပြား adapted the way Wiktionary's S'gaw
//!   Karen loans from Burmese are (ကျောင်း → ကၠိ, ပြ → ပၠး, စက် → စဲး,
//!   ချောကလက် → ခၠီကလဲး: medial ျ/ြ → ၠ, a stop final → း); ကၠး is also
//!   one model's answer. The unadapted ကျပ် was the other candidate.
//!
//! Other currencies raise `NotImplementedError`: the only candidates for
//! dollar, euro, baht and cent are single-model answers that disagree
//! (and one cent candidate, စဲး, is the word for "machine"). `to='cheque'`
//! raises too: the shared cheque format writes Latin "AND" and "MINUS".
//!
//! A scientific `str(number)` ("1e+16") still raises `ValueError` from
//! `int()`, as before.

use crate::base::{
    check_maxval, pow10_big, unsupported_mode, verify_ordinal, verify_ordinal_float, Lang,
    N2WError, Result,
};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use std::sync::OnceLock;

/// Units 1..=9 (index 0 unused: zero is [`ZERO`]).
const ONES: [&str; 10] = [
    "", "တ", "ခံ", "သၢ", "လွံၢ်", "ယဲၢ်", "ဃု", "နွံ", "ဃိး", "ခွံ",
];
/// "One" as a unit after another place, e.g. 11 တဆံတၢ.
const ONE_FINAL: &str = "တၢ";
/// The tens place word: `digit + ဆံ` (20 ခံဆံ; 10 တဆံ).
const TEN: &str = "ဆံ";
/// The hundreds place word: `digit + ကယၤ` (100 တကယၤ).
const HUNDRED: &str = "ကယၤ";
/// The thousands place word: `digit + ကထိ` (1000 တကထိ).
const THOUSAND: &str = "ကထိ";
/// The ten-thousands place word: `digit + ကလး` (10^4 တကလး; #262).
const TEN_THOUSAND: &str = "ကလး";
/// The hundred-thousands place word: `digit + ကလီၢ်` (10^5 တကလီၢ်; #262).
const HUNDRED_THOUSAND: &str = "ကလီၢ်";
/// The million: `digit + ကကွဲၢ်` below ten million, noun-first above (#262).
const MILLION: &str = "ကကွဲၢ်";
/// Zero, minus and the decimal point: best candidates (#262, module docs).
const ZERO: &str = "သုည";
const NEGWORD: &str = "အနုတ် ";
const POINTWORD: &str = "ဒသမ";
/// Ordinal frame `cardinal + ခါ + တ + ခါ`; "first" is suppletive (#262).
const ORDINAL_SUFFIX: &str = "ခါတခါ";
const FIRST: &str = "အခီၣ်ထံးတခါ";

/// The exclusive ceiling: a count of millions has words up to 999999
/// (gladiaio/num2words2#143, #147, #262).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

#[derive(Default)]
pub struct LangKsw {
    currency_forms: OnceLock<CurrencyForms>,
}

impl LangKsw {
    pub fn new() -> Self {
        LangKsw::default()
    }
}

/// Words for `1 <= n <= 999999` (see the module docs for the composition).
fn below_million(n: u32) -> String {
    debug_assert!((1..=999_999).contains(&n));
    let digit = |d: u32| ONES[d as usize];
    let (ht, tt) = (n / 100_000, n / 10_000 % 10);
    let (th, h, t, o) = (n / 1000 % 10, n / 100 % 10, n / 10 % 10, n % 10);
    let mut parts: Vec<String> = Vec::new();
    if ht != 0 {
        parts.push(format!("{}{}", digit(ht), HUNDRED_THOUSAND));
    }
    if tt != 0 {
        parts.push(format!("{}{}", digit(tt), TEN_THOUSAND));
    }
    if th != 0 {
        parts.push(format!("{}{}", digit(th), THOUSAND));
    }
    if h != 0 {
        parts.push(format!("{}{}", digit(h), HUNDRED));
    }
    // A unit "one" after any other place takes its final form.
    let unit = |first: bool| if o == 1 && !first { ONE_FINAL } else { digit(o) };
    if t != 0 {
        let u = if o != 0 { unit(false) } else { "" };
        parts.push(format!("{}{}{}", digit(t), TEN, u));
    } else if o != 0 {
        parts.push(unit(parts.is_empty()).to_string());
    }
    parts.concat()
}

/// Words for `0 <= n < 10^12`: 1-9 million as `digit + ကကွဲၢ်` joined to the
/// rest; ten million and up as `ကကွဲၢ် + count`, then a space (#262).
fn words(n: u64) -> String {
    if n == 0 {
        return ZERO.to_string();
    }
    let (m, r) = ((n / 1_000_000) as u32, (n % 1_000_000) as u32);
    if m == 0 {
        return below_million(r);
    }
    // The remainder never starts the number, so a unit one is final-form.
    let rest = match r {
        0 => String::new(),
        1 => ONE_FINAL.to_string(),
        _ => below_million(r),
    };
    if m < 10 {
        format!("{}{}{}", ONES[m as usize], MILLION, rest)
    } else if rest.is_empty() {
        format!("{}{}", MILLION, below_million(m))
    } else {
        format!("{}{} {}", MILLION, below_million(m), rest)
    }
}

/// The integer cardinal: zero, negatives with the minus word before them,
/// and `OverflowError` from 10^12.
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    let w = words(number.abs().to_u64().expect("|n| < 10^12 after check_maxval"));
    Ok(if number.is_negative() { format!("{}{}", NEGWORD, w) } else { w })
}

/// Reconstruct Python's `str(f)` (== `repr(f)`) for a finite/`inf`/`nan` f64.
///
/// `precision` is the repr's fractional-digit count, supplied by the binding as
/// `abs(Decimal(str(f)).as_tuple().exponent)`. In the plain-decimal window
/// (`repr` exponent in `-4..=15`) formatting the abs value to that many places
/// reproduces the repr exactly. Outside it, `repr` is scientific and this
/// mirrors CPython's `"1e+16"` / `"1e-05"` shape (two-digit zero-padded
/// exponent) — a form the string algorithm then rejects via `int()`.
fn float_repr(value: f64, precision: u32) -> String {
    let neg = value.is_sign_negative();
    let abs = value.abs();
    let body = if abs.is_finite() && abs != 0.0 {
        // "1.2345e3" / "1e-5" — LowerExp is shortest round-trip, like repr.
        let es = format!("{:e}", abs);
        let epos = es.find('e').expect("LowerExp of a finite nonzero has 'e'");
        let exp: i32 = es[epos + 1..]
            .parse()
            .expect("LowerExp exponent is an integer");
        if exp >= 16 || exp <= -5 {
            let mantissa = &es[..epos];
            let sign = if exp < 0 { "-" } else { "+" };
            format!("{}e{}{:02}", mantissa, sign, exp.abs())
        } else {
            format!("{:.*}", precision as usize, abs)
        }
    } else {
        // 0.0 -> "0.0" (precision is 1); inf/nan -> "inf"/"NaN", both of which
        // have no "." and fail `int()` exactly as Python's do.
        format!("{:.*}", precision as usize, abs)
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Reconstruct Python's `Decimal.__str__` from a `BigDecimal`.
///
/// A faithful port of CPython's `Decimal.__str__` (the `not eng` arm): recovers
/// the `(coefficient, exponent)` pair and places the point per `dotplace`,
/// emitting an `E±d` suffix in the same band Python does. This is what makes a
/// trailing-zero `Decimal("1.10")` render "1.10" (scale is preserved) and a
/// large-scale `Decimal("1E+2")` render "1E+2" (and then raise, no ".").
fn decimal_repr(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    let exp = -scale; // Python `_exp`
    let sign = if int_val.is_negative() { "-" } else { "" };
    let digits = int_val.magnitude().to_string(); // Python `_int`
    let ndigits = digits.len() as i64;
    let leftdigits = exp + ndigits;

    // `if self._exp <= 0 and leftdigits > -6: dotplace = leftdigits` else 1.
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), digits),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", digits, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let dp = dotplace as usize;
        (digits[..dp].to_string(), format!(".{}", &digits[dp..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// `int(s)` for the integer part. Python raises `ValueError` on a non-numeric
/// token (e.g. a scientific mantissa `"1e+16"`), which maps to `N2WError::Value`
/// with Python's exact message text.
fn parse_pyint(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

impl LangKsw {
    /// The cardinal of a float/Decimal, from Python's `str(number)`: the sign
    /// becomes the minus word, the integer part reads as a cardinal and each
    /// digit after the point on its own; an integral `Decimal("5")` reads as
    /// the integer, and a scientific token raises `ValueError` from `int()`.
    fn cardinal_from_str(&self, s: &str) -> Result<String> {
        let s = s.trim();
        let (neg, s) = match s.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", s),
        };
        let Some((left, right)) = s.split_once('.') else {
            return Ok(format!("{}{}", neg, int_to_word(&parse_pyint(s)?)?));
        };
        let mut out = format!("{}{} {}", neg, int_to_word(&parse_pyint(left)?)?, POINTWORD);
        for ch in right.chars() {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
            })?;
            out.push(' ');
            out.push_str(if d == 0 { ZERO } else { ONES[d as usize] });
        }
        Ok(out)
    }
}

impl Lang for LangKsw {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// Every float/Decimal goes through `str(number)` (see `cardinal_from_str`).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_ordinal(&verify_ordinal_float(value)?)
    }

    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        self.to_ordinal_num(&verify_ordinal_float(value)?)
    }

    /// `Decimal("Infinity")` parses, then fails in `int()` as `ValueError`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
    }

    fn default_currency(&self) -> &str {
        "MMK"
    }

    /// No conjunction between kyat and pya: base's `" "` after the separator
    /// already spaces them.
    fn default_separator(&self) -> &str {
        ""
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        int_to_word(value)
    }

    /// `cardinal + ခါတခါ`, "first" အခီၣ်ထံးတခါ (see the module docs).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        if value == &BigInt::from(1) {
            return Ok(FIRST.to_string());
        }
        Ok(format!("{}{}", int_to_word(value)?, ORDINAL_SUFFIX))
    }

    /// The digits in the ordinal frame: 2ခါတခါ.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// The plain cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let s = match value {
            FloatValue::Float { value, precision } => float_repr(*value, *precision),
            FloatValue::Decimal { value, .. } => decimal_repr(value),
        };
        self.cardinal_from_str(&s)
    }

    fn lang_name(&self) -> &str {
        "Num2Word_KSW"
    }

    /// MMK only (see the module docs); other codes raise.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        (code == "MMK")
            .then(|| self.currency_forms.get_or_init(|| CurrencyForms::new(&["ကၠး"], &["ပၠး"])))
    }

    /// Karen nouns do not inflect for number.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        Ok(forms[0].clone())
    }

    /// The shared cheque format writes Latin "AND"/"MINUS".
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(unsupported_mode("cheque"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omniglot_forms() {
        let k = LangKsw::new();
        let c = |n: i64| k.to_cardinal(&BigInt::from(n)).unwrap();
        assert_eq!(c(1), "တ");
        assert_eq!(c(10), "တဆံ");
        assert_eq!(c(11), "တဆံတၢ");
        assert_eq!(c(40), "လွံၢ်ဆံ");
        assert_eq!(c(100), "တကယၤ");
        assert_eq!(c(200), "ခံကယၤ");
        assert_eq!(c(1000), "တကထိ");
        assert_eq!(c(9999), "ခွံကထိခွံကယၤခွံဆံခွံ");
        // #262: KSWC Psalm 91:7, Numbers 1:46 and 26:51.
        assert_eq!(c(10_000), "တကလး");
        assert_eq!(c(100_000), "တကလီၢ်");
        assert_eq!(c(603_550), "ဃုကလီၢ်သၢကထိယဲၢ်ကယၤယဲၢ်ဆံ");
        assert_eq!(c(601_730), "ဃုကလီၢ်တကထိနွံကယၤသၢဆံ");
        assert_eq!(c(999_999), "ခွံကလီၢ်ခွံကလးခွံကထိခွံကယၤခွံဆံခွံ");
        // #262: KSWC 1 Chronicles 22:14 (တကကွဲၢ်), Revelation 9:16 (2 * 10^8).
        assert_eq!(c(1_000_000), "တကကွဲၢ်");
        assert_eq!(c(1_000_001), "တကကွဲၢ်တၢ");
        assert_eq!(c(200_000_000), "ကကွဲၢ်ခံကယၤ");
        assert_eq!(c(205_000_000), "ကကွဲၢ်ခံကယၤယဲၢ်");
        assert_eq!(c(200_000_005), "ကကွဲၢ်ခံကယၤ ယဲၢ်");
        assert!(matches!(
            k.to_cardinal(&BigInt::from(1_000_000_000_000i64)),
            Err(N2WError::Overflow(_))
        ));
        assert_eq!(c(0), "သုည");
        assert_eq!(c(-3), "အနုတ် သၢ");
    }

    #[test]
    fn ordinals() {
        let k = LangKsw::new();
        let o = |n: i64| k.to_ordinal(&BigInt::from(n)).unwrap();
        assert_eq!(o(1), "အခီၣ်ထံးတခါ");
        assert_eq!(o(2), "ခံခါတခါ");
        assert_eq!(o(11), "တဆံတၢခါတခါ");
        assert!(matches!(k.to_ordinal(&BigInt::from(-1)), Err(N2WError::Type(_))));
    }
}
