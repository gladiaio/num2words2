//! Port of `lang_ES_NI.py` (Spanish — Nicaragua), the class the `"es_NI"`
//! registry key resolves to (verified in `__init__.py`: `"es_NI":
//! lang_ES_NI.Num2Word_ES_NI()`).
//!
//! `Num2Word_ES_NI` subclasses `Num2Word_ES` and overrides **only**
//! `CURRENCY_FORMS` and `to_currency`, so for the four cardinal/ordinal/year
//! modes this is `Num2Word_ES` verbatim. Chain:
//! `Num2Word_ES_NI` → `Num2Word_ES` → `Num2Word_EUR` → `Num2Word_Base`.
//!
//! # Currency: `super(Num2Word_ES, self)` skips its own parent
//!
//! The single load-bearing subtlety of this module. `to_currency` calls
//!
//! ```python
//! result = super(Num2Word_ES, self).to_currency(...)
//! return result.replace("uno", "un")
//! ```
//!
//! `super(Num2Word_ES, self)` starts the MRO lookup *after* `Num2Word_ES`, not
//! after `Num2Word_ES_NI`. The next class is `Num2Word_EUR`, which defines no
//! `to_currency`, so the call lands on `Num2Word_Base.to_currency` and
//! `Num2Word_ES.to_currency` is **bypassed entirely**. Two consequences, both
//! observable:
//!
//! * ES's integer fast path (which special-cases `abs(val) == 1` to emit "un"
//!   and picks the singular/plural form by hand) never runs. Base's integer
//!   branch runs instead, and it pluralizes via `pluralize`, which agrees only
//!   with `n == 1` — hence `to_currency(1000000)` == "un millón euros", with a
//!   plural noun after a singular "millón". Reproduced.
//! * ES's *targeted* fixups (`"veintiuno euro"` → `"veintiún euro"`, `"uno
//!   céntimo"` → `"un céntimo"`, ...) never run. ES_NI substitutes one blanket
//!   `replace("uno", "un")` over the whole rendered string — see below.
//!
//! # The blanket `replace("uno", "un")`
//!
//! It is a plain substring replace, not a word-boundary one, so it also fires
//! inside "veintiuno" and yields the **unaccented** "veintiun" — where ES's
//! targeted replace would have produced the correctly accented "veintiún".
//! Verified against the interpreter and preserved verbatim:
//!
//! | input | output |
//! |---|---|
//! | `21` NIO | `veintiun córdobas` (not "veintiún") |
//! | `101.0` NIO | `ciento un córdobas con cero centavos` |
//! | `21.21` NIO | `veintiun córdobas con veintiun centavos` |
//! | `1` NIO | `un córdoba` |
//!
//! Applying it to the finished string (rather than to the money/cents pieces)
//! is what lets it reach both segments at once, so the port delegates first and
//! replaces last, exactly as Python does.
//!
//! `to_cheque` is **not** overridden and therefore does *not* get the replace:
//! `to_cheque(1.0, "NIO")` == "UNO AND 00/100 CÓRDOBAS", with "UNO" intact.
//!
//! Shape: **engine**. ES defines `mid_numwords`/`low_numwords` and inherits
//! `set_high_numwords` from `Num2Word_EUR`, letting `Num2Word_Base.to_cardinal`
//! drive `splitnum`/`clean`/`merge`. So `cards` + `merge` are supplied here and
//! the `base.rs` engine does the rest.
//!
//! # Number scale
//!
//! ES sets `GIGA_SUFFIX = None` and `MEGA_SUFFIX = "illón"`, so EUR's
//! `set_high_numwords` emits *only* the `10^(n-3)` (mega) cards and skips the
//! `10^n` (giga) ones. That yields the Spanish long scale — 10^6 "millón",
//! 10^12 "billón", 10^18 "trillón" — with no card for 10^9, which is why
//! 10^9 comes out compositionally as "mil millones". `MAXVAL` is 10^57.
//!
//! ES also calls `gen_high_numwords([], [], lows)` with **empty** units/tens.
//! The comprehension `[u + t for t in tens for u in units]` over two empty
//! lists is empty, so the function degenerates to `[] + lows == lows` and the
//! Latin-prefix elision machinery never runs. The high list is therefore just
//! `lows`, hardcoded below — no `gen_high_numwords` port is needed.
//!
//! # Faithfully reproduced Python oddities
//!
//! Verified against the interpreter; all are preserved verbatim.
//!
//! 1. ~~**Unaccented "vigesimo"**.~~ `to_ordinal` for 11..=29 does
//!    `self.ords[dec].replace("é", "e")`, and 20 used to land there too.
//!    Fixed (gladiaio/num2words2#252): 20 keeps its accent and the caller's
//!    gender ("vigésimo", "vigésima", "centésimo vigésimo"); only the fused
//!    21..=29 forms drop it ("vigesimoprimero"), as the RAE spells them.
//! 2. **`ords` typos**: 400 is `"cuadrigentésim"` (standard Spanish is
//!    *cuadringentésimo*) and 700 is `"septigentésim"` (standard:
//!    *septingentésimo*). Corpus rows 123456 and 700 confirm both.
//! 3. **Ordinal/cardinal scale mismatch**: `ords[1e9] = "billonésim"` while
//!    `cards[1e12] = "billón"`. The ordinals follow the *short* scale, the
//!    cardinals the *long* scale. Hence `to_cardinal(10^9)` == "mil millones"
//!    but `to_ordinal(10^9)` == "billonésimo", and `to_ordinal(10^12)` ==
//!    "trillonésimo" while `to_cardinal(10^12)` == "un billón".
//! 4. **Missing separator in the >1000 ordinal branch**: the high-part cardinal
//!    is concatenated straight onto the scale word, so `to_ordinal(99999)` ==
//!    "noventa y nuevemilésimo noningentésimo nonagésimo noveno" (note
//!    "nuevemilésimo"). Corpus-confirmed.
//! 5. **`replace("oo", "o")`** at every recursion level: "decimooctavo" →
//!    "decimoctavo" (documented in the Python source as deliberate).
//! 6. **`to_ordinal(0)`** raises ValueError here (Python returns `""`; #160).
//! 7. Two hard crashes near 10^18, see [`log1000_floor`] and the notes below.
//!
//! # Crashes reproduced (see `concerns` in the port report)
//!
//! `to_ordinal` computes `dec = 1000 ** int(math.log(int(value), 1000))` using
//! **binary floating point**, which rounds *up* to an exact power just below
//! the true boundary. Two distinct failures follow, both confirmed against
//! CPython:
//!
//! * `999999999999996..=999999999999999` → float log yields exactly `5.0`, so
//!   `dec = 10^15 > value`, `divmod` gives `high_part = 0`, `low_part = value`,
//!   and `to_ordinal(low_part)` re-enters with the *same* value forever →
//!   Python `RecursionError`. Guarded here by the `high_part.is_zero()` check;
//!   see [`RECURSION_MSG`] for why the error variant is approximate.
//! * `999999999999995072..=999999999999999999` → float log yields exactly
//!   `6.0`, so `dec = 10^18`, which is **absent** from `ords` (it stops at
//!   1e15) → Python `KeyError: 1000000000000000000`. The `KeyError` fires
//!   *before* the recursion because Python evaluates the `%` format tuple left
//!   to right (`self.ords[dec]` precedes the recursive call), so the ordering
//!   of the two checks below is load-bearing.
//!
//! Both windows are outside the frozen corpus but are real reachable inputs.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError,
    Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// `Num2Word_ES_NI.CURRENCY_FORMS`.
