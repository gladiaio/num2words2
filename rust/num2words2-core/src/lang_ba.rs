//! Port of `lang_BA.py` (Bashkir).
//!
//! Registry check: `CONVERTER_CLASSES["ba"] = lang_BA.Num2Word_BA()`
//! (`__init__.py:332`), so `Num2Word_BA` is the class this key resolves to.
//!
//! Shape: **self-contained**. `Num2Word_BA` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`. `base.py`'s
//! `__init__` only builds `self.cards` / sets `self.MAXVAL` when one of those
//! three attributes exists, so for Bashkir **neither is ever created**:
//! `cards`/`merge` stay at their trait defaults here and are never consulted.
//! Python consequently has **no overflow check at all** — `to_cardinal` is
//! overridden outright and recurses over `_int_to_word` for every integer (see
//! bug 1 below). The port adds one (gladiaio/num2words2#203): `maxval` is
//! 10^12, where "миллиард" would start to stack, and every mode raises
//! `OverflowError` from there.
//!
//! `setup()` overrides two inherited attributes:
//!   * `negword  = "минус "`  (note the **trailing space**, see below)
//!   * `pointword = "өтөр"`
//!
//! All four in-scope modes are overridden by `Num2Word_BA`; nothing is
//! inherited from `Num2Word_Base` except the attributes above. In particular
//! `to_ordinal` does **not** call `verify_ordinal`, so negative ordinals are
//! accepted rather than raising (`to_ordinal(-1)` == "минус беренсе").
//!
//! # No error paths
//!
//! For integer input this language cannot raise. Every list index is guarded by
//! a range check that bounds it to 0..=9, and `_int_to_word` is never reached
//! with 0 from a recursive call (each recursion site tests the remainder for
//! truthiness first), so the unreachable `ones[0] == ""` slot never surfaces.
//!
//! The **float path** does add one: `Value` (Python `ValueError`), raised when
//! `str(number)` comes out in exponent notation and Bashkir feeds that text to
//! `int()`. See [`float_str`] and [`py_int`].
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. The following look wrong but are exactly what
//! Python emits, verified against the frozen corpus:
//!
//! 1. **The scale table stops at "миллиард" (10^9) and then recurses.** The
//!    final `else` arm divides by 10^9 and calls `_int_to_word` on the quotient
//!    with no further scale words, so large values stack "миллиард" onto a
//!    recursively-built multiplier instead of naming higher scales:
//!      * 10^12 → "бер мең миллиард"          (lit. "one thousand milliard")
//!      * 10^15 → "бер миллион миллиард"
//!      * 10^18 → "бер миллиард миллиард"
//!      * 10^21 → "бер мең миллиард миллиард"
//!    All four are corpus rows. This makes Python's `_int_to_word`
//!    **unbounded**, and on a large enough integer the port's recursion
//!    overflowed the native stack. Fixed (gladiaio/num2words2#203): the port
//!    raises `OverflowError` from 10^12, so none of the rows above is
//!    produced any more.
//! 2. **`negword` is used raw, not stripped.** Bashkir's `to_cardinal` does
//!    `ret = self.negword` and concatenates directly, unlike
//!    `Num2Word_Base.to_cardinal` which does `"%s " % self.negword.strip()`.
//!    The two happen to agree because `negword` is exactly `"минус "`, but the
//!    trailing space is load-bearing and is preserved verbatim in [`NEGWORD`].
//! 3. **Fixed (gladiaio/num2words2#148): `to_ordinal` glued "-се" onto the
//!    bare cardinal** regardless of the final word's phonology — "нуль-се",
//!    "ун бер-се", "туғыҙ-се". The port harmonises the ending on the last
//!    word instead: "нуленсе", "ун беренсе", "туғыҙынсы", "өсөнсө"; see
//!    [`join_ordinal`]. The digit form (`to_ordinal_num`) is unchanged.
//! 4. **`to_ordinal_num` never validates**, so it emits "0-се" and "-1-се".
//! 5. **Hundreds are always explicit**: 100 → "бер йөҙ" ("one hundred"), never
//!    a bare "йөҙ". Likewise 1000 → "бер мең". Same for the recursive scales.
//!
//! # The float/Decimal path: `str()`, not `float2tuple`
//!
//! `Num2Word_BA` overrides **`to_cardinal`**, not `to_cardinal_float`, and its
//! non-integer branch never reaches `base.float2tuple`:
//!
//! ```python
//! n = str(number).strip()
//! if n.startswith("-"):
//!     n = n[1:]; ret = self.negword
//! else:
//!     ret = ""
//! if "." in n:
//!     left, right = n.split(".", 1)
//!     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
//!     ret += " ".join(self._int_to_word(int(d)) for d in right)
//!     return ret
//! ```
//!
//! So the fractional digits are **the characters of `str(number)`**, not a
//! `abs(value - pre) * 10**precision` product. This has three consequences that
//! set Bashkir apart from the 26 languages inheriting `default_to_cardinal_float`:
//!
//! 1. **The f64 artefacts never arise.** `float2tuple`'s `674.9999999999998` /
//!    `< 0.01` rescue dance has no analogue here: `str(2.675)` is `"2.675"` and
//!    the digits are read straight off. The two routes happen to agree on 2.675
//!    and 1.005 — because `repr` is shortest-round-trip and the heuristic exists
//!    precisely to recover it — but they agree by coincidence, not by shared
//!    code. Nothing in this file rounds, so `round_ties_even` has no place in it.
//! 2. **`self.precision` is never read**, so the `precision=` kwarg (issue #580)
//!    is a **silent no-op** for Bashkir. `num2words(2.675, lang="ba",
//!    precision=1)` is still "ике өтөр алты ете биш", verified against the live
//!    interpreter. [`LangBa::to_cardinal_float`] therefore ignores
//!    `precision_override`, and `FloatValue::precision` with it.
//! 3. **Exponent notation raises `ValueError`.** Bashkir hands `str()`'s output
//!    to `int()` unfiltered, so any value `repr` spells with an `e` crashes:
//!    `num2words(1e16, lang="ba")` is
//!    `ValueError: invalid literal for int() with base 10: '1e+16'`, and
//!    `num2words(1.5e-5, lang="ba")` is `... : 'e'` — the latter from the
//!    *per-character* `int(d)` in the join, after `int("1")` has already
//!    succeeded. Both messages are reproduced verbatim.
//!
//! ## Why `str()` is reconstructed rather than passed in
//!
//! The trait hands over a [`FloatValue`], not the text Python printed, so the
//! text has to be rebuilt. That is not a re-implementation of float formatting:
//! Rust's `{:e}` and Python's `repr` both emit the *shortest round-trip* digits,
//! which is the hard part and is already solved on both sides. What is rebuilt
//! is only the **layout** — where the point goes and whether the exponent form
//! is used — which is a range check on one integer. See [`float_str`].
//!
//! `FloatValue`'s Float/Decimal split is load-bearing here for a reason beyond
//! precision: **`float` and `Decimal` stringify by different rules**, and the
//! outputs genuinely diverge.
//!
//! | value | `str()` | `to_cardinal("ba")` |
//! |---|---|---|
//! | `0.00001` (float) | `'1e-05'` | `ValueError` |
//! | `Decimal("0.00001")` | `'0.00001'` | нуль өтөр нуль нуль нуль нуль бер |
//!
//! `repr(float)` goes exponential outside `-4 < decpt <= 16`;
//! `Decimal.__str__` goes exponential (with a capital `E` and an *unpadded*
//! exponent) when its own exponent is positive or `leftdigits <= -6`. Collapsing
//! the two arms — or routing both through one spelling — changes real answers,
//! so [`float_str`] and [`decimal_str`] are separate ports of the two separate
//! CPython functions.
//!
//! # Currency
//!
//! `Num2Word_BA` overrides `to_currency` **wholesale** and shares almost nothing
//! with `Num2Word_Base.to_currency`. It also defines its own three-entry
//! `CURRENCY_FORMS` (RUB/USD/EUR) and no `CURRENCY_PRECISION` /
//! `CURRENCY_ADJECTIVES`, so the divisor is 100 for every code.
//!
//! `to_cheque` is **not** overridden, so it comes straight from
//! `Num2Word_Base` — which means the two entry points disagree about unknown
//! currency codes, and the corpus records both halves of that split:
//!
//! * `to_currency` does `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["RUB"])`
//!   — an unknown code **silently falls back to roubles**. `currency:GBP` on
//!   12.34 yields "ун ике **һум** һәм утыҙ дүрт **тин**", not pounds, and
//!   never raises.
//! * `to_cheque` does `CURRENCY_FORMS[currency]` — an unknown code raises
//!   `NotImplementedError`. `cheque:GBP` is a corpus error row.
//!
//! So [`Lang::currency_forms`] here is the *strict* lookup (`None` for unknown,
//! which is what `default_to_cheque` needs) and the RUB fallback lives inside
//! [`LangBa::to_currency`], mirroring where Python puts it.
//!
//! Because `to_currency` is fully overridden, these inherited hooks are never
//! reached and are deliberately left at their trait defaults:
//! `pluralize` (Bashkir never calls it — `cr1[0]`/`cr2[0]` are indexed
//! directly, so the abstract `Num2Word_Base.pluralize` never fires),
//! `cents_verbose`, `cents_terse`, `currency_precision` and
//! `currency_adjective`. `money_verbose` keeps the base default
//! (`self.to_cardinal(number)`), which is what `to_cheque` calls.
//!
//! ## More reproduced Python quirks (currency)
//!
//! 6. **`self.parse_currency` does not exist.** `to_currency` opens with
//!    `try: ... self.parse_currency(val) except AttributeError:`. No class in
//!    the MRO defines `parse_currency` (`base.py` has `parse_minus` and imports
//!    `parse_currency_parts` as a free function), so the attribute lookup
//!    always raises and the `except` body is the *only* path ever taken. The
//!    `try` block is dead code; this port implements the fallback directly.
//! 7. **`adjective` and `cents=False` are near-no-ops.** `adjective` is
//!    accepted and never read. `cents=False` does not switch to terse digits
//!    the way `Num2Word_Base` does — it drops the subunit segment entirely.
//! 8. **Cents are truncated to two digits, never rounded**:
//!    `int(parts[1][:2].ljust(2, "0"))`. So `1.005` → 0 cents ("00"), `0.5` →
//!    50 cents ("5" → "50"), `0.01` → 1 cent. See [`split_currency`].
//! 9. **A float with zero cents prints no cents at all.** `1.0` → "бер евро",
//!    because the guard is `if cents and right` and `right == 0` is falsy.
//!    This is the one language where the int-vs-float split that
//!    `base.to_currency` hinges on makes **no** observable difference: `1` and
//!    `1.0` both render "бер евро". The distinction is still threaded through
//!    faithfully rather than collapsed, because it is the *value* of `right`
//!    that decides, not the type.
//!
//! # Python `//` vs Rust `/`
//!
//! `_int_to_word` is only ever entered with a non-negative value (`to_cardinal`
//! strips the "-" from the *string* before `int()`), and every recursive call
//! passes a quotient or remainder of a non-negative number. On non-negative
//! operands Python's floor-`//`/`%` and Rust's truncating-`/`/`%` coincide, so
//! plain `/` and `%` are used below.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::str::FromStr;

