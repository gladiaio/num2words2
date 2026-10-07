//! Port of `lang_GU.py` (Gujarati).
//!
//! Shape: **self-contained**. `Num2Word_GU` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords` and no
//! `set_high_numwords`/`merge`, so Python's `Num2Word_Base.__init__` never
//! builds `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `_int_to_word`, a recursive Indian-system decomposition
//! (thousand / lakh / crore / arab / kharab). `cards`/`merge` therefore stay
//! at their trait defaults, and Python has **no overflow check**: the `else`
//! arm of `_int_to_word` recurses on `number // 10**11` forever, so
//! arbitrarily large values stack `ખર્વ` suffixes instead of raising. On a
//! large enough integer the port's recursion overflowed the native stack, so
//! it adds a ceiling (gladiaio/num2words2#203): `maxval` is 10^22, where the
//! multiplier would itself need `ખર્વ`, and every mode raises `OverflowError`
//! from there.
//!
//! Inherited from `Num2Word_Base` (GU overrides all four in-scope modes, so
//! nothing is left to the trait defaults):
//!   * `to_cardinal`, `to_ordinal`, `to_ordinal_num`, `to_year` — all defined
//!     in `lang_GU.py` itself.
//!
//! `setup()` sets `negword = "ઋણ "` (note the trailing space) and
//! `pointword = "દશાંશ"`. Both are overridden below for completeness even
//! though the overridden `to_cardinal` reaches for the constants directly and
//! the `pointword` path is float-only (out of scope).
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following all look wrong but are exactly
//! what Python emits, verified against `bench/corpus.jsonl`:
//!
//! 1. **No compound tens (fixed, gladiaio/num2words2#247).** Python's
//!    `_int_to_word` glued the tens word and the ones word with a plain space
//!    ("વીસ એક" for 21, "નેવું નવ" for 99) instead of the real, irregular
//!    Gujarati words. The port reads [`BELOW_HUNDRED`]: 21 is "એકવીસ", 42
//!    "બેતાલીસ", 99 "નવ્વાણું".
//! 2. **`to_ordinal_num` mixed numeral systems (fixed, #224).** Python gave
//!    Gujarati digits for 1/2/3 ("૧લો", "૨જો", "૩જો") but `str(number) + "મો"`,
//!    i.e. ASCII digits, for everything else ("5મો"). The port writes ASCII
//!    digits throughout, the script the module uses for every other number:
//!    "1લો", "2જો", "3જો", "4મો".
//! 3. **`to_ordinal_num` has no case for 3's neighbours.** `3` maps to "3જો",
//!    the same suffix Python gives `2` ("2જો"); there is no "૩ત્રીજો"-style
//!    form. Kept verbatim.
//! 4. **Negative ordinals are not special-cased.** `to_ordinal(-1)` does not
//!    hit the `number == 1` arm, so it falls through to `cardinal + "મો"` ==
//!    "ઋણ એકમો" ("negative one-th"). Likewise `to_ordinal_num(-1)` ==
//!    "-1મો". Corpus-confirmed; no exception is raised.
//! 5. **`to_ordinal` computes the cardinal it then throws away.** For
//!    1/2/3/4/6 the `cardinal` local is built and discarded. Reproduced (it is
//!    side-effect free, but keeping it preserves the crash surface if
//!    `to_cardinal` ever raises).
//! 6. (Python's unreachable `tens[1]` went with the `tens` table, #247.)
//! 7. **`ones[0]` is the empty string** and `_int_to_word`'s `number < 0` arm
//!    is dead code on every in-scope path (`to_cardinal` strips the sign
//!    before recursing, so `_int_to_word` only ever sees non-negatives; only
//!    the out-of-scope `to_currency` could reach it). Both are preserved
//!    verbatim — see [`int_to_word`].
//!
//! # Scale cascade
//!
//! The thresholds are the Indian system, and the recursion on the top arm is
//! what lets the Python module swallow unbounded input (the port stops at
//! 10^22):
//!
//! | range                   | divisor  | word   |
//! |-------------------------|----------|--------|
//! | `< 10^5`                | `10^3`   | હજાર   |
//! | `< 10^7`                | `10^5`   | લાખ    |
//! | `< 10^9`                | `10^7`   | કરોડ   |
//! | `< 10^11`               | `10^9`   | અબજ    |
//! | otherwise               | `10^11`  | ખર્વ   |
//!
//! So `10^21` == "દસ અબજ ખર્વ" (10^21 // 10^11 == 10^10 → "દસ અબજ") and
//! `10^15` == "દસ હજાર ખર્વ". Both corpus-confirmed.
//!
//! # Errors
//!
//! Below the 10^22 ceiling none of the four in-scope modes can raise for
//! integer input: there is no card table to overflow, no dict lookup to miss, and no list index that can
//! go out of range (every index is arithmetically bounded to 0..=9). Every
//! `gu` row in the corpus for cardinal/ordinal/ordinal_num/year is `ok: true`.
//!
//! # Currency
//!
//! `Num2Word_GU` declares a four-entry `CURRENCY_FORMS` (INR/USD/EUR/GBP) and
//! overrides `to_currency` **wholesale** — it never calls `super()`, never
//! touches `parse_currency_parts`, and never consults `CURRENCY_PRECISION`.
//! Everything else in the currency surface is `Num2Word_Base`'s:
//!
//!   * `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION` — both inherited as the
//!     empty dict (GU subclasses `Num2Word_Base` directly, not `lang_EUR` /
//!     `lang_EU`), so `currency_adjective` is always absent and
//!     `currency_precision` is `.get(code, 100)` == **100 for every code**.
//!     Both trait defaults already say exactly that, so neither is overridden.
//!   * `pluralize` — abstract in Python and *unreachable* for GU: the
//!     overridden `to_currency` inlines its own `cr[0]`/`cr[1]` choice and
//!     `to_cheque` takes the plural unconditionally. Left at the trait default
//!     (which raises, as `Num2Word_Base.pluralize` does).
//!   * `_money_verbose` / `_cents_verbose` / `_cents_terse` — not overridden by
//!     GU; the first two are `to_cardinal` and the third is
//!     `default_cents_terse`. GU's `to_currency` calls none of them (it reaches
//!     for `_int_to_word` directly), but `to_cheque` does use `_money_verbose`.
//!   * `to_cheque` — not overridden by GU, so `default_to_cheque` applies
//!     verbatim. It is the *only* consumer of the `currency_forms` trait hook
//!     here, and the reason that hook must report a genuine `None` for an
//!     unknown code (see [`LangGu::forms_or_inr`]).
//!
//! ## Faithfully reproduced Python quirks — currency
//!
//! 8. **An unknown currency code silently becomes INR.** `to_currency` does
//!    `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`, so
//!    `to_currency(1.0, "JPY")` == "એક રૂપિયો" — a *rupee*, not a yen — and
//!    likewise for KWD/BHD/CNY/CHF and any other code on earth. No
//!    NotImplementedError is ever raised from this path. All 108 `gu` currency
//!    corpus rows are `ok: true` because of this; the nine non-INR/USD/EUR/GBP
//!    codes in the corpus all render as rupees.
//! 9. **…but `to_cheque` does *not* share that fallback.** It is base's, and
//!    base does `self.CURRENCY_FORMS[currency]` inside `try/except KeyError`,
//!    raising NotImplementedError. So the same code splits: `currency:JPY`
//!    succeeds (as rupees) while `cheque:JPY` raises. Corpus-confirmed on both
//!    sides — five of the nine cheque rows are `NotImplementedError`.
//! 10. **`CURRENCY_PRECISION` is never consulted by `to_currency`**, which
//!    hardcodes two decimal places via `parts[1][:2]`. The 3-decimal (KWD,
//!    BHD) and 0-decimal (JPY) currencies therefore get plain hundredths:
//!    `to_currency(12.34, "KWD")` == "બાર રૂપિયા અને ત્રીસ ચાર પૈસા", with no
//!    divisor-1000 mils and no divisor-1 rounding-away-of-cents. Base's
//!    `divisor == 1` int-coercion never runs either. Corpus-confirmed.
//! 11. **`cents=False` drops the cents segment entirely** rather than
//!    switching it to terse digits: the guard is `if cents and right:`, so GU
//!    never reaches `_cents_terse`. Base would have rendered "… , 34"; GU
//!    renders "બાર રૂપિયા". Not corpus-covered (`diff_test.py` always passes
//!    `cents=True`) but read straight off the source.
//! 12. **`adjective=True` is accepted and ignored.** GU's `to_currency` takes
//!    the parameter and never looks at `CURRENCY_ADJECTIVES` — which is empty
//!    anyway, so even base's path would have been a no-op.
//! 13. **The cents value is truncated, never rounded.** `parts[1][:2]` slices
//!    the decimal string, so `to_currency(0.567)` == "… અને પચાસ છ પૈસા" (56,
//!    not 57). Base quantizes with ROUND_HALF_UP first; GU does not.
//! 14. **Sub-hundredth digits vanish silently**, so `to_currency(0.004)`
//!    yields no cents segment at all ("શૂન્ય રૂપિયા"), where base would have
//!    routed 0.4 fractional cents through the float path.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `_int_to_word`'s early return for 0.
const ZERO_WORD: &str = "શૂન્ય";

