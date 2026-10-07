//! Port of `lang_EL.py` (Greek), via its `Num2Word_EUR` → `Num2Word_Base`
//! ancestry. Registry key `"el"` resolves to `Num2Word_EL` (verified in
//! `num2words2/__init__.py`).
//!
//! Shape: **engine**. `Num2Word_EL.setup` defines `mid_numwords` /
//! `low_numwords` and overrides `merge`, letting `Num2Word_Base.to_cardinal`
//! drive `splitnum` / `clean`. Only `to_ordinal` and `to_ordinal_num` are
//! overridden; `to_year` is inherited unchanged (`= to_cardinal`), so the
//! trait default is left in place.
//!
//! # The card table is *only* mid + low — no high numwords at all
//!
//! This is the single most surprising thing about the module and it is load
//! bearing. `Num2Word_EL.setup` runs `Num2Word_EUR.setup(self)` (which fills
//! `self.high_numwords` with the Latin illion stems) and then immediately
//! does:
//!
//! ```text
//! self.GIGA_SUFFIX = ""   # Don't use the EU pattern
//! self.MEGA_SUFFIX = ""   # Don't use the EU pattern
//! ```
//!
//! `Num2Word_Base.__init__` calls `setup()` *before* `set_numwords()`, so by
//! the time `Num2Word_EUR.set_high_numwords` runs, both suffixes are the empty
//! string and its two bodies are guarded by `if self.GIGA_SUFFIX:` /
//! `if self.MEGA_SUFFIX:`. Both are falsy ⇒ **not a single high card is
//! inserted**. `high_numwords` is computed and thrown away.
//!
//! Consequence: the highest card is the mid entry `10**12`, and
//! `MAXVAL = 1000 * list(self.cards.keys())[0]` = `10**15`. Greek therefore
//! overflows at one quadrillion — far earlier than its EUR siblings. The
//! corpus confirms: `cardinal(10**15)` → `OverflowError`.
//!
//! # Faithfully reproduced Python bugs
//!
//! Everything below is wrong-looking Greek but is exactly what the Python
//! emits, and every item is pinned by a row in `bench/corpus.jsonl`.
//!
//! 1–7. ~~**`to_ordinal` glued "ος" onto the cardinal**~~ and kept only
//!    the first two words of a compound: 121..=129 all read "εκατοστός
//!    εικοστός", 30 "τριάντος", 1001 "χίλια πρώτος", 2000 "δύο
//!    χιλιάδεςος", 13 "δεκατρίτος", 9 "έννατος". Fixed
//!    (gladiaio/num2words2#251): every component takes its own ordinal
//!    (121 "εκατοστός εικοστός πρώτος", 13 "δέκατος τρίτος", 9 "ένατος",
//!    2000 "δισχιλιοστός"), sourced up to 9999 plus the round 10**4,
//!    10**5, 10**6 and 10**9; other values from 10000 up raise
//!    OverflowError. Only
//!    `to_ordinal(0)` == `"μηδένος"` survives from the old rule.
//! 8. Mixed-gender agreement in `merge`: hundreds go feminine before
//!    χιλιάδες ("διακόσιες χιλιάδες") but stay neuter before εκατομμύρια
//!    ("διακόσια τριάντα τέσσερα εκατομμύρια"), and the *tens/units* are never
//!    feminised, so 1234567 renders "διακόσιες τριάντα **τέσσερα** χιλιάδες"
//!    (should be τέσσερις). All reproduced.
//!
//! # Dead code in the Python, reproduced structurally
//!
//! `merge`'s `cnum == 1` fast-path returns unconditionally (its four arms
//! cover every `nnum`: `== 100`, `== 1000`, `>= 10**6`, `< 10**6`). That makes
//! the `if cnum == 1:` sub-branches inside the `nnum == 1000` / `== 10**6` /
//! `== 10**9` / `== 10**12` arms unreachable, as is the trailing
//! `elif nnum >= 1000000:` arm (10**6/10**9/10**12 are the only cards that
//! large, and each has its own arm). They are kept below, marked, so the
//! shape matches the source.
//!
//! # No cross-call mutable state
//!
//! `Num2Word_EL` stashes nothing between calls: there is no `str_to_number`
//! override and no `_pending_*` flag. The stateless Rust path is faithful.
//!
//! # The currency surface
//!
//! `Num2Word_EL` declares its **own** class-level `CURRENCY_FORMS`, so the
//! `lang_EUR.py` trap does not apply: the in-place rewrite that
//! `Num2Word_EN.__init__` performs on `Num2Word_EUR.CURRENCY_FORMS` at import
//! time never reaches Greek. Checked against the live interpreter —
//! `Num2Word_EL.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` is `False`, and
//! EL sees exactly three codes (EUR/USD/GBP). Every other code is a corpus
//! `NotImplementedError` row.
//!
//! `CURRENCY_ADJECTIVES` is the opposite: EL *shares* EUR's dict (identity
//! `True`), and nothing mutates that one — EN rewrites forms, never
//! adjectives. So Greek inherits all 16 EUR adjectives, of which only USD
//! overlaps its three codes.
//!
//! `CURRENCY_PRECISION` stays Base's empty dict (EN's rebind creates an
//! *instance* attribute and does not leak), so every divisor is 100.
//!
//! EL overrides `to_currency` wholesale. Two behaviours are its own, both
//! pinned by corpus rows:
//!
//! 1. **A whole float renders no cents.** Base prints "one euro and zero
//!    cents" for `1.0`; EL folds whole floats into the int arm, so `1.0` EUR
//!    is just "ένα ευρώ".
//! 2. **The divisor is a hard-coded literal 100** — EL's `to_currency` never
//!    reads `CURRENCY_PRECISION`, here or in either `parse_currency_parts`
//!    call. Harmless today (EL's precision table is empty, so 100 is right for
//!    all three codes) but it means the 3-decimal / 0-decimal machinery could
//!    never engage for Greek even if such a code were added.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, prefix_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// Ordinals 1..=9 (masculine), for [`LangEl::to_ordinal`] (#251).
const UNITS_ORD: [&str; 9] = [
    "πρώτος", "δεύτερος", "τρίτος", "τέταρτος", "πέμπτος", "έκτος", "έβδομος",
    "όγδοος", "ένατος",
];

