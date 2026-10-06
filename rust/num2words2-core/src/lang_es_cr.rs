//! Port of `lang_ES_CR.py` (Spanish — Costa Rica).
//!
//! Registry check: `__init__.py` maps `"es_CR" -> lang_ES_CR.Num2Word_ES_CR()`,
//! so this file ports `Num2Word_ES_CR` — the class the key actually resolves to.
//!
//! **`Num2Word_ES_CR` overrides only `CURRENCY_FORMS` and `to_currency`.** For
//! `to_cardinal` / `to_ordinal` / `to_ordinal_num` / `to_year` this language is
//! byte-for-byte `Num2Word_ES`. Everything below is therefore a port of
//! `lang_ES.py` via its `lang_EUR` → `Num2Word_Base` ancestry. (`lang_EU.py` is
//! Basque and is *not* in this chain despite the similar name.)
//!
//! Shape: **engine**. `Num2Word_ES.setup` defines `high_numwords` /
//! `mid_numwords` / `low_numwords` and `merge`, and lets
//! `Num2Word_Base.to_cardinal` drive `splitnum`/`clean`. `to_ordinal`,
//! `to_ordinal_num` and `to_year` are overridden outright.
//!
//! # Card table
//!
//! `setup` calls `gen_high_numwords([], [], lows)`. With `units == tens == []`
//! the comprehension `[u + t for t in tens for u in units]` is empty, so the
//! Latin-prefix elision machinery is a no-op and the function just returns
//! `lows`. Those nine stems then go through `Num2Word_EUR.set_high_numwords`
//! with `GIGA_SUFFIX = None` (long-scale "illiard" rungs suppressed) and
//! `MEGA_SUFFIX = "illón"`, yielding one card every 10^6 from 10^6 to 10^54.
//!
//! `MAXVAL = 1000 * list(self.cards.keys())[0]`. Python indexes the *first
//! inserted* key of an `OrderedDict`, which here is also the largest (10^54),
//! so `Cards::highest()` reproduces it exactly: MAXVAL == 10^57.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are exactly what Python
//! emits, verified against the interpreter:
//!
//! 1. **`to_ordinal(20)` == "vigesimo", not "vigésimo".** The `value <= 29`
//!    branch does `self.ords[dec].replace("é", "e")` to handle Spanish
//!    *sobreesdrújula* orthography, and 20 lands in that branch, so the accent
//!    is stripped even though nothing is suffixed. 30..=100 keep the accent
//!    ("trigésimo"). Likewise `to_ordinal(120)` == "centésimo vigesimo".
//! 2. **Missing space before "milésimo"/"millonésimo"/…** The `value < 1e18`
//!    branch formats `"%s%s%s %s" % (cardinal, ords[dec], gender_stem, ...)`
//!    with no separator after `cardinal`, so `to_ordinal(2000)` ==
//!    "dosmilésimo" and `to_ordinal(99999)` ==
//!    "noventa y nuevemilésimo noningentésimo nonagésimo noveno".
//! 3. **`.replace("oo", "o")`** is applied at *every* recursion level, not just
//!    the top. It exists for "decimooctavo" -> "decimoctavo" (and
//!    "vigesimooctavo" -> "vigesimoctavo"), but it is an unconditional global
//!    replace, not a targeted fix.
//! 4. **`to_ordinal(0)`** raises ValueError here (Python returns `""`; #160). `to_ordinal_num(0)` == "0º".
//! 5. **`errmsg_toobig` reads "deber ser inferior"**, not "debe ser inferior" —
//!    a typo in the Python source, preserved verbatim in [`LangEsCr::to_cardinal`].
//! 6. **`ords` keys 1e3..1e15 are Python floats** (`1e3`, `1e6`, …) yet are
//!    looked up with `int` values. Python dicts hash `1000 == 1000.0` to the
//!    same slot, so the lookups succeed; the `BigInt` keys here are the
//!    equivalent.
//!
//! # `int(math.log(value, 1000))` — float error is load-bearing
//!
//! The `value < 1e18` branch picks its scale with
//! `dec = 1000 ** int(math.log(int(value), 1000))`. That is a *float* log, and
//! it disagrees with the exact `floor(log1000(v))` on two contiguous ranges,
//! both of which are observable. Verified exhaustively at the thresholds and
//! against 535k sampled values (see [`py_log1000`]):
//!
//! * `v` in `[999999999999996, 10^15)` -> returns 5 (exact: 4), so
//!   `dec = 10^15 > v`, giving `high_part == 0` and `low_part == v`. The
//!   recursive `to_ordinal(low_part)` is then called with the *same* value:
//!   Python raises **`RecursionError`**.
//! * `v` in `[999999999999995072, 10^18)` -> returns 6 (exact: 5), so
//!   `dec = 10^18`, which is absent from `ords`: Python raises
//!   **`KeyError: 1000000000000000000`**. This fires *before* the recursion
//!   because the `%`-format tuple is evaluated left to right and `ords[dec]`
//!   precedes `to_ordinal(low_part)`.
//!
//! `RecursionError` has no `N2WError` variant (PORTING.md maps neither it nor
//! its `RuntimeError` base). Letting it recur for real would overflow the Rust
//! stack and abort the process, which is strictly worse than any error, so
//! [`LangEsCr::to_ordinal_gender`] guards on the exact trigger condition
//! (`high_part == 0`, checked *after* the `ords` lookup so the `KeyError` range
//! still reports `Key`) and returns `N2WError::Value` carrying the Python
//! exception name. **This is the one place where the error *type* is knowingly
//! not reproducible** — flagged in the report.
//!
//! # Currency
//!
//! `to_currency` is a three-layer sandwich, and every layer is observable:
//!
//! ```text
//! Num2Word_ES_CR.to_currency   separator=" y", currency="CRC"
//!   -> Num2Word_ES.to_currency     int arm hand-rolled; float arm -> super()
//!        -> Num2Word_Base.to_currency
//!        <- six literal .replace() fixups          (ES)
//!   <- one blanket .replace("uno", "un")           (ES_CR)
//! ```
//!
//! `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
//! inherited from `Num2Word_Base` untouched, and `pluralize` from
//! `Num2Word_EUR`; the trait defaults already mirror those, so they are not
//! re-implemented here. Note that `to_cheque` reaches `to_cardinal` through
//! `_money_verbose` and therefore gets **none** of the un/uno fixups:
//! `to_cheque(1.0, "EUR")` is `"UNO AND 00/100 EUROS"`, not `"UN ..."`.
//!
//! ## `CURRENCY_FORMS` shadows, it does not extend
//!
//! `Num2Word_ES_CR.CURRENCY_FORMS` is a class attribute **of its own**, so
//! attribute lookup stops at `Num2Word_ES_CR`: neither `Num2Word_ES`'s
//! ~150-entry table nor the `lang_EUR` dict that `Num2Word_EN.__init__` mutates
//! in place is ever consulted. Only CRC/EUR/USD resolve — GBP, JPY, KWD, BHD,
//! INR, CNY and CHF all raise NotImplementedError here even though `lang_ES.py`
//! defines every one of them. The corpus records exactly that.
//!
//! `CURRENCY_PRECISION` is likewise never defined down this chain, so it is
//! `Num2Word_Base`'s empty dict and every code has divisor 100. (EN *rebinds*
//! rather than mutates it, so EN's mils table does not leak.) The 3-decimal and
//! 0-decimal branches are therefore unreachable for this language — doubly so,
//! since KWD/BHD/JPY are not in the forms table to begin with.
//!
//! `CURRENCY_ADJECTIVES` *is* inherited — from `Num2Word_EUR`, unmutated, since
//! neither ES nor ES_CR defines one. Of the three codes that resolve, only USD
//! carries an adjective ("US"), and it is reachable only on the float arm.
//!
//! ## Faithfully reproduced Python quirks (currency)
//!
//! All verified against the interpreter:
//!
//! 7. **Double space after "menos" on negative integers (fixed, #160).**
//!    `Num2Word_ES`'s integer arm sets `minus_str = self.negword` — the *raw*
//!    value, trailing space and all — so Python gives `to_currency(-1, "EUR")`
//!    == `"menos un euro"`. The port strips negword as the float arm does, so
//!    both give a single space: `"menos un euro"`.
//! 8. **`adjective=` is ignored on the integer arm.** `Num2Word_ES` never reads
//!    `CURRENCY_ADJECTIVES` there, so `to_currency(2, "USD", adjective=True)` is
//!    `"dos dólares"`, while the float `to_currency(12.34, "USD",
//!    adjective=True)` *is* prefixed: `"doce US dólares y ..."`.
//! 9. **"colónes" is misspelled.** RAE has "colones", and `lang_ES.py` spells
//!    the same entry correctly; `lang_ES_CR.py` does not. It is what the class
//!    emits, so it is what this port emits.
//! 10. **The un/uno fixups are blind substring replaces**, and the two layers
//!     disagree with each other. See [`es_uno_fixups`] and
//!     [`Lang::to_currency`].
//!
//! ## Out of scope: fractional cents
//!
//! `to_currency(1.011, "EUR")` is reachable and Python answers
//! `"un euro y un punto un céntimos"` — `_money_verbose` is fine but the cents
//! word comes from `self.to_cardinal(float(right))`, i.e. Base's float path.
//! `cardinal_from_decimal` is left at its trait default (which routes to
//! `floatpath`) per the porting contract; the fixup layers above still apply on
//! top of whatever it returns. No corpus row exercises this. Flagged in the
//! report.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError,
    Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `setup` and never mutated.
