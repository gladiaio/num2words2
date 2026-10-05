//! Port of `lang_LIJ.py` (Ligurian / Genoese).
//!
//! Shape: **self-contained**. `Num2Word_LIJ` subclasses `Num2Word_EU` (which
//! subclasses `Num2Word_Base`), but it overrides `__init__` with a bare `pass`
//! — so `Num2Word_Base.__init__` never runs. That has three consequences the
//! port depends on:
//!
//!   * `setup()` is never called, so `self.negword` / `self.pointword` are
//!     **never set**. LIJ sidesteps this by using its own class attribute
//!     `MINUS_PREFIX_WORD = "meno "` in `to_cardinal`/`to_ordinal`. Anything
//!     that reads `self.negword` — i.e. `to_currency` — dies with
//!     `AttributeError`; the frozen corpus records exactly that for
//!     `currency:*` with a negative arg. See [`LangLij::to_currency`].
//!   * `self.cards` / `self.MAXVAL` are never built (LIJ defines no
//!     `high_numwords`/`mid_numwords`/`low_numwords` either), so `cards`
//!     and `merge` stay at their trait defaults and are never reached.
//!   * The only ceiling is `big_number_to_cardinal`'s `EXPONENTS` table;
//!     `maxval()` reports it (10^36) — see "Ceiling" below.
//!
//! Inherited from `Num2Word_EU` — note that `Num2Word_EU` is the *Basque*
//! converter, so LIJ picks up two Basque-flavoured methods verbatim. This is
//! not a transcription slip; it is what the key "lij" actually resolves to,
//! and the corpus confirms it:
//!   * `to_ordinal_num(number)` → `str(number) + "."`     e.g. 1 → "1."
//!   * `to_year(val, longval=True)` → `self.to_cardinal(val) + ". urtea"`
//!     e.g. 2000 → "doamia. urtea" — Basque "urtea" ("the year") glued onto a
//!     Ligurian numeral, with the stray "." from the ordinal-style format.
//!     `to_year` resolves `self.to_cardinal` dynamically, so it picks up the
//!     LIJ override below (including its "meno " sign handling: -500 →
//!     "meno çinqueçento. urtea").
//!
//! # Gender / plural
//!
//! LIJ's `to_cardinal(number, gender="m")` and
//! `to_ordinal(number, gender="m", plural=False)` take extra arguments that
//! the four ported modes never vary — every reachable call passes the default
//! `gender="m"` / `plural=False`. The parameters are threaded through here
//! anyway so the recursion matches Python one-for-one; the `"f"`/plural
//! branches are unreachable from `to_cardinal`, `to_ordinal`, `to_ordinal_num`
//! and `to_year`. Likewise the `gender = gender if remainder != 1 else "m"`
//! re-binds are no-ops under `gender == "m"`, but are kept verbatim.
//!
//! `to_currency` is what makes the `"f"` branch live: LIJ picks
//! `gender = "f" if currency in CURRENCIES_FEM else "m"` and threads it into
//! `to_cardinal`, so GBP/INR/ITL inflect the *unit* count — 1.0 GBP is
//! "uña sterliña", 202 GBP is "doe sterliñe", 301 GBP is "træ sterliñe". The
//! plural branches of `to_ordinal` stay dead: nothing in the currency path
//! calls it.
//!
//! # Ceiling
//!
//! `big_number_to_cardinal` splits `number // 10**6` into 3-digit triplets and
//! zips them against the 10-entry `EXPONENTS` table. `len(triplets) >
//! len(EXPONENTS)` raises `NotImplementedError("The given number is too
//! large.")`. `len(str(mils)) > 30` ⇒ 11 triplets, so the wall is at
//! **10**36** exactly (10**36 - 1 is the largest convertible value, and
//! `to_cardinal` checks the sign *first*, so -10**36 raises too). Verified
//! against the interpreter. Python raises `NotImplementedError` there; fixed
//! (gladiaio/num2words2#159): the port raises `OverflowError` ("abs(v) must
//! be less than 10^36.").
//!
//! # Faithfully reproduced Python oddities
//!
//! Verified against the interpreter; all are preserved verbatim.
//!
//! 1. **`big_to_odinal`'s "quattòrze" branch emits a double "e".** The branch
//!    does `string[:-4] + "orze"` (dropping "òrze", re-adding "orze") and then
//!    unconditionally appends "eximo", yielding "...quattorze" + "eximo" =
//!    "...quattorzeeximo". Presumably `[:-4] + "orz"` was meant. Unreachable
//!    for 14 itself (< 20 goes to `small_to_ordinal` → "quattorzen"), but live
//!    for every larger number whose cardinal ends in "quattòrze":
//!    `to_ordinal(114)` == "çentoquattorzeeximo", `to_ordinal(614)` ==
//!    "seiçentoquattorzeeximo". Not in the corpus — confirmed by hand against
//!    Python.
//! 2. The method is named `big_to_odinal` (missing "r") in the Python source.
//!    Kept as [`big_to_odinal`] so the port greps back to its origin.
//! 3. The "ëi"/"ei" branch likewise re-adds "ei" before appending "eximo",
//!    giving "...seieximo" (e.g. `to_ordinal(123456)` ==
//!    "çentovintitræmiaquattroçentoçinquanteseieximo"). Unlike (1) this one is
//!    corpus-confirmed, so it is intended (or at least frozen) behaviour.
//! 4. `to_ordinal(0)` returns "zero" and `to_ordinal(-1)` returns
//!    "meno primmo" — LIJ never calls `verify_ordinal`, so unlike most
//!    languages it neither rejects nor crashes on these.
//! 5. `hundreds_to_cardinal` applies `.replace("ë", "e")` only to the
//!    *hundreds prefix*, and `thousands_to_cardinal` only to the *thousands
//!    prefix* — so 600 → "seiçento" (ë stripped) while the remainder keeps
//!    its diacritic: 123456 → "çentovintitræmiaquattroçentoçinquantesëi".
//! 6. **Float routing is numeric, not repr-based.** `to_cardinal` tests
//!    `int(number) != number` and `to_ordinal` tests `number % 1 != 0`, so a
//!    *whole* float/Decimal (5.0, Decimal("5.00"), 1E+2) takes the integer
//!    grammar — `to_ordinal(5.0)` == "quinto", never "çinque". And since the
//!    sign tests are numeric `< 0` comparisons, `-0.0` is **not** negative:
//!    `to_ordinal(-0.0)` == "zero", `to_year(-0.0)` == "zero. urtea".
//! 7. **`to_ordinal(2.5)` == "segondo virgola çinque"** — `float_to_words`
//!    with `ordinal=True` ordinalizes only the *integer prefix*
//!    (`self.to_ordinal(int(float_number), gender, plural)`); the fractional
//!    digits stay cardinal and masculine.
//! 8. **`to_fraction` is Base's, and it reads `self.negword` on the negative
//!    branch** — which (bare `__init__`, see above) does not exist, so any
//!    sign-negative fraction raises `AttributeError`. The dispatcher routes
//!    every `"n/d"` *string* through `to_fraction` regardless of `to=`, so
//!    `num2words("-3/4", to="year"/"currency"/...)` all raise AttributeError
//!    too. Non-negative fractions never touch `negword` (the sign is a
//!    conditional *expression*) and render fine — including `-3/-4` (signs
//!    cancel) and `-5/1` (short-circuits into LIJ's own `to_cardinal`, which
//!    uses `MINUS_PREFIX_WORD`, not `negword`).
//! 9. **`to_cardinal(Decimal("NaN"))` raises `decimal.InvalidOperation`**, not
//!    ValueError: the very first test `number < 0` is a NaN comparison, which
//!    the decimal module refuses. See [`LangLij::str_to_number`].
//!
//! # Grammatical kwargs (the live `"f"`/plural branches)
//!
//! The public Python signatures are `to_cardinal(number, gender="m")` and
//! `to_ordinal(number, gender="m", plural=False)`; `to_year(val, longval=True)`
//! is EU's (`longval` accepted and ignored); `to_ordinal_num(number)` and
//! `to_currency(...)` take nothing extra. LIJ only ever *tests* `gender ==
//! "f"` (everything else — "m", None, 1, "x" — behaves masculine, no raise)
//! and uses `plural` for truthiness only, so the `*_kw` hooks normalise
//! accordingly. `zero` is immune to both gender and plural in the ordinals.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `MINUS_PREFIX_WORD` — note the trailing space (LIJ concatenates it raw
/// rather than going through `negword.strip()` like the base class).
const MINUS_PREFIX_WORD: &str = "meno ";

