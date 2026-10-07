//! Port of `lang_ES_GT.py` (Guatemalan Spanish).
//!
//! Registry check: `CONVERTER_CLASSES["es_GT"]` → `lang_ES_GT.Num2Word_ES_GT()`,
//! so this file ports the class the key really resolves to.
//!
//! **`Num2Word_ES_GT` overrides `CURRENCY_FORMS` and `to_currency` and nothing
//! else.** Every other mode (`to_cardinal`/`to_ordinal`/`to_ordinal_num`/
//! `to_year`) is inherited verbatim from `Num2Word_ES`. This port is therefore
//! a port of `lang_ES.py` through its `lang_EUR` → `Num2Word_Base` ancestry.
//! (`lang_EU.py` is Basque and is *not* in the chain despite the similar name.)
//!
//! Shape: **engine**. `lang_ES.setup()` defines `high_numwords` /
//! `mid_numwords` / `low_numwords` and overrides `merge`, letting
//! `Num2Word_Base.to_cardinal` drive `splitnum`/`clean`. So `cards` + `merge`
//! are supplied here and the default `to_cardinal` does the folding.
//!
//! # Card table and MAXVAL
//!
//! `setup()` calls `gen_high_numwords([], [], lows)` with **empty** units and
//! tens, so the Latin-prefix generator degenerates to `[] + lows` — the
//! elision replacement table never fires. `Num2Word_EUR.set_high_numwords`
//! then walks `zip(high, range(3 + 6*9, 3, -6))` = `range(57, 3, -6)`, and
//! because ES sets `GIGA_SUFFIX = None` only the `MEGA_SUFFIX` ("illón") half
//! of each step is inserted, at `10**(n-3)`:
//!
//! ```text
//! 10^54 nonillón   10^48 octillón  10^42 septillón  10^36 sextillón
//! 10^30 quintillón 10^24 cuatrillón 10^18 trillón   10^12 billón   10^6 millón
//! ```
//!
//! Note the **long scale with no 10^9 card**: 10^9 is built compositionally as
//! "mil millones". `MAXVAL = 1000 * cards.keys()[0]` = `1000 * 10^54` = 10^57.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following were verified against
//! the interpreter and are preserved exactly:
//!
//! 1. **`to_ordinal(10**9)` == "billonésimo"** (and 10^12 → "trillonésimo",
//!    10^15 → "cuadrillonésimo"). The `ords` table maps `1e9 → "billonésim"`,
//!    i.e. it uses the *short* scale, while `to_cardinal` uses the *long*
//!    scale and renders 10^9 as "mil millones" and 10^12 as "un billón". The
//!    two tables disagree; the corpus pins both, so both are reproduced.
//! 2. **No space between the cardinal head and the `-ésim` stem.** The format
//!    is `"%s%s%s %s" % (cardinal, ords[dec], gender_stem, ...)`, so
//!    `to_ordinal(2000)` == "dosmilésimo" and `to_ordinal(99999)` ==
//!    "noventa y nuevemilésimo noningentésimo nonagésimo noveno".
//! 3. **`int(math.log(int(value), 1000))` is computed in `f64`** and rounds
//!    *up* across a power-of-1000 boundary for values a few ulps below it.
//!    Two distinct crashes fall out, both reproduced (see [`log1000_trunc`]):
//!    * `value` in `[999999999999995072, 999999999999999999]` → `k` = 6 →
//!      `dec` = 10^18, which is **missing** from `ords` (it stops at 1e15) →
//!      `KeyError`. Modelled as [`N2WError::Key`].
//!    * `value` in `[999999999999996, 999999999999999]` → `k` = 5 → `dec` =
//!      10^15 > `value`, so `divmod` yields `high_part` = 0 and `low_part` ==
//!      `value`, and `to_ordinal` recurses on its own argument forever →
//!      Python `RecursionError`. This propagates to every larger value whose
//!      tail lands in that window, since `to_ordinal` recurses on
//!      `low_part = value % 10^15`: the full trigger set below 10^18 is
//!      `{v : v % 10^15 in [999999999999996, 999999999999999]}` minus the
//!      `KeyError` range above — 3996 values in all, e.g. 9999999999999998
//!      and 500999999999999998. See the note on error variants below.
//!    Evaluation order matters and is preserved: Python builds the format
//!    tuple left to right, so `ords[dec]` raises `KeyError` *before* the
//!    recursive call is made. Hence 10^18-1 is a `KeyError`, not a
//!    `RecursionError`.
//! 4. **`.replace("oo", "o")` is applied at every recursion level**, not just
//!    at the top. It exists for "decimooctavo" → "decimoctavo" but is a blunt
//!    global replace over the whole assembled string.
//! 5. ~~**`to_ordinal(20)` == "vigesimo"**~~, unaccented: the `value <= 29`
//!    branch strips the accent and forces `gender_stem` back to "o", and 20
//!    landed in it too. Fixed (gladiaio/num2words2#252): 20 keeps its accent and the caller's
//!    gender ("vigésimo", "vigésima", "centésimo vigésimo"); only the fused
//!    21..=29 forms drop it ("vigesimoprimero"), as the RAE spells them.
//! 6. **`errmsg_toobig` reads "abs(%s) deber ser inferior a %s."** — "deber"
//!    is a typo for "debe" in the Python source. Kept verbatim, which is why
//!    [`LangEsGt::to_cardinal`] does its own overflow check instead of letting
//!    `default_to_cardinal` emit the English message.
//! 7. `to_ordinal(0)` raises ValueError here (Python returns `""`; #160).
//!
//! # The currency surface
//!
//! `Num2Word_ES_GT.to_currency` is a thin wrapper:
//!
//! ```python
//! def to_currency(self, val, currency="GTQ", cents=True, separator=" y",
//!                 adjective=False):
//!     result = super(Num2Word_ES_GT, self).to_currency(...)
//!     return result.replace("uno", "un")
//! ```
//!
//! so the real work is `Num2Word_ES.to_currency`, which itself splits: pure
//! `int`s take a hand-rolled branch, everything else defers to
//! `Num2Word_Base.to_currency` and then patches the result with six literal
//! `str.replace` calls. [`LangEsGt::to_currency`] transcribes that whole chain;
//! the replace cascade is order-dependent and load-bearing (see bug 10).
//!
//! `CURRENCY_FORMS` is redefined on `Num2Word_ES_GT`, so it **shadows**
//! `Num2Word_ES`'s ~190-entry table rather than extending it, and it is not the
//! dict `Num2Word_EN.__init__` mutates (that one lives on `Num2Word_EUR`).
//! Verified against the live interpreter: only `GTQ`, `EUR` and `USD` resolve,
//! and every other code — including `GBP`, which `Num2Word_ES` *does* define —
//! raises `NotImplementedError`. The corpus pins exactly that for GBP/JPY/KWD/
//! BHD/INR/CNY/CHF.
//!
//! `CURRENCY_ADJECTIVES` is **not** redefined anywhere in the chain, so
//! `Num2Word_EUR`'s 16-entry table is inherited live (`Num2Word_EN` mutates
//! `CURRENCY_FORMS` but never this one). Only `USD` appears in both tables, so
//! `USD` is the sole code for which `adjective=True` is observable — every
//! other adjective entry is shadowed by the forms lookup raising first.
//!
//! `CURRENCY_PRECISION` is `Num2Word_Base`'s empty dict (`Num2Word_EN` *rebinds*
//! it on the instance, so that never leaks here). The divisor is therefore
//! always 100: the 3-decimal (KWD/BHD, divisor 1000) and 0-decimal (JPY,
//! divisor 1) branches are unreachable, and [`Lang::currency_precision`] is
//! deliberately left at its default rather than given an empty map.
//!
//! ## More faithfully reproduced Python bugs (currency)
//!
//! 8. **The int branch double-spaced negatives (fixed, #160).** Python builds
//!    `minus_str = self.negword` — `"menos "` *with* its trailing space — so
//!    `to_currency(-1, "EUR")` == `"menos  un euro"` with two spaces. The port
//!    strips negword like the float branch does: `"menos un euro"`.
//! 9. **The int branch ignores `adjective`.** It never consults
//!    `CURRENCY_ADJECTIVES`, so `to_currency(2, "USD", adjective=True)` ==
//!    `"dos dólares"` while `to_currency(12.34, "USD", adjective=True)` ==
//!    `"doce US dólares y treinta y cuatro centavos"`.
//! 10. **`result.replace("uno", "un")` is an unanchored global replace**, and it
//!     runs *after* `Num2Word_ES`'s own accented fix-ups. The interaction is
//!     visible and asymmetric:
//!     * (Python; the port restores the accent, #253)
//!       `to_currency(21, "EUR")` == `"veintiun euros"` — the int branch never
//!       reaches ES's `"veintiuno euro"` → `"veintiún euro"` rule, so GT's
//!       blunt replace strips the "o" and leaves the word **unaccented**.
//!     * `to_currency(21.21, "EUR")` == `"veintiún euros y veintiún céntimos"`
//!       — ES's rules fire first and consume the "uno", so GT's replace is a
//!       no-op and the accent survives.
//!     * `to_currency(21.21, "GTQ")` == `"veintiun quetzales y veintiun
//!       centavos"` — ES has no "quetzal" rule, but `"veintiuno centavos"`
//!       *contains* `"uno centavo"`, so the cents half is fixed by ES and the
//!       units half by GT. Same string, two different code paths.
//!     Because the replace is unanchored it also rewrites the middle of a
//!     number: `to_currency(1000001, "GTQ")` == `"un millón un quetzales"`.
//! 11. **`money_str` is computed then discarded when `abs(val) == 1`.** The int
//!     branch calls `self.to_cardinal(abs_val)` unconditionally and only then
//!     overwrites it with the literal `"un"`. Kept in that order because
//!     `to_cardinal` can raise: the lookup order `CURRENCY_FORMS` →
//!     `to_cardinal` is what makes `to_currency(10**57, "GBP")` a
//!     `NotImplementedError` but `to_currency(10**57, "GTQ")` an
//!     `OverflowError`.
//!
//! `to_cheque` is inherited from `Num2Word_Base` untouched — no `"uno"` →
//! `"un"` fix-up reaches it, which is why `to_cheque(1, "GTQ")` == `"UNO AND
//! 00/100 QUETZALES"` rather than `"UN ..."`. The trait default already mirrors
//! it, so it is not overridden here.
//!
//! # Error variants
//!
//! * Negative input to `to_ordinal`/`to_ordinal_num` → `verify_ordinal` raises
//!   `TypeError` → [`N2WError::Type`]. (The corpus pins this for -1, -7, -21,
//!   -42, -100, -999, -1000, -1000000.)
//! * `to_cardinal(value)` with `abs(value) >= 10^57` → `OverflowError` →
//!   [`N2WError::Overflow`].
//! * The missing `ords[10**18]` entry → `KeyError` → [`N2WError::Key`].
//! * **Python's `RecursionError` has no `N2WError` counterpart.** The 3996
//!   inputs that trigger it (see bug 3) would blow the Rust stack — an abort,
//!   which is strictly worse than a wrong error type because callers cannot
//!   catch it. So the non-terminating recursion is detected up front and
//!   reported as [`N2WError::Value`] carrying Python's own RecursionError
//!   text. Both sides raise; only the exception *type* diverges, and this is
//!   the sole knowing divergence in the port — see the report's `concerns`.
//!   None of these values appear in the corpus.
//!
//! # Cross-call mutable state (dispatcher must skip the Rust path)
//!
//! `Num2Word_ES.str_to_number` recognises Spanish ordinal-suffix strings
//! ("1ro", "2da", ...) and stashes `self._pending_ordinal = (int(digits),
//! gender)` before returning the bare int; `Num2Word_ES.to_cardinal` then
//! consumes and clears that flag, returning `to_ordinal(value, gender)`
//! instead of the cardinal. That handshake spans two calls and is invisible to
//! this stateless port: `LangEsGt::to_cardinal(1)` always yields "uno", never
//! "primero". `num2words()` already guards this by capturing
//! `_plain_int = type(number) is int` *before* any converter preprocessing and
//! keying the Rust fast path off that, so string inputs like "1ro" stay on the
//! Python path. That invariant is load-bearing — see `concerns`.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError,
    Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `setup()` and never reassigned.
