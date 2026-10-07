//! Latin (registry key `"la"`).
//!
//! The **number words** (cardinal / ordinal / ordinal_num / year, plus the
//! float grammar) are a port of PR savoirfairelinux/num2words#657 (Piotr
//! Balwierz's `add-latin` branch): properly declined cardinals across 3 genders
//! × 6 cases for the declined forms (1, 2, 3, 200..900), Classical macrons
//! (`ūnus`, `trēs`, strippable with `macrons=False`), true ordinals with a Roman
//! `to_ordinal_num` (`4` → "IV"), and BC years ("ante Chrīstum"). See
//! [`la_to_cardinal_pos`] / [`la_ordinal_int`]. `MAXVAL` is 10^18; larger values
//! raise `OverflowError`. PR #657 only knew the simple ordinals and returned
//! the *cardinal* for every compound (21 → "vīgintī ūnus"); fixed (#181):
//! compounds below 10^6 are built from ordinal parts ("vīcēsimus prīmus",
//! "centēsimus prīmus", "bis mīllēsimus", "deciēs mīllēsimus"), and other
//! ordinals from 10^6 up (only 10^6 itself is in the table) raise
//! `OverflowError`. See [`la_ordinal_compound`].
//!
//! The **currency** path is deliberately NOT from PR #657 — that module ships
//! no `CURRENCY_FORMS` and would raise NotImplementedError. It keeps the fork's
//! own hand-rolled `to_currency` + [`LangLa::int_to_word`] (a basic, no-macron
//! cardinal), so `to_currency` output is unchanged from the frozen corpus.
//! `int_to_word`'s quirks (bugs 3–8 below) therefore still describe the
//! currency surface, but no longer the plain cardinals.
//!
//! `setup()` only rebinds plain scalars (`negword`, `pointword`, `ones`,
//! `tens`, `hundred`, `thousand`, `million`), so they are module consts here
//! rather than struct fields — `LangLa` is a unit struct.
//!
//! Inherited from `Num2Word_Base` and **not** overridden by LA:
//!   * `str_to_number(value) -> Decimal(value)`. LA stashes no flag in it, so
//!     there is **no cross-call mutable-state handshake** of the `lang_ES`
//!     `_pending_ordinal` kind. The Rust path is safe to dispatch to.
//!   * `to_cheque` — base's, so it goes through `currency::default_to_cheque`
//!     and the strict [`Lang::currency_forms`] lookup. This is the *only* LA
//!     entry point that can raise on an unknown currency code.
//!   * `_money_verbose` -> `self.to_cardinal(number)`, which resolves to LA's
//!     override. The trait default does the same, so it is not re-implemented.
//!   * `pluralize` — base's abstract `raise NotImplementedError`. **Unreachable
//!     for LA**: `to_currency` open-codes its own two-form selection and
//!     `to_cheque` takes `cr1[-1]` directly, so neither ever calls it. Left at
//!     the trait default deliberately.
//!   * `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` on this
//!     class (verified on the live class object, not the source), so the
//!     `currency_precision` (-> 100) and `currency_adjective` (-> `None`) trait
//!     defaults are already correct and are not overridden.
//!
//! Number words (PR #657): `to_cardinal`/`to_ordinal`/`to_year` take
//! `gender=`/`case=`/`macrons=` kwargs; `to_ordinal_num` is a Roman numeral.
//! `to_currency` is the fork's own, hand-rolled over [`LangLa::int_to_word`].
//!
//! # `int_to_word` quirks (currency path only)
//!
//! [`LangLa::int_to_word`] is the fork's basic cardinal, retained solely for
//! `to_currency`. It is *not* Classical Latin, but it is what the frozen corpus
//! records for the currency surface, so it is kept verbatim. The bugs below
//! describe `int_to_word`; the PR #657 cardinals above do not share them.
//!
//! 3. **Values ≥ 10^9 are emitted as bare digits.** `_int_to_word`'s final
//!    `else` is `return str(number)`, so `to_cardinal(10**9) == "1000000000"`
//!    and `to_cardinal(10**21) == "1000000000000000000000"` — decimal
//!    numerals, not words. The negword still gets prefixed by `to_cardinal`,
//!    so `to_cardinal(-10**9) == "minus 1000000000"` (verified; no corpus row
//!    covers it, but it falls out of the same code path). This fallback is
//!    why Latin needs `BigInt` and can never be narrowed to a fixed-width int.
//! 4. **`ones[0]` is `""`, and there is no `zero` attribute.** Zero only works
//!    because of the falsy check `self.ones[0] if self.ones[0] else "zero"`,
//!    which always takes the else branch. Modelled literally in [`ZERO`].
//! 5. **Teens are formed by juxtaposition**: 11 → "decem unus", 19 → "decem
//!    novem" (Latin proper: "undecim", "undeviginti"). Likewise 12345 →
//!    "decem duo mille …".
//! 6. **`unus` is prefixed unconditionally** to hundreds and thousands: 100 →
//!    "unus centum" (not "centum"), 1000 → "unus mille" (not "mille"),
//!    1000000 → "unus decies centena milia".
//! 7. **`million = "decies centena milia"`** ("ten hundred thousands") is used
//!    as the multiplicand for *every* millions count, so 2·10^6 →
//!    "duo decies centena milia" — literally "two ten hundred thousands".
//! 8. **`_int_to_word`'s `number < 0` branch is dead code.** `to_cardinal`
//!    strips the sign before calling it, and `to_currency` passes `abs(val)`.
//!    Reproduced below to mirror the source structure; it is unreachable from
//!    every in-scope entry point.
//! 9. `pointword` is the English `"point"` — visible only on the float path
//!    ([`LangLa::to_cardinal_float`], now ported below). It is emitted raw,
//!    not `title`d: LA's `to_cardinal` writes `self.pointword` literally, and
//!    `is_title` is false anyway.
//! 10. **An unknown currency code silently becomes EUR** in `to_currency`:
//!     `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
//!     So `to_currency(1, "GBP")` is "unus euro" and `to_currency(1, "JPY")` is
//!     "unus euro" — nine distinct codes in the corpus all render as euros.
//!     `to_cheque` does **not** share this: it is base's, uses `[currency]`, and
//!     raises NotImplementedError for the very same codes. Both are in the
//!     corpus, so the leniency cannot be hoisted into `currency_forms`.
//! 11. **`CURRENCY_PRECISION` is `{}`, so every currency is 2-decimal.** LA's
//!     `to_currency` never consults it at all: KWD/BHD get no 1000 divisor and
//!     JPY no 1 divisor. Combined with bug 10, `currency:KWD 12.34` is
//!     "decem duo euros triginta quattuor centesimae" — a 3-decimal currency
//!     rendered as 2-decimal euros. `to_cheque` *does* read it, but only ever
//!     finds the 100 default.
//! 12. **Cents truncate rather than round**: `parts[1][:2]` slices the string,
//!     so `1.999` -> "unus euro nonaginta novem centesimae" (99, not 100 and a
//!     carry). Verified against the interpreter.
//! 13. **`adjective` is accepted and ignored.** LA declares the parameter and
//!     never reads it; `CURRENCY_ADJECTIVES` is `{}` anyway.
//! 14. **`cents=False` omits the subunit entirely** rather than rendering it as
//!     terse digits the way `Num2Word_Base.to_currency` does — the guard is
//!     `if cents and right`, so there is no `_cents_terse` call. LA therefore
//!     answers `to_currency(12.34, cents=False)` with "decem duo euros", where
//!     base would say "decem duo euros,34".
//!
//! # Error variants
//!
//! The four integer modes have **no** reachable raise or crash site: no card
//! table means no `OverflowError`, no dict lookups means no `KeyError`, every
//! list index is provably bounded (see [`digit`]), and no `int()` ever sees a
//! stray `"-"`. Every integer-mode corpus row is `ok: true`.
//!
//! (The PR #657 number words do raise: `OverflowError` for cardinals ≥ 10^18
//! and for ordinals past 10^6, #181; `TypeError` for negative ordinals.)
//!
//! The currency surface adds exactly two:
//!   * `NotImplemented` — `to_cheque` on any code outside {EUR, USD}, raised by
//!     `currency::default_to_cheque` off a `currency_forms` miss. Seven of the
//!     nine `cheque:*` corpus rows are this. `to_currency` never raises it
//!     (bug 10).
//!   * `Value` — `to_currency` on a value whose `str()` uses exponent notation
//!     (`int('1e+21')` -> ValueError). Not covered by the corpus; see
//!     `split_value` and "Known divergence" below.
//!
//! The remaining `ok: false` rows for `"la"` are `fraction` and `w2n_cardinal`,
//! both outside this port's scope.
//!
//! # Known divergence (one, and it is not fixable here)
//!
//! A float whose `repr` switches to *negative* exponent notation — i.e. a
//! non-zero `abs(v) < 1e-4`, such as `1e-05` — should raise ValueError
//! (`int('1e-05')`), but this port renders it "zero euros".
//!
//! It cannot be fixed inside this file, because the distinction is erased
//! before the value arrives. `__init__.py` passes `str(number)` plus a single
//! `is_int` flag; for non-ints it does **not** say whether the caller had a
//! `float` or a `decimal.Decimal`. Yet Python disagrees with itself on exactly
//! that basis for the same numeric value (verified):
//!
//! ```text
//! to_currency(1e-05)             -> ValueError   # str() is '1e-05'
//! to_currency(Decimal('0.00001')) -> 'zero euros' # str() is '0.00001'
//! ```
//!
//! Both reach this crate as the identical `BigDecimal { digits: 1, scale: 5 }`,
//! so no rule over `(digits, scale)` can separate them; "zero euros" is right
//! for one input and wrong for the other. Telling them apart needs the original
//! string (or `type(number)`) plumbed through the shim, which is off-limits and
//! shared by ~150 languages. The *positive*-exponent half of the same problem
//! (`1e+21`, `1.5e+20`) **is** reproduced exactly, because a negative
//! `BigDecimal` scale is unambiguous — a plain decimal string can never produce
//! one, and both `float` and `Decimal` stringify to exponent form there.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dollarium/dollaria and euro/eurones with
//! centesima/centesimae. Numerals 1-3 do not yet agree in gender with the noun
//! ("duo centesimae"). Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{Kwargs, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.negword`, verbatim — note the trailing space is part of the literal.
///
/// `to_cardinal` uses `ret = self.negword` **raw**, not the
/// `"%s " % self.negword.strip()` dance that `Num2Word_Base.parse_minus` and
/// `Num2Word_Base.to_cardinal` perform. Same result here only because the
/// literal already ends in exactly one space.
const NEGWORD: &str = "minus ";

