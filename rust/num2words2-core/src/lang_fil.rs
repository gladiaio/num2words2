//! Port of `lang_FIL.py` (Filipino / Tagalog).
//!
//! Registry check: `__init__.py` maps `"fil" -> lang_FIL.Num2Word_FIL()`, which
//! is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_FIL` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr(...)` guard in `Num2Word_Base.__init__` never fires: `self.cards`
//! is never built and `self.MAXVAL` is never set. All four in-scope methods
//! (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`) are overridden
//! outright, so nothing inherited is reachable and `cards`/`maxval`/`merge`
//! stay at their trait defaults. Python has **no overflow check** — see bug
//! 1; this port raises `OverflowError` from 10^15 (`maxval`).
//!
//! `setup()` sets `exclude_title = ["at", "menos", "punto"]`, but `is_title`
//! is left at the `False` that `Num2Word_Base.__init__` assigns *before*
//! calling `setup()`, and FIL's `to_cardinal` never calls `title()` anyway.
//! Both are mirrored here for fidelity; neither affects output.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below looks wrong and is exactly
//! what Python emits — all of it is pinned by rows in `bench/corpus.jsonl`.
//!
//! 1. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's final
//!    `return str(number)` is a fallthrough, not a raise: FIL has no numword
//!    above `milyon`, so `to_cardinal(10**9)` was `"1000000000"` (and
//!    `to_ordinal(10**9)` `"ika-1000000000"`). This port continues the
//!    million branch's composition with bilyon (10^9) and trilyon (10^12) —
//!    en.wiktionary.org/wiki/bilyon, /wiki/trilyon (Tagalog, short scale) —
//!    so `to_cardinal(10**9) == "isa bilyon"`, and raises `OverflowError` at
//!    10^15 (`maxval`).
//! 2. **`" at "` is used as a flat conjunction at every level**, so hundreds
//!    and tens are both joined with it: `999` is
//!    `"siyam daan at siyamnapu at siyam"` (two "at"s). Idiomatic Filipino
//!    would contract to "siyam na raan at siyamnapu't siyam"; the port keeps
//!    the Python.
//! 3. **No ligature/linker particles.** `ones[h] + " " + hundred` yields
//!    `"isa daan"` for 100 and `"dalawa daan"` for 200, where Filipino wants
//!    "isang daan" / "dalawang daan". Likewise `"isa libo"` (1000) and
//!    `"isa milyon"` (10^6). Preserved verbatim.
//! 4. **Thousands/millions remainders are joined with a bare space, not
//!    "at"** — unlike the tens/hundreds arms. Hence `1001` is
//!    `"isa libo isa"` while `101` is `"isa daan at isa"`.
//! 5. **`to_ordinal_num` does no sign handling**: it is literally
//!    `"ika-" + str(number)`, so `to_ordinal_num(-1) == "ika--1"` (double
//!    hyphen). Corpus-confirmed.
//! 6. **`to_ordinal` special-cases only `1`**, so `to_ordinal(0)` is
//!    `"ika-sero"` and `to_ordinal(-1)` is `"ika-menos isa"` — the negative
//!    word ends up *inside* the ordinal prefix.
//! 7. `tens[1] == "sampu"` is dead code: the `number < 20` teens arm catches
//!    10..=19 before the `number < 100` arm can ever see `t == 1`. Kept in the
//!    table so the indices line up with Python.
//!
//! # Currency
//!
//! `Num2Word_FIL` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base.to_currency`, so the trait's
//! `default_to_currency` is bypassed entirely. The differences are all
//! load-bearing:
//!
//! * **An unknown currency code does not raise.** Python's
//!   `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!   falls back to the *first* entry — PHP — so `to_currency(1, "GBP")` is
//!   `"isa piso"`, not a NotImplementedError. The corpus pins this for GBP,
//!   JPY, KWD, BHD, INR, CNY and CHF (bug 8).
//! * **`CURRENCY_PRECISION` is never consulted.** The `[:2]`/`ljust(2, "0")`
//!   slice hardcodes 2 decimal places, so the 3-decimal (KWD/BHD) and
//!   0-decimal (JPY) paths that `base.py` implements do not exist here:
//!   `12.34 KWD` is "labindalawa piso tatlumpu at apat sentimo", the same
//!   two-decimal reading as EUR (bug 9).
//! * **`adjective` is accepted and ignored** — FIL has no
//!   `CURRENCY_ADJECTIVES`, and the parameter is never read (bug 10).
//! * **`pluralize` is not called.** `to_currency` indexes the form tuples
//!   directly (`cr1[1] if left != 1 else cr1[0]`). FIL's `pluralize` override
//!   exists but is unreachable — see its doc comment.
//! * **The cents digits are truncated, not rounded.** `parts[1][:2]` drops
//!   everything past the second decimal, so `1.239` yields 23 sentimo, where
//!   `base.py`'s ROUND_HALF_UP quantize would give 24 (bug 11). No corpus row
//!   reaches past two decimals.
//!
//! `to_cheque`, by contrast, is inherited from `Num2Word_Base` untouched, and
//! it reads `CURRENCY_FORMS` with a **subscript** rather than `.get`. So the
//! two entry points disagree about the same input: `cheque:GBP` raises
//! NotImplementedError while `currency:GBP` silently prints pesos. Both are
//! corpus-pinned. That is why `currency_forms()` below reports the honest
//! 3-entry table (which is what `to_cheque` needs) and the PHP fallback lives
//! inside `to_currency` (bug 12).
//!
//! # Errors
//!
//! No *integer* input raises, and neither does `to_currency`: FIL never
//! indexes past a table (every lookup is guarded by the range check
//! immediately above it), has no MAXVAL to overflow, and its currency lookup
//! falls back rather than failing. The one raising path is the inherited
//! `to_cheque` on a code outside `CURRENCY_FORMS`, and that
//! `NotImplemented` is constructed by `currency::default_to_cheque` — so this
//! module still never names an `N2WError` itself.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword` — note the **trailing space**, which Python relies on when
/// it does `self.negword + self.to_cardinal(...)` (no separator of its own).
const NEGWORD: &str = "menos ";
/// `self.pointword`. Only reachable on the float path, which is out of scope.
const POINTWORD: &str = "punto";

