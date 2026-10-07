//! Port of `lang_SO.py` (Somali).
//!
//! # Best-candidate words (gladiaio/num2words2#154)
//!
//! UNVERIFIED (#154): "taban" (minus) — best candidate: "negative"
//!   ("tirooyinka taban" = negative numbers, Somali grade-7 maths material); no
//!   source reads a negative number aloud.
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "bilyan" (10^9; so.wikipedia "Bilyan"), composed like the million
//! arm, and raises `OverflowError` from 10^12, which `maxval()` reports. 10^12
//! is spelled inconsistently (tiriliyan / tirilyan), so it is not used. Where
//! the notes below describe the digit fallback ("1000000000", "no overflow
//! check"), they describe Python; that arm is now unreachable.
//!
//! Registry check: `CONVERTER_CLASSES["so"]` is `lang_SO.Num2Word_SO()`, which
//! is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_SO` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. `Num2Word_Base.
//! __init__` only builds `self.cards`/`self.MAXVAL` when one of those three
//! attributes exists, so for Somali it builds **neither**. `to_cardinal` is
//! overridden outright and drives a private `_int_to_word` recursion; `cards`,
//! `maxval` and `merge` therefore stay at their trait defaults here and are
//! never consulted. There is **no overflow check** at any size — see bug 2.
//!
//! Inheritance chain is exactly `Num2Word_SO -> Num2Word_Base -> object`. All
//! four in-scope modes are overridden by `Num2Word_SO`, so nothing in this file
//! depends on inherited behaviour:
//!   * `to_cardinal`    — overridden (string sign-strip + `_int_to_word`)
//!   * `to_ordinal`     — overridden (`to_cardinal` + "-aad")
//!   * `to_ordinal_num` — overridden (`str(number)` + ".")
//!   * `to_year`        — overridden (delegates to `to_cardinal`, ignoring
//!     its own `longval=True` parameter entirely)
//!
//! `setup()` also sets `pointword = "point"` (English; the port says "dhibic",
//! gladiaio/num2words2#154: "sero dhibic shan" = 0.5), used only by the float branch of
//! `to_cardinal`, which is out of scope (integer input only) and unreachable
//! here: an integer's decimal repr never contains ".".
//!
//! ## Currency surface
//!
//! `Num2Word_SO` defines `CURRENCY_FORMS` (SOS/USD/EUR) and overrides
//! `to_currency` **wholesale** — it shares no code with
//! `Num2Word_Base.to_currency`: no `parse_currency_parts`, no `pluralize`, no
//! `CURRENCY_PRECISION`, no `isinstance(val, int)` branch. It does *not*
//! override `to_cheque`, so cheques run `Num2Word_Base.to_cheque`
//! (`currency::default_to_cheque`) against the same forms table. That split is
//! the source of quirk 7 below: an unknown code prints happily via
//! `to_currency` but raises via `to_cheque`.
//!
//! No cross-call mutable state: `Num2Word_SO` sets no flags in one method for
//! another to consume, so the stateless Rust path is a faithful substitute.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Everything below looks wrong but is exactly
//! what Python emits, and every item is confirmed against the frozen corpus:
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** `setup` makes `ones[0]` the
//!    empty string, and `_int_to_word` opens with
//!    `return self.ones[0] if self.ones[0] else "zero"`, so Python always
//!    answered the English "zero". The port says the Somali "eber":
//!    `to_cardinal(0)` == "eber" and `to_ordinal(0)` == "eber-aad".
//! 2. **Numbers >= 10^9 come back as digits, not words.** `_int_to_word`'s
//!    final `else` is `return str(number)  # Fallback for very large numbers`.
//!    There is no `MAXVAL` and no `OverflowError` — the function silently
//!    degrades. So `to_cardinal(10**9)` == "1000000000",
//!    `to_ordinal(10**9)` == "1000000000-aad", and `to_cardinal(10**21)` ==
//!    "1000000000000000000000". All four are corpus rows. This is why
//!    [`LangSo::int_to_word`] takes a `BigInt`: the fallback must render
//!    arbitrarily large values, so the input is genuinely unbounded.
//! 3. **The negword was the English "minus ".** It is now the best
//!    candidate "taban " (see UNVERIFIED above).
//! 4. **Teens and compounds are bare juxtaposition.** 11 is "toban kow"
//!    ("ten one"), not the idiomatic "kow iyo toban"; 100 is "kow boqol"
//!    ("one hundred"), never a bare "boqol". No conjunction is ever inserted
//!    at any magnitude. Reproduced verbatim.
//! 5. **`_int_to_word`'s `number < 0` branch is dead code** on every in-scope
//!    path. `to_cardinal` strips the "-" from the *string* before calling
//!    `int()`, so `_int_to_word` only ever receives a non-negative value from
//!    the four modes in scope. (Only the out-of-scope `to_currency` could reach
//!    it, and it pre-`abs()`es too.) It is preserved in [`LangSo::int_to_word`]
//!    for fidelity, with the same ordering as Python: the `== 0` test precedes
//!    the `< 0` test.
//! 6. **`to_ordinal_num` ignores the sign convention and the language.** It is
//!    `str(number) + "."` — so `to_ordinal_num(-1)` == "-1." and
//!    `to_ordinal_num(0)` == "0.". A trailing dot is the German/Polish ordinal
//!    convention, not a Somali one, and "-aad" (used by `to_ordinal`) never
//!    appears here.
//! 7. **An unknown currency code never raises in `to_currency`** — it silently
//!    becomes Somali shillings. See [`FALLBACK_CURRENCY`].
//! 8. **Cents are read lexically and truncated, never rounded**, and
//!    `CURRENCY_PRECISION` is ignored outright. See [`LangSo::to_currency`].
//!
//! No in-scope integer input raises: the corpus contains zero `ok: false` rows
//! for cardinal/ordinal/ordinal_num/year, so those four modes return `Ok`
//! unconditionally. The currency surface added here raises only where Python
//! does: `to_cheque` on a code outside SOS/USD/EUR (`NotImplementedError`).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use doolar / yuuro with senti. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `setup`: `self.ones`. Index 0 is `""` and is unreachable — see bug 1.
const ONES: [&str; 10] = [
    "", "kow", "laba", "saddex", "afar", "shan", "lix", "toddoba", "siddeed", "sagaal",
];

