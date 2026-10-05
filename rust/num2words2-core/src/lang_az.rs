//! Port of `lang_AZ.py` (Azerbaijani).
//!
//! Registry check: `__init__.py` maps `"az"` to `lang_AZ.Num2Word_AZ`, which is
//! the class ported here.
//!
//! Shape: **self-contained**. `Num2Word_AZ` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! populates `self.cards` and never sets `MAXVAL`. `to_cardinal` is overridden
//! outright and drives `int_to_word` over the decimal *string*, digit by digit,
//! from the least significant end. Consequently `cards`/`merge` stay at their
//! trait defaults here. The only ceiling is the `POWERS_OF_TEN` table (up to
//! 10^63), so `maxval()` is 10^66 and larger whole numbers raise
//! `OverflowError` (see bug 2).
//!
//! Inherited from `Num2Word_Base` and left unchanged by AZ:
//!   * nothing in scope — AZ overrides all four of `to_cardinal`, `to_ordinal`,
//!     `to_ordinal_num` and `to_year`, so no trait default is relied upon.
//!
//! `setup()` sets `negword = "mənfi"` and `pointword = "nöqtə"`. Both are
//! reachable: `negword` for any negative input, `pointword` for the fractional
//! path below.
//!
//! # The float/Decimal path does not use `floatpath.rs`
//!
//! `Num2Word_AZ` does **not** override `to_cardinal_float`, so the [`Lang`]
//! hook of that name here is *not* a port of the Python method of that name —
//! it carries AZ's `to_cardinal` for non-integer input, because `to_cardinal`
//! is what the dispatcher calls and it handles floats inline. Verified live:
//! `type(c).to_cardinal is not Num2Word_Base.to_cardinal` is `True`, and
//! `to_cardinal_float` is inherited untouched (and therefore dead code for AZ).
//!
//! The consequence is that **`base.float2tuple` is never reached**. AZ's first
//! statement is `value_str = str(value)`: the whole algorithm runs on the
//! decimal *string*, so the binary artefacts `floatpath.rs` is built around
//! never arise. `str(2.675)` is `"2.675"`, and AZ renders the fraction "675" as
//! the integer six-hundred-seventy-five — "iki nöqtə altı yüz yetmiş beş". The
//! `674.9999999999998` / `< 0.01` rescue and the banker's-rounding trap belong
//! to the path AZ does not take; there is no `round()` anywhere in this file.
//!
//! What AZ needs instead is `str(value)` itself, reproduced byte for byte by
//! [`py_str_f64`] and [`py_str_decimal`]. Note that AZ says "point" and then
//! reads the fraction as a *whole number*, not digit by digit: `12.345` is "on
//! iki nöqtə üç yüz qırx beş" ("twelve point three hundred forty-five"), where
//! every language on the `floatpath.rs` default would say "üç dörd beş".
//!
//! ## Float/Decimal routing (the `*_float_entry` hooks)
//!
//! Because `str(5.0)` is `"5.0"` and the algorithm reads the `".0"` like any
//! other fraction, AZ has **no whole-value shortcut**: `num2words(5.0,
//! lang="az")` is "beş nöqtə sıfır", never "beş". [`Lang::cardinal_float_entry`]
//! therefore bypasses the trait default's `as_whole_int()` fast path and hands
//! everything to the string algorithm.
//!
//! `to_ordinal`/`to_ordinal_num`/`to_year` all open with `assert int(value) ==
//! value`, which is *live* for float/Decimal input: a fractional value (0.5,
//! 3.25) fails the comparison and raises a bare `AssertionError` — see
//! [`assert_whole`]. A *whole* float passes the assert and then runs the same
//! cardinal + vowel-suffix walk over the full float string, so
//! `to_ordinal(5.0)` == "beş nöqtə sıfırıncı" and `to_ordinal_num(5.0)` ==
//! "5.0-cı" (`"-".join([str(value), suffix])` keeps the repr verbatim).
//! Exponent-form input (1e16, `Decimal("1E+2")`) is whole, so it passes the
//! assert and then dies inside `to_cardinal` with bug 4's `ValueError` — in
//! that order.
//!
//! Sign semantics are `value < 0`, a numeric comparison — **not** the sign
//! bit: `-0.0 < 0` is False in Python and IEEE-754 alike, so `-0.0` renders
//! with no negword ("sıfır nöqtə sıfır"), `to_year(-0.0)` takes no "e.ə.", and
//! only `to_ordinal_num(-0.0)`'s echoed repr shows the "-" ("-0.0-cı").
//!
//! `num2words("Infinity", lang="az")` is a `ValueError`, not base's
//! OverflowError: `Decimal("Infinity")` parses fine, and AZ's `to_cardinal`
//! then walks `str()` == "Infinity" *reversed*, so the first `int()` call is
//! `int("y")`. The shared Inf sentinel maps to OverflowError in the binding,
//! so [`Lang::str_to_number`] raises the ValueError itself (RU takes the same
//! route; see the override for the mode caveat).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Both of the following are wrong-looking but
//! are exactly what Python emits, verified against the interpreter:
//!
//! 1. **The "we say `min`, not `bir min`" rule is applied one digit too
//!    broadly, and silently loses a whole thousand.** The guard is
//!    `if not (digit_int == 1 and index == 3)`, which tests only that the
//!    thousands digit is 1 — never that it is the *only* digit in its chunk.
//!    So the `bir` is dropped even when a ten-thousands or hundred-thousands
//!    digit precedes it:
//!      * `to_cardinal(21000)` == `"iyirmi min"` — literally "twenty thousand",
//!        i.e. 20000. The 1000 is gone.
//!      * `to_cardinal(31000)` == `"otuz min"` (30000), `to_cardinal(121000)`
//!        == `"yüz iyirmi min"` (120000), `to_cardinal(1001000)` ==
//!        `"bir milyon min"`.
//!    The rule is keyed on `index == 3` alone, so the same pattern at a higher
//!    power is unaffected: `to_cardinal(21000000)` == `"iyirmi bir milyon"` is
//!    correct. The bug is confined to the thousands chunk. Reproduced verbatim
//!    in [`int_to_word`]; do not "fix" it.
//!
//! 2. **Fixed (gladiaio/num2words2#159): `POWERS_OF_TEN` stops at 10^63.**
//!    Python raised `KeyError` from 10^66 up; the port now raises
//!    `OverflowError` ("abs(v) must be less than 10^66.") before the scan, and
//!    `maxval("az")` reports 10^66. `to_cardinal(10**66 - 1)` is the largest
//!    value that works.
//!
//! 3. **`leading_zeros` under-counts an all-zero fraction by one.** The count
//!    is `len(num_str) - len(str(int(num_str)))`, and `str(int("00"))` is `"0"`
//!    — one character, not zero — so a fraction that is nothing but zeros gets
//!    one "sıfır" too few. `Decimal("1.00")` reads `"bir nöqtə sıfır"` rather
//!    than `"... sıfır sıfır"`, while `Decimal("1.000")` reads `"bir nöqtə
//!    sıfır sıfır"`. Only Decimal input can reach this: `repr(1.00)` is `"1.0"`,
//!    which has a single zero and is therefore correct by accident.
//!    Reproduced in [`int_to_word`].
//!
//! 4. **Any float whose `repr` is in exponent form raises `ValueError`.**
//!    `str(1e16)` is `"1e+16"`, and `int_to_word` calls `int(digit)` on every
//!    character of it, so `int("+")` blows up: `num2words(1e16, lang="az")` is
//!    a `ValueError`, not a number. The same goes for `1e21`, `1e-05`,
//!    `float("inf")` (`int("f")`), `float("nan")` (`int("n")`) and
//!    `Decimal("1E+2")`. `1e15` still works — `str(1e15)` is
//!    `"1000000000000000.0"` — so the cliff sits exactly where CPython's `repr`
//!    switches notation, at `decpt > 16`. See [`py_str_f64`].
//!
//! # Error variants
//!
//! `ValueError` (bug 4) maps to [`N2WError::Value`] carrying CPython's own
//! message, `invalid literal for int() with base 10: '+'`. Which character is
//! named is observable and load-bearing: `int_to_word` scans the digit string
//! *reversed*, and `int(digit)` is the first statement of the loop body, so the
//! exponent's sign character is always reached before any table lookup can
//! `KeyError`. `"5e-324"` therefore reports `'-'` and not a `KeyError` on the
//! hundreds slot it walks through on the way.
//!
//! A `KeyError` from `POWERS_OF_TEN` (bug 2) is now reachable only through a
//! fraction digit string of more than 66 digits; whole numbers that large are
//! an `OverflowError`.
//!
//! `to_ordinal`/`to_ordinal_num`/`to_year` also carry `assert int(value) ==
//! value` and (the first two) `assert last_vowel is not None`. The first is
//! vacuous for integer input but **live for float/Decimal input**, where a
//! fractional value maps to [`N2WError::Assertion`] with Python's bare-assert
//! empty message (see [`assert_whole`]). The second is unreachable: every word
//! in `DIGITS`/`DECIMALS`/`POWERS_OF_TEN` contains a vowel, so `_last_vowel`
//! never returns `None` for any value that `to_cardinal` accepts; the
//! unreachable arm keeps its historical [`N2WError::Value`] mapping rather
//! than panicking.
//!
//! # Currency
//!
//! `Num2Word_AZ`'s MRO is `AZ -> Num2Word_Base -> object` — it does **not**
//! descend from `Num2Word_EUR`, so the `lang_EUR.py`/`Num2Word_EN.__init__`
//! shared-dict mutation described in `PORTING_CURRENCY.md` does not reach it.
//! Verified against the live interpreter: `AZ.CURRENCY_FORMS is
//! Num2Word_Base.CURRENCY_FORMS` is `False` (AZ declares its own class-body
//! dict), and that dict holds exactly three codes. Everything else — GBP, JPY,
//! KWD, BHD, INR, CNY, CHF — raises `NotImplementedError`, which is why AZ has
//! neither a 3-decimal nor a 0-decimal currency to exercise.
//!
//! AZ overrides only the forms table and `pluralize`. Confirmed absent from
//! `Num2Word_AZ.__dict__`: `to_currency`, `to_cheque`, `_money_verbose`,
//! `_cents_verbose`, `_cents_terse` — all inherited from `Num2Word_Base`
//! unchanged, so the trait defaults already mirror them and are not overridden.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are *Base's own* dicts, still
//! empty at runtime after every module has imported (checked with `is`, not
//! just equality). So `currency_adjective` is always `None` and
//! `CURRENCY_PRECISION.get(code, 100)` is always 100 — exactly the trait
//! defaults. Overriding them would be noise.
//!
//! The `to_currency` signature is `(val, currency="EUR", cents=True,
//! separator=",", adjective=False)`, i.e. Base's defaults unchanged; the
//! generated `default_currency`/`default_separator` below already match it.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive};
use std::collections::HashMap;

