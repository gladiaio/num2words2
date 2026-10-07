//! Port of `lang_ES_VE.py` (Spanish — Venezuela).
//!
//! Registry check: `CONVERTER_CLASSES["es_VE"]` → `lang_ES_VE.Num2Word_ES_VE()`,
//! which is the class this file ports.
//!
//! `Num2Word_ES_VE(Num2Word_ES)` overrides **only** `__init__` (currency forms)
//! and `to_currency`. Both are out of scope, so every in-scope mode
//! (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`) is inherited
//! verbatim from `Num2Word_ES`. This file is therefore a port of `lang_ES.py`
//! reached through the chain `ES_VE → ES → EUR → Num2Word_Base`.
//!
//! Shape: **engine**. `Num2Word_ES.setup` defines `high_numwords` /
//! `mid_numwords` / `low_numwords` and `merge`, so `Num2Word_Base.to_cardinal`
//! drives `splitnum`/`clean`/`merge`. `to_cardinal` stays at the trait default
//! (`default_to_cardinal`); `to_ordinal` / `to_ordinal_num` / `to_year` are
//! overridden, exactly as in Python.
//!
//! # Card table
//!
//! `setup` calls `gen_high_numwords([], [], lows)` with **empty** units and
//! tens, so the Latin-prefix cross product `[u + t for t in tens for u in
//! units]` is empty and `high_numwords` collapses to `lows` itself (9 stems).
//! `Num2Word_EUR.set_high_numwords` then runs with `cap = 3 + 6*9 = 57` over
//! `zip(high, range(57, 3, -6))`. ES sets `GIGA_SUFFIX = None`, so the
//! `10**n` (illiard) half is skipped entirely and only the `10**(n-3)`
//! (`MEGA_SUFFIX = "illón"`) half lands:
//!
//! ```text
//!   10^6  millón    10^12 billón     10^18 trillón    10^24 cuatrillón
//!   10^30 quintillón  10^36 sextillón  10^42 septillón  10^48 octillón
//!   10^54 nonillón
//! ```
//!
//! This is the long scale: there is deliberately **no** card at 10^9, so
//! 10^9 renders as "mil millones" and 10^15 as "mil billones" via `merge`.
//! `MAXVAL = 1000 * 10^54 = 10^57`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are wrong-looking but
//! are exactly what Python emits, verified against the interpreter:
//!
//! 1. ~~**Missing accent on 20.**~~ The `value <= 29` branch does
//!    `self.ords[dec].replace("é", "e")` for the fused forms
//!    ("vigesimoprimero"), and 20 used to land there too ("vigesimo").
//!    Fixed (gladiaio/num2words2#252): 20 keeps its accent and the caller's
//!    gender ("vigésimo", "vigésima", "centésimo vigésimo"); only the fused
//!    21..=29 forms drop it ("vigesimoprimero"), as the RAE spells them.
//! 2. **No space before the scale word in the `1e3`–`1e18` branch.** The
//!    format is `"%s%s%s %s" % (cardinal, ords[dec], gender_stem, ...)` — the
//!    cardinal is glued straight onto the ordinal stem. Hence
//!    `to_ordinal(2000)` == "dosmilésimo", `to_ordinal(99999)` ==
//!    "noventa y nuevemilésimo noningentésimo nonagésimo noveno", and
//!    `to_ordinal(123456)` == "ciento veintitrésmilésimo ...". Preserved.
//! 3. **`to_ordinal(0)` returned the empty string** in Python (`value == 0` →
//!    `text = ""`); the public entry points raise ValueError instead (#160).
//! 4. **`replace("oo", "o")` is applied to the whole string at every recursion
//!    level**, not just the intended "decimooctavo" → "decimoctavo" junction.
//! 5. **`to_ordinal` above 1e18 silently degrades to a cardinal**: the final
//!    `else` returns `self.to_cardinal(value)`, so `to_ordinal(10**18)` ==
//!    "un trillón" and `to_ordinal(10**21)` == "mil trillones" — not ordinals
//!    at all.
//! 6. **Two float-rounding craters in `int(math.log(int(value), 1000))`** —
//!    see [`log1000_floor`]. Both are genuine Python crashes:
//!    * `to_ordinal(v)` for `v` in `[999_999_999_999_996, 10**15)` →
//!      `RecursionError` (the log floors to 5, so `dec = 10**15 > v`,
//!      `divmod` yields `high_part = 0` and `low_part == value`, and
//!      `to_ordinal(low_part)` recurses on the *same* value forever).
//!      The window is also reached **transitively**: `to_ordinal(10**16 - 1)`
//!      splits into `high_part = 9`, `low_part = 999_999_999_999_999`, and
//!      dies inside the recursive call. `10**17 - 1` and `10**17 - 2` do the
//!      same. The guard below is per-level, so those propagate naturally.
//!    * `to_ordinal(v)` for `v` in `[999_999_999_999_995_072, 10**18)` →
//!      `KeyError: 1000000000000000000` (the log floors to 6, so
//!      `dec = 1000**6 = 10**18`, which is absent from `ords`).
//!    The `KeyError` wins over the `RecursionError` in the second window
//!    because Python evaluates the `%`-format tuple left to right, so
//!    `self.ords[dec]` is looked up *before* `self.to_ordinal(low_part)` is
//!    called. [`LangEsVe::to_ordinal_gender`] preserves that ordering.
//!
//! # Error variants
//!
//! * `to_ordinal` / `to_ordinal_num` of a negative → `verify_ordinal` raises
//!   `TypeError` → [`N2WError::Type`].
//! * `to_cardinal` at or above `MAXVAL` (10^57) → `OverflowError` →
//!   [`N2WError::Overflow`] (raised by `default_to_cardinal` in `base.rs`).
//! * The missing `ords[1000**6]` → `KeyError` → [`N2WError::Key`].
//! * Python's `RecursionError` has **no** counterpart in `N2WError`. It is
//!   reported as [`N2WError::Value`] with an explicit "RecursionError" message
//!   rather than being allowed to actually blow the Rust stack (which would
//!   abort the whole process instead of surfacing a catchable error). This is
//!   the one place where the exception *type* cannot be matched — see the
//!   port report's `concerns`.

