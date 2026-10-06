//! Port of `lang_ZU.py` (Zulu / isiZulu).
//!
//! Registry check: `CONVERTER_CLASSES["zu"]` is `lang_ZU.Num2Word_ZU`
//! (`__init__.py:401`), so this is the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_ZU` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and **never sets
//! `MAXVAL`**. Every in-scope entry point is overridden, so the engine
//! (`splitnum`/`clean`/`merge`) is unreachable and `cards`/`maxval`/`merge`
//! stay at their trait defaults. There is no overflow check at all — see the
//! `str(number)` fallthrough below.
//!
//! Nothing is inherited from `Num2Word_Base` in the integer scope: ZU overrides
//! `to_cardinal`, `to_ordinal`, `to_ordinal_num` and `to_year` outright.
//! (`to_year(self, val, longval=True)` ignores `longval` entirely and just
//! calls `to_cardinal`, which is why the trait's single-argument `to_year`
//! is a faithful signature here.)
//!
//! The currency surface is split: ZU overrides `to_currency` and `pluralize`,
//! but inherits `to_cheque`, `_money_verbose`, `_cents_verbose` and
//! `_cents_terse` from `Num2Word_Base` untouched — verified against the live
//! interpreter (`to_cheque.__func__.__qualname__ == "Num2Word_Base.to_cheque"`),
//! so those stay at their trait defaults here. `CURRENCY_ADJECTIVES` and
//! `CURRENCY_PRECISION` are both inherited *empty*, so every code resolves to
//! the default divisor 100 and `adjective=` can never fire; neither hook is
//! overridden.
//!
//! `CURRENCY_FORMS` is ZU's **own** class attribute, not `Num2Word_EUR`'s, so
//! the `lang_EUR.py` mutation trap (EN's `__init__` rewriting the shared class
//! dict in place) does not apply — `CONVERTER_CLASSES["zu"].CURRENCY_FORMS`
//! reads back exactly the three entries in the source literal, in source order.
//!
//! No cross-call mutable state: no method sets an instance flag that another
//! reads. `setup()` assigns only constant tables, and `self.precision` is
//! touched solely by the base class's float paths, which ZU never reaches:
//! its `to_cardinal` handles floats and Decimals inline as *string*
//! operations on `str(number)`, so `float2tuple`/`to_cardinal_float` from
//! `base.py` never run for this language. Consequently the `precision=`
//! kwarg (issue #580) is a **no-op** for ZU — the dispatcher sets
//! `converter.precision` (the attribute exists, `Num2Word_Base.__init__`
//! sets it to 2) but nothing on ZU's path ever reads it back — so the Rust
//! `to_cardinal_float` override ignores `precision_override`, verified live:
//! `num2words(0.5, lang="zu", precision=3)` == `"iqanda ichashazi kuhlanu"`.
//!
//! # Faithfully reproduced Python behaviour
//!
//! These all look wrong and are all exactly what Python emits. Verified
//! against the frozen corpus (`bench/corpus.jsonl`, lang "zu"):
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the raw decimal string.**
//!    The final `return str(number)` has no guard: `to_cardinal(10**9)` ==
//!    `"1000000000"`, not words, and no `OverflowError` is raised. Corpus
//!    confirms this all the way to 10^21 (`"1000000000000000000000"`), so the
//!    value must stay a `BigInt` — it is never bounded and never parsed.
//!    Consequently `to_ordinal(10**9)` == `"we-1000000000"` and
//!    `to_cardinal(-10**9)` == `"ngaphansi kwe 1000000000"`.
//! 2. **`negword` is `"ngaphansi kwe "` with a trailing space and is
//!    concatenated raw**, not `"%s " % negword.strip()` the way
//!    `Num2Word_Base.to_cardinal` does it. The spacing works out identically,
//!    but the trailing `.strip()` in ZU's `to_cardinal` is what would absorb a
//!    doubled space — it is a no-op for every reachable integer input.
//! 3. **`to_ordinal` never calls `verify_ordinal`**, so negatives and zero
//!    sail through instead of raising `TypeError`: `to_ordinal(0)` ==
//!    `"we-iqanda"` and `to_ordinal(-1)` == `"we-ngaphansi kwe kunye"` —
//!    an ordinal prefix glued onto a negation. Corpus agrees.
//! 4. **`to_ordinal_num` interpolates the sign**: `"we-" + str(-1)` produces
//!    the double hyphen `"we--1"`. Corpus agrees.
//! 5. **The `na` joiner is uniform and unelided.** Zulu orthography would
//!    normally contract "na + i" (na + ikhulu → nekhulu), but the module just
//!    concatenates `" na "`, yielding `"ikhulu na ishumi na kunye"` for 111
//!    and `"inkulungwane na ikhulu"` for 1100. Preserved verbatim.
//! 6. **Scale words carry no multiplier when the quotient is 1** (`if h > 1`,
//!    `if t > 1`), so 100 == `"ikhulu"` and 1000 == `"inkulungwane"` with no
//!    leading "kunye" — but 2000 == `"kubili inkulungwane"`.
//! 7. **An unknown currency code does not raise — it silently borrows ZAR.**
//!    `to_currency` looks the code up with
//!    `.get(currency, list(self.CURRENCY_FORMS.values())[0])`, and ZAR is the
//!    first key inserted, so `to_currency(1, "JPY")` == `"kunye randi"`: Zulu
//!    rand for a yen amount. `to_cheque` is *not* ZU's, and does the strict
//!    `self.CURRENCY_FORMS[currency]`, so `to_cheque(1234.56, "JPY")` raises
//!    NotImplementedError on the same input. Both halves are corpus-pinned;
//!    see [`FALLBACK_CODE`].
//! 8. **`_int_to_word`'s 10^9 digit-string fallthrough leaks into money.**
//!    `to_currency(10**9, "ZAR")` == `"1000000000 randi"`, and
//!    `to_currency(1000000000.5, "ZAR")` ==
//!    `"1000000000 randi amashumi amahlanu isenti"` — bare digits next to a
//!    spelled-out cents clause, because `to_currency` calls `_int_to_word`
//!    directly rather than going through `to_cardinal`.
//! 9. **The float path is `str(number)` surgery, not arithmetic.** Digits
//!    after the point are read one character at a time off the repr, so
//!    `base.float2tuple`'s binary-rounding artefacts never arise: `2.675` →
//!    `"2.675"` → `"... isithupha isikhombisa kuhlanu"` (6-7-5, no rescue
//!    heuristic needed), Decimal trailing zeros survive (`Decimal("1.10")` →
//!    `"kunye ichashazi kunye iqanda"`), and the 10^9 digit fallthrough
//!    leaks into the integer part (`1234567890.5` →
//!    `"1234567890 ichashazi kuhlanu"`). All corpus-pinned.
//! 10. **Exponent notation crashes it.** `str(float)` flips to scientific
//!    below 1e-4 (`repr(1e-05)` == `"1e-05"`) and `str(Decimal)` below an
//!    adjusted exponent of -6 (`str(Decimal("1E-7"))` == `"1E-7"`), and the
//!    `int()` calls then choke: no-dot forms fail on the whole literal
//!    (`int("1e-05")` → `ValueError: invalid literal for int() with base 10:
//!    '1e-05'`), dotted forms fail on the first non-digit fraction char
//!    (`1.5e-05` → right part `"5e-05"` → `int("e")` → `... 'e'`, and
//!    `Decimal("1.05E-7")` → `... 'E'`). All four shapes verified live;
//!    reproducing them requires reconstructing Python's exact float-repr /
//!    Decimal-str spellings, which [`py_float_repr`]/[`py_decimal_str`] do.
//! 11. **`precision=` is dead** — see the header note; `_precision_override`
//!    is deliberately unused in `to_cardinal_float`.
//!
//! # A note on the unreachable negative-index hazard
//!
//! `_int_to_word` does `self.ones[number]` for `number < 10` without a lower
//! bound. Reached with a negative it would silently wrap around Python's
//! negative list indexing (`ones[-1]` == `"isishiyagalolunye"`) rather than
//! raise. It is unreachable from the four in-scope entry points because
//! `to_cardinal` peels the sign off first, so this port takes `&BigInt`
//! non-negative in [`LangZu::int_to_word`] and does not model the wraparound.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// `self.ones`. Index 0 is `""`; it is never reached by the integer modes
/// because `_int_to_word` returns "iqanda" for zero before the `< 10`
/// branch. (The float path *does* use `ones[0]`, hence the `or "iqanda"`
/// fallback in [`LangZu::str_cardinal`].)
const ONES: [&str; 10] = [
    "",
    "kunye",
    "kubili",
    "kuthathu",
    "kune",
    "kuhlanu",
    "isithupha",
    "isikhombisa",
    "isishiyagalombili",
    "isishiyagalolunye",
];

