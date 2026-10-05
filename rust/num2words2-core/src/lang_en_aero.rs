//! Port of `lang_EN_AERO.py` — ICAO/aviation English (`en_AERO`).
//!
//! Registry check: `CONVERTER_CLASSES["en_AERO"]` →
//! `lang_EN_AERO.Num2Word_EN_AERO()` (the ICAO base class, `PROFILE = "ICAO"`).
//! The module also defines `Num2Word_EN_AERO_FAA` / `_USN` / `_US_Army` /
//! `_NATO` subclasses, but those are registered under *other* keys
//! (`en_Aero_FAA`, …) and are out of scope for this file. All five profiles
//! share `_ICAO_DIGITS`/`_ICAO_DECIMAL`/`_ICAO_MINUS` today anyway, so they
//! are byte-identical in the four modes we implement — see `_PROFILES`.
//!
//! Shape: **self-contained**. `Num2Word_EN_AERO` subclasses `Num2Word_EN`
//! (and therefore inherits its cards/merge/MAXVAL through `lang_EUR` →
//! `Num2Word_Base`), but it overrides `to_cardinal` outright with a
//! digit-by-digit renderer that never consults `self.cards`. So `cards`,
//! `maxval` and `merge` stay at their trait defaults here: nothing in the
//! four in-scope modes can reach them.
//!
//! # The `_english` delegate
//!
//! Python's `__init__` builds a *sibling* plain-English converter
//! (`self._english = Num2Word_EN()`) and routes `to_ordinal`, `to_fraction`,
//! `to_currency` and `to_cheque` through it. This is deliberate, and the
//! docstring says why: the digit-by-digit cardinal must not leak into the
//! ordinal builder, or `Num2Word_EN.to_ordinal(3)` would read the last
//! cardinal word "tree" and emit "treeth" instead of "third". The same goes
//! for fractions: `to_fraction(22, 7)` is "twenty-two sevenths", never
//! "too too sevenths".
//!
//! [`LangEnAero`] mirrors that literally by owning a [`LangEn`]. Consequences
//! worth stating, because they look like inconsistencies but are the spec:
//!
//! * `to_cardinal(3)` == "tree" but `to_ordinal(3)` == "third" — the two
//!   modes genuinely use different converters.
//! * `to_ordinal` inherits `Num2Word_EN`'s overflow ceiling (MAXVAL) and its
//!   "and" joiner (`to_ordinal(101)` == "one hundred and first"), while
//!   `to_cardinal` has **no** ceiling at all: it only stringifies digits, so
//!   it happily renders values far above `Num2Word_EN.MAXVAL`.
//! * `to_currency(12.34, "USD")` == "twelve dollars, thirty-four cents", not
//!   "wun too dollars…". Money is *not* read digit-by-digit. The corpus
//!   confirms this for all 108 currency and 9 cheque rows.
//!
//! # Currency: pure delegation, and why no table is built here
//!
//! Python is a two-liner — `to_currency`/`to_cheque` are `(*args, **kwargs)`
//! pass-throughs to `self._english`. So *every* currency hook resolves on the
//! delegate: `CURRENCY_FORMS`, `CURRENCY_ADJECTIVES`, `CURRENCY_PRECISION`,
//! `pluralize`, `_money_verbose`, `_cents_verbose`, `_cents_terse` and the
//! `to_cardinal` they call. None of `Num2Word_EN_AERO`'s own inherited copies
//! are ever consulted, even though the instance carries them.
//!
//! [`LangEnAero`] mirrors that by forwarding both methods to the [`LangEn`] it
//! already owns, and overrides **nothing else**. Consequences:
//!
//! * No `CURRENCY_FORMS` table is built in this file — the one that matters is
//!   `LangEn`'s, constructed once in `LangEn::new()`, which `LangEnAero::new()`
//!   already calls once and stores. Adding a second table here would be dead
//!   weight that could silently drift from the delegate's.
//! * `lang_name()` is deliberately left at its default: it is unreachable from
//!   this file, because the `NotImplementedError` is raised inside the
//!   delegate's `to_currency`, where `self.__class__.__name__` is the
//!   *delegate's* name. Python therefore reports `Num2Word_EN`, **not**
//!   `Num2Word_EN_AERO`, for an unknown code — verified against CPython:
//!   `to_currency(12.34, "XXX")` raises
//!   `Currency code "XXX" not implemented for "Num2Word_EN"`.
//!   Forwarding reproduces that quirk for free; a locally-built table would
//!   have emitted the wrong class name.
//!
//! # Precision quirk: JPY is a 2-decimal currency here
//!
//! `Num2Word_EN.__init__` *rebinds* `CURRENCY_PRECISION` to
//! `{BHD, KWD, OMR, JOD, TND, LYD, IQD: 1000}`. JPY/KRW are **absent**, so
//! they fall through to the default 100 and keep their historical sen/jeon
//! subunits (the Python comment says this is intentional, to satisfy the test
//! fixtures). Hence `to_currency(12.34, "JPY")` == "twelve yen, thirty-four
//! sen" — the zero-decimal (`divisor == 1`) branch in
//! `currency::default_to_currency` is **unreachable** for every code this
//! language defines.
//!
//! # Overrides that differ from the `Num2Word_EN` parent
//!
//! * `to_year(value)` ignores `Num2Word_EN`'s century logic entirely and is
//!   just `to_cardinal(value)`. Aviation reads years digit-by-digit, so
//!   `to_year(1971)` == "wun niner seven wun", **not** "nineteen seventy-one".
//!   Negatives are *not* special-cased into a "BC" suffix the way the parent
//!   does: the raw value reaches `_digits_of`, which strips the sign into the
//!   leading "minus" word. Hence `to_year(-44)` == "minus fower fower"
//!   (parent EN would say "forty-four BC"). Verified against the corpus.
//! * `to_ordinal_num(value)` is `verify_ordinal(value); str(int(value))` — it
//!   drops the parent's `"%s%s" % (value, self.to_ordinal(value)[-2:])`
//!   suffix logic. So it returns bare digits: `to_ordinal_num(1)` == "1",
//!   not "1st". This coincides with the trait's default `to_ordinal_num`
//!   *except* for the `verify_ordinal` guard, which the default lacks, so the
//!   override below exists purely to reproduce the negative-input TypeError.
//!
//! # Digit table
//!
//! `_ICAO_DIGITS` respells six digits to survive radio static. Kept verbatim,
//! including the non-obvious spellings, which are correct ICAO and **not**
//! typos: 1 → "wun", 2 → "too", 3 → "tree", 4 → "fower", 5 → "fife",
//! 8 → "ait", 9 → "niner". 0/6/7 are unchanged ("zero"/"six"/"seven").
//!
//! # Errors
//!
//! `verify_ordinal` (from `Num2Word_Base`) raises `TypeError` on negatives,
//! reached by both `to_ordinal` (via the `_english` delegate's own
//! `verify_ordinal`) and `to_ordinal_num` (via `self`'s). No `KeyError` is
//! reachable in these modes: `to_cardinal`'s `self._digit_table[ch]` lookup is
//! guarded by `ch.isdigit()`, and `BigInt`'s decimal rendering only ever
//! produces ASCII `0`-`9` plus a leading `-` that `_digits_of` strips first.

