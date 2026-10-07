//! Port of `lang_TA.py` (Tamil).
//!
//! Shape: **self-contained**. `Num2Word_TA` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` skips the `self.cards = OrderedDict()` branch
//! entirely and never sets `MAXVAL`. All four in-scope methods
//! (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`) are overridden
//! outright, so `cards`/`merge` stay at their trait defaults here, and in
//! Python **nothing overflows** — every integer, however large, renders. On a
//! large enough integer the port's recursion overflowed the native stack, so
//! it adds a ceiling (gladiaio/num2words2#203): `maxval` is 10^14, where the
//! crore multiplier would itself need "கோடி", and every mode raises
//! `OverflowError` from there.
//!
//! The algorithm is the Indian numbering system: crore (10^7), lakh (10^5),
//! thousand, then a special hundreds table and a teens table, joined with
//! single spaces by `" ".join(parts)`.
//!
//! Nothing is inherited from `Num2Word_Base` that matters: TA overrides every
//! in-scope entry point, and `title()` is never reached (TA's `to_cardinal`
//! does not call it), so `is_title` stays irrelevant.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **`to_ordinal` glues its suffix on with no separator.** For any `n` not
//!    in the `ordinals` table (i.e. outside 1..=10), `_int_to_ordinal` returns
//!    `self._int_to_cardinal(n) + "ஆவது"` — no space. So `to_ordinal(11)` is
//!    "பதினொன்றுஆவது" and `to_ordinal(1000)` is "ஆயிரம்ஆவது". Preserved.
//! 2. **`to_ordinal` accepts negatives and produces nonsense**, because it
//!    only special-cases `n == 0` and the 1..=10 table before falling through
//!    to the cardinal path: `to_ordinal(-42)` == "கழித்தல் நாற்பது இரண்டுஆவது"
//!    ("minus forty two-th"). Preserved.
//! 3. **The thousands scale word takes no "ஒரு"**, unlike lakh and crore:
//!    1000 renders as bare "ஆயிரம்" while 100000 is "ஒரு இலட்சம்" and 10000000
//!    is "ஒரு கோடி". Asymmetric in Python; preserved.
//! 4. ~~**Crore stacks on itself past 10^14**~~ rather than promoting to a
//!    larger scale word, because `_int_to_word` recurses on the crore
//!    quotient: in Python 10^15 == "பத்து கோடி கோடி", 10^21 ==
//!    "ஒரு கோடி கோடி கோடி". The port raises `OverflowError` there instead
//!    (gladiaio/num2words2#203).
//! 5. **`self.scale` is dead.** The Python `__init__` builds
//!    `{1000: "ஆயிரம்", 100000: "இலட்சம்", 10000000: "கோடி",
//!    1000000000000: "டிரில்லியன்"}` and never reads it — the scale words are
//!    hardcoded as string literals inside `_int_to_word`. In particular
//!    "டிரில்லியன்" (trillion) is **unreachable**: 10^12 renders via the crore
//!    path as "ஒரு இலட்சம் கோடி". Not ported (it has no observable effect).
//! 6. **`_setup` is a latent `AttributeError`.** TA defines
//!    `def _setup(self): super()._setup()`, but `Num2Word_Base` has no
//!    `_setup` (it has `setup`, which base's `__init__` calls). Calling
//!    `_setup()` would raise; nothing ever does. Not ported.
//! 7. **The `else` arm of the hundreds branch is dead code.** Python's
//!    `self.ones[n // 100] + " நூறு"` fallback can never fire: by the time
//!    control reaches it, `n < 1000` always holds (every larger scale has
//!    already been stripped with `n %= ...`), so `(n // 100) * 100` is always
//!    one of the nine keys in `hundreds_special`. Reproduced anyway in
//!    [`int_to_word`] for structural fidelity — see the comment there.
//!
//! # Currency
//!
//! TA overrides `to_currency` **outright** and shares almost nothing with
//! `Num2Word_Base.to_currency`. It inherits `to_cheque` unchanged.
//!
//! `CURRENCY_FORMS` is declared in TA's own class body, so — unlike the 16
//! classes that read `Num2Word_EUR.CURRENCY_FORMS` — it is untouched by the
//! `Num2Word_EN.__init__` mutation that rewrites EUR/GBP and injects ~24 extra
//! ISO codes into that shared dict. Confirmed against the live interpreter:
//! `Num2Word_TA.CURRENCY_FORMS is Num2Word_EN.CURRENCY_FORMS` is `False`. The
//! seven codes in [`build_currency_forms`] are the whole table, which is why
//! JPY/KWD/BHD/CNY/CHF legitimately raise NotImplementedError.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}`, so the trait
//! defaults (100, `None`) already match and are not overridden.
//!
//! ## Faithfully reproduced Python quirks (currency)
//!
//! 8. **The signature is `to_currency(n, currency="INR")`** — there is no
//!    `cents`, `separator` or `adjective` parameter at all, so all three are
//!    ignored here. (Passing any of them to the Python converter is a
//!    `TypeError`; see the crate-level note in the port report — 19 of 172
//!    converters have such a reduced signature and the dispatcher forwards the
//!    kwargs regardless.)
//! 9. **There is no `has_decimal` guard.** The cents segment is gated on
//!    `right > 0` alone, so a whole float prints no subunit: `1.0` is
//!    "ஒன்று யூரோ" where Base would print a zero-cents segment. `Decimal("5.00")`
//!    likewise renders "ஐந்து ரூபாய்". `has_decimal` is therefore ignored.
//! 10. **`pluralize` is never called.** TA takes `cr_major[0]` / `cr_minor[0]`
//!     directly, which is why it never trips Base's abstract `pluralize`. Both
//!     forms of every entry are identical anyway — Tamil does not inflect these
//!     nouns for number — but the arity of 2 is kept as the ported data.
//! 11. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** The
//!     divisor is hardcoded 100 in the `has_fractional_cents` test, and
//!     `parse_currency_parts` is called with no `divisor=` argument so it takes
//!     currency.py's default of 100. TA thus has no 3-decimal or 0-decimal
//!     path whatsoever. Moot in practice, since its `CURRENCY_PRECISION` is
//!     empty and every code resolves to 100 regardless.
//! 12. **`keep_precision` skips the ROUND_HALF_UP quantize**, so a value with
//!     fractional cents *truncates* rather than rounds: `2.675` is
//!     "இரண்டு ரூபாய் அறுபது ஏழு பைசா" (67 paise, not 68).
//! 13. ~~**Past 10^26 TA stops speaking Tamil**~~ and hands back the bare
//!     number (`to_currency(10**26)` == "100000000000000000000000000 INR").
//!     Unreachable since gladiaio/num2words2#203: the 10^14 ceiling raises
//!     `OverflowError` first.
//!
//! # Error behaviour
//!
//! For **integer** input below the 10^14 ceiling none of the four in-scope
//! methods can raise: there is no table lookup that can miss and no `int()`
//! parse. From the ceiling up every method raises `OverflowError` (#203). The
//! `try/except BaseException` wrappers around Python's
//! `to_cardinal`/`to_ordinal`/`to_ordinal_num` are a float-truncation fallback
//! (e.g. `to_cardinal(12.34)` raises `KeyError` on `teens[12.34]`, is caught,
//! and retries as `int(12.34)` → "பன்னிரண்டு"); on the integer path those
//! handlers are unreachable. The cardinal no longer truncates a float: it is
//! read with "புள்ளி" and each fractional digit (gladiaio/num2words2#206; see
//! [`LangTa::to_cardinal_float`]). `to_year` has no such wrapper at all.
//!
//! # State
//!
//! None. Every method is a pure function of its argument — no flag is set in
//! one call and consumed by another.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// "பூஜ்ஜியம்" — the zero word, returned by both the cardinal and ordinal paths.
const ZERO: &str = "பூஜ்ஜியம்";

