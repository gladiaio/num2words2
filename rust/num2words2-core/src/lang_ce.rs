//! Port of `lang_CE.py` (Chechen).
//!
//! Shape: **self-contained**. `Num2Word_CE` subclasses `Num2Word_EUR` (→
//! `Num2Word_Base`), and `Num2Word_EUR.setup` *does* set `high_numwords`, so
//! Python really does build `self.cards`/`MAXVAL` in `Num2Word_Base.__init__`
//! — but every one of the four in-scope methods is overridden, so cards,
//! `merge` and `MAXVAL` are never read. Two consequences:
//!
//!   * `cards`/`merge` stay at their trait defaults here.
//!   * **Fixed (gladiaio/num2words2#159): the ceiling is 10^34.** Python
//!     returned the literal string `"NOT IMPLEMENTED"` as the "words" for
//!     `number >= 10**34` (and for ±Infinity), while its inherited `MAXVAL`
//!     claimed ~10^606. The port raises `OverflowError` ("abs(v) must be less
//!     than 10^34.") there instead, and `maxval("ce")` reports 10^34.
//!
//! Python's `to_cardinal(number, clazz="д", case="abs")` carries two keyword
//! arguments. The bare trait entry points always run at the defaults ("д" /
//! "abs", or "ORD" via `to_ordinal`); the `*_kw` hooks accept the caller's
//! kwargs bag and thread them through — `to_cardinal_kw` takes both `clazz`
//! and `case`, `to_ordinal_kw` only `clazz`, `to_year_kw` and `to_currency_kw`
//! only `case`, exactly the Python signatures. A bad `case` value surfaces as
//! Python's `KeyError` from the suffix-table subscript ("bogus" is in no
//! table); a non-string value (e.g. an explicit `None`) is NotImplemented so
//! the dispatcher falls back to Python, which raises the original
//! `TypeError`/`KeyError` itself. The internal helpers below keep both
//! parameters so the recursion stays faithful — the language recurses with
//! `case="attr"` on its own. See [`DEFAULT_CLAZZ`].
//!
//! No cross-call mutable state: `to_cardinal` never touches `self`.
//!
//! # Chechen grammar, in one paragraph
//!
//! Numerals inflect for case, and 4/14/40 (and their compounds) agree in noun
//! class. The tables encode the class slot as the literal marker `"д*"`, which
//! `.replace("д*", clazz)` rewrites — hence "д*иъ" → "диъ" under the default
//! class "д". `makecase` picks a stored form when the entry has one, else
//! glues a case suffix onto the `abs` form, choosing the vowel-stem or
//! consonant-stem suffix table by the last character of `abs`.
//!
//! # Faithfully reproduced Python bugs and oddities
//!
//! Verified against the frozen corpus; all preserved verbatim.
//!
//! 1. **The tables mix two visually identical characters.** Chechen uses
//!    palochka `Ӏ` (U+04C0), but some entries carry Ukrainian `І` (U+0406)
//!    instead. Both survive into output and are observable: `to_cardinal(100)`
//!    == "бӀе" (U+04C0) while `to_ordinal(100)` == "бІолгІа" (U+0406). Same
//!    split for 40/60/80 `ORD`, 1000 `ORD` ("эзарлагІа") and every `ILLIONS`
//!    `ORD`. These tables were extracted mechanically from the Python source
//!    rather than retyped, precisely because the two are indistinguishable on
//!    screen.
//! 2. **Typos in the numword tables**, kept as-is: `CARDINALS[12]["ORD"]` is
//!    "шийтталга" (no palochka, unlike every sibling "…лгӀа");
//!    `CARDINALS[8]["ORD"]` is "борхӀалӀа" (missing "г");
//!    `CARDINALS[18]["ORD"]` is "берхитталӀа" (lost both the stem palochka and
//!    the "г"); `CARDINALS[19]["ORD"]` is "ткъаесналгӀа" though its `abs` is
//!    "ткъайесна" (dropped "й"); `CARDINALS[5]["instr"]` is "нхеанца" (should
//!    be "пхеанца" — "н" for "п"); `CARDINALS[1]["all"]` is "цхаьнга" (missing
//!    "ь"). The first four are reachable from `to_ordinal`; the last two need
//!    a `case=` kwarg the Rust path never receives.
//! 3. **`to_ordinal` silently degrades to a cardinal at ≥ 10^6.** The illions
//!    branch renders every group with `case="attr"` and only forwards `case`
//!    to the sub-10^6 remainder, so `to_ordinal(10**6)` == "цхьа миллион" —
//!    identical to `to_cardinal(10**6)`, with the ordinal suffix nowhere. The
//!    `ORD` forms in `ILLIONS` are dead code, never read by any branch.
//! 4. **`CARDINALS[1000000]` is unreachable.** The `number < 1000000` branch
//!    excludes it and 10^6 falls into the illions branch, which reads
//!    `ILLIONS[6]` instead. Kept in the table for fidelity; never consulted.
//! 5. **Zero has no `ORD` entry**, so `to_ordinal(0)` falls through `makecase`
//!    to the suffix path: "ноль" ends in "ь" (a consonant), giving
//!    "ноль" + "алгӀа" == "нольалгӀа".
//! 6. **`to_ordinal` accepts negatives** — it never calls `verify_ordinal`, so
//!    `to_ordinal(-1)` == "минус цхьалгӀа". Only `to_ordinal_num` guards, and
//!    it raises `TypeError` (see [`verify_ordinal`]).
//! 7. `CARDINALS[1000][tcase]` in the thousands branch is a **direct dict
//!    access, not a `makecase` call**, so it cannot synthesise a missing case
//!    and would raise `KeyError` instead. Every case key happens to be present,
//!    so this never fires — but the shape is preserved.
//! 8. The module-level `MINUS` constant is shadowed by `setup`'s
//!    `self.negword`. `DECIMALPOINT` is *not* shadowed: the float branch joins
//!    on the module constant "а" and never reads `self.pointword`, so the
//!    "запятая" that `setup` assigns is dead in CE. See [`DECIMALPOINT`].
//! 9. **The float branch drops both keyword arguments.** It recurses as
//!    `self.to_cardinal(int(abs_number))` and `self.to_cardinal(int(c))` — no
//!    `clazz=`, no `case=` — so a float always renders at the defaults even
//!    when the caller asked for another class or case. `to_ordinal(1.5)` is
//!    therefore a plain cardinal, and `to_currency`'s fractional-cents branch
//!    passes a `case=` that is silently discarded.
//! 10. **`precision=` is inert.** `Num2Word_Base` carries `self.precision`, and
//!    the dispatcher's `precision=` kwarg (issue #580) assigns it — but CE's
//!    float branch slices `str(abs_number)` instead of calling
//!    `float2tuple`, so it never reads the attribute. Verified live:
//!    `num2words(2.675, lang="ce", precision=1)` == `num2words(2.675,
//!    lang="ce")`. Hence [`LangCe::to_cardinal_float`] ignores its override.
//!
//! # Error variants
//!
//! `to_ordinal_num` on a negative raises `TypeError` → [`N2WError::Type`],
//! carrying `Num2Word_Base.errmsg_negord` verbatim. The `KeyError` paths
//! ([`N2WError::Key`]) model Python's dict lookups faithfully. Under the
//! default `clazz`/`case` they are unreachable from *integer* input, but the
//! Decimal arm of the float path reaches them constantly — every `cardinal_dec`
//! row in the corpus is a `KeyError` (see [`LangCe::cardinal_decimal`]).

