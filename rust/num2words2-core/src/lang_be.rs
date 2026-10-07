//! Port of `lang_BE.py` (Belarusian / беларуская).
//!
//! Registry check: `__init__.py` maps `"be" -> lang_BE.Num2Word_BE()`, so this
//! file ports `Num2Word_BE` — the only class the module defines.
//!
//! Shape: **self-contained**. `Num2Word_BE` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. `cards`/`merge` stay
//! at their trait defaults. The `THOUSANDS` table (keys 1..=10) ends at
//! `10**30` ("нанільён"); Python's `10**33` raised `KeyError: 11`. Fixed
//! (gladiaio/num2words2#159): `maxval()` is 10^33 and `_int2word` raises
//! `OverflowError` for `abs(n) >= 10**33` up front, so the table `KeyError`
//! is unreachable from the public entry points.
//!
//! Inherited from `Num2Word_Base` unchanged, so the trait defaults are right:
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`.
//!     Note this is *not* guarded by `verify_ordinal`, so negatives pass
//!     through verbatim: `to_ordinal_num(-1) == "-1"`.
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` → default
//!     delegates through `&self` and picks up the `to_cardinal` override
//!     below. Belarusian has no special year formatting; `to_year(-500) ==
//!     "мінус пяцьсот"`. The `**kwargs` catch-all swallows *every* kwarg
//!     unread — see [`Lang::to_year_kw`] below.
//!   * `verify_ordinal` raises `TypeError` on negatives → `to_ordinal(-1)`
//!     is `N2WError::Type`, while `to_cardinal(-1)` is fine.
//!   * `is_title` is `False` and `setup()` never changes it, so `title()` is
//!     the identity here. The call is kept where Python has it.
//!   * `to_currency`, `to_cheque` and `_cents_terse` are Base's → the trait
//!     defaults (`currency::default_to_currency` / `default_to_cheque` /
//!     `default_cents_terse`) are exactly right. BE overrides only the data
//!     table plus `pluralize`/`_money_verbose`/`_cents_verbose`.
//!
//! # Grammatical kwargs: `gender=`
//!
//! Both overridden converters take a gender parameter — `to_cardinal(number,
//! gender="m")` and `to_ordinal(number, gender="m")` — dispatched by
//! `num2words(..., gender=...)`. The semantics live entirely in dict lookups:
//!
//!   * `_int2word` only consults `ONES[gender]` for the *final* chunk, and
//!     only when that chunk has a units digit outside the teens. So a bogus
//!     gender is a **lazy** `KeyError`: `to_cardinal(1234, gender="x")` raises
//!     `KeyError: 'x'`, but `to_cardinal(0/11/100, gender="x")` succeed —
//!     zero returns early, teens use `TENS`, and round hundreds never index
//!     `ONES`. Corpus-confirmed. [`Gender::Bad`] carries the key's repr so the
//!     lookup site can raise exactly that.
//!   * `to_ordinal` first runs `verify_ordinal` (negatives raise `TypeError`
//!     *before* the gender can KeyError), then applies the bool handshake
//!     `isinstance(gender, bool) and gender` → `"f"`. `gender=False` is *not*
//!     rewritten and reaches `ONES[False]` → `KeyError: False` when a units
//!     digit needs it.
//!   * `gender=None` is **not** treated as the default: `ONES[None]` →
//!     `KeyError: None` (lazily, as above).
//!   * The feminine/neuter ordinal endings ("f" → "…ая"/"…цяя", "n" →
//!     "…ае"/"…цяе") apply to whatever `lastword` the fallback chain
//!     produced, matched or not — `to_ordinal(2, gender="f") == "дзвая"` is
//!     Python's genuine output ("дзве" survives every arm, then loses one
//!     char to "ая").
//!
//! `to_ordinal_num` and `to_currency` accept no extra kwargs (Base
//! signatures), so their `*_kw` hooks stay at the trait default: any kwarg →
//! NotImplemented → Python raises the original `TypeError`. `to_year` is the
//! opposite: Base's `**kwargs` accepts and *ignores* everything, so
//! `to_year_kw` succeeds for any bag and always renders the default-gender
//! cardinal.
//!
//! # to_ordinal over floats/Decimals
//!
//! `verify_ordinal` is numeric, so whole floats pass it and then flow through
//! the overridden `to_cardinal`'s *string* branch — the ordinal machinery then
//! runs over "пяць коска нуль" and rewrites only the last word:
//! `to_ordinal(5.0) == "пяць коска нулявы"`. Notable consequences, all
//! corpus-confirmed:
//!   * `to_ordinal(1.0) == "коска нулявы"` — the 3-word "адзін коска нуль"
//!     trips the `outwords[-3] in ["адзін", "адна"]` blanking.
//!   * `to_ordinal(-0.0) == "мінус нуль коска нулявы"` — `abs(-0.0) == -0.0`
//!     is numerically true, so negative zero *passes* the negativity check.
//!   * Fractional values raise `TypeError` (`errmsg_floatord`), true
//!     negatives raise `TypeError` (`errmsg_negord`) — first check wins for
//!     `-1.5`.
//!   * `to_ordinal(1e16)` passes verify (it *is* whole) and then dies in
//!     `int('1e+16')` → `ValueError`, exactly like the cardinal path.
//!   * inf/nan die inside `verify_ordinal`'s `int(value)`:
//!     `OverflowError("cannot convert float infinity to integer")` /
//!     `ValueError("cannot convert float NaN to integer")`.
//!
//! # str inputs: inf/nan punt to Python
//!
//! BE does *not* override `str_to_number`; Python's is `Decimal(value)`, which
//! parses `"Infinity"`/`"NaN"` successfully, and the failure only happens
//! later inside BE's `to_cardinal` (`int("Infinity")` → `ValueError`). The
//! Rust dispatcher, however, hard-codes Base semantics for `ParsedNumber::Inf`
//! / `NaN` (`OverflowError` / `ValueError "cannot convert …"`), which is wrong
//! for BE's cardinal (corpus: `num2words("Infinity", lang="be")` →
//! `ValueError`) and unfixable per-mode from this file. So [`Lang::str_to_number`]
//! below returns `NotImplemented` for inf/nan parses — the binding surfaces
//! that as `NotImplementedError` and the dispatcher falls back to the original
//! Python string path, which reproduces every mode's behaviour exactly
//! (cardinal/year `ValueError`, ordinal `OverflowError`, ordinal_num echoes
//! "Infinity"). All other strings pass through untouched.
//!
//! # Currency
//!
//! `Num2Word_BE` descends straight from `Num2Word_Base` (MRO is
//! `BE → Base → object`), **not** from `Num2Word_EUR`, and it declares its own
//! `CURRENCY_FORMS` class attribute. So the `lang_EUR.py` mutation trap does
//! not apply here: `Num2Word_EN.__init__` rewrites *`Num2Word_EUR`'s* dict, and
//! BE's is a different object — verified live with
//! `be.CURRENCY_FORMS is en.CURRENCY_FORMS` → `False`. BE therefore sees
//! exactly the eight codes in its own class body and nothing English adds;
//! GBP/JPY/INR/CNY/CHF/KWD/BHD all raise `NotImplementedError`, as the corpus
//! confirms.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited from Base
//! and are empty (confirmed live), so:
//!   * `currency_adjective` stays at the trait default (`None`) — `adjective=True`
//!     is a silent no-op for every BE code, exactly as in Python.
//!   * `currency_precision` stays at the default 100 for *every* code. BE has no
//!     3-decimal or 0-decimal currency, so `default_to_currency`'s `divisor == 1`
//!     branch is unreachable and KWD/BHD/JPY fail on the forms lookup instead —
//!     which is why they are `NotImplementedError` rows rather than mils rows.
//!
//! Every BE entry carries **three** forms on both sides (`pluralize` indexes
//! 0..=2), unlike the two-form EUR/EN tables. Dropping the third would silently
//! change output: `to_cheque` takes `cr1[-1]`, i.e. index 2.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below is wrong-looking but is
//! exactly what CPython emits; each was verified against the interpreter.
//!
//! 1. ~~**Latin homoglyphs in the tables.**~~ `ords_adjective["дзевяць"]`
//!    ended in a Latin `i` (U+0069) and `TWENTIES_ORD[4][1]` held a Latin
//!    `a` (U+0061), so `to_ordinal(900)` and `to_ordinal(60000)` mixed
//!    scripts. Fixed (gladiaio/num2words2#251): both are Cyrillic.
//! 2. ~~**`to_ordinal(80) == "сямідзясяты"`** ("seventieth").~~ The
//!    `lastword[-9:] == "семдзесят"` tail test also matched "восемдзесят".
//!    Fixed (#251): the tens 50..=90 are whole-word `ords` entries
//!    (пяцідзясяты, шасцідзясяты, сямідзясяты, васьмідзясяты, дзевяносты)
//!    and the tail arm is gone.
//! 3. ~~**`TWENTIES_ORD` substring shadowing.**~~ `"семдзесят"` is a
//!    substring of `"восемдзесят"` and the scan takes the first hit, so
//!    `to_ordinal(80000)` read "восямідзесяцітысячны". Fixed (#251): 80 is
//!    listed before 70. The scan order is still load-bearing.
//! 4. **Big scales lose their ordinal suffix.** The `except KeyError` chain
//!    has no arm matching a `"...даў"` tail (`[-1]` is `ў`, `[-2]` is `а`, and
//!    the `[-3:] == "наў"` test sees `"даў"`), so `"мільярдаў"` falls through
//!    *every* branch unchanged. Hence `to_ordinal(10**10) ==
//!    "дзесяцімільярдаў"` and `to_ordinal(10**11) == "стамільярдаў"` — plain
//!    genitive plurals with no ordinal ending at all. Corpus-confirmed.
//! 5. **`elif lastword[-3:] == "наў"` is dead code.** The preceding
//!    `elif lastword[-1] == "н" or lastword[-2] == "н"` cannot catch `"наў"`
//!    (`[-1]` is `ў`, `[-2]` is `а`), so the `"наў"` arm *is* live for
//!    `"мільёнаў"` → `"мільённы"`. Kept in place; noted because the ordering
//!    reads as if it were unreachable.
//! 6. ~~**`to_ordinal(1234567890)` returned the cardinal verbatim**~~ and
//! 7. ~~**`to_ordinal(50/60/90)` were no-ops**~~: "пяцьдзясят",
//!    "шэсцьдзясят" and "дзевяноста" matched no arm. Fixed with 2 (#251).
//!    Also fixed there: the penultimate word took its oblique stem in every
//!    compound ("ста пяты" for 105); it now does so only before a scale
//!    word it fuses with ("двухтысячны"), so 105 is "сто пяты".
//! 8. `ONES_FEMININE` is defined at module scope in Python and never read
//!    (the class uses `ONES["f"]`). It is omitted here as dead data.
//!
//! # The float / Decimal path
//!
//! BE overrides **`to_cardinal`**, not `to_cardinal_float`, and handles
//! non-integers inline — verified live: `to_cardinal_float` on the class is
//! still `Num2Word_Base`'s. So `base.float2tuple` is **never reached** for BE,
//! and neither is any of its f64 arithmetic:
//!
//! ```python
//! def to_cardinal(self, number, gender="m"):
//!     n = str(number).replace(",", ".")
//!     if "." in n:
//!         ...
//!         left, right = abs_n.split(".")
//!         ...
//!         decimal_part = (ZERO + " ") * leading_zero_count + self._int2word(int(right), gender)
//! ```
//!
//! The spec here is the *decimal string*, so the usual warning is inverted for
//! this language: `float2tuple`'s binary artefacts must **not** be reproduced,
//! because Python never computes them. `str(2.675)` is `'2.675'`, so `right`
//! is `"675"` and the answer is "шэсцьсот семдзесят пяць" — a whole number,
//! not the digit-by-digit spelling Base would emit. Corpus-confirmed. Likewise
//! `str(3.14)` gives `right == "14"` → "чатырнаццаць" (*fourteen*), where Base
//! would say "адзін чатыры" (*one four*).
//!
//! That makes `str(number)` the entire porting problem, so it is reproduced
//! exactly rather than approximated — see [`py_repr_float`] (CPython's
//! `float_repr`) and [`py_str_decimal`] (the decimal spec's
//! `to-scientific-string`). Both were differentially fuzzed against the live
//! interpreter: 4,047 f64 values (random bit patterns, subnormals, ±inf, nan,
//! the e-notation thresholds) and 3,042 `Decimal` literals, byte-identical
//! apart from the negative-zero case noted below.
//!
//! `FloatValue`'s `precision` field is deliberately **unread** on both arms:
//! Python's `precision` only ever feeds `float2tuple`, which BE bypasses. The
//! `precision=` kwarg is likewise a no-op — `num2words` sets
//! `converter.precision` and `Num2Word_BE.to_cardinal` never reads it, so
//! `num2words(1.5, lang='be', precision=3)` is still "адзін коска пяць"
//! (verified live). `precision_override` is therefore ignored, not honoured.
//!
//! # Error variants
//!
//! * `N2WError::Type` — `verify_ordinal` on a negative (deliberate raise).
//! * `N2WError::Overflow` — `abs(n) >= 10**33` (#159). Python crashed there
//!   with a `KeyError` on `THOUSANDS[i]` past index 10; that `N2WError::Key`
//!   arm is now unreachable from the public entry points.
//! * `N2WError::Index` — defensive only; see [`char_from_end`].
//! * `N2WError::Value` — `int()` on a token that is not a decimal integer.
//!   Reachable in normal use: any float whose `repr` goes to e-notation dies
//!   here, so `num2words(1e16, lang='be')` raises `ValueError`, not
//!   `OverflowError`. See [`py_int`].

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, Zero};
use std::collections::HashMap;

