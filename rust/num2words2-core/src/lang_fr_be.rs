//! Port of `lang_FR_BE.py` (Belgian French), via its
//! `Num2Word_FR` → `Num2Word_EUR` → `Num2Word_Base` ancestry.
//!
//! Registry check: `__init__.py` maps `"fr_BE"` → `lang_FR_BE.Num2Word_FR_BE()`,
//! so this file ports `Num2Word_FR_BE` — the class the key actually resolves to.
//!
//! Shape: **engine**. `Num2Word_FR_BE.setup()` chains up through
//! `Num2Word_FR.setup()` → `Num2Word_EUR.setup()`, populating
//! `high_numwords`/`mid_numwords`/`low_numwords`, then overrides
//! `mid_numwords` with the Belgian tens. `to_cardinal` is *not* overridden
//! anywhere in the chain, so `Num2Word_Base.to_cardinal` drives
//! `splitnum`/`clean`/`merge` — i.e. the `base.rs` default. This file
//! therefore supplies `cards` + `maxval` + `merge` and overrides only
//! `to_ordinal` / `to_ordinal_num` / `to_year` / `to_fraction` plus their
//! float/Decimal entries (all defined on `Num2Word_FR`).
//!
//! # What Belgian French changes vs. metropolitan `lang_FR`
//!
//! Two deltas, both load-bearing:
//!
//! 1. `mid_numwords` gains 90 "nonante" and 70 "septante" (metropolitan FR has
//!    neither — it builds 70/90 by vigesimal composition off 60), and 80 is
//!    **"quatre-vingt" without the trailing `s`** (FR uses "quatre-vingts").
//!    Hence `to_cardinal(80)` == "quatre-vingt" here but "quatre-vingts" in FR.
//! 2. `merge` joins a trailing 1 with **" et "** (spaces) rather than FR's
//!    **"-et-"**, and drops FR's `cnum != 80` guard. So 21 → "vingt et un",
//!    71 → "septante et un", and 81 → "quatre-vingt et un" (FR would emit
//!    "quatre-vingt-un" for 81, since its guard suppresses the "et").
//!
//! # Cards / MAXVAL
//!
//! `Num2Word_EUR.setup` builds `high_numwords` = `["cent"] + gen_high_numwords(...)`
//! = 1 + (10 units × 9 tens) + 9 lows = **100** entries. `set_high_numwords`
//! then does `cap = 3 + 6*100 = 603` and `zip(high, range(603, 3, -6))` —
//! `range(603, 3, -6)` yields exactly 100 values (603 down to 9), so `zip`
//! consumes all of `high` with nothing truncated. Each step inserts *two*
//! cards on the long scale: `10**n` → word+"illiard" and `10**(n-3)` →
//! word+"illion". The last pair (word "m", n=9) gives
//! `10**9` → "milliard" and `10**6` → "million".
//!
//! `MAXVAL = 1000 * list(self.cards.keys())[0]`. Python's `cards` is an
//! `OrderedDict` whose first *inserted* key is `10**603`, which is also its
//! maximum — so `Cards::highest()` reproduces it — giving `MAXVAL = 10**606`.
//! Values `>= MAXVAL` raise `OverflowError` in `base.rs`'s
//! `default_to_cardinal`, matching `Num2Word_Base.to_cardinal`.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits (all confirmed against the frozen corpus):
//!
//! 1. `to_ordinal(0)` == **"zéroième"**. `to_cardinal(0)` is "zéro"; it matches
//!    neither `ords` key, its last char is "o" (not "e"), and it does not end
//!    in "ts"/"ents" — so the bare "ième" suffix is glued on. Not a real word.
//! 2. `to_ordinal_num(0)` == **"0me"** — `Num2Word_FR.to_ordinal_num` only
//!    special-cases `value == 1` ("1er"); everything else, including 0, gets
//!    "me".
//! 3. **Fixed (#165).** Python's `_BIG_UNITS` branch re-attaches the `s`
//!    it stripped, so `to_ordinal(10**7)` == "dix millionièmes". The port
//!    drops it like the "cents"/"vingts" agreement `s` (#65):
//!    "dix millionième", "cent milliardième".
//! 4. `word.endswith("ents")` in the `to_ordinal` fallback is dead code —
//!    any string ending in "ents" already ends in "ts", so the first
//!    disjunct always fires first. Kept verbatim.
//! 5. `to_ordinal(2000)` == "deux millième", not "deux millièmes" — "mille"
//!    is absent from `_BIG_UNITS`, so it falls through to the generic
//!    `e`-stripping path and never pluralizes.
//!
//! # Errors
//!
//! `Num2Word_Base.verify_ordinal` raises **TypeError** for negatives, so
//! `to_ordinal(-1)` and `to_ordinal_num(-1)` → `N2WError::Type`. `to_cardinal`
//! accepts negatives (it strips the sign first) and `to_year` delegates
//! straight to `to_cardinal`, so both are total over the integers below
//! MAXVAL. No `IndexError`/`KeyError`/`ValueError` crash sites exist in this
//! chain for integer input.
//!
//! # Cross-call mutable state
//!
//! None. Nothing in the `FR_BE` → `FR` → `EUR` → `Base` chain stashes a flag
//! in one method for another to consume (contrast `lang_ES.str_to_number`'s
//! `_pending_ordinal`). `self.precision` is written by the float paths only,
//! which are out of scope here.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// `Num2Word_FR._BIG_UNITS`, in source order. Order is load-bearing: the
/// `to_ordinal` loop returns on first match, and the longer names must be
/// tested before the shorter ones they contain as substrings.
const BIG_UNITS: [&str; 6] = [
    "trilliard",
    "trillion",
    "billiard",
    "billion",
    "milliard",
    "million",
];