/// `_int_to_word`'s early return for 0.
const ZERO_WORD: &str = "нуль";

/// `setup(): self.negword = "минус "` — trailing space intentional, see the
/// module docs. Bashkir's `to_cardinal` concatenates this without stripping.
const NEGWORD: &str = "минус ";

/// `setup(): self.pointword = "өтөр"`. Read by the float branch of
/// `to_cardinal` and by nothing else — Bashkir never reaches
/// `Num2Word_Base.to_cardinal_float`, which is the only other consumer.
const POINTWORD: &str = "өтөр";

/// `to_currency(..., separator=" һәм ", ...)` — Bashkir's own default, which
/// differs from `Num2Word_Base`'s `","`. See [`LangBa::to_currency`] for why
/// this is applied here rather than taken from the caller.
const SEPARATOR: &str = " һәм ";

/// `Num2Word_Base.to_currency(..., separator=",", ...)`. The Python shim calls
/// the core with `kwargs.get("separator", ",")`, so this value reaching us is
/// indistinguishable from "the caller said nothing" — see
/// [`LangBa::to_currency`].
const BASE_SEPARATOR: &str = ",";

/// `to_currency(..., currency="RUB", ...)`: the code an unrecognised currency
/// falls back to, via `CURRENCY_FORMS.get(currency, CURRENCY_FORMS["RUB"])`.
const FALLBACK_CURRENCY: &str = "RUB";

/// `ones`. Index 0 is `""` in Python and is unreachable: `_int_to_word(0)`
/// returns "нуль" before any lookup, and no recursive call passes 0.
const ONES: [&str; 10] = [
    "", "бер", "ике", "өс", "дүрт", "биш", "алты", "ете", "һигеҙ", "туғыҙ",
];

