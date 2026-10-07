//! Port of `lang_CNH.py` (Hakha Chin).
//!
//! # Best-candidate words (gladiaio/num2words2#154)
//!
//! UNVERIFIED (#154): "pakpalawng" (zero) — best candidate: Van Bik Hakha-
//!   English dictionary, "nought: pakpalawng" (chin-dictionary.com).
//! UNVERIFIED (#154): "deh" (decimal) — best candidate: "deh" = dot in the
//!   same dictionary; no source reads a decimal with it.
//! UNVERIFIED (#154): "zuh" (minus) — best candidate: the subtraction word
//!   ("Paruk ah pahnih zuh"); no source reads a negative number.
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. No
//! scale word above a million is attested for this language, so the port
//! raises `OverflowError` from 10^9, which `maxval()` reports. Where the notes
//! below describe the digit fallback ("1000000000", "no overflow check"), they
//! describe Python; that arm is now unreachable.
//!
//! Registry check: `CONVERTER_CLASSES["cnh"]` is `lang_CNH.Num2Word_CNH`, which
//! is the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_CNH` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `__init__` never builds `self.cards` and never sets `self.MAXVAL`. All four
//! in-scope modes (`to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year`)
//! are overridden outright, so **nothing** is inherited from `Num2Word_Base`
//! and `cards`/`maxval`/`merge` stay at their trait defaults. In particular
//! `splitnum`/`clean`/`merge` are never reached, and there is no overflow
//! check of any kind — see bug 1 below for what happens instead.
//!
//! Everything runs through the private `_int_to_word` helper, which handles
//! 0..10^9-1 by 1/2/3-digit chunking and gives up above that.
//!
//! # Faithfully reproduced Python behaviour
//!
//! This is a port, not a rewrite. The following all look wrong but are exactly
//! what Python emits, each verified against a `bench/corpus.jsonl` row:
//!
//! 1. **`_int_to_word` silently degrades to digits at 10^9.** The chain of
//!    `if number < ...` tests ends at `1000000000`; past it the function just
//!    `return str(number)`. So `to_cardinal(10**9)` == `"1000000000"` — bare
//!    digits, no words, and *no* `OverflowError`. This is CNH's de facto
//!    MAXVAL, and it leaks into the other modes: `to_ordinal(10**9)` ==
//!    `"1000000000-nak"`, and `to_year` likewise. Corpus confirms this all the
//!    way up to 10^21 (`"1000000000000000000000"`), which is why the value
//!    stays a `BigInt` here and is never cast to a fixed-width int.
//! 2. **Hundreds join with `" le "`, thousands and millions join with a bare
//!    space.** `101` == `"pakhat phazar le pakhat"` but `1001` ==
//!    `"pakhat thawngkhat pakhat"` and `1000001` == `"pakhat milin pakhat"`.
//!    The asymmetry is in the source (`" le " + ...` vs `" " + ...`) and is
//!    preserved verbatim.
//! 3. **`to_ordinal` accepts negatives and zero.** `Num2Word_Base.verify_ordinal`
//!    is never called, so `to_ordinal(-1)` == `"zuh pakhat-nak"` and
//!    `to_ordinal(0)` == `"zero-nak"` rather than raising `TypeError`. The
//!    `-nak` suffix attaches to the *whole* phrase, so `to_ordinal(-999)` ==
//!    `"zuh pakua phazar le pakua kip le pakua-nak"`.
//! 4. **`to_ordinal_num` does not go through `to_cardinal`,** so it keeps the
//!    minus sign as a digit prefix: `to_ordinal_num(-42)` == `"-42-nak"`.
//! 5. **`tens[1]` breaks the pattern.** 20..90 are `"<unit> kip"`, but 10 is
//!    `"pahra"`, not `"pakhat kip"`. Kept as-is.
//! 6. **English words survived in a Hakha Chin table (replaced by the
//!    best candidates above, #154):** `_int_to_word(0)` is
//!    `"zero"` (not a Chin numeral), and `setup` sets `pointword = "decimal"`
//!    and `negword = "minus "`. Not our problem to fix.
//! 7. **`exclude_title` / `is_title` are dead config.** `setup` populates
//!    `exclude_title = ["minus", "decimal", "le"]`, but CNH's `to_cardinal`
//!    never calls `self.title()` (unlike `Num2Word_Base.to_cardinal`), so the
//!    list can never take effect. Mirrored on the trait anyway for fidelity.
//! 8. **`to_year` ignores its `longval` parameter** entirely — the body is just
//!    `return self.to_cardinal(val)`, so years get no era/pairing treatment
//!    and negatives come out as `"zuh panga phazar"` for -500.
//!
//! # The currency surface
//!
//! `Num2Word_CNH` defines its own `CURRENCY_FORMS` (three codes) and overrides
//! `to_currency` and `pluralize`. It inherits `to_cheque`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse`, `CURRENCY_ADJECTIVES` (`{}`) and
//! `CURRENCY_PRECISION` (`{}`) from `Num2Word_Base` untouched. There is no
//! `lang_EUR`/`lang_EU` in the MRO — `Num2Word_CNH -> Num2Word_Base -> object`
//! — so the `Num2Word_EN.__init__` mutation of the shared `Num2Word_EUR`
//! class dict (the trap `PORTING_CURRENCY.md` warns about) does not reach this
//! table. Verified against the live interpreter: CNH's `CURRENCY_FORMS` is
//! exactly its own three-entry literal, and both `CURRENCY_PRECISION` and
//! `CURRENCY_ADJECTIVES` are empty dicts.
//!
//! More reproduced Python behaviour, all corpus-confirmed:
//!
//! 9. **`to_currency` never raises for an unknown code.** It looks the code up
//!    with `.get(currency, list(self.CURRENCY_FORMS.values())[0])`, so anything
//!    outside `{MMK, USD, EUR}` silently becomes **MMK** — the first value in
//!    the dict's insertion order. `to_currency(1, "GBP")` == `"pakhat kyat"`,
//!    not a `NotImplementedError`. `to_cheque` is the opposite: it inherits
//!    `Num2Word_Base.to_cheque`, which subscripts `self.CURRENCY_FORMS[currency]`
//!    and converts the `KeyError` into `NotImplementedError`. So the same
//!    "GBP" is kyats through one entry point and an exception through the
//!    other. The fallback therefore lives *only* inside `to_currency`;
//!    `currency_forms()` (which is what `default_to_cheque` calls) does a plain
//!    lookup and returns `None`.
//! 10. **`to_currency` ignores `adjective` entirely.** The parameter is in the
//!    signature and never read. Moot in practice — `CURRENCY_ADJECTIVES` is
//!    empty — but ported as written.
//! 11. **`to_currency` ignores `CURRENCY_PRECISION`.** It hard-codes two
//!    fractional digits via `parts[1][:2].ljust(2, "0")` rather than consulting
//!    a divisor, so neither the 3-decimal (KWD/BHD) nor the 0-decimal (JPY)
//!    convention exists here. `to_currency(12.34, "JPY")` renders a cents
//!    segment — `"pahra le pahnih kyat pathum kip le pali pya"` — where a
//!    precision-aware language would round to whole yen. Doubly moot since
//!    CNH's `CURRENCY_PRECISION` is `{}` (every code is 100), but it means
//!    `Num2Word_Base.to_currency`'s zero-decimal pre-rounding branch is dead
//!    code for CNH, and so `crate::currency::default_to_currency` is never
//!    reached.
//! 12. **`to_currency` does not use `pluralize`.** It open-codes the choice as
//!    `cr1[1] if left != 1 else cr1[0]`, indexing the form tuple directly. So
//!    `pluralize` — which CNH does override — is dead code: nothing in the
//!    class or in `Num2Word_Base` calls it once `to_currency` is overridden.
//!    Ported anyway, since it is a public method a caller could invoke.
//! 13. **`to_currency` collapses the int/float distinction that
//!    `Num2Word_Base.to_currency` is built around.** The base branches on
//!    `isinstance(val, int)` to decide whether cents appear; CNH never looks at
//!    the type, it only asks whether `str(val)` contains a `"."`. The two agree
//!    by accident for `1` (no dot -> no cents) and, via a *different* rule, for
//!    `1.0`: `parts[1]` is `"0"`, so `right` is `0`, which is falsy, so the
//!    `if cents and right:` segment is skipped. Both render `"pakhat euro"`.
//!    That is CNH's own doing, not a collapse introduced by this port — the
//!    `Int`/`Decimal` arms below are kept apart and reach the same place by the
//!    routes Python takes.
//! 14. **Zero cents are dropped, and `cents=False` drops the segment
//!    entirely.** `if cents and right:` is a truthiness test on both. So
//!    `to_currency(12.00, "EUR")` == `"pahra le pahnih euro"` (no `"zero
//!    cent"`), and `cents=False` yields no cents at all rather than falling
//!    back to `_cents_terse`'s digit form.
//! 15. **Cents truncate, they never round.** `parts[1][:2]` slices the decimal
//!    text. `to_currency(0.999, "EUR")` takes `"99"`, not 100 — no carry into
//!    the units.
//!
//! # Errors
//!
//! The four integer modes are total: `ones`/`tens` are only ever indexed 0..=9
//! (guarded by `< 10` and by `divmod` on a value `< 100`), the `>= 10^9`
//! fallback returns a string instead of raising, and no dict lookup exists to
//! `KeyError` on. `Result` is returned only to satisfy the trait.
//!
//! The float/Decimal *entries* add exactly one error: `N2WError::Value` from
//! `int(n)` when `str(number)` carries no `"."` and is not plain digits — the
//! exponential forms of bug 22. `to_ordinal_num` never raises (it never calls
//! `int()`), which is why `to_ordinal_num(1e16)` is `"1e+16-nak"` while
//! `to_ordinal(1e16)` is a ValueError.
//!
//! The currency surface has exactly two error paths:
//!
//! * `to_cheque` with a code outside `{MMK, USD, EUR}` -> `NotImplemented`,
//!   from `Num2Word_Base.to_cheque`'s `KeyError` -> `NotImplementedError`
//!   conversion.
//! * `to_currency` on a value whose `str()` is in scientific notation ->
//!   `Value`, because `int("1e+16")` is a `ValueError`. See
//!   [`LangCnh::split_currency`].
//!
//! # The float / Decimal cardinal path
//!
//! `Num2Word_CNH` does **not** override `to_cardinal_float`; it routes
//! non-integers back through its own `to_cardinal`, whose `"." in n` branch
//! (module route below, bug 16) is what actually renders them. So the trait's
//! `to_cardinal_float` is overridden here to reproduce that branch rather than
//! inherit `floatpath::default_to_cardinal_float`. Two facts make it a
//! *different* algorithm from the base float path:
//!
//! 16. **CNH renders the raw `str(number)` digits — no `float2tuple`, no
//!     rounding.** The base path computes `post = abs(value-pre) * 10**precision`
//!     in binary f64 and leans on a `< 0.01` heuristic to rescue artefacts like
//!     `2.675 -> 674.9999… -> 675`. CNH never does any of that: it takes the
//!     characters of `str(number)` after the `"."` verbatim, so `2.675` renders
//!     `paruk parih panga` (6 7 5) straight off the repr. The whole trap the
//!     base path exists to survive simply does not apply.
//! 17. **`self.precision` is never read, so the `precision=` kwarg has no
//!     effect.** `Num2Word_CNH.to_cardinal(number)` takes no `precision`
//!     argument and never consults `self.precision`; the digit count is
//!     `len(str(number).split(".")[1])`, fixed by the repr. Verified live:
//!     `num2words(2.675, lang="cnh", precision=1)` still yields all three
//!     fractional words. `precision_override` is therefore ignored.
//! 18. **Each fractional digit is a *single* `ones` word (or `"zero"`).** The
//!     loop is `ret += " " + (self.ones[int(digit)] or "zero")`, so `0` becomes
//!     `"zero"` (because `ones[0]` is the falsy `""`) and `1..9` become the bare
//!     unit word — never the tens/hundreds machinery. `0.01` is
//!     `"pakpalawng deh pakpalawng pakhat"`, digit by digit.
//! 19. **The integer part still goes through `_int_to_word(int(left))`,** so it
//!     inherits every integer quirk, including bug 1: the Decimal
//!     `98746251323029.99` renders its left part as bare digits
//!     `"98746251323029"` (past 10^9) followed by `"deh pakua pakua"`.
//! 20. **Sign is detected from the *text*, `str(number).startswith("-")`,** not
//!     a numeric `< 0`. That matters only for negative zero: `str(-0.0)` is
//!     `"-0.0"`, so `to_cardinal(-0.0)` is `"zuh pakpalawng deh pakpalawng"`. The
//!     Float arm below uses `f64::is_sign_negative()` (the sign *bit*) to match,
//!     deliberately **not** `FloatValue::is_negative()` (a `< 0.0` test, which
//!     reads `-0.0` as positive). No corpus row exercises `-0.0`; this keeps the
//!     untested edge faithful anyway.
//!
//! 21. **Whole floats keep their ".0" tail.** Routing is `"." in str(number)`,
//!     and `str(5.0)` is `"5.0"`, so `to_cardinal(5.0)` == `"panga decimal
//!     zero"` — never Base's whole-value integer route. `Decimal("5")` (str
//!     `"5"`, no dot) *does* take the integer path. `cardinal_float_entry`
//!     carries this routing; `ordinal`/`year` inherit it by composition
//!     (`to_ordinal(5.0)` == `"panga deh pakpalawng-nak"`, `to_year(5.0)` ==
//!     `"panga deh pakpalawng"`), and `to_ordinal_num(5.0)` suffixes the raw
//!     repr: `"5.0-nak"`.
//! 22. **Exponential string forms raise ValueError.** `str(1e16)` ==
//!     `"1e+16"` and `str(Decimal("1E+2"))` == `"1E+2"` contain no `"."`, so
//!     they fall through to `int(n)`, which raises `ValueError: invalid
//!     literal for int() with base 10: '1e+16'`. The same route kills string
//!     input `"1e3"` (`Decimal("1e3")` -> str `"1E+3"` -> `int` ValueError)
//!     and `"Infinity"`/`"NaN"` (`int("Infinity")` / `int("NaN")` ->
//!     ValueError — see [`Lang::str_to_number`] below for how the binding's
//!     hard-wired OverflowError is pre-empted for Inf). Corpus-pinned for
//!     1e+16/1e+20 (float), 1E+2/1E+20 (Decimal) and "1e3"/"1E3"/"Infinity"/
//!     "-Infinity"/"NaN" (str) across cardinal/ordinal/year.
//!
//! `cardinal_from_decimal` (the fractional-cents entry point) still stays at its
//! trait default: CNH's `to_currency` hard-slices two decimal digits out of the
//! text and never hands a fractional cent count downstream, so it is never
//! reached — see the currency notes above.
//!
//! # Cross-call state
//!
//! None. `Num2Word_CNH` stashes no flag in one method for another to consume
//! (no `_pending_ordinal`-style handshake), so the stateless Rust path is a
//! faithful substitute and the dispatcher needs no special-casing. The
//! `CURRENCY_FORMS` table is immutable after construction.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `self.ones`. Index 0 is `""` in Python; `_int_to_word` guards it with the
/// `number == 0` early return and with `if o` in the tens branch, so the empty
/// string is never emitted on the integer path.
const ONES: [&str; 10] = [
    "", "pakhat", "pahnih", "pathum", "pali", "panga", "paruk", "parih", "pariat", "pakua",
];