/// `self.negword`. Note the **trailing space**: Python concatenates it
/// directly (`self.negword + self._int_to_word(-n)`) rather than joining, so
/// the space is load-bearing. Do not trim it.
const NEGWORD: &str = "கழித்தல் ";

/// Suffix appended by `_int_to_ordinal` — **without** a separating space.
const ORDINAL_SUFFIX: &str = "ஆவது";

/// Suffix appended by `to_ordinal_num`, a different word from [`ORDINAL_SUFFIX`].
const ORDINAL_NUM_SUFFIX: &str = "-வது";

/// The year path's thousand word, distinct from the cardinal "ஆயிரம்".
const YEAR_THOUSAND: &str = "ஆயிரத்து";

/// `self.ones`; index 0 is the empty string and is never emitted (guarded by
/// `if n > 0`).
const ONES: [&str; 10] = [
    "",
    "ஒன்று",
    "இரண்டு",
    "மூன்று",
    "நான்கு",
    "ஐந்து",
    "ஆறு",
    "ஏழு",
    "எட்டு",
    "ஒன்பது",
];

/// `self.tens`; index 0 is the empty string and is never emitted (guarded by
/// `if n >= 10`, which makes `n // 10 >= 1`).
const TENS: [&str; 10] = [
    "",
    "பத்து",
    "இருபது",
    "முப்பது",
    "நாற்பது",
    "ஐம்பது",
    "அறுபது",
    "எழுபது",
    "எண்பது",
    "தொண்ணூறு",
];

/// `self.teens`, a dict keyed 11..=19 in Python. Indexed here by `n - 11`.
const TEENS: [&str; 9] = [
    "பதினொன்று",
    "பன்னிரண்டு",
    "பதின்மூன்று",
    "பதினான்கு",
    "பதினைந்து",
    "பதினாறு",
    "பதினேழு",
    "பதினெட்டு",
    "பத்தொன்பது",
];

/// `self.hundreds_special`, a dict keyed 100..=900 in Python. Indexed here by
/// `n // 100 - 1`. Note 900 is "தொள்ளாயிரம்", which is morphologically a
/// *thousand* word, not a hundred word — that is what Python ships.
const HUNDREDS_SPECIAL: [&str; 9] = [
    "நூறு",
    "இருநூறு",
    "முன்னூறு",
    "நானூறு",
    "ஐநூறு",
    "அறுநூறு",
    "எழுநூறு",
    "எண்ணூறு",
    "தொள்ளாயிரம்",
];

/// `self.ordinals`, a dict keyed 1..=10 in Python. Indexed here by `n - 1`.
/// Every other value falls through to the cardinal + "ஆவது" path.
const ORDINALS: [&str; 10] = [
    "முதல்",
    "இரண்டாம்",
    "மூன்றாம்",
    "நான்காம்",
    "ஐந்தாம்",
    "ஆறாம்",
    "ஏழாம்",
    "எட்டாம்",
    "ஒன்பதாம்",
    "பத்தாம்",
];

fn crore() -> BigInt {
    BigInt::from(10_000_000u32)
}
fn lakh() -> BigInt {
    BigInt::from(100_000u32)
}
fn thousand() -> BigInt {
    BigInt::from(1_000u32)
}
fn hundred() -> BigInt {
    BigInt::from(100u32)
}

