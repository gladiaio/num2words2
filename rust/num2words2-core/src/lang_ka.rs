//! Port of `lang_KA.py` (Georgian).
//!
//! Registry check: `CONVERTER_CLASSES["ka"] == lang_KA.Num2Word_KA()`
//! (`__init__.py:343`), so this file ports `Num2Word_KA` — the only class the
//! module defines.
//!
//! Shape: **self-contained**. `Num2Word_KA` subclasses `Num2Word_Base` but its
//! `setup()` defines none of `high_numwords`/`mid_numwords`/`low_numwords`.
//! `Num2Word_Base.__init__` guards the card-building block with
//! `if any(hasattr(self, field) for field in [...])`, so Python never builds
//! `self.cards` and **never sets `self.MAXVAL`**. All four in-scope methods are
//! overridden by KA, so `cards`/`maxval`/`merge` stay at their trait defaults
//! here and are never consulted. There is no overflow check at all — see bug 3.
//!
//! Nothing is inherited from `Num2Word_Base` in scope: KA overrides
//! `to_cardinal`, `to_ordinal`, `to_ordinal_num` and `to_year`. In particular
//! `verify_ordinal` is never called, which is why negatives and zero sail
//! straight through `to_ordinal` instead of raising `TypeError` (bug 4).
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is wrong-looking but is
//! exactly what Python emits, verified against `bench/corpus.jsonl`:
//!
//! 1. **Composite tens are not composed.** Georgian is vigesimal and writes 21
//!    as one word, "ოცდაერთი". `_int_to_word` instead concatenates the table
//!    entries with a space: `to_cardinal(21)` == "ოცი ერთი" ("twenty one"),
//!    `to_cardinal(31)` == "ოცდაათი ერთი", `to_cardinal(99)` ==
//!    "ოთხმოცდაათი ცხრა". Note `tens[3]` == "ოცდაათი" already *means* thirty
//!    (literally "twenty-and-ten"), so the composed forms read oddly.
//! 2. **Hundreds carry a redundant "one".** The `< 1000` arm is unconditionally
//!    `ones[n // 100] + " " + hundred`, with no `n // 100 == 1` special case, so
//!    `to_cardinal(100)` == "ერთი ასი" ("one hundred") rather than plain "ასი".
//!    The same shape appears at every scale: `to_cardinal(1000)` ==
//!    "ერთი ათასი", `to_cardinal(10**6)` == "ერთი მილიონი".
//! 3. **No words above 10^12 — and no `OverflowError`.** The final `else` of
//!    `_int_to_word` is `return str(number)`, a bare digit fallback. So
//!    `to_cardinal(10**12)` == "1000000000000" (a numeral, not words) and
//!    `to_ordinal(10**21)` == "მე-1000000000000000000000". Because `MAXVAL` was
//!    never set (see above), the base class's overflow guard is unreachable and
//!    arbitrarily large input silently returns its own digits. This is why
//!    [`int_to_word`] is infallible: there is no error path.
//! 4. **`to_ordinal` prefixes the minus phrase.** It only special-cases 1..=10
//!    and otherwise returns `"მე-" + to_cardinal(number)`, with no
//!    `verify_ordinal` call. Hence `to_ordinal(0)` == "მე-ნული" ("th-zero") and
//!    `to_ordinal(-1)` == "მე-მინუს ერთი" ("th-minus one") — the ordinal prefix
//!    lands on the negation word. Both are corpus-confirmed, not speculation.
//! 5. **`to_year` emits English.** `to_year(-44)` == "BC ორმოცი ოთხი` — the
//!    literal ASCII "BC " is prepended in a Georgian module. The positive arm is
//!    the no-op `return "" + self.to_cardinal(val)`.
//! 6. `tens[1]` ("ათი") is dead: the `number < 20` arm catches 10 first and
//!    returns `teens[0]`, which is the same word. Kept in the table for
//!    index alignment.
//! 7. `to_year`'s `longval=True` parameter is accepted and never read. The Rust
//!    fast path in `num2words()` only fires when no kwargs are passed, so the
//!    trait's single-argument `to_year` covers the dispatched surface exactly.
//!
//! ## Currency quirks (phase 2)
//!
//! 8. **An unknown currency code silently prints lari.** `to_currency` looks the
//!    code up with `self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["GEL"])`,
//!    so `currency:GBP`, `currency:JPY`, `currency:KWD` … all render Georgian
//!    lari/tetri instead of raising. Only GEL/USD/EUR exist. This fallback is
//!    **local to `to_currency`**: the inherited `to_cheque` subscripts
//!    `CURRENCY_FORMS[currency]` and converts the `KeyError` into
//!    `NotImplementedError`, which is why `cheque:GBP` raises while
//!    `currency:GBP` quietly prints lari. Both halves are corpus-confirmed.
//! 9. **`CURRENCY_PRECISION` is empty, so every code is 100 subunits.** KA never
//!    defines it and `to_currency` never consults it — the fractional digits are
//!    sliced out of the decimal *string*, hard-coded to two places. So JPY
//!    (0-decimal) and KWD/BHD (3-decimal) get cents anyway: `currency:JPY` on
//!    `12.34` is "თორმეტი ლარი ოცდაათი ოთხი თეთრი", not a rounded whole unit.
//!    The trait's `currency_precision` default of 100 is exactly
//!    `CURRENCY_PRECISION.get(code, 100)` against an empty dict, so it is left
//!    alone — the inherited `to_cheque` reads it and wants 100.
//! 10. **A float with zero cents drops the cents segment.** `right` is
//!    `int(parts[1][:2].ljust(2, "0"))`, so `1.0` gives `int("00")` == 0, and the
//!    guard is `if cents and right:` — an *int* truthiness test. So `1.0` prints
//!    "ერთი euro", identical to the int `1`. This inverts `Num2Word_Base`, whose
//!    whole point is that a float renders cents even when they are zero. KA does
//!    not call `base.to_currency` at all, so the `isinstance(val, int)` branch
//!    never runs and the int/float distinction collapses *for this language*.
//! 11. **`[:2]` truncates rather than rounds.** `0.005` -> `parts[1][:2]` == "00"
//!    -> 0 cents; `12.349` -> "34" -> 34 cents. No ROUND_HALF_UP anywhere.
//!    Conversely `ljust` pads on the right, so `0.5` is *fifty* cents, not five.
//! 12. **`cents=False` drops the cents entirely.** There is no `_cents_terse`
//!    fallback — the guard is `if cents and right:`, so `cents=False` prints only
//!    the unit. Base would have printed "12 euros, 34" instead.
//! 13. `adjective` is accepted and never read; `CURRENCY_ADJECTIVES` is empty.
//!
//! # The float/Decimal path is a *string* algorithm
//!
//! `Num2Word_KA.to_cardinal` opens with `n = str(number).strip()` and branches
//! on `"." in n`. It never calls `Num2Word_Base.to_cardinal_float`, never calls
//! `float2tuple`, and never reads `self.precision`, so none of [`crate::floatpath`]
//! applies. The fractional digits spoken are literally the characters after the
//! `.` in `str(value)`, each one run back through `_int_to_word`. See the note
//! on [`LangKa::to_cardinal_float`] and [`cardinal_from_str`]. The two classic
//! f64-artefact cases (`1.005`, `2.675`) come out right, but via `repr`'s
//! shortest round-trip rather than `float2tuple`'s `< 0.01` rescue.
//!
//! # Error variants
//!
//! For integer input every in-scope method is total: `str(number).strip()` of an
//! int never contains "." (so the `pointword` split is unreachable), `int(n)` on
//! the resulting digit string never raises, and the `_int_to_word` ladder
//! terminates in the `str(number)` fallback rather than an exception.
//!
//! The float/Decimal path adds one, from the `int()` calls `to_cardinal` makes
//! on the pieces of `str(number)`:
//!
//! * `Value` (`ValueError`) whenever `str(value)` is not a plain decimal
//!   literal — exponent notation, `inf`, `nan`. `str(float)` switches to
//!   exponent form at `|v| >= 1e16` and `0 < |v| < 1e-4`, so `1e16` raises
//!   `invalid literal for int() with base 10: '1e+16'` (no `.`, the whole string
//!   reaches `int()`) while `1.5e16` raises on the fragment `'e'` (the `.`
//!   splits it, so the digit loop hits `int("e")` first). Same for `str(Decimal)`
//!   in its uppercase-`E` exponent form.
//!
//! The currency surface adds two:
//!
//! * `NotImplemented` — from the **inherited** `to_cheque` only, for a code
//!   outside GEL/USD/EUR. `to_currency` never raises it (quirk 8).
//! * `Value` — `to_currency` runs `int()` over `str(val).split(".")[0]`, so a
//!   float large enough for Python to `repr` in scientific notation feeds
//!   `int("1e+16")` and raises `ValueError`. See the note on [`LangKa::to_currency`].
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD and EUR use დოლარი / ევრო with ცენტი (the noun stays singular
//! after a numeral). Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`. Note the **trailing space** — it is part of the constant,
/// not added at the call site.
const NEGWORD: &str = "მინუს ";

/// `self.pointword`. Reached on the float/Decimal path via
/// [`cardinal_from_str`]. Unlike `Num2Word_Base.to_cardinal_float`, which
/// pushes `self.title(self.pointword)` as its own list element, `Num2Word_KA`
/// concatenates `" " + pointword + " "` by hand and never calls `title()`.
/// Identical here (`is_title` is false), but reproduced as written.
const POINTWORD: &str = "წერტილი";

/// `self.zero`.
const ZERO_WORD: &str = "ნული";

/// `self.ones`; index 0 is "" and is only ever reached via `number == 0`,
/// which returns `zero` first.
const ONES: [&str; 10] = [
    "",
    "ერთი",
    "ორი",
    "სამი",
    "ოთხი",
    "ხუთი",
    "ექვსი",
    "შვიდი",
    "რვა",
    "ცხრა",
];

/// `self.tens`, indexed by the tens digit. Index 0 is unreachable (the
/// `< 20` arm handles 10..19) and so is index 1 — see bug 6.
const TENS: [&str; 10] = [
    "",
    "ათი",
    "ოცი",
    "ოცდაათი",
    "ორმოცი",
    "ორმოცდაათი",
    "სამოცი",
    "სამოცდაათი",
    "ოთხმოცი",
    "ოთხმოცდაათი",
];

/// `self.teens`, indexed by `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "ათი",
    "თერთმეტი",
    "თორმეტი",
    "ცამეტი",
    "თოთხმეტი",
    "თხუთმეტი",
    "თექვსმეტი",
    "ჩვიდმეტი",
    "თვრამეტი",
    "ცხრამეტი",
];