use std::sync::OnceLock;
use crate::base::{check_maxval, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// The noun-class marker embedded in the tables; `.replace`d with `clazz`.
const MASK: &str = "д*";

/// Python's `to_cardinal(number, clazz="д", ...)` default: class "д".
/// The dispatcher never passes `clazz=`, so this is always the value used.
const DEFAULT_CLAZZ: &str = "д";

/// `self.negword`, set in `Num2Word_CE.setup`. Note the override concatenates
/// `self.negword + " "` directly rather than `.strip()`-ing it as
/// `Num2Word_Base.to_cardinal` does; identical here since it has no padding.
const NEGWORD: &str = "минус";

/// `self.pointword`, set in `Num2Word_CE.setup` — and **dead**.
///
/// `Num2Word_Base.to_cardinal_float` would read it, but CE overrides
/// `to_cardinal` and handles floats inline, joining on the module-level
/// [`DECIMALPOINT`] instead. Nothing in CE ever reads `self.pointword`; it is
/// surfaced through [`Lang::pointword`] because that hook reports what
/// `setup()` assigns, not what the language happens to use.
const POINTWORD: &str = "запятая";

/// `lang_CE.py`'s module-level `DECIMALPOINT`, the word the float branch
/// actually joins on.
///
/// The module defines it as a commented-out "запятая" overridden by `"а"`:
///
/// ```python
/// # DECIMALPOINT = "запятая"  # check !
/// DECIMALPOINT = "а"
/// ```
///
/// so CE says "two *and* six seven five", not "two comma six seven five". This
/// is why CE cannot inherit `floatpath::default_to_cardinal_float`, which emits
/// `lang.title(lang.pointword())` — for CE that would print [`POINTWORD`].
const DECIMALPOINT: &str = "а";

// ---------------------------------------------------------------------------
// Tables, extracted mechanically from lang_CE.py (see module docs, bug 1).
// Case order within each entry mirrors the Python dict's insertion order.
// ---------------------------------------------------------------------------

const CARDINALS: &[(u64, &[(&str, &str)])] = &[
    (0, &[("attr", "ноль"), ("abs", "ноль"), ("gen", "нолан"), ("dat", "нолана"), ("erg", "ноло"), ("instr", "ноланца"), ("mat", "ноланах"), ("comp", "ноланал"), ("all", "ноланга")]),
    (1, &[("attr", "цхьа"), ("obl", "цхьана"), ("abs", "цхьаъ"), ("gen", "цхьаннан"), ("dat", "цхьанна"), ("erg", "цхьамма"), ("instr", "цхьаьнца"), ("mat", "цхьаннах"), ("comp", "цхьаннал"), ("all", "цхаьнга"), ("ORD", "цхьалгӀа")]),
    (2, &[("attr", "ши"), ("obl", "шина"), ("abs", "шиъ"), ("gen", "шиннан"), ("dat", "шинна"), ("erg", "шимма"), ("instr", "шинца"), ("mat", "шиннах"), ("comp", "шиннал"), ("all", "шинга"), ("ORD", "шолгӀа")]),
    (3, &[("attr", "кхо"), ("obl", "кхона"), ("abs", "кхоъ"), ("gen", "кхааннан"), ("dat", "кхаанна"), ("erg", "кхаамма"), ("instr", "кхаанца"), ("mat", "кхааннах"), ("comp", "кхааннал"), ("all", "кхаанга"), ("ORD", "кхоалгӀа")]),
    (4, &[("attr", "д*и"), ("obl", "д*еа"), ("abs", "д*иъ"), ("gen", "д*еаннан"), ("dat", "д*еанна"), ("erg", "д*еамма"), ("instr", "д*еанца"), ("mat", "д*еаннах"), ("comp", "д*еаннал"), ("all", "д*еанга"), ("ORD", "д*оьалгӀа")]),
    (5, &[("attr", "пхи"), ("obl", "пхеа"), ("abs", "пхиъ"), ("gen", "пхеаннан"), ("dat", "пхеанна"), ("erg", "пхеамма"), ("instr", "нхеанца"), ("mat", "пхеаннах"), ("comp", "пхеаннал"), ("all", "пхеанга"), ("ORD", "пхоьалгӀа")]),
    (6, &[("abs", "ялх"), ("attr", "ялх"), ("ORD", "йолхалгӀа")]),
    (7, &[("abs", "ворхӀ"), ("attr", "ворхӀ"), ("ORD", "ворхӀалгӀа")]),
    (8, &[("abs", "бархӀ"), ("attr", "бархӀ"), ("ORD", "борхӀалӀа")]),
    (9, &[("abs", "исс"), ("attr", "исс"), ("ORD", "уьссалгӀа")]),
    (10, &[("attr", "итт"), ("abs", "итт"), ("gen", "иттаннан"), ("dat", "иттанна"), ("erg", "иттамма"), ("instr", "иттанца"), ("mat", "иттаннах"), ("comp", "иттаннал"), ("all", "иттанга"), ("ORD", "уьтталгӀа")]),
    (11, &[("abs", "цхьайтта"), ("attr", "цхьайтта"), ("ORD", "цхьайтталгӀа")]),
    (12, &[("abs", "шийтта"), ("attr", "шийтта"), ("ORD", "шийтталга")]),
    (13, &[("abs", "кхойтта"), ("attr", "кхойтта"), ("ORD", "кхойтталгӀа")]),
    (14, &[("abs", "д*ейтта"), ("attr", "д*ейтта"), ("ORD", "д*ейтталгӀа")]),
    (15, &[("abs", "пхийтта"), ("attr", "пхийтта"), ("ORD", "пхийтталгӀа")]),
    (16, &[("abs", "ялхитта"), ("attr", "ялхитта"), ("ORD", "ялхитталгӀа")]),
    (17, &[("abs", "вуьрхӀитта"), ("attr", "вуьрхӀитта"), ("ORD", "вуьрхӀитталгӀа")]),
    (18, &[("abs", "берхӀитта"), ("attr", "берхӀитта"), ("ORD", "берхитталӀа")]),
    (19, &[("abs", "ткъайесна"), ("attr", "ткъайесна"), ("ORD", "ткъаесналгӀа")]),
    (20, &[("abs", "ткъа"), ("gen", "ткъаннан"), ("dat", "ткъанна"), ("erg", "ткъамма"), ("instr", "ткъанца"), ("mat", "ткъаннах"), ("comp", "ткъаннал"), ("all", "ткъанга"), ("attr", "ткъе"), ("ORD", "ткъолгӀа")]),
    (40, &[("abs", "шовзткъа"), ("attr", "шовзткъе"), ("ORD", "шовзткъалгІа")]),
    (60, &[("abs", "кхузткъа"), ("attr", "кхузткъе"), ("ORD", "кхузткъалгІа")]),
    (80, &[("abs", "дезткъа"), ("attr", "дезткъе"), ("ORD", "дезткъалгІа")]),
    (100, &[("attr", "бӀе"), ("abs", "бӀе"), ("obl", "бӀен"), ("gen", "бӀеннан"), ("dat", "бӀенна"), ("erg", "бӀемма"), ("instr", "бӀенца"), ("mat", "бӀеннах"), ("comp", "бӀеннал"), ("all", "бӀенга"), ("ORD", "бІолгІа")]),
    (1000, &[("attr", "эзар"), ("abs", "эзар"), ("obl", "эзаран"), ("gen", "эзарнан"), ("dat", "эзарна"), ("erg", "эзарно"), ("instr", "эзарнаца"), ("mat", "эзарнах"), ("comp", "эзарнал"), ("all", "эзаранга"), ("ORD", "эзарлагІа")]),
    // Dead entry: unreachable (module docs, bug 4). Retained for fidelity.
    (1000000, &[("attr", "миллион"), ("abs", "миллион"), ("ORD", "миллионалгІа")]),
];

/// `CARDINALS["casesuffix_cons"]` — glued onto `abs` forms ending in a consonant.
const CASESUFFIX_CONS: &[(&str, &str)] = &[
    ("gen", "аннан"),
    ("dat", "анна"),
    ("erg", "амма"),
    ("instr", "анца"),
    ("mat", "аннах"),
    ("comp", "аннал"),
    ("all", "анга"),
    ("obl", "ан"),
    ("ORD", "алгӀа"),
];

/// `CARDINALS["casesuffix_voc"]` — glued onto `abs` forms ending in a vowel.
const CASESUFFIX_VOC: &[(&str, &str)] = &[
    ("gen", "ннан"),
    ("dat", "нна"),
    ("erg", "мма"),
    ("instr", "нца"),
    ("mat", "ннах"),
    ("comp", "ннал"),
    ("all", "нга"),
    ("obl", "н"),
    ("ORD", "лгӀа"),
];

/// `ILLIONS`, `"attr"` slot only — the sole slot any branch reads (bug 3).
/// Keys are the powers of ten the illions loop walks, ascending.
const ILLIONS: &[(u32, &str)] = &[
    (6, "миллион"),
    (9, "миллиард"),
    (12, "биллион"),
    (15, "биллиард"),
    (18, "триллион"),
    (21, "триллиард"),
    (24, "квадриллион"),
    (27, "квадриллиард"),
    (30, "квинтиллион"),
    (33, "квинтиллиард"),
];

/// `reversed([6, 9, 12, 15, 18, 21, 24, 27, 30, 33])` — descending, as Python
/// iterates it. Together with `number % 10**6` these cover every digit group
/// of a value in `[10**6, 10**34)` with no gap.
const POTS_DESC: [u32; 10] = [33, 30, 27, 24, 21, 18, 15, 12, 9, 6];

// ---------------------------------------------------------------------------
// Currency tables and constants.
// ---------------------------------------------------------------------------

/// The `mcase` `_money_verbose`/`_cents_verbose` resolve to.
///
/// Both read `mcase = "attr"; if case != "abs": mcase = "obl"`. The bare
/// `to_currency` trait hook has no `case=` parameter, so it runs at the
/// Python default `"abs"` and resolves to `"attr"`; `to_currency_kw` receives
/// the caller's `case=` and any non-"abs" string — including "bogus", which
/// Python happily maps to "obl" too — selects the oblique forms.
fn money_case(case: &str) -> &'static str {
    if case != "abs" {
        "obl"
    } else {
        "attr"
    }
}

/// Python 3.9's message for calling a 3-parameter method with 2 arguments.
///
/// 3.10+ prefixes the qualified name (`Num2Word_CE._money_verbose() ...`); the
/// reference interpreter here is 3.9.6, which emits the bare name. The corpus
/// records only the exception *type*, so the text is documentation rather than
/// a matched assertion — but it is the real string, copied from the raise.
const MONEY_VERBOSE_ARITY_ERR: &str =
    "_money_verbose() missing 1 required positional argument: 'case'";

/// `Num2Word_CE.CURRENCY_FORMS` — CE's **own** class attribute.
///
/// This is the one language where the `lang_EUR.py` trap does *not* apply.
/// `Num2Word_EN.__init__` mutates `Num2Word_EUR.CURRENCY_FORMS` **in place**,
/// and the 16 classes that never declare their own dict silently inherit the
/// English rewrite. CE declares one, so attribute lookup stops at
/// `Num2Word_CE` and the EUR dict is never consulted. Verified live rather
/// than assumed:
///
/// ```text
/// >>> CONVERTER_CLASSES['ce'].CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS
/// False
/// ```
///
/// Four codes, and only four. Everything EN grafts onto the shared dict — JPY,
/// KWD, BHD, INR, CNY, CHF and ~19 others — is invisible from here, which is
/// why every one of those corpus rows is a NotImplementedError. Note the
/// consequence for the two currencies the contract calls out: CE reaches
/// neither the 3-decimal (KWD/BHD, divisor 1000) nor the 0-decimal (JPY,
/// divisor 1) path, because it cannot name those currencies at all.
///
/// Both forms of each pair are kept even though `to_currency` only ever reads
/// `[0]` (see [`LangCe::to_currency`]): `to_cheque` takes `cr1[-1]`, and the
/// tuple arity is ported data regardless.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    [
        ("EUR", CurrencyForms::new(&["Евро", "Евро"], &["Сент", "Сенташ"])),
        ("RUB", CurrencyForms::new(&["Сом", "Сомаш"], &["Кепек", "Кепекаш"])),
        ("USD", CurrencyForms::new(&["Доллар", "Доллараш"], &["Сент", "Сенташ"])),
        ("GBP", CurrencyForms::new(&["Фунт", "Фунташ"], &["Пенни", "Пенни"])),
    ]
    .into_iter()
    .collect()
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged and **never read**.
///
/// CE declares no adjectives of its own, so it sees EUR's 16 English ones
/// (confirmed live) — but [`LangCe::to_currency`] accepts `adjective` and
/// never consults it, and `to_cheque` has no adjective step. So this table is
/// unreachable through every path CE exposes:
///
/// ```text
/// >>> c.to_currency(12.34, "USD", adjective=True)
/// 'шийтта Доллар, ткъе дейтта Сент'   # not "US Доллар"
/// ```
///
/// Ported anyway because it is data the class genuinely carries, and dropping
/// it would misrepresent CE as having an empty table like `Num2Word_Base`.
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Australian"),
        ("BYN", "Belarusian"),
        ("CAD", "Canadian"),
        ("EEK", "Estonian"),
        ("USD", "US"),
        ("RUB", "Russian"),
        ("NOK", "Norwegian"),
        ("MXN", "Mexican"),
        ("RON", "Romanian"),
        ("INR", "Indian"),
        ("HUF", "Hungarian"),
        ("ISK", "íslenskar"),
        ("UZS", "Uzbekistan"),
        ("SAR", "Saudi"),
        ("JPY", "Japanese"),
        ("KRW", "Korean"),
    ]
    .into_iter()
    .collect()
}

// ---------------------------------------------------------------------------
// Error constructors, mirroring the Python exception each site would raise.
// ---------------------------------------------------------------------------

/// Python `KeyError`: a missing dict key. Python renders the key with `repr`.
fn key_error(key: impl std::fmt::Display) -> N2WError {
    N2WError::Key(format!("'{}'", key))
}

/// The vowel a stem must end in to take the `casesuffix_voc` table.
///
/// Python writes `CARDINALS[number]["abs"][-1] in "а"`, a *substring* test
/// against a one-character string — which for a single character is just
/// equality with "а" (U+0430). Indexing is by character, never by byte.
const VOWEL_A: char = 'а';

fn cardinals_entry(n: u64) -> Result<&'static [(&'static str, &'static str)]> {
    CARDINALS
        .iter()
        .find(|(k, _)| *k == n)
        .map(|(_, v)| *v)
        .ok_or_else(|| key_error(n))
}

fn case_of(entry: &[(&'static str, &'static str)], case: &str) -> Option<&'static str> {
    entry.iter().find(|(c, _)| *c == case).map(|(_, w)| *w)
}

/// `CARDINALS[n][case]` — a direct dict access, raising `KeyError` when absent.
fn cardinals_at(n: u64, case: &str) -> Result<&'static str> {
    case_of(cardinals_entry(n)?, case).ok_or_else(|| key_error(case))
}

/// Python's `Num2Word_CE.makecase`.
///
/// Stored form if the entry has this case; otherwise `abs` + a case suffix,
/// picking the suffix table by whether `abs` ends in "а".
fn makecase(number: u64, case: &str, clazz: &str) -> Result<String> {
    let entry = cardinals_entry(number)?;

    if let Some(word) = case_of(entry, case) {
        return Ok(word.replace(MASK, clazz));
    }

    // Python: CARDINALS[number]["abs"] — KeyError if the entry lacks "abs".
    let abs_form = case_of(entry, "abs").ok_or_else(|| key_error("abs"))?;

    // `abs_form[-1] in "а"`. Every table entry is a non-empty literal, so the
    // `[-1]` cannot IndexError; `None` is unreachable and falls to the
    // consonant table, which is what an empty stem would mean anyway.
    let is_vowel_stem = abs_form.chars().next_back() == Some(VOWEL_A);
    let table = if is_vowel_stem {
        CASESUFFIX_VOC
    } else {
        CASESUFFIX_CONS
    };

    let suffix = table
        .iter()
        .find(|(c, _)| *c == case)
        .map(|(_, w)| *w)
        .ok_or_else(|| key_error(case))?;

    Ok(abs_form.replace(MASK, clazz) + suffix)
}

