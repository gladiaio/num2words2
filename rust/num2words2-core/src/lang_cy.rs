//! Port of `lang_CY.py` (Welsh).
//!
//! Shape: **self-contained**. `Num2Word_CY` subclasses `Num2Word_EUR` →
//! `Num2Word_Base`, but it overrides `__init__` with a bare `pass`. That single
//! line severs the whole inherited setup chain:
//!
//!   * `Num2Word_Base.__init__` never runs, so `setup()` is never called and
//!     `Num2Word_EUR.setup`'s `high_numwords` is never built.
//!   * `self.cards` and `self.MAXVAL` are therefore never created, and
//!     `set_high_numwords` / `gen_high_numwords` / `splitnum` / `clean` /
//!     `merge` are all dead code for this language.
//!   * `self.negword`, `self.pointword`, `self.is_title`, `self.precision` and
//!     the `errmsg_*` strings are likewise never assigned. CY substitutes its
//!     own class-level `MINUS_PREFIX_WORD = "meinws "` /
//!     `FLOAT_INFIX_WORD = " pwynt "` and hardcodes `("meinws", None)` into the
//!     word list, so nothing in the four in-scope modes ever touches the
//!     missing attributes. Base's `to_currency`/`to_fraction` did, and died
//!     with `AttributeError` on a negative value; the port's `negword` is
//!     "meinws" (gladiaio/num2words2#157).
//!
//! Consequently `cards`/`merge` stay at their trait defaults here, and
//! `to_cardinal` is overridden outright. The ceiling is CY's own explicit
//! `999 * 10**33` guard, which `maxval()` reports. Python raises
//! NotImplementedError there; fixed (gladiaio/num2words2#159): the port
//! raises `OverflowError` ("abs(v) must be less than 999 * 10^33."), like
//! `Num2Word_Base.to_cardinal` would.
//!
//! Inherited from `Num2Word_Base` (CY does not override either, so the trait
//! defaults are correct):
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`.
//!     Note Python returns the **int itself**, not a string; the corpus
//!     stringifies it, which the trait default reproduces.
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)` → the default
//!     delegates through `&self` and picks up the `to_cardinal` override below.
//!     Welsh has no special year form; `to_year(1999)` == `to_cardinal(1999)`.
//!
//! # How Welsh numerals work here
//!
//! The algorithm never builds a plain string until the very end. Every table
//! entry is a `(word, mutation)` pair where `mutation` describes the consonant
//! mutation this word triggers on the word that *follows* it (soft "SM" or
//! aspirate "AM"). [`makestring`] walks the assembled list, applies each
//! pending mutation to the next word, and joins with single spaces. Numerals
//! are vigesimal (base-20): 30 is "deg ar hugain" (ten on twenty), 80 is
//! "pedwar ugain" (four twenties), and 50-59 have a special "hanner (cant)"
//! (half a hundred) form.
//!
//! `OBJ` (`"__OBJECT__"`) is a placeholder marking where a counted noun would
//! be inserted (Welsh puts the noun *between* numeral parts). The `counted=`
//! kwarg that fills it is out of scope — the dispatcher's Rust fast path only
//! fires when no kwargs are present — so every `OBJ` is dropped here. See the
//! bug list below for why dropping it is nonetheless load-bearing.
//!
//! # Faithfully reproduced Python bugs and quirks
//!
//! This is a port, not a rewrite. All of the following are exactly what Python
//! emits, verified against the frozen corpus:
//!
//! 1. **`makestring`'s `continue` leaks the pending mutation across `OBJ`.**
//!    When an `OBJ` entry is skipped (`counted` is None), Python `continue`s
//!    *before* `lastmut = mut` runs, so the `OBJ`'s own `None` mutation is
//!    never consumed and the previous word's mutation survives to hit the next
//!    real word. This is what makes `200` == "dau gant" work: the list is
//!    `[("dau","SM"), (OBJ,None), ("cant",None)]`, and "cant" is soft-mutated
//!    to "gant" only because `OBJ` failed to clear the "SM". With a `counted`
//!    noun present the mutation would land on the noun instead and "cant" would
//!    stay unmutated. Modelled exactly in [`makestring`].
//! 2. **`CARDINAL_WORDS[19]` and `CARDINAL_WORDS_FEM[19]` are missing their
//!    `OBJ`** — every other entry in both tables has one. `19` is the only
//!    value whose counted noun therefore has nowhere to go. Kept verbatim;
//!    harmless in scope (all `OBJ`s are dropped anyway) but the asymmetry is
//!    clearly unintentional.
//! 3. **`ORDINAL_WORDS_FEM` is dead code.** `hundred_group` picks its table as
//!    `if gender == "fem": CW = CARDINAL_WORDS_FEM` *before* it ever checks
//!    `ordinal`, so a feminine ordinal silently falls back to the feminine
//!    **cardinal** words. Nothing in the module references
//!    `ORDINAL_WORDS_FEM`. It is transcribed below for the record but,
//!    faithfully, never read.
//! 4. **No "a"/"ac" conjunction between magnitude groups.** `1001` is
//!    "mil un", not "mil ac un"; `2001` is "dwy fil un". The `ac`/`a` join is
//!    only ever emitted *inside* `hundred_group`, between a hundreds part and
//!    its own remainder.
//! 5. **The `ac` list is tested against `until100`, not against the units.**
//!    So `1776` → "mil saith cant **a un** ar bymtheg a thrigain": the
//!    remainder is 76, which is not in the `ac` list, so "a" is chosen even
//!    though the word it precedes is "un" (which *would* have taken "ac" had
//!    the check been on the unit). Reproduced by [`AC_VALUES`].
//! 6. **`lowestgroup` compares group *values*, not positions.**
//!    `ordinal and (lowestgroup == gr)` matches any group whose value equals
//!    the lowest non-zero group's value, so `to_cardinal(1_000_001, ordinal=True)`
//!    would ordinalize both the millions and the units group. Unreachable from
//!    the four in-scope modes (see `to_ordinal`'s range gate below), but the
//!    comparison is ported as written rather than "fixed" to a position check.
//! 7. ~~**`to_ordinal` raises `KeyError` for every negative input.**~~ Fixed
//!    (gladiaio/num2words2#158). The guard is `if number < 20: return
//!    makestring(ORDINAL_WORDS[number])`, and every negative satisfies `< 20`,
//!    so Python's `ORDINAL_WORDS[-1]` missed the dict. The port raises Base's
//!    `errmsg_negord` `TypeError` instead, and a fractional float/Decimal
//!    `errmsg_floatord` (Python: `KeyError` too). `to_cardinal` and
//!    `to_ordinal_num` are unaffected.
//!
//! # Float / Decimal routing (the entry hooks)
//!
//! CY's `isinstance(number, float)` guard fires for **every** float, whole
//! values included: `to_cardinal(1.0)` is "un pwynt dim", never "un". So
//! [`Lang::cardinal_float_entry`] is overridden to skip the base default's
//! whole-value shortcut entirely — floats always take [`LangCy::float_to_words`].
//! Python sent every Decimal (and so every numeric string) down the integer
//! branch, which floored the fraction away: `"1.5"` was "un" and `"0.1"` was
//! "" (gladiaio/num2words2#156). A fractional Decimal now reads like a float
//! from its exact digits ([`LangCy::decimal_to_words`]); a whole one keeps the
//! integer branch ([`LangCy::decimal_to_cardinal`]).
//! `to_year` is Base's `to_cardinal(value)`, so the default
//! `year_float_entry` (→ `cardinal_float_entry`) is already right.
//!
//! `to_ordinal(float/Decimal)` is its own zoo ([`Lang::ordinal_float_entry`]):
//!
//! * Fractional and negative values raise `TypeError` first (bug 7, #158).
//! * `number < 20` → `ORDINAL_WORDS[number]`: `5.0` → "pumed", `-0.0` →
//!   "dimfed" (`-0.0 == 0`).
//! * `number == 100` → "canfed" (`1E+2` included); `> 100` →
//!   NotImplementedError — so `to_ordinal(101.0)` raises where
//!   `to_cardinal(101.0)` renders.
//! * 20 ≤ n < 100 → `to_cardinal(n, ordinal=True)`. For a *float* the
//!   isinstance guard wins first and `ordinal=True` is silently ignored:
//!   `to_ordinal(21.0)` is the **cardinal** "un ar hugain pwynt dim". A
//!   Decimal reaches the integer branch with `ordinal=True` live:
//!   `to_ordinal(Decimal('42'))` is "ail a deugain".
//!
//! `float_to_words` reads the digits out of `str(abs_float)`. Python repr
//! picks exponent notation at `abs >= 1e16` (and below `1e-4`), where
//! `.split(".")[1]` raises **IndexError** ("1e+16" has no dot) or the
//! digit-by-digit `int(c)` hits the 'e' and raises **ValueError**
//! ("1.5e+16"). The prefix `to_cardinal(int(abs_float))` is computed *before*
//! the split, so a float past `999 * 10**33` raises OverflowError (#159)
//! first. Reproduced via [`py_float_str`].
//!
//! `Decimal('Infinity')` / `Decimal('NaN')` parse fine in `str_to_number` and
//! only blow up inside `to_cardinal`: the `not number < 999 * 10**33` ceiling
//! fires for ±Infinity (an OverflowError since #159; NotImplementedError in
//! Python), and `number < 0` raises **decimal.InvalidOperation** for NaN.
//! Both are served natively by [`Lang::inf_result`] / [`Lang::nan_result`].
//!
//! # Fractions
//!
//! CY inherits `Num2Word_Base.to_fraction`, whose negative branch is
//! `sign = "%s " % self.negword.strip()` — and the `pass` `__init__` never
//! created `self.negword`, so in Python **any negative fraction raised
//! AttributeError** (`-1/2`, `1/-2`). Fixed (#157): the sign is "meinws".
//! `Num2Word_CY` overrides `to_currency`, `_money_verbose` and
//! `_cents_verbose`; it inherits `pluralize` from `Num2Word_EUR` and
//! `to_cheque` / `_cents_terse` from `Num2Word_Base` (all four confirmed on the
//! live interpreter via `__func__.__qualname__`).
//!
//! Two consequences of the `pass` __init__ dominate this surface:
//!
//! 1. ~~**`self.negword` never exists**~~, and `Num2Word_Base.to_currency`
//!    reaches for it — `minus_str = "%s " % self.negword.strip() if
//!    is_negative else ""` — so in Python a **negative float raised
//!    `AttributeError`**. Fixed (#157): `negword` is "meinws", CY's own
//!    cardinal minus word, so `-0.5` reads "meinws ...".
//! 2. **`CURRENCY_PRECISION` is `Num2Word_Base`'s shared `{}`** (verified:
//!    `Num2Word_CY.CURRENCY_PRECISION is Num2Word_Base.CURRENCY_PRECISION`).
//!    `Num2Word_EN.__init__` *rebinds* rather than mutates it, so EN's mils
//!    table does not leak here. Every code is therefore divisor 100 — CY has no
//!    3-decimal and no 0-decimal currency, and `currency_precision` stays at its
//!    trait default. The `divisor == 1` branch of `default_to_currency` is
//!    unreachable for Welsh.
//!
//! ## `CURRENCY_FORMS` is CY's own dict, not the mutated EUR one
//!
//! The usual `lang_EUR.py` trap does **not** apply: `Num2Word_CY` rebinds
//! `CURRENCY_FORMS` in its own class body, so it is a fresh dict object and
//! `Num2Word_EN.__init__`'s in-place mutation of `Num2Word_EUR.CURRENCY_FORMS`
//! never touches it (verified: `Num2Word_CY.CURRENCY_FORMS is
//! Num2Word_EUR.CURRENCY_FORMS` → `False`). It holds exactly four codes —
//! EUR, USD, GBP, CNY — and EN's ~24 added codes are **absent**. JPY, KWD, BHD,
//! INR and CHF all raise `NotImplementedError` on the float path, as the corpus
//! records.
//!
//! `CURRENCY_ADJECTIVES`, by contrast, *is* the shared `Num2Word_EUR` object
//! (verified `is` → `True`) and nothing mutates it, so its 16 entries are
//! inherited verbatim. Only `USD` is both adjectivised and present in CY's
//! forms, so `adjective=True` is observable for USD alone.
//!
//! # More faithfully reproduced Python bugs (currency)
//!
//! 8. **`to_currency` ignores `currency=` entirely for `int` input.** CY's
//!    override intercepts `isinstance(val, int)` *before* delegating to
//!    `super()`, and its branch never consults `CURRENCY_FORMS`. So every int
//!    is denominated in mutated GBP — `to_currency(2, "JPY")` is
//!    `"dau bunnoedd"`, not an error and not yen. This is why the corpus shows
//!    `currency:JPY` / `currency:KWD` / `currency:CHF` succeeding for `0`, `1`,
//!    `2`, `100`, `1000000` while every float with the same code raises
//!    `NotImplementedError`. Reproduced in [`LangCy::to_currency`].
//! 9. **The int branch said `"minws "`, not `"meinws "` (fixed, #180).** CY's
//!    own `MINUS_PREFIX_WORD` is `"meinws "` and `to_cardinal` hardcodes
//!    `("meinws", None)`, but Python's `to_currency` spells the negative
//!    prefix `"minws "` — a typo. The port says `"meinws "` like the cardinal
//!    and the float path: `to_currency(-1)` is `"meinws un bunt"`.
//! 10. **The unit words in the int branch are pre-mutated literals.**
//!     `"bunt"` / `"bunnoedd"` are the soft-mutated forms of `punt`/`punnoedd`,
//!     hardcoded as strings rather than produced by [`softmutation`] — so the
//!     mutation fires even when nothing precedes it that would trigger one
//!     (`"dim bunnoedd"` after "dim", which triggers no mutation at all).
//! 11. **`_cents_verbose(1, ...)` drops the numeral.** The `number > 1` guard
//!     sends 1 to `m = [(OBJ, None)]`, so `0.01` renders `"... ceiniog"` — the
//!     counted noun alone, with no "un".
//! 12. **`_money_verbose` always asks `to_cardinal` for the *feminine* form**,
//!     even for masculine currencies: `2.0 USD` is `"dwy dolar, ..."`.
//!     The dead `if currency in CURRENCIES_FEM` guard is commented out in the
//!     source with the note "always true in this context". `_cents_verbose`, by
//!     contrast, leaves gender at its masculine default.
//! 13. ~~**Every float prints the unit twice.**~~ Fixed
//!     (gladiaio/num2words2#162). `_money_verbose` already appends the
//!     currency noun (as `counted=`, or as the `o <plural>` partitive above
//!     100) and `_cents_verbose` the subunit noun, and then
//!     `Num2Word_Base.to_currency` appended `pluralize(left, cr1)` and
//!     `pluralize(right, cr2)` on top: `1.5` was "un punt punt, hanner cant
//!     ceiniog ceiniogau", `1.01` "un punt punt, ceiniog ceiniog". The port
//!     keeps the counted forms and drops the generic ones, so each unit is
//!     named once ("un punt, hanner cant ceiniog"). With `adjective=True`
//!     only the adjective survives from the generic form ("un dolar US").
//! 14. **Zero cents leave a double space.** `_cents_verbose(0, ...)` returns
//!     `""`, and Base's `"%s%s %s%s %s %s"` template still emits the spaces
//!     around it: `1.0` → `"un euro,  ceiniogau"` (two spaces after the
//!     comma). The `has_decimal` guard keeps the segment alive for `1.0`.
//!
//! # Error variants
//!
//! * `NotImplementedError` → [`N2WError::NotImplemented`]: `to_ordinal(n)` for
//!   `n > 100`, `to_cardinal(n)` for `abs(n) >= 999 * 10**33`, and a currency
//!   code outside CY's four on the float/cheque paths.
//! * `TypeError` → [`N2WError::Type`]: `to_ordinal(n)` for `n < 0` or a
//!   fractional value (bug 7, fixed in #158; Python raised `KeyError`).
//! * Python's `AttributeError` for `to_currency(<negative float>)` (the
//!   missing `self.negword`) is fixed (#157).

