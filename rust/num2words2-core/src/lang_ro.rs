//! Port of `lang_RO.py` (Romanian).
//!
//! Shape: **engine**. `Num2Word_RO` subclasses `lang_EUR.Num2Word_EUR` which
//! subclasses `Num2Word_Base`. RO supplies `mid_numwords`/`low_numwords` and a
//! `merge`, and inherits EUR's `set_high_numwords` + `gen_high_numwords`, so
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean` exactly as in
//! `lang_en.rs`.
//!
//! Inheritance chain walked: `Num2Word_RO` → `Num2Word_EUR` (set_high_numwords,
//! gen_high_numwords, setup's `high_numwords`) → `Num2Word_Base` (to_cardinal,
//! splitnum, clean, verify_ordinal, to_year). `lang_EU` (Basque) is unrelated
//! despite the similar name and is *not* in this chain.
//!
//! # Long scale
//!
//! EUR's `set_high_numwords` steps -6 and emits a GIGA/MEGA pair per stem:
//! `cap = 3 + 6*len(high)` = 603 for RO's 100 stems, so cards run from
//! 10^603 ("centiliard/e") down to 10^9 ("miliard/e") / 10^6 ("milion"), and
//! `MAXVAL = 1000 * 10^603` = 10^606. Values are therefore genuinely BigInt.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is wrong-looking but is
//! exactly what Python emits, and every item is pinned by a corpus row.
//!
//! 1. **`numwords_inflections` only covers 100 / 1000 / 10^6 / 10^9.** In
//!    `merge`'s `1 <= lnum < 10` branch, a right operand outside that set makes
//!    the branch `return (rtext, rnum)` — **discarding `ltext` entirely**.
//!    Hence `to_cardinal(10**12)` == "bilion" (not "un bilion"), and by the
//!    same path `to_cardinal(2 * 10**12)` == "bilion" too: the multiplier word
//!    is silently dropped for every card at 10^12 and above. Corpus rows:
//!    `10**12` → "bilion", `10**18` → "trilion".
//! 2. **Un-inflected slash forms leak into output.** Because of bug 1 the raw
//!    card text is returned before `inflect` ever splits it, so the "word/plural"
//!    table syntax reaches the caller verbatim: `to_cardinal(10**15)` ==
//!    "biliard/e" and `to_cardinal(10**21)` == "triliard/e" (GIGA_SUFFIX is
//!    declared as "iliard/e"). Corpus-confirmed, slash and all.
//! 3. **`merge` returns arithmetically wrong `num` values.** The `1 <= lnum < 10`
//!    branch returns `rnum` rather than `lnum * rnum` (so "trei sute" carries
//!    100, not 300), and the `else` branch returns `lnum * rnum` where addition
//!    is meant (so "trei sute patruzeci și cinci" carries 100*45 = 4500). The
//!    numbers are only ever re-consumed by the next `merge`/`inflect` as a
//!    plural selector, and `pluralize` keys on `n % 100`, which happens to stay
//!    correct for these bogus values — the output is right by luck, not design.
//!    Preserved exactly, since a "fix" would change `pluralize`'s input.
//! 4. **`inflect` ignores its `value` parameter.** It pluralizes on
//!    `side_effect` (the caller's `lnum`) and never reads `value`. See
//!    [`LangRo::inflect`].
//! 5. **The "iliare" round-trip.** `inflect` builds the plural of "miliard/e"
//!    as `text[0][:-1] + text[1]` = "miliare", then a follow-up `elif` rewrites
//!    the "iliare" tail to GIGA_SUFFIX_I ("iliarde") to repair it — so the
//!    intermediate "miliare"/"de miliare" is a real, load-bearing state.
//! 6. **`self.ords` is dead code.** `setup` builds an `ords` dict (containing a
//!    stray English key "three") that `to_ordinal` never reads — RO's
//!    `to_ordinal` is pure string concatenation. Not ported; it has no
//!    observable effect.
//! 7. **`to_ordinal` just glues "lea" on.** `"al %slea" % cardinal`, so
//!    `to_ordinal(10**15)` == "al biliard/elea" — the slash survives into
//!    the ordinal too. Python also rewrote "o sută"/"o mie" to "una …"
//!    ("al una sutălea", "al una mielea"); fixed (gladiaio/num2words2#252):
//!    "al o sutălea", "al o miilea", "al o mie unulea", "al un
//!    milionulea".
//!
//! # Cross-call mutable state: `to_currency`'s `gen_numwords[1]` flip
//!
//! `Num2Word_RO.to_currency` mutates shared state that `to_cardinal` reads:
//! `self.gen_numwords[1] = "una"` → `super().to_currency(...)` → revert to "o".
//! `numwords_inflections[100]` and `[1000]` alias that same list object, so
//! while the flag is set, `to_cardinal(100)` yields "una sută" instead of
//! "o sută" — which is exactly why the corpus says `currency:EUR 100` is
//! "una sută de euro" while `cardinal 100` is "o sută". `gen_numwords_n`
//! (10^6 / 10^9) is a *different* list and is never touched, which is why
//! `currency:EUR 1000000` stays "un milion de euro".
//!
//! The mutation is modelled here as [`RoUna`] — a borrowed view over the same
//! `LangRo` whose only difference is that `merge` reads
//! [`GEN_NUMWORDS_UNA`] where the pristine path reads [`GEN_NUMWORDS`].
//! `to_currency` drives `currency::default_to_currency` through that view, so
//! every `money_verbose`/`cents_verbose` → `to_cardinal` hop underneath sees
//! "una", exactly as Python's mutated list does. Nothing is allocated per
//! call and `LangRo` stays `Sync` (the registry hands out
//! `&'static (dyn Lang + Sync)`, so `Cell` was not an option).
//!
//! # The poisoning on the error path, and where it survives
//!
//! Python's revert is not in a `try/finally`, so any raise inside
//! `super().to_currency` leaves `gen_numwords[1] == "una"` forever. Because
//! `CONVERTER_CLASSES` holds one *instance* per language, that poisoning is
//! process-wide: after a single failed `to_currency`, `to_cardinal(100)`
//! answers "una sută" for the rest of the interpreter's life. It is visible in
//! the frozen corpus — the `w2n_cardinal` rows (emitted after the
//! `currency:GBP` → NotImplementedError rows) spell 100 "una sută" and 1000
//! "una mie", while the `cardinal` rows (emitted before any currency call)
//! spell them "o sută" / "o mie".
//!
//! A scoped view cannot poison anything, so the question is whether the leak
//! is observable *through the dispatcher*. Two exception types can escape RO's
//! `to_currency`, and they behave differently:
//!
//! * **`NotImplementedError`** (currency absent from RO's `CURRENCY_FORMS` —
//!   everything but RON/EUR/USD). `__init__.py`'s Rust fast path wraps the
//!   call in `except NotImplementedError: pass` and falls through to the
//!   Python converter, which then runs `to_currency` for real and poisons the
//!   instance exactly as before. **Behaviour preserved**, by luck rather than
//!   design, but preserved.
//! * **`OverflowError`** (`abs(val) >= MAXVAL`, i.e. 10^606, raised by
//!   `to_cardinal` underneath `_money_verbose`). The fast path does *not*
//!   catch this, so the Python converter never runs and the instance stays
//!   clean where Python would have poisoned it. The raised type still matches
//!   (verified: both sides give OverflowError for `to_currency(10**700, "RON")`).
//!   **This is the one divergence**, and it is only observable by making a
//!   >= 10^606 currency call and then inspecting a *later* `to_cardinal(100)`.
//!
//! Every corpus row matches either way: the poisoning only ever escapes into
//! the four integer modes, which the corpus emits *before* the first currency
//! call. Note `cheque:EUR`/`cheque:USD` are emitted after 12 *successful*
//! `currency:*` rows, each of which reverts cleanly, so they legitimately read
//! "O MIE …", not "UNA MIE …". Flagged in the report regardless.

