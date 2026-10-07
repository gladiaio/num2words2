//! Port of `lang_MK.py` (Macedonian).
//!
//! Shape: **self-contained**. `Num2Word_MK` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives a recursive `_int_to_word`. Consequently
//! `cards`/`merge` stay at their trait defaults here. Python has no overflow
//! check at all — `_int_to_word` has a catch-all `else` that returns
//! `str(number)`; this port spells the scales up to 10^18 and raises
//! `OverflowError` from 10^21 (see bug 3 below).
//!
//! MK overrides all four in-scope modes, so nothing is inherited from
//! `Num2Word_Base` except the `__init__` → `setup()` call that installs the
//! word tables. In particular `to_ordinal_num` does **not** fall through to
//! the base's identity return — MK appends a period.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong, and every
//! item below is exactly what Python emits — each is pinned by a row in
//! `bench/corpus.jsonl`:
//!
//! 1. **Zero (fixed, gladiaio/num2words2#154).** Python's `_int_to_word`
//!    opens with `return self.ones[0] if self.ones[0] else "zero"`, and
//!    `ones[0]` is `""`, so Python always answered with the English "zero".
//!    This port says the Macedonian "нула" instead: `to_cardinal(0)` ==
//!    "нула" and `to_ordinal(0)` == "нула-ти" (the suffix is still glued on,
//!    see bug 6).
//! 2. **No teens table.** 11..=19 are built from the generic
//!    `tens[n // 10] + " " + ones[n % 10]` rule, so 11 == "десет еден"
//!    ("ten one") rather than the correct "единаесет". Likewise 12 ==
//!    "десет два". Real Macedonian has dedicated teen forms; this module has
//!    no table for them.
//! 3. **Numbers >= 10^9 (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` chain stops at the millions branch and the final `else`
//!    is `return str(number)`, so `to_cardinal(10**9)` was the digit string
//!    "1000000000". This port continues the same `N <scale>` composition with
//!    the Macedonian long-scale words милијарда (10^9), билион (10^12),
//!    билијарда (10^15) and трилион (10^18), and raises `OverflowError` at
//!    10^21 (`maxval`), where no word is attested.
//! 4. **`hundred`/`thousand`/`million` are never inflected or elided.** 100 ==
//!    "еден сто" ("one hundred", where Macedonian says "сто"), 200 == "два сто"
//!    (vs. "двесте"), 1000 == "еден илјада", and there is no plural agreement:
//!    9_000_000 == "девет милион", not "девет милиони".
//! 5. **`negword` keeps its trailing space.** `setup` sets
//!    `negword = "minus "` — with the space — and MK's own `to_cardinal`
//!    concatenates it *verbatim* rather than trimming it the way
//!    `Num2Word_Base.to_cardinal` does. The English word itself is replaced
//!    by the Macedonian "минус " (gladiaio/num2words2#154), and likewise
//!    `pointword` "point" by "запирка" (Macedonian writes a decimal comma),
//!    so `to_cardinal(-1)` == "минус еден" and `1.5` == "еден запирка пет".
//! 6. **The ordinal suffix is bolted onto the cardinal.**
//!    `to_ordinal` is `self.to_cardinal(number) + "-ти"`, applied blindly with
//!    no agreement and no adjustment of the final word: `to_ordinal(2)` ==
//!    "два-ти" and `to_ordinal(-1)` == "минус еден-ти". Note that MK's ordinal
//!    therefore *accepts negatives* rather than raising the way most modules
//!    do.
//!
//! # Currency
//!
//! `Num2Word_MK` declares `CURRENCY_FORMS` **on its own class** — only MKD,
//! USD and EUR — so it is one of the classes *not* affected by
//! `Num2Word_EN.__init__`'s in-place mutation of the shared
//! `Num2Word_EUR.CURRENCY_FORMS` dict. Confirmed against the live
//! interpreter: MK's table holds exactly those three codes and its EUR entry
//! is `("euro", "euros")`. It inherits `CURRENCY_ADJECTIVES = {}` and
//! `CURRENCY_PRECISION = {}` from `Num2Word_Base`, so every code has
//! precision 100 and no adjective — the trait defaults already match, and
//! neither hook is overridden here.
//!
//! MK overrides `to_currency` **outright**; none of the base's currency
//! machinery (`parse_currency_parts`, `pluralize`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse`, `CURRENCY_PRECISION`) is reached from
//! it. It does *not* override `to_cheque`, which therefore runs
//! `Num2Word_Base.to_cheque` verbatim — the trait default. That split is the
//! source of the asymmetry in bug 7 below.
//!
//! `pluralize` is deliberately left at its raising default: MK never defines
//! it and never calls it (its `to_currency` inlines a `!= 1` ternary
//! instead), so `Num2Word_Base.pluralize`'s `raise NotImplementedError`
//! remains the faithful behaviour.
//!
//! # Faithfully reproduced Python bugs, currency half
//!
//! 7. **An unknown currency code does not raise — but only in
//!    `to_currency`.** MK does
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`,
//!    silently falling back to the dict's *first inserted* entry, "MKD". So
//!    `to_currency(1, "GBP")` == "еден денар" — Macedonian denars for a
//!    request for pounds. `to_cheque("GBP")` meanwhile indexes
//!    `CURRENCY_FORMS[currency]` in the base and raises NotImplementedError.
//!    The corpus pins both halves: every `currency:{GBP,JPY,KWD,BHD,INR,CNY,
//!    CHF}` row renders денари, and every matching `cheque:` row is
//!    `NotImplementedError`. The fallback's dependence on dict order is why
//!    the table below is built from an ordered array and the fallback taken
//!    from index 0 rather than from a `HashMap` iteration.
//! 8. **Cents are truncated to two digits, never rounded, and the currency's
//!    precision is ignored.** `int(parts[1][:2].ljust(2, "0"))` slices the
//!    first two characters off the *decimal string*. So `0.999` -> 99 cents
//!    (not 100), `1234.5678` -> 56 cents, and `0.5` -> "50" -> 50 cents.
//!    Because `CURRENCY_PRECISION` is never consulted, the 3-decimal
//!    currencies (KWD/BHD) and the 0-decimal ones (JPY) get the same
//!    hard-coded /100 treatment as everything else — the corpus confirms
//!    `currency:KWD 12.34` == "...триесет четири дени", not a mils reading,
//!    and `currency:JPY 0.5` still emits a cents segment.
//! 9. **Zero cents suppress the segment entirely.** The guard is
//!    `if cents and right`, so the float `1.0` renders "еден euro" with no
//!    cents at all — where `Num2Word_Base.to_currency` would emit
//!    "... 00 cents" for a float. `cents=False` likewise just drops the
//!    segment rather than falling back to `_cents_terse`.
//! 10. **`adjective` is accepted and ignored.** MK takes the parameter and
//!    never reads it; `CURRENCY_ADJECTIVES` is empty anyway.
//! 11. **The unit/subunit words stay English (or Macedonian) per the table,
//!    but the number is always Macedonian**, and the negative marker is still
//!    the "минус " of bug 5: `currency:EUR -12.34` ==
//!    "минус десет два euros триесет четири cents".
//!
//! # Error variants
//!
//! The four integer modes raise only `OverflowError`, for `abs(n) >= 10^21`
//! (gladiaio/num2words2#147); the float and currency paths share that
//! ceiling because every one of them goes through `_int_to_word`.
//!
//! On the currency surface only `to_cheque` raises, via the inherited
//! `Num2Word_Base.to_cheque`: a code absent from `CURRENCY_FORMS` is a
//! `KeyError` that the base catches and re-raises as `NotImplementedError`
//! with the message `Currency code "X" not implemented for "Num2Word_MK"`.
//! That is `currency_forms()` returning `None` plus the trait's default
//! `to_cheque` — no override needed. `to_currency` itself has no raising
//! path for any value the shim can hand it (see the scientific-notation note
//! on `plain_decimal_string`).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use долар/долари and евро/евра with цент/центи.
//! Examples in these docs that quote English nouns record Python's output.

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

/// `setup`: `self.negword = "minus "`; this port uses the Macedonian word
/// (#154). The trailing space is load-bearing — MK's `to_cardinal`
/// concatenates it raw (see bug 5).
const NEGWORD: &str = "минус ";

/// `setup`: `self.pointword = "point"`, replaced by the Macedonian decimal
/// comma "запирка" (#154).
const POINTWORD: &str = "запирка";

/// `setup`: `self.ones`. Index 0 is `""` and is never reached as a word —
/// `_int_to_word` intercepts 0 first and returns [`ZERO_WORD`]. See bug 1.
const ONES: [&str; 10] = [
    "", "еден", "два", "три", "четири", "пет", "шест", "седум", "осум", "девет",
];

/// `setup`: `self.tens`. Index 0 is `""` and is unreachable — the `< 100`
/// branch only runs for `number >= 10`, so `tens_val` is always 1..=9.
const TENS: [&str; 10] = [
    "",
    "десет",
    "дваесет",
    "триесет",
    "четириесет",
    "педесет",
    "шеесет",
    "седумдесет",
    "осумдесет",
    "деведесет",
];

/// `setup`: `self.hundred`.
const HUNDRED: &str = "сто";
/// `setup`: `self.thousand`.
const THOUSAND: &str = "илјада";
/// `setup`: `self.million`.
const MILLION: &str = "милион";

/// The scales above `million` that Python lacks (gladiaio/num2words2#147),
/// largest first, as `(power of ten, word)`. Macedonian uses the long scale:
/// милијарда 10^9, билион 10^12, билијарда 10^15, трилион 10^18
/// (Wiktionary милијарда, билион; mk.wikipedia Трилион). Composed like
/// `million` — multiplier, space, uninflected word.
const HIGH_SCALES: [(u32, &str); 4] = [
    (18, "трилион"),
    (15, "билијарда"),
    (12, "билион"),
    (9, "милијарда"),
];

/// The exclusive ceiling: 10^21 would need a трилијарда, which the sources
/// the scale table is drawn from do not attest (#147).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(21))
}