///
/// `to_ordinal`/`to_ordinal_num` shadow it with a *local* `gender_stem`, so the
/// instance attribute stays "o" for the whole lifetime of the converter and
/// `merge` can treat it as a constant.
const GENDER_STEM: &str = "o";

fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

/// Python's `s[:-n]` — slices by **character**, not byte.
///
/// Load-bearing: `merge` does `ntext[:-3] + "lones"` on words like "millón"
/// (6 chars / 7 bytes, because "ó" is two bytes). Byte slicing would yield
/// "mill" + "lones" and could split the "ó" mid-codepoint.
fn drop_last_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    s.chars().take(count.saturating_sub(n)).collect()
}

/// Reproduces `int(math.log(int(v), 1000))` for `1000 < v < 10^18`.
///
/// Exact everywhere except the two ranges documented in the module header,
/// where CPython's float log overshoots the true floor. Modelling those
/// explicitly is cheaper and more predictable than trying to re-derive f64
/// rounding, and it was checked exhaustively at both thresholds.
fn py_log1000(v: &BigInt) -> u32 {
    // Ordered high-to-low; the two ranges do not overlap.
    if *v >= BigInt::from(999_999_999_999_995_072u64) && *v < pow10(18) {
        return 6;
    }
    if *v >= BigInt::from(999_999_999_999_996u64) && *v < pow10(15) {
        return 5;
    }
    let thousand = BigInt::from(1000);
    let mut k = 0u32;
    let mut p = BigInt::one();
    loop {
        let next = &p * &thousand;
        if next <= *v {
            p = next;
            k += 1;
        } else {
            return k;
        }
    }
}

