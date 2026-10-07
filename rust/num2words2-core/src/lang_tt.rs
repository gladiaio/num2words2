//! Port of `lang_TT.py` (Tatar).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "миллиард" (10^9) and "триллион" (10^12) (Wiktionary), composed
//! like the million arm, and raises `OverflowError` from 10^15, which
//! `maxval()` reports. Where the notes below describe the digit fallback
//! ("1000000000", "no overflow check"), they describe Python; that arm is now
//! unreachable.
//!
//! Shape: **self-contained**. `Num2Word_TT` subclasses `Num2Word_Base`, but
//! its `setup` defines only flat `ones`/`tens` lists plus three scale words —
//! no `high_numwords`/`mid_numwords`/`low_numwords`. Python therefore never
//! builds `self.cards` and never sets `MAXVAL`, and `to_cardinal` is
//! overridden outright to drive a hand-written `_int_to_word` recursion.
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here,
//! and there is **no overflow check at all** — see bug 2 below for what
//! happens past the top of the scale table.
//!
//! Inheritance chain: `Num2Word_TT` -> `Num2Word_Base`. Every one of the four
//! in-scope methods is overridden by TT, so nothing is inherited from
//! `Num2Word_Base` on the in-scope paths:
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden: `to_cardinal(n)` + a harmonised ending
//!     (upstream: a fixed "-нче"; fixed, see bug 5)
//!   * `to_ordinal_num` — overridden: `str(n) + "."` (the base returns `n`
//!     bare, so the trait default is *not* correct here)
//!   * `to_year`        — overridden: `to_cardinal(val)`, ignoring `longval`
//!
//! No cross-call mutable state: `setup` only populates constant tables, and
//! none of the four methods writes to `self`. Nothing for the dispatcher to
//! skip.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following look wrong and are
//! exactly what Python emits — each is confirmed against the frozen corpus:
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** Python's `_int_to_word`
//!    opens with `return self.ones[0] if self.ones[0] else "zero"`. `ones[0]`
//!    is the empty string (a placeholder so the list can be indexed by
//!    digit), which is falsy, so Python always answered with the English
//!    "zero". This port says the Tatar "нуль" (as the Bashkir module does):
//!    `to_cardinal(0)` == "нуль" and `to_ordinal(0)` == "нуленче".
//! 2. **Numbers >= 10^9 come back as digits.** The `elif` ladder in
//!    `_int_to_word` stops at `number < 1000000000` and the trailing `else`
//!    is `return str(number)` — a "fallback for very large numbers" that
//!    emits no words whatsoever. So `to_cardinal(10**9)` == "1000000000" and
//!    `to_ordinal(10**12)` == "1000000000000-нче". No exception, no billion
//!    word: the scale table simply has no entry above `million`. This is why
//!    the value must stay a `BigInt` — the fallback is `str()` on an
//!    unbounded int.
//! 3. **Negative and decimal words (fixed, #154).** Python's `setup` sets the
//!    English `negword = "minus "` and `pointword = "point"`. This port uses
//!    "минус " and "өтер" (Tatar "comma": Tatar writes a decimal comma; the
//!    Bashkir module likewise uses өтөр), so `to_cardinal(-1)` == "минус бер"
//!    and `1.5` == "бер өтер биш".
//! 4. **`бер йөз` for 100.** Hundreds always carry an explicit "бер" ("one"),
//!    including the bare hundred: `result = self.ones[hundreds_val] + " " +
//!    self.hundred` with no `hundreds_val == 1` special case. Likewise
//!    `to_cardinal(1000)` == "бер мең" and `to_cardinal(10**6)` ==
//!    "бер миллион". Idiomatic or not, it is what ships.
//! 5. **Ordinals (fixed, gladiaio/num2words2#148).** Python glued "-нче" onto
//!    the cardinal unconditionally ("бер-нче", "ун-нче", "өч-нче"). The port
//!    harmonises the ending on the last word instead ("беренче", "унынчы",
//!    "өченче", "кырыгынчы"); see [`join_ordinal`]. The digit fallback of
//!    bug 2 still flows through, hyphenated ("1000000000-нче").
//!
//! # Error variants
//!
//! For integer input every one of the four integer paths is total:
//! `to_cardinal` cannot raise (the `else` fallback of bug 2 catches everything
//! the ladder misses, and there is no `MAXVAL` check to overflow), and
//! `to_ordinal` / `to_ordinal_num` / `to_year` are pure concatenation over it.
//! The corpus agrees: every `cardinal`/`ordinal`/`ordinal_num`/`year` row for
//! "tt" is `"ok": true`. The remaining `ok: false` rows are `fraction`
//! (TypeError, out of scope) and `cheque` (NotImplementedError, handled below).
//!
//! The currency surface adds exactly two error paths, both ported below:
//! `NotImplemented` out of `to_cheque` for a code outside `CURRENCY_FORMS`,
//! and `Value` out of `to_currency` for a float whose `repr` is exponential
//! (see [`python_str_float`]). `to_currency` itself can never raise
//! NotImplementedError — see bug 6.
//!
//! # Currency: what Python actually defines
//!
//! `Num2Word_TT` declares its **own** three-entry `CURRENCY_FORMS` as a class
//! attribute. It does *not* descend from `Num2Word_EUR`, so the shared-dict
//! mutation `Num2Word_EN.__init__` performs (`self.CURRENCY_FORMS["EUR"] =
//! ...`, which rewrites EUR's table in place for the 16 classes that read it)
//! cannot reach TT. Its EUR entry is its own literal. Confirmed against the
//! live interpreter rather than the source:
//!
//! ```text
//! CONVERTER_CLASSES['tt'].CURRENCY_FORMS
//!   {'RUB': (('сум', 'сум'), ('тиен', 'тиен')),
//!    'USD': (('dollar', 'dollars'), ('cent', 'cents')),
//!    'EUR': (('euro', 'euros'), ('cent', 'cents'))}
//!   insertion order: ['RUB', 'USD', 'EUR']
//! CONVERTER_CLASSES['tt'].CURRENCY_PRECISION   {}
//! CONVERTER_CLASSES['tt'].CURRENCY_ADJECTIVES  {}
//! ```
//!
//! `to_currency` is overridden **wholesale** and shares no code with
//! `Num2Word_Base.to_currency`: it never reaches `pluralize`,
//! `_money_verbose`, `_cents_verbose`, `_cents_terse`,
//! `parse_currency_parts`, `prefix_currency` or `CURRENCY_PRECISION`. Those
//! hooks are therefore left at their trait defaults here.
//!
//! `to_cheque` is *not* overridden, so `Num2Word_Base.to_cheque` runs and
//! [`crate::currency::default_to_cheque`] serves it unchanged. It needs only
//! [`Lang::currency_forms`] (whose `None` is what turns Python's `KeyError`
//! into `NotImplementedError`), [`Lang::lang_name`] (for that message), the
//! default `currency_precision` of 100 — `CURRENCY_PRECISION` is the empty
//! dict inherited from `Num2Word_Base`, never mutated, so `.get(code, 100)` is
//! 100 for every code — and the default `money_verbose`, which routes back
//! through TT's own `to_cardinal`.
//!
//! # Faithfully reproduced Python bugs, currency edition
//!
//! 6. **An unknown currency code silently becomes roubles.**
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    falls back to the *first* entry instead of raising, so `to_currency` has
//!    no NotImplementedError path at all and GBP/JPY/CHF/... all render with
//!    Tatar `сум`/`тиен`. The corpus pins this:
//!    `{"lang": "tt", "to": "currency:GBP", "arg": "1", "out": "бер сум"}`.
//!    `to_cheque` — which is base's, not TT's — does *not* share the fallback
//!    and raises for those same codes, so the two surfaces disagree about
//!    which codes exist.
//! 7. **`CURRENCY_PRECISION` is ignored by `to_currency`.** `parts[1][:2]` is
//!    hardcoded, so a 3-decimal currency loses its mils and a 0-decimal one
//!    gains cents that cannot exist. Both follow from bug 6 anyway (neither
//!    KWD nor JPY is in the table), and TT's dict is empty regardless, so even
//!    base's `to_cheque` uses divisor 100 for them.
//! 8. **A float with zero cents drops the segment; base would print it.**
//!    `if cents and right:` is a truthiness test on the *number*, so `1.0`
//!    (`parts[1] == "0"` -> `right == 0`) renders "бер euro", exactly like int
//!    `1`. Base reaches the same output for `1` by a different route
//!    (`isinstance(val, int)`) but renders `1.0` as "one euro, нуль cents".
//!    The `CurrencyValue` arms are still kept distinct here because they
//!    produce different `str(val)` strings ("1" vs "1.0"), which is what the
//!    code branches on.
//! 9. **`cents=False` omits the cents, it does not abbreviate them.** Base's
//!    `cents=False` means "render the subunit as digits" via `_cents_terse`;
//!    TT's `if cents and right:` drops the whole segment.
//! 10. **`adjective` is accepted and completely ignored** — no
//!     `prefix_currency` call, and `CURRENCY_ADJECTIVES` is empty regardless.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use доллар / евро with цент (singular after a
//! numeral). Examples in these docs that quote English nouns record Python's
//! output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.ones`. Index 0 is the empty string — see bug 1; it is never read as
/// a word, only as the (always-falsy) zero guard.
const ONES: [&str; 10] = [
    "", "бер", "ике", "өч", "дүрт", "биш", "алты", "җиде", "сигез", "тугыз",
];