/// `tens`. Indices 0 and 1 are `""` in Python and are unreachable: this table
/// is only indexed by `number // 10` for `20 <= number < 100`, i.e. 2..=9.
const TENS: [&str; 10] = [
    "",
    "",
    "егерме",
    "утыҙ",
    "ҡырҡ",
    "илле",
    "алтмыш",
    "етмеш",
    "һикһән",
    "туҡһан",
];

/// `teens`, indexed by `number - 10` for `10 <= number < 20`.
const TEENS: [&str; 10] = [
    "ун",
    "ун бер",
    "ун ике",
    "ун өс",
    "ун дүрт",
    "ун биш",
    "ун алты",
    "ун ете",
    "ун һигеҙ",
    "ун туғыҙ",
];

/// Narrow a value already proven `< 10` (or `< 100` before the `- 10`) to a
/// table index. The guard in every caller bounds the value, so the `unwrap`
/// cannot fire.
fn idx(n: &BigInt) -> usize {
    n.to_usize()
        .expect("guarded by a range check to 0..=99 before this call")
}

/// `Num2Word_BA._int_to_word`. Requires `number >= 0`.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // if number < 10: return ones[number]
    if *number < BigInt::from(10u32) {
        return ONES[idx(number)].to_string();
    }

    // elif number < 20: return teens[number - 10]
    if *number < BigInt::from(20u32) {
        return TEENS[idx(number) - 10].to_string();
    }

    // elif number < 100:
    //     return tens[number // 10] + (" " + ones[number % 10] if number % 10 else "")
    if *number < BigInt::from(100u32) {
        let div = number / 10u32;
        let rem = number % 10u32;
        let mut out = TENS[idx(&div)].to_string();
        if !rem.is_zero() {
            out.push(' ');
            out.push_str(ONES[idx(&rem)]);
        }
        return out;
    }

    // elif number < 1000:
    //     return ones[number // 100] + " йөҙ"
    //            + (" " + self._int_to_word(number % 100) if number % 100 else "")
    if *number < BigInt::from(1000u32) {
        let div = number / 100u32;
        let rem = number % 100u32;
        let mut out = ONES[idx(&div)].to_string();
        out.push_str(" йөҙ");
        if !rem.is_zero() {
            out.push(' ');
            out.push_str(&int_to_word(&rem));
        }
        return out;
    }

    // The three remaining arms are structurally identical: recurse on the
    // quotient, append the scale word, recurse on a non-zero remainder.
    // Bashkir has no scale word above миллиард, so the last arm's quotient
    // recursion is what produces "бер мең миллиард" for 10^12 (see module docs).
    let (scale, scale_word) = if *number < BigInt::from(1_000_000u32) {
        (BigInt::from(1_000u32), " мең")
    } else if *number < BigInt::from(1_000_000_000u32) {
        (BigInt::from(1_000_000u32), " миллион")
    } else {
        (BigInt::from(1_000_000_000u32), " миллиард")
    };

    let div = number / &scale;
    let rem = number % &scale;
    let mut out = int_to_word(&div);
    out.push_str(scale_word);
    if !rem.is_zero() {
        out.push(' ');
        out.push_str(&int_to_word(&rem));
    }
    out
}

/// Attach the ordinal ending to the **last word** of a spelled cardinal
/// (gladiaio/num2words2#148). Upstream glued a fixed "-се" onto the whole
/// cardinal ("бер-се", "өс-се", "ун-се"). Bashkir ordinals take
/// *-(ы)нсы/-(е)нсе*, harmonised for backness, and rounded to *-(ө)нсө*
/// after a stem whose last vowel is ө (Wiktionary,
/// `Module:number_list/data/ba` and the entry өсөнсө):
///
/// * back stem (last vowel а ы о у): *-ынсы* / *-нсы* — унынсы, алтынсы, and
///   the loan миллионынсы (attested on ba.wikipedia), so о does not round;
/// * front stem (ә е и ү э): *-енсе* / *-нсе* — беренсе, икенсе, дүртенсе;
/// * ө: *-өнсө* — өсөнсө, йөҙөнсө;
/// * a final soft sign is dropped and the stem counts as front: нуль ->
///   нуленсе ("Нуленсе меридиан", ba.wikipedia).
///
/// 123rd is therefore "бер йөҙ егерме өсөнсө". A cardinal that is not a word
/// at all keeps the old hyphenated "-се".
fn join_ordinal(cardinal: &str) -> String {
    const VOWELS: &str = "аыоуяюёәеиөүэ";
    let (head, stem) = match cardinal.rfind(' ') {
        Some(i) => cardinal.split_at(i + 1),
        None => ("", cardinal),
    };
    if let Some(soft) = stem.strip_suffix('ь') {
        return format!("{}{}енсе", head, soft);
    }
    let last = match stem.chars().filter(|c| VOWELS.contains(*c)).last() {
        Some(v) => v,
        None => return format!("{}-се", cardinal),
    };
    let (link, ending) = match last {
        'а' | 'ы' | 'о' | 'у' | 'я' | 'ю' | 'ё' => ("ы", "нсы"),
        'ө' => ("ө", "нсө"),
        _ => ("е", "нсе"),
    };
    if stem.ends_with(|c: char| VOWELS.contains(c)) {
        format!("{}{}{}", head, stem, ending)
    } else {
        format!("{}{}{}{}", head, stem, link, ending)
    }
}

/// `Num2Word_BA.to_cardinal` restricted to integer input.
///
/// Python stringifies the value, peels a leading "-" off the *string*, and
/// feeds the rest to `int()`. With an integral input there is no "." in the
/// string, so the `pointword` branch is dead and this reduces to sign handling
/// plus `_int_to_word` on the magnitude.
fn cardinal(value: &BigInt) -> Result<String> {
    if value.is_negative() {
        // ret = self.negword  ->  "минус " (already ends in a space)
        Ok(format!("{}{}", NEGWORD, checked_int_to_word(&value.abs())?))
    } else {
        checked_int_to_word(value)
    }
}

