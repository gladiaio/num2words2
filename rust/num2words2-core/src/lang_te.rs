//! Port of `lang_TE.py` (Telugu).
//!
//! Registry: `CONVERTER_CLASSES["te"] -> lang_TE.Num2Word_TE` (verified in
//! `num2words2/__init__.py:320`).
//!
//! Shape: **engine**. `Num2Word_TE` subclasses `Num2Word_EUR` and defines
//! `low_numwords`/`mid_numwords`/`high_numwords` + `merge`, so
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean`/`merge`. `cards`,
//! `maxval` and `merge` are all live here.
//!
//! # Inheritance chain
//!
//! `Num2Word_TE` -> `Num2Word_EUR` -> `Num2Word_Base`. TE overrides both
//! `setup` and `set_high_numwords` **without calling `super()`**, so EUR
//! contributes nothing that is observable in the four *numeric* modes:
//!   * EUR's `setup` (the Latin `cent`/`illion` stem generator) is fully
//!     shadowed — `high_numwords` is TE's `[(7, ..), (5, ..), (3, ..)]`, not
//!     EUR's list of stems.
//!   * EUR's `set_high_numwords` (long-scale illiard/illion pairing over
//!     `zip(high, range(cap, 3, -6))`) is fully shadowed — TE ignores the
//!     `high` argument entirely and reads `self.high_numwords` directly,
//!     mapping `10**n -> word`.
//!   * EUR's `GIGA_SUFFIX`/`MEGA_SUFFIX`/`gen_high_numwords` are never
//!     reached.
//!
//! EUR *does* contribute the entire currency surface — `pluralize`,
//! `CURRENCY_FORMS` and `CURRENCY_ADJECTIVES` all resolve to it, and the forms
//! dict is not the one `lang_EUR.py` appears to define. See "Currency" below.
//!
//! Inherited from `Num2Word_Base` and left at the trait defaults:
//!   * `negword` = `"(-) "` — TE's `setup` never assigns `negword`, so the
//!     base default survives and negatives render as `"(-) ఏడు"` (not a
//!     Telugu word). Confirmed against the corpus.
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` — TE and EUR both
//!     leave it alone, so years are plain cardinals with no era handling and
//!     **no** rejection of negatives: `to_year(-500)` == `"(-) అయిదు వంద"`.
//!   * `title`/`is_title`/`exclude_title` — never enabled for TE.
//!
//! # Numbering system
//!
//! Indian (vedic) grouping, not the Western short/long scale. The card table
//! is `10**7 -> కోట్ల` (crore), `10**5 -> లక్ష` (lakh), `10**3 -> వేయి`
//! (thousand), `100 -> వంద`, plus every value `0..=99` spelled out
//! individually. There is deliberately **no card for 10**6**, so a million is
//! "పది లక్ష" (ten lakh) and a billion is "ఒకటి వంద కోట్ల" (one hundred
//! crore).
//!
//! `MAXVAL = 1000 * highest card = 1000 * 10**7 = 10**10`, so `to_cardinal`
//! raises `OverflowError` from `10**10` upward. This is a genuinely low
//! ceiling compared to most languages — values are still `BigInt` (the
//! overflow check must compare against `10**10` without truncating a huge
//! input), but nothing below the ceiling exceeds 10 digits.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong-looking and
//! all are exactly what Python emits, verified against the interpreter and
//! the frozen corpus:
//!
//! 1. **The `modifiers` table is dead code.** `merge` and `to_ordinal` both
//!    test `text[-1] in self.modifiers`. `text[-1]` is a *single character*,
//!    but every one of the 17 `modifiers` entries is 2-4 characters long —
//!    each intended combining vowel sign (U+0C3E..U+0C4D, U+0C01..U+0C03) is
//!    padded with stray ASCII spaces, e.g. `" ి "` is `U+0020 U+0C3F U+0020`
//!    and `" ొ"` is `U+0020 U+0C4A`. A 1-char string can never equal a 2-char
//!    string, so **both tests are always False**. Consequences:
//!      * `merge`'s `lnum >= 100 > rnum` branch always takes the `else` arm
//!        and appends the plural infix `ల`; the `ltext[:-1]` arm that would
//!        strip a trailing vowel sign is unreachable.
//!      * `to_ordinal` never strips anything and is always
//!        `to_cardinal(value) + "వ"`.
//!    [`last_char_is_modifier`] performs the real comparison rather than
//!    hard-coding `false`, so the deadness is emergent from the ported table
//!    exactly as in Python. Fixing the padding would change output.
//!
//! 2. **`cards[50]` had a trailing space (fixed, #160)**: `"యాభై "`
//!    (`lang_TE.py:64`), while every other round ten is unpadded. In Python it
//!    leaks into output: `to_cardinal(50)` == `"యాభై "`, `to_ordinal(50)` ==
//!    `"యాభై వ"` (unlike `"నలభైవ"` for 40), currency `"యాభై  cents"`. The
//!    port's table drops the space, so 50 behaves like the other round tens.
//!
//! 3. **`cards[35]` is `"ముప్పై ఐదు"`**, using `ఐదు` for "five" where every
//!    other compound in the table (5, 25, 45, 55, ...) uses `అయిదు`. A
//!    one-off inconsistency in the source table, kept verbatim.
//!
//! 4. **`merge`'s `100 > lnum > rnum` branch (the `"%s-%s"` hyphen join) is
//!    unreachable.** Because `cards` enumerates every value `0..=99`, any
//!    sub-100 value is a direct card hit that `splitnum` resolves with
//!    `div == 1` (taking `merge`'s first branch), and any `merge` carrying a
//!    remainder has already accumulated `lnum >= elem >= 100`. No TE output
//!    ever contains a hyphen. Ported anyway for structural fidelity.
//!
//! 5. **`to_cardinal(100)` == `"ఒకటి వంద"`** ("one hundred", via the
//!    `rnum > lnum` multiply branch) whereas `to_cardinal(101)` ==
//!    `"ఒకటి వందల ఒకటి"` — the `ల` infix appears only when a remainder
//!    follows. Likewise `1000` -> `"ఒకటి వేయి"` but `1001` ->
//!    `"ఒకటి వేయిల ఒకటి"`, and `1100` -> `"ఒకటి వేయి ఒకటి వంద"` (no infix,
//!    since `rnum == 100` fails the strict `100 > rnum`).
//!
//! # Error variants
//!
//! Only two are reachable in scope, both deliberate raises rather than
//! crashes:
//!   * `OverflowError` -> [`N2WError::Overflow`], from
//!     `Num2Word_Base.to_cardinal` at `value >= MAXVAL` (`10**10`).
//!   * `TypeError` -> [`N2WError::Type`], from `verify_ordinal` on negative
//!     input to `to_ordinal`/`to_ordinal_num`. `to_cardinal` and `to_year`
//!     accept negatives.
//!
//! `to_ordinal_num` calls `verify_ordinal` and then `to_ordinal`, which calls
//! `verify_ordinal` again — harmless, but it means a negative fails on the
//! first check and an overflowing value fails inside the nested
//! `to_cardinal`, so `to_ordinal_num(-1)` is a `TypeError` while
//! `to_ordinal_num(10**10)` is an `OverflowError`. Both match the corpus.
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_TE` defines no `str_to_number` and stashes no flags; the
//! only instance state is the immutable `cards`/`modifiers` tables built in
//! `setup`. Nothing for the Python dispatcher to skip.
//!
//! # Currency
//!
//! TE defines **no** currency members at all — no `CURRENCY_FORMS`, no
//! `CURRENCY_ADJECTIVES`, no `CURRENCY_PRECISION`, no `pluralize`, no
//! `to_currency`/`to_cheque`, no `_money_verbose`/`_cents_verbose`. Everything
//! is inherited, and *what* is inherited is not what `lang_EUR.py` reads like.
//! See [`CURRENCY_FORMS`] — the table is assembled at import time by a
//! different module.
//!
//! Resolution through the MRO (`Num2Word_TE` -> `Num2Word_EUR` ->
//! `Num2Word_Base`):
//!   * `CURRENCY_FORMS` -> `Num2Word_EUR.CURRENCY_FORMS`, **as mutated by
//!     `Num2Word_EN.__init__`** (see below).
//!   * `CURRENCY_ADJECTIVES` -> `Num2Word_EUR.CURRENCY_ADJECTIVES`, unmutated
//!     — nothing in the library writes into it, only reads.
//!   * `CURRENCY_PRECISION` -> `Num2Word_Base.CURRENCY_PRECISION`, which is
//!     `{}` — see [`Lang::currency_precision`] note below. **Every** currency
//!     is 2-decimal for TE, including KWD/BHD and JPY.
//!   * `pluralize` -> `Num2Word_EUR.pluralize` (`forms[0 if n == 1 else 1]`).
//!   * `to_currency` / `to_cheque` / `_money_verbose` / `_cents_verbose` /
//!     `_cents_terse` -> `Num2Word_Base`, i.e. the trait defaults.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// The plural/oblique infix `merge` appends before a remainder (`ల`, U+0C32).
const LA: &str = "\u{c32}";

/// The ordinal suffix `to_ordinal` appends (`వ`, U+0C35).
const VA: &str = "\u{c35}";

/// `self.modifiers` from `lang_TE.py`, verbatim.
///
/// Written with `\u{}` escapes because every entry mixes a lone Telugu
/// combining mark with ASCII spaces, which renders as mojibake in an editor
/// and would be trivially corrupted by hand-transcription. The trailing
/// comment on each line gives the exact codepoint sequence.
///
/// **Every entry is 2-4 chars**, which is precisely why the `text[-1] in
/// modifiers` tests in `merge`/`to_ordinal` can never match — see the module
/// docs, bug 1. Do not "clean up" the spaces.
const MODIFIERS: [&str; 17] = [
    "\u{20}\u{c4d}\u{20}\u{20}", // U+0020 U+0C4D U+0020 U+0020
    "\u{c3e}\u{20}",             // U+0C3E U+0020
    "\u{20}\u{c3f}\u{20}",       // U+0020 U+0C3F U+0020
    "\u{20}\u{c40}\u{20}",       // U+0020 U+0C40 U+0020
    "\u{20}\u{c41}\u{20}",       // U+0020 U+0C41 U+0020
    "\u{20}\u{c42}\u{20}",       // U+0020 U+0C42 U+0020
    "\u{20}\u{c43}\u{20}",       // U+0020 U+0C43 U+0020
    "\u{20}\u{c44}\u{20}\u{20}", // U+0020 U+0C44 U+0020 U+0020
    "\u{20}\u{c46}\u{20}",       // U+0020 U+0C46 U+0020
    "\u{20}\u{c47}\u{20}",       // U+0020 U+0C47 U+0020
    "\u{20}\u{c48}\u{20}",       // U+0020 U+0C48 U+0020
    "\u{20}\u{c4a}",             // U+0020 U+0C4A
    "\u{20}\u{c4b}\u{20}",       // U+0020 U+0C4B U+0020
    "\u{20}\u{c4c}\u{20}",       // U+0020 U+0C4C U+0020
    "\u{20}\u{c01}\u{20}",       // U+0020 U+0C01 U+0020
    "\u{20}\u{c02}\u{20}",       // U+0020 U+0C02 U+0020
    "\u{20}\u{c03}\u{20}",       // U+0020 U+0C03 U+0020
];