const HUNDRED: &str = "ასი";
const THOUSAND: &str = "ათასი";
const MILLION: &str = "მილიონი";
const BILLION: &str = "მილიარდი";

/// `Num2Word_KA.to_currency`'s own default `separator=" "`, confirmed against
/// the interpreter: `Num2Word_KA.to_currency.__defaults__` is
/// `("GEL", True, " ", False)`.
const DEFAULT_SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default `separator=","`.
///
/// The dispatcher (`__init__.py:546`) sends `kwargs.get("separator", ",")` down
/// the Rust fast path — *base's* default, not KA's — so a caller who says
/// nothing about the separator is indistinguishable from one who explicitly
/// asked for `","`. `to_currency` reads this value as the "caller said nothing"
/// sentinel it is and substitutes [`DEFAULT_SEPARATOR`]; see the note on
/// [`LangKa::to_currency`]. `lang_br.rs` and `lang_sa.rs` take the same
/// approach for the same reason.
const BASE_DEFAULT_SEPARATOR: &str = ",";

/// The code `to_currency` falls back to for an unknown currency (quirk 8).
const FALLBACK_CURRENCY: &str = "GEL";

/// The `to_ordinal` special cases for 1..=10, indexed by `number - 1`.
/// Every other input (including 0 and negatives) takes the `მე-` + cardinal
/// path — see bug 4.
const ORDINALS: [&str; 10] = [
    "პირველი", // 1
    "მეორე",   // 2
    "მესამე",  // 3
    "მეოთხე",  // 4
    "მეხუთე",  // 5
    "მეექვსე", // 6
    "მეშვიდე", // 7
    "მერვე",   // 8
    "მეცხრე",  // 9
    "მეათე",   // 10
];