/// `Num2Word_LIJ.FLOAT_INFIX_WORD` — the decimal separator word, with a space
/// on each side (`float_to_words` glues it in raw as `prefix + INFIX +
/// postfix`).
const FLOAT_INFIX_WORD: &str = " virgola ";

/// Basque suffix inherited from `Num2Word_EU.to_year`. See module docs.
const YEAR_SUFFIX: &str = ". urtea";

const SMALL_CARDINALS: [&str; 20] = [
    "zero",
    "un",
    "doî",
    "trei",
    "quattro",
    "çinque",
    "sëi",
    "sette",
    "eutto",
    "neuve",
    "dexe",
    "unze",
    "dozze",
    "trezze",
    "quattòrze",
    "chinze",
    "sezze",
    "dïsette",
    "dixeutto",
    "dixineuve",
];

const TENS: [&str; 10] = [
    "zero",
    "dexe",
    "vinti",
    "trenta",
    "quaranta",
    "çinquanta",
    "sciuscianta",
    "settanta",
    "ottanta",
    "novanta",
];

const HUNDREDS: [&str; 4] = ["zero", "çento", "duxento", "trexento"];

const THOUSANDS: [&str; 4] = ["zero", "mille", "doamia", "træmia"];

const EXPONENTS: [&str; 10] = [
    "mion",
    "miliardo",
    "bilion",
    "biliardo",
    "trilion",
    "triliardo",
    "quadrilion",
    "quadriliardo",
    "quintilion",
    "quintiliardo",
];

const SMALL_ORDINALS: [&str; 20] = [
    "zero",
    "primmo",
    "segondo",
    "terso",
    "quarto",
    "quinto",
    "sesto",
    "setten",
    "otten",
    "noven",
    "dexen",
    "unzen",
    "dozzen",
    "trezzen",
    "quattorzen",
    "chinzen",
    "sezzen",
    "dïsetten",
    "dixotten",
    "dixinoven",
];

// ---- currency data ------------------------------------------------------
//
// `Num2Word_LIJ` declares its **own** class-body `CURRENCY_FORMS`, so it does
// not read (or get mutated by) the shared `Num2Word_EUR` dict that
// `Num2Word_EN.__init__` rewrites in place. Confirmed against the live
// interpreter: LIJ's MRO is `Num2Word_LIJ -> Num2Word_EU -> Num2Word_Base`,
// `Num2Word_EN` is not on it, and `LIJ.CURRENCY_FORMS is EU.CURRENCY_FORMS` is
// False. The literal below therefore *is* the runtime table — no EN overlay,
// and none of EN's ~24 extra codes (AED/KRW/BRL/…) are visible here.

/// Python's module-level `CENTS`.
const CENTS: [&str; 2] = ["citto", "citti"];
/// Python's `DOLLAR[0]` — the unit half of `DOLLAR = (("dòllao","dòllai"), CENTS)`.
const DOLLAR: [&str; 2] = ["dòllao", "dòllai"];
/// Python's `FRANC[0]`.
const FRANC: [&str; 2] = ["franco", "franchi"];