/// The `number < 1000000` half of `to_cardinal`, for a non-negative value.
///
/// Split out from the BigInt entry point because every branch below 10^6 does
/// small-integer arithmetic. The caller proves `number < 1_000_000` before
/// calling, so `u64` is sound here (no fixed-width cast happens above that).
/// Branch order matches Python exactly.
fn to_cardinal_small(number: u64, clazz: &str, case: &str) -> Result<String> {
    // elif number < 20
    if number < 20 {
        return makecase(number, case, clazz);
    }

    // elif number < 100 — Chechen counts in twenties (vigesimal).
    if number < 100 {
        let twens = number / 20;
        let units = number % 20;
        let base = twens * 20;
        if units == 0 {
            return makecase(number, case, clazz);
        }
        let twenties = makecase(base, "attr", clazz)?;
        let rest = to_cardinal_small(units, clazz, case)?;
        // The trailing `.replace(MASK, clazz)` is redundant — `makecase`
        // already substituted the class — but it is what Python writes.
        return Ok(twenties + " " + &rest.replace(MASK, clazz));
    }

    // elif number < 1000
    if number < 1000 {
        let hundreds = number / 100;
        let tens = number % 100;
        let hundert = if hundreds > 1 {
            cardinals_at(hundreds, "attr")?.replace(MASK, clazz) + " "
        } else {
            String::new()
        };
        if tens != 0 {
            let rest = to_cardinal_small(tens, clazz, case)?;
            // Note: the hundred word is pinned to "abs" here regardless of
            // `case`; only the remainder carries the case.
            return Ok(hundert + cardinals_at(100, "abs")? + " " + &rest);
        }
        return Ok(hundert + &makecase(100, case, clazz)?);
    }

    // elif number < 1000000
    let thousands = number / 1000;
    let hundert = number % 1000;
    let tcase = if hundert > 0 { "attr" } else { case };

    let tausend = if thousands > 1 {
        let head = to_cardinal_small(thousands, clazz, "attr")?;
        // Direct dict access, not makecase (module docs, bug 7).
        head + " " + cardinals_at(1000, tcase)?
    } else {
        makecase(1000, tcase, clazz)?
    };

    let rest = if hundert != 0 {
        format!(" {}", to_cardinal_small(hundert, clazz, case)?)
    } else {
        String::new()
    };

    Ok(tausend + &rest)
}

fn pow10(n: u32) -> BigInt {
    BigInt::from(10).pow(n)
}

// ---------------------------------------------------------------------------
// The float path's one hard dependency: `repr(float)`.
// ---------------------------------------------------------------------------

/// CPython's `repr(float)` for a **non-negative, finite** `f64`.
///
/// # Why this exists at all
///
/// `floatpath.rs` deliberately refuses to reimplement `repr(float)` and takes
/// `precision` from the Python side instead, because `base.float2tuple` only
/// ever needs the *digit count*. CE is different in kind: its float branch is
///
/// ```python
/// float_part = str(abs_number).split(".")[1]
/// ```
///
/// — it slices the repr **string**. There is no digit count to be handed; the
/// characters themselves are the input, and the two ways the slice can fail
/// (no `"."` at all → `IndexError`; an `"e"` after the `"."` → `ValueError`)
/// are reachable, observable behaviour. So the repr has to be reconstructed.
///
/// # How it is kept honest
///
/// Two independent facts are extracted, each from the tool that gets it right:
///
/// * **Shape** — `{:e}` gives the shortest round-trip digits and the decimal
///   exponent. From those, `decpt` (the decimal-point position) and the
///   significant-digit count, which drive CPython's fixed-vs-exponent choice in
///   `pystrtod.c:format_float_short`, format code `'r'`:
///
///   ```c
///   /* convert to exponential format at 1e16 */
///   if (decpt <= -4 || decpt > 16)
///       use_exp = 1;
///   ```
///
/// * **Digits** — the fixed branch re-renders with `{:.prec$}`, *not* with the
///   `{:e}` digits. This is not a stylistic choice. Rust's shortest-repr and
///   CPython's `_Py_dg_dtoa` mode 0 disagree on **exact ties**: the double
///   nearest `1658206780088562.25` sits equidistant between the 17-digit
///   candidates `…562.2` and `…562.3`, and CPython emits `.2` where Rust's
///   `{}`/`{:e}` emit `.3`. `{:.1}` is a different code path (`format_exact`)
///   that rounds the *exact* binary value half-to-even, and agrees with CPython.
///   Measured, not assumed: over ~300k doubles — random bit patterns,
///   subnormals, `f64::MAX`, and the tie-prone `n/8 + 1e14` / `n/4 + 1e15`
///   families that expose the disagreement — this function matches `repr(v)`
///   byte for byte, while the naive `{:e}` reconstruction misses 65 of them.
///
/// A tie can only move the *last* digit, never `decpt` or the digit count, so
/// taking the shape from `{:e}` is safe even though its digits are not.
///
/// `precision` is derived here rather than taken from `FloatValue`, matching
/// Python: CE reads `str(abs_number)`, never `self.precision` (module docs,
/// bug 10).
fn python_repr(x: f64) -> String {
    // CPython special-cases zero; `Py_DTSF_ADD_DOT_0` makes it "0.0".
    if x == 0.0 {
        return "0.0".to_string();
    }

    // "2.675e0", "1e21", "5e-1" — mantissa, then the decimal exponent.
    let sci = format!("{:e}", x);
    let (mant, exp_str) = match sci.split_once('e') {
        Some(parts) => parts,
        // Unreachable: `LowerExp` always emits `e<exp>` for finite input, and
        // callers reject NaN/inf before getting here (`int()` raises on those
        // first). Degrade to the raw form rather than panicking.
        None => return sci.clone(),
    };
    let exp: i32 = exp_str.parse().unwrap_or(0);
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let ndigits = digits.chars().count() as i32;

    // `{:e}` normalises to one digit before the point, so its exponent is
    // `decpt - 1`.
    let decpt = exp + 1;

    if decpt <= -4 || decpt > 16 {
        // Exponential form: `d[.ddd]e±XX`. The dot appears only when there is
        // more than one significant digit — which is exactly what decides
        // IndexError vs ValueError upstream — and the exponent is signed and
        // zero-padded to at least two places.
        let e = decpt - 1;
        let mut it = digits.chars();
        let mut s = String::new();
        if let Some(first) = it.next() {
            s.push(first);
        }
        let rest: String = it.collect();
        if !rest.is_empty() {
            s.push('.');
            s.push_str(&rest);
        }
        s.push('e');
        s.push(if e < 0 { '-' } else { '+' });
        s.push_str(&format!("{:02}", e.abs()));
        return s;
    }

    // Fixed form. The count of fractional digits the shortest repr carries is
    // `ndigits - decpt`, floored at 1 because CPython always appends ".0" to a
    // whole float (`Py_DTSF_ADD_DOT_0`) — which is also why the fixed branch
    // always contains a ".", and so never raises IndexError.
    let prec = std::cmp::max(ndigits - decpt, 1) as usize;
    format!("{:.*}", prec, x)
}

// ---------------------------------------------------------------------------
// Decimal arithmetic, for the arm where `isinstance(number, float)` is False.
// ---------------------------------------------------------------------------

/// `10**n` for an exponent taken from a parsed decimal's scale.
///
/// The scale originates in the decimal literal the caller parsed, so it is
/// bounded by that literal's length; a scale that does not fit `u32` would
/// describe a number with more than four billion digits, which cannot have been
/// allocated in the first place.
fn pow10_scale(n: i64) -> BigInt {
    pow10(u32::try_from(n).unwrap_or(0))
}

/// Python's `Decimal.__floordiv__` against a positive integer, for `a >= 0`.
///
/// `Decimal`'s `//` truncates toward zero; every caller has already taken
/// `abs`, so that is also the floor.
///
/// Deliberately *not* `a / b` followed by a truncation: bigdecimal's `Div`
/// rounds at a fixed 100-significant-digit ceiling, which would silently drift
/// on a long input. `a` is exactly `coeff * 10**-scale`, so one exact `BigInt`
/// division of `coeff` by `b * 10**scale` gives the quotient with no rounding
/// step to get wrong.
fn dec_floordiv(a: &BigDecimal, b: &BigInt) -> BigDecimal {
    let (coeff, scale) = a.as_bigint_and_exponent();
    let (num, den) = if scale >= 0 {
        (coeff, b * pow10_scale(scale))
    } else {
        (coeff * pow10_scale(-scale), b.clone())
    };
    // Both operands are non-negative here, so BigInt's truncating division is
    // the floor division Python performs.
    BigDecimal::from(num / den)
}

/// Python's `Decimal.__mod__` against a positive integer, for `a >= 0`:
/// `a - b * (a // b)`.
///
/// Python performs `%` as one operation and `_fix`es the result, so the
/// remainder is rounded to 28 significant digits — not just returned exact.
/// That rounding is load-bearing at the top of the illions range: the exact
/// remainder of a 30-digit input can carry 29+ significant digits, and Python
/// drops the last one before the recursion subscripts it, changing *which*
/// `Decimal` the eventual `KeyError` names. Measured, not assumed:
///
/// ```text
/// >>> Decimal("369019905581504987656260366567E-26") % 1000
/// Decimal("690.1990558150498765626036657")   # 28 sig, exact would be 29
/// ```
fn dec_rem(a: &BigDecimal, b: &BigInt) -> BigDecimal {
    let exact = a - BigDecimal::from(b.clone()) * dec_floordiv(a, b);
    dec_fix(&exact)
}

/// `decimal.getcontext().prec` — the default context's significant-digit limit.
const DECIMAL_CONTEXT_PREC: usize = 28;

/// Round `n / 10**drop` half-to-even, for `n >= 0`. The rounding `_fix` applies
/// under the default context, whose `rounding` is `ROUND_HALF_EVEN`.
fn round_half_even(n: &BigInt, drop: u32) -> BigInt {
    let pow = pow10(drop);
    let (q, r) = n.div_rem(&pow);
    let twice: BigInt = &r * 2;
    match twice.cmp(&pow) {
        std::cmp::Ordering::Less => q,
        std::cmp::Ordering::Greater => q + 1,
        // Exact half: keep the even quotient.
        std::cmp::Ordering::Equal => {
            if q.is_even() {
                q
            } else {
                q + 1
            }
        }
    }
}

/// The coefficient-rounding half of Python's `Decimal._fix` under the default
/// context: round a non-negative value to at most 28 significant digits,
/// half-to-even.
///
/// Every arithmetic operation in Python's `decimal` runs its result through
/// `_fix`, so this is applied wherever CE does Decimal arithmetic that can grow
/// past 28 digits — [`dec_rem`] and [`dec_abs`]. `_fix`'s other job, clamping
/// the exponent to `Etiny`/`Etop`, cannot fire here: the default context's
/// exponent range is ±999999 and CE only reaches this for `number < 10**34`.
///
/// The inputs are always non-negative (callers `abs` first), so the sign is not
/// modelled.
fn dec_fix(v: &BigDecimal) -> BigDecimal {
    let (coeff, scale) = v.as_bigint_and_exponent();
    // `Decimal("0")` has one significant digit by Python's count; a bare zero
    // coefficient needs no rounding regardless.
    let ndigits = coeff.to_string().chars().count();
    if ndigits <= DECIMAL_CONTEXT_PREC {
        return BigDecimal::new(coeff, scale);
    }

    let drop = (ndigits - DECIMAL_CONTEXT_PREC) as u32;
    let rounded = round_half_even(&coeff, drop);
    let new_scale = scale - drop as i64;

    // `_fix`: a carry can widen the coefficient past prec (28 nines round up to
    // 10**28). Python drops the now-trailing zero and bumps the exponent —
    //     if len(self._int) > context.prec:
    //         self_int = self_int[:-1]
    //         exp += 1
    // — which is exact, since the carry guarantees that last digit is a zero.
    if rounded.to_string().chars().count() > DECIMAL_CONTEXT_PREC {
        return BigDecimal::new(rounded / 10, new_scale - 1);
    }
    BigDecimal::new(rounded, new_scale)
}

