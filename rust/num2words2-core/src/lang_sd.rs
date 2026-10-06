//! Port of `lang_SD.py` (Sindhi).
//!
//! Shape: **self-contained**. `Num2Word_SD` subclasses `Num2Word_Base` but its
//! `setup()` defines none of `high_numwords`/`mid_numwords`/`low_numwords`, so
//! the `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires:
//! `self.cards` is never built and `self.MAXVAL` is never set. `to_cardinal` is
//! overridden outright and drives a hand-rolled `_int_to_word` recursion.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here, and
//! there is **no overflow check** — arbitrarily large input is accepted (see
//! bug 3 below for what it returns).
//!
//! All in-scope modes are overridden by the Python class itself. Python's one
//! `to_cardinal` serves both integer and float input (its `"." in n` branch);
//! the Rust trait splits those, so the integer half lands in `to_cardinal` and
//! the fractional half in `to_cardinal_float` (see that method for the float/
//! Decimal port and its two out-of-corpus divergences):
//!   * `to_cardinal(n)`    -> the `_int_to_word` recursion, with sign handling
//!   * `to_ordinal(n)`     -> `to_cardinal(n) + "-و"`
//!   * `to_ordinal_num(n)` -> `str(n) + "."`  (note: *not* the base default,
//!     which returns `str(value)` with no dot)
//!   * `to_year(val, longval=True)` -> `to_cardinal(val)`, ignoring `longval`
//!     entirely. Identical to the base default, but overridden explicitly here
//!     to mirror the Python source.
//!
//! `Num2Word_SD` additionally overrides `to_currency` outright, but **not**
//! `to_cheque`, `pluralize`, `_money_verbose`, `_cents_verbose` or
//! `_cents_terse`; see "Currency" below.
//!
//! Nothing in this module reads or writes instance state across calls. The
//! struct below holds only the `CURRENCY_FORMS` table, built once in
//! [`LangSd::new`] (which the generated registry caches in a `OnceLock`) and
//! thereafter read-only, so the Python dispatcher's `_plain_int` handshake
//! caveat still does not apply.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following are all wrong-looking but are
//! exactly what Python emits, verified against the interpreter and against the
//! frozen corpus:
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** Python's `_int_to_word` opens
//!    with `return self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is
//!    the empty string (a falsy placeholder so that `ones[n]` indexes by
//!    digit), so Python always answered with the English "zero". This port
//!    says the Sindhi ٻڙي instead: `to_cardinal(0)` == "ٻڙي". See [`ZERO`].
//! 2. **No teens.** Sindhi has distinct words for 11-19, but the `< 100` branch
//!    unconditionally composes `tens[n // 10] + " " + ones[n % 10]`. So 11 is
//!    "ڏهه هڪ" — literally "ten one" — 13 is "ڏهه ٽي", and 19 is "ڏهه نو". This
//!    propagates upward: 12345 == "ڏهه ٻه هزار ٽي سو چاليهه پنج" ("ten two
//!    thousand ..."). Preserved verbatim.
//! 3. **Everything >= 10^9 renders as bare ASCII digits.** The `_int_to_word`
//!    branch chain ends in `else: return str(number)  # Fallback for very large
//!    numbers`, so `to_cardinal(10**9)` == `"1000000000"` and
//!    `to_cardinal(10**21)` == `"1000000000000000000000"`. No exception, no
//!    words. Ordinal inherits it: `to_ordinal(10**9)` == `"1000000000-و"`.
//! 4. **`million` is "لک" (lakh) but is applied at 10^6.** A lakh is 10^5 in the
//!    South Asian system, so the scale name is off by a factor of ten, and the
//!    genuine Sindhi grouping (lakh/crore) is not used at all — the code groups
//!    in Western thousands/millions. Hence 10^6 == "هڪ لک" ("one lakh") and
//!    123456789 == "هڪ سو ويهه ٽي لک ..." ("one hundred twenty three lakh ...").
//! 5. **`negword`** is the English "minus " in Python; this port uses منفي
//!    (#154): `to_cardinal(-1)` == "منفي هڪ". The decimal word is still the
//!    English "point" (see [`POINTWORD`]) — the Sindhi term is not confirmed.
//! 6. Orthographic inconsistency in the tables, kept byte-exact: `ones[1]`
//!    ("هڪ") spells its kaf with U+06AA ARABIC LETTER SWASH KAF, while `million`
//!    ("لک") uses U+06A9 ARABIC LETTER KEHEH. These are distinct codepoints that
//!    render near-identically, so the tables below use explicit `\u{...}`
//!    escapes rather than pasted glyphs — a copy-paste round trip through an
//!    editor that normalises Arabic presentation forms would silently corrupt
//!    them, and the corpus compares bytes.
//!
//! # Currency
//!
//! `to_currency` is overridden by the Python class and shares essentially
//! nothing with `Num2Word_Base.to_currency`: it never calls
//! `parse_currency_parts`, `pluralize`, `_cents_verbose` or `_cents_terse`, and
//! never consults `CURRENCY_PRECISION` or `CURRENCY_ADJECTIVES`. It splits
//! `str(val)` on `"."` and works on the two halves textually. Consequences,
//! each verified against the live interpreter and the frozen corpus:
//!
//! 1. **An unknown currency code does not raise.** `to_currency` looks the code
//!    up with `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`,
//!    so anything outside {PKR, USD, EUR} silently renders in Pakistani rupees —
//!    the first entry by insertion order. The corpus tests seven such codes
//!    (GBP, JPY, KWD, BHD, INR, CNY, CHF) and every row comes back in rupees and
//!    paisas. `to_cheque`, which SD does *not* override, keeps the base's plain
//!    `CURRENCY_FORMS[currency]` subscript and so *does* raise for those same
//!    seven codes. The two entry points disagree, deliberately.
//! 2. **`CURRENCY_PRECISION` is ignored.** The subunit is always hundredths,
//!    hardcoded by the `parts[1][:2]` slice. There is no 3-decimal (KWD/BHD,
//!    divisor 1000) or 0-decimal (JPY, divisor 1) handling, and the base's
//!    zero-decimal rounding branch is unreachable. Since SD inherits
//!    `CURRENCY_PRECISION = {}` from `Num2Word_Base` those codes would resolve
//!    to 100 regardless, so this is invisible in the corpus — `currency:JPY` and
//!    `currency:KWD` render identically to `currency:PKR`, cents and all.
//! 3. **No rounding, only truncation.** `12.999` bills 99 cents and `2.675`
//!    bills 67 — the base path's `ROUND_HALF_UP` would give 100 and 68.
//! 4. **`adjective` is accepted and ignored**, so `CURRENCY_ADJECTIVES` (empty
//!    anyway) is never read.
//! 5. **`cents=False` suppresses the cents segment entirely** rather than
//!    switching to terse digits as the base class does: the guard is
//!    `if cents and right:`, and a false `cents` drops the whole clause.
//!    `to_currency(12.34, cents=False)` == `"ڏهه ٻه euros"`.
//!
//! `to_cheque` is inherited unmodified and needs no override here: the base
//! implementation's hooks (`currency_forms`, `currency_precision` at its default
//! 100, `money_verbose` delegating to `to_cardinal`) already resolve to SD's
//! behaviour.
//!
//! # Known divergences
//!
//! Two, both outside the corpus, both stemming from information the
//! `CurrencyValue` boundary cannot carry:
//!
//! 1. **An explicit `separator=","` renders as `" "`.** The dispatcher
//!    substitutes `Num2Word_Base`'s `","` when the caller supplies no separator,
//!    so `","` has to be read as "unset" and replaced with SD's own `" "`
//!    default — which makes a deliberate `","` unrepresentable. Every other
//!    separator is honoured verbatim. The corpus evidence for resolving it this
//!    way, and the reason it cannot be fixed from inside this file, is on
//!    [`SEPARATOR_UNSET`].
//! 2. **Scientific-notation input raises `ValueError` in Python, not here.**
//!    `str(float)` switches to exponent form at `1e16` and below `1e-4`, and
//!    `int("1e+21")` then throws: `to_currency(1e21)` and `to_currency(1e-5)`
//!    are both `ValueError`. The shim hands the core `str(value)`, but
//!    `CurrencyValue::parse` turns it into a `BigDecimal`, which accepts
//!    exponent notation and discards the fact that it was used —
//!    `"1e-05"` and `"0.00001"` become indistinguishable. Reproducing the raise
//!    would need the original string. This path renders the value instead (via
//!    bug 3's digit fallback for the large case). No corpus row reaches it.
//!
//! # Error variants
//!
//! `to_cheque` raises `N2WError::NotImplemented` for a code absent from
//! `CURRENCY_FORMS`, via the inherited `currency::default_to_cheque` and the
//! [`Lang::lang_name`] override below — Python's
//! `NotImplementedError: Currency code "GBP" not implemented for "Num2Word_SD"`,
//! reached through the `KeyError` that `to_cheque`'s bare subscript throws.
//!
//! Otherwise none. The four integer modes and `to_currency` have no reachable
//! crash site and no deliberate `raise`: every list index is derived from a digit
//! of a value already bounded by the enclosing branch, so `ones[i]`/`tens[i]` can
//! only be hit with `i` in `0..=9`. With no `MAXVAL` there is no `OverflowError`
//! either (bug 3 swallows the large-input case). `pluralize` is left at the
//! trait default that raises, matching `Num2Word_Base.pluralize` — SD reaches it
//! from neither `to_currency` (which inlines its own two-form selection) nor
//! `to_cheque` (which takes `cr1[-1]` directly).

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `_int_to_word`'s zero case. Python evaluates
/// `self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is `""` (falsy),
/// so Python returns the English literal; this port says ٻڙي. See bug 1.
const ZERO: &str = "\u{067B}\u{0699}\u{064A}"; // ٻڙي

