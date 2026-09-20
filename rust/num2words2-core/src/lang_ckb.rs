//! Port of `lang_CKB.py` (Central Kurdish / Sorani).
//!
//! # Deliberate divergence from upstream: the script
//!
//! `lang_CKB.py` spells every numeral in **Latin transliteration** ("yek",
//! "çwar", "hezar", "sed"). Central Kurdish is written in the Perso-Arabic
//! Sorani alphabet; the Latin (Hawar) alphabet is Kurmanji's, and Kurmanji
//! already has its own converter in this crate (`lang_ku.rs`), which is where
//! Latin-script Kurdish output belongs. The tables here are the Sorani
//! spellings: "یەک", "چوار", "هەزار", "سەد".
//!
//! This is a lexicon change only — the composition rules, the multiplier
//! guards, the 10^9 digit cliff and the currency fallback are all still
//! ported verbatim, bug for bug, and `verify_ordinal` is still not called.
//!
//! ## What is *not* fixed: the ordinal after a vowel
//!
//! ـەم is the genuine Sorani ordinal marker, and gluing it on is already
//! right after a consonant — which is most of the lexicon (یەکەم, چوارەم,
//! پێنجەم, شەشەم, حەوتەم, هەشتەم, بیستەم, چلەم, شەستەم, نەوەدەم, سەدەم,
//! هەزارەم, ملیۆنەم). After a **vowel-final** stem standard Sorani inserts a
//! linking consonant, and the sources disagree on which one: Omniglot writes
//! دووهەم / سێهەم / نۆهەم / دەهەم while other references write سێیەم /
//! نۆیەم / دەیەم. The vowel-final stems in these tables are سێ, نۆ, دە,
//! پەنجا, حەفتا, هەشتا and every ـە-final teen.
//!
//! Both variants are attested and choosing between them needs a native
//! speaker, so this module keeps upstream's blind suffix there rather than
//! guessing. `to_ordinal(3)` is therefore سێەم, which is wrong but no more
//! wrong than it was; it is tracked separately from the script fix.
//!
//! Registry check: `CONVERTER_CLASSES["ckb"]` is `lang_CKB.Num2Word_CKB()`
//! (`__init__.py:381`), which is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_CKB` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! enters the `__init__` branch that builds `self.cards` / `set_numwords()` /
//! `self.MAXVAL`. Both `self.cards` and `self.MAXVAL` therefore **never exist**
//! on a CKB instance. `to_cardinal` is overridden outright and drives a
//! recursive `_int_to_word`, so `cards`/`maxval`/`merge` stay at their trait
//! defaults here and are never consulted. There is **no overflow check** and
//! no `OverflowError` path — see bug 1 below for what happens instead.
//!
//! Inherited from `Num2Word_Base`, but every one of them is overridden by CKB,
//! so nothing is picked up from the base class in the four modes in scope:
//!   * `to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year` — all defined
//!     in `lang_CKB.py` itself.
//!
//! # The float/Decimal path
//!
//! CKB does **not** override `to_cardinal_float`, and it never reaches
//! `Num2Word_Base.to_cardinal_float` either: `num2words()` dispatches every
//! non-integer straight to `converter.to_cardinal(number)`, and CKB's override
//! swallows floats itself. So `base.float2tuple` — and with it the entire f64
//! `post = abs(value - pre) * 10**precision` / `< 0.01` heuristic that
//! `floatpath.rs` exists to reproduce — **is never executed for CKB**.
//!
//! CKB's first statement is `n = str(number).strip()`, and everything after it
//! is string surgery on that repr. The consequences run the opposite way to the
//! base class's:
//!
//! * **No f64 artefacts.** `repr` is shortest-round-trip, so `2.675` spells
//!   `"2.675"` and `1.005` spells `"1.005"` — the exact cases where
//!   `float2tuple` produces `674.9999999999998` and needs rescuing. Here the
//!   noise never appears, so there is nothing to rescue and no `round()` to get
//!   banker's-wrong. Corpus confirms both: `2.675` → `"دوو خاڵ شەش حەوت پێنج"`.
//! * **No rounding, no padding, no `precision`.** The fraction is however many
//!   digits the repr happened to carry, one word each. `Decimal("1.10")` keeps
//!   its written trailing zero (`"یەک خاڵ یەک سفر"`) where the float `1.1`
//!   could not — the `Decimal`/`float` split is *visible* in the output, not
//!   just in the last bits.
//! * **`precision=` is inert** — see [`LangCkb::to_cardinal_float`].
//! * **An exponential repr is a `ValueError`, not a number** — see bug 10.
//!
//! `Num2Word_CKB.to_cardinal` is byte-for-byte `Num2Word_CEB.to_cardinal` with
//! `"siro"` swapped for `"سفر"`, so [`LangCkb::cardinal_from_repr`] and
//! `lang_ceb.rs`'s `cardinal_from_repr` are the same port. The two crates'
//! copies of [`py_str_f64`] / [`py_str_decimal`] are deliberate duplicates: a
//! shared helper would have to live in `base.rs` or `floatpath.rs`, which this
//! phase may not touch. See `concerns`.
//!
//! `Num2Word_Base.to_cardinal` would have called `self.title(...)`; CKB's
//! override does **not**, so `is_title`/`exclude_title` are inert for the four
//! modes in scope. `setup()`'s `exclude_title = ["و", "خاڵ", "نێگەتیڤ"]` is
//! mirrored on the struct for fidelity but is never read: `is_title` stays
//! `false` (set in `Num2Word_Base.__init__`) and `title` is never reached.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, and are confirmed by the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the decimal digits.** The
//!    final line of the if-chain is a bare `return str(number)` — no billion
//!    word exists in the table (`setup` stops at `million = "ملیۆن"`). So
//!    `to_cardinal(10**9)` == `"1000000000"`, and `to_ordinal(10**9)` ==
//!    `"1000000000ەم"` — digits with a word suffix glued on. Corpus rows for
//!    10^9, 1234567890, 10^10, 10^11, 10^12, 10^15, 10^18 and 10^21 all
//!    confirm this. It is unbounded: no exception is ever raised, however
//!    large the input. Modelled in [`LangCkb::int_to_word`].
//!    Negatives compose with it: `to_cardinal(-10**9)` == `"نێگەتیڤ 1000000000"`.
//! 2. **`tens[1] == "دە"` is dead.** 10..=19 are caught by the `number < 20`
//!    branch and served from `teens`, so the `< 100` branch never divides down
//!    to a tens index of 1. The entry is kept verbatim in [`TENS`] anyway.
//! 3. **`ones[0] == ""` is dead** on the integer path — `_int_to_word` handles
//!    0 before the `number < 10` branch. Python only reads `ones[0]` from the
//!    float branch, where `self.ones[int(digit)] or "سفر"` relies on the
//!    empty string being *falsy* to turn a fraction digit 0 into a word. That
//!    is a different mechanism from `_int_to_word`'s explicit `if number == 0`;
//!    both are live and both are needed. Corpus: `1.0` → `"یەک خاڵ سفر"`.
//! 4. **`to_ordinal` is pure concatenation**: `to_cardinal(n) + ەم`, with no
//!    joint and no stem change. So the suffix lands on the *last* word only:
//!    `to_ordinal(999)` == `"نۆ سەد و نەوەد و نۆەم"`, and `to_ordinal(-1)` ==
//!    `"نێگەتیڤ یەکەم"`. Unlike most languages there is no negative-ordinal
//!    guard, so no `ValueError`/`TypeError` for negative ordinals.
//! 5. **`_int_to_word` would index `ones`/`teens` with a negative index** if it
//!    were ever handed a negative number (Python's negative indexing wraps, so
//!    `ones[-5]` would silently yield `"پێنج"`). It is unreachable: the only
//!    two callers, `to_cardinal` and `to_currency`, strip the sign / take
//!    `abs()` first. [`LangCkb::int_to_word`] is only ever called with a
//!    non-negative value here, mirroring that. The float path does not change
//!    this: `cardinal_from_repr` peels the `"-"` off the *string* before any
//!    `int()` runs, so `left` is always non-negative too.
//! 10. **A float whose repr goes exponential is a `ValueError`, not a number.**
//!     Nothing filters the repr before it reaches `int()`, so every value
//!     CPython spells with an `e` crashes — and *where* it crashes depends on
//!     whether the mantissa has a fractional part, which changes the message:
//!     * `1e16` → repr `"1e+16"` → no `"."` → `int("1e+16")` raises
//!       `ValueError: invalid literal for int() with base 10: '1e+16'`.
//!     * `1.5e16` → repr `"1.5e+16"` → splits to `left="1"`, `right="5e+16"`;
//!       `"یەک خاڵ پێنج"` is built successfully and then thrown away when the
//!       loop hits `int('e')` → `ValueError: invalid literal for int() with
//!       base 10: 'e'`. The offending token is the single *character*, not the
//!       whole fragment.
//!     * `1e-05` (i.e. any `abs(v) < 1e-4`) → `"1e-05"` → same as `1e16`.
//!     * `inf` / `nan` → `int("inf")` / `int("nan")` → same shape.
//!
//!     So CKB's float path is only total on `-4 < decpt <= 16`. Both messages
//!     are reproduced verbatim; see [`py_int`] and [`py_int_digit`].
//!
//! # Error variants
//!
//! `N2WError::Value` (Python `ValueError`), and only from the float path —
//! bug 10's `int()` failures on an exponential/non-finite repr. Every *integer*
//! input in the four modes in scope still returns `Ok`: CKB has no overflow
//! ceiling (bug 1), no negative-ordinal guard (bug 4) and no dict lookups that
//! can miss, and `str` of a `BigInt` is never exponential. The 20 non-`ok`
//! corpus rows for `ckb` are all `cheque:*` (`NotImplementedError`) and
//! `fraction` (`TypeError`), both out of scope.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `setup`: `self.negword`. Note the **trailing space** — CKB's `to_cardinal`
/// concatenates it directly (`self.negword + ...`) rather than joining with a
/// separator, so the space is what separates "نێگەتیڤ" from the number.
const NEGWORD: &str = "نێگەتیڤ ";