/// What `_int_to_word` returns for 0: Macedonian "нула", where Python says
/// the English "zero" — see bug 1.
const ZERO_WORD: &str = "нула";

/// Python's `str(val)` for the `Decimal` arm of `to_currency`, rendered in
/// plain (non-scientific) notation.
///
/// `value` is expected already `abs()`-ed, mirroring the order in the Python:
/// `val = abs(val)` runs *before* `str(val)`.
///
/// `BigDecimal` round-trips the scale of the string the shim handed over
/// (`str(1.0)` -> "1.0" -> mantissa 10, scale 1), so re-rendering
/// mantissa+scale reproduces `str(val)` exactly for every value whose Python
/// `repr` is plain decimal. Preserving that scale is what makes `1.0` keep a
/// `"0"` fraction — and therefore parse to `right == 0` and suppress the cents
/// segment (bug 9) — instead of collapsing to the int-like "1".
///
/// **Known divergence.** `repr(float)` switches to scientific notation at
/// `abs(v) >= 1e16` or `0 < abs(v) < 1e-4`, and Python then feeds e.g.
/// "1e-05" to `int()`, raising `ValueError`. The parse to `BigDecimal` has
/// already happened by the time this port sees the value and "1e-05" and
/// "0.00001" are indistinguishable afterwards, so this renders the plain form
/// and returns a string where Python raises. No corpus row (in any language)
/// uses a scientific-notation argument.
fn plain_decimal_string(value: &BigDecimal) -> String {
    let (mantissa, scale) = value.as_bigint_and_exponent();
    // BigInt::to_string() emits ASCII digits only, so char and byte counts
    // agree here; the splits below still go through chars() per the contract.
    let digits = mantissa.abs().to_string();

    if scale <= 0 {
        // Integral: value == mantissa * 10^(-scale). Rendered without a "."
        // just as `str(Decimal("2"))` == "2", which leaves `parts` length 1
        // and so `right == 0`.
        let mut s = digits;
        for _ in 0..(-scale) {
            s.push('0');
        }
        return s;
    }

    let scale = scale as usize;
    let n = digits.chars().count();
    if n <= scale {
        // 0.01 -> mantissa 1, scale 2 -> "0." + "0" + "1".
        format!("0.{}{}", "0".repeat(scale - n), digits)
    } else {
        let int_part: String = digits.chars().take(n - scale).collect();
        let frac_part: String = digits.chars().skip(n - scale).collect();
        format!("{}.{}", int_part, frac_part)
    }
}

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. `Num2Word_MK.to_cardinal` is driven
/// entirely by this string — `str(1e16) == "1e+16"` has no `"."`, so `int()`
/// raises `ValueError` there. Mirrors `lang_sk.rs`'s `python_repr_f64`.
fn python_repr_f64(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // `is_sign_negative` captures -0.0, whose repr is '-0.0'.
    let neg = f.is_sign_negative();
    let s = format!("{:e}", f.abs()); // e.g. "1.234e1", "5e-1", "0e0"
    let (mantissa, exp_s) = match s.split_once('e') {
        Some(parts) => parts,
        None => return s, // unreachable for finite f64
    };
    let exp: i32 = exp_s.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    let ndigits = digits.len() as i32;
    let decpt = exp + 1;

    let body = if decpt <= -4 || decpt > 16 {
        let mut m = String::new();
        m.push_str(&digits[..1]);
        if ndigits > 1 {
            m.push('.');
            m.push_str(&digits[1..]);
        }
        let e = decpt - 1;
        let (esign, eabs) = if e < 0 { ('-', -e) } else { ('+', e) };
        format!("{}e{}{:02}", m, esign, eabs)
    } else if decpt <= 0 {
        format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndigits {
        format!("{}{}.0", digits, "0".repeat((decpt - ndigits) as usize))
    } else {
        let dp = decpt as usize;
        format!("{}.{}", &digits[..dp], &digits[dp..])
    };

    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// CPython's `str(Decimal)`: trailing zeros preserved (`"1.10"` stays
/// `"1.10"`), `E±n` notation exactly when `exp > 0` or the adjusted exponent
/// `< -6` — so `Decimal("1E+3")` prints `"1E+3"` and feeds `int()` a
/// `ValueError`. Mirrors `lang_sk.rs`'s `python_str_decimal`.
fn python_str_decimal(bd: &BigDecimal) -> String {
    let (coeff, scale) = bd.as_bigint_and_exponent();
    let py_exp: i64 = -scale; // Decimal._exp
    let neg = coeff.is_negative(); // BigInt drops the sign of a negative zero
    let int_str = coeff.abs().to_string(); // Decimal._int
    let ndigits = int_str.len() as i64;
    let leftdigits = py_exp + ndigits;

    // dotplace, non-engineering branch of _pydecimal.__str__.
    let dotplace: i64 = if py_exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_str),
        )
    } else if dotplace >= ndigits {
        (
            format!("{}{}", int_str, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let dp = dotplace as usize;
        (int_str[..dp].to_string(), format!(".{}", &int_str[dp..]))
    };

    let exp = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — a sign but no zero-padding, unlike float repr.
        format!("E{:+}", leftdigits - dotplace)
    };

    let sign = if neg { "-" } else { "" };
    format!("{}{}{}{}", sign, intpart, fracpart, exp)
}

