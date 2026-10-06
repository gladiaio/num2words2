//! Port of `lang_TR.py` (Turkish).
//!
//! `Num2Word_TR` subclasses `Num2Word_Base` but overrides `to_cardinal` /
//! `to_ordinal` / `to_splitnum` with its own digit-string algorithm, so this
//! is a *self-contained* language: `cards`/`merge` are never consulted.
//! `to_year` is not overridden in Python, so it falls through to
//! `Num2Word_Base.to_year`, which is just `to_cardinal`.
//!
//! Turkish number words are written solid (no spaces): 1234 →
//! "binikiyüzotuzdört". The Python algorithm walks the decimal digit string
//! triplet by triplet with a thicket of special cases; this file transliterates
//! that control flow branch for branch, including its bugs (see below).
//!
//! Faithfully reproduced Python quirks — do NOT "fix" these:
//!
//! * `ORDINAL_TRIPLETS[6]` is `"kentilyon"`, not `"kentilyonuncu"` — every
//!   other entry carries the ordinal suffix. So `to_ordinal(10**18)` returns
//!   "birkentilyon" (a cardinal), and `to_ordinal_num(10**18)` returns
//!   "1000000000000000000'" with an empty suffix.
//! * `to_cardinal` casts its argument to `float` before splitting digits
//!   (`_to_cardinal_impl`), while `to_ordinal` does not. Above 2^53 the two
//!   therefore disagree: `to_cardinal(1234567890123456789)` reads ...768 but
//!   `to_ordinal` of the same reads ...789. `float_round_digits` reproduces
//!   the double rounding exactly. See `python_float_int`.
//! * Fixed (gladiaio/num2words2#159): because of that float cast, the 65535
//!   inputs in `10**21 - 65536 ..= 10**21 - 2` (all below Python's MAXVAL =
//!   10**21 - 1, so they passed `verify_cardinal`) round *up* to the 22-digit
//!   "1000...0" and index `CARDINAL_TRIPLETS[7]`, which Python raised
//!   `KeyError` for. The port lowers MAXVAL to `10**21 - 65536`, the real
//!   exclusive ceiling, so those inputs are an `OverflowError` and
//!   `maxval("tr") - 1` converts. Python's message also named MAXVAL itself as
//!   "the largest convertible number" while rejecting it; the port names
//!   `MAXVAL - 1`.
//! * `verify_ordinal` raises `TypeError(errmsg_negord)` *inside* its own
//!   `try`, which its `except (ValueError, TypeError)` then swallows and
//!   re-raises as `TypeError(errmsg_nonnum)`. Negative ordinals therefore
//!   report the "not a number" message, not the "not positive" one.
//! * The issue-#64 fix (keep "bir" in e.g. 401607 → "dörtyüzbirbinaltıyüzyedi")
//!   was applied to `to_cardinal` only. `to_ordinal` keeps the old
//!   `not (T == 2 and d[2] == '1')` guard and still drops it, so
//!   `to_ordinal(401607)` is "dörtyüzbinaltıyüzyedinci".
//! * `verify_ordinal` does NOT raise on a non-integral value — it returns
//!   `isordinal = False` and `to_ordinal` then returns its pristine `wrd`,
//!   the empty string: Python's `to_ordinal(0.5)` is `""`. The port raises
//!   TypeError there instead, like every language whose ordinal rejects
//!   fractions (#160); `to_ordinal_num` still sees the `""`. Negatives are flagged by
//!   the *numeric* `abs(value) == value` (so -0.0 passes and reads
//!   "sıfırıncı") and re-raised as `TypeError(errmsg_nonnum)`; the MAXVAL
//!   check still applies to non-integral values, so a huge non-whole
//!   Decimal is OverflowError, not `""`. See `verify_ordinal_float`.
//! * `to_ordinal_num` recovers the vowel-harmonised suffix by stripping
//!   `to_cardinal(value)` off `to_ordinal(value)`; when that fails it keeps
//!   the last four chars of the ordinal. For a non-integral value the
//!   ordinal is `""` (above), the strip fails, and `""[-4:]` is `""` —
//!   hence `to_ordinal_num(0.5)` == `"0.5'"`. And since
//!   `ORDINAL_TRIPLETS[6]` lacks its suffix, `to_ordinal_num(1e20)` is
//!   `"1e+20'"` (cardinal == ordinal == "yüzkentilyon", empty suffix).
//! * `str_to_number` is Base's plain `Decimal(value)`. `Decimal("NaN")`
//!   parses, then `verify_cardinal` dies on `abs(value) >= self.MAXVAL`
//!   with `decimal.InvalidOperation` — not the int()-ValueError the
//!   binding's generic NaN mapping assumes — while `verify_ordinal` turns
//!   `int(NaN)`'s ValueError into `TypeError(errmsg_nonnum)`. A NaN cannot
//!   ride `ParsedNumber` (BigDecimal has no NaN), so the override returns
//!   NotImplemented and the shim reruns the original Python string path,
//!   which owns every mode's exact outcome. Infinity stays on the generic
//!   mapping: OverflowError, the same type TR raises via the MAXVAL compare.
//!
//! Grammatical kwargs (issue #64 part 2/3, upstream #486 + #534):
//! `to_cardinal(value, spaced=False, precision=None, decimal_word=None)`.
//! `precision` is assigned to `self.precision` (used only by the float
//! grammar — the integer reader never consults it); `decimal_word` swaps
//! `self.pointword` for the call; `spaced=True` re-tokenizes the solid
//! output with `_insert_spaces`, a greedy longest-first tokenizer whose
//! token set includes the *current* pointword (i.e. the caller's
//! decimal_word) but NOT the negword — "eksi" survives only because its
//! letters pass through the unknown-char branch verbatim. `to_year` is
//! inherited from `Num2Word_Base` as `to_year(value, **kwargs)`: every
//! kwarg is swallowed unread and the result is plain `to_cardinal(value)`.
//! `to_ordinal`/`to_ordinal_num`/`to_currency` accept no extra kwargs
//! (trait defaults fall back to Python's own TypeError).
//!
//! Currency:
//!
//! * Fixed (gladiaio/num2words2#187): Python's `to_currency` short-circuits
//!   on a *pure* `int`, ignores `currency=`, and glues `CURRENCY_UNIT`
//!   ("lira") and the bare negword on with no spaces ("kırkikilira",
//!   "eksibeşlira"), while every other input went to `Num2Word_Base` with
//!   the signature's default `currency="EUR"` ("kırkiki avro, sıfır sent").
//!   The port sends ints through `Num2Word_Base` too, so `currency=` is
//!   honoured (an unknown code raises `NotImplementedError` like a float),
//!   the number words and the unit are separated by a space even though
//!   cardinals stay unspaced, and an int has no cents segment, as in Base:
//!   42 -> "kırkiki lira". The default is TRY for both paths — the int
//!   path's lira, and the unit every currency test without `currency=`
//!   expects.
//! * `pluralize` is `forms[0] if number == 1 else forms[0]` — a no-op ternary.
//!   Turkish does not inflect the currency name, so `number` is dead.