/// Python's `_int_to_word`. Expects a **non-negative** `n`: the sign is peeled
/// off by [`int_to_cardinal`] before this is ever called, and every recursive
/// call passes a positive quotient.
fn int_to_word(n: &BigInt) -> String {
    if n.is_zero() {
        return ZERO.to_string();
    }

    let mut n = n.clone();
    let mut parts: Vec<String> = Vec::new();

    // Crores (10^7). Recurses on the quotient, which is why crore stacks on
    // itself for very large values (10^15 -> "பத்து கோடி கோடி").
    let c = crore();
    if n >= c {
        let crores = n.div_floor(&c);
        if crores.is_one() {
            parts.push("ஒரு கோடி".to_string());
        } else {
            parts.push(format!("{} கோடி", int_to_word(&crores)));
        }
        n = n.mod_floor(&c);
    }

    // Lakhs (10^5).
    let l = lakh();
    if n >= l {
        let lakhs = n.div_floor(&l);
        if lakhs.is_one() {
            parts.push("ஒரு இலட்சம்".to_string());
        } else {
            parts.push(format!("{} இலட்சம்", int_to_word(&lakhs)));
        }
        n = n.mod_floor(&l);
    }

    // Thousands. Unlike lakh/crore above, the `== 1` case emits a bare
    // "ஆயிரம்" with no "ஒரு" — asymmetric in Python, preserved here.
    let t = thousand();
    if n >= t {
        let thousands = n.div_floor(&t);
        if thousands.is_one() {
            parts.push("ஆயிரம்".to_string());
        } else {
            parts.push(format!("{} ஆயிரம்", int_to_word(&thousands)));
        }
        n = n.mod_floor(&t);
    }

    // Hundreds. `n < 1000` is guaranteed here (every larger scale was stripped
    // above), so `h` is always 1..=9 and the `hundreds_special` lookup always
    // hits. Python's `else: self.ones[n // 100] + " நூறு"` fallback is
    // therefore dead code; it is kept below to mirror the structure, and the
    // `h_idx >= 9` guard is the exact condition under which Python would have
    // taken it.
    let h = hundred();
    if n >= h {
        let hundreds = n.div_floor(&h);
        // Safe: `hundreds` is 1..=9 by the argument above.
        let h_idx = hundreds.to_usize().unwrap_or(usize::MAX);
        if (1..=9).contains(&h_idx) {
            parts.push(HUNDREDS_SPECIAL[h_idx - 1].to_string());
        } else {
            // Unreachable. Python: self.ones[n // 100] + " நூறு".
            parts.push(format!("{} நூறு", ONES[h_idx % 10]));
        }
        n = n.mod_floor(&h);
    }

    // `n < 100` is guaranteed here, so the cast is total.
    let mut small = n.to_u32().unwrap_or(0);

    if 10 < small && small < 20 {
        parts.push(TEENS[(small - 11) as usize].to_string());
    } else {
        if small >= 10 {
            let tens_val = small / 10;
            parts.push(TENS[tens_val as usize].to_string());
            small %= 10;
        }
        if small > 0 {
            parts.push(ONES[small as usize].to_string());
        }
    }

    parts.join(" ")
}

/// Python's `_int_to_cardinal`.
fn int_to_cardinal(n: &BigInt) -> Result<String> {
    check_maxval(n, maxval_ceiling())?;
    if n.is_zero() {
        return Ok(ZERO.to_string());
    }
    if n.sign() == num_bigint::Sign::Minus {
        // NEGWORD carries its own trailing space — plain concatenation.
        return Ok(format!("{}{}", NEGWORD, int_to_word(&(-n))));
    }
    Ok(int_to_word(n))
}

/// Python's `_int_to_ordinal`.
fn int_to_ordinal(n: &BigInt) -> Result<String> {
    if n.is_zero() {
        return Ok(ZERO.to_string());
    }
    // `n in self.ordinals` — keys 1..=10 only, so negatives never match.
    if let Some(i) = n.to_usize() {
        if (1..=10).contains(&i) {
            return Ok(ORDINALS[i - 1].to_string());
        }
    }
    // Note: no separator before the suffix. This is Python's behaviour.
    Ok(format!("{}{}", int_to_cardinal(n)?, ORDINAL_SUFFIX))
}

/// Python's `self.to_cardinal(float(right))`, the fractional-cents branch.
///
/// TA's `to_cardinal` is
/// `try: self._int_to_cardinal(n) except BaseException: self._int_to_cardinal(int(n))`,
/// and handing it a float always lands on one of two outcomes, both equal to
/// `_int_to_cardinal(int(n))`:
///
/// * the float has a fractional part, so `_int_to_word` reaches
///   `self.tens[n // 10]` or `self.ones[n]` (TypeError: list indices must be
///   integers) or `self.teens[n]` (KeyError), and the handler retries with
///   `int(n)`. The fraction always survives to one of those lookups, because
///   every scale is stripped with `n %= ...`, which preserves it; or
/// * the float is whole and the lookup happens to succeed — `self.teens[15.0]`
///   and `self.hundreds_special[100.0]` both hit, since `15.0 == 15` and
///   `hash(15.0) == hash(15)` — in which case it returns exactly the string
///   the int would have produced.
///
/// So this collapses to a truncation. The `float()` round-trip is kept because
/// it is observable: `int(Decimal("2.9999999999999999999"))` is 2, but
/// `int(float(Decimal("2.9999999999999999999")))` is 3, and Python takes the
/// latter. Likewise `1.999999999999999999999` yields 99.9999999999999999999
/// paise, which `float()` rounds to exactly `100.0` -> "நூறு".
///
/// `right` is `fraction * 100` with `fraction` in `[0, 1)`, so the value is in
/// `[0, 100)` and the `as i64` cast cannot overflow.
fn cardinal_from_fractional_cents(right: &BigDecimal) -> Result<String> {
    // `to_f64` formats the coefficient and defers to Rust's float parser, so
    // it is correctly rounded exactly as Python's `float(Decimal)` is.
    let truncated = right.to_f64().map(f64::trunc).unwrap_or(0.0);
    int_to_cardinal(&BigInt::from(truncated as i64))
}