/// `self.ones`. Index 0 is `""` and unreachable via `_int_to_word` (the
/// `number == 0` guard returns "sero" first); it is only ever read as
/// `ones[0]` on the float/decimal path, where Python's `or "sero"` rescues it.
const ONES: [&str; 10] = [
    "", "isa", "dalawa", "tatlo", "apat", "lima", "anim", "pito", "walo", "siyam",
];

/// `self.teens`, keyed by `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "sampu",
    "labing-isa",
    "labindalawa",
    "labintatlo",
    "labing-apat",
    "labinlima",
    "labing-anim",
    "labimpito",
    "labingwalo",
    "labinsiyam",
];

/// `self.tens`, keyed by the tens digit. Index 0 is `""` (unreachable: the
/// `number < 20` arm fires first), index 1 is dead — see bug 7.
const TENS: [&str; 10] = [
    "",
    "sampu",
    "dalawampu",
    "tatlumpu",
    "apatnapu",
    "limampu",
    "animnapu",
    "pitumpu",
    "walumpu",
    "siyamnapu",
];

const HUNDRED: &str = "daan";
const THOUSAND: &str = "libo";
const MILLION: &str = "milyon";
/// 10^9 and 10^12, absent from Python (gladiaio/num2words2#147):
/// en.wiktionary.org/wiki/bilyon and /wiki/trilyon (Tagalog, short scale).
const BILLION: &str = "bilyon";
const TRILLION: &str = "trilyon";

/// The exclusive ceiling (#147): no attested Filipino word for 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

/// The key `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// Python dicts preserve insertion order and `"PHP"` is the first key in
/// `Num2Word_FIL`'s class-body literal, so that expression is always PHP's
/// entry. A `HashMap` has no order, so the identity is pinned here rather
/// than derived — reordering `build_currency_forms` must not change it.
const FALLBACK_CURRENCY: &str = "PHP";

