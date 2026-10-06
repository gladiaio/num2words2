//! Port of `lang_IT.py` (Italian).
//!
//! Registry check: `__init__.py` maps `"it"` → `lang_IT.Num2Word_IT()`, so
//! this ports `Num2Word_IT`.
//!
//! Shape: **self-contained**. `Num2Word_IT` subclasses `Num2Word_EUR` →
//! `Num2Word_Base`, but it overrides `__init__` to call *only* `self.setup()`
//! and never chains to `Num2Word_Base.__init__`. Two consequences drive this
//! whole port:
//!
//!   1. `self.cards` / `self.MAXVAL` are never built (the `high_numwords`
//!      guard in `Num2Word_Base.__init__` never runs), so `splitnum`/`clean`/
//!      `merge` are dead code. The only ceiling is `big_number_to_cardinal`'s
//!      explicit `length >= 66` guard, which Python raises as
//!      `NotImplementedError`. Fixed (gladiaio/num2words2#159): the port
//!      raises `OverflowError` ("abs(v) must be less than 10^65.") and
//!      `maxval()` reports 10^65. `cards`/`merge` stay at their trait
//!      defaults here.
//!   2. The `errmsg_*` attributes are never assigned either — see bug 4 below
//!      (fixed: the port raises Base's wording itself).
//!
//! `Num2Word_EUR.setup` only populates `self.high_numwords`, which is dead
//! for the same reason; `Num2Word_IT.setup` additionally sets
//! `self.negword = "meno"`. `negword` is used by the four in-scope modes only
//! via `to_ordinal_num`'s `verify_ordinal` (which crashes first, see bug 4),
//! so the sign handling below goes through `MINUS_PREFIX_WORD` (`"meno "`)
//! exactly as Python does.
//!
//! Inherited from `Num2Word_Base` and left alone by IT:
//!   * `to_year(value) -> self.to_cardinal(value)` → the trait default
//!     delegates through `&self` and picks up the `to_cardinal` override
//!     below. Verified against the corpus (`year -500` → "meno cinquecento").
//!
//! The `float_to_words`/`to_cardinal_float` float-cardinal path IS ported —
//! see [`LangIt::float_body_f64`] / [`LangIt::float_body_dec`] and the
//! `to_cardinal_float` override. `to_fraction` remains out of scope (a later
//! phase).
//!
//! # Currency
//!
//! `Num2Word_IT` declares its **own** `CURRENCY_FORMS` class attribute, which
//! *shadows* `Num2Word_EUR`'s rather than sharing it. This is the one language
//! family where `PORTING_CURRENCY.md`'s "lang_EUR.py's source is not what runs"
//! warning does **not** apply: `Num2Word_EN.__init__` mutates
//! `Num2Word_EUR.CURRENCY_FORMS` in place, but IT never reads that dict, so
//! none of EN's rewrites (`EUR` → `("euro","euros")`, `GBP` → `("pound",…)`)
//! nor its ~24 added codes leak in. Verified against the live interpreter:
//! IT's table is exactly its 14 source entries, and `EUR` really is
//! `("euro", "euro")`. The corpus agrees — `currency:EUR 2` → "due euro", not
//! "due euros".
//!
//! `CURRENCY_ADJECTIVES` is the opposite case: IT does *not* define one, so it
//! inherits `Num2Word_EUR`'s dict (EN only rebinds `CURRENCY_PRECISION` and
//! mutates `CURRENCY_FORMS`, never the adjectives), and all 16 EUR entries are
//! live here — see [`build_currency_adjectives`].
//!
//! `CURRENCY_PRECISION` is never defined by IT or EUR, and EN *rebinds* rather
//! than mutates it, so IT sees `Num2Word_Base`'s empty `{}` and
//! `.get(code, 100)` always yields 100. That is already the trait default, so
//! `currency_precision` is deliberately **not** overridden. Consequences, both
//! corpus-confirmed:
//!   * **JPY is not zero-decimal here.** `currency:JPY 12.34` →
//!     "dodici yen e trentaquattro sen", not "dodici yen". The `divisor == 1`
//!     branch in `default_to_currency` is unreachable for Italian.
//!   * **KWD/BHD are not 3-decimal here** — they are simply absent from
//!     `CURRENCY_FORMS`, so all 24 of their corpus rows are
//!     `NotImplementedError`.
//!
//! Inherited unchanged from `Num2Word_Base`, hence left at their trait
//! defaults: `to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse`.
//! The defaults route through `&self`, so they pick up the `to_cardinal`
//! override — `to_cheque(1234.56, "EUR")` → "MILLEDUECENTOTRENTAQUATTRO AND
//! 56/100 EURO". `pluralize` comes from `Num2Word_EUR` and *is* overridden,
//! because the trait default raises.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below is wrong-looking but is
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. **`accentuate` strips the accent it just added, one level up.**
//!    `accentuate` accents a word-final "tre" → "tré", but it is re-applied at
//!    every level of `to_cardinal`'s recursion, and its `w[-3:] == "tre"` test
//!    fails against an already-accented "tré". So an inner "tré" that ends up
//!    *word-final* survives, while one that lands mid-word gets silently
//!    de-accented by the `w.replace("tré", "tre")` else-branch:
//!      * `to_cardinal(103)` == "centotré"        (inner "tre" was unaccented)
//!      * `to_cardinal(123)` == "centoventitre"   (inner "ventitré" → de-accented!)
//!      * `to_cardinal(23)`  == "ventitré"
//!    So 103 keeps its accent and 123 loses it. Reproduced by [`accentuate`].
//!
//! 2. **`exponent_length_to_string` indexes `EXPONENT` mod 5**, but
//!    `EXPONENT[0]` is the string "zero" and the table has 11 entries. The
//!    prefix therefore cycles with period 30 and lands on "zero" every fifth
//!    step, so `to_cardinal(10**30)` == "un **zeroilione**" and
//!    `to_cardinal(10**33)` == "un **zeroiliardo**". Worse, the cycle makes
//!    exponents collide outright: `to_cardinal(10**36)` == "un milione",
//!    identical to `to_cardinal(10**6)`. Entries `EXPONENT[5..]`
//!    ("quint", "sest", "sett", "ott", "nov", "dec") are unreachable dead
//!    data. Reproduced by [`exponent_length_to_string`].
//!
//! 3. **`exponent_length_to_string`'s `"mila"` branch is unreachable.**
//!    `exponent_length == 3` requires `len(digits) <= 6`, but
//!    `big_number_to_cardinal` is only called for `number >= 10**6`
//!    (`len(digits) >= 7`). Kept verbatim anyway. The same reasoning makes the
//!    empty-`exponent` path (`int("")` → `ValueError`) unreachable; it is
//!    modelled as `N2WError::Value` rather than a panic, for fidelity.
//!
//! 4. ~~**`to_ordinal_num` raises `AttributeError` for negatives and
//!    floats.**~~ Fixed (gladiaio/num2words2#155/#157). `verify_ordinal`
//!    reaches `raise TypeError(self.errmsg_negord % value)`, but the
//!    `errmsg_*` attributes are only assigned in `Num2Word_Base.__init__`,
//!    which `Num2Word_IT` never calls, so Python died with `AttributeError`
//!    on the lookup. `to_ordinal_num` now agrees with `to_ordinal`, which
//!    deliberately accepts negatives (item 5): a negative is written with its
//!    sign (`-1` → "-1"), and only a fractional value is rejected, with Base's
//!    `errmsg_floatord` `TypeError`.
//!
//! 5. **`to_ordinal` of a negative prefixes "meno "** and recurses, so
//!    `to_ordinal(-1)` == "meno primo" rather than raising, unlike most
//!    languages. This is deliberate upstream (`test_it.py::test_negative`
//!    pins it), so it is kept, and `to_ordinal_num` follows it (item 4).
//!
//! 6. **`to_currency` silently ignores `adjective=True` for `int` input.**
//!    `Num2Word_IT.to_currency` reimplements the integer branch and simply
//!    never consults `CURRENCY_ADJECTIVES`, while the float branch delegates to
//!    `Num2Word_Base.to_currency`, which does apply it. So the flag flips
//!    behaviour on the *type* of the argument:
//!      * `to_currency(1,   "USD", adjective=True)` == "uno dollaro"
//!      * `to_currency(1.0, "USD", adjective=True)` == "uno US dollaro e zero
//!        centesimi"
//!    Both verified against the live interpreter. Reproduced in
//!    [`LangIt::to_currency`] by dropping `adjective` on the `Int` arm only.
//!
//! 7. **`CURRENCIES_UNA = "GBP"` is dead data.** `lang_IT.py` defines it,
//!    plainly intending "una sterlina" for the feminine noun, but nothing ever
//!    reads it. So Italian emits the ungrammatical masculine article:
//!    `to_currency(1, "GBP")` == "**uno** sterlina", and `1.0` gives "uno
//!    sterlina e zero penny". The corpus enshrines both. Likewise
//!    `to_currency(1000000, "EUR")` == "un milione euro" — no "di".
//!
//! Note on the `phonetic_contraction` "diciotto" guard: that one is *not* a
//! bug but a deliberate fix in this fork (the `"io"` → `"o"` rule would
//! otherwise mangle "centodiciotto" into "centodicotto"). It is ported as-is.
//!
//! # Python semantics that matter here
//!
//! All slicing is **character**-based, and the strings genuinely contain "é":
//! `to_cardinal(23)[:-1]` must yield "ventitr", not a byte-truncated mess. All
//! slicing below therefore goes through [`drop_last_chars`] / [`last_chars`],
//! never byte offsets. Python's `s[-3:]` on a shorter string returns the whole
//! string rather than raising, which [`last_chars`] mirrors via
//! `saturating_sub`. `str.split()` with no argument splits on whitespace runs
//! and drops empties — `split_whitespace` matches it.