/// `to_ordinal`'s fallback prefix for everything outside 1..=10.
const ORDINAL_PREFIX: &str = "მე-";

/// Port of `Num2Word_KA._int_to_word`.
///
/// Infallible: the ladder's final `else` is `return str(number)`, so there is
/// no input that escapes without a string (bug 3).
///
/// Python uses `//` and `%` (floor semantics). Every arm below `number < 0`
/// only ever sees a positive value — the negative arm recurses on `abs(number)`
/// first — but `div_mod_floor` is used anyway to keep Python's semantics rather
/// than Rust's truncating `/`.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // Dead code for the in-scope entry points: to_cardinal strips the sign
    // before calling, to_year takes abs(), and to_ordinal delegates to
    // to_cardinal. Ported for fidelity — and it agrees with the to_cardinal
    // path anyway, since negword carries its own trailing space.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1_000);
    let million = BigInt::from(1_000_000);
    let billion = BigInt::from(1_000_000_000);
    let trillion = BigInt::from(1_000_000_000_000u64);

    // `number < 10` -> ones[number]
    if number < &ten {
        // 1..=9, so the unwrap is total.
        return ONES[number.to_usize().unwrap()].to_string();
    }

    // `number < 20` -> teens[number - 10]
    if number < &BigInt::from(20) {
        return TEENS[(number - &ten).to_usize().unwrap()].to_string();
    }

    // `number < 100` -> tens[number // 10] (+ " " + ones[number % 10])
    if number < &hundred {
        let (div, rem) = number.div_mod_floor(&ten);
        // div is 2..=9.
        let mut result = TENS[div.to_usize().unwrap()].to_string();
        if !rem.is_zero() {
            result.push(' ');
            result.push_str(ONES[rem.to_usize().unwrap()]);
        }
        return result;
    }

    // `number < 1000` -> ones[number // 100] + " " + hundred (+ recurse)
    // NB: no `div == 1` special case -> "ერთი ასი" for 100 (bug 2).
    if number < &thousand {
        let (div, rem) = number.div_mod_floor(&hundred);
        // div is 1..=9.
        let mut result = format!("{} {}", ONES[div.to_usize().unwrap()], HUNDRED);
        if !rem.is_zero() {
            result.push(' ');
            result.push_str(&int_to_word(&rem));
        }
        return result;
    }

    // The three remaining scales share one shape: recurse on the quotient,
    // append the scale word, then recurse on the remainder if non-zero.
    // Written out rather than looped to keep the correspondence with the
    // Python elif-ladder one-to-one.
    if number < &million {
        return scale(number, &thousand, THOUSAND);
    }
    if number < &billion {
        return scale(number, &million, MILLION);
    }
    if number < &trillion {
        return scale(number, &billion, BILLION);
    }

    // `else: return str(number)` -- the bare-numeral fallback (bug 3).
    number.to_string()
}

/// The shared body of the thousand/million/billion arms:
/// `_int_to_word(n // unit) + " " + word` then `+ " " + _int_to_word(n % unit)`
/// when the remainder is non-zero.
fn scale(number: &BigInt, unit: &BigInt, word: &str) -> String {
    let (div, rem) = number.div_mod_floor(unit);
    let mut result = format!("{} {}", int_to_word(&div), word);
    if !rem.is_zero() {
        result.push(' ');
        result.push_str(&int_to_word(&rem));
    }
    result
}

// ---- float / Decimal path ----------------------------------------------
//
// `Num2Word_KA.to_cardinal` opens with `n = str(number).strip()` and branches
// on `"." in n`. It never calls `Num2Word_Base.to_cardinal_float`, never calls
// `float2tuple`, and never reads `self.precision`. So none of the machinery in
// [`crate::floatpath`] applies: no `pre`/`post` split, no `10**precision`
// scaling, no `abs(round(post) - post) < 0.01` heuristic, and no banker's
// rounding. The fractional digits spoken are literally the characters after the
// `.` in `str(value)`.
//
// So the whole float path reduces to reproducing Python's `str()`:
// [`py_float_repr`] for `float`, [`py_decimal_str`] for `Decimal`, then feed the
// result through [`cardinal_from_str`] — the exact body of `to_cardinal`. The
// two classic f64-artefact cases (`1.005`, `2.675`) still come out right, but
// for a different reason than the base path: `repr(2.675)` is the shortest
// string that round-trips (`"2.675"`), so the `674.9999999999998` that
// `float2tuple` exists to rescue is never computed here.
//
// These four helpers (`shortest_digits`/`py_float_repr`/`py_decimal_str`/
// `py_int`) are the same language-agnostic reproductions used by `lang_as.rs`,
// `lang_ba.rs`, `lang_eu.rs` and the other string-path modules.