///
/// `to_ordinal` shadows it with a *local* `gender_stem`, so the instance
/// attribute that `merge` reads is always "o".
const GENDER_STEM: &str = "o";

/// Python's `errmsg_negord`, built from two adjacent string literals:
/// `"El número negativo %s no puede ser tratado" " como un ordinal."`
fn errmsg_negord(value: &BigInt) -> String {
    format!(
        "El número negativo {} no puede ser tratado como un ordinal.",
        value
    )
}

/// Python's `errmsg_toobig`. "deber" is a typo in the source; kept verbatim.
fn errmsg_toobig(value: &BigInt, maxval: &BigInt) -> String {
    format!("abs({}) deber ser inferior a {}.", value, maxval)
}

/// Python's `int(math.log(int(value), 1000))`, float warts included.
///
/// CPython's `loghelper` converts an int that fits a double with
/// `PyLong_AsDouble` (round-half-even) and calls C `log` on it; `math.log(x,
/// base)` is then simply `log(x) / log(base)`. So this is
/// `trunc(f64::ln(v) / f64::ln(1000.0))` — *not* an exact integer logarithm.
///
/// The difference is observable. Just below a power of 1000 the relative gap
/// falls under double precision and the quotient rounds up to the next whole
/// number, so `k` over-estimates:
///
/// ```text
/// value = 10^15 - 1  ->  k = 5  (true floor is 4)  ->  dec = 10^15 > value
/// value = 10^18 - 1  ->  k = 6  (true floor is 5)  ->  dec = 10^18, no ords entry
/// ```
///
/// Callers rely on this reproducing Python's over-estimate rather than the
/// mathematically correct floor. Only reached from the `value < 1e18` branch,
/// so `value` is provably below 10^18 and the `to_f64` conversion cannot
/// overflow.
fn log1000_trunc(value: &BigInt) -> u32 {
    let x = value.to_f64().expect("caller guarantees value < 1e18");
    let k = (x.ln() / 1000f64.ln()).trunc();
    k as u32
}