/// Python's `abs(Decimal)` — **which is not a sign flip**.
///
/// `Decimal.__abs__` does not return `copy_abs()`; it routes through
/// `__neg__`/`__pos__`, and both end in `_fix(context)`:
///
/// ```python
/// def __abs__(self, round=True, context=None):
///     if not round:
///         return self.copy_abs()
///     ...
///     if self._sign:
///         ans = self.__neg__(context=context)
///     else:
///         ans = self.__pos__(context=context)
///     return ans
/// ```
///
/// So `abs()` **rounds to the context's 28 significant digits**, and a Decimal
/// wider than that silently loses its tail:
///
/// ```text
/// >>> abs(Decimal("-8938465034294554016749591177719.10"))
/// Decimal("8.938465034294554016749591178E+30")     # the ".10" is gone
/// ```
///
/// `Num2Word_CE.to_cardinal` reaches this on its very first branch
/// (`elif number < 0: ... self.to_cardinal(abs(number), ...)`), which makes the
/// sign observable in the *output*, not just the minus word: a 33-digit
/// negative renders as a clean integer sentence, while the identical positive
/// keeps its fraction and dies on a `KeyError`. Reproduced rather than
/// simplified — `copy_abs()` here would be the tidy answer and the wrong one.
fn dec_abs(v: &BigDecimal) -> BigDecimal {
    dec_fix(&v.abs())
}

/// `repr(Decimal)`, i.e. `"Decimal('%s')" % str(self)`.
///
/// Python puts the `Decimal` object itself in the `KeyError`, and `str(exc)` is
/// `repr(args[0])` — so `CARDINALS[Decimal("9.99")]` surfaces as the message
/// `Decimal('9.99')`, quotes and all.
///
/// `str(Decimal)` is the spec's to-scientific-string conversion, ported from
/// `_pydecimal.Decimal.__str__`: fixed notation only when the exponent is
/// non-positive *and* the value has not shrunk past 1e-6, else scientific with
/// one digit ahead of the point. `context.capitals` defaults to 1, hence the
/// upper-case `E`. Trailing zeros are significant and survive
/// (`Decimal("1.10")` prints "1.10"), which `BigDecimal`'s scale preserves.
fn decimal_repr(v: &BigDecimal) -> String {
    let (coeff, scale) = v.as_bigint_and_exponent();
    let sign = if coeff.is_negative() { "-" } else { "" };
    // Decimal's `_int` (the coefficient digits) and `_exp`.
    let int_digits = coeff.abs().to_string();
    let ndigits = int_digits.chars().count() as i64;
    let exp = -scale;
    let leftdigits = exp + ndigits;

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
    } else if dotplace >= ndigits {
        (
            format!(
                "{}{}",
                int_digits,
                "0".repeat((dotplace - ndigits) as usize)
            ),
            String::new(),
        )
    } else {
        let head: String = int_digits.chars().take(dotplace as usize).collect();
        let tail: String = int_digits.chars().skip(dotplace as usize).collect();
        (head, format!(".{}", tail))
    };

    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        let e = leftdigits - dotplace;
        format!("E{}{}", if e < 0 { "-" } else { "+" }, e.abs())
    };

    format!("Decimal('{}{}{}{}')", sign, intpart, fracpart, exp_part)
}

/// `CARDINALS[number]` where `number` is a `Decimal` — resolved to the `int`
/// key Python's dict lookup would land on.
///
/// The tables are keyed by `int`, but Python hashes the `Decimal`: for an
/// integral value `hash(Decimal("80.00")) == hash(80)` and the two compare
/// equal, so the lookup *succeeds*. A non-integral `Decimal` matches no key and
/// raises `KeyError` naming its `repr` — which is how every `cardinal_dec` row
/// in the corpus ends up.
fn cardinals_key_dec(n: &BigDecimal) -> Result<u64> {
    CARDINALS
        .iter()
        .map(|(k, _)| *k)
        // BigDecimal's PartialEq aligns scales before comparing, so this is the
        // numeric equality Python's dict relies on, not a representation test.
        .find(|k| BigDecimal::from(*k) == *n)
        .ok_or_else(|| N2WError::Key(decimal_repr(n)))
}

/// `self.makecase(number, case, clazz)` with a `Decimal` `number`.
///
/// Resolving the key first and delegating to the integer [`makecase`] is the
/// same lookup Python performs, and keeps the suffix logic in one place.
fn makecase_dec(number: &BigDecimal, case: &str, clazz: &str) -> Result<String> {
    makecase(cardinals_key_dec(number)?, case, clazz)
}

/// Python's `Num2Word_Base.verify_ordinal`, integer half.
///
/// The float check (`errmsg_floatord`) cannot fire for `BigInt` input, so only
/// the negative guard survives. `to_ordinal_num` is the *only* caller — note
/// that `to_ordinal` skips it entirely (module docs, bug 6).
fn verify_ordinal(value: &BigInt) -> Result<()> {
    if value.is_negative() {
        // errmsg_negord = "Cannot treat negative num %s as ordinal."
        return Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            value
        )));
    }
    Ok(())
}