/// `setup(): self.negword`. The trailing space is load-bearing in Python —
/// `to_cardinal` concatenates it directly (`ret + self._int_to_word(...)`)
/// rather than joining, so it is what separates "ઋણ" from the number word.
const NEGWORD: &str = "ઋણ ";

/// `setup(): self.pointword`. Float-only, so unreachable from the four
/// in-scope modes; kept so the trait's `pointword()` reports the truth.
const POINTWORD: &str = "દશાંશ";

/// The cardinals 1..=99, indexed by value (gladiaio/num2words2#247).
///
/// Python had `ones`/`teens`/`tens` and joined a ten and a unit with a space
/// ("વીસ એક" for 21), but Gujarati has its own word for every number below a
/// hundred. Index 0 is never read (the `number == 0` early return). 21..=99
/// from learningujarati.com, saralgujarati.in and atozworksheet.com, which
/// agree entry for entry; 0..=20 and the round tens also match Wikibooks
/// and languagesandnumbers.com. The module's ચાલીસ (40) is kept, so the 40s
/// use -તાલીસ (Wiktionary also has ચાળીસ / -તાળીસ).
const BELOW_HUNDRED: [&str; 100] = [
    "", "એક", "બે", "ત્રણ", "ચાર", "પાંચ", "છ", "સાત", "આઠ", "નવ", // 0..9
    "દસ", "અગિયાર", "બાર", "તેર", "ચૌદ", "પંદર", "સોળ", "સત્તર", "અઢાર", "ઓગણીસ", // 10..19
    "વીસ", "એકવીસ", "બાવીસ", "તેવીસ", "ચોવીસ", "પચ્ચીસ", "છવીસ", "સત્તાવીસ", "અઠ્ઠાવીસ", "ઓગણત્રીસ", // 20..29
    "ત્રીસ", "એકત્રીસ", "બત્રીસ", "તેત્રીસ", "ચોત્રીસ", "પાંત્રીસ", "છત્રીસ", "સાડત્રીસ", "આડત્રીસ", "ઓગણચાલીસ", // 30..39
    "ચાલીસ", "એકતાલીસ", "બેતાલીસ", "ત્રેતાલીસ", "ચુંમાલીસ", "પિસ્તાલીસ", "છેતાલીસ", "સુડતાલીસ", "અડતાલીસ", "ઓગણપચાસ", // 40..49
    "પચાસ", "એકાવન", "બાવન", "ત્રેપન", "ચોપન", "પંચાવન", "છપ્પન", "સત્તાવન", "અઠ્ઠાવન", "ઓગણસાઠ", // 50..59
    "સાઠ", "એકસઠ", "બાસઠ", "ત્રેસઠ", "ચોસઠ", "પાંસઠ", "છાસઠ", "સડસઠ", "અડસઠ", "અગણોસિત્તેર", // 60..69
    "સિત્તેર", "એકોતેર", "બોતેર", "તોતેર", "ચુમોતેર", "પંચોતેર", "છોતેર", "સિત્યોતેર", "ઇઠ્યોતેર", "ઓગણાએંસી", // 70..79
    "એંસી", "એક્યાસી", "બ્યાસી", "ત્યાસી", "ચોર્યાસી", "પંચાસી", "છ્યાસી", "સિત્યાસી", "ઈઠ્યાસી", "નેવ્યાસી", // 80..89
    "નેવું", "એકાણું", "બાણું", "ત્રાણું", "ચોરાણું", "પંચાણું", "છન્નું", "સત્તાણું", "અઠ્ઠાણું", "નવ્વાણું", // 90..99
];

