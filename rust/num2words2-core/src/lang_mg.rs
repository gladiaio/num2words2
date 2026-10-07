//! Port of `lang_MG.py` (Malagasy).
//!
//! Shape: **self-contained**. `Num2Word_MG` subclasses `Num2Word_Base` but its
//! `setup()` defines no `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` never builds `self.cards` and never sets
//! `self.MAXVAL`. `to_cardinal` is overridden outright and drives the private
//! `_int_to_word` recursion. `cards`/`maxval`/`merge` therefore stay at their
//! trait defaults here, and **there is no overflow check** — see the "no
//! ceiling" note below for what happens instead at 10^9.
//!
//! Every method in scope is overridden by MG, so nothing is inherited from
//! `Num2Word_Base` except the (unreached) machinery:
//!   * `to_cardinal`    — overridden (below)
//!   * `to_ordinal`     — overridden ("voalohany" / "faha-" + cardinal)
//!   * `to_ordinal_num` — overridden (`str(number) + "."`)
//!   * `to_year`        — overridden (`self.to_cardinal(val)`; `longval` ignored)
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following look wrong but are
//! exactly what Python emits, and are confirmed against the frozen corpus:
//!
//! 1. **No ceiling, no `OverflowError` — a digit-string fallback instead.**
//!    `_int_to_word`'s final `else` is `return str(number)`, so any value
//!    `>= 1_000_000_000` comes back as bare ASCII digits rather than words:
//!    `to_cardinal(10**9)` == `"1000000000"`, and `to_cardinal(10**21)` ==
//!    `"1000000000000000000000"`. This is the reason the value must stay a
//!    `BigInt` all the way to the fallback: the arbitrary-precision decimal
//!    rendering *is* the output. See [`int_to_word`].
//!
//! 2. **`to_ordinal` prefixes the negative word, producing "faha-minus …".**
//!    `to_ordinal` only special-cases `number == 1`; every other value is
//!    `"faha-" + self.to_cardinal(number)`, and `to_cardinal` renders the sign
//!    itself. So `to_ordinal(-1)` == `"faha-minus iray"`. Unlike most modules,
//!    MG never raises `errmsg_negord` — there is no negative guard at all.
//!
//! 3. **`to_ordinal(0)` == `"faha-aotra"`**, since `_int_to_word(0)` returns
//!    the zero word and 0 is not 1. (Contrast `lang_PL`, which crashes on 0.)
//!
//! 4. **`to_ordinal` inherits the digit fallback**, giving the hybrid
//!    `to_ordinal(10**9)` == `"faha-1000000000"` — a Malagasy prefix glued to
//!    an arabic numeral.
//!
//! 5. **No teens.** `self.tens` has no 11..19 forms, so `_int_to_word` composes
//!    them as ten-plus-unit: 11 == `"folo iray"`, 15 == `"folo dimy"`.
//!
//! 6. **Hundreds always carry an explicit "iray"** because `self.ones[1]` is
//!    prepended unconditionally: 100 == `"iray zato"` (not bare `"zato"`), and
//!    likewise 1000 == `"iray arivo"`, 10^6 == `"iray tapitrisa"`.
//!
//! 7. **Zero and decimal (fixed, gladiaio/num2words2#154).** Python writes
//!    `return self.ones[0] if self.ones[0] else "zero"` with `ones[0] == ""`,
//!    so it always said the English "zero", and `pointword` was "point". The
//!    port says "aotra" and "faingo" (the comma — Madagascar writes a decimal
//!    comma; mg.wiktionary "faingo"): 1.5 == "iray faingo dimy".
//!
//! # Unreachable Python code, deliberately mirrored
//!
//! `_int_to_word` has a `number < 0` branch (`negword + _int_to_word(abs(n))`),
//! but it is **unreachable from the four modes in scope**: `to_cardinal`
//! detaches the `"-"` from the *string* before calling `int()`, so the value
//! handed to `_int_to_word` is always `>= 0`, and every recursive call passes a
//! quotient or remainder that is also `>= 0`. Only the out-of-scope
//! `to_currency` can reach it. It is kept in [`int_to_word`] for fidelity.
//!
//! # The float/Decimal path
//!
//! `self.pointword` (`"faingo"`) and the `"." in n` branch of `to_cardinal` fire
//! only for float/Decimal input. MG overrides `to_cardinal` (not
//! `to_cardinal_float`) and handles non-integers **inline on the string**
//! `str(number)`, so it never calls `base.float2tuple` and never reaches
//! `Num2Word_Base.to_cardinal_float`. The two `PORTING_FLOAT.md` traps therefore
//! do not apply: there is no `round_ties_even` and no `< 0.01` f64-artefact
//! heuristic — `str(2.675)` is literally `"2.675"` and each fractional
//! *character* becomes its own digit word. This is ported in
//! [`Lang::to_cardinal_float`] below. It is byte-identical in structure to
//! `lang_HAW`, whose `to_cardinal` is the same method with different numerals.
//!
//! Because the routing itself is `"." in str(number)`, whole values are NOT
//! collapsed onto the integer path (the base default): `to_cardinal(5.0)` is
//! `"dimy faingo aotra"`, `to_cardinal(-0.0)` is `"minus aotra faingo aotra"`
//! (str(-0.0) keeps the sign), and only a point-free string form — an
//! integer-valued `Decimal` like `Decimal("5")` — takes the bare integer
//! grammar. `cardinal_float_entry` below carries that routing, and it also
//! carries the failure mode hiding in `int(n)`: a value whose `str()` is in
//! exponent form (`1e+16`, `Decimal("1E+2")`) or non-numeric
//! (`Decimal("Infinity")`, `Decimal("NaN")`) raises
//! `ValueError: invalid literal for int() with base 10: '...'`. A *dotted*
//! exponent repr (`1.5e+16`) instead enters the `"."` branch and dies in the
//! digit loop at `int('e')` — same type, different message. `to_ordinal`,
//! `to_ordinal_num` and `to_year` inherit all of this via their own float
//! entries (`number == 1` is numeric, so `to_ordinal(1.0)` == "voalohany").
//!
//! The separate currency hook `cardinal_from_decimal` is left at its trait
//! default: MG's `to_currency` computes fractional cents itself with exact
//! `BigDecimal` arithmetic and `to_cheque` renders only the whole part through
//! `money_verbose`, so nothing on the currency/cheque surface reaches it.
//!
//! # The currency surface
//!
//! `Num2Word_MG` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base.to_currency`. Consequences, all corpus-checked:
//!
//! * **No `NotImplementedError` for an unknown code.** Python does
//!   `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`
//!   — an unknown code silently falls back to the *first* dict value. The class
//!   literal lists `MGA` first, and dicts keep insertion order (3.7+), so GBP /
//!   JPY / KWD / BHD / INR / CNY / CHF all render as **ariary/iraimbilanja**.
//!   `to_cheque`, inherited from the base, does *not* share this and raises
//!   `NotImplementedError` for those same codes.
//! * **`CURRENCY_PRECISION` is never consulted.** MG hardcodes `[:2]`, so the
//!   3-decimal (KWD/BHD) and 0-decimal (JPY) special cases in
//!   `Num2Word_Base.to_currency` never run: `to_currency(12.34, "JPY")` shows
//!   cents, and `to_currency(12.34, "KWD")` splits at 2 places, not 3.
//! * **`pluralize` is never called** — MG picks `forms[0]`/`forms[1]` inline
//!   with `if left != 1`. The abstract `pluralize` therefore stays at its
//!   raising default, exactly as `Num2Word_Base.pluralize` does.
//! * **`adjective` is accepted and ignored.** MG has no `CURRENCY_ADJECTIVES`
//!   and never reads the flag.
//! * **`cents=False` drops the cents segment entirely** rather than rendering
//!   it as digits: the guard is `if cents and right`, with no `_cents_terse`
//!   branch. `to_currency(12.34, "EUR", cents=False)` == `"folo roa euros"`.
//! * **A float with zero cents renders no cents segment** — `1.0` gives
//!   `"iray euro"`, not `"... zero cents"` — because `right` is `0` (falsy).
//!   MG reaches the same place as the base's int path by a different route:
//!   it never calls `isinstance(val, int)` at all, it just looks for a `"."`
//!   in `str(val)`. See [`Lang::to_currency`] for why the two `CurrencyValue`
//!   arms provably converge here.
//! * **The digit fallback leaks into money.** `left` goes through
//!   `_int_to_word`, so `to_currency(1000000000.5, "EUR")` ==
//!   `"1000000000 euros dimampolo cents"`.
//!
//! # Error variants
//!
//! For integer input MG cannot raise: there is no overflow check, no
//! negative-ordinal guard, no dict lookup that can miss, and every list index
//! is arithmetically bounded to 0..=9. All four integer modes return `Ok`.
//! `to_currency` likewise cannot raise (the `.get` fallback removes the only
//! lookup). Only the inherited `to_cheque` raises — `NotImplemented`, on a
//! `CURRENCY_FORMS` KeyError.
//!
//! Float/Decimal/string input *can* raise, and only ever `ValueError`, from
//! the `int()` calls inside `to_cardinal`: exponent-form string shapes
//! (`1e+16`, `"1E+2"`, `"1e3"`) and the special Decimals
//! (`"Infinity"`/`"NaN"`) all die there. See `cardinal_float_entry` and
//! `str_to_number` below.
//!
//! # Currency nouns (gladiaio/num2words2#222)
//!
//! Python's currency table used English nouns here ("dollars", "cents",
//! "euros"). No reliable Malagasy cent noun was found, so USD and EUR raise
//! NotImplementedError. Examples in these docs that quote English nouns record
//! Python's output.

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::ParsedNumber;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.negword`. Note the **trailing space** — MG uses it raw rather than
/// going through `Num2Word_Base.parse_minus`, which would `.strip()` it.
const NEGWORD: &str = "minus ";

/// `self.ones`. Index 0 is `""`; it is read only as the hundreds/thousands
/// multiplier (always 1..=9) and in the dead `ones[0]` test on zero.
const ONES: [&str; 10] = [
    "", "iray", "roa", "telo", "efatra", "dimy", "enina", "fito", "valo", "sivy",
];

/// `self.tens`. Index 0 is `""` and is never reached: the `< 100` branch only
/// runs for `number >= 10`, so `number // 10` is always 1..=9.
const TENS: [&str; 10] = [
    "",
    "folo",
    "roapolo",
    "telopolo",
    "efapolo",
    "dimampolo",
    "enimpolo",
    "fitopolo",
    "valopolo",
    "sivifolo",
];

