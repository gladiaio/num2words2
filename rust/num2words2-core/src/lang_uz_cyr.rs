//! Port of `lang_UZ_CYR.py` (Uzbek, Cyrillic script).
//!
//! Registry: `__init__.py` maps **both** `"uz_Cyrl"` and `"uz_cyr"` to
//! `Num2Word_UZ_CYRILLIC`, so this file is what the `"uz_cyr"` key resolves to.
//! (`"uz"` is a different class, `lang_UZ.Num2Word_UZ` — not ported here.)
//!
//! Shape: **self-contained**. `Num2Word_UZ_CYRILLIC` subclasses
//! `Num2Word_Base` but defines no `high_numwords`/`mid_numwords`/
//! `low_numwords`, so Python never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives `_int2word` over
//! 3-digit chunks. `cards`/`merge` therefore stay at their trait defaults.
//! The `THOUSANDS` table (keys 1..=10) ends at 10^30; Python raised
//! `KeyError` from 10^33 upward. Fixed (gladiaio/num2words2#159): `maxval()`
//! is 10^33 and `_int2word` raises `OverflowError` for `abs(n) >= 10**33`.
//!
//! Inherited from `Num2Word_Base` (unchanged by UZ_CYR, so the trait defaults
//! do the right thing):
//!   * `to_ordinal_num(value) -> value` → default `Ok(value.to_string())`
//!   * `to_year(value) -> self.to_cardinal(value)` → default delegates through
//!     `&self`, picking up the `to_cardinal` override below.
//!   * `title(value)` → `setup()` never touches `is_title`, so it stays
//!     `False` and `title` is the identity. `to_ordinal` calls it anyway.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. Everything below is wrong-looking but is
//! exactly what Python emits, verified against the frozen corpus:
//!
//! 1. **`ZERO` is "нол" but `ords` keys zero as "ноль"** (with a soft sign).
//!    The lookup therefore always misses, the `except KeyError` fallback finds
//!    no matching suffix rule, and `to_ordinal(0)` == "нол" — the *cardinal*,
//!    with no ordinal suffix at all. `ords["ноль"]` is dead code.
//! 2. **"миллиард" gets no ordinal suffix.** It ends in "д" with "р" before
//!    it, which matches none of the `elif` arms in the fallback, so `lastword`
//!    falls through unchanged: `to_ordinal(10**9)` == "миллиард",
//!    `to_ordinal(10**10)` == "ўн миллиард". Every *other* scale word ends in
//!    "н" and does get "инчи" ("миллионинчи", "триллионинчи", ...). Preserved
//!    verbatim — see [`LangUzCyr::ordinal_fallback`].
//! 3. **`HUNDREDS` values contain a space** ("бир юз", "икки юз", ...), but
//!    `to_ordinal` does `to_cardinal(number).split(" ")`. A hundreds word
//!    therefore splits into *two* tokens, which silently shifts what
//!    `outwords[-2]` / `outwords[-3]` and the `len(outwords) == 3` test see.
//!    This is load-bearing for the observed output and is reproduced by
//!    splitting on `' '` here too.
//! 4. **Blanking an interior "бир" (fixed, #160).** Python sets
//!    `outwords[-2]` to `""` whenever it is "бир", then re-joins with `" "`, so
//!    an emptied *interior* slot yields two adjacent spaces *and* loses a
//!    digit: `to_ordinal(1100)` == "бир минг  юзинчи", `to_ordinal(21000)` ==
//!    "йигирма  мингинчи" (i.e. 20000th). The port only drops a *leading* "бир"
//!    (`to_ordinal(100)` == "юзинчи", 1000 == "мингинчи", as in Python) and
//!    keeps an interior one, matching the cardinal: 1100 == "бир минг бир
//!    юзинчи", 21000 == "йигирма бир мингинчи".
//! 5. **`ords_feminine` has a duplicate key.** The literal is
//!    `{"бир": "", "бир": "", "икки": "икки", ...}` — Python keeps the last
//!    binding, and both are `""`, so "бир" → `""`. That empty mapping is what
//!    drives quirk 4. Every other key maps to itself, making the dict a
//!    no-op set-membership test in practice.
//! 6. **`ONES` and `ONES_FEMININE` are byte-identical tables.** The
//!    `feminine` parameter and the `i == 1` selector in `_int2word` therefore
//!    cannot change the output. Both tables are kept so the selector logic
//!    ports 1:1.
//! 7. **All `THOUSANDS` plural triples hold three identical strings**, so
//!    `pluralize`'s form choice is unobservable. Ported faithfully anyway.
//! 8. **`lastword[:-3] in ords_feminine` is dead code.** No single token
//!    `_int2word` can emit has an `ords_feminine` key as its head with exactly
//!    three trailing characters (checked exhaustively against the token set:
//!    нол, ўн, юз, минг, миллион, миллиард, триллион, квадриллион,
//!    квинтиллион, секстиллион, септиллион, октиллион, нониллион, йигирма,
//!    ўттиз, қирқ, эллик, олтмиш, етмиш, саксон, тўқсон and the ONES words).
//!    The "юзинчи" arm can never fire. Ported anyway.
//! 9. **`lastword[-5:] == "ўн"` is a convoluted `== "ўн"`.** A 5-char suffix
//!    can only equal a 2-char string when the whole string is that string.
//!    Ported literally.
//! 10. **The "ш" arm re-tests the string it just mutated.** Python runs two
//!    consecutive `if`s (not `elif`), and the second reads the *rebound*
//!    `lastword`. Because the first arm's replacement ends in "и", the second
//!    can never fire after it — but the sequencing is reproduced exactly.
//!
//! # Currency
//!
//! `CURRENCY_FORMS` covers **only** UZS, EUR and USD, all with three plural
//! forms per side. `pluralize` (the Russian-style 3-form rule) and
//! `_cents_verbose` are the only other currency members the class defines;
//! `to_currency` / `to_cheque` / `_money_verbose` / `_cents_terse` are all
//! `Num2Word_Base`'s. `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both
//! `{}`, so every currency is 2-decimal and `adjective=True` is a no-op.
//!
//! Note this class descends from `Num2Word_Base`, **not** `Num2Word_EUR`, so the
//! shared-class-dict mutation that `Num2Word_EN.__init__` performs cannot reach
//! it — see [`build_currency_forms`].
//!
//! ## Float / Decimal cardinal path (float phase)
//!
//! UZ_CYR reaches non-integer input through its **`to_cardinal` override**, not
//! `Num2Word_Base.to_cardinal_float` (which it does not override). That override
//! does `str(number).replace(',', '.')`, splits on `'.'`, and renders BOTH
//! halves as whole numbers via `_int2word(int(...))`.
//!
//! The digits therefore come from **`str(number)`** — Python's shortest-
//! round-trip repr — and *never* from `base.float2tuple`'s binary f64
//! arithmetic. The two are NOT equivalent: at high magnitude × precision,
//! `abs(value - pre) * 10**precision` drifts more than the `< 0.01` rescue
//! window from the repr digits and gets floored one short. Pure Python for
//! `732685275.4516093` says "…тўқсон уч" (post = `int("4516093")`), while
//! `float2tuple` computes `4516092.999…` → outside the window → floor →
//! `4516092`. An earlier revision of this port reused `float2tuple` on the
//! strength of a small-magnitude fuzz claiming byte-equality; a wider fuzz
//! (magnitudes to 1e16) falsified it within 30 samples. So
//! [`LangUzCyr::to_cardinal_float`] reconstructs `str(number)` exactly —
//! [`py_float_repr`] for floats (shortest digits, the `.0` suffix, Python's
//! scientific-notation thresholds at `|v| >= 1e16` / `0 < |v| < 1e-4`) and
//! [`py_decimal_str`] for Decimals (`_pydecimal.__str__`'s plain/`E±n`
//! selection) — and splits on `'.'` like the original.
//!
//! Scientific reprs are load-bearing *errors*: `str(1e-05)` is `'1e-05'`
//! (no `'.'`), so Python's else-branch does `int('1e-05')` → ValueError;
//! `str(1.5e-05)` is `'1.5e-05'`, so `int('5e-05')` → ValueError; same for
//! `Decimal('1E-7')`/`Decimal('1E+21')`. Confirmed on the live interpreter
//! (with the Rust fast path disabled — the shim otherwise answers for it).
//!
//! ### Fractional cents (`cardinal_from_decimal`) — RESOLVED here
//!
//! Base's `to_currency` reaches its fractional-cents branch when
//! `(value * 100) % 1 != 0` and renders the sub-unit as
//! `self.to_cardinal(float(right))` (base.py line 476). For most languages that
//! resolves to `Num2Word_Base.to_cardinal_float` (digit by digit), which the
//! trait default mirrors — but UZ_CYR **overrides `to_cardinal`**, so pure
//! Python dispatches to its own whole-number string-splitting version:
//!
//! * trait default (digit by digit): `0.55` → "нол вергул беш беш"
//! * `Num2Word_UZ_CYRILLIC` (one number): `0.55` → "нол вергул эллик беш" ← Python
//!
//! [`LangUzCyr::cardinal_from_decimal`] now routes through this language's own
//! `to_cardinal_float`, matching pure Python. No corpus row reaches it (every
//! uz_cyr currency value is ≤ 2 dp, so `has_fractional_cents` never fires);
//! confirmed against the live interpreter.
//!
//! ## Ordinal float/Decimal path
//!
//! `to_ordinal(number)` is one method for every input type: `verify_ordinal`
//! (numeric, so whole floats and `-0.0` pass; fractions raise the *float*
//! TypeError, negative wholes the *negative* one), then the token machinery
//! over `self.to_cardinal(number)` — which for a float is the str-splitting
//! version above. The rewrites therefore fire on float cardinals too:
//! `to_ordinal(1.0)` → cardinal "бир вергул нол" → the `len == 3` rule blanks
//! "бир" → **"вергул нол"** (corpus-pinned), while `to_ordinal(5.0)` comes
//! back as the unchanged cardinal "беш вергул нол" ("нол" matches nothing —
//! quirk 1 again). A scientific repr raises its `int()` ValueError *after*
//! verify passes: `to_ordinal(Decimal("1E+2"))` == ValueError. All in
//! [`Lang::ordinal_float_entry`] via the shared [`LangUzCyr::ordinalize`].
//!
//! ## String "Infinity"
//!
//! `str_to_number` (Base's `Decimal(value)`) parses "Infinity" fine; the
//! ValueError comes later, from `to_cardinal`'s `int("Infinity")`. The
//! binding's default for an infinity parse is Base's OverflowError
//! (`int(Decimal('Infinity'))`), so [`Lang::str_to_number`] intercepts it and
//! raises the ValueError early — see the caveats at the override site.
//!
//! # Error variants
//!
//! * Any currency code outside {UZS, EUR, USD} → `NotImplementedError`
//!   (`N2WError::NotImplemented`), message
//!   `Currency code "X" not implemented for "Num2Word_UZ_CYRILLIC"`. Raised by
//!   both `to_currency` and `to_cheque`. Corpus-confirmed for GBP, JPY, KWD,
//!   BHD, INR, CNY and CHF.
//! * `to_ordinal(n)` for negative `n` → `TypeError` via the inherited
//!   `verify_ordinal` (`N2WError::Type`). Corpus-confirmed for -1, -7, -21,
//!   -42, -100, -999, -1000, -1000000 — and for negative whole floats
//!   (-3.0, -1000000.0) and fractional values (0.5, -1.5) through the float
//!   entry.
//! * `abs(n) >= 10**33` → `OverflowError` (#159). Python crashed with a
//!   `KeyError` on `THOUSANDS[i]` there; that `N2WError::Key` arm is now
//!   unreachable from the public entry points.
//! * `to_cardinal` accepts negatives fine (`_int2word` strips the sign and
//!   recurses), and `to_ordinal_num` never inspects the value.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `ZERO = ('нол',)` — note: *not* the "ноль" that `ords` keys. See quirk 1.
const ZERO: &str = "нол";

