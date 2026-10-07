//! Port of `lang_JW.py` (Javanese).
//!
//! Registry note: the key `"jv"` resolves to `Num2Word_JW` — `__init__.py` has
//! both `"jw": lang_JW.Num2Word_JW()` (line 342) and
//! `"jv": lang_JW.Num2Word_JW()` (line 406, "Alias for Javanese (modern ISO
//! 639-1)"). Same class, two keys; this file is the `jv` spelling.
//!
//! Shape: **self-contained**. `Num2Word_JW` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the `__init__`
//! guard in `base.py`
//!
//! ```text
//! if any(hasattr(self, field) for field in
//!        ["high_numwords", "mid_numwords", "low_numwords"]):
//!     self.cards = OrderedDict(); self.set_numwords()
//!     self.MAXVAL = 1000 * list(self.cards.keys())[0]
//! ```
//!
//! never fires: `self.cards` and `self.MAXVAL` are never created. `to_cardinal`
//! is overridden outright and drives `_int_to_word`. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check at all** — see bug 3 below for what happens instead.
//!
//! Overridden by JW (so the trait defaults are *not* used):
//!   * `to_cardinal`    — own algorithm, no `splitnum`/`clean`
//!   * `to_ordinal`     — `to_cardinal(n) + "-e"`
//!   * `to_ordinal_num` — `str(n) + "."`
//!   * `to_year`        — `to_cardinal(val)`; the `longval` kwarg is accepted
//!     and then ignored, and there is no BC/AD handling, so
//!     `to_year(-500)` == `to_cardinal(-500)` == "minus lima atus".
//!
//! No cross-call mutable state: JW sets no `_pending_*` flags and does not
//! override `str_to_number` in Python, so the stateless Rust path is safe to
//! dispatch to. (The Rust `str_to_number` hook below *is* overridden, but only
//! to bounce `"Infinity"`/`"NaN"` strings back to the Python original — see
//! the override site — not to change any parse result.)
//!
//! Because `to_ordinal` / `to_ordinal_num` / `to_year` are plain wrappers with
//! no type guard, they accept floats and Decimals too: `to_ordinal(5.0)` ==
//! "lima point zero-e" (the float grammar plus the suffix) and
//! `to_ordinal_num(5.0)` == "5.0." (`str(number)` plus a dot — yes, `"-0.0."`
//! and `"1e+16."` are real outputs). The `ordinal_float_entry` /
//! `ordinal_num_float_entry` hooks below reproduce exactly that; `to_year`'s
//! float routing already falls out of `cardinal_float_entry`.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, verified against `bench/corpus.jsonl`:
//!
//! 1. **Zero is English.** `_int_to_word(0)` is
//!    `self.ones[0] if self.ones[0] else "zero"`. `ones[0]` is `""`, which is
//!    falsy, so the fallback always wins and `to_cardinal(0)` == `"zero"` —
//!    not the Javanese "nol". The `self.ones[0]` arm is unreachable.
//! 2. **No teens, and inconsistent hundreds.** The tens table is applied
//!    positionally with no special forms, so 11 is "sepuluh siji" (lit. "ten
//!    one") rather than the real Javanese "sewelas", and 12 is "sepuluh loro"
//!    rather than "rolas". Likewise hundreds use the plain `ones` word while
//!    tens use combining forms from the `tens` table: 800 is "wolu atus" but
//!    80 is "wolung puluh"; 100 is "siji atus", not "satus". 20 is "rong
//!    puluh" but 200 is "loro atus". Preserved verbatim.
//! 3. **Numbers >= 10^9 come back as digits.** `_int_to_word`'s final `else`
//!    is `return str(number)` — a literal "Fallback for very large numbers".
//!    So `to_cardinal(10**9)` == `"1000000000"` and
//!    `to_ordinal(10**9)` == `"1000000000-e"`. No exception is raised, because
//!    `MAXVAL` was never set (see above). The corpus goes to 10**21, which
//!    exceeds `u64::MAX` — hence `BigInt` throughout rather than a fixed-width
//!    cast.
//! 4. **`_int_to_word`'s negative branch is dead** on every path in scope.
//!    `to_cardinal` strips the "-" from `str(number)` *before* calling
//!    `int()`, so `_int_to_word` only ever receives a non-negative value. The
//!    branch is reproduced below for structural fidelity but is unreachable
//!    from the four modes we implement.
//! 5. **`to_ordinal_num` keeps the sign and ignores the fallback**, since it
//!    never consults `_int_to_word`: `to_ordinal_num(-1)` == `"-1."`.
//!
//! # The currency surface
//!
//! `Num2Word_JW` overrides `to_currency` **wholesale** and shares nothing with
//! `Num2Word_Base`'s currency machinery: it never touches
//! `parse_currency_parts`, `prefix_currency`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse` or `pluralize`. It does its own string
//! surgery on `str(val)` and calls `_int_to_word` directly for both halves. It
//! also never reads `CURRENCY_PRECISION`, so there is no divisor and no
//! `ROUND_HALF_UP` quantize — the cent count is always the first two fractional
//! digits (quirks 7 and 8 below).
//!
//! `to_cheque` is the mirror image: JW does **not** define it, so
//! `Num2Word_Base.to_cheque` runs unchanged and reaches `CURRENCY_FORMS`,
//! `CURRENCY_PRECISION` and `_money_verbose` through the trait defaults
//! (`currency_forms`, `100`, and `to_cardinal` respectively). The two entry
//! points therefore **disagree about an unknown currency code** —
//! `to_currency` silently falls back to IDR (quirk 9) while `to_cheque` indexes
//! `CURRENCY_FORMS[currency]` and raises `NotImplementedError`. Both are pinned
//! by the frozen corpus and both are reproduced here.
//!
//! `CURRENCY_FORMS` is a plain class attribute on `Num2Word_JW` with three
//! entries. Nothing merges into it, and — unlike the `lang_EUR` family
//! described in `PORTING_CURRENCY.md` — `Num2Word_EN.__init__` cannot reach it:
//! JW subclasses `Num2Word_Base` directly and *shadows* the base's empty
//! `CURRENCY_FORMS` with its own dict, so EN's in-place mutation of
//! `Num2Word_EUR.CURRENCY_FORMS` lands on a different object. Verified against
//! the live interpreter after import, not read off the source:
//!
//! ```text
//! {'IDR': (('rupiah', 'rupiah'), ('sen', 'sen')),
//!  'USD': (('dollar', 'dollars'), ('cent', 'cents')),
//!  'EUR': (('euro', 'euros'), ('cent', 'cents'))}
//! ```
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both `{}` on the base and
//! are never overridden, so `currency_adjective` (`None`) and
//! `currency_precision` (`100`) stay at their trait defaults. `pluralize` is
//! left at the raising default because `Num2Word_Base.pluralize` is abstract
//! and JW never defines it — it is unreachable on both currency paths anyway
//! (`to_currency` selects forms inline, `to_cheque` takes `cr1[-1]`).
//!
//! # Faithfully reproduced Python bugs (currency)
//!
//! 6. **A float with zero cents drops the cents segment.** The guard is
//!    `if cents and right:` and `right` is an `int`, so `0` is falsy. This is
//!    the *opposite* of `Num2Word_Base.to_currency`, which shows `1.0` as
//!    "one euro, zero cents" because it branches on `isinstance(val, int)`
//!    rather than on the cent count. JW stringifies first and so cannot tell
//!    `1` from `1.0` at all: both render `"siji euro"`. Corpus-pinned on both.
//! 7. **Cents are truncated to two digits, never rounded, and right-padded.**
//!    `int(parts[1][:2].ljust(2, "0"))` — so `0.5` is **50** cents (pad), and
//!    `1.239` would be 23 cents (truncate, not 24). The `[:2]` also means a
//!    3-decimal currency's mils are silently cut to cents.
//! 8. **`CURRENCY_PRECISION` is never consulted by `to_currency`.** So `JPY`
//!    still shows a cents segment (divisor 1 is ignored) and `KWD`/`BHD` still
//!    use two fractional digits rather than three. Corpus-pinned.
//! 9. **An unknown currency code does not raise — it becomes IDR.**
//!    `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`. Python
//!    dicts have preserved insertion order since 3.7 and the literal is written
//!    IDR, USD, EUR, so `values()[0]` is IDR's. Hence `currency="GBP"`,
//!    `"JPY"`, `"KWD"`, `"BHD"`, `"INR"`, `"CNY"` and `"CHF"` all render
//!    "rupiah"/"sen" — all corpus-pinned, 12 rows each.
//! 10. **Singular vs plural is a no-op for IDR.** Its form pairs are identical
//!    (`("rupiah", "rupiah")`), so the `left != 1` / `right != 1` selects
//!    Python performs cannot affect the output for the fallback currency. USD
//!    and EUR do inflect. Preserved verbatim either way.
//!
//! # Errors
//!
//! The four integer modes raise nothing. There is no `MAXVAL` overflow check
//! (bug 3), the table lookups are all provably in range (see [`idx`]), and
//! negatives are handled rather than rejected — `to_ordinal(-1)` returns
//! "minus siji-e" rather than the `errmsg_negord` TypeError that
//! `Num2Word_Base.to_ordinal` would have produced, because JW overrides it. The
//! corpus confirms: every `cardinal`/`ordinal`/`ordinal_num`/`year` row for
//! "jv" is `"ok": true`.
//!
//! The currency surface adds exactly two reachable raises:
//!   * `to_cheque` with a code outside {IDR, USD, EUR} → `NotImplemented`,
//!     message `Currency code "GBP" not implemented for "Num2Word_JW"`. Seven
//!     corpus rows.
//!   * `to_currency` → `Value` (Python `ValueError`) when `int()` is handed a
//!     non-numeric token. Unreachable for any value `str()` renders in plain
//!     decimal notation; see the exponent-notation note on
//!     [`LangJv::to_currency`].
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use dolar / euro with sen. Examples in these docs
//! that quote English nouns record Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`, set in `setup()`. Note the trailing space: `to_cardinal`
/// concatenates it directly and relies on `.strip()` only for the ends.
const NEGWORD: &str = "minus ";

