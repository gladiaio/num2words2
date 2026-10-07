//! Port of `lang_LUS.py` (Mizo / Lushai; also registered under the `miz` alias).
//!
//! Shape: **self-contained**. `Num2Word_LUS` subclasses `Num2Word_Base` but
//! defines no `high_numwords` / `mid_numwords` / `low_numwords`, so the
//! `any(hasattr(...))` guard in `Num2Word_Base.__init__` never fires: Python
//! never builds `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is
//! overridden outright and drives a hand-written `_int_to_word` recursion.
//! Consequently `cards` / `merge` stay at their trait defaults here. Python has
//! **no overflow check** — see bug 1 below for what it did past the table's
//! ceiling; this port raises `OverflowError` from 10^9 (`maxval`).
//!
//! Number system: pure decimal, no teens table. Tens and ones are joined with
//! `" leh "` ("and"), as are hundreds and their remainder — but thousands and
//! millions join their remainder with a **bare space**, not `" leh "`. That
//! asymmetry is Python's, and it is why 101 is "pakhat za leh pakhat" while
//! 1001 is "pakhat sang pakhat".
//!
//! Inherited from `Num2Word_Base` and left alone by LUS:
//!   * `to_year(value, **kwargs) -> self.to_cardinal(value)`. LUS re-declares
//!     it as `to_year(self, val, longval=True)`, but the body is identical to
//!     the base, so the trait default would have sufficed. It is spelled out
//!     below anyway to mirror the source. Note this base has **no BC/AD era
//!     handling at all**, so a negative year is simply a negative cardinal:
//!     `to_year(-44)` == "phak sawmli leh pali" (corpus-confirmed).
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Each of these is what CPython actually
//! emits, and each is corpus-confirmed:
//!
//! 1. **10^9 and up (fixed, gladiaio/num2words2#147).** Python's cascade
//!    stops at `number < 1000000000`; anything `>= 10**9` hit the final
//!    `return str(number)`, so `to_cardinal(10**9)` was "1000000000" — a
//!    *string of digits*, silently. No published source gives a Mizo word
//!    for 10^9, and the module's "million" is itself wrong: Wiktionary has
//!    `nuai` as 10^5 (lakh) and `maktaduai` as 10^6, so this table's `nuai`
//!    already under-counts by ten (an open problem, not touched here).
//!    Rather than stack a new word on that, this port reports `maxval` ==
//!    10^9 and raises `OverflowError` there, on every path (`miz` too).
//! 2. **The digit fallback leaked into the ordinal and the negative forms.**
//!    `to_ordinal` is `to_cardinal(n) + "-na"` with no guard, so Python's
//!    `to_ordinal(10**21)` was "1000000000000000000000-na" and
//!    `to_cardinal(-10**9)` "phak 1000000000". Both raise `OverflowError`
//!    now (#147).
//! 3. **`to_ordinal` accepts negatives and zero.** `Num2Word_Base` carries
//!    `errmsg_negord` / "Cannot treat negative num as ordinal", but LUS's
//!    override never consults it, so `to_ordinal(-1)` == "phak pakhat-na" and
//!    `to_ordinal(0)` == "a awmlo-na". No exception. (Contrast `lang_PL`,
//!    which crashes on both.) Similarly `to_ordinal_num(-1)` == "-1-na" —
//!    the minus sign is simply carried through by `str(number)`.
//! 4. **Zero is a phrase, not a word.** `ones[0]` is `""`, so zero is special
//!    cased to the two-word phrase "a awmlo" ("there is none"). The empty
//!    `ones[0]` / `tens[0]` slots are therefore unreachable in the integer
//!    path — every index into them is guaranteed `1..=9`. (`ones[0]` *is*
//!    reachable in the float path via the `or "a awmlo"` fallback, but floats
//!    are out of scope.)
//! 5. **`to_currency` never raises on an unknown code — it silently bills you
//!    in rupees.** Where every other converter does `CURRENCY_FORMS[currency]`
//!    and lets the `KeyError` become a `NotImplementedError`, LUS writes
//!    `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
//!    The fallback is the *first dict value in declaration order* — INR — so
//!    `to_currency(1.0, currency="GBP")` == "pakhat rupee", and `"JPY"`,
//!    `"CHF"`, `"XYZZY"` likewise. All corpus-confirmed: every `currency:*`
//!    row for a code outside {INR, USD, EUR} reads "rupee"/"paisa". Meanwhile
//!    `to_cheque` is inherited and *does* raise for those same codes, so the
//!    two entry points disagree about which currencies exist. See
//!    [`CURRENCY_FORMS_DECL`].
//! 6. **`to_currency` ignores `CURRENCY_PRECISION` entirely.** The override
//!    never consults it and hard-codes a 2-digit subunit slice, so the
//!    0-decimal and 3-decimal special cases in `Num2Word_Base.to_currency`
//!    are dead code here. `to_currency(0.5, currency="JPY")` is
//!    "a awmlo rupee sawmnga paisa" — *not* the "pakhat ..." the base's
//!    `divisor == 1` ROUND_HALF_UP branch would give, and not yen either
//!    (bug 5). Corpus-confirmed for JPY, KWD and BHD alike.
//! 7. **`adjective` is declared and never read.** `to_currency`'s signature
//!    accepts it, but the body has no `CURRENCY_ADJECTIVES` lookup, so it is
//!    inert. (`CURRENCY_ADJECTIVES` is `{}` for LUS anyway, so even the base
//!    would have been a no-op.)
//! 8. **The digit fallback of bug 1 leaked into currency too.** `to_currency`
//!    calls `_int_to_word` directly, so Python's `to_currency(10**10)` was
//!    "10000000000 rupee"; it raises `OverflowError` now (#147).
//!
//! # Exceptions in scope
//!
//! Across `to_cardinal` / `to_ordinal` / `to_year`, LUS raises only the
//! `OverflowError` of its 10^9 ceiling (bug 1, #147): no negative-ordinal
//! guard (bug 3), and no dict or list access that can miss (bug 4).
//! `to_ordinal_num` never raises.
//!
//! The currency surface adds exactly two raising paths, and they are *not*
//! symmetric — see bug 5 below:
//!
//! * [`LangLus::to_cheque`] (inherited: the trait default is a faithful
//!   `Num2Word_Base.to_cheque`) raises `NotImplementedError` for any code
//!   outside `CURRENCY_FORMS`, because the base indexes `CURRENCY_FORMS[...]`
//!   directly. Corpus: `cheque:GBP/JPY/KWD/BHD/CNY/CHF` all raise;
//!   `cheque:EUR/USD/INR` succeed.
//! * [`LangLus::to_currency`] raises `ValueError` only on exponent-notation
//!   input (`str(1e16)` == `"1e+16"`, which `int()` rejects). No corpus row
//!   reaches it. See [`split_currency`] for the limits of reproducing that.
//!
//! `to_fraction` (TypeError) remains out of scope.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `self.negword`. The trailing space is Python's and is load-bearing: the
/// negative branch concatenates `negword + word` with no separator of its own.
const NEGWORD: &str = "phak ";