const ZERO: &str = "нуль";
const NEGWORD: &str = "мінус";
const POINTWORD: &str = "коска";

/// `ONES["f"]`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0`).
const ONES_F: [&str; 10] = [
    "", "адна", "дзве", "тры", "чатыры", "пяць", "шэсць", "сем", "восем", "дзевяць",
];

/// `ONES["m"]`, keys 1..=9.
const ONES_M: [&str; 10] = [
    "", "адзін", "два", "тры", "чатыры", "пяць", "шэсць", "сем", "восем", "дзевяць",
];

/// `ONES["n"]`, keys 1..=9. Unreachable from the trait (gender is always "m"),
/// kept so the gender plumbing mirrors Python.
const ONES_N: [&str; 10] = [
    "", "адно", "два", "тры", "чатыры", "пяць", "шэсць", "сем", "восем", "дзевяць",
];

/// `TENS`, keys 0..=9 — the 10..19 teens, keyed by the *units* digit.
const TENS: [&str; 10] = [
    "дзесяць",
    "адзінаццаць",
    "дванаццаць",
    "трынаццаць",
    "чатырнаццаць",
    "пятнаццаць",
    "шаснаццаць",
    "сямнаццаць",
    "васямнаццаць",
    "дзевятнаццаць",
];

/// `TWENTIES`, keys 2..=9. Indices 0/1 are absent in Python (guarded `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "дваццаць",
    "трыццаць",
    "сорак",
    "пяцьдзясят",
    "шэсцьдзясят",
    "семдзесят",
    "восемдзесят",
    "дзевяноста",
];

