//! Port of `lang_SR_LATN.py` (Serbian, Gaj's Latin alphabet).
//!
//! Registry check: `CONVERTER_CLASSES["sr_Latn"]` is
//! `lang_SR_LATN.Num2Word_SR_LATN()` (`__init__.py:316`), so the key really
//! does resolve to the class named in the brief.
//!
//! Shape: **self-contained**. `Num2Word_SR_LATN` subclasses `Num2Word_SR`,
//! which subclasses `Num2Word_Base` but defines no `high_numwords` /
//! `mid_numwords` / `low_numwords`. Python therefore never builds
//! `self.cards` and never sets `MAXVAL`, so `cards`/`merge` stay at their
//! trait defaults here. The `SCALE` table ends at 10**30; Python raised
//! `KeyError` from 10**33 up. Fixed (gladiaio/num2words2#159): `maxval()` is
//! 10**33 and larger values raise `OverflowError` up front (see [`scale`]).
//!
//! `Num2Word_SR_LATN` itself is a pure transliteration wrapper: every method
//! calls `super()` and pipes the Cyrillic result through `cyrl_to_latn`. All
//! the arithmetic lives in `lang_SR.py`. This port keeps that structure —
//! tables stay in Cyrillic and transliteration happens at the boundary —
//! because the layering is load-bearing for `to_ordinal` (see below).
//!
//! # Inheritance chain, resolved
//!
//! | method          | defined in       | behaviour                                    |
//! |-----------------|------------------|----------------------------------------------|
//! | `to_cardinal`   | SR_LATN → SR     | `cyrl_to_latn(_int2word(int(n), False))`     |
//! | `to_ordinal`    | SR_LATN → SR     | dict lookup, else cardinal + "и"             |
//! | `to_ordinal_num`| SR_LATN → *Base* | `cyrl_to_latn(value)` → **AttributeError** (fixed, #157) |
//! | `to_year`       | SR_LATN → *Base* | `cyrl_to_latn(self.to_cardinal(value))`      |
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the frozen corpus (`bench/corpus.jsonl`, `"lang": "sr_Latn"`):
//!
//! 1. ~~**`to_ordinal_num` raises `AttributeError` for every input.**~~
//!    Fixed (gladiaio/num2words2#157). `SR` and `SR_LATN` both fail to
//!    override it, so it lands on `Num2Word_Base.to_ordinal_num`, which
//!    returns `value` — an `int`, not a `str` — and `SR_LATN.to_ordinal_num`
//!    then hands it to `cyrl_to_latn`, which calls `s.replace(...)` on it:
//!    `AttributeError: 'int' object has no attribute 'replace'`, for every
//!    input. The transliteration of a numeral is the identity, so the port
//!    returns what `sr` returns: the number itself (`1` → "1", `-1` → "-1",
//!    `1.5` → "1.5").
//! 2. *(Fixed, #248.)* Python glued "и" onto the cardinal outside its
//!    small dict ("dvestai", "četrdeset dvai"). The ordinal is now built in
//!    Cyrillic by `lang_sr`'s `ordinal_cyrl` (last word inflected) and
//!    transliterated: 42 == "četrdeset drugi", 200 == "dvestoti", 0 ==
//!    "nulti", 10**6 == "milioniti". Round thousands and larger round values
//!    still take the "и" fallback ("dve hiljadei"), a known gap.
//! 3. *(Fixed, #248.)* A negative ordinal is "minus" + the ordinal
//!    (`to_ordinal(-1)` == "minus prvi"); `verify_ordinal` is still never
//!    called, so it does not raise.
//! 4. **`_int2word` drops `feminine` when recursing for negatives**
//!    (`self._int2word(abs(number))` omits the argument, so it silently
//!    resets to `False`). Out of scope for the four modes here — `to_cardinal`
//!    only ever passes `feminine=False` — but reproduced anyway in
//!    [`int2word`] so the bug is preserved if the parameter is ever threaded
//!    through.
//! 5. *(Superseded by #248.)* Python's `to_ordinal` transliterated twice
//!    (a Latin cardinal plus a Cyrillic "и", re-run through `cyrl_to_latn`).
//!    The port now builds the whole ordinal in Cyrillic and transliterates
//!    it once — see bug 2.
//! 6. **`to_currency` ignored the currency code for `int` input (fixed,
//!    #176).** `Num2Word_SR.to_currency` intercepts `isinstance(val, int)`
//!    before delegating to Base and hardcodes "динар"/"динара", so in Python
//!    *every* code — including ones with no `CURRENCY_FORMS` entry — rendered
//!    as dinars (`currency:JPY 100` → "sto dinara"). The port looks the code
//!    up: EUR says "jedan evro", an unknown code raises `NotImplementedError`
//!    like the float path. `cents`, `separator` and `adjective` are still
//!    dropped on the int path.
//! 7. **`to_cheque` printed a stringified `bool` where the currency name
//!    belongs (fixed, #176).** See [`build_currency_forms`]; the port now
//!    takes the "many" form: "DVANAEST AND 50/100 EVRA".
//! 8. **Feminine units took a masculine numeral (fixed, #188).** Base's
//!    `_money_verbose` and SR's int shortcut both call `to_cardinal` with
//!    `feminine=False`, so 21 RUB was "dvadeset jedan rublja". The port reads
//!    the unit's gender flag like `_cents_verbose` reads the subunit's:
//!    "dvadeset jedna rublja", "dve rublje" (see `lang_sr`'s quirk 10).
//!
//! # Not a bug, but load-bearing: the four-element currency tuples
//!
//! `Num2Word_SR.CURRENCY_FORMS` is the only table in the library whose form
//! tuples end in a `bool` rather than a string. It changes the behaviour of two
//! *inherited* methods it was never designed for; [`build_currency_forms`]
//! documents how the arity is preserved here.
//!
//! # A note on `cyrl_to_latn`
//!
//! The Python comment claims two-character mappings "must come BEFORE
//! single-character entries so they win during the longest-match scan". This
//! is misleading: every *key* is a single codepoint (Љ is U+0409, not Л+Ј),
//! so there is no longest-match scan and the order is irrelevant. The
//! "digraphs" are digraphs on the *output* side ("Lj"). A plain per-char map
//! is therefore exactly equivalent — see [`cyrl_to_latn`].

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

