//! Port of `lang_UR.py` (Urdu).
//!
//! Shape: **self-contained**. `Num2Word_UR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `__init__` skips the card-building block entirely: `self.cards` is never
//! created and `self.MAXVAL` is never set (verified — `hasattr(c, "cards")`
//! and `hasattr(c, "MAXVAL")` are both `False` on a live instance).
//! `to_cardinal` is overridden outright and drives `_int_to_word` over the
//! South-Asian lakh/crore grouping. So `cards`/`merge` stay at their trait
//! defaults here. Python has no overflow check at all; the port's `maxval` is
//! 10^34, where the multiplier of سنکھ (10^17) would itself need سنکھ (bug 2).
//!
//! Inherited from `Num2Word_Base`: nothing that matters. UR overrides all
//! four in-scope modes (`to_cardinal`, `to_ordinal`, `to_ordinal_num`,
//! `to_year`) itself.
//!
//! No cross-call mutable state: every method is a pure function of its
//! argument. `setup()` only assigns constant tables, and `self.precision`
//! (left at base's `2`) is never read by any UR method — `to_cardinal` gets
//! its digit count from `str(number)`, so `num2words(..., precision=N)` sets
//! the attribute and then changes nothing (verified live).
//!
//! # Float/Decimal path
//!
//! `Num2Word_UR.to_cardinal` handles non-integers **inline** on
//! `str(number)`: it strips a leading `-` textually, splits once on `"."`,
//! renders the integer part through `_int_to_word(int(left))` and then each
//! fractional *character* through `_int_to_word(int(d))`, joined by spaces.
//! `Num2Word_Base.to_cardinal_float` and `float2tuple` are **never reached**
//! for Urdu, which inverts the usual advice for this phase:
//!
//! * The `< 0.01` f64-artefact heuristic is absent and *not* missing:
//!   `repr(2.675)` is `"2.675"`, so the `674.9999999999998` it exists to
//!   repair is never computed. The spec here is `repr` itself, reproduced by
//!   [`py_float_repr`] (including CPython's dtoa round-half-to-even tie).
//! * `precision` — both the kwarg and `FloatValue::precision` — is ignored,
//!   exactly as Python ignores it (the attribute is set, never read).
//! * The Decimal arm stringifies via [`py_decimal_str`] and never touches
//!   f64, so `Decimal("98746251323029.99")` keeps full precision (the
//!   issue-#603 float cast cannot happen) and `Decimal("1.10")` keeps its
//!   trailing zero: "ایک اعشاریہ ایک صفر".
//! * An integer part at or past 10^9 used to hit bug 2's `str(number)`
//!   fallback inside the float rendering; it now takes the full scale:
//!   `1234567890.5` is "ایک ارب تئیس کروڑ ... نوے اعشاریہ پانچ".
//! * `str(float)` in exponent notation detonates in `int()`:
//!   `to_cardinal(1e-05)` raises `ValueError: invalid literal for int() with
//!   base 10: '1e-05'`, and `1.5e-05` fails later, on the `'e'` *character*
//!   of the digit loop. Both are reproduced, message for message.
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following all look wrong but are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **Tens and ones were merely juxtaposed (fixed,
//!    gladiaio/num2words2#247).** Urdu has its own word for each of 21..99,
//!    but Python concatenated the ten and the one with a space:
//!    `to_cardinal(21)` was "بیس ایک" (literally "twenty one"). The port
//!    reads [`BELOW_HUNDRED`]: 21 is "اکیس", 99 "ننانوے", `to_year(2024)`
//!    "دو ہزار چوبیس".
//! 2. **`_int_to_word` gave up at 10^9 (fixed, gladiaio/num2words2#147).**
//!    Python's final `else` is `return str(number)`, so 10^9 came back as
//!    "1000000000". The port continues the ladder with ارب (10^9), کھرب
//!    (10^11), نیل (10^13), پدم (10^15) and سنکھ (10^17), recursing on the
//!    quotient of سنکھ: `to_cardinal(10**9)` == "ایک ارب",
//!    `to_cardinal(10**21)` == "دس ہزار سنکھ". From 10^34 it raises
//!    `OverflowError`. See [`HIGH_SCALE`].
//! 3. **`to_ordinal` accepts negatives and zero without complaint.** Base's
//!    `verify_ordinal` (which would raise `TypeError` on a negative) is never
//!    called, and the dispatcher calls `to_ordinal(number)` directly. So
//!    `to_ordinal(0)` == "صفرواں" and `to_ordinal(-1)` == "منفی ایکواں" — the
//!    suffix is glued onto a *negative cardinal*. Unlike Polish, nothing
//!    crashes here.
//! 4. **The ordinal suffix is concatenated with no separator**, so it fuses
//!    with the last word: `to_ordinal(11)` == "گیارہواں", `to_ordinal(100)` ==
//!    "ایک سوواں".
//! 5. **`to_year` emits a Latin-script "BC " prefix** into otherwise
//!    Urdu-script output, and the positive branch is a no-op `"" + ...`:
//!    `to_year(-44)` == "BC چالیس چار".
//! 6. **`to_ordinal_num` appends a full stop**, ignoring the sign:
//!    `to_ordinal_num(-1)` == "-1.".
//!
//! # Currency
//!
//! `Num2Word_UR` defines `CURRENCY_FORMS` **in its own class dict** (verified:
//! `"CURRENCY_FORMS" in type(c).__dict__` is `True`), so it is *not* the shared
//! `Num2Word_EUR` dict that `Num2Word_EN.__init__` mutates at import time. UR
//! therefore sees exactly three codes — PKR, USD, EUR — and none of EN's ~24
//! additions (AUD/JPY/KWD/…). `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION`
//! are both inherited-empty (`{}`), confirmed on a live instance.
//!
//! `to_currency` is overridden wholesale and shares **not one line** with
//! `Num2Word_Base.to_currency` — it is byte-for-byte the same routine as
//! `Num2Word_BS.to_currency` apart from the plural selector. `to_cheque` is
//! *not* overridden, so it runs `Num2Word_Base.to_cheque` (i.e.
//! [`crate::currency::default_to_cheque`]) unchanged.
//!
//! More faithfully reproduced Python behaviour, all corpus-confirmed:
//!
//! 7. **An unknown currency code silently becomes PKR.** `to_currency` does
//!    `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["PKR"])` — there
//!    is no `KeyError`, hence no `NotImplementedError`. So `currency:GBP` on 0
//!    is "صفر روپے" (*rupees*, not pounds), and JPY/KWD/BHD/INR/CNY/CHF all
//!    render as rupees too. `to_cheque` uses `self.CURRENCY_FORMS[currency]`
//!    and therefore *does* raise for those same codes — the two surfaces
//!    disagree, which is why [`Lang::currency_forms`] here stays strict and the
//!    PKR fallback lives only inside [`LangUr::to_currency`].
//! 8. **`CURRENCY_PRECISION` is ignored entirely by `to_currency`.** The
//!    divisor is hardcoded at 100 by the `[:2].ljust(2, "0")` slice, so the
//!    3-decimal currencies (KWD/BHD) get 2-digit subunits and the 0-decimal
//!    JPY gets subunits at all: `currency:JPY` on 12.34 is
//!    "بارہ روپے تیس چار پیسے". Base's divisor-1/divisor-1000 handling is
//!    never reached. `to_cheque` *does* read `CURRENCY_PRECISION`, but it is
//!    `{}`, so its divisor is 100 for every code as well.
//! 9. **Fractional digits past the second are truncated, not rounded.**
//!    `parts[1][:2]` means 1.999 -> 99 subunits, and short fractions are
//!    right-padded: 0.5 -> "50" -> 50 subunits, not 5.
//! 10. **A float with zero cents drops the cents segment.** `if cents and
//!    right:` gates on `right` being truthy, so the float `1.0` renders as bare
//!    "ایک euro" — *unlike* `Num2Word_Base.to_currency`, which shows
//!    "one euro, zero cents" for a float. UR's int and float paths converge
//!    here; the `isinstance(val, int)` split base relies on is invisible to UR
//!    except through `str(val)`.
//! 11. **`cents=False` omits the cents segment rather than rendering digits.**
//!    Base falls back to `_cents_terse` ("12.34" -> "twelve euros,34"); UR's
//!    `if cents and right:` just drops it, so `cents=False` on 12.34 gives
//!    "بارہ euros". Not corpus-covered (the harness always passes `cents=True`)
//!    but reproduced.
//! 12. **`adjective` is accepted and ignored.** UR never consults
//!    `CURRENCY_ADJECTIVES` (which is empty anyway), so the flag is inert.
//!
//! # Error variants
//!
//! The four integer modes raise only `OverflowError`, at or past `maxval`
//! (10^34, bug 2). `to_currency` has no other raise either (bug 7's PKR
//! fallback removes the only `KeyError` site).
//!
//! One reachable raise is `to_cheque` on a code outside {PKR, USD, EUR}:
//! `Num2Word_Base.to_cheque` catches the `KeyError` from
//! `self.CURRENCY_FORMS[currency]` and re-raises `NotImplementedError`
//! (`N2WError::NotImplemented`), which `default_to_cheque` already produces
//! with Python's exact message.
//!
//! The other is the float path's `int()` on exponent-notation strings (see
//! above): `ValueError` (`N2WError::Value`), constructed by [`py_int`] with
//! CPython's exact message. No other error is constructed in this file.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use ڈالر / یورو with سینٹ. Examples in these docs
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

/// `self.negword` — note the **trailing space**, which is load-bearing:
/// `to_cardinal` concatenates it raw (`ret = self.negword`) rather than going
/// through base's `parse_minus`/`.strip()` dance.
const NEGWORD: &str = "منفی ";

/// `setup()`: `self.pointword = "اعشاریہ"`. Reached only through the
/// `"." in n` branch of `to_cardinal` — see [`LangUr::to_cardinal_float`].
/// Unlike `Num2Word_Base.to_cardinal_float`, which pushes
/// `self.title(self.pointword)` as its own list element, `Num2Word_UR`
/// concatenates `" " + pointword + " "` by hand and never calls `title()`.
/// Identical here (`is_title` is false), but reproduced as written. Must stay
/// byte-identical to [`Lang::pointword`], which was generated from the live
/// instance.
const POINTWORD: &str = "اعشاریہ";
const ZERO_WORD: &str = "صفر";
const HUNDRED: &str = "سو";
const THOUSAND: &str = "ہزار";
const LAKH: &str = "لاکھ";
const CRORE: &str = "کروڑ";

/// The scale words above the crore, as `(power of ten, word)`, largest first
/// (gladiaio/num2words2#147). Python stopped at کروڑ and returned the digits
/// from 10^9 up. ارب and کھرب are the everyday words in Pakistani news;
/// نیل, پدم and سنکھ continue the same hundredfold ladder (Urdu Wikipedia,
/// برصغیر اعدادی نظام; the same exponents as `lang_hi.rs`). The quotient
/// of the top word recurses, so 10^19 is "ایک سو سنکھ".
const HIGH_SCALE: [(u32, &str); 5] = [
    (17, "سنکھ"),
    (15, "پدم"),
    (13, "نیل"),
    (11, "کھرب"),
    (9, "ارب"),
];

/// The suffix `to_ordinal` glues onto the cardinal for anything outside 1..=10.
const ORDINAL_SUFFIX: &str = "واں";

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247).
///
/// Python had `ones`/`teens`/`tens` and joined a ten and a unit with a space
/// ("بیس ایک" for 21), but Urdu has its own word for every number below a
/// hundred. Index 0 is never read: `int_to_word` returns early on zero.
/// Spellings from Wiktionary's Urdu cardinal-number entries, cross-checked
/// against the Wikibooks Urdu numbers page and inurdu.pk.
const BELOW_HUNDRED: [&str; 100] = [
    "", "ایک", "دو", "تین", "چار", "پانچ", "چھ", "سات", "آٹھ", "نو", // 0..9
    "دس", "گیارہ", "بارہ", "تیرہ", "چودہ", "پندرہ", "سولہ", "سترہ", "اٹھارہ", "انیس", // 10..19
    "بیس", "اکیس", "بائیس", "تئیس", "چوبیس", "پچیس", "چھبیس", "ستائیس", "اٹھائیس", "انتیس", // 20..29
    "تیس", "اکتیس", "بتیس", "تینتیس", "چونتیس", "پینتیس", "چھتیس", "سینتیس", "اڑتیس", "انتالیس", // 30..39
    "چالیس", "اکتالیس", "بیالیس", "تینتالیس", "چوالیس", "پینتالیس", "چھیالیس", "سینتالیس", "اڑتالیس", "انچاس", // 40..49
    "پچاس", "اکیاون", "باون", "ترپن", "چون", "پچپن", "چھپن", "ستاون", "اٹھاون", "انسٹھ", // 50..59
    "ساٹھ", "اکسٹھ", "باسٹھ", "تریسٹھ", "چونسٹھ", "پینسٹھ", "چھیاسٹھ", "سڑسٹھ", "اڑسٹھ", "انہتر", // 60..69
    "ستر", "اکہتر", "بہتر", "تہتر", "چوہتر", "پچھتر", "چھہتر", "ستتر", "اٹھہتر", "اناسی", // 70..79
    "اسی", "اکیاسی", "بیاسی", "تراسی", "چوراسی", "پچاسی", "چھیاسی", "ستاسی", "اٹھاسی", "نواسی", // 80..89
    "نوے", "اکیانوے", "بانوے", "ترانوے", "چورانوے", "پچانوے", "چھیانوے", "ستانوے", "اٹھانوے", "ننانوے", // 90..99
];