/// `self.tens`. Index 0 is an unused placeholder (the `number < 10` arm of
/// the ladder catches every value that would index it).
const TENS: [&str; 10] = [
    "", "ун", "егерме", "утыз", "кырык", "илле", "алтмыш", "җитмеш", "сиксән", "туксан",
];

const HUNDRED: &str = "йөз";
const THOUSAND: &str = "мең";
const MILLION: &str = "миллион";

/// `self.negword`. Trailing space is significant: Python concatenates it
/// directly onto the number word (`ret + self._int_to_word(...)`). Python has
/// the English "minus "; Tatar here (#154).
const NEGWORD: &str = "минус ";

/// `self.pointword`, used on the float path. Python has the English "point";
/// this port uses the Tatar decimal comma "өтер" (#154).
const POINTWORD: &str = "өтер";

/// What `_int_to_word(0)` returns. Python's English "zero"; Tatar here (#154).
const ZERO_WORD: &str = "нуль";

/// Attach the ordinal ending to the **last word** of a spelled cardinal
/// (gladiaio/num2words2#148). Upstream glued a fixed "-нче" onto the whole
/// cardinal ("бер-нче", "ун-нче"). Tatar ordinals take *-(ы)нчы/-(е)нче*,
/// harmonised for backness only — no rounding, so өч gives өченче
/// (Wiktionary, `Module:number_list/data/tt`):
///
/// * back stem (last vowel а ы о у): *-ынчы* after a consonant, *-нчы* after
///   a vowel — унынчы, алтынчы, миллионынчы;
/// * front stem (ә е и ө ү э): *-енче* / *-нче* — беренче, икенче, йөзенче;
/// * a final soft sign is dropped and the stem counts as front: нуль ->
///   нуленче;
/// * кырык voices its final к: кырыгынчы.
///
/// 123rd is therefore "бер йөз егерме өченче". A cardinal that is not a word
/// at all — the digit fallback above 10^9 — keeps a hyphen
/// ("1000000000-нче").
fn join_ordinal(cardinal: &str) -> String {
    let (head, stem) = match cardinal.rfind(' ') {
        Some(i) => cardinal.split_at(i + 1),
        None => ("", cardinal),
    };
    if let Some(soft) = stem.strip_suffix('ь') {
        return format!("{}{}енче", head, soft);
    }
    let last = match stem.chars().filter(|c| "аыоуяюёәеиөүэ".contains(*c)).last() {
        Some(v) => v,
        None => return format!("{}-нче", cardinal),
    };
    let back = "аыоуяюё".contains(last);
    let after_vowel = stem.ends_with(|c: char| "аыоуяюёәеиөүэ".contains(c));
    let stem = if stem == "кырык" { "кырыг" } else { stem };
    let ending = match (back, after_vowel) {
        (true, true) => "нчы",
        (true, false) => "ынчы",
        (false, true) => "нче",
        (false, false) => "енче",
    };
    format!("{}{}{}", head, stem, ending)
}

