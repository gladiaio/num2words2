//! Port of `lang_FF.py` (Fulah — Pulaar/Fulfulde).
//!
//! Shape: **self-contained**. `Num2Word_FF` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `MAXVAL`. `to_cardinal` is overridden outright and drives the recursive
//! `_int_to_word` helper. Consequently `cards`/`merge` stay at their trait
//! defaults here. Python has **no overflow check** — see the "10^9 cliff"
//! note below; this port raises `OverflowError` from 10^12 (`maxval`).
//!
//! Inherited from `Num2Word_Base`, but every in-scope entry point is
//! overridden by `Num2Word_FF`, so nothing from the base engine is reachable:
//!   * `to_cardinal`     — overridden (below)
//!   * `to_ordinal`      — overridden: `to_cardinal(n) + "ɓal"`
//!   * `to_ordinal_num`  — overridden: `str(n) + "ɓal"`
//!   * `to_year`         — overridden: `to_cardinal(val)`, ignoring `longval`
//!     and the base class's whole AD/BC + two-halves-of-the-year machinery.
//!
//! Notably `Num2Word_FF.to_cardinal` never calls `self.title()`, so the
//! `is_title`/`exclude_title` pathway in the base class is dead code for this
//! language. `setup()` does populate `exclude_title = ["e", "feccere",
//! "less"]`, and it is reproduced below for fidelity, but `is_title` is left
//! at the `False` that `__init__` assigns, so `title()` would be an identity
//! function even if something did reach it.
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following all look wrong but are
//! exactly what Python emits, and are confirmed by the frozen corpus:
//!
//! 1. **The 10^9 cliff (fixed, gladiaio/num2words2#147).** Python's
//!    `_int_to_word` handles 0, <10, <100, <1000, <10^6 and <10^9, then falls
//!    off the end with a bare `return str(number)`, so `cardinal(10**9)` was
//!    `"1000000000"` and `to_ordinal(10**9)` `"1000000000ɓal"`. This port
//!    adds miliyaar (10^9; Pulaar usage, e.g. "138 miliyaar" at
//!    https://pulaar.org/2015/05/05/kariim-wad-kasoo-duu%C9%93i-6-e-138-miliyaar-2/),
//!    composed exactly like the million branch: `cardinal(10**9)` ==
//!    `"miliyaar go'o"`. No 10^12 word is attested, so 10^12 raises
//!    `OverflowError` (`maxval`).
//! 2. **`negword` is the English word "less "**, not a Fulah word, and it is
//!    concatenated *without* a separating join — the trailing space in the
//!    literal is what separates it. Hence `cardinal(-1)` == `"less go'o"`.
//!    The `.strip()` Python applies afterwards is a no-op for every integer
//!    input (nothing produces leading/trailing whitespace), but it is
//!    reproduced anyway.
//! 3. **The millions branch has no `m > 1` guard**, unlike the hundreds and
//!    thousands branches. `100` → `"teemerre"` and `1000` → `"ujunere"` (bare,
//!    no "go'o"), but `1000000` → `"miliyon go'o"` (with "go'o"). Corpus
//!    confirms all three (the word order since changed, see bug 4). The asymmetry is deliberate-looking enough that it
//!    may be intended, but it is preserved verbatim regardless.
//! 4. **The millions branch put the count first (fixed,
//!    gladiaio/num2words2#263).** `200` is `"teemerre ɗiɗi"` (literally
//!    "hundred two") and `10000` is `"ujunere sappo"` ("thousand ten") — the
//!    count follows the unit — but Python's millions branch wrote
//!    `"go'o miliyon"`, so both orders coexisted in one output. Fula numerals
//!    follow the noun they count, scale nouns included: CLDR's Fula
//!    spell-out rules (`common/rbnf/ff.xml`: `miliyo <<`, `miliyaari <<`)
//!    and Pulaar press usage ("miliyaaruuji ɗiɗi", pulaar.org) put the count
//!    after miliyon/miliyaar. So 10^6 is `"miliyon go'o"`, 10^9
//!    `"miliyaar go'o"`, and `cardinal(123456789)` begins
//!    `"miliyon teemerre e noogaas e tati e ujunere teemerre nayi ..."`.
//!    The scale noun stays singular after numbers above one, as CLDR has it
//!    and as `ujunere ɗiɗi` already was.
//!
//! # Float / Decimal routing
//!
//! `to_cardinal` turns on `str(number)`, so a float/Decimal is routed by the
//! *string* form, never by base's `int(value) == value` whole-value test:
//!
//! * Every repr with a visible `"."` takes the `pointword` branch — **whole
//!   values included**: `5.0` → `"jowi feccere sufri"`, `Decimal("5.00")` →
//!   `"jowi feccere sufri sufri"`. [`Lang::cardinal_float_entry`] is therefore
//!   overridden to bypass the base whole→int routing entirely.
//! * An exponent-form repr has no `"."` and lands in `int(n)`, which raises
//!   `ValueError: invalid literal for int() with base 10: '1e+16'`
//!   (`str(1e16)` == `"1e+16"`; likewise `str(Decimal("1E+2"))` == `"1E+2"`).
//!   Reproduced via [`python_float_repr`] and
//!   [`crate::strnum::python_decimal_str`].
//! * `-0.0` keeps its negword because the sign lives in the *string*
//!   (`"-0.0"` startswith `"-"`): `"less sufri feccere sufri"`. The binding
//!   hands `Decimal("-0.0")` over as a signed-zero float for the same reason
//!   (a `BigDecimal` cannot carry the sign of zero), so both spellings agree.
//! * A whole `Decimal` with **no** fractional digits has no point in its str
//!   (`str(Decimal("5"))` == `"5"`) and takes the integer branch — which is
//!   why `Decimal("5")` is `"jowi"` while `5.0` is `"jowi feccere sufri"`.
//! * `to_ordinal`/`to_year` glue onto `to_cardinal`, so floats follow the
//!   same routing (`to_ordinal(5.0)` == `"jowi feccere sufriɓal"`,
//!   `to_ordinal(1e16)` raises), and `to_ordinal_num` suffixes the raw repr
//!   (`"5.0ɓal"`, `"1e+16ɓal"` — *not* an error there).
//! * `num2words("Infinity")` parses to `Decimal("Infinity")` on the Python
//!   side and only fails *inside* FF's `int("Infinity")` — **ValueError**,
//!   not the base path's OverflowError. The Rust dispatcher hard-codes
//!   `ParsedNumber::Inf` → OverflowError before any language hook runs, so
//!   [`Lang::str_to_number`] is overridden to surface the ValueError at
//!   parse time instead. Known divergence: `to_ordinal_num("Infinity")`
//!   would be `"Infinityɓal"` in Python and ValueError here — no corpus row.
//!
//! # Currency
//!
//! `Num2Word_FF` declares its **own** `CURRENCY_FORMS` class attribute and
//! descends from `Num2Word_Base`, not `Num2Word_EUR` — so it is untouched by
//! the `Num2Word_EN.__init__` mutation that rewrites `Num2Word_EUR`'s shared
//! class dict in place. Confirmed against the live interpreter *after* a full
//! `import num2words2` (which instantiates `Num2Word_EN()` and fires that
//! mutation):
//!
//! ```text
//! {'EUR': (('yero', 'yero'), ('santiim', 'santiim')),
//!  'USD': (('dolaar', 'dolaar'), ('santiim', 'santiim')),
//!  'XOF': (('seefaa', 'seefaa'), ('santiim', 'santiim'))}
//! ```
//!
//! Both forms of every entry are identical, so the singular/plural choice is
//! unobservable in the output — but the arity (2) is load-bearing, because
//! `to_currency` indexes `cr1[1]` directly rather than going through
//! `pluralize`.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited from
//! `Num2Word_Base` and are `{}` at runtime, so `currency_adjective` stays
//! `None` and `currency_precision` stays 100 for **every** code — JPY, KWD and
//! BHD included. (`Num2Word_FF.CURRENCY_PRECISION` *is* the Base dict object,
//! so an in-place mutation elsewhere would leak into it; the live check says
//! nothing performs one — `Num2Word_EN.__init__` *rebinds* rather than mutates,
//! creating an instance attribute that never reaches the class dict.) FF's
//! `to_currency` never consults precision at all, so base's 0-decimal and
//! 3-decimal branches are unreachable here. The corpus pins this: `currency:JPY
//! 12.34` still shows santiim, and `currency:KWD 12.34` slices two subunit
//! digits, not three.
//!
//! `to_currency` is overridden **wholesale**: no `parse_currency_parts`, no
//! `ROUND_HALF_UP` quantize, no per-currency divisor, no `pluralize`, and — the
//! big one — **no `NotImplementedError`**. It resolves the code with
//! `.get(currency, list(self.CURRENCY_FORMS.values())[0])`, so an unknown code
//! silently borrows **XOF**'s forms (the first entry of the class-body literal).
//!
//! `to_cheque` is **not** overridden, so `currency::default_to_cheque` (the port
//! of `Num2Word_Base.to_cheque`) serves it. That one subscripts strictly —
//! `self.CURRENCY_FORMS[currency]` — so it *does* raise NotImplementedError.
//! Both halves of the asymmetry are corpus-pinned: `currency:GBP 0` →
//! `"sufri seefaa"`, `cheque:GBP 1234.56` → NotImplementedError.
//!
//! `_money_verbose`, `_cents_verbose` and `_cents_terse` are inherited
//! unchanged. Only `_money_verbose` is reachable (from `to_cheque`), and it
//! delegates to FF's `to_cardinal` — which is why a cheque at or above 10^9
//! printed bare digits in Python (bug 1); here it gets miliyaar, and raises
//! `OverflowError` from 10^12.
//!
//! # Further faithfully reproduced Python behaviour (currency)
//!
//! 5. **`to_currency` splits `str(val)`; it does no arithmetic.** `right` is
//!    `int(parts[1][:2].ljust(2, "0"))` — the first two *characters* of the
//!    fraction, right-padded to two. So `0.5` → `"5"` → `"50"` → 50 santiim,
//!    while `0.05` → `"05"` → 5. It truncates rather than rounds: `12.349` →
//!    34, `0.005` → 0, `1.999` → 99. Corpus: `currency:EUR 0.5` →
//!    `"sufri yero capanɗe jowi santiim"`.
//! 6. **`1.0` prints no cents.** Its fraction `"0"` pads to `"00"`, so
//!    `right == 0` and `if cents and right:` drops the clause entirely. A float
//!    that happens to be whole therefore renders exactly like an int — the one
//!    place FF agrees with base's `isinstance(val, int)` branch, reached by a
//!    completely different route. Corpus: `currency:EUR 1.0` → `"go'o yero"`.
//! 7. **`cents=False` drops the subunit segment outright**, rather than
//!    emitting zero-padded digits the way base's `_cents_terse` would. The flag
//!    is only ever read as `cents and right`.
//! 8. **`adjective` is declared and never read.** `CURRENCY_ADJECTIVES` is
//!    empty anyway, so even base's behaviour would be a no-op.
//! 9. **The separator is concatenated raw** (`result += separator + ...`), with
//!    no space of its own. FF's own signature default is `" "` — *not* Base's
//!    `","` — and that is what supplies the gap; an explicit `separator=","`
//!    yields `"...yero,capanɗe tati e nayi santiim"`.
//! 10. **`negword` is prepended un-stripped** here: `self.negword + result`.
//!    The trailing space in `"less "` supplies the gap and the trailing
//!    `.strip()` is a no-op — reproduced rather than assumed away.
//!
//! # Error variants
//!
//! For integer input this language has **no reachable exception path**: every
//! list index is guarded (`ONES[0]`/`TENS[0]` are the empty string, but the
//! `number == 0` early return means they are never emitted), the 10^9 fallback
//! swallows what would otherwise be an overflow, and no dict lookup occurs. All
//! four in-scope modes return `Ok` for every `BigInt`.
//!
//! The currency surface adds exactly two:
//!
//! * `NotImplemented` — `to_cheque` on a code outside {XOF, USD, EUR}, with
//!   Python's message verbatim. `to_currency` never raises it (bug: it falls
//!   back to XOF).
//! * `Value` — `int()` in `to_currency` choking on an exponent-notation literal
//!   (`int("1e+21")`). See `parse_int` and the port report for the half of this
//!   that is *not* reproducible across the `CurrencyValue` boundary.
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_FF` defines no `str_to_number` override and stashes no
//! flags between calls; every method is a pure function of its argument. The
//! stateless Rust path is safe to dispatch to.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `setup(): self.ones`. Index 0 is the empty string, matching Python; it is
/// unreachable from `int_to_word` because `number == 0` returns "sufri" first
/// and every other index path guards against a zero digit.
const ONES: [&str; 10] = [
    "", "go'o", "ɗiɗi", "tati", "nayi", "jowi", "jeegoo", "jeeɗiɗi", "jeetati", "jeenayi",
];

/// `setup(): self.tens`. Index 0 is the empty string (unreachable: `number
/// < 10` is handled before the tens branch). Note 10 and 20 are suppletive
/// ("sappo", "noogaas") while 30..90 are regular `"capanɗe " + ONES[n]`.
const TENS: [&str; 10] = [
    "",
    "sappo",
    "noogaas",
    "capanɗe tati",
    "capanɗe nayi",
    "capanɗe jowi",
    "capanɗe jeegoo",
    "capanɗe jeeɗiɗi",
    "capanɗe jeetati",
    "capanɗe jeenayi",
];