/// The hardcoded 1..=10 ordinal ladder from `to_ordinal`, indexed by `n - 1`.
/// These are irregular forms, not `cardinal + ORDINAL_SUFFIX` — e.g. 1 is
/// "پہلا" (not "ایکواں") and 6 is "چھٹا" (not "چھواں"). Note 5, 7, 8, 9
/// and 10 *do* happen to coincide with the suffix rule, but Python spells them
/// out explicitly, so this table reproduces them verbatim.
const ORDINALS_1_TO_10: [&str; 10] = ["پہلا", "دوسرا", "تیسرا", "چوتھا", "پانچواں", "چھٹا", "ساتواں", "آٹھواں", "نواں", "دسواں"];

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_UR.to_currency` declares `separator=" "` in its own signature, but
/// the `Lang` trait takes the separator as a plain argument, and both
/// `num2words2/__init__.py`'s Rust fast path and `bench/diff_test.py`
/// substitute `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default,
/// not UR's — before the value ever reaches Rust. By the time we see it,
/// "caller omitted separator" and "caller asked for a comma" are the same
/// string and the distinction is unrecoverable.
///
/// Every currency row in the frozen corpus comes from `num2words(v, lang="ur",
/// to="currency", currency=c)` with no `separator=`, so Python renders them
/// with UR's " " ("بارہ euros تیس چار cents") while the harness feeds this core
/// a ",". Mapping "," back to " " restores UR's default and reproduces the
/// corpus. The single input it gets wrong is an *explicit* `separator=","`,
/// which Python renders as "بارہ euros,تیس چار cents" (no space — UR
/// concatenates the separator raw) and which we render with a space. Fixing
/// that properly means teaching the shim to pass each converter's own default;
/// `__init__.py` is off-limits here and shared by ~150 languages, so the
/// narrower divergence is the deliberate choice. `lang_bs.rs`, `lang_ca.rs`
/// and `lang_es.rs` hit the identical trap and resolve it the same way. See
/// `concerns`.
const SEPARATOR_UNSET: &str = ",";

