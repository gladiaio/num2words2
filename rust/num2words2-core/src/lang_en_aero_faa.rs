//! Port of `lang_EN_AERO.py` → `Num2Word_EN_AERO_FAA` (registry key `en_Aero_FAA`).
//!
//! Shape: **self-contained**. `Num2Word_EN_AERO` subclasses `Num2Word_EN` (and
//! therefore runs EN's `setup()`, building EN's full card table on `self`), but
//! it overrides `to_cardinal` outright with a digit-by-digit reader. The
//! inherited cards/`merge`/`MAXVAL` are consequently **dead** for all four
//! in-scope modes, so `cards`/`maxval`/`merge` stay at their trait defaults
//! here. There is no overflow check on any path — ICAO reading is per-character,
//! so 10^606 is as cheap as 7.
//!
//! # What `en_Aero_FAA` actually resolves to
//!
//! `CONVERTER_CLASSES["en_Aero_FAA"]` is `Num2Word_EN_AERO_FAA()`, a subclass of
//! `Num2Word_EN_AERO` whose only content is `PROFILE = "FAA"`. `__init__` looks
//! that up in `_PROFILES`, which maps **every** profile (ICAO / FAA / USN /
//! US_ARMY / NATO) to the *same* `(_ICAO_DIGITS, "decimal", "minus")` triple.
//! So `en_Aero_FAA` is byte-for-byte identical to `en_Aero_ICAO`, `en_Aero_USN`,
//! `en_Aero_US_Army` and `en_Aero_NATO` — the module says so explicitly
//! ("Today the four profiles below are intentionally identical"). The profile
//! is inlined below rather than kept as a table, since the FAA row is fixed.
//!
//! # The four in-scope modes
//!
//! | mode | Python | effect |
//! |---|---|---|
//! | `to_cardinal` | overridden in `Num2Word_EN_AERO` | digit-by-digit ICAO |
//! | `to_year` | `return self.to_cardinal(value)` | digit-by-digit ICAO |
//! | `to_ordinal` | `return self._english.to_ordinal(value)` | **plain English** |
//! | `to_ordinal_num` | `verify_ordinal(value); return str(int(value))` | bare digits |
//!
//! Two of these are easy to get wrong, so they are called out:
//!
//! 1. **`to_year` does not use EN's year logic at all.** `Num2Word_EN.to_year`
//!    (century-splitting, "oh-", the `BC` suffix for negatives) is entirely
//!    bypassed — AERO's one-line override delegates to its own digit-by-digit
//!    `to_cardinal`. So `to_year(-500)` is `"minus fife zero zero"`, **not**
//!    `"fife zero zero BC"`: the sign is rendered by the cardinal's minus-word
//!    and no `BC` is ever appended. Confirmed against the frozen corpus.
//! 2. **`to_ordinal` delegates to a *sibling* `Num2Word_EN` instance**
//!    (`self._english`), not to `super()` and not to `self`. This is deliberate
//!    and documented in the Python: routing through `self` would let the
//!    digit-by-digit cardinal leak into EN's ordinal builder and produce
//!    "treeth" instead of "third". Mirrored here by holding a real
//!    [`LangEn`] and forwarding to it, so `to_ordinal` inherits EN's ordinal
//!    behaviour (including its `verify_ordinal` TypeError on negatives) for
//!    free and cannot drift from it.
//!
//! `to_ordinal_num` **overrides** `Num2Word_EN.to_ordinal_num`, which would have
//! produced "1st"/"2nd". AERO returns the bare decimal string instead — `str(1)`
//! is `"1"`, not `"1st"`. It does keep the `verify_ordinal` guard, so negatives
//! still raise TypeError.
//!
//! # Currency and cheque
//!
//! Both delegate to the **sibling** `Num2Word_EN`, exactly as `to_ordinal` does:
//!
//! ```python
//! def to_currency(self, *args, **kwargs): return self._english.to_currency(*args, **kwargs)
//! def to_cheque(self, *args, **kwargs):   return self._english.to_cheque(*args, **kwargs)
//! ```
//!
//! This is not a stylistic choice — it is the whole behaviour. Because the call
//! lands on a `Num2Word_EN` instance, *every* piece of the currency path is
//! EN's, not AERO's: the forms table, the precision table, `pluralize`,
//! `_money_verbose`, and the class name baked into the NotImplementedError. The
//! money amount therefore reads as **composite English**, never digit-by-digit:
//!
//! ```text
//! to_currency(1234.56, "EUR") -> "one thousand, two hundred and thirty-four euros, fifty-six cents"
//! ```
//!
//! not `"wun too tree fower euros, ..."`. The frozen corpus confirms this on all
//! 108 currency and 9 cheque rows.
//!
//! Three traps live here, all verified against the Python:
//!
//! 1. **`_money_verbose` must NOT be routed to the delegate.** AERO inherits
//!    `Num2Word_Base._money_verbose`, which is `self.to_cardinal(number)` — bound
//!    to an AERO instance that is `"wun too tree fower"`. So the trait default
//!    (`self.to_cardinal`) is the *faithful* mirror of AERO's inherited method
//!    and is deliberately left alone. It is dead code: the only caller,
//!    `to_currency`, hands the whole job to `self.english` and never reaches it.
//!    Overriding it to reach for `self.english.to_cardinal` would "fix" a method
//!    Python never calls, while making the class lie about what it inherited.
//! 2. **The NotImplementedError names `Num2Word_EN`, not `Num2Word_EN_AERO_FAA`**
//!    — `base.to_currency` interpolates `self.__class__.__name__`, and `self` is
//!    the delegate. Verified: `to_currency(1.0, currency="XXX")` raises
//!    `Currency code "XXX" not implemented for "Num2Word_EN"`. Falls out for free
//!    from forwarding, since the message is built inside [`LangEn`].
//! 3. **JPY is not a zero-decimal currency in this library.** `Num2Word_EN`
//!    leaves JPY/KRW out of `CURRENCY_PRECISION` (its comment says the historical
//!    sen/jeon subunits are still expected by the fixtures), so JPY's divisor is
//!    100 and `1.0` renders `"one yen, zero sen"`. The `divisor == 1` branch of
//!    `default_to_currency` is unreachable for this language. KWD/BHD (and
//!    OMR/JOD/TND/LYD/IQD) do carry 1000.
//!
//! The forms/precision/adjective/pluralize hooks forward to the delegate rather
//! than sitting at their trait defaults. AERO genuinely *has* those tables —
//! `super().__init__()` runs `Num2Word_EN.__init__`, which populates
//! `self.CURRENCY_FORMS` and `self.CURRENCY_PRECISION` on the AERO instance too —
//! they are merely never read, because `to_currency` delegates before it could
//! read them. Forwarding keeps that inherited state honest at zero cost, and
//! cannot drift from EN's table the way a local copy would.
//!
//! No table is built here. `LangEn::new()` builds EN's `CURRENCY_FORMS` once, and
//! `LangEnAeroFaa::new()` already owns exactly one `LangEn` (for `to_ordinal`),
//! constructed once. Nothing is allocated per call.
//!
//! # Error variants
//!
//! `Num2Word_Base.verify_ordinal` raises `TypeError(errmsg_negord)` for negative
//! input (`"Cannot treat negative num %s as ordinal."`). Integral BigInt input
//! can never trip the `errmsg_floatord` branch. Both `to_ordinal` and
//! `to_ordinal_num` are guarded; `to_cardinal` and `to_year` accept negatives
//! and render them with the minus-word. No AERO path raises OverflowError.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::lang_en::LangEn;
use crate::lang_en_aero::{aero_special_of, aero_special_of_decimal, aero_str_to_number};
use crate::strnum::ParsedNumber;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::Signed;

