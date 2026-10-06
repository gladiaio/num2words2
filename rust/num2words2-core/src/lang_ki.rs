//! Port of `lang_KI.py` (Kikuyu / Gikuyu).
//!
//! Shape: **self-contained**. `Num2Word_KI` subclasses `Num2Word_Base` but its
//! `setup` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python
//! never populates `self.cards` and never sets `MAXVAL`. All four in-scope
//! methods — `to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year` — are
//! overridden outright, so nothing from the base engine (`splitnum`, `clean`,
//! `merge`, `title`, `verify_ordinal`) is ever reached. Accordingly
//! `cards`/`maxval`/`merge` stay at their trait defaults here and there is **no
//! overflow check** at any magnitude.
//!
//! Nothing is inherited that matters: `setup` sets `negword = "njuru "`,
//! `pointword = "ona"` and `exclude_title = ["na", "ona", "njuru"]`, but
//! `exclude_title` is dead code because `to_cardinal` never calls `title()`
//! (and `is_title` is false by default anyway).
//!
//! No cross-call mutable state: `Num2Word_KI` stores nothing between calls, and
//! `base.py` has no `str_to_number`/`_pending_*` handshake for this class to
//! participate in. The stateless Rust path is faithful.
//!
//! # Structure of the algorithm
//!
//! `_int_to_word` is a plain recursive descent over four magnitude bands, all
//! joined with `" na "`:
//!
//! * `0`              → "wĩra"
//! * `< 10`           → `ONES[n]`
//! * `< 100`          → `TENS[t]` + optional `" na " + ONES[o]`
//! * `< 1_000`        → optional `ONES[h] + " "` (only when `h > 1`) + "igana"
//! * `< 1_000_000`    → optional `_int_to_word(t) + " "` (only when `t > 1`) + "ngiri"
//! * `< 1_000_000_000`→ optional `_int_to_word(m) + " "` (only when `m > 1`) + "milioni"
//!
//! The `h > 1` / `t > 1` / `m > 1` guards are what make 100 → "igana" (not
//! "ĩmwe igana") and 1000 → "ngiri" (not "ĩmwe ngiri"), while 200 → "igĩrĩ
//! igana".
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following look wrong but are exactly what
//! Python emits, and are confirmed by the frozen corpus:
//!
//! 1. **Anything `>= 1_000_000_000` is not spelled out at all.** `_int_to_word`
//!    falls off the end of its band ladder and does `return str(number)`, so
//!    `to_cardinal(10**9)` == "1000000000" (bare digits) and
//!    `to_ordinal(1234567890)` == "wa 1234567890". No exception, no words. This
//!    is the single biggest quirk of the module and is preserved verbatim; see
//!    [`int_to_word`]. It also means there is no upper bound to guard — a
//!    10^606 input simply renders as its own decimal digits, which is why this
//!    port keeps `BigInt` end to end and only narrows the provably-bounded
//!    digit indices.
//! 2. **No zero-suppression between bands.** 1_000_001 → "milioni na ĩmwe", and
//!    999_999_999 spells all three bands out longhand with `" na "` between
//!    each, giving the (correct-per-Python) mouthful
//!    "kenda igana na mĩrongo kenda na kenda milioni na kenda igana na mĩrongo
//!    kenda na kenda ngiri na kenda igana na mĩrongo kenda na kenda".
//! 3. **`to_ordinal` accepts negatives and never validates.** KI's `to_ordinal`
//!    is `"wa " + self.to_cardinal(number)` and does *not* call the inherited
//!    `verify_ordinal`, so `to_ordinal(-1)` == "wa njuru ĩmwe" rather than
//!    raising. Likewise `to_ordinal_num(-1)` == "wa -1".
//! 4. **`to_year` ignores its `longval` kwarg** and is a bare alias for
//!    `to_cardinal`, so negative years read "njuru ..." rather than using any
//!    BC/AD convention: `to_year(-44)` == "njuru mĩrongo ĩna na inya".
//!
//! # Currency
//!
//! `Num2Word_KI` overrides `to_currency` **wholesale**, sharing nothing with
//! `Num2Word_Base.to_currency`: no `parse_currency_parts`, no `pluralize`, no
//! `CURRENCY_PRECISION`, no `has_decimal` guard. So `default_to_currency` is
//! bypassed entirely and [`LangKi::to_currency`] transcribes the Python body.
//! `to_cheque` is the opposite case — KI does *not* override it, so it runs
//! `Num2Word_Base.to_cheque` verbatim and the trait default already covers it.
//! That split is the whole story of this surface, and it produces four quirks,
//! all corpus-pinned:
//!
//! 1. **`to_currency` never raises on an unknown code, but `to_cheque` does.**
//!    Python's `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`
//!    silently lends KES's ("shilingi", "senti") to GBP/JPY/KWD/BHD/INR/CNY/CHF,
//!    while `to_cheque` indexes `CURRENCY_FORMS[currency]` and raises
//!    `NotImplementedError`. Hence the corpus pairing a happy `currency:GBP`
//!    with an erroring `cheque:GBP`. See [`FALLBACK_CODE`].
//! 2. **`CURRENCY_PRECISION` never reaches `to_currency`.** KI declares none and
//!    base's is `{}`, so the divisor would be 100 — but `to_currency` does not
//!    consult it at all. The 3-decimal codes truncate at *two* digits anyway
//!    (KWD `1234.56` → 56 senti, not 560 mils) and the 0-decimal code still
//!    prints a cents segment (JPY `0.5` → "… mĩrongo ĩtano senti") rather than
//!    rounding to a whole unit. Only `to_cheque` reads it, and gets 100.
//! 3. **The cents clause hangs off `right`'s truthiness, not `has_decimal`.**
//!    Python's `if cents and right:` means a zero subunit drops the clause no
//!    matter how the value was spelled: `1.0` → "ĩmwe yuro" and
//!    `Decimal("5.00")` → "ithano yuro", never "… wĩra senti". KI therefore
//!    cannot print a zero-cents segment at all.
//! 4. **`pluralize` is dead code.** `to_currency` indexes the tuple itself
//!    (`cr1[1] if left != 1 else cr1[0]`) and `to_cheque` takes `cr1[-1]`
//!    unconditionally, so neither ever calls it. It is ported regardless, since
//!    it is part of the class surface. Moot for output either way: every KI
//!    entry is a 2-tuple of *identical* strings (`("shilingi", "shilingi")`), so
//!    the singular and plural arms cannot be told apart.
//!
//! # Errors
//!
//! `_int_to_word` is only ever entered with a non-negative value (`to_cardinal`
//! strips the sign before parsing), so the `ONES`/`TENS` lookups cannot go out
//! of range and Python's negative-index wraparound never fires. The reachable
//! errors are all `ValueError`/`NotImplementedError`: `NotImplementedError` out
//! of `to_cheque` for a code outside KES/USD/EUR, the `ValueError` documented on
//! [`split_currency`], and — on the float path — the `ValueError` from `int()`
//! choking on an exponent-form repr (`1e+16`, `1.5e-05`, `Decimal("1E+2")`); see
//! [`cardinal_from_str`].