/// `setup`: `self.pointword`. Float path only; inert for the modes in scope.
const POINTWORD: &str = "خاڵ";

/// `_int_to_word`'s zero word. Not in `ones` — `ones[0]` is `""` (bug 3).
const ZERO: &str = "سفر";

/// `setup`: `self.ones`. Index 0 is `""` and dead on the integer path (bug 3).
const ONES: [&str; 10] = [
    "", "یەک", "دوو", "سێ", "چوار", "پێنج", "شەش", "حەوت", "هەشت", "نۆ",
];

/// `setup`: `self.teens`, indexed by `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "دە", "یانزە", "دوانزە", "سیانزە", "چواردە", "پانزە", "شانزدە", "حەڤدە", "هەژدە", "نۆزدە",
];

/// `setup`: `self.tens`, indexed by `number // 10`. Index 0 is `""` (dead: the
/// `< 100` branch is only reached for `number >= 20`) and index 1 is `"دە"`
/// (also dead — see bug 2).
const TENS: [&str; 10] = [
    "", "دە", "بیست", "سی", "چل", "پەنجا", "شەست", "حەفتا", "هەشتا", "نەوەد",
];

/// `setup`: `self.hundred`.
const HUNDRED: &str = "سەد";
/// `setup`: `self.thousand`.
const THOUSAND: &str = "هەزار";
/// `setup`: `self.million`. The largest scale word CKB defines — hence bug 1.
const MILLION: &str = "ملیۆن";

