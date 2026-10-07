//! Port of `lang_MY.py` (Myanmar / Burmese).
//!
//! Shape: **self-contained**. `Num2Word_MY` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`, so Python never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives a hand-written
//! `_int_to_word` cascade. `cards`/`merge` therefore stay at their trait
//! defaults here. Python has **no overflow check** — see bug 2 below for what
//! it did at the top of the range; this port raises `OverflowError` from
//! 10^14 (`maxval`).
//!
//! All four in-scope modes are overridden by the Python class, so nothing is
//! inherited from `Num2Word_Base` except `setup()` being called from
//! `__init__` (which only populates `negword`/`pointword`/`ones`).
//!
//! # Encoding: why every literal is written with `\u{}` escapes
//!
//! Burmese stacks combining marks after their base consonant, and two
//! different mark orders can render **identically** while being distinct byte
//! sequences. `lang_MY.py` contains exactly such a discrepancy (bug 1), so
//! copy-pasting the visually-correct glyphs from the source would silently
//! produce the wrong bytes for every teen. Each constant below is spelled as
//! explicit codepoints, extracted from the Python source via `ast` and
//! cross-checked against the frozen corpus. **Do not "clean up" these escapes
//! into literal glyphs** — the distinction is invisible in an editor.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Bug 1 is wrong-looking but exactly what
//! Python emits, verified against `bench/corpus.jsonl`; bug 2 is fixed:
//!
//! 1. **Inconsistent combining-mark order for "ဆယ့်" (ten, with dot).** The
//!    teens (11..=19) use a *hardcoded literal* [`TEEN_PREFIX`] whose tail is
//!    `U+1037 U+103A` (MYANMAR SIGN DOT BELOW, then MYANMAR SIGN ASAT). The
//!    tens (21..=99) instead *compute* the same-looking text as
//!    `ones[t] + "ဆယ်" + "့"`, i.e. [`SAY`] `+` [`DOT`], whose tail is
//!    `U+103A U+1037` — asat then dot below, the opposite order. So
//!    `to_cardinal(11)` ends `… U+1006 U+101A U+1037 U+103A …` while
//!    `to_cardinal(21)` ends `… U+1006 U+101A U+103A U+1037 …`. Both render
//!    as "ဆယ့်"; they are not equal strings and do not compare, hash, or
//!    round-trip alike. Corpus rows for 11 and 21 confirm both orders.
//!    (The other computed joins — `"ရာ" + "့"`, `"ထောင်" + "့"` — have no
//!    competing literal, so they are internally consistent.)
//!
//! 2. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` ends with `else: return str(number)`, so
//!    `to_cardinal(10**9)` was `"1000000000"` and `to_ordinal(10**9)`
//!    `"1000000000မြောက်"`, silently. Burmese place names grow by 10^1 up to
//!    ကုဋေ (10^7) and the next one is ကောဋိ (10^14) (Wikipedia "Burmese
//!    numerals"; Wiktionary ကုဋေ), so values in between are a multiplier of
//!    ကုဋေ: the ကုဋေ branch, which already recursed on its 1..=99 multiplier,
//!    now takes any multiplier below 10^7 — 10^9 == "တစ်ရာ ကုဋေ" — and
//!    `OverflowError` is raised from 10^14. The English loan ဘီလီယံ
//!    (Wiktionary, "billion") is the alternative not used, so as not to mix
//!    the two systems.
//!
//! # Notes on faithful-but-unreachable code
//!
//! `_int_to_word`'s `if number < 0` branch is **dead** on every path,
//! currency included: `to_cardinal` strips the sign from the string before
//! calling `int()`, every recursive call passes a non-negative remainder, and
//! `to_currency` takes `abs(val)` before stringifying, so both the units and
//! the cents it parses back out are non-negative. It is reproduced in
//! [`int_to_word`] anyway so the cascade matches the Python line for line —
//! note that it would double-apply [`NEGWORD`] if it ever were reached.
//!
//! # Float / Decimal routing
//!
//! `to_cardinal` works on `str(number)` outright, so there is **no**
//! whole-value-to-integer routing and no `float2tuple`: `5.0` keeps its
//! ".0" tail ("ငါး ဒသမ သုည"), a whole `Decimal("5")` takes the integer arm,
//! and a value whose Python string is scientific ("1e+16", `Decimal("1E+2")`)
//! raises `ValueError` from `int()`. All four float-entry hooks reconstruct
//! `str(number)` ([`float_repr`] for a float, `python_decimal_str` for a
//! Decimal) and run the exact string algorithm
//! ([`LangMy::cardinal_from_str`]); `to_ordinal`/`to_year` are that plus
//! their suffix, and `to_ordinal_num` is `str(number)` plus the suffix.
//!
//! `str_to_number` is Base's plain `Decimal(value)` — not overridden in
//! Python — but `Decimal("Infinity")`/`Decimal("NaN")` produce outcomes the
//! binding's generic Inf/NaN mapping cannot express (`int("Infinity")` is a
//! *ValueError* here, not the OverflowError of `int(Decimal("Infinity"))`), so
//! the [`Lang::inf_result`] / [`Lang::nan_result`] hooks serve those two
//! natively as the ValueError `int()` raises. See below.
//!
//! # Currency
//!
//! `Num2Word_MY` overrides `to_currency` wholesale and inherits `to_cheque`
//! unchanged, which makes the two disagree in a way worth stating up front:
//! `to_currency` **cannot** raise for an unknown currency code (it falls back
//! to MMK), while `to_cheque` **does** (it indexes `CURRENCY_FORMS` and
//! translates the KeyError). `currency:GBP` and `cheque:GBP` therefore have
//! opposite fates, and the corpus pins both. See [`LangMy::to_currency`] for
//! the full list of base-class machinery it bypasses (precision, pluralize,
//! adjectives).
//!
//! Everything else the currency path could reach — `currency_precision`,
//! `currency_adjective`, `pluralize`, `money_verbose`, `cents_verbose`,
//! `cents_terse`, `cardinal_from_decimal` — is left at its trait default,
//! matching Python: `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both
//! empty for this class, and no MY code path calls the rest. In particular
//! `cardinal_from_decimal` stays unimplemented: the fractional-cents branch
//! that would need it lives in `default_to_currency`, which MY never enters.
//!
//! # Cross-call mutable state
//!
//! None. `setup()` writes only immutable tables; no method sets a flag that
//! another consumes. The Python dispatcher needs no special handling.
//! `CURRENCY_FORMS` is a class attribute that no `__init__` in MY's MRO
//! mutates (the `Num2Word_EN` aliasing trap documented in PORTING_CURRENCY.md
//! does not apply — `Num2Word_MY` subclasses `Num2Word_Base` directly and
//! defines its own table), so the runtime dict equals the source literal.
//! Verified against the live interpreter.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Tables — see the module docs before touching any escape sequence.
// ---------------------------------------------------------------------------