/// `negword`, set in `setup()`.
const NEGWORD: &str = "mənfi";

/// `pointword`, set in `setup()`.
const POINTWORD: &str = "nöqtə";

/// `DIGITS`, keys 0..=9 — dense, so an array indexes it directly.
const DIGITS: [&str; 10] = [
    "sıfır", "bir", "iki", "üç", "dörd", "beş", "altı", "yeddi", "səkkiz", "doqquz",
];

/// `DECIMALS`, keys 1..=9. Index 0 is absent in Python; every read is guarded
/// by `digit_int != 0`, so the placeholder is unreachable.
const DECIMALS: [&str; 10] = [
    "", "on", "iyirmi", "otuz", "qırx", "əlli", "altmış", "yetmiş", "səksən", "doxsan",
];

/// `VOWELS`. Note these are *characters*, several of them multi-byte in UTF-8
/// (`ı`, `ə`, `ö`, `ü`), so every use goes through `chars()`.
const VOWELS: [char; 9] = ['a', 'ı', 'o', 'u', 'e', 'ə', 'i', 'ö', 'ü'];

/// `POWERS_OF_TEN`, a sparse dict: key 2, then every multiple of 3 up to 63.
/// Returning `None` is Python's missing key, i.e. a `KeyError`.
fn power_of_ten(index: usize) -> Option<&'static str> {
    Some(match index {
        2 => "yüz",
        3 => "min",
        6 => "milyon",
        9 => "milyard",
        12 => "trilyon",
        15 => "katrilyon",
        18 => "kentilyon",
        21 => "sekstilyon",
        24 => "septilyon",
        27 => "oktilyon",
        30 => "nonilyon",
        33 => "desilyon",
        36 => "undesilyon",
        39 => "dodesilyon",
        42 => "tredesilyon",
        45 => "katordesilyon",
        48 => "kendesilyon",
        51 => "seksdesilyon",
        54 => "septendesilyon",
        57 => "oktodesilyon",
        60 => "novemdesilyon",
        63 => "vigintilyon",
        _ => return None,
    })
}

