//! Port of `lang_HA.py` (Hausa).
//!
//! Registry check: `__init__.py` line 280 binds `"ha"` to
//! `lang_HA.Num2Word_HA()`, so this is the right class.
//!
//! Shape: **self-contained**. `Num2Word_HA` subclasses `Num2Word_Base` but
//! defines none of `high_numwords`/`mid_numwords`/`low_numwords`, so
//! `Num2Word_Base.__init__` never builds `self.cards` and never assigns
//! `self.MAXVAL`. `to_cardinal` is overridden outright and drives the private
//! `_int_to_hausa` recursion. Therefore `cards`/`merge` stay at their trait
//! defaults here, and Python has **no overflow check at all** — it happily
//! words 10**606 by recursing through "tiriliyan". On a large enough integer
//! the port's recursion overflowed the native stack, so it adds a ceiling
//! (gladiaio/num2words2#203): `maxval` is 10^15, where "tiriliyan" would
//! stack, and every mode raises `OverflowError` from there.
//!
//! Inherited from `Num2Word_Base`, then immediately overridden by HA, so the
//! base versions are never reached: `to_ordinal`, `to_ordinal_num`, `to_year`.
//! HA's `setup()` sets `negword = "ban "` (note the **trailing space**, which
//! the negative branch relies on) and `pointword = "wajen"`; `is_title` stays
//! `False`, so `title()` is a no-op and `exclude_title` never matters.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every item below is what the interpreter
//! actually emits, cross-checked against the frozen corpus:
//!
//! 1. **Scale words lead their multiplier.** `_int_to_hausa` emits
//!    `"dubu " + words(quotient)`, i.e. the scale name comes *first*:
//!    `2000` → "dubu biyu" (literally "thousand two"), `1234` → "dubu ɗari
//!    biyu talatin da huɗu". Real Hausa says "dubu biyu" for 2000 but
//!    "dubu ɗaya da ɗari biyu..." patterns differ; regardless, the ordering
//!    here is whatever the Python does and is preserved verbatim.
//! 2. **`1010` and `10000` collide.** Both render "dubu goma". For `1010`
//!    quotient is 1 (→ bare "dubu") and the remainder 10 appends " goma";
//!    for `10000` quotient is 10 (→ "dubu " + "goma") with no remainder.
//!    The grammar is genuinely ambiguous and lossy. Corpus confirms both.
//! 3. **`to_ordinal` never calls `verify_ordinal`**, so negatives sail
//!    through and stack the two prefixes: `to_ordinal(-1)` == "na ban ɗaya".
//!    `to_ordinal(0)` == "na sifiri" (no crash, unlike Polish).
//! 4. **`to_ordinal_num` used English suffixes ("2nd") (fixed, #224).** The
//!    Python called them "English-style ordinal suffixes as commonly used in
//!    Hausa contexts", while `to_ordinal` reads "na biyu". Without a settled
//!    Hausa numeric form the port raises NotImplementedError ("does not
//!    support to='ordinal_num'") rather than print English.
//! 5. (Former quirk on the English suffixes' teen guard, gone with them.)
//! 6. **Dead code kept for fidelity.** Inside the `>= 1000` loop the
//!    `scale_value == 100` arm is unreachable (anything under 1000 already
//!    returned above, and 1000 is itself a `SCALE` key so the loop always
//!    hits it first), and the trailing `return str(number)` fallback is
//!    likewise unreachable. Both are mirrored below with comments.
//!
//! # Currency
//!
//! `Num2Word_HA` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base`'s version. Consequences, all corpus-confirmed:
//!
//! 7. **An unknown currency code never raises.** Python opens with
//!    `if currency not in self.CURRENCY_FORMS: currency = "NGN"`, so KWD, BHD,
//!    INR and CHF all silently render as naira/kobo. `to_currency` is therefore
//!    total over currency codes — the `Currency code "X" not implemented`
//!    NotImplementedError is unreachable through this path.
//! 8. **`CURRENCY_PRECISION` is ignored.** The divisor is hardcoded `100`, so
//!    JPY — a zero-decimal currency everywhere else in the library — still
//!    grows a "sen" segment: `12.34` → "yen sha biyu da sen talatin da huɗu".
//! 9. **The int/float distinction is erased.** Unlike `base.to_currency`, which
//!    branches on `isinstance(val, int)` to skip cents, HA funnels everything
//!    through `Decimal(str(value))`. So `1` and `1.0` both give "yuro ɗaya",
//!    and neither shows zero cents.
//! 10. **Negatives emitted a double space (fixed, #160).** Python's
//!    `result.insert(0, self.negword)` inserts "ban " *with* its trailing
//!    space, then `" ".join(result)` adds another. The port inserts the
//!    stripped word: `-12.34` → "ban yuro sha biyu da cent talatin da huɗu".
//! 11. **Zero has no major segment.** `major_units > 0` is false for 0, so the
//!    `if not result` fallback fires and hardcodes "sifiri" (never
//!    `to_cardinal(0)`): `0` → "yuro sifiri". Likewise `0.01` → "cent ɗaya"
//!    with no "yuro sifiri" prefix at all.
//! 12. **Dead branch.** `if major_units == 1: ... else: ...` has two
//!    byte-identical arms. Collapsed below; output unchanged.
//!
//! `to_cheque` is **not** overridden, so it comes from `Num2Word_Base` and does
//! a raw `self.CURRENCY_FORMS[currency]` lookup — which *does* raise
//! NotImplementedError for KWD/BHD/INR/CHF. The two surfaces disagree about
//! unknown codes, and the corpus pins both halves of that disagreement.
//! `CURRENCY_ADJECTIVES`/`CURRENCY_PRECISION` are absent from HA and empty in
//! `Num2Word_Base`, so `currency_adjective`/`currency_precision` stay at their
//! trait defaults (`None` / `100`) — which is why `cheque:JPY` prints
//! "AND 56/100 YEN".
//!
//! # The float / Decimal cardinal path
//!
//! HA does **not** inherit `Num2Word_Base.to_cardinal_float`. It overrides
//! `to_cardinal` and handles non-integers inline: a `float` routes to
//! `float_to_words`, a `Decimal` does not (it is not `isinstance(_, float)`),
//! so it falls straight to `_int_to_hausa(Decimal)`. [`Lang::to_cardinal_float`]
//! below reproduces both arms.
//!
//! **The float arm ([`LangHa::float_to_words`]).** Python takes
//! `str(value - int(value))[2:]` — the shortest-round-trip repr of the *binary*
//! fractional remainder, minus its `"0."` prefix — and feeds `int(...)` of that
//! whole string to `to_cardinal`. It is emphatically **not** the `precision`
//! the harness derives: `3.14 - 3 == 0.14000000000000012`, so `3.14` says
//! "uku wajen «14000000000000012 in words»", not "... one four". The f64
//! artefacts are load-bearing and Rust's `f64` `Display` is the same
//! shortest-round-trip contract as Python's `repr`, so `format!("{}", frac)`
//! reproduces the digit string byte for byte (verified for every corpus row).
//! No rounding, no `precision`, no banker's tie — HA's arm never rounds.
//!
//! **The Decimal arm.** `_int_to_hausa(Decimal)` walks the same `divmod`
//! recursion as the integer path, but the leaf lookups `ONES`/`TEENS`/`TENS`
//! are plain dicts with integer keys. A `Decimal` that equals an integer
//! (`Decimal("5") == 5`, same hash) resolves and yields the ordinary cardinal;
//! in Python a **fractional** `Decimal` — and so every numeric string such as
//! "1.5" — reached one of those dicts with a non-integer key and raised
//! `KeyError`. Fixed (gladiaio/num2words2#156): a fractional Decimal now reads
//! like the float, the fractional digits worded as one integer
//! ("ɗaya wajen biyar"), taken exactly from the Decimal with trailing zeros
//! dropped so `Decimal("1.50")` agrees with `1.5`.
//!
//! **Leading zeros.** Both arms word the fractional digits as one integer, and
//! Python's `int()` dropped their leading zeros, so `0.05` read exactly like
//! `0.5` ("sifiri wajen biyar"). Each leading zero is now read as "sifiri"
//! first (gladiaio/num2words2#205): `0.05` is "sifiri wajen sifiri biyar".
//!
//! **Currency fractional cents.** HA's `to_currency` builds a `Decimal` minor
//! unit and calls `to_cardinal(float(minor_units))` → `float_to_words`. The
//! Rust currency code delegates that to `cardinal_from_decimal`, which is
//! overridden here to the same `float_to_words` route (rather than the base
//! float path) so the two agree. No corpus row reaches it — every currency arg
//! has ≤ 2 decimals, so the minor unit is always whole cents.
//!
//! Fraction remains a later phase.

