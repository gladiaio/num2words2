//! Port of `lang_RW.py` (Kinyarwanda).
//!
//! Shape: **self-contained**. `Num2Word_RW` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires: Python
//! never builds `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is
//! overridden outright and drives `_int_to_word`, a plain recursive descent
//! over 0 / <10 / <100 / <1000 / <10^6 / <10^9 bands.
//!
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here, and
//! **nothing in the integer modes can raise**: there is no overflow check, no
//! `verify_ordinal` call, and no table lookup that can miss. Every
//! `to_cardinal`/`to_ordinal`/`to_ordinal_num`/`to_year` on an integer returns
//! `Ok`. This matches the frozen corpus, in which all 324 such `rw` rows carry
//! `"ok": true`. The only `ok: false` rows there are seven `cheque:*` codes
//! (`NotImplementedError`, reproduced below) and `fraction` (out of scope).
//! The float/Decimal corpora (`corpus_wholefloat.jsonl`, `corpus_strings.jsonl`)
//! do add `ok: false` rows — `ValueError` on exponent-form reprs (`1e+16`,
//! `Decimal("1E+2")`, the string `"1e3"`) and on `"Infinity"` — because RW's
//! `to_cardinal` feeds `int()` whatever `str(number)` produced. All are
//! reproduced by the float entry hooks below.
//!
//! Inherited from `Num2Word_Base` and left alone by RW:
//!   * `is_title` stays `False` (`__init__` sets it; `setup()` never touches
//!     it), so `title()` is an identity. RW's `to_cardinal` override does not
//!     call `self.title()` at all, unlike the base implementation — the
//!     `exclude_title` list RW sets up is therefore dead weight in every
//!     in-scope path. It is reproduced below for fidelity regardless.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly what
//! Python emits, verified against the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns bare digits.** The final
//!    `return str(number)` means `to_cardinal(10**9)` == `"1000000000"` and
//!    `to_cardinal(10**12)` == `"1000000000000"` — decimal numerals, not words.
//!    There is no billion word in the table and no `OverflowError`; the
//!    converter silently degrades. Corpus rows confirm both. Modelled by
//!    [`int_to_word`].
//! 2. **`to_ordinal` never calls `verify_ordinal`,** so negatives sail through:
//!    `to_ordinal(-1)` == `"wa munsi ya rimwe"` ("the -1st"), where every
//!    `verify_ordinal`-calling language raises `TypeError`. Likewise
//!    `to_ordinal(0)` == `"wa zeru"`. Both are corpus-confirmed.
//! 3. **`to_ordinal` inherits the 10^9 fallback,** so `to_ordinal(10**9)` ==
//!    `"wa 1000000000"` — identical to `to_ordinal_num(10**9)`. The two modes
//!    are indistinguishable above the ceiling.
//! 4. **`negword` carries a trailing space** (`"munsi ya "`), unlike the base
//!    class's `"(-) "` convention where callers `.strip()` it. RW's
//!    `to_cardinal` concatenates it raw and `.strip()`s the *whole* result, so
//!    the space lands between "ya" and the number word. The trailing `.strip()`
//!    is a no-op in every reachable case (`_int_to_word` never returns padded
//!    text) but is reproduced anyway.
//! 5. **The hundreds/thousands/millions multiplier is dropped when it is 1**
//!    but *not* when it is 0-ish — `h > 1` guards `ones[h]`, so 100 is "ijana"
//!    (bare) while 200 is "kabiri ijana". Same for "igihumbi" and "miliyoni".
//! 6. **`tens[]` uses different stems from `ones[]`** for the same digit:
//!    `ones[8]` is "umunani" but `tens[8]` is "mirongo inani"; `ones[9]` is
//!    "icyenda" but `tens[9]` is "mirongo cyenda". This is correct Kinyarwanda
//!    noun-class agreement, not a typo — flagged only so a later reviewer does
//!    not "fix" it into consistency.
//!
//! # Float / Decimal routing (`str(number)` decides everything)
//!
//! RW's `to_cardinal` runs on `n = str(number).strip()`, so the *repr* — not
//! the numeric value — picks the branch. Four corpus-pinned consequences:
//!
//! * **Whole-valued floats keep their ".0" tail.** `str(5.0)` is `"5.0"`, so
//!   `to_cardinal(5.0)` == "gatanu akadomo zeru" — never the integer path's
//!   bare "gatanu". Same for `Decimal("5.00")` → "gatanu akadomo zeru zeru"
//!   (trailing zeros survive `str(Decimal)`), while `Decimal("5")` has no "."
//!   and *does* take the integer words. [`Lang::cardinal_float_entry`] is
//!   therefore overridden to send every float/Decimal through the string
//!   algorithm ([`cardinal_from_str`]) instead of the base's whole-value
//!   integer route.
//! * **Exponent-form reprs raise `ValueError`.** `str(1e16)` is `"1e+16"` and
//!   `str(Decimal("1E+2"))` is `"1E+2"` — no ".", so `int(n)` chokes:
//!   `ValueError: invalid literal for int() with base 10: '1e+16'`. The repr
//!   is rebuilt exactly by [`python_float_repr`] /
//!   [`crate::strnum::python_decimal_str`], so the error falls out of the same
//!   `int()` step as in Python.
//! * **`to_ordinal(1.0)` is "mbere".** The guard is `if number == 1:` —
//!   numeric equality, so `1.0` and `Decimal("1.00")` hit it too. Everything
//!   else is `"wa " + to_cardinal(number)`, floats included:
//!   `to_ordinal(2.0)` == "wa kabiri akadomo zeru", and `to_ordinal(-0.0)` ==
//!   "wa munsi ya zeru akadomo zeru" (no verify_ordinal, quirk 2).
//! * **`to_ordinal_num` is `"wa " + str(number)`** for floats too:
//!   `to_ordinal_num(1e16)` == "wa 1e+16" — no error, unlike `to_ordinal`.
//!   The binding supplies Python's own `str(number)` as `repr_str`.
//!
//! String input `"Infinity"`/`"-Infinity"` parses to `Decimal("Infinity")` in
//! Python (base `str_to_number`), and then RW's `int("Infinity")` raises
//! `ValueError`. The Rust dispatcher hard-codes `ParsedNumber::Inf` →
//! `OverflowError` (the base-class behaviour) before any language hook runs,
//! so [`Lang::str_to_number`] is overridden here to surface the `ValueError`
//! at parse time instead. Known divergence, documented rather than fixable
//! from this file: Python's `to_ordinal_num(Decimal("Infinity"))` returns
//! `"wa Infinity"` *without* raising, but with the early raise this port
//! reports `ValueError` for that mode as well. No corpus row exercises it —
//! the Infinity rows are all `to="cardinal"` (ValueError in both worlds).
//!
//! # The currency surface
//!
//! RW overrides `to_currency` **wholesale** and shares almost nothing with
//! `Num2Word_Base.to_currency`. It does not call `pluralize`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse`, `parse_currency_parts` or
//! `CURRENCY_PRECISION`; it re-derives everything from `str(val)` by hand. So
//! [`Lang::to_currency`] is overridden here rather than delegating to
//! `currency::default_to_currency`, and the divergences below are real:
//!
//! 7. **`to_currency` cannot raise `NotImplementedError`.** Python does
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — an unknown code silently renders as **RWF**, the class body's first
//!    key. `to_cheque`, inherited from the base class, uses a `[]` lookup and
//!    *does* raise. Hence the corpus pairing
//!    `currency:JPY 12.34 -> "icumi na kabiri amafaranga …"` beside
//!    `cheque:JPY -> NotImplementedError`, for the same code, in the same
//!    language. Modelled by the `unwrap_or` in [`LangRw::to_currency`] versus
//!    the plain `Option` returned from [`LangRw::currency_forms`].
//! 8. **`CURRENCY_PRECISION` is never consulted**, so the 3-decimal (KWD/BHD,
//!    divisor 1000) and 0-decimal (JPY, divisor 1) handling that
//!    `base.to_currency` performs does not exist here. Those codes are not in
//!    RW's table anyway, so they take the RWF fallback and get a plain
//!    two-decimal split — corpus-confirmed for all four of JPY/KWD/BHD/CNY.
//!    (`to_cheque` *does* read it, but RW defines none and `Num2Word_Base`'s is
//!    `{}`, so the divisor is always the default 100.)
//! 9. **Cents truncate, they do not round.** `int(parts[1][:2].ljust(2, "0"))`
//!    slices the first two fractional digits off the *string*: `12.345` gives
//!    34 cents, and `1.005` gives **0** — which then trips the `if cents and
//!    right` guard, so `to_currency(1.005)` == `"rimwe idolari"` with no cents
//!    segment at all. `base.to_currency`'s ROUND_HALF_UP would say one cent.
//!    Modelled by [`split_str_parts`].
//! 10. **`cents=False` drops the cents segment entirely.** The base class
//!    switches to `_cents_terse` (digits, e.g. "34"); RW's `if cents and right`
//!    just skips it, so `to_currency(12.34, cents=False)` ==
//!    `"icumi na kabiri idolari"`. `_cents_terse`/`_cents_verbose` are
//!    unreachable from RW's currency path.
//! 11. **`adjective=True` is a silent no-op.** RW accepts the parameter and its
//!    body never mentions it; `CURRENCY_ADJECTIVES` is empty. The base class
//!    would prefix the unit.
//! 12. **`separator` is concatenated raw**, with no space of its own:
//!    `result += separator + word + " " + form`. The default `" "` supplies the
//!    space, so a caller passing `separator=" na"` gets
//!    `"… idolari namirongo itatu na kane sentime"` — the words run together.
//!    Reproduced verbatim.
//! 13. **Both plural forms are identical in every entry**
//!    (`("amafaranga", "amafaranga")`, `("sentime", "sentime")`), so the
//!    `cr1[1] if left != 1 else cr1[0]` choice is unobservable in the output.
//!    The index is still computed, because a one-element tuple would make it an
//!    IndexError rather than a silent fallback — see [`plural_form`].
//!
//! RW is also **not** a victim of the `lang_EUR.py` mutation trap described in
//! `PORTING_CURRENCY.md`: it subclasses `Num2Word_Base` (whose `CURRENCY_FORMS`
//! is `{}`) and shadows the attribute with its own class-body dict, so
//! `Num2Word_EN.__init__`'s in-place rewrite of the shared `Num2Word_EUR` dict
//! cannot reach it. Confirmed against the live interpreter: at runtime
//! `CONVERTER_CLASSES["rw"].CURRENCY_FORMS` holds exactly RWF/USD/EUR, with
//! EUR still `("iyero", "iyero")` and no EN-injected codes.
//!
//! # Known gap: `to_currency` on floats whose `repr` is scientific notation
//!
//! The cardinal/ordinal/year float modes reconstruct `str(number)` exactly
//! ([`python_float_repr`]) and reproduce the `int()` ValueError on exponent
//! reprs. `to_currency` cannot: its value arrives pre-parsed as a
//! `CurrencyValue`, which is less than the text Python reads. Python does
//! `int(parts[0])` on whatever `str(abs(val))` produced,
//! so a float big or small enough for `repr` to go exponential dies on the
//! `int()`:
//!
//! ```text
//! num2words(1e16,  to="currency", lang="rw")  # ValueError: invalid literal
//! num2words(1e21,  to="currency", lang="rw")  # ValueError: invalid literal
//! num2words(1e-05, to="currency", lang="rw")  # ValueError: invalid literal
//! num2words(0.0001, to="currency", lang="rw") # ok -> "zeru idolari"
//! ```
//!
//! i.e. `abs(x) >= 1e16` or `0 < abs(x) < 1e-4`. This port returns a string for
//! all three instead (`"10000000000000000 idolari"`, `"zeru idolari"`, …).
//!
//! It is **not** reproducible here, and not for want of an error variant —
//! `N2WError::Value` fits fine. The boundary is the problem: the shim parses
//! `str(val)` into a `BigDecimal` and hands over the number, so the exponent
//! notation is gone by the time this code runs. The two inputs that decide the
//! outcome are indistinguishable on arrival:
//!
//! | Python input | `str(val)` | Python | arrives as |
//! |---|---|---|---|
//! | `1e-05` (float) | `"1e-05"` | ValueError | `BigDecimal(0.00001)`, `has_decimal=true` |
//! | `Decimal("0.00001")` | `"0.00001"` | `"zeru idolari"` | `BigDecimal(0.00001)`, `has_decimal=true` |
//!
//! Same value, same `has_decimal`, opposite behaviour. (`has_decimal` separates
//! the `1e16` pair, but not this one, and `Decimal("1E+16")` raises too — so no
//! flag available here decides all three.) Reproducing this needs the raw
//! `str(val)` carried across the boundary, which is a `currency.rs` change and
//! out of scope for a single language file. No corpus row reaches it: the `rw`
//! block's largest float is `1234.56`, and `bench/diff_test.py` already screens
//! `"e"` when it computes `is_int`.
//!
//! # Unreachable Python quirk (deliberately not modelled)
//!
//! `_int_to_word(-5)` would hit `number < 10` and evaluate `self.ones[-5]`,
//! which in Python is *negative indexing* — it silently returns `ones[5]` ==
//! "gatanu", i.e. the wrong sign with no error. This is unreachable through any
//! in-scope entry point: `to_cardinal` detaches the minus from the *string*
//! (`n[1:]`) before parsing, and `to_currency` does `val = abs(val)` before
//! anything else, so `_int_to_word` only ever sees non-negatives.
//! [`int_to_word`] is therefore only ever called with a non-negative value, and
//! its `to_u64()` guard would send a negative down the `str(number)` fallback
//! rather than reproducing the negative-index behaviour.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// `self.ones`. Index 0 is `""` and is never used: `_int_to_word` handles 0
/// before the `< 10` branch, and every other use is guarded (`if o`, `h > 1`).
const ONES: [&str; 10] = [
    "",
    "rimwe",
    "kabiri",
    "gatatu",
    "kane",
    "gatanu",
    "gatandatu",
    "karindwi",
    "umunani",
    "icyenda",
];

