//! Port of `lang_TH.py` (Thai). Registry key `"th"` → `Num2Word_TH`
//! (verified in `num2words2/__init__.py`).
//!
//! Shape: **self-contained**. `Num2Word_TH` subclasses `Num2Word_Base` and
//! *does* define `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `__init__` builds `self.cards` and sets `MAXVAL` — but every one of those
//! is dead code, because TH overrides `to_cardinal` outright and drives its
//! own `left_num_to_text` → `split_six` → `splitnum` → `add_text_million`
//! pipeline. `cards`/`maxval`/`merge` therefore stay at their trait defaults
//! here, and there is **no overflow check** at any size (see bug 1 below).
//!
//! Inherited from `Num2Word_Base` (unchanged by TH, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`.
//!     Corpus confirms `to_ordinal_num(-1)` == "-1" (no sign handling, no
//!     ordinal suffix — Python literally returns the input unchanged and the
//!     dispatcher stringifies it).
//!   * `to_year(value) -> self.to_cardinal(value)` → default delegates through
//!     `&self`, picking up the `to_cardinal` override below. There is no BC/AD
//!     handling whatsoever: `to_year(-500)` == "ติดลบห้าร้อย" ("minus five
//!     hundred"), verified against the interpreter.
//!
//! # Thai numeral system, as this module models it
//!
//! Thai groups by **six** digits (ล้าน = 10^6), not three, and stacks the
//! ล้าน marker for higher powers: 10^12 is "million million"
//! (หนึ่งล้านล้าน), 10^18 is "million million million". `split_six` chops the
//! digit string into 6-digit groups, and `add_text_million` rejoins them with
//! a literal "ล้าน" between each. This scales indefinitely, which is why the
//! absent overflow check never actually bites.
//!
//! Two morphological rules live in `splitnum`:
//!   * **เอ็ด** — a trailing 1 in a multi-digit group is "et", not "one"
//!     (11 → สิบเอ็ด, 21 → ยี่สิบเอ็ด, 101 → หนึ่งร้อยเอ็ด).
//!   * **ยี่** — 2 in the tens place is "yi", not "song" (20 → ยี่สิบ).
//!   * A bare 1 in the tens place gets no unit word at all (10 → สิบ, not
//!     "หนึ่งสิบ"), via the `index != 1 or num != 1` guard.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the interpreter and preserved verbatim:
//!
//! 1. **`cards` is garbage and `MAXVAL` is 9000.** TH's `low_numwords` is
//!    listed *ascending* (ศูนย์=0 … เก้า=9), but `Num2Word_Base.set_low_numwords`
//!    assumes *descending* and zips against `range(len-1, -1, -1)`. The
//!    resulting table maps `cards[9] = "ศูนย์"` ("zero"), `cards[8] = "หนึ่ง"`
//!    ("one"), … `cards[0] = "เก้า"` ("nine") — every entry inverted. `MAXVAL`
//!    is then `1000 * 9 == 9000`. None of this is ever read, because
//!    `to_cardinal` is overridden and never calls `splitnum`(base)/`clean`/
//!    `merge`. Reproduced here only by *omission*: we leave `cards`/`maxval`
//!    at their trait defaults rather than modelling a table nothing consults.
//!    The consequence that *is* observable: TH never raises `OverflowError`,
//!    at any magnitude (corpus goes to 10^21 and returns fine). Fixed
//!    (gladiaio/num2words2#159): `maxval("th")` used to report that inert
//!    9000 although 10^12 converts fine; it is now None.
//!
//! 2. **Three hardcoded string special-cases in `left_num_to_text`**, compared
//!    against the *decimal string*, not the value:
//!      * `"100"` → "ร้อย"   (the pipeline would emit "หนึ่งร้อย")
//!      * `"100000000"` → "ร้อยล้าน"   (pipeline: "หนึ่งร้อยล้าน")
//!      * `"1000000000"` → "พันล้าน"   (pipeline: "หนึ่งพันล้าน")
//!    These drop the leading "หนึ่ง" ("one") for exactly three magnitudes and
//!    nothing else, which is inconsistent: 1000 stays "หนึ่งพัน", 10^6 stays
//!    "หนึ่งล้าน", 10^12 stays "หนึ่งล้านล้าน". Colloquial Thai does drop the
//!    "one" here, so it is arguably a partial fix rather than a bug — but it
//!    is applied to a string, so it fires *after* the minus sign is stripped:
//!    `to_cardinal(-100)` == "ติดลบร้อย" (corpus-confirmed), and it fires only
//!    on the whole number, never on an internal group (1000100 is unaffected).
//!    Modelled by [`left_num_to_text`].
//!
//! 3. **`length` is a `bool` compared against `0`.** `splitnum` does
//!    `length = len(six_num) > 1` and later tests `length == 0`, which in
//!    Python is `False == 0` → `True`. It means "this group is a single
//!    digit". Ported as `!length`.
//!
//! 4. **`word_num += "เอ็ด"` appends where every sibling branch prepends.**
//!    Harmless in practice: the branch is guarded by `index == 0`, and at
//!    index 0 the `if index:` prefix never ran, so `word_num` is always empty
//!    there. Ported literally regardless.
//!
//! 5. **Zero-suppression asymmetry.** A group of all zeros yields `""`, and
//!    `add_text_million` still welds "ล้าน" onto it — which is exactly how
//!    10^6 becomes "หนึ่งล้าน" (`"หนึ่ง" + "ล้าน" + ""`). Only a *single-digit*
//!    group of "0" produces the word ศูนย์, so the string "0" is the sole way
//!    to ever see it in an integer result.
//!
//! # The currency surface
//!
//! `Num2Word_TH` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base`'s version — no `pluralize`, no `has_decimal`
//! guard, no `CURRENCY_PRECISION`, no adjective support, and its own
//! `NotImplementedError` wording. See [`LangTh::to_currency`] for the
//! divergences, each of which is corpus- or interpreter-confirmed.
//!
//! `to_cheque` is **not** overridden: TH inherits `Num2Word_Base.to_cheque`
//! verbatim, so the trait default (`currency::default_to_cheque`) is left in
//! place and only `lang_name` + `currency_forms` are supplied for it to read.
//! That inheritance is why the two error messages differ — see
//! [`LangTh::to_currency`].
//!
//! `round_2_decimal` is **dead library code**: `lang_TH.py` defines it, but no
//! conversion path calls it (`to_currency` inlines its own rounding via
//! `parse_currency_parts`). Only `tests/lang/test_th.py` pokes it directly, and
//! it is unreachable from every `to_*` entry point the dispatcher exposes, so
//! it is deliberately not ported.
//!
//! # The float / Decimal cardinal path
//!
//! `Num2Word_TH.to_cardinal` is a single method covering *both* integer and
//! non-integer input. The integer half (above) leaves `post == "0"` and skips
//! the "จุด" branch — provably dead for `int` input, since `float2tuple` on an
//! `int` returns `post == 0` with `self.precision = 0`. Float and `Decimal`
//! input open that branch instead.
//!
//! The trait splits the one Python method in two: [`to_cardinal`] takes the
//! `BigInt` path, and the trait hook [`LangTh::to_cardinal_float`] takes the
//! `FloatValue` path. Both run the same `left_num_to_text` pipeline and the
//! same "จุด" + per-digit `low_numwords` rendering, shared through
//! [`cardinal_float_from_value`]. TH never delegates to
//! `Num2Word_Base.to_cardinal_float`, which is why the byte shape differs from
//! the base's — no spaces, no titled pointword ("หนึ่งจุดหนึ่ง", not
//! "หนึ่ง จุด หนึ่ง") — and why the trait's `cardinal_from_decimal` default is
//! left unused.
//!
//! The *currency* fractional-cents branch reaches the same rendering by a
//! separate door — Python's `self.to_cardinal(float(right))` — ported as the
//! f64-only [`to_cardinal_float`] free helper that [`LangTh::to_currency`]
//! calls. It is kept distinct from the `FloatValue` core only because the
//! subunits it sees are always in `[0, 100)`, so it can safely recompute
//! precision from `repr` (see [`cardinal_float_from_value`] for why the general
//! path cannot).
//!
//! # Cross-call mutable state
//!
//! `float2tuple` writes `self.precision` (to 0 for integer input) and
//! `to_cardinal` reads it back on the next line. It is a genuine
//! write-then-read handshake on instance state, but it is confined to a single
//! call and, for integer input, the value it computes is never used — `post`
//! is `"0"` and the padding branch `len(post) < precision` is `1 < 0`, i.e.
//! false. No state survives a call, so the stateless Rust path is safe here.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.negword`, set in `setup`. Used as a bare prefix with **no space** —
/// TH concatenates it directly (`"ติดลบ" + result`) rather than going through
/// the base's `"%s " % negword.strip()` spacing.
///
/// `to_currency` is the one exception: it builds `self.negword + " "`, so the
/// currency path *does* get a separating space. See [`LangTh::to_currency`].
const NEGWORD: &str = "ติดลบ";