/// `self.tens`. Index 0 is `""` and unreachable (`number < 100` implies
/// `t >= 1` there, since `number < 10` was already handled).
const TENS: [&str; 10] = [
    "",
    "ishumi",
    "amashumi amabili",
    "amashumi amathathu",
    "amashumi amane",
    "amashumi amahlanu",
    "amashumi ayisithupha",
    "amashumi ayisikhombisa",
    "amashumi ayisishiyagalombili",
    "amashumi ayisishiyagalolunye",
];

const ZERO_WORD: &str = "iqanda";
const HUNDRED: &str = "ikhulu";
const THOUSAND: &str = "inkulungwane";
const MILLION: &str = "isigidi";
const NEGWORD: &str = "ngaphansi kwe ";
const POINTWORD: &str = "ichashazi";
const JOINER: &str = " na ";
const ORDINAL_ONE: &str = "okokuqala";
const ORDINAL_PREFIX: &str = "we-";

/// The code `to_currency` falls back to for anything not in `CURRENCY_FORMS`.
///
/// Python writes the fallback positionally, not by name:
///
/// ```text
/// self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
/// ```
///
/// `list(dict.values())[0]` is the **first-inserted** entry under Python 3.7+
/// insertion-ordered dicts, and ZU's class body inserts ZAR, then USD, then
/// EUR. Pinning the name here rather than relying on `HashMap` iteration order
/// is the whole point: `HashMap` has no order to borrow, so a positional
/// transcription would be nondeterministic. Live-checked —
/// `list(CURRENCY_FORMS.keys()) == ["ZAR", "USD", "EUR"]`.
///
/// **Not the same thing as `default_currency()`**, even though both are "ZAR"
/// here. `default_currency()` is the signature default — what `currency`
/// becomes when the caller omits the kwarg (`to_currency(val,
/// currency="ZAR")`). `FALLBACK_CODE` is what a code the caller *did* pass but
/// that is missing from the table resolves to. They are independent: reordering
/// the class-body dict would move this one and leave the signature alone. Do
/// not collapse them.
const FALLBACK_CODE: &str = "ZAR";

