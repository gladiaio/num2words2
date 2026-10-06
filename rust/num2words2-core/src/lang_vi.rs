//! Port of `lang_VI.py` (Vietnamese).
//!
//! Shape: **self-contained**. `Num2Word_VI` subclasses plain `object` — it has
//! no `Num2Word_Base` ancestry at all, so there is no inheritance chain to
//! chase, no `cards`, no `MAXVAL`, no `splitnum`/`clean`/`merge`, and no
//! `OverflowError` guard of its own (the port adds one, see bug 5). Every one
//! of the four in-scope methods is defined directly on the class.
//! `cards`/`merge` therefore stay at their trait defaults.
//!
//! Verified: `__init__.py` CONVERTER_CLASSES maps `"vi"` → `lang_VI.Num2Word_VI`.
//!
//! Call graph (all four in-scope entry points):
//!   * `to_cardinal(n)`    → `number_to_text(n)`
//!   * `to_ordinal(n)`     → `to_cardinal(n)` — Vietnamese ordinals are just
//!     cardinals here; there is no ordinal morphology whatsoever.
//!   * `to_ordinal_num(n)` → `"thứ " + str(n)` — pure string concat, no words.
//!   * `to_year(n)`        → `"năm " + to_cardinal(|n|)` (+ `" trước Công nguyên"`
//!     when `n < 0`).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is wrong-looking but is
//! exactly what CPython emits, verified against the interpreter.
//!
//! 1. **The float64 round-trip (the big one).** `number_to_text` does
//!    `number = "%.2f" % number` on an *integer*. Python's `%f` coerces the int
//!    to a C `double` first, so every input above 2^53 is silently rounded to
//!    the nearest representable double before a single word is generated:
//!      * `to_cardinal(10**23)` describes **99999999999999991611392**, not 10^23.
//!      * `to_cardinal(2**53 + 1) == to_cardinal(2**53)` — the +1 vanishes.
//!      * `to_cardinal(10**19 + 1)` == "mười Quintillion" (i.e. exactly 10^19).
//!    Modelled by [`py_float_int`], which reproduces int→double rounding
//!    (round-half-to-even) exactly, including the `OverflowError` CPython
//!    raises once the rounded value reaches 2^1024. This is *not* an academic
//!    edge: it changes the words for ordinary-looking inputs like 10^23.
//!    `%.2f` of an integral double always yields exact digits + `".00"`, so the
//!    `int(the_list[1]) > 0` branch (the " phẩy " decimal tail) is dead for
//!    integer input, and `the_list` always has exactly 2 elements.
//!
//! 2. **`denom[5]` is "trăm nghìn tỷ" at 10^15.** That literally reads "hundred
//!    thousand billion" = 10^17, and it sits where "Quadrillion"/"triệu tỷ"
//!    belongs. Hence the corpus row `10**15` → "một trăm nghìn tỷ". Kept verbatim.
//!
//! 3. **`denom` skips "Quindecillion".** The table runs
//!    …Quattuordecillion (idx 15), **Sexdecillion** (idx 16), Septendecillion…
//!    so every scale from 10^48 up is labelled one rank too high. Kept verbatim.
//!
//! 4. **`denom[20]` ("Vigintillion") is unreachable.** `vietnam_number` scans
//!    `for v in range(len(denom))` → `v` tops out at 20, giving `didx = v-1 = 19`.
//!    Reaching `didx == 20` would need `v == 21`. Dead entry, kept for indexing.
//!
//! 5. **Fixed (gladiaio/num2words2#159): `vietnam_number` runs out of `denom`
//!    at 1000^20 == 10^60.** Python fell off its loop there and returned
//!    `None` (so `num2words(10**63, lang="vi")` was `None`, and negatives a
//!    TypeError). The port checks the exact integer up front and raises
//!    `OverflowError` ("abs(v) must be less than 10^60."), so `maxval("vi")`
//!    is 10^60; the float path raises the same error where `vietnam_number`
//!    would have returned `None`.
//!
//! 6. **Dead store in `_convert_nn`.** `a = "lăm"` is assigned then immediately
//!    overwritten by the if/else below it (which is exhaustive). The net rule is
//!    digit 1 → "mốt", digit 5 → "lăm", else `to_19[digit]`. Collapsed here
//!    because it is provably unobservable, not because it looked untidy.
//!
//! 7. **`_convert_nnn` computes `_convert_nn(lval)` twice** in the
//!    `99 >= r > 0` branch. Pure function, no observable difference.
//!
//! # Error variants
//!
//! * `OverflowError` (from the `"%.2f"` int→float coercion) → [`N2WError::Overflow`].
//! * `TypeError` (`"âm " + None` / `"năm " + None`) → [`N2WError::Type`].
//! See the `concerns` in the report for the one case Rust's `Result<String>`
//! genuinely cannot express: a bare `None` return.
//!
//! # The currency surface
//!
//! `Num2Word_VI` still subclasses `object`, so there is **no currency
//! machinery to inherit**: no `CURRENCY_FORMS`, no `CURRENCY_PRECISION`, no
//! `pluralize`, no `_money_verbose`/`_cents_verbose`/`_cents_terse`, and no
//! `to_cheque`. Verified against the live class — every one of those
//! attributes is absent. It defines exactly one currency method:
//!
//! ```python
//! def to_currency(self, val, currency="VND", cents=True,
//!                 separator=",", adjective=False):
//!     from decimal import Decimal
//!     decimal_val = Decimal(str(val))
//!     has_fractional_cents = (decimal_val * 100) % 1 != 0
//!     if isinstance(val, int):
//!         return self.to_cardinal(val) + " đồng"
//!     if has_fractional_cents:
//!         return self.to_cardinal_float(val) + " đồng"
//!     return self.to_cardinal(val) + " đồng"
//! ```
//!
//! Consequences, all reproduced here and all corroborated by the corpus:
//!
//! 8. **`currency` was read and thrown away (fixed, #219).** Every branch
//!    appends the literal `" đồng"`, so Python gave đồng under EUR, USD, GBP
//!    and every other code. The port raises `NotImplementedError` for any code
//!    but VND (`lang_name` exists only for that message); `currency_forms`,
//!    `currency_precision` and `pluralize` stay at their trait defaults.
//!
//! 9. **`cents`, `separator` and `adjective` are dead parameters.** Never read.
//!    The generated `default_separator()` / `default_currency()` above are
//!    therefore inert for VI; they are kept because the trait resolves them
//!    before this body runs.
//!
//! 10. **`to_cardinal_float` does not exist** — the `has_fractional_cents`
//!     branch is a straight `AttributeError`. So *any* value with more than two
//!     decimal places (`12.345`, `1.011`, `2.675`, `0.001`) raises instead of
//!     converting. See [`LangVi::to_currency`].
//!
//! 11. **`to_cheque` does not exist**, so Python raised `AttributeError`. The
//!     port raises NotImplementedError ("does not support to='cheque'",
//!     #223).
//!
//! 12. **`(decimal_val * 100) % 1` runs *before* the `isinstance(val, int)`
//!     test, and `Decimal.__mod__` is bounded by the arithmetic context.**
//!     `decimal`'s default context is `prec=28`, and `%` raises
//!     `InvalidOperation[DivisionImpossible]` once the quotient `int(m)` needs
//!     more than 28 digits. With `m = val * 100` that is `|val| >= 10**26` —
//!     for **ints as well as floats**, because the guard precedes the branch.
//!     Bisected against the live interpreter: `10**26 - 1` converts,
//!     `10**26` raises; likewise `1e25` vs `1e26`. So `to_currency` can never
//!     reach the `2**1024` OverflowError in [`py_float_int`] nor the `10**60`
//!     `None` fall-off in [`LangVi::vietnam_number`] — `10**26` is far below
//!     both, and this guard fires first. See [`div_impossible`].
//!
//! 13. **The float branch re-enters `number_to_text` with a `float`, not an
//!     `int`,** so `"%.2f" % number` formats the *double itself* rather than
//!     `repr()`'s shortest round-trip. Above 2^53 the two diverge:
//!     `str(1e23)` is `"1e+23"` but `"%.2f" % 1e23` is
//!     `"99999999999999991611392.00"`, and Vietnamese words follow the latter.
//!     Reproduced in [`LangVi::number_to_text_decimal`] by round-tripping the
//!     decimal string back through `f64` — `str(float)` is shortest-round-trip
//!     by construction, so the parse recovers the original double bit for bit.
//!
//! 14. **The `" phẩy "` tail counts *hundredths*, not a fraction.** `12.5`
//!     prints "mười hai phẩy năm mươi" — "twelve point fifty" — because
//!     `"%.2f"` pads to `"12.50"` and the tail is `vietnam_number(50)`.
//!     Likewise `0.01` → "không phẩy một" ("zero point one"). Corpus rows
//!     `0.5` and `0.01` pin both.
//!
//! 15. ~~**`str_to_number` does not exist**~~ — fixed
//!     (gladiaio/num2words2#157). Same `object` ancestry, same plain
//!     attribute-lookup failure as bugs #10/#11: the dispatcher calls
//!     `converter.str_to_number(number)` for every string input, so in
//!     Python *every* string ("12" included) raised AttributeError. The port
//!     uses the shared `Decimal(value)` parse every other language has, so
//!     "12" reads like 12 and "1.5" like `Decimal("1.5")`.
//!
//! 16. **`to_fraction` does not exist** in Python (AttributeError). VI has
//!     no fraction rules, so it now raises NotImplementedError like every
//!     language without them, and "1/0" ZeroDivisionError (#217).

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use std::sync::OnceLock;
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::str::FromStr;

