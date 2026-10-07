//! Port of `lang_TL.py` (Tagalog).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "bilyon" (10^9) and "trilyon" (10^12) (tl.wikipedia "Bilyon";
//! Wiktionary), composed like the million arm, and raises `OverflowError` from
//! 10^15, which `maxval()` reports. Where the notes below describe the digit
//! fallback ("1000000000", "no overflow check"), they describe Python; that
//! arm is now unreachable.
//!
//! Shape: **self-contained**. `Num2Word_TL` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `self.MAXVAL`. `to_cardinal` is overridden outright and drives a private
//! `_int_to_word` recursion. Consequently `cards`/`maxval`/`merge` stay at
//! their trait defaults here, and there is **no overflow check** — see the
//! 10^9 fallback below for what Tagalog does instead of raising.
//!
//! `is_title` is left `False` by `Num2Word_Base.__init__` and TL's
//! `to_cardinal` never calls `self.title(...)` anyway, so no title-casing.
//!
//! Everything in scope is overridden by TL itself, except the cheque surface:
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden ("una"/"ikalawa"/"ika-" + cardinal)
//!   * `to_ordinal_num` — overridden (`str(number) + "."`)
//!   * `to_year`        — overridden (`to_cardinal(val)`; the base class's
//!     BC/AD machinery is bypassed entirely, so `longval` is inert)
//!   * `to_currency`    — overridden outright (below). It never calls up into
//!     `Num2Word_Base.to_currency`, so `parse_currency_parts`, `pluralize`,
//!     `_money_verbose`, `_cents_verbose`, `_cents_terse`, `CURRENCY_PRECISION`
//!     and `CURRENCY_ADJECTIVES` are all unreachable from it.
//!   * `to_cheque`      — **not** overridden; `Num2Word_Base.to_cheque` runs,
//!     so the trait default (`currency::default_to_cheque`) is correct here and
//!     only `currency_forms`/`lang_name` need supplying. Verified against the
//!     live interpreter: `type(c).to_cheque.__qualname__` is
//!     `Num2Word_Base.to_cheque`.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` — TL declares
//! neither and `Num2Word_Base` leaves them empty — so `currency_precision`
//! stays at its default 100 for every code and `currency_adjective` at `None`.
//! Confirmed on the live interpreter, not read off the source.
//!
//! `Num2Word_TL` declares its own `CURRENCY_FORMS` dict literal and inherits
//! from `Num2Word_Base`, **not** `Num2Word_EUR` (MRO: `Num2Word_TL`,
//! `Num2Word_Base`, `object`). The `Num2Word_EN.__init__` shared-class-dict
//! rewrite documented in PORTING_CURRENCY.md therefore does not reach this
//! table: TL's EUR really is `("euro", "euros")` in its own right, and TL sees
//! none of the ~24 codes EN adds to the `Num2Word_EUR` dict.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Every item below looks wrong but is exactly
//! what Python emits, and each is confirmed by a row in the frozen corpus:
//!
//! 1. **No connectives anywhere.** Real Tagalog joins with the linker "at"
//!    ("isang daan at isa") and uses ligatures ("isang daan", not "isa daan").
//!    This module just space-joins bare stems, so `to_cardinal(101)` is
//!    `"isa daan isa"` and `to_cardinal(11)` is `"sampu isa"` — the latter is
//!    not a Tagalog word for 11 ("labing-isa") at all, it is literally
//!    "ten one". Preserved verbatim.
//! 2. **Values >= 10^9 are not converted at all.** `_int_to_word` has no
//!    billion branch and falls through to `return str(number)`, so
//!    `to_cardinal(10**9) == "1000000000"` — digits, not words — and
//!    `to_cardinal(10**21) == "1000000000000000000000"`. This is Tagalog's
//!    de facto (and entirely silent) MAXVAL: it neither raises `OverflowError`
//!    nor degrades gracefully. Modelled by the final arm of [`LangTl::int_to_word`].
//!    Note the sign is stripped *before* this fallback, so
//!    `to_cardinal(-10**9)` is `"minus 1000000000"`.
//! 3. **`to_ordinal` accepts negatives and zero and emits nonsense.** It has
//!    no sign/zero guard (unlike most modules, which raise `TypeError` via
//!    `errmsg_negord`), so it just prefixes the cardinal:
//!    `to_ordinal(0) == "ika-sero"` and `to_ordinal(-1) == "ika-minus isa"`.
//!    The `"ika-"` prefix is also glued onto multi-word cardinals, giving
//!    `to_ordinal(10**6) == "ika-isa milyon"` (prefix binds the first word only).
//! 4. **`ones[0]` is `""`,** and `_int_to_word` guards with
//!    `return self.ones[0] if self.ones[0] else "zero"` — a conditional whose
//!    true arm is unreachable, since `""` is falsy, so Python always said the
//!    English "zero"; its `pointword` was "point". Fixed
//!    (gladiaio/num2words2#154): the port says "sero" and "punto", the
//!    spoken Tagalog forms ("isa punto lima").
//! 5. **The `number < 0` arm of `_int_to_word` is dead code** on *every* path,
//!    not just the cardinal one: `to_cardinal` strips the `"-"` from the
//!    *string* before calling `int()`, and `to_currency` does `val = abs(val)`
//!    before it splits, so `_int_to_word` never sees a negative. It is
//!    reproduced anyway; if it ever ran it would double the negword
//!    (`"minus " + "minus " + ...`).
//! 6. **`to_currency` never raises on an unknown code.** Where every other
//!    module does `self.CURRENCY_FORMS[currency]` and converts the `KeyError`
//!    into `NotImplementedError`, TL does
//!    `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown code
//!    silently falls back to the **first-inserted** entry. So `currency="JPY"`
//!    renders Philippine pisos, and the corpus agrees:
//!    `currency:JPY 12.34 -> "sampu dalawa piso tatlumpu apat sentimo"`.
//!    See [`FALLBACK_CURRENCY`].
//! 7. **`to_currency` ignores `CURRENCY_PRECISION` entirely.** The divisor is
//!    the hard-coded 2 in `parts[1][:2].ljust(2, "0")`. TL's precision map is
//!    empty anyway, so this is unobservable *through TL* — but it means a
//!    3-decimal code (KWD/BHD) and a 0-decimal one (JPY) are both treated as
//!    2-decimal, on top of bug 6 renaming them "piso". Corpus:
//!    `currency:KWD 0.01 -> "sero piso isa sentimo"` (not `.../1000`), and
//!    `currency:JPY 0.5 -> "sero piso limampu sentimo"` — a subunit for a
//!    currency that has none.
//! 8. **`to_currency`'s `adjective` parameter is declared and never read.**
//!    `adjective=True` changes nothing. (`CURRENCY_ADJECTIVES` is `{}` too, so
//!    even `Num2Word_Base` would have been a no-op here.)
//! 9. **Cents truncate, they do not round.** `parts[1][:2]` slices the decimal
//!    string, so `1.005` -> `"00"` -> 0 cents (dropped entirely), and `0.999`
//!    -> `"99"` -> 99 cents, never 100. `Num2Word_Base.to_currency` would have
//!    quantized ROUND_HALF_UP instead.
//! 10. **`to_cheque` covers strictly fewer codes than `to_currency`.** Cheque
//!    is `Num2Word_Base`'s, which indexes `CURRENCY_FORMS[currency]` and does
//!    raise — so the 7 codes that bug 6 silently renders as pisos through
//!    `to_currency` raise `NotImplementedError` through `to_cheque`. Both
//!    halves are in the corpus and both are reproduced.
//!
//! # Error variants
//!
//! `to_cardinal`/`to_ordinal`/`to_ordinal_num`/`to_year` cannot raise: no
//! overflow check, no negative guard, no table lookup that can miss
//! (`ones`/`tens` are indexed only by a digit derived from `divmod`). All 324
//! integer-mode corpus rows are `ok: true`.
//!
//! `to_currency` cannot raise either — bug 6 removes the only `KeyError` site,
//! and all 108 currency corpus rows are `ok: true`.
//!
//! `to_cheque` raises exactly one thing: [`N2WError::NotImplemented`] with
//! Python's message, `Currency code "%s" not implemented for "%s"`. That comes
//! from `currency::default_to_cheque` via [`LangTl::currency_forms`] returning
//! `None`, so nothing here needs to construct it. 7 of the 9 cheque corpus rows
//! take that path.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dolyar / euro with sentimo, as fil does. Examples
//! in these docs that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword`, verbatim from `setup()` — note the **trailing space**, which
/// is load-bearing: `to_cardinal` concatenates it directly (it does not go
/// through `Num2Word_Base.parse_minus`, which would `.strip()` and re-add one).
const NEGWORD: &str = "minus ";

