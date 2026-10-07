//! Port of `lang_PLI.py` (Pali).
//!
//! Shape: **self-contained**. `Num2Word_PLI` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` skips the `cards`/`set_numwords()` block entirely
//! and **never sets `MAXVAL`**. `to_cardinal` is overridden outright and drives
//! `_int_to_word` by magnitude bands. Consequently `cards`/`maxval`/`merge`
//! stay at their trait defaults here, and there is **no overflow check** — see
//! bug 1 below for what happens past the top band instead.
//!
//! Inherited from `Num2Word_Base` but effectively unused:
//!   * `is_title` is `False` and PLI never calls `self.title()` from its own
//!     `to_cardinal`, so no title-casing is applied on any of the four paths.
//!     `setup()` still populates `exclude_title = ["ca", "bindu", "ūna"]`,
//!     which is therefore dead weight; it is reproduced anyway for parity.
//!
//! PLI overrides all four in-scope entry points (`to_cardinal`, `to_ordinal`,
//! `to_ordinal_num`, `to_year`), so nothing inherits from `base.rs` here.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Both of the following look wrong but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **`_int_to_word` fell off the end at 10^9 (fixed,
//!    gladiaio/num2words2#147).** The band ladder stopped at
//!    `number < 1000000000` and returned `str(number)`, so 10^9 was
//!    `"1000000000"` and `to_ordinal(10**9)` `"1000000000ma"`. The port adds
//!    a koṭi band (10^7) whose count is spelled out — 10^9 is "eka sata
//!    koṭi" — and raises `OverflowError` from 10^14 ([`maxval_ceiling`]).
//! 2. **Tens and units were joined with " ca " (fixed,
//!    gladiaio/num2words2#247).** 23 was "vīsati ca ti"; Pali compounds it,
//!    tevīsati. See [`BELOW_HUNDRED`]. (Python's unreachable `tens[1]` went
//!    with the table.)
//!
//! # Non-bug quirks worth not "fixing"
//!
//! * `negword` is `"ūna "` — with a **trailing space** baked into the word, not
//!    supplied by the caller. `to_cardinal` concatenates then `.strip()`s.
//! * The `" ca "` infix joins hundreds↔remainder, but the thousands,
//!   millions and koṭi bands join with a **plain space** instead
//!   (`"eka sahassa eka"`, not `"eka sahassa ca eka"`). Asymmetric, and correct
//!   per the corpus.
//! * `million` is spelled `"dasa-lakkha"` (literally "ten lakh"), hyphen and
//!   all — the one multi-token numword in the table.
//!
//! # The currency surface
//!
//! `Num2Word_PLI` overrides `to_currency` **wholesale** and shares nothing with
//! `Num2Word_Base`'s currency machinery. Concretely it never touches
//! `parse_currency_parts`, `prefix_currency`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse` or `pluralize`: it does its own string
//! surgery on `str(val)` and calls `_int_to_word` directly for both halves. It
//! also never reads `CURRENCY_PRECISION`, so there is no divisor and no
//! `ROUND_HALF_UP` quantize — the cent count is always the first two fractional
//! digits (quirks 3 and 4 below).
//!
//! `to_cheque` is the mirror image: PLI does **not** define it, so
//! `Num2Word_Base.to_cheque` runs unchanged and reaches `CURRENCY_FORMS`,
//! `CURRENCY_PRECISION` and `_money_verbose` through the trait defaults. The
//! two entry points therefore disagree about an unknown currency code —
//! `to_currency` silently falls back to USD (quirk 5) while `to_cheque` indexes
//! `CURRENCY_FORMS[currency]` and raises `NotImplementedError`. Both are pinned
//! by the frozen corpus and both are reproduced here.
//!
//! `CURRENCY_FORMS` is a plain class attribute on `Num2Word_PLI` with three
//! entries. Nothing merges into it: PLI subclasses `Num2Word_Base` directly
//! (not `lang_EUR`/`lang_EU`), and `Num2Word_Base.__init__` has no
//! `CURRENCY_FORMS` merge step. `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION`
//! are both `{}` on the base and are never overridden, so
//! `currency_adjective` (`None`) and `currency_precision` (`100`) stay at their
//! trait defaults.
//!
//! # Faithfully reproduced Python quirks (currency)
//!
//! 3. **A float with zero cents drops the cents segment.** The guard is
//!    `if cents and right:` and `right` is an `int`, so `0` is falsy. This is
//!    the *opposite* of `Num2Word_Base.to_currency`, which shows `1.0` as
//!    "one euro, zero cents" because it branches on `isinstance(val, int)`
//!    rather than on the cent count. PLI stringifies first and so cannot tell
//!    `1` from `1.0` at all: both render `"eka yuro"`. Corpus-pinned.
//! 4. **Cents are truncated to two digits, never rounded, and right-padded.**
//!    `int(parts[1][:2].ljust(2, "0"))` — so `0.5` is **50** cents (pad), and
//!    `1.239` would be 23 cents (truncate, not 24). The `[:2]` also means a
//!    3-decimal currency's mils are silently cut to cents.
//! 5. **An unknown currency code does not raise — it becomes USD.**
//!    `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`. Python
//!    dicts have preserved insertion order since 3.7 and the literal is written
//!    USD, EUR, INR, so `values()[0]` is USD's. Hence `currency="JPY"` and
//!    `currency="KWD"` both render "ḍolara"/"senta" — and, per the missing
//!    `CURRENCY_PRECISION` read, JPY still shows cents and KWD still uses a
//!    100 divisor rather than 1 and 1000 respectively. All corpus-pinned.
//! 6. **Singular vs plural is a no-op.** Every form pair is identical
//!    (`("yuro", "yuro")`), so the `left != 1` / `right != 1` selects that
//!    Python performs cannot affect the output. Preserved verbatim anyway.
//!
//! # Error variants
//!
//! The four integer modes raise nothing: there is no `MAXVAL` check, every list
//! index is guarded by its band, and `to_ordinal`/`to_ordinal_num`/`to_year`
//! are total over the integers.
//!
//! The currency surface adds exactly two reachable raises:
//!   * `to_cheque` with a code outside {USD, EUR, INR} → `NotImplemented`,
//!     message `Currency code "GBP" not implemented for "Num2Word_PLI"`.
//!   * `to_currency` → `Value` (Python `ValueError`) when `int()` is handed a
//!     non-numeric token. Unreachable for any value `str()` renders in plain
//!     decimal notation; see the exponent-notation note on [`LangPli::to_currency`].

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
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