const HUNDRED: &str = "zato";
const THOUSAND: &str = "arivo";
const MILLION: &str = "tapitrisa";

/// The value at which `_int_to_word` gives up and returns `str(number)`.
const BILLION: u32 = 1_000_000_000;

/// The key whose value is `list(CURRENCY_FORMS.values())[0]`.
///
/// Python's fallback for an unknown code is positional, not by name:
/// `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
/// `CURRENCY_FORMS` is a dict *literal* on the class and nothing mutates it
/// (unlike the EUR/EN table — MG defines its own and inherits nothing), so its
/// insertion order is fixed by the source: MGA, USD, EUR. Hence `values()[0]`
/// is permanently the MGA entry. Verified against the live interpreter:
/// `list(c.CURRENCY_FORMS.values())[0]` -> `(('ariary', 'ariary'),
/// ('iraimbilanja', 'iraimbilanja'))`.
const FALLBACK_CODE: &str = "MGA";

pub struct LangMg {
    /// `CURRENCY_FORMS`, built once here rather than per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangMg {
    pub fn new() -> Self {
        // Python:
        //   CURRENCY_FORMS = {
        //       "MGA": (("ariary", "ariary"), ("iraimbilanja", "iraimbilanja")),
        //       "USD": (("dollar", "dollars"), ("cent", "cents")),
        //       "EUR": (("euro", "euros"), ("cent", "cents")),
        //   }
        // Every entry is arity 2 on both sides; `to_currency` indexes [0]/[1]
        // unconditionally, so that arity is an invariant of this table.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "MGA",
            CurrencyForms::new(&["ariary", "ariary"], &["iraimbilanja", "iraimbilanja"]),
        );
        // USD/EUR were English ("dollars", "cents", "euros"); no reliable Malagasy cent noun was found, so
        // they raise NotImplementedError (#222).
        LangMg { currency_forms }
    }

    /// `self.CURRENCY_FORMS.get(currency, list(self.CURRENCY_FORMS.values())[0])`.
    ///
    /// Note this is *not* the same lookup `currency_forms()` exposes to the
    /// trait: that one must return `None` for an unknown code so the inherited
    /// `to_cheque` still raises `NotImplementedError`. Only MG's own
    /// `to_currency` takes the fallback.
    fn forms_or_fallback(&self, currency: &str) -> &CurrencyForms {
        self.currency_forms.get(currency).unwrap_or_else(|| {
            self.currency_forms
                .get(FALLBACK_CODE)
                .expect("MGA is inserted by new() and never removed")
        })
    }
}