/// `Num2Word_UR.CURRENCY_FORMS`'s fallback key. `to_currency` does
/// `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["PKR"])`, so PKR is
/// what every unrecognised code silently becomes — see bug 7.
const FALLBACK_CURRENCY: &str = "PKR";

pub struct LangUr {
    /// `Num2Word_UR.CURRENCY_FORMS`, built once here rather than per call.
    ///
    /// UR's own class dict, untouched by `Num2Word_EN.__init__`'s mutation of
    /// the `Num2Word_EUR` dict — so exactly three codes, no AUD/JPY/KWD/…
    /// leakage. Two forms per side: `to_currency` only ever indexes `[0]`/`[1]`
    /// (`cr1[1] if left != 1 else cr1[0]`), matching Python's arity exactly.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `self.CURRENCY_FORMS["PKR"]`, kept alongside the map purely so the
    /// `.get(currency, <default>)` fallback in `to_currency` is a borrow with
    /// no panic path. Python evaluates that default eagerly on every call and
    /// would `KeyError` if PKR were absent; here it cannot be.
    pkr: CurrencyForms,
}

impl Default for LangUr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangUr {
    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "PKR": (("روپیہ", "روپے"), ("پیسہ", "پیسے")),
        //     "USD": (("dollar", "dollars"), ("cent", "cents")),
        //     "EUR": (("euro", "euros"), ("cent", "cents")),
        // }
        // Verified against the live interpreter, not the source literal — but
        // unlike the lang_EUR-derived classes, the two agree here: UR owns its
        // dict, so nothing rewrites it at import time. Note EUR is
        // ("euro", "euros") in UR's *own* literal, which happens to coincide
        // with what EN's mutation produces for the shared dict; the agreement
        // is a coincidence, not inheritance.
        let pkr = CurrencyForms::new(&["روپیہ", "روپے"], &["پیسہ", "پیسے"]);
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (FALLBACK_CURRENCY, pkr.clone()),
            (
                "USD",
                CurrencyForms::new(&["ڈالر", "ڈالر"], &["سینٹ", "سینٹ"]),
            ),
            (
                "EUR",
                CurrencyForms::new(&["یورو", "یورو"], &["سینٹ", "سینٹ"]),
            ),
        ]
        .into_iter()
        .collect();

        LangUr { currency_forms, pkr }
    }
}