/// The word `_int_to_word` reaches for zero. Python writes
/// `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is `""` (falsy), so
/// the else branch always wins. See bug 4.
const ZERO: &str = "zero";

/// `self.ones`. Index 0 is `""` and is never emitted — see [`ZERO`].
const ONES: [&str; 10] = [
    "", "unus", "duo", "tres", "quattuor", "quinque", "sex", "septem", "octo", "novem",
];

/// `self.tens`. Index 0 is `""`; unreachable because the `< 100` branch is
/// only entered when `number >= 10`, so `tens_val >= 1`.
const TENS: [&str; 10] = [
    "",
    "decem",
    "viginti",
    "triginta",
    "quadraginta",
    "quinquaginta",
    "sexaginta",
    "septuaginta",
    "octoginta",
    "nonaginta",
];

const HUNDRED: &str = "centum";
const THOUSAND: &str = "mille";
/// `self.million`. See bug 7 — this is "ten hundred thousands", used as the
/// multiplicand for every millions count.
const MILLION: &str = "decies centena milia";

/// The separator the pyo3 binding hands us when the Python caller omitted one.
///
/// `Num2Word_LA.to_currency` declares `separator=" "` in its own signature, but
/// the `Lang` trait takes the separator as a plain argument, and both
/// `num2words2/__init__.py`'s Rust fast path and `bench/diff_test.py` substitute
/// `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default, not LA's —
/// before the value ever reaches Rust. By the time we see it, "caller omitted
/// separator" and "caller asked for a comma" are the same string and the
/// information is gone.
///
/// Every currency row in the frozen corpus is generated by
/// `num2words(v, lang="la", to="currency", currency=c)` with no `separator=`, so
/// Python renders them with LA's " " ("decem duo euros triginta quattuor
/// centesimae"), while `diff_test.py` feeds this core a ",". Mapping "," back to
/// " " restores LA's default and reproduces the corpus. The one input it gets
/// wrong is an *explicit* `separator=","`, which Python renders as
/// "decem duo euros,triginta quattuor centesimae" (no space — LA concatenates
/// the separator raw, verified against the interpreter) and which we render as
/// " ". Fixing that properly means teaching the shim to pass the converter's own
/// default; that is out of scope here (`__init__.py` is off-limits and shared by
/// ~150 languages), so the narrower divergence is the deliberate choice.
/// `lang_bs.rs`, `lang_ca.rs` and `lang_es.rs` hit the identical trap and
/// resolve it the same way — 63 Python modules declare `separator=" "`.
const SEPARATOR_UNSET: &str = ",";