/// Port of `Num2Word_EUR.gen_high_numwords`.
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
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// `Num2Word_FR.CURRENCY_FORMS`.
///
/// **Not** `Num2Word_EUR`'s table, and deliberately so. `Num2Word_EN.__init__`
/// mutates `Num2Word_EUR.CURRENCY_FORMS` in place at import time (EUR ->
/// `("euro", "euros"), ("cent", "cents")`, GBP -> `("pound", "pounds")`), and
/// every class that inherits that dict without defining its own therefore
/// serves English forms. `Num2Word_FR` **defines its own** `CURRENCY_FORMS` as
/// a class attribute, which shadows EUR's in the MRO, so `Num2Word_FR_BE`
/// (FR_BE -> FR -> EUR -> Base) never sees the mutation. Verified against the
/// live interpreter: `fr_BE`'s EUR is `("centime", "centimes")`, not
/// `("cent", "cents")`, and the corpus agrees ("douze euros et trente-quatre
/// centimes", "douze livres et trente-quatre pence").
///
/// Consequence: FR's table is *smaller* than the EN-mutated one. KWD, BHD,
/// CHF, AED, SGD, ... are simply absent, which drives the two-faced behaviour
/// documented on `to_currency` below.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const DOLLARS: [&str; 2] = ["dollar", "dollars"];
    const CENTS: [&str; 2] = ["cent", "cents"];
    const CENTIMES: [&str; 2] = ["centime", "centimes"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &CENTIMES));
    m.insert("USD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("FRF", CurrencyForms::new(&["franc", "francs"], &CENTIMES));
    m.insert("CAD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("AUD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("HKD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("GBP", CurrencyForms::new(&["livre", "livres"], &["penny", "pence"]));
    // NB: fen/jiaos and yen/yens are FR's own literals, mismatched subunits
    // and all. Not a transcription slip — see the module quirk list.
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuans"], &["fen", "jiaos"]));
    m.insert("JPY", CurrencyForms::new(&["yen", "yens"], &["sen", "sens"]));
    m.insert("INR", CurrencyForms::new(&["roupie", "roupies"], &["paisa", "paisas"]));
    m.insert(
        "RUB",
        CurrencyForms::new(&["rouble", "roubles"], &["kopeck", "kopecks"]),
    );
    m.insert("KRW", CurrencyForms::new(&["won", "wons"], &["jeon", "jeons"]));
    m.insert(
        "MXN",
        CurrencyForms::new(&["peso", "pesos"], &["centavo", "centavos"]),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — neither
/// `Num2Word_FR` nor `Num2Word_FR_BE` defines one, and unlike `CURRENCY_FORMS`
/// nothing mutates it. Only the codes present in **both** this table and
/// `build_currency_forms` are reachable (USD, CAD, AUD, INR, JPY, KRW, MXN,
/// RUB); the rest are dead entries carried for fidelity.
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    HashMap::from([
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
    ])
}

/// Python `str(float)`/`repr(float)`, used only to build error-message text
/// (`errmsg_floatord`, `errmsg_negord`). The corpora compare exception
/// *types*; the text follows the Python format strings. Whole floats keep
/// their ".0" ("21.0"), |v| >= 1e16 switches to Python's exponent form
/// ("1e+20"), and -0.0 keeps its sign.
fn py_float_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        };
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Python exponent form: mantissa, 'e', sign, two-digit-minimum exponent.
        let s = format!("{:e}", f);
        if let Some((m, e)) = s.split_once('e') {
            let (sign, digits) = match e.strip_prefix('-') {
                Some(d) => ("-", d.to_string()),
                None => ("+", e.to_string()),
            };
            let digits = if digits.len() < 2 {
                format!("0{}", digits)
            } else {
                digits
            };
            return format!("{}e{}{}", m, sign, digits);
        }
        s
    } else if f.fract() == 0.0 {
        // repr keeps the trailing ".0" that Rust's `{}` would drop.
        format!("{:.1}", f)
    } else {
        format!("{}", f)
    }
}

