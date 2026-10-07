//! Port of `lang_BM.py` (Bambara).
//!
//! Shape: **self-contained**. `Num2Word_BM` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the base
//! `__init__` never builds `self.cards` and never sets `MAXVAL`. `to_cardinal`
//! is overridden outright and drives a private `_int_to_word` recursion.
//! Consequently `cards`/`merge` stay at their trait defaults here. Python has
//! **no overflow check** — see the ceiling note below, which was a silent
//! fallback; this port raises `OverflowError` from 10^12 (`maxval`).
//!
//! `setup()` only assigns constants; there is no cross-call mutable state and
//! no `str_to_number` handshake (BM inherits `Num2Word_Base.str_to_number`,
//! which is a plain `Decimal(value)`), so the stateless Rust path is safe to
//! dispatch to for all four in-scope modes — and for the float/Decimal path
//! below, which is the same overridden `to_cardinal`.
//!
//! Bambara builds numbers with the **multiplier after the base word**:
//! 900 is `kɛmɛ kɔnɔntɔn` ("hundred nine"), 9_000_000 is `miliyɔn kɔnɔntɔn`.
//! Reading that as reversed is a misreading of the language, not a bug — it is
//! exactly what Python emits.
//!
//! Inherited from `Num2Word_Base` and left alone by BM:
//!   * `is_title` is `False` (set by `__init__`, never flipped by `setup`).
//!     Irrelevant regardless: BM overrides `to_cardinal` and so never routes
//!     through `Num2Word_Base.to_cardinal`, the only caller of `self.title()`.
//!     `exclude_title = ["ni", "dɔgɔ"]` is therefore dead on *every* BM path,
//!     the float one included: BM interpolates `self.pointword` raw and never
//!     calls `self.title()`. `pointword = "ni"` is live — see the float path.
//!   * `self.precision` (2, from `__init__`) is **never read** by BM. Nothing
//!     in BM's `to_cardinal` consults it, so the `precision=` kwarg that
//!     `__init__.py` threads in by assigning `converter.precision` has no
//!     effect at all: `num2words(2.675, lang="bm", precision=1)` is still
//!     `"fila ni wɔɔrɔ wolonfila duuru"`. Hence `precision_override` is
//!     accepted and ignored in [`LangBm::to_cardinal_float`]. Verified against
//!     the live interpreter.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. Both of these look wrong and are kept:
//!
//! 1. **The 10^9 ceiling (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` handles values below 10^9 and then simply
//!    `return str(number)`, so `to_cardinal(10**9)` was `"1000000000"` — bare
//!    digits, no exception. This port adds miliyari (10^9; Bamadaba
//!    "míliyari", http://cormand.huma-num.fr/Bamadaba/lexicon/m.htm, tone
//!    marks dropped like the module's miliyɔn), composed exactly like the
//!    million branch: `to_cardinal(10**9)` == `"miliyari"`,
//!    `to_cardinal(2 * 10**9)` == `"miliyari fila"`. No 10^12 word is
//!    attested, so 10^12 raises `OverflowError` (`maxval`).
//! 2. **`to_ordinal` inherited that fallback**, so at or above 10^9 Python's
//!    collapsed onto `to_ordinal_num` (`"1000000000nan"`); since #147 it is
//!    `"miliyarinan"`.
//! 3. **The two paths spell the sign differently**: `to_cardinal`/
//!    `to_ordinal` render the sign as the *word* `dɔgɔ` (they strip "-" and
//!    prepend `negword`), while `to_ordinal_num` stringifies the input
//!    untouched and keeps the ASCII "-": `to_ordinal(-10**9)` ==
//!    `"dɔgɔ miliyarinan"` but `to_ordinal_num(-10**9)` == `"-1000000000nan"`.
//!
//! Two further behaviours are unremarkable but worth stating, since they are
//! where sibling languages tend to crash:
//!
//!   * `negword` is `"dɔgɔ "` *with a trailing space*, and `to_cardinal`
//!     `.strip()`s the concatenation. The trailing space is always absorbed by
//!     the following word, so the strip is a no-op in practice — but it is
//!     reproduced via `trim()` rather than assumed away.
//!   * Unlike `lang_PL`, BM's `to_ordinal` does **not** crash on 0 or on
//!     negatives: `to_ordinal(0)` == `"funan"`, `to_ordinal(-1)` ==
//!     `"dɔgɔ kelennan"`. No input in scope raises, so this port returns no
//!     `N2WError` variant on any path.
//!
//! # Currency
//!
//! `Num2Word_BM` overrides `to_currency` **outright** — none of
//! `Num2Word_Base.to_currency`'s machinery runs. Consequences, all
//! corpus-pinned:
//!
//! * **An unknown currency code does not raise.** Python falls back to
//!   `list(self.CURRENCY_FORMS.values())[0]`, which dict insertion order makes
//!   XOF. So `to_currency(1, "JPY")` is `"kelen seefa"`, not a
//!   `NotImplementedError`. See [`FALLBACK_CODE`].
//! * **`CURRENCY_PRECISION` is never consulted.** BM's `to_currency` truncates
//!   the fraction to two digits by *string slicing*, so KWD/BHD (3-decimal)
//!   and JPY (0-decimal) get plain 1/100 subunits like everything else.
//! * **`pluralize` is never called** by BM's own `to_currency`; it indexes
//!   `cr1[1]`/`cr1[0]` directly. Every BM form is a 2-tuple of *identical*
//!   strings (`("seefa", "seefa")`), so the singular/plural choice is
//!   observationally a no-op — but it is reproduced rather than dropped.
//! * **`adjective` is accepted and ignored**; `CURRENCY_ADJECTIVES` is empty.
//!
//! # The float/Decimal path
//!
//! BM does **not** override `to_cardinal_float`, and `Num2Word_Base`'s never
//! runs either: BM overrides `to_cardinal` and handles non-integers inline,
//! before the base class gets a look in. So none of `floatpath.rs` applies
//! here — **no `float2tuple`, no `10**precision` scaling, no banker's
//! rounding, and none of the f64 artefacts that path exists to preserve**.
//! BM's whole float path is one line:
//!
//! ```python
//! n = str(number).strip()
//! ```
//!
//! and then pure string surgery on the result: split at the ".", `int()` the
//! left, and map each *character* of the right through `self.ones`. The
//! consequences run opposite to every language that inherits base's path:
//!
//! * **The artefact cases are trivially right.** `str(2.675)` is `"2.675"`
//!   (`repr` is shortest-round-trip), so the digits are `6 7 5` by
//!   construction. Base's path is the one that has to compute
//!   `674.9999999999998` and rescue it with `abs(round(post) - post) < 0.01`;
//!   BM never scales, so there is nothing to rescue. Same for `1.005`.
//! * **`str()` is the whole spec**, so this port lives or dies on reproducing
//!   `repr(float)` and `str(Decimal)` byte for byte — see
//!   [`python_float_repr`] and [`python_decimal_str`].
//! * **Trailing zeros are significant**, because they are characters rather
//!   than a computed remainder: `Decimal("1.10")` is `"kelen ni kelen fu"`,
//!   and `Decimal("5.00")` is `"duuru ni fu fu"`.
//! * **Exponent notation raises `ValueError`**, since `int()` chokes on the
//!   literal — the same hole `split_currency` documents, reached the same way.
//!   `1e16` → `"1e+16"` → no `"."` → `int("1e+16")` raises quoting the whole
//!   literal; `1.5e16` → `"1.5e+16"` → the `"."` branch → `int("e")` raises
//!   quoting just the offending *character*. Both messages are reproduced.
//! * **`float("nan")`/`float("inf")` raise `ValueError` too**, not TypeError:
//!   `str()` gives `"nan"`/`"inf"`, and `int()` rejects them like any other
//!   non-numeric literal.
//!
//! `to_cheque` is **not** overridden, so `Num2Word_Base.to_cheque` runs — and
//! *that* one indexes `self.CURRENCY_FORMS[currency]` and raises
//! `NotImplementedError` on a miss. Hence the asymmetry the corpus records:
//! `to_currency(1234.56, "GBP")` == `"ba ni … seefa …"` while
//! `to_cheque(1234.56, "GBP")` raises. Only [`LangBm::lang_name`] and
//! [`LangBm::currency_forms`] are needed to serve it; `currency_precision`
//! (100), `money_verbose` (→ `to_cardinal`) and the rest stay at their trait
//! defaults, which already mirror `Num2Word_Base`.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.pointword`. Live on the float path, where BM interpolates it raw.
const POINTWORD: &str = "ni";