/// `self.negword`, set in `setup()`.
const NEGWORD: &str = "минус";

/// `self.pointword`, set in `setup()`. Separates the two halves on the
/// float/Decimal path — see [`LangUzCyr::to_cardinal_float`].
const POINTWORD: &str = "вергул";

/// `ONES`, keys 1..=9. Index 0 is absent in Python (guarded by `n1 > 0`).
const ONES: [&str; 10] = [
    "", "бир", "икки", "уч", "тўрт", "беш", "олти", "етти", "саккиз", "тўққиз",
];

/// `ONES_FEMININE`, keys 1..=9 — byte-identical to `ONES` upstream (quirk 6).
const ONES_FEMININE: [&str; 10] = [
    "", "бир", "икки", "уч", "тўрт", "беш", "олти", "етти", "саккиз", "тўққиз",
];

/// `TENS`, keys 0..=9 — the 10..=19 teens, keyed by the *units* digit.
const TENS: [&str; 10] = [
    "ўн",
    "ўн бир",
    "ўн икки",
    "ўн уч",
    "ўн тўрт",
    "ўн беш",
    "ўн олти",
    "ўн етти",
    "ўн саккиз",
    "ўн тўққиз",
];

/// `TWENTIES`, keys 2..=9. Indices 0/1 are absent in Python (guarded `n2 > 1`).
const TWENTIES: [&str; 10] = [
    "",
    "",
    "йигирма",
    "ўттиз",
    "қирқ",
    "эллик",
    "олтмиш",
    "етмиш",
    "саксон",
    "тўқсон",
];

