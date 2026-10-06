//! Port of `lang_FA.py` (Persian / Farsi).
//!
//! Shape: **self-contained**. `Num2Word_FA` subclasses `Num2Word_Base` but its
//! `__init__` only does `self.number = 0` — it never calls `super().__init__()`,
//! so `self.cards`, `self.MAXVAL`, `self.low_numwords` etc. are *never built*.
//! `to_cardinal` is overridden outright and drives `cardinalPos` over 3-digit
//! chunks. Consequently `cards`/`maxval`/`merge` stay at their trait defaults
//! here and there is **no overflow check** at all (see bug 1 below).
//!
//! The class attributes `errmsg_toobig = "Too large"` and `MAXNUM = 10**36`
//! are dead code — the Python source itself labels them `# Those are unused`,
//! and nothing reads them. They are deliberately not modelled.
//!
//! # Integer inputs never reach the fractional path
//!
//! `to_cardinal` calls `float2tuple(number)`, which returns
//! `(pre, post, precision)`. For an `int` input:
//!   * `pre = int(value) == value`;
//!   * `precision = abs(Decimal(str(value)).as_tuple().exponent)` — `str(int)`
//!     is always plain digits with no exponent, so `Decimal` reports
//!     `exponent == 0` and `precision == 0`;
//!   * `post = abs(value - pre) * 10**0 == 0`.
//! So `float2tuple` is always `(value, 0, 0)`, `y == 0`, and `to_cardinal`
//! returns `cardinalPos(x)` unconditionally. Verified against the interpreter
//! for 0, 1, 10^18, 10^21, 10^36 and 10^100. `fractional`, `farsiFrac`,
//! `farsiFracBig` and the "نیم" (half) special case are unreachable for
//! integer input; they belong to the float/Decimal path, ported below.
//!
//! # The float/Decimal path
//!
//! FA overrides **`to_cardinal`** (not `to_cardinal_float`) and renders the
//! fraction itself — `pointword` is `None`, so nothing here ever emits the
//! Base "(.)" separator. Python's [`Num2Word_FA.to_cardinal`] for a non-int:
//!
//! ```text
//! if number < 0: return "منفی " + self.to_cardinal(-number)
//! if number == 0: return "صفر"
//! x, y, level = self.float2tuple(number)   # level == self.precision
//! if y == 0: return self.cardinalPos(x)
//! if x == 0: return self.fractional(y, level)
//! return self.cardinalPos(x) + " و " + self.fractional(y, level)
//! ```
//!
//! Reproduced in [`fa_cardinal_float`], driving [`fractional`]. The sign is
//! stripped *before* `float2tuple`, so `float2tuple` only ever sees a
//! non-negative value — pre is `int(value)` >= 0 and post = `abs(...)`. This is
//! why the shared [`float2tuple`](crate::floatpath::float2tuple) is reusable:
//! FA's own `float2tuple` applies the round/floor heuristic to Decimal input
//! too (Base's Decimal arm does not), but for a Decimal the fractional part
//! scaled by `10**precision` is *exactly* an integer, so round == floor ==
//! int() and the two agree. The f64 artefacts (`2.675` → `674.999…8` rescued
//! to `675`) survive because `FloatValue::Float` carries the raw double.
//!
//! `precision=` is threaded through as `precision_override` but has **no
//! effect** on FA: its `float2tuple` overwrites `self.precision` from the value
//! on every call, clobbering any override the dispatcher set. Verified against
//! the interpreter — `precision=1` and `precision=4` both yield the full
//! repr-derived precision. So the override is ignored here, matching Python.
//!
//! # The "نیم" (half) quirk and `farsiFracBig` overflow
//!
//! `fractional(number, level)` returns bare "نیم" (half) whenever `number == 5`
//! — *regardless of level*. So `0.5`, `0.05`, `0.005`, `1.005` and `2.005` all
//! render a "half": `0.05` → "نیم", `2.005` → "دو و نیم". Faithful Python bug.
//!
//! `farsiFracBig` has only four entries (10^0/10^3/10^6/10^9 scale). The index
//! is `level // 3`, so `level >= 12` (a Decimal with >= 12 fractional digits)
//! raises `IndexError: list index out of range` in Python. Reproduced.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Both of the following are wrong-looking but
//! are exactly what Python emits, verified against the interpreter:
//!
//! 1. **`cardinalPos` silently truncates above 10^18.** It iterates the
//!    six-element `farsiBig` table (`""`, thousand, million, milliard,
//!    trillion, trilliard = 1000^0..1000^5), peeling one 1000-chunk per
//!    iteration, and then simply *stops* — the remaining high-order digits in
//!    `x` are discarded without any error. Since there is no `MAXVAL` check,
//!    nothing catches it. So `to_cardinal(10**18) == ""` (every one of the six
//!    chunks is zero, `res` is never assigned), and likewise `10**21` and even
//!    `10**606` return `""`. Only the low 18 digits survive:
//!    `to_cardinal(10**18 + 1) == "یک"` ("one"). The corpus records the empty
//!    string for both 10^18 and 10^21. The last faithful value is 10^18 - 1.
//!
//! 2. **`to_ordinal` raises `IndexError` on anything bug 1 empties.**
//!    `to_ordinal` does `r = self.to_cardinal(number)` then `r[-1]`, which is
//!    `IndexError: string index out of range` when `r == ""`. So
//!    `to_ordinal(10**18)` and `to_ordinal(10**21)` both crash — recorded as
//!    `IndexError` in the corpus. This is a crash, not a deliberate raise, but
//!    the exception *type* is observable, so parity means reproducing it
//!    rather than tidying it into an `OverflowError`. See [`index_error`].
//!
//! 3. ~~**`to_currency` of a negative value raises `AttributeError`.**~~
//!    Fixed (gladiaio/num2words2#157). Because `__init__` skips
//!    `super().__init__()`, the `self.negword` that `Num2Word_Base.to_currency`
//!    (and `to_fraction`) format the minus sign from was never assigned, so
//!    every negative currency amount and negative fraction died with
//!    `AttributeError: 'Num2Word_FA' object has no attribute 'negword'`. The
//!    port uses FA's own cardinal minus word, "منفی", as `negword`.
//!
//! # The "سوم" rule
//!
//! `to_ordinal` appends "م" to the cardinal, except that a cardinal ending in
//! the two characters "سه" (three) becomes "سوم" — Python
//! `if r[-1] == "ه" and r[-2] == "س": return r[:-1] + "وم"`. This is genuine
//! Persian morphology (third = سوم), not a bug, and it applies at *any*
//! magnitude: 3 → "سوم", 23 → "بیست و سوم", 103 → "صد و سوم". Note the
//! guard tests the final letter "ه" alone, so 13 ("سیزده") ends in "ه" but
//! its second-to-last letter is "د", giving the correct "سیزدهم". Python
//! short-circuits `and`, so `r[-2]` is only evaluated when `r[-1] == "ه"` —
//! the len-1 `r[-2]` IndexError is modelled below even though no cardinal in
//! the tables is a single "ه".
//!
//! Indexing is by **character**, not byte: every one of these strings is
//! Arabic-script and multi-byte in UTF-8.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `farsiOnes`. Index 0 is `""` (Python relies on this: `cardinal3` returns it
/// for 0, and `cardinalPos` only ever calls with a non-zero chunk).
const FARSI_ONES: [&str; 20] = [
    "", "یک", "دو", "سه", "چهار", "پنج", "شش", "هفت", "هشت", "نه", "ده", "یازده", "دوازده",
    "سیزده", "چهارده", "پانزده", "شانزده", "هفده", "هجده", "نوزده",
];