/// `TWENTIES_ORD` — an ordered tuple of `(cardinal, oblique)` pairs.
///
/// **Order is load-bearing**: `to_ordinal` takes the *first* pair whose `.0` is
/// a substring of the word, so "восемдзесят" sits ahead of "семдзесят", which
/// it contains (bug 3, fixed).
const TWENTIES_ORD: [(&str, &str); 8] = [
    ("дваццаць", "дваццаці"),
    ("трыццаць", "трыццаці"),
    ("сорак", "сарака"),
    ("пяцьдзясят", "пяцідзясяці"),
    ("шэсцьдзясят", "шасцідзясяці"),
    // "восемдзесят" contains "семдзесят", and the scan takes the first hit,
    // so 80 must come before 70 (#251).
    ("восемдзесят", "васьмідзесяці"),
    ("семдзесят", "сямідзесяці"),
    ("дзевяноста", "дзевяноста"),
];

/// `HUNDREDS`, keys 1..=9. Index 0 is absent in Python (guarded `n3 > 0`).
const HUNDREDS: [&str; 10] = [
    "",
    "сто",
    "дзвесце",
    "трыста",
    "чатырыста",
    "пяцьсот",
    "шэсцьсот",
    "семсот",
    "восемсот",
    "дзевяцьсот",
];

/// `THOUSANDS`: chunk index → the three plural forms, keys 1..=10 (10^3 up to
/// 10^30). Index 0 is a placeholder — Python has no key 0, and `_int2word`
/// only looks up `i > 0`. Index 11+ is a `KeyError`, which is Belarusian's de
/// facto (and abrupt) MAXVAL.
const THOUSANDS: [[&str; 3]; 11] = [
    ["", "", ""], // absent in Python
    ["тысяча", "тысячы", "тысяч"],                      // 10^3
    ["мільён", "мільёны", "мільёнаў"],                  // 10^6
    ["мільярд", "мільярды", "мільярдаў"],               // 10^9
    ["трыльён", "трыльёны", "трыльёнаў"],               // 10^12
    ["квадрыльён", "квадрыльёны", "квадрыльёнаў"],      // 10^15
    ["квінтыльён", "квінтыльёны", "квінтыльёнаў"],      // 10^18
    ["секстыльён", "секстыльёны", "секстыльёнаў"],      // 10^21
    ["сэптыльён", "сэптыльёны", "сэптыльёнаў"],         // 10^24
    ["актыльён", "актыльёны", "актыльёнаў"],            // 10^27
    ["нанільён", "нанільёны", "нанільёнаў"],            // 10^30
];

/// `Num2Word_BE.CURRENCY_FORMS`, transcribed from the class body.
///
/// Unlike most of this file's tables these are plain literals: every character
/// was checked against the live interpreter and all of them are genuine
/// Cyrillic (U+0400..U+04FF). The Latin-homoglyph typos that infest
/// `ords_adjective` and `TWENTIES_ORD` (see bugs 1) do **not** occur here, so
/// no `\u{...}` escapes are warranted.
///
/// Note `EUR`'s and `KZT`'s three *identical* unit forms — the words are
/// invariant, but the arity still matters because `pluralize` indexes 0..=2 and
/// `to_cheque` reads index 2.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Shared subunit tuples. Python repeats the literals per entry; hoisting
    // them is safe because `CurrencyForms` copies into owned `String`s.
    const KAPEIKA: [&str; 3] = ["капейка", "капейкі", "капеек"];
    const CENT: [&str; 3] = ["цэнт", "цэнты", "цэнтаў"];
    const TYIIN: [&str; 3] = ["тыйін", "тыйіны", "тыйінаў"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "RUB",
        CurrencyForms::new(
            &["расійскі рубель", "расійскія рублі", "расійскіх рублёў"],
            &KAPEIKA,
        ),
    );
    m.insert("EUR", CurrencyForms::new(&["еўра", "еўра", "еўра"], &CENT));
    m.insert(
        "USD",
        CurrencyForms::new(&["долар", "долары", "долараў"], &CENT),
    );
    m.insert(
        "UAH",
        CurrencyForms::new(&["грыўна", "грыўны", "грыўнаў"], &KAPEIKA),
    );
    m.insert("KZT", CurrencyForms::new(&["тэнге", "тэнге", "тэнге"], &TYIIN));
    m.insert(
        "BYN",
        CurrencyForms::new(
            &["беларускі рубель", "беларускія рублі", "беларускіх рублёў"],
            &KAPEIKA,
        ),
    );
    m.insert("UZS", CurrencyForms::new(&["сум", "сумы", "сумаў"], &TYIIN));
    m.insert(
        "PLN",
        CurrencyForms::new(&["злоты", "злотых", "злотых"], &["грош", "грошы", "грошаў"]),
    );
    m
}

/// The form index chosen by `Num2Word_BE.pluralize` — the East-Slavic
/// one/few/many rule.
///
/// ```python
/// if n % 100 < 10 or n % 100 > 20:
///     if n % 10 == 1:      form = 0
///     elif 5 > n % 10 > 1: form = 1
///     else:                form = 2
/// else:
///     form = 2
/// ```
///
/// Callers pass either a 3-digit chunk (`_int2word`) or a whole currency
/// amount (`to_currency`), always non-negative — `default_to_currency` takes
/// `abs()` first. `mod_floor` keeps Python's `%` semantics rather than Rust's
/// remainder regardless, so a negative would still floor the way Python does
/// instead of silently picking a different form.
fn plural_form_index(n: &BigInt) -> usize {
    let m10 = n.mod_floor(&BigInt::from(10));
    let m100 = n.mod_floor(&BigInt::from(100));
    if m100 < BigInt::from(10) || m100 > BigInt::from(20) {
        if m10 == BigInt::from(1) {
            0
        } else if m10 < BigInt::from(5) && m10 > BigInt::from(1) {
            1
        } else {
            2
        }
    } else {
        2
    }
}

/// The `gender` argument as `ONES[gender]` and to_ordinal's suffix logic see
/// it.
///
/// Python threads the raw kwarg value straight into dict lookups and `==`
/// comparisons, so anything that is not exactly `"m"`/`"f"`/`"n"` only fails
/// *lazily*, at the `ONES[gender]` lookup — and only when a units digit in the
/// final chunk actually needs it. [`Gender::Bad`] therefore carries the
/// Python `repr` of the offending key so the lookup site can raise the exact
/// `KeyError` (`KeyError: 'x'`, `KeyError: False`, `KeyError: None`, ...).
#[derive(Debug, Clone, PartialEq)]
enum Gender {
    M,
    F,
    N,
    Bad(String),
}

/// Decode the `gender=` kwarg.
///
/// `ordinal_bool_handshake` reproduces `to_ordinal`'s
/// `if isinstance(gender, bool) and gender: gender = "f"` — note it rewrites
/// only `True`; `False` stays `False` and later hits `ONES[False]`.
/// `to_cardinal` has no such handshake, so its callers pass `false` and
/// `gender=True` dies as `KeyError: True` (lazily, like every bad key).
///
/// A list gender would be `TypeError: unhashable type: 'list'` at the dict
/// lookup; that is unreachable from any real caller, so it punts to the
/// Python fallback rather than modelling a third laziness flavour.
fn gender_from_kw(kw: &Kwargs, ordinal_bool_handshake: bool) -> Result<Gender> {
    let v = match kw.get("gender") {
        Option::None => return Ok(Gender::M), // Python default gender="m"
        Some(v) => v,
    };
    Ok(match v {
        KwVal::Str(s) => match s.as_str() {
            "m" => Gender::M,
            "f" => Gender::F,
            "n" => Gender::N,
            other => Gender::Bad(format!("'{}'", other)),
        },
        KwVal::Bool(true) if ordinal_bool_handshake => Gender::F,
        KwVal::Bool(b) => Gender::Bad(if *b { "True" } else { "False" }.to_string()),
        KwVal::Int(i) => Gender::Bad(i.to_string()),
        KwVal::None => Gender::Bad("None".to_string()),
        KwVal::List(_) => return Err(N2WError::Fallback("kwargs".into())),
    })
}

// --- Python exception encoding -------------------------------------------

fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

fn type_error(msg: String) -> N2WError {
    N2WError::Type(msg)
}

fn value_error(msg: String) -> N2WError {
    N2WError::Value(msg)
}

