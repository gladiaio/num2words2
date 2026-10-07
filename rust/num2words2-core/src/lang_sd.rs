//! Port of `lang_SD.py` (Sindhi).
//!
//! Shape: **self-contained**. `Num2Word_SD` subclasses `Num2Word_Base` but its
//! `setup()` defines none of `high_numwords`/`mid_numwords`/`low_numwords`, so
//! the `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires:
//! `self.cards` is never built and `self.MAXVAL` is never set. `to_cardinal` is
//! overridden outright and drives a hand-rolled `_int_to_word` recursion.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here, and
//! there is **no overflow check** — arbitrarily large input is accepted (see
//! bug 3 below for what it returns).
//!
//! All in-scope modes are overridden by the Python class itself. Python's one
//! `to_cardinal` serves both integer and float input (its `"." in n` branch);
//! the Rust trait splits those, so the integer half lands in `to_cardinal` and
//! the fractional half in `to_cardinal_float` (see that method for the float/
//! Decimal port and its two out-of-corpus divergences):
//!   * `to_cardinal(n)`    -> the `_int_to_word` recursion, with sign handling
//!   * `to_ordinal(n)`     -> `to_cardinal(n) + "-و"`
//!   * `to_ordinal_num(n)` -> `str(n) + "."`  (note: *not* the base default,
//!     which returns `str(value)` with no dot)
//!   * `to_year(val, longval=True)` -> `to_cardinal(val)`, ignoring `longval`
//!     entirely. Identical to the base default, but overridden explicitly here
//!     to mirror the Python source.
//!
//! `Num2Word_SD` additionally overrides `to_currency` outright, but **not**
//! `to_cheque`, `pluralize`, `_money_verbose`, `_cents_verbose` or
//! `_cents_terse`; see "Currency" below.
//!
//! Nothing in this module reads or writes instance state across calls. The
//! struct below holds only the `CURRENCY_FORMS` table, built once in
//! [`LangSd::new`] (which the generated registry caches in a `OnceLock`) and
//! thereafter read-only, so the Python dispatcher's `_plain_int` handshake
//! caveat still does not apply.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following are all wrong-looking but are
//! exactly what Python emits, verified against the interpreter and against the
//! frozen corpus:
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** Python's `_int_to_word` opens
//!    with `return self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is
//!    the empty string (a falsy placeholder so that `ones[n]` indexes by
//!    digit), so Python always answered with the English "zero". This port
//!    says the Sindhi ٻڙي instead: `to_cardinal(0)` == "ٻڙي". See [`ZERO`].
//! 2. **No teens (fixed, gladiaio/num2words2#247).** Python's `< 100` branch
//!    composed `tens[n // 10] + " " + ones[n % 10]`, so 11 was "ڏهه هڪ"
//!    (ten one) and 23 "ويهه ٽي". The port reads [`BELOW_HUNDRED`]: 11 is
//!    "يارهن", 23 "ٽريويهه".
//! 3. **Everything >= 10^9 rendered as bare ASCII digits (fixed,
//!    gladiaio/num2words2#147).** The `_int_to_word` chain ended in `else:
//!    return str(number)`, so 10^9 was `"1000000000"`. The port adds ارب
//!    (10^9) and کرب (10^11), recursing on the کرب quotient, and raises
//!    `OverflowError` from 10^22 ([`maxval_ceiling`]).
//! 4. **`million` was "لک" (lakh) applied at 10^6 (fixed,
//!    gladiaio/num2words2#147).** A lakh is 10^5, so 10^6 came out as
//!    "هڪ لک" and 123456789 as "هڪ سو ويهه ٽي لک ...". The port groups by
//!    هزار, لک (10^5) and ڪروڙ (10^7): 10^6 is "ڏهه لک", 123456789
//!    "ٻارهن ڪروڙ چوٽيهه لک ڇاونجاهه هزار ست سو اوڻانوي".
//! 5. **`negword` and `pointword`** are the English "minus " and "point" in
//!    Python; this port uses منفي and اعشاريه (#154): `to_cardinal(-1)` ==
//!    "منفي هڪ", `to_cardinal(1.5)` == "هڪ اعشاريه پنج".
//!
//!    UNVERIFIED (#154): اعشاريه — best candidate, not confirmed by a Sindhi
//!    source for reading a decimal point aloud. Basis: the Sindhi Language
//!    Authority dictionary (dic.sindhila.edu.pk) attests the adjective
//!    اعشاري "decimal" (اعشاري نظام, decimal system) from the same root;
//!    Urdu, the other language of Pakistani schooling, reads 1.5 as
//!    "ایک اعشاریہ پانچ"; indifferentlanguages.com gives "اعشاريه پوائنٽ" for
//!    "decimal point". Written in Sindhi letters (ي U+064A, ه U+0647). The
//!    alternative ڏهائي is attested only as "tenth part / decimal system".
//!    Needs a native speaker.
//! 6. **Two kafs, on purpose.** `ones[1]` ("هڪ") spells its kaf with U+06AA
//!    ARABIC LETTER SWASH KAF, while لک uses U+06A9 KEHEH. That is Sindhi
//!    orthography (ڪ is /k/, ک is /kʰ/), not an inconsistency. The two
//!    render near-identically, so the tables use explicit `\u{...}` escapes:
//!    an editor that normalises Arabic presentation forms would silently
//!    corrupt them.
//!
//! # Currency
//!
//! `to_currency` is overridden by the Python class and shares essentially
//! nothing with `Num2Word_Base.to_currency`: it never calls
//! `parse_currency_parts`, `pluralize`, `_cents_verbose` or `_cents_terse`, and
//! never consults `CURRENCY_PRECISION` or `CURRENCY_ADJECTIVES`. It splits
//! `str(val)` on `"."` and works on the two halves textually. Consequences,
//! each verified against the live interpreter and the frozen corpus:
//!
//! 1. **An unknown currency code does not raise.** `to_currency` looks the code
//!    up with `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`,
//!    so anything outside {PKR, USD, EUR} silently renders in Pakistani rupees —
//!    the first entry by insertion order. The corpus tests seven such codes
//!    (GBP, JPY, KWD, BHD, INR, CNY, CHF) and every row comes back in rupees and
//!    paisas. `to_cheque`, which SD does *not* override, keeps the base's plain
//!    `CURRENCY_FORMS[currency]` subscript and so *does* raise for those same
//!    seven codes. The two entry points disagree, deliberately.
//! 2. **`CURRENCY_PRECISION` is ignored.** The subunit is always hundredths,
//!    hardcoded by the `parts[1][:2]` slice. There is no 3-decimal (KWD/BHD,
//!    divisor 1000) or 0-decimal (JPY, divisor 1) handling, and the base's
//!    zero-decimal rounding branch is unreachable. Since SD inherits
//!    `CURRENCY_PRECISION = {}` from `Num2Word_Base` those codes would resolve
//!    to 100 regardless, so this is invisible in the corpus — `currency:JPY` and
//!    `currency:KWD` render identically to `currency:PKR`, cents and all.
//! 3. **No rounding, only truncation.** `12.999` bills 99 cents and `2.675`
//!    bills 67 — the base path's `ROUND_HALF_UP` would give 100 and 68.
//! 4. **`adjective` is accepted and ignored**, so `CURRENCY_ADJECTIVES` (empty
//!    anyway) is never read.
//! 5. **`cents=False` suppresses the cents segment entirely** rather than
//!    switching to terse digits as the base class does: the guard is
//!    `if cents and right:`, and a false `cents` drops the whole clause.
//!    `to_currency(12.34, cents=False)` == `"ڏهه ٻه euros"`.
//!
//! `to_cheque` is inherited unmodified and needs no override here: the base
//! implementation's hooks (`currency_forms`, `currency_precision` at its default
//! 100, `money_verbose` delegating to `to_cardinal`) already resolve to SD's
//! behaviour.
//!
//! # Known divergences
//!
//! Two, both outside the corpus, both stemming from information the
//! `CurrencyValue` boundary cannot carry:
//!
//! 1. **An explicit `separator=","` renders as `" "`.** The dispatcher
//!    substitutes `Num2Word_Base`'s `","` when the caller supplies no separator,
//!    so `","` has to be read as "unset" and replaced with SD's own `" "`
//!    default — which makes a deliberate `","` unrepresentable. Every other
//!    separator is honoured verbatim. The corpus evidence for resolving it this
//!    way, and the reason it cannot be fixed from inside this file, is on
//!    [`SEPARATOR_UNSET`].
//! 2. **Scientific-notation input raises `ValueError` in Python, not here.**
//!    `str(float)` switches to exponent form at `1e16` and below `1e-4`, and
//!    `int("1e+21")` then throws: `to_currency(1e21)` and `to_currency(1e-5)`
//!    are both `ValueError`. The shim hands the core `str(value)`, but
//!    `CurrencyValue::parse` turns it into a `BigDecimal`, which accepts
//!    exponent notation and discards the fact that it was used —
//!    `"1e-05"` and `"0.00001"` become indistinguishable. Reproducing the raise
//!    would need the original string. This path renders the value instead (via
//!    bug 3's digit fallback for the large case). No corpus row reaches it.
//!
//! # Error variants
//!
//! `to_cheque` raises `N2WError::NotImplemented` for a code absent from
//! `CURRENCY_FORMS`, via the inherited `currency::default_to_cheque` and the
//! [`Lang::lang_name`] override below — Python's
//! `NotImplementedError: Currency code "GBP" not implemented for "Num2Word_SD"`,
//! reached through the `KeyError` that `to_cheque`'s bare subscript throws.
//!
//! Otherwise none. The four integer modes and `to_currency` have no reachable
//! crash site and no deliberate `raise`: every list index is derived from a digit
//! of a value already bounded by the enclosing branch, so `ones[i]`/`tens[i]` can
//! only be hit with `i` in `0..=9`. With no `MAXVAL` there is no `OverflowError`
//! either (bug 3 swallows the large-input case). `pluralize` is left at the
//! trait default that raises, matching `Num2Word_Base.pluralize` — SD reaches it
//! from neither `to_currency` (which inlines its own two-form selection) nor
//! `to_cheque` (which takes `cr1[-1]` directly).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use ڊالر / يورو with سينٽ. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `_int_to_word`'s zero case. Python evaluates
/// `self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is `""` (falsy),
/// so Python returns the English literal; this port says ٻڙي. See bug 1.
const ZERO: &str = "\u{067B}\u{0699}\u{064A}"; // ٻڙي