pub struct LangZu {
    /// `self.exclude_title`. Inert: `is_title` is never set true by ZU, so
    /// `Num2Word_Base.title` returns its input untouched. Carried for fidelity.
    exclude_title: Vec<String>,
    /// `Num2Word_ZU.CURRENCY_FORMS`, built once here and only ever read.
    /// Rebuilding it per `to_currency` call is what made an earlier revision
    /// of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangZu {
    fn default() -> Self {
        Self::new()
    }
}

impl LangZu {
    pub fn new() -> Self {
        // `Num2Word_ZU.CURRENCY_FORMS`, verbatim. Both sides of every entry
        // carry two identical forms — Zulu does not inflect these nouns for
        // number — but the arity is load-bearing: `to_currency` indexes
        // `cr1[1]`/`cr2[1]` unconditionally when the count is not 1, so
        // collapsing a pair to one form would turn a plural into an
        // IndexError.
        const SUBUNIT: [&str; 2] = ["isenti", "isenti"];
        let mut currency_forms: HashMap<&'static str, CurrencyForms> = HashMap::new();
        currency_forms.insert("ZAR", CurrencyForms::new(&["randi", "randi"], &SUBUNIT));
        currency_forms.insert("USD", CurrencyForms::new(&["idola", "idola"], &SUBUNIT));
        currency_forms.insert("EUR", CurrencyForms::new(&["iyuro", "iyuro"], &SUBUNIT));

        LangZu {
            exclude_title: ["na", "ichashazi", "ngaphansi", "kwe"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            currency_forms,
        }
    }

    /// Python's `_int_to_word`. `number` is always non-negative: the only
    /// in-scope caller is `to_cardinal`, which strips the sign first.
    ///
    /// The ladder is a direct transcription — note that each scale delegates
    /// its remainder back through the *full* ladder, so the hundreds inside a
    /// thousands group re-enter at the `< 1000` branch.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        let ten = BigInt::from(10);
        if number < &ten {
            // Safe: 0 < number < 10.
            let d = digit(number);
            return ONES[d].to_string();
        }

        let hundred = BigInt::from(100);
        if number < &hundred {
            let (t, o) = number.div_rem(&ten);
            let (t, o) = (digit(&t), digit(&o));
            let mut out = TENS[t].to_string();
            if o != 0 {
                out.push_str(JOINER);
                out.push_str(ONES[o]);
            }
            return out;
        }

        let thousand = BigInt::from(1000);
        if number < &thousand {
            let (h, r) = number.div_rem(&hundred);
            let mut out = String::new();
            // Python: `(self.ones[h] + " " if h > 1 else "") + self.hundred`
            if h > BigInt::one() {
                out.push_str(ONES[digit(&h)]);
                out.push(' ');
            }
            out.push_str(HUNDRED);
            if !r.is_zero() {
                out.push_str(JOINER);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        let million = BigInt::from(1_000_000);
        if number < &million {
            let (t, r) = number.div_rem(&thousand);
            let mut out = String::new();
            if t > BigInt::one() {
                out.push_str(&self.int_to_word(&t));
                out.push(' ');
            }
            out.push_str(THOUSAND);
            if !r.is_zero() {
                out.push_str(JOINER);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        let billion = BigInt::from(1_000_000_000);
        if number < &billion {
            let (m, r) = number.div_rem(&million);
            let mut out = String::new();
            if m > BigInt::one() {
                out.push_str(&self.int_to_word(&m));
                out.push(' ');
            }
            out.push_str(MILLION);
            if !r.is_zero() {
                out.push_str(JOINER);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // Bug 1 above: `return str(number)` — digits, not words, no raise.
        number.to_string()
    }

    /// Python's `to_cardinal` on an already-stringified number — the shape
    /// the float/Decimal path actually runs:
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "iqanda")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The recursion for negatives re-enters with the sign-stripped
    /// *substring*, so a `ValueError` raised deeper up (scientific notation,
    /// bug 10) quotes the literal without its minus — `-1e-05` fails on
    /// `'1e-05'`. The trailing `.strip()`s are kept though they never bite:
    /// `negword` supplies the only interior spacing and every fragment is
    /// non-empty.
    fn str_cardinal(&self, n: &str) -> Result<String> {
        let n = n.trim();
        if let Some(rest) = n.strip_prefix('-') {
            let body = self.str_cardinal(rest)?;
            return Ok(format!("{}{}", NEGWORD, body).trim().to_string());
        }
        if let Some((left, right)) = n.split_once('.') {
            // `int(left)` — the huge-value digit fallthrough of int_to_word
            // (bug 1) applies to the integer part exactly as in Python:
            // 1234567890.5 → "1234567890 ichashazi kuhlanu".
            let mut ret = format!("{} {}", self.int_to_word(&py_int(left)?), POINTWORD);
            for ch in right.chars() {
                if !ch.is_ascii_digit() {
                    // `int(digit)` on the offending character — Python
                    // quotes just that one char: `int("e")` → `... 'e'`.
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    )));
                }
                let d = (ch as u8 - b'0') as usize;
                ret.push(' ');
                // `self.ones[int(digit)] or "iqanda"` — ones[0] is "" and
                // falsy, so zero digits render as the zero word.
                ret.push_str(if d == 0 { ZERO_WORD } else { ONES[d] });
            }
            return Ok(ret.trim().to_string());
        }
        Ok(self.int_to_word(&py_int(n)?))
    }
}

/// Extract a 0..=9 `BigInt` as a `usize` index.
///
/// Every call site has already bounded the value below 10 (or is a `div_rem`
/// quotient/remainder that is), so the conversion cannot fail. Uses the
/// decimal string rather than `ToPrimitive` to avoid widening the import set.
fn digit(n: &BigInt) -> usize {
    n.to_string().parse::<usize>().unwrap_or(0)
}

/// Python's `int(s)` for the strings this module can produce.
///
/// The reachable inputs are pure ASCII digit runs (success) or repr/str
/// spellings carrying `e`/`E`/`+` exponent syntax (ValueError, quoting the
/// whole literal — `int("1e-05")` → `invalid literal for int() with base 10:
/// '1e-05'`). Signs never reach here: `to_cardinal` peels a leading `-`
/// before any `int()` call, and repr never emits an interior `+` outside the
/// exponent forms that fail anyway. Python's wider `int()` grammar
/// (whitespace, underscores, non-ASCII digits) is unreachable from
/// `str(float)`/`str(Decimal)` output, so it is not modelled.
fn py_int(s: &str) -> Result<BigInt> {
    if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
        Ok(s.parse().expect("pure digit string parses"))
    } else {
        Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        )))
    }
}