/// Python's `int(s)` on a fragment of `str(number)`, preserving the ValueError.
///
/// `int()` also tolerates surrounding whitespace, a leading `+`/`-` and `_`
/// digit separators. None of those can occur in a fragment of `repr(float)` or
/// `Decimal.__str__` — `to_cardinal` peels the sign off before splitting, and
/// neither speller emits underscores — so `BigInt::from_str` is exact here.
///
/// The message is CPython's verbatim. It is not compared by the corpus (which
/// records only `type(e).__name__`), but a caller may be matching on it, and
/// both spellings this can produce are real: `'1e+16'` for a whole
/// exponent-form string that had no `.`, and `'e'` for the per-character
/// `int(d)` in the fractional join.
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// CPython's `repr(float)` — which is `str(float)` on Python 3.
///
/// # The digits: why `{:e}` alone is not enough
///
/// Rust's shortest-round-trip spelling (`{}`, `{:e}`) and CPython's `repr`
/// agree on *how many* significant digits a double needs, and almost always on
/// what they are — but **they break exact ties differently**, and that is the
/// banker's-rounding trap wearing a different hat:
///
/// ```text
/// f64::from_bits(4835708046038665529)   exact value 1962311374373454.25
///     Rust  format!("{}", f)  ->  "1962311374373454.3"    ties away from zero
///     CPython repr(f)         ->  "1962311374373454.2"    ties to even
/// ```
///
/// Both spellings round-trip, so both are legitimate "shortest" answers; they
/// are equidistant from the true value and the algorithms disagree on which to
/// pick. David Gay's `dtoa` (mode 0), which CPython uses, resolves the tie to
/// **even**. Differential fuzzing put this at ~50 in 210k random doubles — rare,
/// but Bashkir spells every one of those digits out loud, so it is a wrong
/// answer, not a rounding wobble.
///
/// The fix is to never use Rust's shortest *digits*, only its shortest
/// *length*:
///
/// 1. `{:e}` says how many significant digits the shortest round-trip needs
///    (`k`). Tie choice cannot change that count — both candidates in a tie are
///    `k` digits long — so this number is trustworthy.
/// 2. `{:.*e}` with `k-1` re-emits exactly `k` significant digits from the
///    double's exact value. Rust's *fixed-precision* float formatting rounds
///    half **to even** (`{:.0}` of 2.5 is "2", of 0.125 to 2dp is "0.12"), which
///    is the rule `dtoa` mode 0 applies, so the digits now match CPython's.
///
/// Step 2 is what makes the nearest-`k`-digit decimal come out, ties to even —
/// which is precisely `repr`'s definition once `k` is fixed.
///
/// # The layout
///
/// Given the digits, the rest is presentation. Rust's `{:e}` is
/// unconditionally exponential (`1.234e-5`) while `repr` switches between fixed
/// and exponential; reconstructing that switch is a range check on one integer.
///
/// `decpt` is the point's position relative to the digit string
/// (`value == 0.<digits> * 10^decpt`, so `decpt == e10 + 1`). CPython's
/// `format_float_short` uses fixed notation for `-4 < decpt <= 16` and
/// exponential outside it, which is why `str(1e15)` is `"1000000000000000.0"`
/// but `str(1e16)` is `"1e+16"`, and `str(1e-4)` is `"0.0001"` but `str(1e-5)`
/// is `"1e-05"`. Both edges verified against the interpreter; the whole
/// function was then differentially fuzzed against `str()` over ~210k doubles
/// (random bit patterns plus the subnormal/overflow extremes) — it is what
/// caught the tie bug above, and it now reports zero divergence.
///
/// Fixed notation appends `".0"` to an integral value (CPython's
/// `Py_DTSF_ADD_DOT_0`), which is why `1.0` is "бер өтөр нуль" and not "бер" —
/// the `"." in n` test in `to_cardinal` sees that dot.
fn float_str(f: f64) -> String {
    // str(nan)/str(inf)/str(-inf). Unreachable through num2words — Python
    // computes `abs(Decimal(str(value)).as_tuple().exponent)` for `precision`
    // before ever calling us, and `Decimal("nan").as_tuple().exponent` is the
    // string "n", so `abs()` raises TypeError up there first. Spelled out
    // anyway so this stays total rather than panicking in `split_once`.
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // The sign is taken from the bit, not from `< 0.0`: repr(-0.0) is "-0.0",
    // and Bashkir's to_cardinal peels that "-" into a negword. `-0.0 < 0.0` is
    // false, so testing the value would silently drop it.
    let sign = if f.is_sign_negative() { "-" } else { "" };
    let a = f.abs();

    // Step 1: the shortest round-trip *length*. Only the digit count is read
    // from this string — its digits may be the away-from-zero side of a tie.
    // `0.0` gives "0e0", so `k` is never 0.
    let shortest = format!("{:e}", a);
    let (mantissa, _) = shortest
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let k = mantissa.chars().filter(char::is_ascii_digit).count();

    // Step 2: re-emit exactly `k` significant digits from the exact value, with
    // Rust's fixed-precision formatter — which rounds half to even, as dtoa
    // mode 0 does. The exponent is re-read rather than reused: rounding at `k`
    // digits is allowed to carry (9.99e2 -> 1.00e3), and only this string knows.
    let exact = format!("{:.*e}", k - 1, a);
    let (mantissa, e10) = exact
        .split_once('e')
        .expect("{:e} on a finite f64 always emits an exponent");
    let e10: i64 = e10
        .parse()
        .expect("{:e}'s exponent is a plain decimal integer");
    let ds: String = mantissa.chars().filter(|c| *c != '.').collect();
    let decpt = e10 + 1;

    if decpt > 16 || decpt <= -4 {
        // Exponential: one digit, the rest, then a >=2-digit signed exponent
        // ("1e+16", "1e-05", "1.5e-05").
        let mantissa = if ds.len() > 1 {
            format!("{}.{}", &ds[..1], &ds[1..])
        } else {
            ds
        };
        let sgn = if e10 < 0 { '-' } else { '+' };
        return format!("{}{}e{}{:02}", sign, mantissa, sgn, e10.abs());
    }

    if decpt <= 0 {
        // 0.5 -> decpt 0; 0.01 -> decpt -1.
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), ds)
    } else if decpt as usize >= ds.len() {
        // Integral: Py_DTSF_ADD_DOT_0 appends ".0". 1.0, 100.0, 1e15, ...
        format!(
            "{}{}{}.0",
            sign,
            ds,
            "0".repeat(decpt as usize - ds.len())
        )
    } else {
        format!("{}{}.{}", sign, &ds[..decpt as usize], &ds[decpt as usize..])
    }
}