/// `Num2Word_TT.to_currency`'s own default: `separator=" "`. Note this is
/// *not* `Num2Word_Base`'s `","` — see [`BASE_DEFAULT_SEPARATOR`].
const SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default separator, which both callers use as
/// a de-facto "caller said nothing" sentinel.
///
/// Python resolves a default argument at the *call site*: omit `separator=`
/// and `Num2Word_TT.to_currency` binds its own `" "`. The `Lang` trait takes
/// `separator: &str` unconditionally, so that resolution has already happened
/// by the time we are called — and both callers hardcode **base**'s default
/// rather than the language's:
///
/// * `num2words2/__init__.py`'s Rust fast path: `kwargs.get("separator", ",")`
/// * `bench/diff_test.py`: `_rust.to_currency(lang, arg, is_int, code, True, ",", False)`
///
/// This matters more for TT than for base, because the two apply the separator
/// differently: base emits `separator + " "` (so `","` gives `"euros, thirty"`)
/// while TT emits the separator *alone* as the whole joint
/// (`result += separator + cents_str`). TT's `" "` default is what supplies
/// that space at all.
///
/// The frozen corpus was generated through the *Python* converter with no
/// `separator=` kwarg, so every "tt" currency row shows TT's `" "`:
///
/// ```text
/// {"lang": "tt", "to": "currency:EUR", "arg": "12.34",
///  "out": "ун ике euros утыз дүрт cents"}
/// ```
///
/// Taking the incoming `","` at face value would render
/// `"ун ике euros,утыз дүрт cents"` — no space at all — and fail all 54 rows
/// that carry cents. So `","` is read back as "unset" and TT's own default
/// restored. This is right across the whole reachable input space bar one
/// case: an explicit `separator=","`, which Python renders with a comma and
/// this renders with a space. A real fix belongs at the boundary (pass
/// `Option<&str>` and let the language supply its default), which is
/// `base.rs` / `currency.rs` / `__init__.py` and out of scope for this file;
/// it is flagged in the report. `lang_oc.rs`, `lang_as.rs`, `lang_ba.rs` and
/// `lang_br.rs` — the other languages whose Python signature carries a
/// non-`","` default — resolve it the same way.
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// Python's `str(value)` for a **non-negative float**, reconstructed from the
/// `BigDecimal` the shim parsed out of that very string.
///
/// `to_currency` does `parts = str(val).split(".")` and then `int()`s the
/// pieces, so it branches on the *notation* `repr` chose, not on the value.
/// `BigDecimal`'s own `Display` is not that function and disagrees in three
/// ways that are reachable here:
///
/// | input `str(v)` | `BigDecimal::to_string()` | Python `str(v)` |
/// |---|---|---|
/// | `1e-05`  | `0.00001` | `1e-05`  |
/// | `5e-324` | `5E-324`  | `5e-324` |
/// | `0.0`    | `0`       | `0.0`    |
///
/// The first is the damaging one: Python feeds `"1e-05"` to `int()` and raises
/// ValueError, where `Display` would hand back a clean `"0.00001"` and invent
/// an answer Python never produces.
///
/// So the notation is derived rather than borrowed. CPython's float repr emits
/// the shortest round-tripping digit string plus a decimal-point position
/// `decpt`, and switches to exponential form iff `decpt <= -4 || decpt > 16`.
/// `as_bigint_and_exponent()` yields `(int_val, scale)` with
/// `value == int_val * 10^-scale`; `int_val` carries no leading zeros, so
/// `decpt == len(int_val) - scale` — invariant to the trailing zeros a literal
/// like `"1.0"` contributes, since they lift `len` and `scale` in lockstep.
/// Verified to round-trip `str(abs(float))` on 10,026 values spanning the
/// subnormals, both notation thresholds and 1e±300.
///
/// Caveat: this is `str(float)`. `__init__.py` also lets a `decimal.Decimal`
/// reach `CurrencyValue::Decimal`, and `str(Decimal)` follows different rules
/// (it preserves the declared scale and uses `E+` notation on other
/// thresholds). No corpus row exercises it; flagged in the report.
fn python_str_float(d: &BigDecimal) -> String {
    // `str(0.0)` is "0.0"; abs() upstream has already folded -0.0 into it.
    if d.is_zero() {
        return "0.0".to_string();
    }

    let (int_val, scale) = d.as_bigint_and_exponent();
    // BigInt's Display of a non-negative value: ASCII digits, no leading zero.
    // Byte and char indices therefore coincide for every slice below.
    let all_digits = int_val.abs().to_string();
    let nd_full = all_digits.len() as i64;
    let digits = all_digits.trim_end_matches('0');
    // Unreachable (a non-zero int_val always keeps a non-zero digit), but the
    // slicing below would panic on an empty string, so it is not left to luck.
    let digits = if digits.is_empty() { "0" } else { digits };
    let nd = digits.len() as i64;
    let decpt = nd_full - scale;

    if decpt <= -4 || decpt > 16 {
        // Exponential: "1e-05", "1.5e-05", "1e+16". The exponent carries an
        // explicit sign and is zero-padded to two digits, no wider.
        let exp = decpt - 1;
        let mant = if nd > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits.to_string()
        };
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        // "0.01" (decpt -1), "0.5" (decpt 0).
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= nd {
        // Integral value: repr keeps a ".0" tail — "1.0", "100.0".
        format!("{}{}.0", digits, "0".repeat((decpt - nd) as usize))
    } else {
        // "12.34" (digits "1234", decpt 2).
        format!("{}.{}", &digits[..decpt as usize], &digits[decpt as usize..])
    }
}