/// The joint between every pair of groups: `" و "` ("and").
const JOINT: &str = " و ";

/// `to_ordinal` / `to_ordinal_num` suffix, glued on with no separator.
///
/// ـەم is the real Sorani ordinal marker and is correct as-is after a
/// consonant, which covers most of the lexicon: یەکەم, چوارەم, پێنجەم,
/// شەشەم, حەوتەم, هەشتەم, بیستەم, چلەم, شەستەم, نەوەدەم, سەدەم, هەزارەم,
/// ملیۆنەم.
///
/// **Known gap, deliberately not fixed here.** After a *vowel-final* stem
/// standard Sorani inserts a linking consonant, and the sources disagree on
/// which: Omniglot writes دووهەم / سێهەم / نۆهەم / دەهەم (ـهەم) while other
/// references write سێیەم / نۆیەم / دەیەم (ـیەم). Both are attested. The
/// vowel-final stems in these tables are سێ, نۆ, دە, پەنجا, حەفتا, هەشتا and
/// every ـە-final teen. Picking between the two variants needs a native
/// speaker, so this module keeps upstream's blind suffix there rather than
/// guessing; see the module header.
const ORDINAL_SUFFIX: &str = "ەم";

/// The ceiling of `_int_to_word`'s word-producing branches. At or above this,
/// Python falls through to `return str(number)` (bug 1).
const BILLION: u64 = 1_000_000_000;

/// `self.__class__.__name__`, for `to_cheque`'s NotImplementedError message.
const LANG_NAME: &str = "Num2Word_CKB";

/// The subunit divisor CKB's `to_currency` **hardcodes**.
///
/// `Num2Word_CKB.to_currency` reads exactly two fractional digits —
/// `int(parts[1][:2].ljust(2, "0"))` — and never consults
/// `CURRENCY_PRECISION`. So every code is 2-decimal on the `to_currency` path,
/// including the 3-decimal (KWD/BHD) and 0-decimal (JPY) ones. See bug 8.
///
/// This is *not* `currency_precision`, which stays at the trait default of 100
/// because `Num2Word_CKB` inherits `Num2Word_Base.CURRENCY_PRECISION == {}`
/// (verified live: it is still the unmutated base dict). The two happen to
/// agree at 100, but they are independent knobs — `to_cheque` reads the former,
/// `to_currency` this one.
const CENTS_PER_UNIT: i64 = 100;

/// The code whose forms `to_currency` falls back to for an unknown currency.
///
/// Python writes `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
/// — the *first inserted* entry, since dicts preserve insertion order. The
/// class body lists IQD first, so the fallback is IQD. Verified live:
/// `list(CURRENCY_FORMS.keys()) == ['IQD', 'IRR', 'USD', 'EUR']`. See bug 7.
const FALLBACK_CURRENCY: &str = "IQD";

pub struct LangCkb {
    /// `setup`: `self.exclude_title`. Stored to mirror `setup()` faithfully;
    /// never read, because CKB's `to_cardinal` override never calls `title`.
    exclude_title: Vec<String>,
    /// `Num2Word_CKB.CURRENCY_FORMS`, built once (see [`LangCkb::new`]).
    ///
    /// A plain class attribute on `Num2Word_CKB` itself, so — unlike the
    /// `Num2Word_EUR` dict that `Num2Word_EN.__init__` rewrites in place — no
    /// other language's `__init__` can reach it. Verified live: the four
    /// literals in the class body are exactly what runs.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// The [`FALLBACK_CURRENCY`] entry, cloned out at construction so the
    /// unknown-code path in `to_currency` is a field read rather than a lookup
    /// that could fail.
    fallback_forms: CurrencyForms,
}

impl Default for LangCkb {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCkb {
    /// Builds the `CURRENCY_FORMS` table **once**. `to_currency` / `to_cheque`
    /// only ever read it; nothing is constructed per call.
    pub fn new() -> Self {
        // Insertion order mirrors the class body. The map itself is unordered,
        // so the "first entry" fallback is pinned by FALLBACK_CURRENCY instead.
        let iqd = CurrencyForms::new(&["دینار", "دینار"], &["فلس", "فلس"]);
        let mut currency_forms = HashMap::new();
        currency_forms.insert(FALLBACK_CURRENCY, iqd.clone());
        currency_forms.insert("IRR", CurrencyForms::new(&["ریاڵ", "ریاڵ"], &["دینار", "دینار"]));
        currency_forms.insert("USD", CurrencyForms::new(&["دۆلار", "دۆلار"], &["سەنت", "سەنت"]));
        currency_forms.insert("EUR", CurrencyForms::new(&["یۆرۆ", "یۆرۆ"], &["سەنت", "سەنت"]));

        LangCkb {
            exclude_title: vec!["و".to_string(), "خاڵ".to_string(), "نێگەتیڤ".to_string()],
            currency_forms,
            fallback_forms: iqd,
        }
    }

