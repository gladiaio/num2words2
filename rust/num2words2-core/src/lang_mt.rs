//! Port of `lang_MT.py` (Maltese).
//!
//! # Scale words above a million (gladiaio/num2words2#147)
//!
//! Upstream stopped at a million and returned `str(number)` from 10^9 up. The
//! port adds "biljun" (10^9) and "triljun" (10^12) (Wiktionary;
//! newsbook.com.mt), composed like the million arm, and raises `OverflowError`
//! from 10^15, which `maxval()` reports. The plural ("żewġ biljuni") is not
//! applied, as for miljun. Where the notes below describe the digit fallback
//! ("1000000000", "no overflow check"), they describe Python; that arm is now
//! unreachable.
//!
//! Shape: **self-contained**. `Num2Word_MT` subclasses `Num2Word_Base` but
//! defines only `setup()` — no `high_numwords`/`mid_numwords`/`low_numwords`.
//! `Num2Word_Base.__init__` guards its card-table build behind
//! `any(hasattr(self, f) for f in ["high_numwords", "mid_numwords",
//! "low_numwords"])`, so for MT `self.cards` is **never created** and
//! `self.MAXVAL` is **never set**. `to_cardinal` is overridden outright and
//! drives a private `_int_to_word` recursion.
//!
//! Consequently `cards`/`maxval`/`merge` stay at their trait defaults here and
//! there is **no overflow check whatsoever** — `to_cardinal(10**21)` returns a
//! string rather than raising `OverflowError` (see bug 4 below). This is
//! confirmed by the frozen corpus, where every large-value row is `ok: true`.
//!
//! Inherited from `Num2Word_Base` and left alone by MT: nothing in scope. MT
//! overrides all four in-scope entry points (`to_cardinal`, `to_ordinal`,
//! `to_ordinal_num`, `to_year`) itself.
//!
//! No cross-call mutable state: `setup()` assigns only constants, and none of
//! the four modes writes back to `self`. The stateless Rust path is safe.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Everything below is linguistically wrong —
//! this module emits Maltese that no Maltese speaker would produce — but it is
//! exactly what Python emits, and every item is confirmed against the frozen
//! corpus. Do not "fix" any of it.
//!
//! 1. **`tens[1]` is the cardinal "għaxra" ("ten"), not a teens stem.** The
//!    `10..=99` branch concatenates `tens[n/10] + " " + ones[n%10]`, so the
//!    teens come out as literal "ten one", "ten two", …: `to_cardinal(11)` ==
//!    "għaxra wieħed", `to_cardinal(19)` == "għaxra disgħa". Real Maltese has
//!    dedicated teens ("ħdax", "tnax", …); the table has no slot for them.
//! 2. **Round tens/hundreds/thousands are never elided.** The hundreds branch
//!    unconditionally prefixes the unit word, so `to_cardinal(100)` ==
//!    "wieħed mija" ("one hundred", literally) where Maltese says "mija", and
//!    `to_cardinal(200)` == "tnejn mija" where Maltese says "mitejn". Likewise
//!    `to_cardinal(1000)` == "wieħed elf" (Maltese: "elf") and
//!    `to_cardinal(1000000)` == "wieħed miljun".
//! 3. **The tens/ones join uses a space where Maltese needs the "u" conjunction**
//!    ("għoxrin u wieħed"): `to_cardinal(21)` == "għoxrin wieħed".
//! 4. **No scale word above "miljun", and the fallback leaks digits.** The
//!    `_int_to_word` chain stops at `number < 1000000000`; the `else` arm is
//!    literally `return str(number)`. So `to_cardinal(10**9)` ==
//!    "1000000000" and `to_cardinal(10**21)` == "1000000000000000000000" —
//!    the decimal string, not words, and **no** `OverflowError`. Modelled by
//!    the final arm of [`LangMt::int_to_word`].
//! 5. **`to_ordinal` ignores Maltese article assimilation and sanity alike.**
//!    Only 1..=10 have real ordinal forms; everything else is `"l-" +
//!    to_cardinal(number)` with no assimilation of the definite article to the
//!    following consonant. Hence `to_ordinal(30)` == "l-tletin" (Maltese:
//!    "it-tletin"), `to_ordinal(20)` == "l-għoxrin", and — because the branch
//!    is a bare `else` with no guard on sign or zero —
//!    `to_ordinal(0)` == "l-żero" and `to_ordinal(-1)` == "l-minus wieħed".
//!    Those last two are nonsense, are not errors, and are in the corpus.
//! 6. **`to_ordinal_num` is `str(number) + "."` with no sign guard**, so
//!    `to_ordinal_num(-1)` == "-1.".
//! 7. **`to_year` ignores its `longval` parameter and does no BC/AD handling** —
//!    it is a bare delegation to `to_cardinal`. So `to_year(-500)` ==
//!    "minus ħamsa mija" (no "BC"), and `to_year(1984)` reads as a plain
//!    cardinal rather than the usual year-pair phrasing.
//! 8. **`negword` carries a trailing space** ("minus ", which is Maltese).
//!    `to_cardinal` relies on a final `.strip()` to tidy the seam. Python's
//!    zero and `pointword` were the English "zero"/"point"; the port says
//!    "żero" and "punt" (fixed, gladiaio/num2words2#154).
//!
//! # Float / Decimal cardinal path
//!
//! MT overrides `to_cardinal` outright and handles non-integers **inline**, so
//! it never touches `Num2Word_Base.float2tuple`, `self.precision`, banker's
//! rounding, or the `< 0.01` f64-artefact rescue. It stringifies the value and
//! slices the decimal point out of the *string*: `str(number).split(".", 1)`,
//! then `int(left)` through `_int_to_word` and each fractional character through
//! `_int_to_word(int(digit))`. Ported in [`LangMt::to_cardinal_float`], which
//! reconstructs `str(number)` exactly — CPython's `repr(float)` for the float
//! arm ([`python_float_repr`]: shortest round-trip, ".0" kept on whole values,
//! "-0.0" signed, exponent form past 1e16) and the spec `str(Decimal)` for the
//! Decimal arm ([`python_decimal_str`]: scale preserved, so "5.00" keeps both
//! zeros; capital-E scientific form for a positive exponent). `2.675` stays
//! "675" and `1.005` stays "005" with **no** artefact repair — there is no
//! float2tuple arithmetic to produce one.
//!
//! **The string IS the routing.** Because Python's `to_cardinal` branches on
//! `"." in str(number)`, a whole float still speaks its ".0" tail and the
//! whole-value shortcut of the base `cardinal_float_entry` is wrong here. All
//! four float entry hooks are therefore overridden:
//! * `cardinal_float_entry` — everything through the string algorithm:
//!   `5.0` -> "ħamsa punt żero", `Decimal("5.00")` -> "ħamsa point zero
//!   zero", `-0.0` -> "minus zero point zero", `Decimal("12.")` (str "12", no
//!   dot) -> "għaxra tnejn". Exponent-form strings ("1e+16", "1E+2") have no
//!   dot either, so `int()` raises `ValueError` — corpus-pinned.
//! * `ordinal_float_entry` — the 1..=10 ladder is *numeric* (`5.0 == 5` ->
//!   "il-ħames"); everything else is `"l-" + to_cardinal(number)`, float
//!   spelling and ValueErrors included ("l-minus żero punt żero").
//! * `ordinal_num_float_entry` — `str(number) + "."`, no error even on
//!   exponent forms ("1e+16.").
//! * `year_float_entry` — bare `to_cardinal` delegation.
//!
//! Consequences, all pinned by the corpus:
//! * The integer-part bugs 1-4 reach floats too: `12.34` -> "għaxra tnejn
//!   point …" (broken teens), `100.5` -> "wieħed mija point …" (un-elided
//!   hundred), and the digit-leaking fallback surfaces in the huge Decimal row
//!   `98746251323029.99` -> "98746251323029 punt disgħa disgħa".
//! * The `precision=` kwarg is inert — MT's method never reads `self.precision`
//!   (`num2words(1.5, lang='mt', precision=5)` == the un-overridden result), so
//!   `precision_override` is accepted and ignored.
//! * Fractional digits are spelt one glyph at a time via the same `_int_to_word`
//!   ladder, so a leading-zero fraction reads "żero …": `0.01` -> "zero point
//!   zero wieħed".
//!
//! # Errors
//!
//! None, for the four integer modes. No integer input raises: there is no
//! overflow check, no dict lookup that can miss, and no list index that can go
//! out of range (every index is arithmetically bounded by its enclosing range
//! check). All four modes are total over the integers, so every method returns
//! `Ok`. Floats/Decimals whose `str()` is exponent-form raise `ValueError`
//! from `int()` on cardinal/ordinal/year (never on ordinal_num), and string
//! input that parses to `Decimal("Infinity")` surfaces the same `ValueError`
//! Python's later `int("Infinity")` would — see
//! [`LangMt::str_to_number`]. The currency surface *can* also raise — see
//! below.
//!
//! # Currency
//!
//! `Num2Word_MT` declares `CURRENCY_FORMS` with exactly two codes (EUR, USD)
//! and overrides `to_currency` **outright**, ignoring `Num2Word_Base`'s entire
//! currency machinery. It does not define `CURRENCY_ADJECTIVES` or
//! `CURRENCY_PRECISION`, so both stay at `Num2Word_Base`'s empty dicts and
//! every code keeps the default divisor of 100.
//!
//! The two entry points diverge sharply, and the corpus pins both:
//!
//! * **`to_currency` never raises on an unknown code.** It looks the code up
//!   with `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!   — a `dict.get` with a *default*, not a subscript. So GBP, JPY, KWD, BHD,
//!   INR, CNY and CHF all silently render as **euros** (bug 9). Reproduced via
//!   [`LangMt::currency_fallback`].
//! * **`to_cheque` does raise.** MT does not override it, and
//!   `Num2Word_Base.to_cheque` subscripts `self.CURRENCY_FORMS[currency]`
//!   inside a `try`, converting `KeyError` into `NotImplementedError`. Hence
//!   `to_cheque(1234.56, "GBP")` raises while `to_currency(1234.56, "GBP")`
//!   happily says "ewro". The trait default reproduces this as long as
//!   [`Lang::currency_forms`] reports `None` for unknown codes, which is why
//!   that hook stays strict while `to_currency` uses the lenient lookup.
//!
//! `to_cheque`, `money_verbose`, `cents_verbose`, `cents_terse`, `pluralize`
//! and `currency_precision` are therefore all left at their trait defaults:
//! `default_to_cheque` + `money_verbose` -> `to_cardinal` already reproduces
//! `Num2Word_Base.to_cheque` byte for byte for MT. `pluralize` is never
//! reached (MT's `to_currency` inlines its own singular/plural choice and
//! `to_cheque` takes `cr1[-1]` unconditionally), so its raising default is
//! correct.
//!
//! # Faithfully reproduced Python bugs, continued (currency)
//!
//! 9. **Unknown currency codes silently become euros**, as described above.
//!    `to_currency(1.0, "JPY")` == "wieħed ewro".
//! 10. **The subunit divisor is never consulted.** `to_currency` hard-codes a
//!    two-digit fractional slice (`parts[1][:2]`), so 3-decimal currencies
//!    (KWD/BHD) and 0-decimal ones (JPY) are all treated as 2-decimal — on top
//!    of already having been coerced to EUR forms by bug 9.
//! 11. **Cents are truncated, never rounded.** `parts[1][:2]` slices the
//!    decimal *string*: `12.345` -> 34 cents and `12.999` -> 99 cents, where
//!    `Num2Word_Base` would ROUND_HALF_UP to 35 and 100. Likewise `1.005` ->
//!    0 cents, so it renders as a bare "wieħed ewro".
//! 12. **`adjective` is accepted and ignored.** MT's signature takes it but
//!    never reads it, so `adjective=True` changes nothing.
//! 13. **`cents=True` still hides zero cents.** The guard is `if cents and
//!    right:` — a truthiness test on the cent *count* — so a float with zero
//!    cents drops the segment entirely: `to_currency(1.0)` == "wieħed ewro",
//!    not "wieħed ewro żero ċenteżmi". This is the opposite of
//!    `Num2Word_Base.to_currency`, which always shows a float's cents. Because
//!    MT reaches the same result for `1` and `1.0`, the int/float distinction
//!    that `CurrencyValue` preserves is **not** observable here — but it is
//!    still threaded through faithfully rather than collapsed.
//! 14. **The `default separator` is `" "`, not base's `","`.** MT's signature
//!    is `separator=" "`, so cents run on with a plain space and no comma.
//! 15. **`_int_to_word`'s digit-leaking fallback (bug 4) reaches money too.**
//!    `to_currency(1000000000000000.0)` == "1000000000000000 ewro".
//!
//! # Errors (currency)
//!
//! `to_currency` reproduces one Python crash: `int(parts[0])` raises
//! `ValueError` when `str(val)` came out in exponential notation, which
//! CPython's float repr does for `abs(v) >= 1e16` (`str(1e16)` == `'1e+16'`,
//! and `'1e+16'.split(".")` leaves the whole token for `int()`). Mapped to
//! [`N2WError::Value`] with CPython's exact message. See `concerns` in the
//! port report for the one input band where this cannot be reproduced.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). USD uses dollaru/dollari with ċenteżmu/ċenteżmi, like EUR.
//! Examples in these docs that quote English nouns record Python's output.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

