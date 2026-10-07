//! Port of `lang_GL.py` (Galician).
//!
//! Shape: **self-contained**. `Num2Word_GL` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` probe in `Num2Word_Base.__init__` never fires: Python builds
//! **no `self.cards`** and **never sets `self.MAXVAL`**. `to_cardinal` is
//! overridden outright and drives `_int_to_word`, a plain recursive
//! divide-by-scale routine. Consequently `cards`/`merge` stay at their trait
//! defaults here. Python has **no overflow check at all** — `MAXVAL` does
//! not exist on the instance; this port reports and enforces a `maxval` of
//! 10^24 (bug 5, gladiaio/num2words2#147).
//!
//! Inherited from `Num2Word_Base` and left alone by GL — but GL happens to
//! re-declare two of them identically, so the observable behaviour is the
//! same either way:
//!   * `to_ordinal_num(value) -> str(value) + "."`  (GL overrides; the base
//!     default would have returned the value untouched, so this one *is* a
//!     real override.)
//!   * `to_year(val, longval=True) -> self.to_cardinal(val)` (GL overrides
//!     with a body identical to the base default; either way it delegates
//!     through `&self` and picks up the `to_cardinal` override below.)
//!
//! No cross-call mutable state: GL defines no `str_to_number` and stashes no
//! pending flags, so the stateless Rust path is a faithful stand-in for the
//! Python dispatcher's fast path.
//!
//! # Currency
//!
//! `Num2Word_GL` declares its own `CURRENCY_FORMS` **dict literal** — it does
//! not subclass `Num2Word_EUR`, so it never sees the dict that
//! `Num2Word_EN.__init__` mutates in place, and none of EN's ~24 extra codes
//! (JPY/KWD/CHF/INR/...) leak in. Verified against the live interpreter: the
//! table is exactly `{"EUR", "USD"}` and `id(Num2Word_GL.CURRENCY_FORMS) !=
//! id(Num2Word_Base.CURRENCY_FORMS)`.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited from
//! `Num2Word_Base` and both are `{}`, so `currency_adjective` and
//! `currency_precision` stay at their trait defaults (`None` / `100`).
//!
//! GL **overrides `to_currency` outright** — `Num2Word_Base.to_currency` is
//! never reached, so `pluralize`, `_cents_verbose`, `_cents_terse`,
//! `parse_currency_parts` and the whole `CURRENCY_PRECISION` machinery are
//! dead code for this language. `pluralize` therefore stays at the trait
//! default that raises, exactly mirroring `Num2Word_Base.pluralize` — which
//! also raises `NotImplementedError` and is likewise never called.
//!
//! `to_cheque` is **not** overridden, so the base implementation runs: it
//! reaches `_money_verbose` (-> `to_cardinal`) and indexes
//! `self.CURRENCY_FORMS[currency]` *directly*, which is why cheques raise
//! where `to_currency` silently falls back. See bugs 12 and 13 below.
//!
//! # Faithfully reproduced Python bugs / oddities — currency
//!
//! 10. **An unknown currency code never raises.** `to_currency` looks the code
//!     up as `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS
//!     .values())[0])`, so `GBP`, `JPY`, `KWD`, `BHD`, `INR`, `CNY`, `CHF` —
//!     everything outside `{EUR, USD}` — silently render as **euros**, with
//!     `céntimo(s)` subunits. Corpus-confirmed: `currency:JPY` of `1` is
//!     `"un euro"`, not a `NotImplementedError`. See [`CURRENCY_FALLBACK_KEY`].
//! 11. **`CURRENCY_PRECISION` is ignored by `to_currency`.** The subunit
//!     divisor is hard-wired to 100 by the `[:2].ljust(2, "0")` string slice,
//!     so the 3-decimal currencies get cents rather than mils and the
//!     0-decimal ones are not rounded to whole units: `currency:KWD` of
//!     `1234.56` is `"... cincuenta seis céntimos"` (56/100), and
//!     `currency:JPY` of `12.34` still prints a cents segment. Moot in
//!     practice — bug 10 means neither code reaches its own forms anyway.
//! 12. **`to_cheque` and `to_currency` disagree about unknown codes.** GL
//!     inherits `Num2Word_Base.to_cheque`, whose `self.CURRENCY_FORMS[
//!     currency]` is a bare subscript, so it raises `NotImplementedError` for
//!     the very codes `to_currency` happily aliases to euros. Corpus-confirmed:
//!     `cheque:GBP` -> `NotImplementedError`, `currency:GBP` -> `"un euro"`.
//! 13. **Cents are truncated, never rounded.** `int(parts[1][:2].ljust(2,
//!     "0"))` slices the *decimal string*: `12.345` -> 34 céntimos (not 35),
//!     and `0.001` -> `"00"` -> 0 -> the cents segment vanishes entirely.
//! 14. **`cents=False` drops the cents segment instead of digitising it.**
//!     `Num2Word_Base` routes `cents=False` to `_cents_terse` ("56"); GL's
//!     guard is `if cents and right:`, so `cents=False` emits no cents at all.
//! 15. **`adjective=` is accepted and silently ignored.** GL takes the kwarg
//!     to keep the base signature but never reads it, and its inherited
//!     `CURRENCY_ADJECTIVES` is empty regardless.
//! 16. **Zero cents suppress the segment even for floats.** `1.0` -> parts[1]
//!     is `"0"` -> `right == 0` -> falsy -> `"un euro"`. GL reaches the same
//!     place as the base's `isinstance(val, int)` shortcut, but by a different
//!     road: it never inspects the type, it just tests whether `right` is
//!     truthy. So for GL — uniquely — `1` and `1.0` genuinely agree, and the
//!     `Int`/`Decimal` split of [`CurrencyValue`] is behaviourally inert here.
//!     It is still honoured on both arms rather than collapsed.
//! 17. **Scientific notation is a `ValueError`, not a number.** `to_currency`
//!     parses `str(val)`, so `1e16` -> `parts[0] == "1e+16"` ->
//!     `int("1e+16")` -> `ValueError`. See [`LangGl::split_currency`].
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Every one of these looks wrong and is
//! exactly what Python emits — all confirmed against the frozen corpus:
//!
//! 1. **No teens, no compound tens.** `_int_to_word` has no 11..19 table and
//!    joins tens to ones with a bare space, so 11 == "dez un" (literally
//!    "ten one", not "once") and 21 == "vinte un" (not "vinte e un").
//!    Galician normally uses "e" between tens and units; GL never emits it.
//! 2. **"un cento", not "cen"/"cento".** The hundreds branch is
//!    unconditionally `ones[h] + " cento"`, so 100 == "un cento" (real
//!    Galician is "cen") and 200 == "dous cento" (real: "douscentos").
//! 3. **"un mil", not "mil".** The thousands branch always recurses on the
//!    multiplier, so 1000 == "un mil" (real Galician: "mil").
//! 4. **`millón` is never pluralised** and the scale word is never joined
//!    with "de": 10^7 == "dez millón" (real: "dez millóns").
//! 5. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's final
//!    `else` of `_int_to_word` is `return str(number)`, so
//!    `to_cardinal(10**9)` was the digit string "1000000000". Galician has
//!    no word for 10^9 — it says "mil millóns" (RAG lists no "millardo") —
//!    and uses the long scale above it: billón 10^12 ("un millón de
//!    millóns") and trillón 10^18 (academia.gal/dicionario, "billón",
//!    "trillón"). So the `millón` branch now takes multipliers up to
//!    999999 — 10^9 == "un mil millón", in bug 3/4's style — and `billón`
//!    and `trillón` branches compose the same way; `OverflowError` from
//!    10^24 (`maxval`).
//! 6. **`to_ordinal` is just the cardinal plus a literal `"-o"` suffix**,
//!    hyphen included: 0 == "cero-o", 100 == "un cento-o". Not a real
//!    Galician ordinal ("primeiro", "centésimo").
//! 7. **`to_ordinal` accepts negatives** — it never calls
//!    `verify_ordinal()`, so `to_ordinal(-1)` == "menos un-o" rather than
//!    raising `TypeError` the way most modules would.
//! 8. **`negword` keeps its trailing space.** Python's is the English
//!    `"minus "`; this port uses the Galician "menos " (gladiaio/num2words2#154).
//!    GL's `to_cardinal` interpolates it *raw* rather than via the base's
//!    `"%s " % self.negword.strip()` idiom; the trailing space makes the two
//!    coincide, and the final `.strip()` cleans up the rest. See [`NEGWORD`].
//! 9. **Zero and the decimal word (fixed, #154).** `ones[0]` is `""`, so
//!    Python's `_int_to_word(0)` takes the
//!    `self.ones[0] if self.ones[0] else "zero"` branch and yields the English
//!    "zero", and its `pointword` is the English "point". This port uses the
//!    RAG-standard "cero" and "coma" (Galician writes a decimal comma):
//!    `0` == "cero", `1.5` == "un coma cinco".
//!
//! # Errors
//!
//! None of the four in-scope modes can raise for integer input: every list
//! index is derived from a `% 10` or `/ 100` of a bounded positive value, so
//! `ones`/`tens` are never indexed out of range, and the 10^9 fallback
//! swallows what would otherwise overflow. `Result` is returned only to
//! satisfy the trait.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD uses dólar/dólares with centavo/centavos. Examples in these
//! docs that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.negword`. Note the trailing space, verbatim from `lang_GL.py`'s
/// `setup()`; the word is Galician here, English there (#154). GL's
/// `to_cardinal` uses this raw
/// (`ret = self.negword`), unlike `Num2Word_Base.to_cardinal`, which would
/// have written `"%s " % self.negword.strip()`. The two agree here only
/// because the literal already carries exactly one trailing space.
const NEGWORD: &str = "menos ";