    /// Python's `parts = str(abs(val)).split(".")` → `(left, right)`.
    ///
    /// ```python
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Reproduced structurally on the parsed value rather than by re-rendering
    /// a string, because `BigDecimal`'s `Display` is not `str(float)`:
    ///
    /// * `left` is `str(...)` split at the dot, i.e. the digits before it —
    ///   truncation toward zero of a non-negative value. `with_scale(0)` is
    ///   exactly that (bigdecimal 0.4.10 implements it with `BigInt` division,
    ///   which truncates; `with_scale_round` is the rounding one).
    /// * `right` is `parts[1][:2]` right-padded to two digits. Slicing a digit
    ///   string to two chars *truncates* the rest and padding supplies missing
    ///   digits as zeros, which together are exactly
    ///   `trunc(frac * 100)`: `1.005` → `"005"[:2]` → `"00"` → `0`, and
    ///   `trunc(0.005 * 100) == trunc(0.5) == 0`. `2.675` → `"67"` → `67`, and
    ///   `trunc(67.5) == 67`. **Not** rounding — `1.005` yields 0 cents, not 1.
    ///
    /// A `CurrencyValue::Int` stringifies without a dot, so `parts` has length
    /// 1 and `right` is 0 — an int can never show cents. A float that happens
    /// to be whole (`1.0`) takes the `Decimal` arm, but `str(1.0) == "1.0"`
    /// gives `parts[1] == "0"` → `int("00")` → 0, so it lands on zero cents too
    /// and `if cents and right:` skips the segment either way. The two arms
    /// therefore agree here, unlike in `Num2Word_Base.to_currency`. Corpus:
    /// both `1` and `1.0` render `"یەک یۆرۆ"`.
    fn currency_parts(&self, val: &CurrencyValue) -> (BigInt, BigInt) {
        match val {
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let abs = d.abs();
                let left = abs.with_scale(0);
                let fraction = &abs - &left;
                let right = (fraction * BigDecimal::from(CENTS_PER_UNIT)).with_scale(0);
                (
                    left.as_bigint_and_exponent().0,
                    right.as_bigint_and_exponent().0,
                )
            }
        }
    }

    /// Port of `Num2Word_CKB._int_to_word`.
    ///
    /// Only ever called with a **non-negative** value (see bug 5): `to_cardinal`
    /// strips the `"-"` before recursing.
    ///
    /// Values `>= 10^9` take Python's final `return str(number)` — the decimal
    /// digits, not words (bug 1). That guard is hoisted here so the recursive
    /// worker can run on `u64`: below 10^9 every value, and every quotient and
    /// remainder derived from it, provably fits.
    fn int_to_word(&self, number: &BigInt) -> String {
        match number.to_u64() {
            Some(n) if n < BILLION => int_to_word_small(n),
            // >= 10^9 (or, unreachably, negative): Python's `return str(number)`.
            _ => number.to_string(),
        }
    }