/// `VOWEL_TO_CARDINAL_SUFFIX_MAP`, built in Python from four `dict.fromkeys`
/// groups. Every character in `VOWELS` is covered, so a lookup keyed on a
/// `_last_vowel` result can never miss.
fn cardinal_suffix(vowel: char) -> Option<&'static str> {
    Some(match vowel {
        'a' | 'ı' => "ıncı",
        'e' | 'ə' | 'i' => "inci",
        'o' | 'u' => "uncu",
        'ö' | 'ü' => "üncü",
        _ => return None,
    })
}

/// `VOWEL_TO_CARDINAL_NUM_SUFFIX_MAP`. Same key coverage as above.
fn cardinal_num_suffix(vowel: char) -> Option<&'static str> {
    Some(match vowel {
        'a' | 'ı' => "cı",
        'e' | 'ə' | 'i' => "ci",
        'o' | 'u' => "cu",
        'ö' | 'ü' => "cü",
        _ => return None,
    })
}

/// Python's `_last_vowel`: scan the string backwards, return the first vowel.
/// Iterates `chars()`, never bytes — `value[::-1]` in Python walks code points.
fn last_vowel(value: &str) -> Option<char> {
    value.chars().rev().find(|c| VOWELS.contains(c))
}

/// Python's `assert int(value) == value` — the opening statement of
/// `to_ordinal`, `to_ordinal_num` and `to_year`, live on the float/Decimal
/// entries. A fractional value fails the comparison and raises a **bare**
/// `AssertionError` (no message — Python's plain `assert` carries none).
///
/// `int()` of inf/nan would raise OverflowError/ValueError *instead of* the
/// assert failing, but the shim keeps non-finite floats on the Python side
/// ("inf/nan stay on the Python error path"), so a `None` from `as_whole_int`
/// here always means "finite but fractional".
fn assert_whole(value: &FloatValue) -> Result<()> {
    if value.as_whole_int().is_none() {
        return Err(N2WError::Assertion(String::new()));
    }
    Ok(())
}