/// Python's `s[:-n]` — drop the last `n` *characters* (not bytes).
///
/// Load-bearing for accented numwords: "millón"[:-3] is "mil" and
/// "trillón"[:-3] is "tril", but "ó" is two bytes in UTF-8, so byte slicing
/// would corrupt them.
fn drop_last_chars(s: &str, n: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let keep = chars.len().saturating_sub(n);
    chars[..keep].iter().collect()
}

/// `Num2Word_ES_GT.CURRENCY_FORMS`, the class body verbatim.
///
/// Redefining the attribute *shadows* `Num2Word_ES`'s much larger table, so
/// this really is the whole of it — three codes. Confirmed against the live
/// interpreter, which is the only way to be sure given that `Num2Word_EN`
/// mutates the sibling dict on `Num2Word_EUR` at import time.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Python binds this tuple once and shares it between GTQ and USD.
    const CENTAVOS: [&str; 2] = ["centavo", "centavos"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "GTQ",
        CurrencyForms::new(&["quetzal", "quetzales"], &CENTAVOS),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "euros"], &["céntimo", "céntimos"]),
    );
    m.insert("USD", CurrencyForms::new(&["dólar", "dólares"], &CENTAVOS));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unmodified.
///
/// Nothing in the `ES_GT` → `ES` → `EUR` chain redefines or mutates this table
/// (`Num2Word_EN.__init__` rewrites `CURRENCY_FORMS`, not this), so the class
/// body is what runs — verified against the interpreter.
///
/// Ported whole because it is the data Python carries, but note that
/// `to_currency` looks up `CURRENCY_FORMS` *first* and raises on a miss, so the
/// only entry that can ever be observed is **USD**: it is the sole code present
/// in both tables. The other fifteen are dead.
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

