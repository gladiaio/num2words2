//! Port of `lang_RU.py` (direct `Num2Word_Base` subclass).
//!
//! RU is a **self-contained** language: it overrides `to_cardinal` with its own
//! chunk-based algorithm and never touches `cards`/`merge`/`splitnum`. Only
//! `verify_ordinal` and `to_year` are inherited from `base.py`.
//!
//! The module's data model is a three-level "classifier":
//!
//! ```text
//! {n: [case_1 .. case_6]}
//! case:   text | [gender_m, gender_f, gender_n, plural]
//! gender: text | [animate, inanimate]
//! ```
//!
//! `case_classifier_element` walks that tree, short-circuiting the moment it
//! hits a plain string. `C`/`G` below mirror the two "text or nested" unions
//! exactly, because collapsing them would change which level the walk stops at.
//!
//! Only the `n`(ominative)/singular/masculine/animate defaults are reachable
//! from the plain entry points, but `to_cardinal`/`to_ordinal` accept
//! `case=`/`plural=`/`gender=`/`animate=` kwargs (see `to_cardinal_kw`, and
//! `to_cardinal_float_kw` for float/Decimal input, where the "." branch
//! ignores them), and
//! the ordinal chunk helpers internally request other cases and genders
//! (`case="g"`, `gender="f"`, `gender="n"`), so the tables must be complete.
//!
//! Kwarg quirks reproduced from the Python (see `opts_from_kwargs`):
//!
//! * `case` is resolved on *every* rendering path (`CASE_INDEXES[case]` runs
//!   inside `case_classifier_element` and `get_thousands_elements`, and at
//!   least one of those is reached for every input), so an unknown case is a
//!   `KeyError` — but only *after* `verify_ordinal`, so `to_ordinal(-5,
//!   case="xx")` is TypeError, not KeyError, exactly as the kwargs corpus
//!   records.
//! * `gender` is looked up **lazily**: `GENDER_PLURAL_INDEXES[gender]` only
//!   runs when a case slot is a `[m, f, n, plural]` list *and* `plural` is
//!   falsy. `to_cardinal(5, gender="xx")` is "пять" (ONES[5] is a plain
//!   string per case); `to_cardinal(1, gender="xx")` is KeyError.
//! * `plural` and `animate` are used as `if plural:` / `if animate:` —
//!   Python truthiness, not strict bools.
//!
//! Float/Decimal routing (`cardinal_float_entry` and friends): RU's
//! `to_cardinal` stringifies the input and branches on `"." in n`, so *every*
//! float keeps its decimal tail — `to_cardinal(5.0)` is "пять целых ноль
//! десятых", never the base whole-value integer route. `to_ordinal` /
//! `to_ordinal_num` instead run base `verify_ordinal` then `int(number)`, so
//! whole floats ordinalize ("пятый") and non-whole/negative ones are
//! TypeError. `to_year` is base's, i.e. RU's own `to_cardinal`.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Classifier machinery (Python: case_classifier_element / get_num_element)
// ---------------------------------------------------------------------------

/// A gender slot: plain text, or Python's `[animate, inanimate]` pair.
#[derive(Clone)]
enum G {
    S(String),
    A(String, String),
}

/// A case slot: plain text, or Python's `[m, f, n, plural]` list.
#[derive(Clone)]
enum C {
    S(String),
    G([G; 4]),
}

/// Python's `[case_1 .. case_6]`: n, g, d, a, i, p.
type Classifier = [C; 6];

/// The `gender=` kwarg, resolved *lazily* like Python's
/// `GENDER_PLURAL_INDEXES[gender]`: the dict lookup only runs when a case
/// slot is a `[m, f, n, plural]` list and `plural` is falsy, so an invalid
/// gender is inert on inputs whose case slots are plain strings
/// (`to_cardinal(5, gender="xx")` == "пять" in Python).
#[derive(Clone, Copy)]
enum GenderArg<'a> {
    /// GENDER_PLURAL_INDEXES: m=0, f=1, n=2, p(lural)=3
    Idx(usize),
    /// An unresolvable value; the KeyError (or TypeError for an unhashable
    /// list) is raised only if the lookup is actually reached.
    Bad(&'a KwVal),
}

/// Resolved `CASE_INDEXES` lookup, lazy `GENDER_PLURAL_INDEXES` argument and
/// the plural/animacy flags — i.e. the `**kwargs` bundle threaded through
/// every `get_num_element`.
///
/// Note the collision hazard in the Python: `case="n"` means *nominative*
/// (`CASE_INDEXES["n"] == 0`) while `gender="n"` means *neuter*
/// (`GENDER_PLURAL_INDEXES["n"] == 2`). They are separate dicts; keeping case
/// as a pre-resolved index and gender as its own type keeps the two from
/// being confused.
#[derive(Clone, Copy)]
struct Opts<'a> {
    /// CASE_INDEXES: n=0, g=1, d=2, a=3, i=4, p=5
    case: usize,
    plural: bool,
    gender: GenderArg<'a>,
    animate: bool,
}

/// Python's module-level defaults: D_CASE="n", D_PLURAL=False, D_GENDER="m",
/// D_ANIMATE=True.
impl Default for Opts<'_> {
    fn default() -> Self {
        Opts { case: 0, plural: false, gender: GenderArg::Idx(0), animate: true }
    }
}

const CASE_N: usize = 0;
const CASE_G: usize = 1;
const GENDER_M: usize = 0;
const GENDER_F: usize = 1;
const GENDER_NEUT: usize = 2;

/// Python's `case_classifier_element`.
fn case_classifier_element(cl: &Classifier, o: Opts<'_>) -> Result<String> {
    Ok(match &cl[o.case] {
        C::S(text) => text.clone(),
        C::G(genders) => {
            // Python: `gender = case[GENDER_PLURAL_INDEXES["plural"]]` when
            // plural, which is index 3 — the gender kwarg is never looked up.
            let idx = if o.plural {
                3
            } else {
                match o.gender {
                    GenderArg::Idx(i) => i,
                    // GENDER_PLURAL_INDEXES[gender] misses only here.
                    GenderArg::Bad(kv) => return Err(dict_lookup_error(kv)),
                }
            };
            match &genders[idx] {
                G::S(text) => text.clone(),
                G::A(animate, inanimate) => {
                    if o.animate {
                        animate.clone()
                    } else {
                        inanimate.clone()
                    }
                }
            }
        }
    })
}

/// The exception Python raises for `SOME_DICT[key]` with a bad key: KeyError
/// carrying the key itself, or TypeError for an unhashable key. `None`, bools
/// and ints are hashable but never present, so they are KeyErrors too (the
/// binding can only carry the message string, so `KeyError: None` renders as
/// `KeyError: 'None'` — same type, which is all the corpus records).
fn dict_lookup_error(kv: &KwVal) -> N2WError {
    match kv {
        KwVal::Str(s) => N2WError::Key(s.clone()),
        KwVal::None => N2WError::Key("None".into()),
        KwVal::Bool(b) => N2WError::Key(if *b { "True" } else { "False" }.into()),
        KwVal::Int(i) => N2WError::Key(i.to_string()),
        KwVal::List(_) => N2WError::Type("unhashable type: 'list'".into()),
    }
}

/// A `{num: classifier}` dict, keyed by small dense integers.
struct Table(Vec<Option<Classifier>>);

impl Table {
    fn build(entries: Vec<(usize, Classifier)>) -> Table {
        let max = entries.iter().map(|(k, _)| *k).max().unwrap_or(0);
        let mut v: Vec<Option<Classifier>> = (0..=max).map(|_| None).collect();
        for (k, c) in entries {
            v[k] = Some(c);
        }
        Table(v)
    }
}

/// Python's `get_num_element(cases_dict, num, **kwargs)`.
///
/// A missing key is a Python `KeyError`. Every call site except the THOUSANDS
/// ones is statically in range; see `thousands_elements` for the KeyError note.
fn get_num_element(t: &Table, num: usize, o: Opts<'_>) -> Result<String> {
    match t.0.get(num).and_then(|x| x.as_ref()) {
        Some(cl) => case_classifier_element(cl, o),
        None => Err(N2WError::Key(num.to_string())),
    }
}

/// Python's `CASE_INDEXES[case]` for a string key.
fn case_index(s: &str) -> Option<usize> {
    Some(match s {
        "n" | "nominative" | "и" | "именительный" => 0,
        "g" | "genitive" | "р" | "родительный" => 1,
        "d" | "dative" | "д" | "дательный" => 2,
        "a" | "accusative" | "в" | "винительный" => 3,
        "i" | "instrumental" | "т" | "творительный" => 4,
        "p" | "prepositional" | "п" | "предложный" => 5,
        _ => return None,
    })
}

/// Python's `GENDER_PLURAL_INDEXES[gender]` for a string key. Note `"p"` is a
/// *valid* gender selecting the plural forms even with `plural=False`.
fn gender_index(s: &str) -> Option<usize> {
    Some(match s {
        "m" | "masculine" | "м" | "мужской" => 0,
        "f" | "feminine" | "ж" | "женский" => 1,
        "n" | "neuter" | "с" | "средний" => 2,
        "p" | "plural" => 3,
        _ => return None,
    })
}

/// Python truthiness for the `plural=` / `animate=` kwargs — the source only
/// ever tests `if plural:` / `if animate:`, so any type is accepted.
fn kw_truthy(kv: Option<&KwVal>, default: bool) -> bool {
    match kv {
        None => default,
        Some(KwVal::Bool(b)) => *b,
        Some(KwVal::Int(i)) => *i != 0,
        Some(KwVal::Str(s)) => !s.is_empty(),
        Some(KwVal::List(l)) => !l.is_empty(),
        Some(KwVal::None) => false,
    }
}

// ---------------------------------------------------------------------------
// Table construction helpers
// ---------------------------------------------------------------------------

/// Six cases, each a plain string.
fn all6(w: [&str; 6]) -> Classifier {
    [
        C::S(w[0].into()),
        C::S(w[1].into()),
        C::S(w[2].into()),
        C::S(w[3].into()),
        C::S(w[4].into()),
        C::S(w[5].into()),
    ]
}

/// One case as `[m, f, n, plural]`, all plain strings.
fn g4(m: &str, f: &str, n: &str, p: &str) -> C {
    C::G([G::S(m.into()), G::S(f.into()), G::S(n.into()), G::S(p.into())])
}

