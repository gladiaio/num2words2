//! Port of `lang_PL.py` (Polish).
//!
//! Shape: **self-contained**. `Num2Word_PL` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` table
//! ends at 10^63 (see below); Python raised `KeyError` from 10^66 up. Fixed
//! (gladiaio/num2words2#159): `maxval()` is 10^66 and `_int2word` /
//! `to_ordinal` raise `OverflowError` for `abs(n) >= 10**66` up front.
//!
//! Inherited from `Num2Word_Base` (unchanged by PL, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`
//!   * `to_year(value)        -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following are all wrong-looking but are
//! exactly what Python emits, verified against the interpreter:
//!
//! 1. `HUNDREDS` tuples hold **one** form, yet `last_fragment_to_ordinal`
//!    indexes them with `level` (0 or 1). `HUNDREDS[n3][1]` therefore raises
//!    `IndexError: tuple index out of range` — e.g. `to_ordinal(110000)`,
//!    `to_ordinal(120000)`, `to_ordinal(210000)` all blow up. Modelled by
//!    [`hundreds_at`].
//! 2. The `else` arm of `last_fragment_to_ordinal` appends ordinal words but
//!    `to_ordinal` then concatenates the level suffix **without a separator**,
//!    so `to_ordinal(123000)` == "sto dwudziesty trzecitysięczny".
//! 3. Four table typos, fixed upstream by PR #660 and adopted here:
//!    `TWENTIES_ORDINALS[3][1]` "trzydiesto" → "trzydziesto",
//!    `HUNDREDS_ORDINALS[8][1]` "ośiemset" → "osiemset",
//!    `HUNDREDS_ORDINALS[5][1]` "pięcset" → "pięćset", and
//!    `prefixes_ordinal[3]` "milairdowy" → "miliardowy" — so
//!    `to_ordinal(10**9)` == "miliardowy".
//! 4. `to_ordinal(0)` returned an `IndexError` upstream (the `while last == 0`
//!    loop pops the only fragment, then indexes an empty list); PR #668 fixes
//!    it to "zerowy", adopted here.
//! 5. ~~`to_ordinal(n)` for **every** negative `n` raises
//!    `ValueError: invalid literal for int() with base 10: '-'`~~ — fixed
//!    (gladiaio/num2words2#155). The minus sign survived into `splitbyx` /
//!    `get_digits` and died in `int()`. The port now raises Base's
//!    `errmsg_negord` `TypeError`, like most languages, and a fractional
//!    float/Decimal raises `errmsg_floatord` instead of an empty
//!    `NotImplementedError`; a whole float (`2.0`) converts like its int.
//! 6. `to_currency`'s integer path second-guesses its own `pluralize`: any
//!    value above 1 whose cardinal *ends with* "jeden" is forced to form 1
//!    instead. Polish grammar wants form 2 there ("dwadzieścia jeden
//!    złotych"), so Python emits the ungrammatical "dwadzieścia jeden
//!    złote". See [`LangPl::to_currency`].
//!
//! # Error variants
//!
//! Polish raises three exception types that the four-way mapping in
//! PORTING.md (Overflow/Type/NotImplemented/ZeroDivision) does not cover:
//! `ValueError`, `IndexError` and `KeyError`. These map to the dedicated
//! `N2WError::Value`/`Index`/`Key` variants in `base.rs`. They are Python
//! crashes rather than deliberate raises, but the exception *type* is
//! observable, so parity means reproducing it rather than tidying it into a
//! `TypeError`. See [`value_error`], [`index_error`], [`key_error`].

use std::sync::OnceLock;
use crate::base::{
    check_maxval, pow10_big, verify_ordinal, verify_ordinal_float, Lang, N2WError, Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

const ZERO: &str = "zero";
const NEGWORD: &str = "minus";

/// `ONES`; index 0 is absent in Python (guarded by `n1 > 0`).
const ONES: [&str; 10] = [
    "", "jeden", "dwa", "trzy", "cztery", "pięć", "sześć", "siedem", "osiem", "dziewięć",
];

/// `ONES_ORDINALS`, keys 1..=19, two forms each. Index 0 is absent in Python.
const ONES_ORDINALS: [(&str, &str); 20] = [
    ("", ""), // absent in Python
    ("pierwszy", "pierwszo"),
    ("drugi", "dwu"),
    ("trzeci", "trzy"),
    ("czwarty", "cztero"),
    ("piąty", "pięcio"),
    ("szósty", "sześcio"),
    ("siódmy", "siedmio"),
    ("ósmy", "ośmio"),
    ("dziewiąty", "dziewięcio"),
    ("dziesiąty", "dziesięcio"),
    ("jedenasty", "jedenasto"),
    ("dwunasty", "dwunasto"),
    ("trzynasty", "trzynasto"),
    ("czternasty", "czternasto"),
    ("piętnasty", "piętnasto"),
    ("szesnasty", "szesnasto"),
    ("siedemnasty", "siedemnasto"),
    ("osiemnasty", "osiemnasto"),
    ("dziewiętnasty", "dziewiętnasto"),
];

/// `TENS`, keys 0..=9 (the 10..19 teens, keyed by the units digit).
const TENS: [&str; 10] = [
    "dziesięć",
    "jedenaście",
    "dwanaście",
    "trzynaście",
    "czternaście",
    "piętnaście",
    "szesnaście",
    "siedemnaście",
    "osiemnaście",
    "dziewiętnaście",
];

/// `TWENTIES`, keys 2..=9. Indices 0/1 are absent in Python (guarded `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "dwadzieścia",
    "trzydzieści",
    "czterdzieści",
    "pięćdziesiąt",
    "sześćdziesiąt",
    "siedemdziesiąt",
    "osiemdziesiąt",
    "dziewięćdziesiąt",
];