/// Python's `CURRENCIES_FEM = {"GBP", "INR", "ISK", "ITL"}` — the codes whose
/// *unit* count is feminine.
///
/// "ISK" is dead weight: it has no `CURRENCY_FORMS` entry, and the forms
/// lookup raises `NotImplementedError` before `gender` is ever computed. Kept
/// so the set matches the source one-for-one.
const CURRENCIES_FEM: [&str; 4] = ["GBP", "INR", "ISK", "ITL"];

/// `Num2Word_LIJ.CURRENCY_FORMS`, verbatim.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AUD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("CAD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("CHF", CurrencyForms::new(&FRANC, &CENTS));
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &CENTS));
    m.insert("FRF", CurrencyForms::new(&FRANC, &CENTS));
    m.insert(
        "GBP",
        CurrencyForms::new(&["sterliña", "sterliñe"], &["penny", "pence"]),
    );
    m.insert("HKD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("INR", CurrencyForms::new(&["rupia", "rupie"], &["paisa", "paise"]));
    m.insert("ITL", CurrencyForms::new(&["lia", "lie"], &CENTS));
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("SGD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("USD", CurrencyForms::new(&DOLLAR, &CENTS));
    m
}

/// `Num2Word_LIJ.CURRENCY_ADJECTIVES`, verbatim.
///
/// Each entry is a **pair** (singular, plural) that LIJ zips against the two
/// unit forms, so this cannot go through the `currency_adjective` trait hook —
/// that returns a single `&str` for `currency::prefix_currency`, which LIJ does
/// not use. See [`LangLij::to_currency`].
fn build_currency_adjectives() -> HashMap<&'static str, (&'static str, &'static str)> {
    [
        ("AUD", ("australian", "australien")),
        ("CAD", ("canadeise", "canadeixi")),
        ("CHF", ("svissero", "svisseri")),
        ("FRF", ("franseise", "franseixi")),
        ("HKD", ("de Hong Kong", "de Hong Kong")),
        ("ITL", ("italiaña", "italiañe")),
        ("SGD", ("de Scingapô", "de Scingapô")),
        ("NZD", ("neozelandeise", "neozelandeixi")),
        ("USD", ("american", "americhen")),
    ]
    .into_iter()
    .collect()
}

/// Python `tuple[i]` past the end. LIJ's own table always has two forms, so
/// this is unreachable — but the exception *type* is mapped rather than
/// panicking, in case the table is ever edited.
fn tuple_index_error() -> N2WError {
    N2WError::Index("tuple index out of range".into())
}

/// Python `s[:-n]`, counting **characters** not bytes (the tables are full of
/// ç/ë/î/ò/æ/ï). Python clamps rather than panicking when `n > len(s)`.
fn drop_last(s: &str, n: usize) -> String {
    let total = s.chars().count();
    s.chars().take(total.saturating_sub(n)).collect()
}

/// Python `s[-1]`. Returns `None` only for an empty string, where Python would
/// raise `IndexError`; every call site here is guarded by `number >= 20`, so
/// the string is never empty and the `None` arm is unreachable.
fn last_char(s: &str) -> Option<char> {
    s.chars().next_back()
}

/// Port of `small_to_cardinal`. Python asserts `number < 20`.
fn small_to_cardinal(number: u32, gender: &str) -> String {
    debug_assert!(number < 20);
    let mut string = SMALL_CARDINALS[number as usize].to_string();
    if gender == "f" {
        // Sequential `if`s in Python, not `elif` — mutually exclusive on
        // `number`, so the distinction is moot, but kept as-is.
        if number == 1 {
            string = drop_last(&string, 1) + "ña";
        }
        if number == 2 {
            string = drop_last(&string, 1) + "e";
        }
        if number == 3 {
            string = drop_last(&string, 2) + "æ";
        }
    }
    string
}

/// Port of `tens_to_cardinal`. Python asserts `20 <= number < 100`.
fn tens_to_cardinal(number: u32, gender: &str) -> String {
    debug_assert!((20..100).contains(&number));
    let (tens, units) = (number / 10, number % 10);
    let mut string = TENS[tens as usize].to_string();
    // Bare tens
    if units == 0 {
        return string;
    }
    // Tens + units, with phonetic contractions
    if tens != 2 {
        string = drop_last(&string, 1) + "e";
    }
    if units == 1 || units == 8 {
        string = drop_last(&string, 1);
    }
    let gender = if units != 1 { gender } else { "m" };
    string + &small_to_cardinal(units, gender)
}

/// Port of `hundreds_to_cardinal`. Python asserts `100 <= number < 1000`.
fn hundreds_to_cardinal(number: u32, gender: &str) -> Result<String> {
    debug_assert!((100..1000).contains(&number));
    let (hundreds, remainder) = (number / 100, number % 100);
    let string = if (hundreds as usize) < HUNDREDS.len() {
        let s = HUNDREDS[hundreds as usize].to_string();
        if (80..=89).contains(&remainder) {
            drop_last(&s, 1)
        } else {
            s
        }
    } else {
        let s = small_to_cardinal(hundreds, "m").replace('ë', "e");
        // Phonetic adjustment
        if (80..=89).contains(&remainder) {
            s + "çent"
        } else {
            s + "çento"
        }
    };
    if remainder == 0 {
        Ok(string)
    } else {
        let gender = if remainder != 1 { gender } else { "m" };
        Ok(string + &to_cardinal_gender(&BigInt::from(remainder), gender)?)
    }
}