/// `self.pointword`. English "point" in Python; Galician "coma" here (#154).
const POINTWORD: &str = "coma";

/// What `_int_to_word(0)` returns. English "zero" in Python; "cero" here.
const ZERO_WORD: &str = "cero";

/// The `millón` scale and the long-scale words above it (#147), largest
/// first, as `(power of ten, word)`. Each takes a multiplier up to 999999:
/// 10^9 is "mil millóns", 10^15 "mil billóns" (academia.gal/dicionario,
/// "billón", "trillón").
const SCALES: [(u32, &str); 3] = [(18, "trillón"), (12, "billón"), (6, MILLION)];

/// The exclusive ceiling: 10^24 would need a cuatrillón, which is not
/// in the sources the scale table is drawn from (#147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(24))
}

/// `self.ones`. Index 0 is the empty string — that emptiness is load-bearing:
/// it is what makes `_int_to_word(0)` fall through to [`ZERO_WORD`].
const ONES: [&str; 10] = [
    "", "un", "dous", "tres", "catro", "cinco", "seis", "sete", "oito", "nove",
];

/// `self.tens`. Index 0 is unused (the `< 10` branch catches those first).
/// There is deliberately no teens table — see module docs, bug 1.
const TENS: [&str; 10] = [
    "", "dez", "vinte", "trinta", "corenta", "cincuenta", "sesenta", "setenta", "oitenta",
    "noventa",
];