/// `TWENTIES_ORDINALS`, keys 2..=9. Key 3's stem "trzydiesto" was a typo,
/// fixed to "trzydziesto" by PR #660.
const TWENTIES_ORDINALS: [(&str, &str); 10] = [
    ("", ""), // absent in Python
    ("", ""), // absent in Python
    ("dwudziesty", "dwudziesto"),
    ("trzydziesty", "trzydziesto"), // PR #660 fixed the "trzydiesto" typo
    ("czterdziesty", "czterdziesto"),
    ("pięćdziesiąty", "pięćdziesięcio"),
    ("sześćdziesiąty", "sześćdziesięcio"),
    ("siedemdziesiąty", "siedemdziesięcio"),
    ("osiemdziesiąty", "osiemdziesięcio"),
    ("dziewięćdziesiąty", "dziewięćdziesięcio"),
];

/// `HUNDREDS`, keys 1..=9. **One form only** — see bug 1 in the module docs.
const HUNDREDS: [&str; 10] = [
    "",
    "sto",
    "dwieście",
    "trzysta",
    "czterysta",
    "pięćset",
    "sześćset",
    "siedemset",
    "osiemset",
    "dziewięćset",
];

/// `HUNDREDS_ORDINALS`, keys 1..=9. The "pięcset"/"ośiemset" stem typos were
/// fixed to "pięćset"/"osiemset" by PR #660.
const HUNDREDS_ORDINALS: [(&str, &str); 10] = [
    ("", ""), // absent in Python
    ("setny", "stu"),
    ("dwusetny", "dwustu"),
    ("trzysetny", "trzystu"),
    ("czterysetny", "czterystu"),
    ("pięćsetny", "pięćset"), // PR #660 fixed the "pięcset" typo
    ("sześćsetny", "sześćset"),
    ("siedemsetny", "siedemset"),
    ("osiemsetny", "osiemset"), // PR #660 fixed the "ośiemset" typo
    ("dziewięćsetny", "dziewięćset"),
];

/// `prefixes_ordinal`, keys 1..=3 only. Index 0 is absent in Python and
/// unreachable (guarded by `level > 0`); index >= 4 is a `KeyError`.
const PREFIXES_ORDINAL: [&str; 4] = [
    "",
    "tysięczny",
    "milionowy",
    "miliardowy", // PR #660 fixed the "milairdowy" typo
];

/// `prefixes` — the 10^(6*x) stems.
const PREFIXES: [&str; 10] = [
    "mi", "bi", "try", "kwadry", "kwinty", "seksty", "septy", "okty", "nony", "decy",
];

/// `suffixes` — 10^x / 10^(x+3).
const SUFFIXES: [&str; 2] = ["lion", "liard"];

// --- Python exception encoding -------------------------------------------
//
// N2WError models only Overflow/Type/NotImplemented/ZeroDivision. Polish also
// raises ValueError, IndexError and KeyError. Encode the Python name in the
// message so a later phase can remap without re-deriving the semantics.

// These three mirror crashes in lang_PL.py, not deliberate raises: the
// exception *type* is observable behaviour a caller may catch, so parity
// requires reproducing it rather than tidying it into a TypeError.
fn value_error(msg: String) -> N2WError {
    N2WError::Value(msg)
}

fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// Python's `int(s)` for the digit-string shapes this module produces.
/// `int("-")` and `int("")` raise `ValueError`; `int("001")` == 1.
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10).ok_or_else(|| {
        value_error(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
///
/// Reproduces the negative-input hazard exactly: the head chunk `n[:start]`
/// can be `"-"` alone (→ `ValueError`) or `"-d"` (→ a negative chunk that
/// trips `get_digits` later).
fn splitbyx(n: &str, x: usize) -> Result<Vec<BigInt>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let slice = |i: usize, j: usize| -> String { chars[i..j.min(length)].iter().collect() };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse_int(&slice(0, start))?);
        }
        let mut i = start;
        while i < length {
            out.push(parse_int(&slice(i, i + x))?);
            i += x;
        }
    } else {
        out.push(parse_int(n)?);
    }
    Ok(out)
}

/// Python's `"%03d" % n`: field width 3 **including** the sign.
/// `-1` → `"-01"`, `-12` → `"-12"`, `-123` → `"-123"`, `1` → `"001"`.
fn fmt_03(n: &BigInt) -> String {
    let mag = n.abs().to_string();
    if n.is_negative() {
        let pad = 3usize.saturating_sub(1 + mag.len());
        format!("-{}{}", "0".repeat(pad), mag)
    } else {
        let pad = 3usize.saturating_sub(mag.len());
        format!("{}{}", "0".repeat(pad), mag)
    }
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// For negative `n` the `'-'` survives into the slice and `int('-')` raises
/// `ValueError` — this is the sole reason `to_ordinal` rejects negatives.
fn get_digits(n: &BigInt) -> Result<[usize; 3]> {
    let s = fmt_03(n);
    let chars: Vec<char> = s.chars().collect();
    // fmt_03 always yields >= 3 chars, so the [-3:] slice is total.
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c
            .to_digit(10)
            .ok_or_else(|| value_error(format!("invalid literal for int() with base 10: '{}'", c)))?
            as usize;
    }
    Ok(a)
}