use crate::base::{KwVal, Kwargs, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::{BigDecimal, RoundingMode};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

const ZERO: &str = "sıfır";
const ZEROTH: &str = "sıfırıncı";
const NEGWORD: &str = "eksi";
const CARDINAL_HUNDRED: &str = "yüz";
const ORDINAL_HUNDRED: &str = "yüzüncü";

const ERRMSG_NONNUM: &str = "Sadece sayılar yazıya çevrilebilir.";

fn errmsg_toobig(value: &BigInt, maxval: &BigInt) -> String {
    format!(
        "abs({}) sayı yazıya çevirmek için çok büyük. \
         Yazıya çevrilebilecek en büyük rakam {}.",
        value,
        maxval - 1u32
    )
}

// The `_ => ""` arms stand in for Python's `dict.get(key, "")`: "0" is absent
// from every table, and "1" is additionally absent from HUNDREDS (which is why
// 100 reads "yüz", not "biryüz").

fn cardinal_ones(d: u8) -> &'static str {
    match d {
        b'1' => "bir",
        b'2' => "iki",
        b'3' => "üç",
        b'4' => "dört",
        b'5' => "beş",
        b'6' => "altı",
        b'7' => "yedi",
        b'8' => "sekiz",
        b'9' => "dokuz",
        _ => "",
    }
}

fn ordinal_ones(d: u8) -> &'static str {
    match d {
        b'1' => "birinci",
        b'2' => "ikinci",
        b'3' => "üçüncü",
        b'4' => "dördüncü",
        b'5' => "beşinci",
        b'6' => "altıncı",
        b'7' => "yedinci",
        b'8' => "sekizinci",
        b'9' => "dokuzuncu",
        _ => "",
    }
}

fn cardinal_tens(d: u8) -> &'static str {
    match d {
        b'1' => "on",
        b'2' => "yirmi",
        b'3' => "otuz",
        b'4' => "kırk",
        b'5' => "elli",
        b'6' => "altmış",
        b'7' => "yetmiş",
        b'8' => "seksen",
        b'9' => "doksan",
        _ => "",
    }
}

fn ordinal_tens(d: u8) -> &'static str {
    match d {
        b'1' => "onuncu",
        b'2' => "yirminci",
        b'3' => "otuzuncu",
        b'4' => "kırkıncı",
        b'5' => "ellinci",
        b'6' => "altmışıncı",
        b'7' => "yetmişinci",
        b'8' => "sekseninci",
        b'9' => "doksanıncı",
        _ => "",
    }
}

/// Python's `HUNDREDS` — deliberately has no "1" key.
fn hundreds(d: u8) -> &'static str {
    match d {
        b'2' => "iki",
        b'3' => "üç",
        b'4' => "dört",
        b'5' => "beş",
        b'6' => "altı",
        b'7' => "yedi",
        b'8' => "sekiz",
        b'9' => "dokuz",
        _ => "",
    }
}

/// `CARDINAL_TRIPLETS[i]`. Python raises `KeyError` for i outside 1..=6.
fn cardinal_triplet(i: usize) -> Result<&'static str> {
    match i {
        1 => Ok("bin"),
        2 => Ok("milyon"),
        3 => Ok("milyar"),
        4 => Ok("trilyon"),
        5 => Ok("katrilyon"),
        6 => Ok("kentilyon"),
        _ => Err(N2WError::Key(i.to_string())),
    }
}

/// `ORDINAL_TRIPLETS[i]`. Note index 6 is "kentilyon" — missing the ordinal
/// suffix in the Python source. Reproduced verbatim.
fn ordinal_triplet(i: usize) -> Result<&'static str> {
    match i {
        1 => Ok("bininci"),
        2 => Ok("milyonuncu"),
        3 => Ok("milyarıncı"),
        4 => Ok("trilyonuncu"),
        5 => Ok("katrilyonuncu"),
        6 => Ok("kentilyon"),
        _ => Err(N2WError::Key(i.to_string())),
    }
}

/// Python's `int(float(n))` for `n >= 0`.
///
/// `float(int)` rounds to nearest double, ties to even. Every double with
/// magnitude >= 1 is an exact integer, so `int()` of it is lossless and the
/// result is that rounded integer. Done on `BigInt` rather than via `to_f64`
/// so the rounding mode is explicit and not dependent on num-bigint's
/// conversion details.
fn python_float_int(n: &BigInt) -> BigInt {
    if n.is_zero() {
        return BigInt::zero();
    }
    let bits = n.bits();
    if bits <= 53 {
        return n.clone();
    }
    let shift = (bits - 53) as usize;
    let mut q = n >> shift;
    let rem = n - (&q << shift);
    let half = BigInt::one() << (shift - 1);
    // Round half to even.
    if rem > half || (rem == half && q.is_odd()) {
        q += 1;
    }
    q << shift
}

/// The fields `to_splitnum` sets on the Python instance.
///
/// Only `integers_to_read[0]` (the integer digit string) is read by the
/// integer paths, so the fractional element is not modelled.
struct SplitNum {
    /// `integers_to_read[0]`, ASCII decimal digits (byte indexing is safe).
    digits: Vec<u8>,
    total_triplets_to_read: usize,
    total_digits_outside_triplets: usize,
    order_of_last_zero_digit: usize,
}

impl SplitNum {
    /// Python's `to_splitnum`, given the already-computed digit string
    /// (`str(int(val))`).
    fn new(digits: Vec<u8>) -> Self {
        let l = digits.len();
        let total_triplets_to_read = if l % 3 > 0 { l / 3 + 1 } else { l / 3 };
        let total_digits_outside_triplets = l % 3;

        // Count trailing zeros, but the Python loop is `range(len - 1)`, so the
        // count saturates at l - 1: an all-zero string "0" yields 0, not 1.
        let mut order_of_last_zero_digit = 0usize;
        let mut found = false;
        for i in 0..l.saturating_sub(1) {
            if digits[l - 1 - i] == b'0' && !found {
                order_of_last_zero_digit = i + 1;
            } else {
                found = true;
            }
        }

        SplitNum {
            digits,
            total_triplets_to_read,
            total_digits_outside_triplets,
            order_of_last_zero_digit,
        }
    }
}

/// `Num2Word_TR.__init__` *rebinds* `self.CURRENCY_FORMS` to this dict rather
/// than mutating the inherited one, so the `lang_EUR`/`lang_EN` shared-class-dict
/// trap does not apply: TR sees exactly these three codes and nothing else.
/// Verified against the live interpreter, not just the source.
///
/// Every entry is a 2-tuple with both forms identical — Turkish does not
/// pluralize the currency name.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("TRY", CurrencyForms::new(&["lira", "lira"], &["kuruş", "kuruş"]));
    m.insert("EUR", CurrencyForms::new(&["avro", "avro"], &["sent", "sent"]));
    m.insert("USD", CurrencyForms::new(&["dolar", "dolar"], &["sent", "sent"]));
    m
}