use std::sync::OnceLock;
use crate::base::{check_maxval, floatord_error, pow10_big, py_num_str, Lang, N2WError, Result};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

const ZERO: &str = "zero";

/// `MINUS_PREFIX_WORD`. Note the trailing space — it is the separator.
const MINUS_PREFIX_WORD: &str = "meno ";

const CARDINAL_WORDS: [&str; 20] = [
    ZERO,
    "uno",
    "due",
    "tre",
    "quattro",
    "cinque",
    "sei",
    "sette",
    "otto",
    "nove",
    "dieci",
    "undici",
    "dodici",
    "tredici",
    "quattordici",
    "quindici",
    "sedici",
    "diciassette",
    "diciotto",
    "diciannove",
];

const ORDINAL_WORDS: [&str; 20] = [
    ZERO,
    "primo",
    "secondo",
    "terzo",
    "quarto",
    "quinto",
    "sesto",
    "settimo",
    "ottavo",
    "nono",
    "decimo",
    "undicesimo",
    "dodicesimo",
    "tredicesimo",
    "quattordicesimo",
    "quindicesimo",
    "sedicesimo",
    "diciassettesimo",
    "diciottesimo",
    "diciannovesimo",
];

/// `EXPONENT_PREFIXES` (aliased as `EXPONENT` in Python). Index 0 is the
/// literal "zero", and indices 5..=10 are unreachable — see module bug 2.
const EXPONENT: [&str; 11] = [
    ZERO, "m", "b", "tr", "quadr", "quint", "sest", "sett", "ott", "nov", "dec",
];

