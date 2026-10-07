//! Port of `lang_EN_AERO.py`'s `Num2Word_EN_AERO_US_Army` (ICAO/aviation
//! English, US Army profile — registry key `en_Aero_US_Army`).
//!
//! Shape: **self-contained** for `to_cardinal`/`to_year`, but *delegating*
//! for `to_ordinal`. `Num2Word_EN_AERO` subclasses `Num2Word_EN`, so the full
//! EN engine (cards/merge/MAXVAL, built via `lang_EUR` → `Num2Word_Base`) is
//! inherited and still present on the instance — but `to_cardinal` is
//! overridden outright with a digit-by-digit reader that never consults
//! `self.cards`. Hence `cards`/`maxval`/`merge` stay at their trait defaults
//! here and there is **no OverflowError path** on the cardinal side: the
//! inherited EN `MAXVAL` (10^306) is unreachable because `splitnum` is never
//! called. `to_cardinal(10**400)` happily emits 401 digit words.
//!
//! # The `_english` sibling instance — why this file uses `LangEn`
//!
//! `Num2Word_EN_AERO.__init__` stashes a *separate* plain-English converter:
//!
//! ```python
//! self._english = Num2Word_EN()
//! ```
//!
//! and `to_ordinal` forwards to it rather than to `super()`. This is
//! deliberate (the docstring spells it out): routing ordinals through the
//! inherited EN `to_ordinal` would let the digit-by-digit cardinal leak into
//! the ordinal builder and produce "treeth" instead of "third", because EN's
//! `to_ordinal` is built as `to_cardinal(value)` + suffix-fixup on the last
//! word. Forwarding to a pristine `Num2Word_EN` sidesteps the override.
//!
//! This port mirrors that structure exactly by holding a `crate::lang_en::LangEn`
//! field. It is a *read* of a sibling language module, not an edit — the
//! `_english` handshake is load-bearing and reimplementing the EN engine here
//! would be a rewrite, not a port.
//!
//! # Modes (all four in scope, and where each comes from)
//!
//! | mode          | defined in         | behaviour                                  |
//! |---------------|--------------------|--------------------------------------------|
//! | `to_cardinal` | `Num2Word_EN_AERO` | digit-by-digit ICAO respelling             |
//! | `to_year`     | `Num2Word_EN_AERO` | `return self.to_cardinal(value)` — verbatim |
//! | `to_ordinal`  | `Num2Word_EN_AERO` | `self._english.to_ordinal(value)` — plain EN |
//! | `to_ordinal_num` | `Num2Word_EN_AERO` | `verify_ordinal` then `str(int(value))`  |
//! | `to_currency` | `Num2Word_EN_AERO` | `self._english.to_currency(*a, **kw)` — plain EN |
//! | `to_cheque`   | `Num2Word_EN_AERO` | `self._english.to_cheque(*a, **kw)` — plain EN |
//!
//! # Currency: the whole surface is a forward to `_english`
//!
//! ```python
//! def to_currency(self, *args, **kwargs):
//!     return self._english.to_currency(*args, **kwargs)
//!
//! def to_cheque(self, *args, **kwargs):
//!     return self._english.to_cheque(*args, **kwargs)
//! ```
//!
//! Same rationale as `to_ordinal`, and the corpus pins it hard: `to_cardinal`
//! of 1234 is "wun too tree fower", but `currency:EUR` of 1234.56 is "one
//! thousand, two hundred and thirty-four euros, fifty-six cents". Money is
//! read as composite English, not digit-by-digit — because `_money_verbose` /
//! `_cents_verbose` resolve on the *pristine `Num2Word_EN` instance*, whose
//! `to_cardinal` was never overridden. All 117 `en_Aero_US_Army`
//! currency+cheque corpus rows are byte-identical to the plain `en` rows for
//! the same `(to, arg)`, and none of them raise.
//!
//! So this file overrides exactly the two hooks Python overrides and forwards
//! them. It deliberately does **not** define `currency_forms`,
//! `currency_precision`, `pluralize`, `money_verbose`, `cents_verbose` or
//! `cents_terse` on `LangEnAeroUsArmy`:
//!
//! * There is no table to build here. `CURRENCY_FORMS` / `CURRENCY_PRECISION`
//!   do exist on the Python AERO instance (`super().__init__()` runs EN's
//!   `__init__`, which populates them), but they are **dead** — nothing on the
//!   class ever reads them, because both entry points hand off to `_english`
//!   first. Mirroring them here would be a second copy of EN's ~30-entry table
//!   that no code path can reach: pure drift risk.
//! * The table *is* still built once, in `LangEn::new()`, which this struct
//!   calls once from its own `new()` and stores. No per-call construction.
//! * Overriding `currency_forms` + `pluralize` on `self` and letting
//!   `default_to_currency` run against `self` would be actively **wrong**: the
//!   default `money_verbose` calls `self.to_cardinal`, which here is the
//!   digit-by-digit reader — it would emit "wun too tree fower euros".
//!
//! One quirk falls out of the forward for free: the NotImplementedError for an
//! unknown code names **`Num2Word_EN`**, not `Num2Word_EN_AERO_US_Army`, since
//! Python formats `self.__class__.__name__` on the `_english` instance that
//! actually raises. Delegating reproduces that, as long as `LangEn::lang_name`
//! reports `"Num2Word_EN"`. No corpus row exercises it (every AERO currency row
//! succeeds), so it is unverified here — see the report's `concerns`.
//!
//! # Behaviour worth flagging (faithfully reproduced, not "fixed")
//!
//! 1. **`to_year` drops the BC suffix.** EN's `to_year` maps negative years to
//!    an `abs()` value plus a "BC" suffix and applies century phrasing
//!    ("nineteen eighty-four"). AERO overrides `to_year` with a bare
//!    `to_cardinal` call, so *all* of that is discarded: `to_year(-500)` is
//!    "minus fife zero zero", **not** "fife zero zero BC", and `to_year(1984)`
//!    is "wun niner ait fower", not century-paired. Corpus-confirmed.
//! 2. **`to_ordinal_num` returns a bare number, no suffix.** AERO overrides
//!    EN's `"%s%s" % (value, self.to_ordinal(value)[-2:])` with plain
//!    `str(int(value))`, so `to_ordinal_num(3)` is `"3"`, **not** `"3rd"`.
//!    That looks like an oversight (the ICAO rationale for bypassing ordinals
//!    doesn't apply to a numeric suffix), but it is exactly what Python emits
//!    and the corpus pins it for every value tested. Preserved verbatim.
//! 3. **Mixed registers.** `to_cardinal(3)` is "tree" while `to_ordinal(3)` is
//!    "third" — the two modes speak different dialects by design.
//! 4. **The `profile` parameter is inert.** `_PROFILES` maps all five keys
//!    (ICAO/FAA/USN/US_ARMY/NATO) to the *same* `_ICAO_DIGITS` table, so this
//!    class is byte-for-byte identical to `Num2Word_EN_AERO` and its three
//!    siblings today. The `ValueError` for an unknown profile is unreachable
//!    from the registry (`PROFILE = "US_ARMY"` is a valid key), so `new()` is
//!    infallible. Modelled as a plain constructor rather than a checked one.