/// `Num2Word_TA.CURRENCY_FORMS`, verbatim from TA's own class body.
///
/// Each entry carries two identical forms because Tamil does not inflect these
/// nouns for number. The arity of 2 mirrors the Python tuples and is kept even
/// though TA only ever reads index 0 (`cr_major[0]` / `cr_minor[0]`) and
/// `to_cheque` only ever reads index -1.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const RUPEE: [&str; 2] = ["ரூபாய்", "ரூபாய்"];
    const DOLLAR: [&str; 2] = ["டாலர்", "டாலர்"];
    const CENT: [&str; 2] = ["சென்ட்", "சென்ட்"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("INR", CurrencyForms::new(&RUPEE, &["பைசா", "பைசா"]));
    m.insert("LKR", CurrencyForms::new(&RUPEE, &["சதம்", "சதம்"]));
    m.insert("USD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("EUR", CurrencyForms::new(&["யூரோ", "யூரோ"], &CENT));
    m.insert("GBP", CurrencyForms::new(&["பவுண்ட்", "பவுண்ட்"], &["பென்னி", "பென்னி"]));
    m.insert("SGD", CurrencyForms::new(&DOLLAR, &CENT));
    m.insert("MYR", CurrencyForms::new(&["ரிங்கிட்", "ரிங்கிட்"], &["சென்", "சென்"]));
    m
}

pub struct LangTa {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangTa {
    pub fn new() -> Self {
        LangTa {
            // Built once here, never per call.
            currency_forms: build_currency_forms(),
        }
    }
}

impl Default for LangTa {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// கோடி (10^7), so from 10^14 the crore multiplier would itself need கோடி and
/// the word would stack ("கோடி கோடி").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(14))
}

impl Lang for LangTa {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "INR"
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "புள்ளி"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(int_to_cardinal(value)?)
    }

    /// TA's float/Decimal cardinal path.
    ///
    /// TA overrides `to_cardinal` (not `to_cardinal_float`), and its whole body
    /// is:
    ///
    /// ```python
    /// def to_cardinal(self, n):
    ///     try:
    ///         if isinstance(n, str): n = int(n)
    ///         return self._int_to_cardinal(n)
    ///     except BaseException:
    ///         return self._int_to_cardinal(int(n))
    /// ```
    ///
    /// A non-integer raised inside `_int_to_word` and the retry truncated it,
    /// so in Python the fraction was dropped entirely (`2.5` -> "இரண்டு") and
    /// `-0.5` lost its sign. Fixed (gladiaio/num2words2#206): the value is read
    /// like `Num2Word_Base.to_cardinal_float` — the integer part, the decimal
    /// word "புள்ளி" (TA's own `pointword`), then each fractional digit:
    /// `2.5` -> "இரண்டு புள்ளி ஐந்து". Whole values never get here (the
    /// default `cardinal_float_entry` sends them to the integer path).
    ///
    /// `precision_override` (the `precision=` kwarg) sets the digit count, as
    /// it does for the Base reading.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        default_to_cardinal_float(self, value, precision_override)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        int_to_ordinal(value)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // Python: str(n) + "-வது". No ordinals table, no sign handling —
        // to_ordinal_num(-1) == "-1-வது".
        Ok(format!("{}{}", value, ORDINAL_NUM_SUFFIX))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // The thousands split would otherwise slip a value past the ceiling.
        check_maxval(value, maxval_ceiling())?;
        let t = thousand();

        // n < 1000: plain cardinal. This arm also swallows every negative
        // year, so `to_year(-500)` == "கழித்தல் ஐநூறு".
        if value < &t {
            return Ok(int_to_cardinal(value)?);
        }

        let remainder = value.mod_floor(&t);

        if value < &BigInt::from(2000u32) {
            // Python computes `thousands` here and never uses it: the word is
            // the bare literal "ஆயிரத்து".
            let mut result = YEAR_THOUSAND.to_string();
            if is_positive(&remainder) {
                result.push(' ');
                result.push_str(&int_to_cardinal(&remainder)?);
            }
            return Ok(result);
        }

        let thousands = value.div_floor(&t);
        let mut result = format!("{} {}", int_to_cardinal(&thousands)?, YEAR_THOUSAND);
        if is_positive(&remainder) {
            result.push(' ');
            result.push_str(&int_to_cardinal(&remainder)?);
        }
        Ok(result)
    }

    /// `to_ordinal(float/Decimal)` — collapses to
    /// `_int_to_ordinal(int(n))`, exactly like the cardinal path.
    ///
    /// Python's `to_ordinal` wraps `_int_to_ordinal(n)` in
    /// `except BaseException: return self._int_to_ordinal(int(n))`. A whole
    /// float/Decimal either hits a hash-equal dict key (`5.0 in
    /// self.ordinals`, `teens[11.0]`, `hundreds_special[100.0]`) and returns
    /// the identical string, or raises on a list index
    /// (`self.tens[2.0]` → TypeError) and retries with `int(n)` — same
    /// string either way. A fractional value always raises somewhere
    /// (`ordinals`/`teens` miss, or list index) and retries with the
    /// truncation: `to_ordinal(1.5)` → `_int_to_ordinal(1)` → "முதல்",
    /// `to_ordinal(-1.5)` → `_int_to_ordinal(-1)` → "கழித்தல் ஒன்றுஆவது".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        int_to_ordinal(&ta_trunc(value))
    }

    /// `to_ordinal_num(float/Decimal)` — `str(n) + "-வது"` for every numeric
    /// input (the `int(n)` in the try-block only fires for `str` input, which
    /// never reaches this hook). `repr_str` is Python's `str(value)`, so
    /// `5.00` keeps its trailing zeros and `1E+2` its exponent form.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_NUM_SUFFIX))
    }

    /// `to_year(float/Decimal)` — **no** try/except wrapper in Python, so
    /// the non-int value walks `_int_to_cardinal`/`_int_to_word` live and
    /// the list-index lookups raise TypeError uncaught. Only values whose
    /// residual digits are absorbed entirely by dict lookups
    /// (`hundreds_special`, `teens`) or by `== 1` scale quotients survive:
    /// `to_year(1000.0)` → "ஆயிரத்து", `to_year(1e20)` →
    /// "ஆயிரம் கோடி கோடி ஆயிரத்து", but `to_year(5.0)`,
    /// `to_year(1234.0)`, `to_year(-3.0)` all raise TypeError on
    /// `self.ones[...]`/`self.tens[...]`. See [`ta_year_sim`].
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let (x, is_decimal) = match value {
            FloatValue::Float { value, .. } => (
                BigDecimal::from_f64(*value).ok_or_else(|| {
                    // NaN/inf never reach the year hook (the shim keeps
                    // non-finite input on the Python side).
                    N2WError::Value(format!("cannot convert {} to Decimal", value))
                })?,
                false,
            ),
            FloatValue::Decimal { value, .. } => (value.clone(), true),
        };
        // The simulation recurses like `_int_to_word`, so it needs the same
        // ceiling (#203).
        check_maxval(&x.abs().with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        ta_year_sim(&x, is_decimal)
    }

    // ---- currency -------------------------------------------------------
    //
    // TA overrides `to_currency` wholesale and inherits `to_cheque` from
    // `Num2Word_Base` unchanged, so only the class name, the forms table and
    // `to_currency` itself are language-specific here.
    //
    // Deliberately NOT overridden, because the trait defaults already match:
    //   * `currency_precision` — TA's CURRENCY_PRECISION is `{}`, so
    //     `.get(code, 100)` is 100 for every code, which is the default.
    //   * `currency_adjective` — CURRENCY_ADJECTIVES is `{}` -> None.
    //   * `pluralize` — TA never calls it; Base's is abstract and the default
    //     raises NotImplemented, which is the faithful behaviour if reached.
    //   * `money_verbose` — Base's `_money_verbose` is `self.to_cardinal(n)`,
    //     and the default routes to `to_cardinal` -> `int_to_cardinal`. That
    //     is what `to_cheque` needs.
    //   * `cardinal_from_decimal` — unreachable: TA's own `to_currency` never
    //     delegates to `default_to_currency`, and its fractional-cents branch
    //     goes through [`cardinal_from_fractional_cents`] instead.

    fn lang_name(&self) -> &str {
        "Num2Word_TA"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_TA.to_currency(n, currency="INR")`.
    ///
    /// `cents`, `separator` and `adjective` do not exist on the Python
    /// signature, so they are ignored (quirk 8). `has_decimal` is ignored too:
    /// the cents segment is gated on `right > 0` alone (quirk 9).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Past the #203 ceiling, before anything else reads the value. Python's
        // `except BaseException` fallback to the bare number (only reachable
        // from 10**26) is therefore gone: a value that large raises instead.
        let whole = match val {
            CurrencyValue::Int(v) => v.clone(),
            CurrencyValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
        };
        check_maxval(&whole, maxval_ceiling())?;

        // `(decimal_val * 100) % 1 != 0`. An int can never have fractional
        // cents — the product is integral.
        let has_fractional_cents = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => {
                let scaled = value * BigDecimal::from(100);
                // with_scale(0) truncates toward zero, matching Decimal's
                // sign-of-dividend `%`; either way this only tests != 0.
                &scaled - scaled.with_scale(0) != BigDecimal::zero()
            }
        };

        // divisor 100, not `self.currency_precision(currency)`: Python calls
        // `parse_currency_parts(n, is_int_with_cents=False,
        // keep_precision=has_fractional_cents)` and lets currency.py's
        // `divisor=100` default stand (quirk 11).
        let (left, right, is_negative) = parse_currency_parts(val, false, has_fractional_cents, 100);

        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        let mut result: Vec<String> = Vec::new();

        // Python appends `self.negword.strip()`: the trailing space is
        // stripped here and put back by the `" ".join(result)` below. `left`
        // is already the absolute value (parse_currency_parts abs()es it), so
        // `int_to_cardinal` cannot prepend a second negword.
        if is_negative {
            result.push(NEGWORD.trim().to_string());
        }
        result.push(int_to_cardinal(&left)?);
        result.push(forms.unit[0].clone());

        // Python: `if right > 0`. This is the *only* cents guard, so a whole
        // float (1.0) or a scaled Decimal ("5.00") prints no subunit.
        if right > BigDecimal::zero() {
            // Python: `if isinstance(right, Decimal)`. `right` is a Decimal
            // exactly when parse_currency_parts kept precision, i.e. exactly
            // when has_fractional_cents — otherwise it returned a plain int.
            let right_words = if has_fractional_cents {
                cardinal_from_fractional_cents(&right)?
            } else {
                // Safe: !keep_precision leaves `cents` at scale 0, so the
                // coefficient is the value.
                int_to_cardinal(&right.as_bigint_and_exponent().0)?
            };
            result.push(right_words);
            result.push(forms.subunit[0].clone());
        }

        Ok(result.join(" "))
    }
}