/// `self.negword`. Note the **trailing space**, which is load-bearing: it is
/// what separates the sign from the number ("အနုတ် တစ်").
const NEGWORD: &str = "\u{1021}\u{1014}\u{102F}\u{1010}\u{103A}\u{0020}"; // "အနုတ် "

/// `self.ones`, digits 0..=9.
const ONES: [&str; 10] = [
    "\u{101E}\u{102F}\u{100A}",                             // 0 သုည
    "\u{1010}\u{1005}\u{103A}",                             // 1 တစ်
    "\u{1014}\u{103E}\u{1005}\u{103A}",                     // 2 နှစ်
    "\u{101E}\u{102F}\u{1036}\u{1038}",                     // 3 သုံး
    "\u{101C}\u{1031}\u{1038}",                             // 4 လေး
    "\u{1004}\u{102B}\u{1038}",                             // 5 ငါး
    "\u{1001}\u{103C}\u{1031}\u{102C}\u{1000}\u{103A}",     // 6 ခြောက်
    "\u{1001}\u{102F}\u{1014}\u{1005}\u{103A}",             // 7 ခုနစ်
    "\u{101B}\u{103E}\u{1005}\u{103A}",                     // 8 ရှစ်
    "\u{1000}\u{102D}\u{102F}\u{1038}",                     // 9 ကိုး
];

