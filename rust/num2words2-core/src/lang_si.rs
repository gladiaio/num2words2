//! Port of `lang_SI.py` (Sinhala).
//!
//! Registry check: `CONVERTER_CLASSES["si"]` is `lang_SI.Num2Word_SI()`
//! (`__init__.py:364`), so this file ports `Num2Word_SI` — the only class the
//! module defines.
//!
//! Shape: **self-contained**. `Num2Word_SI` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` skips its `set_numwords()` branch entirely:
//! `self.cards` is never created and **`self.MAXVAL` is never assigned**.
//! `to_cardinal` is overridden outright and drives a hand-written
//! `_int_to_word` recursion, so `cards`/`maxval`/`merge` stay at their trait
//! defaults here and are never consulted. There is no overflow check of any
//! kind — see bug 4 below for what happens instead.
//!
//! The numbering system is Indic: units of **lakh** (10^5) and **crore**
//! (10^7) rather than million/billion.
//!
//! Inherited from `Num2Word_Base` but overridden by SI (so the trait defaults
//! are *not* used):
//!   * `to_ordinal(value)`     → `to_cardinal(value) + " වැනි"`
//!   * `to_ordinal_num(value)` → `str(value) + "."`
//!   * `to_year(value, longval=True)` → `to_cardinal(value)`; the `longval`
//!     parameter is accepted and then ignored, which is why the trait's
//!     single-argument `to_year` is a faithful signature.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong and is exactly
//! what Python emits; all are confirmed against the frozen corpus.
//!
//! 1. **`negword` and `pointword` (fixed, gladiaio/num2words2#154).** Python's
//!    `setup()` sets the English `self.negword = "minus "` and
//!    `self.pointword = "point"` in a module whose every other word is
//!    Sinhala. This port uses the Sinhala සෘණ ("negative") and දශම
//!    ("decimal"): `to_cardinal(-1)` == "සෘණ එක", `to_cardinal(1.5)` ==
//!    "එක දශම පහ", `to_year(-500)` == "සෘණ පහ සියය".
//! 2. **`self.ones[0]` is `""`**, so the zero ternary
//!    `return self.ones[0] if self.ones[0] else "බිංදුව"` can never take its
//!    first arm — the empty string is falsy. The conditional is dead and zero
//!    is always [`ZERO_WORD`]. Reproduced as an unconditional return.
//! 3. **Bare/independent word forms are concatenated with no combining
//!    forms**, which is not how Sinhala actually builds numerals. The output
//!    reads as a digit-by-digit gloss: 11 → "දහය එක" ("ten one"), 1100 →
//!    "දහස සියය" ("thousand hundred"), 2000 → "දෙක දහස" ("two thousand"
//!    using the free form දෙක rather than the attributive දෙ). Kept verbatim.
//! 4. **No overflow guard — large values leak raw ASCII digits.** The final
//!    `else` of `_int_to_word` is `return str(number)` ("Fallback for very
//!    large numbers"), reached for every `number >= 1_000_000_000`. So
//!    `to_cardinal(10**9)` == "1000000000" — decimal digits, not words — and
//!    `to_ordinal(10**9)` == "1000000000 වැනි", a bare numeral with a Sinhala
//!    ordinal suffix glued on. This module therefore **never raises**: no
//!    input reaches an `OverflowError`, so there is no `N2WError` path here at
//!    all. Corpus-confirmed up to 10^21.
//! 5. **`to_ordinal_num` skips `verify_ordinal`**, so negatives pass straight
//!    through the `str(number) + "."` template: `to_ordinal_num(-1)` == "-1."
//!    rather than the `TypeError` (`errmsg_negord`) that `verify_ordinal`
//!    exists to raise. `to_ordinal` likewise accepts negatives:
//!    `to_ordinal(-1)` == "සෘණ එක වැනි". Both corpus-confirmed.
//! 6. **`_int_to_word`'s `number < 0` arm is dead code.** `to_cardinal` strips
//!    the sign from the *string* before calling `int()`, and every recursive
//!    call passes a quotient or remainder that is non-negative by
//!    construction, so nothing can reach it. It is reproduced in
//!    [`LangSi::int_to_word`] for fidelity but is unreachable from the four
//!    in-scope entry points. (Were it reachable it would emit a *doubled*
//!    prefix, since `to_cardinal` already prepends `negword` itself.)
//! 7. **Every unknown currency code silently becomes rupees.**
//!    `to_currency` looks the code up with
//!    `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["LKR"])`, so it
//!    *cannot* raise: `to_currency(1, "GBP")` == "එක රුපියල්", and so do JPY,
//!    KWD, BHD, INR, CNY and CHF. Only LKR/USD/EUR are real entries. This is
//!    the single largest divergence from every other language in the library,
//!    where an unknown code is a NotImplementedError. `to_cheque` is *not*
//!    overridden and uses the strict `[]` lookup, so the same GBP that renders
//!    as rupees through `to_currency` raises through `to_cheque` — both halves
//!    corpus-confirmed.
//! 8. **Floats whose `repr` is exponential raise ValueError.** `to_currency`
//!    parses `str(val)`, so `1e-05` -> `int("1e-05")` -> `ValueError: invalid
//!    literal for int() with base 10: '1e-05'` rather than any num2words
//!    error. Same for `1e+16` and up. See the "Known divergence" note below —
//!    this one is *not* reproduced.
//! 9. **`CURRENCY_PRECISION` is ignored by `to_currency`.** The two-digit
//!    slice `parts[1][:2]` hard-wires a divisor of 100, so 3-decimal
//!    currencies lose their third digit and 0-decimal ones gain cents anyway.
//!    `to_cheque` *does* consult it — but SI's is `{}`, so it defaults to 100
//!    for every code and the trait's default hook is already correct. Nothing
//!    to override.
//! 10. **`adjective=True` does nothing.** `CURRENCY_ADJECTIVES` is empty and
//!    `to_currency` never reads it, so the parameter is accepted and dropped.
//! 11. **`1.0` renders no cents, for the wrong reason.**
//!    `Num2Word_Base.to_currency` shows zero cents on a float ("one euro,
//!    zero cents"); SI's `if cents and right:` treats `right == 0` as falsy and
//!    skips the segment, so `1.0` and the int `1` coincide at "එක euro". The
//!    int/float distinction still has to be honoured — `0.5` and `0.01` take
//!    the decimal branch and depend on it.
//!
//! 12. **Whole floats keep their ".0" tail.** Routing is `"." in str(number)`,
//!    and `str(5.0)` is `"5.0"`, so `to_cardinal(5.0)` == "පහ දශම බිංදුව" —
//!    never Base's whole-value integer route. `Decimal("5")` (str `"5"`, no
//!    dot) *does* take the integer path. `cardinal_float_entry` carries this
//!    routing; `to_ordinal`/`to_year` inherit it by composition
//!    (`to_ordinal(5.0)` == "පහ දශම බිංදුව වැනි"), and `to_ordinal_num(5.0)`
//!    suffixes the raw repr: "5.0.".
//! 13. **Exponential string forms raise ValueError on the worded modes.**
//!    `str(1e16)` == "1e+16" and `str(Decimal("1E+2"))` == "1E+2" contain no
//!    `"."`, so they fall through to `int(n)` — `ValueError: invalid literal
//!    for int() with base 10: '1e+16'`. The same route kills string input
//!    "1e3" (`Decimal("1e3")` -> str "1E+3" -> int) and "Infinity"/"NaN"
//!    (`int("Infinity")`/`int("NaN")`). `to_ordinal_num` never calls `int()`,
//!    so `to_ordinal_num(1e16)` == "1e+16." succeeds where `to_ordinal(1e16)`
//!    raises. Corpus-pinned for 1e+16/1e+20 (float), 1E+2/1E+20 (Decimal) and
//!    "1e3"/"1E3"/"Infinity"/"-Infinity"/"NaN" (str).
//!
//! # Error variants
//!
//! The four integer modes never raise: per bug 4 this module has no ceiling
//! and no `raise` on any integer path, so `to_cardinal`/`to_ordinal`/
//! `to_ordinal_num`/`to_year` are total over `BigInt` and the corpus has zero
//! `ok: false` rows for them. The float/Decimal *entries* are another story:
//! bug 13's exponential forms surface [`N2WError::Value`] through
//! [`parse_int`].
//!
//! The currency surface adds exactly one reachable error, and it is not SI's:
//! `to_cheque` on an unknown code raises [`N2WError::NotImplemented`] with
//! `Currency code "X" not implemented for "Num2Word_SI"`, raised by
//! `currency::default_to_cheque` off this file's [`Lang::currency_forms`]
//! returning `None`. `to_currency` cannot raise at all (bug 7).
//!
//! [`N2WError::Value`] appears in [`parse_int`] to type bug 8's `ValueError`
//! correctly, but is unreachable — see below.
//!
//! # Known divergence (bug 8)
//!
//! Python's `to_currency` is driven by `str(val)`, and `repr(float)` switches
//! to exponential notation at `|x| >= 1e16` and `0 < |x| < 1e-4`. `int("1e-05")`
//! then raises `ValueError`. This port **cannot** reproduce that: by the time
//! the value reaches Rust it is a `BigDecimal`, and
//! `BigDecimal::from_str("1e-05")` and `from_str("0.00001")` are the *same*
//! value (digits 1, scale 5) — the notation is gone. Reproducing it would also
//! be wrong for `Decimal` operands, where `str(Decimal("0.00001"))` is
//! `"0.00001"` and Python succeeds. So [`plain_decimal_string`] always renders
//! plain notation, which is right for `Decimal` input and wrong for
//! exponential-`repr` floats: SI returns a string where Python raises.
//! The corpus contains no such row for any language (its `arg`s are all plain),
//! so this is untested either way, and closing it would need the notation
//! carried across the boundary — a `currency.rs` change, out of scope here.
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_SI` reads only the immutable tables `setup()` installs; it
//! sets no flag in one method for another to consume. (`Num2Word_Base` mutates
//! `self.precision` in `float2tuple`, but that is the float path, which is out
//! of scope and unreachable from integer input.)
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use ඩොලර් / යූරෝ with ශත. Sinhala puts the noun
//! before the number ("ඩොලර් 2"); the number-first order is unchanged and
//! still open. Examples in these docs that quote English nouns record Python's
//! output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `setup(): self.negword = "minus "`, here in Sinhala (#154), trailing space
/// included.
///
/// Note SI's `to_cardinal` concatenates `self.negword` **raw**, unlike
/// `Num2Word_Base.to_cardinal`/`parse_minus`, which normalise via
/// `"%s " % self.negword.strip()`. Here the two happen to coincide because the
/// literal already carries exactly one trailing space.
const NEGWORD: &str = "සෘණ ";

/// `setup(): self.pointword = "point"`, here the Sinhala දශම (#154). Used by
/// the float path.
const POINTWORD: &str = "දශම";

/// The `else` arm of `_int_to_word`'s zero ternary — the only reachable one
/// (see bug 2).
const ZERO_WORD: &str = "බිංදුව";

/// `setup(): self.ones`. Index 0 is `""` and is never read as a word: callers
/// only index this with 1..=9, because `number == 0` returns earlier and the
/// hundreds/lakh/crore arms guard their `== 1` case separately.
const ONES: [&str; 10] = [
    "", "එක", "දෙක", "තුන", "හතර", "පහ", "හය", "හත", "අට", "නවය",
];

/// `setup(): self.tens`. Index 0 is `""` and unreachable: the tens arm runs
/// only for `10 <= number < 100`, so `number // 10` is 1..=9.
///
/// Index 1 is "දහය" — the standalone word for *ten*, which is why 11 comes out
/// as "දහය එක" rather than a fused teen (bug 3).
const TENS: [&str; 10] = [
    "", "දහය", "විස්ස", "තිහ", "හතළිහ", "පනහ", "හැට", "හැත්තෑව", "අසූව", "අනූව",
];