/// `HUNDREDS`, keys 1..=9. **Each value contains a space** — see quirk 3.
const HUNDREDS: [&str; 10] = [
    "",
    "бир юз",
    "икки юз",
    "уч юз",
    "тўрт юз",
    "беш юз",
    "олти юз",
    "етти юз",
    "саккиз юз",
    "тўққиз юз",
];

/// `Num2Word_UZ_CYRILLIC.CURRENCY_FORMS`, verbatim from the class body.
///
/// **The `lang_EUR` shared-dict trap does not apply here.** That trap bites
/// classes descending from `Num2Word_EUR`, whose `CURRENCY_FORMS` class dict
/// `Num2Word_EN.__init__` mutates in place at import time. `Num2Word_UZ_CYRILLIC`
/// subclasses `Num2Word_Base` directly (MRO is exactly
/// `[Num2Word_UZ_CYRILLIC, Num2Word_Base, object]`) and declares its **own**
/// `CURRENCY_FORMS` in its class body, so nothing upstream can reach it.
/// Confirmed against the live interpreter, not read off the source: the runtime
/// dict holds these three codes and nothing else.
///
/// Consequently every other code — GBP, JPY, KWD, BHD, INR, CNY, CHF — raises
/// `NotImplementedError`, which is exactly what the corpus expects (84 of this
/// language's 108 currency rows are that error).
///
/// Arity is load-bearing: **three** forms on both sides of every entry, because
/// `pluralize` can return index 2. Dropping the third form would turn every
/// `form == 2` case (e.g. 0, 12, 99 units) into an IndexError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    const TSENT: [&str; 3] = ["цент", "цент", "цент"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "UZS",
        CurrencyForms::new(&["сўм", "сўм", "сўм"], &["тийин", "тийин", "тийин"]),
    );
    m.insert("EUR", CurrencyForms::new(&["евро", "евро", "евро"], &TSENT));
    m.insert(
        "USD",
        CurrencyForms::new(&["доллар", "доллар", "доллар"], &TSENT),
    );
    m
}

/// The form index `Num2Word_UZ_CYRILLIC.pluralize` selects.
///
/// ```python
/// if n % 100 < 10 or n % 100 > 20:
///     if n % 10 == 1:        form = 0
///     elif 5 > n % 10 > 1:   form = 1
///     else:                  form = 2
/// else:
///     form = 2
/// ```
///
/// Factored out because Python has **one** `pluralize` serving two callers with
/// different tuple types: `_int2word` passes a `THOUSANDS` triple, while
/// `to_currency` passes a `CURRENCY_FORMS` tuple. Rust's trait method is typed
/// `&[String]` and the scale-word path is typed `&[&'static str; 3]`, so the two
/// call sites cannot share a signature — but they must not drift apart on the
/// rule itself, so the rule lives here exactly once.
///
/// `mod_floor` rather than Rust's `%` to keep Python's floor semantics. `n` is
/// non-negative at every reachable call site (`_int2word` strips the sign;
/// `to_currency` passes `abs(val_int)` or a parsed magnitude), so the two agree
/// in practice — but the port should not depend on that.
fn plural_form_index(n: &BigInt) -> usize {
    let m10 = n.mod_floor(&BigInt::from(10));
    let m100 = n.mod_floor(&BigInt::from(100));
    if m100 < BigInt::from(10) || m100 > BigInt::from(20) {
        if m10 == BigInt::from(1) {
            0
        } else if m10 < BigInt::from(5) && m10 > BigInt::from(1) {
            1
        } else {
            2
        }
    } else {
        2
    }
}

/// Python's `KeyError`. A crash site, not a deliberate raise; the
/// `THOUSANDS` miss is unreachable since the 10^33 MAXVAL check (#159).
fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

