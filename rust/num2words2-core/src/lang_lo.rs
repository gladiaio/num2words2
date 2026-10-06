//! Port of `lang_LO.py` (Lao).
//!
//! Shape: **self-contained**. `Num2Word_LO` subclasses `Num2Word_Base`, but its
//! `setup()` defines only `negword`, `pointword` and an `ones` list — no
//! `high_numwords`/`mid_numwords`/`low_numwords`. The `any(hasattr(...))` guard
//! in `Num2Word_Base.__init__` therefore never fires, so Python never builds
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int_to_word`, a plain recursive descent over the Lao
//! myriad-style scale (สิบ/ຊາວ/ຮ້ອຍ/ພັນ/ໝື່ນ/ແສນ/ລ້ານ).
//!
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here, and
//! there is **no overflow check at all** — see bug 1 below for what happens
//! instead.
//!
//! Inherited from `Num2Word_Base` and *not* overridden by LO:
//!   * `to_cheque` — the only inherited method that runs. It reads
//!     `CURRENCY_FORMS[currency]` *strictly* (`KeyError` -> `NotImplementedError`)
//!     and `_money_verbose` -> `to_cardinal`, both of which the trait defaults
//!     already provide. See bug 5 for the contradiction this creates with LO's
//!     own `to_currency`.
//!   * `verify_ordinal` is inherited but **never called**, which is why
//!     negatives sail through `to_ordinal` (bug 2).
//!   * `pluralize`, `_cents_verbose` and `_cents_terse` are all unreachable —
//!     LO's `to_currency` never calls them and `to_cheque` does not either — so
//!     their trait defaults (`pluralize` raises) are deliberately left alone.
//!
//! # Currency
//!
//! `CURRENCY_FORMS` is defined on `Num2Word_LO` itself (LAK/USD/EUR), so none of
//! the `lang_EUR`/`Num2Word_EN` shared-dict mutation described in
//! `PORTING_CURRENCY.md` applies here — the live interpreter confirms exactly
//! those three codes. `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both
//! empty (`{}`), so every code is divisor 100 and no adjective exists; the trait
//! defaults cover both and neither hook is overridden.
//!
//! `to_currency` is overridden outright and shares no code with
//! `Num2Word_Base.to_currency`: no `parse_currency_parts`, no divisor, no
//! `pluralize`. It re-splits `str(val)` by hand.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Each of the following is wrong-looking but is
//! exactly what Python emits, and each is pinned by the frozen corpus:
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns the decimal digits.** The
//!    final `else` arm is `return str(number)`, so `to_cardinal(10**9)` is the
//!    ASCII string `"1000000000"`, not Lao words. Every value >= 10^9 is echoed
//!    back as digits, forever — there is no `OverflowError` and no upper bound.
//!    Corpus confirms this out to 10^21. See [`LangLo::int_to_word`].
//! 2. **`to_ordinal` never calls `verify_ordinal`**, so negatives are accepted
//!    and produce `"ທີ່" + to_cardinal(n)` — e.g. `to_ordinal(-1)` ==
//!    `"ທີ່ລົບ ໜຶ່ງ"` ("th-minus one"). Most languages raise `TypeError` here.
//! 3. **`to_ordinal`/`to_ordinal_num` are pure prefixing** — the "ordinal" is
//!    just `"ທີ່"` glued onto the cardinal (or onto `str(number)`), with no
//!    separator and no morphology. So `to_ordinal(10**9)` == `"ທີ່1000000000"`,
//!    inheriting bug 1.
//! 4. **`negword` carries a trailing space** (`"ລົບ "`, not `"ລົບ"`), unlike the
//!    base class's convention of stripping it at the call site. `to_cardinal`
//!    concatenates it raw, which happens to give the right single space —
//!    but it means [`Lang::negword`] here returns a *space-suffixed* word, so a
//!    caller applying the base's usual `"%s " % negword.strip()` idiom would
//!    get the same result only by coincidence. `to_currency` concatenates it
//!    raw too.
//! 5. **`to_currency` and `to_cheque` disagree about unknown currency codes.**
//!    LO's `to_currency` looks the code up *leniently* —
//!    `CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["LAK"])` — so an
//!    unimplemented code silently renders in **Lao kip**, while the inherited
//!    `to_cheque` does a strict `CURRENCY_FORMS[currency]` and raises
//!    `NotImplementedError` for the very same code. The corpus pins both halves:
//!    `currency:GBP` -> `"ສິບສອງ ກີບ ສາມສິບສີ່ ອັດ"` (kip/at, not pounds/pence)
//!    while `cheque:GBP` -> `NotImplementedError`. Same for JPY, KWD, BHD, INR,
//!    CNY and CHF. See [`LangLo::to_currency`].
//! 6. **`to_currency` ignores `CURRENCY_PRECISION` entirely.** It hardcodes a
//!    two-digit subunit via `parts[1][:2]`, so the 3-decimal (KWD/BHD) and
//!    0-decimal (JPY) currencies get plain hundredths like everything else —
//!    and since those codes are not in `CURRENCY_FORMS` anyway they land on the
//!    LAK fallback first. `currency:JPY` of `0.5` is `"ສູນ ກີບ ຫ້າສິບ ອັດ"`, not
//!    the whole-unit rounding `Num2Word_Base.to_currency` would apply. LO's
//!    `CURRENCY_PRECISION` is `{}` regardless, so `to_cheque` uses 100.
//! 7. **A float with zero cents drops the cents segment.** The guard is
//!    `if cents and right`, and `right` is an int, so `1.0` (which yields
//!    `right = 0`) renders `"ໜຶ່ງ ເອີໂຣ"` — identical to the int `1`.
//!    `Num2Word_Base.to_currency` would emit a "zero cents" tail here. The
//!    int/float distinction is still threaded through faithfully (see
//!    [`split_currency`]) because it only *coincidentally* collapses in LO's
//!    output.
//! 8. **`cents=False` omits the cents rather than making them terse**, because
//!    the same `cents and right` guard suppresses the whole segment instead of
//!    switching to `_cents_terse` the way the base class does.
//! 9. **`adjective` is accepted and completely ignored** — declared in the
//!    signature, never read, and `CURRENCY_ADJECTIVES` is empty anyway.
//! 10. **Subunits are truncated, not rounded.** `parts[1][:2]` slices the
//!    decimal string, so `12.349` gives 34 cents (not 35) and `0.001` gives 0.
//!
//! # Non-bugs that look like bugs
//!
//! * The trailing `.strip()` in `to_cardinal` is a no-op for integer input:
//!   every `_int_to_word` arm returns a non-empty, non-padded string and the
//!   only prefix is `""` or `"ລົບ "`. Reproduced anyway ([`str::trim`]) so the
//!   port matches the source line for line.
//! * The `if number < 0` arm of `_int_to_word` is **dead code** on the four
//!   in-scope entry points: `to_cardinal` strips the sign before calling, and
//!   every recursive call passes a non-negative quotient or remainder. It is
//!   reachable only from `to_currency` (out of scope). Ported regardless.
//! * `to_cardinal`'s `"." in n` branch (the `pointword` per-digit path) is
//!   unreachable from `BigInt` input — `str(int)` never contains a dot. The
//!   `BigInt` `to_cardinal` above therefore only runs the integer `else` branch.
//!   The dotted branch *is* ported, in [`LangLo::to_cardinal_float`]: because
//!   `Num2Word_LO` overrides `to_cardinal` and handles non-integers inline via
//!   `str(number)` string-splitting — nothing like `Num2Word_Base`'s
//!   `float2tuple` — the trait's default `to_cardinal_float` is wrong for LO and
//!   is overridden. See that method for the full port and its consequences.
//!
//! # Float routing: every float/Decimal goes through the string surgery
//!
//! `to_cardinal` branches on `"." in str(number)`, never on
//! `int(value) == value`, so a *whole* float still reads its trailing zero:
//! `to_cardinal(5.0)` == `"ຫ້າ ຈຸດ ສູນ"` and `Decimal("5.00")` reads two zero
//! digits, while a point-free `Decimal("5")` takes the integer reading. The
//! base trait's `cardinal_float_entry` (whole -> int path) is therefore
//! overridden to send **all** float/Decimal input through the cascade;
//! `to_ordinal`/`to_year` float entries prefix that same output
//! (`"ທີ່"`/`"ປີ "`), and `to_ordinal_num`'s float entry prefixes the
//! binding-provided `str(value)`.
//!
//! `str(number)` is rebuilt exactly: `str(Decimal)` via
//! [`crate::strnum::python_decimal_str`] (trailing zeros and exponent forms
//! kept — `"5.00"`, `"1E+2"`), and `str(float)` via [`py_float_str`], which
//! mirrors CPython's shortest-repr formatting *including* the switch to
//! exponent form at `|v| >= 1e16` or `0 < |v| < 1e-4`. Exponential text then
//! fails in `int()` exactly as Python's does: `int("1e+16")` / `int("1E+2")`
//! raise ValueError (no dot in the repr), and a mantissa-dotted `1.23e+16`
//! dies on `int("e")` in the digit walk. `Decimal("-0.0")` arrives as
//! `FloatValue::Float { -0.0 }` (the binding converts it — `BigDecimal` has
//! no signed zero) and the sign bit keeps the negword:
//! `"ລົບ ສູນ ຈຸດ ສູນ"`, as pinned.
//!
//! # String input: Infinity/NaN raise ValueError, not OverflowError
//!
//! `num2words("Infinity", lang="lo")` parses to `Decimal("Infinity")` and
//! lands in this `to_cardinal`: `str()` gives `"Infinity"` — no `"."` — and
//! `int("Infinity")` raises **ValueError**. The binding would otherwise map
//! `ParsedNumber::Inf` to the base path's OverflowError before any LO code
//! runs, so [`Lang::str_to_number`] intercepts Inf (and NaN, whose binding
//! message differs from `int("NaN")`'s). Known gap (unpinned): Python's
//! `to_ordinal_num(Decimal("Infinity"))` echoes `"ທີ່Infinity"`, which the
//! entry-level interception turns into the cardinal path's ValueError; the
//! strings corpus pins Infinity/NaN under `to=cardinal` only.
//!
//! # Error variants
//!
//! **The four integer modes raise nothing.** No `MAXVAL` check, no dict lookups,
//! no list indexing that can run out of range (every `ones[...]` index is pinned
//! to 0..=9 by the branch guard that precedes it), and no `int()` of a
//! non-numeric token. Every *integer* corpus row for `lo` in those modes is
//! `"ok": true`. The float/Decimal side of cardinal/ordinal/year raises
//! `Value` — Python's ValueError from `int()` — exactly when `str(number)`
//! is exponential (see the float-routing section above).
//!
//! The currency surface has exactly two:
//!
//! * `NotImplemented` — from the inherited `to_cheque` only, on a code outside
//!   {LAK, USD, EUR}. Message: `Currency code "X" not implemented for
//!   "Num2Word_LO"`. **`to_currency` cannot raise it** (bug 5).
//! * `Value` — from `int()` on an exponent-notation token inside
//!   [`split_currency`]; see that function for the exact conditions and the one
//!   known gap.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.ones`, indices 0..=9. Every read is guarded by a range check that
/// pins the index into this table, so there is no `IndexError` path.
const ONES: [&str; 10] = [
    "ສູນ",   // 0
    "ໜຶ່ງ",  // 1
    "ສອງ",   // 2
    "ສາມ",   // 3
    "ສີ່",    // 4
    "ຫ້າ",   // 5
    "ຫົກ",   // 6
    "ເຈັດ",  // 7
    "ແປດ",   // 8
    "ເກົ້າ",  // 9
];

