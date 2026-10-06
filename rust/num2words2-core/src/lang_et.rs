//! Port of `lang_ET.py` (Estonian).
//!
//! Shape: **self-contained**. `Num2Word_ET` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL` (see the `hasattr` guard at the
//! end of `Num2Word_Base.__init__`). All four in-scope entry points are
//! overridden outright and drive a hand-written `_int_to_word` recursion.
//! Consequently `cards`/`merge` stay at their trait defaults here, and Python
//! has **no overflow check at all** — arbitrarily large values are accepted
//! (see bug 5 below for what happens past 10^12). On a large enough integer
//! the port's recursion overflowed the native stack, so it adds a ceiling
//! (gladiaio/num2words2#203): `maxval` is 10^15 and every mode raises
//! `OverflowError` from there.
//!
//! `Num2Word_ET._setup` is a bare `super()._setup()` passthrough, and nothing
//! in the base ever calls it (`__init__` calls `setup`, not `_setup`), so it is
//! dead code and is not modelled.
//!
//! Nothing here is stateful: no method stashes a flag for another to consume,
//! so the Rust path being stateless costs nothing. (`self.precision` is touched
//! only on the float path, which is out of scope.)
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong and is exactly
//! what Python emits — each is pinned by a `bench/corpus.jsonl` row.
//!
//! 1. *(Fixed, #248.)* Python glued an `s` onto the cardinal for every
//!    `n > 100` outside its special cases ("ükssada ükss", "ükssada
//!    üksteists"). The port now builds standard Estonian compound ordinals:
//!    every component but the last in the genitive, the last one ordinal —
//!    101 == "saja esimene", 121 == "saja kahekümne esimene", 200 ==
//!    "kahesajas", 1001 == "tuhande esimene", 2021 == "kahe tuhande
//!    kahekümne esimene", 10**6 == "miljones". Round thousands from 2000
//!    and other round values from 10**6 up ("kaks tuhats", "üks triljons")
//!    still take the `+ "s"` fallback; that remains a known gap.
//! 2. ~~**`to_ordinal` on negatives is Python list indexing, not arithmetic.**~~
//!    Fixed (gladiaio/num2words2#155). `_int_to_ordinal` checks `n == 0`
//!    then `n < 10`, and every negative passed `n < 10`, reaching
//!    `self.ordinals_ones[n]` with a negative index: Python wrapped around
//!    and returned a *positive* ordinal (`to_ordinal(-1)` == "üheksas") for
//!    `-10..=-1` and raised `IndexError` below that. The port now raises
//!    Base's `errmsg_negord` `TypeError` for every negative, like most
//!    languages, and so does the truncating float entry (`-0.5`, `-1.5`).
//! 3. *(Fixed, #248.)* Compound ordinals 21..99 used the *cardinal* tens
//!    stem ("kakskümmend esimene"); they now use the genitive, "kahekümne
//!    esimene".
//! 4. *(Fixed, #248.)* `ordinals_tens[4]` was the typo "nelikümnes"; it is
//!    now "neljakümnes", like its genitive-stem neighbours.
//! 5. **Above 10^12 the trillions branch recurses into itself**, so the scale
//!    word repeats instead of naming a higher scale: `10**15` ==
//!    "tuhat triljonit", `10**18` == "üks miljon triljonit", `10**21` ==
//!    "üks miljard triljonit". All three are corpus rows. Since Python has no
//!    MAXVAL, this continues indefinitely (10^24 would be
//!    "üks triljon triljonit"). The port stops at 10^15, before the word
//!    repeats (gladiaio/num2words2#203).
//! 6. **The `thousands == 100` special case is redundant-looking but load-
//!    bearing**: it yields "sada tuhat" for 100_000 where the general arm would
//!    give "ükssada tuhat". Only the *exact* multiplier 100 is special — 101_000
//!    still goes through the general arm as "ükssada üks tuhat". Preserved as-is.
//!
//! # Non-bugs worth noting
//!
//! * `to_year` has three branches (`< 1000`, `< 2000`, `else`) whose bodies are
//!    **identical** — all three call `self._int_to_cardinal(n)`. It is therefore
//!    exactly `to_cardinal` for integers, negatives included
//!    (`to_year(-500)` == "miinus viissada"). Collapsed to one call here; the
//!    dead branching is not observable.
//! * `to_ordinal_num` is `str(n) + "."` with no sign handling, so
//!    `to_ordinal_num(-21)` == "-21.".
//! * `negword` is "miinus " **with a trailing space**, and `_int_to_cardinal`
//!    concatenates it raw (`self.negword + ...`) rather than going through
//!    `base.py`'s `"%s " % self.negword.strip()`. Same result, but the space
//!    comes from the table here, so it must not be trimmed.
//!
//! # Currency
//!
//! `Num2Word_ET` declares its **own** `CURRENCY_FORMS` in the class body, so it
//! is a fresh dict rather than the one `Num2Word_EN.__init__` mutates in place
//! on `Num2Word_EUR`. The EUR-table trap therefore does not apply here: ET sees
//! exactly its seven codes (EUR/USD/GBP/SEK/NOK/DKK/RUB) and nothing else, which
//! is why JPY, KWD, BHD, INR, CNY and CHF all raise. Live-interpreter confirmed,
//! not read off the source.
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both left at Base's empty
//! dict, so `currency_precision` stays at the trait default of 100 for every
//! code and `currency_adjective` is never consulted. `pluralize` is likewise
//! left at the (raising) default — ET never calls it, having inlined the form
//! choice; see oddity 9 below.
//!
//! `to_cheque` is **not** overridden, so `currency::default_to_cheque` (Base's)
//! runs unchanged: it reaches back through `money_verbose` -> `to_cardinal` ->
//! `_int_to_cardinal`, and takes `cr1[-1]`, the plural — hence
//! "... AND 56/100 EUROT". Only `currency_forms` + `lang_name` are needed to
//! feed it.
//!
//! ## Further faithfully reproduced Python bugs (currency)
//!
//! 7. **`to_currency`'s error message omits the word "code".** ET raises
//!    `Currency "JPY" not implemented for "Num2Word_ET"`, where `base.py` says
//!    `Currency code "%s" ...`. Since `to_cheque` is inherited and *does* use
//!    Base's wording, the same converter reports a missing code two different
//!    ways depending on the entry point. Both are reproduced verbatim.
//! 8. **The cents segment keys on `isinstance(val, int)` alone**; Base's
//!    `has_decimal` guard does not exist here. So a float always prints cents —
//!    `to_currency(1.0)` == "üks euro ja null senti" (corpus row) — and, less
//!    obviously, `to_currency(Decimal("5"))` prints "viis eurot ja null senti"
//!    where Base would say "viis eurot". `CurrencyValue::has_decimal` is
//!    therefore deliberately ignored by this override.
//! 9. **`adjective=` is accepted and never read.** ET's signature carries it for
//!    compatibility but the body has no `CURRENCY_ADJECTIVES` lookup, so
//!    `adjective=True` is silently a no-op. (Moot in practice: the dict is empty
//!    anyway.)
//! 10. **The divisor 100 is hardcoded.** Both the `(Decimal(str(val)) * 100) % 1`
//!    test and the `parse_currency_parts` call (which takes the kwarg default)
//!    ignore `CURRENCY_PRECISION` entirely. Unobservable today — the dict is
//!    empty — but it means a 3-decimal code added to ET's table would still be
//!    split at 1/100.
//! 11. **`cents=False` does not zero-pad.** ET emits `str(right)`, not
//!    `_cents_terse(right)`, so `to_currency(0.01, cents=False)` gives "1"
//!    where Base gives "01".
//! 12. **The unit/subunit form is chosen by `left == 1` / `right == 1`, with no
//!    Estonian case agreement.** Estonian wants the partitive after most
//!    numerals; ET just picks index 0 or 1. Corpus-pinned as-is
//!    ("null eurot", "kaks eurot", "üks euro").