/// `setup`: `self.negword = "minus "` — the trailing space is load-bearing in
/// Python's `ret + self._int_to_word(...)` seam, then removed by `.strip()`.
const NEGWORD: &str = "minus ";

/// `setup`: `self.pointword` — Python's English "point", here the Maltese
/// "punt" (gladiaio/num2words2#154).
/// Used by [`LangMt::to_cardinal_float`] as the integer/fraction separator,
/// verbatim (MT does *not* run it through `title()`, unlike `Num2Word_Base`).
const POINTWORD: &str = "punt";

/// `setup`: `self.ones`. Index 0 is the empty string — see [`ZERO`].
const ONES: [&str; 10] = [
    "", "wieħed", "tnejn", "tlieta", "erbgħa", "ħamsa", "sitta", "sebgħa", "tmienja", "disgħa",
];

/// `setup`: `self.tens`. Note index 1 is "għaxra", the *cardinal* ten — this is
/// what produces the broken teens described in bug 1.
const TENS: [&str; 10] = [
    "", "għaxra", "għoxrin", "tletin", "erbgħin", "ħamsin", "sittin", "sebgħin", "tmenin",
    "disgħin",
];

const HUNDRED: &str = "mija";
const THOUSAND: &str = "elf";
const MILLION: &str = "miljun";