/// `self.negword`. Note the trailing space — see bug 4 in the module docs.
const NEGWORD: &str = "ລົບ ";

/// `self.pointword`. Reachable only via the float/Decimal path — `str(int)`
/// never contains the dot that gates it.
const POINTWORD: &str = "ຈຸດ";

/// 10 — a standalone word, not `ones[1] + ສິບ`.
const TEN: &str = "ສິບ";
/// 20 — likewise standalone, and the reason 20..=29 gets its own arm.
const TWENTY: &str = "ຊາວ";
/// Scale suffixes: 10^2, 10^3, 10^4, 10^5, 10^6.
const HUNDRED: &str = "ຮ້ອຍ";
const THOUSAND: &str = "ພັນ";
const TEN_THOUSAND: &str = "ໝື່ນ";
const HUNDRED_THOUSAND: &str = "ແສນ";
/// 10^6. Unlike the smaller scales this one is joined with a **leading space**
/// (`" ລ້ານ"`), which is why `to_cardinal(10**6)` == `"ໜຶ່ງ ລ້ານ"` while
/// `to_cardinal(1000)` == `"ໜຶ່ງພັນ"` (no space).
const MILLION: &str = " ລ້ານ";

/// The ordinal prefix, glued on with no separator.
const ORDINAL_PREFIX: &str = "ທີ່";
/// The year prefix, which *does* carry a trailing space.
const YEAR_PREFIX: &str = "ປີ ";