/// Python `str(value)` for the float-or-Decimal union handed to the hooks.
fn py_num_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => py_float_str(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

pub struct LangFrBe {
    cards: Cards,
    maxval: BigInt,
    /// `Num2Word_FR.ords`, as a Vec to preserve Python's dict insertion order
    /// (`cinq` then `neuf`). The `to_ordinal` loop `break`s on first match.
    ords: Vec<(&'static str, &'static str)>,
    exclude_title: Vec<String>,
    /// Built once here, never per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangFrBe {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFrBe {
    pub fn new() -> Self {
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
        let mut high = vec!["cent".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords: long scale, GIGA_SUFFIX="illiard",
        // MEGA_SUFFIX="illion". cap = 3 + 6*len(high); zip(high, range(cap, 3, -6)).
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // mirrors range(cap, 3, -6) exhausting / zip truncating
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illiard", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illion", word),
            );
            n -= 6;
        }

        // --- Num2Word_FR_BE.setup: overrides Num2Word_FR's mid_numwords ---
        // 90 "nonante" / 70 "septante" are Belgian; 80 is "quatre-vingt"
        // WITHOUT the `s` that metropolitan FR carries.
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mille"),
                (100, "cent"),
                (90, "nonante"),
                (80, "quatre-vingt"),
                (70, "septante"),
                (60, "soixante"),
                (50, "cinquante"),
                (40, "quarante"),
                (30, "trente"),
            ],
        );

        // --- Num2Word_FR.setup: low_numwords (21 entries → values 20..0) ---
        set_low_numwords(
            &mut cards,
            &[
                "vingt", "dix-neuf", "dix-huit", "dix-sept", "seize", "quinze", "quatorze",
                "treize", "douze", "onze", "dix", "neuf", "huit", "sept", "six", "cinq", "quatre",
                "trois", "deux", "un", "zéro",
            ],
        );

        // MAXVAL = 1000 * first-inserted card key = 1000 * 10**603 = 10**606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangFrBe {
            cards,
            maxval,
            ords: vec![("cinq", "cinquième"), ("neuf", "neuvième")],
            exclude_title: vec!["et".into(), "virgule".into(), "moins".into()],
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. For integral input only the negative
    /// check can fire; Python raises TypeError (not ValueError) there.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.verify_ordinal` for float/Decimal input, checks in
    /// Python's order:
    ///
    ///   1. `not value == int(value)` -> TypeError(errmsg_floatord)
    ///   2. `not abs(value) == value` -> TypeError(errmsg_negord)
    ///
    /// Both comparisons are *numeric*, so -0.0 passes both (`int(-0.0) ==
    /// -0.0` and `abs(-0.0) == -0.0` in IEEE): `to_ordinal(-0.0)` is
    /// "zéroième", not an error. A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
    /// Returns the integral value for the integer-path continuation.
    /// (NaN/±inf would make `int(value)` raise before either check, but the
    /// dispatcher keeps those on the Python side; `as_whole_int` maps them to
    /// the float-message arm, which is unreachable for them in practice.)
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_num_str(value)
                    )))
                } else {
                    Ok(i)
                }
            }
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_num_str(value)
            ))),
        }
    }
}

