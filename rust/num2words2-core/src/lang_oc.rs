//! Port of `lang_OC.py` (Occitan). Registry key `"oc"` → `Num2Word_OC`.
//!
//! Shape: **self-contained**. `Num2Word_OC` subclasses `Num2Word_Base` but its
//! `setup` defines only flat `ones`/`tens` lists plus `hundred`/`thousand`/
//! `million` strings — it never sets `high_numwords`, `mid_numwords` or
//! `low_numwords`. `Num2Word_Base.__init__` gates card construction behind
//! `if any(hasattr(self, field) for field in [...])`, so for OC that branch is
//! skipped entirely: **`self.cards` is never built and `self.MAXVAL` is never
//! assigned**. `to_cardinal` is overridden outright and drives a recursive
//! `_int_to_word`. Consequently `cards`/`maxval`/`merge` stay at their trait
//! defaults here, and **there is no overflow check at any magnitude** — see
//! bug 1 below for what happens instead.
//!
//! Inherited from `Num2Word_Base` and *not* overridden by OC: nothing in
//! scope. OC overrides all four of `to_cardinal`, `to_ordinal`,
//! `to_ordinal_num` and `to_year` itself, so no base-class behaviour leaks in.
//! (`to_year(self, val, longval=True)` ignores `longval` and simply forwards
//! to `to_cardinal`.)
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! This is a port, not a rewrite. Everything below looks wrong and is exactly
//! what Python emits, each line confirmed against the frozen corpus:
//!
//! 1. **Numbers ≥ 10^9 are not spelled at all.** `_int_to_word`'s final `else`
//!    is `return str(number)  # Fallback for very large numbers`, so
//!    `to_cardinal(10**9)` == `"1000000000"` — the decimal digits, verbatim.
//!    This is not an error path: it returns `Ok`. It composes with everything
//!    downstream, giving `to_ordinal(10**9)` == `"1000000000-en"` and
//!    `to_cardinal(-10**9)` == `"mens 1000000000"`. Because there is no
//!    MAXVAL, this holds for arbitrarily large BigInts (corpus covers 10^21).
//! 2. **No teens.** `tens[1]` is "dètz" and 11..=19 fall through the generic
//!    `tens[t] + " " + ones[o]` arm, so 11 == "dètz un", 15 == "dètz cinc",
//!    19 == "dètz nòu". Real Occitan has "onze"/"quinze"/"dètz-e-nòu".
//! 3. **No elision or hyphens.** 21 == "vint un" (Occitan: "vint-e-un"),
//!    99 == "nonanta nòu".
//! 4. **Bare hundreds/thousands/millions take an explicit "un".** 100 ==
//!    "un cent" (Occitan: "cent"), 1000 == "un mil", 10^6 == "un milion".
//!    Note the asymmetry: the hundreds arm indexes `ones[hundreds_val]`
//!    directly, while the thousands/millions arms *recurse*, which is why
//!    100000 == "un cent mil" (recursion produces "un cent" for the 100).
//! 5. **`million` is never pluralised**: 999999999 ends "...nòu milion...".
//! 6. **Zero, minus and the decimal word (fixed, gladiaio/num2words2#154).**
//!    Python's `_int_to_word(0)` reads
//!    `self.ones[0] if self.ones[0] else "zero"`; `ones[0]` is hardcoded `""`
//!    in `setup`, which is falsy, so Python always yields the English "zero",
//!    and its `negword`/`pointword` are the English "minus "/"point". This
//!    port says the Occitan "zèro", "mens " and "virgula" (Occitan writes a
//!    decimal comma): `0` == "zèro", `-1` == "mens un", `1.5` == "un virgula
//!    cinc". Modelled as a plain [`ZERO_WORD`] return.
//!
//! # Error variants
//!
//! For integer input, all four cardinal-family modes are total: OC has no
//! MAXVAL, no dict lookups and no `int()` of a user token, so nothing can
//! raise. `to_currency` is likewise total (see below — it never raises, not
//! even for an unknown code). The only raising path in scope is `to_cheque`,
//! which OC inherits unmodified from `Num2Word_Base` and which turns a missing
//! `CURRENCY_FORMS` key into `NotImplementedError`. The remaining `ok: false`
//! rows in the corpus for `"oc"` are `fraction` (TypeError) and `w2n_cardinal`
//! (Words2NumError) — both out of scope per PORTING.md.
//!
//! # Currency surface
//!
//! `Num2Word_OC` declares its own two-entry `CURRENCY_FORMS` (EUR, USD) and
//! overrides `to_currency` **wholesale** — it shares no code with
//! `Num2Word_Base.to_currency` and therefore never reaches `pluralize`,
//! `_money_verbose`, `_cents_verbose`, `_cents_terse`, `parse_currency_parts`
//! or `CURRENCY_PRECISION`. Those hooks are deliberately left at their trait
//! defaults here. See [`LangOc::to_currency`] for the four behavioural
//! divergences from base that this buys.
//!
//! `to_cheque` is *not* overridden in Python, so `Num2Word_Base.to_cheque`
//! runs and `crate::currency::default_to_cheque` serves it unchanged. It needs
//! only [`Lang::currency_forms`] (for the `KeyError` -> `NotImplementedError`
//! conversion), [`Lang::lang_name`] (for the message), the default
//! `currency_precision` of 100 and the default `money_verbose`, which routes
//! back through OC's own `to_cardinal`.
//!
//! `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are both `{}` — OC declares
//! neither, and the empty dicts it inherits from `Num2Word_Base` are never
//! mutated by another module (only `Num2Word_EUR`'s table is, by
//! `Num2Word_EN.__init__`; OC does not descend from `Num2Word_EUR`). Confirmed
//! against the live interpreter, not the source:
//!
//! ```text
//! CONVERTER_CLASSES['oc'].CURRENCY_FORMS
//!   {'EUR': (('èuro', 'èuros'), ('centim', 'centims')),
//!    'USD': (('dollar', 'dollars'), ('cent', 'cents'))}
//! CONVERTER_CLASSES['oc'].CURRENCY_PRECISION   {}
//! CONVERTER_CLASSES['oc'].CURRENCY_ADJECTIVES  {}
//! ```