/// `setup(): self.hundred` (10^2).
const HUNDRED: &str = "සියය";
/// `setup(): self.thousand` (10^3).
const THOUSAND: &str = "දහස";
/// `setup(): self.lakh` (10^5).
const LAKH: &str = "ලක්ෂය";
/// `setup(): self.crore` (10^7).
const CRORE: &str = "කෝටිය";

/// `to_ordinal`'s suffix: `return self.to_cardinal(number) + " වැනි"`.
const ORDINAL_SUFFIX: &str = " වැනි";

/// The class name, for `to_cheque`'s NotImplementedError message.
const LANG_NAME: &str = "Num2Word_SI";

/// The `CURRENCY_FORMS` key SI's `to_currency` falls back to for *every*
/// unknown code — see [`LangSi::forms_or_lkr`] and bug 7.
const LKR: &str = "LKR";

pub struct LangSi {
    /// `Num2Word_SI.CURRENCY_FORMS`, built once in [`LangSi::new`].
    ///
    /// Held on the struct rather than rebuilt per call: the py binding keeps
    /// one `OnceLock<LangSi>` per process, so this table is allocated exactly
    /// once for the program's lifetime and `to_currency`/`to_cheque` only
    /// borrow from it.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangSi {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSi {
    pub fn new() -> Self {
        // `Num2Word_SI.CURRENCY_FORMS` verbatim.
        //
        // Note SI defines this dict *itself* rather than inheriting
        // `Num2Word_EUR`'s. That matters: `Num2Word_EN.__init__` mutates
        // `Num2Word_EUR.CURRENCY_FORMS` in place at import time, rewriting EUR
        // and GBP and adding ~24 codes, and every class sharing that dict sees
        // the result. SI shares nothing with it (verified: `c.CURRENCY_FORMS
        // is lang_EUR.Num2Word_EUR.CURRENCY_FORMS` -> False), so it sees these
        // three codes and only these three — no AUD/JPY/KWD, and its own
        // ("euro", "euros") literal is what runs.
        //
        // Both LKR forms are identical, as are both සත forms: Sinhala does not
        // inflect these for number, so the `!= 1` selection below is a no-op
        // for LKR and only bites for the two English-worded codes.
        let mut forms = HashMap::new();
        forms.insert(
            LKR,
            CurrencyForms::new(&["රුපියල්", "රුපියල්"], &["සත", "සත"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["ඩොලර්", "ඩොලර්"], &["ශත", "ශත"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["යූරෝ", "යූරෝ"], &["ශත", "ශත"]),
        );
        LangSi { forms }
    }

    /// `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["LKR"])` — the
    /// *lenient* lookup used only by `to_currency` (bug 7).
    ///
    /// Contrast [`Lang::currency_forms`], which is the strict `[]` lookup
    /// `to_cheque` needs. Keeping the two separate is what lets
    /// `to_currency("GBP")` succeed while `to_cheque("GBP")` raises.
    ///
    /// The `expect` encodes a constructor invariant, not an input check:
    /// `new()` always installs `LKR`, so the fallback can never be absent.
    /// (Python would raise KeyError here for the same reason it cannot.)
    fn forms_or_lkr(&self, code: &str) -> &CurrencyForms {
        self.forms
            .get(code)
            .or_else(|| self.forms.get(LKR))
            .expect("new() always installs CURRENCY_FORMS[\"LKR\"] as the fallback")
    }

    /// Port of `Num2Word_SI._int_to_word`.
    ///
    /// The `number >= 1_000_000_000` fallback (bug 4) is the only place a
    /// `BigInt` escapes the `u64` fast path, and it just stringifies. Every
    /// other arm is provably bounded by 10^9, so narrowing to `u64` there is
    /// sound rather than a lossy cast: `to_u64()` returns `None` for anything
    /// above `u64::MAX`, which is far past 10^9 and so lands in the same
    /// fallback Python would take.
    fn int_to_word(&self, number: &BigInt) -> String {
        // Python: `if number == 0: return self.ones[0] if self.ones[0] else "බිංදුව"`
        // — ones[0] is "" (falsy), so this is unconditionally ZERO_WORD (bug 2).
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // Python: `if number < 0: return self.negword + self._int_to_word(abs(number))`
        // Dead code from the in-scope entry points (bug 6), reproduced anyway.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        match number.to_u64() {
            // Every worded arm of the Python cascade lives below 10^9.
            Some(n) if n < 1_000_000_000 => self.bounded_to_word(n),
            // Python's `else: return str(number)  # Fallback for very large numbers`
            _ => number.to_string(),
        }
    }

    /// The `1 <= number < 1_000_000_000` arms of `_int_to_word`.
    ///
    /// Split out from [`LangSi::int_to_word`] purely so the arithmetic can use
    /// `u64`. Callers only ever reach it with `n >= 1`:
    ///   * `int_to_word` returns early on 0;
    ///   * the quotient recursions (`thousands_val`/`lakh_val`/`crore_val`)
    ///     run only in the `!= 1` branch of a value already `>=` that unit, so
    ///     the quotient is `>= 2`;
    ///   * the remainder recursions are all guarded by `if remainder:`.
    ///
    /// So `ONES[0]`/`TENS[0]` (both `""`) are never read.
    fn bounded_to_word(&self, n: u64) -> String {
        if n < 10 {
            // Python: `elif number < 10: return self.ones[number]`
            ONES[n as usize].to_string()
        } else if n < 100 {
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                TENS[tens_val].to_string()
            } else {
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            }
        } else if n < 1_000 {
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            let mut result = if hundreds_val == 1 {
                HUNDRED.to_string()
            } else {
                format!("{} {}", ONES[hundreds_val], HUNDRED)
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.bounded_to_word(remainder));
            }
            result
        } else if n < 100_000 {
            // 10^3 .. 10^5: thousands_val is 1..=99, so this covers e.g.
            // 10_000 -> "දහය දහස" ("ten thousand").
            let thousands_val = n / 1_000;
            let remainder = n % 1_000;
            let mut result = if thousands_val == 1 {
                THOUSAND.to_string()
            } else {
                format!("{} {}", self.bounded_to_word(thousands_val), THOUSAND)
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.bounded_to_word(remainder));
            }
            result
        } else if n < 10_000_000 {
            // 10^5 .. 10^7: lakh_val is 1..=99. 10^6 has no word of its own —
            // it renders as "දහය ලක්ෂය" ("ten lakh").
            let lakh_val = n / 100_000;
            let remainder = n % 100_000;
            let mut result = if lakh_val == 1 {
                LAKH.to_string()
            } else {
                format!("{} {}", self.bounded_to_word(lakh_val), LAKH)
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.bounded_to_word(remainder));
            }
            result
        } else {
            // 10^7 .. 10^9: crore_val is 1..=99. 10^8 -> "දහය කෝටිය".
            let crore_val = n / 10_000_000;
            let remainder = n % 10_000_000;
            let mut result = if crore_val == 1 {
                CRORE.to_string()
            } else {
                format!("{} {}", self.bounded_to_word(crore_val), CRORE)
            };
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.bounded_to_word(remainder));
            }
            result
        }
    }
}

