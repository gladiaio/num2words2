//! Port of `lang_AF.py` (Afrikaans) via its `Num2Word_EUR` → `Num2Word_Base`
//! ancestry.
//!
//! Engine-style: AF supplies `high_numwords`/`mid_numwords`/`low_numwords` +
//! `merge` and lets `Num2Word_Base.to_cardinal` drive `splitnum`/`clean`.
//!
//! # Inherited behaviour chased to its definition
//!
//! * `set_high_numwords` comes from `Num2Word_EUR`: long scale, step -6, with
//!   *two* cards per stem — `GIGA_SUFFIX` at `10**n` and `MEGA_SUFFIX` at
//!   `10**(n-3)`. AF overrides the suffixes to "iljard"/"iljoen", giving
//!   miljoen 10^6 / miljard 10^9 / biljoen 10^12 / biljard 10^15 / ...
//! * `gen_high_numwords` also comes from `Num2Word_EUR` and is reproduced
//!   verbatim below (see `gen_high_numwords`), elision table included.
//! * `AF.setup()` calls `super().setup()`, which assigns the *English* EUR
//!   `high_numwords`; AF then immediately reassigns `high_numwords` to its own
//!   Afrikaans list, so the EUR assignment is dead. Only AF's list is modelled.
//! * `is_title` stays false (`Num2Word_Base.__init__`), so `title()` is a
//!   no-op; `exclude_title` is `[]`. Both left at the trait defaults.
//!
//! # Faithfully reproduced Python oddities
//!
//! Per the porting contract these are preserved verbatim, not fixed:
//!
//! * `to_ordinal(0)` → "nullde" (double "l"). `ords["nul"] = "nulld"` and
//!   `to_ordinal` appends "e", so "nul" → "nulld" → "nullde". Idiomatic
//!   Afrikaans is "nulde".
//! * `to_year(1801)` → "agttien een", `to_year(1901)` → "negentien een".
//!   The tens-less years drop the "nul": normal usage is "agttien nul een".
//! * `to_year(10000)` → "een honderd honderd" (century=100, year_part=0).
//!   Nonsense, but that is what the `val >= 2000` branch computes.
//! * `to_ordinal_num` special-cases only 2..=8 for the "de" suffix, so
//!   22 → "22ste" while 2 → "2de".
//! * The 10^600/10^603 stem is "send" (AF's `high_numwords[0]`), yielding
//!   "sendiljoen"/"sendiljard" rather than a "sentiljoen" spelling.
//! * `merge`'s `nnum == 100 or nnum == 1000` space rule is redundant with the
//!   `nnum >= 10**6` arm for every card AF actually has, but is kept as-is.
//!
//! # Verification
//!
//! All 305 integer in-scope corpus rows (cardinal/ordinal/ordinal_num/year)
//! reproduce exactly, cross-checked by re-implementing this exact logic and
//! diffing against `Num2Word_AF` over ~73k additional calls spanning 0..2200,
//! powers of ten to 10^606, random values to 10^60, negatives, the overflow
//! boundary, and a -60..12000 year sweep. Zero mismatches.
//!
//! Currency: all 117 `currency:*`/`cheque:*` corpus rows reproduce exactly,
//! plus 2268 differential cases diffed against the live `Num2Word_AF` covering
//! what the corpus does not reach — ZAR (AF's *own* default currency, which the
//! corpus never exercises), `adjective=True`, `cents=False`, fractional cents,
//! 1e21, half-up rounding ties (0.005/2.675), and unknown/lowercased codes.
//! Zero mismatches.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// The elision table is Latin/English-oriented while AF's units and tens are
/// Afrikaans ("kwin" not "quint", "seks" not "sex", "tre" not "tres", "okto"
/// not "octo"), so most rules are dead here. Exactly three still fire:
/// `novemn`, `novemo` and `unno`. They are all kept anyway — reproducing the
/// inherited method, not an optimised equivalent.
///
/// Note `"novemnonagint"` → `"novenonagint"`: the rule's trailing `n` consumes
/// the first `n` of "nonagint", so the result is *not* "novennonagint".
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — tens is the outer loop.
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