/// Ordinals 10, 20, …, 90.
const TENS_ORD: [&str; 9] = [
    "δέκατος", "εικοστός", "τριακοστός", "τεσσαρακοστός", "πεντηκοστός",
    "εξηκοστός", "εβδομηκοστός", "ογδοηκοστός", "ενενηκοστός",
];

/// Ordinals 100, 200, …, 900.
const HUNDREDS_ORD: [&str; 9] = [
    "εκατοστός", "διακοσιοστός", "τριακοσιοστός", "τετρακοσιοστός",
    "πεντακοσιοστός", "εξακοσιοστός", "επτακοσιοστός", "οκτακοσιοστός",
    "εννιακοσιοστός",
];

/// Ordinals 1000, 2000, …, 9000.
const THOUSANDS_ORD: [&str; 9] = [
    "χιλιοστός", "δισχιλιοστός", "τρισχιλιοστός", "τετρακισχιλιοστός",
    "πεντακισχιλιοστός", "εξακισχιλιοστός", "επτακισχιλιοστός",
    "οκτακισχιλιοστός", "εννεακισχιλιοστός",
];

/// `Num2Word_EL.low_numwords`, 20 down to 0.
const LOW_NUMWORDS: [&str; 21] = [
    "είκοσι",
    "δεκαεννέα",
    "δεκαοκτώ",
    "δεκαεπτά",
    "δεκαέξι",
    "δεκαπέντε",
    "δεκατέσσερα",
    "δεκατρία",
    "δώδεκα",
    "έντεκα",
    "δέκα",
    "εννέα",
    "οκτώ",
    "επτά",
    "έξι",
    "πέντε",
    "τέσσερα",
    "τρία",
    "δύο",
    "ένα",
    "μηδέν",
];