/// `self.negword`. English "minus " in Python; منفي here. See bug 5.
const NEGWORD: &str = "\u{0645}\u{0646}\u{0641}\u{064A} "; // منفي

/// `self.ones`. Index 0 is the empty placeholder that makes bug 1 possible;
/// it is never emitted, because every call site either guards on a nonzero
/// remainder or is the `_int_to_word` zero branch that returns [`ZERO`].
const ONES: [&str; 10] = [
    "",                                     // (empty placeholder — see bug 1)
    "\u{0647}\u{06AA}",                     // هڪ    (hik)    — note U+06AA, cf. bug 6
    "\u{067B}\u{0647}",                     // ٻه    (ba)
    "\u{067D}\u{064A}",                     // ٽي    (ṭe)
    "\u{0686}\u{0627}\u{0631}",             // چار   (chār)
    "\u{067E}\u{0646}\u{062C}",             // پنج   (panj)
    "\u{0687}\u{0647}\u{0647}",             // ڇهه   (chhahe)
    "\u{0633}\u{062A}",                     // ست    (sat)
    "\u{0627}\u{067A}",                     // اٺ    (aṭh)
    "\u{0646}\u{0648}",                     // نو    (nav)
];

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247; Sindhi
/// was not in that issue's list but had the same bug). Python had `ones` and
/// `tens` and no teens, so 11 was "ڏهه هڪ" (ten one) and 23 "ويهه ٽي".
/// Sindhi has its own word for every number below a hundred. Each entry is
/// the headword the Sindhi Language Authority dictionary
/// (dic.sindhila.edu.pk) gives for that value; paheliyan.pk's 1-100 list
/// agrees except for 23/53/63/73/83, where it writes ٽي- for the
/// dictionary's ٽري-. Escaped like the other tables (bug 6).
const BELOW_HUNDRED: [&str; 100] = [
    "", // 0 (never read)
    "\u{0647}\u{06AA}", // 1 هڪ
    "\u{067B}\u{0647}", // 2 ٻه
    "\u{067D}\u{064A}", // 3 ٽي
    "\u{0686}\u{0627}\u{0631}", // 4 چار
    "\u{067E}\u{0646}\u{062C}", // 5 پنج
    "\u{0687}\u{0647}\u{0647}", // 6 ڇهه
    "\u{0633}\u{062A}", // 7 ست
    "\u{0627}\u{067A}", // 8 اٺ
    "\u{0646}\u{0648}", // 9 نو
    "\u{068F}\u{0647}\u{0647}", // 10 ڏهه
    "\u{064A}\u{0627}\u{0631}\u{0647}\u{0646}", // 11 يارهن
    "\u{067B}\u{0627}\u{0631}\u{0647}\u{0646}", // 12 ٻارهن
    "\u{062A}\u{064A}\u{0631}\u{0647}\u{0646}", // 13 تيرهن
    "\u{0686}\u{0648}\u{068F}\u{0647}\u{0646}", // 14 چوڏهن
    "\u{067E}\u{0646}\u{062F}\u{0631}\u{0647}\u{0646}", // 15 پندرهن
    "\u{0633}\u{0648}\u{0631}\u{0647}\u{0646}", // 16 سورهن
    "\u{0633}\u{062A}\u{0631}\u{0647}\u{0646}", // 17 سترهن
    "\u{0627}\u{0631}\u{0699}\u{0647}\u{0646}", // 18 ارڙهن
    "\u{0627}\u{0648}\u{06BB}\u{064A}\u{0647}\u{0647}", // 19 اوڻيهه
    "\u{0648}\u{064A}\u{0647}\u{0647}", // 20 ويهه
    "\u{0627}\u{064A}\u{06AA}\u{064A}\u{0647}\u{0647}", // 21 ايڪيهه
    "\u{067B}\u{0627}\u{0648}\u{064A}\u{0647}\u{0647}", // 22 ٻاويهه
    "\u{067D}\u{0631}\u{064A}\u{0648}\u{064A}\u{0647}\u{0647}", // 23 ٽريويهه
    "\u{0686}\u{0648}\u{0648}\u{064A}\u{0647}\u{0647}", // 24 چوويهه
    "\u{067E}\u{0646}\u{062C}\u{0648}\u{064A}\u{0647}\u{0647}", // 25 پنجويهه
    "\u{0687}\u{0648}\u{064A}\u{0647}\u{0647}", // 26 ڇويهه
    "\u{0633}\u{062A}\u{0627}\u{0648}\u{064A}\u{0647}\u{0647}", // 27 ستاويهه
    "\u{0627}\u{067A}\u{0627}\u{0648}\u{064A}\u{0647}\u{0647}", // 28 اٺاويهه
    "\u{0627}\u{0648}\u{06BB}\u{067D}\u{064A}\u{0647}\u{0647}", // 29 اوڻٽيهه
    "\u{067D}\u{064A}\u{0647}\u{0647}", // 30 ٽيهه
    "\u{0627}\u{064A}\u{06AA}\u{067D}\u{064A}\u{0647}\u{0647}", // 31 ايڪٽيهه
    "\u{067B}\u{067D}\u{064A}\u{0647}\u{0647}", // 32 ٻٽيهه
    "\u{067D}\u{064A}\u{067D}\u{064A}\u{0647}\u{0647}", // 33 ٽيٽيهه
    "\u{0686}\u{0648}\u{067D}\u{064A}\u{0647}\u{0647}", // 34 چوٽيهه
    "\u{067E}\u{0646}\u{062C}\u{067D}\u{064A}\u{0647}\u{0647}", // 35 پنجٽيهه
    "\u{0687}\u{067D}\u{064A}\u{0647}\u{0647}", // 36 ڇٽيهه
    "\u{0633}\u{062A}\u{067D}\u{064A}\u{0647}\u{0647}", // 37 ستٽيهه
    "\u{0627}\u{067A}\u{067D}\u{064A}\u{0647}\u{0647}", // 38 اٺٽيهه
    "\u{0627}\u{0648}\u{06BB}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 39 اوڻيتاليهه
    "\u{0686}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 40 چاليهه
    "\u{0627}\u{064A}\u{06AA}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 41 ايڪيتاليهه
    "\u{067B}\u{0627}\u{0626}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 42 ٻائيتاليهه
    "\u{067D}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 43 ٽيتاليهه
    "\u{0686}\u{0648}\u{0626}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 44 چوئيتاليهه
    "\u{067E}\u{0646}\u{062C}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 45 پنجيتاليهه
    "\u{0687}\u{0627}\u{0626}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 46 ڇائيتاليهه
    "\u{0633}\u{062A}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 47 ستيتاليهه
    "\u{0627}\u{067A}\u{064A}\u{062A}\u{0627}\u{0644}\u{064A}\u{0647}\u{0647}", // 48 اٺيتاليهه
    "\u{0627}\u{0648}\u{06BB}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 49 اوڻونجاهه
    "\u{067E}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 50 پنجاهه
    "\u{0627}\u{064A}\u{06AA}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 51 ايڪونجاهه
    "\u{067B}\u{0627}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 52 ٻاونجاهه
    "\u{067D}\u{0631}\u{064A}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 53 ٽريونجاهه
    "\u{0686}\u{0648}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 54 چوونجاهه
    "\u{067E}\u{0646}\u{062C}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 55 پنجونجاهه
    "\u{0687}\u{0627}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 56 ڇاونجاهه
    "\u{0633}\u{062A}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 57 ستونجاهه
    "\u{0627}\u{067A}\u{0648}\u{0646}\u{062C}\u{0627}\u{0647}\u{0647}", // 58 اٺونجاهه
    "\u{0627}\u{0648}\u{06BB}\u{0647}\u{067A}", // 59 اوڻهٺ
    "\u{0633}\u{067A}", // 60 سٺ
    "\u{0627}\u{064A}\u{06AA}\u{0647}\u{067A}", // 61 ايڪهٺ
    "\u{067B}\u{0627}\u{0647}\u{067A}", // 62 ٻاهٺ
    "\u{067D}\u{0631}\u{064A}\u{0647}\u{067A}", // 63 ٽريهٺ
    "\u{0686}\u{0648}\u{0647}\u{067A}", // 64 چوهٺ
    "\u{067E}\u{0646}\u{062C}\u{0647}\u{067A}", // 65 پنجهٺ
    "\u{0687}\u{0627}\u{0647}\u{067A}", // 66 ڇاهٺ
    "\u{0633}\u{062A}\u{0647}\u{067A}", // 67 ستهٺ
    "\u{0627}\u{067A}\u{0647}\u{067A}", // 68 اٺهٺ
    "\u{0627}\u{0648}\u{06BB}\u{0647}\u{062A}\u{0631}", // 69 اوڻهتر
    "\u{0633}\u{062A}\u{0631}", // 70 ستر
    "\u{0627}\u{064A}\u{06AA}\u{0647}\u{062A}\u{0631}", // 71 ايڪهتر
    "\u{067B}\u{0627}\u{0647}\u{062A}\u{0631}", // 72 ٻاهتر
    "\u{067D}\u{0631}\u{064A}\u{0647}\u{062A}\u{0631}", // 73 ٽريهتر
    "\u{0686}\u{0648}\u{0647}\u{062A}\u{0631}", // 74 چوهتر
    "\u{067E}\u{0646}\u{062C}\u{0647}\u{062A}\u{0631}", // 75 پنجهتر
    "\u{0687}\u{0627}\u{0647}\u{062A}\u{0631}", // 76 ڇاهتر
    "\u{0633}\u{062A}\u{0647}\u{062A}\u{0631}", // 77 ستهتر
    "\u{0627}\u{067A}\u{0647}\u{062A}\u{0631}", // 78 اٺهتر
    "\u{0627}\u{0648}\u{06BB}\u{0627}\u{0633}\u{064A}", // 79 اوڻاسي
    "\u{0627}\u{0633}\u{064A}", // 80 اسي
    "\u{0627}\u{064A}\u{06AA}\u{0627}\u{0633}\u{064A}", // 81 ايڪاسي
    "\u{067B}\u{064A}\u{0627}\u{0633}\u{064A}", // 82 ٻياسي
    "\u{067D}\u{0631}\u{064A}\u{0627}\u{0633}\u{064A}", // 83 ٽرياسي
    "\u{0686}\u{0648}\u{0631}\u{0627}\u{0633}\u{064A}", // 84 چوراسي
    "\u{067E}\u{0646}\u{062C}\u{0627}\u{0633}\u{064A}", // 85 پنجاسي
    "\u{0687}\u{0647}\u{0627}\u{0633}\u{064A}", // 86 ڇهاسي
    "\u{0633}\u{062A}\u{0627}\u{0633}\u{064A}", // 87 ستاسي
    "\u{0627}\u{067A}\u{0627}\u{0633}\u{064A}", // 88 اٺاسي
    "\u{0627}\u{0648}\u{06BB}\u{0627}\u{0646}\u{0648}\u{064A}", // 89 اوڻانوي
    "\u{0646}\u{0648}\u{064A}", // 90 نوي
    "\u{0627}\u{064A}\u{06AA}\u{0627}\u{0646}\u{0648}\u{064A}", // 91 ايڪانوي
    "\u{067B}\u{064A}\u{0627}\u{0646}\u{0648}\u{064A}", // 92 ٻيانوي
    "\u{067D}\u{064A}\u{0627}\u{0646}\u{0648}\u{064A}", // 93 ٽيانوي
    "\u{0686}\u{0648}\u{0631}\u{0627}\u{0646}\u{0648}\u{064A}", // 94 چورانوي
    "\u{067E}\u{0646}\u{062C}\u{0627}\u{0646}\u{0648}\u{064A}", // 95 پنجانوي
    "\u{0687}\u{0647}\u{0627}\u{0646}\u{0648}\u{064A}", // 96 ڇهانوي
    "\u{0633}\u{062A}\u{0627}\u{0646}\u{0648}\u{064A}", // 97 ستانوي
    "\u{0627}\u{067A}\u{0627}\u{0646}\u{0648}\u{064A}", // 98 اٺانوي
    "\u{0646}\u{0648}\u{0627}\u{0646}\u{0648}\u{064A}", // 99 نوانوي
];