/// `Num2Word_LA.CURRENCY_FORMS`'s de-facto fallback key.
///
/// `to_currency` does
/// `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
/// `CURRENCY_FORMS` is a dict literal with EUR written first, and Python dicts
/// preserve insertion order, so `values()[0]` is **always** EUR's pair. That is
/// what every unrecognised code silently becomes — see bug 10.
const FALLBACK_CURRENCY: &str = "EUR";

/// Narrow a provably-small `BigInt` to a table index.
///
/// Every call site is guarded by a branch that bounds the value to `0..=9`
/// (`number < 10`, or a `// 10` / `// 100` of a value below 100 / 1000), so
/// the conversion cannot fail. Panicking here would mean the guard above it
/// was rewritten incorrectly.
fn digit(n: &BigInt) -> usize {
    n.to_usize()
        .filter(|d| *d < 10)
        .expect("branch guard bounds this value to 0..=9")
}

/// Python's `int(s)` on the integer part of a repr string, as reached by LA's
/// float `to_cardinal`. The float path feeds only the digits before the ".",
/// which `format!` guarantees are `[0-9]+`, so this never fails in practice;
/// the error arm mirrors the `ValueError` `int()` raises on a bad literal.
#[allow(dead_code)] // kept from the port; currently unreferenced (#246)
fn parse_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// PR savoirfairelinux/num2words#657 — declined Latin cardinals/ordinals.
//
// Data and algorithm are a faithful port of Piotr Balwierz's `lang_LA.py`
// (branch `add-latin`). It replaces the fork's basic `_int_to_word`-based
// number words (which stay live *only* for `to_currency`, unchanged). Cardinals
// decline across 3 genders × 6 cases for the declined forms (1, 2, 3, 200..900);
// tables carry Classical macrons, strippable with `macrons=False`.
//
// gender index: m=0, f=1, n=2 ; case index: nom=0 gen=1 dat=2 acc=3 abl=4 voc=5
// ─────────────────────────────────────────────────────────────────────────────

const CASES: [&str; 6] = ["nom", "gen", "dat", "acc", "abl", "voc"];

fn gender_idx(g: &str) -> Option<usize> {
    match g {
        "m" => Some(0),
        "f" => Some(1),
        "n" => Some(2),
        _ => None,
    }
}

fn case_idx(c: &str) -> Option<usize> {
    CASES.iter().position(|&x| x == c)
}

// _DECL_1 / _DECL_2 / _DECL_3 — [gender][case].
const DECL_1: [[&str; 6]; 3] = [
    ["ūnus", "ūnīus", "ūnī", "ūnum", "ūnō", "ūne"],
    ["ūna", "ūnīus", "ūnī", "ūnam", "ūnā", "ūna"],
    ["ūnum", "ūnīus", "ūnī", "ūnum", "ūnō", "ūnum"],
];
const DECL_2: [[&str; 6]; 3] = [
    ["duo", "duōrum", "duōbus", "duōs", "duōbus", "duo"],
    ["duae", "duārum", "duābus", "duās", "duābus", "duae"],
    ["duo", "duōrum", "duōbus", "duo", "duōbus", "duo"],
];
const DECL_3: [[&str; 6]; 3] = [
    ["trēs", "trium", "tribus", "trēs", "tribus", "trēs"],
    ["trēs", "trium", "tribus", "trēs", "tribus", "trēs"],
    ["tria", "trium", "tribus", "tria", "tribus", "tria"],
];

// _hundred_paradigm: stem + these endings, [gender][case].
const HUND_ENDING: [[&str; 6]; 3] = [
    ["ī", "ōrum", "īs", "ōs", "īs", "ī"],
    ["ae", "ārum", "īs", "ās", "īs", "ae"],
    ["a", "ōrum", "īs", "a", "īs", "a"],
];
// _HUNDREDS stems, index by the hundreds digit 2..=9 (1 → "centum" is special).
const HUND_STEM: [&str; 10] = [
    "", "", "ducent", "trecent", "quadringent", "quīngent", "sescent",
    "septingent", "octingent", "nōngent",
];

// _UNDER_20_INVARIANT, index 4..=19 (0..=3 handled elsewhere).
const UNDER_20: [&str; 20] = [
    "", "", "", "", "quattuor", "quīnque", "sex", "septem", "octō", "novem",
    "decem", "ūndecim", "duodecim", "tredecim", "quattuordecim", "quīndecim",
    "sēdecim", "septendecim", "duodēvīgintī", "ūndēvīgintī",
];

// _TENS_INVARIANT, index 2..=9.
const TENS_INV: [&str; 10] = [
    "", "", "vīgintī", "trīgintā", "quadrāgintā", "quīnquāgintā", "sexāgintā",
    "septuāgintā", "octōgintā", "nōnāgintā",
];

// _MILIA_DECL (neut. pl.), index by case.
const MILIA: [&str; 6] = ["mīlia", "mīlium", "mīlibus", "mīlia", "mīlibus", "mīlia"];

// _zero: "nullus/nulla/nullum" declension (no macrons), [gender][case].
const ZERO_DECL: [[&str; 6]; 3] = [
    ["nullus", "nullius", "nulli", "nullum", "nullo", "nulle"],
    ["nulla", "nullius", "nulli", "nullam", "nulla", "nulla"],
    ["nullum", "nullius", "nulli", "nullum", "nullo", "nullum"],
];

// _decline_us_um_adj endings, [gender][case].
const ADJ_ENDING: [[&str; 6]; 3] = [
    ["us", "ī", "ō", "um", "ō", "e"],
    ["a", "ae", "ae", "am", "ā", "a"],
    ["um", "ī", "ō", "um", "ō", "um"],
];

// _HIGH_UNITS in descending power. Only the nominative (sg/pl) is ever used;
// every unit is masculine, and its modifier is always masc.nom.
const HIGH_UNITS: [(u64, &str, &str); 4] = [
    (1_000_000_000_000_000, "billiardus", "billiardi"),
    (1_000_000_000_000, "billio", "billiones"),
    (1_000_000_000, "miliardus", "miliardi"),
    (1_000_000, "milio", "miliones"),
];