impl Default for LangMg {
    fn default() -> Self {
        Self::new()
    }
}

/// Python's `_int_to_word`, verbatim including the `str(number)` fallback.
fn int_to_word(number: &BigInt) -> String {
    // `if number == 0: return self.ones[0] if self.ones[0] else "zero"`.
    // ones[0] is "" (falsy), so this is unconditionally "zero".
    if number.is_zero() {
        return "aotra".to_string();
    }

    // Unreachable from to_cardinal/to_ordinal/to_year (the sign is stripped as
    // a string before int() is called, and no recursive call passes a negative)
    // — mirrored anyway, since Python has it.
    if number.is_negative() {
        return format!("{}{}", NEGWORD, int_to_word(&number.abs()));
    }

    // `else: return str(number)` — the fallback for very large numbers. This
    // must render the full BigInt; the value is NOT bounded here.
    if number >= &BigInt::from(BILLION) {
        return number.to_string();
    }

    // Below 10^9 the value provably fits a u32 (999_999_999 < u32::MAX), so the
    // rest of the recursion can use machine arithmetic without changing
    // behaviour. Python's `//` and `%` agree with Rust's `/` and `%` here
    // because both operands are non-negative.
    int_to_word_small(number.to_u32().expect("checked < 10^9 and >= 0 above"))
}