const HUNDRED: &str = "cento";
const THOUSAND: &str = "mil";
const MILLION: &str = "millón";

/// `self.__class__.__name__`, for `Num2Word_Base.to_cheque`'s
/// `'Currency code "%s" not implemented for "%s"'`.
const LANG_NAME: &str = "Num2Word_GL";

/// The key of `list(self.CURRENCY_FORMS.values())[0]` — the entry
/// `to_currency` falls back to for an unknown code (module docs, bug 10).
///
/// Python re-evaluates that `list(...)[0]` on *every* call and takes the first
/// value in insertion order. `Num2Word_GL.CURRENCY_FORMS` is a dict literal
/// with `"EUR"` written first and is never mutated, and dicts have preserved
/// insertion order since 3.7, so the fallback is deterministically the EUR
/// entry. A `HashMap` has no order, hence the explicit key.
const CURRENCY_FALLBACK_KEY: &str = "EUR";

/// `Num2Word_GL.to_currency`'s own default `separator=" "` — a bare space,
/// where `Num2Word_Base` declares `separator=","`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot be a plain parameter default.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_GL.to_currency` declares `separator=" "`, but the `Lang` trait has
/// no per-language parameter defaults: `__init__.py`'s fast path and
/// `bench/diff_test.py` both send `kwargs.get("separator", ",")` — i.e.
/// `Num2Word_Base`'s default — so by the time the value reaches this side, the
/// information needed to tell "unset" from "explicitly a comma" is gone.
///
/// So `,` is read back as the unset sentinel and GL's own default restored.
/// This is the only reading that matches the oracle: all 54 float-with-cents
/// rows of the `gl` currency corpus were generated by `num2words(v, lang="gl",
/// to="currency", currency=c)` with no `separator=`, and every one of them
/// expects a plain space ("... euros trinta catro céntimos"). Same convention
/// and same sentinel as `lang_as.rs` and `lang_ca.rs`.
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets `" "` here where Python would give `","`. Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// Holds the `CURRENCY_FORMS` table, built once in [`LangGl::new`].
///
/// The four integer modes are stateless — `setup()` only assigns constant
/// tables, which live as `const`s above. The currency table is a `HashMap`
/// rather than a `const` because `to_currency` needs keyed lookup; it is
/// built once per process (`num2words2-py` holds a `OnceLock<LangGl>`) and
/// never per call.
pub struct LangGl {
    /// `Num2Word_GL.CURRENCY_FORMS`. GL's own dict literal — see module docs.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]`, cloned out at construction so
    /// `to_currency`'s fallback is a field read rather than Python's per-call
    /// list materialisation. See [`CURRENCY_FALLBACK_KEY`].
    currency_fallback: CurrencyForms,
}