/// Python's `int(s)` for the digit-string shapes this module produces.
///
/// Unreachable failure in practice: `splitbyx` is only ever fed
/// `abs(n).to_string()` (the sign is stripped by `_int2word` before the call,
/// and `to_ordinal` rejects negatives outright), so every chunk parses. The
/// `ValueError` arm exists only to keep the port total.
fn parse_int(s: &str) -> Result<BigInt> {
    BigInt::parse_bytes(s.as_bytes(), 10)
        .ok_or_else(|| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `str(float)` / `repr(float)` — shortest round-trip digits.
///
/// Rust's `{}` Display shares CPython repr's shortest-round-trip contract,
/// but byte fidelity needs two corrections:
///
/// 1. **Presentation.** Display never appends `.0` to a whole float and
///    never switches to exponent form. CPython does both: it renders
///    `d.ddd` exponent-free only while the decimal point lands in
///    `(-4, 16]`, i.e. for `1e-4 <= |v| < 1e16` (and 0.0), and always shows
///    a fraction. Both boundaries are exact in f64: `1e16` is exactly
///    representable, and the literal `1e-4` is the nearest double *above*
///    decimal 1e-4, so `a < 1e-4` matches CPython's decision bit for bit.
/// 2. **Tie-breaks.** When a double sits exactly midway between the two
///    shortest candidates (e.g. `…891554.25` at 1e15 scale, where ulp is
///    0.25), CPython's dtoa picks the even last digit (`…891554.2`) while
///    Rust's shortest algorithm picks the other (`…891554.3`). The digit
///    *count* is objective — minimal round-trip length — so it is taken
///    from the shortest form, and the digits are then re-rendered with
///    fixed-precision `{:.p}` / `{:.pe}` formatting, which is correctly
///    rounded with ties-to-even, matching CPython. A 128-case fuzz
///    divergence collapsed to zero under this scheme.
///
/// The scientific arm matters only as an *error source* here — its output
/// feeds `int()` which raises ValueError — but the message quotes the
/// literal, so the format is reproduced exactly: shortest mantissa (no
/// trailing `.0`), `e`, forced sign, exponent zero-padded to two digits
/// (`1e+16`, `1.5e-05`, `1e+300`).
fn py_float_repr(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string(); // repr(float('nan')) — sign is dropped
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Digit count from the shortest form, digits from ties-even fixed
        // exponential rendering (see doc point 2).
        let s = format!("{:e}", f); // shortest, e.g. "1e16", "1.5e-5"
        let (mant, _) = s.split_once('e').expect("LowerExp always emits 'e'");
        let p = mant.split_once('.').map_or(0, |(_, frac)| frac.len());
        let s = format!("{:.*e}", p, f);
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits 'e'");
        let exp: i32 = exp.parse().expect("LowerExp exponent is an integer");
        let sign = if exp < 0 { '-' } else { '+' };
        return format!("{}e{}{:02}", mant, sign, exp.unsigned_abs());
    }
    // Digit count from the shortest form; `.max(1)` because Python repr
    // always shows a fraction ("1" -> "1.0", "-0" -> "-0.0").
    let p = format!("{}", f)
        .split_once('.')
        .map_or(0, |(_, frac)| frac.len());
    format!("{:.*}", p.max(1), f)
}

/// Python's `str(Decimal)` — the `_pydecimal.Decimal.__str__` algorithm
/// (eng=False) for finite values, driven from the BigDecimal's
/// `(coefficient, scale)` pair, which matches Python's
/// `(digits, exponent=-scale)` tuple digit for digit (`BigDecimal::from_str`
/// preserves trailing zeros: "1.10" stays coefficient 110 / scale 2).
///
/// Plain notation iff `exponent <= 0 and adjusted > -7` (leftdigits > -6);
/// otherwise `dE±n` form — which downstream `int()` then rejects, exactly
/// like Python (`Decimal('1E-7')` → ValueError `'1E-7'`).
///
/// One unreachable infidelity: BigDecimal stores the sign on the BigInt, and
/// BigInt has no negative zero, so a `Decimal('-0.00')`'s sign is already
/// gone by the time it crosses the binding. Harmless: `int('-0') == int('0')`
/// and the plain form is what any reachable zero produces.
fn py_decimal_str(v: &BigDecimal) -> String {
    let (int_val, scale) = v.as_bigint_and_exponent();
    let sign = if int_val.is_negative() { "-" } else { "" };
    let digits = int_val.magnitude().to_string(); // "0" for zero, like Python
    let exp = -scale; // Python's Decimal exponent
    let ndigits = digits.len() as i64;
    let leftdigits = exp + ndigits;
    // dotplace: where the '.' falls inside `digits` (eng=False branch).
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };
    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(dotplace.unsigned_abs() as usize), digits),
        )
    } else if dotplace >= ndigits {
        (
            // digits are ASCII, byte ops are safe.
            format!("{}{}", digits, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        (
            digits[..dotplace as usize].to_string(),
            format!(".{}", &digits[dotplace as usize..]),
        )
    };
    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace) // Python's "%+d"
    };
    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
fn splitbyx(n: &str, x: usize) -> Result<Vec<BigInt>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let slice = |i: usize, j: usize| -> String { chars[i..j.min(length)].iter().collect() };

    let mut out: Vec<BigInt> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse_int(&slice(0, start))?);
        }
        let mut i = start;
        while i < length {
            out.push(parse_int(&slice(i, i + x))?);
            i += x;
        }
    } else {
        out.push(parse_int(n)?);
    }
    Ok(out)
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `[n1, n2, n3]`
/// (units, tens, hundreds).
///
/// Only ever called with a 3-digit chunk in `0..=999`, so `"%03d"` yields
/// exactly three characters and the `[-3:]` slice is a no-op.
fn get_digits(n: &BigInt) -> [usize; 3] {
    let s = format!("{:0>3}", n.to_string());
    let chars: Vec<char> = s.chars().collect();
    let tail = &chars[chars.len() - 3..];
    let mut a = [0usize; 3];
    for (k, c) in tail.iter().rev().enumerate() {
        a[k] = c.to_digit(10).unwrap_or(0) as usize;
    }
    a
}

/// Python's `s[-k]`, i.e. the k-th character from the end (k >= 1).
/// Out of range is an `IndexError` in Python; unreachable here because every
/// token `_int2word` emits is at least two characters long.
fn char_from_end(chars: &[char], k: usize) -> Result<char> {
    if chars.len() < k {
        return Err(N2WError::Index("string index out of range".to_string()));
    }
    Ok(chars[chars.len() - k])
}

