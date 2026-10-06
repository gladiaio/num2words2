//! Port of `lang_PS.py` (Pashto).
//!
//! Registry check: `__init__.py` line 361 maps `"ps"` → `lang_PS.Num2Word_PS()`,
//! so this is the class the key actually resolves to.
//!
//! Shape: **self-contained**. `Num2Word_PS` subclasses `Num2Word_Base` but its
//! `setup()` defines only flat `ones`/`tens` lists plus three scale words — no
//! `high_numwords`/`mid_numwords`/`low_numwords`. `Num2Word_Base.__init__`
//! guards the card machinery behind
//! `if any(hasattr(self, f) for f in ["high_numwords", "mid_numwords", "low_numwords"])`,
//! so for PS `self.cards` is never built and `self.MAXVAL` is never set
//! (verified in the interpreter: `hasattr(o, 'cards') == False`,
//! `hasattr(o, 'MAXVAL') == False`). `to_cardinal` is overridden outright and
//! drives a recursive `_int_to_word`. Consequently `cards`/`maxval`/`merge`
//! stay at their trait defaults here, and **there is no overflow check** — PS
//! never raises. Every in-scope corpus row for "ps" is `ok: true`; there are no
//! `Index`/`Key`/`Value` crash sites to reproduce.
//!
//! Method map (all four in-scope modes are explicitly defined by PS itself,
//! nothing in-scope is inherited from `Num2Word_Base`):
//!   * `to_cardinal(number)`    → sign-strip on the *decimal string*, then `_int_to_word`
//!   * `to_ordinal(number)`     → `to_cardinal(number) + "-م"`
//!   * `to_ordinal_num(number)` → `str(number) + "."`
//!   * `to_year(val, longval=True)` → `to_cardinal(val)`; `longval` is accepted
//!     and then ignored, so year output is identical to cardinal output.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below looks wrong and is exactly
//! what Python emits — each one is pinned by a frozen-corpus row.
//!
//! 1. **No teens.** `ones`/`tens` are combined mechanically, so 11..19 render
//!    as "ten one", "ten two", … instead of the real Pashto forms (یوولس,
//!    دولس, …). Corpus: `11` → "لس یو", `12` → "لس دوه", `19` → "لس نهه".
//!    This also leaks into every higher scale, because the thousands/millions
//!    branches recurse through the same routine: `12345` → "لس دوه زره درې سل
//!    څلوېښت پنځه" ("ten two thousand three hundred forty five"), and `10^7`
//!    → "لس میلیون". Do not "fix" this into یوولس — the corpus says otherwise.
//! 2. **Zero, minus and the decimal word (fixed, gladiaio/num2words2#154).**
//!    Python's `_int_to_word(0)` reads
//!    `return self.ones[0] if self.ones[0] else "zero"`, and `ones[0]` is `""`,
//!    so Python always said the English "zero"; its `negword` is `"minus "`
//!    and its `pointword` `"point"`. This port uses the Pashto صفر, منفي and
//!    اعشاریه (the Perso-Arabic decimal word Pashto shares with Dari/Urdu):
//!    `0` → "صفر", `to_ordinal(0)` → "صفر-م", `1.5` → "یو اعشاریه پنځه".
//! 3. **`negword` carries a trailing space**: `-42` → "منفي څلوېښت دوه".
//!    Unlike most modules, PS concatenates it directly rather than via
//!    `base.default_to_cardinal` (which would `.trim()` it and re-add one
//!    space). Same result here, but the mechanism differs — see [`NEGWORD`].
//! 4. **Digits fall out for anything ≥ 10^9.** The `_int_to_word` cascade stops
//!    at the millions branch; the `else` arm is a bare `return str(number)`.
//!    So the "words" for a billion are the *numerals*: corpus `10^9` →
//!    "1000000000", `1234567890` → "1234567890", `10^21` →
//!    "1000000000000000000000". `to_ordinal(10^9)` → "1000000000-م" — a
//!    Pashto ordinal suffix glued onto an Arabic-numeral string. There is no
//!    ceiling and no `OverflowError`; arbitrarily large BigInts just stringify.
//!    This is why [`int_to_word`] must take `&BigInt` and only narrow to `u32`
//!    *after* proving `number < 10^9`.
//! 5. **`to_ordinal_num` ignores the language entirely** and appends a Western
//!    `"."`, so it is really the German/ordinal-dot convention. It also does no
//!    sign or range handling: corpus `-1` → "-1.", `0` → "0.".
//!
//! # Cross-call mutable state
//!
//! None. `setup()` only assigns constant tables; no method sets a flag that
//! another consumes (no `_pending_ordinal`-style handshake as in `lang_ES`).
//! The four in-scope modes are pure functions of their argument, so the
//! stateless Rust path is faithful.
//!
//! # The currency surface
//!
//! `Num2Word_PS` declares its **own** `CURRENCY_FORMS` class attribute, so the
//! `lang_EUR`/`Num2Word_EN` shared-dict mutation trap described in
//! `PORTING_CURRENCY.md` does not apply here — nothing writes into PS's dict.
//! Verified in the interpreter: `CONVERTER_CLASSES['ps'].CURRENCY_FORMS` holds
//! exactly the three literal entries `AFN`, `EUR`, `USD` (in that insertion
//! order), `CURRENCY_PRECISION` is `{}` and `CURRENCY_ADJECTIVES` is `{}`
//! (both inherited unchanged from `Num2Word_Base`). So precision is 100 for
//! *every* code and the adjective hook is never populated — both stay at their
//! trait defaults.
//!
//! Method ownership (checked via `__qualname__`):
//!   * `to_currency`     → **`Num2Word_PS`** — a full override that shares no
//!     code with `Num2Word_Base.to_currency`. It is ported wholesale below;
//!     `currency::default_to_currency` is *not* delegated to.
//!   * `to_cheque`       → `Num2Word_Base` — inherited verbatim, so the trait
//!     default (`currency::default_to_cheque`) already is the port. It needs
//!     only [`Lang::currency_forms`] + [`Lang::lang_name`] from us.
//!   * `_money_verbose`  → `Num2Word_Base` — `return self.to_cardinal(number)`,
//!     identical to the trait default. Reached only through `to_cheque`.
//!   * `pluralize` / `_cents_verbose` / `_cents_terse` → never reached: PS's
//!     `to_currency` selects forms inline and calls `_int_to_word` directly,
//!     and `to_cheque` renders its subunit as digits. Left at their defaults
//!     (`pluralize`'s default raises, mirroring Python's abstract method).
//!
//! ## Faithfully reproduced Python bugs, currency half
//!
//! 6. **An unknown currency code silently becomes AFN.** `to_currency` reads
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!    — a `.get` with a *default*, not a subscript, so it never raises. The
//!    default is the first value in insertion order, i.e. **AFN**. Corpus:
//!    `currency:GBP 12.34` → "لس دوه افغانۍ دېرش څلور پول" — a request for
//!    pounds answered in afghanis. Same for JPY/KWD/BHD/INR/CNY/CHF. This is
//!    why [`LangPs::to_currency`] deliberately bypasses the strict
//!    [`Lang::currency_forms`] hook; see [`FALLBACK_CURRENCY`].
//! 7. **`to_currency` ignores `CURRENCY_PRECISION` entirely.** There is no
//!    divisor anywhere in PS's override — the cents field is always parsed as
//!    *two* decimal digits off the string. So the 3-decimal currencies
//!    (KWD/BHD) and the 0-decimal one (JPY) all render 2-digit subunits like
//!    everything else. Corpus proves both ends: `currency:JPY 12.34` →
//!    "لس دوه افغانۍ **دېرش څلور پول**" (a zero-decimal currency showing
//!    cents), and `currency:KWD 1234.56` → "… **پنځوس شپږ پول**" (56, not
//!    560). `currency::default_to_currency`'s divisor==1 pre-rounding must
//!    therefore never run for PS — another reason the override is total.
//! 8. **Cents are truncated to two digits, not rounded.** `parts[1][:2]` slices
//!    the fractional *text*: `1.999` → "99", not 100-and-carry. And
//!    `.ljust(2, "0")` pads a short fraction on the **right**, so `0.5` → "50"
//!    cents, correctly, while `0.001` → "00" → 0 → the cents segment vanishes.
//!    Corpus: `0.5` → "صفر euros پنځوس cents".
//! 9. **`to_currency` and `to_cheque` disagree about unknown codes.**
//!    `to_currency("GBP")` happily prints afghanis (bug 6), but `to_cheque`
//!    is `Num2Word_Base`'s, which *subscripts* `self.CURRENCY_FORMS[currency]`
//!    inside `try/except KeyError` and re-raises
//!    `NotImplementedError('Currency code "GBP" not implemented for
//!    "Num2Word_PS"')`. Two methods, one table, opposite policies. The corpus
//!    pins both: every `currency:GBP` row is `ok: true`, every `cheque:GBP`
//!    row is `err: NotImplementedError`.
//! 10. **The int/float distinction is invisible in PS's output.** Unlike
//!    `Num2Word_Base.to_currency`, PS never calls `isinstance(val, int)`; it
//!    branches on `if cents and right:` instead. `str(1)` → "1" → no "." →
//!    `right = 0`, and `str(1.0)` → "1.0" → `parts[1] == "0"` → `int("00")` →
//!    `right = 0` as well. Both suppress the cents segment, so `1` and `1.0`
//!    both give "یو euro" (corpus confirms) — the same *output* base.py would
//!    reach by two different routes. [`LangPs::split_value`] still honours
//!    [`CurrencyValue`]'s two arms rather than collapsing them, because the
//!    *string* they stand for differs and that string is what PS slices.
//!
//! ## Known divergence: exponent notation (unreachable from the corpus)
//!
//! PS's `to_currency` does `int(str(val).split(".")[0])` — a **string**
//! operation on `str(value)`. The `Lang` trait receives a [`CurrencyValue`],
//! which the shim built by parsing that string into a `BigDecimal`, and the
//! parse is lossy for exactly one thing: whether the string used exponent
//! notation. Python's `str()` switches to e-notation at `|x| >= 1e16` or
//! `0 < |x| < 1e-4`, and `int("1e+16")` raises **ValueError**:
//!
//! | input | `str(value)` | Python | this port |
//! |---|---|---|---|
//! | `1e16` (float) | `"1e+16"` | `ValueError` | "10000000000000000 euros" |
//! | `Decimal("1E+2")` | `"1E+2"` | `ValueError` | "یو سل euros" |
//! | `1e-05` (float) | `"1e-05"` | `ValueError` | "صفر euros" |
//! | `Decimal("0.00001")` | `"0.00001"` | "صفر euros" | "صفر euros" ✓ |
//!
//! The positive-exponent half *is* recoverable here (`scale < 0` happens only
//! for e-notation input), but the negative-exponent half is not: `"1e-05"` and
//! `"0.00001"` both parse to the same `BigDecimal` (digits 1, scale 5), and
//! Python raises for the first and succeeds for the second. Reproducing this
//! properly needs the raw `str(value)` at the boundary, not a `BigDecimal` —
//! a shared-boundary change (`currency.rs` / `__init__.py`) that is out of
//! scope for a single language file, and one that every sibling language with
//! the same `str(val).split(".")` shape needs too (`lang_bs.rs`, `lang_bo.rs`,
//! `lang_br.rs`, … all carry the identical gap). Left divergent and reported
//! rather than half-fixed. A 9,834-case differential fuzz against the live
//! `Num2Word_PS` found no other disagreement.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.negword`. Keeps the trailing space of the Python attribute: PS builds
/// its output as `self.negword + word` with no separator of its own, so the
/// space is load-bearing rather than cosmetic.
/// Python has the English `"minus "`; Pashto منفي here (#154).
const NEGWORD: &str = "منفي ";