/// `Num2Word_EL.mid_numwords`. Note the *absence* of anything above 10**12:
/// this list alone determines `MAXVAL` (see the module docs).
const MID_NUMWORDS: [(i64, &str); 12] = [
    (1_000_000_000_000, "τρισεκατομμύριο"),
    (1_000_000_000, "δισεκατομμύριο"),
    (1_000_000, "εκατομμύριο"),
    (1000, "χίλια"),
    (100, "εκατό"),
    (90, "ενενήντα"),
    (80, "ογδόντα"),
    (70, "εβδομήντα"),
    (60, "εξήντα"),
    (50, "πενήντα"),
    (40, "σαράντα"),
    (30, "τριάντα"),
];

/// `merge`'s hundreds table (the `nnum == 100` arm), cnum 2..=9.
const HUNDREDS: [(i64, &str); 8] = [
    (2, "διακόσια"),
    (3, "τριακόσια"),
    (4, "τετρακόσια"),
    (5, "πεντακόσια"),
    (6, "εξακόσια"),
    (7, "επτακόσια"),
    (8, "οκτακόσια"),
    (9, "εννιακόσια"),
];

/// `merge`'s explicit "round hundreds of thousands" table (the `nnum == 1000`
/// arm), cnum 200..=900. Feminine, because χιλιάδες is feminine.
const HUNDRED_THOUSANDS: [(i64, &str); 8] = [
    (200, "διακόσιες χιλιάδες"),
    (300, "τριακόσιες χιλιάδες"),
    (400, "τετρακόσιες χιλιάδες"),
    (500, "πεντακόσιες χιλιάδες"),
    (600, "εξακόσιες χιλιάδες"),
    (700, "επτακόσιες χιλιάδες"),
    (800, "οκτακόσιες χιλιάδες"),
    (900, "εννιακόσιες χιλιάδες"),
];

/// The `str.replace` chain in `merge`'s `nnum == 1000` fallback arm, applied
/// in source order. Neuter hundreds → feminine, and ένα → μία.
const FEMININE_REPLACEMENTS: [(&str, &str); 9] = [
    ("ένα", "μία"),
    ("διακόσια", "διακόσιες"),
    ("τριακόσια", "τριακόσιες"),
    ("τετρακόσια", "τετρακόσιες"),
    ("πεντακόσια", "πεντακόσιες"),
    ("εξακόσια", "εξακόσιες"),
    ("επτακόσια", "επτακόσιες"),
    ("οκτακόσια", "οκτακόσιες"),
    ("εννιακόσια", "εννιακόσιες"),
];

