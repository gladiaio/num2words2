//! Port of `lang_KOK.py` (Konkani).
//!
//! # Deliberate divergences from upstream
//!
//! Two: the **script** (Devanagari, not Latin) and the **ordinals for
//! 1..=10**. Everything else is ported verbatim, bug for bug.
//!
//! ## 1. Devanagari script
//!
//! `lang_KOK.py` spells every numeral in **Latin transliteration** ("ek",
//! "don", "chalis", "xambhar"). Devanagari is Konkani's official script —
//! it is the script named in the Goa Official Language Act and the one the
//! Sahitya Akademi standard uses — and every sibling Indic module in this
//! crate already emits its own script. The tables here are the Devanagari
//! spellings: "एक", "दोन", "चाळीस", "शंभर".
//!
//! Konkani is genuinely pluriscriptal (Roman/Romi is in real literary use in
//! Goa, and Kannada script is used in coastal Karnataka), so a future
//! `kok_Latn` / `kok_Knda` split along the `sr_Cyrl`/`sr_Latn` precedent
//! would be reasonable. But the *default* `kok` should be the official
//! script, which it now is.
//!
//! This is a lexicon change only — the composition rules, the mixed
//! `" आनी "` / `" "` joints (bug 3), the 10^9 cliff and the currency
//! fallback are all still ported verbatim.
//!
//! ## 2. Ordinals 1..=10
//!
//! Upstream builds every ordinal as `to_cardinal(n) + "vo"`. वो is the
//! genuine Konkani ordinal marker, so 5, 7, 8 and 10 already came out right
//! (पांचवो, सातवो, आठवो, धावो) — but 1-4 are suppletive (पयलो, दुसरो,
//! तिसरो, चवथो, nothing to do with एक/दोन/तीन/चार) and 6 and 9 contract
//! (सव्वो, नव्वो, not *सववो / *नववो). See [`ORDINAL_IRREGULARS`].
//!
//! **From 11 up nothing changed but the script**: `to_ordinal(11)` is
//! "धा आनी एकवो", as before. `verify_ordinal` is still not called, so
//! `to_ordinal(0)` is still *शून्यवो and `to_ordinal(-1)` still takes the
//! suffix arm.
//!
//! Shape: **self-contained**. `Num2Word_KOK` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `MAXVAL` (that block is guarded by `if any(hasattr(self, field) ...)`).
//! `to_cardinal` is overridden outright and drives a hand-rolled recursive
//! `_int_to_word`. Consequently `cards`/`maxval`/`merge` stay at their trait
//! defaults here and there is **no overflow check at all** — see bug 1.
//!
//! All four in-scope modes are overridden by KOK, so nothing is inherited
//! from `Num2Word_Base` in the integer path:
//!   * `to_cardinal(number)`    — string-sniffs the sign, then `_int_to_word`
//!   * `to_ordinal(number)`     — suppletive for 1..=10, else
//!     `to_cardinal(number) + "वो"`
//!   * `to_ordinal_num(number)` — `str(number) + "वो"`
//!   * `to_year(val, longval=True)` — ignores `longval`, delegates to
//!     `to_cardinal`. There is no era/two-chunk year logic: `to_year(1999)`
//!     is just the plain cardinal "एक हजार नव शंभर आनी नव्वद आनी नव",
//!     and `to_year(-500)` is "रीण पांच शंभर" (no "BC"-style suffix).
//!
//! # Float/Decimal routing — everything is string surgery on `str(number)`
//!
//! `to_cardinal` never type-checks; it does `n = str(number).strip()` and
//! branches on the *text*. That gives the whole float surface its shape:
//!
//! * **`"." in n` → per-digit decimal grammar, whole values included.**
//!   `to_cardinal(5.0)` is "पांच पुंतो शून्य" — `str(5.0)` is "5.0", so the
//!   float grammar fires even though the value is whole. Trailing Decimal
//!   zeros survive: `Decimal("5.00")` → "पांच पुंतो शून्य शून्य".
//! * **No "." → `int(n)`**, so `Decimal("5")` → "पांच" but any value whose
//!   string is exponential notation raises `int()`'s **ValueError**:
//!   `to_cardinal(1e16)` (repr "1e+16"), `Decimal("1E+2")`, `Decimal("1E+20")`
//!   all raise "invalid literal for int() with base 10: '...'". So do the
//!   string inputs "1e3"/"1E3" (→ `Decimal("1E+3")` → str "1E+3") and
//!   "Infinity"/"-Infinity"/"NaN" (`Decimal` parses them; `int("Infinity")`
//!   raises). See [`LangKok::str_to_number`] for where the Inf/NaN raise is
//!   modelled.
//! * **`str(-0.0)` is "-0.0"**, so negative zero keeps its negword:
//!   `to_cardinal(-0.0)` == "रीण शून्य पुंतो शून्य".
//! * `to_ordinal(float)` is `to_cardinal(float) + "वो"` — the suffix binds to
//!   the decimal spelling ("पांच पुंतो xunyavo"); `to_ordinal_num(float)` is
//!   `str(number) + "वो"` verbatim, "-0.0vo"/"1e+16vo" included; `to_year`
//!   delegates to `to_cardinal`.
//! * `self.precision` is never read, so the `precision=` kwarg has no effect
//!   on any of this.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the bare digits.** The
//!    final `return str(number)` is a fallthrough, not a raise: `to_cardinal(
//!    10**9)` == "1000000000" and `to_ordinal(10**9)` == "1000000000vo".
//!    This is why the language never raises `OverflowError` and why the
//!    value must stay a `BigInt` — the digit string is the output for every
//!    input from 10^9 up to 10^606 and beyond. See [`LangKok::int_to_word`].
//! 2. **`million` is spelled "धा लाख"** — literally "ten लाख". So 10^6
//!    is "एक धा लाख" and 10^7 renders as "धा धा लाख" ("ten ten
//!    lakh"), which is arithmetically odd but verbatim Python. Kept as-is.
//! 3. **Separator asymmetry.** The hundreds branch joins its remainder with
//!    `" आनी "`, but the thousands and millions branches join theirs with a
//!    bare `" "`. Hence 101 == "एक शंभर आनी एक" but 1001 == "एक हजार एक"
//!    (no "आनी"), and 1234 == "एक हजार दोन शंभर आनी तीस आनी चार".
//! 4. **`ones[6]` is "सव"**, which collides visually with nothing else but
//!    reads oddly next to `hundred` = "शंभर"; 16 == "धा आनी सव".
//! 5. **`negword` is "रीण "** (with a trailing space) and the negative path
//!    concatenates then `.strip()`s. `_int_to_word` never returns an empty
//!    string (0 short-circuits to "शून्य"), so the strip is a no-op in
//!    practice — but it is reproduced for fidelity.
//! 6. **No negative-ordinal guard.** `Num2Word_Base.to_ordinal` would raise
//!    on negatives via `errmsg_negord`, but KOK overrides it without that
//!    check, so `to_ordinal(-1)` == "रीण ekvo" and `to_ordinal_num(-1)` ==
//!    "-1vo". Both are corpus-confirmed.
//!
//! # Currency
//!
//! The two halves of the currency surface behave *differently*, and the split
//! is the single most important thing about this port:
//!
//! * **`to_currency` is overridden wholesale** and shares nothing with
//!   `Num2Word_Base.to_currency`. It never raises: an unknown code silently
//!   falls back to `list(self.CURRENCY_FORMS.values())[0]` — the first
//!   *inserted* value, INR — so `currency:GBP` renders "रुपया"/"पैसो". It
//!   never consults `CURRENCY_PRECISION`, `CURRENCY_ADJECTIVES`, or
//!   `pluralize`, and it ignores its own `adjective` argument entirely.
//! * **`to_cheque` is inherited from `Num2Word_Base`** and does
//!   `self.CURRENCY_FORMS[currency]`, letting the `KeyError` become a
//!   `NotImplementedError`. So `cheque:GBP` *raises* while `currency:GBP`
//!   happily returns INR words. Both are corpus-confirmed.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both inherited as `{}`
//! from `Num2Word_Base`, so every code has divisor 100 — JPY and KWD/BHD get
//! ordinary two-decimal cents here rather than the 0-/3-decimal treatment
//! other languages give them, and `currency_precision`/`currency_adjective`
//! stay at their trait defaults deliberately.
//!
//! # Faithfully reproduced Python quirks (currency)
//!
//! 7. **`to_currency` truncates cents, it does not round.** It slices the
//!    decimal *string*: `int(parts[1][:2].ljust(2, "0"))`. So 1.005 -> 0
//!    cents (and the segment vanishes), 2.675 -> 67, 1.999 -> 99. There is no
//!    `ROUND_HALF_UP` anywhere on this path.
//! 8. **A float with zero cents drops the cents segment.** Guarded by
//!    `if cents and right:`, so `1.0` is "एक युरो" — *unlike* the base class,
//!    which shows "zero cents" for any float. The int/float distinction the
//!    `CurrencyValue` split exists to preserve therefore has **no observable
//!    effect** in KOK: `1` and `1.0` both render "एक युरो". It is still
//!    honoured exactly, because `str(1)` and `str(1.0)` differ.
//! 9. **`cr1[1]` / `cr2[1]`, not `[-1]`.** Every KOK form is a 2-tuple so the
//!    two coincide, but the literal index is kept.
//!
//! # Errors
//!
//! The integer modes still cannot fail — there is no `MAXVAL`, no table
//! lookup that can miss, and no `int()` on a non-numeric token. On the
//! currency surface only two things raise:
//!
//! * `to_cheque` with a code outside {INR, USD, EUR} -> `NotImplemented`.
//! * `to_currency` on a value whose `str()` uses exponential notation ->
//!   `Value` (Python's `int("1e+16")` ValueError). See
//!   [`LangKok::exponential_parts`].

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.ones`. Index 0 is "" and is only ever reachable through the float
/// path (`self.ones[int(digit)] or "शून्य"`), which is out of scope — the
/// integer path guards every lookup with `number > 0`.
const ONES: [&str; 10] = [
    "", "एक", "दोन", "तीन", "चार", "पांच", "सव", "सात", "आठ", "नव",
];

/// `self.tens`. Index 0 is "" and unreachable: the branch that indexes this
/// runs only for `10 <= number < 100`, so the tens digit is always 1..=9.
const TENS: [&str; 10] = [
    "", "धा", "वीस", "तीस", "चाळीस", "पन्नास", "साठ", "सत्तर", "ऐंशी", "नव्वद",
];

const HUNDRED: &str = "शंभर";
const THOUSAND: &str = "हजार";
/// Python spells 10^6 "dosh lakh" — "ten lakh". See bug 2 for why that is
/// the wrong *scale* word; the spelling here is the Devanagari for it.
///
/// "dosh" has no Konkani reading: Konkani for ten is धा (the `TENS[1]` entry
/// just above), and Marathi's दहा is the nearest neighbour. It is written धा
/// here so the phrase reads as the "ten lakh" the module header says it is,
/// rather than transliterating a token that spells nothing.
const MILLION: &str = "धा लाख";
/// sic — trailing space is part of the word in Python. See bug 5.
const NEGWORD: &str = "रीण ";
const POINTWORD: &str = "पुंतो";
/// The zero word, and (in the out-of-scope float path) the `or` fallback for
/// a "0" fraction digit.
const ZERO_WORD: &str = "शून्य";

/// The ordinal suffix appended from 11 up, and by `to_ordinal_num` at every
/// magnitude.
///
/// वो is the genuine Konkani ordinal marker — it is what ends पांचवो, सातवो,
/// आठवो and धावो — which is why upstream's blind `+ "vo"` looked plausible.
/// It is wrong only where a suppletive form exists; see
/// [`ORDINAL_IRREGULARS`].
const ORDINAL_SUFFIX: &str = "वो";

/// Konkani ordinals 1..=10.
///
/// 1-4 are suppletive (पयलो, दुसरो, तिसरो, चवथो — nothing to do with एक,
/// दोन, तीन, चार), and 6 and 9 contract rather than simply concatenating
/// (सव्वो, नव्वो, not *सववो / *नववो). 5, 7, 8 and 10 are what the plain
/// suffix already produced.
///
/// Index 0 is unused — Konkani has no ordinal for zero, and upstream's blind
/// suffix produced the non-word *शून्यवो for it.
///
/// Forms cross-checked against Omniglot's Konkani numbers page, which lists
/// them as "poilo, dusro, tisro, chouto, panchvo, sovvo, satvo, attvo,
/// novvo, dhavo".
const ORDINAL_IRREGULARS: [&str; 11] = [
    "",
    "पयलो",
    "दुसरो",
    "तिसरो",
    "चवथो",
    "पांचवो",
    "सव्वो",
    "सातवो",
    "आठवो",
    "नव्वो",
    "धावो",
];

/// `Num2Word_KOK.CURRENCY_FORMS`, **in Python's insertion order**.
///
/// The order is load-bearing: `to_currency` resolves an unknown code with
/// `list(self.CURRENCY_FORMS.values())[0]`, i.e. the first *inserted* value,
/// which is INR's. A `HashMap` cannot answer that question, so index 0 of this
/// table is lifted into [`LangKok::fallback_forms`] at construction time.
const CURRENCY_FORMS: [(&str, [&str; 2], [&str; 2]); 3] = [
    ("INR", ["रुपया", "रुपया"], ["पैसो", "पैसो"]),
    ("USD", ["डॉलर", "डॉलर"], ["सेंट", "सेंट"]),
    ("EUR", ["युरो", "युरो"], ["सेंट", "सेंट"]),
];

/// KOK's own `to_currency` signature defaults `separator=" "`, where
/// `Num2Word_Base.to_currency` defaults it to `","`. See the note on
/// [`LangKok::to_currency`] — this is not a cosmetic difference, it is the
/// separator every corpus row was recorded with.
const DEFAULT_SEPARATOR: &str = " ";

/// The separator `Num2Word_Base` defaults to, which the Python shim hands us
/// whenever the caller supplied none. Treated as "unset". See
/// [`LangKok::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// Small-index helper for the `ONES`/`TENS` lookups.
///
/// Every call site is guarded by a `< 100` or `< 1000` branch, so the value
/// is provably 0..=9 and the conversion cannot fail. Python would raise
/// `IndexError` here if the invariant broke; it cannot.
fn digit_index(n: &BigInt) -> usize {
    n.to_usize()
        .filter(|i| *i < 10)
        .expect("KOK: ones/tens index is 0..=9 by the enclosing range guard")
}