/// `Num2Word_LO.to_currency`'s own default `separator=" "`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot simply be a parameter default.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_LO.to_currency` declares `separator=" "`, but the `Lang` trait has
/// no per-language defaults: `__init__.py`'s currency fast path (and
/// `bench/diff_test.py`) both substitute `kwargs.get("separator", ",")` —
/// **`Num2Word_Base`'s** default — before the value ever reaches Rust. By then
/// "caller omitted separator" and "caller explicitly passed a comma" are the
/// same string, and the information needed to tell them apart no longer exists
/// on this side of the boundary.
///
/// So `,` is read back as the unset sentinel and LO's own default restored.
/// This is the only reading that matches the oracle: every float row of the `lo`
/// currency corpus was generated by `num2words(v, lang="lo", to="currency",
/// currency=c)` with no `separator=`, and every one of them expects `" "` —
/// taking the comma at face value would render `"ສິບສອງ ເອີໂຣ,ສາມສິບສີ່ ເຊັນ"`
/// and fail all 63 of them. It is also what the *shipped* shim needs: a plain
/// `num2words(12.34, lang="lo", to="currency", currency="EUR")` must not start
/// emitting commas just because the fast path took over.
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets `" "` here where Python would give `","`. Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report. This
/// mirrors the convention `lang_ca.rs` / `lang_bo.rs` / `lang_as.rs` established
/// for the same problem.
const SEPARATOR_UNSET: &str = ",";

/// `to_currency`'s fallback code: `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["LAK"])`.
///
/// Note Python evaluates `self.CURRENCY_FORMS["LAK"]` *eagerly* as the `.get`
/// default, so a class that dropped the LAK entry would raise `KeyError` on
/// every call. LO always has it, so the lookup below is infallible.
const FALLBACK_CURRENCY: &str = "LAK";

fn bi(n: u32) -> BigInt {
    BigInt::from(n)
}

/// `self.ones[i]` where the caller has already proven `0 <= i <= 9`.
///
/// The conversion cannot fail: each call site is dominated by a comparison
/// bounding the value to a single decimal digit.
fn ones_at(n: &BigInt) -> &'static str {
    let i = n
        .to_usize()
        .expect("ones index is bounded to 0..=9 by the caller's range check");
    ONES[i]
}

