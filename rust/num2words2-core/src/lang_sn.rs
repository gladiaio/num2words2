//! Port of `lang_SN.py` (Shona / chiShona).
//!
//! Registry check: `__init__.py` maps `"sn"` to `lang_SN.Num2Word_SN()`, so
//! this file ports `Num2Word_SN` — the only class the module defines.
//!
//! Shape: **self-contained**. `Num2Word_SN` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. That matters:
//! `Num2Word_Base.__init__` only builds `self.cards` / sets `self.MAXVAL`
//! when one of those three attributes exists, so a live `Num2Word_SN` has
//! **neither `cards` nor `MAXVAL`** (verified against the interpreter). There
//! is therefore no overflow ceiling at all — `cards`/`maxval`/`merge` stay at
//! their trait defaults here and are never consulted.
//!
//! `to_cardinal`, `to_ordinal`, `to_ordinal_num` and `to_year` are all
//! overridden in Python; nothing in scope falls through to `Num2Word_Base`.
//! `setup()` overrides `negword` to "minus" (no trailing space) and
//! `pointword` to "poindi" — and note SN's own code concatenates `negword`
//! directly rather than going through `Num2Word_Base`'s `negword.strip()`.
//!
//! # Structure of `_int_to_sn_word`
//!
//! A hand-written recursive cascade over magnitude bands — 0, negatives,
//! <=10, teens, tens, hundreds, 1000-9999, 10000-99999, 100000-999999,
//! millions, billions, then an open-ended trillions tail. The trillions arm
//! has no upper bound and recurses on `n / 10**12`, so arbitrarily large
//! BigInt values keep working (10**24 -> "tiriyoni tiriyoni", 10**27 ->
//! "tiriyoni tiriyoni churu"). Nothing here can raise for integer input: every
//! dict lookup is guarded by the band check that precedes it, so the port is
//! infallible and returns `String` rather than `Result`.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the interpreter and are preserved verbatim:
//!
//! 1. **`to_ordinal` mangles words by blind prefix-stripping.** The chain
//!    strips a 2-character prefix from cardinals beginning "mazana" or
//!    "zvuru" before prepending "we". Stripping "ma" from "mazana" is at
//!    least defensible; stripping "zv" from "zvuru" is not, and yields the
//!    non-word "uru": `to_ordinal(2000)` == "weuru zviviri",
//!    `to_ordinal(100000)` == "weuru zana". Both are in the corpus.
//! 2. **`to_ordinal` accepts negatives and zero** rather than raising. It
//!    never calls `verify_ordinal`, so `to_ordinal(-1)` == "weminus motsi"
//!    ("we" glued onto the negword) and `to_ordinal(0)` == "wezero".
//! 3. **The "ne<unit>" connector is inconsistent between bands.** The 1-9
//!    remainder of a thousands/millions value uses the post-"ne" form
//!    (`ones_after_ne[1]` = "imwe") only when the leading count is exactly 1
//!    or the band is 10000-99999; the 100000-999999 and >=1000 (count>=2)
//!    arms recurse into `_int_to_sn_word` instead and so get the plain form
//!    (`ones[1]` = "motsi"). Hence 10001 -> "zvuru gumi neimwe" but
//!    100001 -> "zvuru zana nemotsi", and 1001 -> "churu neimwe" but
//!    2001 -> "zvuru zviviri nemotsi". Not a transcription slip — the two
//!    spellings come from genuinely different code paths.
//! 4. **`ones_after_ne[6]` is "nhatu", not "tanhatu"** (`ones[6]`). So
//!    16 -> "gumi nenhatu" while 6 -> "tanhatu". Kept as written.
//! 5. **The billions arm lacks the `remainder < 10` special case** that the
//!    thousands and millions arms have, so 1000000001 -> "bhiriyoni nemotsi".
//! 6. **`ten_thousands` / `hundred_thousands` are misnomers**: both bands do
//!    `divmod(number, 1000)`, so the variables actually hold thousands. The
//!    behaviour is right; only the Python names mislead.
//! 7. **`_get_thousands_form`'s `self.ones[number]` fallback is dead code.**
//!    It is only ever called with `2 <= thousands <= 9`, every one of which is
//!    a key of `thousands_forms`. Modelled below but unreachable.
//! 8. **`to_cardinal`'s `except Exception` fallback is unreachable** for
//!    integer input: `_int_to_sn_word` cannot raise on an int, so the bare
//!    `return self._int_to_sn_word(int(number))` retry never fires.
//!
//! # The currency surface
//!
//! `Num2Word_SN` overrides `to_currency` **wholesale** — nothing routes through
//! `Num2Word_Base.to_currency`, so `pluralize`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse`, `CURRENCY_PRECISION` and
//! `CURRENCY_ADJECTIVES` are all unreachable from it and stay at their trait
//! defaults here.
//!
//! `to_cheque` is *not* overridden, so `Num2Word_Base.to_cheque` runs and the
//! trait's `default_to_cheque` is exactly right: it needs only
//! `currency_forms()`, `lang_name()` and `money_verbose()` — whose default
//! (`to_cardinal`) is what base's `_money_verbose` does.
//!
//! Verified against the live interpreter: `Num2Word_SN` declares its own
//! class-body `CURRENCY_FORMS` (USD/ZWL/ZAR) and subclasses `Num2Word_Base`
//! directly, **not** `Num2Word_EUR` — so the `lang_EUR.py` in-place mutation
//! trap (`Num2Word_EN.__init__` rewriting the shared EUR dict) does not apply.
//! SN sees three codes and nothing else. `CURRENCY_PRECISION` is `{}`, so every
//! code has divisor 100 and SN has no 3-decimal (KWD/BHD) or 0-decimal (JPY)
//! behaviour whatsoever — those codes simply raise.
//!
//! # Faithfully reproduced Python quirks — currency
//!
//! 9. **`to_currency` raises its own message, `to_cheque` raises Base's.**
//!    SN's override says `Currency {code} not implemented for Shona`; the
//!    inherited `to_cheque` still says
//!    `Currency code "{code}" not implemented for "Num2Word_SN"`. Both are
//!    verified against the interpreter and both are `NotImplementedError`.
//! 10. **SN never prints "zero cents".** The guard is `if cents and
//!     decimal_part:` and `0` is falsy, so the cents tail vanishes for whole
//!     values. This is why `1` and `1.0` both render "dhora rimwe" — SN is
//!     immune to the int-vs-float split that `Num2Word_Base.to_currency`
//!     depends on, and `has_decimal` is never consulted.
//! 11. **`to_currency` takes `integer_part` from *float* arithmetic but the
//!     fractional-cents arm takes it from *exact decimal* arithmetic.**
//!     `_split_currency` does `int(value)` on the float in one arm and
//!     `int(Decimal(str(value)))` in the other. They disagree above 2**53:
//!     `int(1e23)` is 99999999999999991611392 while `Decimal("1e+23")` is
//!     10**23, and Python prints the former. See [`Lang::to_currency`].
//! 12. **The separator is glued to the subunit with no space** (`"ne"` +
//!     `"sendi"` -> `"nesendi"`) while the final `" ".join` adds one in front,
//!     so a custom `separator=" uye "` double-spaces:
//!     `"madhora gumi nepiri  uye sendi makumi matatu nechina"`.
//! 13. **`to_currency` has no `adjective` parameter.** Its Python signature is
//!     `(n, currency="USD", cents=True, separator="ne")`, so `adjective=` is a
//!     `TypeError` there. SN is one of 18 converters shaped this way; the trait
//!     passes the flag regardless, and it is ignored here. See `concerns`.
//! 14. **`major_plural` / `minor_plural` are unpacked and never read.** SN
//!     builds its plurals by gluing a literal `"ma"` onto the *singular*
//!     (`"ma" + "dhora"`), which is why ZAR's `("randi", "marandi")` plural form
//!     never appears even though 2 randi renders "marandi maviri".
//!
//! # Cross-call mutable state
//!
//! None. `Num2Word_SN` stashes no flag in one method for another to consume
//! (no `_pending_ordinal`-style handshake), so the stateless Rust path is a
//! faithful substitute and the Python dispatcher needs no special casing.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `setup()` overrides this to "minus" — no trailing space, unlike the
/// `Num2Word_Base` default of "(-) ".
const NEGWORD: &str = "minus";