/// `farsiTens`. Index 1 is "ده" (ten), reached only via the `< 100` branch
/// when the units digit is 0 — 10..=19 are handled by `FARSI_ONES` first.
const FARSI_TENS: [&str; 10] = [
    "", "ده", "بیست", "سی", "چهل", "پنجاه", "شصت", "هفتاد", "هشتاد", "نود",
];

/// `farsiHundreds`.
const FARSI_HUNDREDS: [&str; 10] = [
    "", "صد", "دویست", "سیصد", "چهارصد", "پانصد", "ششصد", "هفتصد", "هشتصد", "نهصد",
];

/// `farsiBig`: the scale suffix for chunk `i`, i.e. 1000^i. Each non-empty
/// entry carries a **leading space** — that is the separator between the chunk
/// and its scale word, so `format!("{}{}", chunk, big)` needs no extra space.
///
/// Six entries is the whole ceiling of this language: see bug 1 in the module
/// docs. There is no 1000^6 entry and no error when one is needed.
const FARSI_BIG: [&str; 6] = [
    "", " هزار", " میلیون", " میلیارد", " تریلیون", " تریلیارد",
];

/// `farsiFrac`: the sub-1000 fractional scale word, indexed by `level % 3`
/// (tenths / hundredths). Index 0 is `""` and always in bounds.
const FARSI_FRAC: [&str; 3] = ["", "دهم", "صدم"];