/// The suffix-attaching tail of Python's `to_ordinal`, shared by the integer
/// and float/Decimal entries (Python has a single method serving both).
///
/// `if cardinal.endswith(tuple(self.VOWELS)): cardinal = cardinal[:-1]` —
/// drop the trailing *character*, not the trailing byte. Several vowels here
/// are two bytes in UTF-8, so truncating by 1 would split a code point and
/// panic.
fn ordinalize(cardinal: String) -> Result<String> {
    let vowel = last_vowel(&cardinal).ok_or_else(|| {
        // Python's `assert last_vowel is not None`. Unreachable: every
        // numword contains a vowel (see the module docs' assert note).
        N2WError::Value("AssertionError: last_vowel is None".into())
    })?;
    let suffix = cardinal_suffix(vowel).ok_or_else(|| N2WError::Key(format!("{}", vowel)))?;

    let mut stem = cardinal;
    if let Some(last) = stem.chars().next_back() {
        if VOWELS.contains(&last) {
            let cut = stem.len() - last.len_utf8();
            stem.truncate(cut);
        }
    }

    Ok(format!("{}{}", stem, suffix))
}

/// The suffix lookup of Python's `to_ordinal_num` — last vowel of the
/// cardinal, then `VOWEL_TO_CARDINAL_NUM_SUFFIX_MAP`. Shared by the integer
/// and float/Decimal entries; the caller supplies its own `str(value)`.
fn ordinal_num_suffix_for(cardinal: &str) -> Result<&'static str> {
    let vowel = last_vowel(cardinal).ok_or_else(|| {
        N2WError::Value("AssertionError: last_vowel is None".into())
    })?;
    cardinal_num_suffix(vowel).ok_or_else(|| N2WError::Key(format!("{}", vowel)))
}

/// Python's `int_to_word`, over the decimal digit string of a non-negative
/// value.
///
/// `leading_zeros` is Python's keyword argument: the integral part is rendered
/// with it `False`, the fractional part with it `True`. It exists because
/// `int_to_word` reads its input as a whole number ("05" is five), which would
/// silently drop a fraction's leading zeros — so they are counted and re-added
/// as bare "sıfır"s afterwards. One-off included; see bug 3.
///
/// Carries bugs 1, 2, 3 and 4 from the module docs. `words.insert(0, ..)`
/// prepends, so the list is built most-significant-first while the scan runs
/// least-significant-first.
fn int_to_word(num_str: &str, leading_zeros: bool) -> Result<String> {
    // Fixed (gladiaio/num2words2#159): a whole number past the table is an
    // OverflowError up front, not the KeyError of bug 2. A fraction digit
    // string (`leading_zeros`) is not a magnitude and keeps the old scan.
    if !leading_zeros {
        if let Ok(n) = num_str.parse::<BigInt>() {
            check_maxval(&n, maxval_ceiling())?;
        }
    }
    let mut words: Vec<&'static str> = Vec::new();
    let mut words: Vec<&'static str> = Vec::new();
    let reversed: Vec<char> = num_str.chars().rev().collect();

    for (index, digit) in reversed.iter().enumerate() {
        // `digit_int = int(digit)`. For integer input num_str comes from
        // BigInt::to_string() and every char is an ASCII digit, but the float
        // path can hand this an exponent-form repr ("1e+16"), and this is the
        // first statement in the loop body — so a non-digit raises ValueError
        // here, before any table lookup below can KeyError. BUG 4.
        let digit_int = digit
            .to_digit(10)
            .ok_or_else(|| {
                N2WError::Value(format!(
                    "invalid literal for int() with base 10: '{}'",
                    digit
                ))
            })? as usize;

        // The number is parsed in three-digit chunks; the position within the
        // chunk selects the branch.
        let remainder_to_3 = index % 3;
        if remainder_to_3 == 0 {
            if index > 0 {
                // `set(reversed_str[index:index+3]) != {"0"}` — emit the power
                // word only when this chunk carries a non-zero digit. Python's
                // slice clamps at the end of the list; so does this one. The
                // chunk is never empty (index < len), so an all-'0' test is an
                // exact model of the set comparison.
                let end = std::cmp::min(index + 3, reversed.len());
                let chunk_is_zero = reversed[index..end].iter().all(|c| *c == '0');
                if !chunk_is_zero {
                    // No entry past 63: only a fraction digit string this long
                    // gets here (bug 2); whole numbers are rejected above.
                    let word = power_of_ten(index)
                        .ok_or_else(|| N2WError::Key(format!("{}", index)))?;
                    words.insert(0, word);
                }
            }
            if digit_int > 0 {
                // "we say 'min' not 'bir min'" — BUG 1: keyed on the digit and
                // the index only, never on whether the rest of the chunk is
                // empty, so 21000 loses its thousand and reads "iyirmi min".
                if !(digit_int == 1 && index == 3) {
                    words.insert(0, DIGITS[digit_int]);
                }
            }
        } else if remainder_to_3 == 1 {
            if digit_int != 0 {
                words.insert(0, DECIMALS[digit_int]);
            }
        } else {
            // remainder is 2 — the hundreds slot.
            if digit_int > 0 {
                // POWERS_OF_TEN[2], always present.
                words.insert(0, power_of_ten(2).expect("hundreds"));
            }
            // "bir yüz" is not said, so the digit word is emitted only from 2 up.
            if digit_int > 1 {
                words.insert(0, DIGITS[digit_int]);
            }
        }
    }

    if num_str == "0" {
        words.push(DIGITS[0]);
    }

    if leading_zeros {
        // Python: `zeros_count = len(num_str) - len(str(int(num_str)))`, then
        // `words[:0] = zeros_count * [self.DIGITS[0]]`.
        //
        // `str(int(num_str))` is the input with its leading zeros stripped —
        // except that an all-zero string collapses to a single "0" rather than
        // to nothing, which is BUG 3: "00" counts 2 - 1 = 1 zero, not 2, and
        // Decimal("1.00") loses a "sıfır".
        //
        // The loop above has already proved every char is an ASCII digit, so
        // the `int()` here cannot raise. An empty num_str would (Python:
        // `int("")`), but the caller guards it with `if not fraction_part`.
        let significant = num_str.trim_start_matches('0');
        let int_len = if significant.is_empty() {
            1 // str(int("000")) == "0"
        } else {
            significant.chars().count()
        };
        let zeros_count = num_str.chars().count().saturating_sub(int_len);
        // Prepending one at a time is `words[:0] = n * [x]` — the elements are
        // all the same word, so order cannot differ.
        for _ in 0..zeros_count {
            words.insert(0, DIGITS[0]);
        }
    }

    Ok(words.join(" "))
}