/// `setup`: `self.tens`. Index 0 is `""` and is unreachable: the `< 100` branch
/// is only entered for `number >= 10`, so `number // 10` is always 1..=9.
const TENS: [&str; 10] = [
    "",
    "toban",
    "labaatan",
    "soddon",
    "afartan",
    "konton",
    "lixdan",
    "toddobaatan",
    "siddeetan",
    "sagaashan",
];

const HUNDRED: &str = "boqol";
const THOUSAND: &str = "kun";
const MILLION: &str = "milyan";

/// `setup`: `self.negword`. English, and kept that way — see bug 3.
const NEGWORD: &str = "taban ";

/// The literal in `_int_to_word`'s zero guard — see bug 1.
const ZERO_WORD: &str = "eber";

/// `_int_to_word`'s `else` threshold: at or above this, Python returns
/// `str(number)` rather than words (bug 2).
const DIGIT_FALLBACK_FLOOR: u32 = 1_000_000_000;

/// The class name, for `to_cheque`'s NotImplementedError message.
const LANG_NAME: &str = "Num2Word_SO";

/// Python: `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
///
/// `list(values())[0]` is the **first-inserted** entry. The class body lists
/// `SOS` before `USD` before `EUR`, and dicts have preserved insertion order
/// since CPython 3.7 — the live interpreter confirms
/// `list(CURRENCY_FORMS.keys()) == ['SOS', 'USD', 'EUR']`. So every unknown
/// code falls back to the Somali shilling rather than raising (bug 7). A
/// `HashMap` has no order of its own, so the choice is pinned to a constant
/// here rather than left to iteration order.
const FALLBACK_CURRENCY: &str = "SOS";