/// A string-valued kwarg with a Python-signature default.
///
/// Absent -> the default, exactly as the signature would bind it. Present as
/// a string -> that string, whatever it is — Python performs no validation up
/// front; a bad `case` only explodes later, inside a dict subscript. Present
/// as anything else (an explicit `None`, an int, ...) -> NotImplemented, so
/// the dispatcher falls back to Python and the original raise (a `TypeError`
/// from `str.replace(…, None)`, a `KeyError: None`, ...) happens there with
/// its own message.
fn kw_str<'a>(kw: &'a Kwargs, key: &str, default: &'a str) -> Result<&'a str> {
    match kw.get(key) {
        None => Ok(default),
        Some(KwVal::Str(s)) => Ok(s.as_str()),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

pub struct LangCe {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangCe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCe {
    pub fn new() -> Self {
        LangCe {
            // Built once here, never per call. `to_currency` only reads these,
            // and rebuilding a table on every call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// The `isinstance(number, float)` branch of `Num2Word_CE.to_cardinal`.
    ///
    /// ```python
    /// if isinstance(number, float):
    ///     negative = number < 0
    ///     abs_number = abs(number)
    ///     entires = self.to_cardinal(int(abs_number))
    ///     float_part = str(abs_number).split(".")[1]
    ///     postfix = " ".join(
    ///         [self.to_cardinal(int(c)) for c in float_part]
    ///     )
    ///     result = entires + " " + DECIMALPOINT + " " + postfix
    ///     if negative:
    ///         result = self.negword + " " + result
    ///     return result
    /// ```
    ///
    /// Nothing here touches `float2tuple`: no `10**precision` scaling, no
    /// round-vs-floor heuristic, no zero-padding to `precision`. CE reads the
    /// repr's digits directly, so `2.675` yields "675" because that is what
    /// `repr` says — where `base.float2tuple` would compute `674.9999999999998`
    /// and lean on its `< 0.01` rescue to get back to the same place.
    ///
    /// Neither `clazz` nor `case` is threaded into the two recursive calls
    /// (module docs, bug 9), so both parts always render at the defaults.
    ///
    /// The comment above `postfix` in the source — "Drops the trailing zero and
    /// comma" — describes nothing the code does; every fractional digit is
    /// emitted, trailing zeros included. `1.10` is not reachable as a float
    /// (`repr(1.10)` is "1.1"), which is presumably how the claim survived.
    fn cardinal_float(&self, number: f64) -> Result<String> {
        // `negative = number < 0` — False for -0.0 and for NaN, both of which
        // Python's `<` answers the same way.
        let negative = number < 0.0;
        let abs_number = number.abs();

        // `entires = self.to_cardinal(int(abs_number))` runs *before* the
        // split, so `int()`'s own failures win over the IndexError below.
        if abs_number.is_nan() {
            return Err(N2WError::Value("cannot convert float NaN to integer".into()));
        }
        if abs_number.is_infinite() {
            return Err(N2WError::Overflow(
                "cannot convert float infinity to integer".into(),
            ));
        }
        // `BigInt::from_f64` truncates toward zero and is exact — the same
        // contract as `int(float)`. It only returns None for non-finite input,
        // which the two guards above have already rejected.
        let ipart = BigInt::from_f64(abs_number).ok_or_else(|| {
            N2WError::Value(format!("cannot convert float {} to integer", abs_number))
        })?;
        let entires = self.cardinal(&ipart, DEFAULT_CLAZZ, "abs")?;

        // `float_part = str(abs_number).split(".")[1]`.
        //
        // A repr with no "." is an IndexError, and that is a live path, not a
        // theoretical one: `repr(1e16)` is "1e+16", so `num2words(1e16,
        // lang="ce")` raises. Fixed-form reprs always carry a ".", so this only
        // fires in exponential form with a single significant digit.
        let repr = python_repr(abs_number);
        let float_part = repr
            .split_once('.')
            .map(|(_, frac)| frac)
            .ok_or_else(|| N2WError::Index("list index out of range".into()))?;

        // `postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])`.
        //
        // The other exponential-form outcome lands here: `repr(1.5e21)` is
        // "1.5e+21", so `float_part` is "5e+21" and `int("e")` raises
        // ValueError on the second character — after "5" has already been
        // converted, which the eager list comprehension discards.
        let mut postfix: Vec<String> = Vec::new();
        for c in float_part.chars() {
            let digit = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    c
                ))
            })?;
            postfix.push(self.cardinal(&BigInt::from(digit), DEFAULT_CLAZZ, "abs")?);
        }

        // `entires + " " + DECIMALPOINT + " " + postfix` — the module constant,
        // not `self.pointword`.
        let result = format!("{} {} {}", entires, DECIMALPOINT, postfix.join(" "));
        if negative {
            // `self.negword + " " + result`, with no `.strip()`.
            return Ok(format!("{} {}", NEGWORD, result));
        }
        Ok(result)
    }

    /// `Num2Word_CE.to_cardinal` reached with a `Decimal`.
    ///
    /// There is no Decimal branch. `isinstance(number, float)` is False for a
    /// `Decimal`, so it falls straight through into the *integer* branches and
    /// they run on Decimal arithmetic — comparisons, `//`, `%` and all. The
    /// walk only breaks when a non-integral value reaches a `CARDINALS[...]`
    /// subscript, because an integral `Decimal` hashes and compares equal to
    /// its `int` key and a fractional one matches nothing.
    ///
    /// So the failure is a `KeyError`, and *where* it lands depends on how far
    /// the fraction is carried. Both corpus shapes are reproduced:
    ///
    /// ```text
    /// Decimal("0.01")              -> KeyError: Decimal('0.01')
    ///     `< 20` on the first hop, so the input itself is the key.
    /// Decimal("98746251323029.99") -> KeyError: Decimal('9.99')
    ///     illions -> `% 10**6` -> 323029.99 -> `% 1000` -> 29.99 ->
    ///     `% 20` -> 9.99, which is finally `< 20`. The fraction rides the
    ///     remainders all the way down and only the last hop subscripts it.
    /// ```
    ///
    /// An *integral* Decimal has no such problem and converts normally —
    /// `Decimal("5")` is "пхиъ" — which is why this cannot just raise.
    ///
    /// Python's Decimal context (28 significant digits) cannot bite inside the
    /// reachable range: the widest intermediate is `number // 10**6`, and
    /// `number < 10**34` bounds it at 28 digits exactly.
    fn cardinal_decimal(&self, number: &BigDecimal, clazz: &str, case: &str) -> Result<String> {
        // elif number < 0
        if number.is_negative() {
            // `abs(number)` — Python's, which rounds to 28 significant digits.
            // See [`dec_abs`]; `number.abs()` here would be a silent divergence.
            let inner = self.cardinal_decimal(&dec_abs(number), clazz, case)?;
            return Ok(format!("{} {}", NEGWORD, inner));
        }

        // elif number < 20
        if *number < BigDecimal::from(20u64) {
            return makecase_dec(number, case, clazz);
        }

        // elif number < 100
        if *number < BigDecimal::from(100u64) {
            let twenty = BigInt::from(20);
            let twens = dec_floordiv(number, &twenty);
            let units = dec_rem(number, &twenty);
            let base = twens * BigDecimal::from(20u64);
            if units.is_zero() {
                return makecase_dec(number, case, clazz);
            }
            let twenties = makecase_dec(&base, "attr", clazz)?;
            let rest = self.cardinal_decimal(&units, clazz, case)?;
            return Ok(twenties + " " + &rest.replace(MASK, clazz));
        }

        // elif number < 1000
        if *number < BigDecimal::from(1000u64) {
            let hundreds = dec_floordiv(number, &BigInt::from(100));
            let tens = dec_rem(number, &BigInt::from(100));
            let hundert = if hundreds > BigDecimal::one() {
                // CARDINALS[hundreds]["attr"] — `hundreds` is a Decimal, but an
                // integral one, so it resolves to its int key.
                cardinals_at(cardinals_key_dec(&hundreds)?, "attr")?.replace(MASK, clazz) + " "
            } else {
                String::new()
            };
            if !tens.is_zero() {
                let rest = self.cardinal_decimal(&tens, clazz, case)?;
                return Ok(hundert + cardinals_at(100, "abs")? + " " + &rest);
            }
            return Ok(hundert + &makecase(100, case, clazz)?);
        }

        // elif number < 1000000
        if *number < BigDecimal::from(1_000_000u64) {
            let thousands = dec_floordiv(number, &BigInt::from(1000));
            let hundert = dec_rem(number, &BigInt::from(1000));
            let tcase = if hundert > BigDecimal::zero() { "attr" } else { case };

            let tausend = if thousands > BigDecimal::one() {
                let head = self.cardinal_decimal(&thousands, clazz, "attr")?;
                // Literal 1000, so this is the integer subscript either way.
                head + " " + cardinals_at(1000, tcase)?
            } else {
                makecase(1000, tcase, clazz)?
            };

            let rest = if !hundert.is_zero() {
                format!(" {}", self.cardinal_decimal(&hundert, clazz, case)?)
            } else {
                String::new()
            };
            return Ok(tausend + &rest);
        }

        // elif number < 10**34
        if *number < BigDecimal::from(pow10(34)) {
            let mut out: Vec<String> = Vec::new();
            for &pot in POTS_DESC.iter() {
                // step = number // 10**pot % 1000 — `//` binds first.
                let step = dec_rem(&dec_floordiv(number, &pow10(pot)), &BigInt::from(1000));
                if step > BigDecimal::zero() {
                    let words = self.cardinal_decimal(&step, clazz, "attr")?;
                    let illion = ILLIONS
                        .iter()
                        .find(|(k, _)| *k == pot)
                        .map(|(_, w)| *w)
                        .ok_or_else(|| key_error(pot))?;
                    out.push(format!("{} {}", words, illion));
                }
            }
            let rest = dec_rem(number, &pow10(6));
            // `if rest:` — Decimal truthiness, so Decimal("0.00") is falsy.
            if !rest.is_zero() {
                out.push(self.cardinal_decimal(&rest, clazz, case)?);
            }
            return Ok(out.join(" "));
        }

        // Python returned the string "NOT IMPLEMENTED" here (#159).
        Err(N2WError::Overflow(format!(
            "abs({}) must be less than {}.",
            number,
            maxval_ceiling()
        )))
    }

    /// Python's `Num2Word_CE.to_cardinal(number, clazz, case)`, integer path.
    ///
    /// The `isinstance(number, float)` branch lives in [`Self::cardinal_float`];
    /// everything from `elif number < 0` down is reproduced here in order.
    fn cardinal(&self, number: &BigInt, clazz: &str, case: &str) -> Result<String> {
        // elif number < 0
        if number.is_negative() {
            let inner = self.cardinal(&number.abs(), clazz, case)?;
            return Ok(format!("{} {}", NEGWORD, inner));
        }

        // The four sub-10^6 branches. `number` is non-negative and bounded by
        // 10^6 here, so it provably fits u64.
        let million = pow10(6);
        if *number < million {
            let n = number
                .to_u64()
                .ok_or_else(|| N2WError::Value("value below 10^6 must fit u64".into()))?;
            return to_cardinal_small(n, clazz, case);
        }

        // elif number < 10**34
        if *number < pow10(34) {
            let mut out: Vec<String> = Vec::new();
            for &pot in POTS_DESC.iter() {
                // step = number // 10**pot % 1000 — `number` is positive, so
                // BigInt's truncating division agrees with Python's floor.
                let step = (number / pow10(pot)) % 1000u32;
                if step > BigInt::zero() {
                    let s = step
                        .to_u64()
                        .ok_or_else(|| N2WError::Value("step below 1000 must fit u64".into()))?;
                    let words = to_cardinal_small(s, clazz, "attr")?;
                    let illion = ILLIONS
                        .iter()
                        .find(|(k, _)| *k == pot)
                        .map(|(_, w)| *w)
                        .ok_or_else(|| key_error(pot))?;
                    out.push(format!("{} {}", words, illion));
                }
            }
            // Only the sub-10^6 remainder ever sees `case` (module docs, bug 3).
            let rest = number % &million;
            if !rest.is_zero() {
                let r = rest
                    .to_u64()
                    .ok_or_else(|| N2WError::Value("remainder below 10^6 must fit u64".into()))?;
                out.push(to_cardinal_small(r, clazz, case)?);
            }
            return Ok(out.join(" "));
        }

        // number >= 10**34. Python returned the string "NOT IMPLEMENTED"
        // here (#159); `number` is already non-negative.
        check_maxval(number, maxval_ceiling())?;
        unreachable!("number >= 10**34 is rejected by check_maxval")
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `ILLIONS` ends below
/// 10^34, so 10^34 and above raise `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10(34))
}

impl Lang for LangCe {

    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "RUB"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    /// What `setup()` assigns — reported for fidelity, never used. CE's float
    /// path joins on [`DECIMALPOINT`] instead (see [`POINTWORD`]).
    fn pointword(&self) -> &str {
        POINTWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value, DEFAULT_CLAZZ, "abs")
    }

    /// CE does not inherit `Num2Word_Base.to_cardinal_float` — it overrides
    /// `to_cardinal` and dispatches on `isinstance(number, float)` inline, so
    /// this hook routes to whichever arm Python's type test would pick.
    ///
    /// `precision_override` is discarded on purpose: the dispatcher's
    /// `precision=` kwarg assigns `self.precision`, and no CE code path reads it
    /// (module docs, bug 10). Honouring it here would invent behaviour Python
    /// does not have.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            // `precision` goes unread for the same reason: CE derives its digits
            // from `str(abs_number)`, not from `float2tuple`.
            FloatValue::Float { value, .. } => self.cardinal_float(*value),
            // A Decimal fails `isinstance(number, float)` and falls through into
            // the integer branches, running on Decimal arithmetic.
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value, DEFAULT_CLAZZ, "abs"),
        }
    }

    /// `to_cardinal(float/Decimal)`, full routing.
    ///
    /// The base default short-circuits a whole value to the integer path —
    /// which is exactly what CE does **not** do. Python's `to_cardinal` tests
    /// `isinstance(number, float)` before anything else, so `1.0` takes the
    /// float branch and renders its repr's fractional digit: "цхьаъ а ноль",
    /// never plain "цхьаъ". A whole `Decimal` conversely never sees the float
    /// branch and resolves through the integer branches' dict lookups. Both
    /// arms already live in [`Lang::to_cardinal_float`] above; this override
    /// only removes the whole-value shortcut in front of them.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)` — `to_cardinal(number, clazz, case="ORD")`.
    ///
    /// For a float the `case` dies in the float branch (module docs, bug 9),
    /// so `to_ordinal(5.0)` is the plain cardinal "пхиъ а ноль". A Decimal
    /// carries "ORD" through the integer branches, so `to_ordinal(Decimal
    /// ("5.0"))` really is "пхоьалгӀа" — same call, opposite outcome, decided
    /// solely by `isinstance`. No `verify_ordinal` anywhere (bug 6), so
    /// negatives render with the minus word and `1e16` still IndexErrors out
    /// of the repr slice.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value, .. } => self.cardinal_float(*value),
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value, DEFAULT_CLAZZ, "ORD"),
        }
    }

    /// `to_ordinal_num(float/Decimal)` — `verify_ordinal(number)`, then
    /// `str(number) + "-й"`.
    ///
    /// This is the one CE entry point that *does* call `verify_ordinal`, and
    /// Base's runs both checks on the original value:
    ///
    /// ```python
    /// if not value == int(value):
    ///     raise TypeError(self.errmsg_floatord % value)   # 0.5, Decimal("1.5")
    /// if not abs(value) == value:
    ///     raise TypeError(self.errmsg_negord % value)     # -3.0
    /// ```
    ///
    /// Both comparisons are numeric, so `-0.0` (and `Decimal("-0.0")`) pass —
    /// `abs(-0.0) == -0.0` is True — and come out as "-0.0-й", sign intact,
    /// because the suffix is glued onto `str(number)`, which is `repr_str`
    /// here. A whole huge float sails through too: `int(1e20)` is exact, so
    /// "1e+20-й" with the exponent form verbatim. `%s` renders the value the
    /// same way, hence `repr_str` in both messages.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let whole = value.as_whole_int().ok_or_else(|| {
            N2WError::Type(format!("Cannot treat float {} as ordinal.", repr_str))
        })?;
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                repr_str
            )));
        }
        Ok(format!("{}-й", repr_str))
    }

    /// `to_year(float/Decimal)` — `to_cardinal(year, case="abs")`, i.e. the
    /// exact cardinal routing: floats through the float branch (whole values
    /// included), Decimals through the integer branches.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }


    /// `converter.str_to_number` is Base's plain `Decimal(value)` — CE does
    /// not override it. The parse itself is therefore the default, but the
    /// *outcomes* for the two non-finite specials diverge from what the
    /// binding's generic Inf/NaN mapping assumes, because CE's `to_cardinal`
    /// never calls `int()`:
    ///
    /// * `Decimal("Infinity")` falls through every `<` comparison to the
    ///   literal `return "NOT IMPLEMENTED"` in Python; the port keeps the
    ///   default [`Lang::inf_result`], an OverflowError (#159).
    /// * `Decimal("NaN")` dies on the very first comparison, `number < 0`,
    ///   with `decimal.InvalidOperation` — not the ValueError from `int()`.
    ///
    /// Neither is expressible through `ParsedNumber` (a `BigDecimal` cannot
    /// hold Inf/NaN), so the parse itself is the plain default and the two
    /// specials are served natively by the [`Lang::inf_result`] default /
    /// the [`Lang::nan_result`] hook below — no Python fallback.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal("NaN")`. CE's `to_cardinal` dies on its very first branch,
    /// `number < 0`, because `Decimal("NaN") < 0` raises
    /// `decimal.InvalidOperation` — not the ValueError of `int(NaN)`. No digit
    /// is present in "NaN", so the dispatcher propagates it natively.
    fn nan_result(&self, _to: &str) -> Result<String> {
        Err(N2WError::Custom {
            module: "decimal",
            class: "InvalidOperation",
            msg: "[<class 'decimal.InvalidOperation'>]".into(),
        })
    }

    /// `Decimal("-0.0")`. `BigDecimal` cannot carry the sign, so the binding
    /// would demote it to `Float{-0.0}` and CE's float branch would render
    /// "ноль а ноль". But `Decimal("-0.0")` is *not* a float in Python: it
    /// falls through the integer branches, `number < 20` is true, and the
    /// `CARDINALS` lookup hashes it to the integer key `0` — so it renders as
    /// plain zero. cardinal/year run at `case="abs"` ("ноль"), ordinal at
    /// `case="ORD"` ("нольалгӀа"). `ordinal_num` is left to the demoted float
    /// path (returns `None`): it echoes `str(number)` == "-0.0" as "-0.0-й",
    /// which the repr-carrying float hook reproduces and this one cannot.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        let zero = BigInt::zero();
        match to {
            "cardinal" | "year" => Some(self.cardinal(&zero, DEFAULT_CLAZZ, "abs")),
            "ordinal" => Some(self.cardinal(&zero, DEFAULT_CLAZZ, "ORD")),
            _ => None,
        }
    }

    // ---- grammatical kwargs ----------------------------------------------
    //
    // The Python signatures, verbatim:
    //
    //   to_cardinal(number, clazz="д", case="abs")
    //   to_ordinal(number, clazz="д")
    //   to_ordinal_num(number)            # no extras -> trait default
    //   to_year(year, case="abs")
    //   to_currency(val, currency="RUB", cents=True, separator=",",
    //               adjective=False, case="abs")   # see to_currency_kw above
    //
    // No value validation happens at the boundary: `clazz` is spliced into
    // the tables by `.replace("д*", clazz)` (so `clazz="x"` yields "xейтта"),
    // and `case` is a dict key that only fails inside `makecase`'s suffix
    // subscript — `case="bogus"` is the corpus's KeyError. An explicit
    // `None` (or any non-string) is NotImplemented via [`kw_str`] so Python
    // raises its own error.

    /// `to_cardinal(number, clazz=..., case=...)`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["clazz", "case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let clazz = kw_str(kw, "clazz", DEFAULT_CLAZZ)?;
        let case = kw_str(kw, "case", "abs")?;
        self.cardinal(value, clazz, case)
    }

    /// `to_ordinal(number, clazz=...)` — no `case` in this signature; it is
    /// pinned to "ORD" by the forwarding call.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["clazz"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let clazz = kw_str(kw, "clazz", DEFAULT_CLAZZ)?;
        self.cardinal(value, clazz, "ORD")
    }

    /// `to_year(year, case=...)` — no `clazz` in this signature; the
    /// forwarding call leaves it at the default.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let case = kw_str(kw, "case", "abs")?;
        self.cardinal(value, DEFAULT_CLAZZ, case)
    }

    /// `to_cardinal(float/Decimal, clazz=..., case=...)`. The float branch
    /// drops both kwargs on the floor (module docs, bug 9) but *accepts*
    /// them; only a Decimal actually feels them, riding the integer branches.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["clazz", "case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let clazz = kw_str(kw, "clazz", DEFAULT_CLAZZ)?;
        let case = kw_str(kw, "case", "abs")?;
        match value {
            FloatValue::Float { value, .. } => self.cardinal_float(*value),
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value, clazz, case),
        }
    }

    /// `to_ordinal(number, clazz="д")` → `to_cardinal(number, clazz, case="ORD")`.
    /// Deliberately *not* guarded by `verify_ordinal` — negatives are accepted
    /// and rendered with the minus word (module docs, bug 6).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value, DEFAULT_CLAZZ, "ORD")
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        Ok(format!("{}-й", value))
    }

    /// `to_year(year, case="abs")` → `to_cardinal(year, case=case)`. No
    /// era/century handling at all; identical to `to_cardinal` by default.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value, DEFAULT_CLAZZ, "abs")
    }

    // ---- currency -------------------------------------------------------
    //
    // CE overrides `to_currency` wholesale and diverges from
    // `Num2Word_Base.to_currency` in three ways that are all corpus-visible:
    //
    //   1. **Integers print cents.** Base early-returns on
    //      `isinstance(val, int)` and skips the cents segment; CE has no such
    //      branch and passes `is_int_with_cents=False` instead, so `right` is
    //      a plain `0` that still renders. `to_currency(1, "EUR")` is
    //      "цхьа Евро, ноль Сент", not "цхьа Евро".
    //   2. **No `has_decimal` guard.** Base gates the cents segment behind
    //      `has_decimal or right > 0`. CE has no gate at all, so the flag is
    //      never read and `Decimal("5")` renders cents just like `5.00`.
    //   3. **`pluralize` is never called.** CE binds `devise = cr1[0]` and
    //      `centime = cr2[0]` once, up front — always the singular. So
    //      "ши Евро" (2), "дезткъе ткъайесна Сент" (99), and even the
    //      fractional branch, where Base would force the *plural* `cr2[1]`.
    //
    // `_cents_terse` is inherited from Base unchanged and CE's
    // CURRENCY_PRECISION is Base's empty dict, so `currency_precision` stays
    // at the trait default of 100 for every code and `cents_terse` at its
    // default zero-pad-to-2. Both verified live:
    //   to_currency(0.05, "USD", cents=False) == "ноль Доллар, 05 Сент"

    fn lang_name(&self) -> &str {
        "Num2Word_CE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Inherited from `Num2Word_EUR` and unreachable — see
    /// [`build_currency_adjectives`].
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Inherited, and non-default (Base's raises NotImplementedError), so it is
    /// ported — but no CE path reaches it. `to_currency` takes `cr1[0]`/`cr2[0]`
    /// directly and `to_cheque` takes `cr1[-1]`; only `default_to_currency`
    /// calls `pluralize`, and CE overrides `to_currency` wholesale.
    ///
    /// Python indexes the tuple, so a one-form entry with `n != 1` would raise
    /// IndexError. All four CE entries carry two forms, so that cannot fire —
    /// mapped to `Index` rather than a panic so the exception type survives if
    /// the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_CE._money_verbose(self, number, currency, case)` — **three**
    /// positional parameters, where `Num2Word_Base`'s (and every other
    /// language's) takes two. CE is the only class in the library that widens
    /// this signature.
    ///
    /// This trait hook *is* the two-argument call, and CE has no two-argument
    /// `_money_verbose`, so reaching it is precisely Python's `TypeError`. The
    /// sole live call site is the one `to_cheque` inherits from Base:
    ///
    /// ```python
    /// words = self._money_verbose(whole, currency)   # boom, every time
    /// ```
    ///
    /// So `to_cheque` is **unconditionally broken** for CE on every currency it
    /// implements, and the corpus records exactly that split:
    ///
    /// ```text
    /// cheque:EUR/USD/GBP  -> TypeError            (arity, raised here)
    /// cheque:JPY/KWD/...  -> NotImplementedError  (code lookup, raised first)
    /// ```
    ///
    /// Leaving `to_cheque` at its default reproduces both *and* keeps the raise
    /// at the call site Python raises from: `default_to_cheque` looks the
    /// currency up before calling `money_verbose`, matching Base's order.
    ///
    /// `to_currency` never lands here — it calls `cardinal(.., MONEY_CASE)`
    /// directly, which is what `_money_verbose` resolves to under `case="abs"`.
    fn money_verbose(&self, _number: &BigInt, _currency: &str) -> Result<String> {
        Err(N2WError::Type(MONEY_VERBOSE_ARITY_ERR.into()))
    }

    /// The fractional-cents hook: Python's `self.to_cardinal(float(right))`.
    ///
    /// The trait default routes to `floatpath::cardinal_from_bigdecimal`, which
    /// ends in `Num2Word_Base.to_cardinal_float` — the wrong method for CE,
    /// which overrides `to_cardinal` and never inherits that path. The default
    /// would print `lang.pointword()` ("запятая") where CE prints
    /// [`DECIMALPOINT`] ("а"), and would run `float2tuple`'s scaling and
    /// zero-padding where CE just slices the repr.
    ///
    /// The `float(right)` cast is reproduced rather than staying in arbitrary
    /// precision, because it is what Python does and it is lossy: `right` is a
    /// `Decimal` like 1.100, and the f64 it becomes is what `str()` then sees.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        self.cardinal_float(f)
    }

    /// `Num2Word_CE._cents_verbose(self, number, currency, case)` — the same
    /// three-parameter widening as [`money_verbose`](Self::money_verbose), and
    /// the same `TypeError` on a two-argument call.
    ///
    /// Unreachable in practice: the only caller is `default_to_currency`, which
    /// CE's `to_currency` override displaces. Ported for symmetry, so that the
    /// pair cannot drift apart and a future refactor that routes CE back
    /// through the default path fails loudly instead of silently inventing a
    /// two-argument method Python does not have.
    fn cents_verbose(&self, _number: &BigInt, _currency: &str) -> Result<String> {
        Err(N2WError::Type(
            "_cents_verbose() missing 1 required positional argument: 'case'".into(),
        ))
    }

    /// `Num2Word_CE.to_currency` at its `case="abs"` default. The full body,
    /// `case=` included, lives in [`LangCe::currency_with_case`];
    /// [`Lang::to_currency_kw`] below reaches it with the caller's `case`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        self.currency_with_case(val, currency, cents, separator, adjective, "abs")
    }

    /// `to_currency(..., case=...)` — the one extra kwarg CE's signature
    /// carries beyond the trait hook.
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["case"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let case = kw_str(kw, "case", "abs")?;
        self.currency_with_case(val, currency, cents, separator, adjective, case)
    }
}