/// `self.pointword`, set in `setup`. Concatenated with no surrounding spaces,
/// unlike the base's `" ".join` float path.
const POINTWORD: &str = "จุด";

/// `self.mid_numwords`. Indexed by digit position within a 6-digit group, so
/// only 0..=5 are ever reachable from `splitnum` (a group is at most 6 chars).
/// Index 6 ("ล้าน") is dead here — `add_text_million` hardcodes that literal
/// instead of reading this table.
const MID_NUMWORDS: [&str; 7] = ["", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน"];

/// `self.low_numwords`, ascending 0..=9 (note: the base class expects this
/// *descending* — see bug 1 in the module docs).
const LOW_NUMWORDS: [&str; 10] = [
    "ศูนย์", "หนึ่ง", "สอง", "สาม", "สี่", "ห้า", "หก", "เจ็ด", "แปด", "เก้า",
];

/// "et" — replaces "one" in the units slot of a multi-digit group.
const ET: &str = "เอ็ด";
/// "yi" — replaces "song" ("two") in the tens slot.
const YI: &str = "ยี่";
/// "million" (10^6), the group separator stacked by [`add_text_million`].
const MILLION: &str = "ล้าน";

/// `self.CURRENCY_FORMS`, assigned in `setup()`.
///
/// TH **rebinds** the attribute (`self.CURRENCY_FORMS = {...}`) rather than
/// mutating it, and subclasses `Num2Word_Base` directly rather than
/// `Num2Word_EUR`. Both facts matter: the shared-class-dict mutation that
/// `Num2Word_EN.__init__` performs on `Num2Word_EUR.CURRENCY_FORMS` — the trap
/// that hands Swedish and Hungarian English plurals — cannot reach TH. Three
/// codes is genuinely all there is; verified against the live interpreter.
///
/// Both forms of every pair are identical because Thai does not inflect for
/// number. The arity is kept at 2 anyway to match the Python tuples: TH's
/// `to_currency` only ever reads index `[0]`, but `Num2Word_Base.to_cheque` —
/// which TH inherits — reads `cr1[-1]`, so a 1-tuple would not be equivalent.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("THB", CurrencyForms::new(&["บาท", "บาท"], &["สตางค์", "สตางค์"]));
    m.insert(
        "USD",
        CurrencyForms::new(&["ดอลลาร์", "ดอลลาร์"], &["เซนต์", "เซนต์"]),
    );
    m.insert("EUR", CurrencyForms::new(&["ยูโร", "ยูโร"], &["เซนต์", "เซนต์"]));
    m
}