/// `self.negword`. English "minus " in Python; منفي here. See bug 5.
const NEGWORD: &str = "\u{0645}\u{0646}\u{0641}\u{064A} "; // منفي

/// `self.ones`. Index 0 is the empty placeholder that makes bug 1 possible;
/// it is never emitted, because every call site either guards on a nonzero
/// remainder or is the `_int_to_word` zero branch that returns [`ZERO`].
const ONES: [&str; 10] = [
    "",                                     // (empty placeholder — see bug 1)
    "\u{0647}\u{06AA}",                     // هڪ    (hik)    — note U+06AA, cf. bug 6
    "\u{067B}\u{0647}",                     // ٻه    (ba)
    "\u{067D}\u{064A}",                     // ٽي    (ṭe)
    "\u{0686}\u{0627}\u{0631}",             // چار   (chār)
    "\u{067E}\u{0646}\u{062C}",             // پنج   (panj)
    "\u{0687}\u{0647}\u{0647}",             // ڇهه   (chhahe)
    "\u{0633}\u{062A}",                     // ست    (sat)
    "\u{0627}\u{067A}",                     // اٺ    (aṭh)
    "\u{0646}\u{0648}",                     // نو    (nav)
];

/// `self.tens`. Index 0 is an unused empty placeholder: the `< 100` branch is
/// only entered when `number >= 10`, so `tens[number // 10]` has index >= 1.
const TENS: [&str; 10] = [
    "",                                                     // (unreachable placeholder)
    "\u{068F}\u{0647}\u{0647}",                             // ڏهه     (ḍahe)
    "\u{0648}\u{064A}\u{0647}\u{0647}",                     // ويهه    (vīhe)
    "\u{067D}\u{064A}\u{0647}\u{0647}",                     // ٽيهه    (ṭīhe)
    "\u{0686}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}",     // چاليهه  (chālīhe)
    "\u{067E}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}",     // پنجاهه  (panjāhe)
    "\u{0633}\u{067A}",                                     // سٺ      (saṭh)
    "\u{0633}\u{062A}\u{0631}",                             // ستر     (satar)
    "\u{0627}\u{0633}\u{064A}",                             // اسي     (asī)
    "\u{0646}\u{0648}\u{064A}",                             // نوي     (navī)
];