/// Port of `thousands_to_cardinal`. Python asserts `1000 <= number < 10**6`.
fn thousands_to_cardinal(number: u32, gender: &str) -> Result<String> {
    debug_assert!((1000..1_000_000).contains(&number));
    let (thousands, remainder) = (number / 1000, number % 1000);
    let string = if (thousands as usize) < THOUSANDS.len() {
        THOUSANDS[thousands as usize].to_string()
    } else {
        to_cardinal_gender(&BigInt::from(thousands), "m")?
            // fossilised forms doa- and træ-
            .replace("doî", "doa")
            .replace("trei", "træ")
            // no length markers on "sëi"
            .replace('ë', "e")
            // no grave accent on "quattòrze" since it isn't the main stress
            .replace('ò', "o")
            + "mia"
    };
    if remainder == 0 {
        Ok(string)
    } else {
        let gender = if remainder != 1 { gender } else { "m" };
        Ok(string + &to_cardinal_gender(&BigInt::from(remainder), gender)?)
    }
}

/// Port of `big_number_to_cardinal`. Python asserts `number >= 10**6`.
///
/// `mils` is unbounded (up to 10**30), so it stays a `BigInt` and is split via
/// its decimal string exactly as Python does with `str(mils)[::-1]`.
fn big_number_to_cardinal(number: &BigInt, gender: &str) -> Result<String> {
    debug_assert!(*number >= BigInt::from(1_000_000u32));
    let million = BigInt::from(1_000_000u32);
    let mils = number / &million;
    let remainder = number % &million;

    // `mils >= 1` here, so `to_string()` never carries a sign or leading zero.
    let rev_mils: Vec<char> = mils.to_string().chars().rev().collect();
    let mut triplets: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < rev_mils.len() {
        let end = (i + 3).min(rev_mils.len());
        // Python: int(rev_mils[i:i+3][::-1]) — re-reverse the 3-char window.
        // At most 3 digits, so the u32 parse is provably in range.
        let chunk: String = rev_mils[i..end].iter().rev().collect();
        triplets.push(chunk.parse::<u32>().expect("<=3 decimal digits"));
        i += 3;
    }

    // Python: `len(triplets) > len(EXPONENTS)` raised NotImplementedError
    // here; it is an OverflowError since gladiaio/num2words2#159.
    if triplets.len() > EXPONENTS.len() {
        check_maxval(number, maxval_ceiling())?;
    }

    let mut string = if !remainder.is_zero() {
        let gender = if !remainder.is_one() { gender } else { "m" };
        to_cardinal_gender(&remainder, gender)?
    } else {
        String::new()
    };

    // Python `zip` stops at the shorter sequence; `triplets.len() <=
    // EXPONENTS.len()` is guaranteed above, so it stops at `triplets`.
    for (triplet, exponent) in triplets.iter().zip(EXPONENTS.iter()) {
        let prefix = if *triplet == 0 {
            String::new()
        } else if *triplet == 1 {
            format!("un {}", exponent)
        } else {
            let head = to_cardinal_gender(&BigInt::from(*triplet), "m")? + " ";
            if exponent.ends_with('n') {
                head + &drop_last(exponent, 1) + "in"
            } else {
                head + &drop_last(exponent, 1) + "i"
            }
        };
        if !prefix.is_empty() {
            if !string.is_empty() {
                if string.contains(" e ") {
                    string = format!("{}, {}", prefix, string);
                } else {
                    string = format!("{} e {}", prefix, string);
                }
            } else {
                string = prefix;
            }
        }
    }
    Ok(string)
}

/// Port of `to_cardinal(number, gender="m")`.
///
/// The `int(number) != number` float branch is dead for integer input and is
/// omitted along with `float_to_words`.
fn to_cardinal_gender(number: &BigInt, gender: &str) -> Result<String> {
    if number.is_negative() {
        // Sign is handled *before* the size dispatch, so -10**36 raises
        // OverflowError just like +10**36.
        return Ok(MINUS_PREFIX_WORD.to_string() + &to_cardinal_gender(&(-number), gender)?);
    }
    // Each branch below 10**6 is bounded by its own guard, so the u32
    // narrowing is provably lossless. Above that we stay in BigInt.
    if *number < BigInt::from(20u32) {
        Ok(small_to_cardinal(number.to_u32().expect("< 20"), gender))
    } else if *number < BigInt::from(100u32) {
        Ok(tens_to_cardinal(number.to_u32().expect("< 100"), gender))
    } else if *number < BigInt::from(1000u32) {
        hundreds_to_cardinal(number.to_u32().expect("< 1000"), gender)
    } else if *number < BigInt::from(1_000_000u32) {
        thousands_to_cardinal(number.to_u32().expect("< 10**6"), gender)
    } else {
        big_number_to_cardinal(number, gender)
    }
}

/// Port of `small_to_ordinal`. Python asserts `number < 20`.
fn small_to_ordinal(number: u32, gender: &str, plural: bool) -> String {
    debug_assert!(number < 20);
    let mut string = SMALL_ORDINALS[number as usize].to_string();
    if gender == "f" && string != "zero" {
        if string.ends_with('o') {
            string = drop_last(&string, 1) + "a";
        } else {
            string = drop_last(&string, 1) + "ña";
        }
    }
    // Runs against the *gender-adjusted* string, matching Python's ordering.
    if plural && string != "zero" {
        if string.ends_with('o') {
            string = drop_last(&string, 1) + "i";
        } else if string.ends_with('a') {
            string = drop_last(&string, 1) + "e";
        }
    }
    string
}

