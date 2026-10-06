//! Port of `lang_EN_AERO.py` → `Num2Word_EN_AERO_NATO` (key `"en_Aero_NATO"`).
//!
//! ICAO/aviation English, NATO profile. Registry check: `__init__.py` maps
//! `"en_Aero_NATO"` → `lang_EN_AERO.Num2Word_EN_AERO_NATO()`, a subclass of
//! `Num2Word_EN_AERO` whose only delta is `PROFILE = "NATO"`. All five entries
//! in the module's `_PROFILES` table (ICAO/FAA/USN/US_ARMY/NATO) are bound to
//! the *same* `_ICAO_DIGITS` dict, `"decimal"` and `"minus"` — the Python
//! comments say so explicitly ("Today the four profiles below are
//! intentionally identical"). So NATO is byte-for-byte identical to plain
//! `en_Aero_ICAO`, and this file hardcodes the ICAO table rather than modelling
//! the profile indirection. The `ValueError` for an unknown profile is
//! unreachable: `PROFILE = "NATO"` is a class constant present in `_PROFILES`,
//! and the Rust API exposes no `profile=` constructor argument.
//!
//! Shape: **self-contained**. `Num2Word_EN_AERO.to_cardinal` is overridden
//! outright — it stringifies the input and emits one word per digit, never
//! touching `cards`/`splitnum`/`merge`. Consequently there is **no overflow
//! check** on the cardinal/year path: `Num2Word_EN`'s `MAXVAL` is inherited but
//! never consulted, so 10^600 renders as 601 digit-words instead of raising
//! `OverflowError`. The ordinal path *does* overflow, because it delegates
//! (see below).
//!
//! # The sibling-instance handshake
//!
//! `Num2Word_EN_AERO.__init__` stashes `self._english = Num2Word_EN()` and
//! routes `to_ordinal`, `to_currency` and `to_cheque` (plus the out-of-scope
//! `to_fraction`) through it. This is *delegation*, not cross-call mutable state:
//! the sibling is constructed once and never mutated, so the Rust path stays
//! stateless. Reproduced faithfully by holding a [`LangEn`] field and calling
//! its trait methods — `LangEn::new()` is the exact analogue of a fresh
//! `Num2Word_EN()`, since `Num2Word_EN_AERO.__init__` calls `super().__init__()`
//! and adds no `setup()` delta.
//!
//! The stated reason for the handshake (per the module docstring) is that a
//! digit-by-digit cardinal leaking into `Num2Word_EN.to_ordinal`'s
//! last-word-substitution builder would yield "treeth" for `to_ordinal(3)`
//! instead of "third". Corpus row `{"to": "ordinal", "arg": "3"}` → `"third"`
//! confirms the delegation is live.
//!
//! # Inherited / overridden method map
//!
//!   * `to_cardinal`    — AERO override, digit-by-digit. Ignores EN entirely.
//!   * `to_year`        — AERO override, `return self.to_cardinal(value)`.
//!     Discards `Num2Word_EN.to_year` wholesale, so there is no "nineteen
//!     eighty-four" pairing, no `oh-` infix, and **no `BC` suffix**: negatives
//!     fall through to the cardinal's `"minus"` prefix. Corpus:
//!     `year(-500)` → `"minus fife zero zero"`, not `"five hundred BC"`.
//!   * `to_ordinal`     — AERO override → `self._english.to_ordinal(value)`,
//!     i.e. plain `Num2Word_EN` (which *does* range-check via its cardinal).
//!   * `to_ordinal_num` — AERO override: `verify_ordinal` then `str(int(value))`.
//!     Note this drops `Num2Word_EN.to_ordinal_num`'s `"%s%s" % (value, ord[-2:])`
//!     suffix logic, so the output is bare digits: `0` → `"0"`, not `"0th"`.
//!     It also skips EN's ordinal call, hence **no overflow check** here either.
//!   * `verify_ordinal` — inherited unchanged from `Num2Word_Base`.
//!   * `to_currency`    — AERO override → `self._english.to_currency(*a, **kw)`.
//!   * `to_cheque`      — AERO override → `self._english.to_cheque(*a, **kw)`.
//!     Both are pure delegation, so money is read as plain English composite
//!     cardinals, never digit-by-digit: `cheque:USD 1234.56` → "ONE THOUSAND,
//!     TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS".
//!   * `CURRENCY_FORMS`/`CURRENCY_ADJECTIVES`/`CURRENCY_PRECISION`/`pluralize`
//!     — inherited from `Num2Word_EN`/`Num2Word_EUR`. Present on the AERO
//!     instance and modelled by delegation, though unreachable on the currency
//!     path (the delegate resolves its own).
//!   * `_money_verbose`/`_cents_verbose` — inherited from `Num2Word_Base`, i.e.
//!     `self.to_cardinal(number)`, which on an AERO receiver is the
//!     digit-by-digit override. Left at the trait default for that reason; see
//!     the note in the impl block.
//!   * `cards`/`maxval`/`merge` — inherited from `Num2Word_EN` and populated by
//!     its `setup()`, but unreachable in all four in-scope modes. Delegated to
//!     the sibling below so the engine path, if ever driven, matches Python's
//!     inherited state exactly.
//!
//! # Faithfully reproduced Python oddities
//!
//! 1. The respellings are the spec, not typos: `1` → "wun", `2` → "too",
//!    `3` → "tree", `4` → "fower", `5` → "fife", `8` → "ait", `9` → "niner".
//!    `0`/`6`/`7` are unrespelled ("zero"/"six"/"seven"). Kept verbatim.
//! 2. `to_year` accepting negatives while `to_ordinal` rejects them is a real
//!    asymmetry — `to_year` never calls `verify_ordinal`.
//! 3. `_digits_of` normalises `","`/`"_"` away and defaults an empty integer
//!    part to `"0"`. Both are dead code for integer input (`str(int)` never
//!    contains a separator and is never empty), so neither is modelled.
//!
//! # Error variants
//!
//! `to_ordinal` / `to_ordinal_num` on a negative → `N2WError::Type`
//! ("Cannot treat negative num %s as ordinal."), from `Num2Word_Base.
//! verify_ordinal`'s `if not abs(value) == value` arm. `to_ordinal` past
//! `Num2Word_EN`'s `MAXVAL` → `N2WError::Overflow`, raised inside the
//! delegate. `to_cardinal`/`to_year`/`to_ordinal_num` are total for integers.
//!
//! An unknown currency code → `N2WError::NotImplemented`, raised inside the
//! delegate and therefore naming **`Num2Word_EN`**, not this class:
//! `Currency code "XXX" not implemented for "Num2Word_EN"`. Confirmed against
//! the Python. No corpus row exercises it — all 108 currency and 9 cheque rows
//! for this language are `ok: true`.

