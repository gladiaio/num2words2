//! Port of `lang_TI.py` (Tigrinya).
//!
//! # Deliberate divergence from upstream: Ge'ez script
//!
//! Upstream `lang_TI.py` spells every numeral in **Latin transliteration**
//! ("ḥade", "mi'ti", "shiḥ"). Tigrinya is written in Ge'ez, and the sibling
//! Ethiopic language in this crate (`lang_am.rs`, Amharic) already emits Ge'ez,
//! so the transliteration was an upstream accident rather than a choice. The
//! word tables here are the Ge'ez spellings: "ሓደ", "ሚእቲ", "ሽሕ".
//!
//! This is the **one** place this port knowingly departs from byte-for-byte
//! upstream fidelity, and it is a lexicon change only — the structure below
//! (the composition rules, the `> 1` multiplier guards, the billion cliff, the
//! missing `verify_ordinal`, the currency fallback) is still ported verbatim,
//! bug for bug. The frozen-corpus fixtures in this file were re-spelled to
//! match; every other assertion they make is unchanged.
//!
//! Two consequences worth naming, both inherited structure rather than new
//! behaviour:
//!
//! * **Ordinals are still suffixal, not suppletive.** Upstream builds every
//!   ordinal as `to_cardinal(n) + "ay"`, so `to_ordinal(1)` is "ሓደኣይ" — the
//!   mechanical form, not the idiomatic Tigrinya "ቀዳማይ". Fixing that means
//!   replacing the algorithm, not the script, so it is left alone here.
//! * **The billion cliff still emits ASCII digits.** Above 10^9 upstream
//!   returns `str(number)`; those are Western digits, not Ge'ez numerals, and
//!   that is untouched.
//!
//! Shape: **self-contained**. `Num2Word_TI` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the guard in
//! `Num2Word_Base.__init__` never fires: Python builds **no** `self.cards` and
//! never sets `self.MAXVAL` (verified: `hasattr(n, "cards") is False`).
//! `to_cardinal` is overridden outright and drives `_int_to_word`, so
//! `cards`/`maxval`/`merge` stay at their trait defaults here and `splitnum`/
//! `clean`/`merge` are unreachable. There is **no overflow check** at any size.
//!
//! Inherited from `Num2Word_Base` and left alone by TI:
//!   * `is_title` stays `False`, so `title()` is the identity. TI's
//!     `to_cardinal` never calls `title()` anyway.
//!
//! Every method in scope is overridden by TI, so nothing here relies on a
//! base-class default.
//!
//! # Structure
//!
//! Tigrinya composes with a single connector `" ን "` at every level, and
//! suppresses the multiplier word when it would be one ("ሓደ"): 100 is
//! "ሚእቲ" (not "ሓደ ሚእቲ"), 1000 is "ሽሕ", 10^6 is "ሚልዮን". Note the
//! `> 1` guard is present on the millions branch too, unlike some sibling
//! languages (e.g. `lang_PAP`) that only guard the thousands.
//!
//! Teens are not lexicalised: 11 is "ዓሰርተ ን ሓደ" (literally "ten and
//! one"), built by the generic `< 100` branch. That is the Python behaviour,
//! not an omission.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following are all wrong-looking but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **The billion cliff.** `_int_to_word` has no branch above 10^9 and ends
//!    with `return str(number)`, so any `abs(n) >= 10**9` falls out as raw
//!    ASCII digits instead of words: `to_cardinal(10**9) == "1000000000"` and
//!    `to_cardinal(10**21) == "1000000000000000000000"`. No exception, no
//!    words — the digits are the output. Modelled in [`LangTi::int_to_word`].
//! 2. The cliff leaks into the other modes: `to_ordinal(10**9)` is
//!    `"1000000000ኣይ"`, which is also exactly what `to_ordinal_num(10**9)`
//!    returns — the two modes silently converge above the cliff.
//! 3. `to_ordinal` never calls `verify_ordinal`, so zero and negatives pass
//!    straight through and get suffixed: `to_ordinal(0) == "ባዶኣይ"`,
//!    `to_ordinal(-1) == "ኣሉታ ሓደኣይ"`. The suffix lands on the *last
//!    word* of a multi-word cardinal, e.g. `to_ordinal(11)` is
//!    `"ዓሰርተ ን ሓደኣይ"`.
//! 4. `to_year` ignores its `longval` parameter entirely and delegates to
//!    `to_cardinal`, so there is no BC/AD handling and no year-pairing:
//!    `to_year(1999)` is the plain cardinal, and `to_year(-500)` is
//!    `"ኣሉታ ሓሙሽተ ሚእቲ"` ("negative five hundred") rather than
//!    anything resembling "500 BC".
//!
//! # Currency
//!
//! `Num2Word_TI` subclasses `Num2Word_Base` *directly* (MRO verified:
//! `Num2Word_TI -> Num2Word_Base -> object`), so it never sees the
//! `Num2Word_EUR` class dict that `Num2Word_EN.__init__` mutates at import
//! time. Its `CURRENCY_FORMS` is its own four-entry class attribute and is
//! read verbatim.
//!
//! TI overrides `to_currency` **wholesale** and shares almost none of Base's
//! currency machinery. The consequences are all observable:
//!
//! 1. **`to_currency` never raises `NotImplementedError`.**
//!    `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])` falls
//!    back to the *first inserted* entry — ETB — for every unknown code. So
//!    `to_currency(0, "GBP")` is "ባዶ ብር", and JPY/KWD/BHD/INR/CNY/CHF all
//!    silently render as Ethiopian ብር. `to_cheque` is Base's and *does*
//!    raise, which is why the corpus has `cheque:GBP` erroring while
//!    `currency:GBP` succeeds.
//! 2. **No precision table.** `CURRENCY_PRECISION` is `{}`, so `.get(code,100)`
//!    is always 100: KWD/BHD get 2 decimals rather than 3, and JPY still
//!    renders subunits (`to_currency(12.34,"JPY")` ends "…ሰላሳ ን ኣርባዕተ
//!    ሳንቲም"). Both the 1000- and 1-divisor branches are dead for TI.
//! 3. **No `pluralize`, no adjective.** `to_currency` indexes the form tuple
//!    inline (`cr1[1] if left != 1 else cr1[0]`) and accepts `adjective`
//!    without ever reading it. `CURRENCY_ADJECTIVES` is `{}` anyway.
//! 4. **Cents truncate, they do not round.** `right` is built from the first
//!    two *characters* after the "." in `str(val)`, so 2.675 is 67 ሳንቲም
//!    (ROUND_HALF_UP would say 68), 12.345 is 34, and 0.005 truncates to 0 —
//!    which, being falsy, drops the cents segment entirely ("ባዶ ዩሮ").
//! 5. **`left`/`right` go through `_int_to_word`, not `to_cardinal`**, so the
//!    billion cliff applies to the units too: `to_currency(1e15)` is
//!    "1000000000000000 ዩሮ" (digits, verified against the interpreter).
//!
//! Base's `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse`
//! are all inherited unchanged, so their trait defaults already match; only the
//! data tables, the class name, `pluralize` and `to_currency` are overridden.
//!
//! # Float / Decimal cardinal path
//!
//! `pointword` ("ነጥቢ") and the `"." in n` branch of `to_cardinal` are the
//! float *cardinal* path. TI overrides `to_cardinal` (not `to_cardinal_float`)
//! and handles non-integers **inline on the string** `str(number)`, so the port
//! overrides [`Lang::to_cardinal_float`] to reconstruct that exact string
//! ([`python_str`]) and run TI's split-and-per-digit logic
//! ([`LangTi::cardinal_from_str`]). Neither `PORTING_FLOAT.md` trap applies: TI
//! never calls `float2tuple`, so there is no banker's-`round()` and no
//! `abs(value-pre)*10**precision` f64 artefact to preserve — the fraction digits
//! are read straight off the repr. `precision=` is dropped by the dispatcher
//! before `to_cardinal` is reached, so `precision_override` is ignored.
//!
//! `cardinal_from_decimal` (Base's fractional-cents entry point) stays at its
//! default. Nothing reaches it: TI's `to_currency` never calls it, and
//! `default_to_currency` (its only other caller) is unreachable because
//! `to_currency` is overridden here.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `setup(): self.ones`. Index 0 is the empty string, exactly as in Python.
/// The `< 10` branch is guarded by the zero test above it, so index 0 is only
/// ever reachable via the (out-of-scope) decimal-digit loop, which maps it to
/// "ባዶ" instead.
const ONES: [&str; 10] = [
    "",
    "ሓደ",
    "ክልተ",
    "ሰለስተ",
    "ኣርባዕተ",
    "ሓሙሽተ",
    "ሽዱሽተ",
    "ሸውዓተ",
    "ሸሞንተ",
    "ትሽዓተ",
];

