//! Port of `lang_SR.py` (Serbian, Cyrillic script).
//!
//! Registry: `__init__.py` maps **both** `"sr"` and `"sr_Cyrl"` to
//! `lang_SR.Num2Word_SR()`. (`"sr_Latn"` is a different class,
//! `Num2Word_SR_LATN`, and is not this file's concern.)
//!
//! Shape: **self-contained**. `Num2Word_SR` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` guard in `Num2Word_Base.__init__` never fires: Python builds
//! neither `self.cards` nor `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `SCALE` table ends
//! at 10^30; `maxval()` is 10^33 and larger values raise `OverflowError`
//! (gladiaio/num2words2#159, see below).
//!
//! `setup()` sets `negword = "минус"` and `pointword = "запета"`. The
//! pointword is reached through the float/Decimal branch of `to_cardinal`,
//! ported below as [`LangSr::cardinal_float_str`].
//!
//! Inherited from `Num2Word_Base` (unchanged by SR, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`
//!   * `to_year(value)        -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below. There is
//!     no era/BC handling: `to_year(-500)` == `to_cardinal(-500)` ==
//!     "минус петсто".
//!
//! # The float branch of `to_cardinal`
//!
//! Python's `to_cardinal` starts with `n = str(number).replace(",", ".")` and
//! branches on `"." in n`. For integral input (`to_cardinal`, [`LangSr::to_cardinal`])
//! `str(BigInt)` never contains `.` or `,`, so control reaches the `else` arm:
//! `self._int2word(int(n), feminine)`. Non-integer input (a Python `float` or
//! `Decimal`) reaches the `"." in n` arm, ported here as
//! [`LangSr::to_cardinal_float`] — `pointword`, leading-zero "нула" prefixes and
//! the whole-number rendering of the fraction included. Because SR overrides
//! `to_cardinal` rather than `to_cardinal_float`, that override is what the
//! dispatcher reaches for floats; SR never inherits `Num2Word_Base`'s
//! digit-by-digit `to_cardinal_float`.
//!
//! Because the branch test is on the *string*, the full float entry
//! ([`Lang::cardinal_float_entry`]) is overridden to send **every**
//! float/Decimal — whole values included — through the string algorithm:
//! `to_cardinal(5.0)` reads `str(5.0)` == "5.0", finds the ".", and renders
//! "пет запета нула" where the base default would say "пет". Values
//! whose Python string form has *no* "." fall to `int(n)`:
//!
//! * `Decimal("5")`, `Decimal("500")` — plain digits, integer path.
//! * `1e+16`, `1e+20` (repr exponent form) and `Decimal("1E+2")` /
//!   `Decimal("1E+20")` (spec `__str__` exponent form) — `int("1e+16")`
//!   raises **ValueError** ("invalid literal for int() with base 10: ...").
//!   That is Python's crash, corpus-pinned, and reproduced by
//!   [`python_number_str`].
//! * `Decimal("Infinity")` (string input "Infinity") — `int("Infinity")` is
//!   the same ValueError; see the `str_to_number` override.
//!
//! `to_year` is Base's `self.to_cardinal(value)`, so the default
//! `year_float_entry` (→ the overridden `cardinal_float_entry`) is already
//! right. `to_ordinal` `int()`s the **value**, not the string — truncation
//! toward zero — so `to_ordinal(2.5)` == "други" and `to_ordinal(1e+16)`
//! *succeeds* ("десет билијардии") where the cardinal raises; see the
//! `ordinal_float_entry` override below.
//!
//! # Grammatical kwargs
//!
//! `to_cardinal(self, number, feminine=False)` is the only SR signature with
//! an extra kwarg. `feminine` flips the `ONES` gender column
//! (`to_cardinal(2, feminine=True)` == "две") — except on the negative
//! recursion, which drops it (quirk 5), and in the teens (one form only).
//! Ported in `to_cardinal_kw`/`to_cardinal_float_kw`; only `bool` and `None`
//! values are accepted (Python treats `feminine=None` as falsy via
//! `feminine or SCALE[...]`; a truthy non-bool like `feminine=2` would
//! `int()` to an out-of-range tuple index — those fall back to Python).
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are exactly what
//! Python emits; none are "fixed" here:
//!
//! 1. *(Fixed, #248.)* Python tabled only 1..=20, the round tens, 100 and
//!    1000 and glued "и" onto the cardinal for the rest ("двадесет
//!    једани"). Compound ordinals now inflect their last word
//!    ([`crate::compound_ordinal`]): 21 == "двадесет први", 101 == "сто
//!    први", 1100 == "хиљада стоти", 200 == "двестоти", 0 == "нулти",
//!    10**6 == "милионити". Round thousands and larger round values other
//!    than 10**6 (2000, 10**9, …) still take Python's "и" fallback ("две
//!    хиљадеи"); that remains a known gap.
//! 2. *(Fixed, #248.)* A negative ordinal is "минус" + the ordinal
//!    (`to_ordinal(-1)` == "минус први"); SR still never raises for it.
//! 3. **`pluralize` treats `n % 100 == 10` as a teen.** The guard is
//!    `if number % 100 < 10 or number % 100 > 20`, so a remainder of exactly
//!    10 falls to the `else` and takes form 2. Hence `to_cardinal(10**10)`
//!    == "десет милијарди" (genitive plural), not "десет милијарда".
//! 4. **The "skip један" test compares the whole chunk, not the digit.**
//!    Python writes `if not (chunk_len > 0 and chunk == 1)`. So the unit word
//!    is suppressed only when the scaled chunk is *exactly* 1 — giving
//!    "хиљада" for 1000 and "милион" for 10**6 — while 1001's low chunk
//!    (chunk_len == 0) still renders "један": "хиљада један".
//! 5. **`_int2word` drops `feminine` on the negative recursion.** Python:
//!    `" ".join([self.negword, self._int2word(abs(number))])` — the
//!    `feminine` argument is not forwarded, so `_int2word(-2, feminine=True)`
//!    yields "минус два" rather than "минус две". Unreachable from the four
//!    ported modes (they always pass `feminine=False`), but reproduced.
//! 6. **Fixed (gladiaio/num2words2#159): `SCALE` ends at 10^30.** Keys run
//!    0..=10, i.e. up to 10^30 ("квинтилион"), covering values below 10^33.
//!    At or above 10^33 Python reached chunk index 11+ and died with
//!    `KeyError: 11`. `maxval()` is now 10^33 and [`LangSr::int2word`] raises
//!    `OverflowError` for `abs(n) >= 10**33` before touching [`scale_at`].
//! 7. **Long scale.** `SCALE[3]` is "милијарда" for 10^9 and `SCALE[4]` is
//!    "билион" for 10^12, as the Python comments state explicitly.
//!
//! # The currency surface
//!
//! `Num2Word_SR` defines its **own** `CURRENCY_FORMS` class attribute, so the
//! `lang_EUR.py`/`Num2Word_EN.__init__` shared-dict mutation documented in
//! `PORTING_CURRENCY.md` does **not** reach it: SR knows exactly three codes
//! (RUB, EUR, RSD) and nothing else. It defines neither `CURRENCY_ADJECTIVES`
//! nor `CURRENCY_PRECISION`, so both stay at `Num2Word_Base`'s empty dicts —
//! every code has precision `.get(code, 100)` == 100. SR therefore has **no**
//! 3-decimal (KWD/BHD) and no 0-decimal (JPY) currency, and the `divisor == 1`
//! branch of `Num2Word_Base.to_currency` is unreachable here. Verified live.
//!
//! Each `CURRENCY_FORMS` entry is a pair of **4-tuples**: three plural forms
//! plus a trailing gender flag, e.g. `("dinar", "dinara", "dinara", False)`.
//! `pluralize` only ever selects index 0..=2, so the flag is never a plural
//! form; `_cents_verbose` reads it as `[1][-1]` to pick the cents' gender.
//! See [`build_currency_forms`] for why the flag is carried as the *string*
//! `"False"`/`"True"` rather than a `bool`.
//!
//! # Faithfully reproduced Python quirks (currency)
//!
//! 8. **`to_currency` ignored `currency=` for `int` input (fixed, #176).**
//!    SR's Python override intercepts `isinstance(val, int)` *before* any
//!    `CURRENCY_FORMS` lookup and appends a hardcoded "динар"/"динара", so
//!    `to_currency(1, currency="EUR")` was "један динар" and even a
//!    nonexistent code succeeded. The port looks the code up first — an
//!    unknown code raises `NotImplementedError`, as on the float path — and
//!    picks the unit with [`LangSr::pluralize`]: "један евро", "пет евра".
//!    RSD is unchanged ("динара" is both its few and many form).
//! 9. **The `elif` in Python's dinar rule was dead code.** Python writes
//!    `if left % 10 == 1 and left % 100 != 11: "динар"` /
//!    `elif 2 <= left % 10 <= 4 and not (12 <= left % 100 <= 14): "динара"` /
//!    `else: "динара"` — the last two arms are byte-identical, so the elif can
//!    never change the output. Moot since #176: the port uses
//!    [`LangSr::pluralize`] on the table's forms instead.
//! 10. **`_money_verbose` dropped the unit's gender (fixed, #188).** It is
//!    `Num2Word_Base`'s, i.e. `self.to_cardinal(number)`, which passes
//!    `feminine=False`, and the int shortcut did the same. So a feminine unit
//!    got a masculine numeral: `to_currency(21, "RUB")` == "двадесет један
//!    рубља". [`LangSr::money_verbose`] now reads the unit tuple's gender flag
//!    (as `_cents_verbose` already did for the subunit) and the int path goes
//!    through it: "двадесет једна рубља", "две рубље". Only the units word is
//!    re-gendered ([`feminine_last`]); scale words keep their own gender
//!    ("два милиона").
//! 11. **`to_cheque` printed the gender flag as the currency name (fixed,
//!    #176).** `Num2Word_Base.to_cheque` takes `cr1[-1]` as "the plural
//!    form", but SR's `cr1` is a 4-tuple whose last element is the gender
//!    **bool**, so Python printed "ДВАНАЕСТ AND 50/100 FALSE" (RUB: "TRUE").
//!    [`LangSr::to_cheque`] takes the "many" form (index 2) instead:
//!    "ДВАНАЕСТ AND 50/100 ЕВРА". The "AND"/"MINUS" words stay English, as
//!    in every other language's cheque format.
//!
//! # Fractional cents (`cardinal_from_decimal`) — closed by dynamic dispatch
//!
//! Reached only when `(value * 100) % 1 != 0` on the currency path. Python
//! evaluates `self.to_cardinal(float(right))`, which lands in
//! **`Num2Word_SR.to_cardinal`'s float branch**. The trait default
//! (`floatpath::cardinal_from_bigdecimal`) calls `lang.to_cardinal_float`
//! *through the trait*, and SR overrides that hook with the very same string
//! algorithm, so the fractional digits render as a whole number ("петсто
//! шездесет седам"), not digit by digit — matching Python with no
//! `cardinal_from_decimal` override. (Unpinned: no `sr` currency corpus row
//! carries more than 2 decimals.)
//!
//! # Error variants
//!
//! The `SCALE` `KeyError` of quirk 6 is still mapped to `N2WError::Key` in
//! [`scale_at`], but since #159 the MAXVAL check makes it unreachable from
//! the public entry points: too-large values are an `OverflowError`.
//!
//! On the currency side the only reachable raise is the deliberate
//! `NotImplementedError` for an unknown code on the float path, which
//! `currency::default_to_currency`/`default_to_cheque` already emit with
//! Python's exact message from [`LangSr::lang_name`].

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::{default_to_currency, parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `ZERO[0]`.
const ZERO: &str = "нула";

/// `setup()`: `self.negword`.
const NEGWORD: &str = "минус";

/// `setup()`: `self.pointword`. Exposed via the `pointword()` trait method,
/// but unreachable from the four ported modes: only the float branch of
/// `to_cardinal` (out of scope) consumes it.
const POINTWORD: &str = "запета";

/// `ONES`: digit → (masculine, feminine). Python's dict has keys 1..=9 only;
/// index 0 is a placeholder, unreachable behind the `digit_right > 0` guard.
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

/// `TENS`: keys 0..=9, indexed by the *units* digit when the tens digit is 1.
/// `TENS[0]` == "десет" (10), `TENS[1]` == "једанаест" (11), etc.
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

/// `TWENTIES`: Python's dict has keys 2..=9 only; 0 and 1 are placeholders,
/// unreachable behind the `digit_mid > 1` guard.
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

/// `HUNDREDS`: Python's dict has keys 1..=9 only; index 0 is a placeholder,
/// unreachable behind the `digit_left > 0` guard.
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

/// `SCALE`: chunk index → (form0, form1, form2, is_feminine).
///
/// Keys 0..=10 exactly as in Python — 10^0 through 10^30. Index 11 and above
/// would be a `KeyError`; the 10^33 MAXVAL check keeps it unreachable (quirk
/// 6). The trailing `bool` is
/// read as `SCALE[chunk_len][-1]` to pick the gender of the unit word, and is
/// never a `pluralize` output (`pluralize` only ever returns index 0, 1 or 2).
const SCALE: [(&str, &str, &str, bool); 11] = [
    ("", "", "", false),
    ("хиљада", "хиљаде", "хиљада", true), // 10^3
    ("милион", "милиона", "милиона", false), // 10^6
    ("милијарда", "милијарде", "милијарди", true), // 10^9 - long scale
    ("билион", "билиона", "билиона", false), // 10^12
    ("билијарда", "билијарде", "билијарди", true), // 10^15
    ("трилион", "трилиона", "трилиона", false), // 10^18
    ("трилијарда", "трилијарде", "трилијарди", true), // 10^21
    ("квадрилион", "квадрилиона", "квадрилиона", false), // 10^24
    ("квадрилијарда", "квадрилијарде", "квадрилијарди", true), // 10^27
    ("квинтилион", "квинтилиона", "квинтилиона", false), // 10^30
];

/// Ordinal hundreds 100..=900 (#248). Index 0 is unused.
const HUNDREDS_ORD: [&str; 10] = [
    "", "стоти", "двестоти", "тристоти", "четиристоти", "петстоти", "шестстоти", "седамстоти",
    "осамстоти", "деветстоти",
];

/// The `ordinals` dict local to `to_ordinal`, in Python's insertion order.
///
/// Note the gaps: 21..=29, 31..=39, ..., and everything above 100 except
/// 1000, are absent and fall through to the "и"-suffix path.
const ORDINALS: [(i64, &str); 30] = [
    (1, "први"),
    (2, "други"),
    (3, "трећи"),
    (4, "четврти"),
    (5, "пети"),
    (6, "шести"),
    (7, "седми"),
    (8, "осми"),
    (9, "девети"),
    (10, "десети"),
    (11, "једанаести"),
    (12, "дванаести"),
    (13, "тринаести"),
    (14, "четрнаести"),
    (15, "петнаести"),
    (16, "шеснаести"),
    (17, "седамнаести"),
    (18, "осамнаести"),
    (19, "деветнаести"),
    (20, "двадесети"),
    (30, "тридесети"),
    (40, "четрдесети"),
    (50, "педесети"),
    (60, "шездесети"),
    (70, "седамдесети"),
    (80, "осамдесети"),
    (90, "деведесети"),
    (100, "стоти"),
    (1000, "хиљадити"),
    (0, ""), // padding; never matched — see `ordinal_word`
];

/// Python's `num in ordinals` → `ordinals[num]`.
///
/// The `(0, "")` padding row in [`ORDINALS`] is skipped explicitly: 0 is *not*
/// a key in Python's dict; [`ordinal_cyrl`] handles 0 ("нулти", #248).
fn ordinal_word(num: &BigInt) -> Option<&'static str> {
    let n = num.to_i64()?;
    if n == 0 {
        return None;
    }
    ORDINALS
        .iter()
        .find(|(k, _)| *k == n)
        .map(|(_, w)| *w)
}