const LA_MAXVAL_WORDS_EXP: u32 = 18; // 10^18

/// Strip Classical macrons to plain ASCII when `macrons` is false.
fn apply_macrons(s: &str, macrons: bool) -> String {
    if macrons {
        return s.to_string();
    }
    s.chars()
        .map(|c| match c {
            'ā' => 'a',
            'ē' => 'e',
            'ī' => 'i',
            'ō' => 'o',
            'ū' => 'u',
            'Ā' => 'A',
            'Ē' => 'E',
            'Ī' => 'I',
            'Ō' => 'O',
            'Ū' => 'U',
            'ȳ' => 'y',
            'Ȳ' => 'Y',
            other => other,
        })
        .collect()
}

/// `_form(n, gender, case)` — declined cardinal for 1, 2, 3 or 200..=900.
fn la_form(n: u32, g: usize, c: usize) -> &'static str {
    match n {
        1 => DECL_1[g][c],
        2 => DECL_2[g][c],
        3 => DECL_3[g][c],
        _ => {
            // n is h*100 with 2 <= h <= 9. Paradigm = stem + ending; the two
            // string pieces are concatenated at call sites that need them.
            // Callers of la_form for hundreds use la_hundred instead.
            unreachable!("la_form only handles 1,2,3; hundreds via la_hundred")
        }
    }
}

/// A declined hundreds cardinal (200..=900): stem + gender/case ending.
fn la_hundred(h: usize, g: usize, c: usize) -> String {
    format!("{}{}", HUND_STEM[h], HUND_ENDING[g][c])
}

/// `_low(n, gender, case)` — cardinal 0..=19 (0 → empty).
fn la_low(n: u32, g: usize, c: usize) -> String {
    match n {
        0 => String::new(),
        1 | 2 | 3 => la_form(n, g, c).to_string(),
        _ => UNDER_20[n as usize].to_string(),
    }
}

/// `_under_1000(n, gender, case)` — 0..=999. Form-sensitive for 1,2,3,200..900.
fn la_under_1000(n: u32, g: usize, c: usize) -> String {
    let h = n / 100;
    let rest = n % 100;
    let t = rest / 10;
    let u = rest % 10;
    let mut parts: Vec<String> = Vec::new();
    if h != 0 {
        if h == 1 {
            parts.push("centum".to_string());
        } else {
            parts.push(la_hundred(h as usize, g, c));
        }
    }
    if rest == 0 {
        // nothing
    } else if rest <= 19 {
        parts.push(la_low(rest, g, c));
    } else {
        if t != 0 {
            parts.push(TENS_INV[t as usize].to_string());
        }
        if u != 0 {
            parts.push(la_low(u, g, c));
        }
    }
    parts.join(" ")
}

/// `_zero(gender, case)`.
fn la_zero(g: usize, c: usize) -> &'static str {
    ZERO_DECL[g][c]
}

/// `_to_cardinal_pos(n, gender, case)` — compose a non-negative < 10^18.
fn la_to_cardinal_pos(n: &BigInt, g: usize, c: usize) -> String {
    if n.is_zero() {
        return la_zero(g, c).to_string();
    }
    if n == &BigInt::from(1) {
        return la_form(1, g, c).to_string();
    }

    let mut parts: Vec<String> = Vec::new();
    let mut rest = n.clone();

    for (power, sg, pl) in HIGH_UNITS.iter() {
        let p = BigInt::from(*power);
        let (count, r) = rest.div_rem(&p);
        rest = r;
        if count.is_zero() {
            continue;
        }
        if count == BigInt::from(1) {
            // "ūnus" agrees with the singular masc. noun.
            parts.push(format!("{} {}", DECL_1[0][0], sg));
        } else {
            let cnt = count.to_u32().expect("count < 1000 by construction");
            parts.push(format!("{} {}", la_under_1000(cnt, 0, 0), pl));
        }
    }

    // Thousands: 1000 → "mīlle"; 2000+ → "<neut.pl. cardinal> mīlia[case]".
    let thousand = BigInt::from(1000);
    let (thousands, r) = rest.div_rem(&thousand);
    rest = r;
    if thousands == BigInt::from(1) {
        parts.push("mīlle".to_string());
    } else if thousands >= BigInt::from(2) {
        let th = thousands.to_u32().expect("thousands < 1000 by construction");
        parts.push(format!("{} {}", la_under_1000(th, 2, 0), MILIA[c]));
    }

    if rest > BigInt::zero() {
        let rr = rest.to_u32().expect("remainder < 1000");
        parts.push(la_under_1000(rr, g, c));
    }

    parts.join(" ")
}

/// `_ORDINALS_LOW` lookup (masc.nom.sg. citation stem).
fn la_ordinal_low(n: u32) -> Option<&'static str> {
    let s = match n {
        1 => "prīmus",
        2 => "secundus",
        3 => "tertius",
        4 => "quārtus",
        5 => "quīntus",
        6 => "sextus",
        7 => "septimus",
        8 => "octāvus",
        9 => "nōnus",
        10 => "decimus",
        11 => "ūndecimus",
        12 => "duodecimus",
        13 => "tertius decimus",
        14 => "quārtus decimus",
        15 => "quīntus decimus",
        16 => "sextus decimus",
        17 => "septimus decimus",
        18 => "duodēvīcēsimus",
        19 => "ūndēvīcēsimus",
        20 => "vīcēsimus",
        30 => "trīcēsimus",
        40 => "quadrāgēsimus",
        50 => "quīnquāgēsimus",
        60 => "sexāgēsimus",
        70 => "septuāgēsimus",
        80 => "octōgēsimus",
        90 => "nōnāgēsimus",
        100 => "centēsimus",
        200 => "ducentēsimus",
        300 => "trecentēsimus",
        400 => "quadringentēsimus",
        500 => "quīngentēsimus",
        600 => "sescentēsimus",
        700 => "septingentēsimus",
        800 => "octingentēsimus",
        900 => "nōngentēsimus",
        1000 => "mīllēsimus",
        1_000_000 => "deciēs centiēs mīllēsimus",
        _ => return None,
    };
    Some(s)
}

