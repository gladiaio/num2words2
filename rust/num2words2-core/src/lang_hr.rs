//! Port of `lang_HR.py` (Croatian).
//!
//! Shape: **self-contained**. `Num2Word_HR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `SCALE` table ends
//! at 10^30; `maxval()` is 10^33 and larger values raise `OverflowError`
//! (gladiaio/num2words2#159, see below).
//!
//! `setup()` sets `negword = "minus"` and `pointword = "zarez"`; everything
//! else stays at the `Num2Word_Base.__init__` defaults (notably
//! `is_title = False`, so [`Lang::title`] is a no-op here).
//!
//! Inherited from `Num2Word_Base` (unchanged by HR, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`.
//!     Verified against the corpus: `0` → "0", `-1` → "-1". For a
//!     float/Decimal the dispatcher `str()`s the returned value, which
//!     [`Lang::ordinal_num_float_entry`]'s default echoes (`1e+16` → "1e+16",
//!     `Decimal("1E+2")` → "1E+2", `-0.0` → "-0.0" — all corpus-pinned).
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` → default
//!     delegates through `&self`, picking up the `to_cardinal` override
//!     below. HR adds no era/century handling at all, so `to_year` is
//!     *literally* `to_cardinal`: year `2024` → "dvije tisuće dvadeset
//!     četiri" and year `-500` → "minus petsto" (no "prije Krista"/BC
//!     suffix). Note the `**kwargs`: Base's signature swallows every keyword
//!     unread, so `to_year(5, anything=...)` is just "pet" — see
//!     [`LangHr::to_year_kw`] (`Lang::to_year_kw`).
//!
//! # Float/Decimal routing — everything is textual
//!
//! `Num2Word_HR.to_cardinal` starts with `n = str(number).replace(",", ".")`
//! and branches on `"." in n`. That single string test is the whole routing
//! story, and it differs from Base semantics in every direction at once:
//!
//!   * A **whole float keeps its ".0" tail**: `str(5.0)` is "5.0", so
//!     `to_cardinal(5.0)` is "pet zarez nula" (the "0" fractional digit
//!     is counted both as a leading zero and by `_int2word(0)` — see
//!     [`LangHr::cardinal_float_textual`]), never Base's whole-value "pet".
//!     [`Lang::cardinal_float_entry`] is therefore overridden to send *every*
//!     float/Decimal through the textual branch, like `lang_cs.rs`.
//!   * An **integral Decimal has no point**: `str(Decimal("5"))` is "5", so it
//!     takes the `else` arm — `int("5")` → "pet".
//!   * A **string form in exponent notation feeds `int()` a non-integer
//!     token** and raises `ValueError`: `str(1e16)` is "1e+16" (repr switches
//!     to exponent form at 1e16), `str(Decimal("1E+2"))` is "1E+2". Both are
//!     corpus-pinned as ValueError for cardinal and year. This is HR's de
//!     facto float ceiling, and it is a *ValueError*, unlike the int path's
//!     10^33 `OverflowError`.
//!   * `str(-0.0)` is "-0.0", so **negative zero renders the negword**:
//!     "minus nula zarez nula" (corpus-pinned for float and Decimal
//!     alike; the binding smuggles `Decimal("-0.0")` in as an f64 `-0.0`
//!     because `BigDecimal` cannot carry the sign of zero).
//!
//! `to_ordinal`, by contrast, opens with `int(number)` — plain truncation
//! toward zero — so floats lose their fraction *before* the table lookup:
//! `to_ordinal(2.5)` == "drugi", `to_ordinal(0.5)` == "nulti",
//! `to_ordinal(-1.5)` == "minus prvi", and `to_ordinal(1e16)` ==
//! "deset bilijardii" (int(1e16) is exact, then cardinal + "i"). No
//! ValueError here: `int(float)` succeeds where `int(str)` failed. See
//! [`Lang::ordinal_float_entry`].
//!
//! # The Infinity quirk (`str_to_number`)
//!
//! `num2words("Infinity", lang="hr")` in Python: `str_to_number` happily
//! returns `Decimal("Infinity")`, then HR's `to_cardinal` does
//! `int("Infinity")` → **ValueError**. The shim, however, maps
//! `ParsedNumber::Inf` to Base's OverflowError before any HR code runs, so
//! the only interception point is [`LangHr::str_to_number`] (`Lang::str_to_number`), which turns the
//! parsed Inf into the exact ValueError `int()` would raise. Corpus-pinned
//! for "Infinity" and "-Infinity" (cardinal). Known cost, documented rather
//! than hidden: `to_ordinal(Decimal("Infinity"))` in Python raises
//! OverflowError (from `int()`, not caught by the `except (ValueError,
//! TypeError)` arm), which this override turns into ValueError too — no
//! corpus row exercises it. NaN is left to the shim, whose ValueError already
//! matches `int("NaN")`'s type.
//!
//! # Grammatical kwargs: `feminine`
//!
//! `to_cardinal(self, number, feminine=False)` is the only HR signature with
//! a grammatical kwarg. It selects `ONES[digit][1]` ("jedna"/"dvije") — but
//! only where a scale word doesn't force it anyway (`SCALE[1]` tisuća is
//! feminine regardless), and it is silently dropped on negatives (quirk 4
//! below). Corpus-pinned: `21` feminine → "dvadeset jedna", `-5` feminine →
//! "minus pet". [`LangHr::to_cardinal_kw`] / [`LangHr::to_cardinal_float_kw`]
//! (`Lang::to_cardinal_kw` / `Lang::to_cardinal_float_kw`) accept exactly
//! `feminine` (bool or None; anything else falls back to Python).
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. The following all look wrong but are exactly
//! what Python emits, verified against the interpreter and the frozen corpus:
//!
//! 1. *(Fixed, #248.)* Python tabled only 1..=20, the round tens, 100 and
//!    1000, and glued "i" onto the cardinal for everything else ("četrdeset
//!    dvai", "dvjestoi"). Compound ordinals now inflect their last word
//!    ([`crate::compound_ordinal`]): 21 == "dvadeset prvi", 101 == "sto
//!    prvi", 1001 == "tisuća prvi", 200 == "dvjestoti", 0 == "nulti",
//!    10**6 == "milijunti". Round thousands and larger round values other
//!    than 10**6 (2000, 10000, 10**10, …) still take Python's `+ "i"`
//!    fallback ("dvije tisućei"); that remains a known gap.
//! 2. *(Fixed, #248.)* A negative ordinal is "minus" + the ordinal
//!    (`to_ordinal(-1)` == "minus prvi"). HR still never calls
//!    `verify_ordinal`, so negatives do not raise.
//! 3. **`SCALE[5]` is "bilijardu"**, an accusative form where every other
//!    entry is nominative ("bilijarda" would be the pattern-consistent word).
//!    The corpus confirms `to_cardinal(10**15)` == "bilijardu". Kept verbatim.
//! 4. **`_int2word` drops `feminine` when recursing on a negative** — Python's
//!    `self._int2word(abs(number))` omits the argument, so the flag silently
//!    resets to `False`. Unobservable in the four in-scope modes (which never
//!    pass `feminine=True`), but reproduced anyway. See [`LangHr::int2word`].
//! 5. **The `HUNDREDS` word for 600 is "šesto"**, which is also what
//!    `ONES[6]` + "o" would spell. That is genuinely the Croatian word; noted
//!    only because it makes 675 read "šesto sedamdeset pet".
//!
//! # The SCALE ceiling — now an `OverflowError` (#159)
//!
//! `SCALE` is keyed 0..=10, i.e. chunk indices up to 1000^10 == 10^30
//! ("kvintilijun"). `_int2word` indexes it with the chunk index for **every**
//! non-zero chunk, so a value needing a 12th chunk — that is, any
//! `abs(n) >= 10**33` — raised `KeyError` in Python with the missing chunk
//! index as the key.
//!
//! The leading chunk of a Python `int` is never zero, so the guard
//! `if chunk_len > 0 and chunk != 0` never spares it: the crash is
//! unconditional above the ceiling. Fixed (gladiaio/num2words2#159):
//! `maxval()` is 10^33 and `_int2word` raises `OverflowError` for
//! `abs(n) >= 10**33` before any lookup, so `to_cardinal(10**33 - 1)` is
//! "devetsto devedeset devet kvintilijuna …" and `to_cardinal(10**33)` is an
//! `OverflowError`. [`scale`] still returns `None` past the table and the call
//! sites map that to [`N2WError::Key`], but that arm is unreachable from the
//! public entry points.
//!
//! # Currency
//!
//! `Num2Word_HR` carries its own `CURRENCY_FORMS` — HRK, EUR, USD and nothing
//! else — so it is untouched by the `lang_EUR.py` class-dict mutation that
//! `Num2Word_EN.__init__` performs (verified live: HR's dict `is not` EN's).
//! Every other code therefore raises `NotImplementedError`, which the corpus
//! pins for GBP/JPY/KWD/BHD/INR/CNY/CHF.
//!
//! It overrides `to_currency` and `_cents_verbose`; `to_cheque`,
//! `_money_verbose` and `_cents_terse` stay `Num2Word_Base`'s (the port fixes
//! `to_cheque`'s unit word, quirk 6, and makes `_money_verbose` and the cents
//! honour the gender flags, quirk 11). It defines
//! neither `CURRENCY_ADJECTIVES` nor `CURRENCY_PRECISION`, so both remain
//! Base's empty dicts and [`Lang::currency_precision`] keeps its default 100
//! for every code.
//!
//! See [`build_currency_forms`] for why each form tuple's trailing gender flag
//! is carried as the *string* `"False"`/`"True"` rather than a `bool`.
//!
//! # Faithfully reproduced Python oddities (currency)
//!
//! 6. **`to_cheque` printed the gender flag as the currency name (fixed,
//!    #189).** `Num2Word_Base.to_cheque` does
//!    `cr1, _cr2 = self.CURRENCY_FORMS[currency]` then
//!    `unit = cr1[-1] if isinstance(cr1, tuple) else cr1`, intending "the
//!    plural form". HR's `cr1` is a **4-tuple**, so `cr1[-1]` is the trailing
//!    `bool`, which `"%s"` renders as `False`/`True` and `.upper()` then
//!    shouts: `to_cheque(12.5, "EUR")` == "DVANAEST AND 50/100 FALSE" (HRK:
//!    "TRUE"). Same bug as `sr` (#176); [`LangHr::to_cheque`] goes through the
//!    shared `lang_sr::sr_to_cheque`, which takes the "many" form:
//!    "DVANAEST AND 50/100 EURA". "AND"/"MINUS" stay English, as in every
//!    other language's cheque format.
//! 7. **`to_currency` accepts `adjective=` and then ignores it.** The parameter
//!    is in HR's signature but its body never reads it — unlike
//!    `Num2Word_Base.to_currency`, which would apply `prefix_currency`. So
//!    `to_currency(12.34, "EUR", adjective=True)` == the plain
//!    "dvanaest eura, trideset četiri centa" (verified live). Moot in practice
//!    since `CURRENCY_ADJECTIVES` is empty anyway, but the parameter is dropped
//!    here rather than wired to [`Lang::currency_adjective`].
//! 8. **`to_currency` never consults `CURRENCY_PRECISION`.** It calls
//!    `parse_currency_parts` without a `divisor=`, taking that function's
//!    default of 100, and hardcodes `(decimal_val * 100) % 1` for its
//!    fractional-cents test. HR's table is empty so the two agree today, but
//!    the 100 below is Python's literal, not `self.currency_precision(...)`:
//!    adding a 3-decimal code to `CURRENCY_FORMS` would *not* make it a
//!    3-decimal currency here. Consequently the `divisor == 1` (JPY/KRW)
//!    rounding shortcut in `Num2Word_Base.to_currency` has no counterpart at
//!    all — HR never reaches that code.
//! 9. **HR's `is_float` is not Base's `has_decimal` guard.** Base decides
//!    whether to print cents with `isinstance(val, float) or "." in str(val)`;
//!    HR instead uses `not isinstance(val, int)`, fixed at the top of the
//!    function. The two disagree on `Decimal("5")`, which HR renders as
//!    "pet eura, nula centi" where Base would say "pet eura" (verified live).
//!    So [`CurrencyValue`]'s `has_decimal` field is deliberately **unread**
//!    here — the `Int` vs `Decimal` variant is the whole test.
//! 10. **The `separator=""` default is falsy, and means a comma.** HR's
//!    signature says `separator=""`, then its body does
//!    `sep = separator if separator else ","`. So an omitted separator — and an
//!    explicitly empty one — both produce ", ", while a real one is used as
//!    given: `separator=" i"` yields "dvanaest eura i trideset četiri centa"
//!    (verified live). The generated [`Lang::default_separator`] below returns
//!    `""` because that is the literal in the signature; the falsy-to-comma
//!    step happens inside [`LangHr::to_currency`], as it does in Python.
//!
//! 11. **The numerals ignored the currency's gender (fixed, #196).** Python
//!    spelled both amounts with `self.to_cardinal` (masculine), although the
//!    form tuples flag kuna and lipa as feminine: `to_currency(1, "HRK")` was
//!    "jedan kuna" and 2.02 "dva kune, dva lipe". The units numeral now goes
//!    through [`LangHr::money_verbose`], which reads the unit's flag, and the
//!    cents through `_cents_verbose`, which reads the subunit's: "jedna
//!    kuna", "dvije kune, dvije lipe". EUR/USD (euro/dolar, cent) are
//!    masculine and unchanged. As for `sr` (#188), only the units word is
//!    re-gendered.
//!
//! # Scope: fractional cents
//!
//! Reached when `(value * 100) % 1 != 0`, i.e. the value carries more than two
//! decimal places, which makes `parse_currency_parts` hand back a `Decimal`
//! rather than an `int` for the cents.
//!
//! * HR's branch is `self.to_cardinal_float(float(right))` — it names
//!   `to_cardinal_float`, so it lands on `Num2Word_Base`'s digit-by-digit float
//!   renderer directly, **not** on `Num2Word_HR.to_cardinal`'s own float branch
//!   (HR defines no method called `to_cardinal_float`; its float grammar lives
//!   *inside* `to_cardinal`).
//! * The Rust `Lang::to_cardinal_float` hook, however, now carries HR's
//!   textual grammar (it backs [`Lang::cardinal_float_entry`]), and the trait
//!   default of `cardinal_from_decimal` dispatches virtually through it. The
//!   two grammars agree on single-digit fractions ("zarez pet") but diverge on
//!   multi-digit ones: 65.35 cents is Base's "šezdeset pet zarez tri pet"
//!   (digit by digit) vs the textual "… zarez trideset pet" (one integer).
//!   [`LangHr::cardinal_from_decimal`] (`Lang::cardinal_from_decimal`)
//!   therefore pins the Base grammar explicitly — f64-cast, repr-derived
//!   precision, `floatpath::default_to_cardinal_float` — exactly the method
//!   Python resolves.
//!
//! Verified against the live interpreter — `to_currency(0.001, "EUR")` ==
//! "nula eura, nula zarez jedan centi" and `to_currency(12.345, "EUR")` ==
//! "dvanaest eura, trideset četiri zarez pet centi". (Contrast `lang_SR`,
//! whose sibling code reaches `self.to_cardinal` and therefore *does* diverge
//! under the same trait default.)
//!
//! The terse half of that corner — `cents=False` **and** fractional cents,
//! where Python emits `str(float(right))` — is handled by [`repr_float`], which
//! documents the one place Rust's float formatting differs from CPython's and
//! why the difference is closable here.
//!
//! # Verification
//!
//! All 117 `hr` currency + cheque corpus rows match byte for byte (cheque
//! rows bar the unit word, quirk 6). Beyond the
//! corpus, ~20k differential cases were run against the live interpreter —
//! every code (including **HRK**, which the corpus never exercises, and unknown
//! codes), `cents=True`/`False`, `separator` omitted/`""`/`" i"`/`","`, `int` /
//! `float` / `Decimal` inputs, negatives, exact powers of ten, the 11..=14
//! paucal boundaries, and fractional cents — with zero mismatches.
//!
//! The float/Decimal, string and kwargs behaviour above was additionally
//! checked against the live interpreter: whole floats ("pet zarez nula
//! nula"), `Decimal("5.00")`'s three nulas, the 1e16 / `1E+2` ValueErrors
//! (message included), ordinal truncation (2.5 → "drugi", 1e16 → "deset
//! bilijardii"), `Decimal("-0.0")` / `Decimal("-0")`, `feminine` threading
//! into both halves of "dvadeset jedna zarez dvadeset jedna", `to_year`'s
//! kwargs swallow, `int("Infinity")`'s exact ValueError text, and
//! `to_currency(0.6535)`'s digit-by-digit "šezdeset pet zarez tri pet centi".

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `ZERO[0]`.
const ZERO: &str = "nula";

/// `setup(): self.negword = "minus"`.
const NEGWORD: &str = "minus";

/// `setup(): self.pointword = "zarez"`.
const POINTWORD: &str = "zarez";

/// `ONES`, keys 1..=9, `(masculine, feminine)`. Index 0 is absent in Python
/// (guarded by `digit_right > 0`).
const ONES: [(&str, &str); 10] = [
    ("", ""), // absent in Python
    ("jedan", "jedna"),
    ("dva", "dvije"),
    ("tri", "tri"),
    ("četiri", "četiri"),
    ("pet", "pet"),
    ("šest", "šest"),
    ("sedam", "sedam"),
    ("osam", "osam"),
    ("devet", "devet"),
];

/// `TENS`, keys 0..=9 — the 10..=19 teens, keyed by the *units* digit.
const TENS: [&str; 10] = [
    "deset",
    "jedanaest",
    "dvanaest",
    "trinaest",
    "četrnaest",
    "petnaest",
    "šesnaest",
    "sedamnaest",
    "osamnaest",
    "devetnaest",
];

/// `TWENTIES`, keys 2..=9. Indices 0/1 are absent in Python (guarded by
/// `digit_mid > 1`, with `digit_mid == 1` handled by [`TENS`] instead).
const TWENTIES: [&str; 10] = [
    "", // absent in Python
    "", // absent in Python
    "dvadeset",
    "trideset",
    "četrdeset",
    "pedeset",
    "šezdeset",
    "sedamdeset",
    "osamdeset",
    "devedeset",
];

/// `HUNDREDS`, keys 1..=9. Index 0 is absent in Python (guarded by
/// `digit_left > 0`).
const HUNDREDS: [&str; 10] = [
    "", // absent in Python
    "sto",
    "dvjesto",
    "tristo",
    "četiristo",
    "petsto",
    "šesto",
    "sedamsto",
    "osamsto",
    "devetsto",
];

/// `SCALE`: chunk index → `(singular, paucal, plural, is_feminine)`.
///
/// Keys 0..=10 only — see the module docs on the `KeyError` ceiling. Index 5
/// is "bilijardu" in the Python source; that typo is load-bearing for parity.
const SCALE: [(&str, &str, &str, bool); 11] = [
    ("", "", "", false),                                          // 10^0
    ("tisuća", "tisuće", "tisuća", true),                         // 10^3
    ("milijun", "milijuna", "milijuna", false),                   // 10^6
    ("milijarda", "milijarde", "milijardi", false),               // 10^9
    ("bilijun", "bilijuna", "bilijuna", false),                   // 10^12
    ("bilijardu", "bilijarde", "bilijardi", false),               // 10^15
    ("trilijun", "trilijuna", "trilijuna", false),                // 10^18
    ("trilijarda", "trilijarde", "trilijardi", false),            // 10^21
    ("kvadrilijun", "kvadrilijuna", "kvadrilijuna", false),       // 10^24
    ("kvadrilijarda", "kvadrilijarde", "kvadrilijardi", false),   // 10^27
    ("kvintilijun", "kvintilijuna", "kvintilijuna", false),       // 10^30
];

/// Python's `SCALE[idx]`. `None` models the `KeyError` the dict raises past
/// key 10; callers turn it into [`N2WError::Key`] with `idx` as the key, which
/// is what CPython puts in the exception (`KeyError: 11`). Unreachable from
/// the public entry points since the 10^33 MAXVAL check (#159).
fn scale(idx: usize) -> Option<&'static (&'static str, &'static str, &'static str, bool)> {
    SCALE.get(idx)
}