/// The `(left, right)` split inlined at the top of `Num2Word_LO.to_currency`.
///
/// ```text
/// is_negative = False
/// if val < 0:
///     is_negative = True
///     val = abs(val)
///
/// parts = str(val).split(".")
/// left = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// (Byte for byte the body of `Num2Word_BO._split_currency`, which `lang_bo.rs`
/// ports the same way. The sign is handled by the caller; this takes `abs`.)
///
/// # Why this is arithmetic and not string surgery
///
/// The Python operates on `str(val)`, but the string is gone by the time the
/// value reaches Rust — `currency.rs` deliberately keeps `repr(float)` on the
/// Python side and hands over a parsed `BigDecimal`. The arithmetic below is
/// nonetheless *exactly* equivalent, because `BigDecimal` retains the scale of
/// the literal it was parsed from:
///
/// * `int(parts[0])` is the truncated integer part, and `with_scale(0)` divides
///   by a power of ten with `BigInt` division, i.e. truncates toward zero. `val`
///   is already non-negative here, so truncation is the floor.
/// * `int(parts[1][:2].ljust(2, "0"))` is `floor(frac * 100)`. The `[:2]`
///   truncation and the `ljust` zero-padding are both subsumed: the scale
///   carries `"1.0"`'s trailing zero, and truncating at two places is what
///   discarding the third digit does.
///
/// | `str(val)` | `parts[1][:2].ljust(2,"0")` | Python | `floor(frac*100)` |
/// |---|---|---|---|
/// | `"1.0"`   | `"00"` | 0  | 0  |
/// | `"0.5"`   | `"50"` | 50 | 50 |
/// | `"0.01"`  | `"01"` | 1  | 1  |
/// | `"12.34"` | `"34"` | 34 | 34 |
/// | `"99.99"` | `"99"` | 99 | 99 |
/// | `"0.001"` | `"00"` | 0  | 0  |
/// | `"0.567"` | `"56"` | 56 | 56 |
///
/// `parts[0]` is never `""` and `parts[1]` is never `""` for any string Python's
/// `str()` produces, so the two `else 0` fallbacks are dead; they coincide with
/// the arithmetic anyway.
///
/// # The exponent-notation `ValueError`
///
/// `str()` switches to exponent notation for a float outside roughly
/// `1e-4 <= |v| < 1e16`, and `int()` chokes on the resulting token:
/// `str(1e16)` is `"1e+16"`, `parts` is `["1e+16"]`, and `int("1e+16")` raises
/// **`ValueError`** — not `NotImplementedError`, and not anything LO catches. So
/// `to_currency` raises for large floats rather than returning a string.
///
/// The check below keys off the *representation*, which is what Python's
/// `str().split(".")` does too, so it stays faithful for the `Decimal` inputs the
/// shim also accepts: `Decimal("1E+3")` stringifies with an exponent and raises,
/// while `Decimal(1000)` stringifies as plain digits and succeeds — and the two
/// parse to different scales (-3 vs 0), so both land correctly.
///
/// A **positive** exponent is therefore recoverable: plain decimal notation can
/// never yield a negative `BigDecimal` scale, so `scale < 0` proves `str()`
/// emitted an exponent. No false positives are possible.
///
/// Two exponent cases are **not** recoverable. Both are flagged in the port
/// report rather than papered over, and no corpus row exercises either.
///
/// 1. *Negative exponent.* `str(1e-05)` is `"1e-05"` (CPython goes scientific
///    below `1e-4`) and raises `ValueError`, but it parses to the very same
///    `(1, scale=5)` as `Decimal("0.00001")`, which stringifies as `"0.00001"`
///    and does *not* raise — Python returns `"ສູນ ເອີໂຣ"` for it. The two are
///    indistinguishable once across the boundary, so a float `< 1e-4` returns
///    `(0, 0)` here (`"ສູນ ເອີໂຣ"`) where Python raises. Discriminating on the
///    value instead (`|v| < 1e-4`) would fix the float and break the `Decimal`,
///    so the representation-faithful rule is kept.
///
/// 2. *Exponent absorbed by the mantissa's own digits.* When a float's repr
///    carries at least as many fractional digits as its exponent the scale
///    comes out `>= 0` and no error fires — but Python does not error either.
///    It splits the **exponent string** on the dot and parses the pieces:
///    `str(1.2345678901234568e+16).split(".")` is
///    `["1", "2345678901234568e+16"]`, so `left = int("1") = 1` and
///    `right = int("23") = 23`, giving `"ໜຶ່ງ ເອີໂຣ ຊາວສາມ ເຊັນ"` — one euro
///    and twenty-three cents for a value of ~1.2e16. This port instead reads
///    the number it was handed and returns `"12345678901234568 ເອີໂຣ"` (bug 1's
///    digit echo). Reproducing Python's answer would mean re-deriving
///    `repr(float)` in Rust, which `currency.rs` deliberately keeps on the
///    Python side; the divergence only affects floats `>= 1e16`, where Python's
///    own output is nonsense.
fn split_currency(val: &CurrencyValue) -> Result<(BigInt, BigInt)> {
    match val {
        // str(int) never contains "." — parts has length 1, so `right` is 0 and
        // the cents segment is unreachable for a true int.
        CurrencyValue::Int(v) => Ok((v.abs(), BigInt::zero())),
        CurrencyValue::Decimal { value: d, .. } => {
            let d = d.abs();
            if d.as_bigint_and_exponent().1 < 0 {
                // str(n) used exponent notation; int() on that token raises.
                return Err(N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    d
                )));
            }
            let left = d.with_scale(0);
            let frac = &d - &left;
            let right = (frac * BigDecimal::from(100)).with_scale(0);
            Ok((
                left.as_bigint_and_exponent().0,
                right.as_bigint_and_exponent().0,
            ))
        }
    }
}

/// Python's `str(float)` for a finite-or-not f64, sign included.
///
/// CPython's `float.__str__` is the shortest round-trip repr, printed in
/// fixed notation while the decimal point sits in `-3 <= decpt <= 16` and in
/// exponent notation outside it — i.e. e-form iff `|v| >= 1e16` or
/// `0 < |v| < 1e-4` (`str(1e16)` == `"1e+16"`, `str(1e15)` ==
/// `"1000000000000000.0"`, `str(0.0001)` == `"0.0001"`, `str(1e-05)` ==
/// `"1e-05"`).
///
/// * **Fixed form** — `format!("{:.*}", precision, mag)`: `precision` is the
///   repr-derived fractional digit count the binding carried over
///   (`abs(Decimal(repr(v)).as_tuple().exponent)`), and correctly rounding
///   the *exact* f64 to that many places reproduces the shortest-repr digits
///   byte for byte (both sides are IEEE-754 doubles).
/// * **Exponent form** — Rust's `{:e}` is shortest-round-trip too (`"1e16"`,
///   `"9.99e-5"`); reformatted to Python's spelling: explicit `+`, exponent
///   zero-padded to two digits (`1e+16`, `1e-05`, `1e+100`).
/// * **inf/nan** — `str(float("inf"))` == `"inf"`, `str(float("nan"))` ==
///   `"nan"` (CPython prints NaN unsigned regardless of the sign bit).
///
/// The sign comes from the *bit*, not a `< 0` compare: `str(-0.0)` is
/// `"-0.0"`, which is why LO renders the negword for negative zero.
fn py_float_str(value: f64, precision: u32) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    let neg = value.is_sign_negative();
    let mag = value.abs();
    let body = if mag.is_infinite() {
        "inf".to_string()
    } else if mag != 0.0 && !(1e-4..1e16).contains(&mag) {
        // repr picks exponent form. {:e} gives e.g. "1e16" / "9.99e-5".
        let s = format!("{:e}", mag);
        let (mant, exp) = s.split_once('e').expect("{:e} always contains 'e'");
        let e: i32 = exp.parse().expect("{:e} exponent is a valid i32");
        format!("{}e{}{:02}", mant, if e < 0 { '-' } else { '+' }, e.abs())
    } else {
        format!("{:.*}", precision as usize, mag)
    };
    if neg {
        format!("-{}", body)
    } else {
        body
    }
}