/// Python's `int(s)` on a fragment of `str(number)`: only an optionally
/// signed ASCII digit run parses; anything with an `e`/`E` (exponent form,
/// "Infinity") raises `ValueError` with the literal quoted — exactly the
/// error `to_cardinal(1e16)` shows.
fn py_int(s: &str) -> Result<BigInt> {
    let err = || N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s));
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit()) {
        return Err(err());
    }
    let n: BigInt = body.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

pub struct LangMk {
    /// `Num2Word_MK.CURRENCY_FORMS`. Built once in `new()` and stored:
    /// rebuilding the table on every `to_currency` call is what made an
    /// earlier revision of this port an order of magnitude slower than the
    /// Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the dict's first-inserted
    /// entry (MKD), which `to_currency` falls back to for an unknown code.
    /// Held separately because a `HashMap` has no first entry; see bug 7.
    fallback_forms: CurrencyForms,
}

impl Default for LangMk {
    fn default() -> Self {
        Self::new()
    }
}

impl LangMk {
    pub fn new() -> Self {
        // `CURRENCY_FORMS`, in source (== dict insertion) order. The order is
        // load-bearing: `to_currency`'s fallback is the *first* value, so MKD
        // must stay at index 0. Arity 2 on every entry — `to_currency` indexes
        // `cr1[1]`/`cr1[0]` directly, mirroring Python's tuple subscript.
        let table = [
            ("MKD", &["денар", "денари"][..], &["дени", "дени"][..]),
            ("USD", &["долар", "долари"][..], &["цент", "центи"][..]),
            ("EUR", &["евро", "евра"][..], &["цент", "центи"][..]),
        ];

        let (_, fallback_unit, fallback_subunit) = table[0];
        let fallback_forms = CurrencyForms::new(fallback_unit, fallback_subunit);

        let currency_forms: HashMap<&'static str, CurrencyForms> = table
            .into_iter()
            .map(|(k, u, s)| (k, CurrencyForms::new(u, s)))
            .collect();

        LangMk {
            currency_forms,
            fallback_forms,
        }
    }

