//! Port of `lang_ES_CO.py` (Colombian Spanish), via its full ancestry
//! `Num2Word_ES_CO` → `Num2Word_ES` → `Num2Word_EUR` → `Num2Word_Base`.
//!
//! The registry key `"es_CO"` resolves to `Num2Word_ES_CO`, which overrides
//! **only** `CURRENCY_FORMS` and `to_currency`. Every one of the four integer
//! modes therefore comes from `Num2Word_ES` verbatim, and that half of this
//! file is effectively a port of `lang_ES.py`'s cardinal/ordinal engine; the
//! currency surface at the bottom is where ES_CO's own two overrides live.
//!
//! Shape: **engine**. `Num2Word_ES.setup` supplies `high_numwords` /
//! `mid_numwords` / `low_numwords` and overrides `merge`, so `self.cards` is
//! built and `Num2Word_Base.to_cardinal` drives `splitnum`/`clean`/`merge`.
//!
//! # Card table construction
//!
//! `setup` calls `gen_high_numwords([], [], lows)`. With empty `units` and
//! `tens` the comprehension `[u + t for t in tens for u in units]` yields the
//! empty list, so the Latin-prefix elision machinery is a no-op and
//! `high_numwords` is just `lows` — nine entries. `Num2Word_EUR`'s
//! `set_high_numwords` then runs with `cap = 3 + 6*9 = 57` over
//! `zip(high, range(57, 3, -6))`. ES sets `GIGA_SUFFIX = None`, so the
//! `illiard` half is skipped entirely and only `MEGA_SUFFIX = "illón"` cards
//! land, at `10**(n-3)`:
//!
//! ```text
//! 10^54 nonillón   10^48 octillón   10^42 septillón  10^36 sextillón
//! 10^30 quintillón 10^24 cuatrillón 10^18 trillón    10^12 billón
//! 10^6  millón
//! ```
//!
//! This is a **long scale** with no intermediate `mil-` cards: 10^9 is not a
//! card, so a thousand million is spelled "mil millones" by `merge`. The
//! highest card is 10^54, hence `MAXVAL = 1000 * 10^54 = 10^57` — verified
//! against the interpreter.
//!
//! Python's `cards` is an `OrderedDict` in insertion order (high, then mid,
//! then low); for Spanish that order is strictly descending, so the sorted
//! [`Cards`] table iterates identically and `MAXVAL`'s `list(keys())[0]`
//! is the same as `highest()`.
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the interpreter and the frozen corpus, and are preserved verbatim:
//!
//! 1. **`to_ordinal(20)` == "vigesimo"** — unaccented. The `value <= 29` arm
//!    applies `.replace("é", "e")` to build "decimo-"/"vigesimo-" prefixes,
//!    and it fires even when the unit digit is 0, so 20 loses its accent while
//!    30 ("trigésimo") keeps it. Likewise 120 → "centésimo vigesimo".
//! 2. **The ordinal scale disagrees with the cardinal scale.** `ords` maps
//!    1e9 → "billonésim" and 1e12 → "trillonésim", but the *cards* call 10^12
//!    "billón" and 10^18 "trillón". So `to_cardinal(10**9)` == "mil millones"
//!    while `to_ordinal(10**9)` == "billonésimo", and `to_ordinal(10**12)` ==
//!    "trillonésimo" though `to_cardinal(10**12)` == "un billón". Not a
//!    transcription error here — that is what Python emits.
//! 3. **No separator between the high cardinal and the ordinal stem** in the
//!    `value < 1e18` arm: `"%s%s%s %s" % (cardinal, ords[dec], ...)` glues
//!    them, giving "dosmilésimo" (2000), "noventa y nuevemilésimo" (99999),
//!    "ciento veintitrésmilésimo cuadrigentésimo quincuagésimo sexto"
//!    (123456), and "nuevemillonésimo" (9999999).
//! 4. **`.replace("oo", "o")` runs at every recursion level** — the documented
//!    "decimoctavo" fix (18 → "decimooctavo" → "decimoctavo"). Rust's
//!    `str::replace` is non-overlapping and left-to-right, matching Python.
//! 5. **`ords` is keyed by `1e3`/`1e6`/… floats** yet indexed with `int`s.
//!    Python's numeric hashing makes `ords[1000]` find the `1e3` entry, so
//!    `to_ordinal(1000)` == "milésimo" rather than a `KeyError`. Modelled here
//!    with `u64` keys, which collapses the int/float distinction the same way.
//! 6. **`to_ordinal(0)`** raises ValueError here (Python returns `""`; #160).
//! 7. **`errmsg_toobig`** ships the typo "deber ser inferior" (for "debe ser
//!    inferior"); reproduced verbatim in the `Overflow` message.
//!
//! # Crash sites reproduced (see the `concerns` in the port report)
//!
//! * **`KeyError`** for `to_ordinal(v)` with `v` in `[10^18 - 4928, 10^18)`.
//!   The bucket is `dec = 1000 ** int(math.log(int(value), 1000))`; because
//!   `float(10**18 - 1) == 1e18` (10^18 is far past 2^53, ulp is 128) the log
//!   rounds up to exactly `6.0`, so `dec` becomes 10^18 — a key `ords` does
//!   not have. Python evaluates `self.ords[dec]` *before* the recursive
//!   `to_ordinal(low_part)` when building the `%`-format tuple, so the
//!   `KeyError` wins over the recursion. [`LangEsCo::ordinal_g`] preserves
//!   that ordering. Verified: `to_ordinal(10**18 - 1)` raises
//!   `KeyError: 1000000000000000000`.
//! * **`RecursionError`**, rooted at exactly four inputs — 10^15 minus 1, 2, 3
//!   or 4. There the log rounds up to `5.0`, so `dec = 10^15 > value`, making
//!   `divmod` yield `high_part = 0, low_part = value`; `ords[1e15]` *does*
//!   exist, so no `KeyError` intercepts it and `to_ordinal(low_part)` recurses
//!   on its own argument forever. `N2WError` has no `Recursion` variant, and
//!   letting Rust recur would abort the process on stack overflow rather than
//!   return — so `dec > value` is guarded and surfaced as [`N2WError::Value`].
//!   **That variant is a stand-in and does not match Python's
//!   `RecursionError`**; no corpus row covers any affected value.
//!
//!   The poisoned set is wider than those four roots: any `value` in
//!   [10^15, 10^18) whose `value % 10^15` lands on a root recurses too, via the
//!   nested `to_ordinal(low_part)` call — e.g. `to_ordinal(10**16 - 1)` splits
//!   into `high_part = 9, low_part = 10**15 - 1` and dies inside the nested
//!   call. Those need no extra guard here: the root's `Err` propagates back out
//!   through `?` exactly as Python's exception unwinds through its frames. A
//!   157k-pair differential fuzz against the interpreter confirms the two
//!   implementations fail on precisely the same inputs.
//!
//! # The float/Decimal cardinal path is the base default — deliberately not
//! overridden here
//!
//! `Num2Word_ES_CO` overrides neither `to_cardinal_float` nor `float2tuple`
//! (verified live: both resolve to `Num2Word_Base`). Its `to_cardinal`
//! override — inherited from `Num2Word_ES` — handles a *non-integer* argument
//! only by delegating to `super().to_cardinal`, i.e. `Num2Word_Base.to_cardinal`,
//! whose `assert int(value) == value` sends whole-number floats down the
//! integer path and everything else to the unmodified base `to_cardinal_float`.
//! So the entire fractional rendering for `es_CO` is the base float path
//! verbatim, and [`LangEsCo`] intentionally leaves the trait's default
//! `to_cardinal_float` (→ `floatpath::default_to_cardinal_float`) in place.
//!
//! This was not assumed — it was measured against the live interpreter's own
//! `to_cardinal_float`: **zero** mismatches over 6042 floats and 3023 Decimals,
//! plus all 24 `es_CO` `cardinal`/`cardinal_dec` corpus rows byte-for-byte.
//! The pointword is `"punto"`, `is_title` is false, and the digit/`pre`
//! renderings route back through this file's own `to_cardinal` (overflow-typo
//! message and all), so nothing about the override needs re-teaching to the
//! float path.
//!
//! The corpus rows `0.0 → "cero"` and `1.0 → "uno"` come from that integer
//! dispatch, *not* from `to_cardinal_float`: called directly, Python's
//! `to_cardinal_float(0.0)` yields `"cero punto cero"`, exactly as the Rust
//! default does — the whole-float shortcut lives in the caller, and the
//! parity harness must reproduce it (route `int(v) == v` to `to_cardinal`)
//! just as it must for the 26 other base-inheriting languages.
//!
//! One divergence exists and is out of scope here: the `precision=` /
//! `precision_override` kwarg. Python's `float2tuple` unconditionally rewrites
//! `self.precision` from `repr(value)` *after* `to_cardinal_float` sets it, so
//! the kwarg is a silent no-op in the interpreter; `default_to_cardinal_float`
//! instead honours the override. That difference is entirely inside
//! `floatpath.rs`, shared by every base-inheriting language, untested by the
//! corpus, and off-limits from this file — see the port report's `concerns`.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError,
    Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `Num2Word_ES.setup` and never reassigned.