/// Python's `repr(float)` (== `str(float)`) for a finite f64.
///
/// ZU's `to_cardinal` works on `str(number)`, so the exact spelling — where
/// the notation flips to scientific, the `.0` on whole floats, the two-digit
/// zero-padded exponent — is load-bearing. Rust's `{}` shares the
/// shortest-round-trip digits contract with CPython's repr but never uses
/// scientific notation, drops the `.0`, **and resolves exact-tie shortest
/// candidates differently**, so the string is rebuilt here:
///
/// * CPython (`format_float_short`, mode `'r'`) goes scientific when the
///   decimal-point position `decpt` is `<= -4` or `> 16`: `repr(0.0001)` ==
///   `'0.0001'` but `repr(0.00001)` == `'1e-05'`; `repr(1e16)` == `'1e+16'`
///   but `repr(9999999999999998.0)` keeps all 16 digits. `decpt` comes from
///   `{:e}`'s exponent (the shortest-length contract is shared, so the
///   digit *count* — and hence the regime — always agrees).
/// * **Positional regime — the tie trap.** When a float sits exactly
///   halfway between the two shortest candidates (coarse ulp, e.g.
///   `980819185788849.25` between `…849.2` and `…849.3`, both of which
///   round-trip), CPython's dtoa rounds half-to-EVEN (`…849.2`) while
///   Rust's `{}`/`{:e}` shortest digits land on `…849.3`. Randomized
///   cross-validation against the live interpreter caught four of these in
///   1 500 samples, so this is not theoretical. The fix: format with
///   `{:.prec$}`, whose fixed mode rounds the *exact* binary expansion
///   half-to-even (verified: `{:.1}` of `0.25` → `"0.2"`, of
///   `980819185788849.25` → `"…849.2"`) — exactly repr's rounding at
///   repr's length. `prec` is the `FloatValue` precision field, computed on
///   the Python side as `abs(Decimal(str(value)).as_tuple().exponent)`,
///   i.e. repr's own fraction-digit count; `max(prec, 1)` restores the
///   mandatory `.0` of whole floats (`str(1.0)` == `"1.0"` — the corpus
///   `1.0` → `"kunye ichashazi iqanda"` row) should a whole value ever
///   arrive with precision 0. `{:.prec$}` also keeps `-0.0`'s sign.
/// * Scientific regime: `d[.ddd]e±XX`, exponent `%+03d`-shaped — sign
///   always present, at least two digits (`1e-05`, `1e+16`, `5e-324`).
///   Ties cannot arise here: an exact decimal midpoint at the last shortest
///   digit needs an ulp on the order of that digit's place, which only
///   coarse large-magnitude floats have — sub-1e-4 floats have ulps many
///   orders finer than their shortest digit grid. (For `decpt > 16` the
///   value is whole, which the dispatcher never routes here at all.)
fn py_float_repr(f: f64, prec: u32) -> String {
    // Only finite values can arrive: the dispatcher routes inf/nan down the
    // Python error path before the Rust call.
    let sci = format!("{:e}", f);
    let (mantissa, exp) = sci.split_once('e').expect("{:e} always contains e");
    let exp: i32 = exp.parse().expect("{:e} exponent is an integer");
    // Position of the decimal point relative to the start of the shortest
    // digit string (CPython's decpt): value == 0.<digits> * 10^decpt.
    let decpt = exp + 1;

    if !(decpt <= -4 || decpt > 16) {
        // Positional: fixed formatting at repr's own fraction length, which
        // rounds the exact value half-to-even just like CPython's dtoa.
        return format!("{:.*}", prec.max(1) as usize, f);
    }

    // Scientific: d[.ddd]e±XX with the exponent zero-padded to 2 digits.
    let neg = mantissa.starts_with('-');
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let mut s = String::new();
    if neg {
        s.push('-');
    }
    s.push_str(&digits[..1]);
    if digits.len() > 1 {
        s.push('.');
        s.push_str(&digits[1..]);
    }
    let e10 = decpt - 1;
    s.push('e');
    s.push(if e10 < 0 { '-' } else { '+' });
    s.push_str(&format!("{:02}", e10.abs()));
    s
}