/// `self.high_numwords`: `(exponent, word)`, mapped by TE's
/// `set_high_numwords` to `cards[10**n] = word`.
const HIGH: [(u32, &str); 3] = [(7, "కోట్ల"), (5, "లక్ష"), (3, "వేయి")];

/// The effective `CURRENCY_FORMS` for `Num2Word_TE`: `(code, unit, subunit)`.
///
/// # This table is NOT what `lang_EUR.py` says. Read this before "fixing" it.
///
/// `Num2Word_TE` inherits `CURRENCY_FORMS` from `Num2Word_EUR`, and
/// `lang_EUR.py`'s class body lists 22 codes with e.g.
/// `"EUR": (("euro", "euro"), ...)` and `"GBP": (("pound sterling",
/// "pounds sterling"), ...)`. **That literal is not what TE sees at runtime.**
/// TE sees 39 codes, `EUR` pluralises to `"euros"`, and `GBP` is plain
/// `"pound"/"pounds"`.
///
/// The reason is a shared-mutable-class-attribute side effect in
/// `lang_EN.py`. `Num2Word_EN(Num2Word_EUR)` declares no `CURRENCY_FORMS` of
/// its own, so in its `__init__` the statement
///
/// ```python
/// self.CURRENCY_FORMS["EUR"] = (("euro", "euros"), ("cent", "cents"))
/// ```
///
/// is an **item assignment**, not a rebind: attribute lookup walks the MRO to
/// `Num2Word_EUR.CURRENCY_FORMS` and mutates *that dict in place*. There is one
/// dict object for the whole `Num2Word_EUR` subtree, so every one of EN's ~27
/// writes is visible to every EUR subclass — `hu`, `kn`, `sv` and `te`
/// included. `num2words2/__init__.py` instantiates every converter eagerly in
/// the `CONVERTER_CLASSES` literal, so all of this has already happened before
/// any caller can observe it, and the merged result is deterministic.
/// `Num2Word_EN_IN.__init__` writes `INR` too, with the identical value.
///
/// Verified at runtime rather than reasoned about:
/// `CONVERTER_CLASSES["te"].CURRENCY_FORMS is lang_EUR.Num2Word_EUR
/// .CURRENCY_FORMS` is `True`, and this table is a mechanical dump of that
/// dict after a full `import num2words2`. The whole thing was code-generated
/// from the live object — `fillér`, `króna`/`krónur`, `øre`, `öre` are easy to
/// corrupt by hand. Regenerate rather than retype.
///
/// Consequences worth stating out loud, all corpus-confirmed:
///   * `EUR` is `("euro", "euros")` — EN's value, not EUR's `("euro", "euro")`.
///   * `GBP` is `("pound", "pounds")`, not `("pound sterling", ...)`.
///   * `SAR` is `("riyal", "riyals")`, not EUR's `("saudi riyal", ...)`.
///   * 17 codes exist *only* because EN added them: AED, BRL, CHF, HKD, IQD,
///     JOD, KWD, LYD, NGN, NZD, OMR, QAR, SGD, TND, ZAR (plus BHD, CNY).
///   * `PLN` and `RON` keep **three** unit forms. `pluralize` only ever indexes
///     0 or 1, so the third is unreachable, but the arity is preserved because
///     dropping it would be a silent behaviour change if `pluralize` ever
///     grew a third branch.
const CURRENCY_FORMS: [(&str, &[&str], &[&str]); 39] = [
    ("AED", &["dirham", "dirhams"], &["fils", "fils"]),
    ("AUD", &["dollar", "dollars"], &["cent", "cents"]),
    ("BHD", &["dinar", "dinars"], &["fils", "fils"]),
    ("BRL", &["real", "reais"], &["cent", "cents"]),
    ("BYN", &["rouble", "roubles"], &["kopek", "kopeks"]),
    ("CAD", &["dollar", "dollars"], &["cent", "cents"]),
    ("CHF", &["franc", "francs"], &["rappen", "rappen"]),
    ("CNY", &["yuan", "yuan"], &["fen", "fen"]),
    ("EEK", &["kroon", "kroons"], &["sent", "senti"]),
    ("EUR", &["euro", "euros"], &["cent", "cents"]),
    ("GBP", &["pound", "pounds"], &["penny", "pence"]),
    ("HKD", &["dollar", "dollars"], &["cent", "cents"]),
    ("HUF", &["forint", "forint"], &["fillér", "fillér"]),
    ("INR", &["rupee", "rupees"], &["paisa", "paise"]),
    ("IQD", &["dinar", "dinars"], &["fils", "fils"]),
    ("ISK", &["króna", "krónur"], &["aur", "aurar"]),
    ("JOD", &["dinar", "dinars"], &["fils", "fils"]),
    ("JPY", &["yen", "yen"], &["sen", "sen"]),
    ("KRW", &["won", "won"], &["jeon", "jeon"]),
    ("KWD", &["dinar", "dinars"], &["fils", "fils"]),
    ("LTL", &["litas", "litas"], &["cent", "cents"]),
    ("LVL", &["lat", "lats"], &["santim", "santims"]),
    ("LYD", &["dinar", "dinars"], &["dirham", "dirhams"]),
    ("MXN", &["peso", "pesos"], &["cent", "cents"]),
    ("NGN", &["naira", "naira"], &["kobo", "kobo"]),
    ("NOK", &["krone", "kroner"], &["øre", "øre"]),
    ("NZD", &["dollar", "dollars"], &["cent", "cents"]),
    ("OMR", &["rial", "rials"], &["baisa", "baisa"]),
    ("PLN", &["zloty", "zlotys", "zlotu"], &["grosz", "groszy"]),
    ("QAR", &["riyal", "riyals"], &["dirham", "dirhams"]),
    ("RON", &["leu", "lei", "de lei"], &["ban", "bani", "de bani"]),
    ("RUB", &["rouble", "roubles"], &["kopek", "kopeks"]),
    ("SAR", &["riyal", "riyals"], &["halalah", "halalas"]),
    ("SEK", &["krona", "kronor"], &["öre", "öre"]),
    ("SGD", &["dollar", "dollars"], &["cent", "cents"]),
    ("TND", &["dinar", "dinars"], &["millime", "millimes"]),
    ("USD", &["dollar", "dollars"], &["cent", "cents"]),
    ("UZS", &["sum", "sums"], &["tiyin", "tiyins"]),
    ("ZAR", &["rand", "rand"], &["cent", "cents"]),
];