use std::sync::OnceLock;
use crate::base::{
    check_maxval, pow10_big, strictly_negative, verify_ordinal, verify_ordinal_float, Kwargs,
    KwVal, Lang, N2WError, Result,
};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// Python's `OBJ = "__OBJECT__"` sentinel: where a counted noun would be
/// spliced in. Compared by value in `makestring`, exactly as Python does.
const OBJ: &str = "__OBJECT__";

/// The mutation a word triggers on the word that *follows* it.
///
/// Python stores `None` / `"SM"` / `"AM"` in the second tuple slot; `Option<Mut>`
/// mirrors that. `mutate()` in Python has no `else` (the comment says "does not
/// occur"), so it would return `None` for any other tag — unreachable, since
/// `makestring` only calls it under `if lastmut:` and the tables hold nothing
/// else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mut {
    /// "SM" — soft mutation (treiglad meddal).
    Soft,
    /// "AM" — aspirate mutation (treiglad llaes).
    Aspirate,
}

/// One `(word, triggers_mutation)` table entry.
type Entry = (&'static str, Option<Mut>);

/// The same pair, but borrowing a word that need not be `'static`.
///
/// `_money_verbose` / `_cents_verbose` splice a currency noun out of
/// `CURRENCY_FORMS` (a `String` owned by [`LangCy`]) into the middle of a word
/// list, so those lists cannot be `Vec<Entry>`. `Entry` coerces to `Word<'a>`
/// for any `'a`, which lets both feed the one [`makestring`].
type Word<'a> = (&'a str, Option<Mut>);

use Mut::{Aspirate as AM, Soft as SM};

/// `CARDINAL_WORDS`, keys 0..=19 (masculine).
///
/// Index 19 is missing its `OBJ` — see module bug 2.
const CARDINAL_WORDS: [&[Entry]; 20] = [
    /*  0 */ &[("dim", None), (OBJ, None)],
    /*  1 */ &[("un", None), (OBJ, None)],
    /*  2 */ &[("dau", Some(SM)), (OBJ, None)],
    /*  3 */ &[("tri", Some(AM)), (OBJ, None)],
    /*  4 */ &[("pedwar", None), (OBJ, None)],
    /*  5 */ &[("pump", None), (OBJ, None)],
    /*  6 */ &[("chwech", Some(AM)), (OBJ, None)],
    /*  7 */ &[("saith", None), (OBJ, None)],
    /*  8 */ &[("wyth", None), (OBJ, None)],
    /*  9 */ &[("naw", None), (OBJ, None)],
    /* 10 */ &[("deg", None), (OBJ, None)],
    /* 11 */ &[("un", None), (OBJ, None), ("ar ddeg", None)],
    /* 12 */ &[("deuddeg", None), (OBJ, None)],
    /* 13 */ &[("tri", Some(AM)), (OBJ, None), ("ar ddeg", None)],
    /* 14 */ &[("pedwar", None), (OBJ, None), ("ar ddeg", None)],
    /* 15 */ &[("pymtheg", None), (OBJ, None)],
    /* 16 */ &[("un", None), (OBJ, None), ("ar bymtheg", None)],
    /* 17 */ &[("dau", Some(SM)), (OBJ, None), ("ar bymtheg", None)],
    /* 18 */ &[("deunaw", None), (OBJ, None)],
    /* 19 */ &[("pedwar", None), ("ar bymtheg", None)], // no OBJ — bug 2
];

