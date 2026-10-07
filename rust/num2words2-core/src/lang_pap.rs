//! Port of `lang_PAP.py` (Papiamento).
//!
//! Registry check: `CONVERTER_CLASSES["pap"]` is `lang_PAP.Num2Word_PAP()`
//! (`__init__.py:396`), so this is the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_PAP` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `self.MAXVAL`. `to_cardinal` is overridden outright and drives its own
//! recursive `_int_to_word`. Consequently `cards`/`merge` stay at their trait
//! defaults here. Python has **no overflow check** at all — see the "billion
//! cliff" note below; this port raises `OverflowError` from 10^12 (`maxval`).
//!
//! Inherited from `Num2Word_Base` and left unchanged by PAP:
//!   * `is_title` stays `False`, so `title()` is the identity. `setup()` does
//!     assign `exclude_title = ["i", "punto", "menos"]`, but with `is_title`
//!     false it is dead data — reproduced below only for documentation.
//!
//! # Faithfully reproduced Python behaviour that looks wrong
//!
//! 1. **The billion cliff (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` handles 0..10^9-1 and then simply `return str(number)`,
//!    so `to_cardinal(10**9)` was the digit string `"1000000000"`. This port
//!    adds biyon (10^9; Aruban usage, matching the module's Aruban "miyon",
//!    e.g. https://www.noticiacla.com/news/39378), composed exactly like the
//!    million branch: `to_cardinal(10**9)` == `"un biyon"`. No 10^12 word is
//!    attested, so 10^12 raises `OverflowError` (`maxval`).
//! 2. **`to_ordinal` never calls `verify_ordinal`.** Base's guard against
//!    negative/float ordinals is bypassed, so `to_ordinal(-1)` does not raise —
//!    it returns `"di menos un"` (corpus-confirmed). Likewise `to_ordinal(0)`
//!    returns `"di sero"` rather than crashing.
//! 3. **`to_ordinal` only special-cases 1.** Every other value is
//!    `"di " + to_cardinal(n)` — hence `to_ordinal(10**9) == "di un biyon"`.
//! 4. **`to_ordinal_num` is `str(number) + "i"`** with no suffix logic and no
//!    sign handling: `to_ordinal_num(-1) == "-1i"`.
//! 5. **`tens[1]` is unreachable.** `setup()` sets `tens = ["", "dies", ...]`,
//!    but `_int_to_word` routes 10..=19 through `teens` before the `< 100`
//!    branch can index `tens[1]`. Kept in the table verbatim anyway.
//! 6. **Thousands drop "un" but millions keep it.** The `< 1000000` branch
//!    guards its prefix with `if t > 1` (so 1000 → `"mil"`, not `"un mil"`),
//!    while the `< 1000000000` branch has no such guard (so 10^6 →
//!    `"un miyon"`). The asymmetry is in the Python; both are corpus-confirmed.
//! 7. **`to_currency` never raises for an unknown code.** It reads
//!    `CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`, so
//!    GBP/JPY/KWD/BHD/INR/CNY/CHF quietly render as ANG florins rather than
//!    raising NotImplementedError. The inherited `to_cheque` does the opposite
//!    (`CURRENCY_FORMS[currency]` inside a `try`), so **the same code raises
//!    from `to_cheque` and succeeds from `to_currency`** — corpus-confirmed on
//!    both sides.
//! 8. **`to_currency` ignores `CURRENCY_PRECISION`, `pluralize` and
//!    `adjective`.** It is a wholesale override that never calls up to
//!    `Num2Word_Base.to_currency`, so none of Base's machinery runs: no
//!    `parse_currency_parts`, no ROUND_HALF_UP, no divisor. `adjective=True`
//!    is accepted and dropped on the floor (`CURRENCY_ADJECTIVES` is empty
//!    anyway), and its own `pluralize` is dead code on this path — the plural
//!    is picked inline with `cr1[1] if left != 1 else cr1[0]`. The visible
//!    consequence is that the 0-decimal (JPY) and 3-decimal (KWD/BHD) corpus
//!    rows behave like ordinary /100 currencies: `0.5 JPY` is
//!    `"sero florin sinkuenta sèn"`, where Base would have rounded to a whole
//!    unit and dropped the subunit entirely.
//! 9. **Cents are truncated, not rounded**, and read off the *decimal string*:
//!    `int(str(abs(val)).split(".")[1][:2].ljust(2, "0"))`. So `0.5` → `50`
//!    sèn (right-padded), `1.005` → `0` sèn (dropped, no carry), and `2.675` →
//!    `67` sèn, not the `68` that Base's ROUND_HALF_UP would give. Reproduced
//!    below with exact `BigDecimal` arithmetic rather than string slicing —
//!    see `to_currency` for why the two agree.
//!
//! # Error variants
//!
//! The four integer modes raise only `OverflowError`, for `abs(n) >= 10^12`
//! (#147; Python's billion cliff returned digits instead). `to_currency`
//! shares that ceiling and is otherwise total (unknown codes fall back rather
//! than raise — see the currency section). The inherited `to_cheque` also
//! raises `NotImplementedError` for a code outside PAP's own four.
//!
//! # Sign handling
//!
//! Python's `to_cardinal` works on `str(number)`: it tests `n.startswith("-")`
//! and recurses on `n[1:]`. For integer input `str()` emits a leading `"-"`
//! exactly for negatives and nothing else, so that string dance is equivalent
//! to `negword + to_cardinal(abs(n))`, which is what this port does. The
//! trailing `.strip()` on the Python result then only ever removes the space
//! that `negword` ("menos ") itself contributes — and it never does, since the
//! recursive result is non-empty and unpadded. Reproduced as `trim()` for
//! parity regardless.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::str::FromStr;