/// `Num2Word_FIL.CURRENCY_FORMS`, verbatim.
///
/// FIL declares the dict in its own class body and no other class writes into
/// it, so — unlike the `Num2Word_EUR` table that `Num2Word_EN.__init__`
/// mutates in place at import time — the source literal *is* what runs.
/// Checked against the live interpreter, not just the file.
///
/// Every entry is a 2-tuple whose two forms are **identical** ("piso"/"piso").
/// Filipino does not inflect these nouns for number, so the singular/plural
/// branch in `to_currency` can never change the output. The pairs are kept at
/// arity 2 anyway: `to_currency` indexes `[1]` unconditionally when
/// `left != 1`, and collapsing them to one form would turn that into an
/// IndexError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const SENTIMO: [&str; 2] = ["sentimo", "sentimo"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    // Insertion order matches the Python literal for readability only; the
    // PHP fallback is keyed by name (see FALLBACK_CURRENCY), not by position.
    m.insert("PHP", CurrencyForms::new(&["piso", "piso"], &SENTIMO));
    m.insert("USD", CurrencyForms::new(&["dolyar", "dolyar"], &SENTIMO));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &SENTIMO));
    m
}

/// Narrow a table index to `usize`.
///
/// Every call site sits behind a `number < 10` / `< 100` / `< 1000` guard and
/// operates on a value `to_cardinal` has already made non-negative, so the
/// operand is provably in `0..=999` and the conversion cannot fail. This is
/// the only place a `BigInt` is narrowed; the value itself is never cast.
fn idx(n: &BigInt) -> usize {
    debug_assert!(!n.is_negative() && n < &BigInt::from(1000));
    n.to_usize()
        .expect("guarded to 0..=999 by the range check at the call site")
}

pub struct LangFil {
    /// `self.exclude_title`. Inert: `is_title` is false and FIL's `to_cardinal`
    /// never calls `title()`. Carried for fidelity with `setup()`.
    exclude_title: Vec<String>,
    /// `self.CURRENCY_FORMS`. Built once here, never per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangFil {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFil {
    pub fn new() -> Self {
        LangFil {
            exclude_title: vec!["at".into(), "menos".into(), "punto".into()],
            currency_forms: build_currency_forms(),
        }
    }

    /// `parts = str(val).split(".")` -> `(left, right)`, with `val` already
    /// non-negative.
    ///
    /// Python slices the *decimal string*:
    ///
    /// ```python
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Done here as arithmetic on the `BigDecimal` instead of re-deriving
    /// Python's `str()`. The two agree exactly for every plain decimal string,
    /// which is all the shim can produce for the values that reach here:
    ///
    /// * `left` is the integer part. `parts[0]` holds precisely the digits
    ///   before the point, and `int()` of them is the truncation of a
    ///   non-negative value — `with_scale(0)`.
    /// * `right` is `trunc(frac * 100)`. `[:2]` keeps the first two fractional
    ///   digits and `ljust(2, "0")` right-pads a lone digit, so `"5"` -> `"50"`
    ///   -> 50, exactly `trunc(0.5 * 100)`. `"239"` -> `"23"` -> 23, exactly
    ///   `trunc(0.239 * 100)` — Python truncates the third decimal away rather
    ///   than rounding it (bug 11), and `with_scale(0)` truncates to match.
    /// * Python's two emptiness guards are unreachable: `str()` of a
    ///   non-negative number never yields an empty `parts[0]` (`"0.5"`, never
    ///   `".5"`) and never a trailing bare `"."`. Both fall out of the
    ///   arithmetic as 0 regardless.
    ///
    /// The one input the arithmetic would *not* match is a value whose
    /// `str()` is in exponent form (`str(1e21) == "1e+21"`), where Python's
    /// `int("1e+21")` raises ValueError while `BigDecimal` parses it happily.
    /// See `concerns` — no corpus row reaches it.
    fn currency_parts(val: &CurrencyValue) -> (BigInt, BigInt) {
        match val {
            // `str(int)` has no ".", so `len(parts) == 1` and right is 0.
            // This is the only thing the Int/Decimal split changes for FIL:
            // unlike `base.to_currency`, nothing here tests `isinstance`.
            CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
            // `has_decimal` is deliberately ignored: Python reads only the
            // shape of `str(val)`, so `Decimal("5")` and `Decimal("5.00")`
            // both land on right == 0 and print no cents either way.
            CurrencyValue::Decimal { value, .. } => {
                let abs = value.abs();
                let left = abs.with_scale(0).as_bigint_and_exponent().0;
                let frac = &abs - BigDecimal::from(left.clone());
                let right = (frac * BigDecimal::from(100))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                (left, right)
            }
        }
    }

