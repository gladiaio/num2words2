//! Port of `lang_BG.py` (Bulgarian).
//!
//! Shape: **self-contained**. `Num2Word_BG` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires: Python
//! never builds `self.cards` and never sets `self.MAXVAL`. `to_cardinal`,
//! `to_ordinal`, `to_ordinal_num` and `to_year` are all overridden outright
//! and drive `_int_to_word` recursively. Consequently `cards`/`merge` stay at
//! their trait defaults here, and Python has **no overflow check at all** —
//! `to_cardinal(10**600)` simply recurses and answers. On a large enough
//! integer the port's recursion overflowed the native stack, so it adds a
//! ceiling (gladiaio/num2words2#203): `maxval` is 10^12, where "милиард" would
//! start to stack (oddity 6), and every mode raises `OverflowError` from there.
//!
//! Nothing is inherited from `Num2Word_Base` in scope: all four modes are
//! overridden. `Num2Word_BG._setup` is dead code (it calls `super()._setup()`,
//! but `Num2Word_Base` defines only `setup()`, not `_setup()`, and `__init__`
//! calls `self.setup()` — the no-op base one). Were `_setup` ever invoked it
//! would raise `AttributeError`; nothing invokes it, so it is not modelled.
//! `is_title` is never set and `title()` is never called by BG's own
//! `to_cardinal`, so no title-casing path exists.
//!
//! # The float / Decimal cardinal path
//!
//! BG **overrides `to_cardinal`** and handles non-integers inline; it does
//! *not* override `to_cardinal_float`, and `Num2Word_Base.to_cardinal_float`
//! is unreachable because the dispatcher only ever calls `converter
//! .to_cardinal(number)` for `to="cardinal"`. So the trait's
//! [`Lang::to_cardinal_float`] hook is implemented against BG's own body
//! rather than delegating to `floatpath::default_to_cardinal_float`. The two
//! differ in five observable ways:
//!
//! 13. ~~**BG branches on `isinstance(n, float)`, so `Decimal` input never
//!     reaches the float branch at all**~~ — fixed (gladiaio/num2words2#156).
//!     Python fell through to the integer tail and silently **truncated**
//!     (`Decimal("12.345")` == "дванадесет", `Decimal("-1.5")` == "минус
//!     един"), and string input, parsed to a Decimal, did the same. A
//!     fractional Decimal now takes the same branch as a float, fed from
//!     `float2tuple`'s exact Decimal arm (so #603's trillion-scale digits
//!     survive): `Decimal("12.345")` == "дванадесет точка три четири пет".
//! 14. **The fractional digits use `self.ones[d]`, not `self.to_cardinal(d)`.**
//!     Base renders each post-decimal digit through `to_cardinal`, which for
//!     BG forces `masculine=True`; BG's own loop indexes the neuter `ones`
//!     table directly. They differ on exactly the two keys of
//!     `ones_masculine`: `3.14` == "три точка **едно** четири" (not "един")
//!     and `2.25` == "два точка **две** пет" (not "два"), while the integer
//!     part of `2.25` *is* the masculine "два". Same class, same digit, two
//!     spellings, one string apart.
//! 15. **The sign is taken from `n < 0`, not from Base's `value < 0 and
//!     pre == 0`.** BG writes `negword` and then `abs(pre)`, so it needs no
//!     special case for `int(-0.5) == 0`; `-0.5` == "минус нула точка пет" and
//!     `-12.34` == "минус дванадесет точка три четири" come out of the same
//!     branch. Note BG concatenates `self.negword` raw — the trailing space is
//!     the word separator — where Base does `self.negword.strip()` and joins.
//! 16. **`precision=` is accepted by the dispatcher and then thrown away.**
//!     `num2words(..., precision=N)` assigns `converter.precision = N` before
//!     the call, but BG's float branch calls `float2tuple` first, and
//!     `float2tuple` *reassigns* `self.precision` from `repr(value)` before
//!     anything reads it. So `num2words(1.005, lang="bg", precision=1)` is
//!     still "един точка нула нула пет". `precision_override` is accordingly
//!     ignored here — the one language-visible consequence of issue #580 not
//!     applying. (`self.precision` is instance state, but `float2tuple`
//!     always writes it before `to_cardinal` reads it, so nothing leaks
//!     between calls and a stateless port is exact.)
//! 17. **`except BaseException` swallows `float2tuple`'s OverflowError.**
//!     `float2tuple` computes `abs(value - pre) * 10**self.precision`, and
//!     CPython's `float * int` converts the exact int with `PyLong_AsDouble`,
//!     which raises `OverflowError("int too large to convert to float")` once
//!     `10**precision` passes ~1.8e308. `repr(5e-324)` is "5e-324", so its
//!     precision is 324 and the multiply raises — whereupon BG's blanket
//!     `except BaseException` re-runs `self._int_to_cardinal(int(n))` and
//!     answers **"нула"**. Verified live: `5e-324`, `-5e-324` and `1e-310` all
//!     give "нула", while `1e-308` (precision 308, the last one that fits)
//!     spells out all 308 digits. Modelled in [`base_float2tuple`].
//!
//! The `except BaseException` wrapper is otherwise inert: its body re-runs the
//! same `self._int_to_cardinal(int(n))` that raised, so `int(n)`'s own
//! failures propagate unchanged — `float("inf")` raises `OverflowError` and
//! `float("nan")` raises `ValueError`, both from the `n != int(n)` test itself.
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. Everything below is exactly what Python
//! emits, verified row-by-row against the frozen corpus:
//!
//! 1. **Ordinals of negatives lose the masculine form that cardinals keep.**
//!    `to_cardinal` strips the sign itself and recurses, so
//!    `to_cardinal(-42)` reaches `_int_to_cardinal(42)` → `masculine=True` →
//!    "минус четиридесет и два". But `_int_to_ordinal` calls
//!    `_int_to_cardinal(-42)` *with the sign still attached*, hitting that
//!    method's own `n < 0` branch, which calls `_int_to_word(-n)` **without**
//!    `masculine=True`. So the ordinal is built on "минус четиридесет и две"
//!    (neuter "две", not masculine "два") and yields
//!    "минус четиридесет и двети". Same for `to_ordinal(-1)` →
//!    "минус едноти" ("едно", not "един") and `to_ordinal(-21)` →
//!    "минус двадесет и едноти". The `n < 0` branch of `_int_to_cardinal` is
//!    reachable *only* through `_int_to_ordinal`; `to_cardinal` never reaches
//!    it. Modelled in [`LangBg::int_to_cardinal`].
//! 2. **Ordinals are formed by gluing a suffix onto the cardinal**, so
//!    non-table values read as run-ons rather than real Bulgarian:
//!    `to_ordinal(0)` == "нулати", `to_ordinal(10**12)` == "хиляда
//!    милиардати", `to_ordinal(2*10**6)` == "два милионати". Only the 28
//!    keys in `ordinals` (1..=20, 30..=90 by ten, 100, 1000) get a genuine
//!    ordinal word, plus 10**6 "милионен" (was "един милионти",
//!    gladiaio/num2words2#252), a final hundreds word ("двестотен",
//!    "хиляда петстотен"; was "двестати") and round thousands with a
//!    one-word multiplier ("двехиляден", "десетхиляден"; was "две
//!    хилядити", #259).
//! 3. **`to_ordinal_num` uses Python's floor modulo on negatives**, which
//!    flips the suffix versus a truncating `%`. `-999 % 10 == 1` in Python
//!    (not `-9`), so `to_ordinal_num(-999)` == "-999-ви"; `-42 % 10 == 8`, so
//!    `to_ordinal_num(-42)` == "-42-ми". Rust's `%` truncates, so this uses
//!    `mod_floor`. See [`LangBg::to_ordinal_num`].
//! 4. **`self.scale` and `self.ones_feminine` are dead tables.** `scale` maps
//!    100/1000/10**6/10**9/10**12 to singular/plural pairs but `_int_to_word`
//!    hardcodes every one of those words inline and never reads it. Nothing
//!    in the module ever passes `feminine=True`, so "една"/"две" are
//!    unreachable in the four in-scope modes. Both are kept below for
//!    structural fidelity and flagged as unreachable.
//! 5. **`to_year`'s "N хиляди" branch is dead.** It is guarded by
//!    `1000 <= n < 2000`, where `n // 1000` is always exactly 1, so the
//!    `thousands == 1` arm always wins and the `else` never runs. Kept
//!    verbatim.
//! 6. **The 100/1000 scale words carry no agreement.** Above 10**9 the
//!    billions count recurses through `_int_to_word` and is suffixed with a
//!    flat " милиарда", so in Python `10**15` == "един милион милиарда" and
//!    `10**21` == "хиляда милиарда милиарда" — the library has no numword
//!    above "милиард" and stacks it instead. The port stops at 10^12 (#203).
//!
//! # Currency
//!
//! `Num2Word_BG` overrides `to_currency` **wholesale** and inherits
//! `to_cheque` from `Num2Word_Base`. That split drives every choice below:
//!
//! 7. **The two surfaces raise different messages for the same missing code.**
//!    BG's own `to_currency` says `Currency "KWD" not implemented for
//!    "Num2Word_BG"`, while the inherited `to_cheque` says `Currency code
//!    "KWD" ...`. Verified against the live interpreter. This is why
//!    [`LangBg::to_currency`] cannot delegate to
//!    `currency::default_to_currency` — that helper emits Base's "code"
//!    wording.
//! 8. **BG never calls `pluralize`.** It inlines `cr1[0] if left == 1 else
//!    cr1[1]`, and the inherited `to_cheque` takes `cr1[-1]` unconditionally.
//!    So `pluralize` stays at the trait default (raise), exactly mirroring
//!    `Num2Word_Base.pluralize`, and is unreachable.
//! 9. **`adjective=` is accepted and never read.** BG's signature carries it,
//!    the body ignores it, and `CURRENCY_ADJECTIVES` is empty. So
//!    `to_currency(12.34, "USD", adjective=True)` == the `False` output.
//! 10. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** BG calls
//!    `parse_currency_parts` without a `divisor` and hardcodes `* 100` in its
//!    fractional-cents probe, so the divisor is always 100. BG's own
//!    `CURRENCY_PRECISION` is `{}` regardless, so the trait's default of 100
//!    is already right and Base's `divisor == 1` normalisation is dead here —
//!    **JPY is a 100-subunit currency in BG** ("нула йени и петдесет сена").
//! 11. **The gender of the unit word is never agreed with.** `money_str` comes
//!    from `_int_to_cardinal`, which forces `masculine=True`, so a feminine or
//!    neuter unit still gets a masculine numeral: `to_currency(1, "JPY")` ==
//!    "един йена" (not "една йена") and `to_currency(0.01, "GBP")` == "нула
//!    паунда и един пени" (not "едно пени"). Corpus-confirmed.
//! 12. **`CURRENCY_FORMS` is BG's own class attribute**, so — unlike the 16
//!    classes that read `Num2Word_EUR`'s shared dict — it is untouched by
//!    `Num2Word_EN.__init__`'s in-place mutation. The live interpreter
//!    confirms exactly five codes at runtime; no AUD/CAD/CHF/KWD/INR leak in,
//!    so every other ISO code raises NotImplementedError.
//!
//! # Errors
//!
//! Below the 10^12 ceiling none of the four integer modes can fail. Every
//! table lookup is guarded by a range check (`tens` is only indexed with
//! 10..=19 or a multiple of ten in 20..=90; `ones` only with 0..=9), and the
//! `try/except BaseException` wrappers in `to_cardinal`/`to_ordinal`/
//! `to_ordinal_num` only exist to catch `float`/`str` coercion failures that
//! integer input cannot trigger — their `except` bodies re-run the same call
//! and are unreachable here. So all four always return `Ok`.
//!
//! The currency surface can fail only on an unknown currency code, with the
//! two distinct `NotImplemented` messages described in oddity 7.