/// `self.hundred` — سو (so).
const HUNDRED: &str = "\u{0633}\u{0648}";
/// `self.thousand` — هزار (hazār).
const THOUSAND: &str = "\u{0647}\u{0632}\u{0627}\u{0631}";
/// `self.million` — لک (lakh). Applied at 10^6 despite meaning 10^5; see bug 4.
/// Note U+06A9 KEHEH here vs U+06AA SWASH KAF in `ONES[1]` (bug 6).
const MILLION: &str = "\u{0644}\u{06A9}";

/// The suffix `to_ordinal` appends: `"-و"` (hyphen + U+0648 ARABIC LETTER WAW).
const ORDINAL_SUFFIX: &str = "-\u{0648}";

/// `self.pointword` — the word between the integer and fractional parts on the
/// float path. Still the English "point": the Sindhi decimal word needs a
/// native-speaker check (gladiaio/num2words2#154), so it is left as is.
const POINTWORD: &str = "point";

/// The 10^9 ceiling past which `_int_to_word` gives up and returns digits.
const FALLBACK_THRESHOLD: u64 = 1_000_000_000;

// ---- currency ----------------------------------------------------------
//
// `Num2Word_SD.CURRENCY_FORMS` is a class attribute the class declares itself,
// so — unlike the EUR-family languages — it is *not* the dict `Num2Word_EN`
// mutates at import time. Verified against the live interpreter: SD's table
// holds exactly PKR/USD/EUR and nothing English ever added. `CURRENCY_ADJECTIVES`
// and `CURRENCY_PRECISION` are both inherited unset (`{}`) from `Num2Word_Base`,
// so precision is 100 for every code (the trait default) and the adjective hook
// is never consulted.

/// PKR unit forms — روپي (rupī, sg) / روپيا (rupiyā, pl). Escaped rather than
/// pasted, for the same reason as the numeral tables: see bug 6.
const RUPEE_SG: &str = "\u{0631}\u{0648}\u{067E}\u{064A}";
const RUPEE_PL: &str = "\u{0631}\u{0648}\u{067E}\u{064A}\u{0627}";
/// PKR subunit forms — پئسو (paiso, sg) / پئسا (paisā, pl).
const PAISA_SG: &str = "\u{067E}\u{0626}\u{0633}\u{0648}";
const PAISA_PL: &str = "\u{067E}\u{0626}\u{0633}\u{0627}";