/// Python's `int(s)`, for the tokens `str(float)` / `str(Decimal)` can yield.
///
/// `parse_bytes` accepts exactly Python's shape for those: an optional `+`/`-`
/// then decimal digits. It diverges from the real `int()` on surrounding
/// whitespace (`int(" 5 ") == 5`) and digit underscores (`int("1_0") == 10`),
/// neither of which any `repr` or `Decimal.__str__` can produce, so no caller
/// on this path can observe it.
///
/// The failure arm is *not* defensive — it is how BE rejects every float whose
/// `repr` went exponential (`int('1e+16')`), plus `inf` and `nan`. Python
/// raises `ValueError`; the message quotes the offending token with `repr`,
/// which for these ASCII tokens is just single quotes.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10).ok_or_else(|| {
        value_error(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Exact test: is `|f|` precisely `(d - 1/2) * 10^scale`?
///
/// That is the midpoint between the two length-k candidates `d-1` and `d`, so
/// a `true` here is the tie CPython's dtoa breaks toward even. Everything is
/// done in `BigInt`: an f64 is exactly `m * 2^e`, so the question
/// `m * 2^(e+1) == (2d - 1) * 10^scale` is decidable with no rounding at all.
/// Both sides are cross-multiplied rather than divided so that negative powers
/// stay integral.
fn is_exact_tie_below(f: f64, d: &BigInt, scale: i32) -> bool {
    let bits = f.abs().to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    // Subnormals have no implicit leading 1 and a fixed exponent.
    let (m, e) = if biased == 0 {
        (BigInt::from(frac), -1074i32)
    } else {
        (BigInt::from(frac | (1u64 << 52)), biased - 1075)
    };

    let (mut ln, mut ld) = (m, BigInt::from(1));
    let p = e + 1;
    if p >= 0 {
        ln <<= p as usize;
    } else {
        ld <<= (-p) as usize;
    }

    let (mut rn, mut rd) = (d * 2 - 1, BigInt::from(1));
    if scale >= 0 {
        rn *= BigInt::from(10).pow(scale as u32);
    } else {
        rd *= BigInt::from(10).pow((-scale) as u32);
    }

    ln * rd == rn * ld
}

/// Does the decimal `digits * 10^scale` read back as exactly `|f|`?
///
/// Rust's `str -> f64` is correctly rounded, the same contract as
/// `_Py_dg_strtod`, so this answers "would dtoa accept this candidate" exactly.
fn roundtrips(digits: &str, scale: i32, f: f64) -> bool {
    match format!("{}e{}", digits, scale).parse::<f64>() {
        Ok(v) => v == f.abs(),
        Err(_) => false,
    }
}

/// Python's `repr(float)` — i.e. `str(float)`, which is what BE's
/// `to_cardinal` calls.
///
/// A faithful transcription of CPython's `float_repr` →
/// `PyOS_double_to_string(v, 'r', 0, Py_DTSF_ADD_DOT_0, NULL)` →
/// `format_float_short`, which is:
///
/// 1. take the shortest round-tripping digit string and its decimal point
///    position `decpt` (`_Py_dg_dtoa` mode 0);
/// 2. `use_exp = decpt <= -4 || decpt > 16`;
/// 3. exponential → `d[0]` `.d[1:]` `e` `±` `%02d`; fixed → pad, and append
///    `.0` when the value is integral (the `ADD_DOT_0` flag).
///
/// Rust's `{:e}` supplies step 1: `LowerExp` for `f64` is shortest
/// round-trip, the same contract as `_Py_dg_dtoa` mode 0, and `d.ddde<exp>`
/// gives `decpt == exp + 1` directly. `{}` (Display) is *not* usable here — it
/// never goes exponential and never appends `.0`, so it would turn
/// `repr(1e16) == '1e+16'` into `"10000000000000000"` (a ValueError silently
/// becoming a number) and `repr(1.0) == '1.0'` into `"1"` (the whole
/// fractional branch silently skipped).
///
/// The step-2 thresholds are load-bearing, not cosmetic: `1e15` has
/// `decpt == 16` and converts fine ("адзін квадрыльён коска нуль"), while
/// `1e16` has `decpt == 17`, renders as `'1e+16'`, and raises `ValueError`.
///
/// `nan` never carries a sign in CPython (`repr(-float('nan')) == 'nan'`),
/// but `-0.0` and `-inf` do.
fn py_repr_float(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    // "-1.234e1" -> sign "-", digits "1234", exp 1. Always well-formed for a
    // finite f64, so the parses cannot fail.
    let s = format!("{:e}", f);
    let (mant, exp) = match s.split_once('e') {
        Some(p) => p,
        None => return s,
    };
    let exp: i32 = match exp.parse() {
        Ok(e) => e,
        Err(_) => return s,
    };
    let (sign, mant) = match mant.strip_prefix('-') {
        Some(m) => ("-", m),
        None => ("", mant),
    };
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut dlen = digits.len() as i32;
    let decpt = exp + 1;

    // Banker's rounding, one layer deeper than usual: it is inside the
    // shortest-digit *generation*, not in a round() call.
    //
    // Rust and CPython both emit a shortest string that round-trips, but when
    // two equally short candidates both round-trip and the value sits exactly
    // between them, they break the tie differently. Rust's `{:e}` rounds half
    // away from zero; `_Py_dg_dtoa` rounds half to even
    // (`if (j1 > 0 || (j1 == 0 && dig & 1))` — round up only if the last digit
    // is odd). So `-1025087167514585.25` is `'-1025087167514585.2'` in Python
    // but `-1025087167514585.3` in Rust — and for BE that is the difference
    // between "коска два" and "коска тры". Ties are not exotic: they hit ~1.5%
    // of uniformly random doubles in the 1e13..1e18 range, where the ulp is a
    // large enough power of two to leave a terminating .5 at the cut.
    //
    // Correcting it needs the tie to be *exact* (checked in integer
    // arithmetic, never in f64) AND the lower candidate to genuinely
    // round-trip. That second test is not redundant: for `2**-24`
    // (= 5.9604644775390625e-08) the value is exactly midway between
    // ...062 and ...063, yet only ...063 round-trips — a power of two has a
    // half-width gap below it, so the interval is asymmetric. Gay's dtoa
    // applies ties-to-even only in the branch where *both* neighbours are
    // acceptable; with one candidate the parity never gets a say and Python
    // keeps the odd ...063.
    let dint = BigInt::parse_bytes(digits.as_bytes(), 10).unwrap_or_else(BigInt::zero);
    if dint.is_odd() && is_exact_tie_below(f, &dint, decpt - dlen) {
        let cand: BigInt = &dint - 1;
        let cs = cand.to_string();
        // The last digit was odd, so the decrement cannot borrow and the digit
        // count is preserved; both guards are belt-and-braces.
        if !cand.is_zero() && cs.len() as i32 == dlen && roundtrips(&cs, decpt - dlen, f) {
            digits = cs;
            dlen = digits.len() as i32;
        }
    }

    if decpt <= -4 || decpt > 16 {
        // Exponential. Note CPython does *not* apply ADD_DOT_0 here, so a
        // one-digit mantissa stays bare: '1e+16', not '1.0e+16'.
        let e = decpt - 1;
        let mut out = String::from(sign);
        out.push_str(&digits[..1]);
        if dlen > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        // "%02d" — at least two digits, but never truncated: 1e-100 -> 'e-100'.
        out.push_str(&format!("{:02}", e.abs()));
        out
    } else if decpt <= 0 {
        // 0.0000ddddd
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= dlen {
        // ddddd000.0 — the ADD_DOT_0 arm. This is why `1.0` keeps its ".0"
        // and so still takes BE's fractional branch ("адзін коска нуль").
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - dlen) as usize))
    } else {
        // ddd.dd
        format!("{}{}.{}", sign, &digits[..decpt as usize], &digits[decpt as usize..])
    }
}