/// `self.tens`. Index 0 is `""` and unreachable (the tens branch only runs for
/// `10 <= number < 100`, so `t` is 1..=9). Note index 1 is "pahra", not
/// "pakhat kip" — see bug 5 in the module docs.
const TENS: [&str; 10] = [
    "",
    "pahra",
    "pahnih kip",
    "pathum kip",
    "pali kip",
    "panga kip",
    "paruk kip",
    "parih kip",
    "pariat kip",
    "pakua kip",
];

const HUNDRED: &str = "phazar";
const THOUSAND: &str = "thawngkhat";
/// Doubtful but kept (#258): "milin" is unattested, and "milian" is a "rich
/// person" in chin-dictionary.com. The Hakha Bible (HCL06) counts with
/// za 100, thong 10^3, sang 10^4, sing 10^5 and nuai 10^6 ("nuai khat le
/// sing khat" = 1,100,000), which this module's scale does not follow; one
/// source was not enough to switch.
const MILLION: &str = "milin";

/// `self.negword`, trailing space included exactly as Python spells it.
const NEGWORD: &str = "zuh ";

/// `self.pointword`.
const POINTWORD: &str = "deh";

/// What `_int_to_word` returns for 0 — an English word, see bug 6.
const ZERO_WORD: &str = "pakpalawng";