/// `to_ordinal` in Cyrillic, shared with `lang_sr_latn` (which
/// transliterates the result). `cardinal` renders the Cyrillic cardinal.
pub(crate) fn ordinal_cyrl(
    value: &BigInt,
    cardinal: &dyn Fn(&BigInt) -> Result<String>,
) -> Result<String> {
    if value.is_negative() {
        return Ok(format!("{} {}", NEGWORD, ordinal_cyrl(&value.abs(), cardinal)?));
    }
    if value.is_zero() {
        return Ok("нулти".to_string());
    }
    if let Some(word) = ordinal_word(value) {
        return Ok(word.to_string());
    }
    // Propagates the MAXVAL OverflowError for huge inputs (#159).
    let card = cardinal(value)?;
    if let Some(n) = value.to_u64() {
        if n == 1_000_000 {
            return Ok("милионити".to_string());
        }
        let small = |v: u64| ordinal_word(&BigInt::from(v));
        if let Some(word) =
            crate::compound_ordinal::last_word_ordinal(n, &card, small, Some(&HUNDREDS_ORD))
        {
            return Ok(word);
        }
    }
    // Python's "simplified implementation" fallback (known gap, see quirk 1).
    Ok(format!("{}и", card))
}

/// `SCALE[idx]`. Missing keys are Python's `KeyError` (unreachable since the
/// MAXVAL check; see quirk 6).
fn scale_at(idx: usize) -> Result<&'static (&'static str, &'static str, &'static str, bool)> {
    SCALE
        .get(idx)
        .ok_or_else(|| N2WError::Key(idx.to_string()))
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
///
/// `n` is always `str(abs(number))` here — `_int2word` takes `abs()` before
/// chunking — so every chunk is a run of 1..=3 ASCII digits and therefore
/// fits `u32` (max 999). The head chunk `n[:start]` is 1 or 2 digits (max 99).
/// This is why SR, unlike PL, has no `int("-")` `ValueError` hazard.
///
/// The `parse` failure is mapped to Python's `int()` `ValueError` for
/// completeness; the invariant above means it is unreachable in practice.
fn splitbyx(n: &str, x: usize) -> Result<Vec<u32>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let parse = |s: String| -> Result<u32> {
        s.parse::<u32>().map_err(|_| {
            N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
        })
    };

    let mut out: Vec<u32> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse(chars[..start].iter().collect())?);
        }
        let mut i = start;
        while i < length {
            let end = (i + x).min(length);
            out.push(parse(chars[i..end].iter().collect())?);
            i += x;
        }
    } else {
        out.push(parse(n.to_string())?);
    }
    Ok(out)
}