impl Default for LangGl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangGl {
    pub fn new() -> Self {
        // `Num2Word_GL.CURRENCY_FORMS`, verbatim. Two forms per side, matching
        // Python's arity — `to_currency` indexes `cr1[1]`/`cr2[1]` directly.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (
                "EUR",
                CurrencyForms::new(&["euro", "euros"], &["céntimo", "céntimos"]),
            ),
            (
                // Not a typo to fix: Python really spells the USD unit
                // "dollar"/"dollars" (English) in a Galician module. Real
                // Galician is "dólar"/"dólares". Corpus-confirmed.
                "USD",
                CurrencyForms::new(&["dólar", "dólares"], &["centavo", "centavos"]),
            ),
        ]
        .into_iter()
        .collect();

        let currency_fallback = currency_forms
            .get(CURRENCY_FALLBACK_KEY)
            .expect("CURRENCY_FALLBACK_KEY is inserted directly above")
            .clone();

        LangGl {
            currency_forms,
            currency_fallback,
        }
    }

    /// Python's `parts = str(val).split(".")` and the `left`/`right`
    /// extraction that follows it, for an already-absolute value.
    ///
    /// ```text
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// This is a **string** operation, not an arithmetic one, and that is
    /// load-bearing twice over:
    ///
    /// * `[:2]` truncates rather than rounds (module docs, bug 13), and
    /// * `.ljust(2, "0")` pads on the *right*, so a single fractional digit
    ///   scales up: `0.5` -> `"5"` -> `"50"` -> 50 céntimos, not 5.
    ///
    /// The `BigDecimal` was parsed from the `str(number)` the Python shim
    /// sent, and `bigdecimal` preserves the written scale exactly (probed:
    /// `"1.0"` -> `(10, 1)`, `"0.5"` -> `(5, 1)`, `"0.001"` -> `(1, 3)`), so
    /// `scale` *is* `len(parts[1])` and the digit string can be rebuilt
    /// losslessly. `abs()` preserves the scale too, and `str(abs(x))` differs
    /// from `str(x)` only by the sign, so re-deriving the split from the
    /// absolute value is exact.
    ///
    /// # Errors
    ///
    /// `scale < 0` means the incoming string carried an exponent with a
    /// positive net power (`"1e+16"` -> `(1, -16)`; a plain `"10000000000000000"`
    /// gives scale 0, so there is no false positive). Python's `str(val)` then
    /// renders the same notation, and `int("1e+16")` raises **`ValueError`** —
    /// reproduced here as [`N2WError::Value`]. The message is rebuilt from
    /// `BigDecimal`'s own `Display`, which agrees with Python for float input
    /// (`1e16` -> `"1e+16"`) but not for a `Decimal("1E+2")`, where Python
    /// keeps `"1E+2"` and `Display` normalises to `"100"`. The exception
    /// *type* is right either way; the corpus asserts only the type.
    ///
    /// # Known gap: small floats
    ///
    /// The mirror-image case is **not** detectable and is knowingly wrong.
    /// Python's `repr` switches to exponent form below `1e-4` as well, so
    /// `to_currency(1e-05)` also raises `ValueError` — but `"1e-05"` and
    /// `"0.00001"` both parse to the *same* `BigDecimal` `(1, 5)`, and only the
    /// first raises in Python (`str(Decimal("0.00001"))` keeps the plain form
    /// and yields "cero euros"). The distinguishing information is the wire
    /// string, which `CurrencyValue::Decimal` has already discarded by the time
    /// this method runs. Telling them apart needs the raw `&str` carried
    /// through `to_currency`, which lives in `currency.rs`/`base.rs` — outside
    /// this port's remit.
    ///
    /// Blast radius: floats with `0 < |x| < 1e-4`, which Rust renders as
    /// "cero euros" where Python raises `ValueError`. No corpus row covers it.
    /// Flagged in the port report.
    fn split_currency(&self, val: &BigDecimal) -> Result<(BigInt, BigInt)> {
        let (int_val, scale) = val.as_bigint_and_exponent();

        if scale < 0 {
            // int("1e+16") -> ValueError. See the doc comment above.
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                val
            )));
        }

        // scale == 0: `str(val)` has no ".", so `parts` has one element and
        // `right` stays 0.
        if scale == 0 {
            return Ok((int_val, BigInt::zero()));
        }

        // `val` is absolute, so these digits carry no sign.
        let scale = scale as usize;
        let digits = int_val.to_string();
        let padded = if digits.len() <= scale {
            // Python always writes at least one integer digit ("0.5", never
            // ".5"), so widen to scale+1 to keep parts[0] non-empty.
            format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits)
        } else {
            digits
        };
        let (int_part, frac_part) = padded.split_at(padded.len() - scale);

        // Both halves are pure ASCII digits, so `chars()` and byte offsets
        // coincide and neither parse can fail.
        let left = BigInt::from_str(int_part).expect("int_part is ASCII digits");

        // `parts[1][:2].ljust(2, "0")`
        let mut frac: String = frac_part.chars().take(2).collect();
        while frac.len() < 2 {
            frac.push('0');
        }
        let right = BigInt::from_str(&frac).expect("frac is two ASCII digits");

        Ok((left, right))
    }

    /// `_int_to_word`.
    ///
    /// Mirrors the Python cascade, except that its `str(number)` fallback
    /// for `number >= 10**9` is replaced by the long-scale branches and the
    /// `maxval` check (module docs, bug 5, #147).
    ///
    /// All divisions here run on non-negative values (the `< 0` arm recurses
    /// on `abs` first), so Rust's truncating `/` and `%` agree with Python's
    /// floor `//` and `%` — no `div_mod_floor` needed.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        // Python: `return self.ones[0] if self.ones[0] else "zero"`.
        // ONES[0] == "" is falsy, so this always yields the zero word.
        if number.is_zero() {
            return Ok(if ONES[0].is_empty() {
                ZERO_WORD.to_string()
            } else {
                ONES[0].to_string()
            });
        }

        // Unreachable from the four in-scope modes: `to_cardinal` strips the
        // sign before calling in, so `_int_to_word` only ever sees
        // non-negative values. Ported anyway because Python has it, and
        // `to_currency` (out of scope) does reach `_int_to_word` directly.
        if number.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&number.abs())?));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if number < &ten {
            // 1..=9: always in range.
            return Ok(ONES[idx(number)].to_string());
        }

        if number < &hundred {
            let tens_val = number / &ten; // 1..=9
            let ones_val = number % &ten; // 0..=9
            if ones_val.is_zero() {
                return Ok(TENS[idx(&tens_val)].to_string());
            }
            // Bare space join — no "e". See module docs, bug 1.
            return Ok(format!("{} {}", TENS[idx(&tens_val)], ONES[idx(&ones_val)]));
        }

        if number < &thousand {
            let hundreds_val = number / &hundred; // 1..=9
            let remainder = number % &hundred;
            // Unconditional "<ones> cento" — see module docs, bug 2.
            let mut result = format!("{} {}", ONES[idx(&hundreds_val)], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        if number < &million {
            let thousands_val = number / &thousand; // 1..=999
            let remainder = number % &thousand;
            // Always recurses on the multiplier — see module docs, bug 3.
            let mut result = format!("{} {}", self.int_to_word(&thousands_val)?, THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        // Python stopped here: `if number < 10**9` was the last branch and
        // `return str(number)` followed (bug 5). Galician says 10^9 as "mil
        // millóns", so the `millón` branch takes multipliers up to 999999,
        // and `billón` (10^12) and `trillón` (10^18) compose the same way;
        // the maxval check keeps the trillón multiplier below 10^6 (#147).
        for &(exp, word) in SCALES.iter() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let scale_val = number / &scale;
                let remainder = number % &scale;
                // Never pluralised — see module docs, bug 4.
                let mut result = format!("{} {}", self.int_to_word(&scale_val)?, word);
                if !remainder.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&remainder)?);
                }
                return Ok(result);
            }
        }
        unreachable!("every value >= 10^6 and below maxval has a SCALES entry")
    }

    /// GL's `to_cardinal` string body, run on a reconstructed `str(number)`.
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"): n = n[1:]; ret = self.negword   else: ret = ""
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
    /// The reconstructed `s` never carries surrounding whitespace, but the
    /// `.strip()`/`.trim()` bookends are reproduced anyway. `int(left)` and
    /// `int(digit)` on any non-digit token raise `ValueError` — unreachable
    /// once the fixed-notation callers have filtered exponent forms out, but
    /// guarded regardless so the failure mode stays GL's.
    fn cardinal_from_py_str(&self, s: &str) -> Result<String> {
        let n = s.trim();
        let (neg, n) = match n.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, n),
        };
        let mut ret = String::new();
        if neg {
            // `ret = self.negword` — raw "menos " (trailing space is the join).
            ret.push_str(NEGWORD);
        }
        if let Some((left, right)) = n.split_once('.') {
            // int(left): the integer part (>= 10^24 raises OverflowError, #147).
            let left_int = BigInt::from_str(left).map_err(|_| int_value_err(left))?;
            ret.push_str(&self.int_to_word(&left_int)?);
            ret.push(' ');
            ret.push_str(self.pointword());
            ret.push(' ');
            // Digitise each fractional character via int(digit).
            for ch in right.chars() {
                let d = ch
                    .to_digit(10)
                    .ok_or_else(|| int_value_err(&ch.to_string()))?;
                ret.push_str(&self.int_to_word(&BigInt::from(d))?);
                ret.push(' ');
            }
            Ok(ret.trim().to_string())
        } else {
            let v = BigInt::from_str(n).map_err(|_| int_value_err(n))?;
            ret.push_str(&self.int_to_word(&v)?);
            Ok(ret.trim().to_string())
        }
    }
}