/// `self.ones`; index 0 is "fu" (zero) and is used as the zero word.
const ONES: [&str; 10] = [
    "fu",
    "kelen",
    "fila",
    "saba",
    "naani",
    "duuru",
    "wɔɔrɔ",
    "wolonfila",
    "segi",
    "kɔnɔntɔn",
];

/// `self.tens`; index 0 is "" and is unreachable (the caller guarantees n >= 10).
const TENS: [&str; 10] = [
    "",
    "tan",
    "mugan",
    "bisaba",
    "binaani",
    "biduuru",
    "biwɔɔrɔ",
    "biwolonfila",
    "bisegi",
    "bikɔnɔntɔn",
];

const HUNDRED: &str = "kɛmɛ";
const THOUSAND: &str = "ba";
const MILLION: &str = "miliyɔn";
/// 10^9, absent from Python (gladiaio/num2words2#147): Bamadaba "míliyari"
/// ("milliard"), tones dropped like [`MILLION`].
const BILLION: &str = "miliyari";

/// The exclusive ceiling (#147): no attested Bambara word for 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// `self.negword`. The trailing space is Python's, not a typo — see module docs.
const NEGWORD: &str = "dɔgɔ ";

/// `Num2Word_BM.to_currency`'s own default `separator=" "`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot be a plain parameter default.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_BM.to_currency` declares `separator=" "`, but the `Lang` trait has
/// no per-language defaults: `__init__.py`'s fast path (and
/// `bench/diff_test.py`) both send `kwargs.get("separator", ",")` — i.e.
/// `Num2Word_Base`'s default — so by the time the value reaches this side, the
/// information needed to tell "unset" from "explicitly a comma" is gone.
///
/// So `,` is read back as the unset sentinel and BM's own default restored.
/// This is the only reading that matches the oracle: every float row of the
/// `bm` currency corpus was generated by `num2words(v, lang="bm",
/// to="currency", currency=c)` with no `separator=`, and all of them expect a
/// plain space (`"… ero bisaba ni naani santimu"`). Same convention (and same
/// sentinel) as `lang_as.rs` and `lang_ca.rs`.
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets `" "` here where Python gives `","` (and note Python
/// concatenates it raw, so it would render `"ero,bisaba"` with no space).
/// Fixing that properly needs `Option<&str>` in the trait signature, which
/// lives in `base.rs` — outside this port's remit. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// The code Python's `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// `CURRENCY_FORMS` is a dict literal declaring XOF, USD, EUR in that order,
/// and Python 3.7+ dicts iterate in insertion order — so the `.get(currency,
/// <default>)` fallback for an unknown code is always XOF's `("seefa",
/// "seefa")`. Verified against the live interpreter: `to_currency(100, "ZZZ")`
/// == `"kɛmɛ seefa"`. A `HashMap` has no order, so the index is pinned here
/// rather than recovered from iteration.
const FALLBACK_CODE: &str = "XOF";

/// Python's inline currency split, which is a **string** operation, not an
/// arithmetic one:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `to_currency` has already taken `abs`, so this only ever sees a
/// non-negative value; the abs is re-applied here to keep the two steps in one
/// place.
///
/// Three consequences worth spelling out, all corpus-pinned:
///
/// * `right` is built by *left-justifying the first two fraction digits*, so
///   `0.5` → `"5"` → `"50"` → 50 santimu, while `0.05` → `"05"` → 5. It is a
///   digit-slice, not a multiplication, and it silently truncates anything
///   past the second decimal with no rounding (`12.349` → 34, `0.005` → 0).
/// * A fraction of `"0"` (i.e. `1.0`) yields `right == 0`, which
///   `to_currency` treats as falsy and drops the cents clause entirely — the
///   one place BM's float path agrees with base's int path (`1.0` →
///   `"kelen ero"`, not `"kelen ero fu santimu"`).
/// * A `CurrencyValue::Int` stringifies without a `"."` at all, so `parts` has
///   length 1 and `right` stays 0 — the same outcome, reached by a different
///   route.
///
/// # The exponent-notation hole
///
/// `str(float)` switches to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal:
/// `to_currency(1e16)` raises `ValueError: invalid literal for int() with
/// base 10: '1e+16'`. A negative `BigDecimal` scale is exactly the "the source
/// string used `e+` notation" signal (a plain digit string parses to scale 0,
/// never below), for floats and `Decimal`s alike — `str(Decimal("1E+16"))`
/// fails identically — so that arm is reproduced.
///
/// The `e-` side is **not** reproducible here and is flagged in the port
/// report: `1e-05` and `Decimal("0.00001")` parse to the *same* `BigDecimal`
/// (digits 1, scale 5) yet Python raises `ValueError` for the first and
/// returns `"fu ero"` for the second. The discriminator is the original
/// string, which the `CurrencyValue` boundary does not carry.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let d = match val {
        // str(int) never contains ".", so parts == [digits] and right == 0.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    // value == digits * 10^-scale
    let (digits, scale) = d.as_bigint_and_exponent();

    if scale < 0 {
        // str(d) would be "1e+16"-shaped; int() on it raises. Python's message
        // quotes the exact offending literal, which is unrecoverable from the
        // parsed value — the type is what callers observe.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            d
        )));
    }

    // No "." in str(d): parts == [str(d)], so right stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    // `d` is non-negative, so this is a bare ASCII digit string.
    let s = digits.abs().to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

