//! Port of `lang_TET.py` (Tetum), via its `lang_EUR` → `Num2Word_Base` ancestry.
//!
//! Shape: **engine**, driven locally. `Num2Word_TET` defines
//! `mid_numwords`/`low_numwords`/`set_high_numwords` + `merge`, so Python
//! builds `self.cards` and lets `Num2Word_Base.to_cardinal` run
//! `splitnum`/`clean`/`merge`. The card table and `MAXVAL` are therefore real
//! and are exposed through `cards()`/`maxval()`.
//!
//! **But the fold cannot use `base::clean`.** TET's `merge` mutates
//! `self.count` (see "Cross-call mutable state" below), and the `Lang` trait's
//! `merge` takes `&self`. Interior mutability is not an option either: the
//! generated PyO3 registry hands out `&'static (dyn Lang + Sync)`, so a
//! `Cell`/`RefCell` field would make `LangTet` non-`Sync` and fail to compile.
//! Instead [`LangTet::clean_counted`] mirrors `base::clean` byte for byte with
//! an explicit `&mut i64` counter threaded through the recursion, and
//! `to_cardinal`/`to_ordinal` are overridden to drive it. `base::splitnum` is
//! reused unchanged — it only reads `cards` and never calls `merge`.
//! The trait's `merge` is consequently left at its `unimplemented!()` default;
//! nothing on the TET path reaches it.
//!
//! Setup notes (chased through the inheritance chain):
//!   * `setup()` calls `super().setup()` (which builds the full EUR Latin-prefix
//!     `high_numwords`) and then **throws it away**, reassigning
//!     `high_numwords = gen_high_numwords([], [], ["kuatr","tr","b","m"])`.
//!     With empty `units`/`tens` the comprehension yields `[]`, so the result is
//!     just the `lows` list. The EUR elision table is dead code here.
//!   * `Num2Word_EUR.set_high_numwords`: `cap = 3 + 6*4 = 27`,
//!     `zip(high, range(27, 3, -6))` → 27, 21, 15, 9. `GIGA_SUFFIX` is `None`
//!     so the `10**n` entries are skipped entirely; only
//!     `cards[10**(n-3)] = word + "iliaun"` lands → 10^24 kuatriliaun,
//!     10^18 triliaun, 10^12 biliaun, 10^6 miliaun. Hence `MAXVAL = 10^27`.
//!   * `is_title` is never set, so `title()` is the identity. It is *not* a
//!     whitespace normaliser here — `exclude_title` (["resin","vírgula",
//!     "menus"]) is carried for fidelity but is unreachable.
//!   * `self.hundreds` is built in `setup()` and **never read** by any method.
//!
//! # Cross-call mutable state (IMPORTANT — flagged for the dispatcher)
//!
//! `setup()` sets `self.count = 0` and `merge` increments it, but **nothing
//! ever resets it**. `num2words2/__init__.py` instantiates
//! `Num2Word_TET()` once at import, so `self.count` is effectively
//! process-global and monotonically increasing across every call.
//!
//! The counter gates one branch: the first time `merge` folds a value whose
//! decimal middle digits are all "0" (e.g. 101), it returns without the "ho"
//! infix and bumps `count` to 1; **every** later such fold — in any later call
//! — returns "ho …" instead. So Python's answer depends on call history.
//!
//! This port models `count` as a **per-call local starting at 0** (equivalent
//! to a fresh converter instance per call). Justification, all verified
//! against the interpreter:
//!   * It reproduces all 305 in-scope corpus rows exactly.
//!   * The state is unobservable for every corpus input: presetting `count` to
//!     0/1/2/7/99 gives byte-identical output for all of them, because
//!     `ho_result`/`remove_ho` strip the "ho" back out again.
//!   * It equals what a fresh Python process emits for its first call.
//!
//! It is **not** unobservable in general. Inputs like 100101, 101001,
//! 1000101, 1001001, 10000101, … keep the "ho" (their `remove_ho` guard
//! short-circuits, see the `value_str[:-4]` quirk below), so real Python gives
//! "rihun atus ida atus ida ida" on a cold instance and
//! "rihun atus ida ho atus ida ida" once warmed. None of those values are in
//! the corpus. **The Python dispatcher must not be allowed to compare against
//! a warmed instance for such inputs.**
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are odd but are exactly
//! what Python emits:
//!
//! 1. `merge` renders 100 as "atus ida" (`ntext + " " + ctext`, i.e.
//!    "hundred one"), so 101 is "atus ida ida" and 1001 "rihun ida ida". The
//!    `hundreds` dict that would give "atus rua" etc. is never consulted.
//! 2. `remove_ho`'s guard is `value_str[:-4].endswith("0")` — a *string* test
//!    on the digits with the last four dropped. For 100001 that is "10", which
//!    ends in "0", so the whole strip block is skipped. It is a digit-pattern
//!    heuristic, not arithmetic, and is reproduced literally.
//! 3. `ho_result` does `result[3:]` to drop a leading "ho", assuming the third
//!    character is the space that `merge`'s "ho %s %s" put there.
//! 4. `ho_result` replaces `low + "iliaun" + " ho"` for lows in order
//!    ["kuatr","tr","b","m"]. The order is load-bearing: "kuatriliaun" *ends
//!    with* "triliaun", so the "kuatr" pass must consume it before the "tr"
//!    pass sees it. Preserved exactly.
//! 5. `to_ordinal` inlines a verbatim copy of `Num2Word_Base.clean` rather than
//!    calling it — same semantics, so [`LangTet::clean_counted`] serves both.
//! 6. `to_ordinal` computes `words_split = words.split()` **once**, then keeps
//!    reassigning `words` while continuing to read the now-stale `words_split`
//!    for `word_first` and for the final `len(str(num)) > 3` block.
//! 7. In that final block Python writes `"haat" in words_split[-1:]` — a slice,
//!    so this is *list* membership (exact equality against the last word), not
//!    the substring test used a few lines earlier on `second_word`. The two
//!    read alike and behave differently; both are reproduced as written.
//! 8. `merge`'s dead `self.count += 0` in the `count >= 1` arm is a no-op and
//!    is simply dropped.
//! 9. ~~`to_ordinal` never checks `MAXVAL`~~ (only `to_cardinal` does), so in
//!    Python ordinals from 10^27 are rendered, wrongly ("dakuatriliaun rihun
//!    idak" for 10^27). The port raises `OverflowError` there too (#215).
//!
//! # Error variants
//!
//! * negative `to_ordinal`/`to_ordinal_num` → `TypeError` via
//!   `Num2Word_Base.verify_ordinal` → [`N2WError::Type`].
//! * `to_cardinal(v)` with `abs(v) >= 10^27` → `OverflowError` →
//!   [`N2WError::Overflow`].
//! * `to_ordinal`'s `words_split[0]` would raise `IndexError` on empty `words`
//!   → [`N2WError::Index`]. Believed unreachable (no card word is empty), but
//!   modelled rather than silently swallowed.
//! * `to_currency` with `abs(val) >= 10^27` → `OverflowError`, like the
//!   cardinal. Python raised `decimal.InvalidOperation` from 10**26 already
//!   (its `(Decimal(str(val)) * 100) % 1` overran the 28-digit default
//!   context), below its own MAXVAL; the port does not (#215).
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). No reliable Tetum euro subunit or pound noun was found, so EUR
//! and GBP raise NotImplementedError. Examples in these docs that quote
//! English nouns record Python's output.

