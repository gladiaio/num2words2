//! Port of `lang_EN_IN.py` (`Num2Word_EN_IN`), an engine-style language.
//!
//! Inheritance chain: `Num2Word_EN_IN` -> `Num2Word_EN` -> `Num2Word_EUR`
//! -> `Num2Word_Base`. EN_IN contributes exactly one in-scope override:
//! `set_high_numwords`. Everything else (`merge`, `to_ordinal`,
//! `to_ordinal_num`, `to_year`, the `ords` table, `negword`/`pointword`/
//! `exclude_title`, and the mid/low numwords) is inherited from `Num2Word_EN`
//! verbatim and is reproduced here rather than shared, because `lang_en.rs`
//! keeps its `cards`/`ords` private and offers no hook to swap the card table.
//!
//! # The load-bearing detail: `set_high_numwords` drops its argument
//!
//! ```python
//! def set_high_numwords(self, high):
//!     self.cards[10 ** 19] = "mahashankh"
//!     ...
//!     self.cards[10 ** 5] = "lakh"
//! ```
//!
//! There is no `super()` call and `high` is never read. `Num2Word_EUR.setup`
//! still builds the ~100 Latin-prefix illion stems into `self.high_numwords`,
//! and `Num2Word_Base.set_numwords` still passes them in — they are simply
//! discarded. So EN_IN has **no illion cards at all**: the table is 8 Vedic
//! cards + 9 mid + 21 low = 38 entries, versus EN's ~130. Consequences:
//!
//! - `MAXVAL = 1000 * list(self.cards.keys())[0]`. `cards` is an `OrderedDict`
//!   and the first *inserted* key is `10**19`, so MAXVAL is `10**22` — not the
//!   `10**303`-ish ceiling EN enjoys. Verified against `corpus_edges.jsonl`:
//!   `9999999999999999999999` converts, `10**22` raises OverflowError.
//!   The high cards are inserted descending, so Python's insertion order and
//!   this crate's sorted-descending `Cards` agree and `highest()` == `keys()[0]`.
//! - There is no `10**21` card, so `10**21` renders as "one hundred mahashankh".
//!
//! # Preserved upstream bug: ordinals of Vedic scale words
//!
//! `Num2Word_EN.to_ordinal` looks the final word up in `self.ords`, which only
//! knows English scale names ("hundred"/"thousand"/"million"/"billion"). EN_IN
//! never extends it, so every Vedic card falls through to the generic `+ "th"`
//! rule and yields "one lakhth", "one croreth", "one arabth", "one padmath",
//! "ten shankhth", "one hundred mahashankhth". These are wrong English but they
//! are what Python emits (confirmed in `corpus.jsonl`) and are reproduced
//! verbatim. Conversely "million"/"billion" remain in `ords` while being
//! unreachable here, since no card ever produces those words.
//!
//! # Preserved upstream bug: `CURRENCY_FORMS` is mutated shared class state
//!
//! `CURRENCY_FORMS` is a **class attribute** declared once on `Num2Word_EUR`.
//! `Num2Word_EN.__init__` then does
//!
//! ```python
//! self.CURRENCY_FORMS["EUR"] = (("euro", "euros"), ("cent", "cents"))
//! ```
//!
//! which is *not* an instance-attribute assignment: `self.CURRENCY_FORMS`
//! resolves through the MRO to `Num2Word_EUR`'s dict and `__setitem__` mutates
//! that dict **in place**. Constructing a single `Num2Word_EN` therefore grows
//! `Num2Word_EUR.CURRENCY_FORMS` from 22 keys to 39 and rewrites entries other
//! languages read — `GBP` becomes ("pound", "pounds") rather than ("pound
//! sterling", "pounds sterling"), `EUR` gains a real plural ("euro", "euros")
//! instead of ("euro", "euro"), and `SAR` drops to ("riyal", "riyals").
//! `num2words2/__init__.py` instantiates every converter at import, so the
//! mutation has always already happened by the time anything is called; the
//! contaminated table *is* the runtime behaviour and hence the spec.
//!
//! For EN_IN specifically the result is order-independent — verified against
//! the live package: the fully-imported `en_IN` table is byte-identical to a
//! pristine `EUR + EN + EN_IN` merge, because the other converters sharing that
//! same dict object (`hu`, `kn`, `sv`, `te`, the `en_*` variants) never write a
//! conflicting value for any key. So flattening it into one table built in
//! `new()` is faithful. The 39 entries below were generated from the live
//! Python dict rather than transcribed by hand.
//!
//! `CURRENCY_PRECISION`, by contrast, *is* a plain rebind in
//! `Num2Word_EN.__init__` (`self.CURRENCY_PRECISION = {...}`), so it creates a
//! genuine instance attribute shadowing the empty base one: exactly the seven
//! 3-decimal dinar/rial codes at 1000, everything else defaulting to 100.
//! Note JPY/KRW are deliberately *not* 0-decimal here — they keep the 100
//! default and their historical sen/jeon subunits, so EN_IN never exercises
//! `base.to_currency`'s `divisor == 1` branch at all.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::{BigInt, Sign};
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