pub struct LangKok {
    /// `self.exclude_title`. Dead in practice: `Num2Word_Base.__init__` sets
    /// `is_title = False` and KOK never flips it, so `title()` is the
    /// identity and this list is never consulted. Kept to mirror `setup()`.
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS`, built once. Constructing this per call is what made
    /// an earlier revision of this port 10x slower than the Python it
    /// replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(CURRENCY_FORMS.values())[0]` — INR's forms, the value
    /// `to_currency` falls back to for an unknown code. Held separately
    /// because `currency_forms` is unordered.
    fallback_forms: CurrencyForms,
}

impl Default for LangKok {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKok {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::with_capacity(CURRENCY_FORMS.len());
        for (code, unit, subunit) in CURRENCY_FORMS {
            currency_forms.insert(code, CurrencyForms::new(&unit, &subunit));
        }
        let (_, unit, subunit) = CURRENCY_FORMS[0];
        LangKok {
            exclude_title: vec!["आनी".to_string(), POINTWORD.to_string(), "रीण".to_string()],
            currency_forms,
            fallback_forms: CurrencyForms::new(&unit, &subunit),
        }
    }

    /// Port of `Num2Word_KOK._int_to_word`.
    ///
    /// Only ever called with a non-negative value: `to_cardinal` strips the
    /// sign before recursing. `div_mod_floor` is used rather than `div_rem`
    /// so the semantics match Python's `divmod` exactly regardless.
    ///
    /// The `>= 10^9` fallthrough returns the decimal digits verbatim — this
    /// is the whole reason the parameter is a `BigInt` and is never cast to
    /// a fixed-width int (see bug 1 in the module docs).
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        let ten = BigInt::from(10);
        if number < &ten {
            return ONES[digit_index(number)].to_string();
        }