/// Python's bare `int(token)` over a `str(val)` fragment.
///
/// The tokens reaching this are digit runs in every plain-notation case; the
/// only way it fails is an exponential `repr` reaching `to_currency`, where
/// Python raises ValueError with this exact message — `int("1e-05")` for the
/// whole string, or `int("5e")` once `parts[1][:2]` has sliced `"5e-05"` down.
/// `BigInt::from_str`'s own error text is a num-bigint string ("invalid digit
/// found in string"), so the message is built here instead of borrowed.
fn python_int(token: &str) -> Result<BigInt> {
    // `int()` tolerates a leading sign; `str(abs(val))` never has one, but the
    // check mirrors what it accepts rather than what it happens to receive.
    let body = match token.as_bytes().first() {
        // '+'/'-' are ASCII, so slicing past them stays on a char boundary.
        Some(b'+') | Some(b'-') => &token[1..],
        _ => token,
    };
    if body.is_empty() || !body.bytes().all(|c| c.is_ascii_digit()) {
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            token
        )));
    }
    BigInt::from_str(token).map_err(|e| N2WError::Value(e.to_string()))
}

/// Python's `str(number)` for the value that reaches `Num2Word_TT.to_cardinal`
/// on the float/Decimal path.
///
/// TT overrides `to_cardinal` and handles non-integers **inline**, from the
/// *string* form of the number:
///
/// ```python
/// n = str(number).strip()
/// ...
/// left, right = n.split(".", 1)
/// ret += self._int_to_word(int(left)) + " " + self.pointword + " "
/// for digit in right:
///     ret += self._int_to_word(int(digit)) + " "
/// ```
///
/// It never touches `base.float2tuple`, `self.precision`, `round()` or the
/// `< 0.01` artefact heuristic — the fractional digits are read *literally* off
/// `str(number)`. So the two float-path traps (banker's rounding, the
/// `674.9999...` rescue) do **not** apply here: `2.675` renders "алты җиде биш"
/// (repr digits 6·7·5), not the reconstructed-integer 675, and `1.005` keeps
/// its literal "нуль нуль биш". The whole behaviour therefore hinges on
/// reproducing `str(number)` byte for byte, which is what this does.
///
/// * **Float arm** — reproduce CPython's `repr`/`str(float)`. Rust's
///   `format!("{}", f)` is the same shortest-round-trip digit string (never in
///   exponent form for f64), so parsing it into a `BigDecimal` and running it
///   through [`python_str_float`] yields exactly `str(abs(f))`; the sign is put
///   back from the f64's sign bit so `str(-0.0)` stays "-0.0" as in Python.
/// * **Decimal arm** — reproduce `str(Decimal)`, which *preserves the declared
///   scale* (trailing zeros): `Decimal("1.10")` is "1.10", not "1.1". Neither
///   [`python_str_float`] (shortest, trims zeros) nor `BigDecimal::Display`
///   (collapses `0.00` to `0`) does that, so it is rebuilt from
///   `(int_val, scale)` directly. `scale <= 0` is an integer-valued Decimal
///   (e.g. `Decimal("5")` -> "5"), which then flows down the dotless `int(n)`
///   branch, exactly like Python.
fn python_str_number(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => {
            // `str(float('nan'))` / `str(float('inf'))`: no dot, so the caller's
            // dotless `int(n)` branch raises ValueError, matching Python.
            if value.is_nan() {
                return "nan".to_string();
            }
            if value.is_infinite() {
                return if *value < 0.0 { "-inf" } else { "inf" }.to_string();
            }
            // `format!("{}", f)` is shortest-round-trip and always positional
            // for f64 (no 'e'), so it is a valid decimal for `from_str`.
            let s = format!("{}", value.abs());
            let d = BigDecimal::from_str(&s)
                .expect("Rust f64 Display is always a valid decimal literal");
            let body = python_str_float(&d);
            // Sign bit, not `< 0.0`: matches `repr`, which prints "-0.0".
            if value.is_sign_negative() {
                format!("-{}", body)
            } else {
                body
            }
        }
        FloatValue::Decimal { value, .. } => {
            let (int_val, scale) = value.as_bigint_and_exponent();
            let neg = int_val.is_negative();
            let mag = int_val.abs().to_string();
            let body = if scale < 0 {
                // Positive exponent (`Decimal("1E+2")`, `Decimal("1e3")` ->
                // "1E+3"): Python's `str(Decimal)` keeps E-notation, which the
                // caller's dotless branch then feeds to `int()` -> ValueError.
                // `strnum::python_decimal_str` is the CPython `__str__` port.
                crate::strnum::python_decimal_str(&value.abs())
            } else if scale == 0 {
                // Integer-valued Decimal: `str` has no dot ("5").
                mag
            } else {
                let scale = scale as usize;
                if mag.len() <= scale {
                    // 0.00…digits — leading "0" before the point.
                    format!("0.{}{}", "0".repeat(scale - mag.len()), mag)
                } else {
                    let point = mag.len() - scale;
                    format!("{}.{}", &mag[..point], &mag[point..])
                }
            };
            if neg {
                format!("-{}", body)
            } else {
                body
            }
        }
    }
}

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "миллиард"), (12, "триллион")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

