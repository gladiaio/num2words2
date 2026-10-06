//! Kazakh (`"kk"`, the ISO 639-1 code). `"kz"` (the country code) is kept as
//! an alias for the same converter, the way `"cz"` maps to `cs`.
//!
//! # One converter for two codes (gladiaio/num2words2#166)
//!
//! Upstream shipped two unrelated Kazakh modules: `lang_KK.py` and
//! `lang_KZ.py`. Each was broken in a different place:
//!
//! * `kk` said 0, decimals and negatives in English ("zero", "бір point бес",
//!   "minus бір"), glued its ordinal suffix on with a hyphen ("бір-інші", the
//!   kk row of #148), and printed bare digits from 10^9 up.
//! * `kz` had native words ("нөл", "бір бүтін бес", "минус бір"), real
//!   vowel-harmonic ordinals ("бірінші", "алтыншы") and scale words up to
//!   нониллион, but its currency default was EUR, which is not in its table,
//!   so `to="currency"` raised on every call that did not name a code.
//!
//! This module is the `kz` engine (a port of `lang_KZ.py`) with these changes
//! so it can serve `kk`:
//!
//! * **Currency defaults to KZT** (теңге/тиын), as `kk` did. The rest of the
//!   currency surface is `kz`'s, i.e. `Num2Word_Base.to_currency` with a ","
//!   separator ("бір теңге, елу тиын"). EUR is added to the table (еуро/цент)
//!   so `currency="EUR"`, which `kk` accepted, keeps working. Unknown codes
//!   raise `NotImplementedError` instead of `kk`'s old silent fallback to
//!   теңге.
//! * **100–199 keep `kk`'s form**: every hundred carries its multiplier, so
//!   100 == "бір жүз" and 101 == "бір жүз бір" (the `kz` module said "жүз",
//!   "жүз бір"). Which form is preferred is an open question for a native
//!   speaker; keeping `kk`'s form means `kk` callers see no change there.
//! * **`to_ordinal_num` keeps `kk`'s "1."** (`str(number) + "."`) rather than
//!   `kz`'s bare "1".
//! * **Overflow is an `OverflowError`.** The `THOUSANDS` table stops at
//!   нониллион (10^30), so 10^33 and above cannot be spelled. Python's `kz`
//!   raised `KeyError: 11` there; this port raises `OverflowError` and
//!   reports `maxval` as 10^33.
//!
//! `setup()` sets `negword = "минус"` and `pointword = "бүтін"` ("whole").
//!
//! Inherited from `Num2Word_Base` (the trait defaults do the right thing):
//!   * `to_year(value) -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below. Kazakh
//!     has no special year form: `to_year(1999)` == `to_cardinal(1999)` ==
//!     "бір мың тоғыз жүз тоқсан тоғыз".
//!
//! No cross-call mutable state.
//!
//! # Float / Decimal cardinal path (`to_cardinal_float`)
//!
//! Python overrides **`to_cardinal`**, not `to_cardinal_float`, and handles
//! non-integers inline: `n = str(number).replace(",", ".")`, and on `"." in n`
//! it splits `left.right`, spells `int(left)` and — crucially — `int(right)` as
//! a *single whole number* (not digit by digit), prefixing one "нөл" per
//! leading zero of `right`, then joins `left бүтін <zeros> <int(right)>`. A
//! leading "минус" is added for any negative. It never reads `self.precision`,
//! so the `precision=` kwarg (issue #580 → `precision_override`) is **inert**:
//! `num2words(1.5, 'kk', precision=5)` and `precision=0` both give
//! "бір бүтін бес".
//!
//! `right` (Python's fractional digit string) is reconstructed from
//! `float2tuple`'s `post`, zero-padded to `precision`. That is exact here:
//! `precision` is by construction the number of fractional digits in
//! `str(number)` (`abs(Decimal(str(f)).as_tuple().exponent)` for floats, the
//! Decimal scale otherwise), so `post` == `int(right)` and the padded string ==
//! `right`. Using `float2tuple` keeps the load-bearing f64 arithmetic and its
//! `< 0.01` rescue heuristic (e.g. `2.675` → `674.999…` → `675`) intact rather
//! than recomputing from a decimal string.
//!
//! ```text
//! to_cardinal(1.23)  "бір бүтін жиырма үш"   Base would say "бір бүтін екі үш"
//! to_cardinal(0.25)  "нөл бүтін жиырма бес"  Base would say "нөл бүтін екі бес"
//! ```
//!
//! # Fractional cents
//!
//! `base.to_currency` renders fractional cents (a subunit that is not a whole
//! number, e.g. 65.3 cents) as `self.to_cardinal(float(right))`. The trait's
//! `cardinal_from_decimal` default routes to `floatpath` (Base's
//! digit-by-digit spelling), so it disagrees with the whole-number fraction
//! above whenever the fractional cents carry two or more significant digits.
//! Left at the default.
//!
//! # Faithfully reproduced Python bugs
//!
//! `to_ordinal` picks its suffix by testing the cardinal's final character
//! against three hand-written character classes. Those classes are
//! **incomplete**, and the resulting wrong forms are kept:
//!
//! 1. `қ` (U+049B) is **missing** from the consonant class, and no vowel class
//!    matches it either, so "қырық" (40) falls through to the `else` arm and
//!    takes the front-vowel suffix: `to_ordinal(40)` == "қырықінші".
//!    Idiomatic Kazakh is "қырқыншы".
//! 2. The vowel arms append `ншы`/`нші` with no linking consonant, so
//!    `to_ordinal(20)` == "жиырманшы" (idiomatic: "жиырмасыншы").
//! 3. The back/front decision for consonant-final words inspects only the last
//!    **two** characters, so "миллиард" (last two = "рд", no back vowel) is
//!    misfiled as front: `to_ordinal(10**9)` == "бір миллиардінші"
//!    (idiomatic: "бір миллиардыншы"). "триллион" and friends escape this only
//!    because their last two characters are "он".
//! 4. `у` (U+0443) appears in no class at all, so "елу" (50) also reaches the
//!    `else` arm. Here the fallback happens to be right: "елуінші".
//! 5. There is no "one thousand" elision: `to_cardinal(1000)` == "бір мың",
//!    not "мың". Kept verbatim.
//!
//! # Error variants
//!
//! `THOUSANDS` has keys 1..=10 (up to "нониллион" == 1000^10 == 10^30), so the
//! largest representable value is 10^33 - 1. At 10^33 the top chunk index
//! becomes 11 and this port raises `OverflowError` (see above).
//!
//! Negatives are safe in every mode: `_int2word` strips the sign and recurses
//! on `abs(n)` *before* `splitbyx`/`get_digits` ever see the string, so no
//! `'-'` survives into `int()`.