/// `setup(): self.ones`. Index 0 is `""` (the `< 10` branch reaches it only
/// for 0, which the `number == 0` guard has already claimed).
const ONES: [&str; 10] = [
    "", "un", "dos", "tres", "kuater", "sinku", "seis", "shete", "ocho", "nuebe",
];

/// `setup(): self.teens`, indexed `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "dies",
    "diesun",
    "diesdos",
    "diestres",
    "dieskuater",
    "diessinku",
    "diesseis",
    "diesshete",
    "diesocho",
    "diesnuebe",
];

/// `setup(): self.tens`. Index 0 is `""` and index 1 ("dies") is unreachable
/// — see bug note 5.
const TENS: [&str; 10] = [
    "", "dies", "binti", "trinta", "kuarenta", "sinkuenta", "sesenta", "setenta", "ochenta",
    "nobenta",
];

const ZERO_WORD: &str = "sero";
const HUNDRED: &str = "shen";
const THOUSAND: &str = "mil";
const MILLION: &str = "miyon";
/// 10^9, absent from Python (gladiaio/num2words2#147): Aruban "biyon", the
/// counterpart of [`MILLION`]'s Aruban spelling.
const BILLION: &str = "biyon";

/// The exclusive ceiling (#147): no attested Papiamentu word for 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}
const NEGWORD: &str = "menos ";
const POINTWORD: &str = "punto";

/// `Num2Word_PAP.CURRENCY_FORMS`, exactly the four entries the class body
/// declares — ANG, AWG, USD, EUR, in that insertion order.
///
/// PAP declares its **own** class-attribute dict, so the `lang_EUR.py` trap
/// does not apply: `Num2Word_EN.__init__`'s in-place mutation lands on
/// `Num2Word_EUR.CURRENCY_FORMS`, a different object. The live interpreter
/// agrees — `CONVERTER_CLASSES["pap"].CURRENCY_FORMS` has these four keys and
/// no EN-injected AUD/CAD/JPY/KWD/... See the corpus rows for JPY and KWD:
/// they render "florin", i.e. the fallback below, not EN's yen/dinar.
///
/// Both PAP form tuples are singular==plural, which is why every corpus row
/// reads the same regardless of count.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const SEN: [&str; 2] = ["sèn", "sèn"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("ANG", CurrencyForms::new(&FLORIN, &SEN));
    m.insert("AWG", CurrencyForms::new(&FLORIN, &SEN));
    m.insert("USD", CurrencyForms::new(&["dolar", "dolar"], &SEN));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &SEN));
    m
}

/// The unit forms of the **first** `CURRENCY_FORMS` entry — see
/// `currency_fallback` on the struct for why that matters.
const FLORIN: [&str; 2] = ["florin", "florin"];