/// Numeral adverbs ("n times"), Allen & Greenough §138, in the module's
/// "-iēs" spelling (as in "deciēs centiēs mīllēsimus"). Index 1..=19, then
/// the tens and hundreds below.
const ADVERB_LOW: [&str; 20] = [
    "", "semel", "bis", "ter", "quater", "quīnquiēs", "sexiēs", "septiēs",
    "octiēs", "noviēs", "deciēs", "ūndeciēs", "duodeciēs", "terdeciēs",
    "quaterdeciēs", "quīndeciēs", "sēdeciēs", "septiēs deciēs", "duodēvīciēs",
    "ūndēvīciēs",
];
const ADVERB_TENS: [&str; 10] = [
    "", "", "vīciēs", "trīciēs", "quadrāgiēs", "quīnquāgiēs", "sexāgiēs",
    "septuāgiēs", "octōgiēs", "nōnāgiēs",
];
const ADVERB_HUNDREDS: [&str; 10] = [
    "", "centiēs", "ducentiēs", "trecentiēs", "quadringentiēs", "quīngentiēs",
    "sescentiēs", "septingentiēs", "octingentiēs", "nōngentiēs",
];

/// Numeral adverb for 1..=999, largest part first ("vīciēs semel", one of
/// the orders A&G §138 gives for 21).
fn la_adverb(n: u32) -> String {
    let (h, rest) = (n / 100, n % 100);
    let mut parts: Vec<&str> = Vec::new();
    if h != 0 {
        parts.push(ADVERB_HUNDREDS[h as usize]);
    }
    if rest != 0 && rest <= 19 {
        parts.push(ADVERB_LOW[rest as usize]);
    } else if rest != 0 {
        parts.push(ADVERB_TENS[(rest / 10) as usize]);
        if rest % 10 != 0 {
            parts.push(ADVERB_LOW[(rest % 10) as usize]);
        }
    }
    parts.join(" ")
}

/// Compound ordinal (masc.nom.sg.) for 1..=999: every part is an ordinal,
/// largest first — "vīcēsimus prīmus" (21), "centēsimus prīmus" (101),
/// as in A&G §133. The subtractive 18/19 come from the table.
fn la_ordinal_under_1000(n: u32) -> String {
    if let Some(stem) = la_ordinal_low(n) {
        return stem.to_string();
    }
    let (h, rest) = (n / 100 * 100, n % 100);
    let mut parts: Vec<&str> = Vec::new();
    if h != 0 {
        parts.push(la_ordinal_low(h).expect("hundreds are in the table"));
    }
    if rest != 0 {
        if let Some(stem) = la_ordinal_low(rest) {
            parts.push(stem);
        } else {
            parts.push(la_ordinal_low(rest / 10 * 10).expect("tens are in the table"));
            parts.push(la_ordinal_low(rest % 10).expect("units are in the table"));
        }
    }
    parts.join(" ")
}

/// Compound ordinal (masc.nom.sg.) for 1..1_000_000. Thousands take the
/// numeral adverb: "bis mīllēsimus" (2000), "deciēs mīllēsimus" (10000),
/// "centiēs mīllēsimus" (100000), then the ordinal of the remainder:
/// "mīllēsimus prīmus" (1001). `None` past the table's 10^6.
fn la_ordinal_compound(n: u32) -> Option<String> {
    if let Some(stem) = la_ordinal_low(n) {
        return Some(stem.to_string());
    }
    if n >= 1_000_000 {
        return None;
    }
    let (k, r) = (n / 1000, n % 1000);
    let mut parts: Vec<String> = Vec::new();
    if k == 1 {
        parts.push("mīllēsimus".to_string());
    } else if k > 1 {
        parts.push(format!("{} mīllēsimus", la_adverb(k)));
    }
    if r != 0 {
        parts.push(la_ordinal_under_1000(r));
    }
    Some(parts.join(" "))
}

/// `_decline_us_um_adj(stem, gender, case)` — 1st/2nd-decl. adjective from its
/// masc.nom.sg. citation form to any other slot. Compound stems (space) get
/// each word inflected.
fn la_decline_adj(stem: &str, g: usize, c: usize) -> String {
    if g == 0 && c == 0 {
        return stem.to_string();
    }
    if let Some((head, tail)) = stem.split_once(' ') {
        return format!("{} {}", la_decline_adj(head, g, c), la_decline_adj(tail, g, c));
    }
    let base = if stem.ends_with("us") || stem.ends_with("um") {
        &stem[..stem.len() - 2]
    } else {
        return stem.to_string();
    };
    format!("{}{}", base, ADJ_ENDING[g][c])
}