use crate::base::{check_maxval, pow10_big, year_float_error, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword`. Note the **trailing space**: BG's `to_cardinal` and
/// `_int_to_cardinal` concatenate it raw (`self.negword + ...`), unlike
/// `Num2Word_Base.to_cardinal` which does `"%s " % self.negword.strip()`.
const NEGWORD: &str = "минус ";

/// `self.pointword`. BG's `to_cardinal` splices it as `" " + self.pointword`
/// and never runs it through `title()` (Base does), which is a distinction
/// without a difference: `is_title` is false for BG.
const POINTWORD: &str = "точка";

/// `self.ones`, keys 0..=9. Index 0 ("нула") doubles as the zero word.
const ONES: [&str; 10] = [
    "нула", "едно", "две", "три", "четири", "пет", "шест", "седем", "осем", "девет",
];

/// `self.tens`. Keys are 10..=19 plus the multiples of ten 20..=90; every
/// other key is absent in Python, but `_int_to_word` can only ever produce
/// those, so no `KeyError` is reachable.
fn tens_word(n: u32) -> Option<&'static str> {
    Some(match n {
        10 => "десет",
        11 => "единадесет",
        12 => "дванадесет",
        13 => "тринадесет",
        14 => "четиринадесет",
        15 => "петнадесет",
        16 => "шестнадесет",
        17 => "седемнадесет",
        18 => "осемнадесет",
        19 => "деветнадесет",
        20 => "двадесет",
        30 => "тридесет",
        40 => "четиридесет",
        50 => "петдесет",
        60 => "шестдесет",
        70 => "седемдесет",
        80 => "осемдесет",
        90 => "деветдесет",
        _ => return None,
    })
}

/// `self.ordinals`. The only values that get a real ordinal word; everything
/// else falls through to the suffix-gluing branch of `_int_to_ordinal`.
fn ordinal_word(n: u32) -> Option<&'static str> {
    Some(match n {
        1 => "първи",
        2 => "втори",
        3 => "трети",
        4 => "четвърти",
        5 => "пети",
        6 => "шести",
        7 => "седми",
        8 => "осми",
        9 => "девети",
        10 => "десети",
        11 => "единадесети",
        12 => "дванадесети",
        13 => "тринадесети",
        14 => "четиринадесети",
        15 => "петнадесети",
        16 => "шестнадесети",
        17 => "седемнадесети",
        18 => "осемнадесети",
        19 => "деветнадесети",
        20 => "двадесети",
        30 => "тридесети",
        40 => "четиридесети",
        50 => "петдесети",
        60 => "шестдесети",
        70 => "седемдесети",
        80 => "осемдесети",
        90 => "деветдесети",
        100 => "стотен",
        1000 => "хиляден",
        _ => return None,
    })
}

/// Python's `if feminine and n in self.ones_feminine / elif masculine and n in
/// self.ones_masculine / else self.ones[n]` chain.
///
/// `ones_feminine`/`ones_masculine` hold only keys 1 and 2, so any other digit
/// falls through to `ones` regardless of the flags. `feminine` is never `true`
/// from any in-scope caller (see module docs, oddity 4).
fn ones_form(n: u32, masculine: bool, feminine: bool) -> &'static str {
    if feminine {
        // self.ones_feminine
        match n {
            1 => return "една",
            2 => return "две",
            _ => {}
        }
    }
    if masculine {
        // self.ones_masculine
        match n {
            1 => return "един",
            2 => return "два",
            _ => {}
        }
    }
    ONES[n as usize]
}

/// `Num2Word_BG.CURRENCY_FORMS`, verbatim from the class body.
///
/// BG declares its own class attribute rather than inheriting
/// `Num2Word_EUR`'s, so `Num2Word_EN.__init__`'s in-place mutation of the
/// shared EUR dict does **not** reach it. Confirmed against the live
/// interpreter: the runtime table is exactly these five codes. Note EUR keeps
/// the invariant `("евро", "евро")` here — that is BG's own literal, not the
/// EUR-module one English rewrites.
///
/// Every entry carries exactly two forms, which is what makes the direct
/// `cr[0]`/`cr[1]` indexing in [`index_form`] unreachable-by-IndexError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 2] = ["цент", "цента"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "BGN",
        CurrencyForms::new(&["лев", "лева"], &["стотинка", "стотинки"]),
    );
    m.insert("EUR", CurrencyForms::new(&["евро", "евро"], &CENTS));
    m.insert("USD", CurrencyForms::new(&["долар", "долара"], &CENTS));
    m.insert("GBP", CurrencyForms::new(&["паунд", "паунда"], &["пени", "пенса"]));
    m.insert("JPY", CurrencyForms::new(&["йена", "йени"], &["сен", "сена"]));
    m
}