/// The separator the FFI bridge sends when the caller passed none.
///
/// `Num2Word_SO.to_currency` declares `separator=" "`, but the bridge cannot
/// express "caller omitted it": `num2words2/__init__.py` (via
/// `kwargs.get("separator", ",")`) and `bench/diff_test.py` both send
/// `Num2Word_Base`'s default `","` literally. The corpus was generated through
/// the Python path with `separator=` omitted, so every expected string uses
/// `" "` — the 36 currency rows with a cents segment all depend on mapping this
/// back.
///
/// Right for the default call and for every caller who passes anything other
/// than `","`; wrong only for an explicit `separator=","`, which Python renders
/// `"toban laba euros,soddon afar cents"` and this renders with a space. That
/// one case is indistinguishable from the default at this boundary. Fixing it
/// properly means teaching the binding each language's own default, which
/// cannot be done from this file. ~100 of the 156 Python modules override this
/// default, so the issue is systemic rather than SO-specific; the sibling ports
/// (`lang_haw.rs`, `lang_gl.rs`, `lang_br.rs`) adopt the same mapping.
/// Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// SO's own `to_currency` default, restored when [`SEPARATOR_UNSET`] arrives.
const SEPARATOR_DEFAULT: &str = " ";

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 1] = [(9, "bilyan")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

pub struct LangSo {
    /// `CURRENCY_FORMS`, built once in [`LangSo::new`] and cached by the caller
    /// (`num2words2-py` holds this in a `OnceLock`), never per call.
    ///
    /// Every entry carries exactly two unit forms and two subunit forms,
    /// matching Python's tuple arity — `to_currency` indexes `[0]`/`[1]`
    /// directly and `default_to_cheque` takes `.last()`, so the arity is
    /// load-bearing. SOS's two forms are deliberately identical ("shilin",
    /// "shilin"): Somali does not inflect it here, and that is Python's table,
    /// not a typo to fix.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangSo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSo {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        let mut forms = HashMap::with_capacity(3);
        forms.insert(
            "SOS",
            CurrencyForms::new(&["shilin", "shilin"], &["sent", "sent"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["doolar", "doolar"], &["senti", "senti"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["yuuro", "yuuro"], &["senti", "senti"]),
        );
        LangSo { forms }
    }

    /// Python's `Num2Word_SO._int_to_word`.
    ///
    /// Branch order is Python's, which matters: `== 0` is tested before `< 0`,
    /// and the digit fallback is only reachable for *positive* values >= 10^9
    /// because the `< 0` branch has already recursed on `abs(number)`.
    fn int_to_word(&self, number: &BigInt) -> String {
        // `if number == 0: return self.ones[0] if self.ones[0] else "zero"`
        // — `ones[0]` is "" (falsy), so this is always "zero". See bug 1.
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // `if number < 0: return self.negword + self._int_to_word(abs(number))`
        // Dead on every in-scope path (bug 5); preserved for fidelity.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // `else: return str(number)` — the digit fallback (bug 2). Must stay on
        // BigInt: `number` is unbounded here.
        if *number >= BigInt::from(DIGIT_FALLBACK_FLOOR) {
            // Scale words above a million (gladiaio/num2words2#147), composed like
            // the million arm. Every entry point rejects values at or above
            // `maxval_ceiling()` first, so the top quotient is always below 1000.
            for &(exp, word) in SCALES.iter().rev() {
                let scale = pow10_big(exp);
                if number >= &scale {
                    let mut result = format!("{} {}", self.int_to_word(&(number / &scale)), word);
                    let rest = number % &scale;
                    if !rest.is_zero() {
                        result.push(' ');
                        result.push_str(&self.int_to_word(&rest));
                    }
                    return result;
                }
            }
            // Unreachable: callers check `maxval_ceiling()` first.
            return number.to_string();
        }

        // Past this point Python's remaining branches all bound `number` to
        // 1..=999_999_999, which is provably within u32 — so narrowing here is
        // safe and the rest of the recursion can use fixed-width arithmetic.
        // `to_u32` cannot fail given the checks above; fall back to the digit
        // string rather than panic if that reasoning is ever invalidated.
        match number.to_u32() {
            Some(n) => self.small_to_word(n),
            None => number.to_string(),
        }
    }

    /// The `1 <= number < 10^9` tail of `_int_to_word`.
    ///
    /// Every recursive call Python makes here (`remainder`, `thousands_val`,
    /// `millions_val`) is itself in `1..=999_999_999`, so recursing on `u32`
    /// rather than back through [`LangSo::int_to_word`] is equivalent: neither
    /// the zero guard, the negative branch, nor the digit fallback can trigger.
    /// Each `remainder` recursion is guarded by Python's `if remainder:`.
    fn small_to_word(&self, number: u32) -> String {
        if number < 10 {
            // `return self.ones[number]`
            return ONES[number as usize].to_string();
        }

        if number < 100 {
            let tens_val = (number / 10) as usize;
            let ones_val = (number % 10) as usize;
            if ones_val == 0 {
                return TENS[tens_val].to_string();
            }
            return format!("{} {}", TENS[tens_val], ONES[ones_val]);
        }

        if number < 1_000 {
            // Note: hundreds uses `self.ones[hundreds_val]` *directly*, not a
            // recursive call — hence "kow boqol" for 100, never a bare "boqol".
            let hundreds_val = (number / 100) as usize;
            let remainder = number % 100;
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            return result;
        }

        if number < 1_000_000 {
            let thousands_val = number / 1_000;
            let remainder = number % 1_000;
            let mut result = format!("{} {}", self.small_to_word(thousands_val), THOUSAND);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.small_to_word(remainder));
            }
            return result;
        }

        // `elif number < 1000000000:` — guaranteed by the caller.
        let millions_val = number / 1_000_000;
        let remainder = number % 1_000_000;
        let mut result = format!("{} {}", self.small_to_word(millions_val), MILLION);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&self.small_to_word(remainder));
        }
        result
    }

    /// The float/Decimal arm of `Num2Word_SO.to_cardinal`, driven by the
    /// *string* `str(number)` exactly as Python is.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword          # "minus "
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// This is deliberately **not** the base `float2tuple` path: SO reads the
    /// literal repr digits, so a high-precision float keeps its exact shortest
    /// round-trip fraction (`0.12345678901234` speaks all 14 digits) rather than
    /// the `<0.01`-rescued / floored `post` the base path would compute. The
    /// integer [`Lang::to_cardinal`] models the same method on the *value*, which
    /// is only equivalent while there is no ".".
    ///
    /// Three faithful oddities:
    ///
    /// * The sign is stripped off the *string*, then `ret` (the negword) prefixes
    ///   both branches — so a negative with zero integer part still prints
    ///   `"taban eber ..."` (`int_to_word(0)` is "zero", bug 1), and the "."
    ///   branch keeps its negword too.
    /// * `int(digit)` runs per **character**, so a malformed fraction character
    ///   raises `ValueError` quoting that one char, where a malformed whole `n`
    ///   (e.g. the exponent-form repr `"1e+21"`) quotes the entire literal.
    /// * Each fraction digit goes through [`LangSo::int_to_word`], not the raw
    ///   `ones` table, so digit 0 speaks "zero" (not the empty `ones[0]`).
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        // Python's `str(number).strip()`; `str()` never emits surrounding space,
        // so this only ever no-ops. Reproduced rather than assumed away.
        let n = n.trim();

        // `if n.startswith("-"): n = n[1:]; ret = self.negword; else: ret = ""`
        let (n, ret): (&str, &str) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD),
            None => (n, ""),
        };

        if let Some((left, right)) = n.split_once('.') {
            // `ret += self._int_to_word(int(left)) + " " + self.pointword + " "`
            let mut out = String::from(ret);
            out.push_str(&self.checked_int_to_word(&python_int(left)?)?);
            out.push(' ');
            out.push_str(self.pointword());
            out.push(' ');
            // `for digit in right: ret += self._int_to_word(int(digit)) + " "`
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                out.push_str(&self.checked_int_to_word(&BigInt::from(d))?);
                out.push(' ');
            }
            // `return ret.strip()`
            Ok(out.trim().to_string())
        } else {
            // `return (ret + self._int_to_word(int(n))).strip()`
            Ok(format!("{}{}", ret, self.checked_int_to_word(&python_int(n)?)?)
                .trim()
                .to_string())
        }
    }
}