pub struct LangPap {
    /// `setup(): self.exclude_title`. Dead data in Python (`is_title` is
    /// `False`), retained so the trait method reports what Python holds.
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS`. Built once here rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — PAP's `to_currency` passes
    /// this as the `dict.get` default, so an **unknown code silently renders
    /// as ANG** instead of raising (bug note 7).
    ///
    /// Python resolves it positionally through dict insertion order, which
    /// since 3.7 is guaranteed to be class-body order: ANG is written first,
    /// so the fallback is ANG's `(("florin","florin"), ("sèn","sèn"))`. A
    /// `HashMap` has no order to index, so the entry is materialised here
    /// instead of recovered from the table. Verified against the live
    /// interpreter and against the GBP/JPY/KWD/BHD/INR/CNY/CHF corpus rows,
    /// which all say "florin".
    currency_fallback: CurrencyForms,
}

impl Default for LangPap {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPap {
    pub fn new() -> Self {
        LangPap {
            exclude_title: vec!["i".to_string(), "punto".to_string(), "menos".to_string()],
            currency_forms: build_currency_forms(),
            currency_fallback: CurrencyForms::new(&FLORIN, &["sèn", "sèn"]),
        }
    }

    /// Port of `Num2Word_PAP._int_to_word`.
    ///
    /// Only ever called with a **non-negative** value: `to_cardinal` peels the
    /// sign off before calling. (Python would happily negative-index
    /// `self.ones[-3]` here and return "shete" for -3, but no in-scope path
    /// reaches that, so the small-index conversions below cannot fail.)
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_zero() {
            return Ok(ZERO_WORD.to_string());
        }
        if *number < BigInt::from(10) {
            // Safe: 0 < number < 10.
            return Ok(ONES[number.to_usize().unwrap()].to_string());
        }
        if *number < BigInt::from(20) {
            // Safe: 10 <= number < 20.
            return Ok(TEENS[(number - BigInt::from(10)).to_usize().unwrap()].to_string());
        }
        if *number < BigInt::from(100) {
            // Python: t, o = divmod(number, 10)
            let (t, o) = number.div_mod_floor(&BigInt::from(10));
            let t = t.to_usize().unwrap(); // 2..=9
            let o = o.to_usize().unwrap(); // 0..=9
            // Python: self.tens[t] + (" i " + self.ones[o] if o else "")
            let mut out = TENS[t].to_string();
            if o != 0 {
                out.push_str(" i ");
                out.push_str(ONES[o]);
            }
            return Ok(out);
        }
        if *number < BigInt::from(1000) {
            // Python: h, r = divmod(number, 100)
            let (h, r) = number.div_mod_floor(&BigInt::from(100));
            let hu = h.to_usize().unwrap(); // 1..=9
            // Python: base = (self.ones[h] + " " if h > 1 else "") + self.hundred
            let mut out = String::new();
            if hu > 1 {
                out.push_str(ONES[hu]);
                out.push(' ');
            }
            out.push_str(HUNDRED);
            // Python: base + (" i " + self._int_to_word(r) if r else "")
            if !r.is_zero() {
                out.push_str(" i ");
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }
        if *number < BigInt::from(1_000_000) {
            // Python: t, r = divmod(number, 1000)
            let (t, r) = number.div_mod_floor(&BigInt::from(1000));
            // Python: base = (self._int_to_word(t) + " " if t > 1 else "")
            //                + self.thousand
            // Note the `t > 1` guard: 1000 -> "mil", never "un mil".
            let mut out = String::new();
            if t > BigInt::from(1) {
                out.push_str(&self.int_to_word(&t)?);
                out.push(' ');
            }
            out.push_str(THOUSAND);
            // Python: base + (" " + self._int_to_word(r) if r else "")
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }
        if *number < BigInt::from(1_000_000_000) {
            // Python: m, r = divmod(number, 1000000)
            let (m, r) = number.div_mod_floor(&BigInt::from(1_000_000));
            // Python: base = self._int_to_word(m) + " " + self.million
            // No `m > 1` guard here, unlike the thousands branch above:
            // 10**6 -> "un miyon".
            let mut out = self.int_to_word(&m)?;
            out.push(' ');
            out.push_str(MILLION);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r)?);
            }
            return Ok(out);
        }
        // Python: return str(number) — the billion cliff (bug note 1). The
        // biyon branch instead, composed like the million one (#147); the
        // maxval check above keeps `b` below 1000.
        let (b, r) = number.div_mod_floor(&BigInt::from(1_000_000_000));
        let mut out = self.int_to_word(&b)?;
        out.push(' ');
        out.push_str(BILLION);
        if !r.is_zero() {
            out.push(' ');
            out.push_str(&self.int_to_word(&r)?);
        }
        Ok(out)
    }

    /// Port of `Num2Word_PAP.to_cardinal` operating on the Python `str(number)`
    /// — the exact form Python's own code splits on:
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
    /// The recursion is string-level in Python: `to_cardinal(n[1:])` hands a
    /// `str` straight back, whose `str(number)` is then a no-op. Reproduced as
    /// such rather than folded into an `abs()`. One level deep in practice — no
    /// repr yields a second leading "-". Because the sign is read off the
    /// *string*, negative zero keeps its "-" (`str(-0.0) == "-0.0"`), so `-0.0`
    /// renders "menos sero punto sero" — a case the base float path, which tests
    /// `value < 0.0`, would strip. That divergence is why PAP cannot inherit the
    /// default `to_cardinal_float`.
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        // n = str(number).strip(). Python strips its own whitespace set and Rust
        // trims Unicode's; no repr contains either.
        let n = n.trim();

        // if n.startswith("-"): return (self.negword + self.to_cardinal(n[1:])).strip()
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_str(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        // if "." in n: left, right = n.split(".", 1) — the *first* dot only, so
        // a stray second dot would land in `right` and reach int() as a char.
        // Unreachable from a real repr.
        let (left, right) = match n.split_once('.') {
            Some(halves) => halves,
            // return self._int_to_word(int(n))
            None => return self.int_to_word(&py_int(n)?),
        };

        // ret = self._int_to_word(int(left)) + " " + self.pointword
        //
        // `int(left)` is the whole integer part, so the scales and the 10^12
        // ceiling (bug note 1) apply to it.
        let mut ret = self.int_to_word(&py_int(left)?)?;
        ret.push(' ');
        ret.push_str(POINTWORD);

        // for digit in right: ret += " " + (self.ones[int(digit)] or "sero")
        //
        // Per *character* — no grouping, no rounding, no padding: the fraction
        // is exactly the digits the repr carried. `self.ones[0]` is `""`
        // (falsy), so the `or "sero"` is what turns a fraction digit 0 into a
        // word — a different mechanism from `_int_to_word`'s `number == 0`
        // guard, but the same output.
        for digit in right.chars() {
            let word = ONES[py_int_digit(digit)?];
            ret.push(' ');
            ret.push_str(if word.is_empty() { ZERO_WORD } else { word });
        }
        // Cosmetic: `_int_to_word` never returns "", so nothing is trimmed here.
        Ok(ret.trim().to_string())
    }
}

impl Lang for LangPap {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }


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
    /// `"promé"` when `number == 1`, else `"di " + to_cardinal(number)`,
    /// for *any* input (no `verify_ordinal`). `number == 1` is *numeric*
    /// equality, so `1.0` and `Decimal("1.00")` both take the special word.
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let is_one = match value {
            FloatValue::Float { value: f, .. } => *f == 1.0,
            FloatValue::Decimal { value: d, .. } => d == &bigdecimal::BigDecimal::from(1),
        };
        if is_one {
            return Ok("promé".to_string());
        }
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("di {}", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "i"` — no `== 1`
    /// special case here. `repr_str` is the dispatcher's exact `str(value)`
    /// (float repr / `Decimal.__str__`), so trailing zeros and `1E+2`-style
    /// exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}i", repr_str))
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
        "ANG"
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