/// `setup(): self.tens`. Index 0 is unreachable (`number >= 10` in the only
/// branch that reads this table).
const TENS: [&str; 10] = [
    "",
    "ዓሰርተ",
    "ዕስራ",
    "ሰላሳ",
    "ኣርብዓ",
    "ሓምሳ",
    "ስሳ",
    "ሰብዓ",
    "ሰማንያ",
    "ተስዓ",
];

const ZERO_WORD: &str = "ባዶ";
const HUNDRED: &str = "ሚእቲ";
const THOUSAND: &str = "ሽሕ";
const MILLION: &str = "ሚልዮን";

/// `setup(): self.negword`. The trailing space is part of the Python literal.
const NEGWORD: &str = "ኣሉታ ";
/// `setup(): self.pointword`. Float path only — unused in scope.
const POINTWORD: &str = "ነጥቢ";

/// The connector joining every level: "ten *and* one", "hundred *and* five".
const AND: &str = " ን ";

/// `to_ordinal` / `to_ordinal_num` suffix.
const ORDINAL_SUFFIX: &str = "ኣይ";

/// `Num2Word_TI.CURRENCY_FORMS`, in Python's class-body **insertion order**.
///
/// The order is load-bearing and this must stay a sequence rather than an
/// unordered literal: `to_currency` falls back to
/// `list(self.CURRENCY_FORMS.values())[0]` for unknown codes, which since
/// Python 3.7 is the first *inserted* entry. Verified against the live
/// interpreter — `list(c.CURRENCY_FORMS.values())[0]` is ETB's ብር/ሳንቲም
/// pair — and against the corpus, where every unimplemented code renders as
/// ብር.
///
/// Each side carries exactly two forms, matching the Python tuples. TI's
/// singular and plural are identical throughout ("ብር"/"ብር"), so the
/// `left != 1` branch is invisible in the output — but the arity is what the
/// inline `cr1[1]` index depends on, so it is kept exact.
const CURRENCY_FORMS: [(&str, [&str; 2], [&str; 2]); 4] = [
    ("ETB", ["ብር", "ብር"], ["ሳንቲም", "ሳንቲም"]),
    ("ERN", ["ናቕፋ", "ናቕፋ"], ["ሳንቲም", "ሳንቲም"]),
    ("USD", ["ዶላር", "ዶላር"], ["ሰንት", "ሰንት"]),
    ("EUR", ["ዩሮ", "ዩሮ"], ["ሰንት", "ሰንት"]),
];

/// Python's `str(val).split(".")` keeps at most the first two fractional
/// characters (`parts[1][:2].ljust(2, "0")`), i.e. hundredths.
const CENTS_PER_UNIT: i64 = 100;