/// Exactly 10 — the hardcoded `"တစ်ဆယ်"` literal.
const TEN: &str = "\u{1010}\u{1005}\u{103A}\u{1006}\u{101A}\u{103A}";

/// Prefix for 11..=19 — the hardcoded `"တစ်ဆယ့်"` literal.
///
/// **Bug 1**: tail is `U+1037 U+103A` (dot below, *then* asat). The computed
/// tens form ([`SAY`] + [`DOT`]) emits `U+103A U+1037` instead. Identical
/// glyph, different bytes. This asymmetry is Python's, and is observable.
const TEEN_PREFIX: &str = "\u{1010}\u{1005}\u{103A}\u{1006}\u{101A}\u{1037}\u{103A}";

/// `"ဆယ်"` — the tens suffix appended to `ones[t]` for 20..=99.
const SAY: &str = "\u{1006}\u{101A}\u{103A}";

/// `"့"` — MYANMAR SIGN DOT BELOW, the "and then" joiner.
const DOT: &str = "\u{1037}";

const HUNDRED: &str = "\u{101B}\u{102C}"; // ရာ
const THOUSAND: &str = "\u{1011}\u{1031}\u{102C}\u{1004}\u{103A}"; // ထောင်
const TEN_THOUSAND: &str = "\u{101E}\u{1031}\u{102C}\u{1004}\u{103A}\u{1038}"; // သောင်း
const LAKH: &str = "\u{101E}\u{102D}\u{1014}\u{103A}\u{1038}"; // သိန်း (hundred thousand)
const MILLION: &str = "\u{101E}\u{1014}\u{103A}\u{1038}"; // သန်း

/// `" ကုဋေ"` (ten million). Note the **leading space** baked into the literal.
const KUTAY: &str = "\u{0020}\u{1000}\u{102F}\u{100B}\u{1031}";

/// `"မြောက်"` — the ordinal suffix, appended with no separator.
const ORDINAL_SUFFIX: &str = "\u{1019}\u{103C}\u{1031}\u{102C}\u{1000}\u{103A}";

// --- CURRENCY_FORMS literals -----------------------------------------------
//
// Extracted from `lang_MY.py` via `ast.literal_eval` and cross-checked against
// the frozen corpus, for the same reason the tables above are escaped: the
// marks are invisible in an editor. Note [`CENT`]'s tail is `U+1037 U+103A`
// (dot below, then asat) — the same order as [`TEEN_PREFIX`] and the *opposite*
// of the computed [`SAY`] + [`DOT`]. See bug 1 in the module docs.

/// `"ကျပ်"` (kyat) — MMK unit, and the fallback unit for every unknown code.
const MMK_UNIT: &str = "\u{1000}\u{103B}\u{1015}\u{103A}";

/// `"ပြား"` (pya) — MMK subunit.
const MMK_SUB: &str = "\u{1015}\u{103C}\u{102C}\u{1038}";

/// `"ဒေါ်လာ"` (dollar) — USD unit.
const USD_UNIT: &str = "\u{1012}\u{1031}\u{102B}\u{103A}\u{101C}\u{102C}";

/// `"ယူရို"` (euro) — EUR unit.
const EUR_UNIT: &str = "\u{101A}\u{1030}\u{101B}\u{102D}\u{102F}";

/// `"ဆင့်"` (cent) — the subunit shared by USD and EUR.
const CENT: &str = "\u{1006}\u{1004}\u{1037}\u{103A}";

/// `" ခုနှစ်"` — the year suffix. Leading space is part of the literal.
const YEAR_SUFFIX: &str = "\u{0020}\u{1001}\u{102F}\u{1014}\u{103E}\u{1005}\u{103A}";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Index [`ONES`] with a `BigInt` the caller has **proven** to be in `0..=9`.
///
/// Every call site is guarded by a range check inherited from the Python
/// cascade (e.g. `100 <= n < 1000` forces `n / 100` into `1..=9`), so the
/// `expect` is unreachable rather than a latent panic. This is the only place
/// a `BigInt` narrows to a fixed-width int, and it never does so on a value
/// that could exceed one digit.
fn ones_at(n: &BigInt) -> &'static str {
    let i = n.to_usize().expect("guarded by the caller's range check: 0..=9");
    ONES[i]
}