/// Mirrors Python's `if remainder > 0` on the year path. `remainder` there is
/// always non-negative (the branch is guarded by `value >= 1000`), so this is
/// effectively `!= 0`.
fn is_positive(n: &BigInt) -> bool {
    n.sign() == num_bigint::Sign::Plus
}

/// `int(n)` for a float/Decimal — truncation toward zero, the value both
/// `except BaseException` retries feed to `_int_to_cardinal`/`_int_to_ordinal`.
fn ta_trunc(value: &FloatValue) -> BigInt {
    match value {
        // trunc(-0.5) == -0.0 -> BigInt 0, so the sign is dropped, matching
        // Python's int(-0.5) == 0.
        FloatValue::Float { value, .. } => {
            BigInt::from_f64(value.trunc()).unwrap_or_else(BigInt::zero)
        }
        FloatValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
    }
}

/// `self.ones[<float/Decimal>]` / `self.tens[<float/Decimal>]` — CPython's
/// message for a list indexed with a non-int. Only the exception *type* is
/// corpus-observable.
fn ta_type_error(is_decimal: bool) -> N2WError {
    N2WError::Type(format!(
        "list indices must be integers or slices, not {}",
        if is_decimal { "decimal.Decimal" } else { "float" }
    ))
}

/// Positive floor division, all operands positive: Python's `//` on
/// float/Decimal truncates here. Exact for every corpus value (the floats
/// are powers of ten or small, so the f64 ops Python performs are exact and
/// agree with this arbitrary-precision replay).
fn ta_floordiv(a: &BigDecimal, b: &BigDecimal) -> BigInt {
    (a / b).with_scale(0).as_bigint_and_exponent().0
}

/// Positive `a % b`, consistent with [`ta_floordiv`].
fn ta_mod(a: &BigDecimal, b: &BigDecimal) -> BigDecimal {
    a - BigDecimal::from(ta_floordiv(a, b)) * b
}

