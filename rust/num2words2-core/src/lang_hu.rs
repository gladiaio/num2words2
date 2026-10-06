//! Port of `lang_HU.py` (Hungarian).
//!
//! Shape: **self-contained**. `Num2Word_HU` subclasses `lang_EUR.Num2Word_EUR`
//! (→ `Num2Word_Base`) and *does* define `mid_numwords`/`low_numwords`, so
//! Python really does build `self.cards` and set
//! `MAXVAL = 1000 * 10**603 == 10**606`. But `to_cardinal` is overridden
//! outright: it never calls `Num2Word_Base.to_cardinal`, so `splitnum`/
//! `clean`/`merge` are never reached and **the `MAXVAL` overflow check never
//! runs**. The cards table survives only as a lookup table for
//! `self.cards[value]` / `self.cards[exp]`. Hence `cards()`/`maxval()` are
//! exposed below to mirror the constructed Python state, but `merge` is left
//! at the trait default (unreachable). The port runs the `MAXVAL` check itself
//! — see bug 4.
//!
//! Inheritance chain actually exercised:
//!   * `Num2Word_EUR.setup` → `gen_high_numwords` + `high_numwords`
//!     (reimplemented locally as [`gen_high_numwords`] rather than importing
//!     `lang_en`'s copy, to keep this file self-contained per the porting
//!     contract).
//!   * `Num2Word_EUR.set_high_numwords` → long-scale illiárd/illió pairing,
//!     driven by `GIGA_SUFFIX = "illiárd"` / `MEGA_SUFFIX = "illió"`.
//!   * `Num2Word_Base.set_mid_numwords` / `set_low_numwords` → reused from
//!     `base.rs`.
//!   * `Num2Word_Base.verify_ordinal` → inlined in `to_ordinal_num`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is what the interpreter
//! actually emits, confirmed against the frozen corpus:
//!
//! 1. **The `partial_ords` suffix-stripping doubles the "m".** `partial_ords`
//!    maps the *bare stems* `"illió" -> "milliomod"` and
//!    `"illiárd" -> "milliárdod"`, but `to_ordinal` strips only the matched
//!    stem (5 resp. 7 chars) and splices in a replacement that re-adds its own
//!    leading `m`. The `m` of `millió` is left behind, so
//!    `to_ordinal(10**6)` == `"egym" + "milliomod" + "ik"` ==
//!    **"egymmilliomodik"** (note the `mm`), and `to_ordinal(10**9)` ==
//!    **"egymmilliárdodik"**. Corpus-confirmed.
//! 2. **Every mega/giga scale collapses onto *million* wording.** Because the
//!    keys are the bare stems, `billió`/`trillió`/… all match `"illió"` and
//!    get `"milliomod"`. So `to_ordinal(10**12)` (cardinal `"egybillió"`) ==
//!    **"egybmilliomodik"** — the `b` of `billió` survives and the ordinal
//!    claims *million*. Likewise `to_ordinal(10**18)` == "egytrmilliomodik",
//!    `to_ordinal(10**15)` == "egybmilliárdodik",
//!    `to_ordinal(10**21)` == "egytrmilliárdodik". All corpus-confirmed.
//! 3. **`to_ordinal` never calls `verify_ordinal`**, so negatives are happily
//!    ordinalised: `to_ordinal(-1)` == "mínusz első". Only `to_ordinal_num`
//!    verifies, and there a negative raises `TypeError`.
//! 4. **Fixed (gladiaio/num2words2#159): no overflow guard.** Python's
//!    `to_cardinal` bypasses `MAXVAL` entirely, so a value with ≥ 607 decimal
//!    digits computes `exp = 10**606`, one step past the highest card
//!    (`10**603` == "centilliárd"), and `self.cards[exp]` raised `KeyError`.
//!    The port checks `MAXVAL` (10^606) at the top of `to_cardinal_z` and
//!    raises `OverflowError`, so [`card`]'s `KeyError` arm is unreachable for
//!    whole numbers.
//! 5. **`két` only ever replaces a bare 2.** The `zero == "" and value == 2`
//!    special case tests the *whole* value, so 12 in a compound stays
//!    `"tizenkettő"`, giving `to_cardinal(12345)` ==
//!    "tizenkettőezer-háromszáznegyvenöt" (idiomatic Hungarian would be
//!    "tizenkétezer-…"). Corpus-confirmed.
//! 6. **`tens_to_cardinal` drops the `zero=""` context.** It calls
//!    `self.to_cardinal(value % 10)` with the *default* `zero="nulla"` rather
//!    than propagating `""`, so the `két` rule can never fire in the units
//!    slot of a 30–99 value: 42 → "negyvenkettő", never "negyvenkét".
//!    Harmless in practice (the remainder is non-zero there) but preserved.
//! 7. **A trailing "két" matches *nothing* in `partial_ords`, so the ordinal
//!    loop falls through.** `hundreds_to_cardinal`/`thousands_to_cardinal`
//!    *do* propagate `zero=""` into the remainder (unlike `tens_to_cardinal`,
//!    bug 6), so a value ending in exactly 2 renders as "…két":
//!    `to_cardinal(102)` == "százkét", `to_cardinal(2002)` == "kétezer-két".
//!    But `partial_ords` only keys `"kettő"` — never `"két"` — so no branch
//!    of the `for` loop matches, `out` is returned unmodified, and the bare
//!    `"ik"` is appended: **`to_ordinal(102)` == "százkétik"**,
//!    `to_ordinal(2002)` == "kétezer-kétik". This is the only input class
//!    where the loop completes without a `break`. The corpus does not cover
//!    it; verified directly against the interpreter and reproduced here.
//! 8. **A negative *int* amount rendered with a doubled space (fixed,
//!    #160).** `Num2Word_HU.to_currency` builds `minus_str` from the **raw**
//!    `self.negword` (`"mínusz "`) and feeds it to `"%s %s %s"`, so Python's
//!    `to_currency(-5, "HUF")` is "mínusz  öt forint". The port strips
//!    negword, as the float path does: "mínusz öt forint".
//! 9. **`adjective=True` is silently ignored for ints.** HU's int branch
//!    never consults `CURRENCY_ADJECTIVES`, so `to_currency(5, "USD",
//!    adjective=True)` == "öt dollars", while the float path *does* apply it:
//!    `to_currency(-5.0, "USD", adjective=True)` == "mínusz öt US dollars,
//!    nulla cents". Verified against the interpreter.
//! 10. **Whole floats ride the integer branches, but `big_number_to_cardinal`
//!    measures `len(str(value))` — and `str` of a float is not `str` of the
//!    int.** Below 1e16 the repr carries a trailing `".0"` (two extra chars),
//!    which the `digits % 3 == 0 → digits - 2` correction happens to absorb
//!    for every 10**(3k), so `to_cardinal(10**15 as float)` still reads
//!    "egybilliárd". At/above 1e16 the repr flips to exponent form
//!    (`"1e+16"`, 5 chars) → `exp = 10**3`, and the head recursion
//!    (`1e16 // 1000 == 1e13`) lands on a value whose repr grows back to
//!    fixed notation (`"10000000000000.0"`, 16 chars) → `exp = 10**15 >
//!    value`, where `rest = to_cardinal(value % exp, "")` re-enters with the
//!    *same* value forever: **`RecursionError`**. `1e16`/`1e20`
//!    corpus-confirmed; modelled by an explicit `value < exp` check in
//!    [`LangHu::pyfloat_whole_cardinal`] rather than a real 1000-deep spin.
//! 11. **`Decimal("1E+20")` → "százbilliárdezer".** Same short-repr trap
//!    (`len("1E+20") == 5` → `exp = 10**3`), but decimal *divide-integer*
//!    normalises the quotient to exponent 0, so the head is
//!    `Decimal("100000000000000000")` whose plain 18-char str yields
//!    `exp = 10**15` → "százbilliárd", then the top level appends its own
//!    `cards[10**3]`: "százbilliárd" + "ezer". The ordinal strips the
//!    trailing "ezer" → "százbilliárdezredik". Both corpus-confirmed. A
//!    whole Decimal whose str is *longer* than its numeric digits (trailing
//!    fractional zeros, `Decimal("1234567.000")`) keeps its exponent through
//!    `%` (ideal exponent `min(e, 0)`), so there the recursion *is* infinite
//!    → RecursionError, same guard.
//! 12. **`to_ordinal` welcomes floats** (bug 3 — no `verify_ordinal`), so
//!    `to_ordinal(-1.0)` == "mínusz első" and `to_ordinal(0.5)` runs the
//!    fraction grammar through the suffix loop: "nulla egész öt tized" ends
//!    with no `partial_ords` key → bare "ik" → "nulla egész öt tizedik".
//!    Only `to_ordinal_num` verifies: the float check fires first
//!    ("Cannot treat float 0.5 as ordinal."), then the negative check
//!    ("Cannot treat negative num -3.0 as ordinal."); `-0.0` passes both
//!    (`int(-0.0) == -0.0`, `abs(-0.0) == -0.0`) → "-0.0.".
//! 13. **`to_year` tests numeric `val < 0`,** so `-0.0` gets no "i. e. "
//!    prefix ("nulla") while `-1.5` does ("i. e. egy egész öt tized" — the
//!    cardinal of `abs(val)`, fraction grammar included).
//!
//! # Grammatical kwargs
//!
//! * `to_cardinal(value, zero=ZERO)` — the only converter kwarg. `zero=""`
//!   flips 2 to "két" and silences 0 (`to_cardinal(0, zero="") == ""`);
//!   any other string is emitted verbatim for 0. `zero=None` is only *read*
//!   when `value == 0`, where Python returns the raw `None`
//!   (`N2WError::ReturnsNone`); elsewhere it is dead (negatives recurse with
//!   the default, and `None == ""` is False).
//! * `to_year(val, suffix=None, longval=True)` — `suffix` is a *prefix*
//!   override for the BC marker and forces `abs(val)` even on positives;
//!   `longval` is accepted and never read, exactly like the Python body.
//!
//! # Currency state: a shared dict mutated by *another language*
//!
//! `Num2Word_HU` defines no `CURRENCY_FORMS` of its own and inherits
//! `Num2Word_EUR`'s. But `Num2Word_EN.__init__` — which also inherits rather
//! than shadows the attribute — does `self.CURRENCY_FORMS["EUR"] = …` for 27
//! codes. That mutates **`Num2Word_EUR.CURRENCY_FORMS` in place**, and
//! `num2words2/__init__.py` instantiates every converter at import time, so by
//! the time anyone calls `hu` the table HU sees is EUR's *as rewritten by EN*.
//! Confirmed at runtime: `Num2Word_HU().CURRENCY_FORMS is
//! Num2Word_EUR.CURRENCY_FORMS` → `True`, and the dict carries EN's KWD/BHD/
//! CNY/CHF/… additions that appear nowhere in `lang_EUR.py`.
//!
//! [`build_currency_forms`] therefore encodes the **post-import** table, not
//! what `lang_EUR.py` reads like on disk. Without that, `hu` would render EUR
//! as "euro" (EUR's literal) instead of the corpus's "euros" (EN's rewrite),
//! and would raise NotImplementedError for KWD/BHD/CNY/CHF entirely. The
//! snapshot is safe to freeze: the mutation happens only in `__init__`, and a
//! full sweep of every language × currency leaves the dict unchanged.
//!
//! `CURRENCY_ADJECTIVES` is shared the same way but nobody mutates it, so it
//! matches `lang_EUR.py` verbatim.
//!
//! `CURRENCY_PRECISION` is *not* shared: `Num2Word_EN.__init__` **rebinds**
//! it (`self.CURRENCY_PRECISION = {…}`) instead of mutating, which creates an
//! instance attribute on the EN object and leaves `Num2Word_Base`'s empty
//! `{}` intact for everyone else. So HU keeps divisor 100 for *every* code —
//! the 3-decimal currencies are **not** 3-decimal here (`to_currency(12.34,
//! "KWD")` == "tizenkettő dinars, harmincnégy fils", i.e. 34 fils out of 100,
//! and the cheque reads "56/100 DINARS" not "560/1000"), and the 0-decimal
//! ones still take a subunit (`to_currency(0.01, "JPY")` == "nulla yen, egy
//! sen"). `Num2Word_Base.to_currency`'s `divisor == 1` rounding branch is
//! therefore unreachable from HU. All corpus-confirmed. The trait's default
//! `currency_precision` already returns 100 unconditionally, so it is left
//! alone rather than overridden with an identical body.
//!
//! # Error variants
//!
//! * `to_ordinal_num` on a negative → `N2WError::Type` (Python
//!   `verify_ordinal` raises `TypeError`).
//! * `self.cards[...]` misses → `N2WError::Key` (Python `KeyError`). See
//!   bug 4.
//! * An unknown currency code → `N2WError::NotImplemented` (Python
//!   `NotImplementedError`), raised by `Num2Word_Base`. HU's int branch
//!   catches the `KeyError` from its own lookup and delegates to `super()`,
//!   which repeats the lookup and raises — so the message is identical on
//!   both paths.
//! * `pluralize` / the int branch indexing past a currency's form tuple →
//!   `N2WError::Index` (Python `IndexError`). Unreachable with the frozen
//!   table, whose every entry has ≥ 2 forms.