/// The `"zero"` from `_int_to_word`'s falsy-`ones[0]` fallback (bug 1).
const ZERO_WORD: &str = "zero";

/// `self.ones`. Index 0 is `""` and is never emitted — `_int_to_word` returns
/// early for 0, and every other lookup uses a nonzero digit.
const ONES: [&str; 10] = [
    "", "siji", "loro", "telu", "papat", "lima", "enem", "pitu", "wolu", "sanga",
];

/// `self.tens`. Index 0 is `""`; unreachable, as `_int_to_word` only consults
/// this table for `10 <= number < 100`, where `number // 10` is 1..=9.
///
/// 5 and 6 are the irregular "seket"/"sewidak" (no " puluh" suffix); the rest
/// are `<combining form> puluh`.
const TENS: [&str; 10] = [
    "",
    "sepuluh",
    "rong puluh",
    "telung puluh",
    "patang puluh",
    "seket",
    "sewidak",
    "pitung puluh",
    "wolung puluh",
    "sanga puluh",
];

/// `self.hundred` / `self.thousand` / `self.million`.
const HUNDRED: &str = "atus";
const THOUSAND: &str = "ewu";
const MILLION: &str = "yuta";

/// `self.pointword`, set in `setup()`. Used by the float/Decimal branch of
/// `to_cardinal` (see [`cardinal_from_repr`]); the same value the
/// [`pointword`](LangJv::pointword) hook returns.
const POINTWORD: &str = "point";