use crate::base::{Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::ParsedNumber;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

const ZERO: &str = "нөл";
const NEGWORD: &str = "минус";
const TEN: &str = "он";
const HUNDRED: &str = "жүз";

/// `ONES`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0` /
/// `n3 > 1`), so the empty slot here is never read.
const ONES: [&str; 10] = [
    "", "бір", "екі", "үш", "төрт", "бес", "алты", "жеті", "сегіз", "тоғыз",
];

/// `TWENTIES`, keys 2..=9. Indices 0 and 1 are absent in Python; `n2 == 1` is
/// handled separately by `TEN`, and `n2 == 0` appends nothing.
const TWENTIES: [&str; 10] = [
    "", "", "жиырма", "отыз", "қырық", "елу", "алпыс", "жетпіс", "сексен", "тоқсан",
];

/// `THOUSANDS`, keys 1..=10. Index 0 is absent in Python (guarded by `i > 0`).
/// Index 11 and beyond do not exist — that is the 10^33 ceiling (`MAXVAL`).
const THOUSANDS: [&str; 11] = [
    "",
    "мың",
    "миллион",
    "миллиард",
    "триллион",
    "квадриллион",
    "квинтиллион",
    "секстиллион",
    "септиллион",
    "октиллион",
    "нониллион",
];

/// Vowels taking the bare `ншы` suffix: а о ұ ы е э.
const BACK_VOWELS: &str = "аоұыеэ";

/// Vowels taking the bare `нші` suffix: ә і ү ө.
const FRONT_VOWELS: &str = "әіүө";

/// The consonant class. Transcribed verbatim from Python — note that `қ`
/// (U+049B) and `у` (U+0443) are **absent**; see module bugs 1 and 4.
const CONSONANTS: &str = "бвгғджзйклмнңпрстфхһцчшщ";

/// Back vowels probed against the last two characters to choose `ыншы` over
/// `інші` after a consonant: а о ұ ы.
const LAST2_BACK: &str = "аоұы";

/// 10^33: the first value whose top 3-digit chunk has no `THOUSANDS` word.
fn maxval() -> &'static BigInt {
    static MAXVAL: OnceLock<BigInt> = OnceLock::new();
    MAXVAL.get_or_init(|| BigInt::from(10u32).pow(33))
}

