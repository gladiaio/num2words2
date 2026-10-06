//! Port of `lang_KU.py` (Kurdish / Kurmanji).
//!
//! Shape: **self-contained**. `Num2Word_KU` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so the
//! `hasattr` guard in `Num2Word_Base.__init__` never fires: Python never builds
//! `self.cards` and never sets `self.MAXVAL`. `to_cardinal` is overridden
//! outright and drives a hand-written `_int_to_word` recursion. Consequently
//! `cards`/`maxval`/`merge` stay at their trait defaults here, and there is
//! **no overflow check at all** — see bug 1 below for what happens instead.
//!
//! Inherited from `Num2Word_Base` but overridden by KU (so the trait defaults
//! are *not* used):
//!   * `to_ordinal`     — KU's own; notably does **not** call `verify_ordinal`,
//!     which is why negative ordinals succeed instead of raising (bug 4).
//!   * `to_ordinal_num` — KU's own: `str(number) + "em"`.
//!   * `to_year`        — KU's own: ignores `longval` and the base class's
//!     BC/AD machinery, delegating straight to `to_cardinal`.
//!
//! `setup()` also assigns `self.exclude_title = ["û", "xal", "negatîv"]`, but
//! `Num2Word_Base.__init__` leaves `is_title = False` and KU's `to_cardinal`
//! never calls `self.title()`. The list is therefore dead — `is_title()` and
//! `exclude_title()` are left at their trait defaults deliberately.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below looks wrong and is exactly
//! what Python emits; each is pinned by a row in the frozen corpus.
//!
//! 1. **`_int_to_word` gives up at 10^9 and returns bare digits.** The final
//!    `return str(number)` has no word forms behind it, so `to_cardinal(10**9)`
//!    == `"1000000000"` — the *numeral*, not words. There is no "milyar"
//!    branch. This is the de facto ceiling, and it degrades silently rather
//!    than raising `OverflowError`. Hence `to_ordinal(10**9)` ==
//!    `"1000000000em"`, identical to `to_ordinal_num(10**9)`.
//! 2. **Asymmetric "yek" suppression.** The hundreds branch suppresses the
//!    multiplier for exactly one hundred (`if h > 1`), so 100 == `"sed"`, but
//!    the thousand and million branches recurse unconditionally, so 1000 ==
//!    `"yek hezar"` and 10^6 == `"yek milyon"`. The suppression then leaks
//!    through the recursion: 100000 == `"sed hezar"` (not "yek sed hezar") and
//!    10^8 == `"sed milyon"`.
//! 3. **Ordinals concatenate "em" with no separator or elision**, producing a
//!    doubled vowel on words already ending in "e"/"ê": `to_ordinal(3)` ==
//!    `"sêem"`, `to_ordinal(10)` == `"dehem"`. Preserved verbatim.
//! 4. **Negative ordinals do not raise.** KU's `to_ordinal` skips
//!    `verify_ordinal`, so `to_ordinal(-1)` == `"negatîv yekem"` — the "em"
//!    lands on the end of a negated cardinal. `to_ordinal_num(-1)` == `"-1em"`.
//! 5. **`tens[1]` ("deh") is unreachable.** The `< 100` branch only runs for
//!    `number >= 20` (10..19 is consumed by `teens`), so the tens quotient is
//!    never 1. Kept in the table for positional fidelity.
//! 6. **`to_ordinal(1)`'s special case is a no-op.** `if number == 1: return
//!    "yekem"` produces exactly what the general path would (`to_cardinal(1)`
//!    + `"em"` == `"yek"` + `"em"`). Harmless, but reproduced as written.
//!
//! # Currency
//!
//! `Num2Word_KU` declares its **own** `CURRENCY_FORMS` class attribute, so it is
//! untouched by the `lang_EUR` / `Num2Word_EN.__init__` mutation trap described
//! in PORTING_CURRENCY.md — EN mutates `Num2Word_EUR`'s dict, and KU does not
//! share it. The live table is exactly the source literal, four codes wide.
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}`, so
//! `currency_precision` (always 100) and `currency_adjective` (always `None`)
//! stay at their trait defaults.
//!
//! KU overrides `to_currency` outright and defines `pluralize`. Everything else
//! on the currency path — `to_cheque`, `_money_verbose`, `_cents_verbose`,
//! `_cents_terse` — is inherited from `Num2Word_Base` unchanged, and the trait
//! defaults already mirror those.
//!
//! ## More faithfully reproduced Python bugs
//!
//! 7. **An unknown currency code does not raise; it silently becomes TRY.**
//!    `to_currency` looks the code up with
//!    `CURRENCY_FORMS.get(currency, list(CURRENCY_FORMS.values())[0])`, and that
//!    default is the *first inserted* entry — TRY. So `to_currency(0, "JPY")` is
//!    `"sifir lîre"`, quietly denominated in Turkish lira. Only `to_cheque`,
//!    which is Base's and subscripts the dict, raises NotImplementedError. The
//!    corpus pins both halves: every one of the nine codes it exercises returns
//!    a value through `to_currency`, while GBP/JPY/KWD/BHD/INR/CNY/CHF raise
//!    through `to_cheque`.
//! 8. **`cents=False` drops the subunit segment instead of writing it tersely.**
//!    Base would emit `"... 56"` via `_cents_terse`; KU's `if cents and right:`
//!    just omits it, so `to_currency(12.34, "USD", cents=False)` is
//!    `"donzdeh dolar"`.
//! 9. **`adjective=` is accepted and then completely ignored.** KU never touches
//!    `CURRENCY_ADJECTIVES` (which is empty anyway), so `adjective=True` changes
//!    nothing.
//! 10. **Cents are truncated to two digits, not rounded, and short fractions are
//!     right-padded.** `int(parts[1][:2].ljust(2, "0"))` means `12.345` -> 34
//!     (not 35), `12.005` -> `int("00")` -> 0 -> *no cents at all*, `0.099` -> 9,
//!     and `1.5` -> 50. A 3-decimal currency gets no special treatment: KU has no
//!     `CURRENCY_PRECISION`, so KWD is truncated at 2 digits like everything else
//!     (and falls back to TRY's forms per bug 7 regardless).
//! 11. **Bug 1's digit fallback leaks into money.** `to_currency` calls
//!     `_int_to_word` directly, so `to_currency(10**9, "USD")` is
//!     `"1000000000 dolar"` and `to_currency(1234567890.12, "USD")` is
//!     `"1234567890 dolar donzdeh sent"`.
//! 12. **`pluralize` is dead code.** KU defines it, but `to_currency` inlines its
//!     own `cr1[1] if left != 1 else cr1[0]` and Base's `to_cheque` takes `cr1[-1]`
//!     unconditionally, so nothing in the library reaches it. It is ported anyway
//!     (the trait default *raises*, which would be wrong for KU) and differs from
//!     `Num2Word_EUR.pluralize`: it returns `forms[-1]`, not `forms[1]`, and
//!     answers `""` for an empty tuple rather than raising IndexError.
//! 13. **The separator is concatenated with no following space.** Base formats
//!     `"%s%s %s %s"`, putting a space *after* the separator; KU writes
//!     `result += separator + self._int_to_word(right) + " " + ...`. KU's own
//!     default (`" "`) hides this, but an explicit `separator=","` yields
//!     `"donzdeh dolar,sî û çar sent"` where Base's languages give `", sî..."`.
//!
//! Note that all four of KU's entries — and TRY's fallback — carry *identical*
//! singular and plural forms (`("lîre", "lîre")`), so the `left != 1` branch is
//! unobservable through the shipped table. It is ported literally regardless.
//!
//! # Notes on the port
//!
//! Python's `to_cardinal` is string-driven: it does `n = str(number).strip()`,
//! tests `n.startswith("-")`, and recurses on the substring `n[1:]`. For
//! integral input that is exactly "negate and recurse on the absolute value",
//! which is what [`LangKu::cardinal`] does. The `"." in n` branch (float input)
//! is out of scope per PORTING.md and omitted.
//!
//! `_int_to_word` is never reached with a negative: `to_cardinal` strips the
//! sign before calling it. (Were it, Python's negative list indexing would make
//! `self.ones[-1]` == "neh" — a latent trap this port cannot hit, since the
//! only in-scope caller guarantees a non-negative argument.)

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`. The trailing space is load-bearing: Python builds
/// `negword + cardinal` and only then `.strip()`s the result.
const NEGWORD: &str = "negatîv ";