/// `self.negword` — note the trailing space, which is part of the Python value.
const NEGWORD: &str = "ūna ";
/// `self.pointword`.
const POINTWORD: &str = "bindu";

/// The word `_int_to_word` returns for 0. Not a member of `ones`.
const ZERO_WORD: &str = "suñña";

/// `self.ones`. Index 0 is `""` and is unreachable from `_int_to_word`
/// (0 is intercepted by the `ZERO_WORD` guard; every other read is guarded by
/// a non-zero digit). The float path *does* index `ONES[0]` per fractional
/// digit, but Python's `self.ones[int(digit)] or "suñña"` swaps its `""` for
/// `ZERO_WORD` before it is emitted, so its value never reaches the output —
/// [`LangPli::cardinal_from_numstr`] mirrors that `or`.
const ONES: [&str; 10] = [
    "", "eka", "dvi", "ti", "catu", "pañca", "cha", "satta", "aṭṭha", "nava",
];

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247; Pali
/// was not in that issue's list but had the same bug). Python joined tens
/// and units with " ca " ("vīsati ca ti" for 23); Pali writes each as one
/// compound, unit first: ekavīsati, tevīsati, ekūnatiṃsa. From the Pali
/// number tables of Ānandajoti (ancient-buddhist-texts.net, also reproduced
/// on obo.genaud.net) and Duroiselle's grammar (tipitaka.net, ch. 8), in the
/// module's -a stem spellings (tiṃsa, cattāḷīsa); 1..=19 and the round tens
/// are the module's own words. Index 0 is never read.
const BELOW_HUNDRED: [&str; 100] = [
    "", "eka", "dvi", "ti", "catu", "pañca", "cha", "satta", "aṭṭha", "nava", // 0..9
    "dasa", "ekādasa", "dvādasa", "terasa", "cuddasa", "pannarasa", "soḷasa", "sattarasa", "aṭṭhārasa", "ekūnavīsati", // 10..19
    "vīsati", "ekavīsati", "dvāvīsati", "tevīsati", "catuvīsati", "pañcavīsati", "chabbīsati", "sattavīsati", "aṭṭhavīsati", "ekūnatiṃsa", // 20..29
    "tiṃsa", "ekatiṃsa", "dvattiṃsa", "tettiṃsa", "catuttiṃsa", "pañcatiṃsa", "chattiṃsa", "sattatiṃsa", "aṭṭhatiṃsa", "ekūnacattāḷīsa", // 30..39
    "cattāḷīsa", "ekacattāḷīsa", "dvecattāḷīsa", "tecattāḷīsa", "catucattāḷīsa", "pañcacattāḷīsa", "chacattāḷīsa", "sattacattāḷīsa", "aṭṭhacattāḷīsa", "ekūnapaññāsa", // 40..49
    "paññāsa", "ekapaññāsa", "dvepaññāsa", "tepaññāsa", "catupaññāsa", "pañcapaññāsa", "chappaññāsa", "sattapaññāsa", "aṭṭhapaññāsa", "ekūnasaṭṭhi", // 50..59
    "saṭṭhi", "ekasaṭṭhi", "dvesaṭṭhi", "tesaṭṭhi", "catusaṭṭhi", "pañcasaṭṭhi", "chasaṭṭhi", "sattasaṭṭhi", "aṭṭhasaṭṭhi", "ekūnasattati", // 60..69
    "sattati", "ekasattati", "dvesattati", "tesattati", "catusattati", "pañcasattati", "chasattati", "sattasattati", "aṭṭhasattati", "ekūnāsīti", // 70..79
    "asīti", "ekāsīti", "dvāsīti", "tiyāsīti", "caturāsīti", "pañcāsīti", "chāsīti", "sattāsīti", "aṭṭhāsīti", "ekūnanavuti", // 80..89
    "navuti", "ekanavuti", "dvenavuti", "tenavuti", "catunavuti", "pañcanavuti", "channavuti", "sattanavuti", "aṭṭhanavuti", "ekūnasata", // 90..99
];

