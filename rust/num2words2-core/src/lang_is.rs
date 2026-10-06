//! Port of `lang_IS.py` (Icelandic), via `lang_EUR` → `Num2Word_Base`.
//!
//! Registry check: `__init__.py` maps `"is"` → `lang_IS.Num2Word_IS()`.
//!
//! Shape: **engine**. `Num2Word_IS.setup` defines `high_numwords` /
//! `mid_numwords` / `low_numwords` and the class overrides `merge`, so
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean` unchanged. Only
//! `to_ordinal`, `to_ordinal_num` and `to_year` are overridden.
//!
//! # Card construction
//!
//! `setup` calls `gen_high_numwords([], [], lows)` with **empty** `units` and
//! `tens`. The comprehension `[u + t for t in tens for u in units]` over an
//! empty `tens` yields `[]`, every elision replacement is a no-op on the empty
//! list, and the function returns `[] + lows` — i.e. `high_numwords` is just
//! `lows` verbatim. IS therefore skips the whole Latin-prefix machinery that
//! EN/EUR use; there is no `gen_high_numwords` helper needed here.
//!
//! `Num2Word_EUR.set_high_numwords` then pairs those 8 stems with
//! `range(3 + 6*8, 3, -6)` = 51, 45, …, 9, emitting **both** an illjarður
//! (long-scale "-illiard") card at `10**n` and an illjón card at `10**(n-3)`:
//!
//! ```text
//! 10^51 oktilljarður   10^48 oktilljón
//! 10^45 septilljarður  10^42 septilljón
//! 10^39 sextilljarður  10^36 sextilljón
//! 10^33 kvintilljarður 10^30 kvintilljón
//! 10^27 kvaðrilljarður 10^24 kvaðrilljón
//! 10^21 trilljarður    10^18 trilljón
//! 10^15 billjarður     10^12 billjón
//! 10^9  milljarður     10^6  milljón
//! ```
//!
//! `MAXVAL = 1000 * list(self.cards.keys())[0]`. Python's `cards` is an
//! `OrderedDict` and the first key inserted is `10**51`, so `MAXVAL == 10**54`.
//! (`Cards::highest()` returns the same value because IS's insertion order —
//! high descending, then mid descending, then low descending — is already
//! sorted descending.)
//!
//! # Notes on faithfulness
//!
//! * `is_title` is never set by IS, so it stays `False` from
//!   `Num2Word_Base.__init__` and `title()` is an identity. `exclude_title` is
//!   populated (`["og", "komma", "mínus"]`) but is dead weight — reproduced
//!   here anyway so the field matches the Python object.
//! * `genderize` uses `adj.replace(last, ...)`, a **substring** replace over
//!   the whole adjective phrase, not a replace of the final token only. With
//!   the real card vocabulary the last token is the only occurrence, so the
//!   two coincide — but the substring semantics are what is ported.
//! * `to_ordinal` is a "simplified implementation" (the Python comment says so)
//!   and produces plainly wrong Icelandic for most composite numbers. See the
//!   bug list on [`LangIs::to_ordinal`].
//!
//! # Currency
//!
//! IS declares its **own** `CURRENCY_FORMS` class dict, so the
//! `Num2Word_EN.__init__` mutation that rewrites `Num2Word_EUR`'s shared table
//! in place never reaches it. Verified against the live interpreter:
//! `Num2Word_IS.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` → `False`, and
//! the instance view holds exactly ISK/EUR/USD. Every other code — including
//! the ~24 that EN adds to EUR's dict — raises `NotImplementedError`.
//!
//! `CURRENCY_PRECISION` is *not* overridden (IS inherits `Num2Word_Base`'s
//! empty dict, confirmed live), so `.get(code, 100)` is always 100 and the
//! trait's default `currency_precision` is already exact. There is no
//! 3-decimal or 0-decimal path here: KWD/BHD/JPY are absent from the forms
//! table and raise before precision is ever consulted.
//!
//! `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` are all
//! inherited from `Num2Word_Base` unchanged, so their trait defaults stand.
//!
//! ## The tuple leak
//!
//! `Num2Word_IS.pluralize(self, n, noun)` **shadows**
//! `Num2Word_EUR.pluralize(self, n, forms)` with an incompatible contract: it
//! expects a single noun *string*, but `Num2Word_Base.to_currency` — which IS
//! delegates every float to — calls `self.pluralize(left, cr1)` with the
//! `CURRENCY_FORMS` **tuple**. Python's duck typing lets that through, every
//! branch falls out returning the tuple untouched, and the `"%s"` template
//! then stringifies the tuple itself:
//!
//! ```text
//! num2words(12.34, lang="is", to="currency", currency="EUR")
//!   -> "tólf ('evra', 'evrur'), þrjátíu og fjórir ('sent', 'sent')"
//! ```
//!
//! That is a genuine upstream bug and it is what the corpus pins, so
//! [`LangIs::pluralize`] reproduces the tuple repr byte for byte. Note the
//! asymmetry it creates: `to_currency(2, ...)` (a true `int`) never reaches
//! `pluralize` — IS's own override indexes `cr1` directly — so ints print
//! clean ("tveir evrur") while floats leak.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// `Num2Word_IS.GIGA_SUFFIX` — overrides EUR's "illiard".
const GIGA_SUFFIX: &str = "illjarður";
/// `Num2Word_IS.MEGA_SUFFIX` — overrides EUR's "illion".
const MEGA_SUFFIX: &str = "illjón";

// Gender indices, mirroring the module-level constants in lang_IS.py.
const KK: usize = 0; // Karlkyn (male)
const KVK: usize = 1; // Kvenkyn (female)
const HK: usize = 2; // Hvorugkyn (neuter)