use crate::base::{Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::lang_en::LangEn;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign};
use num_traits::Signed;

/// Python's `_ICAO_DIGITS`, indexed by digit value rather than by char.
///
/// The Python original is a `dict` keyed by the *character* `"0"`..`"9"`;
/// since every key is a distinct decimal digit, a positional table is the
/// same mapping with the same total-ness.
const ICAO_DIGITS: [&str; 10] = [
    "zero", "wun", "too", "tree", "fower", "fife", "six", "seven", "ait", "niner",
];

/// Python's `_ICAO_DECIMAL`.
const ICAO_DECIMAL: &str = "decimal";

/// Python's `_ICAO_MINUS`.
const ICAO_MINUS: &str = "minus";

pub struct LangEnAero {
    /// Mirrors `self._english = Num2Word_EN()` — the sibling converter that
    /// `to_ordinal` delegates to so digit-by-digit cardinals never leak in.
    english: LangEn,
    /// Mirrors `self._digit_table`, from `_PROFILES["ICAO"]`.
    digit_table: [&'static str; 10],
    /// Mirrors `self._decimal_word`. Unreachable from integer input (kept so
    /// `to_cardinal` stays a line-for-line image of the Python).
    decimal_word: &'static str,
    /// Mirrors `self._minus_word`.
    minus_word: &'static str,
}

impl Default for LangEnAero {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnAero {
    pub fn new() -> Self {
        // Python resolves `profile or self.PROFILE` → "ICAO" for this key, then
        // unpacks `_PROFILES["ICAO"]`. The ValueError for an unknown profile is
        // unreachable here: the registry constructs this class with no argument.
        LangEnAero {
            english: LangEn::new(),
            digit_table: ICAO_DIGITS,
            decimal_word: ICAO_DECIMAL,
            minus_word: ICAO_MINUS,
        }
    }