/// Python's `str(abs(val)).split(".")` bookkeeping, shared by both arms of
/// `to_currency`:
///
/// ```python
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// Returns `(left, right, is_negative)`. `right` is always a 2-digit subunit
/// count in `0..=99`, so `u32` is ample.
///
/// This works off `as_bigint_and_exponent()` rather than re-rendering the
/// `BigDecimal`, which keeps us out of `Display`'s notation heuristics: the
/// value is `digits * 10^-scale` with both taken verbatim from the `str(value)`
/// the shim parsed, so reconstructing from them *is* Python's string. Python's
/// `abs()` before `str()` is just the sign of `digits`.
fn split_value(val: &CurrencyValue) -> (BigInt, u32, bool) {
    match val {
        // str(int) has no ".", so parts == [digits] and right stays 0. This is
        // the only trace of base's `isinstance(val, int)` split that survives
        // into UR — and it happens to agree with it (bug 10).
        CurrencyValue::Int(v) => (v.abs(), 0, v.is_negative()),
        CurrencyValue::Decimal { value: d, .. } => {
            let is_negative = d.is_negative();
            let (digits, scale) = d.as_bigint_and_exponent();
            let digits = digits.abs();

            if scale <= 0 {
                // No "." in the string: the value is integral as written.
                // scale == 0 is the ordinary case (Decimal("100")).
                // scale < 0 means exponent notation, where Python takes a
                // different and broken route — see `concerns`.
                let pow = BigInt::from(10u32).pow((-scale) as u32);
                return (digits * pow, 0, is_negative);
            }

            let pow = BigInt::from(10u32).pow(scale as u32);
            let (left, frac) = digits.div_rem(&pow);
            // `parts[1]` is `frac` left-padded with zeros to `scale` digits, so
            // `int(parts[1][:2].ljust(2, "0"))` is exactly "the first two
            // fractional digits as a 2-digit number": floor division that keeps
            // leading zeros and right-pads short fractions.
            //   0.5    -> frac 5,   scale 1 -> 5*100/10    = 50  (not 5)
            //   0.01   -> frac 1,   scale 2 -> 1*100/100   = 1
            //   12.34  -> frac 34,  scale 2 -> 34*100/100  = 34
            //   1.999  -> frac 999, scale 3 -> 999*100/1000 = 99 (bug 9)
            //   1.005  -> frac 5,   scale 3 -> 5*100/1000  = 0
            let right = (frac * BigInt::from(100u32) / &pow).to_u32().unwrap_or(0);
            (left, right, is_negative)
        }
    }
}

/// Narrow a `BigInt` the caller has already proven is small.
///
/// Every call site sits inside a branch that bounds the value below 1000, so
/// the conversion cannot truncate. Python indexes its lists with the raw int
/// and would `IndexError` if this were ever violated; the `unwrap_or(0)` keeps
/// us panic-free without inventing an error variant Python never raises.
fn small(n: &BigInt) -> usize {
    n.to_usize().unwrap_or(0)
}

/// Shared tail of the thousand/lakh/crore branches of `_int_to_word`.
///
/// Python repeats this shape three times:
/// ```text
/// result = self._int_to_word(number // divisor) + " " + word
/// remainder = number % divisor
/// if remainder:
///     result += " " + self._int_to_word(remainder)
/// ```
/// `n` is strictly positive at every call site, so Python's floor `//`/`%` and
/// `div_rem` agree.
fn group(n: &BigInt, divisor: &BigInt, word: &str) -> String {
    let (div, rem) = n.div_rem(divisor);
    let mut result = format!("{} {}", int_to_word(&div), word);
    if !rem.is_zero() {
        result.push(' ');
        result.push_str(&int_to_word(&rem));
    }
    result
}

/// The exclusive ceiling (gladiaio/num2words2#147, #203): the largest scale word is
/// سنکھ (10^17), so from 10^34 its multiplier would itself need سنکھ.
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(34))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

/// Port of `Num2Word_UR._int_to_word`.
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Python's `if number < 0` arm. Dead code across all four in-scope modes:
    // `to_cardinal` strips the sign before calling in, and `to_year` hands over
    // `abs(val)`. Kept because Python keeps it — and note it would double the
    // negword ("منفی منفی ...") if it ever were reached.
    if n.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&n.abs()));
    }

    if n < &BigInt::from(100) {
        return BELOW_HUNDRED[small(n)].to_string();
    }

    if n < &BigInt::from(1000) {
        let v = small(n);
        // Python indexes `self.ones[number // 100]` directly here rather than
        // recursing, so 100 is "ایک سو" ("one hundred"), never bare "سو".
        let mut result = format!("{} {}", BELOW_HUNDRED[v / 100], HUNDRED);
        let rem = v % 100;
        if rem != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(rem)));
        }
        return result;
    }

    // South-Asian grouping: thousand (10^3), then lakh (10^5), then crore
    // (10^7) — not the western thousand/million/billion ladder.
    if n < &BigInt::from(100_000) {
        return group(n, &BigInt::from(1000), THOUSAND);
    }

    if n < &BigInt::from(10_000_000) {
        return group(n, &BigInt::from(100_000), LAKH);
    }

    if n < &BigInt::from(1_000_000_000) {
        return group(n, &BigInt::from(10_000_000), CRORE);
    }

    // Python returned `str(number)` here (bug 2, fixed): the largest scale
    // word that fits, its quotient spelled recursively.
    let ten = BigInt::from(10u8);
    let (power, word) = HIGH_SCALE
        .iter()
        .find(|(p, _)| *n >= ten.pow(*p))
        .expect("n >= 10^9 here, and HIGH_SCALE ends at 10^9");
    group(n, &ten.pow(*power), word)
}