const ROMAN_TABLE: [(u32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
    (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
    (10, "X"), (9, "IX"), (5, "V"), (4, "IV"),
    (1, "I"),
];

/// `_to_roman(n)`. Falls back to plain digits for n <= 0 or n >= 4000.
fn la_to_roman(n: &BigInt) -> String {
    let mut m = match n.to_u32() {
        Some(v) if (1..4000).contains(&v) => v,
        _ => return n.to_string(),
    };
    let mut out = String::new();
    for (v, sym) in ROMAN_TABLE.iter() {
        while m >= *v {
            out.push_str(sym);
            m -= *v;
        }
    }
    out
}

fn la_normalize_grammar(gender: &str, case: &str) -> Result<(usize, usize)> {
    let g = gender_idx(gender).ok_or_else(|| {
        N2WError::Value(format!("gender must be 'm', 'f' or 'n'; got '{}'", gender))
    })?;
    let c = case_idx(case).ok_or_else(|| {
        N2WError::Value(format!(
            "case must be one of nom/gen/dat/acc/abl/voc; got '{}'",
            case
        ))
    })?;
    Ok((g, c))
}

fn la_maxval_words() -> BigInt {
    BigInt::from(10).pow(LA_MAXVAL_WORDS_EXP)
}

/// Read `gender`/`case`/`macrons` from a kwargs bag (PR #657 signature). An
/// unexpected key mirrors Python's TypeError for a bad keyword argument.
fn la_grammar_kw(kw: &Kwargs) -> Result<(&str, &str, bool)> {
    if !kw.only(&["gender", "case", "macrons"]) {
        return Err(N2WError::Type(
            "got an unexpected keyword argument".into(),
        ));
    }
    let g = kw.str("gender").unwrap_or("m");
    let c = kw.str("case").unwrap_or("nom");
    let m = kw.bool("macrons").unwrap_or(true);
    Ok((g, c, m))
}

/// `to_cardinal(value, gender, case, macrons)` for an integer value.
fn la_cardinal_int(value: &BigInt, gender: &str, case: &str, macrons: bool) -> Result<String> {
    let (g, c) = la_normalize_grammar(gender, case)?;
    if value.is_negative() {
        let inner = la_cardinal_int(&(-value), gender, case, true)?;
        return Ok(apply_macrons(
            &format!("{} {}", NEGWORD.trim(), inner),
            macrons,
        ));
    }
    let maxw = la_maxval_words();
    if value >= &maxw {
        return Err(N2WError::Overflow(format!(
            "abs({}) maius esse nōn dēbet quam {}.",
            value, maxw
        )));
    }
    Ok(apply_macrons(&la_to_cardinal_pos(value, g, c), macrons))
}

/// `to_ordinal(value, gender, case, macrons)` for an integer value.
fn la_ordinal_int(value: &BigInt, gender: &str, case: &str, macrons: bool) -> Result<String> {
    // verify_ordinal (integer path): a negative value raises TypeError.
    if value.is_negative() {
        return Err(N2WError::Type(format!(
            "Numerus negātīvus {} ōrdinālis fierī nōn potest.",
            value
        )));
    }
    // Latin has no ordinal zero; PR #657's cardinal fallback ("nullus") stays.
    if value.is_zero() {
        return la_cardinal_int(value, gender, case, macrons);
    }
    if let Some(stem) = value.to_u32().and_then(la_ordinal_compound) {
        let (g, c) = la_normalize_grammar(gender, case)?;
        return Ok(apply_macrons(&la_decline_adj(&stem, g, c), macrons));
    }
    // PR #657 fell back to the cardinal for every compound (#181). Below a
    // million the compound ordinal is built above; past it the classical
    // forms ("vīciēs centiēs mīllēsimus") are not generated, so say so
    // instead of returning the cardinal.
    let (_, _) = la_normalize_grammar(gender, case)?;
    Err(N2WError::Overflow(format!(
        "abs({}) must be at most 1000000 for to='ordinal' in la.",
        value
    )))
}

/// `to_ordinal_num(value)` — Roman numeral. verify_ordinal first.
fn la_ordinal_num_int(value: &BigInt) -> Result<String> {
    if value.is_negative() {
        return Err(N2WError::Type(format!(
            "Numerus negātīvus {} ōrdinālis fierī nōn potest.",
            value
        )));
    }
    Ok(la_to_roman(value))
}

/// `to_year(value, macrons)`. Negative years take an "ante Chrīstum" (BC) prefix.
fn la_year_int(value: &BigInt, macrons: bool) -> Result<String> {
    if value.is_negative() {
        let body = format!(
            "ante Chrīstum {}",
            la_cardinal_int(&(-value), "m", "nom", true)?
        );
        Ok(apply_macrons(&body, macrons))
    } else {
        la_cardinal_int(value, "m", "nom", macrons)
    }
}

pub struct LangLa {
    /// `Num2Word_LA.CURRENCY_FORMS`, built once here rather than per call.
    ///
    /// LA declares its own class-level dict, so it is **untouched** by
    /// `Num2Word_EN.__init__`'s in-place mutation of the `Num2Word_EUR` dict:
    /// exactly two codes, no AUD/JPY/KWD/… leakage, and EUR keeps LA's own
    /// `("euro", "euros")` rather than English's. Verified against the live
    /// interpreter, not the source literal.
    ///
    /// Both entries carry exactly **two** forms, matching Python's 2-tuples;
    /// `to_currency` only ever indexes `[0]` and `[1]`.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — EUR's pair, kept alongside the
    /// map purely so the `.get(currency, <default>)` fallback in `to_currency`
    /// is a borrow with no panic path. Python evaluates that default eagerly on
    /// every call; here it cannot fail.
    eur: CurrencyForms,
}

impl Default for LangLa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLa {
    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "EUR": (("euro", "euros"), ("centesima", "centesimae")),
        //     "USD": (("dollar", "dollars"), ("cent", "cents")),
        // }
        let eur = CurrencyForms::new(&["euro", "eurones"], &["centesima", "centesimae"]);
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (FALLBACK_CURRENCY, eur.clone()),
            (
                "USD",
                CurrencyForms::new(&["dollarium", "dollaria"], &["centesima", "centesimae"]),
            ),
        ]
        .into_iter()
        .collect();

        LangLa { currency_forms, eur }
    }

    /// Port of `to_currency`'s hand-rolled value decomposition.
    ///
    /// ```python
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// Returns `(left, right, is_negative)` with `right` in `0..=99`.
    ///
    /// This is *not* `parse_currency_parts` — LA never calls it, and never
    /// consults `CURRENCY_PRECISION` either (it is `{}` on this class anyway, so
    /// every code is implicitly 2-decimal: KWD/BHD get no 1000 divisor and JPY
    /// no 1 divisor — see bug 11).
    ///
    /// The `Int`/`Decimal` split does **no** work here beyond "an int's `str()`
    /// has no dot": LA skips the cents segment on `if cents and right`, not on
    /// `isinstance(val, int)`. That is why `1.0` prints "unus euro" with no
    /// cents even though it is a float — `str(1.0)` is "1.0", `parts[1]` is the
    /// truthy `"0"`, and `right` computes to a falsy 0. The two variants still
    /// must not be collapsed: `Int` must never grow a fraction.
    ///
    /// `parts[0]` is never empty for either variant (`str` always emits a
    /// leading "0"), so Python's `if parts[0] else 0` guard is unreachable.
    fn split_value(val: &CurrencyValue) -> Result<(BigInt, u32, bool)> {
        match val {
            // str(int) has no ".", so parts == [digits] and right stays 0.
            CurrencyValue::Int(v) => Ok((v.abs(), 0, v.is_negative())),
            CurrencyValue::Decimal { value: d, .. } => {
                let is_negative = d.is_negative();
                // `value == digits * 10^-scale`, with `digits`/`scale` taken
                // verbatim from the `str(value)` the shim parsed — so
                // reconstructing the plain-notation string from them reproduces
                // Python's `str(abs(val))` exactly. `abs` before `str` is just
                // the sign of `digits`. Note this deliberately does **not** use
                // `BigDecimal`'s `Display`, which re-normalises (it renders
                // `1e-05` as "0.00001", which `str(1e-05)` does not).
                let (digits, scale) = d.as_bigint_and_exponent();
                let digits = digits.abs();

                if scale < 0 {
                    // A negative scale can only come from exponent notation in
                    // the source string, and `str()` emits that for both floats
                    // (`str(1e21) == '1e+21'`) and Decimals
                    // (`str(Decimal('1E+2')) == '1E+2'`). Either way Python then
                    // does `int('1e+21')` and raises ValueError — this is a
                    // genuine reachable crash, not a deliberate raise. Verified:
                    // `to_currency(1e21)` -> ValueError. The message is
                    // best-effort (only the exception *type* is compared, and
                    // reproducing `repr(float)` is explicitly out of scope per
                    // `currency.rs`'s header).
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        d.abs()
                    )));
                }

                if scale == 0 {
                    // No "." in the string: the value is integral as written.
                    return Ok((digits, 0, is_negative));
                }

                let pow = BigInt::from(10u32).pow(scale as u32);
                let (left, frac) = digits.div_rem(&pow);
                // `parts[1]` is `frac` zero-padded to `scale` digits;
                // `int(parts[1][:2].ljust(2, "0"))` is then exactly "the first
                // two fractional digits as a 2-digit number", i.e. floor
                // division that keeps leading zeros and pads short fractions on
                // the right. 0.5 -> frac 5, scale 1 -> 5*100/10 = 50.
                // 1.999 -> frac 999, scale 3 -> 999*100/1000 = 99 (truncation,
                // not rounding — bug 12). 0.01 -> frac 1, scale 2 -> 1.
                let right = (frac * BigInt::from(100u32) / &pow).to_u32().unwrap_or(0);
                Ok((left, right, is_negative))
            }
        }
    }

    /// Python's `_int_to_word`.
    ///
    /// The cascade is a straight transcription. Python's `//` and `%` are
    /// floor division, but every operand that reaches them here is strictly
    /// positive (the `number < 0` arm intercepts the rest), so
    /// `div_mod_floor` and truncating division agree — the usual
    /// negative-floor-division trap does not bite.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            // `self.ones[0] if self.ones[0] else "zero"` — always the else.
            return ZERO.to_string();
        }

        if number.is_negative() {
            // Dead code in Python too. See bug 8.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        if *number < BigInt::from(10) {
            return ONES[digit(number)].to_string();
        }

        if *number < BigInt::from(100) {
            let (tens_val, ones_val) = number.div_mod_floor(&BigInt::from(10));
            if ones_val.is_zero() {
                return TENS[digit(&tens_val)].to_string();
            }
            return format!("{} {}", TENS[digit(&tens_val)], ONES[digit(&ones_val)]);
        }

        if *number < BigInt::from(1000) {
            let (hundreds_val, remainder) = number.div_mod_floor(&BigInt::from(100));
            // `self.ones[hundreds_val] + " " + self.hundred` — unconditional,
            // hence "unus centum" for 100. See bug 6.
            let mut result = format!("{} {}", ONES[digit(&hundreds_val)], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000) {
            let (thousands_val, remainder) = number.div_mod_floor(&BigInt::from(1000));
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000_000) {
            let (millions_val, remainder) = number.div_mod_floor(&BigInt::from(1_000_000));
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        // `return str(number)  # Fallback for very large numbers`. See bug 3.
        number.to_string()
    }
}