/// `farsiFracBig`: the 1000^k fractional scale word, indexed by `level // 3`.
/// Only four entries — a Decimal with >= 12 fractional digits indexes past the
/// end and raises `IndexError` in Python (see [`fractional`]).
const FARSI_FRAC_BIG: [&str; 4] = ["", "هزارم", "میلیونیم", "میلیاردیم"];

/// `fractional`'s `number == 5` special case: bare "half".
const HALF_WORD: &str = "نیم";

/// `farsiSeperator` (Python's spelling). Note the surrounding spaces.
const SEPARATOR: &str = " و ";

/// Persian "zero".
const ZERO_WORD: &str = "صفر";

/// Persian "minus", **with** the trailing space Python bakes into the literal
/// (`"منفی " + self.to_cardinal(-number)`).
const NEGWORD: &str = "منفی ";

/// The ordinal suffix appended by `to_ordinal` / `to_ordinal_num`.
const ORDINAL_SUFFIX: &str = "م";

/// `r[:-1] + "وم"` — the replacement tail for a cardinal ending in "سه".
const SOOM_SUFFIX: &str = "وم";

/// ARABIC LETTER HEH (U+0647) — the `r[-1]` the "سوم" rule tests for.
const HEH: char = 'ه';

/// ARABIC LETTER SEEN (U+0633) — the `r[-2]` the "سوم" rule tests for.
const SEEN: char = 'س';

/// Mirrors a *crash* in lang_FA.py, not a deliberate raise: `to_ordinal`
/// indexes `r[-1]` on the empty string that `cardinalPos` returns for values
/// >= 10^18. The exception type is observable behaviour a caller may catch, so
/// parity requires reproducing it rather than tidying it into an
/// `OverflowError`.
fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

/// `Num2Word_FA.CURRENCY_FORMS`, verbatim from the class body.
///
/// FA declares its own table instead of inheriting `Num2Word_EUR`'s, so none of
/// the entries `Num2Word_EN.__init__` mutates into the shared class dict reach
/// here: EUR stays `("یورو", "یورو")`, and GBP/JPY/CHF/INR/CNY/KWD/BHD do not
/// exist at all. Any code outside these four raises NotImplementedError, which
/// is exactly what the corpus records for all seven of those.
///
/// IRR and IRT carry **empty** subunit forms `("", "")` — not an oversight to
/// tidy up. `to_currency(1.5, "IRT")` really does render "یک تومان و  پنجاه ",
/// with a trailing space where the subunit name would go; inventing a word
/// there would change output.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const CENTS: [&str; 2] = ["سنت", "سنت"];
    // The `("", "")` both Iranian entries carry for their subunit.
    const NO_SUBUNIT: [&str; 2] = ["", ""];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("IRR", CurrencyForms::new(&["ریال", "ریال"], &NO_SUBUNIT));
    // Iranian Toman — Python carries this comment too.
    m.insert("IRT", CurrencyForms::new(&["تومان", "تومان"], &NO_SUBUNIT));
    m.insert("EUR", CurrencyForms::new(&["یورو", "یورو"], &CENTS));
    m.insert("USD", CurrencyForms::new(&["دلار", "دلار"], &CENTS));
    m
}