/// `self.ones`, keys 0..=10.
const ONES: [&str; 11] = [
    "zero",
    "motsi",
    "piri",
    "tatu",
    "china",
    "shanu",
    "tanhatu",
    "nomwe",
    "sere",
    "pfumbamwe",
    "gumi",
];

/// `self.ones_after_ne`, keys 0..=10 — the forms used after the "ne" (and)
/// connector. Differs from `ONES` at 1 ("imwe" vs "motsi") and at 6 ("nhatu"
/// vs "tanhatu"); every other entry is identical.
const ONES_AFTER_NE: [&str; 11] = [
    "zero",
    "imwe",
    "piri",
    "tatu",
    "china",
    "shanu",
    "nhatu",
    "nomwe",
    "sere",
    "pfumbamwe",
    "gumi",
];

/// `self.tens_forms`, keys 2..=9. Indices 0 and 1 are absent in Python; the
/// call sites guard the range, so the empty slots are never read.
const TENS_FORMS: [&str; 10] = [
    "", // absent in Python
    "", // absent in Python
    "maviri",
    "matatu",
    "mana",
    "mashanu",
    "matanhatu",
    "manomwe",
    "masere",
    "mapfumbamwe",
];

/// `self.ordinals`, keys 1..=10. Index 0 is absent in Python — `to_ordinal(0)`
/// therefore misses the table and falls through to the cardinal path.
const ORDINALS: [&str; 11] = [
    "", // absent in Python
    "wekutanga",
    "wechipiri",
    "wechitatu",
    "wechina",
    "wechishanu",
    "wechitanhatu",
    "wechinomwe",
    "wechisere",
    "wechipfumbamwe",
    "wegumi",
];

/// `_get_thousands_form`'s local `thousands_forms`, keys 2..=9.
const THOUSANDS_FORMS: [&str; 10] = [
    "", // absent in Python
    "", // absent in Python
    "viri",
    "tatu",
    "na",
    "shanu",
    "tanhatu",
    "nomwe",
    "sere",
    "pfumbamwe",
];

/// Port of `Num2Word_SN._get_thousands_form`.
///
/// Python: `thousands_forms.get(number, self.ones[number])`. The fallback is
/// dead code (see module quirk 7) but is modelled for fidelity; `number` is
/// always 2..=9 at the single call site.
fn get_thousands_form(n: usize) -> &'static str {
    match THOUSANDS_FORMS.get(n) {
        Some(w) if !w.is_empty() => w,
        // Unreachable: `.get(number, self.ones[number])` fallback.
        _ => ONES[n],
    }
}