impl Lang for LangLa {

    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        // PR #657: `if int(value) != value` chooses the float grammar; a whole
        // value collapses to the integer cardinal (Decimal("2.0") -> "duo").
        if let Some(i) = value.as_whole_int() {
            return self.to_cardinal(&i);
        }
        // Non-integer: the base decimal grammar (pre "virgula" digit-by-digit),
        // driven by this language's macron-bearing `to_cardinal`.
        default_to_cardinal_float(self, value, precision_override)
    }

    /// PR #657 `to_cardinal(float, macrons=)`. Gender/case do not affect the
    /// float grammar (the pre/post parts are masc.nom); only `macrons` matters.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["gender", "case", "macrons"]) {
            return Err(N2WError::Type(
                "got an unexpected keyword argument".into(),
            ));
        }
        let macrons = kw.bool("macrons").unwrap_or(true);
        let s = self.cardinal_float_entry(value, precision_override)?;
        Ok(apply_macrons(&s, macrons))
    }

    /// PR #657 `to_ordinal(float)` — `verify_ordinal` rejects fractional
    /// (TypeError) and negative (TypeError) values; a whole value ordinalises.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) if !i.is_negative() => self.to_ordinal(&i),
            Some(_) => Err(N2WError::Type(
                "Numerus negātīvus ōrdinālis fierī nōn potest.".into(),
            )),
            None => Err(N2WError::Type(
                "Numerus frāctus ōrdinālis fierī nōn potest.".into(),
            )),
        }
    }

    /// PR #657 `to_ordinal_num(float)` — Roman numeral for a whole value;
    /// fractional / negative raise TypeError via `verify_ordinal`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            Some(i) if !i.is_negative() => Ok(la_to_roman(&i)),
            Some(_) => Err(N2WError::Type(
                "Numerus negātīvus ōrdinālis fierī nōn potest.".into(),
            )),
            None => Err(N2WError::Type(
                "Numerus frāctus ōrdinālis fierī nōn potest.".into(),
            )),
        }
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
        NEGWORD
    }

    /// PR #657 `pointword = "virgula"` (comma / decimal point).
    fn pointword(&self) -> &str {
        "virgula"
    }

    /// PR #657 `MAXVAL = 10^18`; drives the sentence `maxval("la")` and the
    /// overflow guard in `la_cardinal_int`.
    fn maxval(&self) -> &BigInt {
        static MV: std::sync::OnceLock<BigInt> = std::sync::OnceLock::new();
        MV.get_or_init(|| BigInt::from(10).pow(LA_MAXVAL_WORDS_EXP))
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)`: it peels a leading `"-"` off the
    /// *string*, then re-parses with `int()`. For integral input that is
    /// exactly "take the sign, then the absolute value", which is what this
    /// does — no digit round-trip needed. The `"." in n` branch is the float
    /// path and cannot trigger for a `BigInt`; the dispatcher additionally
    /// gates the Rust fast path on `type(number) is int`, so decimals never
    /// arrive here at all.
    /// PR #657 `to_cardinal` — masc.nom.sg. citation form by default.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        la_cardinal_int(value, "m", "nom", true)
    }

    /// PR #657 `to_cardinal(gender=, case=, macrons=)`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        let (g, c, m) = la_grammar_kw(kw)?;
        la_cardinal_int(value, g, c, m)
    }

    /// PR #657 `to_ordinal` — declined; compounds below 10^6 are built from
    /// ordinal parts (#181), larger values raise OverflowError.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        la_ordinal_int(value, "m", "nom", true)
    }

    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        let (g, c, m) = la_grammar_kw(kw)?;
        la_ordinal_int(value, g, c, m)
    }

    /// PR #657 `to_ordinal_num` — a Roman numeral.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        la_ordinal_num_int(value)
    }

    /// PR #657 `to_year(macrons=)` — negatives take an "ante Chrīstum" prefix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        la_year_int(value, true)
    }

    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["macrons"]) {
            return Err(N2WError::Type(
                "to_year() got an unexpected keyword argument".into(),
            ));
        }
        la_year_int(value, kw.bool("macrons").unwrap_or(true))
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for the NotImplementedError message that
    /// `to_cheque` raises on an unknown code.
    fn lang_name(&self) -> &str {
        "Num2Word_LA"
    }

    /// `self.CURRENCY_FORMS[code]` — a **strict** lookup, `None` for a miss.
    ///
    /// This is the hook the inherited `Num2Word_Base.to_cheque` reaches, and it
    /// wants the strict semantics: base does `self.CURRENCY_FORMS[currency]`
    /// inside a `try`/`except KeyError` and re-raises NotImplementedError. LA's
    /// *own* `to_currency` is the lenient one (`.get(code, <EUR>)`, see bug 10)
    /// and does its fallback inline rather than through this hook — which is
    /// why `currency:GBP` yields "euros" while `cheque:GBP` raises. Both
    /// behaviours are in the corpus; they are not reconcilable into one hook.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python's `Num2Word_LA.to_currency`. Overrides `Num2Word_Base`'s wholesale
    /// — no `parse_currency_parts`, no `pluralize`, no `CURRENCY_PRECISION`, no
    /// `CURRENCY_ADJECTIVES`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
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
        // `adjective` is accepted and then never read — LA's signature declares
        // it purely for interface compatibility with `Num2Word_Base`, and
        // `CURRENCY_ADJECTIVES` is `{}` on this class regardless. Verified:
        // `to_currency(12.34, adjective=True)` is byte-identical to the plain
        // call. See bug 13.
        let _ = adjective;

        let (left, right, is_negative) = Self::split_value(val)?;

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — bug 10.
        let forms = self.currency_forms.get(currency).unwrap_or(&self.eur);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::from(1u32);
        let left_str = self.int_to_word(&left);
        // `cr1[1] if left != 1 else cr1[0]`.
        let mut result = format!(
            "{} {}",
            left_str,
            if left == one { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — note this is *not* base's cents=False
        // ("render the subunit as terse digits"). For LA, `cents=False` drops
        // the subunit segment outright, so `_cents_terse`/`_cents_verbose` are
        // never reached and the `cents_terse` hook stays at its trait default.
        // Verified: `to_currency(12.34, cents=False)` -> "decem duo euros".
        // See bug 14. `right` is falsy at 0, which is what makes `1.0` render
        // without cents.
        if cents && right != 0 {
            let cents_str = self.int_to_word(&BigInt::from(right));
            let separator = if separator == SEPARATOR_UNSET {
                " "
            } else {
                separator
            };
            // `result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])`
            // — the separator is concatenated raw, with no space of its own.
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right == 1 { &cr2[0] } else { &cr2[1] });
        }

        if is_negative {
            // `self.negword + result`; NEGWORD already carries its trailing
            // space, so no extra separator is inserted.
            result.insert_str(0, NEGWORD);
        }

        // Python's trailing `.strip()`. Reachable only in principle: every
        // branch above produces a non-space-padded string. Kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[cfg(test)]
mod la_pr657_tests {
    use super::*;
    use crate::base::Kwargs;

    fn card(v: i64) -> String {
        LangLa::new().to_cardinal(&BigInt::from(v)).unwrap()
    }
    fn ordn(v: i64) -> String {
        LangLa::new().to_ordinal_num(&BigInt::from(v)).unwrap()
    }

    #[test]
    fn cardinals_decline_with_macrons() {
        assert_eq!(card(0), "nullus");
        assert_eq!(card(1), "ūnus");
        assert_eq!(card(50), "quīnquāgintā");
        assert_eq!(card(100), "centum");
        assert_eq!(card(200), "ducentī");
        assert_eq!(card(2024), "duo mīlia vīgintī quattuor");
        assert_eq!(
            LangLa::new().to_cardinal(&BigInt::from(38_630_666)).unwrap(),
            "trīgintā octō miliones sescenta trīgintā mīlia sescentī sexāgintā sex"
        );
    }

    #[test]
    fn gender_and_case_kwargs() {
        let mut kw = Kwargs::default();
        kw.0.push(("gender".into(), crate::base::KwVal::Str("f".into())));
        kw.0.push(("case".into(), crate::base::KwVal::Str("abl".into())));
        assert_eq!(LangLa::new().to_cardinal_kw(&BigInt::from(1), &kw).unwrap(), "ūnā");
    }

    #[test]
    fn macrons_false_strips_diacritics() {
        let mut kw = Kwargs::default();
        kw.0.push(("macrons".into(), crate::base::KwVal::Bool(false)));
        assert_eq!(LangLa::new().to_cardinal_kw(&BigInt::from(50), &kw).unwrap(), "quinquaginta");
    }

    #[test]
    fn ordinal_num_is_roman() {
        assert_eq!(ordn(4), "IV");
        assert_eq!(ordn(2024), "MMXXIV");
        assert_eq!(ordn(0), "0");
    }

    #[test]
    fn ordinals_decline() {
        assert_eq!(LangLa::new().to_ordinal(&BigInt::from(3)).unwrap(), "tertius");
        assert_eq!(LangLa::new().to_ordinal(&BigInt::from(20)).unwrap(), "vīcēsimus");
    }

    #[test]
    fn year_bc_prefix() {
        assert_eq!(LangLa::new().to_year(&BigInt::from(-44)).unwrap(), "ante Chrīstum quadrāgintā quattuor");
    }

    #[test]
    fn overflow_past_1e18() {
        let big = BigInt::from(10).pow(18);
        assert!(matches!(LangLa::new().to_cardinal(&big), Err(N2WError::Overflow(_))));
    }
}