/// Python `s[:-n]`, character-based. `s[:-n]` on a string of `<= n` chars
/// yields "" rather than raising.
fn drop_last_chars(s: &str, n: usize) -> String {
    let cs: Vec<char> = s.chars().collect();
    if cs.len() <= n {
        String::new()
    } else {
        cs[..cs.len() - n].iter().collect()
    }
}

/// Python `s[-n:]`, character-based. On a string of `< n` chars Python returns
/// the whole string, hence `saturating_sub`.
fn last_chars(s: &str, n: usize) -> String {
    let cs: Vec<char> = s.chars().collect();
    cs[cs.len().saturating_sub(n)..].iter().collect()
}

/// `omitt_if_zero` (Python's spelling of the name, typo included).
fn omitt_if_zero(number_to_string: &str) -> &str {
    if number_to_string == ZERO {
        ""
    } else {
        number_to_string
    }
}

/// `accentuate`: accent a word-final "tre", and strip "tré" everywhere else.
///
/// See module bug 1 — the `last_chars(w, 3) == "tre"` test is checked against
/// the *original* word (so an already-accented "tré" fails it and falls into
/// the de-accenting else-branch), while the `[:-3]` slice is taken from the
/// *replaced* string. Both details are load-bearing.
fn accentuate(string: &str) -> String {
    string
        .split_whitespace()
        .map(|w| {
            // We shouldn't accentuate a single "tre": it has to be a composite
            // word — hence the `> 3` length test (in *characters*).
            if last_chars(w, 3) == "tre" && w.chars().count() > 3 {
                drop_last_chars(&w.replace("tré", "tre"), 3) + "tré"
            } else {
                w.replace("tré", "tre")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `phonetic_contraction`: elide colliding vowels at compound seams.
///
/// The `"diciotto"` early-return is a deliberate fix in this fork, not a bug:
/// without it the `"io"` → `"o"` rule would turn "centodiciotto" into
/// "centodicotto". The remaining rules are applied globally and *in order* —
/// `.replace()` in both languages rewrites every occurrence, and each stage
/// sees the previous stage's output.
fn phonetic_contraction(string: &str) -> String {
    if string.contains("diciotto") {
        return string.to_string();
    }

    string
        .replace("oo", "o") // ex. "centootto"
        .replace("ao", "o") // ex. "settantaotto"
        .replace("io", "o") // ex. "ventiotto"
        .replace("au", "u") // ex. "trentauno"
        .replace("iu", "u") // ex. "ventiunesimo"
}

/// `exponent_length_to_string`. Always called with a length of the form 3n.
///
/// The `% 5` and the `"zero"` at `EXPONENT[0]` are module bug 2; the
/// `== 3` branch is module bug 3 (unreachable). Both ported verbatim.
fn exponent_length_to_string(exponent_length: usize) -> String {
    let prefix = EXPONENT[(exponent_length / 6) % 5];

    if exponent_length == 3 {
        "mila".to_string()
    } else if exponent_length % 6 == 0 {
        format!("{}ilione", prefix)
    } else {
        format!("{}iliardo", prefix)
    }
}

/// Python raised `ValueError` (`int("")`). Unreachable in practice — see
/// module bug 3 — but modelled rather than panicked on.
fn value_error(msg: &str) -> N2WError {
    N2WError::Value(msg.to_string())
}

/// `abs(Decimal(str(f)).as_tuple().exponent)` for an f64 — the fractional-digit
/// count of `repr(f)`. Mirrors the private `floatpath::float_repr_precision`,
/// reproduced here because the currency fractional-cents entry
/// ([`LangIt::cardinal_from_decimal`]) reconstructs `float(right)`'s precision
/// before handing off to the float path. Rust's `{}` for f64 is shortest
/// round-trip like Python's `repr`, so the count of digits after the point
/// matches (exponent-notation reprs carry no `'.'` and yield 0, as in Python's
/// exponent-0 tuple).
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// `Num2Word_IT.CURRENCY_FORMS`, transcribed from the class body.
///
/// Unlike the 16 classes that read `Num2Word_EUR`'s mutated dict, IT declares
/// its own attribute, so this really is the source literal — including
/// `EUR: ("euro", "euro")`, which EN would have rewritten to `("euro",
/// "euros")` had IT shared the dict. Verified against the live interpreter.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // GENERIC_DOLLARS / GENERIC_CENTS in lang_IT.py.
    const DOLLARS: [&str; 2] = ["dollaro", "dollari"];
    const CENTS: [&str; 2] = ["centesimo", "centesimi"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &CENTS));
    m.insert("USD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("CAD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("AUD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("HKD", CurrencyForms::new(&DOLLARS, &CENTS));
    m.insert("GBP", CurrencyForms::new(&["sterlina", "sterline"], &["penny", "penny"]));
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"]));
    m.insert("CHF", CurrencyForms::new(&["franco", "franchi"], &CENTS));
    m.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
    m.insert("INR", CurrencyForms::new(&["rupia", "rupie"], &["paisa", "paise"]));
    m.insert("RUB", CurrencyForms::new(&["rublo", "rubli"], &["copeco", "copechi"]));
    m.insert("KRW", CurrencyForms::new(&["won", "won"], &["jeon", "jeon"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesos"], &CENTS));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited verbatim — IT defines none of
/// its own, and EN never mutates this dict (it only rewrites `CURRENCY_FORMS`
/// and *rebinds* `CURRENCY_PRECISION`), so all 16 EUR entries are live.
///
/// Eight of them (BYN, EEK, NOK, RON, HUF, ISK, UZS, SAR) name codes that are
/// absent from IT's `CURRENCY_FORMS`, so the forms lookup raises
/// NotImplementedError long before the adjective is consulted: dead data,
/// ported for fidelity. Of the rest, only the float path can reach them at all
/// — see bug 6.
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

pub struct LangIt {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl LangIt {
    pub fn new() -> Self {
        LangIt {
            // Built once here, never per call: `to_currency` only ever reads
            // these, and rebuilding them on each call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `tens_to_cardinal`. Only ever called with `20 <= number < 100`, so
    /// `tens` is 2..=9 and both table lookups are in range.
    fn tens_to_cardinal(&self, number: u32) -> String {
        let tens = (number / 10) as usize;
        let units = (number % 10) as usize;
        // STR_TENS = {2: "venti", 3: "trenta", 4: "quaranta", 6: "sessanta"};
        // the script extrapolates the rest from the base forms.
        let prefix = match tens {
            2 => "venti".to_string(),
            3 => "trenta".to_string(),
            4 => "quaranta".to_string(),
            6 => "sessanta".to_string(),
            _ => drop_last_chars(CARDINAL_WORDS[tens], 1) + "anta",
        };
        let postfix = omitt_if_zero(CARDINAL_WORDS[units]);
        phonetic_contraction(&format!("{}{}", prefix, postfix))
    }

    /// `hundreds_to_cardinal`. Only ever called with `100 <= number < 1000`.
    fn hundreds_to_cardinal(&self, number: u32) -> Result<String> {
        let hundreds = (number / 100) as usize;
        let mut prefix = "cento".to_string();
        if hundreds != 1 {
            prefix = format!("{}{}", CARDINAL_WORDS[hundreds], prefix);
        }
        // Recurses through the *override*, so the inner value is accentuated.
        let inner = self.cardinal(&BigInt::from(number % 100))?;
        let postfix = omitt_if_zero(&inner);
        Ok(phonetic_contraction(&format!("{}{}", prefix, postfix)))
    }

    /// `thousands_to_cardinal`. Only ever called with `1000 <= number < 10**6`.
    ///
    /// Note the absence of `phonetic_contraction` here: Python's comment says
    /// "mille" and "mila" don't need any. Hence `to_cardinal(2008)` is
    /// "duemilaotto", *not* "duemilotto".
    fn thousands_to_cardinal(&self, number: u32) -> Result<String> {
        let thousands = number / 1000;
        let prefix = if thousands == 1 {
            "mille".to_string()
        } else {
            format!("{}mila", self.cardinal(&BigInt::from(thousands))?)
        };
        let inner = self.cardinal(&BigInt::from(number % 1000))?;
        let postfix = omitt_if_zero(&inner);
        Ok(format!("{}{}", prefix, postfix))
    }

    /// `big_number_to_cardinal`. Only ever called with `number >= 10**6`, and
    /// `number` is genuinely unbounded here (`BigInt`, never a fixed-width
    /// cast) up to the maxval (10^65) guard.
    fn big_number_to_cardinal(&self, number: &BigInt) -> Result<String> {
        // Python: digits = [c for c in str(int(number))]. `number` is known
        // non-negative at this point, so no sign leaks into the digit list.
        let digits: Vec<char> = number.to_string().chars().collect();
        let length = digits.len();
        check_maxval(number, maxval_ceiling())?;
        // This is how many digits come before the "illion" term.
        //   cento miliardi => 3
        //   dieci milioni => 2
        //   un miliardo => 1
        // `length % 3 or 3`: Python's `or` yields 3 when the modulo is 0.
        let predigits = if length % 3 != 0 { length % 3 } else { 3 };
        let multiplier: String = digits[..predigits].iter().collect();
        let exponent: String = digits[predigits..].iter().collect();
        // Default infix string: "milione", "biliardo", "sestilione", ecc.
        let mut infix = exponent_length_to_string(exponent.len());

        // Python compares the *list* `multiplier == ["1"]`, i.e. exactly one
        // digit and it is '1' — equivalent to this string compare.
        let prefix = if multiplier == "1" {
            "un ".to_string()
        } else {
            let p = self.cardinal(
                &multiplier
                    .parse::<BigInt>()
                    .map_err(|_| value_error("invalid literal for int() with base 10"))?,
            )?;
            // Plural form: "milione" -> " milioni".
            infix = format!(" {}i", drop_last_chars(&infix, 1));
            p
        };

        // Python: `if set(exponent) != set("0")`, read as "does the value of
        // exponent equal 0?". set("0") == {'0'}, so the two are equal exactly
        // when `exponent` is non-empty and every char is '0'. An empty
        // `exponent` takes the *postfix* branch and feeds "" to int() —
        // unreachable (see module bug 3) but modelled as ValueError.
        let exponent_is_zero = !exponent.is_empty() && exponent.chars().all(|c| c == '0');
        let postfix = if !exponent_is_zero {
            let p = self.cardinal(
                &exponent
                    .parse::<BigInt>()
                    .map_err(|_| value_error("invalid literal for int() with base 10: ''"))?,
            )?;
            if p.contains(" e ") {
                infix.push_str(", ");
            } else {
                infix.push_str(" e ");
            }
            p
        } else {
            String::new()
        };

        Ok(format!("{}{}{}", prefix, infix, postfix))
    }

    /// `Num2Word_IT.to_cardinal`, integer path only.
    ///
    /// Python's negative branch recurses (which accentuates the positive part),
    /// prepends "meno ", and then `return string if number < 0 else
    /// accentuate(string)` deliberately skips a second `accentuate` pass. The
    /// early return below is exactly equivalent — and the skip matters, because
    /// a second pass would be *lossy*, not idempotent (see module bug 1).
    fn cardinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            let positive_part = self.cardinal(&-number)?;
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, positive_part));
        }
        // Python's `elif int(number) != number` float branch is out of scope:
        // integer input only.
        let string = if number < &BigInt::from(20) {
            // Safe: 0 <= number < 20.
            CARDINAL_WORDS[number.to_usize().expect("0 <= number < 20")].to_string()
        } else if number < &BigInt::from(100) {
            self.tens_to_cardinal(number.to_u32().expect("20 <= number < 100"))
        } else if number < &BigInt::from(1000) {
            self.hundreds_to_cardinal(number.to_u32().expect("100 <= number < 1000"))?
        } else if number < &BigInt::from(1_000_000) {
            self.thousands_to_cardinal(number.to_u32().expect("1000 <= number < 10**6"))?
        } else {
            self.big_number_to_cardinal(number)?
        };
        Ok(accentuate(&string))
    }

    /// `Num2Word_IT.to_ordinal`, integer path only.
    fn ordinal(&self, number: &BigInt) -> Result<String> {
        if number.is_negative() {
            // No verify_ordinal call here — see module bug 5.
            return Ok(format!("{}{}", MINUS_PREFIX_WORD, self.ordinal(&-number)?));
        }
        // Python's float branch is out of scope: integer input only.

        // `number` is non-negative, so Rust's truncating `%` agrees with
        // Python's floor `%` and `tens` lands in 0..=99.
        let tens = (number % BigInt::from(100)).to_u32().expect("0 <= tens < 100");
        // Italian grammar is poorly defined here ¯\_(ツ)_/¯:
        //   centodecimo VS centodieciesimo VS centesimo decimo?
        let is_outside_teens = !(10 < tens && tens < 20);

        if number < &BigInt::from(20) {
            Ok(ORDINAL_WORDS[number.to_usize().expect("0 <= number < 20")].to_string())
        } else if is_outside_teens && tens % 10 == 3 {
            // Gets rid of the accent: "ventitré" -> "ventitr" + "eesimo".
            Ok(drop_last_chars(&self.cardinal(number)?, 1) + "eesimo")
        } else if is_outside_teens && tens % 10 == 6 {
            Ok(self.cardinal(number)? + "esimo")
        } else {
            let mut string = drop_last_chars(&self.cardinal(number)?, 1);
            // "duemila" -> "duemil" -> "duemill" -> "duemillesimo".
            if last_chars(&string, 3) == "mil" {
                string.push('l');
            }
            Ok(string + "esimo")
        }
    }

    /// `Num2Word_IT.float_to_words(float_number, ordinal=True)` for a
    /// **non-negative, non-integral f64** — the ordinal float grammar:
    /// `to_ordinal(int(x))` + " virgola " + one cardinal per repr digit.
    /// Unlike the cardinal entry, `to_ordinal` returns this **without** a
    /// trailing `accentuate` pass.
    fn float_ordinal_body_f64(&self, x: f64, precision: u32) -> Result<String> {
        let s = format!("{:.*}", precision as usize, x);
        let (int_str, frac) = match s.split_once('.') {
            Some(p) => p,
            None => return Err(N2WError::Index("list index out of range".into())),
        };
        let pre: BigInt = int_str
            .parse()
            .map_err(|_| value_error("invalid literal for int() with base 10"))?;
        let prefix = self.ordinal(&pre)?;

        let mut parts: Vec<String> = Vec::new();
        for c in frac.chars() {
            let d = c
                .to_digit(10)
                .ok_or_else(|| value_error("invalid literal for int() with base 10"))?;
            parts.push(self.cardinal(&BigInt::from(d))?);
        }
        Ok(format!("{} virgola {}", prefix, parts.join(" ")))
    }

    /// `Num2Word_IT.float_to_words` for the **f64** arm, applied to a
    /// **non-negative** value (the caller strips the sign).
    ///
    /// ```python
    /// def float_to_words(self, float_number, ordinal=False):
    ///     prefix = self.to_cardinal(int(float_number))
    ///     float_part = str(float_number).split(".")[1]
    ///     postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
    ///     return prefix + Num2Word_IT.FLOAT_INFIX_WORD + postfix
    /// ```
    ///
    /// This never touches `base.float2tuple`: the fractional digits come
    /// straight out of `str(float_number)`, one character at a time, so `2.675`
    /// renders "sei sette cinque" (6 7 5) because `str(2.675)` is literally
    /// `"2.675"` — no `< 0.01` artefact heuristic, no banker's rounding.
    ///
    /// The `int(number) != number` gate in `to_cardinal` means an
    /// integer-valued float never enters `float_to_words`; it falls through to
    /// the integer branches instead (so `1.0` → "uno", not "uno virgola zero").
    ///
    /// `precision` is the repr-derived fractional-digit count. In the normal
    /// range it equals `len(str(float_number).split(".")[1])`, so a fixed format
    /// to that many places reproduces the exact repr digits (verified
    /// byte-for-byte against the interpreter over the corpus float set; `{:.N}`
    /// rounds the exact binary value half-to-even, agreeing with CPython where a
    /// shortest-repr reconstruction would not). Exponent-notation reprs
    /// (`abs < 1e-4`) are out of corpus scope — see the port report.
    fn float_body_f64(&self, x: f64, precision: u32) -> Result<String> {
        // `elif int(number) != number` — integer-valued floats skip float_to_words.
        if x == x.trunc() {
            // `int(x)`: the exact integer value of the float. `{:.0}` on an
            // integer-valued f64 is exact (no tie to resolve).
            let int_part: BigInt = format!("{:.0}", x)
                .parse()
                .map_err(|_| value_error("invalid literal for int() with base 10"))?;
            // `self.cardinal` already applies the final `accentuate` pass, which
            // is exactly what `to_cardinal` does for the integer branches.
            return self.cardinal(&int_part);
        }

        // Reconstruct `str(float_number)`: shortest round-trip == fixed format to
        // the repr-derived precision (see the doc note above).
        let s = format!("{:.*}", precision as usize, x);
        let (int_str, frac) = match s.split_once('.') {
            Some(p) => p,
            // Python: `str(float_number).split(".")[1]` with no '.' → IndexError.
            None => return Err(N2WError::Index("list index out of range".into())),
        };

        // prefix = to_cardinal(int(float_number)); `int_str` equals int(x) for
        // the fixed form the reconstruction produces (int() truncates toward
        // zero on a non-negative).
        let pre: BigInt = int_str
            .parse()
            .map_err(|_| value_error("invalid literal for int() with base 10"))?;
        let prefix = self.cardinal(&pre)?;

        // postfix = " ".join(to_cardinal(int(c)) for c in float_part)
        let mut parts: Vec<String> = Vec::new();
        for c in frac.chars() {
            // Python `int(c)`: a non-digit (e.g. 'e' from exponent notation)
            // raises ValueError. Out of corpus scope, reproduced as the variant.
            let d = c
                .to_digit(10)
                .ok_or_else(|| value_error("invalid literal for int() with base 10"))?;
            parts.push(self.cardinal(&BigInt::from(d))?);
        }

        // prefix + FLOAT_INFIX_WORD (" virgola ") + postfix, then the outer
        // `accentuate` pass `to_cardinal` applies for non-negative input. The
        // prefix was accentuated once inside `self.cardinal`; this second pass
        // reproduces Python's double application (bug 1's map is not idempotent,
        // so the repeat is load-bearing, not redundant).
        let raw = format!("{} virgola {}", prefix, parts.join(" "));
        Ok(accentuate(&raw))
    }

    /// `Num2Word_IT.float_to_words` for the **Decimal** arm, applied to a
    /// **non-negative** value.
    ///
    /// A `Decimal` is not a `float`, but IT's `to_cardinal` gates on
    /// `int(number) != number` (not `isinstance(number, float)`), so a Decimal
    /// flows through the *same* `float_to_words` — reading `str(Decimal)`, which
    /// preserves trailing zeros and exact digits: `Decimal("1.10")` →
    /// "uno virgola uno zero", and `Decimal("2.00")` is integer-valued →
    /// "due".
    ///
    /// The digit split is reconstructed from the coefficient at scale
    /// `precision` rather than from `str(Decimal)` directly, so it is
    /// independent of `BigDecimal`'s Display. For the fixed-notation Decimals in
    /// scope this equals `int(x)` and `str(x).split(".")[1]` exactly. Scientific
    /// -notation Decimals (adjusted exponent `< -6`, or exponent `> 0`) are out
    /// of corpus scope — see the port report.
    fn float_body_dec(&self, x: &BigDecimal, precision: u32) -> Result<String> {
        // `x` is non-negative; force scale = precision (exact padding, no
        // rounding, since the value carries at most `precision` fractional
        // digits) so the coefficient's low `precision` digits are the fraction.
        let scaled = x.with_scale(precision as i64);
        let coeff = scaled.as_bigint_and_exponent().0; // >= 0
        let divisor = BigInt::from(10).pow(precision);
        let int_part = &coeff / &divisor;
        let frac_val = &coeff % &divisor;

        // `int(number) == number` — an integer-valued Decimal skips float_to_words.
        if frac_val.is_zero() {
            return self.cardinal(&int_part);
        }

        let prefix = self.cardinal(&int_part)?;

        // Zero-pad to `precision` digits: str(Decimal) keeps the full fractional
        // field, so "0.001" → "001", not "1".
        let frac_raw = frac_val.to_string();
        let frac_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(frac_raw.len())),
            frac_raw
        );

        let mut parts: Vec<String> = Vec::new();
        for c in frac_str.chars() {
            // Every char is a decimal digit by construction.
            let d = c.to_digit(10).expect("frac_str is all decimal digits");
            parts.push(self.cardinal(&BigInt::from(d))?);
        }

        let raw = format!("{} virgola {}", prefix, parts.join(" "));
        Ok(accentuate(&raw))
    }
}

impl Default for LangIt {
    fn default() -> Self {
        LangIt::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `big_number_to_cardinal`
/// names numbers of up to 65 digits, so 10^65 and above raise
/// `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(65))
}

impl Lang for LangIt {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

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
        " e"
    }

    // cards/maxval/merge stay at their trait defaults: Python never builds
    // self.cards for this class, so splitnum/clean/merge are unreachable and
    // there is no MAXVAL overflow check. See the module docs.

    /// `Num2Word_IT.setup` sets `self.negword = "meno"`.
    fn negword(&self) -> &str {
        "meno"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.cardinal(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.ordinal(value)
    }

    /// `str(int(value))`. Negatives are accepted, as `to_ordinal` accepts
    /// them (module bug 4, #155).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(value.to_string())
    }

    /// `Num2Word_IT.to_cardinal(<float | Decimal>)` — the float/Decimal path.
    ///
    /// IT does **not** override Python's `to_cardinal_float`; instead its
    /// overridden `to_cardinal` handles non-integers inline:
    ///
    /// ```python
    /// def to_cardinal(self, number):
    ///     if number < 0:
    ///         positive_part = self.to_cardinal(-number)
    ///         string = Num2Word_IT.MINUS_PREFIX_WORD + positive_part
    ///     elif int(number) != number:
    ///         string = self.float_to_words(number)
    ///     ...
    ///     return string if number < 0 else accentuate(string)
    /// ```
    ///
    /// So the sign branch recurses on the positive part, prepends "meno ", and
    /// returns **without** the trailing `accentuate` (the positive body already
    /// got it). This override reproduces exactly that: render the non-negative
    /// body, then prepend the sign.
    ///
    /// `precision_override` (the `precision=` kwarg) is ignored, exactly as
    /// Python ignores it here: IT never calls `Num2Word_Base.__init__`, so it
    /// has no `self.precision`, the dispatcher's `hasattr(converter,
    /// "precision")` guard is false, and `float_to_words` reads `str(number)`
    /// regardless (confirmed live: `num2words(2.675, lang="it", precision=1)`
    /// is unchanged).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let neg = value.is_negative();
        let body = match value {
            FloatValue::Float { value, precision } => {
                self.float_body_f64(value.abs(), *precision)?
            }
            FloatValue::Decimal { value, precision } => {
                self.float_body_dec(&value.abs(), *precision)?
            }
        };
        if neg {
            Ok(format!("{}{}", MINUS_PREFIX_WORD, body))
        } else {
            Ok(body)
        }
    }

    /// `to_ordinal(float/Decimal)` — `Num2Word_IT.to_ordinal`'s full routing.
    ///
    /// ```python
    /// if number < 0:
    ///     return Num2Word_IT.MINUS_PREFIX_WORD + self.to_ordinal(-number)
    /// if isinstance(number, float) and not number.is_integer():
    ///     return self.float_to_words(number, ordinal=True)
    /// number = int(number)
    /// ...
    /// ```
    ///
    /// The float branch is gated on `isinstance(float)`, so a fractional
    /// `Decimal` skips it and is silently truncated by `int()` —
    /// `Decimal("1.5")` → "primo" while `1.5` → "primo virgola cinque".
    /// `-0.0` is not `< 0`, so it truncates to `ORDINAL_WORDS[0]` == "zero".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let strictly_negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if strictly_negative {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: -*value,
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: -value.clone(),
                    precision: *precision,
                },
            };
            return Ok(format!(
                "{}{}",
                MINUS_PREFIX_WORD,
                self.ordinal_float_entry(&abs)?
            ));
        }
        match value {
            FloatValue::Float { value: v, precision } if v.fract() != 0.0 => {
                self.float_ordinal_body_f64(*v, *precision)
            }
            FloatValue::Float { value: v, .. } => {
                let i = BigInt::from_f64(v.trunc()).ok_or_else(|| {
                    value_error("cannot convert float infinity to integer")
                })?;
                self.ordinal(&i)
            }
            FloatValue::Decimal { value: d, .. } => {
                // int(Decimal) truncates toward zero.
                self.ordinal(&d.with_scale(0).as_bigint_and_exponent().0)
            }
        }
    }

    /// `to_ordinal_num(float/Decimal)`: `str(int(value))` for a whole value,
    /// negatives included as in `to_ordinal`. A fractional value raises Base's
    /// `errmsg_floatord` `TypeError` — Python meant to, but died with
    /// `AttributeError` on the missing attribute (module bug 4, #157).
    fn ordinal_num_float_entry(&self, value: &FloatValue, _repr_str: &str) -> Result<String> {
        value
            .as_whole_int()
            .map(|i| i.to_string())
            .ok_or_else(|| floatord_error(py_num_str(value)))
    }

    /// `Num2Word_IT.to_fraction` (issue #584): Italian '-o → -i' plurals, the
    /// idiomatic "mezzo"/"mezzi" for halves, and "un" (not "uno") as the
    /// unit numerator.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator == &BigInt::one() || numerator.is_zero() {
            return self.cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();

        let den_word = if abs_d == BigInt::from(2) {
            if abs_n.is_one() { "mezzo" } else { "mezzi" }.to_string()
        } else {
            let mut den = self.ordinal(&abs_d)?;
            // Italian -o → -i for plural masculine nouns.
            if !abs_n.is_one() && den.ends_with('o') {
                den.pop();
                den.push('i');
            }
            den
        };
        let num_word = if abs_n.is_one() {
            "un".to_string()
        } else {
            self.cardinal(&abs_n)?
        };
        let sign = if is_negative { "meno " } else { "" };
        Ok(format!("{}{} {}", sign, num_word, den_word))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`. `Decimal("NaN")`
    /// parses fine; IT's `to_cardinal` then dies on the very first comparison
    /// (`number < 0`) with `decimal.InvalidOperation`. The binding otherwise
    /// maps `ParsedNumber::NaN` to `int(NaN)`'s ValueError, so the
    /// InvalidOperation must be raised here. Infinity keeps the default
    /// routing: `int(Decimal("Infinity"))` → OverflowError, which the binding
    /// already produces.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    /// Currency's fractional-cents entry point. `base.to_currency` computes
    /// `cents_str = self.to_cardinal(float(right))` (base.py line 476) — it
    /// routes the fractional cents through IT's own float path, so this must go
    /// through the overridden [`to_cardinal_float`](Self::to_cardinal_float),
    /// **not** the base "(.)"-pointword path the trait default would use:
    ///
    /// * pure-Python IT: `to_currency(1.011, "USD")` → "uno dollaro e uno
    ///   **virgola** uno centesimi"
    /// * trait default:  "uno dollaro e uno **(.)** uno centesimi"
    ///
    /// Unreachable from the frozen corpus (no IT currency row has fractional
    /// cents), verified against the pure-Python interpreter.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        // Python: `float(right)`. Reproduce the f64 cast and its repr-derived
        // precision, then hand off to the float path.
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        let precision = float_repr_precision(f);
        self.to_cardinal_float(&FloatValue::Float { value: f, precision }, None)
    }

    // to_year is inherited unchanged from Num2Word_Base
    // (`return self.to_cardinal(value)`); the trait default does exactly that
    // and dispatches back through the override above.

    // ---- currency -------------------------------------------------------
    //
    // IT overrides `to_currency` (integer branch only) and inherits
    // `pluralize` from `Num2Word_EUR`. Everything else on the currency path —
    // `to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse` — comes
    // from `Num2Word_Base` unchanged, and the trait defaults already mirror it.
    //
    // `currency_precision` is *deliberately* not overridden: IT's
    // CURRENCY_PRECISION resolves to Base's empty `{}`, so `.get(code, 100)`
    // is always 100 — already the trait default. See the module docs for why
    // that makes JPY 2-decimal and KWD/BHD unreachable here.
    //
    // `cardinal_from_decimal` IS overridden (below), because IT's float path is
    // now ported. `Num2Word_IT` overrides `float_to_words` with
    // `FLOAT_INFIX_WORD = " virgola "` and never consults `pointword` — indeed
    // it has no `pointword` attribute at all, since it never calls
    // `Num2Word_Base.__init__`, so the trait's "(.)" default is a phantom here.
    // The float path and the currency fractional-cents entry are overridden
    // together, as required. Unreachable from the frozen corpus: no IT row has
    // fractional cents (every arg is integral at 2 decimals), so
    // `default_to_currency`'s `has_fractional_cents` branch never fires — the
    // override is verified against the pure-Python interpreter instead.

    fn lang_name(&self) -> &str {
        "Num2Word_IT"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Reached only from the float path (`default_to_currency`); IT's integer
    /// branch open-codes the same rule inline instead of calling this. Python
    /// indexes the tuple directly, so a one-form entry with `n != 1` would
    /// raise IndexError — every entry in IT's table has exactly two forms, so
    /// that is unreachable, but it is mapped to `Index` rather than panicking
    /// in case the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_IT.to_currency`.
    ///
    /// Python special-cases `isinstance(val, int)` with a hand-rolled branch
    /// and delegates everything else to `Num2Word_Base.to_currency` via
    /// `super()`. The int branch differs from Base's in three ways, all
    /// preserved here:
    ///
    ///   1. It ignores `adjective` entirely (module bug 6).
    ///   2. It hardcodes the literal `"meno"` instead of reading
    ///      `self.negword.strip()`. Same string either way, so unobservable —
    ///      transcribed literally regardless.
    ///   3. It calls `self.to_cardinal` directly rather than
    ///      `self._money_verbose`. Base's `_money_verbose` *is*
    ///      `self.to_cardinal`, so again unobservable.
    ///
    /// The `except (KeyError, AttributeError)` fallback re-enters Base's
    /// `to_currency`, which repeats the same failed lookup and turns it into
    /// `NotImplementedError` — so an unknown code raises identically on both
    /// arms. (`AttributeError` cannot fire: `CURRENCY_FORMS` always exists.)
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait hands us None when the caller omitted separator=; resolve it to
        // this language's own default (" e") before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        // `isinstance(val, int)` — a true Python int. A float that happens to
        // be whole (1.0) is NOT this arm and still renders cents.
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                // KeyError -> super().to_currency(...), which raises
                // NotImplementedError from its own identical lookup.
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };
            let cr1 = &forms.unit;

            // Python: minus_str = "meno" if val < 0 else "" — the literal, not
            // negword. `adjective` is not consulted at all here (bug 6).
            let minus = v.is_negative();
            let abs_val = v.abs();
            let money_str = self.cardinal(&abs_val)?;

            // Python: cr1[0] if abs_val == 1 else (cr1[1] if len(cr1) > 1 else
            // cr1[0]). Both arms index a tuple, so an empty one would raise
            // IndexError; IT's entries all have two forms, so unreachable.
            let currency_str = if abs_val.is_one() {
                cr1.first()
            } else {
                cr1.get(1).or_else(|| cr1.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            let result = if minus {
                format!("meno {} {}", money_str, currency_str)
            } else {
                format!("{} {}", money_str, currency_str)
            };
            // Python's trailing .strip(); a no-op for every real entry, kept
            // because it is what the source does.
            return Ok(result.trim().to_string());
        }

        // Floats/Decimals: `super().to_currency(...)` == Num2Word_Base's, which
        // pluralizes via Num2Word_EUR and *does* honour `adjective`.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