/// `Num2Word_SD.to_currency`'s own default: `separator=" "`, a bare space — not
/// the `","` that `Num2Word_Base.to_currency` defaults to.
const SD_SEPARATOR: &str = " ";

/// The separator the binding hands us when the Python caller supplied none.
///
/// `Num2Word_SD.to_currency` declares `separator=" "`, but the `Lang` trait takes
/// `separator` as a plain required argument, so "caller omitted it" has to be
/// encoded in the value itself. Both callers substitute **`Num2Word_Base`'s**
/// default rather than SD's, because neither can see a per-language signature:
/// `num2words2/__init__.py` forwards `kwargs.get("separator", ",")`, and
/// `bench/diff_test.py` hardcodes `","`. By the time the value crosses the
/// boundary, "omitted" and "explicitly `,`" are indistinguishable.
///
/// The corpus proves `","` must be read as "omitted" here. Rows are generated by
/// `num2words(v, lang=l, to="currency", currency=c)` with no `separator=`, yet
/// `de` — whose Python default is `separator=" und"` — renders
/// `"zwölf Euro und vierunddreißig Cent"` with no comma, while `en`, which keeps
/// the base `","`, renders `"twelve euros, thirty-four cents"`. Both are diffed
/// through a core call passing `","`. So a language that overrides `to_currency`
/// with its own default must restore that default when it sees the sentinel.
///
/// The one unreproducible case is a caller explicitly passing `separator=","`,
/// which gets `" "` where Python would give `","`. That is unresolvable without
/// an `Option<&str>` on the trait; see the module-level "Known divergences".
const SEPARATOR_UNSET: &str = ",";

/// Python's `int(parts[1][:2].ljust(2, "0"))` divisor: SD slices exactly two
/// fractional digits out of `str(val)`, hardcoding hundredths regardless of
/// `CURRENCY_PRECISION`. See divergence 2.
const CENTS_DIVISOR: i64 = 100;

pub struct LangSd {
    /// `self.CURRENCY_FORMS`. Built once in [`LangSd::new`] and cached by the
    /// generated registry's `OnceLock`, never per call.
    forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the entry `to_currency` falls
    /// back to for an unknown code. Python evaluates that on the dict's
    /// *insertion* order, which a `HashMap` does not preserve, so the first
    /// entry (PKR) is resolved once here and stored outright rather than
    /// recovered from `forms` at lookup time.
    fallback_forms: CurrencyForms,
}

impl LangSd {
    pub fn new() -> Self {
        // Insertion order in lang_SD.py is PKR, USD, EUR — PKR first, which is
        // what `list(...)[0]` picks up as the unknown-code fallback.
        let pkr = CurrencyForms::new(&[RUPEE_SG, RUPEE_PL], &[PAISA_SG, PAISA_PL]);
        let mut forms = HashMap::new();
        forms.insert("PKR", pkr.clone());
        forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]),
        );
        LangSd {
            forms,
            fallback_forms: pkr,
        }
    }
}

impl Default for LangSd {
    fn default() -> Self {
        Self::new()
    }
}

/// Python's `_int_to_word`.
///
/// The `number < 0` arm is faithfully reproduced even though it is **dead code
/// on every in-scope path**: `to_cardinal` strips the sign from the string form
/// before calling in, and the recursive call sites all pass a positive
/// remainder. It would only matter to a caller reaching `_int_to_word` directly.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO.to_string();
    }
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }
    // Every branch below 10^9 is bounded, so a u64 is provably wide enough
    // once we know the value is under the threshold. Anything else — including
    // BigInts too large for u64 at all — takes the digit fallback (bug 3).
    match number.to_u64() {
        Some(n) if n < FALLBACK_THRESHOLD => bounded_to_word(n),
        _ => number.to_string(),
    }
}

/// The `1 <= number < 10^9` portion of `_int_to_word`.
///
/// Invariant: never called with 0. Python's zero case is handled one level up
/// in [`int_to_word`], and each recursion here is guarded by Python's
/// `if remainder:` / a nonzero quotient. Were it called with 0 it would return
/// `ONES[0]` (`""`) rather than [`ZERO`] — the guard is what keeps bug 1's
/// zero word confined to the top-level entry point.
fn bounded_to_word(number: u64) -> String {
    if number < 10 {
        return ONES[number as usize].to_string();
    }

    if number < 100 {
        let tens_val = (number / 10) as usize;
        let ones_val = (number % 10) as usize;
        // No teens special case: 11 becomes "ten one". See bug 2.
        if ones_val == 0 {
            return TENS[tens_val].to_string();
        }
        return format!("{} {}", TENS[tens_val], ONES[ones_val]);
    }

    if number < 1_000 {
        // Note: hundreds use `ones[h]` directly, not a recursive call.
        let hundreds_val = (number / 100) as usize;
        let remainder = number % 100;
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&bounded_to_word(remainder));
        }
        return result;
    }

    if number < 1_000_000 {
        let thousands_val = number / 1_000;
        let remainder = number % 1_000;
        let mut result = format!("{} {}", bounded_to_word(thousands_val), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&bounded_to_word(remainder));
        }
        return result;
    }

    // number < 10^9, guaranteed by the caller.
    let millions_val = number / 1_000_000;
    let remainder = number % 1_000_000;
    let mut result = format!("{} {}", bounded_to_word(millions_val), MILLION);
    if remainder != 0 {
        result.push(' ');
        result.push_str(&bounded_to_word(remainder));
    }
    result
}