/// `HUNDREDS[n3][level]` — the 1-element tuple. `level == 1` is an
/// `IndexError` in Python; see bug 1 in the module docs.
fn hundreds_at(n3: usize, level: usize) -> Result<&'static str> {
    if level != 0 {
        return Err(index_error("tuple index out of range"));
    }
    HUNDREDS
        .get(n3)
        .filter(|_| n3 >= 1)
        .copied()
        .ok_or_else(|| key_error(n3.to_string()))
}

/// `HUNDREDS_ORDINALS[n3][level]` — 2-element tuples, keys 1..=9.
fn hundreds_ordinals_at(n3: usize, level: usize) -> Result<&'static str> {
    let t = HUNDREDS_ORDINALS
        .get(n3)
        .filter(|_| n3 >= 1)
        .ok_or_else(|| key_error(n3.to_string()))?;
    match level {
        0 => Ok(t.0),
        1 => Ok(t.1),
        _ => Err(index_error("tuple index out of range")),
    }
}

/// `ONES_ORDINALS[k][level]` — 2-element tuples, keys 1..=19.
fn ones_ordinals_at(k: usize, level: usize) -> Result<&'static str> {
    let t = ONES_ORDINALS
        .get(k)
        .filter(|_| k >= 1)
        .ok_or_else(|| key_error(k.to_string()))?;
    match level {
        0 => Ok(t.0),
        1 => Ok(t.1),
        _ => Err(index_error("tuple index out of range")),
    }
}

/// `TWENTIES_ORDINALS[n2][level]` — 2-element tuples, keys 2..=9.
fn twenties_ordinals_at(n2: usize, level: usize) -> Result<&'static str> {
    let t = TWENTIES_ORDINALS
        .get(n2)
        .filter(|_| n2 >= 2)
        .ok_or_else(|| key_error(n2.to_string()))?;
    match level {
        0 => Ok(t.0),
        1 => Ok(t.1),
        _ => Err(index_error("tuple index out of range")),
    }
}

/// `Num2Word_PL.CURRENCY_FORMS` — PL's **own** class-body dict.
///
/// This is the one place where the `lang_EUR.py` trap does *not* apply.
/// `Num2Word_EN.__init__` rewrites EUR/GBP and adds ~24 codes by mutating
/// `Num2Word_EUR.CURRENCY_FORMS` in place, and the 16 classes that inherit
/// that dict see the result. `Num2Word_PL` subclasses `Num2Word_Base` and
/// declares its own dict, so none of EN's edits reach it. Confirmed on the
/// live interpreter after a full `import num2words2`:
/// `CONVERTER_CLASSES["pl"].CURRENCY_FORMS` holds exactly these four codes,
/// and it `is not` EUR's dict. Hence JPY/CHF/INR/CNY/KWD/BHD all raise
/// NotImplementedError for Polish — which is what the corpus freezes.
///
/// Arity is load-bearing: every entry carries **three** forms on both sides
/// because `Num2Word_PL.pluralize` indexes 0 (n == 1), 1 (2-4) or 2 (rest).
/// Dropping the third form would silently turn "pięć złotych" into a panic
/// or the wrong case.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 3] = ["cent", "centy", "centów"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "PLN",
        CurrencyForms::new(
            &["złoty", "złote", "złotych"],
            &["grosz", "grosze", "groszy"],
        ),
    );
    m.insert("EUR", CurrencyForms::new(&["euro", "euro", "euro"], &CENTS));
    m.insert(
        "USD",
        CurrencyForms::new(
            &[
                "dolar amerykański",
                "dolary amerykańskie",
                "dolarów amerykańskich",
            ],
            &CENTS,
        ),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(
            &[
                "funt brytyjski",
                "funty brytyjskie",
                "funtów brytyjskich",
            ],
            &["pens", "pensy", "pensów"],
        ),
    );
    m
}

pub struct LangPl {
    /// `THOUSANDS`: chunk index → the three plural forms. Keys 1..=21, i.e.
    /// up to 1000^21 == 10^63 ("decyliard"). A chunk index of 22 or more
    /// (10^66 and up) is pre-empted by the MAXVAL check (#159).
    thousands: HashMap<usize, [String; 3]>,
    /// `CURRENCY_FORMS`, built once here rather than per `to_currency` call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangPl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPl {
    pub fn new() -> Self {
        let mut thousands: HashMap<usize, [String; 3]> = HashMap::new();
        thousands.insert(
            1,
            [
                "tysiąc".to_string(),
                "tysiące".to_string(),
                "tysięcy".to_string(),
            ],
        );

        // Python: `for idx, (p, s) in enumerate(itertools.product(prefixes, suffixes))`
        // — product varies the *last* iterable fastest, so the order is
        // (mi,lion), (mi,liard), (bi,lion), (bi,liard), ... giving
        // THOUSANDS[2]="milion", [3]="miliard", [4]="bilion", ..., [21]="decyliard".
        let mut idx = 0usize;
        for p in PREFIXES.iter() {
            for s in SUFFIXES.iter() {
                let name = format!("{}{}", p, s);
                thousands.insert(
                    idx + 2,
                    [name.clone(), format!("{}y", name), format!("{}ów", name)],
                );
                idx += 1;
            }
        }

        LangPl {
            thousands,
            currency_forms: build_currency_forms(),
        }
    }

    /// `THOUSANDS[i]`, raising `KeyError` past 21 (unreachable, #159).
    fn thousands_at(&self, i: usize) -> Result<&[String; 3]> {
        self.thousands
            .get(&i)
            .ok_or_else(|| key_error(i.to_string()))
    }