/// Python's `str(Decimal)` — the decimal spec's `to-scientific-string`, i.e.
/// `_pydecimal.Decimal.__str__` with `eng=False`.
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # fixed point
/// else:
///     dotplace = 1                   # scientific
/// ```
///
/// then intpart/fracpart are cut at `dotplace` and an `E±d` tail is appended
/// iff `leftdigits != dotplace`. `context.capitals` defaults to 1, so the
/// exponent marker is an **uppercase** `E` — unlike `repr(float)`'s `e`.
///
/// `BigDecimal` stores `(int_val, scale)` with `value == int_val * 10^-scale`,
/// so Python's `_exp` is `-scale` and `_int` is `|int_val|` — and crucially
/// `from_str` preserves the written scale rather than normalising, which is
/// what keeps `Decimal('1.10')`'s trailing zero alive (`right == "10"` →
/// "дзесяць", not "адзін").
///
/// `BigDecimal`'s own `Display` is *not* usable: it renders `Decimal('0.000')`
/// as `"0"` (dropping the fractional branch entirely) and `Decimal('1E+2')` as
/// `"100"` (turning a ValueError into a number).
///
/// **Negative zero**: `BigInt` has no `-0`, so a `-0`-coefficient `Decimal`
/// cannot reach this function with its sign intact. The float/Decimal binding
/// (`num2words2-py`'s `float_value`) compensates by rerouting any zero
/// Decimal whose source text carries a `-` through the *Float* arm as `-0.0`
/// — behaviourally identical for BE, since `repr(-0.0) == '-0.0'` and an
/// all-zero fraction renders as a single "нуль" whatever its written scale
/// (`'-0.0'` and `'-0.000'` both → "мінус нуль коска нуль"). The string
/// pipeline (`from_string` → `dec_mode`) has no such rescue, so a `"-0.0"`
/// *string* input would lose its negword here where Python keeps it; no
/// corpus row exercises that path, and it is unfixable from this file.
fn py_str_decimal(d: &BigDecimal) -> String {
    let (int_val, scale) = d.as_bigint_and_exponent();
    let exp: i64 = -scale;
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string();
    let dlen = int_digits.len() as i64;
    let leftdigits = exp + dlen;

    let dotplace: i64 = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= dlen {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - dlen) as usize)),
            String::new(),
        )
    } else {
        (
            int_digits[..dotplace as usize].to_string(),
            format!(".{}", &int_digits[dotplace as usize..]),
        )
    };

    let expo = if leftdigits == dotplace {
        String::new()
    } else {
        // Python: ['e', 'E'][context.capitals] + "%+d" % (leftdigits - dotplace)
        let e = leftdigits - dotplace;
        format!("E{}{}", if e < 0 { "-" } else { "+" }, e.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expo)
}

// --- Python string semantics ---------------------------------------------
//
// Every string here is Cyrillic, so byte offsets and character offsets differ.
// Python slices/indexes by *character*; these helpers do the same. Indexing by
// byte would silently corrupt the multi-byte letters.

/// Python's `s[-k]`. Raises `IndexError` when `len(s) < k`.
///
/// Defensive only: the shortest word this can see is 3 characters ("сто",
/// "два", "тры", "сем") and `k` never exceeds 2, so the error arm is
/// unreachable in practice. Modelled rather than `unwrap`ped so a future table
/// edit degrades into Python's exception instead of a panic.
fn char_from_end(chars: &[char], k: usize) -> Result<char> {
    if chars.len() < k || k == 0 {
        return Err(index_error("string index out of range"));
    }
    Ok(chars[chars.len() - k])
}

/// Python's `s[:-k]` — drop the last `k` characters (empty if `k >= len`).
fn drop_last(chars: &[char], k: usize) -> String {
    if chars.len() <= k {
        String::new()
    } else {
        chars[..chars.len() - k].iter().collect()
    }
}

/// Python's `s[-k:]` — the last `k` characters, or all of them if `k >= len`.
fn last_n(chars: &[char], k: usize) -> String {
    chars[chars.len().saturating_sub(k)..].iter().collect()
}

/// Python's `s[: s.rfind(c) + 1]` — keep through the last occurrence of `c`.
///
/// `str.rfind` returns -1 when absent, so Python's slice degrades to `s[:0]`
/// == `""`. Reproduced: every call site has already proven `c` is present, but
/// the fallback is kept faithful rather than assumed away.
fn keep_through_last(chars: &[char], c: char) -> String {
    match chars.iter().rposition(|&x| x == c) {
        Some(i) => chars[..=i].iter().collect(),
        None => String::new(),
    }
}

/// Port of `utils.splitbyx(str(n), 3)` with `format_int=True`.
///
/// Only ever fed `str(abs(n))` here — `to_cardinal`/`_int2word` strip the sign
/// and `verify_ordinal` rejects negatives — so, unlike `lang_PL`, the negative
/// head-chunk hazard is unreachable and the parse cannot fail.
fn splitbyx(n: &str, x: usize) -> Vec<BigInt> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let parse = |i: usize, j: usize| -> BigInt {
        let s: String = chars[i..j.min(length)].iter().collect();
        BigInt::parse_bytes(s.as_bytes(), 10).unwrap_or_else(BigInt::zero)
    };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse(0, start));
        }
        let mut i = start;
        while i < length {
            out.push(parse(i, i + x));
            i += x;
        }
    } else {
        out.push(parse(0, length));
    }
    out
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// `n` is always a chunk in 0..=999 here (`splitbyx` only ever sees `abs(n)`),
/// so `"%03d"`'s sign handling is moot and the `[-3:]` slice is a no-op above
/// 99. The zero-pad is written out by hand rather than via `{:03}` so the
/// result does not depend on `num-bigint`'s Display honouring width flags.
fn get_digits(n: &BigInt) -> [usize; 3] {
    let mag = n.abs().to_string();
    let s = format!("{}{}", "0".repeat(3usize.saturating_sub(mag.len())), mag);
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).unwrap_or(0) as usize;
    }
    a
}

pub struct LangBe {
    /// `self.ords` — full-word cardinal → ordinal overrides. A miss here is
    /// the `KeyError` that drives the whole `except` chain in `to_ordinal`,
    /// so this table's *absences* are as load-bearing as its entries.
    ords: HashMap<&'static str, &'static str>,
    /// `self.ords_adjective` — cardinal → oblique/adjectival stem, used both
    /// for the penultimate word and (via `lastword[:-3]`) to build "<n>соты".
    ords_adjective: HashMap<&'static str, &'static str>,
    /// `self.CURRENCY_FORMS`. Built once here rather than per call: the
    /// currency hooks only ever read it, and rebuilding a `HashMap` of owned
    /// `String`s on every `to_currency` is what made an earlier revision of
    /// this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangBe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangBe {
    pub fn new() -> Self {
        let mut ords: HashMap<&'static str, &'static str> = HashMap::new();
        for (k, v) in [
            ("нуль", "нулявы"),
            ("адзін", "першы"),
            ("два", "другі"),
            ("тры", "трэці"),
            ("чатыры", "чацвёрты"),
            ("пяць", "пяты"),
            ("шэсць", "шосты"),
            ("сем", "сёмы"),
            ("восем", "восьмы"),
            ("дзевяць", "дзявяты"),
            // The tens 50..=90 had no entry and fell through the tail
            // matching below: 50/60/90 came back as the cardinal and 80 as
            // 70's "сямідзясяты" (gladiaio/num2words2#251).
            ("пяцьдзясят", "пяцідзясяты"),
            ("шэсцьдзясят", "шасцідзясяты"),
            ("семдзесят", "сямідзясяты"),
            ("восемдзесят", "васьмідзясяты"),
            ("дзевяноста", "дзевяносты"),
            ("сто", "соты"),
            ("тысяча", "тысячны"),
        ] {
            ords.insert(k, v);
        }

        let mut ords_adjective: HashMap<&'static str, &'static str> = HashMap::new();
        for (k, v) in [
            ("адзін", "адна"),
            ("адна", "адна"),
            ("дзве", "двух"),
            ("тры", "трох"),
            ("чатыры", "чатырох"),
            ("пяць", "пяці"),
            ("шэсць", "шасці"),
            ("сем", "сямі"),
            ("восем", "васьмі"),
            ("дзевяць", "дзевяці"),
            ("сто", "ста"),
        ] {
            ords_adjective.insert(k, v);
        }

        LangBe {
            ords,
            ords_adjective,
            currency_forms: build_currency_forms(),
        }
    }