    /// Port of `Num2Word_MK._int_to_word`.
    ///
    /// Each branch recurses on a strictly smaller magnitude. Python's final
    /// `else` stringified everything from 10^9 (bug 3); here the
    /// [`HIGH_SCALES`] branches take over and [`maxval_ceiling`] raises
    /// `OverflowError` beyond them (gladiaio/num2words2#147).
    ///
    /// `//` and `%` are Python's floor semantics, so `div_mod_floor` is used
    /// rather than Rust's truncating `/` and `%`. Every call site below is
    /// non-negative anyway — the negative branch is intercepted first, and
    /// `to_cardinal` strips the sign before calling in — so the two agree
    /// here; `div_mod_floor` is used to keep the correspondence exact rather
    /// than because a negative can reach it.
    fn int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        // `self.ones[0] if self.ones[0] else "zero"` — ones[0] == "" is
        // falsy, so this is unconditionally the zero word. See bug 1.
        if number.is_zero() {
            return Ok(ZERO_WORD.to_string());
        }

        // Unreachable from the four in-scope modes: `to_cardinal` detaches the
        // "-" from `str(number)` before calling in, so `_int_to_word` only
        // ever sees a non-negative value. Ported anyway for fidelity — and
        // note it would emit "минус " (with its trailing space) un-stripped.
        if number.is_negative() {
            return Ok(format!("{}{}", NEGWORD, self.int_to_word(&number.abs())?));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if number < &ten {
            // Bounded by the branch guard: 1..=9.
            let d = number.to_usize().expect("number < 10 fits usize");
            return Ok(ONES[d].to_string());
        }

        if number < &hundred {
            let (tens_val, ones_val) = number.div_mod_floor(&ten);
            // Bounded by the branch guard: tens_val 1..=9, ones_val 0..=9.
            let t = tens_val.to_usize().expect("number < 100 => tens fit usize");
            let o = ones_val.to_usize().expect("mod 10 fits usize");
            if o == 0 {
                return Ok(TENS[t].to_string());
            }
            return Ok(format!("{} {}", TENS[t], ONES[o]));
        }

        if number < &thousand {
            let (hundreds_val, remainder) = number.div_mod_floor(&hundred);
            // Bounded by the branch guard: 1..=9.
            let h = hundreds_val
                .to_usize()
                .expect("number < 1000 => hundreds fit usize");
            // Note: no elision — 100 == "еден сто", not "сто". See bug 4.
            let mut result = format!("{} {}", ONES[h], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        if number < &million {
            let (thousands_val, remainder) = number.div_mod_floor(&thousand);
            let mut result = format!("{} {}", self.int_to_word(&thousands_val)?, THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        if number < &BigInt::from(1_000_000_000) {
            let (millions_val, remainder) = number.div_mod_floor(&million);
            let mut result = format!("{} {}", self.int_to_word(&millions_val)?, MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder)?);
            }
            return Ok(result);
        }

        // Python: `return str(number)  # Fallback for very large numbers` —
        // bug 3. The scales above million instead, same composition (#147);
        // the maxval check above keeps the multiplier below 1000.
        for &(exp, word) in HIGH_SCALES.iter() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let (scale_val, remainder) = number.div_mod_floor(&scale);
                let mut result = format!("{} {}", self.int_to_word(&scale_val)?, word);
                if !remainder.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&remainder)?);
                }
                return Ok(result);
            }
        }
        unreachable!("every value >= 10^9 and below maxval has a HIGH_SCALES entry")
    }
}