use crate::base::{check_maxval, negord_error, pow10_big, py_num_str, strictly_negative, verify_ordinal, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.ones` — index 0 is an unused empty filler.
const ONES: [&str; 10] = [
    "", "üks", "kaks", "kolm", "neli", "viis", "kuus", "seitse", "kaheksa", "üheksa",
];

/// `self.tens` — index 0 is an unused empty filler.
const TENS: [&str; 10] = [
    "",
    "kümme",
    "kakskümmend",
    "kolmkümmend",
    "nelikümmend",
    "viiskümmend",
    "kuuskümmend",
    "seitsekümmend",
    "kaheksakümmend",
    "üheksakümmend",
];
/// `self.ordinals_ones`. Index 0 is an empty filler, once reachable via
/// Python's negative indexing (bug 2, fixed in #155); never read now.
const ORDINALS_ONES: [&str; 10] = [
    "",
    "esimene",
    "teine",
    "kolmas",
    "neljas",
    "viies",
    "kuues",
    "seitsmes",
    "kaheksas",
    "üheksas",
];

/// `self.ordinals_tens`. Index 4 was the typo "nelikümnes" in Python (bug 4,
/// fixed in #248).
const ORDINALS_TENS: [&str; 10] = [
    "",
    "kümnes",
    "kahekümnes",
    "kolmekümnes",
    "neljakümnes",
    "viiekümnes",
    "kuuekümnes",
    "seitsmekümnes",
    "kaheksakümnes",
    "üheksakümnes",
];

/// Genitive of 1..=9, the stem of every non-final component of a compound
/// ordinal ("kahekümne esimene", "kahesajas"; #248). Index 0 is unused.
const GEN_ONES: [&str; 10] = [
    "", "ühe", "kahe", "kolme", "nelja", "viie", "kuue", "seitsme", "kaheksa", "üheksa",
];

/// Genitive of `1..=99`.
fn gen_below_100(r: usize) -> String {
    match r {
        1..=9 => GEN_ONES[r].to_string(),
        10 => "kümne".to_string(),
        11..=19 => format!("{}teistkümne", GEN_ONES[r - 10]),
        _ if r % 10 == 0 => format!("{}kümne", GEN_ONES[r / 10]),
        _ => format!("{}kümne {}", GEN_ONES[r / 10], GEN_ONES[r % 10]),
    }
}

/// Genitive of the hundreds digit `h` in 1..=9: "saja", "kahesaja", ….
fn gen_hundreds(h: usize) -> String {
    if h == 1 {
        "saja".to_string()
    } else {
        format!("{}saja", GEN_ONES[h])
    }
}

/// Genitive of `1..=999`.
fn gen_below_1000(m: usize) -> String {
    let (h, r) = (m / 100, m % 100);
    match (h, r) {
        (0, _) => gen_below_100(r),
        (_, 0) => gen_hundreds(h),
        _ => format!("{} {}", gen_hundreds(h), gen_below_100(r)),
    }
}

/// Ordinal of `1..=99`: genitive tens + ordinal ones for compounds.
fn ord_below_100(r: usize) -> String {
    match r {
        1..=9 => ORDINALS_ONES[r].to_string(),
        10 => "kümnes".to_string(),
        11..=19 => TEENS_ORDINALS[r - 11].to_string(),
        _ if r % 10 == 0 => ORDINALS_TENS[r / 10].to_string(),
        _ => format!("{}kümne {}", GEN_ONES[r / 10], ORDINALS_ONES[r % 10]),
    }
}

/// Ordinal of `1..=999`: "sajas", "kahesajas", "saja kahekümne esimene".
fn ord_below_1000(m: usize) -> String {
    let (h, r) = (m / 100, m % 100);
    match (h, r) {
        (0, _) => ord_below_100(r),
        (_, 0) => format!("{}s", gen_hundreds(h)),
        _ => format!("{} {}", gen_hundreds(h), ord_below_100(r)),
    }
}

/// The `teens_map` literal inside `_int_to_word`, indexed by `n - 11`.
const TEENS: [&str; 9] = [
    "üksteist",
    "kaksteist",
    "kolmteist",
    "neliteist",
    "viisteist",
    "kuusteist",
    "seitseteist",
    "kaheksateist",
    "üheksateist",
];

/// The `teens_ordinals` literal inside `_int_to_ordinal`, indexed by `n - 11`.
const TEENS_ORDINALS: [&str; 9] = [
    "üheteistkümnes",
    "kaheteistkümnes",
    "kolmeteistkümnes",
    "neljateistkümnes",
    "viieteistkümnes",
    "kuueteistkümnes",
    "seitsmeteistkümnes",
    "kaheksateistkümnes",
    "üheksateistkümnes",
];

/// The hundreds if/elif ladder in `_int_to_word`, indexed by the multiplier.
/// Index 0 is unreachable (the branch is guarded by `n >= 100`).
const HUNDREDS: [&str; 10] = [
    "",
    "ükssada",
    "kakssada",
    "kolmsada",
    "nelisada",
    "viissada",
    "kuussada",
    "seitsesada",
    "kaheksasada",
    "üheksasada",
];

const NEGWORD: &str = "miinus ";

fn big(n: i64) -> BigInt {
    BigInt::from(n)
}

/// `Num2Word_ET.CURRENCY_FORMS`, verbatim from the class body.
///
/// ET shadows the inherited attribute with its own dict, so — unlike the 16
/// classes that read `Num2Word_EUR`'s table — nothing `Num2Word_EN.__init__`
/// writes can reach it. These seven codes are the whole of it; every other code
/// raises (oddity 7).
///
/// Note SEK/NOK/DKK share one identical entry and RUB's unit tuple is
/// `("rubla", "rubla")` — both forms the same word. That is the Python data, not
/// a transcription slip: Estonian "rubla" is invariant here, and "ööri" is
/// likewise repeated on both sides.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Shared literals, exactly as the Python tuples repeat them.
    const SENT: [&str; 2] = ["sent", "senti"];
    const KROON: [&str; 2] = ["kroon", "krooni"];
    const OORI: [&str; 2] = ["ööri", "ööri"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["euro", "eurot"], &SENT));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollarit"], &SENT));
    m.insert("GBP", CurrencyForms::new(&["nael", "naela"], &["penn", "penni"]));
    m.insert("SEK", CurrencyForms::new(&KROON, &OORI));
    m.insert("NOK", CurrencyForms::new(&KROON, &OORI));
    m.insert("DKK", CurrencyForms::new(&KROON, &OORI));
    m.insert("RUB", CurrencyForms::new(&["rubla", "rubla"], &["kopikas", "kopikat"]));
    m
}

/// `cr[0] if n == 1 else cr[1]`, as ET spells it out inline.
///
/// Every ET entry has exactly two forms, so the miss is unreachable — but it is
/// mapped to `Index` rather than panicking, because that is the `IndexError`
/// Python would raise if the table ever grew a one-form entry.
fn index_form(forms: &[String], singular: bool) -> Result<&str> {
    forms
        .get(if singular { 0 } else { 1 })
        .map(String::as_str)
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

pub struct LangEt {
    /// `CURRENCY_FORMS`, built once in [`LangEt::new`] and only ever read.
    /// Rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangEt {
    pub fn new() -> Self {
        LangEt {
            currency_forms: build_currency_forms(),
        }
    }

    /// `_int_to_word`. `n` is always non-negative: the only callers are
    /// `_int_to_cardinal` (which strips the sign first) and this function's own
    /// recursion on quotients.
    fn int_to_word(&self, n: &BigInt) -> String {
        if n.is_zero() {
            return "null".to_string();
        }

        let mut parts: Vec<String> = Vec::new();
        let mut n = n.clone();

        // Each scale block is an independent `if` in Python that mutates `n`,
        // not an elif chain — so a value can fall through several of them.
        for &(scale, one_form, many_form) in &[
            (1_000_000_000_000i64, "üks triljon", "triljonit"),
            (1_000_000_000i64, "üks miljard", "miljardit"),
            (1_000_000i64, "üks miljon", "miljonit"),
        ] {
            let s = big(scale);
            if n >= s {
                let (q, r) = n.div_rem(&s);
                if q.is_one() {
                    parts.push(one_form.to_string());
                } else {
                    // Recurses on BigInt: for n >= 10^24 the trillions arm
                    // re-enters its own branch (bug 5).
                    parts.push(format!("{} {}", self.int_to_word(&q), many_form));
                }
                n = r;
            }
        }

        // Thousands: unlike the scales above, `1` yields a bare "tuhat" (no
        // "üks"), and the multiplier 100 is special-cased to "sada" (bug 6).
        let thousand = big(1000);
        if n >= thousand {
            let (q, r) = n.div_rem(&thousand);
            if q.is_one() {
                parts.push("tuhat".to_string());
            } else if q == big(100) {
                parts.push("sada tuhat".to_string());
            } else {
                parts.push(format!("{} tuhat", self.int_to_word(&q)));
            }
            n = r;
        }

        // Hundreds. `n < 1000` here, so the multiplier is 1..=9 and the Python
        // if/elif ladder is total.
        let hundred = big(100);
        if n >= hundred {
            let (q, r) = n.div_rem(&hundred);
            parts.push(HUNDREDS[q.to_usize().expect("bounded to 1..=9")].to_string());
            n = r;
        }

        // `n < 100` from here on, so a usize is provably wide enough.
        let small = n.to_usize().expect("bounded to 0..=99");
        if (11..=19).contains(&small) {
            parts.push(TEENS[small - 11].to_string());
        } else {
            // n == 10 lands here (the guard is `10 < n < 20`), giving "kümme".
            let mut small = small;
            if small >= 10 {
                parts.push(TENS[small / 10].to_string());
                small %= 10;
            }
            if small > 0 {
                parts.push(ONES[small].to_string());
            }
        }

        parts.join(" ")
    }

    /// `_int_to_cardinal`.
    fn int_to_cardinal(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok("null".to_string());
        }
        if n.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&-n)));
        }
        Ok(self.int_to_word(n))
    }

    /// `_int_to_ordinal`.
    fn int_to_ordinal(&self, n: &BigInt) -> Result<String> {
        if n.is_zero() {
            return Ok("nullis".to_string());
        }

        // Every negative would satisfy `n < 10` and wrap around in the list
        // index below (bug 2, #155): reject it like Base's verify_ordinal.
        verify_ordinal(n)?;
        // Compound ordinals: genitive components + ordinal last (#248).
        if let Some(v) = n.to_usize().filter(|v| *v < 1_000_000) {
            if v < 1000 {
                return Ok(ord_below_1000(v));
            }
            let (k, low) = (v / 1000, v % 1000);
            let head = if k == 1 {
                "tuhande".to_string()
            } else {
                format!("{} tuhande", gen_below_1000(k))
            };
            if low != 0 {
                return Ok(format!("{} {}", head, ord_below_1000(low)));
            }
            if k == 1 {
                return Ok("tuhandes".to_string());
            }
        }

        if *n == big(1_000_000) {
            return Ok("miljones".to_string());
        }

        // Round values without a rule: cardinal + "s" (bug 1, known gap).
        Ok(format!("{}s", self.int_to_cardinal(n)?))
    }

    /// `_int_to_word(n)` run on a **non-int** `n` (a float or Decimal), which
    /// is what `to_year` hands it — there is no try/except on that path, so
    /// the crash points are the observable behaviour:
    ///
    /// * Every arithmetic step (`//`, `%`, `==`, `>=`) works fine on
    ///   float/Decimal, and the `teens_map[n]` **dict** lookup succeeds for
    ///   integral values (`hash(11.0) == hash(11)`), so e.g. `11.0` renders
    ///   "üksteist" and `1e20` renders "ükssada miljonit triljonit".
    /// * The tens/ones arms index **lists** (`self.tens[tens_val]`,
    ///   `self.ones[n]`), and a float/Decimal index raises **TypeError** even
    ///   when integral — `to_year(5.0)`, `to_year(10.0)`, `to_year(21.0)` all
    ///   die there, while `to_year(100.0)`/`to_year(1000.0)` never reach a
    ///   list and survive.
    /// * A fractional value in the open interval (10, 20) misses the teens
    ///   dict: **KeyError**.
    ///
    /// The quotients Python computes (`n // scale`) are integral
    /// floats/Decimals, so the recursion stays in this simulation — a
    /// `12.0`-thousands multiplier still renders through the teens dict, and
    /// a `10.0` one still TypeErrors in the tens arm, exactly like CPython.
    /// Exact `BigDecimal` arithmetic stands in for both float and Decimal
    /// arms: every corpus value is exactly representable, and `//`/`%` on
    /// them are exact in Python too.
    fn year_word_sim(&self, n0: &BigDecimal) -> Result<String> {
        let mut n = n0.clone();
        let mut parts: Vec<String> = Vec::new();

        for (scale, one_form, many_form) in [
            (1_000_000_000_000i64, "üks triljon", "triljonit"),
            (1_000_000_000i64, "üks miljard", "miljardit"),
            (1_000_000i64, "üks miljon", "miljonit"),
        ] {
            let s = BigDecimal::from(scale);
            if n >= s {
                // `n // scale` — floor; n is non-negative here so truncation
                // (`with_scale(0)`) agrees.
                let q = (&n / &s).with_scale(0);
                if q == BigDecimal::from(1) {
                    parts.push(one_form.to_string());
                } else {
                    parts.push(format!("{} {}", self.year_word_sim(&q)?, many_form));
                }
                n = n - &q * &s;
            }
        }

        let thousand = BigDecimal::from(1000);
        if n >= thousand {
            let q = (&n / &thousand).with_scale(0);
            if q == BigDecimal::from(1) {
                parts.push("tuhat".to_string());
            } else if q == BigDecimal::from(100) {
                parts.push("sada tuhat".to_string());
            } else {
                parts.push(format!("{} tuhat", self.year_word_sim(&q)?));
            }
            n = n - &q * &thousand;
        }

        let hundred = BigDecimal::from(100);
        if n >= hundred {
            let q = (&n / &hundred).with_scale(0);
            // `hundreds == 1` … `== 9`: float comparison works, and n < 1000
            // here bounds the quotient to 1..=9.
            let i = q
                .as_bigint_and_exponent()
                .0
                .to_usize()
                .expect("bounded to 1..=9");
            parts.push(HUNDREDS[i].to_string());
            n = n - &q * &hundred;
        }

        let ten = BigDecimal::from(10);
        let twenty = BigDecimal::from(20);
        if n > ten && n < twenty {
            if n.is_integer() {
                let i = n
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0
                    .to_usize()
                    .expect("bounded to 11..=19");
                parts.push(TEENS[i - 11].to_string());
            } else {
                // `teens_map[10.5]` — a dict miss: KeyError, key repr'd.
                return Err(N2WError::Key(format!("{}", n)));
            }
        } else {
            if n >= ten {
                // `self.tens[tens_val]` with a float/Decimal index.
                return Err(N2WError::Type(
                    "list indices must be integers or slices, not float".into(),
                ));
            }
            if n > BigDecimal::from(0) {
                // `self.ones[n]` with a float/Decimal index.
                return Err(N2WError::Type(
                    "list indices must be integers or slices, not float".into(),
                ));
            }
        }

        Ok(parts.join(" "))
    }
}