/// Scale words. Each carries Python's leading space, because `_int_to_word`
/// writes `self._int_to_word(...) + " હજાર"` — the space is part of the
/// literal, not a join.
const HUNDRED: &str = " સો";
const THOUSAND: &str = " હજાર";
const LAKH: &str = " લાખ";
const CRORE: &str = " કરોડ";
const ARAB: &str = " અબજ";
const KHARAB: &str = " ખર્વ";

/// `to_ordinal`'s irregular forms. Python special-cases 1, 2, 3, 4 and 6 —
/// note that **5 is absent**, so `to_ordinal(5)` == "પાંચમો" via the suffix.
const ORD_1: &str = "પહેલો";
const ORD_2: &str = "બીજો";
const ORD_3: &str = "ત્રીજો";
const ORD_4: &str = "ચોથો";
const ORD_6: &str = "છઠ્ઠો";
/// `to_ordinal`'s fallback suffix, appended with **no** separator.
const ORD_SUFFIX: &str = "મો";

/// `to_ordinal_num`'s irregular forms — Gujarati digits, unlike the fallback.
const ORD_NUM_1: &str = "1લો";
const ORD_NUM_2: &str = "2જો";
const ORD_NUM_3: &str = "3જો";

/// `to_year`'s era prefixes. Both carry Python's trailing space.
const YEAR_BC: &str = "ઈસવીસન પૂર્વે ";
const YEAR_AD: &str = "સન ";

/// `Num2Word_GU.to_currency`'s own default `separator=" અને "` ("and").
///
/// See [`SEPARATOR_UNSET`] for why this cannot just be a parameter default.
const SEPARATOR_DEFAULT: &str = " અને ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_GU.to_currency` declares `separator=" અને "`, but the `Lang` trait
/// has no per-language parameter defaults, and the value is resolved on the
/// Python side *before* it crosses the boundary: `__init__.py`'s currency fast
/// path passes `kwargs.get("separator", ",")` and `bench/diff_test.py` passes a
/// literal `","` — in both cases **`Num2Word_Base`'s** default, not GU's. By
/// the time Rust sees it, "caller omitted separator" and "caller explicitly
/// passed a comma" are the same string and the information needed to tell them
/// apart is gone.
///
/// So `,` is read back as the unset sentinel and GU's own default restored.
/// This is the only reading that matches the oracle: every one of the 45 float
/// rows in the `gu` currency corpus was generated with no `separator=` kwarg
/// and expects " અને " (e.g. "બાર યૂરો અને ત્રીસ ચાર સેન્ટ").
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " અને " here where Python would give ",". Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report. The same
/// convention is used by `lang_ca.rs`, whose Python default is `" amb"`.
const SEPARATOR_UNSET: &str = ",";