/// Python: `return self.ones[0] if self.ones[0] else "zero"`. `ones[0]` is `""`,
/// which is falsy, so Python *always* yielded the English "zero"; the port
/// says the Maltese "żero" (gladiaio/num2words2#154).
const ZERO: &str = "żero";

/// `to_ordinal`'s hard-coded ladder for 1..=10, indexed by `n - 1`.
///
/// Several entries end in an apostrophe (`raba'`, `seba'`, `disa'`) — that is
/// the Maltese għajn elision, not a typo, and must survive verbatim.
const ORDINALS_1_10: [&str; 10] = [
    "l-ewwel",   // 1  first
    "it-tieni",  // 2  second
    "it-tielet", // 3  third
    "ir-raba'",  // 4  fourth
    "il-ħames",  // 5  fifth
    "is-sitt",   // 6  sixth
    "is-seba'",  // 7  seventh
    "it-tmien",  // 8  eighth
    "id-disa'",  // 9  ninth
    "l-għaxar",  // 10 tenth
];

/// Python's `int(s)`, including the failure mode.
///
/// CPython raises `ValueError: invalid literal for int() with base 10: '...'`
/// for a non-numeric token; MT reaches that with exponential-notation floats.
/// `BigInt::from_str` accepts the same `[+-]?digits` grammar `int()` does for
/// every string this module can produce (they all come from a decimal
/// rendering, so no whitespace or underscores are in play).
fn py_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s)
        .map_err(|_| N2WError::Value(format!("invalid literal for int() with base 10: '{}'", s)))
}