/// `Num2Word_AF.ords`, in **insertion order**.
///
/// This is a `&[(…)]` slice and not a map on purpose: `to_ordinal` scans the
/// dict and `break`s on the first `endswith` hit, so Python's dict insertion
/// order is load-bearing. "nul" must be tested before "een"; the single-word
/// keys must all be tested before the compound endings "ig"/"erd"/"end"/
/// "joen"/"rd". Re-ordering silently changes output (e.g. "dertien" would hit
/// "drie"-family keys or "tien" depending on order).
const ORDS: &[(&str, &str)] = &[
    ("nul", "nulld"),
    ("een", "eerst"),
    ("twee", "tweed"),
    ("drie", "derd"),
    ("vier", "vierd"),
    ("vyf", "vyfd"),
    ("ses", "sesd"),
    ("sewe", "sewend"),
    ("agt", "agst"),
    ("nege", "negend"),
    ("tien", "tiend"),
    ("elf", "elfd"),
    ("twaalf", "twaalfd"),
    // Compound endings.
    ("ig", "igst"),
    ("erd", "erdst"),
    ("end", "endst"),
    ("joen", "joenst"),
    ("rd", "rdst"),
];

/// `Num2Word_AF.CURRENCY_FORMS`, verbatim from the class body.
///
/// AF declares its **own** `CURRENCY_FORMS` in the class body, which shadows
/// `Num2Word_EUR.CURRENCY_FORMS` entirely. That matters: `Num2Word_EN.__init__`
/// mutates the *EUR* dict in place (`self.CURRENCY_FORMS["EUR"] = ...`) and
/// `__init__.py` instantiates `Num2Word_EN()` at import time, so every class
/// that merely inherits the dict silently picks up English forms plus EN's ~24
/// extra codes. AF does not: `Num2Word_AF.CURRENCY_FORMS is
/// Num2Word_EUR.CURRENCY_FORMS` is `False` on the live interpreter, so the
/// table is exactly these five codes and nothing else.
///
/// Every other code — JPY, KWD, BHD, INR, CHF, and the ~20 more EN would have
/// contributed — therefore raises NotImplementedError. The corpus pins that:
/// AF has no 3-decimal (KWD/BHD) or 0-decimal (JPY) currency to render.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("ZAR", CurrencyForms::new(&["rand", "rand"], &["sent", "sent"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["sent", "sent"]));
    m.insert("GBP", CurrencyForms::new(&["pond", "pond"], &["penny", "pence"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollar"], &["sent", "sent"]));
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["jiao", "fen"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged.
///
/// The mirror image of the forms table above: AF defines no
/// `CURRENCY_ADJECTIVES`, so it reads EUR's shared dict — and unlike
/// `CURRENCY_FORMS`, EN never writes to that one, so no mutation leaks in and
/// the source literal *is* what runs (`Num2Word_AF.CURRENCY_ADJECTIVES is
/// Num2Word_EUR.CURRENCY_ADJECTIVES` is `True`).
///
/// Only USD overlaps AF's five currency codes, so "US" is the sole reachable
/// entry — `adjective=True` on any other code finds nothing and leaves the
/// forms bare. The 15 dead entries are kept because the ported artefact is the
/// inherited dict, not the reachable subset of it.
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

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the fractional
/// digit count of the shortest round-trip repr. Mirrors the private helper in
/// `floatpath`.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Best-effort `str(number)` for the TypeError messages (the corpus compares
/// exception types only, so the float arm's repr need not be byte-exact).
fn py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

pub struct LangAf {
    cards: Cards,
    maxval: BigInt,
    hundred: BigInt,
    thousand: BigInt,
    mega: BigInt,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangAf {
    fn default() -> Self {
        Self::new()
    }
}

impl LangAf {
    pub fn new() -> Self {
        // AF's own Afrikaans stems (these replace the EUR ones set by super().setup()).
        let lows = ["non", "okt", "sept", "sext", "kwint", "kwadr", "tr", "b", "m"];
        let units = [
            "", "un", "duo", "tre", "kwattuor", "kwin", "seks", "sept", "okto", "novem",
        ];
        let tens = [
            "des",
            "vigint",
            "trigint",
            "kwadragint",
            "kwinquagint",
            "seksagint",
            "septuagint",
            "oktogint",
            "nonagint",
        ];
        let mut high = vec!["send".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));
        // 1 + (9 tens * 10 units) + 9 lows = 100 stems.

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords: cap = 3 + 6*len(high) = 603;
        // zip(high, range(603, 3, -6)) pairs all 100 stems (range has exactly
        // 100 elements), GIGA at 10**n and MEGA at 10**(n-3). The `n <= 3`
        // guard reproduces the range's stop bound, and `zip` truncation is
        // moot here since both sides are length 100.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(
                BigInt::from(10u8).pow(n as u32),
                format!("{}{}", word, "iljard"), // AF GIGA_SUFFIX
            );
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}{}", word, "iljoen"), // AF MEGA_SUFFIX
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "duisend"),
                (100, "honderd"),
                (90, "negentig"),
                (80, "tagtig"),
                (70, "sewentig"),
                (60, "sestig"),
                (50, "vyftig"),
                (40, "veertig"),
                (30, "dertig"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "twintig",
                "negentien",
                "agttien",
                "sewentien",
                "sestien",
                "vyftien",
                "veertien",
                "dertien",
                "twaalf",
                "elf",
                "tien",
                "nege",
                "agt",
                "sewe",
                "ses",
                "vyf",
                "vier",
                "drie",
                "twee",
                "een",
                "nul",
            ],
        );

        // MAXVAL = 1000 * first inserted card. Python's OrderedDict insertion
        // order here is strictly descending (high 10^603 down, then mid, then
        // low), so keys()[0] == the highest card == 10^603 → MAXVAL = 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangAf {
            cards,
            maxval,
            hundred: BigInt::from(100),
            thousand: BigInt::from(1000),
            mega: BigInt::from(10u8).pow(6),
            // Built once here, never per call: the currency path only reads
            // these, and rebuilding them on each call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. Integer input can only trip the
    /// negative check; both arms raise TypeError in Python.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == Sign::Minus {
            return Err(N2WError::Type(format!(
                "Kan nie die negatiewe getal {} as 'n ordinale getal behandel nie.",
                value
            )));
        }
        Ok(())
    }

    /// `verify_ordinal` for a float/Decimal: the float check fires first,
    /// then the negative one — both TypeError, with AF's Afrikaans wording.
    /// `-0.0` passes both. Returns the whole value on success.
    fn verify_ordinal_num(&self, value: &FloatValue) -> Result<BigInt> {
        let whole = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Kan nie die desimale getal {} as 'n ordinale getal behandel nie.",
                    py_num_str(value)
                )))
            }
        };
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Kan nie die negatiewe getal {} as 'n ordinale getal behandel nie.",
                py_num_str(value)
            )));
        }
        Ok(whole)
    }

    /// Python's `self.to_cardinal(x)` for an f64 — Base semantics: whole →
    /// integer path, fractional → the base float grammar ("komma" + digits).
    fn cardinal_num_f64(&self, x: f64) -> Result<String> {
        if x.fract() == 0.0 {
            if let Some(i) = BigInt::from_f64(x) {
                return self.to_cardinal(&i);
            }
        }
        default_to_cardinal_float(
            self,
            &FloatValue::Float {
                value: x,
                precision: float_repr_precision(x),
            },
            None,
        )
    }

    /// Python's `self.to_cardinal(x)` for a Decimal.
    fn cardinal_num_dec(&self, x: &BigDecimal) -> Result<String> {
        if x.is_integer() {
            return self.to_cardinal(&x.with_scale(0).as_bigint_and_exponent().0);
        }
        let precision = x.as_bigint_and_exponent().1.max(0) as u32;
        default_to_cardinal_float(
            self,
            &FloatValue::Decimal {
                value: x.clone(),
                precision,
            },
            None,
        )
    }

    /// Shared tail of `to_year`'s two century-splitting branches.
    fn year_century_split(&self, val: &BigInt) -> Result<String> {
        let century = val.div_floor(&self.hundred);
        let year_part = val.mod_floor(&self.hundred);
        if year_part.is_zero() {
            Ok(format!("{} honderd", self.to_cardinal(&century)?))
        } else {
            Ok(format!(
                "{} {}",
                self.to_cardinal(&century)?,
                self.to_cardinal(&year_part)?
            ))
        }
    }
}