/// `self.pointword`: "punto" (Python had the English "point"; #154), reached on the float/Decimal path
/// (see [`LangTl::cardinal_from_str`] / [`LangTl::to_cardinal_float`]).
const POINTWORD: &str = "punto";

/// `_int_to_word`'s zero: "sero" where Python said the English "zero" (#154).
const ZERO_WORD: &str = "sero";

/// `self.ones`. Index 0 is `""` and is never returned (see bug 4).
const ONES: [&str; 10] = [
    "", "isa", "dalawa", "tatlo", "apat", "lima", "anim", "pito", "walo", "siyam",
];

/// `self.tens`. Index 0 is `""` and is unreachable (the `< 100` branch is only
/// entered when `number >= 10`, so `tens_val >= 1`).
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

/// The Python class name, for `Num2Word_Base.to_cheque`'s NotImplementedError
/// message (`self.__class__.__name__`).
const LANG_NAME: &str = "Num2Word_TL";

/// `Num2Word_TL.to_currency`'s own default `separator=" "` — a bare space.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_TL.to_currency` declares `separator=" "`, but the `Lang` trait
/// carries no per-language parameter defaults, and both `__init__.py`'s
/// currency fast path and `bench/diff_test.py` substitute
/// `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default, not TL's —
/// before the value crosses the boundary. By then "caller omitted separator"
/// and "caller explicitly passed a comma" are the same `&str`, and the
/// information needed to tell them apart no longer exists on this side.
///
/// So `,` is read back as the unset sentinel and TL's own default restored.
/// This is the only reading the oracle supports: every float row of the `tl`
/// currency corpus was generated by `num2words(v, lang="tl", to="currency",
/// currency=c)` with no `separator=`, and all 54 of them expect a bare space
/// ("sampu dalawa euros tatlumpu apat cents", not "...euros,tatlumpu apat
/// cents").
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " " where Python would give ",". Expressing that case
/// needs `Option<&str>` in the trait signature, which lives in `base.rs` —
/// outside this port's remit. Flagged in the port report. `lang_bo.rs`,
/// `lang_ca.rs`, `lang_es.rs`, `lang_eu.rs` and `lang_fo.rs` resolve the
/// identical conflict the same way.
const SEPARATOR_UNSET: &str = ",";