/// The shortest round-tripping decimal digits of `a` (finite, non-negative),
/// plus `decpt`, the decimal-point position such that
/// `a == 0.<digits> * 10**decpt`. This is CPython's `_Py_dg_dtoa(a, 0, 0, ...)`
/// — David Gay's `dtoa` in mode 0.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with Gay on the digit
/// *count* and almost always the digits. It disagrees on **exact ties**: when
/// `a` sits precisely halfway between the two shortest candidates, both
/// round-trip, and Gay's dtoa takes the one with an **even** last digit while
/// Rust rounds half **up**. `repr(-78198386800398.125)` is
/// `'-78198386800398.12'`; Rust's `{:e}` says `...13`. The block below detects
/// that tie with no bignum and corrects it.
///
/// Write `a = m * 2**e` with `m` odd, `q = digits.len() - decpt`. The tie
/// condition `a * 10**q == k + 1/2` reduces to `e + q + 1 == 0`, and when
/// `q < 0` additionally `5**-q | m` (with `-q <= 22`, since `5**23 > 2**53`).
/// Then `2k + 1 == m * 5**q` (or `m / 5**-q`), and because `5 ≡ 1 (mod 4)` that
/// odd integer is `≡ m (mod 4)`, so `k` is even exactly when `m % 4 == 1`.
fn shortest_digits(a: f64) -> (String, i32) {
    // "d[.ddd]e<exp>", shortest round-trip.
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
            // k even: Python wants k, Rust gave k+1. Last digit odd => nonzero.
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
        // dtoa never emits trailing zeros; stripping them leaves decpt alone.
        while digits.len() > 1 && *digits.last().expect("non-empty") == b'0' {
            digits.pop();
        }
    }
    (String::from_utf8(digits).expect("ASCII digits"), decpt)
}