/// `setup(): self.hundred`.
const HUNDRED: &str = "teemerre";
/// `setup(): self.thousand`.
const THOUSAND: &str = "ujunere";
/// `setup(): self.million`.
const MILLION: &str = "miliyon";
/// 10^9, absent from Python (gladiaio/num2words2#147): Pulaar "miliyaar".
const BILLION: &str = "miliyaar";

/// The exclusive ceiling (#147): no attested Pulaar word for 10^12.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}
/// `setup(): self.negword`. The trailing space is load-bearing — Python
/// concatenates it directly (`self.negword + self.to_cardinal(...)`) with no
/// separator of its own.
const NEGWORD: &str = "less ";
/// `setup(): self.pointword`. Only reachable on the float path, which is out
/// of scope; carried for fidelity.
const POINTWORD: &str = "feccere";
/// `_int_to_word(0)`. Also the per-digit word for `0` on the float path.
const ZERO_WORD: &str = "sufri";
/// The ordinal suffix, appended with **no** separating space by both
/// `to_ordinal` and `to_ordinal_num`.
const ORDINAL_SUFFIX: &str = "ɓal";

/// Port of `Num2Word_FF._int_to_word`.
///
/// Callers only ever reach this with a non-negative value: `to_cardinal`
/// strips the sign before recursing, and every internal recursion passes a
/// quotient or remainder of a non-negative dividend. `div_rem` therefore
/// agrees with Python's `divmod` (they differ only for negative operands).
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    // if number == 0: return "sufri"
    if number.is_zero() {
        return Ok(ZERO_WORD.to_string());
    }

    // if number < 10: return self.ones[number]
    if *number < BigInt::from(10) {
        let i = number
            .to_usize()
            .expect("value proven < 10 by the guard above");
        return Ok(ONES[i].to_string());
    }

    // if number < 100:
    //     t, o = divmod(number, 10)
    //     return self.tens[t] + (" e " + self.ones[o] if o else "")
    if *number < BigInt::from(100) {
        let (t, o) = number.div_rem(&BigInt::from(10));
        let t = t.to_usize().expect("quotient of a value < 100 by 10 is < 10");
        let o = o.to_usize().expect("remainder mod 10 is < 10");
        let mut s = TENS[t].to_string();
        if o != 0 {
            s.push_str(" e ");
            s.push_str(ONES[o]);
        }
        return Ok(s);
    }

    // if number < 1000:
    //     h, r = divmod(number, 100)
    //     base = self.hundred + (" " + self.ones[h] if h > 1 else "")
    //     return base + (" e " + self._int_to_word(r) if r else "")
    if *number < BigInt::from(1000) {
        let (h, r) = number.div_rem(&BigInt::from(100));
        let h = h
            .to_usize()
            .expect("quotient of a value < 1000 by 100 is < 10");
        let mut s = HUNDRED.to_string();
        // NB: `h > 1`, so 100 is bare "teemerre" with no "go'o".
        if h > 1 {
            s.push(' ');
            s.push_str(ONES[h]);
        }
        if !r.is_zero() {
            s.push_str(" e ");
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // if number < 1000000:
    //     t, r = divmod(number, 1000)
    //     base = self.thousand + (" " + self._int_to_word(t) if t > 1 else "")
    //     return base + (" e " + self._int_to_word(r) if r else "")
    if *number < BigInt::from(1_000_000) {
        let (t, r) = number.div_rem(&BigInt::from(1000));
        let mut s = THOUSAND.to_string();
        // NB: `t > 1`, so 1000 is bare "ujunere" with no "go'o".
        if t > BigInt::one() {
            s.push(' ');
            s.push_str(&int_to_word(&t)?);
        }
        if !r.is_zero() {
            s.push_str(" e ");
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // if number < 1000000000:
    //     m, r = divmod(number, 1000000)
    //     base = self._int_to_word(m) + " " + self.million
    //     return base + (" e " + self._int_to_word(r) if r else "")
    if *number < BigInt::from(1_000_000_000) {
        let (m, r) = number.div_rem(&BigInt::from(1_000_000));
        // NB: no `m > 1` guard here, unlike the two branches above — this is
        // why 10**6 is "miliyon go'o" but 10**3 is bare "ujunere". The count
        // follows the scale noun, as it does for ujunere/teemerre (#263).
        let mut s = MILLION.to_string();
        s.push(' ');
        s.push_str(&int_to_word(&m)?);
        if !r.is_zero() {
            s.push_str(" e ");
            s.push_str(&int_to_word(&r)?);
        }
        return Ok(s);
    }

    // return str(number)
    //
    // The 10^9 cliff in Python (bug 1). The miliyaar branch instead, composed
    // like the million one (#147); the maxval check above keeps `m` < 1000.
    let (m, r) = number.div_rem(&BigInt::from(1_000_000_000));
    let mut s = BILLION.to_string();
    s.push(' ');
    s.push_str(&int_to_word(&m)?);
    if !r.is_zero() {
        s.push_str(" e ");
        s.push_str(&int_to_word(&r)?);
    }
    Ok(s)
}

/// Python's `int(s)` on a fragment of `str(val)`, inside `to_currency`.
///
/// `int()` accepts only a plain digit string, so an exponent-notation literal
/// raises ValueError rather than parsing. Python quotes the offending literal
/// in the message, and `BigDecimal`'s own rendering supplies the same text for
/// the `e+` case (see `to_currency`), so the message survives intact too.
fn parse_int(s: &str) -> Result<BigInt> {
    s.parse::<BigInt>()
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// Python's `int(digit)` on a *single* character of the fractional string,
/// inside `Num2Word_FF.to_cardinal`'s float branch.
///
/// `int('5')` -> 5; `int('e')` raises `ValueError` with the character quoted.
/// This is how a `str(number)` in scientific notation (`"1.5e+16"`) blows up:
/// the `'e'` is fed to `int()` per-digit. `char::to_digit(10)` accepts exactly
/// the ASCII `0-9` that our reconstructed decimal string ever contains; any
/// other character reproduces Python's message verbatim.
fn digit_word(ch: char) -> Result<&'static str> {
    match ch.to_digit(10) {
        // `self.ones[int(digit)] or "sufri"` — ONES[0] is "" (falsy) -> "sufri".
        Some(0) => Ok(ZERO_WORD),
        Some(d) => Ok(ONES[d as usize]),
        None => Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            ch
        ))),
    }
}