use crate::base::{check_maxval, set_low_numwords, set_mid_numwords, splitnum, Cards, Lang, N2WError, Node, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// `merge`'s "these numbers get a -k suffix" table, from `to_ordinal`.
/// Note the gaps: 1, 4 and 6 are deliberately absent (handled separately).
const ORD_SUFFIX_K: [i64; 15] = [90, 80, 70, 60, 50, 40, 30, 20, 10, 9, 8, 7, 5, 3, 2];
/// Numbers that take the "da" prefix but no "k" suffix.
const ORD_NO_K: [i64; 2] = [6, 4];
/// Hundreds taking "dah" + words + "k".
const ORD_HUNDREDS_K: [i64; 7] = [900, 800, 700, 500, 300, 200, 100];
/// Hundreds taking "dah" + words, no "k".
const ORD_HUNDREDS_NO_K: [i64; 2] = [600, 400];

/// `ho_result`'s `lows`, in the order Python iterates them. Order matters —
/// see quirk 4 in the module docs.
const LOWS: [&str; 4] = ["kuatr", "tr", "b", "m"];
const MEGA_SUFFIX: &str = "iliaun";

/// `Num2Word_TET.CURRENCY_FORMS`.
///
/// TET declares its **own** class-level dict, so — unlike the 16 classes that
/// read `Num2Word_EUR`'s — it is *not* touched by `Num2Word_EN.__init__`'s
/// in-place mutation of the shared EUR table. Here `lang_TET.py`'s source
/// really is what runs, confirmed against the live interpreter: five codes and
/// no more. EUR reads `("euro", "euros")` because TET spells that pair out
/// itself, not because English rewrote it; GBP keeps TET's own
/// `("pound sterling", "pound sterling")` / `("pence", "pence")` rather than
/// EUR's `("penny", "pence")`.
///
/// Consequently every code EN adds to the shared dict (JPY, KWD, BHD, INR,
/// CNY, CHF, …) raises NotImplementedError for TET — which is exactly what the
/// 60 error rows in the corpus expect.
///
/// Both forms of every entry are identical except EUR's, so `pluralize`'s
/// singular/plural choice is only observable on EUR.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Module-level `DOLLAR` / `CENTS` in lang_TET.py, shared by AUD/CAD/USD.
    const DOLLAR: [&str; 2] = ["dolar", "dolar"];
    const CENTS: [&str; 2] = ["sentavu", "sentavu"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AUD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("CAD", CurrencyForms::new(&DOLLAR, &CENTS));
    // EUR ("euros") and GBP ("pound sterling", "pence") were English; no
    // reliable Tetum subunit / pound noun was found, so they raise
    // NotImplementedError (#222).
    m.insert("USD", CurrencyForms::new(&DOLLAR, &CENTS));
    m
}

/// Round to the default decimal context's 28 significant digits, ROUND_HALF_EVEN.
///
/// `Decimal.__mul__` rounds its result to `prec` significant digits, so
/// `decimal_val * 100` is only exact while the operand carries at most 28 of
/// them. Only a caller-supplied `Decimal` ever exceeds that: a float's `repr`
/// yields at most 17 significant digits, and an int big enough to overflow 28
/// raises at the `% 1` step regardless of how the product was rounded.
///
/// `Decimal("1.5000000000000000000000000001")` is the case that bites. Python
/// rounds the product to `150.0000000000000000000000000`, so `% 1` is 0 and
/// `has_fractional_cents` comes out **False** — "dolar ida sentavu lima nulu".
/// Exact `BigDecimal` arithmetic would say True and take the fractional
/// branch, so the context rounding is reproduced rather than idealised away.
///
/// Only the *value* is reproduced, not Python's coefficient/exponent
/// normalisation: the one caller merely compares a magnitude and tests
/// integrality, and both are exponent-invariant.
fn decimal_context_round(v: &BigDecimal) -> BigDecimal {
    const PREC: u64 = 28;

    let digits = v.digits();
    if digits <= PREC {
        return v.clone();
    }
    let diff = digits - PREC;
    let (int_val, scale) = v.as_bigint_and_exponent();
    let p = BigInt::from(10).pow(diff as u32);

    // BigInt's div_rem truncates toward zero, so round the magnitude and
    // reapply the sign — ROUND_HALF_EVEN is symmetric about zero.
    let negative = int_val.is_negative();
    let (mut q, r) = int_val.abs().div_rem(&p);
    let twice = &r * BigInt::from(2);
    if twice > p || (twice == p && q.is_odd()) {
        q += BigInt::one();
    }
    if negative {
        q = -q;
    }
    BigDecimal::new(q, scale - diff as i64)
}

pub struct LangTet {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangTet {
    fn default() -> Self {
        Self::new()
    }
}

impl LangTet {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords over
        // high = gen_high_numwords([], [], ["kuatr","tr","b","m"]) == the lows.
        // cap = 3 + 6*len(high) = 27; zip(high, range(cap, 3, -6)).
        // GIGA_SUFFIX is None -> no 10**n card; MEGA_SUFFIX -> 10**(n-3).
        let high = LOWS;
        let mut n: i64 = 3 + 6 * high.len() as i64;
        for word in high.iter() {
            if n <= 3 {
                break; // mirrors range(cap, 3, -6) running out
            }
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}{}", word, MEGA_SUFFIX),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "rihun"),
                (100, "atus"),
                (90, "sia nulu"),
                (80, "ualu nulu"),
                (70, "hitu nulu"),
                (60, "neen nulu"),
                (50, "lima nulu"),
                (40, "haat nulu"),
                (30, "tolu nulu"),
                (20, "rua nulu"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "sanulu", "sia", "ualu", "hitu", "neen", "lima", "haat", "tolu", "rua", "ida",
                "mamuk",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0] == 1000 * 10^24 == 10^27.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangTet {
            cards,
            maxval,
            // Carried for fidelity; is_title is always False so title() never
            // consults it.
            exclude_title: vec!["resin".into(), "vírgula".into(), "menus".into()],
            // Built once here, never per call — `to_currency`/`to_cheque` only
            // ever read it.
            currency_forms: build_currency_forms(),
        }
    }

    /// `Num2Word_TET.merge`, with `self.count` passed explicitly.
    ///
    /// Python names the args `curr`/`next` and unpacks `curr + next` (tuple
    /// concatenation) into `ctext, cnum, ntext, nnum`.
    fn merge_counted(
        &self,
        curr: (&str, &BigInt),
        next: (&str, &BigInt),
        count: &mut i64,
    ) -> (String, BigInt) {
        let (ctext, cnum) = curr;
        let (ntext, nnum) = next;
        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);

        if cnum.is_one() && nnum < &hundred {
            return (ntext.to_string(), nnum.clone());
        }

        if nnum < cnum {
            if nnum < &ten {
                let sum = cnum + nnum;
                // Python: value_str = str(cnum + nnum); if int(value_str) > 100
                if sum > hundred {
                    let chars: Vec<char> = sum.to_string().chars().collect();
                    // value_str[1:-1] — the middle digits. Always non-empty
                    // here (sum > 100 => at least 3 digits), but guard anyway.
                    let zero_list: &[char] = if chars.len() >= 2 {
                        &chars[1..chars.len() - 1]
                    } else {
                        &[]
                    };
                    // Python's all() over an empty slice is True.
                    let all_zero = zero_list.iter().all(|c| *c == '0');
                    if all_zero {
                        if *count >= 1 {
                            // Python does a no-op `self.count += 0` here.
                            return (format!("ho {} {}", ctext, ntext), sum);
                        }
                        *count += 1;
                        return (format!("{} {}", ctext, ntext), sum);
                    }
                }
                return (format!("{} resin {}", ctext, ntext), sum);
            } else {
                return (format!("{} {}", ctext, ntext), cnum + nnum);
            }
        }

        (format!("{} {}", ntext, ctext), cnum * nnum)
    }

    /// `Num2Word_Base.clean`, mirrored with an explicit merge counter.
    ///
    /// Identical in structure to `base::clean` (including appending the tail
    /// `val[2:]` as a *nested list* rather than splicing it); the only reason
    /// it is duplicated here is that `merge` needs `&mut count`, which the
    /// `Lang` trait's `&self` signature cannot carry. `to_ordinal` inlines this
    /// same loop in Python, so both entry points share it.
    fn clean_counted(&self, val: Vec<Node>, count: &mut i64) -> Node {
        let mut val = val;
        while val.len() != 1 {
            let mut out: Vec<Node> = Vec::new();
            let both_leaf = val.len() >= 2
                && matches!(&val[0], Node::Leaf(..))
                && matches!(&val[1], Node::Leaf(..));
            if both_leaf {
                let (lt, ln) = match &val[0] {
                    Node::Leaf(t, n) => (t.clone(), n.clone()),
                    _ => unreachable!(),
                };
                let (rt, rn) = match &val[1] {
                    Node::Leaf(t, n) => (t.clone(), n.clone()),
                    _ => unreachable!(),
                };
                let (mt, mn) = self.merge_counted((&lt, &ln), (&rt, &rn), count);
                out.push(Node::Leaf(mt, mn));
                if val.len() > 2 {
                    out.push(Node::List(val[2..].to_vec()));
                }
            } else {
                for elem in val.into_iter() {
                    match elem {
                        Node::List(inner) => {
                            if inner.len() == 1 {
                                out.push(inner.into_iter().next().unwrap());
                            } else {
                                out.push(self.clean_counted(inner, count));
                            }
                        }
                        leaf => out.push(leaf),
                    }
                }
            }
            val = out;
        }
        val.into_iter().next().unwrap()
    }

    /// `Num2Word_TET.ho_result`.
    fn ho_result(&self, result: &str, value: &BigInt) -> String {
        let mut result = result.to_string();

        // Python: index = result.find("ho"); count_ho = result.count("ho")
        //         if index != -1 and count_ho >= 1
        // The second test is implied by the first; both reduce to "contains".
        if !result.contains("ho") {
            return result;
        }

        // Python: value_str = len(str(value)) — a *length*, and str() keeps the
        // minus sign for negatives (to_cardinal passes the signed original).
        let value_len = value.to_string().chars().count();
        if result.contains("rihun") && value_len > 7 {
            result = result.replace("rihun ho", "ho rihun");
        }

        for low in LOWS.iter() {
            result = result.replace(
                &format!("{}{} ho", low, MEGA_SUFFIX),
                &format!("ho {}{}", low, MEGA_SUFFIX),
            );
        }

        if result.starts_with("ho") {
            // Python: result[3:] — drops "ho" plus the space merge emitted.
            // Character-indexed, not byte-indexed.
            result = result.chars().skip(3).collect();
        }
        result
    }

    /// `Num2Word_TET.remove_ho`.
    fn remove_ho(&self, result: &str, value: &BigInt) -> String {
        let value_str = value.to_string();
        let mut result = self.ho_result(result, value);

        let vchars: Vec<char> = value_str.chars().collect();
        // Python: end_value = value_str[:-4] ("" when len <= 4).
        let end_value: String = vchars[..vchars.len().saturating_sub(4)].iter().collect();
        let end_true = end_value.ends_with('0');

        if !end_true && value > &BigInt::from(100) {
            // value > 100 guarantees at least 3 chars, so both indexes are safe.
            let last = vchars[vchars.len() - 1];
            let second_last = vchars[vchars.len() - 2];
            if last != '0' && second_last == '0' {
                result = result.replace("ho", "");
                result = result.replace("  ", " ");
            }
        }
        result
    }

    /// `Num2Word_TET.ho_result`, parameterised by the value's decimal *string*
    /// length rather than a `BigInt`, for the float/Decimal path.
    ///
    /// Identical in structure to [`LangTet::ho_result`]; the only difference is
    /// that `str(value)` on the float path carries a decimal point (and a sign),
    /// so `len(str(value))` and the `value_str > 7` test see those characters
    /// too. Python computes `value_str = len(str(value))` — a *length* — so the
    /// caller passes `value_str.chars().count()`.
    fn ho_result_str(&self, result: &str, value_len: usize) -> String {
        let mut result = result.to_string();

        // Python: `if index != -1 and count_ho >= 1` — i.e. "ho" present.
        if !result.contains("ho") {
            return result;
        }

        if result.contains("rihun") && value_len > 7 {
            result = result.replace("rihun ho", "ho rihun");
        }

        for low in LOWS.iter() {
            result = result.replace(
                &format!("{}{} ho", low, MEGA_SUFFIX),
                &format!("ho {}{}", low, MEGA_SUFFIX),
            );
        }

        if result.starts_with("ho") {
            result = result.chars().skip(3).collect();
        }
        result
    }

    /// `Num2Word_TET.remove_ho`, driven by `str(value)` / `value > 100` for the
    /// float/Decimal path.
    ///
    /// TET's `to_cardinal` calls this on the whole float cardinal string with
    /// the ORIGINAL float/Decimal value. That matters: the default float path
    /// already ran `remove_ho` on the integer part via `to_cardinal(pre)`, but
    /// that inner pass used the *integer* `pre`. This outer pass uses `value`,
    /// whose `str()` carries the decimal point — so `value_str[-2] == "0"` can
    /// match a *fractional* digit (e.g. `.01`, `.005`, `.05`) and strip a "ho"
    /// the inner pass left in place. `604603031.01` is such a case: the integer
    /// rendering keeps "…haat ho rihun…", and only this outer pass removes it.
    fn remove_ho_value(&self, result: &str, value_str: &str, gt_100: bool) -> String {
        let vchars: Vec<char> = value_str.chars().collect();
        let mut result = self.ho_result_str(result, vchars.len());

        // Python: end_value = value_str[:-4]; end_true = end_value.endswith("0")
        let end_value: String = vchars[..vchars.len().saturating_sub(4)].iter().collect();
        let end_true = end_value.ends_with('0');

        if !end_true && gt_100 {
            // Python indexes value_str[-1] and value_str[-2] unconditionally
            // under `value > 100`; a decimal string for such a value always has
            // at least two chars, but guard rather than risk a panic.
            if vchars.len() >= 2 {
                let last = vchars[vchars.len() - 1];
                let second_last = vchars[vchars.len() - 2];
                if last != '0' && second_last == '0' {
                    result = result.replace("ho", "");
                    result = result.replace("  ", " ");
                }
            }
        }
        result
    }

    /// `Num2Word_Base.verify_ordinal`. The float test is unreachable for
    /// integer input; only the negative test can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.to_cardinal` for integral input, with the merge counter
    /// threaded through. TET's own `to_cardinal` wraps this in `remove_ho`.
    fn cardinal_words(&self, value: &BigInt, count: &mut i64) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            // negword is "menus "; Python emits "%s " % negword.strip().
            out = format!("{} ", self.negword().trim());
        }

        if v >= self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v, self.maxval
            )));
        }

        let tree = splitnum(self, &v).ok_or_else(|| {
            N2WError::Overflow(format!("abs({}) must be less than {}.", v, self.maxval))
        })?;
        let words = match self.clean_counted(tree, count) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        Ok(self.title(&format!("{}{}", out, words)))
    }
}