pub struct LangTh {
    /// Built once in `new()`. `to_currency` and the inherited `to_cheque` only
    /// ever read it; rebuilding it per call is what made an earlier revision of
    /// this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangTh {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTh {
    pub fn new() -> Self {
        LangTh {
            currency_forms: build_currency_forms(),
        }
    }
}

/// Port of `utils.splitbyx(n, x, format_int=False)`.
///
/// Yields left-to-right chunks of `x` chars, with any short remainder first.
/// `format_int=False` means chunks stay strings (leading zeros preserved),
/// which the rest of the pipeline depends on — `splitnum` reads digit
/// *positions* out of them.
///
/// Python's `range(start, length, x)` with `start = length % x` guarantees
/// every subsequent chunk is exactly `x` long, so no bounds check is needed.
fn splitbyx(n: &[char], x: usize) -> Vec<Vec<char>> {
    let length = n.len();
    let mut out: Vec<Vec<char>> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(n[..start].to_vec());
        }
        let mut i = start;
        while i < length {
            out.push(n[i..i + x].to_vec());
            i += x;
        }
    } else {
        out.push(n.to_vec());
    }
    out
}

/// Port of `Num2Word_TH.split_six`.
///
/// Splits into 6-digit groups, reverses the *group order* (least-significant
/// group first), then reverses the *digits within* each group (least-
/// significant digit first). The double reversal is what lets `splitnum`
/// use the loop index directly as a power-of-ten position.
fn split_six(num_txt: &[char]) -> Vec<Vec<char>> {
    let mut result = splitbyx(num_txt, 6);
    result.reverse();
    result
        .into_iter()
        .map(|mut group| {
            group.reverse();
            group
        })
        .collect()
}