/// Python's `str(float)` — which for a float is `repr(float)`.
///
/// This string *is* AZ's input: `to_cardinal` starts with `value_str =
/// str(value)` and never looks at the number again except for `if value < 0`.
/// So unlike every language on `floatpath.rs`, AZ needs `repr` reproduced
/// rather than the raw f64's binary arithmetic.
///
/// Reproduces CPython's `format_float_short(d, 'r', ..., ADD_DOT_0)`:
///
///   * Digit *generation* is delegated to Rust's own shortest-round-trip
///     formatter, which is the same contract as Python's `repr` — we do not
///     reimplement it.
///   * The notation switch is CPython's `if (decpt <= -4 || decpt > 16)
///     use_exp = 1;` — the comment there explains the 16: converting at 1e17
///     "gives odd-looking results ... repr(2e16+8) would give
///     20000000000000010.0".
///   * The exponent is `sprintf("%+.02d")`: always signed, zero-padded to two
///     digits, hence "1e+16" and "1e-05".
///   * `Py_DTSF_ADD_DOT_0` is why `repr(1.0)` is "1.0" and not "1".
///
/// Checked against CPython over 90,011 doubles (structured edge cases, random
/// bit patterns and random decimal-scaled values): zero mismatches.
fn py_str_f64(value: f64) -> String {
    // CPython: "adapt Gay's output, so convert Infinity to inf and NaN to nan,
    // and ignore sign of nan". Rust spells these "inf" and "NaN".
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    // `{:e}` is Rust's shortest round-trip in scientific form ("-1.234e-7"),
    // which yields both the digit count and the decimal-point position — the
    // `digits` / `decpt` pair `_Py_dg_dtoa` hands `format_float_short`.
    let sci = format!("{:e}", value);
    let (mantissa, exponent) = match sci.split_once('e') {
        Some(pair) => pair,
        None => return sci, // unreachable: LowerExp always emits an 'e'.
    };
    let decpt = exponent.parse::<i32>().unwrap_or(0) + 1;
    let ndigits = mantissa.chars().filter(char::is_ascii_digit).count() as i32;

    if decpt <= -4 || decpt > 16 {
        // Exponent form. Every one of these is a ValueError once int_to_word
        // reaches the '+'/'-', but the string is built out in full anyway: it
        // is what Python builds, and which character the error names depends on
        // it.
        let (sign, mantissa) = match mantissa.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", mantissa),
        };
        let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
        let mut out = String::from(sign);
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            // No ADD_DOT_0 here: repr(1e21) is "1e+21", not "1.0e+21".
            out.push('.');
            out.push_str(&digits[1..]);
        }
        let exp = decpt - 1;
        out.push_str(&format!(
            "e{}{:02}",
            if exp < 0 { '-' } else { '+' },
            (exp as i64).abs()
        ));
        out
    } else {
        // Fixed form. The fraction is `ndigits - decpt` wide, floored at 1 by
        // ADD_DOT_0 (repr(1e15) is "1000000000000000.0", one trailing zero).
        //
        // Re-rendering with `{:.n$}` instead of splicing the `{:e}` digits back
        // together is deliberate, and the one place Rust and CPython disagree:
        // on an *exact* tie Rust's shortest formatter rounds away from zero
        // while CPython's dtoa rounds to even. 2181495296738027.25 is exactly
        // representable, so `{:e}` gives "2.1814952967380273e15" — splicing
        // would print "2181495296738027.3" where Python prints
        // "...027.2". `{:.n$}` is exact and ties-to-even, so it agrees.
        // The tie moves only the last digit's value, never `ndigits` or
        // `decpt`, so reading those off `{:e}` is still sound.
        let frac = std::cmp::max(ndigits - decpt, 1) as usize;
        format!("{:.*}", frac, value)
    }
}