/// `self.hundred` — سو (so).
const HUNDRED: &str = "\u{0633}\u{0648}";
/// `self.thousand` — هزار (hazār).
const THOUSAND: &str = "\u{0647}\u{0632}\u{0627}\u{0631}";
/// The Indian scale (bug 4, fixed in #147): لک 10^5, ڪروڙ 10^7, ارب 10^9 and
/// کرب 10^11, each defined by value in the Sindhi Language Authority
/// dictionary (لک "هڪ سؤ هزار", ڪروڙ "هڪ سؤ لک", ارب "سؤ ڪروڙ", کرب
/// "سؤ ارب"). Python applied لک at 10^6 and stopped at 10^9. Nothing above
/// کرب is attested, so its quotient recurses up to the 10^22 ceiling.
/// Note U+06A9 KEHEH in لک and کرب vs U+06AA SWASH KAF in ڪروڙ and هڪ: Sindhi
/// spells /kʰ/ and /k/ with different letters.
const LAKH: &str = "\u{0644}\u{06A9}"; // لک
const CRORE: &str = "\u{06AA}\u{0631}\u{0648}\u{0699}"; // ڪروڙ
const ARAB: &str = "\u{0627}\u{0631}\u{0628}"; // ارب
const KHARAB: &str = "\u{06A9}\u{0631}\u{0628}"; // کرب