/// `_ICAO_DIGITS`, indexed by digit value rather than keyed by character.
///
/// The respellings are the point of the module: 1→"wun", 2→"too", 3→"tree",
/// 4→"fower", 5→"fife", 8→"ait", 9→"niner" are ICAO Annex 10 vol II
/// pronunciations, not typos. 0/6/7 keep their plain English spelling.
const ICAO_DIGITS: [&str; 10] = [
    "zero", "wun", "too", "tree", "fower", "fife", "six", "seven", "ait", "niner",
];

/// `_ICAO_DECIMAL` — the `_PROFILES["FAA"]` decimal mark.
const ICAO_DECIMAL: &str = "decimal";

/// `_ICAO_MINUS` — the `_PROFILES["FAA"]` sign word.
const ICAO_MINUS: &str = "minus";

pub struct LangEnAeroFaa {
    /// Python's `self._english = Num2Word_EN()`: the sibling plain-English
    /// converter that `to_ordinal` delegates to.
    english: LangEn,
    /// `self._digit_table`, `self._decimal_word`, `self._minus_word` — set from
    /// `_PROFILES[chosen]` in `__init__`. Held as fields (rather than used as
    /// constants directly) to keep the profile indirection visible, since the
    /// module exists to make historical profiles addable later.
    digit_table: [&'static str; 10],
    decimal_word: &'static str,
    minus_word: &'static str,
    /// Inherited from `Num2Word_EN.setup()`. Inert for all four in-scope modes:
    /// `title()` only consults it when `is_title()` is true, and EN leaves that
    /// false. Kept so the inherited state is represented rather than silently
    /// dropped.
    exclude_title: Vec<String>,
}

impl Default for LangEnAeroFaa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnAeroFaa {
    pub fn new() -> Self {
        // Num2Word_EN_AERO.__init__: chosen = profile or self.PROFILE -> "FAA";
        // "FAA" is in _PROFILES, so the ValueError branch is unreachable for
        // this class. digits, decimal_word, minus_word = _PROFILES["FAA"].
        LangEnAeroFaa {
            english: LangEn::new(),
            digit_table: ICAO_DIGITS,
            decimal_word: ICAO_DECIMAL,
            minus_word: ICAO_MINUS,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal` for integral BigInt input.
    ///
    /// Python checks `value == int(value)` (float guard — unreachable here) then
    /// `abs(value) == value` (sign guard). Only the second can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// Port of `Num2Word_EN_AERO._digits_of`.
    ///
    /// Returns `(is_negative, int_part, frac_part)`. Faithful to the Python
    /// including the parts that integral input never exercises: the `,`/`_`
    /// separator strip, the split on the *first* `.`, and the empty-integer-part
    /// backfill to `"0"`. For BigInt input `s` is always `-?[0-9]+`, so the
    /// separator strip and the `.` branch are always no-ops and `frac_part` is
    /// always empty — they are kept so the structure matches the original.
    fn digits_of(s: &str) -> (bool, String, String) {
        // Python: str(value) for non-str/non-Decimal input. BigInt::to_string
        // is the same decimal rendering, so no .strip() is needed.
        let is_negative = s.starts_with('-');
        let s = if is_negative { &s[1..] } else { s };

        // Drop any thousands separators users might pass in.
        let s = s.replace(',', "").replace('_', "");

        // Python: s.split(".", 1) — split on the FIRST dot only, so the
        // remainder (dots and all) stays in frac_part.
        let (int_part, frac_part) = match s.find('.') {
            Some(i) => (s[..i].to_string(), s[i + 1..].to_string()),
            None => (s.clone(), String::new()),
        };

        // Ensure at least one digit on the integer side ("." is invalid).
        let int_part = if int_part.is_empty() {
            "0".to_string()
        } else {
            int_part
        };
        (is_negative, int_part, frac_part)
    }

    /// The digit-table lookup, `self._digit_table[ch]`.
    ///
    /// Python indexes a dict, so a character that passes `ch.isdigit()` but is
    /// absent from the table raises `KeyError`. That is reachable in Python only
    /// via the `str` input branch of `_digits_of`, because `str.isdigit()` is
    /// true for non-ASCII digits ('٣', '²', …) that `_ICAO_DIGITS` has no key
    /// for. BigInt input renders as ASCII only, so `N2WError::Key` is
    /// unreachable from this crate — it is modelled anyway rather than
    /// `unwrap()`-ing, so the crash type stays correct if a string path is ever
    /// wired in.
    fn digit_word(&self, ch: char) -> Result<&'static str> {
        match ch.to_digit(10) {
            Some(d) => Ok(self.digit_table[d as usize]),
            None => Err(N2WError::Key(format!("'{}'", ch))),
        }
    }

    /// Port of `Num2Word_EN_AERO.to_cardinal` operating on the normalised
    /// string, so the (unreachable-for-integers) fractional branch is preserved
    /// exactly as written rather than dropped.
    fn cardinal_str(&self, s: &str) -> Result<String> {
        let (is_negative, int_part, frac_part) = Self::digits_of(s);
        let mut words: Vec<&str> = Vec::new();
        if is_negative {
            words.push(self.minus_word);
        }
        // Python filters with `if ch.isdigit()`, silently skipping anything
        // else. ASCII digits are the only survivors for BigInt input.
        for ch in int_part.chars() {
            if ch.is_ascii_digit() {
                words.push(self.digit_word(ch)?);
            }
        }
        if !frac_part.is_empty() {
            words.push(self.decimal_word);
            for ch in frac_part.chars() {
                if ch.is_ascii_digit() {
                    words.push(self.digit_word(ch)?);
                }
            }
        }
        Ok(words.join(" "))
    }
}

impl Lang for LangEnAeroFaa {

    /// `maxval()` is None (gladiaio/num2words2#159): the cardinal reads digit
    /// by digit with no ceiling, so the inherited `Num2Word_EN.MAXVAL` (10^306)
    /// is not reported — the cardinal happily converts past it.
    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        None
    }

    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        // The Decimal("Infinity")/("NaN") sentinels smuggled through by
        // `aero_str_to_number` render as `_digits_of` reads format(v, "f")
        // = "Infinity"/"-Infinity"/"NaN": no digit chars, only the sign
        // word — "" / "minus" / "" (all three are corpus rows).
        if let Some(sp) = aero_special_of(value) {
            return Ok(sp.cardinal_words().to_string());
        }
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("fife decimal zero"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` → `self._english.to_ordinal(value)`.
    ///
    /// The delegate's `verify_ordinal` polices the type: whole values
    /// ordinalise in plain English (`5.0` → "fifth", `Decimal("1E+2")` →
    /// "one hundredth", `-0.0` → "zeroth"); fractional or negative values
    /// raise TypeError. The Infinity/NaN sentinels reproduce the `int(value)`
    /// raise inside that comparison: OverflowError / ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(sp) = aero_special_of(value) {
            return Err(sp.int_error());
        }
        self.english.ordinal_float_entry(value)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` — AERO's own,
    /// i.e. Base's — then `str(int(value))`. Bare truncated digits, no
    /// suffix: `5.00` → "5", `1e+16` → "10000000000000000", `-0.0` → "0".
    /// Float-ness is checked before sign (so `-1.5` raises the *float*
    /// message); `%s` interpolates `str(value)` = `repr_str`.
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
    /// digit reading, ".0" tail included: `1971.0` → "wun niner seven wun
    /// decimal zero".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// Port of `to_fraction`: `return self._english.to_fraction(...)` —
    /// standard English forms ("one half", "three quarters", "twenty-two
    /// sevenths"), with both the cardinal numerator and ordinal denominator
    /// rendered by the delegate, never digit-by-digit. `n/0`'s
    /// ZeroDivisionError also originates inside the delegate.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        self.english.to_fraction(numerator, denominator)
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, with
    /// Infinity/NaN carried through as sentinels (see `lang_en_aero`)
    /// because AERO's string-reading cardinal *succeeds* on them.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        aero_str_to_number(s)
    }
    // negword/pointword are inherited from Num2Word_EN.setup(). Both are inert
    // here: the default `to_cardinal` that would consume them is overridden, and
    // AERO renders its own sign via `minus_word` ("minus", no trailing space).
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "point"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    // cards() / maxval() / merge() intentionally left at the trait defaults:
    // Python builds EN's card table on this instance via setup(), but the
    // to_cardinal override means nothing ever reads it.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal_str(&value.to_string())
    }

    /// `self._english.to_ordinal(value)` — plain English, NOT digit-by-digit.
    /// LangEn::to_ordinal runs its own verify_ordinal, so negatives raise
    /// TypeError from inside the delegate exactly as in Python.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.english.to_ordinal(value)
    }

    /// `self.verify_ordinal(value); return str(int(value))`.
    ///
    /// Overrides EN's "1st"/"2nd" form with the bare decimal string. The guard
    /// is AERO's own (`self`, i.e. base's verify_ordinal), not the delegate's.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// `return self.to_cardinal(value)` — EN's century/"oh-"/BC logic is bypassed
    /// entirely. 1776 -> "wun seven seven six"; -500 -> "minus fife zero zero".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal cardinal path.
    ///
    /// AERO **does not** inherit `Num2Word_Base.to_cardinal_float` in practice:
    /// it overrides `to_cardinal` outright, and the dispatcher calls *that*
    /// directly for float/Decimal input, so the base float path (float2tuple +
    /// `pointword`) is never reached. This mirrors the non-integer arms of
    /// `Num2Word_EN_AERO._digits_of`, which read `str(float)` /
    /// `format(Decimal, "f")` **character by character** — `precision` and the
    /// `float2tuple` binary heuristic play no part.
    ///
    /// That string-reading is load-bearing and is exactly why the base path is
    /// wrong here: the float `1e16` stringifies to `"1e+16"` and reads as
    /// `"wun wun six"` (the digits of the repr, 'e'/'+' skipped), **not** the
    /// sixteen-zero expansion `float2tuple` would produce. The `float` and
    /// `Decimal` arms are also not interchangeable — `float(1e16)` -> `"1e+16"`
    /// -> `"wun wun six"`, while `Decimal("1E+16")` -> `format(_, "f")` ->
    /// `"10000000000000000"` -> seventeen words. `FloatValue` preserves that
    /// split, so [`python_str`] can pick the right stringification.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) has no effect:
    /// AERO's method takes no such parameter and reads the string regardless.
    /// The existing [`Self::cardinal_str`] then does the digit-by-digit render,
    /// including the sign, the `,`/`_` strip, the split on the first `.`, the
    /// `decimal` mark and the silent skip of any non-digit (so `"1e+16"` drops
    /// its 'e'/'+' and `"nan"`/`"inf"` collapse to `""`).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_str(&python_str(value))
    }

    // ---- currency ----------------------------------------------------
    //
    // See the module docs: `to_currency`/`to_cheque` delegate to the sibling
    // `Num2Word_EN`, so the whole surface is EN's. The table hooks below forward
    // to the same delegate, mirroring the `CURRENCY_FORMS`/`CURRENCY_PRECISION`
    // that `Num2Word_EN.__init__` really does install on the AERO instance.
    //
    // `money_verbose` / `cents_verbose` / `cents_terse` are deliberately NOT
    // overridden: AERO inherits `Num2Word_Base`'s versions, which route through
    // `self.to_cardinal` — i.e. digit-by-digit — and the trait defaults do
    // exactly that. They are dead either way (`to_currency` delegates past
    // them); the defaults keep them honest rather than pointing them at the
    // delegate, which Python never does.

    /// `self.__class__.__name__`. Inert on the currency path: `to_currency` and
    /// `to_cheque` both hand off to [`LangEn`], so a missing-code error is built
    /// from *its* `lang_name` and names `"Num2Word_EN"` — verified against the
    /// Python. Recorded accurately here regardless, since the trait's fallback
    /// (`"Num2Word_Base"`) would be wrong for every other reader of the hook.
    fn lang_name(&self) -> &str {
        "Num2Word_EN_AERO_FAA"
    }

    /// `self.CURRENCY_FORMS[code]` — EN's table, including its `__init__`
    /// overrides of the `lang_EUR` defaults (`EUR` is `("euro", "euros")`, not
    /// EUR's `("euro", "euro")`; `GBP` is `("pound", "pounds")`, not
    /// `("pound sterling", "pounds sterling")`).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.english.currency_forms(code)
    }

    /// `self.CURRENCY_ADJECTIVES[code]` — inherited untouched from `lang_EUR`
    /// (`USD` -> "US", giving "two US dollars" when `adjective=True`).
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.english.currency_adjective(code)
    }

    /// `self.CURRENCY_PRECISION.get(code, 100)`. EN sets 1000 for the seven
    /// 3-decimal currencies (BHD/KWD/OMR/JOD/TND/LYD/IQD) and lists nothing
    /// else, so every other code — JPY and KRW included — defaults to 100.
    fn currency_precision(&self, code: &str) -> i64 {
        self.english.currency_precision(code)
    }

    /// `Num2Word_EUR.pluralize`: `forms[0]` when `n == 1`, else `forms[1]`.
    /// Reached through the delegate, which inherits the same function.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        self.english.pluralize(n, forms)
    }

    /// `return self._english.to_currency(*args, **kwargs)`.
    ///
    /// The `CurrencyValue::Int` / `CurrencyValue::Decimal` split is passed
    /// through untouched — it is what makes `1` render `"one euro"` while `1.0`
    /// renders `"one euro, zero cents"`. Both forms are in the corpus.
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

    /// `return self._english.to_cheque(*args, **kwargs)`.
    ///
    /// The delegation is load-bearing: `to_cheque` renders its amount with
    /// `_money_verbose`, so routing through `self` would emit
    /// `"WUN TOO TREE FOWER AND 56/100 EUROS"` instead of the corpus's
    /// `"ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 EUROS"`.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        self.english.to_cheque(val, currency)
    }
}

/// `str(value)` as `Num2Word_EN_AERO._digits_of` sees a numeric input.
///
/// `_digits_of` normalises to a string before reading digits: `format(value,
/// "f")` for a `Decimal`, `str(value)` for anything else (here always a
/// `float`). The `FloatValue` split *is* Python's `isinstance(value, Decimal)`,
/// so the two arms map straight onto the two stringifications — and they are
/// deliberately not the same function (see [`python_float_repr`] vs
/// [`python_decimal_format_f`]).
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => python_float_repr(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_format_f(value),
    }
}

/// `str(value)` for a Python `float` — the shortest round-trip repr.
///
/// In the normal range the value formats to exactly `precision` fractional
/// digits, where `precision` is the repr-derived count handed across the
/// boundary (`abs(Decimal(repr(v)).as_tuple().exponent)`), so `{:.precision}`
/// reproduces `repr(float)` digit for digit. Outside `[1e-4, 1e16)` CPython
/// switches to scientific (`"1e+16"`, `"1e-05"`, `"1.5e-07"`); the thresholds
/// `a >= 1e16 || a < 1e-4` are exactly CPython's `decpt > 16 || decpt <= -4`
/// (`decpt > 16` <=> `a >= 1e16`, `decpt <= -4` <=> `a < 1e-4`). Rust's `{:e}`
/// is shortest round-trip like Python's repr, so the mantissa digits agree;
/// only the exponent presentation (mandatory sign, two-digit minimum) is
/// reshaped to match. `_digits_of` reads whichever form character by character.
///
/// `nan`/`inf` are unreachable through the boundary (the Python-side precision
/// step does `abs()` on a non-numeric Decimal exponent and raises `TypeError`
/// first), but are spelled Python's lowercase way regardless so that
/// `cardinal_str` collapses them to `""` exactly as `_digits_of` would.
fn python_float_repr(v: f64, precision: u32) -> String {
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

/// `format(value, "f")` — CPython's fixed-point formatting of a `Decimal`.
///
/// Unlike `str(Decimal)` (which goes scientific when `_exp > 0` or
/// `leftdigits <= -6`), the `"f"` presentation type is **always** positional:
/// the coefficient digits sit at `dotplace = leftdigits`, zero-padded on
/// whichever side is short, never an exponent. `_digits_of` uses
/// `format(value, "f")`, so this is the string it reads — e.g.
/// `Decimal("1E+16")` -> `"10000000000000000"` (seventeen chars), where the
/// numerically equal *float* `1e16` would repr as `"1e+16"`.
///
/// Reconstructed from `as_bigint_and_exponent` rather than delegated, because
/// `BigDecimal`'s own `Display` is not `format(Decimal, "f")`: it drops the
/// trailing zeros Python keeps in scale (`"0.00"` -> `"0"`, silently eating two
/// fractional words) and lowercases/scientific-ises large magnitudes. The pair
/// it returns is exactly Python's `(_int, _exp)` — `scale` is `-_exp`, the
/// coefficient keeps its significant digits verbatim, trailing zeros included
/// (`"1.10"` is 110x10^-2 on both sides).
///
/// # Known gap: negative zero
///
/// `format(Decimal("-0.0"), "f")` is `"-0.0"`, so Python emits the minus-word.
/// `BigInt` has no signed zero, so `BigDecimal::from_str("-0.0")` has already
/// dropped the sign before this runs and the minus-word is lost. The sign dies
/// at the `FloatValue::Decimal` boundary, so it cannot be recovered here. Not
/// in the corpus; flagged in the port report.
fn python_decimal_format_f(d: &BigDecimal) -> String {
    let (coefficient, scale) = d.as_bigint_and_exponent();
    let exp = -scale; // Python's Decimal `_exp`.
    let sign = if coefficient.is_negative() { "-" } else { "" };
    // Python's `_int`: the unsigned coefficient. BigInt renders ASCII digits,
    // so byte slicing below is char slicing.
    let digits = coefficient.abs().to_string();
    let ndig = digits.len() as i64;
    let leftdigits = exp + ndig;
    // The `"f"` type never uses an exponent, so dotplace is always leftdigits.
    let dotplace = leftdigits;

    let (intpart, fracpart) = if dotplace <= 0 {
        // 0.<zeros><digits>
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), digits),
        )
    } else if dotplace >= ndig {
        // <digits><zeros>, no fractional part.
        (
            format!("{}{}", digits, "0".repeat((dotplace - ndig) as usize)),
            String::new(),
        )
    } else {
        let k = dotplace as usize;
        (digits[..k].to_string(), format!(".{}", &digits[k..]))
    };
    format!("{}{}{}", sign, intpart, fracpart)
}