/// `to_currency`'s fallback code — see bug 6.
///
/// Python spells it `list(self.CURRENCY_FORMS.values())[0]`, i.e. "whichever
/// entry was inserted first". `CURRENCY_FORMS` is a dict literal ordered
/// `PHP, USD, EUR` and dicts preserve insertion order, so that is PHP. Pinning
/// the *key* rather than reproducing a positional lookup keeps the intent
/// legible; a `HashMap` has no insertion order to index into anyway.
///
/// Verified against the live interpreter rather than read off the source:
/// `list(CONVERTER_CLASSES["tl"].CURRENCY_FORMS.keys())` == `['PHP', 'USD',
/// 'EUR']`.
const FALLBACK_CURRENCY: &str = "PHP";

/// The hard-coded 2 in `parts[1][:2].ljust(2, "0")` — see bugs 7 and 9.
///
/// Deliberately *not* `currency_precision(code)`: TL's `to_currency` never
/// consults `CURRENCY_PRECISION`, so this stays 100 even for KWD/BHD/JPY.
const CENT_DIVISOR: i64 = 100;

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "bilyon"), (12, "trilyon")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

pub struct LangTl {
    /// `Num2Word_TL.CURRENCY_FORMS`.
    ///
    /// Built once in [`LangTl::new`] and stored, never per call: the binding
    /// holds the converter in a `OnceLock`, so this table is constructed
    /// exactly once per process.
    ///
    /// Each entry is a 2-tuple of 2-tuples, `((unit_sg, unit_pl), (sub_sg,
    /// sub_pl))`. The arity is load-bearing: `to_currency` indexes `[1]` for
    /// the plural and `Num2Word_Base.to_cheque` takes `cr1[-1]`, so both forms
    /// must be present — including for PHP, where the two are *identical*
    /// ("piso"/"piso"). Collapsing PHP to a single form would still print
    /// correctly today but would silently change `cr1[-1]`'s meaning.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangTl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTl {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        // Insertion order is irrelevant to a HashMap; FALLBACK_CURRENCY
        // captures the one place Python's ordering was observable.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("PHP", &["piso", "piso"][..], &["sentimo", "sentimo"][..]),
            ("USD", &["dolyar", "dolyar"][..], &["sentimo", "sentimo"][..]),
            ("EUR", &["euro", "euro"][..], &["sentimo", "sentimo"][..]),
        ]
        .into_iter()
        .map(|(k, u, s)| (k, CurrencyForms::new(u, s)))
        .collect();
        LangTl { currency_forms }
    }

    /// The `(left, right)` split at the head of `Num2Word_TL.to_currency`, for
    /// a value already made non-negative.
    ///
    /// Python works on the *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Taking the first two fractional digits and right-padding to two is
    /// exactly `trunc(frac * 100)` for any plain decimal string — `"5"` →
    /// `"50"` → 50 is `trunc(0.5 * 100)`, `"345"` → `"34"` is
    /// `trunc(0.345 * 100)` — so this computes the split arithmetically rather
    /// than round-tripping the value back through `BigDecimal`'s `Display`.
    /// Identical results, one less dependency on a formatting impl we do not
    /// control. (`lang_eu.rs` and `lang_fo.rs` port the same idiom the same
    /// way.)
    ///
    /// A true `int` has no "." in `str(val)`, so `len(parts) > 1` is false and
    /// `right` is 0 — which is also what `frac == 0` gives here. TL therefore
    /// needs no `isinstance(val, int)` branch: it never had one, and the
    /// int/float distinction is invisible in its output. `1.0` reaches
    /// `parts[1] == "0"` (truthy!), so Python computes
    /// `int("0".ljust(2, "0")) == 0` and then `if cents and right:` drops the
    /// segment on the *value* being 0 — not on the type. `Int(1)` and
    /// `Decimal(1.0)` therefore both give "isa euro", which the corpus
    /// confirms for both `arg: "1"` and `arg: "1.0"`.
    ///
    /// The one input this cannot reproduce is a float whose `str()` is
    /// scientific notation (`1e+16`, `1e-05`): Python feeds that to `int()` and
    /// raises `ValueError`. `str()` already happened on the Python side, and
    /// `BigDecimal::from_str` parses `"1e-05"` and `"0.00001"` to the identical
    /// value+scale, so the two are no longer distinguishable here. No corpus
    /// row exercises it; flagged in the port report.
    fn split_currency(val: &CurrencyValue) -> (BigInt, BigInt) {
        match val {
            CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let abs = d.abs();
                // int(parts[0]) — with_scale(0) truncates via BigInt division,
                // and abs is non-negative, so this is floor as well.
                let left = abs.with_scale(0).as_bigint_and_exponent().0;
                let frac = &abs - BigDecimal::from(left.clone());
                let right = (frac * BigDecimal::from(CENT_DIVISOR))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                (left, right)
            }
        }
    }

    /// Port of `Num2Word_TL._int_to_word`.
    ///
    /// Infallible — mirrors Python, which cannot raise here (see module docs).
    /// Recursion depth is bounded at 4 (milyon → libo → daan → ones).
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            // Python: `return self.ones[0] if self.ones[0] else "zero"`.
            // ones[0] == "" is falsy, so the else arm always wins.
            return ZERO_WORD.to_string();
        }

        if number.is_negative() {
            // Dead code on every in-scope path (see bug 5), reproduced anyway.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        // From here on `number >= 1`, so div_mod_floor == Python's divmod
        // (both operands positive: floor and truncation agree).

        if number < &ten {
            // 1..=9 — to_usize always succeeds.
            return ONES[number.to_usize().unwrap()].to_string();
        }

        if number < &hundred {
            let (tens_val, ones_val) = number.div_mod_floor(&ten);
            let t = TENS[tens_val.to_usize().unwrap()];
            if ones_val.is_zero() {
                return t.to_string();
            }
            return format!("{} {}", t, ONES[ones_val.to_usize().unwrap()]);
        }

        if number < &thousand {
            let (hundreds_val, remainder) = number.div_mod_floor(&hundred);
            // Python indexes `self.ones[hundreds_val]` directly — no recursion —
            // so 100..=999 always yields a bare "<ones> daan" head.
            let mut result = format!("{} {}", ONES[hundreds_val.to_usize().unwrap()], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &million {
            let (thousands_val, remainder) = number.div_mod_floor(&thousand);
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &billion {
            let (millions_val, remainder) = number.div_mod_floor(&million);
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

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

        // Python: `return str(number)` — "Fallback for very large numbers".
        // Emits digits, not words. See bug 2.
        number.to_string()
    }

    /// The body of `Num2Word_TL.to_cardinal` when its argument is a
    /// float/Decimal, driven by `str(number)`:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
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
    /// Three details, all shared with the reviewed sibling ports (`lang_pa`,
    /// `lang_as`):
    ///
    /// * The sign is stripped **textually** (`n.startswith("-")`), so `-0.0`
    ///   — whose `str` is `"-0.0"` — keeps its negword even though the value is
    ///   not `< 0`: Python answers "minus sero punto sero". (Reachable only for
    ///   `FloatValue::Float`; a `BigDecimal` cannot carry negative zero — see
    ///   the report.)
    /// * `split(".", 1)` caps at one split, so a second dot would stay inside
    ///   `right` and detonate in the digit loop (`int('.')` → `ValueError`)
    ///   rather than being ignored. `str()` never produces one.
    /// * `int(left)` runs *before* the digit loop, so for a `str` like
    ///   `"1.5e+16"` the failing literal reported is `'e'`, not `"1.5e+16"` —
    ///   order is load-bearing for the `ValueError` message.
    ///
    /// Note `_int_to_word` is called, **not** `to_cardinal`: identical for every
    /// non-negative value (the only kind reachable here, the sign having been
    /// peeled), but it is what Python writes.
    ///
    /// Python builds the fractional part by appending `word + " "` per digit and
    /// then `.strip()`s; a space-separated join plus a trailing `.trim()` is
    /// byte-identical. The `.trim()` is otherwise a no-op (`int_to_word` never
    /// returns "" — 0 short-circuits to "zero"), kept for fidelity.
    fn cardinal_from_str(&self, number: &str) -> Result<String> {
        let n = number.trim();
        let (n, mut ret) = match n.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD.to_string()),
            None => (n, String::new()),
        };

        let Some(dot) = n.find('.') else {
            // else: (ret + self._int_to_word(int(n))).strip()
            ret.push_str(&self.checked_int_to_word(&py_int(n)?)?);
            return Ok(ret.trim().to_string());
        };

        // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
        let (left, right) = (&n[..dot], &n[dot + 1..]);
        ret.push_str(&self.checked_int_to_word(&py_int(left)?)?);
        ret.push(' ');
        ret.push_str(POINTWORD);
        ret.push(' ');

        // for digit in right: ret += _int_to_word(int(digit)) + " " — iterate
        // *characters*, one per fractional digit.
        let mut first = true;
        for d in right.chars() {
            if !first {
                ret.push(' ');
            }
            first = false;
            let mut buf = [0u8; 4];
            ret.push_str(&self.checked_int_to_word(&py_int(d.encode_utf8(&mut buf))?)?);
        }
        Ok(ret.trim().to_string())
    }
}

impl Lang for LangTl {
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

    /// `to_ordinal(float/Decimal)`. TL's `to_ordinal` compares the raw value
    /// first — `number == 1` / `number == 2` are *numeric* equality, so the
    /// whole floats 1.0 / 2.0 (and `Decimal("1.0")`) hit the special forms
    /// "una" / "ikalawa". Everything else is `"ika-" + self.to_cardinal(number)`,
    /// i.e. the float string grammar with the prefix: "ika-lima punto sero".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let (is_one, is_two) = match value {
            FloatValue::Float { value, .. } => (*value == 1.0, *value == 2.0),
            FloatValue::Decimal { value, .. } => (
                *value == BigDecimal::from(1),
                *value == BigDecimal::from(2),
            ),
        };
        if is_one {
            return Ok("una".to_string());
        }
        if is_two {
            return Ok("ikalawa".to_string());
        }
        Ok(format!("ika-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — the repr the
    /// binding computed, dot appended, sign and exponent form included
    /// ("-0.0.", "1e+16.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, with the Inf
    /// interception: Python parses "Infinity" fine and the ValueError only
    /// fires later, inside TL's `int("Infinity")` (`to_cardinal` reads
    /// `str(number)`, strips the sign, finds no "." and calls `int()`).
    /// The binding otherwise hard-codes `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any TL code runs, so the
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

    /// The float/Decimal path. `Num2Word_TL` never defines `to_cardinal_float`;
    /// its `to_cardinal` handles both int and float via `str(number)` and
    /// branches on `"." in n`. `Num2Word_Base.to_cardinal_float`/`float2tuple`
    /// are therefore **never reached** — which matters here, because the base
    /// path reconstructs the fractional digits from `float2tuple`'s binary
    /// `abs(value-pre) * 10**precision` and its `< 0.01` floor/round heuristic,
    /// whereas TL reads them straight out of `repr(float)`. The two disagree on
    /// the **last** fractional digit of high-precision floats (e.g.
    /// `-9.388200339328929` ends in "siyam" via `str`, but the base path floors
    /// the scaled product to "…walo"), so inheriting the default is wrong.
    ///
    /// `precision_override` (the `precision=` kwarg) is accepted by the
    /// dispatcher but TL's `to_cardinal` takes no such parameter and reads every
    /// digit of `str(value)` regardless, so it is dropped — confirmed on the
    /// live interpreter (`num2words(2.675, lang='tl', precision=1)` is
    /// unchanged).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            // Python's str(float) == repr(float). The raw f64 crosses the
            // boundary so repr can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        self.cardinal_from_str(&n)
    }

    /// Port of `Num2Word_TL.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input, peels a leading `"-"` into `ret`, then
    /// looks for `"."`. `str(int)` never contains one, so integers always take
    /// the `else` branch: `(ret + self._int_to_word(int(n))).strip()`. The
    /// float branch (`pointword` + per-digit decimals) is handled by
    /// [`LangTl::to_cardinal_float`] / [`LangTl::cardinal_from_str`], since the
    /// Rust dispatcher routes non-integers there rather than through this
    /// `&BigInt` entry point.
    ///
    /// Peeling the sign off the *string* means `_int_to_word` receives the
    /// absolute value; `BigInt::abs` reproduces `n[1:]` exactly (there is no
    /// negative zero to disagree about).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // `.strip()` — only ever trims NEGWORD's trailing space when
        // _int_to_word returns "" (which it cannot), so it is a no-op in
        // practice. Kept for fidelity.
        Ok(format!("{}{}", ret, self.checked_int_to_word(&n)?)
            .trim()
            .to_string())
    }

    /// Port of `Num2Word_TL.to_ordinal`.
    ///
    /// No negative/zero guard and no float guard — see bug 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("una".to_string()); // first
        }
        if value == &BigInt::from(2) {
            return Ok("ikalawa".to_string()); // second
        }
        Ok(format!("ika-{}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_TL.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Not the trait default (which returns the bare value) — TL appends a
    /// period, and does so for negatives too: `to_ordinal_num(-1) == "-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_TL.to_year`: `return self.to_cardinal(val)`.
    ///
    /// TL discards `longval` and bypasses `Num2Word_Base.to_year` entirely, so
    /// there is no BC/AD suffix and no two-chunk ("nineteen eighty-four")
    /// reading: `to_year(-44) == "minus apatnapu apat"`, not "44 BC".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError.
    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `Num2Word_TL.CURRENCY_FORMS[code]`.
    ///
    /// Only `Num2Word_Base.to_cheque` reads this through the trait — TL's own
    /// `to_currency` bypasses it for the `.get(...)`-with-fallback of bug 6, so
    /// returning `None` here is what makes an unknown code raise on the cheque
    /// path while still rendering pisos on the currency path.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // `currency_adjective` and `currency_precision` stay at their trait
    // defaults (`None` / 100): TL declares neither map and `Num2Word_Base`
    // leaves both `{}`, so `CURRENCY_PRECISION.get(code, 100)` is 100 for every
    // code — which is what `to_cheque` uses to print "56/100". `pluralize` also
    // stays at the raising default: it is abstract in `Num2Word_Base` and
    // nothing TL can reach ever calls it (TL's `to_currency` selects the plural
    // inline with `left != 1`, and `to_cheque` takes `cr1[-1]` unconditionally).
    // `cardinal_from_decimal` likewise stays at its raising default — TL has no
    // fractional-cents path to reach it.

    /// Port of `Num2Word_TL.to_currency`:
    ///
    /// ```python
    /// def to_currency(self, val, currency="PHP", cents=True, separator=" ",
    ///                 adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
    ///
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///
    ///     if is_negative:
    ///         result = self.negword + result
    ///
    ///     return result.strip()
    /// ```
    ///
    /// A complete override: `Num2Word_Base.to_currency` is never called, so
    /// none of `parse_currency_parts`, `pluralize`, `_money_verbose`,
    /// `_cents_verbose`, `_cents_terse`, `CURRENCY_PRECISION` or
    /// `CURRENCY_ADJECTIVES` is reachable from here. Infallible — there is no
    /// `NotImplementedError` path (bug 6) and no table index that can escape
    /// its range.
    ///
    /// Note the plural is selected by `left != 1` / `right != 1` directly
    /// rather than by `pluralize`, which `Num2Word_TL` leaves abstract. Note
    /// also that `_int_to_word` is called, **not** `to_cardinal`: identical for
    /// every non-negative value (the only kind that gets here), but it is what
    /// Python writes.
    ///
    /// The `default_to_currency` "pure ints never show cents" branch has no
    /// analogue here and must not be reintroduced: TL reaches the same outcome
    /// through the `if cents and right:` truthiness test instead. See
    /// [`LangTl::split_currency`].
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // declared by Python, never read — bug 8.
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Python: `if val < 0: is_negative = True; val = abs(val)`. The abs is
        // folded into split_currency, which every later step reads through.
        let is_negative = val.is_negative();
        let (left, right) = Self::split_currency(val);

        // `.get(currency, list(CURRENCY_FORMS.values())[0])` — bug 6. Python
        // evaluates the default eagerly on every call, but it cannot fail:
        // PHP is always present.
        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            None => &self.currency_forms[FALLBACK_CURRENCY],
        };

        let one = BigInt::one();
        let left_str = self.checked_int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left != one {
                &forms.unit[1]
            } else {
                &forms.unit[0]
            }
        );

        // `if cents and right:` — a truthiness test on the cent *count*, not on
        // the type of `val`. This is what makes `1.0` print "isa euro" with no
        // cents segment even though it is a float.
        if cents && !right.is_zero() {
            let cents_str = self.checked_int_to_word(&right)?;
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            });
        }

        if is_negative {
            // negword carries a trailing space ("minus "), so this is the seam
            // the final strip() does *not* touch.
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}