/// Python's `_int_to_word`.
///
/// Only ever called with a **non-negative** value: `to_cardinal` strips the
/// sign before recursing, so the `self.ones[number]` / `self.tens[t]` lookups
/// can never hit Python's negative-index wraparound. The `to_usize` calls below
/// are each guarded by the range test immediately preceding them.
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);

    if number.is_zero() {
        return Ok(ONES[0].to_string());
    }

    if number < &ten {
        // 0 < number < 10, so the cast is total.
        let i = number.to_usize().expect("guarded: 0 < number < 10");
        return Ok(ONES[i].to_string());
    }

    if number < &hundred {
        // divmod: both operands non-negative, so div_rem == Python's divmod.
        let (t, o) = number.div_rem(&ten);
        let ti = t.to_usize().expect("guarded: 1 <= t <= 9");
        let oi = o.to_usize().expect("guarded: 0 <= o <= 9");
        let tail = if oi != 0 {
            format!(" ni {}", ONES[oi])
        } else {
            String::new()
        };
        return Ok(format!("{}{}", TENS[ti], tail));
    }

    if number < &thousand {
        let (h, r) = number.div_rem(&hundred);
        let hi = h.to_usize().expect("guarded: 1 <= h <= 9");
        // `h > 1`: exactly 100 is bare "kɛmɛ", never "kɛmɛ kelen".
        let base = if hi > 1 {
            format!("{} {}", HUNDRED, ONES[hi])
        } else {
            HUNDRED.to_string()
        };
        let tail = if !r.is_zero() {
            format!(" ni {}", int_to_word(&r)?)
        } else {
            String::new()
        };
        return Ok(format!("{}{}", base, tail));
    }

    if number < &million {
        let (t, r) = number.div_rem(&thousand);
        // `t > 1`: exactly 1000 is bare "ba", never "ba kelen".
        let base = if t > BigInt::from(1) {
            format!("{} {}", THOUSAND, int_to_word(&t)?)
        } else {
            THOUSAND.to_string()
        };
        let tail = if !r.is_zero() {
            format!(" ni {}", int_to_word(&r)?)
        } else {
            String::new()
        };
        return Ok(format!("{}{}", base, tail));
    }

    if number < &billion {
        let (m, r) = number.div_rem(&million);
        // `m > 1`: exactly 10^6 is bare "miliyɔn", never "miliyɔn kelen".
        let base = if m > BigInt::from(1) {
            format!("{} {}", MILLION, int_to_word(&m)?)
        } else {
            MILLION.to_string()
        };
        let tail = if !r.is_zero() {
            format!(" ni {}", int_to_word(&r)?)
        } else {
            String::new()
        };
        return Ok(format!("{}{}", base, tail));
    }

    // Python: `return str(number)`, the silent digit fallback — see module
    // docs quirk 1. The miliyari branch instead, composed like the million
    // one (#147); the maxval check above keeps `b` below 1000.
    let (b, r) = number.div_rem(&billion);
    // `b > 1`: exactly 10^9 is bare "miliyari", like "miliyɔn".
    let base = if b > BigInt::from(1) {
        format!("{} {}", BILLION, int_to_word(&b)?)
    } else {
        BILLION.to_string()
    };
    let tail = if !r.is_zero() {
        format!(" ni {}", int_to_word(&r)?)
    } else {
        String::new()
    };
    Ok(format!("{}{}", base, tail))
}

/// CPython's `repr(float)` — which for a `float` is also `str(float)`, and
/// therefore the entire input to BM's float path.
///
/// Two halves, both load-bearing.
///
/// # 1. The digits
///
/// `repr` is shortest-round-trip, and so is Rust's `{}`/`{:e}`, so the digits
/// agree — *except on exact ties*, where the double sits precisely halfway
/// between two equally short candidates that both round-trip. CPython's dtoa
/// breaks those to **even**; Rust's shortest formatter does not (it is neither
/// ties-to-even nor consistently ties-away — it disagrees with CPython on
/// roughly 1 double in 10,000 sampled uniformly). `670352580196876.25` is
/// exactly such a value: CPython prints `670352580196876.2`, Rust's `{:e}`
/// gives `...3`, and BM would then say `saba` where Python says `fila`.
///
/// The repair is to re-derive the digits through Rust's *exact* formatter
/// (`{:.n$}`), which **is** round-half-to-even, once `{:e}` has told us how
/// many fractional digits the shortest form has. A tie cannot change that
/// count (a carry would produce a *shorter* representation, which the shortest
/// algorithm would have found first), so the count is safe to take from `{:e}`
/// even though the final digit is not. Differentially tested against CPython
/// on 300k doubles — 678 mismatches without this step, 0 with it.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`
/// (`format_float_short`, format code `'r'`), pads the exponent to two digits,
/// and appends `.0` to anything that would otherwise look like an integer.
/// Rust's `{}` does none of this — it never uses exponent form and never adds
/// `.0` — so `1e16` and `1.0` would both come out wrong, in opposite
/// directions. Both matter here: `str(1.0)` is `"1.0"` → `"kelen ni fu"`, and
/// `str(1e16)` is `"1e+16"` → `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this. It is `abs(Decimal(str(value)).as_tuple().exponent)`,
/// which for an exponent-form repr is the *exponent*, not a digit count:
/// `1e16` arrives with `precision == 16`, and `1e-05` with `precision == 5`.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). BM feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and BM renders that
    // "dɔgɔ fu ni fu".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `{:e}` is shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the digits
    // and the decimal-point position fall straight out. `decpt` is CPython's:
    // the value is `0.<digits> * 10**decpt`.
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