    /// Port of `_digits_of` for the integer inputs in scope.
    ///
    /// Python normalises to `(sign, int-part, frac-part)` and handles `str` /
    /// `Decimal` / everything-else. Our input is always a `BigInt`, i.e. the
    /// `else: s = str(value)` branch. The comma/underscore stripping and the
    /// `"." in s` split are then both no-ops (`BigInt::to_string` emits no
    /// separators and no point), as is the `if not int_part: int_part = "0"`
    /// guard, since `str(0)` is already `"0"`. So the frac part is always
    /// empty and only the sign split survives.
    fn digits_of(&self, value: &BigInt) -> (bool, String) {
        let s = value.to_string();
        let is_negative = s.starts_with('-');
        let int_part = if is_negative { s[1..].to_string() } else { s };
        (is_negative, int_part)
    }

    /// Port of `_digits_of` for the non-integer inputs the float path carries.
    ///
    /// Python's `to_cardinal` is overridden outright and reads the value's
    /// *string* form digit-by-digit — `str(value)` for a `float`,
    /// `format(value, "f")` for a `Decimal` — and never touches `float2tuple`
    /// or `to_cardinal_float`. This reproduces `_digits_of`'s
    /// `(sign, int-part, frac-part)` split from that string content:
    ///
    /// * **Float** — [`python_float_repr`] reproduces Python's `str(value)`
    ///   digit-for-digit, **including the scientific form**: in the normal
    ///   range both are the correctly-rounded `p`-digit decimal of the *same*
    ///   IEEE-754 double, and outside `[1e-4, 1e16)` CPython's repr switches
    ///   to `"1e+16"` / `"1e-05"`, which AERO then reads lexically ('e'/'+'
    ///   are skipped by the digit filter) — `1e16` is "wun wun six", not the
    ///   sixteen-zero expansion. This is precisely why the f64-artefact
    ///   heuristic in `base.float2tuple` is irrelevant on this route: `2.675`
    ///   reads as its repr digits "six seven fife", not the `674.999…`-rescued
    ///   `675`. `is_sign_negative()` mirrors `str(value).startswith("-")` (so
    ///   `-0.0` carries the leading "minus" word, as in Python — corpus:
    ///   `cardinal(-0.0)` → "minus zero decimal zero"). A sci-form mantissa
    ///   with a fraction splits on its '.' exactly as `_digits_of` does
    ///   (`"1.5e+20"` → int "1", frac "5e+20" → "wun decimal fife too zero").
    /// * **Decimal** — rebuild `format(value, "f")` from the absolute unscaled
    ///   digits at scale `precision`, which preserves trailing zeros exactly
    ///   (`Decimal("1.10")` -> int "1", frac "10"). `precision` equals the
    ///   Decimal's own scale (`abs(as_tuple().exponent)`), so forcing that
    ///   scale is a faithful no-op that also guards against any normalisation.
    ///
    /// The `if not int_part: int_part = "0"` guard is a no-op here (both arms
    /// always emit at least one integer digit) but is reproduced for parity.
    fn digits_of_float(&self, v: &FloatValue) -> (bool, String, String) {
        match v {
            FloatValue::Float { value, precision } => {
                let is_negative = value.is_sign_negative();
                let signed = python_float_repr(*value, *precision);
                let s = signed.strip_prefix('-').unwrap_or(&signed).to_string();
                let (int_part, frac_part) = match s.split_once('.') {
                    Some((i, f)) => (i.to_string(), f.to_string()),
                    None => (s, String::new()),
                };
                let int_part = if int_part.is_empty() {
                    "0".to_string()
                } else {
                    int_part
                };
                (is_negative, int_part, frac_part)
            }
            FloatValue::Decimal { value, .. } => {
                let is_negative = value.is_negative();
                // format(value, "f"): fixed-point, using the Decimal's *own*
                // exponent, trailing zeros kept. `as_bigint_and_exponent`
                // returns (unscaled, scale) with value == unscaled * 10^-scale,
                // so a NEGATIVE scale here is a positive Decimal exponent —
                // `Decimal("1E+2")` -> (1, -2) — which format "f" expands to
                // plain integer digits with NO fractional part ("100", never
                // "100.00"). The `precision` field can't express that case
                // (it's the |scale|), so the sign is read off the value itself.
                let (unscaled, scale) = value.abs().as_bigint_and_exponent();
                if scale <= 0 {
                    // format(Decimal("1E+20"), "f") == "1" + 20 * "0": integer
                    // digits only, so the "decimal" word never appears.
                    let digits =
                        (unscaled * BigInt::from(10).pow((-scale) as u32)).to_string();
                    return (is_negative, digits, String::new());
                }
                let scale = scale as usize;
                let digits = unscaled.to_string();
                let (int_part, frac_part) = {
                    // Ensure at least one integer digit before the point, as
                    // format(value, "f") renders "0.01" not ".01".
                    let padded = if digits.len() < scale + 1 {
                        format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits)
                    } else {
                        digits
                    };
                    let split = padded.len() - scale;
                    (padded[..split].to_string(), padded[split..].to_string())
                };
                let int_part = if int_part.is_empty() {
                    "0".to_string()
                } else {
                    int_part
                };
                (is_negative, int_part, frac_part)
            }
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`not value == int(value)` → TypeError) cannot fire on
    /// `BigInt`; only the negative check is reachable. Message text matches
    /// `errmsg_negord` byte for byte.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == Sign::Minus {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

impl Lang for LangEnAero {