    /// Port of `Num2Word_FIL._int_to_word`.
    ///
    /// Only ever reached with a non-negative value: `to_cardinal` detaches the
    /// `"-"` from the *string* before parsing, so the recursive call always
    /// sees a bare magnitude. (Were a negative to reach Python here, `< 10`
    /// would be true and `self.ones[-5]` would silently negative-index to
    /// "lima" — unreachable from any in-scope entry point, so not modelled.)
    ///
    /// `div_mod_floor` rather than `%` keeps Python's divmod semantics; for
    /// the non-negative operands here the two agree, but the floor form is
    /// what `divmod` means.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_zero() {
            return Ok("sero".to_string());
        }
        if number < &BigInt::from(10) {
            return Ok(ONES[idx(number)].to_string());
        }
        if number < &BigInt::from(20) {
            return Ok(TEENS[idx(&(number - BigInt::from(10)))].to_string());
        }
        if number < &BigInt::from(100) {
            // t, o = divmod(number, 10)
            let (t, o) = number.div_mod_floor(&BigInt::from(10));
            let mut s = TENS[idx(&t)].to_string();
            if !o.is_zero() {
                s.push_str(" at ");
                s.push_str(ONES[idx(&o)]);
            }
            return Ok(s);
        }
        if number < &BigInt::from(1000) {
            // h, r = divmod(number, 100); base = ones[h] + " " + hundred
            let (h, r) = number.div_mod_floor(&BigInt::from(100));
            let mut s = format!("{} {}", ONES[idx(&h)], HUNDRED);
            if !r.is_zero() {
                // "at" here, but a bare space in the two arms below — bug 4.
                s.push_str(" at ");
                s.push_str(&self.int_to_word(&r)?);
            }
            return Ok(s);
        }
        if number < &BigInt::from(1_000_000) {
            // t, r = divmod(number, 1000)
            let (t, r) = number.div_mod_floor(&BigInt::from(1000));
            let mut s = format!("{} {}", self.int_to_word(&t)?, THOUSAND);
            if !r.is_zero() {
                s.push(' ');
                s.push_str(&self.int_to_word(&r)?);
            }
            return Ok(s);
        }
        if number < &BigInt::from(1_000_000_000) {
            // m, r = divmod(number, 1000000)
            let (m, r) = number.div_mod_floor(&BigInt::from(1_000_000));
            let mut s = format!("{} {}", self.int_to_word(&m)?, MILLION);
            if !r.is_zero() {
                s.push(' ');
                s.push_str(&self.int_to_word(&r)?);
            }
            return Ok(s);
        }
        // Python: `return str(number)` — bug 1. bilyon / trilyon instead,
        // composed like `milyon` (#147); the maxval check above keeps the
        // trilyon multiplier below 1000.
        let unit = if number < &BigInt::from(1_000_000_000_000u64) {
            (BigInt::from(1_000_000_000), BILLION)
        } else {
            (BigInt::from(1_000_000_000_000u64), TRILLION)
        };
        let (m, r) = number.div_mod_floor(&unit.0);
        let mut s = format!("{} {}", self.int_to_word(&m)?, unit.1);
        if !r.is_zero() {
            s.push(' ');
            s.push_str(&self.int_to_word(&r)?);
        }
        Ok(s)
    }
}