/// `self.__class__.__name__`. The file is `lang_jv.rs` but the Python class is
/// `Num2Word_JW` — the `jv`/`jw` keys share one class (see the header). This is
/// the string `to_cheque` interpolates into its `NotImplementedError`, so the
/// registry key must **not** be substituted here.
const LANG_NAME: &str = "Num2Word_JW";

/// `Num2Word_JW.to_currency`'s own `separator=" "` default.
///
/// JW narrows `Num2Word_Base`'s `","`, and every currency corpus row is
/// generated by `num2words(v, lang="jv", to="currency", currency=C)` with
/// **no** separator kwarg — so `" "` is what each expected string carries
/// ("sepuluh loro euros telung puluh papat cents", not
/// "sepuluh loro euros,telung puluh papat cents").
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s `separator=","` default, used here as the
/// "caller did not pass one" sentinel — see [`LangJv::to_currency`].
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// The key whose value is `list(CURRENCY_FORMS.values())[0]` — the fallback an
/// unknown code lands on (bug 9). Python's dict literal is written IDR, USD,
/// EUR, and dicts have preserved insertion order since 3.7, so the first value
/// is IDR's.
const FALLBACK_CURRENCY: &str = "IDR";

pub struct LangJv {
    /// `Num2Word_JW.CURRENCY_FORMS`, built once in [`LangJv::new`] and never
    /// per call — rebuilding it per call is what made an earlier revision of
    /// this port 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(CURRENCY_FORMS.values())[0]`, resolved once. A `HashMap` is enough
    /// for lookup but cannot answer "the first value", so the IDR entry is
    /// cloned out here rather than making the map an ordered one — it is two
    /// short strings, and holding it by value keeps the struct free of a
    /// self-referential borrow. See [`FALLBACK_CURRENCY`].
    fallback_forms: CurrencyForms,
}