    /// `maxval()` is None (gladiaio/num2words2#159): the cardinal reads digit
    /// by digit with no ceiling, so the inherited `Num2Word_EN.MAXVAL` (10^306)
    /// is not reported — the cardinal happily converts past it.
    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        None
    }
    // cards() / maxval() / merge() stay at their trait defaults: to_cardinal is
    // overridden below and never drives splitnum/clean, so nothing reaches them.
    // (Python still *has* Num2Word_EN's cards on this instance — they are simply
    // dead weight for every mode in scope.)

    fn negword(&self) -> &str {
        // Inherited from Num2Word_EN's setup(). Unused: the AERO to_cardinal
        // emits self._minus_word ("minus", no trailing space) instead, and
        // default_to_cardinal is never reached.
        "minus "
    }

    fn pointword(&self) -> &str {
        "point"
    }

    /// Port of `Num2Word_EN_AERO.to_cardinal` — digit-by-digit, never composite.
    ///
    /// `5739` → "fife seven tree niner", not "five thousand seven hundred
    /// thirty-nine". The `isdigit()` guard and the fractional branch are kept
    /// even though integer input makes them total/dead respectively, so this
    /// reads against the Python line for line.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (is_negative, int_part) = self.digits_of(value);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(self.minus_word);
        }
        for ch in int_part.chars() {
            if let Some(d) = ch.to_digit(10) {
                words.push(self.digit_table[d as usize]);
            }
        }
        // The `if frac_part:` arm of the Python is unreachable for BigInt input
        // (digits_of always returns an empty fractional part), so self.decimal_word
        // is never emitted here. Touched to keep the field live and documented.
        let _ = self.decimal_word;
        Ok(words.join(" "))
    }