/// `GENDERS`: numeral → (karlkyn, kvenkyn, hvorugkyn).
///
/// A slice rather than a map: four entries, and lookup is a linear scan in
/// both languages' hot paths anyway.
const GENDERS: &[(&str, [&str; 3])] = &[
    ("einn", ["einn", "ein", "eitt"]),
    ("tveir", ["tveir", "tvær", "tvö"]),
    ("þrír", ["þrír", "þrjár", "þrjú"]),
    ("fjórir", ["fjórir", "fjórar", "fjögur"]),
];

/// `PLURALS`: noun → (singular, plural). Only "hundrað" is listed.
const PLURALS: &[(&str, [&str; 2])] = &[("hundrað", ["hundrað", "hundruð"])];

fn gender_forms(word: &str) -> Option<&'static [&'static str; 3]> {
    GENDERS.iter().find(|(k, _)| *k == word).map(|(_, v)| v)
}

fn plural_forms(word: &str) -> Option<&'static [&'static str; 2]> {
    PLURALS.iter().find(|(k, _)| *k == word).map(|(_, v)| v)
}

/// Python's `str()` of a tuple whose elements are all `str`.
///
/// Required because `Num2Word_IS.pluralize` returns `base.to_currency`'s
/// `cr1`/`cr2` tuple unchanged and the `"%s"` template stringifies it (see the
/// module docs). This is the *rendering* half of that bug.
///
/// Element quoting is the plain `'…'` form. Python's `repr(str)` would switch
/// to `"…"` for a string containing an apostrophe, and would escape
/// backslashes and non-printables — but IS's currency vocabulary is a closed
/// set of nine plain-letter words, optionally prefixed by a
/// `CURRENCY_ADJECTIVES` value ("íslenskar ", "US "), so none of those cases
/// can arise. Non-ASCII stays literal in both languages (Python since PEP
/// 3138), so "króna" renders as `'króna'`, not `'kr\xf3na'`.
fn py_tuple_repr(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| format!("'{}'", s)).collect();
    match inner.len() {
        0 => "()".to_string(),
        // Python disambiguates a 1-tuple with a trailing comma: `('x',)`.
        1 => format!("({},)", inner[0]),
        _ => format!("({})", inner.join(", ")),
    }
}