/// Narrow a bounded `BigInt` (0..=9 at every call site) to a list index.
///
/// Every caller has already proven the value is a single digit via a
/// `% 10` or a `< 10` / `/ 100` bound, so `to_usize` cannot fail; the
/// `unwrap_or(0)` is belt-and-braces and unreachable.
fn idx(n: &BigInt) -> usize {
    n.to_usize().unwrap_or(0)
}

/// `int(token)`'s `ValueError` message. GL never catches it, so only the
/// exception *type* is observable; the text mirrors CPython for readability.
fn int_value_err(token: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        token
    ))
}

/// Rust's `{:e}` yields the shortest round-trip mantissa+exponent — the same
/// significant-digit sequence CPython's `repr` uses — so `(digits, e)` here
/// satisfies `fabs == D.DDD * 10^e` with `digits` carrying no leading or
/// trailing zeros (except the lone `"0"` for `fabs == 0.0`).
///
/// This is the load-bearing choice for GL: the language digitises the *repr
/// string* rather than `base.float2tuple`'s `abs(value-pre)*10^precision`, and
/// the two disagree in the last fractional digit once a float carries ~16
/// significant digits (the multiply-out drifts; the repr does not). Reading
/// the digits straight off `{:e}` reproduces `str(float)` exactly.
fn float_e_parts(fabs: f64) -> (String, i64) {
    if fabs == 0.0 {
        return ("0".to_string(), 0);
    }
    let s = format!("{:e}", fabs);
    // `{:e}` is always `<mant>e<exp>` with `<mant>` a normalised `d[.ddd]`.
    let (mant, exp) = s.split_once('e').expect("{:e} always contains 'e'");
    let e0: i64 = exp.parse().expect("{:e} exponent is an integer");
    let (ip, fp) = match mant.split_once('.') {
        Some((i, f)) => (i, f),
        None => (mant, ""),
    };
    let mut digits = String::with_capacity(ip.len() + fp.len());
    digits.push_str(ip);
    digits.push_str(fp);
    let trimmed = digits.trim_end_matches('0');
    let digits = if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    };
    // `ip` is a single significant digit, so the leading-digit exponent is
    // `e0 + len(ip) - 1`.
    (digits, e0 + ip.len() as i64 - 1)
}