    /// Port of the float/Decimal path of `Num2Word_EN_AERO.to_cardinal`.
    ///
    /// AERO does **not** use `Num2Word_Base.to_cardinal_float`/`float2tuple`:
    /// its overridden `to_cardinal` handles a non-integer `value` inline by
    /// reading `_digits_of(value)` (the value's string form) digit-by-digit.
    /// So this override reproduces that body rather than delegating to
    /// `floatpath::default_to_cardinal_float`, which would (a) use `pointword`
    /// ("point") instead of the ICAO `_decimal_word` ("decimal") and (b) route
    /// through the `< 0.01` f64-artefact heuristic that AERO never touches.
    ///
    /// `precision_override` (issue #580) is deliberately ignored: Python's
    /// `AERO.to_cardinal` takes no `precision` kwarg and never consults
    /// `self.precision`, so setting it has no effect on this mode's output.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let (is_negative, int_part, frac_part) = self.digits_of_float(value);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(self.minus_word);
        }
        for ch in int_part.chars() {
            if let Some(d) = ch.to_digit(10) {
                words.push(self.digit_table[d as usize]);
            }
        }
        if !frac_part.is_empty() {
            words.push(self.decimal_word);
            for ch in frac_part.chars() {
                if let Some(d) = ch.to_digit(10) {
                    words.push(self.digit_table[d as usize]);
                }
            }
        }
        Ok(words.join(" "))
    }

    /// Port of `to_ordinal` — delegates to the plain-English sibling.
    ///
    /// Deliberate: ICAO does not standardise ordinals, and routing through the
    /// AERO cardinal would produce "treeth" for 3. The delegate runs its own
    /// `verify_ordinal`, which is where the negative-input TypeError comes from.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.english.to_ordinal(value)
    }

    /// Port of `to_ordinal_num`: `verify_ordinal(value); return str(int(value))`.
    ///
    /// Note this deliberately does **not** call `to_ordinal` — no "st"/"nd"/"th"
    /// suffix is appended, unlike the Num2Word_EN parent it overrides.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// Port of `to_year`: aviation reads years digit-by-digit, so this is just
    /// `to_cardinal`. No century logic, no "BC" suffix — see the module docs.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries ----------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so AERO's `to_cardinal` string-reading, the `_english`
    // delegation of `to_ordinal`, and `to_ordinal_num`'s `verify_ordinal`
    // all become reachable with fractional and *whole* non-int input alike.

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// AERO's `to_cardinal` reads `str(value)` / `format(value, "f")` and
    /// therefore keeps the ".0" tail Python's string carries: `5.0` →
    /// "fife decimal zero", **never** the trait default's whole-value integer
    /// route ("fife"). The `Decimal("Infinity")`/`Decimal("NaN")` sentinels
    /// (see [`aero_str_to_number`]) render exactly as `_digits_of` reads
    /// `format(value, "f")` = "Infinity"/"-Infinity"/"NaN": no digit chars,
    /// so only the sign word survives — `""` / `"minus"` / `""` (corpus).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if let Some(sp) = aero_special_of(value) {
            return Ok(sp.cardinal_words().to_string());
        }
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` → `self._english.to_ordinal(value)`.
    ///
    /// The delegate's `verify_ordinal` does the type policing: whole values
    /// ordinalise in plain English (`5.0` → "fifth", `Decimal("1E+2")` →
    /// "one hundredth", `-0.0` → "zeroth"), fractional or negative values
    /// raise TypeError. The Infinity/NaN sentinels reproduce the `int(value)`
    /// raise *inside* that comparison: OverflowError / ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(sp) = aero_special_of(value) {
            return Err(sp.int_error());
        }
        self.english.ordinal_float_entry(value)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` — AERO's own,
    /// i.e. `Num2Word_Base`'s — then `str(int(value))`.
    ///
    /// Bare truncated digits, no English suffix: `5.00` → "5", `1e+16` →
    /// "10000000000000000", `-0.0` → "0" (both checks pass numerically).
    /// Python's check order is float-ness first, then sign, so `-1.5` raises
    /// the *float* message. `%s` interpolates `str(value)` = `repr_str`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        if let Some(sp) = aero_special_of(value) {
            return Err(sp.int_error());
        }
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            ))),
            Some(i) => {
                if i.sign() == Sign::Minus {
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

    /// `to_year(float/Decimal)` = `to_cardinal(value)` — the same digit-by-
    /// digit string reading, ".0" tail and all: `to_year(1971.0)` →
    /// "wun niner seven wun decimal zero".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// Port of `to_fraction`: `return self._english.to_fraction(numerator,
    /// denominator)` — standard English forms ("one half", "three quarters",
    /// "twenty-two sevenths"), never digit-by-digit. Both the cardinal
    /// numerator and the ordinal denominator are the *delegate's*, so `22/7`
    /// is "twenty-two sevenths", not "too too sevenths". The ZeroDivisionError
    /// for `n/0` also originates inside the delegate.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        self.english.to_fraction(numerator, denominator)
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, with
    /// `Decimal("Infinity")`/`Decimal("NaN")` carried across the boundary as
    /// sentinels instead of the dispatcher's hardwired base errors, because
    /// AERO's string-reading `to_cardinal` *succeeds* on them (→ ""/"minus").
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        aero_str_to_number(s)
    }

    // ---- currency ----------------------------------------------------
    //
    // Python:
    //     def to_currency(self, *args, **kwargs):
    //         return self._english.to_currency(*args, **kwargs)
    //     def to_cheque(self, *args, **kwargs):
    //         return self._english.to_cheque(*args, **kwargs)
    //
    // Whole-method forwarding, so no other currency hook is overridden here —
    // see the module docs for why that is the faithful choice rather than a
    // shortcut (it is what makes the unknown-code error name the *delegate*,
    // and what keeps the forms table single-sourced in `LangEn::new()`).

    /// Port of `Num2Word_EN_AERO.to_currency` — forwards to the `_english`
    /// sibling, so money reads as ordinary English, never digit-by-digit.
    ///
    /// The forwarded call lands in `Num2Word_Base.to_currency` with
    /// `self` bound to the delegate, which is exactly what
    /// `Lang::to_currency`'s default does for `LangEn`. In particular the
    /// `CurrencyValue::Int` / `Decimal` split is preserved end to end: `100`
    /// renders "one hundred dollars" (no cents segment), while `1.0` renders
    /// "one dollar, zero cents".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Keep the Infinity/NaN sentinels out of the delegate's arithmetic
        // (no corpus row reaches this; Python's delegate raises from the
        // int() cast inside parse_currency_parts, same types as here).
        if let CurrencyValue::Decimal { value: d, .. } = val {
            if let Some(sp) = aero_special_of_decimal(d) {
                return Err(sp.int_error());
            }
        }
        self.english
            .to_currency(val, currency, cents, separator, adjective)
    }

    /// Port of `Num2Word_EN_AERO.to_cheque` — forwards to the `_english`
    /// sibling. `1234.56` / KWD → "ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR
    /// AND 560/1000 DINARS" (3-digit subunit, plural unit, upper-cased).
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        self.english.to_cheque(val, currency)
    }
}