/// `self.low_numwords`, in Python source order.
///
/// `set_low_numwords` zips this against `range(99, -1, -1)`, so index 0 is
/// the word for 99 and index 99 is the word for 0. One entry is odd and
/// intentional: `"ముప్పై ఐదు"` (35) uses `ఐదు` where its neighbours use
/// `అయిదు` (module docs, bug 3). Python's 50 also carries a trailing space,
/// dropped here (bug 2).
///
/// # DO NOT retype or reformat this table — it is not NFC
///
/// `lang_TE.py` stores the vowel sign ై **decomposed**, as
/// `U+0C46 U+0C56` (VOWEL SIGN E + AI LENGTH MARK), not as the canonically
/// equivalent precomposed `U+0C48`. 78 of these 100 entries contain it (every
/// word built on ఇరవై / ముప్పై / నలభై / యాభై / అరవై / డెబ్బై / ఎనభై / తొంభై).
/// The two forms are visually identical and compare equal under NFC, but they
/// are **different bytes**, and the corpus fixes the decomposed bytes that
/// Python emits.
///
/// Any round-trip through a normalizing editor, formatter, or hand-copy will
/// silently rewrite these to NFC and break `to_cardinal` for ~80% of the
/// range while looking completely correct in a diff. This table was spliced
/// programmatically out of the Python module for exactly that reason; if it
/// ever needs to change, regenerate it rather than typing it, and verify with
/// a byte comparison rather than by eye.
const LOW: [&str; 100] = [
    "తొంభై తొమ్మిది",
    "తొంభై ఎనిమిది",
    "తొంభై ఏడు",
    "తొంభై ఆరు",
    "తొంభై అయిదు",
    "తొంభై నాలుగు",
    "తొంభై మూడు",
    "తొంభై రెండు",
    "తొంభై ఒకటి",
    "తొంభై",
    "ఎనభై తొమ్మిది",
    "ఎనభై ఎనిమిది",
    "ఎనభై ఏడు",
    "ఎనభై ఆరు",
    "ఎనభై అయిదు",
    "ఎనభై నాలుగు",
    "ఎనభై మూడు",
    "ఎనభై రెండు",
    "ఎనభై ఒకటి",
    "ఎనభై",
    "డెబ్బై తొమ్మిది",
    "డెబ్బై ఎనిమిది",
    "డెబ్బై ఏడు",
    "డెబ్బై ఆరు",
    "డెబ్బై అయిదు",
    "డెబ్బై నాలుగు",
    "డెబ్బై మూడు",
    "డెబ్బై రెండు",
    "డెబ్బై ఒకటి",
    "డెబ్బై",
    "అరవై తొమ్మిది",
    "అరవై ఎనిమిది",
    "అరవై ఏడు",
    "అరవై ఆరు",
    "అరవై అయిదు",
    "అరవై నాలుగు",
    "అరవై మూడు",
    "అరవై రెండు",
    "అరవై ఒకటి",
    "అరవై",
    "యాభై తొమ్మిది",
    "యాభై ఎనిమిది",
    "యాభై ఏడు",
    "యాభై ఆరు",
    "యాభై అయిదు",
    "యాభై నాలుగు",
    "యాభై మూడు",
    "యాభై రెండు",
    "యాభై ఒకటి",
    "యాభై",  // 50: lang_TE.py:64 has a trailing space here, dropped - see bug 2
    "నలభై తొమ్మిది",
    "నలభై ఎనిమిది",
    "నలభై ఏడు",
    "నలభై ఆరు",
    "నలభై అయిదు",
    "నలభై నాలుగు",
    "నలభై మూడు",
    "నలభై రెండు",
    "నలభై ఒకటి",
    "నలభై",
    "ముప్పై తొమ్మిది",
    "ముప్పై ఎనిమిది",
    "ముప్పై ఏడు",
    "ముప్పై ఆరు",
    "ముప్పై ఐదు",  // 35: `aidu` not `ayidu` - verbatim, see bug 3
    "ముప్పై నాలుగు",
    "ముప్పై మూడు",
    "ముప్పై రెండు",
    "ముప్పై ఒకటి",
    "ముప్పై",
    "ఇరవై తొమ్మిది",
    "ఇరవై ఎనిమిది",
    "ఇరవై ఏడు",
    "ఇరవై ఆరు",
    "ఇరవై అయిదు",
    "ఇరవై నాలుగు",
    "ఇరవై మూడు",
    "ఇరవై రెండు",
    "ఇరవై ఒకటి",
    "ఇరవై",
    "పందొమ్మిది",
    "పధ్ధెనిమిది",
    "పదిహేడు",
    "పదహారు",
    "పదునయిదు",
    "పధ్నాలుగు",
    "పదమూడు",
    "పన్నెండు",
    "పదకొండు",
    "పది",
    "తొమ్మిది",
    "ఎనిమిది",
    "ఏడు",
    "ఆరు",
    "అయిదు",
    "నాలుగు",
    "మూడు",
    "రెండు",
    "ఒకటి",
    "సున్న",
];