/// Python's `_int_to_word`.
///
/// Python's final `else` (bug 2) returned the digits from 10^9; the ကုဋေ
/// branch now covers everything below the 10^14 maxval (#147).
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1_000);
    let ten_thousand = BigInt::from(10_000);
    let lakh = BigInt::from(100_000);
    let million = BigInt::from(1_000_000);
    let ten_million = BigInt::from(10_000_000);

    if number.is_zero() {
        return Ok(ONES[0].to_string());
    }

    if number.is_negative() {
        // Dead for all in-scope modes (see module docs); ported for fidelity.
        return Ok(format!("{}{}", NEGWORD, int_to_word(&number.abs())?));
    } else if *number < ten {
        return Ok(ones_at(number).to_string());
    } else if *number == ten {
        return Ok(TEN.to_string());
    } else if *number < BigInt::from(20) {
        // Bug 1 lives here: TEEN_PREFIX's mark order differs from SAY + DOT.
        return Ok(format!("{}{}", TEEN_PREFIX, ones_at(&(number - &ten))));
    } else if *number < hundred {
        let tens_val = number / &ten;
        let ones_val = number % &ten;
        let tens_word = format!("{}{}", ones_at(&tens_val), SAY);
        if ones_val.is_zero() {
            return Ok(tens_word);
        } else {
            return Ok(format!("{}{}{}", tens_word, DOT, ones_at(&ones_val)));
        }
    } else if *number < thousand {
        let hundreds_val = number / &hundred;
        let remainder = number % &hundred;
        let mut result = format!("{}{}", ones_at(&hundreds_val), HUNDRED);
        if !remainder.is_zero() {
            result.push_str(DOT);
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    } else if *number < ten_thousand {
        let thousands_val = number / &thousand;
        let remainder = number % &thousand;
        let mut result = format!("{}{}", ones_at(&thousands_val), THOUSAND);
        if !remainder.is_zero() {
            result.push_str(DOT);
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    } else if *number < lakh {
        // From here up the joiner is a space, not the dot.
        let ten_thousands_val = number / &ten_thousand;
        let remainder = number % &ten_thousand;
        let mut result = format!("{}{}", ones_at(&ten_thousands_val), TEN_THOUSAND);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    } else if *number < million {
        let hundred_thousands_val = number / &lakh;
        let remainder = number % &lakh;
        let mut result = format!("{}{}", ones_at(&hundred_thousands_val), LAKH);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    } else if *number < ten_million {
        let millions_val = number / &million;
        let remainder = number % &million;
        let mut result = format!("{}{}", ones_at(&millions_val), MILLION);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    } else {
        // The only branch that *recurses* on the head instead of indexing
        // `ones`. Python capped the head at 99 and returned `str(number)`
        // from 10^9 (bug 2); it now runs to 9_999_999, the maxval check's
        // bound — the next place name, ကောဋိ, is 10^14 (#147).
        let ten_millions_val = number / &ten_million;
        let remainder = number % &ten_million;
        let mut result = format!("{}{}", int_to_word(&ten_millions_val)?, KUTAY);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder)?);
        }
        return Ok(result);
    }
}

/// The exclusive ceiling (gladiaio/num2words2#147): 10^14 is ကောဋိ, the next
/// Burmese place after ကုဋေ, which this cascade does not compose.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(14))
}