impl Lang for LangAf {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ZAR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " en"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "komma"
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        if cnum.is_one() {
            if nnum == &self.hundred || nnum == &self.thousand {
                return (format!("een {}", ntext), nnum.clone());
            } else if nnum < &self.mega {
                // Python `return next` — the right tuple, unmodified.
                return (ntext, nnum.clone());
            }
            ctext = "een".to_string();
        }

        let val;
        if nnum > cnum {
            if nnum >= &self.mega {
                ctext.push(' ');
            } else if nnum == &self.hundred || nnum == &self.thousand {
                ctext.push(' ');
            }
            val = cnum * nnum;
        } else {
            let ten = BigInt::from(10);
            // Python chained comparison: nnum < 10 and 10 < cnum and cnum < 100.
            if nnum < &ten && &ten < cnum && cnum < &self.hundred {
                if nnum.is_one() {
                    ntext = "een".to_string();
                }
                // Afrikaans compound formation: vier-en-dertig.
                ntext = format!("{}-en-{}", ntext, ctext);
                ctext = String::new();
            } else if cnum >= &self.mega {
                ctext.push(' ');
            } else if cnum >= &self.hundred {
                ctext.push(' ');
            }
            val = cnum + nnum;
        }

        (format!("{}{}", ctext, ntext), val)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut outword = self.to_cardinal(value)?;
        // "een miljoen" -> "miljoenste", not "een miljoenste" (which reads
        // as the fraction 1/10**6); likewise "honderdste", "duisendste",
        // "miljardste" — as nl does (#252, #259).
        if let Some(rest) = outword.strip_prefix("een ") {
            if !rest.contains(' ') {
                outword = rest.to_string();
            }
        }