    // `is_title` stays false (the base default), so `title()` is the identity
    // and `exclude_title` never actually gates anything.
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_PAP.to_cardinal`, integer path only.
    ///
    /// The Python operates on `str(number)`; for integers the `"." in n` branch
    /// (the float path) is unreachable, so only the sign test and the
    /// `_int_to_word(int(n))` tail remain.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if *value < BigInt::zero() {
            // Python: (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = self.to_cardinal(&-value)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        self.int_to_word(value)
    }

    /// Port of `Num2Word_PAP.to_ordinal`.
    ///
    /// No `verify_ordinal` call, so negatives and zero pass straight through
    /// (bug note 2).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if *value == BigInt::from(1) {
            return Ok("promé".to_string());
        }
        Ok(format!("di {}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_PAP.to_ordinal_num`: `str(number) + "i"`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}i", value))
    }

    /// Port of `Num2Word_PAP.to_year`: ignores `longval` and delegates.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_PAP.to_cardinal` for non-integer input.
    ///
    /// PAP overrides `to_cardinal` (not `to_cardinal_float`) and handles floats
    /// and Decimals inline by splitting `str(number)` — so the port reconstructs
    /// that exact string and hands it to [`Self::cardinal_from_str`]. The float
    /// and Decimal arms of `str()` are not interchangeable (issue #603): a float
    /// stringifies to shortest round-trip and goes exponential outside
    /// [1e-4, 1e16), while a Decimal stringifies exactly and goes exponential on
    /// its own written exponent. Reproducing both is what makes the exponent-form
    /// `ValueError` fire exactly where Python's does (`int("1e+16")`).
    ///
    /// PAP's `to_cardinal(self, number)` takes **no** `precision=` kwarg, and the
    /// live interpreter confirms the kwarg is dropped before this method is
    /// reached (`num2words(0.5, lang='pap', precision=3) == "sero punto sinku"`),
    /// so `precision_override` is ignored here.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        self.cardinal_from_str(&python_str(value))
    }

    // ---- currency -------------------------------------------------------
    //
    // PAP overrides `to_currency` and `pluralize`, and defines its own
    // `CURRENCY_FORMS`. Everything else on the currency path — `to_cheque`,
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` — is inherited from
    // `Num2Word_Base` unchanged (confirmed via `__qualname__` on the live
    // instance), and the trait defaults already mirror those, so they are not
    // overridden here.
    //
    // `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` on the live
    // instance, so `currency_precision` (default 100 == Base's
    // `.get(code, 100)` on an empty dict) and `currency_adjective` (default
    // `None`) are left alone too.
    //
    // `cardinal_from_decimal` stays at its raising default: PAP's own
    // `to_currency` has no fractional-cents branch to reach it, and Base's —
    // the only caller — is never entered.

    fn lang_name(&self) -> &str {
        "Num2Word_PAP"
    }

    /// `CURRENCY_FORMS[code]`, strictly — **no ANG fallback here**.
    ///
    /// This hook feeds the inherited `to_cheque`, whose Python is
    /// `try: self.CURRENCY_FORMS[currency] except KeyError: raise
    /// NotImplementedError`. The fallback is a quirk of `to_currency`'s
    /// `.get(...)` call alone (bug note 7), so it lives there and must not
    /// leak into this lookup — doing so would turn the seven expected
    /// `cheque:*` NotImplementedError rows into florin cheques.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_PAP.pluralize`.
    ///
    /// Note `forms[-1]`, not `forms[1]`: PAP takes the *last* form, so it is
    /// total for any arity and — unlike `Num2Word_EUR.pluralize` — cannot
    /// IndexError. The empty-tuple guard returns `""` rather than raising.
    ///
    /// Dead on PAP's own currency path (bug note 8); ported because the class
    /// defines it and it is reachable as a public method.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        // Python: if not forms: return ""
        if forms.is_empty() {
            return Ok(String::new());
        }
        // Python: return forms[0] if n == 1 else forms[-1]
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Port of `Num2Word_PAP.to_currency` — a wholesale override that shares
    /// nothing with `Num2Word_Base.to_currency` (bug note 8).
    ///
    /// # Why `has_decimal` is not consulted
    ///
    /// Base gates the cents segment on `has_decimal or right > 0`; PAP gates it
    /// on `if cents and right:` — the truthiness of the integer `right` alone.
    /// So the int/float split that is load-bearing everywhere else is invisible
    /// here: `1` and `1.0` both give `right == 0` and both print `"un euro"`,
    /// and `Decimal("5.00")` prints `"sinku euro"` where Base would say
    /// "five euro, zero cents". The `Int`/`Decimal` variants are still kept
    /// apart below, but only because `str(int)` has no `"."` to split — they
    /// converge on the same answer.
    ///
    /// # Why exact arithmetic reproduces Python's string slicing
    ///
    /// Python computes the parts by slicing `str(abs(val))`:
    /// `left = int(parts[0])` and
    /// `right = int(parts[1][:2].ljust(2, "0"))`. The shim hands us
    /// `BigDecimal::from_str(str(val))` — the *same* decimal string, parsed
    /// losslessly — so for any plain (non-exponent) decimal notation the slice
    /// is exactly `trunc(|v|)` and `trunc(frac(|v|) * 100)`: taking the first
    /// two fractional digits and right-padding with `"0"` *is* a truncating
    /// scale-2 shift. `0.5` → `"5"` → `"50"` → 50 == trunc(0.5 × 100); `1.005`
    /// → `"00"` → 0 == trunc(0.5); `2.675` → `"67"` → 67 == trunc(67.5).
    ///
    /// # Known divergence: exponent-form `str(value)`
    ///
    /// The equivalence above holds only while `str(value)` is plain decimal
    /// notation. When it is not — a float with `|v| >= 1e16` or `0 < |v| < 1e-4`
    /// (`str(1e16) == "1e+16"`), or a `Decimal` with adjusted exponent outside
    /// roughly -6..15 (`str(Decimal("0.000000001")) == "1E-9"`) — Python feeds
    /// that text to `int()` and raises **ValueError**, e.g.
    /// `invalid literal for int() with base 10: '1e+16'`. Nastier still,
    /// `Decimal("1.23E+4")` *does* contain a ".", so it splits into
    /// `("1", "23E+4")` and silently renders 12300 as "un euro binti i tres
    /// sèn". This port raises neither and renders 12300 correctly.
    ///
    /// This is **not fixable from inside this file**: the trigger is the
    /// *notation* of `str(val)`, which `CurrencyValue` does not carry. Both
    /// `1e-05` (a float) and `Decimal("0.00001")` arrive here as the identical
    /// `BigDecimal { digits: 1, scale: 5 }` with `has_decimal: true`, yet
    /// Python raises for the first and returns "sero euro" for the second.
    /// Reproducing it would require the boundary to pass the original string
    /// through. No corpus row exercises any of this, and the inherited
    /// `to_cheque` is immune (Base does `Decimal(str(val))`, which parses
    /// exponent form happily) — verified against the live interpreter.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait resolves an omitted kwarg through `default_separator()`.
        let separator = separator.unwrap_or(self.default_separator());
        // Python's signature accepts `adjective` and never reads it.
        let _ = adjective;

        // Python: is_negative = val < 0; val = abs(val)
        let is_negative = val.is_negative();

        // Python: parts = str(val).split(".")
        //         left  = int(parts[0]) if parts[0] else 0
        //         right = int(parts[1][:2].ljust(2, "0"))
        //                 if len(parts) > 1 and parts[1] else 0
        let (left, right) = match val {
            // str(int) never contains ".", so `parts` is a 1-element list and
            // `right` stays 0 — an int can never print sèn.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value, .. } => {
                let abs = value.abs();
                // with_scale(0) truncates toward zero; `abs` is non-negative,
                // so this is int(parts[0]).
                let int_part = abs.with_scale(0);
                let frac = &abs - &int_part;
                let cents_part = (frac * BigDecimal::from(100)).with_scale(0);
                (
                    int_part.as_bigint_and_exponent().0,
                    cents_part.as_bigint_and_exponent().0,
                )
            }
        };

        // Python: cr1, cr2 = self.CURRENCY_FORMS.get(
        //             currency, list(self.CURRENCY_FORMS.values())[0])
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // Python: result = self._int_to_word(left) + " "
        //                  + (cr1[1] if left != 1 else cr1[0])
        // Both indexes are direct tuple subscripts in Python, so a 1-element
        // form tuple would IndexError. Every PAP entry has two, making this
        // unreachable — mapped rather than unwrapped so the exception *type*
        // survives if the table ever changes.
        let unit = if left.is_one() { cr1.first() } else { cr1.get(1) }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
        let mut result = format!("{} {}", self.int_to_word(&left)?, unit);

        // Python: if cents and right:
        //             result += separator + self._int_to_word(right) + " "
        //                       + (cr2[1] if right != 1 else cr2[0])
        // Note `right` is tested for truthiness, so 0 sèn prints nothing at
        // all — and that `separator` is *not* followed by a space of its own.
        if cents && !right.is_zero() {
            let subunit = if right.is_one() { cr2.first() } else { cr2.get(1) }
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right)?);
            result.push(' ');
            result.push_str(subunit);
        }

        // Python: if is_negative: result = self.negword + result
        //         return result.strip()
        if is_negative {
            result.insert_str(0, NEGWORD);
        }
        Ok(result.trim().to_string())
    }
}