impl Lang for LangTet {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "USD"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "menus "
    }
    fn pointword(&self) -> &str {
        "vírgula"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    // `merge` is intentionally left at the trait default. TET's merge needs
    // `&mut self.count`, which `&self` cannot express, so the fold runs through
    // `clean_counted`/`merge_counted` instead. Nothing on this path calls it.

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python reads the process-global `self.count`; we start fresh per call.
        // See "Cross-call mutable state" in the module docs.
        let mut count: i64 = 0;
        let result = self.cardinal_words(value, &mut count)?;
        // Python passes the *original* (possibly negative) value here.
        Ok(self.remove_ho(&result, value))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut count: i64 = 0;
        // Python's `out` is initialised to "" and never assigned again.
        let out = "";

        // Python's to_ordinal does NOT check MAXVAL, and splitnum then reads
        // 10**27 as "kuatriliaun rihun" (a thousand quadrillion). The port
        // checks it like to_cardinal does (#215). splitnum only returns None
        // for negatives, which verify_ordinal has already rejected; Python
        // would hit `len(None)` -> TypeError, so model that shape.
        check_maxval(value, &self.maxval)?;
        let tree = splitnum(self, value).ok_or_else(|| {
            N2WError::Type("object of type 'NoneType' has no len()".into())
        })?;
        let (mut words, num) = match self.clean_counted(tree, &mut count) {
            Node::Leaf(t, n) => (t, n),
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };

        words = self.remove_ho(&words, value);

        let in_list = |ks: &[i64]| ks.iter().any(|k| num == BigInt::from(*k));

        // Sequential `if`s in Python, not elif — several can fire in turn.
        if in_list(&ORD_SUFFIX_K) {
            words = format!("da{}k", words);
        }
        if in_list(&ORD_NO_K) {
            words = format!("da{}", words);
        }
        if num.is_one() {
            words = "dahuluk".to_string();
        }
        if in_list(&ORD_HUNDREDS_K) {
            words = format!("dah{}k", words);
        }
        if in_list(&ORD_HUNDREDS_NO_K) {
            words = format!("dah{}", words);
        }

        // Computed ONCE; `words` keeps changing underneath it (quirk 6).
        let words_split: Vec<String> = words.split_whitespace().map(|s| s.to_string()).collect();

        if words_split.len() >= 3 && num < BigInt::from(100) {
            let first_word = format!("da{}", words_split[0]);
            let second_word = words_split[1..].join(" ");
            // Substring test (contrast with the list test below).
            if second_word.contains("haat") || second_word.contains("neen") {
                words = format!("{} {}", first_word, second_word);
            } else {
                words = format!("{} {}k", first_word, second_word);
            }
        }

        // Python indexes words_split[0] unconditionally -> IndexError if empty.
        let first = words_split
            .first()
            .ok_or_else(|| N2WError::Index("list index out of range".into()))?;
        let word_first = format!("dah{}", first);
        if word_first == "dahatus" && words_split.len() >= 3 {
            let word_second = words_split[1..].join(" ");
            if word_second.contains("haat") || word_second.contains("neen") {
                words = format!("{} {}", word_first, word_second);
            } else {
                words = format!("{} {}k", word_first, word_second);
            }
        }

        if num.to_string().chars().count() > 3 {
            // Python: `"haat" in words_split[-1:]` — a LIST slice, so this is
            // exact equality against the last word, not a substring test.
            let last_is = |w: &str| words_split.last().map(|s| s.as_str() == w).unwrap_or(false);
            if last_is("haat") || last_is("neen") {
                words = format!("da{}", words);
            } else {
                words = format!("da{}k", words);
            }
        }

        Ok(self.title(&format!("{}{}", out, words)))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{} antes Kristu", self.to_cardinal(&value.abs())?));
        }
        self.to_cardinal(value)
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for a
    /// non-integral value first; whole values run the same splitnum/suffix
    /// pipeline as the integer path (numeric comparisons make 5.0 and 5
    /// indistinguishable there), and the negative TypeError comes from
    /// `to_ordinal(BigInt)`'s own verify.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => self.to_ordinal(&i),
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_ordinal_num(float/Decimal)`: same `verify_ordinal`, then
    /// `"%sº" % value` with Python's `str(value)` ("5.0º", "-0.0º").
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    return Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )));
                }
                Ok(format!("{}º", repr_str))
            }
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_year(float/Decimal)`: `if val < 0` (numeric) →
    /// `to_cardinal(abs(val)) + " antes Kristu"`, keeping the float grammar
    /// ("ida vírgula lima antes Kristu" for -1.5).
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
            Ok(format!(
                "{} antes Kristu",
                self.cardinal_float_entry(&abs, None)?
            ))
        } else {
            self.cardinal_float_entry(value, None)
        }
    }

    /// `Num2Word_TET.to_cardinal` on the non-integral branch.
    ///
    /// TET does not override `to_cardinal_float`; it overrides `to_cardinal`,
    /// which for non-integral input runs
    /// `remove_ho(super().to_cardinal(value), value)`. `super().to_cardinal`
    /// (`Num2Word_Base.to_cardinal`) delegates non-integral input straight to
    /// `Num2Word_Base.to_cardinal_float` — the default float path — so the
    /// port is `remove_ho(default_to_cardinal_float(value), value)`.
    ///
    /// The `num2words` dispatcher routes only *non-whole* floats here (whole
    /// floats like `1.0` satisfy `int(value) == value` and take the integer
    /// `to_cardinal` path instead), so this method never sees a whole value —
    /// matching the 26 languages that inherit the bare default unchanged.
    ///
    /// Why the wrapper is not redundant: the default path already applies
    /// `remove_ho` to the integer part inside `to_cardinal(pre)`, but with the
    /// integer `pre`. This outer pass re-applies it with the ORIGINAL value,
    /// whose `str()` carries the fractional digits — verified against the live
    /// interpreter to be the only thing that strips a stray "ho" out of e.g.
    /// `604603031.01`. On every corpus row the string has no "ho" and the pass
    /// is a no-op, so it changes nothing there.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let result =
            crate::floatpath::default_to_cardinal_float(self, value, precision_override)?;

        // `str(value)` and `value > 100`, from the original float/Decimal.
        // f64's `{}` is shortest-round-trip (same contract as Python's repr),
        // and BigDecimal's `{}` preserves scale, so both reproduce the digit
        // string `remove_ho` slices. `precision_override` does not affect this
        // string — Python reads `str(value)`, not the rescaled precision.
        let (value_str, gt_100) = match value {
            FloatValue::Float { value, .. } => (format!("{}", value), *value > 100.0),
            FloatValue::Decimal { value, .. } => {
                (format!("{}", value), value > &BigDecimal::from(100))
            }
        };
        Ok(self.remove_ho_value(&result, &value_str, gt_100))
    }

    // ---- currency -------------------------------------------------------
    //
    // TET overrides `to_currency` wholesale and inherits everything else:
    // `to_cheque`, `_money_verbose`, `_cents_verbose` and `_cents_terse` from
    // `Num2Word_Base`, `pluralize` from `Num2Word_EUR`. The trait defaults
    // already mirror all of those, so only `pluralize`, the class name, the
    // forms table and `to_currency` itself appear here.
    //
    // `currency_precision` is deliberately NOT overridden: TET never declares
    // `CURRENCY_PRECISION`, and `Num2Word_EN.__init__` *rebinds* rather than
    // mutates it, so nothing leaks in. `.get(code, 100)` is therefore always
    // 100 — the trait default. TET's `to_currency` hardcodes 100 anyway, and
    // it implements no 3-decimal (KWD/BHD) or 0-decimal (JPY) currency: those
    // codes are absent from the table and raise instead.
    //
    // `currency_adjective` is likewise not overridden. TET *does* inherit
    // `Num2Word_EUR.CURRENCY_ADJECTIVES`, but its `to_currency` signature is
    // `(val, currency="USD", cents=True)` — there is no `adjective` parameter
    // and no `prefix_currency` call anywhere on the path, so the table is dead
    // data. `to_cheque` never consults adjectives either. Wiring it up would
    // advertise a behaviour Python does not have.

    fn lang_name(&self) -> &str {
        "Num2Word_TET"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. Every TET entry has two forms, so this is
    /// unreachable — mapped to `Index` rather than panicking, as in `lang_en`.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_TET.to_currency`.
    ///
    /// Python's signature is `(val, currency="USD", cents=True)` — it has
    /// neither `separator` nor `adjective`, hence the two ignored parameters
    /// (and hence no generated `default_separator`: there is no separator to
    /// default). TET emits no separator at all; the cents segment is joined
    /// with a plain space.
    ///
    /// # Faithfully reproduced Python quirks
    ///
    /// 1. **Word order is inverted relative to `Num2Word_Base.to_currency`.**
    ///    Base emits `money_str` then the currency name ("two dollars"); TET
    ///    emits the currency name *first* — `"%s%s %s" % (minus_str,
    ///    curr_name, money_str)` → "dolar rua". The corpus confirms it.
    /// 2. The `is_integer_input` early return is **dead code**. It formats
    ///    identically to the `right == 0` branch below it, and for an `int`
    ///    `parse_currency_parts(..., is_int_with_cents=False)` always returns
    ///    `right == 0`, so control could never reach one without the other
    ///    agreeing. Kept because it is what Python executes.
    /// 3. `parse_currency_parts` is called with the default
    ///    `keep_precision=False` **even when `has_fractional_cents` is true**,
    ///    so `right` is always the ROUND_HALF_UP-quantized whole-cent count.
    ///    The "fractional" branch then re-derives cents as `right / 100.0` —
    ///    from the already-rounded value. 1.011 USD is therefore
    ///    "dolar ida mamuk vírgula mamuk ida sentavu" (…0.01…), not 1.1 cents.
    /// 4. `pluralize(1, cr2)` — hardcoded singular — in the fractional branch,
    ///    and the operands swap places: whole cents render as
    ///    `pluralize(right, cr2)` then `cents_str`, fractional cents as
    ///    `cents_str` then `pluralize(1, cr2)`.
    /// 5. The `cents` flag is ignored on the fractional path: `cents=False`
    ///    still spells 1.011 out in words, because the
    ///    `has_fractional_cents and right > 0` test precedes it.
    /// 6. `hasattr(self, "to_cardinal_float")` is always true —
    ///    `Num2Word_Base` defines it — so the `else` arm of that ternary is
    ///    unreachable and is not modelled.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // decimal_val = Decimal(str(val)) — the *signed* value; TET takes no
        // abs here, unlike several sibling languages.
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };

        // Python's `% 1` below raised InvalidOperation(DivisionImpossible)
        // from 10**26, under its own 10**27 MAXVAL. The ceiling is checked here
        // instead, before the CURRENCY_FORMS lookup (#215).
        check_maxval(&decimal_val.with_scale(0).as_bigint_and_exponent().0, &self.maxval)?;

        // has_fractional_cents = (decimal_val * 100) % 1 != 0
        //
        // The 100 is hardcoded in Python; CURRENCY_PRECISION is never consulted
        // on this path. The context rounding fires before the CURRENCY_FORMS
        // lookup below, matching Python's statement order.
        let scaled = decimal_context_round(&(&decimal_val * BigDecimal::from(100)));
        // `scaled >= 0` or not, with_scale(0) truncates toward zero — the same
        // direction Decimal's `%` uses, so the != 0 test agrees on both signs.
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // parse_currency_parts(val, is_int_with_cents=False): keep_precision
        // and divisor stay at their defaults (False, 100).
        let (left, right, is_negative) = parse_currency_parts(val, false, false, 100);
        // keep_precision=False means `right` came through `with_scale(0)`, so
        // its scale is 0 and this is the integer Python's `int(...)` produced.
        let right = right.as_bigint_and_exponent().0;

        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // negword is "menus "; Python emits "%s " % self.negword.strip().
        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        let money_str = self.money_verbose(&left, currency)?;

        if is_integer_input {
            let curr_name = self.pluralize(&left, cr1)?;
            return Ok(format!("{}{} {}", minus_str, curr_name, money_str));
        }

        let cents_str = if has_fractional_cents && right.is_positive() {
            // fractional_cents = right / 100.0 -> self.to_cardinal_float(...).
            // `right` is a whole cent count in 0..=99, so right/100 is exact as
            // a 2-place decimal and `cardinal_from_decimal` — which casts to
            // f64 and derives the precision from its repr, exactly as Python's
            // float path does — recovers the same double.
            self.cardinal_from_decimal(&BigDecimal::new(right.clone(), 2))?
        } else if cents {
            self.cents_verbose(&right, currency)?
        } else {
            self.cents_terse(&right, currency)?
        };

        if right.is_zero() {
            Ok(format!(
                "{}{} {}",
                minus_str,
                self.pluralize(&left, cr1)?,
                money_str
            ))
        } else if has_fractional_cents {
            Ok(format!(
                "{}{} {} {} {}",
                minus_str,
                self.pluralize(&left, cr1)?,
                money_str,
                cents_str,
                // Hardcoded singular — Python passes a literal 1.
                self.pluralize(&BigInt::one(), cr2)?
            ))
        } else {
            Ok(format!(
                "{}{} {} {} {}",
                minus_str,
                self.pluralize(&left, cr1)?,
                money_str,
                self.pluralize(&right, cr2)?,
                cents_str
            ))
        }
    }
}