/// `Num2Word_EL.CURRENCY_FORMS` — EL's own class dict, not EUR's.
///
/// Three codes only. Note "πέννα" (double ν) singular against "πένες" (single
/// ν) plural: that inconsistency is the source's, and both spellings are
/// pinned by corpus rows, so it is transcribed verbatim.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "EUR",
        CurrencyForms::new(&["ευρώ", "ευρώ"], &["λεπτό", "λεπτά"]),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["δολάριο", "δολάρια"], &["σεντ", "σεντς"]),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(&["λίρα", "λίρες"], &["πέννα", "πένες"]),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged (EL shares the
/// dict object itself, and nothing in the library mutates it).
///
/// Only USD overlaps EL's three currency codes, so it is the one reachable
/// entry: `to_currency(2.5, "USD", adjective=True)` gives
/// "δύο US δολάρια και πενήντα σεντς". The other 15 are carried because
/// `to_currency`'s `currency in self.CURRENCY_ADJECTIVES` test reads the whole
/// dict — they are unreachable only because the *forms* lookup raises first.
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

fn is(n: &BigInt, k: i64) -> bool {
    *n == BigInt::from(k)
}

pub struct LangEl {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEl {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // set_high_numwords contributes NOTHING: GIGA_SUFFIX and MEGA_SUFFIX
        // are both "" by the time set_numwords() runs. See module docs.

        set_mid_numwords(&mut cards, &MID_NUMWORDS);
        set_low_numwords(&mut cards, &LOW_NUMWORDS);

        // MAXVAL = 1000 * list(self.cards.keys())[0]. Python's OrderedDict
        // insertion order here (mid descending, then low descending) is
        // already descending, so keys()[0] == highest() == 10**12.
        // MAXVAL therefore == 10**15.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangEl {
            cards,
            maxval,
            // is_title is False for EL, so exclude_title is never consulted;
            // carried for fidelity with setup().
            exclude_title: vec!["και".into(), "κόμμα".into(), "μείον".into()],
            // Built once here, never per call. `to_currency` only ever reads
            // these tables, and rebuilding them on each call is what made an
            // earlier revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_EL.pluralize`, factored over the only thing the rule reads:
    /// whether `n == 1`.
    ///
    /// The trait's `pluralize` takes a `&BigInt`, but Python's
    /// `to_currency` calls `self.pluralize(right, cr2)` with `right` still a
    /// **Decimal** on the fractional-cents path (`Decimal("1.100")` for
    /// 1.011 EUR). Keeping the rule here lets both callers share it without
    /// having to pretend a fractional cent count is an integer.
    fn pluralize_forms(&self, is_one: bool, forms: &[String]) -> Result<String> {
        let form = if is_one { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            // Python indexes the tuple directly, so a one-form entry with
            // n != 1 raises IndexError. All three EL entries carry two forms,
            // so this is unreachable — mapped rather than panicked so the
            // exception type survives if the table ever changes.
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_Base.verify_ordinal`. The float branch cannot fire for
    /// integral input, so only the negative check survives.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

impl Lang for LangEl {
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
        " και"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    fn negword(&self) -> &str {
        "μείον "
    }

    fn pointword(&self) -> &str {
        "κόμμα"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_EL.merge`. `curr` = `(ctext, cnum)`, `next` =
    /// `(ntext, nnum)`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext, cnum) = l;
        let (ntext, nnum) = r;

        let c100 = BigInt::from(100);
        let c1000 = BigInt::from(1000);
        let mega = BigInt::from(1_000_000i64);
        let giga = BigInt::from(1_000_000_000i64);
        let tera = BigInt::from(1_000_000_000_000i64);

        // ---- Special handling for 1 -------------------------------------
        // These four arms are exhaustive over nnum, so cnum == 1 ALWAYS
        // returns here. That is what makes the `cnum == 1` sub-branches
        // further down dead code.
        if cnum.is_one() {
            if *nnum == c100 {
                return ("εκατό".to_string(), c100);
            } else if *nnum == c1000 {
                return ("χίλια".to_string(), c1000);
            } else if *nnum >= mega {
                return (format!("ένα {}", ntext), cnum * nnum);
            } else {
                // nnum < 1000000 → `return next`. Also the path that yields
                // "μηδέν" for 0 (merge(("ένα",1), ("μηδέν",0))).
                return (ntext.to_string(), nnum.clone());
            }
        }

        if nnum > cnum {
            if *nnum == c100 {
                // Hundreds. cnum is provably 2..=9: card 100 is only chosen
                // by splitnum when 100 <= value < 1000 (1000 is a card and
                // would win otherwise), so div = value // 100 ∈ 1..9, and
                // cnum == 1 already returned above.
                for (k, w) in HUNDREDS {
                    if is(cnum, k) {
                        return (w.to_string(), BigInt::from(k * 100));
                    }
                }
                // Python falls off the if/elif chain and returns None here,
                // which would blow up in `clean`'s tuple unpack (TypeError).
                // Unreachable per the argument above.
                unreachable!("EL merge: nnum == 100 with cnum outside 2..=9");
            } else if *nnum == c1000 {
                // Thousands — χιλιάδες is feminine.
                if cnum.is_one() {
                    // DEAD: cnum == 1 returned in the fast-path above.
                    if ctext.contains("ένα") {
                        return (
                            format!("{} χιλιάδες", ctext.replace("ένα", "μία")),
                            cnum * nnum,
                        );
                    } else {
                        return ("χίλια".to_string(), c1000);
                    }
                } else if is(cnum, 2) {
                    return ("δύο χιλιάδες".to_string(), BigInt::from(2000));
                } else if is(cnum, 3) {
                    return ("τρεις χιλιάδες".to_string(), BigInt::from(3000));
                } else if is(cnum, 4) {
                    return ("τέσσερις χιλιάδες".to_string(), BigInt::from(4000));
                }
                for (k, w) in HUNDRED_THOUSANDS {
                    if is(cnum, k) {
                        return (w.to_string(), BigInt::from(k * 1000));
                    }
                }
                // Fallback: feminise in place. Note this only rewrites ένα
                // and the *hundreds*; τέσσερα/τρία are left neuter, which is
                // why 1234567 says "διακόσιες τριάντα τέσσερα χιλιάδες".
                let mut ctext_new = ctext.to_string();
                for (from, to) in FEMININE_REPLACEMENTS {
                    ctext_new = ctext_new.replace(from, to);
                }
                (format!("{} χιλιάδες", ctext_new), cnum * nnum)
            } else if *nnum == mega {
                // Millions — εκατομμύρια is neuter, so no feminisation.
                if cnum.is_one() {
                    // DEAD (fast-path); would produce the same string anyway.
                    return ("ένα εκατομμύριο".to_string(), mega);
                } else if is(cnum, 2) {
                    return ("δύο εκατομμύρια".to_string(), BigInt::from(2_000_000i64));
                } else if is(cnum, 3) {
                    return ("τρία εκατομμύρια".to_string(), BigInt::from(3_000_000i64));
                } else if is(cnum, 4) {
                    return (
                        "τέσσερα εκατομμύρια".to_string(),
                        BigInt::from(4_000_000i64),
                    );
                } else {
                    return (format!("{} εκατομμύρια", ctext), cnum * nnum);
                }
            } else if *nnum == giga {
                if cnum.is_one() {
                    // DEAD (fast-path).
                    return ("ένα δισεκατομμύριο".to_string(), giga);
                } else {
                    return (format!("{} δισεκατομμύρια", ctext), cnum * nnum);
                }
            } else if *nnum == tera {
                if cnum.is_one() {
                    // DEAD (fast-path).
                    return ("ένα τρισεκατομμύριο".to_string(), tera);
                } else {
                    return (format!("{} τρισεκατομμύρια", ctext), cnum * nnum);
                }
            } else if *nnum >= mega {
                // DEAD: 10**6 / 10**9 / 10**12 are the only cards >= 10**6
                // and each has its own arm above.
                if cnum.is_one() {
                    return (format!("ένα {}", ntext), cnum * nnum);
                } else {
                    return (format!("{} {}", ctext, ntext), cnum * nnum);
                }
            } else {
                // Regular multiplication (tens/units × a smaller card).
                (format!("{} {}", ctext, ntext), cnum * nnum)
            }
        } else {
            // Addition (smaller unit appended).
            (format!("{} {}", ctext, ntext), cnum + nnum)
        }
    }

    /// Greek ordinals: every component of the number takes its own ordinal
    /// form — 121 is "εκατοστός εικοστός πρώτος", 1001 "χιλιοστός πρώτος",
    /// 2345 "δισχιλιοστός τριακοσιοστός τεσσαρακοστός πέμπτος".
    ///
    /// Python glued "ος" onto the cardinal and dropped words past the
    /// second (gladiaio/num2words2#251). The forms are sourced up to 9999
    /// and for the round δεκακισχιλιοστός (10**4), εκατοντακισχιλιοστός
    /// (10**5), εκατομμυριοστός (10**6) and δισεκατομμυριοστός (10**9);
    /// other values from 10000 up raise OverflowError — compounds of the
    /// higher thousands have competing forms (δεκακισχιλιοστός /
    /// δεκαχιλιοστός / μυριοστός, …). 0 keeps Python's "μηδένος".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_zero() {
            return Ok(format!("{}ος", self.to_cardinal(value)?));
        }
        if is(value, 10_000) {
            return Ok("δεκακισχιλιοστός".to_string());
        }
        if is(value, 100_000) {
            return Ok("εκατοντακισχιλιοστός".to_string());
        }
        if is(value, 1_000_000) {
            return Ok("εκατομμυριοστός".to_string());
        }
        if is(value, 1_000_000_000) {
            return Ok("δισεκατομμυριοστός".to_string());
        }
        let n = match value.to_u32() {
            Some(n) if n < 10_000 => n as usize,
            _ => {
                // Still raise the cardinal's own OverflowError past maxval.
                self.to_cardinal(value)?;
                return Err(N2WError::Overflow(format!(
                    "lang='el' spells ordinals up to 9999 and for 10**4, 10**5, 10**6, 10**9; \
                     {} has no ordinal form",
                    value
                )));
            }
        };
        let mut parts: Vec<&str> = Vec::new();
        if n >= 1000 {
            parts.push(THOUSANDS_ORD[n / 1000 - 1]);
        }
        if n % 1000 >= 100 {
            parts.push(HUNDREDS_ORD[n % 1000 / 100 - 1]);
        }
        match n % 100 {
            0 => {}
            11 => parts.push("ενδέκατος"),
            12 => parts.push("δωδέκατος"),
            r => {
                if r >= 10 {
                    parts.push(TENS_ORD[r / 10 - 1]);
                }
                if r % 10 > 0 {
                    parts.push(UNITS_ORD[r % 10 - 1]);
                }
            }
        }
        Ok(parts.join(" "))
    }

    /// Port of `Num2Word_EL.to_ordinal_num`: `str(value) + "ος"`.
    /// Note it never touches `to_cardinal`, so it does NOT overflow at
    /// 10**15 — `to_ordinal_num(10**21)` == "1000000000000000000000ος".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}ος", value))
    }

    // to_year is NOT overridden by Num2Word_EL; Num2Word_Base.to_year just
    // delegates to to_cardinal, which is exactly the trait default here.

    /// `to_ordinal(float/Decimal)` — `verify_ordinal` gates the float domain:
    /// a fractional value raises TypeError ("Cannot treat float %s as
    /// ordinal.") before the negative check ("Cannot treat negative num %s
    /// as ordinal."); -0.0 passes both (`abs(-0.0) == -0.0`). A surviving
    /// whole value takes the integer ordinal path — base `to_cardinal`
    /// routes a whole float through `int(value)` — so `5.00` is "πέμπτος"
    /// and `1E+20` still overflows inside `to_cardinal`.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = verify_ordinal_float(value, &ordinal_float_repr(value))?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)` — the same `verify_ordinal` gate, then
    /// `str(value) + "ος"` with no cardinal call, so "1e+16ος" and "1E+20ος"
    /// never overflow.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        verify_ordinal_float(value, repr_str)?;
        Ok(format!("{}ος", repr_str))
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_EL` overrides `to_currency`, `_money_verbose`, `_cents_verbose`
    // and `pluralize`, and declares its own `CURRENCY_FORMS`. The rest of the
    // currency path is inherited unchanged and the trait defaults already
    // mirror it, so it is deliberately left alone:
    //
    //   * `CURRENCY_PRECISION` — Base's empty dict, so `.get(code, 100)` is 100
    //     for every code, which is the trait default. (EN's rebind creates an
    //     *instance* attribute and does not leak into the shared class dict.)
    //   * `_cents_terse` — Base's, which reads that same precision -> width 2.
    //   * `to_cheque` — Base's, i.e. `currency::default_to_cheque`. It routes
    //     through `_money_verbose`, so EL's GBP feminine fix-up applies there
    //     too.
    //   * `to_cardinal_float` / `cardinal_from_decimal` — Base's.

    fn lang_name(&self) -> &str {
        "Num2Word_EL"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EL.pluralize`: `forms[0] if n == 1 else forms[1]`.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        self.pluralize_forms(n.is_one(), forms)
    }

    /// `Num2Word_EL._money_verbose`: GBP's λίρα is feminine, so 1 is "μία"
    /// rather than the neuter "ένα" `to_cardinal` would give.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        if currency == "GBP" && number.is_one() {
            return Ok("μία".to_string());
        }
        self.to_cardinal(number)
    }

    /// `Num2Word_EL._cents_verbose`: the same feminine fix-up, for GBP's πέννα.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        if currency == "GBP" && number.is_one() {
            return Ok("μία".to_string());
        }
        self.to_cardinal(number)
    }

    /// Port of `Num2Word_EL.to_currency` — a full rewrite of Base's, not a
    /// tweak of it. See the module docs for the two behaviours that are EL's
    /// alone (whole floats print no cents; the divisor is a hard-coded 100).
    ///
    /// # `float` vs `Decimal` is not recoverable here
    ///
    /// EL branches on `isinstance(val, float)`, which `CurrencyValue` cannot
    /// express: the shim sends `is_int` (`type(val) is int`) and `has_decimal`
    /// (`isinstance(val, float) or "." in str(val)`), and a float and a
    /// dotted Decimal produce the *same* pair. Python does distinguish them —
    /// `5.0` gives "πέντε ευρώ" but `Decimal("5.0")` gives "πέντε ευρώ και
    /// μηδέν λεπτά". Every non-int is treated as a float below, which is what
    /// the corpus pins (`1.0` -> "ένα ευρώ") and the only reading the
    /// abstraction supports. See `concerns`.
    ///
    /// The quantize inside `parse_currency_parts` is always a no-op for EL:
    /// `has_fractional_cents` is False exactly when the value already has <= 2
    /// decimal places, and that is precisely when `keep_precision` is False.
    /// Greek currency therefore never rounds.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` == the caller omitted the kwarg, so EL's own default (" και")
        // applies. Base's is ",".
        let separator = separator.unwrap_or(self.default_separator());

        // `decimal_val = Decimal(str(val))`, then
        // `has_fractional_cents = (decimal_val * 100) % 1 != 0`.
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };
        // Decimal's `%` truncates toward zero, so `x % 1 != 0` is exactly "x
        // has a fractional part" — for negatives too.
        let has_fractional_cents = !(&decimal_val * BigDecimal::from(100)).is_integer();

        // `isinstance(val, float) and val == int(val)`.
        let is_whole_float = match val {
            CurrencyValue::Int(_) => false,
            CurrencyValue::Decimal { value, .. } => value.is_integer(),
        };
        let is_int = matches!(val, CurrencyValue::Int(_));

        // Python:
        //   if isinstance(val, int) or (isinstance(val, float) and val == int(val)):
        //       left, right, is_negative = parse_currency_parts(
        //           int(val) if isinstance(val, float) else val,
        //           is_int_with_cents=False)
        //   else:
        //       left, right, is_negative = parse_currency_parts(
        //           val, is_int_with_cents=False,
        //           keep_precision=has_fractional_cents)
        let (left, right, is_negative) = match val {
            CurrencyValue::Int(_) => parse_currency_parts(val, false, false, 100),
            // `int(val)` truncates and re-enters as a Python int, which sends
            // parse_currency_parts down its integer arm (cents := 0).
            CurrencyValue::Decimal { value, .. } if is_whole_float => {
                let truncated =
                    CurrencyValue::Int(value.with_scale(0).as_bigint_and_exponent().0);
                parse_currency_parts(&truncated, false, false, 100)
            }
            _ => parse_currency_parts(val, false, has_fractional_cents, 100),
        };

        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        let mut cr1 = forms.unit.clone();
        let cr2 = forms.subunit.clone();
        if adjective {
            if let Some(adj) = self.currency_adjective(currency) {
                cr1 = prefix_currency(adj, &cr1);
            }
        }

        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        let money_str = self.money_verbose(&left, currency)?;

        // Python:
        //   is_whole_float = isinstance(val, float) and val == int(val)
        //   has_decimal = (isinstance(val, float) and not is_whole_float) \
        //                 or str(val).find(".") != -1
        //   if (has_decimal and not is_whole_float) or right > 0:
        //
        // Only the conjunction `has_decimal and not is_whole_float` is ever
        // read, and it collapses to "val is a float that is not whole":
        //   * is_whole_float     -> the conjunction is False whatever
        //     has_decimal says;
        //   * not is_whole_float -> a float makes has_decimal's first disjunct
        //     True, while an int makes both disjuncts False (str(int) has no
        //     dot).
        // So `str(val)` never actually matters, which is fortunate because
        // repr(float) deliberately lives on the Python side. (It is not even
        // faithful to guess it: str(1e21) is "1e+21", with no dot, so EL
        // computes has_decimal=False there — and the guard is unaffected
        // because 1e21 is a whole float anyway.)
        let has_decimal_and_not_whole = !is_int && !is_whole_float;

        // The `or right > 0` tail is dead: `right` is only ever non-zero on the
        // non-whole-float path, where the first term is already True. Kept for
        // shape.
        if has_decimal_and_not_whole || right > BigDecimal::zero() {
            let cents_str = if has_fractional_cents {
                // Python guards this with `isinstance(right, Decimal) and
                // has_fractional_cents`, but `right` is a Decimal exactly when
                // keep_precision was set, i.e. exactly when
                // has_fractional_cents — the isinstance half is redundant.
                if cents {
                    // Python: `self.to_cardinal_float(float(right))`. EL does
                    // not override to_cardinal_float, so the default float path
                    // is the same function.
                    self.cardinal_from_decimal(&right)?
                } else {
                    // Python: `str(float(right))`.
                    //
                    // KNOWN GAP, deliberately not papered over. Rust's `{}` for
                    // f64 is shortest-round-trip, the same digits as Python's
                    // repr, but the two disagree on *notation*: Python switches
                    // to exponent form below 1e-4, Rust never does. `right` is
                    // cents, so it is always in [0, 100) and the >=1e16 half of
                    // Python's rule is unreachable — but the low half is not.
                    // At 1.0000001 EUR with cents=False, `right` is 1e-05 and
                    // Python prints "1e-05" where this prints "0.00001".
                    //
                    // Reachable only via cents=False AND >= 7 decimal places;
                    // no corpus row covers it. Fixing it means reimplementing
                    // repr(float)'s notation rule, which `currency.rs` warns is
                    // "a permanent source of drift" — so it is reported in
                    // `concerns` instead of guessed at here.
                    let f = right
                        .to_f64()
                        .ok_or_else(|| N2WError::Value(format!("cannot represent {}", right)))?;
                    format!("{}", f)
                }
            } else {
                // `int(right) if isinstance(right, Decimal) else right` —
                // right is already integral on this path, so truncating is
                // exact.
                let right_int = right.with_scale(0).as_bigint_and_exponent().0;
                if cents {
                    self.cents_verbose(&right_int, currency)?
                } else {
                    self.cents_terse(&right_int, currency)?
                }
            };

            // Python's single format, with `right` still a Decimal when the
            // cents are fractional: `self.pluralize(right, cr2)`. Comparing the
            // BigDecimal against one reproduces `n == 1` for both shapes —
            // Decimal("1.100") != 1, while Decimal("1.000") == 1, exactly as
            // BigDecimal's numeric PartialEq behaves.
            return Ok(format!(
                "{}{} {}{} {} {}",
                minus_str,
                money_str,
                self.pluralize(&left, &cr1)?,
                separator,
                cents_str,
                self.pluralize_forms(right == BigDecimal::one(), &cr2)?,
            ));
        }

        Ok(format!(
            "{}{} {}",
            minus_str,
            money_str,
            self.pluralize(&left, &cr1)?
        ))
    }
}