pub struct LangTt {
    /// `Num2Word_TT.CURRENCY_FORMS`, built once in [`LangTt::new`].
    ///
    /// The binding holds each converter in a `OnceLock` and hands out
    /// `&'static` references, so this is constructed one time per process.
    /// Rebuilding it per `to_currency` call is what made an earlier revision
    /// of this port an order of magnitude slower than the Python it replaces.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl LangTt {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "RUB": (("сум", "сум"), ("тиен", "тиен")),
        //     "USD": (("dollar", "dollars"), ("cent", "cents")),
        //     "EUR": (("euro", "euros"), ("cent", "cents")),
        // }
        // Two forms per side, exactly as Python: `to_currency` indexes [0] and
        // [1] directly, so the arity is load-bearing. RUB's two are identical
        // — сум is invariant here — but they stay two entries because the
        // index, not the value, is what the code selects on.
        let mut forms = HashMap::with_capacity(3);
        forms.insert(
            "RUB",
            CurrencyForms::new(&["сум", "сум"], &["тиен", "тиен"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["доллар", "доллар"], &["цент", "цент"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["евро", "евро"], &["цент", "цент"]),
        );
        LangTt { forms }
    }

    /// `list(self.CURRENCY_FORMS.values())[0]` — the fallback `to_currency`
    /// uses for an unrecognised code (bug 6).
    ///
    /// Python dicts iterate in insertion order and `CURRENCY_FORMS` is a class
    /// body literal, so "the first value" is permanently RUB's. That ordering
    /// is the *only* thing pinning this entry, which is why it is spelled out
    /// as a constant here rather than derived from the `HashMap` — which has
    /// no order to derive it from.
    fn fallback_forms(&self) -> &CurrencyForms {
        self.forms
            .get("RUB")
            .expect("RUB is the first CURRENCY_FORMS entry and is inserted in new()")
    }

    /// Python's `_int_to_word`.
    ///
    /// The ladder is reproduced arm for arm, in order. Every division here
    /// runs on a strictly positive value (the `number < 0` arm above them
    /// takes the sign away first), so floor and truncating division agree;
    /// `div_mod_floor` is used anyway to keep the Python `//` / `%` semantics
    /// literal rather than incidental.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            // Bug 1: ONES[0] is "" (falsy), so Python always lands on its
            // zero fallback (English there, ZERO_WORD here).
            return if ONES[0].is_empty() {
                ZERO_WORD.to_string()
            } else {
                ONES[0].to_string()
            };
        }