use crate::base::{
    clean, set_low_numwords, set_mid_numwords, splitnum, Cards, KwVal, Kwargs, Lang, N2WError, Node,
    Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `GIGA_SUFFIX`. The "/e" is table syntax that `inflect` is meant to split —
/// but bug 2 above lets it escape into output unsplit for 10^15 and beyond.
const GIGA_SUFFIX: &str = "iliard/e";
const MEGA_SUFFIX: &str = "ilion";
/// `MEGA_SUFFIX_I` — "inflection for mi/billion follows different rule".
const MEGA_SUFFIX_I: &str = "ilioane";
const GIGA_SUFFIX_I: &str = "iliarde";

/// `gen_numwords`: the feminine multiplier words used before "sută"/"mie".
///
/// Index 0 is "" and is never reached (`merge` guards with `1 <= lnum < 10`).
/// Index 1 is the "o" that `to_currency` temporarily rewrites to "una" — see
/// the cross-call-state note in the module docs. Pristine value kept here.
const GEN_NUMWORDS: [&str; 10] = [
    "", "o", "două", "trei", "patru", "cinci", "șase", "șapte", "opt", "nouă",
];

/// `gen_numwords` as it reads *during* `to_currency`, i.e. after
/// `self.gen_numwords[1] = "una"`.
///
/// "romanian currency has a particularity for numeral: one". Differs from
/// [`GEN_NUMWORDS`] at index 1 only; [`RoUna`] swaps this in for the duration
/// of the currency call. Kept as a separate const rather than mutated in place
/// because the registry requires `LangRo: Sync`.
const GEN_NUMWORDS_UNA: [&str; 10] = [
    "", "una", "două", "trei", "patru", "cinci", "șase", "șapte", "opt", "nouă",
];

/// `gen_numwords_n`: the masculine/neuter variant used before "milion"/"miliard".
/// Differs from [`GEN_NUMWORDS`] only at index 1 ("un" vs "o").
///
/// `to_currency` does **not** touch this list — it is a distinct object in
/// Python — so 10^6 and 10^9 keep saying "un" even in the "una" mode.
const GEN_NUMWORDS_N: [&str; 10] = [
    "", "un", "două", "trei", "patru", "cinci", "șase", "șapte", "opt", "nouă",
];

/// `Num2Word_RO.CURRENCY_FORMS`.
///
/// RO declares this in its own class body, so — unlike the 16 classes that
/// read the dict `Num2Word_EN.__init__` mutates in place — it is a *separate*
/// dict and sees none of English's additions. Confirmed against the live
/// interpreter: `'CURRENCY_FORMS' in Num2Word_RO.__dict__` is `True`, and the
/// runtime table is exactly these three codes. Everything else (GBP, JPY, KWD,
/// BHD, INR, CNY, CHF, …) therefore raises NotImplementedError, which is what
/// the corpus expects for all 84 of those rows.
///
/// All six tuples carry **three** forms, and RO's `pluralize` really does index
/// `[2]` (the "de …" form) — dropping it would break every value >= 20.
///
/// Note `cenţi`: the source uses U+0163 (t with cedilla), not the U+021B
/// (t with comma) used by `și`/`șase` elsewhere in the file. Copied verbatim.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 3] = ["cent", "cenţi", "de cenţi"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "RON",
        CurrencyForms::new(&["leu", "lei", "de lei"], &["ban", "bani", "de bani"]),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(&["euro", "euro", "de euro"], &CENTS),
    );
    m.insert(
        "USD",
        CurrencyForms::new(&["dolar", "dolari", "de dolari"], &CENTS),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — RO declares no
/// `CURRENCY_ADJECTIVES` of its own, and nothing mutates EUR's (only
/// `CURRENCY_FORMS` is written to in place by `Num2Word_EN.__init__`).
/// Verified against the live interpreter.
///
/// Only RON and USD overlap RO's `CURRENCY_FORMS`, so the other 14 entries are
/// dead: `adjective=True` on a code RO cannot render raises before the
/// adjective is ever consulted. Kept because the inherited table is the ported
/// data, not a curated subset.
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

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Kept local rather than reused from `lang_en.rs`: the porting contract is one
/// file per language, and depending on a sibling language module would couple
/// this file to a registry that is generated later.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    // Python: [u + t for t in tens for u in units] — tens is the OUTER loop.
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

/// `rnum in self.numwords_inflections` plus the dict lookup, in one step.
///
/// Free rather than a method so that both the pristine table and the "una"
/// table can be passed in; the membership test *and* the selected table both
/// matter to `merge`.
fn lookup_inflection(
    table: &[(BigInt, &'static [&'static str; 10])],
    key: &BigInt,
) -> Option<&'static [&'static str; 10]> {
    table.iter().find(|(k, _)| k == key).map(|(_, v)| *v)
}