/// `CARDINAL_WORDS_FEM`, keys 0..=19 (feminine).
///
/// Index 19 is missing its `OBJ` — see module bug 2.
const CARDINAL_WORDS_FEM: [&[Entry]; 20] = [
    /*  0 */ &[("dim", None), (OBJ, None)],
    /*  1 */ &[("un", None), (OBJ, None)],
    /*  2 */ &[("dwy", Some(SM)), (OBJ, None)],
    /*  3 */ &[("tair", None), (OBJ, None)],
    /*  4 */ &[("pedair", None), (OBJ, None)],
    /*  5 */ &[("pump", None), (OBJ, None)],
    /*  6 */ &[("chwech", Some(AM)), (OBJ, None)],
    /*  7 */ &[("saith", None), (OBJ, None)],
    /*  8 */ &[("wyth", None), (OBJ, None)],
    /*  9 */ &[("naw", None), (OBJ, None)],
    /* 10 */ &[("deg", None), (OBJ, None)],
    /* 11 */ &[("un", None), (OBJ, None), ("ar ddeg", None)],
    /* 12 */ &[("deuddeg", None), (OBJ, None)],
    /* 13 */ &[("tair", None), (OBJ, None), ("ar ddeg", None)],
    /* 14 */ &[("pedair", None), (OBJ, None), ("ar ddeg", None)],
    /* 15 */ &[("pymtheg", None), (OBJ, None)],
    /* 16 */ &[("un", None), (OBJ, None), ("ar bymtheg", None)],
    /* 17 */ &[("dwy", Some(SM)), (OBJ, None), ("ar bymtheg", None)],
    /* 18 */ &[("deunaw", None), (OBJ, None)],
    /* 19 */ &[("pedair", None), ("ar bymtheg", None)], // no OBJ — bug 2
];

/// `ORDINAL_WORDS`, keys 0..=19 (masculine).
const ORDINAL_WORDS: [&[Entry]; 20] = [
    /*  0 */ &[("dimfed", None), (OBJ, None)],
    /*  1 */ &[(OBJ, None), ("cyntaf", None)],
    /*  2 */ &[("ail", Some(SM)), (OBJ, None)],
    /*  3 */ &[("trydydd", None), (OBJ, None)],
    /*  4 */ &[("pedwerydd", None), (OBJ, None)],
    /*  5 */ &[("pumed", None), (OBJ, None)],
    /*  6 */ &[("chweched", None), (OBJ, None)],
    /*  7 */ &[("saithfed", None), (OBJ, None)],
    /*  8 */ &[("wythfed", None), (OBJ, None)],
    /*  9 */ &[("nawfed", None), (OBJ, None)],
    /* 10 */ &[("degfed", None), (OBJ, None)],
    /* 11 */ &[("unfed", Some(SM)), (OBJ, None), ("ar ddeg", None)],
    /* 12 */ &[("deuddegfed", None), (OBJ, None)],
    /* 13 */ &[("trydydd", None), (OBJ, None), ("ar ddeg", None)],
    /* 14 */ &[("pedwerydd", None), (OBJ, None), ("ar ddeg", None)],
    /* 15 */ &[("pymthegfed", None), (OBJ, None)],
    /* 16 */ &[("unfed", None), (OBJ, None), ("ar bymtheg", None)],
    /* 17 */ &[("ail", Some(SM)), (OBJ, None), ("ar bymtheg", None)],
    /* 18 */ &[("deunawfed", None), (OBJ, None)],
    /* 19 */ &[("pedwerydd", None), (OBJ, None), ("ar bymtheg", None)],
];

/// `ORDINAL_WORDS_FEM`, keys 0..=19.
///
/// **Dead in Python** — see module bug 3. `hundred_group` tests `gender ==
/// "fem"` before it tests `ordinal`, so a feminine ordinal gets
/// `CARDINAL_WORDS_FEM` instead and this table is never read. Transcribed for
/// the record; deliberately left unreferenced so the Rust port has the same
/// reachable behaviour.
#[allow(dead_code)]
const ORDINAL_WORDS_FEM: [&[Entry]; 20] = [
    /*  0 */ &[("dimfed", None), (OBJ, None)],
    /*  1 */ &[(OBJ, None), ("gyntaf", None)],
    /*  2 */ &[("ail", Some(SM)), (OBJ, None)],
    /*  3 */ &[("trydedd", Some(SM)), (OBJ, None)],
    /*  4 */ &[("pedwaredd", Some(SM)), (OBJ, None)],
    /*  5 */ &[("pumed", None), (OBJ, None)],
    /*  6 */ &[("chweched", None), (OBJ, None)],
    /*  7 */ &[("saithfed", None), (OBJ, None)],
    /*  8 */ &[("wythfed", None), (OBJ, None)],
    /*  9 */ &[("nawfed", None), (OBJ, None)],
    /* 10 */ &[("degfed", None), (OBJ, None)],
    /* 11 */ &[("unfed", Some(SM)), (OBJ, None), ("ar ddeg", None)],
    /* 12 */ &[("deuddegfed", None), (OBJ, None)],
    /* 13 */ &[("trydedd", Some(SM)), (OBJ, None), ("ar ddeg", None)],
    /* 14 */ &[("pedwaredd", Some(SM)), (OBJ, None), ("ar ddeg", None)],
    /* 15 */ &[("pymthegfed", None), (OBJ, None)],
    /* 16 */ &[("unfed", None), (OBJ, None), ("ar bymtheg", None)],
    /* 17 */ &[("ail", Some(SM)), (OBJ, None), ("ar bymtheg", None)],
    /* 18 */ &[("deunawfed", None), (OBJ, None)],
    /* 19 */ &[("pedwaredd", None), (OBJ, None), ("ar bymtheg", None)],
];

/// `MILLION_WORDS`: exponent → magnitude word. Python keys are 3..=33 step 3.
///
/// Indexed as `MILLION_WORDS[pot - 3]` where `pot` ranges over 6..=36, so the
/// live key range is exactly 3..=33 and a `KeyError` is unreachable — the
/// `999 * 10**33` ceiling in `to_cardinal` stops before 10**36 would need a
/// key 36. Spellings kept verbatim, including "secsttiliwn" (double t) and
/// "dengiliwn".
fn million_word(exponent: u32) -> Entry {
    match exponent {
        3 => ("mil", None),
        6 => ("miliwn", None),
        9 => ("biliwn", None),
        12 => ("triliwn", None),
        15 => ("cwadriliwn", None),
        18 => ("cwintiliwn", None),
        21 => ("secsttiliwn", None),
        24 => ("septiliwn", None),
        27 => ("octiliwn", None),
        30 => ("noniliwn", None),
        33 => ("dengiliwn", None),
        // Python: MILLION_WORDS[pot - 3] → KeyError. Unreachable given the
        // 999 * 10**33 ceiling; kept as the same exception type regardless.
        _ => unreachable!("million_word({}) — outside the 999*10**33 ceiling", exponent),
    }
}

/// `STR_TENS` values, keys 1..=4 (20, 40, 60, 80).
const STR_TENS_1: &[Entry] = &[("ugain", None), (OBJ, None)];
const STR_TENS_2: &[Entry] = &[("deugain", None), (OBJ, None)];
const STR_TENS_3: &[Entry] = &[("trigain", None), (OBJ, None)];
const STR_TENS_4: &[Entry] = &[("pedwar ugain", None), (OBJ, None)];

/// `ORD_STR_TENS` values, keys 1..=4.
const ORD_STR_TENS_1: &[Entry] = &[("ugainfed", None), (OBJ, None)];
const ORD_STR_TENS_2: &[Entry] = &[("deugainfed", None), (OBJ, None)];
const ORD_STR_TENS_3: &[Entry] = &[("trigainfed", None), (OBJ, None)];
const ORD_STR_TENS_4: &[Entry] = &[("pedwar ugainfed", None), (OBJ, None)];

/// `STR_TENS.get(tens)`: vigesimal scores. Returns `None` for `tens == 0`,
/// which `hundred_group` relies on (a 0-19 remainder inside a hundreds group
/// emits its unit words with no score attached).
fn str_tens(tens: i64) -> Option<&'static [Entry]> {
    match tens {
        1 => Some(STR_TENS_1),
        2 => Some(STR_TENS_2),
        3 => Some(STR_TENS_3),
        4 => Some(STR_TENS_4),
        _ => None,
    }
}

/// `ORD_STR_TENS.get(tens)`: ordinal forms of the scores.
fn ord_str_tens(tens: i64) -> Option<&'static [Entry]> {
    match tens {
        1 => Some(ORD_STR_TENS_1),
        2 => Some(ORD_STR_TENS_2),
        3 => Some(ORD_STR_TENS_3),
        4 => Some(ORD_STR_TENS_4),
        _ => None,
    }
}

/// The `until100` values that take "ac" rather than "a" after a hundreds part.
///
/// Tested against the whole 0-99 remainder, not the unit — see module bug 5.
const AC_VALUES: [i64; 16] = [1, 8, 11, 16, 20, 21, 31, 36, 41, 48, 61, 68, 71, 81, 88, 91];

// `STR_TENS_INFORMAL` is defined in Python but never read: `hundred_group`'s
// only use site is commented out (`# if informal: pass`). The `informal=` kwarg
// is threaded through every call and then ignored. Not ported.

/// Python's `mutate(word, mutation)`.
///
/// Python has no `else` arm here (the comment reads "does not occur"), so a tag
/// other than SM/AM would return `None`. Unreachable: `makestring` only calls
/// it under `if lastmut:` and the tables hold nothing else. `Mut` makes that
/// unrepresentable rather than relying on the comment.
fn mutate(word: &str, mutation: Mut) -> String {
    match mutation {
        Mut::Soft => softmutation(word),
        Mut::Aspirate => aspiratedmutation(word),
    }
}