/// Port of `big_to_odinal` — Python's spelling, missing the "r". Asserts
/// `number >= 20`.
///
/// Note Python calls `self.to_cardinal(number)` with **no** gender argument,
/// so the stem is always masculine regardless of the `gender` parameter;
/// `gender` only drives the final suffix swap.
fn big_to_odinal(number: &BigInt, gender: &str, plural: bool) -> Result<String> {
    debug_assert!(*number >= BigInt::from(20u32));
    let mut string = to_cardinal_gender(number, "m")?;

    // Adjust diacritics and perform contractions
    if string.ends_with("ëi") || string.ends_with("ei") {
        string = drop_last(&string, 2) + "ei";
    } else if string.ends_with("quattòrze") {
        // Drops "òrze" and re-adds "orze"; the unconditional "eximo" below
        // then yields "...quattorzeeximo". Faithful — see module docs (1).
        string = drop_last(&string, 4) + "orze";
    } else if string.ends_with('î') {
        string = drop_last(&string, 1) + "i";
    } else if matches!(last_char(&string), Some(c) if "oeai".contains(c)) {
        string = drop_last(&string, 1);
    }
    string += "eximo";

    // Additional phonetic adjustments
    if string.ends_with("mieximo") {
        string = drop_last(&string, 5) + "lleximo";
    } else if string.ends_with("oeutteximo") {
        string = drop_last(&string, 10) + "otteximo";
    } else if string.ends_with("eutteximo") {
        string = drop_last(&string, 9) + "otteximo";
    } else if string.ends_with("neuveximo") {
        string = drop_last(&string, 8) + "oveximo";
    }

    // Gender/number adjustment
    if gender == "f" {
        if plural {
            string = drop_last(&string, 1) + "e";
        } else {
            string = drop_last(&string, 1) + "a";
        }
    } else if plural {
        string = drop_last(&string, 1) + "i";
    }
    Ok(string)
}

/// Port of `to_ordinal(number, gender="m", plural=False)`.
///
/// The `number % 1 != 0` float branch is dead for integer input. Unlike most
/// languages, LIJ never calls `verify_ordinal`, so negatives and 0 are fine.
fn to_ordinal_gender(number: &BigInt, gender: &str, plural: bool) -> Result<String> {
    if number.is_negative() {
        return Ok(MINUS_PREFIX_WORD.to_string() + &to_ordinal_gender(&(-number), gender, plural)?);
    }
    if *number < BigInt::from(20u32) {
        Ok(small_to_ordinal(number.to_u32().expect("< 20"), gender, plural))
    } else {
        big_to_odinal(number, gender, plural)
    }
}

/// The `gender=` kwarg, normalised. LIJ only ever tests `gender == "f"` (in
/// `small_to_cardinal`, `small_to_ordinal`, `big_to_odinal`); any other value
/// — "m", an explicit None, an int, an arbitrary string — falls through every
/// `==` test and behaves exactly like the default "m", with no raise anywhere.
fn kw_gender(kw: &Kwargs) -> &'static str {
    if kw.str("gender") == Some("f") {
        "f"
    } else {
        "m"
    }
}

/// Python truthiness of the `plural=` kwarg (`if plural and ...` /
/// `elif plural:` — never an identity or equality test). Absent means the
/// signature default `False`; an explicit `plural=None` is falsy too.
fn kw_truthy(v: Option<&KwVal>) -> bool {
    match v {
        Some(KwVal::Bool(b)) => *b,
        Some(KwVal::Int(i)) => *i != 0,
        Some(KwVal::Str(s)) => !s.is_empty(),
        Some(KwVal::List(l)) => !l.is_empty(),
        Some(KwVal::None) | None => false,
    }
}

/// Python `-value` on the float/Decimal, for the `to_cardinal(-number)` /
/// `to_ordinal(-number)` sign recursion.
fn negated(value: &FloatValue) -> FloatValue {
    match value {
        FloatValue::Float { value, precision } => FloatValue::Float {
            value: -value,
            precision: *precision,
        },
        FloatValue::Decimal { value, precision } => FloatValue::Decimal {
            value: -value.clone(),
            precision: *precision,
        },
    }
}

/// Port of `Num2Word_LIJ.float_to_words(float_number, gender="m",
/// plural=False, ordinal=False)` for a **non-negative, non-whole** value.
///
/// ```python
/// def float_to_words(self, float_number, gender="m", plural=False, ordinal=False):
///     if ordinal:
///         prefix = self.to_ordinal(int(float_number), gender, plural)
///     else:
///         prefix = self.to_cardinal(int(float_number), gender)
///     float_part = str(float_number).split(".")[1]
///     postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
///     return prefix + Num2Word_LIJ.FLOAT_INFIX_WORD + postfix
/// ```
///
/// The sign never reaches this function: both callers (`to_cardinal`,
/// `to_ordinal`) strip it first and recurse, exactly like Python — see the
/// entry hooks. Three things the port depends on:
///
/// 1. **Only the integer prefix is ordinalized** (`ordinal=True` threads
///    `gender`/`plural` into `to_ordinal(int(...))`); the per-digit postfix is
///    always the bare masculine `self.to_cardinal(int(c))` — hence
///    `to_ordinal(3.25)` == "terso virgola doî çinque".
/// 2. **The fractional digits are `str(value).split(".")[1]`** — the exact
///    shortest-repr digits (float) or the exact Decimal digits *with trailing
///    zeros* (Decimal), one word per digit. There is no rounding heuristic and
///    `self.precision`/`precision=` are never read. `float2tuple`
///    reconstructs exactly those digits: its `post`, zero-padded on the left
///    to the repr-derived precision, equals `str(value).split(".")[1]` in
///    every corpus case, and keeping the raw f64 arithmetic preserves the
///    load-bearing artefacts (e.g. `2.675` -> "675").
/// 3. **The infix is glued raw**: `prefix + " virgola " + postfix`, no
///    `title`/`pointword` involvement (`pointword` is never even set — bare
///    `__init__`).
fn float_to_words(value: &FloatValue, gender: &str, plural: bool, ordinal: bool) -> Result<String> {
    let precision = value.precision() as usize;
    let (pre, post) = float2tuple(value);

    // `float_part = str(value).split(".")[1]` — `post` (always < 10**precision,
    // so at most `precision` digits) zero-padded on the left. This carries
    // the Decimal arm's trailing zeros too: Decimal("1.10") -> post 10 ->
    // "10", Decimal("5.00") -> post 0 -> "00".
    let post_str = post.to_string();
    let float_part = format!(
        "{}{}",
        "0".repeat(precision.saturating_sub(post_str.chars().count())),
        post_str
    );

    // `prefix = self.to_ordinal(int(v), gender, plural)` when ordinal, else
    // `self.to_cardinal(int(v), gender)`.
    let prefix = if ordinal {
        to_ordinal_gender(&pre, gender, plural)?
    } else {
        to_cardinal_gender(&pre, gender)?
    };

    // `postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])`
    // — one masculine cardinal per fractional digit.
    let mut digits: Vec<String> = Vec::with_capacity(float_part.chars().count());
    for ch in float_part.chars() {
        let d = ch.to_digit(10).ok_or_else(|| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
        })?;
        digits.push(to_cardinal_gender(&BigInt::from(d), "m")?);
    }
    let postfix = digits.join(" ");

    Ok(format!("{}{}{}", prefix, FLOAT_INFIX_WORD, postfix))
}