/// Reproduce Python's `str(val)` for a `Decimal`/float operand of
/// `to_currency`, in plain (non-exponential) notation.
///
/// SI's `to_currency` is *string*-driven — it splits `str(val)` on `"."` and
/// slices the fractional digits — so the port has to rebuild the same string.
/// The `BigDecimal` already carries `str(value)`'s exact digits and scale
/// (`currency.rs` parses the Python-side string rather than a binary double),
/// so this is a re-render of that, not a second float-formatting algorithm.
///
/// Crucially it must **not** normalise: `1.0` has to stay `"1.0"` (scale 1),
/// because Python's `str(1.0)` is `"1.0"` and the presence of a `"."` is what
/// the `len(parts) > 1` test keys on. `BigDecimal::from_str` preserves scale,
/// and `as_bigint_and_exponent` hands it back untouched.
///
/// `magnitude()` drops the sign because callers pass an already-`abs()`-ed
/// value (Python does `val = abs(val)` before `str(val)`); it is belt-and-
/// braces against a sign leaking into the digit string.
///
/// Byte/char note: `digits.to_string()` is pure ASCII decimal digits, so
/// `len()` and `split_at` are character-safe here.
fn plain_decimal_string(d: &BigDecimal) -> String {
    // value == digits * 10^(-scale)
    let (digits, scale) = d.as_bigint_and_exponent();
    let s = digits.magnitude().to_string();

    if scale <= 0 {
        // Integral with `-scale` trailing zeros and no ".". Unreachable from
        // `repr(float)` (which always emits one, e.g. "100.0") but reachable
        // from a Decimal like Decimal("1E+2").
        let mut out = s;
        out.push_str(&"0".repeat((-scale) as usize));
        return out;
    }

    let scale = scale as usize;
    if s.len() > scale {
        let (int_part, frac_part) = s.split_at(s.len() - scale);
        format!("{}.{}", int_part, frac_part)
    } else {
        // Sub-unit magnitude: pad out the leading zeros Python prints,
        // e.g. digits=1 scale=2 -> "0.01".
        format!("0.{}{}", "0".repeat(scale - s.len()), s)
    }
}