/// The `-nak` ordinal suffix, appended to the whole rendered phrase.
const ORDINAL_SUFFIX: &str = "-nak";

/// `self.__class__.__name__`, for `Num2Word_Base.to_cheque`'s
/// `'Currency code "%s" not implemented for "%s"'` message.
const LANG_NAME: &str = "Num2Word_CNH";

/// `list(self.CURRENCY_FORMS.values())[0]` — `to_currency`'s fallback for an
/// unknown code (module docs, bug 9).
///
/// Python evaluates that expression fresh on every call; it resolves to the
/// **first entry in the dict literal's insertion order**, and `lang_CNH.py`
/// writes `MMK` first. Dicts preserve insertion order in every Python this
/// library supports, and `Num2Word_CNH.CURRENCY_FORMS` is never mutated (no
/// `__init__` writes into it, and nothing else in the MRO shares the dict), so
/// the answer is the constant `"MMK"` for the life of the process. Named here
/// rather than left implicit because it is a *position*, not a code, that
/// Python is selecting: reordering the literal would silently change it.
const CURRENCY_FALLBACK_KEY: &str = "MMK";

/// Narrow a provably-small `BigInt` to a table index.
///
/// Every call site is guarded so the value is 0..=9: the `< 10` branch passes
/// `number` itself, and the tens/hundreds branches pass a quotient or remainder
/// of a value already bounded by 100 / 1000. The `expect` documents that
/// invariant rather than defending against it.
fn idx(n: &BigInt) -> usize {
    n.to_usize()
        .expect("guarded by _int_to_word's range checks: always 0..=9")
}

/// Reconstruct `str(Decimal)`'s **sign-free** plain form with exactly
/// `precision` fractional digits, for the float-cardinal path.
///
/// `value` is already absolute. `precision` is authoritative — it is Python's
/// `abs(Decimal(str(value)).as_tuple().exponent)`, i.e. the number of fractional
/// characters `str(value)` printed, so it drives the digit count (and preserves
/// trailing zeros: `Decimal("1.100")` keeps all three, unlike a float). Working
/// from `precision` rather than the `BigDecimal`'s own scale makes this robust
/// even if the parse normalised the scale away.
///
/// `precision == 0` means `str(value)` had no `"."` (e.g. `Decimal("5")`), so
/// no dot is produced and the caller takes the integer branch.
fn decimal_to_plain_string(value: &BigDecimal, precision: u32) -> String {
    if precision == 0 {
        // str(Decimal) with a non-negative exponent, e.g. "5" — no dot.
        return value.with_scale(0).as_bigint_and_exponent().0.to_string();
    }
    // Pin the scale to `precision`; a no-op when it already matches, a pad when
    // the parse dropped trailing zeros. `value` is absolute, so `unscaled` is
    // non-negative and its `to_string()` carries no minus.
    let scaled = value.with_scale(precision as i64);
    let digits = scaled.as_bigint_and_exponent().0.to_string();

    // Widen so there is always at least one integer digit, mirroring Python's
    // "0.5", never ".5". `width - digits.len()` leading zeros, then slice.
    let precision = precision as usize;
    let width = digits.len().max(precision + 1);
    let padded = format!("{}{}", "0".repeat(width - digits.len()), digits);
    let (int_part, frac_part) = padded.split_at(padded.len() - precision);
    format!("{}.{}", int_part, frac_part)
}