use crate::base::{
    check_maxval, set_low_numwords, set_mid_numwords, Cards, KwVal, Kwargs, Lang, N2WError, Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// `lang_HU.ZERO`, and the default value of `to_cardinal`'s `zero` kwarg.
const ZERO: &str = "nulla";

/// `self.negword`. Note the trailing space: HU's `to_cardinal`/`to_ordinal`
/// concatenate it **raw** (unlike `Num2Word_Base`, which does
/// `"%s " % self.negword.strip()`).
const NEGWORD: &str = "mínusz ";

/// `Num2Word_HU.GIGA_SUFFIX` — overrides EUR's "illiard".
const GIGA_SUFFIX: &str = "illiárd";
/// `Num2Word_HU.MEGA_SUFFIX` — overrides EUR's "illion".
const MEGA_SUFFIX: &str = "illió";

/// `self.mid_numwords`, in Python's declaration order.
const MID: [(i64, &str); 9] = [
    (1000, "ezer"),
    (100, "száz"),
    (90, "kilencven"),
    (80, "nyolcvan"),
    (70, "hetven"),
    (60, "hatvan"),
    (50, "ötven"),
    (40, "negyven"),
    (30, "harminc"),
];

/// The local `low_numwords` list in `Num2Word_HU.setup` (units 9 → 1).
const LOW_BASE: [&str; 9] = [
    "kilenc", "nyolc", "hét", "hat", "öt", "négy", "három", "kettő", "egy",
];

/// `self.partial_ords`, **in Python's insertion order** — load-bearing.
///
/// `to_ordinal` walks this in `dict` order and `break`s on the first suffix
/// match, so reordering changes output. In particular `"egy"` precedes
/// `"illió"`, which is why `to_ordinal(1000001)` ("egymillió-egy") matches on
/// the trailing "egy" and yields the *sane* "egymillió-egyedik", while
/// `to_ordinal(1000000)` ("egymillió") falls through to `"illió"` and yields
/// the broken "egymmilliomodik" (bug 1). Likewise `"száz"` precedes `"ezer"`,
/// so "ezerszáz" ordinalises on "száz" → "ezerszázadik".
const PARTIAL_ORDS: [(&str, &str); 23] = [
    ("nulla", "nullad"),
    ("egy", "egyed"),
    ("kettő", "ketted"),
    ("három", "harmad"),
    ("négy", "negyed"),
    ("öt", "ötöd"),
    ("hat", "hatod"),
    ("hét", "heted"),
    ("nyolc", "nyolcad"),
    ("kilenc", "kilenced"),
    ("tíz", "tized"),
    ("húsz", "huszad"),
    ("harminc", "harmincad"),
    ("negyven", "negyvened"),
    ("ötven", "ötvened"),
    ("hatvan", "hatvanad"),
    ("hetven", "hetvened"),
    ("nyolcvan", "nyolcvanad"),
    ("kilencven", "kilencvened"),
    ("száz", "század"),
    ("ezer", "ezred"),
    // Bare stems, not "millió"/"milliárd" — the source of bugs 1 and 2.
    ("illió", "milliomod"),
    ("illiárd", "milliárdod"),
];

fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

/// Python `RecursionError` — HU's `to_cardinal` re-enters itself with
/// unchanged arguments on some float/Decimal reprs (bugs 10/11) and the
/// interpreter kills it at the recursion limit. Returned eagerly instead of
/// actually spinning 1000 frames deep.
fn recursion_error() -> N2WError {
    N2WError::Custom {
        module: "builtins",
        class: "RecursionError",
        msg: "maximum recursion depth exceeded".into(),
    }
}

/// Python's `repr(float)` / `str(float)` for an f64.
///
/// Rust's `{}` shares the shortest-round-trip digit contract but *never*
/// switches to exponent notation, so it cannot stand in above 1e16 (or below
/// 1e-4). Rule: fixed notation for decimal exponent in `[-4, 16)` (whole
/// values get a trailing ".0"), otherwise `<shortest mantissa>e±NN` with the
/// exponent zero-padded to two digits. Non-finite values fall back to `{}`
/// — callers only ever test those strings for a missing '.', so the
/// inf/-inf/NaN vs Python inf/-inf/nan spelling difference is unobservable.
fn py_float_repr(f: f64) -> String {
    if !f.is_finite() {
        return format!("{}", f);
    }
    // `{:e}` = shortest digits + decimal exponent, e.g. "1e16", "1.2345e19".
    let sci = format!("{:e}", f);
    let (mant, exp) = sci.split_once('e').expect("{:e} always has an exponent");
    let exp: i32 = exp.parse().expect("{:e} exponent is an integer");
    if !(-4..16).contains(&exp) {
        return format!("{}e{}{:02}", mant, if exp < 0 { "-" } else { "+" }, exp.abs());
    }
    let s = format!("{}", f);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// CPython `float_floordiv`: `mod = fmod(x, y); div = (x - mod) / y;
/// floor(div)` with a half-ulp correction. Positive operands only here.
fn py_floordiv(x: f64, y: f64) -> f64 {
    let m = x % y; // Rust `%` is fmod for finite operands
    let div = (x - m) / y;
    let mut fd = div.floor();
    if div - fd > 0.5 {
        fd += 1.0;
    }
    fd
}

/// Python's numeric `value < 0` — NOT the sign bit. `-0.0 < 0` is False, so
/// [`FloatValue::is_negative`] (sign-bit aware, right for `str(value)`
/// rendering) would wrongly send `-0.0` down the negative branches of
/// `to_ordinal`/`to_year`.
fn fv_lt_zero(v: &FloatValue) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value < 0.0,
        // BigDecimal has no negative zero, matching Decimal('-0.0') < 0.
        FloatValue::Decimal { value, .. } => value.is_negative(),
    }
}