pub struct LangTr {
    maxval: BigInt,
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangTr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTr {
    pub fn new() -> Self {
        // Python: MAXVAL = 10 ** ((len(CARDINAL_TRIPLETS) + 1) * 3) - 1 =
        // 10**21 - 1. Lowered to 10**21 - 2**16 (#159): `to_cardinal`'s float
        // cast rounds every value from there up to 10**21, one triplet past
        // the table, so that is the real exclusive ceiling (module docs).
        LangTr {
            maxval: BigInt::from(10u8).pow(21u32) - 65536u32,
            // Built once here, never per call.
            currency_forms: build_currency_forms(),
        }
    }

    /// Python's `verify_cardinal`.
    ///
    /// The Python `float(value)` probe only guards non-numeric input (which
    /// cannot occur for a `BigInt`). It *can* raise `OverflowError` for ints
    /// above ~1.8e308, but any such value also trips the MAXVAL check below
    /// and would raise `OverflowError` regardless, so the outcome is identical.
    fn verify_cardinal(&self, value: &BigInt) -> Result<()> {
        if value.abs() >= self.maxval {
            return Err(N2WError::Overflow(errmsg_toobig(value, &self.maxval)));
        }
        Ok(())
    }

    /// Python's `verify_ordinal`.
    ///
    /// The negative check precedes the MAXVAL check, so a large negative is a
    /// `TypeError`, not an `OverflowError`. And the `TypeError` Python raises
    /// there is caught by its own `except (ValueError, TypeError)` and
    /// re-raised with `errmsg_nonnum` — hence that message here.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(ERRMSG_NONNUM.to_string()));
        }
        if value.abs() >= self.maxval {
            return Err(N2WError::Overflow(errmsg_toobig(value, &self.maxval)));
        }
        Ok(())
    }

    /// Python's `_to_cardinal_impl` for integral input.
    fn to_cardinal_impl(&self, value: &BigInt) -> Result<String> {
        self.verify_cardinal(value)?;

        // Python: `if str(value).startswith("-")` → negword, then re-parses the
        // digits as a float. Both branches go through `float(...)`, so the
        // digit string is the float-rounded one either way.
        let (pre_word, magnitude) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        let digits = python_float_int(&magnitude).to_string().into_bytes();
        let sn = SplitNum::new(digits);
        self.read_cardinal(&sn, pre_word)
    }

    fn read_cardinal(&self, sn: &SplitNum, pre_word: &str) -> Result<String> {
        let d = &sn.digits;
        let l = d.len();
        let t = sn.total_triplets_to_read;
        let o = sn.total_digits_outside_triplets;
        let z = sn.order_of_last_zero_digit;
        let mut wrd = String::new();

        // Dead in practice: z saturates at l - 1. Kept for structural fidelity.
        if z >= l {
            return Ok(format!("{}{}", pre_word, wrd));
        }

        if t == 1 {
            if o == 2 {
                if z == 1 {
                    wrd += cardinal_tens(d[0]);
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == 0 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_ones(d[1]);
                }
                return Ok(format!("{}{}", pre_word, wrd));
            }

            if o == 1 && z == 0 {
                wrd += cardinal_ones(d[0]);
                // Note: zero returns bare ZERO, dropping pre_word.
                if d[0] == b'0' {
                    return Ok(ZERO.to_string());
                }
                return Ok(format!("{}{}", pre_word, wrd));
            }

            if o == 0 {
                if z == 2 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == 1 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == 0 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += cardinal_ones(d[2]);
                    return Ok(format!("{}{}", pre_word, wrd));
                }
            }
        }

        if t >= 2 {
            // Leading (partial) triplet. Branches that do not return fall
            // through into the triplet loop below.
            if o == 2 {
                if z == l - 1 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == l - 2 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_ones(d[1]);
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z < l - 2 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_ones(d[1]);
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            if o == 1 {
                if z == l - 1 {
                    // "bin", not "birbin", for exactly 1_000..1_999-shaped leads.
                    if !(t == 2 && d[0] == b'1') {
                        wrd += cardinal_ones(d[0]);
                    }
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z < l - 1 {
                    if !(t == 2 && d[0] == b'1') {
                        wrd += cardinal_ones(d[0]);
                    }
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            if o == 0 {
                if z == l - 1 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == l - 2 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z == l - 3 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += cardinal_ones(d[2]);
                    wrd += cardinal_triplet(t - 1)?;
                    return Ok(format!("{}{}", pre_word, wrd));
                }
                if z < l - 3 {
                    // Issue #64: unlike to_ordinal, this keeps "bir"
                    // (401607 → "dörtyüzbirbinaltıyüzyedi").
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += cardinal_ones(d[2]);
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            for i in (1..=t - 1).rev() {
                let reading_triplet_order = t - i;
                let lrdo = if o == 0 {
                    reading_triplet_order * 3
                } else {
                    (reading_triplet_order - 1) * 3 + o
                };
                // lrdo + 2 <= l - 1 for every reachable i; see the module tests.
                if &d[lrdo..lrdo + 3] == b"000" {
                    continue;
                }

                if d[lrdo] != b'0' {
                    wrd += hundreds(d[lrdo]);
                    if z == l - lrdo - 1 {
                        if i == 1 {
                            wrd += CARDINAL_HUNDRED;
                            return Ok(format!("{}{}", pre_word, wrd));
                        } else {
                            wrd += CARDINAL_HUNDRED;
                            wrd += cardinal_triplet(i - 1)?;
                            return Ok(format!("{}{}", pre_word, wrd));
                        }
                    } else {
                        wrd += CARDINAL_HUNDRED;
                    }
                }

                if d[lrdo + 1] != b'0' {
                    if z == l - lrdo - 2 {
                        if i == 1 {
                            wrd += cardinal_tens(d[lrdo + 1]);
                            return Ok(format!("{}{}", pre_word, wrd));
                        } else {
                            wrd += cardinal_tens(d[lrdo + 1]);
                            wrd += cardinal_triplet(i - 1)?;
                            return Ok(format!("{}{}", pre_word, wrd));
                        }
                    } else {
                        wrd += cardinal_tens(d[lrdo + 1]);
                    }
                }

                if d[lrdo + 2] != b'0' {
                    if z == l - lrdo - 3 {
                        if i == 1 {
                            wrd += cardinal_ones(d[lrdo + 2]);
                            return Ok(format!("{}{}", pre_word, wrd));
                        }
                        if i == 2 {
                            // "x001yyy" drops the "bir" before "bin".
                            if &d[lrdo..lrdo + 2] != b"00" || d[lrdo + 2] != b'1' {
                                wrd += cardinal_ones(d[lrdo + 2]);
                            }
                            wrd += cardinal_triplet(i - 1)?;
                            return Ok(format!("{}{}", pre_word, wrd));
                        }
                        // i > 2
                        wrd += cardinal_ones(d[lrdo + 2]);
                        wrd += cardinal_triplet(i - 1)?;
                        return Ok(format!("{}{}", pre_word, wrd));
                    } else if &d[lrdo..lrdo + 2] != b"00" {
                        wrd += cardinal_ones(d[lrdo + 2]);
                    } else if i == 2 && d[lrdo + 2] != b'1' {
                        // Python's inner `if not d[lrdo:lrdo+2] == "00"` here is
                        // dead (it is "00" by the branch we are in), so only the
                        // elif can fire. For i != 2 the ones digit of a "00x"
                        // triplet is dropped entirely — reproduced.
                        wrd += cardinal_ones(d[lrdo + 2]);
                    }
                }

                wrd += cardinal_triplet(i - 1)?;
            }
        }

        Ok(format!("{}{}", pre_word, wrd))
    }

    /// Python's `to_ordinal`. Note it feeds `to_splitnum` the *exact* integer,
    /// with no float round-trip — unlike `to_cardinal`.
    fn to_ordinal_impl(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        let sn = SplitNum::new(value.to_string().into_bytes());
        let d = &sn.digits;
        let l = d.len();
        let t = sn.total_triplets_to_read;
        let o = sn.total_digits_outside_triplets;
        let z = sn.order_of_last_zero_digit;
        let mut wrd = String::new();

        if z >= l {
            return Ok(wrd);
        }

        if t == 1 {
            if o == 2 {
                if z == 1 {
                    wrd += ordinal_tens(d[0]);
                    return Ok(wrd);
                }
                if z == 0 {
                    wrd += cardinal_tens(d[0]);
                    wrd += ordinal_ones(d[1]);
                    return Ok(wrd);
                }
            }

            if o == 1 && z == 0 {
                wrd += ordinal_ones(d[0]);
                if d[0] == b'0' {
                    return Ok(ZEROTH.to_string());
                }
                return Ok(wrd);
            }

            if o == 0 {
                if z == 2 {
                    wrd += hundreds(d[0]);
                    wrd += ORDINAL_HUNDRED;
                    return Ok(wrd);
                }
                if z == 1 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += ordinal_tens(d[1]);
                    return Ok(wrd);
                }
                if z == 0 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    if d[2] != b'0' {
                        wrd += ordinal_ones(d[2]);
                    }
                    return Ok(wrd);
                }
            }
        }

        if t >= 2 {
            if o == 2 {
                if z == l - 1 {
                    wrd += cardinal_tens(d[0]);
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z == l - 2 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_ones(d[1]);
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z < l - 2 {
                    wrd += cardinal_tens(d[0]);
                    wrd += cardinal_ones(d[1]);
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            if o == 1 {
                if z == l - 1 {
                    if !(t == 2 && d[0] == b'1') {
                        wrd += cardinal_ones(d[0]);
                    }
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z < l - 1 {
                    if !(t == 2 && d[0] == b'1') {
                        wrd += cardinal_ones(d[0]);
                    }
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            if o == 0 {
                if z == l - 1 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z == l - 2 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z == l - 3 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    wrd += cardinal_ones(d[2]);
                    wrd += ordinal_triplet(t - 1)?;
                    return Ok(wrd);
                }
                if z < l - 3 {
                    wrd += hundreds(d[0]);
                    wrd += CARDINAL_HUNDRED;
                    wrd += cardinal_tens(d[1]);
                    // The issue-#64 fix was NOT applied here: to_ordinal still
                    // drops "bir" where to_cardinal keeps it.
                    if !(t == 2 && d[2] == b'1') {
                        wrd += cardinal_ones(d[2]);
                    }
                    wrd += cardinal_triplet(t - 1)?;
                }
            }

            for i in (1..=t - 1).rev() {
                let reading_triplet_order = t - i;
                let lrdo = if o == 0 {
                    reading_triplet_order * 3
                } else {
                    (reading_triplet_order - 1) * 3 + o
                };
                if &d[lrdo..lrdo + 3] == b"000" {
                    continue;
                }

                if d[lrdo] != b'0' {
                    // Equivalent to to_cardinal's `hundreds(d[lrdo])`: HUNDREDS
                    // simply lacks a "1" key. Spelled out as Python does here.
                    if d[lrdo] != b'1' {
                        wrd += cardinal_ones(d[lrdo]);
                    }
                    if z == l - lrdo - 1 {
                        if i == 1 {
                            wrd += ORDINAL_HUNDRED;
                            return Ok(wrd);
                        } else {
                            wrd += CARDINAL_HUNDRED;
                            wrd += ordinal_triplet(i - 1)?;
                            return Ok(wrd);
                        }
                    } else {
                        wrd += CARDINAL_HUNDRED;
                    }
                }

                if d[lrdo + 1] != b'0' {
                    if z == l - lrdo - 2 {
                        if i == 1 {
                            wrd += ordinal_tens(d[lrdo + 1]);
                            return Ok(wrd);
                        } else {
                            wrd += cardinal_tens(d[lrdo + 1]);
                            wrd += ordinal_triplet(i - 1)?;
                            return Ok(wrd);
                        }
                    } else {
                        wrd += cardinal_tens(d[lrdo + 1]);
                    }
                }

                if d[lrdo + 2] != b'0' {
                    if z == l - lrdo - 3 {
                        if i == 1 {
                            wrd += ordinal_ones(d[lrdo + 2]);
                            return Ok(wrd);
                        }
                        if i == 2 {
                            if &d[lrdo..lrdo + 2] != b"00" || d[lrdo + 2] != b'1' {
                                wrd += cardinal_ones(d[lrdo + 2]);
                            }
                            wrd += ordinal_triplet(i - 1)?;
                            return Ok(wrd);
                        }
                        wrd += cardinal_ones(d[lrdo + 2]);
                        wrd += ordinal_triplet(i - 1)?;
                        return Ok(wrd);
                    } else if &d[lrdo..lrdo + 2] != b"00" {
                        wrd += cardinal_ones(d[lrdo + 2]);
                    } else if d[lrdo + 2] != b'1' {
                        // NOTE: unlike to_cardinal's matching branch, Python has
                        // no `if i == 2` guard here, so the "00x" ones digit is
                        // emitted for every triplet (only a literal "001" is
                        // dropped). to_cardinal instead drops it whenever i != 2.
                        // The asymmetry is Python's; do not "harmonise" it.
                        wrd += cardinal_ones(d[lrdo + 2]);
                    }
                }

                wrd += cardinal_triplet(i - 1)?;
            }
        }

        Ok(wrd)
    }

    /// Python's `verify_ordinal` on a float/Decimal (see module header).
    ///
    /// * `int(value) == value` failing does NOT raise — it only clears
    ///   `isordinal`, and the caller then returns the pristine `""`.
    /// * `not abs(value) == value` is a *numeric* comparison: -0.0 passes
    ///   (0.0 == -0.0), any true negative raises TypeError — which Python's
    ///   own `except (ValueError, TypeError)` re-raises with `errmsg_nonnum`.
    ///   `FloatValue::is_negative()` is sign-bit aware and would wrongly
    ///   flag -0.0, hence the explicit `< 0` checks.
    /// * The MAXVAL check runs for non-integral values too, so a huge
    ///   non-whole Decimal is OverflowError, not `""`.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<()> {
        let numerically_negative = match value {
            FloatValue::Float { value: f, .. } => *f < 0.0,
            // Decimal("-0.0") never reaches this arm: the binding re-routes
            // signed decimal zero through the Float arm.
            FloatValue::Decimal { value: d, .. } => d.is_negative(),
        };
        if numerically_negative {
            return Err(N2WError::Type(ERRMSG_NONNUM.to_string()));
        }
        if self.float_exceeds_maxval(value) {
            return Err(N2WError::Overflow(errmsg_toobig_repr(
                &float_value_repr(value),
                &self.maxval,
            )));
        }
        Ok(())
    }

    /// `abs(value) >= self.MAXVAL` on the original float/Decimal, exactly.
    fn float_exceeds_maxval(&self, value: &FloatValue) -> bool {
        if let Some(i) = value.as_whole_int() {
            return i.abs() >= self.maxval;
        }
        match value {
            // A non-whole f64 is < 2^53 ~ 9e15, five orders below MAXVAL.
            FloatValue::Float { .. } => false,
            FloatValue::Decimal { value: d, .. } => {
                d.abs() >= BigDecimal::from(self.maxval.clone())
            }
        }
    }

    /// Port of `Num2Word_TR.to_cardinal_float` (issue #64 part 3 / upstream
    /// #487 + #534), parameterized on the pointword so `decimal_word=` can
    /// swap it per call. TR does *not* use `Num2Word_Base.to_cardinal_float`
    /// (`float2tuple`); it formats the value to a fixed number of decimals
    /// with Python's `"{:.{p}f}".format` and reads the integer/fraction
    /// parts.
    ///
    /// The float-vs-Decimal split is load-bearing: `"{:.2f}"` of the *float*
    /// `12.345` is "12.35" (the double is 12.34500000000000064), while of the
    /// *Decimal* `12.345` it is "12.34" (exact half-to-even at the 2-decimal
    /// scale). `FloatValue` keeps them apart, so each arm reproduces its own
    /// rounding: `f64` string formatting for `Float`, `with_scale_round(0,
    /// HalfEven)` on `abs*10^p` for `Decimal` (both round half-to-even).
    ///
    /// `precision_override` is the `precision=` kwarg, which Python assigns
    /// to `self.precision`; the default is 2. `natural` (the repr-derived
    /// fractional-digit count, i.e. `FloatValue::precision`) only shrinks the
    /// effective precision when the caller left it at the default 2 and the
    /// value has a single fractional digit (so 0.1 -> "bir", not "on").
    fn cardinal_float_impl(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        pointword: &str,
    ) -> Result<String> {
        // self.precision (default 2) possibly overridden by the kwarg.
        let self_precision: u32 = precision_override.unwrap_or(2);
        // natural = abs(Decimal(str(abs_value)).as_tuple().exponent).
        let natural: u32 = value.precision();
        let prec: usize = if self_precision == 2 && natural != 0 && natural < 2 {
            natural as usize
        } else {
            self_precision.max(1) as usize
        };

        let is_negative = value.is_negative();

        // whole_str = "{:.{prec}f}".format(abs_value) -> (int_part, frac_part),
        // frac_part having exactly `prec` digits. Reproduce Python's rounding
        // per arm; never collapse Float into Decimal or the digits can differ.
        let (int_part, frac_part) = match value {
            FloatValue::Float { value: f, .. } => {
                let s = format!("{:.*}", prec, f.abs());
                match s.split_once('.') {
                    Some((i, fr)) => (i.to_string(), fr.to_string()),
                    None => (s, "0".repeat(prec)),
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                let ten_pow = BigInt::from(10).pow(prec as u32);
                // round(abs*10^prec) half-to-even == "{:.prec f}" for Decimal.
                let scaled = (d.abs() * BigDecimal::from(ten_pow.clone()))
                    .with_scale_round(0, RoundingMode::HalfEven)
                    .as_bigint_and_exponent()
                    .0;
                let int_val = &scaled / &ten_pow;
                let frac_val = &scaled % &ten_pow;
                let frac_str = frac_val.to_string();
                let frac_part = format!(
                    "{}{}",
                    "0".repeat(prec.saturating_sub(frac_str.len())),
                    frac_str
                );
                (int_val.to_string(), frac_part)
            }
        };

        let mut wrd = String::from(pointword);
        if prec == 2 {
            // Historical 2-digit reading: preserve tens-and-ones, with the
            // "0x" leading-zero case spelled "sıfır <one>".
            let fb = frac_part.as_bytes();
            if fb[0] == b'0' && fb[1] != b'0' {
                wrd.push_str(ZERO);
                wrd.push_str(cardinal_ones(fb[1]));
            } else {
                wrd.push_str(cardinal_tens(fb[0]));
                wrd.push_str(cardinal_ones(fb[1]));
            }
        } else {
            // Read the fractional part as one integer, spelling leading zeros
            // out separately (as 0.03 reads at precision 2).
            let stripped = frac_part.trim_start_matches('0');
            let leading_zeros = frac_part.len() - stripped.len();
            for _ in 0..leading_zeros {
                wrd.push_str(ZERO);
            }
            if !stripped.is_empty() {
                let n: BigInt = stripped.parse().unwrap();
                wrd.push_str(&self.to_cardinal_impl(&n)?);
            }
        }

        if int_part == "0" {
            wrd = format!("{}{}", ZERO, wrd);
        } else {
            let n: BigInt = int_part.parse().unwrap();
            wrd = format!("{}{}", self.to_cardinal_impl(&n)?, wrd);
        }

        if is_negative {
            // Python: `self.negword + " " + wrd` — the *spaced* form, unlike
            // the currency int fast-path which uses the bare negword.
            wrd = format!("{} {}", NEGWORD, wrd);
        }
        Ok(wrd)
    }
}

/// `errmsg_toobig` with the value already rendered. Only the exception
/// *type* is corpus-observable for these rows.
fn errmsg_toobig_repr(value_repr: &str, maxval: &BigInt) -> String {
    format!(
        "abs({}) sayı yazıya çevirmek için çok büyük. \
         Yazıya çevrilebilecek en büyük rakam {}.",
        value_repr,
        maxval - 1u32
    )
}

/// Best-effort Python `str(value)` for the OverflowError message.
///
/// Reachable only for |value| >= MAXVAL ~ 1e21. `str(Decimal)` is exact via
/// `python_decimal_str`; a float that big always reprs in exponent form, so
/// Rust's `{:e}` ("1e21") is massaged into Python's shape ("1e+21").
fn float_value_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value: f, .. } => {
            let s = format!("{:e}", f);
            match s.split_once('e') {
                Some((m, e)) if !e.starts_with('-') => format!("{}e+{}", m, e),
                _ => s,
            }
        }
        FloatValue::Decimal { value: d, .. } => python_decimal_str(d),
    }
}

/// Python truthiness of a kwargs value (`if spaced:` accepts any object).
fn kw_truthy(v: &KwVal) -> bool {
    match v {
        KwVal::Bool(b) => *b,
        KwVal::Int(i) => *i != 0,
        KwVal::Str(s) => !s.is_empty(),
        KwVal::List(l) => !l.is_empty(),
        KwVal::None => false,
    }
}

/// `decimal_word=` extraction. Python's `if decimal_word is not None:
/// self.pointword = decimal_word` accepts anything, but a non-str only
/// crashes where the pointword is actually consumed (the float grammar's
/// `str + int` TypeError, `_insert_spaces`'s `len(int)` TypeError) — return
/// NotImplemented for those so the Python original owns its exact crash.
fn tr_decimal_word(kw: &Kwargs) -> Result<Option<&str>> {
    match kw.get("decimal_word") {
        None | Some(KwVal::None) => Ok(None),
        Some(KwVal::Str(s)) => Ok(Some(s.as_str())),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

/// Port of `Num2Word_TR._insert_spaces` (issue #64 part 2, upstream #486).
///
/// Greedy longest-first tokenizer over the solid output. The token set is
/// the *cardinal* tables + ZERO + the current pointword (i.e. the caller's
/// decimal_word) — NOT the negword: "eksi" survives only because its letters
/// fall through the unknown-char branch verbatim, and the next matched token
/// then gets a space in front ("eksibir" -> "eksi bir"). Python sorts with
/// `key=len` (code points, not bytes — hence `chars().count()`) over a set;
/// equal-length ties cannot collide (two distinct equal-length tokens cannot
/// both prefix the same suffix), so the set's arbitrary tie order is
/// irrelevant and duplicates need no dedup.
fn insert_spaces(text: &str, pointword: &str) -> String {
    let mut tokens: Vec<&str> = vec![
        // CARDINAL_TRIPLETS.values()
        "bin", "milyon", "milyar", "trilyon", "katrilyon", "kentilyon",
        // CARDINAL_HUNDRED
        "yüz",
        // CARDINAL_TENS.values()
        "on", "yirmi", "otuz", "kırk", "elli", "altmış", "yetmiş", "seksen",
        "doksan",
        // CARDINAL_ONES.values()
        "bir", "iki", "üç", "dört", "beş", "altı", "yedi", "sekiz", "dokuz",
        // ZERO
        "sıfır",
    ];
    tokens.push(pointword);
    // `{t for t in tokens if t}` — drop empties (pointword may be "").
    tokens.retain(|t| !t.is_empty());
    tokens.sort_by_key(|t| std::cmp::Reverse(t.chars().count()));

    let mut out: Vec<&str> = Vec::new();
    let mut i = 0usize;
    while i < text.len() {
        let ch = text[i..].chars().next().unwrap();
        if ch.is_whitespace() {
            // Python folds runs of whitespace into a single " " and never
            // emits one at the very start (`if out and out[-1] != " "`).
            if matches!(out.last(), Some(last) if *last != " ") {
                out.push(" ");
            }
            i += ch.len_utf8();
            continue;
        }
        if let Some(tok) = tokens.iter().copied().find(|t| text[i..].starts_with(*t)) {
            // A space goes before every matched token — including right
            // after an unknown char, which is how "eksi" gets its space.
            if matches!(out.last(), Some(last) if *last != " ") {
                out.push(" ");
            }
            out.push(tok);
            i += tok.len();
        } else {
            // Unknown char: pass through verbatim, no space of its own.
            out.push(&text[i..i + ch.len_utf8()]);
            i += ch.len_utf8();
        }
    }
    out.concat().trim().to_string()
}

impl Lang for LangTr {
    /// Python's signature says `currency="EUR"`, but its int path always
    /// said lira. TRY for both paths since #187.
    fn default_currency(&self) -> &str {
        "TRY"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    fn negword(&self) -> &str {
        // Python's negword carries no trailing space; it is concatenated
        // directly ("eksi" + "bir" = "eksibir").
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "virgül"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal_impl(value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.to_ordinal_impl(value)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal_impl(value)?;
        let ordinal = self.to_ordinal_impl(value)?;
        // Recover the vowel-harmonised suffix by stripping the cardinal off the
        // ordinal. `starts_with` guarantees a char boundary, so the byte slice
        // is safe despite the non-ASCII words.
        let suffix: String = if ordinal.starts_with(&cardinal) {
            ordinal[cardinal.len()..].to_string()
        } else {
            // Python's defensive fallback: `ordinal[-4:]` — last four *chars*.
            let chars: Vec<char> = ordinal.chars().collect();
            chars[chars.len().saturating_sub(4)..].iter().collect()
        };
        Ok(format!("{}'{}", value, suffix))
    }

    /// `lang_TR.py` does not override `to_year`; `Num2Word_Base.to_year`
    /// delegates straight to `to_cardinal`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `Num2Word_TR.to_cardinal_float` — see [`LangTr::cardinal_float_impl`]
    /// for the full port; this entry uses the default pointword.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        self.cardinal_float_impl(value, precision_override, self.pointword())
    }

    // ---- float/Decimal entry routing -------------------------------------

    /// `to_cardinal(float/Decimal)` full routing (`_to_cardinal_impl`):
    /// `verify_cardinal` first (its MAXVAL check is reachable by huge
    /// non-whole Decimals, which must raise OverflowError rather than read
    /// through the float grammar), then `int(value) == value` decides between
    /// the integer digit reader and the float grammar.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        if self.float_exceeds_maxval(value) {
            return Err(N2WError::Overflow(errmsg_toobig_repr(
                &float_value_repr(value),
                &self.maxval,
            )));
        }
        match value.as_whole_int() {
            Some(i) => self.to_cardinal_impl(&i),
            None => self.to_cardinal_float(value, precision_override),
        }
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for
    /// numeric negatives (-0.0 passes and reads "sıfırıncı") and
    /// OverflowError above MAXVAL; a non-integral value merely clears
    /// `isordinal`, so Python's `to_ordinal(0.5)` returns the pristine `""`.
    /// The port raises TypeError instead (#160). Whole
    /// values read the *exact* integer digits — no float round-trip, unlike
    /// to_cardinal.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.verify_ordinal_float(value)?;
        match value.as_whole_int() {
            Some(i) => self.to_ordinal_impl(&i),
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                float_value_repr(value)
            ))),
        }
    }

    /// `to_ordinal_num(float/Decimal)`: after `verify_ordinal`, the suffix
    /// is recovered by stripping `to_cardinal(value)` off
    /// `to_ordinal(value)`. For a non-integral value the ordinal is `""`
    /// (above), the strip fails, and the fallback `""[-4:]` is `""` — hence
    /// `to_ordinal_num(0.5)` == `"0.5'"`. And since `ORDINAL_TRIPLETS[6]`
    /// lacks its suffix, `to_ordinal_num(1e20)` is `"1e+20'"` (cardinal ==
    /// ordinal == "yüzkentilyon"). The cardinal is computed first, exactly
    /// as Python orders it — its float-cast KeyError band fires before the
    /// (immune) ordinal runs.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        self.verify_ordinal_float(value)?;
        let (cardinal, ordinal) = match value.as_whole_int() {
            Some(i) => {
                let c = self.to_cardinal_impl(&i)?;
                let o = self.to_ordinal_impl(&i)?;
                (c, o)
            }
            None => (self.to_cardinal_float(value, None)?, String::new()),
        };
        let suffix: String = if ordinal.starts_with(&cardinal) {
            ordinal[cardinal.len()..].to_string()
        } else {
            // Python's defensive fallback: `ordinal[-4:]` — last four chars.
            let chars: Vec<char> = ordinal.chars().collect();
            chars[chars.len().saturating_sub(4)..].iter().collect()
        };
        Ok(format!("{}'{}", repr_str, suffix))
    }

    // `to_year(float/Decimal)` needs no override: the trait default routes
    // through `cardinal_float_entry`, exactly like `Num2Word_Base.to_year`
    // -> `to_cardinal`.

    // ---- string inputs ----------------------------------------------------

    /// Base's `str_to_number` is plain `Decimal(value)`; TR then treats a
    /// NaN mode-dependently (decimal.InvalidOperation from the MAXVAL
    /// compare in `verify_cardinal`, TypeError(errmsg_nonnum) from
    /// `verify_ordinal`'s int() catch). The binding's generic
    /// `ParsedNumber::NaN` mapping (int()-ValueError) matches neither, so
    /// return NotImplemented: the shim reruns the original Python string
    /// pipeline, which owns every mode's exact outcome. Infinity stays on
    /// the generic mapping: OverflowError, the same type TR raises via the
    /// MAXVAL compare.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        // NaN is carried through natively now and split per mode in
        // `nan_result`; Infinity keeps riding the generic mapping
        // (`inf_result` default → OverflowError, the type TR raises via the
        // MAXVAL compare).
        python_decimal_parse(s)
    }

    /// `Decimal('NaN')` per mode, reproducing TR's own crash sites:
    ///
    /// * `to_cardinal` / `to_year` (== cardinal) run `verify_cardinal`, whose
    ///   `abs(value) >= self.MAXVAL` compares `Decimal('NaN')` against an int
    ///   and raises `decimal.InvalidOperation` — not the int()-ValueError the
    ///   binding's generic NaN mapping assumes.
    /// * `to_ordinal` / `to_ordinal_num` run `verify_ordinal`, whose
    ///   `int(Decimal('NaN'))` raises `ValueError`, caught by its own
    ///   `except (ValueError, TypeError)` and re-raised as
    ///   `TypeError(errmsg_nonnum)`.
    fn nan_result(&self, to: &str) -> Result<String> {
        match to {
            "cardinal" | "year" => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            _ => Err(N2WError::Type(ERRMSG_NONNUM.to_string())),
        }
    }

    // ---- grammatical kwargs ------------------------------------------------
    //
    // Python signature: `to_cardinal(value, spaced=False, precision=None,
    // decimal_word=None)`. `precision=` never reaches these hooks — the
    // dispatcher keeps it on the Python side for the int and string entries
    // (`"precision" not in kwargs` guards both) and strips it into
    // `precision_override` for the float entry — so it is deliberately
    // absent from the `only()` lists: were it ever to arrive, the
    // NotImplemented fallback to Python is exact by construction.
    // `to_ordinal`/`to_ordinal_num`/`to_currency` accept no extra kwargs;
    // their trait defaults fall back to Python's own TypeError.

    /// `to_cardinal(int, spaced=, decimal_word=)`. The integer reader never
    /// consults the pointword, so `decimal_word` only matters through
    /// `_insert_spaces`'s token set when `spaced` is truthy.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["spaced", "decimal_word"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let decimal_word = tr_decimal_word(kw)?;
        let wrd = self.to_cardinal_impl(value)?;
        if kw.get("spaced").map(kw_truthy).unwrap_or(false) {
            Ok(insert_spaces(&wrd, decimal_word.unwrap_or(self.pointword())))
        } else {
            Ok(wrd)
        }
    }

    /// `to_cardinal(float/Decimal, spaced=, decimal_word=)` with
    /// `precision=` already extracted into `precision_override` by the
    /// dispatcher. Full routing like [`Lang::cardinal_float_entry`], then
    /// `_insert_spaces` while the pointword is still the caller's
    /// decimal_word, so the custom word is recognized as its own token.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["spaced", "decimal_word", "precision"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        // `precision=` normally arrives already extracted into
        // `precision_override` (the real dispatcher strips it before the Rust
        // call), but it can also ride in the kwargs bag; honour either,
        // preferring the bag. Python does `converter.precision =
        // int(precision_override)`, so a non-int is deferred to Python.
        let precision_override = match kw.get("precision") {
            None | Some(KwVal::None) => precision_override,
            Some(KwVal::Int(p)) if *p >= 0 => Some(*p as u32),
            Some(KwVal::Bool(b)) => Some(u32::from(*b)),
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        let pointword = tr_decimal_word(kw)?.unwrap_or(self.pointword());
        if self.float_exceeds_maxval(value) {
            return Err(N2WError::Overflow(errmsg_toobig_repr(
                &float_value_repr(value),
                &self.maxval,
            )));
        }
        let wrd = match value.as_whole_int() {
            Some(i) => self.to_cardinal_impl(&i)?,
            None => self.cardinal_float_impl(value, precision_override, pointword)?,
        };
        if kw.get("spaced").map(kw_truthy).unwrap_or(false) {
            Ok(insert_spaces(&wrd, pointword))
        } else {
            Ok(wrd)
        }
    }

    /// `Num2Word_Base.to_year(value, **kwargs)`: every kwarg is swallowed
    /// unread and the result is plain `to_cardinal(value)` — `spaced=True`
    /// on to_year does NOT space the output.
    fn to_year_kw(&self, value: &BigInt, _kw: &Kwargs) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_TR` overrides only `pluralize` and the int fast-path of
    // `to_currency`. `to_cheque`, `_money_verbose`, `_cents_verbose` and
    // `_cents_terse` are inherited from `Num2Word_Base` unchanged, and
    // `CURRENCY_ADJECTIVES` / `CURRENCY_PRECISION` are the (empty) base class
    // dicts — confirmed empty at runtime, so `currency_adjective` (None) and
    // `currency_precision` (100 for every code, including KWD/BHD/JPY) stay at
    // their trait defaults. The default `money_verbose`/`cents_verbose` call
    // back through `to_cardinal`, which is TR's own digit-string reader, so
    // they need no override either.

    fn lang_name(&self) -> &str {
        "Num2Word_TR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_TR.pluralize`: `forms[0] if number == 1 else forms[0]`.
    ///
    /// Both arms of Python's ternary are `forms[0]`, so `number` is unused —
    /// Turkish keeps one form for every count ("bir lira", "yüz lira").
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        if forms.len() >= 2 {
            return Ok(forms[0].clone());
        }
        // Python guards with `isinstance(forms, tuple) and len(forms) >= 2`
        // and otherwise `return forms` — handing the *tuple itself* back to a
        // caller that renders it with %s, i.e. its repr. Unreachable for TR:
        // all three CURRENCY_FORMS entries are 2-tuples, and CURRENCY_ADJECTIVES
        // is empty so prefix_currency (which would swap in a list) never runs.
        // Reproduced rather than turned into an exception Python never raises.
        Ok(format!(
            "({})",
            forms.iter().map(|f| format!("'{}',", f)).collect::<String>()
        ))
    }

    /// `Num2Word_TR.to_currency`.
    ///
    /// Python's int arm dropped `currency`, `cents`, `separator` and
    /// `adjective`, never looked at `CURRENCY_FORMS`, and glued the parts
    /// together with no spaces ("kırkikilira"). Since #187 every input
    /// defers to `Num2Word_Base`, as floats always did.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    fn frac_prec(f: f64) -> u32 {
        let s = format!("{}", f);
        match s.split_once('.') {
            Some((_, fr)) if !fr.contains('e') => fr.len() as u32,
            _ => 0,
        }
    }
    fn cf(f: f64) -> String {
        let v = FloatValue::Float { value: f, precision: frac_prec(f) };
        LangTr::new().to_cardinal_float(&v, None).unwrap()
    }
    fn cd(s: &str) -> String {
        let bd = BigDecimal::from_str(s).unwrap();
        let prec = s.split_once('.').map(|(_, fr)| fr.len() as u32).unwrap_or(0);
        let v = FloatValue::Decimal { value: bd, precision: prec };
        LangTr::new().to_cardinal_float(&v, None).unwrap()
    }

    #[test]
    fn corpus_float() {
        assert_eq!(cf(0.5), "sıfırvirgülbeş");
        assert_eq!(cf(1.5), "birvirgülbeş");
        assert_eq!(cf(2.25), "ikivirgülyirmibeş");
        assert_eq!(cf(3.14), "üçvirgülondört");
        assert_eq!(cf(0.01), "sıfırvirgülsıfırbir");
        assert_eq!(cf(0.1), "sıfırvirgülbir");
        assert_eq!(cf(0.99), "sıfırvirgüldoksandokuz");
        assert_eq!(cf(1.01), "birvirgülsıfırbir");
        assert_eq!(cf(12.34), "onikivirgülotuzdört");
        assert_eq!(cf(99.99), "doksandokuzvirgüldoksandokuz");
        assert_eq!(cf(100.5), "yüzvirgülbeş");
        assert_eq!(cf(1234.56), "binikiyüzotuzdörtvirgülellialtı");
        assert_eq!(cf(-0.5), "eksi sıfırvirgülbeş");
        assert_eq!(cf(-1.5), "eksi birvirgülbeş");
        assert_eq!(cf(-12.34), "eksi onikivirgülotuzdört");
        assert_eq!(cf(1.005), "birvirgül");
        assert_eq!(cf(2.675), "ikivirgülaltmışyedi");
    }

    #[test]
    fn corpus_decimal() {
        assert_eq!(cd("0.01"), "sıfırvirgülsıfırbir");
        assert_eq!(cd("1.10"), "birvirgülon");
        assert_eq!(cd("12.345"), "onikivirgülotuzdört");
        assert_eq!(cd("98746251323029.99"),
            "doksansekiztrilyonyediyüzkırkaltımilyarikiyüzellibirmilyonüçyüzyirmiüçbinyirmidokuzvirgüldoksandokuz");
        assert_eq!(cd("0.001"), "sıfırvirgül");
        // Half-to-even ties on the *exact* decimal (unlike the f64 arm):
        // 2.675 -> 268 (even), not 267 like the float; 2.5 -> prec 1.
        assert_eq!(cd("2.675"), "ikivirgülaltmışsekiz");
        assert_eq!(cd("2.5"), "ikivirgülbeş");
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod entry_and_kwargs_tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    fn fl(f: f64) -> FloatValue {
        let s = format!("{}", f);
        let prec = match s.split_once('.') {
            Some((_, fr)) if !fr.contains('e') => fr.len() as u32,
            _ => 0,
        };
        FloatValue::Float { value: f, precision: prec }
    }
    fn dc(s: &str) -> FloatValue {
        let bd = BigDecimal::from_str(s).unwrap();
        let prec = s.split_once('.').map(|(_, fr)| fr.len() as u32).unwrap_or(0);
        FloatValue::Decimal { value: bd, precision: prec }
    }
    fn kw(items: &[(&str, KwVal)]) -> Kwargs {
        Kwargs(items.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
    }
    fn l() -> LangTr {
        LangTr::new()
    }

    #[test]
    fn ordinal_float_corpus() {
        assert_eq!(l().ordinal_float_entry(&fl(0.0)).unwrap(), "sıfırıncı");
        assert_eq!(l().ordinal_float_entry(&fl(-0.0)).unwrap(), "sıfırıncı");
        assert_eq!(l().ordinal_float_entry(&fl(1.0)).unwrap(), "birinci");
        assert_eq!(l().ordinal_float_entry(&fl(21.0)).unwrap(), "yirmibirinci");
        assert_eq!(
            l().ordinal_float_entry(&fl(1234.0)).unwrap(),
            "binikiyüzotuzdördüncü"
        );
        assert_eq!(l().ordinal_float_entry(&fl(1e16)).unwrap(), "onkatrilyonuncu");
        // ORDINAL_TRIPLETS[6] lacks its suffix: cardinal == ordinal.
        assert_eq!(l().ordinal_float_entry(&fl(1e20)).unwrap(), "yüzkentilyon");
        // Non-integral -> TypeError, not Python's pristine "" (#160).
        assert!(matches!(l().ordinal_float_entry(&fl(0.5)), Err(N2WError::Type(_))));
        assert!(matches!(l().ordinal_float_entry(&fl(3.25)), Err(N2WError::Type(_))));
        // Numeric negatives -> TypeError(errmsg_nonnum).
        assert!(matches!(
            l().ordinal_float_entry(&fl(-2.0)),
            Err(N2WError::Type(m)) if m == ERRMSG_NONNUM
        ));
        assert!(matches!(
            l().ordinal_float_entry(&fl(-1.5)),
            Err(N2WError::Type(_))
        ));
        assert!(matches!(
            l().ordinal_float_entry(&dc("-3.0")),
            Err(N2WError::Type(_))
        ));
        // Decimal arm.
        assert_eq!(l().ordinal_float_entry(&dc("5.00")).unwrap(), "beşinci");
        assert_eq!(l().ordinal_float_entry(&dc("1E+2")).unwrap(), "yüzüncü");
        assert_eq!(
            l().ordinal_float_entry(&dc("12345.000")).unwrap(),
            "onikibinüçyüzkırkbeşinci"
        );
        assert_eq!(l().ordinal_float_entry(&dc("1E+20")).unwrap(), "yüzkentilyon");
        // MAXVAL applies to non-integral values too (OverflowError, not "").
        assert!(matches!(
            l().ordinal_float_entry(&dc("1000000000000000000000.5")),
            Err(N2WError::Overflow(_))
        ));
        assert!(matches!(
            l().ordinal_float_entry(&fl(1e21)),
            Err(N2WError::Overflow(_))
        ));
    }

    #[test]
    fn ordinal_num_float_corpus() {
        let t = |v: FloatValue, r: &str| l().ordinal_num_float_entry(&v, r).unwrap();
        assert_eq!(t(fl(0.0), "0.0"), "0.0'ıncı");
        assert_eq!(t(fl(-0.0), "-0.0"), "-0.0'ıncı");
        assert_eq!(t(fl(1.0), "1.0"), "1.0'inci");
        assert_eq!(t(fl(2.0), "2.0"), "2.0'nci");
        assert_eq!(t(fl(3.0), "3.0"), "3.0'üncü");
        assert_eq!(t(fl(10.0), "10.0"), "10.0'uncu");
        assert_eq!(t(fl(1234.0), "1234.0"), "1234.0'üncü");
        assert_eq!(t(fl(1e16), "1e+16"), "1e+16'uncu");
        // Empty suffix: ORDINAL_TRIPLETS[6] quirk.
        assert_eq!(t(fl(1e20), "1e+20"), "1e+20'");
        // Non-integral: ordinal "" -> ""[-4:] -> "".
        assert_eq!(t(fl(0.5), "0.5"), "0.5'");
        assert_eq!(t(fl(3.25), "3.25"), "3.25'");
        assert_eq!(t(dc("5.00"), "5.00"), "5.00'inci");
        assert_eq!(t(dc("1E+2"), "1E+2"), "1E+2'üncü");
        assert_eq!(t(dc("1E+20"), "1E+20"), "1E+20'");
        assert!(matches!(
            l().ordinal_num_float_entry(&fl(-1.5), "-1.5"),
            Err(N2WError::Type(_))
        ));
        assert!(matches!(
            l().ordinal_num_float_entry(&fl(-1000000.0), "-1000000.0"),
            Err(N2WError::Type(_))
        ));
    }

    #[test]
    fn cardinal_float_entry_maxval() {
        // Huge non-whole Decimal: OverflowError from verify_cardinal, not a
        // float-grammar reading.
        assert!(matches!(
            l().cardinal_float_entry(&dc("1000000000000000000000.5"), None),
            Err(N2WError::Overflow(_))
        ));
        // Whole values still route through the integer reader.
        assert_eq!(l().cardinal_float_entry(&fl(1e20), None).unwrap(), "yüzkentilyon");
        assert_eq!(l().cardinal_float_entry(&fl(-0.0), None).unwrap(), "sıfır");
        // precision= rides precision_override (corpus: {"precision": 3}).
        assert_eq!(
            l().cardinal_float_entry(&fl(1.5), Some(3)).unwrap(),
            "birvirgülbeşyüz"
        );
    }

    #[test]
    fn str_nan_falls_back() {
        // NaN is now carried through natively (split per mode in `nan_result`),
        // so str_to_number succeeds rather than declining.
        assert!(l().str_to_number("NaN").is_ok());
        // Everything else stays on the generic parse.
        assert!(matches!(
            l().str_to_number("Infinity"),
            Ok(ParsedNumber::Inf { negative: false })
        ));
        assert!(matches!(l().str_to_number("5"), Ok(ParsedNumber::Dec(_))));
    }

    #[test]
    fn kwargs_corpus() {
        let sp = kw(&[("spaced", KwVal::Bool(true))]);
        let unsp = kw(&[("spaced", KwVal::Bool(false))]);
        let c = |n: i64, k: &Kwargs| l().to_cardinal_kw(&BigInt::from(n), k).unwrap();
        assert_eq!(c(0, &sp), "sıfır");
        assert_eq!(c(11, &sp), "on bir");
        assert_eq!(c(21, &sp), "yirmi bir");
        assert_eq!(c(100, &sp), "yüz");
        assert_eq!(c(1234, &sp), "bin iki yüz otuz dört");
        // The negword is not a token: "eksi" survives via the unknown-char
        // branch, and the next token gets the space.
        assert_eq!(c(-5, &sp), "eksi beş");
        assert_eq!(c(1234, &unsp), "binikiyüzotuzdört");
        assert_eq!(c(-5, &unsp), "eksibeş");

        let nokta = kw(&[("decimal_word", KwVal::Str("nokta".into()))]);
        let f = |v: FloatValue, k: &Kwargs| l().to_cardinal_float_kw(&v, None, k).unwrap();
        assert_eq!(f(fl(1.5), &nokta), "birnoktabeş");
        assert_eq!(f(fl(2.05), &nokta), "ikinoktasıfırbeş");
        assert_eq!(f(fl(1.5), &sp), "bir virgül beş");
        let both = kw(&[
            ("spaced", KwVal::Bool(true)),
            ("decimal_word", KwVal::Str("nokta".into())),
        ]);
        assert_eq!(f(fl(3.14), &both), "üç nokta on dört");
        // Whole float through the kw entry keeps the int reading.
        assert_eq!(f(fl(-5.0), &sp), "eksi beş");
        // Unknown kwarg -> Fallback (decline) -> Python raises its own TypeError.
        assert!(matches!(
            l().to_cardinal_kw(&BigInt::from(5), &kw(&[("case", KwVal::Str("x".into()))])),
            Err(N2WError::Fallback(_))
        ));
        // to_year swallows every kwarg unread.
        assert_eq!(
            l().to_year_kw(&BigInt::from(5), &sp).unwrap(),
            "beş"
        );
    }
}