/// Sign-free `str(value)` for a *float* whose repr shows **no** decimal point
/// — i.e. `FloatValue::has_visible_point()` said no. For a finite f64 that
/// means "whole and `|v| >= 1e16`", where CPython's repr switches to exponent
/// form (`"1e+16"`); non-finite values print `"inf"`/`"nan"` (the sign having
/// been peeled off by the caller, as Python's `n[1:]` does textually).
///
/// The mantissa is rebuilt from the float's **exact** integer digits (every
/// whole f64 is exactly representable), not CPython's shortest-round-trip
/// digits, so for values whose exact expansion is longer than the shortest
/// repr the text can differ from Python's — harmless, because every output of
/// this function contains a non-digit (`e`/`.`/`inf`/`nan`) and exists only to
/// be fed to [`parse_int`], which raises the same ValueError for either
/// spelling. For the corpus-pinned cases (1e+16, 1e+20) they coincide exactly.
fn float_no_point_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return "inf".to_string();
    }
    // Finite without a visible point => whole and |v| >= 1e16, so this BigInt
    // conversion is exact.
    let digits = BigInt::from_f64(v.abs())
        .expect("finite whole f64 converts exactly")
        .to_string();
    let exp = digits.len() - 1; // >= 16, so always two+ exponent digits
    let mant = digits.trim_end_matches('0');
    let mant = if mant.len() <= 1 {
        // Power of ten: bare leading digit, no ".".
        digits[..1].to_string()
    } else {
        format!("{}.{}", &mant[..1], &mant[1..])
    };
    format!("{}e+{}", mant, exp)
}