/// Port of `Num2Word_FA.cardinal3`.
///
/// Only ever called with `0 <= number <= 999`: `cardinalPos` passes a chunk
/// taken `mod 1000`, and the recursive call passes `number % 100`. That
/// invariant is what keeps the `FARSI_HUNDREDS[x]` index (x = number / 100)
/// inside its 0..=9 bounds — Python would raise IndexError past that, but the
/// call is unreachable, so the bound is documented rather than modelled.
fn cardinal3(number: u32) -> String {
    debug_assert!(number <= 999, "cardinal3 invariant: chunk is taken mod 1000");

    if number <= 19 {
        return FARSI_ONES[number as usize].to_string();
    }
    if number < 100 {
        let (x, y) = (number / 10, number % 10);
        if y == 0 {
            return FARSI_TENS[x as usize].to_string();
        }
        return format!("{}{}{}", FARSI_TENS[x as usize], SEPARATOR, FARSI_ONES[y as usize]);
    }
    let (x, y) = (number / 100, number % 100);
    if y == 0 {
        return FARSI_HUNDREDS[x as usize].to_string();
    }
    format!("{}{}{}", FARSI_HUNDREDS[x as usize], SEPARATOR, cardinal3(y))
}

/// Port of `Num2Word_FA.cardinalPos`.
///
/// Walks `farsiBig` low chunk first, prepending each rendered chunk, so the
/// output reads high-to-low. Returns `""` for 0 and — per bug 1 — for any
/// value whose low 18 digits are all zero.
fn cardinal_pos(number: &BigInt) -> String {
    let thousand = BigInt::from(1000u32);
    let mut x = number.clone();
    let mut res = String::new();

    for big in FARSI_BIG.iter() {
        // `x` is non-negative here (to_cardinal strips the sign before
        // calling), so div_mod_floor agrees with Python's divmod.
        let (quot, rem) = x.div_mod_floor(&thousand);
        x = quot;
        let y = rem
            .to_u32()
            .expect("n mod 1000 always fits in u32");

        if y == 0 {
            continue;
        }

        // `cardinal3(y)` is non-empty because y != 0, so `yx` is never "" and
        // the `res.is_empty()` test below stays equivalent to Python's
        // `res == ""`.
        let mut yx = format!("{}{}", cardinal3(y), big);

        // Python: `if b == " هزار" and y == 1: yx = "هزار"`. One thousand is
        // bare "هزار", never "یک هزار" — and note this drops the *leading
        // space* of the suffix too. The test is on the suffix string, which
        // only matches FARSI_BIG[1]; no other scale gets this treatment, so
        // 10^6 stays "یک میلیون".
        if *big == FARSI_BIG[1] && y == 1 {
            yx = FARSI_BIG[1].trim_start().to_string();
        }

        res = if res.is_empty() {
            yx
        } else {
            format!("{}{}{}", yx, SEPARATOR, res)
        };
    }

    // Whatever is left in `x` (i.e. everything at or above 10^18) is silently
    // dropped — no error, no MAXVAL check. This is bug 1.
    res
}

/// Port of `Num2Word_FA.fractional`.
///
/// ```text
/// def fractional(self, number, level):
///     if number == 5:
///         return "نیم"
///     x = self.cardinalPos(number)
///     ld3, lm3 = divmod(level, 3)
///     ltext = (farsiFrac[lm3] + " " + farsiFracBig[ld3]).strip()
///     return x + " " + ltext
/// ```
///
/// `number` is the fractional integer (`post`), always non-negative here.
/// `level` is the precision. `farsiFrac[lm3]` is always in bounds (`lm3` is
/// `level % 3` ∈ {0,1,2}); `farsiFracBig[ld3]` is not — `ld3 = level // 3 >= 4`
/// (i.e. `level >= 12`) is Python's `IndexError`, reproduced.
fn fractional(number: &BigInt, level: u32) -> Result<String> {
    // if number == 5: return "نیم" — regardless of level. Faithful bug.
    if number == &BigInt::from(5) {
        return Ok(HALF_WORD.to_string());
    }
    let x = cardinal_pos(number);
    // ld3, lm3 = divmod(level, 3)
    let ld3 = (level / 3) as usize;
    let lm3 = (level % 3) as usize;
    let frac = FARSI_FRAC[lm3];
    let frac_big = *FARSI_FRAC_BIG
        .get(ld3)
        .ok_or_else(|| index_error("list index out of range"))?;
    // (farsiFrac[lm3] + " " + farsiFracBig[ld3]).strip()
    let ltext = format!("{} {}", frac, frac_big);
    let ltext = ltext.trim();
    // x + " " + ltext
    Ok(format!("{} {}", x, ltext))
}