/// The shortest round-tripping decimal digits of `a` (which must be finite,
/// non-negative), plus `decpt`, the decimal-point position such that
/// `a == 0.<digits> * 10**decpt`. This is CPython's `_Py_dg_dtoa(a, 0, 0, ...)`
/// — David Gay's `dtoa` in mode 0.
///
/// Rust's `{:e}` is also shortest-round-trip, and agrees with Gay on both the
/// digit *count* and, almost always, the digits. It disagrees on one thing:
/// **exact ties**. When `a`'s exact binary value sits precisely halfway between
/// the two shortest candidates, both round-trip, and Gay's dtoa takes the one
/// with an **even** last digit (`if (dig & 1) goto bump_up;`) while Rust rounds
/// half **up**. `repr(-78198386800398.125)` is `'-78198386800398.12'`; Rust's
/// `{:e}` says `...13`. The sibling `lang_as.rs` port fuzzed this over 3.7M
/// values (2.4M of them `m/2**k`, the tie-prone shape): the tie is the *only*
/// divergence, and the correction below fixes every one without introducing a
/// single new mismatch.
///
/// Detecting the tie needs no bignum. Write `a = m * 2**e` with `m` odd, and
/// let `q = digits.len() - decpt`. The tie condition — `a * 10**q == k + 1/2`
/// for integer `k` — reduces to:
///
/// * `e + q + 1 == 0`, and
/// * if `q < 0`, additionally `5**-q` divides `m` (and `-q <= 22`, since
///   `5**23 > 2**53 > m`).
///
/// Then `2k + 1 == m * 5**q` (or `m / 5**-q`), and because `5 ≡ 1 (mod 4)`,
/// that odd integer is `≡ m (mod 4)` either way. So `k` is even exactly when
/// `m % 4 == 1` — no big integers, no exact decimal expansion.
///
/// The fix-up itself is direction-agnostic: in a tie Python's answer is always
/// the candidate with the even last digit, so if Rust's last digit is odd we
/// step toward the even neighbour, and `k`'s parity says which way.
fn shortest_digits(a: f64) -> (String, i32) {
    // "d[.ddd]e<exp>", shortest round-trip.
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
            // k is even, so Python wants k and Rust gave k+1. The last digit
            // is odd, hence non-zero, so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k is odd, so Python wants k+1 and Rust gave k. Carry like
            // dtoa's `roundoff`: "99" -> "1" with decpt bumped.
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
        // dtoa never emits trailing zeros; stripping them leaves decpt alone.
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), which `Num2Word_UR.to_cardinal`
/// promotes from a formatting detail to *the entire specification* of the
/// float path.
///
/// This is CPython's `format_float_short(..., 'r', ...)` in `pystrtod.c`.
/// Rust's own `{}` cannot stand in: it never switches to exponent notation
/// (`format!("{}", 1e16_f64)` is `"10000000000000000"`, where Python says
/// `'1e+16'` — and that difference is the whole reason `to_cardinal(1e16)`
/// raises `ValueError`) and it prints `1`, not `1.0`, for integral floats.
///
/// The rules, straight from `format_float_short`:
///
/// * exponent notation iff `decpt <= -4 || decpt > 16` — the `> 16` (rather
///   than `> 17`) is deliberate upstream, so that `repr(2e16+8)` does not
///   render as `20000000000000010.0`;
/// * the exponent is `%+.02d`: always signed, zero-padded to two digits, hence
///   `1e-05` but `1e+100`;
/// * `Py_DTSF_ADD_DOT_0` appends `.0` to an otherwise integral fixed-notation
///   result, but never in exponent notation (`repr(1e16) == '1e+16'`);
/// * `nan` drops its sign, `inf` keeps it. Rust would say `NaN`/`inf`.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        // CPython prints "nan" for both signs of NaN.
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and to_cardinal
    // strips that minus textually into a negword.
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
        // 0.5 -> decpt 0 -> "0." + "" + "5"; 0.01 -> decpt -1 -> "0." + "0" + "1".
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        // Integral: pad right with zeros, then ADD_DOT_0. 1.0 -> "1" + ".0".
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

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__` with `eng=False` and
/// the default context (`capitals=1`, hence an uppercase `E`).
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`: the shim builds this value with
/// `BigDecimal::from_str(str(value))`, and that parse preserves trailing zeros
/// and negative exponents rather than normalising, so `"1.10"` round-trips as
/// `(110, 2)` and `"1E+16"` as `(1, -16)`.
///
/// The `leftdigits > -6` guard is what keeps every `"0".repeat(...)` below
/// bounded by five: in the no-exponent branch `_exp <= 0` forces
/// `dotplace <= len(_int)`, and in the exponent branch `dotplace == 1`. So
/// `Decimal("1E-1000000000")` renders as `'1E-1000000000'` rather than trying
/// to materialise a billion zeros — same as Python.
///
/// # The negative-zero hole
///
/// `Decimal` carries `_sign` independently of `_int`, so `Decimal("-0.0")` is
/// signed zero and `str()` gives `'-0.0'`; `to_cardinal` then strips that
/// minus textually and answers "منفی صفر اعشاریہ صفر". A `BigDecimal` cannot
/// represent it: its `int_val` is a `BigInt`, which has no negative zero, so
/// `BigDecimal::from_str("-0.0")` has already discarded the sign before this
/// function is called. We emit `'0.0'` and drop the negword. **Unreachable
/// through the shim today**: every negative-zero Decimal is numerically whole,
/// and `__init__.py` routes whole values to the pure-Python converter — the
/// hole only opens if a caller drives the core directly. The discriminator is
/// the original string, which the `FloatValue::Decimal` boundary does not
/// carry; fixing it needs the shim to pass `decimal_str` through, which lives
/// in `num2words2-py` — outside this port's remit. Flagged in the port report.
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
        // "%+d" — signed, but *not* zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)`, for the strings [`cardinal_from_str`] hands it.
///
/// The real builtin is more permissive than this: it also accepts non-ASCII
/// decimal digits (`int("١٢") == 12`) and strips a slightly different
/// whitespace set. None of that is reachable — every string that gets here is
/// a fragment of `str(float)` / `str(Decimal)`, which is ASCII by
/// construction — so the extra generality is deliberately not ported. What is
/// ported is the underscore rule (`int("1_0") == 10`, `int("1_")` raises) and,
/// crucially, the error message: Python formats the **original, unstripped**
/// argument with `%.200R`, i.e. `repr(s)`. Every literal that reaches this
/// function is plain ASCII with no quote or backslash, so `'{}'` is exactly
/// what `repr` would print.
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
    // int() permits '_' as a digit separator, but not leading, trailing or
    // doubled.
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