pub struct LangEsGt {
    cards: Cards,
    maxval: BigInt,
    /// Python's `self.ords`. The source keys 1e3..1e15 as *floats*, but
    /// `ords[1000]` still hits `1e3` because `hash(1000) == hash(1000.0)` and
    /// `1000 == 1000.0`, so plain integer keys are equivalent here.
    ords: HashMap<BigInt, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEsGt {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsGt {
    pub fn new() -> Self {
        // setup(): gen_high_numwords([], [], lows) -> [] + lows. With units and
        // tens empty the comprehension yields nothing, so the elision table in
        // gen_high_numwords never applies and high_numwords == lows.
        let lows = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords, with ES's GIGA_SUFFIX = None and
        // MEGA_SUFFIX = "illón": cap = 3 + 6*len(high) = 57; the zip walks
        // range(57, 3, -6) and only cards[10**(n-3)] is ever set.
        let cap: i64 = 3 + 6 * lows.len() as i64;
        let mut n = cap;
        for word in lows.iter() {
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
        // 30 words -> values 29 down to 0.
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

        // MAXVAL = 1000 * list(self.cards.keys())[0]; the OrderedDict's first
        // key is the first high numword inserted, 10^54. So MAXVAL = 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

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
            (BigInt::from(1000), "milésim"),
            (BigInt::from(10u8).pow(6), "millonésim"),
            (BigInt::from(10u8).pow(9), "billonésim"),
            (BigInt::from(10u8).pow(12), "trillonésim"),
            (BigInt::from(10u8).pow(15), "cuadrillonésim"),
        ]
        .into_iter()
        .collect();

        LangEsGt {
            cards,
            maxval,
            ords,
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // ever read these tables; rebuilding them on each call is what made
            // an earlier revision of this port slower than the Python it
            // replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `self.ords[key]`, raising `KeyError` exactly as Python's dict does.
    ///
    /// The only reachable miss is `dec == 10**18` from the float-log
    /// over-estimate described in the module docs.
    fn ords_get(&self, key: &BigInt) -> Result<&'static str> {
        self.ords
            .get(key)
            .copied()
            .ok_or_else(|| N2WError::Key(key.to_string()))
    }

    /// `Num2Word_Base.verify_ordinal`, integer half only.
    ///
    /// The float check (`errmsg_floatord`) is unreachable: this port takes
    /// `BigInt`, so `value == int(value)` always holds.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(errmsg_negord(value)));
        }
        Ok(())
    }

    /// `Num2Word_ES.to_ordinal(value, gender="m")`.
    ///
    /// The trait exposes only the gender-less entry point, so `gender` is
    /// always "m" in practice, but the parameter is threaded through exactly
    /// as Python does so the branch structure stays faithful.
    fn to_ordinal_gender(&self, value: &BigInt, gender: &str) -> Result<String> {
        let mut gender_stem = if gender == "f" { "a" } else { "o" };

        self.verify_ordinal(value)?;

        let ten = BigInt::from(10);
        let twenty_nine = BigInt::from(29);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let e18 = BigInt::from(10u8).pow(18);

        let text: String = if value.is_zero() {
            String::new()
        } else if value <= &ten {
            format!("{}{}", self.ords_get(value)?, gender_stem)
        } else if value <= &twenty_nine && *value != BigInt::from(20) {
            // "According to RAE recommendations, simple forms are preferred up
            // to 30 / Ortography for sobreesdrújulas": the accent is dropped
            // and the feminine stem is discarded.
            gender_stem = "o";
            let dec = (value / &ten) * &ten;
            format!(
                "{}{}{}",
                self.ords_get(&dec)?.replace('é', "e"),
                gender_stem,
                self.to_ordinal_gender(&(value % &ten), gender)?
            )
        } else if value <= &hundred {
            let dec = (value / &ten) * &ten;
            format!(
                "{}{} {}",
                self.ords_get(&dec)?,
                gender_stem,
                self.to_ordinal_gender(&(value - &dec), gender)?
            )
        } else if value <= &thousand {
            let cen = (value / &hundred) * &hundred;
            format!(
                "{}{} {}",
                self.ords_get(&cen)?,
                gender_stem,
                self.to_ordinal_gender(&(value - &cen), gender)?
            )
        } else if value < &e18 {
            // dec = 1000 ** int(math.log(int(value), 1000)) — see log1000_trunc
            // for why this can exceed `value` or land on a missing key.
            let dec = BigInt::from(1000).pow(log1000_trunc(value));
            let (high_part, low_part) = value.div_mod_floor(&dec);

            // Python evaluates this statement before the format tuple.
            let cardinal = if high_part.is_one() {
                String::new()
            } else {
                self.to_cardinal(&high_part)?
            };

            // ...and the format tuple left to right, so ords[dec] raises
            // KeyError *before* to_ordinal(low_part) is ever called. Order
            // preserved: 10^18-1 must be a KeyError, not a RecursionError.
            let ord_dec = self.ords_get(&dec)?;

            // `dec > value` => high_part == 0 and low_part == value, i.e.
            // to_ordinal recursing on its own argument. Python spins until
            // RecursionError; we cannot, so we report it. See module docs.
            if &low_part == value {
                return Err(N2WError::Value(
                    "maximum recursion depth exceeded in comparison".to_string(),
                ));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                ord_dec,
                gender_stem,
                self.to_ordinal_gender(&low_part, gender)?
            )
        } else {
            self.to_cardinal(value)?
        };

        // Applied at every level, not just the top: "decimooctavo" ->
        // "decimoctavo". Rust's replace is non-overlapping and left-to-right,
        // matching Python's.
        Ok(text.trim().replace("oo", "o"))
    }
}

impl Lang for LangEsGt {

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "GTQ"
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