const ZERO: &str = "нула";
/// `setup()` sets `self.negword = "минус"` and `self.pointword = "запета"`
/// (the latter is transliterated to "zapeta" and used by the float path —
/// see [`LangSrLatn::to_cardinal_float`]).
const NEGWORD: &str = "минус";

/// `ONES`, keys 1..=9, `(masculine, feminine)`. Index 0 is absent in Python
/// (guarded by `digit_right > 0`).
const ONES: [(&str, &str); 10] = [
    ("", ""), // absent in Python
    ("један", "једна"),
    ("два", "две"),
    ("три", "три"),
    ("четири", "четири"),
    ("пет", "пет"),
    ("шест", "шест"),
    ("седам", "седам"),
    ("осам", "осам"),
    ("девет", "девет"),
];

/// `TENS`, keys 0..=9 — the teens, reached when `digit_mid == 1`.
const TENS: [&str; 10] = [
    "десет",
    "једанаест",
    "дванаест",
    "тринаест",
    "четрнаест",
    "петнаест",
    "шеснаест",
    "седамнаест",
    "осамнаест",
    "деветнаест",
];

/// `TWENTIES`, keys 2..=9. Indices 0 and 1 are absent in Python (guarded by
/// `digit_mid > 1`).
const TWENTIES: [&str; 10] = [
    "", // absent in Python
    "", // absent in Python
    "двадесет",
    "тридесет",
    "четрдесет",
    "педесет",
    "шездесет",
    "седамдесет",
    "осамдесет",
    "деведесет",
];

/// `HUNDREDS`, keys 1..=9. Index 0 is absent in Python (guarded by
/// `digit_left > 0`).
const HUNDREDS: [&str; 10] = [
    "", // absent in Python
    "сто",
    "двеста",
    "триста",
    "четиристо",
    "петсто",
    "шесто",
    "седамсто",
    "осамсто",
    "деветсто",
];

/// `SCALE[chunk_len]` → `(one, few, many, is_feminine)`.
///
/// Python's `SCALE` is a dict with keys 0..=10, i.e. it tops out at 10**30
/// ("квинтилион"). A chunk index of 11 or more — any value >= 10**33 —
/// raised `KeyError` in Python; `int2word` now rejects those values with
/// `OverflowError` first (#159), so that arm is unreachable. Serbian uses the long scale, hence
/// милијарда at 10**9 rather than a short-scale "billion".
fn scale(chunk_len: usize) -> Result<(&'static str, &'static str, &'static str, bool)> {
    Ok(match chunk_len {
        0 => ("", "", "", false),
        1 => ("хиљада", "хиљаде", "хиљада", true), // 10^3
        2 => ("милион", "милиона", "милиона", false), // 10^6
        3 => ("милијарда", "милијарде", "милијарди", true), // 10^9 - long scale
        4 => ("билион", "билиона", "билиона", false), // 10^12
        5 => ("билијарда", "билијарде", "билијарди", true), // 10^15
        6 => ("трилион", "трилиона", "трилиона", false), // 10^18
        7 => ("трилијарда", "трилијарде", "трилијарди", true), // 10^21
        8 => ("квадрилион", "квадрилиона", "квадрилиона", false), // 10^24
        9 => ("квадрилијарда", "квадрилијарде", "квадрилијарди", true), // 10^27
        10 => ("квинтилион", "квинтилиона", "квинтилиона", false), // 10^30
        _ => return Err(key_error(&chunk_len.to_string())),
    })
}

// --- Python exception encoding -------------------------------------------

/// Python raised `KeyError` — a missing `SCALE` entry for values >= 10**33
/// (unreachable since the MAXVAL check, #159).
fn key_error(msg: &str) -> N2WError {
    N2WError::Key(msg.to_string())
}

// --- utils.py ------------------------------------------------------------

/// `utils.splitbyx(n, 3)` over a decimal string, with `format_int=True`.
///
/// `start = length % x` guarantees `length - start` is a multiple of `x`, so
/// every subsequent slice is exactly `x` long and `i + x <= length` always
/// holds; the `min` is belt-and-braces against a panic where Python would
/// simply yield a short slice.
///
/// The input is `str(number)` for a **non-negative** number here (`int2word`
/// strips the sign before calling), so it is pure ASCII digits and byte
/// slicing is safe.
fn splitbyx(n: &str, x: usize) -> Vec<u32> {
    let length = n.len();
    let mut out = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(n[..start].parse::<u32>().unwrap_or(0));
        }
        let mut i = start;
        while i < length {
            out.push(n[i..(i + x).min(length)].parse::<u32>().unwrap_or(0));
            i += x;
        }
    } else {
        out.push(n.parse::<u32>().unwrap_or(0));
    }
    out
}