/// `int(x)` for a **non-negative** `BigDecimal` — truncation toward zero.
///
/// Agrees with `BigDecimal::with_scale(0).as_bigint_and_exponent().0`, which
/// is how `currency.rs` spells the same operation: bigdecimal 0.4.10's
/// `with_scale` is plain integer division on the mantissa, i.e. truncation, at
/// every scale. This is written out longhand for two reasons:
///
///   * Truncation-not-rounding is the whole point here — it is what makes the
///     arithmetic below equal to Python's `parts[1][:2]` string slice (see
///     [`LangGu::to_currency`]). Reaching for a general-purpose rescale method
///     would leave that load-bearing detail resting on a dependency's default
///     rounding mode, which `with_scale_round` shows is a knob.
///   * The `exp > 0` guard. `with_scale` would materialise `10^exp` for a
///     pathological scale such as `Decimal("1E-1000000000")`; once the divisor
///     out-digits the mantissa the quotient is 0 regardless, so short-circuit.
///
/// Callers pass `abs()`-ed values only, so `BigInt`'s truncating division
/// agrees with Python's `int()` (also truncation) and the floor/truncate
/// distinction never bites.
fn trunc_nonneg(d: &BigDecimal) -> BigInt {
    let (mantissa, exp) = d.as_bigint_and_exponent();
    if exp > 0 {
        if exp as u128 > mantissa.to_string().len() as u128 {
            return BigInt::zero();
        }
        mantissa / BigInt::from(10).pow(exp as u32)
    } else if exp < 0 {
        // Negative scale == the source string was in exponent notation
        // ("1e+16"). Python would have raised ValueError on it; see the port
        // report. Scaling the mantissa up at least keeps the value right.
        mantissa * BigInt::from(10).pow(exp.unsigned_abs() as u32)
    } else {
        mantissa
    }
}

/// Port of `Num2Word_GU._int_to_word`.
///
/// The `number < 0` arm is dead on every in-scope path — `to_cardinal` strips
/// the sign before calling in — but it exists in Python and is reproduced
/// verbatim rather than dropped.
///
/// All `/` and `%` below run on strictly positive values (the zero and
/// negative arms return first), so Rust's truncating division agrees with
/// Python's floor division and there is no sign-of-remainder divergence.
///
/// Recursion depth is bounded by `digits / 11` on the `ખર્વ` arm (~55 frames
/// for a 606-digit input), so no stack guard is needed.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Dead code from to_cardinal/to_ordinal/to_year (sign already stripped);
    // reachable in Python only via the out-of-scope to_currency.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    // number is now in 1..=inf, so to_usize() is safe wherever it is bounded.
    if number < &BigInt::from(100) {
        return BELOW_HUNDRED[number.to_usize().expect("bounded by 100")].to_string();
    }

    if number < &BigInt::from(1000) {
        let n = number.to_usize().expect("bounded by 1000");
        // ones[number // 100] + " સો" — the divisor is 1..=9 here, never 0.
        let mut result = format!("{}{}", BELOW_HUNDRED[n / 100], HUNDRED);
        let remainder = n % 100;
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word(&BigInt::from(remainder)));
        }
        return result;
    }

    // The Indian-system cascade: each arm is
    //   self._int_to_word(number // div) + word  [+ " " + recurse(number % div)]
    for (limit, div, word) in [
        (100_000u64, 1_000u64, THOUSAND),
        (10_000_000, 100_000, LAKH),
        (1_000_000_000, 10_000_000, CRORE),
        (100_000_000_000, 1_000_000_000, ARAB),
    ] {
        if number < &BigInt::from(limit) {
            return scale(number, &BigInt::from(div), word);
        }
    }

    // else: Kharab. Unbounded — recurses on number // 10**11, which is exactly
    // why this module never raises OverflowError.
    scale(number, &BigInt::from(100_000_000_000u64), KHARAB)
}

/// One arm of the scale cascade, shared verbatim by હજાર/લાખ/કરોડ/અબજ/ખર્વ.
fn scale(number: &BigInt, div: &BigInt, word: &str) -> String {
    let mut result = format!("{}{}", int_to_word(&(number / div)), word);
    let remainder = number % div;
    if !remainder.is_zero() {
        result.push(' ');
        result.push_str(&int_to_word(&remainder));
    }
    result
}

// ---- float / Decimal path ------------------------------------------------
//
// `Num2Word_GU` does **not** inherit `Num2Word_Base.to_cardinal_float`
// (float2tuple + the `< 0.01` rescue heuristic). Instead its overridden
// `to_cardinal` handles non-integers *inline* off `str(number)`:
//
// ```python
// n = str(number).strip()
// if n.startswith("-"):
//     n = n[1:]; ret = self.negword
// else:
//     ret = ""
// if "." in n:
//     left, right = n.split(".", 1)
//     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//     ret += " ".join(self._int_to_word(int(d)) for d in right)
//     return ret.strip()
// else:
//     return (ret + self._int_to_word(int(n))).strip()
// ```
//
// So the fractional part is the *literal digit string* of `str(number)`
// spoken digit-by-digit — never a `post` integer, never zero-padded to a
// precision, and the f64-artefact `< 0.01` heuristic never runs. The entire
// specification is therefore Python's `repr(float)` / `str(Decimal)`, which is
// reproduced exactly by [`py_float_repr`] / [`py_decimal_str`]. `precision`
// (and the `precision=` kwarg) is set on the converter by `__init__.py` but
// GU's `to_cardinal` never reads `self.precision`, so it is ignored here too.
//
// This mirrors the already-verified `lang_as.rs` (Assamese) helpers verbatim;
// GU differs only in that it wraps the result in `.strip()` (a no-op on every
// reachable output, applied in `to_cardinal_float` for fidelity).