    /// Port of `Num2Word_PL.pluralize`.
    ///
    /// ```python
    /// if n == 1: form = 0
    /// elif 5 > n % 10 > 1 and (n % 100 < 10 or n % 100 > 20): form = 1
    /// else: form = 2
    /// ```
    /// `n` is a 3-digit chunk and always non-negative by the time this runs
    /// (`get_digits` rejects negative chunks first), but `mod_floor` is used
    /// anyway to keep Python's `%` semantics rather than Rust's `%`.
    fn pluralize(&self, n: &BigInt, forms: &[String; 3]) -> String {
        let form = if n.is_one() {
            0usize
        } else {
            let m10 = n.mod_floor(&BigInt::from(10));
            let m100 = n.mod_floor(&BigInt::from(100));
            let chained = m10 < BigInt::from(5) && m10 > BigInt::one();
            if chained && (m100 < BigInt::from(10) || m100 > BigInt::from(20)) {
                1
            } else {
                2
            }
        };
        forms[form].clone()
    }

    /// Port of `Num2Word_PL._int2word`. Called only with non-negative values
    /// from `to_cardinal` (which strips the sign), but `to_ordinal`'s
    /// `pre_part` can hand it a negative, which raises `ValueError` via
    /// `get_digits` exactly as Python does.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3)?;
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x)?;

            if n3 > 0 {
                words.push(HUNDREDS[n3].to_string());
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1].to_string());
            } else if n1 > 0 && !(i > 0 && x.is_one()) {
                words.push(ONES[n1].to_string());
            }

            if i > 0 {
                let forms = self.thousands_at(i)?;
                words.push(self.pluralize(x, forms));
            }
        }

        Ok(words.join(" "))
    }

    /// Port of `Num2Word_PL.last_fragment_to_ordinal`.
    ///
    /// `level` is 0 or 1 — `to_ordinal` passes `0 if level == 0 else 1`.
    fn last_fragment_to_ordinal(
        &self,
        last: &BigInt,
        words: &mut Vec<String>,
        level: usize,
    ) -> Result<()> {
        let [n1, n2, n3] = get_digits(last)?;
        let last_two = n2 * 10 + n1;

        if last_two == 0 {
            words.push(hundreds_ordinals_at(n3, level)?.to_string());
        } else if level == 1 && last.is_one() {
            return Ok(());
        } else if last_two < 20 {
            // NB: HUNDREDS has a single form — this is the IndexError path
            // when level == 1 (e.g. to_ordinal(110000)).
            if n3 > 0 {
                words.push(hundreds_at(n3, level)?.to_string());
            }
            words.push(ones_ordinals_at(last_two, level)?.to_string());
        } else if last_two % 10 == 0 {
            // Same IndexError path when level == 1 (e.g. to_ordinal(120000)).
            if n3 > 0 {
                words.push(hundreds_at(n3, level)?.to_string());
            }
            words.push(twenties_ordinals_at(n2, level)?.to_string());
        } else {
            // This arm hardcodes form 0 regardless of `level`, which is why
            // to_ordinal(123000) == "sto dwudziesty trzecitysięczny".
            if n3 > 0 {
                words.push(hundreds_at(n3, 0)?.to_string());
            }
            words.push(twenties_ordinals_at(n2, 0)?.to_string());
            words.push(ones_ordinals_at(n1, 0)?.to_string());
        }
        Ok(())
    }

    /// The body of `Num2Word_PL.to_cardinal` after the whole-`float`
    /// short-circuit, i.e. the string-driven decimal path:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     decimal_parts = []
    ///     for digit in right:
    ///         if digit == "0": decimal_parts.append(ZERO[0])
    ///         else:            decimal_parts.append(ONES[int(digit)][0])
    ///     decimal_part = " ".join(decimal_parts)
    ///     result = "%s %s %s" % (self._int2word(int(left)),
    ///                            self.pointword, decimal_part)
    ///     if is_negative: result = self.negword + " " + result
    ///     return result
    /// else:
    ///     is_negative = n.startswith("-")
    ///     if is_negative:
    ///         abs_n = n[1:]
    ///         return self.negword + " " + self._int2word(int(abs_n))
    ///     else:
    ///         return self._int2word(int(n))
    /// ```
    ///
    /// `n` here is already `str(number)` — the caller supplies the shortest
    /// round-trip repr of an `f64` (`format!("{}", f)`) or the reconstructed
    /// `str(Decimal)`. Both use `.` for the point, so the `,`→`.` replace is a
    /// no-op kept only for fidelity.
    ///
    /// The `else` (no-dot) arm is reachable only from an integer `Decimal`
    /// (precision 0); an `f64` that survives the whole-number short-circuit
    /// always formats with a `.`.
    fn cardinal_from_str(&self, number: &str) -> Result<String> {
        let n = number.replace(',', ".");
        if let Some(dot) = n.find('.') {
            let is_negative = n.starts_with('-');
            // abs_n = n[1:] if is_negative else n
            let (abs_start, dot) = if is_negative { (1usize, dot) } else { (0usize, dot) };
            let left = &n[abs_start..dot];
            let right = &n[dot + 1..];

            // decimal digits, spoken individually
            let mut decimal_parts: Vec<&str> = Vec::new();
            for ch in right.chars() {
                if ch == '0' {
                    decimal_parts.push(ZERO);
                } else {
                    // ONES[int(digit)][0]; digit is 1..=9 here (0 handled above).
                    let d = ch.to_digit(10).ok_or_else(|| {
                        value_error(format!(
                            "invalid literal for int() with base 10: '{}'",
                            ch
                        ))
                    })? as usize;
                    decimal_parts.push(ONES[d]);
                }
            }
            let decimal_part = decimal_parts.join(" ");

            // self._int2word(int(left))
            let left_word = self.int2word(&parse_int(left)?)?;
            let result = format!("{} {} {}", left_word, self.pointword(), decimal_part);
            if is_negative {
                Ok(format!("{} {}", NEGWORD, result))
            } else {
                Ok(result)
            }
        } else {
            let is_negative = n.starts_with('-');
            if is_negative {
                let abs_n = &n[1..];
                Ok(format!("{} {}", NEGWORD, self.int2word(&parse_int(abs_n)?)?))
            } else {
                self.int2word(&parse_int(&n)?)
            }
        }
    }
}