/// `self.hundred`.
const HUNDRED: &str = "sata";
/// `self.thousand`.
const THOUSAND: &str = "sahassa";
/// `self.million` — "ten lakh", hyphenated, exactly as Python spells it.
const MILLION: &str = "dasa-lakkha";
/// 10^7 (gladiaio/num2words2#147). Python stopped at 10^9 and returned the
/// digits; koṭi is the Pali ten million, and the Pali tables build the next
/// steps from it (koṭisata 10^9, koṭisahassa 10^10, koṭisatasahassa
/// 10^12), so the port spells the koṭi count from 10^7 up.
const KOTI: &str = "koṭi";

/// The Python class name, for `to_cheque`'s `NotImplementedError` message.
const LANG_NAME: &str = "Num2Word_PLI";

/// `Num2Word_PLI.to_currency`'s own `separator=" "` default.
///
/// PLI narrows `Num2Word_Base`'s `","`, and every currency corpus row is
/// generated by `num2words(v, lang="pli", to="currency", currency=C)` with
/// **no** separator kwarg — so `" "` is what each expected string carries
/// ("dvādasa yuro tiṃsa ca catu senta", not "dvādasa yuro,tiṃsa ca catu senta").
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s `separator=","` default, used here as the
/// "caller did not pass one" sentinel — see [`LangPli::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// The key whose value is `list(CURRENCY_FORMS.values())[0]` — the fallback an
/// unknown code lands on (quirk 5). Python's dict literal is written USD, EUR,
/// INR, and dicts have preserved insertion order since 3.7, so the first value
/// is USD's.
const FALLBACK_CURRENCY: &str = "USD";