impl Lang for LangSo {
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

    /// `to_ordinal(float/Decimal)`. SO's `to_ordinal` is
    /// `self.to_cardinal(number) + "-aad"` for *every* input, so the float
    /// entry is the float cardinal plus the suffix — "kow dhibic shan-aad".
    /// An exponent-form Decimal repr ("1E+2") still dies in `int()` with
    /// ValueError inside the cardinal, before the suffix is ever appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-aad", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr the
    /// binding computed, dot appended, sign and exponent form included
    /// ("-0.0.", "1e+16.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError fires
    /// later, inside SO's `int("Infinity")` (`to_cardinal` reads
    /// `str(number)`, strips the sign, finds no "." and calls `int()`).
    /// The binding otherwise hard-codes `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any SO code runs, so the
    /// ValueError must be raised here. The sign is sliced off before
    /// `int()` sees it, so both signs quote the same literal.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "SOS"
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
        "dhibic"
    }

    /// Python's `Num2Word_SO.to_cardinal`.
    ///
    /// Python works on the *string*: `n = str(number).strip()`, then strips a
    /// leading "-" off `n` and re-parses with `int(n)`. For integer input that
    /// is exactly `value.abs()` (a decimal repr has no surrounding whitespace,
    /// and BigInt has no negative zero), so the string round-trip is elided.
    /// The `"." in n` float branch is unreachable for integers and omitted.
    ///
    /// The trailing `.strip()` is kept as `trim()`: `negword` ends in a space,
    /// so it would matter if `_int_to_word` ever returned an empty string. It
    /// cannot (0 yields "zero", bug 1), making the trim a no-op in practice —
    /// but it is Python's, so it stays.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (magnitude, ret) = if value.is_negative() {
            (value.abs(), NEGWORD)
        } else {
            (value.clone(), "")
        };

        // `_int_to_word` receives a non-negative value here — hence bug 5.
        Ok(format!("{}{}", ret, self.checked_int_to_word(&magnitude)?)
            .trim()
            .to_string())
    }

    /// `return cardinal + "-aad"`. Applied to the whole string, so the suffix
    /// lands on the last word only: `to_ordinal(-1)` == "taban kow-aad" and
    /// `to_ordinal(10**9)` == "1000000000-aad" (bug 2).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-aad", self.to_cardinal(value)?))
    }

    /// `return str(number) + "."` — sign and all. See bug 6.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is accepted and then ignored, so years get no century
    /// treatment: 1900 is plain "kow kun sagaal boqol", not "nineteen hundred".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the float/Decimal path of `Num2Word_SO.to_cardinal`.
    ///
    /// SO overrides `to_cardinal` (not `to_cardinal_float`) and handles
    /// non-integers inline off `str(number)`. The dispatcher never routes SO
    /// through the base `to_cardinal_float`, so the Rust core reconstructs
    /// `str(number)` — [`python_str`] mirrors CPython's `repr(float)` and
    /// `str(Decimal)` exactly — and replays SO's string algorithm in
    /// [`LangSo::cardinal_from_str`].
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**: SO's
    /// `to_cardinal` takes no precision argument and never reads `self.precision`
    /// (confirmed against the live interpreter — `precision=` leaves the output
    /// unchanged), so honouring it would diverge from Python.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_from_str(&python_str(value))
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `CURRENCY_FORMS[code]` — a strict lookup, used only by
    /// `default_to_cheque`, which turns `None` into Python's
    /// `NotImplementedError`.
    ///
    /// `to_currency` deliberately does **not** route through this hook: Python
    /// reaches the same dict via `.get(currency, <SOS forms>)`, which never
    /// raises. Hence `to_cheque("GBP")` raises while `to_currency("GBP")`
    /// happily prints shillings. That asymmetry is real and the corpus pins
    /// both halves of it.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // `currency_precision` is intentionally NOT overridden: `Num2Word_SO`
    // inherits `Num2Word_Base.CURRENCY_PRECISION`, which is `{}` (confirmed
    // against the live interpreter), so every code takes the `.get(code, 100)`
    // default of 100 — exactly the trait default. This is why BHD/KWD
    // (nominally 3-decimal) and JPY (nominally 0-decimal) behave like EUR.
    //
    // `currency_adjective` is NOT overridden: `CURRENCY_ADJECTIVES` is `{}`.
    //
    // `pluralize` / `cents_verbose` / `cents_terse` are NOT overridden and are
    // unreachable: SO's `to_currency` does its own form selection, and
    // `default_to_cheque` takes `forms.unit.last()` without pluralizing. The
    // trait default for `pluralize` raises NotImplemented, mirroring Python's
    // abstract `raise NotImplementedError` — correct precisely because nothing
    // calls it.
    //
    // `money_verbose` is NOT overridden: base's `_money_verbose` is
    // `self.to_cardinal(number)`, which is the trait default, and it dispatches
    // to SO's overridden `to_cardinal`. `to_cheque` relies on that.
    //
    // `to_cheque` is NOT overridden: `Num2Word_SO` does not define it, so
    // Python runs `Num2Word_Base.to_cheque` — `currency::default_to_cheque`.

    /// Port of `Num2Word_SO.to_currency` — a full override sharing no code with
    /// `Num2Word_Base.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="SOS", cents=True, separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
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
    /// # Quirks reproduced (each pinned by a corpus row)
    ///
    /// 7. **An unknown currency never raises.** `.get(currency, <SOS forms>)`
    ///    silently substitutes the Somali shilling, so `to_currency(1, "GBP")`
    ///    is `"kow shilin"` — a pound rendered as a *shilling*, with no error.
    ///    The corpus pins this for GBP/JPY/KWD/BHD/INR/CNY/CHF, all of which
    ///    come out identical to SOS.
    /// 8. **Cents are parsed lexically and truncated, never rounded.**
    ///    `parts[1][:2]` takes the first two fraction *characters*, so `0.5` ->
    ///    `"5"` -> `ljust(2, "0")` -> `"50"` -> 50 cents (right), but `12.345`
    ///    -> `"34"` -> 34 cents, **truncating** the 5 that base's
    ///    `ROUND_HALF_UP` would carry to 35. `parse_currency_parts` is never
    ///    reached.
    /// 9. **`CURRENCY_PRECISION` is ignored outright.** The divisor is
    ///    hard-wired to "first two fraction digits", so JPY (0-decimal) still
    ///    gets cents — `to_currency(12.34, "JPY")` is
    ///    `"toban laba shilin soddon afar sent"` — and KWD/BHD get 2 subunits,
    ///    not 3. Corpus-confirmed for all four codes.
    /// 10. **`adjective` is accepted and never read.** `CURRENCY_ADJECTIVES` is
    ///    empty anyway, so even base's behaviour would be a no-op.
    /// 11. **Int and float converge here, but only by accident.** Python
    ///    branches on `str(val)`, not `isinstance(val, int)`: `str(1)` is `"1"`
    ///    (one part -> `right = 0`) while `str(1.0)` is `"1.0"`
    ///    (`parts[1] == "0"`, truthy -> `right = int("00") == 0`). Both land on
    ///    `right == 0`, which is falsy, so both skip cents and print
    ///    `"kow euro"`. The two variants are kept distinct below rather than
    ///    collapsed, because the *reason* they agree is arithmetic on `right`,
    ///    not a shared branch.
    /// 12. **`cents=False` drops the segment entirely** rather than falling back
    ///    to the terse digit form base would use — `_cents_terse` is
    ///    unreachable.
    ///
    /// # Errors
    ///
    /// `N2WError::Value` where Python's `int()` raises `ValueError` on a
    /// non-numeric field — reachable only for a float whose repr is in
    /// scientific notation (see the port report's concerns).
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
        let _ = adjective; // Python names the parameter and never reads it. Quirk 10.

        // Restore SO's own separator default. See [`SEPARATOR_UNSET`].
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Python: `if val < 0: is_negative = True; val = abs(val)` — the abs()
        // happens *before* str(), so the sign never reaches the split.
        let is_negative = val.is_negative();
        let s = match val {
            // `str(int)` — never contains ".", so `parts` has length 1 and
            // `right` stays 0. Quirk 11.
            CurrencyValue::Int(i) => if is_negative { i.abs() } else { i.clone() }.to_string(),
            // `str(float)`. The Python shim already stringified the float with
            // its own repr and the core parsed that, so `BigDecimal::to_string`
            // reproduces `str(val)` exactly for every ordinary decimal repr —
            // crucially preserving the trailing ".0" of `1.0`, which is what
            // makes quirk 11 work out.
            CurrencyValue::Decimal { value: d, .. } => if is_negative { d.abs() } else { d.clone() }.to_string(),
        };

        // Python: `parts = str(val).split(".")`, then indexes `parts[0]` and
        // `parts[1]`. Splitting lazily gives the same two fields: `p1` is the
        // text between the first and second dot, exactly as Python would see it.
        let mut parts = s.split('.');
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(p0).map_err(|_| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", p0))
            })?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // Sliced by chars(), per the porting contract. Quirk 8.
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut f: String = frac.chars().take(2).collect();
                while f.chars().count() < 2 {
                    f.push('0'); // ljust(2, "0")
                }
                BigInt::from_str(&f).map_err(|_| {
                    N2WError::Value(format!("invalid literal for int() with base 10: '{}'", f))
                })?
            }
            _ => BigInt::zero(),
        };

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — SOS on a miss,
        // never an error. Quirk 7.
        let forms = self.forms.get(currency).unwrap_or_else(|| {
            self.forms
                .get(FALLBACK_CURRENCY)
                .expect("new() always inserts SOS")
        });
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::from(1);
        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is falsy at 0, so a float with zero
        // cents (1.0) prints no cents segment. Quirks 11 and 12.
        if cents && !right.is_zero() {
            // Python: `result += separator + cents_str + " " + ...`. No space is
            // inserted *before* the separator; SO's " " default supplies it.
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                self.checked_int_to_word(&right)?,
                if right != one { &cr2[1] } else { &cr2[0] }
            ));
        }

        // `if is_negative: result = self.negword + result` — "minus " (with its
        // trailing space) is prepended, then the whole string is stripped.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}