/// `to_19`; indexed 0..=19 directly by value.
const TO_19: [&str; 20] = [
    "không",
    "một",
    "hai",
    "ba",
    "bốn",
    "năm",
    "sáu",
    "bảy",
    "tám",
    "chín",
    "mười",
    "mười một",
    "mười hai",
    "mười ba",
    "mười bốn",
    "mười lăm",
    "mười sáu",
    "mười bảy",
    "mười tám",
    "mười chín",
];

/// `tens`; index `v` corresponds to the value `20 + 10*v`.
const TENS: [&str; 8] = [
    "hai mươi",
    "ba mươi",
    "bốn mươi",
    "năm mươi",
    "sáu mươi",
    "bảy mươi",
    "tám mươi",
    "chín mươi",
];

/// `denom`; index `d` labels the scale `1000^d`.
///
/// Preserved verbatim including the three defects documented in the module
/// header: `[5]` is "trăm nghìn tỷ" at 10^15, "Quindecillion" is missing
/// between `[15]` and `[16]`, and `[20]` is unreachable. `[0]` is "" and is
/// never used (the caller only reaches this table for values >= 1000, which
/// forces `didx >= 1`).
const DENOM: [&str; 21] = [
    "",
    "nghìn",
    "triệu",
    "tỷ",
    "nghìn tỷ",
    "trăm nghìn tỷ",
    "Quintillion",
    "Sextillion",
    "Septillion",
    "Octillion",
    "Nonillion",
    "Decillion",
    "Undecillion",
    "Duodecillion",
    "Tredecillion",
    "Quattuordecillion",
    "Sexdecillion",
    "Septendecillion",
    "Octodecillion",
    "Novemdecillion",
    "Vigintillion",
];

