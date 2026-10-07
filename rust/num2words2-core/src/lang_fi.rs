//! Port of `lang_FI.py` (Finnish), via `lang_EUR` → `Num2Word_Base`.
//!
//! Shape: **self-contained**, but only just. `Num2Word_FI` *is* an engine
//! language in spirit — it defines `high/mid/low_numwords` and a `merge` — yet
//! it overrides `splitnum`, `clean`, `merge` and `to_cardinal` with its own
//! four-argument variants that thread an `Options` (ordinal/case/plural/prefer)
//! through every step. Two things make the shared `base.rs` engine unusable:
//!
//!   1. FI's card *values* are not strings. They are Finnish **stems** paired
//!      with a Kotus inflection class — `("tuha", 46)` — or lists of such parts
//!      mixed with literal text — `[("y", 31), "toista"]`. The final surface
//!      form is only produced by [`inflect`], at merge time, once the required
//!      grammatical case is known. `base::Cards` maps `BigInt -> String`, so it
//!      cannot carry them.
//!   2. FI keeps a **second** table, `self.ords`, parallel to `self.cards`.
//!      `splitnum` picks one or the other from `options.ordinal`, so ordinals
//!      are produced by re-running the whole engine over a different table
//!      rather than by post-processing the cardinal.
//!
//! So `cards()`/`maxval()`/`merge()` stay at their trait defaults here and this
//! file carries private ports of `splitnum`/`clean`/`merge`/`inflect` plus its
//! own [`Table`] type. The tree shape and the fold order are otherwise
//! identical to `base.rs` — see the comments there.
//!
//! # Inherited behaviour
//!
//!   * `Num2Word_EUR.setup` supplies `high_numwords` = `["cent"] +
//!     gen_high_numwords(units, tens, lows)` (100 Latin stems). EUR's
//!     `set_high_numwords` is **not** used — FI overrides it (see
//!     [`translate_high`]).
//!   * `Num2Word_Base.verify_ordinal` raises `TypeError` on negatives; FI's
//!     `to_ordinal` calls it, hence `to_ordinal(-1)` → `N2WError::Type`.
//!   * `Num2Word_Base.title` is a no-op because `is_title` is never set true.
//!   * `MAXVAL = 1000 * list(self.cards.keys())[0]` = 1000 · 10^600 = **10^603**.
//!   * The **float/Decimal cardinal path is inherited unchanged.** FI overrides
//!     only `to_cardinal` (the integer engine, above); it defines neither
//!     `to_cardinal_float` nor `float2tuple`, so `Num2Word_Base.to_cardinal_float`
//!     applies verbatim — which is exactly what the trait default
//!     [`Lang::to_cardinal_float`] → `floatpath::default_to_cardinal_float`
//!     ports. That default calls back into this file's `to_cardinal` for the
//!     integer part and each fractional digit, and reads `negword` ("miinus ")
//!     and `pointword` ("pilkku") from the impl below, so `0.5` →
//!     "nolla pilkku viisi", `-0.5` → "miinus nolla pilkku viisi", `2.675` →
//!     "kaksi pilkku kuusi seitsemän viisi" (the f64-artefact `.675` rescued by
//!     the `< 0.01` heuristic), and the trillion-scale Decimal
//!     `98746251323029.99` round-trips exactly. Since `is_title` is false,
//!     `title(pointword)` leaves "pilkku" untouched. Verified against the frozen
//!     corpus (`cardinal_dec` + float `cardinal` rows) and the live interpreter;
//!     no override is needed here and adding one would only risk divergence.
//!
//! # Faithfully reproduced Python bugs
//!
//! `Num2Word_FI.set_high_numwords` rewrites the Latin stems into Finnish. Its
//! `octo`/`nove` arms mean to reach the "two from twenty" / "one from twenty"
//! forms by looking 10 entries back in the *decade* order, but they are wrong
//! in two ways, and both are preserved verbatim by [`translate_high`]:
//!
//! 1. The arms do `numword = high[i + -10]` and then slice `"octo"`/`"nove"`
//!    off **that** word — which generally does not start with `octo`/`nove` at
//!    all, so the slice removes 4 (resp. 5) arbitrary characters. Hence
//!    10^480 is "duodegintiljoona" (from `high[10]` = "nonagint" → "duode" +
//!    "gint"), 10^474 is "undetogintiljoona" (from "novoctogint" → "unde" +
//!    "togint") and 10^468 is "duodektogintiljoona".
//! 2. For `i < 10` Python's `high[i - 10]` wraps around to the **end** of the
//!    list — the short `lows` stems. So 10^594 = "undeiljoona" (`high[-9]` =
//!    "non", sliced to "") and 10^588 = "duodeiljoona" (`high[-8]` = "oct",
//!    sliced to ""), both missing their scale name entirely. [`wrap_index`]
//!    reproduces Python's negative indexing.
//!
//! Two more oddities that are *not* bugs but look like them, kept as-is:
//! `"sexagint"` → `"seagint"` → **"seagintiljoona"** (10^360), because the
//! blanket `replace("sex", "se")` also eats the `sex` of the decade name; and
//! `"septrigint"` → **"septenrigintiljoona"** (10^222), because the `sept` arm
//! re-prefixes a stem whose `sept` was already elided by EUR.
//!
//! Every high stem in this file is generated, not transcribed, and the
//! generator was diffed against the live Python `cards`/`ords` tables for all
//! 101 high entries.
//!
//! # Currency
//!
//! `Num2Word_FI` declares its **own** class-body `CURRENCY_FORMS`, which
//! shadows `Num2Word_EUR.CURRENCY_FORMS`. So the `lang_EUR` trap — where
//! `Num2Word_EN.__init__` mutates the shared EUR class dict in place and 16
//! other classes silently read the English result — **does not reach FI**:
//! `Num2Word_FI.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` is `False`, and
//! the live table holds `("euro", "euroa")`, not EN's `("euro", "euros")`.
//! This file therefore transcribes `lang_FI.py` literally, loops included.
//!
//! FI defines no `CURRENCY_PRECISION`, so `Num2Word_Base`'s empty dict applies
//! and `.get(code, 100)` yields **100 for every code** — the trait default.
//! Deliberately not overridden: JPY is a *2-decimal* currency here (its rare
//! `sen` subunit is kept), so `default_to_currency`'s `divisor == 1` branch is
//! unreachable, and KWD/BHD are simply absent from the table rather than being
//! 3-decimal — they raise NotImplementedError.
//!
//! `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
//! inherited from `Num2Word_Base` unchanged; `pluralize` from `Num2Word_EUR`.
//! Only `to_currency` is overridden — see [`LangFi::to_currency`].
//!
//! # Float/Decimal entry routing
//!
//! `to_cardinal` routes on `assert int(value) == value` — whole values take
//! the integer engine, fractional ones `to_cardinal_float` — which is exactly
//! the trait-default `cardinal_float_entry`, so that hook stays untouched. The
//! other three modes need overrides:
//!
//!   * `to_ordinal(float/Decimal)` runs `Num2Word_Base.verify_ordinal` first:
//!     fractional → `TypeError` (`errmsg_floatord`), negative → `TypeError`
//!     (`errmsg_negord`). `abs(-0.0) == -0.0` is true in Python, so **-0.0 is
//!     a valid ordinal input** ("nollas") — the override must not use the
//!     sign-bit-aware `FloatValue::is_negative`. Whole non-negative floats run
//!     the ordinal engine (`to_ordinal(5.0)` == "viides").
//!   * `to_ordinal_num(float/Decimal)` is `str(value) + "."` with no
//!     validation at all, so `to_ordinal_num(-1.5)` == "-1.5." — the override
//!     appends "." to the Python repr the binding computed.
//!   * `to_year(float/Decimal)`: `val < 0` (false for -0.0!) flips to abs +
//!     " ennen ajanlaskun alkua", then `to_cardinal(val).replace(" ", "")` —
//!     so fractional years glue the decimal grammar together:
//!     `to_year(0.5)` == "nollapilkkuviisi".
//!
//! String inputs need no hook: FI does not override `str_to_number`, so the
//! default `Decimal(str)` parse feeds the same float entries.
//!
//! # Grammatical kwargs
//!
//! `to_cardinal`/`to_ordinal` accept `(case="nominative", plural=False,
//! prefer=None)`, `to_ordinal_num` `(case, plural)` (both **unread** — no
//! validation, still `str(value) + "."`), `to_year` `(suffix=None,
//! longval=True)` (`longval` unread). Quirks reproduced by the `*_kw` hooks:
//!
//!   * `case` goes through `NAME_TO_CASE[case]` — an unknown name (or a
//!     non-string like `None`) raises **KeyError**, and in `to_ordinal` that
//!     lookup happens *before* `verify_ordinal`, so
//!     `to_ordinal(-5, case="bogus")` is KeyError, not TypeError.
//!   * `case="accusative"` maps to ACC=12, which **no** KOTUS table defines,
//!     so it dies later with `KeyError: 12` inside `inflect`.
//!   * `to_cardinal(<fractional float>, case=<non-nominative>)` raises
//!     `NotImplementedError`; with nominative it falls to `to_cardinal_float`,
//!     silently ignoring `plural`/`prefer`.
//!   * `prefer=None` behaves as empty (`options.prefer or set()`).
//!   * `to_year(-n, suffix=s)` keeps the caller's suffix — the BC default only
//!     fills in when `suffix` is falsy (`suffix = suffix or ...` twice).
//!
//! # Cross-call state
//!
//! None. `Num2Word_FI` has no `_pending_*` handshake; `self.ords` is built once
//! in `__init__` and only read afterwards.

