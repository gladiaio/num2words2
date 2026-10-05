//! Port of `lang_LV.py` (Latvian).
//!
//! Registry check: `CONVERTER_CLASSES["lv"]` is `lang_LV.Num2Word_LV`, which is
//! the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_LV` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` table
//! (keys 1..=10) ends at 10^30; `maxval()` is 10^33 and larger values raise
//! `OverflowError`. See bug 4.
//!
//! Inherited from `Num2Word_Base` (unchanged by LV, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`.
//!     Verified: Python returns the *int* unchanged, and the corpus records
//!     its `str()` — including the sign, e.g. `-7` → "-7".
//!   * `to_year(value)        -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below. LV has no
//!     era/apposition handling, so years are plain cardinals: `to_year(-44)`
//!     == "mīnus četrdesmit četri".
//!
//! `setup()` sets `negword = "mīnus"` and `pointword = "komats"`.
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. All of the following are what Python emits,
//! verified against the interpreter:
//!
//! 1. `THOUSANDS[7]` is **"sikstiljons"** — a typo for "sekstiljons". Kept
//!    verbatim; it is corpus-visible (`to_cardinal(10**21)` == "sikstiljons").
//!    `THOUSANDS[10]` is likewise "nontiljons" (rather than "noniljons").
//! 2. `to_ordinal` is, in the module's own words, "a simplified
//!    implementation": outside its 28-entry lookup table it just strips a
//!    trailing "s" from the cardinal and glues on "ais". This produces
//!    non-words for most inputs, e.g. `to_ordinal(0)` == "nulleais",
//!    `to_ordinal(200)` == "divi simtiais", `to_ordinal(2000)` ==
//!    "divi tūkstošiais". All corpus-confirmed. Do not "fix" these.
//! 3. `to_ordinal` accepts negatives (unlike e.g. Polish) because it routes
//!    through `to_cardinal`, which strips the sign before chunking. The suffix
//!    is then appended to the whole phrase: `to_ordinal(-7)` ==
//!    "mīnus septiņiais", `to_ordinal(-1)` == "mīnus vienais". Corpus-confirmed.
//! 4. Fixed (gladiaio/num2words2#159): Python has no `MAXVAL`. `_int2word`
//!    indexes `THOUSANDS[i]` with the chunk index, and the table stops at 10,
//!    so Python's `10**33` raised `KeyError: 11`. `maxval()` is now 10^33 and
//!    `_int2word` raises `OverflowError` for `abs(n) >= 10**33` first, so
//!    [`LangLv::thousands_at`] never misses from the public entry points.
//! 5. `pluralize`'s third form (`forms[2]`, the genitive "tūkstošu"/"miljonu"
//!    column) is **dead code** on the `_int2word` path: `form` is 2 only when
//!    `n == 0`, but `_int2word` `continue`s on zero chunks before ever calling
//!    it. It is very much *alive* on the currency path, though — `0.01 USD` is
//!    "nulle **dolāru**, viens cents", genitive, because `pluralize(0, cr1)`
//!    lands on form 2. Corpus-confirmed.
//! 6. `to_currency`'s integer fast path **bypasses `pluralize` entirely**,
//!    hardcoding `cr1[0]` for `abs(val) == 1` and `cr1[1]` otherwise. That
//!    disagrees with LV's own plural rule in both directions, and the corpus
//!    pins the buggy answers:
//!      * `to_currency(0, "USD")` == "nulle **dolāri**" — `pluralize(0)` would
//!        say "dolāru" (form 2).
//!      * `to_currency(21, "USD")` == "divdesmit viens **dolāri**" —
//!        `pluralize(21)` would say "dolārs" (form 0, since 21 % 10 == 1).
//!    The float path *does* call `pluralize`, so the two paths disagree with
//!    each other. Verified against the interpreter; do not unify them.
//! 7. The same integer fast path silently **ignores the `adjective` kwarg**:
//!    it never consults `CURRENCY_ADJECTIVES`. Verified:
//!    `to_currency(2, "USD", adjective=True)` == "divi dolāri", while the float
//!    `to_currency(2.5, "USD", adjective=True)` == "divi ASV dolāri, piecdesmit
//!    centi". `cents` and `separator` are likewise dead on the integer path.
//!
//! # The float/Decimal cardinal path — and the fractional-cents gap left open
//!
//! `Num2Word_LV.to_cardinal` reads a decimal part as a *whole number*, padding
//! one "nulle" per leading zero (`0.19` → "nulle komats **deviņpadsmit**",
//! `0.3456` → "...trīs tūkstoši četri simti piecdesmit seši"), NOT digit by
//! digit like `Num2Word_Base` (`0.19` → "nulle komats **viens deviņi**"). This
//! is now reproduced by the [`LangLv::to_cardinal_float`] override, so the
//! `cardinal`/`cardinal_dec` corpus rows pass (`12.34` → "...komats trīsdesmit
//! četri", `2.675` → "...komats seši simti septiņdesmit pieci").
//!
//! **Still deliberately left at the trait default**: `cardinal_from_decimal`,
//! the fractional-cents entry point (`Num2Word_Base.to_currency`'s
//! `self.to_cardinal(float(right))` branch). It routes to Base's digit-by-digit
//! `cardinal_from_bigdecimal`, so it is **wrong and quiet** for LV. It coincides
//! with LV's whole-number reading whenever the fraction is leading-zeros plus a
//! single significant digit, and diverges from two significant digits on.
//! Reachable only for currency inputs with more decimals than the currency has
//! (`1.011 USD`, `0.0019 EUR`); no corpus row exercises it — every LV
//! currency/cheque arg has <= 2 decimals — so this is latent, not a live
//! failure, and remains out of scope for this port.
//!
//! # Hundreds spelling rule
//!
//! `HUNDRED` carries three forms — ("simts", "simti", "simtu") — selected by a
//! three-way branch that is easy to misread, so spelled out here:
//!   * `n3 == 1, n2 == 0, n1 > 0` → "simtu" (101 → "simtu viens")
//!   * `n3 > 1`                   → ONES + "simti" (200 → "divi simti")
//!   * otherwise (`n3 == 1`)      → "simts" (100 → "simts"; 110 → "simts
//!     desmit", because `n2 != 0` drops it out of the "simtu" arm)

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::ParsedNumber;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const ZERO: &str = "nulle";
const NEGWORD: &str = "mīnus";
const POINTWORD: &str = "komats";