/// Python's `int(token)` on a digit string, with its exact ValueError.
///
/// ValueError (not TypeError) is what `int("1e-05")` raises, hence
/// [`N2WError::Value`]. Unreachable from [`plain_decimal_string`]'s output,
/// which is always parseable — but *reachable* from `cardinal_float_entry`'s
/// no-visible-point branch, where the exponential spellings of bug 13
/// ("1e+16", "1E+2", "inf") land exactly the way Python's `int(n)` sees them.
fn parse_int(token: &str) -> Result<BigInt> {
    BigInt::from_str(token).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            token
        ))
    })
}

impl Lang for LangSi {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "LKR"
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

    /// `self.__class__.__name__`, interpolated into `to_cheque`'s
    /// NotImplementedError.
    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// The **strict** `self.CURRENCY_FORMS[currency]` lookup.
    ///
    /// Only `Num2Word_Base.to_cheque` reaches this — SI overrides
    /// `to_currency` and routes it through the lenient
    /// [`LangSi::forms_or_lkr`] instead. So a missing code raises here (via
    /// `to_cheque`'s `except KeyError -> NotImplementedError`) while sailing
    /// through `to_currency`. Corpus: `cheque:GBP` is NotImplementedError but
    /// `currency:GBP` returns rupees.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// Port of `Num2Word_SI.to_cardinal`, integer path only.
    ///
    /// Python works on `n = str(number).strip()`: it peels a leading `"-"` off
    /// the *string*, stashes `self.negword` in `ret`, then checks for `"."`.
    /// `str(int)` never contains a `"."`, so integers always take the `else`
    /// branch: `return (ret + self._int_to_word(int(n))).strip()`. The float
    /// branch (`pointword` + per-digit decimals) is out of scope.
    ///
    /// The trailing `.strip()` is reproduced as `trim()` even though it is a
    /// no-op on every reachable value — `negword`'s trailing space is always
    /// followed by a word, and `_int_to_word` never returns padding. Kept so
    /// the port matches the source line-for-line.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            // Python: `n = n[1:]; ret = self.negword`
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&n)).trim().to_string())
    }

    /// Port of `Num2Word_SI.to_ordinal`: `to_cardinal(number) + " වැනි"`.
    ///
    /// No `verify_ordinal` call, so negatives and the raw-digit fallback both
    /// flow through: `to_ordinal(-1)` == "සෘණ එක වැනි" and
    /// `to_ordinal(10**9)` == "1000000000 වැනි" (bugs 4 and 5).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_SI.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Note this overrides `Num2Word_Base.to_ordinal_num`, which returns the
    /// value *unchanged*; SI stringifies and appends a period. Negatives are
    /// not rejected (bug 5): `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_SI.to_year`: `def to_year(self, val, longval=True):
    /// return self.to_cardinal(val)`. `longval` is accepted and ignored, so
    /// there is no era handling and no two-digit pairing — `to_year(1999)` is
    /// just the cardinal "දහස නවය සියය අනූව නවය", and `to_year(-500)` is
    /// "සෘණ පහ සියය" rather than anything BC-flavoured.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of the **float/Decimal** branch of `Num2Word_SI.to_cardinal`.
    ///
    /// `Num2Word_SI` overrides `to_cardinal` and handles non-integers inline —
    /// it never overrides `to_cardinal_float`, never calls `float2tuple`, and
    /// never reads `self.precision`. It works purely on `str(number)`:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"): n = n[1:]; ret = self.negword
    /// else:                 ret = ""
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
    /// The Rust trait routes float/Decimal input here because the integer
    /// `to_cardinal` above only accepts `&BigInt`, so this method carries SI's
    /// string-driven code path.
    ///
    /// **Why not inherit `default_to_cardinal_float`.** Base's `float2tuple`
    /// computes `abs(value - pre) * 10**precision` in binary f64, which loses
    /// the fractional digits once the magnitude passes ~1e13: e.g.
    /// `164489259578107.2` yields `.1` through base but `.2` through SI, and
    /// `221811789179999.88` yields `.87` vs `.88`. SI reads the digits straight
    /// out of `str(value)`'s shortest round-trip repr, so it stays exact. The
    /// two disagree on ~8% of random floats above 1e13 — that divergence is why
    /// this override exists.
    ///
    /// **Reconstructing `str(number)`.**
    /// * *Float*: the shortest round-trip repr's fractional part has exactly
    ///   `precision` digits (that is how `precision` was derived, Python-side,
    ///   as `abs(Decimal(str(value)).as_tuple().exponent)`), so
    ///   `format!("{:.p}", |v|)` reproduces it byte for byte. Verified equal to
    ///   CPython `repr` on 8k+ values including the large-magnitude ones above.
    ///   The sign is read from the **sign bit** (`is_sign_negative`), not
    ///   `v < 0`, because Python keys on `str(number)` and `str(-0.0)` is
    ///   `"-0.0"` — SI prepends the negword for negative zero (`num2words(-0.0,
    ///   lang='si')` == "සෘණ බිංදුව දශම බිංදුව").
    /// * *Decimal*: the arm is exact; [`plain_decimal_string`] re-renders the
    ///   Decimal's own digits and scale in plain notation, matching
    ///   `str(Decimal)` for every non-exponential value — all five corpus
    ///   `cardinal_dec` rows, including the trailing-zero case `1.10` (scale is
    ///   preserved, so `right == "10"`) and `98746251323029.99`, whose integer
    ///   part exceeds 1e9 and therefore leaks raw digits through
    ///   [`LangSi::int_to_word`]'s bug-4 fallback: "98746251323029 දශම නවය නවය".
    ///   The exponential-`str(Decimal)` case (e.g. `Decimal("1E+2")`) is the
    ///   same untested divergence flagged as bug 8 for `to_currency`.
    ///
    /// **`precision_override` is ignored**, exactly as Python: `to_cardinal`
    /// never reads `self.precision`, so the `precision=` kwarg — which the
    /// dispatcher applies by mutating `self.precision` — has no effect on SI's
    /// output (`num2words(1.23456, lang='si', precision=3)` still emits all six
    /// digits).
    ///
    /// `parse_int` is reused for `int(left)` and each `int(digit)`, preserving
    /// Python's `ValueError` on a non-numeric token (reachable only from the
    /// exponential-repr edge above, never from `format!`/`plain_decimal_string`
    /// output).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Rebuild Python's `n = str(number).strip()` as (had-leading-minus,
        // unsigned decimal string).
        let (is_neg, n) = match value {
            FloatValue::Float { value: v, precision } => (
                v.is_sign_negative(),
                format!("{:.*}", *precision as usize, v.abs()),
            ),
            FloatValue::Decimal { value: d, .. } => {
                (d.is_negative(), plain_decimal_string(&d.abs()))
            }
        };

        // Python: `ret = self.negword` (spliced raw, trailing space included)
        // when the string started with "-", else "".
        let neg_prefix = if is_neg { NEGWORD } else { "" };

        // Python: `if "." in n:`
        if let Some((left, right)) = n.split_once('.') {
            // ret += self._int_to_word(int(left)) + " " + self.pointword + " "
            let mut ret = String::from(neg_prefix);
            ret.push_str(&self.int_to_word(&parse_int(left)?));
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // for digit in right: ret += self._int_to_word(int(digit)) + " "
            for ch in right.chars() {
                let d = parse_int(&ch.to_string())?;
                ret.push_str(&self.int_to_word(&d));
                ret.push(' ');
            }
            // return ret.strip()
            Ok(ret.trim().to_string())
        } else {
            // Python: `return (ret + self._int_to_word(int(n))).strip()`.
            // Reached only for input rebuilt without a "." — an integer-valued
            // Decimal such as Decimal("5") (str "5"); non-exponential floats
            // always carry at least one fractional digit.
            let val = parse_int(&n)?;
            Ok(format!("{}{}", neg_prefix, self.int_to_word(&val))
                .trim()
                .to_string())
        }
    }

    /// `to_cardinal(float/Decimal)` FULL routing — module docs, bugs 12/13.
    ///
    /// Python's `to_cardinal` is string-driven: `"." in str(number)` picks the
    /// decimal grammar, and `str(5.0)` is `"5.0"`, so **whole floats keep
    /// their ".0" tail** ("පහ දශම බිංදුව") instead of taking Base's
    /// whole-value integer route. Without a visible point the sign-free string
    /// lands in `int(n)`:
    ///   * `Decimal("5")` -> `"5"` -> the integer path ("පහ");
    ///   * repr-exponential floats (`str(1e16)` == "1e+16") and exponential
    ///     Decimals (`str(Decimal("1E+2"))` == "1E+2") -> `int()` ValueError,
    ///     typed and worded by [`parse_int`].
    ///
    /// The Decimal arm reconstructs the string with
    /// [`python_decimal_str`] (scientific-aware) rather than
    /// [`plain_decimal_string`] (plain-only): `Decimal("1E+2")` must stay
    /// "1E+2" so `int()` fails, not normalise to "100" and succeed.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        // Python: `ret = self.negword` when the string started with "-", else
        // "" — sign-bit aware for the Float arm, exactly like the "." branch.
        let neg_prefix = if value.is_negative() { NEGWORD } else { "" };
        let text = match value {
            FloatValue::Float { value: v, .. } => float_no_point_repr(*v),
            FloatValue::Decimal { value: d, .. } => python_decimal_str(&d.abs()),
        };
        // Python: `return (ret + self._int_to_word(int(n))).strip()`
        let val = parse_int(&text)?;
        Ok(format!("{}{}", neg_prefix, self.int_to_word(&val))
            .trim()
            .to_string())
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + " වැනි"` with no type guard, so floats get
    /// the full decimal phrase plus the suffix ("පහ දශම බිංදුව වැනි") and
    /// bug 13's ValueError propagates unchanged for exponential forms.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, no `int()`
    /// anywhere — so it succeeds where the other modes raise ("1e+16.") and
    /// "-0.0" keeps its textual minus ("-0.0."). `repr_str` is the Python-side
    /// `str(number)`, carried across by the binding because repr(float) /
    /// str(Decimal) are Python's to recompute, not ours.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is `return self.to_cardinal(val)`
    /// (`longval` ignored), so the full float routing above applies verbatim,
    /// ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which SI does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError. The
    /// binding otherwise maps `ParsedNumber::Inf` to the base integer path's
    /// OverflowError before any SI code runs, so the ValueError must be
    /// raised here. (NaN needs no interception: the binding's ValueError
    /// already matches `int("NaN")`'s type.)
    ///
    /// Known gap (unpinned): Python's `to_ordinal_num(Decimal("Infinity"))`
    /// would be "Infinity."; this entry-level interception raises instead.
    /// The strings corpus pins Infinity under `to=cardinal` only.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Port of `Num2Word_SI.to_currency`, which replaces
    /// `Num2Word_Base.to_currency` wholesale.
    ///
    /// SI shares almost nothing with the base implementation, so
    /// `currency::default_to_currency` is deliberately *not* delegated to:
    ///
    /// * **No `parse_currency_parts`, no `CURRENCY_PRECISION`, no rounding.**
    ///   SI splits `str(val)` on `"."` and takes `parts[1][:2]`, so the divisor
    ///   is hard-wired to 100 by the slice width. KWD/BHD (3-decimal) and JPY
    ///   (0-decimal) get plain cents like everything else — see bugs 7 and 9.
    /// * **No `pluralize`.** It selects `cr[1] if n != 1 else cr[0]` inline, so
    ///   the abstract `pluralize` is never reached and its NotImplementedError
    ///   never fires.
    /// * **No `has_fractional_cents` branch**, hence no call into
    ///   `cardinal_from_decimal`. Truncating at two digits means sub-cent
    ///   precision is discarded rather than rendered, so the float path stays
    ///   unreachable and `cardinal_from_decimal` correctly keeps its default.
    /// * **`adjective` is accepted and ignored** — `CURRENCY_ADJECTIVES` is
    ///   empty and never consulted (bug 10).
    ///
    /// The `Int`/`Decimal` split is preserved exactly as Python's implicit one:
    /// `str(1)` is `"1"` (no `"."`, so `len(parts) == 1`, so `right = 0`, so no
    /// cents segment), while `str(1.0)` is `"1.0"`. SI reaches the same
    /// "no cents" answer for `1.0` by a different route than `base` — see bug
    /// 11 — but the two must not be collapsed, because `0.5` and `0.01` do
    /// depend on the decimal branch.
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
        // Python: `if val < 0: is_negative = True; val = abs(val)`
        let is_negative = val.is_negative();

        // Python: `parts = str(val).split(".")` — of the *absolute* value, the
        // sign having just been stripped above.
        let s = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => plain_decimal_string(&d.abs()),
        };
        // `splitn(2, '.')` is Python's `split(".")` for this input: `str()` of
        // a number yields at most one ".", so the cap never truncates.
        let mut parts = s.splitn(2, '.');
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next(); // None <=> Python's `len(parts) > 1` is False

        // Python: `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            parse_int(p0)?
        };

        // Python: `right = int(parts[1][:2].ljust(2, "0"))
        //          if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` **truncates**, it does not round: 1.239 -> "23" -> twenty-three
        // cents, 1.999 -> "99". `ljust` then right-pads, so "5" -> "50" (0.5 is
        // fifty cents, not five). Both verified against the interpreter.
        let right = match p1 {
            Some(f) if !f.is_empty() => {
                let mut t: String = f.chars().take(2).collect();
                while t.chars().count() < 2 {
                    t.push('0');
                }
                parse_int(&t)?
            }
            _ => BigInt::zero(),
        };

        // Python: `cr1, cr2 = self.CURRENCY_FORMS.get(currency,
        //                                             self.CURRENCY_FORMS["LKR"])`
        let forms = self.forms_or_lkr(currency);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // Python: `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        // Indexing mirrors Python's; every SI entry has exactly two forms, so
        // neither index can be out of range.
        let left_str = self.to_cardinal(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // Python: `if cents and right:` — `right` is an int, so this is
        // "cents requested AND the cents are non-බිංදුව". Note `cents=False`
        // drops the segment outright rather than falling back to
        // `_cents_terse`, which is why SI never calls it.
        if cents && !right.is_zero() {
            let cents_str = self.to_cardinal(&right)?;
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // Python: `return (self.negword if is_negative else "") + result`
        // NEGWORD is spliced **raw**, with its trailing space and without the
        // `negword.strip() + " "` normalisation `default_to_currency` applies.
        // They coincide only because the literal is already "සෘණ ".

        Ok(format!(
            "{}{}",
            if is_negative { NEGWORD } else { "" },
            result
        ))
    }
}