/// Python's `str(float)` (== `repr(float)`), which `Num2Word_KA.to_cardinal`
/// promotes from a formatting detail to *the entire specification* of the float
/// path. This is CPython's `format_float_short(..., 'r', ...)` in `pystrtod.c`.
///
/// Rust's own `{}` cannot stand in: it never switches to exponent notation
/// (`format!("{}", 1e16_f64)` is `"10000000000000000"`, where Python says
/// `'1e+16'` — and that difference is why `to_cardinal(1e16)` raises
/// `ValueError`) and it prints `1`, not `1.0`, for integral floats.
///
/// Rules from `format_float_short`:
/// * exponent notation iff `decpt <= -4 || decpt > 16`;
/// * the exponent is `%+.02d`: always signed, zero-padded to two digits;
/// * `Py_DTSF_ADD_DOT_0` appends `.0` to an integral fixed-notation result, but
///   never in exponent notation;
/// * `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and to_cardinal
    // strips that minus textually into a negword.
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
        // 0.5 -> decpt 0 -> "0." + "" + "5"; 0.01 -> decpt -1 -> "0." + "0" + "1".
        format!("{}0.{}{}", sign, "0".repeat(-decpt as usize), digits)
    } else if decpt >= ndigits {
        // Integral: pad right with zeros, then ADD_DOT_0. 1.0 -> "1" + ".0".
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

/// Python's `str(Decimal)` — `_pydecimal.Decimal.__str__` with `eng=False` and
/// the default context (`capitals=1`, hence an uppercase `E`).
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`: the shim builds this value with
/// `BigDecimal::from_str(str(value))`, and that parse preserves trailing zeros
/// and negative exponents rather than normalising, so `"1.10"` round-trips as
/// `(110, 2)` and `"1E+16"` as `(1, -16)`.
///
/// # The negative-zero hole
///
/// `Decimal` carries `_sign` independently of `_int`, so `Decimal("-0.0")` is
/// signed zero and `str()` gives `'-0.0'`; a `BigDecimal` cannot represent it
/// (`int_val` is a `BigInt`, no negative zero), so `BigDecimal::from_str("-0.0")`
/// has already discarded the sign. We emit `'0.0'` and drop the negword. The
/// discriminator is the original string, which the `FloatValue::Decimal`
/// boundary does not carry — flagged in the port report. (The `float` arm has
/// no such hole: `-0.0_f64` keeps its sign bit, so `py_float_repr(-0.0)` is
/// `"-0.0"` and matches Python.)
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
        // "%+d" — signed, but *not* zero-padded, unlike repr(float)'s "%+.02d".
        let d = leftdigits - dotplace;
        format!("E{}{}", if d < 0 { '-' } else { '+' }, d.abs())
    };

    format!("{}{}{}{}", sign, intpart, fracpart, expstr)
}

/// Python's `int(s)`, for the fragments [`cardinal_from_str`] hands it. Every
/// string that reaches here is a slice of `str(float)` / `str(Decimal)`, i.e.
/// plain ASCII, so the extra generality of the real builtin (non-ASCII digits,
/// exotic whitespace) is deliberately not ported. What is ported is the
/// underscore rule and the error message: Python formats the **original,
/// unstripped** argument with `%.200R` (`repr(s)`), which for the ASCII literals
/// here is exactly `'{}'`.
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
    // int() permits '_' as a digit separator, but not leading/trailing/doubled.
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

/// The full body of `Num2Word_KA.to_cardinal`, driven by `str(number)`:
///
/// ```text
/// n = str(number).strip()
/// if n.startswith("-"):
///     n = n[1:]
///     ret = self.negword          # "მინუს "
/// else:
///     ret = ""
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret += self._int_to_word(int(left)) + " " + self.pointword + " "
///     ret += " ".join(self._int_to_word(int(d)) for d in right)
///     return ret.strip()
/// else:
///     return (ret + self._int_to_word(int(n))).strip()
/// ```
///
/// Three details that matter:
///
/// * The sign is stripped **textually**, so `-0.0` (whose `str` is `"-0.0"`)
///   keeps its negword even though the value is not `< 0`: KA answers
///   "მინუს ნული წერტილი ნული". The `float` arm reproduces this; the `Decimal`
///   arm cannot (see [`py_decimal_str`]).
/// * `split(".", 1)` caps at one split, so a second dot stays inside `right`
///   and detonates in the digit loop rather than being ignored.
/// * `int(left)` runs *before* the digit loop, so for `1.5e+16` the failing
///   literal reported is `'e'`, not `'1.5e+16'`. Order is load-bearing for the
///   error message; keep it.
fn cardinal_from_str(number: &str) -> Result<String> {
    // str(number).strip() — a no-op for real repr()s, applied for fidelity.
    let n = number.trim();
    let (n, mut ret) = match n.strip_prefix('-') {
        Some(rest) => (rest, NEGWORD.to_string()),
        None => (n, String::new()),
    };

    let Some(dot) = n.find('.') else {
        // else: return (ret + _int_to_word(int(n))).strip()
        ret.push_str(&int_to_word(&py_int(n)?));
        return Ok(ret.trim().to_string());
    };

    // n.split(".", 1) — maxsplit=1, so `right` keeps any further dots. '.' is
    // ASCII and `left` is ASCII digits, so byte slicing lands on char bounds.
    let (left, right) = (&n[..dot], &n[dot + 1..]);
    ret.push_str(&int_to_word(&py_int(left)?));
    ret.push(' ');
    ret.push_str(POINTWORD);
    ret.push(' ');

    // " ".join(self._int_to_word(int(d)) for d in right) — Python iterates
    // *characters*, so index by chars(), never bytes.
    let mut first = true;
    for d in right.chars() {
        if !first {
            ret.push(' ');
        }
        first = false;
        let mut buf = [0u8; 4];
        ret.push_str(&int_to_word(&py_int(d.encode_utf8(&mut buf))?));
    }
    // return ret.strip()
    Ok(ret.trim().to_string())
}

pub struct LangKa {
    /// `Num2Word_KA.CURRENCY_FORMS`. Built once in [`LangKa::new`] and only read
    /// thereafter — the generated registry parks each language in a `OnceLock`
    /// and calls `new` through `get_or_init`, so this table is constructed a
    /// single time per process rather than per conversion.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `self.CURRENCY_FORMS["GEL"]` — the default `to_currency`'s `.get` falls
    /// back to for an unknown code (quirk 8).
    ///
    /// Python spells the fallback as an explicit `CURRENCY_FORMS["GEL"]`
    /// subscript, so — unlike the sibling languages that write
    /// `list(CURRENCY_FORMS.values())[0]` — there is no dict-ordering question
    /// to answer here. Held as its own field so the lookup cannot panic; the
    /// duplicate is two short string pairs.
    ///
    /// Note that Python evaluates this subscript *eagerly on every call*, as
    /// `dict.get`'s second argument, even when `currency` is a hit. GEL is a
    /// class constant and always present, so that has no observable effect.
    fallback_forms: CurrencyForms,
}

impl Default for LangKa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKa {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::new();
        // Arity is load-bearing: `to_currency` indexes `cr1[0]`/`cr1[1]` and
        // `cr2[0]`/`cr2[1]`, and the inherited `to_cheque` takes `cr1[-1]`, so
        // both forms of both tuples must survive verbatim.
        //
        // GEL's singular and plural are the *same word* — Georgian does not
        // inflect the noun after a numeral. That is not a transcription slip:
        // `("ლარი", "ლარი")` and `("თეთრი", "თეთრი")` are what the source says,
        // so the `left != 1` test below is a no-op for GEL and a real choice
        // only for USD/EUR.
        currency_forms.insert(
            "GEL",
            CurrencyForms::new(&["ლარი", "ლარი"], &["თეთრი", "თეთრი"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["დოლარი", "დოლარი"], &["ცენტი", "ცენტი"]),
        );
        // KA declares CURRENCY_FORMS on its own class, so `Num2Word_EN.__init__`
        // mutating `Num2Word_EUR`'s shared class dict never reaches it: EUR here
        // is KA's own literal, and the ~24 codes EN adds to that shared dict are
        // absent (which is why they fall back to GEL — quirk 8 — rather than
        // resolving). Verified against the live interpreter, not the source.
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["ევრო", "ევრო"], &["ცენტი", "ცენტი"]),
        );
        let fallback_forms = currency_forms
            .get(FALLBACK_CURRENCY)
            .expect("GEL is inserted above")
            .clone();
        LangKa {
            currency_forms,
            fallback_forms,
        }
    }
}