/// Python's `str(Decimal)` — `to-scientific-string` from the General Decimal
/// Arithmetic spec, as `_pydecimal.Decimal.__str__` implements it.
///
/// `BigDecimal` stores (coefficient, scale) with `value = coefficient *
/// 10**-scale`, so Python's `_int` is the coefficient's digits and its `_exp`
/// is `-scale`. `BigDecimal`'s own `Display` is *not* this function — it prints
/// `Decimal("1E+2")` as "100" and `Decimal("0.0")` as "0" — so the conversion is
/// spelled out rather than delegated. Checked against CPython over 3,039
/// Decimals: the only divergence is negative zero (see below).
fn py_str_decimal(value: &BigDecimal) -> String {
    let (coefficient, scale) = value.as_bigint_and_exponent();
    // `sign = ['', '-'][self._sign]`. BigInt has no negative zero, so a
    // Decimal("-0.0") arrives here as plain "0.0" where Python would say
    // "-0.0". Unobservable for AZ: to_cardinal strips the '-' off the integral
    // part and re-adds it from `value < 0`, which is False for -0 either way.
    let sign = if coefficient.is_negative() { "-" } else { "" };
    let int_digits = coefficient.abs().to_string(); // Decimal._int
    let exp = -scale; // Decimal._exp
    let ndigits = int_digits.len() as i64;
    let leftdigits = exp + ndigits;

    // `if self._exp <= 0 and leftdigits > -6: dotplace = leftdigits`
    // `elif not eng: dotplace = 1`. __str__ passes eng=False, so the
    // engineering-notation arms below it are dead.
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
    } else if dotplace >= ndigits {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - ndigits) as usize)),
            String::new(),
        )
    } else {
        let cut = dotplace as usize;
        (
            int_digits[..cut].to_string(),
            format!(".{}", &int_digits[cut..]),
        )
    };

    let exppart = if leftdigits == dotplace {
        String::new()
    } else {
        // `['e', 'E'][context.capitals] + "%+d" % (leftdigits-dotplace)`. The
        // default context has capitals=1, so 'E' — and "%+d" is *not* padded to
        // two digits, unlike repr(float)'s "%+.02d". Hence "1E+2", not "1E+02".
        let exp = leftdigits - dotplace;
        format!("E{}{}", if exp < 0 { "-" } else { "+" }, exp.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exppart)
}

/// The body of `Num2Word_AZ.to_cardinal`, given the `str(value)` it opens with
/// and the `value < 0` it closes with.
///
/// Split out because the sign cannot be recovered from the string: `str(-0.5)`
/// starts with '-' but so does nothing about `Decimal("-0.0")`'s sign, and
/// Python asks the *number*, not the text.
fn cardinal_from_py_str(value_str: &str, is_negative: bool) -> Result<String> {
    // Python: `parts = value_str.split("."); parts[0]; parts[1] if len(parts) >
    // 1 else ""`. Split-then-index, not a 2-way split — a third field would be
    // dropped rather than folded into the fraction. No str() output has two
    // dots, but splitn(2, '.') would not be the same function.
    let mut parts = value_str.split('.');
    let integral_part = parts.next().unwrap_or("");
    let fraction_part = parts.next().unwrap_or("");

    // `if integral_part.startswith("-"): integral_part = integral_part[1:]`
    let integral_part = integral_part.strip_prefix('-').unwrap_or(integral_part);

    // Python binds the integral part first, so when both halves are
    // unrenderable its error is the one that escapes.
    let integral_part_in_words = int_to_word(integral_part, false)?;
    let fraction_part_in_words = if fraction_part.is_empty() {
        String::new()
    } else {
        int_to_word(fraction_part, true)?
    };

    let mut value_in_words = integral_part_in_words;
    if !fraction_part.is_empty() {
        // `" ".join([integral_part_in_words, self.pointword, ...])`. Note the
        // bare pointword — AZ never runs it through title().
        value_in_words = format!("{} {} {}", value_in_words, POINTWORD, fraction_part_in_words);
    }
    if is_negative {
        value_in_words = format!("{} {}", NEGWORD, value_in_words);
    }
    Ok(value_in_words)
}