/// Port of `Num2Word_TH.splitnum`.
///
/// `six_num` is a digit group **already reversed** by [`split_six`], so
/// `index` is the power of ten and `MID_NUMWORDS[index]` is its unit word.
///
/// The branch order is load-bearing and reproduced exactly:
///   1. `length && num == 1 && index == 0` → เอ็ด (trailing "one" → "et")
///   2. `index == 1 && num == 2`           → ยี่ prefix ("twenty", not "two-ten")
///   3. `index != 1 || num != 1`           → the ordinary digit word
///
/// Falling through all three (i.e. `index == 1 && num == 1`) deliberately
/// emits *no* digit word, leaving a bare "สิบ" — that is how 10 → "สิบ".
fn splitnum(six_num: &[char]) -> String {
    // Python: `length = len(six_num) > 1` — a bool, later compared `== 0`.
    let length = six_num.len() > 1;
    let mut word_num = String::new();

    for (index, ch) in six_num.iter().enumerate() {
        // Python's `map(int, six_num)`. Callers only ever pass decimal digit
        // strings (built from BigInt::to_string), so this cannot fail.
        let num = ch.to_digit(10).expect("split_six only yields decimal digits") as usize;

        if num != 0 {
            if index != 0 {
                word_num = format!("{}{}", MID_NUMWORDS[index], word_num);
            }

            if length && num == 1 && index == 0 {
                // Python appends here while every other branch prepends; see
                // bug 4. Equivalent because word_num is empty at index 0.
                word_num.push_str(ET);
            } else if index == 1 && num == 2 {
                word_num = format!("{}{}", YI, word_num);
            } else if index != 1 || num != 1 {
                word_num = format!("{}{}", LOW_NUMWORDS[num], word_num);
            }
        } else if index == 0 && !length {
            // Python: `elif num == 0 and index == 0 and length == 0`.
            // `length == 0` is `False == 0` → True, i.e. "single-digit group".
            // The only integer that reaches this is 0 itself.
            word_num = LOW_NUMWORDS[0].to_string();
        }
    }

    word_num
}

/// Port of `Num2Word_TH.add_text_million`.
///
/// Welds the per-group words back together with "ล้าน" between each, walking
/// the (already least-significant-first) list in reverse so the output reads
/// most-significant-first. An empty group contributes an empty string but
/// still gets its separator — that is how 10^6 → "หนึ่ง" + "ล้าน" + "".
fn add_text_million(word_num: &[String]) -> String {
    let mut result = String::new();
    for (index, t) in word_num.iter().rev().enumerate() {
        if index == 0 {
            result = t.clone();
        } else {
            result = format!("{}{}{}", result, MILLION, t);
        }
    }
    result
}

/// Port of `Num2Word_TH.left_num_to_text`.
///
/// `number` is the *decimal string* of the (already sign-stripped) integer.
/// The three special cases compare against that string — see bug 2.
fn left_num_to_text(number: &str) -> String {
    // Special case for exactly 100
    if number == "100" {
        return "ร้อย".to_string();
    }

    // Special cases for 100 million and 1 billion
    if number == "100000000" {
        return "ร้อยล้าน".to_string();
    }
    if number == "1000000000" {
        return "พันล้าน".to_string();
    }

    let chars: Vec<char> = number.chars().collect();
    let left_num_list = split_six(&chars);

    let left_text_list: Vec<String> = left_num_list.iter().map(|g| splitnum(g)).collect();

    add_text_million(&left_text_list)
}

/// Port of `Num2Word_TH.to_cardinal`, integer path only.
///
/// Python does:
/// ```text
/// negative = number < 0
/// pre, post = self.float2tuple(number)   # int input -> (number, 0), precision := 0
/// pre = "{}".format(pre)                 # e.g. "-100"
/// post = "{}".format(post)               # always "0" for ints
/// if negative: pre = pre.lstrip("-")     # -> "100"
/// result = self.left_num_to_text(pre)
/// if not post == "0": ...                # dead for ints
/// if negative: result = "ติดลบ" + result
/// ```
/// So for integers this reduces to: stringify, strip the sign, run the
/// pipeline, re-attach the sign word with **no separating space**.
fn to_cardinal(value: &BigInt) -> String {
    let negative = value.is_negative();

    // `pre = "{}".format(int(value))`, then `pre.lstrip("-")` when negative.
    // BigInt::to_string emits at most one leading '-', so trim_start_matches
    // matches lstrip exactly. Note this runs *before* left_num_to_text, which
    // is why the "100" special-case fires for -100 → "ติดลบร้อย".
    let pre = value.to_string();
    let pre = if negative {
        pre.trim_start_matches('-')
    } else {
        pre.as_str()
    };

    let result = left_num_to_text(pre);

    if negative {
        format!("{}{}", NEGWORD, result)
    } else {
        result
    }
}