/// `str(number)` for the value the dispatcher handed over: `repr(float)` via
/// [`py_float_str`], `str(Decimal)` via the spec port in `strnum` — which
/// keeps trailing zeros (`"1.10"`, leading zeros of `"0.001"` included) and
/// exponent forms (`"1E+2"`), byte for byte.
fn py_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, precision } => py_float_str(*value, *precision),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// `int(token)` on a run of decimal digits. Python only ever hands a non-digit
/// token here through exponent notation (which no corpus row hits and whose LO
/// output is a crash anyway), so a parse failure reproduces the `ValueError`
/// `int()` raises, as [`N2WError::Value`] — never a foreign exception name.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// `int(digit)` on a single fractional character, matching Python's per-digit
/// `int(digit)` in the `for digit in right` loop.
fn py_digit(ch: char) -> Result<BigInt> {
    ch.to_digit(10).map(BigInt::from).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

pub struct LangLo {
    /// `Num2Word_LO.CURRENCY_FORMS`, built once in [`LangLo::new`].
    ///
    /// The binding holds each `LangLo` in a `OnceLock`
    /// (`LO.get_or_init(LangLo::new)`), so this table is constructed exactly
    /// once per process rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangLo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLo {
    pub fn new() -> Self {
        // Num2Word_LO.CURRENCY_FORMS — three codes, each with two identical
        // forms per side. Lao does not inflect for number, so the singular and
        // plural slots carry the same word; the arity is kept at Python's 2
        // because the inherited `to_cheque` reads `cr1[-1]`.
        //
        // This is the class's own dict, not the `lang_EUR`/`Num2Word_EN` shared
        // one — `Num2Word_LO` subclasses `Num2Word_Base` directly and rebinds
        // `CURRENCY_FORMS`, so EN's import-time mutation cannot reach it. The
        // live interpreter confirms exactly these three codes; note EUR's forms
        // are LO's own and are *not* the mutated ("euro", "euros") pair.
        let mut currency_forms = HashMap::new();
        currency_forms.insert("LAK", CurrencyForms::new(&["ກີບ", "ກີບ"], &["ອັດ", "ອັດ"]));
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["ໂດລາ", "ໂດລາ"], &["ເຊັນ", "ເຊັນ"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["ເອີໂຣ", "ເອີໂຣ"], &["ເຊັນ", "ເຊັນ"]),
        );
        LangLo { currency_forms }
    }

    /// Port of `Num2Word_LO._int_to_word`.
    ///
    /// A straight recursive descent, ordered exactly as the Python `if/elif`
    /// chain — the order is load-bearing (`number == 0` is tested *before*
    /// `number < 0`, and the standalone `== 10` / `== 20` arms precede the
    /// general tens arm).
    ///
    /// Beyond 10^9 the Python falls off the end of the chain into
    /// `return str(number)`, echoing the decimal digits. That is bug 1 in the
    /// module docs and is why this returns `String` rather than words for large
    /// input — and why it cannot overflow or raise.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ONES[0].to_string();
        }

        if number.is_negative() {
            // Dead on every in-scope path (see module docs); ported for fidelity.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        } else if number < &bi(10) {
            return ones_at(number).to_string();
        } else if number == &bi(10) {
            return TEN.to_string();
        } else if number < &bi(20) {
            return format!("{}{}", TEN, ones_at(&(number - 10u32)));
        } else if number == &bi(20) {
            return TWENTY.to_string();
        } else if number < &bi(30) {
            return format!("{}{}", TWENTY, ones_at(&(number - 20u32)));
        } else if number < &bi(100) {
            // 30..=99. Python: tens_word = ones[n // 10] + "ສິບ"; append the
            // units word only when non-zero.
            let tens_val = number / 10u32;
            let ones_val = number % 10u32;
            let tens_word = format!("{}{}", ones_at(&tens_val), TEN);
            if ones_val.is_zero() {
                return tens_word;
            } else {
                return format!("{}{}", tens_word, ones_at(&ones_val));
            }
        }

        // The four "ones[scale_digit] + suffix [+ ' ' + recurse(remainder)]"
        // arms. Each divisor's quotient is 1..=9, so `ones_at` is always in
        // range. Note the separator is a space here but the suffix itself is
        // glued directly onto the digit word.
        for (limit, divisor, suffix) in [
            (1_000u32, 100u32, HUNDRED),
            (10_000, 1_000, THOUSAND),
            (100_000, 10_000, TEN_THOUSAND),
            (1_000_000, 100_000, HUNDRED_THOUSAND),
        ] {
            if number < &bi(limit) {
                let scale_val = number / divisor;
                let remainder = number % divisor;
                let mut result = format!("{}{}", ones_at(&scale_val), suffix);
                if !remainder.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&remainder));
                }
                return result;
            }
        }

        if number < &bi(1_000_000_000) {
            // 10^6..10^9-1. Unlike the arms above, the multiplier here is
            // *recursed* (it can be 1..=999) and ລ້ານ carries a leading space.
            let millions_val = number / 1_000_000u32;
            let remainder = number % 1_000_000u32;
            let mut result = format!("{}{}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            return result;
        }

        // Bug 1: `return str(number)` — the digits, verbatim, with no ceiling.
        number.to_string()
    }
}