/// Port of `utils.get_digits(n)`: `[int(x) for x in reversed(("%03d" % n)[-3:])]`.
///
/// Returns `[units, tens, hundreds]` — Python unpacks it as
/// `digit_right, digit_mid, digit_left`. `n <= 999` (see [`splitbyx`]), so
/// `"%03d"` yields exactly 3 digits and the `[-3:]` slice is total.
fn get_digits(n: u32) -> [usize; 3] {
    let s = format!("{:03}", n);
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).unwrap_or(0) as usize;
    }
    a
}

/// Port of `Num2Word_SR.pluralize(number, forms)`.
///
/// `forms` is a `SCALE` row; only indices 0..=2 are ever selected, so the
/// trailing gender flag is never returned. Reproduces quirk 3: a remainder of
/// exactly 10 is *not* `< 10` and *not* `> 20`, so it takes form 2.
///
/// `number` is a non-negative chunk, so Rust's `%` matches Python's here.
fn pluralize(
    number: u32,
    forms: &'static (&'static str, &'static str, &'static str, bool),
) -> &'static str {
    let form = if number % 100 < 10 || number % 100 > 20 {
        if number % 10 == 1 {
            0
        } else if 1 < number % 10 && number % 10 < 5 {
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

/// Python's `str(number)` for a float/Decimal input — the string
/// `Num2Word_SR.to_cardinal` reads (`n = str(number).replace(",", ".")`).
///
/// * **float** — `repr(float)`. Fixed notation (`{:.*}` at the shim's
///   repr-derived precision, so `str(Decimal("1.10"))`-style trailing zeros
///   and `-0.0`'s sign survive) whenever Python's repr shows a decimal
///   point; otherwise repr picked exponent form (`abs(v) >= 1e16`:
///   "1e+16"), reconstructed by [`python_float_exp_repr`]. The exponent form
///   is load-bearing: it has no ".", so `to_cardinal` falls through to
///   `int("1e+16")` and raises ValueError — corpus-pinned, while
///   `to_ordinal(1e+16)` (which `int()`s the *value*) succeeds.
/// * **Decimal** — [`python_decimal_str`], the spec `__str__`: trailing
///   zeros kept ("5.00" -> right "00", two leading zeros *plus*
///   `int("00") == 0`, hence the tripled "нула"), scientific form for
///   positive exponents ("1E+2", "1E+20" — again a ValueError from `int()`).
///
/// Known gap (unpinned): a float in `(0, 1e-4)` also reprs in exponent form
/// ("1e-05"), but `FloatValue::has_visible_point` reports `true` for every
/// finite fractional float, so such values take the fixed-notation arm here
/// and render instead of raising. No `sr` corpus row reaches that range.
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

/// Extract SR's `feminine=` kwarg. Python's signature is
/// `to_cardinal(self, number, feminine=False)`; the flag only ever feeds
/// `feminine or SCALE[chunk_len][-1]` followed by `int(is_feminine)`, so:
///
/// * absent / `False` / explicit `None` (falsy) -> masculine column;
/// * `True` -> feminine column;
/// * any *other* key, or a non-bool value (`feminine=2` would `int()` to a
///   tuple index Python crashes on) -> `NotImplemented`, so the dispatcher
///   falls back to the original Python and its genuine semantics.
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

/// `Num2Word_SR.CURRENCY_FORMS` — RUB, EUR, RSD and nothing else.
///
/// Python spells these names in Latin script ("dinar", "evro", ...) inside
/// otherwise Cyrillic output ("један dinar, педесет para"). This port
/// transliterates them to Cyrillic (gladiaio/num2words2#154); Serbian
/// Latin/Cyrillic is a one-to-one mapping, and `sr_Latn` keeps its own Latin
/// table.

///
/// # Why the gender flag is a `"False"`/`"True"` string
///
/// Python's entries are pairs of **4-tuples**: `("dinar", "dinara", "dinara",
/// False)`. That trailing element is a `bool`, and it is read from two places
/// that want two different things out of it:
///
/// * `Num2Word_SR._cents_verbose` reads `CURRENCY_FORMS[cur][1][-1]` and uses
///   it as the `feminine` argument — its intended purpose.
/// * `Num2Word_Base.to_cheque` reads `cr1[-1]` believing it is the plural unit
///   name — quirk 11, fixed (#176) by [`LangSr::to_cheque`], which reads the
///   "many" form at index 2 instead.
///
/// `CurrencyForms` stores `Vec<String>`, so the flag is kept as the text
/// `"True"`/`"False"` and [`LangSr::cents_verbose`] recovers the boolean by
/// comparing against `"True"`.
///
/// The arity is load-bearing beyond that: [`LangSr::pluralize`] indexes 0..=2,
/// so dropping the third form would silently change output.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "RUB",
        CurrencyForms::new(
            &["рубља", "рубље", "рубљи", "True"],
            &["копејка", "копејке", "копејки", "True"],
        ),
    );
    m.insert(
        "EUR",
        CurrencyForms::new(
            &["евро", "евра", "евра", "False"],
            &["цент", "цента", "центи", "False"],
        ),
    );
    m.insert(
        "RSD",
        CurrencyForms::new(
            &["динар", "динара", "динара", "False"],
            &["пара", "паре", "пара", "True"],
        ),
    );
    m
}