/// `ONES`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0` /
/// `n3 > 1`).
const ONES: [&str; 10] = [
    "", "viens", "divi", "trīs", "četri", "pieci", "seši", "septiņi", "astoņi", "deviņi",
];

/// `TENS`, keys 0..=9 — the 10..19 teens, keyed by the *units* digit.
const TENS: [&str; 10] = [
    "desmit",
    "vienpadsmit",
    "divpadsmit",
    "trīspadsmit",
    "četrpadsmit",
    "piecpadsmit",
    "sešpadsmit",
    "septiņpadsmit",
    "astoņpadsmit",
    "deviņpadsmit",
];

/// `TWENTIES`, keys 2..=9. Indices 0/1 are absent in Python (guarded `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "divdesmit",
    "trīsdesmit",
    "četrdesmit",
    "piecdesmit",
    "sešdesmit",
    "septiņdesmit",
    "astoņdesmit",
    "deviņdesmit",
];

/// `HUNDRED` — ("simts", "simti", "simtu"). See the module docs for the
/// selection rule.
const HUNDRED: [&str; 3] = ["simts", "simti", "simtu"];

/// `THOUSANDS`: chunk index → (nominative sg, nominative pl, genitive pl).
/// Keys 1..=10, i.e. up to 1000^10 == 10^30 ("nontiljons"). Index 0 is absent
/// in Python and unreachable (guarded by `i > 0`); index >= 11 would be a
/// `KeyError`, pre-empted by the 10^33 MAXVAL check (#159).
///
/// Key 7 ships "sikstiljons"; key 10 ships "nontiljons". Both are Python's
/// spelling — see bug 1 in the module docs.
const THOUSANDS: [[&str; 3]; 11] = [
    ["", "", ""], // absent in Python
    ["tūkstotis", "tūkstoši", "tūkstošu"],
    ["miljons", "miljoni", "miljonu"],
    ["miljards", "miljardi", "miljardu"],
    ["triljons", "triljoni", "triljonu"],
    ["kvadriljons", "kvadriljoni", "kvadriljonu"],
    ["kvintiljons", "kvintiljoni", "kvintiljonu"],
    ["sikstiljons", "sikstiljoni", "sikstiljonu"], // sic — Python typo
    ["septiljons", "septiljoni", "septiljonu"],
    ["oktiljons", "oktiljoni", "oktiljonu"],
    ["nontiljons", "nontiljoni", "nontiljonu"], // sic — Python spelling
];