/// The suffix `to_ordinal` appends: `"-و"` (hyphen + U+0648 ARABIC LETTER WAW).
const ORDINAL_SUFFIX: &str = "-\u{0648}";

/// `self.pointword` — the word between the integer and fractional parts on the
/// float path. Python said the English "point". UNVERIFIED (#154): see the
/// module header for the basis of اعشاريه.
const POINTWORD: &str = "\u{0627}\u{0639}\u{0634}\u{0627}\u{0631}\u{064A}\u{0647}"; // اعشاريه

/// Where the `u64` ladder hands over to [`ARAB`]/[`KHARAB`].
const FALLBACK_THRESHOLD: u64 = 1_000_000_000;

// ---- currency ----------------------------------------------------------
//
// `Num2Word_SD.CURRENCY_FORMS` is a class attribute the class declares itself,
// so — unlike the EUR-family languages — it is *not* the dict `Num2Word_EN`
// mutates at import time. Verified against the live interpreter: SD's table
// holds exactly PKR/USD/EUR and nothing English ever added. `CURRENCY_ADJECTIVES`
// and `CURRENCY_PRECISION` are both inherited unset (`{}`) from `Num2Word_Base`,
// so precision is 100 for every code (the trait default) and the adjective hook
// is never consulted.

/// PKR unit forms — روپي (rupī, sg) / روپيا (rupiyā, pl). Escaped rather than
/// pasted, for the same reason as the numeral tables: see bug 6.
const RUPEE_SG: &str = "\u{0631}\u{0648}\u{067E}\u{064A}";
const RUPEE_PL: &str = "\u{0631}\u{0648}\u{067E}\u{064A}\u{0627}";
/// PKR subunit forms — پئسو (paiso, sg) / پئسا (paisā, pl).
const PAISA_SG: &str = "\u{067E}\u{0626}\u{0633}\u{0648}";
const PAISA_PL: &str = "\u{067E}\u{0626}\u{0633}\u{0627}";