/// Python's `cr[0] if n == 1 else cr[1]` on a `CURRENCY_FORMS` tuple.
///
/// BG indexes the tuple directly, so a one-form entry would raise IndexError.
/// All five BG entries have two forms, so that is unreachable — but the type
/// is mapped rather than panicked so the exception survives if the table ever
/// changes. This is *not* `pluralize`: BG never calls it (oddity 8).
fn index_form(forms: &[String], singular: bool) -> Result<&str> {
    forms
        .get(if singular { 0 } else { 1 })
        .map(String::as_str)
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// Python's `int(value)` on a `float`, including the two ways it fails.
///
/// BG reaches this through the `n != int(n)` test, and its `except
/// BaseException` handler re-runs `self._int_to_cardinal(int(n))` — the very
/// call that raised — so both exceptions propagate to the caller unchanged.
/// Live-checked: `to_cardinal(float("inf"))` raises OverflowError,
/// `to_cardinal(float("nan"))` raises ValueError, with these messages.
///
/// Not reachable through the shim as it stands (it derives `precision` from
/// `abs(Decimal(str(value)).as_tuple().exponent)` first, and `abs("F")` raises
/// TypeError on the Python side long before Rust is called), but modelled so
/// the port is a faithful whole.
fn python_int_of_float(value: f64) -> Result<BigInt> {
    if value.is_nan() {
        return Err(N2WError::Value("cannot convert float NaN to integer".into()));
    }
    if value.is_infinite() {
        return Err(N2WError::Overflow(
            "cannot convert float infinity to integer".into(),
        ));
    }
    // `trunc` on a finite double is exactly representable, so `from_f64` is
    // total here; it returns None only for NaN/inf, both ruled out above.
    Ok(BigInt::from_f64(value.trunc()).expect("finite f64 truncates exactly"))
}

/// `Num2Word_Base.float2tuple(value)`, float arm — with the OverflowError it
/// can raise made explicit, because BG's `except BaseException` catches it.
///
/// ```python
/// pre = int(value)
/// self.precision = abs(Decimal(str(value)).as_tuple().exponent)
/// post = abs(value - pre) * 10**self.precision
/// if abs(round(post) - post) < 0.01:
///     post = int(round(post))
/// else:
///     post = int(math.floor(post))
/// return pre, post
/// ```
///
/// `precision` is the shim's, i.e. exactly the `self.precision` this method
/// assigns — the port deliberately does not reimplement `repr(float)`.
///
/// Deliberately *not* `floatpath::float2tuple`, for two reasons, both
/// measured rather than assumed:
///
/// * That port scales by `10f64.powi(precision)`. Python multiplies by the
///   exact int `10**precision`, which CPython converts with `PyLong_AsDouble`
///   — correctly rounded. `powi` multiplies repeatedly and drifts an ulp off
///   the correctly-rounded value for **252 of the 309 exponents 0..=308**,
///   starting at `10**33` (`1.0000000000000001e33` vs `1e33`).
/// * Past `10**308` `powi` returns `inf` and the caller ends up stringifying a
///   saturated `i128`, where Python raises OverflowError — which BG catches
///   and turns into "нула" (oddity 17). A `Result` is the only way to say that.
///
/// Both divergences need `precision >= 33`, which no corpus row reaches; they
/// are fixed here anyway because BG must own the raise either way.
fn base_float2tuple(value: f64, precision: u32) -> Result<(BigInt, BigInt)> {
    let pre = python_int_of_float(value)?;

    // `float * int` → CONVERT_TO_DOUBLE → PyLong_AsDouble, which raises
    // rather than returning inf. num-bigint's `to_f64` *does* return
    // Some(inf), so the finiteness check is what reproduces the raise.
    let scale = match BigInt::from(10).pow(precision).to_f64() {
        Some(s) if s.is_finite() => s,
        _ => {
            return Err(N2WError::Overflow(
                "int too large to convert to float".into(),
            ))
        }
    };

    // `value - pre`: for a non-integral double `|value| < 2**52`, so `pre` is
    // exact in f64 and this is the same subtraction Python performs.
    let post = (value - value.trunc()).abs() * scale;

    // Python's round() is round-half-to-EVEN (`round(2.5) == 2`); Rust's
    // `f64::round()` is half-away-from-zero and would break every exact tie.
    let rounded = post.round_ties_even();
    let out = if (rounded - post).abs() < 0.01 {
        // The rescue arm: 2.675 really does give 674.9999999999998 here, and
        // this is what pulls it back to 675. Recomputing from the decimal
        // string would give the mathematically right — and so wrong — answer.
        rounded
    } else {
        post.floor()
    };
    Ok((
        pre,
        BigInt::from_f64(out).expect("finite scale keeps `out` finite"),
    ))
}

/// `abs(Decimal(str(f)).as_tuple().exponent)` for an f64.
///
/// Mirrors `floatpath::float_repr_precision`, which is private to that module.
/// Rust's `{}` for f64 is shortest-round-trip, the same contract as Python's
/// `repr`, so counting the digits after the point agrees. The two part company
/// only in exponent form (`repr(1e21)` is "1e+21" → precision 21, while Rust
/// prints all 22 digits → precision 0), which this cannot reach: its only
/// caller feeds it a subunit count in `[0, 100)`.
fn repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

pub struct LangBg {
    /// `CURRENCY_FORMS`, built once in [`LangBg::new`] and only ever read.
    /// Rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangBg {
    pub fn new() -> Self {
        LangBg {
            currency_forms: build_currency_forms(),
        }
    }

    /// `_int_to_word(n, masculine=False, feminine=False)`.
    ///
    /// Expects `n >= 0` — every caller strips the sign first. Recurses on the
    /// billions count, which is unbounded, so this is `BigInt` throughout.
    fn int_to_word(&self, n: &BigInt, masculine: bool, feminine: bool) -> String {
        if n.is_zero() {
            return ONES[0].to_string();
        }

        let mut parts: Vec<String> = Vec::new();
        let mut n = n.clone();

        let two = BigInt::from(2);
        let billion = BigInt::from(1_000_000_000u64);
        let million = BigInt::from(1_000_000u64);
        let thousand = BigInt::from(1000u32);
        let hundred = BigInt::from(100u32);

        // Handle billions. `billions` may itself exceed 10**9 — the library
        // has no numword above "милиард", so it recurses and stacks the word
        // (10**21 -> "хиляда милиарда милиарда"). The recursive call passes
        // neither flag, so the count is always neuter.
        if n >= billion {
            let billions = &n / &billion;
            if billions.is_one() {
                parts.push("един милиард".to_string());
            } else if billions == two {
                parts.push("два милиарда".to_string());
            } else {
                parts.push(format!(
                    "{} милиарда",
                    self.int_to_word(&billions, false, false)
                ));
            }
            n %= &billion;
        }

        // Handle millions. `n < 10**9` here, so `millions < 1000`.
        if n >= million {
            let millions = &n / &million;
            if millions.is_one() {
                parts.push("един милион".to_string());
            } else if millions == two {
                parts.push("два милиона".to_string());
            } else {
                parts.push(format!(
                    "{} милиона",
                    self.int_to_word(&millions, false, false)
                ));
            }
            n %= &million;
        }

        // Handle thousands. `n < 10**6` here, so `thousands < 1000`.
        if n >= thousand {
            let thousands = &n / &thousand;
            if thousands.is_one() {
                // Bare "хиляда" — no "един" prefix, unlike милион/милиард.
                parts.push("хиляда".to_string());
            } else if thousands == two {
                parts.push("две хиляди".to_string());
            } else {
                parts.push(format!(
                    "{} хиляди",
                    self.int_to_word(&thousands, false, false)
                ));
            }
            n %= &thousand;
        }

        // Handle hundreds. 100/200/300 are suppletive; 400..900 are
        // `ones[h] + "стотин"` (-> "четиристотин", "петстотин", ...).
        if n >= hundred {
            let hundreds = (&n / &hundred).to_u32().expect("hundreds is 1..=9");
            match hundreds {
                1 => parts.push("сто".to_string()),
                2 => parts.push("двеста".to_string()),
                3 => parts.push("триста".to_string()),
                _ => parts.push(format!("{}стотин", ONES[hundreds as usize])),
            }
            n %= &hundred;
        }

        // Tens and ones. `n < 100` from here, so a u32 is provably enough.
        let small = n.to_u32().expect("n < 100 after the hundreds step");
        if small >= 20 {
            let tens = (small / 10) * 10;
            parts.push(tens_word(tens).expect("multiple of ten in 20..=90").to_string());
            let unit = small % 10;
            if unit > 0 {
                // Bulgarian joins the last unit with "и" as a separate word.
                parts.push("и".to_string());
                parts.push(ones_form(unit, masculine, feminine).to_string());
            }
        } else if small >= 10 {
            parts.push(tens_word(small).expect("10..=19").to_string());
        } else if small > 0 {
            parts.push(ones_form(small, masculine, feminine).to_string());
        }

        parts.join(" ")
    }

    /// `_int_to_cardinal(n)`.
    ///
    /// The `n < 0` branch is reachable **only** via `_int_to_ordinal`
    /// (`to_cardinal` strips the sign before ever calling this), and it drops
    /// `masculine=True` — see module docs, oddity 1.
    fn int_to_cardinal(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_zero() {
            return Ok(ONES[0].to_string());
        }

        if n.is_negative() {
            // Note: no `masculine=True` here, unlike the positive path.
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&(-n), false, false)));
        }

        Ok(self.int_to_word(n, true, false))
    }

    /// `Num2Word_BG.to_ordinal(n)` fed a float/Decimal, resolved to its
    /// observable result.
    ///
    /// `_int_to_ordinal(n)` first probes `n in self.ordinals` — a whole
    /// float/Decimal hash-matches its int key (`5.0` → "пети",
    /// `Decimal("1E+2")` → "стотен") — and otherwise builds
    /// `_int_to_cardinal(n)`, whose dict lookups succeed for every whole
    /// value (`-1.0` → "минус едно" → "минус едноти") but raise KeyError on a
    /// fractional residue; `to_ordinal`'s bare `except BaseException` then
    /// retries as `_int_to_ordinal(int(n))`. Whole values produce the exact
    /// same words the truncated retry would, so the observable result is
    /// *always* `_int_to_ordinal(int(n))` — truncation toward zero.
    fn ordinal_numeric(&self, value: &FloatValue) -> Result<String> {
        let trunc = match value {
            FloatValue::Float { value, .. } => {
                BigInt::from_f64(value.trunc()).unwrap_or_else(BigInt::zero)
            }
            FloatValue::Decimal { value, .. } => value.with_scale(0).as_bigint_and_exponent().0,
        };
        self.int_to_ordinal(&trunc)
    }

    /// `Num2Word_BG._int_to_cardinal(n)` fed a whole float/Decimal (the
    /// `to_year` path; fractional values are rejected before this, #158).
    /// Note the negative branch drops `masculine=True`, so "-21.0" reads
    /// "минус двадесет и едно" — unlike the masculine int path.
    fn cardinal_year_numeric(&self, value: &FloatValue) -> Result<String> {
        let (neg, _, whole) = decompose_numeric(value);
        check_maxval(&whole, maxval_ceiling())?;

        // `if n == 0: return self.ones[0]` — numeric, so -0.0 lands here.
        if whole.is_zero() {
            return Ok(ONES[0].to_string());
        }
        if neg {
            // No masculine flag on the negative recursion (oddity 1).
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&whole, false, false)));
        }
        Ok(self.int_to_word(&whole, true, false))
    }

    /// `_int_to_ordinal(n)`: table lookup, else glue a suffix onto the
    /// cardinal.
    fn int_to_ordinal(&self, n: &BigInt) -> Result<String> {
        if let Some(word) = n.to_u32().and_then(ordinal_word) {
            return Ok(word.to_string());
        }

        let cardinal = self.int_to_cardinal(n)?;
        // 10**6 is "милионен", not "един милионти" (#252), as 1000 is
        // "хиляден".
        if cardinal == "един милион" {
            return Ok("милионен".to_string());
        }

        // A round number of thousands is one compound word on the
        // multiplier: "двехиляден", "петхиляден", "десетхиляден"; Python
        // glued "ти" onto the cardinal, "две хилядити" (#259).
        if let Some(mult) = cardinal.strip_suffix(" хиляди") {
            if !mult.contains(' ') {
                return Ok(format!("{}хиляден", mult));
            }
        }
        // A final hundreds word takes its own ordinal: "двестотен",
        // "петстотен", "хиляда двестотен" — not "двестати" (#259).
        let (head, last) = match cardinal.rsplit_once(' ') {
            Some((h, l)) => (Some(h), l),
            None => (None, cardinal.as_str()),
        };
        let hundreds = match last {
            "сто" => Some("стотен".to_string()),
            "двеста" => Some("двестотен".to_string()),
            "триста" => Some("тристотен".to_string()),
            _ => last.strip_suffix("стотин").map(|m| format!("{}стотен", m)),
        };
        if let Some(word) = hundreds {
            return Ok(match head {
                Some(h) => format!("{} {}", h, word),
                None => word,
            });
        }

        // Python slices `cardinal[:-4]` / `[:-3]` *by character*. Since the
        // guard is `endswith(suffix)` and each suffix is exactly that many
        // characters ("един" = 4, "два"/"три" = 3), dropping the suffix is
        // identical to dropping the character count — `strip_suffix` is
        // exactly faithful and cannot split a UTF-8 boundary.
        Ok(if let Some(stem) = cardinal.strip_suffix("един") {
            format!("{}първи", stem)
        } else if let Some(stem) = cardinal.strip_suffix("два") {
            format!("{}втори", stem)
        } else if let Some(stem) = cardinal.strip_suffix("три") {
            format!("{}трети", stem)
        } else if cardinal.ends_with('т') {
            format!("{}и", cardinal)
        } else {
            format!("{}ти", cardinal)
        })
    }

    /// `Num2Word_BG.to_cardinal(n)` where Python's `n` is a **float**.
    ///
    /// ```python
    /// try:
    ///     if isinstance(n, float) and n != int(n):
    ///         pre, post = self.float2tuple(n)
    ///         if n < 0:
    ///             result = self.negword
    ///             pre = abs(pre)
    ///         else:
    ///             result = ""
    ///         result += self._int_to_cardinal(pre)
    ///         if self.precision > 0:
    ///             result += " " + self.pointword
    ///             post_str = str(post)
    ///             post_str = "0" * (self.precision - len(post_str)) + post_str
    ///             for digit in post_str:
    ///                 result += " " + self.ones[int(digit)]
    ///         return result.strip()
    ///     if n < 0:
    ///         return self.negword + self.to_cardinal(-n)
    ///     return self._int_to_cardinal(int(n))
    /// except BaseException:
    ///     return self._int_to_cardinal(int(n))
    /// ```
    ///
    /// `precision` is `self.precision` as `float2tuple` would have set it, and
    /// the caller-supplied `precision=` kwarg is *not* it — see oddity 16.
    fn to_cardinal_f64(&self, n: f64, precision: u32) -> Result<String> {
        // `n != int(n)` evaluates `int(n)` first, so inf/nan raise right here
        // and the `except` body raises the same thing again (oddity 17).
        let pre_int = python_int_of_float(n)?;

        // `isinstance(n, float) and n != int(n)`. NaN is already gone, so this
        // is just "has a fractional part".
        if n != n.trunc() {
            let (pre, post) = match base_float2tuple(n, precision) {
                Ok(t) => t,
                // `except BaseException: return self._int_to_cardinal(int(n))`.
                // Reachable: precision >= 309 (i.e. |n| < 1e-308) overflows the
                // `10**precision` float cast, and every such n truncates to 0,
                // so this answers "нула" — sign and all detail discarded.
                Err(_) => return Ok(self.int_to_cardinal(&pre_int)?),
            };

            return self.fraction_words(n < 0.0, pre, post, precision);
        }

        // "For integers" — an integral float such as 1.0 or -3.0.
        if n < 0.0 {
            // `self.negword + self.to_cardinal(-n)`: the recursion re-enters
            // the positive path, so the magnitude *is* masculine. `-n` is
            // integral too, so this bottoms out immediately.
            return Ok(format!("{}{}", NEGWORD, self.to_cardinal_f64(-n, precision)?));
        }
        Ok(self.int_to_cardinal(&pre_int)?)
    }

    /// The fractional branch of BG's `to_cardinal`, from `float2tuple`'s
    /// `(pre, post)`. Shared by the float and (since #156) Decimal paths.
    fn fraction_words(
        &self,
        negative: bool,
        pre: BigInt,
        post: BigInt,
        precision: u32,
    ) -> Result<String> {
        // `if n < 0` — not Base's `value < 0 and pre == 0`. BG writes the
        // sign itself and abs()es `pre`, which covers int(-0.5) == 0 for
        // free. `-0.0 < 0` is false in Python and in Rust alike.
        let (mut result, pre) = if negative {
            (NEGWORD.to_string(), pre.abs())
        } else {
            (String::new(), pre)
        };

        // `pre` is now >= 0, so this takes `_int_to_cardinal`'s positive
        // path: the integer part is masculine ("два точка две пет").
        result.push_str(&self.int_to_cardinal(&pre)?);

        // Always true for a non-integral double — `repr` only goes to
        // exponent form at >= 1e16, where every double is integral — but
        // Python tests it, so the port does too.
        if precision > 0 {
            result.push(' ');
            result.push_str(POINTWORD);

            // `"0" * (self.precision - len(post_str)) + post_str`. Python's
            // `str * negative` is "", so an over-long post is left alone.
            let mut post_str = post.to_string();
            let len = post_str.chars().count();
            if (precision as usize) > len {
                post_str = format!("{}{}", "0".repeat(precision as usize - len), post_str);
            }

            // `for digit in post_str` — every character, *not* Base's
            // `for i in range(self.precision)`. Identical whenever post
            // fits its precision, which is every case I could find.
            for ch in post_str.chars() {
                let d = ch
                    .to_digit(10)
                    .expect("float2tuple yields a non-negative decimal integer");
                result.push(' ');
                // `self.ones[int(digit)]` — the NEUTER table, where Base
                // would recurse into to_cardinal and get masculine
                // "един"/"два" for 1 and 2 (oddity 14).
                result.push_str(ONES[d as usize]);
            }
        }
        Ok(result.trim().to_string())
    }

    /// `Num2Word_BG.to_cardinal(n)` where Python's `n` is a **Decimal**.
    ///
    /// The float branch is guarded by `isinstance(n, float)`, and a Decimal is
    /// not one, so Python truncated every Decimal here (oddity 13). Since
    /// #156 only *whole* Decimals reach this; fractional ones take
    /// [`LangBg::fraction_words`]:
    ///
    /// ```python
    /// if n < 0:
    ///     return self.negword + self.to_cardinal(-n)
    /// return self._int_to_cardinal(int(n))
    /// ```
    fn to_cardinal_bigdecimal(&self, n: &BigDecimal) -> Result<String> {
        if n.is_negative() {
            // `-n` is positive, so the recursion lands on the tail below and
            // the magnitude keeps its masculine agreement. Decimal("-0.0") is
            // *not* < 0 in Python, and BigDecimal has no signed zero either,
            // so both answer plain "нула".
            return Ok(format!("{}{}", NEGWORD, self.to_cardinal_bigdecimal(&-n.clone())?));
        }
        // `int(n)` truncates toward zero; so does `with_scale(0)`.
        self.int_to_cardinal(&n.with_scale(0).as_bigint_and_exponent().0)
    }
}