use crate::base::{
    set_low_numwords, set_mid_numwords, Cards, Kwargs, KwVal, Lang, N2WError, Result,
};
use crate::currency::{parse_currency_parts, prefix_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `setup` and never reassigned.
///
/// `to_ordinal` binds a *local* `gender_stem`, shadowing nothing on `self`, so
/// the instance attribute that `merge` reads is always "o".
const GENDER_STEM: &str = "o";

/// `Num2Word_ES_VE.to_currency`'s own `separator=" y"` default.
///
/// ES_VE narrows `Num2Word_ES`'s `" con"`, which in turn narrows
/// `Num2Word_Base`'s `","`. The corpus rows are generated by calling
/// `num2words(v, lang="es_VE", to="currency", currency=C)` with **no**
/// separator, so " y" is what every expected string carries
/// ("doce euros y treinta y cuatro céntimos").
const DEFAULT_SEPARATOR: &str = " y";

/// `Num2Word_Base.to_currency`'s `separator=","` default, used here as the
/// "caller did not pass one" sentinel — see [`LangEsVe::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// Python's `int(math.log(int(value), 1000))`, as an exact integer step
/// function.
///
/// CPython's `loghelper` converts the int to a C double and takes `log(x) /
/// log(1000.0)`. Reproducing that in Rust would mean betting that Rust's
/// `f64::ln` agrees with the platform libm to the last ulp — a bet this port
/// declines. The step points were instead binary-searched against CPython and
/// verified by brute-force scan, so this is exact and float-free.
///
/// The floor is mathematically correct for the first four steps and **wrong**
/// for the last two, where the double rounds up across the boundary:
///
/// | result | Python's range                                  | correct? |
/// |--------|-------------------------------------------------|----------|
/// | 1      | `[10^3, 10^6)`                                  | yes      |
/// | 2      | `[10^6, 10^9)`                                  | yes      |
/// | 3      | `[10^9, 10^12)`                                 | yes      |
/// | 4      | `[10^12, 999_999_999_999_996)`                  | yes      |
/// | 5      | `[999_999_999_999_996, 999_999_999_999_995_072)`| **no** — starts 4 early |
/// | 6      | `[999_999_999_999_995_072, 10^18)`              | **no** — starts 4928 early |
///
/// Only called with `1000 < value < 10**18`, so the result is always 1..=6.
fn log1000_floor(value: u64) -> u32 {
    // Ordered high → low; the two upper thresholds are the float craters.
    if value >= 999_999_999_999_995_072 {
        6
    } else if value >= 999_999_999_999_996 {
        5
    } else if value >= 1_000_000_000_000 {
        4
    } else if value >= 1_000_000_000 {
        3
    } else if value >= 1_000_000 {
        2
    } else if value >= 1_000 {
        1
    } else {
        0
    }
}

/// Drop the last `n` **characters** of `s`, Python's `s[:-n]`.
///
/// Byte slicing would corrupt every accented scale word: "millón" is 6 chars
/// but 7 bytes, and `"millón"[:-3]` must yield "mil" (→ "millones"), not a
/// split of the "ó".
fn drop_last_chars(s: &str, n: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let keep = chars.len().saturating_sub(n);
    chars[..keep].iter().collect()
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

pub struct LangEsVe {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    /// `self.CURRENCY_FORMS` after `Num2Word_ES_VE.__init__` has copied
    /// `Num2Word_ES`'s class dict and applied its VES/VEB/USD update.
    ///
    /// Built once in [`LangEsVe::new`] and read by reference thereafter:
    /// rebuilding 172 entries per call is what made an earlier revision of
    /// this port 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangEsVe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsVe {
    pub fn new() -> Self {
        // setup(): high_numwords = gen_high_numwords([], [], lows).
        // units and tens are empty, so the cross product is empty and
        // gen_high_numwords degenerates to `[] + lows` == lows.
        let high = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords with GIGA_SUFFIX=None, MEGA_SUFFIX="illón":
        //   cap = 3 + 6*len(high) = 57
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**(n-3)] = word + "illón"
        // range(57, 3, -6) yields exactly 9 values for the 9 stems, so zip
        // consumes both fully.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // range exhausted (zip stops at the shorter sequence)
            }
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illón", word),
            );
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

        // Num2Word_Base.__init__: MAXVAL = 1000 * first card = 1000 * 10^54.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // CURRENCY_FORMS. Chased through the whole chain:
        //
        //   Num2Word_ES_VE.__init__ does `self.CURRENCY_FORMS.copy()`, which
        //   resolves via the MRO to `Num2Word_ES.CURRENCY_FORMS` — a class
        //   attribute that *shadows* `Num2Word_EUR.CURRENCY_FORMS` rather than
        //   extending it. EUR-only codes (EEK, and EUR's own English wordings)
        //   are therefore invisible here. The copy is then `.update()`d with
        //   VES, VEB (new — the pre-2018 bolívar) and USD.
        //
        // 172 entries, every one arity (2, 2) — verified by dumping the live
        // instance dict, which is also what generated this table, so no code
        // was transcribed by hand.
        //
        // Quirks below are Python's and are deliberate: "lek " (ALL) carries a
        // trailing space, MAD's singular is accented ("dírham") while its
        // plural is not ("dirhams"), and CNY's subunits are the mismatched
        // pair ("fen", "jiaos") — the corpus pins all three.
        let table: &[(&'static str, &[&'static str], &[&'static str])] = &[
            ("EUR", &["euro", "euros"], &["céntimo", "céntimos"]),
            ("ESP", &["peseta", "pesetas"], &["céntimo", "céntimos"]),
            ("USD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("PEN", &["sol", "soles"], &["céntimo", "céntimos"]),
            ("CRC", &["colón", "colones"], &["céntimo", "céntimos"]),
            ("AUD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("CAD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("GBP", &["libra", "libras"], &["penique", "peniques"]),
            ("RUB", &["rublo", "rublos"], &["kopeyka", "kopeykas"]),
            ("SEK", &["corona", "coronas"], &["öre", "öre"]),
            ("NOK", &["corona", "coronas"], &["øre", "øre"]),
            ("PLN", &["zloty", "zlotys"], &["grosz", "groszy"]),
            ("MXN", &["peso", "pesos"], &["centavo", "centavos"]),
            ("RON", &["leu", "leus"], &["ban", "bani"]),
            ("INR", &["rupia", "rupias"], &["paisa", "paisas"]),
            ("HUF", &["florín", "florines"], &["fillér", "fillér"]),
            ("FRF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("CNY", &["yuan", "yuanes"], &["fen", "jiaos"]),
            ("CZK", &["corona", "coronas"], &["haléř", "haléř"]),
            ("NIO", &["córdoba", "córdobas"], &["centavo", "centavos"]),
            ("VES", &["bolívar", "bolívares"], &["centavo", "centavos"]),
            ("BRL", &["real", "reales"], &["centavo", "centavos"]),
            ("CHF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("JPY", &["yen", "yenes"], &["sen", "sen"]),
            ("KRW", &["won", "wones"], &["jeon", "jeon"]),
            ("KPW", &["won", "wones"], &["chon", "chon"]),
            ("TRY", &["lira", "liras"], &["kuruş", "kuruş"]),
            ("ZAR", &["rand", "rands"], &["céntimo", "céntimos"]),
            ("KZT", &["tenge", "tenges"], &["tïın", "tïın"]),
            ("UAH", &["hryvnia", "hryvnias"], &["kopiyka", "kopiykas"]),
            ("THB", &["baht", "bahts"], &["satang", "satang"]),
            ("AED", &["dirham", "dirhams"], &["fils", "fils"]),
            ("AFN", &["afghani", "afghanis"], &["pul", "puls"]),
            ("ALL", &["lek ", "leke"], &["qindarkë", "qindarka"]),
            ("AMD", &["dram", "drams"], &["luma", "lumas"]),
            ("ANG", &["florín", "florines"], &["centavo", "centavos"]),
            ("AOA", &["kwanza", "kwanzas"], &["céntimo", "céntimos"]),
            ("ARS", &["peso", "pesos"], &["centavo", "centavos"]),
            ("AWG", &["florín", "florines"], &["centavo", "centavos"]),
            ("AZN", &["manat", "manat"], &["qəpik", "qəpik"]),
            ("BBD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("BDT", &["taka", "takas"], &["paisa", "paisas"]),
            ("BGN", &["lev", "leva"], &["stotinka", "stotinki"]),
            ("BHD", &["dinar", "dinares"], &["fils", "fils"]),
            ("BIF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("BMD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("BND", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("BOB", &["boliviano", "bolivianos"], &["centavo", "centavos"]),
            ("BSD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("BTN", &["ngultrum", "ngultrum"], &["chetrum", "chetrum"]),
            ("BWP", &["pula", "pulas"], &["thebe", "thebes"]),
            ("BYN", &["rublo", "rublos"], &["kópek", "kópeks"]),
            ("BYR", &["rublo", "rublos"], &["kópek", "kópeks"]),
            ("BZD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("CDF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("CLP", &["peso", "pesos"], &["centavo", "centavos"]),
            ("COP", &["peso", "pesos"], &["centavo", "centavos"]),
            ("CUP", &["peso", "pesos"], &["centavo", "centavos"]),
            ("CVE", &["escudo", "escudos"], &["centavo", "centavos"]),
            ("CYP", &["libra", "libras"], &["céntimo", "céntimos"]),
            ("DJF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("DKK", &["corona", "coronas"], &["øre", "øre"]),
            ("DOP", &["peso", "pesos"], &["centavo", "centavos"]),
            ("DZD", &["dinar", "dinares"], &["céntimo", "céntimos"]),
            ("ECS", &["sucre", "sucres"], &["centavo", "centavos"]),
            ("EGP", &["libra", "libras"], &["piastra", "piastras"]),
            ("ERN", &["nakfa", "nakfas"], &["céntimo", "céntimos"]),
            ("ETB", &["birr", "birrs"], &["céntimo", "céntimos"]),
            ("FJD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("FKP", &["libra", "libras"], &["penique", "peniques"]),
            ("GEL", &["lari", "laris"], &["tetri", "tetris"]),
            ("GHS", &["cedi", "cedis"], &["pesewa", "pesewas"]),
            ("GIP", &["libra", "libras"], &["penique", "peniques"]),
            ("GMD", &["dalasi", "dalasis"], &["butut", "bututs"]),
            ("GNF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("GTQ", &["quetzal", "quetzales"], &["centavo", "centavos"]),
            ("GYD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("HKD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("HNL", &["lempira", "lempiras"], &["centavo", "centavos"]),
            ("HRK", &["kuna", "kunas"], &["lipa", "lipas"]),
            ("HTG", &["gourde", "gourdes"], &["céntimo", "céntimos"]),
            ("IDR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
            ("ILS", &["séquel", "séqueles"], &["agora", "agoras"]),
            ("IQD", &["dinar", "dinares"], &["fils", "fils"]),
            ("IRR", &["rial", "riales"], &["dinar", "dinares"]),
            ("ISK", &["corona", "coronas"], &["eyrir", "aurar"]),
            ("ITL", &["lira", "liras"], &["céntimo", "céntimos"]),
            ("JMD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("JOD", &["dinar", "dinares"], &["piastra", "piastras"]),
            ("KES", &["chelín", "chelines"], &["céntimo", "céntimos"]),
            ("KGS", &["som", "som"], &["tyiyn", "tyiyn"]),
            ("KHR", &["riel", "rieles"], &["céntimo", "céntimos"]),
            ("KMF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("KWD", &["dinar", "dinares"], &["fils", "fils"]),
            ("KYD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("LAK", &["kip", "kips"], &["att", "att"]),
            ("LBP", &["libra", "libras"], &["piastra", "piastras"]),
            ("LKR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
            ("LRD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("LSL", &["loti", "lotis"], &["céntimo", "céntimos"]),
            ("LTL", &["lita", "litas"], &["céntimo", "céntimos"]),
            ("LVL", &["lat", "lats"], &["céntimo", "céntimos"]),
            ("LYD", &["dinar", "dinares"], &["dírham", "dírhams"]),
            ("MAD", &["dírham", "dirhams"], &["céntimo", "céntimos"]),
            ("MDL", &["leu", "lei"], &["ban", "bani"]),
            ("MGA", &["ariary", "ariaris"], &["iraimbilanja", "iraimbilanja"]),
            ("MKD", &["denar", "denares"], &["deni", "denis"]),
            ("MMK", &["kiat", "kiats"], &["pya", "pyas"]),
            ("MNT", &["tugrik", "tugriks"], &["möngö", "möngö"]),
            ("MOP", &["pataca", "patacas"], &["avo", "avos"]),
            ("MRO", &["ouguiya", "ouguiyas"], &["khoums", "khoums"]),
            ("MRU", &["ouguiya", "ouguiyas"], &["khoums", "khoums"]),
            ("MUR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
            ("MVR", &["rufiyaa", "rufiyaas"], &["laari", "laari"]),
            ("MWK", &["kuacha", "kuachas"], &["tambala", "tambalas"]),
            ("MYR", &["ringgit", "ringgit"], &["céntimo", "céntimos"]),
            ("MZN", &["metical", "metical"], &["centavo", "centavos"]),
            ("NAD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("NGN", &["naira", "nairas"], &["kobo", "kobo"]),
            ("NPR", &["rupia", "rupias"], &["paisa", "paisas"]),
            ("NZD", &["dólar", "dólares"], &["centavo", "centavos"]),
            ("OMR", &["rial", "riales"], &["baisa", "baisa"]),
            ("PAB", &["balboa", "balboas"], &["centésimo", "centésimos"]),
            ("PGK", &["kina", "kinas"], &["toea", "toea"]),
            ("PHP", &["peso", "pesos"], &["centavo", "centavos"]),
            ("PKR", &["rupia", "rupias"], &["paisa", "paisas"]),
            ("PLZ", &["zloty", "zlotys"], &["grosz", "groszy"]),
            ("PYG", &["guaraní", "guaranís"], &["céntimo", "céntimos"]),
            ("QAR", &["rial", "riales"], &["dírham", "dírhams"]),
            ("QTQ", &["quetzal", "quetzales"], &["centavo", "centavos"]),
            ("RSD", &["dinar", "dinares"], &["para", "para"]),
            ("RUR", &["rublo", "rublos"], &["kopek", "kopeks"]),
            ("RWF", &["franco", "francos"], &["céntimo", "céntimos"]),
            ("SAR", &["riyal", "riales"], &["halala", "halalas"]),
            ("SBD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("SCR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
            ("SDG", &["libra", "libras"], &["piastra", "piastras"]),
            ("SGD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("SHP", &["libra", "libras"], &["penique", "peniques"]),
            ("SKK", &["corona", "coronas"], &["halier", "haliers"]),
            ("SLL", &["leona", "leonas"], &["céntimo", "céntimos"]),
            ("SRD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("SSP", &["libra", "libras"], &["piastra", "piastras"]),
            ("STD", &["dobra", "dobras"], &["céntimo", "céntimos"]),
            ("SVC", &["colón", "colones"], &["centavo", "centavos"]),
            ("SYP", &["libra", "libras"], &["piastra", "piastras"]),
            ("SZL", &["lilangeni", "emalangeni"], &["céntimo", "céntimos"]),
            ("TJS", &["somoni", "somonis"], &["dirame", "dirames"]),
            ("TMT", &["manat", "manat"], &["tenge", "tenge"]),
            ("TND", &["dinar", "dinares"], &["milésimo", "milésimos"]),
            ("TOP", &["paanga", "paangas"], &["céntimo", "céntimos"]),
            ("TTD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("TWD", &["nuevo dólar", "nuevos dólares"], &["céntimo", "céntimos"]),
            ("TZS", &["chelín", "chelines"], &["céntimo", "céntimos"]),
            ("UAG", &["hryvnia", "hryvnias"], &["kopiyka", "kopiykas"]),
            ("UGX", &["chelín", "chelines"], &["céntimo", "céntimos"]),
            ("UYU", &["peso", "pesos"], &["centésimo", "centésimos"]),
            ("UZS", &["sum", "sum"], &["tiyin", "tiyin"]),
            ("VEF", &["bolívar fuerte", "bolívares fuertes"], &["céntimo", "céntimos"]),
            ("VND", &["dong", "dongs"], &["xu", "xu"]),
            ("VUV", &["vatu", "vatu"], &["nenhum", "nenhum"]),
            ("WST", &["tala", "tala"], &["centavo", "centavos"]),
            ("XAF", &["franco CFA", "francos CFA"], &["céntimo", "céntimos"]),
            ("XCD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("XOF", &["franco CFA", "francos CFA"], &["céntimo", "céntimos"]),
            ("XPF", &["franco CFP", "francos CFP"], &["céntimo", "céntimos"]),
            ("YER", &["rial", "riales"], &["fils", "fils"]),
            ("YUM", &["dinar", "dinares"], &["para", "para"]),
            ("ZMW", &["kwacha", "kwachas"], &["ngwee", "ngwee"]),
            ("ZRZ", &["zaire", "zaires"], &["likuta", "makuta"]),
            ("ZWL", &["dólar", "dólares"], &["céntimo", "céntimos"]),
            ("VEB", &["bolívar", "bolívares"], &["centavo", "centavos"]),
        ];
        let currency_forms = table
            .iter()
            .map(|&(code, unit, subunit)| (code, CurrencyForms::new(unit, subunit)))
            .collect();

        LangEsVe {
            cards,
            maxval,
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            currency_forms,
        }
    }

    /// `self.ords`. Keys are `int` 1..=10, 20..=90 step 10, 100..=900 step 100,
    /// plus `float` 1e3/1e6/1e9/1e12/1e15.
    ///
    /// Python mixes int and float keys in one dict; that is harmless because
    /// `hash(1000) == hash(1000.0)` and `1000 == 1000.0`, so `ords[1000]`
    /// finds the `1e3` entry. A single u64-keyed table reproduces it.
    ///
    /// Every lookup site is provably `< 10**18` (see `to_ordinal_gender`), so
    /// a u64 key is sufficient and never truncates.
    fn ords(&self, key: u64) -> Option<&'static str> {
        Some(match key {
            1 => "primer",
            2 => "segund",
            3 => "tercer",
            4 => "cuart",
            5 => "quint",
            6 => "sext",
            7 => "séptim",
            8 => "octav",
            9 => "noven",
            10 => "décim",
            20 => "vigésim",
            30 => "trigésim",
            40 => "cuadragésim",
            50 => "quincuagésim",
            60 => "sexagésim",
            70 => "septuagésim",
            80 => "octogésim",
            90 => "nonagésim",
            100 => "centésim",
            200 => "ducentésim",
            300 => "tricentésim",
            400 => "cuadrigentésim",
            500 => "quingentésim",
            600 => "sexcentésim",
            700 => "septigentésim",
            800 => "octigentésim",
            900 => "noningentésim",
            1_000 => "milésim",                 // 1e3
            1_000_000 => "millonésim",          // 1e6
            1_000_000_000 => "billonésim",      // 1e9
            1_000_000_000_000 => "trillonésim", // 1e12
            1_000_000_000_000_000 => "cuadrillonésim", // 1e15
            _ => return None,
        })
    }

    /// `self.ords[key]`, raising Python's `KeyError` when absent.
    ///
    /// Reachable only for `key == 1000**6 == 10**18` (bug 6 above); every
    /// other call site is covered by the table.
    fn ords_get(&self, key: u64) -> Result<&'static str> {
        self.ords(key)
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The `value == int(value)` float check cannot fail for a BigInt, so only
    /// the sign check survives.
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
    /// `gender` is a caller-facing parameter in Python but the `Lang` trait's
    /// `to_ordinal` takes none, so the public entry point passes the default
    /// "m". The parameter is threaded through the recursion anyway, because
    /// the `value <= 29` branch forwards the *original* gender to its
    /// sub-call while forcing its own stem to "o" — dropping it would change
    /// the shape of the recursion.
    fn to_ordinal_gender(&self, value: &BigInt, gender: char) -> Result<String> {
        let gender_stem = if gender == 'f' { "a" } else { "o" };

        self.verify_ordinal(value)?;

        let text: String = if value.is_zero() {
            // Bug 3: only reachable from the recursion; the public entry
            // points reject 0 first.
            String::new()
        } else if value <= &BigInt::from(10) {
            let v = value.to_u64().expect("0 < value <= 10");
            format!("{}{}", self.ords_get(v)?, gender_stem)
        } else if value <= &BigInt::from(29) && *value != BigInt::from(20) {
            // "According to RAE recommendations, simple forms are preferred up
            // to 30 / Ortography for sobreesdrújulas" — the stem's accent is
            // stripped and the stem vowel is forced to "o" regardless of
            // `gender`, but `gender` is still handed to the recursive call.
            // 20 is not fused and skips this arm: "vigésimo" (bug 1, #252).
            let gender_stem = "o";
            let v = value.to_u64().expect("10 < value <= 29");
            let dec = (v / 10) * 10;
            format!(
                "{}{}{}",
                self.ords_get(dec)?.replace('é', "e"),
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(v % 10), gender)?
            )
        } else if value <= &BigInt::from(100) {
            let v = value.to_u64().expect("29 < value <= 100");
            let dec = (v / 10) * 10;
            format!(
                "{}{} {}",
                self.ords_get(dec)?,
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(v - dec), gender)?
            )
        } else if value <= &BigInt::from(1000) {
            // Python compares against the float 1e3; 1000 <= 1000.0 holds, so
            // 1000 lands here (cen == 1000 → ords[1e3] → "milésimo").
            let v = value.to_u64().expect("100 < value <= 1000");
            let cen = (v / 100) * 100;
            format!(
                "{}{} {}",
                self.ords_get(cen)?,
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(v - cen), gender)?
            )
        } else if value < &pow10(18) {
            // Round down to the nearest 1e(3n). Bounded by 1e18, so u64 is
            // provably sufficient here (and only here).
            let v = value.to_u64().expect("1000 < value < 10**18 fits u64");
            let k = log1000_floor(v);
            // k <= 6, and 1000**6 == 10**18 < u64::MAX, so this cannot wrap.
            let dec = 1000u64.pow(k);
            let (high_part, low_part) = v.div_rem(&dec);

            let cardinal = if high_part != 1 {
                self.to_cardinal(&BigInt::from(high_part))?
            } else {
                String::new()
            };

            // Evaluation order is load-bearing (bug 6): Python builds the
            // %-format tuple left to right, so `self.ords[dec]` raises its
            // KeyError *before* `self.to_ordinal(low_part)` would recurse.
            // In the 1000**6 window both faults are armed and the KeyError
            // must win.
            let stem = self.ords_get(dec)?;

            // Bug 6, second fault: when the float log overshoots, dec > v, so
            // high_part == 0 and low_part == v — Python then calls
            // to_ordinal(value) on the *same* value and never terminates.
            // Model it as an error instead of actually recursing, which in
            // Rust would overflow the stack and abort the process rather than
            // surface something a caller can handle.
            if low_part == v {
                return Err(N2WError::Value(format!(
                    "RecursionError: maximum recursion depth exceeded (Python \
                     to_ordinal({}) recurses on itself: int(math.log(value, \
                     1000)) == {} overshoots, so dec == {} > value)",
                    v, k, dec
                )));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                stem,
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(low_part), gender)?
            )
        } else {
            // Bug 5: >= 1e18 degrades to a plain cardinal.
            self.to_cardinal(value)?
        };

        // Bug 4: applied at every level, to the whole string.
        // "decimooctavo" -> "decimoctavo".
        Ok(text.trim().replace("oo", "o"))
    }

    // ---- currency ------------------------------------------------------

    /// The `NotImplementedError` every missing-code path in the chain raises.
    ///
    /// `self.__class__.__name__` is the *concrete* class, so it reads
    /// "Num2Word_ES_VE" even though the `raise` lives in `Num2Word_Base`.
    fn currency_not_implemented(&self, currency: &str) -> N2WError {
        N2WError::NotImplemented(format!(
            "Currency code \"{}\" not implemented for \"{}\"",
            currency,
            self.lang_name()
        ))
    }

    /// `Num2Word_ES.to_currency` — the middle link of the chain.
    ///
    /// Splits on `isinstance(val, int)`: a true int gets ES's hand-rolled
    /// branch (which never shows cents and never consults
    /// `CURRENCY_ADJECTIVES`), while anything else goes to `super()` and comes
    /// back through six literal `str.replace` fixups.
    fn es_to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: &str,
        adjective: bool,
    ) -> Result<String> {
        if let CurrencyValue::Int(v) = val {
            // Python: `except (KeyError, AttributeError)` -> `super().to_currency(...)`.
            // That fallback's int path re-looks-up the very same missing key and
            // raises NotImplementedError, so it has exactly one possible
            // outcome and is inlined here rather than round-tripped through
            // `currency::default_to_currency`. (`AttributeError` cannot fire:
            // CURRENCY_FORMS is always bound.)
            let forms = match self.currency_forms(currency) {
                Some(f) => f,
                None => return Err(self.currency_not_implemented(currency)),
            };

            let abs_val = v.abs();
            // to_cardinal runs *before* the `== 1` test in Python, so an
            // OverflowError from a huge value beats the "un" substitution.
            let money_str = self.to_cardinal(&abs_val)?;

            // Python uses `self.negword` raw here, so "menos " plus the
            // "%s %s %s" format's own space gave "menos  cinco euros".
            // Stripped like every other minus site (#160).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };

            // `cr1[0]` / `cr1[1]`: unguarded in Python too. Every one of the
            // 172 entries in the table is arity 2, so neither can be missing.
            let (money_str, currency_str) = if abs_val.is_one() {
                ("un".to_string(), forms.unit[0].clone())
            } else if forms.unit.len() > 1 {
                (money_str, forms.unit[1].clone())
            } else {
                (money_str, forms.unit[0].clone())
            };

            // `adjective` is ignored on this path — ES's int branch simply
            // never looks at CURRENCY_ADJECTIVES. Confirmed against CPython:
            // to_currency(12, currency="USD", adjective=True) == "doce dólares",
            // while the float path gives "doce US dólares y ...".
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        let result = self.base_to_currency(val, currency, cents, separator, adjective)?;
        // ES's fixup chain, in order and with Python's `str.replace`
        // semantics (every occurrence, plain substring). The order matters:
        // "veintiuno euro" must be caught before "uno euro" would turn
        // "veintiuno euros" into the unaccented "veintiun euros". Note each
        // pattern is the *singular*, which still matches inside the plural —
        // "veintiuno céntimos" contains "veintiuno céntimo", so 0.21 EUR comes
        // out "cero euros y veintiún céntimos".
        let result = result.replace("veintiuno euro", "veintiún euro");
        let result = result.replace("veintiuno céntimo", "veintiún céntimo");
        let result = result.replace("uno euro", "un euro");
        let result = result.replace("uno céntimo", "un céntimo");
        let result = result.replace("uno centavo", "un centavo");
        let result = result.replace("uno dólar", "un dólar");
        Ok(result)
    }

    /// `Num2Word_Base.to_currency`, reached through `Num2Word_ES`'s `super()`
    /// call and therefore only ever with a non-int value.
    ///
    /// Open-coded rather than delegating to `currency::default_to_currency`,
    /// which is **not** a faithful port of `base.py` at two points this
    /// language reaches:
    ///
    /// 1. It has no `has_decimal or right > 0` else-branch, so it always
    ///    renders a cents segment. `Decimal("5")` must render "cinco euros",
    ///    not "cinco euros y cero céntimos".
    /// 2. Its fractional-cents branch calls `pluralize(right, cr2)` where
    ///    base.py hardcodes `cr2[1]`.
    fn base_to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: &str,
        adjective: bool,
    ) -> Result<String> {
        let d = match val {
            CurrencyValue::Decimal { value: d, .. } => d.clone(),
            // `es_to_currency` owns the int case outright: its only delegation
            // to base is the missing-code fallback, which raises before it can
            // get here.
            CurrencyValue::Int(_) => unreachable!("ints never reach base_to_currency"),
        };

        // base.py's `CURRENCY_PRECISION.get(currency, 100) == 1 and not
        // isinstance(val, int)` pre-check is dead code for this language:
        // CURRENCY_PRECISION is `{}` the whole way up
        // ES_VE -> ES -> EUR -> Num2Word_Base, so the divisor is *always* 100.
        // JPY/KRW keep 2-decimal cents and KWD/BHD keep 2 rather than 3 — the
        // corpus pins both ("doce yenes y treinta y cuatro sen",
        // "doce dinares y treinta y cuatro fils").
        let divisor = self.currency_precision(currency);

        // has_fractional_cents = (decimal_val * divisor) % 1 != 0.
        // Decimal's `%` and BigDecimal's with_scale(0) both truncate toward
        // zero, so the two agree on negatives as well.
        let scaled = &d * BigDecimal::from(divisor);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        let (left, right, is_negative) = parse_currency_parts(
            &CurrencyValue::Decimal { value: d.clone(), has_decimal: true, is_float: true },
            false,
            has_fractional_cents,
            divisor,
        );

        // base.py looks the code up *after* parse_currency_parts; order kept
        // even though parse cannot raise.
        let forms = self
            .currency_forms(currency)
            .ok_or_else(|| self.currency_not_implemented(currency))?;
        let mut cr1 = forms.unit.clone();
        let cr2 = forms.subunit.clone();
        if adjective {
            if let Some(adj) = self.currency_adjective(currency) {
                cr1 = prefix_currency(adj, &cr1);
            }
        }

        // Here the minus *is* stripped, unlike ES's int branch above.
        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        let money_str = self.money_verbose(&left, currency)?;

        // `has_decimal = isinstance(val, float) or str(val).find(".") != -1`.
        //
        // The shim hands the core `str(value)` and a bare `is_int` flag, which
        // collapses float and Decimal into one BigDecimal — so the
        // `isinstance` half is not recoverable here. The scale reproduces the
        // `str(val)` half exactly: BigDecimal keeps the scale of the literal it
        // parsed, and scale > 0 holds for precisely the strings that carry a
        // "." ("1.0" -> 1, "5" -> 0, "5E+2" -> -2, matching str(Decimal(...))).
        //
        // Residual divergence, and its exact window: a *float* whose repr has
        // no "." — i.e. |v| >= 1e16 — is has_decimal=True in Python via the
        // isinstance clause but False here. It only changes the output when
        // right == 0 too, so the window is integral floats >= 1e16:
        // to_currency(1e16, "EUR") is "diez mil billones euros y cero
        // céntimos" in Python, "diez mil billones euros" here. Sub-1e-4 floats
        // also lose the ".", but they always carry fractional cents and so take
        // the same branch either way. No corpus row lands in the window; see
        // the port report's `concerns`.
        let has_decimal = d.as_bigint_and_exponent().1 > 0;

        // Python's `(isinstance(right, Decimal) and right > 0) or
        // (isinstance(right, int) and right > 0)` — right is always one or the
        // other, so both collapse to `right > 0`.
        if !has_decimal && right <= BigDecimal::zero() {
            return Ok(format!(
                "{}{} {}",
                minus_str,
                money_str,
                self.pluralize(&left, &cr1)?
            ));
        }

        let mut right = right;
        if has_fractional_cents {
            // Python's `isinstance(right, Decimal)`: right is a Decimal
            // exactly when parse_currency_parts was told keep_precision.
            let whole_cents = right.with_scale(0); // int(right), truncating
            let fractional_part = &right - &whole_cents;
            if fractional_part > BigDecimal::zero() {
                // to_cardinal(float(right)) — the float cardinal path, which is
                // a later phase, so this surfaces the default
                // `cardinal_from_decimal` NotImplementedError and the Python
                // shim falls back to the Python converter. See `concerns`.
                let cents_str = self.cardinal_from_decimal(&right)?;
                // base.py hardcodes cr2[1] here; no pluralize call.
                let sub = if cr2.len() > 1 { &cr2[1] } else { &cr2[0] };
                return Ok(format!(
                    "{}{} {}{} {} {}",
                    minus_str,
                    money_str,
                    self.pluralize(&left, &cr1)?,
                    separator,
                    cents_str,
                    sub
                ));
            }
            // Unreachable in practice: has_fractional_cents implies a nonzero
            // fractional part. Mirrored anyway.
            right = whole_cents;
        }

        let right_int = right.with_scale(0).as_bigint_and_exponent().0;
        let cents_str = if cents {
            self.cents_verbose(&right_int, currency)?
        } else {
            self.cents_terse(&right_int, currency)?
        };

        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str,
            money_str,
            self.pluralize(&left, &cr1)?,
            separator,
            cents_str,
            self.pluralize(&right_int, &cr2)?
        ))
    }
}

/// `10**n` as a BigInt.
fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

impl Lang for LangEsVe {

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "VES"
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
    /// Note that `Num2Word_ES.to_cardinal` is *not* overridden here even
    /// though Python overrides it — the Python override exists solely to
    /// consume the `_pending_ordinal` handshake set by `str_to_number`, which
    /// the stateless Rust path never sets. See the port report's `concerns`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext, cnum) = l;
        let (ntext, nnum) = r;
        let mut ctext = ctext.to_string();
        let mut ntext = ntext.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                // "uno" is absorbed: 1*100 -> "cien", 1*1000 -> "mil".
                return (ntext, nnum.clone());
            }
            // ...but not before a millón and up: "un millón".
            ctext = "un".to_string();
        } else if cnum == &hundred && !(nnum % &thousand).is_zero() {
            // Python: `elif cnum == 100 and not nnum % 1000 == 0`, i.e.
            // `not (nnum % 1000 == 0)`. "cien" -> "ciento" before a remainder,
            // but stays "cien" before "mil" (100_000 -> "cien mil").
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if cnum < &hundred {
                // Only sub-100 pairs take "y": 55 -> "cincuenta y cinco",
                // while 555 -> "quinientos cincuenta y cinco".
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && cnum > &BigInt::one() {
            // Pluralise the scale word: "millón" -> "millones",
            // "trillón" -> "trillones". Character slicing, not byte.
            ntext = format!("{}lones", drop_last_chars(&ntext, 3));
        }

        if nnum == &hundred {
            if cnum == &BigInt::from(5) {
                ctext = "quinien".to_string();
                ntext = String::new();
            } else if cnum == &BigInt::from(7) {
                ctext = "sete".to_string();
            } else if cnum == &BigInt::from(9) {
                ctext = "nove".to_string();
            }
            ntext.push('t');
            ntext.push_str(GENDER_STEM);
            ntext.push('s');
        } else {
            // Spanish drops the final 'o' of 'uno' (and adds an accent in
            // 'veintiuno') before any noun like 'mil' or 'millones':
            // 31000 -> "treinta y un mil", 21000 -> "veintiún mil".
            if nnum >= &thousand {
                if ctext.ends_with("veintiuno") {
                    ctext = format!("{}ún", drop_last_chars(&ctext, 3));
                } else if ctext.ends_with("uno") {
                    ctext = drop_last_chars(&ctext, 1);
                }
            }
            ntext = format!(" {}", ntext);
        }

        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        crate::lang_es::es_reject_zero_ordinal(value)?;
        // Python's default gender is "m".
        self.to_ordinal_gender(value, 'm')
    }

    /// `Num2Word_ES.to_ordinal_num(value, gender="m")` — the ordinal *stem* is
    /// never consulted, only the sign check and the gender marker.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        // gender defaults to "m" => gender_stem == "o" => "º".
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year` ignores `suffix`/`longval` entirely and just
    /// forwards to `to_cardinal`, so negative years read "menos quinientos"
    /// rather than taking a BC suffix.
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
        let gender = if kw.str("gender") == Some("f") { 'f' } else { 'm' };
        self.to_ordinal_gender(value, gender)
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

    /// `Num2Word_ES.to_fraction` (issue #584), inherited unchanged by
    /// `Num2Word_ES_VE`: idiomatic medio/tercio/cuarto for denominators
    /// 2/3/4, ordinal-as-noun otherwise (a bare "s" plural appended only when
    /// the ordinal isn't s-final already), and the apocopated "un" for
    /// numerator 1. `denominator == 1` (not -1!) or `numerator == 0`
    /// short-circuits to the plain cardinal.
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

    // ---- currency ------------------------------------------------------
    //
    // Not overridden, because Python does not override them either:
    //
    // * `currency_precision` — CURRENCY_PRECISION is `{}` on Num2Word_Base and
    //   is shadowed by nobody in the ES_VE -> ES -> EUR chain, so
    //   `.get(code, 100)` is always 100. No 3-decimal (KWD/BHD) or 0-decimal
    //   (JPY) handling exists for this language, and the corpus confirms it.
    // * `money_verbose` / `cents_verbose` — `_money_verbose` and
    //   `_cents_verbose` both just call `self.to_cardinal`, which is the
    //   trait default.
    // * `cents_terse` — `_cents_terse` is base's, and with divisor 100 the
    //   default `default_cents_terse` gives the same "%02d".
    // * `to_cheque` — neither ES nor ES_VE overrides `to_cheque`, and
    //   `currency::default_to_cheque` is a faithful port of base's. Note
    //   ES_VE's `.replace("uno", "un")` lives on `to_currency` only, so it
    //   does *not* touch cheques: to_cheque(1.0, "USD") is "UNO AND 00/100
    //   DÓLARES". All 9 cheque corpus rows verified against it.

    /// `self.__class__.__name__`, for the NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_ES_VE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `CURRENCY_ADJECTIVES`, inherited untouched from `Num2Word_EUR` —
    /// `Num2Word_ES` shadows `CURRENCY_FORMS` but *not* this, so the English
    /// adjectives survive into Spanish and `to_currency(12.34, "USD",
    /// adjective=True)` really does read "doce US dólares y treinta y cuatro
    /// centavos". Verified against CPython.
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        Some(match code {
            "AUD" => "Australian",
            "BYN" => "Belarusian",
            "CAD" => "Canadian",
            "EEK" => "Estonian",
            "USD" => "US",
            "RUB" => "Russian",
            "NOK" => "Norwegian",
            "MXN" => "Mexican",
            "RON" => "Romanian",
            "INR" => "Indian",
            "HUF" => "Hungarian",
            "ISK" => "íslenskar",
            "UZS" => "Uzbekistan",
            "SAR" => "Saudi",
            "JPY" => "Japanese",
            "KRW" => "Korean",
            _ => return None,
        })
    }

    /// `Num2Word_EUR.pluralize` — `forms[0]` for exactly 1, `forms[1]`
    /// otherwise (so 0 takes the plural: "cero euros").
    ///
    /// Every entry in this language's table is arity 2, so the IndexError can
    /// only be reached through an `adjective` prefix of a malformed tuple —
    /// `prefix_currency` preserves arity, so not at all. Mapped anyway rather
    /// than panicking.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES_VE.to_currency`.
    ///
    /// # The `separator` default
    ///
    /// Python's signature is
    /// `to_currency(val, currency="VES", cents=True, separator=" y",
    /// adjective=False, old=False)`, and the `num2words()` dispatcher does not
    /// pass a separator unless the caller did — so the corpus rows all carry
    /// " y". The trait has no way to express "argument omitted": both the
    /// Python shim (`kwargs.get("separator", ",")`) and `bench/diff_test.py`
    /// hand us `Num2Word_Base`'s `","` when the caller specified nothing.
    ///
    /// So "," is treated as that omission sentinel and mapped to this class's
    /// own default. This is right for the two cases that occur — no separator
    /// (-> " y", matching Python) and an explicit `separator=" con"` (passed
    /// through, matching Python) — and wrong only for an explicit
    /// `separator=","`, which renders " y". Flagged in the port report; the
    /// clean fix is in the shim, which is outside this file.
    ///
    /// `old=False` cannot vary through *this* entry point (it only forwards
    /// currency/cents/separator/adjective); the `old` -> VES-becomes-VEB
    /// rewrite lives in `to_currency_kw` below, which the kwargs dispatcher
    /// calls. VEB is in the forms table and also works as a direct code.
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
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        let result = self.es_to_currency(val, currency, cents, separator, adjective)?;

        // ES_VE's parting shot, and the whole reason this subclass's floats
        // read correctly where ES's do not: a blanket, whole-string,
        // every-occurrence replace. ES only fixes "uno" before euro/céntimo/
        // centavo/dólar, so "uno libra" survives it; this catches the rest.
        //
        // It is indiscriminate, and that is load-bearing: it also rewrites
        // "veintiuno" -> "veintiun" (accent restored below, #253), so in
        // Python to_currency(21, "EUR") == "veintiun
        // euros" (no accent) while the float 21.0 == "veintiún euros y cero
        // céntimos" (ES's accented fixup got there first). Both verified
        // against CPython.
        // The blanket rewrite leaves "veintiun", which is never written
        // without its accent: "veintiún dólares" (#253).
        Ok(result.replace("uno", "un").replace("veintiun ", "veintiún "))
    }

    /// `Num2Word_ES_VE.to_currency(..., old=False)` — the one extra kwarg in
    /// the whole ES family (corpus: kwargs slice). Python's `if old:` is a
    /// *truthiness* test, and the rewrite only touches the default code:
    /// `currency = "VEB" if currency == "VES" else currency`. An explicit
    /// non-VES code is passed through untouched even with `old=True`. VEB and
    /// VES share the ("bolívar", "bolívares") forms, so the output is
    /// identical either way — reproduced faithfully anyway.
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["old"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        // Python truthiness of the `old` value (bool/int/str/list/None).
        let old = match kw.get("old") {
            Some(KwVal::Bool(b)) => *b,
            Some(KwVal::Int(i)) => *i != 0,
            Some(KwVal::Str(s)) => !s.is_empty(),
            Some(KwVal::List(l)) => !l.is_empty(),
            Some(KwVal::None) | None => false,
        };
        let currency = if old && currency == "VES" {
            "VEB"
        } else {
            currency
        };
        self.to_currency(val, currency, cents, separator, adjective)
    }
}
