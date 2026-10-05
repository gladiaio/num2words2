//! Port of `lang_EN_AERO.py` → `Num2Word_EN_AERO_USN` (key `"en_Aero_USN"`).
//!
//! Registry check: `__init__.py` maps both `"en_Aero_USN"` and
//! `"en_Aero_US_Navy"` to `lang_EN_AERO.Num2Word_EN_AERO_USN()`. That class is
//! a two-line subclass of `Num2Word_EN_AERO` that only sets
//! `PROFILE = "USN"`; `_PROFILES["USN"]` is `(_ICAO_DIGITS, "decimal",
//! "minus")` — byte-identical to the `"ICAO"` entry. The Python module says so
//! outright: the profiles "are intentionally identical — they exist as named
//! entry points". So this file's behaviour equals `en_Aero_ICAO`'s; the
//! duplication is deliberate and mirrors upstream.
//!
//! Shape: **self-contained**. `Num2Word_EN_AERO` subclasses `Num2Word_EN`
//! (which *is* an engine language) but overrides `to_cardinal` outright with a
//! digit-by-digit reader that never consults `self.cards` or `merge`. The
//! inherited EN card table is still built by `Num2Word_EN.__init__` via
//! `super().__init__()`, but nothing in the four in-scope modes reads it
//! through `self`. Hence `cards`/`maxval`/`merge` stay at their trait defaults
//! here — including **no overflow check**: EN's `MAXVAL` is inherited but
//! unreachable, so `to_cardinal(10**400)` happily spells 401 digits rather
//! than raising `OverflowError`. (Corpus confirms `10**21` → 22 digit-words.)
//!
//! # The delegation handshake — the one structurally interesting bit
//!
//! `Num2Word_EN_AERO.__init__` stashes a **sibling** plain-English converter:
//!
//! ```python
//! self._english = Num2Word_EN()
//! ```
//!
//! `to_ordinal` then routes through it — `return self._english.to_ordinal(value)` —
//! specifically so the digit-by-digit cardinal does *not* leak into EN's
//! ordinal builder (the module's own docstring notes this would otherwise turn
//! "third" into "treeth" via vowel-substitution chaining). This is delegation
//! to a separate object, **not** cross-call mutable state: `_english` is
//! constructed once in `__init__`, is never mutated, and no method sets a flag
//! another consumes. Nothing for the dispatcher to skip.
//!
//! Modelled here by embedding a [`LangEn`] and calling its trait `to_ordinal`,
//! which is exactly what Python does. The consequence is worth stating plainly
//! because it looks like a bug and is not:
//!
//! * `to_cardinal(3)` == "tree"   (AERO's own digit reader)
//! * `to_ordinal(3)`  == "third"  (plain EN, via `_english`)
//!
//! These two modes disagree by design. Corpus agrees on both.
//!
//! # Mode-by-mode provenance
//!
//! | mode | source | note |
//! |---|---|---|
//! | `to_cardinal` | `Num2Word_EN_AERO.to_cardinal` | digit-by-digit, ICAO table |
//! | `to_year` | `Num2Word_EN_AERO.to_year` | `return self.to_cardinal(value)` — drops EN's "BC" suffix entirely |
//! | `to_ordinal` | `Num2Word_EN.to_ordinal` via `self._english` | plain English words |
//! | `to_ordinal_num` | `Num2Word_EN_AERO.to_ordinal_num` | `verify_ordinal` + `str(int(value))` — **overrides** EN's "1st"/"2nd" form, so it returns bare digits |
//!
//! `to_year` is the sharpest divergence from the `lang_en` parent: EN's
//! `to_year(-500)` yields "five hundred BC", but AERO's override never calls
//! `super()` — it re-reads the whole value, sign included, through the digit
//! reader. Corpus: `year(-500)` → "minus fife zero zero", `year(-44)` →
//! "minus fower fower". No "BC" anywhere, and no century-splitting ("1900" is
//! "wun niner zero zero", not "nineteen hundred").
//!
//! # Fidelity notes
//!
//! * `_ICAO_DIGITS` respells 1→"wun", 2→"too", 3→"tree", 4→"fower",
//!   5→"fife", 8→"ait", 9→"niner"; 0/6/7 keep "zero"/"six"/"seven". These are
//!   the upstream table's spellings verbatim — "wun", "too", "fower", "ait"
//!   are *not* typos to fix, they are the ICAO Annex 10 vol II respellings.
//! * `to_ordinal_num` returning "42" rather than "42nd" is likewise upstream's
//!   choice, not an omission: AERO shadows `Num2Word_EN.to_ordinal_num`.
//!   Corpus confirms `ordinal_num(1)` → "1", not "1st".
//! * Out of scope and therefore absent: `to_fraction` and the aviation
//!   phraseology helpers (`to_altitude`, `to_flight_level`, `to_heading`,
//!   `to_squawk`, `to_runway`, `to_frequency`) — none are reachable from the
//!   ported modes.
//!
//! # The float / `Decimal` path (phase 3)
//!
//! `Num2Word_EN_AERO` does **not** override `to_cardinal_float`; it overrides
//! `to_cardinal`, and *that* method handles floats and `Decimal`s inline via
//! `_digits_of`. So a non-integral value reaches AERO through
//! `converter.to_cardinal(value)` in Python, never through the base
//! `float2tuple` machinery. Two consequences make the inherited
//! `default_to_cardinal_float` wrong for AERO, which is why this file overrides
//! it:
//!
//! * **The radix word is "decimal", not the pointword.** `_digits_of` +
//!   `to_cardinal` emit `self._decimal_word` ("decimal"); the base float path
//!   would emit `self.pointword` ("point"). Corpus: `0.5` → "zero decimal
//!   fife".
//! * **The sign word is "minus", placed by reading the string.** `-0.5` →
//!   "minus zero decimal fife". The base path would instead prepend
//!   `negword().trim()` ("(-)") when `pre == 0`, giving the wrong "(-) zero
//!   ...". AERO reads the leading "-" off `str(value)` and emits `minus_word`.
//!
//! The reading is purely lexical: AERO renders every *digit character* of the
//! value's string form, so it never touches `float2tuple`'s binary-artefact
//! heuristic. `str(float)` is reproduced by [`python_float_str`]. For the vast
//! majority of values Python's shortest `repr` is fixed-point with exactly
//! `precision` fractional digits, and `format!("{:.precision$}")` of the raw
//! `f64` equals it byte-for-byte (`1.005`/`2.675` included — verified against
//! every corpus float). When Python's `repr` instead switches to exponent
//! notation — `str(1e16)` == "1e+16", `str(1e-5)` == "1e-05" — AERO reads that
//! `e`/`+`/exponent-digit string *verbatim* ("wun wun six", "wun zero fife"),
//! because the reader consumes characters, not a value; `python_float_str`
//! therefore rebuilds the exponent form rather than a fixed-point expansion,
//! matching CPython's `decpt <= -4 || decpt > 16` threshold. `format(Decimal,
//! "f")` is
//! reproduced by rescaling the `BigDecimal` to `precision` fractional digits,
//! which preserves trailing zeros (`Decimal("1.10")` → "wun decimal wun zero").
//! `precision=`/`precision_override` is ignored, exactly as Python's
//! `to_cardinal` ignores `self.precision`.
//!
//! # Currency (phase 2)
//!
//! `to_currency` and `to_cheque` are two more `_english` delegations, in the
//! same shape as `to_ordinal`:
//!
//! ```python
//! def to_currency(self, *args, **kwargs):
//!     return self._english.to_currency(*args, **kwargs)
//! def to_cheque(self, *args, **kwargs):
//!     return self._english.to_cheque(*args, **kwargs)
//! ```
//!
//! Delegation — not a local table — is the whole point, and it is load-bearing
//! in a way that is easy to get wrong. `Num2Word_EN_AERO` *is* a
//! `Num2Word_EN` subclass, so it inherits `CURRENCY_FORMS`,
//! `CURRENCY_ADJECTIVES`, `CURRENCY_PRECISION` and `pluralize` intact. Were
//! `to_currency` to run against `self`, `Num2Word_Base._money_verbose` would
//! call `self.to_cardinal` and resolve to **AERO's digit-by-digit reader** —
//! `to_currency(12.34, "USD")` would emit "wun too dollars, tree fower cents".
//! Routing through `_english` rebinds `self` to the plain-English object, so
//! `_money_verbose` reaches `Num2Word_EN.to_cardinal` and the corpus gets
//! "twelve dollars, thirty-four cents". Every one of the 108 currency rows and
//! 9 cheque rows is plain composite English — no ICAO respelling appears in a
//! single one, which is exactly this rebinding, observable.
//!
//! Consequences worth spelling out:
//!
//! * The inherited currency hooks (`currency_forms`, `currency_precision`,
//!   `pluralize`, `money_verbose`, `cents_verbose`, `cents_terse`,
//!   `lang_name`) stay at their trait defaults **on this struct** and are
//!   deliberately not overridden: nothing can reach them. `default_to_currency`
//!   only consults the hooks of the object it is handed, and it is handed
//!   `self.english`. Mirroring EN's table here would be a second copy that
//!   Python guarantees can never diverge — so it must not exist.
//! * The `Currency code "X" not implemented for "Y"` message therefore names
//!   **`Num2Word_EN`**, not `Num2Word_EN_AERO_USN`: Python builds it from
//!   `self.__class__.__name__` where `self` is the `_english` sibling. Falling
//!   back to a local table with `lang_name() = "Num2Word_EN_AERO_USN"` would
//!   silently change that string. No corpus row exercises it (all 117 rows
//!   succeed), which is precisely why it needs stating rather than testing.
//! * `cardinal_from_decimal` (fractional-cents path) likewise stays at its
//!   default here and is unreachable: the fractional branch runs inside
//!   `LangEn::to_currency`, so it is EN's hook that answers.
//!
//! Cost: the `LangEn` — and with it EN's card table and `CURRENCY_FORMS` — is
//! built once, in `LangEnAeroUsn::new()`, matching `__init__`'s single
//! `Num2Word_EN()` construction. Nothing is rebuilt per call.
//!
//! # Currency fidelity notes (all inherited from `Num2Word_EN`, via `_english`)
//!
//! * **JPY is not zero-decimal here.** `Num2Word_EN.CURRENCY_PRECISION` lists
//!   only BHD/KWD/OMR/JOD/TND/LYD/IQD at 1000; JPY and KRW fall through to the
//!   default 100 and keep their historical sen/jeon subunits, so
//!   `currency(12.34, "JPY")` → "twelve yen, thirty-four sen" rather than
//!   rounding to "twelve yen". `lang_EN.py` says this is deliberate ("still
//!   expected by the test fixtures"). Corpus agrees. The `divisor == 1` branch
//!   of `default_to_currency` is therefore dead for every EN currency.
//! * `Num2Word_EN.__init__` **overrides** several `Num2Word_EUR` forms after
//!   `super().__init__()`, and the overrides are observable: EUR's
//!   `("euro", "euro")` becomes `("euro", "euros")` (corpus: `0` → "zero
//!   euros", cheque → "EUROS"), and `("pound sterling", "pounds sterling")`
//!   becomes `("pound", "pounds")` (corpus: "zero pounds", not "zero pounds
//!   sterling").