// ---- the float/Decimal path's `str(number)` reconstruction --------------
//
// PAP's `to_cardinal` works entirely on `str(number)`, so a faithful port has
// to hand `cardinal_from_str` the byte-for-byte string CPython would produce.
// These helpers are lifted from the sibling `lang_ceb.rs` port (Cebuano shares
// PAP's exact `_int_to_word`/`str`-splitting structure); each is documented and
// mass-verified against CPython there. Only the language constants differ.

/// CPython's `str(number)` for the value that reached PAP's `to_cardinal`.
fn python_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => py_str_f64(*value),
        FloatValue::Decimal { value, .. } => py_str_decimal(value),
    }
}

/// Python's `int(s)` for a whole token. CPython's message is
/// `invalid literal for int() with base 10: '<s>'`, quoted exactly — this is
/// what surfaces for exponent-form strings that carry no ".": `int("1e+16")`.
///
/// The accepted grammars differ only in ways that cannot bite here: Python's
/// `int` also takes surrounding whitespace, a `+` sign and `_` separators, none
/// of which a `str(float)`/`str(Decimal)` can emit.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `int(ch)` for a *single* character, as the fraction loop calls it.
/// The message quotes the one offending character, matching CPython — which is
/// why a Decimal like `1.23E+4` reports `'E'` rather than `'23E+4'`.
///
/// One unreachable divergence: CPython's `int()` accepts any Unicode `Nd`
/// codepoint, while `char::to_digit(10)` is ASCII-only. No `str(float)` or
/// `str(Decimal)` can emit a non-ASCII digit, so they agree on every string
/// that reaches here.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10)
        .map(|d| d as usize)
        .ok_or_else(|| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch)))
}