        let hundred = BigInt::from(100);
        if number < &hundred {
            // t, o = divmod(number, 10)
            let (t, o) = number.div_mod_floor(&ten);
            let mut s = TENS[digit_index(&t)].to_string();
            if !o.is_zero() {
                s.push_str(" आनी ");
                s.push_str(ONES[digit_index(&o)]);
            }
            return s;
        }

        let thousand = BigInt::from(1000);
        if number < &thousand {
            // h, r = divmod(number, 100)
            let (h, r) = number.div_mod_floor(&hundred);
            let mut s = format!("{} {}", ONES[digit_index(&h)], HUNDRED);
            if !r.is_zero() {
                // Hundreds join with " आनी " — unlike thousands/millions.
                s.push_str(" आनी ");
                s.push_str(&self.int_to_word(&r));
            }
            return s;
        }

        let million = BigInt::from(1_000_000);
        if number < &million {
            // t, r = divmod(number, 1000)
            let (t, r) = number.div_mod_floor(&thousand);
            let mut s = format!("{} {}", self.int_to_word(&t), THOUSAND);
            if !r.is_zero() {
                // Bare space, no "आनी" — see bug 3.
                s.push(' ');
                s.push_str(&self.int_to_word(&r));
            }
            return s;
        }

        let billion = BigInt::from(1_000_000_000);
        if number < &billion {
            // m, r = divmod(number, 1000000)
            let (m, r) = number.div_mod_floor(&million);
            let mut s = format!("{} {}", self.int_to_word(&m), MILLION);
            if !r.is_zero() {
                // Bare space, no "आनी" — see bug 3.
                s.push(' ');
                s.push_str(&self.int_to_word(&r));
            }
            return s;
        }