impl Lang for LangKa {

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
        "GEL"
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
        "წერტილი"
    }

    /// Port of `Num2Word_KA.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).strip()`, detaches a leading "-" into
    /// `ret = self.negword`, then checks for "." — `str(int)` never contains
    /// one, so integers always take the `else` branch:
    /// `return (ret + self._int_to_word(int(n))).strip()`.
    ///
    /// The trailing `.strip()` is a no-op here (negword's trailing space is
    /// interior once the word is appended, and `_int_to_word` never returns
    /// padded output), but it is applied for fidelity.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", ret, int_to_word(&n)).trim().to_string())
    }

    /// Port of `Num2Word_KA.to_ordinal`.
    ///
    /// 1..=10 have suppletive forms; everything else — including 0 and every
    /// negative — gets `"მე-" + to_cardinal(number)` (bug 4). No
    /// `verify_ordinal`, so nothing raises.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // to_u32 rejects negatives and anything huge, which is exactly the set
        // the Python `elif` chain falls through on.
        if let Some(n) = value.to_u32() {
            if (1..=10).contains(&n) {
                return Ok(ORDINALS[(n - 1) as usize].to_string());
            }
        }
        Ok(format!("{}{}", ORDINAL_PREFIX, self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_KA.to_ordinal_num`: `str(number) + "."`.
    /// Negatives keep their sign: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_KA.to_year`.
    ///
    /// Negative years get the ASCII prefix "BC " and the *absolute* value's
    /// cardinal, so no negword appears (bug 5). The positive arm is Python's
    /// no-op `"" + self.to_cardinal(val)`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            Ok(format!("BC {}", self.to_cardinal(&value.abs())?))
        } else {
            self.to_cardinal(value)
        }
    }

    /// `to_ordinal(float/Decimal)`: the `number == 1 ... == 10` chain matches
    /// numerically, so whole values 1..=10 (5.0, Decimal("5.00")) take the
    /// suppletive forms; everything else — negatives and fractions included —
    /// is `"მე-" + to_cardinal(number)` with the value's own str() grammar
    /// ("მე-ერთი ასი წერტილი ნული" for 100.0).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i.is_positive() && i <= BigInt::from(10) {
                return self.to_ordinal(&i);
            }
        }
        Ok(format!("მე-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."` — Python's str,
    /// handed in as `repr_str` ("5.0.", "-0.0.", "1E+2.").
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `if val < 0` (numeric — Decimal("-0.0") is
    /// not < 0) → `"BC " + to_cardinal(abs(val))`, keeping the float grammar
    /// ("BC ერთი წერტილი ხუთი" for -1.5).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let negative = match value {
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
            Ok(format!("BC {}", self.cardinal_float_entry(&abs, None)?))
        } else {
            self.cardinal_float_entry(value, None)
        }
    }

    /// `str_to_number` is Base's `Decimal(value)`, but KA's `to_cardinal`
    /// then runs `int()` over the dot-free `str()` form, so `"Infinity"`
    /// raises **ValueError** (`int('Infinity')`) rather than the shared Inf
    /// sentinel's OverflowError. No digit present → the dispatcher
    /// propagates it, exactly like the original.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    /// The `"." in n` branch of `Num2Word_KA.to_cardinal` — the same method as
    /// [`LangKa::to_cardinal`]. Python has one `to_cardinal` that splits on
    /// `str(number)`, so the trait's int/float boundary lands mid-method here
    /// rather than at a class boundary.
    ///
    /// **`Num2Word_Base.to_cardinal_float` is never reached for Georgian**, and
    /// neither is `float2tuple`. Consequences that invert the usual advice:
    ///
    /// * There is no `precision` anywhere. `num2words(..., precision=1)` sets
    ///   `converter.precision`, but `Num2Word_KA.to_cardinal` never reads it —
    ///   verified live, `num2words(1.23456, lang="ka", precision=1)` is
    ///   unchanged at five fractional words. So `precision_override` is accepted
    ///   and dropped, the same shape as `to_year`'s `longval` (bug 7) and
    ///   `to_currency`'s `adjective` (quirk 13).
    /// * `FloatValue::precision` is likewise unused: the digit count comes from
    ///   `str()`, not from a precision field. Every digit after the point is
    ///   spoken separately with no cap, so `0.000001` is six words and the
    ///   Decimal `98746251323029.99` keeps full precision for free (issue
    ///   #603's `float()` cast never happens — the Decimal arm stringifies).
    ///
    /// The `< 0.01` artefact heuristic is absent and *not* missing: `repr(2.675)`
    /// is `"2.675"`, so the `674.9999999999998` it exists to repair is never
    /// computed. `1.005` and `2.675` come out right by a different mechanism.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // precision= is set on the converter by __init__.py, then ignored by
        // Num2Word_KA.to_cardinal. Reproduce the ignoring.
        let _ = precision_override;
        let n = match value {
            // Python's str(float). The raw f64 crosses the boundary precisely so
            // that repr() can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, and never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    // ---- currency ------------------------------------------------------
    //
    // Only the three hooks below are overridden. KA defines no
    // CURRENCY_ADJECTIVES and no CURRENCY_PRECISION, and leaves `pluralize`,
    // `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` at
    // `Num2Word_Base` — whose behaviour the trait defaults already reproduce.
    // In particular `pluralize` stays at the default that raises: KA's
    // `to_currency` picks its forms with an inline `left != 1` test and never
    // calls it, and the inherited `to_cheque` does not call it either.

    /// `self.__class__.__name__`, for the inherited `to_cheque`'s
    /// `NotImplementedError` message.
    fn lang_name(&self) -> &str {
        "Num2Word_KA"
    }

    /// `CURRENCY_FORMS[code]` — a plain lookup that misses for anything but
    /// GEL/USD/EUR.
    ///
    /// Deliberately **not** the `.get(code, CURRENCY_FORMS["GEL"])` fallback of
    /// quirk 8: that fallback is local to `to_currency`. The inherited
    /// `to_cheque` subscripts `CURRENCY_FORMS[currency]` and catches `KeyError`,
    /// so it must still see a miss here — that is what makes `cheque:GBP` raise
    /// `NotImplementedError` while `currency:GBP` quietly prints lari.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_KA.to_currency`.
    ///
    /// KA ignores `base.to_currency` wholesale — no `parse_currency_parts`, no
    /// `divisor`, no `pluralize`, no `isinstance(val, int)` branch. It slices the
    /// decimal *string*:
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// The int/float split still arrives intact and is still honoured — it is
    /// simply carried by the string rather than by a type test. `str(1)` is "1"
    /// (no dot, so `right` is 0) and `str(1.0)` is "1.0" (a dot, but `int("00")`
    /// is also 0), so for *this* language both print "ერთი euro". Collapsing
    /// `CurrencyValue` would nonetheless be wrong: it is what produces the "1.0"
    /// spelling in the first place.
    ///
    /// # The `separator` argument is not what Python's default would be
    ///
    /// KA's own signature defaults to `separator=" "`, but the trait cannot carry
    /// a per-language default argument: the dispatcher resolves it before the
    /// call and uses `Num2Word_Base`'s `","`. Since the frozen corpus was
    /// generated through Python — where KA's `" "` applies — `","` is read back
    /// as the "unset" sentinel it is. This is exact both for a caller who omits
    /// `separator` and for one who passes anything other than `","`, and wrong
    /// only for an explicit `separator=","`, which Python renders with a comma
    /// and this renders with a space. See `concerns`.
    ///
    /// # `Value` on scientific notation
    ///
    /// `int(parts[0])` inherits Python's rejection of `repr`-style exponents, so
    /// `to_currency(1e16)` raises `ValueError`. `BigDecimal`'s `Display` keeps
    /// the `1e+16` form for a large exponent, so `BigInt::from_str` rejects it
    /// identically. It does *not* keep the `1e-05` form (it prints `0.00001`),
    /// so a tiny float diverges — flagged in `concerns`; no corpus row covers it.
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
        // Python accepts `adjective` and never reads it: KA has no
        // CURRENCY_ADJECTIVES and never calls prefix_currency, so
        // `adjective=True` is byte-identical to the plain call (quirk 13).
        let _ = adjective;

        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            DEFAULT_SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)` happens *before*
        // `str(val)`, so the string never carries a sign. The abs() is
        // conditional in Python, so it stays conditional here. Note `-0.0 < 0`
        // is False in Python and `BigDecimal` has no signed zero, so both sides
        // agree that -0.0 is not negative.
        let is_negative = val.is_negative();
        let s = match val {
            CurrencyValue::Int(v) => {
                if is_negative {
                    v.abs().to_string()
                } else {
                    v.to_string()
                }
            }
            CurrencyValue::Decimal { value: d, .. } => {
                if is_negative {
                    d.abs().to_string()
                } else {
                    d.to_string()
                }
            }
        };

        // `str(val).split(".")` splits on every dot; Python then reads only
        // parts[0] and parts[1], so trailing fragments are ignored either way.
        let mut parts = s.split('.');
        let part0 = parts.next().unwrap_or("");
        let part1 = parts.next();

        // `int(parts[0]) if parts[0] else 0`. Evaluated before `right`, so a
        // scientific-notation string raises here first.
        let left = if part0.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_str(part0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        //
        // `[:2]` truncates and `ljust` pads on the right, so "5" -> "50" (0.5 is
        // fifty subunits) and "01" -> "01" (0.01 is one). Sliced by chars, not
        // bytes. A whole float gives "0" -> "00" -> 0, which is falsy (quirk 10).
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

        // `.get(currency, self.CURRENCY_FORMS["GEL"])` — quirk 8.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.fallback_forms);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        let one = BigInt::one();

        // `left_str + " " + (cr1[1] if left != 1 else cr1[0])`. Note 0 takes the
        // plural: "ნული euros". `_int_to_word` is called directly, not
        // `to_cardinal`, so a left part >= 10^12 falls back to bare digits
        // (bug 3) rather than raising.
        let mut result = format!(
            "{} {}",
            int_to_word(&left),
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        // `if cents and right:` — `right` is an int, so 0 is falsy and a float
        // with zero cents drops the whole segment (quirk 10). `cents=False`
        // drops it too, with no terse fallback (quirk 12).
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&int_to_word(&right));
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `result = self.negword + result` — raw, keeping the trailing space of
        // "მინუს " rather than the `"%s " % negword.strip()` base would use.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `result.strip()`. A no-op for every reachable input, but it is what
        // Python writes.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;

    fn flt(v: f64) -> String {
        // Num2Word_KA ignores precision, so any value is equally correct; pass
        // the honest repr-derived one.
        let precision = py_float_repr(v)
            .split_once('.')
            .map(|(_, f)| f.len() as u32)
            .unwrap_or(0);
        LangKa::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        LangKa::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `"lang": "ka", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(flt(0.0), "ნული წერტილი ნული");
        assert_eq!(flt(0.5), "ნული წერტილი ხუთი");
        assert_eq!(flt(1.0), "ერთი წერტილი ნული");
        assert_eq!(flt(1.5), "ერთი წერტილი ხუთი");
        assert_eq!(flt(2.25), "ორი წერტილი ორი ხუთი");
        assert_eq!(flt(3.14), "სამი წერტილი ერთი ოთხი");
        assert_eq!(flt(0.01), "ნული წერტილი ნული ერთი");
        assert_eq!(flt(0.1), "ნული წერტილი ერთი");
        assert_eq!(flt(0.99), "ნული წერტილი ცხრა ცხრა");
        assert_eq!(flt(1.01), "ერთი წერტილი ნული ერთი");
        assert_eq!(flt(12.34), "თორმეტი წერტილი სამი ოთხი");
        assert_eq!(flt(99.99), "ოთხმოცდაათი ცხრა წერტილი ცხრა ცხრა");
        assert_eq!(flt(100.5), "ერთი ასი წერტილი ხუთი");
        assert_eq!(
            flt(1234.56),
            "ერთი ათასი ორი ასი ოცდაათი ოთხი წერტილი ხუთი ექვსი"
        );
        assert_eq!(flt(-0.5), "მინუს ნული წერტილი ხუთი");
        assert_eq!(flt(-1.5), "მინუს ერთი წერტილი ხუთი");
        assert_eq!(flt(-12.34), "მინუს თორმეტი წერტილი სამი ოთხი");
        // The two f64-artefact cases: right answer via repr(), not float2tuple.
        assert_eq!(flt(1.005), "ერთი წერტილი ნული ნული ხუთი");
        assert_eq!(flt(2.675), "ორი წერტილი ექვსი შვიდი ხუთი");
    }

    /// Every `"lang": "ka", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(dec("0.01"), "ნული წერტილი ნული ერთი");
        // Trailing zero survives: str(Decimal("1.10")) == "1.10".
        assert_eq!(dec("1.10"), "ერთი წერტილი ერთი ნული");
        assert_eq!(dec("12.345"), "თორმეტი წერტილი სამი ოთხი ხუთი");
        // Issue #603's value: int part >= 10^12 => bare digits (bug 3), exact
        // fraction preserved (no float() cast on the Decimal arm).
        assert_eq!(
            dec("98746251323029.99"),
            "98746251323029 წერტილი ცხრა ცხრა"
        );
        assert_eq!(dec("0.001"), "ნული წერტილი ნული ნული ერთი");
    }

    /// -0.0 is not `< 0`, but its float str() starts with '-', stripped
    /// textually into a negword. (The Decimal arm cannot represent -0.0.)
    #[test]
    fn negative_zero_float_keeps_negword() {
        assert_eq!(flt(-0.0), "მინუს ნული წერტილი ნული");
        assert_eq!(flt(0.0), "ნული წერტილი ნული");
    }

    /// precision= is honoured by __init__.py and then ignored by the converter.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangKa::new();
        let v = FloatValue::Float {
            value: 1.23456,
            precision: 5,
        };
        let want = "ერთი წერტილი ორი სამი ოთხი ხუთი ექვსი";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want);
        for p in [0u32, 1, 2, 5, 9] {
            assert_eq!(l.to_cardinal_float(&v, Some(p)).unwrap(), want);
        }
    }

    /// str(float)/str(Decimal) go exponential outside their fixed ranges, and
    /// int() chokes. Which literal lands in the message depends on whether a
    /// '.' split the string first.
    #[test]
    fn exponent_notation_raises_value_error() {
        let l = LangKa::new();
        let f = |v: f64| {
            l.to_cardinal_float(&FloatValue::Float { value: v, precision: 0 }, None)
                .unwrap_err()
        };
        assert!(matches!(f(1e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e+16'"));
        assert!(matches!(f(1e-5), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e-05'"));
        // "1.5e+16" splits: int("1") is fine, then the digit loop hits 'e'.
        assert!(matches!(f(1.5e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'e'"));
        let d = |s: &str| {
            l.to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision: 0,
                },
                None,
            )
            .unwrap_err()
        };
        assert!(matches!(d("1E+16"), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1E+16'"));
        assert!(matches!(d("1E-7"), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1E-7'"));
    }

    /// Integer-valued Decimals have no dot in str(), so they take the else
    /// branch and print just the integer — no pointword.
    #[test]
    fn integer_decimals_have_no_point() {
        assert_eq!(dec("5"), "ხუთი");
        assert_eq!(dec("1.00E+2"), "ერთი ასი"); // str -> "100"
    }

    /// Boundaries: largest fixed-notation float, and a pure fraction.
    #[test]
    fn large_and_small_boundaries() {
        assert_eq!(flt(1e15), "1000000000000000 წერტილი ნული");
        assert_eq!(flt(0.0001), "ნული წერტილი ნული ნული ნული ერთი");
        assert_eq!(
            dec("0.000001"),
            "ნული წერტილი ნული ნული ნული ნული ნული ერთი"
        );
    }

    /// repr() reproduction, including the dtoa tie cases.
    #[test]
    fn py_float_repr_matches_cpython() {
        for (v, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (0.5, "0.5"),
            (1.0, "1.0"),
            (0.01, "0.01"),
            (1.005, "1.005"),
            (2.675, "2.675"),
            (100.5, "100.5"),
            (1e15, "1000000000000000.0"),
            (1e16, "1e+16"),
            (1.5e16, "1.5e+16"),
            (0.0001, "0.0001"),
            (1e-5, "1e-05"),
            (98746251323029.99, "98746251323029.98"),
            (-78198386800398.125, "-78198386800398.12"),
            (-1267860061485775.25, "-1267860061485775.2"),
        ] {
            assert_eq!(py_float_repr(v), want, "repr({:?})", v);
        }
    }

    /// str(Decimal) reproduction — _pydecimal.Decimal.__str__.
    #[test]
    fn py_decimal_str_matches_cpython() {
        for (s, want) in [
            ("1.10", "1.10"),
            ("0.00", "0.00"),
            ("-0.5", "-0.5"),
            ("1E+16", "1E+16"),
            ("1E-7", "1E-7"),
            ("0.0000001", "1E-7"),
            ("0.000001", "0.000001"),
            ("1.00E+2", "100"),
            ("12.345", "12.345"),
            ("98746251323029.99", "98746251323029.99"),
            ("0.01", "0.01"),
        ] {
            assert_eq!(
                py_decimal_str(&BigDecimal::from_str(s).unwrap()),
                want,
                "{}",
                s
            );
        }
    }
}