/// `_int_to_word(n)` replayed with a **non-int-typed** `n` (float or
/// Decimal), as `to_year` runs it — live, with no `except BaseException`
/// safety net:
///
/// * dict lookups (`teens`, `hundreds_special`) succeed by hash equality
///   whenever the value is whole (`teens[12.0]` hits);
/// * list indexing (`self.tens[n // 10]`, `self.ones[n]`) raises
///   **TypeError** for any non-int index — whole or not;
/// * a fractional value in the open teens interval misses the dict:
///   **KeyError**;
/// * scale quotients recurse through this same function (they are
///   floats/Decimals themselves), and `== 1` comparisons are numeric, so
///   `1000.0 // 1000 == 1` takes the bare-"ஆயிரம்" arm without indexing.
///
/// Net effect: only values whose residue at every level is 0, a whole teen,
/// or absorbed by `hundreds_special`/`== 1` produce words; everything else
/// raises exactly where Python does.
fn ta_word_sim(n: &BigDecimal, is_decimal: bool) -> Result<String> {
    if n.is_zero() {
        return Ok(ZERO.to_string());
    }

    let mut n = n.clone();
    let mut parts: Vec<String> = Vec::new();

    // Crores (10^7): `crores = n // 10000000` recurses on the quotient.
    let crore_bd = BigDecimal::from(10_000_000u32);
    if n >= crore_bd {
        let crores = BigDecimal::from(ta_floordiv(&n, &crore_bd));
        if crores == BigDecimal::from(1u32) {
            parts.push("ஒரு கோடி".to_string());
        } else {
            parts.push(format!("{} கோடி", ta_word_sim(&crores, is_decimal)?));
        }
        n = ta_mod(&n, &crore_bd);
    }

    // Lakhs (10^5).
    let lakh_bd = BigDecimal::from(100_000u32);
    if n >= lakh_bd {
        let lakhs = BigDecimal::from(ta_floordiv(&n, &lakh_bd));
        if lakhs == BigDecimal::from(1u32) {
            parts.push("ஒரு இலட்சம்".to_string());
        } else {
            parts.push(format!("{} இலட்சம்", ta_word_sim(&lakhs, is_decimal)?));
        }
        n = ta_mod(&n, &lakh_bd);
    }

    // Thousands — the `== 1` case is the bare "ஆயிரம்", as in the int path.
    let thousand_bd = BigDecimal::from(1000u32);
    if n >= thousand_bd {
        let thousands = BigDecimal::from(ta_floordiv(&n, &thousand_bd));
        if thousands == BigDecimal::from(1u32) {
            parts.push("ஆயிரம்".to_string());
        } else {
            parts.push(format!("{} ஆயிரம்", ta_word_sim(&thousands, is_decimal)?));
        }
        n = ta_mod(&n, &thousand_bd);
    }

    // Hundreds: `(n // 100) * 100` is a whole 100..=900 whenever this branch
    // runs, and Python's dict lookup hits it by hash equality — so this arm
    // never raises, even for a fractional n.
    let hundred_bd = BigDecimal::from(100u32);
    if n >= hundred_bd {
        let h = ta_floordiv(&n, &hundred_bd);
        let h_idx = h.to_usize().unwrap_or(0);
        debug_assert!((1..=9).contains(&h_idx));
        parts.push(HUNDREDS_SPECIAL[h_idx - 1].to_string());
        n = ta_mod(&n, &hundred_bd);
    }

    let ten_bd = BigDecimal::from(10u32);
    let twenty_bd = BigDecimal::from(20u32);
    if n > ten_bd && n < twenty_bd {
        // `self.teens[n]` — dict: whole values hit by hash equality,
        // fractional ones raise KeyError.
        if n.is_integer() {
            let i = n
                .with_scale(0)
                .as_bigint_and_exponent()
                .0
                .to_usize()
                .expect("teens index 11..=19 fits usize");
            parts.push(TEENS[i - 11].to_string());
        } else {
            return Err(N2WError::Key(format!("{}", n)));
        }
    } else {
        if n >= ten_bd {
            // `self.tens[n // 10]` — list index with a float/Decimal.
            return Err(ta_type_error(is_decimal));
        }
        if n > BigDecimal::from(0u32) {
            // `self.ones[n]` — same.
            return Err(ta_type_error(is_decimal));
        }
    }

    Ok(parts.join(" "))
}

/// `_int_to_cardinal(n)` over a non-int-typed `n`, no safety net.
fn ta_cardinal_sim(n: &BigDecimal, is_decimal: bool) -> Result<String> {
    if n.is_zero() {
        // `-0.0 == 0` is true, so negative zero lands here and loses its sign.
        return Ok(ZERO.to_string());
    }
    if n.is_negative() {
        let pos = -n.clone();
        return Ok(format!("{}{}", NEGWORD, ta_word_sim(&pos, is_decimal)?));
    }
    ta_word_sim(n, is_decimal)
}