impl LangJv {
    pub fn new() -> Self {
        // `Num2Word_JW.CURRENCY_FORMS`, verbatim and in the literal's order.
        // Built once here, never per call. IDR's pairs are (form, form) — the
        // singular/plural select is a no-op for it (bug 10) — while USD and EUR
        // genuinely inflect. The arity is 2 throughout, because `to_currency`
        // indexes `cr1[1]`/`cr2[1]` directly and `to_cheque` takes `cr1[-1]`.
        let mut currency_forms: HashMap<&'static str, CurrencyForms> = HashMap::new();
        currency_forms.insert(
            "IDR",
            CurrencyForms::new(&["rupiah", "rupiah"], &["sen", "sen"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dolar", "dolar"], &["sen", "sen"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["euro", "euro"], &["sen", "sen"]),
        );

        // `list(CURRENCY_FORMS.values())[0]` resolved once — see bug 9.
        let fallback_forms = currency_forms
            .get(FALLBACK_CURRENCY)
            .expect("FALLBACK_CURRENCY is inserted above")
            .clone();

        LangJv {
            currency_forms,
            fallback_forms,
        }
    }
}

impl Default for LangJv {
    fn default() -> Self {
        Self::new()
    }
}

/// Narrow a table index to `usize`.
///
/// Every caller has already bounded the value by an enclosing `number < 10`,
/// `< 100`, `< 1000` comparison, so the digit is 0..=9 and the conversion
/// cannot fail. Falling back to 0 rather than panicking keeps a hypothetical
/// future miscount from aborting the process; index 0 is the empty string in
/// both tables.
fn idx(n: &BigInt) -> usize {
    n.to_usize().unwrap_or(0)
}

/// Python's `_int_to_word`.
///
/// Mirrors the original's cascade exactly, including the dead negative branch
/// (bug 4) and the `str(number)` fallback for >= 10^9 (bug 3).
fn int_to_word(number: &BigInt) -> String {
    // `self.ones[0] if self.ones[0] else "zero"` — ones[0] == "" is falsy.
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Dead on every in-scope path: to_cardinal strips the sign first.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    let ten = BigInt::from(10u32);
    let hundred = BigInt::from(100u32);
    let thousand = BigInt::from(1_000u32);
    let million = BigInt::from(1_000_000u32);
    let billion = BigInt::from(1_000_000_000u32);

    if number < &ten {
        return ONES[idx(number)].to_string();
    }

    if number < &hundred {
        // number > 0 here, so div_rem == floor division; no Python `%` skew.
        let (tens_val, ones_val) = number.div_rem(&ten);
        if ones_val.is_zero() {
            return TENS[idx(&tens_val)].to_string();
        }
        return format!("{} {}", TENS[idx(&tens_val)], ONES[idx(&ones_val)]);
    }

    if number < &thousand {
        let (hundreds_val, remainder) = number.div_rem(&hundred);
        // `self.ones[hundreds_val] + " " + self.hundred` — the plain ones word,
        // not a combining form (bug 2): 800 -> "wolu atus".
        let mut result = format!("{} {}", ONES[idx(&hundreds_val)], HUNDRED);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if number < &million {
        let (thousands_val, remainder) = number.div_rem(&thousand);
        let mut result = format!("{} {}", int_to_word(&thousands_val), THOUSAND);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    if number < &billion {
        let (millions_val, remainder) = number.div_rem(&million);
        let mut result = format!("{} {}", int_to_word(&millions_val), MILLION);
        if !remainder.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&remainder));
        }
        return result;
    }

    // `return str(number)  # Fallback for very large numbers` (bug 3).
    //
    // Only reachable at top level: the recursive calls above pass a quotient
    // or remainder that the enclosing bound already forced below 10^9.
    number.to_string()
}

// ---- float / Decimal path -----------------------------------------------
//
// JW does **not** override `to_cardinal_float`; in Python the float and
// Decimal cases are handled *inline* by `to_cardinal`, which stringifies the
// input and walks the fractional characters one at a time:
//
// ```python
// def to_cardinal(self, number):
//     n = str(number).strip()
//     if n.startswith("-"):
//         n = n[1:]
//         ret = self.negword
//     else:
//         ret = ""
//     if "." in n:
//         left, right = n.split(".", 1)
//         ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//         for digit in right:
//             ret += self._int_to_word(int(digit)) + " "
//         return ret.strip()
//     else:
//         return (ret + self._int_to_word(int(n))).strip()
// ```
//
// So there is **no** `float2tuple`, no banker's-`round()`, and no `< 0.01`
// heuristic on this path — the fraction is literally the digits of
// `str(number)`. The load-bearing artefacts move one level down, into
// reproducing CPython's `str(float)` / `str(Decimal)` exactly. The Rust
// dispatcher routes every float/Decimal to `to_cardinal_float`, so JW must
// override *that* hook to run the same string algorithm (the integer
// `to_cardinal` above never sees a dot).

/// Python's `int(s)` on a whole token, with CPython's error text.
///
/// Reached with whatever `str(number)` / a `split(".", 1)` half produced.
/// `str(float)`/`str(Decimal)` only ever emit plain digits here, but exponent
/// forms (`"1E+2"`) would land a non-numeric token and raise `ValueError`,
/// exactly as Python's `int()` does — same variant, matching message.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s))
    })
}

/// Python's `int(ch)` for a single fractional character.
///
/// `char::to_digit(10)` is ASCII-only; no `str(float)`/`str(Decimal)` can emit
/// a non-ASCII digit, so it agrees with CPython on every string that reaches
/// here. A stray `'E'`/`'+'` from a scientific repr raises `ValueError` on the
/// offending character, quoting it as CPython does.
fn py_int_digit(ch: char) -> Result<usize> {
    ch.to_digit(10).map(|d| d as usize).ok_or_else(|| {
        N2WError::Value(format!("invalid literal for int() with base 10: '{}'", ch))
    })
}