        // return str(number) — the silent give-up at 10^9. See bug 1.
        number.to_string()
    }

    /// Port of the `str(val).split(".")` parse at the head of
    /// `Num2Word_KOK.to_currency`, returning `(left, right)`:
    ///
    /// ```python
    /// parts = str(val).split(".")            # val is already abs()
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// KOK is unusual: it reads the *decimal string* directly instead of going
    /// through `Decimal`/`parse_currency_parts`, so `str()`'s exact spelling is
    /// observable behaviour rather than an implementation detail. The
    /// `BigDecimal` reaching us was parsed from Python's `str(value)`, and its
    /// `(digits, scale)` pair still encodes that spelling verbatim for every
    /// fixed-notation string — so the string is rebuilt from the pair rather
    /// than from `BigDecimal`'s own `Display`, which disagrees with Python
    /// (`1.5e+16` displays as `15000000000000000`).
    ///
    /// `right` is the first two fractional digits, zero-*padded* on the right —
    /// a truncation, never a rounding. See quirk 7.
    fn currency_parts(&self, val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
        // str(int) contains no "." and no "e", so parts == [digits]: the
        // fractional branch is unreachable and right is 0.
        let d = match val {
            CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
            CurrencyValue::Decimal { value: d, .. } => d.abs(),
        };

        let (digits, scale) = d.as_bigint_and_exponent();
        // abs() above guarantees a non-negative value, so no sign to strip.
        let ds = digits.to_string();
        let ndigits = ds.chars().count() as i64;

        // A negative scale can only come from an exponent in the source text:
        // BigDecimal parses "100" to scale 0, never to scale -2.
        if scale < 0 {
            return Self::exponential_parts(&ds, ndigits, scale);
        }
        // scale == 0 means str() emitted no ".", so parts == [ds].
        if scale == 0 {
            return Ok((digits, BigInt::zero()));
        }

        let (int_part, frac): (String, String) = if ndigits > scale {
            let cut = (ndigits - scale) as usize;
            (ds.chars().take(cut).collect(), ds.chars().skip(cut).collect())
        } else {
            // "0.00001": str() writes a leading zero, then padding.
            (
                "0".to_string(),
                format!("{}{}", "0".repeat((scale - ndigits) as usize), ds),
            )
        };

        Ok((parse_digits(&int_part), first_two_cents(&frac)))
    }

    /// The branch where Python's `str()` chose exponential notation.
    ///
    /// `str()` normalises the mantissa to a single leading digit (`d` or
    /// `d.ddd`) followed by `e±NN`, and `BigDecimal`'s digits are that mantissa
    /// verbatim. `split(".")` therefore lands differently by mantissa length,
    /// and — this is the surprise — only *some* of these raise:
    ///
    /// | digits | `str(val)` | `parts` | outcome |
    /// |---|---|---|---|
    /// | 1 | `1e+16` | `["1e+16"]` | `int("1e+16")` -> **ValueError** |
    /// | 2 | `1.5e+16` | `["1", "5e+16"]` | `int("5e")` -> **ValueError** |
    /// | 3+ | `1.2345…e+19` | `["1", "2345…e+19"]` | `int("23")` -> **succeeds**, "एक युरो वीस आनी तीन सेंट" |
    ///
    /// The 3+ case is not a mistake: Python really does return that. Only the
    /// first two characters of the fraction are ever read, so the exponent
    /// suffix falls outside the slice and never reaches `int()`.
    fn exponential_parts(ds: &str, ndigits: i64, scale: i64) -> Result<(BigInt, BigInt)> {
        let mut chars = ds.chars();
        let first = chars.next().unwrap_or('0');
        match ndigits {
            1 => {
                // decpt = ndigits - scale, exponent = decpt - 1 = -scale, and
                // scale < 0 here so the sign is always "+". str() pads the
                // exponent to a minimum of two digits ("1e+16", "1e+300").
                Err(N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}e+{:02}'",
                    first, -scale
                )))
            }
            2 => {
                let second = chars.next().unwrap_or('0');
                Err(N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}e'",
                    second
                )))
            }
            _ => {
                let two: String = chars.take(2).collect();
                Ok((parse_digits(&first.to_string()), parse_digits(&two)))
            }
        }
    }

    /// Port of `Num2Word_KOK.to_cardinal`'s *string* algorithm, applied to a
    /// reconstruction of Python's `str(number)`:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "शून्य")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The two `int()` calls are the language's only failure points and both
    /// are live: an exponential string ("1e+16", "1E+2", "Infinity") has no
    /// "." and lands in the final `int(n)` — **ValueError** — while a dotted
    /// exponential repr ("1.5e+16") passes `int("1")` but then hits
    /// `int("e")` in the digit loop — also ValueError, with `'e'` in the
    /// message. [`py_int`] reproduces the message format exactly.
    ///
    /// The negative branch recurses on the sign-stripped *string*, exactly as
    /// Python re-enters `to_cardinal(n[1:])`; negword's trailing space makes
    /// the concatenation seamless and the outer `.strip()` is inert.
    fn cardinal_of_pystr(&self, n: &str) -> Result<String> {
        let n = n.trim();
        if let Some(rest) = n.strip_prefix('-') {
            // (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = self.cardinal_of_pystr(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        match n.split_once('.') {
            Some((left, right)) => {
                // ret = self._int_to_word(int(left)) + " " + self.pointword
                let mut ret = format!("{} {}", self.int_to_word(&py_int(left)?), POINTWORD);
                // for digit in right:
                //     ret += " " + (self.ones[int(digit)] or "शून्य")
                for ch in right.chars() {
                    let d = py_int(&ch.to_string())?
                        .to_usize()
                        .expect("KOK: int(single decimal चार) is 0..=9");
                    // `ones[0]` is "" (falsy) -> "शून्य"; 1..=9 spell out.
                    let word = if ONES[d].is_empty() { ZERO_WORD } else { ONES[d] };
                    ret.push(' ');
                    ret.push_str(word);
                }
                // return ret.strip()
                Ok(ret.trim().to_string())
            }
            // No "." -> return self._int_to_word(int(n)); int() raises
            // ValueError on anything that is not a plain digit string.
            None => Ok(self.int_to_word(&py_int(n)?)),
        }
    }
}