/// Python's `str[2:]` — drop the first two *characters*.
///
/// The Shona tables are pure ASCII so byte slicing would agree today, but
/// Python slices by character and the port should not depend on that
/// coincidence.
fn drop2(s: &str) -> String {
    s.chars().skip(2).collect()
}

fn big(n: u64) -> BigInt {
    BigInt::from(n)
}

/// Port of `Num2Word_SN._int_to_sn_word`.
///
/// Infallible: every dict lookup is range-guarded by the band test above it,
/// so no `KeyError` is reachable for integer input. Recurses on the trillions
/// tail, so there is no upper bound.
fn int_to_sn_word(number: &BigInt) -> String {
    // if number == 0: return self.ones[0]
    if number.is_zero() {
        return ONES[0].to_string();
    }

    // if number < 0: return self.negword + " " + self._int_to_sn_word(-number)
    // Note: negword is used raw ("minus"), not via Num2Word_Base's .strip().
    if number.is_negative() {
        return format!("{} {}", NEGWORD, int_to_sn_word(&-number));
    }

    // From here on `number` is strictly positive, so div_rem == Python divmod.

    // if number <= 10: return self.ones[number]
    if number <= &big(10) {
        return ONES[number.to_usize().expect("0 < n <= 10")].to_string();
    }

    // Numbers 11-19: "gumi ne" + ones_after_ne[number - 10]
    if number < &big(20) {
        let idx = (number - big(10)).to_usize().expect("11 <= n < 20");
        return format!("gumi ne{}", ONES_AFTER_NE[idx]);
    }

    // Numbers 20-99
    if number < &big(100) {
        let (tens, units) = number.div_rem(&big(10));
        let tens = tens.to_usize().expect("2 <= tens <= 9");
        let units = units.to_usize().expect("0 <= units <= 9");
        if units == 0 {
            return format!("makumi {}", TENS_FORMS[tens]);
        }
        return format!("makumi {} ne{}", TENS_FORMS[tens], ONES_AFTER_NE[units]);
    }

    // Numbers 100-999
    if number < &big(1000) {
        let (hundreds, remainder) = number.div_rem(&big(100));
        let hundreds = hundreds.to_usize().expect("1 <= hundreds <= 9");
        if hundreds == 1 {
            if remainder.is_zero() {
                return "zana".to_string();
            }
            // Single-digit remainders take the post-"ne" form...
            if remainder < big(10) {
                let r = remainder.to_usize().expect("1 <= r < 10");
                return format!("zana ne{}", ONES_AFTER_NE[r]);
            }
            // ...anything larger recurses.
            return format!("zana ne{}", int_to_sn_word(&remainder));
        }
        if remainder.is_zero() {
            return format!("mazana {}", TENS_FORMS[hundreds]);
        }
        // No ones_after_ne shortcut on this arm: 201 -> "mazana maviri nemotsi".
        return format!(
            "mazana {} ne{}",
            TENS_FORMS[hundreds],
            int_to_sn_word(&remainder)
        );
    }

    // Numbers 1000-9999
    if number < &big(10000) {
        let (thousands, remainder) = number.div_rem(&big(1000));
        let thousands_u = thousands.to_usize().expect("1 <= thousands <= 9");
        if thousands_u == 1 {
            if remainder.is_zero() {
                return "churu".to_string();
            }
            if remainder < big(10) {
                let r = remainder.to_usize().expect("1 <= r < 10");
                return format!("churu ne{}", ONES_AFTER_NE[r]);
            }
            return format!("churu ne{}", int_to_sn_word(&remainder));
        }
        // thousands is 2..=9 here, so the `thousands < 10` tests in Python are
        // always true on this arm and the `else` branches are dead. Modelled
        // anyway so the shape matches the source.
        if remainder.is_zero() {
            if thousands_u < 10 {
                return format!("zvuru zvi{}", get_thousands_form(thousands_u));
            }
            return format!("zvuru {}", int_to_sn_word(&thousands));
        }
        if thousands_u < 10 {
            return format!(
                "zvuru zvi{} ne{}",
                get_thousands_form(thousands_u),
                int_to_sn_word(&remainder)
            );
        }
        return format!(
            "zvuru {} ne{}",
            int_to_sn_word(&thousands),
            int_to_sn_word(&remainder)
        );
    }

    // Numbers 10000-99999. Python names the quotient `ten_thousands` but
    // divides by 1000, so it actually holds thousands (10..=99).
    if number < &big(100000) {
        let (ten_thousands, remainder) = number.div_rem(&big(1000));
        if remainder.is_zero() {
            return format!("zvuru {}", int_to_sn_word(&ten_thousands));
        }
        // This arm *does* take the ones_after_ne shortcut: 10001 -> "neimwe".
        if remainder < big(10) {
            let r = remainder.to_usize().expect("1 <= r < 10");
            return format!("zvuru {} ne{}", int_to_sn_word(&ten_thousands), ONES_AFTER_NE[r]);
        }
        return format!(
            "zvuru {} ne{}",
            int_to_sn_word(&ten_thousands),
            int_to_sn_word(&remainder)
        );
    }

    // Numbers 100000-999999. Same misnomer; quotient is thousands (100..=999).
    if number < &big(1000000) {
        let (hundred_thousands, remainder) = number.div_rem(&big(1000));
        if remainder.is_zero() {
            return format!("zvuru {}", int_to_sn_word(&hundred_thousands));
        }
        // No ones_after_ne shortcut on this arm: 100001 -> "zvuru zana nemotsi",
        // in contrast with 10001 -> "zvuru gumi neimwe" just above.
        return format!(
            "zvuru {} ne{}",
            int_to_sn_word(&hundred_thousands),
            int_to_sn_word(&remainder)
        );
    }

    // Millions
    if number < &big(1000000000) {
        let (millions, remainder) = number.div_rem(&big(1000000));
        if millions == BigInt::from(1) {
            if remainder.is_zero() {
                return "miriyoni".to_string();
            }
            if remainder < big(10) {
                let r = remainder.to_usize().expect("1 <= r < 10");
                return format!("miriyoni ne{}", ONES_AFTER_NE[r]);
            }
            return format!("miriyoni ne{}", int_to_sn_word(&remainder));
        }
        // 2 gets the irregular "mbiri"; 3..=9 take the plain `ones` form.
        // Python's `if millions in self.ones else ...` guard is redundant
        // (3..=9 are always keys) but harmless.
        if remainder.is_zero() {
            if millions == BigInt::from(2) {
                return "miriyoni mbiri".to_string();
            }
            if millions < big(10) {
                let m = millions.to_usize().expect("3 <= millions <= 9");
                return format!("miriyoni {}", ONES[m]);
            }
            return format!("miriyoni {}", int_to_sn_word(&millions));
        }
        if millions == BigInt::from(2) {
            return format!("miriyoni mbiri ne{}", int_to_sn_word(&remainder));
        }
        if millions < big(10) {
            let m = millions.to_usize().expect("3 <= millions <= 9");
            return format!("miriyoni {} ne{}", ONES[m], int_to_sn_word(&remainder));
        }
        return format!(
            "miriyoni {} ne{}",
            int_to_sn_word(&millions),
            int_to_sn_word(&remainder)
        );
    }

    // Billions
    if number < &big(1000000000000) {
        let (billions, remainder) = number.div_rem(&big(1000000000));
        if billions == BigInt::from(1) {
            if remainder.is_zero() {
                return "bhiriyoni".to_string();
            }
            // No `remainder < 10` shortcut here, unlike the millions arm:
            // 1000000001 -> "bhiriyoni nemotsi".
            return format!("bhiriyoni ne{}", int_to_sn_word(&remainder));
        }
        let base: String = if billions == BigInt::from(2) {
            "mbiri".to_string()
        } else if billions < big(10) {
            ONES[billions.to_usize().expect("3 <= billions <= 9")].to_string()
        } else {
            int_to_sn_word(&billions)
        };
        if remainder.is_zero() {
            return format!("bhiriyoni {}", base);
        }
        return format!("bhiriyoni {} ne{}", base, int_to_sn_word(&remainder));
    }

    // Trillions — open-ended: no upper guard, and `trillions` recurses, so
    // 10**24 -> "tiriyoni tiriyoni" and 10**27 -> "tiriyoni tiriyoni churu".
    let (trillions, remainder) = number.div_rem(&big(1000000000000));
    if trillions == BigInt::from(1) {
        if remainder.is_zero() {
            return "tiriyoni".to_string();
        }
        return format!("tiriyoni ne{}", int_to_sn_word(&remainder));
    }
    let base: String = if trillions == BigInt::from(2) {
        "mbiri".to_string()
    } else if trillions < big(10) {
        ONES[trillions.to_usize().expect("3 <= trillions <= 9")].to_string()
    } else {
        int_to_sn_word(&trillions)
    };
    if remainder.is_zero() {
        return format!("tiriyoni {}", base);
    }
    format!("tiriyoni {} ne{}", base, int_to_sn_word(&remainder))
}