pub struct LangEnIn {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    /// The merged `CURRENCY_FORMS` table, built once in `new()`.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
    currency_precision: HashMap<&'static str, i64>,
}

impl Default for LangEnIn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnIn {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_EN_IN.set_high_numwords — the `high` argument is ignored.
        // Insertion order is descending, matching the Python OrderedDict, so
        // `cards.highest()` reproduces `list(self.cards.keys())[0]` == 10**19.
        let ten = BigInt::from(10u8);
        for (exp, word) in [
            (19u32, "mahashankh"),
            (17, "shankh"),
            (15, "padma"),
            (13, "neel"),
            (11, "kharab"),
            (9, "arab"),
            (7, "crore"),
            (5, "lakh"),
        ] {
            cards.insert(ten.pow(exp), word);
        }

        // Num2Word_EN.setup — mid/low numwords, inherited unchanged.
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

        // Num2Word_Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0].
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Num2Word_EN.setup's `ords`, copied verbatim. "million"/"billion" are
        // dead entries under EN_IN's card table but are kept for fidelity.
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

        // The merged CURRENCY_FORMS table — see the module docs for why this is
        // the union of Num2Word_EUR's class dict with the in-place mutations
        // Num2Word_EN.__init__ and Num2Word_EN_IN.__init__ perform on it.
        // Built once here and stored: constructing it per call is what made an
        // earlier revision of this port 10x slower than the Python original.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            // --- Num2Word_EUR.CURRENCY_FORMS, the declared class attribute ---
            ("AUD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("BYN", &["rouble", "roubles"][..], &["kopek", "kopeks"][..]),
            ("CAD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("EEK", &["kroon", "kroons"][..], &["sent", "senti"][..]),
            ("EUR", &["euro", "euros"][..], &["cent", "cents"][..]),
            ("GBP", &["pound", "pounds"][..], &["penny", "pence"][..]),
            ("LTL", &["litas", "litas"][..], &["cent", "cents"][..]),
            ("LVL", &["lat", "lats"][..], &["santim", "santims"][..]),
            ("USD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("RUB", &["rouble", "roubles"][..], &["kopek", "kopeks"][..]),
            ("SEK", &["krona", "kronor"][..], &["öre", "öre"][..]),
            ("NOK", &["krone", "kroner"][..], &["øre", "øre"][..]),
            // Three unit forms; pluralize only ever indexes 0 or 1, but the
            // arity is load-bearing and must not be trimmed.
            ("PLN", &["zloty", "zlotys", "zlotu"][..], &["grosz", "groszy"][..]),
            ("MXN", &["peso", "pesos"][..], &["cent", "cents"][..]),
            ("RON", &["leu", "lei", "de lei"][..], &["ban", "bani", "de bani"][..]),
            ("INR", &["rupee", "rupees"][..], &["paisa", "paise"][..]),
            ("HUF", &["forint", "forint"][..], &["fillér", "fillér"][..]),
            ("ISK", &["króna", "krónur"][..], &["aur", "aurar"][..]),
            ("UZS", &["sum", "sums"][..], &["tiyin", "tiyins"][..]),
            // EUR declares ("saudi riyal", ...); Num2Word_EN overwrites it.
            ("SAR", &["riyal", "riyals"][..], &["halalah", "halalas"][..]),
            ("JPY", &["yen", "yen"][..], &["sen", "sen"][..]),
            ("KRW", &["won", "won"][..], &["jeon", "jeon"][..]),
            // --- keys Num2Word_EN.__init__ adds to that same dict ---
            ("NGN", &["naira", "naira"][..], &["kobo", "kobo"][..]),
            ("NZD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("HKD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("SGD", &["dollar", "dollars"][..], &["cent", "cents"][..]),
            ("CHF", &["franc", "francs"][..], &["rappen", "rappen"][..]),
            ("AED", &["dirham", "dirhams"][..], &["fils", "fils"][..]),
            ("CNY", &["yuan", "yuan"][..], &["fen", "fen"][..]),
            ("BRL", &["real", "reais"][..], &["cent", "cents"][..]),
            ("ZAR", &["rand", "rand"][..], &["cent", "cents"][..]),
            ("QAR", &["riyal", "riyals"][..], &["dirham", "dirhams"][..]),
            ("KWD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            ("BHD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            ("OMR", &["rial", "rials"][..], &["baisa", "baisa"][..]),
            ("JOD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            ("TND", &["dinar", "dinars"][..], &["millime", "millimes"][..]),
            ("LYD", &["dinar", "dinars"][..], &["dirham", "dirhams"][..]),
            ("IQD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            // Num2Word_EN_IN.__init__ re-sets INR to the value EN already gave
            // it, so it adds nothing; listed above with the rest.
        ]
        .into_iter()
        .map(|(code, unit, subunit)| (code, CurrencyForms::new(unit, subunit)))
        .collect();

        // Num2Word_EUR.CURRENCY_ADJECTIVES — never mutated anywhere in the
        // EN_IN chain, so it is the pristine class attribute.
        let currency_adjectives: HashMap<&'static str, &'static str> = [
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
        .collect();

        // Num2Word_EN.__init__'s CURRENCY_PRECISION rebind. 3-decimal
        // currencies only; everything else falls back to 100.
        let currency_precision: HashMap<&'static str, i64> = [
            ("BHD", 1000),
            ("KWD", 1000),
            ("OMR", 1000),
            ("JOD", 1000),
            ("TND", 1000),
            ("LYD", 1000),
            ("IQD", 1000),
        ]
        .into_iter()
        .collect();

        LangEnIn {
            cards,
            maxval,
            ords,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
            currency_forms,
            currency_adjectives,
            currency_precision,
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check cannot fire on integer
    /// input, so only the negative branch survives: TypeError, not ValueError.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == Sign::Minus {
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
                if i.sign() == Sign::Minus {
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

impl Lang for LangEnIn {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "INR"
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

    /// `Num2Word_EN.merge`, inherited unchanged.
    ///
    /// Python's `100 > lnum > rnum` is a chained comparison and `lnum >= 100 >
    /// rnum` likewise; both are spelled out as explicit conjunctions here.
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

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        default_to_cardinal(self, value)
    }

    /// `Num2Word_EN.to_ordinal`.
    ///
    /// Python mutates `lastword` in the `except KeyError` branch:
    /// `if lastword[-1] == "y": lastword = lastword[:-1] + "ie"` then
    /// `lastword += "th"` — so "ninety" would become "ninetieth" via this path,
    /// though in practice `ords` catches it first. Unknown words (every Vedic
    /// scale name) just gain "th".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        // to_cardinal never returns "", so both `last()` unwraps are safe:
        // str::split always yields at least one element.
        let last = outwords.last().unwrap().clone();
        let mut lastwords: Vec<String> = last.split('-').map(|s| s.to_string()).collect();
        let lastword = lastwords.last().unwrap().to_lowercase();

        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => (*o).to_string(),
            None => {
                if lastword.ends_with('y') {
                    // Drop the final char, not the final byte.
                    let mut chars = lastword.chars();
                    chars.next_back();
                    format!("{}ieth", chars.as_str())
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

    /// `Num2Word_EN.to_ordinal_num`: `"%s%s" % (value, self.to_ordinal(value)[-2:])`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        // Python's [-2:] is by character and clamps rather than panicking on
        // strings shorter than 2.
        let suffix: String = {
            let chars: Vec<char> = ord.chars().collect();
            chars[chars.len().saturating_sub(2)..].iter().collect()
        };
        Ok(format!("{}{}", value, suffix))
    }

    /// `Num2Word_EN.to_year`. The float guard is unreachable on integer input.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut val = value.clone();
        let mut suffix: Option<&str> = None;
        if val.sign() == Sign::Minus {
            val = -val;
            suffix = Some("BC");
        }
        let hundred = BigInt::from(100);
        let ten = BigInt::from(10);
        // Python: high, low = (val // 100, val % 100). `val` is non-negative
        // here, so floor and truncating division agree.
        let high = &val / &hundred;
        let low = &val % &hundred;

        // If year is 00XX, X00X, or beyond 9999, go cardinal.
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
    // override: EN_IN inherits base's `assert int(value) == value` routing,
    // which is exactly the trait default (whole -> int path).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "fifth", 1e+20 -> "ten mahashankhth");
    /// fractional or negative values raise TypeError.
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

    /// `Num2Word_EN.to_year(float/Decimal)`, inherited unchanged by EN_IN:
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
    /// by EN_IN: idiomatic "half/halves" and "quarter/quarters" for
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
            // 1/100 is "one hundredth", not "one one hundredth" (#217): the
            // ordinal of a power of ten already starts with "one".
            if matches!(abs_d.to_string().strip_prefix('1'), Some(z) if z.bytes().all(|b| b == b'0')) {
                if let Some(rest) = w.strip_prefix("one ") {
                    w = rest.to_string();
                }
            }
            if !abs_n.is_one() {
                w.push('s');
            }
            w
        };
        let sign = if is_negative { "minus " } else { "" };
        Ok(format!("{}{} {}", sign, self.to_cardinal(&abs_n)?, den_word))
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_EN_IN` defines no currency behaviour of its own beyond seeding
    // INR (already present) and re-defaulting `to_currency`'s currency kwarg —
    // see the note on `to_currency` below. Everything here is the inherited
    // `Num2Word_EUR` / `Num2Word_Base` behaviour, which the trait defaults
    // already implement for `money_verbose` (-> to_cardinal), `cents_verbose`
    // (-> to_cardinal), `cents_terse` (-> default_cents_terse at this
    // language's precision), `to_currency` and `to_cheque`. Only the four
    // tables and `pluralize` need overriding.

    /// `self.__class__.__name__`, for the NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_EN_IN"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `self.CURRENCY_PRECISION.get(code, 100)`.
    fn currency_precision(&self, code: &str) -> i64 {
        self.currency_precision.get(code).copied().unwrap_or(100)
    }

    /// `Num2Word_EUR.pluralize`:
    ///
    /// ```python
    /// def pluralize(self, n, forms):
    ///     form = 0 if n == 1 else 1
    ///     return forms[form]
    /// ```
    ///
    /// Note `n == 1` — not `abs(n) == 1` — but every caller in `base` already
    /// passes an absolute value, so the sign never reaches here. The `forms[1]`
    /// lookup is a real Python list index: a one-element form tuple with
    /// `n != 1` would raise IndexError rather than falling back to the
    /// singular. No EN_IN entry has fewer than two forms, so the branch is
    /// unreachable, but it is reproduced as `N2WError::Index` rather than
    /// silently clamping, to keep the failure mode Python's.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    // `to_currency` is deliberately NOT overridden.
    //
    // Python's override exists only to change a default argument:
    //
    // ```python
    // def to_currency(self, val, currency="INR", **kwargs):
    //     return super().to_currency(val, currency=currency, **kwargs)
    // ```
    //
    // It re-defaults the currency from EUR to INR and otherwise delegates
    // straight to `Num2Word_Base.to_currency`. At this layer `currency` has
    // already been resolved to a concrete `&str` by the caller, and "the caller
    // explicitly asked for EUR" and "the caller passed nothing" arrive as the
    // identical string — so the default cannot be reapplied here without
    // wrongly rewriting explicit EUR requests into INR. The default belongs to
    // the dispatcher; see `concerns` in the report, as the shim currently
    // hardcodes "EUR" for every language.
}