        if number.is_negative() {
            // Unreachable from to_cardinal (which strips the sign before
            // calling in), but present in Python and reachable via
            // to_currency, so it is ported rather than dropped.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        if number < &ten {
            // 1..=9; index 0 already handled above.
            return ONES[number.to_usize().unwrap()].to_string();
        }

        if number < &hundred {
            let (tens_val, ones_val) = number.div_mod_floor(&ten);
            let t = TENS[tens_val.to_usize().unwrap()];
            if ones_val.is_zero() {
                return t.to_string();
            }
            return format!("{} {}", t, ONES[ones_val.to_usize().unwrap()]);
        }

        if number < &thousand {
            let (hundreds_val, remainder) = number.div_mod_floor(&hundred);
            // Bug 4: no `hundreds_val == 1` shortcut — 100 is "бер йөз".
            let mut result = format!("{} {}", ONES[hundreds_val.to_usize().unwrap()], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &million {
            let (thousands_val, remainder) = number.div_mod_floor(&thousand);
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        if number < &billion {
            let (millions_val, remainder) = number.div_mod_floor(&million);
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        // Scale words above a million (gladiaio/num2words2#147), composed like
        // the million arm. Every entry point rejects values at or above
        // `maxval_ceiling()` first, so the top quotient is always below 1000.
        for &(exp, word) in SCALES.iter().rev() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let mut result = format!("{} {}", self.int_to_word(&(number / &scale)), word);
                let rest = number % &scale;
                if !rest.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&rest));
                }
                return result;
            }
        }

        // Bug 2: "Fallback for very large numbers" — bare digits, no words.
        number.to_string()
    }
}

impl Default for LangTt {
    fn default() -> Self {
        Self::new()
    }
}

