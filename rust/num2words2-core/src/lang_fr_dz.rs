//! Port of `lang_FR_DZ.py` (French — Algeria), registry key `"fr_DZ"`.
//!
//! Shape: **engine**. The full chain is
//! `Num2Word_FR_DZ` → `Num2Word_FR` → `Num2Word_EUR` → `Num2Word_Base`.
//!
//! `Num2Word_FR_DZ` overrides **only** `CURRENCY_FORMS` and `to_currency`. The
//! integer modes (`to_cardinal`/`to_ordinal`/`to_ordinal_num`/`to_year`)
//! therefore behave *identically to plain `fr`* — that half of this file is a
//! port of `Num2Word_FR` with the FR_DZ name. If `lang_fr.rs` and this file
//! ever disagree on those four modes, one of them is wrong.
//!
//! # The `super(Num2Word_FR, self)` MRO skip — the trap in this file
//!
//! `Num2Word_FR_DZ.to_currency` looks like a pass-through wrapper, but the
//! `super()` call names an explicit *start* class:
//!
//! ```python
//! def to_currency(self, val, currency="DZD", cents=True,
//!                 separator=" et", adjective=False):
//!     return super(Num2Word_FR, self).to_currency(...)   # NOT super()
//! ```
//!
//! `super(Num2Word_FR, self)` resolves to the entry *after* `Num2Word_FR` in
//! `type(self).__mro__`, i.e. `Num2Word_EUR` — which defines no `to_currency`
//! — so it lands on `Num2Word_Base.to_currency`. **`Num2Word_FR.to_currency` is
//! skipped entirely**, even though FR is this class's direct parent. Verified
//! against the live interpreter:
//! `super(Num2Word_FR, fr_dz).to_currency.__func__.__qualname__` is
//! `"Num2Word_Base.to_currency"`.
//!
//! That matters because FR's `to_currency` is not a superset of Base's — it
//! *differs observably*:
//!
//! * FR special-cases `isinstance(val, int)` and, on a **missing currency
//!   code, returns `self.to_cardinal(val)` instead of raising** — a silent
//!   fallback. Base raises `NotImplementedError`.
//! * FR's int branch ignores `adjective=` outright.
//!
//! The corpus pins the difference, and it is the single discriminator between
//! a correct port and a plausible-looking wrong one:
//!
//! ```text
//! fr    to_currency(1, "GBP") -> "un livre"            # FR: found in FR's table
//! fr    to_currency(1, "DZD") -> "un"                  # FR: KeyError -> to_cardinal
//! fr_DZ to_currency(1, "GBP") -> NotImplementedError   # Base: raises
//! fr_DZ to_currency(1, "DZD") -> "un dinar"
//! ```
//!
//! So `to_currency` is **not** overridden below: the trait default already
//! *is* `Num2Word_Base.to_currency` (`currency::default_to_currency`), which is
//! exactly what the MRO skip selects. The `currency="DZD"` / `separator=" et"`
//! kwarg defaults are carried by the generated `default_currency` /
//! `default_separator` hooks. Routing this through a hand-written copy of
//! `Num2Word_FR.to_currency` would turn all 24 `NotImplementedError` int rows
//! (GBP/JPY/KWD/BHD/INR/CNY/CHF × `0`,`1`,`2`,`100`,`1000000`) into bare
//! cardinals.
//!
//! # Currency data, as the live class actually sees it
//!
//! * `CURRENCY_FORMS` is FR_DZ's **own** class attribute — three codes only
//!   (DZD/EUR/USD). It shadows both `Num2Word_FR`'s 14-code table and
//!   `Num2Word_EUR`'s 23-code one, so every other code raises. Being its own
//!   dict, it is also immune to the `Num2Word_EN.__init__` in-place mutation
//!   that rewrites `Num2Word_EUR.CURRENCY_FORMS` at import time (see
//!   `PORTING_CURRENCY.md`) — EUR here is FR_DZ's `("euro", "euros")`, never
//!   EUR's `("euro", "euro")` nor EN's rewrite of it.
//! * `CURRENCY_ADJECTIVES` is **not** redefined, so the 16-entry
//!   `Num2Word_EUR` table is inherited (EN leaves it alone).
//! * `CURRENCY_PRECISION` is `{}` all the way up to `Num2Word_Base`, so
//!   `.get(code, 100)` is always 100 — the `currency_precision` hook keeps its
//!   default. There is no 3-decimal (KWD/BHD → 1000) or 0-decimal (JPY → 1)
//!   currency in this language: those codes are absent from the table and
//!   raise before precision is ever consulted.
//! * `pluralize` comes from `Num2Word_FR`, not `Num2Word_EUR`: `abs(n) <= 1`
//!   picks the singular, so **zero takes the singular** ("zéro euro et un
//!   centime"). EUR's `n == 1` rule would emit "zéro euros" and fail the
//!   corpus.
//! * `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
//!   all `Num2Word_Base`'s — the trait defaults already mirror them.
//!
//! Card table (built in `new()`, mirroring the Python `__init__` → `setup` →
//! `set_numwords` sequence):
//!   * `Num2Word_EUR.setup` builds 100 `high_numwords`
//!     (`["cent"] + gen_high_numwords(units, tens, lows)`).
//!   * `Num2Word_EUR.set_high_numwords` is **long scale**: `cap = 3 + 6*100 =
//!     603`, and each word `w` at exponent `n` contributes *two* cards —
//!     `10^n → w+"illiard"` and `10^(n-3) → w+"illion"`. So `10^6` is
//!     "million", `10^9` "milliard", `10^12` "billion", `10^15` "billiard",
//!     `10^18` "trillion", `10^21` "trilliard". (Contrast `lang_en.rs`, which
//!     overrides this to the short scale with step -3.)
//!   * Highest card is `10^603` ("centilliard"), so
//!     `MAXVAL = 1000 * 10^603 = 10^606`. Values are `BigInt` throughout —
//!     nothing here is bounded by `u64`/`i128`.
//!   * `Num2Word_FR.setup` supplies mid cards with **no 70 and no 90**: French
//!     builds those additively (60+10 → "soixante-dix", 80+10 →
//!     "quatre-vingt-dix"), which is what drives `merge`'s hyphen rules.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. All of the following are exactly what Python
//! emits and are pinned by rows in `bench/corpus.jsonl`:
//!
//! 1. `to_ordinal(0)` == **"zéroième"**. The `for...else` fallback only strips
//!    a trailing "e" before appending "ième"; "zéro" ends in "o", so the vowel
//!    survives. Real French is "zéroième"-less (there is no such word), but the
//!    corpus pins it.
//! 2. `to_ordinal_num(0)` == **"0me"** and `to_ordinal_num(2)` == "2me". The
//!    Python is a bare `"er" if value == 1 else "me"`, so it emits neither the
//!    correct French "0e"/"2e" nor "2ème" — and no feminine/plural variants.
//! 3. `to_ordinal(1000001)` == **"un million unième"** while
//!    `to_ordinal(1000000)` == "millionième". The big-unit branch only fires on
//!    an *exact* `"un " + unit` match or a `" <unit>s"` suffix, so any trailing
//!    remainder falls through to the generic suffixing. The Python source even
//!    carries a comment questioning this ("Is this right for such things as
//!    1001 ... ?"); it is preserved as-is.
//! 4. `word.endswith("ts") or word.endswith("ents")` in the fallback — the
//!    `"ents"` arm is dead code, fully subsumed by `"ts"`. Kept verbatim.
//! 5. `stripped = word.rstrip("s")` strips *every* trailing "s", not just one.
//!    Harmless for the current card words but reproduced exactly.
//! 6. `to_currency(1000000, "EUR")` == **"un million euros"**, not the correct
//!    French "un million d'euros". `Num2Word_Base.to_currency` just joins the
//!    cardinal to `pluralize(...)`, and nothing in the chain knows about the
//!    elided "de". Pinned by a corpus row.
//! 7. `to_cheque` emits the **English** "AND" between the words and the
//!    fraction — "MILLE DEUX CENT TRENTE-QUATRE AND 56/100 EUROS". The literal
//!    is hardcoded in `Num2Word_Base.to_cheque` and FR_DZ inherits it, so a
//!    French cheque comes out half-English. Pinned by a corpus row.
//! 8. `to_cheque`'s subunit is `int((abs_val - whole) * divisor)` — a
//!    *truncation*, unlike `to_currency`'s ROUND_HALF_UP. So `999.999 EUR`
//!    cheques as "…QUATRE-VINGT-DIX-NEUF AND 99/100 EUROS" (99 cents, not a
//!    carry to 1000) and `2.675 USD` as "67/100".
//!
//! # Deliberate raises
//!
//! `verify_ordinal` (inherited unchanged from `Num2Word_Base`) raises
//! `TypeError` for negatives, so `to_ordinal(-1)` and `to_ordinal_num(-1)` are
//! `N2WError::Type` — matching the corpus. `to_cardinal` is unaffected and
//! renders "moins un".
//!
//! An unknown currency code raises `NotImplementedError` from both
//! `to_currency` and `to_cheque` (`N2WError::NotImplemented`), with Python's
//! exact text — `Currency code "GBP" not implemented for "Num2Word_FR_DZ"`.
//! That is 84 of the 117 currency/cheque corpus rows, because the table has
//! only three codes. `pluralize`'s tuple index is the one latent
//! `IndexError`, unreachable with the current two-form entries. No
//! `KeyError`/`ValueError` crash sites exist in this chain.
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_FR`/`_FR_DZ` define no `_pending_*` handshake (contrast
//! `lang_ES.str_to_number`). Python writes `self.precision` on the
//! float/Decimal cardinal path, which fractional-cent inputs (`1.011 USD` →
//! "un dollar et un virgule un cents") reach via `cardinal_from_decimal`; the
//! Rust equivalent in `floatpath` threads precision through as an argument
//! instead, so nothing here is stateful. Safe to call statelessly.

use crate::base::{
    clean, set_low_numwords, set_mid_numwords, splitnum, Cards, Lang, N2WError, Node, Result,
};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_FR._BIG_UNITS`. Order is load-bearing: "trilliard" is
/// tested before "trillion", "billiard" before "billion", "milliard" before
/// "million", so the longer long-scale name always wins.
const BIG_UNITS: &[&str] = &[
    "trilliard",
    "trillion",
    "billiard",
    "billion",
    "milliard",
    "million",
];

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// `[u + t for t in tens for u in units]` — `tens` is the **outer** loop — then
/// reversed, then the Latin-prefix elisions are applied in order.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
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