/// `Num2Word_SN.CURRENCY_FORMS`, transcribed from the class body.
///
/// Three codes, all with divisor 100. `Num2Word_SN` subclasses
/// `Num2Word_Base` directly and declares this dict itself, so it is *not* the
/// shared `Num2Word_EUR` table that `Num2Word_EN.__init__` mutates in place —
/// none of EN's ~24 added codes leak in here. Confirmed by dumping
/// `CONVERTER_CLASSES["sn"].CURRENCY_FORMS` from the live interpreter.
///
/// Arity is 2 on both sides throughout, matching Python's tuples. The plural
/// forms are dead weight (module quirk 14) but are part of the ported data.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // ("sendi", "masendi") is shared by all three entries in Python.
    const SENDI: [&str; 2] = ["sendi", "masendi"];
    const DHORA: [&str; 2] = ["dhora", "madhora"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("USD", CurrencyForms::new(&DHORA, &SENDI));
    m.insert("ZWL", CurrencyForms::new(&DHORA, &SENDI));
    m.insert("ZAR", CurrencyForms::new(&["randi", "marandi"], &SENDI));
    m
}

pub struct LangSn {
    /// Built once in `new()` and read-only thereafter — never reconstructed
    /// per `to_currency` call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangSn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSn {
    pub fn new() -> Self {
        LangSn {
            currency_forms: build_currency_forms(),
        }
    }
}

/// The dict key as Python would repr it in the KeyError — `KeyError: 0.5` /
/// `KeyError: Decimal('1.5')`. Only the exception *type* is corpus-checked,
/// so a close rendering of the value suffices.
fn sn_key_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, precision } => {
            if value.is_finite() {
                format!("{:.*}", *precision as usize, value)
            } else {
                format!("{}", value)
            }
        }
        FloatValue::Decimal { value, .. } => {
            format!("Decimal('{}')", crate::strnum::python_decimal_str(value))
        }
    }
}