/// Port of `Num2Word_FF.to_cardinal` on the *string* it produced from
/// `str(number)`. Reproduces the method verbatim, recursion and all:
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     return (self.negword + self.to_cardinal(n[1:])).strip()
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "sufri")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// The float/Decimal dispatcher reconstructs `str(number)` (see
/// `to_cardinal_float`) and hands it here; `s` is already whitespace-free, so
/// the leading `.strip()` is a no-op. `split(".", 1)` keeps only the first two
/// parts, matching `split_once`.
fn to_cardinal_from_str(s: &str) -> Result<String> {
    // if n.startswith("-"): (self.negword + self.to_cardinal(n[1:])).strip()
    if let Some(rest) = s.strip_prefix('-') {
        let inner = to_cardinal_from_str(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }
    // if "." in n:
    if let Some((left, right)) = s.split_once('.') {
        // ret = self._int_to_word(int(left)) + " " + self.pointword
        let left_int = parse_int(left)?;
        let mut ret = format!("{} {}", int_to_word(&left_int)?, POINTWORD);
        // for digit in right: ret += " " + (self.ones[int(digit)] or "sufri")
        for ch in right.chars() {
            ret.push(' ');
            ret.push_str(digit_word(ch)?);
        }
        return Ok(ret.trim().to_string());
    }
    // return self._int_to_word(int(n))
    int_to_word(&parse_int(s)?)
}

/// Reconstruct Python's `str(number)` from a `FloatValue`.
///
/// `Num2Word_FF.to_cardinal` does no float arithmetic at all — it `str()`s the
/// input and splits on `"."`. So this port must reproduce `str`, **not**
/// `base.float2tuple`: the base path's `abs(value-pre)*10**precision` loses a
/// low-order digit at large magnitude (e.g. `55258828724.79639` -> base yields
/// `...jeetati`, FF's `str` yields `...jeenayi`). Both branches below rebuild
/// the exact decimal `repr`, so no rounding heuristic is involved.
fn float_to_str(v: &FloatValue) -> String {
    match v {
        // `str(float)`: shortest round-trip repr with exactly `precision`
        // fractional digits. `format!("{:.*}", precision, mag)` reproduces it
        // for the fixed-notation range (verified digit-for-digit against
        // CPython over 5e5 samples). `precision` is the repr's own fraction
        // length, so re-formatting to that many places recovers the same
        // digits without a genuine rounding boundary. Scientific-notation
        // magnitudes are out of range here — see the port report.
        FloatValue::Float { value, precision } => {
            let neg = *value < 0.0 || (*value == 0.0 && value.is_sign_negative());
            let body = format!("{:.*}", *precision as usize, value.abs());
            format!("{}{}", if neg { "-" } else { "" }, body)
        }
        // `str(Decimal)`: fixed-point with exactly `precision` fractional
        // digits, trailing zeros preserved (`Decimal("1.10")` -> `"1.10"`, not
        // `"1.1"`). Forcing the scale to `precision` restores any trailing zero
        // the `BigDecimal` may have normalised away, then the mantissa digits
        // are placed around a decimal point by hand — avoiding any dependence
        // on `BigDecimal`'s own `Display`.
        FloatValue::Decimal { value, precision } => {
            let p = *precision as usize;
            let (mant, _scale) = value.with_scale(*precision as i64).as_bigint_and_exponent();
            let neg = mant.is_negative();
            let mut digits = mant.abs().to_string(); // ASCII 0-9, no sign
            let body = if p == 0 {
                digits
            } else {
                // ljust the *integer* side: need at least one digit before the
                // point, i.e. len > p.
                while digits.len() <= p {
                    digits.insert(0, '0');
                }
                let split = digits.len() - p;
                format!("{}.{}", &digits[..split], &digits[split..])
            };
            format!("{}{}", if neg { "-" } else { "" }, body)
        }
    }
}

/// Python `repr()` of a **non-negative** f64 — the string FF's `to_cardinal`
/// sees after the sign is detached. Rust's `{}` produces the same
/// shortest-round-trip digits but never switches to exponent form and drops
/// the ".0" of whole values, so the two Python-isms are reapplied here:
///
/// * exponent form for `|v| >= 1e16` or `0 < |v| < 1e-4`, with the sign
///   always shown and the exponent zero-padded to two digits (`"1e+16"`,
///   `"1.5e-05"`) — exactly repr's thresholds;
/// * a trailing `".0"` for whole values in positional form (`"5.0"`).
///
/// inf/nan come back as `"inf"`/`"nan"`, which `parse_int` then rejects with
/// the same ValueError Python's `int()` would raise.
fn python_float_repr_abs(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return "inf".to_string();
    }
    if f != 0.0 && (f >= 1e16 || f < 1e-4) {
        let s = format!("{:e}", f); // "1e16" / "1.2345e-7"
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("f64 exponent is a small integer");
        return format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }
    let s = format!("{}", f);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

/// Python's `str(number)` for either `FloatValue` arm, sign included — the
/// exact string `Num2Word_FF.to_cardinal` receives. Float goes through the
/// repr rules above (with the sign-bit-aware minus, so `-0.0` keeps its
/// sign); Decimal is the spec `to-scientific-string` algorithm, which is why
/// `Decimal("1E+2")` reads back `"1E+2"` and dies in `int()` exactly like
/// Python.
fn py_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value: f, .. } => {
            let body = python_float_repr_abs(f.abs());
            if f.is_sign_negative() {
                format!("-{}", body)
            } else {
                body
            }
        }
        FloatValue::Decimal { value: d, .. } => python_decimal_str(d),
    }
}