/// Python's `get_cases(prefix, post_group)` — CASE_POSTFIXES expanded inline.
///
/// CASE_POSTFIXES mixes three postfix shapes: a plain `str`, a `dict`
/// `{0: .., 1: ..}` selected by `post_group`, and a `list` of animate/inanimate
/// variants whose members are themselves either `str` or such a dict. Rather
/// than model those unions, the fixed 6x4 table is written out directly.
///
/// Every case in CASE_POSTFIXES is a 4-list, so this always yields `C::G`.
fn get_cases(prefix: &str, post_group: usize) -> Classifier {
    let p = |suf: &str| format!("{}{}", prefix, suf);
    // Python: `prefix + postfix[post_group]` for a `{0: .., 1: ..}` postfix.
    let d = |a: &str, b: &str| if post_group == 0 { p(a) } else { p(b) };
    [
        // [{0: "ой", 1: "ый"}, "ая", "ое", "ые"]
        C::G([G::S(d("ой", "ый")), G::S(p("ая")), G::S(p("ое")), G::S(p("ые"))]),
        // ["ого", "ой", "ого", "ых"]
        C::G([G::S(p("ого")), G::S(p("ой")), G::S(p("ого")), G::S(p("ых"))]),
        // ["ому", "ой", "ому", "ым"]
        C::G([G::S(p("ому")), G::S(p("ой")), G::S(p("ому")), G::S(p("ым"))]),
        // [["ого", {0: "ой", 1: "ый"}], "ую", "ое", ["ых", "ые"]]
        C::G([
            G::A(p("ого"), d("ой", "ый")),
            G::S(p("ую")),
            G::S(p("ое")),
            G::A(p("ых"), p("ые")),
        ]),
        // ["ым", "ой", "ым", "ыми"]
        C::G([G::S(p("ым")), G::S(p("ой")), G::S(p("ым")), G::S(p("ыми"))]),
        // ["ом", "ой", "ом", "ых"]
        C::G([G::S(p("ом")), G::S(p("ой")), G::S(p("ом")), G::S(p("ых"))]),
    ]
}

/// Python's `get_ord_classifier(prefixes, post_groups)`.
fn ord_classifier(prefixes: &[(usize, &str, usize)]) -> Vec<(usize, Classifier)> {
    prefixes.iter().map(|(n, p, pg)| (*n, get_cases(p, *pg))).collect()
}

// ---------------------------------------------------------------------------
// Raw word data
// ---------------------------------------------------------------------------

/// HUNDREDS, as raw strings — needed twice: as its own table, and as the
/// source of HUNDREDS_ORD_PREFIXES (which reuses the genitive form).
const HUNDREDS_RAW: [(usize, [&str; 6]); 9] = [
    (1, ["сто", "ста", "ста", "сто", "ста", "ста"]),
    (2, ["двести", "двухсот", "двумстам", "двести", "двумястами", "двухстах"]),
    (3, ["триста", "трёхсот", "трёмстам", "триста", "тремястами", "трёхстах"]),
    (
        4,
        ["четыреста", "четырёхсот", "четырёмстам", "четыреста", "четырьмястами", "четырёхстах"],
    ),
    (5, ["пятьсот", "пятисот", "пятистам", "пятьсот", "пятьюстами", "пятистах"]),
    (6, ["шестьсот", "шестисот", "шестистам", "шестьсот", "шестьюстами", "шестистах"]),
    (7, ["семьсот", "семисот", "семистам", "семьсот", "семьюстами", "семистах"]),
    (
        8,
        ["восемьсот", "восьмисот", "восьмистам", "восемьсот", "восемьюстами", "восьмистах"],
    ),
    (
        9,
        ["девятьсот", "девятисот", "девятистам", "девятьсот", "девятьюстами", "девятистах"],
    ),
];

/// Python: TENS_PREFIXES — the 11..19 stems.
const TENS_PREFIXES: [(usize, &str); 9] = [
    (1, "один"),
    (2, "две"),
    (3, "три"),
    (4, "четыр"),
    (5, "пят"),
    (6, "шест"),
    (7, "сем"),
    (8, "восем"),
    (9, "девят"),
];

const TENS_POSTFIXES: [&str; 6] =
    ["надцать", "надцати", "надцати", "надцать", "надцатью", "надцати"];

/// Python: THOUSANDS_PREFIXES — the 10^6 .. 10^30 stems (chunk index 2..10).
const THOUSANDS_PREFIXES: [(usize, &str); 9] = [
    (2, "миллион"),
    (3, "миллиард"),
    (4, "триллион"),
    (5, "квадриллион"),
    (6, "квинтиллион"),
    (7, "секстиллион"),
    (8, "септиллион"),
    (9, "октиллион"),
    (10, "нониллион"),
];

/// Python: THOUSANDS_POSTFIXES — six cases x the three `pluralize` forms.
const THOUSANDS_POSTFIXES: [[&str; 3]; 6] = [
    ["", "а", "ов"],
    ["а", "ов", "ов"],
    ["у", "ам", "ам"],
    ["", "а", "ов"],
    ["ом", "ами", "ами"],
    ["е", "ах", "ах"],
];

// ---------------------------------------------------------------------------
// Currency data
// ---------------------------------------------------------------------------

/// `Num2Word_RU.CURRENCY_FORMS`, verbatim.
///
/// RU subclasses `Num2Word_Base` directly and declares its **own** class-level
/// `CURRENCY_FORMS`, so it never sees the shared `Num2Word_EUR` dict that
/// `Num2Word_EN.__init__` mutates in place. None of EN's ~24 extra codes leak
/// in: JPY, KWD, BHD, INR, CNY and CHF all raise NotImplementedError here, as
/// the corpus records.
///
/// Every entry carries **three** forms on both sides, because RU's `pluralize`
/// indexes 0/1/2 (one / few / many). Dropping the third would silently change
/// output for every n outside `n % 10 == 1`.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const KOPEKS: [&str; 3] = ["копейка", "копейки", "копеек"];
    const CENTS: [&str; 3] = ["цент", "цента", "центов"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("RUB", CurrencyForms::new(&["рубль", "рубля", "рублей"], &KOPEKS));
    m.insert("EUR", CurrencyForms::new(&["евро", "евро", "евро"], &CENTS));
    m.insert("USD", CurrencyForms::new(&["доллар", "доллара", "долларов"], &CENTS));
    m.insert("UAH", CurrencyForms::new(&["гривна", "гривны", "гривен"], &KOPEKS));
    m.insert(
        "KZT",
        CurrencyForms::new(&["тенге", "тенге", "тенге"], &["тиын", "тиына", "тиынов"]),
    );
    m.insert(
        "BYN",
        CurrencyForms::new(
            &["белорусский рубль", "белорусских рубля", "белорусских рублей"],
            &KOPEKS,
        ),
    );
    m.insert(
        "UZS",
        CurrencyForms::new(&["сум", "сума", "сумов"], &["тийин", "тийина", "тийинов"]),
    );
    m.insert(
        "PLN",
        CurrencyForms::new(
            // "слотых" is a typo for "злотых" in lang_RU.py. Kept verbatim:
            // the Python is the spec, bugs included.
            &["польский злотый", "польских слотых", "польских злотых"],
            &["грош", "гроша", "грошей"],
        ),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(
            &["фунт стерлингов", "фунта стерлингов", "фунтов стерлингов"],
            &["пенни", "пенни", "пенни"],
        ),
    );
    m
}

// ---------------------------------------------------------------------------
// The language
// ---------------------------------------------------------------------------

pub struct LangRu {
    ones: Table,
    ones_ord: Table,
    tens: Table,
    tens_ord: Table,
    twenties: Table,
    twenties_ord: Table,
    hundreds: Table,
    hundreds_ord: Table,
    /// Python's THOUSANDS: `{num: [case][plural_form]}` — a `pluralize` triple
    /// per case, *not* a classifier.
    thousands: Vec<Option<[[String; 3]; 6]>>,
    thousands_ord: Table,
    /// Built once in `new()`. `to_currency`/`to_cheque` only ever read it, and
    /// rebuilding the table per call is what made an earlier revision of this
    /// port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangRu {
    fn default() -> Self {
        Self::new()
    }
}