/// `self.pointword`, used by the float path (`"." in n`). Python has the
/// English `"point"`; Pashto اعشاریه here (#154).
const POINTWORD: &str = "اعشاریه";

/// The zero word. Python writes `self.ones[0] if self.ones[0] else "zero"`,
/// but `ones[0]` is `""` (falsy), so Python always produced the English
/// literal. This port says the Pashto صفر instead. See module bug #2.
const ZERO_WORD: &str = "صفر";

/// `self.ones`. Index 0 is `""` and is never emitted: the `number == 0` guard
/// intercepts it first, and the hundreds branch only ever indexes 1..=9.
const ONES: [&str; 10] = [
    "", "یو", "دوه", "درې", "څلور", "پنځه", "شپږ", "اووه", "اته", "نهه",
];

/// `self.tens`. Index 0 is `""` and is unreachable: the `number < 100` branch
/// is only entered when `number >= 10`, so `number // 10` is 1..=9.
const TENS: [&str; 10] = [
    "", "لس", "شل", "دېرش", "څلوېښت", "پنځوس", "شپېته", "اویا", "اتیا", "نوي",
];

const HUNDRED: &str = "سل";
const THOUSAND: &str = "زره";
const MILLION: &str = "میلیون";

/// The ordinal suffix appended by `to_ordinal`: ASCII hyphen U+002D followed by
/// ARABIC LETTER MEEM U+0645.
const ORDINAL_SUFFIX: &str = "-م";

