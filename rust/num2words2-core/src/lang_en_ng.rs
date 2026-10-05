//! Port of `lang_EN_NG.py` — Nigerian English.
//!
//! Registry check: `__init__.py` maps `"en_NG"` → `lang_EN_NG.Num2Word_EN_NG()`,
//! which is the class this file ports.
//!
//! `Num2Word_EN_NG(lang_EN.Num2Word_EN)` overrides **only** `CURRENCY_FORMS`,
//! `CURRENCY_ADJECTIVES` and `to_currency` — all three are currency-path state,
//! which `PORTING.md` puts out of scope. It does not touch `setup`,
//! `set_high_numwords`, `merge`, `to_cardinal`, `to_ordinal`, `to_ordinal_num`
//! or `to_year`. So for the four in-scope modes this language is *behaviourally
//! identical to plain `en`*, and this file is a deliberate transcription of
//! `lang_en.rs` rather than a thin re-export: the porting contract asks for one
//! self-standing file per language, and coupling to another agent's in-flight
//! module would be fragile.
//!
//! Beyond the integer engine this file also ports the float/Decimal entry
//! behaviour (`verify_ordinal`'s TypeErrors with the -0.0 pass-through,
//! `to_ordinal_num`'s `str(value) + ordinal[-2:]`, `to_year`'s issue-#67
//! non-integer-float guard with silent Decimal truncation),
//! `Num2Word_EN.to_fraction`'s idiomatic half/quarter forms (issue #584),
//! and the grammatical kwargs its Python signatures accept:
//! `to_currency(kobo=...)` (the `cents` flag renamed, unique to en_NG) and
//! the inherited `to_year(suffix=..., longval=...)` (shared with `lang_en.rs`).
//!
//! Inheritance chain walked: `Num2Word_EN_NG` → `lang_EN.Num2Word_EN` →
//! `lang_EUR.Num2Word_EUR` → `base.Num2Word_Base`. EN overrides
//! `set_high_numwords` to the short scale (step -3), so EUR's long-scale
//! illiard/illion pairing does not apply — only EUR's `gen_high_numwords`
//! stem generation and `setup`'s units/tens/lows tables are reused.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// Python `str(float)`/`repr(float)`, used only to build error-message text
/// (`errmsg_floatord`, `errmsg_negord`, `to_year`'s TypeError). The corpora
/// compare exception *types*; the text follows the Python format strings.
/// Whole floats keep their ".0" ("21.0"), |v| >= 1e16 switches to Python's
/// exponent form ("1e+20"), and -0.0 keeps its sign.
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