/// Python's `str(Decimal)`, reconstructed from the parsed `BigDecimal`.
///
/// The binding sends `str(number)` and `BigDecimal::from_str` keeps the
/// coefficient and scale exactly as written (trailing zeros and `E±n`
/// exponents included), so CPython's `Decimal.__str__` algorithm — sign,
/// coefficient digits, exponent — round-trips the original spelling:
/// `"1.10"` stays `"1.10"`, `"0.0000001"` (arriving as `"1E-7"`) renders
/// back to `"1E-7"`, `"1.05E-7"` to `"1.05E-7"`. Positional iff
/// `_exp <= 0 and leftdigits > -6`; otherwise one digit left of the point
/// and an `E%+d` suffix (no zero padding, unlike float repr — verified live:
/// the ValueError literal for `Decimal("0.0000001")` is `'1E-7'`).
///
/// **One unreproducible spelling: negative zero.** `str(Decimal("-0.0"))`
/// keeps the sign, so Python emits the negword (`"ngaphansi kwe iqanda
/// ichashazi iqanda"`), but `BigInt` cannot carry a sign on zero and the
/// discriminator is destroyed by the `BigDecimal` parse one layer above this
/// file — same class of hole as the `e-` currency case below. This port
/// renders `Decimal("-0.0")` unsigned. The float `-0.0` is unaffected: f64
/// keeps its sign bit and `{:.prec$}` prints it. No corpus row exercises a
/// negative-zero Decimal.
fn py_decimal_str(d: &BigDecimal) -> String {
    // value == bigint * 10^-scale; Python's Decimal._exp is -scale.
    let (bigint, scale) = d.as_bigint_and_exponent();
    let sign = if bigint.is_negative() { "-" } else { "" };
    let digits = bigint.abs().to_string();
    let exp = -scale;
    let leftdigits = exp + digits.len() as i64;
    // eng=False branch of CPython's __str__: dotplace 1 when scientific.
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), digits),
        )
    } else if dotplace as usize >= digits.len() {
        (
            format!("{}{}", digits, "0".repeat(dotplace as usize - digits.len())),
            String::new(),
        )
    } else {
        (
            digits[..dotplace as usize].to_string(),
            format!(".{}", &digits[dotplace as usize..]),
        )
    };
    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — sign always, no zero padding, capital E (capitals=1).
        format!("E{:+}", leftdigits - dotplace)
    };
    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}