pub struct LangUzCyr {
    /// `THOUSANDS`: chunk index → the three plural forms. Keys 1..=10, i.e.
    /// up to 1000^10 == 10^30 ("нониллион"). A non-zero chunk at index >= 11
    /// (10^33 and up) is pre-empted by the MAXVAL check (#159).
    thousands: HashMap<usize, [&'static str; 3]>,
    /// `self.ords`, set in `setup()`. Note the "ноль" key, which `ZERO`
    /// ("нол") can never match — quirk 1.
    ords: HashMap<&'static str, &'static str>,
    /// `self.ords_feminine`, set in `setup()`. The duplicate "бир" key in the
    /// Python literal collapses to a single `"бир" -> ""` entry — quirk 5.
    ords_feminine: HashMap<&'static str, &'static str>,
    /// `CURRENCY_FORMS`. Built once in `new()`, never per call — this table is
    /// read-only at runtime and rebuilding it on each `to_currency` is what made
    /// an earlier revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangUzCyr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangUzCyr {
    pub fn new() -> Self {
        let mut thousands: HashMap<usize, [&'static str; 3]> = HashMap::new();
        thousands.insert(1, ["минг", "минг", "минг"]); // 10^3
        thousands.insert(2, ["миллион", "миллион", "миллион"]); // 10^6
        thousands.insert(3, ["миллиард", "миллиард", "миллиард"]); // 10^9
        thousands.insert(4, ["триллион", "триллион", "триллион"]); // 10^12
        thousands.insert(5, ["квадриллион", "квадриллион", "квадриллион"]); // 10^15
        thousands.insert(6, ["квинтиллион", "квинтиллион", "квинтиллион"]); // 10^18
        thousands.insert(7, ["секстиллион", "секстиллион", "секстиллион"]); // 10^21
        thousands.insert(8, ["септиллион", "септиллион", "септиллион"]); // 10^24
        thousands.insert(9, ["октиллион", "октиллион", "октиллион"]); // 10^27
        thousands.insert(10, ["нониллион", "нониллион", "нониллион"]); // 10^30

        let mut ords: HashMap<&'static str, &'static str> = HashMap::new();
        ords.insert("ноль", "нолинчи"); // dead: ZERO is "нол" — quirk 1
        ords.insert("бир", "биринчи");
        ords.insert("икки", "иккинчи");
        ords.insert("уч", "учинчи");
        ords.insert("тўрт", "тўртинчи");
        ords.insert("беш", "бешинчи");
        ords.insert("олти", "олтинчи");
        ords.insert("етти", "еттинчи");
        ords.insert("саккиз", "саккизинчи");
        ords.insert("тўққиз", "тўққизинчи");
        ords.insert("юз", "юзинчи");

        let mut ords_feminine: HashMap<&'static str, &'static str> = HashMap::new();
        // Python literal lists "бир" twice, both mapping to "" — last wins,
        // so a single entry is exact.
        ords_feminine.insert("бир", "");
        ords_feminine.insert("икки", "икки");
        ords_feminine.insert("уч", "уч");
        ords_feminine.insert("тўрт", "тўрт");
        ords_feminine.insert("беш", "беш");
        ords_feminine.insert("олти", "олти");
        ords_feminine.insert("етти", "етти");
        ords_feminine.insert("саккиз", "саккиз");
        ords_feminine.insert("тўққиз", "тўққиз");

        LangUzCyr {
            thousands,
            ords,
            ords_feminine,
            currency_forms: build_currency_forms(),
        }
    }

    /// `THOUSANDS[i]`, raising `KeyError` past 10 (unreachable, #159).
    fn thousands_at(&self, i: usize) -> Result<&[&'static str; 3]> {
        self.thousands.get(&i).ok_or_else(|| key_error(i.to_string()))
    }

    /// `Num2Word_UZ_CYRILLIC.pluralize` as reached from `_int2word` — i.e. with
    /// a `THOUSANDS` scale triple. The rule itself lives in
    /// [`plural_form_index`]; see its docs for why it is shared with the
    /// `Lang::pluralize` trait method rather than written twice.
    ///
    /// `n` is a non-negative 3-digit chunk here. Every `THOUSANDS` triple holds
    /// three identical strings (quirk 7), so the form index cannot affect the
    /// output. Kept for fidelity.
    ///
    /// Indexing is infallible: the array type pins the length at 3 and
    /// `plural_form_index` returns 0..=2, so the IndexError that Python's
    /// tuple-indexing could raise is unrepresentable on this path.
    fn pluralize(&self, n: &BigInt, forms: &[&'static str; 3]) -> String {
        forms[plural_form_index(n)].to_string()
    }

    /// Port of `Num2Word_UZ_CYRILLIC._int2word`.
    ///
    /// `feminine` is only ever `true` on the currency path (`_cents_verbose`
    /// with 'UZS'), which is out of scope; `to_cardinal` always passes the
    /// default `false`. It is threaded through anyway so the `ones` selector
    /// ports 1:1 — and since `ONES_FEMININE == ONES` (quirk 6) it is inert.
    fn int2word(&self, n: &BigInt, feminine: bool) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_negative() {
            // Python: ' '.join([self.negword, self._int2word(abs(n))])
            // Note this uses self.negword verbatim, NOT negword.strip().
            return Ok(format!("{} {}", NEGWORD, self.int2word(&n.abs(), false)?));
        }

        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3)?;
        let mut i = chunks.len();
        for x in chunks.iter() {
            i -= 1;

            if x.is_zero() {
                continue;
            }

            let [n1, n2, n3] = get_digits(x);

            if n3 > 0 {
                words.push(HUNDREDS[n3].to_string());
            }

            if n2 > 1 {
                words.push(TWENTIES[n2].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1].to_string());
            } else if n1 > 0 {
                // Python: ONES_FEMININE if i == 1 or feminine and i == 0 else ONES
                // (`and` binds tighter than `or`).
                let ones: &[&str; 10] = if i == 1 || (feminine && i == 0) {
                    &ONES_FEMININE
                } else {
                    &ONES
                };
                words.push(ones[n1].to_string());
            }

            if i > 0 {
                let forms = self.thousands_at(i)?;
                words.push(self.pluralize(x, forms));
            }
        }

        Ok(words.join(" "))
    }

    /// The body of `to_ordinal`'s `except KeyError:` handler — the suffix
    /// rules applied when `self.ords[lastword]` misses.
    ///
    /// Ported arm for arm, including short-circuit order (`lastword[-1] == "а"`
    /// is tested before `lastword[-2] == "м"`, so a one-character string would
    /// never reach the `[-2]` access), the dead "юзинчи" arm (quirk 8), the
    /// `[-5:] == "ўн"` curiosity (quirk 9), and the mutate-then-retest "ш" arm
    /// (quirk 10).
    ///
    /// There is **no `else`**: a word matching none of the arms is returned
    /// unchanged, which is exactly why "нол" and "миллиард" come back without
    /// an ordinal suffix (quirks 1 and 2).
    fn ordinal_fallback(&self, lastword: &str) -> Result<String> {
        let c: Vec<char> = lastword.chars().collect();
        let len = c.len();

        // lastword[:-3]
        let head: String = c[..len.saturating_sub(3)].iter().collect();
        // lastword[-5:]
        let tail5: String = c[len.saturating_sub(5)..].iter().collect();

        if let Some(v) = self.ords_feminine.get(head.as_str()) {
            // Python: self.ords_feminine.get(lastword[:-3], lastword) + "юзинчи"
            // The `in` test above guarantees the get() hits, so the default
            // (`lastword`) is unreachable.
            return Ok(format!("{}юзинчи", v));
        }

        if char_from_end(&c, 1)? == 'а' || char_from_end(&c, 2)? == 'м' {
            return Ok(format!("{}нчи", lastword));
        }
        if char_from_end(&c, 1)? == 'к' {
            return Ok(format!("{}инчи", lastword));
        }
        if char_from_end(&c, 1)? == 'қ' {
            return Ok(format!("{}инчи", lastword));
        }
        if tail5 == "ўн" {
            return Ok(format!("{}инчи", lastword));
        }
        if char_from_end(&c, 2)? == 'ш' || char_from_end(&c, 1)? == 'ш' {
            // Two consecutive `if`s in Python, not `elif` — the second reads
            // the value the first may have just rebound.
            let mut w = lastword.to_string();
            if char_from_end(&c, 2)? == 'ш' {
                let mut t: Vec<char> = w.chars().collect();
                t.pop();
                w = t.into_iter().collect::<String>() + "инчи";
            }
            let wc: Vec<char> = w.chars().collect();
            if char_from_end(&wc, 1)? == 'ш' {
                w = format!("{}инчи", w);
            }
            return Ok(w);
        }
        if char_from_end(&c, 1)? == 'н' || char_from_end(&c, 2)? == 'н' {
            return Ok(format!("{}инчи", lastword));
        }
        if char_from_end(&c, 1)? == 'з' || char_from_end(&c, 2)? == 'з' {
            return Ok(format!("{}инчи", lastword));
        }

        // No else in Python — fall through unchanged.
        Ok(lastword.to_string())
    }