impl LangCe {
    /// `Num2Word_CE.to_currency`, `case=` and all.
    ///
    /// Divisor is hardcoded **100** throughout — both in `has_fractional_cents`
    /// and via `parse_currency_parts`'s default. CE never consults
    /// `CURRENCY_PRECISION`, which is moot anyway: its dict is Base's empty one
    /// and none of its four codes is a 3- or 0-decimal currency.
    ///
    /// `case` is only ever consumed through `_money_verbose`/`_cents_verbose`'s
    /// `mcase` reduction (see [`money_case`]) and the fractional-cents float
    /// call, which discards it (module docs, bug 9). `adjective` is accepted
    /// and never read, exactly as in Python (divergence 3 above).
    fn currency_with_case(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
        case: &str,
    ) -> Result<String> {
        let mcase = money_case(case);
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it through this language's own default (Base's ",").
        let separator = separator.unwrap_or(self.default_separator());

        // `has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0`.
        //
        // An int can never satisfy this — `Decimal(str(5)) * 100` is exact —
        // so `keep_precision` is false for the whole Int arm. Python's Decimal
        // `%` truncates toward zero, as does `with_scale(0)`, so the two agree
        // on negatives: -12.34 yields `Decimal("-0.00")`, which is falsy,
        // exactly like the zero difference here.
        let has_fractional_cents = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => {
                let scaled = value * BigDecimal::from(100);
                &scaled - scaled.with_scale(0) != BigDecimal::zero()
            }
        };

        // parse_currency_parts(val, is_int_with_cents=False, keep_precision=..)
        //
        // `is_int_with_cents=False` is divergence 1: an int keeps its full
        // magnitude as `left` and gets `right = 0`, rather than being split
        // into units and cents. `to_currency(100, "EUR")` is a hundred euro,
        // not one euro.
        let (left, right, is_negative) = parse_currency_parts(val, false, has_fractional_cents, 100);

        // Python parses *before* the CURRENCY_FORMS lookup; the order is kept
        // even though parsing cannot fail.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        // `devise = cr1[0]` / `centime = cr2[0]` — bound once, always the
        // singular, for every quantity including the fractional branch below.
        //
        // Python's `except KeyError` wraps these two subscripts as well, but a
        // tuple raises IndexError, not KeyError, so an empty form tuple would
        // escape the handler rather than become a NotImplementedError. All four
        // entries are non-empty, so neither can fire.
        let index_err = || N2WError::Index("tuple index out of range".into());
        let devise = forms.unit.first().ok_or_else(index_err)?;
        let centime = forms.subunit.first().ok_or_else(index_err)?;

        // `"%s " % self.negword.strip()`.
        let minus_str = if is_negative {
            format!("{} ", NEGWORD.trim())
        } else {
            String::new()
        };

        // `self._money_verbose(left, currency, case)`, reduced to `mcase`.
        // `left` is always >= 0 — parse_currency_parts abs()es it — so the
        // negative branch of `cardinal` is unreachable from here; the sign is
        // carried by `minus_str` alone.
        let money_str = self.cardinal(&left, DEFAULT_CLAZZ, mcase)?;

        let cents_str = if has_fractional_cents {
            // Python's `isinstance(right, Decimal)` arm. `right` is a Decimal
            // exactly when keep_precision was true, i.e. iff
            // has_fractional_cents — the Int arm always yields a plain 0 and
            // the non-precision Decimal arm an `int(fraction * divisor)`.
            if cents {
                // `self.to_cardinal(float(right), case=case)` — CE's *own*
                // float branch, reached through the `cardinal_from_decimal`
                // override rather than Base's `to_cardinal_float`. That
                // override is what makes this arm say "а" and not "запятая":
                // CE joins on the module-level `DECIMALPOINT`, and
                // `self.pointword` is dead. The `case=` is accepted and
                // dropped, as it is for every float (module docs, bug 9).
                //
                // Verified live, because no corpus row reaches this arm — every
                // `ce` currency arg has <= 2 decimals, so has_fractional_cents
                // is always false:
                //
                //   to_currency(1.011, "USD") == "цхьа Доллар, цхьаъ а цхьаъ Сент"
                //   to_currency(2.675, "USD") == "ши Доллар, кхузткъе ворхӀ а пхиъ Сент"
                //   to_currency(0.005, "USD") == "ноль Доллар, ноль а пхиъ Сент"
                self.cardinal_from_decimal(&right)?
            } else {
                // `str(float(right))`. `right` is a cent count in [0, 100), so
                // Rust's shortest-round-trip `{}` and Python's `repr` agree and
                // neither reaches exponent form. (They part company below
                // ~1e-4, where Python switches to "1e-05" and Rust does not —
                // needs cents=False and a sub-micro-unit value to reach.)
                let f = right
                    .to_f64()
                    .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", right)))?;
                format!("{}", f)
            }
        } else {
            // `right` has scale 0 on this arm, so the unscaled BigInt *is* the
            // cent count. On the fractional arm above it would not be —
            // Decimal("0.500") unscales to 500 — hence the gating.
            let right_int = right.as_bigint_and_exponent().0;
            if cents {
                // `self._cents_verbose(right, currency, case)`, same `mcase`.
                self.cardinal(&right_int, DEFAULT_CLAZZ, mcase)?
            } else {
                // `self._cents_terse(right, currency)` — Base's, inherited
                // unchanged, and the trait default already mirrors it.
                self.cents_terse(&right_int, currency)?
            }
        };

        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str, money_str, devise, separator, cents_str, centime
        ))
    }
}