/// Python's `Num2Word_Base.verify_ordinal` over the float/Decimal domain:
///
/// ```python
/// if not value == int(value):
///     raise TypeError(self.errmsg_floatord % value)
/// if not abs(value) == value:
///     raise TypeError(self.errmsg_negord % value)
/// ```
///
/// The float check runs first, so `-1.5` reports the *float* message; the
/// negative check compares numerically, so `-0.0` (== `abs(-0.0)`) passes.
/// `repr` is Python's `str(value)`, interpolated verbatim into the message.
fn verify_ordinal_float(value: &FloatValue, repr: &str) -> Result<BigInt> {
    match value.as_whole_int() {
        None => Err(N2WError::Type(format!(
            "Cannot treat float {} as ordinal.",
            repr
        ))),
        Some(i) if i.is_negative() => Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            repr
        ))),
        Some(i) => Ok(i),
    }
}

/// Best-effort `str(value)` for the TypeError messages raised by
/// [`verify_ordinal_float`] when no repr was handed in (the `to_ordinal`
/// entry). Exact for every reachable message: `str(Decimal)` is the spec
/// transcription, a fractional float's shortest round-trip matches Rust's
/// `Display` in the non-exponent range, and a whole float re-gains its
/// Python ".0" tail. (A whole float >= 1e16 would diverge — Python uses
/// exponent form — but such values pass verification and never reach a
/// message.)
fn ordinal_float_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
        FloatValue::Float { value, .. } => {
            if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e16 {
                format!("{:.1}", value)
            } else {
                value.to_string()
            }
        }
    }
}