use crate::base::{Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Grammatical cases (module-level constants in lang_FI.py)
// ---------------------------------------------------------------------------

const NOM: u8 = 10; // nominative: the dictionary form
const GEN: u8 = 11; // genitive: ~of/'s
/// accusative: Python's `NAME_TO_CASE` maps "accusative" to 12, but **no**
/// `KOTUS_TYPE` entry defines it, so `to_cardinal(n, case="accusative")` dies
/// with `KeyError: 12` inside `inflect` — see [`kotus_suffixes`], which
/// returns that `KeyError`. Reachable only through the `*_kw` hooks.
const ACC: u8 = 12;
const PTV: u8 = 13; // partitive: as an object
const INE: u8 = 14; // inessive: in
const ELA: u8 = 15; // elative: from/out of
const ILL: u8 = 16; // illative: into
const ADE: u8 = 17; // adessive: at/on
const ABL: u8 = 18; // ablative: from
const ALL: u8 = 19; // allative: to
const ESS: u8 = 20; // essive: as (in the role of)
const TRANSL: u8 = 21; // translative: to (the role of)
const INSTRUC: u8 = 22; // instructive: with
const ABE: u8 = 23; // abessive: without
const COM: u8 = 24; // comitative: together with

/// `NAME_TO_CASE[case]`. A miss is Python's `KeyError: 'bogus'` — the repr'd
/// key is the whole message.
fn name_to_case(name: &str) -> Result<u8> {
    Ok(match name {
        "nominative" => NOM,
        "genitive" => GEN,
        "accusative" => ACC,
        "partitive" => PTV,
        "inessive" => INE,
        "elative" => ELA,
        "illative" => ILL,
        "adessive" => ADE,
        "ablative" => ABL,
        "allative" => ALL,
        "essive" => ESS,
        "translative" => TRANSL,
        "instructive" => INSTRUC,
        "abessive" => ABE,
        "comitative" => COM,
        other => return Err(N2WError::Key(format!("'{}'", other))),
    })
}

/// `BACK_TO_FRONT`: vowel harmony. Insertion order matters only in that the
/// replacements are applied in sequence; `ä`/`ö`/`y` are never re-matched, so
/// they cannot cascade.
const BACK_TO_FRONT: [(char, &str); 3] = [('a', "ä"), ('o', "ö"), ('u', "y")];

// ---------------------------------------------------------------------------
// KOTUS_TYPE — nominal inflection tables
// ---------------------------------------------------------------------------

/// One `CASE: (SINGULAR_SUFFIX+, PLURAL_SUFFIX+)` row.
///
/// Python stores either a plain `str` or a `tuple` of alternatives per slot.
/// Both are modelled as a slice: length 1 is the plain string, length > 1 is
/// the tuple of choices resolved against `options.prefer` in [`inflect`].
type Row = (u8, &'static [&'static str], &'static [&'static str]);

/// Kotus type 5/risti, no gradation.
const K5: &[Row] = &[
    (NOM, &["i"], &["it"]),
    (GEN, &["in"], &["ien"]),
    (PTV, &["ia"], &["eja"]),
    (INE, &["issa"], &["eissa"]),
    (ELA, &["ista"], &["eista"]),
    (ILL, &["iin"], &["eihin"]),
    (ADE, &["illa"], &["eilla"]),
    (ABL, &["ilta"], &["eilta"]),
    (ALL, &["ille"], &["eille"]),
    (ESS, &["ina"], &["eina"]),
    (TRANSL, &["iksi"], &["eiksi"]),
    (INSTRUC, &["ein"], &["ein"]),
    (ABE, &["itta"], &["eitta"]),
    (COM, &["eine"], &["eine"]),
];