/// Fulah (Pulaar/Fulfulde) — `CONVERTER_CLASSES["ff"]` → `Num2Word_FF`.
///
/// The numeral tables are module constants (the converter holds no mutable
/// state across calls — see the module docs); only `CURRENCY_FORMS` needs to
/// live on the instance, because it is a map and Python's fallback depends on
/// its *insertion order*.
pub struct LangFf {
    /// `Num2Word_FF.CURRENCY_FORMS`, built once in `new()`. The registry caches
    /// the instance behind a `OnceLock`, so this runs at most once per process;
    /// rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the **XOF** entry, i.e. the
    /// first value in the class-body dict literal's insertion order.
    /// `to_currency` silently falls back to it for any unknown code. Held
    /// separately because a `HashMap` has no insertion order to recover it from.
    currency_forms_fallback: CurrencyForms,
}

impl Default for LangFf {
    fn default() -> Self {
        Self::new()
    }
}

impl LangFf {
    pub fn new() -> Self {
        // Class-body insertion order is XOF, USD, EUR. Only the first entry's
        // identity matters beyond lookup (it is `.get()`'s default), and that is
        // captured in `currency_forms_fallback` rather than left to the map.
        //
        // Both forms of every entry are identical in the Python source; the
        // duplication is transcribed rather than collapsed, because
        // `to_currency` indexes `cr1[1]`/`cr2[1]` directly and the arity is
        // what keeps that index in range.
        const SANTIIM: [&str; 2] = ["santiim", "santiim"];
        let xof = CurrencyForms::new(&["seefaa", "seefaa"], &SANTIIM);

        let mut currency_forms = HashMap::new();
        currency_forms.insert("XOF", xof.clone());
        currency_forms.insert("USD", CurrencyForms::new(&["dolaar", "dolaar"], &SANTIIM));
        currency_forms.insert("EUR", CurrencyForms::new(&["yero", "yero"], &SANTIIM));

        LangFf {
            currency_forms,
            currency_forms_fallback: xof,
        }
    }
}

