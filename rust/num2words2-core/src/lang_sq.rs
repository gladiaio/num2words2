//! Port of `lang_SQ.py` (Albanian).
//!
//! Registry check: `CONVERTER_CLASSES["sq"]` is `lang_SQ.Num2Word_SQ()`, which
//! is the class ported here.
//!
//! Shape: **engine**. `Num2Word_SQ` subclasses `Num2Word_Base` directly and
//! defines `high_numwords`/`mid_numwords`/`low_numwords` + `merge`, so
//! `Num2Word_Base.__init__` builds `self.cards` and `MAXVAL`, and
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean`/`merge`. SQ overrides
//! `to_cardinal` only to short-circuit zero before delegating to `super()`.
//!
//! Inherited from `Num2Word_Base` and left alone by SQ:
//!   * `is_title = False` — so `title()` is the identity and `exclude_title`
//!     (`["e", "presje", "minus"]`) is **dead**. It is still declared below to
//!     mirror the source; it can never fire while `is_title()` stays false.
//!   * `verify_ordinal` — reproduced as [`verify_ordinal`]; only
//!     `to_ordinal_num` calls it (see bug 3).
//!   * `errmsg_toobig` = `"abs(%s) must be less than %s."`, emitted by
//!     `default_to_cardinal` in `base.rs`.
//!
//! # Card table and MAXVAL
//!
//! `set_high_numwords` is overridden but does **not** append an "illion"
//! suffix the way EN does — the words go in verbatim:
//!
//! ```text
//! high = ["trilion", "miliard", "milion"]; max = 3 + 3*3 = 12
//! zip(high, range(12, 3, -3)) -> ("trilion",12), ("miliard",9), ("milion",6)
//! ```
//!
//! So the top card is `10**12` ("trilion") and
//! `MAXVAL = 1000 * 10**12 = 10**15`. Python takes `list(cards.keys())[0]`
//! (insertion order, high words first) which coincides with the maximum key
//! here, so `Cards::highest()` gives the identical answer. Anything `>= 10**15`
//! raises `OverflowError`, which is why `to_cardinal(10**15)` fails while
//! `to_cardinal(10**12)` == "një trilion".
//!
//! Note `10**12` is named "trilion" and `10**9` "miliard" — a long-scale
//! labelling that makes SQ's "trilion" an English *trillion* but its "miliard"
//! an English *billion*. That is what the table says; not a porting slip.
//!
//! # Faithfully reproduced Python bugs / oddities
//!
//! 1. **Hundreds above 100 are two words.** `merge` special-cases only
//!    `cnum == 1` into the fused "njëqind", so 100 -> "njëqind" but
//!    200 -> "dy qind", 555 -> "pesë qind e pesëdhjetë e pesë". Idiomatic
//!    Albanian writes "dyqind"/"pesëqind"; Python does not, so neither do we.
//! 2. **`to_ordinal` is barely ordinal.** Only the 14 keys in `ordinal_forms`
//!    (0..=10, 20, 100, 1000) get real ordinal words. Everything else is just
//!    `"i " + to_cardinal(value)` — e.g. `to_ordinal(11)` ==
//!    "i njëmbëdhjetë" (the bare cardinal), `to_ordinal(30)` == "i tridhjetë".
//!    Also note `ordinal_forms[20]` is "i njezeti" — missing the diaeresis
//!    that the cardinal "njëzet" has. Kept verbatim.
//! 3. **`to_ordinal` accepts negatives; `to_ordinal_num` does not.**
//!    `to_ordinal` never calls `verify_ordinal`, and every negative satisfies
//!    its `value <= 10` branch, whose `.get(value, default)` default is
//!    evaluated eagerly — so `to_ordinal(-1)` == "i minus një". Meanwhile
//!    `to_ordinal_num(-1)` raises `TypeError`. The asymmetry is real.
//! 4. **Dead branches, ported as written.** In `to_ordinal` the
//!    `elif value == 20 / == 100 / == 1000` arms are unreachable (those keys
//!    are in `ordinal_forms`, caught by the first check), and the `value <= 10`
//!    arm can only ever return its default. In `to_ordinal_num` the
//!    `value % 10 == 3` arm and the `else` arm both yield "-ti", so only
//!    "-shi" (1) and "-ri" (2) are ever distinct.
//! 5. **An unknown currency is not an error.** `to_currency` ends with
//!    `else: return self.to_cardinal(val)` instead of raising
//!    `NotImplementedError`, so `to_currency(12.34, "KWD")` yields the bare
//!    number "dymbëdhjetë presje tre katër" — no currency name at all. Worse,
//!    that fallback runs *after* `val = abs(val)`, and the `is_negative` flag
//!    is only consumed in the known-currency branch: `to_currency(-12.34,
//!    "KWD")` silently loses its sign. Both are corpus-confirmed.
//! 6. **`to_currency` ignores `CURRENCY_PRECISION` and `adjective`.** The
//!    divisor is hard-coded to 100, so JPY — a real 0-decimal currency, and
//!    one SQ lists in `CURRENCY_FORMS` — is given "senë" subunits:
//!    `to_currency(12.34, "JPY")` == "dymbëdhjetë jenë, tridhjetë e katër
//!    senë". The `adjective` parameter is accepted and never read.
//! 7. **`cents=False` drops the cents rather than abbreviating them.** SQ
//!    never calls `_cents_terse`; the whole segment is behind
//!    `if cents_value > 0 and cents`, so `to_currency(12.34, "EUR",
//!    cents=False)` == "dymbëdhjetë euro" — the ",34" is simply gone.
//! 8. **`to_cheque` contradicts `to_currency` on which codes exist.** SQ does
//!    not override `to_cheque`, so it uses `Num2Word_Base`'s, which *does*
//!    raise `NotImplementedError` on a missing code. KWD therefore raises for
//!    cheques while quietly degrading to a bare cardinal for currency. Base's
//!    version also reads `CURRENCY_PRECISION` (empty here → 100), which is why
//!    `to_cheque(1234.56, "JPY")` is "... AND 56/100 JENË".
//! 9. **Empty and cent-less currency strings (fixed, #160).** Python's
//!    `to_currency(0)` is `""` (no segment is ever appended), and a Decimal
//!    or string amount takes the `isinstance(val, float)` else-arm that zeroes
//!    the cents, so `Decimal("0.1")` was `""` and `"1.5"` was "një lek". The
//!    port says "zero lekë" for 0 and renders Decimal cents like float ones:
//!    `Decimal("0.1")` == "dhjetë qindarkë", `"1.5"` == "një lek, pesëdhjetë
//!    qindarkë".
//!
//! # Currency shape
//!
//! `to_currency` and `pluralize` are overridden outright — `Num2Word_Base`'s
//! `to_currency`, `parse_currency_parts`, `_money_verbose`, `_cents_verbose`
//! and `_cents_terse` are all bypassed, so the shared `default_to_currency` is
//! not used. `to_cheque` is *not* overridden and rides the base default, which
//! does reach `money_verbose`. `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION`
//! are both left at `Num2Word_Base`'s empty dicts, so their hooks keep the
//! trait defaults.
//!
//! SQ's `to_currency` reaches the float path (`to_cardinal_float`) two ways —
//! the unknown-currency fallback and the fractional-cents branch — so
//! `float2tuple`/`to_cardinal_float` are ported here even though the general
//! float/Decimal *cardinal* surface belongs to a later phase.
//!
//! # Float / Decimal cardinal surface
//!
//! Now wired: [`Lang::to_cardinal_float`] (see the `impl Lang` block) serves
//! the `cardinal`/`cardinal_dec` float and Decimal surfaces. SQ overrides
//! `to_cardinal` (only to short-circuit zero) but *not*
//! `Num2Word_Base.to_cardinal_float` or `float2tuple` (checked against the live
//! class), so its float cardinal path is the base engine's, reached as
//! `SQ.to_cardinal(v)` → `super().to_cardinal(v)` → `to_cardinal_float(v)` for
//! non-integral `v`. The override therefore delegates to
//! `floatpath::default_to_cardinal_float`, the faithful port of that base
//! method, which renders the integer part and each fractional digit back
//! through `self.to_cardinal` (so 0 → "zero"). Verified byte-for-byte against
//! all 24 sq `cardinal`/`cardinal_dec` corpus rows.
//!
//! It must **not** be hung on the private `to_cardinal_decimal` /
//! `to_cardinal_float(f64)` methods below — those exist for the currency
//! fractional-cents branch, where Python casts to `float` first. A Decimal
//! *cardinal* keeps full precision: `num2words(Decimal("98746251323029.99"))`
//! takes base `float2tuple`'s exact Decimal arm (issue #603). Routing it
//! through an `f64` cast reprs the value as `98746251323029.98` and yields
//! "… presje nëntë tetë" instead of the correct "… presje nëntë nëntë".
//! `default_to_cardinal_float` preserves the `FloatValue::Decimal` arm, so the
//! exact digits survive.
//!
//! # Out of scope
//!
//! `to_fraction` is a later phase.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