/// `-value`, keeping the precision (Python negation keeps the exponent).
fn fv_neg(v: &FloatValue) -> FloatValue {
    match v {
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

/// Rebuild the `Decimal` a `(numeric value, scale)` pair stands for, so
/// [`python_decimal_str`] renders Python's exact `str(value)`. `scale` is
/// the BigDecimal convention (`-as_tuple().exponent`): 3 for
/// `Decimal("12345.000")`, -20 for `Decimal("1E+20")`. Both directions are
/// exact by construction — a whole value with scale `-k` is divisible by
/// `10**k`.
fn dec_repr(v: &BigInt, scale: i64) -> BigDecimal {
    if scale >= 0 {
        BigDecimal::new(v * pow10(scale as u32), scale)
    } else {
        BigDecimal::new(v / pow10((-scale) as u32), scale)
    }
}

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Kept local rather than importing `lang_en`'s identical copy: the porting
/// contract is one self-contained file per language.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — `tens` is the outer loop.
    for t in tens {
        for u in units {
            out.push(format!("{}{}", u, t));
        }
    }
    out.reverse();

    const REPLACEMENTS: &[(&str, &str)] = &[
        ("novemn", "noven"),
        ("novemo", "novo"),
        ("octoo", "octo"),
        ("quintd", "quind"),
        ("quintn", "quin"),
        ("quintq", "quinq"),
        ("quints", "quins"),
        ("quintt", "quint"),
        ("quintv", "quinv"),
        ("septenn", "septen"),
        ("septent", "sept"),
        ("sexn", "sen"),
        ("sexs", "ses"),
        ("tresd", "tred"),
        ("tresn", "tren"),
        ("tress", "tres"),
        ("tresv", "trev"),
        ("unno", "uno"),
    ];
    for (k, v) in REPLACEMENTS {
        out = out.iter().map(|o| o.replace(k, v)).collect();
    }
    // `lows` is appended *after* the replacements — it is not rewritten.
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// Rebuilds `self.low_numwords` exactly as `Num2Word_HU.setup` does.
///
/// The two-statement construction has a subtlety worth spelling out: the
/// second statement's `low_numwords` refers to the **local** 9-element list,
/// *not* the 19-element `self.low_numwords` the first statement just built.
/// So the result is
/// `huszon* (9) + húsz (1) + [tizen* (9) + tíz (1) + units (9)] + nulla (1)`
/// = 30 entries, which `set_low_numwords` maps to 29 → 0.
fn build_low_numwords() -> Vec<String> {
    // self.low_numwords = ["tizen" + w for w in low_numwords] + ["tíz"] + low_numwords
    let mut teens: Vec<String> = LOW_BASE.iter().map(|w| format!("tizen{}", w)).collect();
    teens.push("tíz".to_string());
    teens.extend(LOW_BASE.iter().map(|s| s.to_string()));

    // self.low_numwords = (["huszon" + w for w in low_numwords] + ["húsz"]
    //                      + self.low_numwords + [ZERO])
    let mut out: Vec<String> = LOW_BASE.iter().map(|w| format!("huszon{}", w)).collect();
    out.push("húsz".to_string());
    out.extend(teens);
    out.push(ZERO.to_string());
    out
}

/// Builds `self.cards`, reproducing `Num2Word_Base.__init__`'s
/// `set_high_numwords` → `set_mid_numwords` → `set_low_numwords` sequence.
///
/// Resulting keys: every `10**(3k)` for k in 2..=201 (i.e. 10**6 … 10**603),
/// plus 1000, 100, 90…30, and 29…0.
fn build_cards() -> Cards {
    let mut cards = Cards::new();

    // --- Num2Word_EUR.setup ---
    let lows = ["non", "oct", "sept", "sext", "quint", "quadr", "tr", "b", "m"];
    let units = [
        "", "un", "duo", "tres", "quattuor", "quint", "sex", "septen", "octo", "novem",
    ];
    let tens = [
        "dec",
        "vigint",
        "trigint",
        "quadragint",
        "quinquagint",
        "sexagint",
        "septuagint",
        "octogint",
        "nonagint",
    ];
    let mut high: Vec<String> = vec!["cent".to_string()];
    high.extend(gen_high_numwords(&units, &tens, &lows));
    debug_assert_eq!(high.len(), 100);

    // --- Num2Word_EUR.set_high_numwords ---
    // cap = 3 + 6 * len(high) == 603; zip(high, range(cap, 3, -6)) pairs all
    // 100 words with n = 603, 597, …, 9. Both suffixes are non-empty for HU,
    // so both branches always fire.
    let cap: i64 = 3 + 6 * high.len() as i64;
    let mut n = cap;
    for word in high.iter() {
        if n <= 3 {
            break; // range(cap, 3, -6) exhausted
        }
        cards.insert(pow10(n as u32), format!("{}{}", word, GIGA_SUFFIX));
        cards.insert(pow10((n - 3) as u32), format!("{}{}", word, MEGA_SUFFIX));
        n -= 6;
    }

    // --- Num2Word_HU.setup ---
    set_mid_numwords(&mut cards, &MID);

    let low_owned = build_low_numwords();
    let low_refs: Vec<&str> = low_owned.iter().map(|s| s.as_str()).collect();
    set_low_numwords(&mut cards, &low_refs);

    cards
}

/// The `CURRENCY_FORMS` table HU actually sees at runtime.
///
/// This is `Num2Word_EUR.CURRENCY_FORMS` **after** `Num2Word_EN.__init__` has
/// rewritten it in place — see the module docs. Entries EN overwrote or added
/// are marked; everything else is EUR's own. Built once and stored on the
/// struct: constructing it per call is what made an earlier revision of this
/// port 10x slower than the Python it replaces.
///
/// Arity is load-bearing. `PLN` and `RON` carry a third form, which HU's int
/// branch and `Num2Word_EUR.pluralize` both ignore — they only ever index
/// slot 0 or slot 1. Dropping it would still be wrong, since `len(cr1) > 1`
/// and the `IndexError` surface are observable.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m = HashMap::new();
    let mut add = |code: &'static str, unit: &[&str], subunit: &[&str]| {
        m.insert(code, CurrencyForms::new(unit, subunit));
    };

    // --- added by Num2Word_EN.__init__ (absent from lang_EUR.py) ---
    add("AED", &["dirham", "dirhams"], &["fils", "fils"]);
    add("BHD", &["dinar", "dinars"], &["fils", "fils"]);
    add("BRL", &["real", "reais"], &["cent", "cents"]);
    add("CHF", &["franc", "francs"], &["rappen", "rappen"]);
    add("CNY", &["yuan", "yuan"], &["fen", "fen"]);
    add("HKD", &["dollar", "dollars"], &["cent", "cents"]);
    add("IQD", &["dinar", "dinars"], &["fils", "fils"]);
    add("JOD", &["dinar", "dinars"], &["fils", "fils"]);
    add("KWD", &["dinar", "dinars"], &["fils", "fils"]);
    add("LYD", &["dinar", "dinars"], &["dirham", "dirhams"]);
    add("NGN", &["naira", "naira"], &["kobo", "kobo"]);
    add("NZD", &["dollar", "dollars"], &["cent", "cents"]);
    add("OMR", &["rial", "rials"], &["baisa", "baisa"]);
    add("QAR", &["riyal", "riyals"], &["dirham", "dirhams"]);
    add("SGD", &["dollar", "dollars"], &["cent", "cents"]);
    add("TND", &["dinar", "dinars"], &["millime", "millimes"]);
    add("ZAR", &["rand", "rand"], &["cent", "cents"]);

    // --- overwritten by Num2Word_EN.__init__ (EUR's literal in the comment) ---
    add("AUD", &["dollar", "dollars"], &["cent", "cents"]); // EUR: GENERIC_DOLLARS
    add("CAD", &["dollar", "dollars"], &["cent", "cents"]); // EUR: GENERIC_DOLLARS
    add("EUR", &["euro", "euros"], &["cent", "cents"]); // EUR: ("euro", "euro")
    add("GBP", &["pound", "pounds"], &["penny", "pence"]); // EUR: ("pound sterling", …)
    add("INR", &["rupee", "rupees"], &["paisa", "paise"]); // EUR: identical
    add("JPY", &["yen", "yen"], &["sen", "sen"]); // EUR: identical
    add("KRW", &["won", "won"], &["jeon", "jeon"]); // EUR: identical
    add("MXN", &["peso", "pesos"], &["cent", "cents"]); // EUR: identical
    add("SAR", &["riyal", "riyals"], &["halalah", "halalas"]); // EUR: ("saudi riyal", …)
    add("USD", &["dollar", "dollars"], &["cent", "cents"]); // EUR: GENERIC_DOLLARS

    // --- untouched lang_EUR.py entries ---
    add("BYN", &["rouble", "roubles"], &["kopek", "kopeks"]);
    add("EEK", &["kroon", "kroons"], &["sent", "senti"]);
    add("HUF", &["forint", "forint"], &["fillér", "fillér"]);
    add("ISK", &["króna", "krónur"], &["aur", "aurar"]);
    add("LTL", &["litas", "litas"], &["cent", "cents"]);
    add("LVL", &["lat", "lats"], &["santim", "santims"]);
    add("NOK", &["krone", "kroner"], &["øre", "øre"]);
    add("PLN", &["zloty", "zlotys", "zlotu"], &["grosz", "groszy"]);
    add("RON", &["leu", "lei", "de lei"], &["ban", "bani", "de bani"]);
    add("RUB", &["rouble", "roubles"], &["kopek", "kopeks"]);
    add("SEK", &["krona", "kronor"], &["öre", "öre"]);
    add("UZS", &["sum", "sums"], &["tiyin", "tiyins"]);

    debug_assert_eq!(m.len(), 39);
    m
}