impl Lang for LangFrBe {
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
        " et"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "moins "
    }
    fn pointword(&self) -> &str {
        "virgule"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_FR_BE.merge`.
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
                return (ntext, nnum.clone());
            }
            // NB: no `else` fall-through in Python either — when cnum == 1 and
            // nnum >= 1_000_000 the `else` block is skipped entirely and we
            // drop straight to the comparison ladder. Hence "un million".
        } else {
            // Drop the trailing 's' of 'cents'/'vingts' before a following
            // word: "sept cent vingt-neuf", not "sept cents vingt-neuf".
            // Python's `%` floors toward -inf; `mod_floor` matches it. (cnum
            // is non-negative here in practice, but (cnum-80) can be, e.g.,
            // -76 for cnum=4 → 24 under Python semantics, 24 under mod_floor.)
            let a = (cnum - BigInt::from(80)).mod_floor(&hundred).is_zero();
            let b = (cnum % &hundred).is_zero() && cnum < &thousand;
            if (a || b) && nnum < &million && ctext.chars().last() == Some('s') {
                ctext.pop();
            }
            if cnum < &thousand
                && nnum != &thousand
                && last_char_is_not_s(&ntext)
                && (nnum % &hundred).is_zero()
            {
                ntext.push('s');
            }
        }

        if nnum < cnum && cnum < &hundred {
            if (nnum % BigInt::from(10)).is_one() {
                // Belgian: " et " with spaces, and NO `cnum != 80` guard —
                // 81 → "quatre-vingt et un" (metropolitan FR: "quatre-vingt-un").
                return (format!("{} et {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{}-{}", ctext, ntext), cnum + nnum);
        }
        if nnum > cnum {
            return (format!("{} {}", ctext, ntext), cnum * nnum);
        }
        (format!("{} {}", ctext, ntext), cnum + nnum)
    }

    /// Port of `Num2Word_FR.to_ordinal`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("premier".to_string());
        }
        let word = self.to_cardinal(value)?;

        // Big-unit ordinals: drop a leading "un ", strip trailing 's', append
        // "ième".
        for unit in BIG_UNITS.iter() {
            if word == format!("un {}", unit) || word.ends_with(&format!(" {}s", unit)) {
                // Python's rstrip("s") removes ALL trailing 's', not just one.
                let stripped = word.trim_end_matches('s');
                let stripped = stripped.strip_prefix("un ").unwrap_or(stripped);
                // The plural "s" of "millions" is an agreement marker and
                // drops in the ordinal, as for "cents"/"vingts" (#65, #165):
                // "deux millionième", not "deux millionièmes".
                return Ok(format!("{}ième", stripped));
            }
            if word == *unit {
                return Ok(format!("{}ième", unit));
            }
        }

        // Python's `for ... else`: the else arm runs only if no `break` fired.
        let mut word = word;
        let mut matched = false;
        for (src, repl) in self.ords.iter() {
            if word.ends_with(src) {
                // `src` is ASCII ("cinq"/"neuf") and `word` ends with it, so
                // the split point is guaranteed to be a char boundary.
                let keep = word.len() - src.len();
                word = format!("{}{}", &word[..keep], repl);
                matched = true;
                break;
            }
        }
        if !matched {
            // Python: word[-1] == "e" → word[:-1]. String::pop is char-aware,
            // matching Python's character-wise slicing.
            if word.ends_with('e') {
                word.pop();
            }
            // "deux cents" → "deux cent" + "ième". The `endswith("ents")`
            // disjunct is dead code in Python (see module docs); kept verbatim.
            if word.ends_with("ts") || word.ends_with("ents") {
                word.pop();
            }
            word.push_str("ième");
        }
        Ok(word)
    }

    /// Port of `Num2Word_FR.to_ordinal_num`: "1er", everything else "<n>me".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut out = value.to_string();
        out.push_str(if value.is_one() { "er" } else { "me" });
        Ok(out)
    }

    /// Port of `Num2Word_FR.to_year`: `return self.to_cardinal(int(val))` —
    /// no century splitting, no BC/AD suffix. Negative years just inherit
    /// to_cardinal's "moins " prefix (year -500 → "moins cinq cents").
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float/negative checks and `to_year`'s
    // `int()` truncation become reachable here — unlike on the BigInt hooks
    // above, where they are dead code. `to_cardinal` needs no override: the
    // FR_BE → FR → EUR → Base chain inherits base's `assert int(value) ==
    // value` routing, which is exactly the trait default (whole -> int path).

    /// `Num2Word_FR.to_ordinal(float/Decimal)`: `verify_ordinal`, then the
    /// integer path. Whole values ordinalise (5.0 -> "cinquième", 1.0 ->
    /// "premier" via the `value == 1` special case, 21.0 -> the Belgian
    /// "vingt et unième", Decimal("1E+2") -> "centième", 1e+16 -> "dix
    /// billiardième"); fractional or negative values raise
    /// TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `Num2Word_FR.to_ordinal_num(float/Decimal)`:
    /// `str(value) + ("er" if value == 1 else "me")`. The numeric half is
    /// Python's `str(value)` (`repr_str`), so 1.0 yields "1.0er",
    /// Decimal("5.00") "5.00me", -0.0 "-0.0me" and Decimal("1E+20")
    /// "1E+20me". No overflow is possible — Python never words the value.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        Ok(format!(
            "{}{}",
            repr_str,
            if i.is_one() { "er" } else { "me" }
        ))
    }

    /// `Num2Word_FR.to_year(float/Decimal)`: `self.to_cardinal(int(val))` —
    /// `int()` truncates toward zero, so 2.5 -> "deux", -1.5 -> "moins un",
    /// 0.5 -> "zéro". No non-integer guard (contrast EN's TypeError):
    /// fractional values are silently truncated, floats and Decimals alike.
    /// NaN/±inf floats reproduce `int()`'s ValueError/OverflowError, though
    /// the dispatcher keeps those on the Python side.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = match value {
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
                // Exact: every whole f64 is exactly representable.
                BigInt::from_f64(f.trunc()).ok_or_else(|| {
                    N2WError::Value("cannot convert float NaN to integer".into())
                })?
            }
            // int(Decimal) truncates toward zero.
            FloatValue::Decimal { value: d, .. } => d.with_scale(0).as_bigint_and_exponent().0,
        };
        self.to_cardinal(&i)
    }

    // ---- fractions --------------------------------------------------------

    /// Port of `Num2Word_FR.to_fraction` (issue #584), inherited unchanged by
    /// `Num2Word_FR_BE`: idiomatic demi/tiers/quart for denominators 2/3/4 —
    /// "tiers" is invariant in the plural ("1000000/3" -> "un million
    /// tiers"), demi/quart take -s. Every other denominator is
    /// ordinal-as-noun, appending "s" only when the numerator isn't 1 *and*
    /// the ordinal doesn't already end in 's' (a guard only;
    /// `to_ordinal` no longer pluralises big units, #165). `d == 1`
    /// (exactly 1, not -1) or `n == 0` short-circuits to the signed cardinal
    /// before any sign normalisation, and a zero denominator raises
    /// ZeroDivisionError first ("0/0" raises).
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        // is_negative = (numerator < 0) ^ (denominator < 0)
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();
        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "demi" } else { "demis" }).to_string()
        } else if abs_d == BigInt::from(3) {
            // 'tiers' is invariant in plural.
            "tiers".to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "quart" } else { "quarts" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() && !w.ends_with('s') {
                w.push('s');
            }
            w
        };
        let sign = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        Ok(format!("{}{} {}", sign, self.to_cardinal(&abs_n)?, den_word))
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_FR_BE"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    // `currency_precision` is deliberately NOT overridden. No class in the
    // FR_BE → FR → EUR → Base chain defines `CURRENCY_PRECISION`, so it
    // resolves to `Num2Word_Base.CURRENCY_PRECISION == {}` and every code
    // takes the `.get(code, 100)` default of 100 — including JPY. EN *does*
    // define 1000/1 precisions, but it **rebinds** the attribute in
    // `__init__` (`self.CURRENCY_PRECISION = {...}`, an instance attribute)
    // rather than mutating the shared class dict, so unlike CURRENCY_FORMS it
    // cannot leak here. Hence JPY divides by 100 and keeps its cents segment:
    // 12.34 JPY → "douze yens et trente-quatre sens", not "douze yens".
    // That also means `default_to_currency`'s `divisor == 1` pre-rounding
    // branch is unreachable for this language.

    /// `Num2Word_FR.pluralize`: `forms[0] if abs(n) <= 1 else forms[1]`.
    ///
    /// Overrides `Num2Word_EUR.pluralize` (`forms[0 if n == 1 else 1]`) so 0
    /// takes the singular as well: "zéro euro et un centime", not "zéro euros".
    /// Python indexes the tuple directly, so a one-form entry with `abs(n) > 1`
    /// raises IndexError; every entry in FR's table has two forms, so that is
    /// unreachable — mapped to `Index` rather than panicking in case it ever
    /// is not.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.abs() <= BigInt::one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_FR.to_currency`, which splits hard on `isinstance(val,
    /// int)` and hands only the float path to `super()`.
    ///
    /// The int branch is not a shortcut through Base — it diverges in two
    /// observable ways, both reproduced here:
    ///
    /// 1. **An unknown currency code raises (fixed, #219).** Python caught the
    ///    `KeyError` and returned a bare `to_cardinal(val)` — the number with
    ///    no currency word at all; the port raises NotImplementedError.
    ///    So `to_currency(100, "KWD")` == "cent" and `to_currency(-100, "KWD")`
    ///    == "moins cent", while the *float* path for the same code raises
    ///    NotImplementedError from Base. The corpus pins both halves: KWD/BHD/CHF
    ///    ints give "zéro"/"un"/"deux"/"cent"/"un million", their floats give
    ///    NotImplementedError.
    /// 2. **`adjective` is ignored.** FR's int branch never calls
    ///    `prefix_currency`, so `to_currency(2, "USD", adjective=True)` is
    ///    "deux dollars" — yet `to_currency(2.0, "USD", adjective=True)` goes to
    ///    Base and *is* prefixed: "deux US dollars et zéro cent". Same flag,
    ///    same code, opposite answers, decided purely by int vs float.
    ///
    /// The int branch also reimplements pluralization inline rather than calling
    /// `self.pluralize`, and uses `self.to_cardinal` where Base would use
    /// `_money_verbose`. Neither changes output here (`_money_verbose` is
    /// unoverridden, and the inline rule matches `pluralize` for every 2-form
    /// entry), but the inline `cr1[1] if len(cr1) > 1 else cr1[0]` is *softer*
    /// than `pluralize`'s bare `forms[1]`: on a hypothetical 1-form entry it
    /// would return the singular where `pluralize` raises IndexError.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        if let CurrencyValue::Int(v) = val {
            let cr1 = match self.currency_forms.get(currency) {
                Some(forms) => &forms.unit,
                // Python's `except KeyError: return self.to_cardinal(val)`
                // printed a bare number; raise like the float path (#219).
                None => return Err(crate::currency::unknown_currency(self, currency)),
            };

            let minus_str = if v.is_negative() {
                format!("{} ", self.negword().trim())
            } else {
                String::new()
            };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            // `cr1[0] if abs_val <= 1 else (cr1[1] if len(cr1) > 1 else cr1[0])`
            let currency_str = if abs_val <= BigInt::one() {
                cr1.first()
            } else {
                cr1.get(1).or_else(|| cr1.first())
            }
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // Floats/Decimals: `super(Num2Word_FR, self).to_currency(...)`.
        // `Num2Word_EUR` does not override `to_currency`, so super() resolves
        // straight to `Num2Word_Base.to_currency` — i.e. this default.
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }

    // `to_cheque` is NOT overridden: nothing in the FR_BE → FR → EUR chain
    // defines it, so `Num2Word_Base.to_cheque` runs and `default_to_cheque`
    // reproduces it given the right forms + lang_name. It always takes the
    // last (plural) unit form and always divides by 100 here, so JPY yields
    // "MILLE DEUX CENT TRENTE-QUATRE AND 56/100 YENS" and the codes missing
    // from FR's table raise NotImplementedError — unlike `to_currency`'s int
    // path, `to_cheque` has no bare-cardinal fallback.
}

/// Python: `ntext[-1] != "s"` — compares the last *character*, not byte.
fn last_char_is_not_s(s: &str) -> bool {
    s.chars().last() != Some('s')
}