/// `str(number)` for whatever the Python dispatcher handed SO's `to_cardinal`.
///
/// The [`FloatValue`] split is exactly Python's `isinstance(value, Decimal)`:
/// the two arms stringify by different rules (`repr(float)` vs `str(Decimal)`)
/// and must not be collapsed — issue #603 exists precisely because a `float()`
/// cast rounds a large Decimal away.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Python's `int(s)`, for the strings `str()` can produce.
///
/// `BigInt::from_str` and `int()` agree on every reachable input: plain ASCII
/// digit runs with an optional sign. Where they diverge (`int()` also accepts
/// surrounding whitespace, underscores, non-ASCII digits) is unreachable from
/// `str(float)` / `str(Decimal)`. The message is Python's, quoting the offending
/// literal verbatim — so an exponent-form repr like `"1e+21"` (no ".") reaches
/// the non-fractional branch and raises `ValueError: invalid literal for int()
/// with base 10: '1e+21'`, exactly as the live interpreter does.
fn python_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// CPython's `repr(float)` — shortest round-trip, with its exact exponent and
/// `.0` rules. Rust's `{}` is shortest round-trip too but never adds a `.0` and
/// never uses exponent form, so `1.0` and `1e16` would both come out wrong; this
/// transcribes CPython's `float_repr` / `format_float_short` instead.
///
/// The carried `precision` is deliberately *not* used to shortcut this: it is
/// `abs(Decimal(str(value)).as_tuple().exponent)`, which for an exponent-form
/// repr is the exponent, not a fraction-digit count (`1e16` arrives with
/// `precision == 16`).
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf); fed straight to int(), which rejects
    // them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `{:e}` is shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the digits
    // and the decimal-point position fall straight out. `decpt` is CPython's:
    // the value is `0.<digits> * 10**decpt`.
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').expect("LowerExp always emits an 'e'");
    let exp: i32 = exp.parse().expect("LowerExp emits an integer exponent");
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut decpt = exp + 1;

    // Tie repair: `{:e}` and CPython can pick different-looking shortest forms
    // for a half-way value. Re-round to the same number of fractional digits to
    // realign. Only reachable when the shortest form has fractional digits.
    let frac_digits = digits.chars().count() as i32 - decpt;
    if frac_digits > 0 && a != 0.0 {
        let t = format!("{:.*}", frac_digits as usize, a);
        let (ip, fp) = t.split_once('.').expect("frac_digits > 0 forces a point");
        let all = format!("{}{}", ip, fp);
        let trimmed = all.trim_start_matches('0');
        if !trimmed.is_empty() {
            let lead = all.chars().count() - trimmed.chars().count();
            digits = trimmed.to_string();
            decpt = ip.chars().count() as i32 - lead as i32;
        }
    }

    let n = digits.chars().count() as i32;

    if decpt <= -4 || decpt > 16 {
        // CPython: mantissa, then "e", then "%+.02d" of decpt-1.
        let e = decpt - 1;
        let mut out = String::from(sign);
        let mut it = digits.chars();
        out.push(it.next().expect("a finite double has at least one digit"));
        if n > 1 {
            out.push('.');
            out.push_str(it.as_str());
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (e as i64).abs()));
        out
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= n {
        // Py_DTSF_ADD_DOT_0: an integral value still reprs with a ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - n) as usize))
    } else {
        let k = decpt as usize;
        format!(
            "{}{}.{}",
            sign,
            digits.chars().take(k).collect::<String>(),
            digits.chars().skip(k).collect::<String>()
        )
    }
}