impl LangRu {
    pub fn new() -> Self {
        // -- ONES ---------------------------------------------------------
        let ones = Table::build(vec![
            (0, all6(["ноль", "ноля", "нолю", "ноль", "нолём", "ноле"])),
            (
                1,
                [
                    g4("один", "одна", "одно", "одни"),
                    g4("одного", "одной", "одного", "одних"),
                    g4("одному", "одной", "одному", "одним"),
                    C::G([
                        G::A("одного".into(), "один".into()),
                        G::S("одну".into()),
                        G::S("одно".into()),
                        G::A("одних".into(), "одни".into()),
                    ]),
                    g4("одним", "одной", "одним", "одними"),
                    g4("одном", "одной", "одном", "одних"),
                ],
            ),
            (
                2,
                [
                    g4("два", "две", "два", "двое"),
                    g4("двух", "двух", "двух", "двоих"),
                    g4("двум", "двум", "двум", "двоим"),
                    C::G([
                        G::A("двух".into(), "два".into()),
                        G::A("двух".into(), "две".into()),
                        G::S("два".into()),
                        G::S("двоих".into()),
                    ]),
                    g4("двумя", "двумя", "двумя", "двоими"),
                    g4("двух", "двух", "двух", "двоих"),
                ],
            ),
            (
                3,
                [
                    g4("три", "три", "три", "трое"),
                    g4("трёх", "трёх", "трёх", "троих"),
                    g4("трём", "трём", "трём", "троим"),
                    C::G([
                        G::A("трёх".into(), "три".into()),
                        G::A("трёх".into(), "три".into()),
                        G::S("три".into()),
                        G::S("троих".into()),
                    ]),
                    g4("тремя", "тремя", "тремя", "троими"),
                    g4("трёх", "трёх", "трёх", "троих"),
                ],
            ),
            (
                4,
                [
                    g4("четыре", "четыре", "четыре", "четверо"),
                    g4("четырёх", "четырёх", "четырёх", "четверых"),
                    g4("четырём", "четырём", "четырём", "четверым"),
                    C::G([
                        G::A("четырёх".into(), "четыре".into()),
                        G::A("четырёх".into(), "четыре".into()),
                        G::S("четыре".into()),
                        G::S("четверых".into()),
                    ]),
                    g4("четырьмя", "четырьмя", "четырьмя", "четверыми"),
                    g4("четырёх", "четырёх", "четырёх", "четверых"),
                ],
            ),
            (5, all6(["пять", "пяти", "пяти", "пять", "пятью", "пяти"])),
            (6, all6(["шесть", "шести", "шести", "шесть", "шестью", "шести"])),
            (7, all6(["семь", "семи", "семи", "семь", "семью", "семи"])),
            (8, all6(["восемь", "восьми", "восьми", "восемь", "восемью", "восьми"])),
            (9, all6(["девять", "девяти", "девяти", "девять", "девятью", "девяти"])),
        ]);

        // -- ONES_ORD -----------------------------------------------------
        // Python seeds the dict with the irregular 3, then `.update()`s the
        // generated classifier. ONES_ORD_PREFIXES has no key 3, so the
        // irregular entry survives.
        let mut ones_ord_entries = vec![(
            3,
            [
                g4("третий", "третья", "третье", "третьи"),
                g4("третьего", "третьей", "третьего", "третьих"),
                g4("третьему", "третьей", "третьему", "третьим"),
                C::G([
                    G::A("третьего".into(), "третий".into()),
                    G::S("третью".into()),
                    G::S("третье".into()),
                    G::A("третьих".into(), "третьи".into()),
                ]),
                g4("третьим", "третьей", "третьим", "третьими"),
                g4("третьем", "третьей", "третьем", "третьих"),
            ],
        )];
        // ONES_ORD_PREFIXES zipped with ONES_ORD_POSTFIXES_GROUPS.
        ones_ord_entries.extend(ord_classifier(&[
            (0, "нулев", 0),
            (1, "перв", 1),
            (2, "втор", 0),
            (4, "четвёрт", 1),
            (5, "пят", 1),
            (6, "шест", 0),
            (7, "седьм", 0),
            (8, "восьм", 0),
            (9, "девят", 1),
        ]));
        let ones_ord = Table::build(ones_ord_entries);

        // -- TENS (10..19, keyed by the ones digit) ------------------------
        let mut tens_entries =
            vec![(0, all6(["десять", "десяти", "десяти", "десять", "десятью", "десяти"]))];
        for (num, prefix) in TENS_PREFIXES {
            let w: Vec<String> =
                TENS_POSTFIXES.iter().map(|s| format!("{}{}", prefix, s)).collect();
            tens_entries.push((
                num,
                all6([&w[0], &w[1], &w[2], &w[3], &w[4], &w[5]]),
            ));
        }
        let tens = Table::build(tens_entries);

        // -- TENS_ORD -----------------------------------------------------
        let mut tens_ord_prefixes: Vec<(usize, String, usize)> = vec![(0, "десят".into(), 1)];
        tens_ord_prefixes
            .extend(TENS_PREFIXES.iter().map(|(n, p)| (*n, format!("{}надцат", p), 1)));
        let tens_ord = Table::build(
            tens_ord_prefixes.iter().map(|(n, p, pg)| (*n, get_cases(p, *pg))).collect(),
        );

        // -- TWENTIES -----------------------------------------------------
        let twenties = Table::build(vec![
            (2, all6(["двадцать", "двадцати", "двадцати", "двадцать", "двадцатью", "двадцати"])),
            (3, all6(["тридцать", "тридцати", "тридцати", "тридцать", "тридцатью", "тридцати"])),
            (4, all6(["сорок", "сорока", "сорока", "сорок", "сорока", "сорока"])),
            (
                5,
                all6([
                    "пятьдесят",
                    "пятидесяти",
                    "пятидесяти",
                    "пятьдесят",
                    "пятьюдесятью",
                    "пятидесяти",
                ]),
            ),
            (
                6,
                all6([
                    "шестьдесят",
                    "шестидесяти",
                    "шестидесяти",
                    "шестьдесят",
                    "шестьюдесятью",
                    "шестидесяти",
                ]),
            ),
            (
                7,
                all6([
                    "семьдесят",
                    "семидесяти",
                    "семидесяти",
                    "семьдесят",
                    "семьюдесятью",
                    "семидесяти",
                ]),
            ),
            (
                8,
                all6([
                    "восемьдесят",
                    "восьмидесяти",
                    "восьмидесяти",
                    "восемьдесят",
                    "восемьюдесятью",
                    "восьмидесяти",
                ]),
            ),
            (
                9,
                all6(["девяносто", "девяноста", "девяноста", "девяносто", "девяноста", "девяноста"]),
            ),
        ]);

        let twenties_ord = Table::build(ord_classifier(&[
            (2, "двадцат", 1),
            (3, "тридцат", 1),
            (4, "сороков", 0),
            (5, "пятидесят", 1),
            (6, "шестидесят", 1),
            (7, "семидесят", 1),
            (8, "восьмидесят", 1),
            (9, "девяност", 1),
        ]));

        // -- HUNDREDS -----------------------------------------------------
        let hundreds =
            Table::build(HUNDREDS_RAW.iter().map(|(n, w)| (*n, all6(*w))).collect());

        // Python: `{num: case[1] if num != 1 else "сот"}` — the genitive form
        // ("двухсот", "трёхсот", ...) becomes the ordinal stem; 1 is irregular.
        let hundreds_ord = Table::build(
            HUNDREDS_RAW
                .iter()
                .map(|(n, w)| {
                    let prefix = if *n != 1 { w[1] } else { "сот" };
                    (*n, get_cases(prefix, 1))
                })
                .collect(),
        );

        // -- THOUSANDS ----------------------------------------------------
        let mut thousands: Vec<Option<[[String; 3]; 6]>> = (0..=10).map(|_| None).collect();
        thousands[1] = Some([
            ["тысяча".into(), "тысячи".into(), "тысяч".into()],
            ["тысячи".into(), "тысяч".into(), "тысяч".into()],
            ["тысяче".into(), "тысячам".into(), "тысячам".into()],
            ["тысячу".into(), "тысячи".into(), "тысяч".into()],
            ["тысячей".into(), "тысячами".into(), "тысячами".into()],
            ["тысяче".into(), "тысячах".into(), "тысячах".into()],
        ]);
        for (num, prefix) in THOUSANDS_PREFIXES {
            let mk = |case: usize| -> [String; 3] {
                let c = THOUSANDS_POSTFIXES[case];
                [
                    format!("{}{}", prefix, c[0]),
                    format!("{}{}", prefix, c[1]),
                    format!("{}{}", prefix, c[2]),
                ]
            };
            thousands[num] = Some([mk(0), mk(1), mk(2), mk(3), mk(4), mk(5)]);
        }

        let mut thousands_ord_prefixes: Vec<(usize, String, usize)> =
            vec![(1, "тысячн".into(), 1)];
        thousands_ord_prefixes
            .extend(THOUSANDS_PREFIXES.iter().map(|(n, p)| (*n, format!("{}н", p), 1)));
        let thousands_ord = Table::build(
            thousands_ord_prefixes.iter().map(|(n, p, pg)| (*n, get_cases(p, *pg))).collect(),
        );

        LangRu {
            ones,
            ones_ord,
            tens,
            tens_ord,
            twenties,
            twenties_ord,
            hundreds,
            hundreds_ord,
            thousands,
            thousands_ord,
            currency_forms: build_currency_forms(),
        }
    }