/// Python's inline currency split, which is a **string** operation on
/// `str(abs(val))` rather than an arithmetic one:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `to_currency` has already taken `abs` by this point; it is re-applied here
/// so the stringification and the split stay in one place.
///
/// Four consequences, all corpus-pinned:
///
/// * `right` comes from *right-padding the first two fraction digits*, not
///   from multiplying: `0.5` → `"5"` → `"50"` → 50 amasenti, while `0.05` →
///   `"05"` → 5. Anything past the second decimal is truncated with no
///   rounding at all, so `12.349` → 34 (not 35) and `0.005` → 0 (not 1).
/// * A fraction of `"0"` (i.e. `1.0`) yields `right == 0`, which `to_currency`
///   treats as falsy and drops the cents clause entirely: `1.0` → `"kunye
///   iyuro"`, not `"...iqanda isenti"`. This is the one point where ZU's float
///   path coincides with base's int-only path, and it is reached by ZU's own
///   route rather than by base's `has_decimal` guard — which is why
///   `CurrencyValue::Decimal`'s `has_decimal` flag is unused here.
/// * `Decimal("5")` (scale 0, no `"."` in `str`) and `Decimal("5.00")` (scale
///   2, `right == 0`) both land on `"kuhlanu idola"` — by the two different
///   routes above, so the scale carries everything `has_decimal` would.
/// * A `CurrencyValue::Int` stringifies without a `"."`, so `parts` has length
///   1 and `right` stays 0.
///
/// # The exponent-notation hole
///
/// `str(float)` flips to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal:
/// `to_currency(1e16)` raises `ValueError: invalid literal for int() with base
/// 10: '1e+16'`. A negative `BigDecimal` scale is exactly the "the source
/// string used `e+` notation" signal — a plain digit string parses to scale 0,
/// never below — so that arm is reproduced faithfully.
///
/// The `e-` side is **not** reproducible from here, and is flagged in the port
/// report: Python raises `ValueError` for `1e-05` but returns `"iqanda idola"`
/// for `Decimal("0.00001")`. The two are distinguishable *in principle* — the
/// shim sends `str(value)`, and those strings differ (`"1e-05"` vs
/// `"0.00001"`) — but `BigDecimal::from_str` normalises both to digits 1 /
/// scale 5, and `CurrencyValue::Decimal` retains only the parsed value, not the
/// literal it came from. So the discriminator is destroyed by the parse, one
/// layer above this file. Fixing it means giving `CurrencyValue` the original
/// spelling (a `currency.rs` change affecting every language that transcribes
/// this same `str(val).split(".")` idiom — 63 of the Python modules do), which
/// is out of scope for a single-language port. This port takes the `Decimal`
/// reading for both. Corpus has no `e-` row, so nothing regresses today.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let d = match val {
        // str(int) never contains ".", so parts == [digits] and right == 0.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    // value == digits * 10^-scale
    let (digits, scale) = d.as_bigint_and_exponent();

    if scale < 0 {
        // str(d) would be "1e+16"-shaped and int() rejects it. Python quotes
        // the offending literal, which is unrecoverable from the parsed value;
        // the exception *type* is what callers observe.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            d
        )));
    }

    // No "." in str(d): parts == [str(d)], so right stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    // `d` is non-negative, so this is a bare ASCII digit string.
    let s = digits.abs().to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