        // First matching suffix wins, then `break` — see ORDS on why order matters.
        for (key, rep) in ORDS.iter() {
            if outword.ends_with(key) {
                // All AF numwords are ASCII, and a matched suffix always ends
                // on a char boundary regardless, so this slice is safe.
                outword = format!("{}{}", &outword[..outword.len() - key.len()], rep);
                break;
            }
        }

        Ok(format!("{}e", outword))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        // Python: `if value in [2, 3, 4, 5, 6, 7, 8]` — contiguous, so a range
        // test is equivalent for integer input.
        if value >= &BigInt::from(2) && value <= &BigInt::from(8) {
            Ok(format!("{}de", value))
        } else {
            Ok(format!("{}ste", value))
        }
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        let two_thousand = BigInt::from(2000);

        if value >= &two_thousand {
            if value == &two_thousand {
                self.to_cardinal(value)
            } else if value < &BigInt::from(2010) {
                // 2001-2009: "twee duisend een", etc.
                Ok(format!(
                    "{} {}",
                    self.to_cardinal(&two_thousand)?,
                    self.to_cardinal(&(value - &two_thousand))?
                ))
            } else {
                // 2010+: "twintig tien", "twintig elf", etc.
                self.year_century_split(value)
            }
        } else if value < &BigInt::from(1000) {
            // Also the path every negative year takes, so the floor-division
            // below is never reached with a negative operand.
            self.to_cardinal(value)
        } else if value
            .div_floor(&self.hundred)
            .mod_floor(&BigInt::from(10))
            .is_zero()
        {
            // Python: `elif not (val // 100) % 10` — e.g. 1000..1099 stay cardinal.
            self.to_cardinal(value)
        } else {
            self.year_century_split(value)
        }
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for any
    /// fractional or negative value; a whole non-negative one flows through
    /// `to_cardinal` + the `ords` suffix table — identical to the integer
    /// ordinal ("5.0" → "vyfde", `1e+16` → "tien biljardste").
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        self.to_ordinal(&whole)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal`, then
    /// `str(value) + ("de" if value in [2..8] else "ste")` — the membership
    /// test is numeric, so `5.0` and `Decimal("5.00")` both take "de"
    /// ("5.0de", "5.00de") while everything else takes "ste" ("-0.0ste",
    /// "1e+16ste").
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        let is_2_to_8 = value.as_whole_int().is_some()
            && whole >= BigInt::from(2)
            && whole <= BigInt::from(8);
        if is_2_to_8 {
            Ok(format!("{}de", repr_str))
        } else {
            Ok(format!("{}ste", repr_str))
        }
    }

    /// `to_year(float/Decimal)` — `Num2Word_AF.to_year`'s arithmetic run on
    /// the raw float/Decimal: negatives and sub-1000 values go straight to
    /// the cardinal, values whose hundreds digit is zero stay cardinal, and
    /// the rest split as `val // 100` + `val % 100` — so `1234.0` reads
    /// "twaalf vier-en-dertig", `100000.0` "een duisend honderd", and
    /// `1e+20` "een triljoen honderd".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: v, .. } => {
                let v = *v;
                if v >= 2000.0 {
                    if v == 2000.0 {
                        return self.cardinal_num_f64(v);
                    }
                    if v < 2010.0 {
                        // "twee duisend een", etc.
                        return Ok(format!(
                            "{} {}",
                            self.to_cardinal(&BigInt::from(2000))?,
                            self.cardinal_num_f64(v - 2000.0)?
                        ));
                    }
                } else if v < 1000.0 {
                    // Also every negative year and every small fraction.
                    return self.cardinal_num_f64(v);
                } else if (v / 100.0).floor().rem_euclid(10.0) == 0.0 {
                    return self.cardinal_num_f64(v);
                }
                // Century split: val // 100 and val % 100.
                let century = (v / 100.0).floor();
                let year_part = v.rem_euclid(100.0);
                if year_part == 0.0 {
                    Ok(format!("{} honderd", self.cardinal_num_f64(century)?))
                } else {
                    Ok(format!(
                        "{} {}",
                        self.cardinal_num_f64(century)?,
                        self.cardinal_num_f64(year_part)?
                    ))
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                let hundred = BigDecimal::from(100);
                let two_thousand = BigDecimal::from(2000);
                if d >= &two_thousand {
                    if d == &two_thousand {
                        return self.cardinal_num_dec(d);
                    }
                    if d < &BigDecimal::from(2010) {
                        return Ok(format!(
                            "{} {}",
                            self.to_cardinal(&BigInt::from(2000))?,
                            self.cardinal_num_dec(&(d - &two_thousand))?
                        ));
                    }
                } else if d < &BigDecimal::from(1000) {
                    return self.cardinal_num_dec(d);
                } else {
                    // Decimal // truncates toward zero; positive range here.
                    let q = (d / &hundred)
                        .with_scale_round(0, bigdecimal::RoundingMode::Down);
                    let q_int = q.as_bigint_and_exponent().0;
                    if (&q_int % BigInt::from(10)).is_zero() {
                        return self.cardinal_num_dec(d);
                    }
                }
                let century = (d / &hundred).with_scale_round(0, bigdecimal::RoundingMode::Down);
                let year_part = d - &century * &hundred;
                if year_part.is_zero() {
                    Ok(format!("{} honderd", self.cardinal_num_dec(&century)?))
                } else {
                    Ok(format!(
                        "{} {}",
                        self.cardinal_num_dec(&century)?,
                        self.cardinal_num_dec(&year_part)?
                    ))
                }
            }
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_AF.to_currency` only restates `Num2Word_Base.to_currency`'s
    // signature with Afrikaans defaults (`currency="ZAR"`, `separator=" en"`)
    // and forwards every argument to `super()` unchanged — EUR defines no
    // `to_currency`, so the call lands straight on Base. Those two defaults are
    // already carried by `default_currency`/`default_separator` above, so the
    // trait's `to_currency` is that method, and overriding it here would only
    // re-express the delegation Python is doing.
    //
    // `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
    // likewise inherited from Base untouched, and the trait defaults already
    // mirror them. So only the two data tables, the class name and AF's own
    // plural rule are overridden.

