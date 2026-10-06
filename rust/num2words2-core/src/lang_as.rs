//! Port of `lang_AS.py` (Assamese). Registry key `"as"` → `Num2Word_AS`.
//!
//! Shape: **self-contained**. `Num2Word_AS` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python's
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `self.MAXVAL` (that block is guarded by
//! `if any(hasattr(self, field) for field in [...])`). `to_cardinal` is
//! overridden outright and drives the recursive `_int_to_word` below.
//! Consequently `cards`/`merge` stay at their trait defaults here. Python has
//! **no overflow check at all** — the recursion peels off factors of 10^7
//! ("কোটি") indefinitely, and on a large enough integer the port overflowed
//! the native stack and killed the process. So the port adds a ceiling
//! (gladiaio/num2words2#203): `maxval` is 10^14, where the crore multiplier
//! would itself need "কোটি", and every mode raises `OverflowError` from
//! there. All four in-scope modes are overridden by the
//! Python class, so nothing is inherited from `Num2Word_Base` except the
//! `negword`/`pointword` slots that `setup()` immediately replaces.
//!
//! The numbering system is Indian (হাজাৰ / লাখ / কোটি = 10^3 / 10^5 / 10^7),
//! not the short scale, and `_int_to_word` recurses on the quotient — so in
//! Python 10^15 is "দহ কোটি কোটি" (ten crore crore) and 10^21 is
//! "এক কোটি কোটি কোটি". Both are past the port's 10^14 ceiling.
//!
//! # The float/Decimal path is a *string* algorithm
//!
//! `Num2Word_AS.to_cardinal` opens with `n = str(number).strip()` and branches
//! on `"." in n`. It never calls `Num2Word_Base.to_cardinal_float`, never calls
//! `float2tuple`, and never reads `self.precision`. So none of the machinery in
//! [`crate::floatpath`] applies here: no `pre`/`post` split, no
//! `10**precision` scaling, no `abs(round(post) - post) < 0.01` heuristic, and
//! no banker's-rounding trap. The fractional digits spoken are literally the
//! characters after the `.` in `repr(value)`.
//!
//! That means the whole float path reduces to reproducing Python's `str()`:
//! [`py_float_repr`] for `float`, [`py_decimal_str`] for `Decimal`. The two
//! classic f64-artefact cases still come out right, but for a different reason
//! than everywhere else — `repr(2.675)` is the shortest string that round-trips
//! (`"2.675"`), so the `674.9999999999998` artefact that `float2tuple` has to
//! rescue is never computed in the first place. Both mechanisms happen to agree
//! on 1.005 and 2.675; they are not the same mechanism.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. The following look wrong but are exactly
//! what Python emits, and every one is pinned by a corpus row:
//!
//! 1. **The hundreds branch omits the separating space.** Every other branch
//!    of `_int_to_word` joins its remainder with `" " + ...`; the `< 1000`
//!    branch alone writes `+ (self._int_to_word(number % 100) if ...)` with
//!    no space. So the hundred-word "শ" fuses onto the next word:
//!    `101` → "এক শএক", `110` → "এক শদহ", `999` → "নয় শনব্বই নয়",
//!    `1905` → "এক হাজাৰ নয় শপাঁচ". A bare `100` is unaffected ("এক শ")
//!    because the remainder is empty. Note the space that *does* appear is
//!    the leading one baked into the `" শ"` literal ([`HUNDRED`]), not a
//!    separator. See [`int_to_word`].
//! 2. **Tens are not compounded.** `21` is "বিশ এক" (literally "twenty one"
//!    as two juxtaposed words) rather than the idiomatic Assamese "একৈশ";
//!    `42` is "চল্লিশ দুই". The table has no 21–99 compound forms at all.
//!    Reproduced verbatim.
//! 3. **`to_ordinal` inserts a stray space before the suffix**:
//!    `cardinal + " -তম"` yields "এক -তম", not "এক-তম". `to_ordinal_num`
//!    uses the tight `"-তম"` instead, so the two disagree by a space.
//! 4. **Negatives pass straight through `to_ordinal`.** Python's
//!    `Num2Word_Base` normally rejects negative ordinals
//!    (`errmsg_negord`), but `Num2Word_AS.to_ordinal` never calls the base
//!    and never checks: `to_ordinal(-1)` == "ঋণাত্মক এক -তম", and
//!    `to_ordinal_num(-1)` == "-1-তম" (the minus survives `str(number)`).
//!    No error is raised. Corpus-confirmed.
//! 5. **`to_year` ignores its sign and its `longval` argument.** It is a flat
//!    `"চন " + self.to_cardinal(val)`, so a negative year renders the prefix
//!    *before* the negative marker: `to_year(-500)` == "চন ঋণাত্মক পাঁচ শ"
//!    ("year negative five hundred") with no BC/AD handling whatsoever.
//!
//! # Error variants
//!
//! For integer input below the 10^14 ceiling all four modes are total — no
//! table lookup that can miss, and no negative-ordinal guard. The only
//! `_int_to_word` list indexes (`ones`, `tens`, `teens`) are proven in range
//! by the branch guards that precede them, so no `IndexError` is reachable
//! either.
//!
//! The float path adds one, from the `int()` calls `to_cardinal` makes on the
//! pieces of `str(number)`:
//!
//! * `Value` (`ValueError`) whenever `str(value)` is not a plain decimal
//!   literal — exponent notation, `inf`, `nan`. `str(float)` switches to
//!   exponent form at `|v| >= 1e16` and `0 < |v| < 1e-4`, so `1e16` raises
//!   `invalid literal for int() with base 10: '1e+16'` (no `.`, the whole
//!   string reaches `int()`) while `1.5e16` raises on the fragment `'e'` (the
//!   `.` splits it, so the digit loop hits `int("e")` first). See
//!   [`LangAs::to_cardinal_float`].
//!
//! The currency surface adds two:
//!
//! * `NotImplemented` from `to_cheque` for a code outside `CURRENCY_FORMS`
//!   (base's `except KeyError: raise NotImplementedError`). Note
//!   `to_currency` raises **nothing** — see [`LangAs::to_currency`].
//! * `Value` from `to_currency` when `str(val)` would be in exponent
//!   notation — see [`split_currency`].
//!
//! # Currency shape
//!
//! `Num2Word_AS` overrides `to_currency` wholesale and never calls
//! `Num2Word_Base.to_currency`, so none of `parse_currency_parts`,
//! `pluralize`, `_money_verbose`, `_cents_verbose`, `_cents_terse`,
//! `CURRENCY_ADJECTIVES` or `CURRENCY_PRECISION` is reachable from it. It does
//! *not* override `to_cheque`, which does go through base and therefore does
//! reach `_money_verbose` (→ `to_cardinal`) and `CURRENCY_PRECISION` (empty →
//! 100 for every code, the trait default). That asymmetry is why `JPY`/`KWD`
//! still render two decimal digits of "cents" here.

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

/// `setup()`: `self.negword = "ঋণাত্মক "`. Note the **trailing space** — it is
/// part of the literal and `to_cardinal` concatenates it raw (unlike
/// `Num2Word_Base`, which does `"%s " % self.negword.strip()`). Same result
/// here, since there is exactly one trailing space, but the concatenation is
/// reproduced as written.
const NEGWORD: &str = "ঋণাত্মক ";

/// `setup()`: `self.pointword = "দশমিক"`. Reached only through the `"." in n`
/// branch of `to_cardinal` — see [`LangAs::to_cardinal_float`]. Unlike
/// `Num2Word_Base.to_cardinal_float`, which pushes `self.title(self.pointword)`
/// as its own list element, `Num2Word_AS` concatenates `" " + pointword + " "`
/// by hand and never calls `title()`. Identical here (`is_title` is false), but
/// reproduced as written.
const POINTWORD: &str = "দশমিক";

/// `_int_to_word`'s early return for zero. Note `ones[0]` is `""`, so this
/// guard is what makes `to_cardinal(0)` produce a word at all.
const ZERO_WORD: &str = "শূন্য";

/// `ones`. Index 0 is `""` and is only ever reached via `number % 10 == 0`
/// (guarded against) or `number // 100 == 0` (impossible for `number >= 100`).
const ONES: [&str; 10] = [
    "", "এক", "দুই", "তিনি", "চাৰি", "পাঁচ", "ছয়", "সাত", "আঠ", "নয়",
];

/// `tens`. Indices 0 and 1 are `""`: the `< 20` teens branch runs first, so
/// `tens[0]`/`tens[1]` are unreachable.
const TENS: [&str; 10] = [
    "",
    "",
    "বিশ",
    "ত্ৰিশ",
    "চল্লিশ",
    "পঞ্চাশ",
    "ষাঠি",
    "সত্তৰ",
    "আশী",
    "নব্বই",
];

/// `teens`, indexed `number - 10`, covering 10..=19.
const TEENS: [&str; 10] = [
    "দহ",
    "এঘাৰ",
    "বাৰ",
    "তেৰ",
    "চৈধ্য",
    "পোন্ধৰ",
    "ষোল্ল",
    "সোতৰ",
    "ওঠৰ",
    "উনিশ",
];

/// `" শ"` (hundred) — the leading space is part of the Python literal.
/// Crucially there is no *trailing* space, and the hundreds branch adds none
/// before recursing; that is bug 1 in the module docs.
const HUNDRED: &str = " শ";
/// `"হাজাৰ"` = 10^3.
const THOUSAND: &str = "হাজাৰ";
/// `"লাখ"` = 10^5 (one lakh).
const LAKH: &str = "লাখ";
/// `"কোটি"` = 10^7 (one crore). The top of the table — the `else` branch
/// recurses on `number // 10000000`, so this word stacks for large values.
const CRORE: &str = "কোটি";

/// `to_ordinal`: `cardinal + " -তম"`. Space included (bug 3).
const ORDINAL_SUFFIX: &str = " -তম";
/// `to_ordinal_num`: `str(number) + "-তম"`. No space (bug 3).
const ORDINAL_NUM_SUFFIX: &str = "-তম";
/// `to_year`: `"চন " + self.to_cardinal(val)`.
const YEAR_PREFIX: &str = "চন ";

/// `Num2Word_AS.to_currency`'s own default `separator=" আৰু"` ("and").
///
/// See [`SEPARATOR_UNSET`] for why this cannot be a plain parameter default.
/// Note it carries a *leading* space but no trailing one, and Python
/// concatenates the cents word straight onto it — bug 6 in
/// [`LangAs::to_currency`].
const SEPARATOR_DEFAULT: &str = " আৰু";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_AS.to_currency` declares `separator=" আৰু"`, but the `Lang` trait
/// has no per-language defaults: `__init__.py`'s fast path (and
/// `bench/diff_test.py`) both send `kwargs.get("separator", ",")` — i.e.
/// `Num2Word_Base`'s default — so by the time the value reaches this side, the
/// information needed to tell "unset" from "explicitly a comma" is gone.
///
/// So `,` is read back as the unset sentinel and AS's own default restored.
/// This is the only reading that matches the oracle: all 63 float rows of the
/// `as` currency corpus were generated by `num2words(v, lang="as",
/// to="currency", currency=c)` with no `separator=`, and every one of them
/// expects " আৰু". Same convention (and same sentinel) as `lang_ca.rs`.
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " আৰু" here where Python would give ",". Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// Python's `_split_currency`, which is a **string** operation, not an
/// arithmetic one:
///
/// ```text
/// parts = str(n).split(".")
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `n` is always non-negative (`to_currency` has already taken `abs`).
///
/// Two consequences worth spelling out, both corpus-pinned:
///
/// * `right` is built by *left-justifying the first two fraction digits*, so
///   `0.5` → `"5"` → `"50"` → 50 centavos, while `0.05` → `"05"` → 5. It is a
///   digit-slice, not a multiplication, and it silently truncates a third
///   decimal (`12.349` → 34, no rounding).
/// * a fraction of `"0"` (i.e. `1.0`) yields `right == 0`, which
///   `to_currency` treats as falsy and drops the cents clause entirely — the
///   one place where AS's float path agrees with base's int path.
///
/// # The exponent-notation hole
///
/// `str(float)` switches to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal:
/// `to_currency(1e16)` raises `ValueError: invalid literal for int() with
/// base 10: '1e+16'`, and `1.5e16` raises on the fragment `'5e'`. A negative
/// `BigDecimal` scale is exactly the "the source string used `e+` notation"
/// signal (a plain digit string parses to scale 0, never below), for floats
/// and `Decimal`s alike — `str(Decimal("1E+16"))` fails identically — so that
/// arm is reproduced.
///
/// The `e-` side is **not** reproducible here and is flagged in the port
/// report: `1e-05` and `Decimal("0.00001")` parse to the *same* `BigDecimal`
/// (digits 1, scale 5) yet Python raises `ValueError` for the first and
/// returns "শূন্য ইউৰো" for the second. The discriminator is the original
/// string, which the `CurrencyValue` boundary does not carry.
fn split_currency(n: &BigDecimal) -> Result<(BigInt, BigInt)> {
    // value == digits * 10^-scale
    let (digits, scale) = n.as_bigint_and_exponent();

    if scale < 0 {
        // str(n) would be "1e+16"-shaped; int() on it raises. Python's message
        // quotes the exact offending literal, which is unrecoverable from the
        // parsed value — the type is what callers observe.
        return Err(N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            n
        )));
    }

    // No "." in str(n): parts == [str(n)], so right stays 0.
    if scale == 0 {
        return Ok((digits, BigInt::zero()));
    }

    // `n` is non-negative, so this is a bare ASCII digit string.
    let s = digits.abs().to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // parts[1][:2].ljust(2, "0") — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

/// The shortest round-tripping decimal digits of `a` (which must be finite,
/// non-negative), plus `decpt`, the decimal-point position such that
/// `a == 0.<digits> * 10**decpt`. This is CPython's `_Py_dg_dtoa(a, 0, 0, ...)`
/// — David Gay's `dtoa` in mode 0.
///
/// Rust's `{:e}` is also shortest-round-trip, and agrees with Gay on both the
/// digit *count* and, almost always, the digits. It disagrees on one thing:
/// **exact ties**. When `a`'s exact binary value sits precisely halfway between
/// the two shortest candidates, both round-trip, and Gay's dtoa takes the one
/// with an **even** last digit (`if (dig & 1) goto bump_up;`) while Rust rounds
/// half **up**. `repr(-78198386800398.125)` is `'-78198386800398.12'`; Rust's
/// `{:e}` says `...13`. Fuzzed over 3.7M values (2.4M of them `m/2**k`, the
/// tie-prone shape), that is the *only* divergence: it fires 20,958 times and
/// the correction below fixes every one without introducing a single new
/// mismatch.
///
/// Detecting the tie needs no bignum. Write `a = m * 2**e` with `m` odd, and
/// let `q = digits.len() - decpt`. The tie condition — `a * 10**q == k + 1/2`
/// for integer `k` — reduces to:
///
/// * `e + q + 1 == 0`, and
/// * if `q < 0`, additionally `5**-q` divides `m` (and `-q <= 22`, since
///   `5**23 > 2**53 > m`).
///
/// Then `2k + 1 == m * 5**q` (or `m / 5**-q`), and because `5 ≡ 1 (mod 4)`,
/// that odd integer is `≡ m (mod 4)` either way. So `k` is even exactly when
/// `m % 4 == 1` — no big integers, no exact decimal expansion.
///
/// The fix-up itself is direction-agnostic: in a tie Python's answer is always
/// the candidate with the even last digit, so if Rust's last digit is odd we
/// step toward the even neighbour, and `k`'s parity says which way.
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
            // k is even, so Python wants k and Rust gave k+1. The last digit
            // is odd, hence non-zero, so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k is odd, so Python wants k+1 and Rust gave k. Carry like
            // dtoa's `roundoff`: "99" -> "1" with decpt bumped.
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

/// Python's `str(float)` (== `repr(float)`), which `Num2Word_AS.to_cardinal`
/// promotes from a formatting detail to *the entire specification* of the
/// float path.
///
/// This is CPython's `format_float_short(..., 'r', ...)` in `pystrtod.c`.
/// Rust's own `{}` cannot stand in: it never switches to exponent notation
/// (`format!("{}", 1e16_f64)` is `"10000000000000000"`, where Python says
/// `'1e+16'` — and that difference is the whole reason `to_cardinal(1e16)`
/// raises `ValueError`) and it prints `1`, not `1.0`, for integral floats.
///
/// The rules, straight from `format_float_short`:
///
/// * exponent notation iff `decpt <= -4 || decpt > 16` — the `> 16` (rather
///   than `> 17`) is deliberate upstream, so that `repr(2e16+8)` does not
///   render as `20000000000000010.0`;
/// * the exponent is `%+.02d`: always signed, zero-padded to two digits, hence
///   `1e-05` but `1e+100`;
/// * `Py_DTSF_ADD_DOT_0` appends `.0` to an otherwise integral fixed-notation
///   result, but never in exponent notation (`repr(1e16) == '1e+16'`);
/// * `nan` drops its sign, `inf` keeps it. Rust would say `NaN`/`inf`.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        // CPython prints "nan" for both signs of NaN.
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
/// The `leftdigits > -6` guard is what keeps every `"0".repeat(...)` below
/// bounded by five: in the no-exponent branch `_exp <= 0` forces
/// `dotplace <= len(_int)`, and in the exponent branch `dotplace == 1`. So
/// `Decimal("1E-1000000000")` renders as `'1E-1000000000'` rather than trying
/// to materialise a billion zeros — same as Python.
///
/// # The negative-zero hole
///
/// `Decimal` carries `_sign` independently of `_int`, so `Decimal("-0.0")` is
/// signed zero and `str()` gives `'-0.0'`; `to_cardinal` then strips that minus
/// textually and answers "ঋণাত্মক শূন্য দশমিক শূন্য". A `BigDecimal` cannot
/// represent it: its `int_val` is a `BigInt`, which has no negative zero, so
/// `BigDecimal::from_str("-0.0")` has already discarded the sign before this
/// function is called. We emit `'0.0'` and drop the negword.
///
/// Same shape as the `e-` hole in [`split_currency`]: the discriminator is the
/// original string, which the `FloatValue::Decimal` boundary does not carry.
/// Fixing it needs the shim to pass `decimal_str` through, which lives in
/// `num2words2-py` — outside this port's remit. Flagged in the port report.
///
/// The blast radius is exactly the negative zeros that render *without* an
/// exponent — `Decimal("-0")` through `Decimal("-0.000000")`. Beyond that the
/// `E±n` form kicks in, `int()` raises `ValueError` either way, and the
/// messages agree: `to_cardinal` removes the sign from `n` *before* calling
/// `int()`, so Python reports `'0E-19'` too, not `'-0E-19'`. Confirmed by
/// differential fuzz: of 60,025 Decimal cases (74 of them negative zeros), the
/// 11 fixed-notation negative zeros are the only divergence.
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

/// Python's `int(s)`, for the strings [`cardinal_from_str`] hands it.
///
/// The real builtin is more permissive than this: it also accepts non-ASCII
/// decimal digits (`int("١٢") == 12`) and strips a slightly different
/// whitespace set. None of that is reachable — every string that gets here is
/// a fragment of `str(float)` / `str(Decimal)`, which is ASCII by
/// construction — so the extra generality is deliberately not ported. What is
/// ported is the underscore rule (`int("1_0") == 10`, `int("1_")` raises) and,
/// crucially, the error message: Python formats the **original, unstripped**
/// argument with `%.200R`, i.e. `repr(s)`. Every literal that reaches this
/// function is plain ASCII with no quote or backslash, so `'{}'` is exactly
/// what `repr` would print.
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
    // int() permits '_' as a digit separator, but not leading, trailing or
    // doubled.
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

/// The body of `Num2Word_AS.to_cardinal`, driven by `str(number)`:
///
/// ```text
/// n = str(number).strip()
/// if n.startswith("-"):
///     n = n[1:]
///     ret = self.negword
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
/// Three details that matter:
///
/// * The sign is stripped **textually**, so `-0.0` (whose `str` is `"-0.0"`)
///   keeps its negword even though the value is not `< 0`: Python answers
///   "ঋণাত্মক শূন্য দশমিক শূন্য". `Num2Word_Base.to_cardinal_float` reaches
///   the same place via `if value < 0 and pre == 0`, which `-0.0` would fail.
/// * `split(".", 1)` caps at one split, so a second dot stays inside `right`
///   and detonates in the digit loop rather than being ignored.
/// * The digit loop is a generator consumed by `join`, and `int(left)` runs
///   *before* it. So for `1.5e+16` the failing literal reported is `'e'`, not
///   `'1.5e+16'` — the left half parsed fine and the loop got as far as the
///   `e`. Order is load-bearing for the error message; keep it.
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

    // " ".join(self._int_to_word(int(d)) for d in right) — Python iterates
    // *characters*, so index by chars(), never bytes.
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

/// Narrow a `BigInt` the caller has already proven is `< 1000`.
///
/// Every call site sits behind a range check from `_int_to_word`'s if/elif
/// ladder, so the conversion cannot fail; `unwrap_or(0)` keeps the port
/// panic-free without inventing behaviour Python does not have.
fn small(n: &BigInt) -> usize {
    n.to_usize().unwrap_or(0)
}

/// Python's `_int_to_word`. `number` is always non-negative: `to_cardinal`
/// strips the sign from the *string* before calling `int()`.
fn int_to_word(number: &BigInt) -> String {
    if number.is_zero() {
        return ZERO_WORD.to_string();
    }

    // number < 10  →  ones[number]
    if *number < BigInt::from(10) {
        return ONES[small(number)].to_string();
    }

    // number < 20  →  teens[number - 10]
    if *number < BigInt::from(20) {
        return TEENS[small(number) - 10].to_string();
    }

    // number < 100  →  tens[number // 10] + (" " + ones[number % 10] if number % 10 else "")
    if *number < BigInt::from(100) {
        let n = small(number);
        let mut out = TENS[n / 10].to_string();
        if n % 10 != 0 {
            out.push(' ');
            out.push_str(ONES[n % 10]);
        }
        return out;
    }

    // number < 1000  →  ones[number // 100] + " শ" + (_int_to_word(number % 100) if number % 100 else "")
    //
    // BUG 1: no " " before the recursive call, unlike every branch below.
    if *number < BigInt::from(1000) {
        let n = small(number);
        let mut out = ONES[n / 100].to_string();
        out.push_str(HUNDRED);
        if n % 100 != 0 {
            out.push_str(&int_to_word(&BigInt::from(n % 100)));
        }
        return out;
    }

    // The three remaining branches are structurally identical in Python —
    // _int_to_word(number // D) + " WORD" + (" " + _int_to_word(number % D)
    // if number % D else "") — differing only in (D, WORD). The `else` arm
    // (crore) has no upper bound, which is why this is BigInt all the way
    // down and why no OverflowError exists for Assamese.
    let (divisor, word) = if *number < BigInt::from(100_000) {
        (BigInt::from(1_000), THOUSAND)
    } else if *number < BigInt::from(10_000_000) {
        (BigInt::from(100_000), LAKH)
    } else {
        (BigInt::from(10_000_000), CRORE)
    };

    // div_mod_floor matches Python's // and % exactly; the operands are
    // non-negative here, so it coincides with truncating division.
    let (quotient, remainder) = number.div_mod_floor(&divisor);

    let mut out = int_to_word(&quotient);
    out.push(' ');
    out.push_str(word);
    if !remainder.is_zero() {
        out.push(' ');
        out.push_str(&int_to_word(&remainder));
    }
    out
}

pub struct LangAs {
    /// `CURRENCY_FORMS`. Only three codes, each `(("x","x"), ("y","y"))` — the
    /// singular and plural forms are *identical* throughout, so the
    /// `cr1[1] if left != 1 else cr1[0]` choice below is never observable.
    /// Ported as written anyway; the arity (2+2) is what keeps that indexing
    /// in range.
    ///
    /// Built once here rather than per call: `to_currency` reaches for
    /// `CURRENCY_FORMS["INR"]` on every unknown code, and rebuilding the table
    /// each time is what made an earlier revision of this port 10x slower than
    /// the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangAs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangAs {
    pub fn new() -> Self {
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            (
                "INR",
                CurrencyForms::new(&["ৰুপী", "ৰুপী"], &["পইচা", "পইচা"]),
            ),
            (
                "USD",
                CurrencyForms::new(&["ডলাৰ", "ডলাৰ"], &["চেণ্ট", "চেণ্ট"]),
            ),
            (
                "EUR",
                CurrencyForms::new(&["ইউৰো", "ইউৰো"], &["চেণ্ট", "চেণ্ট"]),
            ),
        ]
        .into_iter()
        .collect();
        LangAs { currency_forms }
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// কোটি (10^7), so from 10^14 the crore multiplier would itself need কোটি and
/// the word would stack ("এক কোটি কোটি").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(14))
}

/// `int_to_word` behind [`maxval_ceiling`], checked before the first recursive
/// step. Every entry point that hands over a caller-supplied integer goes
/// through here.
fn checked_int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    Ok(int_to_word(number))
}