/// Negate a `FloatValue`, preserving its precision — Python's `-number` in the
/// `to_cardinal` recursion. `str(-x)` has the same fractional digits as
/// `str(x)`, so the repr-derived precision is unchanged.
fn fa_negate(v: &FloatValue) -> FloatValue {
    match v {
        FloatValue::Float { value, precision } => FloatValue::Float {
            value: -*value,
            precision: *precision,
        },
        FloatValue::Decimal { value, precision } => FloatValue::Decimal {
            value: -value.clone(),
            precision: *precision,
        },
    }
}

/// Python's `number == 0`: `0.0 == 0` for a float, a zero-valued `Decimal`
/// (including `Decimal("-0.00")`) for the Decimal arm.
fn fa_is_zero(v: &FloatValue) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value == 0.0,
        FloatValue::Decimal { value, .. } => value.is_zero(),
    }
}

/// Port of `Num2Word_FA.to_cardinal` for the float/Decimal path.
///
/// The sign check precedes everything, so `float2tuple` never sees a negative
/// value. `level` is the value's own precision; `precision=` overrides are
/// clobbered by `float2tuple` in Python and so are not consulted (see module
/// docs).
fn fa_cardinal_float(v: &FloatValue) -> Result<String> {
    // if number < 0: return "منفی " + self.to_cardinal(-number)
    // `is_negative()` is a strict `< 0` (false for -0.0 / Decimal("-0.00")),
    // exactly Python's `number < 0`.
    if v.is_negative() {
        let neg = fa_negate(v);
        return Ok(format!("{}{}", NEGWORD, fa_cardinal_float(&neg)?));
    }
    // if number == 0: return "صفر"
    if fa_is_zero(v) {
        return Ok(ZERO_WORD.to_string());
    }

    // x, y, level = self.float2tuple(number)
    let (pre, post) = float2tuple(v);
    let level = v.precision();

    // if y == 0: return self.cardinalPos(x)
    if post.is_zero() {
        return Ok(cardinal_pos(&pre));
    }
    // if x == 0: return self.fractional(y, level)
    if pre.is_zero() {
        return fractional(&post, level);
    }
    // return self.cardinalPos(x) + " و " + self.fractional(y, level)
    Ok(format!(
        "{}{}{}",
        cardinal_pos(&pre),
        SEPARATOR,
        fractional(&post, level)?
    ))
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the precision FA's
/// `float2tuple` derives from `str(value)`. Mirrors the private helper in
/// `floatpath`; duplicated because that one is not `pub`. Rust's `{}` for f64
/// is shortest-round-trip, the same contract as Python's `repr`.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

pub struct LangFa {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangFa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFa {
    pub fn new() -> Self {
        // Python's __init__ only sets `self.number = 0`, which nothing in the
        // four in-scope modes ever reads — and it is that same __init__ (never
        // chaining to super()) that left `negword` undefined; see bug 3.
        //
        // CURRENCY_FORMS is immutable class data in Python, so it is built once
        // here rather than per call.
        LangFa {
            currency_forms: build_currency_forms(),
        }
    }
}

impl Lang for LangFa {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "IRT"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " و "
    }

    // cards / maxval / merge stay at their trait defaults: Python never builds
    // them for this class (see module docs).

    fn negword(&self) -> &str {
        NEGWORD
    }

    /// Port of `Num2Word_FA.to_cardinal`.
    ///
    /// The `float2tuple` call in Python collapses to `(value, 0, 0)` for every
    /// integer, so the fractional branches are dead here — see module docs.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python: `if number < 0: return "منفی " + self.to_cardinal(-number)`.
        // The sign test precedes the zero test, so -0 is impossible for BigInt
        // and 0 falls through to "صفر" as it should.
        if value.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.to_cardinal(&(-value))?));
        }
        if value.is_zero() {
            return Ok(ZERO_WORD.to_string());
        }
        Ok(cardinal_pos(value))
    }

    /// Port of `Num2Word_FA.to_ordinal`.
    ///
    /// ```text
    /// r = self.to_cardinal(number)
    /// if r[-1] == "ه" and r[-2] == "س":
    ///     return r[:-1] + "وم"
    /// return r + "م"
    /// ```
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let r = self.to_cardinal(value)?;
        let chars: Vec<char> = r.chars().collect();

        // Python evaluates `r[-1]` first: IndexError on the empty string that
        // cardinalPos returns for >= 10^18 (bug 2).
        let last = match chars.last() {
            Some(c) => *c,
            None => return Err(index_error("string index out of range")),
        };

        if last == HEH {
            // `and` short-circuits, so `r[-2]` is only reached once `r[-1]`
            // matched. Unreachable in practice — no cardinal is a lone "ه" —
            // but modelled for fidelity rather than assumed away.
            if chars.len() < 2 {
                return Err(index_error("string index out of range"));
            }
            if chars[chars.len() - 2] == SEEN {
                let head: String = chars[..chars.len() - 1].iter().collect();
                return Ok(format!("{}{}", head, SOOM_SUFFIX));
            }
        }

        Ok(format!("{}{}", r, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_FA.to_ordinal_num` (a `@staticmethod`):
    /// `str(value) + "م"`. Purely lexical — the minus sign survives verbatim,
    /// so -5 gives "-5م", and the 10^18 ceiling does not apply because
    /// `to_cardinal` is never called.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_FA.to_year`: `return self.to_cardinal(value)`.
    /// No era suffix, no two-digit-pair splitting — years are plain cardinals.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_ordinal(float/Decimal)`: FA's `to_ordinal` is
    /// `self.to_cardinal(number)` + the "م"/"سوم" suffix rule, and its
    /// `to_cardinal` handles floats/Decimals itself — so the ordinal float
    /// path is just the cardinal float path re-suffixed: `0.5` → "نیمم",
    /// `3.25` → "سه و بیست و پنج صدمم", `-1000000.0` → "منفی یک میلیونم".
    /// The `r[-1]` probe on the empty cardinal that values >= 10^18 produce
    /// raises IndexError (`1e+20`), exactly as the int path does.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let r = self.cardinal_float_entry(value, None)?;
        let chars: Vec<char> = r.chars().collect();

        let last = match chars.last() {
            Some(c) => *c,
            None => return Err(index_error("string index out of range")),
        };
        if last == HEH {
            if chars.len() < 2 {
                return Err(index_error("string index out of range"));
            }
            if chars[chars.len() - 2] == SEEN {
                let head: String = chars[..chars.len() - 1].iter().collect();
                return Ok(format!("{}{}", head, SOOM_SUFFIX));
            }
        }
        Ok(format!("{}{}", r, ORDINAL_SUFFIX))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(value) + "م"`, purely lexical —
    /// "5.0م", "-17م", "1E+2م".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`. `Decimal("NaN")`
    /// parses fine; FA's `to_cardinal` then dies on the very first comparison
    /// (`number < 0`) with `decimal.InvalidOperation`. The binding otherwise
    /// maps `ParsedNumber::NaN` to `int(NaN)`'s ValueError, so the
    /// InvalidOperation must be raised here. Infinity keeps the default
    /// routing (`int(Decimal("Infinity"))` → OverflowError inside
    /// `float2tuple`), which the binding already produces.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    /// Float/Decimal cardinal. FA overrides `to_cardinal` (not
    /// `to_cardinal_float`) in Python and renders the fraction with its own
    /// `fractional`/`farsiFrac`/"نیم" machinery, never Base's "(.)" pointword —
    /// so this must NOT fall through to `default_to_cardinal_float`. See
    /// [`fa_cardinal_float`] and the module docs.
    ///
    /// `precision_override` is ignored: FA's `float2tuple` recomputes
    /// `self.precision` from the value every call, clobbering it. Verified
    /// against the interpreter (`precision=1` and `precision=4` agree).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        fa_cardinal_float(value)
    }

    // ---- currency ----------------------------------------------------
    //
    // FA overrides only `CURRENCY_FORMS`, `pluralize` and `to_currency` (and
    // the latter merely restates Base's body with its own kwarg defaults). It
    // defines neither `CURRENCY_ADJECTIVES` nor `CURRENCY_PRECISION`, so both
    // stay at `Num2Word_Base`'s empty dicts: no adjective ever applies and the
    // divisor is 100 for *every* code, including the ones that are 3-decimal
    // (KWD/BHD) or 0-decimal (JPY) elsewhere. That is unobservable here only
    // because FA's table has none of them — they all raise NotImplementedError
    // first. `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque`
    // are inherited unchanged, so the trait defaults already cover them.
    //
    // Fractional cents (`cardinal_from_decimal`) — the KNOWN GAP the integer
    // phase deferred, now closed. Base reaches it as
    // `self.to_cardinal(float(right))`, which for FA is FA's *own* to_cardinal:
    // it renders a real Persian fraction via `fractional`/`farsiFrac`/
    // `farsiFracBig`, including the "نیم" (half) special case — NOT Base's
    // pointword. So `to_currency(1.011, "USD")` → "...یک و یک دهم سنت", not
    // "...یک (.) یک سنت". Verified against the pure-Python converter
    // (`c.to_currency(...)`, bypassing the Rust fast path). No corpus row
    // exercises this — every fa currency arg has <= 2 decimals — so it is
    // parity-improving and regression-free. See [`Self::cardinal_from_decimal`].

    fn lang_name(&self) -> &str {
        "Num2Word_FA"
    }

    /// Fractional-cents bridge. Python's currency path does
    /// `cents_str = self.to_cardinal(float(right))`, so the Decimal is
    /// float-cast (reproducing any repr rounding) and run through FA's own
    /// float cardinal — the same rendering as [`fa_cardinal_float`], with the
    /// "نیم" quirk and no pointword. The precision is derived from the cast
    /// f64's repr exactly as FA's `float2tuple` does.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        let precision = float_repr_precision(f);
        fa_cardinal_float(&FloatValue::Float { value: f, precision })
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_FA.pluralize`:
    ///
    /// ```text
    /// if isinstance(forms, tuple):
    ///     return forms[0]
    /// return forms
    /// ```
    ///
    /// Persian does not inflect a counted noun, so the first form comes back
    /// for every `n`. Ignoring `n` is the whole point, not an oversight — and
    /// both members of each pair are identical anyway, so even a Base-style
    /// `forms[0 if n == 1 else 1]` would agree on this table.
    ///
    /// The `isinstance` test can only go one way in practice: the sole caller
    /// (`Num2Word_Base.to_currency`) passes a `CURRENCY_FORMS` tuple, and
    /// `prefix_currency` — the one thing that could substitute something else —
    /// returns a tuple too and is dead here regardless, since
    /// `CURRENCY_ADJECTIVES` is empty. Python's `forms[0]` on an empty tuple
    /// would be IndexError, so the miss maps to `Index` rather than panicking.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .first()
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_FA.to_currency`, which is a pass-through:
    ///
    /// ```text
    /// def to_currency(self, val, currency="IRT", cents=True,
    ///                 separator=" و ", adjective=False):
    ///     return super(Num2Word_FA, self).to_currency(...)
    /// ```
    ///
    /// The kwarg defaults are already carried by [`Self::default_currency`] and
    /// [`Self::default_separator`], so the body is Base's verbatim. Python
    /// raised `AttributeError` for every negative amount, because Base reads
    /// the minus sign from `self.negword`, which FA never set (bug 3, fixed in
    /// #157: "منفی").
    ///
    /// Base looks the currency up — raising NotImplementedError on a miss —
    /// before anything else, on the int branch and the float branch alike.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        if self.currency_forms(currency).is_none() {
            return Err(N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            )));
        }

        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }
}