/// Sign-free `str(value)` for a *float* whose repr shows **no** decimal point
/// — i.e. `FloatValue::has_visible_point()` said no. For a finite f64 that
/// means "whole and `|v| >= 1e16`", where CPython's repr switches to exponent
/// form (`"1e+16"`); non-finite values print `"inf"`/`"nan"` (str(float), the
/// sign having been peeled off by the caller; `str(nan)` never has one).
///
/// The mantissa is rebuilt from the float's **exact** integer digits (every
/// whole f64 is exactly representable), not from CPython's shortest-round-trip
/// digits, so for values whose exact expansion is longer than the shortest
/// repr the text can differ from Python's — harmless here, because every
/// output of this function contains a non-digit (`e`/`.`/`inf`/`nan`) and
/// exists only to be fed to `int(n)`, which raises the same `ValueError` for
/// either spelling. For the corpus-pinned cases (1e+16, 1e+20) the two
/// spellings coincide exactly.
fn float_no_point_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return "inf".to_string();
    }
    // Finite without a visible point => whole and |v| >= 1e16, so this BigInt
    // conversion is exact.
    let digits = BigInt::from_f64(v.abs())
        .expect("finite whole f64 converts exactly")
        .to_string();
    let exp = digits.len() - 1; // >= 16, so always two+ exponent digits
    let mant = digits.trim_end_matches('0');
    let mant = if mant.len() <= 1 {
        // Power of ten: bare leading digit, no ".".
        digits[..1].to_string()
    } else {
        format!("{}.{}", &mant[..1], &mant[1..])
    };
    format!("{}e+{}", mant, exp)
}

/// `forms[1] if n != 1 else forms[0]` — the plural choice `to_currency`
/// open-codes for *both* the unit and the subunit (module docs, bug 12).
///
/// Deliberately **not** `pluralize`. CNH's `pluralize` takes `forms[-1]` for
/// `n != 1` and guards the empty tuple; this indexes `[1]` blind. The two agree
/// only because every CNH form tuple happens to have exactly two entries — keep
/// them separate so that stays a coincidence rather than a dependency.
///
/// Python subscripts the tuple, so a one-form entry raises IndexError for
/// `n != 1`. Unreachable with the table [`LangCnh::new`] builds, but mapped to
/// `N2WError::Index` rather than panicking so the exception *type* survives if
/// the table is ever edited.
fn pick_form(n: &BigInt, forms: &[String]) -> Result<String> {
    let i = if n.is_one() { 0 } else { 1 };
    forms
        .get(i)
        .cloned()
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
}

/// The exclusive ceiling (gladiaio/num2words2#147): no scale word above
/// a million is attested, so 10^9 raises `OverflowError` instead of
/// coming back as digits.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(9))
}

/// Holds the `CURRENCY_FORMS` table, built once in [`LangCnh::new`].
///
/// The four integer modes are stateless — `setup()` only assigns constant
/// tables, which live as `const`s above and stay there. The currency table is a
/// `HashMap` rather than a `const` because `to_currency`/`to_cheque` need keyed
/// lookup. It is built **once per process** (`num2words2-py` holds a
/// `OnceLock<LangCnh>` and hands out `&'static LangCnh`), never per call.
pub struct LangCnh {
    /// `Num2Word_CNH.CURRENCY_FORMS`, verbatim.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]`, cloned out at construction so
    /// `to_currency`'s fallback is a field read rather than Python's per-call
    /// `.values()` list materialisation. See [`CURRENCY_FALLBACK_KEY`].
    currency_fallback: CurrencyForms,
}

impl Default for LangCnh {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCnh {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        // `Num2Word_CNH.CURRENCY_FORMS`, verbatim — three codes, two forms per
        // side. Both forms of every pair are identical (Hakha Chin does not
        // inflect these nouns for number), but the arity is load-bearing:
        // `to_currency` indexes `cr1[1]`/`cr2[1]` unguarded, so a one-form
        // entry would `IndexError` in Python and panic here. Keep them paired.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            // MMK is first in the dict literal, which makes it the fallback —
            // see CURRENCY_FALLBACK_KEY.
            ("MMK", CurrencyForms::new(&["kyat", "kyat"], &["pya", "pya"])),
            // "dollar"/"cent"/"euro" are English words sitting in a Hakha Chin
            // table, exactly as bug 6 describes for "zero". Not ours to fix.
            (
                "USD",
                CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"]),
            ),
            (
                "EUR",
                CurrencyForms::new(&["euro", "euro"], &["cent", "cent"]),
            ),
        ]
        .into_iter()
        .collect();

        let currency_fallback = currency_forms
            .get(CURRENCY_FALLBACK_KEY)
            .expect("CURRENCY_FALLBACK_KEY is inserted directly above")
            .clone();