/// `self.pointword`. Inert in the integer scope — only the float branch of
/// `to_cardinal` reads it. Kept to mirror `setup()`.
const POINTWORD: &str = "decimal";

/// `self.ones`. Index 0 is `""` and is unreachable for integers (see bug 4).
const ONES: [&str; 10] = [
    "", "pakhat", "pahnih", "pathum", "pali", "panga", "paruk", "pasarih", "pariat", "pakua",
];

/// `self.tens`. Index 0 is `""` and is unreachable (see bug 4). Note there is
/// no teens table: 11 is built as `tens[1] + " leh " + ones[1]` == "sawm leh
/// pakhat", not a dedicated word.
const TENS: [&str; 10] = [
    "", "sawm", "sawmhnih", "sawmthum", "sawmli", "sawmnga", "sawmruk", "sawmsarih", "sawmriat",
    "sawmkua",
];

const HUNDRED: &str = "za";
const THOUSAND: &str = "sang";
const MILLION: &str = "nuai";

/// The zero phrase. Python spells this inline in `_int_to_word`.
const ZERO_WORD: &str = "a awmlo";

/// The conjunction joining tens↔ones and hundreds↔remainder. Deliberately
/// *not* used for thousands/millions remainders — that asymmetry is Python's.
const LEH: &str = " leh ";

/// `Num2Word_LUS.CURRENCY_FORMS`, **in source declaration order**.
///
/// The order is load-bearing, not cosmetic. `to_currency` falls back to
/// `list(self.CURRENCY_FORMS.values())[0]` for an unknown code (bug 5), and
/// since Python 3.7 a dict literal iterates in insertion order, that expression
/// resolves to whichever entry is written first — INR. Keeping the table as an
/// ordered array rather than a bare `HashMap` literal ties "the fallback" to
/// "the first declaration" structurally, so the two cannot drift apart.
///
/// `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both inherited unchanged
/// from `Num2Word_Base` (`{}` and `{}`), which is why `currency_adjective` and
/// `currency_precision` are left at their trait defaults (`None` / `100`).
const CURRENCY_FORMS_DECL: [(&str, [&str; 2], [&str; 2]); 3] = [
    ("INR", ["rupee", "rupee"], ["paisa", "paisa"]),
    ("USD", ["dollar", "dollar"], ["cent", "cent"]),
    ("EUR", ["euro", "euro"], ["cent", "cent"]),
];