/// Python's `int(s)` acceptance for the strings PL's `to_ordinal` splits: an
/// optional sign followed by ASCII digits. Anything else — a '.' or the 'E'
/// of a scientific `str(Decimal)` — fails, which the callers turn into the
/// ValueError `splitbyx`'s `int()` raises.
fn is_plain_int_str(s: &str) -> bool {
    let t = s.strip_prefix('-').unwrap_or(s);
    !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit())
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^63, so 10^66 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(66))
}

impl Lang for LangPl {
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
        ","
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "przecinek"
    }

    /// Port of `Num2Word_PL.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and looks for `"."`; `str(int)` never
    /// contains one, so integers always take the `else` branch. The float
    /// branch (`pointword`, per-digit decimals) is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // Python: abs_n = n[1:]; negword + " " + self._int2word(int(abs_n))
            let result = self.int2word(&value.abs())?;
            Ok(format!("{} {}", NEGWORD, result))
        } else {
            self.int2word(value)
        }
    }

    /// Port of `Num2Word_PL.to_ordinal`.
    ///
    /// `if number % 1 != 0: raise NotImplementedError()` is unreachable for
    /// integers, so it is not modelled. Raises `KeyError` for level >= 4
    /// (>= 10^12), `TypeError` for every negative (#155), and
    /// `OverflowError` for values >= 10^66 (#159).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;
        // PR savoirfairelinux/num2words#668: splitbyx("0") yields a single
        // zero fragment; the pop loop below empties the list and then indexes
        // it (IndexError). Handle zero explicitly.
        if value.is_zero() {
            return Ok("zerowy".to_string());
        }
        check_maxval(value, maxval_ceiling())?;
        let mut words: Vec<String> = Vec::new();
        let mut fragments = splitbyx(&value.to_string(), 3)?;

        let mut level: usize = 0;
        // fragments[-1] on an empty list → IndexError. splitbyx always yields
        // at least one element, so only the pop loop below can empty it.
        let mut last = fragments
            .last()
            .ok_or_else(|| index_error("list index out of range"))?
            .clone();
        while last.is_zero() {
            level += 1;
            fragments.pop();
            last = fragments
                .last()
                .ok_or_else(|| index_error("list index out of range"))?
                .clone();
        }

        if fragments.len() > 1 {
            let pow = BigInt::from(1000).pow(level as u32);
            let pre_part = self.int2word(&(value - &last * pow))?;
            words.push(pre_part);
        }

        self.last_fragment_to_ordinal(&last, &mut words, if level == 0 { 0 } else { 1 })?;

        let mut output = words.join(" ");
        if last.is_one() && level > 0 && !output.is_empty() {
            output.push(' ');
        }
        if level > 0 {
            // prefixes_ordinal has keys 1..=3; level >= 4 → KeyError.
            let p = PREFIXES_ORDINAL
                .get(level)
                .filter(|_| level >= 1)
                .copied()
                .ok_or_else(|| key_error(level.to_string()))?;
            output.push_str(p);
        }
        Ok(output)
    }

    /// Float / `Decimal` cardinal path.
    ///
    /// PL overrides **`to_cardinal`** (not `to_cardinal_float`) and handles
    /// non-integers inline off `str(number)`, so it never touches
    /// `base.float2tuple`. Two consequences:
    ///
    /// * The f64-artefact and banker's-rounding traps of the base float path
    ///   do **not** apply here: PL reads the *shortest repr digits*
    ///   (`str(2.675) == "2.675"` → "sześć siedem pięć"), never
    ///   `abs(value-pre)*10**precision`.
    /// * `precision_override` (the `precision=` kwarg) is ignored, exactly as
    ///   Python's `to_cardinal(self, number)` ignores it — verified on the
    ///   interpreter: `num2words(1.5, lang="pl", precision=3)` is unchanged.
    ///
    /// ```python
    /// def to_cardinal(self, number):
    ///     if isinstance(number, float) and number == int(number):
    ///         return self._int2word(int(number))
    ///     n = str(number).replace(",", ".")
    ///     ...
    /// ```
    ///
    /// The whole-`float` short-circuit calls `_int2word(int(number))` **without
    /// stripping the sign**, so a negative whole float (`-1.0`, `-9999.0`)
    /// reproduces Python's `ValueError` via `get_digits` — do not route it
    /// through the sign-stripping integer `to_cardinal`. A `Decimal` is not a
    /// `float`, so it skips the short-circuit and always takes the string arm
    /// (hence `Decimal("-5")` → "minus pięć", but `-5.0` → `ValueError`).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                // isinstance(number, float) and number == int(number)
                if *f == f.trunc() {
                    let n = BigInt::from_f64(*f).ok_or_else(|| {
                        // int(nan/inf) raises in Python too; unreachable for
                        // the corpus. Kept as a ValueError rather than a panic.
                        value_error(format!("cannot convert float {} to integer", f))
                    })?;
                    // Sign intentionally NOT stripped — mirrors _int2word(int(number)).
                    return self.int2word(&n);
                }
                // n = str(number): Rust's `{}` is shortest round-trip, matching
                // Python's `repr`/`str` over the finite non-exponential range PL
                // is exercised on.
                self.cardinal_from_str(&format!("{}", f))
            }
            FloatValue::Decimal { value: d, .. } => {
                // n = str(number): the General-Decimal-Arithmetic string, so
                // trailing zeros survive ("5.0" → "pięć przecinek zero") and
                // scientific forms keep their 'E' ("1E+2") and die in int().
                self.cardinal_from_str(&python_decimal_str(d))
            }
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values
    /// included. PL's `to_cardinal` must see every float/Decimal itself: the
    /// whole-`float` short-circuit feeds `_int2word(int(number))` **without
    /// stripping the sign** (so `-1.0` is ValueError while `1.0` is "jeden"),
    /// and a whole `Decimal` is *not* a float, so it takes the string arm and
    /// keeps its fractional field ("5.0" → "pięć przecinek zero"). The trait
    /// default's `as_whole_int()` → integer-path shortcut would erase both.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`.
    ///
    /// Python raised an empty `NotImplementedError` for a fractional value
    /// and split `str(number)` for a whole one, where any '.' or 'E' died in
    /// `int()` with `ValueError` — so every float failed. Now Base's
    /// `verify_ordinal` rules apply (#155): fractional → `errmsg_floatord`,
    /// negative → `errmsg_negord`, whole → the integer ordinal.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_ordinal(&verify_ordinal_float(value)?)
    }

    /// `Decimal('-0.0')` per mode. `BigDecimal` cannot carry the sign, so the
    /// binding cannot demote it to a signed-zero `Float` without losing the
    /// negword — PL renders it through its own float grammar *with* the
    /// negword instead of leaning on the Python fallback.
    ///
    /// Only cardinal and year need serving here: PL's `to_cardinal(Decimal)`
    /// (which year delegates to) reads `str(number)` == "-0.0", strips the
    /// sign textually, and speaks "minus zero przecinek zero". The other two
    /// modes coincide with the demoted `Float{-0.0}` path and return `None`:
    ///   * ordinal → zero, which `ordinal_float_entry` already gives for
    ///     `Float{-0.0}`;
    ///   * ordinal_num → Base echoes `str(number)` == "-0.0", which the default
    ///     `ordinal_num_float_entry` already returns from `repr_str`.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            // to_year delegates to to_cardinal, so both render identically.
            "cardinal" | "year" => Some(self.cardinal_from_str("-0.0")),
            _ => None,
        }
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`.
    /// `Decimal("Infinity")` parses fine in Python; the failure happens
    /// *next*, inside PL's own `to_cardinal`: `str(number)` == "Infinity"
    /// has no '.', so `int("Infinity")` raises **ValueError**. The binding
    /// otherwise maps `ParsedNumber::Inf` to the base integer path's
    /// OverflowError, so the ValueError must be raised here. (NaN needs no
    /// interception: the binding's ValueError already matches.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(value_error(format!(
                "invalid literal for int() with base 10: '{}Infinity'",
                if negative { "-" } else { "" }
            ))),
            other => Ok(other),
        }
    }

    // to_ordinal_num: PL does not override Num2Word_Base.to_ordinal_num,
    // which returns the value unchanged → the trait default is correct.
    //
    // to_year: PL does not override Num2Word_Base.to_year, which delegates
    // to to_cardinal — and the trait's `year_float_entry` default routes
    // back through `cardinal_float_entry` above, so float/Decimal years
    // pick up the same string-driven grammar ("5.0" → "pięć przecinek
    // zero", "-1.0" → ValueError).

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_PL` defines exactly three things on this surface:
    // `CURRENCY_FORMS`, `pluralize` and a `to_currency` override for the
    // integer path. Everything else — `to_cheque`, `_money_verbose`,
    // `_cents_verbose`, `_cents_terse` — is inherited from `Num2Word_Base`
    // untouched (verified with `"x" in type(c).__dict__` on the live class),
    // so the trait defaults already mirror it.
    //
    // Deliberately NOT overridden:
    //   * `currency_precision` — `Num2Word_PL.CURRENCY_PRECISION` *is*
    //     `Num2Word_Base.CURRENCY_PRECISION`, and it is still `{}` after a
    //     full import (EN *rebinds* rather than mutates it, so its 1000s do
    //     not leak). Every code therefore uses the default divisor of 100,
    //     including the 3-decimal ones — moot here, since PL implements no
    //     KWD/BHD at all.
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is likewise Base's
    //     empty dict, so `adjective=True` is accepted and has no effect.
    //     PL's int path does not even consult it (see `to_currency`).

    fn lang_name(&self) -> &str {
        "Num2Word_PL"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_PL.pluralize`.
    ///
    /// ```python
    /// if n == 1:
    ///     form = 0
    /// elif 5 > n % 10 > 1 and (n % 100 < 10 or n % 100 > 20):
    ///     form = 1
    /// else:
    ///     form = 2
    /// return forms[form]
    /// ```
    ///
    /// `mod_floor` rather than `%`: Python's `%` floors on negatives. Every
    /// reachable caller passes a non-negative `n` (`to_currency` takes `abs`,
    /// `parse_currency_parts` returns `abs`), so the two agree today, but the
    /// port matches the source's semantics rather than relying on that.
    ///
    /// Note this duplicates the rule in the inherent [`LangPl::pluralize`]
    /// used by `int2word` for the THOUSANDS scale words. Python has one method
    /// serving both call sites; the split here exists only because the
    /// cardinal path takes a fixed `&[String; 3]` and is already frozen. The
    /// unit test at the bottom of this file pins the two together.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = plural_form_index(n);
        // `forms[form]` → IndexError if the tuple is short. All four PL
        // entries carry three forms, so this is unreachable; modelled for
        // fidelity rather than papered over with an unwrap.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| index_error("tuple index out of range"))
    }

    /// Port of `Num2Word_PL.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True,
    ///                 separator=",", adjective=False):
    ///     if isinstance(val, int):
    ///         try:
    ///             cr1, cr2 = self.CURRENCY_FORMS[currency]
    ///         except (KeyError, AttributeError):
    ///             return super().to_currency(val, ...)
    ///         minus_str = self.negword if val < 0 else ""
    ///         abs_val = abs(val)
    ///         money_str = self.to_cardinal(abs_val)
    ///         if abs_val > 1 and money_str.endswith("jeden"):
    ///             currency_str = cr1[1] if len(cr1) > 1 else cr1[0]
    ///         else:
    ///             currency_str = self.pluralize(abs_val, cr1)
    ///         return ("%s %s %s" % (minus_str, money_str, currency_str)).strip()
    ///     return super().to_currency(val, ...)
    /// ```
    ///
    /// Only the *integer* path is PL's own; floats fall through to
    /// `Num2Word_Base.to_currency` (i.e. [`default_to_currency`]) unchanged.
    /// That split is why `CurrencyValue`'s Int/Decimal distinction cannot be
    /// collapsed: `1` gives "jeden euro" while `1.0` gives "jeden euro, zero
    /// centów".
    ///
    /// Two quirks are reproduced verbatim:
    ///
    /// 1. **The "jeden" override** (bug 6 in the module docs). `abs_val > 1`
    ///    plus a cardinal ending in "jeden" forces form 1, overriding
    ///    `pluralize`, which would have said form 2. So `to_currency(21,
    ///    "USD")` is "dwadzieścia jeden dolary amerykańskie" where Polish
    ///    wants "dolarów amerykańskich". It fires for 21/101/1001/10121/…,
    ///    all confirmed against the interpreter.
    /// 2. **`minus_str` is the bare negword**, not Base's
    ///    `"%s " % negword.strip()`. The `"%s %s %s"` join supplies the space
    ///    and the trailing `.strip()` eats the leading one when the value is
    ///    non-negative — so both branches still land on "minus X" / "X".
    ///
    /// `adjective` is threaded to `super()` but never applied on the int path,
    /// exactly as in Python. It makes no observable difference: PL's
    /// `CURRENCY_ADJECTIVES` is Base's empty dict.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it through this language's own default (Base's ",").
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(v) = val {
            // `except (KeyError, AttributeError): return super().to_currency(...)`.
            // super() then re-does the same lookup and raises
            // NotImplementedError, so an unknown code with an int argument
            // still reports `Currency code "XYZ" not implemented for
            // "Num2Word_PL"` — verified on the interpreter for `(5, "XYZ")`.
            let cr1 = match self.currency_forms.get(currency) {
                Some(forms) => &forms.unit,
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };

            let minus_str = if v.is_negative() { NEGWORD } else { "" };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            let currency_str = if abs_val > BigInt::one() && money_str.ends_with("jeden") {
                // `cr1[1] if len(cr1) > 1 else cr1[0]` — the bare index on the
                // else arm is an IndexError for an empty tuple. Unreachable
                // for PL's table; transcribed rather than simplified.
                if cr1.len() > 1 {
                    cr1[1].clone()
                } else {
                    cr1.first()
                        .cloned()
                        .ok_or_else(|| index_error("tuple index out of range"))?
                }
            } else {
                // Fully qualified: the inherent `LangPl::pluralize` shadows the
                // trait method for `self.pluralize(..)`, and it takes a
                // `&[String; 3]` that a `&[String]` cannot coerce to.
                Lang::pluralize(self, &abs_val, cr1)?
            };

            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats/Decimals: `super().to_currency(...)` verbatim.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}