/// The shortest round-trip decimal digits of `a` (finite, non-negative), plus
/// CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// Rust's shortest `{:e}` and CPython's `repr` agree on every double except an
/// exact tie, where CPython breaks to even and Rust breaks away from zero.
/// Rust's fixed-precision `{:.*e}` *is* correctly rounded half-to-even, so
/// re-emitting the same number of significant digits through it applies
/// CPython's rule — kept only if it still round-trips.
fn shortest_repr_digits(a: f64) -> (String, i32) {
    let split = |s: &str| -> (String, i32) {
        let (mant, exp) = s.split_once('e').expect("{:e} always emits an 'e'");
        (
            mant.chars().filter(|c| *c != '.').collect(),
            exp.parse::<i32>().expect("{:e} exponent is an integer") + 1,
        )
    };

    let shortest = format!("{:e}", a);
    let (digits, decpt) = split(&shortest);

    let ties_even = format!("{:.*e}", digits.len() - 1, a);
    if ties_even.parse::<f64>() == Ok(a) {
        return split(&ties_even);
    }
    (digits, decpt)
}

/// CPython's `str(float)` / `repr(float)`
/// (`PyOS_double_to_string(v, 'r', 0, Py_DTSF_ADD_DOT_0)`):
///
/// * exponent form when `decpt <= -4 || decpt > 16` (`str(1e16) == "1e+16"`,
///   `str(1e-05) == "1e-05"`), with the exponent `%+.02d`;
/// * otherwise positional, appending `.0` when nothing follows the point
///   (so `str(1.0) == "1.0"`).
///
/// The sign is the sign *bit*, not `v < 0.0`, so `str(-0.0) == "-0.0"` and
/// PAP's `startswith("-")` fires on negative zero.
fn py_str_f64(v: f64) -> String {
    // Unreachable from the shim (it derives `precision` from a finite Decimal),
    // handled anyway so this is a faithful `str()`: PAP would go on to raise
    // ValueError on `int("inf")`/`int("nan")`, exactly as Python does.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            // No ADD_DOT_0 in exponent form: `str(1e16)` is "1e+16".
            digits.clone()
        };
        let exp = decpt - 1;
        let (esign, eabs) = if exp < 0 {
            ("-", -(exp as i64))
        } else {
            ("+", exp as i64)
        };
        return format!("{}{}e{}{:0>2}", sign, mantissa, esign, eabs);
    }
    if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndig {
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndig) as usize))
    } else {
        format!("{}{}.{}", sign, &digits[..decpt as usize], &digits[decpt as usize..])
    }
}

