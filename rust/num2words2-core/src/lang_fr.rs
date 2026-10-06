//! Port of `lang_FR.py` (via `lang_EUR` → `Num2Word_Base`).
//!
//! Engine-style language: FR supplies `mid_numwords`/`low_numwords` + `merge`
//! and inherits `Num2Word_EUR.set_high_numwords`, letting the base engine's
//! `splitnum`/`clean` drive `to_cardinal`.
//!
//! Unlike EN (which overrides `set_high_numwords` to the short scale), FR uses
//! EUR's **long scale**: each Latin stem yields a `-illiard` at 10^n and an
//! `-illion` at 10^(n-3), stepping by 6. So 10^9 = "milliard", 10^12 =
//! "billion", 10^15 = "billiard", 10^18 = "trillion", … up to 10^603 =
//! "centilliard". MAXVAL is therefore 1000 * 10^603 = 10^606.

use crate::base::{clean, set_low_numwords, set_mid_numwords, splitnum, Cards, Lang, N2WError, Node, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Duplicated locally rather than imported from `lang_en` to keep this file
/// self-contained (the registry in `lib.rs` is generated mechanically).
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

/// `Num2Word_FR._BIG_UNITS`, in source order — the order is load-bearing
/// because "trilliard" must be tested before "trillion", etc.
const BIG_UNITS: [&str; 6] = [
    "trilliard",
    "trillion",
    "billiard",
    "billion",
    "milliard",
    "million",
];

/// `Num2Word_FR.ords`. A slice (not a map) to preserve Python's dict
/// insertion order, which `to_ordinal` iterates.
const ORDS: [(&str, &str); 2] = [("cinq", "cinquième"), ("neuf", "neuvième")];

/// `Num2Word_FR.CURRENCY_FORMS`, transcribed from the class body verbatim.
///
/// FR declares its **own** `CURRENCY_FORMS`, so — unlike the 16 classes that
/// inherit the dict — it never sees the entries `Num2Word_EN.__init__` mutates
/// into `Num2Word_EUR.CURRENCY_FORMS`. Confirmed against the live interpreter:
/// FR's EUR is `("euro", "euros")` / `("centime", "centimes")` because that is
/// FR's own literal, not because English rewrote it. Nothing here is inherited.
///
/// Consequences worth naming, since they look like typos and are not:
///   * CNY's subunits are `("fen", "jiaos")` — two *different* units, so 0.01
///     CNY is "un fen" but 0.34 is "trente-quatre jiaos". Upstream data bug.
///   * JPY/KRW carry sen/jeon subunits and FR has no `CURRENCY_PRECISION`
///     override, so they are ordinary 100-subunit currencies here.
///   * KWD/BHD/CHF are absent, which is what drives them down the
///     KeyError paths in `to_currency` / `to_cheque` (see those methods).
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
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuans"], &["fen", "jiaos"]));
    m.insert("JPY", CurrencyForms::new(&["yen", "yens"], &["sen", "sens"]));
    m.insert("INR", CurrencyForms::new(&["roupie", "roupies"], &["paisa", "paisas"]));
    m.insert("RUB", CurrencyForms::new(&["rouble", "roubles"], &["kopeck", "kopecks"]));
    m.insert("KRW", CurrencyForms::new(&["won", "wons"], &["jeon", "jeons"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &["centavo", "centavos"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — FR declares none
/// and `Num2Word_EN.__init__` never touches this dict (it only mutates
/// `CURRENCY_FORMS`), so the class-body literal is what runs. Verified against
/// the live interpreter.
///
/// Half of these codes (BYN, EEK, HUF, ISK, NOK, RON, SAR, UZS) have no entry
/// in FR's `CURRENCY_FORMS`, so they are unreachable: the forms lookup decides
/// the outcome long before an adjective could be prefixed. Kept anyway — the
/// dict is ported data, not a curated list.
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

pub struct LangFr {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangFr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFr {
    pub fn new() -> Self {
        // Num2Word_EUR.setup
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

        // Num2Word_EUR.set_high_numwords — long scale:
        //   cap = 3 + 6 * len(high)                        # 603
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**n]     = word + GIGA_SUFFIX      # "illiard"
        //       cards[10**(n-3)] = word + MEGA_SUFFIX      # "illion"
        // `high` has 100 entries and range(603, 3, -6) has 100 values, so the
        // zip consumes both exactly.
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illiard", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illion", word),
            );
            n -= 6;
        }

        // Note: no cards for 70 or 90 — French builds those by merging
        // (soixante-dix, quatre-vingt-dix).
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mille"),
                (100, "cent"),
                (80, "quatre-vingts"),
                (60, "soixante"),
                (50, "cinquante"),
                (40, "quarante"),
                (30, "trente"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "vingt", "dix-neuf", "dix-huit", "dix-sept", "seize", "quinze", "quatorze",
                "treize", "douze", "onze", "dix", "neuf", "huit", "sept", "six", "cinq", "quatre",
                "trois", "deux", "un", "zéro",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]  ->  1000 * 10**603
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangFr {
            cards,
            maxval,
            exclude_title: vec!["et".into(), "virgule".into(), "moins".into()],
            // Built once here, never per call — `to_currency` only reads them.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check is unreachable for
    /// BigInt input, so only the negative check survives.
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

impl Lang for LangFr {
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

    /// `Num2Word_Base.to_cardinal` for integral input.
    ///
    /// Reimplemented rather than delegating to `base::default_to_cardinal`
    /// solely because FR overrides `errmsg_toobig` with a French message.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = format!("{} ", self.negword().trim());
        }

        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "Nombre trop grand pour être converti en mots (abs({}) > {}).",
                v,
                self.maxval()
            )));
        }

        let tree = splitnum(self, &v).ok_or_else(|| {
            N2WError::Overflow(format!(
                "Nombre trop grand pour être converti en mots (abs({}) > {}).",
                v,
                self.maxval()
            ))
        })?;
        let words = match clean(self, tree) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        Ok(self.title(&format!("{}{}", out, words)))
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext_in, cnum) = l;
        let (ntext_in, nnum) = r;
        let mut ctext = ctext_in.to_string();
        let mut ntext = ntext_in.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            // NB: when nnum >= 1_000_000 Python falls through the `cnum == 1`
            // branch without returning *and* without running the `else` body,
            // so ctext/ntext stay untouched. Hence "un million" (no -s).
        } else {
            // if ((not (cnum - 80) % 100 or (not cnum % 100 and cnum < 1000))
            //     and nnum < 1000000 and ctext[-1] == "s"):
            //     ctext = ctext[:-1]
            //
            // Python's % on a negative left operand floors (e.g. (20-80) % 100
            // == 40), so mod_floor is used rather than Rust's remainder. Only
            // the ==0 test matters here, where the two agree, but keep it exact.
            let c_minus_80 = cnum.clone() - BigInt::from(80);
            let cond = (c_minus_80.mod_floor(&hundred).is_zero()
                || (cnum.mod_floor(&hundred).is_zero() && cnum < &thousand))
                && nnum < &million
                && ctext.chars().last() == Some('s');
            if cond {
                ctext.pop();
            }

            // if cnum < 1000 and nnum != 1000 and ntext[-1] != "s" and not nnum % 100:
            //     ntext += "s"
            // The `nnum != 1000` guard is what keeps "deux mille" singular.
            if cnum < &thousand
                && nnum != &thousand
                && ntext.chars().last() != Some('s')
                && nnum.mod_floor(&hundred).is_zero()
            {
                ntext.push('s');
            }
        }

        // if nnum < cnum < 100  (Python chained comparison)
        if nnum < cnum && cnum < &hundred {
            if nnum.mod_floor(&BigInt::from(10)).is_one() && cnum != &BigInt::from(80) {
                return (format!("{}-et-{}", ctext, ntext), cnum + nnum);
            }
            return (format!("{}-{}", ctext, ntext), cnum + nnum);
        }
        if nnum > cnum {
            return (format!("{} {}", ctext, ntext), cnum * nnum);
        }
        (format!("{} {}", ctext, ntext), cnum + nnum)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("premier".to_string());
        }
        let word = self.to_cardinal(value)?;

        // Big-unit ordinals: drop a leading "un " and the trailing plural "s".
        for &unit in BIG_UNITS.iter() {
            if word == format!("un {}", unit) || word.ends_with(&format!(" {}s", unit)) {
                // Python's str.rstrip("s") strips *all* trailing 's'.
                let rstripped = word.trim_end_matches('s');
                let stripped = match rstripped.strip_prefix("un ") {
                    Some(rest) => rest,
                    None => rstripped,
                };
                // The plural "s" of "millions" is an agreement marker and
                // drops in the ordinal, as for "cents"/"vingts" (#65, #165):
                // "deux millionième", not "deux millionièmes".
                return Ok(format!("{}ième", stripped));
            }
            if word == unit {
                return Ok(format!("{}ième", unit));
            }
        }

        let mut word = word;
        let mut matched = false;
        for &(src, repl) in ORDS.iter() {
            if word.ends_with(src) {
                // `src` is ASCII and is a suffix of `word`, so this byte index
                // is a valid char boundary.
                let head = word.len() - src.len();
                word.truncate(head);
                word.push_str(repl);
                matched = true;
                break;
            }
        }
        if !matched {
            // Python's for/else: only runs when no `src` matched.
            if word.chars().last() == Some('e') {
                word.pop();
            }
            // Drop the trailing 's' of "cents" / "vingts" before -ième.
            // (The `endswith("ents")` arm is redundant — anything ending in
            // "ents" already ends in "ts" — but it is kept for fidelity.)
            if word.ends_with("ts") || word.ends_with("ents") {
                word.pop();
            }
            word.push_str("ième");
        }
        Ok(word)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut out = value.to_string();
        out.push_str(if value.is_one() { "er" } else { "me" });
        Ok(out)
    }

    /// FR's `to_year` ignores suffix/longval entirely and just delegates.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float/negative checks and `to_year`'s
    // `int()` truncation become reachable here — unlike on the BigInt hooks
    // above, where they are dead code. `to_cardinal` needs no override: FR
    // inherits base's `assert int(value) == value` routing, which is exactly
    // the trait default (whole -> int path).

    /// `Num2Word_FR.to_ordinal(float/Decimal)`: `verify_ordinal`, then the
    /// integer path. Whole values ordinalise (5.0 -> "cinquième", 1.0 ->
    /// "premier" via the `value == 1` special case, Decimal("1E+2") ->
    /// "centième", 1e+16 -> "dix billiardième");
    /// fractional or negative values raise TypeError.
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

    /// Port of `Num2Word_FR.to_fraction` (issue #584): idiomatic
    /// demi/tiers/quart for denominators 2/3/4 — "tiers" is invariant in the
    /// plural ("1000000/3" -> "un million tiers"), demi/quart take -s. Every
    /// other denominator is ordinal-as-noun, appending "s" only when the
    /// numerator isn't 1 *and* the ordinal doesn't already end in 's' (a
    /// guard only; `to_ordinal` no longer pluralises big units, #165).
    /// `d == 1` (exactly 1, not -1) or `n == 0`
    /// short-circuits to the signed cardinal before any sign normalisation,
    /// and a zero denominator raises ZeroDivisionError first ("0/0" raises).
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

    // ---- currency -------------------------------------------------------
    //
    // FR overrides `CURRENCY_FORMS`, `pluralize` and `to_currency`. It does
    // *not* override `_money_verbose`, `_cents_verbose`, `_cents_terse` or
    // `to_cheque`, so those keep the trait defaults that mirror
    // `Num2Word_Base`.
    //
    // `currency_precision` is deliberately NOT overridden: FR declares no
    // `CURRENCY_PRECISION`, and `Num2Word_EN.__init__` *rebinds* rather than
    // mutates that attribute, so its 3-decimal table never leaks up to
    // `Num2Word_Base`. FR therefore sees `{}` and every currency divides by
    // 100 — including JPY/KRW. That is why the corpus wants
    // "douze yens et trente-quatre sens" instead of a rounded, subunit-less
    // yen, and why `default_to_currency`'s `divisor == 1` branch is
    // unreachable from FR.

    fn lang_name(&self) -> &str {
        "Num2Word_FR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_FR.pluralize`: `forms[0] if abs(n) <= 1 else forms[1]`.
    ///
    /// Overrides `Num2Word_EUR.pluralize` (`forms[0 if n == 1 else 1]`): in
    /// French **zero takes the singular**, so 1.0 EUR is
    /// "un euro et zéro centime", not "…zéro centimes". The `abs()` also means
    /// -1 is singular, though the currency path always passes a magnitude.
    ///
    /// Python indexes `forms[1]` directly, so a one-form entry with abs(n) > 1
    /// would raise IndexError. Every FR entry has two forms, making that
    /// unreachable — it is mapped to `Index` rather than panicking so the
    /// exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.abs() <= BigInt::one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_FR.to_currency`.
    ///
    /// FR intercepts the integer case with its own body and delegates
    /// everything else to `Num2Word_Base.to_currency` via `super()`. The int
    /// branch is not a reimplementation of Base's — it differs in three ways
    /// that are all observable, and all reproduced here:
    ///
    /// 1. **An unknown currency raises (fixed, #219).** Python caught the
    ///    `KeyError` and returned a bare `to_cardinal(val)` — `to_currency(0,
    ///    "KWD")` was "zéro", no unit — while the float path raised. Both
    ///    raise `NotImplementedError` now.
    /// 2. **`adjective` is ignored**, since FR's branch never consults
    ///    `CURRENCY_ADJECTIVES`. `to_currency(2, "USD", adjective=True)` is
    ///    "deux dollars", not "deux US dollars" — the adjective only survives
    ///    on the float path.
    /// 3. **The plural rule is inlined, not `self.pluralize`.** It agrees with
    ///    FR's `pluralize` on the 0/1-singular rule but degrades safely
    ///    (`cr1[1] if len(cr1) > 1 else cr1[0]`) where `pluralize` would
    ///    IndexError.
    ///
    /// Note the fallback returns `to_cardinal(val)` on the **signed** value,
    /// not `abs_val`, so a negative int keeps its "moins".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Python: `if isinstance(val, int)` — true ints only. A float that
        // happens to be whole (1.0) is *not* one and still renders cents.
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
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

            let currency_str = if abs_val <= BigInt::one() {
                forms.unit.first()
            } else {
                forms.unit.get(1).or_else(|| forms.unit.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // Floats/Decimals: super(Num2Word_FR, self).to_currency(...).
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