// ---------------------------------------------------------------------
// Shared AERO helpers.
//
// Python defines Num2Word_EN_AERO and all four service subclasses in ONE
// module (`lang_EN_AERO.py`); the subclasses add nothing but `PROFILE`.
// The pieces below are that shared behaviour, `pub(crate)` so the four
// profile files (`lang_en_aero_faa/nato/us_army/usn.rs`) reuse them
// instead of drifting apart — exactly the sharing the Python source has.
// ---------------------------------------------------------------------

/// `str(value)` for a Python `float` — the shortest round-trip repr.
///
/// In the normal range the value formats to exactly `precision` fractional
/// digits, where `precision` is the repr-derived count handed across the
/// boundary (`abs(Decimal(repr(v)).as_tuple().exponent)`), so `{:.precision}`
/// reproduces `repr(float)` digit for digit. Outside `[1e-4, 1e16)` CPython
/// switches to scientific (`"1e+16"`, `"1e-05"`, `"1.5e-07"`); the thresholds
/// `a >= 1e16 || a < 1e-4` are exactly CPython's `decpt > 16 || decpt <= -4`.
/// Rust's `{:e}` is shortest round-trip like Python's repr, so the mantissa
/// digits agree; only the exponent presentation (mandatory sign, two-digit
/// minimum) is reshaped to match. AERO's `_digits_of` then reads whichever
/// form character by character — which is why `1e16` is "wun wun six".
///
/// `nan`/`inf` cannot reach this through the boundary (the Python-side
/// precision step raises TypeError first), but are spelled Python's lowercase
/// way regardless so a digit reader collapses them to `""` as `_digits_of`
/// would.
pub(crate) fn python_float_repr(v: f64, precision: u32) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let a = v.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // "-1.234e-5" -> "-1.234e-05"; "1e16" -> "1e+16".
        let sci = format!("{:e}", v);
        let (mantissa, exp) = sci
            .split_once('e')
            .expect("Rust's {:e} always emits an 'e' separator");
        let exp: i32 = exp
            .parse()
            .expect("Rust's {:e} always emits a decimal exponent");
        return format!(
            "{}e{}{:02}",
            mantissa,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }
    format!("{:.*}", precision as usize, v)
}