impl Lang for LangZu {

    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("comma nulla"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`. ZU's `to_ordinal` compares the raw value
    /// first — `number == 1` is *numeric* equality, so the whole float 1.0
    /// (and `Decimal("1.0")`) hits the special form "okokuqala". Everything
    /// else is `"we-" + self.to_cardinal(number)`, i.e. the float string
    /// grammar with the prefix: "we-kuhlanu ichashazi iqanda".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let is_one = match value {
            FloatValue::Float { value, .. } => *value == 1.0,
            FloatValue::Decimal { value, .. } => *value == BigDecimal::from(1),
        };
        if is_one {
            return Ok(ORDINAL_ONE.to_string());
        }
        Ok(format!(
            "{}{}",
            ORDINAL_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `"we-" + str(number)` — the repr the
    /// binding computed, prefix glued on, sign and exponent form included
    /// ("we--0.0", "we-1e+16").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside ZU's `int("Infinity")` (`to_cardinal` recurses
    /// past the sign, finds no "." and calls `int()`). The binding otherwise
    /// hard-codes `ParsedNumber::Inf` to the base integer path's
    /// OverflowError before any ZU code runs, so the ValueError must be
    /// raised here. The sign is stripped by the recursion before `int()`
    /// sees it, so both signs quote the same literal.
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
        "ZAR"
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
        "ichashazi"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)` and re-enters itself with the
    /// sign-stripped substring (`self.to_cardinal(n[1:])`). For integral input
    /// that recursion is exactly `int_to_word(abs(n))`: `n[1:]` of a decimal
    /// literal parses back to the absolute value, and the `"." in n` branch is
    /// unreachable. The trailing `.strip()` is preserved though it never bites
    /// — `int_to_word` yields no leading or trailing space.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let body = self.int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, body).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// Python's `to_ordinal`. No `verify_ordinal`, so 0 and negatives pass
    /// straight through into the `"we-"` + cardinal form (bug 3).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok(ORDINAL_ONE.to_string());
        }
        Ok(format!("{}{}", ORDINAL_PREFIX, self.to_cardinal(value)?))
    }

    /// Python's `to_ordinal_num`: `"we-" + str(number)`, sign and all,
    /// hence `"we--1"` for -1 (bug 4).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, value))
    }

    /// Python's `to_year(val, longval=True)` discards `longval` and defers
    /// wholesale to `to_cardinal` — no century splitting, no era suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// ZU's float/Decimal path: `Num2Word_ZU.to_cardinal` reached with a
    /// non-integral number. It is pure string surgery on `str(number)` — no
    /// `float2tuple`, no rounding, no zero-padding to a precision — so the
    /// job here is to rebuild Python's exact spelling of the value
    /// ([`py_float_repr`] / [`py_decimal_str`]) and run the transcribed
    /// string logic over it ([`LangZu::str_cardinal`]).
    ///
    /// `precision_override` is ignored: ZU never reads `self.precision`
    /// (bug 11 / header note), so `num2words(0.5, lang="zu", precision=3)`
    /// is just `0.5`. The `Float` arm's repr-derived `precision` *field* is
    /// consumed by [`py_float_repr`] — not to pad or round the output the
    /// way base's float path would, but to pin repr's fraction length for
    /// the half-even tie fix documented there. The `Decimal` arm ignores
    /// its precision field entirely; the parsed value carries its scale.
    ///
    /// The `Float`/`Decimal` split stays load-bearing even without
    /// arithmetic: the two arms *stringify differently*.
    /// `num2words(0.0000001)` raises ValueError (`repr` says `'1e-07'`)
    /// while `num2words(Decimal("0.0000001"))` raises on `'1E-7'`, and
    /// `Decimal("1.10")` keeps its trailing zero where the float `1.1`
    /// would not.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let n = match value {
            FloatValue::Float { value, precision } => py_float_repr(*value, *precision),
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        self.str_cardinal(&n)
    }

    // ---- currency -------------------------------------------------------
    //
    // ZU overrides `to_currency` and `pluralize`. Everything else on the
    // currency path — `to_cheque`, `_money_verbose`, `_cents_verbose`,
    // `_cents_terse` — is inherited from `Num2Word_Base` verbatim, and the
    // trait defaults already mirror it, so only the data table, the class
    // name and those two methods appear here.
    //
    // `currency_precision` and `currency_adjective` are deliberately absent:
    // ZU inherits both dicts *empty* from the base class, so
    // `CURRENCY_PRECISION.get(code, 100)` is always 100 (the trait default)
    // and `currency in self.CURRENCY_ADJECTIVES` is never true. There is no
    // 3-decimal or 0-decimal currency here — `to_currency("KWD")` does not
    // reach a divisor of 1000, it reaches ZAR's forms at divisor 100.

    fn lang_name(&self) -> &str {
        "Num2Word_ZU"
    }

    /// The **strict** lookup, `self.CURRENCY_FORMS[currency]`.
    ///
    /// This hook feeds the inherited `to_cheque`, which is the only caller
    /// that indexes the dict directly and raises NotImplementedError on a
    /// miss. ZU's own `to_currency` does *not* go through here — it uses the
    /// forgiving `.get(..., <first value>)` and never raises — so this must
    /// stay strict or the cheque path would silently emit ZAR for unknown
    /// codes instead of raising.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_ZU.pluralize`:
    ///
    /// ```text
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Note `forms[-1]` — the *last* form, not `forms[1]` the way
    /// `Num2Word_EUR.pluralize` does it. For ZU's own two-form entries the two
    /// readings coincide, but the empty guard and the negative index are both
    /// transcribed as written: this never raises, where the base class's
    /// abstract `pluralize` and EUR's indexing version both can.
    ///
    /// Dead on every in-scope path — ZU's `to_currency` indexes `cr1`/`cr2`
    /// itself and the inherited `to_cheque` takes `cr1[-1]` directly, so
    /// nothing calls this. Ported anyway: it is a real method on the class.
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

    /// Python's `Num2Word_ZU.to_currency`.
    ///
    /// A wholesale override that shares nothing with `Num2Word_Base`'s: no
    /// `parse_currency_parts`, no `pluralize`, no `_money_verbose`, no
    /// rounding, no `has_decimal` guard, no NotImplementedError. It splits
    /// `str(abs(val))` on `"."` and indexes the forms tuple by hand.
    ///
    /// `adjective` is accepted and then ignored — ZU never reads it, and its
    /// `CURRENCY_ADJECTIVES` is empty anyway, so there is nothing to prefix.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `None` means the caller omitted the kwarg; the trait's
        // `default_separator` carries ZU's own `separator=" "` default, read
        // from the live signature.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is evaluated *before* `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code silently borrows ZAR's forms rather than raising (bug 7).
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CODE))
            .expect("FALLBACK_CODE is inserted by new()");

        // `cr1[1] if left != 1 else cr1[0]` — a direct index, not pluralize().
        let unit = if left.is_one() {
            &forms.unit[0]
        } else {
            &forms.unit[1]
        };
        // `self._int_to_word(left)`, not `to_cardinal`: past 10^9 this is a
        // bare digit string (bug 8).
        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `if cents and right:` — a zero cents count is falsy, so the clause
        // vanishes rather than rendering "iqanda isenti".
        if cents && !right.is_zero() {
            let subunit = if right.is_one() {
                &forms.subunit[0]
            } else {
                &forms.subunit[1]
            };
            // The separator is concatenated raw, carrying no space of its own:
            // an explicit `separator=","` renders "...idola,amashumi...".
            result.push_str(&format!(
                "{}{} {}",
                separator,
                self.int_to_word(&right),
                subunit
            ));
        }

        if is_negative {
            // Raw concatenation of `negword`, which already ends in a space —
            // not base's `"%s " % negword.strip()`.
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}