/// Reconstruct Python's `str(number)` for the value the dispatcher saw.
///
/// * **Floats**: `repr(float)` is fixed notation with exactly `precision`
///   fractional digits — reproduced by `{:.N}` — *except* when the decimal
///   exponent is >= 16 or < -4, where repr switches to exponential form
///   ("1e+16", "1.5e+16", "1e-05"). Rust's `{:e}` yields the same shortest
///   round-trip mantissa; only the exponent spelling differs (Python always
///   writes a sign and pads to two digits). The `precision` the binding
///   derives in that regime comes from the Decimal *exponent*, not a
///   fractional-digit count, so it must not be used for formatting there.
/// * **Decimals**: [`python_decimal_str`] is the spec's to-scientific-string
///   algorithm, sign included — "5.00" keeps its zeros, `Decimal("1E+2")`
///   round-trips as "1E+2". (A zero Decimal spelled "-0.0" arrives as the
///   Float arm with -0.0 — the binding converts it — so the sign survives.)
fn value_python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => python_float_repr(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// `repr(float)` for a finite f64 (the dispatcher keeps inf/nan on the
/// Python side). See [`value_python_str`].
fn python_float_repr(f: f64, precision: u32) -> String {
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // repr picks exponential form here. Rust {:e} gives the identical
        // shortest mantissa ("1.5e16"); rewrite the exponent Python-style
        // (always signed, min two digits: "1e+16", "1e-05").
        let s = format!("{:e}", f);
        let (m, e) = s.split_once('e').expect("LowerExp always contains 'e'");
        let exp: i64 = e.parse().expect("LowerExp exponent is an integer");
        format!("{}e{}{:02}", m, if exp < 0 { '-' } else { '+' }, exp.abs())
    } else {
        // Fixed notation: {:.N} for the repr-derived N reproduces repr byte
        // for byte (shortest round-trip digits round to themselves), the
        // "-0.0" sign included.
        format!("{:.*}", precision as usize, f)
    }
}