/// `to_year(<float/Decimal>)` replayed live (see
/// [`Lang::year_float_entry`] on [`LangTa`]).
fn ta_year_sim(n: &BigDecimal, is_decimal: bool) -> Result<String> {
    let thousand_bd = BigDecimal::from(1000u32);
    if *n < thousand_bd {
        // Also swallows every negative year; the cardinal sim raises the
        // TypeError itself where the digits demand it.
        return ta_cardinal_sim(n, is_decimal);
    }

    // n >= 1000 here, so `//`/`%` are plain truncation.
    let remainder = ta_mod(n, &thousand_bd);
    if *n < BigDecimal::from(2000u32) {
        // Python computes `thousands` and never uses it on this branch.
        let mut result = YEAR_THOUSAND.to_string();
        if remainder > BigDecimal::from(0u32) {
            result.push(' ');
            result.push_str(&ta_cardinal_sim(&remainder, is_decimal)?);
        }
        return Ok(result);
    }

    let thousands = BigDecimal::from(ta_floordiv(n, &thousand_bd));
    let mut result = format!("{} {}", ta_cardinal_sim(&thousands, is_decimal)?, YEAR_THOUSAND);
    if remainder > BigDecimal::from(0u32) {
        result.push(' ');
        result.push_str(&ta_cardinal_sim(&remainder, is_decimal)?);
    }
    Ok(result)
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    /// Drives the same entry point the corpus harness does. `arg` is Python's
    /// `repr(value)`, so "100" is an int and "12.34" a float.
    fn cur(arg: &str, currency: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let val = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangTa::new().to_currency(&val, currency, true, None, false)
    }

    fn cheque(arg: &str, currency: &str) -> Result<String> {
        use std::str::FromStr;
        LangTa::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), currency)
    }

    /// Frozen-corpus rows, verbatim.
    #[test]
    fn corpus_currency() {
        for (arg, want) in [
            ("0", "பூஜ்ஜியம் யூரோ"),
            ("1", "ஒன்று யூரோ"),
            ("2", "இரண்டு யூரோ"),
            ("100", "நூறு யூரோ"),
            ("12.34", "பன்னிரண்டு யூரோ முப்பது நான்கு சென்ட்"),
            ("0.01", "பூஜ்ஜியம் யூரோ ஒன்று சென்ட்"),
            // A whole float still prints no cents: TA has no has_decimal guard.
            ("1.0", "ஒன்று யூரோ"),
            ("99.99", "தொண்ணூறு ஒன்பது யூரோ தொண்ணூறு ஒன்பது சென்ட்"),
            ("1234.56", "ஆயிரம் இருநூறு முப்பது நான்கு யூரோ ஐம்பது ஆறு சென்ட்"),
            ("-12.34", "கழித்தல் பன்னிரண்டு யூரோ முப்பது நான்கு சென்ட்"),
            ("1000000", "பத்து இலட்சம் யூரோ"),
            ("0.5", "பூஜ்ஜியம் யூரோ ஐம்பது சென்ட்"),
        ] {
            assert_eq!(cur(arg, "EUR").unwrap(), want, "EUR {}", arg);
        }
        assert_eq!(cur("12.34", "INR").unwrap(), "பன்னிரண்டு ரூபாய் முப்பது நான்கு பைசா");
        assert_eq!(cur("12.34", "USD").unwrap(), "பன்னிரண்டு டாலர் முப்பது நான்கு சென்ட்");
        assert_eq!(cur("12.34", "GBP").unwrap(), "பன்னிரண்டு பவுண்ட் முப்பது நான்கு பென்னி");
        assert_eq!(cur("1.05", "LKR").unwrap(), "ஒன்று ரூபாய் ஐந்து சதம்");
        assert_eq!(cur("1.05", "MYR").unwrap(), "ஒன்று ரிங்கிட் ஐந்து சென்");
        assert_eq!(cur("1.05", "SGD").unwrap(), "ஒன்று டாலர் ஐந்து சென்ட்");
    }

    /// TA's table has seven codes and nothing else; JPY/KWD/BHD/CNY/CHF are
    /// corpus NotImplementedError rows.
    #[test]
    fn corpus_currency_not_implemented() {
        for code in ["JPY", "KWD", "BHD", "CNY", "CHF"] {
            match cur("12.34", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_TA\"", code)
                ),
                other => panic!("{}: expected NotImplemented, got {:?}", code, other),
            }
        }
    }

    #[test]
    fn corpus_cheque() {
        for (code, want) in [
            ("EUR", "ஆயிரம் இருநூறு முப்பது நான்கு AND 56/100 யூரோ"),
            ("USD", "ஆயிரம் இருநூறு முப்பது நான்கு AND 56/100 டாலர்"),
            ("GBP", "ஆயிரம் இருநூறு முப்பது நான்கு AND 56/100 பவுண்ட்"),
            ("INR", "ஆயிரம் இருநூறு முப்பது நான்கு AND 56/100 ரூபாய்"),
        ] {
            assert_eq!(cheque("1234.56", code).unwrap(), want, "cheque {}", code);
        }
        for code in ["JPY", "KWD", "BHD", "CNY", "CHF"] {
            assert!(matches!(cheque("1234.56", code), Err(N2WError::NotImplemented(_))));
        }
        assert_eq!(
            cheque("-1234.56", "INR").unwrap(),
            "MINUS ஆயிரம் இருநூறு முப்பது நான்கு AND 56/100 ரூபாய்"
        );
        assert_eq!(cheque("1.05", "INR").unwrap(), "ஒன்று AND 05/100 ரூபாய்");
    }

    #[test]
    fn negatives_take_one_stripped_negword() {
        assert_eq!(cur("-0.5", "INR").unwrap(), "கழித்தல் பூஜ்ஜியம் ரூபாய் ஐம்பது பைசா");
        assert_eq!(cur("-5", "INR").unwrap(), "கழித்தல் ஐந்து ரூபாய்");
        assert_eq!(cur("-1.0", "INR").unwrap(), "கழித்தல் ஒன்று ரூபாய்");
        assert_eq!(cur("-0.01", "INR").unwrap(), "கழித்தல் பூஜ்ஜியம் ரூபாய் ஒன்று பைசா");
    }

    /// keep_precision skips the ROUND_HALF_UP quantize, so fractional cents
    /// truncate. All values checked against the live Python converter.
    #[test]
    fn fractional_cents_truncate() {
        assert_eq!(cur("2.675", "INR").unwrap(), "இரண்டு ரூபாய் அறுபது ஏழு பைசா");
        assert_eq!(cur("0.655", "INR").unwrap(), "பூஜ்ஜியம் ரூபாய் அறுபது ஐந்து பைசா");
        assert_eq!(cur("12.345", "INR").unwrap(), "பன்னிரண்டு ரூபாய் முப்பது நான்கு பைசா");
        // 15.5 paise -> teens[15.5] KeyError -> int(15.5).
        assert_eq!(cur("1.155", "INR").unwrap(), "ஒன்று ரூபாய் பதினைந்து பைசா");
        // 0.5 paise -> ones[0.5] TypeError -> int(0.5) == 0.
        assert_eq!(cur("1.005", "INR").unwrap(), "ஒன்று ரூபாய் பூஜ்ஜியம் பைசா");
        assert_eq!(cur("1.00001", "INR").unwrap(), "ஒன்று ரூபாய் பூஜ்ஜியம் பைசா");
    }

    /// Python does `int(float(right))`, not `int(right)`, and the difference
    /// is observable.
    #[test]
    fn fractional_cents_go_through_float() {
        // 2.9999999999999999999 paise: float() rounds to 3.0, so "மூன்று" —
        // int(Decimal(...)) would have truncated to 2.
        assert_eq!(cur("1.029999999999999999999", "INR").unwrap(), "ஒன்று ரூபாய் மூன்று பைசா");
        // 99.9999999999999999999 paise: float() rounds to exactly 100.0, and
        // _int_to_word(100.0) succeeds outright via hundreds_special[100.0].
        assert_eq!(cur("1.999999999999999999999", "INR").unwrap(), "ஒன்று ரூபாய் நூறு பைசா");
    }

    /// The #203 ceiling comes before Python's 10**26 bare-number fallback
    /// (bug 13), which is no longer reachable.
    #[test]
    fn currency_overflows_at_the_ceiling() {
        assert!(cur("99999999999999", "INR").unwrap().starts_with("தொண்ணூறு ஒன்பது இலட்சம்"));
        for arg in ["100000000000000", "-100000000000000", "1e26", "1e30"] {
            assert!(matches!(cur(arg, "INR"), Err(N2WError::Overflow(_))), "{}", arg);
        }
        assert!(matches!(
            cur("1000000000000000000000000000000", "JPY"),
            Err(N2WError::Overflow(_))
        ));
    }

    /// Float cardinal path == truncation toward zero. Every row is a frozen
    /// corpus `"to": "cardinal"` float row (arg has a dot).
    #[test]
    fn corpus_cardinal_float() {
        let l = LangTa::new();
        // (raw f64, repr-derived precision, expected). Python dropped every
        // fraction (0.5 -> "பூஜ்ஜியம்"); since #206 it is read after "புள்ளி".
        let cases: &[(f64, u32, &str)] = &[
            (0.0, 1, "பூஜ்ஜியம்"),
            (0.5, 1, "பூஜ்ஜியம் புள்ளி ஐந்து"),
            (1.0, 1, "ஒன்று"),
            (1.5, 1, "ஒன்று புள்ளி ஐந்து"),
            (2.25, 2, "இரண்டு புள்ளி இரண்டு ஐந்து"),
            (3.14, 2, "மூன்று புள்ளி ஒன்று நான்கு"),
            (0.01, 2, "பூஜ்ஜியம் புள்ளி பூஜ்ஜியம் ஒன்று"),
            (0.1, 1, "பூஜ்ஜியம் புள்ளி ஒன்று"),
            (0.99, 2, "பூஜ்ஜியம் புள்ளி ஒன்பது ஒன்பது"),
            (1.01, 2, "ஒன்று புள்ளி பூஜ்ஜியம் ஒன்று"),
            (12.34, 2, "பன்னிரண்டு புள்ளி மூன்று நான்கு"),
            (99.99, 2, "தொண்ணூறு ஒன்பது புள்ளி ஒன்பது ஒன்பது"),
            (100.5, 1, "நூறு புள்ளி ஐந்து"),
            (1234.56, 2, "ஆயிரம் இருநூறு முப்பது நான்கு புள்ளி ஐந்து ஆறு"),
            (-0.5, 1, "கழித்தல் பூஜ்ஜியம் புள்ளி ஐந்து"),
            (-1.5, 1, "கழித்தல் ஒன்று புள்ளி ஐந்து"),
            (-12.34, 2, "கழித்தல் பன்னிரண்டு புள்ளி மூன்று நான்கு"),
            (1.005, 3, "ஒன்று புள்ளி பூஜ்ஜியம் பூஜ்ஜியம் ஐந்து"),
            (2.675, 3, "இரண்டு புள்ளி ஆறு ஏழு ஐந்து"),
        ];
        for (v, p, want) in cases {
            let fv = FloatValue::Float { value: *v, precision: *p };
            assert_eq!(l.cardinal_float_entry(&fv, None).unwrap(), *want, "float {}", v);
        }
    }

    /// Decimal cardinal path (`"to": "cardinal_dec"` corpus rows) — exact,
    /// never routed through f64 (#603), and no longer truncated (#206).
    #[test]
    fn corpus_cardinal_decimal() {
        use std::str::FromStr;
        let l = LangTa::new();
        let cases: &[(&str, u32, &str)] = &[
            ("0.01", 2, "பூஜ்ஜியம் புள்ளி பூஜ்ஜியம் ஒன்று"),
            ("1.10", 2, "ஒன்று புள்ளி ஒன்று பூஜ்ஜியம்"),
            ("12.345", 3, "பன்னிரண்டு புள்ளி மூன்று நான்கு ஐந்து"),
            ("98746251323029.99", 2, "தொண்ணூறு எட்டு இலட்சம் எழுபது நான்கு ஆயிரம் அறுநூறு இருபது ஐந்து கோடி பதின்மூன்று இலட்சம் இருபது மூன்று ஆயிரம் இருபது ஒன்பது புள்ளி ஒன்பது ஒன்பது"),
            ("0.001", 3, "பூஜ்ஜியம் புள்ளி பூஜ்ஜியம் பூஜ்ஜியம் ஒன்று"),
            ("-1.5", 1, "கழித்தல் ஒன்று புள்ளி ஐந்து"),
            ("-0.5", 1, "கழித்தல் பூஜ்ஜியம் புள்ளி ஐந்து"),
        ];
        for (s, p, want) in cases {
            let fv = FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: *p,
            };
            assert_eq!(l.cardinal_float_entry(&fv, None).unwrap(), *want, "decimal {}", s);
        }
    }

    /// `cents` / `separator` / `adjective` do not exist on TA's Python
    /// signature, so nothing may react to them.
    #[test]
    fn extra_kwargs_are_inert() {
        let l = LangTa::new();
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        let want = "பன்னிரண்டு ரூபாய் முப்பது நான்கு பைசா";
        assert_eq!(l.to_currency(&v, "INR", true, None, false).unwrap(), want);
        assert_eq!(l.to_currency(&v, "INR", false, Some(" X"), true).unwrap(), want);
    }
}