impl Lang for LangAs {
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

    /// `to_ordinal(float/Decimal)`. Python's `to_ordinal` is a flat
    /// `self.to_cardinal(number) + " -তম"` (stray space included — bug 3),
    /// and for a float/Decimal that `to_cardinal` is the `str(number)`
    /// algorithm above. So whole values keep their ".0" tail *and* gain the
    /// suffix — `to_ordinal(5.0)` == "পাঁচ দশমিক শূন্য -তম", `Decimal("5.00")`
    /// == "পাঁচ দশমিক শূন্য শূন্য -তম" — and -0.0 keeps its textual negword
    /// ("ঋণাত্মক শূন্য দশমিক শূন্য -তম"). Exponent-form inputs (1e16,
    /// `Decimal("1E+2")`) raise `int()`'s ValueError from inside
    /// `to_cardinal` before the suffix is ever appended.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            self.to_cardinal_float(value, None)?,
            ORDINAL_SUFFIX
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "-তম"`, verbatim (no
    /// space — bug 3). `repr_str` is the binding's Python `str(value)`
    /// (`repr(float)` / `str(Decimal)`), which is exactly the string Python
    /// concatenates — so `str()` never raises and even the exponent forms
    /// that explode in `to_ordinal` come out fine: `to_ordinal_num(1e16)` ==
    /// "1e+16-তম", `Decimal("1E+20")` == "1E+20-তম", -0.0 == "-0.0-তম".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, ORDINAL_NUM_SUFFIX))
    }

    /// `to_year(float/Decimal)`: `"চন " + self.to_cardinal(val)` — the same
    /// flat prefix as the int path (bug 5: the sign is not special-cased, so
    /// `to_year(-0.0)` is "চন ঋণাত্মক শূন্য দশমিক শূন্য" with the era word
    /// *before* the negword, and there is no BC handling at all). The
    /// exponent-form ValueError propagates from `to_cardinal` unchanged.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!(
            "{}{}",
            YEAR_PREFIX,
            self.to_cardinal_float(value, None)?
        ))
    }

    /// AS inherits Base's `str_to_number` (`Decimal(value)`), which parses
    /// "Infinity"/"-Infinity" *successfully* — the ValueError Python shows
    /// for `num2words("Infinity", lang="as")` happens later, inside AS's
    /// `to_cardinal` (`int("Infinity")` → `ValueError: invalid literal for
    /// int() with base 10: 'Infinity'`; "-Infinity" strips its sign
    /// textually first and reports 'Infinity' too). The Rust dispatcher
    /// hard-codes Base's `int(Decimal("Infinity"))` semantics
    /// (`OverflowError`) for `ParsedNumber::Inf`, which AS never executes,
    /// so Inf parses punt to the Python fallback instead: it runs the
    /// original converter and reproduces every mode byte for byte
    /// (cardinal/ordinal/year raise the ValueError; `to_ordinal_num` happily
    /// returns "Infinity-তম"). NaN stays on the dispatcher path — its
    /// hard-coded ValueError already matches the type AS raises from
    /// `int("NaN")`. Same shape as `lang_be.rs`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        let parsed = python_decimal_parse(s)?;
        Ok(parsed)
    }

    /// `Decimal('Infinity')` / `-Infinity` reached AS. AS overrides
    /// `str_to_number` with Base's `Decimal(value)`, so the parse succeeds and
    /// the failure surfaces per mode inside AS's own converters — **not** at
    /// Base's `int(Decimal('Infinity'))` (which the trait default renders as
    /// `OverflowError`). AS's `to_cardinal` strips the sign textually then does
    /// `int("Infinity")` → `ValueError`; `to_ordinal` (`to_cardinal + " -তম"`)
    /// and `to_year` (`"চন " + to_cardinal`) raise the same `ValueError` first.
    /// `to_ordinal_num` is the textual `str(number) + "-তম"`, so it echoes.
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => {
                let token = if negative { "-Infinity" } else { "Infinity" };
                Ok(format!("{}{}", token, ORDINAL_NUM_SUFFIX))
            }
            // cardinal/ordinal/year: int() sees the sign-stripped token.
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".to_string(),
            )),
        }
    }

    /// `Decimal('NaN')` reached AS. `int("NaN")` → `ValueError` on the
    /// cardinal/ordinal/year paths; `to_ordinal_num` echoes "NaN-তম".
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!("NaN{}", ORDINAL_NUM_SUFFIX)),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".to_string(),
            )),
        }
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
        " আৰু"
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "দশমিক"
    }

    /// Python:
    /// ```text
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
    /// else:
    ///     ret = ""
    /// ...
    /// return ret + self._int_to_word(int(n))
    /// ```
    /// The sign is stripped textually and `negword` (already space-suffixed)
    /// is prepended, so `-1` → "ঋণাত্মক এক". The `"." in n` branch is the
    /// float path and is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let (prefix, magnitude) = if value.is_negative() {
            (NEGWORD, -value)
        } else {
            ("", value.clone())
        };
        Ok(format!("{}{}", prefix, checked_int_to_word(&magnitude)?))
    }

    /// The `"." in n` branch of `Num2Word_AS.to_cardinal`, which is the same
    /// method as [`LangAs::to_cardinal`] — Python has one `to_cardinal` and
    /// splits on `str(number)`, so the trait's int/float split lands mid-method
    /// here rather than at a class boundary.
    ///
    /// **`Num2Word_Base.to_cardinal_float` is never reached for Assamese**, and
    /// neither is `float2tuple`. The consequences are worth being explicit
    /// about, because they invert the usual advice for this path:
    ///
    /// * There is no `precision` anywhere. `num2words(..., precision=2)` does
    ///   set `converter.precision` (base's `__init__` leaves the attribute at
    ///   `2`, so `hasattr` passes and the kwarg is honoured *syntactically*),
    ///   but `Num2Word_AS.to_cardinal` never reads it. Verified live:
    ///   `num2words(1.23456, lang="as", precision=1)` is unchanged at six
    ///   fractional words. So `precision_override` is accepted and dropped —
    ///   the same shape as `to_year`'s `longval` (bug 5) and `to_currency`'s
    ///   `adjective` (bug 10).
    /// * `FloatValue::precision` is likewise unused: the digit count comes
    ///   from `repr`, not from a precision field.
    /// * Every digit after the point is spoken separately, with no cap, so
    ///   `0.000001` is six words and `98746251323029.99` (Decimal) keeps full
    ///   precision for free — the `float()` cast of issue #603 never happens
    ///   because the Decimal arm stringifies instead of converting.
    ///
    /// The `< 0.01` artefact heuristic that `float2tuple` needs is absent here
    /// and *not* missing: `repr(2.675)` is `"2.675"`, so the
    /// `674.9999999999998` it exists to repair is never computed. See the
    /// module docs.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // precision= is set on the converter by __init__.py, then ignored by
        // Num2Word_AS.to_cardinal. Reproduce the ignoring.
        let _ = precision_override;
        let n = match value {
            // Python's str(float). The raw f64 crosses the boundary precisely
            // so that repr() can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, and never routed through f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    /// Python: `return self.to_cardinal(number) + " -তম"`. No negative guard,
    /// no zero guard — see bugs 3 and 4.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", self.to_cardinal(value)?, ORDINAL_SUFFIX))
    }

    /// Python: `return str(number) + "-তম"`. The raw decimal string, so a
    /// negative keeps its minus: `-1` → "-1-তম".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, ORDINAL_NUM_SUFFIX))
    }

    /// Python: `return "চন " + self.to_cardinal(val)`. `longval` is accepted
    /// and ignored; negatives are not special-cased (bug 5).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", YEAR_PREFIX, self.to_cardinal(value)?))
    }

    // ---- currency ------------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_AS"
    }

    /// `CURRENCY_FORMS[code]`. Reached only by the inherited `to_cheque`;
    /// `to_currency` consults the field directly because it needs
    /// `.get(currency, CURRENCY_FORMS["INR"])` semantics, which this
    /// `Option`-returning hook cannot express.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_AS.to_currency` — a full override that shares no code with
    /// `Num2Word_Base.to_currency`.
    ///
    /// ```text
    /// try:
    ///     left, right, is_negative = self.parse_currency(val)
    /// except AttributeError:
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///     left, right = self._split_currency(val)
    /// cr1, cr2 = self.CURRENCY_FORMS.get(currency, self.CURRENCY_FORMS["INR"])
    /// ...
    /// ```
    ///
    /// # Faithfully reproduced Python bugs
    ///
    /// 6. **`parse_currency` does not exist.** No class in the package defines
    ///    it — `Num2Word_Base` imports `parse_currency_parts`, which is a
    ///    different, module-level function. So the `try` block raises
    ///    `AttributeError` on *attribute lookup*, every single call, and the
    ///    `except` arm is the only live path. The `try` is dead scaffolding
    ///    (`lang_BA`, `lang_BO` and `lang_EU` carry the same copy-paste).
    ///    Verified: `hasattr(Num2Word_AS(), "parse_currency")` is `False`.
    /// 7. **The separator eats the space before the cents word.** Python does
    ///    `result += separator + cents_str`, and `separator` (" আৰু") has no
    ///    trailing space, so `12.34` → "বাৰ ইউৰো আৰুত্ৰিশ চাৰি চেণ্ট" — the
    ///    "and" is glued to the "thirty". Every float row in the corpus pins
    ///    this.
    /// 8. **`CURRENCY_FORMS` misses are silently INR.** Unlike base (and
    ///    unlike this class's own `to_cheque`), the lookup is a `.get(code,
    ///    CURRENCY_FORMS["INR"])`, so `to_currency` **never raises
    ///    NotImplementedError**: `GBP`, `CHF`, `CNY` and friends all come out
    ///    as rupees/paisa. `to_cheque` on those same codes *does* raise —
    ///    corpus rows `currency:GBP` (ok) vs `cheque:GBP` (NotImplementedError)
    ///    sit side by side.
    /// 9. **`CURRENCY_PRECISION` is never consulted.** `JPY` therefore gets a
    ///    subunit it does not have (`12.34` → "…আৰুত্ৰিশ চাৰি পইচা") and
    ///    `KWD`/`BHD` get two mil digits instead of three. Base's
    ///    zero-decimal rounding never runs.
    /// 10. **`adjective` is accepted and ignored** — `CURRENCY_ADJECTIVES` is
    ///    empty for AS anyway, so base would have ignored it too.
    ///
    /// `_int_to_word` (not `to_cardinal`) renders both halves, so the sign is
    /// applied exactly once, at the end, via the raw `negword` — which already
    /// carries its trailing space.
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
        let _ = adjective; // bug 10: accepted, never read.

        // Restore AS's own `separator=" আৰু"` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // Bug 6: the `except AttributeError` arm is the only reachable one.
        let is_negative = val.is_negative();
        let (left, right) = match val {
            // str(int) has no ".", so _split_currency returns (abs(val), 0)
            // and the cents clause is dropped — the int/float split base.py
            // gets from `isinstance(val, int)`, arrived at the long way round.
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            // `abs` is unconditional here where Python guards it with
            // `if val < 0`; identical for non-negatives, and for -0.0 (which
            // is not `< 0`) Python's own str()/int() round-trip yields 0 too.
            CurrencyValue::Decimal { value: d, .. } => split_currency(&d.abs())?,
        };

        // Bug 8: unknown code -> INR, no error. "INR" is inserted by `new`, so
        // the fallback lookup cannot miss (Python would KeyError if it could).
        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            None => self
                .currency_forms
                .get("INR")
                .expect("CURRENCY_FORMS[\"INR\"] is populated by LangAs::new"),
        };
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        let one = BigInt::from(1);
        let left_str = checked_int_to_word(&left)?;
        // `if cents and right` — a zero `right` is falsy, so 1.0 shows no cents.
        let cents_str = if cents && !right.is_zero() {
            checked_int_to_word(&right)?
        } else {
            String::new()
        };

        let mut result = format!(
            "{} {}",
            left_str,
            if left != one { &cr1[1] } else { &cr1[0] }
        );

        if !cents_str.is_empty() {
            // Bug 7: separator and cents_str are concatenated with no space.
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right != one { &cr2[1] } else { &cr2[0] });
        }

        // `self.negword + result if is_negative else result` — the conditional
        // binds looser than `+`, so negword prefixes the whole string.
        Ok(if is_negative {
            format!("{}{}", NEGWORD, result)
        } else {
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn flt(v: f64) -> String {
        // `precision` is what the shim derives from repr() on the Python side.
        // Num2Word_AS ignores it, so any value here is equally correct; pass
        // the honest one.
        let precision = py_float_repr(v)
            .split_once('.')
            .map(|(_, f)| f.len() as u32)
            .unwrap_or(0);
        LangAs::new()
            .to_cardinal_float(&FloatValue::Float { value: v, precision }, None)
            .unwrap()
    }

    fn dec(s: &str) -> String {
        let value = BigDecimal::from_str(s).unwrap();
        let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
        LangAs::new()
            .to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    /// Every `"lang": "as", "to": "cardinal"` corpus row whose arg has a dot.
    #[test]
    fn corpus_float_rows() {
        assert_eq!(flt(0.0), "শূন্য দশমিক শূন্য");
        assert_eq!(flt(0.5), "শূন্য দশমিক পাঁচ");
        assert_eq!(flt(1.0), "এক দশমিক শূন্য");
        assert_eq!(flt(1.5), "এক দশমিক পাঁচ");
        assert_eq!(flt(2.25), "দুই দশমিক দুই পাঁচ");
        assert_eq!(flt(3.14), "তিনি দশমিক এক চাৰি");
        assert_eq!(flt(0.01), "শূন্য দশমিক শূন্য এক");
        assert_eq!(flt(0.1), "শূন্য দশমিক এক");
        assert_eq!(flt(0.99), "শূন্য দশমিক নয় নয়");
        assert_eq!(flt(1.01), "এক দশমিক শূন্য এক");
        assert_eq!(flt(12.34), "বাৰ দশমিক তিনি চাৰি");
        assert_eq!(flt(99.99), "নব্বই নয় দশমিক নয় নয়");
        assert_eq!(flt(100.5), "এক শ দশমিক পাঁচ");
        assert_eq!(flt(1234.56), "এক হাজাৰ দুই শত্ৰিশ চাৰি দশমিক পাঁচ ছয়");
        assert_eq!(flt(-0.5), "ঋণাত্মক শূন্য দশমিক পাঁচ");
        assert_eq!(flt(-1.5), "ঋণাত্মক এক দশমিক পাঁচ");
        assert_eq!(flt(-12.34), "ঋণাত্মক বাৰ দশমিক তিনি চাৰি");
        // The two f64-artefact cases. Right answer, but via repr(), not via
        // float2tuple's `< 0.01` rescue — see the module docs.
        assert_eq!(flt(1.005), "এক দশমিক শূন্য শূন্য পাঁচ");
        assert_eq!(flt(2.675), "দুই দশমিক ছয় সাত পাঁচ");
    }

    /// Every `"lang": "as", "to": "cardinal_dec"` corpus row.
    #[test]
    fn corpus_decimal_rows() {
        assert_eq!(dec("0.01"), "শূন্য দশমিক শূন্য এক");
        // Trailing zero survives: str(Decimal("1.10")) == "1.10", two digits.
        assert_eq!(dec("1.10"), "এক দশমিক এক শূন্য");
        assert_eq!(dec("12.345"), "বাৰ দশমিক তিনি চাৰি পাঁচ");
        // Issue #603's value: exact at trillion scale, no float() cast.
        assert_eq!(
            dec("98746251323029.99"),
            "নব্বই আঠ লাখ সত্তৰ চাৰি হাজাৰ ছয় শবিশ পাঁচ কোটি তেৰ লাখ \
             বিশ তিনি হাজাৰ বিশ নয় দশমিক নয় নয়"
        );
        assert_eq!(dec("0.001"), "শূন্য দশমিক শূন্য শূন্য এক");
    }

    /// -0.0 is not `< 0`, but its str() starts with '-', and to_cardinal
    /// strips the sign textually. Base's float path would drop the negword.
    #[test]
    fn negative_zero_keeps_negword() {
        assert_eq!(flt(-0.0), "ঋণাত্মক শূন্য দশমিক শূন্য");
        assert_eq!(flt(0.0), "শূন্য দশমিক শূন্য");
    }

    /// precision= is honoured by __init__.py and then ignored by the converter.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangAs::new();
        let v = FloatValue::Float {
            value: 1.23456,
            precision: 5,
        };
        let want = "এক দশমিক দুই তিনি চাৰি পাঁচ ছয়";
        assert_eq!(l.to_cardinal_float(&v, None).unwrap(), want);
        for p in [0u32, 1, 2, 5, 9] {
            assert_eq!(l.to_cardinal_float(&v, Some(p)).unwrap(), want);
        }
    }

    /// str(float) goes exponential outside [1e-4, 1e16), and int() chokes.
    /// Which literal lands in the message depends on whether a '.' split the
    /// string first.
    #[test]
    fn exponent_notation_raises_value_error() {
        let l = LangAs::new();
        let f = |v: f64| {
            l.to_cardinal_float(&FloatValue::Float { value: v, precision: 0 }, None)
                .unwrap_err()
        };
        // No '.' in "1e+16" -> the whole string reaches int().
        assert!(matches!(f(1e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e+16'"));
        assert!(matches!(f(1e21), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e+21'"));
        assert!(matches!(f(1e-5), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: '1e-05'"));
        // "1.5e+16" splits: int("1") is fine, then the digit loop hits 'e'.
        assert!(matches!(f(1.5e16), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'e'"));
        assert!(matches!(f(f64::INFINITY), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'inf'"));
        assert!(matches!(f(f64::NAN), N2WError::Value(m)
            if m == "invalid literal for int() with base 10: 'nan'"));
        // Decimal's exponent form is uppercase, and its own threshold differs.
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

    /// The last value below 1e16 still renders in fixed notation, so it is the
    /// largest float this path can speak. 1e15 -> "1000000000000000.0".
    #[test]
    fn large_and_small_boundaries() {
        assert_eq!(flt(1e13), "দহ লাখ কোটি দশমিক শূন্য");
        // From 10^14 কোটি would stack: the #203 ceiling.
        assert!(matches!(
            LangAs::new().to_cardinal_float(&FloatValue::Float { value: 1e15, precision: 1 }, None),
            Err(N2WError::Overflow(_))
        ));
        assert_eq!(flt(0.0001), "শূন্য দশমিক শূন্য শূন্য শূন্য এক");
        // Decimal keeps fixed notation down to 1e-6 (leftdigits > -6).
        assert_eq!(
            dec("0.000001"),
            "শূন্য দশমিক শূন্য শূন্য শূন্য শূন্য শূন্য এক"
        );
        // Decimal("1.00E+2") normalises to str "100": no dot, no pointword.
        assert_eq!(dec("1.00E+2"), "এক শ");
        assert_eq!(dec("0.00"), "শূন্য দশমিক শূন্য শূন্য");
        assert_eq!(dec("-0.5"), "ঋণাত্মক শূন্য দশমিক পাঁচ");
        assert_eq!(dec("-12.34"), "ঋণাত্মক বাৰ দশমিক তিনি চাৰি");
    }

    /// repr() reproduction, including the tie cases where Rust's `{:e}` and
    /// Gay's dtoa disagree. Exhaustively fuzzed against CPython separately.
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
            (1e21, "1e+21"),
            (1e100, "1e+100"),
            (0.0001, "0.0001"),
            (1e-5, "1e-05"),
            (5e-324, "5e-324"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (98746251323029.99, "98746251323029.98"),
            // Exact ties: the true values are ...398.125 and ...775.25, and
            // dtoa rounds the last digit to even. Rust's {:e} rounds up.
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
            assert_eq!(py_decimal_str(&BigDecimal::from_str(s).unwrap()), want, "{}", s);
        }
    }
}