// ---- str(float) / str(Decimal) reproduction ---------------------------------
//
// `Num2Word_TL.to_cardinal` promotes `str(number)` from a formatting detail to
// the entire specification of the float path, so these reproduce CPython's
// `repr`/`str` exactly. They are value-agnostic and copied verbatim from the
// reviewed sibling ports (`lang_pa`, `lang_as`), where they were fuzzed over
// millions of values; nothing here is TL-specific.

/// Shortest round-trip decimal digits of a **non-negative, finite** f64, plus
/// the decimal point position `decpt` (value == 0.d1d2… × 10^decpt, i.e. the
/// first digit sits just left of position `decpt`). This is CPython's
/// `_Py_dg_dtoa(mode 0)`, the David Gay shortest-representation used by `repr`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with Gay on the digit
/// *count* and, almost always, the digits. It disagrees on **exact ties**: when
/// the value sits precisely halfway between two shortest candidates, Gay's dtoa
/// picks the one with an **even** last digit while Rust rounds half **up**.
/// `repr(-78198386800398.125)` is `'-78198386800398.12'`; Rust's `{:e}` says
/// `…13`. This mirrors the reviewed sibling implementation (`lang_as`), fuzzed
/// over millions of values, where that tie is the only divergence.
///
/// Detecting the tie needs no bignum. Write `a = m·2^e` with `m` odd and let
/// `q = digits.len() - decpt`. The tie condition reduces to `e + q + 1 == 0`,
/// plus (when `q < 0`) `5^-q | m` with `-q <= 22`. In a tie `2k+1 == m·5^q`
/// (or `m / 5^-q`), and since `5 ≡ 1 (mod 4)` that odd integer is `≡ m (mod 4)`,
/// so `k` is even exactly when `m % 4 == 1`. The fix-up steps Rust's odd last
/// digit toward the even neighbour, `k`'s parity choosing the direction.
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let mut digits: Vec<u8> = mant.bytes().filter(|c| *c != b'.').collect();
    let mut decpt: i32 = exp.parse::<i32>().expect("{:e} exponent is an integer") + 1;

    // Decompose a == m * 2**e exactly, then reduce m to odd.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mut m, mut e) = if biased == 0 {
        (frac, -1074i32) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };
    if m == 0 {
        // a == 0.0: dtoa reports digits "0", decpt 1. No tie to break.
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }
    let z = m.trailing_zeros() as i32;
    m >>= z;
    e += z;

    let q = digits.len() as i32 - decpt;
    let mut tie = e + q + 1 == 0;
    if tie && q < 0 {
        let r = -q as u32;
        tie = r <= 22 && m % 5u64.pow(r) == 0;
    }
    if !tie {
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }

    let last = digits[digits.len() - 1] - b'0';
    if last % 2 == 1 {
        if m % 4 == 1 {
            // k even: Python wants k, Rust gave k+1. Odd last digit, so no borrow.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k odd: Python wants k+1, Rust gave k. Carry like dtoa's roundoff.
            let mut i = digits.len();
            loop {
                if i == 0 {
                    digits.insert(0, b'1');
                    decpt += 1;
                    break;
                }
                i -= 1;
                if digits[i] == b'9' {
                    digits[i] = b'0';
                } else {
                    digits[i] += 1;
                    break;
                }
            }
        }
        // dtoa never emits trailing zeros; stripping them leaves decpt alone.
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`). This is CPython's
/// `format_float_short(..., 'r', ...)` in `pystrtod.c`.
///
/// Rust's `{}` cannot stand in: it never switches to exponent notation
/// (`format!("{}", 1e16_f64)` is `"10000000000000000"`, where Python says
/// `'1e+16'` — the difference that makes `to_cardinal(1e16)` raise `ValueError`)
/// and it prints `1`, not `1.0`, for integral floats. The rules, from
/// `format_float_short`:
///
/// * exponent notation iff `decpt <= -4 || decpt > 16` (the `> 16`, not `> 17`,
///   is deliberate upstream);
/// * the exponent is `%+.02d`: signed, zero-padded to two digits — `1e-05` but
///   `1e+100`;
/// * `Py_DTSF_ADD_DOT_0` appends `.0` to an otherwise integral fixed-notation
///   result, but never in exponent notation (`repr(1e16) == '1e+16'`);
/// * `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and to_cardinal strips
    // that minus textually into a negword.
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_digits(value.abs());
    let ndigits = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let exp = decpt - 1;
        let mut mant = String::from(&digits[..1]);
        if digits.len() > 1 {
            mant.push('.');
            mant.push_str(&digits[1..]);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        // 0.5 -> decpt 0 -> "0." + "" + "5"; 0.01 -> decpt -1 -> "0." + "0" + "1".
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        // Integral: pad right with zeros, then ADD_DOT_0. 1.0 -> "1" + ".0".
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - ndigits) as usize)
        )
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__` with `eng=False` and
/// the default context (uppercase `E`).
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`: the shim builds this with `BigDecimal::from_str(str)`,
/// preserving trailing zeros and negative exponents, so `"1.10"` round-trips as
/// `(110, 2)` and `"1E+16"` as `(1, -16)`.
///
/// # The negative-zero hole
///
/// `Decimal` carries `_sign` independently, so `Decimal("-0.0")` is signed zero
/// and `str()` gives `'-0.0'`; TL then strips that minus textually and answers
/// "minus sero punto sero". A `BigDecimal` cannot represent it — its `int_val`
/// is a `BigInt` with no negative zero, so `BigDecimal::from_str("-0.0")` has
/// already discarded the sign before this function runs. We emit `'0.0'` and
/// drop the negword. The discriminator is the original string, which the
/// `FloatValue::Decimal` boundary (in `num2words2-py`) does not carry, so the
/// fix is out of this port's remit. Flagged in the report. Blast radius is
/// exactly the fixed-notation negative zeros (`-0` … `-0.000000`); beyond that
/// the `E±n` form raises `ValueError` regardless of sign.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // i128 so that `-scale` cannot overflow for a pathological i64::MIN scale.
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string(); // Decimal._int
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let d = dotplace as usize;
        (int_digits[..d].to_string(), format!(".{}", &int_digits[d..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — signed, but *not* zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)`, for the fragments [`LangTl::cardinal_from_str`] hands it —
/// always ASCII pieces of `str(float)`/`str(Decimal)`. Ports the underscore rule
/// (`int("1_0") == 10`, `int("1_")` raises) and, crucially, the error message,
/// which formats the original argument with `%.200R` (i.e. `repr(s)`); every
/// literal that reaches here is plain ASCII, so `'{}'` matches what `repr`
/// prints. Raises the same `ValueError` Python raises (`N2WError::Value`).
fn py_int(s: &str) -> Result<BigInt> {
    let err = || {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    };
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    // int() permits '_' as a digit separator, but not leading, trailing or doubled.
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body.chars().all(|c| c.is_ascii_digit() || c == '_')
    {
        return Err(err());
    }
    let digits: String = body.chars().filter(|c| *c != '_').collect();
    let n: BigInt = digits.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}