fn key_error(key: usize) -> N2WError {
    N2WError::Key(key.to_string())
}

/// `to_ordinal`'s inline `ordinals` dict. Insertion order is irrelevant — it
/// is only ever probed with `in` / `[]`.
const ORDINALS: [(u32, &str); 29] = [
    (1, "prvi"),
    (2, "drugi"),
    (3, "treći"),
    (4, "četvrti"),
    (5, "peti"),
    (6, "šesti"),
    (7, "sedmi"),
    (8, "osmi"),
    (9, "deveti"),
    (10, "deseti"),
    (11, "jedanaesti"),
    (12, "dvanaesti"),
    (13, "trinaesti"),
    (14, "četrnaesti"),
    (15, "petnaesti"),
    (16, "šesnaesti"),
    (17, "sedamnaesti"),
    (18, "osamnaesti"),
    (19, "devetnaesti"),
    (20, "dvadeseti"),
    (30, "trideseti"),
    (40, "četrdeseti"),
    (50, "pedeseti"),
    (60, "šezdeseti"),
    (70, "sedamdeseti"),
    (80, "osamdeseti"),
    (90, "devedeseti"),
    (100, "stoti"),
    (1000, "tisući"),
];

/// Ordinal hundreds 100..=900 (#248). Index 0 is unused.
const HUNDREDS_ORD: [&str; 10] = [
    "", "stoti", "dvjestoti", "tristoti", "četiristoti", "petstoti", "šeststoti", "sedamstoti",
    "osamstoti", "devetstoti",
];