pub struct LangTi {
    /// `setup(): self.exclude_title`. Dead data in Python (`is_title` is
    /// `False`), retained so the trait method reports what Python holds.
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS` as a lookup. Built once in [`LangTi::new`] and only
    /// ever read afterwards — rebuilding it per call is what made an earlier
    /// revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the ETB entry, precomputed.
    ///
    /// Held separately because [`Lang::currency_forms`] must keep returning
    /// `None` for unknown codes (Base's `to_cheque` relies on that to raise
    /// `NotImplementedError`), while `to_currency` must instead substitute
    /// this. The two lookups genuinely disagree, so they cannot share a path.
    fallback_forms: CurrencyForms,
}

impl Default for LangTi {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTi {
    pub fn new() -> Self {
        let (_, fallback_unit, fallback_subunit) = CURRENCY_FORMS[0];
        LangTi {
            exclude_title: vec![
                "ናይ".to_string(),
                "ነጥቢ".to_string(),
                "ኣሉታ".to_string(),
            ],
            currency_forms: CURRENCY_FORMS
                .iter()
                .map(|(code, unit, subunit)| (*code, CurrencyForms::new(unit, subunit)))
                .collect(),
            fallback_forms: CurrencyForms::new(&fallback_unit, &fallback_subunit),
        }
    }

    /// Python's `str(val).split(".")` surgery, the head of `to_currency`:
    ///
    /// ```text
    /// val   = abs(val)
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// TI is the one language that runs `str(val)` *inside* `to_currency` and
    /// then does text surgery on it, but the shim parses that string into a
    /// `BigDecimal` at the boundary and the text is gone before this runs. So
    /// the split is reconstructed arithmetically. For every value whose `str()`
    /// is plain decimal notation the two are provably identical:
    ///
    /// * `parts[0]` is the integer part, so `left` is `trunc(abs(val))`; abs
    ///   has already been taken, so truncation and floor agree.
    /// * `parts[1][:2].ljust(2,"0")` takes the first two fractional digits and
    ///   right-pads, which is exactly `trunc(frac * 100)`: "5" -> "50" -> 50,
    ///   "01" -> 1, "345" -> "34" -> 34.
    /// * a `str()` with no "." parses to scale <= 0, whose `frac` is 0, giving
    ///   `right == 0` — the same answer as Python's `len(parts) > 1` guard.
    ///   That covers both `int` input and `Decimal("5")`, and `Decimal("5.00")`
    ///   lands on `int("00") == 0` either way.
    ///
    /// Consequently `has_decimal` is *not* consulted: TI branches on the text
    /// of `str(val)`, never on `isinstance`, so int `5`, `Decimal("5")`,
    /// `Decimal("5.00")` and `5.0` all collapse to the same "ሓሙሽተ ዩሮ"
    /// (verified against the interpreter). This is the one place where reading
    /// `has_decimal` would be actively wrong.
    ///
    /// # Known divergence: negative-exponent floats
    ///
    /// The guard below catches *positive* exponent forms, but the mirror case
    /// cannot be caught from here. `float 1e-05` and `Decimal("0.00001")` reach
    /// this function as **byte-identical state** — same digits (1), same scale
    /// (5), same `has_decimal` (true) — yet Python disagrees about them:
    /// `str(1e-05)` is "1e-05" and raises ValueError, while
    /// `str(Decimal("0.00001"))` is "0.00001" and yields "ባዶ ዩሮ". Only the
    /// discarded string tells them apart, so no rule available here can be both
    /// sound and complete; guessing on scale would misfire on legitimate small
    /// `Decimal`s. The plain-decimal reading is chosen, which means floats with
    /// `0 < |v| < 1e-4` return "ባዶ <unit>" where Python raises ValueError.
    /// Fixing it needs `CurrencyValue` to carry the original `str(val)`.
    fn split_currency_parts(&self, val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
        match val {
            // str(int) never contains ".", so parts has one element.
            CurrencyValue::Int(v) => Ok((v.abs(), BigInt::zero())),
            CurrencyValue::Decimal { value, .. } => {
                let v = value.abs();
                // A negative scale means `str(val)` used exponent notation
                // ("1e+21", "1.5e+16", "1E+21") — a plain decimal string always
                // parses to scale >= 0, so this has no false positives. Python
                // then feeds a token containing "e" to int(), which always
                // raises ValueError: from parts[0] ("1e+21") when the repr has
                // no ".", or from parts[1][:2] ("5e") when it does. Both raise
                // the same type, so the branch is collapsed here.
                //
                // The *message* is best-effort, since the literal Python quotes
                // is the string this boundary discarded. It comes out exact for
                // the common float reprs ("1e+21", "1e+16" — bigdecimal Displays
                // those verbatim); it differs for `Decimal("1E+21")` (we
                // lowercase the "E") and for fractional mantissas, where Python
                // quotes the mangled fragment "5e" rather than "1.5e+16". The
                // variant is what callers catch, and that is exact.
                if v.as_bigint_and_exponent().1 < 0 {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{v}'"
                    )));
                }
                let left = v.with_scale(0).as_bigint_and_exponent().0;
                let frac = &v - BigDecimal::from(left.clone());
                let right = (frac * BigDecimal::from(CENTS_PER_UNIT))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                Ok((left, right))
            }
        }
    }

    /// Port of `Num2Word_TI._int_to_word`.
    ///
    /// Only ever called with a **non-negative** value: `to_cardinal` peels the
    /// sign off before calling, so the `divmod`s below match Python's
    /// (Python's floor-division semantics only diverge from Rust's truncating
    /// `/` on negatives). All the small-index conversions are therefore
    /// provably in range.
    fn int_to_word(&self, number: &BigInt) -> String {
        // Python: if number == 0: return "ባዶ"
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // Python: if number < 10: return self.ones[number]
        if *number < BigInt::from(10) {
            // Safe: 0 < number < 10.
            return ONES[to_index(number)].to_string();
        }

        // Python: t, o = divmod(number, 10)
        //         return self.tens[t] + (" ን " + self.ones[o] if o else "")
        if *number < BigInt::from(100) {
            let (t, o) = number.div_mod_floor(&BigInt::from(10));
            let t = to_index(&t); // 1..=9
            let o = to_index(&o); // 0..=9
            let mut out = TENS[t].to_string();
            if o != 0 {
                out.push_str(AND);
                out.push_str(ONES[o]);
            }
            return out;
        }

        // Python: h, r = divmod(number, 100)
        //         base = (self.ones[h] + " " if h > 1 else "") + self.hundred
        //         return base + (" ን " + self._int_to_word(r) if r else "")
        if *number < BigInt::from(1000) {
            let (h, r) = number.div_mod_floor(&BigInt::from(100));
            let h = to_index(&h); // 1..=9
            let mut out = String::new();
            // The `h > 1` guard: 100 -> "ሚእቲ", never "ሓደ ሚእቲ".
            if h > 1 {
                out.push_str(ONES[h]);
                out.push(' ');
            }
            out.push_str(HUNDRED);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // Python: t, r = divmod(number, 1000)
        //         base = (self._int_to_word(t) + " " if t > 1 else "")
        //                + self.thousand
        //         return base + (" ን " + self._int_to_word(r) if r else "")
        if *number < BigInt::from(1_000_000) {
            let (t, r) = number.div_mod_floor(&BigInt::from(1000));
            let mut out = String::new();
            // The `t > 1` guard: 1000 -> "ሽሕ", never "ሓደ ሽሕ".
            if t > BigInt::from(1) {
                out.push_str(&self.int_to_word(&t));
                out.push(' ');
            }
            out.push_str(THOUSAND);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // Python: m, r = divmod(number, 1000000)
        //         base = (self._int_to_word(m) + " " if m > 1 else "")
        //                + self.million
        //         return base + (" ን " + self._int_to_word(r) if r else "")
        if *number < BigInt::from(1_000_000_000) {
            let (m, r) = number.div_mod_floor(&BigInt::from(1_000_000));
            let mut out = String::new();
            // The `m > 1` guard is present here too: 10**6 -> "ሚልዮን".
            if m > BigInt::from(1) {
                out.push_str(&self.int_to_word(&m));
                out.push(' ');
            }
            out.push_str(MILLION);
            if !r.is_zero() {
                out.push_str(AND);
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }

        // Python: return str(number) — the billion cliff (bug note 1).
        // `number` is unbounded here (values reach 10**21+ in the corpus), so
        // this must stay BigInt: never cast.
        number.to_string()
    }

    /// Port of `Num2Word_TI.to_cardinal` for **non-integer** input.
    ///
    /// TI's `to_cardinal` works entirely on `str(number)` — the float/Decimal
    /// path is `n.split(".", 1)` with a per-*character* fraction loop, never
    /// `float2tuple`. So the two `PORTING_FLOAT.md` traps do not apply: there is
    /// no `round()` and no `abs(value-pre)*10**precision` arithmetic — nothing to
    /// round banker's-vs-away, and no f64 multiply artefact to preserve. The one
    /// thing that must be exact is `str(number)` itself, which
    /// [`python_str`] reconstructs byte-for-byte for both arms.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "ባዶ")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The recursion is string-level in Python: `to_cardinal(n[1:])` hands a
    /// `str` straight back, whose `str(number)` is then a no-op. Reproduced as
    /// such rather than folded into an `abs()` — one level deep in practice, since
    /// no repr yields a second leading "-". Because the sign is read off the
    /// *string*, negative zero keeps its "-" (`str(-0.0) == "-0.0"`), so `-0.0`
    /// renders "ኣሉታ ባዶ ነጥቢ ባዶ" — a case the base float path, which
    /// tests `value < 0.0`, would strip. That divergence is precisely why TI
    /// cannot inherit the default `to_cardinal_float`.
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        // n = str(number).strip(). Python strips its own whitespace set and Rust
        // trims Unicode's; no repr contains either.
        let n = n.trim();

        // if n.startswith("-"):
        //     return (self.negword + self.to_cardinal(n[1:])).strip()
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
            None => return Ok(self.int_to_word(&py_int(n)?)),
        };

        // ret = self._int_to_word(int(left)) + " " + self.pointword
        //
        // `int(left)` is the whole integer part, so the billion cliff (bug note
        // 1) applies: at 1e9 and above it comes back as bare digits
        // ("98746251323029 ነጥቢ ትሽዓተ ትሽዓተ").
        let mut ret = self.int_to_word(&py_int(left)?);
        ret.push(' ');
        ret.push_str(POINTWORD);

        // for digit in right: ret += " " + (self.ones[int(digit)] or "ባዶ")
        //
        // Per *character* — no grouping, no rounding, no padding: the fraction
        // is exactly the digits the repr carried. `self.ones[0]` is `""`
        // (falsy), so the `or "ባዶ"` is what turns a fraction digit 0 into a
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

/// Convert a BigInt the caller has proven to be a small non-negative index.
///
/// Every call site above is guarded by an explicit range check, so the
/// conversion cannot fail.
fn to_index(n: &BigInt) -> usize {
    debug_assert!(!n.is_negative() && *n < BigInt::from(10));
    // `expect` documents the invariant; the guards make it unreachable.
    n.to_usize().expect("caller proved this value is in 0..=9")
}

impl Lang for LangTi {

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

    /// `to_ordinal(float/Decimal)`. TI's `to_ordinal` is
    /// `self.to_cardinal(number) + "ኣይ"` for *every* input, so the float
    /// entry is the float cardinal plus the suffix — "ሓደ ነጥቢ ባዶኣይ".
    /// An exponent-form Decimal repr ("1E+2") still dies in `int()` with
    /// ValueError inside the cardinal, before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "ኣይ"` — the repr the
    /// binding computed, suffix glued on, sign and exponent form included
    /// ("-0.0ኣይ", "1e+16ኣይ").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside TI's `int("Infinity")` (`to_cardinal` recurses
    /// past the sign, finds no "." and calls `int()`). The binding otherwise
    /// hard-codes `ParsedNumber::Inf` to the base integer path's
    /// OverflowError before any TI code runs, so the ValueError must be
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
        "ETB"
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
        "ነጥቢ"
    }

    // `is_title` stays false (the base default), so `title()` is the identity
    // and `exclude_title` never actually gates anything.
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_TI.to_cardinal`, integer path only.
    ///
    /// Python operates on `n = str(number).strip()`. For integer input the
    /// `"." in n` branch (the float path) is unreachable, leaving only the
    /// sign test and the `_int_to_word(int(n))` tail.
    ///
    /// The Python sign branch recurses through `to_cardinal(n[1:])` — dropping
    /// the leading "-" from the *string*, which for integer input is exactly
    /// `value.abs()`, and which cannot re-enter the sign branch. The `.strip()`
    /// is a no-op in practice (`negword` supplies the separating space and
    /// `int_to_word` never returns an empty string for a non-zero input), but
    /// it is reproduced for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // Python: (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = self.int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// Port of `Num2Word_TI.to_ordinal`: `self.to_cardinal(number) + "ኣይ"`.
    ///
    /// No `verify_ordinal` call, so zero and negatives pass straight through
    /// (bug note 3).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_TI.to_ordinal_num`: `str(number) + "ኣይ"`.
    ///
    /// Digits, not words — and the sign is kept: `-1` -> "-1ኣይ".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_TI.to_year`: ignores `longval` and delegates
    /// (bug note 4).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_TI.to_cardinal` for non-integer input.
    ///
    /// TI overrides `to_cardinal` (not `to_cardinal_float`) and handles floats
    /// and Decimals inline by splitting `str(number)`, so the port reconstructs
    /// that exact string via [`python_str`] and hands it to
    /// [`Self::cardinal_from_str`]. The float and Decimal arms of `str()` are not
    /// interchangeable (issue #603): a float stringifies to shortest round-trip
    /// and goes exponential outside `[1e-4, 1e16)`, while a Decimal stringifies
    /// exactly and goes exponential on its own written exponent. Reproducing both
    /// is what makes the exponent-form `ValueError` fire exactly where Python's
    /// does (`int("1e+16")`).
    ///
    /// `Num2Word_TI.to_cardinal(self, number)` takes **no** `precision=` kwarg,
    /// and the live interpreter confirms the kwarg is dropped before this method
    /// is reached (`num2words(0.5, lang='ti', precision=3) == "ባዶ ነጥቢ
    /// ሓሙሽተ"`, unchanged), so `precision_override` is ignored here.
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
    // TI overrides `to_currency` and `pluralize`; everything else on the
    // currency path (`to_cheque`, `_money_verbose`, `_cents_verbose`,
    // `_cents_terse`) is inherited from `Num2Word_Base` unchanged and the
    // trait defaults already mirror it. `CURRENCY_ADJECTIVES` and
    // `CURRENCY_PRECISION` are both `{}`, so `currency_adjective` (None) and
    // `currency_precision` (100) stay at their defaults too.

    fn lang_name(&self) -> &str {
        "Num2Word_TI"
    }

    /// `CURRENCY_FORMS[code]` — a *strict* lookup, unlike `to_currency`'s.
    ///
    /// Returning `None` for an unknown code is what makes Base's `to_cheque`
    /// raise `NotImplementedError`, matching the corpus rows for
    /// `cheque:{GBP,JPY,KWD,BHD,INR,CNY,CHF}`. `to_currency` deliberately does
    /// *not* come through here; it substitutes [`LangTi::fallback_forms`].
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_TI.pluralize`:
    ///
    /// ```text
    /// if not forms: return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Note this takes the **last** form, not `forms[1]` as `Num2Word_EUR`'s
    /// does. Identical for TI's two-form tuples, but the rule is what is being
    /// ported, so it is kept literal. The empty guard means TI's `pluralize` —
    /// alone among the ones that matter — cannot raise `IndexError`.
    ///
    /// Unreachable on every path this crate exercises: TI's `to_currency`
    /// indexes the tuple inline and Base's `to_cheque` takes `cr1[-1]`
    /// directly. Ported because it is a public method on the Python class.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms.last() {
            // Python: if not forms: return ""
            None => Ok(String::new()),
            Some(last) => Ok(if n.is_one() {
                forms[0].clone()
            } else {
                last.clone()
            }),
        }
    }

    /// Port of `Num2Word_TI.to_currency`.
    ///
    /// ```text
    /// def to_currency(self, val, currency="ETB", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " \
    ///                 + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// `adjective` is accepted and never read — that is the Python signature,
    /// not an oversight here, so no adjective prefixing happens even though
    /// the parameter exists.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `None` means the kwarg was omitted, so TI's own default (" ") applies.
        let separator = separator.unwrap_or(self.default_separator());

        // Python: is_negative = val < 0; val = abs(val)
        let is_negative = val.is_negative();
        let (left, right) = self.split_currency_parts(val)?;

        // Python: cr1, cr2 = CURRENCY_FORMS.get(currency, list(...values())[0])
        // No NotImplementedError here — unknown codes fall back to ETB.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);

        // Python: self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
        let mut result = format!(
            "{} {}",
            self.int_to_word(&left),
            pick_form(&forms.unit, &left)?
        );

        // Python: if cents and right:  -- `right == 0` is falsy, so a zero-cent
        // value ("1.0", "0.005") drops the segment entirely.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                self.int_to_word(&right),
                pick_form(&forms.subunit, &right)?
            ));
        }

        // Python: if is_negative: result = self.negword + result
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // Python: return result.strip()
        Ok(result.trim().to_string())
    }
}