/// Kotus type 7/ovi, no gradation. Only reachable as the plural half of the
/// derived type 108.
const K7: &[Row] = &[
    (NOM, &["i"], &["et"]),
    (GEN, &["en"], &["ien"]),
    (PTV, &["ea"], &["ia"]),
    (INE, &["essa"], &["issa"]),
    (ELA, &["esta"], &["ista"]),
    (ILL, &["een"], &["iin"]),
    (ADE, &["ella"], &["illa"]),
    (ABL, &["elta"], &["ilta"]),
    (ALL, &["elle"], &["ille"]),
    (ESS, &["ena"], &["ina"]),
    (TRANSL, &["eksi"], &["iksi"]),
    (INSTRUC, &["in"], &["in"]),
    (ABE, &["etta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// Kotus type 8/nalle, no gradation. Only reachable as the singular half of
/// the derived type 108.
const K8: &[Row] = &[
    (NOM, &["e"], &["et"]),
    (GEN, &["en"], &["ejen", "ein"]),
    (PTV, &["ea"], &["eja"]),
    (INE, &["essa"], &["eissa"]),
    (ELA, &["esta"], &["eista"]),
    (ILL, &["een"], &["eihin"]),
    (ADE, &["ella"], &["eilla"]),
    (ABL, &["elta"], &["eilta"]),
    (ALL, &["elle"], &["eille"]),
    (ESS, &["ena"], &["eina"]),
    (TRANSL, &["eksi"], &["eiksi"]),
    (INSTRUC, &["ein"], &["ein"]),
    (ABE, &["etta"], &["eitta"]),
    (COM, &["eine"], &["eine"]),
];

/// Kotus type 9/kala, t-d gradation (sata). Keyed 109 in Python.
const K109: &[Row] = &[
    (NOM, &["ta"], &["dat"]),
    (GEN, &["dan"], &["tojen", "tain"]),
    (PTV, &["taa"], &["toja"]),
    (INE, &["dassa"], &["doissa"]),
    (ELA, &["dasta"], &["doista"]),
    (ILL, &["taan"], &["toihin"]),
    (ADE, &["dalla"], &["doilla"]),
    (ABL, &["dalta"], &["doilta"]),
    (ALL, &["dalle"], &["doille"]),
    (ESS, &["tana"], &["toina"]),
    (TRANSL, &["daksi"], &["doiksi"]),
    (INSTRUC, &["doin"], &["doin"]),
    (ABE, &["datta"], &["doitta"]),
    (COM, &["toine"], &["toine"]),
];

/// Kotus type 10/koira, no gradation.
const K10: &[Row] = &[
    (NOM, &["a"], &["at"]),
    (GEN, &["an"], &["ien", "ain"]),
    (PTV, &["aa"], &["ia"]),
    (INE, &["assa"], &["issa"]),
    (ELA, &["asta"], &["ista"]),
    (ILL, &["aan"], &["iin"]),
    (ADE, &["alla"], &["illa"]),
    (ABL, &["alta"], &["ilta"]),
    (ALL, &["alle"], &["ille"]),
    (ESS, &["ana"], &["ina"]),
    (TRANSL, &["aksi"], &["iksi"]),
    (INSTRUC, &["in"], &["in"]),
    (ABE, &["atta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// Kotus type 27/käsi, t-d gradation.
const K27: &[Row] = &[
    (NOM, &["si"], &["det"]),
    (GEN, &["den"], &["sien", "tten"]),
    (PTV, &["tta"], &["sia"]),
    (INE, &["dessa"], &["sissa"]),
    (ELA, &["desta"], &["sista"]),
    (ILL, &["teen"], &["siin"]),
    (ADE, &["della"], &["silla"]),
    (ABL, &["delta"], &["silta"]),
    (ALL, &["delle"], &["sille"]),
    (ESS, &["tena"], &["sina"]),
    (TRANSL, &["deksi"], &["siksi"]),
    (INSTRUC, &["sin"], &["sin"]),
    (ABE, &["detta"], &["sitta"]),
    (COM, &["sine"], &["sine"]),
];

/// Kotus type 31/kaksi, t-d gradation.
const K31: &[Row] = &[
    (NOM, &["ksi"], &["hdet"]),
    (GEN, &["hden"], &["ksien"]),
    (PTV, &["hta"], &["ksia"]),
    (INE, &["hdessa"], &["ksissa"]),
    (ELA, &["hdesta"], &["ksista"]),
    (ILL, &["hteen"], &["ksiin"]),
    (ADE, &["hdella"], &["ksilla"]),
    (ABL, &["hdelta"], &["ksilta"]),
    (ALL, &["hdelle"], &["ksille"]),
    (ESS, &["htena"], &["ksina"]),
    (TRANSL, &["hdeksi"], &["ksiksi"]),
    (INSTRUC, &["ksin"], &["ksin"]),
    (ABE, &["hdetta"], &["ksitta"]),
    (COM, &["ksine"], &["ksine"]),
];

/// Kotus type 32/sisar, no gradation. Only reachable as the base of the
/// derived type 132.
const K32: &[Row] = &[
    (NOM, &[""], &["et"]),
    (GEN, &["en"], &["ien", "ten"]),
    (PTV, &["ta"], &["ia"]),
    (INE, &["essa"], &["issa"]),
    (ELA, &["esta"], &["ista"]),
    (ILL, &["een"], &["iin"]),
    (ADE, &["ella"], &["illa"]),
    (ABL, &["elta"], &["ilta"]),
    (ALL, &["elle"], &["ille"]),
    (ESS, &["ena"], &["ina"]),
    (TRANSL, &["eksi"], &["iksi"]),
    (INSTRUC, &["in"], &["in"]),
    (ABE, &["etta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// Kotus type 38/nainen, no gradation.
const K38: &[Row] = &[
    (NOM, &["nen"], &["set"]),
    (GEN, &["sen"], &["sten", "sien"]),
    (PTV, &["sta"], &["sia"]),
    (INE, &["sessa"], &["sissa"]),
    (ELA, &["sesta"], &["sista"]),
    (ILL, &["seen"], &["siin"]),
    (ADE, &["sella"], &["silla"]),
    (ABL, &["selta"], &["silta"]),
    (ALL, &["selle"], &["sille"]),
    (ESS, &["sena"], &["sina"]),
    (TRANSL, &["seksi"], &["siksi"]),
    (INSTRUC, &["sin"], &["sin"]),
    (ABE, &["setta"], &["sitta"]),
    (COM, &["sine"], &["sine"]),
];

/// Kotus type 45/kahdeksas, nt-nn gradation. Carries every ordinal stem.
const K45: &[Row] = &[
    (NOM, &["s"], &["nnet"]),
    (GEN, &["nnen"], &["nsien"]),
    (PTV, &["tta"], &["nsia"]),
    (INE, &["nnessa"], &["nsissa"]),
    (ELA, &["nnesta"], &["nsista"]),
    (ILL, &["nteen"], &["nsiin"]),
    (ADE, &["nnella"], &["nsilla"]),
    (ABL, &["nnelta"], &["nsilta"]),
    (ALL, &["nnelle"], &["nsille"]),
    (ESS, &["ntena"], &["nsina"]),
    (TRANSL, &["nneksi"], &["nsiksi"]),
    (INSTRUC, &["nsin"], &["nsin"]),
    (ABE, &["nnetta"], &["nsitta"]),
    (COM, &["nsine"], &["nsine"]),
];

/// Kotus type 46/tuhat, nt-nn gradation.
const K46: &[Row] = &[
    (NOM, &["t"], &["nnet"]),
    (GEN, &["nnen"], &["nsien", "nten"]),
    (PTV, &["tta"], &["nsia"]),
    (INE, &["nnessa"], &["nsissa"]),
    (ELA, &["nnesta"], &["nsista"]),
    (ILL, &["nteen"], &["nsiin"]),
    (ADE, &["nnella"], &["nsilla"]),
    (ABL, &["nnelta"], &["nsilta"]),
    (ALL, &["nnelle"], &["nsille"]),
    (ESS, &["ntena"], &["nsina"]),
    (TRANSL, &["nneksi"], &["nsiksi"]),
    (INSTRUC, &["nsin"], &["nsin"]),
    (ABE, &["nnetta"], &["nsitta"]),
    (COM, &["nsine"], &["nsine"]),
];

/// kolme. Python builds this at import time as
/// `{c: (KOTUS_TYPE[8][c][0], KOTUS_TYPE[7][c][1]) for c in KOTUS_TYPE[8]}` —
/// singular column from type 8, plural column from type 7 — then overrides
/// INSTRUC/ABE/COM. Written out literally here and verified cell-by-cell
/// against the live `lang_FI.KOTUS_TYPE[108]`.
///
/// Note the GEN plural is type 7's plain `"ien"`, **not** type 8's
/// `("ejen", "ein")` tuple, and that ACC (12) is absent because type 8 has no
/// ACC key for the comprehension to iterate.
const K108: &[Row] = &[
    (NOM, &["e"], &["et"]),
    (GEN, &["en"], &["ien"]),
    (PTV, &["ea"], &["ia"]),
    (INE, &["essa"], &["issa"]),
    (ELA, &["esta"], &["ista"]),
    (ILL, &["een"], &["iin"]),
    (ADE, &["ella"], &["illa"]),
    (ABL, &["elta"], &["ilta"]),
    (ALL, &["elle"], &["ille"]),
    (ESS, &["ena"], &["ina"]),
    (TRANSL, &["eksi"], &["iksi"]),
    // the three explicit overrides that follow the comprehension
    (INSTRUC, &["en"], &["in"]),
    (ABE, &["etta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// seitsemän, kahdeksan, yhdeksän. `KOTUS_TYPE[10].copy()` with NOM replaced
/// by `("an", "at")`.
const K110: &[Row] = &[
    (NOM, &["an"], &["at"]),
    (GEN, &["an"], &["ien", "ain"]),
    (PTV, &["aa"], &["ia"]),
    (INE, &["assa"], &["issa"]),
    (ELA, &["asta"], &["ista"]),
    (ILL, &["aan"], &["iin"]),
    (ADE, &["alla"], &["illa"]),
    (ABL, &["alta"], &["ilta"]),
    (ALL, &["alle"], &["ille"]),
    (ESS, &["ana"], &["ina"]),
    (TRANSL, &["aksi"], &["iksi"]),
    (INSTRUC, &["in"], &["in"]),
    (ABE, &["atta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// kymmenen. `KOTUS_TYPE[32].copy()` with NOM replaced by `("en", "et")`.
const K132: &[Row] = &[
    (NOM, &["en"], &["et"]),
    (GEN, &["en"], &["ien", "ten"]),
    (PTV, &["ta"], &["ia"]),
    (INE, &["essa"], &["issa"]),
    (ELA, &["esta"], &["ista"]),
    (ILL, &["een"], &["iin"]),
    (ADE, &["ella"], &["illa"]),
    (ABL, &["elta"], &["ilta"]),
    (ALL, &["elle"], &["ille"]),
    (ESS, &["ena"], &["ina"]),
    (TRANSL, &["eksi"], &["iksi"]),
    (INSTRUC, &["in"], &["in"]),
    (ABE, &["etta"], &["itta"]),
    (COM, &["ine"], &["ine"]),
];

/// `KOTUS_TYPE[kotus_type][case]`, reproducing both `KeyError`s.
fn kotus_suffixes(kotus: u16, case: u8) -> Result<(&'static [&'static str], &'static [&'static str])> {
    let rows: &'static [Row] = match kotus {
        5 => K5,
        7 => K7,
        8 => K8,
        10 => K10,
        27 => K27,
        31 => K31,
        32 => K32,
        38 => K38,
        45 => K45,
        46 => K46,
        108 => K108,
        109 => K109,
        110 => K110,
        132 => K132,
        _ => return Err(N2WError::Key(format!("{}", kotus))),
    };
    for &(c, sg, pl) in rows {
        if c == case {
            return Ok((sg, pl));
        }
    }
    // Only ACC (12) can land here, and only if a caller supplies
    // case="accusative" — which this trait cannot. Python: `KeyError: 12`.
    Err(N2WError::Key(format!("{}", case)))
}

// ---------------------------------------------------------------------------
// Card / ord values
// ---------------------------------------------------------------------------

/// One element of a card value.
///
/// Python mixes three things in the same list: a bare `str` ("toista"), a
/// 2-tuple `(stem, kotus_type)`, and a 3-tuple `(stem, kotus_type, case)` whose
/// third slot overrides the case for singular nominative only.
///
/// [`Part::Text`] additionally carries merge output: `merge` replaces a value
/// with the already-inflected surface string, and Python then re-feeds that
/// string to `inflect`, where the `not isinstance(part, tuple)` arm passes it
/// through untouched. So `inflect` is idempotent on merged text, and modelling
/// merged text as a `Text` part preserves that exactly.
#[derive(Debug, Clone)]
enum Part {
    Text(String),
    Stem {
        stem: String,
        kotus: u16,
        /// The 3-tuple's `part[2]`.
        case_override: Option<u8>,
    },
}

/// Python's card value: a single part or a list of parts. `inflect` normalises
/// the former with `if not isinstance(parts, list): parts = [parts]`, so one
/// uniform `Vec` covers both.
type Value = Vec<Part>;

fn text(s: impl Into<String>) -> Value {
    vec![Part::Text(s.into())]
}

fn stem(s: &str, kotus: u16) -> Part {
    Part::Stem { stem: s.to_string(), kotus, case_override: None }
}

/// The 3-tuple form, e.g. `("kymmen", 132, PTV)`.
fn stem_case(s: &str, kotus: u16, case: u8) -> Part {
    Part::Stem { stem: s.to_string(), kotus, case_override: Some(case) }
}

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

/// Port of `lang_FI.Options`.
///
/// `case` is always [`NOM`] and `plural` always false when driven through the
/// `Lang` trait, whose four modes take no grammatical arguments. `merge`'s
/// multiplication arm still flips `case` to [`PTV`] internally, so both are
/// live. `prefer` is always empty, which makes the tuple-of-alternatives
/// resolution in [`inflect`] always fall to the first choice — exactly what
/// Python does for `prefer=None`.
#[derive(Debug, Clone)]
struct Options {
    ordinal: bool,
    case: u8,
    plural: bool,
    prefer: Vec<String>,
}

impl Options {
    /// `Options.variation(case=...)`: everything else is inherited.
    fn with_case(&self, case: u8) -> Options {
        Options { case, ..self.clone() }
    }
}

// ---------------------------------------------------------------------------
// inflect
// ---------------------------------------------------------------------------

/// Port of `lang_FI.inflect`.
fn inflect(parts: &Value, options: &Options) -> Result<String> {
    let mut out = String::new();
    for part in parts {
        let (stem, kotus, case_override) = match part {
            // "part is plain text, concat and continue"
            Part::Text(t) => {
                out.push_str(t);
                continue;
            }
            Part::Stem { stem, kotus, case_override } => (stem, *kotus, *case_override),
        };

        // "predefined case (kaksikymmentä, ...)": the 3-tuple overrides the
        // singular nominative only.
        let mut tmp_case = options.case;
        if let Some(c) = case_override {
            if options.case == NOM && !options.plural {
                tmp_case = c;
            }
        }

        let (sg, pl) = kotus_suffixes(kotus, tmp_case)?;
        // Python indexes the (singular, plural) tuple with the bool directly.
        let choices = if options.plural { pl } else { sg };

        // "many choices, choose preferred or first": Python intersects the
        // choice set with `options.prefer` and takes the single common element
        // if and only if exactly one matches; otherwise choices[0].
        let mut suffix = choices[0].to_string();
        if choices.len() > 1 {
            let common: Vec<&str> = choices
                .iter()
                .copied()
                .filter(|c| options.prefer.iter().any(|p| p.as_str() == *c))
                .collect();
            if common.len() == 1 {
                suffix = common[0].to_string();
            }
        }

        // "apply vowel harmony" — gated on the *stem*'s characters, not the
        // suffix's: only a stem with no back vowel at all fronts its suffix.
        if !stem.chars().any(|ch| BACK_TO_FRONT.iter().any(|(b, _)| *b == ch)) {
            for (back, front) in BACK_TO_FRONT {
                suffix = suffix.replace(back, front);
            }
        }

        out.push_str(stem);
        out.push_str(&suffix);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Table — the OrderedDict behind self.cards / self.ords
// ---------------------------------------------------------------------------

/// `BigInt -> Value`, in descending key order.
///
/// Python uses an `OrderedDict` and `splitnum` walks it in **insertion** order:
/// high (10^600 … 10^12, 10^9, 10^6), then mid (1000, 100, 90 … 30), then low
/// (20 … 0). That happens to be strictly descending — verified against the live
/// table — so a descending `Vec` reproduces the iteration exactly and allows
/// the same binary-search shortcut `base::Cards` uses.
#[derive(Debug, Default)]
struct Table {
    entries: Vec<(BigInt, Value)>,
}

impl Table {
    fn new() -> Self {
        Table { entries: Vec::new() }
    }

    /// Append in Python's insertion order; callers must stay descending.
    fn push(&mut self, key: BigInt, value: Value) {
        self.entries.push((key, value));
    }

    fn get(&self, key: &BigInt) -> Option<&Value> {
        self.entries
            .binary_search_by(|(k, _)| key.cmp(k))
            .ok()
            .map(|i| &self.entries[i].1)
    }

    /// Entries from the first key `<= value`, descending — Python's
    /// `for elem in elems: if elem > value: continue`.
    fn iter_from(&self, value: &BigInt) -> impl Iterator<Item = &(BigInt, Value)> {
        let start = self.entries.partition_point(|(k, _)| k > value);
        self.entries[start..].iter()
    }

    fn highest(&self) -> Option<&BigInt> {
        self.entries.first().map(|(k, _)| k)
    }
}

// ---------------------------------------------------------------------------
// splitnum tree
// ---------------------------------------------------------------------------

/// Same distinction as `base::Node`: `Leaf` is Python's `(value, num)` tuple,
/// `List` a nested list. `clean` branches on it, so it must not be flattened.
#[derive(Debug, Clone)]
enum Node {
    Leaf(Value, BigInt),
    List(Vec<Node>),
}

impl Node {
    fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf(..))
    }
}

// ---------------------------------------------------------------------------
// High numword generation
// ---------------------------------------------------------------------------

/// Port of `Num2Word_EUR.gen_high_numwords` (duplicated rather than imported so
/// this file stays self-contained and does not depend on another language's
/// module).
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — tens is the outer loop.
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

/// Python's `s[n:]` — by character, and empty (never a panic) when `s` is
/// shorter than `n`.
fn char_slice_from(s: &str, n: usize) -> String {
    s.chars().skip(n).collect()
}

/// Python's `high[i + -10]`, including the negative-index wraparound that
/// bug (2) in the module docs depends on.
fn wrap_index(i: usize, len: usize) -> usize {
    let idx = i as isize - 10;
    (if idx < 0 { idx + len as isize } else { idx }) as usize
}

/// The stem-rewriting half of `Num2Word_FI.set_high_numwords`.
///
/// Reproduces the `octo`/`nove` look-back bugs verbatim; see the module docs.
/// Diffed against the live Python table for all 100 stems.
fn translate_high(high: &[String]) -> Vec<String> {
    const REPLACEMENTS: [(&str, &str); 4] = [
        ("qu", "kv"),
        ("x", "ks"),
        ("c", "k"),
        ("kent", "sent"), // applied after c -> k to cent
    ];

    let mut translated: Vec<String> = Vec::with_capacity(high.len());
    for (i, numword) in high.iter().enumerate() {
        let mut numword: String = numword.clone();

        // 1e6**6 is sekstiljoona but 1e6**16 is sedekiljoona.
        if numword.starts_with("sex") && numword != "sext" {
            // Blanket replace, not just the prefix: this is what turns
            // "sexagint" into "seagint".
            numword = numword.replace("sex", "se");
        }
        // 1e6**7 is septiljoona but 1e6**17 is septendekiljoona.
        else if numword.starts_with("sept") && numword != "sept" {
            numword = format!("septen{}", char_slice_from(&numword, "sept".len()));
        }
        // 1e6**8 is oktiljoona but 1e6**18 is duodevigintiljoona (2 from 20).
        else if numword.starts_with("octo") {
            // NB: indexes the *original* list, then slices "octo".len() off a
            // word that generally does not start with "octo".
            let w = &high[wrap_index(i, high.len())];
            numword = format!("duode{}", char_slice_from(w, "octo".len()));
        }
        // 1e6**9 is noniljoona but 1e6**19 is undevigintiljoona (1 from 20).
        else if numword.starts_with("nove") {
            let w = &high[wrap_index(i, high.len())];
            // len("nove") + 1 == 5 — one more than the prefix it claims to strip.
            numword = format!("unde{}", char_slice_from(w, "nove".len() + 1));
        }

        // apply general replacements to all numwords
        for (a, b) in REPLACEMENTS {
            numword = numword.replace(a, b);
        }
        translated.push(numword);
    }
    translated
}

// ---------------------------------------------------------------------------
// Currency tables
// ---------------------------------------------------------------------------

/// `Num2Word_FI.CURRENCY_FORMS`.
///
/// FI's own class-body dict — **not** the `Num2Word_EUR` one that
/// `Num2Word_EN.__init__` mutates in place, because declaring the attribute on
/// the subclass shadows it outright. See the module docs.
///
/// Built in the same three steps as the Python class body — the literal dict,
/// then the `crowns` / `dollars` / `pounds` `for curr_code in ...` loops that
/// write into it at class-definition time — so the entries line up one-for-one
/// with `lang_FI.py`. Verified against the live table: 24 codes, every one a
/// 2-form tuple on both sides.
///
/// (The loops also leave a stray `curr_code = "GBP"` class attribute behind,
/// Python's for-loop variable outliving its scope. It is never read, so it has
/// no counterpart here.)
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Module-level constants in lang_FI.py.
    const GENERIC_CENTS: [&str; 2] = ["sentti", "senttiä"];
    const GENERIC_CENTAVOS: [&str; 2] = ["centavo", "centavoa"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();

    // ---- the literal dict ----
    m.insert("BRL", CurrencyForms::new(&["real", "realia"], &GENERIC_CENTAVOS));
    m.insert("CHF", CurrencyForms::new(&["frangi", "frangia"], &["rappen", "rappenia"]));
    m.insert("CNY", CurrencyForms::new(&["juan", "juania"], &["fen", "feniä"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euroa"], &GENERIC_CENTS));
    // historical
    m.insert("FIM", CurrencyForms::new(&["markka", "markkaa"], &["penni", "penniä"]));
    m.insert("INR", CurrencyForms::new(&["rupia", "rupiaa"], &["paisa", "paisaa"]));
    // rare subunit
    m.insert("JPY", CurrencyForms::new(&["jeni", "jeniä"], &["sen", "seniä"]));
    // rare subunit
    m.insert("KRW", CurrencyForms::new(&["won", "wonia"], &["jeon", "jeonia"]));
    // rare subunit
    m.insert("KPW", CurrencyForms::new(&["won", "wonia"], &["chon", "chonia"]));
    m.insert("MXN", CurrencyForms::new(&["peso", "pesoa"], &GENERIC_CENTAVOS));
    m.insert("RUB", CurrencyForms::new(&["rupla", "ruplaa"], &["kopeekka", "kopeekkaa"]));
    m.insert("TRY", CurrencyForms::new(&["liira", "liiraa"], &["kuruş", "kuruşia"]));
    m.insert("ZAR", CurrencyForms::new(&["randi", "randia"], &GENERIC_CENTS));

    // ---- crowns ----
    for code in ["DKK", "ISK", "NOK", "SEK"] {
        m.insert(code, CurrencyForms::new(&["kruunu", "kruunua"], &["äyri", "äyriä"]));
    }
    // ---- dollars ----
    for code in ["AUD", "CAD", "HKD", "NZD", "SGD", "USD"] {
        m.insert(code, CurrencyForms::new(&["dollari", "dollaria"], &GENERIC_CENTS));
    }
    // ---- pounds ----
    // Python loops here too (`for curr_code in ("GBP",)`), but over a 1-tuple;
    // written as a plain insert since the loop carries no extra data.
    m.insert("GBP", CurrencyForms::new(&["punta", "puntaa"], &["penny", "pennyä"]));

    m
}

/// `Num2Word_FI.CURRENCY_ADJECTIVES`. Shadows EUR's dict, same as the forms.
///
/// Only ever consulted on the float path: FI's `to_currency` int branch never
/// calls `prefix_currency` — see [`LangFi::to_currency`].
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Australian"),
        ("BRL", "Brasilian"),
        ("CAD", "Kanadan"),
        ("CHF", "Sveitsin"),
        ("DKK", "Tanskan"),
        ("FIM", "Suomen"), // historical
        ("GBP", "Englannin"),
        ("HKD", "Hongkongin"),
        ("INR", "Intian"),
        ("ISK", "Islannin"),
        ("KRW", "Etelä-Korean"),
        ("KPW", "Pohjois-Korean"),
        ("MXN", "Meksikon"),
        ("NOK", "Norjan"),
        ("NZD", "Uuden-Seelannin"),
        ("RUB", "Venäjän"),
        ("SEK", "Ruotsin"),
        ("SGD", "Singaporen"),
        ("TRY", "Turkin"),
        ("USD", "Yhdysvaltain"),
        ("ZAR", "Etelä-Afrikan"),
    ]
    .into_iter()
    .collect()
}

// ---------------------------------------------------------------------------
// The language
// ---------------------------------------------------------------------------

pub struct LangFi {
    cards: Table,
    ords: Table,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangFi {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFi {
    pub fn new() -> Self {
        // ---- Num2Word_EUR.setup ----
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

        let mut cards = Table::new();
        let mut ords = Table::new();

        // ---- Num2Word_FI.set_high_numwords ----
        let translated = translate_high(&high);
        let ten = BigInt::from(10u8);
        // max = 6 * len(translated); zip(translated, range(max, 0, -6)).
        // Both sides are 100 long, so every stem is consumed and n ends at 6.
        let mut n: i64 = 6 * translated.len() as i64;
        for word in translated.iter() {
            if n <= 0 {
                break; // range(max, 0, -6) is exhausted (unreachable: lengths match)
            }
            if n == 6 {
                // "irregularity considering short scale and long scale" — 10^9
                // is inserted *before* 10^6, keeping the table descending.
                cards.push(ten.pow(9), vec![stem("miljard", 5)]);
                ords.push(ten.pow(9), vec![stem("miljardi", 45)]);
            }
            cards.push(ten.pow(n as u32), vec![stem(&format!("{}iljoon", word), 10)]);
            ords.push(ten.pow(n as u32), vec![stem(&format!("{}iljoona", word), 45)]);
            n -= 6;
        }

        // ---- Num2Word_FI.setup: mid_numwords / mid_ords ----
        cards.push(BigInt::from(1000), vec![stem("tuha", 46)]);
        cards.push(BigInt::from(100), vec![stem("sa", 109)]);
        cards.push(BigInt::from(90), vec![stem("yhdeks", 110), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(80), vec![stem("kahdeks", 110), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(70), vec![stem("seitsem", 110), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(60), vec![stem("kuu", 27), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(50), vec![stem("vii", 27), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(40), vec![stem("nelj", 10), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(30), vec![stem("kolm", 108), stem_case("kymmen", 132, PTV)]);

        ords.push(BigInt::from(1000), vec![stem("tuhanne", 45)]);
        ords.push(BigInt::from(100), vec![stem("sada", 45)]);
        ords.push(BigInt::from(90), vec![stem("yhdeksä", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(80), vec![stem("kahdeksa", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(70), vec![stem("seitsemä", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(60), vec![stem("kuude", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(50), vec![stem("viide", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(40), vec![stem("neljä", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(30), vec![stem("kolma", 45), stem("kymmene", 45)]);

        // ---- Num2Word_FI.setup: low_numwords / low_ords ----
        cards.push(BigInt::from(20), vec![stem("ka", 31), stem_case("kymmen", 132, PTV)]);
        cards.push(BigInt::from(19), vec![stem("yhdeks", 110), Part::Text("toista".into())]);
        cards.push(BigInt::from(18), vec![stem("kahdeks", 110), Part::Text("toista".into())]);
        cards.push(BigInt::from(17), vec![stem("seitsem", 110), Part::Text("toista".into())]);
        cards.push(BigInt::from(16), vec![stem("kuu", 27), Part::Text("toista".into())]);
        cards.push(BigInt::from(15), vec![stem("vii", 27), Part::Text("toista".into())]);
        cards.push(BigInt::from(14), vec![stem("nelj", 10), Part::Text("toista".into())]);
        cards.push(BigInt::from(13), vec![stem("kolm", 108), Part::Text("toista".into())]);
        cards.push(BigInt::from(12), vec![stem("ka", 31), Part::Text("toista".into())]);
        cards.push(BigInt::from(11), vec![stem("y", 31), Part::Text("toista".into())]);
        cards.push(BigInt::from(10), vec![stem("kymmen", 132)]);
        cards.push(BigInt::from(9), vec![stem("yhdeks", 110)]);
        cards.push(BigInt::from(8), vec![stem("kahdeks", 110)]);
        cards.push(BigInt::from(7), vec![stem("seitsem", 110)]);
        cards.push(BigInt::from(6), vec![stem("kuu", 27)]);
        cards.push(BigInt::from(5), vec![stem("vii", 27)]);
        cards.push(BigInt::from(4), vec![stem("nelj", 10)]);
        cards.push(BigInt::from(3), vec![stem("kolm", 108)]);
        cards.push(BigInt::from(2), vec![stem("ka", 31)]);
        cards.push(BigInt::from(1), vec![stem("y", 31)]);
        cards.push(BigInt::zero(), vec![stem("noll", 10)]);

        ords.push(BigInt::from(20), vec![stem("kahde", 45), stem("kymmene", 45)]);
        ords.push(BigInt::from(19), vec![stem("yhdeksä", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(18), vec![stem("kahdeksa", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(17), vec![stem("seitsemä", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(16), vec![stem("kuude", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(15), vec![stem("viide", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(14), vec![stem("neljä", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(13), vec![stem("kolma", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(12), vec![stem("kahde", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(11), vec![stem("yhde", 45), Part::Text("toista".into())]);
        ords.push(BigInt::from(10), vec![stem("kymmene", 45)]);
        ords.push(BigInt::from(9), vec![stem("yhdeksä", 45)]);
        ords.push(BigInt::from(8), vec![stem("kahdeksa", 45)]);
        ords.push(BigInt::from(7), vec![stem("seitsemä", 45)]);
        ords.push(BigInt::from(6), vec![stem("kuude", 45)]);
        ords.push(BigInt::from(5), vec![stem("viide", 45)]);
        ords.push(BigInt::from(4), vec![stem("neljä", 45)]);
        ords.push(BigInt::from(3), vec![stem("kolma", 45)]);
        ords.push(BigInt::from(2), vec![stem("toi", 38)]);
        ords.push(BigInt::from(1), vec![stem("ensimmäi", 38)]);
        ords.push(BigInt::zero(), vec![stem("nolla", 45)]);

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10^600 = 10^603.
        // Computed from `cards` only, but `to_ordinal` checks against it too.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangFi {
            cards,
            ords,
            maxval,
            exclude_title: vec!["pilkku".into(), "miinus".into()],
            // Built once here, never per call: `to_currency` only ever reads
            // these, and rebuilding them on each call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// Port of `Num2Word_FI.splitnum`.
    ///
    /// `None` is Python falling off the loop and implicitly returning `None`
    /// (value above every card) — impossible after the MAXVAL guard.
    #[allow(clippy::never_loop)] // mirrors Python's `for ...: return` (first card <= value)
    fn splitnum(&self, value: &BigInt, options: &Options) -> Result<Option<Vec<Node>>> {
        let elems = if options.ordinal { &self.ords } else { &self.cards };

        for (elem, word) in elems.iter_from(value) {
            let mut out: Vec<Node> = Vec::new();
            let (div, mod_) = if value.is_zero() {
                (BigInt::one(), BigInt::zero())
            } else {
                value.div_mod_floor(elem)
            };

            if div.is_one() {
                let one = BigInt::one();
                // elems[1]; present in both tables.
                let v = elems.get(&one).cloned().ok_or_else(|| N2WError::Key("1".into()))?;
                out.push(Node::Leaf(v, one));
            } else {
                if &div == value {
                    // "The system tallies, eg Roman Numerals": Python would do
                    // `div * elems[elem]`, repeating a *tuple/list* of stems —
                    // which `inflect` could not read back. Unreachable for FI:
                    // reaching here needs elem == 1 with value >= 2, but cards
                    // 0..=20 are all present, so the first card <= value is
                    // always >= 2 once value >= 2.
                    return Err(N2WError::Type(
                        "lang_FI splitnum tally branch is unreachable".into(),
                    ));
                }
                match self.splitnum(&div, options)? {
                    Some(sub) => out.push(Node::List(sub)),
                    None => return Ok(None),
                }
            }

            out.push(Node::Leaf(word.clone(), elem.clone()));

            if !mod_.is_zero() {
                match self.splitnum(&mod_, options)? {
                    Some(sub) => out.push(Node::List(sub)),
                    None => return Ok(None),
                }
            }

            return Ok(Some(out));
        }
        Ok(None)
    }

    /// Port of `Num2Word_FI.merge`.
    ///
    /// <http://www.kielitoimistonohjepankki.fi/ohje/49>
    ///
    /// `None` mirrors Python falling off the end of the if/elif chain when
    /// `lnum == rnum` (and `lnum != 1`) — there is no `else`, so it returns
    /// `None` and `to_cardinal`'s `words, num = self.clean(...)` then dies with
    /// `TypeError: cannot unpack non-sequence NoneType`. Unreachable with FI's
    /// card set: `div == elem` would require `elem**2 <= value < elem**2+elem`
    /// while `elem` is also the largest card `<= value`, which no gap in
    /// {0..20, 30..90 by 10, 100, 1000, 10^6, 10^9, 10^12 …} allows.
    ///
    /// Named `merge_parts`, not `merge`, purely to keep it clear of the
    /// `Lang::merge` trait method it would otherwise shadow — that one takes
    /// `&str` and is never reached for FI.
    fn merge_parts(
        &self,
        l: (&Value, &BigInt),
        r: (&Value, &BigInt),
        options: &Options,
    ) -> Result<Option<(Value, BigInt)>> {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;

        let one = BigInt::one();
        let two = BigInt::from(2);
        let thousand = BigInt::from(1000);

        // "ignore lpair if lnum is 1"
        if *lnum == one {
            let rt = inflect(rtext, options)?;
            return Ok(Some((text(rt), rnum.clone())));
        }
        // "rnum is added to lnum"
        if lnum > rnum {
            let lt = inflect(ltext, options)?;
            let rt = inflect(rtext, options)?;
            // "separate groups with space"
            let fmt = if *lnum >= thousand { " " } else { "" };
            return Ok(Some((text(format!("{}{}{}", lt, fmt, rt)), lnum + rnum)));
        }
        // "rnum is multiplied by lnum"
        if lnum < rnum {
            let (lval, rt): (Value, String) = if options.ordinal {
                // "kahdessadas, not toinensadas"
                let lval = if *lnum == two { vec![stem("kahde", 45)] } else { ltext.clone() };
                (lval, inflect(rtext, options)?)
            } else {
                // "kaksituhatta but kahdettuhannet"
                let mut rcase = options.case;
                if options.case == NOM && !options.plural {
                    rcase = PTV;
                }
                (ltext.clone(), inflect(rtext, &options.with_case(rcase))?)
            };
            let lt = inflect(&lval, options)?;
            // NB: the multiplication arm never takes the space `fmt`; only the
            // addition arm above rebinds it.
            return Ok(Some((text(format!("{}{}", lt, rt)), lnum * rnum)));
        }

        Ok(None)
    }

    /// Port of `Num2Word_FI.clean` (identical in structure to `base.clean`,
    /// including the quirk where the tail `val[2:]` is appended as a *nested
    /// list* rather than spliced).
    fn clean(&self, val: Vec<Node>, options: &Options) -> Result<Node> {
        let mut val = val;
        while val.len() != 1 {
            let mut out: Vec<Node> = Vec::new();
            if val.len() >= 2 && val[0].is_leaf() && val[1].is_leaf() {
                let (lt, ln) = match &val[0] {
                    Node::Leaf(t, n) => (t.clone(), n.clone()),
                    _ => unreachable!(),
                };
                let (rt, rn) = match &val[1] {
                    Node::Leaf(t, n) => (t.clone(), n.clone()),
                    _ => unreachable!(),
                };
                match self.merge_parts((&lt, &ln), (&rt, &rn), options)? {
                    Some((mt, mn)) => out.push(Node::Leaf(mt, mn)),
                    None => {
                        return Err(N2WError::Type(
                            "cannot unpack non-sequence NoneType object".into(),
                        ))
                    }
                }
                if val.len() > 2 {
                    out.push(Node::List(val[2..].to_vec()));
                }
            } else {
                for elem in val.into_iter() {
                    match elem {
                        Node::List(inner) => {
                            if inner.len() == 1 {
                                out.push(inner.into_iter().next().unwrap());
                            } else {
                                out.push(self.clean(inner, options)?);
                            }
                        }
                        leaf => out.push(leaf),
                    }
                }
            }
            val = out;
        }
        Ok(val.into_iter().next().unwrap())
    }

    /// Drives splitnum + clean for an already-non-negative, in-range value.
    fn convert(&self, value: &BigInt, options: &Options) -> Result<String> {
        let tree = self.splitnum(value, options)?.ok_or_else(|| {
            N2WError::Overflow(format!("abs({}) must be less than {}.", value, self.maxval))
        })?;
        match self.clean(tree, options)? {
            Node::Leaf(v, _) => inflect(&v, options),
            Node::List(_) => Err(N2WError::Type("clean did not reduce".into())),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`. The float arm cannot fire on a
    /// `BigInt`; the negative arm is the one the corpus exercises.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// The body of `Num2Word_FI.to_cardinal` past the `NAME_TO_CASE` lookup
    /// and the int/float routing, for an integral value with an arbitrary
    /// `Options`. `to_cardinal` and `to_cardinal_kw` both land here.
    fn to_cardinal_opts(&self, value: &BigInt, options: &Options) -> Result<String> {
        // `out = self.negword` — raw, see `to_cardinal`'s doc.
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out.push_str(self.negword());
        }

        if v >= self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v, self.maxval
            )));
        }

        let words = self.convert(&v, options)?;
        Ok(self.title(&format!("{}{}", out, words)))
    }

    /// The body of `Num2Word_FI.to_ordinal` past the `NAME_TO_CASE` lookup.
    /// Note Python resolves `case` *before* `verify_ordinal`, so callers must
    /// build `Options` (and surface its KeyError) first.
    fn to_ordinal_opts(&self, value: &BigInt, options: &Options) -> Result<String> {
        self.verify_ordinal(value)?;
        if value >= &self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                value, self.maxval
            )));
        }

        let words = self.convert(value, options)?;
        Ok(self.title(&words))
    }

    /// Builds `Options` from a kwargs bag, in Python's evaluation order:
    /// `case = NAME_TO_CASE[case]` first (KeyError on unknown or non-string
    /// keys), then `plural`/`prefer` verbatim.
    ///
    /// Values our port cannot model faithfully (e.g. `plural=2`, whose Python
    /// behaviour mixes tuple-indexing with truthiness) return NotImplemented
    /// so the dispatcher falls back to Python. The corpus only exercises
    /// bools, case names and string lists.
    fn options_from_kwargs(&self, kw: &Kwargs, ordinal: bool) -> Result<Options> {
        let case = match kw.get("case") {
            None => NOM,
            Some(KwVal::Str(s)) => name_to_case(s)?,
            // NAME_TO_CASE[None] / NAME_TO_CASE[5] / ... — a dict miss on the
            // non-string key. Only the exception type is oracle-checked.
            Some(other) => return Err(N2WError::Key(format!("{:?}", other))),
        };
        let plural = match kw.get("plural") {
            None => false,
            Some(KwVal::Bool(b)) => *b,
            // Python indexes `(sg, pl)[plural]`, so ints 0/1 behave as bools.
            Some(KwVal::Int(0)) => false,
            Some(KwVal::Int(1)) => true,
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        let prefer = match kw.get("prefer") {
            // `options.prefer or set()` — None is the empty preference set.
            None | Some(KwVal::None) => Vec::new(),
            Some(KwVal::List(l)) => l.clone(),
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        Ok(Options { ordinal, case, plural, prefer })
    }
}

/// Best-effort Python `str()` of a float/Decimal for TypeError messages
/// (`errmsg_floatord` / `errmsg_negord`). The corpus records only the
/// exception *type*; the message follows base.py's format strings.
fn float_msg_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value: f, .. } => {
            if f.is_finite() && f.fract() == 0.0 && f.abs() < 1e16 {
                format!("{:.1}", f)
            } else {
                format!("{}", f)
            }
        }
        FloatValue::Decimal { value: d, .. } => crate::strnum::python_decimal_str(d),
    }
}

impl Lang for LangFi {

    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        // Python class attribute MAXVAL (self-contained converter).
        Some(num_bigint::BigInt::from(10u32).pow(603))
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
        " ja"
    }

    // `cards()` / `maxval()` / `merge()` intentionally stay at their trait
    // defaults: FI's tables are not `base::Cards` and its `merge` needs the
    // structured value plus an `Options`, so the shared engine is bypassed
    // entirely. See the module docs.

    fn negword(&self) -> &str {
        "miinus "
    }

    fn pointword(&self) -> &str {
        "pilkku"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_FI.to_cardinal` for integral input.
    ///
    /// Note FI concatenates `self.negword` **raw** (`out = self.negword`),
    /// unlike `Num2Word_Base.to_cardinal`, which rebuilds it as
    /// `"%s " % self.negword.strip()`. Both yield "miinus " here, so the
    /// difference is invisible — but it is the reason this is written out
    /// rather than delegated to `base::default_to_cardinal`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // `is_title` is never set, so `title` inside is a no-op — kept for
        // fidelity.
        let options = Options { ordinal: false, case: NOM, plural: false, prefer: Vec::new() };
        self.to_cardinal_opts(value, &options)
    }

    /// Port of `Num2Word_FI.to_ordinal`.
    ///
    /// Re-runs the whole engine against `self.ords`; it does **not**
    /// post-process the cardinal. Negatives raise `TypeError` via
    /// `verify_ordinal` before anything else happens.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let options = Options { ordinal: true, case: NOM, plural: false, prefer: Vec::new() };
        self.to_ordinal_opts(value, &options)
    }

    /// Port of `Num2Word_FI.to_ordinal_num`: `str(value) + "."`.
    ///
    /// No `verify_ordinal`, no MAXVAL check — so unlike `to_ordinal` it happily
    /// accepts negatives: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_FI.to_year` with `suffix=None`.
    ///
    /// `self.to_cardinal(val).replace(" ", "") + suffix`: every space inside the
    /// cardinal is squeezed out (so 1005 → "tuhatviisi", not "tuhat viisi"),
    /// but the BC suffix keeps its own leading space because it is appended
    /// afterwards. The sign is stripped before `to_cardinal`, so a BC year
    /// never gets "miinus".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut val = value.clone();
        // suffix = suffix or "" -> "", which is falsy, hence the BC default.
        let mut suffix = "";
        if val.is_negative() {
            val = val.abs();
            suffix = " ennen ajanlaskun alkua";
        }
        Ok(format!("{}{}", self.to_cardinal(&val)?.replace(' ', ""), suffix))
    }

    // ---- float/Decimal entry routing --------------------------------------
    //
    // `cardinal_float_entry` stays at the trait default: FI's
    // `assert int(value) == value` routing *is* base semantics (whole ->
    // integer engine, fractional -> to_cardinal_float), verified against the
    // frozen corpus. Only ordinal/ordinal_num/year need overrides.

    /// `Num2Word_FI.to_ordinal(float/Decimal)` — `Num2Word_Base.verify_ordinal`
    /// runs first (after the NAME_TO_CASE lookup, which the no-kwargs entry
    /// cannot fail):
    ///
    ///   * fractional (`not value == int(value)`) → TypeError errmsg_floatord —
    ///     checked **before** the sign, so `to_ordinal(-1.5)` is the floatord
    ///     message;
    ///   * negative (`not abs(value) == value`) → TypeError errmsg_negord.
    ///     `abs(-0.0) == -0.0` is true, so **-0.0 passes** and yields "nollas";
    ///   * whole non-negative → the ordinal engine, same words as the int path
    ///     (`to_ordinal(5.0)` == "viides", `to_ordinal(Decimal("1E+2"))` ==
    ///     "sadas").
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                float_msg_repr(value)
            ))),
            Some(i) => {
                if i.is_negative() {
                    return Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        float_msg_repr(value)
                    )));
                }
                self.to_ordinal(&i)
            }
        }
    }

    /// `Num2Word_FI.to_ordinal_num(float/Decimal)`: `str(value) + "."`, no
    /// validation whatsoever — negatives and fractions included
    /// (`to_ordinal_num(-1.5)` == "-1.5.", `to_ordinal_num(1e+16)` == "1e+16.").
    /// `repr_str` is Python's `str(value)` as computed by the binding.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `Num2Word_FI.to_year(float/Decimal)`. `val < 0` is a *value* test —
    /// false for -0.0 (unlike `FloatValue::is_negative`, which reads the sign
    /// bit), so `to_year(-0.0)` == "nolla" with no BC suffix. The cardinal
    /// (int engine for whole values, decimal grammar for fractional) then has
    /// every space squeezed out, and the BC suffix keeps its own leading
    /// space: `to_year(-1.5)` == "yksipilkkuviisi ennen ajanlaskun alkua".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let (below_zero, abs) = match value {
            FloatValue::Float { value: f, precision } => (
                *f < 0.0,
                FloatValue::Float { value: f.abs(), precision: *precision },
            ),
            FloatValue::Decimal { value: d, precision } => (
                d.is_negative(),
                FloatValue::Decimal { value: d.abs(), precision: *precision },
            ),
        };
        let (val, suffix) = if below_zero {
            (&abs, " ennen ajanlaskun alkua")
        } else {
            (value, "")
        };
        let cardinal = self.cardinal_float_entry(val, None)?;
        Ok(format!("{}{}", cardinal.replace(' ', ""), suffix))
    }

    // ---- grammatical kwargs ------------------------------------------------

    /// `to_cardinal(value, case="nominative", plural=False, prefer=None)`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case", "plural", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let options = self.options_from_kwargs(kw, false)?;
        self.to_cardinal_opts(value, &options)
    }

    /// `to_ordinal(value, case="nominative", plural=False, prefer=None)`.
    ///
    /// `NAME_TO_CASE[case]` runs before `verify_ordinal`, so
    /// `to_ordinal(-5, case="bogus")` is **KeyError**, not TypeError —
    /// `options_from_kwargs` resolves the case first.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case", "plural", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let options = self.options_from_kwargs(kw, true)?;
        self.to_ordinal_opts(value, &options)
    }

    /// `to_ordinal_num(value, case="nominative", plural=False)`: the body never
    /// reads either kwarg (no NAME_TO_CASE lookup!), so `case="bogus"` is
    /// accepted and the result is still `str(value) + "."`.
    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["case", "plural"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        Ok(format!("{}.", value))
    }

    /// `to_year(val, suffix=None, longval=True)`. `longval` is never read.
    ///
    /// `suffix = suffix or ""` first, then — negatives only — `suffix = suffix
    /// or " ennen ajanlaskun alkua"`: a caller-supplied *truthy* suffix wins
    /// over the BC default, an empty/None one falls through to it.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let mut suffix: String = match kw.get("suffix") {
            None | Some(KwVal::None) => String::new(),
            Some(KwVal::Str(s)) => s.clone(),
            // A non-string suffix would concatenate onto str in Python
            // (TypeError) only after conversion; fall back to Python.
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        let mut val = value.clone();
        if val.is_negative() {
            val = val.abs();
            if suffix.is_empty() {
                suffix = " ennen ajanlaskun alkua".to_string();
            }
        }
        Ok(format!("{}{}", self.to_cardinal(&val)?.replace(' ', ""), suffix))
    }

    /// `to_cardinal(float/Decimal, case=..., plural=..., prefer=...)`.
    ///
    /// Python resolves `case` first (KeyError on a bad name, even for
    /// fractional input), then routes: whole → the integer engine *with* the
    /// options; fractional → NotImplementedError unless the case is
    /// nominative, in which case `to_cardinal_float` runs and `plural`/
    /// `prefer` are silently dropped.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["case", "plural", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let options = self.options_from_kwargs(kw, false)?;
        if let Some(i) = value.as_whole_int() {
            return self.to_cardinal_opts(&i, &options);
        }
        if options.case != NOM {
            // Python: raise NotImplementedError(...). NotImplemented also
            // makes the live binding fall back to Python, which raises the
            // very same exception — either route yields NotImplementedError.
            return Err(N2WError::NotImplemented(
                "Cases other than nominative are not implemented for \
                 cardinal floating point numbers."
                    .into(),
            ));
        }
        self.to_cardinal_float(value, precision_override)
    }

    // ---- currency -------------------------------------------------------
    //
    // `currency_precision` stays at the trait default (100 for every code):
    // FI defines no `CURRENCY_PRECISION`, so `Num2Word_Base`'s empty dict and
    // its `.get(code, 100)` are exactly that. `to_cheque`, `money_verbose`,
    // `cents_verbose` and `cents_terse` are inherited from `Num2Word_Base`
    // unchanged, and the trait defaults already mirror them.

    fn lang_name(&self) -> &str {
        "Num2Word_FI"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// raises IndexError. Every entry in FI's table has exactly two forms, so
    /// this is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_FI.to_currency`.
    ///
    /// FI intercepts **integers only** and hand-rolls the whole string; every
    /// other value is handed straight to `Num2Word_Base.to_currency` via
    /// `super()`. The int branch is not a re-spelling of Base's — it differs in
    /// three observable ways, all reproduced here:
    ///
    /// 1. **The double space (fixed, #160).** FI's Python concatenates
    ///    `self.negword` (`"miinus "`) *raw* into `"%s %s %s"`, so
    ///    `to_currency(-2, "EUR")` is `"miinus  kaksi euroa"`. The port strips
    ///    it as Base does and says `"miinus kaksi euroa"`.
    /// 2. **`adjective` is ignored.** The int branch never calls
    ///    `prefix_currency`, so `to_currency(2, "USD", adjective=True)` is
    ///    `"kaksi dollaria"` while `to_currency(2.0, "USD", adjective=True)` is
    ///    `"kaksi Yhdysvaltain dollaria ja nolla senttiä"`.
    /// 3. **`pluralize` is bypassed** for an inline `cr1[0]`/`cr1[1]` pick.
    ///    Same result as `Num2Word_EUR.pluralize` for FI's uniformly 2-form
    ///    table, but it is the literal code path.
    ///
    /// `cents` and `separator` are simply unused by the int branch — Python
    /// accepts and drops them, since an int never renders a cents segment.
    ///
    /// A missing code raises `KeyError` on `self.CURRENCY_FORMS[currency]`,
    /// which the `except (KeyError, AttributeError)` turns into a `super()`
    /// call — and *that* is what raises the NotImplementedError, carrying
    /// `self.__class__.__name__` == `"Num2Word_FI"`. So the fallback is a real
    /// delegation, not a shortcut to the error: routing it through
    /// `default_to_currency` keeps the message's origin honest.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `separator=" ja"` is FI's own signature default; the trait resolves
        // `None` through `default_separator()` for us.
        let separator = separator.unwrap_or(self.default_separator());

        // "Handle integers specially - just add currency name without cents".
        // Note this is `isinstance(val, int)`, so a whole *float* like 1.0 does
        // NOT come here — it keeps its cents segment via Base.
        if let CurrencyValue::Int(v) = val {
            let cr1 = match self.currency_forms.get(currency) {
                Some(forms) => &forms.unit,
                // KeyError -> "Fallback to base implementation for unknown
                // currency", which is where NotImplementedError comes from.
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };

            // `minus_str = self.negword if val < 0 else ""` — raw in Python;
            // stripped here so there is no double space (#160).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            // `cr1[0]` if abs_val == 1, else `cr1[1] if len(cr1) > 1 else cr1[0]`.
            let currency_str = if abs_val.is_one() {
                cr1.first()
            } else if cr1.len() > 1 {
                cr1.get(1)
            } else {
                cr1.first()
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            // ("%s %s %s" % (...)).strip() — see quirk (1) above.
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // "For floats, use the parent class implementation".
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