use crate::base::{Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `setup`: `self.negword = "minus "` (English; Occitan "mens " here, bug 6).
/// Note the trailing space; it is part of the constant and is what separates
/// the sign from the number.
const NEGWORD: &str = "mens ";

/// What `_int_to_word(0)` returns: English "zero" in Python, "zèro" here.
const ZERO_WORD: &str = "zèro";

/// `setup`: `self.ones`. Index 0 is the empty string (see bug 6).
const ONES: [&str; 10] = [
    "", "un", "dos", "tres", "quatre", "cinc", "sièis", "sèt", "uèch", "nòu",
];

/// `setup`: `self.tens`. Index 0 is empty and index 1 is the bare "dètz",
/// which is why there are no teen forms (bug 2).
const TENS: [&str; 10] = [
    "",
    "dètz",
    "vint",
    "trenta",
    "quaranta",
    "cinquanta",
    "seissanta",
    "setanta",
    "ochanta",
    "nonanta",
];

const HUNDRED: &str = "cent";
const THOUSAND: &str = "mil";
const MILLION: &str = "milion";

/// `setup`: `self.pointword = "point"` (English; Occitan "virgula" here, bug
/// 6). Live on the float/Decimal path, where OC interpolates it raw (no
/// `title()`) between the integral part and the spelled-out fraction digits.
const POINTWORD: &str = "virgula";

/// `Num2Word_OC.to_currency`'s own default: `separator=" "`. Note this is *not*
/// `Num2Word_Base`'s `","` — see [`BASE_DEFAULT_SEPARATOR`].
const SEPARATOR: &str = " ";

/// `Num2Word_Base.to_currency`'s default separator, which the callers use as a
/// de-facto "caller said nothing" sentinel.
///
/// Python resolves a default argument at the *call site*: omit `separator=` and
/// `Num2Word_OC.to_currency` binds its own `" "`. The `Lang` trait takes
/// `separator: &str` unconditionally, so the resolution has already happened by
/// the time we are called — and both callers hardcode **base**'s default rather
/// than the language's:
///
/// * `num2words2/__init__.py`'s Rust fast path: `kwargs.get("separator", ",")`
/// * `bench/diff_test.py`: `_rust.to_currency(lang, arg, is_int, code, True, ",", False)`
///
/// The frozen corpus was generated through the *Python* converter with no
/// `separator=` kwarg, so every `"oc"` currency row shows OC's `" "`:
///
/// ```text
/// {"lang": "oc", "to": "currency:EUR", "arg": "12.34",
///  "out": "dètz dos èuros trenta quatre centims"}
/// ```
///
/// Taking the incoming `","` at face value would render
/// `"dètz dos èuros,trenta quatre centims"` and fail all 60 rows that carry
/// cents. So `","` is read back as "unset" and OC's own default restored.
/// This is right across the whole reachable input space bar one case — an
/// explicit `separator=","`, which Python renders with a comma and this
/// renders with a space. A real fix belongs at the boundary (pass
/// `Option<&str>` and let the language supply its default), which is `base.rs`
/// / `currency.rs` / `__init__.py` and out of scope for this file; it is
/// flagged in the report. `lang_as.rs`, `lang_ba.rs` and `lang_br.rs` — the
/// other languages whose Python signature carries a non-`","` default — resolve
/// it the same way.
const BASE_DEFAULT_SEPARATOR: &str = ",";

pub struct LangOc {
    /// `Num2Word_OC.CURRENCY_FORMS`, built once in [`LangOc::new`].
    ///
    /// The binding holds each converter in a `OnceLock` and hands out `&'static`
    /// references, so this is constructed one time per process. Rebuilding it
    /// per `to_currency` call is what made an earlier revision of this port an
    /// order of magnitude slower than the Python it replaces.
    forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangOc {
    fn default() -> Self {
        Self::new()
    }
}

impl LangOc {
    pub fn new() -> Self {
        // CURRENCY_FORMS = {
        //     "EUR": (("èuro", "èuros"), ("centim", "centims")),
        //     "USD": (("dollar", "dollars"), ("cent", "cents")),
        // }
        // Two forms per side, exactly as Python: `to_currency` indexes [0] and
        // [1] directly, so the arity is load-bearing.
        let mut forms = HashMap::with_capacity(2);
        forms.insert(
            "EUR",
            CurrencyForms::new(&["èuro", "èuros"], &["centim", "centims"]),
        );
        forms.insert(
            "USD",
            CurrencyForms::new(&["dollar", "dollars"], &["cent", "cents"]),
        );
        LangOc { forms }
    }

    /// `list(self.CURRENCY_FORMS.values())[0]` — the fallback `to_currency`
    /// uses for an unrecognised code (bug 7).
    ///
    /// Python dicts iterate in insertion order and `CURRENCY_FORMS` is a class
    /// body literal, so "the first value" is permanently EUR's. That ordering
    /// is the *only* thing pinning this entry, which is why it is spelled out
    /// as a constant here rather than derived from the `HashMap` (which has no
    /// order to derive it from).
    fn fallback_forms(&self) -> &CurrencyForms {
        self.forms
            .get("EUR")
            .expect("EUR is the first CURRENCY_FORMS entry and is inserted in new()")
    }

    /// Python's `_int_to_word`.
    ///
    /// Every threshold below is compared against a `BigInt`; the value is
    /// deliberately never narrowed to a fixed-width int, because the final
    /// `else` (bug 1) must stringify arbitrarily large inputs. Indices into
    /// `ONES`/`TENS` are narrowed only after the surrounding guard has proven
    /// them to be in `0..10`.
    fn int_to_word(&self, number: &BigInt) -> String {
        // `if number == 0: return self.ones[0] if self.ones[0] else "zero"`
        // — ones[0] is "" (falsy), so this is unconditionally the zero word
        // (bug 6).
        if number.is_zero() {
            return ZERO_WORD.to_string();
        }

        // `if number < 0: return self.negword + self._int_to_word(abs(number))`.
        // Dead code on every in-scope path: `to_cardinal` strips the sign from
        // the *string* before calling in, so this arm is only reachable from
        // `to_currency` (out of scope). Ported anyway for fidelity — it is the
        // reason a stray negative yields "mens " glued on with no extra space.

        if number.is_negative() {
            return format!("{}{}", NEGWORD, self.int_to_word(&number.abs()));
        }

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let billion = BigInt::from(1_000_000_000);

        if number < &ten {
            // 1..=9 — narrowing is safe under the guard.
            let i = u32::try_from(number).expect("0 < number < 10") as usize;
            ONES[i].to_string()
        } else if number < &hundred {
            // Both operands are positive here, so Python's floor-division `//`
            // and `%` agree with Rust's truncating `/` and `%`.
            let tens_val = u32::try_from(number / &ten).expect("< 10") as usize;
            let ones_val = u32::try_from(number % &ten).expect("< 10") as usize;
            if ones_val == 0 {
                TENS[tens_val].to_string()
            } else {
                // No hyphen, no "e" (bugs 2 and 3).
                format!("{} {}", TENS[tens_val], ONES[ones_val])
            }
        } else if number < &thousand {
            let hundreds_val = u32::try_from(number / &hundred).expect("< 10") as usize;
            let remainder = number % &hundred;
            // Indexes ONES directly rather than recursing, so 100 == "un cent"
            // (bug 4).
            let mut result = format!("{} {}", ONES[hundreds_val], HUNDRED);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            result
        } else if number < &million {
            let thousands_val = number / &thousand;
            let remainder = number % &thousand;
            let mut result = format!("{} {}", self.int_to_word(&thousands_val), THOUSAND);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            result
        } else if number < &billion {
            let millions_val = number / &million;
            let remainder = number % &million;
            // MILLION is never pluralised (bug 5).
            let mut result = format!("{} {}", self.int_to_word(&millions_val), MILLION);
            if !remainder.is_zero() {
                result.push(' ');
                result.push_str(&self.int_to_word(&remainder));
            }
            result
        } else {
            // `return str(number)  # Fallback for very large numbers` (bug 1).
            // Not an error — a successful return of the bare decimal digits.
            number.to_string()
        }
    }

    /// `Num2Word_OC.to_cardinal`, driven by the **string** form of the value
    /// rather than the value itself — this is the float/Decimal branch:
    ///
    /// ```python
    /// n = str(number).strip()
    /// if n.startswith("-"):
    ///     n = n[1:]
    ///     ret = self.negword
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
    /// The integer [`Lang::to_cardinal`] models the same method on the *value*,
    /// which for an int is equivalent (stripping "-" from `str(int)` is just
    /// `abs`) and much cheaper. Once a "." can appear that equivalence breaks,
    /// so this arm follows Python literally.
    ///
    /// Details that look like slips and are not:
    ///
    /// * OC does **not** recurse on the sign (unlike some siblings): it strips
    ///   exactly one leading "-" from the string and records `negword` as a
    ///   flat prefix on `ret`.
    /// * `split(".", 1)` splits on the **first** dot only, so a stray second
    ///   dot lands in `right` and reaches `int(digit)` as a bad literal.
    ///   `str()` never emits two dots, but `split_once` keeps the semantics.
    /// * Each fraction character is rendered through `_int_to_word(int(digit))`,
    ///   **not** a bare `ones[]` lookup — so a `'0'` digit becomes "zèro"
    ///   (via bug 6), where the integer path's `ones[0]` would be "".
    /// * `int(digit)` is per **character**, so a malformed fraction (the 'e' of
    ///   "1.5e+16") raises `ValueError` quoting one character, where a
    ///   malformed whole `n` (the "1e+16" of an exponent literal) quotes the
    ///   entire literal.
    fn cardinal_from_str(&self, n: &str) -> Result<String> {
        // `n = str(number).strip()` — str()'s output never carries surrounding
        // whitespace, so this only ever no-ops. Reproduced, not assumed away.
        let n = n.trim();

        // `if n.startswith("-"): n = n[1:]; ret = self.negword else: ret = ""`.
        let (ret_prefix, n) = match n.strip_prefix('-') {
            Some(rest) => (NEGWORD, rest),
            None => ("", n),
        };

        if let Some((left, right)) = n.split_once('.') {
            // `ret += int_to_word(int(left)) + " " + pointword + " "`.
            // `int(left)` runs before the loop, so a bad left part raises first.
            let mut ret = format!(
                "{}{} {} ",
                ret_prefix,
                self.int_to_word(&python_int(left)?),
                POINTWORD
            );
            // `for digit in right: ret += int_to_word(int(digit)) + " "`.
            for ch in right.chars() {
                // `int(digit)` on a single character: a non-digit (the 'e' of
                // "1.5e+16") raises ValueError quoting just that character.
                let d = python_int(&ch.to_string())?;
                ret.push_str(&self.int_to_word(&d));
                ret.push(' ');
            }
            // `return ret.strip()`.
            Ok(ret.trim().to_string())
        } else {
            // `return (ret + int_to_word(int(n))).strip()`. Reached by an
            // integral Decimal ("5" -> "cinc") and by exponent-form floats
            // ("1e+16"), where `int(n)` then raises.
            Ok(format!("{}{}", ret_prefix, self.int_to_word(&python_int(n)?))
                .trim()
                .to_string())
        }
    }
}

/// CPython's `repr(float)` — which for a `float` is also `str(float)`, and
/// therefore the entire input to OC's float path.
///
/// Two halves, both load-bearing.
///
/// # 1. The digits
///
/// `repr` is shortest-round-trip, and so is Rust's `{}`/`{:e}`, so the digits
/// agree — *except on exact ties*, where the double sits precisely halfway
/// between two equally short candidates that both round-trip. CPython's dtoa
/// breaks those to **even**; Rust's shortest formatter does not (it disagrees
/// with CPython on roughly 1 double in 10,000 sampled uniformly).
/// `670352580196876.25` is exactly such a value: CPython prints
/// `670352580196876.2`, Rust's `{:e}` gives `...3`.
///
/// The repair is to re-derive the digits through Rust's *exact* formatter
/// (`{:.n$}`), which **is** round-half-to-even, once `{:e}` has told us how
/// many fractional digits the shortest form has. A tie cannot change that
/// count (a carry would produce a *shorter* representation, which the shortest
/// algorithm would have found first), so the count is safe to take from `{:e}`
/// even though the final digit is not.
///
/// # 2. The placement
///
/// CPython switches to exponent notation iff `decpt <= -4 || decpt > 16`,
/// pads the exponent to two digits, and appends `.0` to anything that would
/// otherwise look like an integer. Rust's `{}` does none of this, so `1e16`
/// and `1.0` would both come out wrong. Both matter here: `str(1.0)` is `"1.0"`
/// -> `"un virgula zèro"`, and `str(1e16)` is `"1e+16"` -> `ValueError`.
///
/// The `precision` that `FloatValue::Float` carries is deliberately *not* used
/// to shortcut this. It is `abs(Decimal(str(value)).as_tuple().exponent)`,
/// which for an exponent-form repr is the *exponent*, not a digit count:
/// `1e16` arrives with `precision == 16`.
fn python_float_repr(v: f64) -> String {
    // repr(nan) / repr(inf) / repr(-inf). OC feeds these straight to int(),
    // which rejects them like any other bad literal.
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-inf" } else { "inf" }).to_string();
    }
    // The sign bit, not `v < 0.0`: repr(-0.0) is "-0.0", and OC renders that
    // "mens zèro virgula zèro".
    let sign = if v.is_sign_negative() { "-" } else { "" };
    let a = v.abs();

    // `{:e}` is shortest-round-trip in `<d>[.<ddd>]e<exp>` form, so the digits
    // and the decimal-point position fall straight out. `decpt` is CPython's:
    // the value is `0.<digits> * 10**decpt`.
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

/// CPython's `str(Decimal)` — the spec's to-scientific-string, transcribed
/// from `_pydecimal.Decimal.__str__`:
///
/// ```python
/// leftdigits = self._exp + len(self._int)
/// if self._exp <= 0 and leftdigits > -6:
///     dotplace = leftdigits          # no exponent required
/// else:
///     dotplace = 1                   # scientific: 1 digit left of the point
/// if dotplace <= 0:
///     intpart, fracpart = '0', '.' + '0'*(-dotplace) + self._int
/// elif dotplace >= len(self._int):
///     intpart, fracpart = self._int + '0'*(dotplace-len(self._int)), ''
/// else:
///     intpart, fracpart = self._int[:dotplace], '.' + self._int[dotplace:]
/// exp = '' if leftdigits == dotplace else 'E' + "%+d" % (leftdigits-dotplace)
/// return sign + intpart + fracpart + exp
/// ```
///
/// `BigDecimal` is the same (unscaled, scale) pair as Python's `(_int, _exp)`
/// — `_exp == -scale` — and `from_str` preserves the scale as written, so
/// `Decimal("1.10")`'s trailing zero survives the crossing.
///
/// This reads `as_bigint_and_exponent()` rather than `BigDecimal`'s own
/// `Display`, which is **not** `str(Decimal)`: it renders `Decimal("0.00")` as
/// `"0"`, losing the two digits OC would have spoken (`"zèro virgula zèro zèro"`).
/// The capital `E` and the unpadded exponent are Python's too.
///
/// # The negative-zero hole
///
/// Python's `Decimal` carries a sign flag independent of its digits, so
/// `str(Decimal("-0.0"))` is `"-0.0"` and OC prepends `mens`. `BigInt` has no
/// negative zero, so `BigDecimal::from_str("-0.0")` discards the sign before
/// this function ever sees it, and the `mens` is lost. The discriminator is
/// the original string, which the `FloatValue::Decimal` boundary does not
/// carry — the *float* `-0.0` is fine, because f64 keeps its sign bit. Flagged
/// in the port report; no corpus row falls in the hole.
fn python_decimal_str(d: &BigDecimal) -> String {
    let (unscaled, scale) = d.as_bigint_and_exponent();
    let sign = if unscaled.is_negative() { "-" } else { "" };
    // Python's `_int`: the unsigned coefficient. BigInt renders ASCII digits.
    let int_digits = unscaled.abs().to_string();
    let exp: i64 = -scale;
    let ndig = int_digits.chars().count() as i64;
    let leftdigits = exp + ndig;
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
    } else if dotplace >= ndig {
        (
            format!("{}{}", int_digits, "0".repeat((dotplace - ndig) as usize)),
            String::new(),
        )
    } else {
        let k = dotplace as usize;
        (
            int_digits.chars().take(k).collect::<String>(),
            format!(".{}", int_digits.chars().skip(k).collect::<String>()),
        )
    };

    // `"%+d"` — always signed, never zero-padded. Capital E: context.capitals
    // defaults to 1.
    let exp_part = if leftdigits == dotplace {
        String::new()
    } else {
        format!("E{:+}", leftdigits - dotplace)
    };

    format!("{}{}{}{}", sign, intpart, fracpart, exp_part)
}

/// `str(number)` for whatever the Python dispatcher handed the converter.
///
/// The `FloatValue` split is exactly Python's `isinstance(value, Decimal)`:
/// the two arms stringify by different rules and must not be collapsed.
fn python_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => python_float_repr(*value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Python's `int(s)`, for the strings `str()` can produce.
///
/// `BigInt::from_str` and `int()` agree on everything reachable here: plain
/// ASCII digit runs with an optional sign. They diverge on inputs `str(float)`
/// and `str(Decimal)` cannot emit, so the divergence is unreachable rather than
/// papered over. The message is Python's, quoting the offending literal.
fn python_int(s: &str) -> Result<BigInt> {
    BigInt::from_str(s).map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

impl Lang for LangOc {

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

    /// `to_ordinal(float/Decimal)` — Python's `to_ordinal` is
    /// `to_cardinal(number) + "-en"` for *any* input (no
    /// `verify_ordinal`), so the float path is the float cardinal put through
    /// the same literal transformation: `5.0` -> "cinc virgula zèro-en".
    /// Errors from the cardinal (`int("1e+16")` -> ValueError) propagate
    /// before the transformation, exactly as in Python.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let cardinal = self.cardinal_float_entry(value, None)?;
        Ok(format!("{}-en", cardinal))
    }

    /// `to_ordinal_num(float/Decimal)`: `str(number) + "."`. `repr_str` is the
    /// dispatcher's exact `str(value)` (float repr / `Decimal.__str__`), so
    /// trailing zeros and `1E+2`-style exponent forms survive verbatim.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}.", repr_str))
    }

    /// `converter.str_to_number` — the base `Decimal(value)` parse, except the
    /// Infinity sentinel becomes the ValueError this language's own
    /// `to_cardinal` raises (`int("Infinity")` after the `"." in n` test
    /// fails); the shared dispatcher would otherwise report Base's
    /// OverflowError. NaN keeps the base sentinel: the dispatcher's
    /// ValueError for it already matches `int("NaN")`.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } => Err(N2WError::Value(
                "invalid literal for int() with base 10: 'Infinity'".into(),
            )),
            p => Ok(p),
        }
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
        POINTWORD
    }

    /// Python's `to_cardinal`.
    ///
    /// The original works on `str(number)`: it strips a leading "-" textually,
    /// records `negword`, and re-parses the remainder with `int()`. For integer
    /// input the `"." in n` split is unreachable, so this reduces to sign-strip
    /// + `_int_to_word` + `.strip()`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // `n = str(number).strip()` — no-op on a BigInt's decimal form.
        // `if n.startswith("-"): n = n[1:]; ret = self.negword`
        let (ret, n) = if value.is_negative() {
            (NEGWORD, value.abs())
        } else {
            ("", value.clone())
        };

        // `return (ret + self._int_to_word(int(n))).strip()`.
        // The trailing `.strip()` is a no-op for integers — `_int_to_word`
        // never pads its result, and NEGWORD's space is interior once
        // concatenated — but it is applied here to match the source exactly.
        Ok(format!("{}{}", ret, self.int_to_word(&n)).trim().to_string())
    }

    /// Python: `return self.to_cardinal(number) + "-en"`.
    ///
    /// Unconditional suffix — no agreement, no special-casing of 0 or of
    /// negatives, and no guard against the bug-1 digit fallback. Hence
    /// `to_ordinal(0)` == "zèro-en", `to_ordinal(-1)` == "mens un-en" and
    /// `to_ordinal(10**9)` == "1000000000-en".
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}-en", self.to_cardinal(value)?))
    }

    /// Python: `return str(number) + "."`.
    ///
    /// Note this ignores `_int_to_word` entirely, so it is total and keeps the
    /// minus sign: `to_ordinal_num(-1)` == "-1.".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}.", value))
    }

    /// Python: `def to_year(self, val, longval=True): return self.to_cardinal(val)`.
    ///
    /// `longval` is accepted and ignored; there is no era handling, so a
    /// negative year keeps the plain "mens " prefix rather than gaining a
    /// "BC"-style suffix: `to_year(-500)` == "mens cinc cent".
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// The float/Decimal cardinal path.
    ///
    /// OC reaches non-integers through its own `to_cardinal` override, not
    /// through `Num2Word_Base.to_cardinal_float`, so this hook exists only to
    /// route to the same string-driven logic and stop the base implementation
    /// running. There is **no `float2tuple` here, no `10**precision` scaling,
    /// no banker's rounding, and none of the f64 artefacts that path exists to
    /// preserve** — OC's whole float path is `n = str(number).strip()` followed
    /// by pure string surgery. The artefact cases are trivially right because
    /// `str(2.675)` is `"2.675"` (repr is shortest-round-trip), so the digits
    /// are `6 7 5` by construction; there is nothing to compute and nothing to
    /// rescue. See [`python_float_repr`] and [`python_decimal_str`] for the two
    /// stringifications this port lives or dies on.
    ///
    /// `precision_override` is the `precision=` kwarg. `__init__.py` applies it
    /// by assigning `converter.precision`, which OC never reads (its
    /// `to_cardinal` consults `str(number)`, never `self.precision`) — so it is
    /// accepted and **ignored**, matching the live interpreter.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_from_str(&python_str(value))
    }

    // ---- currency ------------------------------------------------------

    /// Used only for `to_cheque`'s NotImplementedError message — OC's
    /// `to_currency` has no error path that mentions the class name.
    fn lang_name(&self) -> &str {
        "Num2Word_OC"
    }

    /// `self.CURRENCY_FORMS[code]`.
    ///
    /// Deliberately a plain lookup with **no** EUR fallback: this hook exists
    /// for `Num2Word_Base.to_cheque`, which does `self.CURRENCY_FORMS[currency]`
    /// inside a `try` and converts the `KeyError` into `NotImplementedError`.
    /// `None` here is what produces that. OC's `to_currency` does *not* go
    /// through this — it calls `.get(currency, <first value>)` and can never
    /// miss, so it consults `LangOc::fallback_forms` itself.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.forms.get(code)
    }

    // `currency_precision` is not overridden: OC's CURRENCY_PRECISION is the
    // empty dict inherited from Num2Word_Base, so `.get(code, 100)` is 100 for
    // every code — the trait default. `to_cheque` reads it (giving the "56/100"
    // in "... AND 56/100 ÈUROS"); OC's `to_currency` never looks at it, which
    // is why KWD/BHD get 2-decimal cents and JPY is not rounded to a whole unit
    // (bug 7 below).
    //
    // `currency_adjective` is not overridden: CURRENCY_ADJECTIVES is `{}`.
    //
    // `pluralize` is not overridden: it is abstract in Num2Word_Base and OC
    // reaches it from neither `to_currency` (which indexes `cr1`/`cr2` by hand)
    // nor `to_cheque` (which takes `cr1[-1]` unconditionally). The trait
    // default raises, matching Python's `raise NotImplementedError`.
    //
    // `money_verbose` / `cents_verbose` / `cents_terse` are not overridden:
    // Python's `_money_verbose` is `return self.to_cardinal(number)` — the
    // trait default — and it is the only one of the three OC can reach, via
    // `to_cheque`. `to_currency` calls `_int_to_word` directly instead.
    //
    // `to_cheque` is not overridden: Num2Word_Base.to_cheque runs verbatim, and
    // `currency::default_to_cheque` is its port. Traced below.

    /// Python's `Num2Word_OC.to_currency` — a full override that shares nothing
    /// with `Num2Word_Base.to_currency`:
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True,
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
    ///
    /// # Faithfully reproduced Python bugs / oddities (continuing the list above)
    ///
    /// 7. **An unknown currency code silently becomes EUR.** The lookup is
    ///    `.get(currency, list(self.CURRENCY_FORMS.values())[0])`, and the first
    ///    value is EUR's. So `to_currency(1, "JPY")` is `"un èuro"`, not an
    ///    error — this method has no `NotImplementedError` path at all, unlike
    ///    every other language and unlike OC's own `to_cheque`. The corpus
    ///    pins it for GBP, JPY, KWD, BHD, INR, CNY and CHF, all 12 rows each:
    ///    `{"lang": "oc", "to": "currency:JPY", "arg": "1", "out": "un èuro"}`.
    /// 8. **The int/float split is done by string, not by type.** Base branches
    ///    on `isinstance(val, int)` and skips cents for a true int while still
    ///    rendering them for `1.0`. OC instead asks whether `str(val)` contains
    ///    a `"."`, then throws the answer away anyway: `1.0` -> `parts[1] == "0"`
    ///    -> `right == 0` -> falsy -> `if cents and right` skips the segment. So
    ///    int `1` and float `1.0` both give `"un èuro"` — the same output base
    ///    reaches by the opposite reasoning. `CurrencyValue`'s two arms are
    ///    still kept distinct here because they produce different strings for
    ///    `str(val)` ("1" vs "1.0"), which is what the code actually branches on.
    /// 9. **Cents are truncated to 2 digits regardless of the currency.**
    ///    `parts[1][:2]` is hardcoded, so a 3-decimal currency loses its mils
    ///    (KWD 1.234 -> "vint tres centims", not 234 fils) and a 0-decimal one
    ///    gains cents that cannot exist (JPY 0.5 -> "cinquanta centims"). Both
    ///    follow from bug 7 anyway, since neither code is in the table.
    /// 10. **`cents=False` omits the cents, it does not abbreviate them.**
    ///    Base's `cents=False` means "render the subunit as digits" via
    ///    `_cents_terse`. OC's `if cents and right:` simply drops the whole
    ///    segment, so `to_currency(12.34, cents=False)` is `"dètz dos èuros"`.
    ///    `_cents_terse` is unreachable from here.
    /// 11. **`adjective` is accepted and completely ignored** — no
    ///    `prefix_currency` call, and `CURRENCY_ADJECTIVES` is empty regardless.
    ///
    /// # `str(val)` on the Rust side, and the one place this port diverges
    ///
    /// `parts = str(val).split(".")` runs on the value *after* `abs()`, so the
    /// string never carries a sign. `CurrencyValue::Decimal` was parsed from the
    /// `str(value)` the Python shim produced, and `BigDecimal` preserves scale,
    /// so `to_string()` round-trips that literal exactly ("1.0" stays "1.0",
    /// not "1"; "0.01" stays "0.01") for every value whose `str` is in plain
    /// decimal form. That covers the whole corpus and everything a caller is
    /// likely to pass.
    ///
    /// It does not cover values whose `str` is *exponential*, because Python
    /// and `BigDecimal` switch to exponential notation at different
    /// magnitudes. `int()` rejects an exponential token, so Python raises
    /// `ValueError` wherever its own `repr` went exponential — meaning the
    /// disagreement is over *whether the ValueError happens*, not over any
    /// spelled-out string:
    ///
    /// | value | `str(val)` | Python | here |
    /// |---|---|---|---|
    /// | `1e15` | `1000000000000000.0` | `"1000000000000000 èuros"` | same |
    /// | `0.0001` | `0.0001` | `"zèro èuros"` | same |
    /// | `1e-05` | `1e-05` | ValueError | `"zèro èuros"` |
    /// | `1e-06` | `1e-06` | ValueError | `"zèro èuros"` |
    /// | `1e-07` | `1e-07` | ValueError | ValueError (`N2WError::Value`) |
    /// | `1e+16` | `1e+16` | ValueError | ValueError (`N2WError::Value`) |
    ///
    /// The window is exactly `1e-06 ..= 1e-05`-ish: Python's float `repr` goes
    /// exponential below `1e-4`, while `BigDecimal`'s `Display` holds out until
    /// it would print more than five leading zeros. Outside that window the two
    /// agree, including on the exception *type* at `1e-07` and `1e+16`, where
    /// `BigDecimal` also prints exponentially and `BigInt::from_str` rejects it
    /// — the same `ValueError`-shaped failure by the same route.
    ///
    /// **This is not fixable in this file.** `CurrencyValue::Decimal` carries a
    /// parsed `BigDecimal`, not the string it was parsed from, and float and
    /// `Decimal` reprs disagree in precisely this window: Python's
    /// `str(1e-05)` is `"1e-05"` (-> ValueError) while
    /// `str(Decimal("1e-5"))` is `"0.00001"` (-> `"zèro èuros"`). Both arrive
    /// here as the identical `BigDecimal { int_val: 1, scale: 5 }`, so no rule
    /// applied to the value can be right for both — the two inputs are already
    /// indistinguishable by the time OC is called. Making it exact needs
    /// `CurrencyValue::Decimal` to keep the original `str(value)` alongside the
    /// number, which is a `currency.rs` change and out of scope here. Flagged
    /// in the report; no corpus row falls in the window.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool, // bug 11: accepted and ignored
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        // Restore OC's own `separator=" "` default; see BASE_DEFAULT_SEPARATOR.
        let separator = if separator == BASE_DEFAULT_SEPARATOR {
            SEPARATOR
        } else {
            separator
        };

        // `if val < 0: is_negative = True; val = abs(val)`. The `abs` is folded
        // into the stringification below, which is the only place `val` is read
        // afterwards.
        let is_negative = val.is_negative();

        // `parts = str(val).split(".")`
        let s = match val {
            CurrencyValue::Int(i) => i.abs().to_string(),
            CurrencyValue::Decimal { value: d, .. } => d.abs().to_string(),
        };
        // Python's `split(".")` splits on *every* dot; `parts[0]` and `parts[1]`
        // are the first two segments. `split` + two `next()`s reproduces that,
        // where `splitn(2, ..)` would fold a hypothetical third segment into
        // `parts[1]`. `str(val)` never has two dots, but the distinction costs
        // nothing.
        let mut segments = s.split('.');
        let p0 = segments.next().unwrap_or("");
        let p1 = segments.next();

        // `left = int(parts[0]) if parts[0] else 0`
        let left = if p0.is_empty() {
            BigInt::zero()
        } else {
            // Python's bare `int()` raises ValueError on a non-numeric token,
            // which is the only way this call can fail.
            BigInt::from_str(p0).map_err(|e| N2WError::Value(e.to_string()))?
        };

        // `right = int(parts[1][:2].ljust(2, "0")) if len(parts) > 1 and parts[1] else 0`
        // — `len(parts) > 1` is `p1.is_some()`, `parts[1]` truthiness is the
        // non-empty check. Sliced and padded by `chars`, never by bytes.
        let right = match p1 {
            Some(frac) if !frac.is_empty() => {
                let mut digits: String = frac.chars().take(2).collect();
                while digits.chars().count() < 2 {
                    digits.push('0'); // `.ljust(2, "0")`
                }
                BigInt::from_str(&digits).map_err(|e| N2WError::Value(e.to_string()))?
            }
            _ => BigInt::zero(),
        };

        // `cr1, cr2 = self.CURRENCY_FORMS.get(currency, list(...values())[0])`
        // — bug 7: the fallback is EUR, so this never fails.
        let forms = match self.forms.get(currency) {
            Some(f) => f,
            None => self.fallback_forms(),
        };
        let (cr1, cr2) = (&forms.unit, &forms.subunit);

        // `result = left_str + " " + (cr1[1] if left != 1 else cr1[0])`
        let left_str = self.int_to_word(&left);
        let mut result = format!(
            "{} {}",
            left_str,
            if left.is_one() { &cr1[0] } else { &cr1[1] }
        );

        // `if cents and right:` — bug 10: an omission, not a terse rendering.
        if cents && !right.is_zero() {
            // `result += separator + cents_str + " " + (cr2[1] if right != 1 else cr2[0])`
            result.push_str(separator);
            result.push_str(&self.int_to_word(&right));
            result.push(' ');
            result.push_str(if right.is_one() { &cr2[0] } else { &cr2[1] });
        }

        // `if is_negative: result = self.negword + result`. NEGWORD's own
        // trailing space is what separates it from the number; there is no
        // `.strip()` on it here, unlike `to_cardinal`'s `"%s " % negword.strip()`
        // idiom elsewhere in the library. Same result, different route.
        if is_negative {
            result = format!("{}{}", NEGWORD, result);
        }

        // `return result.strip()` — a no-op on every reachable input (nothing
        // pads the ends), kept to match the source.
        Ok(result.trim().to_string())
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;

    /// `num2words(v, lang="oc")` for a float.
    fn f(v: f64) -> Result<String> {
        LangOc::new().to_cardinal_float(
            // The shim feeds abs(Decimal(str(v)).as_tuple().exponent); OC
            // ignores it, so a deliberately wrong value pins that it stays so.
            &FloatValue::Float { value: v, precision: 99 },
            None,
        )
    }

    /// `num2words(Decimal(s), lang="oc")`.
    fn d(s: &str) -> Result<String> {
        LangOc::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: 99,
            },
            None,
        )
    }

    /// Every `"to": "cardinal"` corpus row for oc whose arg has a dot.
    #[test]
    fn corpus_cardinal_float() {
        assert_eq!(f(0.0).unwrap(), "zèro virgula zèro");
        assert_eq!(f(0.5).unwrap(), "zèro virgula cinc");
        assert_eq!(f(1.0).unwrap(), "un virgula zèro");
        assert_eq!(f(1.5).unwrap(), "un virgula cinc");
        assert_eq!(f(2.25).unwrap(), "dos virgula dos cinc");
        assert_eq!(f(3.14).unwrap(), "tres virgula un quatre");
        assert_eq!(f(0.01).unwrap(), "zèro virgula zèro un");
        assert_eq!(f(0.1).unwrap(), "zèro virgula un");
        assert_eq!(f(0.99).unwrap(), "zèro virgula nòu nòu");
        assert_eq!(f(1.01).unwrap(), "un virgula zèro un");
        assert_eq!(f(12.34).unwrap(), "dètz dos virgula tres quatre");
        assert_eq!(f(99.99).unwrap(), "nonanta nòu virgula nòu nòu");
        assert_eq!(f(100.5).unwrap(), "un cent virgula cinc");
        assert_eq!(
            f(1234.56).unwrap(),
            "un mil dos cent trenta quatre virgula cinc sièis"
        );
        assert_eq!(f(-0.5).unwrap(), "mens zèro virgula cinc");
        assert_eq!(f(-1.5).unwrap(), "mens un virgula cinc");
        assert_eq!(f(-12.34).unwrap(), "mens dètz dos virgula tres quatre");
    }

    /// Every `"to": "cardinal_dec"` corpus row for oc.
    #[test]
    fn corpus_cardinal_decimal() {
        assert_eq!(d("0.01").unwrap(), "zèro virgula zèro un");
        // The trailing zero is a character, not a computed remainder.
        assert_eq!(d("1.10").unwrap(), "un virgula un zèro");
        assert_eq!(d("12.345").unwrap(), "dètz dos virgula tres quatre cinc");
        // Issue #603: the Decimal arm never float-casts, so the ".99" survives
        // at trillion scale. The left part is past 10^9, hence the bare digits.
        assert_eq!(
            d("98746251323029.99").unwrap(),
            "98746251323029 virgula nòu nòu"
        );
        assert_eq!(d("0.001").unwrap(), "zèro virgula zèro zèro un");
    }

    /// The two artefact cases base's float path exists to rescue. OC never
    /// scales by `10**precision`, so `str()` hands it the right digits and the
    /// `< 0.01` heuristic is not merely unused but unreachable.
    #[test]
    fn float_artefacts_are_not_reachable() {
        // base.float2tuple computes 674.9999999999998 for this one.
        assert_eq!(f(2.675).unwrap(), "dos virgula sièis sèt cinc");
        assert_eq!(f(1.005).unwrap(), "un virgula zèro zèro cinc");
        // Banker's rounding never enters either: these are literal digits.
        assert_eq!(f(2.5).unwrap(), "dos virgula cinc");
        assert_eq!(f(0.005).unwrap(), "zèro virgula zèro zèro cinc");
    }

    /// CPython breaks exact shortest-repr ties to even; Rust's `{:e}` does not.
    /// 670352580196876.2 is such a value. Live-interpreter verified.
    #[test]
    fn shortest_repr_ties_go_to_even() {
        assert_eq!(f(670352580196876.2).unwrap(), "670352580196876 virgula dos");
    }

    /// `str(float)`'s two placement quirks, both observable here.
    #[test]
    fn float_repr_placement() {
        // ADD_DOT_0: an integral float still has a fraction to speak.
        assert_eq!(f(1.0).unwrap(), "un virgula zèro");
        assert_eq!(f(-1.0).unwrap(), "mens un virgula zèro");
        // repr(-0.0) keeps the sign bit; `value < 0.0` would not.
        assert_eq!(f(-0.0).unwrap(), "mens zèro virgula zèro");
        // The digit fallback in _int_to_word reaches the float path too.
        assert_eq!(f(1000000000.5).unwrap(), "1000000000 virgula cinc");
        assert_eq!(f(1e15).unwrap(), "1000000000000000 virgula zèro");
        // Just inside the exponent threshold (decpt == 16).
        assert_eq!(f(9999999999999998.0).unwrap(), "9999999999999998 virgula zèro");
        // 1e-4 stays positional (decpt == -3); 1e-5 does not.
        assert_eq!(f(0.0001).unwrap(), "zèro virgula zèro zèro zèro un");
        // A big-but-spellable integral part (< 10^9).
        assert_eq!(
            f(123456789.5).unwrap(),
            "un cent vint tres milion quatre cent cinquanta sièis mil sèt cent ochanta nòu virgula cinc"
        );
    }

    /// Exponent notation reaches int() as a bad literal. Two distinct messages,
    /// because two distinct int() calls raise. Live-interpreter verified.
    #[test]
    fn exponent_notation_raises_value_error() {
        // No "." in the literal -> int(n) raises, quoting all of it. The sign
        // is stripped by the negword branch *before* int() sees the string, so
        // the message never carries a "-".
        for (v, lit) in [
            (1e16, "1e+16"),
            (1e21, "1e+21"),
            (1e-5, "1e-05"),
            (1e300, "1e+300"),
            (-1e16, "1e+16"),
            (-1e-5, "1e-05"),
            (f64::INFINITY, "inf"),
            (f64::NEG_INFINITY, "inf"),
        ] {
            match f(v) {
                Err(N2WError::Value(m)) => assert_eq!(
                    m,
                    format!("invalid literal for int() with base 10: '{}'", lit)
                ),
                other => panic!("{}: expected Value, got {:?}", lit, other),
            }
        }
        // NaN's repr has no sign to strip.
        match f(f64::NAN) {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: 'nan'")
            }
            other => panic!("nan: expected Value, got {:?}", other),
        }
        // A "." *is* present -> the fraction loop raises on one character.
        for v in [1.5e16, 1.5e-5] {
            match f(v) {
                Err(N2WError::Value(m)) => {
                    assert_eq!(m, "invalid literal for int() with base 10: 'e'")
                }
                other => panic!("{}: expected Value, got {:?}", v, other),
            }
        }
    }

    /// `str(Decimal)` is not `str(float)` and not `BigDecimal`'s Display.
    #[test]
    fn decimal_str_rules() {
        // Display would say "0"; str(Decimal("0.00")) keeps both digits.
        assert_eq!(d("0.00").unwrap(), "zèro virgula zèro zèro");
        assert_eq!(d("5.00").unwrap(), "cinc virgula zèro zèro");
        // Integral Decimals have no "." at all -> the no-dot branch.
        assert_eq!(d("5").unwrap(), "cinc");
        assert_eq!(d("-1").unwrap(), "mens un");
        assert_eq!(d("-0.5").unwrap(), "mens zèro virgula cinc");
        // Exactly at the scientific threshold: adjusted exponent -6 stays
        // positional, -7 flips to "1E-7" and int() then raises.
        assert_eq!(d("0.000001").unwrap(), "zèro virgula zèro zèro zèro zèro zèro un");
        match d("0.0000001") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1E-7'")
            }
            other => panic!("0.0000001: expected Value, got {:?}", other),
        }
        // str(Decimal) goes scientific at exp > 0 with a capital E.
        match d("1E+16") {
            Err(N2WError::Value(m)) => {
                assert_eq!(m, "invalid literal for int() with base 10: '1E+16'")
            }
            other => panic!("1E+16: expected Value, got {:?}", other),
        }
    }

    /// OC never reads `self.precision`, so `precision=` is inert. Verified
    /// against the live interpreter.
    #[test]
    fn precision_override_is_ignored() {
        let l = LangOc::new();
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        assert_eq!(
            l.to_cardinal_float(&v, Some(1)).unwrap(),
            "dos virgula sièis sèt cinc"
        );
        assert_eq!(
            l.to_cardinal_float(&v, Some(9)).unwrap(),
            "dos virgula sièis sèt cinc"
        );
    }
}