/// Python's inline `forms[1] if n != 1 else forms[0]`, as `to_currency` writes
/// it for both `cr1` and `cr2`.
///
/// Python indexes the tuple directly, so a single-form entry with `n != 1`
/// would raise `IndexError`. Every entry in [`CURRENCY_FORMS`] carries two
/// forms and the fallback does too, so this is unreachable — but it is mapped
/// to `N2WError::Index` rather than panicking so the exception type survives if
/// the table ever changes.
fn pick_form(forms: &[String], n: &BigInt) -> Result<String> {
    let idx = if n.is_one() { 0 } else { 1 };
    forms
        .get(idx)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

// ---- the float/Decimal path's `str(number)` reconstruction --------------
//
// TI's `to_cardinal` works entirely on `str(number)`, so a faithful port has to
// hand `cardinal_from_str` the byte-for-byte string CPython would produce. These
// helpers mirror the sibling `lang_pap.rs` / `lang_ceb.rs` ports (Tigrinya shares
// their exact `_int_to_word`/`str`-splitting structure); each is documented and
// mass-verified against CPython there. Only the language constants differ, and
// none of those constants appear in the helpers themselves.

/// CPython's `str(number)` for the value that reached TI's `to_cardinal`.
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
/// TI's `startswith("-")` fires on negative zero.
fn py_str_f64(v: f64) -> String {
    // Unreachable from the shim (it derives `precision` from a finite Decimal),
    // handled anyway so this is a faithful `str()`: TI would go on to raise
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
/// keeps it exponential, so TI raises), on `0.0` (Python keeps the ".0"), and on
/// the `e`/`E` case. `BigDecimal::from_str` keeps the written scale (`"1.10"`
/// stays coefficient 110 / scale 2), which is what makes the trailing "ባዶ"
/// appear, and `(coefficient, -scale)` is exactly Python's `(_int, _exp)`.
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

    /// Every `"lang": "ti", "to": "currency:*"` row in the frozen corpus (108),
    /// pasted from `bench/corpus.jsonl` rather than retyped.
    #[rustfmt::skip]
    const CURRENCY_CORPUS: &[(&str, &str, bool, &str)] = &[
        ("EUR", "0", true, "ባዶ ዩሮ"),
        ("EUR", "1", true, "ሓደ ዩሮ"),
        ("EUR", "2", true, "ክልተ ዩሮ"),
        ("EUR", "100", true, "ሚእቲ ዩሮ"),
        ("EUR", "12.34", false, "ዓሰርተ ን ክልተ ዩሮ ሰላሳ ን ኣርባዕተ ሰንት"),
        ("EUR", "0.01", false, "ባዶ ዩሮ ሓደ ሰንት"),
        ("EUR", "1.0", false, "ሓደ ዩሮ"),
        ("EUR", "99.99", false, "ተስዓ ን ትሽዓተ ዩሮ ተስዓ ን ትሽዓተ ሰንት"),
        ("EUR", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ዩሮ ሓምሳ ን ሽዱሽተ ሰንት"),
        ("EUR", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ዩሮ ሰላሳ ን ኣርባዕተ ሰንት"),
        ("EUR", "1000000", true, "ሚልዮን ዩሮ"),
        ("EUR", "0.5", false, "ባዶ ዩሮ ሓምሳ ሰንት"),
        ("USD", "0", true, "ባዶ ዶላር"),
        ("USD", "1", true, "ሓደ ዶላር"),
        ("USD", "2", true, "ክልተ ዶላር"),
        ("USD", "100", true, "ሚእቲ ዶላር"),
        ("USD", "12.34", false, "ዓሰርተ ን ክልተ ዶላር ሰላሳ ን ኣርባዕተ ሰንት"),
        ("USD", "0.01", false, "ባዶ ዶላር ሓደ ሰንት"),
        ("USD", "1.0", false, "ሓደ ዶላር"),
        ("USD", "99.99", false, "ተስዓ ን ትሽዓተ ዶላር ተስዓ ን ትሽዓተ ሰንት"),
        ("USD", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ዶላር ሓምሳ ን ሽዱሽተ ሰንት"),
        ("USD", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ዶላር ሰላሳ ን ኣርባዕተ ሰንት"),
        ("USD", "1000000", true, "ሚልዮን ዶላር"),
        ("USD", "0.5", false, "ባዶ ዶላር ሓምሳ ሰንት"),
        ("GBP", "0", true, "ባዶ ብር"),
        ("GBP", "1", true, "ሓደ ብር"),
        ("GBP", "2", true, "ክልተ ብር"),
        ("GBP", "100", true, "ሚእቲ ብር"),
        ("GBP", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("GBP", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("GBP", "1.0", false, "ሓደ ብር"),
        ("GBP", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("GBP", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("GBP", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("GBP", "1000000", true, "ሚልዮን ብር"),
        ("GBP", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("JPY", "0", true, "ባዶ ብር"),
        ("JPY", "1", true, "ሓደ ብር"),
        ("JPY", "2", true, "ክልተ ብር"),
        ("JPY", "100", true, "ሚእቲ ብር"),
        ("JPY", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("JPY", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("JPY", "1.0", false, "ሓደ ብር"),
        ("JPY", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("JPY", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("JPY", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("JPY", "1000000", true, "ሚልዮን ብር"),
        ("JPY", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("KWD", "0", true, "ባዶ ብር"),
        ("KWD", "1", true, "ሓደ ብር"),
        ("KWD", "2", true, "ክልተ ብር"),
        ("KWD", "100", true, "ሚእቲ ብር"),
        ("KWD", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("KWD", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("KWD", "1.0", false, "ሓደ ብር"),
        ("KWD", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("KWD", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("KWD", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("KWD", "1000000", true, "ሚልዮን ብር"),
        ("KWD", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("BHD", "0", true, "ባዶ ብር"),
        ("BHD", "1", true, "ሓደ ብር"),
        ("BHD", "2", true, "ክልተ ብር"),
        ("BHD", "100", true, "ሚእቲ ብር"),
        ("BHD", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("BHD", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("BHD", "1.0", false, "ሓደ ብር"),
        ("BHD", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("BHD", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("BHD", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("BHD", "1000000", true, "ሚልዮን ብር"),
        ("BHD", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("INR", "0", true, "ባዶ ብር"),
        ("INR", "1", true, "ሓደ ብር"),
        ("INR", "2", true, "ክልተ ብር"),
        ("INR", "100", true, "ሚእቲ ብር"),
        ("INR", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("INR", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("INR", "1.0", false, "ሓደ ብር"),
        ("INR", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("INR", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("INR", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("INR", "1000000", true, "ሚልዮን ብር"),
        ("INR", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("CNY", "0", true, "ባዶ ብር"),
        ("CNY", "1", true, "ሓደ ብር"),
        ("CNY", "2", true, "ክልተ ብር"),
        ("CNY", "100", true, "ሚእቲ ብር"),
        ("CNY", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("CNY", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("CNY", "1.0", false, "ሓደ ብር"),
        ("CNY", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("CNY", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("CNY", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("CNY", "1000000", true, "ሚልዮን ብር"),
        ("CNY", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
        ("CHF", "0", true, "ባዶ ብር"),
        ("CHF", "1", true, "ሓደ ብር"),
        ("CHF", "2", true, "ክልተ ብር"),
        ("CHF", "100", true, "ሚእቲ ብር"),
        ("CHF", "12.34", false, "ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("CHF", "0.01", false, "ባዶ ብር ሓደ ሳንቲም"),
        ("CHF", "1.0", false, "ሓደ ብር"),
        ("CHF", "99.99", false, "ተስዓ ን ትሽዓተ ብር ተስዓ ን ትሽዓተ ሳንቲም"),
        ("CHF", "1234.56", false, "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ብር ሓምሳ ን ሽዱሽተ ሳንቲም"),
        ("CHF", "-12.34", false, "ኣሉታ ዓሰርተ ን ክልተ ብር ሰላሳ ን ኣርባዕተ ሳንቲም"),
        ("CHF", "1000000", true, "ሚልዮን ብር"),
        ("CHF", "0.5", false, "ባዶ ብር ሓምሳ ሳንቲም"),
    ];

    /// Every `"lang": "ti", "to": "cheque:*"` row (9). `None` = NotImplementedError.
    #[rustfmt::skip]
    const CHEQUE_CORPUS: &[(&str, &str, Option<&str>)] = &[
        ("EUR", "1234.56", Some("ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ AND 56/100 ዩሮ")),
        ("USD", "1234.56", Some("ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ AND 56/100 ዶላር")),
        ("GBP", "1234.56", None),
        ("JPY", "1234.56", None),
        ("KWD", "1234.56", None),
        ("BHD", "1234.56", None),
        ("INR", "1234.56", None),
        ("CNY", "1234.56", None),
        ("CHF", "1234.56", None),
    ];

    /// Rebuild the `CurrencyValue` the shim would hand us for a corpus `arg`.
    /// `arg` is `repr(value)`, so "12.34" is a float and "100" an int — and
    /// `has_decimal` mirrors `isinstance(val, float) or "." in str(val)`.
    fn value_of(arg: &str, is_int: bool) -> CurrencyValue {
        CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap()
    }

    #[test]
    fn currency_matches_corpus() {
        let ti = LangTi::new();
        for (code, arg, is_int, expected) in CURRENCY_CORPUS {
            let got = ti
                .to_currency(&value_of(arg, *is_int), code, true, None, false)
                .unwrap_or_else(|_| panic!("{code} {arg} raised"));
            assert_eq!(&got, expected, "currency:{code} arg={arg}");
        }
    }

    #[test]
    fn cheque_matches_corpus() {
        let ti = LangTi::new();
        for (code, arg, expected) in CHEQUE_CORPUS {
            let got = ti.to_cheque(&BigDecimal::from_str(arg).unwrap(), code);
            match (got, expected) {
                (Ok(g), Some(e)) => assert_eq!(&g, e, "cheque:{code}"),
                (Err(N2WError::NotImplemented(m)), None) => assert_eq!(
                    m,
                    format!("Currency code \"{code}\" not implemented for \"Num2Word_TI\"")
                ),
                (g, e) => panic!("cheque:{code}: got {g:?}, expected {e:?}"),
            }
        }
    }

    /// Unknown codes fall back to `list(CURRENCY_FORMS.values())[0]` (ETB) in
    /// `to_currency`, but are still absent from the strict `currency_forms`
    /// lookup that `to_cheque` uses. The two must disagree.
    #[test]
    fn unknown_code_falls_back_in_currency_but_not_cheque() {
        let ti = LangTi::new();
        assert!(ti.currency_forms("GBP").is_none());
        assert_eq!(
            ti.to_currency(&value_of("2", true), "GBP", true, None, false).unwrap(),
            "ክልተ ብር"
        );
        // ETB itself is the fallback, so it must render identically.
        assert_eq!(
            ti.to_currency(&value_of("2", true), "ETB", true, None, false).unwrap(),
            "ክልተ ብር"
        );
        // ERN is the only code whose unit differs from the fallback's.
        assert_eq!(
            ti.to_currency(&value_of("2", true), "ERN", true, None, false).unwrap(),
            "ክልተ ናቕፋ"
        );
    }

    /// `str(val)`-based cents truncate; they never round. Verified against the
    /// interpreter: 2.675 -> 67 ሳንቲም, 12.345 -> 34, 0.005 -> no segment.
    #[test]
    fn cents_truncate_rather_than_round() {
        let ti = LangTi::new();
        let c = |a: &str| ti.to_currency(&value_of(a, false), "EUR", true, None, false).unwrap();
        assert_eq!(c("2.675"), "ክልተ ዩሮ ስሳ ን ሸውዓተ ሰንት");
        assert_eq!(c("12.345"), "ዓሰርተ ን ክልተ ዩሮ ሰላሳ ን ኣርባዕተ ሰንት");
        assert_eq!(c("0.005"), "ባዶ ዩሮ");
    }

    /// TI reads the *text* of `str(val)`, never `isinstance`, so int 5,
    /// `Decimal("5")`, `Decimal("5.00")` and 5.0 all collapse to one output.
    /// This is the case where honouring `has_decimal` would be wrong.
    #[test]
    fn has_decimal_is_not_consulted() {
        let ti = LangTi::new();
        let c = |v: CurrencyValue| ti.to_currency(&v, "EUR", true, None, false).unwrap();
        assert_eq!(c(CurrencyValue::parse("5", true, false, false).unwrap()), "ሓሙሽተ ዩሮ");
        assert_eq!(c(CurrencyValue::parse("5", false, false, false).unwrap()), "ሓሙሽተ ዩሮ");
        assert_eq!(c(CurrencyValue::parse("5.00", false, false, false).unwrap()), "ሓሙሽተ ዩሮ");
        assert_eq!(c(CurrencyValue::parse("5.0", false, true, true).unwrap()), "ሓሙሽተ ዩሮ");
    }

    /// `cents=False` still routes through `_int_to_word` — TI never calls
    /// `_cents_terse`, so the flag only ever suppresses the whole segment.
    #[test]
    fn cents_flag_suppresses_the_segment() {
        let ti = LangTi::new();
        assert_eq!(
            ti.to_currency(&value_of("12.34", false), "EUR", false, None, false).unwrap(),
            "ዓሰርተ ን ክልተ ዩሮ"
        );
    }

    /// An explicit separator overrides TI's " " default; `None` must not.
    #[test]
    fn separator_is_honoured() {
        let ti = LangTi::new();
        assert_eq!(
            ti.to_currency(&value_of("12.34", false), "EUR", true, Some(", "), false).unwrap(),
            "ዓሰርተ ን ክልተ ዩሮ, ሰላሳ ን ኣርባዕተ ሰንት"
        );
        assert_eq!(ti.default_separator(), " ");
    }

    /// `left` goes through `_int_to_word`, so the billion cliff reaches
    /// currency too: verified `to_currency(1e15, "EUR")` is digits + " ዩሮ".
    #[test]
    fn billion_cliff_reaches_currency() {
        let ti = LangTi::new();
        assert_eq!(
            ti.to_currency(&value_of("1000000000000000.0", false), "EUR", true, None, false).unwrap(),
            "1000000000000000 ዩሮ"
        );
    }

    /// `adjective` is accepted and never read by the Python signature.
    #[test]
    fn adjective_is_ignored() {
        let ti = LangTi::new();
        assert_eq!(
            ti.to_currency(&value_of("2", true), "EUR", true, None, true).unwrap(),
            "ክልተ ዩሮ"
        );
        assert!(ti.currency_adjective("EUR").is_none());
    }

    /// `CURRENCY_PRECISION` is `{}`, so every code is 2-decimal.
    #[test]
    fn precision_is_always_the_default() {
        let ti = LangTi::new();
        for code in ["EUR", "USD", "ETB", "ERN", "KWD", "BHD", "JPY"] {
            assert_eq!(ti.currency_precision(code), 100, "{code}");
        }
    }

    /// Exponent-form `str(val)` feeds an "e" token to `int()` in Python, which
    /// raises ValueError. A plain decimal string always parses to scale >= 0,
    /// so a negative scale is a sound detector for it.
    #[test]
    fn exponent_notation_is_a_value_error() {
        let ti = LangTi::new();
        for s in ["1e+21", "1E+21", "1.5e+16", "1e+16"] {
            let v = CurrencyValue::parse(s, false, true, true).unwrap();
            assert!(
                matches!(ti.to_currency(&v, "EUR", true, None, false), Err(N2WError::Value(_))),
                "{s} should be a ValueError"
            );
        }
        // ...while the plain-decimal spelling of the same magnitude does not.
        let v = CurrencyValue::parse("1000000000000000000000", false, false, false).unwrap();
        assert!(ti.to_currency(&v, "EUR", true, None, false).is_ok());
    }

    /// Pins the one **known divergence** from Python, so it fails loudly if
    /// `CurrencyValue` ever starts carrying the original `str(val)` and this
    /// can finally be made exact.
    ///
    /// Python raises ValueError for `float 1e-05` (repr "1e-05") but returns
    /// "ባዶ ዩሮ" for `Decimal("0.00001")`. Both arrive here as digits=1,
    /// scale=5, has_decimal=true — identical — so this port cannot honour both
    /// and takes the plain-decimal reading. See `split_currency_parts`.
    #[test]
    fn negative_exponent_floats_diverge_from_python() {
        let ti = LangTi::new();
        let small = CurrencyValue::parse("0.00001", false, true, true).unwrap();
        // Correct for Decimal("0.00001"); Python would raise for float 1e-05.
        assert_eq!(
            ti.to_currency(&small, "EUR", true, None, false).unwrap(),
            "ባዶ ዩሮ"
        );
        // The two inputs really are indistinguishable at this boundary.
        let as_float_repr = CurrencyValue::parse("0.00001", false, true, true).unwrap();
        assert_eq!(
            format!("{:?}", small.clone()),
            format!("{as_float_repr:?}"),
            "if these ever differ, the divergence is fixable"
        );
    }

    /// Guards the assumption `split_currency_parts` rests on: `with_scale(0)`
    /// truncates toward zero and plain decimal strings never yield scale < 0.
    #[test]
    fn bigdecimal_assumptions_hold() {
        for s in ["0", "5", "5.00", "1.0", "0.5", "12.345", "1000000000000000.0"] {
            assert!(
                BigDecimal::from_str(s).unwrap().as_bigint_and_exponent().1 >= 0,
                "{s} must not parse to a negative scale"
            );
        }
        let trunc = |s: &str| BigDecimal::from_str(s).unwrap().with_scale(0).to_string();
        assert_eq!(trunc("12.999"), "12");
        assert_eq!(trunc("0.5"), "0");
    }

    /// Build the `FloatValue` the shim hands to `to_cardinal_float` for a float
    /// corpus `arg` (`repr(value)`): precision = fractional digit count.
    fn float_val(arg: &str) -> FloatValue {
        let value: f64 = arg.parse().unwrap();
        let precision = arg.split_once('.').map_or(0, |(_, f)| f.len() as u32);
        FloatValue::Float { value, precision }
    }

    /// Build the `FloatValue::Decimal` for a `cardinal_dec` row.
    fn dec_val(arg: &str) -> FloatValue {
        let value = BigDecimal::from_str(arg).unwrap();
        let precision = value.as_bigint_and_exponent().1.max(0) as u32;
        FloatValue::Decimal { value, precision }
    }

    /// Every `"lang": "ti", "to": "cardinal"` row with a float `arg` in the
    /// frozen corpus, pasted verbatim.
    #[rustfmt::skip]
    const FLOAT_CARDINAL_CORPUS: &[(&str, &str)] = &[
        ("0.0", "ባዶ ነጥቢ ባዶ"),
        ("0.5", "ባዶ ነጥቢ ሓሙሽተ"),
        ("1.0", "ሓደ ነጥቢ ባዶ"),
        ("1.5", "ሓደ ነጥቢ ሓሙሽተ"),
        ("2.25", "ክልተ ነጥቢ ክልተ ሓሙሽተ"),
        ("3.14", "ሰለስተ ነጥቢ ሓደ ኣርባዕተ"),
        ("0.01", "ባዶ ነጥቢ ባዶ ሓደ"),
        ("0.1", "ባዶ ነጥቢ ሓደ"),
        ("0.99", "ባዶ ነጥቢ ትሽዓተ ትሽዓተ"),
        ("1.01", "ሓደ ነጥቢ ባዶ ሓደ"),
        ("12.34", "ዓሰርተ ን ክልተ ነጥቢ ሰለስተ ኣርባዕተ"),
        ("99.99", "ተስዓ ን ትሽዓተ ነጥቢ ትሽዓተ ትሽዓተ"),
        ("100.5", "ሚእቲ ነጥቢ ሓሙሽተ"),
        ("1234.56", "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ ነጥቢ ሓሙሽተ ሽዱሽተ"),
        ("-0.5", "ኣሉታ ባዶ ነጥቢ ሓሙሽተ"),
        ("-1.5", "ኣሉታ ሓደ ነጥቢ ሓሙሽተ"),
        ("-12.34", "ኣሉታ ዓሰርተ ን ክልተ ነጥቢ ሰለስተ ኣርባዕተ"),
        ("1.005", "ሓደ ነጥቢ ባዶ ባዶ ሓሙሽተ"),
        ("2.675", "ክልተ ነጥቢ ሽዱሽተ ሸውዓተ ሓሙሽተ"),
    ];

    /// Every `"lang": "ti", "to": "cardinal_dec"` row.
    #[rustfmt::skip]
    const DECIMAL_CARDINAL_CORPUS: &[(&str, &str)] = &[
        ("0.01", "ባዶ ነጥቢ ባዶ ሓደ"),
        ("1.10", "ሓደ ነጥቢ ሓደ ባዶ"),
        ("12.345", "ዓሰርተ ን ክልተ ነጥቢ ሰለስተ ኣርባዕተ ሓሙሽተ"),
        ("98746251323029.99", "98746251323029 ነጥቢ ትሽዓተ ትሽዓተ"),
        ("0.001", "ባዶ ነጥቢ ባዶ ባዶ ሓደ"),
    ];

    #[test]
    fn float_cardinal_matches_corpus() {
        let ti = LangTi::new();
        for (arg, expected) in FLOAT_CARDINAL_CORPUS {
            let got = ti.to_cardinal_float(&float_val(arg), None).unwrap();
            assert_eq!(&got, expected, "cardinal float arg={arg}");
        }
    }

    #[test]
    fn decimal_cardinal_matches_corpus() {
        let ti = LangTi::new();
        for (arg, expected) in DECIMAL_CARDINAL_CORPUS {
            let got = ti.to_cardinal_float(&dec_val(arg), None).unwrap();
            assert_eq!(&got, expected, "cardinal_dec arg={arg}");
        }
    }

    /// The `precision=` kwarg is dropped by the dispatcher before TI's
    /// `to_cardinal` runs, so `precision_override` must not change the output.
    #[test]
    fn precision_override_is_ignored() {
        let ti = LangTi::new();
        assert_eq!(
            ti.to_cardinal_float(&float_val("0.5"), Some(3)).unwrap(),
            "ባዶ ነጥቢ ሓሙሽተ"
        );
    }

    /// Negative zero keeps its sign because it is read off `str(-0.0) == "-0.0"`,
    /// not from `value < 0.0` — verified against the interpreter.
    #[test]
    fn negative_zero_keeps_negword() {
        let ti = LangTi::new();
        let neg_zero = FloatValue::Float { value: -0.0, precision: 1 };
        assert_eq!(
            ti.to_cardinal_float(&neg_zero, None).unwrap(),
            "ኣሉታ ባዶ ነጥቢ ባዶ"
        );
    }

    /// The four already-verified integer modes must not have shifted.
    #[test]
    fn integer_modes_unchanged() {
        let ti = LangTi::new();
        assert_eq!(ti.to_cardinal(&BigInt::from(1234)).unwrap(), "ሽሕ ን ክልተ ሚእቲ ን ሰላሳ ን ኣርባዕተ");
        assert_eq!(ti.to_cardinal(&BigInt::from(0)).unwrap(), "ባዶ");
        assert_eq!(ti.to_ordinal(&BigInt::from(11)).unwrap(), "ዓሰርተ ን ሓደኣይ");
        assert_eq!(ti.to_ordinal_num(&BigInt::from(-1)).unwrap(), "-1ኣይ");
        assert_eq!(ti.to_year(&BigInt::from(1999)).unwrap(), "ሽሕ ን ትሽዓተ ሚእቲ ን ተስዓ ን ትሽዓተ");
    }
}