fn type_error(msg: impl Into<String>) -> N2WError {
    N2WError::Type(msg.into())
}

/// `AttributeError: 'Num2Word_VI' object has no attribute '<name>'`.
///
/// `Num2Word_VI` inherits from `object`, so a missing method is a plain
/// attribute-lookup failure — not a deliberate `NotImplementedError`. One
/// call site reaches it: `to_cardinal_float` (bug #10). (`str_to_number`,
/// bug #15, and `to_cheque`, bug #11, are fixed.)
fn missing_attr(name: &str) -> N2WError {
    N2WError::Attribute(format!(
        "'Num2Word_VI' object has no attribute '{}'",
        name
    ))
}

/// `decimal.InvalidOperation([DivisionImpossible])` — bug #12.
///
/// `decimal` is a stdlib module, not a language module, but it is still a
/// non-builtin exception class, which is exactly what `N2WError::Custom` is
/// for: the binding imports `decimal`, looks up `InvalidOperation` and raises
/// the real class, so `except decimal.InvalidOperation` keeps working.
///
/// The message is `str(e)` of what CPython raises. Python's `args` is
/// `([DivisionImpossible],)` — a list holding the condition *class* — whereas
/// `Custom` can only carry a string, so `args` is the rendered form. The
/// exception type and `str(e)` match; `e.args[0]` is a `str` rather than a
/// `list`. Flagged in the report.
fn div_impossible() -> N2WError {
    N2WError::Custom {
        module: "decimal",
        class: "InvalidOperation",
        msg: "[<class 'decimal.DivisionImpossible'>]".into(),
    }
}

/// Reproduces CPython's `int` → `float` coercion, returning the *exact integer
/// value* of the resulting double.
///
/// `number_to_text` does `"%.2f" % number`, and `%f` coerces its operand to a C
/// `double`. `"%.2f"` of an integral double prints that double's exact digits
/// (CPython uses correctly-rounded David Gay formatting — it does **not** clip
/// to 17 significant digits), so `int(the_list[0])` recovers precisely the
/// integer this function returns. Any int with a magnitude above 2^53 is
/// therefore mangled before conversion, which is Python bug #1 above.
///
/// Rounding is IEEE-754 round-half-to-even on the 53-bit significand, matching
/// CPython's `PyLong_AsDouble`. Returns `OverflowError` exactly when CPython
/// does: when the rounded result reaches 2^1024 (i.e. would be `inf`).
///
/// `n` must be non-negative (`number_to_text` applies `abs()` first).
/// Validated against CPython's `float()` over 30k random widths in 54..=1030
/// bits plus the overflow-boundary values `2^1024 - 2^970{,-1}`: zero mismatches.
fn py_float_int(n: &BigInt) -> Result<BigInt> {
    if n.is_zero() {
        return Ok(BigInt::zero());
    }
    // `bits()` is the magnitude's bit length; n >= 0 here.
    let bits = n.bits();
    if bits <= 53 {
        // Exactly representable — the coercion is lossless.
        return Ok(n.clone());
    }
    let shift = bits - 53;
    let mut hi = n >> shift; // 53 significant bits, in [2^52, 2^53)
    let rem = n - (&hi << shift); // the bits being discarded
    let half = BigInt::one() << (shift - 1);

    // Round half to even: round up on a strict majority, or on an exact tie
    // when the retained significand is odd.
    if rem > half || (rem == half && hi.is_odd()) {
        hi += 1u32;
    }
    let res = hi << shift; // carry out of the significand is fine: gives 2^bits

    if res >= (BigInt::one() << 1024u32) {
        // CPython: OverflowError("int too large to convert to float")
        return Err(N2WError::Overflow(
            "int too large to convert to float".into(),
        ));
    }
    Ok(res)
}

/// The exclusive ceiling (gladiaio/num2words2#159): `denom` ends at
/// 1000^19, so 10^60 and above raise `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(60))
}

fn too_large(value: &BigInt) -> N2WError {
    N2WError::Overflow(format!(
        "abs({}) must be less than {}.",
        value,
        maxval_ceiling()
    ))
}

pub struct LangVi;

impl Default for LangVi {
    fn default() -> Self {
        Self::new()
    }
}

impl LangVi {
    pub fn new() -> Self {
        LangVi
    }

    /// Port of `_convert_nn`. Callers only ever pass `0 <= val < 100`.
    ///
    /// Python returns `None` for `val >= 100` (the `dval + 10 > val` scan finds
    /// no match), but that is unreachable: `vietnam_number` gates on `val < 100`
    /// and `_convert_nnn` only ever passes `val % 100`.
    fn convert_nn(&self, val: u32) -> String {
        debug_assert!(val < 100, "_convert_nn is only ever called with val < 100");
        if val < 20 {
            return TO_19[val as usize].to_string();
        }
        // Python scans `((k, 20 + 10*v) for (v, k) in enumerate(tens))` for the
        // first `dval + 10 > val`; that is just the tens digit.
        let dcap = TENS[(val / 10 - 2) as usize];
        let unit = val % 10;
        if unit != 0 {
            // Bug #6: the `a = "lăm"` dead store is elided; the exhaustive
            // if/else plus the trailing "năm" override reduce to exactly this.
            let a = match unit {
                1 => "mốt",
                5 => "lăm",
                d => TO_19[d as usize],
            };
            return format!("{} {}", dcap, a);
        }
        dcap.to_string()
    }