/// CPython's `str(Decimal)` — the spec's to-scientific-string, transcribed
/// from `_pydecimal.Decimal.__str__`:
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # no exponent required
/// else:
///     dotplace = 1                   # scientific: 1 digit left of the point
/// if dotplace <= 0:
///     intpart, fracpart = '0', '.' + '0'*(-dotplace) + self._int
/// elif dotplace >= len(self._int):
///     intpart, fracpart = self._int + '0'*(dotplace-len(self._int)), ''
/// else:
///     intpart, fracpart = self._int[:dotplace], '.' + self._int[dotplace:]
/// exp = '' if leftdigits == dotplace else 'E' + "%+d" % (leftdigits-dotplace)
/// return sign + intpart + fracpart + exp
/// ```
///
/// `BigDecimal` is the same (unscaled, scale) pair as Python's
/// `(_int, _exp)` — `_exp == -scale` — and `from_str` preserves the scale as
/// written, so `Decimal("1.10")`'s trailing zero survives the crossing.
///
/// Note this reads `as_bigint_and_exponent()` rather than `BigDecimal`'s own
/// `Display`, which is **not** `str(Decimal)`: it renders `Decimal("0.00")` as
/// `"0"`, losing the two digits BM would have spoken (`"fu ni fu fu"`). The
/// capital `E` and the unpadded exponent are Python's too — `str` gives
/// `"1E-7"` where a float would repr as `"1e-07"`.
///
/// # The negative-zero hole
///
/// Python's `Decimal` carries a sign flag independent of its digits, so
/// `Decimal("-0.0")` is negative and BM prepends `dɔgɔ`. `BigInt` has no
/// negative zero, so `BigDecimal::from_str("-0.0")` discards the sign before
/// this function ever sees it, and the `dɔgɔ` is lost. The discriminator is
/// the original string, which the `FloatValue::Decimal` boundary does not
/// carry — the same shape of hole `split_currency` documents for `1e-05`.
/// Affects `Decimal("-0")`, `Decimal("-0.0")`, `Decimal("-0.00")`, … only;
/// the *float* `-0.0` is fine, because f64 keeps its sign bit. Flagged in the
/// port report.
fn python_decimal_str(d: &BigDecimal) -> String {
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let sign = if unscaled.is_negative() { "-" } else { "" };
    // Python's `_int`: the unsigned coefficient. BigInt renders ASCII digits.
    let int_digits = unscaled.abs().to_string();
    let exp: i64 = -scale;
    let ndig = int_digits.chars().count() as i64;
    let leftdigits = exp + ndig;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), int_digits),
        )
    } else if dotplace >= ndig {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - ndig) as usize)),
            String::new(),
        )
    } else {
        let k = dotplace as usize;
        (
            int_digits.chars().take(k).collect::<String>(),
            format!(".{}", int_digits.chars().skip(k).collect::<String>()),
        )
    };

    // `"%+d"` — always signed, never zero-padded. Capital E: context.capitals
    // defaults to 1.
    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}

/// `str(number)` for whatever the Python dispatcher handed the converter.
///
/// The `FloatValue` split is exactly Python's `isinstance(value, Decimal)`:
/// the two arms stringify by different rules and must not be collapsed.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Python's `int(s)`, for the strings `str()` can produce.
///
/// `BigInt::from_str` and `int()` agree on everything reachable here: plain
/// ASCII digit runs with an optional sign. They diverge on inputs `str(float)`
/// and `str(Decimal)` cannot emit (`int` also accepts surrounding whitespace,
/// underscore separators and non-ASCII decimal digits), so the divergence is
/// unreachable rather than papered over.
///
/// The message is Python's, quoting the offending literal verbatim.
fn python_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Bambara.
///
/// BM's `setup()` assigns only constants, which live as module consts here, so
/// the sole per-instance state is the `CURRENCY_FORMS` table — built once in
/// [`LangBm::new`] and cached by the generated registry's `OnceLock`. Building
/// it per call is what made an earlier revision of this port an order of
/// magnitude slower than the Python it replaces.
pub struct LangBm {
    /// `Num2Word_BM.CURRENCY_FORMS`. Declared on the class itself, *not*
    /// inherited: BM subclasses `Num2Word_Base` (whose table is an empty dict)
    /// and never touches `Num2Word_EUR`, so English's import-time mutation of
    /// the shared EUR dict cannot reach it. Confirmed against the live
    /// interpreter — `CONVERTER_CLASSES["bm"].CURRENCY_FORMS` has exactly
    /// these three codes and no EN-injected extras.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangBm {
    fn default() -> Self {
        Self::new()
    }
}

impl LangBm {
    pub fn new() -> Self {
        let mut forms = HashMap::with_capacity(3);
        // Insertion order is irrelevant to a HashMap; the "first value"
        // fallback Python relies on is pinned by FALLBACK_CODE instead.
        forms.insert(
            "XOF",
            CurrencyForms::new(&["seefa", "seefa"], &["santimu", "santimu"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["dolari", "dolari"], &["santimu", "santimu"]),
        );
        forms.insert(
            "EUR",
            CurrencyForms::new(&["ero", "ero"], &["santimu", "santimu"]),
        );
        LangBm { forms }
    }

    /// `Num2Word_BM.to_cardinal`, driven by the string rather than the value.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + self.ones[int(digit)]
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The integer [`Lang::to_cardinal`] models the same method on the *value*,
    /// which is equivalent there (stripping a leading "-" from `str(int)` is
    /// just `abs`) and much cheaper. Once a "." can appear that equivalence
    /// breaks, so this arm follows Python literally.
    ///
    /// Three details that look like slips and are not:
    ///
    /// * The recursion passes a **string** to `to_cardinal`, whose first act is
    ///   `str(number)` — a no-op on a `str`. So it re-enters here, not through
    ///   the `BigInt` overload, and a negative float keeps the "." branch.
    /// * `split(".", 1)` splits on the **first** dot only, so a second one
    ///   lands in `right` and reaches `int(digit)` as a bad literal. Not
    ///   reachable from `str()`, but `split_once` keeps the semantics anyway.
    /// * `int(digit)` is called per **character**, so the ValueError from a
    ///   malformed fraction quotes one character (`'e'`), where a malformed
    ///   whole `n` quotes the entire literal (`'1e+16'`).
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        // Python's str.strip(); str()'s output never has surrounding space, so
        // this only ever no-ops. Reproduced rather than assumed away.
        let n = n.trim();

        if let Some(rest) = n.strip_prefix('-') {
            let inner = self.cardinal_from_str(rest)?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }

        if let Some((left, right)) = n.split_once('.') {
            // `int(left)` runs before the loop, so a bad left raises first.
            let mut ret = format!("{} {}", int_to_word(&python_int(left)?)?, POINTWORD);
            for ch in right.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })? as usize;
                ret.push(' ');
                ret.push_str(ONES[d]);
            }
            // Python's `ret.strip()`. `ret` starts with a word and ends with
            // one whenever `right` is non-empty, so this too is a no-op.
            return Ok(ret.trim().to_string());
        }

        int_to_word(&python_int(n)?)
    }
}