pub struct LangPli {
    /// `self.exclude_title`. Dead in practice (`is_title` is False and PLI's
    /// `to_cardinal` never calls `title()`), but carried for parity.
    exclude_title: Vec<String>,
    /// `Num2Word_PLI.CURRENCY_FORMS`, built once in [`LangPli::new`] and never
    /// per call — rebuilding it per call is what made an earlier revision of
    /// this port 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(CURRENCY_FORMS.values())[0]`, resolved once. A `HashMap` is enough
    /// for lookup but cannot answer "the first value", so the USD entry is
    /// cloned out here rather than making the map an ordered one — it is two
    /// short strings, and holding it by value keeps the struct free of a
    /// self-referential borrow. See [`FALLBACK_CURRENCY`].
    fallback_forms: CurrencyForms,
}

impl Default for LangPli {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPli {
    pub fn new() -> Self {
        // `Num2Word_PLI.CURRENCY_FORMS`, verbatim and in the literal's order.
        // Built once here, never per call. Note every pair is (form, form):
        // PLI carries two forms per side like Python does, but they are equal,
        // so the singular/plural select is a no-op (quirk 6). The arity is
        // still 2, because `to_currency` indexes `cr1[1]`/`cr2[1]` directly.
        let mut currency_forms: HashMap<&'static str, CurrencyForms> = HashMap::new();
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["ḍolara", "ḍolara"], &["senta", "senta"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["yuro", "yuro"], &["senta", "senta"]),
        );
        currency_forms.insert(
            "INR",
            CurrencyForms::new(&["rūpa", "rūpa"], &["paisā", "paisā"]),
        );

        // `list(CURRENCY_FORMS.values())[0]` resolved once — see quirk 5.
        let fallback_forms = currency_forms
            .get(FALLBACK_CURRENCY)
            .expect("FALLBACK_CURRENCY is inserted above")
            .clone();

        LangPli {
            exclude_title: ["ca", "bindu", "ūna"].iter().map(|s| s.to_string()).collect(),
            currency_forms,
            fallback_forms,
        }
    }

    /// `int_to_word` behind [`maxval_ceiling`]. Every entry point that hands
    /// over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }


    /// Port of `Num2Word_PLI._int_to_word`.
    ///
    /// Infallible by construction — see bug 1 in the module docs: the ladder's
    /// fallthrough returns `str(number)` rather than raising, so no input of
    /// any magnitude can fail.
    ///
    /// Only ever called with **non-negative** values: `to_cardinal` strips the
    /// sign before delegating here, and the recursive calls below all pass a
    /// quotient or remainder of a non-negative value. (`to_currency` is the one
    /// caller that could pass something else, and it is out of scope.) The
    /// `div_mod_floor` calls therefore match Python's `divmod` exactly, with no
    /// negative-operand divergence to worry about.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // Band guards make every index below provably in range, so the
        // `to_usize` unwraps cannot fire. Bands are checked in Python's order.
        if number < &BigInt::from(100) {
            // Python: teens, then `tens[t] + " ca " + ones[o]` (fixed, #247).
            let i = number.to_usize().expect("guarded: number < 100");
            return BELOW_HUNDRED[i].to_string();
        }

        if number < &BigInt::from(1000) {
            let (h, r) = number.div_mod_floor(&BigInt::from(100));
            let h = h.to_usize().expect("guarded: number < 1000 => h < 10");
            // Python: base = self.ones[h] + " " + self.hundred
            let base = format!("{} {}", BELOW_HUNDRED[h], HUNDRED);
            // Python: base + (" ca " + self._int_to_word(r) if r else "")
            return if !r.is_zero() {
                format!("{} ca {}", base, self.int_to_word(&r))
            } else {
                base
            };
        }