/// `self.tens`. Index 0 is `""` and unreachable (the `< 100` branch is only
/// entered for `number >= 10`, so `t >= 1`).
const TENS: [&str; 10] = [
    "",
    "icumi",
    "makumyabiri",
    "mirongo itatu",
    "mirongo ine",
    "mirongo itanu",
    "mirongo itandatu",
    "mirongo irindwi",
    "mirongo inani",
    "mirongo cyenda",
];

const HUNDRED: &str = "ijana";
const THOUSAND: &str = "igihumbi";
const MILLION: &str = "miliyoni";

/// `self.negword`. The trailing space is in the Python source — see quirk 4.
const NEGWORD: &str = "munsi ya ";
/// `self.pointword`, the token RW's `to_cardinal` prints between the integer
/// part and the spelled-out fractional digits on the float/Decimal path — see
/// [`to_cardinal_float`](LangRw::to_cardinal_float).
const POINTWORD: &str = "akadomo";
/// `self.exclude_title`. Dead in every in-scope path (`is_title` is False and
/// RW's `to_cardinal` never calls `title()`), kept for fidelity.
const EXCLUDE_TITLE: [&str; 4] = ["na", "akadomo", "munsi", "ya"];

const ZERU: &str = "zeru";

/// The ceiling of `_int_to_word`'s word-producing bands: `1000000000`.
const BILLION: u64 = 1_000_000_000;