/// Shortest round-trip decimal digits of `a` and its decimal point position,
/// matching CPython/Gay's dtoa. Returns `(digits, decpt)` where the value is
/// `0.<digits> * 10**decpt`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with dtoa on the digit
/// *count* and almost always the digits; it disagrees only on **exact ties**,
/// where dtoa picks the candidate with an even last digit while Rust rounds
/// half up. The block below detects that tie with no bignum (write
/// `a = m * 2**e` with `m` odd, `q = digits.len() - decpt`; the tie is
/// `e + q + 1 == 0`, plus `5**-q | m` when `q < 0`) and steps to the even
/// neighbour, exactly as `lang_as.rs` documents and fuzzes.
fn shortest_digits(a: f64) -> (String, i32) {
    let sci = format!("{:e}", a);
    let (mant, exp) = sci
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let mut digits: Vec<u8> = mant.bytes().filter(|c| *c != b'.').collect();
    let mut decpt: i32 = exp.parse::<i32>().expect("{:e} exponent is an integer") + 1;

    // Decompose a == m * 2**e exactly, then reduce m to odd.
    let bits = a.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let frac = bits & ((1u64 << 52) - 1);
    let (mut m, mut e) = if biased == 0 {
        (frac, -1074i32) // subnormal: no implicit leading bit
    } else {
        (frac | (1u64 << 52), biased - 1075)
    };
    if m == 0 {
        // a == 0.0: dtoa reports digits "0", decpt 1. No tie to break.
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }
    let z = m.trailing_zeros() as i32;
    m >>= z;
    e += z;

    let q = digits.len() as i32 - decpt;
    let mut tie = e + q + 1 == 0;
    if tie && q < 0 {
        let r = -q as u32;
        tie = r <= 22 && m % 5u64.pow(r) == 0;
    }
    if !tie {
        return (String::from_utf8(digits).expect("ASCII digits"), decpt);
    }

    let last = digits[digits.len() - 1] - b'0';
    if last % 2 == 1 {
        if m % 4 == 1 {
            // k even: Python wants k, Rust gave k+1. Last digit is odd (hence
            // non-zero), so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k odd: Python wants k+1, Rust gave k. Carry like dtoa's roundoff.
            let mut i = digits.len();
            loop {
                if i == 0 {
                    digits.insert(0, b'1');
                    decpt += 1;
                    break;
                }
                i -= 1;
                if digits[i] == b'9' {
                    digits[i] = b'0';
                } else {
                    digits[i] += 1;
                    break;
                }
            }
        }
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), CPython's
/// `format_float_short(..., 'r', ...)`. Rust's own `{}` cannot stand in: it
/// never switches to exponent notation and prints `1`, not `1.0`, for integral
/// Python's `number == k` for a small integer `k` — numeric equality across
/// both FloatValue arms (`1.0 == 1` and `Decimal("1.00") == 1` are both
/// True). NaN/inf compare unequal, matching Python.
fn float_eq_int(v: &FloatValue, k: i64) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value == k as f64,
        FloatValue::Decimal { value, .. } => value == &BigDecimal::from(k),
    }
}

/// floats. Rules from `format_float_short`: exponent notation iff
/// `decpt <= -4 || decpt > 16`; the exponent is `%+.02d`; a trailing `.0` is
/// appended to an otherwise-integral fixed result but never in exponent form;
/// `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and the sign is
    // stripped textually into a negword.
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let (digits, decpt) = shortest_digits(value.abs());
    let ndigits = digits.len() as i32;

    if decpt <= -4 || decpt > 16 {
        let exp = decpt - 1;
        let mut mant = String::from(&digits[..1]);
        if digits.len() > 1 {
            mant.push('.');
            mant.push_str(&digits[1..]);
        }
        format!(
            "{}{}e{}{:02}",
            sign,
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        format!(
            "{}{}{}.0",
            sign,
            digits,
            "0".repeat((decpt - ndigits) as usize)
        )
    } else {
        let d = decpt as usize;
        format!("{}{}.{}", sign, &digits[..d], &digits[d..])
    }
}

/// Python's `str(Decimal)` — `Decimal.__str__` with the default context.
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`; the shim builds the value with
/// `BigDecimal::from_str(str(value))`, which preserves trailing zeros and
/// negative exponents (so `"1.10"` round-trips as `(110, 2)`). The
/// `leftdigits > -6` guard keeps every `"0".repeat(...)` bounded.
///
/// Caveat (same as `lang_as.rs`): a `BigDecimal` cannot represent `-0.0`, so a
/// fixed-notation negative zero loses its negword here. Beyond the exponent
/// threshold `int()` raises `ValueError` with a message that agrees either way.
/// Flagged in the port report.
fn py_decimal_str(value: &BigDecimal) -> String {
    let (int_val, scale) = value.as_bigint_and_exponent();
    // i128 so that `-scale` cannot overflow for a pathological i64::MIN scale.
    let exp = -(scale as i128);
    let sign = if int_val.is_negative() { "-" } else { "" };
    let int_digits = int_val.abs().to_string(); // Decimal._int
    let len = int_digits.len() as i128;

    let leftdigits = exp + len;
    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat(-dotplace as usize), int_digits),
        )
    } else if dotplace >= len {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - len) as usize)),
            String::new(),
        )
    } else {
        let d = dotplace as usize;
        (int_digits[..d].to_string(), format!(".{}", &int_digits[d..]))
    };

    let expstr = if leftdigits == dotplace {
        String::new()
    } else {
        // "%+d" — signed, but not zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)` for the ASCII fragments [`cardinal_from_str`] hands it.