/// The `ordinals` dict literal in `Num2Word_LV.to_ordinal`, in source order.
///
/// Sparse: 1..=20, then the round tens 30..=90, then 100 and 1000 — 29 entries.
/// Anything else falls through to the "strip s, append ais" fallback. Note the
/// absence of 0 — hence `to_ordinal(0)` == "nulleais" (bug 2).
const ORDINALS: [(i64, &str); 29] = [
    (1, "pirmais"),
    (2, "otrais"),
    (3, "trešais"),
    (4, "ceturtais"),
    (5, "piektais"),
    (6, "sestais"),
    (7, "septītais"),
    (8, "astotais"),
    (9, "devītais"),
    (10, "desmitais"),
    (11, "vienpadsmitais"),
    (12, "divpadsmitais"),
    (13, "trīspadsmitais"),
    (14, "četrpadsmitais"),
    (15, "piecpadsmitais"),
    (16, "sešpadsmitais"),
    (17, "septiņpadsmitais"),
    (18, "astoņpadsmitais"),
    (19, "deviņpadsmitais"),
    (20, "divdesmitais"),
    (30, "trīsdesmitais"),
    (40, "četrdesmitais"),
    (50, "piecdesmitais"),
    (60, "sešdesmitais"),
    (70, "septiņdesmitais"),
    (80, "astoņdesmitais"),
    (90, "deviņdesmitais"),
    (100, "simtais"),
    (1000, "tūkstošais"),
];

// --- Currency ------------------------------------------------------------
//
// `Num2Word_LV` subclasses `Num2Word_Base`, **not** `Num2Word_EUR`, and
// declares its own `CURRENCY_FORMS` in its class body. So the shared-mutable
// -EUR-dict trap does not apply here: `Num2Word_EN.__init__`'s in-place writes
// land on `Num2Word_EUR.CURRENCY_FORMS`, a different object. Confirmed against
// the live interpreter — LV's runtime table is exactly the source literal, 13
// entries, with none of EN's ~24 additions. Hence no JPY, KWD, BHD, INR, CNY
// or CHF, all of which raise NotImplementedError (corpus-confirmed).
//
// The module-level tuples from `lang_LV.py`, shared by several codes.
const GENERIC_DOLLARS: [&str; 3] = ["dolārs", "dolāri", "dolāru"];
const GENERIC_CENTS: [&str; 3] = ["cents", "centi", "centu"];
const GENERIC_KRONA: [&str; 3] = ["krona", "kronas", "kronu"];
const GENERIC_ERA: [&str; 3] = ["ēre", "ēras", "ēru"];

