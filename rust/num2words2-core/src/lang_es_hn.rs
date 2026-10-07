//! Port of `lang_ES_HN.py` (Spanish — Honduras).
//!
//! # What the key actually resolves to
//!
//! `CONVERTER_CLASSES["es_HN"]` → `lang_ES_HN.Num2Word_ES_HN`, which subclasses
//! `Num2Word_ES` and overrides **only** `CURRENCY_FORMS` and `to_currency`.
//! Both are out of scope, so for `to_cardinal` / `to_ordinal` /
//! `to_ordinal_num` / `to_year` this file is a port of `Num2Word_ES` verbatim.
//! Inheritance chain chased: `Num2Word_ES_HN` → `Num2Word_ES` → `Num2Word_EUR`
//! → `Num2Word_Base`. (`lang_EU` is Basque and is *not* in this chain despite
//! the similar name.)
//!
//! # Shape: engine
//!
//! `Num2Word_ES.setup` defines `high_numwords`/`mid_numwords`/`low_numwords`
//! and `merge`, and does **not** override `Num2Word_Base.to_cardinal` in any
//! way that matters here (see "_pending_ordinal" below). So `cards` + `maxval`
//! + `merge` are supplied and the default `to_cardinal` in `base.rs` drives
//! `splitnum`/`clean`.
//!
//! ## Card table
//!
//! `setup` calls `gen_high_numwords([], [], lows)`. With empty `units`/`tens`
//! the comprehension `[u + t for t in tens for u in units]` is empty, so the
//! function degenerates to returning `lows` unchanged — none of EUR's Latin
//! elision rules ever fire. `high_numwords` is therefore exactly
//! `["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"]`.
//!
//! `Num2Word_ES` sets `GIGA_SUFFIX = None` and `MEGA_SUFFIX = "illón"`, so
//! `Num2Word_EUR.set_high_numwords` (`cap = 3 + 6*9 = 57`;
//! `zip(high, range(57, 3, -6))`) emits the *long scale* with a **step of 6**
//! and no `-illiard` rung:
//!
//! ```text
//! 10^54 nonillón   10^48 octillón   10^42 septillón  10^36 sextillón
//! 10^30 quintillón 10^24 cuatrillón 10^18 trillón    10^12 billón
//! 10^6  millón
//! ```
//!
//! There is deliberately **no 10^9 card**: 10^9 is built by `merge` as
//! "mil millones", and 10^15 as "mil billones". `MAXVAL = 1000 * 10^54 = 10^57`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following were verified against
//! the interpreter and are preserved byte for byte:
//!
//! 1. **`to_ordinal` glues a cardinal onto the scale stem with no space.**
//!    `to_ordinal(99999)` == "noventa y nuevemilésimo noningentésimo nonagésimo
//!    noveno" and `to_ordinal(123456)` == "ciento veintitrésmilésimo ...".
//!    The `%s%s%s %s` format concatenates `cardinal` and `self.ords[dec]`
//!    directly. Looks wrong; it is what Python emits.
//! 2. ~~**`to_ordinal(20)` == "vigesimo"**~~, unaccented: the `value <= 29`
//!    branch builds the *sobreesdrújula* spelling ("decimoprimero",
//!    "vigesimoquinto"), and 20 landed in it too. Fixed (gladiaio/num2words2#252): 20 keeps its accent and the caller's
//!    gender ("vigésimo", "vigésima", "centésimo vigésimo"); only the fused
//!    21..=29 forms drop it ("vigesimoprimero"), as the RAE spells them.
//! 3. **`.replace("oo", "o")` is applied at every recursion level**, not just
//!    the top. This is the intentional "decimooctavo" → "decimoctavo" fix, but
//!    it fires on any "oo" the concatenation happens to produce.
//! 4. **`dec = 1000 ** int(math.log(int(value), 1000))` is float math**, and it
//!    overshoots just below two exact powers of 1000. See [`log1000_floor`] and
//!    the two crash bands below. This is the single nastiest thing in the file.
//! 5. `to_ordinal(10**18)` == "un trillón" — the `value < 1e18` guard fails, so
//!    the `else` arm silently returns the **cardinal** instead of an ordinal.
//!    Likewise `to_ordinal(10**21)` == "mil trillones".
//! 6. `to_ordinal(0)` raises ValueError here (Python returns `""`; #160).
//!
//! ## The two `math.log` crash bands
//!
//! `math.log(v, 1000)` is `log(float(v)) / log(1000.0)` in CPython. Near
//! 1000^5 and 1000^6 the f64 quotient rounds *up* to a whole number, so
//! `int()` floors to `k` when the true floor is `k-1`, and `dec` ends up
//! **larger than `value`**. Then `divmod(value, dec) == (0, value)` and the
//! function recurses on the *same* value. Two distinct outcomes:
//!
//! * **Band A** — `v` in `[999999999999996, 999999999999999]` (4 values, just
//!   under 10^15): `k` = 5, `dec` = 10^15. `self.ords[1e15]` **exists**
//!   ("cuadrillonésim"), so Python proceeds to `to_ordinal(low_part)` with
//!   `low_part == value` and dies with **`RecursionError`**.
//!   Band A also kills values that merely *recurse into* it: `to_ordinal(10^16
//!   - 1)` splits as `high_part` = 9, `low_part` = 999999999999999, and the
//!   recursive call lands in the band. Same for `10^17 - 1`. So the observable
//!   `RecursionError` set is larger than the 4 direct members — the guard below
//!   keys off the `low_part == value` condition, which catches every route in.
//! * **Band B** — `v` in `[999999999999995072, 999999999999999999]` (4928
//!   values, just under 10^18): `k` = 6, `dec` = 10^18. `self.ords` stops at
//!   `1e15`, so `self.ords[dec]` raises **`KeyError: 1000000000000000000`**
//!   *before* the recursion is ever reached (Python builds the `%` tuple
//!   left-to-right: `cardinal`, then `self.ords[dec]`, then `to_ordinal(...)`).
//!
//! Band B maps cleanly to [`N2WError::Key`]. Band A does **not** map to
//! anything: `RecursionError` has no `N2WError` variant. Since `low_part ==
//! value` is exactly equivalent to non-termination (each call re-derives the
//! identical `dec`), it is detected up front and reported as
//! [`N2WError::Value`] rather than blowing the Rust stack. **The exception
//! *type* is therefore not faithful for those 4 values** — see the port report.
//!
//! # Cross-call mutable state (`_pending_ordinal`) — NOT reproduced
//!
//! `Num2Word_ES.str_to_number` recognises Spanish ordinal-suffix strings
//! ("1ro", "2da", ...), stashes `self._pending_ordinal = (int(digits), gender)`
//! and returns the bare int; `Num2Word_ES.to_cardinal` then consumes and clears
//! that flag to return `to_ordinal(value, gender=...)` instead of the cardinal.
//! That is a stateful handshake spanning two method calls, and the Rust path is
//! stateless — this port always takes the `pending is None` branch and delegates
//! to `Num2Word_Base.to_cardinal`.
//!
//! This is safe **only** because `num2words()` gates the Rust fast path on
//! `_plain_int = type(number) is int`, captured *before* `str_to_number` runs
//! (and additionally on `not kwargs`). A plain int can never set the flag, so
//! Rust is never handed a value with a pending ordinal. If that gate is ever
//! relaxed, this port will return "uno" where Python returns "primero", and the
//! flag will leak into the following call. Flagged in the report.
//!
//! # Gender
//!
//! `to_ordinal(value, gender="m")` and `to_ordinal_num(value, gender="m")` take
//! a gender kwarg, but the Rust fast path only runs when `not kwargs`, so
//! gender is always "m" and `gender_stem` is always "o". The `value <= 29`
//! branch hard-sets `gender_stem = "o"` anyway. `self.gender_stem` (used by
//! `merge`) is set once in `setup` to "o" and is never mutated — `to_ordinal`
//! shadows it with a *local*, so there is no hidden coupling between the two.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use num_bigint::BigInt;
use crate::floatpath::FloatValue;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `Num2Word_ES.setup` and never reassigned.
const GENDER_STEM: &str = "o";