///
/// `to_ordinal`/`to_ordinal_num` shadow it with a *local* `gender_stem`, so
/// the instance attribute stays "o" for `merge`'s whole lifetime.
const GENDER_STEM: &str = "o";

/// Python's `int(math.log(int(value), 1000))`.
///
/// CPython's `loghelper` converts an int argument with `PyLong_AsDouble` and
/// only falls back to the `frexp` path when *that* overflows (|x| > ~1.8e308).
/// Every value reaching here is < 10^18, so this is exactly
/// `log(float(value)) / log(1000.0)` in double precision — including the
/// rounding that makes `float(10**18 - 1) == 1e18` and pushes the quotient up
/// to `6.0`. Computing an exact integer log instead would be *more* correct
/// and *less* faithful, so the float path is kept deliberately.
fn log_bucket(value: &BigInt) -> i32 {
    let x = value.to_f64().unwrap_or(f64::INFINITY);
    (x.ln() / 1000f64.ln()).trunc() as i32
}

fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

/// `verify_ordinal`'s float leg. Python evaluates `int(value)` *first*
/// (inside `value == int(value)`), so NaN raises ValueError and ±inf raises
/// OverflowError before any TypeError; a finite fractional value fails the
/// comparison and raises TypeError with ES's `errmsg_floatord`.
fn float_ordinal_reject(value: &FloatValue) -> N2WError {
    if let FloatValue::Float { value: f, .. } = value {
        if f.is_nan() {
            return N2WError::Value("cannot convert float NaN to integer".into());
        }
        if f.is_infinite() {
            return N2WError::Overflow("cannot convert float infinity to integer".into());
        }
    }
    let shown = match value {
        FloatValue::Float { value: f, .. } => format!("{}", f),
        FloatValue::Decimal { value: d, .. } => crate::strnum::python_decimal_str(d),
    };
    N2WError::Type(format!(
        "El float {} no puede ser tratado como un ordinal.",
        shown
    ))
}