pub struct LangHu {
    cards: Cards,
    /// `1000 * list(self.cards.keys())[0]` == 10**606. Constructed by
    /// `Num2Word_Base.__init__` but **never read** by HU — `to_cardinal` is
    /// overridden and skips the overflow check. Kept for state parity.
    maxval: BigInt,
    /// `self.CURRENCY_FORMS`, frozen at its post-import value. See
    /// [`build_currency_forms`].
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangHu {
    fn default() -> Self {
        Self::new()
    }
}

impl LangHu {
    pub fn new() -> Self {
        let cards = build_cards();
        let maxval = cards
            .highest()
            .map(|h| h * BigInt::from(1000))
            .unwrap_or_else(BigInt::zero);
        LangHu {
            cards,
            maxval,
            currency_forms: build_currency_forms(),
        }
    }

    /// `self.cards[key]`, raising `KeyError` on a miss exactly as Python does.
    fn card(&self, key: &BigInt) -> Result<String> {
        self.cards
            .get(key)
            .map(|s| s.to_string())
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `Num2Word_HU.to_cardinal(value, zero=ZERO)`.
    ///
    /// The `zero` kwarg is threaded through the whole recursion: it is both
    /// the word emitted for 0 (`""` in compound position, so the segment
    /// vanishes) and the flag that switches 2 from "kettő" to "két".
    fn to_cardinal_z(&self, value: &BigInt, zero: &str) -> Result<String> {
        // The MAXVAL check Python never runs (bug 4, #159).
        check_maxval(value, &self.maxval)?;
        // `if int(value) != value` is always false for integral input.
        if value.is_negative() {
            // NB: recurses with the *default* zero, not the current one.
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.to_cardinal_z(&(-value), ZERO)?
            ));
        }
        if value.is_zero() {
            return Ok(zero.to_string());
        }
        if zero.is_empty() && *value == BigInt::from(2) {
            return Ok("két".to_string());
        }
        if *value < BigInt::from(30) {
            return self.card(value);
        }
        if *value < BigInt::from(100) {
            return self.tens_to_cardinal(value);
        }
        if *value < BigInt::from(1000) {
            return self.hundreds_to_cardinal(value);
        }
        if *value < pow10(6) {
            return self.thousands_to_cardinal(value);
        }
        self.big_number_to_cardinal(value)
    }