    /// Python's `verify_ordinal` from `base.py`. The float check is
    /// unreachable here — input is always integral.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// Resolve the `case=`/`plural=`/`gender=`/`animate=` kwargs bag into an
    /// `Opts`, with Python's evaluation discipline:
    ///
    /// * `case` — `CASE_INDEXES[case]` is technically lazy in Python too, but
    ///   every integer rendering reaches at least one `case_classifier_element`
    ///   or `get_thousands_elements` call (n<0 recurses on abs(n); n==0 hits
    ///   ONES[0]; every nonzero chunk emits at least one element), so resolving
    ///   it eagerly raises the same KeyError at the same entry points. Callers
    ///   must still run `verify_ordinal` *first* — Python's `to_ordinal(-5,
    ///   case="xx")` is TypeError, not KeyError.
    /// * `gender` — genuinely lazy; see `GenderArg`.
    /// * `plural`/`animate` — truthiness, not bools.
    /// * an explicit `None` for `case`/`gender` is a KeyError like any other
    ///   missing dict key (there is no `if case is None` default in the
    ///   Python), while for `plural`/`animate` it is simply falsy.
    fn opts_from_kwargs<'a>(&self, kw: &'a Kwargs) -> Result<Opts<'a>> {
        let case = match kw.get("case") {
            None => 0,
            Some(KwVal::Str(s)) => match case_index(s) {
                Some(i) => i,
                None => return Err(dict_lookup_error(kw.get("case").unwrap())),
            },
            Some(other) => return Err(dict_lookup_error(other)),
        };
        let gender = match kw.get("gender") {
            None => GenderArg::Idx(0),
            Some(KwVal::Str(s)) => match gender_index(s) {
                Some(i) => GenderArg::Idx(i),
                None => GenderArg::Bad(kw.get("gender").unwrap()),
            },
            Some(other) => GenderArg::Bad(other),
        };
        Ok(Opts {
            case,
            plural: kw_truthy(kw.get("plural"), false),
            gender,
            animate: kw_truthy(kw.get("animate"), true),
        })
    }

    /// Python's `get_thousands_elements(num, case)`.
    ///
    /// THOUSANDS only has keys 1..=10, so a chunk index above 10 — i.e. a value
    /// >= 10^33 — was a bare `KeyError` in Python. `int2word` now rejects
    /// those values up front with `OverflowError` (gladiaio/num2words2#159),
    /// so the `Key` arm here is unreachable from the public entry points.
    fn thousands_elements(&self, num: usize, case: usize) -> Result<&[String; 3]> {
        match self.thousands.get(num).and_then(|x| x.as_ref()) {
            Some(cases) => Ok(&cases[case]),
            None => Err(N2WError::Key(num.to_string())),
        }
    }

    /// Python's `pluralize`. `n` is a chunk in 0..=999, so it is never
    /// negative and Rust's `%` agrees with Python's.
    fn pluralize(n: u32, forms: &[String; 3]) -> String {
        if matches!(n % 100, 11 | 12 | 13 | 14) {
            return forms[2].clone();
        }
        if n % 10 == 1 {
            return forms[0].clone();
        }
        if matches!(n % 10, 2 | 3 | 4) {
            return forms[1].clone();
        }
        forms[2].clone()
    }

    /// Python's `Num2Word_RU._int2word`.
    fn int2word(&self, n: &BigInt, cardinal: bool, o: Opts<'_>) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_negative() {
            // Python: " ".join([self.negword, ...]) — negword is "минус" with
            // no trailing space, and there is no `.strip()` here.
            let inner = self.int2word(&n.abs(), cardinal, o)?;
            return Ok(format!("{} {}", self.negword(), inner));
        }

        if n.is_zero() {
            let t = if cardinal { &self.ones } else { &self.ones_ord };
            return get_num_element(t, 0, o);
        }

        let digits = n.to_string();
        let chunks = splitbyx(&digits, 3);
        // Join into one word if the number ends on a 'тысячный'-style zero chunk.
        let ord_join = *chunks.last().unwrap() == 0;
        let len = chunks.len();
        // Python: `i - 1 - max([i for i, e in enumerate(chunks) if e != 0])`.
        // The comprehension's `i` does NOT leak in Python 3, so the outer `i`
        // (== len(chunks)) is intact. n != 0 here, so a nonzero chunk exists.
        let rightmost_nonzero = chunks.iter().rposition(|&e| e != 0).unwrap();
        let rightest_nonzero_chunk_i = len - 1 - rightmost_nonzero;

        let mut words: Vec<String> = Vec::new();
        for (j, &x) in chunks.iter().enumerate() {
            // Python decrements `i` before the zero-skip, so it tracks the
            // chunk position regardless of whether the chunk is skipped.
            let i = len - 1 - j;
            if x == 0 {
                continue;
            }

            let (n1, n2, n3) = get_digits(x);
            let mut chunk_words: Vec<String>;

            if cardinal {
                chunk_words = self.chunk_cardinal(n3, n2, n1, i, o)?;
                if i > 0 {
                    let forms = self.thousands_elements(i, o.case)?;
                    chunk_words.push(Self::pluralize(x, forms));
                }
            } else if !(ord_join && rightest_nonzero_chunk_i == i) {
                // ordinal, not joined
                chunk_words = self.chunk_ordinal(n3, n2, n1, i, o)?;
                if i > 0 {
                    let t_case = if rightest_nonzero_chunk_i == i { o.case } else { CASE_N };
                    let forms = self.thousands_elements(i, t_case)?;
                    chunk_words.push(Self::pluralize(x, forms));
                }
            } else {
                // ordinal, joined ("двухтысячный")
                chunk_words = self.chunk_ordinal_join(n3, n2, n1, i, o)?;
                if i > 0 {
                    chunk_words.push(get_num_element(&self.thousands_ord, i, o)?);
                }
                chunk_words = vec![chunk_words.concat()];
            }

            words.extend(chunk_words);
        }

        Ok(words.join(" "))
    }

    /// Python's `__chunk_cardianl` (sic — the typo is in the original).
    fn chunk_cardinal(
        &self,
        hundreds: u32,
        tens: u32,
        ones: u32,
        chunk_num: usize,
        o: Opts<'_>,
    ) -> Result<Vec<String>> {
        let mut words = Vec::new();
        if hundreds > 0 {
            words.push(get_num_element(&self.hundreds, hundreds as usize, o)?);
        }
        if tens > 1 {
            words.push(get_num_element(&self.twenties, tens as usize, o)?);
        }
        if tens == 1 {
            words.push(get_num_element(&self.tens, ones as usize, o)?);
        } else if ones > 0 {
            let w_ones = if chunk_num == 1 {
                // Thousands are feminine.
                let mut f = o;
                f.gender = GenderArg::Idx(GENDER_F);
                get_num_element(&self.ones, ones as usize, f)?
            } else {
                // chunk_num == 0 and chunk_num > 1 take the same branch.
                get_num_element(&self.ones, ones as usize, o)?
            };
            words.push(w_ones);
        }
        Ok(words)
    }

    /// Python's `__chunk_ordinal`.
    ///
    /// Note the several `get_num_element(TABLE, n)` calls with **no kwargs**:
    /// those reset to the module defaults rather than inheriting the caller's
    /// case/gender/animacy. Reproduced exactly via `Opts::default()`.
    fn chunk_ordinal(
        &self,
        hundreds: u32,
        tens: u32,
        ones: u32,
        chunk_num: usize,
        o: Opts<'_>,
    ) -> Result<Vec<String>> {
        let d = Opts::default();
        let mut words = Vec::new();

        if hundreds > 0 {
            if tens == 0 && ones == 0 {
                words.push(get_num_element(&self.hundreds_ord, hundreds as usize, o)?);
            } else {
                words.push(get_num_element(&self.hundreds, hundreds as usize, d)?);
            }
        }

        if tens > 1 {
            if ones == 0 {
                words.push(get_num_element(&self.twenties_ord, tens as usize, o)?);
            } else {
                words.push(get_num_element(&self.twenties, tens as usize, d)?);
            }
        }

        if tens == 1 {
            // NOTE (faithful port of a bug): TENS_ORD is used for *every*
            // chunk, not just the last one, so 10001 renders as
            // "десятый тысяч первый" rather than "десять тысяч первый".
            words.push(get_num_element(&self.tens_ord, ones as usize, o)?);
        } else if ones > 0 {
            let w_ones: Option<String> = if chunk_num == 0 {
                Some(get_num_element(&self.ones_ord, ones as usize, o)?)
            } else if chunk_num > 0 && ones == 1 && hundreds == 0 && tens == 0 {
                // тысячный, миллионный etc. — the "one" is left implicit.
                None
            } else if chunk_num == 1 {
                // Thousands are feminine.
                let f = Opts { gender: GenderArg::Idx(GENDER_F), ..Opts::default() };
                Some(get_num_element(&self.ones, ones as usize, f)?)
            } else {
                Some(get_num_element(&self.ones, ones as usize, d)?)
            };
            // Python: `if w_ones:` — None and "" are both falsy.
            if let Some(w) = w_ones {
                if !w.is_empty() {
                    words.push(w);
                }
            }
        }

        Ok(words)
    }

    /// Python's `__chunk_ordinal_join` — the "двухтысячный" single-word path.
    fn chunk_ordinal_join(
        &self,
        hundreds: u32,
        tens: u32,
        ones: u32,
        chunk_num: usize,
        o: Opts<'_>,
    ) -> Result<Vec<String>> {
        let d = Opts::default();
        let g = Opts { case: CASE_G, ..Opts::default() };
        let mut words = Vec::new();

        if hundreds > 1 {
            words.push(get_num_element(&self.hundreds, hundreds as usize, g)?);
        } else if hundreds == 1 {
            // стО, not стА
            words.push(get_num_element(&self.hundreds, hundreds as usize, d)?);
        }

        if tens == 9 {
            // девяностО, not девяностА
            words.push(get_num_element(&self.twenties, tens as usize, d)?);
        } else if tens > 1 {
            words.push(get_num_element(&self.twenties, tens as usize, g)?);
        }

        if tens == 1 {
            words.push(get_num_element(&self.tens, ones as usize, g)?);
        } else if ones > 0 {
            let w_ones: Option<String> = if chunk_num == 0 {
                Some(get_num_element(&self.ones_ord, ones as usize, o)?)
            } else if chunk_num > 0 && ones == 1 && tens != 1 {
                // `tens != 1` is always true in this branch (the `tens == 1`
                // arm above already consumed that case), but it is in the
                // original condition, so it is kept.
                if tens == 0 && hundreds == 0 {
                    None
                } else {
                    // двадцатиодномиллионный
                    let neut = Opts { gender: GenderArg::Idx(GENDER_NEUT), ..Opts::default() };
                    Some(get_num_element(&self.ones, 1, neut)?)
                }
            } else {
                Some(get_num_element(&self.ones, ones as usize, g)?)
            };
            if let Some(w) = w_ones {
                if !w.is_empty() {
                    words.push(w);
                }
            }
        }

        Ok(words)
    }

    /// Body of `Num2Word_RU.to_cardinal` for float/Decimal input; `kw` is
    /// consulted only where Python passes `case`/`plural`/`gender`/`animate`.
    fn cardinal_float_with(&self, value: &FloatValue, kw: &Kwargs) -> Result<String> {
        let n = match value {
            FloatValue::Float { value: f, .. } => {
                if *f == 0.0 {
                    // _int2word(0, cardinal=True, case=case, ...) — "ноль".
                    return self.int2word(&BigInt::zero(), true, self.opts_from_kwargs(kw)?);
                }
                if f.abs() < 0.01 {
                    // Python's `abs(number) < 0.01` guard returned "ноль" for
                    // every such float, so 0.005 read as zero while
                    // Decimal("0.005") read "ноль целых пять тысячных"
                    // (#208). Read the same digits positionally (repr goes
                    // exponential below 1e-4): Rust's f64 Display is the
                    // shortest round-trip, always positional.
                    format!("{}", f)
                } else {
                    py_float_repr(*f)
                }
            }
            // No `< 0.01` guard on the Decimal arm; str(Decimal) verbatim.
            FloatValue::Decimal { value: d, .. } => py_decimal_str(d),
        };
        self.cardinal_from_str(&n, kw)
    }

    /// The `"." in n` arm of `Num2Word_RU.to_cardinal`, driven off the decimal
    /// string `n` (`str(number).replace(",", ".")`).
    ///
    /// RU never touches `base.float2tuple`: it reads the *shortest repr digits*
    /// directly, so the f64-artefact / banker's-rounding traps of the base float
    /// path do not apply here. `str(2.675) == "2.675"` yields "шестьсот семьдесят
    /// пять тысячных", not the `674.999…` the tuple path would rescue.
    ///
    /// ```python
    /// n = n.replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     decimal_part = self._int2word(int(right), cardinal=True, gender="f")
    ///     result = "%s %s %s %s" % (
    ///         self._int2word(int(left), cardinal=True, gender="f"),
    ///         self.pluralize(int(left), self.pointword),
    ///         decimal_part,
    ///         self.__decimal_bitness(right),
    ///     )
    ///     if is_negative:
    ///         result = self.negword + " " + result
    ///     return result
    /// else:
    ///     if "e" in n.lower() or "E" in n:
    ///         n = str(int(float(n)))
    ///     return self._int2word(int(n), cardinal=True, case=case, ...)  # caller's kwargs
    /// ```
    fn cardinal_from_str(&self, number: &str, kw: &Kwargs) -> Result<String> {
        // The digit strings that reach here are pure ASCII (`str(float)` /
        // `str(Decimal)`), so byte slicing is safe throughout.
        let n = number.replace(',', ".");
        if let Some(_dot) = n.find('.') {
            let is_negative = n.starts_with('-');
            let abs_n: &str = if is_negative { &n[1..] } else { &n };
            // `abs_n.split(".")` — exactly one dot in every repr the shim feeds.
            let dotpos = abs_n.find('.').unwrap();
            let left = &abs_n[..dotpos];
            let right = &abs_n[dotpos + 1..];

            let int_left = parse_int(left)?;
            let int_right = parse_int(right)?;

            // Both `_int2word` calls in the "%s %s %s %s" tuple pass gender="f"
            // and default case/plural/animate.
            let fem = Opts { gender: GenderArg::Idx(GENDER_F), ..Opts::default() };
            let left_word = self.int2word(&int_left, true, fem)?;

            // pluralize(int(left), self.pointword) — the ("целая","целых","целых")
            // 3-tuple. RU's pointword is a plural triple, not a single word, so
            // the trait's `pointword() -> &str` is deliberately not used here.
            let pointword: [String; 3] =
                ["целая".into(), "целых".into(), "целых".into()];
            let celaya = self.pluralize(&int_left, &pointword)?;

            let decimal_part = self.int2word(&int_right, true, fem)?;
            let bitness = self.decimal_bitness(right)?;

            let result =
                format!("{} {} {} {}", left_word, celaya, decimal_part, bitness);
            if is_negative {
                Ok(format!("{} {}", self.negword(), result))
            } else {
                Ok(result)
            }
        } else {
            // Integer string. A float only lands here via exponent notation
            // ("1e+16"); a Decimal via a bare integer ("5") or its own exponent
            // form ("1E+2").
            if n.to_lowercase().contains('e') {
                // n = str(int(float(n)))
                let f: f64 = n.parse().map_err(|_| {
                    N2WError::Value(format!(
                        "could not convert string to float: '{}'",
                        n
                    ))
                })?;
                let bi = BigInt::from_f64(f.trunc()).ok_or_else(|| {
                    N2WError::Value(format!(
                        "cannot convert float {} to integer",
                        f
                    ))
                })?;
                self.int2word(&bi, true, self.opts_from_kwargs(kw)?)
            } else {
                // int(n) — parses the sign; int2word re-emits the negword.
                self.int2word(&parse_int(&n)?, true, self.opts_from_kwargs(kw)?)
            }
        }
    }

    /// Python's `Num2Word_RU.__decimal_bitness(n)` where `n` is the fractional
    /// digit string `right` (leading/trailing zeros intact).
    ///
    /// ```python
    /// if n[-1] == "1" and n[-2:] != "11":
    ///     return self._int2word(10 ** len(n), cardinal=False, gender="f")
    /// return self._int2word(10 ** len(n), cardinal=False, case="g", plural=True)
    /// ```
    fn decimal_bitness(&self, right: &str) -> Result<String> {
        let chars: Vec<char> = right.chars().collect();
        // n[-1]; an empty `right` is a Python IndexError, unreachable from any
        // repr the shim produces.
        let last = *chars.last().ok_or_else(|| {
            N2WError::Index("string index out of range".into())
        })?;
        // n[-2:]: the last two chars, or the whole (1-char) string.
        let last2: String = if chars.len() >= 2 {
            chars[chars.len() - 2..].iter().collect()
        } else {
            right.to_string()
        };
        // 10 ** len(n) as a BigInt — len can exceed the small scales the corpus
        // exercises (Decimal precision is arbitrary).
        let ten_pow = BigInt::from(10).pow(chars.len() as u32);

        if last == '1' && last2 != "11" {
            self.int2word(&ten_pow, false, Opts { gender: GenderArg::Idx(GENDER_F), ..Opts::default() })
        } else {
            self.int2word(
                &ten_pow,
                false,
                Opts { case: CASE_G, plural: true, ..Opts::default() },
            )
        }
    }
}