impl Default for LangEt {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// triljon (10^12), so from 10^15 the module would stack it ("triljonit
/// triljonit").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

impl Lang for LangEt {
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
        " ja "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "koma"
    }

    // `cards`/`maxval`/`merge` intentionally left at their trait defaults:
    // Python never populates `self.cards` or `self.MAXVAL` for ET, and the
    // splitnum/clean engine is never reached. There is no overflow check.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python's `to_cardinal` tests for str/float first; for integral input
        // both tests fail and it drops straight to `_int_to_cardinal(int(n))`.
        Ok(self.int_to_cardinal(value)?)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // The `except BaseException` handler re-raises identically (bug 2), so
        // propagating the error unchanged is faithful.
        self.int_to_ordinal(value)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // All three Python branches have identical bodies.
        Ok(self.int_to_cardinal(value)?)
    }

    /// `Num2Word_ET.to_cardinal` for non-integer input.
    ///
    /// ET overrides `to_cardinal` outright rather than `to_cardinal_float`, so
    /// this reproduces the **inline** float handling of the Python method,
    /// which diverges from `base.to_cardinal_float` in several load-bearing
    /// ways:
    ///
    /// * **Digits render through `self.ones`, not `to_cardinal`.** In Python a
    ///   `0` digit is `self.ones[0]` — the empty filler string — so `0.05`
    ///   became "null koma  viis" (two spaces), indistinguishable from `0.5`
    ///   once the spaces collapse. The port says "null" for a zero digit, as
    ///   Base does (gladiaio/num2words2#205): "null koma null viis".
    /// * **The sign is prepended whenever `n < 0`**, with `pre = abs(pre)` —
    ///   not only when `pre == 0` as Base does. It uses the raw `negword`
    ///   ("miinus ", trailing space) directly, mirroring `self.negword`.
    /// * **The whole `post` string is spoken**, left-padded to but never
    ///   truncated at `precision` (`"0"*(precision-len)` is `""` when negative,
    ///   and `for digit in post_str` then walks every char).
    /// * **`self.pointword` is used raw** (no `title`); moot, as ET is not a
    ///   title language.
    ///
    /// `precision_override` is deliberately ignored: Python's `float2tuple`
    /// reassigns `self.precision` from the value's own `repr` before the
    /// padding loop reads it, clobbering any override the `num2words` wrapper
    /// set — issue #580 has no effect on ET's float path.
    ///
    /// A **`Decimal`** never entered the float branch in Python, because
    /// `isinstance(n, float)` is False for `Decimal`: `to_cardinal` truncated
    /// it, so `Decimal("12.345")` (and the string "12.345") was "kaksteist"
    /// (gladiaio/num2words2#156). A fractional Decimal now takes the same
    /// branch as a float, fed from `float2tuple`'s exact Decimal arm.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        // Ignored on purpose — see the doc above.
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Python only takes the fractional branch when `n != int(n)`; an
        // integer-valued value drops to `_int_to_cardinal(int(n))`.
        // `float2tuple` yields `pre == int(n)` there (and `post == 0`).
        let (pre, post) = float2tuple(value);
        let whole = match value {
            FloatValue::Float { value: f, .. } => *f == f.trunc(),
            FloatValue::Decimal { value: d, .. } => d.is_integer(),
        };
        if whole {
            return Ok(self.int_to_cardinal(&pre)?);
        }

        let precision = value.precision() as usize;
        let mut pre = pre;
        let mut result = String::new();

        // `if n < 0: result = self.negword; pre = abs(pre)`.
        if strictly_negative(value) {
            result.push_str(NEGWORD);
            pre = pre.abs();
        }
        result.push_str(&self.int_to_cardinal(&pre)?);

        if precision > 0 {
            // `result += " " + self.pointword` (raw, no title).
            result.push(' ');
            result.push_str(self.pointword());

            // `post_str = "0"*(precision-len) + post_str`. A negative
            // repeat count is "" in Python, so an over-long post is
            // spoken in full rather than truncated.
            let post_str = post.to_string();
            let pad = precision.saturating_sub(post_str.len());
            let padded = format!("{}{}", "0".repeat(pad), post_str);

            // `for digit in post_str: result += " " + self.ones[int(digit)]`,
            // except that `self.ones[0]` is the empty filler: a `0` digit
            // reads "null" (#205).
            for ch in padded.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("non-digit {:?} in fraction", ch))
                })? as usize;
                result.push(' ');
                result.push_str(if d == 0 { "null" } else { ONES[d] });
            }
        }

        // `return result.strip()`.
        Ok(result.trim().to_string())
    }

    /// `to_ordinal(float/Decimal)`. Python:
    ///
    /// ```python
    /// try:
    ///     return self._int_to_ordinal(n)          # n is the raw float/Decimal
    /// except BaseException:
    ///     return self._int_to_ordinal(int(n))     # truncated retry
    /// ```
    ///
    /// For every non-int input the two collapse to
    /// `_int_to_ordinal(int(n))`: the first call either raises (list indices
    /// TypeError for `n < 10` and the tens arm, teens-dict KeyError for a
    /// fractional 10..20) and is retried on `int(n)`, or it succeeds on a
    /// branch — `== 0/10/100/1000`, integral teens dict, or the cardinal+"s"
    /// arm — whose result equals the truncated-int one. So the port runs the
    /// integer `_int_to_ordinal` on the truncation directly:
    /// `2.5` -> "teine". Negatives (`-0.5` included, which would truncate to
    /// 0) raise `errmsg_negord` instead of wrapping around (bug 2, #155).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if strictly_negative(value) {
            return Err(negord_error(py_num_str(value)));
        }
        match value {
            FloatValue::Float { value: f, .. } if f.is_nan() => {
                // The first call survives every comparison (all False for
                // nan), lands in the cardinal+"s" arm, and _int_to_word(nan)
                // joins zero parts: "" + "s".
                return Ok("s".to_string());
            }
            FloatValue::Float { value: f, .. } if f.is_infinite() => {
                // inf >= 10**12 forever: _int_to_word recurses on inf//10**12
                // == inf until the interpreter kills it.
                return Err(N2WError::Custom {
                    module: "builtins",
                    class: "RecursionError",
                    msg: "maximum recursion depth exceeded".into(),
                });
            }
            _ => {}
        }
        let num = match value {
            FloatValue::Float { value, .. } => {
                BigInt::from_f64(value.trunc()).expect("finite after the guards above")
            }
            // `with_scale(0)` truncates toward zero — exactly `int(Decimal)`.
            FloatValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
        };
        self.int_to_ordinal(&num)
    }

    /// `to_ordinal_num(float/Decimal)` — `str(n) + "."`, no validation at
    /// all: "-0.0." / "1e+16." / "5.00." all pass through verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)` — all three Python branches call
    /// `_int_to_cardinal(n)` with the **raw** value and no try/except, so the
    /// result is [`LangEt::year_word_sim`]'s success-or-crash simulation:
    /// `-1000.0` -> "miinus tuhat", `11.0` -> "üksteist", but `5.0`, `10.0`,
    /// `21.0`, `12345.000` -> TypeError from a list index. `n == 0`
    /// (including `-0.0`) short-circuits to "null"; a negative value takes
    /// `negword + _int_to_word(-n)`.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let d = match value {
            FloatValue::Float { value: f, .. } => {
                if f.is_nan() {
                    // Every comparison is False: _int_to_word(nan) joins zero
                    // parts and to_year returns "".
                    return Ok(String::new());
                }
                if f.is_infinite() {
                    // The trillions arm recurses on inf forever.
                    return Err(N2WError::Custom {
                        module: "builtins",
                        class: "RecursionError",
                        msg: "maximum recursion depth exceeded".into(),
                    });
                }
                BigDecimal::from_f64(*f).expect("finite")
            }
            FloatValue::Decimal { value, .. } => value.clone(),
        };
        // The simulation recurses like `_int_to_word`, so it needs the same
        // ceiling (#203).
        check_maxval(&d.abs().with_scale(0).as_bigint_and_exponent().0, maxval_ceiling())?;
        if d.is_zero() {
            // `if n == 0: return "null"` — numeric, so -0.0 loses its sign.
            return Ok("null".to_string());
        }
        if d < BigDecimal::from(0) {
            // `self.negword + self._int_to_word(-n)` — negword is "miinus ".
            return Ok(format!("{}{}", NEGWORD, self.year_word_sim(&-d)?));
        }
        self.year_word_sim(&d)
    }

    // ---- currency -------------------------------------------------------
    //
    // ET overrides only `to_currency` and the forms table. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` are inherited from
    // `Num2Word_Base` unchanged, and the trait defaults already mirror them —
    // `money_verbose` routes back through this file's `to_cardinal`, which is
    // what makes the inherited `to_cheque` speak Estonian. `pluralize`,
    // `currency_adjective` and `currency_precision` are left at their defaults
    // because ET never reaches them (see the module doc).

    fn lang_name(&self) -> &str {
        "Num2Word_ET"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_ET.to_currency`.
    ///
    /// Deliberately does **not** delegate to `currency::default_to_currency`:
    /// ET's body differs from Base's in the error wording (oddity 7), in keying
    /// the cents segment on `isinstance(val, int)` rather than Base's
    /// `has_decimal` guard (oddity 8), in ignoring `adjective` (oddity 9), in
    /// hardcoding divisor 100 (oddity 10), in using bare `str()` instead of
    /// `_cents_terse` (oddity 11), and in inlining the form choice rather than
    /// calling `pluralize` (oddity 12).
    ///
    /// Its format string is `"%s%s %s%s%s %s"` — note there is **no** space
    /// around `separator`, unlike Base's `"%s%s %s%s %s %s"`. Both spaces come
    /// from the default separator `" ja "` itself, so the two must not be
    /// conflated.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Accepted and never read, exactly as in Python (oddity 9).
        _adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it through this language's own default (" ja ").
        let separator = separator.unwrap_or(self.default_separator());

        // `is_integer_input = isinstance(val, int)`. This alone decides whether
        // cents appear; `has_decimal` is never consulted (oddity 8).
        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // `has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0`, with the
        // 100 hardcoded (oddity 10). Python's Decimal `%` truncates toward zero,
        // as does `with_scale(0)`, so the two agree on negatives: -12.34 gives
        // `Decimal("-0.00")`, which is falsy — matching the zero difference here.
        let has_fractional_cents = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => {
                let scaled = value * BigDecimal::from(100);
                &scaled - scaled.with_scale(0) != BigDecimal::zero()
            }
        };

        // Python parses *before* the CURRENCY_FORMS lookup, so an unknown code
        // still parses first. Neither step can fail, but the order is kept.
        let (left, right, is_negative) =
            parse_currency_parts(val, false, has_fractional_cents, 100);

        // Note the message: no "code". See oddity 7.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // Hardcoded in Python, not `self.negword` — same string either way.
        let minus_str = if is_negative { "miinus " } else { "" };
        // `left` is always >= 0 (`parse_currency_parts` abs()es it), so this
        // takes `_int_to_cardinal`'s positive path.
        let money_str = self.int_to_cardinal(&left)?;
        let currency_str = index_form(cr1, left.is_one())?;

        // For integers, don't show cents.
        if is_integer_input {
            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // For floats, always show cents (even if zero).
        let cents_str = if cents {
            if has_fractional_cents {
                // `self.to_cardinal(float(right)) if right > 0 else "null"`.
                //
                // ET overrides `to_cardinal`'s float path with its own
                // (`float2tuple` + `pointword` + `self.ones[digit]`), whereas
                // `cardinal_from_decimal`'s default routes to Base's
                // `to_cardinal_float`, which renders each digit via
                // `to_cardinal`. Python's ET emitted a run of spaces for a 0
                // digit (`to_currency(1.01005)` was "üks euro ja üks koma   viis
                // senti"); both now say "null" (#205), so they agree.
                if right.is_positive() {
                    self.cardinal_from_decimal(&right)?
                } else {
                    "null".to_string()
                }
            } else {
                // `self._int_to_cardinal(right) if right > 0 else "null"`. The
                // guard is redundant — `_int_to_cardinal(0)` is "null" too — but
                // it is kept because it is what Python writes.
                let right_int = right.as_bigint_and_exponent().0;
                if right_int.is_positive() {
                    self.int_to_cardinal(&right_int)?
                } else {
                    "null".to_string()
                }
            }
        } else {
            // `str(float(right) if isinstance(right, Decimal) else right)`.
            // No zero padding (oddity 11).
            if has_fractional_cents {
                // `str(float(Decimal))`. `right` is a cent count in [0, 100)
                // whose fraction is non-zero whenever this branch is live, so
                // Rust's shortest-round-trip `{}` and Python's `repr` agree;
                // neither reaches exponent form, and neither can drop a ".0"
                // that the other keeps.
                let f = right.to_f64().ok_or_else(|| {
                    N2WError::Value(format!("cannot represent {} as f64", right))
                })?;
                format!("{}", f)
            } else {
                right.as_bigint_and_exponent().0.to_string()
            }
        };

        // `cr2[0] if right == 1 else cr2[1]`. Python compares the *Decimal* in
        // the fractional case, so this compares `BigDecimal`: at 1.011 EUR,
        // right is Decimal("1.100") — numerically 1.1, so it takes the plural,
        // while Decimal("1.000") would compare equal to 1 and take the singular.
        let cents_currency = index_form(cr2, right == BigDecimal::one())?;

        // "%s%s %s%s%s %s" — no space bracketing `separator`; see the doc above.
        Ok(format!(
            "{}{} {}{}{} {}",
            minus_str, money_str, currency_str, separator, cents_str, cents_currency
        ))
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use std::str::FromStr;

    fn f(l: &LangEt, value: f64, precision: u32) -> String {
        l.to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    // Decimal precision is unread on ET's path (it truncates to int), so any
    // value works here.
    fn d(l: &LangEt, s: &str) -> String {
        l.to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 0,
            },
            None,
        )
        .unwrap()
    }

    #[test]
    fn float_corpus_rows() {
        let l = LangEt::new();
        assert_eq!(f(&l, 0.5, 1), "null koma viis");
        assert_eq!(f(&l, 1.5, 1), "üks koma viis");
        assert_eq!(f(&l, 3.14, 2), "kolm koma üks neli");
        assert_eq!(f(&l, 12.34, 2), "kaksteist koma kolm neli");
        assert_eq!(f(&l, -12.34, 2), "miinus kaksteist koma kolm neli");
        assert_eq!(f(&l, 2.25, 2), "kaks koma kaks viis");
        assert_eq!(f(&l, 0.1, 1), "null koma üks");
        // f64-artefact + leading-zero-in-fraction rows (extra spaces on 0):
        assert_eq!(f(&l, 1.005, 3), "üks koma null null viis");
        assert_eq!(f(&l, 2.675, 3), "kaks koma kuus seitse viis");
        assert_eq!(f(&l, 0.01, 2), "null koma null üks");
        assert_eq!(f(&l, 1.01, 2), "üks koma null üks");
        assert_eq!(f(&l, 0.99, 2), "null koma üheksa üheksa");
        assert_eq!(f(&l, 99.99, 2), "üheksakümmend üheksa koma üheksa üheksa");
        assert_eq!(f(&l, 100.5, 1), "ükssada koma viis");
        assert_eq!(
            f(&l, 1234.56, 2),
            "tuhat kakssada kolmkümmend neli koma viis kuus"
        );
        assert_eq!(f(&l, -0.5, 1), "miinus null koma viis");
        assert_eq!(f(&l, -1.5, 1), "miinus üks koma viis");
        // Integer-valued floats drop to the int path.
        assert_eq!(f(&l, 1.0, 1), "üks");
        assert_eq!(f(&l, 0.0, 1), "null");
    }

    #[test]
    fn decimal_corpus_rows() {
        let l = LangEt::new();
        // Decimal never enters the fraction branch: int(value), fraction dropped.
        assert_eq!(d(&l, "0.01"), "null");
        assert_eq!(d(&l, "1.10"), "üks");
        assert_eq!(d(&l, "12.345"), "kaksteist");
        assert_eq!(d(&l, "0.001"), "null");
        assert_eq!(d(&l, "-12.34"), "miinus kaksteist");
        assert_eq!(
            d(&l, "98746251323029.99"),
            "üheksakümmend kaheksa triljonit seitsesada nelikümmend kuus \
             miljardit kakssada viiskümmend üks miljonit kolmsada kakskümmend \
             kolm tuhat kakskümmend üheksa"
        );
    }
}