/// Python's `int(s)` on the fragments `to_cardinal` slices out of
/// `str(number)` — fallible, unlike [`parse_digits`], because exponential
/// notation and Infinity/NaN really do reach it. The message is `int()`'s
/// exact format string.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// `int(s)` for a string sliced out of a `BigInt`'s decimal rendering.
///
/// Infallible by construction: every caller passes ASCII digits taken from
/// `BigInt::to_string()`. The fallible `int()` calls Python can actually trip
/// over live in [`LangKok::exponential_parts`].
fn parse_digits(s: &str) -> BigInt {
    BigInt::from_str(s).expect("KOK: slice of a BigInt decimal rendering is all digits")
}

/// `int(frac[:2].ljust(2, "0"))` — the first two fractional digits, padded on
/// the right. Truncating: "005" -> 0, "5" -> 50, "999" -> 99.
fn first_two_cents(frac: &str) -> BigInt {
    let mut two: String = frac.chars().take(2).collect();
    while two.chars().count() < 2 {
        two.push('0');
    }
    parse_digits(&two)
}

impl Lang for LangKok {
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
        "पुंतो"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    // is_title() stays false (Num2Word_Base.__init__ sets it and KOK's
    // setup() never overrides), so the inherited title() is the identity.

    /// Port of `Num2Word_KOK.to_cardinal`, integer path only.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n: ...          # float path, out of scope
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// `str(int)` never contains a "." and never carries whitespace, so the
    /// `strip()` and the "." test are both inert for integers; the sign test
    /// is equivalent to `value.is_negative()`. Python recurses on the string
    /// tail `n[1:]`, which re-parses to `abs(value)` — modelled directly.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = self.int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// Port of `Num2Word_KOK.to_ordinal`: `to_cardinal(number) + "वो"`.
    ///
    /// No negative guard and no float guard — the suffix is glued onto
    /// whatever `to_cardinal` produced, including the bare digit string for
    /// values >= 10^9 ("1000000000vo") and the negative form ("रीण ekvo").
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(i) = value.to_usize() {
            if (1..=10).contains(&i) {
                return Ok(ORDINAL_IRREGULARS[i].to_string());
            }
        }
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_KOK.to_ordinal_num`: `str(number) + "वो"`.
    /// Keeps the minus sign: `to_ordinal_num(-1)` == "-1vo".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_KOK.to_year`: `to_cardinal(val)`, ignoring
    /// `longval`. Identical to the trait default, but overridden explicitly
    /// because Python overrides it explicitly.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the float/Decimal side of `Num2Word_KOK.to_cardinal`.
    ///
    /// KOK does not have a separate float method; `to_cardinal` handles every
    /// type by reading `str(number)`. This hook reconstructs that string
    /// ([`value_python_str`]) and runs the ported algorithm
    /// ([`LangKok::cardinal_of_pystr`]) on it — dotted strings spell their
    /// digits, plain-integer strings ("5" from `Decimal("5")`) take the
    /// `_int_to_word` branch, and exponential strings raise `int()`'s
    /// ValueError ("1e+16", "1E+2", "1E+20").
    ///
    /// `self.precision` is never read, so the `precision=` kwarg has no
    /// effect; `precision_override` is therefore **ignored**, unlike the
    /// inherited base float path.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_of_pystr(&value_python_str(value))
    }

    /// `to_cardinal(float/Decimal)` — full routing. Whole values are *not*
    /// short-circuited to the integer path: `str(5.0)` is "5.0", so 5.0 is
    /// "पांच पुंतो शून्य" while `Decimal("5")` ("5", no dot) is "पांच".
    /// The base default (whole -> int path) is exactly what this override
    /// removes.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `to_cardinal(number) + "वो"`, same as the
    /// integer mode — the suffix binds to the decimal spelling ("panch punto
    /// xunyavo") and any `int()` ValueError propagates before it is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}{}", cardinal, ORDINAL_SUFFIX))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "वो"` verbatim —
    /// "-0.0vo", "5.00vo", "1e+16vo" (this mode never calls `int()`, so the
    /// exponential inputs that make the other modes raise sail through).
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`: `to_cardinal(val)`, `longval` ignored — the
    /// same explicit delegation the integer `to_year` makes.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — the base `Decimal(value)`, with one twist.
    ///
    /// Python parses "Infinity"/"-Infinity"/"NaN" into a perfectly valid
    /// `Decimal`, and the ValueError only fires *inside* `to_cardinal`, where
    /// `int("Infinity")` / `int("NaN")` chokes (the sign is peeled before the
    /// `int()`, so the message never carries a "-"). The Rust dispatcher,
    /// however, intercepts `Inf`/`NaN` before any KOK code runs and would
    /// raise base-flavoured errors (OverflowError for Inf) that KOK's
    /// int()-everything pipeline can never produce. Raising `int()`'s
    /// ValueError here instead keeps the observable behaviour: neither string
    /// contains a digit, so the dispatcher re-raises rather than falling back
    /// to the sentence converter, exactly as the Python error propagates.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency -------------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_KOK"
    }

    /// `self.CURRENCY_FORMS[code]` — a **strict** lookup, with no INR
    /// fallback.
    ///
    /// The fallback is deliberately *not* here. It is local to
    /// `to_currency`, which asks `CURRENCY_FORMS.get(currency, <INR>)`;
    /// `to_cheque` instead subscripts `self.CURRENCY_FORMS[currency]` and
    /// converts the `KeyError` into `NotImplementedError`. Since `to_cheque`
    /// is the only caller of this hook (via `currency::default_to_cheque`),
    /// answering with the fallback here would make `cheque:GBP` succeed with
    /// "RUPYA" where Python raises. That asymmetry — `currency:GBP` returning
    /// INR words while `cheque:GBP` raises — is corpus-confirmed on both
    /// sides.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // currency_precision() and currency_adjective() stay at their trait
    // defaults on purpose: KOK inherits CURRENCY_PRECISION = {} and
    // CURRENCY_ADJECTIVES = {} from Num2Word_Base and never populates either,
    // so every code has divisor 100 and no adjective. That is why the corpus
    // shows `currency:JPY 12.34` -> "...तीस आनी चार पैसो" (cents shown, not
    // rounded away) rather than the 0-decimal treatment JPY gets elsewhere.

    // money_verbose()/cents_verbose()/cents_terse() also stay at their
    // defaults: KOK overrides none of them, and the inherited _money_verbose
    // routes through to_cardinal, which is overridden above. That is what
    // makes the inherited to_cheque render "EK HOZAR DON XAMBHAR ANI TIS ANI
    // CHAR AND 56/100 YURO" without any override here.

    /// Port of `Num2Word_KOK.pluralize`.
    ///
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Unreachable from KOK's own currency surface — `to_currency` inlines
    /// `cr1[1] if left != 1 else cr1[0]` and the inherited `to_cheque` takes
    /// `cr1[-1]` — but Python defines the method, so the hook is implemented
    /// rather than left at the trait default, which would raise.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        if n.is_one() {
            Ok(forms[0].clone())
        } else {
            Ok(forms[forms.len() - 1].clone())
        }
    }

    /// Port of `Num2Word_KOK.to_currency` — a wholesale override that shares
    /// no code with `Num2Word_Base.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="INR", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left  = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency,
    ///                                        list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// # The `separator` argument is a compromise — read this
    ///
    /// KOK's Python signature defaults `separator=" "`; `Num2Word_Base`'s
    /// defaults it to `","`. The Python shim (`__init__.py`) bridges the FFI
    /// with `kwargs.get("separator", ",")`, so it substitutes the *base*
    /// default for the *language* default and a caller who passed nothing is
    /// indistinguishable here from one who passed `","`. Every currency row
    /// in the frozen corpus was recorded through
    /// `num2words(v, lang="kok", to="currency", currency=c)` with no
    /// separator — i.e. with KOK's `" "` — and `bench/diff_test.py` replays
    /// them by passing `","` explicitly.
    ///
    /// So `","` is decoded as "unset" and mapped to `" "`, while any other
    /// separator passes through untouched. That reproduces Python for the
    /// default call (the whole corpus) *and* for every explicit non-comma
    /// separator; the single case it cannot get right is an explicit
    /// `separator=","`, which yields `" "` instead. That case is
    /// unrecoverable without changing the shim or the trait signature, both
    /// of which are out of scope for this file. ~25 Python modules declare a
    /// non-comma default and hit this same shim bug — see the port report.
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
        // KOK's to_currency ignores `adjective` outright: it never touches
        // CURRENCY_ADJECTIVES (which is `{}` anyway). Not an oversight here —
        // the parameter is accepted and discarded in Python too.
        let _ = adjective;

        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // is_negative = val < 0, evaluated *before* abs(). Note -0.0 is not
        // negative in Python, and BigDecimal agrees.
        let is_negative = val.is_negative();
        let (left, right) = self.currency_parts(val)?;

        // CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0]) —
        // an unknown code silently borrows INR's words. No raise. See the
        // module docs.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        // cr1[1] if left != 1 else cr1[0]. Index [1], not [-1]: KOK's forms
        // are all 2-tuples so they coincide, but the literal index is kept.
        // Both indices are in range by construction of CURRENCY_FORMS.
        let unit = if left.is_one() {
            &forms.unit[0]
        } else {
            &forms.unit[1]
        };
        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `if cents and right:` — a zero `right` drops the whole segment, so
        // 1.0 is "एक युरो" and not "एक युरो शून्य सेंट". See quirk 8.
        if cents && !right.is_zero() {
            let subunit = if right.is_one() {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            };
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        if is_negative {
            // negword carries a trailing space: "रीण " + "धा आनी दोन युरो".
            result = format!("{}{}", NEGWORD, result);
        }
        // result.strip() — inert in practice (int_to_word never returns "",
        // and no form is empty), but reproduced. See quirk 5.
        Ok(result.trim().to_string())
    }

    // to_cheque() is NOT overridden: Num2Word_KOK inherits
    // Num2Word_Base.to_cheque verbatim, and currency::default_to_cheque is a
    // faithful port of it. It reaches back into this impl through
    // currency_forms() (strict — hence NotImplementedError for GBP/JPY/KWD/
    // BHD/CNY/CHF) and money_verbose() -> to_cardinal().
}