/// The `1 ..= 999_999_999` arm of `_int_to_word`.
fn int_to_word_small(number: u32) -> String {
    if number < 10 {
        // `return self.ones[number]`
        ONES[number as usize].to_string()
    } else if number < 100 {
        let tens_val = (number / 10) as usize;
        let ones_val = (number % 10) as usize;
        if ones_val == 0 {
            TENS[tens_val].to_string()
        } else {
            format!("{} {}", TENS[tens_val], ONES[ones_val])
        }
    } else if number < 1_000 {
        // `result = self.ones[hundreds_val] + " " + self.hundred` — the "iray"
        // in "iray zato" comes from here.
        let hundreds_val = (number / 100) as usize;
        let remainder = number % 100;
        let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else if number < 1_000_000 {
        let thousands_val = number / 1_000;
        let remainder = number % 1_000;
        let mut result = format!("{} {}", int_to_word_small(thousands_val), THOUSAND);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    } else {
        // number < 1_000_000_000, guaranteed by the caller.
        let millions_val = number / 1_000_000;
        let remainder = number % 1_000_000;
        let mut result = format!("{} {}", int_to_word_small(millions_val), MILLION);
        if remainder != 0 {
            result.push(' ');
            result.push_str(&int_to_word_small(remainder));
        }
        result
    }
}

/// Python `repr()` of a **non-negative** f64 — the string MG's `to_cardinal`
/// sees after detaching the sign. Rust's `{}` produces the same
/// shortest-round-trip digits but never switches to exponent form and drops
/// the ".0" of whole values, so the two Python-isms are reapplied here:
///
/// * exponent form for `|v| >= 1e16` or `0 < |v| < 1e-4`, with the sign
///   always shown and the exponent zero-padded to two digits (`"1e+16"`,
///   `"1.5e-05"`) — exactly repr's thresholds;
/// * a trailing `".0"` for whole values in positional form (`"5.0"`).
///
/// inf/nan come back as `"inf"`/`"nan"`, which the caller's `int()` port then
/// rejects with the same ValueError Python would raise.
fn python_float_repr_abs(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return "inf".to_string();
    }
    if f != 0.0 && (f >= 1e16 || f < 1e-4) {
        let s = format!("{:e}", f); // "1e16" / "1.2345e-7"
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("f64 exponent is a small integer");
        return format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        );
    }
    let s = format!("{}", f);
    if s.contains('.') {
        s
    } else {
        format!("{}.0", s)
    }
}