/// `self.ones`. Index 0 is `""` — unreachable from `_int_to_word` (the
/// `number == 0` guard fires first), but used by the float path Python keeps.
const ONES: [&str; 10] = [
    "", "yek", "du", "sê", "çar", "pênc", "şeş", "heft", "heşt", "neh",
];

/// `self.teens`, indexed by `number - 10` for 10..=19.
const TEENS: [&str; 10] = [
    "deh", "yanzdeh", "donzdeh", "sêzdeh", "çardeh", "panzdeh", "şanzdeh", "hivdeh", "hijdeh",
    "nozdeh",
];

/// `self.tens`. Index 0 is `""` and index 1 ("deh") is dead code — see bug 5.
const TENS: [&str; 10] = [
    "", "deh", "bîst", "sî", "çil", "pêncî", "şêst", "heftê", "heştê", "not",
];

const HUNDRED: &str = "sed";
const THOUSAND: &str = "hezar";
const MILLION: &str = "milyon";

/// The word for zero, emitted by `_int_to_word(0)`.
const ZERO: &str = "sifir";

/// The joiner between a magnitude and its remainder: Python's `" û "`.
const AND: &str = " û ";

/// The code `to_currency`'s unknown-code fallback resolves to (bug 7).
///
/// Python writes `list(self.CURRENCY_FORMS.values())[0]`, i.e. the first entry
/// in the class dict's *insertion* order. `CURRENCY_FORMS` is a dict literal
/// whose first key is "TRY", and dicts have preserved insertion order since
/// CPython 3.7 — so that expression is exactly TRY's forms. Naming the code
/// pins the semantics to the literal rather than to a `HashMap`, which has no
/// order to borrow.
const FALLBACK_CURRENCY: &str = "TRY";

/// `Num2Word_KU.CURRENCY_FORMS`, in the source dict's insertion order.
///
/// KU declares this on its own class, so — unlike the 16 classes that read
/// `Num2Word_EUR`'s shared dict — nothing mutates it at import time. What is
/// written here is what runs; verified against the live interpreter.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    // Insertion order matters only for FALLBACK_CURRENCY above; the map itself
    // is unordered, so the first key is called out by name rather than implied.
    m.insert("TRY", CurrencyForms::new(&["lîre", "lîre"], &["qurûş", "qurûş"]));
    m.insert("IQD", CurrencyForms::new(&["dînar", "dînar"], &["fils", "fils"]));
    m.insert("USD", CurrencyForms::new(&["dolar", "dolar"], &["sent", "sent"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["sent", "sent"]));
    m
}