/// `Num2Word_ES.ords`.
///
/// Python's dict mixes int keys (1..=900) with float keys (1e3, 1e6, 1e9,
/// 1e12, 1e15). Lookups are always by int, and `hash(1000) == hash(1e3)` with
/// `1000 == 1e3`, so `ords[1000]` finds the `1e3` entry. Modelled here as one
/// numeric table — every key fits in i64 (max 10^15).
///
/// Note the gaps: there is no entry for 11..=19, and nothing above 1e15 (which
/// is what makes Band B a `KeyError`).
const ORDS: &[(i64, &str)] = &[
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
];

/// `self.ords[key]`, returning `None` where Python would raise `KeyError`.
fn ords_get(key: i64) -> Option<&'static str> {
    ORDS.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// Python's `s[:-n]` — drop the last `n` **characters**.
///
/// Byte slicing would corrupt "millón"[:-3] ("ó" is two bytes), which is
/// exactly the slice `merge` performs to build "millones".
fn strip_last_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if n >= count {
        return String::new();
    }
    s.chars().take(count - n).collect()
}

fn pow10(n: u32) -> BigInt {
    BigInt::from(10u32).pow(n)
}

/// `int(math.log(int(value), 1000))`, reproduced exactly — **including the
/// float error that makes it overshoot**.
///
/// CPython's `math.log(x, base)` is `loghelper(x) / loghelper(base)`; for an
/// int that fits a double it is literally `log(float(x)) / log(1000.0)`. That
/// identity was verified against the interpreter for the boundary values.
///
/// `value` is caller-guaranteed to be in `(1000, 10^18)` — the `value <= 1e3`
/// and `value < 1e18` guards in `to_ordinal` bracket it — so the `u64` cast is
/// proven safe, and `u64 as f64` rounds to nearest exactly as
/// `PyLong_AsDouble` does. `f64 as i32` truncates toward zero, as `int()` does.
///
/// Do **not** "fix" this into an exact integer log: `log1000_floor(10^15 - 1)`
/// returning 5 (not 4) is the documented Band A bug.
fn log1000_floor(value: u64) -> i32 {
    ((value as f64).ln() / 1000f64.ln()) as i32
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

pub struct LangEsHn {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    /// `Num2Word_ES_HN.CURRENCY_FORMS` — a single `HNL` entry that *replaces*
    /// (does not merge with) the inherited EUR/ES tables, so only "HNL" is a
    /// known code here. Built once in [`LangEsHn::new`].
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangEsHn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEsHn {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_ES.setup: gen_high_numwords([], [], lows) == lows, because
        // the [u + t for t in tens for u in units] comprehension is empty.
        let high = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        // Num2Word_EUR.set_high_numwords with GIGA_SUFFIX=None,
        // MEGA_SUFFIX="illón": cap = 3 + 6*len(high); zip(high, range(cap, 3, -6)).
        // The GIGA rung is skipped entirely, so the step is 6, not 3.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
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

        // MAXVAL = 1000 * list(self.cards.keys())[0]; the OrderedDict's first
        // key is the highest card (10^54), so MAXVAL == 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // `Num2Word_ES_HN.CURRENCY_FORMS = {'HNL': (('lempira','lempiras'),
        // ('centavo','centavos'))}` — the sole known code.
        let mut currency_forms: HashMap<&'static str, CurrencyForms> = HashMap::new();
        currency_forms.insert(
            "HNL",
            CurrencyForms::new(&["lempira", "lempiras"], &["centavo", "centavos"]),
        );

        LangEsHn {
            cards,
            maxval,
            // setup: self.exclude_title = ["y", "menos", "punto"].
            // Only consulted when is_title is true, which nothing sets here.
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            currency_forms,
        }
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The `value == int(value)` check can't fail for a BigInt, so only the
    /// `abs(value) == value` (non-negative) check has teeth. Both raise
    /// `TypeError`; the message is `errmsg_negord` from `Num2Word_ES.setup`
    /// (note the adjacent-literal concatenation in the Python source produces
    /// "...tratado como un ordinal.").
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
    /// `gender` is threaded through the recursion exactly as Python does: the
    /// 11..=29 branch forces its *local* stem to "o" but still forwards the
    /// caller's gender downward. The trait's `to_ordinal` enters with "m";
    /// `to_ordinal_kw` passes the caller's value ("f" -> "a" stems).
    fn to_ordinal_gender(&self, value: &BigInt, gender: &str) -> Result<String> {
        let gender_stem = if gender == "f" { "a" } else { "o" };
        self.verify_ordinal(value)?;

        let text: String = if value.is_zero() {
            String::new()
        } else if value <= &BigInt::from(10) {
            let v = value.to_i64().expect("0 < value <= 10");
            format!("{}{}", ords_get(v).expect("ords 1..=10 present"), gender_stem)
        } else if value <= &BigInt::from(29) && *value != BigInt::from(20) {
            // RAE: simple forms preferred up to 30; the accent is stripped for
            // the sobreesdrújula spelling ("décim" -> "decim"). gender_stem is
            // forced to "o" here even when gender == "f" — but the *unit*
            // keeps the caller's gender ("vigesimoprimera").
            let v = value.to_i64().expect("11 <= value <= 29");
            let dec = (v / 10) * 10;
            format!(
                "{}{}{}",
                ords_get(dec).expect("ords 10/20 present").replace('é', "e"),
                "o",
                self.to_ordinal_gender(&BigInt::from(v % 10), gender)?
            )
        } else if value <= &BigInt::from(100) {
            let v = value.to_i64().expect("30 <= value <= 100");
            let dec = (v / 10) * 10;
            format!(
                "{}{} {}",
                ords_get(dec).expect("ords 30..=100 present"),
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(v - dec), gender)?
            )
        } else if value <= &BigInt::from(1000) {
            let v = value.to_i64().expect("101 <= value <= 1000");
            let cen = (v / 100) * 100;
            format!(
                "{}{} {}",
                ords_get(cen).expect("ords 100..=900,1000 present"),
                gender_stem,
                self.to_ordinal_gender(&BigInt::from(v - cen), gender)?
            )
        } else if value < &pow10(18) {
            // Round down to the nearest 1e(3n) — via float log, bugs and all.
            let v = value.to_u64().expect("1000 < value < 10^18 fits u64");
            let k = log1000_floor(v);
            let dec = BigInt::from(1000u32).pow(k as u32);

            let (high_part, low_part) = value.div_rem(&dec);

            // Python evaluates this line first, then builds the % tuple
            // left-to-right: cardinal, ords[dec], gender_stem, to_ordinal(low).
            // The ordering decides Band A (RecursionError) vs Band B (KeyError).
            let cardinal = if !high_part.is_one() {
                self.to_cardinal(&high_part)?
            } else {
                String::new()
            };

            // Band B: dec == 10^18 overshoots the ords table -> KeyError,
            // raised before the recursion below is reached.
            let stem = dec
                .to_i64()
                .and_then(ords_get)
                .ok_or_else(|| N2WError::Key(format!("{}", dec)))?;

            // Band A: the float log overshot, so dec > value, high_part == 0
            // and low_part == value. Python recurses forever and dies with
            // RecursionError, which has no N2WError variant; reported as Value.
            if &low_part == value {
                return Err(N2WError::Value(format!(
                    "maximum recursion depth exceeded in comparison (Python \
                     RecursionError: math.log float overshoot made dec={} > \
                     value={}, so to_ordinal recurses on itself)",
                    dec, value
                )));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                stem,
                gender_stem,
                self.to_ordinal_gender(&low_part, gender)?
            )
        } else {
            // value >= 10^18: silently returns the *cardinal*, not an ordinal.
            self.to_cardinal(value)?
        };

        // "decimooctavo" -> "decimoctavo". Applied at every level, not just the
        // outermost one.
        Ok(text.trim().replace("oo", "o"))
    }
}

impl Lang for LangEsHn {

    fn es_currency_ordinal_fires(&self) -> bool {
        true
    }

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "HNL"
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
    /// `self.gender_stem` is the constant "o"; `to_ordinal`'s gender_stem is a
    /// separate local and never reaches here.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext_in, cnum) = l;
        let (ntext_in, nnum) = r;
        let mut ctext = ctext_in.to_string();
        let mut ntext = ntext_in.to_string();

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
            // Python: `not nnum % 1000 == 0` parses as `not (nnum % 1000 == 0)`.
            // "cien" -> "ciento", but only when the follower isn't a clean
            // multiple of 1000 ("cien mil" keeps "cien").
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if cnum < &hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && cnum > &one {
            // "millón" -> "millones", "billón" -> "billones". Every high card
            // is 10^(6k), so this fires for all of them (and never for "mil").
            ntext = format!("{}lones", strip_last_chars(&ntext, 3));
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
            // Spanish apocopates "uno" before a noun like "mil"/"millones":
            // 31000 -> "treinta y un mil", 21000 -> "veintiún mil".
            if nnum >= &thousand {
                if ctext.ends_with("veintiuno") {
                    ctext = format!("{}ún", strip_last_chars(&ctext, 3));
                } else if ctext.ends_with("uno") {
                    ctext = strip_last_chars(&ctext, 1);
                }
            }
            ntext = format!(" {}", ntext);
        }

        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    /// `Num2Word_ES.to_cardinal` minus the `_pending_ordinal` handshake, which
    /// a plain int can never trigger (see module docs). Delegates to
    /// `Num2Word_Base.to_cardinal` == `base.rs::default_to_cardinal`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        crate::base::default_to_cardinal(self, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        crate::lang_es::es_reject_zero_ordinal(value)?;
        // Python's default gender is "m".
        self.to_ordinal_gender(value, "m")
    }

    /// `Num2Word_ES.to_ordinal_num(value, gender="m")` — verify, then the raw
    /// digits plus the masculine ordinal indicator. Note `to_ordinal_num(0)`
    /// == "0º" (verify_ordinal permits zero).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_ES.to_year(val, suffix=None, longval=True)` ignores both
    /// kwargs and is a plain alias for `to_cardinal` — no BC/AD suffix, no
    /// "nineteen eighty-four" pairing. Negative years just get "menos".
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
    /// `Num2Word_ES_HN`: idiomatic medio/tercio/cuarto for denominators
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

    // ---- currency ----------------------------------------------------------
    //
    // `Num2Word_ES_HN.to_currency` calls `super(Num2Word_ES, self).to_currency`
    // — i.e. it *skips* `Num2Word_ES`'s currency override and runs
    // `Num2Word_Base.to_currency` (EUR defines none) — then applies a blanket
    // `.replace("uno", "un")`. So this is the base currency machinery with the
    // HNL forms, EUR's `pluralize`, and the `uno`->`un` fix-up. Note it does
    // NOT get ES's `abs_val == 1 -> "un"` special-case; the base int path calls
    // `money_verbose -> to_cardinal`, which is what fires the `_pending_ordinal`
    // stash for "1ro" on the Python side (see `concerns`).

    /// `Num2Word_ES_HN.CURRENCY_FORMS[code]` — only "HNL" is known.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`. Inherited by
    /// `Num2Word_ES` (whose own currency path is bypassed here) and reached by
    /// the base currency machinery. Every HNL form pair has arity 2, so the
    /// index is always in range.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let idx = if n.is_one() { 0 } else { 1 };
        forms
            .get(idx)
            .cloned()
            .ok_or_else(|| N2WError::Index("list index out of range".into()))
    }

    /// `Num2Word_ES_HN.to_currency` = `Num2Word_Base.to_currency` +
    /// `.replace("uno", "un")`. The blanket replace is faithful (it also turns
    /// "veintiuno" into "veintiun"), matching the Python one-liner exactly.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        let result = crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )?;
        Ok(result.replace("uno", "un"))
    }
}