use crate::base::{check_maxval, pow10_big, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::sync::OnceLock;

/// `Num2Word_HA.ONES`, keys 0..=9.
const ONES: [&str; 10] = [
    "sifiri", "ɗaya", "biyu", "uku", "huɗu", "biyar", "shida", "bakwai", "takwas", "tara",
];

/// `Num2Word_HA.TEENS`, keys 10..=19; index here is `n - 10`.
const TEENS: [&str; 10] = [
    "goma",
    "sha ɗaya",
    "sha biyu",
    "sha uku",
    "sha huɗu",
    "sha biyar",
    "sha shida",
    "sha bakwai",
    "sha takwas",
    "sha tara",
];

/// `Num2Word_HA.TENS`, keys 2..=9. Indices 0 and 1 are absent in Python;
/// the `number < 100` branch can only produce `tens` in 2..=9, so the two
/// placeholders are never read. A lookup that *did* reach them would be a
/// `KeyError` in Python — unreachable, hence not modelled as an error.
const TENS: [&str; 10] = [
    "", // absent in Python
    "", // absent in Python
    "ashirin",
    "talatin",
    "arba'in",
    "hamsin",
    "sittin",
    "saba'in",
    "tamanin",
    "casa'in",
];

/// `Num2Word_HA.negword`, assigned by `setup()`. The trailing space is
/// load-bearing: `to_cardinal` concatenates it *unstripped*
/// (`self.negword + self.to_cardinal(abs(value))`), unlike
/// `Num2Word_Base.to_cardinal` which does `"%s " % self.negword.strip()`.
const NEGWORD: &str = "ban ";

/// `sorted(self.SCALE.keys(), reverse=True)` paired with `self.SCALE[k]`.
///
/// Held as decimal-exponent/word pairs so the BigInt keys can be rebuilt
/// without a fixed-width cast: the largest is only 10**12 here, but the
/// *values* fed to `_int_to_hausa` are unbounded.
const SCALE: [(u32, &str); 5] = [
    (12, "tiriliyan"),
    (9, "biliyan"),
    (6, "miliyan"),
    (3, "dubu"),
    (2, "ɗari"),
];

fn pow10(exp: u32) -> BigInt {
    BigInt::from(10u8).pow(exp)
}

/// `minor_units` is a plain `int` on the whole-cents branch of `to_currency`
/// but a `Decimal` on the fractional-cents branch, and Python then branches on
/// `isinstance(minor_units, Decimal)` to decide between `to_cardinal(int)` and
/// `to_cardinal(float(...))`. The two must stay distinguishable.
enum Minor {
    Int(BigInt),
    Dec(BigDecimal),
}

pub struct LangHa {
    /// Precomputed `sorted(SCALE.keys(), reverse=True)` → (value, word).
    scale: Vec<(BigInt, &'static str)>,
    /// `Num2Word_HA.CURRENCY_FORMS`, built once here and never per call.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl LangHa {
    pub fn new() -> Self {
        // Num2Word_HA.CURRENCY_FORMS, in Python's declaration order. Both forms
        // of every entry are identical — HA's own to_currency only ever reads
        // index [0] — but the 2-arity is preserved because `pluralize` and
        // `base.to_cheque` (`cr1[-1]`) index into these tuples.
        let mut currency_forms = HashMap::new();
        currency_forms.insert(
            "NGN",
            CurrencyForms::new(&["naira", "naira"], &["kobo", "kobo"]),
        );
        currency_forms.insert(
            "USD",
            CurrencyForms::new(&["dala", "dala"], &["cent", "cent"]),
        );
        currency_forms.insert(
            "EUR",
            CurrencyForms::new(&["yuro", "yuro"], &["cent", "cent"]),
        );
        currency_forms.insert(
            "GBP",
            CurrencyForms::new(&["fam", "fam"], &["pence", "pence"]),
        );
        currency_forms.insert("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"]));
        currency_forms.insert(
            "CNY",
            CurrencyForms::new(&["yuan", "yuan"], &["fen", "fen"]),
        );

        LangHa {
            scale: SCALE.iter().map(|(e, w)| (pow10(*e), *w)).collect(),
            currency_forms,
        }
    }

    /// `int_to_hausa` behind [`maxval_ceiling`], checked before the first
    /// recursive step (gladiaio/num2words2#203).
    fn checked_int_to_hausa(&self, number: &BigInt) -> Result<String> {
        check_maxval(number, maxval_ceiling())?;
        Ok(self.int_to_hausa(number))
    }

    /// Port of `Num2Word_HA._int_to_hausa`.
    ///
    /// Only ever called with `number > 0` from `to_cardinal` (0 and negatives
    /// are handled by the caller), but the `number == 0` guard is kept because
    /// the recursive calls below are all gated on `remainder > 0` — mirroring
    /// Python exactly rather than reasoning about which arms are live.
    fn int_to_hausa(&self, number: &BigInt) -> String {
        if number.is_zero() {
            return String::new();
        }

        // number < 10 → ONES[number]
        if number < &BigInt::from(10u8) {
            let i = number.to_usize().expect("0 <= number < 10");
            return ONES[i].to_string();
        }

        // number < 20 → TEENS[number]
        if number < &BigInt::from(20u8) {
            let i = number.to_usize().expect("10 <= number < 20") - 10;
            return TEENS[i].to_string();
        }

        // number < 100 → TENS[tens] [+ " da " + ONES[units]]
        if number < &BigInt::from(100u8) {
            let n = number.to_usize().expect("20 <= number < 100");
            let (tens, units) = (n / 10, n % 10);
            let mut result = TENS[tens].to_string();
            if units > 0 {
                result.push_str(" da ");
                result.push_str(ONES[units]);
            }
            return result;
        }

        // number < 1000 → "ɗari" [+ " " + ONES[hundreds]] [+ tail]
        if number < &BigInt::from(1000u16) {
            let n = number.to_usize().expect("100 <= number < 1000");
            let (hundreds, remainder) = (n / 100, n % 100);
            let mut result = if hundreds == 1 {
                // Python hardcodes the literal here rather than SCALE[100].
                "ɗari".to_string()
            } else {
                format!("ɗari {}", ONES[hundreds])
            };

            if remainder > 0 {
                // " da " for a bare unit, plain " " for anything >= 10.
                if remainder < 10 {
                    result.push_str(" da ");
                } else {
                    result.push(' ');
                }
                result.push_str(&self.int_to_hausa(&BigInt::from(remainder)));
            }
            return result;
        }

        // Thousands and above. `number >= 1000` is guaranteed here, and 1000
        // is a SCALE key, so this loop always returns.
        for (scale_value, scale_name) in &self.scale {
            if number >= scale_value {
                let (quotient, remainder) = number.div_mod_floor(scale_value);
                let mut result;

                if scale_value == &BigInt::from(100u8) {
                    // Unreachable: `number < 1000` already returned above, so
                    // the loop matches 1000 (or larger) before ever reaching
                    // the 100 entry. Mirrored from Python for fidelity.
                    result = if quotient.is_one() {
                        "ɗari".to_string()
                    } else {
                        format!("ɗari {}", self.int_to_hausa(&quotient))
                    };
                } else if scale_value == &BigInt::from(1000u16) {
                    result = if quotient.is_one() {
                        "dubu".to_string()
                    } else {
                        format!("dubu {}", self.int_to_hausa(&quotient))
                    };
                } else {
                    // Millions, billions, trillions — and, by recursion on
                    // the quotient, everything above (10**15 → "tiriliyan
                    // dubu", 10**18 → "tiriliyan miliyan", ...).
                    result = if quotient.is_one() {
                        scale_name.to_string()
                    } else {
                        format!("{} {}", scale_name, self.int_to_hausa(&quotient))
                    };
                }

                if remainder.is_positive() {
                    if remainder < BigInt::from(10u8) {
                        result.push_str(" da ");
                    } else {
                        result.push(' ');
                    }
                    result.push_str(&self.int_to_hausa(&remainder));
                }

                return result;
            }
        }

        // Python's `return str(number)` fallback. Unreachable, since the loop
        // above always matches the 1000 entry for `number >= 1000`.
        number.to_string()
    }

    /// Port of `Num2Word_HA.to_cardinal` for a `float` argument.
    ///
    /// Mirrors the guard order of Python's `to_cardinal`: `value == 0` first,
    /// then the sign test (which recurses on `abs`, so the trailing space in
    /// `NEGWORD` is the only separator), then the `isinstance(value, float)`
    /// arm → `float_to_words`.
    fn cardinal_float(&self, value: f64) -> Result<String> {
        if value == 0.0 {
            return Ok(ONES[0].to_string());
        }
        if value < 0.0 {
            // `self.negword + self.to_cardinal(abs(value))` — abs stays a
            // float, so the recursion lands back on `float_to_words`.
            return Ok(format!("{}{}", NEGWORD, self.cardinal_float(value.abs())?));
        }
        self.float_to_words(value)
    }

    /// Port of `Num2Word_HA.float_to_words`. Only reached for a positive,
    /// non-zero `value` (the caller strips zero and the sign first).
    ///
    /// The whole fractional remainder is worded as a **single integer**, not
    /// digit by digit, and it comes from the *binary* subtraction — see the
    /// module docs. `precision` is deliberately never consulted.
    fn float_to_words(&self, value: f64) -> Result<String> {
        // `if value == int(value): return self.to_cardinal(int(value))`.
        let trunc = value.trunc();
        if value == trunc {
            return self.to_cardinal(&BigInt::from(trunc as i128));
        }

        // integer_part = int(value); decimal_part = value - integer_part.
        // int() truncates toward zero; `value > 0` here, so `trunc` == floor.
        let decimal_part = value - trunc;

        // decimal_str = str(decimal_part)[2:] — drop the leading "0.". Rust's
        // f64 Display is the same shortest-round-trip contract as Python repr,
        // so the digit string matches; skipping two `chars()` mirrors `[2:]`.
        let s = format!("{}", decimal_part);
        let decimal_str: String = s.chars().skip(2).collect();

        // result = to_cardinal(integer_part) + " " + pointword + " ".
        let mut result = self.to_cardinal(&BigInt::from(trunc as i128))?;
        result.push(' ');
        result.push_str(self.pointword());
        result.push(' ');

        // decimal_num = int(decimal_str); result += to_cardinal(decimal_num).
        // A non-numeric `decimal_str` (e.g. an exponent form for a sub-1e-4
        // remainder) is Python's `int(...)` ValueError; unreached by the corpus.
        let decimal_num: BigInt = decimal_str.parse().map_err(|_| {
            N2WError::Value(format!(
                "invalid literal for int() with base 10: {:?}",
                decimal_str
            ))
        })?;
        // int() dropped the leading zeros, so 0.05 read like 0.5 (#205).
        result.push_str(&self.leading_zeros(&decimal_str));
        // At most 17 significant digits, so this stays clear of the stack
        // depth the #203 ceiling guards against; read it unchecked.
        result.push_str(&self.int_to_hausa(&decimal_num));
        Ok(result)
    }

    /// One "sifiri " per leading zero of the fractional digit run, which the
    /// Python reading (`int(digits)` worded as one number) dropped, so 0.05
    /// read exactly like 0.5 (gladiaio/num2words2#205).
    fn leading_zeros(&self, digits: &str) -> String {
        let zeros = digits.len() - digits.trim_start_matches('0').len();
        format!("{} ", ONES[0]).repeat(zeros)
    }

    /// Port of `Num2Word_HA.to_cardinal` for a `Decimal` argument.
    ///
    /// A `Decimal` is not a `float`, so Python skips the `float_to_words` arm
    /// and calls `_int_to_hausa(Decimal)`. An integral Decimal resolves through
    /// the integer-keyed `ONES`/`TEENS`/`TENS` dicts (Python hashes
    /// `Decimal("5")` like `5`) and yields the ordinary cardinal; a fractional
    /// Decimal reached a leaf lookup with a non-integer key → `KeyError`; it
    /// now reads like `float_to_words` (module docs, #156).
    fn cardinal_decimal(&self, value: &BigDecimal) -> Result<String> {
        if value.is_zero() {
            return Ok(ONES[0].to_string());
        }
        if value.is_negative() {
            return Ok(format!(
                "{}{}",
                NEGWORD,
                self.cardinal_decimal(&value.abs())?
            ));
        }

        // `with_scale(0)` truncates toward zero (== floor here, value > 0),
        // matching `int(Decimal)`. Integral ⇒ ordinary cardinal; otherwise the
        // recursion would `KeyError` on a fractional dict key.
        let truncated = value.with_scale(0);
        if (value - truncated.clone()).is_zero() {
            Ok(self.checked_int_to_hausa(&truncated.as_bigint_and_exponent().0)?)
        } else {
            // `float_to_words`' shape: the fractional digits as one integer.
            let precision = value.as_bigint_and_exponent().1.max(0) as u32;
            let (pre, mut post) = float2tuple(&FloatValue::Decimal {
                value: value.clone(),
                precision,
            });
            // `post` padded back to `precision` digits keeps its leading
            // zeros (#205).
            let digits = format!("{:0>width$}", post.to_string(), width = precision as usize);
            let ten = BigInt::from(10);
            while !post.is_zero() && (&post % &ten).is_zero() {
                post /= &ten;
            }
            Ok(format!(
                "{} {} {}{}",
                self.to_cardinal(&pre)?,
                self.pointword(),
                self.leading_zeros(&digits),
                self.to_cardinal(&post)?
            ))
        }
    }
}