/// The form index `Num2Word_PL.pluralize` selects for `n`.
///
/// Split out so the trait `pluralize` and the test below share one expression
/// of the rule. The inherent [`LangPl::pluralize`] on the (frozen) cardinal
/// path states it independently; `plural_rule_matches_cardinal_path` asserts
/// they cannot drift.
fn plural_form_index(n: &BigInt) -> usize {
    if n.is_one() {
        return 0;
    }
    let m10 = n.mod_floor(&BigInt::from(10));
    let m100 = n.mod_floor(&BigInt::from(100));
    let chained = m10 < BigInt::from(5) && m10 > BigInt::one();
    if chained && (m100 < BigInt::from(10) || m100 > BigInt::from(20)) {
        1
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    /// `Num2Word_PL.pluralize` is a single Python method serving both the
    /// cardinal path (THOUSANDS scale words) and the currency path. The port
    /// states the rule twice — the inherent method predates this change and is
    /// frozen — so pin the two together rather than trusting inspection.
    #[test]
    fn plural_rule_matches_cardinal_path() {
        let lang = LangPl::new();
        let forms = [
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        for n in 0i64..=1000 {
            let n = BigInt::from(n);
            let via_cardinal = lang.pluralize(&n, &forms);
            let via_currency = Lang::pluralize(&lang, &n, &forms).unwrap();
            assert_eq!(via_cardinal, via_currency, "n = {}", n);
        }
    }

    /// The three form buckets, spot-checked against the live interpreter:
    /// `1` → form 0, `2..=4` (outside the 11-20 teens) → form 1, rest → 2.
    #[test]
    fn plural_form_buckets() {
        let cases: &[(i64, usize)] = &[
            (0, 2),
            (1, 0),
            (2, 1),
            (4, 1),
            (5, 2),
            (11, 2),
            (12, 2),
            (14, 2),
            (21, 2),
            (22, 1),
            (25, 2),
            (100, 2),
            (1234, 1),
        ];
        for (n, want) in cases {
            assert_eq!(plural_form_index(&BigInt::from(*n)), *want, "n = {}", n);
        }
    }

    fn f(lang: &LangPl, v: f64) -> Result<String> {
        // precision is ignored on the float arm; the binding derives it, but
        // PL's str-based path does not consult it. 0 is fine here.
        lang.to_cardinal_float(&FloatValue::Float { value: v, precision: 0 }, None)
    }

    fn dec(lang: &LangPl, s: &str) -> Result<String> {
        let value: BigDecimal = s.parse().unwrap();
        let precision = match s.split_once('.') {
            Some((_, frac)) => frac.len() as u32,
            None => 0,
        };
        lang.to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
    }

    #[test]
    fn float_corpus() {
        let lang = LangPl::new();
        let cases: &[(f64, &str)] = &[
            (0.0, "zero"),
            (0.5, "zero przecinek pięć"),
            (1.0, "jeden"),
            (1.5, "jeden przecinek pięć"),
            (2.25, "dwa przecinek dwa pięć"),
            (3.14, "trzy przecinek jeden cztery"),
            (0.01, "zero przecinek zero jeden"),
            (0.1, "zero przecinek jeden"),
            (0.99, "zero przecinek dziewięć dziewięć"),
            (1.01, "jeden przecinek zero jeden"),
            (12.34, "dwanaście przecinek trzy cztery"),
            (99.99, "dziewięćdziesiąt dziewięć przecinek dziewięć dziewięć"),
            (100.5, "sto przecinek pięć"),
            (1234.56, "tysiąc dwieście trzydzieści cztery przecinek pięć sześć"),
            (-0.5, "minus zero przecinek pięć"),
            (-1.5, "minus jeden przecinek pięć"),
            (-12.34, "minus dwanaście przecinek trzy cztery"),
            // f64-artefact cases: PL reads shortest repr digits, not float2tuple.
            (1.005, "jeden przecinek zero zero pięć"),
            (2.675, "dwa przecinek sześć siedem pięć"),
        ];
        for (v, want) in cases {
            assert_eq!(&f(&lang, *v).unwrap(), want, "v = {}", v);
        }
    }

    #[test]
    fn whole_floats_and_negatives() {
        let lang = LangPl::new();
        // Whole floats short-circuit to _int2word(int(number)).
        assert_eq!(f(&lang, 0.0).unwrap(), "zero");
        assert_eq!(f(&lang, 1.0).unwrap(), "jeden");
        assert_eq!(f(&lang, 100.0).unwrap(), "sto");
        assert_eq!(
            f(&lang, 9999.0).unwrap(),
            "dziewięć tysięcy dziewięćset dziewięćdziesiąt dziewięć"
        );
        assert_eq!(f(&lang, 1e16).unwrap(), "dziesięć biliardów");
        assert_eq!(f(&lang, 1e21).unwrap(), "tryliard");
        // 1e23 is not exactly 10^23; int(1e23) == 99999999999999991611392,
        // so from_f64 must be exact, not a shortest-repr string parse.
        assert_eq!(
            f(&lang, 1e23).unwrap(),
            "dziewięćdziesiąt dziewięć tryliardów dziewięćset dziewięćdziesiąt \
             dziewięć trylionów dziewięćset dziewięćdziesiąt dziewięć biliardów \
             dziewięćset dziewięćdziesiąt dziewięć bilionów dziewięćset \
             dziewięćdziesiąt dziewięć miliardów dziewięćset dziewięćdziesiąt \
             jeden milionów sześćset jedenaście tysięcy trzysta \
             dziewięćdziesiąt dwa"
        );
        // Negative whole floats reproduce Python's ValueError (int(-1.0) fed
        // to _int2word → get_digits sees '-').
        for v in [-1.0, -5.0, -9999.0] {
            match f(&lang, v) {
                Err(N2WError::Value(m)) => {
                    assert_eq!(m, "invalid literal for int() with base 10: '-'")
                }
                other => panic!("v = {} expected ValueError, got {:?}", v, other),
            }
        }
    }

    #[test]
    fn decimal_corpus() {
        let lang = LangPl::new();
        let cases: &[(&str, &str)] = &[
            ("0.01", "zero przecinek zero jeden"),
            ("1.10", "jeden przecinek jeden zero"),
            ("12.345", "dwanaście przecinek trzy cztery pięć"),
            (
                "98746251323029.99",
                "dziewięćdziesiąt osiem bilionów siedemset czterdzieści sześć \
                 miliardów dwieście pięćdziesiąt jeden milionów trzysta \
                 dwadzieścia trzy tysiące dwadzieścia dziewięć przecinek \
                 dziewięć dziewięć",
            ),
            ("0.001", "zero przecinek zero zero jeden"),
            // Integer Decimals take the no-dot arm; the sign is stripped there,
            // unlike a negative whole float.
            ("5", "pięć"),
            ("-5", "minus pięć"),
            ("0", "zero"),
            ("5.0", "pięć przecinek zero"),
            ("1.00", "jeden przecinek zero zero"),
            ("0.0", "zero przecinek zero"),
            ("0.00", "zero przecinek zero zero"),
            ("0.99", "zero przecinek dziewięć dziewięć"),
            ("-12.34", "minus dwanaście przecinek trzy cztery"),
        ];
        for (s, want) in cases {
            assert_eq!(&dec(&lang, s).unwrap(), want, "dec = {}", s);
        }
    }
}