/// CPython's `repr(float)` / `str(float)`, which is what MT's `to_cardinal`
/// runs on: `n = str(number).strip()`. MT's float branch is a *string*
/// algorithm — it splits on ".", feeds `int()` the pieces and spells the
/// fraction digit by digit — so the exact repr, not the numeric value, drives
/// the output (and the ValueErrors). **MT never uses `base.float2tuple`**, so
/// the two `floatpath.rs` traps — banker's rounding of a binary `post`, and
/// the `< 0.01` artefact heuristic — do not arise here.
///
/// # 1. The digits
///
/// `{:e}` is Rust's shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the
/// significant digits and the decimal-point position fall straight out. A rare
/// tie can leave `{:e}`'s final digit one off the value CPython's dtoa would
/// pick; re-formatting with `{:.*}` at the known digit count repairs it. This
/// exact function is differentially tested against CPython on 300k doubles in
/// the sibling ports (`lang_rw`, `lang_bm`, `lang_ki`): 0 mismatches with the
/// repair.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`
/// (`format_float_short`, format code `'r'`), pads the exponent to two digits,
/// and appends `.0` to anything that would otherwise look like an integer.
/// Rust's `{}` does none of this, so both `1e16` and `1.0` would come out
/// wrong in opposite directions. Both matter to MT: `str(1.0)` is `"1.0"` →
/// "wieħed punt żero", and `str(1e16)` is `"1e+16"` → `int("1e+16")` raises
/// `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this: for an exponent-form repr it is the *exponent*
/// (`abs(Decimal(str(v)).as_tuple().exponent)`), not a digit count — `1e16`
/// arrives with `precision == 16`.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). MT feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and MT renders that
    // "minus żero punt żero".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `decpt` is CPython's: the value is `0.<digits> * 10**decpt`.
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').expect("LowerExp always emits an 'e'");
    let exp: i32 = exp.parse().expect("LowerExp emits an integer exponent");
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut decpt = exp + 1;

    // Tie repair — see the doc comment. Only reachable when the shortest form
    // has fractional digits; `a == 0.0` is excluded because `{:e}` reports it
    // as "0e0" and there is nothing to round.
    let frac_digits = digits.chars().count() as i32 - decpt;
    if frac_digits > 0 && a != 0.0 {
        let t = format!("{:.*}", frac_digits as usize, a);
        let (ip, fp) = t.split_once('.').expect("frac_digits > 0 forces a point");
        let all = format!("{}{}", ip, fp);
        let trimmed = all.trim_start_matches('0');
        if !trimmed.is_empty() {
            let lead = all.chars().count() - trimmed.chars().count();
            digits = trimmed.to_string();
            decpt = ip.chars().count() as i32 - lead as i32;
        }
    }

    let n = digits.chars().count() as i32;

    if decpt <= -4 || decpt > 16 {
        // CPython: mantissa, then "e", then "%+.02d" of decpt-1.
        let e = decpt - 1;
        let mut out = String::from(sign);
        let mut it = digits.chars();
        out.push(it.next().expect("a finite double has at least one digit"));
        if n > 1 {
            out.push('.');
            out.push_str(it.as_str());
        }
        out.push('e');
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&format!("{:02}", (e as i64).abs()));
        out
    } else if decpt <= 0 {
        format!("{}0.{}{}", sign, "0".repeat((-decpt) as usize), digits)
    } else if decpt >= n {
        // Py_DTSF_ADD_DOT_0: an integral value still reprs with a ".0".
        format!("{}{}{}.0", sign, digits, "0".repeat((decpt - n) as usize))
    } else {
        let k = decpt as usize;
        format!(
            "{}{}.{}",
            sign,
            digits.chars().take(k).collect::<String>(),
            digits.chars().skip(k).collect::<String>()
        )
    }
}