/// Kurdish (Kurmanji).
///
/// Stateless apart from the currency tables: `setup()` only assigns constant
/// tables, and no method stashes state for another to consume.
pub struct LangKu {
    /// Built once here rather than per call — `to_currency` only ever reads it.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// TRY's entry, pre-resolved so the unknown-code fallback (bug 7) is a
    /// plain borrow instead of a lookup that could itself miss.
    currency_fallback: CurrencyForms,
}

impl Default for LangKu {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKu {
    pub fn new() -> Self {
        let currency_forms = build_currency_forms();
        // Indexing is deliberate: FALLBACK_CURRENCY must name a key that
        // `build_currency_forms` actually inserts, and this runs once at
        // construction, so a bad edit to either fails loudly and immediately
        // rather than silently changing what unknown codes render as.
        let currency_fallback = currency_forms[FALLBACK_CURRENCY].clone();
        LangKu {
            currency_forms,
            currency_fallback,
        }
    }

    /// Python's `parts = str(abs(val)).split(".")` plus the two `int()` calls
    /// that consume it, returning `(left, right)`.
    ///
    /// KU's `to_currency` is *string*-driven: it stringifies the value and
    /// slices the text. The Rust boundary only carries the parsed number, so
    /// this reconstructs the digits `str()` would have printed from the
    /// `BigDecimal`'s `(digits, scale)` pair — which is exactly what a canonical
    /// decimal repr encodes: `str(0.099)` == "0.099" <-> digits=99, scale=3.
    ///
    /// `right` is Python's `int(parts[1][:2].ljust(2, "0"))`: the first two
    /// fractional digits, **truncated** (never rounded) and right-padded, hence
    /// bounded to 0..=99 (bug 10).
    fn currency_parts(&self, val: &CurrencyValue) -> Result<(BigInt, u64)> {
        let value = match val {
            // A true `int`: `str()` has no ".", so `parts` has length 1 and the
            // `len(parts) > 1` guard leaves `right` at 0. Cents never appear.
            CurrencyValue::Int(v) => return Ok((v.abs(), 0)),
            CurrencyValue::Decimal { value, .. } => value,
        };

        // Python abs()es before str()ing; for a decimal repr the sign is just a
        // prefix, so abs()ing the parsed value is the same reconstruction.
        let (digits, scale) = value.abs().as_bigint_and_exponent();

        if scale < 0 {
            // A negative scale can only come from a string with a positive net
            // exponent (bigdecimal computes scale = digits_after_point - exp and
            // never normalises trailing zeros away, so "100" is scale 0, not
            // -2). Python's str() of such a value keeps the exponent, and every
            // shape of that then feeds an "e" to int(): "1e+21" -> parts
            // ["1e+21"] -> int("1e+21"), "1.5E+3" -> parts ["1", "5E+3"] ->
            // int("5E"). Both raise ValueError, so this is the exception Python
            // raises, not an approximation of it. See `concerns` for the
            // small-magnitude ("1e-05") half, which is *not* recoverable here.
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: {} carries an exponent \
                 in Python's str() form",
                value
            )));
        }
        if scale == 0 {
            // No "." in str(): parts has length 1, so right stays 0.
            return Ok((digits, 0));
        }

        let scale = scale as usize;
        let s = digits.to_string();
        // str() always prints at least one integer digit ("0.5", never ".5"),
        // so pad the digit string out to scale+1 before splitting.
        let padded = if s.len() <= scale {
            format!("{}{}", "0".repeat(scale + 1 - s.len()), s)
        } else {
            s
        };
        let (part0, part1) = padded.split_at(padded.len() - scale);

        // `int(parts[0]) if parts[0] else 0` — part0 is never empty here (the
        // padding above guarantees a digit), so the falsy branch is dead.
        let left = BigInt::from_str(part0)
            .map_err(|e| N2WError::Value(e.to_string()))?;

        // `parts[1][:2].ljust(2, "0")`. Sliced by chars per PORTING.md, though
        // these are all ASCII digits.
        let mut frac: String = part1.chars().take(2).collect();
        while frac.len() < 2 {
            frac.push('0');
        }
        let right = frac
            .parse::<u64>()
            .map_err(|e| N2WError::Value(e.to_string()))?;
        Ok((left, right))
    }

    /// Python's `_int_to_word`. `number` must be non-negative (guaranteed by
    /// [`LangKu::cardinal`], the only in-scope caller).
    fn int_to_word(&self, number: &BigInt) -> String {
        // Bug 1: the Python chain of `if number < ...` branches ends in a bare
        // `return str(number)`. Anything at or above 10^9 renders as digits.
        // Checking the ceiling first also proves the remaining value fits a
        // u64, so the cast below is sound rather than hopeful.
        let billion = BigInt::from(1_000_000_000u64);
        if *number >= billion {
            return number.to_string();
        }

        // Proven bounded: 0 <= number < 10^9.
        let n = number
            .to_u64()
            .expect("0 <= number < 10^9 is representable as u64");
        self.int_to_word_small(n)
    }

    /// The `< 10^9` half of `_int_to_word`, on a value proven to fit a u64.
    fn int_to_word_small(&self, number: u64) -> String {
        if number == 0 {
            return ZERO.to_string();
        }
        if number < 10 {
            return ONES[number as usize].to_string();
        }
        if number < 20 {
            return TEENS[(number - 10) as usize].to_string();
        }
        if number < 100 {
            let (t, o) = (number / 10, number % 10);
            let mut s = TENS[t as usize].to_string();
            if o != 0 {
                s.push_str(AND);
                s.push_str(ONES[o as usize]);
            }
            return s;
        }
        if number < 1_000 {
            let (h, r) = (number / 100, number % 100);
            // Bug 2: the multiplier is suppressed for h == 1, so 100 == "sed".
            let mut s = String::new();
            if h > 1 {
                s.push_str(ONES[h as usize]);
                s.push(' ');
            }
            s.push_str(HUNDRED);
            if r != 0 {
                s.push_str(AND);
                s.push_str(&self.int_to_word_small(r));
            }
            return s;
        }
        if number < 1_000_000 {
            let (t, r) = (number / 1_000, number % 1_000);
            // No `if t > 1` guard here — hence "yek hezar" for 1000 (bug 2).
            let mut s = self.int_to_word_small(t);
            s.push(' ');
            s.push_str(THOUSAND);
            if r != 0 {
                s.push_str(AND);
                s.push_str(&self.int_to_word_small(r));
            }
            return s;
        }
        // number < 10^9, enforced by int_to_word before the cast.
        let (m, r) = (number / 1_000_000, number % 1_000_000);
        let mut s = self.int_to_word_small(m);
        s.push(' ');
        s.push_str(MILLION);
        if r != 0 {
            s.push_str(AND);
            s.push_str(&self.int_to_word_small(r));
        }
        s
    }

    /// Python's `to_cardinal` for integral input.
    ///
    /// Python inspects `str(number)` and recurses on the `n[1:]` substring when
    /// it sees a leading "-"; for integers that is exactly negate-and-recurse.
    /// The final `.strip()` is reproduced by `trim()` — it is a no-op for every
    /// integral input (the inner result never has surrounding whitespace), but
    /// it is what keeps `NEGWORD`'s trailing space from being observable.
    fn cardinal(&self, value: &BigInt) -> String {
        if value.is_negative() {
            let inner = self.cardinal(&value.abs());
            return format!("{}{}", NEGWORD, inner).trim().to_string();
        }
        self.int_to_word(value)
    }

    /// Python's `to_cardinal` for **float** input: the `"." in n` branch of
    /// `Num2Word_KU.to_cardinal`, which is *string*-driven and never touches
    /// `base.float2tuple`. It stringifies the float (`n = str(number)`), splits
    /// on the first ".", renders the integer part with `_int_to_word`, then
    /// appends the pointword and each **raw** fractional digit as
    /// `self.ones[int(digit)] or "sifir"`.
    ///
    /// Reading `str(number)`'s digits verbatim is exactly why KU cannot inherit
    /// base.py's float path. `base.float2tuple` recomputes the fraction as
    /// `abs(value - pre) * 10**precision` in f64 and loses the final digit for
    /// long reprs — `str(5.525882872479642)` keeps `...642`, but the f64 product
    /// rounds it to `...641`. KU sidesteps that entirely by never doing the
    /// arithmetic. Consequently `precision` is *unused for the digit values*
    /// (KU's `to_cardinal` takes no `precision=` kwarg); it is threaded in only
    /// to restore the ".0" of an integer-valued float, whose Rust `Display`
    /// (unlike Python's `str`) drops the trailing fractional part.
    fn cardinal_float(&self, value: f64, precision: u32) -> Result<String> {
        // Python tests `str(number).strip().startswith("-")` — the sign *bit*,
        // not `value < 0`. `str(-0.0) == "-0.0"` starts with "-", so KU prepends
        // negword even for negative zero; `is_sign_negative` reproduces that.
        let neg = value.is_sign_negative();
        let a = value.abs();

        // Python's `str(float)` switches to exponential form for a nonzero
        // magnitude below 1e-4 or at/above 1e16 (and prints "inf"/"nan" for a
        // non-finite value). KU then feeds that text to `int()`, which raises
        // ValueError: "1e+16" has no "." so `int("1e+16")` fails, and "1.5e+16"
        // splits a "5e+16" run whose `int("e")` fails. Either path is a
        // ValueError; reproduce the *variant* (the diff harness compares the
        // exception type, not its message).
        if !a.is_finite() || (a != 0.0 && (a < 1e-4 || a >= 1e16)) {
            return Err(N2WError::Value(format!(
                "invalid literal for int() with base 10: str({}) is not a plain decimal",
                value
            )));
        }

        // Rust's f64 `Display` is shortest-round-trip like Python's `repr`, so
        // the digits match across this range — save for a few genuine dtoa
        // *ties* above ~1e13 where "N.d2" and "N.d3" both round-trip to the same
        // double and the two libraries break the tie oppositely (see concerns).
        // Display never uses exponential form, so within [1e-4, 1e16) `s` is a
        // plain decimal string.
        let s = format!("{}", a);
        let (int_str, frac_str): (String, String) = match s.split_once('.') {
            Some((i, f)) => (i.to_string(), f.to_string()),
            // No ".": an integer-valued float, whose Python `str` is "N.0".
            // `precision` is 1 for such a value, recovering the single "0".
            None => (s, "0".repeat(precision as usize)),
        };

        // `_int_to_word(int(left))` — `left` is the non-negative integer part.
        let left = BigInt::from_str(&int_str).map_err(|e| N2WError::Value(e.to_string()))?;
        let mut ret = self.int_to_word(&left);

        if !frac_str.is_empty() {
            // `ret += " " + self.pointword`
            ret.push(' ');
            ret.push_str(self.pointword());
            // `for digit in right: ret += " " + (self.ones[int(digit)] or "sifir")`
            for ch in frac_str.chars() {
                let d = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!("non-digit {:?} in fractional part", ch))
                })?;
                ret.push(' ');
                // `ones[0]` is "" (falsy), so a 0 digit becomes "sifir".
                ret.push_str(if d == 0 { ZERO } else { ONES[d as usize] });
            }
        }

        // Python: negative -> `(negword + <inner>).strip()`, positive ->
        // `<inner>.strip()`. Both strips are no-ops beyond negword's own space.
        Ok(if neg {
            format!("{}{}", NEGWORD, ret).trim().to_string()
        } else {
            ret.trim().to_string()
        })
    }
}