/// Python's `text[-1] in self.modifiers`.
///
/// Takes the last **character** (not byte — the words are Telugu) and tests
/// it for membership in [`MODIFIERS`] by string equality, exactly as Python's
/// `in` does against a list. Always returns `false` in practice because every
/// modifier is multi-character; see module docs, bug 1.
///
/// Python would raise `IndexError` on `""[-1]`, but the callers can never
/// pass an empty string: `merge`'s `ltext` is either a card word (all
/// non-empty) or a previous `merge` result (all non-empty), and
/// `to_ordinal`'s input is a completed `to_cardinal`. `false` is therefore an
/// unreachable-branch placeholder, not a behavioural choice.
fn last_char_is_modifier(text: &str) -> bool {
    match text.chars().next_back() {
        Some(c) => {
            let mut buf = [0u8; 4];
            let last: &str = c.encode_utf8(&mut buf);
            MODIFIERS.iter().any(|m| *m == last)
        }
        None => false,
    }
}

pub struct LangTe {
    cards: Cards,
    maxval: BigInt,
    /// [`CURRENCY_FORMS`] materialised once, in [`LangTe::new`].
    ///
    /// Python resolves `self.CURRENCY_FORMS[code]` to a single dict shared
    /// across the whole `Num2Word_EUR` subtree and built once at import. The
    /// equivalent here is one owned map per converter instance, built once in
    /// `new()`. Rebuilding it per `to_currency` call would allocate 39
    /// `CurrencyForms` (each with its own `Vec<String>`) to answer a single
    /// lookup, which is how an earlier iteration of this port ended up
    /// measurably slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangTe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTe {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // TE's set_high_numwords ignores its `high` argument and reads
        // self.high_numwords directly: cards[10**n] = word.
        for (n, word) in HIGH.iter() {
            cards.insert(BigInt::from(10u8).pow(*n), *word);
        }