/// Swap a final masculine `ONES` word for its feminine form ("двадесет
/// један" → "двадесет једна", "два" → "две"); shared with `sr_Latn`.
///
/// `_int2word`'s own `feminine` flag would also feminize the ones word of
/// every higher chunk ("две милиона"), which is wrong for the masculine
/// scale words. The last token of `_int2word` is a `ONES` word exactly when
/// the units chunk ends in one, so swapping that token genders the units
/// alone (#188). Takes and returns Cyrillic.
pub(crate) fn feminine_last(cyr: &str) -> String {
    let (head, last) = match cyr.rsplit_once(' ') {
        Some((h, l)) => (Some(h), l),
        None => (None, cyr),
    };
    match ONES.iter().skip(1).find(|(m, _)| *m == last) {
        Some((_, f)) => match head {
            Some(h) => format!("{} {}", h, f),
            None => f.to_string(),
        },
        None => cyr.to_string(),
    }
}

/// `Num2Word_Base.to_cheque` for SR's 4-tuple forms, shared with `sr_Latn`
/// and `hr` (whose `CURRENCY_FORMS` has the same shape, #189).
///
/// Identical to `currency::default_to_cheque` except for the unit word:
/// Base takes `cr1[-1]`, which for SR is the gender flag ("FALSE"/"TRUE"),
/// so this takes the "many" form at index 2 — "ДВАНАЕСТ AND 50/100 ЕВРА"
/// (quirk 11, #176).
pub(crate) fn sr_to_cheque<L: Lang + ?Sized>(
    lang: &L,
    val: &BigDecimal,
    currency: &str,
) -> Result<String> {
    let forms = lang.currency_forms(currency).ok_or_else(|| {
        N2WError::NotImplemented(format!(
            "Currency code \"{}\" not implemented for \"{}\"",
            currency,
            lang.lang_name()
        ))
    })?;
    let is_negative = val.is_negative();
    let abs_val = val.abs();
    let whole = abs_val.with_scale(0).as_bigint_and_exponent().0;
    // SR's (and HR's) CURRENCY_PRECISION is empty, so the divisor is always 100.
    let sub = ((&abs_val - BigDecimal::from(whole.clone())) * BigDecimal::from(100))
        .with_scale(0)
        .as_bigint_and_exponent()
        .0;
    let words = lang.money_verbose(&whole, currency)?;
    let unit = forms
        .unit
        .get(2)
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
    let sign = if is_negative { "MINUS " } else { "" };
    Ok(format!("{}{} AND {:0>2}/100 {}", sign, words, sub.to_string(), unit).to_uppercase())
}