/// `Num2Word_ES_CR.CURRENCY_FORMS`, verbatim — all three entries of it.
///
/// This is the whole table, not a delta: the class attribute shadows
/// `Num2Word_ES`'s (see module header), which is why so many corpus rows are
/// NotImplementedError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTIMOS: [&str; 2] = ["céntimo", "céntimos"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    // "colónes" is a typo for "colones" (quirk 9) — preserved.
    m.insert("CRC", CurrencyForms::new(&["colón", "colónes"], &CENTIMOS));
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &CENTIMOS));
    m.insert(
        "USD",
        CurrencyForms::new(&["dólar", "dólares"], &["centavo", "centavos"]),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged.
///
/// Neither `Num2Word_ES` nor `Num2Word_ES_CR` defines or mutates it, and
/// `Num2Word_EN.__init__` only ever writes to `CURRENCY_FORMS`, so this dict is
/// exactly EUR's class body. Fifteen of the sixteen codes are unreachable here
/// (they are not in `CURRENCY_FORMS`); only USD can ever match. It is ported in
/// full anyway, because the lookup Python performs is against this table and
/// trimming it would encode an inference rather than the data.
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

/// The six `str.replace` calls closing `Num2Word_ES.to_currency`'s float arm,
/// in source order.
///
/// Both the order and the blind-substring semantics are load-bearing; these
/// read like word-aware apocope rules but are not, and the difference shows up
/// in real corpus-adjacent values:
///
/// * `"veintiuno dólares y uno centavo"` (21.01 USD): the `"uno centavo"` rule
///   fires first, then `"uno dólar"` matches **inside** `"veintiuno dólares"`
///   (`veinti|uno dólar|es`), giving `"veintiun dólares y un centavo"`.
/// * The `"veintiuno euro"`/`"veintiuno céntimo"` rules exist only for EUR and
///   céntimos, so 21 cents renders "veintiún" against EUR but "veintiun"
///   against USD: `to_currency(21.21, "EUR")` == `"veintiún euros y veintiún
///   céntimos"` while `to_currency(21.21, "USD")` == `"veintiun dólares y
///   veintiun centavos"` — accented in one, unaccented in the other, because
///   the accented rewrites simply never mention "centavo" or "dólar".
///
/// Rewriting this as a rule would change output. It is a transcription.
fn es_uno_fixups(s: &str) -> String {
    s.replace("veintiuno euro", "veintiún euro")
        .replace("veintiuno céntimo", "veintiún céntimo")
        .replace("uno euro", "un euro")
        .replace("uno céntimo", "un céntimo")
        .replace("uno centavo", "un centavo")
        .replace("uno dólar", "un dólar")
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

pub struct LangEsCr {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<BigInt, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEsCr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsCr {
    pub fn new() -> Self {
        // setup(): gen_high_numwords([], [], lows) == lows, since both the
        // units and tens lists are empty (see module header).
        let lows = [
            "non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m",
        ];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords, with GIGA_SUFFIX=None / MEGA_SUFFIX="illón":
        //   cap = 3 + 6*len(high)                       -> 57
        //   for word, n in zip(high, range(cap, 3, -6)) -> n = 57, 51, ..., 9
        //       cards[10**(n-3)] = word + "illón"       -> 10^54 ... 10^6
        // Both sides of the zip have 9 elements, so neither truncates.
        let cap: u32 = 3 + 6 * lows.len() as u32;
        let mut n = cap;
        for word in lows.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(pow10(n - 3), format!("{}illón", word));
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

        // MAXVAL = 1000 * list(self.cards.keys())[0]; the first-inserted key is
        // also the largest (10^54), so this is 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let mut ords: HashMap<BigInt, &'static str> = HashMap::new();
        for (k, v) in [
            (1i64, "primer"),
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
        ] {
            ords.insert(BigInt::from(k), v);
        }
        // Python spells these as floats (1e3, 1e6, ...) but looks them up with
        // ints; dict hashing makes the two equivalent.
        ords.insert(pow10(3), "milésim");
        ords.insert(pow10(6), "millonésim");
        ords.insert(pow10(9), "billonésim");
        ords.insert(pow10(12), "trillonésim");
        ords.insert(pow10(15), "cuadrillonésim");

        LangEsCr {
            cards,
            maxval,
            ords,
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            // Built once here, never per call. `to_currency` only reads these.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`errmsg_floatord`) is unreachable for `BigInt` input;
    /// only the negative check can fire, and it raises `TypeError` — not
    /// `ValueError` — which is what the corpus records for every negative
    /// `ordinal` / `ordinal_num` row.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `self.ords[key]`, raising `KeyError` when absent (as Python does for
    /// `dec == 10^18`).
    fn ord_word(&self, key: &BigInt) -> Result<&'static str> {
        self.ords
            .get(key)
            .copied()
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `Num2Word_ES.to_ordinal(value, gender="m")`.
    ///
    /// The dispatcher only ever calls with the default gender, but the method
    /// recurses passing `gender` through, so the parameter is kept internally.
    fn to_ordinal_gender(&self, value: &BigInt, gender: &str) -> Result<String> {
        let mut gender_stem = if gender == "f" { "a" } else { "o" };
        self.verify_ordinal(value)?;

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);

        let text: String = if value.is_zero() {
            String::new()
        } else if *value <= ten {
            format!("{}{}", self.ord_word(value)?, gender_stem)
        } else if *value <= BigInt::from(29) {
            // RAE: simple forms preferred up to 30. The unconditional
            // `.replace("é", "e")` is what makes 20 render as "vigesimo".
            gender_stem = "o";
            let dec = (value / &ten) * &ten;
            format!(
                "{}{}{}",
                self.ord_word(&dec)?.replace('é', "e"),
                gender_stem,
                self.to_ordinal_gender(&(value % &ten), gender)?
            )
        } else if *value <= hundred {
            let dec = (value / &ten) * &ten;
            format!(
                "{}{} {}",
                self.ord_word(&dec)?,
                gender_stem,
                self.to_ordinal_gender(&(value - &dec), gender)?
            )
        } else if *value <= thousand {
            let cen = (value / &hundred) * &hundred;
            format!(
                "{}{} {}",
                self.ord_word(&cen)?,
                gender_stem,
                self.to_ordinal_gender(&(value - &cen), gender)?
            )
        } else if *value < pow10(18) {
            // Round down to the nearest 1e(3n). See py_log1000 for why this is
            // not simply floor(log1000(value)).
            let dec = pow10(3 * py_log1000(value));
            let high_part = value / &dec;
            let low_part = value % &dec;

            // Python evaluates the %-format operands left to right:
            //   cardinal -> ords[dec] -> gender_stem -> to_ordinal(low_part)
            // so `cardinal` is computed first and `ords[dec]` can raise before
            // the recursion is ever entered. Order preserved here.
            let cardinal = if !high_part.is_one() {
                self.to_cardinal(&high_part)?
            } else {
                String::new()
            };
            let ordw = self.ord_word(&dec)?;
            if high_part.is_zero() {
                // dec > value: Python calls to_ordinal(low_part) with
                // low_part == value and recurses until RecursionError.
                return Err(N2WError::Value(format!(
                    "RecursionError: maximum recursion depth exceeded (to_ordinal({}))",
                    value
                )));
            }
            format!(
                "{}{}{} {}",
                cardinal,
                ordw,
                gender_stem,
                self.to_ordinal_gender(&low_part, gender)?
            )
        } else {
            self.to_cardinal(value)?
        };

        // Applied at every level, not just the top: "decimooctavo" ->
        // "decimoctavo".
        Ok(text.trim().replace("oo", "o"))
    }

    /// `Num2Word_ES.to_currency` — i.e. the `super()` call that
    /// `Num2Word_ES_CR.to_currency` makes. Kept as its own method so the two
    /// classes' fixup layers stay visibly distinct; they do *not* compose into
    /// one rule (see [`Lang::to_currency`]).
    ///
    /// `separator` is already resolved to a concrete `&str` by the caller.
    fn es_to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: &str,
        adjective: bool,
    ) -> Result<String> {
        // `if isinstance(val, int)` — pure ints only. A float that happens to
        // be whole (1.0) takes the arm below and still prints cents.
        if let CurrencyValue::Int(v) = val {
            // Python: `except (KeyError, AttributeError): return
            // super().to_currency(...)`. Base repeats the lookup and converts
            // the KeyError into NotImplementedError, so delegating keeps that
            // message coming from one place instead of duplicating it. (The
            // AttributeError half is dead code: `CURRENCY_FORMS` always
            // resolves through the MRO.)
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };
            let cr1 = &forms.unit;

            // `minus_str = self.negword if val < 0 else ""` — raw in Python,
            // stripped here. Quirk 7 (fixed).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();
            // Python computes this unconditionally and then throws it away when
            // abs_val == 1. It can raise OverflowError first, and the lookup
            // above can raise NotImplementedError before *it*, so the order of
            // all three is observable: to_currency(10**60, "GBP") is
            // NotImplementedError, to_currency(10**60, "EUR") is OverflowError.
            let cardinal = self.to_cardinal(&abs_val)?;

            let (money_str, currency_str) = if abs_val.is_one() {
                // `money_str = "un"`: a hand-written apocope that discards
                // to_cardinal(1) == "uno".
                ("un".to_string(), cr1.first())
            } else {
                // `cr1[1] if len(cr1) > 1 else cr1[0]`.
                (cardinal, cr1.get(1).or_else(|| cr1.first()))
            };
            let currency_str =
                currency_str.ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            // `adjective` is deliberately not consulted here — Python's integer
            // arm never reads CURRENCY_ADJECTIVES (quirk 8). It *is* honoured on
            // the float arm below.
            //
            // `("%s %s %s" % (...)).strip()`: trim() matches strip() at the ends.
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats/Decimals: `super(Num2Word_ES, self).to_currency(...)`, which is
        // `Num2Word_Base.to_currency` (EUR defines none). This is the arm that
        // honours `adjective=`, renders the cents segment, and uses the *trimmed*
        // negword.
        let result = default_to_currency(self, val, currency, cents, separator, adjective)?;
        Ok(es_uno_fixups(&result))
    }
}

impl Lang for LangEsCr {

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "CRC"
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
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if *nnum < million {
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        } else if *cnum == hundred && !(nnum % &thousand).is_zero() {
            // "cien" -> "ciento", but only when the tail is not a round
            // thousand: 100_000 stays "cien mil".
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if *cnum < hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && *cnum > BigInt::one() {
            // "millón" -> "millones", "billón" -> "billones", ... Character
            // slicing is required here ("ó" is two bytes).
            ntext = format!("{}lones", drop_last_chars(&ntext, 3));
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
            ntext.push('t');
            ntext.push_str(GENDER_STEM);
            ntext.push('s');
        } else {
            // Spanish drops the final 'o' of 'uno' (and adds an accent in
            // 'veintiuno') before a noun like 'mil' or 'millones':
            // 31000 -> "treinta y un mil", 21000 -> "veintiún mil".
            if *nnum >= thousand {
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

    /// `Num2Word_ES.to_cardinal` -> `Num2Word_Base.to_cardinal`.
    ///
    /// The Python override exists only to service the `_pending_ordinal`
    /// handshake set by `str_to_number` (see the report): with no string
    /// parsing in this path, `_pending_ordinal` is always `None` and the
    /// override degenerates to `super().to_cardinal(value)`.
    ///
    /// The overflow test is duplicated ahead of `default_to_cardinal` purely to
    /// carry Spanish `errmsg_toobig` — including its "deber ser" typo. The
    /// condition is identical to the base engine's, so the inner check can
    /// never fire.
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
        self.to_ordinal_gender(value, "m")
    }

    /// `Num2Word_ES.to_ordinal_num`: the masculine indicator "º" (U+00BA
    /// MASCULINE ORDINAL INDICATOR), never the digit-zero lookalike.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year(val, suffix=None, longval=True)` ignores both
    /// keyword arguments and is a plain cardinal — no BC/AD suffix, no
    /// "nineteen-ninety-nine" pairing. Negative years therefore render with
    /// `negword`: `to_year(-500)` == "menos quinientos".
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
        let gender = if kw.str("gender") == Some("f") { "f" } else { "m" };
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
    /// `Num2Word_ES_CR`: idiomatic medio/tercio/cuarto for denominators
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

    // ---- currency -------------------------------------------------------
    //
    // `to_cheque`, `money_verbose`, `cents_verbose` and `cents_terse` are
    // inherited from `Num2Word_Base` and `currency_precision` is Base's empty
    // dict, so the trait defaults already match — only the tables, the class
    // name, EUR's plural rule and the two-layer `to_currency` are overridden.

    /// `self.__class__.__name__`, for the NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_ES_CR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a single-form entry with `n != 1`
    /// would raise IndexError. All three of this language's entries carry two
    /// forms, so that is unreachable — mapped to `Index` rather than panicking
    /// in case the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES_CR.to_currency`: delegate to `Num2Word_ES`, then
    /// `result.replace("uno", "un")`.
    ///
    /// That final replace is unconditional and untargeted — it rewrites every
    /// "uno" anywhere in the string, including inside "veintiuno" and in the
    /// middle of a compound. It is *also* applied on top of ES's own six
    /// fixups, and the two layers do not agree:
    ///
    /// * Integer arm — ES does nothing, so this rule alone acts:
    ///   `to_currency(21, "EUR")` == `"veintiun euros"` (no accent).
    /// * Float arm — ES's `"veintiuno euro"` -> `"veintiún euro"` rewrite has
    ///   already consumed the "uno", so this rule finds nothing:
    ///   `to_currency(21.21, "EUR")` == `"veintiún euros y veintiún céntimos"`
    ///   (accented). Same number, same currency, different accent by arm.
    /// * It fires on ordinary compounds too: `to_currency(1000001, "EUR")` ==
    ///   `"un millón un euros"`, from "un millón uno".
    ///
    /// No currency word in this language's table contains "uno", so the unit and
    /// subunit names survive the replace intact — but that is luck, not design.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` == the caller omitted `separator=`, so this language's own
        // default (" y") applies. The trait resolves that for the *default*
        // body; an override has to do it itself.
        let separator = separator.unwrap_or(self.default_separator());
        let result = self.es_to_currency(val, currency, cents, separator, adjective)?;
        Ok(result.replace("uno", "un"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floatpath::FloatValue;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A Python `int` arg (a corpus `arg` that `repr`s without a dot).
    fn int(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, true, false, false).unwrap()
    }

    /// Build a `FloatValue::Float` the way the binding does: the raw f64 plus
    /// the repr-derived precision (`abs(Decimal(str(v)).as_tuple().exponent)`).
    fn flt(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    /// Build a `FloatValue::Decimal` from the literal decimal string; precision
    /// is `abs(exponent)` of that literal.
    fn dec(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// Every `"lang": "es_CR", "to": "cardinal"` corpus row whose `arg` is a
    /// non-integral float — i.e. the ones that actually reach the float path
    /// (whole floats like `1.0`/`0.0` are routed to the integer `to_cardinal`
    /// by Python's `assert int(value) == value`, so they are not here).
    ///
    /// `Num2Word_ES` overrides neither `to_cardinal_float` nor `float2tuple`,
    /// and its `to_cardinal` override degenerates to `super().to_cardinal` for
    /// non-string input, so es_CR's float path *is* `Num2Word_Base`'s. This
    /// locks that the inherited trait default reproduces it byte for byte,
    /// including the f64-artefact rescues for 1.005 / 2.675.
    #[test]
    fn cardinal_float_corpus_rows() {
        let l = LangEsCr::new();
        let rows: Vec<(FloatValue, &str)> = vec![
            (flt(0.5, 1), "cero punto cinco"),
            (flt(1.5, 1), "uno punto cinco"),
            (flt(2.25, 2), "dos punto dos cinco"),
            (flt(3.14, 2), "tres punto uno cuatro"),
            (flt(0.01, 2), "cero punto cero uno"),
            (flt(0.1, 1), "cero punto uno"),
            (flt(0.99, 2), "cero punto nueve nueve"),
            (flt(1.01, 2), "uno punto cero uno"),
            (flt(12.34, 2), "doce punto tres cuatro"),
            (flt(99.99, 2), "noventa y nueve punto nueve nueve"),
            (flt(100.5, 1), "cien punto cinco"),
            (
                flt(1234.56, 2),
                "mil doscientos treinta y cuatro punto cinco seis",
            ),
            (flt(-0.5, 1), "menos cero punto cinco"),
            (flt(-1.5, 1), "menos uno punto cinco"),
            (flt(-12.34, 2), "menos doce punto tres cuatro"),
            // f64 artefacts: 1.005 -> 4.999.../1000 rescued to 005, 2.675 ->
            // 674.9999999999998 rescued to 675 by the `< 0.01` heuristic.
            (flt(1.005, 3), "uno punto cero cero cinco"),
            (flt(2.675, 3), "dos punto seis siete cinco"),
        ];
        for (v, want) in rows {
            let got = l.to_cardinal_float(&v, None).unwrap();
            assert_eq!(got, want, "to_cardinal_float({:?})", v);
        }
    }

    /// Every `"lang": "es_CR", "to": "cardinal_dec"` corpus row — Decimal input,
    /// the exact arbitrary-precision arm (never a float cast). Includes the
    /// issue #603 trillion-scale value that a float cast would corrupt.
    #[test]
    fn cardinal_dec_corpus_rows() {
        let l = LangEsCr::new();
        let rows: Vec<(FloatValue, &str)> = vec![
            (dec("0.01", 2), "cero punto cero uno"),
            (dec("1.10", 2), "uno punto uno cero"),
            (dec("12.345", 3), "doce punto tres cuatro cinco"),
            (
                dec("98746251323029.99", 2),
                "noventa y ocho billones setecientos cuarenta y seis mil \
                 doscientos cincuenta y un millones trescientos veintitrés mil \
                 veintinueve punto nueve nueve",
            ),
            (dec("0.001", 3), "cero punto cero cero uno"),
        ];
        for (v, want) in rows {
            let got = l.to_cardinal_float(&v, None).unwrap();
            assert_eq!(got, want, "to_cardinal_float({:?})", v);
        }
    }

    /// A Python `float` arg. `has_decimal` is `isinstance(val, float) or "." in
    /// str(val)`, so it is unconditionally true for a float — including `1e21`,
    /// whose repr carries no dot at all.
    fn float(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, false, true, true).unwrap()
    }

    /// Every succeeding `"lang": "es_CR", "to": "currency:*"` row of the frozen
    /// corpus, verbatim.
    #[test]
    fn currency_corpus_rows() {
        let l = LangEsCr::new();
        let rows: Vec<(&str, CurrencyValue, &str)> = vec![
            ("EUR", int("0"), "cero euros"),
            ("EUR", int("1"), "un euro"),
            ("EUR", int("2"), "dos euros"),
            ("EUR", int("100"), "cien euros"),
            ("EUR", float("12.34"), "doce euros y treinta y cuatro céntimos"),
            ("EUR", float("0.01"), "cero euros y un céntimo"),
            ("EUR", float("1.0"), "un euro y cero céntimos"),
            (
                "EUR",
                float("99.99"),
                "noventa y nueve euros y noventa y nueve céntimos",
            ),
            (
                "EUR",
                float("1234.56"),
                "mil doscientos treinta y cuatro euros y cincuenta y seis céntimos",
            ),
            (
                "EUR",
                float("-12.34"),
                "menos doce euros y treinta y cuatro céntimos",
            ),
            ("EUR", int("1000000"), "un millón euros"),
            ("EUR", float("0.5"), "cero euros y cincuenta céntimos"),
            ("USD", int("0"), "cero dólares"),
            ("USD", int("1"), "un dólar"),
            ("USD", int("2"), "dos dólares"),
            ("USD", int("100"), "cien dólares"),
            (
                "USD",
                float("12.34"),
                "doce dólares y treinta y cuatro centavos",
            ),
            ("USD", float("0.01"), "cero dólares y un centavo"),
            ("USD", float("1.0"), "un dólar y cero centavos"),
            (
                "USD",
                float("99.99"),
                "noventa y nueve dólares y noventa y nueve centavos",
            ),
            (
                "USD",
                float("1234.56"),
                "mil doscientos treinta y cuatro dólares y cincuenta y seis centavos",
            ),
            (
                "USD",
                float("-12.34"),
                "menos doce dólares y treinta y cuatro centavos",
            ),
            ("USD", int("1000000"), "un millón dólares"),
            ("USD", float("0.5"), "cero dólares y cincuenta centavos"),
        ];
        for (cur, val, want) in rows {
            let got = l.to_currency(&val, cur, true, None, false).unwrap();
            assert_eq!(got, want, "to_currency({:?}, {})", val, cur);
        }
    }

    /// The seven codes the corpus records as NotImplementedError, across all
    /// twelve `arg` values it pairs them with — 84 rows.
    ///
    /// Every one of these codes *is* defined in `lang_ES.py`;
    /// `Num2Word_ES_CR.CURRENCY_FORMS` shadows that table rather than extending
    /// it, so they are unreachable here. KWD/BHD (divisor 1000 elsewhere) and
    /// JPY (divisor 1) are among them, which is why this language never
    /// exercises the 3-decimal or 0-decimal branches.
    #[test]
    fn currency_codes_shadowed_away() {
        let l = LangEsCr::new();
        let args = || {
            vec![
                int("0"),
                int("1"),
                int("2"),
                int("100"),
                float("12.34"),
                float("0.01"),
                float("1.0"),
                float("99.99"),
                float("1234.56"),
                float("-12.34"),
                int("1000000"),
                float("0.5"),
            ]
        };
        let mut n = 0;
        for cur in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for val in args() {
                match l.to_currency(&val, cur, true, None, false) {
                    Err(N2WError::NotImplemented(m)) => assert_eq!(
                        m,
                        format!(
                            "Currency code \"{}\" not implemented for \"Num2Word_ES_CR\"",
                            cur
                        )
                    ),
                    other => panic!("{} {:?}: want NotImplemented, got {:?}", cur, val, other),
                }
                n += 1;
            }
        }
        assert_eq!(n, 84, "corpus has 84 NotImplementedError currency rows");
    }

    /// Both `"lang": "es_CR", "to": "cheque:*"` corpus rows, plus the codes it
    /// records as NotImplementedError.
    #[test]
    fn cheque_corpus_rows() {
        let l = LangEsCr::new();
        let v = BigDecimal::from_str("1234.56").unwrap();
        assert_eq!(
            l.to_cheque(&v, "EUR").unwrap(),
            "MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 EUROS"
        );
        assert_eq!(
            l.to_cheque(&v, "USD").unwrap(),
            "MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 DÓLARES"
        );
        for cur in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert!(matches!(
                l.to_cheque(&v, cur),
                Err(N2WError::NotImplemented(_))
            ));
        }
    }

    /// `to_cheque` reaches `to_cardinal` via `_money_verbose` and so bypasses
    /// every un/uno fixup, and it always takes the plural unit.
    #[test]
    fn cheque_keeps_uno_and_pluralizes() {
        let l = LangEsCr::new();
        let cases = [
            ("1.0", "EUR", "UNO AND 00/100 EUROS"),
            ("21.21", "EUR", "VEINTIUNO AND 21/100 EUROS"),
            ("2.0", "CRC", "DOS AND 00/100 COLÓNES"),
            ("0.01", "CRC", "CERO AND 01/100 COLÓNES"),
            ("-1234.56", "EUR", "MINUS MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 EUROS"),
            // int() truncates toward zero; no half-even bump to CATORCE.
            ("13.5", "EUR", "TRECE AND 50/100 EUROS"),
        ];
        for (v, cur, want) in cases {
            let d = BigDecimal::from_str(v).unwrap();
            assert_eq!(l.to_cheque(&d, cur).unwrap(), want, "to_cheque({}, {})", v, cur);
        }
    }

    /// Quirk 7 (fixed, #160): the integer arm no longer double-spaces the
    /// minus word.
    #[test]
    fn negative_int_single_space() {
        let l = LangEsCr::new();
        for (val, want) in [
            (int("-1"), "menos un euro"),
            (int("-2"), "menos dos euros"),
            (int("-21"), "menos veintiun euros"),
        ] {
            assert_eq!(l.to_currency(&val, "EUR", true, None, false).unwrap(), want);
        }
        for (val, want) in [
            (float("-1.0"), "menos un euro y cero céntimos"),
            (float("-0.01"), "menos cero euros y un céntimo"),
        ] {
            assert_eq!(l.to_currency(&val, "EUR", true, None, false).unwrap(), want);
        }
    }

    /// Quirk 10: the blanket `replace("uno", "un")` and ES's targeted rewrites
    /// disagree, so the same 21 is accented on one arm and bare on the other.
    #[test]
    fn uno_fixups_are_substring_replaces() {
        let l = LangEsCr::new();
        let cases: Vec<(&str, CurrencyValue, &str)> = vec![
            // Integer arm: only ES_CR's blanket rule runs -> no accent.
            ("EUR", int("21"), "veintiun euros"),
            ("EUR", int("31"), "treinta y un euros"),
            ("EUR", int("101"), "ciento un euros"),
            // ...and it happily rewrites mid-compound.
            ("EUR", int("1000001"), "un millón un euros"),
            ("EUR", int("21000"), "veintiún mil euros"),
            ("EUR", int("100000000021"), "cien mil millones veintiun euros"),
            // Float arm, EUR: ES's accented rules consume the "uno" first.
            ("EUR", float("21.21"), "veintiún euros y veintiún céntimos"),
            ("EUR", float("21.0"), "veintiún euros y cero céntimos"),
            ("EUR", float("21.01"), "veintiún euros y un céntimo"),
            ("EUR", float("1.21"), "un euro y veintiún céntimos"),
            // Float arm, USD: no accented rule mentions dólar/centavo, so the
            // blanket rule lands instead -> unaccented. Note "uno dólar" also
            // matches *inside* "veintiuno dólares".
            ("USD", float("21.21"), "veintiun dólares y veintiun centavos"),
            ("USD", float("21.01"), "veintiun dólares y un centavo"),
            ("USD", float("1.21"), "un dólar y veintiun centavos"),
            // CRC mixes the two: céntimos accented, colónes not.
            ("CRC", float("21.21"), "veintiun colónes y veintiún céntimos"),
            ("CRC", float("21.0"), "veintiun colónes y cero céntimos"),
            ("CRC", float("1.21"), "un colón y veintiún céntimos"),
        ];
        for (cur, val, want) in cases {
            let got = l.to_currency(&val, cur, true, None, false).unwrap();
            assert_eq!(got, want, "to_currency({:?}, {})", val, cur);
        }
    }

    /// `currency="CRC"` / `separator=" y"` are this class's own signature
    /// defaults, and "colónes" is spelled as `lang_ES_CR.py` spells it.
    #[test]
    fn defaults_and_the_colones_typo() {
        let l = LangEsCr::new();
        assert_eq!(l.default_currency(), "CRC");
        assert_eq!(l.default_separator(), " y");
        let cur = l.default_currency();
        for (val, want) in [
            (int("1"), "un colón"),
            (int("2"), "dos colónes"),
            (int("0"), "cero colónes"),
            (int("-3"), "menos tres colónes"),
        ] {
            assert_eq!(l.to_currency(&val, cur, true, None, false).unwrap(), want);
        }
        assert_eq!(
            l.to_currency(&float("1.0"), cur, true, None, false).unwrap(),
            "un colón y cero céntimos"
        );
    }

    /// Quirk 8: `adjective=` is honoured on the float arm and silently dropped
    /// on the integer arm.
    #[test]
    fn adjective_only_reaches_the_float_arm() {
        let l = LangEsCr::new();
        assert_eq!(
            l.to_currency(&float("12.34"), "USD", true, None, true).unwrap(),
            "doce US dólares y treinta y cuatro centavos"
        );
        for (val, want) in [(int("2"), "dos dólares"), (int("1"), "un dólar")] {
            assert_eq!(l.to_currency(&val, "USD", true, None, true).unwrap(), want);
        }
        // EUR has no adjective, so the flag is inert there.
        assert_eq!(
            l.to_currency(&float("1.0"), "EUR", true, None, true).unwrap(),
            "un euro y cero céntimos"
        );
    }

    /// `cents=false` routes through `_cents_terse`, which zero-pads to
    /// `len(str(divisor)) - 1` == 2 digits.
    #[test]
    fn cents_terse() {
        let l = LangEsCr::new();
        assert_eq!(
            l.to_currency(&float("1.0"), "EUR", false, None, false).unwrap(),
            "un euro y 00 céntimos"
        );
        assert_eq!(
            l.to_currency(&float("21.21"), "EUR", false, None, false).unwrap(),
            "veintiún euros y 21 céntimos"
        );
        assert_eq!(
            l.to_currency(&float("12.34"), "USD", false, None, false).unwrap(),
            "doce dólares y 34 centavos"
        );
    }

    /// `1e21` is a float whose repr has no dot, yet `has_decimal` is still true
    /// (`isinstance(val, float)` short-circuits), so the cents segment shows.
    #[test]
    fn float_without_a_dot_in_its_repr_still_prints_cents() {
        let l = LangEsCr::new();
        assert_eq!(
            l.to_currency(&float("1e+21"), "EUR", true, None, false).unwrap(),
            "mil trillones euros y cero céntimos"
        );
        assert_eq!(
            l.to_currency(&float("1000000.5"), "EUR", true, None, false).unwrap(),
            "un millón euros y cincuenta céntimos"
        );
    }

    /// The forms lookup precedes `to_cardinal`, so an unknown code out-ranks an
    /// out-of-range value even though Python evaluates both.
    #[test]
    fn notimplemented_outranks_overflow() {
        let l = LangEsCr::new();
        let huge = int(&format!("1{}", "0".repeat(60)));
        assert!(matches!(
            l.to_currency(&huge, "GBP", true, None, false),
            Err(N2WError::NotImplemented(_))
        ));
        assert!(matches!(
            l.to_currency(&huge, "EUR", true, None, false),
            Err(N2WError::Overflow(_))
        ));
    }
}