/// Python's `softmutation` (treiglad meddal).
///
/// Python indexes `word[0]` / `word[1]` directly. `word[1]` is only reached
/// after `word[0]` already matched a specific letter, thanks to `and`'s
/// short-circuit, so a 1-character word like "a" never touches `word[1]`. No
/// table word is a single p/t/c/d, so the `IndexError` Python would raise on
/// e.g. `softmutation("p")` is unreachable; `nth(1)` yielding `None` here
/// stands in for it and cannot be observed.
///
/// Indexed by `chars()`, never bytes: all the words are ASCII today, but the
/// `word[1:]` slice must stay a character slice to be safe.
fn softmutation(word: &str) -> String {
    let mut it = word.chars();
    let c0 = it.next();
    let c1 = it.next();
    // Everything after the first character, as Python's `word[1:]`.
    let rest: String = word.chars().skip(1).collect();

    if c0 == Some('p') && c1 != Some('h') {
        format!("b{}", rest)
    } else if c0 == Some('t') && c1 != Some('h') {
        format!("d{}", rest)
    } else if c0 == Some('c') && c1 != Some('h') {
        format!("g{}", rest)
    } else if c0 == Some('b') || c0 == Some('m') {
        format!("f{}", rest)
    } else if c0 == Some('d') && c1 != Some('d') {
        // Python: "d" + word — the whole word, not the tail. "deg" → "ddeg".
        format!("d{}", word)
    } else if word.starts_with("ll") {
        // Python: word[1:] — drops one l, giving "l...".
        rest
    } else if word.starts_with("rh") {
        format!("r{}", word.chars().skip(2).collect::<String>())
    } else if word == "ugain" {
        "hugain".to_string()
    } else {
        word.to_string()
    }
}

/// Python's `aspiratedmutation` (treiglad llaes).
///
/// Note the `word[1] != "h"` guards: "chwech" already starts "ch", so it falls
/// through to the `else` and is returned unchanged rather than becoming
/// "chchwech".
fn aspiratedmutation(word: &str) -> String {
    let mut it = word.chars();
    let c0 = it.next();
    let c1 = it.next();
    let rest: String = word.chars().skip(1).collect();

    if c0 == Some('p') && c1 != Some('h') {
        format!("ph{}", rest)
    } else if c0 == Some('t') && c1 != Some('h') {
        format!("th{}", rest)
    } else if c0 == Some('c') && c1 != Some('h') {
        format!("ch{}", rest)
    } else {
        word.to_string()
    }
}

/// Python's module-level `makestring(result, counted=None)`.
///
/// Concatenates the `(word, mutation)` list, applying each entry's mutation to
/// the *following* word.
///
/// **The `continue` on a dropped `OBJ` is load-bearing** (module bug 1): it
/// skips `lastmut = mut`, so the pending mutation survives the `OBJ` and lands
/// on the next real word. `[("dau","SM"), (OBJ,None), ("cant",None)]` is
/// "dau gant" precisely because of this. `counted` is always `None` in the
/// four in-scope modes, so every `OBJ` takes that path.
fn makestring<'a>(result: &[Word<'a>], counted: Option<&str>) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut lastmut: Option<Mut> = None;
    // Python rebinds `counted = None` after the first substitution, so only the
    // first OBJ is filled.
    let mut counted = counted;

    // `Word` is Copy, so `&(w, mut_)` binds the fields by value.
    for &(w, mut_) in result.iter() {
        let word: &str = if w == OBJ {
            match counted.take() {
                // `continue` — does NOT run `lastmut = mut`. See bug 1.
                None => continue,
                Some(c) => c,
            }
        } else {
            w
        };

        match lastmut {
            Some(m) => out.push(mutate(word, m)),
            None => out.push(word.to_string()),
        }
        lastmut = mut_;
    }
    out.join(" ")
}

/// Python's `str(<float>)` for a finite value, reconstructed.
///
/// * Fixed range — `1e-4 <= abs < 1e16` (and 0.0): fixed notation with the
///   repr-derived `precision` fractional digits, which reproduces the exact
///   shortest-repr digits (verified byte-for-byte against the live
///   interpreter over the corpus float set; the existing port relied on the
///   same equivalence).
/// * Exponent range — `abs >= 1e16` or `0 < abs < 1e-4`: Python repr picks
///   exponent form ("1e+16", "5e-05", "1.5e+16"). Rust's `{:e}` is the same
///   shortest mantissa; only the exponent spelling differs (`e16` → `e+16`,
///   zero-padded to two digits like CPython).
///
/// The sign uses the sign *bit* (`str(-0.0)` is "-0.0"), matching Python's
/// formatter — callers that need Python's `< 0` semantics (float_to_words)
/// pass the already-abs()'d value.
fn py_float_str(value: f64, precision: u32) -> String {
    let a = value.abs();
    let mag = if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        let s = format!("{:e}", a);
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("exponent is a small int");
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else {
        format!("{:.*}", precision as usize, a)
    };
    if value.is_sign_negative() {
        format!("-{}", mag)
    } else {
        mag
    }
}

/// Python `repr` of the `raw=True` word list — a list of `(str, str|None)`
/// tuples: `[('dau', 'SM'), ('__OBJECT__', None)]`. No table word contains a
/// quote, so the single-quote form is always what CPython prints.
fn py_repr_wordlist(words: &[Entry]) -> String {
    let items: Vec<String> = words
        .iter()
        .map(|(w, m)| {
            let m_repr = match m {
                Some(Mut::Soft) => "'SM'",
                Some(Mut::Aspirate) => "'AM'",
                None => "None",
            };
            format!("('{}', {})", w, m_repr)
        })
        .collect();
    format!("[{}]", items.join(", "))
}

/// Python truthiness for a kwarg value (`if raw:` and friends).
fn kw_truthy(v: &KwVal) -> bool {
    match v {
        KwVal::Bool(b) => *b,
        KwVal::Int(i) => *i != 0,
        KwVal::Str(s) => !s.is_empty(),
        KwVal::List(l) => !l.is_empty(),
        KwVal::None => false,
    }
}

/// Python's `Num2Word_CY.hundred_group`.
///
/// `number` is a single group of three digits (0..=999), so `i64` is provably
/// wide enough — the caller has already reduced the BigInt via
/// `(n % 10**pot) // 10**(pot-3)`.
///
/// `informal` is accepted and ignored, exactly as in Python (its only use site
/// is commented out).
fn hundred_group(number: i64, _informal: bool, gender: &str, ordinal: bool) -> Vec<Entry> {
    let hundreds = number / 100;
    let until100 = number % 100; // 0 - 99
    let mut result: Vec<Entry> = Vec::new();

    // NOTE: the fem branch wins before `ordinal` is ever consulted, so
    // ORDINAL_WORDS_FEM is unreachable. Module bug 3.
    let cw: &[&[Entry]; 20] = if gender == "fem" {
        &CARDINAL_WORDS_FEM
    } else if ordinal {
        &ORDINAL_WORDS
    } else {
        &CARDINAL_WORDS
    };

    if hundreds > 0 {
        if hundreds > 1 {
            // Python uses CARDINAL_WORDS here unconditionally — never CW — so
            // the hundreds multiplier is always masculine and always cardinal,
            // even for a feminine or ordinal group. 300000 is "tri chant mil"
            // with masculine "tri" although the group is feminine.
            result.extend_from_slice(CARDINAL_WORDS[hundreds as usize]);
        }
        result.extend_from_slice(&[("cant", None), (OBJ, None)]);
        if until100 != 0 {
            if AC_VALUES.contains(&until100) {
                result.push(("ac", None));
            } else {
                result.push(("a", Some(AM)));
            }
        }
    }

    if until100 != 0 {
        if !ordinal && (50..=59).contains(&until100) {
            // The "half a hundred" idiom: 50-59 are "hanner cant ..." standalone
            // or "... a hanner ..." after an explicit hundreds part. Cardinal
            // only — ordinals fall through to the vigesimal branch, which is why
            // to_ordinal(50) is "degfed a deugain" while to_cardinal(50) is
            // "hanner cant".
            let units = number % 10;
            if hundreds > 0 {
                if units == 0 {
                    result.push(("hanner", None));
                } else if units == 1 {
                    result.extend_from_slice(&[("hanner ac un", None), (OBJ, None)]);
                } else {
                    result.push(("hanner a", Some(AM)));
                    result.extend_from_slice(cw[units as usize]);
                }
            } else if units == 0 {
                result.extend_from_slice(&[("hanner cant", None), (OBJ, None)]);
            } else if units == 1 {
                result.extend_from_slice(&[("hanner cant ac un", None), (OBJ, None)]);
            } else {
                result.push(("hanner cant a", Some(AM)));
                result.extend_from_slice(cw[units as usize]);
            }
        } else if (number < 20 && number > 0) || (number == 0 && hundreds == 0) {
            // Python indexes the *cardinal* tables directly here, ignoring CW —
            // so this branch is never ordinal. Unreachable for ordinals anyway:
            // to_ordinal handles < 20 itself and never delegates here.
            if gender == "fem" {
                result.extend_from_slice(CARDINAL_WORDS_FEM[number as usize]);
            } else {
                result.extend_from_slice(CARDINAL_WORDS[number as usize]);
            }
        } else {
            // Vigesimal: `tens` counts whole scores of 20 in the 0-99
            // remainder, `units` is the residue mod 20 of the *whole group*
            // (identical to until100 % 20, since 100 % 20 == 0).
            let tens = until100 / 20;
            let units = number % 20;
            let degau = if ordinal && units == 0 {
                ord_str_tens(tens)
            } else {
                str_tens(tens)
            };

            if units != 0 {
                if tens > 1 {
                    // 40/60/80 take "a" (aspirate): 42 → "dau a deugain".
                    result.extend_from_slice(cw[units as usize]);
                    if let Some(d) = degau {
                        result.push(("a", Some(AM)));
                        result.extend_from_slice(d);
                    }
                } else {
                    // 20 takes "ar" (soft): 21 → "un ar hugain". tens == 0
                    // leaves degau None, so 1-19 within a hundreds group emit
                    // just the unit words.
                    result.extend_from_slice(cw[units as usize]);
                    if let Some(d) = degau {
                        result.push(("ar", Some(SM)));
                        result.extend_from_slice(d);
                    }
                }
            } else if let Some(d) = degau {
                result.extend_from_slice(d);
            }
        }
    }
    result
}