/// Python's `int(s)` for the ASCII digit fragments `cardinal_from_str` splits
/// out. `int("")` / `int("-")` raise ValueError; `int("005") == 5`.
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Shortest round-trip decimal digits of a finite non-negative `f64`, as
/// `(digits, decpt)` where the value is `0.digits * 10**decpt`. Mirrors CPython
/// dtoa's round-half-to-**even** so `py_float_repr` reproduces `repr(float)`.
///
/// (Ported verbatim from the proven PA/PL float ports — `str(float)` is the
/// entire specification of RU's float arm, exactly as it is theirs.)
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

/// Python's `str(float)` (== `repr(float)`) — CPython `format_float_short(...,
/// 'r', ...)`. Rust's `{}` cannot stand in: it never switches to exponent
/// notation and prints `1`, not `1.0`, for integral floats — both of which RU's
/// `str(number)`-driven float arm depends on.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0".
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
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(value)` for a float-or-Decimal — the `%s` in base
/// `verify_ordinal`'s error messages ("Cannot treat float %s as ordinal.").
fn float_value_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => py_float_repr(*value),
        FloatValue::Decimal { value, .. } => py_decimal_str(value),
    }
}

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__`, default context.
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`, so trailing zeros (`"1.10"`) and positive exponents
/// (`"1E+2"`) round-trip. Ported verbatim from the proven PA float port.
///
/// **Known divergence** (negative zero): `BigInt` has no signed zero, so the
/// binding demotes `Decimal("-0.0")` to `FloatValue::Float { -0.0 }` before
/// this module runs — indistinguishable from a true float `-0.0`. Python
/// treats them differently in RU: the float takes the float-zero guard
/// ("ноль") while the Decimal takes the string path ("минус ноль целых
/// ноль десятых"). The float behaviour is kept (it is the reachable one for
/// real floats and the corpus's float rows); the Decimal("-0.0")
/// cardinal/year rows cannot be matched from this file.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string();
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 { leftdigits } else { 1 };

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
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `utils.splitbyx(n, x)` with `format_int=True`.
///
/// `s` is the decimal form of a non-negative BigInt, so it is pure ASCII and
/// byte slicing is safe. Chunks of 3 digits always fit in a `u32`.
fn splitbyx(s: &str, x: usize) -> Vec<u32> {
    let length = s.len();
    let mut out = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(s[..start].parse().unwrap());
        }
        let mut i = start;
        while i < length {
            out.push(s[i..i + x].parse().unwrap());
            i += x;
        }
    } else {
        out.push(s.parse().unwrap());
    }
    out
}

/// Python's `utils.get_digits(n)`: `reversed(("%03d" % n)[-3:])`, i.e.
/// `(ones, tens, hundreds)`. The `[-3:]` truncation is mirrored by the `% 10`
/// on the hundreds digit; `splitbyx(_, 3)` guarantees `x <= 999` regardless.
fn get_digits(x: u32) -> (u32, u32, u32) {
    (x % 10, (x / 10) % 10, (x / 100) % 10)
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangRu {
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
        // Python's `setup()`. Note: no trailing space, unlike the base default.
        "минус"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, true, Opts::default())
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        self.int2word(value, false, Opts::default())
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(value.to_string())
    }

    /// Inherited unchanged from `base.py`: `to_year` just defers to
    /// `to_cardinal` (no BC/AD suffix handling in RU).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- grammatical kwargs ----------------------------------------------
    //
    // Python signatures: `to_cardinal(self, number, case=D_CASE,
    // plural=D_PLURAL, gender=D_GENDER, animate=D_ANIMATE)` and `to_ordinal`
    // identical. `to_ordinal_num(self, value)` takes none, so the base
    // default (empty bag or Python fallback) is already exact; `to_year` is
    // base's `(self, val, **kwargs)` which silently swallows anything — no
    // corpus row exercises it, so the conservative Python fallback stands.

    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case", "plural", "gender", "animate"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let o = self.opts_from_kwargs(kw)?;
        self.int2word(value, true, o)
    }

    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case", "plural", "gender", "animate"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        // verify_ordinal runs before any dict lookup: to_ordinal(-5,
        // case="xx") is TypeError, not KeyError (kwargs corpus).
        self.verify_ordinal(value)?;
        let o = self.opts_from_kwargs(kw)?;
        self.int2word(value, false, o)
    }

    // ---- float / Decimal entry routing ------------------------------------

    /// `to_cardinal(float/Decimal)` in full. RU's own `to_cardinal` never
    /// routes whole values to the integer path: it stringifies and branches
    /// on `"." in str(number)`, so `5.0` keeps its ".0" tail ("пять целых
    /// ноль десятых") and `Decimal("5")` (no dot) renders as an integer.
    /// `to_cardinal_float` below implements exactly that.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // A genuine float `-0.0` takes the float-zero guard in
        // `to_cardinal_float` and renders "ноль". The type-ambiguous
        // `Decimal("-0.0")` (which BigInt cannot hold) is intercepted earlier
        // by the binding via `neg_zero_decimal` below, so it never reaches
        // here demoted — every value that arrives is served natively.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: base `verify_ordinal(number)`, then
    /// `int2word(int(str(int(number))), cardinal=False, <defaults>)`.
    ///
    /// * non-whole -> TypeError `errmsg_floatord` ("Cannot treat float %s as
    ///   ordinal.");
    /// * negative whole -> TypeError `errmsg_negord` — but `-0.0` passes both
    ///   checks (`abs(-0.0) == -0.0` is True) and ordinalizes to "нулевой";
    /// * whole -> ordinal words, kwargs at the module defaults.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                float_value_str(value)
            ))),
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        float_value_str(value)
                    )))
                } else {
                    self.int2word(&i, false, Opts::default())
                }
            }
        }
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` then
    /// `str(int(value))` — so `5.00` -> "5", `-0.0` -> "0", `1E+20` ->
    /// "100000000000000000000", and non-whole/negative raise like to_ordinal.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            ))),
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )))
                } else {
                    Ok(i.to_string())
                }
            }
        }
    }

    /// `to_year(float/Decimal)`: base's `to_year` is `self.to_cardinal(val)`,
    /// i.e. RU's own string-driven cardinal — identical to the cardinal entry,
    /// negative-zero decline included (see `cardinal_float_entry`).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `Decimal('-0.0')` per mode. BigDecimal has no signed zero, so the
    /// binding demotes it to `Float { -0.0 }` unless this hook claims it. RU's
    /// `to_cardinal`/`to_year` read `str(number)` == "-0.0", take the "."
    /// string path and emit the negword-prefixed decimal grammar ("минус ноль
    /// целых ноль десятых") — unlike the genuine float, which the float-zero
    /// guard collapses to "ноль". Only cardinal/year diverge; ordinal and
    /// ordinal_num run `int(str(int(value)))` == 0 either way ("нулевой" / "0"),
    /// so those return `None` and ride the exact `Float { -0.0 }` demotion.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            "cardinal" | "year" => Some(self.cardinal_from_str("-0.0", &Kwargs::default())),
            _ => None,
        }
    }

    // ---- string inputs -----------------------------------------------------

    /// Base `str_to_number` (`Decimal(value)`), with one RU-specific fixup:
    /// `Decimal("Infinity")` parses fine, but RU's `to_cardinal` then does
    /// `int(str(number))` on the dotless, e-less string "Infinity", which is
    /// `ValueError: invalid literal for int() with base 10: 'Infinity'` — not
    /// the OverflowError of base's `int(Decimal('Infinity'))`. The error is
    /// raised here because the shared Inf sentinel maps to OverflowError.
    ///
    /// (For `to_ordinal("Infinity")` Python *would* OverflowError inside
    /// `verify_ordinal`'s `int(value)`; only the cardinal rows exist in the
    /// corpus, and the mode is not visible from this hook.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                if negative { "-Infinity" } else { "Infinity" }
            ))),
            other => Ok(other),
        }
    }

    /// The float / `Decimal` cardinal path.
    ///
    /// RU overrides **`to_cardinal`** (not `to_cardinal_float`) and handles
    /// non-integers inline off `str(number)`, so `base.float2tuple` /
    /// `to_cardinal_float` are never reached. Two float-only wrinkles:
    ///
    /// * ~~`if abs(number) < 0.01: return self._int2word(0, ...)`~~ — in
    ///   Python a *float* whose magnitude is below one hundredth collapsed to
    ///   "ноль" (so `0.005` → "ноль"), while `Decimal("0.005")` read "ноль
    ///   целых пять тысячных". Fixed (gladiaio/num2words2#208): such a float
    ///   reads its digits like the Decimal; only a float zero (`-0.0`) still
    ///   takes the shortcut.
    ///
    /// `precision_override` (the `precision=` kwarg) is ignored, exactly as
    /// Python's `to_cardinal(self, number, ...)` — which has no `precision`
    /// parameter — ignores it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        self.cardinal_float_with(value, &Kwargs::default())
    }

    /// `to_cardinal(float/Decimal, case=, plural=, gender=, animate=)`. The
    /// kwargs reach only the float-zero and dot-less (Decimal("5"),
    /// exponent repr) branches; the "." branch hardcodes `gender="f"` and
    /// default case for both parts, so `to_cardinal(125.1, gender="f")` is
    /// the plain "сто двадцать пять целых одна десятая" and a bad `case=`
    /// raises no KeyError there.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["case", "plural", "gender", "animate"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let _ = precision_override;
        self.cardinal_float_with(value, kw)
    }

    // ---- currency -------------------------------------------------------
    //
    // RU overrides `pluralize`, `_money_verbose`, `_cents_verbose` and
    // `to_currency`; everything else on the currency path is Base's.
    //
    // Three tables decide the rest, and only one of them is RU's own:
    //
    // * CURRENCY_FORMS  — declared on `Num2Word_RU` (see `build_currency_forms`).
    // * CURRENCY_ADJECTIVES — Base's, empty at runtime, so `currency_adjective`
    //   keeps its `None` default and `adjective=True` is a no-op.
    // * CURRENCY_PRECISION — Base's, and *still* empty at runtime: EN rebinds
    //   the name in `__init__` rather than mutating it, so its mils table never
    //   reaches Base. Every RU currency therefore has divisor 100, which is the
    //   `currency_precision` default. RU has no 3-decimal and no 0-decimal
    //   currency at all, so `_cents_terse` and the `divisor == 1` branch are
    //   inherited untouched.

    fn lang_name(&self) -> &str {
        "Num2Word_RU"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_RU.pluralize` — the one/few/many rule.
    ///
    /// Every reachable call site passes a non-negative `n` (`abs(val)` for the
    /// unit, `parse_currency_parts`' already-absolute count for the subunit),
    /// but `mod_floor` is used anyway so the sign behaviour tracks Python's
    /// flooring `%` rather than Rust's truncating one.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        // mod_floor by a positive modulus lands in 0..modulus, so both
        // conversions are total.
        let m100 = n.mod_floor(&BigInt::from(100)).to_u32().unwrap_or(0);
        let m10 = n.mod_floor(&BigInt::from(10)).to_u32().unwrap_or(0);
        let i = if matches!(m100, 11..=14) {
            2
        } else if m10 == 1 {
            0
        } else if matches!(m10, 2..=4) {
            1
        } else {
            2
        };
        // Python indexes the tuple directly, so a shorter tuple would be an
        // IndexError. Every RU entry has exactly three forms, making this
        // unreachable — but the exception type is preserved regardless.
        forms
            .get(i)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Python's `_money_verbose`. UAH ("гривна") is feminine; the rest are
    /// masculine.
    ///
    /// Reached from Base's float path and from `to_cheque` — but *not* from
    /// RU's own integer branch, which calls `to_cardinal` instead. See
    /// `to_currency`.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let gender = if currency == "UAH" { GENDER_F } else { GENDER_M };
        self.int2word(number, true, Opts { gender: GenderArg::Idx(gender), ..Opts::default() })
    }

    /// Python's `_cents_verbose`. "копейка" (UAH/RUB/BYN) is feminine; "цент",
    /// "пенни", "грош" etc. take the masculine default.
    ///
    /// Its `str`/`float` preamble is unreachable from here: Base only ever
    /// hands `_cents_verbose` a whole `int` — the fractional-subunit case
    /// returns earlier through the `to_cardinal(float(right))` branch — and the
    /// trait signature already narrows the argument to `BigInt`.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let gender = if matches!(currency, "UAH" | "RUB" | "BYN") { GENDER_F } else { GENDER_M };
        self.int2word(number, true, Opts { gender: GenderArg::Idx(gender), ..Opts::default() })
    }

    /// Python's `Num2Word_RU.to_currency`.
    ///
    /// RU intercepts the integer case and hands everything else to
    /// `Num2Word_Base.to_currency`. Two deviations from Base are load-bearing:
    ///
    /// 1. `isinstance(val, float) and val == int(val)` demotes a whole float to
    ///    an `int` *before* the int check, so `1.0` renders "один евро" with no
    ///    cents segment where Base would say "один евро, ноль центов". The
    ///    comment in the original blames scientific notation (`1e+18` reprs as
    ///    a whole float), but the rule applies to every whole float.
    /// 2. The int branch calls `to_cardinal`, **not** `_money_verbose`, so the
    ///    unit is always masculine. UAH is feminine, which makes
    ///    `to_currency(1, "UAH")` say "один гривна" while `to_currency(1.5,
    ///    "UAH")` says "одна гривна". Bug, preserved.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // (1) `if isinstance(val, float) and val == int(val): val = int(val)`.
        //
        // `is_integer()` is the `val == int(val)` half. The `isinstance(val,
        // float)` half cannot be tested here — `CurrencyValue::Decimal` covers
        // Python `float` *and* `Decimal`, and `has_decimal` is true for both
        // `1.0` and `Decimal("1.0")` — so every integral Decimal is demoted.
        // That is exact for the float inputs the shim actually produces.
        let val = match val {
            CurrencyValue::Decimal { value, .. } if value.is_integer() => {
                // `int()` truncates toward zero and so does `with_scale(0)`;
                // the value is integral here either way.
                CurrencyValue::Int(value.with_scale(0).as_bigint_and_exponent().0)
            }
            other => other.clone(),
        };

        if let CurrencyValue::Int(v) = &val {
            // Python: `except (KeyError, AttributeError): return super()...`.
            // Base then re-raises the miss as NotImplementedError, so an
            // unknown code takes the same exit as the float path.
            //
            // `adjective` is silently dropped on this branch, but
            // CURRENCY_ADJECTIVES is empty for RU, so Base would not have
            // applied it either.
            if let Some(forms) = self.currency_forms.get(currency) {
                // negword is used raw here — no `.strip()`, unlike Base — and
                // the whole line is `.strip()`ed instead, which absorbs the
                // empty slot left by a positive value.
                let minus = if v.is_negative() { self.negword() } else { "" };
                let abs = v.abs();
                let money = self.to_cardinal(&abs)?;
                let unit = self.pluralize(&abs, &forms.unit)?;
                return Ok(format!("{} {} {}", minus, money, unit).trim().to_string());
            }
        }

        // Floats (and unknown codes) fall through to `super().to_currency`.
        crate::currency::default_to_currency(
            self,
            &val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn c(n: i64) -> String {
        LangRu::new().to_cardinal(&BigInt::from(n)).unwrap()
    }
    fn o(n: i64) -> String {
        LangRu::new().to_ordinal(&BigInt::from(n)).unwrap()
    }

    #[test]
    fn cardinals() {
        assert_eq!(c(0), "ноль");
        assert_eq!(c(1), "один");
        assert_eq!(c(11), "одиннадцать");
        assert_eq!(c(20), "двадцать");
        assert_eq!(c(42), "сорок два");
        assert_eq!(c(100), "сто");
        assert_eq!(c(101), "сто один");
        assert_eq!(c(1000), "одна тысяча");
        assert_eq!(c(2000), "две тысячи");
        assert_eq!(c(1000000), "один миллион");
        assert_eq!(c(-42), "минус сорок два");
        assert_eq!(c(-1000), "минус одна тысяча");
    }

    #[test]
    fn ordinals() {
        assert_eq!(o(0), "нулевой");
        assert_eq!(o(3), "третий");
        assert_eq!(o(100), "сотый");
        assert_eq!(o(200), "двухсотый");
        assert_eq!(o(1000), "тысячный");
        assert_eq!(o(1001), "тысяча первый");
        assert_eq!(o(2000), "двухтысячный");
        assert_eq!(o(10000), "десятитысячный");
        assert_eq!(o(100000), "стотысячный");
        assert_eq!(o(1000000), "миллионный");
        // Faithful reproduction of the TENS_ORD-in-every-chunk bug.
        assert_eq!(o(10001), "десятый тысяч первый");
        assert_eq!(o(100001), "сотый тысяч первый");
    }

    #[test]
    fn big() {
        let l = LangRu::new();
        let sextillion: BigInt = BigInt::from(10).pow(21);
        assert_eq!(l.to_cardinal(&sextillion).unwrap(), "один секстиллион");
        assert_eq!(l.to_ordinal(&sextillion).unwrap(), "секстиллионный");
    }

    // ---- currency -------------------------------------------------------

    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A float argument, as the shim builds it: `has_decimal` is true because
    /// Python's guard is `isinstance(val, float) or "." in str(val)`.
    fn f(s: &str) -> CurrencyValue {
        CurrencyValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            has_decimal: true,
            is_float: true,
        }
    }
    fn i(n: i64) -> CurrencyValue {
        CurrencyValue::Int(BigInt::from(n))
    }
    fn cur(v: CurrencyValue, code: &str) -> String {
        LangRu::new().to_currency(&v, code, true, None, false).unwrap()
    }

    #[test]
    fn currency_ints_skip_cents() {
        assert_eq!(cur(i(0), "EUR"), "ноль евро");
        assert_eq!(cur(i(1), "EUR"), "один евро");
        assert_eq!(cur(i(2), "EUR"), "два евро");
        assert_eq!(cur(i(100), "EUR"), "сто евро");
        assert_eq!(cur(i(1000000), "EUR"), "один миллион евро");
        assert_eq!(cur(i(0), "USD"), "ноль долларов");
        assert_eq!(cur(i(1), "USD"), "один доллар");
        assert_eq!(cur(i(2), "USD"), "два доллара");
        assert_eq!(cur(i(1), "GBP"), "один фунт стерлингов");
        assert_eq!(cur(i(-12), "USD"), "минус двенадцать долларов");
    }

    /// `1.0` is a *float*, yet RU demotes it to `int` and drops the cents
    /// segment — Base alone would say "один евро, ноль центов".
    #[test]
    fn currency_whole_float_demotes_to_int() {
        assert_eq!(cur(f("1.0"), "EUR"), "один евро");
        assert_eq!(cur(f("1.0"), "USD"), "один доллар");
        assert_eq!(cur(f("1.0"), "GBP"), "один фунт стерлингов");
        assert_eq!(cur(f("-1.0"), "USD"), "минус один доллар");
        // repr(1e18) has no dot, but is_integer() still catches it.
        assert_eq!(cur(f("1e+18"), "EUR"), "один квинтиллион евро");
    }

    #[test]
    fn currency_floats_render_cents() {
        assert_eq!(cur(f("12.34"), "EUR"), "двенадцать евро, тридцать четыре цента");
        assert_eq!(cur(f("0.01"), "EUR"), "ноль евро, один цент");
        assert_eq!(cur(f("0.5"), "EUR"), "ноль евро, пятьдесят центов");
        assert_eq!(cur(f("-12.34"), "EUR"), "минус двенадцать евро, тридцать четыре цента");
        assert_eq!(
            cur(f("99.99"), "EUR"),
            "девяносто девять евро, девяносто девять центов"
        );
        assert_eq!(
            cur(f("1234.56"), "USD"),
            "одна тысяча двести тридцать четыре доллара, пятьдесят шесть центов"
        );
        assert_eq!(
            cur(f("1234.56"), "GBP"),
            "одна тысяча двести тридцать четыре фунта стерлингов, пятьдесят шесть пенни"
        );
    }

    /// `_cents_verbose` is feminine for копейка, masculine for цент.
    #[test]
    fn currency_cents_gender() {
        assert_eq!(cur(f("1.01"), "RUB"), "один рубль, одна копейка");
        assert_eq!(cur(f("1.01"), "BYN"), "один белорусский рубль, одна копейка");
        assert_eq!(cur(f("1.01"), "USD"), "один доллар, один цент");
    }

    /// The int branch calls `to_cardinal` (masculine) while the float branch
    /// calls `_money_verbose` (feminine for UAH) — so the unit disagrees with
    /// itself across the int/float boundary. Faithful port of the bug.
    #[test]
    fn currency_uah_gender_bug() {
        assert_eq!(cur(i(1), "UAH"), "один гривна");
        assert_eq!(cur(f("21.0"), "UAH"), "двадцать один гривна");
        assert_eq!(cur(f("1.5"), "UAH"), "одна гривна, пятьдесят копеек");
        assert_eq!(cur(f("21.5"), "UAH"), "двадцать одна гривна, пятьдесят копеек");
    }

    /// RU declares its own CURRENCY_FORMS, so EN's mutation of the shared
    /// Num2Word_EUR dict never reaches it.
    #[test]
    fn currency_unknown_code_not_implemented() {
        let l = LangRu::new();
        for code in ["JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for v in [i(1), f("12.34"), f("1.0")] {
                match l.to_currency(&v, code, true, None, false) {
                    Err(N2WError::NotImplemented(m)) => assert_eq!(
                        m,
                        format!("Currency code \"{}\" not implemented for \"Num2Word_RU\"", code)
                    ),
                    other => panic!("{} {:?}: expected NotImplemented, got {:?}", code, v, other),
                }
            }
        }
    }

    #[test]
    fn cheque() {
        let l = LangRu::new();
        let v = BigDecimal::from_str("1234.56").unwrap();
        assert_eq!(
            l.to_cheque(&v, "EUR").unwrap(),
            "ОДНА ТЫСЯЧА ДВЕСТИ ТРИДЦАТЬ ЧЕТЫРЕ AND 56/100 ЕВРО"
        );
        assert_eq!(
            l.to_cheque(&v, "USD").unwrap(),
            "ОДНА ТЫСЯЧА ДВЕСТИ ТРИДЦАТЬ ЧЕТЫРЕ AND 56/100 ДОЛЛАРОВ"
        );
        assert_eq!(
            l.to_cheque(&v, "GBP").unwrap(),
            "ОДНА ТЫСЯЧА ДВЕСТИ ТРИДЦАТЬ ЧЕТЫРЕ AND 56/100 ФУНТОВ СТЕРЛИНГОВ"
        );
        assert!(matches!(
            l.to_cheque(&v, "JPY"),
            Err(N2WError::NotImplemented(_))
        ));
    }

    /// `cents=False` takes Base's `_cents_terse`, zero-padded to the divisor's
    /// width (100 -> 2 digits for every RU currency).
    #[test]
    fn currency_terse_cents() {
        let l = LangRu::new();
        assert_eq!(
            l.to_currency(&f("12.34"), "EUR", false, None, false).unwrap(),
            "двенадцать евро, 34 цента"
        );
        assert_eq!(
            l.to_currency(&f("12.05"), "EUR", false, None, false).unwrap(),
            "двенадцать евро, 05 центов"
        );
    }

    /// pluralize's one/few/many boundaries, via the unit slot.
    #[test]
    fn currency_pluralize_boundaries() {
        for (n, want) in [
            (1, "доллар"),
            (2, "доллара"),
            (4, "доллара"),
            (5, "долларов"),
            (11, "долларов"),
            (12, "долларов"),
            (14, "долларов"),
            (21, "доллар"),
            (22, "доллара"),
            (25, "долларов"),
            (100, "долларов"),
            (111, "долларов"),
            (121, "доллар"),
        ] {
            let got = cur(i(n), "USD");
            assert!(got.ends_with(want), "{}: {:?} should end with {:?}", n, got, want);
        }
    }

    // ---- float / Decimal cardinal path ---------------------------------

    fn ff(v: f64) -> String {
        LangRu::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision: 0 }, None)
            .unwrap()
    }
    fn dd(s: &str, p: u32) -> String {
        LangRu::new()
            .to_cardinal_float(
                &FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: p },
                None,
            )
            .unwrap()
    }

    #[test]
    fn cardinal_floats() {
        // Exact corpus rows: "lang": "ru", "to": "cardinal", float args.
        assert_eq!(ff(0.0), "ноль");
        assert_eq!(ff(0.5), "ноль целых пять десятых");
        assert_eq!(ff(1.0), "одна целая ноль десятых");
        assert_eq!(ff(1.5), "одна целая пять десятых");
        assert_eq!(ff(2.25), "две целых двадцать пять сотых");
        assert_eq!(ff(3.14), "три целых четырнадцать сотых");
        assert_eq!(ff(0.01), "ноль целых одна сотая");
        assert_eq!(ff(0.1), "ноль целых одна десятая");
        assert_eq!(ff(0.99), "ноль целых девяносто девять сотых");
        assert_eq!(ff(1.01), "одна целая одна сотая");
        assert_eq!(ff(12.34), "двенадцать целых тридцать четыре сотых");
        assert_eq!(ff(99.99), "девяносто девять целых девяносто девять сотых");
        assert_eq!(ff(100.5), "сто целых пять десятых");
        assert_eq!(
            ff(1234.56),
            "одна тысяча двести тридцать четыре целых пятьдесят шесть сотых"
        );
        assert_eq!(ff(-0.5), "минус ноль целых пять десятых");
        assert_eq!(ff(-1.5), "минус одна целая пять десятых");
        assert_eq!(ff(-12.34), "минус двенадцать целых тридцать четыре сотых");
        // The f64-artefact cases: RU reads the shortest repr, not float2tuple.
        assert_eq!(ff(1.005), "одна целая пять тысячных");
        assert_eq!(ff(2.675), "две целых шестьсот семьдесят пять тысячных");
    }

    #[test]
    fn cardinal_float_below_hundredth_reads_its_digits() {
        // Python's abs(number) < 0.01 shortcut returned "ноль" here (#208).
        assert_eq!(ff(0.001), "ноль целых одна тысячная");
        assert_eq!(ff(0.005), "ноль целых пять тысячных");
        assert_eq!(ff(-0.005), "минус ноль целых пять тысячных");
        assert_eq!(ff(0.00001), "ноль целых одна стотысячная");
        assert_eq!(ff(-0.0), "ноль");
    }

    #[test]
    fn cardinal_float_exponent_is_integer() {
        // str(1e16) == "1e+16" -> no dot -> int(float(n)) -> integer words.
        assert_eq!(ff(1e16), "десять квадриллионов");
        assert_eq!(ff(1e18), "один квинтиллион");
        assert_eq!(
            ff(123456789.5),
            "сто двадцать три миллиона четыреста пятьдесят шесть тысяч \
             семьсот восемьдесят девять целых пять десятых"
        );
    }

    #[test]
    fn cardinal_decimals() {
        // Exact corpus rows: "lang": "ru", "to": "cardinal_dec".
        assert_eq!(dd("0.01", 2), "ноль целых одна сотая");
        assert_eq!(dd("1.10", 2), "одна целая десять сотых");
        assert_eq!(dd("12.345", 3), "двенадцать целых триста сорок пять тысячных");
        assert_eq!(dd("0.001", 3), "ноль целых одна тысячная");
        assert_eq!(
            dd("98746251323029.99", 2),
            "девяносто восемь триллионов семьсот сорок шесть миллиардов \
             двести пятьдесят одна миллион триста двадцать три тысячи \
             двадцать девять целых девяносто девять сотых"
        );
        // Decimal has no <0.01 guard and a bare-integer Decimal takes the int arm.
        assert_eq!(dd("5", 0), "пять");
        assert_eq!(dd("-5", 0), "минус пять");
    }

    // ---- float/Decimal entry routing ------------------------------------

    fn fentry(v: f64) -> String {
        LangRu::new()
            .cardinal_float_entry(&FloatValue::Float { value: v, precision: 1 }, None)
            .unwrap()
    }

    /// RU never routes whole floats to the integer path: `str(5.0)` has a
    /// dot, so the decimal grammar applies (wholefloat corpus).
    #[test]
    fn cardinal_float_entry_keeps_decimal_tail() {
        assert_eq!(fentry(1.0), "одна целая ноль десятых");
        assert_eq!(fentry(2.0), "две целых ноль десятых");
        assert_eq!(fentry(-21.0), "минус двадцать одна целая ноль десятых");
        assert_eq!(fentry(1000000.0), "одна миллион целых ноль десятых");
        // A genuine float -0.0 is served natively: the abs()<0.01 guard in
        // to_cardinal_float renders it "ноль". (Decimal("-0.0") is intercepted
        // earlier by `neg_zero_decimal` and never reaches here demoted.)
        assert_eq!(
            LangRu::new()
                .cardinal_float_entry(&FloatValue::Float { value: -0.0, precision: 1 }, None)
                .unwrap(),
            "ноль"
        );
        let l = LangRu::new();
        // Decimal("5.00") keeps its written scale -> "ноль сотых".
        let d = FloatValue::Decimal {
            value: BigDecimal::from_str("5.00").unwrap(),
            precision: 2,
        };
        assert_eq!(
            l.cardinal_float_entry(&d, None).unwrap(),
            "пять целых ноль сотых"
        );
        assert_eq!(l.year_float_entry(&d).unwrap(), "пять целых ноль сотых");
    }

    /// to_ordinal(float): verify_ordinal, then int() — whole floats
    /// ordinalize, non-whole and negative raise TypeError (wholefloat corpus).
    #[test]
    fn ordinal_float_entry_verifies() {
        let l = LangRu::new();
        let f = |v: f64| FloatValue::Float { value: v, precision: 1 };
        assert_eq!(l.ordinal_float_entry(&f(5.0)).unwrap(), "пятый");
        assert_eq!(l.ordinal_float_entry(&f(-0.0)).unwrap(), "нулевой");
        assert_eq!(l.ordinal_float_entry(&f(1e16)).unwrap(), "десятиквадриллионный");
        match l.ordinal_float_entry(&f(2.5)) {
            Err(N2WError::Type(m)) => assert_eq!(m, "Cannot treat float 2.5 as ordinal."),
            other => panic!("expected TypeError, got {:?}", other),
        }
        match l.ordinal_float_entry(&f(-3.0)) {
            Err(N2WError::Type(m)) => {
                assert_eq!(m, "Cannot treat negative num -3.0 as ordinal.")
            }
            other => panic!("expected TypeError, got {:?}", other),
        }
        // Decimal whole -> ordinal; Decimal fractional -> TypeError.
        let d = |s: &str| FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: 2,
        };
        assert_eq!(l.ordinal_float_entry(&d("5.00")).unwrap(), "пятый");
        assert_eq!(l.ordinal_float_entry(&d("1E+2")).unwrap(), "сотый");
        assert!(matches!(l.ordinal_float_entry(&d("1.50")), Err(N2WError::Type(_))));
    }

    /// to_ordinal_num(float): verify_ordinal then str(int(value)).
    #[test]
    fn ordinal_num_float_entry_verifies() {
        let l = LangRu::new();
        let f = |v: f64| FloatValue::Float { value: v, precision: 1 };
        assert_eq!(l.ordinal_num_float_entry(&f(5.0), "5.0").unwrap(), "5");
        assert_eq!(l.ordinal_num_float_entry(&f(-0.0), "-0.0").unwrap(), "0");
        assert_eq!(
            l.ordinal_num_float_entry(&f(1e20), "1e+20").unwrap(),
            "100000000000000000000"
        );
        assert!(matches!(
            l.ordinal_num_float_entry(&f(2.5), "2.5"),
            Err(N2WError::Type(_))
        ));
        assert!(matches!(
            l.ordinal_num_float_entry(&f(-1.0), "-1.0"),
            Err(N2WError::Type(_))
        ));
    }

    /// num2words("Infinity", lang="ru"): Decimal parses, then RU's
    /// `int("Infinity")` raises ValueError — not base's OverflowError.
    #[test]
    fn infinity_string_is_value_error() {
        let l = LangRu::new();
        match l.str_to_number("Infinity") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'Infinity'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        assert!(matches!(l.str_to_number("-Infinity"), Err(N2WError::Value(_))));
        // NaN keeps the shared sentinel (shim maps it to ValueError too).
        assert!(matches!(l.str_to_number("nan"), Ok(ParsedNumber::NaN)));
        assert!(matches!(l.str_to_number("5"), Ok(ParsedNumber::Dec(_))));
    }

    // ---- grammatical kwargs ---------------------------------------------

    use crate::base::{KwVal, Kwargs};

    fn kw(items: &[(&str, KwVal)]) -> Kwargs {
        Kwargs(items.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
    }
    fn s(v: &str) -> KwVal {
        KwVal::Str(v.to_string())
    }

    /// Exact kwargs-corpus rows.
    #[test]
    fn cardinal_kwargs() {
        let l = LangRu::new();
        let ck = |n: i64, items: &[(&str, KwVal)]| {
            l.to_cardinal_kw(&BigInt::from(n), &kw(items)).unwrap()
        };
        assert_eq!(ck(1234, &[("case", s("g"))]), "одной тысячи двухсот тридцати четырёх");
        assert_eq!(ck(1234, &[("case", s("a"))]), "одну тысячу двести тридцать четырёх");
        assert_eq!(
            ck(1234, &[("case", s("i"))]),
            "одной тысячей двумястами тридцатью четырьмя"
        );
        assert_eq!(ck(-5, &[("case", s("g"))]), "минус пяти");
        assert_eq!(ck(21, &[("case", s("родительный"))]), "двадцати одного");
        assert_eq!(ck(21, &[("case", s("n")), ("gender", s("f"))]), "двадцать одна");
        assert_eq!(ck(21, &[("case", s("n")), ("gender", s("ж"))]), "двадцать одна");
        assert_eq!(
            ck(2, &[("case", s("n")), ("plural", KwVal::Bool(true))]),
            "двое"
        );
        assert_eq!(
            ck(
                1,
                &[
                    ("case", s("a")),
                    ("plural", KwVal::Bool(false)),
                    ("animate", KwVal::Bool(false))
                ]
            ),
            "один"
        );
        // Unknown case: KeyError, even for 0.
        assert!(matches!(
            l.to_cardinal_kw(&BigInt::from(0), &kw(&[("case", s("xx"))])),
            Err(N2WError::Key(_))
        ));
        // Bad gender is lazy: ONES[5] has plain-string case slots.
        assert_eq!(ck(5, &[("case", s("n")), ("gender", s("xx"))]), "пять");
        assert!(matches!(
            l.to_cardinal_kw(&BigInt::from(1), &kw(&[("gender", s("xx"))])),
            Err(N2WError::Key(_))
        ));
        // Unknown kwarg -> Fallback (decline) -> Python raises the TypeError.
        assert!(matches!(
            l.to_cardinal_kw(&BigInt::from(1), &kw(&[("foo", s("bar"))])),
            Err(N2WError::Fallback(_))
        ));
    }

    /// Exact kwargs-corpus rows: ordinal, incl. verify-before-KeyError.
    #[test]
    fn ordinal_kwargs() {
        let l = LangRu::new();
        let ok = |n: i64, items: &[(&str, KwVal)]| {
            l.to_ordinal_kw(&BigInt::from(n), &kw(items)).unwrap()
        };
        assert_eq!(ok(1234, &[("case", s("g"))]), "тысяча двести тридцать четвёртого");
        assert_eq!(ok(21, &[("case", s("i"))]), "двадцать первым");
        assert_eq!(ok(3, &[("case", s("d"))]), "третьему");
        assert_eq!(ok(100, &[("case", s("p"))]), "сотом");
        assert_eq!(
            ok(1, &[("case", s("n")), ("plural", KwVal::Bool(true))]),
            "первые"
        );
        // -5 with a *bad* case is still TypeError: verify_ordinal runs first.
        assert!(matches!(
            l.to_ordinal_kw(&BigInt::from(-5), &kw(&[("case", s("xx"))])),
            Err(N2WError::Type(_))
        ));
        assert!(matches!(
            l.to_ordinal_kw(&BigInt::from(5), &kw(&[("case", s("xx"))])),
            Err(N2WError::Key(_))
        ));
    }

    #[test]
    fn negative_ordinal_is_type_error() {
        let l = LangRu::new();
        assert!(matches!(
            l.to_ordinal(&BigInt::from(-1)),
            Err(N2WError::Type(_))
        ));
        assert!(matches!(
            l.to_ordinal_num(&BigInt::from(-1)),
            Err(N2WError::Type(_))
        ));
    }
}