/// Python's `_int_to_word`, entry point.
///
/// Python tests `number < 1000000000` last and falls through to
/// `return str(number)`. Anything at or above 10^9 is therefore rendered as
/// bare decimal digits (quirk 1). Below that the value provably fits in a
/// `u64`, so [`int_to_word_small`] does the recursive work on a fixed-width
/// int — the BigInt is never truncated, only handed off once bounded.
fn int_to_word(value: &BigInt) -> String {
    match value.to_u64() {
        Some(n) if n < BILLION => int_to_word_small(n),
        // Covers both `>= 10^9` and (unreachably) negatives, matching Python's
        // final `return str(number)` for the former.
        _ => value.to_string(),
    }
}

/// Python's `_int_to_word` for `0 <= n < 10^9`.
///
/// Every recursive call shrinks the value (`divmod` by a strictly larger
/// power), so no inner call can re-enter the `str(number)` fallback.
fn int_to_word_small(n: u64) -> String {
    // if number == 0: return "zeru"
    if n == 0 {
        return ZERU.to_string();
    }

    // if number < 10: return self.ones[number]
    if n < 10 {
        return ONES[n as usize].to_string();
    }

    // if number < 100:
    //     t, o = divmod(number, 10)
    //     return self.tens[t] + (" na " + self.ones[o] if o else "")
    if n < 100 {
        let (t, o) = (n / 10, n % 10);
        let mut out = TENS[t as usize].to_string();
        if o != 0 {
            out.push_str(" na ");
            out.push_str(ONES[o as usize]);
        }
        return out;
    }

    // if number < 1000:
    //     h, r = divmod(number, 100)
    //     base = (self.ones[h] + " " if h > 1 else "") + self.hundred
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if n < 1_000 {
        let (h, r) = (n / 100, n % 100);
        let mut out = String::new();
        if h > 1 {
            out.push_str(ONES[h as usize]);
            out.push(' ');
        }
        out.push_str(HUNDRED);
        if r != 0 {
            out.push_str(" na ");
            out.push_str(&int_to_word_small(r));
        }
        return out;
    }

    // if number < 1000000:
    //     t, r = divmod(number, 1000)
    //     base = (self._int_to_word(t) + " " if t > 1 else "") + self.thousand
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if n < 1_000_000 {
        let (t, r) = (n / 1_000, n % 1_000);
        let mut out = String::new();
        if t > 1 {
            out.push_str(&int_to_word_small(t));
            out.push(' ');
        }
        out.push_str(THOUSAND);
        if r != 0 {
            out.push_str(" na ");
            out.push_str(&int_to_word_small(r));
        }
        return out;
    }

    // if number < 1000000000:
    //     m, r = divmod(number, 1000000)
    //     base = (self._int_to_word(m) + " " if m > 1 else "") + self.million
    //     return base + (" na " + self._int_to_word(r) if r else "")
    let (m, r) = (n / 1_000_000, n % 1_000_000);
    let mut out = String::new();
    if m > 1 {
        out.push_str(&int_to_word_small(m));
        out.push(' ');
    }
    out.push_str(MILLION);
    if r != 0 {
        out.push_str(" na ");
        out.push_str(&int_to_word_small(r));
    }
    out
}

/// Python's `to_cardinal`, integer path.
///
/// Python works on `str(number).strip()` and detaches the sign by slicing
/// (`n[1:]`), then recurses. For an integer that is exactly negation, so this
/// recurses on `-value` instead of reparsing a string. The full string form —
/// including the `"." in n` float/Decimal branch — lives in
/// [`cardinal_from_str`], reached via
/// [`to_cardinal_float`](LangRw::to_cardinal_float).
fn cardinal(value: &BigInt) -> String {
    // if n.startswith("-"):
    //     return (self.negword + self.to_cardinal(n[1:])).strip()
    if value.is_negative() {
        let inner = cardinal(&value.abs());
        return format!("{}{}", NEGWORD, inner).trim().to_string();
    }
    int_to_word(value)
}

/// CPython's `repr(float)` / `str(float)`, which is what RW's `to_cardinal`
/// runs on: `n = str(number).strip()`. RW's float branch is a *string*
/// algorithm — it splits on ".", feeds `int()` the pieces and spells the
/// fraction digit by digit — so the exact repr, not the numeric value, drives
/// the output (and the ValueErrors). **RW never uses `base.float2tuple`**, so
/// the two `floatpath.rs` traps — banker's rounding of a binary `post`, and
/// the `< 0.01` artefact heuristic — do not arise here; recomputing
/// `abs(2.675 - 2) * 1000` in `f64` (which gives `674.999…`) would compute
/// the *wrong* thing to reproduce.
///
/// # 1. The digits
///
/// `{:e}` is Rust's shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the
/// significant digits and the decimal-point position fall straight out. A rare
/// tie can leave `{:e}`'s final digit one off the value CPython's dtoa would
/// pick; re-formatting with `{:.*}` at the known digit count repairs it. This
/// exact function is differentially tested against CPython on 300k doubles in
/// the sibling ports (`lang_bm`, `lang_ki`): 0 mismatches with the repair.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`
/// (`format_float_short`, format code `'r'`), pads the exponent to two digits,
/// and appends `.0` to anything that would otherwise look like an integer.
/// Rust's `{}` does none of this, so both `1e16` and `1.0` would come out
/// wrong in opposite directions. Both matter to RW: `str(1.0)` is `"1.0"` →
/// "rimwe akadomo zeru", and `str(1e16)` is `"1e+16"` → `int("1e+16")` raises
/// `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this: for an exponent-form repr it is the *exponent*
/// (`abs(Decimal(str(v)).as_tuple().exponent)`), not a digit count — `1e16`
/// arrives with `precision == 16`.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). RW feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and RW renders that
    // "munsi ya zeru akadomo zeru".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `decpt` is CPython's: the value is `0.<digits> * 10**decpt`.
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').expect("LowerExp always emits an 'e'");
    let exp: i32 = exp.parse().expect("LowerExp emits an integer exponent");
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut decpt = exp + 1;

    // Tie repair — see the doc comment. Only reachable when the shortest form
    // has fractional digits; `a == 0.0` is excluded because `{:e}` reports it
    // as "0e0" and there is nothing to round.
    let frac_digits = digits.chars().count() as i32 - decpt;
    if frac_digits > 0 && a != 0.0 {
        let t = format!("{:.*}", frac_digits as usize, a);
        let (ip, fp) = t.split_once('.').expect("frac_digits > 0 forces a point");
        let all = format!("{}{}", ip, fp);
        let trimmed = all.trim_start_matches('0');
        if !trimmed.is_empty() {
            let lead = all.chars().count() - trimmed.chars().count();
            digits = trimmed.to_string();
            decpt = ip.chars().count() as i32 - lead as i32;
        }
    }

    let n = digits.chars().count() as i32;

    if decpt <= -4 || decpt > 16 {
        // CPython: mantissa, then "e", then "%+.02d" of decpt-1.
        let e = decpt - 1;
        let mut out = String::from(sign);
        let mut it = digits.chars();
        out.push(it.next().expect("a finite double has at least one digit"));
        if n > 1 {
            out.push('.');
            out.push_str(it.as_str());
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (e as i64).abs()));
        out
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= n {
        // Py_DTSF_ADD_DOT_0: an integral value still reprs with a ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - n) as usize))
    } else {
        let k = decpt as usize;
        format!(
            "{}{}.{}",
            sign,
            digits.chars().take(k).collect::<String>(),
            digits.chars().skip(k).collect::<String>()
        )
    }
}