impl Lang for LangSn {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "USD"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        "ne"
    }

    fn negword(&self) -> &str {
        // setup(): self.negword = "minus"
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "poindi"
    }

    /// Port of `Num2Word_SN.to_cardinal`, integer path only.
    ///
    /// Python wraps the body in `try/except Exception` and retries with
    /// `int(number)`; for integer input the first attempt cannot raise, so the
    /// retry is unreachable. The `isinstance(number, str)` / `float` branches
    /// are out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(int_to_sn_word(value))
    }

    /// Port of `Num2Word_SN.to_ordinal`.
    ///
    /// No `verify_ordinal` call, so negatives and zero are accepted and simply
    /// get "we" glued onto their cardinal — including onto the negword
    /// ("weminus motsi") and onto "zero" ("wezero").
    ///
    /// The prefix chain is reproduced in source order. Only two arms actually
    /// mutate the cardinal, both by chopping two characters: "mazana..." loses
    /// "ma" (defensible) and "zvuru..." loses "zv", producing the non-word
    /// "uru" (quirk 1). The "zana" test can never fire on a "mazana" string —
    /// `startswith` is anchored — so the chain order is behaviourally inert,
    /// but it is kept literal.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // if number in self.ordinals  -> keys 1..=10
        if value >= &BigInt::from(1) && value <= &big(10) {
            let i = value.to_usize().expect("1 <= n <= 10");
            return Ok(ORDINALS[i].to_string());
        }

        let cardinal = int_to_sn_word(value);

        if cardinal.starts_with("gumi") {
            Ok(format!("we{}", cardinal))
        } else if cardinal.starts_with("makumi") {
            Ok(format!("we{}", cardinal))
        } else if cardinal.starts_with("zana") {
            Ok(format!("we{}", cardinal))
        } else if cardinal.starts_with("mazana") {
            // Remove "ma" prefix.
            Ok(format!("we{}", drop2(&cardinal)))
        } else if cardinal.starts_with("churu") {
            Ok(format!("we{}", cardinal))
        } else if cardinal.starts_with("zvuru") {
            // Remove "zv" prefix -> "uru".
            Ok(format!("we{}", drop2(&cardinal)))
        } else {
            Ok(format!("we{}", cardinal))
        }
    }

    /// Port of `Num2Word_SN.to_ordinal_num`: `str(number) + "."`.
    ///
    /// Shona has no "1st"-style abbreviation, so the module just appends a
    /// period to the digits. Negatives keep their sign: -1 -> "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_SN.to_year`: years are plain cardinals, with no
    /// century splitting and no BC/AD handling. -500 -> "minus mazana mashanu".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(int_to_sn_word(value))
    }

    /// Port of the SN float/Decimal cardinal path.
    ///
    /// SN reaches the float path two genuinely different ways, so the two
    /// `FloatValue` arms are *not* symmetric:
    ///
    /// * **Float** — `Num2Word_SN.to_cardinal(float)` dispatches to the
    ///   overridden `to_cardinal_float(number)`, which is self-contained and
    ///   ignores `base.float2tuple` entirely: it takes the digits straight from
    ///   `str(number)` and looks each up in `self.ones`. So `2.675` renders the
    ///   *repr* digits "675" ("piri poindi tanhatu nomwe shanu"), **not** the
    ///   `674.999…`-rescued-to-675 artefact the base path would produce — the
    ///   `< 0.01` heuristic never runs here. Reproduced with
    ///   `format!("{:.p}", num)`, `p` = the repr-derived precision, which
    ///   equals Python `str(number)` for every finite normal-range double
    ///   (the only ones the corpus and any float literal reach).
    ///
    /// * **Decimal** — a `Decimal` is *neither* `str` nor `float`, so SN's
    ///   Python `to_cardinal` never called `to_cardinal_float` for it: the
    ///   `except` retry truncated it to `_int_to_sn_word(int(number))`, so
    ///   `Decimal("1.5")` — and the string "1.5" — was "motsi"
    ///   (gladiaio/num2words2#156). A fractional Decimal now reads like the
    ///   float, from its exact digits; a whole one keeps the integer reading
    ///   (`Decimal("5.00")` -> "shanu"). `precision_override` never reaches
    ///   SN's Python code (its `to_cardinal`/`to_cardinal_float` take no
    ///   `precision=` kwarg), so it is ignored on both arms.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            // Decimal arm: `_int_to_sn_word(int(number))` via the except-retry.
            // `with_scale(0)` divides the mantissa by 10**scale, and BigInt
            // division truncates toward zero — exactly `int(Decimal)`, sign and
            // all (`int(Decimal("-2.5")) == -2`). The sign, if any, is emitted
            // by `int_to_sn_word` itself (it prepends the raw "minus ").
            FloatValue::Decimal { value: d, precision } if !d.is_integer() => {
                // Python truncated here (#156); read it like the float arm.
                let (pre, post) = float2tuple(value);
                let sign = if d.is_negative() { format!("{} ", NEGWORD) } else { String::new() };
                let mut result = int_to_sn_word(&pre.abs());
                result.push(' ');
                result.push_str(self.pointword());
                for ch in format!("{:0>w$}", post, w = *precision as usize).chars() {
                    let d = ch.to_digit(10).expect("float2tuple yields decimal digits");
                    result.push(' ');
                    result.push_str(ONES[d as usize]);
                }
                Ok(format!("{}{}", sign, result))
            }
            FloatValue::Decimal { value, .. } => {
                let int_part = value.with_scale(0).as_bigint_and_exponent().0;
                Ok(int_to_sn_word(&int_part))
            }
            // Float arm: port of `Num2Word_SN.to_cardinal_float(number)`.
            FloatValue::Float { value, precision } => {
                // `if number < 0: sign = self.negword + " "; number = abs(number)`
                let mut sign = String::new();
                let mut num = *value;
                if num < 0.0 {
                    sign = format!("{} ", NEGWORD); // "minus "
                    num = num.abs();
                }

                // `integer_part = int(number)` — truncation toward zero; `num`
                // is already non-negative, so this is its whole part.
                let integer_part = BigInt::from_f64(num.trunc()).ok_or_else(|| {
                    // Unreachable for a finite double, which every float literal
                    // and every corpus row is.
                    N2WError::Value(format!("cannot int() {}", num))
                })?;

                // `decimal_part = str(number).split(".")[1] if "." in str(number)`
                // else "". The gate is *`"." in str(number)`*, not the
                // precision: Python's repr picks exponent form (no dot) for
                // |v| >= 1e16 and for non-zero |v| < 1e-4, and then SN's own
                // code leaves `decimal_part` empty — `1e+16` is plain
                // "tiriyoni zvuru gumi", no "poindi" tail, even though the
                // shim-derived precision (`abs(Decimal(str(v)).exponent)`) is
                // 16 there. For every dotted repr, `format!("{:.p}", num)`
                // with the repr-derived precision reproduces the string
                // (including the trailing ".0" of whole-valued floats:
                // `str(1.0) == "1.0"`, precision 1, so "motsi poindi zero").
                let abs_num = num.abs();
                let visible_point =
                    num.is_finite() && (num == 0.0 || (abs_num >= 1e-4 && abs_num < 1e16));
                let decimal_part: String = if visible_point && *precision > 0 {
                    let s = format!("{:.*}", *precision as usize, num);
                    match s.split_once('.') {
                        Some((_, frac)) => frac.to_string(),
                        None => String::new(),
                    }
                } else {
                    String::new()
                };

                let mut result = int_to_sn_word(&integer_part);

                // `if decimal_part:` — an empty digit string is falsy, so a
                // whole-valued float with `precision == 0` skips this block.
                if !decimal_part.is_empty() {
                    result.push(' ');
                    result.push_str(self.pointword()); // "poindi"; no title()
                    for ch in decimal_part.chars() {
                        // `self.ones[int(digit)]` — digits are 0..=9, always a
                        // key; `format!("{:.}")` emits only ASCII digits.
                        let d = ch
                            .to_digit(10)
                            .expect("formatted fractional part is all digits");
                        result.push(' ');
                        result.push_str(ONES[d as usize]);
                    }
                }

                // `return sign + result` (sign is "" when non-negative).
                Ok(format!("{}{}", sign, result))
            }
        }
    }

    /// Full `to_cardinal(float/Decimal)` routing. Python's gate is
    /// `isinstance(number, float)`, **not** the base default's whole-value
    /// shortcut: a whole-valued *float* still renders through
    /// `to_cardinal_float` and speaks its ".0" tail — `5.0` -> "shanu poindi
    /// zero", `-0.0` -> "zero poindi zero" (numeric `< 0`, so no negword) —
    /// while a whole Decimal reads through `_int_to_sn_word(int(n))`:
    /// `Decimal("5.00")` -> "shanu". Both arms already live in `to_cardinal_float`; this override
    /// only removes the whole-value shortcut in front of them.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // Negative zero: `neg_zero_decimal` (below) serves the demoted
        // `Decimal("-0.0")` natively, so a `Float { -0.0 }` arriving here is a
        // *real* float and takes the to_cardinal_float reading — "zero poindi
        // zero" (`-0.0 < 0` is False, so no negword).
        self.to_cardinal_float(value, precision_override)
    }

    /// `Decimal('-0.0')` — distinct from the float `-0.0`. `BigDecimal` cannot
    /// carry the sign, so the binding demotes the value to `Float { -0.0 }`
    /// (whose cardinal reads "zero poindi zero"); this hook intercepts it first
    /// and serves SN's genuine Decimal reading, which routes through
    /// `_int_to_sn_word(Decimal('-0.0'))` — and `Decimal('-0.0') == 0` is True,
    /// so no fractional dict lookup ever happens:
    ///
    /// * `to_cardinal` → `_int_to_sn_word(...)` → `"zero"`.
    /// * `to_ordinal`  → misses the 1..=10 table, `"we" + "zero"` → `"wezero"`.
    /// * `to_year`     → `_int_to_sn_word(...)` → `"zero"`.
    /// * `to_ordinal_num` == `str(number) + "."` → `"-0.0."`, which depends on
    ///   the exact decimal string; returning `None` lets the demoted
    ///   `Float { -0.0 }` reach `ordinal_num_float_entry(value, repr_str)`,
    ///   where `repr_str` is Python's `str(number)` ("-0.0") and the answer is
    ///   reconstructed byte-exactly.
    ///
    /// All verified live against the pure-Python oracle.
    fn neg_zero_decimal(&self, to: &str) -> Option<Result<String>> {
        match to {
            "cardinal" | "year" => Some(Ok("zero".into())),
            "ordinal" => Some(Ok("wezero".into())),
            // ordinal_num needs the exact "-0.0" string; defer to the float path.
            _ => None,
        }
    }


    /// `to_ordinal(float/Decimal)` — **no** try/except in Python:
    ///
    /// ```python
    /// if number in self.ordinals: return self.ordinals[number]   # hash: 5.0 hits 5
    /// cardinal = self._int_to_sn_word(number)                    # raw value!
    /// ... "we" prefix chain ...
    /// ```
    ///
    /// Integral values (float or Decimal) sail through the dict lookups by
    /// hash equality and give exactly the integer result — `5.0` ->
    /// "wechishanu", `-0.0` -> "wezero", `12345.000` -> "weuru gumi nepiri…".
    /// A fractional value misses its first dict lookup and raises
    /// **KeyError**, uncaught (`0.5`, `2.5`, `Decimal("1.5")` …).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_ordinal(&i),
            None => Err(N2WError::Key(sn_key_repr(value))),
        }
    }

    /// `to_ordinal_num(float/Decimal)` — `str(number) + "."`, no validation:
    /// "-0.0.", "1e+16.", "5.00." all pass through verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)` — `_int_to_sn_word(number)` with the raw
    /// value and no try/except: integral values equal the integer year
    /// (`5.0` -> "shanu"), fractional ones raise **KeyError** exactly like
    /// the ordinal path (`0.5`, `-1.5`, `3.25` …).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_year(&i),
            None => Err(N2WError::Key(sn_key_repr(value))),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `to_currency` is a wholesale override (below). `to_cheque` is *not*
    // overridden in Python, so the trait default — a faithful port of
    // `Num2Word_Base.to_cheque` — is what should run, and it needs only the
    // two hooks below plus `money_verbose` (default: `to_cardinal`, exactly
    // what base's `_money_verbose` does).
    //
    // Everything else stays at its default on purpose: `pluralize`,
    // `cents_verbose`, `cents_terse`, `currency_adjective` and
    // `currency_precision` are all unreachable for SN. `CURRENCY_PRECISION` is
    // `{}` on the live class, so the default `100` is already correct and an
    // override would be noise.

    /// For `Num2Word_Base.to_cheque`'s
    /// `'Currency code "%s" not implemented for "%s"' % (currency, self.__class__.__name__)`.
    fn lang_name(&self) -> &str {
        "Num2Word_SN"
    }

    /// `CURRENCY_FORMS[code]`. Used by the inherited `to_cheque`; SN's own
    /// `to_currency` does its lookup inline because it raises a *different*
    /// message (module quirk 9).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_SN.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, n, currency="USD", cents=True, separator="ne"):
    /// ```
    ///
    /// Note the signature: **no `adjective` parameter** (module quirk 13), so
    /// `_adjective` is ignored here. Nothing in this body reaches
    /// `currency::default_to_currency`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it to this language's own default ("ne") before the body.
        let separator = separator.unwrap_or(self.default_separator());

        let mut result: Vec<String> = Vec::new();

        // `decimal_val = Decimal(str(n))` — the *signed* value. The `abs`
        // below happens afterwards and does not affect this test.
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };

        // `has_fractional_cents = (decimal_val * 100) % 1 != 0`. The 100 is
        // hardcoded in Python — `CURRENCY_PRECISION` is never consulted on this
        // path, which is why SN has no KWD/BHD or JPY divisor behaviour.
        //
        // `Decimal.__mod__` truncates toward zero rather than flooring, and so
        // does `with_scale(0)`; they agree. For a `!= 0` test the sign is moot
        // anyway.
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        // `if value < 0: result.append(self.negword); value = abs(value)`.
        // negword goes in as its own list element, so the `" ".join` below
        // yields a plain "minus " prefix.
        if val.is_negative() {
            result.push(NEGWORD.to_string());
        }
        let abs_dec = decimal_val.abs();

        // Python calls `_split_currency` *before* checking the currency code,
        // but that call cannot raise for int/float input, so hoisting the
        // lookup above it is unobservable — and it keeps the unknown-code error
        // winning over the fractional-cents bail below, which is the order
        // Python produces.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            // SN's own wording. NOT Base's
            // `Currency code "X" not implemented for "Y"` — that message still
            // comes out of `to_cheque`, which SN does not override.
            N2WError::NotImplemented(format!("Currency {} not implemented for Shona", currency))
        })?;
        // `major_singular, major_plural = currency_forms[0]` — the plurals are
        // unpacked and then never read (module quirk 14).
        let major_singular = &forms.unit[0];
        let minor_singular = &forms.subunit[0];

        // ---- _split_currency(value, has_fractional_cents) ----
        let integer_part: BigInt;
        let decimal_part: BigInt;
        if has_fractional_cents {
            // `integer_part = int(Decimal(str(value)))` — *exact decimal*
            // truncation on this arm, unlike the float truncation below.
            integer_part = abs_dec.with_scale(0).as_bigint_and_exponent().0;
            // `decimal_part = decimal_val*100 - integer_part*100` is a Decimal,
            // non-integral by construction, and is read only under
            // `if cents and decimal_part:` — so a `cents=False` call never
            // needs it and stays fully supported here.
            if cents {
                // Python renders it as
                // `self.to_cardinal_float(float(decimal_part))`, and SN's
                // `to_cardinal_float` is its *own* override that slices the
                // digits straight out of `str(number)`. That makes the result
                // depend on `repr(float)` in ways the trait's
                // `cardinal_from_decimal` provably does not reproduce: at
                // 1.0000001 the cents float is 1e-05, whose repr has no "."
                // at all, so Python prints a bare "zero" where the float path
                // would print "zero poindi zero zero zero zero motsi"; and at
                // 1.00000015 the repr is "1.5e-05", so Python's
                // `int("e")` raises ValueError. Reimplementing CPython's
                // shortest-repr *and* its fixed/exponential switchover is
                // exactly the drift this core refuses to own.
                //
                // So: bail. `num2words.__init__` wraps the Rust currency call
                // in `except NotImplementedError: pass` and falls through to
                // the Python converter, which gets all of the above right.
                // No corpus row reaches this branch — every `sn` currency arg
                // has at most two decimal places. See `concerns`.
                return Err(N2WError::NotImplemented(
                    "Num2Word_SN.to_currency: fractional cents render via \
                     Num2Word_SN.to_cardinal_float, whose repr(float) \
                     semantics this core does not reproduce; deferring to the \
                     Python converter"
                        .into(),
                ));
            }
            decimal_part = BigInt::zero(); // unread: `if cents` is already false
        } else {
            match val {
                // int input: `int(value)` is the value itself (arbitrary
                // precision, no float round-trip), and
                // `(value - integer_part) * 100` is exact int `0`.
                CurrencyValue::Int(i) => {
                    integer_part = i.abs();
                    decimal_part = BigInt::zero();
                }
                CurrencyValue::Decimal { .. } => {
                    // float input: `int(value)` and
                    // `int(round((value - integer_part) * 100))` are *binary*
                    // operations in Python, so they run on f64 here rather than
                    // on the BigDecimal — the two genuinely disagree above
                    // 2**53. `str(1e23)` is "1e+23", so the BigDecimal is
                    // 10**23 while `int(1e23)` is 99999999999999991611392, and
                    // Python prints the latter (verified). The BigDecimal was
                    // parsed from `repr(float)`, which round-trips, so `to_f64`
                    // hands back the identical double Python held.
                    let f = abs_dec.to_f64().ok_or_else(|| {
                        // Unreachable for float input: repr(float) is always
                        // finite and in f64 range. Only an out-of-scope Decimal
                        // input (e.g. `Decimal("1e400")`) could land here.
                        N2WError::Value(format!("cannot represent {} as f64", abs_dec))
                    })?;
                    let ip = f.trunc();
                    // Python's `round()` is half-to-**even**; `f64::round()` is
                    // half-away-from-zero, so `round_ties_even()` is required.
                    // (Ties are in fact unreachable: this arm only runs when
                    // `repr(value)` has <= 2 decimals — anything longer is
                    // `has_fractional_cents` — so the product is a whole number
                    // of cents plus float noise, and 2.675 goes to the arm
                    // above instead of rounding to 67 vs 68 here.)
                    let dp = ((f - ip) * 100.0).round_ties_even();
                    integer_part = BigInt::from_f64(ip)
                        .ok_or_else(|| N2WError::Value(format!("cannot int() {}", ip)))?;
                    decimal_part = BigInt::from_f64(dp)
                        .ok_or_else(|| N2WError::Value(format!("cannot int() {}", dp)))?;
                }
            }
        }

        // ---- major currency unit ----
        //
        // `integer_part` is >= 0 here: Python took `abs(value)` before the
        // split, so the negative arm cannot reach these tables.
        let ten = BigInt::from(10);
        if integer_part.is_one() {
            // `major_singular + " rimwe"` — the only place the bare singular
            // is used.
            result.push(format!("{} rimwe", major_singular));
        } else if integer_part == BigInt::from(2) {
            // The irregular 2. Note "ma" is glued onto the *singular*, so ZAR
            // gives "marandi maviri" without ever reading its plural form.
            result.push(format!("ma{} maviri", major_singular));
        } else if integer_part.is_positive() && integer_part < ten {
            // Python: `elif integer_part < 10 and integer_part in self.ones:`
            // then `if integer_part in self.tens_forms:` (keys 2..=9).
            // 0/1/2 are handled above, so integer_part is 3..=9 here — always
            // an `ones` key *and* always a `tens_forms` key, which makes both
            // the second half of the outer guard and that arm's inner `else`
            // (the cardinal fallback) dead code.
            let i = integer_part.to_usize().expect("3 <= integer_part <= 9");
            result.push(format!("ma{} {}", major_singular, TENS_FORMS[i]));
        } else {
            // Two Python arms merged, because they emit the same string:
            //   * integer_part == 0 enters `integer_part in self.ones` but
            //     misses `tens_forms`, taking that arm's cardinal fallback
            //     -> "madhora zero";
            //   * integer_part >= 10 takes the outer `else`
            //     -> "madhora zana", "madhora gumi nepiri", ...
            result.push(format!(
                "ma{} {}",
                major_singular,
                int_to_sn_word(&integer_part)
            ));
        }

        // ---- minor currency unit (cents) ----
        //
        // `if cents and decimal_part:` — `0` is falsy, so SN never emits a
        // "zero cents" tail (module quirk 10). That is what makes 1 and 1.0
        // agree on "dhora rimwe", and why `has_decimal` is irrelevant here.
        if cents && !decimal_part.is_zero() {
            if decimal_part.is_one() {
                result.push(format!("{}{} rimwe", separator, minor_singular));
            } else {
                // The `isinstance(decimal_part, Decimal)` arm is the
                // fractional-cents branch, already returned above.
                result.push(format!(
                    "{}{} {}",
                    separator,
                    minor_singular,
                    int_to_sn_word(&decimal_part)
                ));
            }
        }

        // The separator is concatenated onto the subunit with no space
        // ("ne" + "sendi" -> "nesendi") while this join adds one in front of
        // it. A custom separator therefore double-spaces (module quirk 12).
        Ok(result.join(" "))
    }
}