/// The body of `Num2Word_UR.to_cardinal`, driven by `str(number)`:
///
/// ```text
/// n = str(number).strip()
/// if n.startswith("-"):
///     n = n[1:]
///     ret = self.negword
/// else:
///     ret = ""
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
///     ret += " ".join(self._int_to_word(int(d)) for d in right)
///     return ret.strip()
/// else:
///     return (ret + self._int_to_word(int(n))).strip()
/// ```
///
/// Four details that matter:
///
/// * The sign is stripped **textually**, so `-0.0` (whose `str` is `"-0.0"`)
///   keeps its negword even though the value is not `< 0`: Python answers
///   "منفی صفر اعشاریہ صفر". `Num2Word_Base.to_cardinal_float` reaches the
///   same place via `if value < 0 and pre == 0`, which `-0.0` would fail.
/// * `split(".", 1)` caps at one split, so a second dot stays inside `right`
///   and detonates in the digit loop rather than being ignored.
/// * The digit loop is a generator consumed by `join`, and `int(left)` runs
///   *before* it. So for `1.5e+16` the failing literal reported is `'e'`, not
///   `'1.5e+16'` — the left half parsed fine and the loop got as far as the
///   `e`. Order is load-bearing for the error message; keep it.
/// * Both returns go through `.strip()` (unlike sibling `lang_AS`, which
///   returns `ret` raw). A no-op for every reachable input — `negword`'s
///   trailing space is always followed by a word, and `_int_to_word` never
///   returns "" — but preserved for fidelity, exactly like the integer path.
fn cardinal_from_str(number: &str) -> Result<String> {
    let n = number.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    let Some(dot) = n.find('.') else {
        ret.push_str(&checked_int_to_word(&py_int(n)?)?);
        return Ok(ret.trim().to_string());
    };

    // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
    let (left, right) = (&n[..dot], &n[dot + 1..]);
    ret.push_str(&checked_int_to_word(&py_int(left)?)?);
    ret.push(' ');
    ret.push_str(POINTWORD);
    ret.push(' ');

    // " ".join(self._int_to_word(int(d)) for d in right) — Python iterates
    // *characters*, so index by chars(), never bytes.
    let mut first = true;
    for d in right.chars() {
        if !first {
            ret.push(' ');
        }
        first = false;
        let mut buf = [0u8; 4];
        ret.push_str(&int_to_word(&py_int(d.encode_utf8(&mut buf))?));
    }
    Ok(ret.trim().to_string())
}