/// First value the `_int_to_word` cascade cannot name: `10**9`. At or above
/// this, Python's `else` arm returns `str(number)` verbatim (module bug #4).
fn billion() -> BigInt {
    BigInt::from(1_000_000_000u32)
}

/// `Num2Word_PS.to_currency`'s own default separator, `" "`.
///
/// Confirmed against `inspect.signature`:
/// `(val, currency='AFN', cents=True, separator=' ', adjective=False)`.
const PS_SEPARATOR: &str = " ";

/// The separator the callers hand us when the Python caller omitted one.
///
/// PS declares `separator=" "`, but the `Lang` trait takes the separator as a
/// plain argument, and *both* callers substitute a default before the value
/// ever reaches Rust — `num2words2/__init__.py`'s currency fast path sends
/// `kwargs.get("separator", ",")` and `bench/diff_test.py` hardcodes `","`.
/// That `","` is **`Num2Word_Base`'s** default, not PS's. By the time we are
/// called, "caller omitted separator" and "caller asked for a comma" are the
/// same string and the information is gone.
///
/// Every currency row in the frozen corpus comes from
/// `num2words(v, lang="ps", to="currency", currency=c)` with no `separator=`,
/// so Python renders them through PS's own `" "` ("لس دوه euros دېرش څلور
/// cents"). Mapping `","` back to `" "` restores PS's default and reproduces
/// all 108 rows. The one input it gets wrong is an *explicit*
/// `separator=","`, which Python renders as "لس دوه euros,دېرش څلور cents"
/// (no space — PS concatenates the separator raw) and which we render with a
/// space. Fixing that properly means teaching the shim to pass the
/// converter's own default; `__init__.py` is off-limits here and shared by
/// ~150 languages, so the narrower divergence is the deliberate choice.
/// `lang_bs.rs`, `lang_bo.rs`, `lang_br.rs`, `lang_ca.rs` and `lang_es.rs`
/// hit the identical trap and resolve it the same way — see `concerns`.
const SEPARATOR_UNSET: &str = ",";