impl Lang for LangFf {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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

    fn pointword(&self) -> &str {
        "feccere"
    }

    /// `setup(): self.exclude_title = ["e", "feccere", "less"]`.
    ///
    /// Reproduced for fidelity only. `is_title` stays `False` (the trait
    /// default, matching `__init__`), and `Num2Word_FF.to_cardinal` never
    /// calls `title()` anyway, so this list is never consulted.
    fn exclude_title(&self) -> &[String] {
        static EXCL: OnceLock<Vec<String>> = OnceLock::new();
        EXCL.get_or_init(|| {
            vec!["e".to_string(), "feccere".to_string(), "less".to_string()]
        })
    }

    /// Port of `Num2Word_FF.to_cardinal`, integer path only.
    ///
    /// Python stringifies the input and branches on the leading `"-"` and on
    /// the presence of `"."`. `str(int)` never contains a `"."`, so integers
    /// always fall through to `self._int_to_word(int(n))`. The float branch
    /// (`pointword` + per-digit `ones`, with `"sufri"` for a zero digit) is
    /// out of scope.
    ///
    /// The negative branch recurses through `to_cardinal` on the *digit
    /// string* `n[1:]`, which re-enters the same integer path with the
    /// absolute value — so it is equivalent to prefixing `int_to_word(|n|)`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            // (self.negword + self.to_cardinal(n[1:])).strip()
            let inner = int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        int_to_word(value)
    }

    /// Port of `Num2Word_FF.to_ordinal`: `self.to_cardinal(number) + "ɓal"`.
    ///
    /// No separator, no stem change, and no guard against negatives or zero —
    /// `to_ordinal(0)` == `"sufriɓal"` and `to_ordinal(-1)` == `"less go'oɓal"`
    /// (both in the corpus). Above the 10^9 cliff it degenerates to digits
    /// plus the suffix, making it identical to `to_ordinal_num`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let card = self.to_cardinal(value)?;
        Ok(format!("{}{}", card, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_FF.to_ordinal_num`: `str(number) + "ɓal"`.
    ///
    /// Overrides the base default (`return value`) only by appending the
    /// suffix. The sign is kept verbatim: `to_ordinal_num(-1)` == `"-1ɓal"`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_FF.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The `longval=True` parameter is accepted and ignored by Python, and
    /// the base class's AD/BC handling is bypassed entirely — negative years
    /// simply come back with the `"less "` prefix (`year(-44)` ==
    /// `"less capanɗe nayi e nayi"`), not a "BC" marker. This matches the
    /// trait default, but is spelled out so the override is not mistaken for
    /// an omission.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal branch of `Num2Word_FF.to_cardinal`.
    ///
    /// FF does **not** override `to_cardinal_float`; it overrides `to_cardinal`
    /// and handles non-integers inline via `str(number).split(".", 1)`. So the
    /// inherited `default_to_cardinal_float` (which drives `base.float2tuple`)
    /// is the wrong engine here — it rounds the fraction through binary
    /// arithmetic and drops a low-order digit at large magnitude. This override
    /// instead rebuilds `str(number)` and runs FF's own string algorithm.
    ///
    /// `precision_override` is ignored: `Num2Word_FF.to_cardinal(self, number)`
    /// takes no `precision=` parameter, so the kwarg never reaches this code in
    /// Python (it would `TypeError` at the call site). The reconstructed string
    /// uses the value's repr-derived precision, exactly as `str()` does.
    ///
    /// Faithfully reproduced quirks:
    ///   * `1.0` (float) -> `"go'o feccere sufri"`: `str(1.0)` is `"1.0"`, so the
    ///     `"."` branch fires and a trailing `"0"` digit -> `"sufri"`.
    ///   * `Decimal("1.10")` -> `"go'o feccere go'o sufri"`: the trailing zero is
    ///     a real fractional digit (unlike the float `1.1`).
    ///   * `Decimal("98746251323029.99")` raises `OverflowError`: the integer
    ///     part is past the 10^12 maxval (#147; Python returned its digits).
    ///   * A negative with a zero integer part keeps its sign because the sign
    ///     lives in the *string* (`"-0.5"`), not in a truncated int:
    ///     `-0.5` -> `"less sufri feccere jowi"`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        to_cardinal_from_str(&float_to_str(value))
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// FF routes on the string, not the value: `"." in str(number)` decides
    /// between the pointword grammar and `int(n)`. So the base default's
    /// whole→int shortcut is wrong here — `str(5.0)` is `"5.0"` and must read
    /// `"jowi feccere sufri"`, while the point-free `Decimal("5")` stays
    /// `"jowi"`, and an exponent-form repr (`"1e+16"`, `"1E+2"`) raises
    /// `ValueError` from `int()`. `to_cardinal_from_str` reproduces the whole
    /// method (sign recursion, `int(left)`, per-digit `int()`), so this entry
    /// only has to rebuild Python's `str(number)`.
    ///
    /// `precision_override` is ignored: `Num2Word_FF.to_cardinal(self, number)`
    /// takes no `precision=` parameter, so the kwarg never reaches this code in
    /// Python.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        to_cardinal_from_str(&py_str(value))
    }

    /// `to_ordinal(float/Decimal)`: `to_cardinal(number) + "ɓal"`, no special
    /// cases — `to_ordinal(5.0)` == `"jowi feccere sufriɓal"`, and an
    /// exponent-form repr propagates the cardinal's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "ɓal"` — the raw repr,
    /// never an error: `to_ordinal_num(1e16)` == `"1e+16ɓal"`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`: bare `self.to_cardinal(val)`, string routing
    /// included — `to_year(5.0)` == `"jowi feccere sufri"`, `to_year(1e16)`
    /// raises ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`) and this
    /// override does not change what parses — `python_decimal_parse` still
    /// decides. It exists because `Decimal("Infinity")`/`Decimal("NaN")` *do*
    /// parse in Python, and FF's `to_cardinal` then dies at `int("Infinity")`
    /// with `ValueError: invalid literal for int() with base 10: 'Infinity'`
    /// (str(Decimal) capitalises whatever the input case, and the sign of
    /// "-Infinity" is stripped before `int()` sees it). The bridge hard-wires
    /// the *base* integer-path errors for Inf/NaN parses (OverflowError /
    /// "cannot convert NaN"), which FF never produces, so the raise is
    /// surfaced here instead. The one input this would misserve is
    /// `to_ordinal_num("Infinity")` (Python: `"Infinityɓal"`), which no corpus
    /// exercises.
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

    // ---- currency ---------------------------------------------------------
    //
    // `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are Base's empty dicts at
    // runtime, so `currency_adjective` (None) and `currency_precision` (100 for
    // every code) are left at their trait defaults, which already say exactly
    // that. `_money_verbose`/`_cents_verbose`/`_cents_terse` are inherited
    // unchanged and their trait defaults already mirror Base. `to_cheque` is
    // inherited too, so `currency::default_to_cheque` serves it via the two
    // hooks below. `cardinal_from_decimal` stays at its raising default: FF's
    // `to_currency` never reaches base's fractional-cents branch (it has no
    // fractional-cents concept at all — bug 5 truncates at two digits).

    /// `self.__class__.__name__`, for the NotImplementedError `to_cheque`
    /// raises. `to_currency` never raises it — see `currency_forms`.
    fn lang_name(&self) -> &str {
        "Num2Word_FF"
    }

    /// `self.CURRENCY_FORMS[currency]` — a **strict** subscript, matching the
    /// `try: cr1, _cr2 = self.CURRENCY_FORMS[currency] except KeyError:` in
    /// `Num2Word_Base.to_cheque`, which is this hook's only caller here.
    /// `to_currency` deliberately does *not* route through it: it uses `.get()`
    /// with an XOF default and never raises.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Python:
    /// ```python
    /// def pluralize(self, n, forms):
    ///     if not forms:
    ///         return ""
    ///     return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Overriding `Num2Word_Base.pluralize`'s bare `raise NotImplementedError`,
    /// so the trait's raising default would be wrong here. Unreachable in
    /// practice — FF overrides `to_currency` wholesale and picks its form by
    /// hand, and `to_cheque` takes `cr1[-1]` without consulting this — but the
    /// method exists on the class and is transcribed for that reason.
    ///
    /// Note `forms[-1]` (**last**), not `forms[1]`: with a 3-form entry this
    /// would differ from `Num2Word_EUR.pluralize`. It also never raises
    /// IndexError, because the empty case returns `""` first.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms {
            [] => Ok(String::new()),
            _ if n.is_one() => Ok(forms[0].clone()),
            _ => Ok(forms[forms.len() - 1].clone()),
        }
    }

    /// Python's `Num2Word_FF.to_currency`:
    ///
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
    /// This shares nothing with `Num2Word_Base.to_currency`. It is a
    /// string-level reimplementation, and the `str(val)` it splits is what makes
    /// the Int/Decimal distinction reach here at all: `str(1)` == `"1"` has one
    /// part (`right` = 0), `str(1.0)` == `"1.0"` has two — which then *also*
    /// yields `right` = 0 via `int("0".ljust(2, "0"))`, so both print
    /// `"go'o yero"` (bug 6). They agree, but by different routes, and only when
    /// the cents are zero.
    ///
    /// The words come from `_int_to_word`, **not** `to_cardinal`, so they
    /// share its scales and 10^12 ceiling (#147; Python's digit fallback gave
    /// `"1000000000 yero"`) without inheriting the negword handling.
    /// `is_negative` is captured from the *original* value
    /// and re-applied at the end, which is why taking `abs` up front is safe.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // declared and never read by Python (bug 8)
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`; resolve
        // it through this language's own default (" ") before the ported body.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is evaluated *before* `val = abs(val)`.
        let is_negative = val.is_negative();

        // `str(abs(val))`. The Decimal arm was parsed from Python's own
        // `str(value)` on the shim side, so rendering it back reproduces the
        // literal for every plain decimal string. It reproduces the `e+` case
        // too — `BigDecimal` keeps a negative scale and re-renders "1e+21",
        // which `parse_int` then rejects exactly as Python's `int()` does. The
        // `e-` case is *not* recoverable; see the port report.
        let s = match val {
            CurrencyValue::Int(i) => i.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `parts = str(val).split(".")`, then parts[0] / parts[1]. Python splits
        // on every ".", so parts[1] would stop at a second one; a decimal
        // rendering never carries two, making this 2-way split equivalent.
        let mut parts = s.split('.');
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            parse_int(p0)?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // `[:2]` truncates to two digits — it does not round (bug 5).
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut padded: String = frac.chars().take(2).collect();
                while padded.chars().count() < 2 {
                    padded.push('0'); // ljust(2, "0")
                }
                parse_int(&padded)?
            }
            _ => BigInt::zero(),
        };

        // `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        // — an unknown code borrows XOF's forms instead of raising.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_forms_fallback);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // `result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])`
        // — a direct index, not `pluralize()`. Every FF entry carries exactly
        // two forms, so `[1]` is in range.
        let one = BigInt::one();
        let mut result = format!(
            "{} {}",
            int_to_word(&left)?,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so this is `right != 0`
        // (bugs 6 and 7).
        if cents && !right.is_zero() {
            // Python concatenates the separator raw, with no space of its own
            // (bug 9).
            result.push_str(separator);
            result.push_str(&int_to_word(&right)?);
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — "less " *with* its trailing space,
        // concatenated raw, which is what supplies the gap (bug 10).
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        Ok(result.trim().to_string())
    }
}