pub struct LangSr {
    /// Built once in [`LangSr::new`] and only ever read. `Num2Word_SR` reads
    /// `CURRENCY_FORMS` as a class attribute, so rebuilding it per call would
    /// be both wrong in spirit and measurably slower than the Python.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangSr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSr {
    pub fn new() -> Self {
        LangSr {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_SR._int2word(number, feminine=False)`.
    ///
    /// **`SCALE` access order is load-bearing** (quirk 6). Python touches
    /// `SCALE` in exactly two places per chunk, and both are conditional:
    ///
    /// 1. `SCALE[chunk_len][-1]`, only inside the `elif digit_right > 0`
    ///    branch *and* only when the "skip један" test lets it through. So a
    ///    chunk whose tens digit is 1 (the `if digit_mid == 1` arm) never
    ///    reaches it.
    /// 2. `pluralize(chunk, SCALE[chunk_len])`, only when
    ///    `chunk_len > 0 and chunk != 0`.
    ///
    /// In practice the ceiling is sharp at 10^33: the leading chunk of
    /// `str(number)` is never zero, so the highest chunk index always clears
    /// the `chunk != 0` guard of access 2 and raises. (Verified: 3000 random
    /// values in 10^33..10^45 all raise, including ones whose leading chunk
    /// takes the `digit_mid == 1` arm and thus skips access 1.) Since #159 an
    /// up-front `check_maxval` turns that into `OverflowError`, so neither
    /// access can miss.
    fn int2word(&self, number: &BigInt, feminine: bool) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        if number.is_negative() {
            // Python: " ".join([self.negword, self._int2word(abs(number))])
            // `feminine` is NOT forwarded — quirk 5, reproduced verbatim.
            return Ok(format!("{} {}", NEGWORD, self.int2word(&number.abs(), false)?));
        }

        if number.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<&str> = Vec::new();
        let chunks = splitbyx(&number.to_string(), 3)?;
        let mut chunk_len = chunks.len();

        for chunk in chunks {
            // `chunks` is non-empty, so this never underflows.
            chunk_len -= 1;
            let [digit_right, digit_mid, digit_left] = get_digits(chunk);

            if digit_left > 0 {
                words.push(HUNDREDS[digit_left]);
            }

            if digit_mid > 1 {
                words.push(TWENTIES[digit_mid]);
            }

            if digit_mid == 1 {
                words.push(TENS[digit_right]);
            } else if digit_right > 0 {
                // Python: `if not (chunk_len > 0 and chunk == 1)` — the test is
                // on the whole chunk, not the digit (quirk 4).
                if !(chunk_len > 0 && chunk == 1) {
                    let is_feminine = feminine || scale_at(chunk_len)?.3;
                    let gender_idx = usize::from(is_feminine);
                    let ones = ONES[digit_right];
                    words.push(if gender_idx == 0 { ones.0 } else { ones.1 });
                }
            }

            if chunk_len > 0 && chunk != 0 {
                words.push(pluralize(chunk, scale_at(chunk_len)?));
            }
        }

        Ok(words.join(" "))
    }

    /// Port of `Num2Word_SR.to_cardinal(number, feminine=False)` for a
    /// float/Decimal `number` — the whole method, both branches:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count \
    ///         + self._int2word(int(right), feminine)
    ///     result = "%s %s %s" % (self._int2word(int(left), feminine),
    ///                            self.pointword, decimal_part)
    ///     if is_negative:
    ///         result = self.negword + " " + result
    ///     return result
    /// else:
    ///     return self._int2word(int(n), feminine)
    /// ```
    ///
    /// Two properties set SR apart from the inherited digit-by-digit float
    /// path and are load-bearing:
    ///
    /// 1. **The fraction is one whole number, not a digit sequence.**
    ///    `int(right)` is fed to `_int2word`, so `2.675` -> "два запета
    ///    шесто седамдесет пет" (675 as a number). SR never touches
    ///    `float2tuple`, so the f64-artefact `< 0.01` heuristic is
    ///    irrelevant: it reads `repr(2.675)` == "2.675" and parses "675".
    /// 2. **Leading fraction zeros become "нула" words.** `0.01` splits to
    ///    `right == "01"`, one leading zero, so `int("01") == 1` is prefixed
    ///    by a single "нула": "нула запета нула један". An all-zero `right`
    ///    reads one "нула" per digit written ("0" from `1.0` -> "нула");
    ///    Python counted it as a leading zero *and* `int("0")`, doubling it
    ///    (fixed, gladiaio/num2words2#237).
    ///
    /// The `else` arm is where the exponent-form strings die: `int("1e+16")`
    /// / `int("1E+2")` raise ValueError with Python's exact message.
    ///
    /// `feminine` comes from the kwarg surface (default `False`); the
    /// negative recursion inside `int2word` still drops it (quirk 5).
    fn cardinal_float_str(&self, value: &FloatValue, feminine: bool) -> Result<String> {
        // n = str(number).replace(",", "."). The reconstruction never emits a
        // comma, so the replace is a no-op; the sign is kept on the string so
        // the `startswith("-")` test below matches Python byte for byte.
        let n = python_number_str(value);

        // Python's int() on a token of str(number). Reachable failures are
        // the exponent forms ("1e+16", "1E+2", "inf") — exactly where Python
        // raises ValueError, message included.
        let to_int = |s: &str| -> Result<BigInt> {
            s.parse::<BigInt>().map_err(|_| {
                N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
            })
        };

        match n.find('.') {
            Some(_) => {
                let is_negative = n.starts_with('-');
                // abs_n = n[1:] if is_negative else n. The "-" is one ASCII byte.
                let abs_n = if is_negative { &n[1..] } else { &n[..] };
                // left, right = abs_n.split("."). Exactly one "." is present.
                let (left, right) = abs_n.split_once('.').unwrap();

                // leading_zero_count = len(right) - len(right.lstrip("0")). A
                // fully-zero `right` counts every char, matching lstrip("0")=="".
                // The final int(right) word already says one zero, so an all-zero
                // fraction gets len - 1 leading zeros: exactly the digits written
                // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
                let leading_zero_count = right.chars().take_while(|&c| c == '0').count()
                    .min(right.len().saturating_sub(1));

                // decimal_part = (ZERO[0] + " ") * leading_zero_count
                //                + self._int2word(int(right), feminine)
                let mut decimal_part = format!("{} ", ZERO).repeat(leading_zero_count);
                decimal_part.push_str(&self.int2word(&to_int(right)?, feminine)?);

                // result = "%s %s %s" % (_int2word(int(left), feminine),
                //                        pointword, decimal_part)
                let mut result = format!(
                    "{} {} {}",
                    self.int2word(&to_int(left)?, feminine)?,
                    POINTWORD,
                    decimal_part
                );

                // if is_negative: result = self.negword + " " + result
                if is_negative {
                    result = format!("{} {}", NEGWORD, result);
                }
                Ok(result)
            }
            None => {
                // else: return self._int2word(int(n), feminine). int(n) keeps
                // the sign, and int2word renders "минус ..." for a negative.
                self.int2word(&to_int(&n)?, feminine)
            }
        }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangSr {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "RSD"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "запета"
    }

    /// Port of `Num2Word_SR.to_cardinal(number, feminine=False)`, integral arm.
    ///
    /// A plain call never passes `feminine=True` (the currency path does via
    /// `_cents_verbose`, and the kwarg surface via `to_cardinal_kw`), so this
    /// delegates with `feminine = false`. The float/Decimal branch lives in
    /// [`LangSr::cardinal_float_str`].
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, false)
    }

    /// Port of `Num2Word_SR.to_ordinal(number)`.
    ///
    /// Python's `try: num = int(number) / except (ValueError, TypeError):
    /// return str(number)` guard cannot trigger for a `BigInt`, so it is
    /// omitted. Outside the small `ordinals` table the last word of the
    /// cardinal is inflected (#248) — quirks 1 and 2.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        ordinal_cyrl(value, &|v| self.to_cardinal(v))
    }