/// `Num2Word_SD.to_currency`'s own default: `separator=" "`, a bare space — not
/// the `","` that `Num2Word_Base.to_currency` defaults to.
const SD_SEPARATOR: &str = " ";

/// The separator the binding hands us when the Python caller supplied none.
///
/// `Num2Word_SD.to_currency` declares `separator=" "`, but the `Lang` trait takes
/// `separator` as a plain required argument, so "caller omitted it" has to be
/// encoded in the value itself. Both callers substitute **`Num2Word_Base`'s**
/// default rather than SD's, because neither can see a per-language signature:
/// `num2words2/__init__.py` forwards `kwargs.get("separator", ",")`, and
/// `bench/diff_test.py` hardcodes `","`. By the time the value crosses the
/// boundary, "omitted" and "explicitly `,`" are indistinguishable.
///
/// The corpus proves `","` must be read as "omitted" here. Rows are generated by
/// `num2words(v, lang=l, to="currency", currency=c)` with no `separator=`, yet
/// `de` — whose Python default is `separator=" und"` — renders
/// `"zwölf Euro und vierunddreißig Cent"` with no comma, while `en`, which keeps
/// the base `","`, renders `"twelve euros, thirty-four cents"`. Both are diffed
/// through a core call passing `","`. So a language that overrides `to_currency`
/// with its own default must restore that default when it sees the sentinel.
///
/// The one unreproducible case is a caller explicitly passing `separator=","`,
/// which gets `" "` where Python would give `","`. That is unresolvable without
/// an `Option<&str>` on the trait; see the module-level "Known divergences".
const SEPARATOR_UNSET: &str = ",";

/// Python's `int(parts[1][:2].ljust(2, "0"))` divisor: SD slices exactly two
/// fractional digits out of `str(val)`, hardcoding hundredths regardless of
/// `CURRENCY_PRECISION`. See divergence 2.
const CENTS_DIVISOR: i64 = 100;

pub struct LangSd {
    /// `self.CURRENCY_FORMS`. Built once in [`LangSd::new`] and cached by the
    /// generated registry's `OnceLock`, never per call.
    forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the entry `to_currency` falls
    /// back to for an unknown code. Python evaluates that on the dict's
    /// *insertion* order, which a `HashMap` does not preserve, so the first
    /// entry (PKR) is resolved once here and stored outright rather than
    /// recovered from `forms` at lookup time.
    fallback_forms: CurrencyForms,
}

impl LangSd {
    pub fn new() -> Self {
        // Insertion order in lang_SD.py is PKR, USD, EUR — PKR first, which is
        // what `list(...)[0]` picks up as the unknown-code fallback.
        let pkr = CurrencyForms::new(&[RUPEE_SG, RUPEE_PL], &[PAISA_SG, PAISA_PL]);
        let mut forms = HashMap::new();
        forms.insert("PKR", pkr.clone());
        forms.insert(
            "USD",
            CurrencyForms::new(&["ڊالر", "ڊالر"], &["سينٽ", "سينٽ"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["يورو", "يورو"], &["سينٽ", "سينٽ"]),
        );
        LangSd {
            forms,
            fallback_forms: pkr,
        }
    }
}

impl Default for LangSd {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#147, #203): the largest scale
/// word is کرب (10^11), so from 10^22 its multiplier would itself need کرب.
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(22))
}

/// `int_to_word` behind [`maxval_ceiling`]. Every entry point that hands
/// over a caller-supplied integer goes through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

/// Python's `_int_to_word`.
///
/// The `number < 0` arm is faithfully reproduced even though it is **dead code
/// on every in-scope path**: `to_cardinal` strips the sign from the string form
/// before calling in, and the recursive call sites all pass a positive
/// remainder. It would only matter to a caller reaching `_int_to_word` directly.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO.to_string();
    }
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }
    // Every branch below 10^9 is bounded, so a u64 is provably wide enough
    // once we know the value is under the threshold. Anything else — including
    // BigInts too large for u64 at all — takes the digit fallback (bug 3).
    if let Some(n) = number.to_u64().filter(|&n| n < FALLBACK_THRESHOLD) {
        return bounded_to_word(n);
    }
    // Python returned `str(number)` here (bug 3, fixed in #147).
    let kharab = BigInt::from(100_000_000_000u64);
    let (divisor, word) = if *number < kharab {
        (BigInt::from(FALLBACK_THRESHOLD), ARAB)
    } else {
        (kharab, KHARAB)
    };
    let (q, r) = (number / &divisor, number % &divisor);
    let mut result = format!("{} {}", int_to_word(&q), word);
    if !r.is_zero() {
        result.push(' ');
        result.push_str(&int_to_word(&r));
    }
    result
}

/// The `1 <= number < 10^9` portion of `_int_to_word`.
///
/// Invariant: never called with 0. Python's zero case is handled one level up
/// in [`int_to_word`], and each recursion here is guarded by Python's
/// `if remainder:` / a nonzero quotient. Were it called with 0 it would return
/// `ONES[0]` (`""`) rather than [`ZERO`] — the guard is what keeps bug 1's
/// zero word confined to the top-level entry point.
fn bounded_to_word(number: u64) -> String {
    if number < 10 {
        return ONES[number as usize].to_string();
    }

    if number < 100 {
        // No teens and no compounds in Python (bug 2, fixed in #247).
        return BELOW_HUNDRED[number as usize].to_string();
    }

    if number < 1_000 {
        // Note: hundreds use `ones[h]` directly, not a recursive call.
        let hundreds_val = (number / 100) as usize;
        let remainder = number % 100;
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&bounded_to_word(remainder));
        }
        return result;
    }

    // Python: thousands below 10^6, then لک "millions" (bug 4). Now
    // thousand, lakh and crore.
    let (divisor, word) = if number < 100_000 {
        (1_000, THOUSAND)
    } else if number < 10_000_000 {
        (100_000, LAKH)
    } else {
        (10_000_000, CRORE)
    };
    let mut result = format!("{} {}", bounded_to_word(number / divisor), word);
    if number % divisor != 0 {
        result.push(' ');
        result.push_str(&bounded_to_word(number % divisor));
    }
    result
}