#[cfg(test)]
mod float_entry_tests {
    use super::*;
    use std::str::FromStr as _;

    fn fv(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn dv(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// The float-entry routing rows of bench/corpus_wholefloat.jsonl and
    /// bench/corpus_strings.jsonl (bugs 12/13).
    #[test]
    fn entry_routing() {
        let l = LangSi::new();
        // Whole float keeps its ".0" tail through every worded mode.
        assert_eq!(
            l.cardinal_float_entry(&fv(5.0, 1), None).unwrap(),
            "පහ දශම බිංදුව"
        );
        assert_eq!(
            l.ordinal_float_entry(&fv(5.0, 1)).unwrap(),
            "පහ දශම බිංදුව වැනි"
        );
        assert_eq!(l.year_float_entry(&fv(5.0, 1)).unwrap(), "පහ දශම බිංදුව");
        // ordinal_num echoes the Python repr and appends the period.
        assert_eq!(l.ordinal_num_float_entry(&fv(5.0, 1), "5.0").unwrap(), "5.0.");
        // Negative zero: sign bit -> the negword.
        assert_eq!(
            l.ordinal_float_entry(&fv(-0.0, 1)).unwrap(),
            "සෘණ බිංදුව දශම බිංදුව වැනි"
        );
        // Trailing zeros survive through the Decimal arm.
        assert_eq!(
            l.cardinal_float_entry(&dv("5.00", 2), None).unwrap(),
            "පහ දශම බිංදුව බිංදුව"
        );
        // Decimal without a point takes the integer path.
        assert_eq!(l.cardinal_float_entry(&dv("5", 0), None).unwrap(), "පහ");
        assert_eq!(l.ordinal_float_entry(&dv("100", 0)).unwrap(), "සියය වැනි");
        // Exponential forms -> int() ValueError, message quoting the literal.
        match l.cardinal_float_entry(&fv(1e16, 16), None) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // #211: str(Decimal) is written out ("100"), so it reads.
        assert_eq!(
            l.cardinal_float_entry(&dv("1E+2", 2), None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );
        assert!(matches!(
            l.year_float_entry(&fv(1e20, 20)),
            Err(N2WError::Value(_))
        ));
        // ...but ordinal_num never calls int(): the repr sails through.
        assert_eq!(l.ordinal_num_float_entry(&fv(1e16, 16), "1e+16").unwrap(), "1e+16.");
        // str_to_number: Infinity pre-empts the binding's OverflowError.
        assert!(matches!(l.str_to_number("Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(l.str_to_number("-Infinity"), Err(N2WError::Value(_))));
        // Ordinary strings still parse through the base Decimal grammar.
        assert!(matches!(l.str_to_number("1.5"), Ok(ParsedNumber::Dec(_))));
    }
}