use crate::base::{Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::lang_en::LangEn;
use crate::lang_en_aero::{
    python_float_repr,
};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign};
use num_traits::Signed;

/// `_ICAO_DIGITS` — the per-profile digit respelling table.
///
/// `US_ARMY` resolves to this same table (see note 4 in the module docs).
/// Kept as a `match` rather than a map: Python's `self._digit_table[ch]` is a
/// dict lookup guarded by `if ch.isdigit()`, and on integer input the only
/// characters that ever reach it are ASCII `0`-`9`.
fn icao_digit(ch: char) -> Option<&'static str> {
    Some(match ch {
        '0' => "zero",
        '1' => "wun",
        '2' => "too",
        '3' => "tree",
        '4' => "fower",
        '5' => "fife",
        '6' => "six",
        '7' => "seven",
        '8' => "ait",
        '9' => "niner",
        _ => return None,
    })
}

/// `_ICAO_MINUS`.
const MINUS_WORD: &str = "minus";

/// `_ICAO_DECIMAL`. The AERO float reader emits *this*, not `self.pointword`
/// ("point") — `to_cardinal` joins `self._decimal_word` between the integer
/// and fractional digit runs. So the float path is spelled "decimal" and the
/// inherited `pointword` is dead for this class.
const DECIMAL_WORD: &str = "decimal";