        if number < &BigInt::from(1_000_000) {
            let (t, r) = number.div_mod_floor(&BigInt::from(1000));
            let base = format!("{} {}", self.int_to_word(&t), THOUSAND);
            // NB: plain space here, not " ca " — see the module docs.
            return if !r.is_zero() {
                format!("{} {}", base, self.int_to_word(&r))
            } else {
                base
            };
        }

        if number < &BigInt::from(10_000_000) {
            let (m, r) = number.div_mod_floor(&BigInt::from(1_000_000));
            let base = format!("{} {}", self.int_to_word(&m), MILLION);
            return if !r.is_zero() {
                format!("{} {}", base, self.int_to_word(&r))
            } else {
                base
            };
        }

        // Python: dasa-lakkha up to 10^9, then `return str(number)` (bug 1,
        // fixed in #147). From 10^7 the koṭi count is spelled out, up to the
        // 10^14 ceiling that `checked_int_to_word` enforces.
        let (k, r) = number.div_mod_floor(&BigInt::from(10_000_000));
        let base = format!("{} {}", self.int_to_word(&k), KOTI);
        if !r.is_zero() {
            format!("{} {}", base, self.int_to_word(&r))
        } else {
            base
        }
    }

    /// The string-processing core of `Num2Word_PLI.to_cardinal`, driven by a
    /// value already rendered as Python's `str(number)` (see
    /// [`LangPli::to_cardinal_float`] for how the two `FloatValue` arms produce
    /// that string). This is a direct transcription of the non-integer half of
    /// the Python method:
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "suñña")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// It recurses once on a leading `"-"` exactly as Python does — the string
    /// can never carry two minus signs, so a single level suffices.
    fn cardinal_from_numstr(&self, n: &str) -> Result<String> {
        // n = str(number).strip()
        let n = n.trim();

        // if n.startswith("-"): return (negword + to_cardinal(n[1:])).strip()
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_numstr(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        // if "." in n: split on the FIRST "." — str.split(".", 1). The dot is
        // ASCII, so byte-slicing here is safe regardless of what surrounds it.
        if let Some(dot) = n.find('.') {
            let left = &n[..dot];
            let right = &n[dot + 1..];

            // int(left) — Python raises ValueError on a non-numeric token, and
            // so does BigInt::from_str (mapped to N2WError::Value). For every
            // str(float)/str(Decimal) `left` is a plain digit run, so this never
            // fires in practice; str(number) always yields a digit before ".".
            let left = BigInt::from_str(left).map_err(|e| N2WError::Value(e.to_string()))?;

            // ret = self._int_to_word(int(left)) + " " + self.pointword
            let mut ret = format!("{} {}", self.checked_int_to_word(&left)?, POINTWORD);

            // for digit in right: ret += " " + (self.ones[int(digit)] or "suñña")
            // ONES[0] is "" (falsy), so Python's `or` swaps in ZERO_WORD; every
            // other digit uses ONES[d]. Digits are ASCII, indexed by value.
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
                })? as usize;
                let word = ONES[d];
                ret.push(' ');
                ret.push_str(if word.is_empty() { ZERO_WORD } else { word });
            }

            return Ok(ret.trim().to_string());
        }

        // return self._int_to_word(int(n)) — no fractional part. Unreachable for
        // the corpus (str(number) here always contains "."), but kept for parity
        // with Python, which would reach it if str(number) ever lacked a dot.
        let bi = BigInt::from_str(n).map_err(|e| N2WError::Value(e.to_string()))?;
        self.checked_int_to_word(&bi)
    }
}

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. `Num2Word_PLI.to_cardinal` is driven
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

/// The exclusive ceiling (gladiaio/num2words2#147). The next Pali step after
/// koṭi is pakoṭi (10^14, koṭi of koṭis), which the port does not use; from
/// there the koṭi count would itself need koṭi, so it raises
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(14))
}