use crate::base::{Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::lang_en::LangEn;
use crate::lang_en_aero::{aero_special_of, aero_special_of_decimal, aero_str_to_number};
use crate::strnum::ParsedNumber;
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign};

/// `_ICAO_DIGITS`, indexed by digit value.
///
/// Python keys this dict by the *character* ("0".."9") and looks up
/// `self._digit_table[ch]`, which would raise `KeyError` on a non-digit key.
/// That branch is unreachable: every caller guards with `ch.isdigit()` first,
/// and our input is `BigInt`, whose decimal rendering is `-?[0-9]+`. Indexing
/// by value is therefore equivalent and total.
const ICAO_DIGITS: [&str; 10] = [
    "zero",  // 0
    "wun",   // 1  (not "one" — ICAO respelling)
    "too",   // 2  (not "two")
    "tree",  // 3  (not "three")
    "fower", // 4  (not "four")
    "fife",  // 5  (not "five")
    "six",   // 6
    "seven", // 7
    "ait",   // 8  (not "eight")
    "niner", // 9  (not "nine")
];

const ICAO_DECIMAL: &str = "decimal";
const ICAO_MINUS: &str = "minus";

/// `Num2Word_EN_AERO_USN` — US Navy / Coast Guard profile.
pub struct LangEnAeroUsn {
    /// Python's `self._english = Num2Word_EN()`: the sibling used by
    /// `to_ordinal` so plain-English words survive instead of "treeth".
    english: LangEn,
    /// `_PROFILES["USN"][0]` — the ICAO digit table.
    digit_table: [&'static str; 10],
    /// `_PROFILES["USN"][1]` — the word for the radix point ("decimal").
    /// The integer `to_cardinal` never reads it (`BigInt` input carries no
    /// "."), but the float/`Decimal` path (`to_cardinal_float`) does: AERO
    /// says "decimal", not the base `pointword` "point".
    decimal_word: &'static str,
    /// `_PROFILES["USN"][2]`.
    minus_word: &'static str,
    /// `self.profile = chosen`. The `__init__` `ValueError` guard for an
    /// unknown profile cannot fire: `PROFILE = "USN"` is a class constant and
    /// `_PROFILES` contains it.
    #[allow(dead_code)]
    profile: &'static str,
}

impl Default for LangEnAeroUsn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnAeroUsn {
    pub fn new() -> Self {
        LangEnAeroUsn {
            english: LangEn::new(),
            digit_table: ICAO_DIGITS,
            decimal_word: ICAO_DECIMAL,
            minus_word: ICAO_MINUS,
            profile: "USN",
        }
    }