/// `utils.get_digits(n)` → `(digit_right, digit_mid, digit_left)`.
///
/// Python: `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` — zero-pad to
/// three, keep the last three, reverse. Every chunk from `splitbyx(_, 3)` is
/// in 0..=999, so the `[-3:]` truncation never actually bites.
fn get_digits(n: u32) -> (usize, usize, usize) {
    let s = format!("{:03}", n);
    let chars: Vec<char> = s.chars().collect();
    let last3 = &chars[chars.len().saturating_sub(3)..];
    let d: Vec<usize> = last3
        .iter()
        .rev()
        .map(|c| c.to_digit(10).unwrap_or(0) as usize)
        .collect();
    (d[0], d[1], d[2])
}

/// Parse an all-ASCII-digit slice of a reconstructed numeric string (a
/// `left`/`right` fragment from the float path) into a `BigInt`.
///
/// The inputs are internally generated by `format!`/`BigDecimal::with_scale`,
/// so they are always non-empty ASCII digits and the parse cannot fail; the
/// error arm mirrors the `ValueError` Python's `int()` would raise on a bad
/// token rather than panicking. Leading zeros are accepted, matching `int`.
fn parse_bigint(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>()
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `str(number)` for a float/Decimal input — the string
/// `Num2Word_SR.to_cardinal` splits (`n = str(number).replace(",", ".")`).
///
/// * **float** — `repr(float)`. Fixed notation (`{:.*}` at the shim's
///   repr-derived precision) whenever the repr shows a decimal point;
///   otherwise repr picked exponent form (`abs(v) >= 1e16`: "1e+16"),
///   reconstructed by [`python_float_exp_repr`]. The exponent form has no
///   ".", so `to_cardinal` falls to `int("1e+16")` and raises ValueError —
///   corpus-pinned (`cardinal(1e+16)` == ValueError), while
///   `ordinal(1e+16)` (which `int()`s the *value*) succeeds.
/// * **Decimal** — [`python_decimal_str`], the spec `__str__`: trailing
///   zeros kept ("5.00"), scientific form for positive exponents ("1E+2",
///   "1E+20" — again ValueError from `int()`).
///
/// Known gap (unpinned): a float in `(0, 1e-4)` also reprs in exponent form
/// ("1e-05"), but `FloatValue::has_visible_point` reports `true` for every
/// finite fractional float, so such values take the fixed-notation arm here
/// and render instead of raising. No `sr_Latn` corpus row reaches that range.
fn python_number_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, precision } => {
            if v.has_visible_point() {
                format!("{:.*}", *precision as usize, value)
            } else {
                python_float_exp_repr(*value)
            }
        }
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Python's `repr(float)` for the exponent-form cases: shortest mantissa
/// (Rust's `{:e}` is shortest-round-trip, the same contract), "e", an
/// explicit sign and a >= 2-digit exponent — `1e16` -> "1e+16", `1.5e20` ->
/// "1.5e+20". Non-finite values print as Python does ("inf"/"-inf"/"nan");
/// `int()` on those strings raises the same ValueError as on "1e+16".
fn python_float_exp_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }
    let s = format!("{:e}", v);
    match s.split_once('e') {
        Some((mant, exp)) => {
            let e: i64 = exp.parse().unwrap_or(0);
            let sign = if e < 0 { "-" } else { "+" };
            format!("{}e{}{:02}", mant, sign, e.abs())
        }
        None => s,
    }
}