impl Lang for LangMg {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "MGA"
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
        "faingo"
    }

    /// Python:
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
    /// else:
    ///     ret = ""
    /// ...
    /// return (ret + self._int_to_word(int(n))).strip()
    /// ```
    /// The sign is detached from the *string*, so `_int_to_word` only ever sees
    /// a magnitude. The `"." in n` branch cannot fire for integer input.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let n = value.to_string();
        let (ret, magnitude_str) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", n.as_str()),
        };
        // `int(n)` on a sign-stripped decimal rendering of a BigInt — always
        // valid, so this cannot raise the ValueError that bites other modules.
        let magnitude: BigInt = magnitude_str
            .parse()
            .expect("decimal rendering of a BigInt minus its sign is always parseable");

        // Python's trailing `.strip()`: a no-op in practice (negword's space is
        // consumed by the word that follows, and _int_to_word never returns a
        // padded string), but reproduced for fidelity.
        Ok(format!("{}{}", ret, int_to_word(&magnitude))
            .trim()
            .to_string())
    }

    /// Python:
    /// ```python
    /// if number == 1:
    ///     return "voalohany"
    /// else:
    ///     return "faha-" + self.to_cardinal(number)
    /// ```
    /// No negative guard and no float guard — hence "faha-minus iray" for -1
    /// and "faha-zero" for 0.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value == &BigInt::from(1u32) {
            return Ok("voalohany".to_string());
        }
        Ok(format!("faha-{}", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "."` — no words at all, and the sign
    /// survives: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`
    /// — `longval` is accepted and ignored, and negative years get no "BC"
    /// treatment, just the negword: `to_year(-500)` == "minus dimy zato".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// MG routes on the string, not the value: `"." in str(number)` decides
    /// between the digit-word grammar and `int(n)`. Three corpus-pinned
    /// behaviours hang off the exact shape of `str()`:
    ///
    /// * **Whole floats keep their ".0"** — `str(5.0)` is `"5.0"`, so
    ///   `to_cardinal(5.0)` == `"dimy faingo aotra"`, never the bare integer
    ///   word the base default would pick. Only a point-free string — an
    ///   integer-valued `Decimal` like `Decimal("5")` — takes the int path.
    /// * **Exponent-form reprs raise ValueError.** `str(1e16)` is `"1e+16"`
    ///   (no "."), so Python falls to `int("1e+16")` and raises
    ///   `invalid literal for int() with base 10: '1e+16'`. Same for
    ///   `Decimal("1E+2")`. A *dotted* e-form (`1.5e+16` -> `"1.5e+16"`)
    ///   enters the `"."` branch instead and dies in the digit loop at
    ///   `int('e')` — same type, message `'e'` (`'E'` for Decimals).
    /// * **-0.0 keeps its sign** — `str(-0.0)` is `"-0.0"`, sign detached as
    ///   a string, hence `"minus aotra faingo aotra"`.
    ///
    /// The bridge converts a signed-zero `Decimal("-0.0")` to
    /// `Float { -0.0 }` (BigDecimal cannot carry the sign), so the Float arm
    /// may stand in for a Decimal zero; for zero both arms produce the same
    /// words, with `precision` (not this repr) driving the digit count.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        // n = str(number).strip() minus its sign — Python detaches a leading
        // "-" before any other test; the sign is re-applied from the value
        // below (int path) or inside to_cardinal_float (dotted path).
        let n = match value {
            FloatValue::Float { value: f, .. } => python_float_repr_abs(f.abs()),
            FloatValue::Decimal { value: d, .. } => {
                let s = crate::strnum::python_decimal_str(d);
                s.strip_prefix('-').map(str::to_string).unwrap_or(s)
            }
        };

        match n.split_once('.') {
            // `"." in n`: the digit-word grammar — unless a non-digit
            // character kills one of the int() calls first.
            Some((left, right)) => {
                // `int(left)` runs before the digit loop. Unreachable for
                // every str() shape in practice (a dotted repr's left part is
                // always plain digits), mirrored anyway for order fidelity.
                if !left.bytes().all(|b| b.is_ascii_digit()) || left.is_empty() {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        left
                    )));
                }
                // `for digit in right: int(digit)` — first non-digit raises;
                // this is where "1.5e+16" / Decimal("1.5E+20") die.
                if let Some(bad) = right.chars().find(|c| !c.is_ascii_digit()) {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        bad
                    )));
                }
                self.to_cardinal_float(value, precision_override)
            }
            // No "." — `int(n)`: plain digits take the integer grammar; an
            // exponent/Infinity/NaN string raises exactly like Python's int().
            None => {
                if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
                    return Err(N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        n
                    )));
                }
                let magnitude: BigInt = n.parse().expect("all-ASCII-digit string parses");
                let ret = if value.is_negative() { NEGWORD } else { "" };
                Ok(format!("{}{}", ret, int_to_word(&magnitude))
                    .trim()
                    .to_string())
            }
        }
    }

    /// `to_ordinal(float/Decimal)`. Python's `number == 1` is a *numeric*
    /// comparison, so `1.0` and `Decimal("1.00")` hit the "voalohany"
    /// special case; everything else — negative zero included — is "faha-"
    /// glued to the float cardinal: `to_ordinal(0.0)` == "faha-zero point
    /// zero", `to_ordinal(-0.0)` == "faha-minus zero point zero". An
    /// exponent-form repr propagates the cardinal's ValueError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if value.as_whole_int() == Some(BigInt::from(1u32)) {
            return Ok("voalohany".to_string());
        }
        Ok(format!("faha-{}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`, identical to the
    /// int path. `repr_str` IS Python's `str(number)`, so
    /// `to_ordinal_num(1e16)` == "1e+16." and `Decimal("5.00")` == "5.00.".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `to_year(float/Decimal)`: the same bare `self.to_cardinal(val)`
    /// delegation as the int path, string routing included — so
    /// `to_year(5.0)` == "dimy faingo aotra" and `to_year(1e16)` raises
    /// ValueError.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `str_to_number` is inherited from the base (`Decimal(value)`), and
    /// this override does not change what parses — `python_decimal_parse`
    /// still decides. It exists because `Decimal("Infinity")` and
    /// `Decimal("NaN")` *do* parse in Python, and MG's `to_cardinal` then
    /// dies at `int("Infinity")` with
    /// `ValueError: invalid literal for int() with base 10: 'Infinity'`
    /// (str(Decimal) capitalises to 'Infinity' whatever the input case, and
    /// the sign of "-Infinity" is stripped before int() sees it). The bridge
    /// hard-wires the *base* integer-path errors for Inf/NaN parses
    /// (OverflowError / "cannot convert NaN"), which MG never produces, so
    /// the raise is surfaced here instead. Observably identical for every
    /// corpus row — Infinity/NaN strings only appear with to="cardinal";
    /// the one input this would misserve is `to_ordinal_num("Infinity")`
    /// (Python: "Infinity."), which no corpus exercises.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            ParsedNumber::NaN => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'NaN'".into(),
            )),
            other => Ok(other),
        }
    }

    /// Port of the float/Decimal branch of `Num2Word_MG.to_cardinal`.
    ///
    /// MG overrides `to_cardinal` and handles non-integers **inline** on the
    /// string `str(number)` — it never calls `base.float2tuple` and never
    /// reaches `Num2Word_Base.to_cardinal_float`. So the two `PORTING_FLOAT.md`
    /// traps do **not** apply: no `round_ties_even`, no `< 0.01` f64-artefact
    /// heuristic. `str(2.675)` is literally `"2.675"`, and each fractional
    /// **character** becomes its own digit word.
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]; ret = self.negword          # "minus "
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
    /// The `precision=` kwarg is inert for MG: `to_cardinal` takes no such
    /// parameter and reads the fractional length straight off `repr`, so the
    /// live interpreter returns the same string with or without it. Hence
    /// `precision_override` is ignored and `value.precision()` (the repr-derived
    /// length, `abs(Decimal(str(v)).as_tuple().exponent)`) drives the split.
    ///
    /// # Reconstructing `str(number)`
    ///
    /// * **Float** — `format!("{:.p}", value.abs())` with `p = precision` yields
    ///   exactly the repr's fractional digits: `precision` was derived from that
    ///   same shortest round-trip repr, so rounding the raw f64 to `p` places
    ///   reproduces it (`2.675` -> `"2.675"`, `1.005` -> `"1.005"`). Note plain
    ///   `{}` would render `1.0` as `"1"` and lose the `".0"`, so the explicit
    ///   precision is load-bearing: `1.0` must give `"iray faingo aotra"`.
    /// * **Decimal** — the mantissa at scale `precision` gives the exact digits
    ///   with trailing zeros preserved (`Decimal("1.10")` -> `"1"`, `"10"`),
    ///   matching `str(Decimal)`. Corpus row `1.10` -> `"iray faingo iray aotra"`.
    ///
    /// # Quirks reproduced
    ///
    /// * The 10^9 digit fallback fires on the **integer part** too:
    ///   `Decimal("98746251323029.99")` -> `"98746251323029 faingo sivy sivy"` —
    ///   the integer part is bare digits. Corpus-pinned.
    /// * `zero` is emitted for every `0` digit, so `0.01` -> `"zero point zero
    ///   iray"` and `1.005` -> `"iray point zero zero dimy"`.
    /// * A negative fraction keeps the negword and prints `int_to_word(0)`:
    ///   `-0.5` -> `"minus aotra faingo dimy"`. There is no `pre == 0` sign rescue
    ///   like the base path — the `"-"` is stripped lexically from the string.
    ///
    /// # Errors
    ///
    /// `N2WError::Value` mirrors Python's `int()` `ValueError` on a non-digit
    /// fractional character — unreachable in practice, because
    /// [`Lang::cardinal_float_entry`] above intercepts every
    /// scientific-notation string form (where the non-digit characters live)
    /// and raises the corresponding ValueError before this grammar runs.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // MG's split length is the repr-derived precision; the `precision=`
        // kwarg never reaches this string-based path in Python.
        let precision = value.precision();
        let is_negative = value.is_negative();

        // (int_left, frac_digits) reconstructed from `str(number)`.
        let (int_left, frac): (BigInt, String) = match value {
            FloatValue::Float { value: f, .. } => {
                // `{:.p}` always emits exactly `p` fractional digits and never
                // scientific notation, so split_once('.') is safe for p > 0.
                let s = format!("{:.*}", precision as usize, f.abs());
                match s.split_once('.') {
                    Some((i, fr)) => (
                        BigInt::from_str(i).unwrap_or_else(|_| BigInt::zero()),
                        fr.to_string(),
                    ),
                    None => (
                        BigInt::from_str(&s).unwrap_or_else(|_| BigInt::zero()),
                        String::new(),
                    ),
                }
            }
            FloatValue::Decimal { value: d, .. } => {
                // str(Decimal) shows the exact digits; the mantissa at scale
                // `precision` is that digit string with trailing zeros kept.
                let (mant, _scale) =
                    d.abs().with_scale(precision as i64).as_bigint_and_exponent();
                let digits = mant.to_string(); // non-negative, no leading zeros
                let p = precision as usize;
                if digits.len() > p {
                    let (i, fr) = digits.split_at(digits.len() - p);
                    (
                        BigInt::from_str(i).unwrap_or_else(|_| BigInt::zero()),
                        fr.to_string(),
                    )
                } else {
                    // Integer part is 0; left-pad the fraction to `p` digits,
                    // e.g. Decimal("0.01") -> mant "1" -> frac "01".
                    let mut fr = "0".repeat(p - digits.len());
                    fr.push_str(&digits);
                    (BigInt::zero(), fr)
                }
            }
        };

        // Build `ret` exactly as Python concatenates, then `.strip()`.
        let mut ret = String::new();
        if is_negative {
            ret.push_str(NEGWORD); // "minus " — trailing space is load-bearing.
        }
        ret.push_str(&int_to_word(&int_left));

        // Python emits the point + digit words only when `"." in n`, i.e. when
        // there is a fractional part (precision > 0). precision == 0 reproduces
        // the integer `else` branch (bare `int_to_word`, no "point").
        if precision > 0 {
            ret.push(' ');
            ret.push_str(self.pointword()); // "faingo"
            for ch in frac.chars() {
                ret.push(' ');
                let digit = ch.to_digit(10).ok_or_else(|| {
                    N2WError::Value(format!(
                        "invalid literal for int() with base 10: '{}'",
                        ch
                    ))
                })?;
                ret.push_str(&int_to_word(&BigInt::from(digit)));
            }
        }

        Ok(ret.trim().to_string())
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_MG"
    }

    /// The strict `CURRENCY_FORMS[code]` lookup, i.e. the one that can raise.
    ///
    /// Only the inherited `to_cheque` reaches this; MG's own `to_currency`
    /// goes through [`LangMg::forms_or_fallback`] instead. Keeping this hook
    /// strict is what makes `to_cheque("GBP")` raise `NotImplementedError`
    /// while `to_currency("GBP")` quietly answers in ariary.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // `currency_precision` is deliberately NOT overridden: MG defines no
    // CURRENCY_PRECISION, so it inherits `Num2Word_Base.CURRENCY_PRECISION`
    // ({}) and every code resolves to the default 100. The trait default is
    // already 100. (`to_currency` never reads it at all; only the inherited
    // `to_cheque` does, which is why cheque always renders "NN/100".)
    //
    // `currency_adjective` is not overridden either: MG defines no
    // CURRENCY_ADJECTIVES, and the trait default returns None.
    //
    // `pluralize` stays at its raising default, matching the abstract
    // `Num2Word_Base.pluralize`. Neither MG's `to_currency` nor the base's
    // `to_cheque` calls it, so it is unreachable rather than wrong.
    //
    // `money_verbose` / `cents_verbose` / `cents_terse` stay at their defaults
    // (`self.to_cardinal(number)` / zero-padded digits), matching the base's
    // `_money_verbose` / `_cents_verbose` / `_cents_terse`. Only
    // `_money_verbose` is reachable, via `to_cheque`.

    /// Python:
    /// ```python
    /// def to_currency(self, val, currency="MGA", cents=True,
    ///                 separator=" ", adjective=False):
    ///     is_negative = False
    ///     if val < 0:
    ///         is_negative = True
    ///         val = abs(val)
    ///
    ///     parts = str(val).split(".")
    ///     left = int(parts[0]) if parts[0] else 0
    ///     right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0
    ///
    ///     cr1, cr2 = self.CURRENCY_FORMS.get(
    ///         currency, list(self.CURRENCY_FORMS.values())[0]
    ///     )
    ///
    ///     left_str = self._int_to_word(left)
    ///     result = left_str + " " + (cr1[1] if left != 1 else cr1[0])
    ///
    ///     if cents and right:
    ///         cents_str = self._int_to_word(right)
    ///         result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])
    ///
    ///     if is_negative:
    ///         result = self.negword + result
    ///
    ///     return result.strip()
    /// ```
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
        // `Num2Word_MG.to_currency` declares `separator=" "` in its own
        // signature, but both callers — `__init__.py`'s Rust fast path and
        // `bench/diff_test.py` — hand us `kwargs.get("separator", ",")`, i.e.
        // `Num2Word_Base`'s default rather than MG's. The trait signature
        // cannot distinguish "caller said ','" from "caller said nothing", so
        // the base default is read as "unspecified" and MG's own default is
        // applied. Every cents-bearing corpus row depends on this
        // ("folo roa euros telopolo efatra cents", not "euros,telopolo"). An
        // explicit `separator=","` is the single input this gets wrong, and the
        // bridge cannot express that case anyway. See `concerns`.
        let separator = if separator == "," { " " } else { separator };

        // MG never reads CURRENCY_ADJECTIVES and never forwards the flag.
        let _ = adjective;

        // `if val < 0: is_negative = True; val = abs(val)`.
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")`, then `int(parts[0])` and
        // `int(parts[1][:2].ljust(2, "0"))`.
        //
        // MG never calls `isinstance(val, int)`: the int/float split is implicit
        // in whether `str(val)` contains a ".". The two arms below therefore
        // converge rather than being collapsed —
        //   * `Int`: `str(5)` == "5", so `len(parts) > 1` is False and Python
        //     hard-codes `right = 0`.
        //   * `Decimal`: `str(5.0)` == "5.0", so `parts[1]` == "0" and Python
        //     computes `int("0".ljust(2, "0"))` == 0.
        // Both land on `right == 0`, which the `if cents and right` guard below
        // treats identically (falsy). So unlike `base.to_currency`, MG shows no
        // cents for `1.0` either, and the distinction is unobservable *here*.
        // It is preserved in the match anyway so the reasoning stays checkable.
        //
        // For a non-empty `parts[1]`, `int(parts[1][:2].ljust(2, "0"))` is
        // exactly `floor(frac * 100)`: taking the first two fractional digits
        // truncates ( "345" -> "34" == floor(0.345 * 100) ), and right-padding a
        // single digit scales it ( "5" -> "50" == 0.5 * 100 ). BigDecimal is an
        // exact decimal, so the arithmetic reproduces the string slicing.
        let (left, right) = match val {
            CurrencyValue::Int(v) => (v.abs(), BigInt::zero()),
            CurrencyValue::Decimal { value: d, .. } => {
                let d = d.abs();
                // `int(parts[0])` — truncation, and `d` is already non-negative.
                let whole = d.with_scale(0);
                let left = whole.as_bigint_and_exponent().0;
                let frac = &d - &whole;
                let right = (frac * BigDecimal::from(100))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                (left, right)
            }
        };

        // `.get(currency, list(self.CURRENCY_FORMS.values())[0])` — no raise.
        let forms = self.forms_or_fallback(currency);
        let one = BigInt::from(1u32);

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`.
        // Indexing is unconditional in Python and every entry has arity 2.
        let left_str = int_to_word(&left);
        let unit = &forms.unit[if left == one { 0 } else { 1 }];
        let mut result = format!("{} {}", left_str, unit);

        // `if cents and right:` — note this *drops* the segment when
        // cents=False; there is no terse branch. And `right == 0` is falsy, so
        // a whole float like 1.0 never renders cents.
        if cents && !right.is_zero() {
            let cents_str = int_to_word(&right);
            let subunit = &forms.subunit[if right == one { 0 } else { 1 }];
            result.push_str(separator);
            result.push_str(&cents_str);
            result.push(' ');
            result.push_str(subunit);
        }

        // `result = self.negword + result` — negword is used raw (trailing
        // space intact), not `.strip()`ed as `Num2Word_Base` would.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `return result.strip()` — a no-op in practice (`_int_to_word` never
        // returns a padded string and negword's space is consumed by the word
        // after it), reproduced for fidelity.
        Ok(result.trim().to_string())
    }

    // `to_cheque` is NOT overridden. MG inherits `Num2Word_Base.to_cheque`,
    // which the trait default (`currency::default_to_cheque`) already mirrors:
    // strict `CURRENCY_FORMS[currency]` lookup -> NotImplementedError,
    // divisor 100, `_money_verbose(whole)` -> MG's `to_cardinal`, the plural
    // form `cr1[-1]` unconditionally, and `.upper()` over the whole string.
    //   to_cheque(1234.56, "EUR")
    //     -> "IRAY ARIVO ROA ZATO TELOPOLO EFATRA AND 56/100 EUROS"
    //   to_cheque(1234.56, "GBP") -> NotImplementedError
}