/// RO's `errmsg_toobig`. The Python literal uses a backslash line
/// continuation, so the real string reads "...pentru a fi convertit..."
/// with a single space. Note Python tests `value >= MAXVAL` but the message
/// text claims `abs(value) > MAXVAL` — kept verbatim.
fn errmsg_toobig(v: &BigInt, maxval: &BigInt) -> N2WError {
    N2WError::Overflow(format!(
        "Numărul e prea mare pentru a fi convertit în cuvinte (abs({}) > {}).",
        v, maxval
    ))
}

/// `Num2Word_Base.to_cardinal` for RO (which does not override it).
///
/// Generic over the driving `Lang` because `splitnum`/`clean` dispatch `merge`
/// through it: [`LangRo`] enters here with the pristine `gen_numwords`, and
/// [`RoUna`] with the `to_currency` "una" flip. Reimplemented rather than
/// delegated to `base::default_to_cardinal` for one reason only: RO overrides
/// `errmsg_toobig` with a Romanian message. The success path is identical to
/// the default.
fn ro_to_cardinal<L: Lang + ?Sized>(lang: &L, value: &BigInt) -> Result<String> {
    let mut out = String::new();
    let mut v = value.clone();
    if v.is_negative() {
        v = v.abs();
        // Python: "%s " % self.negword.strip() — "minus " → "minus".
        out = format!("{} ", lang.negword().trim());
    }

    if &v >= lang.maxval() {
        return Err(errmsg_toobig(&v, lang.maxval()));
    }

    // Unreachable in practice: card 0 exists, so splitnum matches any
    // non-negative value below MAXVAL. Mapped to the same error for safety.
    let tree = splitnum(lang, &v).ok_or_else(|| errmsg_toobig(&v, lang.maxval()))?;
    let words = match clean(lang, tree) {
        Node::Leaf(t, _) => t,
        Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
    };
    Ok(lang.title(&format!("{}{}", out, words)))
}