/// Extract SR's `feminine=` kwarg (`to_cardinal(self, number,
/// feminine=False)`, which SR_LATN forwards via `*args, **kwargs`). The flag
/// only ever feeds `feminine or SCALE[chunk_len][-1]` then
/// `int(is_feminine)`, so absent / `False` / explicit `None` are all the
/// masculine column and `True` is the feminine one. Any other key, or a
/// non-bool value (`feminine=2` would `int()` to a tuple index Python
/// crashes on), is `NotImplemented` so the dispatcher falls back to the
/// original Python and its genuine semantics.
fn feminine_kwarg(kw: &Kwargs) -> Result<bool> {
    if !kw.only(&["feminine"]) {
        return Err(N2WError::Fallback("kwargs".into()));
    }
    match kw.get("feminine") {
        Option::None | Some(KwVal::None) => Ok(false),
        Some(KwVal::Bool(b)) => Ok(*b),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

// --- lang_SR.py ----------------------------------------------------------

/// `Num2Word_SR.pluralize(number, forms)` — the Slavic 1 / 2-4 / other rule.
fn pluralize(number: u32, forms: (&'static str, &'static str, &'static str, bool)) -> &'static str {
    let form = if number % 100 < 10 || number % 100 > 20 {
        if number % 10 == 1 {
            0
        } else if number % 10 > 1 && number % 10 < 5 {
            1
        } else {
            2
        }
    } else {
        2
    };
    match form {
        0 => forms.0,
        1 => forms.1,
        _ => forms.2,
    }
}

/// `Num2Word_SR._int2word`. Returns **Cyrillic**; the caller transliterates.
fn int2word(number: &BigInt, feminine: bool) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    if number.is_negative() {
        // Python: " ".join([self.negword, self._int2word(abs(number))]).
        // Note the dropped `feminine` argument — reproduced verbatim (bug 4).
        return Ok(format!("{} {}", NEGWORD, int2word(&number.abs(), false)?));
    }
    if number.is_zero() {
        return Ok(ZERO.to_string());
    }

    let mut words: Vec<&'static str> = Vec::new();
    let digits = number.to_string();
    let chunks = splitbyx(&digits, 3);
    let mut chunk_len = chunks.len();

    for chunk in &chunks {
        chunk_len -= 1;
        let (digit_right, digit_mid, digit_left) = get_digits(*chunk);

        if digit_left > 0 {
            words.push(HUNDREDS[digit_left]);
        }

        if digit_mid > 1 {
            words.push(TWENTIES[digit_mid]);
        }

        if digit_mid == 1 {
            words.push(TENS[digit_right]);
        } else if digit_right > 0 {
            // Skip 'један' for thousands (1000, 1001, etc.)
            if !(chunk_len > 0 && *chunk == 1) {
                // SCALE is indexed here *before* the `chunk != 0` guard below,
                // so a >= 10**33 chunk whose skip-condition misses raised
                // KeyError at this line in Python. Order preserved; the MAXVAL
                // check (#159) keeps such chunks from reaching it.
                let is_feminine = feminine || scale(chunk_len)?.3;
                words.push(if is_feminine {
                    ONES[digit_right].1
                } else {
                    ONES[digit_right].0
                });
            }
        }

        if chunk_len > 0 && *chunk != 0 {
            words.push(pluralize(*chunk, scale(chunk_len)?));
        }
    }

    Ok(words.join(" "))
}

/// `Num2Word_SR.to_cardinal(number, feminine)` for a float/Decimal `number`
/// — the whole method, both branches, returning **Cyrillic** (the caller
/// transliterates, exactly like the `cyrl_to_latn(super().to_cardinal(...))`
/// wrapper):
///
/// ```python
/// n = str(number).replace(",", ".")
/// if "." in n:
///     is_negative = n.startswith("-")
///     left, right = (n[1:] if is_negative else n).split(".")
///     leading = len(right) - len(right.lstrip("0"))
///     decimal_part = ("нула " * leading) + self._int2word(int(right), feminine)
///     result = "%s %s %s" % (self._int2word(int(left), feminine),
///                            self.pointword, decimal_part)
///     if is_negative: result = self.negword + " " + result
/// else:
///     result = self._int2word(int(n), feminine)
/// return result
/// ```
///
/// The fraction is one whole number, not a digit sequence (`2.675` reads
/// "675"), and every leading fraction zero becomes a "нула" word. An
/// all-zero `right` reads one "нула" per digit written (Python counted
/// `"0"` from `1.0` as a leading zero *and* `int("0")`, doubling it; fixed,
/// gladiaio/num2words2#237).
/// The `else` arm is where exponent-form strings die: `int("1e+16")` /
/// `int("1E+2")` raise ValueError with Python's exact message (see
/// [`parse_bigint`]).
fn cardinal_float_str(value: &FloatValue, feminine: bool) -> Result<String> {
    // n = str(number).replace(",", ".") — the reconstruction never emits a
    // comma; the sign stays on the string so `startswith("-")` matches.
    let n = python_number_str(value);

    if let Some((left, right)) = {
        let is_negative = n.starts_with('-');
        let abs_n = if is_negative { &n[1..] } else { &n[..] };
        abs_n.split_once('.').map(|(l, r)| (l.to_string(), r.to_string()))
    } {
        let is_negative = n.starts_with('-');
        // leading_zero_count = len(right) - len(right.lstrip("0"))
        // The final int(right) word already says one zero, so an all-zero
        // fraction gets len - 1 leading zeros: exactly the digits written
        // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
        let leading = right.chars().take_while(|&c| c == '0').count()
            .min(right.len().saturating_sub(1));
        let left_int = parse_bigint(&left)?;
        let right_int = parse_bigint(&right)?;

        // decimal_part = ("нула " * leading) + _int2word(int(right), feminine)
        let mut decimal_part = String::new();
        for _ in 0..leading {
            decimal_part.push_str(ZERO);
            decimal_part.push(' ');
        }
        decimal_part.push_str(&int2word(&right_int, feminine)?);

        // result = "<int(left)> запета <decimal_part>"
        let body = format!(
            "{} {} {}",
            int2word(&left_int, feminine)?,
            "запета",
            decimal_part
        );
        Ok(if is_negative {
            format!("{} {}", NEGWORD, body)
        } else {
            body
        })
    } else {
        // else: _int2word(int(n), feminine). int(n) keeps the sign, and
        // int2word renders "минус ..." for a negative; exponent-form and
        // non-finite strings ("1e+16", "1E+2", "inf") raise ValueError here.
        int2word(&parse_bigint(&n)?, feminine)
    }
}

// --- lang_SR_LATN.py -----------------------------------------------------

/// `_CYRL_TO_LATN` / `cyrl_to_latn` — Serbian Cyrillic → Gaj's Latin.
///
/// Python chains 36 `str.replace` calls. Each key is a single codepoint and no
/// replacement *value* contains a Cyrillic codepoint, so no replacement can be
/// re-matched by a later pass — the chain is exactly a per-character map, and
/// running it twice is idempotent (which `to_ordinal` and `to_year` both rely
/// on; see bug 5).
///
/// All 30 letters of the Serbian Cyrillic alphabet are covered. Anything else
/// (spaces, Latin text, digits) passes through untouched, matching `replace`.
fn cyrl_to_latn(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            // Digraphs are case-sensitive: one Cyrillic char → two Latin.
            'Љ' => out.push_str("Lj"),
            'Њ' => out.push_str("Nj"),
            'Џ' => out.push_str("Dž"),
            'љ' => out.push_str("lj"),
            'њ' => out.push_str("nj"),
            'џ' => out.push_str("dž"),
            // Single characters.
            'А' => out.push('A'),
            'а' => out.push('a'),
            'Б' => out.push('B'),
            'б' => out.push('b'),
            'В' => out.push('V'),
            'в' => out.push('v'),
            'Г' => out.push('G'),
            'г' => out.push('g'),
            'Д' => out.push('D'),
            'д' => out.push('d'),
            'Ђ' => out.push('Đ'),
            'ђ' => out.push('đ'),
            'Е' => out.push('E'),
            'е' => out.push('e'),
            'Ж' => out.push('Ž'),
            'ж' => out.push('ž'),
            'З' => out.push('Z'),
            'з' => out.push('z'),
            'И' => out.push('I'),
            'и' => out.push('i'),
            'Ј' => out.push('J'),
            'ј' => out.push('j'),
            'К' => out.push('K'),
            'к' => out.push('k'),
            'Л' => out.push('L'),
            'л' => out.push('l'),
            'М' => out.push('M'),
            'м' => out.push('m'),
            'Н' => out.push('N'),
            'н' => out.push('n'),
            'О' => out.push('O'),
            'о' => out.push('o'),
            'П' => out.push('P'),
            'п' => out.push('p'),
            'Р' => out.push('R'),
            'р' => out.push('r'),
            'С' => out.push('S'),
            'с' => out.push('s'),
            'Т' => out.push('T'),
            'т' => out.push('t'),
            'Ћ' => out.push('Ć'),
            'ћ' => out.push('ć'),
            'У' => out.push('U'),
            'у' => out.push('u'),
            'Ф' => out.push('F'),
            'ф' => out.push('f'),
            'Х' => out.push('H'),
            'х' => out.push('h'),
            'Ц' => out.push('C'),
            'ц' => out.push('c'),
            'Ч' => out.push('Č'),
            'ч' => out.push('č'),
            'Ш' => out.push('Š'),
            'ш' => out.push('š'),
            other => out.push(other),
        }
    }
    out
}

// --- currency ------------------------------------------------------------

/// `Num2Word_SR.CURRENCY_FORMS`, transcribed with its **four**-element tuples
/// intact.
///
/// ```python
/// CURRENCY_FORMS = {
///     "RUB": (("rublja", "rublje", "rublji", True),
///             ("kopejka", "kopejke", "kopejki", True)),
///     "EUR": (("evro", "evra", "evra", False),
///             ("cent", "centa", "centi", False)),
///     "RSD": (("dinar", "dinara", "dinara", False),
///             ("para", "pare", "para", True)),
/// }
/// ```
///
/// Two things about this table are unusual and both are load-bearing.
///
/// **The values are already Latin.** `lang_SR.py` keeps its *number* words in
/// Cyrillic but wrote the currency nouns in Latin, so `cyrl_to_latn` passes
/// over them untouched and Cyrillic `sr` inherits Latin currency names. Not our
/// bug to fix; do not "restore" them to Cyrillic.
///
/// **The fourth element is a `bool`, not a string.** It is the grammatical
/// gender flag `_cents_verbose` hands to `_int2word` as its `feminine`
/// argument, via `CURRENCY_FORMS[currency][1][-1]` — para and kopejka are
/// feminine, cent is not. [`CurrencyForms`] can only hold strings, so the flag
/// is carried as the literal `"True"` / `"False"`. That is not a workaround: it
/// is *exactly* the text Python produces wherever that slot reaches a format
/// string. Two consumers depend on the fourth element, and each is satisfied by
/// this encoding:
///
/// 1. `cents_verbose` reads the **subunit's** last element back as a bool
///    (`== "True"`), reproducing `[1][-1]`. Hence
///    `to_currency(1.21, "RSD")` → "jedan dinar, dvadeset **jedna** para"
///    (feminine), while `to_currency(1.21, "EUR")` → "...dvadeset **jedan**
///    cent" (masculine).
/// 2. `Num2Word_Base.to_cheque` does
///    `unit = cr1[-1] if isinstance(cr1, tuple) else cr1`, meaning that for a
///    4-tuple Python picks the **bool** instead of the plural noun:
///    "HILJADA DVESTA TRIDESET ČETIRI AND 56/100 FALSE". Fixed (#176): the
///    shared `lang_sr::sr_to_cheque` reads the "many" form at index 2, so the
///    cheque ends in "EVRA". The fourth element only feeds `cents_verbose`.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "RUB",
        CurrencyForms::new(
            &["rublja", "rublje", "rublji", "True"],
            &["kopejka", "kopejke", "kopejki", "True"],
        ),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(
            &["evro", "evra", "evra", "False"],
            &["cent", "centa", "centi", "False"],
        ),
    );
    m.insert(
        "RSD",
        CurrencyForms::new(
            &["dinar", "dinara", "dinara", "False"],
            &["para", "pare", "para", "True"],
        ),
    );
    m
}