/// Reconstruct CPython's `str(f)` for a finite float in the fixed-notation
/// range, or `Err(Value)` where GL would raise `ValueError`.
///
/// `repr(float)` switches to exponent form when the decimal point position
/// `decpt` (with `value == 0.d1d2… * 10^decpt`) satisfies `decpt <= -4` or
/// `decpt > 16`. GL then feeds a token containing `'e'` (or `"nan"`/`"inf"`)
/// to `int()`, which raises `ValueError`. Everything else is fixed-point,
/// rebuilt exactly as CPython's `format_float_short` does (always keeping a
/// decimal point, e.g. `1.0`, since GL relies on the `"."` to take its
/// fractional branch).
fn float_to_py_str(f: f64) -> Result<String> {
    if !f.is_finite() {
        // str(nan)/str(inf)/str(-inf): no '.', so int(token) -> ValueError.
        return Err(int_value_err(&format!("{}", f)));
    }
    // str(-0.0) == "-0.0": the sign is the sign *bit*, not `f < 0`.
    let neg = f.is_sign_negative();
    let fabs = f.abs();
    let (digits, e) = float_e_parts(fabs);
    let decpt = e + 1;
    if fabs != 0.0 && (decpt <= -4 || decpt > 16) {
        // repr uses exponent notation here -> GL's int() -> ValueError.
        return Err(int_value_err(&format!("{:e}", fabs)));
    }
    let dlen = digits.len() as i64;
    let body = if decpt <= 0 {
        // 0.00…digits
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= dlen {
        // digits with trailing zeros, then the mandatory ".0"
        format!("{}{}.0", digits, "0".repeat((decpt - dlen) as usize))
    } else {
        // split the digits at the point (digits are ASCII, so byte = char)
        let k = decpt as usize;
        format!("{}.{}", &digits[..k], &digits[k..])
    };
    Ok(if neg { format!("-{}", body) } else { body })
}

/// Reconstruct CPython's `str(Decimal)` from a `BigDecimal`, or `Err(Value)`
/// where GL would raise `ValueError`.
///
/// `Decimal.__str__` uses exponent form when `exp > 0` or the adjusted
/// exponent `exp + len(coeff) - 1 < -6`; GL then hands a `'E'`-bearing token
/// to `int()` and raises `ValueError`. The fixed-point branch reproduces
/// Decimal's own algorithm, and — because `bigdecimal` preserves the written
/// scale (`"1.10" -> (110, 2)`) — the coefficient and exponent match CPython's
/// `as_tuple()` exactly, trailing zeros included.
fn decimal_to_py_str(d: &BigDecimal) -> Result<String> {
    // value == mantissa * 10^-scale, i.e. Decimal exponent `exp == -scale`.
    let (mantissa, scale) = d.as_bigint_and_exponent();
    let neg = mantissa.is_negative();
    let coeff = mantissa.abs().to_string(); // significant digits; "0" for zero
    let exp: i64 = -scale;
    let adjexp = exp + coeff.len() as i64 - 1;
    if exp > 0 || adjexp < -6 {
        // Decimal renders exponent notation here -> GL's int() -> ValueError.
        return Err(int_value_err(&format!("{}", d)));
    }
    let sign = if neg { "-" } else { "" };
    if exp == 0 {
        // Integer-valued Decimal: str() has no '.', GL takes its whole branch.
        return Ok(format!("{}{}", sign, coeff));
    }
    // exp < 0, i.e. scale > 0: fixed-point with `scale` fractional digits.
    let scale_u = scale as usize;
    let padded = if coeff.len() <= scale_u {
        // Keep at least one integer digit ("0.001", never ".001").
        format!("{}{}", "0".repeat(scale_u + 1 - coeff.len()), coeff)
    } else {
        coeff
    };
    let (ip, fp) = padded.split_at(padded.len() - scale_u);
    Ok(format!("{}{}.{}", sign, ip, fp))
}

impl Lang for LangGl {
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
    /// `to_cardinal(number) + "-o"` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "cinco coma cero-o".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}-o", cardinal))
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
        "EUR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        // Verbatim, trailing space included — see NEGWORD.
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }


    /// `to_cardinal`.
    ///
    /// Python works on `str(number)`: it detaches a leading "-", sets
    /// `ret = self.negword`, and (for integers — no "." in the digits)
    /// returns `(ret + self._int_to_word(int(n))).strip()`.
    ///
    /// Splitting on `is_negative()`/`abs()` is equivalent for integer input:
    /// `str(BigInt)` yields exactly one optional leading "-" followed by
    /// digits, and Python ints have no "-0".
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        // Python's trailing `.strip()`. `_int_to_word` never returns padding,
        // so in practice this only trims NEGWORD's space off a "-0"-ish edge
        // that BigInt cannot produce — but reproduce it regardless.
        Ok(format!("{}{}", ret, self.int_to_word(&n)?).trim().to_string())
    }

    /// `to_ordinal`: cardinal + a literal "-o". No `verify_ordinal()` call,
    /// so negatives sail through. Module docs, bugs 6 and 7.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}-o", cardinal))
    }

    /// `to_ordinal_num`: `str(number) + "."`. Digits, never words, and the
    /// minus sign survives: -1 -> "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `to_year(val, longval=True)`: ignores `longval` entirely and just
    /// delegates to `to_cardinal`. No BC/AD suffix, no era handling — so
    /// -500 -> "menos cinco cento".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / Decimal cardinal path.
    ///
    /// **Routing note.** In Python GL overrides `to_cardinal`, *not*
    /// `to_cardinal_float`: `num2words(0.5, lang="gl")` dispatches to
    /// `to_cardinal`, which handles non-integers inline off `str(number)`
    /// (`if "." in n: ...`). The inherited `Num2Word_Base.to_cardinal_float`
    /// is dead code for GL. The Rust core routes float input here instead, so
    /// this override reproduces GL's *inline* handling rather than the base
    /// path — which is why GL cannot inherit `default_to_cardinal_float`:
    ///
    ///   * GL digitises the raw `repr` string, so a float with ~16 significant
    ///     digits keeps its exact `str()` digits, whereas `float2tuple`'s
    ///     `abs(value-pre)*10^precision` drifts in the last one
    ///     (`0.44308006468156513` -> "…un tres" vs the base's "…un dous").
    ///   * A value whose `repr`/`str` uses exponent notation
    ///     (`|x| >= 1e16`, `0 < |x| < 1e-4`, or an equivalently small Decimal)
    ///     is fed to `int()` and raises `ValueError` — where the base path
    ///     would happily digitise it.
    ///
    /// **`precision_override` is ignored**, matching Python: `precision=` is
    /// popped in `__init__.py` and, although GL *has* a `self.precision`
    /// attribute, its `to_cardinal` never reads it (it digitises the full
    /// `str(number)`), so the kwarg is inert for GL.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Rebuild `str(number)` exactly, then run GL's string body on it.
        let s = match value {
            FloatValue::Float { value, .. } => float_to_py_str(*value)?,
            FloatValue::Decimal { value, .. } => decimal_to_py_str(value)?,
        };
        self.cardinal_from_py_str(&s)
    }

    // ---- currency ------------------------------------------------------

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `self.CURRENCY_FORMS[code]` — the **strict** lookup, matching the bare
    /// subscript in `Num2Word_Base.to_cheque`. `None` here becomes that
    /// method's `NotImplementedError`.
    ///
    /// Deliberately *not* where bug 10's euro fallback lives: `to_cheque` must
    /// keep raising for the codes `to_currency` aliases, so the fallback stays
    /// local to [`LangGl::to_currency`], exactly as `.get(k, default)` vs
    /// `[k]` splits them in Python.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_GL.to_currency` — a full override; `Num2Word_Base
    /// .to_currency` is never reached and neither is `pluralize`.
    ///
    /// ```text
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    /// left_str = self._int_to_word(left)
    /// result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     cents_str = self._int_to_word(right)
    ///     result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// Note it drives `_int_to_word`, not `to_cardinal` — equivalent here,
    /// since `val` has been made absolute and `to_cardinal` on a non-negative
    /// integer is just `_int_to_word` plus a no-op `.strip()`.
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
        // Restore GL's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };
        // `adjective` is in the signature and never read (module docs, bug 15).
        let _ = adjective;

        // `if val < 0: is_negative = True; val = abs(val)`, then the
        // `str(val).split(".")` extraction.
        //
        // The Int/Decimal arms are kept apart even though GL — alone among the
        // ported languages — cannot tell them apart (module docs, bug 16): an
        // int stringifies without a ".", so `parts` has one element and `right`
        // is 0, which is the same place the base's `isinstance(val, int)`
        // shortcut lands. Reached by a different route, so ported by that route.
        let (left, right, is_negative) = match val {
            CurrencyValue::Int(v) => {
                let is_negative = v < &BigInt::zero();
                (v.abs(), BigInt::zero(), is_negative)
            }
            CurrencyValue::Decimal { value: d, .. } => {
                // `val < 0` — a comparison, not `is_negative()`: Python reads
                // `-0.0 < 0` as False, and so does this.
                let is_negative = d < &BigDecimal::zero();
                let (left, right) = self.split_currency(&d.abs())?;
                (left, right, is_negative)
            }
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code silently becomes euros (module docs, bug 10).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);
        // Both sides always carry exactly two forms (the table is the const
        // literal built in `new`), so Python's unguarded `cr1[1]`/`cr2[1]` —
        // and these index expressions — cannot go out of range.
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::from(1);
        let left_str = self.int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — a truthiness test on `right`, so zero cents
        // drop the whole segment (bug 16), and `cents=False` drops it too
        // rather than falling back to `_cents_terse` (bug 14).
        if cents && !right.is_zero() {
            let cents_str = self.int_to_word(&right)?;
            // Python: `result += separator + cents_str + " " + ...`. No space
            // is inserted *before* the separator; GL's " " default supplies
            // the single space itself.
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        if is_negative {
            // `self.negword` raw — the trailing space in "menos " is what
            // separates it from the number. See NEGWORD.
            result = format!("{}{}", NEGWORD, result);
        }

        // Python's trailing `.strip()`. Nothing here can produce padding, but
        // reproduce it regardless.
        Ok(result.trim().to_string())
    }
}