use crate::base::{Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::lang_en::LangEn;
use crate::lang_en_aero::{
    python_float_repr,
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::Signed;

/// Port of `_ICAO_DIGITS`, indexed by digit value rather than by `"0"`..`"9"`
/// string keys. `_PROFILES["NATO"]` binds this same dict.
const ICAO_DIGITS: [&str; 10] = [
    "zero",  // "0"
    "wun",   // "1"
    "too",   // "2"
    "tree",  // "3"
    "fower", // "4"
    "fife",  // "5"
    "six",   // "6"
    "seven", // "7"
    "ait",   // "8"
    "niner", // "9"
];

/// Port of `_ICAO_MINUS`, via `self._minus_word`.
const MINUS_WORD: &str = "minus";

/// Port of `_ICAO_DECIMAL`, via `self._decimal_word`. This is what
/// `Num2Word_EN_AERO.to_cardinal` emits as the decimal mark for non-integer
/// input — **not** the inherited `pointword` ("point"), which AERO never
/// consults because it overrides `to_cardinal` outright. Corpus:
/// `cardinal 0.5` → `"zero decimal fife"`.
const DECIMAL_WORD: &str = "decimal";

pub struct LangEnAeroNato {
    /// Port of `self._english = Num2Word_EN()`. Constructed once, never
    /// mutated; `to_ordinal` routes through it.
    english: LangEn,
}

impl Default for LangEnAeroNato {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnAeroNato {
    pub fn new() -> Self {
        LangEnAeroNato {
            english: LangEn::new(),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// Python's first arm (`if not value == int(value)` → `errmsg_floatord`)
    /// is unreachable for `BigInt`, so only the negative arm is modelled.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

/// Port of `Num2Word_EN_AERO._digits_of`, for the two numeric arms the float
/// path can carry. Returns `(is_negative, integer-part-string,
/// fractional-part-string)`, each already reduced to decimal digits.
///
/// Python's `_digits_of` branches on the *input type*. The dispatcher has
/// already made that split for us:
///
///   * `float`   → Python does `s = str(value)`. `FloatValue::Float` carries
///     the raw f64 plus the repr-derived precision (`abs(Decimal(str(value))
///     .as_tuple().exponent)`), so `format!("{:.p$}", value)` reconstructs
///     `str(value)` exactly: Rust's `{:.N}` and CPython's `repr`/`str` are both
///     correctly-rounded shortest formatting of the *same* IEEE-754 double, so
///     their digit strings coincide (verified equal over 200k random
///     non-scientific values). The f64 artefact is therefore preserved, not
///     repaired: `2.675` formats to `"2.675"` because the stored double is
///     `2.67499999999999982…`, which rounds up at 3 places — the same reason
///     `str(2.675) == "2.675"`. The sign, including negative zero
///     (`str(-0.0) == "-0.0"`), rides through the format output verbatim.
///
///   * `Decimal` → Python does `s = format(value, "f")`, exact fixed-point.
///     Reconstructed with exactly `precision` fractional digits using the
///     repr-derived precision — the same field `base.float2tuple`'s Decimal arm
///     trusts — rather than `BigDecimal`'s internal scale, so a trailing-zero
///     Decimal like `1.10` (precision 2) keeps both fractional digits
///     regardless of how `from_str` normalised it.
///
/// Python's `.replace(",", "").replace("_", "")` and the empty-int-part
/// `"0"` fallback are dead for these two arms — `str(float)` /
/// `format(Decimal, "f")` never emit a separator and never start with `"."` —
/// so they are not modelled, matching the note already made for the integer
/// path.
fn digits_of(v: &FloatValue) -> (bool, String, String) {
    match v {
        FloatValue::Float { value, precision } => {
            // `python_float_repr` also covers CPython's scientific range
            // (|v| >= 1e16 or < 1e-4): `str(1e16)` is "1e+16", which the
            // digit reader then takes lexically — "wun wun six".
            let s = python_float_repr(*value, *precision);
            let (is_negative, body) = match s.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, s.as_str()),
            };
            match body.split_once('.') {
                Some((int_part, frac_part)) => {
                    (is_negative, int_part.to_string(), frac_part.to_string())
                }
                None => (is_negative, body.to_string(), String::new()),
            }
        }
        FloatValue::Decimal { value, .. } => {
            let is_negative = value.is_negative();
            // `format(value, "f")` is driven by the Decimal's own exponent,
            // *not* by the repr-derived `precision` (which is its absolute
            // value and cannot tell `1E+2` from `1.00`). A non-positive scale
            // — the source string used exponent form, `Decimal("1E+2")` —
            // expands to trailing zeros with NO fractional part: format 'f'
            // gives "100", never "100.00". A positive scale keeps exactly
            // that many fractional digits, trailing zeros included
            // (`Decimal("1.10")` → "1.10"). A zero coefficient collapses to
            // a bare "0" whatever the exponent (`format(Decimal("0E+2"),
            // "f")` == "0"), matching CPython.
            let (digits, scale) = value.abs().as_bigint_and_exponent();
            if scale <= 0 {
                let s = if num_traits::Zero::is_zero(&digits) {
                    "0".to_string()
                } else {
                    format!("{}{}", digits, "0".repeat((-scale) as usize))
                };
                (is_negative, s, String::new())
            } else {
                let mut s = digits.to_string();
                let p = scale as usize;
                // Guarantee at least one integer digit before the split.
                while s.len() <= p {
                    s.insert(0, '0');
                }
                let split = s.len() - p;
                (is_negative, s[..split].to_string(), s[split..].to_string())
            }
        }
    }
}

impl Lang for LangEnAeroNato {
    // cards/maxval/merge are inherited from Num2Word_EN via setup() and are
    // unreachable in every in-scope mode (to_cardinal is overridden). Delegated
    // rather than defaulted so the inherited state matches Python's.
    fn cards(&self) -> &Cards {
        self.english.cards()
    }

    fn maxval(&self) -> &BigInt {
        self.english.maxval()
    }

    /// `maxval()` is None (gladiaio/num2words2#159): the cardinal reads digit
    /// by digit with no ceiling, so the inherited `Num2Word_EN.MAXVAL` (10^306)
    /// is not reported — the cardinal happily converts past it.
    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        None
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        self.english.merge(l, r)
    }

    /// Inherited from `Num2Word_EN.setup` — note the trailing space, and note
    /// that AERO's `to_cardinal` ignores it in favour of `self._minus_word`
    /// ("minus", no trailing space).
    fn negword(&self) -> &str {
        "minus "
    }

    /// Inherited from `Num2Word_EN.setup`. AERO's own decimal mark is
    /// `self._decimal_word` ("decimal"), used only on the out-of-scope float
    /// path.
    fn pointword(&self) -> &str {
        "point"
    }

    fn exclude_title(&self) -> &[String] {
        self.english.exclude_title()
    }

    /// Port of `Num2Word_EN_AERO.to_cardinal` + `_digits_of`, integer path.
    ///
    /// Python: `s = str(value)`; strip a leading `"-"` into `is_negative`;
    /// emit `self._minus_word` then one `_digit_table[ch]` per `ch.isdigit()`.
    /// `str(int)` never contains `"."`, so the fractional branch (`"decimal"`
    /// + per-digit) is unreachable and out of scope.
    ///
    /// The `isdigit()` guard is preserved as an ASCII-digit filter. In Python
    /// it is a genuine guard — `str.isdigit()` accepts non-ASCII digits such as
    /// `"٥"`, which would then `KeyError` on `_digit_table` — but `BigInt`
    /// stringifies to `[-]?[0-9]+` only, so nothing is ever skipped here.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let s = value.to_string();
        let mut words: Vec<&str> = Vec::new();

        let digits = if let Some(stripped) = s.strip_prefix('-') {
            words.push(MINUS_WORD);
            stripped
        } else {
            &s[..]
        };

        for ch in digits.chars() {
            if let Some(d) = ch.to_digit(10) {
                words.push(ICAO_DIGITS[d as usize]);
            }
        }

        Ok(words.join(" "))
    }

    /// Port of `Num2Word_EN_AERO.to_ordinal`: `self._english.to_ordinal(value)`.
    ///
    /// Deliberately does *not* call our own `verify_ordinal` — the delegate
    /// runs `Num2Word_EN.to_ordinal`, which calls it first thing, so the
    /// TypeError for negatives surfaces from inside `LangEn` exactly as in
    /// Python. Any `OverflowError` past EN's MAXVAL also originates there.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.english.to_ordinal(value)
    }

    /// Port of `Num2Word_EN_AERO.to_ordinal_num`:
    /// `self.verify_ordinal(value); return str(int(value))`.
    ///
    /// Bare digits — this override discards `Num2Word_EN.to_ordinal_num`'s
    /// `"%s%s" % (value, self.to_ordinal(value)[-2:])` suffix, so `0` → `"0"`
    /// (not `"0th"`) and `1` → `"1"` (not `"1st"`). Because it never calls
    /// `to_ordinal`, no cardinal is built and no MAXVAL check runs: unlike
    /// `to_ordinal`, this is total for every non-negative integer.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// Port of `Num2Word_EN_AERO.to_year`: `return self.to_cardinal(value)`.
    ///
    /// Aviation reads years digit-by-digit, so `Num2Word_EN.to_year`'s
    /// century-pairing / `oh-` / `BC` logic is discarded entirely. `1971` →
    /// "wun niner seven wun", and `-500` → "minus fife zero zero" (the sign is
    /// handled by the cardinal's `"minus"`, never as a `BC` suffix).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries ----------------------------------------

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// AERO's `to_cardinal` reads the value's *string* form, so `5.0` keeps
    /// its ".0" tail ("fife decimal zero") — never the trait default's
    /// whole-value integer route.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` → `self._english.to_ordinal(value)`.
    ///
    /// The delegate's `verify_ordinal` polices the type: whole values
    /// ordinalise in plain English (`5.0` → "fifth", `-0.0` → "zeroth");
    /// fractional or negative values raise TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.english.ordinal_float_entry(value)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` — AERO's own,
    /// i.e. Base's — then `str(int(value))`. Bare truncated digits: `5.00` →
    /// "5", `1e+16` → "10000000000000000", `-0.0` → "0". Float-ness is
    /// checked before sign (`-1.5` raises the *float* message); `%s`
    /// interpolates `str(value)` = `repr_str`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            ))),
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )))
                } else {
                    Ok(i.to_string())
                }
            }
        }
    }

    /// `to_year(float/Decimal)` = `to_cardinal(value)` — the same lexical
    /// reading, ".0" tail included: `1971.0` → "wun niner seven wun decimal
    /// zero".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// Port of `to_fraction`: `return self._english.to_fraction(...)` —
    /// standard English forms ("one half", "three quarters", "twenty-two
    /// sevenths"), both halves rendered by the delegate, never
    /// digit-by-digit. `n/0`'s ZeroDivisionError also comes from inside it.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        self.english.to_fraction(numerator, denominator)
    }


    /// Port of the float/Decimal reach of `Num2Word_EN_AERO.to_cardinal`.
    ///
    /// AERO overrides `to_cardinal`, **not** `to_cardinal_float` — so in Python
    /// a non-integer never touches `Num2Word_Base.to_cardinal_float`/`float2tuple`
    /// at all; it flows straight into the digit-by-digit override. This method
    /// reproduces exactly that override for the `FloatValue` the dispatcher hands
    /// us: stringify (via [`digits_of`]), emit `"minus"` for a negative, one
    /// ICAO word per integer digit, then — only if there is a fractional part —
    /// `"decimal"` followed by one ICAO word per fractional digit.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is deliberately
    /// ignored: `Num2Word_EN_AERO.to_cardinal(self, value)` has no `precision`
    /// parameter, so the kwarg never changes AERO's output — the digit count is
    /// whatever `str(float)` / `format(Decimal, "f")` produced. Honouring it here
    /// would diverge from Python.
    ///
    /// Total for every corpus input; `digits_of` yields only ASCII digits, so the
    /// `to_digit` guard (mirroring Python's `ch.isdigit()`) never actually skips.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let (is_negative, int_part, frac_part) = digits_of(value);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(MINUS_WORD);
        }
        for ch in int_part.chars() {
            if let Some(d) = ch.to_digit(10) {
                words.push(ICAO_DIGITS[d as usize]);
            }
        }
        if !frac_part.is_empty() {
            words.push(DECIMAL_WORD);
            for ch in frac_part.chars() {
                if let Some(d) = ch.to_digit(10) {
                    words.push(ICAO_DIGITS[d as usize]);
                }
            }
        }
        Ok(words.join(" "))
    }

    // ---- currency ----------------------------------------------------
    //
    // `Num2Word_EN_AERO` defines exactly two currency methods, both pure
    // delegation to the sibling instance:
    //
    //     def to_currency(self, *args, **kwargs):
    //         return self._english.to_currency(*args, **kwargs)
    //     def to_cheque(self, *args, **kwargs):
    //         return self._english.to_cheque(*args, **kwargs)
    //
    // ICAO standardises digit-by-digit *transmission*, not money; a
    // digit-by-digit cardinal leaking into the currency builder would render
    // 1234.56 USD as "wun too tree fower dollars". So both modes hand off to a
    // plain `Num2Word_EN`. Corpus confirms the handoff is live:
    // `cheque:USD 1234.56` → "ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND
    // 56/100 DOLLARS", not the AERO reading.
    //
    // The delegation is load-bearing in a way that is easy to get wrong. Inside
    // `self._english.to_currency(...)`, **`self` is the sibling**, so *every*
    // hook the base implementation reaches resolves on `Num2Word_EN`:
    // `_money_verbose` → EN's composite cardinal, `pluralize` → EUR's rule,
    // `CURRENCY_FORMS` → EN's table, and `self.__class__.__name__` → the string
    // `"Num2Word_EN"`. Verified against the Python:
    //
    //     >>> Num2Word_EN_AERO_NATO().to_currency(1, "XXX")
    //     NotImplementedError: Currency code "XXX" not implemented for "Num2Word_EN"
    //
    // — *not* "Num2Word_EN_AERO_NATO", even though that is the receiver's class.
    // Re-implementing the hooks here and driving `currency::default_to_currency`
    // would silently produce the AERO class name in that message *and* a
    // digit-by-digit `money_verbose`. Delegating to `LangEn`'s own trait methods
    // reproduces both correctly by construction.
    //
    // No forms table is built here: `LangEn::new()` builds its table once, and
    // `self.english` holds it for this struct's lifetime, so these are plain
    // lookups on an already-constructed table — nothing is built per call.

    /// Models `self.__class__.__name__` for *this* receiver, which really is
    /// `"Num2Word_EN_AERO_NATO"`.
    ///
    /// Deliberately NOT the name that appears in a bad-currency-code message:
    /// that message is raised inside the delegate and carries `"Num2Word_EN"`
    /// (see the block comment above). Nothing on the delegated currency path
    /// reads this hook, so the two cannot conflict.
    fn lang_name(&self) -> &str {
        "Num2Word_EN_AERO_NATO"
    }

    /// `Num2Word_EN.__init__` mutates the `Num2Word_EUR.CURRENCY_FORMS` *class*
    /// dict in place (`self.CURRENCY_FORMS["EUR"] = ...` resolves to the class
    /// attribute and never rebinds), so the AERO instance and its sibling share
    /// one dict object — `a.CURRENCY_FORMS is a._english.CURRENCY_FORMS` is
    /// `True`. Delegating models that shared identity exactly.
    ///
    /// EN's overrides matter here: base EUR has `("euro", "euro")` and
    /// `("pound sterling", "pounds sterling")`, while EN replaces them with
    /// `("euro", "euros")` and `("pound", "pounds")`/`("penny", "pence")`.
    /// Corpus `currency:EUR 2` → "two euros" pins the EN table, not EUR's.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.english.currency_forms(code)
    }

    /// `CURRENCY_ADJECTIVES`, inherited from `Num2Word_EUR` and never shadowed
    /// by EN or AERO.
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.english.currency_adjective(code)
    }

    /// `CURRENCY_PRECISION`, set by `Num2Word_EN.__init__` and inherited by
    /// AERO: 1000 for the seven 3-decimal currencies (BHD/KWD/OMR/JOD/TND/
    /// LYD/IQD), 100 for everything else.
    ///
    /// Unlike `CURRENCY_FORMS` this is a plain attribute *assignment*, so the
    /// instances hold distinct-but-identical dicts (`a.CURRENCY_PRECISION is
    /// a._english.CURRENCY_PRECISION` is `False`). Contents are what matter.
    ///
    /// Note JPY/KRW are absent → divisor 100, not 1. The Python comment says
    /// this is deliberate ("their historical sen/jeon subunits are still
    /// expected by the test fixtures"), so the zero-decimal branch in
    /// `default_to_currency` is unreachable for this language. Corpus agrees:
    /// `currency:JPY 12.34` → "twelve yen, thirty-four sen", and
    /// `cheque:JPY 1234.56` → "... AND 56/100 YEN".
    ///
    /// Overriding this is what keeps the *inherited* default `cents_terse`
    /// correct on a direct call: Python's `a._cents_terse(5, "KWD")` → `"005"`,
    /// which needs the 1000 divisor to pick width 3.
    fn currency_precision(&self, code: &str) -> i64 {
        self.english.currency_precision(code)
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`. Inherited by EN
    /// and by AERO; `Num2Word_Base.pluralize` raises `NotImplementedError`, so
    /// this override is required rather than cosmetic.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        self.english.pluralize(n, forms)
    }

    // `money_verbose` / `cents_verbose` are deliberately NOT overridden.
    //
    // They are inherited straight from `Num2Word_Base` (`return
    // self.to_cardinal(number)`), and on an AERO receiver `self.to_cardinal` is
    // the digit-by-digit override — so the faithful behaviour is the trait
    // default, which routes back through *our* `to_cardinal`:
    //
    //     >>> a._money_verbose(12, "USD")            # AERO receiver
    //     'wun too'
    //     >>> a._english._money_verbose(12, "USD")   # sibling receiver
    //     'twelve'
    //
    // Delegating them to `self.english` would be wrong for a direct call, and
    // is unnecessary for the currency path (the delegate resolves its own).
    //
    // `cents_terse` is likewise left at the default: its formatting logic is
    // identical to Python's `_cents_terse`, and the override of
    // `currency_precision` above feeds it the right divisor.
    //
    // `cardinal_from_decimal` stays at the default (raises NotImplemented).
    // It is only reachable from `default_to_currency`'s fractional-cents
    // branch, which this language never enters — `to_currency` hands off before
    // reaching it, and the fractional path is out of scope for this phase.

    /// Port of `Num2Word_EN_AERO.to_currency` → `self._english.to_currency(...)`.
    ///
    /// Straight pass-through. Python's `*args, **kwargs` forwarding means any
    /// argument the caller omitted is filled by `Num2Word_Base.to_currency`'s
    /// own defaults (`currency="EUR"`, `cents=True`, `separator=","`,
    /// `adjective=False`) *at the delegate*, never by AERO — and since the Rust
    /// trait method receives all five explicitly from the dispatcher, forwarding
    /// them verbatim is exact.
    ///
    /// The `CurrencyValue::Int` / `::Decimal` split rides through untouched, so
    /// the delegate still sees `1` and `1.0` as different things: corpus
    /// `currency:EUR 1` → "one euro" (int, no cents) vs `currency:EUR 1.0` →
    /// "one euro, zero cents" (float, cents even at zero).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        self.english
            .to_currency(val, currency, cents, separator, adjective)
    }

    /// Port of `Num2Word_EN_AERO.to_cheque` → `self._english.to_cheque(...)`.
    ///
    /// Pass-through, same reasoning as `to_currency`. `Num2Word_Base.to_cheque`
    /// takes `currency="USD"` by default; the dispatcher supplies it here.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        self.english.to_cheque(val, currency)
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    fn cf(v: &FloatValue) -> String {
        LangEnAeroNato::new().to_cardinal_float(v, None).unwrap()
    }
    fn flt(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }
    fn dec(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    #[test]
    fn float_corpus_rows() {
        // (value, repr-derived precision, expected) — from corpus `cardinal`.
        let cases: &[(f64, u32, &str)] = &[
            (0.0, 1, "zero decimal zero"),
            (0.5, 1, "zero decimal fife"),
            (1.0, 1, "wun decimal zero"),
            (1.5, 1, "wun decimal fife"),
            (2.25, 2, "too decimal too fife"),
            (3.14, 2, "tree decimal wun fower"),
            (0.01, 2, "zero decimal zero wun"),
            (0.1, 1, "zero decimal wun"),
            (0.99, 2, "zero decimal niner niner"),
            (1.01, 2, "wun decimal zero wun"),
            (12.34, 2, "wun too decimal tree fower"),
            (99.99, 2, "niner niner decimal niner niner"),
            (100.5, 1, "wun zero zero decimal fife"),
            (1234.56, 2, "wun too tree fower decimal fife six"),
            (-0.5, 1, "minus zero decimal fife"),
            (-1.5, 1, "minus wun decimal fife"),
            (-12.34, 2, "minus wun too decimal tree fower"),
            (1.005, 3, "wun decimal zero zero fife"),
            (2.675, 3, "too decimal six seven fife"),
        ];
        for (v, p, expect) in cases {
            assert_eq!(cf(&flt(*v, *p)), *expect, "float {v} (prec {p})");
        }
    }

    #[test]
    fn decimal_corpus_rows() {
        // from corpus `cardinal_dec`.
        let cases: &[(&str, u32, &str)] = &[
            ("0.01", 2, "zero decimal zero wun"),
            ("1.10", 2, "wun decimal wun zero"),
            ("12.345", 3, "wun too decimal tree fower fife"),
            (
                "98746251323029.99",
                2,
                "niner ait seven fower six too fife wun tree too tree zero too niner decimal niner niner",
            ),
            ("0.001", 3, "zero decimal zero zero wun"),
        ];
        for (s, p, expect) in cases {
            assert_eq!(cf(&dec(s, *p)), *expect, "decimal {s} (prec {p})");
        }
    }

    #[test]
    fn negative_zero_float() {
        // str(-0.0) == "-0.0" → sign survives.
        assert_eq!(cf(&flt(-0.0, 1)), "minus zero decimal zero");
    }
}