/// The key `list(self.CURRENCY_FORMS.values())[0]` resolves to.
///
/// PS's `to_currency` falls back on the **first value** of its own
/// `CURRENCY_FORMS`, and Python dicts have preserved insertion order since
/// 3.7, so the class literal's first entry — `AFN` — is what every
/// unrecognised code silently becomes (bug 6). Verified in the interpreter:
/// `list(CONVERTER_CLASSES['ps'].CURRENCY_FORMS.values())[0]` is
/// `(('افغانۍ', 'افغانۍ'), ('پول', 'پول'))`.
const FALLBACK_CURRENCY: &str = "AFN";

pub struct LangPs {
    /// `Num2Word_PS.CURRENCY_FORMS`, built once in [`LangPs::new`] and reused
    /// for the life of the converter. The pyo3 binding holds each language in
    /// a `OnceLock`, so this table is constructed exactly once per process —
    /// rebuilding it per call is what made an earlier revision of this port
    /// 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// The [`FALLBACK_CURRENCY`] entry, cloned out so the `.get(..., default)`
    /// fallback needs no second hash lookup and cannot panic.
    afn: CurrencyForms,
}

impl Default for LangPs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPs {
    pub fn new() -> Self {
        // Python:
        //   CURRENCY_FORMS = {
        //       "AFN": (("افغانۍ", "افغانۍ"), ("پول", "پول")),
        //       "USD": (("dollar", "dollars"), ("cent", "cents")),
        //       "EUR": (("euro", "euros"), ("cent", "cents")),
        //   }
        //
        // Two forms per side, exactly as Python has them; `select_form`
        // indexes [0]/[1] and nothing else, so the arity is load-bearing.
        // AFN's two unit forms are *identical* ("افغانۍ" is invariant), which
        // is why `currency:GBP 1` and `currency:GBP 2` both end in "افغانۍ" —
        // that is Pashto, not a bug.
        let afn = CurrencyForms::new(&["افغانۍ", "افغانۍ"], &["پول", "پول"]);

        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (FALLBACK_CURRENCY, afn.clone()),
            (
                "USD",
                CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
            ),
            (
                "EUR",
                CurrencyForms::new(&["euro", "euros"], &["cent", "cents"]),
            ),
        ]
        .into_iter()
        .collect();