pub struct LangRo {
    cards: Cards,
    maxval: BigInt,
    /// `numwords_inflections`: {100: gen_numwords, 1000: gen_numwords,
    /// 10^6: gen_numwords_n, 10^9: gen_numwords_n}. Membership *and* the
    /// selected table both matter to `merge`.
    numwords_inflections: Vec<(BigInt, &'static [&'static str; 10])>,
    /// The same table as `numwords_inflections`, except that the two entries
    /// aliasing `gen_numwords` (100 and 1000) point at [`GEN_NUMWORDS_UNA`].
    /// This *is* Python's `self.gen_numwords[1] = "una"`, expressed without
    /// mutation; see [`RoUna`].
    numwords_inflections_una: Vec<(BigInt, &'static [&'static str; 10])>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangRo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangRo {
    /// The era handling of `Num2Word_RO.to_year`, applied to the cardinal
    /// `result`. negword is "minus " (with the trailing space); to_cardinal
    /// emits exactly that prefix for negatives. str.replace is global in both
    /// languages, matching Python's `.replace(negword, "")`. A negative year
    /// defaults to "î.Hr." unless the caller passed a suffix; a caller's
    /// suffix is appended to positive years too.
    fn year_suffix(&self, mut result: String, suffix: Option<&str>) -> String {
        let mut suffix = suffix;
        if result.starts_with(self.negword()) {
            result = result.replace(self.negword(), "");
            if suffix.is_none() {
                suffix = Some("î.Hr.");
            }
        }
        match suffix {
            Some(s) => format!("{} {}", result, s),
            None => result,
        }
    }

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

        // Num2Word_EUR.set_high_numwords:
        //   cap = 3 + 6*len(high); zip(high, range(cap, 3, -6))
        //   cards[10**n]     = word + GIGA_SUFFIX
        //   cards[10**(n-3)] = word + MEGA_SUFFIX
        // zip() stops at the shorter sequence; here both are 100 long, but the
        // `n > 3` guard reproduces range()'s stop condition regardless.
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
                (1000, "mie/i"),
                (100, "sută/e"),
                (90, "nouăzeci"),
                (80, "optzeci"),
                (70, "șaptezeci"),
                (60, "șaizeci"),
                (50, "cincizeci"),
                (40, "patruzeci"),
                (30, "treizeci"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "douăzeci",
                "nouăsprezece",
                "optsprezece",
                "șaptesprezece",
                "șaisprezece",
                "cincisprezece",
                "paisprezece",
                "treisprezece",
                "doisprezece",
                "unsprezece",
                "zece",
                "nouă",
                "opt",
                "șapte",
                "șase",
                "cinci",
                "patru",
                "trei",
                "doi",
                "unu",
                "zero",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]; the OrderedDict's first
        // key is the highest card (10^603), so MAXVAL == 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let numwords_inflections: Vec<(BigInt, &'static [&'static str; 10])> = vec![
            (BigInt::from(100), &GEN_NUMWORDS),
            (BigInt::from(1000), &GEN_NUMWORDS),
            (BigInt::from(1_000_000i64), &GEN_NUMWORDS_N),
            (BigInt::from(1_000_000_000i64), &GEN_NUMWORDS_N),
        ];
        // Only the two `gen_numwords` aliases flip; `gen_numwords_n` is a
        // different list object in Python and `to_currency` never writes to it.
        let numwords_inflections_una: Vec<(BigInt, &'static [&'static str; 10])> = vec![
            (BigInt::from(100), &GEN_NUMWORDS_UNA),
            (BigInt::from(1000), &GEN_NUMWORDS_UNA),
            (BigInt::from(1_000_000i64), &GEN_NUMWORDS_N),
            (BigInt::from(1_000_000_000i64), &GEN_NUMWORDS_N),
        ];

        LangRo {
            cards,
            maxval,
            numwords_inflections,
            numwords_inflections_una,
            exclude_title: vec!["și".into(), "virgulă".into(), "minus".into()],
            // Built once here, never per call: `to_currency` and `to_cheque`
            // only ever read these.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_RO.pluralize`'s form selector, split out so that the
    /// `inflect` call sites (which index a fixed 3-tuple and cannot fail) and
    /// the `Lang::pluralize` hook (which takes an arbitrary-length slice and
    /// must be able to raise IndexError) share one copy of the rule.
    ///
    /// `n % 100` uses Python's floor semantics, hence `mod_floor`. `n` is
    /// non-negative on every path the four in-scope modes can reach (it is
    /// always a `merge` `lnum`, derived from `abs(value)`), and the currency
    /// path only ever passes `abs()`-ed parts — but the default
    /// `side_effect=-1` in `inflect`'s signature would land here with -1, where
    /// Python's `-1 % 100 == 99` (Rust's `%` gives -1). No call site uses that
    /// default; `mod_floor` keeps it correct anyway.
    fn plural_form(n: &BigInt) -> usize {
        if n.is_one() {
            0
        } else if n.is_zero() {
            1
        } else {
            let m = n.mod_floor(&BigInt::from(100));
            if m > BigInt::zero() && m < BigInt::from(20) {
                1
            } else {
                2
            }
        }
    }

    /// `Num2Word_RO.pluralize`, for `inflect`'s fixed 3-form tuples.
    fn pluralize(n: &BigInt, forms: &[String; 3]) -> String {
        forms[Self::plural_form(n)].clone()
    }

    /// `Num2Word_RO.merge`, with the `numwords_inflections` table threaded in.
    ///
    /// `inflections` is `self.numwords_inflections` on the pristine path and
    /// `self.numwords_inflections_una` while `to_currency` holds
    /// `gen_numwords[1] == "una"`. Threading it is the *only* difference from
    /// the previously verified body; every branch is otherwise unchanged.
    fn merge_with(
        &self,
        l: (&str, &BigInt),
        r: (&str, &BigInt),
        inflections: &[(BigInt, &'static [&'static str; 10])],
    ) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let one = BigInt::one();
        let two = BigInt::from(2);
        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);

        // Python computes `rtext_i = self.inflect(rnum, rtext, lnum)` up front
        // and then overwrites it inside most branches. `inflect` is pure, so
        // computing it only where it is actually read is byte-identical; the
        // fallback below is that original up-front expression.
        if &one <= lnum && lnum < &ten {
            match lookup_inflection(inflections, rnum) {
                // Bug 1: ltext is dropped on the floor for any rnum outside
                // {100, 1000, 10^6, 10^9} — this is what makes 10^12 "bilion"
                // and 2*10^15 "biliard/e".
                None => (rtext.to_string(), rnum.clone()),
                Some(table) => {
                    let rtext_i = self.inflect(&(lnum * rnum), rtext, lnum);
                    // lnum is 1..=9 here, so the index is always in range.
                    let idx = lnum.to_usize().expect("1 <= lnum < 10");
                    // Bug 3: returns rnum, not lnum * rnum.
                    (format!("{} {}", table[idx], rtext_i), rnum.clone())
                }
            }
        } else if &ten < lnum && lnum < &hundred {
            if lnum.mod_floor(&ten).is_zero() {
                if lookup_inflection(inflections, rnum).is_some() {
                    let rtext_i = self.inflect(&(lnum * rnum), rtext, lnum);
                    (format!("{} {}", ltext, rtext_i), lnum * rnum)
                } else {
                    // Note: raw `rtext`, not the inflected form.
                    (format!("{} și {}", ltext, rtext), lnum + rnum)
                }
            } else {
                let rtext_i = self.inflect(&(lnum * rnum), rtext, lnum);
                // "douăzeci și doi" + "mie/i" → "douăzeci și două de mii".
                // Python replaces EVERY "doi", so Rust's replace-all matches.
                let ltext_i = if lnum.mod_floor(&ten) != two {
                    ltext.to_string()
                } else {
                    ltext.replace("doi", "două")
                };
                (format!("{} {}", ltext_i, rtext_i), lnum * rnum)
            }
        } else {
            // Reached for lnum == 0, lnum == 10 exactly, and lnum >= 100.
            let rtext_i = if lookup_inflection(inflections, rnum).is_some() {
                self.inflect(&(lnum * rnum), rtext, lnum)
            } else {
                self.inflect(rnum, rtext, lnum)
            };
            // Bug 3: multiplies where "o sută" + "patruzeci și cinci" means add.
            (format!("{} {}", ltext, rtext_i), lnum * rnum)
        }
    }

    /// `Num2Word_RO.inflect`.
    ///
    /// `_value` mirrors Python's `value` parameter, which the method **never
    /// reads** — all pluralization keys on `side_effect` (bug 4). Kept in the
    /// signature so the call sites stay legible against the original.
    ///
    /// The method is pure: no `self` mutation, despite the parameter name.
    fn inflect(&self, _value: &BigInt, text: &str, side_effect: &BigInt) -> String {
        let parts: Vec<&str> = text.split('/').collect();
        let mut result = parts[0].to_string();

        if parts.len() > 1 {
            // Python: text[0][:-1] + text[1] — drop the last CHARACTER, not
            // byte: "sută"[:-1] == "sut", and "ă" is two bytes in UTF-8.
            let chars: Vec<char> = parts[0].chars().collect();
            let stem: String = chars[..chars.len().saturating_sub(1)].iter().collect();
            let plural = format!("{}{}", stem, parts[1]);
            let forms = [
                parts[0].to_string(),
                plural.clone(),
                format!("de {}", plural),
            ];
            result = Self::pluralize(side_effect, &forms);
        }

        let one = BigInt::one();
        // "mega inflections also need de/no-de pluralization" — note this tests
        // the WHOLE accumulated string, so a merged phrase that happens to end
        // in "...ilion" (e.g. the right operand "un milion" under a 10^12 left)
        // gets mangled into "de un milioane". Faithful: same test as Python.
        if side_effect > &one && result.ends_with(MEGA_SUFFIX) {
            let inflected = result.replace(MEGA_SUFFIX, MEGA_SUFFIX_I);
            let forms = [
                result.clone(),
                inflected.clone(),
                format!("de {}", inflected),
            ];
            result = Self::pluralize(side_effect, &forms);
        } else if side_effect > &one && result.ends_with("iliare") {
            // Repairs the "miliard"→"miliare" mis-plural built above (bug 5).
            result = result.replace("iliare", GIGA_SUFFIX_I);
        }
        result
    }

    /// `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`value == int(value)`) cannot fail for BigInt input;
    /// only the `abs(value) == value` check is reachable. Both raise TypeError
    /// in Python (errmsg_negord), matching the corpus's `TypeError` rows for
    /// every negative ordinal / ordinal_num.
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