    /// `Num2Word_Base.verify_ordinal`, inherited unchanged through
    /// `Num2Word_EN` and `Num2Word_EN_AERO`.
    ///
    /// ```python
    /// if not value == int(value):   raise TypeError(errmsg_floatord % value)
    /// if not abs(value) == value:   raise TypeError(errmsg_negord % value)
    /// ```
    ///
    /// The float check is vacuous for `BigInt` (`value == int(value)` always
    /// holds), so only the negative check survives. Note `abs(0) == 0`, so
    /// zero passes — corpus: `ordinal(0)` → "zeroth", `ordinal_num(0)` → "0".
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == Sign::Minus {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_EN_AERO._digits_of`, restricted to the integer path.
    ///
    /// Python normalises to `(is_negative, int_part, frac_part)`. For `BigInt`
    /// the `str(value)` branch applies: the `Decimal`/`str` branches, the
    /// `","`/`"_"` separator stripping, the `"."` split and the empty-int_part
    /// backfill (`int_part = "0"`) are all unreachable — `BigInt::to_string`
    /// never emits a separator, a point, or an empty integer part (`0` → "0").
    /// So `frac_part` is always "" and the decimal branch of `to_cardinal`
    /// never runs.
    fn digits_of(&self, value: &BigInt) -> (bool, String) {
        let s = value.to_string();
        match s.strip_prefix('-') {
            Some(rest) => (true, rest.to_string()),
            None => (false, s),
        }
    }

    /// `Num2Word_EN_AERO._digits_of`, the float / `Decimal` arms.
    ///
    /// Returns `(is_negative, int_part, frac_part)` exactly as Python's
    /// `_digits_of` does, so the digit reader in `to_cardinal_float` can be a
    /// byte-for-byte twin of the integer `to_cardinal`.
    ///
    /// * **Float** — Python does `s = str(value)`; [`python_float_str`]
    ///   reproduces that exact string. Usually it is fixed-point with
    ///   `precision` fractional digits, so `format!("{:.precision$}")` of the
    ///   raw `f64` equals it (`str(1.0)` → "1.0", `str(2.675)` → "2.675"); when
    ///   Python's `repr` uses exponent notation the `e`/sign/exponent chars are
    ///   rebuilt so the digit reader sees them (`str(1e16)` → "1e+16" →
    ///   "wun wun six"). The leading "-" is stripped off the string, matching
    ///   `_digits_of`, so `-0.0` reads as negative just as `str(-0.0)` →
    ///   "-0.0" would.
    /// * **Decimal** — Python does `s = format(value, "f")`: fixed-point with
    ///   the coefficient's own scale, trailing zeros kept. Rescaling the
    ///   `BigDecimal` to `precision` (= `abs(exponent)`) fractional digits
    ///   recovers that coefficient without a lossy `float()` cast — the #603
    ///   guarantee — and the point is placed `precision` digits from the right.
    ///
    /// An empty integer part backfills to "0" (`_digits_of`'s `if not
    /// int_part: int_part = "0"`).
    fn float_digits_of(&self, v: &FloatValue) -> (bool, String, String) {
        match v {
            FloatValue::Float { value, precision } => {
                // Python: `s = str(value)`, read character-by-character.
                let s = python_float_str(*value, *precision);
                let (is_neg, s) = match s.strip_prefix('-') {
                    Some(rest) => (true, rest.to_string()),
                    None => (false, s),
                };
                match s.split_once('.') {
                    Some((int_part, frac_part)) => {
                        let int_part = if int_part.is_empty() {
                            "0".to_string()
                        } else {
                            int_part.to_string()
                        };
                        (is_neg, int_part, frac_part.to_string())
                    }
                    None => {
                        let int_part = if s.is_empty() { "0".to_string() } else { s };
                        (is_neg, int_part, String::new())
                    }
                }
            }
            FloatValue::Decimal { value, .. } => {
                // Python's `format(value, "f")` renders with the Decimal's
                // OWN scale: a positive scale keeps its trailing zeros, and a
                // non-positive scale (an exponential spelling such as
                // `Decimal("1E+2")`, or an integral `Decimal("5")`) expands
                // to plain integer digits with NO fractional part — "100",
                // never "100.00". The shim's `precision` is `abs(exponent)`,
                // which for `1E+2` would wrongly grow two fractional zeros,
                // so the scale is read off the value itself.
                let (_, own_scale) = value.as_bigint_and_exponent();
                let scaled = value.with_scale(own_scale.max(0));
                let mantissa = scaled.as_bigint_and_exponent().0;
                let mant_str = mantissa.to_string();
                let (is_neg, mag) = match mant_str.strip_prefix('-') {
                    Some(rest) => (true, rest.to_string()),
                    None => (false, mant_str),
                };
                let prec = own_scale.max(0) as usize;
                if prec == 0 {
                    let int_part = if mag.is_empty() { "0".to_string() } else { mag };
                    (is_neg, int_part, String::new())
                } else {
                    // Left-pad so the point lands after at least one integer
                    // digit: "1" @ scale 2 → "001" → "0.01".
                    let mag = if mag.len() <= prec {
                        format!("{}{}", "0".repeat(prec + 1 - mag.len()), mag)
                    } else {
                        mag
                    };
                    let split = mag.len() - prec;
                    (is_neg, mag[..split].to_string(), mag[split..].to_string())
                }
            }
        }
    }
}

/// Reproduce CPython's `str(float)` / `repr(float)` for a value AERO reads
/// *lexically*.
///
/// AERO's `to_cardinal` turns every digit *character* of `str(value)` into a
/// word, splits the fractional part on `.`, and takes a leading `-` as the
/// sign — so the exact string form is load-bearing, scientific notation and
/// all. `str(1e16)` is `"1e+16"`, which AERO reads as "wun wun six" (the `1` of
/// the mantissa, then the `1` and `6` of the exponent); `str(1e-5)` is
/// `"1e-05"` → "wun zero fife". A fixed-point reconstruction cannot produce
/// those, so the exponent form is rebuilt here.
///
/// CPython's `format_float_short` (format code `'r'`) switches to exponent
/// notation exactly when `decpt <= -4 || decpt > 16`, where `decpt` is the
/// number of digits before the decimal point (the shortest-repr exponent + 1).
/// Below that threshold the shortest decimal has `precision` fractional digits
/// and `format!("{:.precision$}")` of the same `f64` equals it byte-for-byte —
/// the proven path every corpus float takes — so the string is only rebuilt
/// when Python itself would have used the `e` form. Non-finite and zero inputs
/// (unreachable via `num2words`, which never forwards them here) fall through
/// to the same fixed-point format, where `inf`/`nan`/`-0.0` already print as
/// Python's `str` does.
fn python_float_str(value: f64, precision: u32) -> String {
    let a = value.abs();
    if a.is_finite() && a != 0.0 {
        let (digits, decpt) = shortest_digits(a);
        if decpt <= -4 || decpt > 16 {
            // Mantissa: first significant digit, then the rest after a point
            // (the point is dropped when there is only one digit, as CPython
            // does: `1e+16`, not `1.e+16`).
            let d1 = &digits[..1];
            let rest = &digits[1..];
            let mant = if rest.is_empty() {
                d1.to_string()
            } else {
                format!("{}.{}", d1, rest)
            };
            // Exponent: the printed value is `decpt - 1`; the sign is always
            // shown and the magnitude is zero-padded to at least two digits
            // (`e+16`, `e-05`, `e+100`).
            let exp = decpt - 1;
            let sign = if exp < 0 { '-' } else { '+' };
            let body = format!("{}e{}{:02}", mant, sign, exp.abs());
            return if value.is_sign_negative() {
                format!("-{}", body)
            } else {
                body
            };
        }
    }
    // Fixed notation (and unreachable non-finite / zero): `precision` is the
    // repr-derived fractional-digit count, so this is Python's `str` verbatim.
    format!("{:.*}", precision as usize, value)
}

/// Shortest round-trip significant digits of `a` (which must be finite and
/// non-zero), plus CPython's `decpt` — the count of digits before the decimal
/// point. Rust's `{:e}` is shortest round-trip, the same digit string David
/// Gay's `dtoa(mode 0)` yields for CPython, so its mantissa digits and
/// `exponent + 1` give `(digits, decpt)` directly (`1.5e16` → ("15", 17),
/// `1e-5` → ("1", -4)).
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("finite non-zero {:e} always contains 'e'");
    let e: i32 = exp.parse().expect("{:e} exponent is a valid i32");
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, e + 1)
}