/// Python `repr()` of a **non-negative** f64 — the string FIL's `to_cardinal`
/// sees after the leading `"-"` is detached. Rust's `{}` produces the same
/// shortest-round-trip digits but never switches to exponent form and drops
/// the ".0" of whole values, so the two Python-isms are reapplied:
///
/// * exponent form for `|v| >= 1e16` or `0 < |v| < 1e-4`, sign always shown,
///   exponent zero-padded to two digits (`"1e+16"`, `"1.5e-05"`);
/// * a trailing `".0"` for whole values in positional form (`"5.0"`).
///
/// inf/nan come back as `"inf"`/`"nan"`, which the `int()` port rejects with
/// Python's ValueError.
fn python_float_repr_abs(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return "inf".to_string();
    }
    if f != 0.0 && (f >= 1e16 || f < 1e-4) {
        let s = format!("{:e}", f);
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("f64 exponent is a small integer");
        return format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }
    let s = format!("{}", f);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// Python's `int()` ValueError, quoting the offending literal.
fn int_value_error(lit: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        lit
    ))
}

impl Lang for LangFil {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "PHP"
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
        "punto"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_FIL.to_cardinal`, integer path only.
    ///
    /// Python works on `str(number).strip()` and branches on a leading `"-"`,
    /// recursing on `n[1:]` — equivalent to `int_to_word(abs(value))` for
    /// integer input, since `str(int)` is always a bare optional sign plus
    /// digits. The `"." in n` branch (pointword + per-digit decimals) is
    /// unreachable for integers and out of scope.
    ///
    /// The trailing `.strip()` is faithful but a no-op: `int_to_word` never
    /// emits surrounding whitespace, and `NEGWORD`'s trailing space is
    /// consumed by the concatenation.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        self.int_to_word(value)
    }

    /// Port of `Num2Word_FIL.to_ordinal`.
    ///
    /// ```python
    /// if number == 1: return "una"
    /// return "ika-" + self.to_cardinal(number)
    /// ```
    /// No sign or zero guard — see bug 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("una".to_string());
        }
        Ok(format!("ika-{}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_FIL.to_ordinal_num`: `"ika-" + str(number)`.
    /// Negatives yield a double hyphen — see bug 5.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ika-{}", value))
    }

    /// Port of `Num2Word_FIL.to_year`: `to_year(val, longval=True)` ignores
    /// `longval` entirely and delegates straight to `to_cardinal`, so years
    /// read as plain cardinals ("isa libo siyam daan at siyamnapu at siyam"
    /// for 1999, not an English-style "nineteen ninety-nine" pairing).
    /// Negative years get the cardinal's "menos" rather than a BC/AD suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_FIL.to_cardinal`'s **float/Decimal branch**.
    ///
    /// FIL overrides `to_cardinal` and handles non-integers inline, so a float
    /// or Decimal never reaches `Num2Word_Base.to_cardinal_float`. Python:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "sero")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// **Why FIL cannot inherit `default_to_cardinal_float`.** It reads
    /// `str(number)`'s own fractional digits and never consults
    /// `self.precision`, so the `precision=` kwarg is silently ignored:
    /// `num2words(2.675, lang='fil', precision=1)` is still the full
    /// "dalawa punto anim pito lima", where the base path truncates to one
    /// digit. `precision_override` is therefore dropped here — the one real
    /// behavioural difference from the default, invisible on the corpus (which
    /// carries no `precision=`) but load-bearing.
    ///
    /// Digit reconstruction still goes through `float2tuple`, not a re-parse of
    /// a decimal string: the f64 artefacts are load-bearing (`2.675` yields
    /// `674.9999999999998`, rescued to `675` by the `< 0.01` heuristic), and
    /// `str(number)`'s fractional part is exactly `post` left-padded to the
    /// value's own precision. `pre.abs()` is `int(left)`; the sign is carried
    /// by the `NEGWORD` prefix, mirroring Python's `str`-prefix recursion, so
    /// e.g. `-0.5` (pre 0) becomes "menos sero punto lima".
    ///
    /// Each fractional digit uses `ones[d] or "sero"` verbatim — `ones[0]` is
    /// the empty string, falsy in Python, hence "sero"; 1..=9 index the table.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        // `precision=` is dropped: FIL never reads self.precision (see above).
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Use the value's own repr-derived precision — the count of fractional
        // digits str(number) shows, i.e. len(right) in the Python source.
        let precision = value.precision();
        let is_negative = value.is_negative();
        let (pre, post) = float2tuple(value);
        // int(left): the integer part of |value|; the sign is prefixed below.
        let left = pre.abs();

        // No "." in str(number) — an integer-valued Decimal like Decimal("5")
        // (precision 0) — takes Python's `return self._int_to_word(int(n))`
        // arm: the bare integer word, no "punto".
        if precision == 0 {
            let mut ret = self.int_to_word(&left)?;
            if is_negative {
                ret = format!("{}{}", NEGWORD, ret);
            }
            return Ok(ret.trim().to_string());
        }

        // right = str(number)'s fractional part = post left-padded to precision.
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        // ret = _int_to_word(int(left)) + " " + pointword
        let mut ret = format!("{} {}", self.int_to_word(&left)?, POINTWORD);
        // for digit in right: ret += " " + (ones[int(digit)] or "sero")
        for ch in post_str.chars().take(precision as usize) {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("non-digit {:?} in fractional part", ch))
            })?;
            ret.push(' ');
            ret.push_str(if d == 0 { "sero" } else { ONES[d as usize] });
        }

        if is_negative {
            // Python: (self.negword + result).strip(); NEGWORD keeps its space.
            ret = format!("{}{}", NEGWORD, ret);
        }
        // `.strip()` — inert: nothing above leaves surrounding whitespace.
        Ok(ret.trim().to_string())
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// FIL routes on the string, not the value: `"." in str(number)` decides
    /// between the pointword grammar and `int(n)`. Three corpus-pinned
    /// behaviours hang off the exact shape of `str()`:
    ///
    /// * **Whole floats keep their ".0"** — `str(5.0)` is `"5.0"`, so
    ///   `to_cardinal(5.0)` == `"lima punto sero"`. Only a point-free string
    ///   (an integer-valued `Decimal` like `Decimal("5")`) takes the int path.
    /// * **Exponent-form reprs raise ValueError**: `str(1e16)` == `"1e+16"`
    ///   (no "."), so `int("1e+16")` raises; `Decimal("1E+2")` likewise. A
    ///   *dotted* e-form (`"1.5e+16"`) enters the `"."` branch and dies in the
    ///   digit loop at `int('e')` (`'E'` for Decimals).
    /// * **-0.0 keeps its sign** — `str(-0.0)` is `"-0.0"`, hence
    ///   `"menos sero punto sero"`.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() minus its sign — Python detaches a leading
        // "-" before any other test; the sign is re-applied from the value
        // below (int path) or inside to_cardinal_float (dotted path).
        let n = match value {
            FloatValue::Float { value: f, .. } => python_float_repr_abs(f.abs()),
            FloatValue::Decimal { value: d, .. } => {
                let s = python_decimal_str(d);
                s.strip_prefix('-').map(str::to_string).unwrap_or(s)
            }
        };

        match n.split_once('.') {
            Some((left, right)) => {
                // `int(left)` runs before the digit loop; unreachable for any
                // dotted str() in practice, mirrored for order fidelity.
                if left.is_empty() || !left.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(int_value_error(left));
                }
                // `self.ones[int(digit)]` — the first non-digit raises; this
                // is where "1.5e+16" / Decimal("1.5E+20") die.
                if let Some(bad) = right.chars().find(|c| !c.is_ascii_digit()) {
                    return Err(int_value_error(&bad.to_string()));
                }
                self.to_cardinal_float(value, precision_override)
            }
            None => {
                if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(int_value_error(&n));
                }
                let magnitude: BigInt = n.parse().expect("all-ASCII-digit string parses");
                if value.is_negative() {
                    // (self.negword + self.to_cardinal(n[1:])).strip()
                    return Ok(format!("{}{}", NEGWORD, self.int_to_word(&magnitude)?)
                        .trim()
                        .to_string());
                }
                self.int_to_word(&magnitude)
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. `number == 1` is a *numeric* comparison,
    /// so `1.0` and `Decimal("1.00")` hit "una"; everything else is `"ika-"`
    /// glued to the string-routed cardinal — `to_ordinal(5.0)` ==
    /// `"ika-lima punto sero"`, `to_ordinal(-0.0)` == `"ika-menos sero punto
    /// sero"` — and an exponent-form repr propagates the ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int() == Some(BigInt::from(1u32)) {
            return Ok("una".to_string());
        }
        Ok(format!("ika-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ika-" + str(number)` — the raw
    /// repr, never an error: `to_ordinal_num(-0.0)` == `"ika--0.0"`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("ika-{}", repr_str))
    }

    /// `to_year(float/Decimal)`: bare `self.to_cardinal(val)`, string routing
    /// included — `to_year(5.0)` == `"lima punto sero"`, `to_year(1e16)`
    /// raises ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`) and this
    /// override does not change what parses. It exists because
    /// `Decimal("Infinity")`/`Decimal("NaN")` *do* parse in Python, and FIL's
    /// `to_cardinal` then dies at `int("Infinity")` with ValueError — where
    /// the bridge hard-wires the *base* integer-path errors (OverflowError /
    /// "cannot convert NaN"), which FIL never produces. The one input this
    /// would misserve is `to_ordinal_num("Infinity")` (Python:
    /// `"ika-Infinity"`), which no corpus exercises.
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
    //
    // FIL overrides only `to_currency` and `pluralize`. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` are inherited from
    // `Num2Word_Base` unchanged, and the trait defaults already mirror them —
    // `money_verbose` routing through `to_cardinal` is what gives the cheque
    // path its Filipino words. `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES`
    // are both Base's empty dict, so `.get(code, 100)` is a constant 100 and
    // `.get(code)` is always None: the trait defaults are already exact and
    // overriding them would only add noise.

    fn lang_name(&self) -> &str {
        "Num2Word_FIL"
    }

    /// `CURRENCY_FORMS[code]`, as the inherited `to_cheque` subscripts it.
    ///
    /// Reports None for anything outside the three declared codes, which is
    /// what turns `cheque:GBP` into Python's KeyError-to-NotImplementedError.
    /// `to_currency` deliberately does **not** route through this — it applies
    /// the PHP fallback instead (bug 12).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_FIL.pluralize`.
    ///
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Note `forms[-1]`, the *last* form — not `forms[1]` as `Num2Word_EUR`
    /// uses. The distinction is invisible for FIL's own table (every entry is
    /// a 2-tuple, where last == [1]) but it is what the class says, and the
    /// empty-`forms` guard means this override cannot raise at all.
    ///
    /// Unreachable in practice: FIL's `to_currency` indexes the form tuples
    /// itself and `Num2Word_Base.to_cheque` never calls `pluralize`. Ported
    /// because the override exists — leaving it at the trait default would
    /// raise NotImplemented on a path where Python returns a string.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Port of `Num2Word_FIL.to_currency`.
    ///
    /// ```python
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
    /// Note that `separator` is concatenated with no space of its own; FIL's
    /// signature defaults it to `" "` (not Base's `","`), which is what makes
    /// "labindalawa euro tatlumpu at apat sentimo" a plain space-joined
    /// phrase. The `adjective` flag is accepted and dropped on the floor.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // `adjective=False` is in the signature and never read (bug 10).
        _adjective: bool,
    ) -> Result<String> {
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is read *before* the abs below. Python's
        // `-0.0 < 0` is False, and BigDecimal("-0.0") is likewise not
        // negative, so the two agree on that edge.
        let is_negative = val.is_negative();

        let (left, right) = Self::currency_parts(val);

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code silently becomes PHP rather than raising (bug 8).
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CURRENCY))
            .expect("PHP is always present in CURRENCY_FORMS");
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // `cr1[1] if left != 1 else cr1[0]` — a direct subscript, not
        // pluralize(). Index 1 is in range for every entry in the table above,
        // so this cannot be the IndexError Python would raise on a 1-tuple.
        let mut result = format!(
            "{} {}",
            self.int_to_word(&left)?,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — right == 0 is falsy, so a whole float such
        // as 1.0 prints no cents segment at all ("isa euro"), and neither does
        // any true int.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right)?);
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        if is_negative {
            // NEGWORD carries its own trailing space; Python adds none.
            result = format!("{}{}", NEGWORD, result);
        }

        // `.strip()` — faithful but inert: neither branch above can leave
        // surrounding whitespace.
        Ok(result.trim().to_string())
    }
}