/// Python's `num in ordinals` / `ordinals[num]`.
///
/// A linear scan over 29 entries rather than a `to_u32` narrowing: the table
/// is tiny, and comparing `BigInt`s directly keeps the lookup total for the
/// unbounded inputs this crate must accept (`10**606` must miss, not panic).
fn ordinal_lookup(value: &BigInt) -> Option<&'static str> {
    ORDINALS
        .iter()
        .find(|(k, _)| *value == BigInt::from(*k))
        .map(|(_, word)| *word)
}

/// Python's `"%03d" % n` for the non-negative values this module produces.
///
/// `_int2word` strips the sign before stringifying, and `splitbyx(_, 3)` only
/// ever yields chunks in 0..=999, so the sign-aware branch of `%03d` is
/// unreachable and the result is always exactly 3 chars.
fn fmt_03(n: &BigInt) -> String {
    let s = n.to_string();
    let len = s.chars().count();
    if len >= 3 {
        s
    } else {
        format!("{}{}", "0".repeat(3 - len), s)
    }
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
///
/// Infallible here: `_int2word` only ever passes `str(abs(int))`, i.e. a
/// non-empty `[0-9]+` string, so every slice parses. (Contrast `lang_PL`,
/// where a stray "-" reaches this function and raises `ValueError`.)
fn splitbyx(n: &str, x: usize) -> Vec<BigInt> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let take = |i: usize, j: usize| -> BigInt {
        let s: String = chars[i..j.min(length)].iter().collect();
        BigInt::parse_bytes(s.as_bytes(), 10).expect("splitbyx: non-empty digit string")
    };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(take(0, start));
        }
        let mut i = start;
        while i < length {
            out.push(take(i, i + x));
            i += x;
        }
    } else {
        out.push(take(0, length));
    }
    out
}