/// Python's `kz` raised `KeyError` past the scale table; this port raises the
/// `OverflowError` every other language uses for "too large".
fn overflow_error(n: &BigInt) -> N2WError {
    N2WError::Overflow(format!("abs({}) must be less than {}.", n, maxval()))
}

/// The suffix-selection tail of `Num2Word_KZ.to_ordinal`, shared by the
/// integer path and the float/Decimal entry (Python has one method; its
/// `cardinal = self.to_cardinal(number)` call is virtual over the input type,
/// so the same character inspection runs on "бес" and on "бес бүтін нөл"
/// alike).
///
/// Python indexes `cardinal[-1]` unguarded. `to_cardinal` of a non-zero value
/// always yields at least one word, so the empty case is unreachable; an
/// empty string would be an IndexError in Python, so mirror that rather than
/// inventing a fallback.
fn ordinal_suffix(cardinal: String) -> Result<String> {
    let last = cardinal
        .chars()
        .next_back()
        .ok_or_else(|| N2WError::Index("string index out of range".to_string()))?;

    if BACK_VOWELS.contains(last) {
        return Ok(format!("{}ншы", cardinal));
    }
    if FRONT_VOWELS.contains(last) {
        return Ok(format!("{}нші", cardinal));
    }
    if CONSONANTS.contains(last) {
        // Python: cardinal[-2:] — the last two *characters*, or the whole
        // string if it is shorter than two.
        let mut tail: Vec<char> = cardinal.chars().rev().take(2).collect();
        tail.reverse();
        let last2: String = tail.into_iter().collect();

        // Python: any(v in cardinal[-2:] for v in "аоұы")
        if LAST2_BACK.chars().any(|v| last2.contains(v)) {
            return Ok(format!("{}ыншы", cardinal));
        }
        return Ok(format!("{}інші", cardinal));
    }

    // Default case: covers қ and у, neither of which is in any class.
    Ok(format!("{}інші", cardinal))
}

/// `int(n)`'s ValueError for the no-`"."` branch of `to_cardinal`, reached
/// when `str(number)` came out in exponent form (`1e+16`, `1E+2`), or as
/// `inf`/`nan`. The message shape matches CPython's; only the exception type
/// is corpus-checked.
fn int_value_error(literal: &str) -> N2WError {
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        literal
    ))
}

/// `str(number)` for a float that carries no visible point: exponent form for
/// finite values (`1e+16`), `inf`/`nan` otherwise. Only used to build the
/// `int()` ValueError message; the corpus checks exception *types*, so the
/// digits just need to be recognisably Python-shaped, not a full repr port.
fn float_no_point_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let s = format!("{:e}", f); // e.g. "1e16"
    match s.split_once('e') {
        Some((m, e)) if !e.starts_with('-') => format!("{}e+{:0>2}", m, e),
        Some((m, e)) => format!("{}e-{:0>2}", m, &e[1..]),
        None => s,
    }
}