/// `Num2Word_LUS.to_currency`'s own default `separator=" "`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot be a plain parameter default.
const SEPARATOR_DEFAULT: &str = " ";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_LUS.to_currency` declares `separator=" "`, but the `Lang` trait
/// has no per-language parameter defaults: `__init__.py`'s Rust fast path and
/// `bench/diff_test.py` both send `kwargs.get("separator", ",")` — i.e.
/// `Num2Word_Base`'s default — so by the time the value arrives here, the
/// information needed to tell "unset" from "explicitly a comma" is gone.
///
/// So `,` is read back as the unset sentinel and LUS's own default restored.
/// This is the only reading that matches the oracle: all 54 float rows of the
/// `lus` currency corpus were generated by `num2words(v, lang="lus",
/// to="currency", currency=c)` with no `separator=`, and every one of them
/// expects a plain space ("... euro sawmthum leh pali cent"). Same convention
/// and same sentinel as `lang_as.rs` and `lang_ca.rs`.
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets `" "` here where Python would give `","`. Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// The currency split inlined in `Num2Word_LUS.to_currency`, which is a
/// **string** operation, not an arithmetic one:
///
/// ```text
/// parts = str(val).split(".")            # val is already abs()
/// left  = int(parts[0]) if parts[0] else 0
/// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
/// ```
///
/// `n` is always non-negative — `to_currency` takes `abs` before `str`.
///
/// Three consequences, all corpus-pinned:
///
/// * `right` is the *first two fraction digits, left-justified*, so `0.5` →
///   `"5"` → `"50"` → 50 paisa, while `0.05` → `"05"` → 5. It is a digit
///   slice, not a multiplication: a third decimal is truncated, never rounded
///   (`12.349` → 34).
/// * a fraction of `"0"` (i.e. the float `1.0`) yields `right == 0`, which
///   `to_currency` treats as falsy and drops the cents clause. This is why
///   LUS is the rare converter where the `isinstance(val, int)` distinction
///   `Num2Word_Base.to_currency` turns on is **not observable**: LUS never
///   branches on the type, and `1` and `1.0` both render "pakhat euro" for
///   the same reason (`right == 0`), not for the base's reason.
/// * an int reaches here as a scale-0 `BigDecimal`, whose `str()` has no `"."`
///   at all, so `len(parts) == 1` and `right` stays 0 — the `scale == 0` arm.
///
/// # The exponent-notation hole
///
/// `str(float)` switches to exponent notation at `|v| >= 1e16` and
/// `0 < |v| < 1e-4`, and `int()` then chokes on the literal: Python's
/// `to_currency(1e16)` raises `ValueError: invalid literal for int() with
/// base 10: '1e+16'`. A negative `BigDecimal` scale is the "the source string
/// used `e+` notation" signal (a plain digit string parses to scale 0, never
/// below) for floats and `Decimal`s alike, so that arm is reproduced —
/// matching `lang_as.rs`, which ports the identical Python idiom.
///
/// Two known gaps, both flagged in the port report and both unreachable from
/// the corpus:
///
/// * The `e-` side is not detectable: `1e-05` and `Decimal("0.00001")` parse
///   to the *same* `BigDecimal` (digits 1, scale 5), yet Python raises
///   `ValueError` for the first and returns "a awmlo euro" for the second.
///   The discriminator is the original string, which the `CurrencyValue`
///   boundary does not carry.
/// * `scale < 0` over-raises for a repr with two or more fraction digits
///   ahead of the exponent: `str(1.2345e20)` == `"1.2345e+20"` splits to
///   `["1", "2345e+20"]`, whose `[:2]` slice is the clean digits `"23"`, so
///   Python returns "pakhat euro sawmhnih leh pathum cent" rather than
///   raising. Reproducing *that* needs the source string too.
fn split_currency(n: &BigDecimal) -> Result<(BigInt, BigInt)> {
    // value == digits * 10^-scale
    let (digits, scale) = n.as_bigint_and_exponent();

    if scale < 0 {
        // str(n) would be "1e+16"-shaped and int() rejects it. Python's message
        // quotes the offending literal, which is unrecoverable from the parsed
        // value — the exception *type* is what callers observe.
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
    let s = digits.to_string();
    let scale = scale as usize;
    let (int_part, frac_part) = if s.len() > scale {
        let (a, b) = s.split_at(s.len() - scale);
        (a.to_string(), b.to_string())
    } else {
        // str() renders a leading "0" for a pure fraction: 0.5 → "0.5".
        ("0".to_string(), crate::strnum::zero_pad_left(&s, scale))
    };

    // `int(parts[0]) if parts[0] else 0` — the guard is dead for a repr, which
    // never starts with ".", but the digits are pure ASCII either way.
    let left = int_part.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
    // `parts[1][:2].ljust(2, "0")` — first two chars, then pad *right* with "0".
    let head: String = frac_part.chars().take(2).collect();
    let right = format!("{:0<2}", head)
        .parse::<BigInt>()
        .unwrap_or_else(|_| BigInt::zero());

    Ok((left, right))
}

/// Python's `_int_to_word`.
///
/// Only ever reached with a **non-negative** `number`: `to_cardinal` strips the
/// `"-"` before recursing, and every internal recursion passes a quotient or
/// remainder of a non-negative value. That matters because Python's `divmod`
/// floors while Rust's `div_rem` truncates — on non-negative operands the two
/// agree, but `div_mod_floor` is used below regardless so the correspondence to
/// Python is exact rather than merely incidental.
///
/// A top-level call of 10^9 or more raises `OverflowError` (bug 1, #147);
/// below it, every quotient the recursion passes is under 1000.
fn int_to_word(number: &BigInt) -> Result<String> {
    check_maxval(number, maxval_ceiling())?;
    if number.is_zero() {
        return Ok(ZERO_WORD.to_string());
    }

    let ten = BigInt::from(10u32);
    if number < &ten {
        // 1..=9, so the index is in range and never hits the empty slot.
        return Ok(ONES[small(number)].to_string());
    }

    let hundred = BigInt::from(100u32);
    if number < &hundred {
        let (t, o) = number.div_mod_floor(&ten);
        let mut out = TENS[small(&t)].to_string();
        if !o.is_zero() {
            out.push_str(LEH);
            out.push_str(ONES[small(&o)]);
        }
        return Ok(out);
    }

    let thousand = BigInt::from(1_000u32);
    if number < &thousand {
        let (h, r) = number.div_mod_floor(&hundred);
        // Python: `self.ones[h] + " " + self.hundred`.
        let mut out = format!("{} {}", ONES[small(&h)], HUNDRED);
        if !r.is_zero() {
            // Hundreds *do* take " leh " before their remainder.
            out.push_str(LEH);
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    let million = BigInt::from(1_000_000u32);
    if number < &million {
        let (t, r) = number.div_mod_floor(&thousand);
        let mut out = format!("{} {}", int_to_word(&t)?, THOUSAND);
        if !r.is_zero() {
            // Bare space, not " leh " — hence "pakhat sang pakhat" for 1001.
            out.push(' ');
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    let billion = BigInt::from(1_000_000_000u32);
    if number < &billion {
        let (m, r) = number.div_mod_floor(&million);
        let mut out = format!("{} {}", int_to_word(&m)?, MILLION);
        if !r.is_zero() {
            // Bare space again.
            out.push(' ');
            out.push_str(&int_to_word(&r)?);
        }
        return Ok(out);
    }

    // Bug 1: Python's `return str(number)`, now an OverflowError raised by
    // the maxval check above (#147).
    unreachable!("values >= 10^9 are rejected by check_maxval")
}

/// The exclusive ceiling (gladiaio/num2words2#147): no attested Mizo word for
/// 10^9, and the module's million word is itself off by ten (bug 1).
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(9))
}

/// Narrow a BigInt already proven to be in `0..=9` to a table index.
///
/// Every call site guards with an explicit `< 10` / `div_mod_floor(&ten)` /
/// `div_mod_floor(&hundred)` on a value under 1000, so the quotient or
/// remainder is always a single digit and the conversion cannot fail.
fn small(n: &BigInt) -> usize {
    debug_assert!(!n.is_negative() && n < &BigInt::from(10u32));
    n.to_usize().expect("guarded to 0..=9 by the caller")
}

// ---- The float / Decimal path -----------------------------------------------
//
// `Num2Word_LUS` has a single `to_cardinal` that branches on `str(number)`, so
// the trait's integer/float split lands *inside* one Python method here. The
// whole float path is `str()` + a per-digit walk — `Num2Word_Base.float2tuple`
// is never reached, so the `< 0.01` f64-artefact heuristic it carries is absent
// and *not missing*: `repr(2.675)` is `"2.675"`, so the `674.9999999999998`
// that heuristic exists to repair is never computed. The entire specification
// of the fractional part is therefore `repr(float)` / `str(Decimal)`, which the
// three helpers below reproduce. They are byte-identical to `lang_as.rs`, which
// ports the same Python idiom; the digits themselves are language-independent.

/// dtoa's shortest round-trip digits for `a >= 0`, as `(digits, decpt)` where
/// the value is `0.<digits> * 10**decpt`.
///
/// Rust's `{:e}` is also shortest-round-trip and agrees with Gay's dtoa on the
/// digit *count* and almost always the digits. It disagrees only on **exact
/// ties**: when `a` sits precisely halfway between two shortest candidates,
/// dtoa picks the one with an **even** last digit while Rust rounds half up.
/// The block below detects that tie with no bignum (write `a = m * 2**e`, `m`
/// odd; the tie is `e + q + 1 == 0`, plus `5**-q | m` when `q < 0`) and steps
/// to the even neighbour, matching CPython exactly.
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
            // k is even, so Python wants k and Rust gave k+1. Odd last digit is
            // non-zero, so this never borrows.
            *digits.last_mut().expect("non-empty") -= 1;
        } else {
            // k is odd, so Python wants k+1 and Rust gave k. Carry like dtoa's
            // `roundoff`: "99" -> "1" with decpt bumped.
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

/// Python's `str(float)` (== `repr(float)`), which `Num2Word_LUS.to_cardinal`
/// promotes from a formatting detail to the whole spec of the float path.
///
/// This is CPython's `format_float_short(..., 'r', ...)`: Rust's `{}` cannot
/// stand in because it never switches to exponent notation and prints `1`, not
/// `1.0`, for integral floats. Rules straight from `format_float_short`:
/// exponent notation iff `decpt <= -4 || decpt > 16`; the exponent is
/// `%+.02d`; `Py_DTSF_ADD_DOT_0` appends `.0` to an integral fixed-notation
/// result but never in exponent notation; `nan` drops its sign, `inf` keeps it.
fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    // is_sign_negative, not `< 0.0`: str(-0.0) is "-0.0", and the "-" is
    // stripped textually into a negword downstream.
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
/// the default context (uppercase `E`).
///
/// A `BigDecimal`'s `(int_val, scale)` is exactly `Decimal`'s `(_int, _exp)`
/// with `_exp == -scale`; the shim builds this via `BigDecimal::from_str`, which
/// preserves trailing zeros and negative exponents, so `"1.10"` round-trips as
/// `(110, 2)` and renders "1.10", and `"1E+16"` as `(1, -16)`.
///
/// Known hole: `Decimal("-0.0")` is signed zero (`str` "-0.0"), but a
/// `BigInt` has no negative zero, so the sign is already lost before this
/// function — we emit "0.0". Same shape as the `e-` hole in [`split_currency`];
/// the discriminator is the source string, which the boundary does not carry.
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

/// Python's `int(s)`, for the ASCII fragments [`cardinal_from_str`] hands it.
///
/// Every string that reaches here is a slice of `str(float)` / `str(Decimal)`,
/// so it is ASCII by construction; the non-ASCII-digit generality of the real
/// builtin is unreachable and deliberately not ported. What is ported is the
/// underscore rule and the error message, which formats the original argument
/// with `%.200R` (== `repr`), plain `'{}'` for these quote-free literals.
fn py_int(s: &str) -> Result<BigInt> {
    let err = || N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s));
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

/// The float/`Decimal` body of `Num2Word_LUS.to_cardinal`, driven by
/// `str(number)`:
///
/// ```text
/// n = str(number).strip()
/// if n.startswith("-"):
///     return (self.negword + self.to_cardinal(n[1:])).strip()
/// if "." in n:
///     left, right = n.split(".", 1)
///     ret = self._int_to_word(int(left)) + " " + self.pointword
///     for digit in right:
///         ret += " " + (self.ones[int(digit)] or "a awmlo")
///     return ret.strip()
/// return self._int_to_word(int(n))
/// ```
///
/// The per-digit `self.ones[int(digit)] or "a awmlo"` is exactly [`int_to_word`]
/// restricted to a single `0..=9` digit: `ones[0] == ""` is falsy and falls
/// back to the zero phrase, which is what `int_to_word(0)` returns, and for
/// `1..=9` both are `ones[d]`. So the loop reuses `int_to_word` rather than
/// re-encoding the table. `int(left)` runs **before** the digit loop, and a
/// non-digit char surviving `split(".", 1)` (a second `.`, or an `e` from an
/// exponent-notation repr) makes `int()` raise `ValueError` — reproduced by
/// [`py_int`], including which literal the message quotes.
///
/// The recursion on the `"-"` branch mirrors the source literally; because
/// `int_to_word` never returns leading/trailing whitespace and `negword`'s
/// space is interior, every `.strip()`/`.trim()` here is a no-op, kept only so
/// the correspondence to Python is exact.
fn cardinal_from_str(number: &str) -> Result<String> {
    // n = str(number).strip()
    let n = number.trim();

    // n.startswith("-"): recurse on the magnitude, prepend negword, strip.
    if let Some(rest) = n.strip_prefix('-') {
        let inner = cardinal_from_str(rest)?;
        return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
    }

    // "." in n — the fragments are ASCII, so byte-indexing at the dot is safe.
    match n.find('.') {
        Some(dot) => {
            // n.split(".", 1): maxsplit 1, so `right` keeps any further dots.
            let (left, right) = (&n[..dot], &n[dot + 1..]);
            let mut ret = format!("{} {}", int_to_word(&py_int(left)?)?, POINTWORD);
            // Python iterates *characters* of `right`; index by chars().
            for d in right.chars() {
                let mut buf = [0u8; 4];
                let digit = py_int(d.encode_utf8(&mut buf))?;
                ret.push(' ');
                ret.push_str(&int_to_word(&digit)?);
            }
            Ok(ret.trim().to_string())
        }
        None => Ok(int_to_word(&py_int(n)?)?),
    }
}

pub struct LangLus {
    /// `CURRENCY_FORMS`, built once in [`LangLus::new`]. The registry holds
    /// each language in a `OnceLock`, so this is constructed a single time per
    /// process and then only read.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the unknown-code fallback of
    /// bug 5, precomputed from [`CURRENCY_FORMS_DECL`]`[0]` so it tracks the
    /// declaration order by construction.
    currency_fallback: CurrencyForms,
}

impl Default for LangLus {
    fn default() -> Self {
        Self::new()
    }
}

impl LangLus {
    pub fn new() -> Self {
        let mut currency_forms = HashMap::with_capacity(CURRENCY_FORMS_DECL.len());
        for (code, unit, subunit) in CURRENCY_FORMS_DECL {
            currency_forms.insert(code, CurrencyForms::new(&unit, &subunit));
        }
        let (_, unit0, subunit0) = CURRENCY_FORMS_DECL[0];
        LangLus {
            currency_forms,
            currency_fallback: CurrencyForms::new(&unit0, &subunit0),
        }
    }

    /// `cr[1] if n != 1 else cr[0]`, as `to_currency` spells it inline for
    /// both the unit and the subunit.
    ///
    /// Note this is a direct tuple index, **not** a call to [`LangLus::pluralize`]
    /// — which LUS also defines, and which `to_currency` never reaches. The two
    /// happen to agree here because every LUS form tuple has arity 2, so
    /// `cr[1]` and `pluralize`'s `forms[-1]` are the same slot; and because
    /// both forms of every LUS currency are identical strings, neither choice
    /// is observable at all. Ported literally regardless.
    ///
    /// Indexing is safe: every entry in [`CURRENCY_FORMS_DECL`] is `[&str; 2]`,
    /// and the fallback is built from that same array, so `forms` always has
    /// exactly two elements.
    fn pick_form(forms: &[String], n: &BigInt) -> String {
        if n.is_one() {
            forms[0].clone()
        } else {
            forms[1].clone()
        }
    }
}

impl Lang for LangLus {
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

    /// `to_ordinal(float/Decimal)`. `Num2Word_LUS.to_ordinal` is
    /// `self.to_cardinal(number) + "-na"` with **no type guard** — bug 3's
    /// accept-anything behaviour extends to floats — so a float or Decimal
    /// rides the same digit-by-digit decimal grammar and then takes the
    /// suffix: `to_ordinal(5.0)` == "panga decimal a awmlo-na",
    /// `to_ordinal(-0.0)` == "phak a awmlo decimal a awmlo-na", and a whole
    /// `Decimal("100")` == "pakhat za-na". An exponent-notation repr
    /// (`str(1e16)` == "1e+16", `str(Decimal("1E+2"))` == "1E+2") makes the
    /// inner `int()` raise its ValueError *before* Python's `+ "-na"` runs —
    /// the `?` reproduces that ordering.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        Ok(format!("{}-na", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "-na"` never casts to
    /// int, so it *succeeds* on every float and Decimal — "5.0-na",
    /// "-0.0-na", "1e+16-na" and "1E+2-na" are all real Python outputs (the
    /// float extension of bug 3's sign passthrough). `repr_str` is the
    /// binding's Python `str(value)`, exactly the string Python concatenates.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}-na", repr_str))
    }

    /// `Decimal('Infinity')` / `-Infinity` from a string arg. LUS inherits
    /// `Num2Word_Base`'s `str_to_number` (`Decimal(value)`), so the token parses
    /// *successfully* and only blows up later inside each mode:
    ///
    /// * `to_cardinal` / `to_year` reach `int(str(number))` == `int("Infinity")`
    ///   → `ValueError`. `-Infinity` strips its sign and recurses on "Infinity",
    ///   so the message always quotes `'Infinity'`.
    /// * `to_ordinal` == `to_cardinal(number) + "-na"` → the `int()` raises
    ///   before the suffix, so `ValueError` again.
    /// * `to_ordinal_num` == `str(number) + "-na"` never casts, so it *answers*:
    ///   `"Infinity-na"` / `"-Infinity-na"`.
    ///
    /// Byte-exact against the pure-Python oracle (verified live).
    fn inf_result(&self, negative: bool, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok(format!(
                "{}Infinity-na",
                if negative { "-" } else { "" }
            )),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
        }
    }

    /// `Decimal('NaN')` from a string arg. Same shape as [`LangLus::inf_result`]:
    /// `int("NaN")` → `ValueError` on the cardinal/ordinal/year paths, while
    /// `to_ordinal_num` answers `"NaN-na"`.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "ordinal_num" => Ok("NaN-na".into()),
            _ => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
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
        " "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "decimal"
    }

    /// `setup()` sets `exclude_title = ["leh", "phak", "decimal"]`, but
    /// `is_title` stays `False` (set by `Num2Word_Base.__init__`) and LUS's
    /// `to_cardinal` never routes through `self.title()`. So this is inert —
    /// reproduced only for fidelity to `setup()`.
    fn exclude_title(&self) -> &[String] {
        static EXCL: OnceLock<Vec<String>> = OnceLock::new();
        EXCL.get_or_init(|| {
            vec!["leh".to_string(), "phak".to_string(), "decimal".to_string()]
        })
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     return (self.negword + self.to_cardinal(n[1:])).strip()
    /// ...
    /// return self._int_to_word(int(n))
    /// ```
    ///
    /// The Python version round-trips through `str()` and recurses on the
    /// *string* `n[1:]`, which for an integer is exactly the decimal digits of
    /// the absolute value — so recursing on `value.abs()` is equivalent. The
    /// `"." in n` branch is unreachable for integer input (out of scope).
    ///
    /// The trailing `.strip()` is a no-op on the interior space of
    /// `"phak " + word`: `str.strip()` only touches the ends, and `int_to_word`
    /// never returns a value that is empty or space-padded. It is kept so the
    /// correspondence to the source is literal.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.to_cardinal(&value.abs())?;
            return Ok(format!("{}{}", NEGWORD, inner).trim().to_string());
        }
        Ok(int_to_word(value)?)
    }

    /// Python: `return self.to_cardinal(number) + "-na"`.
    ///
    /// No negative guard and no float guard — see bugs 2 and 3.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-na", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "-na"`. The sign rides along: "-1-na".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-na", value))
    }

    /// Python: `to_year(self, val, longval=True)` → `self.to_cardinal(val)`.
    /// `longval` is accepted and ignored; identical to the base's `to_year`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The `"." in n` (and `Decimal`) branch of `Num2Word_LUS.to_cardinal`.
    /// Python has a single `to_cardinal` that splits on `str(number)`, so this
    /// is the same method as [`LangLus::to_cardinal`] reached with a
    /// non-integer. **Neither `Num2Word_Base.to_cardinal_float` nor
    /// `float2tuple` runs** — the fractional part is spelled out digit by digit
    /// straight from `repr`, so the `< 0.01` f64-artefact heuristic is absent
    /// and not missing (`repr(2.675) == "2.675"`).
    ///
    /// `precision=` is accepted by `__init__.py` and set on the converter, but
    /// `to_cardinal` never reads it (verified live:
    /// `num2words(1.23456, lang="lus", precision=1)` is unchanged at six
    /// fractional words). So `precision_override` is dropped — the same shape as
    /// `to_year`'s `longval` and `to_currency`'s `adjective` (bug 7).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // precision= is set on the converter, then ignored. Reproduce that.
        let _ = precision_override;
        let n = match value {
            // Python's str(float): the raw f64 crosses the boundary precisely so
            // that repr() can be reproduced from the bits.
            FloatValue::Float { value, .. } => py_float_repr(*value),
            // Python's str(Decimal) — exact, scale-preserving, never via f64.
            FloatValue::Decimal { value, .. } => py_decimal_str(value),
        };
        cardinal_from_str(&n)
    }

    // ---- currency ------------------------------------------------------

    /// `self.__class__.__name__`, quoted in the inherited `to_cheque`'s
    /// `NotImplementedError`.
    fn lang_name(&self) -> &str {
        "Num2Word_LUS"
    }

    /// `CURRENCY_FORMS.get(code)` — a *plain* lookup, `None` for a miss.
    ///
    /// This is the honest table, and it is what the inherited `to_cheque`
    /// needs: the base does `self.CURRENCY_FORMS[currency]` and converts the
    /// `KeyError` into `NotImplementedError`, which is exactly the corpus's
    /// `cheque:GBP` → NotImplementedError. LUS's own `to_currency` does *not*
    /// go through this contract — it applies the INR fallback of bug 5 on top,
    /// which is why that override does its own lookup.
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
    /// Dead code in practice: `to_currency` is overridden and indexes the form
    /// tuples directly (see [`LangLus::pick_form`]), and the inherited
    /// `to_cheque` takes `cr1[-1]` unconditionally. Nothing in the currency
    /// surface calls it. Ported because it is a real override, and because
    /// leaving the trait default in place would raise `NotImplementedError`
    /// from a method Python answers.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.is_empty() {
            return Ok(String::new());
        }
        Ok(if n.is_one() {
            forms[0].clone()
        } else {
            forms[forms.len() - 1].clone()
        })
    }

    /// Python:
    /// ```python
    /// def to_currency(self, val, currency="INR", cents=True,
    ///                 separator=" ", adjective=False):
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
    /// A wholesale override that shares nothing with `Num2Word_Base.to_currency`:
    /// no `parse_currency_parts`, no `pluralize`, no `CURRENCY_PRECISION`
    /// (bug 6), no `CURRENCY_ADJECTIVES` (bug 7), no `KeyError` → `NotImplemented`
    /// (bug 5), and no `isinstance(val, int)` branch at all.
    ///
    /// Note `cents=False` here **drops the subunit clause entirely** rather
    /// than switching to the terse `_cents_terse` digits the base would use:
    /// the flag is only ever read as `if cents and right`. So `_cents_verbose`
    /// / `_cents_terse` / `_money_verbose` are all unreachable for LUS — the
    /// body calls `_int_to_word` directly, bypassing the `_money_verbose` hook
    /// that `to_cheque` still routes through.
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
        // Bug 7: Python binds `adjective` and never reads it.
        let _ = adjective;
        // See SEPARATOR_UNSET: `,` is the binding's "caller omitted it" marker.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        // `is_negative = val < 0` then `val = abs(val)`, so every string
        // operation downstream sees the magnitude only.
        let is_negative = val.is_negative();
        let abs_val = match val {
            CurrencyValue::Int(v) => BigDecimal::from(v.abs()),
            CurrencyValue::Decimal { value: d, .. } => d.abs(),
        };
        let (left, right) = split_currency(&abs_val)?;

        // Bug 5: a miss falls back to the first declared entry (INR) instead
        // of raising, so this never returns an error.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);

        let mut result = format!(
            "{} {}",
            int_to_word(&left)?,
            LangLus::pick_form(&forms.unit, &left)
        );

        // `if cents and right:` — `right == 0` is falsy, which is what makes
        // the float `1.0` render "pakhat euro" with no cents clause.
        if cents && !right.is_zero() {
            result.push_str(separator);
            result.push_str(&format!(
                "{} {}",
                int_to_word(&right)?,
                LangLus::pick_form(&forms.subunit, &right)
            ));
        }

        if is_negative {
            // `self.negword + result` — NEGWORD's trailing space is the only
            // separator, exactly as in `to_cardinal`.
            result = format!("{}{}", NEGWORD, result);
        }
        // `.strip()`: a no-op in practice (no form is empty, and `int_to_word`
        // never pads), kept so the correspondence to the source is literal.
        Ok(result.trim().to_string())
    }
}