use crate::base::{Lang as LangTrait, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `setup`: `self.ones`. Index 0 is "" and is only reachable through the float
/// path (see [`cardinal_from_str`]), where `or "wĩra"` substitutes for it.
const ONES: [&str; 10] = [
    "",
    "ĩmwe",
    "igĩrĩ",
    "ithatũ",
    "inya",
    "ithano",
    "ithathatũ",
    "mũgwanja",
    "inyanya",
    "kenda",
];

/// `setup`: `self.tens`. Index 0 is "" and is unreachable (the `< 10` band
/// returns before the tens band can produce `t == 0`).
const TENS: [&str; 10] = [
    "",
    "ikũmi",
    "mĩrongo ĩĩrĩ",
    "mĩrongo ĩtatũ",
    "mĩrongo ĩna",
    "mĩrongo ĩtano",
    "mĩrongo ĩtandatũ",
    "mĩrongo mũgwanja",
    "mĩrongo ĩnyanya",
    "mĩrongo kenda",
];

/// `setup`: `self.negword`. Keeps its trailing space, as in Python.
const NEGWORD: &str = "njuru ";
/// `setup`: `self.pointword`. Interpolated between the integer part and the
/// spelled-out fraction digits in the float path.
const POINTWORD: &str = "ona";
/// `setup`: `self.hundred`.
const HUNDRED: &str = "igana";
/// `setup`: `self.thousand`.
const THOUSAND: &str = "ngiri";
/// `setup`: `self.million`.
const MILLION: &str = "milioni";
/// `_int_to_word`'s literal for zero. Also the float path's `or "wĩra"` filler.
const ZERO_WORD: &str = "wĩra";
/// The universal joiner in `_int_to_word`.
const NA: &str = " na ";
/// `to_ordinal` / `to_ordinal_num` prefix.
const WA: &str = "wa ";

/// The code Python's `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// `CURRENCY_FORMS` is a class-body dict literal declaring KES, USD, EUR in
/// that order, and Python 3.7+ dicts iterate in insertion order — so the
/// `.get(currency, <default>)` fallback for an unknown code is always KES's
/// `("shilingi", "senti")`. Verified against the live interpreter:
/// `to_currency(100, "ZZZ")` == `"igana shilingi"`. A `HashMap` has no order,
/// so the index Python takes is pinned here rather than recovered from
/// iteration.
const FALLBACK_CODE: &str = "KES";

/// Python's inline currency split, which is a **string** operation rather than
/// an arithmetic one:
///
/// ```text
/// parts = str(val).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `to_currency` has already taken `abs` by this point; the abs is re-applied
/// here to keep both steps in one place.
///
/// Three consequences, all corpus-pinned:
///
/// * `right` is the *first two fraction digits, left-justified*, so `0.5` →
///   `"5"` → `"50"` → 50 senti while `0.05` → `"05"` → 5. It is a digit slice,
///   not a multiplication: everything past the second decimal is truncated with
///   no rounding (`12.349` → 34, `0.005` → 0).
/// * A fraction of `"0"` (i.e. `1.0`) yields `right == 0`, which `to_currency`
///   reads as falsy and drops the cents clause for — module docs, Currency
///   quirk 3.
/// * A `CurrencyValue::Int` stringifies with no `"."` at all, so `parts` has
///   length 1 and `right` stays 0 — same outcome by a different route.
///
/// # The exponent-notation hole
///
/// `str(float)` switches to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal: `to_currency(1e16)`
/// raises `ValueError: invalid literal for int() with base 10: '1e+16'`. A
/// negative `BigDecimal` scale is exactly the "the source string used `e+`
/// notation" signal — a plain digit string parses to scale 0, never below — for
/// floats and `Decimal`s alike, since `str(Decimal("1E+16"))` fails
/// identically. So that arm is reproduced.
///
/// The `e-` half is **not** reproducible at this boundary, and is flagged in the
/// port report: `1e-05` and `Decimal("0.00001")` parse to the same `BigDecimal`
/// (digits 1, scale 5), yet Python raises `ValueError` for the first and returns
/// `"wĩra yuro"` for the second. The discriminator is the original string, which
/// `CurrencyValue` does not carry. This port takes the `Decimal` reading.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    let d = match val {
        // str(int) never contains ".", so parts == [digits] and right == 0.
        CurrencyValue::Int(v) => return Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => d.abs(),
    };

    // value == digits * 10^-scale
    let (digits, scale) = d.as_bigint_and_exponent();

    if scale < 0 {
        // str(d) would be "1e+16"-shaped and int() on it raises. Python's
        // message quotes the exact offending literal, which is unrecoverable
        // from the parsed value — the type is what callers observe.
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

pub struct LangKi {
    /// `setup`: `self.exclude_title`. Inert in practice — see the module docs
    /// — but transcribed so the port carries everything `setup` assigns.
    exclude_title: Vec<String>,
    /// `Num2Word_KI.CURRENCY_FORMS`. Declared on the class itself, *not*
    /// inherited: KI subclasses `Num2Word_Base` (whose table is an empty dict)
    /// and never touches `Num2Word_EUR`, so English's import-time mutation of
    /// the shared EUR dict cannot reach it — no "euros"-style leakage, and none
    /// of EN's ~24 injected codes. Confirmed against the live interpreter:
    /// `CONVERTER_CLASSES["ki"].CURRENCY_FORMS` has exactly these three codes.
    ///
    /// Built once here rather than per call; `to_currency` and `to_cheque` only
    /// ever read it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangKi {
    pub fn new() -> Self {
        // ("senti", "senti") is shared by all three entries in the Python
        // literal. Every KI form is a 2-tuple of identical strings.
        const SENTI: [&str; 2] = ["senti", "senti"];

        let mut currency_forms = HashMap::with_capacity(3);
        // Insertion order is irrelevant to a HashMap; the "first value"
        // fallback Python relies on is pinned by FALLBACK_CODE instead.
        currency_forms.insert(
            "KES",
            CurrencyForms::new(&["shilingi", "shilingi"], &SENTI),
        );
        currency_forms.insert("USD", CurrencyForms::new(&["dora", "dora"], &SENTI));
        currency_forms.insert("EUR", CurrencyForms::new(&["yuro", "yuro"], &SENTI));

        LangKi {
            exclude_title: vec!["na".to_string(), "ona".to_string(), "njuru".to_string()],
            currency_forms,
        }
    }
}

impl Default for LangKi {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of `Num2Word_KI._int_to_word`.
///
/// `number` is always non-negative: every caller in scope (`to_cardinal`) has
/// already stripped the sign. The band ceilings guarantee that each index into
/// `ONES`/`TENS` is in `0..=9`, so the `to_usize` narrowing below is proven
/// safe — the `BigInt` itself is never narrowed.
///
/// Values `>= 1_000_000_000` return their own decimal digits verbatim; see the
/// module docs, quirk 1.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    let ten = BigInt::from(10u32);
    let hundred = BigInt::from(100u32);
    let thousand = BigInt::from(1_000u32);
    let million = BigInt::from(1_000_000u32);
    let billion = BigInt::from(1_000_000_000u32);

    // if number < 10: return self.ones[number]
    if number < &ten {
        return ONES[number.to_usize().expect("0..9 by band guard")].to_string();
    }

    // if number < 100: t, o = divmod(number, 10)
    //     return self.tens[t] + (" na " + self.ones[o] if o else "")
    if number < &hundred {
        let (t, o) = number.div_rem(&ten);
        let mut s = TENS[t.to_usize().expect("1..9 by band guard")].to_string();
        if !o.is_zero() {
            s.push_str(NA);
            s.push_str(ONES[o.to_usize().expect("1..9 by divmod")]);
        }
        return s;
    }

    // if number < 1000: h, r = divmod(number, 100)
    //     base = (self.ones[h] + " " if h > 1 else "") + self.hundred
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if number < &thousand {
        let (h, r) = number.div_rem(&hundred);
        let mut s = String::new();
        if h > BigInt::one() {
            s.push_str(ONES[h.to_usize().expect("2..9 by band guard")]);
            s.push(' ');
        }
        s.push_str(HUNDRED);
        if !r.is_zero() {
            s.push_str(NA);
            s.push_str(&int_to_word(&r));
        }
        return s;
    }

    // if number < 1000000: t, r = divmod(number, 1000)
    //     base = (self._int_to_word(t) + " " if t > 1 else "") + self.thousand
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if number < &million {
        let (t, r) = number.div_rem(&thousand);
        let mut s = String::new();
        if t > BigInt::one() {
            s.push_str(&int_to_word(&t));
            s.push(' ');
        }
        s.push_str(THOUSAND);
        if !r.is_zero() {
            s.push_str(NA);
            s.push_str(&int_to_word(&r));
        }
        return s;
    }

    // if number < 1000000000: m, r = divmod(number, 1000000)
    //     base = (self._int_to_word(m) + " " if m > 1 else "") + self.million
    //     return base + (" na " + self._int_to_word(r) if r else "")
    if number < &billion {
        let (m, r) = number.div_rem(&million);
        let mut s = String::new();
        if m > BigInt::one() {
            s.push_str(&int_to_word(&m));
            s.push(' ');
        }
        s.push_str(MILLION);
        if !r.is_zero() {
            s.push_str(NA);
            s.push_str(&int_to_word(&r));
        }
        return s;
    }

    // return str(number)  -- quirk 1: no words at all from 10**9 up.
    number.to_string()
}

/// CPython's `repr(float)` / `str(float)`, which is what KI's `to_cardinal`
/// runs on: `n = str(number).strip()`. Reproduced here because the float branch
/// of KI is a *string* algorithm — it splits on ".", feeds `int()` the pieces
/// and speaks the fraction digit by digit — so the exact repr, not the numeric
/// value, drives the output (and the ValueErrors).
///
/// # 1. The digits
///
/// `{:e}` is Rust's shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the
/// significant digits and the decimal-point position fall straight out. A rare
/// tie can leave `{:e}`'s final digit one off the value CPython's dtoa would
/// pick; re-formatting with `{:.*}` at the known digit count repairs it. This
/// exact function is differentially tested against CPython on 300k doubles in
/// the sibling ports (`lang_bm`): 0 mismatches with the repair, 678 without.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`
/// (`format_float_short`, format code `'r'`), pads the exponent to two digits,
/// and appends `.0` to anything that would otherwise look like an integer.
/// Rust's `{}` does none of this, so both `1e16` and `1.0` would come out
/// wrong in opposite directions. Both matter to KI: `str(1.0)` is `"1.0"`
/// → `"ĩmwe ona wĩra"`, and `str(1e16)` is `"1e+16"` → `int("1e+16")` raises
/// `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this: for an exponent-form repr it is the *exponent*
/// (`abs(Decimal(str(v)).as_tuple().exponent)`), not a digit count — `1e16`
/// arrives with `precision == 16`.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). KI feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and KI renders that
    // "njuru wĩra ona wĩra".
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

/// CPython's `str(Decimal)` — the spec's to-scientific-string, transcribed from
/// `_pydecimal.Decimal.__str__`. `BigDecimal` is the same `(unscaled, scale)`
/// pair as Python's `(_int, _exp)` with `_exp == -scale`, and `from_str`
/// preserves the scale as written, so `Decimal("1.10")`'s trailing zero
/// survives the crossing and KI speaks it (`"ĩmwe ona ĩmwe wĩra"`).
///
/// Reads `as_bigint_and_exponent()` rather than `BigDecimal`'s own `Display`,
/// which is *not* `str(Decimal)`: it renders `Decimal("0.00")` as `"0"`, losing
/// the two digits KI would speak. The capital `E` and the unpadded exponent are
/// Python's too — `str` gives `"1E-7"` where a *float* reprs as `"1e-07"`.
///
/// Known hole: `BigInt` has no negative zero, so `BigDecimal::from_str("-0.0")`
/// discards the sign before this sees it, and the leading `njuru` is lost for
/// `Decimal("-0")`/`Decimal("-0.0")`/… . The discriminator is the original
/// string, which the `FloatValue::Decimal` boundary does not carry — the same
/// shape of hole `split_currency` documents for `1e-05`. The *float* `-0.0` is
/// unaffected, since f64 keeps its sign bit.
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

/// `str(number)` for whatever the Python dispatcher handed the converter. The
/// `FloatValue` split is exactly Python's `isinstance(value, Decimal)`: the two
/// arms stringify by different rules and must not be collapsed.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Python's `int(s)`, for the strings `str()` can produce. `BigInt::from_str`
/// and `int()` agree on everything reachable here — plain ASCII digit runs with
/// an optional sign — and the message quotes the offending literal verbatim, as
/// Python's does.
fn python_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>().map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Port of `Num2Word_KI.to_cardinal` driven by the *string*, i.e. the float /
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
///         ret += " " + (self.ones[int(digit)] or "wĩra")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// The integer [`LangTrait::to_cardinal`] models the same method on the *value*,
/// which is equivalent there (stripping a leading "-" from `str(int)` is just
/// `abs`). Once a "." can appear that equivalence breaks, so this follows Python
/// literally. Three details that look like slips and are not:
///
/// * The recursion passes a **string** to `to_cardinal`, whose first act is
///   `str(number)` — a no-op on a `str` — so a negative float re-enters here and
///   keeps the "." branch, rather than routing through the `BigInt` overload.
/// * `int(left)` runs before the digit loop, so a bad left (`"1e+16"`) raises
///   first; the whole literal is quoted.
/// * `int(digit)` is per **character**, so a malformed fraction (`"5e-05"`)
///   quotes a single char (`'e'`), where a malformed whole quotes the literal.
///   The `or "wĩra"` fires *after* a successful `int()`, only for a `0` digit,
///   because `self.ones[0]` is `""`.
fn cardinal_from_str(n: &str) -> Result<String> {
    // Python's str.strip(); str()'s output never has surrounding space, so this
    // only ever no-ops. Reproduced rather than assumed away.
    let n = n.trim();

    if let Some(rest) = n.strip_prefix('-') {
        let inner = cardinal_from_str(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }

    if let Some((left, right)) = n.split_once('.') {
        // `int(left)` runs before the loop, so a bad left raises first.
        let mut ret = format!("{} {}", int_to_word(&python_int(left)?), POINTWORD);
        for ch in right.chars() {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    ch
                ))
            })? as usize;
            ret.push(' ');
            // `self.ones[int(digit)] or "wĩra"`: ONES[0] is "" (falsy), so a 0
            // fraction digit speaks the zero-word; every other digit is ONES[d].
            ret.push_str(if ONES[d].is_empty() { ZERO_WORD } else { ONES[d] });
        }
        // Python's `ret.strip()` — a no-op whenever `right` is non-empty.
        return Ok(ret.trim().to_string());
    }

    Ok(int_to_word(&python_int(n)?))
}