/// `abs(Decimal(str(f)).as_tuple().exponent)` — the `self.precision` that
/// `float2tuple` assigns as a side effect and `to_cardinal` reads back.
///
/// Mirrors `floatpath::float_repr_precision`, which is private to that module.
/// Rust's `{}` for f64 is shortest-round-trip, the same contract as Python's
/// `repr`, so counting digits after the point agrees. The two disagree only in
/// exponent form (`str(1e21)` is "1e+21" → precision 21, while Rust prints all
/// 22 digits → precision 0), which is unreachable here: this is only ever fed
/// subunits in `[0, 100)`.
fn repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Port of `Num2Word_TH.to_cardinal`, **float path** — the other half of the
/// method [`to_cardinal`] ports.
///
/// Reached only from the fractional-cents branch of [`LangTh::to_currency`],
/// as Python's `self.to_cardinal(float(right))`. That dispatches to *TH's* own
/// `to_cardinal`, which handles floats itself and never delegates to
/// `Num2Word_Base.to_cardinal_float`. The trait's `cardinal_from_decimal` hook
/// is therefore unusable here: its default routes through the base's float
/// path, which joins words with spaces and titles the pointword —
/// "หนึ่ง จุด หนึ่ง" where TH says "หนึ่งจุดหนึ่ง". So this is a local helper
/// and `cardinal_from_decimal` is left at its (for TH, dead) default.
///
/// Python:
/// ```text
/// negative = number < 0
/// pre, post = self.float2tuple(number)   # sets self.precision
/// precision = self.precision
/// pre = "{}".format(pre); post = "{}".format(post)
/// if negative: pre = pre.lstrip("-")
/// if len(post) < precision: post = "0" * (precision - len(post)) + post
/// result = self.left_num_to_text(pre)
/// if not post == "0":
///     result += "จุด" + "".join(self.low_numwords[i] for i in map(int, post))
/// if negative: result = "ติดลบ" + result
/// ```
/// Note this is *not* the base's loop: TH iterates every char of `post` rather
/// than `range(self.precision)`, and pads on the left instead of the right.
///
/// The `negative` arm is dead on the only live call path —
/// `parse_currency_parts` returns `right` already absolute — but is ported
/// anyway so the helper is a faithful whole.
fn to_cardinal_float(number: f64) -> String {
    let negative = number < 0.0;

    let precision = repr_precision(number);
    let (pre, post) = float2tuple(&FloatValue::Float {
        value: number,
        precision,
    });

    let pre = pre.to_string();
    let pre = if negative {
        pre.trim_start_matches('-')
    } else {
        pre.as_str()
    };

    // `post` is non-negative (float2tuple takes abs), so this is pure padding.
    let mut post = post.to_string();
    if post.chars().count() < precision as usize {
        let add_zero = precision as usize - post.chars().count();
        post = format!("{}{}", "0".repeat(add_zero), post);
    }

    let mut result = left_num_to_text(pre);

    // Python compares against the *padded* string, so a zero fraction at
    // precision >= 2 ("00") passes this guard and renders "ศูนย์ศูนย์".
    if post != "0" {
        let mut right_text = String::new();
        for ch in post.chars() {
            let d = ch.to_digit(10).expect("float2tuple emits decimal digits") as usize;
            right_text.push_str(LOW_NUMWORDS[d]);
        }
        result = format!("{}{}{}", result, POINTWORD, right_text);
    }

    if negative {
        result = format!("{}{}", NEGWORD, result);
    }
    result
}

/// Port of `Num2Word_TH.to_cardinal`, **float/Decimal path**, driving the trait
/// hook [`LangTh::to_cardinal_float`].
///
/// This is the same Python method [`to_cardinal`] ports, taken down its
/// non-integer branch — the branch the f64-only [`to_cardinal_float`] above
/// ports for the currency caller. The two differ only in where `precision`
/// comes from:
///   * the currency helper recomputes it from `repr(f64)`, safe because it is
///     only ever handed subunits in `[0, 100)`;
///   * here it is read from the [`FloatValue`] — `abs(exponent)` for the
///     Decimal arm, the Python-`repr`-derived digit count for the Float arm.
///     That source is load-bearing: a plain f64 like `1e21` prints with no `.`
///     (Rust `repr` precision 0), yet Python's `str(1e21)` is `"1e+21"` →
///     precision 21, so `num2words(1e21, lang="th")` renders the integer then
///     "จุด" and twenty-one ศูนย์. Only the carried precision reproduces that.
///
/// Python (`number` is a `float` or a `Decimal`):
/// ```text
/// negative = number < 0
/// pre, post = self.float2tuple(number)   # sets self.precision from the value
/// precision = self.precision
/// pre = "{}".format(pre); post = "{}".format(post)
/// if negative: pre = pre.lstrip("-")
/// if len(post) < precision: post = "0" * (precision - len(post)) + post
/// result = self.left_num_to_text(pre)
/// if not post == "0":
///     result += "จุด" + "".join(self.low_numwords[i] for i in map(int, post))
/// if negative: result = "ติดลบ" + result
/// ```
/// Unlike the base's float path this iterates *every* char of `post` (not
/// `range(precision)`) and left-pads instead of right-truncating; here the two
/// coincide because `post` is exactly `precision` digits after padding.
fn cardinal_float_from_value(v: &FloatValue) -> String {
    // Python: `negative = number < 0` — a *numeric* comparison, so -0.0 is
    // NOT negative (unlike FloatValue::is_negative, which reads the sign
    // bit). float2tuple's pre for -0.0 is 0 and never prints a '-', so the
    // lstrip below stays consistent either way.
    let negative = match v {
        FloatValue::Float { value, .. } => *value < 0.0,
        FloatValue::Decimal { value, .. } => value.is_negative(),
    };
    let precision = v.precision();
    let (pre, post) = float2tuple(v);

    // `pre = "{}".format(pre)`, then `pre.lstrip("-")` when negative — the sign
    // is re-attached as a whole word at the end, exactly as in [`to_cardinal`].
    let pre = pre.to_string();
    let pre = if negative {
        pre.trim_start_matches('-')
    } else {
        pre.as_str()
    };

    // `post` is non-negative (float2tuple takes abs), so this is pure left pad.
    let mut post = post.to_string();
    if post.chars().count() < precision as usize {
        let add_zero = precision as usize - post.chars().count();
        post = format!("{}{}", "0".repeat(add_zero), post);
    }

    let mut result = left_num_to_text(pre);

    // Python compares against the *padded* string, so a zero fraction at
    // precision >= 2 (e.g. Decimal("5.00") -> post "00") passes this guard and
    // renders "ศูนย์ศูนย์"; only a genuine precision-0/whole value stays "0".
    if post != "0" {
        let mut right_text = String::new();
        for ch in post.chars() {
            let d = ch.to_digit(10).expect("float2tuple emits decimal digits") as usize;
            right_text.push_str(LOW_NUMWORDS[d]);
        }
        result = format!("{}{}{}", result, POINTWORD, right_text);
    }

    if negative {
        result = format!("{}{}", NEGWORD, result);
    }
    result
}