    /// Port of `Num2Word_CKB.to_cardinal` operating on the repr, which is the
    /// form Python's own code works in:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "سفر")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The recursion really is string-level in Python — `to_cardinal(n[1:])`
    /// hands a `str` back to the same method, whose `str(number)` is then a
    /// no-op — so it is reproduced as such rather than folded into an `abs()`.
    /// One level deep in practice: no repr yields a second leading `"-"`.
    ///
    /// Note this is the *whole* method, integer inputs included. The
    /// [`Lang::to_cardinal`] impl above is the same algorithm specialised to a
    /// `BigInt`, where `str` can never contain a `"."` or an `"e"` and the two
    /// `int()` calls cannot fail; both are kept because the trait splits the
    /// integer and float entry points that Python fuses into one.
    ///
    /// Two details that look like slips but are Python's:
    ///
    /// * `split(".", 1)` splits on the *first* dot only, so a second dot would
    ///   land in `right` and reach `int()` as a character. Unreachable from a
    ///   repr.
    /// * `self.ones[0]` is `""`, which is falsy, so `or "سفر"` is what turns
    ///   a fraction digit 0 into a word (bug 3).
    fn cardinal_from_repr(&self, n: &str) -> Result<String> {
        // n = str(number).strip(). Python strips its own whitespace set and
        // Rust trims Unicode's; no repr contains either.
        let n = n.trim();

        // if n.startswith("-"): return (self.negword + self.to_cardinal(n[1:])).strip()
        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_repr(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        let (left, right) = match n.split_once('.') {
            Some(halves) => halves,
            // return self._int_to_word(int(n))
            None => return Ok(self.int_to_word(&py_int(n)?)),
        };

        // ret = self._int_to_word(int(left)) + " " + self.pointword
        //
        // `int(left)` is the whole integer part, so bug 1 applies at 10^9 and
        // above: it comes back as bare digits ("98746251323029 خاڵ نۆ نۆ").
        let mut ret = self.int_to_word(&py_int(left)?);
        ret.push(' ');
        ret.push_str(POINTWORD);

        // for digit in right: ret += " " + (self.ones[int(digit)] or "سفر")
        //
        // Per *character*, so there is no grouping, no rounding and no padding:
        // the fraction is however many digits the repr had. The partially built
        // `ret` is discarded if a later character is not a digit (bug 10).
        for digit in right.chars() {
            let word = ONES[py_int_digit(digit)?];
            ret.push(' ');
            ret.push_str(if word.is_empty() { ZERO } else { word });
        }

        // Cosmetic: `_int_to_word` never returns "", so nothing can be trimmed.
        Ok(ret.trim().to_string())
    }
}

/// The `< 10^9` body of `_int_to_word`, on `u64`.
///
/// The branch order mirrors Python's if-chain exactly. `divmod` on
/// non-negative operands is the same in both languages, so `/` and `%` are
/// direct equivalents here (no floor-division discrepancy to worry about —
/// the value is never negative).
fn int_to_word_small(number: u64) -> String {
    if number == 0 {
        return ZERO.to_string();
    }
    if number < 10 {
        return ONES[number as usize].to_string();
    }
    if number < 20 {
        return TEENS[(number - 10) as usize].to_string();
    }
    if number < 100 {
        // t, o = divmod(number, 10)
        let (t, o) = (number / 10, number % 10);
        let mut out = TENS[t as usize].to_string();
        if o != 0 {
            out.push_str(JOINT);
            out.push_str(ONES[o as usize]);
        }
        return out;
    }
    if number < 1_000 {
        // h, r = divmod(number, 100)
        let (h, r) = (number / 100, number % 100);
        // base = (self.ones[h] + " " if h > 1 else "") + self.hundred
        // h == 1 is bare "سەد" — no "یەک سەد". Corpus: 100 -> "سەد".
        let mut out = String::new();
        if h > 1 {
            out.push_str(ONES[h as usize]);
            out.push(' ');
        }
        out.push_str(HUNDRED);
        if r != 0 {
            out.push_str(JOINT);
            out.push_str(&int_to_word_small(r));
        }
        return out;
    }
    if number < 1_000_000 {
        // t, r = divmod(number, 1000)
        let (t, r) = (number / 1_000, number % 1_000);
        // No h > 1 style suppression here: 1000 -> "یەک هەزار", not "هەزار".
        let mut out = int_to_word_small(t);
        out.push(' ');
        out.push_str(THOUSAND);
        if r != 0 {
            out.push_str(JOINT);
            out.push_str(&int_to_word_small(r));
        }
        return out;
    }
    // number < 1_000_000_000, guaranteed by the caller.
    // m, r = divmod(number, 1000000)
    let (m, r) = (number / 1_000_000, number % 1_000_000);
    let mut out = int_to_word_small(m);
    out.push(' ');
    out.push_str(MILLION);
    if r != 0 {
        out.push_str(JOINT);
        out.push_str(&int_to_word_small(r));
    }
    out
}

/// Python's `int(s)` for a whole token, as `to_cardinal` calls it on `n` and on
/// `left`.
///
/// The `ValueError` message quotes the entire offending string, matching
/// CPython — that is why `1e+16` reports `'1e+16'` (bug 10).
///
/// `BigInt::from_str` is stricter than `int()` in three ways that no repr can
/// reach: `int()` also accepts surrounding whitespace, an underscore digit
/// separator (`int("1_0") == 10`), and any Unicode `Nd` codepoint. None of
/// those occur in a fragment of `str(float)` or `str(Decimal)`, and both agree
/// on a leading `+`/`-` and on rejecting `""` and `"e"`.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `int(ch)` for a *single* character, as the fraction loop calls it.
///
/// The message quotes the one offending character, so `1.5e+16` reports `'e'`
/// rather than `'5e+16'` (bug 10). `char::to_digit(10)` is ASCII-only where
/// CPython's `int()` accepts `'٥'`; unreachable, since no repr emits a
/// non-ASCII digit.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10)
        .map(|d| d as usize)
        .ok_or_else(|| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch)))
}