    /// Port of `_convert_nnn`. Callers only ever pass `0 <= val < 1000`, so
    /// `to_19[rem]` (rem = val // 100 <= 9) is always in range.
    ///
    /// Note `_convert_nnn(0)` == "" — the empty string, not "không".
    fn convert_nnn(&self, val: u32) -> String {
        debug_assert!(val < 1000, "_convert_nnn is only ever called with val < 1000");
        let mut word = String::new();
        let (mod_, rem) = (val % 100, val / 100);
        if rem > 0 {
            word = format!("{} trăm", TO_19[rem as usize]);
            if mod_ > 0 {
                word.push(' ');
            }
        }
        if mod_ > 0 && mod_ < 10 {
            // Python's `word != "" and A or B` idiom. Both A branches are
            // non-empty strings (truthy), so this is a plain conditional.
            if mod_ == 5 {
                // Note the asymmetry with the else-branch: no space after "lẻ"
                // because "lẻ năm" is spelled out as one literal in Python.
                if !word.is_empty() {
                    word.push_str("lẻ năm");
                } else {
                    word.push_str("năm");
                }
            } else if !word.is_empty() {
                word.push_str("lẻ ");
                word.push_str(&self.convert_nn(mod_));
            } else {
                word.push_str(&self.convert_nn(mod_));
            }
        }
        if mod_ >= 10 {
            word.push_str(&self.convert_nn(mod_));
        }
        word
    }

    /// Port of `vietnam_number`.
    ///
    /// Returns `Ok(None)` where Python falls off the `for` loop and implicitly
    /// returns `None` — i.e. `val >= 1000^20 == 10^60` (bug #5). `val` is
    /// already float-rounded and non-negative by the time it gets here.
    fn vietnam_number(&self, val: &BigInt) -> Option<String> {
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        if *val < hundred {
            // val < 100 fits a u32 trivially.
            return Some(self.convert_nn(to_u32(val)));
        }
        if *val < thousand {
            return Some(self.convert_nnn(to_u32(val)));
        }
        // Python: `for didx, dval in ((v - 1, 1000**v) for v in range(len(denom)))`.
        // v == 0 (dval == 1) and v == 1 (dval == 1000) can never fire here
        // because val >= 1000, so didx >= 1 and denom[0] == "" is never read.
        for v in 0..DENOM.len() as u32 {
            let dval = thousand.pow(v);
            if dval > *val {
                let didx = (v - 1) as usize;
                let modulus = thousand.pow(v - 1);
                let lval = val / &modulus;
                let r = val - (&lval * &modulus);

                // lval < 1000 always (else the previous v would have matched).
                let head = self.convert_nnn(to_u32(&lval));
                let mut ret = format!("{} {}", head, DENOM[didx]);
                if r <= BigInt::from(99) && r > BigInt::zero() {
                    // Python rebuilds the string from scratch here rather than
                    // appending; same result (bug #7).
                    ret = format!("{} {} lẻ", head, DENOM[didx]);
                }
                if r > BigInt::zero() {
                    ret.push(' ');
                    // Recursion depth is bounded by the 21-entry denom table.
                    ret.push_str(&self.vietnam_number(&r)?);
                }
                return Some(ret);
            }
        }
        // val >= 10^60: Python returns None.
        None
    }

    /// Port of `number_to_text`.
    ///
    /// The `" phẩy "` decimal branch is unreachable for integer input: the
    /// `"%.2f"` of an integral double always ends in `".00"`, so
    /// `int(the_list[1]) > 0` is always false. Omitted deliberately.
    fn number_to_text(&self, number: &BigInt) -> Result<String> {
        let is_negative = number.is_negative();
        let number = number.abs();

        // `number = "%.2f" % number` — the lossy int→float coercion (bug #1).
        // This is also the only site that can raise, and it raises *before*
        // the sign is re-applied, so negatives overflow identically.
        check_maxval(&number, maxval_ceiling())?;
        let rounded = py_float_int(&number)?;

        match self.vietnam_number(&rounded) {
            Some(start_word) => {
                if is_negative {
                    Ok(format!("âm {}", start_word))
                } else {
                    Ok(start_word)
                }
            }
            // Unreachable: `number < 10^60` never float-rounds up to 10^60
            // (the nearest double is below it), so `vietnam_number` succeeds.
            None => Err(too_large(&rounded)),
        }
    }

    /// `(Decimal(str(val)) * 100) % 1 != 0`, evaluated under `decimal`'s
    /// default context (bug #12).
    ///
    /// Two context effects are load-bearing and both are reproduced:
    ///
    /// * The multiply is rounded to `prec = 28` significant digits
    ///   (ROUND_HALF_EVEN). `Inexact`/`Rounded` are not trapped, so this is
    ///   silent — a 31-digit `Decimal` can lose its fractional cents here.
    ///   `BigDecimal::with_prec` is half-even too, matching.
    /// * `%` then raises `DivisionImpossible` when the quotient `int(m)` would
    ///   exceed those 28 digits, i.e. `|m| >= 10**28`. `_pydecimal._divide`
    ///   reaches that either through `expdiff > prec` or through the
    ///   `q < 10**prec` test; both collapse to the same bound.
    ///
    /// `Decimal`'s `%` truncates toward zero (the remainder takes the
    /// dividend's sign), unlike `int`'s floor `%` — but only `!= 0` is asked,
    /// so the sign never escapes.
    fn has_fractional_cents(value: &BigDecimal) -> Result<bool> {
        let m = (value * BigDecimal::from(100)).with_prec(28);
        if m.abs() >= BigDecimal::from(BigInt::from(10).pow(28)) {
            return Err(div_impossible());
        }
        // with_scale(0) truncates toward zero — Python's int(m).
        Ok(&m - m.with_scale(0) != BigDecimal::zero())
    }