// ---------------------------------------------------------------------------
// Tests
//
// Every expectation below was produced by the live Python interpreter
// (`num2words(v, lang="ce")`), not written by hand — floats are pinned by
// their exact bit pattern so no decimal literal can round differently on the
// way in.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Render a result the way the generator recorded it: the string, or
    /// `"<PythonExceptionName>: <message>"`.
    fn show(r: Result<String>) -> String {
        match r {
            Ok(s) => s,
            Err(N2WError::Key(m)) => format!("KeyError: {}", m),
            Err(N2WError::Index(m)) => format!("IndexError: {}", m),
            Err(N2WError::Value(m)) => format!("ValueError: {}", m),
            Err(N2WError::Overflow(m)) => format!("OverflowError: {}", m),
            Err(N2WError::Type(m)) => format!("TypeError: {}", m),
            Err(N2WError::NotImplemented(m)) => format!("NotImplementedError: {}", m),
            Err(e) => format!("{:?}", e),
        }
    }

    /// What Python did: a sentence, or an exception rendered as
    /// `"<Name>: <message>"`. Deliberately not `Result`/`Ok`/`Err` — importing
    /// those variants here would shadow the crate's own `Result`.
    enum E {
        Words(&'static str),
        Raises(&'static str),
    }
    use E::{Raises, Words};

    impl E {
        fn want(&self) -> &'static str {
            match self {
                Words(s) | Raises(s) => s,
            }
        }
    }

    #[test]
    fn float_arm_matches_python() {
        // (bits of the f64, repr-derived precision, expected)
        let cases: &[(u64, u32, E)] = &[
        (0x0000000000000000_u64, 1, Words("ноль а ноль")), // 0.0
        (0x3fe0000000000000_u64, 1, Words("ноль а пхиъ")), // 0.5
        (0x3ff0000000000000_u64, 1, Words("цхьаъ а ноль")), // 1.0
        (0x3ff8000000000000_u64, 1, Words("цхьаъ а пхиъ")), // 1.5
        (0x4002000000000000_u64, 2, Words("шиъ а шиъ пхиъ")), // 2.25
        (0x40091eb851eb851f_u64, 2, Words("кхоъ а цхьаъ диъ")), // 3.14
        (0x3f847ae147ae147b_u64, 2, Words("ноль а ноль цхьаъ")), // 0.01
        (0x3fb999999999999a_u64, 1, Words("ноль а цхьаъ")), // 0.1
        (0x3fefae147ae147ae_u64, 2, Words("ноль а исс исс")), // 0.99
        (0x3ff028f5c28f5c29_u64, 2, Words("цхьаъ а ноль цхьаъ")), // 1.01
        (0x4028ae147ae147ae_u64, 2, Words("шийтта а кхоъ диъ")), // 12.34
        (0x4058ff5c28f5c28f_u64, 2, Words("дезткъе ткъайесна а исс исс")), // 99.99
        (0x4059200000000000_u64, 1, Words("бӀе а пхиъ")), // 100.5
        (0x40934a3d70a3d70a_u64, 2, Words("эзар ши бӀе ткъе дейтта а пхиъ ялх")), // 1234.56
        (0xbfe0000000000000_u64, 1, Words("минус ноль а пхиъ")), // -0.5
        (0xbff8000000000000_u64, 1, Words("минус цхьаъ а пхиъ")), // -1.5
        (0xc028ae147ae147ae_u64, 2, Words("минус шийтта а кхоъ диъ")), // -12.34
        (0x3ff0147ae147ae14_u64, 3, Words("цхьаъ а ноль ноль пхиъ")), // 1.005
        (0x4005666666666666_u64, 3, Words("шиъ а ялх ворхӀ пхиъ")), // 2.675
        (0x8000000000000000_u64, 1, Words("ноль а ноль")), // -0.0
        (0xbf847ae147ae147b_u64, 2, Words("минус ноль а ноль цхьаъ")), // -0.01
        (0x4132d687e3d70a3d_u64, 2, Words("цхьа миллион ши бӀе ткъе дейтта эзар пхи бӀе кхузткъе ворхӀ а бархӀ исс")), // 1234567.89
        (0x42dc12218377de66_u64, 1, Words("бӀе ткъе кхо биллион ди бӀе шовзткъе ялхитта миллиард ворхӀ бӀе дезткъе исс миллион шийтта эзар кхо бӀе шовзткъе пхиъ а ялх")), // 123456789012345.6
        (0x3f1a36e2eb1c432d_u64, 4, Words("ноль а ноль ноль ноль цхьаъ")), // 0.0001
        (0x430c6bf526340000_u64, 1, Words("цхьа биллиард а ноль")), // 1000000000000000.0
        (0x3fd3333333333334_u64, 17, Words("ноль а кхоъ ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль ноль диъ")), // 0.30000000000000004
        (0x444b1ae4d6e2ef50_u64, 21, Raises("IndexError: list index out of range")), // 1e+21
        (0x4454542ba12a337c_u64, 20, Raises("ValueError: invalid literal for int() with base 10: 'e'")), // 1.5e+21
        (0x3ee4f8b588e368f1_u64, 5, Raises("IndexError: list index out of range")), // 1e-05
        (0x3eef75104d551d69_u64, 6, Raises("ValueError: invalid literal for int() with base 10: 'e'")), // 1.5e-05
        (0x4341c37937e08000_u64, 16, Raises("IndexError: list index out of range")), // 1e+16
        (0x0000000000000001_u64, 324, Raises("IndexError: list index out of range")), // 5e-324
        (0x46fed09bead87c03_u64, 34, Raises("IndexError: list index out of range")), // 1e+34
        // #159: past the 10^34 ceiling the float is an OverflowError (Python
        // returned "NOT IMPLEMENTED" for 1e34 and hit int('e') here).
        (0x47071c74f0225d03_u64, 33, Raises("OverflowError: abs(15000000000000000913010721715912704) must be less than 10000000000000000000000000000000000.")), // 1.5e+34
        (0xc0934a3d70a3d70a_u64, 2, Words("минус эзар ши бӀе ткъе дейтта а пхиъ ялх")), // -1234.56
        (0x54b249ad2594c37d_u64, 100, Raises("OverflowError: abs(10000000000000000159028911097599180468360808563945281389781327557747838772170381060813469985856815104) must be less than 10000000000000000000000000000000000.")), // 1e+100 (#159; Python: IndexError)
        (0x4034800000000000_u64, 1, Words("ткъа а пхиъ")), // 20.5
        (0x4044000000000000_u64, 1, Words("шовзткъа а ноль")), // 40.0
        (0x408f3ffdf3b645a2_u64, 3, Words("исс бӀе дезткъе ткъайесна а исс исс исс")), // 999.999
        (0x43179085685d83c9_u64, 1, Words("цхьа биллиард ялх бӀе шовзткъе берхӀитта биллион ши бӀе ялх миллиард ворхӀ бӀе дезткъе миллион дезткъе бархӀ эзар пхи бӀе кхузткъе шиъ а шиъ")), // 1658206780088562.2
        (0x42eee708f58b7e34_u64, 2, Words("ши бӀе кхузткъе цхьайтта биллион бархӀ бӀе ткъе цхьа миллиард дезткъе шийтта миллион ворхӀ бӀе ворхӀ эзар кхо бӀе кхойтта а ялх шиъ")), // 271821092707313.62
        ];
        let l = LangCe::new();
        for (bits, precision, want) in cases {
            let value = f64::from_bits(*bits);
            let v = FloatValue::Float {
                value,
                precision: *precision,
            };
            let got = show(l.to_cardinal_float(&v, None));
            assert_eq!(got, want.want(), "float {:?} (bits {:#x})", value, bits);
        }
    }

    #[test]
    fn decimal_arm_matches_python() {
        let cases: &[(&str, E)] = &[
        ("0.01", Raises("KeyError: Decimal('0.01')")),
        ("1.10", Raises("KeyError: Decimal('1.10')")),
        ("12.345", Raises("KeyError: Decimal('12.345')")),
        ("98746251323029.99", Raises("KeyError: Decimal('9.99')")),
        ("0.001", Raises("KeyError: Decimal('0.001')")),
        ("5", Words("пхиъ")),
        ("-12.34", Raises("KeyError: Decimal('12.34')")),
        ("80.00", Words("дезткъа")),
        ("0.0000001", Raises("KeyError: Decimal('1E-7')")),
        ("5.5", Raises("KeyError: Decimal('5.5')")),
        ("20.5", Raises("KeyError: Decimal('0.5')")),
        ("1E+2", Words("бӀе")),
        ("323029.99", Raises("KeyError: Decimal('9.99')")),
        ("29.99", Raises("KeyError: Decimal('9.99')")),
        ("-0.00", Words("ноль")),
        ("1e34", Raises("OverflowError: abs(1e+34) must be less than 10000000000000000000000000000000000.")),
        ("0", Words("ноль")),
        ("19.999", Raises("KeyError: Decimal('19.999')")),
        ("1000000.5", Raises("KeyError: Decimal('0.5')")),
        ("999.5", Raises("KeyError: Decimal('19.5')")),
        ];
        let l = LangCe::new();
        for (s, want) in cases {
            let value = BigDecimal::from_str(s).unwrap();
            // `precision` is `abs(exponent)` on the Python side and unread here,
            // exactly as in Python; the arm never consults it.
            let v = FloatValue::Decimal {
                value,
                precision: 0,
            };
            let got = show(l.to_cardinal_float(&v, None));
            assert_eq!(got, want.want(), "Decimal({:?})", s);
        }
    }

    /// `precision=` assigns `self.precision`, which CE never reads — so every
    /// override must leave the output untouched (module docs, bug 10).
    #[test]
    fn precision_override_is_inert() {
        let l = LangCe::new();
        let v = FloatValue::Float {
            value: 2.675,
            precision: 3,
        };
        let base = show(l.to_cardinal_float(&v, None));
        assert_eq!(base, "шиъ а ялх ворхӀ пхиъ");
        for p in [0u32, 1, 3, 5, 17] {
            assert_eq!(show(l.to_cardinal_float(&v, Some(p))), base, "precision={}", p);
        }
    }

    /// The fractional-cents arm of `to_currency`, which reaches the float path
    /// through `cardinal_from_decimal`. No corpus row covers it (every `ce`
    /// currency arg has <= 2 decimals), so these came from the interpreter with
    /// `num2words2._RUST = None` forced.
    ///
    /// That forcing is not paranoia. `num2words()` routes `to="currency"`
    /// through the compiled core whenever one is installed, so an interpreter
    /// with a stale `_rust.abi3.so` answers these queries *from the old Rust
    /// build* rather than from Python — and reports the very divergence this
    /// override fixes as though it were the spec. Ground truth for a port has
    /// to come from the pure-Python path, or the port is graded against itself.
    #[test]
    fn fractional_cents_matches_python() {
        let cases: &[(&str, &str, bool, E)] = &[
        ("1.011", "USD", true, Words("цхьа Доллар, цхьаъ а цхьаъ Сент")),
        ("2.675", "USD", true, Words("ши Доллар, кхузткъе ворхӀ а пхиъ Сент")),
        ("0.005", "USD", true, Words("ноль Доллар, ноль а пхиъ Сент")),
        ("1.011", "USD", false, Words("цхьа Доллар, 1.1 Сент")),
        ("12.34", "USD", true, Words("шийтта Доллар, ткъе дейтта Сент")),
        ("1.0055", "RUB", true, Words("цхьа Сом, ноль а пхиъ пхиъ Кепек")),
        ];
        let l = LangCe::new();
        for (val, code, cents, want) in cases {
            let v = CurrencyValue::parse(val, false, true, true).unwrap();
            let got = show(l.to_currency(&v, code, *cents, None, false));
            assert_eq!(got, want.want(), "to_currency({}, {})", val, code);
        }
    }

    /// `python_repr` is the whole float arm's foundation: CE slices the repr
    /// string, so a wrong repr is a wrong sentence. Spot-checks of the shapes
    /// that distinguish it from a naive `{}` — CPython's fixed/exponent
    /// threshold, and the exact-tie doubles where Rust's own shortest-repr
    /// disagrees with `repr`.
    #[test]
    fn python_repr_matches_cpython() {
        let cases: &[(f64, &str)] = &[
            (0.0, "0.0"),
            (0.5, "0.5"),
            (2.675, "2.675"),
            (1.005, "1.005"),
            (100.5, "100.5"),
            (1.0, "1.0"),
            (0.0001, "0.0001"),          // decpt == -3, the last fixed one
            (1e-5, "1e-05"),             // decpt == -4, first exponential
            (1e15, "1000000000000000.0"),// decpt == 16, the last fixed one
            (1e16, "1e+16"),             // decpt == 17, first exponential
            (1e21, "1e+21"),
            (1.5e21, "1.5e+21"),
            (1e100, "1e+100"),
            (5e-324, "5e-324"),
            (0.30000000000000004, "0.30000000000000004"),
            (123456789012345.6, "123456789012345.6"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            // Exact ties: the double is …562.25 / …313.625, equidistant between
            // two shortest candidates. CPython rounds half-to-even and emits
            // .2 / .62; Rust's own `{}` and `{:e}` emit .3 / .63.
            (1658206780088562.2, "1658206780088562.2"),
            (271821092707313.62, "271821092707313.62"),
        ];
        for (x, want) in cases {
            assert_eq!(&python_repr(*x), want, "python_repr({:?})", x);
        }
    }

    /// `repr(Decimal)`, the text inside the KeyError. Trailing zeros are
    /// significant and the switch to scientific notation is exponent-driven.
    #[test]
    fn decimal_repr_matches_cpython() {
        let cases: &[(&str, &str)] = &[
            ("0.01", "Decimal('0.01')"),
            ("1.10", "Decimal('1.10')"),
            ("12.345", "Decimal('12.345')"),
            ("0.001", "Decimal('0.001')"),
            ("0.0000001", "Decimal('1E-7')"),   // adjusted < -6 -> scientific
            ("0.000001", "Decimal('0.000001')"),// adjusted == -6 -> fixed
            ("1E+2", "Decimal('1E+2')"),        // positive exponent -> scientific
            ("5", "Decimal('5')"),
            ("0", "Decimal('0')"),
            ("-12.34", "Decimal('-12.34')"),
            ("80.00", "Decimal('80.00')"),
        ];
        for (s, want) in cases {
            let v = BigDecimal::from_str(s).unwrap();
            assert_eq!(&decimal_repr(&v), want, "decimal_repr({:?})", s);
        }
    }

    /// The integer modes must be untouched by this phase.
    #[test]
    fn integer_modes_unchanged() {
        let l = LangCe::new();
        assert_eq!(l.to_cardinal(&BigInt::from(0)).unwrap(), "ноль");
        assert_eq!(l.to_cardinal(&BigInt::from(99)).unwrap(), "дезткъе ткъайесна");
        assert_eq!(l.to_ordinal(&BigInt::from(100)).unwrap(), "бІолгІа");
        assert_eq!(l.to_cardinal(&BigInt::from(100)).unwrap(), "бӀе");
        assert_eq!(l.to_ordinal_num(&BigInt::from(7)).unwrap(), "7-й");
    }

    fn fl(v: f64) -> FloatValue {
        FloatValue::Float {
            value: v,
            precision: 1,
        }
    }

    fn dc(s: &str) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: 0,
        }
    }

    /// The entry hooks, against corpus_wholefloat rows: no whole-value
    /// shortcut for floats, integer branches for Decimals, `case="ORD"` only
    /// felt by the Decimal arm of to_ordinal.
    #[test]
    fn float_entry_hooks_match_corpus() {
        let l = LangCe::new();
        // cardinal: every float takes the float branch, X.0 included.
        assert_eq!(show(l.cardinal_float_entry(&fl(1.0), None)), "цхьаъ а ноль");
        assert_eq!(show(l.cardinal_float_entry(&fl(-0.0), None)), "ноль а ноль");
        assert_eq!(
            show(l.cardinal_float_entry(&fl(-1000000.0), None)),
            "минус цхьа миллион а ноль"
        );
        assert_eq!(
            show(l.cardinal_float_entry(&fl(1e16), None)),
            "IndexError: list index out of range"
        );
        // ordinal: float -> plain cardinal; Decimal -> real ordinal.
        assert_eq!(show(l.ordinal_float_entry(&fl(5.0))), "пхиъ а ноль");
        assert_eq!(show(l.ordinal_float_entry(&fl(-3.0))), "минус кхоъ а ноль");
        assert_eq!(show(l.ordinal_float_entry(&dc("5.00"))), "пхоьалгӀа");
        assert_eq!(show(l.ordinal_float_entry(&dc("0"))), "нольалгӀа");
        assert_eq!(show(l.ordinal_float_entry(&dc("-0.0"))), "нольалгӀа");
        assert_eq!(show(l.ordinal_float_entry(&dc("1E+2"))), "бІолгІа");
        assert_eq!(show(l.ordinal_float_entry(&dc("-3.0"))), "минус кхоалгӀа");
        assert_eq!(
            show(l.ordinal_float_entry(&dc("12345.000"))),
            "шийтта эзар кхо бӀе шовзткъе пхоьалгӀа"
        );
        assert_eq!(show(l.ordinal_float_entry(&dc("1E+20"))), "бӀе триллион");
        // year == cardinal routing.
        assert_eq!(show(l.year_float_entry(&fl(2.5))), "шиъ а пхиъ");
        assert_eq!(show(l.year_float_entry(&dc("-3.0"))), "минус кхоъ");
        // ordinal_num: verify_ordinal, then repr + "-й".
        assert_eq!(show(l.ordinal_num_float_entry(&fl(5.0), "5.0")), "5.0-й");
        assert_eq!(show(l.ordinal_num_float_entry(&fl(-0.0), "-0.0")), "-0.0-й");
        assert_eq!(show(l.ordinal_num_float_entry(&fl(1e16), "1e+16")), "1e+16-й");
        assert_eq!(
            show(l.ordinal_num_float_entry(&fl(0.5), "0.5")),
            "TypeError: Cannot treat float 0.5 as ordinal."
        );
        assert_eq!(
            show(l.ordinal_num_float_entry(&fl(-3.0), "-3.0")),
            "TypeError: Cannot treat negative num -3.0 as ordinal."
        );
        assert_eq!(show(l.ordinal_num_float_entry(&dc("5.00"), "5.00")), "5.00-й");
        assert_eq!(
            show(l.ordinal_num_float_entry(&dc("1.50"), "1.50")),
            "TypeError: Cannot treat float 1.50 as ordinal."
        );
    }

    /// Inf/NaN parse as the base Decimal specials and are served natively by
    /// the `inf_result`/`nan_result` hooks; no Python fallback.
    #[test]
    fn str_to_number_native_inf_nan() {
        let l = LangCe::new();
        assert!(matches!(
            l.str_to_number("Infinity"),
            Ok(ParsedNumber::Inf { negative: false })
        ));
        assert!(matches!(
            l.str_to_number("-Infinity"),
            Ok(ParsedNumber::Inf { negative: true })
        ));
        assert!(matches!(l.str_to_number("NaN"), Ok(ParsedNumber::NaN)));
        assert!(matches!(l.str_to_number("5"), Ok(ParsedNumber::Dec(_))));

        // Native Inf/NaN/-0.0 outcomes.
        // #159: Infinity is an OverflowError, not the "NOT IMPLEMENTED" string.
        assert!(matches!(l.inf_result(false, "cardinal"), Err(N2WError::Overflow(_))));
        assert!(matches!(l.inf_result(true, "cardinal"), Err(N2WError::Overflow(_))));
        assert!(matches!(
            l.nan_result("cardinal"),
            Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                ..
            })
        ));
        assert_eq!(show(l.neg_zero_decimal("cardinal").unwrap()), "ноль");
        assert_eq!(show(l.neg_zero_decimal("ordinal").unwrap()), "нольалгӀа");
        assert_eq!(show(l.neg_zero_decimal("year").unwrap()), "ноль");
        assert!(l.neg_zero_decimal("ordinal_num").is_none());
    }

    /// Grammatical kwargs, against corpus_kwargs rows.
    #[test]
    fn kwargs_hooks_match_corpus() {
        let l = LangCe::new();
        let kw = |pairs: &[(&str, &str)]| {
            Kwargs(
                pairs
                    .iter()
                    .map(|(k, v)| (k.to_string(), KwVal::Str(v.to_string())))
                    .collect(),
            )
        };
        let n = BigInt::from(1234);
        // clazz agreement on 4/14/40 compounds.
        assert_eq!(
            show(l.to_cardinal_kw(&n, &kw(&[("clazz", "б")]))),
            "эзар ши бӀе ткъе бейтта"
        );
        assert_eq!(
            show(l.to_cardinal_kw(&n, &kw(&[("clazz", "x")]))),
            "эзар ши бӀе ткъе xейтта"
        );
        assert_eq!(
            show(l.to_ordinal_kw(&n, &kw(&[("clazz", "й")]))),
            "эзар ши бӀе ткъе йейтталгӀа"
        );
        // cases: attr, gen, ORD, and the KeyError on "bogus".
        assert_eq!(show(l.to_cardinal_kw(&BigInt::from(1), &kw(&[("case", "attr")]))), "цхьа");
        assert_eq!(
            show(l.to_cardinal_kw(&n, &kw(&[("case", "gen")]))),
            "эзар ши бӀе ткъе дейттаннан"
        );
        assert_eq!(
            show(l.to_year_kw(&BigInt::from(21), &kw(&[("case", "ORD")]))),
            "ткъе цхьалгӀа"
        );
        assert_eq!(
            show(l.to_cardinal_kw(&BigInt::from(5), &kw(&[("case", "bogus")]))),
            "KeyError: 'bogus'"
        );
        // to_ordinal has no case= parameter -> fall back to Python (Fallback).
        assert!(matches!(
            l.to_ordinal_kw(&n, &kw(&[("case", "gen")])),
            Err(N2WError::Fallback(_))
        ));
        // An explicit None on an unknown kwarg is Python's problem — decline.
        assert!(matches!(
            l.to_cardinal_kw(&n, &Kwargs(vec![("clazz".into(), KwVal::None)])),
            Err(N2WError::Fallback(_))
        ));
        // currency case=attr -> oblique money/cents forms.
        let v = CurrencyValue::Int(BigInt::from(1234));
        assert_eq!(
            show(l.to_currency_kw(&v, "RUB", true, None, false, &kw(&[("case", "attr")]))),
            "эзар ши бӀе ткъе дейттан Сом, нольан Кепек"
        );
        // and case absent keeps the "attr" money case (default path).
        assert_eq!(
            show(l.to_currency_kw(&v, "RUB", true, None, false, &Kwargs(vec![]))),
            "эзар ши бӀе ткъе дейтта Сом, ноль Кепек"
        );
        // Decimal + kwargs: float arm drops them, Decimal arm feels them.
        assert_eq!(
            show(l.to_cardinal_float_kw(&dc("21"), None, &kw(&[("case", "ORD")]))),
            "ткъе цхьалгӀа"
        );
        assert_eq!(
            show(l.to_cardinal_float_kw(&fl(1.5), None, &kw(&[("case", "ORD")]))),
            "цхьаъ а пхиъ"
        );
    }
}