impl Lang for LangRo {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "RON"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " și"
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
        "virgulă"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// `Num2Word_RO.merge`, against the pristine `gen_numwords`.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        self.merge_with(l, r, &self.numwords_inflections)
    }

    /// `Num2Word_Base.to_cardinal` (RO does not override it).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        ro_to_cardinal(self, value)
    }

    /// `Num2Word_RO.to_ordinal`.
    ///
    /// Concatenation — `self.ords` is never consulted (bug 6) — with the
    /// DOOM stems for a final "mie"/"milion" (bug 7).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("primul".to_string());
        }
        let mut cardinal = self.to_cardinal(value)?;
        // Python rewrote "o sută"/"o mie" to "una …" ("al una sutălea");
        // DOOM keeps the feminine article: "al o sutălea", "al o mie
        // unulea". A final "mie" takes the plural stem, "al o miilea", and a
        // final "milion" the linking "u", "al un milionulea" (#252).
        if cardinal.ends_with("mie") {
            cardinal.truncate(cardinal.len() - "e".len());
            cardinal.push('i');
        } else if cardinal.ends_with("milion") {
            cardinal.push('u');
        }
        Ok(format!("al {}lea", cardinal))
    }

    /// `Num2Word_RO.to_ordinal_num`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("1-ul".to_string());
        }
        Ok(format!("al {}-lea", value))
    }

    /// `Num2Word_RO.to_year`, `suffix=None` path.
    ///
    /// `super().to_year` is `Num2Word_Base.to_year(value, **kwargs)`, which
    /// swallows `longval` and just returns `to_cardinal(value)`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(self.year_suffix(self.to_cardinal(value)?, None))
    }

    /// `Num2Word_RO.to_year(val, suffix=None, longval=True)` with the
    /// caller's kwargs. `longval` is accepted and ignored (base swallows it).
    /// `suffix` is tested truthily, so `suffix=""` behaves like `None`;
    /// non-str values are declined rather than guessing at Python's join.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let suffix = match kw.get("suffix") {
            None | Some(KwVal::None) => None,
            Some(KwVal::Str(s)) if s.is_empty() => None,
            Some(KwVal::Str(s)) => Some(s.as_str()),
            _ => return Err(N2WError::Fallback("kwargs".into())),
        };
        Ok(self.year_suffix(self.to_cardinal(value)?, suffix))
    }

    /// `to_ordinal(float/Decimal)`. `verify_ordinal` raises TypeError for a
    /// non-integral value first, then for negatives — the negative check is
    /// reproduced by `to_ordinal(BigInt)`'s own verify. `-0.0` passes both
    /// (`abs(-0.0) == -0.0`) and renders as "al zerolea".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_ordinal(&i),
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal`, then the literal
    /// `"1-ul"` when `value == 1` (so `1.0` → "1-ul", not "1.0-ul"), else
    /// `"al %s-lea" % value` with Python's `str(value)`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    return Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )));
                }
                if i.is_one() {
                    return Ok("1-ul".to_string());
                }
                Ok(format!("al {}-lea", repr_str))
            }
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_year(float/Decimal)`: base's `to_year` is `to_cardinal(val)`;
    /// RO then strips a leading negword (a *global* `str.replace`) and
    /// appends " î.Hr." — "unu virgulă cinci î.Hr." for -1.5.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(self.year_suffix(self.cardinal_float_entry(value, None)?, None))
    }

    // ---- currency -------------------------------------------------------
    //
    // RO overrides `CURRENCY_FORMS`, `pluralize` and `to_currency`. It
    // inherits `CURRENCY_ADJECTIVES` from `Num2Word_EUR` and everything else
    // (`to_cheque`, `_money_verbose`, `_cents_verbose`, `_cents_terse`) from
    // `Num2Word_Base` unchanged, so the trait defaults already mirror those.
    //
    // `currency_precision` is *not* overridden on purpose: `Num2Word_Base`'s
    // `CURRENCY_PRECISION` is `{}` and neither RO nor EUR defines one (EN's
    // `self.CURRENCY_PRECISION = {...}` rebinds an *instance* attribute and so
    // does not leak). `{}.get(code, 100)` == 100 for every code, which is
    // exactly the trait default. RO therefore has no 3-decimal and no
    // 0-decimal currency: KWD/BHD raise NotImplementedError before precision
    // is consulted, and `default_to_currency`'s `divisor == 1` branch is
    // unreachable — JPY, absent from RO's table, raises too.

    fn lang_name(&self) -> &str {
        "Num2Word_RO"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_RO.pluralize` — the three-way Romanian rule:
    /// 1 → form 0, {0} ∪ {n : 0 < n % 100 < 20} → form 1, else form 2 ("de …").
    ///
    /// Python indexes the tuple directly, so a form list shorter than the
    /// selected index raises IndexError. Every entry in RO's table carries all
    /// three forms, so that is unreachable — but it is mapped to `Index`
    /// rather than panicking so the exception type survives if the table
    /// changes. `prefix_currency` (the `adjective=True` path) preserves arity,
    /// so it cannot shorten a tuple either.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(Self::plural_form(n))
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_RO.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="RON", cents=False,
    ///                 separator=" și", adjective=False):
    ///     self.gen_numwords[1] = "una"
    ///     result = super().to_currency(val, currency, True, separator, adjective)
    ///     self.gen_numwords[1] = "o"
    ///     return (result.replace("unu leu", "un leu")
    ///                   .replace("unu ban", "un ban")
    ///                   .replace(separator + " zero bani", ""))
    /// ```
    ///
    /// Four things to note, all load-bearing:
    ///
    /// 1. **`cents` is accepted and thrown away.** The super call hardcodes
    ///    `True`, so `to_currency(12.34, "EUR", cents=False)` still spells the
    ///    cents out ("… și treizeci și patru de cenţi") instead of taking
    ///    `_cents_terse`'s "34". Verified against the live interpreter.
    /// 2. **The three replaces are RON-only and order-dependent.** "unu leu" →
    ///    "un leu" runs before "unu ban" → "un ban", and the zero-cents strip
    ///    matches the literal word "bani" — so `1.0 RON` collapses all the way
    ///    to "un leu" ("unu leu și zero bani" → "un leu și zero bani" → "un
    ///    leu") while `1.0 EUR` keeps its tail as "unu euro și zero cenţi".
    ///    The replaces are also blind to the adjective prefix: `1 RON` with
    ///    `adjective=True` gives "unu Romanian leu", not "un Romanian leu".
    /// 3. **The strip is keyed on the caller's `separator`**, not on RO's
    ///    default, so `separator=","` strips ", zero bani" and still yields
    ///    "un leu". Both confirmed against the live interpreter.
    /// 4. **"unu ban" bleeds into "unu bani".** `str.replace` is a substring
    ///    op, and "unu ban" is a prefix of "unu bani", so the fractional-cents
    ///    branch — which takes `cr2[1]` ("bani") unconditionally, without
    ///    consulting `pluralize` — gets rewritten mid-word:
    ///    `to_currency(1.011, "RON")` == "un leu și unu virgulă **un bani**",
    ///    which is not grammatical Romanian. Ported as-is.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        // Accepted and never read — the super call hardcodes `cents=True`.
        _cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it through this language's own default (" și").
        let separator = separator.unwrap_or(self.default_separator());

        // `self.gen_numwords[1] = "una"` … `super().to_currency(...)`.
        // On the Err path Python would skip its revert and poison the shared
        // instance; the scoped view cannot — see the module docs.
        let result = default_to_currency(&RoUna(self), val, currency, true, separator, adjective)?;

        // `self.gen_numwords[1] = "o"` — implicit: the view ends here.
        Ok(result
            .replace("unu leu", "un leu")
            .replace("unu ban", "un ban")
            // "if the romanian low text is 0, it is not usually printed"
            .replace(&format!("{} zero bani", separator), ""))
    }
}