impl Lang for LangSd {
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
    /// `to_cardinal(number) + "-و"` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "پنج اعشاريه ٻڙي-و".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}-و", cardinal))
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
        "PKR"
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

    /// `self.pointword`. Consulted on the `"." in n` branch of Python's
    /// `to_cardinal` — the float/Decimal path now served by
    /// [`LangSd::to_cardinal_float`] below.
    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number).strip()`, peels a leading `"-"` into
    /// `ret = self.negword`, then re-parses with `int(n)`. For integer input
    /// that round trip is exactly "take the absolute value and remember the
    /// sign", which is what this does. The `"." in n` fractional branch is
    /// unreachable here (scope is integers) and is omitted.
    ///
    /// The trailing `.strip()` is preserved, though it is a no-op in practice:
    /// `_int_to_word` never returns a value with outer whitespace, and it never
    /// returns `""` (zero yields [`ZERO`]), so there is no case where the
    /// negword prefix is left dangling with a space to trim.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (prefix, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", prefix, checked_int_to_word(&magnitude)?)
            .trim()
            .to_string())
    }

    /// Python's `to_ordinal`: the cardinal with `"-و"` glued on. There is no
    /// per-value inflection — every ordinal, including "ٻڙي-و" and the
    /// digits-fallback "1000000000-و", takes the same suffix.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`. Overrides the base
    /// default (which omits the dot). Negatives keep their sign: `-1` -> `"-1."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year`: delegates straight to `to_cardinal`, discarding the
    /// `longval` flag. No era suffix, no two-digit pairing — 1999 is read as
    /// the plain cardinal "one thousand nine hundred ninety nine", and negative
    /// years just get the negword prefix rather than a BC marker.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Python's `Num2Word_SD.to_cardinal` for **non-integer** input — the
    /// `"." in n` branch. The Rust trait splits the integer and float entry
    /// points that Python's single `to_cardinal` serves off one method, so this
    /// reproduces only the fractional branch:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword          # "minus "
    /// else:
    ///     ret = ""
    /// left, right = n.split(".", 1)
    /// ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    /// for digit in right:
    ///     ret += self._int_to_word(int(digit)) + " "
    /// return ret.strip()
    /// ```
    ///
    /// # Recovering `left`/`right` without `str(number)`
    ///
    /// SD reads the digits straight out of `str(number)`: `left` is the integer
    /// part, `right` the fractional characters, one Sindhi word per digit. The
    /// f64 crosses the FFI boundary as a raw double, so the original repr string
    /// is gone — but `floatpath::float2tuple` reconstructs exactly the same
    /// `(pre, post)` the corpus is built on, and its zero-padded `post` is byte
    /// for byte the `right` string SD iterates, load-bearing f64 artefacts and
    /// all: `2.675` -> `674.9999999999998` rescued to `675`, and the leading
    /// zeros of `0.01` -> `"01"`. `int(left)` is `pre.abs()` — SD peels the sign
    /// off the *string* first, so `left` never carries a minus, and the
    /// negword prefix is added separately (bug 5, prepended to
    /// RTL text). The same `float2tuple` serves the Decimal arm, so `cardinal_dec`
    /// rows come out right too: `1.10` keeps its trailing zero (`post` = 10 padded
    /// to `"10"`), and `98746251323029.99` overflows `int_to_word` into bug 3's
    /// bare-digit fallback for the integer part.
    ///
    /// # `precision=` is ignored, exactly as Python ignores it
    ///
    /// SD overrides `to_cardinal`, which takes no `precision` argument, so
    /// `precision=1` still leaves `2.675` at three fractional words. Verified in
    /// the live interpreter. `_precision_override` is therefore discarded; the
    /// natural repr-derived `value.precision()` always wins.
    ///
    /// # `ValueError` on inputs `int()` cannot parse
    ///
    /// SD reads `str(number)`; when that carries exponent notation the `int()`
    /// calls throw `ValueError`. `str(1e16) == "1e+16"` has no `"."`, so the
    /// *integer* branch runs `int("1e+16")`; `str(1.5e-5) == "1.5e-05"` keeps a
    /// `"."` but then `int("5e-05")` throws in the digit loop. Non-finite floats
    /// (`inf`/`nan`, `str` `"inf"`/`"nan"`) throw the same way. The message is
    /// never observed — only the exception *type* — so a plain `N2WError::Value`
    /// suffices. The predicates below reproduce CPython's `repr`/`Decimal.__str__`
    /// exponent thresholds exactly (fuzzed 500k floats / 200k Decimals, zero
    /// divergence): a float switches to `e`-notation iff `x != 0 and
    /// (|x| >= 1e16 or |x| < 1e-4)`; a Decimal iff its exponent is positive or
    /// its adjusted exponent `< -6`. No corpus row reaches either — SD's corpus
    /// floats sit in `[0.01, ~1e14]` — so this only tightens fidelity for inputs
    /// the harness never generates.
    ///
    /// # The negative-zero hole (Decimal only)
    ///
    /// `str(Decimal("-0.0")) == "-0.0"` keeps the sign, so Python answers
    /// "منفي ٻڙي اعشاريه ٻڙي"; a `BigDecimal` has no signed zero (its `BigInt`
    /// mantissa normalises `-0` to `0`), and the discriminating string is not
    /// carried across the `FloatValue::Decimal` boundary, so this arm drops the
    /// negword. Out of this file's remit — same boundary hole `lang_pa` flags.
    /// The float arm has no such hole: `FloatValue::Float` keeps the raw f64, so
    /// `is_sign_negative()` recovers `-0.0`'s minus faithfully.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Sign is str(number).startswith("-"); also reject the str() forms
        // int() cannot parse (exponent notation / non-finite), which SD raises
        // ValueError on. Digits still come from float2tuple, proven byte-for-byte
        // equal to str(number)'s fractional characters across a 240k fuzz.
        let negative = match value {
            FloatValue::Float { value: f, .. } => {
                if !f.is_finite() || (*f != 0.0 && (f.abs() >= 1e16 || f.abs() < 1e-4)) {
                    return Err(N2WError::Value(
                        "invalid literal for int() with base 10".to_string(),
                    ));
                }
                // is_sign_negative, not `< 0.0`: str(-0.0) == "-0.0" keeps its
                // minus, and SD peels that into a leading negword.
                f.is_sign_negative()
            }
            FloatValue::Decimal { value: d, .. } => {
                // str(Decimal) uses E-notation when the exponent is positive or
                // the adjusted exponent < -6; int() then throws ValueError.
                let (int_val, scale) = d.as_bigint_and_exponent();
                let ndigits = int_val.abs().to_string().len() as i64;
                if scale < 0 || (ndigits - 1 - scale) < -6 {
                    return Err(N2WError::Value(
                        "invalid literal for int() with base 10".to_string(),
                    ));
                }
                // Negative-zero hole: BigDecimal cannot carry "-0.0"'s sign.
                d.is_negative()
            }
        };

        let precision = value.precision() as usize;
        let (pre, post) = float2tuple(value);

        // `ret = self.negword if n.startswith("-") else ""`, then
        // `self._int_to_word(int(left))` on the *unsigned* integer part.
        let mut ret = String::new();
        if negative {
            ret.push_str(NEGWORD);
        }
        ret.push_str(&checked_int_to_word(&pre.abs())?);

        if precision > 0 {
            // `+ " " + self.pointword + " "`
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');

            // `right`: the fractional characters of str(number) — `post`,
            // left-padded with zeros to `precision` digits (post < 10**precision).
            // Mirrors floatpath::default_to_cardinal_float's own padding.
            let post_str = post.to_string();
            let padding = "0".repeat(precision.saturating_sub(post_str.len()));
            for ch in format!("{}{}", padding, post_str).chars().take(precision) {
                // Each char is one ASCII digit; `self._int_to_word(int(digit))`
                // maps 0 -> [`ZERO`] (bug 1) and 1..=9 -> ONES[d].
                let d = ch.to_digit(10).unwrap_or(0);
                ret.push_str(&int_to_word(&BigInt::from(d)));
                ret.push(' ');
            }
        }

        // `return ret.strip()` — drops the trailing digit space. (It would also
        // trim a dangling negword space, but no reachable path leaves one:
        // int_to_word never returns "" — zero yields [`ZERO`].)
        Ok(ret.trim().to_string())
    }

    // ---- currency ------------------------------------------------------

    /// For the `Currency code "X" not implemented for "Num2Word_SD"` message
    /// that `to_cheque` raises. `to_currency` never raises it — see below.
    fn lang_name(&self) -> &str {
        "Num2Word_SD"
    }

    /// `self.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately returns `None` — not the PKR fallback — for an unknown code.
    /// The fallback is a quirk of SD's *own* `to_currency` (which uses
    /// `dict.get(code, default)`); the inherited `to_cheque` uses plain
    /// `self.CURRENCY_FORMS[currency]` subscripting and lets the `KeyError`
    /// become a `NotImplementedError`. Folding the fallback in here would make
    /// `to_cheque("GBP")` succeed with rupees where Python raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Python's `Num2Word_SD.to_currency` — an outright override of the base
    /// implementation that shares almost none of its structure. It parses
    /// `str(val)` textually instead of going through `parse_currency_parts`,
    /// which is the source of most of the divergences listed below.
    ///
    /// ```python
    /// def to_currency(self, val, currency="PKR", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0])
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
    /// Note it calls `_int_to_word`, not `to_cardinal`. For the non-negative
    /// `left`/`right` this path can produce, the two agree.
    ///
    /// # Why the `Int`/`Decimal` split does not branch here
    ///
    /// `Num2Word_Base.to_currency` keys off `isinstance(val, int)` to decide
    /// whether to print cents. SD never does — it asks whether `str(val)` has a
    /// fractional component, then whether that component is truthy *as an int*
    /// (as do the other 62 languages that hand-roll this same `str(val).split(".")`
    /// override). For a float
    /// `1.0` that yields `parts[1] == "0"` -> `right == 0` -> falsy -> cents are
    /// skipped, exactly as for the int `1`. So Python itself collapses the
    /// distinction here, and the corpus agrees: `arg "1"` and `arg "1.0"` both
    /// give `"هڪ euro"`. The match below still keeps the variants apart, because
    /// they reach the same `(left, right)` by different arithmetic, not because
    /// the outputs differ.
    ///
    /// # `right` is a truncating two-digit slice
    ///
    /// `int(parts[1][:2].ljust(2, "0"))` takes the first two fractional
    /// characters and pads *right* with zeros — i.e. it reads hundredths and
    /// truncates the rest. `trunc(fraction * 100)` is the exact arithmetic
    /// equivalent for any plain-decimal `str(val)`:
    ///
    /// | `str(val)` | `parts[1]` | slice+pad | `trunc(frac*100)` |
    /// |---|---|---|---|
    /// | `12.34`  | `"34"`  | `"34"` -> 34 | 34 |
    /// | `0.5`    | `"5"`   | `"50"` -> 50 | 50 |
    /// | `1.0`    | `"0"`   | `"00"` -> 0  | 0  |
    /// | `12.345` | `"345"` | `"34"` -> 34 | 34 |
    /// | `12.005` | `"005"` | `"00"` -> 0  | 0  |
    /// | `12.999` | `"999"` | `"99"` -> 99 | 99 |
    ///
    /// All six verified against the live interpreter. Note the last three: there
    /// is **no rounding anywhere** — `12.999` bills 99 cents, and `2.675` bills
    /// 67, not the 68 that `ROUND_HALF_UP` would give in the base path.
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
        // Restore SD's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SD_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`
        let is_negative = val.is_negative();

        let (left, right) = match val {
            // `str(int)` has no ".", so `len(parts) > 1` is false and `right`
            // stays 0 — a pure int never shows cents.
            CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let d = if is_negative { d.abs() } else { d.clone() };
                // `int(parts[0])`: the integer part, truncated.
                let whole = d.with_scale(0);
                let fraction = &d - &whole;
                let cents_val = (fraction * BigDecimal::from(CENTS_DIVISOR)).with_scale(0);
                (
                    whole.as_bigint_and_exponent().0,
                    cents_val.as_bigint_and_exponent().0,
                )
            }
        };

        // `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        // — an unknown code silently renders as Pakistani rupees rather than
        // raising. The corpus leans on this hard: GBP/JPY/KWD/BHD/INR/CNY/CHF
        // are all absent from SD's table and every one of their rows comes back
        // in rupees and paisas.
        let forms = self.forms.get(currency).unwrap_or(&self.fallback_forms);

        let one = BigInt::one();
        // `cr1[1] if left != 1 else cr1[0]` — note 0 takes the plural, hence
        // the corpus's "ٻڙي euros".
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };
        let mut result = format!("{} {}", checked_int_to_word(&left)?, unit);

        // `if cents and right:` — `right == 0` is falsy, so a float with zero
        // cents drops the segment entirely. Also note that `cents=False` does
        // *not* fall back to terse digits the way the base class does: it
        // suppresses the cents segment outright. `_cents_terse` is unreachable.
        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            // `result += separator + cents_str + " " + ...` — no space before
            // the separator; the space in SD_SEPARATOR is the only one.
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(subunit);
        }

        // `result = self.negword + result` — the negword (prepended to
        // RTL text; see bug 5).
        if is_negative {
            result.insert_str(0, NEGWORD);
        }

        // `result.strip()`. A no-op on every reachable path: `_int_to_word`
        // never returns leading/trailing whitespace and never returns "".
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// precision == abs(Decimal(repr(f)).as_tuple().exponent), computed on the
    /// Python side by the binding. Hardcoded here to the values verified in the
    /// live interpreter.
    fn flt(value: f64, precision: u32) -> String {
        LangSd::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn dec(s: &str, precision: u32) -> String {
        LangSd::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    #[test]
    fn corpus_float_rows() {
        // Every `"lang":"sd","to":"cardinal"` row with a dot in `arg`.
        assert_eq!(flt(0.0, 1), "ٻڙي اعشاريه ٻڙي");
        assert_eq!(flt(0.5, 1), "ٻڙي اعشاريه پنج");
        assert_eq!(flt(1.0, 1), "هڪ اعشاريه ٻڙي");
        assert_eq!(flt(1.5, 1), "هڪ اعشاريه پنج");
        assert_eq!(flt(2.25, 2), "ٻه اعشاريه ٻه پنج");
        assert_eq!(flt(3.14, 2), "ٽي اعشاريه هڪ چار");
        assert_eq!(flt(0.01, 2), "ٻڙي اعشاريه ٻڙي هڪ");
        assert_eq!(flt(0.1, 1), "ٻڙي اعشاريه هڪ");
        assert_eq!(flt(0.99, 2), "ٻڙي اعشاريه نو نو");
        assert_eq!(flt(1.01, 2), "هڪ اعشاريه ٻڙي هڪ");
        assert_eq!(flt(12.34, 2), "ٻارهن اعشاريه ٽي چار");
        assert_eq!(flt(99.99, 2), "نوانوي اعشاريه نو نو");
        assert_eq!(flt(100.5, 1), "هڪ سو اعشاريه پنج");
        assert_eq!(flt(1234.56, 2), "هڪ هزار ٻه سو چوٽيهه اعشاريه پنج ڇهه");
        assert_eq!(flt(-0.5, 1), "منفي ٻڙي اعشاريه پنج");
        assert_eq!(flt(-1.5, 1), "منفي هڪ اعشاريه پنج");
        assert_eq!(flt(-12.34, 2), "منفي ٻارهن اعشاريه ٽي چار");
        assert_eq!(flt(1.005, 3), "هڪ اعشاريه ٻڙي ٻڙي پنج");
        assert_eq!(flt(2.675, 3), "ٻه اعشاريه ڇهه ست پنج"); // f64 artefact -> 675
        // extra live-interpreter checks
        assert_eq!(flt(2.0, 1), "ٻه اعشاريه ٻڙي");
        assert_eq!(flt(1000000.5, 1), "ڏهه لک اعشاريه پنج");
    }

    #[test]
    fn corpus_decimal_rows() {
        // Every `"lang":"sd","to":"cardinal_dec"` row.
        assert_eq!(dec("0.01", 2), "ٻڙي اعشاريه ٻڙي هڪ");
        assert_eq!(dec("1.10", 2), "هڪ اعشاريه هڪ ٻڙي"); // trailing zero kept
        assert_eq!(dec("12.345", 3), "ٻارهن اعشاريه ٽي چار پنج");
        assert_eq!(dec("98746251323029.99", 2), "نو سو ستاسي کرب ڇائيتاليهه ارب پنجويهه ڪروڙ تيرهن لک ٽريويهه هزار اوڻٽيهه اعشاريه نو نو"); // bug 3, fixed (#147)
        assert_eq!(dec("0.001", 3), "ٻڙي اعشاريه ٻڙي ٻڙي هڪ");
    }

    #[test]
    fn precision_override_ignored() {
        // Python's Num2Word_SD.to_cardinal takes no precision arg — 2.675 keeps
        // its three fractional words no matter what precision= is passed.
        let sd = LangSd::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        assert_eq!(
            sd.to_cardinal_float(&v, Some(1)).unwrap(),
            "ٻه اعشاريه ڇهه ست پنج"
        );
    }

    #[test]
    fn float_negative_zero_keeps_negword() {
        // str(-0.0) == "-0.0" -> Python prepends the negword; is_sign_negative
        // recovers it where `< 0.0` would not.
        assert_eq!(flt(-0.0, 1), "منفي ٻڙي اعشاريه ٻڙي");
    }

    #[test]
    fn scientific_and_nonfinite_raise_value() {
        let sd = LangSd::new();
        // str(1e16) == "1e+16" -> int("1e+16") raises ValueError.
        for (v, p) in [(1e16, 17u32), (1e-5, 5), (1.5e-5, 6), (f64::INFINITY, 0)] {
            assert!(matches!(
                sd.to_cardinal_float(&FloatValue::Float { value: v, precision: p }, None),
                Err(N2WError::Value(_))
            ));
        }
        // str(Decimal("1E-7")) == "1E-7" (adjusted exp -7 < -6) -> ValueError.
        assert!(matches!(
            sd.to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str("0.0000001").unwrap(),
                    precision: 7,
                },
                None,
            ),
            Err(N2WError::Value(_))
        ));
    }
}