/// Shared tail of `Num2Word_EN_AERO.to_cardinal` once `_digits_of` has
/// normalised the input to `(sign, integer-digits, fractional-digits)`.
///
/// ```python
/// words = []
/// if is_negative:
///     words.append(self._minus_word)
/// for ch in int_part:
///     if ch.isdigit():
///         words.append(self._digit_table[ch])
/// if frac_part:
///     words.append(self._decimal_word)
///     for ch in frac_part:
///         if ch.isdigit():
///             words.append(self._digit_table[ch])
/// return " ".join(words)
/// ```
///
/// The `ch.isdigit()` guard is kept structurally via `icao_digit` returning
/// `Option` — on the digit strings we build here every char is ASCII `0`-`9`,
/// so nothing is ever actually skipped, exactly as in Python for numeric input.
fn render_digit_words(is_negative: bool, int_part: &str, frac_part: &str) -> String {
    let mut words: Vec<&str> = Vec::new();
    if is_negative {
        words.push(MINUS_WORD);
    }
    for ch in int_part.chars() {
        if let Some(w) = icao_digit(ch) {
            words.push(w);
        }
    }
    if !frac_part.is_empty() {
        words.push(DECIMAL_WORD);
        for ch in frac_part.chars() {
            if let Some(w) = icao_digit(ch) {
                words.push(w);
            }
        }
    }
    words.join(" ")
}

pub struct LangEnAeroUsArmy {
    /// Mirrors `self._english = Num2Word_EN()` — the sibling converter that
    /// `to_ordinal` delegates to so the digit-by-digit cardinal cannot leak in.
    english: LangEn,
}