/// Ports the underscore rule (`int("1_0") == 10`, `int("1_")` raises) and the
/// error message (`repr(s)` of the original argument).
fn py_int(s: &str) -> Result<BigInt> {
    let err = || {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    };
    let t = s.trim();
    let (negative, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body.chars().all(|c| c.is_ascii_digit() || c == '_')
    {
        return Err(err());
    }
    let digits: String = body.chars().filter(|c| *c != '_').collect();
    let n: BigInt = digits.parse().map_err(|_| err())?;
    Ok(if negative { -n } else { n })
}

/// The body of `Num2Word_GU.to_cardinal` for a stringified number `n`, minus
/// the outer `.strip()` (applied by the caller). The sign is stripped
/// *textually*, `split(".", 1)` caps at one split (a second dot detonates in
/// the digit loop), and `int(left)` runs before the digit generator so a
/// malformed left half is the first thing to raise.
fn cardinal_from_str(number: &str) -> Result<String> {
    let n = number.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    let Some(dot) = n.find('.') else {
        ret.push_str(&checked_int_to_word(&py_int(n)?)?);
        return Ok(ret);
    };

    // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots.
    let (left, right) = (&n[..dot], &n[dot + 1..]);
    ret.push_str(&checked_int_to_word(&py_int(left)?)?);
    ret.push(' ');
    ret.push_str(POINTWORD);
    ret.push(' ');

    // " ".join(self._int_to_word(int(d)) for d in right) — iterate *characters*.
    let mut first = true;
    for d in right.chars() {
        if !first {
            ret.push(' ');
        }
        first = false;
        let mut buf = [0u8; 4];
        ret.push_str(&checked_int_to_word(&py_int(d.encode_utf8(&mut buf))?)?);
    }
    Ok(ret)
}

pub struct LangGu {
    /// `Num2Word_GU.CURRENCY_FORMS`, verbatim and complete — GU declares all
    /// four entries itself and inherits none (`Num2Word_Base.CURRENCY_FORMS`
    /// is `{}`), so this table *is* the whole dict.
    ///
    /// Every entry is 2+2, and the singular/plural collapse to the same word
    /// for USD/EUR/GBP — only INR actually inflects ("રૂપિયો"/"રૂપિયા",
    /// "પૈસો"/"પૈસા"). That is Python's data, not a transcription slip: this
    /// table was emitted directly from `Num2Word_GU.CURRENCY_FORMS`.
    ///
    /// Built once here rather than per call, as the porting contract requires.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangGu {
    pub fn new() -> Self {
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("INR", &["રૂપિયો", "રૂપિયા"][..], &["પૈસો", "પૈસા"][..]),
            ("USD", &["ડોલર", "ડોલર"][..], &["સેન્ટ", "સેન્ટ"][..]),
            ("EUR", &["યૂરો", "યૂરો"][..], &["સેન્ટ", "સેન્ટ"][..]),
            ("GBP", &["પાઉન્ડ", "પાઉન્ડ"][..], &["પેન્સ", "પેન્સ"][..]),
        ]
        .into_iter()
        .map(|(code, unit, subunit)| (code, CurrencyForms::new(unit, subunit)))
        .collect();
        LangGu { currency_forms }
    }

    /// `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`.
    ///
    /// Deliberately **not** routed through the `currency_forms` trait hook:
    /// that hook has to keep reporting `None` for an unknown code so
    /// `default_to_cheque` can raise NotImplementedError the way base's
    /// `try: self.CURRENCY_FORMS[currency] / except KeyError:` does. The
    /// INR fallback belongs to `to_currency` alone — module-doc quirks 8 & 9.
    ///
    /// The `["INR"]` lookup cannot fail: `new()` always inserts it.
    fn forms_or_inr(&self, code: &str) -> &CurrencyForms {
        match self.currency_forms.get(code) {
            Some(forms) => forms,
            None => &self.currency_forms["INR"],
        }
    }
}