/// `str(number)` for whatever the Python dispatcher handed the converter. The
/// `FloatValue` split is exactly Python's `isinstance(value, Decimal)`: the
/// two arms stringify by different rules and must not be collapsed.
/// `str(Decimal)` is the spec algorithm in [`crate::strnum::python_decimal_str`]
/// — capital `E`, trailing zeros preserved (`Decimal("5.00")` → "5.00"),
/// exponent form for a positive exponent (`Decimal("1E+2")` → "1E+2", which
/// MT's `int()` then rejects with ValueError).
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Scale words above a million (gladiaio/num2words2#147), as
/// `(exponent, word)`. See the module docs for the sources.
const SCALES: [(u32, &str); 2] = [(9, "biljun"), (12, "triljun")];

/// The exclusive ceiling: 1000 of the largest scale word, 10^15.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

pub struct LangMt {
    /// `Num2Word_MT.CURRENCY_FORMS`. Built once in [`LangMt::new`] and stored;
    /// the py binding holds the `LangMt` in a `OnceLock`, so this table is
    /// constructed exactly once per process rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `list(self.CURRENCY_FORMS.values())[0]` — the value MT's `to_currency`
    /// falls back to for an unknown code (bug 9).
    ///
    /// Python dicts preserve insertion order, and `CURRENCY_FORMS` is a literal
    /// whose first key is `"EUR"`, so this is always the euro entry. A
    /// `HashMap` has no first element, so the fallback is materialised here
    /// rather than recovered from the table.
    currency_fallback: CurrencyForms,
}

impl Default for LangMt {
    fn default() -> Self {
        Self::new()
    }
}

impl LangMt {
    /// `int_to_word` behind [`maxval_ceiling`] (gladiaio/num2words2#147): every
    /// entry point that hands over a caller-supplied integer goes through here.
    fn checked_int_to_word(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_word(number))
    }

    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "EUR": (("ewro", "ewro"), ("ċenteżmu", "ċenteżmi")),
        //     "USD": (("dollar", "dollars"), ("cent", "cents")),
        // }
        // Note EUR's unit forms are ("ewro", "ewro") — singular and plural are
        // the same word, so `left == 1` is unobservable for EUR but not USD
        // ("wieħed dollar" vs "tnejn dollars"). Both entries carry exactly two
        // forms; the indexing in `to_currency` relies on that arity.
        let eur = CurrencyForms::new(&["ewro", "ewro"], &["ċenteżmu", "ċenteżmi"]);
        let mut currency_forms = HashMap::new();
        currency_forms.insert("EUR", eur.clone());
        // Python's USD entry was English ("dollars", "cents"); the Maltese
        // nouns follow EUR's 1 / other split (#222).
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dollaru", "dollari"], &["ċenteżmu", "ċenteżmi"]),
        );
        LangMt {
            currency_forms,
            // "EUR" is the first key in the literal above.
            currency_fallback: eur,
        }
    }

    /// Port of `Num2Word_MT._int_to_word`.
    ///
    /// Branch order mirrors the Python exactly: zero, negative, then the
    /// ascending magnitude ladder, then the `str(number)` fallback.
    fn int_to_word(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return ZERO.to_string();
        }

        if number.is_negative() {
            // Python: `return self.negword + self._int_to_word(abs(number))`.
            // Unreachable from the four in-scope modes — `to_cardinal` strips
            // the sign from the *string* before parsing, so this function only
            // ever sees non-negative values. Ported for fidelity.
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        // Every index below is bounded by its enclosing range check, so the
        // narrowing conversions are total and cannot panic.
        if *number < BigInt::from(10) {
            let n = number.to_usize().expect("0 < n < 10");
            return ONES[n].to_string();
        }

        if *number < BigInt::from(100) {
            let n = number.to_u32().expect("10 <= n < 100");
            let tens_val = (n / 10) as usize;
            let ones_val = (n % 10) as usize;
            if ones_val == 0 {
                return TENS[tens_val].to_string();
            }
            return format!("{} {}", TENS[tens_val], ONES[ones_val]);
        }

        if *number < BigInt::from(1000) {
            let n = number.to_u32().expect("100 <= n < 1000");
            let hundreds_val = (n / 100) as usize;
            let remainder = n % 100;
            // Python: `self.ones[hundreds_val] + " " + self.hundred` — the unit
            // word is never elided (bug 2).
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            // Python's `if remainder:` — a truthiness test, so 0 skips.
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word(&BigInt::from(remainder)));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000) {
            let n = number.to_u64().expect("1000 <= n < 10^6");
            let thousands_val = n / 1000;
            let remainder = n % 1000;
            let mut result = format!(
                "{} {}",
                self.int_to_word(&BigInt::from(thousands_val)),
                THOUSAND
            );
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word(&BigInt::from(remainder)));
            }
            return result;
        }

        if *number < BigInt::from(1_000_000_000) {
            let n = number.to_u64().expect("10^6 <= n < 10^9");
            let millions_val = n / 1_000_000;
            let remainder = n % 1_000_000;
            let mut result = format!(
                "{} {}",
                self.int_to_word(&BigInt::from(millions_val)),
                MILLION
            );
            if remainder != 0 {
                result.push(' ');
                result.push_str(&self.int_to_word(&BigInt::from(remainder)));
            }
            return result;
        }

        // Scale words above a million (gladiaio/num2words2#147), composed like
        // the million arm. Every entry point rejects values at or above
        // `maxval_ceiling()` first, so the top quotient is always below 1000.
        for &(exp, word) in SCALES.iter().rev() {
            let scale = pow10_big(exp);
            if number >= &scale {
                let mut result = format!("{} {}", self.int_to_word(&(number / &scale)), word);
                let rest = number % &scale;
                if !rest.is_zero() {
                    result.push(' ');
                    result.push_str(&self.int_to_word(&rest));
                }
                return result;
            }
        }

        // Python: `return str(number)  # Fallback for very large numbers`.
        // No words, no error — the raw decimal string (bug 4). Only reachable
        // at top level: every recursive call above passes a value < 10^9.
        number.to_string()
    }
}