/// Reconstruct Python's `str(f)` (== `repr(f)`) for a finite f64.
///
/// `precision` is the repr's fractional-digit count, computed Python-side as
/// `abs(Decimal(str(f)).as_tuple().exponent)`. Inside repr's plain-decimal
/// window (shortest-round-trip exponent in -4..=15) formatting the absolute
/// value to `precision` places reproduces the repr exactly; outside it repr
/// is scientific — CPython's `"1e+16"` / `"1e-05"` shape with a two-digit
/// zero-padded exponent — a form the string algorithm then rejects via
/// `int()`, exactly as Python's does. Same reconstruction as lang_ksw.rs,
/// validated there against the live interpreter.
fn float_repr(value: f64, precision: u32) -> String {
    let neg = value.is_sign_negative();
    let abs = value.abs();
    let body = if abs.is_finite() && abs != 0.0 {
        // "1.2345e3" / "1e-5" — LowerExp is shortest round-trip, like repr.
        let es = format!("{:e}", abs);
        let epos = es.find('e').expect("LowerExp of a finite nonzero has 'e'");
        let exp: i32 = es[epos + 1..]
            .parse()
            .expect("LowerExp exponent is an integer");
        if exp >= 16 || exp <= -5 {
            let mantissa = &es[..epos];
            let sign = if exp < 0 { "-" } else { "+" };
            format!("{}e{}{:02}", mantissa, sign, exp.abs())
        } else {
            format!("{:.*}", precision as usize, abs)
        }
    } else {
        // 0.0 -> "0.0" (precision is 1). Non-finite floats never reach the
        // float hooks: the dispatcher keeps inf/nan on the Python side, and
        // string "Infinity"/"NaN" are served by inf_result/nan_result below.
        format!("{:.*}", precision as usize, abs)
    };
    // The sign is the f64 sign *bit*, not `< 0.0` — the two differ only at
    // -0.0, where `str(-0.0) == "-0.0"` does carry the minus, so
    // `num2words(-0.0, lang="my")` prints the negword.
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// `int(s)`: ValueError with Python's exact message on a non-integer token —
/// how a scientific string form ("1e+16", "1E+3") raises in `to_cardinal`.
fn parse_pyint(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

pub struct LangMy {
    /// `CURRENCY_FORMS`, built once in [`LangMy::new`].
    ///
    /// Every pair carries **two identical forms** because that is exactly what
    /// `lang_MY.py` writes (`("ကျပ်", "ကျပ်")`) — Burmese has no plural
    /// inflection here. The arity is kept anyway: `to_currency` reads `[0]`
    /// and the inherited `to_cheque` reads `[-1]`, so a one-element tuple
    /// would still work but would stop matching Python's shape.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl LangMy {
    pub fn new() -> Self {
        // Built once per process: the binding holds `LangMy` in a `OnceLock`.
        let mut forms = HashMap::new();
        forms.insert(
            "MMK",
            CurrencyForms::new(&[MMK_UNIT, MMK_UNIT], &[MMK_SUB, MMK_SUB]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&[USD_UNIT, USD_UNIT], &[CENT, CENT]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&[EUR_UNIT, EUR_UNIT], &[CENT, CENT]),
        );
        LangMy { forms }
    }

    /// The string algorithm of `Num2Word_MY.to_cardinal`, run over a
    /// reconstructed `str(number)` — the float/Decimal entries land here.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword
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
    /// The sign test is **textual**, so "-0.0" carries the negword even
    /// though it is not numerically < 0. `int(left)` / `int(digit)` raise
    /// `ValueError` on the non-digit characters of a scientific form —
    /// `int("1e+16")` for a no-dot token, `int("e")` mid-loop for a
    /// dotted mantissa like "2.5e+16".
    fn cardinal_from_str(&self, s: &str) -> Result<String> {
        // `str(number).strip()` — a no-op for a reconstructed repr, kept so
        // the port matches the source.
        let s = s.trim();
        let (neg, n) = match s.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", s),
        };
        if let Some((left, right)) = n.split_once('.') {
            // `n.split(".", 1)`: everything after the FIRST dot is digits.
            let mut ret = format!(
                "{}{} {} ",
                neg,
                int_to_word(&parse_pyint(left)?)?,
                self.pointword()
            );
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                ret.push_str(&int_to_word(&BigInt::from(d))?);
                ret.push(' ');
            }
            // Python's trailing `.strip()` removes the loop's last space.
            return Ok(ret.trim().to_string());
        }
        // The `else` arm: whole Decimals ("5") convert; scientific no-dot
        // tokens ("1e+16", "1E+3") raise ValueError from int().
        Ok(format!("{}{}", neg, int_to_word(&parse_pyint(n)?)?)
            .trim()
            .to_string())
    }
}