/// The `999 * 10**33` ceiling from `to_cardinal` — the exclusive maxval
/// (gladiaio/num2words2#159).
fn maxval_cy() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| BigInt::from(999) * pow10_big(33))
}

/// `Num2Word_CY.CURRENCY_FORMS` — CY's **own** class-body dict, all four codes.
///
/// Not the EUR dict EN mutates: CY rebinds the name, so this is a distinct
/// object holding only what `lang_CY.py` lists. Everything EN adds to the EUR
/// table (AUD, CAD, JPY, KWD, BHD, CHF, INR, ...) is absent here and must stay
/// absent — the corpus expects `NotImplementedError` for each on the float path.
///
/// `GENERIC_DOLLARS` / `GENERIC_CENTS` are `lang_CY.py`'s own module-level
/// constants, not the same-named ones in `lang_EUR.py` ("dollar"/"cent"); the
/// Welsh module shadows them with "dolar"/"ceiniog".
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const GENERIC_DOLLARS: [&str; 2] = ["dolar", "dolarau"];
    const GENERIC_CENTS: [&str; 2] = ["ceiniog", "ceiniogau"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &GENERIC_CENTS));
    m.insert("USD", CurrencyForms::new(&GENERIC_DOLLARS, &GENERIC_CENTS));
    // Python spells GBP's subunit out inline instead of reusing GENERIC_CENTS.
    // Identical content; transcribed as written.
    m.insert(
        "GBP",
        CurrencyForms::new(&["punt", "punnoedd"], &["ceiniog", "ceiniogau"]),
    );
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuans"], &["ffen", "ffens"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited untouched.
///
/// CY defines none of its own and this is literally the EUR class dict (`is`
/// → `True` on the live interpreter), which nothing mutates. Only `USD` is both
/// listed here and present in [`build_currency_forms`], so `adjective=True` can
/// only ever be observed as `"US dolar"` / `"US dolarau"` — the other fifteen
/// codes fail the `CURRENCY_FORMS` lookup first. Ported whole regardless: it is
/// the inherited data, and the `currency in self.CURRENCY_ADJECTIVES` test reads
/// all of it.
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

pub struct LangCy {
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangCy {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCy {
    pub fn new() -> Self {
        LangCy {
            // Built once here, never per call — `to_currency` only ever reads
            // these tables.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// Python's `Num2Word_CY.to_cardinal(..., raw=True)`: the word list, before
    /// [`makestring`] flattens it.
    ///
    /// Split out of `to_cardinal_full` unchanged. `raw=True` is exactly the same
    /// computation minus the trailing `makestring` call, and `_money_verbose` /
    /// `_cents_verbose` need the list form so they can splice the currency noun
    /// into it (that is the *only* reason `raw` exists in Python).
    ///
    /// The float branch (`isinstance(number, float)` → `float_to_words`) is out
    /// of scope: integer input only.
    fn to_cardinal_raw(
        &self,
        number: &BigInt,
        informal: bool,
        gender: &str,
        ordinal: bool,
    ) -> Result<Vec<Entry>> {
        let mut negative = false;
        let mut number = number.clone();
        if number.is_negative() {
            negative = true;
            number = -number;
        }
        if number.is_zero() {
            // Python: `if raw: return CARDINAL_WORDS[0]`. Note this is the
            // *masculine* table even when gender="fem" — the zero guard runs
            // before any gender dispatch, so `_money_verbose(0, ...)` gets
            // "dim" either way.
            return Ok(CARDINAL_WORDS[0].to_vec());
        }
        // Python's `elif not number < 999 * 10**33` raised NotImplementedError;
        // an OverflowError since gladiaio/num2words2#159.
        check_maxval(&number, maxval_cy())?;

        // Split into groups of three digits, from the right. Python iterates
        // pot ascending, so `lowestgroup` ends up holding the *value* of the
        // lowest non-zero group (module bug 6: a value, not a position).
        let mut groups: Vec<(i64, u32)> = Vec::new();
        let mut lowestgroup: Option<i64> = None;
        for pot in [3u32, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36] {
            let hi = BigInt::from(10u32).pow(pot);
            let lo = BigInt::from(10u32).pow(pot - 3);
            // Both operands are non-negative here (the sign was stripped
            // above), so Python's floor semantics and Rust's truncation agree;
            // div_floor is used anyway to keep the correspondence exact.
            let gr = number.mod_floor(&hi).div_floor(&lo);
            // gr < 1000 by construction, so the i64 cast is provably lossless.
            let gr = gr.to_i64().expect("group of three digits fits i64");
            groups.push((gr, pot));
            // Python: `if gr and not lowestgroup` — note `not lowestgroup` is
            // falsy for 0 too, but a stored lowestgroup is always non-zero
            // (it is only assigned under `if gr`), so `is_none()` is exact.
            if gr != 0 && lowestgroup.is_none() {
                lowestgroup = Some(gr);
            }
        }

        let mut result: Vec<Entry> = Vec::new();
        if negative {
            // MINUS_PREFIX_WORD is "meinws " but the word list hardcodes the
            // unspaced form; makestring supplies the join.
            result.push(("meinws", None));
        }

        for &(gr, pot) in groups.iter().rev() {
            if gr == 0 {
                continue;
            }
            let g = if pot == 6 {
                "fem" // mil (1000) is feminine
            } else if pot == 3 {
                gender // units depend on the following noun
            } else {
                "masc" // millions etc are masculine
            };
            // gr == 1 above the units group emits only the magnitude word:
            // 1000 is "mil", not "un mil"; 10**6 is "miliwn".
            if gr > 1 || pot == 3 {
                let words = hundred_group(gr, informal, g, ordinal && lowestgroup == Some(gr));
                result.extend(words);
            }
            if pot > 3 {
                result.push(million_word(pot - 3));
            }
        }

        // Python branches on `number < 100` here (the *absolute* value — the
        // sign was stripped in place):
        //
        //     if number < 100:  return makestring(result, counted=counted)
        //     else:             if counted: result.extend([("o", "SM"),
        //                                                  (counted, None)])
        //                       return makestring(result)
        //
        // The two arms differ only in where a `counted` noun lands: inside the
        // word list via the first OBJ, or appended as "o <noun>" ("of nouns",
        // the >99 partitive). `counted` is always None in the four in-scope
        // modes, so both arms collapse to the same call and the branch is
        // elided rather than duplicated.
        //
        // Under `raw=True` Python returns `result` *before* either arm, which
        // is where this function stops. `_money_verbose` then re-implements the
        // same "o <plural>" partitive by hand — see [`LangCy::money_verbose`].
        Ok(result)
    }

    /// Python's `Num2Word_CY.to_cardinal(number, informal, gender, ordinal,
    /// counted, raw)` with `raw=False`, `counted=None`.
    ///
    /// `counted` is out of scope (the dispatcher's Rust fast path only fires
    /// with no kwargs), so Python's `number < 100` branch collapses — see
    /// [`LangCy::to_cardinal_raw`]'s tail comment.
    fn to_cardinal_full(
        &self,
        number: &BigInt,
        informal: bool,
        gender: &str,
        ordinal: bool,
    ) -> Result<String> {
        let words = self.to_cardinal_raw(number, informal, gender, ordinal)?;
        Ok(makestring(&words, None))
    }

    /// Python's `Num2Word_CY.float_to_words`, reached from
    /// `to_cardinal(<float>)` (the `isinstance(number, float)` guard).
    ///
    /// ```python
    /// def float_to_words(self, float_number):
    ///     is_negative = float_number < 0
    ///     abs_float = abs(float_number)
    ///     prefix = self.to_cardinal(int(abs_float))
    ///     float_part = str(abs_float).split(".")[1]
    ///     postfix = " ".join([self.to_cardinal(int(c)) for c in float_part])
    ///     result = prefix + Num2Word_CY.FLOAT_INFIX_WORD + postfix
    ///     if is_negative:
    ///         result = "meinws " + result
    ///     return result
    /// ```
    ///
    /// This deliberately does **not** touch `base.float2tuple`: CY reads the
    /// fractional digits straight out of `str(abs_float)`, one character at a
    /// time. There is no `< 0.01` artefact heuristic and no banker's rounding —
    /// `2.675` renders `"chwech saith pump"` (6 7 5) because `str(2.675)` is
    /// literally `"2.675"`, not because the f64 was repaired.
    ///
    /// `precision` is the repr-derived fractional-digit count the shim computed
    /// (`abs(Decimal(str(value)).as_tuple().exponent)`). In the fixed-notation
    /// range it equals `len(str(abs_float).split(".")[1])`, so formatting
    /// `abs_float` to that many places reproduces the exact repr digits
    /// (verified byte-for-byte against the live interpreter over the corpus
    /// float set). Exponent-notation floats (`>= 1e16`, `< 1e-4`) instead take
    /// [`py_float_str`]'s e-form arm: `str(1e16)` is "1e+16", which has no '.'
    /// and raises **IndexError** on the split, while "1.5e+16" splits into a
    /// float_part of "5e+16" whose 'e' raises **ValueError** at `int(c)`.
    ///
    /// Note the order: `prefix = self.to_cardinal(int(abs_float))` runs
    /// *before* the split, so a float at/above `999 * 10**33` raises
    /// OverflowError first (#159).
    ///
    /// `precision_override` is ignored, exactly as Python ignores it here: CY's
    /// `pass` `__init__` never creates `self.precision`, so the dispatcher's
    /// `hasattr(converter, "precision")` guard is false and the kwarg is dropped
    /// (confirmed live: `num2words(2.675, lang="cy", precision=1)` is unchanged).
    fn float_to_words(&self, value: f64, precision: u32) -> Result<String> {
        // Python: `is_negative = float_number < 0` — the *value*, not the sign
        // bit, so -0.0 is NOT negative here ("dim pwynt dim", no meinws).
        let is_negative = value < 0.0;
        let abs_float = value.abs();

        // prefix = self.to_cardinal(int(abs_float)) — int() truncates toward
        // zero; a whole f64 converts exactly at any magnitude.
        let pre = BigInt::from_f64(abs_float.trunc()).expect("finite float truncates exactly");
        let prefix = self.to_cardinal_full(&pre, false, "masc", false)?;

        // float_part = str(abs_float).split(".")[1]
        let s = py_float_str(abs_float, precision);
        let float_part = match s.split_once('.') {
            Some((_, frac)) => frac.to_string(),
            // repr picked exponent form with an integral mantissa ("1e+16"):
            // no '.' → Python's [1] raises IndexError.
            None => return Err(N2WError::Index("list index out of range".into())),
        };

        // postfix = " ".join(to_cardinal(int(c)) for c in float_part)
        let mut parts: Vec<String> = Vec::new();
        for c in float_part.chars() {
            // Python `int(c)`: a non-digit ('e' from "1.5e+16") raises
            // ValueError with exactly this message.
            let d = c.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", c))
            })?;
            parts.push(self.to_cardinal_full(&BigInt::from(d), false, "masc", false)?);
        }
        let postfix = parts.join(" ");

        // result = prefix + FLOAT_INFIX_WORD (" pwynt ") + postfix.
        let result = format!("{} pwynt {}", prefix, postfix);
        // The negative prefix is the literal "meinws " prepended to the whole
        // string (float_to_words line 251), not routed through the word list.
        if is_negative {
            Ok(format!("meinws {}", result))
        } else {
            Ok(result)
        }
    }

    /// `float_to_words` for a fractional `Decimal`, from its exact digits
    /// (#156): the truncated integer part, " pwynt ", then one cardinal per
    /// fractional digit, "meinws " in front of a negative.
    fn decimal_to_words(&self, value: &FloatValue) -> Result<String> {
        let (pre, post) = float2tuple(value);
        let prefix = self.to_cardinal_full(&pre.abs(), false, "masc", false)?;
        let digits = format!("{:0>width$}", post, width = value.precision() as usize);
        let mut parts: Vec<String> = Vec::new();
        for c in digits.chars() {
            let d = c.to_digit(10).expect("float2tuple yields decimal digits");
            parts.push(self.to_cardinal_full(&BigInt::from(d), false, "masc", false)?);
        }
        let result = format!("{} pwynt {}", prefix, parts.join(" "));
        if strictly_negative(value) {
            Ok(format!("meinws {}", result))
        } else {
            Ok(result)
        }
    }

    /// Python's `Num2Word_CY.to_cardinal(<Decimal>)`.
    ///
    /// A `Decimal` is **not** a `float`, so the `isinstance(number, float)`
    /// guard is false and `float_to_words` is *not* used. The Decimal instead
    /// flows through the ordinary integer branch of `to_cardinal`, where the
    /// fractional part is floored away group-by-group
    /// (`(number % 10**pot) // 10**(pot-3)`), yet the sign, zero, ceiling and
    /// `< 100` tests all read the FULL Decimal value.
    ///
    /// The observable consequence is that a purely fractional Decimal renders as
    /// the empty string, not `"dim"`: `Decimal("0.5") == 0` is false, so the
    /// `"dim"` short-circuit is skipped and every digit group is zero, leaving
    /// an empty word list. Only an exact-zero Decimal yields `"dim"`. Negatives
    /// keep their `("meinws", None)` prefix even when the integer part is zero:
    /// `Decimal("-0.5")` is `"meinws"`.
    ///
    /// `ordinal` is `to_cardinal(..., ordinal=True)` — live when `to_ordinal`
    /// delegates a Decimal in 20..100 here (`to_ordinal(Decimal('42'))` →
    /// "ail a deugain"). `lowestgroup` then matters; it holds the *value* of
    /// the lowest non-zero group, exactly like the int path (module bug 6).
    fn decimal_to_cardinal(&self, value: &BigDecimal, ordinal: bool) -> Result<String> {
        let negative = value.is_negative();
        let absval = value.abs();

        // `if number == 0`: the full Decimal. Exact zero → "dim"; a fractional
        // value falls through to the (all-zero) group split → empty string.
        if absval.is_zero() {
            return Ok(makestring(CARDINAL_WORDS[0], None));
        }
        // `elif not number < 999 * 10**33`, on the full Decimal. An
        // OverflowError since gladiaio/num2words2#159.
        if absval >= BigDecimal::from(maxval_cy().clone()) {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                absval,
                maxval_cy()
            )));
        }