    fn lang_name(&self) -> &str {
        "Num2Word_AF"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    // `currency_precision` is deliberately NOT overridden. AF inherits
    // `Num2Word_Base.CURRENCY_PRECISION`, which is `{}`: EN *rebinds* rather
    // than mutates it (`self.CURRENCY_PRECISION = {...}` makes an instance
    // attribute), so EN's 1000-divisor entries never leak out to siblings. The
    // live AF instance reports `{}`, hence `.get(code, 100)` is always 100 —
    // exactly the trait default. AF has no 3-decimal or 0-decimal currency, so
    // the `divisor == 1` fast path in `default_to_currency` is unreachable here
    // and KWD/BHD/JPY fail earlier, on the forms lookup.

    /// `Num2Word_AF.pluralize`: `return forms[0]` — the count is ignored.
    ///
    /// This is the one hook where AF diverges from `Num2Word_EUR.pluralize`
    /// (`forms[0 if n == 1 else 1]`), and it is load-bearing rather than a
    /// no-op — but only just: 8 of AF's 10 form pairs are reduplicated
    /// ("rand"/"rand"), so the two rules agree there and the divergence hides.
    /// It surfaces on exactly the other two, both subunits: GBP's
    /// ("penny", "pence") and CNY's ("jiao", "fen"). EUR's rule would render
    /// 12.34 GBP as "...vier-en-dertig pence"; AF's says "...vier-en-dertig
    /// penny". Both readings are plausible Afrikaans, so a mis-port here is
    /// invisible to inspection. The corpus pins both.
    ///
    /// Python indexes the tuple directly, so an empty forms tuple would raise
    /// IndexError. Every entry in AF's table has two forms, so that is
    /// unreachable — but it is mapped to `Index` rather than panicking, to keep
    /// the exception type honest if the table ever changes.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .first()
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