    /// `number_to_text` re-entered with a **float**, which is what the two
    /// non-int `to_currency` branches do (bug #13).
    ///
    /// Distinct from [`LangVi::number_to_text`] on purpose: that one takes an
    /// `int` and models `%f`'s int→double coercion via [`py_float_int`], and
    /// its `" phẩy "` tail is dead because `"%.2f"` of an integral double
    /// always ends `".00"`. Here the tail is live, and the coercion is a
    /// no-op because the value already *is* a double.
    ///
    /// `"%.2f" % number` prints the double's exact value rounded to two
    /// places, ties to even (CPython routes `%f` through David Gay's
    /// correctly-rounded `dtoa`). Rust's `{:.2}` is exact and ties-to-even
    /// as well, so the two agree; the double itself is recovered by parsing
    /// the decimal string, which round-trips because Python produced it with
    /// `repr`. Cross-checked against CPython over 20k randomized values.
    fn number_to_text_decimal(&self, number: &BigDecimal) -> Result<String> {
        // `number < 0`: BigDecimal has no signed zero, and neither does the
        // comparison — Python agrees, `-0.0 < 0` is False.
        let is_negative = number.is_negative();
        let number = number.abs();

        // `number = "%.2f" % number`. Guarded above to |value| < 10**26, so
        // the f64 parse cannot reach infinity and the string stays short.
        let as_f64: f64 = number
            .to_string()
            .parse()
            .map_err(|_| type_error("float() argument must be a number"))?;
        let formatted = format!("{:.2}", as_f64);

        // `the_list = str(number).split(".")`. Split rather than assert: for a
        // non-finite double `"%.2f"` yields "inf"/"nan" with no point, and then
        // Python's `int(the_list[0])` raises ValueError — so the shape below is
        // the port, and no branch can panic. (`panic = "abort"` in the release
        // profile would take the host interpreter down with it.) Unreachable in
        // practice: bug #12's guard caps |value| below 10**26.
        let (int_part, frac_part) = match formatted.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (formatted.as_str(), None),
        };
        let int_val = BigInt::from_str(int_part).map_err(|_| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                int_part
            ))
        })?;

        // Unreachable for the same reason: 10**26 is far below
        // vietnam_number's 10**60 fall-off. Mapped rather than panicked so the
        // exception type survives if that guard ever moves — every caller
        // concatenates onto the result, so None becomes TypeError.
        let start_word = self.vietnam_number(&int_val).ok_or_else(|| {
            type_error("unsupported operand type(s) for +: 'NoneType' and 'str'")
        })?;

        // `if len(the_list) > 1 and int(the_list[1]) > 0`. frac_part is the two
        // hundredths digits, so this is vietnam_number(0..=99) — bug #14.
        let mut final_result = start_word;
        if let Some(frac_part) = frac_part {
            let frac_val = u32::from_str(frac_part).map_err(|_| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    frac_part
                ))
            })?;
            if frac_val > 0 {
                final_result = format!("{} phẩy {}", final_result, self.convert_nn(frac_val));
            }
        }
        if is_negative {
            final_result = format!("âm {}", final_result);
        }
        Ok(final_result)
    }

    /// Port of `number_to_text` for the **cardinal** float/Decimal path — i.e.
    /// `to_cardinal(x)` reached with a non-integer `x`. `Num2Word_VI` has no
    /// `to_cardinal_float`, so `num2words(<non-int>, lang="vi")` lands straight
    /// in `to_cardinal(number)` → `number_to_text(number)`, whose first act is
    /// `number = "%.2f" % number`.
    ///
    /// `%f` coerces its operand to a C `double`, so a `Decimal` is coerced too:
    /// `"%.2f" % Decimal("2.675")` is `"2.67"` — the *float*-rounded value, not
    /// the exact `"2.68"`. Callers therefore pass `magnitude` already coerced
    /// to `f64`, and `is_negative` taken from the sign of the *original* value
    /// (Python evaluates `number < 0` before `abs()`), mirroring exactly:
    ///
    /// ```python
    /// is_negative = number < 0
    /// number      = "%.2f" % abs(number)
    /// ```
    ///
    /// Distinct from [`LangVi::number_to_text_decimal`] (the *currency*
    /// re-entry) in one load-bearing way: there the `vietnam_number` `None`
    /// fall-off is capped unreachable by bug #12's 10**26 guard. Here there is
    /// no such cap — `to_cardinal(1e61)` really does hand `vietnam_number` a
    /// value >= 10^60 — and that is an `OverflowError` (bug #5, #159).
    ///
    /// `precision` / the `precision=` kwarg are irrelevant: `"%.2f"` is always
    /// two places, and the dispatcher pops `precision=` before `to_cardinal`
    /// (VI defines no `precision` attribute), so it never reaches this method.
    fn number_to_text_float(&self, is_negative: bool, magnitude: f64) -> Result<String> {
        // `number = "%.2f" % magnitude`. CPython routes `%f` through David
        // Gay's correctly-rounded `dtoa` (ties to even); Rust's `{:.2}` is
        // correctly rounded and ties-to-even as well, so the two agree — the
        // same equivalence the currency path already relies on.
        let formatted = format!("{:.2}", magnitude);

        // `the_list = str(number).split(".")`. Finite magnitudes always carry a
        // point; the None arm mirrors `"%.2f"` of a non-finite double
        // ("inf"/"nan"), where Python's `int(the_list[0])` then raises
        // ValueError — reproduced by the `BigInt::from_str` failure below.
        let (int_part, frac_part) = match formatted.split_once('.') {
            Some((i, f)) => (i, Some(f)),
            None => (formatted.as_str(), None),
        };
        let int_val = BigInt::from_str(int_part).map_err(|_| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                int_part
            ))
        })?;

        match self.vietnam_number(&int_val) {
            Some(start_word) => {
                let mut final_result = start_word;
                // `if len(the_list) > 1 and int(the_list[1]) > 0` — the two
                // hundredths digits, so `vietnam_number(0..=99)` == convert_nn
                // (bug #14: the tail counts hundredths, e.g. 12.5 → "... năm
                // mươi").
                if let Some(frac_part) = frac_part {
                    let frac_val = u32::from_str(frac_part).map_err(|_| {
                        N2WError::Value(format!(
                            "invalid literal for int() with base 10: '{}'",
                            frac_part
                        ))
                    })?;
                    if frac_val > 0 {
                        final_result =
                            format!("{} phẩy {}", final_result, self.convert_nn(frac_val));
                    }
                }
                if is_negative {
                    final_result = format!("âm {}", final_result);
                }
                Ok(final_result)
            }
            // vietnam_number ran out of `denom` (int part >= 10^60). Python
            // returned `None` here (#159).
            None => Err(too_large(&int_val)),
        }
    }

    /// `self.to_cardinal(val) + " đồng"`.
    ///
    /// Python concatenates onto `to_cardinal`'s result, so the `None` that
    /// [`LangVi::number_to_text`] can return past 10**60 becomes a TypeError
    /// here rather than propagating — `to_currency` never yields `None`.
    /// Unreachable under bug #12's 10**26 cap, but the sentinel must not leak
    /// to the binding, which would turn it into a bare Python `None`.
    fn dong(&self, cardinal: Result<String>) -> Result<String> {
        match cardinal {
            Ok(s) => Ok(format!("{} đồng", s)),
            Err(N2WError::ReturnsNone) => Err(type_error(
                "unsupported operand type(s) for +: 'NoneType' and 'str'",
            )),
            Err(e) => Err(e),
        }
    }
}