    /// The body of `to_ordinal` *after* `verify_ordinal` and `to_cardinal`:
    /// the token rewrites and the `ords`-or-fallback suffixing, operating on
    /// the cardinal string exactly as Python does. Shared by the integer
    /// [`Lang::to_ordinal`] and the float [`Lang::ordinal_float_entry`] —
    /// Python has a single `to_ordinal(number)` whose `self.to_cardinal(
    /// number)` output is what this machinery sees, whether `number` was an
    /// int ("беш"), a whole float ("беш вергул нол") or `Decimal("100")`
    /// ("бир юз").
    fn ordinalize(&self, cardinal: &str) -> Result<String> {
        // Python's str.split(" ") — a single-space separator that keeps empty
        // fields, NOT whitespace-splitting. HUNDREDS values embed a space, so
        // "бир юз" arrives here as two tokens (quirk 3).
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        let n = outwords.len();
        // to_cardinal never returns "", so outwords is never empty and the
        // [-1] access below is total.
        let mut lastword = outwords[n - 1].to_lowercase();

        if n > 1 {
            let w2 = outwords[n - 2].clone();
            if let Some(v) = self.ords_feminine.get(w2.as_str()) {
                // "бир" -> "" is the interesting case (quirk 5); every other
                // key maps to itself, so this is a no-op for them. Python
                // blanks it anywhere; the port only drops a *leading* "бир"
                // ("бир юз" -> "юзинчи"). An interior one is a real digit:
                // Python's 21000 -> "йигирма  мингинчи" reads as 20000th
                // (quirk 4, fixed).
                if n == 2 || !v.is_empty() {
                    outwords[n - 2] = v.to_string();
                }
            } else if w2 == "ўн" {
                // Python: outwords[-2][:-1] + 'н' — drops the final char and
                // re-appends "н", i.e. "ўн" -> "ў" + "н" -> "ўн". A no-op.
                let mut t: Vec<char> = w2.chars().collect();
                t.pop();
                outwords[n - 2] = t.into_iter().collect::<String>() + "н";
            }
        }

        if n == 3 {
            // Python: `if outwords[-3] in ['бир', 'бир']` — the list literal
            // repeats the same element, so this is a plain == "бир" test.
            if outwords[n - 3] == "бир" {
                outwords[n - 3] = String::new();
            }
        }

        let ord_hit = self.ords.get(lastword.as_str()).map(|v| v.to_string());
        lastword = match ord_hit {
            Some(v) => v,
            // except KeyError:
            None => self.ordinal_fallback(&lastword)?,
        };

        // self.title is the identity here — setup() leaves is_title False.
        outwords[n - 1] = self.title(&lastword);

        // " ".join(outwords).strip(): only a leading slot is ever emptied
        // (see above), so trimming the ends removes it.
        Ok(outwords.join(" ").trim().to_string())
    }