impl Lang for LangSd {

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

    /// `to_ordinal(float/Decimal)` — Python's `to_ordinal` is
    /// `to_cardinal(number) + "-و"` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "پنج point ٻڙي-و".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}-و", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is the
    /// dispatcher's exact `str(value)` (float repr / `Decimal.__str__`), so
    /// trailing zeros and `1E+2`-style exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — the base `Decimal(value)` parse, except the
    /// Infinity sentinel becomes the ValueError this language's own
    /// `to_cardinal` raises (`int("Infinity")` after the `"." in n` test
    /// fails); the shared dispatcher would otherwise report Base's
    /// OverflowError. NaN keeps the base sentinel: the dispatcher's
    /// ValueError for it already matches `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
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

    /// `self.pointword`. Consulted on the `"." in n` branch of Python's
    /// `to_cardinal` — the float/Decimal path now served by
    /// [`LangSd::to_cardinal_float`] below. Plain ASCII "point".
    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number).strip()`, peels a leading `"-"` into
    /// `ret = self.negword`, then re-parses with `int(n)`. For integer input
    /// that round trip is exactly "take the absolute value and remember the
    /// sign", which is what this does. The `"." in n` fractional branch is
    /// unreachable here (scope is integers) and is omitted.
    ///
    /// The trailing `.strip()` is preserved, though it is a no-op in practice:
    /// `_int_to_word` never returns a value with outer whitespace, and it never
    /// returns `""` (zero yields [`ZERO`]), so there is no case where the
    /// negword prefix is left dangling with a space to trim.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (prefix, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", prefix, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Python's `to_ordinal`: the cardinal with `"-و"` glued on. There is no
    /// per-value inflection — every ordinal, including "ٻڙي-و" and the
    /// digits-fallback "1000000000-و", takes the same suffix.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`. Overrides the base
    /// default (which omits the dot). Negatives keep their sign: `-1` -> `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year`: delegates straight to `to_cardinal`, discarding the
    /// `longval` flag. No era suffix, no two-digit pairing — 1999 is read as
    /// the plain cardinal "one thousand nine hundred ninety nine", and negative
    /// years just get the negword prefix rather than a BC marker.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Python's `Num2Word_SD.to_cardinal` for **non-integer** input — the
    /// `"." in n` branch. The Rust trait splits the integer and float entry
    /// points that Python's single `to_cardinal` serves off one method, so this
    /// reproduces only the fractional branch:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword          # "minus "
    /// else:
    ///     ret = ""
    /// left, right = n.split(".", 1)
    /// ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    /// for digit in right:
    ///     ret += self._int_to_word(int(digit)) + " "
    /// return ret.strip()
    /// ```
    ///
    /// # Recovering `left`/`right` without `str(number)`
    ///
    /// SD reads the digits straight out of `str(number)`: `left` is the integer
    /// part, `right` the fractional characters, one Sindhi word per digit. The
    /// f64 crosses the FFI boundary as a raw double, so the original repr string
    /// is gone — but `floatpath::float2tuple` reconstructs exactly the same
    /// `(pre, post)` the corpus is built on, and its zero-padded `post` is byte
    /// for byte the `right` string SD iterates, load-bearing f64 artefacts and
    /// all: `2.675` -> `674.9999999999998` rescued to `675`, and the leading
    /// zeros of `0.01` -> `"01"`. `int(left)` is `pre.abs()` — SD peels the sign
    /// off the *string* first, so `left` never carries a minus, and the
    /// negword prefix is added separately (bug 5, prepended to
    /// RTL text). The same `float2tuple` serves the Decimal arm, so `cardinal_dec`
    /// rows come out right too: `1.10` keeps its trailing zero (`post` = 10 padded
    /// to `"10"`), and `98746251323029.99` overflows `int_to_word` into bug 3's
    /// bare-digit fallback for the integer part.
    ///
    /// # `precision=` is ignored, exactly as Python ignores it
    ///
    /// SD overrides `to_cardinal`, which takes no `precision` argument, so
    /// `precision=1` still leaves `2.675` at three fractional words. Verified in
    /// the live interpreter. `_precision_override` is therefore discarded; the
    /// natural repr-derived `value.precision()` always wins.
    ///
    /// # `ValueError` on inputs `int()` cannot parse
    ///
    /// SD reads `str(number)`; when that carries exponent notation the `int()`
    /// calls throw `ValueError`. `str(1e16) == "1e+16"` has no `"."`, so the
    /// *integer* branch runs `int("1e+16")`; `str(1.5e-5) == "1.5e-05"` keeps a
    /// `"."` but then `int("5e-05")` throws in the digit loop. Non-finite floats
    /// (`inf`/`nan`, `str` `"inf"`/`"nan"`) throw the same way. The message is
    /// never observed — only the exception *type* — so a plain `N2WError::Value`
    /// suffices. The predicates below reproduce CPython's `repr`/`Decimal.__str__`
    /// exponent thresholds exactly (fuzzed 500k floats / 200k Decimals, zero
    /// divergence): a float switches to `e`-notation iff `x != 0 and
    /// (|x| >= 1e16 or |x| < 1e-4)`; a Decimal iff its exponent is positive or
    /// its adjusted exponent `< -6`. No corpus row reaches either — SD's corpus
    /// floats sit in `[0.01, ~1e14]` — so this only tightens fidelity for inputs
    /// the harness never generates.
    ///
    /// # The negative-zero hole (Decimal only)
    ///
    /// `str(Decimal("-0.0")) == "-0.0"` keeps the sign, so Python answers
    /// "منفي ٻڙي point ٻڙي"; a `BigDecimal` has no signed zero (its `BigInt`
    /// mantissa normalises `-0` to `0`), and the discriminating string is not
    /// carried across the `FloatValue::Decimal` boundary, so this arm drops the
    /// negword. Out of this file's remit — same boundary hole `lang_pa` flags.
    /// The float arm has no such hole: `FloatValue::Float` keeps the raw f64, so
    /// `is_sign_negative()` recovers `-0.0`'s minus faithfully.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Sign is str(number).startswith("-"); also reject the str() forms
        // int() cannot parse (exponent notation / non-finite), which SD raises
        // ValueError on. Digits still come from float2tuple, proven byte-for-byte
        // equal to str(number)'s fractional characters across a 240k fuzz.
        let negative = match value {
            FloatValue::Float { value: f, .. } => {
                if !f.is_finite() || (*f != 0.0 && (f.abs() >= 1e16 || f.abs() < 1e-4)) {
                    return Err(N2WError::Value(
                        "invalid literal for int() with base 10".to_string(),
                    ));
                }
                // is_sign_negative, not `< 0.0`: str(-0.0) == "-0.0" keeps its
                // minus, and SD peels that into a leading negword.
                f.is_sign_negative()
            }
            FloatValue::Decimal { value: d, .. } => {
                // str(Decimal) uses E-notation when the exponent is positive or
                // the adjusted exponent < -6; int() then throws ValueError.
                let (int_val, scale) = d.as_bigint_and_exponent();
                let ndigits = int_val.abs().to_string().len() as i64;
                if scale < 0 || (ndigits - 1 - scale) < -6 {
                    return Err(N2WError::Value(
                        "invalid literal for int() with base 10".to_string(),
                    ));
                }
                // Negative-zero hole: BigDecimal cannot carry "-0.0"'s sign.
                d.is_negative()
            }
        };

        let precision = value.precision() as usize;
        let (pre, post) = float2tuple(value);

        // `ret = self.negword if n.startswith("-") else ""`, then
        // `self._int_to_word(int(left))` on the *unsigned* integer part.
        let mut ret = String::new();
        if negative {
            ret.push_str(NEGWORD);
        }
        ret.push_str(&int_to_word(&pre.abs()));

        if precision > 0 {
            // `+ " " + self.pointword + " "`
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');

            // `right`: the fractional characters of str(number) — `post`,
            // left-padded with zeros to `precision` digits (post < 10**precision).
            // Mirrors floatpath::default_to_cardinal_float's own padding.
            let post_str = post.to_string();
            let padding = "0".repeat(precision.saturating_sub(post_str.len()));
            for ch in format!("{}{}", padding, post_str).chars().take(precision) {
                // Each char is one ASCII digit; `self._int_to_word(int(digit))`
                // maps 0 -> [`ZERO`] (bug 1) and 1..=9 -> ONES[d].
                let d = ch.to_digit(10).unwrap_or(0);
                ret.push_str(&int_to_word(&BigInt::from(d)));
                ret.push(' ');
            }
        }

        // `return ret.strip()` — drops the trailing digit space. (It would also
        // trim a dangling negword space, but no reachable path leaves one:
        // int_to_word never returns "" — zero yields [`ZERO`].)
        Ok(ret.trim().to_string())
    }

    // ---- currency ------------------------------------------------------

    /// For the `Currency code "X" not implemented for "Num2Word_SD"` message
    /// that `to_cheque` raises. `to_currency` never raises it — see below.
    fn lang_name(&self) -> &str {
        "Num2Word_SD"
    }

    /// `self.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately returns `None` — not the PKR fallback — for an unknown code.
    /// The fallback is a quirk of SD's *own* `to_currency` (which uses
    /// `dict.get(code, default)`); the inherited `to_cheque` uses plain
    /// `self.CURRENCY_FORMS[currency]` subscripting and lets the `KeyError`
    /// become a `NotImplementedError`. Folding the fallback in here would make
    /// `to_cheque("GBP")` succeed with rupees where Python raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Python's `Num2Word_SD.to_currency` — an outright override of the base
    /// implementation that shares almost none of its structure. It parses
    /// `str(val)` textually instead of going through `parse_currency_parts`,
    /// which is the source of most of the divergences listed below.
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
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0])
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
    /// Note it calls `_int_to_word`, not `to_cardinal`. For the non-negative
    /// `left`/`right` this path can produce, the two agree.
    ///
    /// # Why the `Int`/`Decimal` split does not branch here
    ///
    /// `Num2Word_Base.to_currency` keys off `isinstance(val, int)` to decide
    /// whether to print cents. SD never does — it asks whether `str(val)` has a
    /// fractional component, then whether that component is truthy *as an int*
    /// (as do the other 62 languages that hand-roll this same `str(val).split(".")`
    /// override). For a float
    /// `1.0` that yields `parts[1] == "0"` -> `right == 0` -> falsy -> cents are
    /// skipped, exactly as for the int `1`. So Python itself collapses the
    /// distinction here, and the corpus agrees: `arg "1"` and `arg "1.0"` both
    /// give `"هڪ euro"`. The match below still keeps the variants apart, because
    /// they reach the same `(left, right)` by different arithmetic, not because
    /// the outputs differ.
    ///
    /// # `right` is a truncating two-digit slice
    ///
    /// `int(parts[1][:2].ljust(2, "0"))` takes the first two fractional
    /// characters and pads *right* with zeros — i.e. it reads hundredths and
    /// truncates the rest. `trunc(fraction * 100)` is the exact arithmetic
    /// equivalent for any plain-decimal `str(val)`:
    ///
    /// | `str(val)` | `parts[1]` | slice+pad | `trunc(frac*100)` |
    /// |---|---|---|---|
    /// | `12.34`  | `"34"`  | `"34"` -> 34 | 34 |
    /// | `0.5`    | `"5"`   | `"50"` -> 50 | 50 |
    /// | `1.0`    | `"0"`   | `"00"` -> 0  | 0  |
    /// | `12.345` | `"345"` | `"34"` -> 34 | 34 |
    /// | `12.005` | `"005"` | `"00"` -> 0  | 0  |
    /// | `12.999` | `"999"` | `"99"` -> 99 | 99 |
    ///
    /// All six verified against the live interpreter. Note the last three: there
    /// is **no rounding anywhere** — `12.999` bills 99 cents, and `2.675` bills
    /// 67, not the 68 that `ROUND_HALF_UP` would give in the base path.
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
        // Restore SD's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SD_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`
        let is_negative = val.is_negative();

        let (left, right) = match val {
            // `str(int)` has no ".", so `len(parts) > 1` is false and `right`
            // stays 0 — a pure int never shows cents.
            CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let d = if is_negative { d.abs() } else { d.clone() };
                // `int(parts[0])`: the integer part, truncated.
                let whole = d.with_scale(0);
                let fraction = &d - &whole;
                let cents_val = (fraction * BigDecimal::from(CENTS_DIVISOR)).with_scale(0);
                (
                    whole.as_bigint_and_exponent().0,
                    cents_val.as_bigint_and_exponent().0,
                )
            }
        };

        // `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        // — an unknown code silently renders as Pakistani rupees rather than
        // raising. The corpus leans on this hard: GBP/JPY/KWD/BHD/INR/CNY/CHF
        // are all absent from SD's table and every one of their rows comes back
        // in rupees and paisas.
        let forms = self.forms.get(currency).unwrap_or(&self.fallback_forms);

        let one = BigInt::one();
        // `cr1[1] if left != 1 else cr1[0]` — note 0 takes the plural, hence
        // the corpus's "ٻڙي euros".
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };
        let mut result = format!("{} {}", int_to_word(&left), unit);

        // `if cents and right:` — `right == 0` is falsy, so a float with zero
        // cents drops the segment entirely. Also note that `cents=False` does
        // *not* fall back to terse digits the way the base class does: it
        // suppresses the cents segment outright. `_cents_terse` is unreachable.
        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            // `result += separator + cents_str + " " + ...` — no space before
            // the separator; the space in SD_SEPARATOR is the only one.
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        // `result = self.negword + result` — the negword (prepended to
        // RTL text; see bug 5).
        if is_negative {
            result.insert_str(0, NEGWORD);
        }

        // `result.strip()`. A no-op on every reachable path: `_int_to_word`
        // never returns leading/trailing whitespace and never returns "".
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// precision == abs(Decimal(repr(f)).as_tuple().exponent), computed on the
    /// Python side by the binding. Hardcoded here to the values verified in the
    /// live interpreter.
    fn flt(value: f64, precision: u32) -> String {
        LangSd::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn dec(s: &str, precision: u32) -> String {
        LangSd::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    #[test]
    fn corpus_float_rows() {
        // Every `"lang":"sd","to":"cardinal"` row with a dot in `arg`.
        assert_eq!(flt(0.0, 1), "ٻڙي point ٻڙي");
        assert_eq!(flt(0.5, 1), "ٻڙي point پنج");
        assert_eq!(flt(1.0, 1), "هڪ point ٻڙي");
        assert_eq!(flt(1.5, 1), "هڪ point پنج");
        assert_eq!(flt(2.25, 2), "ٻه point ٻه پنج");
        assert_eq!(flt(3.14, 2), "ٽي point هڪ چار");
        assert_eq!(flt(0.01, 2), "ٻڙي point ٻڙي هڪ");
        assert_eq!(flt(0.1, 1), "ٻڙي point هڪ");
        assert_eq!(flt(0.99, 2), "ٻڙي point نو نو");
        assert_eq!(flt(1.01, 2), "هڪ point ٻڙي هڪ");
        assert_eq!(flt(12.34, 2), "ڏهه ٻه point ٽي چار");
        assert_eq!(flt(99.99, 2), "نوي نو point نو نو");
        assert_eq!(flt(100.5, 1), "هڪ سو point پنج");
        assert_eq!(flt(1234.56, 2), "هڪ هزار ٻه سو ٽيهه چار point پنج ڇهه");
        assert_eq!(flt(-0.5, 1), "منفي ٻڙي point پنج");
        assert_eq!(flt(-1.5, 1), "منفي هڪ point پنج");
        assert_eq!(flt(-12.34, 2), "منفي ڏهه ٻه point ٽي چار");
        assert_eq!(flt(1.005, 3), "هڪ point ٻڙي ٻڙي پنج");
        assert_eq!(flt(2.675, 3), "ٻه point ڇهه ست پنج"); // f64 artefact -> 675
        // extra live-interpreter checks
        assert_eq!(flt(2.0, 1), "ٻه point ٻڙي");
        assert_eq!(flt(1000000.5, 1), "هڪ لک point پنج");
    }

    #[test]
    fn corpus_decimal_rows() {
        // Every `"lang":"sd","to":"cardinal_dec"` row.
        assert_eq!(dec("0.01", 2), "ٻڙي point ٻڙي هڪ");
        assert_eq!(dec("1.10", 2), "هڪ point هڪ ٻڙي"); // trailing zero kept
        assert_eq!(dec("12.345", 3), "ڏهه ٻه point ٽي چار پنج");
        assert_eq!(dec("98746251323029.99", 2), "98746251323029 point نو نو"); // bug 3 fallback
        assert_eq!(dec("0.001", 3), "ٻڙي point ٻڙي ٻڙي هڪ");
    }

    #[test]
    fn precision_override_ignored() {
        // Python's Num2Word_SD.to_cardinal takes no precision arg — 2.675 keeps
        // its three fractional words no matter what precision= is passed.
        let sd = LangSd::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        assert_eq!(
            sd.to_cardinal_float(&v, Some(1)).unwrap(),
            "ٻه point ڇهه ست پنج"
        );
    }

    #[test]
    fn float_negative_zero_keeps_negword() {
        // str(-0.0) == "-0.0" -> Python prepends the negword; is_sign_negative
        // recovers it where `< 0.0` would not.
        assert_eq!(flt(-0.0, 1), "منفي ٻڙي point ٻڙي");
    }

    #[test]
    fn scientific_and_nonfinite_raise_value() {
        let sd = LangSd::new();
        // str(1e16) == "1e+16" -> int("1e+16") raises ValueError.
        for (v, p) in [(1e16, 17u32), (1e-5, 5), (1.5e-5, 6), (f64::INFINITY, 0)] {
            assert!(matches!(
                sd.to_cardinal_float(&FloatValue::Float { value: v, precision: p }, None),
                Err(N2WError::Value(_))
            ));
        }
        // str(Decimal("1E-7")) == "1E-7" (adjusted exp -7 < -6) -> ValueError.
        assert!(matches!(
            sd.to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str("0.0000001").unwrap(),
                    precision: 7,
                },
                None,
            ),
            Err(N2WError::Value(_))
        ));
    }
}