/// `str(number)` for whatever the Python dispatcher handed the converter. The
/// `FloatValue` split is exactly Python's `isinstance(value, Decimal)`: the
/// two arms stringify by different rules and must not be collapsed.
/// `str(Decimal)` is the spec algorithm in [`crate::strnum::python_decimal_str`]
/// — capital `E`, trailing zeros preserved (`Decimal("1.10")` → "1.10"),
/// exponent form for a positive exponent (`Decimal("1E+2")` → "1E+2").
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// Python's `int(s)`, for the strings `str()` can produce here. `BigInt`'s
/// parser and `int()` agree on everything reachable — plain ASCII digit runs
/// with an optional sign — and the message quotes the offending literal
/// verbatim, as Python's does (`'1e+16'`, `'1E+2'`, `'inf'`, `'nan'`).
fn python_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Port of `Num2Word_RW.to_cardinal` driven by the *string*, i.e. the float /
/// Decimal branch:
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     return (self.negword + self.to_cardinal(n[1:])).strip()
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "zeru")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// Details that look like slips and are not:
///
/// * The negative branch recurses with a **string** (`n[1:]`), whose first act
///   is `str(number)` — a no-op on a `str` — so a negative float re-enters
///   here and keeps the "." branch. The recursion here mirrors that shape.
/// * `int(left)` runs before the digit loop, so a bad left (`"1e+16"`) raises
///   first and quotes the whole literal; a malformed *fraction* field
///   (`"5E+300"` from a scientific Decimal) quotes the single char (`'E'`),
///   because `int(digit)` is per **character**.
/// * The integer part goes through `_int_to_word` (not `to_cardinal`), so
///   above 10^9 it degrades to bare digits exactly as quirk 1 describes:
///   `Decimal("98746251323029.99")` → "98746251323029 akadomo icyenda icyenda".
/// * `self.ones[int(digit)] or "zeru"`: `ones[0]` is `""` (falsy), so a 0
///   fraction digit speaks "zeru"; every other digit is `ones[d]`.
/// * Python's `.strip()`s are no-ops in every reachable case (nothing is
///   padded), reproduced as `trim()` because Python does them.
fn cardinal_from_str(n: &str) -> Result<String> {
    // Python's str.strip(); str()'s output never has surrounding space.
    let n = n.trim();

    if let Some(rest) = n.strip_prefix('-') {
        let inner = cardinal_from_str(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }

    if let Some((left, right)) = n.split_once('.') {
        // `int(left)` runs before the loop, so a bad left raises first.
        let mut ret = int_to_word(&python_int(left)?);
        ret.push(' ');
        ret.push_str(POINTWORD);
        for ch in right.chars() {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    ch
                ))
            })?;
            ret.push(' ');
            ret.push_str(if d == 0 { ZERU } else { ONES[d as usize] });
        }
        return Ok(ret.trim().to_string());
    }

    Ok(int_to_word(&python_int(n)?))
}

/// `Num2Word_RW.CURRENCY_FORMS`.
///
/// A `Vec` rather than the usual `HashMap` because **insertion order is
/// load-bearing** here: `to_currency`'s fallback for an unknown code is
/// `list(self.CURRENCY_FORMS.values())[0]`, and Python dicts iterate in
/// insertion order, so that is RWF — the class body's first key. Backing this
/// with a `HashMap` would make every unknown-code row depend on hash order.
/// Three entries, so the linear scan in [`LangRw::currency_forms`] is cheaper
/// than hashing anyway.
///
/// Built once in [`LangRw::new`] and stored, never per call.
fn build_currency_forms() -> Vec<(&'static str, CurrencyForms)> {
    // ("sentime", "sentime") — shared by all three entries.
    const SENTIME: [&str; 2] = ["sentime", "sentime"];

    vec![
        (
            "RWF",
            CurrencyForms::new(&["amafaranga", "amafaranga"], &SENTIME),
        ),
        ("USD", CurrencyForms::new(&["idolari", "idolari"], &SENTIME)),
        ("EUR", CurrencyForms::new(&["iyero", "iyero"], &SENTIME)),
    ]
}