/// `Num2Word_LV.CURRENCY_FORMS`, in source order.
///
/// Every entry carries **three** forms on both sides, and all three are load
/// bearing: `pluralize` indexes 0/1/2, and `to_cheque` takes `cr1[-1]` — so
/// `cheque:USD` is "...DOLĀRU", the genitive. Dropping the third form would
/// silently change both.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AUD", CurrencyForms::new(&GENERIC_DOLLARS, &GENERIC_CENTS));
    m.insert("CAD", CurrencyForms::new(&GENERIC_DOLLARS, &GENERIC_CENTS));
    // replaced by EUR
    m.insert("EEK", CurrencyForms::new(&GENERIC_KRONA, &GENERIC_CENTS));
    m.insert("EUR", CurrencyForms::new(&["eiro", "eiro", "eiro"], &GENERIC_CENTS));
    // The legal/finance spelling mandated for LV EU documents — see the class
    // docstring's citations. A distinct code, not a variant of EUR.
    m.insert(
        "EUR_LEGAL",
        CurrencyForms::new(&["euro", "euro", "euro"], &GENERIC_CENTS),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(
            &["sterliņu mārciņa", "sterliņu mārciņas", "sterliņu mārciņu"],
            &["penss", "pensi", "pensu"],
        ),
    );
    // replaced by EUR
    m.insert("LTL", CurrencyForms::new(&["lits", "liti", "litu"], &GENERIC_CENTS));
    // replaced by EUR
    m.insert(
        "LVL",
        CurrencyForms::new(&["lats", "lati", "latu"], &["santīms", "santīmi", "santīmu"]),
    );
    m.insert("USD", CurrencyForms::new(&GENERIC_DOLLARS, &GENERIC_CENTS));
    m.insert(
        "RUB",
        CurrencyForms::new(&["rublis", "rubļi", "rubļu"], &["kapeika", "kapeikas", "kapeiku"]),
    );
    m.insert("SEK", CurrencyForms::new(&GENERIC_KRONA, &GENERIC_ERA));
    m.insert("NOK", CurrencyForms::new(&GENERIC_KRONA, &GENERIC_ERA));
    m.insert(
        "PLN",
        CurrencyForms::new(&["zlots", "zloti", "zlotu"], &["grasis", "graši", "grašu"]),
    );
    m
}

/// `Num2Word_LV.CURRENCY_ADJECTIVES`.
///
/// "Kreivijas" (RUB) is Python's spelling — a typo for "Krievijas". Kept
/// verbatim. Reachable only from the float path with `adjective=True`; the
/// integer path never reads this table at all (bug 7).
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Austrālijas"),
        ("CAD", "Kanādas"),
        ("EEK", "Igaunijas"),
        ("USD", "ASV"),
        ("RUB", "Kreivijas"), // sic — Python typo for "Krievijas"
        ("SEK", "Zviedrijas"),
        ("NOK", "Norvēģijas"),
    ]
    .into_iter()
    .collect()
}

// LV defines no `CURRENCY_PRECISION`, so it inherits `Num2Word_Base`'s empty
// `{}` and every code resolves to `.get(code, 100)` == 100. The trait's default
// already returns 100, so `currency_precision` is deliberately **not**
// overridden. Consequently LV has neither a 0-decimal nor a 3-decimal currency:
// the `divisor == 1` and `divisor == 1000` branches are both unreachable here,
// and JPY/KWD/BHD raise NotImplementedError long before precision matters.

// --- Python exception encoding -------------------------------------------
//
// This is a crash site, not a deliberate raise: `THOUSANDS[i]` on a missing
// key. Unreachable from the public entry points since the 10^33 MAXVAL check
// (bug 4, #159), which raises OverflowError first.
fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
///
/// Unlike Polish, LV only ever reaches this with a **non-negative** value:
/// `to_cardinal` strips the sign before calling `_int2word`, and `to_ordinal`
/// routes through `to_cardinal`. So `str(n)` is pure digits and the `int()`
/// call cannot raise — the `Result` here is defensive, never `Err` in practice.
fn splitbyx(n: &str, x: usize) -> Result<Vec<BigInt>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let slice = |i: usize, j: usize| -> String { chars[i..j.min(length)].iter().collect() };

    let parse = |s: String| -> Result<BigInt> {
        BigInt::parse_bytes(s.as_bytes(), 10)
            .ok_or_else(|| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
    };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse(slice(0, start))?);
        }
        let mut i = start;
        while i < length {
            out.push(parse(slice(i, i + x))?);
            i += x;
        }
    } else {
        out.push(parse(n.to_string())?);
    }
    Ok(out)
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Only ever called with a chunk in 0..=999 (see [`splitbyx`]), so the
/// zero-padded form is exactly 3 chars and the `[-3:]` slice is the whole
/// string. No sign hazard here.
fn get_digits(n: &BigInt) -> [usize; 3] {
    let s = format!("{:03}", n);
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).unwrap_or(0) as usize;
    }
    a
}