impl Default for LangMy {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangMy {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MMK"
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

    /// `self.pointword` = "ဒသမ". Only consumed by the float branch of
    /// `to_cardinal`, which integral input never reaches; carried for parity.
    fn pointword(&self) -> &str {
        "ဒသမ"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python: `n = str(number).strip()`, then `if n.startswith("-")` peels
        // the sign and `int(n)` re-parses the rest. For a BigInt that is
        // exactly `is_negative()` + `abs()` — `str()` of an int is its decimal
        // repr, so the sign test and the reparse cannot disagree.
        let (ret, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // The `"." in n` branch is unreachable for integral input.
        let joined = format!("{}{}", ret, int_to_word(&magnitude)?);

        // Python's trailing `.strip()`. A no-op in practice — NEGWORD's space
        // is interior once a word follows it, and `_int_to_word` never returns
        // a space-edged string — but kept so the port matches the source.
        Ok(joined.trim().to_string())
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // `cardinal + "မြောက်"`, no separator. Note this happily welds the
        // suffix onto bug 2's raw digits: to_ordinal(10**9) == "1000000000မြောက်".
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // `str(number) + "မြောက်"` — digits, never words. Keeps the minus sign
        // for negatives: to_ordinal_num(-1) == "-1မြောက်".
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // `to_cardinal(val) + " ခုနှစ်"`. Python's `longval` parameter is
        // accepted and then ignored; there is no century/era handling, and
        // negatives just carry the negword: to_year(-500) == "အနုတ် ငါးရာ ခုနှစ်".
        Ok(format!("{}{}", self.to_cardinal(value)?, YEAR_SUFFIX))
    }

    /// Port of the float/Decimal path of `Num2Word_MY.to_cardinal`.
    ///
    /// **Route note — MY does *not* inherit `Num2Word_Base.to_cardinal_float`.**
    /// It *overrides `to_cardinal`* and handles non-integers inline via
    /// `str(number)`, so this deliberately does **not** delegate to
    /// `default_to_cardinal_float`: no `float2tuple`, no `< 0.01` artefact
    /// heuristic, no rounding. The words follow the literal digits of
    /// `repr(float)` / `str(Decimal)` — `2.675` yields `၆ ၇ ၅` (the repr's
    /// own digits, not the rescued `675` the base path computes), and
    /// trailing zeros survive (`Decimal("5.00")` -> "ငါး ဒသမ သုည သုည").
    /// So: reconstruct that string ([`float_repr`] for a float,
    /// `python_decimal_str` for a Decimal) and run the exact string
    /// algorithm, [`LangMy::cardinal_from_str`]. Consequences, corpus-pinned:
    ///
    /// * **Scientific string forms raise `ValueError`.** `str(1e16)` is
    ///   "1e+16" and `str(Decimal("1E+2"))` is "1E+2" — no ".", so Python's
    ///   `"." in n` is false and `int(n)` raises. Reproduced through
    ///   [`parse_pyint`] on the reconstructed token.
    ///
    /// * **The `precision=` kwarg is ignored.** `Num2Word_MY.to_cardinal`
    ///   never reads `self.precision` (it works off the string), so
    ///   `precision_override` is dropped, matching the live interpreter.
    ///
    /// * **Bug 2 applied to the integer part**: Python's
    ///   `_int_to_word(int(left))` fell back to bare digits at >= 10^9
    ///   (`Decimal("98746251323029.99")` -> "98746251323029 ဒသမ ကိုး ကိုး");
    ///   the port spells it with ကုဋေ, below the 10^14 ceiling (#147).
    ///
    /// * **A negative-zero Decimal keeps its negword**: `BigDecimal` cannot
    ///   carry `Decimal("-0.0")`'s sign, so the binding rewrites it to
    ///   `Float { value: -0.0 }`, whose sign *bit* survives into
    ///   [`float_repr`]'s "-0.0" and thence the textual sign test.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let s = match value {
            FloatValue::Float { value, precision } => float_repr(*value, *precision),
            FloatValue::Decimal { value, .. } => python_decimal_str(value),
        };
        self.cardinal_from_str(&s)
    }