impl Lang for LangMt {
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
        " "
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "punt"
    }

    /// Port of `Num2Word_MT.to_cardinal`.
    ///
    /// Python stringifies the input, strips a leading "-" off the *string*,
    /// then re-parses with `int()`. For integer input that round trip is
    /// exactly `abs(value)` (a `BigInt`'s decimal form has no whitespace, no
    /// "+", and no leading zeros), so we take the absolute value directly.
    ///
    /// The `"." in n` branch is the float path and is out of scope.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let ret = if value.is_negative() { NEGWORD } else { "" };
        let magnitude = value.abs();
        // Python: `return (ret + self._int_to_word(int(n))).strip()`.
        Ok(format!("{}{}", ret, self.checked_int_to_word(&magnitude)?)
            .trim()
            .to_string())
    }

    /// Port of `Num2Word_MT.to_ordinal`.
    ///
    /// The 1..=10 ladder is a chain of `==` tests; everything else — including
    /// 0 and every negative — falls through to the bare `else` (bug 5).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(n) = value.to_usize() {
            if (1..=10).contains(&n) {
                return Ok(ORDINALS_1_10[n - 1].to_string());
            }
        }
        // Python: `cardinal = self.to_cardinal(number); return "l-" + cardinal`.
        Ok(format!("l-{}", self.to_cardinal(value)?))
    }

    /// Port of `Num2Word_MT.to_ordinal_num`: `str(number) + "."`, sign and all.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_MT.to_year`: `return self.to_cardinal(val)`.
    ///
    /// The `longval` parameter is accepted and ignored in Python; there is no
    /// BC/AD handling and no year-pair phrasing (bug 7).
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// Port of `Num2Word_MT.to_cardinal`'s float/Decimal branch.
    ///
    /// MT does **not** use `Num2Word_Base.to_cardinal_float`/`float2tuple`. Its
    /// overridden `to_cardinal` stringifies the value and slices on the first
    /// dot, converting the integer part and each fractional character through
    /// the same `_int_to_word` ladder the integer modes use:
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
    /// Reconstructing `str(number)`: see [`python_str`]. The float arm is the
    /// full CPython repr ([`python_float_repr`]) — shortest round-trip digits,
    /// `.0` appended to whole values, `-0.0`'s sign kept, and **exponent form
    /// past 1e16** — and the Decimal arm is the spec `str(Decimal)` algorithm
    /// ([`python_decimal_str`]), scale preserved so `Decimal("5.00")` keeps
    /// both zeros and `Decimal("1E+2")` keeps its `E`. Because no float2tuple
    /// arithmetic runs, the digits are literal — `2.675` -> "675", `1.005` ->
    /// "005" — and the `< 0.01` artefact rescue is simply not part of this
    /// path.
    ///
    /// Exponent-form strings (`str(1e16)` == "1e+16", `str(Decimal("1E+2"))`
    /// == "1E+2") carry no ".", so they fall through to the else branch and
    /// `int()` raises `ValueError: invalid literal for int() with base 10:
    /// '1e+16'` — corpus-pinned for 1e+16/1e+20/1E+2/1E+20 on all of
    /// cardinal/ordinal/year.
    ///
    /// `precision_override` (the `precision=` kwarg) is accepted and **ignored**:
    /// MT's method never consults `self.precision`, so the kwarg is inert in
    /// Python (verified: `num2words(1.5, lang='mt', precision=5)` is unchanged).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        // MT's Python method has no `precision` parameter and never reads
        // `self.precision`; the override is inert. Threaded through, not honoured.
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // Python: `n = str(number).strip()`. The value crossed the binding as
        // the exact f64 / Decimal Python saw, so rebuild its `str()` here.
        let signed = python_str(value);

        // Python: `if n.startswith("-"): n = n[1:]; ret = self.negword`.
        // `format!` keeps the sign of -0.0 and `BigDecimal` keeps a Decimal's,
        // so this string test mirrors `str(number).startswith("-")` exactly.
        let mut ret = String::new();
        let mag: &str = if let Some(rest) = signed.strip_prefix('-') {
            ret.push_str(NEGWORD);
            rest
        } else {
            signed.as_str()
        };

        // Python: `if "." in n:` — `split(".", 1)` splits on the FIRST dot only.
        // '.' is ASCII, so the byte offsets from `find` land on char boundaries.
        if let Some(dot) = mag.find('.') {
            let left = &mag[..dot];
            let right = &mag[dot + 1..];
            // Python: `ret += self._int_to_word(int(left)) + " " + self.pointword + " "`.
            // `int(left)` on a huge integer part re-enters the digit-leaking
            // fallback (bug 4); `_int_to_word` already reproduces that.
            ret.push_str(&self.checked_int_to_word(&py_int(left)?)?);
            ret.push(' ');
            ret.push_str(POINTWORD);
            ret.push(' ');
            // Python: `for digit in right: ret += self._int_to_word(int(digit)) + " "`.
            // Each `digit` is a single character; `int(digit)` on a non-digit
            // raises ValueError — reproduced through `py_int` per char, though it
            // is unreachable in the positional regime this path targets.
            for ch in right.chars() {
                let d = py_int(&ch.to_string())?;
                ret.push_str(&self.checked_int_to_word(&d)?);
                ret.push(' ');
            }
            // Python: `return ret.strip()`.
            Ok(ret.trim().to_string())
        } else {
            // No dot in the reconstructed string. Python:
            // `return (ret + self._int_to_word(int(n))).strip()`.
            ret.push_str(&self.checked_int_to_word(&py_int(mag)?)?);
            Ok(ret.trim().to_string())
        }
    }

    // ---- float/Decimal routing -----------------------------------------

    /// `to_cardinal(float/Decimal)` — the **full** routing, whole values
    /// included. MT's `to_cardinal` reads `str(number)`, so a whole-valued
    /// float keeps its ".0" tail (`5.0` -> "ħamsa punt żero", `-0.0` ->
    /// "minus żero punt żero") and an exponent-form repr raises ValueError;
    /// the base default's whole -> integer-path route would get both wrong. A
    /// Decimal without a visible point (`Decimal("12.")` -> "12") lands in the
    /// same string algorithm's else branch, which *is* the integer path — the
    /// routing is the string, exactly as in Python.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`: Python's ladder is a chain of numeric
    /// `==` tests, so `5.0 == 5` and `Decimal("5.00") == 5` are both True and
    /// return "il-ħames" (corpus: ordinal 5.0 / 5.00 -> "il-ħames"). Anything
    /// else — 0.0, negatives, non-integral values, 11.0 and up — falls into
    /// the bare else: `"l-" + self.to_cardinal(number)`, where the cardinal
    /// spells the float ("l-għaxra wieħed punt żero") or raises the
    /// exponent-form ValueError before the prefix is attached.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        // `as_whole_int` is None for fractional values and NaN/±inf, all of
        // which compare unequal to every ladder constant in Python too.
        if let Some(i) = value.as_whole_int() {
            if let Some(n) = i.to_usize() {
                if (1..=10).contains(&n) {
                    return Ok(ORDINALS_1_10[n - 1].to_string());
                }
            }
        }
        Ok(format!("l-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, floats included —
    /// no error even for exponent forms (`to_ordinal_num(1e16)` == "1e+16."
    /// while `to_ordinal(1e16)` raises). `repr_str` is Python's own
    /// `str(number)`, supplied by the binding.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: `to_year` is a bare `to_cardinal` delegation,
    /// so floats route through the same string algorithm (and raise the same
    /// ValueErrors).
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    // ---- string inputs ---------------------------------------------------

    /// `converter.str_to_number` — base's `Decimal(value)`, with one
    /// deliberate deviation: `Decimal("Infinity")` parses fine in Python and
    /// the ValueError only fires later, inside MT's `int("Infinity")` (the
    /// leading "-" of "-Infinity" is sliced off before `int()` sees it, so
    /// both signs quote the same literal). The Rust dispatcher hard-codes
    /// `ParsedNumber::Inf` -> OverflowError before any language hook runs, so
    /// the ValueError is surfaced here at parse time instead — same observable
    /// type for every mode that reaches `to_cardinal`. Known divergence
    /// (`to_ordinal_num("Infinity")` would return "Infinity." in Python) is
    /// out of the corpus. NaN is left alone: the dispatcher already reports
    /// ValueError for it, matching Python's `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            other => Ok(other),
        }
    }

    // ---- currency ------------------------------------------------------

    /// For `Num2Word_Base.to_cheque`'s `'%s' not implemented for '%s'` message.
    fn lang_name(&self) -> &str {
        "Num2Word_MT"
    }

    /// `CURRENCY_FORMS[code]` — the **strict** lookup.
    ///
    /// Deliberately returns `None` for unknown codes so the inherited
    /// `to_cheque` raises `NotImplementedError`, matching Python's
    /// `self.CURRENCY_FORMS[currency]` subscript. MT's own `to_currency` does
    /// *not* route through here; it uses the lenient `.get(..., default)`
    /// lookup instead (bug 9).
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_MT.to_currency`.
    ///
    /// MT ignores `Num2Word_Base.to_currency` entirely: no `parse_currency_parts`,
    /// no `pluralize`, no `CURRENCY_PRECISION`, no ROUND_HALF_UP quantize, no
    /// `adjective`. It stringifies the value and slices the decimal point out
    /// by hand.
    ///
    /// ```python
    /// parts = str(val).split(".")
    /// left = int(parts[0]) if parts[0] else 0
    /// right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    /// ```
    ///
    /// The `str(val)` is taken *after* `val = abs(val)`, so the sign never
    /// reaches the split. `CurrencyValue::Decimal` already holds the exact
    /// decimal Python's `str(value)` produced (that string is what crossed the
    /// binding), and `BigDecimal`'s `Display` reproduces it verbatim —
    /// preserving scale, so `1.0` stays `"1.0"` and yields `parts[1] == "0"`
    /// exactly as CPython's float repr does.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // MT's signature takes `adjective` and never reads it (bug 12).
        _adjective: bool,
    ) -> Result<String> {
        // Trait hands None when the caller omitted separator=;
        // resolve to this language's own default.
        let separator = separator.unwrap_or(self.default_separator());
        // Python: `if val < 0: is_negative = True; val = abs(val)`.
        let is_negative = val.is_negative();

        // Python: `parts = str(val).split(".")`, on the absolute value.
        //
        // The Int/Decimal split is preserved rather than collapsed: an int
        // stringifies without a ".", so `len(parts) == 1` and the cents branch
        // is skipped structurally. A float like `1.0` keeps its ".0" and takes
        // the `right = 0` path instead. Both land on "wieħed ewro" here, but by
        // different routes.
        let s = match val {
            CurrencyValue::Int(v) => v.abs().to_string(),
            CurrencyValue::Decimal { value: v, .. } => v.abs().to_string(),
        };
        let mut parts = s.splitn(2, '.');
        // `splitn` always yields at least one item, so `parts[0]` always exists.
        let p0 = parts.next().unwrap_or("");
        let p1 = parts.next();

        // Python: `int(parts[0]) if parts[0] else 0` — the emptiness test runs
        // first, so an empty integer part is 0 rather than a ValueError.
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            py_int(p0)?
        };

        // Python: `int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`.
        //
        // `[:2]` truncates — it does not round (bug 11) — and `ljust` pads on
        // the *right*, so "5" becomes "50" (five-tenths -> 50 cents) while
        // "01" stays 1. Sliced by `chars()`, never bytes.
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut two: String = frac.chars().take(2).collect();
                while two.chars().count() < 2 {
                    two.push('0');
                }
                py_int(&two)?
            }
            _ => BigInt::zero(),
        };

        // Python: `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
        // — an unknown code yields the euro entry, not an error (bug 9).
        let forms = self
            .currency_forms
            .get(currency)
            .unwrap_or(&self.currency_fallback);
        let cr1 = &forms.unit;
        let cr2 = &forms.subunit;

        // Python: `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`.
        // Both table entries have arity 2, so index 1 is always populated.
        let left_str = self.checked_int_to_word(&left)?;
        let mut result = format!(
            "{} {}",
            left_str,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // Python: `if cents and right:` — `right` is truthiness-tested, so zero
        // cents drop the whole segment even when `cents=True` (bug 13).
        if cents && !right.is_zero() {
            let cents_str = self.checked_int_to_word(&right)?;
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // Python: `result = self.negword + result` — negword is "minus " and
        // its trailing space is what separates it from the amount.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // Python: `return result.strip()`.
        Ok(result.trim().to_string())
    }
}