/// The `ValueError` for a `Decimal` whose `str()` is scientific notation
/// (`"1E+2"`, `"1E-7"`, `"1.5E+3"`). KU feeds `str(number)` to `int()`: with
/// no "." the whole (sign-stripped) literal is rejected; with a "." the
/// per-character digit loop dies on `int("E")`.
fn decimal_sci_value_error(s: &str) -> N2WError {
    let n = s.strip_prefix('-').unwrap_or(s);
    let lit = if n.contains('.') { "E" } else { n };
    N2WError::Value(format!(
        "invalid literal for int() with base 10: '{}'",
        lit
    ))
}

impl Lang for LangKu {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "TRY"
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
        "xal"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        Ok(self.cardinal(value))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // Bug 6: this special case is indistinguishable from the general path.
        // Bug 4: no verify_ordinal call, so negatives fall through happily.
        if value == &BigInt::from(1u32) {
            return Ok("yekem".to_string());
        }
        Ok(format!("{}em", self.cardinal(value)))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // str(number) + "em" — the sign survives: -1 -> "-1em".
        Ok(format!("{}em", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // KU ignores `longval` and the base class's BC/AD handling entirely.
        Ok(self.cardinal(value))
    }

    /// `Num2Word_KU.to_cardinal` handles non-integers inline via `str(number)`,
    /// so KU does **not** inherit `Num2Word_Base.to_cardinal_float`. Float input
    /// takes KU's own string algorithm ([`LangKu::cardinal_float`]).
    ///
    /// The Decimal arm, by contrast, *is* reproduced bit-for-bit by the base
    /// path: `str(Decimal)`'s fractional digits equal
    /// `int(abs(value - int(value)) * 10**precision)` computed in exact
    /// arbitrary precision, which is what `default_to_cardinal_float`'s Decimal
    /// branch does — and its per-digit / pointword / sign rendering routes back
    /// through KU's own `to_cardinal`, `pointword` and `negword`. So delegate
    /// it rather than re-deriving the string. (The lone gap is a *negative-zero*
    /// Decimal: KU's `str(Decimal("-0.0"))` starts with "-" and prepends
    /// negword, but `num_bigint` has no negative zero, so the sign is already
    /// gone by the time a `BigDecimal` reaches here — unrecoverable at this
    /// layer, and absent from the corpus. See concerns.)
    ///
    /// `precision_override` (the `precision=` kwarg) is dropped for both arms:
    /// KU's `to_cardinal(number)` takes no such parameter and reads `str(number)`
    /// regardless, so an override never changes its output.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, precision } => self.cardinal_float(*value, *precision),
            FloatValue::Decimal { .. } => {
                crate::floatpath::default_to_cardinal_float(self, value, None)
            }
        }
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// KU routes on the string, not the value: `"." in str(number)` decides
    /// between the pointword grammar and `int(n)`. So the base default's
    /// whole→int shortcut is wrong here — `str(5.0)` is `"5.0"` and must read
    /// `"pênc xal sifir"`, while the point-free `Decimal("5")` stays `"pênc"`,
    /// and an exponent-form repr (`"1e+16"`, `"1E+2"`) raises the `int()`
    /// ValueError. The Float arm delegates wholesale to [`LangKu::cardinal_float`],
    /// which already reproduces the scientific-notation raise, the sign-bit
    /// negword of `-0.0`, and the restored ".0" of a whole float.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float {
                value: f,
                precision,
            } => self.cardinal_float(*f, *precision),
            FloatValue::Decimal { value: d, .. } => {
                let s = python_decimal_str(d);
                if s.contains('E') {
                    return Err(decimal_sci_value_error(&s));
                }
                if s.contains('.') {
                    // `"." in n`: the pointword grammar, whole values included
                    // (`Decimal("5.00")` -> "pênc xal sifir sifir").
                    return self.to_cardinal_float(value, precision_override);
                }
                // Point-free, non-scientific: an integer-valued Decimal —
                // Python's `return self._int_to_word(int(n))` arm, with the
                // negword recursion for a leading "-".
                let i = value
                    .as_whole_int()
                    .expect("a point-free, non-scientific Decimal is whole");
                Ok(self.cardinal(&i))
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python's `number == 1` is a *numeric*
    /// comparison, so `1.0` and `Decimal("1.00")` hit "yekem"; everything
    /// else — negative zero included — is the string-routed cardinal plus
    /// "em": `to_ordinal(5.0)` == `"pênc xal sifirem"`, `to_ordinal(-0.0)` ==
    /// `"negatîv sifir xal sifirem"`. An exponent-form repr propagates the
    /// cardinal's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int() == Some(BigInt::from(1u32)) {
            return Ok("yekem".to_string());
        }
        Ok(format!("{}em", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "em"` — the raw repr,
    /// never an error: `to_ordinal_num(1e16)` == `"1e+16em"` and
    /// `Decimal("5.00")` == `"5.00em"`.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}em", repr_str))
    }

    /// `to_year(float/Decimal)`: bare `self.to_cardinal(val)`, string routing
    /// included — `to_year(5.0)` == `"pênc xal sifir"`, `to_year(1e16)`
    /// raises ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`) and this
    /// override does not change what parses — `python_decimal_parse` still
    /// decides. It exists because `Decimal("Infinity")`/`Decimal("NaN")` *do*
    /// parse in Python, and KU's `to_cardinal` then dies at `int("Infinity")`
    /// with ValueError — where the bridge hard-wires the *base* integer-path
    /// errors (OverflowError / "cannot convert NaN"), which KU never
    /// produces. The one input this would misserve is
    /// `to_ordinal_num("Infinity")` (Python: `"Infinityem"`), which no corpus
    /// exercises.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // KU overrides `to_currency` and `pluralize`; `to_cheque`, `_money_verbose`,
    // `_cents_verbose` and `_cents_terse` are Base's, and the trait defaults
    // already mirror them. `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are
    // empty on the live class, so `currency_precision` (100) and
    // `currency_adjective` (None) are left at their defaults too.

    fn lang_name(&self) -> &str {
        "Num2Word_KU"
    }

    /// `CURRENCY_FORMS[code]`, subscripted — this is the *strict* lookup that
    /// Base's `to_cheque` uses, so a miss here is the NotImplementedError the
    /// corpus expects for GBP/JPY/KWD/BHD/INR/CNY/CHF.
    ///
    /// Deliberately **not** the fallback-to-TRY lookup: bug 7 lives inside
    /// `to_currency`'s own `.get(..., default)` call, and leaking it into this
    /// hook would silently make every cheque succeed in lira.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_KU.pluralize`: `"" if not forms else forms[0] if n == 1 else
    /// forms[-1]`.
    ///
    /// Unreachable in practice (bug 12), but the trait default raises
    /// NotImplementedError and KU's method plainly does not, so it is ported.
    /// Note `forms[-1]`, not `Num2Word_EUR`'s `forms[1]`.
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

    /// `Num2Word_KU.to_currency`. Replaces Base's wholesale: no
    /// `parse_currency_parts`, no `CURRENCY_PRECISION`, no `pluralize`, no
    /// adjective handling, and no NotImplementedError.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // `adjective` is in KU's signature and is never read (bug 9).
        _adjective: bool,
    ) -> Result<String> {
        // None means the caller omitted the kwarg, so KU's own default applies.
        let separator = separator.unwrap_or(self.default_separator());

        // `is_negative = val < 0` is read from the *original* value, before
        // abs(). `-0.0 < 0` is False in Python, and BigInt/BigDecimal have no
        // negative zero, so both sides agree that -0.0 is not negative.
        let is_negative = val.is_negative();
        let (left, right) = self.currency_parts(val)?;

        // Bug 7: `.get(currency, list(CURRENCY_FORMS.values())[0])`.
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);

        // `cr1[1] if left != 1 else cr1[0]` — Python subscripts the tuple
        // directly rather than calling pluralize(), so a form short of two
        // entries would raise IndexError. Every KU entry has two, making this
        // unreachable; mapped rather than panicked so the exception type would
        // survive a table change (as in lang_en.rs).
        let unit = if left.is_one() {
            forms.unit.first()
        } else {
            forms.unit.get(1)
        }
        .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

        let mut result = format!("{} {}", self.int_to_word(&left), unit);

        // `if cents and right:` — `right` falsy (0) skips the segment, which is
        // why 1.0 and 12.005 render bare. cents=False drops it outright rather
        // than falling back to _cents_terse (bug 8).
        if cents && right != 0 {
            let subunit = if right == 1 {
                forms.subunit.first()
            } else {
                forms.subunit.get(1)
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;
            // Python calls _int_to_word(right); right is bounded to 0..=99 by
            // the two-digit truncation, so the small path is the same function.
            result.push_str(separator);
            result.push_str(&format!("{} {}", self.int_to_word_small(right), subunit));
        }

        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }
        // Python's trailing `.strip()`.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    fn ku() -> LangKu {
        LangKu::new()
    }

    fn int(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, true, false, false).unwrap()
    }

    /// `has_decimal` is true for every float repr, matching the shim's
    /// `is_int = "." not in arg and "e" not in arg.lower()`.
    fn flt(s: &str) -> CurrencyValue {
        CurrencyValue::parse(s, false, true, true).unwrap()
    }

    fn cur(l: &LangKu, v: &CurrencyValue, code: &str) -> String {
        l.to_currency(v, code, true, None, false).unwrap()
    }

    /// The frozen corpus rows for `to": "currency:*`, verbatim.
    #[test]
    fn corpus_currency_rows() {
        let l = ku();
        // EUR — its own entry.
        assert_eq!(cur(&l, &int("0"), "EUR"), "sifir euro");
        assert_eq!(cur(&l, &int("1"), "EUR"), "yek euro");
        assert_eq!(cur(&l, &int("2"), "EUR"), "du euro");
        assert_eq!(cur(&l, &int("100"), "EUR"), "sed euro");
        assert_eq!(cur(&l, &flt("12.34"), "EUR"), "donzdeh euro sî û çar sent");
        assert_eq!(cur(&l, &flt("0.01"), "EUR"), "sifir euro yek sent");
        assert_eq!(cur(&l, &flt("1.0"), "EUR"), "yek euro");
        assert_eq!(cur(&l, &flt("99.99"), "EUR"), "not û neh euro not û neh sent");
        assert_eq!(
            cur(&l, &flt("1234.56"), "EUR"),
            "yek hezar û du sed û sî û çar euro pêncî û şeş sent"
        );
        assert_eq!(cur(&l, &flt("-12.34"), "EUR"), "negatîv donzdeh euro sî û çar sent");
        assert_eq!(cur(&l, &int("1000000"), "EUR"), "yek milyon euro");
        assert_eq!(cur(&l, &flt("0.5"), "EUR"), "sifir euro pêncî sent");

        // USD — its own entry.
        assert_eq!(cur(&l, &int("0"), "USD"), "sifir dolar");
        assert_eq!(cur(&l, &flt("12.34"), "USD"), "donzdeh dolar sî û çar sent");
        assert_eq!(cur(&l, &flt("-12.34"), "USD"), "negatîv donzdeh dolar sî û çar sent");
        assert_eq!(cur(&l, &int("1000000"), "USD"), "yek milyon dolar");

        // Bug 7: every code KU does not declare silently renders as TRY.
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            assert_eq!(cur(&l, &int("0"), code), "sifir lîre", "{code}");
            assert_eq!(cur(&l, &int("1"), code), "yek lîre", "{code}");
            assert_eq!(cur(&l, &int("2"), code), "du lîre", "{code}");
            assert_eq!(cur(&l, &int("100"), code), "sed lîre", "{code}");
            assert_eq!(cur(&l, &flt("12.34"), code), "donzdeh lîre sî û çar qurûş", "{code}");
            assert_eq!(cur(&l, &flt("0.01"), code), "sifir lîre yek qurûş", "{code}");
            assert_eq!(cur(&l, &flt("1.0"), code), "yek lîre", "{code}");
            assert_eq!(
                cur(&l, &flt("99.99"), code),
                "not û neh lîre not û neh qurûş",
                "{code}"
            );
            assert_eq!(
                cur(&l, &flt("1234.56"), code),
                "yek hezar û du sed û sî û çar lîre pêncî û şeş qurûş",
                "{code}"
            );
            assert_eq!(
                cur(&l, &flt("-12.34"), code),
                "negatîv donzdeh lîre sî û çar qurûş",
                "{code}"
            );
            assert_eq!(cur(&l, &int("1000000"), code), "yek milyon lîre", "{code}");
            assert_eq!(cur(&l, &flt("0.5"), code), "sifir lîre pêncî qurûş", "{code}");
        }
        // KWD/BHD get no 3-decimal treatment: CURRENCY_PRECISION is empty.
        assert_eq!(cur(&l, &flt("1.234"), "KWD"), "yek lîre bîst û sê qurûş");
    }

    /// The frozen corpus rows for `to": "cheque:*`, verbatim.
    #[test]
    fn corpus_cheque_rows() {
        let l = ku();
        let v = BigDecimal::from_str("1234.56").unwrap();
        assert_eq!(
            l.to_cheque(&v, "EUR").unwrap(),
            "YEK HEZAR Û DU SED Û SÎ Û ÇAR AND 56/100 EURO"
        );
        assert_eq!(
            l.to_cheque(&v, "USD").unwrap(),
            "YEK HEZAR Û DU SED Û SÎ Û ÇAR AND 56/100 DOLAR"
        );
        // Base's to_cheque subscripts the dict, so unknown codes raise here
        // even though to_currency quietly falls back to TRY (bug 7).
        for code in ["GBP", "JPY", "KWD", "BHD", "INR", "CNY", "CHF"] {
            match l.to_cheque(&v, code) {
                Err(N2WError::NotImplemented(m)) => assert_eq!(
                    m,
                    format!("Currency code \"{code}\" not implemented for \"Num2Word_KU\"")
                ),
                other => panic!("{code}: expected NotImplemented, got {other:?}"),
            }
        }
    }

    /// Bug 10: two-digit truncation with ljust padding, never rounding.
    #[test]
    fn cents_truncate_and_pad() {
        let l = ku();
        assert_eq!(cur(&l, &flt("12.345"), "USD"), "donzdeh dolar sî û çar sent");
        assert_eq!(cur(&l, &flt("12.305"), "USD"), "donzdeh dolar sî sent");
        // "005"[:2] -> "00" -> 0 -> falsy -> the whole segment vanishes.
        assert_eq!(cur(&l, &flt("12.005"), "USD"), "donzdeh dolar");
        assert_eq!(cur(&l, &flt("1.5"), "USD"), "yek dolar pêncî sent");
        assert_eq!(cur(&l, &flt("0.099"), "USD"), "sifir dolar neh sent");
        assert_eq!(cur(&l, &flt("2.999"), "USD"), "du dolar not û neh sent");
    }

    /// Bugs 8, 9 and 11, plus the -0.0 sign edge.
    #[test]
    fn currency_quirks() {
        let l = ku();
        // Bug 8: cents=False omits rather than terse-renders.
        assert_eq!(
            l.to_currency(&flt("12.34"), "USD", false, None, false).unwrap(),
            "donzdeh dolar"
        );
        // Bug 9: adjective is ignored.
        assert_eq!(
            l.to_currency(&flt("12.34"), "USD", true, None, true).unwrap(),
            "donzdeh dolar sî û çar sent"
        );
        // Bug 11: _int_to_word's digit fallback reaches money.
        assert_eq!(cur(&l, &int("1000000000"), "USD"), "1000000000 dolar");
        assert_eq!(
            cur(&l, &flt("1234567890.12"), "USD"),
            "1234567890 dolar donzdeh sent"
        );
        // -0.0 is not < 0 in Python; no negword.
        assert_eq!(cur(&l, &flt("-0.0"), "USD"), "sifir dolar");
        assert_eq!(cur(&l, &flt("-0.5"), "USD"), "negatîv sifir dolar pêncî sent");
        assert_eq!(cur(&l, &int("-1"), "USD"), "negatîv yek dolar");
        // Bug 13: the separator is concatenated raw, with no space after it.
        // Base writes "%s%s %s %s" (space *after* the separator); KU writes
        // `result += separator + ...`. The default " " hides the difference;
        // any other separator exposes it.
        assert_eq!(
            l.to_currency(&flt("12.34"), "USD", true, Some(","), false).unwrap(),
            "donzdeh dolar,sî û çar sent"
        );
    }

    /// `precision` as the harness computes it:
    /// `abs(Decimal(str(v)).as_tuple().exponent)`. For a value Rust's `Display`
    /// prints with no ".", Python's `str` still writes ".0", so precision is 1.
    fn prec(v: f64) -> u32 {
        match format!("{}", v.abs()).split_once('.') {
            Some((_, f)) => f.len() as u32,
            None => 1,
        }
    }

    fn cf(l: &LangKu, v: f64) -> String {
        l.to_cardinal_float(&FloatValue::Float { value: v, precision: prec(v) }, None)
            .unwrap()
    }

    /// The frozen corpus rows for `to": "cardinal` with a float `arg`, verbatim.
    #[test]
    fn corpus_cardinal_float_rows() {
        let l = ku();
        assert_eq!(cf(&l, 0.0), "sifir xal sifir");
        assert_eq!(cf(&l, 0.5), "sifir xal pênc");
        assert_eq!(cf(&l, 1.0), "yek xal sifir");
        assert_eq!(cf(&l, 1.5), "yek xal pênc");
        assert_eq!(cf(&l, 2.25), "du xal du pênc");
        assert_eq!(cf(&l, 3.14), "sê xal yek çar");
        assert_eq!(cf(&l, 0.01), "sifir xal sifir yek");
        assert_eq!(cf(&l, 0.1), "sifir xal yek");
        assert_eq!(cf(&l, 0.99), "sifir xal neh neh");
        assert_eq!(cf(&l, 1.01), "yek xal sifir yek");
        assert_eq!(cf(&l, 12.34), "donzdeh xal sê çar");
        assert_eq!(cf(&l, 99.99), "not û neh xal neh neh");
        assert_eq!(cf(&l, 100.5), "sed xal pênc");
        assert_eq!(cf(&l, 1234.56), "yek hezar û du sed û sî û çar xal pênc şeş");
        assert_eq!(cf(&l, -0.5), "negatîv sifir xal pênc");
        assert_eq!(cf(&l, -1.5), "negatîv yek xal pênc");
        assert_eq!(cf(&l, -12.34), "negatîv donzdeh xal sê çar");
        // The two f64-artefact cases: KU reads str() digits, so no rounding.
        assert_eq!(cf(&l, 1.005), "yek xal sifir sifir pênc");
        assert_eq!(cf(&l, 2.675), "du xal şeş heft pênc");
    }

    /// Live-interpreter edges not in the frozen corpus: -0.0 (sign bit),
    /// bug 1's digit fallback leaking into the integer part, and sci-notation
    /// floats that make `int()` raise ValueError.
    #[test]
    fn cardinal_float_edges() {
        let l = ku();
        // str(-0.0) == "-0.0" starts with "-": negword is prepended.
        assert_eq!(cf(&l, -0.0), "negatîv sifir xal sifir");
        // Bug 1: the >=1e9 integer part renders as bare digits.
        assert_eq!(cf(&l, 1234567890.12), "1234567890 xal yek du");
        // Python str(1e16)=="1e+16" -> int() ValueError; likewise 1e-5, inf/nan.
        for v in [1e16, 1.5e16, 1e-5, f64::INFINITY, f64::NAN] {
            assert!(
                matches!(
                    l.to_cardinal_float(&FloatValue::Float { value: v, precision: 1 }, None),
                    Err(N2WError::Value(_))
                ),
                "{v} should raise ValueError"
            );
        }
        // 1e-4 is the boundary: Python keeps it non-exponential ("0.0001"),
        // so all four fractional digits (0,0,0,1) render.
        assert_eq!(cf(&l, 1e-4), "sifir xal sifir sifir sifir yek");
    }

    /// The frozen corpus rows for `to": "cardinal_dec` (Decimal input), verbatim.
    /// These route through the base path, which is exact for Decimals.
    #[test]
    fn corpus_cardinal_dec_rows() {
        let l = ku();
        let dec = |s: &str| {
            let precision = s.split_once('.').map(|(_, f)| f.len() as u32).unwrap_or(0);
            FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision,
            }
        };
        let d = |s: &str| l.to_cardinal_float(&dec(s), None).unwrap();
        assert_eq!(d("0.01"), "sifir xal sifir yek");
        assert_eq!(d("1.10"), "yek xal yek sifir");
        assert_eq!(d("12.345"), "donzdeh xal sê çar pênc");
        // Issue #603: exact at trillion scale, no float() round.
        assert_eq!(d("98746251323029.99"), "98746251323029 xal neh neh");
        assert_eq!(d("0.001"), "sifir xal sifir sifir yek");
    }

    /// Bug 12: ported despite being unreachable. Note forms[-1], not forms[1].
    #[test]
    fn pluralize_takes_last_form() {
        let l = ku();
        let three: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        assert_eq!(l.pluralize(&BigInt::from(1), &three).unwrap(), "a");
        assert_eq!(l.pluralize(&BigInt::from(2), &three).unwrap(), "c");
        assert_eq!(l.pluralize(&BigInt::from(2), &[]).unwrap(), "");
    }

    /// str() of a value with a positive net exponent feeds an "e" to int().
    #[test]
    fn exponent_form_raises_value_error() {
        let l = ku();
        // "1e+21" parses to scale -21; Python: int("1e+21") -> ValueError.
        assert!(matches!(
            l.to_currency(&flt("1e+21"), "USD", true, None, false),
            Err(N2WError::Value(_))
        ));
        // A plain integral Decimal ("5", scale 0) must NOT trip that branch.
        assert_eq!(cur(&l, &flt("5"), "USD"), "pênc dolar");
    }
}