impl Lang for LangLo {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "LAK"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    /// `self.negword` — space-suffixed, exactly as Python sets it. See bug 4.
    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "ຈຸດ"
    }

    /// Port of `Num2Word_LO.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).strip()`, peels a leading `"-"` into
    /// `ret = self.negword`, then (no `"."` in an int's repr) returns
    /// `(ret + self._int_to_word(int(n))).strip()`.
    ///
    /// The negword is concatenated **raw**, keeping its own trailing space; the
    /// final `.strip()` is a no-op for integers but is reproduced anyway.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };
        // int(n) after stripping "-" is exactly abs(value).
        let words = self.int_to_word(&value.abs());
        Ok(format!("{}{}", ret, words).trim().to_string())
    }

    /// Port of `Num2Word_LO.to_ordinal`: `"ທີ່" + self.to_cardinal(number)`.
    ///
    /// No `verify_ordinal`, so negatives pass straight through (bug 2), and no
    /// separator, so large values give `"ທີ່1000000000"` (bugs 1 + 3).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", ORDINAL_PREFIX, cardinal))
    }

    /// Port of `Num2Word_LO.to_ordinal_num`: `"ທີ່" + str(number)`.
    ///
    /// The sign survives verbatim: `to_ordinal_num(-1)` == `"ທີ່-1"`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, value))
    }

    /// Port of `Num2Word_LO.to_year`: `"ປີ " + self.to_cardinal(val)`.
    ///
    /// Python's signature is `to_year(self, val, longval=True)`, but `longval`
    /// is accepted and then completely ignored — there is no era handling and
    /// no two-part ("nineteen eighty-four") form. Negatives just inherit
    /// `to_cardinal`'s negword: `to_year(-500)` == `"ປີ ລົບ ຫ້າຮ້ອຍ"`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("{}{}", YEAR_PREFIX, cardinal))
    }

    /// Port of `Num2Word_LO.to_cardinal`'s float/Decimal branch — the dotted
    /// path the `BigInt` `to_cardinal` above can never reach (`str(int)` has no
    /// dot).
    ///
    /// `Num2Word_LO` overrides `to_cardinal` (not `to_cardinal_float`) and
    /// handles non-integers *inline* via `str(number)` string-splitting, which
    /// is nothing like `Num2Word_Base.to_cardinal_float`/`float2tuple`. So the
    /// trait's default (`default_to_cardinal_float`) is wrong for LO and this
    /// overrides it.
    ///
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword
    /// else:
    ///     ret = ""
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
    ///     for digit in right:
    ///         ret += self._int_to_word(int(digit)) + " "
    ///     return ret.strip()
    /// else:
    ///     return (ret + self._int_to_word(int(n))).strip()
    /// ```
    ///
    /// # Consequences of splitting `str()` rather than `float2tuple`
    ///
    /// * **No f64 artefact — trap #2 does not apply.** `str(2.675)` is
    ///   `"2.675"`, so the fractional digits are `ຫົກ ເຈັດ ຫ້າ` (6 7 5) taken
    ///   straight from the repr — *not* the `674.999…` that `float2tuple`'s
    ///   `< 0.01` heuristic would rescue back to `675` (JA/TH take that path; LO
    ///   does not). They agree here, but LO computes it from `str()`.
    /// * **Trailing zeros survive.** `str(Decimal("1.10"))` is `"1.10"`, so the
    ///   fraction `"10"` renders `ໜຶ່ງ ສູນ`, digit for digit.
    /// * **`precision=` is ignored.** LO never reads `self.precision`; it splits
    ///   the repr. Verified live: `precision=1` and `precision=5` both leave
    ///   `2.675` -> `ສອງ ຈຸດ ຫົກ ເຈັດ ຫ້າ` unchanged. So `precision_override` is
    ///   dropped here.
    /// * **`_int_to_word`, not `to_cardinal`, converts each part** — but they
    ///   agree: `left`/each digit are non-negative, so the negword branch and
    ///   trailing `.strip()` of `to_cardinal` are no-ops. `int_to_word` is called
    ///   directly to mirror the Python.
    ///
    /// Rebuilding `str(number)`: [`py_str`] — `repr(float)` (exponent form
    /// included, so `str(1e16)` == `"1e+16"` crashes in `int()` exactly as
    /// Python's does) and the spec port of `str(Decimal)`
    /// ([`crate::strnum::python_decimal_str`], so `Decimal("1E+2")` yields
    /// `"1E+2"` and crashes the same way, while `"1.10"`/`"0.001"` keep their
    /// zeros byte for byte).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>, // ignored: LO splits str(number), not precision
    ) -> Result<String> {
        // n = str(number)
        let s = py_str(value);

        // if n.startswith("-"): n = n[1:]; ret = self.negword  else: ret = ""
        let (neg, n) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s.as_str()),
        };
        let mut ret = String::new();
        if neg {
            // "ລົບ " — trailing space, concatenated raw, exactly as Python (bug 4).
            ret.push_str(NEGWORD);
        }

        if let Some((left, right)) = n.split_once('.') {
            // ret += _int_to_word(int(left)) + " " + pointword + " "
            ret.push_str(&self.int_to_word(&py_int(left)?));
            ret.push(' ');
            // Python uses `self.pointword` raw here (not title()); is_title is
            // false anyway, so this matches either way.
            ret.push_str(POINTWORD);
            ret.push(' ');
            // for digit in right: ret += _int_to_word(int(digit)) + " "
            for ch in right.chars() {
                ret.push_str(&self.int_to_word(&py_digit(ch)?));
                ret.push(' ');
            }
            Ok(ret.trim().to_string())
        } else {
            // else: (ret + _int_to_word(int(n))).strip() — reached when str()
            // has no dot: a point-free Decimal ("5" -> integer reading) or an
            // exponent form ("1e+16"/"1E+2", where int() raises ValueError).
            ret.push_str(&self.int_to_word(&py_int(n)?));
            Ok(ret.trim().to_string())
        }
    }

    /// `to_cardinal(float/Decimal)`, whole values included. LO routes on
    /// `"." in str(number)`, never on `int(value) == value`, so a whole float
    /// still reads its point digits (`5.0` -> "ຫ້າ ຈຸດ ສູນ",
    /// `Decimal("5.00")` -> two zero digits) while a point-free `Decimal("5")`
    /// takes the integer reading — both fall out of the same string surgery,
    /// so *everything* is sent there, overriding the base whole -> int route.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `"ທີ່" + self.to_cardinal(number)` — the
    /// prefix lands in front of whatever the cardinal produced, negword and
    /// point digits included ("ທີ່ລົບ ສູນ ຈຸດ ສູນ" for -0.0), and the
    /// cardinal's ValueError on exponential reprs propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            ORDINAL_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `"ທີ່" + str(number)`. `repr_str` is
    /// the binding's Python `str(value)`, so `"ທີ່1e+16"` / `"ທີ່5.00"` echo
    /// exactly — no conversion, no ValueError.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", ORDINAL_PREFIX, repr_str))
    }

    /// `to_year(float/Decimal)`: `"ປີ " + self.to_cardinal(val)`, same
    /// full-cardinal routing (and the same ValueError pass-through).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            YEAR_PREFIX,
            self.cardinal_float_entry(value, None)?
        ))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which LO does not
    /// override. The Inf/NaN interception reproduces what happens *next* on
    /// the pinned path: `to_cardinal(Decimal("Infinity"))` reads
    /// `str(number)` == "Infinity" (canonical capitalization, minus sign
    /// already peeled by the `startswith("-")` branch), finds no ".", and
    /// dies in `int("Infinity")` with ValueError; NaN dies in `int("NaN")`.
    /// The binding otherwise maps `ParsedNumber::Inf` to the base integer
    /// path's OverflowError (and NaN to a differently-worded ValueError)
    /// before any LO code runs, so the exact errors must be raised here.
    ///
    /// Known gap (unpinned): Python's `to_ordinal_num(Decimal("Infinity"))`
    /// echoes `"ທີ່Infinity"`, which this entry-level interception turns into
    /// the cardinal path's ValueError. The strings corpus pins Infinity/NaN
    /// under `to=cardinal` only.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    /// For the inherited `to_cheque`'s
    /// `Currency code "X" not implemented for "Num2Word_LO"` message.
    fn lang_name(&self) -> &str {
        "Num2Word_LO"
    }

    /// `Num2Word_LO.CURRENCY_FORMS[code]`, with `None` for a missing code.
    ///
    /// Only the inherited `to_cheque` reads currency forms through this hook,
    /// and it wants the strict `CURRENCY_FORMS[currency]` lookup that turns a
    /// `KeyError` into `NotImplementedError`. LO's own `to_currency` uses the
    /// *lenient* `.get(currency, CURRENCY_FORMS["LAK"])` instead and so reads
    /// the field directly rather than going through here — see bug 5 and
    /// [`LangLo::to_currency`].
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_LO.to_currency`.
    ///
    /// ```text
    /// def to_currency(self, val, currency="LAK", cents=True, separator=" ",
    ///                 adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["LAK"])
    ///
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + cr1[0]
    ///
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + cr2[0]
    ///
    ///     return (self.negword if is_negative else "") + result
    /// ```
    ///
    /// A complete override — it shares nothing with
    /// `Num2Word_Base.to_currency`. No `parse_currency_parts`, no
    /// `CURRENCY_PRECISION`, no `pluralize`, no `_cents_verbose`/`_cents_terse`,
    /// and **no `NotImplementedError`**: bugs 5-10 in the module docs all live
    /// here, and every one is corpus-confirmed.
    ///
    /// Note `_int_to_word` is called directly rather than `to_cardinal`. Both
    /// agree because `left`/`right` are non-negative by construction, so
    /// `to_cardinal`'s negword branch and trailing `.strip()` are both no-ops.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // bug 9: declared, never read
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore LO's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` — split_currency
        // takes the absolute value itself.
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val)?;

        // CURRENCY_FORMS.get(currency, CURRENCY_FORMS["LAK"]) — lenient, unlike
        // the strict lookup the inherited `to_cheque` performs through
        // `currency_forms()`. This is why `currency:GBP` renders in kip while
        // `cheque:GBP` raises (bug 5).
        let forms = self
            .currency_forms
            .get(currency)
            .or_else(|| self.currency_forms.get(FALLBACK_CURRENCY))
            .expect("CURRENCY_FORMS always carries the LAK fallback entry");

        // cr1[0] / cr2[0]: the *first* form. LO never reaches for the plural
        // slot (only `to_cheque`'s `cr1[-1]` does), and both slots are equal.
        let mut result = format!("{} {}", self.int_to_word(&left), forms.unit[0]);

        // `cents and right` — `right` is an int, so 0 is falsy and the whole
        // segment vanishes. Bugs 7 and 8.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        // `self.negword` raw, not `"%s " % negword.strip()` — bug 4. Same
        // result either way: the literal already ends in exactly one space.
        Ok(if is_negative {
            format!("{}{}", NEGWORD, result)
        } else {
            result
        })
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;

    fn f(value: f64, precision: u32) -> String {
        LangLo::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn d(s: &str) -> String {
        let bd = BigDecimal::from_str(s).unwrap();
        let precision = bd.as_bigint_and_exponent().1.max(0) as u32;
        LangLo::new()
            .to_cardinal_float(&FloatValue::Decimal { value: bd, precision }, None)
            .unwrap()
    }

    #[test]
    fn corpus_float_rows() {
        assert_eq!(f(0.0, 1), "ສູນ ຈຸດ ສູນ");
        assert_eq!(f(0.5, 1), "ສູນ ຈຸດ ຫ້າ");
        assert_eq!(f(1.0, 1), "ໜຶ່ງ ຈຸດ ສູນ");
        assert_eq!(f(1.5, 1), "ໜຶ່ງ ຈຸດ ຫ້າ");
        assert_eq!(f(2.25, 2), "ສອງ ຈຸດ ສອງ ຫ້າ");
        assert_eq!(f(3.14, 2), "ສາມ ຈຸດ ໜຶ່ງ ສີ່");
        assert_eq!(f(0.01, 2), "ສູນ ຈຸດ ສູນ ໜຶ່ງ");
        assert_eq!(f(0.1, 1), "ສູນ ຈຸດ ໜຶ່ງ");
        assert_eq!(f(0.99, 2), "ສູນ ຈຸດ ເກົ້າ ເກົ້າ");
        assert_eq!(f(1.01, 2), "ໜຶ່ງ ຈຸດ ສູນ ໜຶ່ງ");
        assert_eq!(f(12.34, 2), "ສິບສອງ ຈຸດ ສາມ ສີ່");
        assert_eq!(f(99.99, 2), "ເກົ້າສິບເກົ້າ ຈຸດ ເກົ້າ ເກົ້າ");
        assert_eq!(f(100.5, 1), "ໜຶ່ງຮ້ອຍ ຈຸດ ຫ້າ");
        assert_eq!(f(1234.56, 2), "ໜຶ່ງພັນ ສອງຮ້ອຍ ສາມສິບສີ່ ຈຸດ ຫ້າ ຫົກ");
        assert_eq!(f(-0.5, 1), "ລົບ ສູນ ຈຸດ ຫ້າ");
        assert_eq!(f(-1.5, 1), "ລົບ ໜຶ່ງ ຈຸດ ຫ້າ");
        assert_eq!(f(-12.34, 2), "ລົບ ສິບສອງ ຈຸດ ສາມ ສີ່");
        // f64-artefact cases: str() gives the exact repr digits, no float2tuple.
        assert_eq!(f(1.005, 3), "ໜຶ່ງ ຈຸດ ສູນ ສູນ ຫ້າ");
        assert_eq!(f(2.675, 3), "ສອງ ຈຸດ ຫົກ ເຈັດ ຫ້າ");
    }

    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(d("0.01"), "ສູນ ຈຸດ ສູນ ໜຶ່ງ");
        assert_eq!(d("1.10"), "ໜຶ່ງ ຈຸດ ໜຶ່ງ ສູນ");
        assert_eq!(d("12.345"), "ສິບສອງ ຈຸດ ສາມ ສີ່ ຫ້າ");
        assert_eq!(d("98746251323029.99"), "98746251323029 ຈຸດ ເກົ້າ ເກົ້າ");
        assert_eq!(d("0.001"), "ສູນ ຈຸດ ສູນ ສູນ ໜຶ່ງ");
    }

    #[test]
    fn precision_override_ignored() {
        // LO splits str(number); it never reads self.precision.
        assert_eq!(
            LangLo::new()
                .to_cardinal_float(&FloatValue::Float { value: 2.675, precision: 3 }, Some(1))
                .unwrap(),
            "ສອງ ຈຸດ ຫົກ ເຈັດ ຫ້າ"
        );
    }

    #[test]
    fn whole_floats_keep_their_point_digits() {
        let lo = LangLo::new();
        // "." in str(5.0) -> the float grammar, never the int path.
        assert_eq!(
            lo.cardinal_float_entry(&FloatValue::Float { value: 5.0, precision: 1 }, None)
                .unwrap(),
            "ຫ້າ ຈຸດ ສູນ"
        );
        // str(-0.0) begins "-": the sign bit keeps the negword.
        assert_eq!(
            lo.cardinal_float_entry(&FloatValue::Float { value: -0.0, precision: 1 }, None)
                .unwrap(),
            "ລົບ ສູນ ຈຸດ ສູນ"
        );
        // Decimal("5.00") reads both trailing zeros; Decimal("5") does not.
        let d500 = FloatValue::Decimal {
            value: BigDecimal::from_str("5.00").unwrap(),
            precision: 2,
        };
        assert_eq!(lo.cardinal_float_entry(&d500, None).unwrap(), "ຫ້າ ຈຸດ ສູນ ສູນ");
        let d5 = FloatValue::Decimal {
            value: BigDecimal::from_str("5").unwrap(),
            precision: 0,
        };
        assert_eq!(lo.cardinal_float_entry(&d5, None).unwrap(), "ຫ້າ");
    }

    #[test]
    fn float_entries_prefix_like_python() {
        let lo = LangLo::new();
        let v = FloatValue::Float { value: 5.0, precision: 1 };
        assert_eq!(lo.ordinal_float_entry(&v).unwrap(), "ທີ່ຫ້າ ຈຸດ ສູນ");
        assert_eq!(lo.year_float_entry(&v).unwrap(), "ປີ ຫ້າ ຈຸດ ສູນ");
        assert_eq!(lo.ordinal_num_float_entry(&v, "5.0").unwrap(), "ທີ່5.0");
    }

    #[test]
    fn exponential_reprs_raise_value_error() {
        let lo = LangLo::new();
        // str(1e16) == "1e+16": no ".", int("1e+16") -> ValueError.
        let e16 = FloatValue::Float { value: 1e16, precision: 16 };
        assert!(matches!(
            lo.cardinal_float_entry(&e16, None),
            Err(N2WError::Value(_))
        ));
        // #211: str(Decimal("1E+2")) is written out ("100"), so it reads.
        let d1e2 = FloatValue::Decimal {
            value: BigDecimal::from_str("1E+2").unwrap(),
            precision: 2,
        };
        assert_eq!(
            lo.ordinal_float_entry(&d1e2).unwrap(),
            lo.to_ordinal(&BigInt::from(100)).unwrap()
        );
        // ordinal_num echoes the repr instead of converting.
        assert_eq!(lo.ordinal_num_float_entry(&e16, "1e+16").unwrap(), "ທີ່1e+16");
    }

    #[test]
    fn infinity_string_raises_value_error() {
        let lo = LangLo::new();
        assert!(matches!(lo.str_to_number("Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(lo.str_to_number("-Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(lo.str_to_number("NaN"), Err(N2WError::Value(_))));
        // Plain numbers still parse through the base Decimal grammar.
        assert!(matches!(lo.str_to_number("1.5"), Ok(ParsedNumber::Dec(_))));
    }
}