/// `Num2Word_RO` as it reads *inside* `to_currency`, i.e. with
/// `self.gen_numwords[1] == "una"`.
///
/// Python expresses the flip by mutating the shared list and reverting after
/// `super().to_currency` returns. The registry hands out
/// `&'static (dyn Lang + Sync)`, so a `Cell` on `LangRo` is not available;
/// instead this borrows the very same `LangRo` and overrides the single method
/// the flip can reach — `merge` — to read `numwords_inflections_una`. Every
/// other hook forwards, so `default_to_currency`'s
/// `money_verbose`/`cents_verbose` → `to_cardinal` hops re-enter `splitnum` /
/// `clean` here and dispatch `merge` on this view.
///
/// Borrowed, never cloned: constructing one is a pointer copy, so the
/// per-call cost is nil and the cards table is shared.
struct RoUna<'a>(&'a LangRo);

impl Lang for RoUna<'_> {
    // ---- the flip -------------------------------------------------------

    /// The one method that differs: `gen_numwords[1]` reads "una", not "o".
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        self.0.merge_with(l, r, &self.0.numwords_inflections_una)
    }

    /// Re-entered by `money_verbose`/`cents_verbose`; drives `merge` above.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        ro_to_cardinal(self, value)
    }

    // ---- everything else forwards to the real converter ------------------

    fn cards(&self) -> &Cards {
        self.0.cards()
    }
    fn maxval(&self) -> &BigInt {
        self.0.maxval()
    }
    fn negword(&self) -> &str {
        self.0.negword()
    }
    fn pointword(&self) -> &str {
        self.0.pointword()
    }
    fn is_title(&self) -> bool {
        self.0.is_title()
    }
    fn exclude_title(&self) -> &[String] {
        self.0.exclude_title()
    }
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.0.to_ordinal(value)
    }
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.0.to_ordinal_num(value)
    }
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.0.to_year(value)
    }
    fn lang_name(&self) -> &str {
        self.0.lang_name()
    }
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.0.currency_forms(code)
    }
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.0.currency_adjective(code)
    }
    fn currency_precision(&self, code: &str) -> i64 {
        self.0.currency_precision(code)
    }
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        self.0.pluralize(n, forms)
    }
    fn default_separator(&self) -> &str {
        self.0.default_separator()
    }
    fn default_currency(&self) -> &str {
        self.0.default_currency()
    }
    // `to_currency` is deliberately NOT forwarded: `LangRo::to_currency` is
    // what constructs this view, and forwarding would recurse. Nothing calls
    // it on a `RoUna` — `LangRo::to_currency` invokes `default_to_currency`
    // directly, which is `super().to_currency` in Python. `to_cheque` is not
    // forwarded either: RO's `to_cheque` never sets the flag, so it must not
    // be reachable through this view.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::CurrencyValue;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// `arg` is `repr(value)` on the Python side, so `is_int` / `has_decimal`
    /// are derived the same way the binding derives them: a true `int` has no
    /// dot, and `isinstance(val, float)` is true for everything that does.
    fn cur(arg: &str, code: &str) -> Result<String> {
        let is_int = !arg.contains('.');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangRo::new().to_currency(&v, code, true, None, false)
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangRo::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    fn is_not_impl(r: Result<String>) -> bool {
        matches!(r, Err(N2WError::NotImplemented(_)))
    }

    /// Every `currency:EUR` and `currency:USD` row of the frozen corpus.
    #[test]
    fn corpus_currency_eur_usd() {
        let rows: &[(&str, &str, &str)] = &[
            ("0", "EUR", "zero euro"),
            ("1", "EUR", "unu euro"),
            ("2", "EUR", "doi euro"),
            ("100", "EUR", "una sută de euro"),
            ("12.34", "EUR", "doisprezece euro și treizeci și patru de cenţi"),
            ("0.01", "EUR", "zero euro și unu cent"),
            ("1.0", "EUR", "unu euro și zero cenţi"),
            ("99.99", "EUR", "nouăzeci și nouă de euro și nouăzeci și nouă de cenţi"),
            (
                "1234.56",
                "EUR",
                "una mie două sute treizeci și patru de euro și cincizeci și șase de cenţi",
            ),
            ("-12.34", "EUR", "minus doisprezece euro și treizeci și patru de cenţi"),
            ("1000000", "EUR", "un milion de euro"),
            ("0.5", "EUR", "zero euro și cincizeci de cenţi"),
            ("0", "USD", "zero dolari"),
            ("1", "USD", "unu dolar"),
            ("2", "USD", "doi dolari"),
            ("100", "USD", "una sută de dolari"),
            ("12.34", "USD", "doisprezece dolari și treizeci și patru de cenţi"),
            ("0.01", "USD", "zero dolari și unu cent"),
            ("1.0", "USD", "unu dolar și zero cenţi"),
            ("99.99", "USD", "nouăzeci și nouă de dolari și nouăzeci și nouă de cenţi"),
            (
                "1234.56",
                "USD",
                "una mie două sute treizeci și patru de dolari și cincizeci și șase de cenţi",
            ),
            ("-12.34", "USD", "minus doisprezece dolari și treizeci și patru de cenţi"),
            ("1000000", "USD", "un milion de dolari"),
            ("0.5", "USD", "zero dolari și cincizeci de cenţi"),
        ];
        for (arg, code, want) in rows {
            assert_eq!(cur(arg, code).unwrap(), *want, "currency:{} {}", code, arg);
        }
    }

    /// Both successful `cheque:*` rows. Note "O MIE", not "UNA MIE": `to_cheque`
    /// never sets the "una" flag, so it renders through the pristine table.
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "O MIE DOUĂ SUTE TREIZECI ȘI PATRU AND 56/100 DE EURO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "O MIE DOUĂ SUTE TREIZECI ȘI PATRU AND 56/100 DE DOLARI"
        );
    }

    /// RO declares its own `CURRENCY_FORMS`, so it never sees the codes
    /// `Num2Word_EN.__init__` adds to the dict `Num2Word_EUR` shares. All 84
    /// currency rows and 7 cheque rows for these codes expect NotImplementedError.
    #[test]
    fn corpus_unimplemented_codes() {
        let args = [
            "0", "1", "2", "100", "12.34", "0.01", "1.0", "99.99", "1234.56", "-12.34",
            "1000000", "0.5",
        ];
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            for arg in args {
                assert!(is_not_impl(cur(arg, code)), "currency:{} {}", code, arg);
            }
            assert!(is_not_impl(cheque("1234.56", code)), "cheque:{}", code);
        }
    }

    /// The exact Python message, from the live interpreter.
    #[test]
    fn not_implemented_message() {
        match cur("1", "GBP") {
            Err(N2WError::NotImplemented(m)) => {
                assert_eq!(m, "Currency code \"GBP\" not implemented for \"Num2Word_RO\"")
            }
            other => panic!("want NotImplemented, got {:?}", other),
        }
        match cheque("1234.56", "GBP") {
            Err(N2WError::NotImplemented(m)) => {
                assert_eq!(m, "Currency code \"GBP\" not implemented for \"Num2Word_RO\"")
            }
            other => panic!("want NotImplemented, got {:?}", other),
        }
    }

    /// RON is RO's `default_currency` but no corpus row exercises it, so these
    /// are pinned against the live interpreter instead. They are the only rows
    /// that reach the three `str.replace`s in `to_currency`.
    #[test]
    fn ron_replaces() {
        let rows: &[(&str, &str)] = &[
            ("0", "zero lei"),
            ("1", "un leu"),                 // "unu leu" -> "un leu"
            ("2", "doi lei"),
            ("100", "una sută de lei"),
            ("12.34", "doisprezece lei și treizeci și patru de bani"),
            ("0.01", "zero lei și un ban"),  // "unu ban" -> "un ban"
            ("1.0", "un leu"),               // both replaces + the zero-bani strip
            ("0.0", "zero lei"),             // zero-bani strip alone
            ("2.0", "doi lei"),
            ("1.01", "un leu și un ban"),    // order: leu first, then ban
            ("21.0", "douăzeci și unu de lei"),
            ("-1.0", "minus un leu"),
            ("99.99", "nouăzeci și nouă de lei și nouăzeci și nouă de bani"),
            (
                "1234.56",
                "una mie două sute treizeci și patru de lei și cincizeci și șase de bani",
            ),
            ("-12.34", "minus doisprezece lei și treizeci și patru de bani"),
            ("1000000", "un milion de lei"),
            ("0.5", "zero lei și cincizeci de bani"),
        ];
        for (arg, want) in rows {
            assert_eq!(cur(arg, "RON").unwrap(), *want, "currency:RON {}", arg);
        }
    }

    /// The fractional-cents branch, which `default_to_currency` renders via
    /// `cardinal_from_decimal` and which takes `cr2[1]` unconditionally.
    ///
    /// `1.011 RON` is the sharpest case: "unu leu și unu virgulă unu bani" has
    /// *both* replaces fire, the second one mid-word ("unu ban" is a prefix of
    /// "unu bani"), yielding the ungrammatical "un bani". All pinned against
    /// the live interpreter.
    #[test]
    fn fractional_cents() {
        let rows: &[(&str, &str, &str)] = &[
            ("1.011", "RON", "un leu și unu virgulă un bani"),
            ("0.005", "RON", "zero lei și zero virgulă cinci bani"),
            ("1.005", "RON", "un leu și zero virgulă cinci bani"),
            ("2.675", "RON", "doi lei și șaizeci și șapte virgulă cinci bani"),
            ("0.005", "EUR", "zero euro și zero virgulă cinci cenţi"),
            ("2.675", "EUR", "doi euro și șaizeci și șapte virgulă cinci cenţi"),
            ("1.005", "USD", "unu dolar și zero virgulă cinci cenţi"),
        ];
        for (arg, code, want) in rows {
            assert_eq!(cur(arg, code).unwrap(), *want, "{} {}", code, arg);
        }
    }

    /// The zero-cents strip is keyed on the *caller's* separator, not on RO's
    /// default — so an override still collapses `1.0 RON` to "un leu".
    #[test]
    fn separator_is_used_by_the_strip() {
        let l = LangRo::new();
        let v = CurrencyValue::parse("1.0", false, true, true).unwrap();
        assert_eq!(l.to_currency(&v, "RON", true, Some(" plus"), false).unwrap(), "un leu");
        assert_eq!(l.to_currency(&v, "RON", true, Some(","), false).unwrap(), "un leu");
        // Omitted separator= resolves through default_separator() == " și".
        assert_eq!(l.to_currency(&v, "EUR", true, None, false).unwrap(), "unu euro și zero cenţi");
    }

    /// `cents=False` is accepted and discarded: the super call hardcodes True,
    /// so the cents stay verbose instead of falling back to `_cents_terse`.
    #[test]
    fn cents_flag_is_ignored() {
        let l = LangRo::new();
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        assert_eq!(
            l.to_currency(&v, "EUR", false, None, false).unwrap(),
            "doisprezece euro și treizeci și patru de cenţi"
        );
    }

    /// `CURRENCY_ADJECTIVES` is inherited from `Num2Word_EUR`. Note the
    /// replaces are blind to the prefix: "unu Romanian leu" stays "unu".
    #[test]
    fn adjective_prefix() {
        let l = LangRo::new();
        let one = CurrencyValue::parse("1", true, false, false).unwrap();
        assert_eq!(l.to_currency(&one, "RON", true, None, true).unwrap(), "unu Romanian leu");
        let one_f = CurrencyValue::parse("1.0", false, true, true).unwrap();
        assert_eq!(l.to_currency(&one_f, "RON", true, None, true).unwrap(), "unu Romanian leu");
        let two = CurrencyValue::parse("2", true, false, false).unwrap();
        assert_eq!(l.to_currency(&two, "USD", true, None, true).unwrap(), "doi US dolari");
        // EUR has no adjective; the flag is a no-op.
        assert_eq!(l.to_currency(&one, "EUR", true, None, true).unwrap(), "unu euro");
    }

    /// The "una" flip must stay confined to `to_currency`: the four integer
    /// modes keep the pristine "o", which is what their corpus rows say.
    #[test]
    fn una_flip_does_not_leak() {
        let l = LangRo::new();
        let hundred = BigInt::from(100);
        assert_eq!(l.to_cardinal(&hundred).unwrap(), "o sută");
        assert_eq!(l.to_cardinal(&BigInt::from(1000)).unwrap(), "o mie");
        assert_eq!(l.to_cardinal(&BigInt::from(1234)).unwrap(), "o mie două sute treizeci și patru");
        assert_eq!(l.to_ordinal(&hundred).unwrap(), "al o sutălea");

        // ... including after a currency call that succeeds ...
        let v = CurrencyValue::parse("100", true, false, false).unwrap();
        assert_eq!(l.to_currency(&v, "EUR", true, None, false).unwrap(), "una sută de euro");
        assert_eq!(l.to_cardinal(&hundred).unwrap(), "o sută");

        // ... and after one that raises, where Python would stay poisoned.
        assert!(is_not_impl(l.to_currency(&v, "GBP", true, None, false)));
        assert_eq!(l.to_cardinal(&hundred).unwrap(), "o sută");
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "O MIE DOUĂ SUTE TREIZECI ȘI PATRU AND 56/100 DE EURO"
        );
    }

    /// `gen_numwords_n` is a distinct list in Python; the flip never reaches it.
    #[test]
    fn mega_giga_keep_un() {
        let l = LangRo::new();
        for (arg, want) in [("1000000", "un milion de euro"), ("1000000000", "un miliard de euro")] {
            let v = CurrencyValue::parse(arg, true, false, false).unwrap();
            assert_eq!(l.to_currency(&v, "EUR", true, None, false).unwrap(), want);
        }
    }

    /// RO's three-way plural rule, exercised through the unit word.
    ///
    /// Note 101: form 0 is gated on `n == 1`, not on `n % 100 == 1`, so 101
    /// falls through to form 1 and takes the *plural* — "una sută unu dolari",
    /// not "…unu dolar". Confirmed against the live interpreter.
    #[test]
    fn plural_rule() {
        for (arg, want) in [
            ("1", "unu dolar"),            // form 0 (n == 1)
            ("0", "zero dolari"),          // form 1 (n == 0)
            ("19", "nouăsprezece dolari"), // form 1 (0 < n % 100 < 20)
            ("101", "una sută unu dolari"), // form 1 (n % 100 == 1, but n != 1)
            ("20", "douăzeci de dolari"),  // form 2
            ("100", "una sută de dolari"), // form 2 (n % 100 == 0)
        ] {
            let v = CurrencyValue::parse(arg, true, false, false).unwrap();
            let got = LangRo::new().to_currency(&v, "USD", true, None, false).unwrap();
            assert_eq!(got, want, "{}", arg);
        }
    }
}