    /// `THOUSANDS[i]`, raising `KeyError` outside 1..=10.
    fn thousands_at(&self, i: usize) -> Result<&'static [&'static str; 3]> {
        if i == 0 || i > 10 {
            return Err(key_error(i.to_string()));
        }
        Ok(&THOUSANDS[i])
    }

    /// `ONES[gender]`. Reached only for the final chunk's units digit, so a
    /// bad gender KeyErrors exactly as lazily as Python's dict lookup does:
    /// `to_cardinal(0, gender="x")` succeeds, `to_cardinal(1, gender="x")`
    /// raises `KeyError: 'x'`.
    fn ones(gender: &Gender) -> Result<&'static [&'static str; 10]> {
        match gender {
            Gender::M => Ok(&ONES_M),
            Gender::F => Ok(&ONES_F),
            Gender::N => Ok(&ONES_N),
            Gender::Bad(repr) => Err(key_error(repr.clone())),
        }
    }

    /// Port of `Num2Word_BE.pluralize`, for the `THOUSANDS` scale words.
    ///
    /// Shadows the `Lang::pluralize` hook of the same name — inherent methods
    /// win method resolution, and the argument types differ anyway, so the
    /// `_int2word` call site below binds here while `default_to_currency` binds
    /// to the trait. Both defer to [`plural_form_index`], so there is exactly
    /// one copy of the rule. This one is infallible: `THOUSANDS` rows are fixed
    /// 3-arrays, so the index can never be out of range.
    fn pluralize(&self, n: &BigInt, forms: &[&'static str; 3]) -> String {
        forms[plural_form_index(n)].to_string()
    }

    /// Port of `Num2Word_BE._int2word`.
    ///
    /// Note the sign is handled *here* (not in `to_cardinal` as in `lang_PL`):
    /// `" ".join([self.negword, self._int2word(abs(n), gender)])`.
    fn int2word(&self, n: &BigInt, gender: &Gender) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_negative() {
            return Ok(format!("{} {}", NEGWORD, self.int2word(&n.abs(), gender)?));
        }
        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3);
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                words.push(HUNDREDS[n3].to_string());
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1].to_string());
            } else if n1 > 0 {
                // Thousands (i == 1) are feminine; every higher scale is
                // masculine; only the final chunk honours the caller's gender
                // — which is also the only place a bad gender can KeyError.
                let ones = match i {
                    0 => Self::ones(gender)?,
                    1 => &ONES_F,
                    _ => &ONES_M,
                };
                words.push(ones[n1].to_string());
            }

            if i > 0 {
                let forms = self.thousands_at(i)?;
                words.push(self.pluralize(x, forms));
            }
        }

        Ok(words.join(" "))
    }

    /// Port of `Num2Word_BE.to_cardinal(number, gender="m")` over the
    /// already-stringified input — i.e. everything after `n = str(number)`.
    ///
    /// The integer trait entry point does not route through here: `str(int)`
    /// can never contain a ".", so it always lands in the `else` branch, and
    /// `to_cardinal` below calls `int2word` directly (as it did before this
    /// phase). The `else` branch is kept anyway because `repr` reaches it for
    /// real float input — `1e16` stringifies to `'1e+16'`, which has no dot and
    /// dies in `int()`.
    fn to_cardinal_str(&self, number_str: &str, gender: &Gender) -> Result<String> {
        // n = str(number).replace(",", ".")
        // A no-op for repr/Decimal output (neither ever emits a comma); it
        // exists for the str-input callers Python allows. Kept verbatim.
        let n = number_str.replace(',', ".");

        if !n.contains('.') {
            // else: return self._int2word(int(n), gender)
            return self.int2word(&py_int(&n)?, gender);
        }

        let is_negative = n.starts_with('-');
        // abs_n = n[1:] if is_negative else n — Python slices by character.
        let abs_n: String = if is_negative {
            n.chars().skip(1).collect()
        } else {
            n.clone()
        };

        // left, right = abs_n.split(".")
        // Tuple unpacking, so >1 dot is a ValueError rather than a silent
        // take-the-first-two. Unreachable from repr/Decimal, modelled anyway.
        let parts: Vec<&str> = abs_n.split('.').collect();
        if parts.len() != 2 {
            return Err(value_error(
                "too many values to unpack (expected 2)".to_string(),
            ));
        }
        let (left, right) = (parts[0], parts[1]);

        // if set(right) == {"0"}: leading_zero_count = 0
        // else: leading_zero_count = len(right) - len(right.lstrip("0"))
        //
        // The guard is what stops an all-zero fraction from being spelled out
        // as "нуль" repeated: str(1.0) -> right "0" -> count 0 -> int("0") is
        // 0 -> "адзін коска нуль", not "адзін коска нуль нуль".
        // `set("") == set()`, not `{"0"}`, so an empty `right` takes the else
        // arm and then dies in int("") — faithful, though unreachable.
        let leading_zero_count = if !right.is_empty() && right.chars().all(|c| c == '0') {
            0
        } else {
            right.chars().count() - right.trim_start_matches('0').chars().count()
        };

        // Python builds `decimal_part` on its own line, before `result`, so
        // int(right) is evaluated — and can raise — before int(left). Keep the
        // two as separate statements or `1.5e+16` would report '1' instead of
        // '5e+16' in its ValueError.
        let decimal_part = format!(
            "{}{}",
            format!("{} ", ZERO).repeat(leading_zero_count),
            self.int2word(&py_int(right)?, gender)?
        );

        let mut result = format!(
            "{} {} {}",
            self.int2word(&py_int(left)?, gender)?,
            POINTWORD,
            decimal_part
        );

        // `left` and `right` are already sign-free, so int2word cannot prepend
        // its own negword here — the minus is reattached exactly once.
        if is_negative {
            result = format!("{} {}", NEGWORD, result);
        }
        Ok(result)
    }

    /// The `except KeyError` chain of `to_ordinal` — the fallback applied when
    /// `lastword` is not a key of `self.ords`.
    ///
    /// Returns `lastword` **unchanged** when no arm matches; that is not an
    /// oversight but the source of bug 4 (see module docs).
    fn ordinal_fallback(&self, lastword: &str) -> Result<String> {
        let lw: Vec<char> = lastword.chars().collect();

        let head = drop_last(&lw, 3);
        if let Some(stem) = self.ords_adjective.get(head.as_str()) {
            // Python: self.ords_adjective.get(lastword[:-3], lastword) + "соты"
            // — the `in` test above guarantees the default never applies.
            return Ok(format!("{}соты", stem));
        }

        if last_n(&lw, 7) == "дзесяць" {
            return Ok("дзясяты".to_string());
        }

        let m1 = char_from_end(&lw, 1)?;
        let m2 = char_from_end(&lw, 2)?;

        if m1 == 'ь' || m2 == 'ц' {
            return Ok(format!("{}ты", drop_last(&lw, 2)));
        }

        if m1 == 'к' {
            // `str.replace` is global in both languages: "сорак" → "сарак".
            return Ok(format!("{}авы", lastword.replace('о', "а")));
        }

        if m2 == 'ч' || m1 == 'ч' {
            // Python nests two *sequential* ifs here, and the second tests the
            // string the first may have just rewritten. Not an if/elif.
            let mut lw2 = lastword.to_string();
            if char_from_end(&lw2.chars().collect::<Vec<_>>(), 2)? == 'ч' {
                let c: Vec<char> = lw2.chars().collect();
                lw2 = format!("{}ны", drop_last(&c, 1));
            }
            if char_from_end(&lw2.chars().collect::<Vec<_>>(), 1)? == 'ч' {
                lw2 = format!("{}ны", lw2);
            }
            return Ok(lw2);
        }

        if m1 == 'н' || m2 == 'н' {
            return Ok(format!("{}ны", keep_through_last(&lw, 'н')));
        }

        // Live despite appearances: "наў" has neither 'н' at [-1] nor at [-2],
        // so the arm above cannot swallow it. See bug 5.
        if last_n(&lw, 3) == "наў" {
            return Ok(format!("{}ны", keep_through_last(&lw, 'н')));
        }

        if m1 == 'д' || m2 == 'д' {
            return Ok(format!("{}ны", keep_through_last(&lw, 'д')));
        }

        // No arm matched — Python leaves `lastword` as-is. Bug 4.
        Ok(lastword.to_string())
    }

    /// Port of `Num2Word_BE.to_ordinal(number, gender="m")`, integer input.
    ///
    /// The trait entry point pins `gender` to `M` (see [`Lang::to_ordinal`]);
    /// `to_ordinal_kw` widens it. The bool→"f" handshake happens in
    /// [`gender_from_kw`], before this is called.
    fn to_ordinal_gender(&self, number: &BigInt, gender: &Gender) -> Result<String> {
        // verify_ordinal: ints always pass the float check; negatives raise.
        // Order matters: this precedes the cardinal, so `to_ordinal(-5,
        // gender="x")` is TypeError, not KeyError. Corpus-confirmed.
        if number.is_negative() {
            return Err(type_error(format!(
                "Cannot treat negative num {} as ordinal.",
                number
            )));
        }
        let cardinal = self.int2word(number, gender)?;
        self.ordinal_words(&cardinal, gender)
    }

    /// Everything in `Num2Word_BE.to_ordinal` after the
    /// `outwords = self.to_cardinal(number, gender).split(" ")` line.
    ///
    /// Split out because the float/Decimal entry runs the same machinery over
    /// the *string-derived* cardinal ("пяць коска нуль" → "пяць коска
    /// нулявы"), where the last word is the fraction's tail, not the number's.
    fn ordinal_words(&self, cardinal: &str, gender: &Gender) -> Result<String> {
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        let lastword = outwords[outwords.len() - 1].to_lowercase();

        // Python wraps the next three statements in one `try`. Only
        // `self.ords[lastword]` can raise KeyError, so these two mutations
        // always land, whether or not the lookup succeeds.
        // Only a scale word fuses with the words before it ("двухтысячны"),
        // so only then does the penultimate word take its oblique stem. In
        // a plain compound only the last word is ordinal: 105 is "сто пяты",
        // not "ста пяты" (#251).
        let is_scale = (1..=10).any(|i| lastword.contains(THOUSANDS[i][0]))
            || lastword.contains("тысяч");
        if outwords.len() > 1 && is_scale {
            let i2 = outwords.len() - 2;
            if let Some(adj) = self.ords_adjective.get(outwords[i2].as_str()) {
                outwords[i2] = adj.to_string();
            } else if outwords[i2] == "дзесяць" {
                let c: Vec<char> = outwords[i2].chars().collect();
                outwords[i2] = format!("{}і", drop_last(&c, 1));
            }
        }
        // Python nests this as `if len(outwords) == 3: if outwords[-3] in [...]`.
        // Flattened to one condition (exactly equivalent; the nesting carries no
        // else-branch) so clippy::collapsible_if stays quiet.
        if outwords.len() == 3 && (outwords[0] == "адзін" || outwords[0] == "адна") {
            outwords[0] = String::new();
        }

        let mut lastword = match self.ords.get(lastword.as_str()) {
            Some(o) => o.to_string(),
            None => self.ordinal_fallback(&lastword)?,
        };

        // Python compares the (possibly bool/None) gender with `==`, so a
        // `Bad` gender that survived this far simply matches neither branch —
        // `to_ordinal(0, gender="x") == "нулявы"`, masculine by default.
        if *gender == Gender::F {
            let c: Vec<char> = lastword.chars().collect();
            if last_n(&c, 2) == "ці" {
                lastword = format!("{}цяя", drop_last(&c, 2));
            } else {
                lastword = format!("{}ая", drop_last(&c, 1));
            }
        }
        if *gender == Gender::N {
            let c: Vec<char> = lastword.chars().collect();
            let t = last_n(&c, 2);
            if t == "ці" || t == "ца" {
                lastword = format!("{}цяе", drop_last(&c, 2));
            } else {
                lastword = format!("{}ае", drop_last(&c, 1));
            }
        }

        let n = outwords.len();
        outwords[n - 1] = self.title(&lastword);
        if outwords.len() == 2 && outwords[0].contains("адна") {
            // Collapse "адна X" → "X": the ordinal already carries the "one".
            outwords[0] = outwords[1].clone();
            outwords.pop();
        }

        // `any(...)` is order-independent, so a plain scan of THOUSANDS[1..=10]
        // is faithful. The *inner* TWENTIES_ORD scan below is not (bug 3).
        let last = outwords[outwords.len() - 1].clone();
        let scale_tail = (1..=10).any(|i| last.contains(THOUSANDS[i][0])) || last.contains("тысяч");
        if outwords.len() > 1 && scale_tail {
            let mut new_outwords: Vec<String> = Vec::new();
            for w in outwords.iter() {
                // First match wins, hence 80 is listed before 70.
                let replacement = TWENTIES_ORD.iter().find(|x| w.contains(x.0));
                match replacement {
                    Some((from, to)) => new_outwords.push(w.replace(from, to)),
                    None => new_outwords.push(w.clone()),
                }
            }
            outwords = vec![new_outwords.concat()];
        }

        Ok(outwords.join(" ").trim().to_string())
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangBe {
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

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "коска"
    }

    /// Port of `Num2Word_BE.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and branches on `"." in n`; `str(int)`
    /// never contains one, so integers always take the `else` branch:
    /// `self._int2word(int(n), gender)`. The float branch (`pointword`,
    /// leading-zero padding) is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, &Gender::M)
    }

    /// Port of `Num2Word_BE.to_ordinal`, with Python's default `gender="m"`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.to_ordinal_gender(value, &Gender::M)
    }

    /// `to_ordinal(float/Decimal)` — Base's `verify_ordinal`, then BE's own
    /// `to_cardinal` string branch, then the last-word ordinal machinery.
    ///
    /// `verify_ordinal` is numeric, not a type check:
    ///
    /// ```python
    /// if not value == int(value): raise TypeError(errmsg_floatord % value)
    /// if not abs(value) == value: raise TypeError(errmsg_negord % value)
    /// ```
    ///
    /// so whole floats *pass* and get their ".0" tail ordinalised
    /// (`to_ordinal(5.0) == "пяць коска нулявы"`), `-0.0` passes both checks
    /// (`abs(-0.0) == -0.0` numerically) and keeps its negword, fractional
    /// values are the floatord `TypeError`, and true negatives the negord one
    /// — first check wins for `-1.5`. inf/nan die inside `int(value)` itself.
    /// `%s % value` is `str(value)`, i.e. [`py_repr_float`]/[`py_str_decimal`].
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                if f.is_nan() {
                    return Err(value_error(
                        "cannot convert float NaN to integer".to_string(),
                    ));
                }
                if f.is_infinite() {
                    return Err(N2WError::Overflow(
                        "cannot convert float infinity to integer".to_string(),
                    ));
                }
                if f.fract() != 0.0 {
                    return Err(type_error(format!(
                        "Cannot treat float {} as ordinal.",
                        py_repr_float(*f)
                    )));
                }
                // Numeric comparison: -0.0 == abs(-0.0), so `< 0.0` (not the
                // sign bit) is the faithful test here.
                if *f < 0.0 {
                    return Err(type_error(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_repr_float(*f)
                    )));
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                if !d.is_integer() {
                    return Err(type_error(format!(
                        "Cannot treat float {} as ordinal.",
                        py_str_decimal(d)
                    )));
                }
                // BigDecimal has no -0, so is_negative() is exactly Python's
                // numeric `abs(value) != value` for the values that reach us
                // (the binding reroutes signed-zero Decimals as Float -0.0).
                if d.is_negative() {
                    return Err(type_error(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_str_decimal(d)
                    )));
                }
            }
        }

        // self.to_cardinal(number, gender) — the string branch, then the
        // ordinal rewrite of its final word. Whole Decimals like 1E+2 pass
        // verify but still die in int('1E+2') here, matching Python.
        let n = match value {
            FloatValue::Float { value, .. } => py_repr_float(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        let cardinal = self.to_cardinal_str(&n, &Gender::M)?;
        self.ordinal_words(&cardinal, &Gender::M)
    }

    /// BE inherits Base's `str_to_number` (`Decimal(value)`), which parses
    /// "Infinity"/"NaN" *successfully* — the failure Python shows for
    /// `num2words("Infinity", lang="be")` happens later, per mode, and is
    /// served natively by [`Lang::inf_result`] / [`Lang::nan_result`] below.
    /// So the parse itself just passes the `ParsedNumber::Inf`/`NaN` through.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `-Infinity` reached BE. Behaviour is per mode,
    /// exactly as the original converter produces it:
    ///
    ///   * `cardinal` / `year` feed the raw token to `int("Infinity")` inside
    ///     BE's own `to_cardinal` → `ValueError`.
    ///   * `ordinal` first runs Base's `verify_ordinal`, whose
    ///     `int(Decimal('Infinity'))` raises `OverflowError`.
    ///   * `ordinal_num` inherits Base's identity `to_ordinal_num`, so the
    ///     value is echoed unchanged (`str(Decimal('Infinity'))`).
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        let token = if negative { "-Infinity" } else { "Infinity" };
        match to {
            "ordinal" => Err(N2WError::Overflow(
                "cannot convert Infinity to integer".to_string(),
            )),
            "ordinal_num" => Ok(token.to_string()),
            // cardinal / year (and any other int()-ing mode): ValueError.
            _ => Err(value_error(format!(
                "invalid literal for int() with base 10: '{}'",
                token
            ))),
        }
    }

    /// `Decimal('NaN')` reached BE. `cardinal`/`year` die in `int("NaN")`
    /// (`ValueError`); `ordinal`'s `verify_ordinal` does `int(Decimal('NaN'))`,
    /// also `ValueError`; `ordinal_num` echoes the value unchanged.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal" => Err(value_error("cannot convert NaN to integer".to_string())),
            "ordinal_num" => Ok("NaN".to_string()),
            _ => Err(value_error(
                "invalid literal for int() with base 10: 'NaN'".to_string(),
            )),
        }
    }

    /// `to_cardinal(number, gender="m")` with an explicit kwargs bag.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        // No bool handshake on the cardinal side: gender=True is ONES[True],
        // a lazy KeyError like any other bad key.
        let gender = gender_from_kw(kw, false)?;
        self.int2word(value, &gender)
    }

    /// `to_ordinal(number, gender="m")` with an explicit kwargs bag.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender = gender_from_kw(kw, true)?;
        self.to_ordinal_gender(value, &gender)
    }

    /// Base's `to_year(self, value, **kwargs)`: the catch-all accepts *any*
    /// kwargs and reads none of them, then returns `self.to_cardinal(value)`
    /// — BE's override with its default `gender="m"`. So
    /// `to_year(5, gender="f")` is "пяць", not "пяць" feminised and not a
    /// TypeError; deliberately no `kw.only` guard.
    fn to_year_kw(&self, value: &BigInt, _kw: &Kwargs) -> Result<String> {
        self.int2word(value, &Gender::M)
    }

    /// `to_cardinal(float/Decimal, gender=...)` — same string branch as
    /// [`Lang::to_cardinal_float`], with the caller's gender threaded into
    /// both `int(left)` and `int(right)` renderings, exactly as Python's
    /// single `gender` parameter reaches both `_int2word` calls.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender = gender_from_kw(kw, false)?;
        let n = match value {
            FloatValue::Float { value, .. } => py_repr_float(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        self.to_cardinal_str(&n, &gender)
    }

    /// The float/Decimal cardinal surface.
    ///
    /// BE does not override `to_cardinal_float`; `num2words` hands the float
    /// straight to `Num2Word_BE.to_cardinal(number)`, whose first act is
    /// `str(number)`. So this hook exists only to re-derive that string and
    /// re-enter the real method — the Base implementation this replaces
    /// (`float2tuple` + digit-by-digit spelling) is never executed for BE.
    ///
    /// `gender` is "m": the dispatcher calls `to_cardinal(number)` positionally
    /// and Python's default applies.
    ///
    /// `_precision_override` is ignored on purpose. `num2words` assigns
    /// `converter.precision` for the `precision=` kwarg, but BE's `to_cardinal`
    /// reads only the string — `num2words(1.5, lang='be', precision=3)` is
    /// "адзін коска пяць", unchanged (verified live). Honouring it here would
    /// invent behaviour Python does not have.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // The two arms are not interchangeable: `str` of a float is repr's
        // shortest round-trip (98746251323029.99 -> '98746251323029.98', issue
        // #603), while `str` of a Decimal is exact and keeps its written scale.
        let n = match value {
            FloatValue::Float { value, .. } => py_repr_float(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        self.to_cardinal_str(&n, &Gender::M)
    }

    // to_ordinal_num: BE does not override Num2Word_Base.to_ordinal_num, which
    // returns the value unchanged → the trait default is correct, including
    // the unguarded negative path (to_ordinal_num(-1) == "-1").
    //
    // to_year: BE does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal → the trait default is correct.

    // ---- currency -------------------------------------------------------
    //
    // BE inherits `to_currency`, `to_cheque` and `_cents_terse` from
    // `Num2Word_Base`, so those trait defaults stand. Only the forms table, the
    // class name, the plural rule and the two gendered verbose hooks are
    // BE's own. `currency_adjective`/`currency_precision` are deliberately not
    // overridden: BE's inherited dicts are empty (see the module docs).

    fn lang_name(&self) -> &str {
        "Num2Word_BE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_BE.pluralize` — the trait-facing half; see [`plural_form_index`].
    ///
    /// Python indexes the tuple directly, so a form list shorter than the
    /// chosen index raises `IndexError`. Every BE entry has exactly three forms
    /// on both sides, so that is unreachable — but it is mapped to
    /// `N2WError::Index` rather than panicking, so the exception *type* survives
    /// if the table is ever edited down.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(plural_form_index(n))
            .cloned()
            .ok_or_else(|| index_error("tuple index out of range"))
    }

    /// Port of `Num2Word_BE._money_verbose`: the hryvnia is feminine, every
    /// other unit masculine.
    ///
    /// Calls `_int2word` directly, as Python does — *not* `to_cardinal`, which
    /// would pin the gender back to "m" and lose the distinction
    /// (`to_currency(1.0, "UAH")` → "адна грыўна", not "адзін грыўна").
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let gender = if currency == "UAH" { Gender::F } else { Gender::M };
        self.int2word(number, &gender)
    }

    /// Port of `Num2Word_BE._cents_verbose`.
    ///
    /// The gender set differs from `_money_verbose`'s: the *subunit* is
    /// feminine for UAH, RUB and BYN alike (all three use "капейка"), even
    /// though RUB's and BYN's units are masculine rubles. So
    /// `to_currency(1.01, "RUB")` is "адзін расійскі рубель, адна капейка" —
    /// masculine unit, feminine cent, in one string.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let gender = match currency {
            "UAH" | "RUB" | "BYN" => Gender::F,
            _ => Gender::M,
        };
        self.int2word(number, &gender)
    }
}