pub struct LangLv {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangLv {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLv {
    pub fn new() -> Self {
        LangLv {
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // ever read these tables; rebuilding them on every call is what
            // made an earlier revision of this port slower than the Python it
            // replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `THOUSANDS[i]`, raising `KeyError` past 10 (unreachable). See bug 4 in
    /// the module docs.
    fn thousands_at(&self, i: usize) -> Result<&'static [&'static str; 3]> {
        if (1..=10).contains(&i) {
            Ok(&THOUSANDS[i])
        } else {
            Err(key_error(i.to_string()))
        }
    }

    /// Port of `Num2Word_LV.pluralize`.
    ///
    /// ```python
    /// form = 0 if (n % 10 == 1 and n % 100 != 11) else 1 if n != 0 else 2
    /// ```
    /// `n` is a 3-digit chunk and always non-negative here, but `mod_floor` is
    /// used anyway to keep Python's `%` semantics rather than Rust's `%`.
    /// Form 2 is unreachable from `_int2word` — see bug 5 in the module docs.
    fn pluralize(&self, n: &BigInt, forms: &[&str; 3]) -> String {
        let m10 = n.mod_floor(&BigInt::from(10));
        let m100 = n.mod_floor(&BigInt::from(100));
        let form = if m10.is_one() && m100 != BigInt::from(11) {
            0usize
        } else if !n.is_zero() {
            1
        } else {
            2
        };
        forms[form].to_string()
    }

    /// Port of `Num2Word_LV._int2word`. Called only with non-negative values.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3)?;
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                if n3 == 1 && n2 == 0 && n1 > 0 {
                    words.push(HUNDRED[2].to_string());
                } else if n3 > 1 {
                    words.push(ONES[n3].to_string());
                    words.push(HUNDRED[1].to_string());
                } else {
                    words.push(HUNDRED[0].to_string());
                }
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1].to_string());
            } else if n1 > 0 && !(i > 0 && x.is_one()) {
                // `x == 1` (the whole chunk, not just the units digit) is what
                // suppresses "viens" in "tūkstotis" / "miljons". 21000 keeps it
                // ("divdesmit viens tūkstotis") because x == 21, not 1.
                words.push(ONES[n1].to_string());
            }

            if i > 0 {
                let forms = self.thousands_at(i)?;
                words.push(self.pluralize(x, forms));
            }
        }

        Ok(words.join(" "))
    }
}

/// `int(number)`'s truncation toward zero, for the finite float and Decimal
/// arms. `None` only for a non-finite float (Python's `int()` raises there —
/// see the callers for which exception).
fn trunc_to_bigint(v: &FloatValue) -> Option<BigInt> {
    match v {
        FloatValue::Float { value, .. } => {
            if !value.is_finite() {
                return None;
            }
            BigInt::from_f64(value.trunc())
        }
        FloatValue::Decimal { value, .. } => {
            // `with_scale(0)` truncates toward zero — exactly `int(Decimal)`.
            Some(value.with_scale(0).as_bigint_and_exponent().0)
        }
    }
}

/// `int(n)`'s ValueError for the no-`"."` branch of `to_cardinal`, reached
/// when `str(number)` came out in exponent form (`1e+16`, `1E+2`) or as
/// `inf`/`nan`/`Infinity`. Message shape matches CPython's; the corpus checks
/// exception types.
fn int_value_error(literal: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        literal
    ))
}