        // `(number % 10**pot) // 10**(pot-3)` on the Decimal equals the same
        // digit groups of the truncated integer part — the `//` floors the
        // fraction away (verified against the live Decimal arithmetic). So take
        // the groups from `pre = int(abs)` and reuse the integer machinery.
        let pre = absval.with_scale(0).as_bigint_and_exponent().0; // trunc toward zero (>= 0)

        let mut groups: Vec<(i64, u32)> = Vec::new();
        let mut lowestgroup: Option<i64> = None;
        for pot in [3u32, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36] {
            let hi = BigInt::from(10u32).pow(pot);
            let lo = BigInt::from(10u32).pow(pot - 3);
            let gr = pre.mod_floor(&hi).div_floor(&lo);
            let gr = gr.to_i64().expect("group of three digits fits i64");
            groups.push((gr, pot));
            if gr != 0 && lowestgroup.is_none() {
                lowestgroup = Some(gr);
            }
        }

        let mut result: Vec<Entry> = Vec::new();
        if negative {
            result.push(("meinws", None));
        }
        for &(gr, pot) in groups.iter().rev() {
            if gr == 0 {
                continue;
            }
            // g = gender ("masc") for pot==3, "fem" for pot==6 (mil), else
            // "masc" — pot==3 and the else-arm coincide at the default gender.
            let g = if pot == 6 { "fem" } else { "masc" };
            if gr > 1 || pot == 3 {
                let words = hundred_group(gr, false, g, ordinal && lowestgroup == Some(gr));
                result.extend(words);
            }
            if pot > 3 {
                result.push(million_word(pot - 3));
            }
        }

        // Python's `number < 100` branch differs only in `counted` placement,
        // which is None here, so both arms collapse to `makestring(result)`.
        Ok(makestring(&result, None))
    }

    /// Python's `Num2Word_CY.to_ordinal(number, informal=False, gender="masc")`
    /// with the gender threaded through — shared by the plain hook (masc) and
    /// [`Lang::to_ordinal_kw`].
    ///
    /// Only 20..=99 ever reaches `to_cardinal(..., ordinal=True)`: `< 20` is
    /// table-driven (and ignores `gender` — ORDINAL_WORDS is indexed
    /// directly, so `to_ordinal(2, gender="fem")` is "ail", not the dead
    /// ORDINAL_WORDS_FEM's entry), `== 100` is the literal "canfed", and
    /// `> 100` raises. That range is a single three-digit group, which is why
    /// module bug 6 (`lowestgroup` comparing values) can never fire in scope.
    fn to_ordinal_impl(&self, value: &BigInt, gender: &str) -> Result<String> {
        if value < &BigInt::from(20) {
            // Python: ORDINAL_WORDS[number]. Every negative satisfies `< 20`
            // and missed the dict (KeyError); reject it like Base's
            // verify_ordinal instead. Module bug 7, #158.
            verify_ordinal(value)?;
            // 0..=19 — always present.
            let idx = value.to_usize().expect("0..=19 fits usize");
            return Ok(makestring(ORDINAL_WORDS[idx], None));
        }
        if value == &BigInt::from(100) {
            return Ok("canfed".to_string());
        } else if value > &BigInt::from(100) {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        // Python passes informal=False literally here, discarding its own
        // `informal` parameter. Reproduced.
        self.to_cardinal_full(value, false, gender, true)
    }
}

impl Lang for LangCy {
    fn maxval(&self) -> &BigInt {
        maxval_cy()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "GBP"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    /// Never assigned in Python (`pass` `__init__`), so Base's currency and
    /// fraction paths raised AttributeError on a negative value. CY's own
    /// cardinal minus word (#157).
    fn negword(&self) -> &str {
        "meinws "
    }

    // cards() / maxval() / merge() stay at their trait defaults: Python never
    // builds self.cards for CY (see the module docs on the `pass` __init__),
    // so splitnum/clean/merge are unreachable.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Dispatcher calls to_cardinal(number) — all kwargs at their defaults.
        self.to_cardinal_full(value, false, "masc", false)
    }