        LangCnh {
            currency_forms,
            currency_fallback,
        }
    }

    /// `parts = str(val).split(".")` and the `left`/`right` extraction from
    /// `Num2Word_CNH.to_currency`, for an already-absolute value:
    ///
    /// ```text
    /// left  = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// # Why this reads digits instead of doing arithmetic
    ///
    /// Python slices *text*, so `right` is the first two fractional digits
    /// **truncated**, never rounded (module docs, bug 15), and `[:2]` on a
    /// shorter string is a `ljust` pad rather than an error. Working from the
    /// `BigDecimal`'s own digits + scale reproduces that on the nose: the value
    /// arrived as `str(value)` from the Python side, so its scale is exactly
    /// the number of fractional characters Python printed.
    ///
    /// # The scientific-notation gap
    ///
    /// `scale < 0` means the value was spelled with a positive exponent
    /// (`str(1e16)` == `"1e+16"`). Python's `split(".")` finds no dot, so
    /// `parts[0]` is the whole `"1e+16"` and `int()` raises `ValueError` —
    /// reproduced here with the same message shape and `N2WError::Value`.
    ///
    /// The mirror case is **not** reproducible at this layer: `str()` also goes
    /// scientific for small floats (`str(1e-05)` == `"1e-05"`, a `ValueError`
    /// in Python), but `BigDecimal::from_str("1e-05")` and
    /// `BigDecimal::from_str("0.00001")` are the *same* value with the *same*
    /// scale 5, so the notation Python used is already lost by the time we are
    /// called. This returns `(0, 0)` -> `"pakpalawng kyat"` where Python raises. It
    /// needs the original string across the binding to fix, and no corpus row
    /// exercises it. Flagged in the report.
    fn split_currency(&self, val: &BigDecimal) -> Result<(BigInt, BigInt)> {
        let (int_val, scale) = val.as_bigint_and_exponent();

        if scale < 0 {
            // int("1e+16") -> ValueError. See the doc comment above.
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: '{}'",
                val
            )));
        }

        // scale == 0: `str(val)` has no ".", so `parts` has one element,
        // `len(parts) > 1` is False and `right` stays 0.
        if scale == 0 {
            return Ok((int_val, BigInt::zero()));
        }

        // `val` is already absolute, so these digits carry no sign.
        let scale = scale as usize;
        let digits = int_val.to_string();
        let padded = if digits.len() <= scale {
            // Python always prints at least one integer digit ("0.5", never
            // ".5"), so widen to scale+1 and keep `parts[0]` non-empty. This is
            // also why the `if parts[0] else 0` guard is unreachable.
            format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits)
        } else {
            digits
        };
        let (int_part, frac_part) = padded.split_at(padded.len() - scale);

        // Both halves are pure ASCII digits, so `chars()` and byte offsets
        // coincide here and neither parse can fail.
        let left = BigInt::from_str(int_part).expect("int_part is ASCII digits");

        // `parts[1][:2].ljust(2, "0")` — truncate to two digits, pad to two.
        let mut frac: String = frac_part.chars().take(2).collect();
        while frac.len() < 2 {
            frac.push('0');
        }
        let right = BigInt::from_str(&frac).expect("frac is two ASCII digits");

        Ok((left, right))
    }

    /// Port of `Num2Word_CNH._int_to_word`.
    ///
    /// Python's `divmod` on non-negative operands matches `div_mod_floor`;
    /// `number` is always non-negative here because `to_cardinal` strips the
    /// sign before calling in (Python does it textually via `n[1:]`, we do it
    /// with `abs`), so the two agree. `div_mod_floor` is used regardless to
    /// keep the correspondence with Python exact rather than incidental.
    ///
    /// Infallible — see the "Errors" section of the module docs.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }
        if *number < BigInt::from(10) {
            return ONES[idx(number)].to_string();
        }
        if *number < BigInt::from(100) {
            // Python: t, o = divmod(number, 10)
            //         return self.tens[t] + (" le " + self.ones[o] if o else "")
            let (t, o) = number.div_mod_floor(&BigInt::from(10));
            let mut out = TENS[idx(&t)].to_string();
            if !o.is_zero() {
                out.push_str(" le ");
                out.push_str(ONES[idx(&o)]);
            }
            return out;
        }
        if *number < BigInt::from(1000) {
            // Python: h, r = divmod(number, 100)
            //         base = self.ones[h] + " " + self.hundred
            //         return base + (" le " + self._int_to_word(r) if r else "")
            let (h, r) = number.div_mod_floor(&BigInt::from(100));
            let mut out = format!("{} {}", ONES[idx(&h)], HUNDRED);
            if !r.is_zero() {
                out.push_str(" le ");
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }
        if *number < BigInt::from(1_000_000) {
            // Python: t, r = divmod(number, 1000)
            //         base = self._int_to_word(t) + " " + self.thousand
            //         return base + (" " + self._int_to_word(r) if r else "")
            // Note the bare " " join here vs " le " above — bug 2.
            let (t, r) = number.div_mod_floor(&BigInt::from(1000));
            let mut out = format!("{} {}", self.int_to_word(&t), THOUSAND);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }
        if *number < BigInt::from(1_000_000_000) {
            // Python: m, r = divmod(number, 1000000)
            //         base = self._int_to_word(m) + " " + self.million
            //         return base + (" " + self._int_to_word(r) if r else "")
            let (m, r) = number.div_mod_floor(&BigInt::from(1_000_000));
            let mut out = format!("{} {}", self.int_to_word(&m), MILLION);
            if !r.is_zero() {
                out.push(' ');
                out.push_str(&self.int_to_word(&r));
            }
            return out;
        }
        // Python: return str(number). No words, no OverflowError — bug 1.
        number.to_string()
    }

    /// The `"." in n` / sign branches of `Num2Word_CNH.to_cardinal`, driven off
    /// an already-reconstructed **sign-free** `str(number)`.
    ///
    /// ```text
    /// # (sign handled by the caller via `is_negative`)
    /// if "." in n:
    ///     left, right = n.split(".", 1)
    ///     ret = self._int_to_word(int(left)) + " " + self.pointword
    ///     for digit in right:
    ///         ret += " " + (self.ones[int(digit)] or "zero")
    ///     return ret.strip()
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// `s` is pure ASCII: digits and at most one `"."` (bug 20 stripped the sign
    /// upstream), so `split_once('.')`, `to_digit` and `from_str` cannot fail on
    /// the values this port produces. A malformed digit is still mapped to
    /// `N2WError::Value` — the type Python's `int()` would raise — rather than
    /// silently mis-rendering.
    fn render_float_text(&self, is_negative: bool, s: &str) -> Result<String> {
        let body = match s.split_once('.') {
            Some((left, right)) => {
                // `int(left)` — `left` is non-empty ASCII digits.
                let left_int = BigInt::from_str(left).map_err(|_| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        left
                    ))
                })?;
                // `_int_to_word(int(left)) + " " + self.pointword`
                let mut ret = format!("{} {}", self.checked_int_to_word(&left_int)?, POINTWORD);
                // Each fractional character maps to one `ones` word, or "zero"
                // for '0' (because `ones[0]` is "" and `"" or "zero"` is "zero")
                // — bug 18. Iterated as chars, per fidelity rule.
                for ch in right.chars() {
                    let d = ch.to_digit(10).ok_or_else(|| {
                        N2WError::Value(format!(
                            "invalid literal for int() with base 10: '{}'",
                            ch
                        ))
                    })?;
                    ret.push(' ');
                    ret.push_str(if d == 0 { ZERO_WORD } else { ONES[d as usize] });
                }
                ret
            }
            // No ".": integer text, `_int_to_word(int(n))`. Only reachable for a
            // `Decimal` whose `str()` had exponent >= 0 (precision 0).
            None => {
                let n = BigInt::from_str(s).map_err(|_| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        s
                    ))
                })?;
                self.checked_int_to_word(&n)?
            }
        };

        // Python's `(self.negword + ...).strip()` for the negative case, and the
        // inner `ret.strip()` otherwise. `NEGWORD` carries its trailing space;
        // `trim()` collapses the no-op edges either way.
        let out = if is_negative {
            format!("{}{}", NEGWORD, body)
        } else {
            body
        };
        Ok(out.trim().to_string())
    }
}