/// The shortest round-trip decimal digits of `a` (finite, non-negative), plus
/// CPython's `decpt`: the value is `0.<digits> * 10^decpt`.
///
/// Rust's shortest `{:e}` and CPython's `repr` agree on the digit count always
/// and the digits themselves almost always, parting only on an exact tie:
/// CPython's `dtoa` rounds the tie to **even**, Rust's shortest `flt2dec`
/// rounds it **away from zero**. Rust's *fixed-precision* `{:.*e}` is correctly
/// rounded half-to-even, so re-emitting the same significant-digit count
/// through it applies CPython's tie rule — kept only if it still round-trips,
/// so the rare asymmetric interval near a power of two falls back to shortest.
///
/// (Copied from the CEB port, which validated this against CPython over
/// 777,014 doubles with zero mismatches; `{:e}` alone missed 21 of the first
/// 75,034. The float path here is the same string-of-digits algorithm.)
fn shortest_repr_digits(a: f64) -> (String, i32) {
    let split = |s: &str| -> (String, i32) {
        let (mant, exp) = s.split_once('e').expect("{:e} always emits an 'e'");
        (
            mant.chars().filter(|c| *c != '.').collect(),
            exp.parse::<i32>().expect("{:e} exponent is an integer") + 1,
        )
    };

    let shortest = format!("{:e}", a);
    let (digits, decpt) = split(&shortest);

    let ties_even = format!("{:.*e}", digits.len() - 1, a);
    if ties_even.parse::<f64>() == Ok(a) {
        return split(&ties_even);
    }
    (digits, decpt)
}

/// CPython's `str(float)` / `repr(float)` — `PyOS_double_to_string(v, 'r', 0,
/// Py_DTSF_ADD_DOT_0)`.
///
/// * exponent form when `decpt <= -4 || decpt > 16` (so `str(1e16) == "1e+16"`
///   but `str(1e15) == "1000000000000000.0"`), exponent `%+.02d`;
/// * otherwise positional, appending `.0` if nothing follows the point — which
///   is why `1.0` renders "siji point zero" rather than "siji".
///
/// The sign is read from the sign *bit*, so `str(-0.0) == "-0.0"` and JW's
/// `startswith("-")` fires: `-0.0` renders "minus zero point zero".
fn py_str_f64(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { "-inf" } else { "inf" }.to_string();
    }

    let sign = if v.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_repr_digits(v.abs());
    let ndig = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let mantissa = if ndig > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits.clone()
        };
        let exp = decpt - 1;
        let (esign, eabs) = if exp < 0 {
            ("-", -(exp as i64))
        } else {
            ("+", exp as i64)
        };
        return format!("{}{}e{}{:0>2}", sign, mantissa, esign, eabs);
    }
    if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= ndig {
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - ndig) as usize))
    } else {
        format!("{}{}.{}", sign, &digits[..decpt as usize], &digits[decpt as usize..])
    }
}

/// CPython's `Decimal.__str__` (the spec's `to-scientific-string`), ported from
/// `_pydecimal.Decimal.__str__`.
///
/// `BigDecimal::from_str` keeps the written scale rather than normalising, so
/// `"1.10"` stays coefficient 110 / scale 2 — which is what makes the trailing
/// "zero" appear ("siji point siji zero"). `(coefficient, -scale)` is exactly
/// Python's `(_int, _exp)`. (Copied from the CEB port, validated against
/// CPython over 40,029 Decimals; the only divergence is negative zero, which
/// `BigInt` cannot represent — see the port report.)
fn py_str_decimal(value: &BigDecimal) -> String {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    let exp = -scale;
    let sign = if coefficient.is_negative() { "-" } else { "" };
    let int_digits = coefficient.abs().to_string();
    let ndig = int_digits.len() as i64;

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
        (
            int_digits[..dotplace as usize].to_string(),
            format!(".{}", &int_digits[dotplace as usize..]),
        )
    };

    let exponent = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exponent)
}

/// Port of `Num2Word_JW.to_cardinal` operating on the repr — the form Python's
/// own code walks. See the section comment above for the source.
///
/// The negative handling is JW's own (non-recursive): a single leading `"-"` is
/// stripped and `self.negword` prepended, then the remainder is processed. Each
/// fractional character goes through the *full* `_int_to_word`, so a `0` digit
/// becomes "zero" (bug 1) exactly as an integer 0 would. `int(left)` is the
/// whole integer part, so bug 3 applies there too: at 1e9 and above it comes
/// back as bare digits ("98746251323029 point sanga sanga").
fn cardinal_from_repr(n: &str) -> Result<String> {
    // n = str(number).strip()
    let n = n.trim();

    // if n.startswith("-"): n = n[1:]; ret = self.negword  else: ret = ""
    let (ret, n) = match n.strip_prefix('-') {
        Some(rest) => (NEGWORD.to_string(), rest),
        None => (String::new(), n),
    };

    match n.split_once('.') {
        // if "." in n:
        Some((left, right)) => {
            // ret += self._int_to_word(int(left)) + " " + self.pointword + " "
            let mut ret = ret;
            ret.push_str(&int_to_word(&py_int(left)?));
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // for digit in right: ret += self._int_to_word(int(digit)) + " "
            for digit in right.chars() {
                ret.push_str(&int_to_word(&BigInt::from(py_int_digit(digit)?)));
                ret.push(' ');
            }
            // return ret.strip()
            Ok(ret.trim().to_string())
        }
        // else: return (ret + self._int_to_word(int(n))).strip()
        None => Ok(format!("{}{}", ret, int_to_word(&py_int(n)?)).trim().to_string()),
    }
}

impl Lang for LangJv {

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