impl Lang for LangTh {
    // No `maxval`: the inert Python MAXVAL of 9000 is not reported (bug 1,
    // #159), so `maxval("th")` is None — Thai has no ceiling.

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "THB"
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

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(to_cardinal(value))
    }

    /// Port of `Num2Word_TH.to_cardinal`'s non-integer branch — the hook the
    /// dispatcher reaches for float and `Decimal` cardinal input. TH overrides
    /// `to_cardinal` (not `to_cardinal_float`) and inlines this itself, so the
    /// base's `default_to_cardinal_float` — which space-joins and titles the
    /// pointword — is deliberately *not* used. See [`cardinal_float_from_value`].
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is **ignored**,
    /// and that is a faithful port rather than an oversight: the dispatcher
    /// stashes the override on `self.precision`, but TH's `to_cardinal` then
    /// calls `float2tuple`, which unconditionally recomputes `self.precision`
    /// from the value and clobbers it. So `num2words(12.34, lang="th",
    /// precision=1)` still renders both decimals — verified against the
    /// interpreter. See `concerns`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        Ok(cardinal_float_from_value(value))
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry, whole values included.
    ///
    /// TH's `to_cardinal` runs `float2tuple` unconditionally and compares the
    /// *padded* post string against "0", so a whole value whose precision is
    /// >= 2 still speaks its zeros: `Decimal("5.00")` → "ห้าจุดศูนย์ศูนย์",
    /// `Decimal("1E+2")` → "ร้อยจุดศูนย์ศูนย์" (a positive exponent is a
    /// positive `abs(exponent)` precision), and `1e+16` (str-precision 16)
    /// gets จุด plus sixteen ศูนย์. Base's whole-value integer route would
    /// silently drop all of that, so the default entry is overridden.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        Ok(cardinal_float_from_value(value))
    }

    /// `to_ordinal(float/Decimal)`: `num = int(number)` — truncation toward
    /// zero — then "ที่" + cardinal(int): `2.5` → "ที่สอง", `-1.5` →
    /// "ที่ติดลบหนึ่ง", `Decimal("5.00")` → "ที่ห้า". The `except
    /// (ValueError, TypeError): return str(number)` rescue only fires for
    /// `int(nan)`; `int(inf)`'s OverflowError is not caught and propagates.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let num = match value {
            FloatValue::Float { value, .. } => {
                if value.is_nan() {
                    // int(nan) raises ValueError, which to_ordinal catches
                    // and answers with str(number).
                    return Ok("nan".to_string());
                }
                if value.is_infinite() {
                    return Err(N2WError::Overflow(
                        "cannot convert float infinity to integer".into(),
                    ));
                }
                <BigInt as num_traits::FromPrimitive>::from_f64(value.trunc())
                    .ok_or_else(|| N2WError::Value("cannot convert float to integer".into()))?
            }
            FloatValue::Decimal { value, .. } => {
                // int(Decimal) truncates toward zero.
                value.with_scale(0).as_bigint_and_exponent().0
            }
        };
        self.to_ordinal(&num)
    }

    /// `str_to_number` stays Base's `Decimal(value)`, but TH's `to_cardinal`
    /// then calls `float2tuple`, whose `int(Decimal('NaN'))` raises
    /// **decimal.InvalidOperation** (Infinity keeps the shared sentinel's
    /// OverflowError — `int(Decimal('Infinity'))`). No digit present → the
    /// dispatcher propagates it.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    /// Port of `Num2Word_TH.to_ordinal`.
    ///
    /// Python wraps `int(number)` in `try/except (ValueError, TypeError)` and
    /// falls back to `str(number)`. With a `BigInt` in hand the conversion has
    /// already succeeded, so the fallback is unreachable and the whole method
    /// is a prefix: "ที่" + cardinal.
    ///
    /// Note there is no `verify_ordinal` call, so negatives pass straight
    /// through: `to_ordinal(-1)` == "ที่ติดลบหนึ่ง" ("th-minus-one"), which the
    /// corpus confirms. Most languages raise TypeError here; TH does not.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("ที่{}", to_cardinal(value)))
    }

    // to_ordinal_num: inherited from Num2Word_Base (`return value`), which the
    // trait default renders as `value.to_string()`. Corpus-confirmed for 0, 1
    // and -1.
    //
    // to_year: inherited from Num2Word_Base (`return self.to_cardinal(value)`).
    // The trait default delegates through &self and picks up the override
    // above, including the negative handling. No BC/AD suffix exists.

    // ---- currency -------------------------------------------------------
    //
    // TH overrides `to_currency` outright but inherits `to_cheque`,
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `pluralize` from
    // `Num2Word_Base`, so only the data table, the class name and
    // `to_currency` itself appear here. Deliberately NOT overridden:
    //
    //   * `currency_precision` — TH never defines `CURRENCY_PRECISION`, so it
    //     inherits Base's empty dict and `.get(code, 100)` is always 100. The
    //     trait default already returns 100. TH's `to_currency` does not even
    //     consult it (it hardcodes 100; see below), and the inherited
    //     `to_cheque` reading 100 is exactly right.
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is likewise Base's
    //     empty dict, and TH's `to_currency` ignores its `adjective` parameter
    //     entirely. `to_currency(12.34, "THB", adjective=True)` is confirmed to
    //     be byte-identical to the `adjective=False` call.
    //   * `pluralize` — abstract in Base, and unreachable for TH: the
    //     overridden `to_currency` never calls it (Thai does not inflect for
    //     number, so it indexes `cr1[0]`/`cr2[0]` directly) and `to_cheque`
    //     takes `cr1[-1]` without consulting it. Left raising NotImplemented,
    //     which is what Python would do if anything ever reached it.
    //   * `cardinal_from_decimal` — see [`to_cardinal_float`].

    fn lang_name(&self) -> &str {
        "Num2Word_TH"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_TH.to_currency`. Shares its *shape* with
    /// `Num2Word_Base.to_currency` and almost none of its behaviour.
    ///
    /// Every divergence below is confirmed against the live interpreter:
    ///
    /// 1. **No space before the unit, and no `pluralize`.** Base emits
    ///    `"%s%s %s"` and pluralizes; TH emits `"%s%s%s"` against `cr1[0]`.
    ///    So `to_currency(100)` is "ร้อยยูโร", not "ร้อย ยูโร".
    /// 2. **`minus_str` is `self.negword + " "`, not `negword.strip() + " "`.**
    ///    Identical in practice — TH's negword carries no whitespace — but this
    ///    is the one place TH puts a space after "ติดลบ" at all. Corpus:
    ///    `-12.34` → "ติดลบ สิบสองยูโร สามสิบสี่เซนต์".
    /// 3. **The divisor is hardcoded 100**, both in `has_fractional_cents` and
    ///    in the (defaulted) `parse_currency_parts` argument. TH never reads
    ///    `CURRENCY_PRECISION`, so there is no 3-decimal or 0-decimal handling
    ///    and no `divisor == 1` pre-round. Unobservable in practice: KWD/BHD
    ///    and JPY are not in TH's table and raise before reaching any of it.
    /// 4. **No `has_decimal` guard.** Base skips the cents segment when the
    ///    input has no decimal point *and* zero cents; TH has no such test, so
    ///    every non-int renders cents. `1.0` → "หนึ่งยูโร ศูนย์เซนต์" (corpus).
    ///    The `isinstance(val, int)` early return is the only thing suppressing
    ///    cents.
    /// 5. **Zero cents come from `low_numwords[0]`, not `_cents_verbose`.** The
    ///    `right > 0` ternary short-circuits to the literal "ศูนย์", which
    ///    happens to equal `to_cardinal(0)` — so this is a distinction without
    ///    an output difference, but it is why `_cents_verbose` is dead code for
    ///    TH.
    /// 6. **`cents=False` yields `str(int(right))`, unpadded.** Base's
    ///    `_cents_terse` zero-pads to the divisor width; TH bypasses it, so
    ///    `to_currency(1.0, cents=False)` is "หนึ่งบาท 0สตางค์" — a bare "0",
    ///    not "00". Verified.
    /// 7. **The `left == 0 and right >= 50` special case.** Sub-unit amounts of
    ///    half a unit or more drop the unit entirely: `0.5` → "ห้าสิบเซนต์",
    ///    while `0.49` keeps it → "ศูนย์บาท สี่สิบเก้าสตางค์". Both corpus- or
    ///    interpreter-confirmed. The cutoff is on `right`, i.e. the subunit
    ///    count, so it is a "≥ 50 cents" rule and `minus_str` survives it
    ///    (`-0.5` USD → "ติดลบ ห้าสิบเซนต์").
    /// 8. **A different NotImplementedError message from the one `to_cheque`
    ///    raises.** TH's is `Currency "X" not implemented for "Y"`; the
    ///    inherited `Num2Word_Base.to_cheque` says `Currency code "X" not
    ///    implemented for "Y"`. The missing word "code" is not a typo on my
    ///    part — the two really do differ, because only `to_currency` is
    ///    overridden. Both verified against the interpreter.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // TH accepts `adjective` and never reads it.
        _adjective: bool,
    ) -> Result<String> {
        let separator = separator.unwrap_or(self.default_separator());

        // `is_integer_input = isinstance(val, int)` — a true int, never a float
        // that happens to be whole. This is the whole of the cents suppression.
        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // `decimal_val = Decimal(str(val))`; `(decimal_val * 100) % 1 != 0`.
        // Decimal's `%` is truncated (sign follows the dividend) rather than
        // floored, but the `!= 0` test is sign-blind, so subtracting the
        // truncation is equivalent for either sign.
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        // Python passes no `divisor`, taking currency.py's default of 100.
        let (left, right, is_negative) = parse_currency_parts(val, false, has_fractional_cents, 100);

        // Python looks the forms up *after* parse_currency_parts. Order kept,
        // though it is unobservable — parse_currency_parts cannot raise here.
        let forms = self.currency_forms(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // `self.negword + " "` — no .strip(); see divergence 2.
        let minus_str = if is_negative {
            format!("{} ", NEGWORD)
        } else {
            String::new()
        };
        let money_str = to_cardinal(&left);
        let currency_str = &cr1[0];

        if is_integer_input {
            return Ok(format!("{}{}{}", minus_str, money_str, currency_str));
        }

        // `int(right)`. Already integral unless keep_precision kept the
        // fraction, and non-negative either way (parse_currency_parts abs'd it).
        let right_int = right.with_scale(0).as_bigint_and_exponent().0;

        let cents_str = if has_fractional_cents {
            // Python guards this with `isinstance(right, Decimal) and
            // has_fractional_cents`, but `right` is a Decimal exactly when
            // keep_precision was set, and keep_precision *is* has_fractional_cents
            // — the two conjuncts are the same condition. Note this branch is
            // checked before `cents`, so `cents=False` is ignored for
            // fractional subunits.
            let f = right
                .to_f64()
                .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", right)))?;
            to_cardinal_float(f)
        } else if cents {
            if right_int.is_positive() {
                to_cardinal(&right_int)
            } else {
                LOW_NUMWORDS[0].to_string()
            }
        } else {
            // `str(int(right))` — no zero padding; see divergence 6.
            right_int.to_string()
        };

        let cents_currency = &cr2[0];

        // See divergence 7. `right` (not `right_int`) is compared, so a
        // fractional 50.5 cents also trips it.
        if left.is_zero() && right >= BigDecimal::from(50) {
            return Ok(format!("{}{}{}", minus_str, cents_str, cents_currency));
        }

        Ok(format!(
            "{}{}{}{}{}{}",
            minus_str, money_str, currency_str, separator, cents_str, cents_currency
        ))
    }

    // to_cheque: inherited from `Num2Word_Base` verbatim, so the trait default
    // (`currency::default_to_cheque`) is correct as-is. It reads `lang_name`
    // and `currency_forms` above, takes `cr1[-1]` for the unit (both TH forms
    // are identical, so the plural-unconditional convention is invisible), and
    // routes the whole part through the default `money_verbose` →
    // `to_cardinal`. `.upper()` is a no-op on Thai script, which has no case.
    // Corpus: 1234.56 EUR → "หนึ่งพันสองร้อยสามสิบสี่ AND 56/100 ยูโร".
}