/// Python's `int(val)` on a float/Decimal: truncation toward zero (so
/// `int(-1.5) == -1`, `int(-0.0) == 0`). NaN/inf raise what CPython raises.
fn float_trunc_int(value: &FloatValue) -> Result<BigInt> {
    match value {
        FloatValue::Float { value: f, .. } => {
            if f.is_nan() {
                return Err(N2WError::Value(
                    "cannot convert float NaN to integer".into(),
                ));
            }
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
            // Exact: every whole f64 is exactly representable as an integer.
            Ok(BigInt::from_f64(f.trunc()).expect("finite whole f64 converts exactly"))
        }
        FloatValue::Decimal { value: d, .. } => {
            // int(Decimal): with_scale(0) divides by 10^scale in BigInt,
            // which truncates toward zero — exactly Python's int().
            Ok(d.with_scale(0).as_bigint_and_exponent().0)
        }
    }
}

/// `Num2Word_ES_CO.CURRENCY_FORMS` — three codes, and that is the whole table.
///
/// ES_CO *rebinds* `CURRENCY_FORMS` in its class body rather than extending the
/// inherited one, so `Num2Word_ES`'s ~150 codes are invisible here: GBP, JPY,
/// KWD, BHD, INR, CNY and CHF all raise NotImplementedError for `es_CO` even
/// though plain `es` renders them. The corpus is unambiguous about this — every
/// non-{COP,EUR,USD} row is an error row.
///
/// The `lang_EUR.py` mutation trap does not reach this table either: shadowing
/// the attribute at the `Num2Word_ES` level already broke the chain, so
/// `Num2Word_EN.__init__`'s in-place writes into the shared `Num2Word_EUR` dict
/// land somewhere this class never looks. Confirmed against the live
/// interpreter — `Num2Word_ES_CO().CURRENCY_FORMS` is neither ES's object nor
/// EUR's, and EUR's own `("euro", "euro")` never appears: ES_CO spells the
/// plural itself.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTAVOS: [&str; 2] = ["centavo", "centavos"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("COP", CurrencyForms::new(&["peso", "pesos"], &CENTAVOS));
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "euros"], &["céntimo", "céntimos"]),
    );
    m.insert("USD", CurrencyForms::new(&["dólar", "dólares"], &CENTAVOS));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited untouched — neither ES nor