/// Python's `cr1[1] if left != 1 else cr1[0]` — the inline plural choice
/// `to_currency` makes *instead of* calling `self.pluralize`.
///
/// Both forms are identical in every RW entry (quirk 13), so the result is the
/// same either way today. The index is still computed because Python computes
/// it: a hypothetical one-element tuple would raise IndexError here, not fall
/// back silently.
fn plural_form(forms: &[String], n: &BigInt) -> Result<String> {
    let idx = if n.is_one() { 0 } else { 1 };
    forms
        .get(idx)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// Python's `str(val).split(".")`, reduced to the two integers `to_currency`
/// pulls out of it. `value` must be **non-negative** — Python does
/// `val = abs(val)` before stringifying.
///
/// * `left = int(parts[0]) if parts[0] else 0` — the digits before the point,
///   i.e. the truncated integer part (== floor, since `value >= 0`). `parts[0]`
///   is only empty for a string like ".5", which no `str()` of a number
///   produces, so the `else 0` arm is dead.
/// * `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1]
///   else 0` — the first two fractional digits, zero-padded on the *right*.
///   Slicing a two-digit prefix and right-padding it is exactly
///   `floor(frac * 100)`: "3" -> "30" -> 30, "345" -> "34" -> 34, "0" -> "00"
///   -> 0. So this is a truncation, not a rounding (quirk 9).
///
/// Collapsing both `right` guards into the arithmetic is safe: a value with no
/// fractional digits has `frac == 0`, which the multiply-and-truncate already
/// maps to 0.
fn split_str_parts(value: &BigDecimal) -> (BigInt, BigInt) {
    // int(parts[0]) — `with_scale(0)` truncates, matching int() on the digits
    // left of the point for a non-negative value.
    let integer = value.with_scale(0);
    let left = integer.as_bigint_and_exponent().0;

    // int(parts[1][:2].ljust(2, "0")) == floor(frac * 100).
    let frac = value - &integer;
    let right = (frac * BigDecimal::from(100))
        .with_scale(0)
        .as_bigint_and_exponent()
        .0;

    (left, right)
}

pub struct LangRw {
    /// `self.exclude_title`, owned so `Lang::exclude_title` can hand out a
    /// `&[String]`. Unreachable in scope — see the module docs.
    exclude_title: Vec<String>,
    /// `self.CURRENCY_FORMS`, in Python's insertion order. See
    /// [`build_currency_forms`] for why order matters.
    currency_forms: Vec<(&'static str, CurrencyForms)>,
}

impl Default for LangRw {
    fn default() -> Self {
        Self::new()
    }
}

impl LangRw {
    pub fn new() -> Self {
        LangRw {
            exclude_title: EXCLUDE_TITLE.iter().map(|s| s.to_string()).collect(),
            // Built once here, never per call.
            currency_forms: build_currency_forms(),
        }
    }
}

impl Lang for LangRw {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "RWF"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    // `cards`/`maxval`/`merge` intentionally left at their trait defaults: RW
    // never builds a card table (see module docs) and `to_cardinal` below
    // bypasses the splitnum/clean engine entirely.

    fn negword(&self) -> &str {
        // Trailing space is intentional — verbatim from the Python source.
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "akadomo"
    }

    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Note: unlike `Num2Word_Base.to_cardinal`, RW's override never calls
        // `self.title()`. `is_title` is False anyway, so this is unobservable,
        // but the omission is reproduced rather than papered over.
        Ok(cardinal(value))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // if number == 1: return "mbere"
        // return "wa " + self.to_cardinal(number)
        //
        // No `verify_ordinal` call: negatives and zero pass straight through
        // (quirk 2).
        if value.is_one() {
            return Ok("mbere".to_string());
        }
        Ok(format!("wa {}", cardinal(value)))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // return "wa " + str(number)
        // Overrides the base's bare `return value`; negatives keep their minus,
        // e.g. -1 -> "wa -1".
        Ok(format!("wa {}", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // def to_year(self, val, longval=True): return self.to_cardinal(val)
        // `longval` is accepted and ignored. Identical to the base default, but
        // spelled out because Python spells it out.
        Ok(cardinal(value))
    }

    /// `to_cardinal(float/Decimal)` — the **full** routing, whole values
    /// included. RW's `to_cardinal` reads `str(number)`, so a whole-valued
    /// float keeps its ".0" tail ("gatanu akadomo zeru") and an exponent-form
    /// repr raises ValueError; the base default's whole → integer-path route
    /// would get both wrong. See the module docs' float-routing section.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// Port of `Num2Word_RW.to_cardinal`'s float/Decimal handling.
    ///
    /// RW overrides `to_cardinal` and does the non-integer work **inline on
    /// `str(number)`** — it never reaches `Num2Word_Base.to_cardinal_float` or
    /// `float2tuple`. So the trait default (`default_to_cardinal_float`), which
    /// *does* go through `float2tuple`, is the wrong algorithm here: it matches
    /// every clean corpus row by luck but diverges on `-0.0` (no negword, since
    /// its `is_negative` is `value < 0`), on exponent-form reprs (renders words
    /// where Python raises ValueError) and on high-precision floats whose
    /// binary `post` picks up an artefact `str` never shows. This override
    /// reconstructs `str(number)` exactly ([`python_str`]) and runs RW's string
    /// algorithm on it — see [`cardinal_from_str`].
    ///
    /// `precision_override` (the `precision=` kwarg) is accepted and dropped:
    /// RW's `to_cardinal` reads `str(number)` and never consults
    /// `self.precision`, so the kwarg has no effect — confirmed against the
    /// live interpreter, where `num2words(2.675, lang="rw", precision=1)`
    /// equals the un-kwarg'd result.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        cardinal_from_str(&python_str(value))
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `"mbere" if number == 1 else "wa " + to_cardinal(number)` — the `== 1`
    /// test is *numeric*, so `1.0` and `Decimal("1.00")` return "mbere" too
    /// (corpus: `ordinal 1.0` → "mbere"). Everything else takes the cardinal's
    /// string algorithm with the "wa " prefix, ValueErrors propagating
    /// unchanged (`to_ordinal(1e16)` raises before "wa " is prepended).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // `as_whole_int` is None for fractional values and NaN/±inf, all of
        // which compare != 1 in Python as well.
        if value.as_whole_int().is_some_and(|i| i.is_one()) {
            return Ok("mbere".to_string());
        }
        Ok(format!("wa {}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `"wa " + str(number)`, exactly as the
    /// integer overload — floats are *not* an error here, so
    /// `to_ordinal_num(1e16)` == "wa 1e+16" while `to_ordinal(1e16)` raises.
    /// `repr_str` is Python's own `str(number)`, supplied by the binding.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("wa {}", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is a bare `to_cardinal` alias, so
    /// floats route through the same string algorithm (and raise the same
    /// ValueErrors). Identical to the trait default — which also calls
    /// `cardinal_float_entry` — but spelled out because Python spells it out.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — base's `Decimal(value)`, with one
    /// deliberate deviation: `Decimal("Infinity")` parses fine in Python and
    /// the ValueError only fires later, inside RW's `int("Infinity")`. The
    /// Rust dispatcher hard-codes `ParsedNumber::Inf` → OverflowError (the
    /// base-class `int(Decimal('Infinity'))` behaviour) before any language
    /// hook runs, so the ValueError is surfaced here at parse time instead —
    /// same observable type for every mode that reaches `to_cardinal`.
    /// Known divergence (`to_ordinal_num("Infinity")`) documented in the
    /// module docs. NaN is left alone: the dispatcher already reports
    /// ValueError for it, matching Python's `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                // str(Decimal("Infinity")) is "Infinity"; the sign is sliced
                // off (n[1:]) before int() sees it, so both signs quote the
                // same literal.
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // RW overrides `to_currency` and `pluralize`, and defines `CURRENCY_FORMS`.
    // Everything else on the currency path is inherited from `Num2Word_Base`
    // unchanged and the trait defaults already mirror it:
    //
    //   * `to_cheque`      -> currency::default_to_cheque (see below).
    //   * `_money_verbose` -> `self.to_cardinal(number)`; the trait default is
    //                         exactly that, and dispatches to RW's override.
    //   * `_cents_verbose` / `_cents_terse` -> unreachable (quirk 10).
    //   * `CURRENCY_PRECISION` / `CURRENCY_ADJECTIVES` -> RW defines neither and
    //     `Num2Word_Base`'s are `{}`, so `currency_precision` stays at the
    //     trait's 100 and `currency_adjective` at `None`.
    //   * `cardinal_from_decimal` -> left at its default. RW can never reach the
    //     fractional-cents branch: `split_str_parts` truncates to whole cents,
    //     and RW does not use `default_to_currency`'s float path at all.

    fn lang_name(&self) -> &str {
        "Num2Word_RW"
    }

    /// `CURRENCY_FORMS[code]`, as a *missing-is-None* lookup.
    ///
    /// This is the strict `[]` semantics `Num2Word_Base.to_cheque` needs — it
    /// catches KeyError and re-raises NotImplementedError, which
    /// `currency::default_to_cheque` reproduces from the `None`. RW's own
    /// `to_currency` deliberately does *not* go through this failure mode; it
    /// falls back to RWF instead (quirk 7).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms
            .iter()
            .find(|(c, _)| *c == code)
            .map(|(_, f)| f)
    }

    /// `Num2Word_RW.pluralize`:
    ///
    /// ```python
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code in practice — RW's `to_currency` inlines its own plural choice
    /// (see [`plural_form`]) and `to_cheque` takes `cr1[-1]` unconditionally, so
    /// nothing in the library calls this. Ported because Python defines it, and
    /// because it is an observable public method.
    ///
    /// Note it differs from the inlined rule: `forms[-1]` (last) rather than
    /// `forms[1]` (second), and an empty tuple yields `""` instead of raising.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        if n.is_one() {
            Ok(forms[0].clone())
        } else {
            Ok(forms[forms.len() - 1].clone())
        }
    }

    /// Port of `Num2Word_RW.to_currency`:
    ///
    /// ```python
    /// def to_currency(self, val, currency="RWF", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// Note what is *absent*: no `pluralize`, no `parse_currency_parts`, no
    /// `CURRENCY_PRECISION`, no `has_decimal` guard, and no KeyError path. The
    /// int-vs-float split `base.to_currency` makes with `isinstance(val, int)`
    /// falls out of `str(val)` here instead — `str(5)` has no ".", so `right`
    /// stays 0, while `str(5.0)` == "5.0" gives `parts[1] == "0"` and therefore
    /// `right == 0` as well. Both suppress the cents segment, which is why
    /// `1.0` renders "rimwe iyero" rather than base's "one euro, zero cents".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `adjective` is accepted and never read — quirk 11.
        //
        // `separator: None` means the caller omitted the kwarg; the trait
        // resolves it through `default_separator`, which is RW's `" "`.
        let separator = separator.unwrap_or(self.default_separator());

        // is_negative = val < 0
        //
        // Signed zero matters: `-0.0 < 0` is False in Python, so `to_currency(-0.0)`
        // is "zeru idolari" with no negword. `BigDecimal` has no signed zero, so
        // `CurrencyValue::is_negative` agrees for free.
        let is_negative = val.is_negative();

        // val = abs(val); parts = str(val).split(".")
        let (left, right) = match val {
            // str() of an int never contains ".", so `len(parts) == 1` and
            // `right` stays 0.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value, .. } => split_str_parts(&value.abs()),
        };

        // cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])
        //
        // `.get(..., default)`, not `[...]`: an unknown code renders as RWF
        // rather than raising (quirk 7). `currency_forms[0]` is `values()[0]`.
        let forms = self
            .currency_forms(currency)
            .unwrap_or(&self.currency_forms[0].1);

        // result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
        //
        // `_int_to_word`, not `to_cardinal`: no negword handling, and `left` is
        // non-negative by construction. Above 10^9 it degrades to bare digits
        // (quirk 1), so e.g. Decimal("10000000000000000") -> "10000000000000000
        // idolari".
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            plural_form(&forms.unit, &left)?
        );

        // if cents and right:
        //     result += separator + self._int_to_word(right) + " " + (cr2[1] if right != 1 else cr2[0])
        //
        // Gated on `right` being non-zero, not on the input having a decimal
        // point — and `separator` brings its own spacing or none (quirk 12).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(&plural_form(&forms.subunit, &right)?);
        }

        // if is_negative: result = self.negword + result
        //
        // Raw concatenation of the trailing-space negword (quirk 4), so
        // "munsi ya " + "icumi …" lands correctly spaced.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // return result.strip()
        //
        // A no-op in every reachable case: nothing in the assembled string is
        // padded at either end. Reproduced because Python does it.
        Ok(result.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors `bench/diff_test.py::_currency_call`: the corpus `arg` is
    /// `repr(value)`, `is_int` is "no dot and no exponent", and the harness
    /// passes `separator=None` so `default_separator` (" ") applies.
    fn cur(arg: &str, code: &str) -> String {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangRw::new()
            .to_currency(&v, code, true, None, false)
            .unwrap()
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangRw::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// The full frozen-corpus `currency:` block for `rw`, byte for byte.
    #[test]
    fn corpus_currency_rows() {
        // Known codes: EUR / USD.
        assert_eq!(cur("0", "EUR"), "zeru iyero");
        assert_eq!(cur("1", "EUR"), "rimwe iyero");
        assert_eq!(cur("2", "EUR"), "kabiri iyero");
        assert_eq!(cur("100", "EUR"), "ijana iyero");
        assert_eq!(
            cur("12.34", "EUR"),
            "icumi na kabiri iyero mirongo itatu na kane sentime"
        );
        assert_eq!(cur("0.01", "EUR"), "zeru iyero rimwe sentime");
        // A float that is whole: no cents segment, because parts[1] == "0".
        assert_eq!(cur("1.0", "EUR"), "rimwe iyero");
        assert_eq!(
            cur("99.99", "EUR"),
            "mirongo cyenda na icyenda iyero mirongo cyenda na icyenda sentime"
        );
        assert_eq!(
            cur("1234.56", "EUR"),
            "igihumbi na kabiri ijana na mirongo itatu na kane iyero \
             mirongo itanu na gatandatu sentime"
        );
        assert_eq!(
            cur("-12.34", "EUR"),
            "munsi ya icumi na kabiri iyero mirongo itatu na kane sentime"
        );
        assert_eq!(cur("1000000", "EUR"), "miliyoni iyero");
        assert_eq!(cur("0.5", "EUR"), "zeru iyero mirongo itanu sentime");

        assert_eq!(cur("0", "USD"), "zeru idolari");
        assert_eq!(cur("1", "USD"), "rimwe idolari");
        assert_eq!(
            cur("12.34", "USD"),
            "icumi na kabiri idolari mirongo itatu na kane sentime"
        );
        assert_eq!(cur("0.01", "USD"), "zeru idolari rimwe sentime");
        assert_eq!(cur("1.0", "USD"), "rimwe idolari");
        assert_eq!(cur("1000000", "USD"), "miliyoni idolari");
    }

    /// Quirk 7/8: every code RW does not define renders as RWF, including the
    /// 3-decimal (KWD/BHD) and 0-decimal (JPY) ones, whose divisors RW never
    /// looks at. All corpus-confirmed.
    #[test]
    fn corpus_unknown_codes_fall_back_to_rwf() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert_eq!(cur("0", code), "zeru amafaranga");
            assert_eq!(cur("1", code), "rimwe amafaranga");
            assert_eq!(cur("2", code), "kabiri amafaranga");
            assert_eq!(cur("100", code), "ijana amafaranga");
            assert_eq!(
                cur("12.34", code),
                "icumi na kabiri amafaranga mirongo itatu na kane sentime"
            );
            assert_eq!(cur("0.01", code), "zeru amafaranga rimwe sentime");
            assert_eq!(cur("1.0", code), "rimwe amafaranga");
            assert_eq!(
                cur("99.99", code),
                "mirongo cyenda na icyenda amafaranga mirongo cyenda na icyenda sentime"
            );
            assert_eq!(
                cur("1234.56", code),
                "igihumbi na kabiri ijana na mirongo itatu na kane amafaranga \
                 mirongo itanu na gatandatu sentime"
            );
            assert_eq!(
                cur("-12.34", code),
                "munsi ya icumi na kabiri amafaranga mirongo itatu na kane sentime"
            );
            assert_eq!(cur("1000000", code), "miliyoni amafaranga");
            assert_eq!(cur("0.5", code), "zeru amafaranga mirongo itanu sentime");
        }
    }

    /// The full frozen-corpus `cheque:` block for `rw`.
    #[test]
    fn corpus_cheque_rows() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "IGIHUMBI NA KABIRI IJANA NA MIRONGO ITATU NA KANE AND 56/100 IYERO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "IGIHUMBI NA KABIRI IJANA NA MIRONGO ITATU NA KANE AND 56/100 IDOLARI"
        );
        // Unlike to_currency, to_cheque uses the base class's `[]` lookup.
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!(
                        "Currency code \"{}\" not implemented for \"Num2Word_RW\"",
                        code
                    )
                ),
                other => panic!("{code}: expected NotImplemented, got {other:?}"),
            }
        }
    }

    /// Cross-checked against the live interpreter, not the corpus.
    #[test]
    fn python_quirks() {
        // Quirk 9: cents truncate. 12.345 -> "34", not "35"...
        assert_eq!(
            cur("12.345", "USD"),
            "icumi na kabiri idolari mirongo itatu na kane sentime"
        );
        // ...and 1.005 truncates to 0 cents, which then trips `if cents and
        // right` and drops the segment entirely. Base would say one cent.
        assert_eq!(cur("1.005", "USD"), "rimwe idolari");
        // One fractional digit is ljust-padded: "3" -> "30".
        assert_eq!(cur("12.3", "USD"), "icumi na kabiri idolari mirongo itatu sentime");
        assert_eq!(
            cur("2.999", "USD"),
            "kabiri idolari mirongo cyenda na icyenda sentime"
        );
        // `-0.0 < 0` is False in Python: no negword.
        assert_eq!(cur("-0.0", "USD"), "zeru idolari");
        // Quirk 4: negword's trailing space, on both an int and a float.
        assert_eq!(cur("-1", "RWF"), "munsi ya rimwe amafaranga");
        assert_eq!(cur("-1.0", "RWF"), "munsi ya rimwe amafaranga");
        // Quirk 1 reached through the currency path: _int_to_word degrades to
        // bare digits at 10^9, so a large unit count is printed as numerals.
        let v = CurrencyValue::parse("10000000000000000", false, false, false).unwrap();
        assert_eq!(
            LangRw::new()
                .to_currency(&v, "USD", true, None, false)
                .unwrap(),
            "10000000000000000 idolari"
        );
    }

    /// Knobs the corpus never exercises (diff_test always passes cents=True,
    /// separator=None, adjective=False). Values from the live interpreter.
    #[test]
    fn untested_knobs_match_python() {
        let l = LangRw::new();
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        // Quirk 10: cents=False drops the segment; it does NOT use _cents_terse.
        assert_eq!(
            l.to_currency(&v, "USD", false, None, false).unwrap(),
            "icumi na kabiri idolari"
        );
        // Quirk 12: separator is concatenated raw — " na" runs into the word.
        assert_eq!(
            l.to_currency(&v, "USD", true, Some(" na"), false).unwrap(),
            "icumi na kabiri idolari namirongo itatu na kane sentime"
        );
        // Quirk 11: adjective=True changes nothing.
        assert_eq!(
            l.to_currency(&v, "USD", true, None, true).unwrap(),
            "icumi na kabiri idolari mirongo itatu na kane sentime"
        );
    }

    /// `Num2Word_RW.pluralize` is unreachable from the library, but it is a
    /// public method and it does not follow the rule to_currency inlines.
    #[test]
    fn pluralize_is_last_not_second() {
        let l = LangRw::new();
        let forms: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        assert_eq!(l.pluralize(&BigInt::from(1), &forms).unwrap(), "a");
        // forms[-1], not forms[1].
        assert_eq!(l.pluralize(&BigInt::from(2), &forms).unwrap(), "c");
        // `if not forms: return ""` — no raise.
        assert_eq!(l.pluralize(&BigInt::from(2), &[]).unwrap(), "");
    }

    /// Drive a float `cardinal` row the way the binding does: raw f64 plus the
    /// Python-side precision `abs(Decimal(repr(v)).as_tuple().exponent)`.
    fn cf(value: f64, precision: u32) -> String {
        LangRw::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// Drive a `cardinal_dec` row: BigDecimal parsed from str(Decimal), with the
    /// same precision the binding passes.
    fn cd(decimal_str: &str, precision: u32) -> String {
        LangRw::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(decimal_str).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    /// Every `cardinal` row with a float `arg` in the frozen corpus, byte for
    /// byte. `precision` is `abs(Decimal(repr(v)).as_tuple().exponent)`.
    #[test]
    fn corpus_cardinal_float_rows() {
        assert_eq!(cf(0.0, 1), "zeru akadomo zeru");
        assert_eq!(cf(0.5, 1), "zeru akadomo gatanu");
        assert_eq!(cf(1.0, 1), "rimwe akadomo zeru");
        assert_eq!(cf(1.5, 1), "rimwe akadomo gatanu");
        assert_eq!(cf(2.25, 2), "kabiri akadomo kabiri gatanu");
        assert_eq!(cf(3.14, 2), "gatatu akadomo rimwe kane");
        assert_eq!(cf(0.01, 2), "zeru akadomo zeru rimwe");
        assert_eq!(cf(0.1, 1), "zeru akadomo rimwe");
        assert_eq!(cf(0.99, 2), "zeru akadomo icyenda icyenda");
        assert_eq!(cf(1.01, 2), "rimwe akadomo zeru rimwe");
        assert_eq!(cf(12.34, 2), "icumi na kabiri akadomo gatatu kane");
        assert_eq!(
            cf(99.99, 2),
            "mirongo cyenda na icyenda akadomo icyenda icyenda"
        );
        assert_eq!(cf(100.5, 1), "ijana akadomo gatanu");
        assert_eq!(
            cf(1234.56, 2),
            "igihumbi na kabiri ijana na mirongo itatu na kane akadomo gatanu gatandatu"
        );
        assert_eq!(cf(-0.5, 1), "munsi ya zeru akadomo gatanu");
        assert_eq!(cf(-1.5, 1), "munsi ya rimwe akadomo gatanu");
        assert_eq!(
            cf(-12.34, 2),
            "munsi ya icumi na kabiri akadomo gatatu kane"
        );
        // The f64-artefact cases: RW spells the *repr* digits, never the binary
        // `post`. 1.005 (stored 1.00499…) still formats "005", 2.675 (stored
        // 2.67499…) still formats "675" — no float2tuple rounding involved.
        assert_eq!(cf(1.005, 3), "rimwe akadomo zeru zeru gatanu");
        assert_eq!(cf(2.675, 3), "kabiri akadomo gatandatu karindwi gatanu");
    }

    /// Every `cardinal_dec` (Decimal input) row in the frozen corpus.
    #[test]
    fn corpus_cardinal_dec_rows() {
        assert_eq!(cd("0.01", 2), "zeru akadomo zeru rimwe");
        // Trailing zero preserved: str(Decimal("1.10")) == "1.10" -> "1","10".
        assert_eq!(cd("1.10", 2), "rimwe akadomo rimwe zeru");
        assert_eq!(
            cd("12.345", 3),
            "icumi na kabiri akadomo gatatu kane gatanu"
        );
        // Integer part above 10^9 degrades to bare digits (quirk 1).
        assert_eq!(
            cd("98746251323029.99", 2),
            "98746251323029 akadomo icyenda icyenda"
        );
        assert_eq!(cd("0.001", 3), "zeru akadomo zeru zeru rimwe");
    }

    /// Cross-checked against the live interpreter, not the corpus.
    #[test]
    fn cardinal_float_quirks() {
        // str(-0.0) == "-0.0": the sign bit alone earns the negword, which the
        // trait default (whose is_negative is `value < 0`) would miss.
        assert_eq!(cf(-0.0, 1), "munsi ya zeru akadomo zeru");
        // 5.0 -> repr "5.0", precision 1: a whole-valued float still spells the
        // "0" fractional digit.
        assert_eq!(cf(5.0, 1), "gatanu akadomo zeru");
        // Decimal trailing zeros: str(Decimal("12.340")) == "12.340".
        assert_eq!(cd("12.340", 3), "icumi na kabiri akadomo gatatu kane zeru");
        // Large float whose integer part crosses 10^9 (repr "123456789.5").
        assert_eq!(
            cf(123456789.5, 1),
            "ijana na makumyabiri na gatatu miliyoni na kane ijana na mirongo \
             itanu na gatandatu igihumbi na karindwi ijana na mirongo inani na \
             icyenda akadomo gatanu"
        );
        // precision= is dropped: RW never reads self.precision.
        assert_eq!(
            LangRw::new()
                .to_cardinal_float(
                    &FloatValue::Float { value: 2.675, precision: 3 },
                    Some(1),
                )
                .unwrap(),
            "kabiri akadomo gatandatu karindwi gatanu"
        );
        // A precision-0 Decimal (str "5", no point) takes the bare-integer arm.
        assert_eq!(cd("5", 0), "gatanu");
    }

    fn fv_f(value: f64, precision: u32) -> FloatValue {
        FloatValue::Float { value, precision }
    }

    fn fv_d(s: &str, precision: u32) -> FloatValue {
        FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision,
        }
    }

    /// The wholefloat-corpus routing rows: every float/Decimal — whole values
    /// included — takes the string algorithm, so ".0" tails are spoken and
    /// exponent-form reprs raise ValueError from `int()`.
    #[test]
    fn corpus_float_entry_rows() {
        let l = LangRw::new();
        // Whole floats keep their ".0" (str(5.0) == "5.0").
        assert_eq!(
            l.cardinal_float_entry(&fv_f(5.0, 1), None).unwrap(),
            "gatanu akadomo zeru"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv_f(-0.0, 1), None).unwrap(),
            "munsi ya zeru akadomo zeru"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv_f(1234.0, 1), None).unwrap(),
            "igihumbi na kabiri ijana na mirongo itatu na kane akadomo zeru"
        );
        // Above 10^9 the integer field degrades to bare digits (quirk 1).
        assert_eq!(
            l.cardinal_float_entry(&fv_f(1e9, 1), None).unwrap(),
            "1000000000 akadomo zeru"
        );
        // Decimal without a visible point takes the integer words...
        assert_eq!(l.cardinal_float_entry(&fv_d("5", 0), None).unwrap(), "gatanu");
        // ...while trailing zeros survive str(Decimal).
        assert_eq!(
            l.cardinal_float_entry(&fv_d("5.00", 2), None).unwrap(),
            "gatanu akadomo zeru zeru"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv_d("12345.000", 3), None).unwrap(),
            "icumi na kabiri igihumbi na gatatu ijana na mirongo ine na gatanu \
             akadomo zeru zeru zeru"
        );
        // Exponent-form float reprs: str(1e16) == "1e+16" — no ".", so
        // int() raises ValueError (the dispatcher never sends these here,
        // #211). str(Decimal) is written out ("100"), so Decimals read.
        for v in [
            l.cardinal_float_entry(&fv_f(1e16, 16), None),
            l.cardinal_float_entry(&fv_f(1e20, 20), None),
        ] {
            assert!(matches!(v, Err(N2WError::Value(_))), "{v:?}");
        }
        assert_eq!(
            l.cardinal_float_entry(&fv_d("1E+2", 0), None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );
    }

    /// `to_ordinal` on floats: "mbere" for == 1, else "wa " + cardinal, with
    /// the cardinal's ValueErrors propagating.
    #[test]
    fn corpus_ordinal_entry_rows() {
        let l = LangRw::new();
        // number == 1 is numeric: 1.0 and Decimal("1.00") both hit "mbere".
        assert_eq!(l.ordinal_float_entry(&fv_f(1.0, 1)).unwrap(), "mbere");
        assert_eq!(l.ordinal_float_entry(&fv_d("1.00", 2)).unwrap(), "mbere");
        assert_eq!(
            l.ordinal_float_entry(&fv_f(2.0, 1)).unwrap(),
            "wa kabiri akadomo zeru"
        );
        assert_eq!(
            l.ordinal_float_entry(&fv_f(-0.0, 1)).unwrap(),
            "wa munsi ya zeru akadomo zeru"
        );
        assert_eq!(l.ordinal_float_entry(&fv_d("0", 0)).unwrap(), "wa zeru");
        assert_eq!(l.ordinal_float_entry(&fv_d("5", 0)).unwrap(), "wa gatanu");
        assert_eq!(
            l.ordinal_float_entry(&fv_f(0.5, 1)).unwrap(),
            "wa zeru akadomo gatanu"
        );
        assert!(matches!(
            l.ordinal_float_entry(&fv_f(1e16, 16)),
            Err(N2WError::Value(_))
        ));
        // to_ordinal_num floats are NOT an error: "wa " + str(number), where
        // str(number) is the binding-supplied repr.
        assert_eq!(
            l.ordinal_num_float_entry(&fv_f(1e16, 16), "1e+16").unwrap(),
            "wa 1e+16"
        );
        assert_eq!(
            l.ordinal_num_float_entry(&fv_d("5.00", 2), "5.00").unwrap(),
            "wa 5.00"
        );
        // to_year is a to_cardinal alias.
        assert_eq!(
            l.year_float_entry(&fv_f(5.0, 1)).unwrap(),
            "gatanu akadomo zeru"
        );
        assert_eq!(
            l.year_float_entry(&fv_d("1E+2", 0)).unwrap(),
            l.to_year(&BigInt::from(100)).unwrap()
        );
    }

    /// String inputs: "1e3" parses to Decimal('1E+3'), read as 1000 (#211); "Infinity" is intercepted in str_to_number
    /// (the dispatcher would otherwise report the base class's OverflowError).
    #[test]
    fn corpus_string_rows() {
        let l = LangRw::new();
        let parsed = l.str_to_number("1e3").unwrap();
        match parsed {
            ParsedNumber::Dec(d) => {
                // #211: str(Decimal('1E+3')) is written out as "1000".
                let fv = FloatValue::Decimal { value: d, precision: 0 };
                assert_eq!(
                    l.cardinal_float_entry(&fv, None).unwrap(),
                    l.to_cardinal(&BigInt::from(1000)).unwrap()
                );
            }
            other => panic!("expected Dec, got {other:?}"),
        }
        assert!(matches!(
            l.str_to_number("Infinity"),
            Err(N2WError::Value(_))
        ));
        assert!(matches!(
            l.str_to_number("-Infinity"),
            Err(N2WError::Value(_))
        ));
        // NaN stays a ParsedNumber::NaN — the dispatcher's ValueError already
        // matches Python's int("NaN") type.
        assert!(matches!(l.str_to_number("NaN"), Ok(ParsedNumber::NaN)));
    }

    /// [`python_float_repr`] against CPython ground truth for the corpus range.
    #[test]
    fn float_repr_matches_cpython() {
        for (v, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.0, "1.0"),
            (0.5, "0.5"),
            (1234.0, "1234.0"),
            (1000000000000000.0, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1e20, "1e+20"),
            (2.675, "2.675"),
            (1.005, "1.005"),
            (123456789.5, "123456789.5"),
        ] {
            assert_eq!(python_float_repr(v), want);
        }
    }

    /// The load-bearing assumption in `split_str_parts`: `with_scale(0)`
    /// truncates rather than rounding. currency.rs already relies on this, but
    /// RW's truncate-don't-round cents rule depends on it directly.
    #[test]
    fn with_scale_zero_truncates() {
        for (s, want) in [("0.9", 0), ("1.5", 1), ("2.5", 2), ("34.99", 34)] {
            assert_eq!(
                BigDecimal::from_str(s)
                    .unwrap()
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0,
                BigInt::from(want),
                "with_scale(0) on {s}"
            );
        }
    }
}