    /// The raw float grammar — [`LangSr::cardinal_float_str`] with
    /// `feminine=False`.
    ///
    /// `precision_override` (the base's issue-#580 `precision=` kwarg) is
    /// **not** a parameter of `Num2Word_SR.to_cardinal`, which reads
    /// `str(number)` and consults no per-language precision, so it is ignored
    /// — matching Python, where setting `converter.precision` leaves this
    /// method's output untouched. This hook also serves the fractional-cents
    /// currency path (`cardinal_from_decimal` -> `cardinal_from_bigdecimal`
    /// dispatches through the trait), mirroring Python's virtual
    /// `self.to_cardinal(float(right))`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_str(value, false)
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// SR overrides `to_cardinal` itself and branches on `"." in str(number)`,
    /// so a whole value with a visible point still takes the float grammar:
    /// `5.0` -> "пет запета нула", `Decimal("5.00")` -> "пет запета нула
    /// нула", `-0.0` -> "минус нула запета нула". A pointless string
    /// falls to `int(n)`: `Decimal("5")` -> "пет", but `1e+16` / `Decimal("1E+2")`
    /// raise ValueError. The base default (whole -> int path) would get every
    /// one of those wrong, hence this override.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_str(value, false)
    }

    /// `to_ordinal(float/Decimal)`. Python's `to_ordinal` opens with
    /// `num = int(number)` — `int()` of the *value*, truncation toward zero —
    /// so `2.5` -> 2 -> "други", `-1.5` -> -1 -> "минус први", and `1e+16`
    /// *succeeds* ("десет билијардии") where the cardinal raises ValueError.
    ///
    /// The `except (ValueError, TypeError): return str(number)` guard can
    /// only fire here for `nan` (`int(nan)` is ValueError, so Python returns
    /// `str(nan)` == "nan"); `int(inf)` raises OverflowError, which the guard
    /// does **not** catch. Neither case has a corpus row; both are mirrored
    /// for completeness.
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
        self.to_ordinal(&num)
    }

    // `year_float_entry` is deliberately NOT overridden: Base's `to_year` is
    // `self.to_cardinal(value)`, and the trait default routes through the
    // overridden `cardinal_float_entry` above — so `to_year(5.0)` == "пет
    // запета нула" and `to_year(1e+16)` raises ValueError, as pinned.
    // `ordinal_num_float_entry` stays at the default too: SR never defines
    // `to_ordinal_num`, and the dispatcher's getattr fallback echoes
    // `str(value)` — exactly the default's repr echo.

    /// `converter.str_to_number` — Base's `Decimal(value)`, which SR does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `n = str(number)`
    /// == "Infinity", finds no ".", and dies in `int("Infinity")` with
    /// ValueError. The binding otherwise maps `ParsedNumber::Inf` to the base
    /// integer path's OverflowError before any SR code runs, so the
    /// ValueError must be raised here. (NaN needs no interception: the
    /// binding's ValueError already matches `int("NaN")`'s type.)
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

    /// `to_cardinal(number, feminine=...)` — the only SR signature with an
    /// extra kwarg. `feminine=True` picks the feminine `ONES` column:
    /// "једна", "две", "двадесет једна". The negative recursion still drops
    /// the flag (quirk 5): `to_cardinal(-5, feminine=True)` == "минус пет".
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        let feminine = feminine_kwarg(kw)?;
        self.int2word(value, feminine)
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
        self.cardinal_float_str(value, feminine)
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_SR` overrides exactly three things on this surface —
    // `CURRENCY_FORMS`, `pluralize` and `_cents_verbose` — plus `to_currency`
    // for its int shortcut. `_money_verbose` is overridden here too, to honour
    // the unit's gender flag (quirk 10, #188). Everything else (`_cents_terse`,
    // `to_cheque`, and the whole float path) is `Num2Word_Base`'s, which the
    // trait defaults already mirror, so it is deliberately not overridden:
    //
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is `{}`; default `None`
    //     is right, and `adjective=True` is a silent no-op exactly as in
    //     Python (`if adjective and currency in self.CURRENCY_ADJECTIVES`).
    //   * `currency_precision` — `CURRENCY_PRECISION` is `{}`; every code takes
    //     the `.get(code, 100)` default of 100.
    //   * `cents_terse` — Base's `"%0*d"`, width `len("100") - 1` == 2.
    //   * `to_cheque` — Base's, except for the unit word (quirk 11, #176);
    //     see [`sr_to_cheque`].

    fn lang_name(&self) -> &str {
        "Num2Word_SR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_SR.pluralize(number, forms)` for the currency path.
    ///
    /// Identical rule to the private [`pluralize`] free function above, which
    /// serves `_int2word`'s `SCALE` rows; Python has one method doing both
    /// jobs, but the two call sites carry different types (`u32` + a `SCALE`
    /// tuple vs. `BigInt` + a `CURRENCY_FORMS` tuple) and the `_int2word` path
    /// is already verified against the corpus, so it is left untouched rather
    /// than generalised underneath it.
    ///
    /// Reproduces quirk 3 here too: `number % 100 == 10` is neither `< 10` nor
    /// `> 20`, so it falls to form 2 ("десет евра", not "десет евро").
    ///
    /// `mod_floor` rather than `%`: Python's `%` floors on negatives. `left`
    /// and `right` both arrive non-negative from `parse_currency_parts`, so
    /// the two agree in practice, but the port should not depend on that.
    ///
    /// Python indexes `forms[form]` directly, so a table entry with fewer than
    /// three forms would raise IndexError. All three of SR's entries carry four
    /// elements, so this is unreachable — mapped rather than panicking so the
    /// exception type survives if the table ever changes.
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
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_Base.to_cheque` with the unit word fixed — quirk 11.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        sr_to_cheque(self, val, currency)
    }

    /// Base's `_money_verbose` (`self.to_cardinal(number)`), with the units
    /// word agreeing with the unit's gender flag `[0][-1]` (quirk 10, #188):
    /// "двадесет једна рубља", "две рубље"; EUR/RSD stay masculine.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let words = self.to_cardinal(number)?;
        let feminine = self
            .currency_forms
            .get(currency)
            .and_then(|f| f.unit.last())
            .map_or(false, |flag| flag.as_str() == "True");
        Ok(if feminine { feminine_last(&words) } else { words })
    }

    /// Port of `Num2Word_SR._cents_verbose(number, currency)`:
    /// `self._int2word(number, self.CURRENCY_FORMS[currency][1][-1])`.
    ///
    /// Note it calls `_int2word` directly rather than `to_cardinal`, and that
    /// the gender flag is the subunit tuple's trailing element — so RSD's
    /// `пара` (flag `True`) gets feminine numerals ("једна пара", "две паре")
    /// while EUR's `cent` (flag `False`) stays masculine ("један cent").
    ///
    /// The `CURRENCY_FORMS[currency]` lookup is Python's `KeyError`, but it is
    /// unreachable: `Num2Word_Base.to_currency` resolves `cr1, cr2` from the
    /// same dict, and raises `NotImplementedError` on a miss, before this can
    /// run. Same story for the `[-1]` IndexError on an empty tuple.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let forms = self
            .currency_forms
            .get(currency)
            .ok_or_else(|| N2WError::Key(format!("'{}'", currency)))?;
        let flag = forms
            .subunit
            .last()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
        // The flag is stored as Python's own `"%s" % bool` text; see
        // `build_currency_forms` for why.
        self.int2word(number, flag.as_str() == "True")
    }

    /// Port of `Num2Word_SR.to_currency(val, currency="RSD", cents=True,
    /// separator=",", adjective=False)`.
    ///
    /// Only the `isinstance(val, int)` shortcut is SR's own; every other value
    /// is handed to `super().to_currency(...)` unchanged. In Python that
    /// shortcut never touched `CURRENCY_FORMS`; the port looks the code up
    /// (quirk 8, #176).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` means the caller omitted `separator=`, so SR's own default
        // (Base's ",", per the live signature) applies. Python passes whatever
        // it resolved straight through to `super().to_currency`.
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(_) = val {
            // Python never looked the code up here (quirk 8, #176); the port
            // does, so an unknown code raises like the float path.
            let forms = self.currency_forms.get(currency).ok_or_else(|| {
                N2WError::NotImplemented(format!(
                    "Currency code \"{}\" not implemented for \"{}\"",
                    currency,
                    self.lang_name()
                ))
            })?;
            // parse_currency_parts(val, is_int_with_cents=False). `right` is
            // always 0 on this path and Python discards it; the divisor is
            // likewise unused, so 100 just mirrors the Python default.
            let (left, _right, is_negative) = parse_currency_parts(val, false, false, 100);

            let mut words: Vec<String> = Vec::new();
            if is_negative {
                // Python hardcodes the literal here rather than reading
                // `self.negword`. Same text, but kept distinct on purpose.
                words.push("минус".to_string());
            }
            // `left` is already `abs(val)`. Python: `self.to_cardinal(left)`,
            // always masculine; the port agrees with the unit (quirk 10, #188).
            words.push(self.money_verbose(&left, currency)?);
            words.push(Lang::pluralize(self, &left, &forms.unit)?);

            return Ok(words.join(" "));
        }

        // Floats/Decimals: `super().to_currency(...)`, which is where an
        // unknown code finally raises NotImplementedError.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
