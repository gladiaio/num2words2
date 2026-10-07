//! Port of `lang_EN_NP.py` (English with the Nepalese number system).
//!
//! Registry check: `__init__.py` maps `"en_NP"` → `lang_EN_NP.Num2Word_EN_NP`,
//! so this is the right class.
//!
//! Shape: **engine**. `Num2Word_EN_NP` subclasses `Num2Word_EN` and overrides
//! exactly one method — `set_high_numwords` — replacing the short-scale
//! illion ladder with the Nepalese lakh/crore/arba/kharba/neel/padam/shankha
//! ladder. Everything else (`merge`, `to_ordinal`, `to_ordinal_num`,
//! `to_year`, `ords`, `negword`, `pointword`, `exclude_title`) is inherited
//! verbatim from `Num2Word_EN`, so `Num2Word_Base.to_cardinal` drives
//! `splitnum`/`clean` as usual.
//!
//! Inheritance chain walked: `Num2Word_EN_NP` → `Num2Word_EN` →
//! `Num2Word_EUR` → `Num2Word_Base`.
//!
//! # The one override, and what it silently discards
//!
//! ```python
//! def set_high_numwords(self, high):
//!     self.cards[10 ** 17] = "shankha"
//!     ...
//!     self.cards[10 ** 5]  = "lakh"
//! ```
//!
//! Note the parameter `high` is **accepted and never used**, and `super()` is
//! never called. `Num2Word_EUR.setup` still builds a 100-entry
//! `self.high_numwords` list ("cent", "novemnonagint", ... "m") via
//! `gen_high_numwords`, and `Num2Word_Base.set_numwords` still passes it in —
//! but this override drops it on the floor. None of the `*illion` cards ever
//! reach `self.cards`. That is why this file does not reuse
//! `lang_en::gen_high_numwords`: reproducing that dead computation would have
//! no observable effect. Verified against the interpreter: `len(high_numwords)
//! == 100`, yet `list(cards.keys())` holds only the 7 Nepalese entries plus
//! EN's mid/low words.
//!
//! # MAXVAL is 10^20, not 10^303
//!
//! `Num2Word_Base.__init__` computes `MAXVAL = 1000 * list(self.cards.keys())[0]`
//! — 1000× the *first inserted* key. Since `set_high_numwords` runs before
//! `set_mid_numwords`/`set_low_numwords` and its first assignment is `10**17`,
//! `MAXVAL == 10**20`. Confirmed against the interpreter. So this language
//! overflows four orders of magnitude below plain `en`:
//! `to_cardinal(10**20)` raises `OverflowError`, while `10**20 - 1` still
//! renders ("nine hundred and ninety-nine shankha, ninety-nine padam, ...").
//! Values stay well inside `u64`, but the trait speaks `BigInt` and the
//! overflow bound must be compared exactly, so nothing is narrowed here.
//!
//! # Card ordering
//!
//! Python's `self.cards` is an `OrderedDict` and `splitnum` iterates it in
//! **insertion** order, which for this class happens to be strictly
//! descending: 10^17, 10^15, 10^13, 10^11, 10^9, 10^7, 10^5 (high), then
//! 1000, 100, 90 … 30 (mid), then 20 … 0 (low). `base::Cards` keeps entries
//! sorted descending, so the two iteration orders coincide exactly and no
//! divergence is possible. (This is worth stating because it is *not*
//! guaranteed in general — a language whose insertion order is not sorted
//! would diverge.)
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are odd but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **`ords` retains dead entries.** `Num2Word_EN.setup` maps
//!    `"million" → "millionth"` and `"billion" → "billionth"`, but EN_NP
//!    removes every `*illion` card, so those two keys are unreachable —
//!    `to_cardinal` can never emit "million" or "billion". They are kept
//!    here verbatim rather than pruned.
//! 2. **No Nepalese ordinals.** `ords` is inherited unchanged and knows
//!    nothing of lakh/crore/arba/…, so `to_ordinal` falls into EN's generic
//!    `except KeyError` suffixing branch and simply glues "th" on:
//!    `to_ordinal(100000)` == "one lakhth", `to_ordinal(10**7)` ==
//!    "one croreth", `to_ordinal(10**9)` == "one arbath",
//!    `to_ordinal(10**11)` == "one kharbath", `to_ordinal(10**15)` ==
//!    "one padamth", `to_ordinal(10**18)` == "ten shankhath". All confirmed
//!    against the corpus.
//! 3. **`to_ordinal_num` slices the *words*.** EN does
//!    `"%s%s" % (value, self.to_ordinal(value)[-2:])` — it takes the last two
//!    characters of the spelled-out ordinal. That is why `to_ordinal_num` of
//!    any lakh/crore/arba value is `"...th"` (from "…lakhth"), and why it
//!    inherits `to_ordinal`'s `OverflowError` above 10^20 rather than just
//!    printing the digits.
//! 4. **`to_year` mixes scales.** `to_year` is EN's and hard-codes the
//!    Western "hundred" split, so it never reaches the Nepalese cards for
//!    4-digit years, but `to_year(10**20)` still raises `OverflowError`
//!    through `to_cardinal`.
//!
//! # Error variants
//!
//! * `to_cardinal(v)` with `|v| >= 10**20` → `OverflowError` → `N2WError::Overflow`.
//! * `to_ordinal(v)` / `to_ordinal_num(v)` with `v < 0` → `verify_ordinal`
//!   raises `TypeError` ("Cannot treat negative num %s as ordinal.") →
//!   `N2WError::Type`. Note `to_cardinal(-1)` is fine ("minus one"); only the
//!   ordinal paths reject negatives.
//! * `to_ordinal(v)` / `to_ordinal_num(v)` with `v >= 10**20` →
//!   `OverflowError` propagated out of the inner `to_cardinal`.
//!
//! No cross-call mutable state: EN_NP defines no `str_to_number` and no
//! pending-ordinal handshake, so this converter is safely stateless.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
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