/// CPython's `Decimal.__str__` (`_pydecimal.Decimal.__str__`).
///
/// Read the digits and exponent off `as_bigint_and_exponent()` and reassemble by
/// Python's rule — `BigDecimal`'s own `Display` disagrees on `1E+2` (Python
/// keeps it exponential, so PAP raises), on `0.0` (Python keeps the ".0"), and
/// on the `e`/`E` case. `BigDecimal::from_str` keeps the written scale
/// (`"1.10"` stays coefficient 110 / scale 2), which is what makes the trailing
/// "sero" appear, and `(coefficient, -scale)` is exactly Python's `(_int, _exp)`.
fn py_str_decimal(value: &BigDecimal) -> String {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    let exp = -scale;
    let sign = if coefficient.is_negative() { "-" } else { "" };
    let int_digits = coefficient.abs().to_string();
    let ndig = int_digits.len() as i64;

    let leftdigits = exp + ndig;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= ndig {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - ndig) as usize)),
            String::new(),
        )
    } else {
        (
            int_digits[..dotplace as usize].to_string(),
            format!(".{}", &int_digits[dotplace as usize..]),
        )
    };

    // `['e', 'E'][context.capitals]`, and the default context has capitals=1.
    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A currency call shaped like the binding's: `separator: None` means the
    /// kwarg was omitted, so `default_separator()` (" ") applies. `arg` is the
    /// corpus's `repr(value)`, and the int/float split is read off it exactly
    /// as the Python shim does.
    fn cur(arg: &str, code: &str) -> String {
        let is_int = !arg.contains('.');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangPap::new().to_currency(&v, code, true, None, false).unwrap()
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangPap::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    /// Every `"lang": "pap", "to": "currency:*"` row in `bench/corpus.jsonl`.
    ///
    /// The four fallback codes below (JPY/KWD/BHD/CHF stand in for the seven)
    /// are the load-bearing ones: they prove `to_currency` neither raises on an
    /// unknown code nor honours a 0-/3-decimal divisor (bug notes 7 and 8).
    #[test]
    fn corpus_currency() {
        for (code, unit) in [("EUR", "euro"), ("USD", "dolar"), ("GBP", "florin"),
                             ("JPY", "florin"), ("KWD", "florin"), ("BHD", "florin"),
                             ("INR", "florin"), ("CNY", "florin"), ("CHF", "florin")] {
            assert_eq!(cur("0", code), format!("sero {unit}"));
            assert_eq!(cur("1", code), format!("un {unit}"));
            assert_eq!(cur("2", code), format!("dos {unit}"));
            assert_eq!(cur("100", code), format!("shen {unit}"));
            assert_eq!(cur("1000000", code), format!("un miyon {unit}"));
            assert_eq!(cur("1.0", code), format!("un {unit}"));
            assert_eq!(cur("0.01", code), format!("sero {unit} un sèn"));
            assert_eq!(cur("0.5", code), format!("sero {unit} sinkuenta sèn"));
            assert_eq!(cur("12.34", code), format!("diesdos {unit} trinta i kuater sèn"));
            assert_eq!(cur("-12.34", code), format!("menos diesdos {unit} trinta i kuater sèn"));
            assert_eq!(
                cur("99.99", code),
                format!("nobenta i nuebe {unit} nobenta i nuebe sèn")
            );
            assert_eq!(
                cur("1234.56", code),
                format!("mil dos shen i trinta i kuater {unit} sinkuenta i seis sèn")
            );
        }
    }

    /// Every `"lang": "pap", "to": "cheque:*"` row. EUR/USD resolve; the other
    /// seven raise, because `to_cheque` subscripts `CURRENCY_FORMS` instead of
    /// `.get`-ing it with the ANG default (bug note 7).
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "MIL DOS SHEN I TRINTA I KUATER AND 56/100 EURO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "MIL DOS SHEN I TRINTA I KUATER AND 56/100 DOLAR"
        );
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{code}\" not implemented for \"Num2Word_PAP\"")
                ),
                other => panic!("{code}: expected NotImplementedError, got {other:?}"),
            }
        }
    }

    /// Off-corpus checks against the live interpreter, covering the paths the
    /// corpus leaves untested. Each expectation was read out of Python.
    #[test]
    fn quirks_match_python() {
        // Truncation, not ROUND_HALF_UP: Base would say sesenta i ocho.
        assert_eq!(cur("2.675", "EUR"), "dos euro sesenta i shete sèn");
        // Sub-cent digits vanish without carrying.
        assert_eq!(cur("1.005", "EUR"), "un euro");
        assert_eq!(cur("12.999", "EUR"), "diesdos euro nobenta i nuebe sèn");
        // has_decimal is never consulted: Decimal("5.00") prints no sèn.
        let d = CurrencyValue::Decimal {
            value: BigDecimal::from_str("5.00").unwrap(),
            has_decimal: true,
            is_float: false,
        };
        assert_eq!(
            LangPap::new().to_currency(&d, "EUR", true, None, false).unwrap(),
            "sinku euro"
        );
        // A negative that rounds to nothing keeps its negword.
        assert_eq!(cur("-0.001", "EUR"), "menos sero euro");
        assert_eq!(cur("-1", "EUR"), "menos un euro");
        // The biyon scale reaches to_currency too (#147; was digits).
        assert_eq!(cur("1000000000", "EUR"), "un biyon euro");
        // AWG is the fallback's twin, and ANG is the fallback itself.
        assert_eq!(cur("1", "AWG"), "un florin");
        assert_eq!(cur("1", "ANG"), "un florin");
        // cents=False drops the segment entirely (PAP has no terse branch).
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        assert_eq!(
            LangPap::new().to_currency(&v, "EUR", false, None, false).unwrap(),
            "diesdos euro"
        );
        // adjective=True is accepted and ignored.
        assert_eq!(
            LangPap::new().to_currency(&v, "EUR", true, None, true).unwrap(),
            "diesdos euro trinta i kuater sèn"
        );
        // An explicit separator is inserted raw, with no space of its own —
        // PAP's default " " *is* the whole gap, so a caller passing "," gets
        // "euro,trinta", not "euro, trinta".
        assert_eq!(
            LangPap::new().to_currency(&v, "EUR", true, Some(","), false).unwrap(),
            "diesdos euro,trinta i kuater sèn"
        );
        // pluralize: forms[-1], and "" for an empty tuple.
        let l = LangPap::new();
        let forms = vec!["florin".to_string(), "florins".to_string(), "x".to_string()];
        assert_eq!(l.pluralize(&BigInt::one(), &forms).unwrap(), "florin");
        assert_eq!(l.pluralize(&BigInt::from(2), &forms).unwrap(), "x");
        assert_eq!(l.pluralize(&BigInt::from(2), &[]).unwrap(), "");
    }

    /// A `float` cardinal call as the binding shapes it. `precision` is ignored
    /// by PAP, so any value round-trips; the raw f64 carries the artefacts.
    fn cf(arg: &str) -> String {
        let v = FloatValue::Float {
            value: arg.parse::<f64>().unwrap(),
            precision: 0,
        };
        LangPap::new().to_cardinal_float(&v, None).unwrap()
    }

    /// A `Decimal` cardinal call: the written scale of the string is preserved by
    /// `BigDecimal::from_str`, which is what the exact arm needs.
    fn cd(arg: &str) -> String {
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str(arg).unwrap(),
            precision: 0,
        };
        LangPap::new().to_cardinal_float(&v, None).unwrap()
    }

    /// Every `"lang": "pap", "to": "cardinal"` row whose `arg` carries a dot —
    /// i.e. float input — from `bench/corpus.jsonl`.
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(cf("0.0"), "sero punto sero");
        assert_eq!(cf("0.5"), "sero punto sinku");
        assert_eq!(cf("1.0"), "un punto sero");
        assert_eq!(cf("1.5"), "un punto sinku");
        assert_eq!(cf("2.25"), "dos punto dos sinku");
        assert_eq!(cf("3.14"), "tres punto un kuater");
        assert_eq!(cf("0.01"), "sero punto sero un");
        assert_eq!(cf("0.1"), "sero punto un");
        assert_eq!(cf("0.99"), "sero punto nuebe nuebe");
        assert_eq!(cf("1.01"), "un punto sero un");
        assert_eq!(cf("12.34"), "diesdos punto tres kuater");
        assert_eq!(cf("99.99"), "nobenta i nuebe punto nuebe nuebe");
        assert_eq!(cf("100.5"), "shen punto sinku");
        assert_eq!(
            cf("1234.56"),
            "mil dos shen i trinta i kuater punto sinku seis"
        );
        assert_eq!(cf("-0.5"), "menos sero punto sinku");
        assert_eq!(cf("-1.5"), "menos un punto sinku");
        assert_eq!(cf("-12.34"), "menos diesdos punto tres kuater");
        // The f64-artefact cases: base.float2tuple would reconstruct 675/005,
        // and str(float) carries those exact digits too.
        assert_eq!(cf("1.005"), "un punto sero sero sinku");
        assert_eq!(cf("2.675"), "dos punto seis shete sinku");
    }

    /// Every `"lang": "pap", "to": "cardinal_dec"` row — Decimal input, exact.
    #[test]
    fn corpus_cardinal_dec() {
        assert_eq!(cd("0.01"), "sero punto sero un");
        // Trailing zero survives: str(Decimal("1.10")) == "1.10" -> "... un sero".
        assert_eq!(cd("1.10"), "un punto un sero");
        assert_eq!(cd("12.345"), "diesdos punto tres kuater sinku");
        // Billion-scale exact value (#147; past 10^12 is an OverflowError).
        assert_eq!(
            cd("2000000005.99"),
            "dos biyon sinku punto nuebe nuebe"
        );
        assert_eq!(cd("0.001"), "sero punto sero sero un");
    }

    /// Off-corpus edges read out of the live interpreter.
    #[test]
    fn float_edges_match_python() {
        // Negative zero keeps its sign because PAP tests the *string*, not
        // `value < 0` — the divergence from the base float path.
        assert_eq!(cf("-0.0"), "menos sero punto sero");
        // Large / tiny floats stringify to exponent form, which has no "." (or a
        // stray 'e'), so PAP feeds it to int() and raises ValueError — quoting
        // the exact offending token, as CPython does.
        for (arg, tok) in [("1e16", "1e+16"), ("1e21", "1e+21"), ("1e-05", "1e-05")] {
            let v = FloatValue::Float {
                value: arg.parse::<f64>().unwrap(),
                precision: 0,
            };
            match LangPap::new().to_cardinal_float(&v, None) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", tok)
                ),
                other => panic!("{arg}: expected ValueError, got {other:?}"),
            }
        }
        // A Decimal whose str keeps a "." but embeds an exponent splits, then
        // trips on the 'E' digit: int('E') -> ValueError, quoting just 'E'.
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str("1.23E+4").unwrap(),
            precision: 0,
        };
        match LangPap::new().to_cardinal_float(&v, None) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'E'")
            }
            other => panic!("1.23E+4: expected ValueError, got {other:?}"),
        }
        // Precision override is dropped (PAP's signature has no such kwarg).
        let v = FloatValue::Float {
            value: 0.5,
            precision: 1,
        };
        assert_eq!(
            LangPap::new().to_cardinal_float(&v, Some(3)).unwrap(),
            "sero punto sinku"
        );
    }
}