///
/// ES_NI declares this in its own class body, which **shadows** the ~180-entry
/// `Num2Word_ES.CURRENCY_FORMS` rather than extending it — so exactly these
/// three codes exist and everything else (GBP, JPY, KWD, BHD, INR, CNY, CHF,
/// ...) raises NotImplementedError. The corpus asserts that for all of them.
///
/// Note also that the `lang_EUR` shared-dict trap does **not** apply here:
/// `Num2Word_EN.__init__` mutates `Num2Word_EUR.CURRENCY_FORMS` in place, but
/// ES_NI never reads that dict — `CURRENCY_FORMS in Num2Word_ES_NI.__dict__`
/// is True and `instance.CURRENCY_FORMS is Num2Word_ES_NI.__dict__[...]` was
/// verified against the live interpreter after a full package import.
///
/// There is no `CURRENCY_PRECISION` anywhere in the chain: it resolves to
/// `Num2Word_Base.CURRENCY_PRECISION == {}` (EN *rebinds* its own as an
/// instance attribute, so nothing leaks), leaving `.get(code, 100)` at 100 for
/// every code. The trait's default `currency_precision` already returns 100,
/// so it is deliberately not overridden — ES_NI has no 3-decimal or 0-decimal
/// currency and `default_to_currency`'s `divisor == 1` branch is unreachable.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTAVOS: [&str; 2] = ["centavo", "centavos"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("NIO", CurrencyForms::new(&["córdoba", "córdobas"], &CENTAVOS));
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "euros"], &["céntimo", "céntimos"]),
    );
    m.insert("USD", CurrencyForms::new(&["dólar", "dólares"], &CENTAVOS));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited untouched — neither ES nor
/// ES_NI defines one, and (unlike `CURRENCY_FORMS`) no class mutates it.
///
/// Nearly dead weight, since only USD appears in both this table and ES_NI's
/// three-code `CURRENCY_FORMS`. But it is reachable: `to_currency(1, "USD",
/// adjective=True)` == "un US dólar", so the table is ported rather than
/// dropped.
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