/// `Num2Word_AZ.CURRENCY_FORMS`, transcribed from the class body.
///
/// `CURRENCY_INTEGRAL`/`CURRENCY_FRACTION` are separate class attributes in
/// Python purely so the AZN entry can name them; they are inlined here because
/// nothing else reads them.
///
/// Every entry carries exactly two forms, and in AZ both are always identical —
/// the language does not inflect these nouns for number ("iki avro", not "iki
/// avros"). The arity is still 2 rather than 1 because `pluralize` indexes
/// `forms[1]` for every `n != 1`; collapsing to one form would turn "iki avro"
/// into an IndexError.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AZN", CurrencyForms::new(&["manat", "manat"], &["qəpik", "qəpik"]));
    m.insert("EUR", CurrencyForms::new(&["avro", "avro"], &["sent", "sent"]));
    m.insert("USD", CurrencyForms::new(&["dollar", "dollar"], &["sent", "sent"]));
    m
}

pub struct LangAz {
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangAz {
    pub fn new() -> Self {
        LangAz {
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // read this map; rebuilding it per call is what made an earlier
            // revision of this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
        }
    }
}

impl Default for LangAz {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): `POWERS_OF_TEN` ends at
/// 10^63, so 10^66 and above raise `OverflowError`.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(66))
}

impl Lang for LangAz {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
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
        POINTWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python: value_str = str(value); parts = value_str.split(".").
        // Integer input never contains ".", so integral_part is the whole
        // string and fraction_part is "" — the pointword branch is dead here.
        // (The same Python method serves float input; see to_cardinal_float,
        // which reaches the general form via cardinal_from_py_str.)
        let value_str = value.to_string();
        let integral_part = value_str.strip_prefix('-').unwrap_or(&value_str);

        let integral_part_in_words = int_to_word(integral_part, false)?;

        // `if value < 0` tests the original value, not the stripped string.
        if value.is_negative() {
            Ok(format!("{} {}", NEGWORD, integral_part_in_words))
        } else {
            Ok(integral_part_in_words)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // `assert int(value) == value` is vacuous for integer input.
        let cardinal = self.to_cardinal(value)?;
        ordinalize(cardinal)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        let cardinal = self.to_cardinal(value)?;
        let suffix = ordinal_num_suffix_for(&cardinal)?;

        // `"-".join([str(value), suffix])` — str(value) keeps the minus sign,
        // so to_ordinal_num(-1) == "-1-ci".
        Ok(format!("{}-{}", value, suffix))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        let year = self.to_cardinal(&value.abs())?;
        if value.is_negative() {
            Ok(format!("e.ə. {}", year))
        } else {
            Ok(year)
        }
    }

    // ---- float / Decimal -------------------------------------------------