/// `Num2Word_IS.CURRENCY_FORMS`, the class body verbatim.
///
/// IS's own dict, *not* the EUR table EN mutates — see the module docs. Note
/// it also disagrees with EUR on ISK's subunit: EUR says `("aur", "aurar")`,
/// IS says `("eyrir", "aurar")`. IS wins; EUR's entry is unreachable here.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "ISK",
        CurrencyForms::new(&["króna", "krónur"], &["eyrir", "aurar"]),
    );
    m.insert("EUR", CurrencyForms::new(&["evra", "evrur"], &["sent", "sent"]));
    m.insert("USD", CurrencyForms::new(&["dalur", "dalir"], &["sent", "sent"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged.
///
/// IS defines no `CURRENCY_ADJECTIVES` of its own, and — unlike
/// `CURRENCY_FORMS` — nothing mutates EUR's copy, so the class-body literal is
/// what runs (dumped live to confirm all 16 entries).
///
/// Almost all of it is dead weight, reproduced because it is what the object
/// carries:
///
/// * `adjective=` is consulted only *after* the `CURRENCY_FORMS` lookup, so
///   the 14 codes outside ISK/EUR/USD raise `NotImplementedError` first.
/// * `Num2Word_IS.to_currency`'s int branch ignores `adjective` outright — it
///   never calls `prefix_currency` — so adjectives reach only the float path.
/// * EUR is absent from the table, so `adjective=True` on EUR is a silent
///   no-op.
///
/// That leaves ISK ("íslenskar") and USD ("US") on floats as the only live
/// entries.
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

pub struct LangIs {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangIs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangIs {
    pub fn new() -> Self {
        // setup(): lows == high_numwords, because gen_high_numwords([], [], lows)
        // degenerates to `[] + lows` (see module docs).
        let high = ["okt", "sept", "sext", "kvint", "kvaðr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords:
        //   cap = 3 + 6 * len(high)              -> 51
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**n]     = word + GIGA_SUFFIX
        //       cards[10**(n-3)] = word + MEGA_SUFFIX
        // zip() stops at the shorter sequence; here both have length 8, so the
        // `n > 3` guard below is belt-and-braces that mirrors range's stop.
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(
                BigInt::from(10u8).pow(n as u32),
                format!("{}{}", word, GIGA_SUFFIX),
            );
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}{}", word, MEGA_SUFFIX),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "þúsund"),
                (100, "hundrað"),
                (90, "níutíu"),
                (80, "áttatíu"),
                (70, "sjötíu"),
                (60, "sextíu"),
                (50, "fimmtíu"),
                (40, "fjörutíu"),
                (30, "þrjátíu"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "tuttugu", "nítján", "átján", "sautján", "sextán", "fimmtán", "fjórtán",
                "þrettán", "tólf", "ellefu", "tíu", "níu", "átta", "sjö", "sex", "fimm",
                "fjórir", "þrír", "tveir", "einn", "núll",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0] == 1000 * 10**51 == 10**54.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<&'static str, &'static str> = [
            ("einn", "fyrsti"),
            ("tveir", "annar"),
            ("þrír", "þriðji"),
            ("fjórir", "fjórði"),
            ("fimm", "fimmti"),
            ("sex", "sjötti"),
            ("sjö", "sjöundi"),
            ("átta", "áttundi"),
            ("níu", "níundi"),
            ("tíu", "tíundi"),
            ("ellefu", "ellefti"),
            ("tólf", "tólfti"),
        ]
        .into_iter()
        .collect();

        LangIs {
            cards,
            maxval,
            ords,
            exclude_title: vec!["og".into(), "komma".into(), "mínus".into()],
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// Port of `Num2Word_IS.pluralize(n, noun)` **for the string call site**.
    ///
    /// This shadows `Num2Word_EUR.pluralize(n, forms)` with a *different*
    /// contract — IS takes a single noun string, not a tuple of forms — and
    /// Python has only the one method serving both call sites. The two are
    /// split here because the argument types are irreconcilable in Rust:
    ///
    /// * `merge` (cardinal path) passes a card word → this inherent method.
    /// * `base.to_currency` (currency path) passes the `CURRENCY_FORMS` tuple
    ///   → the [`Lang::pluralize`] impl, which reproduces the tuple leak.
    ///
    /// Inherent methods win name resolution over trait methods, so `merge`'s
    /// `self.pluralize(lnum, rtext)` binds here on the `&str` argument while
    /// `default_to_currency`'s generic `lang.pluralize(&n, &forms)` binds to
    /// the trait. The dispatch matches Python's duck typing by construction.
    ///
    /// (`Num2Word_IS.pluralize_currency` looks like it ought to serve the
    /// currency path, but nothing ever calls it — `base.to_currency` calls
    /// `pluralize`, not `pluralize_currency`. It is dead code in Python and is
    /// therefore not ported.)
    fn pluralize(&self, n: &BigInt, noun: &str) -> String {
        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        // form = 0 if (n % 10 == 1 and n % 100 != 11) else 1
        let form = if n.mod_floor(&ten).is_one() && n.mod_floor(&hundred) != BigInt::from(11) {
            0usize
        } else {
            1usize
        };
        if form == 0 {
            return noun.to_string();
        }
        if noun.contains(GIGA_SUFFIX) {
            return noun.replace(GIGA_SUFFIX, "illjarðar");
        }
        if noun.contains(MEGA_SUFFIX) {
            return noun.replace(MEGA_SUFFIX, "illjónir");
        }
        match plural_forms(noun) {
            None => noun.to_string(),
            Some(forms) => forms[form].to_string(),
        }
    }

    /// Port of `Num2Word_IS.genderize(adj, noun)`.
    ///
    /// Python's `adj.split()[-1]` would raise `IndexError` on an all-whitespace
    /// `adj`. That is unreachable: every card word is non-empty and `merge`
    /// only ever builds `"a b"` / `"a og b"` from non-empty parts. The
    /// `unwrap_or("")` below therefore never fires; `""` is not in `GENDERS`,
    /// so it would fall through to the identity return regardless.
    fn genderize(&self, adj: &str, noun: &str) -> String {
        let last = match adj.split_whitespace().last() {
            Some(w) => w,
            None => "",
        };
        let forms = match gender_forms(last) {
            Some(f) => f,
            None => return adj.to_string(),
        };
        // Default KK; the `elif "illjarð"` arm is redundant in the Python but
        // kept for a 1:1 reading of the branch order.
        let gender = if noun.contains("hund") || noun.contains("þús") {
            HK
        } else if noun.contains("illjarð") {
            KK
        } else if noun.contains("illjón") {
            KVK
        } else {
            KK
        };
        // Python: adj.replace(last, GENDERS[last][gender]) — replaces every
        // occurrence of the token as a substring, not just the final word.
        adj.replace(last, forms[gender])
    }
}

impl Lang for LangIs {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ISK"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    /// For the `Currency code "X" not implemented for "Y"` message. Python
    /// interpolates `self.__class__.__name__`, so it names IS even when the
    /// raise happens inside `Num2Word_Base.to_currency` after IS delegates.
    fn lang_name(&self) -> &str {
        "Num2Word_IS"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    // currency_precision is NOT overridden: IS inherits Num2Word_Base's empty
    // CURRENCY_PRECISION, so `.get(code, 100)` is always 100 — exactly the
    // trait default. Likewise money_verbose / cents_verbose / cents_terse /
    // to_cheque, which IS inherits from Num2Word_Base unchanged.

    /// Port of `Num2Word_IS.pluralize(n, noun)` **for the tuple call site** —
    /// i.e. the one `Num2Word_Base.to_currency` reaches. See the module docs;
    /// the short version is that this always returns the tuple's `str()`.
    ///
    /// Walking the Python with `noun` bound to a `CURRENCY_FORMS` tuple:
    ///
    /// ```text
    /// form = 0 if (n % 10 == 1 and n % 100 != 11) else 1
    /// if form == 0:                  return noun        # the tuple
    /// elif self.GIGA_SUFFIX in noun: return noun.replace(...)
    /// elif self.MEGA_SUFFIX in noun: return noun.replace(...)
    /// elif noun not in PLURALS:      return noun        # the tuple
    /// return PLURALS[noun][form]
    /// ```
    ///
    /// Both live arms return `noun` unchanged, so the count `n` has no effect
    /// whatsoever — corpus row `0.01 EUR` proves it, taking `form == 1` for
    /// the unit and `form == 0` for the subunit and leaking both tuples.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let form = if n.mod_floor(&ten).is_one() && n.mod_floor(&hundred) != BigInt::from(11) {
            0usize
        } else {
            1usize
        };
        if form == 0 {
            return Ok(py_tuple_repr(forms));
        }
        // `self.GIGA_SUFFIX in noun` is a *substring* test when noun is a str
        // (the merge path) but *element equality* when noun is a tuple — this
        // path. It could only fire if a currency form were literally
        // "illjarður"/"illjón", and the body would then raise AttributeError,
        // because tuples have no `.replace`. Unreachable with IS's table;
        // ported rather than dropped so the shape survives a table change.
        if forms.iter().any(|f| f == GIGA_SUFFIX || f == MEGA_SUFFIX) {
            return Err(N2WError::Attribute(
                "'tuple' object has no attribute 'replace'".into(),
            ));
        }
        // `elif noun not in PLURALS` — PLURALS is keyed by str, so a tuple key
        // never hits, the test is always true, and the tuple falls straight
        // back out. `PLURALS[noun][form]` below it is dead.
        Ok(py_tuple_repr(forms))
    }

    /// Port of `Num2Word_IS.to_currency`.
    ///
    /// IS intercepts only the true-`int` case and hands everything else to
    /// `Num2Word_Base.to_currency` verbatim. The int branch is hand-rolled and
    /// diverges from Base in three observable ways:
    ///
    /// 1. It indexes `cr1` directly instead of calling `pluralize`, so ints
    ///    escape the tuple leak that floats suffer.
    /// 2. Python uses `self.negword` (`"mínus "`) **unstripped**, so the
    ///    `"%s %s %s"` template emits a double space: `"mínus tólf evrur"`.
    ///    Fixed here (#160): the port strips it and says `"mínus tólf evrur"`.
    /// 3. It ignores `adjective=` entirely — no `prefix_currency` call — so
    ///    `to_currency(2, currency="ISK", adjective=True)` == `"tveir krónur"`,
    ///    with no "íslenskar". Floats do apply the adjective.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Python: `if isinstance(val, int)` — a real int, never a whole float.
        // `1` prints no cents; `1.0` prints "núll ('sent', 'sent')".
        if let CurrencyValue::Int(v) = val {
            // try:    cr1, cr2 = self.CURRENCY_FORMS[currency]
            // except (KeyError, AttributeError):
            //         return super().to_currency(...)
            //
            // Only the KeyError half is live (CURRENCY_FORMS is a class attr
            // that always exists), and it maps to `None` here. super() then
            // re-raises the same miss as NotImplementedError, so an unknown
            // code still errors rather than falling back to anything.
            if let Some(forms) = self.currency_forms(currency) {
                let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
                let abs_val = v.abs();
                let money_str = self.to_cardinal(&abs_val)?;

                // if abs_val == 1: cr1[0]
                // else:            cr1[1] if len(cr1) > 1 else cr1[0]
                // The `isinstance(cr1, tuple)` guards are always true for IS's
                // table, so they collapse into the tuple arms.
                let currency_str = if abs_val.is_one() {
                    forms.unit.first()
                } else {
                    forms.unit.get(1).or_else(|| forms.unit.first())
                };
                // `cr1[0]` on an empty tuple would be IndexError. IS's table
                // has no empty entry, so this cannot fire.
                let currency_str = currency_str
                    .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

                // ("%s %s %s" % (minus_str, money_str, currency_str)).strip()
                return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                    .trim()
                    .to_string());
            }
        }

        // `return super(Num2Word_IS, self).to_currency(val, ...)` — for floats
        // unconditionally, and for ints whose code missed the table.
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "mínus "
    }
    fn pointword(&self) -> &str {
        "komma"
    }
    // is_title stays false (Num2Word_Base.__init__ default; IS never sets it),
    // so `title()` is an identity and `exclude_title` is never consulted.
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_IS.merge`.
    ///
    /// ```text
    /// if lnum == 1 and rnum < 100:            -> (rtext, rnum)
    /// elif lnum < rnum:                       -> "L R"    , lnum * rnum
    /// elif lnum > rnum and rnum in self.cards -> "L og R" , lnum + rnum
    /// else:                                   -> "L R"    , lnum + rnum
    /// ```
    ///
    /// Both middle arms pluralize the right word against the *left* count and
    /// then genderize the left phrase against the *pluralized* right word —
    /// order matters, since e.g. `genderize("tveir", "hundruð")` must see the
    /// already-pluralized "hundruð" (it matches on the "hund" substring, so the
    /// singular would work too, but "milljón" → "milljónir" does not commute
    /// with the "illjón" test in general).
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);

        if lnum.is_one() && rnum < &hundred {
            return (rtext.to_string(), rnum.clone());
        }
        if lnum < rnum {
            let rtext = self.pluralize(lnum, rtext);
            let ltext = self.genderize(ltext, &rtext);
            return (format!("{} {}", ltext, rtext), lnum * rnum);
        }
        if lnum > rnum && self.cards.get(rnum).is_some() {
            let rtext = self.pluralize(lnum, rtext);
            let ltext = self.genderize(ltext, &rtext);
            return (format!("{} og {}", ltext, rtext), lnum + rnum);
        }
        (format!("{} {}", ltext, rtext), lnum + rnum)
    }

    /// Port of `Num2Word_IS.to_ordinal`.
    ///
    /// The Python `try: number = int(value) except (ValueError, TypeError):
    /// return str(value)` guard is a no-op for the integer-only path we
    /// support, so it is omitted.
    ///
    /// # Faithfully reproduced Python bugs
    ///
    /// The Python author labels this "a simplified implementation". It is
    /// wrong Icelandic almost everywhere outside the hardcoded values, and all
    /// of the following are verbatim corpus expectations:
    ///
    /// 1. The catch-all `cardinal + "asti"` glues the suffix onto whatever the
    ///    cardinal ends with, producing non-words: `to_ordinal(31)` ==
    ///    "þrjátíu og einnasti", `to_ordinal(42)` == "fjörutíu og tveirasti",
    ///    `to_ordinal(200)` == "tvö hundruðasti", `to_ordinal(10**7)` ==
    ///    "tíu milljónirasti".
    /// 2. `to_ordinal(0)` == "núllasti" — a "zeroth" that no dictionary has.
    /// 3. **Negatives are not rejected.** `Num2Word_Base.verify_ordinal` is
    ///    never called, so `to_ordinal(-1)` == "mínus einnasti" instead of
    ///    raising `TypeError` the way most languages do. There is no error
    ///    path here at all beyond `to_cardinal`'s overflow check.
    /// 4. The 20..30 arm hardcodes the tens word but reuses `self.ords` for the
    ///    unit, so 21 → "tuttugasti og fyrsti" is right while 31 (bug 1) is
    ///    not — the asymmetry is real.
    /// 5. Only the *exact* values 30/40/…/90/100/1000 get a proper ordinal;
    ///    101, 1100, 2000 etc. all fall to the "asti" catch-all.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;

        if let Some(o) = self.ords.get(cardinal.as_str()) {
            return Ok((*o).to_string());
        }

        let twenty = BigInt::from(20);
        let thirty = BigInt::from(30);

        if value == &twenty {
            return Ok("tuttugasti".to_string());
        }
        if value > &twenty && value < &thirty {
            // self.ords.get(self.to_cardinal(number % 10),
            //               self.to_cardinal(number % 10))
            let unit = self.to_cardinal(&value.mod_floor(&BigInt::from(10)))?;
            let tail = match self.ords.get(unit.as_str()) {
                Some(o) => (*o).to_string(),
                None => unit,
            };
            return Ok(format!("tuttugasti og {}", tail));
        }

        // The exact-match ladder, in Python's order.
        for (n, word) in [
            (30i64, "þrítugasti"),
            (40, "fertugasti"),
            (50, "fimmtugasti"),
            (60, "sextugasti"),
            (70, "sjötugasti"),
            (80, "áttugasti"),
            (90, "nítugasti"),
            (100, "hundraðasti"),
            (1000, "þúsundasti"),
        ] {
            if value == &BigInt::from(n) {
                return Ok(word.to_string());
            }
        }

        Ok(format!("{}asti", cardinal))
    }

    /// Port of `Num2Word_IS.to_ordinal_num`: `str(value) + "."`.
    ///
    /// No `verify_ordinal`, no digit inspection — negatives render as "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `to_ordinal(float/Decimal)`: IS's `to_ordinal` opens with
    ///
    /// ```python
    /// try:
    ///     number = int(value)
    /// except (ValueError, TypeError):
    ///     return str(value)
    /// ```
    ///
    /// — a truncation, not a verify. So there is **no TypeError path**:
    /// `2.5` → int 2 → "annar", `-1.5` → int -1 → "mínus einnasti",
    /// `0.5` → "núllasti". `int(nan)`'s ValueError *is* caught, so a float
    /// NaN returns its own repr "nan"; `int(±inf)` raises OverflowError,
    /// which the except tuple does not cover, and propagates.
    fn ordinal_float_entry(&self, value: &crate::floatpath::FloatValue) -> Result<String> {
        let i: BigInt = match value {
            crate::floatpath::FloatValue::Float { value: f, .. } => {
                if f.is_nan() {
                    // ValueError from int(nan), caught -> str(value).
                    return Ok("nan".to_string());
                }
                if f.is_infinite() {
                    return Err(N2WError::Overflow(
                        "cannot convert float infinity to integer".into(),
                    ));
                }
                <BigInt as num_traits::FromPrimitive>::from_f64(f.trunc())
                    .expect("finite trunc fits BigInt")
            }
            // int(Decimal) truncates toward zero; with_scale(0) drops the
            // fraction the same way.
            crate::floatpath::FloatValue::Decimal { value: d, .. } => {
                d.with_scale(0).as_bigint_and_exponent().0
            }
        };
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `str(value) + "."` — the repr
    /// verbatim, no verify and no truncation: `5.0` → "5.0.", `0.5` → "0.5.",
    /// `Decimal("5.00")` → "5.00.", `-0.0` → "-0.0.".
    fn ordinal_num_float_entry(
        &self,
        _value: &crate::floatpath::FloatValue,
        repr_str: &str,
    ) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// Port of `Num2Word_IS.to_year`: plain `self.to_cardinal(val)`.
    ///
    /// IS discards the `suffix`/`longval` kwargs entirely — no century
    /// splitting, no BC/AD marker. `to_year(-500)` == "mínus fimm hundruð".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// Drive a corpus row the way `num2words2-py`'s binding does, so the
    /// argument literal decides int-vs-float exactly as `repr(value)` did on
    /// the Python side. That split is the whole ballgame for IS: `1` takes
    /// IS's own int branch and prints "einn evra", while `1.0` falls through
    /// to `Num2Word_Base.to_currency` and prints
    /// "einn ('evra', 'evrur'), núll ('sent', 'sent')".
    ///
    /// `has_decimal` is `!is_int` because every non-int row in the corpus is a
    /// Python `float`, for which `isinstance(val, float)` short-circuits the
    /// guard to true regardless of the repr.
    ///
    /// `cents=true` / `separator=None` / `adjective=false` mirror
    /// `bench/diff_test.py`, i.e. the kwargs left at their defaults.
    fn run_currency(arg: &str, code: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangIs::new().to_currency(&v, code, true, None, false)
    }

    /// The corpus records failures by Python exception name only.
    fn err_name(e: &N2WError) -> &'static str {
        match e {
            N2WError::NotImplemented(_) => "NotImplementedError",
            N2WError::Overflow(_) => "OverflowError",
            N2WError::Type(_) => "TypeError",
            N2WError::Value(_) => "ValueError",
            N2WError::Index(_) => "IndexError",
            N2WError::Key(_) => "KeyError",
            N2WError::Attribute(_) => "AttributeError",
            _ => "OtherError",
        }
    }

    fn check(got: Result<String>, want: std::result::Result<&str, &str>, what: &str) {
        match (got, want) {
            (Ok(g), Ok(w)) => assert_eq!(g, w, "{}", what),
            (Err(e), Err(w)) => assert_eq!(err_name(&e), w, "{}", what),
            (Ok(g), Err(w)) => panic!("{}: expected {}, got Ok({:?})", what, w, g),
            (Err(e), Ok(w)) => panic!("{}: expected Ok({:?}), got {:?}", what, w, e),
        }
    }

    /// Every `is` currency row in the frozen corpus, verbatim.
    #[test]
    fn corpus_currency() {
        let cases: &[(&str, &str, std::result::Result<&str, &str>)] = &[
        ("0",        "EUR",   Ok("núll evrur")),
        ("1",        "EUR",   Ok("einn evra")),
        ("2",        "EUR",   Ok("tveir evrur")),
        ("100",      "EUR",   Ok("eitt hundrað evrur")),
        ("12.34",    "EUR",   Ok("tólf ('evra', 'evrur'), þrjátíu og fjórir ('sent', 'sent')")),
        ("0.01",     "EUR",   Ok("núll ('evra', 'evrur'), einn ('sent', 'sent')")),
        ("1.0",      "EUR",   Ok("einn ('evra', 'evrur'), núll ('sent', 'sent')")),
        ("99.99",    "EUR",   Ok("níutíu og níu ('evra', 'evrur'), níutíu og níu ('sent', 'sent')")),
        ("1234.56",  "EUR",   Ok("eitt þúsund tvö hundruð þrjátíu og fjórir ('evra', 'evrur'), fimmtíu og sex ('sent', 'sent')")),
        ("-12.34",   "EUR",   Ok("mínus tólf ('evra', 'evrur'), þrjátíu og fjórir ('sent', 'sent')")),
        ("1000000",  "EUR",   Ok("ein milljón evrur")),
        ("0.5",      "EUR",   Ok("núll ('evra', 'evrur'), fimmtíu ('sent', 'sent')")),
        ("0",        "USD",   Ok("núll dalir")),
        ("1",        "USD",   Ok("einn dalur")),
        ("2",        "USD",   Ok("tveir dalir")),
        ("100",      "USD",   Ok("eitt hundrað dalir")),
        ("12.34",    "USD",   Ok("tólf ('dalur', 'dalir'), þrjátíu og fjórir ('sent', 'sent')")),
        ("0.01",     "USD",   Ok("núll ('dalur', 'dalir'), einn ('sent', 'sent')")),
        ("1.0",      "USD",   Ok("einn ('dalur', 'dalir'), núll ('sent', 'sent')")),
        ("99.99",    "USD",   Ok("níutíu og níu ('dalur', 'dalir'), níutíu og níu ('sent', 'sent')")),
        ("1234.56",  "USD",   Ok("eitt þúsund tvö hundruð þrjátíu og fjórir ('dalur', 'dalir'), fimmtíu og sex ('sent', 'sent')")),
        ("-12.34",   "USD",   Ok("mínus tólf ('dalur', 'dalir'), þrjátíu og fjórir ('sent', 'sent')")),
        ("1000000",  "USD",   Ok("ein milljón dalir")),
        ("0.5",      "USD",   Ok("núll ('dalur', 'dalir'), fimmtíu ('sent', 'sent')")),
        ("0",        "GBP",   Err("NotImplementedError")),
        ("1",        "GBP",   Err("NotImplementedError")),
        ("2",        "GBP",   Err("NotImplementedError")),
        ("100",      "GBP",   Err("NotImplementedError")),
        ("12.34",    "GBP",   Err("NotImplementedError")),
        ("0.01",     "GBP",   Err("NotImplementedError")),
        ("1.0",      "GBP",   Err("NotImplementedError")),
        ("99.99",    "GBP",   Err("NotImplementedError")),
        ("1234.56",  "GBP",   Err("NotImplementedError")),
        ("-12.34",   "GBP",   Err("NotImplementedError")),
        ("1000000",  "GBP",   Err("NotImplementedError")),
        ("0.5",      "GBP",   Err("NotImplementedError")),
        ("0",        "JPY",   Err("NotImplementedError")),
        ("1",        "JPY",   Err("NotImplementedError")),
        ("2",        "JPY",   Err("NotImplementedError")),
        ("100",      "JPY",   Err("NotImplementedError")),
        ("12.34",    "JPY",   Err("NotImplementedError")),
        ("0.01",     "JPY",   Err("NotImplementedError")),
        ("1.0",      "JPY",   Err("NotImplementedError")),
        ("99.99",    "JPY",   Err("NotImplementedError")),
        ("1234.56",  "JPY",   Err("NotImplementedError")),
        ("-12.34",   "JPY",   Err("NotImplementedError")),
        ("1000000",  "JPY",   Err("NotImplementedError")),
        ("0.5",      "JPY",   Err("NotImplementedError")),
        ("0",        "KWD",   Err("NotImplementedError")),
        ("1",        "KWD",   Err("NotImplementedError")),
        ("2",        "KWD",   Err("NotImplementedError")),
        ("100",      "KWD",   Err("NotImplementedError")),
        ("12.34",    "KWD",   Err("NotImplementedError")),
        ("0.01",     "KWD",   Err("NotImplementedError")),
        ("1.0",      "KWD",   Err("NotImplementedError")),
        ("99.99",    "KWD",   Err("NotImplementedError")),
        ("1234.56",  "KWD",   Err("NotImplementedError")),
        ("-12.34",   "KWD",   Err("NotImplementedError")),
        ("1000000",  "KWD",   Err("NotImplementedError")),
        ("0.5",      "KWD",   Err("NotImplementedError")),
        ("0",        "BHD",   Err("NotImplementedError")),
        ("1",        "BHD",   Err("NotImplementedError")),
        ("2",        "BHD",   Err("NotImplementedError")),
        ("100",      "BHD",   Err("NotImplementedError")),
        ("12.34",    "BHD",   Err("NotImplementedError")),
        ("0.01",     "BHD",   Err("NotImplementedError")),
        ("1.0",      "BHD",   Err("NotImplementedError")),
        ("99.99",    "BHD",   Err("NotImplementedError")),
        ("1234.56",  "BHD",   Err("NotImplementedError")),
        ("-12.34",   "BHD",   Err("NotImplementedError")),
        ("1000000",  "BHD",   Err("NotImplementedError")),
        ("0.5",      "BHD",   Err("NotImplementedError")),
        ("0",        "INR",   Err("NotImplementedError")),
        ("1",        "INR",   Err("NotImplementedError")),
        ("2",        "INR",   Err("NotImplementedError")),
        ("100",      "INR",   Err("NotImplementedError")),
        ("12.34",    "INR",   Err("NotImplementedError")),
        ("0.01",     "INR",   Err("NotImplementedError")),
        ("1.0",      "INR",   Err("NotImplementedError")),
        ("99.99",    "INR",   Err("NotImplementedError")),
        ("1234.56",  "INR",   Err("NotImplementedError")),
        ("-12.34",   "INR",   Err("NotImplementedError")),
        ("1000000",  "INR",   Err("NotImplementedError")),
        ("0.5",      "INR",   Err("NotImplementedError")),
        ("0",        "CNY",   Err("NotImplementedError")),
        ("1",        "CNY",   Err("NotImplementedError")),
        ("2",        "CNY",   Err("NotImplementedError")),
        ("100",      "CNY",   Err("NotImplementedError")),
        ("12.34",    "CNY",   Err("NotImplementedError")),
        ("0.01",     "CNY",   Err("NotImplementedError")),
        ("1.0",      "CNY",   Err("NotImplementedError")),
        ("99.99",    "CNY",   Err("NotImplementedError")),
        ("1234.56",  "CNY",   Err("NotImplementedError")),
        ("-12.34",   "CNY",   Err("NotImplementedError")),
        ("1000000",  "CNY",   Err("NotImplementedError")),
        ("0.5",      "CNY",   Err("NotImplementedError")),
        ("0",        "CHF",   Err("NotImplementedError")),
        ("1",        "CHF",   Err("NotImplementedError")),
        ("2",        "CHF",   Err("NotImplementedError")),
        ("100",      "CHF",   Err("NotImplementedError")),
        ("12.34",    "CHF",   Err("NotImplementedError")),
        ("0.01",     "CHF",   Err("NotImplementedError")),
        ("1.0",      "CHF",   Err("NotImplementedError")),
        ("99.99",    "CHF",   Err("NotImplementedError")),
        ("1234.56",  "CHF",   Err("NotImplementedError")),
        ("-12.34",   "CHF",   Err("NotImplementedError")),
        ("1000000",  "CHF",   Err("NotImplementedError")),
        ("0.5",      "CHF",   Err("NotImplementedError")),
        ];
        for (arg, code, want) in cases {
            check(run_currency(arg, code), *want, &format!("to_currency({}, {})", arg, code));
        }
    }

    /// Every `is` cheque row in the frozen corpus, verbatim.
    #[test]
    fn corpus_cheque() {
        let cases: &[(&str, &str, std::result::Result<&str, &str>)] = &[
        ("1234.56",  "EUR",   Ok("EITT ÞÚSUND TVÖ HUNDRUÐ ÞRJÁTÍU OG FJÓRIR AND 56/100 EVRUR")),
        ("1234.56",  "USD",   Ok("EITT ÞÚSUND TVÖ HUNDRUÐ ÞRJÁTÍU OG FJÓRIR AND 56/100 DALIR")),
        ("1234.56",  "GBP",   Err("NotImplementedError")),
        ("1234.56",  "JPY",   Err("NotImplementedError")),
        ("1234.56",  "KWD",   Err("NotImplementedError")),
        ("1234.56",  "BHD",   Err("NotImplementedError")),
        ("1234.56",  "INR",   Err("NotImplementedError")),
        ("1234.56",  "CNY",   Err("NotImplementedError")),
        ("1234.56",  "CHF",   Err("NotImplementedError")),
        ];
        for (arg, code, want) in cases {
            let v = BigDecimal::from_str(arg).unwrap();
            check(LangIs::new().to_cheque(&v, code), *want, &format!("to_cheque({}, {})", arg, code));
        }
    }

    /// The exact NotImplementedError text, which the corpus records only by
    /// exception name. `self.__class__.__name__` names IS even though the
    /// raise happens inside `Num2Word_Base.to_currency` after IS delegates.
    #[test]
    fn missing_currency_message() {
        let l = LangIs::new();
        // int path: IS's own lookup misses, delegates to super(), super raises.
        let e = l
            .to_currency(&CurrencyValue::Int(1.into()), "GBP", true, None, false)
            .unwrap_err();
        match e {
            N2WError::NotImplemented(m) => {
                assert_eq!(m, "Currency code \"GBP\" not implemented for \"Num2Word_IS\"")
            }
            other => panic!("expected NotImplemented, got {:?}", other),
        }
        // cheque path raises the same message from the same template.
        let e = l.to_cheque(&BigDecimal::from(1), "JPY").unwrap_err();
        match e {
            N2WError::NotImplemented(m) => {
                assert_eq!(m, "Currency code \"JPY\" not implemented for \"Num2Word_IS\"")
            }
            other => panic!("expected NotImplemented, got {:?}", other),
        }
    }

    /// Behaviour the corpus does not cover, pinned against the live Python.
    ///
    /// All four were produced by running `Num2Word_IS` in the interpreter; see
    /// the notes on [`LangIs::to_currency`].
    #[test]
    fn uncovered_by_corpus() {
        let l = LangIs::new();
        let int = |n: i64| CurrencyValue::Int(n.into());
        let flt = |s: &str| CurrencyValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            has_decimal: true,
            is_float: true,
        };

        // 1. Negative *int*: single space after the minus word (#160).
        assert_eq!(
            l.to_currency(&int(-12), "EUR", true, None, false).unwrap(),
            "mínus tólf evrur"
        );
        assert_eq!(
            l.to_currency(&int(-1), "EUR", true, None, false).unwrap(),
            "mínus einn evra"
        );

        // 2. ISK, the default currency, is never exercised by the corpus.
        assert_eq!(l.to_currency(&int(1), "ISK", true, None, false).unwrap(), "einn króna");
        assert_eq!(l.to_currency(&int(2), "ISK", true, None, false).unwrap(), "tveir krónur");
        assert_eq!(
            l.to_currency(&flt("12.34"), "ISK", true, None, false).unwrap(),
            "tólf ('króna', 'krónur'), þrjátíu og fjórir ('eyrir', 'aurar')"
        );

        // 3. adjective=True is ignored on the int path, applied on the float
        //    path (and there it is prefixed *inside* the leaked tuple).
        assert_eq!(l.to_currency(&int(2), "ISK", true, None, true).unwrap(), "tveir krónur");
        assert_eq!(
            l.to_currency(&flt("2.5"), "ISK", true, None, true).unwrap(),
            "tveir ('íslenskar króna', 'íslenskar krónur'), fimmtíu ('eyrir', 'aurar')"
        );

        // 4. cents=false swaps _cents_verbose for _cents_terse (width 2, since
        //    IS inherits Base's empty CURRENCY_PRECISION -> divisor 100).
        assert_eq!(
            l.to_currency(&flt("12.34"), "EUR", false, None, false).unwrap(),
            "tólf ('evra', 'evrur'), 34 ('sent', 'sent')"
        );
        // A caller-supplied separator replaces the "," default.
        assert_eq!(
            l.to_currency(&flt("12.34"), "EUR", true, Some(" og"), false).unwrap(),
            "tólf ('evra', 'evrur') og þrjátíu og fjórir ('sent', 'sent')"
        );

        // 5. has_decimal, not the numeric value, gates the cents segment:
        //    Decimal("5") and Decimal("5.00") are numerically equal.
        let d5 = CurrencyValue::Decimal { value: BigDecimal::from_str("5").unwrap(), has_decimal: false, is_float: false };
        let d500 = CurrencyValue::Decimal { value: BigDecimal::from_str("5.00").unwrap(), has_decimal: true, is_float: false };
        assert_eq!(l.to_currency(&d5, "EUR", true, None, false).unwrap(), "fimm ('evra', 'evrur')");
        assert_eq!(
            l.to_currency(&d500, "EUR", true, None, false).unwrap(),
            "fimm ('evra', 'evrur'), núll ('sent', 'sent')"
        );

        // 6. Negative cheque, and a cheque whose cents are zero.
        assert_eq!(
            l.to_cheque(&BigDecimal::from_str("-1234.56").unwrap(), "EUR").unwrap(),
            "MINUS EITT ÞÚSUND TVÖ HUNDRUÐ ÞRJÁTÍU OG FJÓRIR AND 56/100 EVRUR"
        );
        assert_eq!(
            l.to_cheque(&BigDecimal::from_str("1.0").unwrap(), "ISK").unwrap(),
            "EINN AND 00/100 KRÓNUR"
        );
    }

    /// `pluralize` on the tuple call site ignores `n` entirely -- both live
    /// branches return the tuple untouched. Guards the leak against a
    /// well-meaning "fix".
    #[test]
    fn pluralize_tuple_is_count_independent() {
        let l = LangIs::new();
        let forms: Vec<String> = vec!["evra".into(), "evrur".into()];
        for n in [0i64, 1, 2, 11, 21, 34, 101] {
            // UFCS is required: the inherent `pluralize(&self, n, &str)` wins
            // plain method resolution, which is precisely why `merge` keeps
            // getting the string behaviour while `default_to_currency` --
            // generic over `L: Lang`, where only the trait method is visible --
            // gets the tuple one.
            assert_eq!(
                Lang::pluralize(&l, &BigInt::from(n), &forms).unwrap(),
                "('evra', 'evrur')",
                "n = {}",
                n
            );
        }
        // The string call site (merge's) is unaffected and still pluralizes.
        assert_eq!(l.pluralize(&BigInt::from(2), "hundrað"), "hundruð");
        assert_eq!(l.pluralize(&BigInt::from(1), "hundrað"), "hundrað");
    }
}