/// `str(number)` for a float carrying no visible point: exponent form for
/// finite values, `inf`/`nan` otherwise. Only feeds the ValueError message.
fn float_no_point_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let s = format!("{:e}", f);
    match s.split_once('e') {
        Some((m, e)) if !e.starts_with('-') => format!("{}e+{:0>2}", m, e),
        Some((m, e)) => format!("{}e-{:0>2}", m, &e[1..]),
        None => s,
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangLv {
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
        ","
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "komats"
    }

    /// Port of `Num2Word_LV.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input, swaps "," → "." and looks for "."; a
    /// `str(int)` contains neither, so integers always take the `else` branch:
    /// `"%s%s" % (base_str, self._int2word(int(n)))`. The float branch
    /// (`pointword`, leading-zero "nulle" padding) is out of scope.
    ///
    /// `parse_minus` returns `"%s " % self.negword.strip()` — i.e. "mīnus "
    /// *with* the trailing space — which is why the two `%s` concatenate
    /// directly with no separator.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let words = self.int2word(&value.abs())?;
            Ok(format!("{} {}", NEGWORD, words))
        } else {
            self.int2word(value)
        }
    }

    /// Port of `Num2Word_LV.to_ordinal`.
    ///
    /// The `try: int(number) except (ValueError, TypeError): return str(number)`
    /// guard is unreachable for integer input and is not modelled.
    ///
    /// Outside the `ordinals` table this is the module's self-described
    /// "simplified implementation": strip one trailing "s" from the cardinal,
    /// then append "ais" — see bug 2 in the module docs.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // `if num in ordinals` — the table is keyed by exact int value. A value
        // too large for i64 simply cannot be in a table topping out at 1000.
        if let Some(k) = value.to_i64() {
            if let Some((_, word)) = ORDINALS.iter().find(|(key, _)| *key == k) {
                return Ok(word.to_string());
            }
        }

        let cardinal = self.to_cardinal(value)?;
        // Python: `cardinal[:-1] + "ais"` — drops exactly one character, which
        // is the ASCII "s" we just matched, so this is char-safe.
        match cardinal.strip_suffix('s') {
            Some(stem) => Ok(format!("{}ais", stem)),
            None => Ok(format!("{}ais", cardinal)),
        }
    }

    /// Port of the **float/Decimal branch** of `Num2Word_LV.to_cardinal`.
    ///
    /// LV overrides Python's `to_cardinal` (not `to_cardinal_float`) and reads
    /// `str(number)` directly:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// base_str, n = self.parse_minus(n)
    /// if "." in n:
    ///     left, right = n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count + self._int2word(int(right))
    ///     return "%s%s %s %s" % (base_str, self._int2word(int(left)), self.pointword, decimal_part)
    /// else:
    ///     return "%s%s" % (base_str, self._int2word(int(n)))
    /// ```
    ///
    /// This reads the fractional part as **one whole number** with a "nulle"
    /// prefix per leading zero (`12.34` → "...komats **trīsdesmit četri**"),
    /// unlike `Num2Word_Base`'s digit-by-digit reading (which would give
    /// "...komats trīs četri"). That is why this override exists — the Base
    /// default this file previously inherited was wrong for LV.
    ///
    /// `float2tuple` (Base's binary/Decimal arithmetic) reproduces `int(left)`
    /// as `pre` and `int(right)` as `post`, artefacts included: `2.675` floors
    /// to `674.9999…` and the `< 0.01` heuristic rescues it to `675`, matching
    /// `str(2.675) == "2.675"`. Padding `post` to `precision` chars reconstructs
    /// the exact `right` substring `str(number)` would have produced.
    ///
    /// `precision_override` is **dropped on purpose**: LV's reader takes the
    /// digits from `str(number)`, so the `precision=` kwarg has no effect on it.
    /// Verified live: `num2words(1.2345, lang='lv', precision=2)` still yields
    /// the full "…divi tūkstoši trīs simti četrdesmit pieci".
    ///
    /// Sign follows `value.is_negative()`, standing in for Python's
    /// `str(number).startswith("-")`. These agree except for negative zero
    /// (`str(-0.0) == "-0.0"` starts with "-" but `-0.0 < 0.0` is false) — an
    /// input no corpus row exercises; see concerns.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let precision = value.precision() as usize;

        // `base_str` = parse_minus's `"%s " % negword.strip()` (= "mīnus ") or "".
        let base_str = if value.is_negative() {
            format!("{} ", NEGWORD)
        } else {
            String::new()
        };

        // (pre, post) == (int(value), int(right)); pre is signed, but `left` is
        // the abs string in Python, so `int(left)` == abs(pre).
        let (pre, post) = float2tuple(value);
        let left_word = self.int2word(&pre.abs())?;

        // `"." not in n` — only integer-scale Decimals (e.g. Decimal("5")); the
        // else branch is `"%s%s" % (base_str, _int2word(int(n)))`.
        if precision == 0 {
            return Ok(format!("{}{}", base_str, left_word));
        }

        // right = post zero-padded to `precision` chars (== str(number)'s
        // fractional substring). leading_zero_count counts its leading zeros;
        // int(right) == post with those zeros dropped.
        let post_str = post.to_string();
        let right = format!(
            "{}{}",
            "0".repeat(precision.saturating_sub(post_str.len())),
            post_str
        );
        let leading_zero_count = right.len() - right.trim_start_matches('0').len();

        let mut decimal_part = String::new();
        for _ in 0..leading_zero_count {
            decimal_part.push_str(ZERO);
            decimal_part.push(' ');
        }
        decimal_part.push_str(&self.int2word(&post)?);

        // `"%s%s %s %s" % (base_str, left, pointword, decimal_part)`. LV uses the
        // raw `self.pointword` here (no title()); is_title is false regardless.
        Ok(format!(
            "{}{} {} {}",
            base_str, left_word, POINTWORD, decimal_part
        ))
    }

    // cardinal_from_decimal (the fractional-cents currency entry point) is left
    // at its trait default — a separate, latent gap documented in the module
    // header, and out of scope for this port.

    /// Full `to_cardinal(float/Decimal)` routing — Python's gate is
    /// `"." in str(number)`, NOT the base default's `int(value) == value`:
    ///
    /// * a **visible point** (any finite float below 1e16, or a Decimal with
    ///   positive scale) takes the fractional branch even for whole values —
    ///   `5.0` -> "pieci komats nulle nulle" (one "nulle" per leading zero of
    ///   the fractional string plus `_int2word(0)`), `Decimal("5.00")` ->
    ///   three "nulle".
    /// * **no point** funnels the whole string into `int(n)`: plain digit
    ///   Decimals reach the integer path, while exponent forms (`str(1e16) ==
    ///   "1e+16"`, `str(Decimal("1E+2")) == "1E+2"`) and inf/nan raise
    ///   **ValueError**, not the base default's OverflowError.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        match value {
            FloatValue::Float { value, .. } => Err(int_value_error(&float_no_point_str(*value))),
            FloatValue::Decimal { value, .. } => {
                let s = crate::strnum::python_decimal_str(value);
                match crate::strnum::python_int_parse(&s) {
                    Some(i) => self.to_cardinal(&i),
                    None => Err(int_value_error(&s)),
                }
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python:
    ///
    /// ```python
    /// try: num = int(number)
    /// except (ValueError, TypeError): return str(number)
    /// ```
    ///
    /// `int()` **truncates**: `to_ordinal(2.5)` == `to_ordinal(2)` ==
    /// "otrais", `to_ordinal(-0.0)` == "nulleais" via the table-miss suffix
    /// path (int(-0.0) is 0, sign gone), and `to_ordinal(1e16)` succeeds —
    /// the ordinal table + "ais" suffix run on the truncated integer.
    /// `int(nan)` raises ValueError, which the except arm converts to
    /// `str(number)` == "nan"; `int(inf)` raises OverflowError, which is
    /// *not* in the except tuple and propagates.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let FloatValue::Float { value: f, .. } = value {
            if f.is_nan() {
                return Ok("nan".to_string());
            }
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
        }
        let num = trunc_to_bigint(value).expect("finite after the guards above");
        self.to_ordinal(&num)
    }

    /// `converter.str_to_number` is Base's `Decimal(value)` (LV doesn't
    /// override it), but `Decimal("Infinity")` then hits LV's `to_cardinal`,
    /// where `str(number)` has no "." and `int("Infinity")` raises
    /// **ValueError** — not the OverflowError the binding's generic Inf arm
    /// would produce. NaN already maps to ValueError there.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(int_value_error(if negative {
                "-Infinity"
            } else {
                "Infinity"
            })),
            other => Ok(other),
        }
    }

    // to_ordinal_num: LV does not override Num2Word_Base.to_ordinal_num, which
    // returns the value unchanged → the trait default (`value.to_string()`) is
    // correct, sign included.
    //
    // to_year: LV does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal → the trait default is correct; year_float_entry likewise
    // stays at the default, whose delegation to cardinal_float_entry now
    // carries LV's own str-based routing.

    // ---- currency -------------------------------------------------------
    //
    // LV overrides `to_currency` (integer fast path) and `pluralize`, and
    // supplies its own `CURRENCY_FORMS`/`CURRENCY_ADJECTIVES`. Everything else
    // — `to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse`,
    // `CURRENCY_PRECISION` — is inherited from `Num2Word_Base` unchanged, so
    // the trait defaults already mirror it and are left alone.

    fn lang_name(&self) -> &str {
        "Num2Word_LV"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// Port of `Num2Word_LV.pluralize` for the **currency** path.
    ///
    /// ```python
    /// form = 0 if (n % 10 == 1 and n % 100 != 11) else 1 if n != 0 else 2
    /// return forms[form]
    /// ```
    ///
    /// Same rule as the inherent [`LangLv::pluralize`] that `_int2word` uses —
    /// Python has one method serving both — but the trait hands forms in as
    /// `&[String]` rather than the `&[&str; 3]` the verified integer path
    /// carries, so the branch is restated here rather than reworking that
    /// path's signature. Inherent methods win name resolution, so `_int2word`
    /// keeps calling its own copy and is unaffected.
    ///
    /// Unlike `_int2word`, this path reaches `n == 0` (form 2, the genitive):
    /// `0.01 USD` → "nulle **dolāru**, viens cents".
    ///
    /// Python indexes the tuple directly, so fewer than three forms with
    /// `form == 2` would raise IndexError. Every LV entry has three, so this is
    /// unreachable — mapped to `Index` rather than panicking in case the table
    /// ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let m10 = n.mod_floor(&BigInt::from(10));
        let m100 = n.mod_floor(&BigInt::from(100));
        let form = if m10.is_one() && m100 != BigInt::from(11) {
            0usize
        } else if !n.is_zero() {
            1
        } else {
            2
        };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_LV.to_currency`.
    ///
    /// Two paths, and they do **not** agree with each other:
    ///
    /// * `isinstance(val, int)` → LV's own fast path, which skips `pluralize`,
    ///   `adjective`, `cents` and `separator` (bugs 6 and 7).
    /// * anything else → `super().to_currency(...)`, i.e. `Num2Word_Base`'s
    ///   float path, which honours all of them.
    ///
    /// The int/non-int split is the same one `CurrencyValue` encodes, so it
    /// maps across directly — `1` takes the fast path, `1.0` does not.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        if let CurrencyValue::Int(v) = val {
            // `try: cr1, cr2 = self.CURRENCY_FORMS[currency]`, guarded by
            // `except (KeyError, AttributeError)` whose handler re-enters
            // `Num2Word_Base.to_currency` and lets *that* raise the
            // NotImplementedError. Delegating reproduces the exception type and
            // its exact message rather than restating them here. (The
            // AttributeError arm is dead: `CURRENCY_FORMS` always exists.)
            let cr1 = match self.currency_forms.get(currency) {
                Some(forms) => &forms.unit,
                None => {
                    return crate::currency::default_to_currency(
                        self,
                        val,
                        currency,
                        cents,
                        separator.unwrap_or(self.default_separator()),
                        adjective,
                    );
                }
            };

            // `minus_str = self.negword if val < 0 else ""` — LV takes the raw
            // negword ("mīnus", no trailing space) where Base takes
            // `"%s " % negword.strip()`. Same output, because the format string
            // below spaces the three fields itself and `.strip()` eats the
            // leading space left when minus_str is empty.
            let minus_str = if v.is_negative() { NEGWORD } else { "" };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            // Bug 6: `cr1[0]` / `cr1[1]` hardcoded instead of `pluralize`.
            // The `isinstance(cr1, tuple)` guards Python wraps these in are
            // always true — CurrencyForms is always a sequence — so only the
            // `len(cr1) > 1` fallback survives the translation.
            let currency_str = if abs_val.is_one() {
                cr1.first()
            } else {
                cr1.get(1).or_else(|| cr1.first())
            }
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            // `("%s %s %s" % (minus_str, money_str, currency_str)).strip()`
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats fall through to Num2Word_Base.to_currency unchanged.
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }
}