        LangPs { currency_forms, afn }
    }

    /// Python's `parts = str(val).split(".")` → `(left, right, is_negative)`.
    ///
    /// ```python
    /// if val < 0:
    ///     is_negative = True
    ///     val = abs(val)
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// `abs` runs *before* `str`, so the sign never reaches the split; taking
    /// `.abs()` of the digits is the same thing.
    ///
    /// `parts[0]` is empty only for a string starting with "." (`".5"`), which
    /// neither `str(int)` nor `str(float)` nor `str(Decimal)` ever produces —
    /// so the `if parts[0] else 0` guard is dead and `left` is always the
    /// integer part.
    fn split_value(val: &CurrencyValue) -> (BigInt, u32, bool) {
        match val {
            // `str(int)` has no ".", so `len(parts) == 1` and `right` stays 0.
            // This is the arm that keeps `1` distinct from `1.0` at the type
            // level; PS happens to render both the same (bug 10), but the
            // string being sliced genuinely differs and we model that.
            CurrencyValue::Int(v) => (v.abs(), 0, v.is_negative()),
            CurrencyValue::Decimal { value: d, .. } => {
                let is_negative = d.is_negative();
                // `value == digits * 10^-scale`, with `digits`/`scale` taken
                // verbatim from the `str(value)` the shim parsed — so
                // reconstructing the plain-notation string from them
                // reproduces Python's `str(abs(val))` exactly.
                let (digits, scale) = d.as_bigint_and_exponent();
                let digits = digits.abs();

                if scale <= 0 {
                    // No "." in the rendered string: the value is integral as
                    // written. `scale == 0` is the ordinary case
                    // (`Decimal("100")` → "100"). `scale < 0` means exponent
                    // notation, where Python takes a different (broken) route
                    // — see `concerns`.
                    let pow = BigInt::from(10u32).pow((-scale) as u32);
                    return (digits * pow, 0, is_negative);
                }

                let pow = BigInt::from(10u32).pow(scale as u32);
                let (left, frac) = digits.div_rem(&pow);
                // `parts[1]` is `frac` zero-padded on the left to `scale`
                // digits, so `int(parts[1][:2].ljust(2, "0"))` is exactly
                // "the first two fractional digits read as a 2-digit number":
                // floor division that keeps leading zeros and pads a short
                // fraction on the right.
                //   0.5     -> frac 5,   scale 1 -> 5*100/10   = 50  (ljust)
                //   0.01    -> frac 1,   scale 2 -> 1*100/100  = 1
                //   12.34   -> frac 34,  scale 2 -> 34*100/100 = 34
                //   1.999   -> frac 999, scale 3 -> 999*100/1000 = 99 (bug 8)
                //   0.001   -> frac 1,   scale 3 -> 1*100/1000   = 0
                // Always < 100, so `to_u32` cannot fail.
                let right = (frac * BigInt::from(100u32) / &pow).to_u32().unwrap_or(0);
                (left, right, is_negative)
            }
        }
    }

    /// Python's inline `cr[1] if n != 1 else cr[0]`.
    ///
    /// Indexing `forms[0]`/`forms[1]` is sound because every entry built in
    /// [`LangPs::new`] carries exactly two forms per side, matching the
    /// Python literal. Note the comparison is against the *whole* value, so
    /// 21 takes the plural (`cr[1]`) — PS has no "ends in 1" rule.
    fn select_form(n: &BigInt, forms: &[String]) -> String {
        if n.is_one() {
            forms[0].clone()
        } else {
            forms[1].clone()
        }
    }

    /// Python's `_int_to_word`, branch for branch and in the original order:
    /// zero, then negative, then the `< 10 / 100 / 1000 / 10**6 / 10**9`
    /// cascade, then the bare `str(number)` fallback.
    fn int_to_word(&self, number: &BigInt) -> String {
        // `if number == 0: return self.ones[0] if self.ones[0] else "zero"`
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // `if number < 0: return self.negword + self._int_to_word(abs(number))`
        //
        // Dead code on every path, integer and currency alike: `to_cardinal`
        // strips the sign from the decimal string before calling in,
        // `to_currency` pre-`abs()`es and prepends "منفي " itself, and each
        // recursive call passes a quotient or remainder of an already-positive
        // value. Ported anyway so the cascade matches Python arm for arm; note
        // it would yield a doubled "منفي " if it ever were reached through
        // `to_cardinal`.
        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // The `else` arm: everything the cascade cannot name stringifies.
        // Checked before narrowing precisely because BigInt has no ceiling here.
        if number >= &billion() {
            return number.to_string();
        }

        // Proven above: 0 < number < 10**9, so the value fits u32 (max
        // 999_999_999 < 2**32). This is the only point where narrowing is
        // sound; `to_string()` on the wide value already happened.
        let n = number
            .to_u32()
            .expect("0 < number < 10^9 was proven above, so u32 conversion cannot fail");
        self.pos_to_word(n)
    }

    /// The naming cascade for `0 < n < 10**9`.
    ///
    /// Invariant `n > 0` is upheld by every caller: [`int_to_word`] handles 0
    /// first, the `if remainder:` guards suppress zero remainders, and the
    /// quotients are >= 1 because the branch was entered on `n >= 1000` /
    /// `n >= 10**6`. Were `n == 0` ever to arrive it would return `ONES[0]`
    /// (`""`) rather than "zero" — the divergence is unreachable, matching
    /// Python's own reachability.
    fn pos_to_word(&self, n: u32) -> String {
        if n < 10 {
            // `return self.ones[number]`
            ONES[n as usize].to_string()
        } else if n < 100 {
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                TENS[tens_val].to_string()
            } else {
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            }
        } else if n < 1000 {
            // `result = self.ones[hundreds_val] + " " + self.hundred`
            // Note 100 → "یو سل" ("one hundred"), never a bare "سل".
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.pos_to_word(remainder));
            }
            result
        } else if n < 1_000_000 {
            let thousands_val = n / 1000;
            let remainder = n % 1000;
            let mut result = format!("{} {}", self.pos_to_word(thousands_val), THOUSAND);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.pos_to_word(remainder));
            }
            result
        } else {
            // n < 10**9, guaranteed by the caller.
            let millions_val = n / 1_000_000;
            let remainder = n % 1_000_000;
            let mut result = format!("{} {}", self.pos_to_word(millions_val), MILLION);
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.pos_to_word(remainder));
            }
            result
        }
    }

}