/// Port of `utils.splitbyx(n, x)` with `x == 3` and `format_int=True`.
///
/// `n` is the decimal string of a **non-negative** integer here (`_int2word`
/// has already stripped any sign), so every chunk is 1..=3 digits and lies in
/// `0..=999`. That bound is what licenses `u32` rather than `BigInt`: the
/// chunk width is fixed by the slicing, not by the magnitude of the input.
/// The number of chunks is unbounded, but it is a length, so `usize` is fine.
fn splitbyx(n: &str) -> Vec<u32> {
    // ASCII digits only, so byte indexing and char indexing coincide; go via
    // chars anyway to keep the slicing honest.
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let x = 3usize;
    let parse = |i: usize, j: usize| -> u32 {
        chars[i..j.min(length)]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .expect("splitbyx operates on a non-negative decimal string")
    };

    let mut out: Vec<u32> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse(0, start));
        }
        let mut i = start;
        while i < length {
            out.push(parse(i, i + x));
            i += x;
        }
    } else {
        out.push(parse(0, length));
    }
    out
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Callers only ever pass a `splitbyx` chunk (`0..=999`), for which `"%03d"`
/// yields exactly three digits and the `[-3:]` slice is a no-op. The negative
/// hazard that breaks `lang_PL.to_ordinal` cannot arise here.
fn get_digits(n: u32) -> [usize; 3] {
    let s = format!("{:03}", n);
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).expect("format!(\"{:03}\") emits digits") as usize;
    }
    a
}

/// `Num2Word_KZ.CURRENCY_FORMS` (KZT, USD) plus EUR (еуро/цент), which the
/// old `kk` module accepted and which must keep working for `kk` callers.
///
/// KZ declares its own dict in the class body and subclasses `Num2Word_Base`
/// directly, so it never sees the `lang_EUR` table that `Num2Word_EN.__init__`
/// mutates in place, and none of EN's ~24 added codes leak in. Confirmed
/// against the live interpreter *after* import (i.e. after every converter has
/// been constructed and any in-place mutation has happened):
/// `{"KZT": ("теңге", "тиын"), "USD": ("доллар", "цент")}`. Every other code
/// (GBP, JPY, KWD, BHD, INR, CNY, CHF, ...) raises `NotImplementedError`.
///
/// # The one-element arity is the port, not a shortcut
///
/// KZ's entries are **flat `(str, str)` pairs**, not the `(unit_forms,
/// subunit_forms)` pairs-of-tuples EN uses. `base.py` unpacks them as
/// `cr1, cr2 = ("доллар", "цент")`, so `cr1` is the *string* "доллар" and `cr2`
/// the *string* "цент" — there is no plural form to index. All three readers of
/// that shape are guarded to treat a non-tuple as an opaque whole:
///
/// * `pluralize(n, cr1)` — KZ's returns `form` unchanged (see below).
/// * `cr2[1] if isinstance(cr2, tuple) and len(cr2) > 1 else cr2` — a str, so
///   the whole string.
/// * `to_cheque`: `unit = cr1[-1] if isinstance(cr1, tuple) else cr1` — a str,
///   so the whole string. Without that `isinstance` guard, `cr1[-1]` on a str
///   would yield the last *character*, "р".
///
/// Modelling each side as a **one-element** `Vec` makes all three fall out of
/// the shared code correctly: `unit.last()` and `subunit.get(1).or(first())`
/// both return the single string. Adding a second form here would be a
/// plausible-looking "fix" — Kazakh does have "доллардар" — and would silently
/// change `to_cheque` and the fractional-cents branch to pick it up. Don't.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("USD", CurrencyForms::new(&["доллар"], &["цент"]));
    m.insert("KZT", CurrencyForms::new(&["теңге"], &["тиын"]));
    m.insert("EUR", CurrencyForms::new(&["еуро"], &["цент"]));
    m
}