    /// `Num2Word_Base.verify_ordinal` for float/Decimal input, checks in
    /// Python's order:
    ///
    ///   1. `not value == int(value)` -> TypeError(errmsg_floatord)
    ///   2. `not abs(value) == value` -> TypeError(errmsg_negord)
    ///
    /// Both comparisons are *numeric*, so -0.0 passes both (`int(-0.0) ==
    /// -0.0` and `abs(-0.0) == -0.0` in IEEE) and `to_ordinal(-0.0)` goes on
    /// to render "нол вергул нол". A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
    /// The messages interpolate `str(value)` via the same repr
    /// reconstructions the cardinal path uses.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<()> {
        let py_str = || match value {
            FloatValue::Float { value, .. } => py_float_repr(*value),
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        let whole = match value {
            FloatValue::Float { value: f, .. } => f.fract() == 0.0 && f.is_finite(),
            FloatValue::Decimal { value: d, .. } => d.is_integer(),
        };
        if !whole {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_str()
            )));
        }
        // Numeric negativity — NOT the sign bit: abs(-0.0) == -0.0 in Python.
        let negative = match value {
            FloatValue::Float { value: f, .. } => *f < 0.0,
            FloatValue::Decimal { value: d, .. } => d.is_negative(),
        };
        if negative {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                py_str()
            )));
        }
        Ok(())
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangUzCyr {
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
        "вергул"
    }

    /// Port of `Num2Word_UZ_CYRILLIC.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).replace(',', '.')` and branches on `'.'`.
    /// `str(int)` never contains one, so integers always take the `else`
    /// branch: `self._int2word(int(n))`. The float branch (`pointword`, both
    /// halves rendered via `_int2word`) lives in [`Self::to_cardinal_float`].
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, false)
    }

    /// Port of `Num2Word_UZ_CYRILLIC.to_ordinal`.
    ///
    /// ```python
    /// self.verify_ordinal(number)
    /// outwords = self.to_cardinal(number).split(" ")
    /// lastword = outwords[-1].lower()
    /// try:
    ///     if len(outwords) > 1: ...          # rewrite outwords[-2]
    ///     if len(outwords) == 3: ...         # blank outwords[-3]
    ///     lastword = self.ords[lastword]
    /// except KeyError:
    ///     ...                                # suffix rules
    /// outwords[-1] = self.title(lastword)
    /// return " ".join(outwords).strip()
    /// ```
    ///
    /// The `try` wraps all three statements, but only `self.ords[lastword]`
    /// can raise — the two rewrites use `in` / `.get`. Crucially the rewrites
    /// run *before* the raise, so their mutations survive into the handler.
    /// That ordering is what produces `to_ordinal(100)` == "юзинчи" and
    /// `to_ordinal(1100)` == "бир минг бир юзинчи" (quirk 4, fixed).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // Inherited Num2Word_Base.verify_ordinal. The float check
        // (`errmsg_floatord`) is unreachable for integer input.
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }

        let cardinal = self.to_cardinal(value)?;
        self.ordinalize(&cardinal)
    }

    /// Port of `Num2Word_UZ_CYRILLIC.to_ordinal` for **float/Decimal** input
    /// — the same single Python method, whose three stages all stay live:
    ///
    ///   1. `verify_ordinal(number)`: numeric checks, so whole floats pass
    ///      (5.0, -0.0, `Decimal("1E+2")`), fractional values raise the
    ///      *float* TypeError and negative whole values the *negative* one.
    ///      Corpus-pinned: -1000000.0 / -3.0 / 0.5 / -1.5 are all TypeError.
    ///   2. `self.to_cardinal(number)`: this language's own str-splitting
    ///      version ([`Self::to_cardinal_float`] via the cardinal entry), so
    ///      a whole float keeps its ".0" tail — and a scientific repr raises
    ///      its `int()` ValueError *here*, after verify passed
    ///      (`to_ordinal(Decimal("1E+2"))` == ValueError, corpus-pinned).
    ///   3. the token machinery ([`Self::ordinalize`]) on that cardinal:
    ///      `to_ordinal(1.0)` -> tokens ["бир","вергул","нол"] -> the
    ///      `len == 3` rewrite blanks "бир" -> "вергул нол" (quirks 4/5 in
    ///      full flight); `to_ordinal(5.0)` -> "беш вергул нол" unchanged
    ///      ("нол" misses `ords` and every fallback arm, quirk 1).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.verify_ordinal_float(value)?;
        let cardinal = self.cardinal_float_entry(value, None)?;
        self.ordinalize(&cardinal)
    }

    // to_ordinal_num: UZ_CYR does not override Num2Word_Base.to_ordinal_num,
    // which returns the value unchanged → the trait default is correct.
    //
    // to_year: UZ_CYR does not override Num2Word_Base.to_year, which delegates
    // to to_cardinal → the trait default is correct.

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_UZ_CYRILLIC` defines exactly three things on this surface:
    // `CURRENCY_FORMS`, `pluralize` and `_cents_verbose`. Everything else —
    // `to_currency`, `to_cheque`, `_money_verbose`, `_cents_terse` — is
    // `Num2Word_Base`'s, verified by walking the MRO on the live object rather
    // than eyeballing the source. So only those three are overridden here plus
    // `lang_name` for the error message; the trait defaults already mirror the
    // rest.
    //
    // Deliberately NOT overridden:
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is `{}` (inherited from
    //     Base and never assigned), so the `adjective=True` branch can never
    //     fire. Default `None` is exact.
    //   * `currency_precision` — `CURRENCY_PRECISION` is `{}`, so
    //     `.get(code, 100)` is always 100. Default 100 is exact. This is why
    //     JPY does not take Base's `divisor == 1` shortcut and KWD/BHD do not
    //     take the 1000 path: all three fall straight through to the missing-
    //     forms `NotImplementedError` instead.
    //   * `money_verbose` / `cents_terse` — Base's, and the trait defaults are
    //     Base's.

    fn lang_name(&self) -> &str {
        "Num2Word_UZ_CYRILLIC"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_UZ_CYRILLIC.pluralize` as reached from `to_currency` — i.e.
    /// with a `CURRENCY_FORMS` tuple. Same rule as the scale-word path above;
    /// see [`plural_form_index`].
    ///
    /// Python indexes the tuple directly, so a shorter-than-3 entry with
    /// `form == 2` would raise IndexError. Every entry in this language's table
    /// carries three forms, so it is unreachable — but it is mapped to
    /// `N2WError::Index` rather than panicking so the exception *type* survives
    /// if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .get(plural_form_index(n))
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".to_string()))
    }

    /// Port of `Num2Word_UZ_CYRILLIC._cents_verbose`:
    ///
    /// ```python
    /// def _cents_verbose(self, number, currency):
    ///     return self._int2word(number, currency == 'UZS')
    /// ```
    ///
    /// The `currency == 'UZS'` argument is the `feminine` flag. It is **inert**:
    /// `ONES_FEMININE` and `ONES` are byte-identical tables (quirk 6), so UZS
    /// cents render exactly like EUR/USD cents. Verified against the live
    /// interpreter across 1/2/11/21/34/99/100/101 — every pair identical.
    /// Ported literally regardless, since the tables are data and could diverge.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        self.int2word(number, currency == "UZS")
    }

    // ---- float / Decimal cardinal path ----------------------------------

    /// Port of `Num2Word_UZ_CYRILLIC.to_cardinal` for **non-integer** input.
    ///
    /// UZ_CYR does NOT override `Num2Word_Base.to_cardinal_float`; it overrides
    /// `to_cardinal`, which handles floats/Decimals inline:
    ///
    /// ```python
    /// n = str(number).replace(',', '.')
    /// if '.' in n:
    ///     left, right = n.split('.')
    ///     return '%s %s %s' % (self._int2word(int(left)),
    ///                          self.pointword,
    ///                          self._int2word(int(right)))
    /// else:
    ///     return self._int2word(int(n))
    /// ```
    ///
    /// Both halves are rendered as **whole numbers**, not digit by digit:
    /// `int(left)` is the repr's integer part (trunc-toward-zero), and
    /// `int(right)` is the fractional digits read as a single integer
    /// (`Decimal("1.10")` → "ўн"). Python's `int()` also dropped the leading
    /// zeros, so `1.005` read "беш", indistinguishable from `1.5`; the port
    /// reads each leading zero as "нол" first (gladiaio/num2words2#205), so
    /// `0.01` → "нол вергул нол бир" and `1.005` → "бир вергул нол нол беш".
    ///
    /// The digits come from a byte-exact reconstruction of `str(number)`
    /// ([`py_float_repr`] / [`py_decimal_str`]) — NOT from
    /// `base.float2tuple`, whose binary `abs(value - pre) * 10**precision`
    /// diverges from the repr digits at high magnitude × precision (see the
    /// module header; an earlier revision reused it and emitted
    /// `732685275.4516093` one short, "…тўқсон икки" for "…тўқсон уч").
    /// For this language the decimal string IS the spec: Python never does
    /// f64 arithmetic here, so neither do we.
    ///
    /// Note this diverges from `Num2Word_Base.to_cardinal_float` in three
    /// ways, all faithful to UZ_CYR's own `to_cardinal`:
    ///   * the fractional part is one number, not per-digit;
    ///   * no negword is prepended for `-1 < value < 0` (Python's `int("-0")`
    ///     is 0 and the sign is silently lost), so `-0.5` → "нол вергул беш";
    ///   * scientific reprs raise ValueError from `int()` (`1e-05`,
    ///     `1.5e-05` → `int('5e-05')`, `Decimal('1E-7')`, `1e+16`), where
    ///     base would happily render digits.
    ///
    /// Evaluation order matches Python's `%`-tuple, left to right:
    /// `int(left)`, `_int2word(left)`, `int(right)`, `_int2word(right)` — so
    /// a 10^35 integer part raises its OverflowError before a malformed fraction
    /// raises its ValueError.
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**: Python's
    /// `to_cardinal` reads `str(number)`, never `self.precision`, so the kwarg
    /// cannot change this language's output. Confirmed on the live interpreter
    /// (`precision=1`/`5`/`0` all leave `2.675` → "…олти юз етмиш беш"). The
    /// value's own `precision` field is equally unused — it was derived from
    /// the same repr this method reconstructs.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).replace(',', '.') — a numeric str() never contains
        // ',', so the replace is a no-op and is not reproduced.
        let n = match value {
            FloatValue::Float { value, .. } => py_float_repr(*value),
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        match n.split_once('.') {
            // if '.' in n: left, right = n.split('.') — numeric strings
            // carry at most one '.', so split_once is exact.
            Some((left, right)) => {
                let l = parse_int(left)?;
                let left_words = self.int2word(&l, false)?;
                let r = parse_int(right)?;
                let mut right_words = self.int2word(&r, false)?;
                // int(right) drops leading zeros, so 0.05 read like 0.5 in
                // Python; each one is read as "нол" first (#205). A zero
                // fraction ("5.0", Decimal "5.00") stays a single "нол".
                if !r.is_zero() {
                    let zeros = right.len() - right.trim_start_matches('0').len();
                    let zero_word = self.int2word(&BigInt::zero(), false)?;
                    for _ in 0..zeros {
                        right_words = format!("{} {}", zero_word, right_words);
                    }
                }
                // u'%s %s %s' % (left_words, pointword, right_words)
                Ok(format!("{} {} {}", left_words, POINTWORD, right_words))
            }
            // else: self._int2word(int(n)) — no pointword. Reached by
            // no-fraction Decimals ("5") and scientific/non-finite reprs,
            // where int() raises ValueError ('1e+16', '1E-7', 'inf').
            None => self.int2word(&parse_int(&n)?, false),
        }
    }

    /// Fractional-cents entry point — resolves the module-header KNOWN GAP.
    ///
    /// `base.to_currency`'s fractional-cents branch renders the sub-unit as
    /// `self.to_cardinal(float(right))` (base.py line 476). Because UZ_CYR
    /// overrides `to_cardinal`, that dispatches to its OWN whole-number
    /// string-splitting version — NOT `Num2Word_Base.to_cardinal_float`'s
    /// digit-by-digit rendering, which is what the trait default (via
    /// `cardinal_from_bigdecimal`) would wrongly produce. Concretely, pure
    /// Python renders `to_currency(0.0055, "EUR")` cents as "нол вергул эллик
    /// беш" (55 as one number), where the digit-by-digit default says
    /// "нол вергул беш беш" (5, 5).
    ///
    /// So route through this language's [`Self::to_cardinal_float`].
    /// `float(right)` is reproduced with an f64 cast and the repr-derived
    /// precision — the same path `floatpath::cardinal_from_bigdecimal` already
    /// uses for the rest of the crate. No corpus row reaches this (every
    /// uz_cyr currency value is ≤ 2 dp, so `has_fractional_cents` never fires);
    /// verified against the live interpreter instead —
    /// `0.0055`/`0.0155`/`2.345`/`12.999`/`0.101`/`0.065` EUR all match.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        // Python: `float(right)`.
        let f = value
            .to_f64()
            .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?;
        // The precision field is inert in this language's to_cardinal_float
        // (Python's to_cardinal reads str(number), never self.precision), but
        // it is filled with the repr-derived count anyway so the FloatValue
        // stays well-formed: abs(Decimal(repr(f)).as_tuple().exponent).
        let repr = py_float_repr(f);
        let precision = match repr.split_once('.') {
            Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
            _ => 0,
        };
        self.to_cardinal_float(&FloatValue::Float { value: f, precision }, None)
    }

    // ---- string inputs ----------------------------------------------------

    /// `converter.str_to_number` is Base's `Decimal(value)` — UZ_CYR does not
    /// override it — but an *infinity* parse must not surface as Base's
    /// OverflowError (`int(Decimal('Infinity'))`): UZ_CYR's `to_cardinal`
    /// never calls `int()` on the Decimal, it calls `int(str(number))`, and
    /// `int("Infinity")` raises **ValueError**. Corpus-pinned:
    /// `num2words("Infinity", lang="uz_cyr")` / `"-Infinity"` are ValueError.
    /// Unlike lang_UZ, this class does *not* peel the sign before `int()`
    /// (`str(Decimal('-Infinity'))` == "-Infinity" goes in whole), so the
    /// message keeps it.
    ///
    /// Off-corpus caveats, flagged not repaired — the parse result routes
    /// through one shared entry, so an early raise cannot distinguish modes:
    /// Python's `to_ordinal_num(Decimal('Infinity'))` would return the value
    /// unchanged ("Infinity"), `to_ordinal` would raise OverflowError from
    /// `verify_ordinal`'s `int(value)`, and `to_currency` would raise
    /// decimal.InvalidOperation from `(inf * 100) % 1`. No corpus row reaches
    /// any of them (only `to="cardinal"` Infinity rows exist).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                if negative { "-Infinity" } else { "Infinity" }
            ))),
            other => Ok(other),
        }
    }
}