    // ---- float/Decimal routing --------------------------------------------

    /// `to_cardinal(float/Decimal)` — full routing. MY's `to_cardinal`
    /// stringifies whatever it is given, so *every* value goes through the
    /// string algorithm: whole floats keep their ".0" tail
    /// (`5.0` -> "ငါး ဒသမ သုည"), whole Decimals take the integer arm, and
    /// scientific forms raise ValueError. No whole-value-to-int route exists.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `to_cardinal(number) + "မြောက်"` — the
    /// suffix is glued onto whatever the cardinal produced, decimals
    /// included ("ငါး ဒသမ သုညမြောက်"); a ValueError from the cardinal
    /// (scientific forms) propagates before the suffix is reached.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "မြောက်"` — the raw
    /// Python string form, so "5.00မြောက်", "1e+16မြောက်" and "-0.0မြောက်"
    /// all succeed even where the worded modes raise. `repr_str` is
    /// Python's `str(value)`, computed binding-side.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`: `to_cardinal(val) + " ခုနှစ်"`, same shape
    /// as the integer path — no era handling, negatives just carry the
    /// negword, and the cardinal's ValueError (scientific forms) propagates.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            YEAR_SUFFIX
        ))
    }

    /// `converter.str_to_number` is Base's plain `Decimal(value)` — MY does
    /// not override it, and the parse itself is the default. But the two
    /// non-finite specials produce outcomes the binding's generic Inf/NaN
    /// mapping cannot express, because MY's `to_cardinal` calls
    /// `int(str(number))`, never `int(number)`:
    ///
    /// * `Decimal("Infinity")` -> `int("Infinity")` -> **ValueError** in the
    ///   worded modes (the generic map raises OverflowError), while
    ///   `to_ordinal_num` succeeds with "Infinityမြောက်".
    /// * `Decimal("NaN")` -> `int("NaN")` -> ValueError likewise, with
    ///   "NaNမြောက်" from `to_ordinal_num`.
    ///
    /// A `BigDecimal` cannot hold Inf/NaN, so the parse is the plain default
    /// and both specials are served natively by the [`Lang::inf_result`] /
    /// [`Lang::nan_result`] hooks below — no Python fallback. (Corpus: cardinal
    /// "Infinity"/"-Infinity"/"NaN" are all ValueError.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal("Infinity")` / `-Infinity`. MY's `to_cardinal` runs
    /// `int(str(number))`, and `str(Decimal("Infinity"))` is "Infinity" —
    /// `int("Infinity")` raises **ValueError**, not the base path's
    /// OverflowError. `-Infinity` strips its "-" textually first, so both signs
    /// raise with the token "Infinity". Every worded mode reaches the same
    /// `int()`, so `to` is unread.
    fn inf_result(&self, _negative: bool, _to: &str) -> Result<String> {
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'Infinity'".into(),
        ))
    }

    /// `Decimal("NaN")`. `str(Decimal("NaN"))` is "NaN"; `int("NaN")` raises
    /// **ValueError** in every worded mode.
    fn nan_result(&self, _to: &str) -> Result<String> {
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'NaN'".into(),
        ))
    }

    // ---- currency --------------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_MY"
    }

    /// Only consumed by the **inherited** `to_cheque`; `to_currency` below
    /// deliberately bypasses this hook (see its `.get(...)` fallback note).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Python's `Num2Word_MY.to_currency` — a **total override** of
    /// `Num2Word_Base.to_currency`. It shares almost nothing with the base
    /// implementation, so this does not delegate to `default_to_currency`:
    ///
    /// * **Unknown codes do not raise.** `CURRENCY_FORMS.get(currency,
    ///   CURRENCY_FORMS["MMK"])` silently falls back to kyat/pya, so
    ///   `to_currency(1, "GBP")` is "တစ် ကျပ်", not a NotImplementedError.
    ///   (The inherited `to_cheque` *does* raise for the same code — hence
    ///   `cheque:GBP` erroring while `currency:GBP` succeeds. Both are in the
    ///   corpus.)
    /// * **`CURRENCY_PRECISION` is never consulted.** The cents field is
    ///   always two decimal digits, so JPY (a 0-decimal currency) still prints
    ///   subunits and KWD/BHD (3-decimal) still truncate to 2. No divisor,
    ///   no ROUND_HALF_UP quantize.
    /// * **`pluralize` is never called.** `cr1[0]`/`cr2[0]` take the first
    ///   form unconditionally, so the base's abstract `pluralize` (which
    ///   raises) is unreachable for this language.
    /// * **`adjective` is accepted and ignored**, so `CURRENCY_ADJECTIVES`
    ///   never applies.
    ///
    /// # Known divergence: floats Python reprs in scientific notation
    ///
    /// Because this routine parses `str(val)` rather than doing arithmetic, it
    /// is sensitive to *how* the value was stringified — and the shim boundary
    /// only preserves the value, not the spelling. Python reprs a float in
    /// scientific notation when `abs(x) < 1e-4` or `>= 1e16`, and `int()` then
    /// chokes on the mantissa, so **Python raises ValueError** there. Fed the
    /// same string, `BigDecimal` re-renders it:
    ///
    /// | `str(float)` | BigDecimal Display | Python | here |
    /// |---|---|---|---|
    /// | `1e+16`, `1e+21` | unchanged | ValueError | ValueError ✓ |
    /// | `1e-07` and smaller | `1E-7` | ValueError | ValueError ✓ |
    /// | `1e-05`, `1e-06` | **expanded** to `0.00001` | ValueError | 0 cents ✗ |
    ///
    /// So exponents -5 and -6 — floats in `[1e-6, 1e-4)` — return
    /// "သုည ကျပ်" where Python raises. No corpus row covers this, and it is
    /// not fixable at this layer: recovering the distinction would mean
    /// re-implementing `repr(float)`, which `currency.rs`'s module docs
    /// explicitly reject as a permanent source of drift. Everything else in
    /// the differential (23/24 extra cases + all 117 corpus rows) matches.
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
        // `if val < 0: is_negative = True; val = abs(val)`.
        let is_negative = val.is_negative();

        // `str(val)`, taken on the *absolute* value because Python rebinds
        // `val = abs(val)` first.
        //
        // The whole routine is string-driven rather than arithmetic, and that
        // is load-bearing — see the `[:2]` truncation below. `Int` stringifies
        // with no ".", which is precisely why a true int never reaches the
        // cents branch, while the float `1.0` -> "1.0" does reach it (and then
        // fails the `right` truthiness test). Never collapse the two.
        let text = match val {
            CurrencyValue::Int(i) => i.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `parts = str(val).split(".")` — split on *every* dot, as Python
        // does, then read only [0] and [1].
        let parts: Vec<&str> = text.split('.').collect();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if parts[0].is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(parts[0]).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` **truncates, it never rounds**: 1.999 -> "99" -> 99 cents, and
        // 1.005 -> "00" -> 0 cents (no cents segment at all). `ljust` then
        // right-pads, which is what turns 0.5 -> "5" into 50 cents rather
        // than 5. Sliced by `chars()`, matching Python's str indexing.
        let right = match parts.get(1) {
            Some(frac) if !frac.is_empty() => {
                let mut s: String = frac.chars().take(2).collect();
                while s.chars().count() < 2 {
                    s.push('0');
                }
                BigInt::from_str(&s).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["MMK"])`
        let forms = self
            .forms
            .get(currency)
            .unwrap_or_else(|| self.forms.get("MMK").expect("MMK inserted in new()"));

        // `_int_to_word`, not `to_cardinal`: `left` is already non-negative, so
        // the sign is applied once at the end rather than here.
        let mut result = format!("{} {}", int_to_word(&left)?, forms.unit[0]);

        // `if cents and right:` — `right == 0` is falsy in Python, so a float
        // with zero cents (1.0, 100.0) prints no cents segment. This is the
        // one place the base class would have differed: it always renders
        // cents for a float.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right)?);
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        // `(self.negword if is_negative else "") + result` — the raw negword,
        // trailing space included, and **no** final `.strip()`.
        Ok(format!(
            "{}{}",
            if is_negative { NEGWORD } else { "" },
            result
        ))
    }
}