/// `val` is proven < 1000 at every call site; the fallback is unreachable.
fn to_u32(val: &BigInt) -> u32 {
    use num_traits::ToPrimitive;
    val.to_u32().expect("value is bounded below 1000 by construction")
}

impl Lang for LangVi {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "VND"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    // cards / merge: Num2Word_VI subclasses `object` and has no engine at
    // all — the trait defaults are never reached.

    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.number_to_text(value)
    }

    /// `to_ordinal` is `return self.to_cardinal(number)` — identical output.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_ordinal_num` is `"thứ " + str(number)` — the raw digits, never words.
    /// It never touches `number_to_text`, so it neither float-rounds nor
    /// overflows: `to_ordinal_num(10**400)` happily prints all 401 digits, and
    /// negatives keep their minus sign ("thứ -1").
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("thứ {}", value))
    }

    /// `to_year(val, longval=True)` — `longval` is accepted and ignored.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Python concatenates: `"năm " + self.to_cardinal(val)`. Where
        // to_cardinal falls off the end and returns None, that concatenation
        // raises TypeError — so unlike to_cardinal itself, to_year never
        // yields None. Convert the sentinel rather than propagating it.
        let concat = |r: Result<String>| -> Result<String> {
            match r {
                Err(N2WError::ReturnsNone) => Err(type_error(
                    "can only concatenate str (not \"NoneType\") to str",
                )),
                other => other,
            }
        };
        if value.is_negative() {
            // Python passes `-val` (a positive int), so to_cardinal never sees
            // the sign and never prefixes "âm".
            Ok(format!(
                "năm {} trước Công nguyên",
                concat(self.to_cardinal(&(-value)))?
            ))
        } else {
            Ok(format!("năm {}", concat(self.to_cardinal(value))?))
        }
    }

    /// `to_ordinal_num(float/Decimal)`: still `"thứ " + str(number)` — the
    /// raw Python string form, so "thứ 5.00", "thứ 1e+16" and "thứ -0.0"
    /// all succeed. `repr_str` is Python's `str(value)`, computed
    /// binding-side; nothing here touches `number_to_text`, so this
    /// neither float-rounds nor overflows.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("thứ {}", repr_str))
    }

    /// `to_year(float/Decimal)` — the same two-branch body as the integer
    /// path:
    ///
    /// ```python
    /// if val < 0:
    ///     return "năm " + self.to_cardinal(-val) + " trước Công nguyên"
    /// return "năm " + self.to_cardinal(val)
    /// ```
    ///
    /// The `val < 0` test is **numeric**, unlike the textual sign tests
    /// elsewhere in this file: `-0.0 < 0` is False in Python, so a
    /// negative-zero float is a plain "năm không" — no era suffix, no "âm".
    /// Either branch hands `to_cardinal` a *non-negative* value, so "âm"
    /// can never appear; `to_cardinal(float)` is `number_to_text` with its
    /// `"%.2f"` coercion (a Decimal is coerced to a double the same way, via
    /// the same `str -> f64` round the cardinal path uses). A bare-None
    /// fall-off (>= 10^60) dies in the `"năm " + None` concatenation as
    /// TypeError, exactly like the integer path.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let concat = |r: Result<String>| -> Result<String> {
            match r {
                Err(N2WError::ReturnsNone) => Err(type_error(
                    "can only concatenate str (not \"NoneType\") to str",
                )),
                other => other,
            }
        };
        let (lt_zero, magnitude) = match value {
            // `< 0.0`, NOT the sign bit: -0.0 takes the positive branch.
            FloatValue::Float { value, .. } => (*value < 0.0, value.abs()),
            FloatValue::Decimal { value, .. } => {
                let m: f64 = value.abs().to_string().parse().map_err(|_| {
                    // Unreachable: a finite BigDecimal always renders to an
                    // f64-parseable string. Mapped, not panicked, so the host
                    // interpreter is never taken down by a release-mode abort.
                    N2WError::Value("could not convert Decimal to float".into())
                })?;
                (value.is_negative(), m)
            }
        };
        let cardinal = concat(self.number_to_text_float(false, magnitude))?;
        if lt_zero {
            Ok(format!("năm {} trước Công nguyên", cardinal))
        } else {
            Ok(format!("năm {}", cardinal))
        }
    }

    /// Float/Decimal cardinal path. `Num2Word_VI` has **no** `to_cardinal_float`
    /// — its `to_cardinal(number)` handles non-integers inline via
    /// `number_to_text`, whose `"%.2f" % number` coerces float *and* Decimal
    /// alike to a C double. So this override reproduces `number_to_text` on a
    /// non-integral operand rather than the base `float2tuple`/pointword path
    /// (VI has no `pointword` — the separator is the literal "phẩy").
    ///
    /// `precision_override` (the `precision=` kwarg) is discarded: the
    /// dispatcher pops it before `to_cardinal`, and VI defines no `precision`
    /// attribute, so `precision=` is a proven no-op (verified: `precision=4`
    /// still yields two-place `"%.2f"` output). The `precision` on `FloatValue`
    /// is unused for the same reason — `"%.2f"` is always two places.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            // A Python `float`: the raw f64 crosses intact (both sides are
            // IEEE-754 doubles), so its `"%.2f"` artefacts survive. Sign is
            // taken before `abs`, exactly as `number < 0` runs before `abs`.
            FloatValue::Float { value, .. } => {
                self.number_to_text_float(*value < 0.0, value.abs())
            }
            // A Python `Decimal`: `"%.2f"` coerces it to a double. Sign and
            // `abs` are taken on the *exact* Decimal first (so a value that
            // underflows to -0.0 on coercion still counts as negative), then
            // coerced via the same `str -> f64` round the currency path uses —
            // `float(abs(Decimal(...)))`. Issue #603: `98746251323029.99`
            // coerces to a double whose `"%.2f"` is `...29.98`, not `...29.99`.
            FloatValue::Decimal { value, .. } => {
                let magnitude: f64 = value.abs().to_string().parse().map_err(|_| {
                    // Unreachable: a finite BigDecimal always renders to an
                    // f64-parseable string. Mapped, not panicked, so the host
                    // interpreter is never taken down by a release-mode abort.
                    N2WError::Value("could not convert Decimal to float".into())
                })?;
                self.number_to_text_float(value.is_negative(), magnitude)
            }
        }
    }

    // `Num2Word_VI` had no `to_fraction` (AttributeError, bug #16). VI has no
    // fraction rules, so the trait default raises NotImplementedError (#217).

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_VI` defines `to_currency` and nothing else. It reads no
    // CURRENCY_FORMS, calls no pluralize/_money_verbose/_cents_*, and has no
    // to_cheque — so `currency_forms`, `currency_adjective`,
    // `currency_precision`, `pluralize`, `money_verbose`, `cents_verbose`,
    // `cents_terse` are all left at their trait defaults and
    // are all unreachable. Overriding them would invent behaviour the class
    // does not have.
    //
    // `cardinal_from_decimal` likewise stays at its default: VI's fractional
    // path is an AttributeError, not a float conversion, so there is no
    // fractional-cents rendering to port.

    fn lang_name(&self) -> &str {
        "Num2Word_VI"
    }

    /// Port of `Num2Word_VI.to_currency`.
    ///
    /// `cents`, `separator` and `adjective` are accepted and discarded — bugs
    /// #8 and #9. Every branch ends in the literal `" đồng"`, so Python printed
    /// đồng whatever `currency=` said; the port raises NotImplementedError for
    /// any code but VND instead of naming the wrong currency (#219).
    ///
    /// Statement order matters and is preserved: the `Decimal` guard runs
    /// *before* the int/float branch, so `to_currency(10**26)` raises
    /// InvalidOperation even though the int branch would never have looked at
    /// `has_fractional_cents` (bug #12).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        if currency != self.default_currency() {
            return Err(crate::currency::unknown_currency(self, currency));
        }
        // decimal_val = Decimal(str(val))
        let decimal_val = match val {
            CurrencyValue::Int(v) => BigDecimal::from(v.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };
        // Evaluated for its exception as much as its value: it can raise for
        // either branch, and the int branch then ignores the result.
        let has_fractional_cents = Self::has_fractional_cents(&decimal_val)?;

        // `isinstance(val, int)` — a true int, never a float or a Decimal.
        // Note `has_decimal` is not consulted anywhere: VI does not go through
        // base.to_currency, so Decimal("5") and Decimal("5.00") both take the
        // float branch and both print "năm đồng".
        if let CurrencyValue::Int(v) = val {
            // to_cardinal(int): keeps the "%.2f" int→double mangling, so
            // to_currency(10**23) describes 99999999999999991611392 đồng.
            return self.dong(self.to_cardinal(v));
        }

        if has_fractional_cents {
            // `self.to_cardinal_float(val)` — the method does not exist, so
            // the attribute lookup raises before " đồng" is ever appended
            // (bug #10).
            return Err(missing_attr("to_cardinal_float"));
        }

        let value = match val {
            CurrencyValue::Decimal { value, .. } => value,
            CurrencyValue::Int(_) => unreachable!("int branch returned above"),
        };
        self.dong(self.number_to_text_decimal(value))
    }

    // `Num2Word_VI` had no `to_cheque` (AttributeError, bug #11).
    // No cheque rules, so NotImplementedError ("lang='vi' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `abs(Decimal(str(value)).as_tuple().exponent)` — what the dispatcher
    /// computes Python-side before calling `_RUST.to_cardinal_float`. The
    /// value is irrelevant for VI (bug: `"%.2f"` is always two places), but
    /// the tests thread the honest number through anyway.
    fn repr_precision(f: f64) -> u32 {
        let s = format!("{}", f);
        match s.split_once('.') {
            Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
            _ => 0,
        }
    }

    fn float_case(v: f64) -> String {
        LangVi::new()
            .to_cardinal_float(
                &FloatValue::Float {
                    value: v,
                    precision: repr_precision(v),
                },
                None,
            )
            .unwrap()
    }

    fn dec_case(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = s.split_once('.').map_or(0, |(_, f)| f.len() as u32);
        LangVi::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every float `cardinal` corpus row with a fractional part (whole floats
    /// like 0.0/1.0 are routed to the integer path by the dispatcher and never
    /// reach `to_cardinal_float`), plus the live-interpreter sweep.
    #[test]
    fn float_cardinals_match_python() {
        for (v, want) in [
            (0.5, "không phẩy năm mươi"),
            (1.5, "một phẩy năm mươi"),
            (2.25, "hai phẩy hai mươi lăm"),
            (3.14, "ba phẩy mười bốn"),
            (0.01, "không phẩy một"),
            (0.1, "không phẩy mười"),
            (0.99, "không phẩy chín mươi chín"),
            (1.01, "một phẩy một"),
            (12.34, "mười hai phẩy ba mươi bốn"),
            (99.99, "chín mươi chín phẩy chín mươi chín"),
            (100.5, "một trăm phẩy năm mươi"),
            (1234.56, "một nghìn hai trăm ba mươi bốn phẩy năm mươi sáu"),
            (-0.5, "âm không phẩy năm mươi"),
            (-1.5, "âm một phẩy năm mươi"),
            (-12.34, "âm mười hai phẩy ba mươi bốn"),
            // The f64-artefact rows: "%.2f" % 1.005 == "1.00" (the double is
            // 1.00499999…) and "%.2f" % 2.675 == "2.67" (2.67499999…).
            (1.005, "một"),
            (2.675, "hai phẩy sáu mươi bảy"),
            // Sub-cent values collapse to ".00": no phẩy tail at all, and the
            // sign still survives via the pre-abs `number < 0` test.
            (0.001, "không"),
            (-0.001, "âm không"),
        ] {
            assert_eq!(float_case(v), want, "float {}", v);
        }
    }

    /// Every `cardinal_dec` corpus row. All go through the same
    /// Decimal→double coercion `"%.2f"` performs in Python.
    #[test]
    fn decimal_cardinals_match_python() {
        for (s, want) in [
            ("0.01", "không phẩy một"),
            ("1.10", "một phẩy mười"),
            // float(Decimal("12.345")) is 12.34500000…06 → "%.2f" → "12.35".
            ("12.345", "mười hai phẩy ba mươi lăm"),
            ("-12.345", "âm mười hai phẩy ba mươi lăm"),
            ("0.001", "không"),
            // Issue #603's value: the double coercion lands on …29.98.
            (
                "98746251323029.99",
                "chín mươi tám nghìn tỷ bảy trăm bốn mươi sáu tỷ hai trăm \
                 năm mươi mốt triệu ba trăm hai mươi ba nghìn lẻ hai mươi \
                 chín phẩy chín mươi tám",
            ),
        ] {
            assert_eq!(dec_case(s), want, "decimal {}", s);
        }
    }

    /// `precision=` is a proven no-op for VI: the dispatcher pops it, the
    /// class has no `precision` attribute, and `"%.2f"` is hard-coded.
    #[test]
    fn precision_override_is_discarded() {
        let got = LangVi::new()
            .to_cardinal_float(
                &FloatValue::Float {
                    value: 12.345,
                    precision: 3,
                },
                Some(1),
            )
            .unwrap();
        assert_eq!(got, "mười hai phẩy ba mươi lăm");
    }

    /// Past 10^60 `vietnam_number` runs out of scale words: OverflowError
    /// for either sign (#159; Python returned None / raised TypeError).
    #[test]
    fn overflow_past_1e60() {
        let lang = LangVi::new();
        let big = "2".to_string() + &"0".repeat(60) + ".5";
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str(&big).unwrap(),
            precision: 1,
        };
        assert!(matches!(
            lang.to_cardinal_float(&v, None),
            Err(N2WError::Overflow(_))
        ));
        let neg = FloatValue::Decimal {
            value: BigDecimal::from_str(&format!("-{}", big)).unwrap(),
            precision: 1,
        };
        assert!(matches!(
            lang.to_cardinal_float(&neg, None),
            Err(N2WError::Overflow(_))
        ));
    }
}