pub struct LangLij {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, (&'static str, &'static str)>,
}

impl LangLij {
    pub fn new() -> Self {
        LangLij {
            // Built once here, never per call. These are class-body constants
            // in Python; rebuilding them inside `to_currency` would make the
            // port slower than the interpreter it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }
}

impl Default for LangLij {
    fn default() -> Self {
        LangLij::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `EXPONENTS` names ten
/// triplets above the million, so 10^36 and above raise `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(36))
}

impl Lang for LangLij {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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
        " e"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        to_cardinal_gender(value, "m")
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        to_ordinal_gender(value, "m", false)
    }

    /// Inherited from `Num2Word_EU`: `str(number) + "."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Inherited from `Num2Word_EU`: `self.to_cardinal(val) + ". urtea"` —
    /// the Basque suffix leaks into Ligurian. See module docs.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(self.to_cardinal(value)? + YEAR_SUFFIX)
    }

    /// Port of the non-integral branch of `to_cardinal` +
    /// [`Num2Word_LIJ.float_to_words`]. **LIJ does not use the base float
    /// path** — it never calls `float2tuple`/`pointword`/`self.precision`, and
    /// (since `__init__` is a bare `pass`) `pointword` is never even set, so
    /// the inherited `to_cardinal_float` would be unreachable/broken. Instead:
    ///
    /// ```python
    /// def to_cardinal(self, number, gender="m"):
    ///     if number < 0:
    ///         string = self.MINUS_PREFIX_WORD + self.to_cardinal(-number, gender)
    ///     elif int(number) != number:
    ///         string = self.float_to_words(number, gender)
    ///     ...
    ///
    /// def float_to_words(self, float_number, gender="m", plural=False, ordinal=False):
    ///     prefix = self.to_cardinal(int(float_number), gender)  # ordinal=False here
    ///     float_part = str(float_number).split(".")[1]
    ///     postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
    ///     return prefix + Num2Word_LIJ.FLOAT_INFIX_WORD + postfix
    /// ```
    ///
    /// One consequence the port depends on (the digit/infix mechanics live on
    /// [`float_to_words`]):
    ///
    /// **The sign is handled by `to_cardinal`'s recursion, not by
    /// `float_to_words`.** `int(-0.5) == 0` carries no minus, so relying on
    /// the integer prefix would drop the sign for `|value| < 1`. LIJ instead
    /// prepends `MINUS_PREFIX_WORD` and recurses on `-value` — hence
    /// `-0.5` -> "meno zero virgola çinque". Reproduced by negating the
    /// `FloatValue` and recursing before touching the digits. (Only non-whole
    /// values reach this method, so the numeric `number < 0` test and the
    /// sign-bit `is_negative()` agree — there is no `-0.0` to mis-route.)
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `if number < 0: MINUS_PREFIX_WORD + to_cardinal(-number)`. The
        // negated value is still non-integral, so it re-enters this same
        // method's positive branch. `precision=` never reaches float_to_words,
        // so it is not threaded through.
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.to_cardinal_float(&negated(value), None)?
            ));
        }

        // `float_to_words(value, gender="m")`. `value >= 0` here.
        float_to_words(value, "m", false, false)
    }

    /// `to_cardinal(float/Decimal, gender=...)` — the kwargs entry for the
    /// cardinal float path. Python's `to_cardinal` threads `gender` through
    /// all three branches (sign recursion, whole -> integer grammar,
    /// non-whole -> `float_to_words(number, gender)`); the whole-value test
    /// is the numeric `int(number) != number`, so Decimal("5.00") with
    /// `gender="f"` is just "çinque" and `-0.0` is "zero".
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender = kw_gender(kw);
        if let Some(i) = value.as_whole_int() {
            return to_cardinal_gender(&i, gender);
        }
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                float_to_words(&negated(value), gender, false, false)?
            ));
        }
        float_to_words(value, gender, false, false)
    }

    // `cardinal_float_entry` stays at the trait default: LIJ's `to_cardinal`
    // routes on the numeric `int(number) != number` (whole -> integer
    // grammar, else float_to_words), which is exactly base semantics — and
    // its sign test `number < 0` is numeric too, matching the default's
    // whole-value handling of -0.0 (as_whole_int(-0.0) == 0 -> "zero").

    /// `to_ordinal(float/Decimal)`:
    ///
    /// ```python
    /// def to_ordinal(self, number, gender="m", plural=False):
    ///     if number < 0:
    ///         return self.MINUS_PREFIX_WORD + self.to_ordinal(-number, gender, plural)
    ///     elif number % 1 != 0:
    ///         return self.float_to_words(number, gender, plural, ordinal=True)
    ///     elif number < 20: ...  # whole floats take the integer grammar
    ///     else: ...
    /// ```
    ///
    /// Whole values — `5.0`, `Decimal("5.00")`, `1E+2`, and whole negatives —
    /// land in the ordinal *integer* grammar (`to_ordinal(5.0)` == "quinto",
    /// `to_ordinal(-1000000.0)` == "meno un mioneximo"); checking
    /// `as_whole_int` first reproduces that, and also keeps `-0.0` (numerically
    /// not `< 0`) at plain "zero". Non-whole values go through
    /// `float_to_words(..., ordinal=True)`, which ordinalizes only the integer
    /// prefix: `to_ordinal(2.5)` == "segondo virgola çinque", `to_ordinal(-1.5)`
    /// == "meno primmo virgola çinque".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(n) = value.as_whole_int() {
            // Whole (sign included): identical to the integer entry, since
            // Python's sign recursion re-lands on the same whole-value branch.
            return to_ordinal_gender(&n, "m", false);
        }
        // Non-whole: `number < 0` first (numeric == sign-bit here), then
        // `float_to_words(number, "m", False, ordinal=True)`.
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                float_to_words(&negated(value), "m", false, true)?
            ));
        }
        float_to_words(value, "m", false, true)
    }

    /// `to_ordinal_num(float/Decimal)` — EU's `str(number) + "."`, floats
    /// included: `1.5` -> "1.5.", `-0.0` -> "-0.0.", `1e+16` -> "1e+16.",
    /// `Decimal("1E+2")` -> "1E+2.". `repr_str` is Python's `str(value)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)` — EU's `self.to_cardinal(val) + ". urtea"`,
    /// with `self.to_cardinal` resolving to LIJ's override: whole floats take
    /// the integer grammar, non-whole go through `float_to_words`, and `-0.0`
    /// is "zero. urtea" (numeric sign test). All of that is exactly the
    /// (default) `cardinal_float_entry`, so delegate and suffix.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(self.cardinal_float_entry(value, None)? + YEAR_SUFFIX)
    }

    /// `converter.str_to_number` is Base's bare `Decimal(value)` — LIJ does
    /// not override it. But the very next thing every string mode does is
    /// `self.to_cardinal(number)` (directly, or via `to_year`), and LIJ's
    /// first test there is the comparison `number < 0`, which on a NaN
    /// Decimal raises `decimal.InvalidOperation` — *not* the ValueError that
    /// `int(NaN)` would give in base's integer path. Raising it here (the
    /// parse and the comparison are back-to-back in the dispatcher, with the
    /// same observable outcome) keeps `num2words("NaN", lang="lij")` on
    /// InvalidOperation. "NaN" has no digits, so the dispatcher's
    /// digits-present sentence fallback can never swallow it; NaN-with-payload
    /// strings ("NaN123") fall back to the Python original either way.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                // The message CPython's decimal module gives a NaN comparison.
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    /// `Num2Word_Base.to_fraction`, re-ported here because its negative
    /// branch reads `self.negword` — which `Num2Word_LIJ` never sets (bare
    /// `__init__`, no class attribute), so a sign-negative fraction raises
    /// `AttributeError`. Everything else is the base default verbatim:
    ///
    /// ```python
    /// if denominator == 0: raise ZeroDivisionError("denominator must not be zero")
    /// if denominator == 1 or numerator == 0: return self.to_cardinal(numerator)
    /// is_negative = (numerator < 0) ^ (denominator < 0)
    /// abs_n = abs(int(numerator)); abs_d = abs(int(denominator))
    /// sign = "%s " % self.negword.strip() if is_negative else ""   # <- raises
    /// num_word = self.to_cardinal(abs_n)
    /// den_word = self.to_ordinal(abs_d)
    /// if abs_n != 1: den_word = den_word + "s"
    /// return sign + num_word + " " + den_word
    /// ```
    ///
    /// Ordering matters: `sign` is a conditional *expression*, so `negword`
    /// is only touched when `is_negative` — `-3/-4` (signs cancel) renders
    /// "trei quartos", `0/-5` short-circuits to "zero", and `-5/1`
    /// short-circuits into LIJ's own `to_cardinal` ("meno çinque", via
    /// `MINUS_PREFIX_WORD`). The raise happens *before* `num_word`/`den_word`
    /// are computed, so `-10**36/2` is AttributeError, not OverflowError.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision("denominator must not be zero".into()));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        if is_negative {
            return Err(N2WError::Attribute(
                "'Num2Word_LIJ' object has no attribute 'negword'".into(),
            ));
        }
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();
        let num_word = self.to_cardinal(&abs_n)?;
        let mut den_word = self.to_ordinal(&abs_d)?;
        if !abs_n.is_one() {
            den_word += "s"; // base's naive plural, kept verbatim
        }
        Ok(format!("{} {}", num_word, den_word))
    }

    // ---- grammatical kwargs ----------------------------------------------

    /// `to_cardinal(number, gender="m")` — the only kwarg is `gender`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        to_cardinal_gender(value, kw_gender(kw))
    }

    /// `to_ordinal(number, gender="m", plural=False)`.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender", "plural"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        to_ordinal_gender(value, kw_gender(kw), kw_truthy(kw.get("plural")))
    }

    /// EU's `to_year(val, longval=True)` — `longval` is accepted and then
    /// never read (the body is unconditionally `to_cardinal(val) + ". urtea"`),
    /// so any value, of any type, is a no-op.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.to_year(value)
    }

    // `to_ordinal_num` (EU) and `to_currency` (LIJ) accept no extra kwargs;
    // the trait defaults' empty-bag guard already reproduces Python's
    // TypeError (via the NotImplemented -> Python fallback) for anything else.

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_LIJ` defines `CURRENCY_FORMS`, `CURRENCY_ADJECTIVES`,
    // `pluralize` and `to_currency`. Everything else on the currency path is
    // `Num2Word_Base`'s and is already the trait default:
    //
    //   * `CURRENCY_PRECISION` is never redeclared, so it is Base's empty dict
    //     and `.get(code, 100)` is *always* 100 — hence no `currency_precision`
    //     override. This is why JPY here keeps a 100-cent "sen" subunit
    //     (`12.34 JPY` -> "dozze yen e trentequattro sen") instead of taking
    //     the 0-decimal path, and why `cheque:JPY` prints "56/100".
    //   * `_money_verbose`/`_cents_verbose`/`_cents_terse` are Base's and route
    //     to `self.to_cardinal`, which the trait default already does.
    //   * `to_cheque` is Base's verbatim; `default_to_cheque` reproduces it.
    //     It reads `cr1[-1]` (plural unit) and `_money_verbose` (masculine
    //     `to_cardinal`, no gender threading), and signs with a literal
    //     "MINUS " rather than `negword` — so unlike `to_currency`, cheques on
    //     negative values do *not* raise: `to_cheque(-5.25, "USD")` ==
    //     "MINUS ÇINQUE AND 25/100 DÒLLAI".
    //   * `cardinal_from_decimal` stays at its default: LIJ's `to_currency`
    //     always passes `keep_precision=False`, so fractional cents never
    //     reach it (1.011 USD quantizes to 1.01 -> "un dòllao e un citto").

    fn lang_name(&self) -> &str {
        "Num2Word_LIJ"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_LIJ.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError; every LIJ entry has two forms.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms.get(form).cloned().ok_or_else(tuple_index_error)
    }

    /// Port of `Num2Word_LIJ.to_currency`, which replaces Base's outright.
    ///
    /// Three things make it diverge sharply from `default_to_currency`:
    ///
    /// 1. **`parse_currency_parts(val)` is called with no keyword arguments**,
    ///    so the *module* defaults apply — `is_int_with_cents=True`,
    ///    `keep_precision=False`, `divisor=100`. Base passes
    ///    `is_int_with_cents=False` and branches on `isinstance(val, int)` to
    ///    suppress cents for true ints; LIJ does neither. A plain `int` is
    ///    therefore read as a **count of cents**: `to_currency(100)` is "un
    ///    euro e zero citti" and `to_currency(1)` is "zero euro e un citto",
    ///    not "un euro". The `Int`/`Decimal` split still matters for how the
    ///    value is split, but not for whether cents appear — LIJ always prints
    ///    them, so `has_decimal` is unused here (`Decimal("5")` and
    ///    `Decimal("5.00")` both give "çinque dòllai e zero citti").
    /// 2. **The divisor is hardcoded to 100**, ignoring `CURRENCY_PRECISION`
    ///    entirely — moot for LIJ, whose table is empty, but it means adding a
    ///    3-decimal code to `CURRENCY_FORMS` would silently mis-split it.
    /// 3. **`self.negword` does not exist.** Python's
    ///    `minus_str = "%s " % self.negword.strip() if is_negative else ""` is
    ///    a conditional *expression*, so the attribute is only touched on the
    ///    negative branch. `__init__` is a bare `pass`, `setup()` never runs,
    ///    and there is no class-level fallback — so every negative value dies
    ///    with `AttributeError` while non-negatives sail through.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it to LIJ's own `separator=" e"` before the ported body.
        let separator = separator.unwrap_or(self.default_separator());

        // `parse_currency_parts(val)` — bare call, module defaults. See (1).
        let (left, right, is_negative) = parse_currency_parts(val, true, false, 100);

        // The forms lookup precedes the negword read, so an unknown code
        // raises NotImplementedError even for a negative value: the corpus has
        // `currency:KWD` / arg `-12.34` -> NotImplementedError, not
        // AttributeError.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr2 = &forms.subunit;
        let mut cr1 = forms.unit.clone();

        if adjective {
            if let Some((adj_sg, adj_pl)) = self.currency_adjectives.get(currency) {
                // Python *suffixes* the adjective and rebuilds cr1 as a fresh
                // 2-tuple: `("%s %s" % (cr1[0], adjs[0]), "%s %s" % (cr1[1],
                // adjs[1]))`. Note this is not `currency::prefix_currency`, and
                // it indexes 0 and 1 explicitly rather than mapping the tuple.
                // -> "dozze dòllai americhen e trentequattro citti".
                let sg = cr1.first().ok_or_else(tuple_index_error)?.clone();
                let pl = cr1.get(1).ok_or_else(tuple_index_error)?.clone();
                cr1 = vec![format!("{} {}", sg, adj_sg), format!("{} {}", pl, adj_pl)];
            }
        }

        let gender = if CURRENCIES_FEM.contains(&currency) {
            "f"
        } else {
            "m"
        };

        // See (3). Everything above this line runs first in Python too.
        if is_negative {
            return Err(N2WError::Attribute(
                "'Num2Word_LIJ' object has no attribute 'negword'".into(),
            ));
        }

        // `right` came back with keep_precision=False, so it is a whole number
        // of cents in 0..=99 and carries scale 0.
        let right_int = right.as_bigint_and_exponent().0;

        // Only the *unit* count is gendered; `self.to_cardinal(right)` takes
        // the default gender="m", so 2 GBP-cents is "doî pence", never "doe".
        let money_str = to_cardinal_gender(&left, gender)?;
        let cents_str = if cents {
            to_cardinal_gender(&right_int, "m")?
        } else {
            // Python hardcodes `"%02d" % right` here rather than calling
            // `_cents_terse`, so the width is 2 regardless of the divisor.
            format!("{:0>2}", right_int)
        };

        // Python: "%s%s %s%s %s %s" % (minus_str, money_str, ...) — `minus_str`
        // is provably "" here, since the only branch that sets it raised above.
        Ok(format!(
            "{} {}{} {} {}",
            money_str,
            self.pluralize(&left, &cr1)?,
            separator,
            cents_str,
            self.pluralize(&right_int, cr2)?
        ))
    }
}