impl Lang for LangTt {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "RUB"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }


    // cards / maxval / merge: left at their trait defaults. Python never
    // populates self.cards for TT (no *_numwords tables) and never reaches
    // Num2Word_Base.merge, because to_cardinal is overridden below.

    /// Python's `to_cardinal`.
    ///
    /// Python works from `str(number).strip()` and branches on whether a "."
    /// is present. Integer input can never produce one (`str(int)` has no
    /// decimal point), so only the `else` branch is live here and the float
    /// path is correctly out of scope. The sign is peeled off the *string*,
    /// not the number, then the remainder is re-parsed with `int()` — for an
    /// integer that round-trips exactly, so this ports to an `abs()`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        // The trailing .strip() is a no-op for every integer input: NEGWORD's
        // trailing space is always followed by a non-empty word (bug 1
        // guarantees even zero yields "нуль"). Ported anyway for fidelity.
        Ok(format!("{}{}", ret, self.checked_int_to_word(&n)?).trim().to_string())
    }

    /// Python's `to_ordinal`: cardinal plus a hyphenated suffix, no stem
    /// change. See bug 5.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(join_ordinal(&self.to_cardinal(value)?))
    }

    /// Python's `to_ordinal_num`: `str(number) + "."`.
    ///
    /// Note this is *not* the `Num2Word_Base` default (which returns the value
    /// unchanged), so the trait default would be wrong — TT appends a period.
    /// Negatives keep their sign: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python's `to_year(val, longval=True)`: delegates straight to
    /// `to_cardinal` and ignores `longval` entirely — no century splitting,
    /// so 1900 is "бер мең тугыз йөз", not "nineteen hundred"-style.
    /// Negatives get no BC/AD marker, just the "минус " of bug 3.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- float / Decimal entry routing --------------------------------

    /// `to_cardinal(float/Decimal)` FULL routing.
    ///
    /// Python's `to_cardinal` is string-driven: `"." in str(number)` picks the
    /// decimal grammar, and `str(5.0)` is `"5.0"`, so **whole floats keep
    /// their ".0" tail** ("биш өтер нуль") — never Base's whole-value integer
    /// route. [`LangTt::to_cardinal_float`] already reconstructs `str(number)`
    /// for both arms (dotless forms included: `Decimal("5")` -> "5" -> integer
    /// path, `str(1e16)` == "1e+16" / `str(Decimal("1E+2"))` == "1E+2" ->
    /// `int()` ValueError), so the entry routes everything through it.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number)` + the ordinal
    /// ending, no type guard — floats get the full decimal phrase plus the
    /// ending ("биш өтер нуленче"); the exponential-form ValueError propagates.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(join_ordinal(&self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — no `int()`, so
    /// it succeeds where the other modes raise ("1e+16.") and "-0.0" keeps
    /// its textual minus ("-0.0."). `repr_str` is Python's `str(number)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `return self.to_cardinal(val)` — the full
    /// float routing above, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which TT does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError; the
    /// binding's shared Inf sentinel would otherwise raise OverflowError.
    /// (NaN needs no interception: the binding's ValueError already matches
    /// `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Python's `to_cardinal` on the **float/Decimal** path.
    ///
    /// TT never reaches `Num2Word_Base.to_cardinal_float`: it overrides
    /// `to_cardinal` and, when the number is non-integral, splits `str(number)`
    /// on the first "." and speaks the fractional part **digit by literal
    /// digit**. The trait default (`floatpath::default_to_cardinal_float`) would
    /// instead reconstruct the fraction through `float2tuple` — a different
    /// algorithm that happens to agree on the corpus but is not what TT does —
    /// so it is overridden here.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword          # "minus "
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**, exactly as
    /// Python ignores it here: TT's `to_cardinal` reads `str(number)`, never
    /// `self.precision`, so `precision=` cannot change its output.
    ///
    /// # Faithfully reproduced bugs / quirks
    ///
    /// * The sign is decided by the *string* (`n.startswith("-")`), so it is
    ///   prepended even when the integer part is a huge digit-fallback run —
    ///   and `int(left)` re-strips it, so there is never a doubled "minus".
    /// * `int(left)` and each `int(digit)` are faithful `int()` calls: an
    ///   exponential `repr` (e.g. `str(1e-05) == "1e-05"`, or a mantissa dot
    ///   leaving `right == "5e-05"`) reaches `int("1e-05")` / `int("e")` and
    ///   raises `ValueError`, which surfaces as [`N2WError::Value`] with
    ///   Python's exact message — after any leading fractional digits have
    ///   already been emitted, just as Python builds the string left to right.
    /// * The huge-integer-part digit fallback of `_int_to_word` (bug 2) flows
    ///   straight through the integer part, so `Decimal("98746251323029.99")`
    ///   is "98746251323029 өтер тугыз тугыз".
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>, // TT ignores precision=; see above
    ) -> Result<String> {
        // `n = str(number).strip()`
        let s = python_str_number(value);
        let n = s.trim();

        // `if n.startswith("-"): n = n[1:]; ret = self.negword` (else ret = "")
        let (mut ret, n) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD.to_string(), rest),
            None => (String::new(), n),
        };

        // Python's `n.split(".", 1)`: split on the *first* dot only.
        match n.split_once('.') {
            Some((left, right)) => {
                // `ret += self._int_to_word(int(left)) + " " + self.pointword + " "`
                ret.push_str(&self.checked_int_to_word(&python_int(left)?)?);
                ret.push(' ');
                ret.push_str(POINTWORD);
                ret.push(' ');
                // `for digit in right: ret += self._int_to_word(int(digit)) + " "`
                // — one character at a time; `int(digit)` on a non-digit raises.
                for ch in right.chars() {
                    ret.push_str(&self.checked_int_to_word(&python_int(&ch.to_string())?)?);
                    ret.push(' ');
                }
                // `return ret.strip()`
                Ok(ret.trim().to_string())
            }
            None => {
                // `return (ret + self._int_to_word(int(n))).strip()`
                ret.push_str(&self.checked_int_to_word(&python_int(n)?)?);
                Ok(ret.trim().to_string())
            }
        }
    }

    // ---- currency ------------------------------------------------------

    /// Used only for `to_cheque`'s NotImplementedError message — TT's
    /// `to_currency` has no error path that mentions the class name (bug 6).
    fn lang_name(&self) -> &str {
        "Num2Word_TT"
    }

    /// `self.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately a plain lookup with **no** RUB fallback: this hook exists
    /// for `Num2Word_Base.to_cheque`, which does `self.CURRENCY_FORMS[currency]`
    /// inside a `try` and converts the `KeyError` into `NotImplementedError`.
    /// `None` here is what produces that, and the corpus depends on it —
    /// `cheque:GBP` is an `err: "NotImplementedError"` row while
    /// `currency:GBP` is a happy `"бер сум"`. TT's `to_currency` does *not* go
    /// through this hook: it calls `.get(currency, <first value>)` and can
    /// never miss, so it consults [`LangTt::fallback_forms`] itself.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // Not overridden, and each for a reason worth stating:
    //
    // * `currency_precision` — TT's CURRENCY_PRECISION is the empty dict
    //   inherited from Num2Word_Base and never mutated, so `.get(code, 100)`
    //   is 100 for every code: the trait default. `to_cheque` reads it (giving
    //   the "56/100" in "... AND 56/100 EUROS"); TT's `to_currency` never
    //   looks at it, which is why KWD/BHD get 2-decimal cents and JPY is not
    //   rounded to a whole unit (bug 7).
    // * `currency_adjective` — CURRENCY_ADJECTIVES is `{}`, and `to_currency`
    //   ignores the flag outright anyway (bug 10).
    // * `pluralize` — abstract in Python. TT never calls it (`to_currency` is
    //   its own, and base's `to_cheque` does not pluralize), so the default's
    //   NotImplemented is correct and unreachable rather than merely correct.
    // * `money_verbose` / `cents_verbose` / `cents_terse` — base's defaults.
    //   Only `money_verbose` is reachable, via `to_cheque`, and it routes back
    //   through TT's own `to_cardinal`, which the trait default already does.
    // * `cardinal_from_decimal` — the fractional-cents branch lives in
    //   `default_to_currency`, which TT bypasses entirely. Unreachable.
    // * `to_cheque` — not overridden in Python either; `default_to_cheque` is
    //   a faithful `Num2Word_Base.to_cheque` and the two hooks above are all
    //   it needs from TT.

    /// Python's `to_currency` — overridden wholesale, sharing nothing with
    /// `Num2Word_Base.to_currency`. See bugs 6-10 in the module docs.
    ///
    /// # `str(val)` on the Rust side
    ///
    /// `parts = str(val).split(".")` runs on the value *after* `abs()`, so the
    /// string never carries a sign. The int arm stringifies exactly as
    /// `str(int)` does; the float arm goes through [`python_str_float`] rather
    /// than `BigDecimal::Display`, which is a different function — see there.
    ///
    /// The `CurrencyValue::Int` / `Decimal` split stays load-bearing here even
    /// though TT reaches the same *output* for `1` and `1.0` (bug 8): the two
    /// arms produce different strings ("1" vs "1.0"), and the string is what
    /// the code splits.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // bug 10: accepted and ignored
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore TT's own `separator=" "` default; see BASE_DEFAULT_SEPARATOR.
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`. The `abs` is folded
        // into the stringification below, the only place `val` is read after.
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")`
        let s = match val {
            CurrencyValue::Int(i) => i.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => python_str_float(&d.abs()),
        };
        // Python's `split(".")` splits on *every* dot; `parts[0]` and
        // `parts[1]` are the first two segments. `split` + two `next()`s
        // reproduces that, where `splitn(2, ..)` would fold a hypothetical
        // third segment into `parts[1]`. `str(val)` never has two dots, but
        // the distinction costs nothing.
        let mut segments = s.split('.');
        let p0 = segments.next().unwrap_or("");
        let p1 = segments.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            python_int(p0)?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // — `len(parts) > 1` is `p1.is_some()`, `parts[1]`'s truthiness is the
        // non-empty check. Sliced and padded by `chars`, never by bytes.
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut digits: String = frac.chars().take(2).collect();
                while digits.chars().count() < 2 {
                    digits.push('0'); // `.ljust(2, "0")`
                }
                python_int(&digits)?
            }
            _ => BigInt::zero(),
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(...values())[0])`
        // — bug 6: the fallback is RUB, so this never fails.
        let forms = match self.forms.get(currency) {
            Some(f) => f,
            None => self.fallback_forms(),
        };
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let left_str = self.checked_int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — bug 9: `cents == false` skips the segment
        // rather than abbreviating it, and bug 8: a zero-cent float skips it
        // too, because `right` is tested for *truthiness*, not for presence.
        // Python's `0` is falsy exactly where `is_zero` is true here.
        if cents && !right.is_zero() {
            // `result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])`
            // Note the separator stands alone — TT adds no space of its own
            // after it, which is why its `" "` default is load-bearing.
            result.push_str(separator);
            result.push_str(&self.checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // `if is_negative: result = self.negword + result`. NEGWORD's own
        // trailing space is what separates it from the number; there is no
        // `.strip()` on it here, unlike `to_cardinal`'s
        // `"%s " % negword.strip()` idiom elsewhere in the library. Same
        // result, different route.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `return result.strip()` — a no-op on every reachable input (nothing
        // pads the ends: `int_to_word` never returns "", bug 1 guaranteeing
        // even zero yields "нуль"), kept to match the source.
        Ok(result.trim().to_string())
    }
}