impl LangTrait for LangKi {

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

    /// `to_ordinal(float/Decimal)` — Python's `to_ordinal` is
    /// `"wa " + to_cardinal(number)` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "wa ithano ona wĩra".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("wa {}", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `"wa " + str(number)`. `repr_str` is the
    /// dispatcher's exact `str(value)` (float repr / `Decimal.__str__`), so
    /// trailing zeros and `1E+2`-style exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("wa {}", repr_str))
    }

    /// `converter.str_to_number` — the base `Decimal(value)` parse, except the
    /// Infinity sentinel becomes the ValueError this language's own
    /// `to_cardinal` raises (`int("Infinity")` after the `"." in n` test
    /// fails); the shared dispatcher would otherwise report Base's
    /// OverflowError. NaN keeps the base sentinel: the dispatcher's
    /// ValueError for it already matches `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "KES"
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

    fn exclude_title(&self) -> &[String] {
        // Dead in every in-scope path: `is_title` is false and KI's
        // `to_cardinal` override never routes through `title()`.
        &self.exclude_title
    }

    /// Port of `Num2Word_KI.to_cardinal`.
    ///
    /// Python works on `str(number).strip()` and branches on a leading "-",
    /// recursing on the tail. For integral input that is exactly: emit
    /// `negword + to_cardinal(abs(n))` and `.strip()` the result. The `"." in n`
    /// float branch lives in [`cardinal_from_str`], reached via
    /// [`LangTrait::to_cardinal_float`].
    ///
    /// The `.strip()` is preserved (as `trim`) even though it is a no-op here:
    /// `negword` ends in a space and `_int_to_word` never produces a leading or
    /// trailing one, so "njuru " + "ĩmwe" is already tight.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = int_to_word(&value.abs());
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(int_to_word(value))
    }

    /// Port of `Num2Word_KI.to_ordinal`: `"wa " + self.to_cardinal(number)`.
    ///
    /// Note it delegates through `to_cardinal`, so the sign handling and the
    /// bare-digits fallback both flow through. No `verify_ordinal` call — see
    /// module docs, quirk 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", WA, self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_KI.to_ordinal_num`: `"wa " + str(number)`.
    ///
    /// The sign is kept verbatim: `to_ordinal_num(-1)` == "wa -1".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", WA, value))
    }

    /// Port of `Num2Word_KI.to_year`: a bare alias for `to_cardinal`; the
    /// `longval` kwarg is accepted and ignored. See module docs, quirk 4.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / Decimal cardinal path.
    ///
    /// KI reaches non-integers through its own `to_cardinal` override — which
    /// works on `str(number)` — **not** through `Num2Word_Base.to_cardinal_float`.
    /// So this hook does *not* delegate to `floatpath::default_to_cardinal_float`
    /// (that mirrors the base method KI never runs): there is no `float2tuple`
    /// and no rounding of any kind here. It reconstructs `str(number)` exactly as
    /// CPython would and drives KI's string algorithm — see [`cardinal_from_str`].
    ///
    /// Consequences the base path would get wrong, all matching the live
    /// interpreter:
    ///   * `1e16`, `1e-5`, `Decimal("1E+2")` → `int()` on an exponent-form repr
    ///     raises `ValueError`, rather than rendering.
    ///   * `-0.0` → `str` is `"-0.0"`, so KI prepends `njuru` even though
    ///     `-0.0 < 0.0` is false.
    ///
    /// `precision_override` is the `precision=` kwarg. `__init__.py` applies it by
    /// assigning `converter.precision`, which KI never reads — so it is accepted
    /// and **ignored**, matching the live interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        cardinal_from_str(&python_str(value))
    }

    // ---- currency -------------------------------------------------------
    //
    // KI overrides `to_currency` and `pluralize` and nothing else, so only
    // those two plus the data tables appear here. `to_cheque`,
    // `_money_verbose` (→ `to_cardinal`), `_cents_verbose`, `_cents_terse`,
    // `CURRENCY_PRECISION` (`{}` → 100) and `CURRENCY_ADJECTIVES` (`{}` → no
    // adjective) all come from `Num2Word_Base` unchanged, which is exactly
    // what the trait defaults already are. `cardinal_from_decimal` stays at
    // its default too: `default_to_currency` is the only thing that reaches
    // the fractional-cents branch, and KI never routes through it.

    fn lang_name(&self) -> &str {
        "Num2Word_KI"
    }

    /// Backs the inherited `to_cheque` only.
    ///
    /// `to_cheque` does `self.CURRENCY_FORMS[currency]` and converts the
    /// `KeyError` into `NotImplementedError`, so `None` here is the right
    /// answer for an unknown code. `to_currency` deliberately does *not* go
    /// through this hook — it needs the KES fallback instead of a raise.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_KI.pluralize`:
    ///
    /// ```text
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Unreachable in practice — module docs, Currency quirk 4 — but ported
    /// because the class defines it and the trait default raises. It guards the empty
    /// tuple and takes `forms[-1]`, so unlike `Num2Word_EUR.pluralize` (a bare
    /// `forms[0 if n == 1 else 1]`) it cannot raise `IndexError` at any arity.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms.last() {
            None => Ok(String::new()),
            Some(last) => Ok(if n.is_one() {
                forms[0].clone()
            } else {
                last.clone()
            }),
        }
    }

    /// Port of `Num2Word_KI.to_currency`.
    ///
    /// A wholesale override: `default_to_currency` is never reached. `adjective`
    /// is accepted and ignored, exactly as in Python — KI's
    /// `CURRENCY_ADJECTIVES` is base's empty dict, so even the inherited path
    /// would be a no-op.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `None` is the caller omitting `separator=`; resolve it through this
        // language's own default (" ") before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is evaluated before `val = abs(val)`.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // `.get(currency, list(...values())[0])` — an unknown code silently
        // borrows KES's forms instead of raising. See FALLBACK_CODE.
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CODE))
            .expect("FALLBACK_CODE is inserted by new()");

        let one = BigInt::one();
        // cr1[1] if left != 1 else cr1[0] — a direct index, not pluralize().
        let unit = if left != one {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };
        let mut result = format!("{} {}", int_to_word(&left), unit);

        // `if cents and right:` — module docs, Currency quirk 3.
        if cents && !right.is_zero() {
            let subunit = if right != one {
                &forms.subunit[1]
            } else {
                &forms.subunit[0]
            };
            // Python concatenates the separator raw, so an explicit
            // `separator=","` renders "yuro,mĩrongo ĩtatũ na inya senti" with
            // no space of its own.
            result.push_str(&format!(
                "{}{} {}",
                separator,
                int_to_word(&right),
                subunit
            ));
        }

        // negword carries its own trailing space, so this is "njuru ikũmi …".
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        // `.strip()`: a no-op on every reachable input, kept for fidelity.
        Ok(result.trim().to_string())
    }
}