impl Default for LangEnAeroUsArmy {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnAeroUsArmy {
    pub fn new() -> Self {
        LangEnAeroUsArmy {
            english: LangEn::new(),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// Python checks `value == int(value)` (float guard — vacuous for `BigInt`)
    /// and then `abs(value) == value`, raising `TypeError` with
    /// `errmsg_negord = "Cannot treat negative num %s as ordinal."`.
    ///
    /// AERO does not override this, and neither does EN, so `to_ordinal_num`
    /// reaches the base implementation directly. `LangEn` has its own private
    /// copy that this cannot call across module boundaries, so the check is
    /// reproduced here — the message and type match `base.py` exactly.
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

impl Lang for LangEnAeroUsArmy {

    /// `maxval()` is None (gladiaio/num2words2#159): the cardinal reads digit
    /// by digit with no ceiling, so the inherited `Num2Word_EN.MAXVAL` (10^306)
    /// is not reported — the cardinal happily converts past it.
    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        None
    }
    // cards / maxval / merge intentionally left at their trait defaults: the
    // Python class inherits EN's engine but `to_cardinal` never touches it.

    fn negword(&self) -> &str {
        // Inherited from EN's setup(). Unused by this class's to_cardinal,
        // which joins `_minus_word` ("minus", no trailing space) itself.
        "minus "
    }

    fn pointword(&self) -> &str {
        "point"
    }

    /// Port of `Num2Word_EN_AERO.to_cardinal` over `_digits_of`.
    ///
    /// Python normalises via `_digits_of`, which for a non-str/non-Decimal
    /// value does `s = str(value)`, splits a leading `"-"`, strips `,`/`_`
    /// thousands separators, and splits on `"."`. On `BigInt` input:
    ///
    /// * `to_string()` is always `-?[0-9]+`, so the separator-stripping, the
    ///   `"."` split, the empty-`int_part` fallback (`int_part = "0"`), and the
    ///   whole `frac_part` branch (`_ICAO_DECIMAL` = "decimal") are all
    ///   unreachable. The float/Decimal path is out of scope per the contract.
    /// * every character is ASCII `0`-`9`, so `ch.isdigit()` is always true and
    ///   the `_digit_table[ch]` lookup can never `KeyError`. (It *could* in
    ///   Python: `str.isdigit()` accepts non-ASCII digits like `"٣"` or `"²"`,
    ///   which are absent from `_ICAO_DIGITS` — but only string input reaches
    ///   that, and string input is not this port's concern.)
    ///
    /// The `if ch.isdigit()` filter is kept structurally via `icao_digit`
    /// returning `Option`, so a non-digit would be silently skipped exactly as
    /// Python skips it, rather than panicking.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let s = value.to_string();
        let (is_negative, int_part) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s.as_str()),
        };

        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(MINUS_WORD);
        }
        for ch in int_part.chars() {
            if let Some(w) = icao_digit(ch) {
                words.push(w);
            }
        }
        Ok(words.join(" "))
    }

    /// Port of `Num2Word_EN_AERO.to_ordinal`: `self._english.to_ordinal(value)`.
    ///
    /// Delegates to the pristine EN converter — note this means the negative
    /// `TypeError` comes from *`_english`'s* `verify_ordinal`, not this
    /// instance's. Same type and message either way.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.english.to_ordinal(value)
    }

    /// Port of `Num2Word_EN_AERO.to_ordinal_num`:
    /// `self.verify_ordinal(value); return str(int(value))`.
    ///
    /// No ordinal suffix — see note 2 in the module docs.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// Port of `Num2Word_EN_AERO.to_year`: `return self.to_cardinal(value)`.
    ///
    /// The `**kwargs` (EN's `suffix`/`longval`) are accepted and ignored by
    /// Python; there is no BC handling and no century phrasing. See note 1.
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


    /// The float/Decimal cardinal path.
    ///
    /// # Which Python method this ports
    ///
    /// `Num2Word_EN_AERO` does **not** override `to_cardinal_float`; it
    /// inherits `Num2Word_Base`'s. But that inherited method is never reached,
    /// because AERO overrides `to_cardinal` to intercept *every* input —
    /// integer, float, and `Decimal` alike — through `_digits_of`, its own
    /// string normaliser. The base `to_cardinal` would dispatch non-integers to
    /// `to_cardinal_float`; AERO's does not. So the effective float behaviour
    /// lives in `to_cardinal`/`_digits_of`, and this is the method the Rust
    /// dispatcher routes float/Decimal input to — hence the port lands here.
    ///
    /// # What `_digits_of` does (the spec)
    ///
    /// ```python
    /// if isinstance(value, str):   s = value.strip()
    /// elif isinstance(value, Decimal): s = format(value, "f")
    /// else:                        s = str(value)
    /// is_negative = s.startswith("-"); s = s.lstrip("-")
    /// s = s.replace(",", "").replace("_", "")
    /// int_part, frac_part = s.split(".", 1) if "." in s else (s, "")
    /// if not int_part: int_part = "0"
    /// ```
    ///
    /// This is a *string* reading of the value — it never touches
    /// `float2tuple`, `self.precision`, banker's rounding, or the `< 0.01`
    /// artefact heuristic. Two consequences that make this port unlike the 26
    /// that inherit `default_to_cardinal_float`:
    ///
    /// * **The separator word is "decimal", not `pointword`** — `_ICAO_DECIMAL`
    ///   (see [`DECIMAL_WORD`]), and the sign word is `_ICAO_MINUS` ("minus"),
    ///   prepended whenever the value is negative (not only when the integer
    ///   part is zero, which is the base path's rule).
    /// * **The `precision=` kwarg is inert.** AERO's `to_cardinal` reads
    ///   `str(value)` and never consults `self.precision`, so
    ///   `precision_override` cannot change the output. It is accepted and
    ///   ignored here, matching Python (the dispatcher sets `self.precision`,
    ///   which AERO's reader never looks at).
    ///
    /// # Reconstructing `str(float)` / `format(Decimal, "f")`
    ///
    /// * **Float arm.** `FloatValue::Float.precision` is
    ///   `abs(Decimal(repr(value)).as_tuple().exponent)`, i.e. exactly the
    ///   number of fractional digits `str(value)` prints. Formatting the raw
    ///   f64 to that many places, `format!("{:.p}", value.abs())`, reproduces
    ///   `str(value)` for every non-scientific magnitude (Rust `{:.p}` and
    ///   Python `repr` are both correctly-rounded shortest/fixed forms and
    ///   agree here — verified against the frozen corpus and the live
    ///   interpreter). Floats always carry at least one fractional digit
    ///   (`str(1.0) == "1.0"`), so the "decimal" run is always present.
    /// * **Decimal arm.** `format(Decimal, "f")` prints the value in
    ///   fixed-point with exactly `scale` (= `precision`) fractional digits,
    ///   trailing zeros preserved — which is why `Decimal("1.10")` reads
    ///   "wun decimal wun zero". Since `precision` carries that scale
    ///   (including trailing zeros, which a BigDecimal may have normalised
    ///   away), the digit run is rebuilt from `abs(value) * 10**precision` as
    ///   an exact integer, split so the fractional part is exactly `precision`
    ///   digits wide (zero-padded on the left).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => {
                // Python's `_digits_of` derives the sign from
                // `str(value).startswith("-")`, so negative zero counts as
                // negative (`str(-0.0) == "-0.0"`). `*value < 0.0` is `false`
                // for `-0.0`; `is_sign_negative()` mirrors the string test.
                let is_negative = value.is_sign_negative();
                // Reproduce `str(value)` of the magnitude — fixed-point with
                // `precision` frac digits in the normal range, CPython's
                // scientific form outside `[1e-4, 1e16)` (`str(1e16)` ==
                // "1e+16" → "wun wun six", the 'e'/'+' silently skipped by
                // `icao_digit`) — then split on the point as `_digits_of`
                // does.
                let s = python_float_repr(value.abs(), *precision);
                let (int_part, frac_part) = match s.split_once('.') {
                    Some((i, f)) => (i, f),
                    None => (s.as_str(), ""),
                };
                Ok(render_digit_words(is_negative, int_part, frac_part))
            }
            FloatValue::Decimal { value, .. } => {
                let is_negative = value.is_negative();
                // `format(value, "f")` is driven by the Decimal's own
                // exponent, *not* by the repr-derived `precision` (which is
                // its absolute value and cannot tell `1E+2` from `1.00`). A
                // non-positive scale — the source used exponent form,
                // `Decimal("1E+2")` — expands to trailing zeros with NO
                // fractional part: format 'f' gives "100", never "100.00". A
                // positive scale keeps exactly that many fractional digits,
                // trailing zeros included (`Decimal("1.10")` → "1.10"). A
                // zero coefficient collapses to a bare "0" whatever the
                // exponent (`format(Decimal("0E+2"), "f")` == "0").
                let (digits, scale) = value.abs().as_bigint_and_exponent();
                if scale <= 0 {
                    let int_part = if num_traits::Zero::is_zero(&digits) {
                        "0".to_string()
                    } else {
                        format!("{}{}", digits, "0".repeat((-scale) as usize))
                    };
                    Ok(render_digit_words(is_negative, &int_part, ""))
                } else {
                    let s = digits.to_string(); // non-negative: no sign char
                    let p = scale as usize;
                    let (int_part, frac_part) = if s.len() <= p {
                        // Fewer digits than the fractional width: integer
                        // part is "0", fraction left-zero-padded to `scale`.
                        ("0".to_string(), crate::strnum::zero_pad_left(&s, p))
                    } else {
                        let split = s.len() - p;
                        (s[..split].to_string(), s[split..].to_string())
                    };
                    Ok(render_digit_words(is_negative, &int_part, &frac_part))
                }
            }
        }
    }

    // ---- currency ----------------------------------------------------
    //
    // Both hooks forward to the `_english` sibling, exactly as Python does.
    // Every other currency hook is intentionally left at its trait default:
    // routing through `_english` means none of them is ever consulted on
    // `self`. See the "Currency" section of the module docs.

    /// Port of `Num2Word_EN_AERO.to_currency`:
    /// `return self._english.to_currency(*args, **kwargs)`.
    ///
    /// The forward is total — `currency`, `cents`, `separator` and `adjective`
    /// are `*args`/`**kwargs` in Python and reach `Num2Word_EN` untouched, so
    /// the `CurrencyValue` int/decimal split, the `CURRENCY_ADJECTIVES` prefix
    /// and the mils divisor are all EN's to interpret. Nothing to re-derive.
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

    /// Port of `Num2Word_EN_AERO.to_cheque`:
    /// `return self._english.to_cheque(*args, **kwargs)`.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        self.english.to_cheque(val, currency)
    }
}