/// ES_CO defines one, and nothing mutates it in place (verified: the live
/// instance's dict *is* `Num2Word_EUR`'s object, with EUR's 16 entries).
///
/// Only USD overlaps [`build_currency_forms`], so USD is the single code that
/// can actually reach the adjective branch: every other key here raises on the
/// forms lookup first. Kept whole anyway — it is the inherited data, and
/// trimming it to the reachable entry would encode a conclusion rather than the
/// port.
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

pub struct LangEsCo {
    cards: Cards,
    maxval: BigInt,
    /// `Num2Word_ES.ords`. Python keys 1e3..1e15 are floats but are only ever
    /// looked up with ints; numeric hashing makes the two equivalent, so `u64`
    /// keys reproduce the lookups exactly. Every key fits (max 10^15, and the
    /// out-of-range 10^18 probe fits too, so it can miss rather than panic).
    ords: HashMap<u64, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEsCo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsCo {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_ES.setup: high_numwords = gen_high_numwords([], [], lows).
        // Empty units/tens => the comprehension is empty => result is `lows`.
        let high = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        // Num2Word_EUR.set_high_numwords, with GIGA_SUFFIX = None (skipped)
        // and MEGA_SUFFIX = "illón": zip(high, range(cap, 3, -6)).
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // range(cap, 3, -6) is exhausted; zip stops here
            }
            cards.insert(pow10((n - 3) as u32), format!("{}illón", word));
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mil"),
                (100, "cien"),
                (90, "noventa"),
                (80, "ochenta"),
                (70, "setenta"),
                (60, "sesenta"),
                (50, "cincuenta"),
                (40, "cuarenta"),
                (30, "treinta"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "veintinueve",
                "veintiocho",
                "veintisiete",
                "veintiséis",
                "veinticinco",
                "veinticuatro",
                "veintitrés",
                "veintidós",
                "veintiuno",
                "veinte",
                "diecinueve",
                "dieciocho",
                "diecisiete",
                "dieciséis",
                "quince",
                "catorce",
                "trece",
                "doce",
                "once",
                "diez",
                "nueve",
                "ocho",
                "siete",
                "seis",
                "cinco",
                "cuatro",
                "tres",
                "dos",
                "uno",
                "cero",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10^54 = 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<u64, &'static str> = [
            (1, "primer"),
            (2, "segund"),
            (3, "tercer"),
            (4, "cuart"),
            (5, "quint"),
            (6, "sext"),
            (7, "séptim"),
            (8, "octav"),
            (9, "noven"),
            (10, "décim"),
            (20, "vigésim"),
            (30, "trigésim"),
            (40, "cuadragésim"),
            (50, "quincuagésim"),
            (60, "sexagésim"),
            (70, "septuagésim"),
            (80, "octogésim"),
            (90, "nonagésim"),
            (100, "centésim"),
            (200, "ducentésim"),
            (300, "tricentésim"),
            (400, "cuadrigentésim"),
            (500, "quingentésim"),
            (600, "sexcentésim"),
            (700, "septigentésim"),
            (800, "octigentésim"),
            (900, "noningentésim"),
            (1_000, "milésim"),
            (1_000_000, "millonésim"),
            (1_000_000_000, "billonésim"),
            (1_000_000_000_000, "trillonésim"),
            (1_000_000_000_000_000, "cuadrillonésim"),
        ]
        .into_iter()
        .collect();