    /// **Not** a port of `Num2Word_AZ.to_cardinal_float` — there is no such
    /// method. AZ inherits Base's, which is dead code for it, and overrides
    /// `to_cardinal` instead, handling non-integers inline. This hook is where
    /// the dispatcher's float input lands, so it carries that inline handling.
    ///
    /// `precision_override` (the `precision=` kwarg, issue #580) is ignored,
    /// which is not an omission: `Num2Word_AZ.to_cardinal` never reads
    /// `self.precision`, so the dispatcher's save/restore around the call
    /// changes nothing. Verified live — `num2words(3.14159, lang="az",
    /// precision=2)` and `precision=0` are both byte-identical to the plain
    /// call, all three "üç nöqtə on dörd min yüz əlli doqquz". AZ reads the
    /// fraction as a whole number, so there is no per-digit knob to turn.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `value_str = str(value)` — the whole algorithm's input. The Float and
        // Decimal arms stay apart for the reason floatpath.rs keeps them apart:
        // str(Decimal("98746251323029.99")) is exact, while the f64 nearest
        // that value reprs as "98746251323029.98" (issue #603).
        let value_str = match value {
            FloatValue::Float { value, .. } => py_str_f64(*value),
            FloatValue::Decimal { value, .. } => py_str_decimal(value),
        };
        // `if value < 0` asks the number, not the string — and it is a `<`
        // comparison, not the sign bit: -0.0 is not less than zero in Python
        // or in IEEE-754, so it keeps no negword: both num2words(-0.0) and
        // num2words(Decimal("-0.0")) are "sıfır nöqtə sıfır".
        // (`FloatValue::is_negative()` is sign-bit aware and would wrongly
        // say true for -0.0.)
        let lt_zero = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        cardinal_from_py_str(&value_str, lt_zero)
    }

    /// `to_cardinal(float/Decimal)` — the full entry, whole values included.
    ///
    /// AZ never takes a whole-value shortcut: `str(5.0)` is "5.0" and the
    /// algorithm reads the ".0" like any other fraction, so `num2words(5.0,
    /// lang="az")` is "beş nöqtə sıfır", never "beş". The trait default's
    /// `as_whole_int()` fast path is therefore wrong for AZ and bypassed.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: `assert int(value) == value` — a bare
    /// AssertionError for any fractional value (0.5, 3.25) — then the same
    /// cardinal + vowel-suffix walk as the integer path, over the cardinal of
    /// the *full* float string: `to_ordinal(5.0)` == "beş nöqtə sıfırıncı".
    ///
    /// Exponent-form input (1e16, `Decimal("1E+2")`) is whole, so it passes
    /// the assert and then dies inside `to_cardinal` with bug 4's ValueError
    /// — in that order.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        assert_whole(value)?;
        let cardinal = self.to_cardinal_float(value, None)?;
        ordinalize(cardinal)
    }

    /// `to_ordinal_num(float/Decimal)`: the assert, the cardinal (needed only
    /// for its last vowel — and for its ValueError on exponent forms), then
    /// `"-".join([str(value), suffix])`. `str(value)` is echoed verbatim, so
    /// the ".0" and -0.0's sign survive: "5.0-cı", "5.00-cı", "-0.0-cı".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        assert_whole(value)?;
        let cardinal = self.to_cardinal_float(value, None)?;
        let suffix = ordinal_num_suffix_for(&cardinal)?;
        Ok(format!("{}-{}", repr_str, suffix))
    }

    /// `to_year(float/Decimal)`: the assert, then `year =
    /// self.to_cardinal(abs(value))` — the cardinal of the *absolute* value,
    /// so the negword never appears — and `if value < 0` (numeric, not the
    /// sign bit) picks the "e.ə." era prefix. -0.0 is not < 0, so
    /// `to_year(-0.0)` is a plain "sıfır nöqtə sıfır".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        assert_whole(value)?;
        let (value_str, lt_zero) = match value {
            FloatValue::Float { value: f, .. } => (py_str_f64(f.abs()), *f < 0.0),
            FloatValue::Decimal { value: d, .. } => (py_str_decimal(&d.abs()), d.is_negative()),
        };
        let year = cardinal_from_py_str(&value_str, false)?;
        if lt_zero {
            Ok(format!("e.ə. {}", year))
        } else {
            Ok(year)
        }
    }

    /// Base `str_to_number` (`Decimal(value)`), with one AZ-visible fixup:
    /// `Decimal("Infinity")` parses fine in Python, and AZ's `to_cardinal`
    /// then walks `str()` == "Infinity" *reversed* ("ytinifnI"), so the first
    /// `int()` call is `int("y")` — `ValueError: invalid literal for int()
    /// with base 10: 'y'`, not the OverflowError of base's
    /// `int(Decimal('Infinity'))`. "-Infinity" strips its sign first and
    /// names the same 'y'. The shared Inf sentinel maps to OverflowError in
    /// the binding, so the ValueError is raised here instead.
    ///
    /// (For to="ordinal"/"ordinal_num"/"year" Python *would* OverflowError
    /// inside `assert int(value) == value` before to_cardinal runs, but only
    /// cardinal rows exist in the corpus and the mode is not visible from
    /// this hook — the RU port makes the same call.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'y'".into(),
            )),
            other => Ok(other),
        }
    }

    /// The fractional-cents branch of `Num2Word_Base.to_currency`, which Python
    /// reaches as `self.to_cardinal(float(right))` — AZ's `to_cardinal`, not
    /// Base's `to_cardinal_float`. The trait default routes to
    /// `floatpath::cardinal_from_bigdecimal`, i.e. Base's digit-by-digit
    /// rendering, which AZ does not inherit and which differs as soon as the
    /// leftover cents carry more than one digit: at 1.0655 USD (65.5 cents)
    /// Python says "altı nöqtə əlli beş sent" — "six point fifty-five" — where
    /// Base's path would say "altı nöqtə beş beş".
    ///
    /// The `float(right)` cast is reproduced rather than avoided: it is what
    /// collapses Decimal("6.5500") to "6.55" before str() ever sees it.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let float = value.to_f64().ok_or_else(|| {
            N2WError::Value(format!("cannot represent {} as f64", value))
        })?;
        cardinal_from_py_str(&py_str_f64(float), float < 0.0)
    }

    // ---- currency -------------------------------------------------------
    //
    // AZ defines only `CURRENCY_FORMS` and `pluralize`; `to_currency`,
    // `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` are
    // inherited from `Num2Word_Base` verbatim, and the trait defaults already
    // reproduce them. `currency_adjective`/`currency_precision` are left at
    // their defaults because AZ's are Base's still-empty dicts (see module
    // docs), so `None` and `100` are the right answers for every code.

    fn lang_name(&self) -> &str {
        "Num2Word_AZ"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        // `None` becomes Python's `KeyError` -> the `NotImplementedError` that
        // `to_currency`/`to_cheque` re-raise. AZ knows only AZN/EUR/USD, so
        // this is the live path for every other code in the corpus.
        self.currency_forms.get(code)
    }

    /// `Num2Word_AZ.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Textually identical to `Num2Word_EUR.pluralize`, but arrived at
    /// independently — AZ does not inherit from EUR.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. All three AZ entries have two forms, so that is
    /// unreachable; it is mapped to `Index` rather than panicking so the
    /// exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }
}