        // self.mid_numwords = [(100, "వంద")]
        set_mid_numwords(&mut cards, &[(100, "వంద")]);

        set_low_numwords(&mut cards, &LOW);

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10**7 = 10**10
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Built once, here — never per call. See the field docs.
        let currency_forms = CURRENCY_FORMS
            .iter()
            .map(|(code, unit, subunit)| (*code, CurrencyForms::new(unit, subunit)))
            .collect();

        LangTe {
            cards,
            maxval,
            currency_forms,
        }
    }

    /// Python's `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`value == int(value)`) is vacuous for `BigInt`, so
    /// only the negative check can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == num_bigint::Sign::Minus {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

impl Lang for LangTe {
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
        ","
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    /// TE's `setup` never touches `negword`, so `Num2Word_Base`'s default
    /// survives. Stated explicitly because "(-) " looks like a placeholder
    /// someone forgot to translate — it is what Python emits.
    fn negword(&self) -> &str {
        "(-) "
    }

    /// `self.pointword` from TE's `setup`. Python's value carries a trailing
    /// space, which the float path's own separator doubled ("సున్న బిందువు  అయిదు");
    /// the port drops it (#160).
    fn pointword(&self) -> &str {
        "బిందువు"
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);

        if lnum.is_one() && rnum < &hundred {
            // if lnum == 1 and rnum < 100: return (rtext, rnum)
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            // elif 100 > lnum > rnum: "%s-%s"
            // Unreachable for TE (module docs, bug 4) — ported for fidelity.
            (format!("{}-{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            // elif lnum >= 100 > rnum
            if last_char_is_modifier(ltext) {
                // Dead arm (bug 1). Python: "%s %s" % (ltext[:-1], rtext) —
                // drops the final *character*.
                let mut trimmed = ltext.to_string();
                trimmed.pop();
                (format!("{} {}", trimmed, rtext), lnum + rnum)
            } else {
                // "%s %s" % (ltext + "ల", rtext)
                (format!("{}{} {}", ltext, LA, rtext), lnum + rnum)
            }
        } else if rnum > lnum {
            // elif rnum > lnum: "%s %s", lnum * rnum
            (format!("{} {}", ltext, rtext), lnum * rnum)
        } else {
            // return "%s %s", lnum + rnum
            (format!("{} {}", ltext, rtext), lnum + rnum)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut outwords = self.to_cardinal(value)?;
        if last_char_is_modifier(&outwords) {
            // Dead arm (bug 1): Python's outwords = outwords[:-1].
            outwords.pop();
        }
        outwords.push_str(VA);
        Ok(outwords)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        // Python: "%s%s" % (value, self.to_ordinal(value)[-1:]) — a *slice*,
        // so an empty ordinal would yield "" rather than raising IndexError.
        let suffix: String = ord.chars().next_back().map(String::from).unwrap_or_default();
        Ok(format!("{}{}", value, suffix))
    }

    /// `to_ordinal(float/Decimal)` — `verify_ordinal` gates the float domain:
    /// a fractional value raises TypeError ("Cannot treat float %s as
    /// ordinal.") before the negative check ("Cannot treat negative num %s
    /// as ordinal."); -0.0 passes both (`abs(-0.0) == -0.0`). A surviving
    /// whole value takes the integer ordinal path — base `to_cardinal`
    /// routes a whole float through `int(value)` — so `5.00` is "అయిదువ"
    /// and `1e+12` still overflows inside `to_cardinal` (MAXVAL is 10**10).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = verify_ordinal_float(value, &ordinal_float_repr(value))?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)` — the same gate, then
    /// `"%s%s" % (value, self.to_ordinal(value)[-1:])`: the repr plus the
    /// *last character* of the real ordinal ("5.0వ"). `to_ordinal` runs for
    /// real, so a whole value past MAXVAL ("1E+20") raises OverflowError
    /// here rather than echoing.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = verify_ordinal_float(value, repr_str)?;
        let ord = self.to_ordinal(&i)?;
        let suffix: String = ord.chars().next_back().map(String::from).unwrap_or_default();
        Ok(format!("{}{}", repr_str, suffix))
    }

    // to_year: Num2Word_Base's default (`return self.to_cardinal(value)`) is
    // inherited unchanged through EUR and TE, and the trait default already
    // delegates through `&self`. Negatives are NOT rejected here — only
    // to_ordinal* call verify_ordinal.

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, interpolated into the NotImplementedError
    /// that `to_currency`/`to_cheque` raise for an unknown code. TE is the
    /// concrete class, so it is `Num2Word_TE` and not any of its bases.
    fn lang_name(&self) -> &str {
        "Num2Word_TE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EUR.CURRENCY_ADJECTIVES`, verbatim and complete.
    ///
    /// Unlike `CURRENCY_FORMS`, this dict is never mutated — every reference in
    /// the library reads it (`self.CURRENCY_ADJECTIVES[currency]`), so what TE
    /// inherits is exactly `lang_EUR.py`'s literal. Confirmed by dumping the
    /// live object.
    ///
    /// Only reachable via `adjective=True`, which no corpus row exercises, so
    /// this hook is ported from the source rather than pinned by the oracle.
    /// Note `íslenskar` (ISK) is lowercase Icelandic among otherwise
    /// capitalised English demonyms — that is what `lang_EUR.py` says.
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        match code {
            "AUD" => Some("Australian"),
            "BYN" => Some("Belarusian"),
            "CAD" => Some("Canadian"),
            "EEK" => Some("Estonian"),
            "HUF" => Some("Hungarian"),
            "INR" => Some("Indian"),
            "ISK" => Some("íslenskar"),
            "JPY" => Some("Japanese"),
            "KRW" => Some("Korean"),
            "MXN" => Some("Mexican"),
            "NOK" => Some("Norwegian"),
            "RON" => Some("Romanian"),
            "RUB" => Some("Russian"),
            "SAR" => Some("Saudi"),
            "USD" => Some("US"),
            "UZS" => Some("Uzbekistan"),
            _ => None,
        }
    }

    // currency_precision: deliberately NOT overridden. `Num2Word_TE` and
    // `Num2Word_EUR` both leave `CURRENCY_PRECISION` alone, so it resolves to
    // `Num2Word_Base.CURRENCY_PRECISION`, which is `{}` — every code takes the
    // `.get(code, 100)` default of 100. The trait default returns 100, which is
    // already right.
    //
    // This one is worth being explicit about, because the *forms* table above
    // does leak out of `lang_EN.py` and the precision table looks like it
    // should leak the same way. It does not. `lang_EN.py` writes its forms with
    // `self.CURRENCY_FORMS["KWD"] = ...` (item assignment -> mutates the shared
    // EUR dict) but its precisions with
    //
    //     self.CURRENCY_PRECISION = {"BHD": 1000, "KWD": 1000, ...}
    //
    // which is a *rebind*: it creates an instance attribute on the EN instance
    // and leaves `Num2Word_Base.CURRENCY_PRECISION` empty. Nothing anywhere in
    // the library does `CURRENCY_PRECISION[...] = ...`, so no code path can
    // ever populate it for TE.
    //
    // So, for TE, and confirmed against the corpus:
    //   * KWD and BHD are 2-decimal, not 3. `to_currency(12.34, "KWD")` is
    //     "పన్నెండు dinars, ముప్పై నాలుగు fils" (34 fils), not 340, and
    //     `to_cheque(1234.56, "KWD")` prints "56/100", not "560/1000".
    //   * JPY is 2-decimal, not 0. It keeps its historical `sen` subunit and
    //     `to_currency(12.34, "JPY")` renders cents rather than rounding to a
    //     whole yen — the `divisor == 1` branch in `default_to_currency` is
    //     unreachable for TE.
    // A language-shaped "fix" here would silently break 24 corpus rows.

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Not a Telugu rule — TE inherits EUR's English-style binary plural and
    /// applies it to the (English) currency names in `CURRENCY_FORMS`. The
    /// Telugu words in the output come from `to_cardinal`; the unit names never
    /// get translated. That is what the corpus fixes:
    /// `to_currency(2, "EUR", lang="te")` == `"రెండు euros"`.
    ///
    /// `n` here is always non-negative — `to_currency` passes `abs(val)` on the
    /// int path and `parse_currency_parts` returns magnitudes on the float
    /// path — so the `n == 1` test needs no sign handling.
    ///
    /// The `Index` error is Python's `IndexError` from `forms[1]` on a
    /// one-element tuple. Unreachable with the table above (every entry has 2
    /// or 3 forms), but spelled out rather than unwrapped so a future table
    /// edit surfaces as the exception Python would raise instead of a panic.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    // _money_verbose / _cents_verbose / _cents_terse: TE overrides none of
    // them, so `Num2Word_Base`'s versions apply — `to_cardinal(number)` for the
    // first two and the zero-padded `"%0*d"` for the third. The trait defaults
    // are exactly those. Left alone.
    //
    // to_currency / to_cheque: TE and EUR define neither, so
    // `Num2Word_Base.to_currency` / `.to_cheque` run unmodified and the trait
    // defaults (`currency::default_to_currency` / `default_to_cheque`) already
    // are that code. Left alone.
    //
    // cardinal_from_decimal: left at the default, which raises. It backs the
    // fractional-cents branch of `default_to_currency` (Python's
    // `self.to_cardinal(float(right))`), which TE reaches only for input with
    // more than 2 decimals, e.g. `to_currency(1.234, "EUR")`. Python renders
    // that via `to_cardinal_float` using `pointword`; the float
    // cardinal path is a later phase, so this Rust port raises where Python
    // returns a string. No corpus row hits it — every currency value is 0, 1, 2
    // decimals — but it is a real gap, flagged in `concerns`.
}

/// Python's `Num2Word_Base.verify_ordinal` over the float/Decimal domain:
///
/// ```python
/// if not value == int(value):
///     raise TypeError(self.errmsg_floatord % value)
/// if not abs(value) == value:
///     raise TypeError(self.errmsg_negord % value)
/// ```
///
/// The float check runs first, so `-1.5` reports the *float* message; the
/// negative check compares numerically, so `-0.0` (== `abs(-0.0)`) passes.
/// `repr` is Python's `str(value)`, interpolated verbatim into the message.
fn verify_ordinal_float(value: &FloatValue, repr: &str) -> Result<BigInt> {
    match value.as_whole_int() {
        None => Err(N2WError::Type(format!(
            "Cannot treat float {} as ordinal.",
            repr
        ))),
        Some(i) if i.is_negative() => Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            repr
        ))),
        Some(i) => Ok(i),
    }
}

/// Best-effort `str(value)` for the TypeError messages raised by
/// [`verify_ordinal_float`] when no repr was handed in (the `to_ordinal`
/// entry). Exact for every reachable message: `str(Decimal)` is the spec
/// transcription, a fractional float's shortest round-trip matches Rust's
/// `Display` in the non-exponent range, and a whole float re-gains its
/// Python ".0" tail. (A whole float >= 1e16 would diverge — Python uses
/// exponent form — but such values pass verification and never reach a
/// message.)
fn ordinal_float_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
        FloatValue::Float { value, .. } => {
            if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e16 {
                format!("{:.1}", value)
            } else {
                value.to_string()
            }
        }
    }
}