impl Lang for LangUr {
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
        "PKR"
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
        "اعشاریہ"
    }

    /// Port of `Num2Word_UR.to_cardinal`.
    ///
    /// Python works on `str(number)`: it strips a leading "-", stashes
    /// `self.negword` (trailing space included), re-parses with `int()`, and
    /// `.strip()`s the result. For integral input the `"." in n` branch is
    /// unreachable, and the str/int round-trip is exactly `abs()`.
    ///
    /// The final `.strip()` never actually removes anything — `_int_to_word`
    /// cannot return the empty string (zero short-circuits to "صفر" before
    /// `ONES[0]` could be reached) — but it is preserved for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// The `"." in n` branch of `Num2Word_UR.to_cardinal`, which is the same
    /// method as [`LangUr::to_cardinal`] — Python has one `to_cardinal` and
    /// splits on `str(number)`, so the trait's int/float split lands mid-method
    /// here rather than at a class boundary.
    ///
    /// **`Num2Word_Base.to_cardinal_float` is never reached for Urdu**, and
    /// neither is `float2tuple`. The consequences are worth being explicit
    /// about, because they invert the usual advice for this path:
    ///
    /// * There is no `precision` anywhere. `num2words(..., precision=2)` does
    ///   set `converter.precision` (base's `__init__` leaves the attribute at
    ///   `2`, so `hasattr` passes and the kwarg is honoured *syntactically*),
    ///   but `Num2Word_UR.to_cardinal` never reads it. Verified live:
    ///   `num2words(1.23456, lang="ur", precision=1)` is unchanged at six
    ///   fractional words. So `precision_override` is accepted and dropped —
    ///   the same shape as `to_year`'s `longval` (bug 5) and `to_currency`'s
    ///   `adjective` (bug 12).
    /// * `FloatValue::precision` is likewise unused: the digit count comes
    ///   from `repr`, not from a precision field.
    /// * Every digit after the point is spoken separately, with no cap, so
    ///   `0.0001` is four fractional words and `98746251323029.99` (Decimal)
    ///   keeps full precision for free — the `float()` cast of issue #603
    ///   never happens because the Decimal arm stringifies instead of
    ///   converting.
    ///
    /// The `< 0.01` artefact heuristic that `float2tuple` needs is absent here
    /// and *not* missing: `repr(2.675)` is `"2.675"`, so the
    /// `674.9999999999998` it exists to repair is never computed. See the
    /// module docs.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // precision= is set on the converter by __init__.py, then ignored by
        // Num2Word_UR.to_cardinal. Reproduce the ignoring.
        let _ = precision_override;
        let n = match value {
            // Python's str(float). The raw f64 crosses the boundary precisely
            // so that repr() can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, and never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    /// Port of `Num2Word_UR.to_ordinal`.
    ///
    /// 1..=10 are irregular table lookups; everything else — including 0 and
    /// every negative (bug 3) — is `to_cardinal(number) + ORDINAL_SUFFIX`,
    /// concatenated with no separator (bug 4).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_positive() && value <= &BigInt::from(10) {
            return Ok(ORDINALS_1_TO_10[small(value) - 1].to_string());
        }
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_UR.to_ordinal_num`: `str(number) + "."` (bug 6).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_UR.to_year`.
    ///
    /// Negative years get a Latin-script "BC " prefix; positive years are
    /// `"" + self.to_cardinal(val)`, i.e. plain `to_cardinal` (bug 5). The
    /// Python signature carries an unused `longval=True` parameter.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("BC {}", self.to_cardinal(&value.abs())?))
        } else {
            self.to_cardinal(value)
        }
    }

    /// `to_ordinal(float/Decimal)`: the `number == 1 ... == 10` chain
    /// matches numerically, so whole 1..=10 (5.0, Decimal("5.00")) take the
    /// irregular forms; everything else is `to_cardinal(number) + "واں"` off
    /// the value's own str() grammar ("گیارہ اعشاریہ صفرواں" for 11.0).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i.is_positive() && i <= BigInt::from(10) {
                return self.to_ordinal(&i);
            }
        }
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
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

    /// `str_to_number` stays Base's `Decimal(value)`, but UR's `to_cardinal`
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

    // ---- currency ------------------------------------------------------
    //
    // Only three hooks are overridden. The rest stay at their trait defaults
    // because that is exactly what Python does:
    //
    // * `currency_precision` — UR's CURRENCY_PRECISION is `{}`, so
    //   `.get(code, 100)` is 100 for every code; the default returns 100.
    // * `currency_adjective` — CURRENCY_ADJECTIVES is `{}`; the default is None
    //   (and `to_currency` never consults it anyway, bug 12).
    // * `money_verbose` — base's `_money_verbose` is `self.to_cardinal(number)`;
    //   the default does the same and picks up UR's `to_cardinal`. This is the
    //   only hook `to_cheque` reaches, and it is what makes cheque output
    //   Urdu-script.
    // * `pluralize`, `cents_verbose`, `cents_terse`, `cardinal_from_decimal` —
    //   unreachable. `to_currency` is a full override that open-codes its own
    //   plural selection and never calls them, and `to_cheque` does not use
    //   them either. `pluralize` staying at its raising default is correct:
    //   `Num2Word_UR` never defines one, so reaching it in Python would raise
    //   `NotImplementedError` too.

    fn lang_name(&self) -> &str {
        "Num2Word_UR"
    }

    /// `self.CURRENCY_FORMS[code]` — the **strict** lookup, used by
    /// `to_cheque`. `to_currency`'s lenient `.get(code, PKR)` fallback (bug 7)
    /// deliberately does *not* go through here; keeping this strict is what
    /// makes `cheque:GBP` raise `NotImplementedError` while `currency:GBP`
    /// quietly returns rupees, exactly as the corpus records.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_UR.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="PKR", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["PKR"])
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// Note it calls `_int_to_word` directly, never `to_cardinal` — so the sign
    /// is handled once, here, and cannot double up through `_int_to_word`'s own
    /// negative arm.
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
        // Restore UR's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            " "
        } else {
            separator
        };

        let (left, right, is_negative) = split_value(val);

        // `.get(currency, self.CURRENCY_FORMS["PKR"])` — bug 7. This bypasses
        // the `currency_forms` hook on purpose; that one is strict.
        let forms = self.currency_forms.get(currency).unwrap_or(&self.pkr);

        let left_str = checked_int_to_word(&left)?;
        // `cr1[1] if left != 1 else cr1[0]` — a plain singular/plural pick, not
        // `pluralize`. Zero takes the plural ("صفر euros").
        let unit = if left.is_one() {
            &forms.unit[0]
        } else {
            &forms.unit[1]
        };
        let mut result = format!("{} {}", left_str, unit);

        // `if cents and right:` — a zero `right` suppresses the segment, which
        // is why the float 1.0 renders as bare "ایک euro" (bug 10), and why
        // `cents=False` drops the segment rather than rendering digits (bug 11).
        if cents && right != 0 {
            let right_big = BigInt::from(right);
            let cents_str = int_to_word(&right_big);
            let subunit = if right == 1 {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            };
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(subunit);
        }

        if is_negative {
            // `self.negword` — "منفی ", trailing space included, prepended raw.
            result.insert_str(0, NEGWORD);
        }

        // `.strip()`. A no-op for every reachable input (`_int_to_word` never
        // returns "", so nothing pads the ends), kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn flt(v: f64) -> String {
        // `precision` is what the shim derives from repr() on the Python side.
        // Num2Word_UR ignores it, so any value here is equally correct; pass
        // the honest one.
        let precision = py_float_repr(v)
            .split_once('.')
            .map(|(_, f)| f.len() as u32)
            .unwrap_or(0);
        LangUr::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        LangUr::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `"lang": "ur", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(flt(0.0), "صفر اعشاریہ صفر");
        assert_eq!(flt(0.5), "صفر اعشاریہ پانچ");
        assert_eq!(flt(1.0), "ایک اعشاریہ صفر");
        assert_eq!(flt(1.5), "ایک اعشاریہ پانچ");
        assert_eq!(flt(2.25), "دو اعشاریہ دو پانچ");
        assert_eq!(flt(3.14), "تین اعشاریہ ایک چار");
        assert_eq!(flt(0.01), "صفر اعشاریہ صفر ایک");
        assert_eq!(flt(0.1), "صفر اعشاریہ ایک");
        assert_eq!(flt(0.99), "صفر اعشاریہ نو نو");
        assert_eq!(flt(1.01), "ایک اعشاریہ صفر ایک");
        assert_eq!(flt(12.34), "بارہ اعشاریہ تین چار");
        assert_eq!(flt(99.99), "ننانوے اعشاریہ نو نو");
        assert_eq!(flt(100.5), "ایک سو اعشاریہ پانچ");
        assert_eq!(flt(1234.56), "ایک ہزار دو سو چونتیس اعشاریہ پانچ چھ");
        assert_eq!(flt(-0.5), "منفی صفر اعشاریہ پانچ");
        assert_eq!(flt(-1.5), "منفی ایک اعشاریہ پانچ");
        assert_eq!(flt(-12.34), "منفی بارہ اعشاریہ تین چار");
        // The two f64-artefact cases. Right answer, but via repr(), not via
        // float2tuple's `< 0.01` rescue — see the module docs.
        assert_eq!(flt(1.005), "ایک اعشاریہ صفر صفر پانچ");
        assert_eq!(flt(2.675), "دو اعشاریہ چھ سات پانچ");
    }

    /// Every `"lang": "ur", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(dec("0.01"), "صفر اعشاریہ صفر ایک");
        // Trailing zero survives: str(Decimal("1.10")) == "1.10", two digits.
        assert_eq!(dec("1.10"), "ایک اعشاریہ ایک صفر");
        assert_eq!(dec("12.345"), "بارہ اعشاریہ تین چار پانچ");
        // Issue #603's value: full precision, no float() cast anywhere. The
        // integer part is past 10^9 and takes the scale words (#147).
        assert_eq!(
            dec("98746251323029.99"),
            "نو نیل ستاسی کھرب چھیالیس ارب پچیس کروڑ تیرہ لاکھ تئیس ہزار انتیس اعشاریہ نو نو"
        );
        assert_eq!(dec("0.001"), "صفر اعشاریہ صفر صفر ایک");
    }

    /// -0.0 is not `< 0`, but its str() starts with '-', and to_cardinal
    /// strips the sign textually. Base's float path would drop the negword.
    #[test]
    fn negative_zero_keeps_negword() {
        assert_eq!(flt(-0.0), "منفی صفر اعشاریہ صفر");
        assert_eq!(flt(0.0), "صفر اعشاریہ صفر");
    }

    /// precision= is honoured by __init__.py and then ignored by the converter.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangUr::new();
        let v = FloatValue::Float {
            value: 1.23456,
            precision: 5,
        };
        let want = "ایک اعشاریہ دو تین چار پانچ چھ";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want);
        for p in [0u32, 1, 2, 5, 9] {
            assert_eq!(l.to_cardinal_float(&v, Some(p)).unwrap(), want);
        }
    }

    /// str(float) goes exponential outside [1e-4, 1e16), and int() chokes.
    /// Which literal lands in the message depends on whether a '.' split the
    /// string first. All verified against the pure-Python oracle.
    #[test]
    fn exponent_notation_raises_value_error() {
        let l = LangUr::new();
        let f = |v: f64| {
            l.to_cardinal_float(&FloatValue::Float { value: v, precision: 0 }, None)
                .unwrap_err()
        };
        // No '.' in "1e+16" -> the whole string reaches int().
        assert!(matches!(f(1e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e+16'"));
        assert!(matches!(f(1e21), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e+21'"));
        assert!(matches!(f(1e-5), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e-05'"));
        // "1.5e+16" splits: int("1") is fine, then the digit loop hits 'e'.
        assert!(matches!(f(1.5e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'e'"));
        assert!(matches!(f(f64::INFINITY), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'inf'"));
        assert!(matches!(f(f64::NAN), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'nan'"));
        // Decimal's exponent form is uppercase, and its own threshold differs.
        let d = |s: &str| {
            l.to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision: 0,
                },
                None,
            )
            .unwrap_err()
        };
        assert!(matches!(d("1E+16"), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1E+16'"));
        assert!(matches!(d("1E-7"), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1E-7'"));
        assert!(matches!(d("1.5E-7"), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'E'"));
    }

    /// Integer parts at or past 10^9 take the scale words (bug 2, #147),
    /// and the last value below 1e16 still renders in fixed notation.
    #[test]
    fn large_and_small_boundaries() {
        assert_eq!(
            flt(1234567890.5),
            "ایک ارب تئیس کروڑ پینتالیس لاکھ سڑسٹھ ہزار آٹھ سو نوے اعشاریہ پانچ"
        );
        assert_eq!(flt(1e15), "ایک پدم اعشاریہ صفر");
        assert_eq!(flt(0.0001), "صفر اعشاریہ صفر صفر صفر ایک");
        // Decimal keeps fixed notation down to 1e-6 (leftdigits > -6).
        assert_eq!(dec("0.000001"), "صفر اعشاریہ صفر صفر صفر صفر صفر ایک");
        // Decimal("1.00E+2") normalises to str "100": no dot, no pointword.
        assert_eq!(dec("1.00E+2"), "ایک سو");
        assert_eq!(dec("0.00"), "صفر اعشاریہ صفر صفر");
        assert_eq!(dec("-0.5"), "منفی صفر اعشاریہ پانچ");
        assert_eq!(dec("-12.34"), "منفی بارہ اعشاریہ تین چار");
    }

    /// A dtoa exact tie: the true value is ...398.125, and CPython's repr
    /// rounds the last digit to even ("...12") where Rust's `{:e}` rounds up.
    /// Verified against the pure-Python oracle.
    #[test]
    fn dtoa_tie_rounds_to_even() {
        assert_eq!(
            flt(-78198386800398.125),
            "منفی سات نیل اکیاسی کھرب اٹھانوے ارب اڑتیس کروڑ اڑسٹھ لاکھ تین سو اٹھانوے اعشاریہ ایک دو"
        );
    }

    /// repr() reproduction, including the tie cases where Rust's `{:e}` and
    /// Gay's dtoa disagree. Exhaustively fuzzed against CPython in the
    /// lang_as.rs sibling port (identical code).
    #[test]
    fn py_float_repr_matches_cpython() {
        for (v, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (0.5, "0.5"),
            (1.0, "1.0"),
            (0.01, "0.01"),
            (1.005, "1.005"),
            (2.675, "2.675"),
            (100.5, "100.5"),
            (1e15, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1.5e16, "1.5e+16"),
            (1e21, "1e+21"),
            (1e100, "1e+100"),
            (0.0001, "0.0001"),
            (1e-5, "1e-05"),
            (5e-324, "5e-324"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (98746251323029.99, "98746251323029.98"),
            // Exact ties: the true values are ...398.125 and ...775.25, and
            // dtoa rounds the last digit to even. Rust's {:e} rounds up.
            (-78198386800398.125, "-78198386800398.12"),
            (-1267860061485775.25, "-1267860061485775.2"),
        ] {
            assert_eq!(py_float_repr(v), want, "repr({:?})", v);
        }
    }

    /// str(Decimal) reproduction — _pydecimal.Decimal.__str__.
    #[test]
    fn py_decimal_str_matches_cpython() {
        for (s, want) in [
            ("1.10", "1.10"),
            ("0.00", "0.00"),
            ("-0.5", "-0.5"),
            ("1E+16", "1E+16"),
            ("1E-7", "1E-7"),
            ("0.0000001", "1E-7"),
            ("0.000001", "0.000001"),
            ("1.00E+2", "100"),
            ("12.345", "12.345"),
            ("98746251323029.99", "98746251323029.99"),
            ("0.01", "0.01"),
        ] {
            assert_eq!(py_decimal_str(&BigDecimal::from_str(s).unwrap()), want, "{}", s);
        }
    }
}