    /// Python's `Num2Word_CY.to_ordinal(number, informal=False, gender="masc")`
    /// at its kwarg defaults — see [`LangCy::to_ordinal_impl`].
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.to_ordinal_impl(value, "masc")
    }

    // to_ordinal_num: inherited from Num2Word_Base, which returns the value
    // itself. The trait default's `Ok(value.to_string())` matches the corpus.

    // to_year: inherited from Num2Word_Base — `self.to_cardinal(value)`. The
    // trait default delegates through &self and picks up the override above.

    /// The float/Decimal cardinal path.
    ///
    /// CY does **not** override `to_cardinal_float` in Python — and it must not
    /// use the trait default here either. Python routes both float and Decimal
    /// input through `Num2Word_CY.to_cardinal(number)` (the dispatcher calls
    /// `converter.to_cardinal(number)` for `to="cardinal"`), where CY's own
    /// override branches on `isinstance(number, float)`:
    ///
    ///   * a `float` → [`LangCy::float_to_words`] — a bespoke path that reads the
    ///     fractional digits straight out of `str(abs_float).split(".")[1]` and
    ///     joins them with `" pwynt "`. It never touches `base.float2tuple`, so
    ///     there is no `< 0.01` artefact heuristic and no banker's rounding.
    ///   * a `Decimal` (not a `float`) → the ordinary integer branch, which
    ///     floors the fraction away group-by-group → [`LangCy::decimal_to_cardinal`].
    ///
    /// The base default `default_to_cardinal_float` would instead run
    /// `float2tuple` + `self.pointword`, and CY's `pass` `__init__` never assigns
    /// `pointword` — so the default is both semantically wrong (no `" pwynt "`
    /// infix) and would crash on the missing attribute. Hence this override.
    ///
    /// `precision_override` is dropped, exactly as Python drops it: the
    /// dispatcher only applies `precision=` when `hasattr(converter, "precision")`,
    /// and CY's `pass` `__init__` never creates `self.precision`, so the kwarg
    /// never reaches the conversion (confirmed live: `num2words(2.675, lang="cy",
    /// precision=1)` is unchanged). The repr-derived precision the shim already
    /// put in `FloatValue::Float` is what `float_to_words` uses.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.float_to_words(*value, *precision),
            // Python floored a fractional Decimal away (#156).
            FloatValue::Decimal { value: d, .. } if !d.is_integer() => self.decimal_to_words(value),
            FloatValue::Decimal { value, .. } => self.decimal_to_cardinal(value, false),
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// The base default's whole-value shortcut (`1.0` → integer path → "un")
    /// must not fire: CY's `isinstance(number, float)` guard routes *every*
    /// float through `float_to_words`, so `to_cardinal(1.0)` is
    /// "un pwynt dim", and a whole Decimal takes the integer branch (where it
    /// happens to equal the int rendering). Both live in
    /// [`LangCy::to_cardinal_float`]'s routing, so delegate wholesale.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    // year_float_entry: the trait default delegates to cardinal_float_entry,
    // which is exactly Python (`to_year` is Base's `self.to_cardinal(value)`),
    // so `to_year(1.0)` is "un pwynt dim" and `to_year(1e16)` IndexErrors.


    /// `to_ordinal(float/Decimal)` — Python's dict-lookup gauntlet.
    ///
    /// * A fractional value raises `errmsg_floatord`, a negative one
    ///   `errmsg_negord` (Python: KeyError; module bug 7, #158).
    /// * `number < 20` → `ORDINAL_WORDS[number]`, for a value that
    ///   hash-equals an int key 0..=19 (`5.0`, `Decimal('5.00')`, `-0.0`).
    /// * `number == 100` → "canfed" (`1E+2` too); `> 100` →
    ///   **NotImplementedError**.
    /// * else (20 ≤ n < 100) → `to_cardinal(number, ordinal=True)`: a float
    ///   hits the isinstance guard first and renders as a plain **cardinal**
    ///   ("ugain pwynt dim"); a Decimal reaches the integer branch with
    ///   `ordinal=True` live ("ail a deugain" for `Decimal('42')`).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // Whole and non-negative from here on (-0.0 is whole zero).
        let whole = verify_ordinal_float(value)?;
        // `if number < 20:`
        if whole < BigInt::from(20) {
            let idx = whole.to_usize().expect("0..=19 fits usize");
            return Ok(makestring(ORDINAL_WORDS[idx], None));
        }
        let (eq100, gt100) = match value {
            FloatValue::Float { value, .. } => (*value == 100.0, *value > 100.0),
            FloatValue::Decimal { value, .. } => {
                let hundred = BigDecimal::from(100);
                (*value == hundred, *value > hundred)
            }
        };
        if eq100 {
            return Ok("canfed".to_string());
        }
        if gt100 {
            return Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            ));
        }
        // return self.to_cardinal(number, informal=False, gender=gender,
        //                         ordinal=True)
        match value {
            FloatValue::Float { value, precision } => self.float_to_words(*value, *precision),
            FloatValue::Decimal { value, .. } => self.decimal_to_cardinal(value, true),
        }
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`. Inf/NaN parse
    /// fine here and blow up only later inside `to_cardinal`; the binding
    /// routes `ParsedNumber::Inf`/`NaN` to [`Lang::inf_result`] /
    /// [`Lang::nan_result`] below, which raise CY's exact exception types
    /// natively (no Python fallback).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `Decimal('-Infinity')`. CY's `to_cardinal`
    /// never reaches Base's `int(Decimal Inf)` (which OverflowErrors): the
    /// `not number < 999 * 10**33` ceiling fires first —
    /// `Decimal('Infinity') < 999*10**33` is False for both signs (the `-`
    /// case flips to `+Infinity` via `number = -number` before the ceiling),
    /// so it raises — an OverflowError since #159 (NotImplementedError in
    /// Python). `to_year` reaches the same raise; `to_ordinal` stops at its
    /// own `> 100` NotImplementedError first. `to_ordinal_num` would echo the
    /// repr, but no corpus row exercises it.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}Infinity",
                if negative { "-" } else { "" }
            )),
            "ordinal" => Err(N2WError::NotImplemented(
                "The given number is too large.".to_string(),
            )),
            _ => Err(N2WError::Overflow(format!(
                "abs(Infinity) must be less than {}.",
                maxval_cy()
            ))),
        }
    }

    /// `Decimal('NaN')`. CY's `to_cardinal` does `number < 0` on it, and a
    /// `Decimal('NaN')` comparison raises **decimal.InvalidOperation** before
    /// any rendering. Surfaced as the same class the binding re-raises.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaN".to_string()),
            _ => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".to_string(),
            }),
        }
    }

    /// `Decimal('-0.0')`. Unlike float `-0.0` (which CY's
    /// `isinstance(number, float)` guard sends through `float_to_words` →
    /// "dim pwynt dim"), a `Decimal('-0.0')` is not a `float`: `number == 0`
    /// is True, so `to_cardinal` short-circuits to the integer zero "dim".
    /// `to_year` is `to_cardinal`, so "dim" as well. `to_ordinal` (dict key
    /// 0 → "dimfed") and `to_ordinal_num` (echoes "-0.0") coincide with the
    /// float `-0.0` demotion, so they return `None` (the default path).
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            "cardinal" | "year" => Some(Ok("dim".to_string())),
            _ => None,
        }
    }

    // `to_fraction` is Base's (the trait default). Python's negative branch
    // read the never-assigned `self.negword` and raised AttributeError; with
    // `negword` defined it now says "meinws" (#157).

    // ---- grammatical kwargs ---------------------------------------------

    /// `to_cardinal(number, informal=False, gender="masc", ordinal=False,
    /// counted=None, raw=False)`.
    ///
    /// `ordinal=` in the bag is the *dispatcher's* named parameter: `num2words`
    /// consumes it before any converter method is chosen and a truthy value
    /// rewrites `to = "ordinal"`, so `num2words(n, lang="cy", ordinal=True,
    /// gender=...)` is `converter.to_ordinal(n, gender=...)` — never CY's own
    /// `to_cardinal(..., ordinal=True)` (which has no <20/100 gates and picks
    /// CARDINAL_WORDS for small values). The corpus pins the dispatcher
    /// behaviour, so a truthy `ordinal` reroutes to [`Lang::to_ordinal_kw`]
    /// with the remaining kwargs; a falsy one is dropped, exactly as the
    /// dispatcher never forwards it.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_cardinal(value);
        }
        if !kw.only(&["informal", "gender", "counted", "raw", "ordinal"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        if let Some(v) = kw.get("ordinal") {
            if kw_truthy(v) {
                // to = "ordinal": the rest of the bag rides to_ordinal. A
                // leftover counted=/raw= makes to_ordinal_kw decline, and the
                // Python original then raises its own TypeError.
                let rest: Vec<(String, KwVal)> = kw
                    .0
                    .iter()
                    .filter(|(k, _)| k != "ordinal")
                    .cloned()
                    .collect();
                return self.to_ordinal_kw(value, &Kwargs(rest));
            }
            // Falsy ordinal= keeps to="cardinal" and is simply not forwarded.
        }
        // informal: threaded through and never read (its only use site is
        // commented out). Any value accepted, any value ignored.
        let raw = kw.get("raw").map(kw_truthy).unwrap_or(false);
        let counted: Option<&str> = match kw.get("counted") {
            // counted=None is the signature default.
            None | Some(KwVal::None) => None,
            Some(KwVal::Str(s)) => Some(s.as_str()),
            // A non-str counted would crash inside softmutation's word[0]
            // arithmetic in ways not worth modelling — let Python own it.
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        // gender only ever matters as the literal `== "fem"` comparison:
        // "f", "m", None, 1 ... all behave as masculine.
        let gender = if kw.str("gender") == Some("fem") {
            "fem"
        } else {
            "masc"
        };

        let words = self.to_cardinal_raw(value, false, gender, false)?;
        if raw {
            // Python returns the list object; the corpus stringifies it.
            return Ok(py_repr_wordlist(&words));
        }
        if value.is_zero() {
            // Python's zero guard returns before `counted` is consulted:
            // to_cardinal(0, counted="ci") is plain "dim".
            return Ok(makestring(&words, None));
        }
        if value.abs() < BigInt::from(100) {
            // `counted` fills the first OBJ, landing *inside* the numeral:
            // "un ci ar hugain".
            let w: Vec<Word<'_>> = words.iter().map(|&(a, b)| (a, b)).collect();
            Ok(makestring(&w, counted))
        } else {
            // The >= 100 partitive: "o <noun>", with "o" soft-mutating the
            // noun ("cant o gi").
            let mut w: Vec<Word<'_>> = words.iter().map(|&(a, b)| (a, b)).collect();
            if let Some(c) = counted {
                w.push(("o", Some(SM)));
                w.push((c, None));
            }
            Ok(makestring(&w, None))
        }
    }

    /// `to_ordinal(number, informal=False, gender="masc")`.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["informal", "gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let gender = if kw.str("gender") == Some("fem") {
            "fem"
        } else {
            "masc"
        };
        self.to_ordinal_impl(value, gender)
    }

    /// `Num2Word_Base.to_year(value, **kwargs)` — the `**kwargs` swallows
    /// every keyword unread, so any bag is accepted and the cardinal is
    /// returned.
    fn to_year_kw(&self, value: &BigInt, _kw: &Kwargs) -> Result<String> {
        self.to_cardinal(value)
    }

    // to_ordinal_num_kw / to_currency_kw: neither Python method takes extra
    // kwargs, so the trait defaults (empty bag or Python's own TypeError via
    // fallback) are already exact.

    /// `to_cardinal(<float/Decimal>, **kwargs)`. A float hits the
    /// `isinstance(number, float)` guard before any kwarg is read, so
    /// informal/gender/counted/raw are all silently ignored and
    /// `float_to_words` renders as usual. A Decimal reaches the integer
    /// branch with the kwargs live — no corpus coverage, left to the Python
    /// fallback, which owns it.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.cardinal_float_entry(value, precision_override);
        }
        if !kw.only(&["informal", "gender", "counted", "raw"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        match value {
            FloatValue::Float { value, precision } => self.float_to_words(*value, *precision),
            FloatValue::Decimal { .. } => Err(N2WError::Fallback("kwargs".into())),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // CY overrides `to_currency`, `_money_verbose` and `_cents_verbose`.
    // `pluralize` comes from Num2Word_EUR; `to_cheque` and `_cents_terse` from
    // Num2Word_Base — the trait defaults already mirror both, so they are not
    // restated here. `currency_precision` likewise stays at its default 100:
    // CY's CURRENCY_PRECISION is Base's empty dict, so `.get(code, 100)` is
    // always 100.

    fn lang_name(&self) -> &str {
        "Num2Word_CY"
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
    /// would raise IndexError. All four CY entries carry two forms, so that is
    /// unreachable — mapped to `Index` rather than panicking so the exception
    /// type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Python's `Num2Word_CY._money_verbose`.
    ///
    /// Both arms ask `to_cardinal` for the **feminine** form unconditionally
    /// (module bug 12) and both attach the currency noun themselves — which is
    /// why `to_currency` no longer appends Base's `pluralize(left, cr1)` as
    /// well (bug 13, #162):
    ///
    /// * `number > 100` — the partitive: `... o <plural>`. "o" triggers a soft
    ///   mutation on the noun, so `1234.56 USD` gives "o **dd**olarau".
    /// * otherwise — the noun goes in as `counted`, landing at the first `OBJ`
    ///   and so *inside* the numeral ("pedwar **ceiniog** ar ddeg ar hugain").
    ///
    /// Note `> 100`, not `>= 100`: exactly 100 takes the `counted` arm.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        // Python indexes `self.CURRENCY_FORMS[currency]` bare here — a KeyError
        // if missing. Unreachable in practice: to_currency and to_cheque both
        // resolve the code (raising NotImplementedError) before calling this.
        let forms = self
            .currency_forms
            .get(currency)
            .ok_or_else(|| N2WError::Key(format!("'{}'", currency)))?;

        let raw = self.to_cardinal_raw(number, false, "fem", false)?;
        let mut m: Vec<Word<'_>> = raw.iter().map(|&(w, mu)| (w, mu)).collect();

        if number > &BigInt::from(100) {
            // CURRENCY_FORMS[currency][0][1] — the plural unit.
            let c = &forms.unit[1];
            m.push(("o", Some(SM)));
            m.push((c.as_str(), None));
            Ok(makestring(&m, None))
        } else {
            // CURRENCY_FORMS[currency][0][0] — the singular unit.
            let c = &forms.unit[0];
            Ok(makestring(&m, Some(c.as_str())))
        }
    }

    /// Python's `Num2Word_CY._cents_verbose`.
    ///
    /// Three quirks, all ported: zero cents return `""` (bug 14's source);
    /// `number == 1` drops the numeral entirely and emits only the noun
    /// (bug 11); and gender is left at its masculine default, unlike
    /// [`LangCy::money_verbose`].
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        // Python returns "" before it ever indexes CURRENCY_FORMS, so a missing
        // code would not KeyError at zero. Ordered the same way here.
        if number.is_zero() {
            return Ok(String::new());
        }
        let forms = self
            .currency_forms
            .get(currency)
            .ok_or_else(|| N2WError::Key(format!("'{}'", currency)))?;

        // `if number > 1: to_cardinal(number, raw=True) else: [(OBJ, None)]`.
        // The else arm also catches a hypothetical negative, which cannot occur:
        // Base hands cents through `abs()`.
        let m: Vec<Word<'_>> = if number > &BigInt::one() {
            let raw = self.to_cardinal_raw(number, false, "masc", false)?;
            raw.iter().map(|&(w, mu)| (w, mu)).collect()
        } else {
            vec![(OBJ, None)]
        };

        // CURRENCY_FORMS[currency][1][0] — the singular subunit.
        let c = &forms.subunit[0];
        Ok(makestring(&m, Some(c.as_str())))
    }

    /// Python's `Num2Word_CY.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="GBP", cents=True,
    ///                 separator=",", adjective=False):
    ///     if isinstance(val, int):
    ///         minus_str = "minws " if val < 0 else ""
    ///         money_str = self.to_cardinal(abs(val))
    ///         if abs(val) == 1: currency_str = "bunt"
    ///         else:             currency_str = "bunnoedd"
    ///         return "%s%s %s" % (minus_str, money_str, currency_str)
    ///     return super().to_currency(val, currency=currency, cents=cents,
    ///                                separator=separator, adjective=adjective)
    /// ```
    ///
    /// The int arm never reads `currency`, `cents`, `separator` or `adjective`,
    /// and never touches `CURRENCY_FORMS` — hence module bugs 8, 9 and 10.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`; resolve
        // it to CY's own default (",") before the ported body.
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(v) = val {
            // Python says "minws " here; the port uses CY's own "meinws ",
            // like the cardinal and the float path — bug 9 (#180).
            let minus_str = if v.is_negative() { "meinws " } else { "" };
            let abs_val = v.abs();
            // Raises OverflowError past 999 * 10**33 (#159).
            let money_str = self.to_cardinal(&abs_val)?;
            // Pre-mutated GBP literals, whatever `currency` says — bugs 8, 10.
            let currency_str = if abs_val.is_one() { "bunt" } else { "bunnoedd" };
            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // Floats/Decimals: `super().to_currency(...)` — Num2Word_Base's, minus
        // the generic nouns it appends (module bug 13, #162). Python's
        // `minus_str = "%s " % self.negword.strip()` raised AttributeError
        // (CY's `pass` `__init__` never set `negword`); the port's `negword`
        // is "meinws" (#157).
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let (d, has_decimal, is_float) = match val {
            CurrencyValue::Decimal { value, has_decimal, is_float } => {
                (value.clone(), *has_decimal, *is_float)
            }
            CurrencyValue::Int(_) => unreachable!("int branch returned above"),
        };
        // CY's CURRENCY_PRECISION is Base's empty dict: always 100.
        let scaled = &d * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();
        let (left, right, is_negative) = parse_currency_parts(
            &CurrencyValue::Decimal { value: d, has_decimal, is_float },
            false,
            has_fractional_cents,
            100,
        );
        let minus = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        // `_money_verbose` already names the unit, in the form the counted
        // numeral calls for ("un punt", "dwy bunt", "... o bunnoedd"). Base
        // then appended `pluralize(left, cr1)` again — "un punt punt" — so
        // that second noun is dropped; only `adjective=True`'s adjective is
        // kept from it ("un dolar US").
        let mut money = self.money_verbose(&left, currency)?;
        if adjective {
            if let Some(adj) = self.currency_adjectives.get(currency) {
                money = format!("{} {}", money, adj);
            }
        }
        let right_int = right.as_bigint_and_exponent().0;
        if !has_decimal && right_int.is_zero() {
            return Ok(format!("{}{}", minus, money));
        }
        if has_fractional_cents {
            // Base reads the cents through the float cardinal and names them
            // with the plural subunit — once, so nothing to drop here.
            let sub = forms.subunit.get(1).or_else(|| forms.subunit.first());
            return Ok(format!(
                "{}{}{} {} {}",
                minus,
                money,
                separator,
                self.cardinal_from_decimal(&right)?,
                sub.map(String::as_str).unwrap_or_default()
            ));
        }
        if cents {
            let cents_str = self.cents_verbose(&right_int, currency)?;
            // `_cents_verbose` names the subunit too ("hanner cant ceiniog",
            // or bare "ceiniog" for 1), so Base's `pluralize(right, cr2)` was a
            // second noun ("ceiniog ceiniogau"). Zero cents come back empty;
            // the plural noun then stays the only one (module bug 14).
            if !cents_str.is_empty() {
                return Ok(format!("{}{}{} {}", minus, money, separator, cents_str));
            }
        }
        let cents_str = if cents {
            String::new()
        } else {
            self.cents_terse(&right_int, currency)?
        };
        Ok(format!(
            "{}{}{} {} {}",
            minus,
            money,
            separator,
            cents_str,
            self.pluralize(&right_int, &forms.subunit)?
        ))
    }

    // cardinal_from_decimal: left at its default — fractional cents are out of
    // scope for this phase. See the crate-level note; CY diverges there and it
    // is called out in the port report.
}