impl Lang for LangPli {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "USD"
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
        "bindu"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_PLI.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and branches on a leading `"-"` and on a
    /// `"."`; `str(int)` never contains a `"."`, so integers always fall
    /// through to `self._int_to_word(int(n))`. The float/Decimal branch
    /// (`pointword`, per-digit decimals) is ported separately in
    /// [`LangPli::to_cardinal_float`], which reconstructs `str(number)` and
    /// shares the same string-processing core via
    /// [`LangPli::cardinal_from_numstr`].
    ///
    /// The negative branch recurses through `to_cardinal` on the sign-stripped
    /// *string*, then `.strip()`s `negword + result`. Since `negword` is
    /// `"ūna "` (trailing space) and `_int_to_word` never yields leading or
    /// trailing whitespace, the strip is a no-op in practice — but it is
    /// applied here anyway to mirror the original. One level of recursion
    /// suffices: `str(BigInt)` cannot begin with two minus signs.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.to_cardinal(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        self.checked_int_to_word(value)
    }

    /// Port of `Num2Word_PLI.to_ordinal`.
    ///
    /// Only the exact value 1 is special-cased; everything else — including 0
    /// and every negative — is `to_cardinal(n) + "ma"`. So `to_ordinal(0)` is
    /// `"suññama"` and `to_ordinal(-1)` is `"ūna ekama"`, where the suffix
    /// attaches to the final word rather than the phrase. Total over the
    /// integers: PLI raises nothing here (contrast `lang_PL`, which crashes on
    /// 0 and on every negative).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("paṭhama".to_string());
        }
        Ok(format!("{}ma", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_PLI.to_ordinal_num`: `str(number) + "ma"`.
    ///
    /// No sign handling and no 1 special-case, so `-1` → `"-1ma"` and
    /// `1` → `"1ma"` (not `"paṭhama"`).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}ma", value))
    }

    /// Port of `Num2Word_PLI.to_year`: delegates straight to `to_cardinal`,
    /// ignoring its `longval` parameter. No era suffix, no century pairing —
    /// `to_year(-500)` is just `"ūna pañca sata"`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the float/Decimal branch of `Num2Word_PLI.to_cardinal`.
    ///
    /// PLI overrides `to_cardinal` (not `to_cardinal_float`) and handles
    /// non-integers **inline**, by stringifying the number and walking its
    /// digits — it never calls `base.float2tuple`. So the default float path
    /// (`floatpath::default_to_cardinal_float`, which *does* use `float2tuple`)
    /// is wrong here on two counts: it would render a whole float like `1.0`
    /// off Rust's `{}` (which drops the `.0`), and it re-derives the fractional
    /// digits through the binary-artefact heuristic rather than from
    /// `str(number)`. This override reproduces PLI's algorithm exactly by
    /// reconstructing `str(number)` and delegating to
    /// [`LangPli::cardinal_from_numstr`].
    ///
    /// Reconstructing `str(number)`:
    ///   * **Decimal arm** — `str(Decimal)` keeps scale, so trailing zeros are
    ///     significant (`str(Decimal("1.10")) == "1.10"`). `BigDecimal`'s
    ///     `Display` reproduces that plain-decimal form for every scale the
    ///     corpus exercises, and it stays in arbitrary precision — never an
    ///     f64 cast — so `98746251323029.99` keeps all its digits (issue #603).
    ///   * **Float arm** — `str(float)` is the shortest round-trip string, with
    ///     exactly `precision` fractional digits (`str(1.0) == "1.0"`, one
    ///     digit). Rust's `{}` drops the `.0` on whole floats, so this formats
    ///     to `precision` places instead; correct-rounding to `precision`
    ///     digits reproduces the shortest repr for every plain-decimal value
    ///     (verified byte-for-byte against the interpreter across the trace
    ///     set, incl. `2.675`, `1.005`, `0.1+0.2`).
    ///
    /// `precision_override` is deliberately ignored: PLI's `to_cardinal` never
    /// reads `self.precision` (it iterates `str(number)`'s own characters), so
    /// `num2words(x, lang="pli", precision=k)` cannot change the output —
    /// unlike the base float path, which honours it. Interpreter-confirmed.
    ///
    /// Exponent-notation floats (`1e-07`, `1e+21`) are out of scope and the one
    /// known divergence: Python's `str` emits an `"e"` and no `"."`, so it
    /// reaches `int("1e-07")` → `ValueError`, whereas the `{:.precision}` form
    /// here renders a plain decimal and succeeds. No such value is in the
    /// corpus or reachable trace set; see `concerns`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // PLI never consults self.precision on this path — see the doc above.
        let _ = precision_override;

        let n = match value {
            // `str(Decimal)` — scale preserved, `E+n` form for a positive
            // exponent (`Decimal("1E+2")` reaches `int("1E+2")` -> ValueError).
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
            // `str(float)` == repr(float), exponent form included — so 1e16
            // reaches `int("1e+16")` and raises ValueError, as Python does.
            FloatValue::Float { value, .. } => python_repr_f64(*value),
        };
        self.cardinal_from_numstr(&n)
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "pañca bindu suñña", `-0.0` ->
    /// "ūna suñña bindu suñña", `Decimal("5.00")` -> "pañca bindu suñña
    /// suñña". The base default's whole-value shortcut must not fire here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`:
    ///
    /// ```text
    /// if number == 1:
    ///     return "paṭhama"
    /// return self.to_cardinal(number) + "ma"
    /// ```
    ///
    /// `number == 1` is a *numeric* comparison, so `1.0`, `Decimal("1")` and
    /// `Decimal("1.00")` all short-circuit to "paṭhama" — `as_whole_int()`
    /// reproduces that. Everything else runs the string cardinal and glues
    /// "ma" on; exponent forms raise ValueError before the suffix.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int() == Some(BigInt::from(1)) {
            return Ok("paṭhama".to_string());
        }
        Ok(format!("{}ma", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "ma"`. `repr_str` is
    /// the binding's Python `str(value)`, so exponent forms echo verbatim:
    /// `to_ordinal_num(1e16)` == "1e+16ma" — and there is **no** `== 1`
    /// special case here (`1.0` -> "1.0ma").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}ma", repr_str))
    }

    /// `to_year(float/Decimal)`: PLI's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// Base's `str_to_number` parses "Infinity" *successfully*; PLI's
    /// ValueError comes later, from `int("Infinity")` inside `to_cardinal`.
    /// The Rust dispatcher hard-codes Base's OverflowError for
    /// `ParsedNumber::Inf`, which PLI never executes — so Inf parses punt to
    /// the Python fallback, which reproduces every mode byte for byte. NaN
    /// stays on the dispatcher path (its hard-coded ValueError matches).
    /// Same shape as `lang_as.rs`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity`. Base's dispatcher would raise
    /// OverflowError (`int(Decimal Inf)`), but PLI never runs that: it
    /// stringifies the value and calls `int("Infinity")` inside `to_cardinal`,
    /// which raises **ValueError**. `to_cardinal`, `to_ordinal` (which calls
    /// `to_cardinal`) and `to_year` (ditto) all reach it. Only `to_ordinal_num`
    /// escapes — it is `str(number) + "ma"` with no `int()`, so
    /// `Decimal('Infinity')` renders "Infinityma" / "-Infinityma" verbatim.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => {
                let s = if negative { "-Infinity" } else { "Infinity" };
                Ok(format!("{}ma", s))
            }
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
        }
    }

    /// `Decimal('NaN')`. `to_cardinal`/`to_ordinal`/`to_year` hit
    /// `int("NaN")` -> ValueError (base's default here is already ValueError,
    /// but the `to_ordinal_num` escape hatch — `str(number) + "ma"` == "NaNma"
    /// — must be served too rather than raising).
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaNma".to_string()),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
        }
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, as `to_cheque` interpolates it into its
    /// `NotImplementedError`.
    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `Num2Word_PLI.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately a **strict** lookup that returns `None` for an unknown
    /// code. This hook feeds the inherited `to_cheque`, which in Python does
    /// `CURRENCY_FORMS[currency]` inside a `try/except KeyError` and re-raises
    /// as `NotImplementedError`. `to_currency` must *not* use this hook — it
    /// does a `.get(..., default)` and falls back to USD instead (quirk 5), so
    /// it consults `self.currency_forms`/`self.fallback_forms` directly.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_PLI.pluralize`.
    ///
    /// PLI overrides `Num2Word_Base.pluralize` (which is abstract and raises),
    /// so the hook is implemented here rather than left at the raising default.
    /// It is nonetheless **unreachable** on both currency paths: `to_currency`
    /// picks forms inline with `cr1[1] if left != 1 else cr1[0]`, and
    /// `to_cheque` takes `cr1[-1]` unconditionally. Ported for parity, because
    /// it is a real method a caller can invoke.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        // `if not forms: return ""`
        if forms.is_empty() {
            return Ok(String::new());
        }
        // `return forms[0] if n == 1 else forms[-1]`
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Port of `Num2Word_PLI.to_currency` — a wholesale replacement for
    /// `Num2Word_Base.to_currency`, sharing none of its machinery.
    ///
    /// Python:
    /// ```text
    /// is_negative = val < 0
    /// val = abs(val)
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
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
    /// # Why the int/float split collapses here
    ///
    /// `base.to_currency` branches on `isinstance(val, int)` to decide whether
    /// to render cents at all. PLI never does: it stringifies first, then
    /// tests the *cent count* (`if cents and right`). `1` gives `str` `"1"`
    /// (no `parts[1]`, `right = 0`) and `1.0` gives `str` `"1.0"`
    /// (`parts[1] == "0"`, so `right = int("00") = 0`) — both falsy, both
    /// `"eka yuro"`. So `CurrencyValue::Int` vs `Decimal` is still honoured
    /// exactly (it changes the string being split), it simply cannot change
    /// the output for this language. Corpus-pinned on `1` and `1.0`.
    ///
    /// # `str(val)` fidelity
    ///
    /// `CurrencyValue::Decimal` carries `str(value)` re-parsed into a
    /// `BigDecimal`, and this reproduces Python's `str(val)` via `Display`.
    /// That roundtrip is exact for plain decimal notation — `BigDecimal`
    /// preserves scale, so `"1.0"`, `"0.5"`, `"0.01"` and `"12.34"` all render
    /// back byte-identically, which is every value the corpus exercises. It is
    /// *not* exact for values `repr(float)` renders in exponent notation
    /// (`1e-07`): Python would `int("1e-07")` and raise `ValueError`, and this
    /// code also fails to parse and raises `N2WError::Value` — same variant,
    /// different message text. See `concerns`.
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
        // Python accepts `adjective` and never reads it: PLI has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency.
        let _ = adjective;

        // The trait passes whatever the caller gave, and the dispatcher's
        // "caller said nothing" value is `Num2Word_Base`'s `","` — not PLI's
        // own `" "`. Treat it as the sentinel it is. (The trait has no
        // per-language default-separator hook; see `concerns`.)
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `is_negative = val < 0` is evaluated *before* `val = abs(val)`, and
        // the abs is unconditional, so the string never carries a sign.
        let is_negative = val.is_negative();
        let s = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `str(val).split(".")`: take parts[0] and parts[1]. Splitting on every
        // dot rather than just the first matters only for input Python could
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
        // `[:2]` truncates and `ljust` pads on the right, so "5" -> "50"
        // (0.5 is 50 cents) and "01" -> "01" (0.01 is 1 cent). Sliced by
        // chars, never bytes.
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
        // code becomes USD rather than an error (quirk 5). Not `currency_forms()`,
        // which is the strict lookup `to_cheque` needs.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // `self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 3).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // `result = self.negword + result` — raw, keeping the trailing space
        // baked into "ūna ". Base would strip and re-add it; PLI does not.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input — `int_to_word`
        // never returns padding and no currency form is blank — but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}