impl Lang for LangBm {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }


    fn cardinal_float_entry(
        &self,
        value: &crate::floatpath::FloatValue,
        precision_override: Option<u32>,
    ) -> crate::base::Result<String> {
        // Python's to_cardinal routes every float/Decimal through this
        // language's own decimal grammar — 5.0 keeps its ".0" tail
        // ("comma nulla"), unlike Base's whole-value integer route.
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + "nan"` with no verify_ordinal guard, so a
    /// float keeps its spelled-out ".0" tail and the suffix lands on the last
    /// word: `to_ordinal(5.0)` == "duuru ni funan", `to_ordinal(-1.5)` ==
    /// "dɔgɔ kelen ni duurunan". Exponent-form reprs raise the same
    /// ValueError the cardinal path does (`1e16` → int("1e+16")).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}nan", self.to_cardinal_float(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "nan"` — the repr
    /// verbatim, sign and trailing zeros included: `-0.0` → "-0.0nan",
    /// `Decimal("5.00")` → "5.00nan", `1e16` → "1e+16nan".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}nan", repr_str))
    }

    /// `converter.str_to_number` — base `Decimal(value)` semantics, except
    /// that an Infinity parse is surfaced as the ValueError BM's own
    /// `to_cardinal` raises one step later: `str(Decimal("Infinity"))` has no
    /// "." and `int("Infinity")` chokes on the literal. The dispatcher's
    /// default maps `ParsedNumber::Inf` to base's OverflowError, which BM —
    /// whose to_cardinal never calls `int()` on the *value* — can never
    /// raise. NaN keeps the default routing (its ValueError type matches).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "XOF"
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

    /// `self.pointword`. Live: BM's float path interpolates it between the
    /// integral part and the spelled-out fraction digits.
    fn pointword(&self) -> &str {
        POINTWORD
    }

    /// BM reaches non-integers through its own `to_cardinal` override, not
    /// through `Num2Word_Base.to_cardinal_float`, so this hook exists only to
    /// stop the base implementation running. See the module docs: there is no
    /// `float2tuple` here, and no rounding of any kind.
    ///
    /// `precision_override` is the `precision=` kwarg. `__init__.py` applies it
    /// by assigning `converter.precision`, which BM never reads — so it is
    /// accepted and **ignored**, matching the live interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_from_str(&python_str(value))
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// ...
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// Integer input only, so `str(number)` never contains "." and the decimal
    /// branch is dead. Stripping the leading "-" from the *string* is exactly
    /// `abs()` for an integer, so the recursion is modelled on the value.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.to_cardinal(&value.abs())?;
            // Python's `.strip()` on the concatenation; the trailing space of
            // negword is always absorbed, so this only ever no-ops.
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        int_to_word(value)
    }

    /// Python: `return self.to_cardinal(number) + "nan"`.
    ///
    /// Note the suffix lands on the *whole* cardinal, so it attaches to the
    /// final word: `to_ordinal(-999)` == "dɔgɔ ... ni kɔnɔntɔnnan".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}nan", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "nan"` — the sign survives, so
    /// `to_ordinal_num(-1)` == "-1nan".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}nan", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is accepted and ignored; negative years get no era suffix,
    /// just the plain negword ("dɔgɔ kɛmɛ duuru" for -500).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ----------------------------------------------------

    /// `self.__class__.__name__`, for `Num2Word_Base.to_cheque`'s
    /// `NotImplementedError` message. BM's own `to_currency` never raises it
    /// (it falls back to XOF instead), so this is reachable via `to_cheque`
    /// only.
    fn lang_name(&self) -> &str {
        "Num2Word_BM"
    }

    /// `CURRENCY_FORMS[code]`.
    ///
    /// Consulted by the inherited `Num2Word_Base.to_cheque` — where a `None`
    /// is the `KeyError` that becomes `NotImplementedError`. BM's own
    /// `to_currency` deliberately does *not* go through this hook: it uses
    /// `.get(currency, <XOF>)` and never raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    /// Python:
    /// ```python
    /// def pluralize(self, n, forms):
    ///     if not forms:
    ///         return ""
    ///     return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code on every BM path — `to_currency` indexes the tuple itself and
    /// `to_cheque` takes `cr1[-1]` unconditionally — but BM defines it, so it
    /// is carried rather than left to the trait default (which raises
    /// `NotImplementedError`, as `Num2Word_Base.pluralize` does).
    ///
    /// Note the empty-`forms` guard returns `""`, and `forms[-1]` is the
    /// *last* form, not the second: a 3-form tuple would skip its middle
    /// entry. BM only ever has 2, both identical.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Python:
    /// ```python
    /// def to_currency(self, val, currency="XOF", cents=True, separator=" ",
    ///                 adjective=False):
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
    /// A wholesale override: none of `Num2Word_Base.to_currency` runs, so
    /// there is no `parse_currency_parts`, no `ROUND_HALF_UP` quantize, no
    /// per-currency divisor and no `NotImplementedError`. See the module docs.
    ///
    /// Points of order:
    ///
    /// * The unit word comes from `_int_to_word`, **not** `to_cardinal`, so it
    ///   shares the scales and the 10^12 ceiling (#147; Python's digit
    ///   fallback gave `"1000000000 ero"`) without inheriting the negword
    ///   handling. `is_negative` is taken from the *original* value
    ///   and re-applied at the end, which is why stripping the sign first is
    ///   safe here.
    /// * `cents and right` is an `and` over a Python int, so a zero `right`
    ///   drops the whole clause — floats with `.0` render like ints.
    /// * `negword` carries a trailing space and Python `.strip()`s the join.
    ///   The space is always absorbed by the following word, so the strip
    ///   no-ops; reproduced via `trim()` rather than assumed away.
    /// * `adjective` is ignored outright — the parameter is declared and never
    ///   read, and `CURRENCY_ADJECTIVES` is empty regardless.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore BM's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // `is_negative = val < 0` is evaluated before `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // `.get(currency, list(...values())[0])` — an unknown code silently
        // borrows XOF's forms instead of raising.
        let forms = self
            .forms
            .get(currency)
            .or_else(|| self.forms.get(FALLBACK_CODE))
            .expect("FALLBACK_CODE is inserted by new()");

        let one = BigInt::one();
        // cr1[1] if left != 1 else cr1[0] — a direct index, not pluralize().
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };
        let mut result = format!("{} {}", int_to_word(&left)?, unit);

        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            // Python concatenates the separator raw: an explicit `separator=","`
            // renders "ero,bisaba", with no space of its own.
            result.push_str(&format!(
                "{}{} {}",
                separator,
                int_to_word(&right)?,
                subunit
            ));
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    /// A currency call shaped exactly like `bench/diff_test.py`'s: `cents=True`,
    /// `separator=None` (the caller omitted it), `adjective=False`.
    ///
    /// Signature repair only — these helpers predate `CurrencyValue::parse`
    /// gaining `has_decimal` and `separator` becoming `Option<&str>`, and had
    /// stopped compiling. Same convention as `lang_ban.rs`; every assertion
    /// below is unchanged, and no implementation was touched.
    fn cur(arg: &str, code: &str) -> String {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangBm::new()
            .to_currency(&v, code, true, None, false)
            .unwrap()
    }

    fn cheque(arg: &str, code: &str) -> Result<String> {
        LangBm::new().to_cheque(&BigDecimal::from_str(arg).unwrap(), code)
    }

    fn c(n: i64) -> String {
        LangBm::new().to_cardinal(&BigInt::from(n)).unwrap()
    }
    fn o(n: i64) -> String {
        LangBm::new().to_ordinal(&BigInt::from(n)).unwrap()
    }
    fn on(n: i64) -> String {
        LangBm::new().to_ordinal_num(&BigInt::from(n)).unwrap()
    }
    fn y(n: i64) -> String {
        LangBm::new().to_year(&BigInt::from(n)).unwrap()
    }

    #[test]
    fn corpus_cardinal() {
        assert_eq!(c(0), "fu");
        assert_eq!(c(1), "kelen");
        assert_eq!(c(9), "kɔnɔntɔn");
        assert_eq!(c(10), "tan");
        assert_eq!(c(11), "tan ni kelen");
        assert_eq!(c(100), "kɛmɛ");
        assert_eq!(c(999), "kɛmɛ kɔnɔntɔn ni bikɔnɔntɔn ni kɔnɔntɔn");
        assert_eq!(c(1000), "ba");
        assert_eq!(c(1_000_000), "miliyɔn");
        assert_eq!(c(1_000_001), "miliyɔn ni kelen");
        assert_eq!(
            c(1_234_567),
            "miliyɔn ni ba kɛmɛ fila ni bisaba ni naani ni kɛmɛ duuru ni biwɔɔrɔ ni wolonfila"
        );
    }

    #[test]
    fn corpus_negative() {
        assert_eq!(c(-1), "dɔgɔ kelen");
        assert_eq!(c(-21), "dɔgɔ mugan ni kelen");
        assert_eq!(c(-100), "dɔgɔ kɛmɛ");
        assert_eq!(o(-1), "dɔgɔ kelennan");
        assert_eq!(on(-999), "-999nan");
    }

    #[test]
    fn corpus_ordinal_and_year() {
        assert_eq!(o(0), "funan");
        assert_eq!(o(10), "tannan");
        assert_eq!(on(0), "0nan");
        assert_eq!(y(-500), "dɔgɔ kɛmɛ duuru");
        assert_eq!(y(-44), "dɔgɔ binaani ni naani");
    }

    /// Quirk 1/2 (#147): miliyari at and above 10^9, OverflowError at 10^12.
    #[test]
    fn billion_scale_and_ceiling() {
        assert_eq!(c(1_000_000_000), "miliyari");
        assert_eq!(c(2_000_000_005), "miliyari fila ni duuru");
        assert_eq!(o(1_000_000_000), "miliyarinan");
        assert_eq!(on(1_000_000_000), "1000000000nan");

        let big: BigInt = "1000000000000".parse().unwrap();
        let lang = LangBm::new();
        assert!(matches!(lang.to_cardinal(&big), Err(N2WError::Overflow(_))));
        assert!(matches!(lang.to_ordinal(&big), Err(N2WError::Overflow(_))));
    }

    /// Quirk 3: negatives spell the sign as the word "dɔgɔ" in
    /// cardinal/ordinal but as ASCII "-" in ordinal_num.
    #[test]
    fn negative_sign_asymmetry() {
        assert_eq!(c(-1_000_000_000), "dɔgɔ miliyari");
        assert_eq!(o(-1_000_000_000), "dɔgɔ miliyarinan");
        assert_eq!(on(-1_000_000_000), "-1000000000nan");
        assert_eq!(y(-1_000_000_000), "dɔgɔ miliyari");
    }

    /// The three declared codes, straight from the corpus.
    #[test]
    fn corpus_currency_known_codes() {
        // Pure ints: no "." in str(), so no cents clause.
        assert_eq!(cur("0", "EUR"), "fu ero");
        assert_eq!(cur("1", "EUR"), "kelen ero");
        assert_eq!(cur("2", "EUR"), "fila ero");
        assert_eq!(cur("100", "EUR"), "kɛmɛ ero");
        assert_eq!(cur("1000000", "EUR"), "miliyɔn ero");
        // Floats with cents.
        assert_eq!(cur("12.34", "EUR"), "tan ni fila ero bisaba ni naani santimu");
        assert_eq!(cur("0.01", "EUR"), "fu ero kelen santimu");
        assert_eq!(
            cur("99.99", "EUR"),
            "bikɔnɔntɔn ni kɔnɔntɔn ero bikɔnɔntɔn ni kɔnɔntɔn santimu"
        );
        assert_eq!(
            cur("1234.56", "EUR"),
            "ba ni kɛmɛ fila ni bisaba ni naani ero biduuru ni wɔɔrɔ santimu"
        );
        // ljust: "5" -> "50", not 5.
        assert_eq!(cur("0.5", "EUR"), "fu ero biduuru santimu");
        // A float with zero cents renders like an int — BM's own quirk.
        assert_eq!(cur("1.0", "EUR"), "kelen ero");
        // Negative: negword prepended after the fact.
        assert_eq!(
            cur("-12.34", "EUR"),
            "dɔgɔ tan ni fila ero bisaba ni naani santimu"
        );
        assert_eq!(cur("1", "USD"), "kelen dolari");
        assert_eq!(cur("12.34", "USD"), "tan ni fila dolari bisaba ni naani santimu");
        assert_eq!(cur("1", "XOF"), "kelen seefa");
    }

    /// Unknown codes fall back to XOF rather than raising — including the
    /// 3-decimal (KWD/BHD) and 0-decimal (JPY) currencies, whose precision BM
    /// never consults. All corpus rows.
    #[test]
    fn corpus_currency_unknown_codes_fall_back_to_xof() {
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF", "ZZZ"] {
            assert_eq!(cur("0", code), "fu seefa");
            assert_eq!(cur("100", code), "kɛmɛ seefa");
            assert_eq!(cur("1.0", code), "kelen seefa");
            // JPY would round to a whole unit and KWD/BHD would use mils under
            // base.to_currency; BM's string slice gives plain 1/100 subunits.
            assert_eq!(cur("0.01", code), "fu seefa kelen santimu");
            assert_eq!(
                cur("1234.56", code),
                "ba ni kɛmɛ fila ni bisaba ni naani seefa biduuru ni wɔɔrɔ santimu"
            );
            assert_eq!(
                cur("-12.34", code),
                "dɔgɔ tan ni fila seefa bisaba ni naani santimu"
            );
        }
    }

    /// Not corpus-covered; verified against the live interpreter.
    #[test]
    fn currency_edges() {
        // The scales reach to_currency through _int_to_word (#147).
        assert_eq!(cur("1000000000.0", "EUR"), "miliyari ero");
        // Truncation past the second decimal, no rounding.
        assert_eq!(cur("12.349", "EUR"), "tan ni fila ero bisaba ni naani santimu");
        assert_eq!(cur("0.005", "EUR"), "fu ero");
        // A sub-santimu negative keeps its sign but loses the cents clause.
        assert_eq!(cur("-0.001", "EUR"), "dɔgɔ fu ero");
        // -0.0 is not < 0.
        assert_eq!(cur("-0.0", "EUR"), "fu ero");
        assert_eq!(cur("2.5", "EUR"), "fila ero biduuru santimu");
        assert_eq!(cur("-100", "EUR"), "dɔgɔ kɛmɛ ero");
        // cents=False drops the clause; BM has no terse path at all.
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        assert_eq!(
            LangBm::new().to_currency(&v, "EUR", false, None, false).unwrap(),
            "tan ni fila ero"
        );
        // An explicit non-comma separator is passed through raw (no space).
        let v = CurrencyValue::parse("12.34", false, true, true).unwrap();
        assert_eq!(
            LangBm::new().to_currency(&v, "EUR", true, Some(" ni "), false).unwrap(),
            "tan ni fila ero ni bisaba ni naani santimu"
        );
    }

    /// str(float) goes exponential at 1e16 and int() then raises ValueError.
    #[test]
    fn currency_exponent_literal_raises_value_error() {
        let v = CurrencyValue::parse("1e+16", false, true, true).unwrap();
        assert!(matches!(
            LangBm::new().to_currency(&v, "EUR", true, None, false),
            Err(N2WError::Value(_))
        ));
        // The same magnitude as an int is past maxval (#147): OverflowError.
        let v = CurrencyValue::parse("10000000000000000", false, true, true).unwrap();
        assert!(matches!(
            LangBm::new().to_currency(&v, "EUR", true, None, false),
            Err(N2WError::Overflow(_))
        ));
    }

    /// `to_cheque` is inherited from `Num2Word_Base`, so unlike `to_currency`
    /// it *does* raise on an unknown code. Both halves are corpus rows.
    #[test]
    fn corpus_cheque() {
        assert_eq!(
            cheque("1234.56", "EUR").unwrap(),
            "BA NI KƐMƐ FILA NI BISABA NI NAANI AND 56/100 ERO"
        );
        assert_eq!(
            cheque("1234.56", "USD").unwrap(),
            "BA NI KƐMƐ FILA NI BISABA NI NAANI AND 56/100 DOLARI"
        );
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match cheque("1234.56", code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{}\" not implemented for \"Num2Word_BM\"", code)
                ),
                other => panic!("{}: expected NotImplemented, got {:?}", code, other),
            }
        }
    }

    /// Not corpus-covered; verified against the live interpreter.
    #[test]
    fn cheque_edges() {
        assert_eq!(cheque("-1.0", "USD").unwrap(), "MINUS KELEN AND 00/100 DOLARI");
        assert_eq!(cheque("0", "XOF").unwrap(), "FU AND 00/100 SEEFA");
    }

    // ---- float / Decimal --------------------------------------------

    /// `num2words(v, lang="bm")` for a float.
    fn f(v: f64) -> Result<String> {
        LangBm::new().to_cardinal_float(
            &FloatValue::Float {
                value: v,
                // As computed by the shim: abs(Decimal(str(v)).as_tuple()
                // .exponent). BM ignores it; deliberately fed a wrong value
                // here to pin that it stays ignored.
                precision: 99,
            },
            None,
        )
    }

    /// `num2words(Decimal(s), lang="bm")`.
    fn d(s: &str) -> Result<String> {
        LangBm::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 99,
            },
            None,
        )
    }

    /// Every `"to": "cardinal"` corpus row for bm whose arg has a dot.
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(f(0.0).unwrap(), "fu ni fu");
        assert_eq!(f(0.5).unwrap(), "fu ni duuru");
        assert_eq!(f(1.0).unwrap(), "kelen ni fu");
        assert_eq!(f(1.5).unwrap(), "kelen ni duuru");
        assert_eq!(f(2.25).unwrap(), "fila ni fila duuru");
        assert_eq!(f(3.14).unwrap(), "saba ni kelen naani");
        assert_eq!(f(0.01).unwrap(), "fu ni fu kelen");
        assert_eq!(f(0.1).unwrap(), "fu ni kelen");
        assert_eq!(f(0.99).unwrap(), "fu ni kɔnɔntɔn kɔnɔntɔn");
        assert_eq!(f(1.01).unwrap(), "kelen ni fu kelen");
        assert_eq!(f(12.34).unwrap(), "tan ni fila ni saba naani");
        assert_eq!(
            f(99.99).unwrap(),
            "bikɔnɔntɔn ni kɔnɔntɔn ni kɔnɔntɔn kɔnɔntɔn"
        );
        assert_eq!(f(100.5).unwrap(), "kɛmɛ ni duuru");
        assert_eq!(
            f(1234.56).unwrap(),
            "ba ni kɛmɛ fila ni bisaba ni naani ni duuru wɔɔrɔ"
        );
        assert_eq!(f(-0.5).unwrap(), "dɔgɔ fu ni duuru");
        assert_eq!(f(-1.5).unwrap(), "dɔgɔ kelen ni duuru");
        assert_eq!(f(-12.34).unwrap(), "dɔgɔ tan ni fila ni saba naani");
    }

    /// Every `"to": "cardinal_dec"` corpus row for bm.
    #[test]
    fn corpus_cardinal_decimal() {
        assert_eq!(d("0.01").unwrap(), "fu ni fu kelen");
        // The trailing zero is a character, not a computed remainder.
        assert_eq!(d("1.10").unwrap(), "kelen ni kelen fu");
        assert_eq!(d("12.345").unwrap(), "tan ni fila ni saba naani duuru");
        // Issue #603: the Decimal arm never float-casts, so the ".99" survives
        // at trillion scale. The left part is past the 10^12 maxval (#147;
        // Python returned its digits).
        assert!(matches!(d("98746251323029.99"), Err(N2WError::Overflow(_))));
        assert_eq!(
            d("987462513.99").unwrap(),
            "miliyɔn kɛmɛ kɔnɔntɔn ni bisegi ni wolonfila ni ba kɛmɛ naani ni biwɔɔrɔ ni fila ni kɛmɛ duuru ni tan ni saba ni kɔnɔntɔn kɔnɔntɔn"
        );
        assert_eq!(d("0.001").unwrap(), "fu ni fu fu kelen");
    }

    /// The two artefact cases base's float path exists to rescue. BM never
    /// scales by `10**precision`, so `str()` hands it the right digits and the
    /// `< 0.01` heuristic is not merely unused but unreachable.
    #[test]
    fn float_artefacts_are_not_reachable() {
        // base.float2tuple computes 674.9999999999998 for this one.
        assert_eq!(f(2.675).unwrap(), "fila ni wɔɔrɔ wolonfila duuru");
        assert_eq!(f(1.005).unwrap(), "kelen ni fu fu duuru");
        assert_eq!(f(-1.005).unwrap(), "dɔgɔ kelen ni fu fu duuru");
        // Banker's rounding never enters either: these are literal digits.
        assert_eq!(f(2.5).unwrap(), "fila ni duuru");
        assert_eq!(f(0.5).unwrap(), "fu ni duuru");
        assert_eq!(f(0.005).unwrap(), "fu ni fu fu duuru");
    }

    /// CPython breaks exact shortest-repr ties to even; Rust's `{:e}` does not.
    /// 670352580196876.25 is exactly between ...876.2 and ...876.3, both of
    /// which round-trip. Python says `fila` (2). Verified against the live
    /// interpreter.
    /// (The integer parts are past maxval since #147, so the repr itself is
    /// pinned rather than the cardinal.)
    #[test]
    fn shortest_repr_ties_go_to_even() {
        assert_eq!(python_float_repr(670352580196876.2), "670352580196876.2");
        assert_eq!(python_float_repr(161834668665500.12), "161834668665500.12");
        assert_eq!(python_float_repr(-29489152302236.812), "-29489152302236.812");
    }

    /// `str(float)`'s two formatting quirks, both observable here.
    #[test]
    fn float_repr_placement() {
        // ADD_DOT_0: an integral float still has a fraction to speak.
        assert_eq!(f(1.0).unwrap(), "kelen ni fu");
        assert_eq!(f(-1.0).unwrap(), "dɔgɔ kelen ni fu");
        // repr(-0.0) keeps the sign bit; `value < 0.0` would not.
        assert_eq!(f(-0.0).unwrap(), "dɔgɔ fu ni fu");
        // The scales and the 10^12 ceiling reach the float path too (#147).
        assert_eq!(f(1000000000.5).unwrap(), "miliyari ni duuru");
        assert!(matches!(f(1e15), Err(N2WError::Overflow(_))));
        // Just inside the exponent threshold (decpt == 16): past maxval.
        assert!(matches!(f(9999999999999998.0), Err(N2WError::Overflow(_))));
        // 1e-4 stays positional (decpt == -3); 1e-5 does not.
        assert_eq!(f(0.0001).unwrap(), "fu ni fu fu fu kelen");
    }

    /// Exponent notation reaches int() as a bad literal. Two distinct
    /// messages, because two distinct int() calls raise. Live-interpreter
    /// verified; not corpus-covered.
    #[test]
    fn exponent_notation_raises_value_error() {
        // No "." in the literal -> int(n) raises, quoting all of it. Note the
        // negatives: the sign is stripped by the negword branch *before*
        // int() sees the string, so the message never carries a "-".
        for (v, lit) in [
            (1e16, "1e+16"),
            (1e21, "1e+21"),
            (1e-5, "1e-05"),
            (1e300, "1e+300"),
            (-1e16, "1e+16"),
            (-1e-5, "1e-05"),
            (f64::INFINITY, "inf"),
            (f64::NEG_INFINITY, "inf"),
        ] {
            match f(v) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", lit)
                ),
                other => panic!("{}: expected Value, got {:?}", lit, other),
            }
        }
        // NaN's repr has no sign to strip.
        match f(f64::NAN) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'nan'")
            }
            other => panic!("nan: expected Value, got {:?}", other),
        }
        // A "." *is* present -> the fraction loop raises on one character.
        for v in [1.5e16, 1.5e-5] {
            match f(v) {
                Err(N2WError::Value(m)) => {
                    assert_eq!(m, "invalid literal for int() with base 10: 'e'")
                }
                other => panic!("{}: expected Value, got {:?}", v, other),
            }
        }
    }

    /// `str(Decimal)` is not `str(float)` and not `BigDecimal`'s Display.
    #[test]
    fn decimal_str_rules() {
        // Display would say "0"; str(Decimal("0.00")) keeps both digits.
        assert_eq!(d("0.00").unwrap(), "fu ni fu fu");
        assert_eq!(d("5.00").unwrap(), "duuru ni fu fu");
        // Integral Decimals have no "." at all.
        assert_eq!(d("5").unwrap(), "duuru");
        assert_eq!(d("0").unwrap(), "fu");
        assert_eq!(d("-1").unwrap(), "dɔgɔ kelen");
        assert_eq!(d("-0.5").unwrap(), "dɔgɔ fu ni duuru");
        // Arbitrary precision: no f64 could hold this.
        assert_eq!(
            d("1.000000000000000000001").unwrap(),
            "kelen ni fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu fu kelen"
        );
        // str(Decimal) goes scientific at exp > 0 or adjusted exp < -6, with a
        // capital E and an unpadded exponent -- "1E-7", not "1e-07".
        for (s, lit) in [
            ("1E+16", "1E+16"),
            ("0.0000001", "1E-7"),
            ("-0E+2", "0E+2"),
        ] {
            match d(s) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", lit)
                ),
                other => panic!("{}: expected Value, got {:?}", s, other),
            }
        }
        // 12E2 renormalises to one digit left of the point: "1.2E+3", so the
        // fraction loop raises on 'E' rather than int() on the whole literal.
        match d("12E2") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'E'")
            }
            other => panic!("12E2: expected Value, got {:?}", other),
        }
        // Exactly at the scientific threshold: adjusted exponent -6 flips it.
        assert_eq!(d("0.000001").unwrap(), "fu ni fu fu fu fu fu kelen");
    }

    /// BM never reads `self.precision`, so `precision=` is inert. Verified
    /// against the live interpreter: `num2words(2.675, lang="bm",
    /// precision=1)` is unchanged.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangBm::new();
        let v = FloatValue::Float {
            value: 2.675,
            precision: 3,
        };
        assert_eq!(
            l.to_cardinal_float(&v, Some(1)).unwrap(),
            "fila ni wɔɔrɔ wolonfila duuru"
        );
        assert_eq!(
            l.to_cardinal_float(&v, Some(9)).unwrap(),
            "fila ni wɔɔrɔ wolonfila duuru"
        );
        let z = FloatValue::Float {
            value: 0.5,
            precision: 1,
        };
        assert_eq!(l.to_cardinal_float(&z, Some(5)).unwrap(), "fu ni duuru");
    }

    /// Dead on every BM path, but defined by the class.
    #[test]
    fn pluralize_forms() {
        let l = LangBm::new();
        let forms = vec!["a".to_string(), "b".to_string()];
        assert_eq!(l.pluralize(&BigInt::from(1), &forms).unwrap(), "a");
        assert_eq!(l.pluralize(&BigInt::from(2), &forms).unwrap(), "b");
        assert_eq!(l.pluralize(&BigInt::from(2), &[]).unwrap(), "");
    }
}