/// `10.pow(n)` as a `BigInt`.
fn pow10(n: u32) -> BigInt {
    BigInt::from(10u8).pow(n)
}

/// The effective `CURRENCY_FORMS` for `Num2Word_EN_NP`, built the way Python
/// builds it: `Num2Word_EUR`'s class-level dict, then the per-code overrides
/// `Num2Word_EN.__init__` writes on top. EN_NP itself adds nothing.
///
/// Reproducing the two stages (rather than pre-merging by hand) keeps this
/// diffable line-for-line against the two Python sources. Verified against the
/// interpreter: 39 keys, and `NPR` is deliberately **not** among them — the
/// Nepalese converter has no Nepalese-rupee entry, so `to_currency(x, "NPR")`
/// raises NotImplementedError. That is Python's behaviour, not an omission.
///
/// Arity is load-bearing. `PLN` carries three unit forms and `RON` three of
/// each; `Num2Word_Base.to_cheque` takes `cr1[-1]`, so dropping the third form
/// would silently turn `to_cheque(1234.56, "PLN")` from "... ZLOTU" into
/// "... ZLOTYS" and RON from "... DE LEI" into "... LEI". Both confirmed
/// against the interpreter.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();

    // ---- Num2Word_EUR.CURRENCY_FORMS (class attribute) ----
    // GENERIC_DOLLARS = ("dollar", "dollars"), GENERIC_CENTS = ("cent", "cents")
    m.insert("AUD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("BYN", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"]));
    m.insert("CAD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("EEK", CurrencyForms::new(&["kroon", "kroons"], &["sent", "senti"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cents"]));
    m.insert(
        "GBP",
        CurrencyForms::new(&["pound sterling", "pounds sterling"], &["penny", "pence"]),
    );
    m.insert("LTL", CurrencyForms::new(&["litas", "litas"], &["cent", "cents"]));
    m.insert("LVL", CurrencyForms::new(&["lat", "lats"], &["santim", "santims"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("RUB", CurrencyForms::new(&["rouble", "roubles"], &["kopek", "kopeks"]));
    m.insert("SEK", CurrencyForms::new(&["krona", "kronor"], &["öre", "öre"]));
    m.insert("NOK", CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"]));
    m.insert(
        "PLN",
        CurrencyForms::new(&["zloty", "zlotys", "zlotu"], &["grosz", "groszy"]),
    );
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &["cent", "cents"]));
    m.insert(
        "RON",
        CurrencyForms::new(&["leu", "lei", "de lei"], &["ban", "bani", "de bani"]),
    );
    m.insert("INR", CurrencyForms::new(&["rupee", "rupees"], &["paisa", "paise"]));
    m.insert("HUF", CurrencyForms::new(&["forint", "forint"], &["fillér", "fillér"]));
    m.insert("ISK", CurrencyForms::new(&["króna", "krónur"], &["aur", "aurar"]));
    m.insert("UZS", CurrencyForms::new(&["sum", "sums"], &["tiyin", "tiyins"]));
    m.insert(
        "SAR",
        CurrencyForms::new(&["saudi riyal", "saudi riyals"], &["halalah", "halalas"]),
    );
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"]));

    // ---- Num2Word_EN.__init__ overrides, in source order ----
    // Proper English pluralization. Note EUR ("euro","euro") becomes
    // ("euro","euros") and GBP loses the "sterling".
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m.insert("GBP", CurrencyForms::new(&["pound", "pounds"], &["penny", "pence"]));
    m.insert("NGN", CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"]));
    // Common ISO 4217 codes that downstream users hit (#74).
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
    // EN drops EUR's "saudi " prefix here; CURRENCY_ADJECTIVES["SAR"] still
    // supplies "Saudi" when adjective=True.
    m.insert("SAR", CurrencyForms::new(&["riyal", "riyals"], &["halalah", "halalas"]));
    m.insert("QAR", CurrencyForms::new(&["riyal", "riyals"], &["dirham", "dirhams"]));
    m.insert("KWD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    // 3-decimal currencies (mils as subunit). Issue #256.
    m.insert("BHD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("OMR", CurrencyForms::new(&["rial", "rials"], &["baisa", "baisa"]));
    m.insert("JOD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));
    m.insert("TND", CurrencyForms::new(&["dinar", "dinars"], &["millime", "millimes"]));
    m.insert("LYD", CurrencyForms::new(&["dinar", "dinars"], &["dirham", "dirhams"]));
    m.insert("IQD", CurrencyForms::new(&["dinar", "dinars"], &["fils", "fils"]));

    m
}

pub struct LangEnNp {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    /// Built once in `new()`. Constructing this per call is what made an
    /// earlier revision of this port 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangEnNp {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEnNp {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_EN_NP.set_high_numwords — the sole override. The `high`
        // argument (EUR's 100 illion stems) is ignored, and super() is not
        // called, so no *illion card is ever inserted. Insertion order below
        // matches the Python source line-for-line; it also fixes MAXVAL,
        // since Base.__init__ reads list(cards.keys())[0] == 10**17.
        cards.insert(pow10(17), "shankha");
        cards.insert(pow10(15), "padam");
        cards.insert(pow10(13), "neel");
        cards.insert(pow10(11), "kharba");
        cards.insert(pow10(9), "arba");
        cards.insert(pow10(7), "crore");
        cards.insert(pow10(5), "lakh");

        // Inherited verbatim from Num2Word_EN.setup.
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

        // Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0].
        // Highest card is 10**17 (shankha), so MAXVAL == 10**20.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Num2Word_EN.setup's `ords`, inherited unchanged. "million" and
        // "billion" are dead keys here (see module docs) but are kept
        // verbatim — pruning them would be a rewrite, not a port.
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

        LangEnNp {
            cards,
            maxval,
            ords,
            exclude_title: vec!["and".into(), "point".into(), "minus".into()],
            currency_forms: build_currency_forms(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float branch is unreachable for
    /// integer input, so only the negative check survives — and it raises
    /// `TypeError`, not `ValueError`.
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

impl Lang for LangEnNp {
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

    /// `Num2Word_EN.merge`, inherited unchanged by EN_NP.
    ///
    /// The Nepalese cards ride this untouched: e.g. merging ("thirty-four",
    /// 34) with ("lakh", 10^5) hits the `rnum > lnum` arm and multiplies,
    /// giving "thirty-four lakh"; a following remainder hits the final
    /// comma arm, giving "thirty-four lakh, fifty-six thousand, ...".
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

    /// `Num2Word_EN.to_ordinal`, inherited unchanged.
    ///
    /// Splits the cardinal on " ", then the final token on "-", and rewrites
    /// only the very last hyphen-fragment. With no Nepalese entries in
    /// `ords`, "lakh"/"crore"/"arba"/… fall through to the bare "th" suffix.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        // Python: self.to_cardinal(value).split(" ") — a plain split on a
        // single space, not split_whitespace.
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        let last = outwords.last().unwrap().clone();
        let mut lastwords: Vec<String> = last.split('-').map(|s| s.to_string()).collect();
        let lastword = lastwords.last().unwrap().to_lowercase();

        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => o.to_string(),
            None => {
                // Python: if lastword[-1] == "y": lastword = lastword[:-1] + "ie"
                //         lastword += "th"
                // All numwords here are ASCII, so byte slicing is safe.
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

    /// `Num2Word_EN.to_ordinal_num`: `"%s%s" % (value, self.to_ordinal(value)[-2:])`.
    ///
    /// The suffix is the last two *characters of the words*, which is why
    /// this inherits `to_ordinal`'s OverflowError above 10^20 instead of
    /// simply echoing the digits.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        let suffix: String = {
            let mut tail: Vec<char> = ord.chars().rev().take(2).collect();
            tail.reverse();
            tail.into_iter().collect()
        };
        Ok(format!("{}{}", value, suffix))
    }

    /// `Num2Word_EN.to_year`, inherited unchanged.
    ///
    /// Purely Western in structure — it splits on 100 and never consults the
    /// Nepalese cards for a 4-digit year. `val` is absolute before any
    /// division, so Python's floor-division-on-negatives semantics never
    /// diverge from Rust's truncating `/`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut val = value.clone();
        let mut suffix: Option<&str> = None;
        if val.sign() == Sign::Minus {
            val = -val;
            suffix = Some("BC");
        }
        let hundred = BigInt::from(100);
        let ten = BigInt::from(10);
        let high = &val / &hundred;
        let low = &val % &hundred;

        // Python: if high == 0 or (high % 10 == 0 and low < 10) or high >= 100
        // — 00XX, X00X and anything past 9999 fall back to a plain cardinal.
        let valtext = if high.is_zero()
            || ((&high % &ten).is_zero() && low < ten)
            || high >= hundred
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
    // override: EN_NP inherits base's `assert int(value) == value` routing,
    // which is exactly the trait default (whole -> int path).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "fifth", 1e+16 -> "ten padamth");
    /// fractional or negative values raise TypeError. 1e+20 passes the
    /// verify but overflows EN_NP's 10^20 MAXVAL inside `to_cardinal`,
    /// surfacing the OverflowError Python raises.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s" % (value, to_ordinal(value)[-2:])`.
    /// The numeric half is Python's `str(value)` (`repr_str`), so
    /// Decimal("5.00") yields "5.00th", 1.0 yields "1.0st" and -0.0 yields
    /// "-0.0th". 1e+20 propagates `to_ordinal`'s OverflowError (MAXVAL 10^20).
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

    /// `Num2Word_EN.to_year(float/Decimal)`, inherited unchanged by EN_NP:
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
    /// by EN_NP: idiomatic "half/halves" and "quarter/quarters" for
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
    // EN_NP defines *no* currency behaviour of its own — it overrides only
    // `set_high_numwords`. Everything below is inherited:
    //
    //   CURRENCY_FORMS       Num2Word_EUR (class attr) + Num2Word_EN.__init__
    //   CURRENCY_ADJECTIVES  Num2Word_EUR (class attr)
    //   CURRENCY_PRECISION   Num2Word_EN.__init__ (fresh instance dict)
    //   pluralize            Num2Word_EUR
    //   to_currency          Num2Word_Base   -> default_to_currency
    //   to_cheque            Num2Word_Base   -> default_to_cheque
    //   _money_verbose       Num2Word_Base   -> to_cardinal
    //   _cents_verbose       Num2Word_Base   -> to_cardinal
    //   _cents_terse         Num2Word_Base   -> default_cents_terse
    //
    // Method resolution confirmed against the interpreter, so only the four
    // data hooks plus `pluralize` are overridden here; the rest ride the
    // trait defaults, which already mirror Num2Word_Base.
    //
    // The Nepalese cards do reach the currency surface through
    // `_money_verbose` -> `to_cardinal`: `to_currency(1000000, "USD")` is
    // "ten lakh dollars", not "one million dollars".

    fn lang_name(&self) -> &str {
        "Num2Word_EN_NP"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged. A `match` is
    /// used rather than a second HashMap: it returns `&'static str` with no
    /// allocation and no per-call table construction.
    ///
    /// Note "íslenskar" (ISK) is lowercase and non-ASCII in the Python source
    /// — kept verbatim.
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

    /// `CURRENCY_PRECISION.get(code, 100)`.
    ///
    /// `Num2Word_EN.__init__` *replaces* the dict wholesale (it does not
    /// mutate Base's), so these seven 3-decimal codes are the only entries.
    ///
    /// JPY and KRW are deliberately **absent** and therefore fall to 100, even
    /// though both are 0-decimal in the real world — the Python comment says
    /// the historical sen/jeon subunits are still expected by the fixtures.
    /// So `to_currency(12.34, "JPY")` is "twelve yen, thirty-four sen", and
    /// this language never exercises `default_to_currency`'s `divisor == 1`
    /// branch. Confirmed against the corpus.
    fn currency_precision(&self, code: &str) -> i64 {
        match code {
            "BHD" | "KWD" | "OMR" | "JOD" | "TND" | "LYD" | "IQD" => 1000,
            _ => 100,
        }
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Index 1 unconditionally for every n != 1, so the third form of PLN/RON
    /// is unreachable *here* — but `to_cheque` reads `cr1[-1]` directly, which
    /// is why the tables keep it. A 1-element tuple would raise IndexError in
    /// Python; every entry has >= 2 forms and `prefix_currency` preserves
    /// length, so the fallback is unreachable — it is wired to `Index` rather
    /// than a panic to keep the exception type honest if that ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