/// `Num2Word_FR_DZ.CURRENCY_FORMS` — the class's own dict, verbatim.
///
/// Deliberately **not** merged with `Num2Word_FR`'s or `Num2Word_EUR`'s tables:
/// a subclass attribute shadows the parents' rather than extending them, so
/// these three codes are the *entire* table. Every other code — including the
/// GBP/JPY/KWD/BHD/INR/CNY/CHF the corpus probes — raises
/// `NotImplementedError`. Both sides carry two forms, which is all
/// `Num2Word_FR.pluralize` ever indexes.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "DZD",
        CurrencyForms::new(&["dinar", "dinars"], &["centime", "centimes"]),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "euros"], &["centime", "centimes"]),
    );
    m.insert("USD", CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged (FR, FR_DZ and EN
/// all leave it alone).
///
/// Ported whole because it is the inherited data, but only `USD` is reachable:
/// `to_currency` looks the code up in `CURRENCY_FORMS` first and raises for
/// anything missing, and USD is the only entry present in both tables. So
/// `adjective=True` can only ever produce "US dollar"/"US dollars".
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

pub struct LangFrDz {
    cards: Cards,
    maxval: BigInt,
    /// `Num2Word_FR.setup`'s `self.ords`. A `Vec` rather than a `HashMap`
    /// because Python iterates `self.ords.items()` in insertion order and
    /// `break`s on the first `endswith` hit.
    ords: Vec<(&'static str, &'static str)>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangFrDz {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFrDz {
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
        debug_assert_eq!(high.len(), 100);

        let mut cards = Cards::new();

        // --- Num2Word_EUR.set_high_numwords (long scale, step -6) ---
        // cap = 3 + 6*len(high); zip(high, range(cap, 3, -6)).
        // Both sequences have 100 entries, so the zip exhausts them together;
        // the `n <= 3` break mirrors range()'s exclusive stop.
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            // GIGA_SUFFIX then MEGA_SUFFIX, matching the Python insertion order
            // (descending, which is also `Cards`'s sort order).
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illiard", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illion", word),
            );
            n -= 6;
        }

        // --- Num2Word_FR.setup: mid/low numwords ---
        // Note the absence of 70 and 90 — French composes them additively.
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

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10^603 = 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangFrDz {
            cards,
            maxval,
            ords: vec![("cinq", "cinquième"), ("neuf", "neuvième")],
            exclude_title: vec!["et".into(), "virgule".into(), "moins".into()],
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // read these; rebuilding them per call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`not value == int(value)`) cannot fire for integral
    /// input, so only the negative check survives.
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

impl Lang for LangFrDz {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "DZD"
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

    /// `Num2Word_Base.to_cardinal`, re-implemented rather than delegated to
    /// `base::default_to_cardinal` for one reason: `Num2Word_FR.setup`
    /// overrides `errmsg_toobig` with a French message, and the overflow text
    /// is observable. Everything else is identical to the base engine.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            // Python: "%s " % self.negword.strip() → "moins " (strip then re-add
            // the single space), *not* the raw negword.
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

    /// Port of `Num2Word_FR.merge`.
    ///
    /// Python unpacks `ctext, cnum, ntext, nnum = curr + next` (tuple concat).
    ///
    /// The `cnum == 1` arm has no `return` when `nnum >= 1000000`: it falls
    /// through to the tail comparisons with `ctext`/`ntext` **unmodified**.
    /// That is what makes 10^6 render as "un million" while 100 renders as
    /// bare "cent". The structure below preserves that fall-through exactly.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let ten = BigInt::from(10);
        let eighty = BigInt::from(80);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            // else: fall through, no text fixups.
        } else {
            // Python: `not (cnum - 80) % 100 or (not cnum % 100 and cnum < 1000)`
            // — `%` binds tighter than `not`. mod_floor reproduces Python's
            // floor-mod; cnum is always >= 0 here, but (cnum - 80) can be
            // negative, so the distinction is real.
            let drop_s = (cnum - &eighty).mod_floor(&hundred).is_zero()
                || (cnum.mod_floor(&hundred).is_zero() && cnum < &thousand);
            // `ctext[-1]` is a *character* index in Python — French words carry
            // non-ASCII ("zéro"), so never index by byte.
            if drop_s && nnum < &million && ctext.chars().last() == Some('s') {
                ctext.pop(); // ctext[:-1]
            }
            // Pluralize a round `next` ("deux cents", "neuf millions") — but
            // never "mille" (nnum != 1000), and never double up an existing 's'
            // ("cent vingt-trois" stays put).
            if cnum < &thousand
                && nnum != &thousand
                && ntext.chars().last() != Some('s')
                && nnum.mod_floor(&hundred).is_zero()
            {
                ntext.push('s');
            }
        }

        // Python's chained `nnum < cnum < 100`.
        if nnum < cnum && cnum < &hundred {
            // "vingt-et-un", "soixante-et-onze" — but 80 is excluded, giving
            // "quatre-vingt-un" and "quatre-vingt-onze".
            if nnum.mod_floor(&ten).is_one() && cnum != &eighty {
                return (format!("{}-et-{}", ctext, ntext), cnum + nnum);
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
        let mut word = self.to_cardinal(value)?;

        // Big-unit ordinals: "un million" → "millionième", "dix millions" →
        // "dix millionième". Only exact-`un` or plural-suffix forms qualify;
        // "un million un" deliberately does not (see module docs, bug 3).
        for unit in BIG_UNITS {
            if word == format!("un {}", unit) || word.ends_with(&format!(" {}s", unit)) {
                // rstrip("s") removes *all* trailing 's', not just one.
                let mut stripped = word.trim_end_matches('s').to_string();
                if let Some(rest) = stripped.strip_prefix("un ") {
                    stripped = rest.to_string();
                }
                // The plural "s" of "millions" is an agreement marker and
                // drops in the ordinal, as for "cents"/"vingts" (#65, #165):
                // "deux millionième", not "deux millionièmes".
                return Ok(format!("{}ième", stripped));
            }
            // Unreachable in practice: a bare unit word never escapes merge
            // without its "un ". Ported for completeness.
            if word == *unit {
                return Ok(format!("{}ième", unit));
            }
        }

        // Python `for src, repl in self.ords.items(): ... break` / `else:`.
        // `matched` models the for/else: the else arm runs only if no break.
        let mut matched = false;
        for (src, repl) in &self.ords {
            if word.ends_with(src) {
                // `src` is pure ASCII and `ends_with` guarantees the boundary,
                // so slicing off `src.len()` bytes == Python's `word[:-len(src)]`.
                word = format!("{}{}", &word[..word.len() - src.len()], repl);
                matched = true;
                break;
            }
        }
        if !matched {
            if word.chars().last() == Some('e') {
                word.pop(); // word[:-1]
            }
            // Drop the trailing 's' of "cents"/"vingts": 200 → "deux centième".
            // The "ents" arm is dead code in Python; kept verbatim.
            if word.ends_with("ts") || word.ends_with("ents") {
                word.pop();
            }
            word.push_str("ième");
        }
        Ok(word)
    }

    /// Port of `Num2Word_FR.to_ordinal_num`: `str(value) + ("er" | "me")`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut out = value.to_string();
        out.push_str(if value.is_one() { "er" } else { "me" });
        Ok(out)
    }

    /// Port of `Num2Word_FR.to_year`: plain cardinal, no era suffix and no
    /// century splitting (contrast EN's "nineteen oh-five").
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float/negative checks and `to_year`'s
    // `int()` truncation become reachable here — unlike on the BigInt hooks
    // above, where they are dead code. `to_cardinal` needs no override: the
    // FR_DZ → FR → EUR → Base chain inherits base's `assert int(value) ==
    // value` routing, which is exactly the trait default (whole -> int path).

    /// `Num2Word_FR.to_ordinal(float/Decimal)` (inherited by FR_DZ):
    /// `verify_ordinal`, then the integer path. Whole values ordinalise
    /// (5.0 -> "cinquième", 1.0 -> "premier" via the `value == 1` special
    /// case, Decimal("1E+2") -> "centième", 1e+16 -> "dix
    /// billiardième"); fractional or negative values raise TypeError.
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
    /// `Num2Word_FR_DZ`: idiomatic demi/tiers/quart for denominators 2/3/4 —
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

    // ---- currency -------------------------------------------------------
    //
    // Only the two data tables, the class name and FR's plural rule are
    // language-specific. `to_currency` (via the `super(Num2Word_FR, self)` MRO
    // skip), `to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse`
    // and `CURRENCY_PRECISION.get(code, 100)` all resolve to `Num2Word_Base`,
    // which the trait defaults already mirror — so they are left alone. See
    // the module docs for why overriding `to_currency` here would be a bug.

    fn lang_name(&self) -> &str {
        "Num2Word_FR_DZ"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// Port of `Num2Word_FR.pluralize`: `forms[0] if abs(n) <= 1 else forms[1]`.
    ///
    /// Not `Num2Word_EUR.pluralize` (`forms[0 if n == 1 else 1]`) — FR shadows
    /// it, and the difference is observable at zero: French takes the singular
    /// there, so `0.01 EUR` is "zéro euro et un centime", not "zéro euros …".
    /// `abs()` is faithful rather than load-bearing: `to_currency` only ever
    /// passes non-negative counts (`abs(val_int)`, and `parse_currency_parts`
    /// returns the sign separately).
    ///
    /// Python indexes the tuple directly, so a one-form entry with `abs(n) > 1`
    /// would raise IndexError. All three FR_DZ entries carry two forms, so this
    /// is unreachable — mapped to `Index` rather than panicking so the
    /// exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.abs() <= BigInt::one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