impl Default for LangHa {
    fn default() -> Self {
        Self::new()
    }
}

/// The exclusive ceiling (gladiaio/num2words2#203): the largest scale word is
/// tiriliyan (10^12), so from 10^15 the module would stack it ("tiriliyan
/// tiriliyan").
/// Without it the recursion never ends and a large enough integer overflows
/// the native stack, killing the Python process with SIGSEGV.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(15))
}

impl Lang for LangHa {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NGN"
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "wajen"
    }

    /// Port of `Num2Word_HA.to_cardinal`.
    ///
    /// Note the order of the Python guards: `value == 0` is checked *before*
    /// the sign test, and the `isinstance(value, float)` test sits after both
    /// (irrelevant here — integer input only).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_zero() {
            return Ok(ONES[0].to_string());
        }

        if value.is_negative() {
            // `self.negword + self.to_cardinal(abs(value))` — concatenated
            // raw, so the space comes from NEGWORD's own trailing byte.
            return Ok(format!("{}{}", NEGWORD, self.to_cardinal(&value.abs())?));
        }

        Ok(self.checked_int_to_hausa(value)?)
    }

    /// Port of `Num2Word_HA.to_ordinal`. No `verify_ordinal` call, so
    /// negatives and zero pass straight through.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("na farko".to_string());
        }
        Ok(format!("na {}", self.to_cardinal(value)?))
    }

    /// `to_ordinal_num`: Python appended English suffixes ("2nd") although
    /// the ordinal is "na biyu". Hausa's written numeric ordinal is not
    /// settled here, so it raises rather than guess (quirk 4, #224).
    fn to_ordinal_num(&self, _value: &BigInt) -> Result<String> {
        Err(crate::lang_rm::unsupported("ha", "to='ordinal_num'"))
    }

    /// Port of `Num2Word_HA.to_year`, a bare delegation to `to_cardinal`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_ordinal(float/Decimal)`: `value == 1` matches numerically (1.0 →
    /// "na farko"); everything else — negatives and fractions included — is
    /// `"na " + to_cardinal(value)` with the value's own grammar
    /// ("na sifiri wajen biyar" for 0.5, "na ban ɗaya" for -1.0).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if i.is_one() {
                return Ok("na farko".to_string());
            }
        }
        Ok(format!("na {}", self.cardinal_float_entry(value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: unsupported, like the integer path.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, _repr_str: &str) -> Result<String> {
        Err(crate::lang_rm::unsupported("ha", "to='ordinal_num'"))
    }

    /// `str_to_number` stays Base's `Decimal(value)`, but HA's `to_cardinal`
    /// then hits Decimal arithmetic on the special values: `Decimal('NaN') <
    /// 0` and `divmod(Decimal('Infinity'), scale)` both raise
    /// **decimal.InvalidOperation**, not the shared sentinels' OverflowError/
    /// ValueError. No digit present → the dispatcher propagates it.
    fn str_to_number(&self, s: &str) -> Result<crate::strnum::ParsedNumber> {
        match crate::strnum::python_decimal_parse(s)? {
            crate::strnum::ParsedNumber::Inf { .. } | crate::strnum::ParsedNumber::NaN => {
                Err(N2WError::Custom {
                    module: "decimal",
                    class: "InvalidOperation",
                    msg: "[<class 'decimal.InvalidOperation'>]".into(),
                })
            }
            other => Ok(other),
        }
    }

    /// The float / Decimal cardinal path. HA overrides `to_cardinal` (not
    /// `to_cardinal_float`) and handles non-integers inline, so this reproduces
    /// that inline behaviour: `float` → `float_to_words`, `Decimal` →
    /// `_int_to_hausa(Decimal)`. `precision_override` is ignored — HA's
    /// `float_to_words` reads `str(value - int(value))`, never `self.precision`.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        match value {
            FloatValue::Float { value, .. } => self.cardinal_float(*value),
            FloatValue::Decimal { value, .. } => self.cardinal_decimal(value),
        }
    }

    /// The fractional-cents entry point HA's `to_currency` delegates to. Python
    /// does `self.to_cardinal(float(minor_units))`, i.e. the *float* arm, so
    /// route through `float_to_words` rather than the base float path. Unreached
    /// by the corpus (all currency args have ≤ 2 decimals).
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = value.to_f64().ok_or_else(|| {
            N2WError::Value(format!("cannot represent {} as f64", value))
        })?;
        self.cardinal_float(f)
    }

    // ---- currency ----------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_HA"
    }

    /// Raw `CURRENCY_FORMS[code]` lookup, i.e. **no** NGN fallback.
    ///
    /// HA's own `to_currency` never routes through here (it falls back to NGN
    /// itself, see [`Lang::to_currency`] below); this feeds `base.to_cheque`,
    /// which does a bare dict lookup and turns the KeyError into
    /// NotImplementedError. Hence `cheque:KWD` raises while `currency:KWD`
    /// happily prints naira.
    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// Port of `Num2Word_HA.pluralize`.
    ///
    /// Returns `forms[0]` — the *first* form, not the plural — and ignores `n`
    /// entirely. Unreached by HA's `to_currency` (which indexes CURRENCY_FORMS
    /// directly) and by `base.to_cheque` (which takes `cr1[-1]`), but it is
    /// public API and overrides the abstract base, so it is ported.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        Ok(forms.first().cloned().unwrap_or_default())
    }

    /// Port of `Num2Word_HA.to_currency`.
    ///
    /// Python's signature is `to_currency(self, value, currency="NGN",
    /// cents=True)` — it accepts neither `separator` nor `adjective`, and
    /// consequently ignores both. See the module docs for the behaviours this
    /// diverges from `Num2Word_Base` on.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // `if currency not in self.CURRENCY_FORMS: currency = "NGN"`.
        let code = if self.currency_forms.contains_key(currency) {
            currency
        } else {
            "NGN"
        };
        let forms = self
            .currency_forms
            .get(code)
            .expect("NGN is always present in CURRENCY_FORMS");

        // `is_negative = value < 0` is read *before* the abs below.
        let is_negative = val.is_negative();

        // `value = abs(value)`, then `decimal_val = Decimal(str(value))`. The
        // abs comes first, so the string handed to Decimal is never signed —
        // taking abs on this side is equivalent. An int and a float differ only
        // in the string Python would produce ("1" vs "1.0"), and Decimal("1")
        // and Decimal("1.0") compare and arithmetise identically here, so the
        // int/float split genuinely does not matter for HA (unlike base).
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.abs()),
            CurrencyValue::Decimal { value: d, .. } => d.abs(),
        };

        // `has_fractional_cents = (decimal_val * 100) % 1 != 0`. The 100 is
        // hardcoded in Python: CURRENCY_PRECISION is never consulted.
        // decimal_val >= 0 here, so truncation == Python's floored `%`.
        let scaled = &decimal_val * BigDecimal::from(100);
        let scaled_trunc = scaled.with_scale(0);
        let has_fractional_cents = &scaled - &scaled_trunc != BigDecimal::zero();

        let major_units: BigInt;
        let minor_units: Minor;

        if cents {
            if has_fractional_cents {
                // major_units = int(decimal_val)
                major_units = decimal_val.with_scale(0).as_bigint_and_exponent().0;
                // minor_units = decimal_val * 100 - (major_units * 100)
                minor_units =
                    Minor::Dec(&scaled - BigDecimal::from(&major_units * BigInt::from(100)));
            } else {
                // cents_value = int(round(value * 100)); divmod(cents_value, 100)
                //
                // Python computes `value * 100` in *float* arithmetic and then
                // rounds. This branch only runs when `Decimal(str(value)) * 100`
                // is already integral, and the double nearest str(value) is
                // within half an ulp, so `round(value * 100)` recovers that
                // integer exactly whenever |cents_value| < 2**51 (~2.2e13 major
                // units) — every realistic input, and all 108 corpus rows.
                // Deriving it from the Decimal keeps the result exact and keeps
                // f64 out of the port. See `concerns` for the >2**51 tail.
                let cents_value = scaled_trunc.as_bigint_and_exponent().0;
                let (maj, min) = cents_value.div_mod_floor(&BigInt::from(100));
                major_units = maj;
                minor_units = Minor::Int(min);
            }
        } else {
            // major_units = int(value); minor_units = 0
            major_units = decimal_val.with_scale(0).as_bigint_and_exponent().0;
            minor_units = Minor::Int(BigInt::zero());
        }

        let mut result: Vec<String> = Vec::new();

        // `if major_units > 0` — a zero major unit contributes nothing, which
        // is what routes 0 into the "sifiri" fallback and 0.01 to a bare
        // "cent ɗaya".
        if major_units.is_positive() {
            // currency_forms[0][0] — the singular form, unconditionally.
            // Python's `if major_units == 1 / else` arms here are identical
            // (module docs, quirk 12); collapsed, output unchanged.
            result.push(format!(
                "{} {}",
                forms.unit[0],
                self.to_cardinal(&major_units)?
            ));
        }

        let minor_is_positive = match &minor_units {
            Minor::Int(i) => i.is_positive(),
            Minor::Dec(d) => d.is_positive(),
        };

        if minor_is_positive {
            let minor_words = match &minor_units {
                // `isinstance(minor_units, Decimal)` -> to_cardinal(float(...)),
                // i.e. HA's float_to_words. Only the fractional-cents branch
                // builds a Decimal, and there minor is 100*frac(decimal_val),
                // which is non-integral by construction — so this always lands
                // on the float path. Deferred to the trait default, which raises
                // NotImplemented; the py shim catches it and falls back to the
                // Python converter. Unreached by the corpus.
                Minor::Dec(d) => self.cardinal_from_decimal(d)?,
                Minor::Int(i) => self.to_cardinal(i)?,
            };
            // currency_forms[1][0] — again the singular form.
            if major_units.is_positive() {
                result.push(format!("da {} {}", forms.subunit[0], minor_words));
            } else {
                result.push(format!("{} {}", forms.subunit[0], minor_words));
            }
        }

        if result.is_empty() {
            // The zero case, with "sifiri" hardcoded rather than to_cardinal(0).
            result.push(format!("{} sifiri", forms.unit[0]));
        }

        if is_negative {
            // `result.insert(0, self.negword)` inserts "ban " *with* its
            // trailing space, and the join below adds a second one. Stripped
            // here so the output has single spaces (module docs, quirk 10).
            result.insert(0, NEGWORD.trim().to_string());
        }

        Ok(result.join(" "))
    }
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use crate::base::N2WError;
    use crate::floatpath::FloatValue;
    use std::str::FromStr;

    /// Drive the float arm the way the binding does. `precision` is passed for
    /// signature fidelity but HA ignores it: `float_to_words` reads
    /// `str(value - int(value))`, never `self.precision`.
    fn f(value: f64, precision: u32) -> String {
        LangHa::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// Drive the Decimal arm — exact arbitrary precision, never an f64 cast.
    fn d(s: &str, precision: u32) -> Result<String> {
        LangHa::new().to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision,
            },
            None,
        )
    }

    /// Every non-integral `"to": "cardinal"` corpus row for `ha` — the rows
    /// that actually reach the float path. The whole fractional remainder is
    /// worded as one integer, straight off the binary `value - int(value)`.
    #[test]
    fn corpus_cardinal_float() {
        let rows: &[(f64, u32, &str)] = &[
            (0.5, 1, "sifiri wajen biyar"),
            (1.5, 1, "ɗaya wajen biyar"),
            (2.25, 2, "biyu wajen ashirin da biyar"),
            (3.14, 2, "uku wajen tiriliyan dubu sha huɗu sha biyu"),
            (0.01, 2, "sifiri wajen sifiri ɗaya"),
            (0.1, 1, "sifiri wajen ɗaya"),
            (0.99, 2, "sifiri wajen casa'in da tara"),
            (1.01, 2, "ɗaya wajen sifiri tiriliyan dubu goma da tara"),
            (12.34, 2, "sha biyu wajen tiriliyan dubu talatin da uku ɗari tara casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara tamanin da shida"),
            (99.99, 2, "casa'in da tara wajen tiriliyan dubu tara ɗari takwas casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara arba'in da tara"),
            (100.5, 1, "ɗari wajen biyar"),
            (1234.56, 2, "dubu ɗari biyu talatin da huɗu wajen tiriliyan dubu biyar ɗari biyar casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari huɗu hamsin da huɗu"),
            // pre == 0 → the sign is prepended by cardinal_float, since
            // int(-0.5) == 0 carries no minus.
            (-0.5, 1, "ban sifiri wajen biyar"),
            (-1.5, 1, "ban ɗaya wajen biyar"),
            (-12.34, 2, "ban sha biyu wajen tiriliyan dubu talatin da uku ɗari tara casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara tamanin da shida"),
            // The f64-artefact pair: 1.005 - 1 == 0.004999999999999893 and
            // 2.675 - 2 == 0.6749999999999998, worded whole.
            (1.005, 3, "ɗaya wajen sifiri sifiri tiriliyan dubu huɗu ɗari tara casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari takwas casa'in da uku"),
            (2.675, 3, "biyu wajen tiriliyan dubu shida ɗari bakwai arba'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da takwas"),
        ];
        for &(v, p, want) in rows {
            assert_eq!(f(v, p), want, "float {}", v);
        }
    }

    /// `int(value) == value` floats take `float_to_words`' short-circuit back to
    /// the integer cardinal — no "wajen".
    #[test]
    fn int_valued_floats() {
        assert_eq!(f(0.0, 1), "sifiri");
        assert_eq!(f(1.0, 1), "ɗaya");
    }

    /// Every `"to": "cardinal_dec"` corpus row for `ha` — Decimal input.
    /// Python raised `KeyError` for all five (fractional Decimal into the
    /// integer-keyed dicts); since #156 they read like a float, from the exact
    /// digits (so without the f64 artefacts of the float arm).
    #[test]
    fn corpus_cardinal_dec() {
        let rows = [
            ("0.01", "sifiri wajen sifiri ɗaya"),
            ("1.10", "ɗaya wajen ɗaya"),
            ("12.345", "sha biyu wajen ɗari uku arba'in da biyar"),
            ("0.001", "sifiri wajen sifiri sifiri ɗaya"),
        ];
        for (s, want) in rows {
            let prec = s.split_once('.').map_or(0, |(_, frac)| frac.len() as u32);
            assert_eq!(d(s, prec).unwrap(), want, "Decimal {s}");
        }
        assert!(d("98746251323029.99", 2).unwrap().ends_with(" wajen casa'in da tara"));
    }

    /// An integral Decimal is not a `KeyError`: its dict keys resolve like the
    /// integer path (`Decimal("5")` hashes like `5`). Not in the corpus, but
    /// pins the integral branch.
    #[test]
    fn integral_decimal_resolves() {
        assert_eq!(d("5", 0).unwrap(), "biyar");
        assert_eq!(d("12.00", 2).unwrap(), "sha biyu");
        assert_eq!(d("0", 0).unwrap(), "sifiri");
    }
}