/// CPython's `str(Decimal)` — the spec's to-scientific-string, transcribed from
/// `_pydecimal.Decimal.__str__`:
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # no exponent required
/// else:
///     dotplace = 1                   # scientific: 1 digit left of the point
/// if dotplace <= 0:
///     intpart, fracpart = '0', '.' + '0'*(-dotplace) + self._int
/// elif dotplace >= len(self._int):
///     intpart, fracpart = self._int + '0'*(dotplace-len(self._int)), ''
/// else:
///     intpart, fracpart = self._int[:dotplace], '.' + self._int[dotplace:]
/// exp = '' if leftdigits == dotplace else 'E' + "%+d" % (leftdigits-dotplace)
/// return sign + intpart + fracpart + exp
/// ```
///
/// `BigDecimal` is the same `(unscaled, scale)` pair as Python's `(_int, _exp)`
/// (`_exp == -scale`), and `from_str` preserves the scale as written, so
/// `Decimal("1.10")`'s trailing zero survives. This reads
/// `as_bigint_and_exponent()` rather than `BigDecimal`'s own `Display`, which is
/// **not** `str(Decimal)` (it renders `Decimal("0.00")` as `"0"`, dropping the
/// digits SO would speak). The capital `E` and unpadded exponent are Python's.
///
/// # The negative-zero hole
///
/// Python's `Decimal` carries a sign flag independent of its digits, so
/// `Decimal("-0.0")` is negative. `BigInt` has no negative zero, so
/// `BigDecimal::from_str("-0.0")` discards the sign before this function sees it,
/// and the leading `minus` is lost for `Decimal("-0")`/`"-0.0"`/`"-0.00"` only.
/// The discriminator is the original string, which the `FloatValue::Decimal`
/// boundary does not carry. The *float* `-0.0` is fine (f64 keeps its sign bit).
/// Flagged in the port report.
fn python_decimal_str(d: &BigDecimal) -> String {
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let sign = if unscaled.is_negative() { "-" } else { "" };
    // Python's `_int`: the unsigned coefficient. BigInt renders ASCII digits.
    let int_digits = unscaled.abs().to_string();
    let exp: i64 = -scale;
    let ndig = int_digits.chars().count() as i64;
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
        let k = dotplace as usize;
        (
            int_digits.chars().take(k).collect::<String>(),
            format!(".{}", int_digits.chars().skip(k).collect::<String>()),
        )
    };

    // `"%+d"` — always signed, never zero-padded. Capital E: context.capitals
    // defaults to 1.
    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}