impl Lang for LangEnAeroUsn {

    /// `maxval()` is None (gladiaio/num2words2#159): the cardinal reads digit
    /// by digit with no ceiling, so the inherited `Num2Word_EN.MAXVAL` (10^306)
    /// is not reported — the cardinal happily converts past it.
    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        None
    }
    /// `self.pointword`, read from the live Python instance (inherited from
    /// `Num2Word_Base`). Kept for fidelity, but AERO never speaks it: the
    /// integer modes don't reach it, and the float path uses `decimal_word`
    /// ("decimal") instead — AERO's radix word is "decimal", not "point".
    fn pointword(&self) -> &str {
        "point"
    }

    // cards() / maxval() / merge() intentionally left at their trait defaults:
    // AERO overrides to_cardinal and never reaches the base engine. In
    // particular maxval() stays 0, which is correct-by-omission — the Python
    // override skips Num2Word_Base's `if value >= self.MAXVAL` check, so no
    // input overflows.

    /// `Num2Word_EN_AERO.to_cardinal` — digit-by-digit, never composite.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (is_negative, int_part) = self.digits_of(value);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(self.minus_word);
        }
        for ch in int_part.chars() {
            // Python: `if ch.isdigit(): words.append(self._digit_table[ch])`.
            // Non-digits are silently dropped rather than raising KeyError.
            if let Some(d) = ch.to_digit(10) {
                words.push(self.digit_table[d as usize]);
            }
        }
        // The `if frac_part:` branch is dead for BigInt input — see digits_of.
        Ok(words.join(" "))
    }

    /// `Num2Word_EN_AERO.to_ordinal` → `self._english.to_ordinal(value)`.
    ///
    /// Deliberately *not* self.to_cardinal: plain English, so "third" not
    /// "treeth". `LangEn::to_ordinal` runs its own `verify_ordinal`, which is
    /// where negatives become `TypeError` — matching Python, where the raise
    /// also comes from the `_english` sibling rather than from AERO.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.english.to_ordinal(value)
    }

    /// `Num2Word_EN_AERO.to_ordinal_num` — `verify_ordinal` then
    /// `str(int(value))`.
    ///
    /// This *shadows* `Num2Word_EN.to_ordinal_num`, so the output is bare
    /// digits ("42"), not the English suffixed form ("42nd"). Unlike
    /// `to_ordinal`, this one calls `self.verify_ordinal` — AERO's own
    /// (inherited from Base) — not the sibling's.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// `Num2Word_EN_AERO.to_year` → `self.to_cardinal(value)`.
    ///
    /// Aviation reads years digit-by-digit, so this discards *everything* EN's
    /// `to_year` does: no century split, no "oh-" infix, and no "BC" suffix
    /// for negatives (the sign reaches the digit reader and becomes "minus").
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries ----------------------------------------

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// AERO's `to_cardinal` reads the value's *string* form, so `5.0` keeps
    /// its ".0" tail ("fife decimal zero") — never the trait default's
    /// whole-value integer route. The `Decimal("Infinity")`/`("NaN")`
    /// sentinels (see `lang_en_aero::aero_str_to_number`) render as
    /// `_digits_of` reads them: no digit chars, only the sign word —
    /// "" / "minus" / "" (all three corpus rows).
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
    /// The delegate's `verify_ordinal` polices the type: whole values
    /// ordinalise in plain English (`5.0` → "fifth", `-0.0` → "zeroth");
    /// fractional or negative values raise TypeError. The Infinity/NaN
    /// sentinels reproduce the `int(value)` raise inside that comparison.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(sp) = aero_special_of(value) {
            return Err(sp.int_error());
        }
        self.english.ordinal_float_entry(value)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` — AERO's own,
    /// i.e. Base's — then `str(int(value))`. Bare truncated digits: `5.00` →
    /// "5", `1e+16` → "10000000000000000", `-0.0` → "0". Float-ness is
    /// checked before sign (`-1.5` raises the *float* message); `%s`
    /// interpolates `str(value)` = `repr_str`.
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

    /// `converter.str_to_number` — base `Decimal(value)` semantics, with
    /// Infinity/NaN carried through as sentinels (see `lang_en_aero`)
    /// because AERO's string-reading cardinal *succeeds* on them.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        aero_str_to_number(s)
    }

    /// The float / `Decimal` cardinal path.
    ///
    /// AERO has no `to_cardinal_float`; Python routes non-integral values
    /// through its overridden `to_cardinal`, which reads the value's string
    /// form digit-by-digit. This override is that `to_cardinal`, generalised to
    /// the fractional part — a twin of the integer method, plus a "decimal"
    /// separator and a "minus"-from-the-string sign. It never consults
    /// `float2tuple`, so the binary-artefact heuristic is irrelevant here.
    ///
    /// `precision_override` is ignored: Python's `to_cardinal` reads
    /// `str(value)` and never looks at `self.precision`, so the `precision=`
    /// kwarg has no effect on AERO cardinals.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let (is_negative, int_part, frac_part) = self.float_digits_of(value);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(self.minus_word);
        }
        for ch in int_part.chars() {
            // Non-digits are silently dropped, as Python's `if ch.isdigit()`
            // guard does; our reconstructed parts are already pure digits.
            if let Some(d) = ch.to_digit(10) {
                words.push(self.digit_table[d as usize]);
            }
        }
        // Python: `if frac_part:` — only emit "decimal" when digits follow.
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

    // ---- currency --------------------------------------------------------
    //
    // `Num2Word_EN_AERO.to_currency` / `.to_cheque` are pure `*args` pass-
    // throughs to `self._english`. Both hooks below are the same one line:
    // hand the call to the sibling and let *its* forms table, `pluralize`,
    // `CURRENCY_PRECISION`, `lang_name` and `to_cardinal` answer. See the
    // module docs for why re-declaring any of those here would be a bug and
    // not a convenience.

    /// `Num2Word_EN_AERO.to_currency` → `self._english.to_currency(*args)`.
    ///
    /// The `CurrencyValue::Int` / `::Decimal` split rides through untouched —
    /// Python forwards `*args` without inspecting them, so the
    /// `isinstance(val, int)` branch is taken inside `_english.to_currency`
    /// exactly as it would be for a plain `Num2Word_EN`. Corpus:
    /// `currency(1, "EUR")` → "one euro" (int, no cents segment) vs
    /// `currency(1.0, "EUR")` → "one euro, zero cents" (float, cents shown
    /// though zero).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Keep the Infinity/NaN sentinels out of the delegate's arithmetic
        // (no corpus row reaches this; the delegate raises from an int()
        // cast in Python, same types as here).
        if let CurrencyValue::Decimal { value: d, .. } = val {
            if let Some(sp) = aero_special_of_decimal(d) {
                return Err(sp.int_error());
            }
        }
        self.english
            .to_currency(val, currency, cents, separator, adjective)
    }

    /// `Num2Word_EN_AERO.to_cheque` → `self._english.to_cheque(*args)`.
    ///
    /// Note `Num2Word_Base.to_cheque` calls `self._money_verbose`, so this
    /// delegation is what keeps the cheque body composite: corpus
    /// `cheque(1234.56, "EUR")` → "ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR
    /// AND 56/100 EUROS", not a digit-by-digit reading.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        self.english.to_cheque(val, currency)
    }
}