/// The effective `CURRENCY_FORMS` for `Num2Word_EN_NG`.
///
/// Python builds this table by **mutation**, not by merging. `Num2Word_EN_NG`
/// declares its own class-level `CURRENCY_FORMS` (NGN/EUR/USD), and
/// `Num2Word_EN.__init__` then runs `self.CURRENCY_FORMS[...] = ...`. Because
/// `Num2Word_Base.__init__` never copies the dict, that subscript-assign
/// resolves through the MRO to *EN_NG's own class dict* and mutates it in
/// place, growing it from 3 codes to 27.
///
/// Two consequences, both confirmed against the live Python:
///
///   * `Num2Word_EUR.CURRENCY_FORMS` is shadowed outright, so EUR-only codes
///     are **not** reachable from en_NG — `to_currency(1, currency="RUB")`
///     raises `NotImplementedError`, and so do PLN/SEK/NOK/HUF/RON/ISK/...
///     Do not "helpfully" fold EUR's table in; it would change behaviour.
///   * EN's `GBP` (`pound`/`pounds`, `penny`/`pence`) is what en_NG sees, not
///     EUR's (`pound sterling`/`pounds sterling`).
///
/// The mutation is idempotent — it always assigns the same constants — so
/// materialising the final 27-entry table once here is faithful. Insert order
/// mirrors Python's execution order so this can be diffed line-by-line against
/// the source; later inserts overwrite earlier ones exactly as Python's do.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::with_capacity(27);

    // -- lang_EN_NG.Num2Word_EN_NG.CURRENCY_FORMS (class attribute) --
    m.insert("NGN", CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));

    // -- lang_EN.Num2Word_EN.__init__ then mutates that same dict --
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("GBP", CurrencyForms::new(&["pound", "pounds"], &["penny", "pence"]));
    m.insert("NGN", CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"]));
    m.insert("AUD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("CAD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("NZD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("HKD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("SGD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("CHF", CurrencyForms::new(&["franc", "francs"], &["rappen", "rappen"]));
    m.insert("AED", CurrencyForms::new(&["dirham", "dirhams"], &["fils", "fils"]));
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"]));
    m.insert("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"]));
    m.insert("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &["cent", "cents"]));
    m.insert("BRL", CurrencyForms::new(&["real", "reais"], &["cent", "cents"]));
    m.insert("ZAR", CurrencyForms::new(&["rand", "rand"], &["cent", "cents"]));
    m.insert("SAR", CurrencyForms::new(&["riyal", "riyals"], &["halalah", "halalas"]));
    m.insert("QAR", CurrencyForms::new(&["riyal", "riyals"], &["dirham", "dirhams"]));
    m.insert("KWD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    // 3-decimal currencies (mils as subunit); see `currency_precision`.
    m.insert("BHD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("OMR", CurrencyForms::new(&["rial", "rials"], &["baisa", "baisa"]));
    m.insert("JOD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("TND", CurrencyForms::new(&["dinar", "dinars"], &["millime", "millimes"]));
    m.insert("LYD", CurrencyForms::new(&["dinar", "dinars"], &["dirham", "dirhams"]));
    m.insert("IQD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));

    m
}

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Builds the Latin-prefix illion stems and applies the elision rules that turn
/// e.g. "septen" + "vigint" into the correct "septemvigint"-style forms.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — tens is the OUTER loop.
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

pub struct LangEnNg {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    /// Built once in `new()`. Rebuilding it per call is what made an earlier
    /// revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangEnNg {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnNg {
    pub fn new() -> Self {
        // Num2Word_EUR.setup()
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

        // Num2Word_EN.set_high_numwords: short scale.
        //   max = 3 + 3 * len(high); zip(high, range(max, 3, -3))
        // zip stops at the shorter sequence, hence the `n <= 3` break.
        // len(high) == 100, so max == 303 and the smallest card is 10^6
        // ("m" + "illion" == "million"). MAXVAL therefore lands at 10^306,
        // which is why the exponent is safe to narrow to u32 here.
        let max = 3 + 3 * high.len() as i64;
        let mut n = max;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illion", word));
            n -= 3;
        }

        // Num2Word_EN.setup()
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "thousand"),
                (100, "hundred"),
                (90, "ninety"),
                (80, "eighty"),
                (70, "seventy"),
                (60, "sixty"),
                (50, "fifty"),
                (40, "forty"),
                (30, "thirty"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "twenty", "nineteen", "eighteen", "seventeen", "sixteen", "fifteen", "fourteen",
                "thirteen", "twelve", "eleven", "ten", "nine", "eight", "seven", "six", "five",
                "four", "three", "two", "one", "zero",
            ],
        );

        // base.py: MAXVAL = 1000 * list(self.cards.keys())[0]
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<&str, &str> = [
            ("one", "first"),
            ("two", "second"),
            ("three", "third"),
            ("four", "fourth"),
            ("five", "fifth"),
            ("six", "sixth"),
            ("seven", "seventh"),
            ("eight", "eighth"),
            ("nine", "ninth"),
            ("ten", "tenth"),
            ("eleven", "eleventh"),
            ("twelve", "twelfth"),
            ("twenty", "twentieth"),
            ("thirty", "thirtieth"),
            ("forty", "fortieth"),
            ("fifty", "fiftieth"),
            ("sixty", "sixtieth"),
            ("seventy", "seventieth"),
            ("eighty", "eightieth"),
            ("ninety", "ninetieth"),
            ("hundred", "hundredth"),
            ("thousand", "thousandth"),
            ("million", "millionth"),
            ("billion", "billionth"),
        ]
        .into_iter()
        .collect();

        LangEnNg {
            cards,
            maxval,
            ords,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`. The float branch is
    /// unreachable here — input is integral by construction — so only the
    /// negative branch survives, raising TypeError exactly as Python does.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == num_bigint::Sign::Minus {
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
    /// "zeroth", not an error. A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
    /// Returns the integral value for the integer-path continuation.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        match value.as_whole_int() {
            Some(i) => {
                if i.sign() == num_bigint::Sign::Minus {
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

impl Lang for LangEnNg {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NGN"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
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
        "point"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_EN.merge`. Drives the default `to_cardinal`
    /// (`splitnum`/`clean`) in `base.rs` — this is an engine-style language.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);

        if lnum.is_one() && rnum < &hundred {
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            (format!("{}-{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            (format!("{} and {}", ltext, rtext), lnum + rnum)
        } else if rnum > lnum {
            (format!("{} {}", ltext, rtext), lnum * rnum)
        } else {
            (format!("{}, {}", ltext, rtext), lnum + rnum)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        // Python indexes outwords[-1] / lastwords[-1] unguarded; both splits
        // always yield at least one element, so the unwraps cannot fire.
        let last = outwords.last().unwrap().clone();
        let mut lastwords: Vec<String> = last.split('-').map(|s| s.to_string()).collect();
        let lastword = lastwords.last().unwrap().to_lowercase();

        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => o.to_string(),
            None => {
                // Python: if lastword[-1] == "y": lastword = lastword[:-1] + "ie"
                //         lastword += "th"
                // Byte-slicing is safe: every EN numword is ASCII.
                if lastword.ends_with('y') {
                    format!("{}ieth", &lastword[..lastword.len() - 1])
                } else {
                    format!("{}th", lastword)
                }
            }
        };
        let n = lastwords.len();
        lastwords[n - 1] = self.title(&newlast);
        let n2 = outwords.len();
        outwords[n2 - 1] = lastwords.join("-");
        Ok(outwords.join(" "))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        // Python: "%s%s" % (value, self.to_ordinal(value)[-2:]) — the last two
        // *characters*. Take from the char iterator, never a byte offset.
        let suffix: String = {
            let tail: Vec<char> = ord.chars().rev().take(2).collect();
            tail.into_iter().rev().collect()
        };
        Ok(format!("{}{}", value, suffix))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Python's `suffix` and `longval` params are dispatcher-level and never
        // set for the in-scope `to_year` path, so `suffix` starts as None and
        // only the BC branch can populate it.
        let mut val = value.clone();
        let mut suffix: Option<&str> = None;
        if val.sign() == num_bigint::Sign::Minus {
            val = -val;
            suffix = Some("BC");
        }
        let hundred = BigInt::from(100);
        // val is already absolute, so Python's floor-division semantics for
        // `//` and `%` on negatives cannot bite here.
        let high = &val / &hundred;
        let low = &val % &hundred;

        let ten = BigInt::from(10);
        // 00XX, X00X, or beyond 9999 fall back to a plain cardinal.
        let valtext = if high.is_zero() || ((&high % &ten).is_zero() && low < ten) || high >= hundred
        {
            self.to_cardinal(&val)?
        } else {
            let hightext = self.to_cardinal(&high)?;
            let lowtext = if low.is_zero() {
                "hundred".to_string()
            } else if low < ten {
                format!("oh-{}", self.to_cardinal(&low)?)
            } else {
                self.to_cardinal(&low)?
            };
            format!("{} {}", hightext, lowtext)
        };
        Ok(match suffix {
            Some(s) => format!("{} {}", valtext, s),
            None => valtext,
        })
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float checks and `to_year`'s
    // non-integer-float guard become reachable here — unlike on the BigInt
    // hooks above, where they are dead code. `to_cardinal` needs no
    // override: EN_NG inherits base's `assert int(value) == value` routing,
    // which is exactly the trait default (whole -> int path).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "fifth", Decimal("1E+2") -> "one
    /// hundredth"); fractional or negative values raise TypeError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s" % (value, to_ordinal(value)[-2:])`.
    /// The numeric half is Python's `str(value)` (`repr_str`), so
    /// Decimal("5.00") yields "5.00th", 1.0 yields "1.0st" and -0.0 yields
    /// "-0.0th". An oversized value propagates `to_ordinal`'s OverflowError.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        let ord = self.to_ordinal(&i)?;
        let suffix: String = {
            let chars: Vec<char> = ord.chars().collect();
            let start = chars.len().saturating_sub(2);
            chars[start..].iter().collect()
        };
        Ok(format!("{}{}", repr_str, suffix))
    }

    /// `Num2Word_EN.to_year(float/Decimal)`, inherited unchanged by EN_NG:
    /// non-integer *floats* raise TypeError (issue #67) — NaN/±inf fail
    /// `is_integer()` the same way — while Decimals of any scale skip the
    /// guard (`isinstance(val, float)` is False) and truncate via `int(val)`,
    /// so Decimal("1.5") is year 1. Whole values continue into the integer
    /// `to_year` (BC for negatives).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                if !(f.is_finite() && f.fract() == 0.0) {
                    return Err(N2WError::Type(format!(
                        "to='year' expects an integer; got non-integer float {}",
                        py_float_str(*f)
                    )));
                }
                let i = value
                    .as_whole_int()
                    .ok_or_else(|| N2WError::Type("unreachable: whole float".into()))?;
                self.to_year(&i)
            }
            FloatValue::Decimal { value: d, .. } => {
                // int(Decimal) truncates toward zero.
                let i = d.with_scale(0).as_bigint_and_exponent().0;
                self.to_year(&i)
            }
        }
    }

    /// Port of `Num2Word_EN.to_fraction` (issue #584), inherited unchanged
    /// by EN_NG: idiomatic "half/halves" and "quarter/quarters" for
    /// denominators 2 and 4; every other denominator is `to_ordinal` + bare
    /// "s" ("7/100" -> "seven one hundredths"). The sign is the literal
    /// "minus " prefix from the Python source; `d == 1` (exactly 1, not -1)
    /// or `n == 0` short-circuits to the signed cardinal, before any sign
    /// normalisation.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();
        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "half" } else { "halves" }).to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "quarter" } else { "quarters" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() {
                w.push('s');
            }
            w
        };
        let sign = if is_negative { "minus " } else { "" };
        Ok(format!("{}{} {}", sign, self.to_cardinal(&abs_n)?, den_word))
    }

    // ---- grammatical kwargs ----------------------------------------------
    //
    // en_NG is the one EN-family language whose Python signatures accept
    // extra keyword arguments:
    //
    //   to_currency(self, val, currency="NGN", kobo=True, separator=",",
    //               adjective=False)               # lang_EN_NG.py
    //   to_year(self, val, suffix=None, longval=True)  # inherited lang_EN.py
    //
    // Each hook starts with the `kw.only(...)` guard listing exactly those
    // names: any other kwarg is NotImplemented, so the dispatcher falls back
    // to Python, which raises the original TypeError.

    /// `Num2Word_EN_NG.to_currency`'s `kobo` kwarg: the base `cents`
    /// parameter renamed. The Python body forwards `cents=kobo` to
    /// `Num2Word_Base.to_currency`, where it is only ever tested truthily
    /// (`if cents:`), so every KwVal maps through Python truthiness —
    /// `kobo=None` behaves like `kobo=False`.
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.to_currency(val, currency, cents, separator, adjective);
        }
        if !kw.only(&["kobo"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let kobo = match kw.get("kobo") {
            Some(KwVal::Bool(b)) => *b,
            Some(KwVal::Int(i)) => *i != 0,
            Some(KwVal::Str(s)) => !s.is_empty(),
            Some(KwVal::List(l)) => !l.is_empty(),
            Some(KwVal::None) => false,
            None => cents,
        };
        self.to_currency(val, currency, kobo, separator, adjective)
    }

    /// `Num2Word_EN.to_year(val, suffix=None, longval=True)` with the
    /// caller's kwargs. `longval` is accepted and ignored — the EN body
    /// never reads it. `suffix` participates twice, both times truthily:
    /// a falsy suffix lets negatives default to "BC" (`suffix = "BC" if not
    /// suffix else suffix`), and a falsy suffix is never appended
    /// (`valtext if not suffix else "%s %s"`), so `suffix=""` behaves like
    /// `suffix=None`. Non-str suffix values fall back to Python.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_year(value);
        }
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let mut sfx: Option<String> = match kw.get("suffix") {
            None | Some(KwVal::None) => None,
            Some(KwVal::Str(s)) => {
                if s.is_empty() {
                    None // "" is falsy in both suffix tests
                } else {
                    Some(s.clone())
                }
            }
            // A bool/int/list suffix would be str()-interpolated by Python's
            // "%s %s"; keep those on the Python side rather than guessing.
            _ => return Err(N2WError::Fallback("kwargs".into())),
        };
        // Same body as the no-kwargs `to_year` above, with the caller's
        // suffix taking precedence over the BC default for negatives.
        let mut val = value.clone();
        if val.sign() == num_bigint::Sign::Minus {
            val = -val;
            if sfx.is_none() {
                sfx = Some("BC".to_string());
            }
        }
        let hundred = BigInt::from(100);
        let high = &val / &hundred;
        let low = &val % &hundred;

        let ten = BigInt::from(10);
        let valtext = if high.is_zero() || ((&high % &ten).is_zero() && low < ten) || high >= hundred
        {
            self.to_cardinal(&val)?
        } else {
            let hightext = self.to_cardinal(&high)?;
            let lowtext = if low.is_zero() {
                "hundred".to_string()
            } else if low < ten {
                format!("oh-{}", self.to_cardinal(&low)?)
            } else {
                self.to_cardinal(&low)?
            };
            format!("{} {}", hightext, lowtext)
        };
        Ok(match sfx {
            Some(s) => format!("{} {}", valtext, s),
            None => valtext,
        })
    }

    // ---- currency ------------------------------------------------------
    //
    // `Num2Word_EN_NG` itself overrides only `CURRENCY_FORMS`,
    // `CURRENCY_ADJECTIVES` and `to_currency`; everything else on the currency
    // path is inherited. Mapped hook by hook:
    //
    //   lang_name            -> class name, for the NotImplementedError message
    //   currency_forms       -> the mutated 27-entry dict (see above)
    //   currency_adjective   -> EN_NG.CURRENCY_ADJECTIVES (3 entries)
    //   currency_precision   -> EN.__init__'s CURRENCY_PRECISION
    //   pluralize            -> Num2Word_EUR.pluralize
    //
    // Deliberately *not* overridden, because the trait defaults already match
    // the inherited Python:
    //
    //   money_verbose / cents_verbose -> Num2Word_Base, both = to_cardinal
    //   cents_terse                   -> Num2Word_Base._cents_terse
    //   to_cheque                     -> Num2Word_Base.to_cheque
    //   to_currency                   -> Num2Word_EN_NG.to_currency only
    //     renames the `cents` parameter to `kobo` and forwards to
    //     `super().to_currency(..., cents=kobo, ...)`. The rename is a
    //     Python-level keyword-argument concern with no effect on output, and
    //     the dispatcher's Rust fast path passes `cents` positionally, so the
    //     default delegation to `currency::default_to_currency` is exact.

    fn lang_name(&self) -> &str {
        "Num2Word_EN_NG"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EN_NG.CURRENCY_ADJECTIVES`. EN_NG declares its own class
    /// attribute, which shadows `Num2Word_EUR.CURRENCY_ADJECTIVES` entirely —
    /// nothing mutates this one, so it stays at exactly three entries and
    /// `adjective=True` is a no-op for every other code (Python guards with
    /// `if adjective and currency in self.CURRENCY_ADJECTIVES`).
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        match code {
            "NGN" => Some("Nigerian"),
            "EUR" => Some("European"),
            "USD" => Some("US"),
            _ => None,
        }
    }

    /// `self.CURRENCY_PRECISION.get(code, 100)`.
    ///
    /// `Num2Word_EN.__init__` *rebinds* `self.CURRENCY_PRECISION` (plain
    /// assignment, not subscript-assign), so unlike CURRENCY_FORMS this one
    /// shadows rather than mutates, and holds exactly the seven 3-decimal
    /// codes. JPY and KRW are pointedly absent and therefore fall to the
    /// default 100 — so en_NG has **no** divisor-1 currency, and the
    /// zero-decimal branch of `default_to_currency` is unreachable here.
    /// `to_currency(12.34, currency="JPY")` really is "twelve yen,
    /// thirty-four sen"; the historical sen subunit is intentional.
    fn currency_precision(&self, code: &str) -> i64 {
        match code {
            "BHD" | "KWD" | "OMR" | "JOD" | "TND" | "LYD" | "IQD" => 1000,
            _ => 100,
        }
    }

    /// Port of `Num2Word_EUR.pluralize`:
    ///     form = 0 if n == 1 else 1
    ///     return forms[form]
    ///
    /// Note `n == 0` takes the plural form ("zero euros"), and that the rule
    /// keys off `n == 1` exactly, not `abs(n) == 1` — callers already pass an
    /// absolute value.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        // Python indexes the tuple directly; a short tuple would raise
        // IndexError. Every en_NG entry has arity 2 and `prefix_currency`
        // preserves arity, so this cannot fire — but if it somehow did,
        // IndexError is the exception Python would surface.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