    /// `to_ordinal(float/Decimal)`. `Num2Word_JW.to_ordinal` is
    /// `self.to_cardinal(number) + "-e"` with **no type guard**, so a float or
    /// Decimal rides the same decimal grammar as `to_cardinal` and then takes
    /// the suffix: `to_ordinal(5.0)` == "lima point zero-e", `to_ordinal(-0.0)`
    /// == "minus zero point zero-e", and a whole `Decimal("100")` ==
    /// "siji atus-e" (the ordinal analogue of keeping the ".0" tail). An
    /// exponent-notation repr (`str(1e16)` == "1e+16", `str(Decimal("1E+2"))`
    /// == "1E+2") makes the inner `int()` raise its ValueError *before*
    /// Python's `+ "-e"` runs — the `?` reproduces that ordering.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-e", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` never casts to
    /// int, so it *succeeds* on every float and Decimal — "5.0.", "-0.0.",
    /// "1e+16." and "1E+2." are all real Python outputs (bug 5's
    /// sign-and-repr passthrough extends to floats). `repr_str` is the
    /// binding's Python `str(value)`, exactly the string Python concatenates.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — JW inherits `Num2Word_Base`'s
    /// `Decimal(value)`, so `"Infinity"`/`"NaN"` strings parse *successfully*
    /// and only blow up later, inside `to_cardinal`'s `int("Infinity")` /
    /// `int("NaN")`. The non-finite sentinels pass straight through here and
    /// the mode-aware [`inf_result`](Self::inf_result) / [`nan_result`](Self::nan_result)
    /// hooks reproduce the exact per-mode Python behaviour natively — no
    /// fallback. Finite parses are returned unchanged.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal('Infinity')` / `-Infinity`. `to_cardinal` strips a leading
    /// "-" and then `int("Infinity")` raises `ValueError: invalid literal for
    /// int() with base 10: 'Infinity'` (the sign is gone, so both signs quote
    /// 'Infinity'); `to_ordinal` adds "-e" *after* that int(), so it raises the
    /// same way; `to_year` == `to_cardinal`. `to_ordinal_num` is
    /// `str(number) + "."` and never calls `int()`, so it succeeds with
    /// "Infinity." / "-Infinity.".
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        if to == "ordinal_num" {
            return Ok(format!("{}Infinity.", if negative { "-" } else { "" }));
        }
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'Infinity'".into(),
        ))
    }

    /// `Decimal('NaN')`. `str(Decimal('NaN'))` is "NaN"; `int("NaN")` raises
    /// `ValueError: invalid literal for int() with base 10: 'NaN'` on the
    /// cardinal/ordinal/year paths, while `to_ordinal_num` echoes "NaN.".
    fn nan_result(&self, to: &str) -> Result<String> {
        if to == "ordinal_num" {
            return Ok("NaN.".into());
        }
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'NaN'".into(),
        ))
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "IDR"
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
        "point"
    }

    /// Python's `to_cardinal`, integer path.
    ///
    /// The original goes through the *string*: `n = str(number).strip()`, then
    /// strips a leading `"-"` into `ret`, then `int(n)`s the rest. For integer
    /// input `"." in n` is never true, so the float branch is unreachable and
    /// the whole thing reduces to sign-split + `_int_to_word` + `.strip()`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // Python's `.strip()`. Only ever trims NEGWORD's trailing space in the
        // (impossible) event of an empty word — `_int_to_word` never returns
        // "" — but reproduced so the shape matches.
        Ok(format!("{}{}", ret, int_to_word(&n)).trim().to_string())
    }

    /// `return cardinal + "-e"` — one suffix for every number, no agreement,
    /// no negative rejection. `to_ordinal(-1)` == "minus siji-e".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-e", self.to_cardinal(value)?))
    }

    /// `return str(number) + "."` — never touches `_int_to_word`, so the sign
    /// survives (bug 5) and no fallback applies.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is ignored; there is no BC/AD suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Float / Decimal cardinal path.
    ///
    /// JW does not override `Num2Word_Base.to_cardinal_float` in Python — the
    /// non-integer cases live inline in its `to_cardinal`, reached because the
    /// stringified input contains a `"."`. The Rust dispatcher routes every
    /// float/Decimal here, so this override reproduces that inline algorithm
    /// (see [`cardinal_from_repr`] and the section comment above `py_int`).
    ///
    /// `precision_override` (the `precision=` kwarg) is accepted and ignored:
    /// JW's `to_cardinal` never reads `self.precision`, and the fraction is the
    /// literal digits of `str(number)`. `FloatValue::precision` is likewise
    /// unused — `str(number)` is recomputed from the raw f64 / BigDecimal, which
    /// is more faithful than any digit count. Verified live: `precision=1`,
    /// `2`, `5` all give "loro point enem pitu lima" for 2.675.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        // n = str(number). The float and Decimal arms are not interchangeable
        // (issue #603): `str` of a Decimal keeps every written digit, so
        // Decimal("1.10") ends in "zero" where the float 1.10 could not.
        let n = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        cardinal_from_repr(&n)
    }

    // ---- currency ----------------------------------------------------
    //
    // Only three hooks are overridden. `currency_adjective` ({} -> None),
    // `currency_precision` ({}.get(code, 100) -> 100), `pluralize` (abstract,
    // raises), `money_verbose`/`cents_verbose`/`cents_terse` (-> to_cardinal)
    // and `to_cheque` (Num2Word_Base's, unmodified) all match the trait
    // defaults exactly — see the header. Restating them would be noise.

    /// `self.__class__.__name__`, as the inherited `to_cheque` interpolates it
    /// into its `NotImplementedError`.
    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `Num2Word_JW.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately a **strict** lookup that returns `None` for an unknown
    /// code. This hook feeds the inherited `to_cheque`, which in Python does
    /// `CURRENCY_FORMS[currency]` inside a `try/except KeyError` and re-raises
    /// as `NotImplementedError`. `to_currency` must *not* use this hook — it
    /// does a `.get(..., default)` and falls back to IDR instead (bug 9), so it
    /// consults `self.currency_forms`/`self.fallback_forms` directly.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_JW.to_currency` — a wholesale replacement for
    /// `Num2Word_Base.to_currency`, sharing none of its machinery.
    ///
    /// Python:
    /// ```text
    /// is_negative = False
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency,
    ///                                    list(self.CURRENCY_FORMS.values())[0])
    /// left_str = self._int_to_word(left)
    /// result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    /// if cents and right:
    ///     cents_str = self._int_to_word(right)
    ///     result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    /// if is_negative:
    ///     result = self.negword + result
    /// return result.strip()
    /// ```
    ///
    /// # Why the int/float split collapses here
    ///
    /// `base.to_currency` branches on `isinstance(val, int)` to decide whether
    /// to render cents at all. JW never does: it stringifies first, then tests
    /// the *cent count* (`if cents and right`). `1` gives `str` `"1"` (no
    /// `parts[1]`, `right = 0`) and `1.0` gives `str` `"1.0"`
    /// (`parts[1] == "0"`, so `right = int("00") = 0`) — both falsy, both
    /// `"siji euro"`. So `CurrencyValue::Int` vs `Decimal` is still honoured
    /// exactly (it changes the string being split), it simply cannot change the
    /// output for this language. Corpus-pinned on `1` and `1.0`.
    ///
    /// # `str(val)` fidelity
    ///
    /// `CurrencyValue::Decimal` carries `str(value)` re-parsed into a
    /// `BigDecimal`, and this reproduces Python's `str(val)` via `Display`.
    /// That roundtrip is exact for plain decimal notation — `BigDecimal`
    /// preserves scale, so `"1.0"`, `"0.5"`, `"0.01"`, `"12.34"`, `"99.99"` and
    /// `"1234.56"` all render back byte-identically, which is every value the
    /// corpus exercises.
    ///
    /// It is *not* exact for the floats whose `repr` uses exponent notation,
    /// because the original repr string is not carried across the boundary and
    /// `BigDecimal`'s `Display` picks its own threshold. Three outcomes, none
    /// corpus-covered (see `concerns`):
    ///   * `1e+16`, `1e+21`, `1e-07`, `5e-324` — `Display` keeps the exponent,
    ///     `BigInt::from_str` rejects the token, and this raises
    ///     `N2WError::Value`. Python's `int("1e+21")` raises `ValueError`: same
    ///     variant, different message text.
    ///   * `1e-05`, `1e-06` — `Display` renders them plain (`"0.00001"`), so
    ///     this returns "zero rupiah" where Python raises `ValueError`.
    ///   * `1.2345678901234568e+16` — `Display` renders `"12345678901234568"`,
    ///     so this returns "12345678901234568 rupiah" (via the `str(number)`
    ///     fallback, bug 3) where Python splits the repr on its `"."` and
    ///     returns "siji rupiah rong puluh telu sen".
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Python accepts `adjective` and never reads it: JW has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency.
        let _ = adjective;

        // The trait passes whatever the caller gave, and the dispatcher's
        // "caller said nothing" value is `Num2Word_Base`'s `","` — not JW's own
        // `" "`. Treat it as the sentinel it is. (The trait has no
        // per-language default-separator hook; see `concerns`.)
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` — the test precedes
        // the abs, and the abs runs only on the negative branch. Taking abs
        // unconditionally here is equivalent: the branch Python skips is
        // `val >= 0`, whose `str()` carries no sign anyway. The one input where
        // the two strings differ is `-0.0` (`val < 0` is False, so Python keeps
        // `"-0.0"` while this yields `"0.0"`), and it converges regardless —
        // `int("-0") == int("0") == 0`, giving "zero rupiah" on both sides.
        let is_negative = val.is_negative();
        let s = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };

        // `str(val).split(".")`: take parts[0] and parts[1]. Splitting on every
        // dot rather than just the first matters only for input Python could
        // never produce, but it is what `.split(".")` does.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`. The empty arm is dead — `str()`
        // of a non-negative int or float never starts with "." — but ported.
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads on the right, so "5" -> "50"
        // (0.5 is 50 cents) and "01" -> "01" (0.01 is 1 cent). Sliced by
        // chars, never bytes. `len(parts) > 1` is `part1.is_some()`; the
        // `and parts[1]` guard is the non-empty test.
        let right = match part1 {
            Some(f) if !f.is_empty() => {
                let mut two: String = f.chars().take(2).collect();
                while two.chars().count() < 2 {
                    two.push('0');
                }
                BigInt::from_str(&two).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code becomes IDR rather than an error (bug 9). Deliberately not the
        // `currency_forms()` hook, which is the strict lookup `to_cheque` needs.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (bug 6).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // `result = self.negword + result` — raw, keeping the trailing space
        // baked into "minus ". Base would strip and re-add it; JW does not.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input — `_int_to_word`
        // never returns padding and no currency form is blank — but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;

    /// Python's `abs(Decimal(str(value)).as_tuple().exponent)` for a plain
    /// decimal string — count of chars after the dot.
    fn prec(arg: &str) -> u32 {
        match arg.split_once('.') {
            Some((_, frac)) => frac.len() as u32,
            None => 0,
        }
    }

    fn card_float(arg: &str) -> String {
        let v = FloatValue::Float {
            value: f64::from_str(arg).unwrap(),
            precision: prec(arg),
        };
        LangJv::new().to_cardinal_float(&v, None).unwrap()
    }

    fn card_dec(arg: &str) -> String {
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str(arg).unwrap(),
            precision: prec(arg),
        };
        LangJv::new().to_cardinal_float(&v, None).unwrap()
    }

    #[test]
    fn corpus_cardinal_float() {
        // Every `"to": "cardinal"` row for jv with a dot in `arg`.
        let cases = [
            ("0.0", "zero point zero"),
            ("0.5", "zero point lima"),
            ("1.0", "siji point zero"),
            ("1.5", "siji point lima"),
            ("2.25", "loro point loro lima"),
            ("3.14", "telu point siji papat"),
            ("0.01", "zero point zero siji"),
            ("0.1", "zero point siji"),
            ("0.99", "zero point sanga sanga"),
            ("1.01", "siji point zero siji"),
            ("12.34", "sepuluh loro point telu papat"),
            ("99.99", "sanga puluh sanga point sanga sanga"),
            ("100.5", "siji atus point lima"),
            ("1234.56", "siji ewu loro atus telung puluh papat point lima enem"),
            ("-0.5", "minus zero point lima"),
            ("-1.5", "minus siji point lima"),
            ("-12.34", "minus sepuluh loro point telu papat"),
            ("1.005", "siji point zero zero lima"),
            ("2.675", "loro point enem pitu lima"),
        ];
        for (arg, want) in cases {
            assert_eq!(card_float(arg), want, "float {}", arg);
        }
    }

    #[test]
    fn corpus_cardinal_dec() {
        // Every `"to": "cardinal_dec"` row for jv.
        let cases = [
            ("0.01", "zero point zero siji"),
            ("1.10", "siji point siji zero"),
            ("12.345", "sepuluh loro point telu papat lima"),
            ("98746251323029.99", "98746251323029 point sanga sanga"),
            ("0.001", "zero point zero zero siji"),
        ];
        for (arg, want) in cases {
            assert_eq!(card_dec(arg), want, "dec {}", arg);
        }
    }

    #[test]
    fn negative_zero_float_carries_sign() {
        // str(-0.0) == "-0.0" -> the sign bit fires JW's startswith("-").
        assert_eq!(card_float("-0.0"), "minus zero point zero");
    }

    #[test]
    fn extra_live_interpreter_rows() {
        assert_eq!(card_float("1000000000.5"), "1000000000 point lima");
        assert_eq!(
            card_float("12345.678"),
            "sepuluh loro ewu telu atus patang puluh lima point enem pitu wolu"
        );
        // Decimal that stringifies without a dot -> integer branch.
        assert_eq!(card_dec("100"), "siji atus");
    }

    #[test]
    fn exponent_notation_raises_valueerror_like_python() {
        // str(1e16)=="1e+16" etc. have no dot; int("1e+16") is a ValueError with
        // the character-exact CPython message. Same for a scientific Decimal.
        let l = LangJv::new();
        for (f, tok) in [(1e16, "1e+16"), (1e-5, "1e-05"), (1e21, "1e+21")] {
            let v = FloatValue::Float { value: f, precision: 0 };
            match l.to_cardinal_float(&v, None) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", tok)
                ),
                other => panic!("{}: expected ValueError, got {:?}", tok, other.map(|_| ())),
            }
        }
        let v = FloatValue::Decimal {
            value: BigDecimal::from_str("1E+2").unwrap(),
            precision: 0,
        };
        match l.to_cardinal_float(&v, None) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1E+2'")
            }
            other => panic!("expected ValueError, got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn precision_override_is_inert() {
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        let base = LangJv::new().to_cardinal_float(&v, None).unwrap();
        for p in [0u32, 1, 2, 5, 10] {
            assert_eq!(
                LangJv::new().to_cardinal_float(&v, Some(p)).unwrap(),
                base,
                "precision override {}",
                p
            );
        }
    }
}