impl Default for LangGu {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// ખર્વ (10^11), so from 10^22 its multiplier would itself need ખર્વ and the
/// word would stack ("એક ખર્વ ખર્વ").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(22))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl Lang for LangGu {
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
        "INR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " અને "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "દશાંશ"
    }

    /// Port of `Num2Word_GU.to_cardinal`.
    ///
    /// Python does `n = str(number).strip()`, peels a leading `"-"` into
    /// `ret = self.negword`, and — since an integer's `str()` never contains
    /// `"."` — falls into `return (ret + self._int_to_word(int(n))).strip()`.
    /// Peeling the sign off the decimal string is exactly `abs()`, so the
    /// round-trip through `str`/`int` is elided here.
    ///
    /// The trailing `.strip()` is a no-op for integer input (no arm of
    /// `_int_to_word` produces edge whitespace, and `negword`'s trailing space
    /// is always followed by a word) but is kept for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, checked_int_to_word(&n)?).trim().to_string())
    }

    /// Port of `Num2Word_GU.to_ordinal`.
    ///
    /// Python builds `cardinal` first and only then checks the irregulars, so
    /// 1/2/3/4/6 discard it. Reproduced. Note 5 is *not* irregular, and no
    /// arm matches negatives — `to_ordinal(-1)` == "ઋણ એકમો".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;

        if value == &BigInt::from(1) {
            return Ok(ORD_1.to_string());
        }
        if value == &BigInt::from(2) {
            return Ok(ORD_2.to_string());
        }
        if value == &BigInt::from(3) {
            return Ok(ORD_3.to_string());
        }
        if value == &BigInt::from(4) {
            return Ok(ORD_4.to_string());
        }
        if value == &BigInt::from(6) {
            return Ok(ORD_6.to_string());
        }
        Ok(format!("{}{}", cardinal, ORD_SUFFIX))
    }

    /// Port of `Num2Word_GU.to_ordinal_num`.
    ///
    /// The fallback is `str(number) + "મો"` — ASCII digits, like the irregular
    /// forms for 1/2/3 (#224). `BigInt::to_string()` matches Python's
    /// `str(int)` exactly (no separators, `-` prefix for negatives).
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        if value == &BigInt::from(1) {
            return Ok(ORD_NUM_1.to_string());
        }
        if value == &BigInt::from(2) {
            return Ok(ORD_NUM_2.to_string());
        }
        if value == &BigInt::from(3) {
            return Ok(ORD_NUM_3.to_string());
        }
        Ok(format!("{}{}", value, ORD_SUFFIX))
    }

    /// Port of `Num2Word_GU.to_year`.
    ///
    /// Python's `longval=True` parameter is accepted and ignored. Negatives
    /// take the BC prefix over `to_cardinal(abs(val))`, so the `negword` never
    /// appears in a BC year.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("{}{}", YEAR_BC, self.to_cardinal(&value.abs())?))
        } else {
            Ok(format!("{}{}", YEAR_AD, self.to_cardinal(value)?))
        }
    }

    /// Port of `Num2Word_GU.to_cardinal`'s non-integer branch.
    ///
    /// GU does not use `Num2Word_Base.to_cardinal_float`; its overridden
    /// `to_cardinal` reads `str(number)` and speaks the fractional digits
    /// one at a time. So the raw f64 is turned back into `repr(float)` (or the
    /// Decimal into `str(Decimal)`) and fed to the string body verbatim. The
    /// `precision=` kwarg (`precision_override`) is set on the converter by
    /// `__init__.py` but `to_cardinal` never reads `self.precision`, so it is
    /// ignored, exactly as Python ignores it.
    ///
    /// Python wraps both return arms in `.strip()`; it is a no-op on every
    /// reachable output (the string always begins and ends with a word) but is
    /// applied here for fidelity.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let _ = precision_override;
        let n = match value {
            // Python's str(float); the raw f64 crossed the boundary so repr()
            // can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        Ok(cardinal_from_str(&n)?.trim().to_string())
    }

    /// `to_ordinal(float/Decimal)`. Python computes
    /// `cardinal = self.to_cardinal(number)` first (any exception — e.g. the
    /// ValueError from an exponent-form Decimal — propagates), then checks the
    /// irregulars with **numeric** equality, so `1.0`/`Decimal("1.00")` hit
    /// "પહેલો" while `5.0` falls through to the float-grammar cardinal plus
    /// "મો": "પાંચ દશાંશ શૂન્યમો".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        for (k, word) in [(1, ORD_1), (2, ORD_2), (3, ORD_3), (4, ORD_4), (6, ORD_6)] {
            if float_eq_int(value, k) {
                return Ok(word.to_string());
            }
        }
        Ok(format!("{}{}", cardinal, ORD_SUFFIX))
    }

    /// `to_ordinal_num(float/Decimal)` — numeric equality again for the three
    /// irregulars, everything else `str(number) + "મો"` with
    /// the repr verbatim: "4.0મો", "-0.0મો", "1e+16મો", "5.00મો".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        for (k, word) in [(1, ORD_NUM_1), (2, ORD_NUM_2), (3, ORD_NUM_3)] {
            if float_eq_int(value, k) {
                return Ok(word.to_string());
            }
        }
        Ok(format!("{}{}", repr_str, ORD_SUFFIX))
    }

    /// `to_year(float/Decimal)` — `if val < 0` is **numeric**, so `-0.0`
    /// takes the AD branch and keeps its negword inside the cardinal
    /// ("સન ઋણ શૂન્ય દશાંશ શૂન્ય"), while true negatives take the BC prefix
    /// over `to_cardinal(abs(val))`, negword gone:
    /// "ઈસવીસન પૂર્વે એક દશાંશ પાંચ".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let negative = match value {
            // Numeric `<`, NOT the sign bit: -0.0 < 0 is False.
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if negative {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: value.abs(),
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: value.abs(),
                    precision: *precision,
                },
            };
            Ok(format!("{}{}", YEAR_BC, self.cardinal_float_entry(&abs, None)?))
        } else {
            Ok(format!("{}{}", YEAR_AD, self.cardinal_float_entry(value, None)?))
        }
    }

    /// `converter.str_to_number` is Base's `Decimal(value)` (GU doesn't
    /// override it), but `Decimal("Infinity")` then hits GU's `to_cardinal`,
    /// where `str(number)` has no "." and `int("Infinity")` raises
    /// **ValueError** — not the OverflowError the binding's generic Inf arm
    /// would produce. NaN already maps to ValueError there.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            // GU's to_cardinal peels the "-" before int(), so the failing
            // literal is the unsigned "Infinity" for both signs.
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------
    //
    // Left at the trait default because GU inherits Num2Word_Base's:
    //   * `currency_adjective` — `CURRENCY_ADJECTIVES` is `{}`, and GU's
    //     `to_currency` would ignore it regardless (module-doc quirk 12).
    //   * `currency_precision` — `CURRENCY_PRECISION` is `{}`, so
    //     `.get(code, 100)` is 100 for every code, which is the default.
    //   * `pluralize` — abstract in Python, and unreachable for GU.
    //   * `money_verbose` / `cents_verbose` / `cents_terse` — not overridden.
    //   * `to_cheque` — not overridden; `default_to_cheque` is a faithful port
    //     of base's, and with `currency_forms` + `currency_precision` below it
    //     reproduces every `gu` cheque row, raises included.
    //   * `cardinal_from_decimal` — GU's `to_currency` never produces
    //     fractional cents (it truncates at two digits), so the float path is
    //     genuinely unreachable from here.

    fn lang_name(&self) -> &str {
        "Num2Word_GU"
    }

    /// `CURRENCY_FORMS[code]` — a strict lookup, missing key -> `None`.
    ///
    /// Only `to_cheque` reads this hook for GU, and it must raise on an
    /// unknown code. `to_currency` uses [`LangGu::forms_or_inr`] instead,
    /// which applies Python's `.get(code, CURRENCY_FORMS["INR"])` fallback.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_GU.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="INR", cents=True,
    ///                 separator=" અને ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///     if is_negative:
    ///         result = self.negword + result
    ///     return result.strip()
    /// ```
    ///
    /// This bypasses `default_to_currency` completely — no `pluralize`, no
    /// `parse_currency_parts`, no precision divisor, no adjective. It is a
    /// parallel implementation in Python and stays one here.
    ///
    /// # Why the int/decimal split still holds
    ///
    /// GU reaches the same conclusion base does, by a different route. Base
    /// branches on `isinstance(val, int)`; GU branches on whether `str(val)`
    /// contains a `"."`. For a true `int` it never does, so `right` is 0 and
    /// the cents segment is skipped — `to_currency(1, "INR")` == "એક રૂપિયો".
    /// For the float `1.0` the string *is* "1.0", so `parts[1]` == "0" and
    /// `right` == `int("00")` == 0 — which is **falsy**, so the segment is
    /// skipped again. The two agree on `1` vs `1.0` only by coincidence: base
    /// would have printed "એક રૂપિયો, શૂન્ય પૈસા" for the float, whereas GU's
    /// `if cents and right:` treats zero cents and no cents identically.
    /// Corpus rows `1` and `1.0` both expect "એક રૂપિયો", confirming it.
    ///
    /// # Why the string slicing is done as arithmetic
    ///
    /// Python slices the *decimal string*; the value reaches Rust already
    /// parsed into a `BigDecimal`, and re-deriving `repr(float)` to slice it
    /// again would be a second shortest-round-trip formatter (the exact thing
    /// `currency.rs`'s module doc warns against). The arithmetic below is
    /// provably the same function on every plain-decimal input:
    ///
    ///   * `int(parts[0])` — the integer-part digits of `str(abs(val))` — is
    ///     `trunc(abs(val))`.
    ///   * `int(parts[1][:2].ljust(2, "0"))` — first two fractional digits,
    ///     right-padded — is `trunc(frac * 100)`: truncating the digit string
    ///     at two places and truncating the scaled fraction are the same
    ///     operation ("5" -> "50" == 50 == trunc(0.5*100); "999" -> "99" == 99
    ///     == trunc(0.999*100); "004" -> "00" == 0 == trunc(0.004*100)).
    ///
    /// Verified equivalent against CPython over all 12 distinct corpus
    /// arguments plus 20,000 random values at 0-6 decimal places: zero
    /// mismatches. See the port report for the one input class where the
    /// string form is *not* recoverable (exponent notation).
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
        // Restore GU's own `separator=" અને "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`.
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")` and the two int() casts, as arithmetic
        // on the already-parsed value. An `Int` has no "." in its str(), so it
        // takes the `right = 0` arm by construction rather than by division.
        let (left, right) = match val {
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let abs = d.abs();
                let left = trunc_nonneg(&abs);
                // trunc(v*100) - 100*trunc(v) == trunc(frac*100), exactly:
                // left*100 is integral and 0 <= frac*100 < 100.
                let scaled = trunc_nonneg(&(&abs * BigDecimal::from(100)));
                let right = scaled - &left * BigInt::from(100);
                (left, right)
            }
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])`
        let forms = self.forms_or_inr(currency);
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`.
        // Indexing is unconditional in Python; every entry in the table `new()`
        // builds has exactly two forms, so [0] and [1] are always in range and
        // no IndexError arm is reachable.
        let mut result = format!(
            "{} {}",
            checked_int_to_word(&left)?,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — note `right` is tested for *truthiness*, so
        // zero cents are dropped rather than spelled out (quirk 11/14).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        if is_negative {
            // `self.negword` raw, trailing space and all — unlike base's
            // `"%s " % self.negword.strip()`. Same bytes here ("ઋણ "), but
            // this is what the source says.
            result = format!("{}{}", NEGWORD, result);
        }

        // `.strip()` — a no-op on every reachable output (the result always
        // starts and ends with a word), kept for fidelity.
        Ok(result.trim().to_string())
    }
}