pub struct LangKk {
    /// Built once in `new()`. `to_currency`/`to_cheque` only ever read this,
    /// and rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangKk {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKk {
    pub fn new() -> Self {
        LangKk {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_KZ._int2word`.
    ///
    /// The `feminine` parameter exists in Python but is never read by the
    /// method body (only `_cents_verbose`, out of scope, ever passes it), so
    /// it is omitted here.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        if n.is_negative() {
            // Python: " ".join([self.negword, self._int2word(abs(n))]).
            // negword is "минус" with no padding, so this is a plain space
            // join — note it does *not* go through `negword.strip()` the way
            // `Num2Word_Base.to_cardinal` would.
            return Ok(format!("{} {}", NEGWORD, self.int2word(&n.abs())?));
        }

        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<&str> = Vec::new();
        let chunks = splitbyx(&n.to_string());
        let mut i = chunks.len();

        for x in chunks {
            i -= 1;

            if x == 0 {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                // The `kz` module skips the multiplier for one hundred
                // ("жүз бір"); `kk` always says it ("бір жүз бір"). The `kk`
                // form is kept until a native speaker settles it (#166).
                words.push(ONES[n3]);
                words.push(HUNDRED);
            }

            if n2 == 1 {
                words.push(TEN);
            } else if n2 > 1 {
                words.push(TWENTIES[n2]);
            }

            if n1 > 0 {
                words.push(ONES[n1]);
            }

            if i > 0 {
                // THOUSANDS[i]. Keys stop at 10, so i >= 11 means n >= 10^33:
                // OverflowError (Python's kz raised KeyError: 11).
                let word = THOUSANDS.get(i).copied().ok_or_else(|| overflow_error(n))?;
                words.push(word);
            }
        }

        Ok(words.join(" "))
    }
}

impl Lang for LangKk {
    /// `kk`'s default (KZT). The `kz` module defaulted to EUR, which was not
    /// in its own table, so every call without `currency=` raised (#166).
    fn default_currency(&self) -> &str {
        "KZT"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn maxval(&self) -> &BigInt {
        maxval()
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "бүтін"
    }

    /// Port of `Num2Word_KZ.to_cardinal` for integer input.
    ///
    /// Python computes `n = str(number).replace(",", ".")` and branches on
    /// `"." in n`. A Python `int` never stringifies with a `,` or `.`, so the
    /// fractional branch is unreachable here and the whole method collapses to
    /// `self._int2word(int(n))`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value)
    }

    /// `kk`'s `to_ordinal_num`: `str(number) + "."` — the sign survives
    /// ("-1."). The `kz` module returned the bare number.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, purely textual.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// Port of `Num2Word_KZ.to_ordinal`.
    ///
    /// Suffixes are chosen by inspecting the *last character* of the cardinal
    /// (and, for consonant endings, the last two). All indexing is by `char`:
    /// every letter involved is multi-byte in UTF-8, so byte offsets would be
    /// wrong. See the module docs for the four classification bugs this
    /// faithfully reproduces.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_zero() {
            return Ok(format!("{}інші", ZERO));
        }

        let cardinal = self.to_cardinal(value)?;
        ordinal_suffix(cardinal)
    }

    /// Port of `Num2Word_KZ.to_cardinal`'s `"." in n` branch (float/Decimal
    /// input). Python:
    ///
    /// ```python
    /// n = str(number).replace(",", ".")            # has "." here
    /// is_negative = n.startswith("-")
    /// abs_n = n[1:] if is_negative else n
    /// left, right = abs_n.split(".")
    /// leading_zero_count = len(right) - len(right.lstrip("0"))
    /// result = "%s %s %s" % (
    ///     self._int2word(int(left)),
    ///     self.pointword,
    ///     (ZERO + " ") * leading_zero_count + self._int2word(int(right)),
    /// )
    /// if is_negative: result = self.negword + " " + result
    /// ```
    ///
    /// `right` is Python's fractional digit string; `int(right)` is spelled as a
    /// single whole number (KZ's divergence from Base's digit-by-digit form),
    /// with one "нөл" per leading zero of `right`. Reconstructed from
    /// `float2tuple`: `precision` is exactly the length of `str(number)`'s
    /// fractional part, so `post` == `int(right)` and `post` zero-padded to
    /// `precision` == `right`. The f64 arithmetic (and its `< 0.01` rescue, e.g.
    /// `2.675` → `675`) stays inside `float2tuple`; nothing is recomputed from a
    /// decimal string.
    ///
    /// `precision_override` is ignored: KZ's Python `to_cardinal` never reads
    /// `self.precision`, so the `precision=` kwarg is inert (verified live).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        let precision = value.precision() as usize;

        // (pre, post): pre = int(number) (signed, truncated toward zero),
        // post = the fractional digits as a non-negative integer.
        let (pre, post) = float2tuple(value);

        // right = post zero-padded to `precision` digits (Python's fractional
        // string). ASCII digits only, so byte length == char count.
        let post_digits = post.to_string();
        let right = format!(
            "{}{}",
            "0".repeat(precision.saturating_sub(post_digits.len())),
            post_digits
        );
        // leading_zero_count = len(right) - len(right.lstrip("0")).
        // The final int(right) word already says one zero, so an all-zero
        // fraction gets len - 1 leading zeros: exactly the digits written
        // (gladiaio/num2words2#237; Python read 1.0 as "... zero zero").
        let leading_zero_count = (right.len() - right.trim_start_matches('0').len())
            .min(right.len().saturating_sub(1));

        // int(left): Python strips the sign from the string first, so `left` is
        // the integer part of the *absolute* value; the sign is carried
        // separately by `is_negative` below.
        let left_words = self.int2word(&pre.abs())?;

        // (ZERO + " ") * leading_zero_count + _int2word(int(right))
        let fraction = format!(
            "{}{}",
            format!("{} ", ZERO).repeat(leading_zero_count),
            self.int2word(&post)?
        );

        // "%s %s %s" % (left, pointword, fraction). pointword is used raw in
        // Python (no title()); KZ is not a title language, so this matches.
        let mut result = format!("{} {} {}", left_words, self.pointword(), fraction);

        // if is_negative: result = negword + " " + result. Python's
        // `is_negative = str(number).startswith("-")` follows the *string* sign,
        // which for a float is the IEEE sign bit — so `-0.0` (str "-0.0") is
        // negative even though `-0.0 < 0.0` is false. Match that on the raw f64
        // via `is_sign_negative`; the Decimal arm keeps its own sign. KZ uses
        // the raw negword ("минус") with no strip(), unlike Base's `negword.strip()`.
        let is_negative = match value {
            FloatValue::Float { value, .. } => value.is_sign_negative(),
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if is_negative {
            result = format!("{} {}", NEGWORD, result);
        }
        Ok(result)
    }

    /// Full `to_cardinal(float/Decimal)` routing — Python's gate is
    /// `"." in str(number)`, NOT the base default's `int(value) == value`:
    ///
    /// * a **visible point** (any finite float below 1e16, or a Decimal with
    ///   positive scale) takes the fractional branch even for whole values —
    ///   `5.0` -> "бес бүтін нөл", `Decimal("5.00")` -> "бес бүтін нөл нөл"
    ///   (one "нөл" per fractional digit written; Python added an extra one,
    ///   gladiaio/num2words2#237).
    /// * **no point** funnels the whole string into `int(n)`: plain digit
    ///   Decimals ("5", "100") reach the integer path, while exponent forms
    ///   (`str(1e16) == "1e+16"`, `str(Decimal("1E+2")) == "1E+2"`) and
    ///   inf/nan raise **ValueError**, not the base default's OverflowError.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        match value {
            // A finite float without a visible point stringifies in exponent
            // form; inf/nan stringify as "inf"/"nan". int() rejects them all.
            FloatValue::Float { value, .. } => Err(int_value_error(&float_no_point_str(*value))),
            FloatValue::Decimal { value, .. } => {
                let s = crate::strnum::python_decimal_str(value);
                match crate::strnum::python_int_parse(&s) {
                    Some(i) => self.to_cardinal(&i),
                    None => Err(int_value_error(&s)),
                }
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python:
    ///
    /// ```python
    /// if number == 0: return ZERO + "інші"          # 0.0, -0.0, Decimal("0")
    /// cardinal = self.to_cardinal(number)           # str-routed, see above
    /// ... suffix by cardinal's last character ...
    /// ```
    ///
    /// The zero test is numeric, so `-0.0` and `Decimal("0.00")` short-circuit
    /// to "нөлінші" without ever seeing the float grammar. Everything else
    /// (whole floats included) gets the fractional cardinal plus the suffix:
    /// `5.0` -> "бес бүтін нөл нөлінші". Exponent forms raise the cardinal's
    /// ValueError before any suffix logic runs.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int().is_some_and(|i| i.is_zero()) {
            return Ok(format!("{}інші", ZERO));
        }
        let cardinal = self.cardinal_float_entry(value, None)?;
        ordinal_suffix(cardinal)
    }

    // year_float_entry is deliberately left at the trait default: Python's
    // Num2Word_Base.to_year is `self.to_cardinal(value)` and KZ does not
    // override it, so the default's delegation to cardinal_float_entry —
    // which now carries KZ's own routing — is exactly the port.

    /// `converter.str_to_number` is Base's `Decimal(value)` (KZ doesn't
    /// override it), but `Decimal("Infinity")` then hits KZ's `to_cardinal`,
    /// where `str(number)` has no "." and `int("Infinity")` raises
    /// **ValueError** — not the OverflowError the binding's generic Inf arm
    /// (which models `int(Decimal("Infinity"))` in *base's* integer path)
    /// would produce. NaN already maps to ValueError there, matching
    /// `int("NaN")`, so only Inf needs intercepting.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(int_value_error(if negative {
                "-Infinity"
            } else {
                "Infinity"
            })),
            other => Ok(other),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // KZ inherits `to_currency`, `to_cheque`, `_money_verbose` and
    // `_cents_terse` from `Num2Word_Base` unchanged, so the trait defaults
    // already are the port. It defines neither `CURRENCY_ADJECTIVES` nor
    // `CURRENCY_PRECISION`, and nothing mutates Base's empty dicts in place —
    // both read back as `{}` from the live interpreter after import. So:
    //
    //   * `currency_adjective` stays at the default `None`; the `adjective=True`
    //     kwarg is inert for KZ (`currency in self.CURRENCY_ADJECTIVES` is
    //     always False), and no `prefix_currency` call ever happens.
    //   * `currency_precision` stays at the default 100, matching
    //     `CURRENCY_PRECISION.get(code, 100)` over an empty dict. KZ has no
    //     3-decimal and no 0-decimal currency, so the `divisor == 1000` and
    //     `divisor == 1` branches of `default_to_currency` are unreachable here
    //     — KWD/BHD/JPY raise NotImplementedError at the forms lookup instead,
    //     long before precision matters.
    //
    // Only the forms table, the class name, `pluralize` and `_cents_verbose`
    // are KZ's own.

    /// Feeds `'Currency code "%s" not implemented for "%s"'`.
    fn lang_name(&self) -> &str {
        "Num2Word_KK"
    }


    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_KZ.pluralize`: `def pluralize(self, n, form): return form`.
    ///
    /// Kazakh does not inflect the currency name for number, so this is the
    /// identity on `form` and ignores `n` entirely — "бір доллар", "екі доллар",
    /// "жүз доллар" all take the same word.
    ///
    /// Python's `form` is a *single string*, because KZ's `CURRENCY_FORMS`
    /// entries are flat `(str, str)` pairs (see `build_currency_forms`). The
    /// trait hands it over as the one-element slice that models it, so
    /// `concat()` reconstitutes the string exactly. Unlike `Num2Word_EUR`'s
    /// `forms[0 if n == 1 else 1]`, this cannot raise: the identity has no index
    /// to run off the end of, so there is no IndexError to reproduce and the
    /// `Result` is always `Ok`.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        Ok(forms.concat())
    }

    /// Port of `Num2Word_KZ._cents_verbose`:
    /// `return self._int2word(number, currency == "KZT")`.
    ///
    /// The `feminine` flag KZ computes here (`currency == "KZT"`) is **dead**:
    /// `_int2word` takes the parameter and never reads its body — so KZT and USD
    /// cents spell identically, and only the trailing unit word differs
    /// ("бір тиын" vs "бір цент"). Reproduced as-is rather than "simplified"
    /// away, since the argument is part of the ported signature.
    ///
    /// Behaviourally this equals the inherited default
    /// (`Num2Word_Base._cents_verbose` → `self.to_cardinal`), because KZ's
    /// `to_cardinal` of an int is exactly `_int2word` after a str round-trip
    /// that changes nothing. Overridden anyway because Python defines it: the
    /// default only coincides by way of KZ's `to_cardinal` override, and
    /// spelling it out keeps that coincidence from silently mattering.
    fn cents_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        self.int2word(number)
    }
}