/// The shortest round-trip decimal digits of `a` (finite, non-negative), plus
/// CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// # Why `{:e}` alone is not enough
///
/// Rust's `{:e}` and CPython's `repr` are both "the shortest string that reads
/// back as the same double", and they agree on the digit *count* always and on
/// the digits themselves almost always. They part company on an **exact tie**,
/// where two equally-short decimals are equidistant from the true value and
/// both round-trip: CPython's `_Py_dg_dtoa` breaks the tie to **even**, Rust's
/// `flt2dec` shortest breaks it **away from zero**. The double whose exact
/// value is `670352580196876.25` sits precisely between `...876.2` and
/// `...876.3`, so `repr` says `.2` and `{:e}` says `.3`.
///
/// This is the same banker's-rounding trap `floatpath.rs` documents for
/// `round()`, relocated into the formatter — and it matters here for the same
/// reason: CKB's whole float path *is* the repr string, so a wrong last digit
/// is a wrong word.
///
/// # The fix
///
/// Rust's *fixed-precision* `{:.*e}` is correctly rounded half-to-even, so
/// re-emitting the same number of significant digits through it applies
/// CPython's tie rule. It is kept only if it still round-trips, so a
/// hypothetical asymmetric rounding interval (near a power of two, where the
/// nearer decimal can sit outside the interval) falls back to the shortest
/// digits rather than silently producing a string that reads back as a
/// different double.
fn shortest_repr_digits(a: f64) -> (String, i32) {
    let split = |s: &str| -> (String, i32) {
        let (mant, exp) = s.split_once('e').expect("{:e} always emits an 'e'");
        (
            mant.chars().filter(|c| *c != '.').collect(),
            // `{:e}` normalises to exactly one digit before the point, so the
            // decimal point sits one place left of where CPython counts it.
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

/// CPython's `str(float)` / `repr(float)`, which is where CKB's float path
/// begins and ends.
///
/// `PyOS_double_to_string(v, 'r', 0, Py_DTSF_ADD_DOT_0)` in `pystrtod.c`:
/// take the shortest round-trip digits, then
///
/// * switch to exponent form when `decpt <= -4 || decpt > 16` — hence
///   `str(1e15) == "1000000000000000.0"` but `str(1e16) == "1e+16"`, and
///   `str(0.0001) == "0.0001"` but `str(1e-05) == "1e-05"`. (CPython moved this
///   threshold down from 1e17 so that `repr(2e16+8)` stops claiming
///   `20000000000000010.0` for a value that is really `...008.0`.) Everything
///   on the far side of that boundary is bug 10's `ValueError`.
/// * format that exponent `%+.02d` — signed, at least two digits, so `1e+16`
///   and `1e-05` but `5e-324`.
/// * otherwise print positionally and append `.0` if nothing follows the point
///   (`Py_DTSF_ADD_DOT_0`), which is the whole reason `1.0` is `"یەک خاڵ سفر"`
///   and not `"یەک"`.
fn py_str_f64(v: f64) -> String {
    // Unreachable from the shim, which computes `precision` as
    // `abs(Decimal(str(value)).as_tuple().exponent)` and would raise on the
    // 'F'/'n' exponent of a non-finite Decimal long before Rust is called.
    // Handled anyway so this is a faithful `str()` rather than a
    // faithful-in-context one; CKB goes on to raise `ValueError` on
    // `int("inf")`, exactly as Python does.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    // The sign is taken from the sign *bit*, not from `v < 0.0`, so that
    // `str(-0.0)` is "-0.0" and CKB's `startswith("-")` fires: -0.0 renders
    // "نێگەتیڤ سفر خاڵ سفر".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            // No ADD_DOT_0 in exponent form: `str(1e16)` is "1e+16", not
            // "1.0e+16" — which is exactly why it has no "." for CKB to split
            // on, and so reports the whole token rather than 'e' (bug 10).
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
        // "0." + leading zeros + digits: 0.5 -> "0.5", 1e-4 -> "0.0001".
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndig {
        // Nothing after the point, so Py_DTSF_ADD_DOT_0 supplies ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndig) as usize))
    } else {
        format!(
            "{}{}.{}",
            sign,
            &digits[..decpt as usize],
            &digits[decpt as usize..]
        )
    }
}

/// CPython's `Decimal.__str__` (the spec's `to-scientific-string`), ported from
/// `_pydecimal.Decimal.__str__`.
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # positional
/// else:
///     dotplace = 1                   # scientific
/// ```
///
/// # Why this cannot just be `BigDecimal`'s `Display`
///
/// The two disagree in three places, each of which would change CKB's output or
/// its exception:
///
/// | value | `Decimal.__str__` | `BigDecimal` `Display` |
/// |---|---|---|
/// | `1E+2` | `1E+2` (CKB: `ValueError`) | `100` (CKB: "سەد") |
/// | `0.0` | `0.0` (CKB: "سفر خاڵ سفر") | `0` (CKB: "سفر") |
/// | `1E+16` | `1E+16` | `1e+16` — lowercase |
///
/// So the digits and exponent are read off `as_bigint_and_exponent()` and
/// reassembled by Python's rule instead. That pairing is exact:
/// `BigDecimal::from_str` keeps the written scale rather than normalising
/// (`"1.10"` stays coefficient 110 / scale 2, which is what makes the trailing
/// "سفر" appear), and `(coefficient, -scale)` is precisely Python's
/// `(_int, _exp)` — including for values Python itself cannot tell apart, since
/// `Decimal("1E-7")` and `Decimal("0.0000001")` *are* the same object and both
/// stringify "1E-7".
///
/// One known divergence: negative zero. See [`LangCkb::to_cardinal_float`].
fn py_str_decimal(value: &BigDecimal) -> String {
    // BigDecimal stores value = coefficient * 10^-scale, so Python's `_exp`
    // (which counts the other way) is the negated scale.
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

    // `['e', 'E'][context.capitals]`, and the default context has capitals=1,
    // so the exponent marker is an uppercase 'E' — unlike `repr(float)`'s.
    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

impl Lang for LangCkb {

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

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + "em"` with no verify_ordinal guard, so a
    /// float keeps its spelled-out ".0" tail and the suffix lands on the last
    /// word: `to_ordinal(5.0)` == "پێنج خاڵ سفرەم", `to_ordinal(-1.5)` ==
    /// "نێگەتیڤ یەک خاڵ پێنجەم". Exponent-form reprs raise the cardinal
    /// path's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.to_cardinal_float(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + ەم` — the repr
    /// verbatim, sign and trailing zeros included: `-0.0` → "-0.0ەم",
    /// `Decimal("5.00")` → "5.00ەم", `1e16` → "1e+16ەم".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, except
    /// that an Infinity parse is surfaced as the ValueError CKB's own
    /// `to_cardinal` raises one step later: `str(Decimal("Infinity"))` has no
    /// "." and `int("Infinity")` chokes on the literal. The dispatcher's
    /// default maps `ParsedNumber::Inf` to base's OverflowError, which CKB
    /// can never raise. NaN keeps the default routing (ValueError either way).
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
        "IQD"
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
        "خاڵ"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_CKB.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input (`n = str(number).strip()`) and branches on
    /// the text:
    ///   * `n.startswith("-")` → `(self.negword + self.to_cardinal(n[1:])).strip()`
    ///   * `"." in n` → the float path (out of scope; `str(int)` never has a dot)
    ///   * otherwise → `self._int_to_word(int(n))`
    ///
    /// The negative branch recurses on the *string* `n[1:]`, which for an
    /// integer is exactly `abs(value)` rendered in decimal — that recursive call
    /// can only fall through to `_int_to_word`, so it collapses to a direct
    /// call here. `int("-")` (`ValueError`) is unreachable from a `BigInt`,
    /// since `str` of an integer is never a bare `"-"`.
    ///
    /// The trailing `.strip()` is a no-op in practice — `_int_to_word` never
    /// returns an empty or padded string for a non-negative input — but is kept
    /// for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.int_to_word(value))
    }

    /// Port of `Num2Word_CKB.to_cardinal` for `float` / `Decimal` input.
    ///
    /// The same Python method as [`Lang::to_cardinal`] — CKB fuses both entry
    /// points into one `to_cardinal` and tells them apart by looking for a `"."`
    /// in `str(number)`. So this is not an override of
    /// `Num2Word_Base.to_cardinal_float`: **that method is never reached for
    /// CKB**, and neither is `base.float2tuple`. All the work is in
    /// [`LangCkb::cardinal_from_repr`]; this hook only has to rebuild the `n =
    /// str(number)` that the shim took apart.
    ///
    /// # Why the two arms cannot share a formatter
    ///
    /// `str` of a `Decimal` keeps every written digit and `str` of a `float`
    /// keeps the shortest round-trip ones, so `Decimal("1.10")` ends in
    /// `"سفر"` where the float `1.1` cannot — issue #603's split is visible in
    /// the *output* here, not just in the last bits. Hence [`py_str_decimal`]
    /// for one and [`py_str_f64`] for the other.
    ///
    /// # `precision_override` is deliberately ignored
    ///
    /// `num2words(2.675, lang="ckb", precision=1)` returns the full
    /// `"دوو خاڵ شەش حەوت پێنج"`, not a one-digit fraction. `__init__.py` pops
    /// `precision=`, and since `hasattr(converter, "precision")` is True
    /// (`Num2Word_Base.__init__` sets it) it does assign `converter.precision`
    /// and restore it afterwards — but `Num2Word_CKB.to_cardinal` never reads
    /// the attribute. Only `base.float2tuple` does, and CKB never calls it. So
    /// the kwarg is a no-op for this language rather than unimplemented, and
    /// dropping it is the faithful behaviour. Verified live at precision 1 and
    /// 6: both give the untruncated three-digit fraction.
    ///
    /// # Known divergence: negative zero on the `Decimal` arm
    ///
    /// `str(Decimal("-0.0"))` is `"-0.0"`, so Python renders
    /// `"نێگەتیڤ سفر خاڵ سفر"`. `BigDecimal` has no signed zero — the shim's
    /// `BigDecimal::from_str("-0.0")` yields coefficient `0`, whose sign is
    /// gone by the time this sees it — so the Rust output is `"سفر خاڵ سفر"`.
    /// Not fixable from this file: it is lost in the crossing, before
    /// `to_cardinal_float` is entered. The `float` arm is unaffected (it reads
    /// the IEEE sign bit, and `-0.0` survives as an f64). Flagged in `concerns`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        // n = str(number)
        let n = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        self.cardinal_from_repr(&n)
    }

    /// Port of `Num2Word_CKB.to_ordinal`: `self.to_cardinal(number) + ەم`.
    ///
    /// No joint, no stem change, no negative guard — the suffix simply lands on
    /// the last word (bug 4).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(self.to_cardinal(value)? + ORDINAL_SUFFIX)
    }

    /// Port of `Num2Word_CKB.to_ordinal_num`: `str(number) + ەم`.
    ///
    /// Overrides the base's bare `str(value)`. The sign survives verbatim:
    /// `to_ordinal_num(-1)` == `"-1ەم"`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(value.to_string() + ORDINAL_SUFFIX)
    }

    /// Port of `Num2Word_CKB.to_year`: `self.to_cardinal(val)`.
    ///
    /// The `longval` parameter is accepted and ignored by Python — there is no
    /// century-pair reading ("nineteen eighty-four"); years are read as plain
    /// cardinals. Corpus: 1984 -> "یەک هەزار و نۆ سەد و هەشتا و چوار".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`. Only `to_cheque` reads it: it is the sole
    /// CKB path that can raise `NotImplementedError`, since `to_currency`
    /// falls back to IQD instead of raising (bug 7).
    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `Num2Word_CKB.CURRENCY_FORMS[code]`, strict — **no IQD fallback here**.
    ///
    /// The fallback belongs to `to_currency` alone. `to_cheque` (inherited from
    /// `Num2Word_Base`) does a bare `self.CURRENCY_FORMS[currency]` and lets the
    /// `KeyError` become `NotImplementedError`, so this hook must miss for an
    /// unknown code. Corpus: `cheque:GBP` → NotImplementedError while
    /// `currency:GBP` → `"... دینار ..."`.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // `currency_adjective` and `currency_precision` stay at their trait
    // defaults: `Num2Word_CKB` defines neither `CURRENCY_ADJECTIVES` nor
    // `CURRENCY_PRECISION`, and the `Num2Word_Base` class dicts it inherits are
    // both still empty at runtime (verified live). Note the asymmetry with
    // `Num2Word_EN`, which *mutates* the shared `Num2Word_EUR.CURRENCY_FORMS`
    // in place — CKB owns its forms dict outright, so nothing leaks in.
    // `default_to_cheque` therefore sees divisor 100 for every code, which is
    // where the corpus's `56/100` comes from.

    /// Port of `Num2Word_CKB.pluralize`.
    ///
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Overrides the base's `raise NotImplementedError`. Currently unreachable:
    /// CKB's `to_currency` inlines the equivalent choice itself, and
    /// `default_to_cheque` unconditionally takes the plural form. Implemented
    /// anyway because `Num2Word_CKB` defines it, so the surface must match.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms.last() {
            None => Ok(String::new()),
            Some(plural) => Ok(if n.is_one() {
                forms[0].clone()
            } else {
                plural.clone()
            }),
        }
    }

    /// Port of `Num2Word_CKB.to_currency` — a full override of the base's.
    ///
    /// ```python
    /// def to_currency(self, val, currency="IQD", cents=True, separator=" ", adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// Nothing in it can raise, so this always returns `Ok`. It shares no code
    /// with `Num2Word_Base.to_currency`: no `parse_currency_parts`, no
    /// `pluralize`, no `_money_verbose`/`_cents_verbose`/`_cents_terse`, no
    /// `CURRENCY_PRECISION`, no `CURRENCY_ADJECTIVES`.
    ///
    /// `adjective` is accepted and **ignored** — the parameter exists in the
    /// signature but the body never reads it, so `adjective=True` is a no-op.
    /// (`CURRENCY_ADJECTIVES` is empty for CKB regardless, so the base class
    /// would have been a no-op here too.)
    ///
    /// # Bugs preserved
    ///
    /// 6. **`negword` is concatenated raw, not `.strip()`-ed then spaced.**
    ///    `Num2Word_Base.to_currency` builds `"%s " % self.negword.strip()`;
    ///    CKB writes `self.negword + result`. Both land on `"نێگەتیڤ "` here
    ///    only because `setup` gave `negword` a trailing space. So this uses
    ///    [`NEGWORD`] verbatim — *not* `negword().trim()` + `" "` the way
    ///    `currency::default_to_currency` does.
    /// 7. **An unknown currency code silently renders as Iraqi dinars.**
    ///    `.get(currency, list(self.CURRENCY_FORMS.values())[0])` falls back to
    ///    the first entry instead of raising. So `currency="GBP"` — and even
    ///    `currency="ZZZ"` — yields `"... دینار ... فلس"` rather than the
    ///    `NotImplementedError` every other mode raises. Corpus confirms it for
    ///    GBP, JPY, KWD, BHD, INR, CNY and CHF: all seven render as dînar/fils.
    /// 8. **`CURRENCY_PRECISION` is ignored; every code is 2-decimal.** The
    ///    hardcoded `[:2]` means KWD/BHD get 2 fractional digits rather than 3
    ///    (`12.34` → 34 fils, not 340) and JPY gets cents rather than none
    ///    (`12.34` → `"دوانزە دینار سی و چوار فلس"`, where the base class
    ///    would have rounded to a whole 12). Corpus confirms both.
    /// 9. **The unit form is indexed `cr1[1]`, not `cr1[-1]`.** Identical for
    ///    CKB's four 2-tuples, but it would `IndexError` on a 1-tuple form.
    ///    Mirrored as a direct index rather than `.last()`.
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
        let is_negative = val.is_negative();
        let (left, right) = self.currency_parts(val);

        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms); // bug 7
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
        let mut result = self.int_to_word(&left);
        result.push(' ');
        result.push_str(if left.is_one() { &cr1[0] } else { &cr1[1] }); // bug 9

        // if cents and right:
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // if is_negative: result = self.negword + result
        if is_negative {
            result = format!("{}{}", NEGWORD, result); // bug 6
        }

        // return result.strip() — a no-op in practice (nothing here can leave
        // leading or trailing whitespace), kept for fidelity.
        Ok(result.trim().to_string())
    }

    // `to_cheque` is NOT overridden: `Num2Word_CKB` inherits
    // `Num2Word_Base.to_cheque` verbatim (verified live —
    // `c.to_cheque.__func__.__qualname__ == "Num2Word_Base.to_cheque"`), which
    // `currency::default_to_cheque` already ports. It reaches back through
    // `currency_forms` (strict, so unknown codes raise), `currency_precision`
    // (100) and `money_verbose` → the default → CKB's `to_cardinal`.
    //
    // `cardinal_from_decimal` likewise stays at its default, and is dead code
    // for CKB. Its only caller is `currency::default_to_currency`'s
    // fractional-cents branch, and CKB overrides `to_currency` outright (its
    // `[:2]` truncates rather than carrying a fraction of a cent), so that
    // branch is unreachable. `default_to_cheque` does not call it either.
    //
    // This matters more than it looks: the default routes to
    // `floatpath::cardinal_from_bigdecimal`, which calls
    // `default_to_cardinal_float` *free-function-style* rather than through
    // `self.to_cardinal_float`, so it would bypass the override above and go
    // through `float2tuple` — a path CKB's Python never takes. It is left alone
    // because it is unreachable, not because it would agree.
    //
    // `pointword()` is likewise never read by CKB's own code: `POINTWORD` is
    // spliced in directly by `cardinal_from_repr`, mirroring Python's
    // `self.pointword`. The trait method is kept in sync for the base-class
    // paths that no longer reach it.
}