    /// `tens_to_cardinal` — 30 ≤ value < 100.
    fn tens_to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python: try: return self.cards[value] / except KeyError: ...
        if let Some(w) = self.cards.get(value) {
            return Ok(w.to_string());
        }
        let ten = BigInt::from(10);
        // value // 10 * 10 — value is positive here, so truncating division
        // and Python's floor division agree.
        let base = (value / &ten) * &ten;
        let head = self.card(&base)?;
        // Bug 6: the `zero=""` context is *not* propagated here.
        let rest = self.to_cardinal_z(&(value % &ten), ZERO)?;
        Ok(format!("{}{}", head, rest))
    }

    /// `hundreds_to_cardinal` — 100 ≤ value < 1000.
    fn hundreds_to_cardinal(&self, value: &BigInt) -> Result<String> {
        let hundred = BigInt::from(100);
        let hundreds = value / &hundred;
        let mut prefix = "száz".to_string();
        if !hundreds.is_one() {
            prefix = format!("{}{}", self.to_cardinal_z(&hundreds, "")?, prefix);
        }
        let postfix = self.to_cardinal_z(&(value % &hundred), "")?;
        Ok(format!("{}{}", prefix, postfix))
    }

    /// `thousands_to_cardinal` — 1000 ≤ value < 10**6.
    ///
    /// The hyphen rule is the odd one: Hungarian orthography hyphenates above
    /// 2000, and the source encodes that as `value <= 2000 or not postfix`.
    /// So 1234 → "ezerkétszázharmincnégy" (no hyphen) but
    /// 2001 → "kétezer-egy", and 3000 → "háromezer" (postfix empty).
    fn thousands_to_cardinal(&self, value: &BigInt) -> Result<String> {
        let k = BigInt::from(1000);
        let thousands = value / &k;
        let mut prefix = "ezer".to_string();
        if !thousands.is_one() {
            prefix = format!("{}{}", self.to_cardinal_z(&thousands, "")?, prefix);
        }
        let postfix = self.to_cardinal_z(&(value % &k), "")?;
        let sep = if *value <= BigInt::from(2000) || postfix.is_empty() {
            ""
        } else {
            "-"
        };
        Ok(format!("{}{}{}", prefix, sep, postfix))
    }

    /// `big_number_to_cardinal` — value ≥ 10**6.
    fn big_number_to_cardinal(&self, value: &BigInt) -> Result<String> {
        // digits = len(str(value)); value is positive here, so no sign char.
        let d = value.to_string().len();
        let d = if d % 3 != 0 { d } else { d - 2 };
        let exp = pow10((d / 3 * 3) as u32);

        // Evaluation order matters for error parity: Python binds `rest`
        // first, then evaluates to_cardinal(value // exp), and only then
        // looks up self.cards[exp] — so a missing card surfaces as KeyError
        // after both recursions have run (bug 4).
        let rest = self.to_cardinal_z(&(value % &exp), "")?;
        let head = self.to_cardinal_z(&(value / &exp), "")?;
        let scale = self.card(&exp)?;

        let tail = if rest.is_empty() {
            String::new()
        } else {
            format!("-{}", rest)
        };
        Ok(format!("{}{}{}", head, scale, tail))
    }

    /// The `partial_ords` suffix loop + `"ik"` tail shared by `to_ordinal`
    /// and [`Lang::ordinal_float_entry`].
    ///
    /// Suffix matching mirrors Python's `out[-len(card_word):] == card_word`,
    /// which slices by **character**. `str::ends_with` compares bytes, but the
    /// two agree for valid UTF-8: no character's encoding is a suffix of
    /// another's, so a byte-suffix match always lands on a char boundary —
    /// which also makes the `out[..out.len() - card_word.len()]` slice safe.
    /// (Python's short-string case, where `len(out) < len(card_word)` makes
    /// the slice return all of `out` and the comparison fail on length, is
    /// likewise handled: `ends_with` is simply false.)
    fn ordinalize(&self, mut out: String) -> String {
        for (card_word, ord_word) in PARTIAL_ORDS.iter() {
            if out.ends_with(card_word) {
                out = format!("{}{}", &out[..out.len() - card_word.len()], ord_word);
                break; // first match wins — order is load-bearing
            }
        }
        // No match falls through with `out` unmodified, exactly as Python's
        // for/else-less loop does — see bug 7 ("százkét" -> "százkétik").
        format!("{}ik", out)
    }

    /// `Num2Word_HU.to_cardinal(value, zero=...)` for a float/Decimal
    /// argument — the shared body of [`Lang::cardinal_float_entry`] and
    /// [`Lang::to_cardinal_float_kw`].
    ///
    /// Python's first branch is `if int(value) != value: return
    /// self.to_cardinal_float(value)` — note `zero` is dead on that path.
    /// Whole values then ride the integer branchwork, but with float/Decimal
    /// *arithmetic and reprs* (bugs 10/11), so they get their own faithful
    /// recursions instead of a cast to BigInt.
    fn hu_cardinal_any(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        zero: &str,
    ) -> Result<String> {
        if value.as_whole_int().is_none() {
            return self.to_cardinal_float(value, precision_override);
        }
        match value {
            FloatValue::Float { value: f, .. } => self.pyfloat_whole_cardinal(*f, zero),
            FloatValue::Decimal { value: d, .. } => {
                let (v, scale) = {
                    let scale = d.as_bigint_and_exponent().1;
                    // Whole, so scale-0 truncation is exact.
                    (d.with_scale(0).as_bigint_and_exponent().0, scale)
                };
                self.pydec_whole_cardinal(&v, scale, zero)
            }
        }
    }

    /// `Num2Word_HU.to_cardinal` for a **whole float**. Every branch matches
    /// [`LangHu::to_cardinal_z`] except that the arithmetic stays in f64
    /// (Python never converts) and `big_number_to_cardinal` measures
    /// `len(str(value))` on the *float* repr — the source of bug 10.
    ///
    /// All recursive arguments are whole floats again (`% 10`, `// 100`,
    /// `% exp` of whole operands), so `int(value) != value` never fires
    /// inside the recursion.
    fn pyfloat_whole_cardinal(&self, f: f64, zero: &str) -> Result<String> {
        if f < 0.0 {
            // NB: recurses with the *default* zero, and -0.0 is not < 0.
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.pyfloat_whole_cardinal(-f, ZERO)?
            ));
        }
        if f == 0.0 {
            return Ok(zero.to_string());
        }
        if zero.is_empty() && f == 2.0 {
            return Ok("két".to_string());
        }
        if f < 30.0 {
            // Python: self.cards[value] — hash(5.0) == hash(5) hits the int key.
            return self.card(&BigInt::from(f as i64));
        }
        if f < 100.0 {
            // tens_to_cardinal: try self.cards[value] / except KeyError.
            if let Some(w) = self.cards.get(&BigInt::from(f as i64)) {
                return Ok(w.to_string());
            }
            let base = BigInt::from((py_floordiv(f, 10.0) * 10.0) as i64);
            let head = self.card(&base)?;
            // Bug 6: the `zero=""` context is *not* propagated here.
            let rest = self.pyfloat_whole_cardinal(f % 10.0, ZERO)?;
            return Ok(format!("{}{}", head, rest));
        }
        if f < 1000.0 {
            // hundreds_to_cardinal.
            let hundreds = py_floordiv(f, 100.0);
            let mut prefix = "száz".to_string();
            if hundreds != 1.0 {
                prefix = format!("{}{}", self.pyfloat_whole_cardinal(hundreds, "")?, prefix);
            }
            let postfix = self.pyfloat_whole_cardinal(f % 100.0, "")?;
            return Ok(format!("{}{}", prefix, postfix));
        }
        if f < 1e6 {
            // thousands_to_cardinal.
            let thousands = py_floordiv(f, 1000.0);
            let mut prefix = "ezer".to_string();
            if thousands != 1.0 {
                prefix = format!("{}{}", self.pyfloat_whole_cardinal(thousands, "")?, prefix);
            }
            let postfix = self.pyfloat_whole_cardinal(f % 1000.0, "")?;
            let sep = if f <= 2000.0 || postfix.is_empty() { "" } else { "-" };
            return Ok(format!("{}{}{}", prefix, sep, postfix));
        }

        // big_number_to_cardinal, digits from the float repr (bug 10).
        let slen = py_float_repr(f).len(); // repr is ASCII: bytes == chars
        let d = if slen % 3 != 0 { slen } else { slen - 2 };
        let exp_pow = (d / 3 * 3) as u32;
        // Python coerces the int `exp` through float(); parsing the decimal
        // string is that exact conversion (correctly rounded), where
        // `10f64.powi` need not be.
        let exp_int = pow10(exp_pow);
        let exp_f: f64 = exp_int.to_string().parse().expect("10**n parses");
        if f < exp_f {
            // `rest = self.to_cardinal(value % exp, "")` == to_cardinal(value, "")
            // — identical arguments, infinite recursion (bug 10).
            return Err(recursion_error());
        }
        let m = f % exp_f;
        let rest = self.pyfloat_whole_cardinal(m, "")?;
        let head = self.pyfloat_whole_cardinal(py_floordiv(f, exp_f), "")?;
        let scale = self.card(&exp_int)?;
        let tail = if rest.is_empty() {
            String::new()
        } else {
            format!("-{}", rest)
        };
        Ok(format!("{}{}{}", head, scale, tail))
    }

    /// `Num2Word_HU.to_cardinal` for a **whole Decimal**, tracked as
    /// `(numeric value, scale)` so `str(value)` can be reproduced exactly
    /// where `big_number_to_cardinal` reads it (bug 11).
    ///
    /// Decimal arithmetic facts the recursion relies on, all from the
    /// General Decimal Arithmetic spec (verified on CPython):
    /// * `value // exp` (divide-integer) delivers exponent **0** — the
    ///   quotient's str is plain digits even when the input was `1E+20`.
    /// * `value % exp` keeps the ideal exponent `min(e_value, 0)`, i.e.
    ///   scale `max(scale, 0)` — trailing fractional zeros survive
    ///   (`Decimal("12345.000") % 1000 == Decimal("345.000")`).
    /// * Both ops raise `decimal.InvalidOperation`
    ///   (`[<class 'decimal.DivisionImpossible'>]`) when a result needs more
    ///   than the context's 28 digits. Modelled on the numeric digit counts
    ///   (the coefficient-widening of a scaled remainder is ignored) — only
    ///   reachable for ≥ 29-digit inputs the corpus never exercises.
    /// * Dict lookups (`self.cards[value]`) hit int keys because
    ///   `hash(Decimal("5.00")) == hash(5)` — modelled by looking up the
    ///   numeric value.
    fn pydec_whole_cardinal(&self, v: &BigInt, scale: i64, zero: &str) -> Result<String> {
        if v.is_negative() {
            // Negation keeps the exponent; recurses with the *default* zero.
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.pydec_whole_cardinal(&(-v), scale, ZERO)?
            ));
        }
        if v.is_zero() {
            return Ok(zero.to_string());
        }
        if zero.is_empty() && *v == BigInt::from(2) {
            return Ok("két".to_string());
        }
        if *v < BigInt::from(30) {
            return self.card(v);
        }
        if *v < BigInt::from(100) {
            if let Some(w) = self.cards.get(v) {
                return Ok(w.to_string());
            }
            let ten = BigInt::from(10);
            let base = (v / &ten) * &ten; // divide-integer → exponent 0
            let head = self.card(&base)?;
            let rest = self.pydec_whole_cardinal(&(v % &ten), scale.max(0), ZERO)?;
            return Ok(format!("{}{}", head, rest));
        }
        if *v < BigInt::from(1000) {
            let hundred = BigInt::from(100);
            let hundreds = v / &hundred;
            let mut prefix = "száz".to_string();
            if !hundreds.is_one() {
                prefix = format!("{}{}", self.pydec_whole_cardinal(&hundreds, 0, "")?, prefix);
            }
            let postfix = self.pydec_whole_cardinal(&(v % &hundred), scale.max(0), "")?;
            return Ok(format!("{}{}", prefix, postfix));
        }
        if *v < pow10(6) {
            let k = BigInt::from(1000);
            let thousands = v / &k;
            let mut prefix = "ezer".to_string();
            if !thousands.is_one() {
                prefix = format!("{}{}", self.pydec_whole_cardinal(&thousands, 0, "")?, prefix);
            }
            let postfix = self.pydec_whole_cardinal(&(v % &k), scale.max(0), "")?;
            let sep = if *v <= BigInt::from(2000) || postfix.is_empty() {
                ""
            } else {
                "-"
            };
            return Ok(format!("{}{}{}", prefix, sep, postfix));
        }

        // big_number_to_cardinal, digits from str(Decimal) (bug 11).
        let slen = python_decimal_str(&dec_repr(v, scale)).len(); // ASCII
        let d = if slen % 3 != 0 { slen } else { slen - 2 };
        let exp = pow10((d / 3 * 3) as u32);
        let q = v / &exp;
        let rem = v % &exp;
        // Python evaluates `value % exp` first; it raises DivisionImpossible
        // when the integer quotient or the (exact) remainder exceeds the
        // context's 28-digit precision.
        if q.to_string().len() > 28 || rem.to_string().len() > 28 {
            return Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.DivisionImpossible'>]".into(),
            });
        }
        if q.is_zero() {
            // str longer than the numeric digits (trailing fractional
            // zeros) made exp overshoot: `value % exp` is `value` at the
            // same scale — identical arguments, infinite recursion (bug 11).
            return Err(recursion_error());
        }
        let rest = self.pydec_whole_cardinal(&rem, scale.max(0), "")?;
        let head = self.pydec_whole_cardinal(&q, 0, "")?;
        let scale_word = self.card(&exp)?;
        let tail = if rest.is_empty() {
            String::new()
        } else {
            format!("-{}", rest)
        };
        Ok(format!("{}{}{}", head, scale_word, tail))
    }
}