// ---- Decimal("Infinity") / Decimal("NaN") sentinels -------------------
//
// Python: `num2words("Infinity", lang="en_AERO")` parses to
// `Decimal("Infinity")`, and AERO's `to_cardinal` *succeeds* on it —
// `_digits_of` reads `format(value, "f")` = "Infinity", finds no digit
// characters, and returns just the sign word: "" / "minus" / "" for
// Infinity / -Infinity / NaN (all three are corpus rows).
//
// The Rust dispatcher, however, hardwires `ParsedNumber::Inf`/`NaN` to
// Base's OverflowError/ValueError *before* any per-language hook — correct
// for every converter whose to_cardinal casts to int, wrong for AERO. So
// AERO's `str_to_number` smuggles the specials through as `ParsedNumber::
// Dec` sentinels the float entries un-smuggle. The sentinel is a 1-digit
// coefficient at an exponent (~1e9) no real corpus value can carry (Python
// would need a literal `1E+1000000001`), and every AERO entry checks for it
// before any arithmetic, so the huge magnitude is never materialised.

/// Sentinel exponent for `Decimal("Infinity")` (`-1` coefficient: negative).
pub(crate) const AERO_INF_EXPONENT: i64 = 1_000_000_001;
/// Sentinel exponent for `Decimal("NaN")`.
pub(crate) const AERO_NAN_EXPONENT: i64 = 1_000_000_003;

/// Which special a sentinel stands for.
pub(crate) enum AeroSpecial {
    Inf { negative: bool },
    NaN,
}

impl AeroSpecial {
    /// What AERO's `to_cardinal` says for this special: `_digits_of` finds no
    /// digit characters in "Infinity"/"-Infinity"/"NaN" (`format(value, "f")`
    /// spells them exactly so), leaving only the `_ICAO_MINUS` sign word.
    pub(crate) fn cardinal_words(&self) -> &'static str {
        match self {
            AeroSpecial::Inf { negative: true } => "minus",
            _ => "",
        }
    }

    /// The raise from `int(value)` — reached inside `verify_ordinal`'s
    /// `value == int(value)` comparison (ordinal / ordinal_num) and inside
    /// the currency delegate. Types and messages are CPython's `decimal`
    /// module's own.
    pub(crate) fn int_error(&self) -> N2WError {
        match self {
            AeroSpecial::Inf { .. } => {
                N2WError::Overflow("cannot convert Infinity to integer".into())
            }
            AeroSpecial::NaN => N2WError::Value("cannot convert NaN to integer".into()),
        }
    }
}

/// `converter.str_to_number` for every AERO profile: base `Decimal(value)`
/// semantics with Infinity/NaN mapped to the sentinels above.
pub(crate) fn aero_str_to_number(s: &str) -> Result<ParsedNumber> {
    match python_decimal_parse(s)? {
        ParsedNumber::Inf { negative } => Ok(ParsedNumber::Dec(BigDecimal::new(
            BigInt::from(if negative { -1 } else { 1 }),
            -AERO_INF_EXPONENT,
        ))),
        ParsedNumber::NaN => Ok(ParsedNumber::Dec(BigDecimal::new(
            BigInt::from(1),
            -AERO_NAN_EXPONENT,
        ))),
        other => Ok(other),
    }
}

/// Un-smuggle a sentinel from a raw `BigDecimal` (currency path).
pub(crate) fn aero_special_of_decimal(d: &BigDecimal) -> Option<AeroSpecial> {
    let (m, scale) = d.as_bigint_and_exponent();
    if scale == -AERO_INF_EXPONENT {
        return Some(AeroSpecial::Inf {
            negative: m.sign() == Sign::Minus,
        });
    }
    if scale == -AERO_NAN_EXPONENT {
        return Some(AeroSpecial::NaN);
    }
    None
}

/// Un-smuggle a sentinel from the float-entry `FloatValue`. Only the
/// `Decimal` arm can carry one — the sentinels are born in
/// [`aero_str_to_number`] as `ParsedNumber::Dec`.
pub(crate) fn aero_special_of(v: &FloatValue) -> Option<AeroSpecial> {
    match v {
        FloatValue::Decimal { value, .. } => aero_special_of_decimal(value),
        FloatValue::Float { .. } => None,
    }
}