impl Lang for LangCnh {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MMK"
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
        "deh"
    }

    /// `self.exclude_title` from `setup`. Dead config — see bug 7.
    fn exclude_title(&self) -> &[String] {
        static EXCL: OnceLock<Vec<String>> = OnceLock::new();
        EXCL.get_or_init(|| {
            vec![
                "zuh".to_string(),
                "deh".to_string(),
                "le".to_string(),
            ]
        })
        .as_slice()
    }

    /// Port of `Num2Word_CNH.to_cardinal`, integer path only.
    ///
    /// Python works on `n = str(number).strip()` and recurses textually:
    ///
    /// ```text
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// ```
    ///
    /// The recursive call re-enters `to_cardinal` with the sign-free digits,
    /// which then take neither the `"-"` nor the `"."` branch and land in
    /// `_int_to_word(int(n))`. So the whole dance collapses to
    /// `negword + int_to_word(abs(value))`, which is what we do.
    ///
    /// `str(int)` never contains `"."`, so the `pointword` branch is
    /// unreachable for integers and is not modelled.
    ///
    /// The trailing `.strip()` is kept as `.trim()` for fidelity even though it
    /// is provably a no-op: `NEGWORD`'s trailing space is always consumed by the
    /// concatenation, and `int_to_word` never returns a string that is empty or
    /// padded with whitespace (0 yields "zero", and `abs(value) >= 1` here
    /// anyway).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.checked_int_to_word(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(self.checked_int_to_word(value)?)
    }

    /// Port of `Num2Word_CNH.to_ordinal`: `self.to_cardinal(number) + "-nak"`.
    ///
    /// No `verify_ordinal`, so negatives and zero pass straight through, and
    /// the suffix lands on the end of the entire phrase — bug 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_CNH.to_ordinal_num`: `str(number) + "-nak"`.
    ///
    /// Note this bypasses `to_cardinal` entirely, so the minus sign survives as
    /// a digit prefix — bug 4.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    /// Port of `Num2Word_CNH.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The Python signature takes `longval=True` and never reads it — bug 8.
    /// Spelled out rather than left to the trait default (which happens to do
    /// the same thing) to mirror the explicit override in the source.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float / Decimal branch of `Num2Word_CNH.to_cardinal`, reached via the
    /// trait's `to_cardinal_float` hook. See module docs, bugs 16-20.
    ///
    /// This is **not** `floatpath::default_to_cardinal_float`: CNH renders the
    /// raw `str(number)` digits with no `float2tuple` and no rounding heuristic,
    /// and it ignores `self.precision`, so `precision_override` is dropped
    /// (verified live). The work is to reconstruct the sign-free `str(number)`
    /// text Python's `to_cardinal` would split on, then defer to
    /// [`LangCnh::render_float_text`].
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => {
                // Sign from the sign *bit*, matching `str(v).startswith("-")` —
                // true for -0.0 too (bug 20). Not `FloatValue::is_negative()`,
                // which is a `< 0.0` test and would miss it.
                let is_negative = value.is_sign_negative();
                // `str(float)` always has a ".", so `precision >= 1` and the
                // formatted string always carries the point. Formatting the
                // absolute value to exactly `precision` fractional digits
                // reproduces `repr(value)`: `precision` *is* the repr's
                // fractional length, so rounding to it returns the same digits
                // (e.g. 2.675 -> "2.675", never a re-derived "2.674...").
                let text = format!("{:.prec$}", value.abs(), prec = *precision as usize);
                self.render_float_text(is_negative, &text)
            }
            FloatValue::Decimal { value, precision } => {
                // `str(Decimal)` is exact and keeps trailing zeros; the sign is
                // a plain numeric one here (a Decimal has no signed zero the way
                // a float does).
                let is_negative = value.is_negative();
                let text = decimal_to_plain_string(&value.abs(), *precision);
                self.render_float_text(is_negative, &text)
            }
        }
    }

    /// `to_cardinal(float/Decimal)` FULL routing — module docs, bugs 21/22.
    ///
    /// Python's `to_cardinal` is string-driven: `"." in str(number)` picks the
    /// decimal grammar, and `str(5.0)` is `"5.0"`, so **whole floats keep
    /// their ".0" tail** ("panga deh pakpalawng") instead of taking Base's
    /// whole-value integer route. Without a visible point the sign-free string
    /// lands in `int(n)`:
    ///   * `Decimal("5")` -> `"5"` -> the integer path (`"panga"`);
    ///   * repr-exponential floats (`str(1e16)` == `"1e+16"`) and exponential
    ///     Decimals (`str(Decimal("1E+2"))` == `"1E+2"`) -> `int()` raises
    ///     ValueError — [`LangCnh::render_float_text`]'s no-dot branch
    ///     reproduces both the type and the message shape.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if value.has_visible_point() {
            return self.to_cardinal_float(value, precision_override);
        }
        // Sign handling matches the textual `n.startswith("-")` recursion:
        // is_negative() is sign-bit aware for the Float arm (bug 20), and the
        // reconstructed string below is sign-free, exactly like Python's
        // `n[1:]`.
        let is_negative = value.is_negative();
        let text = match value {
            FloatValue::Float { value, .. } => float_no_point_repr(*value),
            FloatValue::Decimal { value, .. } => python_decimal_str(&value.abs()),
        };
        self.render_float_text(is_negative, &text)
    }

    /// `to_ordinal(float/Decimal)`: Python's `to_ordinal` is
    /// `self.to_cardinal(number) + "-nak"` with no type guard, so floats get
    /// the full decimal phrase plus the suffix ("panga deh pakpalawng-nak") and
    /// bug 22's ValueError propagates unchanged for exponential forms.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.cardinal_float_entry(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "-nak"`, no `int()`
    /// anywhere — so it succeeds where the other modes raise ("1e+16-nak"),
    /// and "-0.0" keeps its textual minus ("-0.0-nak"). `repr_str` is the
    /// Python-side `str(number)`, which the binding carries across because
    /// repr(float)/str(Decimal) are Python's to recompute, not ours.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    /// `to_year(float/Decimal)`: `to_year` is `return self.to_cardinal(val)`
    /// (bug 8 — `longval` ignored), so the full float routing above applies
    /// verbatim, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which CNH does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first), finds
    /// no ".", and dies in `int("Infinity")` with ValueError. The binding
    /// otherwise maps `ParsedNumber::Inf` to the base integer path's
    /// OverflowError before any CNH code runs, so the ValueError must be
    /// raised here. (NaN needs no interception: the binding's ValueError
    /// already matches `int("NaN")`'s type.)
    ///
    /// Known gap (unpinned): Python's `to_ordinal_num(Decimal("Infinity"))`
    /// would be "Infinity-nak"; this entry-level interception raises instead.
    /// The strings corpus pins Infinity under `to=cardinal` only.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_CNH` overrides exactly two things here — `to_currency` and
    // `pluralize` — plus the `CURRENCY_FORMS` table. `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` are inherited from
    // `Num2Word_Base` untouched, and `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION`
    // are the base's empty dicts (confirmed against the live interpreter), so
    // `currency_adjective` and `currency_precision` stay at their trait
    // defaults — `None` and a flat 100 for every code.

    fn lang_name(&self) -> &str {
        LANG_NAME
    }

    /// `self.CURRENCY_FORMS[code]` — a **plain** lookup, no MMK fallback.
    ///
    /// This hook is what `currency::default_to_cheque` calls, and Python's
    /// `to_cheque` subscripts the dict and turns the `KeyError` into
    /// `NotImplementedError`. `to_currency`'s `.get(currency, <first value>)`
    /// fallback is a property of *that method*, not of the table, so it lives
    /// in `to_currency` alone — see bug 9. Leaking it into here would make
    /// `to_cheque("GBP")` return kyats instead of raising.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_CNH.pluralize`:
    ///
    /// ```text
    /// if not forms:
    ///     return ""
    /// return forms[0] if n == 1 else forms[-1]
    /// ```
    ///
    /// Dead code in practice (bug 12): `to_currency` open-codes its own plural
    /// choice via [`pick_form`] and never calls this, and the only inherited
    /// caller — `Num2Word_Base.to_currency` — is overridden away. Ported because
    /// it is a public method.
    ///
    /// Note the two deviations from the common `Num2Word_EUR.pluralize` this
    /// resembles: the empty-tuple guard returns `""` instead of raising
    /// IndexError, and the plural branch takes `forms[-1]`, not `forms[1]`. On a
    /// three-form tuple those differ; CNH has none, but the rule is the rule.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        match forms.last() {
            // `if not forms: return ""`
            None => Ok(String::new()),
            Some(last) => Ok(if n.is_one() {
                // `forms[0]` — safe: `forms.last()` matched, so it is non-empty.
                forms[0].clone()
            } else {
                last.clone()
            }),
        }
    }

    /// Port of `Num2Word_CNH.to_currency`:
    ///
    /// ```text
    /// def to_currency(self, val, currency="MMK", cents=True, separator=" ",
    ///                 adjective=False):
    ///     is_negative = val < 0
    ///     val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0])
    ///     result = self._int_to_word(left) + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         result += separator + self._int_to_word(right) + " " + (
    ///             cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// Nothing of `Num2Word_Base.to_currency` survives the override, so
    /// `currency::default_to_currency` is never reached: no `CURRENCY_PRECISION`
    /// divisor, no `pluralize`, no `adjective`, no `has_decimal` guard and no
    /// `NotImplementedError` — bugs 9-15 in the module docs.
    ///
    /// `adjective` is accepted and dropped, exactly as Python declares and never
    /// reads it (bug 10).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `None` means the caller omitted `separator=`, so CNH's own default
        // applies. `default_separator()` carries it (`" "`, read off the live
        // Python signature — *not* `Num2Word_Base`'s `","`).
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` then `val = abs(val)`, so every digit read
        // below comes from the unsigned text — as it does in Python.
        let is_negative = val.is_negative();

        let (left, right) = match val {
            // `str(int)` never holds a ".", so `len(parts) > 1` is False and
            // `right` stays 0. CNH reaches "ints show no cents" through the
            // text, not through `isinstance(val, int)` — bug 13.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            // `has_decimal` is deliberately ignored: CNH never consults the
            // Python type, only whether `str(val)` split on a dot. `1.0` and
            // `Decimal("1.00")` both land on `right == 0` and drop the cents
            // segment by the truthiness test below, which is the same answer
            // the base's `has_decimal` guard would have given by another route.
            CurrencyValue::Decimal { value, .. } => self.split_currency(&value.abs())?,
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — an unknown
        // code silently becomes MMK rather than raising (bug 9).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);

        let mut result = format!(
            "{} {}",
            self.checked_int_to_word(&left)?,
            pick_form(&left, &forms.unit)?
        );

        // `if cents and right:` — a truthiness test on both, so zero cents
        // vanish and `cents=False` drops the segment outright rather than
        // falling back to `_cents_terse` (bug 14).
        if cents && !right.is_zero() {
            // `separator + int_to_word(right) + " " + form`. The separator is
            // concatenated raw, with no space of its own on either side — CNH's
            // default *is* a bare " ", which is the only reason the corpus reads
            // "euro pathum" and not "euro,pathum". The single space below sits
            // between the cents number and the subunit name, not after the
            // separator; adding one there yields "euro  pathum".
            result.push_str(separator);
            result.push_str(&self.checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(&pick_form(&right, &forms.subunit)?);
        }

        if is_negative {
            // `self.negword` in full, trailing space included — Python does not
            // `.strip()` it here the way `to_cardinal` does.
            result.insert_str(0, NEGWORD);
        }

        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use crate::floatpath::FloatValue;

    fn f(value: f64, precision: u32) -> String {
        LangCnh::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn d(s: &str, precision: u32) -> String {
        LangCnh::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    /// Every `"to": "cardinal"` float row for cnh in bench/corpus.jsonl.
    #[test]
    fn corpus_floats() {
        assert_eq!(f(0.0, 1), "pakpalawng deh pakpalawng");
        assert_eq!(f(0.5, 1), "pakpalawng deh panga");
        assert_eq!(f(1.0, 1), "pakhat deh pakpalawng");
        assert_eq!(f(1.5, 1), "pakhat deh panga");
        assert_eq!(f(2.25, 2), "pahnih deh pahnih panga");
        assert_eq!(f(3.14, 2), "pathum deh pakhat pali");
        assert_eq!(f(0.01, 2), "pakpalawng deh pakpalawng pakhat");
        assert_eq!(f(0.1, 1), "pakpalawng deh pakhat");
        assert_eq!(f(0.99, 2), "pakpalawng deh pakua pakua");
        assert_eq!(f(1.01, 2), "pakhat deh pakpalawng pakhat");
        assert_eq!(f(12.34, 2), "pahra le pahnih deh pathum pali");
        assert_eq!(f(99.99, 2), "pakua kip le pakua deh pakua pakua");
        assert_eq!(f(100.5, 1), "pakhat phazar deh panga");
        assert_eq!(
            f(1234.56, 2),
            "pakhat thawngkhat pahnih phazar le pathum kip le pali deh panga paruk"
        );
        assert_eq!(f(-0.5, 1), "zuh pakpalawng deh panga");
        assert_eq!(f(-1.5, 1), "zuh pakhat deh panga");
        assert_eq!(f(-12.34, 2), "zuh pahra le pahnih deh pathum pali");
        // The f64-artefact cases: raw repr digits, NO float2tuple rescue.
        assert_eq!(f(1.005, 3), "pakhat deh pakpalawng pakpalawng panga");
        assert_eq!(f(2.675, 3), "pahnih deh paruk parih panga");
    }

    /// Every `"to": "cardinal_dec"` row for cnh in bench/corpus.jsonl.
    #[test]
    fn corpus_decimals() {
        assert_eq!(d("0.01", 2), "pakpalawng deh pakpalawng pakhat");
        assert_eq!(d("1.10", 2), "pakhat deh pakhat pakpalawng");
        assert_eq!(d("12.345", 3), "pahra le pahnih deh pathum pali panga");
        // Left part past 10^9 raises OverflowError (#147), not digits.
        let big = FloatValue::Decimal {
            value: BigDecimal::from_str("98746251323029.99").unwrap(),
            precision: 2,
        };
        assert!(matches!(
            LangCnh::new().to_cardinal_float(&big, None),
            Err(N2WError::Overflow(_))
        ));
        assert_eq!(d("0.001", 3), "pakpalawng deh pakpalawng pakpalawng pakhat");
    }

    /// The float-entry routing rows of bench/corpus_wholefloat.jsonl and
    /// bench/corpus_strings.jsonl (bugs 21/22).
    #[test]
    fn entry_routing() {
        let l = LangCnh::new();
        let f5 = FloatValue::Float { value: 5.0, precision: 1 };
        // Whole float keeps its ".0" tail through every worded mode.
        assert_eq!(l.cardinal_float_entry(&f5, None).unwrap(), "panga deh pakpalawng");
        assert_eq!(l.ordinal_float_entry(&f5).unwrap(), "panga deh pakpalawng-nak");
        assert_eq!(l.year_float_entry(&f5).unwrap(), "panga deh pakpalawng");
        // ordinal_num echoes the Python repr and suffixes it.
        assert_eq!(l.ordinal_num_float_entry(&f5, "5.0").unwrap(), "5.0-nak");
        // Negative zero: sign bit -> "minus".
        assert_eq!(
            l.ordinal_float_entry(&FloatValue::Float { value: -0.0, precision: 1 })
                .unwrap(),
            "zuh pakpalawng deh pakpalawng-nak"
        );
        // Decimal without a point takes the integer path.
        let d5 = FloatValue::Decimal {
            value: BigDecimal::from_str("5").unwrap(),
            precision: 0,
        };
        assert_eq!(l.cardinal_float_entry(&d5, None).unwrap(), "panga");
        assert_eq!(l.ordinal_float_entry(&d5).unwrap(), "panga-nak");
        // Exponential forms -> int() ValueError, message quoting the literal.
        let e16 = FloatValue::Float { value: 1e16, precision: 16 };
        match l.cardinal_float_entry(&e16, None) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        assert!(matches!(
            l.year_float_entry(&FloatValue::Float { value: 1e20, precision: 20 }),
            Err(N2WError::Value(_))
        ));
        let e2 = FloatValue::Decimal {
            value: BigDecimal::from_str("1E+2").unwrap(),
            precision: 2,
        };
        // #211: str(Decimal) is written out ("100"), so 1E+2 reads as 100.
        assert_eq!(
            l.cardinal_float_entry(&e2, None).unwrap(),
            l.to_cardinal(&BigInt::from(100)).unwrap()
        );
        // ...but ordinal_num never calls int(): the repr sails through.
        assert_eq!(l.ordinal_num_float_entry(&e16, "1e+16").unwrap(), "1e+16-nak");
        // str_to_number: Infinity pre-empts the binding's OverflowError.
        assert!(matches!(l.str_to_number("Infinity"), Err(N2WError::Value(_))));
        assert!(matches!(l.str_to_number("-Infinity"), Err(N2WError::Value(_))));
        // Ordinary strings still parse through the base Decimal grammar.
        assert!(matches!(l.str_to_number("1.5"), Ok(ParsedNumber::Dec(_))));
    }

    /// Live-interpreter-verified edges beyond the corpus.
    #[test]
    fn live_edges() {
        // precision= kwarg has no effect on cnh — override is dropped.
        let l = LangCnh::new();
        for p in [Some(1u32), Some(5), None] {
            assert_eq!(
                l.to_cardinal_float(&FloatValue::Float { value: 2.675, precision: 3 }, p)
                    .unwrap(),
                "pahnih deh paruk parih panga"
            );
        }
        // Negative zero: str(-0.0) == "-0.0" -> the sign bit surfaces "minus".
        assert_eq!(f(-0.0, 1), "zuh pakpalawng deh pakpalawng");
        // Large-ish float, million branch in the integer part.
        assert_eq!(f(1_000_000.25, 2), "pakhat milin deh pahnih panga");
        // Decimals keep every trailing zero.
        assert_eq!(d("1.100", 3), "pakhat deh pakhat pakpalawng pakpalawng");
        assert_eq!(d("5.00", 2), "panga deh pakpalawng pakpalawng");
        assert_eq!(d("10.0", 1), "pahra deh pakpalawng");
        assert_eq!(d("-0.5", 1), "zuh pakpalawng deh panga");
        assert_eq!(d("-12.34", 2), "zuh pahra le pahnih deh pathum pali");
    }
}