// # Float / Decimal path
//
// `Num2Word_ES_HN` → `Num2Word_ES` → `Num2Word_EUR` → `Num2Word_Base`.
// None of those overrides `to_cardinal_float` or `float2tuple`
// (`to_cardinal_float overridden: False` in the live interpreter), and
// `Num2Word_ES.to_cardinal` degenerates to `super().to_cardinal` for anything
// that is not a pending-ordinal string — a `float`/`Decimal` never sets
// `_pending_ordinal`. So `num2words(0.5)` walks
// `Num2Word_ES.to_cardinal(0.5)` → `Num2Word_Base.to_cardinal(0.5)` →
// (`int(0.5) != 0.5`) → `Num2Word_Base.to_cardinal_float(0.5)`.
//
// That is precisely the trait default `to_cardinal_float`
// (`floatpath::default_to_cardinal_float`), which `LangEsHn` therefore
// inherits **unchanged**. Overriding it would only risk drift. The tests
// below lock the inherited behaviour to the frozen corpus, including the
// two f64-artefact rescues (1.005, 2.675) and the trillion-scale Decimal
// row that a `float()` cast would corrupt (issue #603).

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use crate::floatpath::FloatValue;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// Build a `FloatValue::Float` the way the binding does: the raw f64 plus
    /// the repr-derived precision (`abs(Decimal(str(v)).as_tuple().exponent)`).
    /// Keeping the raw f64 preserves the binary artefacts float2tuple relies on.
    fn flt(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    /// Build a `FloatValue::Decimal` from the literal decimal string; precision
    /// is `abs(exponent)` of that literal. The Decimal arm is exact, never a
    /// float cast.
    fn dec(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// Every `"lang": "es_HN", "to": "cardinal"` corpus row whose `arg` is a
    /// non-integral float — the ones that actually reach the float path. Whole
    /// floats (`0.0`, `1.0`) are routed to the integer `to_cardinal` by Python's
    /// `assert int(value) == value`, so they are deliberately absent here.
    #[test]
    fn cardinal_float_corpus_rows() {
        let l = LangEsHn::new();
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
            // Negative with pre == 0: base prepends negword.strip() ("menos")
            // because int(-0.5) == 0 carries no sign.
            (flt(-0.5, 1), "menos cero punto cinco"),
            (flt(-1.5, 1), "menos uno punto cinco"),
            // Negative with pre != 0: sign rides on to_cardinal(-12) = "menos
            // doce"; no extra negword.
            (flt(-12.34, 2), "menos doce punto tres cuatro"),
            // f64 artefacts: 1.005 -> 4.999.../1000 rescued to 005, 2.675 ->
            // 674.9999999999998 rescued to 675 by the `< 0.01` heuristic
            // (round_ties_even, not f64::round).
            (flt(1.005, 3), "uno punto cero cero cinco"),
            (flt(2.675, 3), "dos punto seis siete cinco"),
        ];
        for (v, want) in rows {
            let got = l.to_cardinal_float(&v, None).unwrap();
            assert_eq!(got, want, "to_cardinal_float({:?})", v);
        }
    }

    /// Every `"lang": "es_HN", "to": "cardinal_dec"` corpus row — Decimal input,
    /// the exact arbitrary-precision arm. Includes the issue #603 trillion-scale
    /// value that a float cast would silently round to `.98`.
    #[test]
    fn cardinal_dec_corpus_rows() {
        let l = LangEsHn::new();
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
}