        LangEsCo {
            cards,
            maxval,
            ords,
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            // Built once here, never per call. `to_currency` and `to_cheque`
            // only ever read these tables.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `self.ords[key]` — a miss is Python's `KeyError`, not a tidy fallback.
    fn ord_word(&self, key: &BigInt) -> Result<&'static str> {
        let k = key
            .to_u64()
            .ok_or_else(|| N2WError::Key(format!("{}", key)))?;
        self.ords
            .get(&k)
            .copied()
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The `value == int(value)` float check cannot fire for integer input;
    /// only the negative check is reachable, and it raises `TypeError`.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_ES.to_ordinal(value, gender="m")`.
    ///
    /// `gender` is threaded through faithfully even though all four public
    /// modes enter with the default "m" — only `to_currency`/`str_to_number`
    /// (both out of scope) ever pass "f".
    fn ordinal_g(&self, value: &BigInt, fem: bool) -> Result<String> {
        // Python: gender_stem = "a" if gender == "f" else "o"
        let mut gender_stem = if fem { "a" } else { "o" };

        self.verify_ordinal(value)?;

        let ten = BigInt::from(10);
        let text: String = if value.is_zero() {
            String::new()
        } else if *value <= ten {
            format!("{}{}", self.ord_word(value)?, gender_stem)
        } else if *value <= BigInt::from(29) {
            // RAE: simple forms up to 30. The local rebind to "o" here means a
            // feminine 21 is still "vigesimoprimera", and the é-stripping
            // fires even when value % 10 == 0, so 20 -> "vigesimo".
            gender_stem = "o";
            let dec = (value / 10u32) * 10u32;
            format!(
                "{}{}{}",
                self.ord_word(&dec)?.replace('é', "e"),
                gender_stem,
                self.ordinal_g(&(value % 10u32), fem)?
            )
        } else if *value <= BigInt::from(100) {
            // dec reaches 100 for value == 100, which ords has.
            let dec = (value / 10u32) * 10u32;
            format!(
                "{}{} {}",
                self.ord_word(&dec)?,
                gender_stem,
                self.ordinal_g(&(value - &dec), fem)?
            )
        } else if *value <= BigInt::from(1000) {
            // cen reaches 1000 for value == 1000; ords[1000] hits the 1e3 key.
            let cen = (value / 100u32) * 100u32;
            format!(
                "{}{} {}",
                self.ord_word(&cen)?,
                gender_stem,
                self.ordinal_g(&(value - &cen), fem)?
            )
        } else if *value < pow10(18) {
            // Round down to the nearest 1e(3n) via the float log (see
            // `log_bucket` — the imprecision is load-bearing).
            let dec = BigInt::from(1000).pow(log_bucket(value) as u32);
            let (high_part, low_part) = value.div_mod_floor(&dec);

            let cardinal = if high_part.is_one() {
                String::new()
            } else {
                self.to_cardinal(&high_part)?
            };

            // Python builds the %-format tuple left to right, so `ords[dec]`
            // is resolved *before* the recursive call. Keep that order: for
            // value in [10^18-4928, 10^18) this KeyError must win over the
            // self-recursion the guard below would otherwise report.
            let ord_dec = self.ord_word(&dec)?;

            // Guard the four inputs (10^15 - 1..4) where the log rounds up,
            // dec > value, and Python recurses on its own argument until it
            // raises RecursionError. Rust would abort on stack overflow; see
            // the module docs — Value is a stand-in for RecursionError.
            if dec > *value {
                return Err(N2WError::Value(format!(
                    "maximum recursion depth exceeded in comparison ({})",
                    value
                )));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                ord_dec,
                gender_stem,
                self.ordinal_g(&low_part, fem)?
            )
        } else {
            self.to_cardinal(value)?
        };

        // Exception: it's not "decimooctavo" but "decimoctavo".
        Ok(text.trim().replace("oo", "o"))
    }
}

impl Lang for LangEsCo {

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "COP"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " y"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "menos "
    }
    fn pointword(&self) -> &str {
        "punto"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// `Num2Word_ES.merge`.
    ///
    /// Character-slicing throughout: `"millón"` is 6 chars but 7 bytes, so
    /// `ntext[:-3]` == "mil" only when counting `chars()`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext_in, cnum) = l;
        let (ntext_in, nnum) = r;
        let mut ctext = ctext_in.to_string();
        let mut ntext = ntext_in.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if *nnum < million {
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        } else if *cnum == hundred && !(nnum % &thousand).is_zero() {
            // "cien" + "t" + "o" -> "ciento" (101 -> "ciento uno"), but
            // 100_000 keeps "cien mil" because 1000 % 1000 == 0.
            ctext.push_str("t");
            ctext.push_str(GENDER_STEM);
        }

        if *nnum < *cnum {
            if *cnum < hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && *cnum > BigInt::one() {
            // "millón"[:-3] + "lones" -> "millones"; likewise billón/trillón.
            let keep = ntext.chars().count().saturating_sub(3);
            ntext = format!("{}lones", ntext.chars().take(keep).collect::<String>());
        }

        if *nnum == hundred {
            if *cnum == BigInt::from(5) {
                ctext = "quinien".to_string();
                ntext = String::new();
            } else if *cnum == BigInt::from(7) {
                ctext = "sete".to_string();
            } else if *cnum == BigInt::from(9) {
                ctext = "nove".to_string();
            }
            ntext.push_str("t");
            ntext.push_str(GENDER_STEM);
            ntext.push_str("s");
        } else {
            // Spanish drops the final 'o' of 'uno' (and adds an accent in
            // 'veintiuno') before any noun like 'mil' or 'millones':
            // 31000 -> "treinta y un mil", 21000 -> "veintiún mil".
            if *nnum >= thousand {
                if ctext.ends_with("veintiuno") {
                    let keep = ctext.chars().count() - 3;
                    ctext = format!("{}ún", ctext.chars().take(keep).collect::<String>());
                } else if ctext.ends_with("uno") {
                    let keep = ctext.chars().count() - 1;
                    ctext = ctext.chars().take(keep).collect::<String>();
                }
            }
            ntext = format!(" {}", ntext);
        }

        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    /// `Num2Word_Base.to_cardinal`, overridden only to carry ES's own
    /// `errmsg_toobig` (typo "deber" included). The engine path is otherwise
    /// untouched — `default_to_cardinal` re-checks the bound harmlessly.
    ///
    /// NOTE: `Num2Word_ES.to_cardinal` also consumes `self._pending_ordinal`,
    /// a flag `str_to_number` stashes for tokens like "1ro". That handshake is
    /// cross-call mutable state with no place in a stateless port; see the
    /// report's `concerns`. With the flag never set, Python falls through to
    /// `super().to_cardinal`, which is exactly what happens here.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let v = value.abs();
        if v >= self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) deber ser inferior a {}.",
                v, self.maxval
            )));
        }
        default_to_cardinal(self, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        crate::lang_es::es_reject_zero_ordinal(value)?;
        self.ordinal_g(value, false)
    }

    /// `Num2Word_ES.to_ordinal_num(value, gender="m")` -> "<digits>º".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year` ignores `suffix`/`longval` and is a plain
    /// cardinal — negative years get "menos", not a BC suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entry (corpus: wholefloat slice) -------------------
    //
    // `Num2Word_ES.to_ordinal`/`to_ordinal_num` start with `verify_ordinal`,
    // so float/Decimal input is accepted only when whole and non-negative:
    // fractional -> TypeError (`errmsg_floatord`), negative whole -> TypeError
    // (`errmsg_negord`). -0.0 *passes* both checks (abs(-0.0) == -0.0) and
    // renders like 0 — to_ordinal(-0.0) raises ValueError like 0 (#160),
    // to_ordinal_num(-0.0) == "-0.0º".
    // `to_year` truncates via `int(val)`: to_year(-1.5) == "menos uno".

    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            // A whole value renders exactly like the int: Python's dict
            // lookups hash 5.0 like 5, so `ords[5.0]` hits. to_ordinal
            // re-runs the (still reachable) negative check.
            Some(i) => self.to_ordinal(&i),
            None => Err(float_ordinal_reject(value)),
        }
    }

    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            // `"%s%s" % (value, "º")` — the Python str of the value, so
            // "5.00º", "1e+16º", "-0.0º".
            Some(i) if !i.is_negative() => Ok(format!("{}º", repr_str)),
            Some(_) => Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                repr_str
            ))),
            None => Err(float_ordinal_reject(value)),
        }
    }

    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        // `return self.to_cardinal(int(val))` — truncation toward zero, so
        // to_year(0.5) == "cero" and to_year(3.25) == "tres".
        self.to_cardinal(&float_trunc_int(value)?)
    }

    // ---- grammatical kwargs (corpus: kwargs slice) -------------------------

    /// `to_ordinal(value, gender="m")` — ES's one ordinal kwarg. Anything not
    /// in the Python signature -> NotImplemented, so the dispatcher falls back
    /// to Python, which raises the original TypeError. Any gender value other
    /// than "f" (an explicit None, "x", an int, ...) is masculine:
    /// `gender_stem = "a" if gender == "f" else "o"`.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        crate::lang_es::es_reject_zero_ordinal(value)?;
        self.ordinal_g(value, kw.str("gender") == Some("f"))
    }

    /// `to_ordinal_num(value, gender="m")`: digits + "ª" for "f", "º" else.
    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.verify_ordinal(value)?;
        let marker = if kw.str("gender") == Some("f") { "ª" } else { "º" };
        Ok(format!("{}{}", value, marker))
    }

    /// `to_year(val, suffix=None, longval=True)`: both kwargs are accepted
    /// and ignored — the body is `to_cardinal(int(val))` regardless.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.to_year(value)
    }

    /// `Num2Word_ES.to_fraction` (issue #584), inherited unchanged by ES_CO:
    /// idiomatic medio/tercio/cuarto for denominators 2/3/4, ordinal-as-noun
    /// otherwise (a bare "s" plural appended only when the ordinal isn't
    /// s-final already), and the apocopated "un" for numerator 1.
    /// `denominator == 1` (not -1!) or `numerator == 0` short-circuits to the
    /// plain cardinal.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision("denominator must not be zero".into()));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();

        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "medio" } else { "medios" }).to_string()
        } else if abs_d == BigInt::from(3) {
            (if abs_n.is_one() { "tercio" } else { "tercios" }).to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "cuarto" } else { "cuartos" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() && !w.ends_with('s') {
                w.push('s');
            }
            w
        };

        // "un" (never "uno") before the denominator noun; the sign is
        // `"%s " % self.negword.strip()`.
        let num_word = if abs_n.is_one() {
            "un".to_string()
        } else {
            self.to_cardinal(&abs_n)?
        };
        let sign = if is_negative { "menos " } else { "" };
        Ok(format!("{}{} {}", sign, num_word, den_word))
    }

    // ---- currency ----------------------------------------------------
    //
    // `currency_precision` is deliberately NOT overridden. `CURRENCY_PRECISION`
    // is `{}` on `Num2Word_Base` and none of EUR/ES/ES_CO defines one, so
    // `.get(code, 100)` yields 100 for *every* code — verified on the live
    // instance. The 0-decimal (divisor 1) and 3-decimal (divisor 1000) branches
    // are therefore both dead for `es_CO`, doubly so because JPY/KWD/BHD are not
    // in its forms table at all and raise before precision is ever consulted.
    //
    // `money_verbose` / `cents_verbose` / `cents_terse` / `to_cheque` are not
    // overridden either: ES_CO inherits `Num2Word_Base`'s straight through
    // (`Num2Word_ES` overrides none of them), and the trait defaults already
    // reproduce those. Notably `to_cheque` never sees the `"uno"` -> `"un"`
    // rewrite below, since ES_CO only wraps `to_currency` — hence Python's
    // "UNO AND 00/100 EUROS" and "VEINTIUNO AND 21/100 EUROS".

    fn lang_name(&self) -> &str {
        "Num2Word_ES_CO"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0]` for exactly 1, `forms[1]` else.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. All three of ES_CO's entries carry two forms, so
    /// this is unreachable — mapped to `Index` rather than panicking so the
    /// exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES_CO.to_currency` over `Num2Word_ES.to_currency`.
    ///
    /// ES_CO's own body is one line — delegate to ES, then
    /// `result.replace("uno", "un")` — but that blanket substitution runs over
    /// *both* of ES's paths and lands after ES's own six targeted rewrites, so
    /// the two are ported together here rather than split.
    ///
    /// The int/float split is on `isinstance(val, int)`, not on whether the
    /// number happens to be whole: `1` is "un peso", `1.0` is "un peso y cero
    /// centavos".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // None => the caller omitted `separator=`, so ES_CO's own default (" y",
        // read from the live signature) applies. Note ES_CO *narrows* ES's
        // " con" to " y" in its override's signature, and since ES_CO always
        // forwards the argument explicitly, ES's " con" is unreachable via this
        // class — every float row in the corpus reads "... y ...", never
        // "... con ...".
        let separator = separator.unwrap_or(self.default_separator());

        // ---- Num2Word_ES.to_currency, integer path: no cents, ever ----
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                // Python catches the KeyError and delegates to
                // `super().to_currency`, whose integer branch repeats the same
                // lookup and raises NotImplementedError from it. The fallback
                // can only ever raise, so it is collapsed to the raise.
                None => {
                    return Err(N2WError::NotImplemented(format!(
                        "Currency code \"{}\" not implemented for \"{}\"",
                        currency,
                        self.lang_name()
                    )))
                }
            };

            // `adjective` is dropped on the floor here — ES's integer branch
            // never consults CURRENCY_ADJECTIVES and never forwards the flag,
            // so `to_currency(1, "USD", adjective=True)` is "un dólar", not
            // "un US dólar". Only the float path below applies it.
            let _ = adjective;

            // Python uses `negword` raw here, so its trailing space collided
            // with the format string's ("menos  un peso"). Stripped like the
            // float path so the output has single spaces (#160).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();

            // Python computes to_cardinal(abs_val) first and discards it when
            // abs_val == 1; kept in that order, though it cannot fail for 1.
            let mut money_str = self.to_cardinal(&abs_val)?;
            let currency_str = if abs_val.is_one() {
                money_str = "un".to_string();
                forms.unit.first()
            } else {
                forms.unit.get(1).or_else(|| forms.unit.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            let result = format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string();
            // ES_CO's blanket rewrite, applied to ES's already-stripped return.
            // It is what turns to_cardinal(21)'s "veintiuno" into the
            // unaccented "veintiun pesos" — ES's targeted "veintiuno euro"
            // rewrite never runs on this path, so the accent is never restored.
            return Ok(result.replace("uno", "un"));
        }

        // ---- float path: Num2Word_Base's, then two layers of patching ----
        let result =
            crate::currency::default_to_currency(self, val, currency, cents, separator, adjective)?;

        // Num2Word_ES's six blind substitutions, in Python's exact order. They
        // are keyed on the currency *word*, not the code, and "peso" is not
        // among them — COP is carried entirely by ES_CO's blanket rewrite
        // below. Order matters: "veintiuno euro" must fire before "uno euro",
        // or 21.21 EUR would lose its accent the way 21.21 USD does.
        let result = result.replace("veintiuno euro", "veintiún euro");
        let result = result.replace("veintiuno céntimo", "veintiún céntimo");
        let result = result.replace("uno euro", "un euro");
        let result = result.replace("uno céntimo", "un céntimo");
        let result = result.replace("uno centavo", "un centavo");
        let result = result.replace("uno dólar", "un dólar");

        // Num2Word_ES_CO's own rewrite, mopping up every "uno" the six above
        // missed — including inside words. This is why 21.21 USD is "veintiun
        // dólares y veintiun centavos" (accent lost: ES's "uno dólar"/"uno
        // centavo" rules already consumed the "uno", leaving "veintiun") while
        // 21.21 EUR keeps both accents ("veintiún euros y veintiún céntimos").
        // It also rescues cases ES's word-keyed rules cannot see, e.g.
        // adjective=True USD 1.0 -> "uno US dólar ..." -> "un US dólar ...".
        Ok(result.replace("uno", "un"))
    }
}