/// CPython's `decimal.Decimal.__str__` (the `eng=False` half of it).
///
/// A `Decimal` spells itself by a **different rule than a float**, and Bashkir
/// reads the spelling character by character, so the difference is observable:
/// `Decimal("0.00001")` prints `"0.00001"` and converts fine, while the float
/// `0.00001` prints `"1e-05"` and raises. Hence a separate port rather than
/// reuse of [`float_str`].
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # fixed notation
/// else:
///     dotplace = 1                   # scientific notation
/// if dotplace <= 0:
///     intpart, fracpart = '0', '.' + '0'*(-dotplace) + self._int
/// elif dotplace >= len(self._int):
///     intpart, fracpart = self._int + '0'*(dotplace-len(self._int)), ''
/// else:
///     intpart, fracpart = self._int[:dotplace], '.' + self._int[dotplace:]
/// exp = '' if leftdigits == dotplace else 'E%+d' % (leftdigits - dotplace)
/// return sign + intpart + fracpart + exp
/// ```
///
/// `bigdecimal`'s parser stores the shim's `str(value)` text as `(int_val,
/// scale)` without normalising, and `Decimal` likewise keeps `_int`/`_exp` as
/// written — so `Decimal._int` is `int_val.abs().to_string()` and `Decimal._exp`
/// is `-scale`, digit for digit. `Decimal("1.10")` and `BigDecimal::from_str
/// ("1.10")` both hold `(110, 2)`, which is why "1.10" keeps its trailing zero
/// and renders "бер өтөр бер нуль" rather than "бер өтөр бер".
///
/// Note the two ways this differs from float spelling beyond the threshold:
/// the exponent marker is a capital `E` and is **not** zero-padded (`"1E-7"`,
/// not `"1e-07"`), and fixed notation never appends `".0"` — `str(Decimal("5"))`
/// is `"5"`, which has no `.` and so takes `to_cardinal`'s *integer* branch.
///
/// # The one unrecoverable case
///
/// A negative-zero `Decimal`. `Decimal("-0.0")` keeps `_sign = 1` and prints
/// `"-0.0"`, but `BigDecimal` carries the sign inside its `BigInt` mantissa,
/// where zero has no sign — `(0, 1)` is all that survives, so this returns
/// `"0.0"` and the negword is lost. Flagged in the port's `concerns`; no corpus
/// row reaches it, and it is not fixable from inside this file.
fn decimal_str(d: &BigDecimal) -> String {
    let (mantissa, scale) = d.as_bigint_and_exponent();
    let sign = if mantissa.is_negative() { "-" } else { "" };
    // `_int`: the digits, unsigned. BigInt::to_string of a non-negative is
    // ASCII digits only, so every byte slice below lands on a char boundary.
    let ds = mantissa.abs().to_string();
    // `_exp`: bigdecimal's `scale` is the negated Decimal exponent.
    let exp = -scale;
    let leftdigits = exp + ds.len() as i64;

    let dotplace = if exp <= 0 && leftdigits > -6 {
        leftdigits
    } else {
        1
    };

    let (intpart, fracpart) = if dotplace <= 0 {
        // `leftdigits > -6` bounds this to at most 5 padding zeros.
        (
            "0".to_string(),
            format!(".{}{}", "0".repeat((-dotplace) as usize), ds),
        )
    } else if dotplace as usize >= ds.len() {
        (
            format!("{}{}", ds, "0".repeat(dotplace as usize - ds.len())),
            String::new(),
        )
    } else {
        (
            ds[..dotplace as usize].to_string(),
            format!(".{}", &ds[dotplace as usize..]),
        )
    };

    // `'E%+d'` — Rust's `{:+}` on an integer matches `%+d` exactly.
    let e = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, e)
}

/// `Num2Word_BA.to_cardinal`, the general (string-driven) body.
///
/// ```python
/// n = str(number).strip()
/// if n.startswith("-"):
///     n = n[1:]; ret = self.negword
/// else:
///     ret = ""
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
///     ret += " ".join(self._int_to_word(int(d)) for d in right)
///     return ret
/// else:
///     return ret + self._int_to_word(int(n))
/// ```
///
/// The integer modes do not route through here: for a `BigInt` the string can
/// never contain a `.`, so [`cardinal`] is this function with the dead branch
/// removed, and it stays separate to keep the 67k verified integer rows off a
/// string round trip.
///
/// Evaluation order is load-bearing for the error cases. Python builds the
/// left-hand side *before* the `" ".join(...)` generator is consumed, so
/// `int(left)` raises first; within the join, digits are converted left to
/// right and the first bad one wins. `"1.5e-05"` therefore reports `'e'` — not
/// `'5e-05'` and not `'1.5e-05'`.
fn cardinal_from_str(n: &str) -> Result<String> {
    // `.strip()`. A no-op on every string `str()` can produce, kept because
    // Python does it.
    let n = n.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        // `ret = self.negword` — used raw, trailing space and all (quirk 2).
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    // `n.split(".", 1)` — the *first* dot, so "1.5e-05" splits to ("1", "5e-05").
    match n.split_once('.') {
        Some((left, right)) => {
            ret.push_str(&checked_int_to_word(&py_int(left)?)?);
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // `" ".join(self._int_to_word(int(d)) for d in right)`. Indexed by
            // chars, not bytes; `right` is ASCII in practice but a stray
            // multi-byte char must reach `int()` whole to be reported whole.
            for (i, d) in right.chars().enumerate() {
                if i > 0 {
                    ret.push(' ');
                }
                // Python's `int(d)` sees a one-character string.
                let mut buf = [0u8; 4];
                ret.push_str(&checked_int_to_word(&py_int(d.encode_utf8(&mut buf))?)?);
            }
            Ok(ret)
        }
        None => {
            ret.push_str(&checked_int_to_word(&py_int(n)?)?);
            Ok(ret)
        }
    }
}

/// `Num2Word_BA._split_currency(abs(val))`, returning `(left, right)`.
///
/// ```python
/// parts = str(n).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// Python re-stringifies the number here and slices the digits. That string
/// round-trip is reproduced *arithmetically* rather than literally, because the
/// `BigDecimal` we hold was already parsed from `str(value)` on the Python side
/// and does not retain the original spelling. The two agree exactly for every
/// plain-decimal spelling — which is every float in `1e-4 <= |x| < 1e16` and
/// every `Decimal` written without an exponent:
///
/// * `parts[1][:2]` right-padded to two digits and parsed is precisely
///   `floor(frac * 100)` — truncation, no rounding. One digit ("0.5" → "50")
///   pads up; three or more ("1.005" → "00") get cut. Both are floor.
/// * `int(parts[0])` on a non-negative number is `floor`.
///
/// Trailing zeros in the source string cannot change the answer either
/// (`"0.5"` → 5/10¹ and `"0.50"` → 50/10² both give 50), so it does not matter
/// whether `BigDecimal::from_str` preserved the scale.
///
/// The known gap is exponent notation — see the `concerns` note on this port.
/// `str(1e16)` is `"1e+16"`, and Python then feeds `"1e+16"` to `int()` and
/// raises `ValueError`; `str(1.23e16)` is `"1.2345678901234568e+16"`, which
/// *splits* on the "." and quietly yields 1 unit and 23 cents. Neither shape is
/// recoverable from a parsed `BigDecimal`, and no corpus row exercises them.
fn split_currency(val: &CurrencyValue) -> (BigInt, BigInt) {
    match val {
        // str(int) never contains "." -> parts has length 1 -> right = 0.
        // This is the branch that gives an int no cents; note that a *float*
        // with zero cents lands on the same output by a different route.
        CurrencyValue::Int(i) => (i.abs(), BigInt::zero()),
        CurrencyValue::Decimal { value: d, .. } => {
            // `val = abs(val)` happened before `_split_currency` in Python.
            let d = d.abs();
            // d == mantissa / 10^exp, mantissa >= 0 after the abs.
            let (mantissa, exp) = d.as_bigint_and_exponent();
            if exp <= 0 {
                // Integral value: no fractional digits to slice.
                let scale = BigInt::from(10u32).pow((-exp) as u32);
                (mantissa * scale, BigInt::zero())
            } else {
                let scale = BigInt::from(10u32).pow(exp as u32);
                // Non-negative operands, so div_rem == Python's //, %.
                let (left, rem) = mantissa.div_rem(&scale);
                // floor(rem/10^exp * 100) == int(parts[1][:2].ljust(2, "0")).
                let right = (rem * 100u32) / scale;
                (left, right)
            }
        }
    }
}