impl Default for LangBg {
    fn default() -> Self {
        Self::new()
    }
}

/// `(numerically negative, has fraction, absolute truncated magnitude)` for
/// either `FloatValue` arm. "Negative" is a strict `< 0`, so `-0.0` is not.
fn decompose_numeric(value: &FloatValue) -> (bool, bool, BigInt) {
    match value {
        FloatValue::Float { value: v, .. } => {
            let neg = *v < 0.0;
            let frac = v.fract() != 0.0;
            let whole = BigInt::from_f64(v.trunc())
                .unwrap_or_else(BigInt::zero)
                .abs();
            (neg, frac, whole)
        }
        FloatValue::Decimal { value: d, .. } => {
            let neg = d.is_negative();
            let frac = !d.is_integer();
            let whole = d.abs().with_scale(0).as_bigint_and_exponent().0;
            (neg, frac, whole)
        }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// милиард (10^9), so from 10^12 the module would stack it ("милиарда
/// милиарда").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

impl Lang for LangBg {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "BGN"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " и"
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // The float/str branches of Python's `to_cardinal` are out of scope
        // (integer input only), leaving the sign split and `_int_to_cardinal`.
        if value.is_negative() {
            // `self.negword + self.to_cardinal(-n)`: the recursion re-enters
            // the positive path, so the magnitude *is* masculine here.
            return Ok(format!("{}{}", NEGWORD, self.int_to_cardinal(&(-value))?));
        }
        Ok(self.int_to_cardinal(value)?)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.int_to_ordinal(value)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // Python's `%` floors toward negative infinity; Rust's truncates.
        // `mod_floor` restores Python semantics, which is load-bearing for
        // negatives: -999 % 10 == 1 -> "-999-ви", -42 % 10 == 8 -> "-42-ми".
        let m100 = value.mod_floor(&BigInt::from(100)).to_u32().unwrap_or(0);
        if (11..=19).contains(&m100) {
            return Ok(format!("{}-ти", value));
        }

        let m10 = value.mod_floor(&BigInt::from(10)).to_u32().unwrap_or(0);
        let suffix = match m10 {
            1 => "-ви",
            2 => "-ри",
            7 | 8 => "-ми",
            _ => "-ти",
        };
        Ok(format!("{}{}", value, suffix))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        let thousand = BigInt::from(1000u32);
        let two_thousand = BigInt::from(2000u32);

        if value < &thousand {
            // Also the path for negatives (BC years): `_int_to_cardinal`
            // handles the sign, dropping masculine agreement as ever.
            return Ok(self.int_to_cardinal(value)?);
        }

        if value < &two_thousand {
            // 1000..=1999 -> "хиляда ..." with the millennium spelled out.
            let thousands = value / &thousand;
            let remainder = value % &thousand;
            let mut result = if thousands.is_one() {
                "хиляда".to_string()
            } else {
                // Dead: `thousands` is provably 1 for 1000..=1999.
                format!("{} хиляди", self.int_to_cardinal(&thousands)?)
            };
            if remainder.is_positive() {
                result.push(' ');
                result.push_str(&self.int_to_cardinal(&remainder)?);
            }
            return Ok(result);
        }

        // 2000 and up: plain cardinal ("две хиляди двадесет и пет").
        Ok(self.int_to_cardinal(value)?)
    }

    /// `to_ordinal(float/Decimal)` — see [`LangBg::ordinal_numeric`]: the
    /// observable result is always `_int_to_ordinal(int(n))`, with the
    /// negative cardinal's neuter "едно" quirk ("минус едноти") intact.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.ordinal_numeric(value)
    }

    /// `to_ordinal_num(float/Decimal)`: `str(n)` + a suffix picked by
    /// `n % 100` / `n % 10` — with Python's per-type `%` semantics. A float
    /// modulo is floored (so `-2.0 % 10 == 8.0` → "-ми"), a Decimal's is
    /// truncated (so `Decimal("-3.0") % 10 == -3` → the default "-ти"), and
    /// any fractional value matches nothing → "-ти".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let suffix = match value {
            FloatValue::Float { value: v, .. } => {
                if v.fract() != 0.0 || !v.is_finite() {
                    "-ти"
                } else {
                    let i = BigInt::from_f64(*v).unwrap_or_else(BigInt::zero);
                    let m100 = i.mod_floor(&BigInt::from(100)).to_u32().unwrap_or(0);
                    let m10 = i.mod_floor(&BigInt::from(10)).to_u32().unwrap_or(0);
                    if (11..=19).contains(&m100) {
                        "-ти"
                    } else {
                        match m10 {
                            1 => "-ви",
                            2 => "-ри",
                            7 | 8 => "-ми",
                            _ => "-ти",
                        }
                    }
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                if !d.is_integer() {
                    "-ти"
                } else {
                    // Decimal % truncates (sign of the dividend), matching
                    // BigInt's `%`; a negative remainder matches nothing.
                    let i = d.with_scale(0).as_bigint_and_exponent().0;
                    let m100 = &i % BigInt::from(100);
                    let m10 = &i % BigInt::from(10);
                    if m100 >= BigInt::from(11) && m100 <= BigInt::from(19) {
                        "-ти"
                    } else if m10 == BigInt::one() {
                        "-ви"
                    } else if m10 == BigInt::from(2) {
                        "-ри"
                    } else if m10 == BigInt::from(7) || m10 == BigInt::from(8) {
                        "-ми"
                    } else {
                        "-ти"
                    }
                }
            }
        };
        Ok(format!("{}{}", repr_str, suffix))
    }

    /// `to_year(float/Decimal)` — `Num2Word_BG.to_year` has **no** exception
    /// net, so in Python the KeyError `_int_to_word` raised on a fractional
    /// residue propagated (`0.5` → KeyError). The port raises a `TypeError`
    /// instead, in en's `to='year'` wording (gladiaio/num2words2#158). Whole
    /// values ride the dict lookups to the same words as their int
    /// counterparts — including the neuter "минус едно" negatives ("-21.0" →
    /// "минус двадесет и едно").
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int().is_none() {
            return Err(year_float_error(value));
        }
        let (neg, _, whole) = decompose_numeric(value);

        // `if n < 1000` — every negative and every value under 1000.
        if neg || whole < BigInt::from(1000) {
            return self.cardinal_year_numeric(value);
        }
        if whole < BigInt::from(2000) {
            // thousands = n // 1000 == 1 for this whole range → "хиляда".
            let mut result = "хиляда".to_string();
            let remainder = &whole - BigInt::from(1000);
            if remainder.is_positive() {
                result.push(' ');
                result.push_str(&self.int_to_cardinal(&remainder)?);
            }
            return Ok(result);
        }
        self.cardinal_year_numeric(value)
    }

    // ---- float / Decimal ------------------------------------------------

    /// The non-integer half of `Num2Word_BG.to_cardinal`.
    ///
    /// Not `floatpath::default_to_cardinal_float`: BG overrides `to_cardinal`
    /// and inlines its own float handling, and the dispatcher only ever calls
    /// `converter.to_cardinal(number)` — `Num2Word_Base.to_cardinal_float` is
    /// unreachable for BG. See oddities 13-17 for the five ways the two differ.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored
    /// because Python ignores it: the dispatcher stores it on
    /// `converter.precision`, and `float2tuple` overwrites that from
    /// `repr(value)` before BG's `if self.precision > 0` ever reads it.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.to_cardinal_f64(*value, *precision),
            // Python's `isinstance(n, float)` was False → truncated (oddity
            // 13, #156). A fractional Decimal now reads like a float, exactly.
            FloatValue::Decimal { value: d, precision } if !d.is_integer() => {
                let (pre, post) = float2tuple(value);
                self.fraction_words(d.is_negative(), pre, post, *precision)
            }
            FloatValue::Decimal { value, .. } => self.to_cardinal_bigdecimal(value),
        }
    }

    /// The fractional-cents hook, reached from [`LangBg::to_currency`] as
    /// Python's `self.to_cardinal(float(right))`.
    ///
    /// Overridden because the default routes to *Base's* float path, and BG's
    /// is different in the ways listed above — most visibly the neuter
    /// `self.ones[d]` for the post-decimal digits (oddity 14). With the
    /// default, `to_currency(1.011, "USD")` rendered "…и един точка **един**
    /// цента" where Python says "…и един точка **едно** цента". The previous
    /// revision measured and flagged exactly this divergence and deferred it
    /// to the float phase; this is that fix. No corpus row reaches it — every
    /// `bg` currency arg has <= 2 decimals, so `has_fractional_cents` is
    /// always false — so it is pinned by [`tests::currency_fractional_cents`]
    /// against the live interpreter instead.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        // Python casts to float *first* (`float(right)`), so the port does too
        // rather than staying in arbitrary precision: the digits can differ.
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        self.to_cardinal_f64(f, repr_precision(f))
    }

    // ---- currency -------------------------------------------------------
    //
    // BG overrides `to_currency` outright and inherits everything else from
    // `Num2Word_Base`. So only `lang_name`, `currency_forms` and
    // `to_currency` are overridden here:
    //
    // * `currency_precision` — BG's CURRENCY_PRECISION is `{}`, so Base's
    //   `.get(code, 100)` is the trait default already.
    // * `currency_adjective` — CURRENCY_ADJECTIVES is `{}` (and unread).
    // * `pluralize` — never called; the default's raise mirrors Base's.
    // * `money_verbose` — Base's `_money_verbose` is `self.to_cardinal(n)`,
    //   which is the trait default and correctly re-enters BG's `to_cardinal`.
    // * `cents_verbose` / `cents_terse` — BG's `to_currency` inlines both and
    //   never calls them; unreachable.
    // * `to_cheque` — inherited verbatim; `default_to_cheque` already matches,
    //   down to Base's "Currency code" wording (oddity 7).

    fn lang_name(&self) -> &str {
        "Num2Word_BG"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_BG.to_currency`.
    ///
    /// Deliberately does **not** delegate to `currency::default_to_currency`:
    /// BG's body differs from Base's in the error wording (oddity 7), in
    /// keying the cents segment on `isinstance(val, int)` rather than Base's
    /// `has_decimal` guard, in inlining pluralization instead of calling
    /// `pluralize`, and in hardcoding divisor 100 (oddity 10).
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
        // resolve it through this language's own default (" и").
        let separator = separator.unwrap_or(self.default_separator());

        // `is_integer_input = isinstance(val, int)`. BG keys the entire cents
        // segment on this alone and never consults `has_decimal`, so a float
        // always prints cents — including `1.0` -> "един евро и нула цента".
        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // `has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0`.
        // Python's Decimal `%` truncates toward zero, as does `with_scale(0)`,
        // so the two agree on negatives: -12.34 gives `Decimal("-0.00")`,
        // which is falsy, exactly like the zero difference here.
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
        let minus_str = if is_negative { "минус " } else { "" };
        // `left` is always >= 0 (`parse_currency_parts` abs()es it), so this
        // takes `_int_to_cardinal`'s positive path: units are masculine, with
        // no agreement against the unit word's gender (oddity 11).
        let money_str = self.int_to_cardinal(&left)?;
        let currency_str = index_form(cr1, left.is_one())?;

        // For integers, don't show cents.
        if is_integer_input {
            return Ok(format!("{}{} {}", minus_str, money_str, currency_str));
        }

        // For floats, always show cents (even if zero).
        let right_int = right.as_bigint_and_exponent().0;
        let cents_str = if cents {
            if has_fractional_cents {
                // `self.to_cardinal(float(right)) if right > 0 else self.ones[0]`.
                //
                // `cardinal_from_decimal` is overridden above to route to BG's
                // own float path, so this now matches. It did not before: the
                // trait default goes to Base's `to_cardinal_float`, which
                // renders each post-decimal digit via `to_cardinal` ->
                // `_int_to_word(d, masculine=True)`, disagreeing on exactly
                // the two keys of `ones_masculine`:
                //   py:  to_currency(1.011, "USD") == "един долар и един точка едно цента"
                //   was: ...                          "един долар и един точка един цента"
                // No corpus row reaches this branch — every `bg` currency arg
                // has <= 2 decimals, so `has_fractional_cents` is always false
                // — so `tests::currency_fractional_cents` pins it instead.
                if right.is_positive() {
                    self.cardinal_from_decimal(&right)?
                } else {
                    ONES[0].to_string()
                }
            } else if right_int.is_positive() {
                self.int_to_cardinal(&right_int)?
            } else {
                ONES[0].to_string()
            }
        } else {
            // `str(float(right) if isinstance(right, Decimal) else right)`.
            // Note there is no zero padding: 0.05 gives "5", where Base's
            // `_cents_terse` would give "05".
            if has_fractional_cents {
                // `str(float(Decimal))`. `right` is a cent count in [0, 100),
                // so Rust's shortest-round-trip `{}` and Python's `repr`
                // agree; neither reaches exponent form.
                let f = right
                    .to_f64()
                    .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", right)))?;
                format!("{}", f)
            } else {
                right_int.to_string()
            }
        };

        // `cr2[0] if right == 1 else cr2[1]`. Python compares the *Decimal* in
        // the fractional case, so this compares `BigDecimal`: at 1.011 USD,
        // right is Decimal("1.100") — i.e. 1.1, not 1 — and takes the plural.
        let cents_currency = index_form(cr2, right == BigDecimal::one())?;

        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str, money_str, currency_str, separator, cents_str, cents_currency
        ))
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// A Python `int` arg (corpus `arg` with no dot).
    fn int(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, true, false, false).unwrap()
    }

    /// A Python `float` arg. `has_decimal` is always true for a float repr —
    /// and BG ignores the flag entirely (it keys on `isinstance(val, int)`),
    /// which `float_ignores_has_decimal` pins.
    fn float(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, false, true, true).unwrap()
    }

    /// Every `"lang": "bg", "to": "currency:*"` row of the frozen corpus that
    /// succeeds, verbatim.
    #[test]
    fn currency_corpus_rows() {
        let bg = LangBg::new();
        let rows: Vec<(&str, CurrencyValue, &str)> = vec![
            ("EUR", int("0"), "нула евро"),
            ("EUR", int("1"), "един евро"),
            ("EUR", int("2"), "два евро"),
            ("EUR", int("100"), "сто евро"),
            ("EUR", float("12.34"), "дванадесет евро и тридесет и четири цента"),
            ("EUR", float("0.01"), "нула евро и един цент"),
            ("EUR", float("1.0"), "един евро и нула цента"),
            ("EUR", float("99.99"), "деветдесет и девет евро и деветдесет и девет цента"),
            ("EUR", float("1234.56"), "хиляда двеста тридесет и четири евро и петдесет и шест цента"),
            ("EUR", float("-12.34"), "минус дванадесет евро и тридесет и четири цента"),
            ("EUR", int("1000000"), "един милион евро"),
            ("EUR", float("0.5"), "нула евро и петдесет цента"),
            ("USD", int("0"), "нула долара"),
            ("USD", int("1"), "един долар"),
            ("USD", int("2"), "два долара"),
            ("USD", int("100"), "сто долара"),
            ("USD", float("12.34"), "дванадесет долара и тридесет и четири цента"),
            ("USD", float("0.01"), "нула долара и един цент"),
            ("USD", float("1.0"), "един долар и нула цента"),
            ("USD", float("99.99"), "деветдесет и девет долара и деветдесет и девет цента"),
            ("USD", float("1234.56"), "хиляда двеста тридесет и четири долара и петдесет и шест цента"),
            ("USD", float("-12.34"), "минус дванадесет долара и тридесет и четири цента"),
            ("USD", int("1000000"), "един милион долара"),
            ("USD", float("0.5"), "нула долара и петдесет цента"),
            ("GBP", int("0"), "нула паунда"),
            ("GBP", int("1"), "един паунд"),
            ("GBP", int("2"), "два паунда"),
            ("GBP", int("100"), "сто паунда"),
            ("GBP", float("12.34"), "дванадесет паунда и тридесет и четири пенса"),
            ("GBP", float("0.01"), "нула паунда и един пени"),
            ("GBP", float("1.0"), "един паунд и нула пенса"),
            ("GBP", float("99.99"), "деветдесет и девет паунда и деветдесет и девет пенса"),
            ("GBP", float("1234.56"), "хиляда двеста тридесет и четири паунда и петдесет и шест пенса"),
            ("GBP", float("-12.34"), "минус дванадесет паунда и тридесет и четири пенса"),
            ("GBP", int("1000000"), "един милион паунда"),
            ("GBP", float("0.5"), "нула паунда и петдесет пенса"),
            // JPY is a *100*-subunit currency in BG: `to_currency` hardcodes
            // divisor 100 and never reads CURRENCY_PRECISION (oddity 10).
            ("JPY", int("0"), "нула йени"),
            ("JPY", int("1"), "един йена"),
            ("JPY", int("2"), "два йени"),
            ("JPY", int("100"), "сто йени"),
            ("JPY", float("12.34"), "дванадесет йени и тридесет и четири сена"),
            ("JPY", float("0.01"), "нула йени и един сен"),
            ("JPY", float("1.0"), "един йена и нула сена"),
            ("JPY", float("99.99"), "деветдесет и девет йени и деветдесет и девет сена"),
            ("JPY", float("1234.56"), "хиляда двеста тридесет и четири йени и петдесет и шест сена"),
            ("JPY", float("-12.34"), "минус дванадесет йени и тридесет и четири сена"),
            ("JPY", int("1000000"), "един милион йени"),
            ("JPY", float("0.5"), "нула йени и петдесет сена"),
        ];
        for (code, v, want) in rows {
            let got = bg.to_currency(&v, code, true, None, false).unwrap();
            assert_eq!(got, want, "currency:{} {:?}", code, v);
        }
    }

    /// BGN — BG's `to_currency` default currency. No corpus rows; values
    /// cross-checked against the live interpreter.
    #[test]
    fn currency_bgn_default() {
        let bg = LangBg::new();
        let d = bg.default_currency();
        assert_eq!(d, "BGN");
        assert_eq!(bg.to_currency(&int("1"), d, true, None, false).unwrap(), "един лев");
        assert_eq!(bg.to_currency(&int("2"), d, true, None, false).unwrap(), "два лева");
        assert_eq!(
            bg.to_currency(&float("1.0"), d, true, None, false).unwrap(),
            "един лев и нула стотинки"
        );
        // "един стотинка": masculine numeral, feminine unit (oddity 11).
        assert_eq!(
            bg.to_currency(&float("0.01"), d, true, None, false).unwrap(),
            "нула лева и един стотинка"
        );
    }

    /// The 48 `err: NotImplementedError` currency rows span 5 codes; the
    /// message is BG's own, *without* Base's "code" (oddity 7).
    #[test]
    fn currency_unknown_code_uses_bg_message() {
        let bg = LangBg::new();
        for code in ["KWD", "BHD", "INR", "CNY", "CHF"] {
            for v in [int("0"), int("1000000"), float("12.34"), float("-12.34")] {
                match bg.to_currency(&v, code, true, None, false) {
                    Err(N2WError::NotImplemented(m)) => assert_eq!(
                        m,
                        format!("Currency \"{}\" not implemented for \"Num2Word_BG\"", code)
                    ),
                    other => panic!("currency:{} {:?}: expected NotImplemented, got {:?}", code, v, other),
                }
            }
        }
    }

    /// Every `"lang": "bg", "to": "cheque:*"` row, via the inherited
    /// `default_to_cheque`. Note `AND` stays English and the unit takes
    /// `cr1[-1]` (plural) unconditionally.
    #[test]
    fn cheque_corpus_rows() {
        let bg = LangBg::new();
        let val = BigDecimal::from_str("1234.56").unwrap();
        for (code, want) in [
            ("EUR", "ХИЛЯДА ДВЕСТА ТРИДЕСЕТ И ЧЕТИРИ AND 56/100 ЕВРО"),
            ("USD", "ХИЛЯДА ДВЕСТА ТРИДЕСЕТ И ЧЕТИРИ AND 56/100 ДОЛАРА"),
            ("GBP", "ХИЛЯДА ДВЕСТА ТРИДЕСЕТ И ЧЕТИРИ AND 56/100 ПАУНДА"),
            ("JPY", "ХИЛЯДА ДВЕСТА ТРИДЕСЕТ И ЧЕТИРИ AND 56/100 ЙЕНИ"),
        ] {
            assert_eq!(bg.to_cheque(&val, code).unwrap(), want, "cheque:{}", code);
        }
    }

    /// `to_cheque` keeps *Base's* wording — "Currency code" — unlike
    /// `to_currency` above. Same class, same missing code, two messages.
    #[test]
    fn cheque_unknown_code_uses_base_message() {
        let bg = LangBg::new();
        let val = BigDecimal::from_str("1234.56").unwrap();
        for code in ["KWD", "BHD", "INR", "CNY", "CHF"] {
            match bg.to_cheque(&val, code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_BG\"", code)
                ),
                other => panic!("cheque:{}: expected NotImplemented, got {:?}", code, other),
            }
        }
    }

    /// `adjective=True` is accepted and ignored (oddity 9).
    #[test]
    fn adjective_is_ignored() {
        let bg = LangBg::new();
        let v = float("12.34");
        assert_eq!(
            bg.to_currency(&v, "USD", true, None, true).unwrap(),
            bg.to_currency(&v, "USD", true, None, false).unwrap(),
        );
        assert!(bg.currency_adjective("USD").is_none());
    }

    /// BG branches on `isinstance(val, int)`, never on `has_decimal`: a float
    /// prints cents whatever the flag says. Pins that the flag is unread.
    #[test]
    fn float_ignores_has_decimal() {
        let bg = LangBg::new();
        let with = CurrencyValue::parse("1.0", false, true, true).unwrap();
        let without = CurrencyValue::parse("1.0", false, false, false).unwrap();
        assert_eq!(
            bg.to_currency(&with, "EUR", true, None, false).unwrap(),
            "един евро и нула цента"
        );
        assert_eq!(
            bg.to_currency(&without, "EUR", true, None, false).unwrap(),
            "един евро и нула цента"
        );
    }

    /// `cents=False`: `str(right)` with no zero padding — "5", not "05".
    /// Cross-checked against the live interpreter.
    #[test]
    fn cents_terse_path_is_unpadded_str() {
        let bg = LangBg::new();
        assert_eq!(
            bg.to_currency(&float("12.34"), "USD", false, None, false).unwrap(),
            "дванадесет долара и 34 цента"
        );
        assert_eq!(
            bg.to_currency(&float("0.05"), "USD", false, None, false).unwrap(),
            "нула долара и 5 цента"
        );
    }

    /// An explicit `separator=` still wins over BG's " и" default.
    #[test]
    fn explicit_separator_overrides_default() {
        let bg = LangBg::new();
        assert_eq!(bg.default_separator(), " и");
        assert_eq!(
            bg.to_currency(&float("12.34"), "USD", true, Some(","), false).unwrap(),
            "дванадесет долара, тридесет и четири цента"
        );
    }

    /// BG never reaches `pluralize`; the default's raise mirrors Base's.
    #[test]
    fn pluralize_is_unreachable_and_still_raises() {
        let bg = LangBg::new();
        let forms = vec!["лев".to_string(), "лева".to_string()];
        assert!(matches!(
            bg.pluralize(&BigInt::from(1), &forms),
            Err(N2WError::NotImplemented(_))
        ));
    }

    // ---- float / Decimal ------------------------------------------------

    /// A Python `float` arg. `precision` is what the shim derives from
    /// `repr(value)`; it is quoted from the live interpreter, never guessed.
    fn f(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    /// A Python `Decimal` arg. `precision` is `abs(as_tuple().exponent)`.
    fn dec(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// Every `"lang": "bg", "to": "cardinal"` corpus row whose `arg` has a dot.
    #[test]
    fn cardinal_float_corpus_rows() {
        let bg = LangBg::new();
        let rows: Vec<(f64, u32, &str)> = vec![
            (0.0, 1, "нула"),
            (0.5, 1, "нула точка пет"),
            (1.0, 1, "един"),
            (1.5, 1, "един точка пет"),
            // Integer part masculine "два", fraction neuter "две" — one string
            // apart, same digit (oddity 14).
            (2.25, 2, "два точка две пет"),
            (3.14, 2, "три точка едно четири"),
            (0.01, 2, "нула точка нула едно"),
            (0.1, 1, "нула точка едно"),
            (0.99, 2, "нула точка девет девет"),
            (1.01, 2, "един точка нула едно"),
            (12.34, 2, "дванадесет точка три четири"),
            (99.99, 2, "деветдесет и девет точка девет девет"),
            (100.5, 1, "сто точка пет"),
            (1234.56, 2, "хиляда двеста тридесет и четири точка пет шест"),
            (-0.5, 1, "минус нула точка пет"),
            (-1.5, 1, "минус един точка пет"),
            (-12.34, 2, "минус дванадесет точка три четири"),
            // The two f64-artefact rows. 1.005 * 1000 is 4.999999999999999
            // away from 1000 -> post 5, padded to "005". 2.675 gives
            // 674.9999999999998, which the `< 0.01` heuristic rescues to 675.
            (1.005, 3, "един точка нула нула пет"),
            (2.675, 3, "два точка шест седем пет"),
        ];
        for (v, p, want) in rows {
            assert_eq!(bg.to_cardinal_float(&f(v, p), None).unwrap(), want, "{}", v);
        }
    }

    /// Every `"lang": "bg", "to": "cardinal_dec"` corpus row.
    ///
    /// Python truncated all five (oddity 13); since #156 a fractional Decimal
    /// reads like a float. The #603 trillion-scale row proves the value is
    /// not routed through an f64: 98746251323029.99 keeps its ...029, which a
    /// float cast would have rounded to ...030.
    #[test]
    fn cardinal_decimal_corpus_rows() {
        let bg = LangBg::new();
        let rows: Vec<(&str, u32, &str)> = vec![
            ("0.01", 2, "нула точка нула едно"),
            ("1.10", 2, "един точка едно нула"),
            ("12.345", 3, "дванадесет точка три четири пет"),
            (
                "98746251329.99",
                2,
                "деветдесет и осем милиарда седемстотин четиридесет и шест милиона \
                 двеста петдесет и едно хиляди триста двадесет и девет точка девет девет",
            ),
            ("0.001", 3, "нула точка нула нула едно"),
        ];
        for (s, p, want) in rows {
            assert_eq!(bg.to_cardinal_float(&dec(s, p), None).unwrap(), want, "{}", s);
        }
    }

    /// Decimal negatives and signed zero read like the matching floats
    /// (#156). `Decimal("-0.0") < 0` is false in Python, and BigDecimal has
    /// no signed zero, so both zeros answer plain "нула" with no sign.
    #[test]
    fn cardinal_decimal_negatives() {
        let bg = LangBg::new();
        for (s, p, want) in [
            ("-1.5", 1, "минус един точка пет"),
            ("-0.5", 1, "минус нула точка пет"),
            ("-12.34", 2, "минус дванадесет точка три четири"),
            ("2.675", 3, "два точка шест седем пет"),
            ("-0.0", 1, "нула"),
            ("0.0", 1, "нула"),
        ] {
            assert_eq!(bg.to_cardinal_float(&dec(s, p), None).unwrap(), want, "{}", s);
        }
    }

    /// Integral floats skip the fraction entirely (`n != int(n)` is false) and
    /// take the "For integers" tail, where the sign recursion restores the
    /// masculine agreement. Live-checked.
    #[test]
    fn cardinal_integral_floats() {
        let bg = LangBg::new();
        for (v, p, want) in [
            (0.0, 1, "нула"),
            (-0.0, 1, "нула"),
            (1.0, 1, "един"),
            (2.0, 1, "два"),
            (-1.0, 1, "минус един"),
            (-3.0, 1, "минус три"),
            // repr(1e21) is "1e+21", so the shim hands over precision 21 —
            // and the integer tail never looks at it (oddity 16).
        ] {
            assert_eq!(bg.to_cardinal_float(&f(v, p), None).unwrap(), want, "{}", v);
        }
        // Past the #203 ceiling (10^12) "милиарда" would stack.
        assert!(matches!(bg.to_cardinal_float(&f(1e21, 21), None), Err(N2WError::Overflow(_))));
    }

    /// More float rows from the live interpreter, chosen for the digits that
    /// separate the neuter fraction table from the masculine integer one.
    #[test]
    fn cardinal_float_live_rows() {
        let bg = LangBg::new();
        for (v, p, want) in [
            (1.1, 1, "един точка едно"),
            (2.2, 1, "два точка две"),
            (1.2, 1, "един точка две"),
            (0.02, 2, "нула точка нула две"),
            (9.99, 2, "девет точка девет девет"),
            (-0.01, 2, "минус нула точка нула едно"),
            (-0.001, 3, "минус нула точка нула нула едно"),
            (0.001, 3, "нула точка нула нула едно"),
            (-2.25, 2, "минус два точка две пет"),
            (-100.5, 1, "минус сто точка пет"),
            (-1234.56, 2, "минус хиляда двеста тридесет и четири точка пет шест"),
            (1000000.5, 1, "един милион точка пет"),
            (1e-7, 7, "нула точка нула нула нула нула нула нула едно"),
            (
                0.5000000001,
                10,
                "нула точка пет нула нула нула нула нула нула нула нула едно",
            ),
            (
                123456789.123,
                3,
                "сто двадесет и три милиона четиристотин петдесет и шест хиляди \
                 седемстотин осемдесет и девет точка едно две три",
            ),
        ] {
            assert_eq!(bg.to_cardinal_float(&f(v, p), None).unwrap(), want, "{}", v);
        }
    }

    /// `precision=` is dead for BG: the dispatcher writes `converter.precision`
    /// and `float2tuple` overwrites it from `repr(value)` before anything reads
    /// it (oddity 16). Live-checked at precision 1, 3 and 5.
    #[test]
    fn precision_override_is_ignored() {
        let bg = LangBg::new();
        for p in [None, Some(1), Some(3), Some(5), Some(0)] {
            assert_eq!(
                bg.to_cardinal_float(&f(1.005, 3), p).unwrap(),
                "един точка нула нула пет"
            );
            assert_eq!(
                bg.to_cardinal_float(&f(2.675, 3), p).unwrap(),
                "два точка шест седем пет"
            );
            assert_eq!(
                bg.to_cardinal_float(&dec("12.345", 3), p).unwrap(),
                "дванадесет точка три четири пет"
            );
        }
    }

    /// `10**precision` past ~1.8e308 raises OverflowError inside `float2tuple`,
    /// and BG's `except BaseException` answers `_int_to_cardinal(int(n))` —
    /// "нула" for every such n, sign discarded (oddity 17). The boundary is
    /// live-checked: precision 308 still spells the fraction out in full.
    #[test]
    fn float2tuple_overflow_is_swallowed() {
        let bg = LangBg::new();
        // repr(5e-324) == "5e-324" -> precision 324; likewise 1e-310 -> 310.
        assert_eq!(bg.to_cardinal_float(&f(5e-324, 324), None).unwrap(), "нула");
        assert_eq!(bg.to_cardinal_float(&f(-5e-324, 324), None).unwrap(), "нула");
        assert_eq!(bg.to_cardinal_float(&f(1e-310, 310), None).unwrap(), "нула");
        assert_eq!(bg.to_cardinal_float(&f(-1e-310, 310), None).unwrap(), "нула");

        // 10**308 fits, so this one goes the long way round: "минус нула точка"
        // then 307 zeros and a one.
        let got = bg.to_cardinal_float(&f(-1e-308, 308), None).unwrap();
        assert!(got.starts_with("минус нула точка нула"), "{}", got);
        assert!(got.ends_with(" едно"), "{}", got);
        assert_eq!(got.split(' ').count(), 3 + 308);
    }

    /// `int(n)` raises before `n != int(n)` can answer, and the `except` body
    /// re-runs the same call, so the exception propagates with Python's type.
    /// Not reachable through the shim (it computes `precision` from
    /// `Decimal(str(value))` first, which raises TypeError on the Python side),
    /// but the messages match the live interpreter.
    #[test]
    fn non_finite_floats_raise_pythons_types() {
        let bg = LangBg::new();
        for v in [f64::INFINITY, f64::NEG_INFINITY] {
            match bg.to_cardinal_float(&f(v, 0), None) {
                Err(N2WError::Overflow(m)) => {
                    assert_eq!(m, "cannot convert float infinity to integer")
                }
                other => panic!("{}: expected Overflow, got {:?}", v, other),
            }
        }
        match bg.to_cardinal_float(&f(f64::NAN, 0), None) {
            Err(N2WError::Value(m)) => assert_eq!(m, "cannot convert float NaN to integer"),
            other => panic!("nan: expected Value, got {:?}", other),
        }
    }

    /// The fractional-cents branch of `to_currency`, i.e. Python's
    /// `self.to_cardinal(float(right))` via `cardinal_from_decimal`. No corpus
    /// row reaches it (every `bg` currency arg has <= 2 decimals), so these are
    /// quoted from the live interpreter.
    ///
    /// 1.021 is the one that pins both tables at once: the cents *count* is
    /// masculine "два" and its fraction digit is neuter "едно".
    #[test]
    fn currency_fractional_cents() {
        let bg = LangBg::new();
        for (arg, code, want) in [
            ("1.011", "USD", "един долар и един точка едно цента"),
            ("1.021", "USD", "един долар и два точка едно цента"),
            ("0.005", "USD", "нула долара и нула точка пет цента"),
            ("2.567", "USD", "два долара и петдесет и шест точка седем цента"),
            ("3.0125", "USD", "три долара и един точка две пет цента"),
            ("0.0075", "USD", "нула долара и нула точка седем пет цента"),
            ("-1.011", "USD", "минус един долар и един точка едно цента"),
            ("12.345", "USD", "дванадесет долара и тридесет и четири точка пет цента"),
            ("0.001", "USD", "нула долара и нула точка едно цента"),
            ("9.999", "USD", "девет долара и деветдесет и девет точка девет цента"),
            ("1.011", "BGN", "един лев и един точка едно стотинки"),
            ("2.567", "BGN", "два лева и петдесет и шест точка седем стотинки"),
        ] {
            assert_eq!(
                bg.to_currency(&float(arg), code, true, None, false).unwrap(),
                want,
                "{} {}",
                arg,
                code
            );
        }
    }
}