/// `self.gender_stem`, set to "o" in `Num2Word_ES.setup` and never reassigned.
///
/// `to_ordinal`/`to_ordinal_num` shadow it with a *local* `gender_stem`, so the
/// instance attribute that `merge` reads stays "o" regardless of any ordinal
/// gender argument. Cardinals are therefore never gender-affected.
const GENDER_STEM: &str = "o";

/// Stands in for Python's `RecursionError`, which `N2WError` has no variant
/// for. Rust would abort the process on real infinite recursion, so the
/// condition is detected up front and reported as an error instead.
const RECURSION_MSG: &str = "maximum recursion depth exceeded \
    (Python raises RecursionError here: dec > value makes high_part 0, so \
     to_ordinal(low_part) re-enters with the same value)";

/// Python's `s[:-n]` — drop the last `n` **characters**.
///
/// Must be character-based, not byte-based: the strings this runs on contain
/// "ó" (2 bytes in UTF-8), e.g. `"millón"[:-3] == "mil"`. Slicing by byte would
/// split the accented codepoint and panic. Python returns `""` when the string
/// is shorter than `n`; so does this.
fn drop_last_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        return String::new();
    }
    s.chars().take(count - n).collect()
}

/// Exact integer reimplementation of `int(math.log(int(value), 1000))`.
///
/// CPython evaluates `math.log(x, base)` as `log(float(x)) / log(1000.0)` (the
/// big-int `frexp` path only engages when the value overflows a double, which
/// cannot happen below 10^18). Recomputing that in Rust would make the result
/// depend on `f64::ln` agreeing with the platform libm bit for bit — a 1-ulp
/// difference at a boundary flips `dec` by a factor of 1000 and silently
/// changes the output or the exception.
///
/// So the float is eliminated entirely. The computed quotient is monotonic
/// non-decreasing in `value`, so each `k` has a single exact threshold; these
/// were recovered by binary-searching CPython and re-verified against
/// `int(math.log(v, 1000))` on 365,931 values (every boundary ±6000 plus
/// 300k random draws below 10^18) with zero mismatches.
///
/// Note the last two thresholds sit *below* their exact powers — 10^15 - 4 and
/// 10^18 - 4928 — which is precisely the float rounding that causes the
/// `RecursionError` and `KeyError` windows documented at module level.
///
/// Only called with `value > 1000`, so the result is always >= 1.
fn log1000_floor(value: &BigInt) -> u32 {
    // (threshold, k), ascending; the last threshold <= value wins.
    let thresholds: [(&str, u32); 6] = [
        ("1000", 1),
        ("1000000", 2),
        ("1000000000", 3),
        ("1000000000000", 4),
        ("999999999999996", 5),        // 10^15 - 4
        ("999999999999995072", 6),     // 10^18 - 4928
    ];
    let mut k = 0;
    for (t, kk) in thresholds.iter() {
        let t: BigInt = t.parse().expect("static threshold literal");
        if value >= &t {
            k = *kk;
        } else {
            break;
        }
    }
    k
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

pub struct LangEsNi {
    cards: Cards,
    maxval: BigInt,
    /// `Num2Word_ES.ords`. Python mixes int keys (1..900) with float keys
    /// (1e3..1e15); since `hash(1000) == hash(1000.0)` and they compare equal,
    /// `ords[1000]` finds the `1e3` entry. Collapsing them to `BigInt` here is
    /// therefore faithful — and note 1e18 is deliberately absent (see module
    /// docs: that absence is what raises `KeyError`).
    ords: HashMap<BigInt, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEsNi {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsNi {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_ES.setup: gen_high_numwords([], [], lows) degenerates to
        // `lows` (empty units/tens => empty comprehension). See module docs.
        let high = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        // Num2Word_EUR.set_high_numwords, with GIGA_SUFFIX = None and
        // MEGA_SUFFIX = "illón":
        //   cap = 3 + 6 * len(high) = 57
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10 ** (n - 3)] = word + "illón"
        // range(57, 3, -6) has exactly 9 entries, matching len(high), so zip
        // consumes both fully: 10^54 nonillón .. 10^6 millón.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
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

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10^54 = 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Typos "cuadrigentésim" (400) and "septigentésim" (700) are verbatim
        // from Python — see module docs.
        let ords: HashMap<BigInt, &'static str> = [
            (BigInt::from(1), "primer"),
            (BigInt::from(2), "segund"),
            (BigInt::from(3), "tercer"),
            (BigInt::from(4), "cuart"),
            (BigInt::from(5), "quint"),
            (BigInt::from(6), "sext"),
            (BigInt::from(7), "séptim"),
            (BigInt::from(8), "octav"),
            (BigInt::from(9), "noven"),
            (BigInt::from(10), "décim"),
            (BigInt::from(20), "vigésim"),
            (BigInt::from(30), "trigésim"),
            (BigInt::from(40), "cuadragésim"),
            (BigInt::from(50), "quincuagésim"),
            (BigInt::from(60), "sexagésim"),
            (BigInt::from(70), "septuagésim"),
            (BigInt::from(80), "octogésim"),
            (BigInt::from(90), "nonagésim"),
            (BigInt::from(100), "centésim"),
            (BigInt::from(200), "ducentésim"),
            (BigInt::from(300), "tricentésim"),
            (BigInt::from(400), "cuadrigentésim"),
            (BigInt::from(500), "quingentésim"),
            (BigInt::from(600), "sexcentésim"),
            (BigInt::from(700), "septigentésim"),
            (BigInt::from(800), "octigentésim"),
            (BigInt::from(900), "noningentésim"),
            (BigInt::from(1_000i64), "milésim"),
            (BigInt::from(1_000_000i64), "millonésim"),
            (BigInt::from(1_000_000_000i64), "billonésim"),
            (BigInt::from(1_000_000_000_000i64), "trillonésim"),
            (BigInt::from(1_000_000_000_000_000i64), "cuadrillonésim"),
        ]
        .into_iter()
        .collect();

        LangEsNi {
            cards,
            maxval,
            ords,
            // Num2Word_ES.setup: exclude_title = ["y", "menos", "punto"]
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            // Built once here, never per call. `to_currency` only reads these,
            // and rebuilding them on every call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `self.ords[key]`, raising Python's `KeyError` when absent.
    fn ords_get(&self, key: &BigInt) -> Result<&'static str> {
        self.ords
            .get(key)
            .copied()
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The first check (`value == int(value)`) cannot fail for integer input,
    /// so only the negative check is reachable. Python raises **TypeError**
    /// here, not ValueError — corpus rows for -1/-7/-21/... confirm it.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            // errmsg_negord, with the implicit string concatenation resolved.
            return Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_ES.to_ordinal(value, gender="m")`.
    ///
    /// `gender` is threaded through recursion exactly as Python does: the
    /// 11..=29 branch overrides the *local* `gender_stem` to "o" but still
    /// forwards the original `gender` downward. Only "m" is reachable from the
    /// `Lang` trait, but the parameter is kept so the port stays 1:1.
    fn to_ordinal_gendered(&self, value: &BigInt, gender: &str) -> Result<String> {
        let mut gender_stem = if gender == "f" { "a" } else { "o" };

        self.verify_ordinal(value)?;

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let e18 = BigInt::from(10u8).pow(18);

        let text: String = if value.is_zero() {
            String::new()
        } else if value <= &ten {
            format!("{}{}", self.ords_get(value)?, gender_stem)
        } else if value <= &BigInt::from(29) && *value != BigInt::from(20) {
            // RAE: simple forms preferred up to 30; "sobreesdrújula" spelling
            // drops the accent via replace("é", "e") -> "decimo"/"vigesimo"
            // in the fused 11..=29 forms; 20 skips it (#252).
            gender_stem = "o";
            let dec = (value / &ten) * &ten;
            format!(
                "{}{}{}",
                self.ords_get(&dec)?.replace('é', "e"),
                gender_stem,
                self.to_ordinal_gendered(&(value % &ten), gender)?
            )
        } else if value <= &hundred {
            let dec = (value / &ten) * &ten;
            format!(
                "{}{} {}",
                self.ords_get(&dec)?,
                gender_stem,
                self.to_ordinal_gendered(&(value - &dec), gender)?
            )
        } else if value <= &thousand {
            let cen = (value / &hundred) * &hundred;
            format!(
                "{}{} {}",
                self.ords_get(&cen)?,
                gender_stem,
                self.to_ordinal_gendered(&(value - &cen), gender)?
            )
        } else if value < &e18 {
            // Round down to the nearest 1e(3n). See log1000_floor for why this
            // is integer thresholds rather than a float log.
            let dec = thousand.pow(log1000_floor(value));
            let (high_part, low_part) = value.div_mod_floor(&dec);

            let cardinal = if !high_part.is_one() {
                self.to_cardinal(&high_part)?
            } else {
                String::new()
            };

            // Order matters. Python builds "%s%s%s %s" % (cardinal,
            // self.ords[dec], gender_stem, self.to_ordinal(low_part)) and
            // evaluates the tuple left to right, so a missing `ords[dec]`
            // (dec == 10^18) raises KeyError *before* the recursive call can
            // blow the stack. Keep the lookup ahead of the recursion guard.
            let ordword = self.ords_get(&dec)?;

            // high_part == 0 <=> dec > value <=> low_part == value, i.e. the
            // recursive call re-enters unchanged. Python spins until
            // RecursionError; detect it instead of overflowing the Rust stack.
            if high_part.is_zero() {
                return Err(N2WError::Value(format!("{}: {}", RECURSION_MSG, value)));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                ordword,
                gender_stem,
                self.to_ordinal_gendered(&low_part, gender)?
            )
        } else {
            self.to_cardinal(value)?
        };

        // Handle exception: it's not "decimooctavo" but "decimoctavo".
        // strip() first, then replace() — applied at *every* recursion level.
        Ok(text.trim().replace("oo", "o"))
    }
}

impl Lang for LangEsNi {

    fn es_currency_ordinal_fires(&self) -> bool {
        true
    }

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NIO"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " con"
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
    /// Python names the params `curr`/`next` and unpacks `curr + next` into
    /// `ctext, cnum, ntext, nnum`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext_in, cnum) = l;
        let (ntext_in, nnum) = r;
        let mut ctext = ctext_in.to_string();
        let mut ntext = ntext_in.to_string();

        let one = BigInt::one();
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                // Python `return next` — drops "uno" entirely ("cien", not
                // "uno cien"); only >= 1e6 keeps a spoken unit ("un millón").
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        } else if cnum == &hundred && !(nnum % &thousand).is_zero() {
            // `not nnum % 1000 == 0` parses as `not ((nnum % 1000) == 0)`:
            // "cien" -> "ciento" only when the next chunk is not a round
            // thousand, which is what keeps 100000 as "cien mil".
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if cnum < &hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && cnum > &one {
            // Pluralize the mega word: "millón" -> "millones", "billón" ->
            // "billones". Character-based slice: "ó" is two bytes.
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
            // Note ntext is only cleared for 5, so 7/9 build on "cien":
            // "sete" + "cien" + "tos" = "setecientos".
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

    /// `Num2Word_ES.to_cardinal` → `Num2Word_Base.to_cardinal` for integers.
    ///
    /// The overflow check is duplicated from the engine purely to emit ES's
    /// own `errmsg_toobig`, "abs(%s) deber ser inferior a %s." — note the
    /// Python typo "deber" (should be "debe"), kept verbatim. Otherwise the
    /// base engine drives splitnum/clean/merge unchanged.
    ///
    /// The `_pending_ordinal` handshake in Python's `Num2Word_ES.to_cardinal`
    /// is intentionally NOT modelled; it is unreachable for integer input.
    /// See the port report's `concerns`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let v = value.abs();
        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) deber ser inferior a {}.",
                v,
                self.maxval()
            )));
        }
        default_to_cardinal(self, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        crate::lang_es::es_reject_zero_ordinal(value)?;
        self.to_ordinal_gendered(value, "m")
    }

    /// `Num2Word_ES.to_ordinal_num(value, gender="m")` — digits plus the
    /// masculine ordinal indicator; no words involved.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year` is `return self.to_cardinal(int(val))` — no
    /// century splitting, no BC/AD suffix. The `int()` cast is a no-op for
    /// integer input, and negatives simply pick up "menos" from to_cardinal
    /// (corpus: -500 -> "menos quinientos").
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
        self.to_ordinal_gendered(value, gender)
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
    /// `Num2Word_ES_NI`: idiomatic medio/tercio/cuarto for denominators
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
    // ES_NI inherits `to_cheque`, `_money_verbose`, `_cents_verbose` and
    // `_cents_terse` unchanged from `Num2Word_Base`, and `pluralize` from
    // `Num2Word_EUR`. `CURRENCY_PRECISION` is Base's empty dict, so the
    // trait's default `currency_precision` (100) is already right. Only the
    // data tables, the class name, that one plural rule and `to_currency`
    // itself are language-specific.

    /// `self.__class__.__name__`, as `Num2Word_Base` interpolates it into
    /// `'Currency code "%s" not implemented for "%s"'`.
    fn lang_name(&self) -> &str {
        "Num2Word_ES_NI"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. All three of ES_NI's entries carry two forms, so
    /// that is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES_NI.to_currency`.
    ///
    /// ```python
    /// result = super(Num2Word_ES, self).to_currency(
    ///     val, currency=currency, cents=cents,
    ///     separator=separator, adjective=adjective)
    /// return result.replace("uno", "un")
    /// ```
    ///
    /// The explicit `super(Num2Word_ES, self)` resolves past `Num2Word_ES` to
    /// `Num2Word_Base.to_currency` (`Num2Word_EUR` defines none), so this
    /// delegates to `default_to_currency` — *not* to any ES-specific body. See
    /// the module docs for why that distinction changes the output.
    ///
    /// `replace` runs on the finished string, after the cents segment has been
    /// appended, so it rewrites both halves in one pass ("veintiuno córdobas
    /// con veintiuno centavos" → "veintiun córdobas con veintiun centavos").
    /// Rust's `str::replace` and Python's `str.replace` agree: every
    /// non-overlapping occurrence, scanned left to right.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait hands us None when the caller omitted `separator=`; resolve it
        // to this language's own default (" con") before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let result = default_to_currency(self, val, currency, cents, separator, adjective)?;
        Ok(result.replace("uno", "un"))
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A Python `int` arg (a corpus `arg` with no dot and no exponent).
    fn int(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, true, false, false).unwrap()
    }

    /// A Python `float` arg. `has_decimal` is always true for a float repr.
    fn float(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, false, true, true).unwrap()
    }

    fn cur(lang: &LangEsNi, code: &str, v: CurrencyValue) -> Result<String> {
        lang.to_currency(&v, code, true, None, false)
    }

    /// Every succeeding `"lang": "es_NI", "to": "currency:*"` row of the frozen
    /// corpus, verbatim. The corpus carries 24 such rows (EUR and USD only).
    #[test]
    fn currency_corpus_rows() {
        let l = LangEsNi::new();
        let rows: Vec<(&str, CurrencyValue, &str)> = vec![
            ("EUR", int("0"), "cero euros"),
            ("EUR", int("1"), "un euro"),
            ("EUR", int("2"), "dos euros"),
            ("EUR", int("100"), "cien euros"),
            ("EUR", float("12.34"), "doce euros con treinta y cuatro céntimos"),
            ("EUR", float("0.01"), "cero euros con un céntimo"),
            ("EUR", float("1.0"), "un euro con cero céntimos"),
            (
                "EUR",
                float("99.99"),
                "noventa y nueve euros con noventa y nueve céntimos",
            ),
            (
                "EUR",
                float("1234.56"),
                "mil doscientos treinta y cuatro euros con cincuenta y seis céntimos",
            ),
            (
                "EUR",
                float("-12.34"),
                "menos doce euros con treinta y cuatro céntimos",
            ),
            ("EUR", int("1000000"), "un millón euros"),
            ("EUR", float("0.5"), "cero euros con cincuenta céntimos"),
            ("USD", int("0"), "cero dólares"),
            ("USD", int("1"), "un dólar"),
            ("USD", int("2"), "dos dólares"),
            ("USD", int("100"), "cien dólares"),
            ("USD", float("12.34"), "doce dólares con treinta y cuatro centavos"),
            ("USD", float("0.01"), "cero dólares con un centavo"),
            ("USD", float("1.0"), "un dólar con cero centavos"),
            (
                "USD",
                float("99.99"),
                "noventa y nueve dólares con noventa y nueve centavos",
            ),
            (
                "USD",
                float("1234.56"),
                "mil doscientos treinta y cuatro dólares con cincuenta y seis centavos",
            ),
            (
                "USD",
                float("-12.34"),
                "menos doce dólares con treinta y cuatro centavos",
            ),
            ("USD", int("1000000"), "un millón dólares"),
            ("USD", float("0.5"), "cero dólares con cincuenta centavos"),
        ];
        for (code, val, want) in rows {
            assert_eq!(cur(&l, code, val.clone()).unwrap(), want, "{code} {val:?}");
        }
    }

    /// The 84 NotImplementedError currency rows: seven codes x twelve args.
    /// ES_NI's own three-code `CURRENCY_FORMS` shadows `Num2Word_ES`'s ~180
    /// entries, so even INR/CHF/CNY — which `Num2Word_ES` defines — are absent.
    #[test]
    fn currency_missing_codes_raise_not_implemented() {
        let l = LangEsNi::new();
        let args = [
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
        ];
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for a in args.iter() {
                match cur(&l, code, a.clone()) {
                    Err(N2WError::NotImplemented(msg)) => assert_eq!(
                        msg,
                        format!("Currency code \"{code}\" not implemented for \"Num2Word_ES_NI\"")
                    ),
                    other => panic!("{code} {a:?}: expected NotImplemented, got {other:?}"),
                }
            }
        }
    }

    /// Both succeeding `to": "cheque:*"` corpus rows, plus the seven that
    /// raise. `to_cheque` is `Num2Word_Base`'s, so it is *not* subject to
    /// `to_currency`'s "uno" -> "un" replace — `1.0` keeps "UNO".
    #[test]
    fn cheque_corpus_rows() {
        let l = LangEsNi::new();
        let v = BigDecimal::from_str("1234.56").unwrap();
        assert_eq!(
            l.to_cheque(&v, "EUR").unwrap(),
            "MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 EUROS"
        );
        assert_eq!(
            l.to_cheque(&v, "USD").unwrap(),
            "MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 DÓLARES"
        );
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match l.to_cheque(&v, code) {
                Err(N2WError::NotImplemented(msg)) => assert_eq!(
                    msg,
                    format!("Currency code \"{code}\" not implemented for \"Num2Word_ES_NI\"")
                ),
                other => panic!("cheque {code}: expected NotImplemented, got {other:?}"),
            }
        }
        // No "uno" -> "un" on the cheque path, and MINUS for negatives.
        assert_eq!(
            l.to_cheque(&BigDecimal::from_str("1.0").unwrap(), "NIO").unwrap(),
            "UNO AND 00/100 CÓRDOBAS"
        );
        assert_eq!(
            l.to_cheque(&BigDecimal::from_str("21.0").unwrap(), "NIO").unwrap(),
            "VEINTIUNO AND 00/100 CÓRDOBAS"
        );
        assert_eq!(
            l.to_cheque(&BigDecimal::from_str("-1234.56").unwrap(), "NIO").unwrap(),
            "MINUS MIL DOSCIENTOS TREINTA Y CUATRO AND 56/100 CÓRDOBAS"
        );
    }

    /// NIO — the `to_currency(currency="NIO")` default and ES_NI's raison
    /// d'être — has **no corpus rows at all**. Pinned here against the live
    /// interpreter instead.
    #[test]
    fn currency_nio_default() {
        let l = LangEsNi::new();
        assert_eq!(l.default_currency(), "NIO");
        assert_eq!(l.default_separator(), " con");
        let rows: Vec<(CurrencyValue, &str)> = vec![
            (int("0"), "cero córdobas"),
            (int("1"), "un córdoba"),
            (int("2"), "dos córdobas"),
            (int("100"), "cien córdobas"),
            (int("-1"), "menos un córdoba"),
            (int("1000000"), "un millón córdobas"),
            (float("1.0"), "un córdoba con cero centavos"),
            (float("0.01"), "cero córdobas con un centavo"),
            (float("1.01"), "un córdoba con un centavo"),
            (float("0.5"), "cero córdobas con cincuenta centavos"),
            (float("12.34"), "doce córdobas con treinta y cuatro centavos"),
            (
                float("-12.34"),
                "menos doce córdobas con treinta y cuatro centavos",
            ),
            (
                float("1234.56"),
                "mil doscientos treinta y cuatro córdobas con cincuenta y seis centavos",
            ),
        ];
        for (val, want) in rows {
            assert_eq!(cur(&l, "NIO", val.clone()).unwrap(), want, "{val:?}");
        }
    }

    /// The blanket `replace("uno", "un")` fires inside "veintiuno" and "ciento
    /// uno" too, yielding the **unaccented** "veintiun" where `Num2Word_ES`'s
    /// targeted replace would have written "veintiún". Python bug, preserved.
    /// All values confirmed against the live interpreter.
    #[test]
    fn blanket_uno_replace_quirks() {
        let l = LangEsNi::new();
        // "veintiuno" -> "veintiun", not "veintiún".
        assert_eq!(cur(&l, "NIO", int("21")).unwrap(), "veintiun córdobas");
        assert_eq!(cur(&l, "NIO", int("-21")).unwrap(), "menos veintiun córdobas");
        // Both segments rewritten in one pass over the finished string.
        assert_eq!(
            cur(&l, "NIO", float("21.21")).unwrap(),
            "veintiun córdobas con veintiun centavos"
        );
        assert_eq!(
            cur(&l, "NIO", float("101.0")).unwrap(),
            "ciento un córdobas con cero centavos"
        );
        assert_eq!(
            cur(&l, "NIO", float("31.0")).unwrap(),
            "treinta y un córdobas con cero centavos"
        );
        // Base's int branch pluralizes on n == 1 only, so a singular "millón"
        // takes a plural noun. ES's own int fast path is bypassed by
        // `super(Num2Word_ES, self)`; this is what that costs.
        assert_eq!(
            cur(&l, "EUR", float("2000000.21")).unwrap(),
            "dos millones euros con veintiun céntimos"
        );
    }

    /// `adjective=True` reaches EUR's inherited `CURRENCY_ADJECTIVES`. USD is
    /// the only code present in both that table and ES_NI's `CURRENCY_FORMS`.
    #[test]
    fn currency_adjective_only_bites_on_usd() {
        let l = LangEsNi::new();
        assert_eq!(
            l.to_currency(&int("1"), "USD", true, None, true).unwrap(),
            "un US dólar"
        );
        assert_eq!(
            l.to_currency(&int("2"), "USD", true, None, true).unwrap(),
            "dos US dólares"
        );
        assert_eq!(
            l.to_currency(&float("12.34"), "USD", true, None, true).unwrap(),
            "doce US dólares con treinta y cuatro centavos"
        );
        // NIO/EUR have no adjective entry, so the flag is inert.
        assert_eq!(
            l.to_currency(&int("1"), "NIO", true, None, true).unwrap(),
            "un córdoba"
        );
        assert_eq!(
            l.to_currency(&int("1"), "EUR", true, None, true).unwrap(),
            "un euro"
        );
    }

    /// `cents=False` routes to Base's `_cents_terse`; precision is 100 for
    /// every code (Base's `CURRENCY_PRECISION` is empty), so the width is 2.
    /// An explicit `separator=` must also override the " con" default.
    #[test]
    fn cents_terse_and_explicit_separator() {
        let l = LangEsNi::new();
        assert_eq!(
            l.to_currency(&float("12.34"), "NIO", false, None, false).unwrap(),
            "doce córdobas con 34 centavos"
        );
        assert_eq!(
            l.to_currency(&float("1.0"), "NIO", false, None, false).unwrap(),
            "un córdoba con 00 centavos"
        );
        assert_eq!(
            l.to_currency(&float("0.01"), "NIO", false, None, false).unwrap(),
            "cero córdobas con 01 centavo"
        );
        assert_eq!(
            l.to_currency(&float("12.34"), "NIO", true, Some(" y"), false).unwrap(),
            "doce córdobas y treinta y cuatro centavos"
        );
        assert_eq!(l.currency_precision("NIO"), 100);
        assert_eq!(l.currency_precision("KWD"), 100);
    }

    use crate::floatpath::FloatValue;

    /// Every non-integer `"lang": "es_NI", "to": "cardinal"` float row of the
    /// frozen corpus (whole-number floats like `1.0`/`0.0` take the integer
    /// path via the shim and never reach here). `Num2Word_ES` overrides
    /// `to_cardinal` but delegates float input to
    /// `super().to_cardinal` -> `Num2Word_Base.to_cardinal_float`, so the
    /// inherited default float path is exact — this pins that.
    #[test]
    fn cardinal_float_corpus_rows() {
        let l = LangEsNi::new();
        // (value, precision = abs(Decimal(str(value)).as_tuple().exponent), out)
        let rows: Vec<(f64, u32, &str)> = vec![
            (0.5, 1, "cero punto cinco"),
            (1.5, 1, "uno punto cinco"),
            (2.25, 2, "dos punto dos cinco"),
            (3.14, 2, "tres punto uno cuatro"),
            (0.01, 2, "cero punto cero uno"),
            (0.1, 1, "cero punto uno"),
            (0.99, 2, "cero punto nueve nueve"),
            (1.01, 2, "uno punto cero uno"),
            (12.34, 2, "doce punto tres cuatro"),
            (99.99, 2, "noventa y nueve punto nueve nueve"),
            (100.5, 1, "cien punto cinco"),
            (1234.56, 2, "mil doscientos treinta y cuatro punto cinco seis"),
            (-0.5, 1, "menos cero punto cinco"),
            (-1.5, 1, "menos uno punto cinco"),
            (-12.34, 2, "menos doce punto tres cuatro"),
            // f64-artefact traps: 0.005*1000 == 4.999...893, 0.675*1000 ==
            // 674.999...998; the `< 0.01` heuristic rescues both to 5 / 675.
            (1.005, 3, "uno punto cero cero cinco"),
            (2.675, 3, "dos punto seis siete cinco"),
        ];
        for (value, precision, want) in rows {
            let v = FloatValue::Float { value, precision };
            assert_eq!(
                l.to_cardinal_float(&v, None).unwrap(),
                want,
                "{value}"
            );
        }
    }

    /// Every `"lang": "es_NI", "to": "cardinal_dec"` Decimal row, verbatim.
    /// Decimal input takes float2tuple's exact arbitrary-precision arm (issue
    /// #603): `98746251323029.99` must keep `.99`, not round at trillion scale.
    #[test]
    fn cardinal_decimal_corpus_rows() {
        let l = LangEsNi::new();
        // (decimal literal, out); precision = the literal's scale.
        let rows: Vec<(&str, &str)> = vec![
            ("0.01", "cero punto cero uno"),
            ("1.10", "uno punto uno cero"),
            ("12.345", "doce punto tres cuatro cinco"),
            (
                "98746251323029.99",
                "noventa y ocho billones setecientos cuarenta y seis mil \
                 doscientos cincuenta y un millones trescientos veintitrés mil \
                 veintinueve punto nueve nueve",
            ),
            ("0.001", "cero punto cero cero uno"),
        ];
        for (lit, want) in rows {
            let d = BigDecimal::from_str(lit).unwrap();
            // precision == abs(exponent) == the BigDecimal's scale.
            let precision = d.as_bigint_and_exponent().1 as u32;
            let v = FloatValue::Decimal { value: d, precision };
            assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want, "{lit}");
        }
    }
}