    /// `Num2Word_Base.to_cardinal`, but with ES's own `errmsg_toobig`.
    ///
    /// `default_to_cardinal` hardcodes the English overflow message, and
    /// `base.rs` is off-limits, so the overflow check is done here first (on
    /// `abs(value)`, exactly as Python does) and the default only runs once
    /// the value is known to be in range.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let v = value.abs();
        if &v >= self.maxval() {
            return Err(N2WError::Overflow(errmsg_toobig(&v, self.maxval())));
        }
        default_to_cardinal(self, value)
    }

    /// `Num2Word_ES.merge`.
    ///
    /// `self.gender_stem` is always "o" here — `to_ordinal`'s gender handling
    /// uses a local variable and never writes the attribute back.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let one = BigInt::one();
        let five = BigInt::from(5);
        let seven = BigInt::from(7);
        let nine = BigInt::from(9);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum == &one {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        } else if cnum == &hundred && !(nnum % &thousand).is_zero() {
            // Python: `elif cnum == 100 and not nnum % 1000 == 0` — `==` binds
            // tighter than `not`. "cien" -> "ciento" before a non-round tail,
            // which is why 100_000_000 stays "cien millones" (1e6 % 1000 == 0)
            // while 101 becomes "ciento uno".
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if cnum < &hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && cnum > &one {
            // "millón" -> "millones", "billón" -> "billones", "trillón" ->
            // "trillones". Character-based slicing: "ó" is two bytes.
            ntext = format!("{}lones", drop_last_chars(&ntext, 3));
        }

        if nnum == &hundred {
            if cnum == &five {
                ctext = "quinien".to_string();
                ntext = String::new();
            } else if cnum == &seven {
                ctext = "sete".to_string();
            } else if cnum == &nine {
                ctext = "nove".to_string();
            }
            ntext.push('t');
            ntext.push_str(GENDER_STEM);
            ntext.push('s');
        } else {
            // Spanish apocopates 'uno' before a noun like 'mil'/'millones':
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
        self.to_ordinal_gender(value, "m")
    }

    /// `Num2Word_ES.to_ordinal_num(value, gender="m")` — the digits verbatim
    /// plus a masculine/feminine ordinal indicator. Default gender is "m", so
    /// this is always "º" here.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year` ignores era/suffix handling entirely and is just
    /// `self.to_cardinal(int(val))`, so negative years read "menos ...".
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
    /// `Num2Word_ES_GT`: idiomatic medio/tercio/cuarto for denominators
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
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` are
    // inherited from `Num2Word_Base` unchanged, and `CURRENCY_PRECISION` is
    // Base's empty dict, so the trait defaults already mirror all four. Only
    // the two data tables, the class name, EUR's plural rule and the
    // `to_currency` chain are language-specific.

    fn lang_name(&self) -> &str {
        "Num2Word_ES_GT"
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
    /// would raise IndexError. All three of GT's entries carry two forms, so
    /// that is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES_GT.to_currency` wrapped around `Num2Word_ES.to_currency`.
    ///
    /// Both Python frames are inlined here, in order:
    ///
    /// 1. `Num2Word_ES.to_currency` — a bespoke branch for pure `int`s, else
    ///    `Num2Word_Base.to_currency` plus six literal `str.replace` fix-ups.
    /// 2. `Num2Word_ES_GT.to_currency` — one more `result.replace("uno", "un")`
    ///    over whatever came back.
    ///
    /// The replace cascade must keep this exact order and these exact literals:
    /// the accented rules (`"veintiuno euro"` → `"veintiún euro"`) only win
    /// because they run before GT's unanchored `"uno"` → `"un"` gets to eat the
    /// same substring. See bugs 8-11 in the module docs.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` means the caller omitted `separator=`, so this language's own
        // default applies — `Num2Word_ES_GT.to_currency(..., separator=" y")`.
        let separator = separator.unwrap_or(self.default_separator());

        // ---- Num2Word_ES.to_currency ------------------------------------
        let result = match val {
            // "Handle integers specially - just add currency name without
            // cents. Only pure integers, NOT floats that happen to be whole
            // numbers."
            CurrencyValue::Int(v) => match self.currency_forms.get(currency) {
                // Python catches `(KeyError, AttributeError)` and falls back to
                // `super().to_currency(...)`, which repeats the same lookup and
                // raises NotImplementedError from there. Delegating keeps that
                // provenance rather than duplicating the message.
                None => default_to_currency(self, val, currency, cents, separator, adjective)?,
                Some(forms) => {
                    let cr1 = &forms.unit;

                    // Bug 8 (fixed): Python uses `self.negword` raw, so its
                    // trailing space doubled up in the format string.
                    let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
                    let abs_val = v.abs();

                    // Bug 11: computed unconditionally, discarded below when
                    // abs_val == 1. Kept in Python's order because it can raise
                    // OverflowError, and only after the forms lookup.
                    let mut money_str = self.to_cardinal(&abs_val)?;

                    let currency_str: &str = if abs_val.is_one() {
                        // "Convert 'uno' to 'un' for currency".
                        money_str = "un".to_string();
                        cr1[0].as_str()
                    } else if cr1.len() > 1 {
                        cr1[1].as_str()
                    } else {
                        cr1[0].as_str()
                    };

                    // Python: `("%s %s %s" % (...)).strip()`.
                    format!("{} {} {}", minus_str, money_str, currency_str)
                        .trim()
                        .to_string()
                }
            },

            // "For floats, use the parent class implementation but fix
            // 'uno' -> 'un'".
            CurrencyValue::Decimal { .. } => {
                let r = default_to_currency(self, val, currency, cents, separator, adjective)?;
                // Order matters: the accented two-word rules must consume their
                // "uno" before the bare ones can.
                let r = r.replace("veintiuno euro", "veintiún euro");
                let r = r.replace("veintiuno céntimo", "veintiún céntimo");
                let r = r.replace("uno euro", "un euro");
                let r = r.replace("uno céntimo", "un céntimo");
                let r = r.replace("uno centavo", "un centavo");
                r.replace("uno dólar", "un dólar")
            }
        };

        // ---- Num2Word_ES_GT.to_currency ---------------------------------
        // "Handle exception, in spanish is 'un euro' and not 'uno euro'".
        // Unanchored and global (bug 10): this also rewrites "veintiuno" ->
        // "veintiun" and the interior of "un millón uno".
        // The blanket rewrite leaves "veintiun", which is never written
        // without its accent: "veintiún dólares" (#253).
        Ok(result.replace("uno", "un").replace("veintiun ", "veintiún "))
    }
}