impl Lang for LangHu {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "HUF"
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

    /// Present for state parity only — HU's `to_cardinal` never consults it.
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "egész"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal_z(value, ZERO)
    }

    /// `Num2Word_HU.to_ordinal`. The suffix loop lives in
    /// [`LangHu::ordinalize`], shared with the float entry.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // Bug 3: no verify_ordinal call, so negatives pass straight through.
        if value.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.to_ordinal(&(-value))?));
        }
        if value.is_one() {
            return Ok("első".to_string());
        }
        if *value == BigInt::from(2) {
            return Ok("második".to_string());
        }
        let out = self.to_cardinal(value)?;
        Ok(self.ordinalize(out))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // Num2Word_Base.verify_ordinal: the float check cannot fire for
        // integral input; the negative check raises TypeError.
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(format!("{}.", value))
    }

    /// `Num2Word_HU.to_year(val, suffix=None, longval=True)`.
    ///
    /// The Python signature takes a `suffix` that acts as a *prefix* override
    /// ("suffix is prefix here", per the source comment). The trait exposes no
    /// such parameter, so this reproduces the `suffix=None` path only: a
    /// negative year gets the BC marker "i. e. " ("időszámításunk előtt") and
    /// its absolute value; everything else is a plain cardinal.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut prefix = "";
        let mut val = value.clone();
        if val.is_negative() {
            val = val.abs();
            prefix = "i. e. ";
        }
        Ok(format!("{}{}", prefix, self.to_cardinal(&val)?))
    }

    // ---- float/Decimal entries (bugs 10–13) -------------------------------

    /// `to_cardinal(float/Decimal)`: fractional → `to_cardinal_float`
    /// (HU's own override below), whole → the integer branchwork with
    /// float/Decimal arithmetic and reprs. See [`LangHu::hu_cardinal_any`].
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // A whole value past MAXVAL is an OverflowError here too (#159).
        if let Some(i) = value.as_whole_int() {
            check_maxval(&i, &self.maxval)?;
        }
        self.hu_cardinal_any(value, precision_override, ZERO)
    }

    /// `to_ordinal(float/Decimal)` — bug 12. No verify_ordinal (bug 3), so
    /// the whole int body runs, `value == 1`/`== 2` comparing numerically
    /// (`Decimal("1.00") == 1`) and the cardinal — fraction grammar included
    /// — feeding the suffix loop.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if fv_lt_zero(value) {
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.ordinal_float_entry(&fv_neg(value))?
            ));
        }
        if let Some(i) = value.as_whole_int() {
            if i.is_one() {
                return Ok("első".to_string());
            }
            if i == BigInt::from(2) {
                return Ok("második".to_string());
            }
        }
        let out = self.cardinal_float_entry(value, None)?;
        Ok(self.ordinalize(out))
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)` then
    /// `str(value) + "."`. The float check fires before the negative one,
    /// and both messages interpolate `str(value)` — which is exactly
    /// `repr_str`. `-0.0` passes both checks (bug 12).
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        if value.as_whole_int().is_none() {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            )));
        }
        // `not abs(value) == value` — numerically False for -0.0.
        if fv_lt_zero(value) {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                repr_str
            )));
        }
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)` — bug 13: numeric `val < 0` (no prefix for
    /// -0.0), then `to_cardinal(abs(val))`, fraction grammar included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        if fv_lt_zero(value) {
            return Ok(format!(
                "i. e. {}",
                self.cardinal_float_entry(&fv_neg(value), None)?
            ));
        }
        self.cardinal_float_entry(value, None)
    }

    // ---- grammatical kwargs ------------------------------------------------

    /// `to_cardinal(value, zero=ZERO)` — the `zero` kwarg (see module docs).
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["zero"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let zero = match kw.get("zero") {
            None => ZERO,
            Some(KwVal::Str(s)) => s.as_str(),
            // zero=None is only *read* at value == 0, where Python returns
            // the raw None; every other branch never touches it (negatives
            // recurse with the default, and None == "" is False).
            Some(KwVal::None) => {
                if value.is_zero() {
                    return Err(N2WError::ReturnsNone);
                }
                ZERO
            }
            // A non-string zero is only observable at value == 0, where
            // Python returns the object itself — fall back to Python.
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        self.to_cardinal_z(value, zero)
    }

    /// Same kwarg on the float/Decimal entry: `zero` is dead on the
    /// fractional path (`int(value) != value` returns before it is read)
    /// and threads into the whole-value branchwork otherwise.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["zero"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let zero = match kw.get("zero") {
            None => ZERO,
            Some(KwVal::Str(s)) => s.as_str(),
            Some(KwVal::None) => {
                if value.as_whole_int().map_or(false, |i| i.is_zero()) {
                    return Err(N2WError::ReturnsNone);
                }
                ZERO
            }
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        self.hu_cardinal_any(value, precision_override, zero)
    }

    /// `to_year(val, suffix=None, longval=True)`. `suffix` is a *prefix*
    /// override ("suffix is prefix here", per the source comment) and its
    /// mere presence forces `abs(val)` even on positives; `longval` is
    /// accepted and never read.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let suffix = match kw.get("suffix") {
            None | Some(KwVal::None) => None,
            Some(KwVal::Str(s)) => Some(s.as_str()),
            // `suffix + " "` on a non-str raises Python's own TypeError —
            // fall back for the exact message.
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        let mut val = value.clone();
        let mut prefix = String::new();
        if val.is_negative() || suffix.is_some() {
            val = val.abs();
            prefix = match suffix {
                Some(s) => format!("{} ", s),
                None => "i. e. ".to_string(),
            };
        }
        Ok(format!("{}{}", prefix, self.to_cardinal(&val)?))
    }

    // ---- currency -------------------------------------------------------

    /// `self.__class__.__name__`, for the NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_HU"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unmodified.
    ///
    /// Reachable only from the float path: HU's int branch never looks at it
    /// (bug 9).
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        Some(match code {
            "AUD" => "Australian",
            "BYN" => "Belarusian",
            "CAD" => "Canadian",
            "EEK" => "Estonian",
            "HUF" => "Hungarian",
            "INR" => "Indian",
            "ISK" => "íslenskar",
            "JPY" => "Japanese",
            "KRW" => "Korean",
            "MXN" => "Mexican",
            "NOK" => "Norwegian",
            "RON" => "Romanian",
            "RUB" => "Russian",
            "SAR" => "Saudi",
            "USD" => "US",
            "UZS" => "Uzbekistan",
            _ => return None,
        })
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Note it indexes unconditionally — a single-form tuple and `n != 1`
    /// raises `IndexError`, which the frozen table never triggers.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_HU.to_currency(val, currency="HUF", cents=True,
    /// separator=",", adjective=False)`.
    ///
    /// HU intercepts **only** `isinstance(val, int)` and hands everything else
    /// to `super()` — i.e. `Num2Word_Base.to_currency`, which is what
    /// [`crate::currency::default_to_currency`] ports. The int/float split is
    /// the whole point of the override and is not cosmetic: `1` takes the
    /// bespoke branch and drops the cents segment, while `1.0` goes to the
    /// base and still renders "egy euro, nulla cents".
    ///
    /// The bespoke branch differs from the base's own int branch in three
    /// observable ways, all preserved here: the doubled space on negatives
    /// (bug 8), the ignored `adjective` (bug 9), and the trailing `.strip()`.
    /// Its inlined pluralisation happens to agree with
    /// `Num2Word_EUR.pluralize` for every entry in the table.
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
        let v = match val {
            CurrencyValue::Int(v) => v,
            // Not an int → `return super().to_currency(...)` verbatim.
            CurrencyValue::Decimal { value: _, .. } => {
                return crate::currency::default_to_currency(
                    self, val, currency, cents, separator, adjective,
                )
            }
        };

        // try: cr1, cr2 = self.CURRENCY_FORMS[currency]
        // except (KeyError, AttributeError): return super().to_currency(...)
        //
        // The fallback re-does the lookup in the base, misses again, and
        // raises NotImplementedError — so delegating reproduces both the
        // variant and the message. (AttributeError is dead: CURRENCY_FORMS is
        // always inherited.)
        let cr1 = match self.currency_forms.get(currency) {
            Some(forms) => &forms.unit,
            None => {
                return crate::currency::default_to_currency(
                    self, val, currency, cents, separator, adjective,
                )
            }
        };

        // minus_str = self.negword if val < 0 else ""
        // Python uses the raw negword here; stripped to avoid bug 8.
        let minus_str = if v.is_negative() { NEGWORD.trim() } else { "" };
        let abs_val = v.abs();
        let money_str = self.to_cardinal(&abs_val)?;

        // if abs_val == 1: cr1[0]
        // else:            cr1[1] if len(cr1) > 1 else cr1[0]
        let index_err = || N2WError::Index("tuple index out of range".into());
        let currency_str: &str = if abs_val.is_one() || cr1.len() <= 1 {
            cr1.first().map(String::as_str).ok_or_else(index_err)?
        } else {
            cr1[1].as_str()
        };

        // ("%s %s %s" % (minus_str, money_str, currency_str)).strip()
        Ok(format!("{} {} {}", minus_str, money_str, currency_str)
            .trim()
            .to_string())
    }

    /// `Num2Word_HU.to_cardinal_float(self, value)`.
    ///
    /// HU **overrides** the base float path. It does **not** touch
    /// `base.float2tuple`'s binary heuristic for the float arm — it splits
    /// `str(value)` on the dot and reads the fractional digits straight out of
    /// the shortest-round-trip repr:
    ///
    /// ```python
    /// def to_cardinal_float(self, value):
    ///     if abs(value) != value:
    ///         return self.negword + self.to_cardinal_float(-value)
    ///     left, right = str(value).split(".")
    ///     return (self.to_cardinal(int(left)) + " egész "
    ///             + self.to_cardinal(int(right)) + " "
    ///             + self.partial_ords[self.cards[10 ** len(right)]])
    /// ```
    ///
    /// Consequences preserved here:
    /// * The `2.675`/`1.005` f64 artefacts never surface: Python reads the
    ///   repr string (`"2.675"` → right `"675"`), not `abs(value-pre)*10**p`.
    ///   So the float arm renders [`py_float_repr`] and slices the string,
    ///   rather than calling `float2tuple` — including the exponent-form
    ///   corner (`1e-05` has no `.`), where the two-element unpack raises
    ///   `ValueError`. The Decimal arm *can* reuse [`float2tuple`], whose
    ///   Decimal branch is exact and equals `(int(left), int(right))`.
    /// * `self.cards[10 ** len(right)]` is a plain lookup ([`LangHu::card`]),
    ///   so a fractional length whose `10**n` is not a card (n == 4, 5, or
    ///   ≥ 7) raises `KeyError` — `N2WError::Key`. The corpus only exercises
    ///   n ∈ {1,2,3} (`tíz`/`száz`/`ezer` → `tized`/`század`/`ezred`).
    /// * `self.partial_ords[word]` is likewise a direct subscript: a card word
    ///   absent from `partial_ords` (e.g. `cards[10**6] == "millió"`, whose
    ///   key is the bare stem `"illió"`) raises `KeyError` too. Reproduced.
    /// * `precision_override` is ignored — Python's `to_cardinal_float(self,
    ///   value)` takes no precision argument, and nothing in the recursion
    ///   consults `self.precision`.
    /// * The literal joiner is `" egész "` (spaces baked in), which matches
    ///   `self.pointword` but is not derived from it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // if abs(value) != value: return self.negword + self.to_cardinal_float(-value)
        // `abs(value) != value` is the numeric test: False for -0.0 (whole,
        // so unreachable here anyway) — exactly [`fv_lt_zero`].
        if fv_lt_zero(value) {
            // NEGWORD keeps its trailing space, exactly as `self.negword`.
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.to_cardinal_float(&fv_neg(value), None)?
            ));
        }

        // left, right = str(value).split(".")
        let (left_int, right_int, right_len): (BigInt, BigInt, usize) = match value {
            FloatValue::Float { value: f, .. } => {
                // Python's repr, exponent form included — slicing it
                // reproduces str(value).split(".") and crucially bypasses
                // the float2tuple artefact heuristic HU never runs.
                let s = py_float_repr(*f);
                let (left, right) = s.split_once('.').ok_or_else(|| {
                    // Python: `left, right = ...split(".")` unpack fails.
                    N2WError::Value("not enough values to unpack (expected 2, got 1)".into())
                })?;
                let left_int = left.parse::<BigInt>().map_err(|_| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        left
                    ))
                })?;
                // int(right): leading zeros are fine (int("005") == 5).
                let right_int = right.parse::<BigInt>().map_err(|_| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        right
                    ))
                })?;
                (left_int, right_int, right.chars().count())
            }
            // Decimal is exact: float2tuple's Decimal branch returns
            // (int(value), abs(value-pre)*10**precision) == (int(left),
            // int(right)), and len(right) == precision for a negative-exponent
            // Decimal.
            FloatValue::Decimal { precision, .. } => {
                let (pre, post) = float2tuple(value);
                (pre, post, *precision as usize)
            }
        };

        // self.partial_ords[self.cards[10 ** len(right)]]
        let card_word = self.card(&pow10(right_len as u32))?; // KeyError on miss
        let ord_word = PARTIAL_ORDS
            .iter()
            .find(|&&(k, _)| k == card_word.as_str())
            .map(|&(_, v)| v)
            .ok_or_else(|| N2WError::Key(format!("'{}'", card_word)))?;

        // to_cardinal(int(left)) + " egész " + to_cardinal(int(right)) + " " + ord
        Ok(format!(
            "{} egész {} {}",
            self.to_cardinal(&left_int)?,
            self.to_cardinal(&right_int)?,
            ord_word
        ))
    }
}

pub fn new() -> LangHu {
    LangHu::new()
}