/// Port of `utils.get_digits(n)`. Returns `[right, mid, left]` — least
/// significant first, matching the `reversed(...)` in Python.
fn get_digits(n: &BigInt) -> [usize; 3] {
    let s = fmt_03(n);
    let chars: Vec<char> = s.chars().collect();
    // fmt_03 always yields >= 3 chars, so the [-3:] slice is total.
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).expect("get_digits: decimal digit") as usize;
    }
    a
}

/// The form *index* `Num2Word_HR.pluralize` selects — 0 (singular), 1 (paucal)
/// or 2 (plural).
///
/// Factored out of [`pluralize`] because Python has exactly one duck-typed
/// `pluralize` that indexes whatever sequence it is handed, while the two Rust
/// call sites carry different form shapes: the `SCALE` 4-tuple in
/// [`LangHr::int2word`], and `CurrencyForms`' `Vec<String>` in
/// [`LangHr::pluralize`]. Keeping the *rule* in one place keeps them from
/// drifting apart.
///
/// `number` is non-negative at every call site — `_int2word` strips the sign
/// before chunking, and `to_currency` only ever passes `abs()`-ed parts — so
/// Python's floor-`%` and Rust's `%` would agree; `mod_floor` is used
/// regardless to keep the Python semantics explicit.
fn pluralize_index(number: &BigInt) -> usize {
    let m100 = number.mod_floor(&BigInt::from(100u32));
    let m10 = number.mod_floor(&BigInt::from(10u32));

    let is_teen = [11u32, 12, 13, 14].iter().any(|t| m100 == BigInt::from(*t));

    if is_teen {
        2 // plural for teens
    } else if m10.is_one() {
        0 // singular
    } else if [2u32, 3, 4].iter().any(|t| m10 == BigInt::from(*t)) {
        1 // paucal
    } else {
        2 // plural
    }
}

/// Port of `Num2Word_HR.pluralize` over a [`SCALE`] entry.
fn pluralize<'a>(number: &BigInt, forms: &'a (&'a str, &'a str, &'a str, bool)) -> &'a str {
    match pluralize_index(number) {
        0 => forms.0,
        1 => forms.1,
        _ => forms.2,
    }
}

/// `Num2Word_HR.CURRENCY_FORMS`, verbatim — HRK, EUR, USD and nothing else.
///
/// # Why the gender flag is a `"False"`/`"True"` string
///
/// Python's entries are pairs of **4-tuples**: `("euro", "eura", "eura",
/// False)`. That trailing element is a `bool`, and two callers read it wanting
/// two different things:
///
/// * `Num2Word_HR._cents_verbose` reads `CURRENCY_FORMS[cur][1][-1]` and passes
///   it as `_int2word`'s `feminine` argument — its intended purpose.
/// * `Num2Word_Base.to_cheque` reads `cr1[-1]` believing it is the plural unit
///   name and interpolates it into a `"%s"` — module-doc quirk 6. Python
///   renders the bool as the text `False`/`True`, which `.upper()` shouts.
///   The port's [`LangHr::to_cheque`] reads the "many" form instead (#189).
///
/// `CurrencyForms` stores `Vec<String>`, so the flag is kept as the exact text
/// Python's `"%s"` produces, and [`LangHr::cents_verbose`] recovers the
/// boolean by comparing against `"True"`. This mirrors what `lang_sr.rs` does
/// with the identical Python shape.
///
/// The arity is load-bearing beyond that: [`LangHr::pluralize`] indexes 0..=2,
/// so dropping the third form would silently change output.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENT: [&str; 4] = ["cent", "centa", "centi", "False"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "HRK",
        CurrencyForms::new(
            &["kuna", "kune", "kuna", "True"],
            &["lipa", "lipe", "lipa", "True"],
        ),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "eura", "eura", "False"], &CENT),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["dolar", "dolara", "dolara", "False"], &CENT),
    );
    m
}

/// Python's `str(float(x))` for the terse fractional-cents corner.
///
/// # This is not a general `repr(float)` — it is correct only on `[0, 100)`
///
/// `currency.rs`'s module docs are explicit that re-deriving Python's float
/// formatting in Rust is a permanent source of drift, and they are right about
/// the general case. This is deliberately *not* that: the shortest-round-trip
/// digits still come from Rust's own formatter, which shares CPython `repr`'s
/// contract. Only two mechanical differences are patched, and only over a
/// domain small enough to test exhaustively:
///
/// 1. **The exponent-notation threshold.** CPython's `repr` switches to
///    scientific below `1e-4` (`repr(1e-05) == "1e-05"`, `repr(1e-4) ==
///    "0.0001"`); Rust's `{}` never does. Rust's `{:e}` always does, so the
///    branch picks the right one and reuses `{:e}`'s mantissa as-is.
/// 2. **Exponent zero-padding.** CPython pads to two digits (`1e-05`); Rust's
///    `{:e}` does not (`1e-5`).
///
/// The other half of `repr`'s rule — switching back to scientific at `>= 1e16`
/// — is **not** implemented, because it is unreachable: the only caller passes
/// `right`, the subunit count, which is `fraction * 100` for `fraction` in
/// `[0, 1)`, hence finite, non-negative and `< 100`. Do not reuse this function
/// outside that domain.
///
/// Verified against CPython over 160,965 values spanning exactly that range —
/// uniform draws over `[0, 100)` and over `[0, 1e-4)`, every power of ten down
/// to `1e-319`, subnormals, and the `1e-4` boundary itself — with zero
/// mismatches.
///
/// The `.0` fixup covers the third difference (`repr(34.0)` is `"34.0"` where
/// `format!("{}", 34.0)` is `"34"`). `right` is provably non-integral whenever
/// this runs (see [`LangHr::to_currency`]), but `to_f64` can round a value like
/// `34.000000000000000001` onto an integral double, so the fixup is not dead.
fn repr_float(v: &BigDecimal) -> Result<String> {
    let f = v
        .to_f64()
        .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", v)))?;

    if f != 0.0 && f.abs() < 1e-4 {
        // Rust: "1e-5" / "1.5e-5"  ->  CPython: "1e-05" / "1.5e-05".
        let s = format!("{:e}", f);
        let (mantissa, exp) = match s.split_once('e') {
            Some(parts) => parts,
            // `{:e}` on a finite f64 always emits an exponent; a value that
            // reached the `< 1e-4` branch cannot be inf/NaN.
            None => return Ok(s),
        };
        let (sign, digits) = match exp.strip_prefix('-') {
            Some(d) => ("-", d),
            None => ("+", exp),
        };
        return Ok(format!("{}e{}{:0>2}", mantissa, sign, digits));
    }

    let s = format!("{}", f);
    if s.contains('.') {
        Ok(s)
    } else {
        Ok(format!("{}.0", s))
    }
}