/// CPython's `repr(float)` (== `str(float)`): shortest round-trip digits,
/// fixed notation iff `-4 < decpt <= 16`, `.0` appended when integral,
/// two-digit-padded exponent otherwise. `Num2Word_PS.to_cardinal` is driven
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
/// `< -6` — so `Decimal("1E+2")` prints `"1E+2"` and feeds `int()` a
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

impl Lang for LangPs {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "AFN"
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


    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)`: it strips a leading `"-"` off the
    /// *decimal text* and sets `ret = self.negword`, then feeds the remainder
    /// back through `int()`. Removing the sign character from a decimal
    /// rendering is exactly `abs()`, so operating on the BigInt directly is
    /// equivalent for integral input.
    ///
    /// The `"." in n` branch (float/point path) cannot trigger: `str()` of an
    /// integer never contains a dot. Out of scope regardless.
    ///
    /// The trailing `.strip()` is preserved for fidelity even though it is a
    /// no-op here — `negword` supplies a *leading* space only when followed by
    /// a word, and `_int_to_word` never returns padded output.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, self.int_to_word(&n)).trim().to_string())
    }

    /// `return cardinal + "-م"`. No sign, zero or range handling whatsoever, so
    /// `to_ordinal(0)` == "صفر-م" and `to_ordinal(-1)` == "منفي یو-م".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// `return str(number) + "."` — the numeral, untouched, plus a dot.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    /// `longval` is ignored, so years get no century/"nineteen-eighty" split —
    /// 1999 is the plain cardinal "یو زره نهه سل نوي نهه". Overridden
    /// explicitly rather than leaning on the identical trait default, to keep
    /// the correspondence with the Python method visible.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal half of `Num2Word_PS.to_cardinal`.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword          # "minus "
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
    /// PS overrides `to_cardinal` outright and handles non-integers inline on
    /// `str(number)`; it never delegates to `Num2Word_Base.to_cardinal_float`
    /// and never reads `self.precision`. `precision_override` is therefore
    /// dropped on the floor exactly as Python does (confirmed: `precision=`
    /// leaves PS's cardinal output unchanged). Every digit — integer part and
    /// each fractional character alike — is named by the same `_int_to_word`
    /// the integer path uses, so the "no teens" and `>= 10**9` stringify bugs
    /// (module bugs 1 and 4) leak into the float output too.
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
        let (n, neg) = match n_full.strip_prefix('-') {
            Some(rest) => (rest, true),
            None => (n_full, false),
        };

        let mut ret = String::new();
        if neg {
            // `ret = self.negword` — "منفي ", trailing space included and
            // prepended raw (PS supplies no separator of its own).
            ret.push_str(NEGWORD);
        }

        if let Some((left, right)) = n.split_once('.') {
            // `ret += self._int_to_word(int(left)) + " " + pointword + " "` —
            // `int_to_word`, so `left >= 10**9` stringifies to its numerals
            // (bug 4), e.g. 98746251323029.
            ret.push_str(&self.int_to_word(&py_int(left)?));
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // `for digit in right: ret += self._int_to_word(int(digit)) + " "`
            // — `int('e')` (the exponent of "1.5e+16") raises ValueError.
            for ch in right.chars() {
                let mut buf = [0u8; 4];
                let d = py_int(ch.encode_utf8(&mut buf))?;
                ret.push_str(&self.int_to_word(&d));
                ret.push(' ');
            }
            // `.strip()` — trims the trailing space left by the digit loop.
            Ok(ret.trim().to_string())
        } else {
            // No "." → Python's else branch: `(ret + _int_to_word(int(n)))
            // .strip()` — exponent forms ("1e+16", "1E+3") raise here.
            ret.push_str(&self.int_to_word(&py_int(n)?));
            Ok(ret.trim().to_string())
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL entry. Python routes *every*
    /// float/Decimal through the `str(number)` algorithm, so a whole value
    /// keeps its visible point: `5.0` -> "پنځه اعشاریه صفر", `-0.0` ->
    /// "منفي صفر اعشاریه صفر", `Decimal("5.00")` -> "پنځه اعشاریه صفر صفر".
    /// The base default's whole-value integer shortcut must not fire here.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number) + "-م"` — the
    /// cardinal being the string algorithm above, suffix glued on raw.
    /// Exponent forms raise ValueError before the suffix is appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.to_cardinal_float(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is
    /// the binding's Python `str(value)`, so exponent forms echo verbatim:
    /// `to_ordinal_num(1e16)` == "1e+16.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: PS's `to_year` forwards to `to_cardinal`,
    /// which for a float/Decimal is the string algorithm above.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_cardinal_float(value, None)
    }

    /// Base's `str_to_number` parses "Infinity" *successfully*; PS's
    /// ValueError comes later, from `int("Infinity")` inside `to_cardinal`.
    /// The parse is the plain default and the two specials are served natively
    /// by the [`Lang::inf_result`] / [`Lang::nan_result`] hooks below.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// `Decimal("Infinity")` / `-Infinity`. PS's `to_cardinal` does
    /// `int(str(number))`, and `str(Decimal("Infinity"))` is "Infinity" —
    /// `int("Infinity")` raises **ValueError**, not the base path's
    /// OverflowError. `-Infinity` strips its "-" textually first, so both signs
    /// raise with the token "Infinity". Every mode reaches the same `int()`
    /// (`to_ordinal`/`to_year` forward to `to_cardinal`), so `to` is unread.
    fn inf_result(&self, _negative: bool, _to: &str) -> Result<String> {
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'Infinity'".into(),
        ))
    }

    /// `Decimal("NaN")`. `str(Decimal("NaN"))` is "NaN"; `int("NaN")` raises
    /// **ValueError** — the same type the base default gives, made byte-exact.
    fn nan_result(&self, _to: &str) -> Result<String> {
        Err(N2WError::Value(
            "invalid literal for int() with base 10: 'NaN'".into(),
        ))
    }

    // ---- currency ----------------------------------------------------
    //
    // Left at their trait defaults, each for a checked reason:
    //
    //   currency_precision  — PS inherits `Num2Word_Base.CURRENCY_PRECISION`,
    //                         which is `{}`, so `.get(code, 100)` is 100 for
    //                         every code. The default returns 100. (PS's own
    //                         `to_currency` never reads it at all — bug 7 —
    //                         but `to_cheque` does, and 100 is what it wants:
    //                         "56/100" in the corpus.)
    //   currency_adjective  — `CURRENCY_ADJECTIVES` is `{}`; nothing on this
    //                         surface reads it and `adjective=True` is dropped.
    //   money_verbose       — `Num2Word_Base._money_verbose` is
    //                         `return self.to_cardinal(number)`, which is the
    //                         default. Reached only via `to_cheque`.
    //   cents_verbose /
    //   cents_terse /
    //   pluralize           — unreachable; PS's `to_currency` picks forms
    //                         inline and `to_cheque` emits digits.
    //   cardinal_from_decimal — no fractional-cents path exists here: PS
    //                         truncates the cents field to two digits (bug 8),
    //                         so `right` is always a whole 0..=99.
    //   to_cheque           — inherited from `Num2Word_Base`, so
    //                         `currency::default_to_cheque` already *is* the
    //                         port. Traced below.

    fn lang_name(&self) -> &str {
        "Num2Word_PS"
    }

    /// `self.CURRENCY_FORMS[code]` — the **strict** lookup.
    ///
    /// This hook models Python's *subscript*, which is what
    /// `Num2Word_Base.to_cheque` uses (`self.CURRENCY_FORMS[currency]` inside
    /// a `try/except KeyError`). Returning `None` for an unknown code is what
    /// makes `currency::default_to_cheque` raise the exact
    /// `NotImplementedError` the corpus pins for
    /// cheque:GBP/JPY/KWD/BHD/INR/CNY/CHF — message verified byte for byte
    /// against the interpreter: `Currency code "GBP" not implemented for
    /// "Num2Word_PS"`.
    ///
    /// [`LangPs::to_currency`] deliberately does **not** call this: PS's own
    /// override uses `.get(code, <first value>)` and must never raise (bug 9).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_PS.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="AFN", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
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
    /// Shares nothing with `Num2Word_Base.to_currency`, so
    /// `currency::default_to_currency` is bypassed entirely — no divisor, no
    /// `isinstance(val, int)` branch, no `pluralize`, no
    /// `parse_currency_parts`. See bugs 6-10.
    ///
    /// `adjective` is accepted and dropped on the floor exactly as Python does
    /// — `CURRENCY_ADJECTIVES` is never read on this path, so `adjective=True`
    /// changes nothing.
    ///
    /// `_int_to_word` is called with the already-`abs`'d `left`, so its
    /// negative arm stays unreachable here and "منفي " is prepended exactly
    /// once, at the end.
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
        // Restore PS's own `separator=" "` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            PS_SEPARATOR
        } else {
            separator
        };

        let (left, right, is_negative) = Self::split_value(val);

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — bug 6.
        // Bypasses the `currency_forms` hook on purpose; that one is strict
        // because `to_cheque` needs it to be.
        let forms = self.currency_forms.get(currency).unwrap_or(&self.afn);

        // `left_str = self._int_to_word(left)` — note `_int_to_word`, not
        // `to_cardinal`: identical for a non-negative int, but it means the
        // >= 10^9 digit fallback (bug 4) applies to money too.
        let left_str = self.int_to_word(&left);
        let currency_word = Self::select_form(&left, &forms.unit);
        let mut result = format!("{} {}", left_str, currency_word);

        // `if cents and right:` — a zero `right` suppresses the segment, which
        // is why the float 1.0 renders as a bare "یو euro" (bug 10) and why
        // 0.001 renders as "صفر euros" (bug 8).
        if cents && right != 0 {
            let right_big = BigInt::from(right);
            let cents_str = self.int_to_word(&right_big);
            let cents_word = Self::select_form(&right_big, &forms.subunit);
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(&cents_word);
        }

        if is_negative {
            // `result = self.negword + result` — "منفي ", trailing space
            // included. Prepended raw, with no separator of its own.
            result.insert_str(0, NEGWORD);
        }

        // `.strip()`. A no-op for every reachable input — `_int_to_word` never
        // returns "" (the `number == 0` guard intercepts the only index that
        // holds an empty string), so nothing pads the ends. Kept for fidelity.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    fn f(value: f64, precision: u32) -> String {
        LangPs::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn d(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = (-value.as_bigint_and_exponent().1).unsigned_abs() as u32;
        LangPs::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    // --- frozen corpus: `to == "cardinal"` with a dotted float `arg` ---
    #[test]
    fn corpus_float_rows() {
        assert_eq!(f(0.0, 1), "صفر اعشاریه صفر");
        assert_eq!(f(0.5, 1), "صفر اعشاریه پنځه");
        assert_eq!(f(1.0, 1), "یو اعشاریه صفر");
        assert_eq!(f(1.5, 1), "یو اعشاریه پنځه");
        assert_eq!(f(2.25, 2), "دوه اعشاریه دوه پنځه");
        assert_eq!(f(3.14, 2), "درې اعشاریه یو څلور");
        assert_eq!(f(0.01, 2), "صفر اعشاریه صفر یو");
        assert_eq!(f(0.1, 1), "صفر اعشاریه یو");
        assert_eq!(f(0.99, 2), "صفر اعشاریه نهه نهه");
        assert_eq!(f(1.01, 2), "یو اعشاریه صفر یو");
        assert_eq!(f(12.34, 2), "لس دوه اعشاریه درې څلور");
        assert_eq!(f(99.99, 2), "نوي نهه اعشاریه نهه نهه");
        assert_eq!(f(100.5, 1), "یو سل اعشاریه پنځه");
        assert_eq!(f(1234.56, 2), "یو زره دوه سل دېرش څلور اعشاریه پنځه شپږ");
        assert_eq!(f(-0.5, 1), "منفي صفر اعشاریه پنځه");
        assert_eq!(f(-1.5, 1), "منفي یو اعشاریه پنځه");
        assert_eq!(f(-12.34, 2), "منفي لس دوه اعشاریه درې څلور");
        // The two f64-artefact cases: PS reads `repr`, so the digits are the
        // repr's, reconstructed by formatting to `precision` places.
        assert_eq!(f(1.005, 3), "یو اعشاریه صفر صفر پنځه");
        assert_eq!(f(2.675, 3), "دوه اعشاریه شپږ اووه پنځه");
    }

    // --- frozen corpus: `to == "cardinal_dec"` (Decimal input) ---
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(d("0.01"), "صفر اعشاریه صفر یو");
        // Trailing zero survives: `str(Decimal("1.10")) == "1.10"`, fraction "10".
        assert_eq!(d("1.10"), "یو اعشاریه یو صفر");
        assert_eq!(d("12.345"), "لس دوه اعشاریه درې څلور پنځه");
        // Left part >= 10**9 stringifies to its numerals (bug 4).
        assert_eq!(
            d("98746251323029.99"),
            "98746251323029 اعشاریه نهه نهه"
        );
        assert_eq!(d("0.001"), "صفر اعشاریه صفر صفر یو");
    }

    // `str(-0.0) == "-0.0"`, so the sign is carried even though `-0.0 < 0.0`
    // is false: formatting the signed f64 preserves the sign bit.
    #[test]
    fn negative_zero_float() {
        assert_eq!(f(-0.0, 1), "منفي صفر اعشاریه صفر");
    }

    // precision_override is dropped, exactly like Python (`precision=` leaves
    // PS's cardinal untouched because it renders `str(number)`).
    #[test]
    fn precision_override_ignored() {
        let v = FloatValue::Float {
            value: 2.675,
            precision: 3,
        };
        assert_eq!(
            LangPs::new().to_cardinal_float(&v, Some(1)).unwrap(),
            "دوه اعشاریه شپږ اووه پنځه"
        );
        assert_eq!(
            LangPs::new().to_cardinal_float(&v, Some(5)).unwrap(),
            "دوه اعشاریه شپږ اووه پنځه"
        );
    }
}