pub struct LangSrLatn {
    /// Built once in [`LangSrLatn::new`]. `lib.rs` holds the language in a
    /// `OnceLock`, so this table is constructed a single time per process and
    /// `to_currency` only ever reads it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangSrLatn {
    pub fn new() -> Self {
        LangSrLatn {
            currency_forms: build_currency_forms(),
        }
    }
}

impl Default for LangSrLatn {
    fn default() -> Self {
        LangSrLatn::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangSrLatn {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "запета"
    }

    /// `SR_LATN.to_cardinal` → `cyrl_to_latn(SR.to_cardinal(...))`.
    ///
    /// `SR.to_cardinal` does `str(number).replace(",", ".")` and branches on a
    /// "." — integer input never contains one, so this reduces to
    /// `_int2word(int(n), feminine=False)`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(cyrl_to_latn(&int2word(value, false)?))
    }

    /// `SR_LATN.to_ordinal` → `cyrl_to_latn(SR.to_ordinal(...))`.
    ///
    /// The `try: int(number) except (ValueError, TypeError): return
    /// str(number)` guard in Python can never fire for the BigInt we are
    /// handed, so it is elided.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // Built in Cyrillic, then transliterated once (#248).
        let cyrl = crate::lang_sr::ordinal_cyrl(value, &|v| int2word(v, false))?;
        Ok(cyrl_to_latn(&cyrl))
    }

    // `to_ordinal_num` (int and float/Decimal): the trait defaults, i.e. what
    // `sr` returns. Python crashed in `cyrl_to_latn` on the non-str (bug 1).

    /// `SR_LATN.to_year` → `cyrl_to_latn(Num2Word_Base.to_year(...))`, and
    /// `Base.to_year` is just `self.to_cardinal(value)` — which dispatches back
    /// to the SR_LATN override, so the string is already Latin and the outer
    /// `cyrl_to_latn` is a no-op. No era handling, no two-digit splitting.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(cyrl_to_latn(&self.to_cardinal(value)?))
    }

    /// `SR_LATN.to_cardinal(number)` for **non-integer** input — the float and
    /// Decimal cardinal path.
    ///
    /// `Num2Word_SR` does **not** override `to_cardinal_float`. Instead its
    /// `to_cardinal` services floats/Decimals inline by string-splitting
    /// `str(number)` (`lang_SR.py`), and the `num2words` dispatcher routes float
    /// input through `converter.to_cardinal(number)` — never through
    /// `to_cardinal_float`. So the inherited `default_to_cardinal_float` is the
    /// *wrong* behaviour for this language (it renders the fractional part
    /// digit-by-digit and leaves the Cyrillic pointword untransliterated, e.g.
    /// `2.675` → "dva запета šest sedam pet"); [`cardinal_float_str`]
    /// reproduces the SR string algorithm instead, and this wrapper is
    /// `cyrl_to_latn(Num2Word_SR.to_cardinal(number))`.
    ///
    /// `precision_override` (issue #580) is a **documented no-op**:
    /// `SR.to_cardinal` never consults `self.precision`, so
    /// `num2words(2.675, lang="sr_Latn", precision=1)` is unchanged — verified
    /// against the live interpreter. This hook also serves the
    /// fractional-cents currency path (`cardinal_from_decimal` ->
    /// `cardinal_from_bigdecimal` dispatches through the trait), mirroring
    /// Python's virtual `self.to_cardinal(float(right))`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        Ok(cyrl_to_latn(&cardinal_float_str(value, false)?))
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// SR branches on `"." in str(number)`, so a whole value with a visible
    /// point still takes the float grammar: `5.0` -> "pet zapeta nula",
    /// `Decimal("5.00")` -> "pet zapeta nula nula", `-0.0` -> "minus
    /// nula zapeta nula". A pointless string falls to `int(n)`:
    /// `Decimal("5")` -> "pet", but `1e+16` / `Decimal("1E+2")` raise
    /// ValueError. The base default (whole -> int path) would get every one
    /// of those wrong, hence this override.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        Ok(cyrl_to_latn(&cardinal_float_str(value, false)?))
    }

    /// `to_ordinal(float/Decimal)`. `SR.to_ordinal` opens with
    /// `num = int(number)` — `int()` of the *value*, truncation toward zero —
    /// so `2.5` -> 2 -> "drugi", `-1.5` -> -1 -> "minus prvi", `0.5` ->
    /// 0 -> "nulti", and `1e+16` *succeeds* ("deset bilijardii") where the
    /// cardinal raises ValueError.
    ///
    /// The `except (ValueError, TypeError): return str(number)` guard can
    /// only fire here for `nan` (`int(nan)` is ValueError, so Python returns
    /// `str(nan)` == "nan", which `cyrl_to_latn` passes through); `int(inf)`
    /// raises OverflowError, which the guard does **not** catch. Neither has
    /// a corpus row; both are mirrored for completeness.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let num = match value {
            FloatValue::Float { value: f, .. } => {
                if f.is_nan() {
                    return Ok("nan".to_string());
                }
                BigInt::from_f64(f.trunc()).ok_or_else(|| {
                    N2WError::Overflow("cannot convert float infinity to integer".into())
                })?
            }
            // int(Decimal) truncates toward zero; with_scale(0) drops the
            // fractional digits the same way (see floatpath::float2tuple).
            FloatValue::Decimal { value: d, .. } => d.with_scale(0).as_bigint_and_exponent().0,
        };
        // Dispatches to the SR_LATN to_ordinal override above — table word or
        // Latin cardinal + "и", double-transliterated exactly as Python.
        self.to_ordinal(&num)
    }

    // `year_float_entry` is deliberately NOT overridden: `SR_LATN.to_year` is
    // `cyrl_to_latn(Base.to_year(...))` == `cyrl_to_latn(self.to_cardinal(...))`,
    // and the trait default routes through the overridden
    // `cardinal_float_entry` above — so `to_year(5.0)` == "pet zapeta nula
    // nula" and `to_year(1e+16)` raises ValueError, as pinned.

    /// `converter.str_to_number` — Base's `Decimal(value)`, which neither SR
    /// nor SR_LATN overrides. The `Inf` interception reproduces what happens
    /// *next* on the pinned path: `to_cardinal(Decimal("Infinity"))` reads
    /// `n = str(number)` == "Infinity", finds no ".", and dies in
    /// `int("Infinity")` with ValueError. The binding otherwise maps
    /// `ParsedNumber::Inf` to the base integer path's OverflowError before
    /// any SR code runs, so the ValueError must be raised here. (NaN needs no
    /// interception: the binding's ValueError already matches `int("NaN")`'s
    /// type.)
    ///
    /// Known gap (unpinned): Python's `to_ordinal(Decimal("Infinity"))`
    /// raises OverflowError (`int()` of the Decimal *object*), which this
    /// entry-level interception turns into the cardinal path's ValueError.
    /// The strings corpus pins Infinity under `to=cardinal` only.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}Infinity'",
                if negative { "-" } else { "" }
            ))),
            other => Ok(other),
        }
    }

    // ---- grammatical kwargs ----------------------------------------------

    /// `SR_LATN.to_cardinal(*args, **kwargs)` forwards SR's `feminine=`
    /// kwarg: `to_cardinal(2, feminine=True)` == "dve". The negative
    /// recursion inside `_int2word` still drops the flag (bug 4):
    /// `to_cardinal(-5, feminine=True)` == "minus pet".
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        let feminine = feminine_kwarg(kw)?;
        Ok(cyrl_to_latn(&int2word(value, feminine)?))
    }

    /// The float/Decimal side of the same kwarg: Python's `to_cardinal`
    /// threads `feminine` into *both* `_int2word` calls of the "." branch.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        let feminine = feminine_kwarg(kw)?;
        Ok(cyrl_to_latn(&cardinal_float_str(value, feminine)?))
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_SR` overrides `to_currency`, `pluralize` and `_cents_verbose`;
    // `Num2Word_SR_LATN` wraps `to_currency` in `cyrl_to_latn`. Everything else
    // on the currency surface is inherited from `Num2Word_Base` unchanged and
    // is already mirrored by the trait defaults, so it is deliberately absent
    // here:
    //
    // * `_money_verbose` is overridden below (bug 8, #188), not inherited.
    // * `_cents_terse`   → `CURRENCY_PRECISION.get(currency, 100)` is 100 for
    //   every code, so the default's `"%02d"` width is right.
    // * `CURRENCY_PRECISION` is `{}` (never rebound), so `currency_precision`
    //   stays at the trait's flat 100 — SR has no 3-decimal and no 0-decimal
    //   currency, and `default_to_currency`'s `divisor == 1` branch is
    //   unreachable. This is *why* `currency:JPY 12.34` raises
    //   NotImplementedError instead of rounding to a whole yen.
    // * `CURRENCY_ADJECTIVES` is `{}`, so `currency_adjective` stays `None` and
    //   `adjective=True` is a no-op — matching `if adjective and currency in
    //   self.CURRENCY_ADJECTIVES`.
    // * `to_cheque` is Base's except for the unit word (bug 7, #176): it goes
    //   through the shared `lang_sr::sr_to_cheque`.

    /// `self.__class__.__name__` for the NotImplementedError message. The
    /// raise sites live in `base.py` and `lang_SR.py`, but the *instance* is a
    /// `Num2Word_SR_LATN`, so that is the name Python interpolates.
    fn lang_name(&self) -> &str {
        "Num2Word_SR_LATN"
    }

    /// `Num2Word_SR.to_currency(val, currency="RSD", ...)`.
    ///
    /// This override is hand-added rather than generated. The generator reads
    /// `inspect.signature(converter.to_currency)`, and `Num2Word_SR_LATN`'s is
    /// `(*args, **kwargs)` — the real defaults sit one level up on
    /// `Num2Word_SR.to_currency`, invisible to `signature()`. sr_Latn is the
    /// only affected language whose default is not Base's "EUR", so it is the
    /// only one where the omission is observable:
    ///
    /// ```text
    /// num2words(1.5, lang="sr_Latn", to="currency")
    ///   == "jedan dinar, pedeset para"      # RSD, verified against Python
    ///   != "jedan evro, pedeset centi"      # what Base's "EUR" default gives
    /// ```
    ///
    /// (`separator` needs no such override: SR's default is `","`, identical to
    /// Base's, so the trait default is already correct.)
    fn default_currency(&self) -> &str {
        "RSD"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_Base.to_cheque` with the "many" form as the unit word instead
    /// of the gender flag (bug 7, #176). The cardinal already arrives Latin.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        crate::lang_sr::sr_to_cheque(self, val, currency)
    }

    /// `Num2Word_SR.pluralize(number, forms)` — the Slavic one / few / many
    /// rule:
    ///
    /// ```python
    /// if number % 100 < 10 or number % 100 > 20:
    ///     if number % 10 == 1:      form = 0
    ///     elif 1 < number % 10 < 5: form = 1
    ///     else:                     form = 2
    /// else:
    ///     form = 2
    /// return forms[form]
    /// ```
    ///
    /// Distinct from the module-level `pluralize` helper, which serves
    /// `SCALE`'s 4-tuples from inside [`int2word`]. This one is the trait hook
    /// `currency::default_to_currency` calls with `CURRENCY_FORMS` entries.
    ///
    /// `mod_floor` rather than `%` because Python's `%` floors on negatives.
    /// Every caller hands us a non-negative `left`/`right` straight out of
    /// `parse_currency_parts`, so the two agree today — the port simply should
    /// not depend on that.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let ten = BigInt::from(10);
        let m100 = n.mod_floor(&BigInt::from(100));
        let m10 = n.mod_floor(&ten);

        let form = if m100 < ten || m100 > BigInt::from(20) {
            if m10.is_one() {
                0
            } else if m10 > BigInt::one() && m10 < BigInt::from(5) {
                1
            } else {
                2
            }
        } else {
            2
        };

        // Python indexes the tuple directly. Every SR entry carries four forms
        // so this cannot miss, but a short tuple would raise IndexError rather
        // than panic, and the exception type is what a caller can observe.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Base's `_money_verbose` (`self.to_cardinal(number)`, Latin), with the
    /// units word agreeing with the unit's gender flag `[0][-1]` (bug 8,
    /// #188): "dvadeset jedna rublja". Cheques read it too.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let words = int2word(number, false)?;
        let feminine = self
            .currency_forms
            .get(currency)
            .and_then(|f| f.unit.last())
            .map_or(false, |flag| flag.as_str() == "True");
        let words = if feminine { crate::lang_sr::feminine_last(&words) } else { words };
        Ok(cyrl_to_latn(&words))
    }

    /// `Num2Word_SR._cents_verbose`:
    /// `self._int2word(number, self.CURRENCY_FORMS[currency][1][-1])`.
    ///
    /// Returns **Cyrillic**, exactly as Python does: `_cents_verbose` is only
    /// ever reached from inside `to_currency`, whose SR_LATN wrapper
    /// transliterates the finished string. Nothing else calls this hook, and
    /// the bindings do not expose it, so the Cyrillic cannot escape.
    ///
    /// `[1][-1]` is the *subunit* tuple's gender flag — see
    /// [`build_currency_forms`] for why it round-trips through `"True"`.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        // Python would raise KeyError here for an unknown code, but the lookup
        // is unreachable: `default_to_currency` resolves the same table first
        // and has already raised NotImplementedError by this point.
        let forms = self
            .currency_forms
            .get(currency)
            .ok_or_else(|| key_error(currency))?;
        let feminine = forms.subunit.last().map(|s| s == "True").unwrap_or(false);
        int2word(number, feminine)
    }

    /// `Num2Word_SR_LATN.to_currency` →
    /// `cyrl_to_latn(Num2Word_SR.to_currency(*args, **kwargs))`.
    ///
    /// `Num2Word_SR.to_currency` splits in two: it services `int` itself and
    /// hands everything else to `Num2Word_Base.to_currency` via `super()`. The
    /// int arm is bug 6 — it never looks at `currency`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it before the ported body. SR's default is Base's ",".
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(v) = val {
            // Python never looked the code up here (bug 6, #176); the port
            // does, so an unknown code raises like the float path.
            let forms = self.currency_forms.get(currency).ok_or_else(|| {
                N2WError::NotImplemented(format!(
                    "Currency code \"{}\" not implemented for \"{}\"",
                    currency,
                    self.lang_name()
                ))
            })?;
            // parse_currency_parts(val, is_int_with_cents=False) reduces to
            // `(abs(val), 0, val < 0)` — the divisor is never consulted.
            let is_negative = v.is_negative();
            let left = v.abs();

            let mut words: Vec<String> = Vec::new();
            if is_negative {
                // Python hardcodes this literal rather than using self.negword.
                // They happen to be the same string; kept as the literal so the
                // two stay independent if negword ever moves.
                words.push("минус".to_string());
            }
            // `self.to_cardinal(left)` dispatches to the SR_LATN override, so
            // this fragment arrives already Latin and the outer cyrl_to_latn
            // re-runs over it idempotently — bug 5's shape again. `left` is
            // non-negative, so `_int2word` never prepends its own "минус".
            // A `left >= 10**33` still raises KeyError from `scale`, exactly as
            // Python does: the int arm has no ceiling of its own.
            // Python: always masculine; the port agrees with the unit (bug 8).
            words.push(self.money_verbose(&left, currency)?);

            words.push(Lang::pluralize(self, &left, &forms.unit)?);

            // `cents`, `separator` and `adjective` are dropped on this path.
            let _ = (cents, separator, adjective);
            return Ok(cyrl_to_latn(&words.join(" ")));
        }

        // Floats/Decimals: `super().to_currency(...)` — Num2Word_Base's, which
        // is what raises NotImplementedError for a code outside the table.
        // The `?` propagates that error untransliterated, as Python's raise
        // unwinds past `cyrl_to_latn` without calling it.
        let out = default_to_currency(self, val, currency, cents, separator, adjective)?;
        Ok(cyrl_to_latn(&out))
    }
}