pub struct LangBa {
    /// `CURRENCY_FORMS`, built once. Rebuilding it per call is what made an
    /// earlier revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangBa {
    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "RUB": (("һум", "һум"), ("тин", "тин")),
        //     "USD": (("доллар", "доллар"), ("цент", "цент")),
        //     "EUR": (("евро", "евро"), ("цент", "цент")),
        // }
        // Both forms of every pair are identical in Bashkir, and only index 0
        // is ever read by `to_currency` — but the arity is kept at Python's
        // two because `to_cheque` takes `cr1[-1]`.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "RUB",
            CurrencyForms::new(&["һум", "һум"], &["тин", "тин"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["доллар", "доллар"], &["цент", "цент"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["евро", "евро"], &["цент", "цент"]),
        );
        LangBa { currency_forms }
    }
}

impl Default for LangBa {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the scale table stops at
/// миллиард (10^9), so from 10^12 the module would stack "миллиард" on
/// itself.
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(12))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl Lang for LangBa {
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
        "RUB"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " һәм "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        cardinal(value)
    }

    /// `Num2Word_BA.to_cardinal` reached with a `float` or a `Decimal`.
    ///
    /// Bashkir does not override `to_cardinal_float` and never calls it — it
    /// overrides `to_cardinal` and handles non-integers inline off `str(number)`
    /// (see the module docs). So this hook is not
    /// `default_to_cardinal_float`-shaped at all: no `float2tuple`, no
    /// `pointword` gating on `precision > 0`, no `title()`, no rounding.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is **ignored**,
    /// because Python ignores it. `__init__.py` does set `converter.precision`
    /// for the call — `hasattr(converter, "precision")` is true, it is 2 by
    /// default — but `Num2Word_BA.to_cardinal` never reads the attribute, so the
    /// assignment has no effect. Verified: `num2words(2.675, lang="ba",
    /// precision=1)` == `num2words(2.675, lang="ba")` == "ике өтөр алты ете биш".
    /// `FloatValue::precision` is unread for the same reason.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // The whole float path is `str(number)`, and float and Decimal spell
        // themselves by different rules — keep the arms apart.
        let n = match value {
            FloatValue::Float { value, .. } => float_str(*value),
            FloatValue::Decimal { value, .. } => decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    /// `return self.to_cardinal(number) + "-се"` upstream; the ending is now
    /// harmonised on the last word (bug 3, fixed — see [`join_ordinal`]).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(join_ordinal(&cardinal(value)?))
    }

    /// `return str(number) + "-се"` — no validation, so "-1-се" is expected.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-се", value))
    }

    /// `return self.to_cardinal(val) + " йыл"` — the `longval` parameter is
    /// accepted and then ignored by Python; years get no special casing.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{} йыл", cardinal(value)?))
    }

    // ---- float / Decimal entry routing --------------------------------
    //
    // `to_ordinal` / `to_ordinal_num` / `to_year` have no type guard in
    // Python, so a float or Decimal flows through them exactly as an int
    // does: the ordinal suffix / year word wraps the *full* decimal phrase
    // (or, for `to_ordinal_num`, the raw `str(number)`).

    /// `to_ordinal(float/Decimal)`: `self.to_cardinal(number)` + the ordinal
    /// ending — "биш өтөр нуленсе" for 5.0; the exponential-form ValueError from the
    /// cardinal propagates unchanged.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(join_ordinal(&self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "-се"` — no `int()`,
    /// so it succeeds where the other modes raise ("1e+16-се") and "-0.0"
    /// keeps its textual minus. `repr_str` is Python's `str(number)`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}-се", repr_str))
    }

    /// `to_year(float/Decimal)`: `self.to_cardinal(val) + " йыл"` — the full
    /// float cardinal plus the year word, ValueErrors included.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{} йыл", self.cardinal_float_entry(value, None)?))
    }

    /// `converter.str_to_number` — Base's `Decimal(value)`, which BA does not
    /// override. The `Inf` interception reproduces what happens *next* on the
    /// pinned path: `to_cardinal(Decimal("Infinity"))` reads `str(number)` ==
    /// "Infinity" (the "-Infinity" case strips its sign textually first),
    /// finds no ".", and dies in `int("Infinity")` with ValueError; the
    /// binding's shared Inf sentinel would otherwise raise OverflowError.
    /// (NaN needs no interception: the binding's ValueError already matches
    /// `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_BA"
    }

    /// The **strict** `CURRENCY_FORMS[code]` lookup, as `to_cheque` performs
    /// it. `None` here is what makes `default_to_cheque` raise
    /// `NotImplementedError` for GBP/JPY/KWD/BHD/INR/CNY/CHF, matching the
    /// corpus error rows. `to_currency` deliberately does *not* route through
    /// the strictness — it has its own RUB fallback, see below.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_BA.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="RUB", cents=True,
    ///                 separator=" һәм ", adjective=False):
    ///     try:
    ///         left, right, is_negative = self.parse_currency(val)
    ///     except AttributeError:                  # always taken, see quirk 6
    ///         is_negative = False
    ///         if val < 0:
    ///             is_negative = True
    ///             val = abs(val)
    ///         left, right = self._split_currency(val)
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["RUB"])
    ///
    ///     left_str = self._int_to_word(int(left))
    ///     cents_str = self._int_to_word(int(right)) if cents and right else ""
    ///
    ///     result = left_str + " " + cr1[0]
    ///     if cents_str:
    ///         result += separator + cents_str + " " + cr2[0]
    ///     return self.negword + result if is_negative else result
    /// ```
    ///
    /// # Why `separator` is not taken at face value
    ///
    /// Python resolves `separator` at the *call site* — omit the kwarg and the
    /// binding fills in this class's own default, `" һәм "`. The Rust core
    /// cannot see that: the shim in `__init__.py` calls it with
    /// `kwargs.get("separator", ",")`, substituting `Num2Word_Base`'s default
    /// for "unspecified", and `bench/diff_test.py` likewise hardcodes `","`.
    /// So `","` arriving here carries no information — it is the sentinel for
    /// an absent kwarg — and every corpus row for this language is a default
    /// call expecting `" һәм "`.
    ///
    /// Treating `","` as "use my default" reproduces Python for the whole
    /// reachable input space bar one case: an explicit `separator=","`, which
    /// Python would honour and this maps back to `" һәм "`. That single
    /// divergence is flagged in the port's `concerns`; it is not fixable from
    /// inside this file, since the shim erases the distinction before we are
    /// called.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Accepted and never read, exactly as in Python: Num2Word_BA has no
        // CURRENCY_ADJECTIVES and its to_currency never consults them.
        _adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        let separator = if separator == BASE_SEPARATOR {
            SEPARATOR
        } else {
            separator
        };

        // CURRENCY_FORMS.get(currency, CURRENCY_FORMS["RUB"]) — an unknown
        // code silently becomes roubles rather than raising. Python evaluates
        // the fallback eagerly as the `.get` default; "RUB" is always present.
        let forms = self.currency_forms.get(currency).unwrap_or_else(|| {
            self.currency_forms
                .get(FALLBACK_CURRENCY)
                .expect("CURRENCY_FORMS[\"RUB\"] is the eager .get default")
        });

        // if val < 0: is_negative = True; val = abs(val)
        let is_negative = val.is_negative();
        let (left, right) = split_currency(val);

        let left_str = checked_int_to_word(&left)?;
        let mut result = format!("{} {}", left_str, forms.unit[0]);

        // `if cents and right` — a zero `right` is falsy, so a float with no
        // cents (1.0) drops the segment just like an int does. Note this is
        // *not* the terse path: cents=False emits nothing at all.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&checked_int_to_word(&right)?);
            result.push(' ');
            result.push_str(&forms.subunit[0]);
        }

        // `self.negword + result` — concatenated raw. negword is "минус "
        // and already carries its trailing space (quirk 2); unlike
        // Num2Word_Base this does not strip and re-space it.
        Ok(if is_negative {
            format!("{}{}", NEGWORD, result)
        } else {
            result
        })
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;

    fn cardinal_f(f: f64) -> Result<String> {
        LangBa::new().to_cardinal_float(&FloatValue::Float { value: f, precision: 0 }, None)
    }

    fn cardinal_d(s: &str) -> Result<String> {
        LangBa::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 0,
            },
            None,
        )
    }

    /// Every `"to": "cardinal"` row with a dot in `arg`, from bench/corpus.jsonl.
    #[test]
    fn corpus_float_rows() {
        for (v, want) in [
            (0.0, "нуль өтөр нуль"),
            (0.5, "нуль өтөр биш"),
            (1.0, "бер өтөр нуль"),
            (1.5, "бер өтөр биш"),
            (2.25, "ике өтөр ике биш"),
            (3.14, "өс өтөр бер дүрт"),
            (0.01, "нуль өтөр нуль бер"),
            (0.1, "нуль өтөр бер"),
            (0.99, "нуль өтөр туғыҙ туғыҙ"),
            (1.01, "бер өтөр нуль бер"),
            (12.34, "ун ике өтөр өс дүрт"),
            (99.99, "туҡһан туғыҙ өтөр туғыҙ туғыҙ"),
            (100.5, "бер йөҙ өтөр биш"),
            (1234.56, "бер мең ике йөҙ утыҙ дүрт өтөр биш алты"),
            (-0.5, "минус нуль өтөр биш"),
            (-1.5, "минус бер өтөр биш"),
            (-12.34, "минус ун ике өтөр өс дүрт"),
            (1.005, "бер өтөр нуль нуль биш"),
            (2.675, "ике өтөр алты ете биш"),
        ] {
            assert_eq!(cardinal_f(v).unwrap(), want, "float {:?}", v);
        }
    }

    /// Every `"to": "cardinal_dec"` row from bench/corpus.jsonl.
    #[test]
    fn corpus_decimal_rows() {
        for (s, want) in [
            ("0.01", "нуль өтөр нуль бер"),
            ("1.10", "бер өтөр бер нуль"),
            ("12.345", "ун ике өтөр өс дүрт биш"),
            (
                "98746251323.99",
                "туҡһан һигеҙ миллиард ете йөҙ ҡырҡ алты миллион ике йөҙ илле бер мең \
                 өс йөҙ егерме өс өтөр туғыҙ туғыҙ",
            ),
            ("0.001", "нуль өтөр нуль нуль бер"),
        ] {
            assert_eq!(cardinal_d(s).unwrap(), want, "Decimal({:?})", s);
        }
    }

    /// `Decimal("1.10")` keeps its trailing zero and `Decimal("98746251323029.99")`
    /// keeps every digit — the two properties issue #603 is about. A `float()`
    /// cast would drop the first and corrupt the second at trillion scale.
    #[test]
    fn decimal_arm_is_exact_and_scale_preserving() {
        assert_eq!(cardinal_d("1.10").unwrap(), "бер өтөр бер нуль");
        assert_eq!(cardinal_d("1.1").unwrap(), "бер өтөр бер");
        assert_eq!(cardinal_d("5.00").unwrap(), "биш өтөр нуль нуль");
        // No ".0" is appended by Decimal.__str__, so this takes the *integer*
        // branch of to_cardinal — unlike the float 5.0, right below.
        assert_eq!(cardinal_d("5").unwrap(), "биш");
        assert_eq!(cardinal_f(5.0).unwrap(), "биш өтөр нуль");
    }

    /// float and Decimal spell themselves by different rules, and Bashkir reads
    /// the spelling. `0.00001` is the case where that is load-bearing.
    #[test]
    fn float_and_decimal_spellings_diverge() {
        // repr(1e-05) == '1e-05'  ->  int('1e-05') explodes.
        assert!(matches!(cardinal_f(0.00001), Err(N2WError::Value(_))));
        // str(Decimal('0.00001')) == '0.00001'  ->  converts fine.
        assert_eq!(
            cardinal_d("0.00001").unwrap(),
            "нуль өтөр нуль нуль нуль нуль бер"
        );
        // Decimal goes exponential one decade later than float does.
        assert_eq!(
            cardinal_d("0.000001").unwrap(),
            "нуль өтөр нуль нуль нуль нуль нуль бер"
        );
        assert!(matches!(cardinal_d("0.0000001"), Err(N2WError::Value(_))));
    }

    /// Python's `repr` switches to exponent notation at `decpt > 16` / `<= -4`,
    /// and Bashkir hands the result to `int()` unfiltered.
    #[test]
    fn exponential_repr_raises_value_error() {
        // No "." in '1e+16' -> the whole string reaches int().
        match cardinal_f(1e16) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // '1.5e-05' has a "." -> int("1") succeeds, then the per-character
        // int(d) in the join trips on 'e'.
        match cardinal_f(1.5e-5) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'e'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // The sign is peeled off the string before int() sees it.
        match cardinal_f(-1e16) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1e+16'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
        // Decimal's own exponent form: capital E, unpadded exponent.
        match cardinal_d("1E+2") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1E+2'")
            }
            other => panic!("expected ValueError, got {:?}", other),
        }
    }

    /// The values either side of both repr thresholds still convert.
    #[test]
    fn plain_reprs_at_the_exponential_edges_do_not_raise() {
        // decpt == 16: the last fixed-notation decade.
        assert_eq!(cardinal_f(1e11).unwrap(), "бер йөҙ миллиард өтөр нуль");
        // 1e15 is past the #203 ceiling (10^12) but still fixed notation.
        assert!(matches!(cardinal_f(1e15), Err(N2WError::Overflow(_))));
        // decpt == -3: the last fixed-notation decade going down.
        assert_eq!(
            cardinal_f(0.0001).unwrap(),
            "нуль өтөр нуль нуль нуль бер"
        );
        assert!(matches!(cardinal_f(1e-5), Err(N2WError::Value(_))));
        assert!(matches!(cardinal_f(1e16), Err(N2WError::Value(_))));
    }

    /// `repr(float)` breaks exact decimal ties **to even**; Rust's shortest
    /// formatter breaks them away from zero. [`float_str`] must follow CPython.
    ///
    /// Each value below is exactly representable and sits exactly halfway
    /// between two 17-significant-digit decimals, so `format!("{}", f)` and
    /// `repr(f)` disagree on the final digit — which Bashkir says out loud.
    #[test]
    fn exact_decimal_ties_round_to_even_like_cpython() {
        // 1962311374373454.25 -> repr '...54.2', Rust Display '...54.3'.
        assert_eq!(float_str(1962311374373454.25), "1962311374373454.2");
        assert_eq!(float_str(2050093655521678.25), "2050093655521678.2");
        assert_eq!(float_str(-145360241606786.125), "-145360241606786.12");
        assert_eq!(float_str(106779538212252.625), "106779538212252.62");
        // Every exact 17-digit tie is far past the #203 ceiling (10^12), so
        // the reading itself now overflows.
        assert!(matches!(cardinal_f(1962311374373454.25), Err(N2WError::Overflow(_))));
    }

    /// [`float_str`] is CPython's `repr`, not Rust's `{}`.
    #[test]
    fn float_str_matches_cpython_repr() {
        for (f, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.0, "1.0"),
            (0.5, "0.5"),
            (0.01, "0.01"),
            (100.5, "100.5"),
            (1.005, "1.005"),
            (2.675, "2.675"),
            (1234.56, "1234.56"),
            (1e15, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1e21, "1e+21"),
            (1e100, "1e+100"),
            (0.0001, "0.0001"),
            (0.00001, "1e-05"),
            (1.5e-5, "1.5e-05"),
            (5e-324, "5e-324"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (123456789012345.6, "123456789012345.6"),
        ] {
            assert_eq!(float_str(f), want, "repr({:?})", f);
        }
    }

    /// [`decimal_str`] is CPython's `Decimal.__str__`, which is a different
    /// function from `repr(float)` — capital E, unpadded exponent, no ".0".
    #[test]
    fn decimal_str_matches_cpython() {
        for (s, want) in [
            ("0.01", "0.01"),
            ("1.10", "1.10"),
            ("5", "5"),
            ("5.00", "5.00"),
            ("100", "100"),
            ("0", "0"),
            ("0.00", "0.00"),
            ("0.00001", "0.00001"),
            ("0.000001", "0.000001"),
            ("0.0000001", "1E-7"),
            ("1E+2", "1E+2"),
            ("1E+16", "1E+16"),
            ("-1.005", "-1.005"),
            ("98746251323029.99", "98746251323029.99"),
        ] {
            assert_eq!(decimal_str(&BigDecimal::from_str(s).unwrap()), want, "Decimal({:?})", s);
        }
    }

    /// -0.5 keeps its sign even though `int(-0.5) == 0` carries none, because
    /// Bashkir peels the "-" off the *string* rather than testing the value.
    /// A float -0.0 keeps it too: the sign bit survives, and `is_sign_negative`
    /// is what reads it (`-0.0 < 0.0` is false).
    #[test]
    fn negative_zero_and_negative_fractions_keep_the_negword() {
        assert_eq!(cardinal_f(-0.5).unwrap(), "минус нуль өтөр биш");
        assert_eq!(cardinal_f(-0.0).unwrap(), "минус нуль өтөр нуль");
        assert_eq!(cardinal_f(-0.01).unwrap(), "минус нуль өтөр нуль бер");
        assert_eq!(cardinal_d("-0.5").unwrap(), "минус нуль өтөр биш");
    }

    /// The `precision=` kwarg is a no-op for Bashkir: `to_cardinal` never reads
    /// `self.precision`. Neither `precision_override` nor `FloatValue::precision`
    /// may change the answer.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangBa::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        let want = "ике өтөр алты ете биш";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want);
        assert_eq!(l.to_cardinal_float(&v, Some(1)).unwrap(), want);
        assert_eq!(l.to_cardinal_float(&v, Some(9)).unwrap(), want);
        // The carried precision is equally unread.
        let v = FloatValue::Float { value: 2.675, precision: 99 };
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want);
    }

    /// The float path must not disturb the integer modes, which route through
    /// `cardinal` and never touch a string.
    #[test]
    fn integer_modes_are_untouched() {
        let l = LangBa::new();
        assert_eq!(l.to_cardinal(&BigInt::from(0)).unwrap(), "нуль");
        assert_eq!(l.to_cardinal(&BigInt::from(-12)).unwrap(), "минус ун ике");
        assert_eq!(l.to_ordinal(&BigInt::from(11)).unwrap(), "ун беренсе");
    }
}