impl Lang for LangMk {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MKD"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn negword(&self) -> &str {
        // Reported verbatim, trailing space included: MK's to_cardinal does
        // not trim it the way Num2Word_Base.to_cardinal would.
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// Port of `Num2Word_MK.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).strip()`, detaches a leading `"-"` into
    /// `ret`, then checks for `"."`. `str(int)` never contains a `"."`, so an
    /// integer always takes the `else` branch:
    /// `(ret + self._int_to_word(int(n))).strip()`. Stripping the sign from
    /// the decimal string and re-parsing is exactly `value.abs()`, which is
    /// what this does. The float branch (`pointword`, per-digit decimals) is
    /// out of scope.
    ///
    /// The trailing `.strip()` is a no-op on the integer path — `_int_to_word`
    /// never returns a value with outer whitespace and never returns `""`
    /// (the only empty table entries, `ones[0]`/`tens[0]`, are both
    /// unreachable) — but it is kept to mirror the source.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };
        let magnitude = value.abs();
        Ok(format!("{}{}", ret, self.int_to_word(&magnitude)?)
            .trim()
            .to_string())
    }

    /// Port of `Num2Word_MK.to_ordinal`: `self.to_cardinal(number) + "-ти"`.
    ///
    /// Applied blindly to whatever the cardinal produced, so it inherits every
    /// cardinal quirk: `to_ordinal(0)` == "нула-ти", `to_ordinal(-1)` ==
    /// "минус еден-ти", `to_ordinal(10**9)` == "1000000000-ти". Unlike most
    /// modules MK never rejects negatives here. See bug 6.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-ти", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_MK.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Note MK *does* override the base's identity return. The sign is kept:
    /// `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_MK.to_year`: `self.to_cardinal(val)`.
    ///
    /// MK declares `to_year(self, val, longval=True)` and ignores `longval`
    /// entirely — there is no century-pairing ("nineteen eighty-four") logic,
    /// so 1984 renders as the plain cardinal "еден илјада девет сто
    /// осумдесет четири". Identical to the trait default, but spelled out
    /// because the Python explicitly overrides it.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Float/Decimal path of `Num2Word_MK.to_cardinal`.
    ///
    /// MK does **not** go through the base `float2tuple`/`to_cardinal_float`
    /// pipeline — it overrides `to_cardinal` outright and processes floats
    /// inline off the *string* representation:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"): n = n[1:]; ret = self.negword
    /// else: ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// ```
    ///
    /// Consequences that make this diverge from the shared float path, all
    /// pinned by `bench/corpus.jsonl`:
    ///
    /// * **No `float2tuple`, so no artefact rescue and no banker's rounding.**
    ///   The fractional digits are taken *verbatim* from `str(number)`, digit
    ///   by digit. `2.675` -> "два запирка шест седум пет" (digits 6,7,5), not a
    ///   rounded "675"; `1.005` -> "еден запирка нула нула пет". `str(float)` is
    ///   the shortest round-trip repr, reproduced here by formatting the f64 to
    ///   the repr-derived `precision`; `format!("{:.p$}", ...)` and Python's
    ///   `str` agree byte-for-byte on every corpus value, artefacts included.
    /// * **Integer-valued floats keep their `".0"`.** `str(1.0)` == "1.0", so
    ///   `1.0` -> "еден запирка нула"; formatting to `precision` (>= 1 for every
    ///   float) preserves the trailing zero that `format!("{}", 1.0_f64)` drops.
    /// * **Decimals keep their scale.** `str(Decimal("1.10"))` == "1.10", so
    ///   the trailing zero survives: `1.10` -> "еден запирка еден нула". Rendered
    ///   through `plain_decimal_string`, same as the currency arm.
    /// * **Each fractional digit routes through `_int_to_word(int(digit))`**, so
    ///   a `0` digit becomes the "нула" of bug 1 (`0.01` ->
    ///   "нула запирка нула еден").
    /// * **The `pointword` "запирка" and the "минус " negword (space and
    ///   all, bug 5) carry over** (`-0.5` -> "минус нула запирка пет").
    ///
    /// MK's `to_cardinal` takes no `precision=` argument, so `precision_override`
    /// (the base's issue-#580 kwarg) is not a parameter of the ported method and
    /// is ignored — the repr-derived precision is what Python uses.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() — repr(float) / str(Decimal), exponent forms
        // included: `str(1e16)` is "1e+16" (no "."), so the else branch feeds
        // it to int() and raises ValueError, exactly as Python.
        let n_full = match value {
            FloatValue::Float { value, .. } => python_repr_f64(*value),
            FloatValue::Decimal { value, .. } => python_str_decimal(value),
        };
        let n_full = n_full.trim();

        // if n.startswith("-"): n = n[1:]; ret = self.negword  else: ret = ""
        let (n, ret) = match n_full.strip_prefix('-') {
            Some(rest) => (rest, NEGWORD),
            None => (n_full, ""),
        };

        match n.split_once('.') {
            Some((left, right)) => {
                // left, right = n.split(".", 1); ret += _int_to_word(int(left))
                let left_int = py_int(left)?;
                let mut out = String::from(ret);
                out.push_str(&self.int_to_word(&left_int)?);
                out.push(' ');
                out.push_str(POINTWORD);
                out.push(' ');
                // for digit in right: ret += _int_to_word(int(digit)) + " "
                // `int(digit)` on a non-digit char (the 'e' of "1.5e+16")
                // raises ValueError, same as Python.
                for ch in right.chars() {
                    let mut buf = [0u8; 4];
                    let d = py_int(ch.encode_utf8(&mut buf))?;
                    out.push_str(&self.int_to_word(&d)?);
                    out.push(' ');
                }
                Ok(out.trim().to_string())
            }
            None => {
                // No "." — Python's else branch: exponent forms ("1e+16",
                // "1E+3") and integral Decimals ("5") land here; the former
                // raise through py_int, the latter word normally.
                let int = py_int(n)?;
                Ok(format!("{}{}", ret, self.int_to_word(&int)?)
                    .trim()
                    .to_string())
            }
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "пет запирка нула", `-0.0` ->
    /// "минус нула запирка нула", `Decimal("5.00")` -> "пет запирка нула нула".
    /// The base default's whole-value integer shortcut must not fire here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-ти"` — the
    /// cardinal being the string algorithm above, suffix glued on raw.
    /// Exponent forms raise ValueError before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-ти", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is
    /// the binding's Python `str(value)`, so exponent forms echo verbatim:
    /// `to_ordinal_num(1e16)` == "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: MK's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// Base's `str_to_number` parses "Infinity"/"NaN" *successfully*; MK's
    /// ValueError comes later, from `int("Infinity")` / `int("NaN")` inside
    /// `to_cardinal`. Both parse through here and are served natively by
    /// [`Lang::inf_result`] / the default [`Lang::nan_result`] (which already
    /// raises ValueError) — no Python fallback.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `Decimal('-Infinity')`. MK's `to_cardinal`
    /// stringifies and runs `int("Infinity")` → **ValueError** (not Base's
    /// OverflowError, which MK never reaches). The "-" of `-Infinity` is
    /// peeled before `int()`, so the message quotes the unsigned token.
    /// `to_ordinal` = cardinal + "-ти" raises the same; `to_ordinal_num` is
    /// `str(number) + "."` and echoes the repr.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}Infinity.",
                if negative { "-" } else { "" }
            )),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".to_string(),
            )),
        }
    }

    /// `Decimal('NaN')`. `int("NaN")` is a **ValueError** too; only
    /// `to_ordinal_num` (pure `str + "."`) succeeds. The default
    /// `nan_result` message ("cannot convert NaN to integer") is also a
    /// ValueError, but this override matches MK's exact `int()` message and
    /// serves the `ordinal_num` echo.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaN.".to_string()),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".to_string(),
            )),
        }
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for the inherited `to_cheque`'s
    /// NotImplementedError message.
    fn lang_name(&self) -> &str {
        "Num2Word_MK"
    }

    /// The raw `CURRENCY_FORMS[code]` lookup — `None` for a missing code.
    ///
    /// This is the *strict* lookup, and it is what the inherited
    /// `to_cheque` needs: the base indexes the dict and turns the `KeyError`
    /// into NotImplementedError. MK's own `to_currency` does **not** go
    /// through here for the miss case — it applies the MKD fallback instead
    /// (bug 7), so the two disagree by design for e.g. "GBP".
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_MK.to_currency`.
    ///
    /// MK replaces the base's currency pipeline wholesale — no
    /// `parse_currency_parts`, no `pluralize`, no `CURRENCY_PRECISION`. The
    /// whole thing is driven off `str(val).split(".")`:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// That string split is also how the int/float distinction survives here:
    /// MK has no `isinstance(val, int)` check, but `str(int)` never contains a
    /// `"."`, so an int lands in the one-element `parts` case and gets
    /// `right == 0` — no cents — while the float `1.0` stringifies to "1.0",
    /// keeps a fraction of `"0"`, and *also* gets `right == 0` and drops its
    /// cents (bug 9). The two arrive at the same output by different routes,
    /// so the `CurrencyValue` split still has to be honoured rather than
    /// collapsed: `1` and `1.0` agree, but `2` and `2.5` would not.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // `adjective` is declared by MK and never read — see bug 10.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // `if val < 0: is_negative = True; val = abs(val)`
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")`, on the already-abs'd value.
        let s = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => plain_decimal_string(&d.abs()),
        };
        let mut parts = s.split('.');
        let int_part = parts.next().unwrap_or("");
        // `Some` iff `str(val)` had a ".", i.e. Python's `len(parts) > 1`.
        let frac_part = parts.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if int_part.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(int_part).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates the decimal string — it never rounds (bug 8), and
        // `ljust` right-pads so "5" (from 0.5) becomes "50", not "05".
        let right = match frac_part {
            Some(frac) if !frac.is_empty() => {
                let mut two: String = frac.chars().take(2).collect();
                while two.chars().count() < 2 {
                    two.push('0');
                }
                BigInt::from_str(&two).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(...values())[0])`
        // — an unknown code silently becomes MKD (bug 7).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::one();

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let left_str = self.int_to_word(&left)?;
        let mut result = format!("{} {}", left_str, if left != one { &cr1[1] } else { &cr1[0] });

        // `if cents and right:` — zero cents suppress the segment outright,
        // and `cents=False` drops it without a `_cents_terse` fallback.
        if cents && !right.is_zero() {
            let cents_str = self.int_to_word(&right)?;
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                cents_str,
                if right != one { &cr2[1] } else { &cr2[0] }
            ));
        }

        // `result = self.negword + result` — "минус ", trailing space and all

        // (bug 5). The space is what separates it from the number.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `return result.strip()` — a no-op on every reachable path (no form
        // is empty and `_int_to_word` never returns outer whitespace), kept to
        // mirror the source.
        Ok(result.trim().to_string())
    }
}
