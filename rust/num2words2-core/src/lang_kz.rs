//! Port of `lang_KZ.py` (Kazakh).
//!
//! Shape: **self-contained**. `Num2Word_KZ` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int2word` over 3-digit chunks. Consequently
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` table
//! ends at 10^30; `maxval()` is 10^33 and larger values raise
//! `OverflowError` (see below).
//!
//! `setup()` sets `negword = "минус"` and `pointword = "бүтін"`.
//!
//! Inherited from `Num2Word_Base` (unchanged by KZ, so the trait defaults do
//! the right thing):
//!   * `to_ordinal_num(value) -> value`  → default `Ok(value.to_string())`
//!     (verified: corpus `ordinal_num` of `-1` is `"-1"`, of `0` is `"0"`).
//!   * `to_year(value)        -> self.to_cardinal(value)` → default delegates
//!     through `&self`, picking up the `to_cardinal` override below. Kazakh
//!     has no special year form: `to_year(1999)` == `to_cardinal(1999)` ==
//!     "бір мың тоғыз жүз тоқсан тоғыз".
//!
//! The currency surface (`CURRENCY_FORMS`, `pluralize`, `_cents_verbose`) is
//! ported below. The `"." in n` branch of `to_cardinal` — which only fires for
//! float/Decimal input — is now ported as `to_cardinal_float` (see
//! "Float / Decimal cardinal path" below). For integer input `str(number)`
//! never contains `,` or `.`, so `to_cardinal` still reduces to
//! `self._int2word(int(n))`.
//!
//! No cross-call mutable state: KZ defines no `str_to_number` override and
//! stashes no flags between methods.
//!
//! # Float / Decimal cardinal path (`to_cardinal_float`)
//!
//! Python overrides **`to_cardinal`**, not `to_cardinal_float`, and handles
//! non-integers inline: `n = str(number).replace(",", ".")`, and on `"." in n`
//! it splits `left.right`, spells `int(left)` and — crucially — `int(right)` as
//! a *single whole number* (not digit by digit), prefixing one "нөл" per
//! leading zero of `right`, then joins `left бүтін <zeros> <int(right)>`. A
//! leading "минус" is added for any negative. It never reads `self.precision`,
//! so the `precision=` kwarg (issue #580 → `precision_override`) is **inert**
//! — confirmed live: `num2words(1.5,'kz',precision=5)` and `precision=0` both
//! give "бір бүтін бес". The Rust core splits int-cardinal (`to_cardinal`,
//! `&BigInt`) from float-cardinal (`to_cardinal_float`, `&FloatValue`), so
//! KZ's `"." in n` branch is ported into the `to_cardinal_float` override.
//!
//! `right` (Python's fractional digit string) is reconstructed from
//! `float2tuple`'s `post`, zero-padded to `precision`. That is exact for KZ:
//! `precision` is by construction the number of fractional digits in
//! `str(number)` (`abs(Decimal(str(f)).as_tuple().exponent)` for floats, the
//! Decimal scale otherwise), so `post` == `int(right)` and the padded string ==
//! `right`. Using `float2tuple` keeps the load-bearing f64 arithmetic and its
//! `< 0.01` rescue heuristic (e.g. `2.675` → `674.999…` → `675`) intact rather
//! than recomputing from a decimal string.
//!
//! This differs from `Num2Word_Base.to_cardinal_float` (the trait default) in
//! two ways that make the default wrong for KZ: (1) Base spells the fraction
//! digit by digit ("отыз төрт" vs Base "үш төрт" for `.34`); (2) Base prepends
//! the negword only when `pre == 0`, KZ always prepends it for negatives.
//!
//! ```text
//! to_cardinal(1.23)  KZ: "бір бүтін жиырма үш"   Base: "бір бүтін екі үш"
//! to_cardinal(0.25)  KZ: "нөл бүтін жиырма бес"  Base: "нөл бүтін екі бес"
//! to_cardinal(1.1)   KZ: "бір бүтін бір"         Base: "бір бүтін бір"  (agree)
//! ```
//!
//! # Fractional cents still diverge (currency gap, deferred to the currency phase)
//!
//! `base.to_currency` renders fractional cents (a subunit that is not a whole
//! number, e.g. 65.3 cents) as `self.to_cardinal(float(right))` — a *virtual*
//! call, so Python lands in **KZ's** float branch (whole-number spelling),
//! which the `to_cardinal_float` override above now reproduces. The trait's
//! **`cardinal_from_decimal`** default, however, still routes to `floatpath`
//! (Base's digit-by-digit spelling), so it disagrees with KZ whenever the
//! fractional cents carry two or more significant digits. It is **left at the
//! default here**: this is the currency surface (a separate, later fixup phase),
//! and no corpus row reaches it — every KZ `currency:USD` case has at most two
//! decimals, so `right` is a whole number of cents and the branch never fires.
//! Flagged rather than patched — see the report's `concerns`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. `to_ordinal` picks its suffix by testing the
//! cardinal's final character against three hand-written character classes.
//! Those classes are **incomplete**, and the resulting wrong forms are exactly
//! what Python emits — every one below is confirmed against the frozen corpus:
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
//! largest representable value is 10^33 - 1 (11 chunks, top chunk index 10).
//! At 10^33 the top chunk index becomes 11 and Python raised `KeyError: 11`.
//! Fixed (gladiaio/num2words2#159): `maxval()` is 10^33 and `_int2word`
//! raises `OverflowError` for `abs(n) >= 10**33` before the lookup, so the
//! `KeyError` is unreachable from the public entry points.
//!
//! Unlike `lang_PL`, negatives are safe in every mode: `_int2word` strips the
//! sign and recurses on `abs(n)` *before* `splitbyx`/`get_digits` ever see the
//! string, so no `'-'` survives into `int()`.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::ParsedNumber;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::collections::HashMap;

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
/// Index 11 and beyond do not exist; the 10^33 MAXVAL check keeps them
/// unreachable.
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

fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// The suffix-selection tail of `Num2Word_KZ.to_ordinal`, shared by the
/// integer path and the float/Decimal entry (Python has one method; its
/// `cardinal = self.to_cardinal(number)` call is virtual over the input type,
/// so the same character inspection runs on "бес" and on "бес бүтін нөл нөл"
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

/// `Num2Word_KZ.CURRENCY_FORMS`, verbatim — two codes, and that is the whole
/// table.
///
/// KZ declares its own dict in the class body and subclasses `Num2Word_Base`
/// directly, so it never sees the `lang_EUR` table that `Num2Word_EN.__init__`
/// mutates in place, and none of EN's ~24 added codes leak in. Confirmed
/// against the live interpreter *after* import (i.e. after every converter has
/// been constructed and any in-place mutation has happened):
/// `{"KZT": ("теңге", "тиын"), "USD": ("доллар", "цент")}`. That is precisely
/// why the corpus expects `NotImplementedError` for EUR, GBP, JPY, KWD, BHD,
/// INR, CNY and CHF — 96 of KZ's 108 currency rows.
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
    m
}

pub struct LangKz {
    /// Built once in `new()`. `to_currency`/`to_cheque` only ever read this,
    /// and rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangKz {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKz {
    pub fn new() -> Self {
        LangKz {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_KZ._int2word`.
    ///
    /// The `feminine` parameter exists in Python but is never read by the
    /// method body (only `_cents_verbose`, out of scope, ever passes it), so
    /// it is omitted here.
    fn int2word(&self, n: &BigInt) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
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
                if n3 > 1 {
                    words.push(ONES[n3]);
                }
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
                // Python: THOUSANDS[i] — a bare dict lookup. Keys stop at 10,
                // so i >= 11 (n >= 10^33) would raise KeyError: 11 — the
                // MAXVAL check above rules that out (#159).
                let word = THOUSANDS
                    .get(i)
                    .copied()
                    .ok_or_else(|| key_error(i.to_string()))?;
                words.push(word);
            }
        }

        Ok(words.join(" "))
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangKz {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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
        ","
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
        let leading_zero_count = right.len() - right.trim_start_matches('0').len();

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
    ///   `5.0` -> "бес бүтін нөл нөл", `Decimal("5.00")` -> "бес бүтін нөл нөл
    ///   нөл" (one "нөл" per leading zero of the fractional string plus
    ///   `_int2word(0)`).
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

    /// Feeds `'Currency code "%s" not implemented for "%s"'`. Must stay exactly
    /// the Python class name: 96 of KZ's 108 corpus currency rows are that
    /// NotImplementedError.
    fn lang_name(&self) -> &str {
        "Num2Word_KZ"
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