pub struct LangSq {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS`, built once. Rebuilding it per call is what made an
    /// earlier revision of this port slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

/// `cents_value` in `Num2Word_SQ.to_currency` is a Python `int` on the
/// ordinary path but a `Decimal` on the fractional-cents path, and the code
/// then branches on `isinstance(cents_value, Decimal)` to pick between
/// `to_cardinal` and `to_cardinal_float`. The two are not interchangeable, so
/// the distinction is modelled rather than collapsed to one numeric type.
enum SqCents {
    Int(BigInt),
    Dec(BigDecimal),
}

impl Default for LangSq {
    fn default() -> Self {
        Self::new()
    }
}

/// Port of `Num2Word_Base.verify_ordinal`.
///
/// The float check (`errmsg_floatord`) cannot fire for `BigInt` input, so only
/// the negative check survives. Python raises `TypeError`, not `ValueError`.
fn verify_ordinal(value: &BigInt) -> Result<()> {
    if value.is_negative() {
        return Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            value
        )));
    }
    Ok(())
}

/// Port of `self.ordinal_forms` — the only genuine ordinal words SQ knows.
///
/// Keys are 0..=10, 20, 100, 1000. Every other value falls through to
/// `"i " + to_cardinal(...)`. `to_i64` is the membership test: no key exceeds
/// 1000, so anything that does not fit an `i64` is definitionally absent.
fn ordinal_form(value: &BigInt) -> Option<&'static str> {
    match value.to_i64()? {
        0 => Some("i zeroi"),
        1 => Some("i pari"),
        2 => Some("i dyti"),
        3 => Some("i treti"),
        4 => Some("i katërti"),
        5 => Some("i pesti"),
        6 => Some("i gjashti"),
        7 => Some("i shtati"),
        8 => Some("i teti"),
        9 => Some("i nënti"),
        10 => Some("i dhjeti"),
        // "i njezeti", not "i njëzeti" — verbatim from Python.
        20 => Some("i njezeti"),
        100 => Some("i njëqindi"),
        1000 => Some("i një mijti"),
        _ => None,
    }
}

impl LangSq {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // SQ's set_high_numwords: max = 3 + 3*len(high); zip(high, range(max, 3, -3)).
        // No "illion" suffix is appended — the words are stored as given.
        let high = ["trilion", "miliard", "milion"];
        let max = 3 + 3 * high.len() as i64;
        let mut n = max;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), *word);
            n -= 3;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mijë"),
                (100, "qind"),
                (90, "nëntëdhjetë"),
                (80, "tetëdhjetë"),
                (70, "shtatëdhjetë"),
                (60, "gjashtëdhjetë"),
                (50, "pesëdhjetë"),
                (40, "dyzet"),
                (30, "tridhjetë"),
                (20, "njëzet"),
            ],
        );
        // 20 words -> cards 19 down to 0.
        set_low_numwords(
            &mut cards,
            &[
                "nëntëmbëdhjetë",
                "tetëmbëdhjetë",
                "shtatëmbëdhjetë",
                "gjashtëmbëdhjetë",
                "pesëmbëdhjetë",
                "katërmbëdhjetë",
                "trembëdhjetë",
                "dymbëdhjetë",
                "njëmbëdhjetë",
                "dhjetë",
                "nëntë",
                "tetë",
                "shtatë",
                "gjashtë",
                "pesë",
                "katër",
                "tre",
                "dy",
                "një",
                "zero",
            ],
        );

        // MAXVAL = 1000 * highest card = 1000 * 10**12 = 10**15.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // CURRENCY_FORMS. Every entry is a 2-tuple of 2-tuples; SQ's
        // `pluralize` only ever indexes [0] and [1], but the arity is kept as
        // written. Note "qindarkë" and "paund" repeat across singular and
        // plural, and CHF's plural "franka zvicer" drops the -an — verbatim.
        let mut currency_forms = HashMap::new();
        currency_forms.insert("ALL", CurrencyForms::new(&["lek", "lekë"], &["qindarkë", "qindarkë"]));
        currency_forms.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "centë"]));
        currency_forms.insert("USD", CurrencyForms::new(&["dollar", "dollarë"], &["cent", "centë"]));
        currency_forms.insert("GBP", CurrencyForms::new(&["paund", "paund"], &["peni", "pence"]));
        currency_forms.insert(
            "CHF",
            CurrencyForms::new(&["frank zviceran", "franka zvicer"], &["centim", "centimë"]),
        );
        currency_forms.insert("JPY", CurrencyForms::new(&["jen", "jenë"], &["sen", "senë"]));
        currency_forms.insert("RUB", CurrencyForms::new(&["rubël", "rubla"], &["kopek", "kopekë"]));

        LangSq {
            cards,
            maxval,
            // Dead while is_title() is false, but declared as the source does.
            exclude_title: vec!["e".into(), "presje".into(), "minus".into()],
            currency_forms,
        }
    }

    /// Port of `Num2Word_Base.float2tuple(float(value))`, returning
    /// `(pre, precision, post)`.
    ///
    /// Takes an `f64` because Python's does: every caller reaching it goes
    /// through `float(...)` first, and each step below is sensitive to the
    /// cast. Python stashes `precision` on `self` for `to_cardinal_float` to
    /// read back; it is returned here instead, since nothing else observes
    /// that state and `&self` is shared.
    ///
    /// The arithmetic is deliberately left in `f64` rather than redone in
    /// exact decimal. Python evaluates `abs(value - pre) * 10**precision` on
    /// binary floats and guards it with `abs(round(post) - post) < 0.01`, a
    /// heuristic meant to rescue 1.239999999 into 1.24. When a value's repr
    /// needs all 17 significant digits the binary noise dwarfs that tolerance,
    /// so the `math.floor` arm fires and truncates the last digit instead:
    /// `float2tuple(30.000000000000004)` yields post=3, not 4. Exact decimal
    /// arithmetic would silently "fix" that and diverge from Python.
    ///
    /// Fallible only for the `10**precision` conversion — see below.
    fn float2tuple(&self, value: f64) -> Result<(BigInt, usize, BigInt)> {
        // pre = int(value) — truncates toward zero.
        let pre_f = value.trunc();
        let pre = BigInt::from_f64(pre_f).unwrap_or_else(BigInt::zero);

        // self.precision = abs(Decimal(str(value)).as_tuple().exponent).
        //
        // `str(float)` is Python's shortest round-tripping repr, and Rust's
        // `Display` for f64 is the same shortest-round-trip algorithm, so the
        // digits agree. The formats differ only in that Python switches to
        // exponent notation (`1e-05`) where Rust stays positional
        // (`0.00001`) — and `Decimal("1E-5").as_tuple().exponent` is -5,
        // i.e. exactly the count of fractional digits in the positional form.
        // So counting the digits after '.' reproduces Python's exponent.
        //
        // Deriving this from the value's own `str` is what matters: the exact
        // decimal can be *longer* than the repr (float(Decimal("65.65924651\
        // 349251")) reprs as "65.6592465134925"), and using the longer one
        // would append a spurious trailing digit to the output.
        //
        // Integral floats have no '.' in Rust's output but repr them as "5.0";
        // hence the fallback of 1. (Unreachable: to_cardinal_decimal routes
        // integral values to the int engine, and the fractional-cents branch
        // only ever produces non-integral values. Values >= 1e16, where
        // Python's repr goes exponential, are integral and so equally out of
        // reach.)
        let repr = format!("{}", value);
        let precision = repr.split_once('.').map_or(1, |(_, frac)| frac.len());

        // `10**precision` is an exact Python int that the multiply then has to
        // convert to float, so build it the same way. `powi` would drift by an
        // ulp past 10**22, where powers of ten stop being exactly
        // representable, and — more importantly — would quietly produce `inf`
        // where Python raises.
        //
        // Past 10**308 the int no longer fits a double and CPython's
        // `float * int` raises OverflowError rather than returning inf. That
        // is reachable: repr(5e-324) is "5e-324", so precision is 324 and
        // `to_currency(5e-324, "EUR")` raises instead of answering.
        let scale = match BigInt::from(10).pow(precision as u32).to_f64() {
            Some(s) if s.is_finite() => s,
            _ => {
                return Err(N2WError::Overflow(
                    "int too large to convert to float".to_string(),
                ))
            }
        };
        let post_f = (value - pre_f).abs() * scale;

        // Python's round() is half-even, hence round_ties_even and not round.
        let rounded = post_f.round_ties_even();
        let post = if (rounded - post_f).abs() < 0.01 {
            BigInt::from_f64(rounded)
        } else {
            BigInt::from_f64(post_f.floor())
        };
        // from_f64 only fails on NaN/inf, which a finite `scale` rules out.
        Ok((pre, precision, post.unwrap_or_else(BigInt::zero)))
    }

    /// Port of `Num2Word_Base.to_cardinal_float` (SQ does not override it).
    ///
    /// The `precision=None` parameter is not modelled: no SQ path passes one,
    /// so `float2tuple` always decides it from `str(value)`.
    fn to_cardinal_float(&self, value: f64) -> Result<String> {
        let (pre, precision, post) = self.float2tuple(value)?;

        // post = str(post); post = "0" * (self.precision - len(post)) + post.
        // Python's `"0" * negative` is "", so a too-long post is not padded —
        // and is then read only up to `precision` by the loop below.
        let mut post_s = post.to_string();
        let len = post_s.chars().count();
        if precision > len {
            post_s = "0".repeat(precision - len) + &post_s;
        }

        let mut out = vec![self.to_cardinal(&pre)?];
        // `if value < 0 and pre == 0` — to_cardinal(0) is "zero" with no minus
        // to carry the sign, so -0.5 would otherwise come out positive.
        if value < 0.0 && pre.is_zero() {
            out.insert(0, self.negword().trim().to_string());
        }
        if precision > 0 {
            out.push(self.title(self.pointword()));
        }
        for ch in post_s.chars().take(precision) {
            // Python: `curr = int(post[i])` — one character, always a digit.
            let digit = ch.to_digit(10).expect("float2tuple yields digits only");
            out.push(self.to_cardinal(&BigInt::from(digit))?);
        }
        Ok(out.join(" "))
    }

    /// Port of `Num2Word_SQ.to_cardinal` for the case where Python's argument
    /// is a `float` rather than an `int`.
    ///
    /// `to_cardinal(0.0)` hits SQ's `value == 0` short-circuit. Otherwise
    /// `Num2Word_Base.to_cardinal` runs `assert int(value) == value`: an
    /// integral float passes and drives `splitnum` with a float, which for
    /// SQ's card table yields exactly what the equivalent int would. (The one
    /// place a float would break — the `div == value` tally arm, which does
    /// `div * self.cards[elem]` and would raise TypeError on a float `div` —
    /// needs elem==1, and elem 1 is only reached for values below 2, where
    /// `div == 1` wins first. So it is unreachable.) A non-integral float
    /// trips the assert and is caught into `to_cardinal_float`.
    fn to_cardinal_decimal(&self, value: &BigDecimal) -> Result<String> {
        if value.is_zero() {
            return Ok("zero".to_string());
        }
        let truncated = value.with_scale(0);
        if (value - &truncated).is_zero() {
            return self.to_cardinal(&truncated.as_bigint_and_exponent().0);
        }
        self.to_cardinal_float(value.to_f64().unwrap_or(f64::NAN))
    }
}

impl Lang for LangSq {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ALL"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "presje"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_SQ.merge`.
    ///
    /// The fall-through order is load-bearing: when `cnum == 1` and `nnum` is
    /// a big card (>= 10**6), Python rewrites `ctext` to "një" and keeps
    /// falling, landing in the `nnum > cnum` block — that is what makes
    /// 10**6 render "një milion" rather than "milion".
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext, cnum) = l;
        let (ntext, nnum) = r;
        let mut ctext = ctext.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);
        let twenty = BigInt::from(20);
        let ten = BigInt::from(10);

        if cnum.is_one() {
            if nnum == &thousand {
                return ("një mijë".to_string(), thousand);
            }
            if nnum == &hundred {
                // Fused into one word — but only for 1. See module bug 1.
                return ("njëqind".to_string(), hundred);
            }
            if nnum < &hundred {
                return (ntext.to_string(), nnum.clone());
            }
            ctext = "një".to_string();
        }

        if nnum > cnum {
            // All three arms are the same "%s %s" formatting in Python; kept
            // separate to mirror the source's if/elif chain.
            if nnum >= &million {
                return (format!("{} {}", ctext, ntext), cnum * nnum);
            } else if nnum >= &thousand {
                return (format!("{} {}", ctext, ntext), cnum * nnum);
            } else if nnum == &hundred {
                return (format!("{} {}", ctext, ntext), cnum * nnum);
            }
            // Falls through to the tail return when nnum > cnum but nnum is
            // none of the above (unreachable with SQ's card table).
        }

        if nnum < cnum {
            if cnum >= &thousand {
                return (format!("{} e {}", ctext, ntext), cnum + nnum);
            } else if cnum >= &hundred {
                return (format!("{} e {}", ctext, ntext), cnum + nnum);
            } else {
                if cnum >= &twenty && nnum < &ten {
                    return (format!("{} e {}", ctext, ntext), cnum + nnum);
                }
                // No separator. Unreachable with SQ's card table: every card
                // below 100 is a multiple of ten, so its remainder is < 10.
                return (format!("{}{}", ctext, ntext), cnum + nnum);
            }
        }

        (format!("{} {}", ctext, ntext), cnum + nnum)
    }

    /// Port of `Num2Word_SQ.to_cardinal`: zero short-circuits *before* the
    /// overflow check, everything else goes to `Num2Word_Base.to_cardinal`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_zero() {
            return Ok("zero".to_string());
        }
        default_to_cardinal(self, value)
    }

    /// Port of `Num2Word_SQ.to_ordinal`.
    ///
    /// Note there is no `verify_ordinal` call — negatives are accepted and
    /// produce "i minus ..." (module bug 3).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        if let Some(form) = ordinal_form(value) {
            return Ok(form.to_string());
        }

        // Python: `if value <= 10: return self.ordinal_forms.get(value, "i " +
        // self.to_cardinal(value))`. Every dict key <= 10 was already returned
        // above, so only the default can win — which for value <= 10 means the
        // value is negative. The eager default is why to_cardinal runs at all.
        if value <= &BigInt::from(10) {
            return Ok(format!("i {}", self.to_cardinal(value)?));
        }

        // The `elif value == 20 / 100 / 1000` arms Python writes here are
        // unreachable: all three are ordinal_forms keys. Only the else runs.
        let cardinal = self.to_cardinal(value)?;
        Ok(format!("i {}", cardinal))
    }

    /// Port of `Num2Word_SQ.to_ordinal_num`.
    ///
    /// `verify_ordinal` rejects negatives with `TypeError`, so the operands of
    /// `%` are always non-negative and Python's floor-mod matches `mod_floor`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        verify_ordinal(value)?;

        let m10 = value.mod_floor(&BigInt::from(10));
        let m100 = value.mod_floor(&BigInt::from(100));

        let suffix = if m10 == BigInt::one() && m100 != BigInt::from(11) {
            "-shi"
        } else if m10 == BigInt::from(2) && m100 != BigInt::from(12) {
            "-ri"
        } else if m10 == BigInt::from(3) && m100 != BigInt::from(13) {
            // Same result as the else arm — a no-op branch in the source.
            "-ti"
        } else {
            "-ti"
        };

        Ok(format!("{}{}", value, suffix))
    }

    /// Port of `Num2Word_SQ.to_year`: "same as cardinal for Albanian".
    /// No century splitting, and negatives keep the "minus " prefix rather
    /// than gaining a BC/AD suffix.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    /// `to_ordinal(float/Decimal)` — Python's dict-membership gauntlet.
    ///
    /// `value in self.ordinal_forms` is *numeric* hash equality, so every
    /// whole value equal to a key (5.0, `Decimal("5.00")`, `1E+2`, and
    /// -0.0 == 0) takes the table form; everything else — fractional,
    /// negative, or merely absent — lands on `"i " + self.to_cardinal(value)`
    /// (the `value <= 10` arm's eager `.get` default and the tail `else` are
    /// the same expression). No `verify_ordinal` here, so negatives render
    /// as "i minus ..." and fractions as "i ... presje ...".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            if let Some(form) = ordinal_form(&i) {
                return Ok(form.to_string());
            }
            // Whole but not a key: base to_cardinal routes a whole float
            // through the integer path, so this is `to_cardinal(int)`.
            return Ok(format!("i {}", self.to_cardinal(&i)?));
        }
        // Fractional: `int(value) == value` fails inside base to_cardinal,
        // which falls through to the float grammar. (UFCS: the inherent
        // `LangSq::to_cardinal_float(f64)` — the currency float-cast helper —
        // would shadow the trait method here.)
        Ok(format!("i {}", Lang::to_cardinal_float(self, value, None)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal` first — TypeError
    /// for a fractional value ("Cannot treat float %s as ordinal.") before
    /// the negative check ("Cannot treat negative num %s as ordinal.");
    /// -0.0 passes both (`abs(-0.0) == -0.0`). Then `str(value)` + the
    /// suffix picked by `value % 10` / `value % 100`, which for a whole
    /// float agrees with integer arithmetic ("21.0-shi", "42.0-ri",
    /// "12.0-ti").
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Cannot treat float {} as ordinal.",
                    repr_str
                )))
            }
        };
        if i.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                repr_str
            )));
        }
        let m10 = i.mod_floor(&BigInt::from(10));
        let m100 = i.mod_floor(&BigInt::from(100));
        let suffix = if m10 == BigInt::one() && m100 != BigInt::from(11) {
            "-shi"
        } else if m10 == BigInt::from(2) && m100 != BigInt::from(12) {
            "-ri"
        } else {
            // The `% 10 == 3` arm is "-ti" too — a no-op branch in the source.
            "-ti"
        };
        Ok(format!("{}{}", repr_str, suffix))
    }

    // ---- currency -------------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_SQ"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // `currency_adjective` and `currency_precision` are deliberately left at
    // their trait defaults: SQ declares neither CURRENCY_ADJECTIVES nor
    // CURRENCY_PRECISION, so both inherit Num2Word_Base's empty dict, and
    // `CURRENCY_PRECISION.get(code, 100)` is 100 for every code. That default
    // is what gives `to_cheque(1234.56, "JPY")` its "56/100" (bug 8).

    /// Port of `Num2Word_SQ.pluralize` — singular for 1, plural otherwise.
    ///
    /// Infallible, but the trait returns `Result` because Python's
    /// `Num2Word_Base.pluralize` is abstract and raises NotImplementedError.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        // `if not forms: return ""`
        let Some(first) = forms.first() else {
            return Ok(String::new());
        };
        if forms.len() == 1 {
            return Ok(first.clone());
        }
        // `if len(forms) >= 2:` — always true past the guards above, so
        // Python's trailing `return forms[0]` is unreachable.
        if n.is_one() {
            Ok(first.clone())
        } else {
            Ok(forms[1].clone())
        }
    }

    /// Port of the float / Decimal cardinal path.
    ///
    /// SQ does not override `Num2Word_Base.to_cardinal_float` or `float2tuple`
    /// (verified against the live class), so its float cardinal surface is the
    /// base engine's. `default_to_cardinal_float` is the faithful port of that
    /// base method; it renders the integer part and every fractional digit via
    /// `self.to_cardinal`, dispatching back through SQ's `to_cardinal` override
    /// (so 0 → "zero"). The `precision=` kwarg is threaded through as
    /// `precision_override` per the shared base contract (no sq corpus row
    /// exercises it; note base `float2tuple` recomputes `self.precision` from
    /// `str(value)`, so in Python the kwarg is clobbered — a `floatpath` matter,
    /// identical for every language on the base path).
    ///
    /// Deliberately *not* routed through SQ's private `to_cardinal_decimal` /
    /// `to_cardinal_float(f64)`: those are the currency float-cast path and
    /// would collapse `FloatValue::Decimal`'s exact arm to an `f64`, so a
    /// Decimal cardinal like `98746251323029.99` (issue #603) would lose its
    /// last digit. `default_to_cardinal_float` keeps the Decimal arm exact.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        default_to_cardinal_float(self, value, precision_override)
    }

    /// The hook stands for `self.to_cardinal(float(right))`, which is what
    /// `Num2Word_Base.to_currency` calls on its fractional-cents branch — so
    /// it maps to `to_cardinal_decimal` (SQ's `to_cardinal`), not straight to
    /// `to_cardinal_float`. The two differ only for zero and integral values,
    /// which that branch cannot produce.
    ///
    /// Unreachable for SQ: only `default_to_currency` consumes this hook and
    /// SQ overrides `to_currency` outright. Wired up because the port exists
    /// and this hook means exactly that expression.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        self.to_cardinal_decimal(value)
    }

    /// Port of `Num2Word_SQ.to_currency`, which replaces the base version
    /// wholesale — no `parse_currency_parts`, no `_money_verbose`, no
    /// `_cents_terse`, and no NotImplementedError for unknown codes.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Accepted and never read by SQ's to_currency (bug 6).
        _adjective: bool,
    ) -> Result<String> {
        // Trait hands None when the caller omitted separator=;
        // resolve to this language's own default.
        let separator = separator.unwrap_or(self.default_separator());
        let mut result: Vec<String> = Vec::new();

        // is_negative = val < 0; val = abs(val)
        let is_negative = val.is_negative();
        let val = match val {
            CurrencyValue::Int(v) => CurrencyValue::Int(v.abs()),
            CurrencyValue::Decimal { value: v, is_float, .. } => CurrencyValue::Decimal { value: v.abs(), has_decimal: true, is_float: *is_float },
        };

        let forms = match self.currency_forms.get(currency) {
            Some(forms) => forms,
            // `else: return self.to_cardinal(val)` — bug 5. Note this returns
            // before `is_negative` is ever consulted, and `val` is already
            // abs()'d, so the sign is dropped.
            None => {
                return match &val {
                    CurrencyValue::Int(v) => self.to_cardinal(v),
                    CurrencyValue::Decimal { value: v, .. } => self.to_cardinal_decimal(v),
                };
            }
        };

        // decimal_val = Decimal(str(val)), with val already abs()'d.
        let decimal_val = match &val {
            CurrencyValue::Int(v) => BigDecimal::from(v.clone()),
            CurrencyValue::Decimal { value: v, .. } => v.clone(),
        };
        // has_fractional_cents = (decimal_val * 100) % 1 != 0. The 100 is
        // hard-coded in the source, not CURRENCY_PRECISION (bug 6). decimal_val
        // is non-negative here, so Decimal's truncating `%` needs no floor fix.
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = !(&scaled - scaled.with_scale(0)).is_zero();

        let (whole, cents_value) = match &val {
            // `if isinstance(val, float):`
            CurrencyValue::Decimal { value: _, .. } => {
                if has_fractional_cents {
                    // whole = int(decimal_val)
                    // cents_value = decimal_val * 100 - (whole * 100)
                    let whole = decimal_val.with_scale(0).as_bigint_and_exponent().0;
                    let cv = &scaled - BigDecimal::from(&whole * BigInt::from(100));
                    (whole, SqCents::Dec(cv))
                } else {
                    // whole = int(val)
                    // cents_value = int(round((val - whole) * 100))
                    let whole = decimal_val.with_scale(0).as_bigint_and_exponent().0;
                    // has_fractional_cents is false, so val*100 is an integer
                    // and (val - whole)*100 is exactly one too: round() can see
                    // no tie and truncation is the same answer.
                    let cv = (&decimal_val - BigDecimal::from(whole.clone())) * BigDecimal::from(100);
                    (whole, SqCents::Int(cv.with_scale(0).as_bigint_and_exponent().0))
                }
            }
            // `else: whole = int(val); cents_value = 0` — a true int never
            // shows cents, which is why `1` is "një euro" and `1.0` is too,
            // but for a different reason (its cents_value rounds to 0).
            CurrencyValue::Int(v) => (v.clone(), SqCents::Int(BigInt::zero())),
        };

        // `if whole > 0:` — so 0.01 EUR is "një cent", with no "zero euro".
        if whole.is_positive() {
            result.push(self.to_cardinal(&whole)?);
            result.push(self.pluralize(&whole, &forms.unit)?);
        }

        // `if cents_value > 0 and cents:`
        let cents_positive = match &cents_value {
            SqCents::Int(v) => v.is_positive(),
            SqCents::Dec(v) => v.is_positive(),
        };
        // Python branches on `isinstance(val, float)`: a *float* renders its
        // cents, while a Decimal (what a string input becomes through
        // str_to_number) takes the else-arm — `whole = int(val);
        // cents_value = 0` — so Python's `to_currency("1.5")` is just
        // "një lek" and `Decimal("0.1")` is "". The port renders Decimal
        // cents exactly like float ones (bug 9, fixed).
        if cents_positive && cents {
            if whole.is_positive() {
                // Python collapses what it has so far into one string and
                // glues the separator onto its tail, so the separator lands
                // with no space before it and exactly one after.
                let main_part = result.join(" ");
                result = vec![format!("{}{}", main_part, separator)];
            }
            match &cents_value {
                SqCents::Dec(v) => {
                    // self.to_cardinal_float(float(cents_value)). The float()
                    // cast is load-bearing and not just a type change: it is
                    // what re-derives `precision` from the *repr* of the
                    // double rather than from the Decimal's own scale, which
                    // is both wider (Decimal("0.500") -> 0.5, precision 1) and
                    // sometimes longer than the shortest round-trip form.
                    result.push(self.to_cardinal_float(v.to_f64().unwrap_or(f64::NAN))?);
                    // self.pluralize(int(cents_value), ...) — the *truncated*
                    // cents, so 0.5 cents pluralizes on 0 and takes the plural.
                    let int_cents = v.with_scale(0).as_bigint_and_exponent().0;
                    result.push(self.pluralize(&int_cents, &forms.subunit)?);
                }
                SqCents::Int(v) => {
                    result.push(self.to_cardinal(v)?);
                    result.push(self.pluralize(v, &forms.subunit)?);
                }
            }
        }

        // Python returns "" for a zero amount — nothing was ever appended.
        // Spell it like any other whole amount instead (bug 9, fixed).
        if result.is_empty() {
            result.push(self.to_cardinal(&whole)?);
            result.push(self.pluralize(&whole, &forms.unit)?);
        }

        // `result.insert(0, self.negword.strip())` — a separate join element,
        // so it is "minus dymbëdhjetë euro, ...".
        if is_negative {
            result.insert(0, self.negword().trim().to_string());
        }

        Ok(result.join(" "))
    }

    // `to_cheque` is intentionally not overridden: SQ inherits
    // Num2Word_Base.to_cheque, which `default_to_cheque` already ports. It
    // reaches `currency_forms` (raising NotImplementedError for KWD/BHD/INR/
    // CNY), `currency_precision` (100) and `money_verbose` (-> to_cardinal),
    // all of which are correct for SQ above.
}