/// Reproduce CPython's `repr(float)` (== `str(float)`) for an `f64`.
///
/// `Num2Word_HR.to_cardinal` starts with `n = str(number)` and then *string-
/// splits* on `.` — it never calls `float2tuple`, so the f64-artefact heuristic
/// in `floatpath` is irrelevant here: the port must rebuild the exact repr
/// string, exponent form included (`str(1e16)` is "1e+16", which is what makes
/// HR raise ValueError above 1e16 — see [`LangHr::cardinal_float_textual`]).
///
/// CPython uses David Gay's shortest `dtoa` (mode 0): a digit string plus a
/// decimal-point position `decpt`. It then prints fixed notation iff
/// `-4 < decpt <= 16` and scientific otherwise, appending `.0` in fixed
/// notation when nothing follows the point (`Py_DTSF_ADD_DOT_0`) and padding
/// the scientific exponent to at least two digits (`1e-05`, `1e+16`). Rust's
/// `{:e}` emits the same shortest digit string with `decpt == exp + 1`, so the
/// two reconstruct identically. Same helper as `lang_cs.rs` (whose Python
/// original shares HR's textual `to_cardinal` shape), duplicated because
/// language modules do not import from one another.
fn python_repr_f64(f: f64) -> String {
    if f.is_nan() {
        // repr(float('nan')) == 'nan', sign dropped. Feeds the no-'.' branch,
        // where int('nan') raises ValueError — reproduced downstream.
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative` captures -0.0, whose repr is '-0.0'.
    let neg = f.is_sign_negative();
    let s = format!("{:e}", f.abs()); // e.g. "1.234e1", "5e-1", "0e0"
    let (mantissa, exp_s) = match s.split_once('e') {
        Some(parts) => parts,
        None => return s, // unreachable for finite f64
    };
    let exp: i32 = exp_s.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let ndigits = digits.len() as i32;
    let decpt = exp + 1;

    let body = if decpt <= -4 || decpt > 16 {
        // Scientific: first digit, optional ".rest", then "e±NN".
        let mut m = String::new();
        m.push_str(&digits[..1]);
        if ndigits > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        let e = decpt - 1;
        let (esign, eabs) = if e < 0 { ('-', -e) } else { ('+', e) };
        format!("{}e{}{:02}", m, esign, eabs)
    } else if decpt <= 0 {
        // 0.00…digits
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndigits {
        // digits, trailing zeros, then the ADD_DOT_0 ".0".
        format!("{}{}.0", digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        // digits[:decpt] "." digits[decpt:]
        let dp = decpt as usize;
        format!("{}.{}", &digits[..dp], &digits[dp..])
    };

    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// Python's `int(token)` for the tokens `Num2Word_HR.to_cardinal` feeds it:
/// `int(left)`, `int(right)` (unsigned — the sign is stripped before the
/// split) and `int(n)` in the else arm (signed — `str(Decimal("-3"))` is
/// "-3", and `_int2word` does its own negative handling).
///
/// A token with any non-digit — the `'e'`/`'E'`/`'+'` of an exponent-form
/// repr ("1e+16", "1E+2"), `'inf'`, `'nan'`, `'Infinity'` — is where Python's
/// `int()` raises `ValueError`, which the HR float path never guards. Same
/// exception type, same message format string.
fn int_from_str(token: &str) -> Result<BigInt> {
    let unsigned = token.strip_prefix('-').unwrap_or(token);
    if unsigned.is_empty() || !unsigned.chars().all(|c| c.is_ascii_digit()) {
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            token
        )));
    }
    Ok(BigInt::parse_bytes(token.as_bytes(), 10).expect("int_from_str: validated digit string"))
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the repr-derived
/// precision Base's `float2tuple` assigns. Local copy of `floatpath`'s
/// private helper, used only by [`LangHr::cardinal_from_decimal`].
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Extract the `feminine=` kwarg the way `Num2Word_HR.to_cardinal` consumes
/// it.
///
/// * omitted → the signature default `False`;
/// * an explicit `None` → falsy, so `None or SCALE[i][-1]` behaves exactly
///   like `False` — `int(None)` is never reached;
/// * a bool → itself (the only shape the kwargs corpus exercises);
/// * anything else (`feminine=2`, a string, …) → NotImplemented, so the
///   dispatcher falls back to Python, which reproduces its own outcome
///   (`gender_idx = int(is_feminine)` then `ONES[d][gender_idx]` —
///   IndexError for 2, TypeError for a non-int, truthiness for 1).
fn kw_feminine(kw: &Kwargs) -> Result<bool> {
    match kw.get("feminine") {
        Option::None | Some(KwVal::None) => Ok(false),
        Some(KwVal::Bool(b)) => Ok(*b),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

pub struct LangHr {
    /// `Num2Word_HR.CURRENCY_FORMS`. Built once in [`LangHr::new`] and read
    /// thereafter — rebuilding it per call is what made an earlier revision of
    /// this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangHr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangHr {
    pub fn new() -> Self {
        LangHr {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_HR._int2word`.
    fn int2word(&self, number: &BigInt, feminine: bool) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_negative() {
            // Python: " ".join([self.negword, self._int2word(abs(number))])
            //
            // Note the dropped `feminine`: the recursive call omits it, so the
            // flag resets to False. Preserved rather than fixed — see the
            // module docs.
            let inner = self.int2word(&number.abs(), false)?;
            return Ok(format!("{} {}", NEGWORD, inner));
        }

        if number.is_zero() {
            return Ok(ZERO.to_string());
        }

        // Special cases for exact powers of thousands. These short-circuit
        // before the chunk loop, which is why 1000 is "tisuća" (bare) rather
        // than "jedna tisuća", while 1001 is "tisuća jedan".
        if *number == BigInt::from(1_000u32) {
            return Ok("tisuća".to_string());
        } else if *number == BigInt::from(1_000_000u32) {
            return Ok("milijun".to_string());
        } else if *number == BigInt::from(1_000_000_000u32) {
            return Ok("milijarda".to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&number.to_string(), 3);
        let mut chunk_len = chunks.len();

        for chunk in &chunks {
            chunk_len -= 1;
            let [digit_right, digit_mid, digit_left] = get_digits(chunk);

            if digit_left > 0 {
                words.push(HUNDREDS[digit_left].to_string());
            }

            if digit_mid > 1 {
                words.push(TWENTIES[digit_mid].to_string());
            }

            if digit_mid == 1 {
                words.push(TENS[digit_right].to_string());
            } else if digit_right > 0 {
                // Skip "jedna/jedan" for thousands, millions etc. when it is
                // the leading digit of a scaled chunk: 1000000 -> "milijun",
                // not "jedan milijun".
                if !(chunk_len > 0 && digit_left == 0 && digit_mid == 0 && digit_right == 1) {
                    let sc = scale(chunk_len).ok_or_else(|| key_error(chunk_len))?;
                    let is_feminine = feminine || sc.3;
                    // Python: gender_idx = int(is_feminine)
                    let gender_idx = usize::from(is_feminine);
                    let ones = ONES[digit_right];
                    words.push(if gender_idx == 0 {
                        ones.0.to_string()
                    } else {
                        ones.1.to_string()
                    });
                }
            }

            if chunk_len > 0 && !chunk.is_zero() {
                let sc = scale(chunk_len).ok_or_else(|| key_error(chunk_len))?;
                words.push(pluralize(chunk, sc).to_string());
            }
        }

        Ok(words.join(" "))
    }

    /// Port of `Num2Word_HR.to_cardinal` over a float/Decimal — the full
    /// textual algorithm, `feminine` included:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count + self._int2word(int(right), feminine)
    ///     result = "%s %s %s" % (self._int2word(int(left), feminine), self.pointword, decimal_part)
    ///     if is_negative:
    ///         result = self.negword + " " + result
    ///     return result
    /// else:
    ///     return self._int2word(int(n), feminine)
    /// ```
    ///
    /// Two consequences of the *textual* approach, both load-bearing and unlike
    /// `float2tuple`:
    ///
    /// * The fractional part is read as **one integer**, not digit by digit:
    ///   `1.10` → "jedan zarez deset" (not "... jedan nula"), `2.675` → "dva
    ///   zarez šesto sedamdeset pet" (no `674.999…` binary-residue rescue is
    ///   ever needed — the repr string carries "675" directly).
    /// * An all-zero fraction reads one "nula" per digit written, so `1.0` →
    ///   "jedan zarez nula" and `Decimal("10.00")` → "deset zarez nula nula"
    ///   (Python counted a `"0"` both as a leading zero and by `int2word(0)`,
    ///   adding one; fixed, gladiaio/num2words2#237).
    ///
    /// `str(number)` is reconstructed per variant:
    /// * `Float` → [`python_repr_f64`], exact — including the exponent form
    ///   ("1e+16") whose `int()` failure is HR's float ceiling, and "-0.0"
    ///   whose sign survives into "minus nula zarez nula". One carve-out:
    ///   the shim smuggles a *Decimal* negative zero in as f64 `-0.0` (the
    ///   sign of zero doesn't fit a `BigDecimal`), so a zero whose `precision`
    ///   isn't repr's fixed 1 is re-expanded to the Decimal's own string
    ///   ("-0" @ 0, "-0.00" @ 2). A genuine float zero always has
    ///   precision 1, where both spellings agree on "0.0"/"-0.0".
    /// * `Decimal` → [`python_decimal_str`], the exact `str(Decimal)` — keeps
    ///   trailing zeros ("5.00"), switches to "1E+2" exponent form when the
    ///   exponent is positive (→ ValueError), prints integral values bare
    ///   ("5" → integer branch).
    ///
    /// The `.replace(",", ".")` is a no-op on both reconstructions (no repr
    /// carries a comma) and is not modelled.
    fn cardinal_float_textual(&self, value: &FloatValue, feminine: bool) -> Result<String> {
        let n = match value {
            FloatValue::Float { value, precision } => {
                if *value == 0.0 && *precision != 1 {
                    // Smuggled Decimal zero (see doc above): rebuild
                    // str(Decimal) from the sign bit and the scale.
                    let sign = if value.is_sign_negative() { "-" } else { "" };
                    if *precision == 0 {
                        format!("{}0", sign)
                    } else {
                        format!("{}0.{}", sign, "0".repeat(*precision as usize))
                    }
                } else {
                    python_repr_f64(*value)
                }
            }
            FloatValue::Decimal { value, .. } => python_decimal_str(value),
        };

        // Python: `if "." in n`.
        let is_negative = n.starts_with('-');
        // abs_n = n[1:] if is_negative else n — '-' is 1 ASCII byte, safe slice.
        let abs_n: &str = if is_negative { &n[1..] } else { &n };

        let (left, right) = match abs_n.split_once('.') {
            Some(parts) => parts,
            None => {
                // else: return self._int2word(int(n), feminine)
                // int(n) sees the *signed* token; exponent forms / inf / nan
                // raise ValueError here. _int2word handles the negative (and
                // drops `feminine` doing so — module-doc quirk 4).
                let n_int = int_from_str(&n)?;
                return self.int2word(&n_int, feminine);
            }
        };

        // leading_zero_count = len(right) - len(right.lstrip("0"))
        // (byte counts are char counts: reprs are pure ASCII).
        // The final int(right) word already says one zero, so an all-zero
        // fraction gets len - 1 leading zeros: exactly the digits written
        // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
        let leading_zero_count = (right.len() - right.trim_start_matches('0').len())
            .min(right.len().saturating_sub(1));

        // int(left), int(right) — a fractional token carrying an exponent
        // ("5e-05" from repr(1.5e-05)) raises ValueError, exactly as Python.
        let left_int = int_from_str(left)?;
        let right_int = int_from_str(right)?;

        // decimal_part = (ZERO + " ") * leading_zero_count + int2word(int(right), feminine)
        let mut decimal_part = String::new();
        for _ in 0..leading_zero_count {
            decimal_part.push_str(ZERO);
            decimal_part.push(' ');
        }
        decimal_part.push_str(&self.int2word(&right_int, feminine)?);

        // result = "%s %s %s" % (int2word(int(left), feminine), pointword, decimal_part)
        let mut result = format!(
            "{} {} {}",
            self.int2word(&left_int, feminine)?,
            POINTWORD,
            decimal_part
        );
        if is_negative {
            // Python: result = self.negword + " " + result
            result = format!("{} {}", NEGWORD, result);
        }
        Ok(result)
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangHr {
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
        ""
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "zarez"
    }

    /// Port of `Num2Word_HR.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).replace(",", ".")` and branches on whether
    /// `"."` is in `n`. `str(int)` never contains one, so integers always take
    /// the `else` branch: `self._int2word(int(n), feminine)` with the default
    /// `feminine=False`. The float branch (pointword, leading-zero padding of
    /// the decimal part) is out of scope.
    ///
    /// The sign is *not* stripped here — unlike most modules, HR hands the
    /// signed value straight to `_int2word`, which does its own negative
    /// handling.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, false)
    }

    /// `to_cardinal(float/Decimal)` — the full entry, whole values included.
    ///
    /// HR's `to_cardinal` is textual over `str(number)`, so *every*
    /// float/Decimal takes the same branch: a visible "." means the decimal
    /// grammar even for whole values (`5.0` → "pet zarez nula",
    /// `Decimal("5.00")` → "pet zarez nula nula"), no "." means `int(n)`
    /// (`Decimal("5")` → "pet"; exponent forms "1e+16"/"1E+2" → ValueError).
    /// Base's whole-value shortcut never applies — see the module docs.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// Port of `Num2Word_HR.to_ordinal` over a float/Decimal.
    ///
    /// Python opens with `num = int(number)` — truncation toward zero — so the
    /// fraction is gone *before* the table lookup: `2.5` → "drugi", `0.5` →
    /// "nulti", `-1.5` → "minus prvi", `-0.0` → "nulti" (int drops the zero's
    /// sign). `int(1e16)` succeeds (unlike the cardinal path's `int("1e+16")`),
    /// so `1e16` → "deset bilijardii". The `except (ValueError, TypeError)`
    /// arm is unreachable for the finite values this hook receives; the
    /// non-finite arms below are defensive fidelity (`int(inf)` →
    /// OverflowError uncaught; `int(nan)` → ValueError caught → `str(number)`).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let num = match value {
            FloatValue::Float { value, .. } => {
                if value.is_nan() {
                    // int(nan) → ValueError → except arm → str(number).
                    return Ok(python_repr_f64(*value));
                }
                if value.is_infinite() {
                    // int(inf) → OverflowError, not in the except tuple.
                    return Err(N2WError::Overflow(
                        "cannot convert float infinity to integer".into(),
                    ));
                }
                BigInt::from_f64(value.trunc())
                    .expect("hr ordinal: finite trunc converts exactly")
            }
            // int(Decimal) truncates toward zero; with_scale(0) does the same
            // (the convention float2tuple's `pre = int(value)` relies on).
            FloatValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
        };
        self.to_ordinal(&num)
    }

    // year_float_entry: HR does not override to_year, so a float year is
    // Base's `self.to_cardinal(value)` — the trait default already delegates
    // to `cardinal_float_entry`, picking up the override above (year 5.0 →
    // "pet zarez nula", year 1e16 → ValueError; both corpus-pinned).
    //
    // ordinal_num_float_entry: Base's to_ordinal_num returns the value
    // unchanged and the dispatcher str()s it — the trait default echoes the
    // Python-computed repr ("1e+16", "1E+2", "-0.0"), which is exactly that.

    /// Port of `Num2Word_HR.to_cardinal`'s **float/Decimal branch** — see
    /// [`LangHr::cardinal_float_textual`] for the algorithm and the
    /// reconstruction of `str(number)`.
    ///
    /// `feminine` is `False` here (the dispatcher calls `to_cardinal` without
    /// it); the kwargs entry with `feminine` is [`LangHr::to_cardinal_float_kw`].
    /// `precision_override` is **ignored**: HR's `to_cardinal` takes no
    /// `precision` and reads only `str(number)`, so the `precision=` kwarg
    /// cannot reach or alter this output (verified live).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_textual(value, false)
    }

    /// `converter.str_to_number` — Base's `Decimal(value)` with one twist.
    ///
    /// Python's `str_to_number` accepts "Infinity" fine; the ValueError comes
    /// one call later, from `int("Infinity")` inside HR's `to_cardinal`. The
    /// shim, however, maps a parsed `Inf` to Base's OverflowError before HR
    /// runs, so the raise is hoisted here — same type, same message, and the
    /// dispatcher propagates it unchanged for the digit-free "Infinity" /
    /// "-Infinity" (corpus-pinned ValueError). NaN passes through: the shim's
    /// ValueError already matches `int("NaN")`'s type, and hoisting it would
    /// misroute diagnostic forms like "nan123" into the sentence fallback.
    /// See the module docs for the (unpinned) ordinal trade-off.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}Infinity'",
                if negative { "-" } else { "" }
            ))),
            other => Ok(other),
        }
    }

    /// `to_cardinal(number, feminine=False)` — the one HR signature with a
    /// grammatical kwarg. Corpus-pinned: `1` → "jedna", `2` → "dvije", `21` →
    /// "dvadeset jedna"; `-5` → "minus pet" (the negative recursion drops the
    /// flag — module-doc quirk 4); `feminine=False` rows equal the plain
    /// cardinal.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["feminine"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let feminine = kw_feminine(kw)?;
        self.int2word(value, feminine)
    }

    /// `to_cardinal(float/Decimal, feminine=...)`: the textual branch threads
    /// `feminine` into *both* `_int2word` calls (`Decimal("21.21")` feminine →
    /// "dvadeset jedna zarez dvadeset jedna").
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["feminine"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let feminine = kw_feminine(kw)?;
        self.cardinal_float_textual(value, feminine)
    }

    /// `Num2Word_Base.to_year(self, value, **kwargs)` — the `**kwargs` bag is
    /// swallowed unread, then `self.to_cardinal(value)`. Any keyword —
    /// `feminine` included — is accepted and ignored: `to_year(5, feminine=
    /// True)` is "pet", not "jedna"-anything. Reproduce the swallow rather
    /// than falling back.
    ///
    /// to_ordinal / to_ordinal_num / to_currency take no extra kwargs in HR's
    /// (or the inherited Base) signatures, so their `*_kw` defaults are
    /// correct: any kwarg → NotImplemented → the original Python raises its
    /// own TypeError.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        let _ = kw; // **kwargs: accepted, never read.
        self.to_year(value)
    }

    /// Port of `Num2Word_HR.to_ordinal`.
    ///
    /// Python guards `int(number)` with `except (ValueError, TypeError):
    /// return str(number)`; a `BigInt` is already an integer, so that arm is
    /// unreachable and is not modelled.
    ///
    /// Outside the 29-entry table, Python glued "i" onto the cardinal;
    /// compounds now inflect their last word instead (#248, see the module
    /// docs for the round values still on that fallback).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{} {}", NEGWORD, self.to_ordinal(&value.abs())?));
        }
        if value.is_zero() {
            return Ok("nulti".to_string());
        }
        if let Some(word) = ordinal_lookup(value) {
            return Ok(word.to_string());
        }
        let cardinal = self.to_cardinal(value)?;
        if let Some(n) = value.to_u64() {
            if n == 1_000_000 {
                return Ok("milijunti".to_string());
            }
            let small = |v: u64| ordinal_lookup(&BigInt::from(v));
            if let Some(word) = crate::compound_ordinal::last_word_ordinal(
                n,
                &cardinal,
                small,
                Some(&HUNDREDS_ORD),
            ) {
                return Ok(word);
            }
        }
        // Python's comment: "This is a simplified implementation".
        Ok(format!("{}i", cardinal))
    }

    // to_ordinal_num: HR does not override Num2Word_Base.to_ordinal_num, which
    // returns the value unchanged for the dispatcher to stringify. The trait
    // default (`Ok(value.to_string())`) already matches — corpus: 0 -> "0",
    // -1 -> "-1".
    //
    // to_year: HR does not override Num2Word_Base.to_year either, and the
    // trait default delegates to `self.to_cardinal`, picking up the override
    // above. Corpus: 2024 -> "dvije tisuće dvadeset četiri", -500 -> "minus
    // petsto".

    // ---- currency -------------------------------------------------------
    //
    // HR overrides `to_currency` and `_cents_verbose`, and supplies its own
    // `CURRENCY_FORMS` + `pluralize`. `_money_verbose` is overridden here too,
    // to honour the unit's gender flag (#196); `_cents_terse` is
    // `Num2Word_Base`'s, which the trait default already mirrors. `to_cheque` is
    // Base's too, except for the unit word (quirk 6, #189).
    // `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are Base's empty dicts, so
    // `currency_adjective` (None) and `currency_precision` (100) are correct as
    // inherited.

    fn lang_name(&self) -> &str {
        "Num2Word_HR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Base's `_money_verbose` (`self.to_cardinal(number)`), with the units
    /// word agreeing with the unit's gender flag (#196): "jedna kuna",
    /// "dvadeset jedna kuna", "dvije kune"; EUR/USD stay masculine. Only the
    /// last word is re-gendered, so scale words keep their own gender.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let words = self.to_cardinal(number)?;
        let feminine = self
            .currency_forms
            .get(currency)
            .and_then(|f| f.unit.last())
            .is_some_and(|flag| flag.as_str() == "True");
        if !feminine {
            return Ok(words);
        }
        let (head, last) = match words.rsplit_once(' ') {
            Some((h, l)) => (Some(h), l),
            None => (None, words.as_str()),
        };
        Ok(match ONES.iter().skip(1).find(|(m, _)| *m == last) {
            Some((_, f)) => match head {
                Some(h) => format!("{} {}", h, f),
                None => f.to_string(),
            },
            None => words.clone(),
        })
    }

    /// `Num2Word_Base.to_cheque` with the "many" form as the unit word instead
    /// of the gender flag (quirk 6, #189), via the helper `sr` shares.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        crate::lang_sr::sr_to_cheque(self, val, currency)
    }

    /// Port of `Num2Word_HR.pluralize` over a `CURRENCY_FORMS` entry.
    ///
    /// Python indexes the tuple directly, so a form list shorter than the
    /// selected index raises IndexError. Every HR entry carries four elements,
    /// so this is unreachable — but it is mapped to [`N2WError::Index`] rather
    /// than unwrapped, so the exception *type* survives if the table changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(pluralize_index(n))
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_HR._cents_verbose(number, currency)`:
    /// `self._int2word(number, self.CURRENCY_FORMS[currency][1][-1])`.
    ///
    /// Note it calls `_int2word` directly rather than `to_cardinal`, and that
    /// the gender flag is the *subunit* tuple's trailing element — so HRK's
    /// `lipa` (flag `True`) would get feminine numerals ("jedna lipa") while
    /// EUR's `cent` (flag `False`) stays masculine ("jedan cent").
    ///
    /// Python's own `to_currency` spelled the cents with `self.to_cardinal`
    /// (masculine) and never reached this; the port's does, so "dvije lipe"
    /// agrees with the feminine subunit (#196). The cents are below 100, so
    /// `_int2word`'s flag only touches the units word.
    ///
    /// The `CURRENCY_FORMS[currency]` miss is Python's `KeyError`; `to_currency`
    /// has already raised NotImplementedError for an unknown code by then.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let forms = self
            .currency_forms
            .get(currency)
            .ok_or_else(|| N2WError::Key(format!("'{}'", currency)))?;
        let flag = forms
            .subunit
            .last()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
        // The flag is stored as Python's own `"%s" % bool` text; see
        // `build_currency_forms` for why.
        self.int2word(number, flag.as_str() == "True")
    }

    /// The fractional-cents renderer, `self.to_cardinal_float(float(right))`.
    ///
    /// Python resolves that name to `Num2Word_Base.to_cardinal_float` — HR
    /// defines no method called `to_cardinal_float`; its float grammar lives
    /// *inside* `to_cardinal`. The trait default (`cardinal_from_bigdecimal`)
    /// dispatches virtually through `Lang::to_cardinal_float`, which in this
    /// module now carries the textual grammar, so the Base digit-by-digit
    /// grammar is pinned explicitly here: f64 cast (Python's `float(right)`),
    /// repr-derived precision, then `default_to_cardinal_float` — whose
    /// `to_cardinal`/`pointword`/`title` callbacks still dispatch to HR, as
    /// the bound method's `self` does. The two grammars agree on single
    /// fractional digits (65.3 → "šezdeset pet zarez tri" both ways) and
    /// differ on multi-digit ones (65.35 → Base "… zarez tri pet", textual
    /// "… zarez trideset pet"); Python says Base. See the module docs.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        let precision = float_repr_precision(f);
        crate::floatpath::default_to_cardinal_float(
            self,
            &FloatValue::Float {
                value: f,
                precision,
            },
            None,
        )
    }

    /// Port of `Num2Word_HR.to_currency(val, currency="EUR", cents=True,
    /// separator="", adjective=False)`.
    ///
    /// HR replaces `Num2Word_Base.to_currency` entirely rather than extending
    /// it, so none of Base's machinery applies: no `has_decimal` guard, no
    /// `CURRENCY_PRECISION`, no `adjective` handling, and the cents are spelled
    /// with `self.to_cardinal` rather than `self._cents_verbose`. See the
    /// module docs, quirks 7-10.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Python names `adjective` in the signature and never reads it — quirk
        // 7. Dropped rather than wired to `currency_adjective`.
        _adjective: bool,
    ) -> Result<String> {
        // Python's `parse_currency_parts(...)` call omits `divisor=`, taking
        // that function's default of 100, and the fractional test hardcodes
        // `* 100`. Both are HR's literals, *not* `currency_precision` — quirk 8.
        const DIVISOR: i64 = 100;

        // The `isinstance(val, int)` fork. `is_float` is `not isinstance(val,
        // int)` — the `Decimal` variant counts as float even when it has no
        // decimal point, so `has_decimal` is deliberately unread — quirk 9.
        //
        // `fractional_cents` doubles as Python's `isinstance(right, Decimal)`
        // test further down: `parse_currency_parts` returns a `Decimal` for the
        // cents exactly when `keep_precision` was set, and an `int` otherwise.
        let (left, right, is_negative, is_float, fractional_cents) = match val {
            CurrencyValue::Int(v) => (v.abs(), BigDecimal::zero(), v.is_negative(), false, false),
            CurrencyValue::Decimal { value, .. } => {
                // has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0
                let scaled = value * BigDecimal::from(DIVISOR);
                let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();
                let (l, r, neg) = parse_currency_parts(val, false, has_fractional_cents, DIVISOR);
                (l, r, neg, true, has_fractional_cents)
            }
        };

        let forms = self.currency_forms(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        // Python: `self.to_cardinal(left)`, always masculine; the numeral
        // agrees with the unit instead ("jedna kuna", #196).
        let money_str = self.money_verbose(&left, currency)?;

        // Python: `if right > 0 or is_float:` — a true `int` always lands here
        // with `right = 0` and `is_float = False`, which is what keeps the cents
        // segment off ints while `1.0` still prints "nula centi".
        if !(right > BigDecimal::zero() || is_float) {
            return Ok(format!(
                "{}{} {}",
                minus_str,
                money_str,
                self.pluralize(&left, cr1)?
            ));
        }

        let (cents_str, subunit) = if fractional_cents {
            // Python's `isinstance(right, Decimal)` arm.
            let s = if cents {
                // Python: `self.to_cardinal_float(float(right))` — Base's
                // digit-by-digit grammar, pinned by the explicit
                // `cardinal_from_decimal` override above.
                self.cardinal_from_decimal(&right)?
            } else {
                // Python: `str(float(right))`.
                repr_float(&right)?
            };
            // Python: `self.pluralize(right, cr2)` — with `right` a *Decimal*
            // here, not an int. Every arm of pluralize compares `number % 100`
            // or `number % 10` against integers, and Decimal's remainder keeps
            // the fractional part (`Decimal("65.3") % 10 == Decimal("5.3")`), so
            // no arm can ever match: the fractional path always lands on form 2.
            //
            // `right` is provably non-integral whenever this runs. Writing
            // `val = integer + fraction`, `has_fractional_cents` is
            // `(val * 100) % 1 != 0`; `integer * 100` is whole, so that is
            // exactly the statement that `right = fraction * 100` has a
            // fractional part. Confirmed live: 65.3, 21.5, 1.5 and 0.1 all
            // return "centi".
            //
            // `.get(2)` rather than `[2]` keeps Python's IndexError type on a
            // short form list, as in `pluralize` above.
            let sub = cr2
                .get(2)
                .cloned()
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
            (s, sub)
        } else {
            // `right` came back from `parse_currency_parts` as `int(fraction *
            // 100)`, so its scale is 0 and the mantissa is the value.
            let r = right.as_bigint_and_exponent().0;
            // Python spells this branch `if right == 0: to_cardinal(0) if cents
            // else "0"` and only then falls through to `to_cardinal(right) if
            // cents else str(right)`. The two are byte-identical at `right == 0`
            // — `to_cardinal(0) == to_cardinal(0)` and `"0" == str(0)` — so the
            // special case is dead code in the original and is collapsed here.
            // (It exists in Python only to sidestep the isinstance check above.)
            // Python: `self.to_cardinal(right)`, always masculine; the subunit's
            // gender flag is honoured instead ("dvije lipe", #196).
            let s = if cents {
                self.cents_verbose(&r, currency)?
            } else {
                r.to_string()
            };
            (s, self.pluralize(&r, cr2)?)
        };

        // Python: `sep = separator if separator else ","`. HR's own default is
        // the empty string, which is falsy — so both an omitted separator and an
        // explicit "" become a comma, while " i" survives — quirk 10.
        let sep = separator.unwrap_or(self.default_separator());
        let sep = if sep.is_empty() { "," } else { sep };

        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str,
            money_str,
            self.pluralize(&left, cr1)?,
            sep,
            cents_str,
            subunit
        ))
    }
}
