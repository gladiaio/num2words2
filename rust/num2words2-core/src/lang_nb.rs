//! Port of `lang_NO.py` (Norwegian Bokmål) — registry key **"nb"**.
//!
//! `CONVERTER_CLASSES["nb"]` is `lang_NO.Num2Word_NO` (an alias of `"no"`),
//! so this is a port of `Num2Word_NO`, not of any `Num2Word_NB`.
//!
//! Shape: **engine**. `Num2Word_NO(lang_EUR.Num2Word_EUR)` defines
//! `high_numwords` (via EUR's `setup`), `mid_numwords`, `low_numwords` and
//! `merge`, and lets `Num2Word_Base.to_cardinal` drive `splitnum`/`clean`.
//! So `cards`/`maxval`/`merge` are all supplied here and the default
//! `to_cardinal` in `base.rs` runs unchanged.
//!
//! Inheritance chain walked: `Num2Word_NO` → `Num2Word_EUR` → `Num2Word_Base`.
//! (`lang_EU` is Basque and is *not* in this chain despite the similar name.)
//!
//! `set_high_numwords` is redefined verbatim in `lang_NO.py` but is byte-identical
//! to the `Num2Word_EUR` version it shadows, so the long scale applies:
//! `GIGA_SUFFIX="illiard"` at 10^n and `MEGA_SUFFIX="illion"` at 10^(n-3),
//! stepping n by -6. With 100 high words, n runs 603 → 9, giving
//! `cards[10^9]="milliard"` / `cards[10^6]="million"` at the bottom and
//! `cards[10^603]="centilliard"` at the top. Hence `MAXVAL = 1000 * 10^603
//! = 10^606` — the reason values must stay `BigInt`.
//!
//! Inherited unchanged from `Num2Word_Base` (trait defaults are wrong for
//! these, so they are overridden below to match Python):
//!   * `to_splitnum` — reimplemented here as a private helper; `to_year` needs it.
//!   * `inflect`, `verify_ordinal` — private helpers below.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. Every one of the following looks wrong and
//! is exactly what Python emits (each is pinned by a `bench/corpus.jsonl` row):
//!
//! 1. **`to_ordinal` suffix matching is order-dependent and over-eager.**
//!    `to_ordinal` walks `ords_pl` in dict *insertion* order, takes the first
//!    key that is a suffix of the cardinal, and `break`s; then it walks
//!    `ords_sg` (`{"en": "første"}`) the same way. The `ords_sg` pass is not
//!    guarded by whether the `ords_pl` pass already fired, and the source
//!    comment claims splitting the tables "needs to be done separately to not
//!    block 13-19 to_ordinal" — but it does not achieve that:
//!    - `to_ordinal(13)`: "tretten" matches no `ords_pl` key, then hits the
//!      `ords_sg` key "en" → **"trettførste"** (not "trettende").
//!    - `to_ordinal(21)`: "tjueen" → **"tjueførste"**; likewise 31, 71, 81, 91.
//!    Insertion order is therefore load-bearing and [`LangNb::ords_pl`] is a
//!    `Vec`, never a `HashMap`. E.g. "to" precedes "tre"/"tolv", so
//!    `to_ordinal(42)` = "førtito" → **"førtiandre"**.
//! 2. **Ordinals silently pass through above 10^6.** `ords_pl` stops at
//!    "million", and the plural forms "millioner"/"milliarder" that `merge`
//!    produces match no key at all, so `to_ordinal(10**7)` == "ti millioner"
//!    and `to_ordinal(10**9)` == "en milliard" — plain cardinals, no ordinal
//!    marking whatsoever.
//! 3. **`merge` discards `ltext` at exactly 100000.** The `lnum == 100 and
//!    rnum == 1000` arm returns the hard-coded `("hundre tusen", 100000)`,
//!    throwing away the "ett hundre" its own recursion just built. 200000
//!    keeps the general path and yields "to hundre tusen", so the table is
//!    inconsistent by design.
//! 4. **`to_year` is asymmetric about the neuter "ett".** `to_year` builds
//!    "hundre" itself via `to_splitnum` rather than through `merge`, so
//!    `to_year(100)` == "en hundre" while `to_cardinal(100)` == "ett hundre".
//! 5. **`to_year` on negatives is nonsense but well-defined.** Python's floor
//!    `divmod(-44, 100)` == `(-1, 56)`, so `to_year(-44)` ==
//!    **"minus en hundre og femtiseks"**. Reproduced with `div_mod_floor`;
//!    Rust's native `/` and `%` truncate toward zero and would give
//!    `(0, -44)` — a different, wrong answer.
//! 6. **`to_currency`'s "og null øre" strip is hardcoded and separator-blind.**
//!    `Num2Word_NO.to_currency` post-processes Base's output with the literal
//!    `result.replace(" og null øre", "")`. The separator is baked into that
//!    literal, so the strip silently stops working the moment the caller passes
//!    anything but the default `separator=" og"` — verified against the live
//!    interpreter:
//!    - `to_currency(1.0, "NOK")` → "en krone"  *(stripped)*
//!    - `to_currency(1.0, "NOK", separator=" plus")` → "en krone plus null øre"
//!    - `to_currency(1.0, "NOK", cents=False)` → "en krone og 00 øre"
//!      *(terse cents render "00", not "null", so the literal misses)*
//!
//!    It is also currency-blind in the other direction: it only ever matches
//!    "øre", so EUR/USD keep their zero cents — `to_currency(1.0, "EUR")` ==
//!    "en euro og null cent" (corpus-pinned), where NOK would drop the tail.
//!    Ported as a literal `str::replace`, which — like Python's — replaces
//!    *every* occurrence, not just the first.
//! 7. **The inherited currency adjectives are almost all unreachable.**
//!    `CURRENCY_ADJECTIVES` comes from `Num2Word_EUR` and carries 16 codes, but
//!    the adjective is only consulted *after* the `CURRENCY_FORMS` lookup
//!    succeeds, and `Num2Word_NO` narrows that table to NOK/EUR/USD. So only
//!    NOK ("Norwegian") and USD ("US") can ever fire; EUR is not in the
//!    adjective table at all, making `adjective=True` a no-op for it. The full
//!    table is still reproduced — it is what the class actually inherits.
//! 8. **`to_year` on a float ignores the divisor and splits at the decimal
//!    point.** `to_splitnum` starts with `if isinstance(val, float): high, low
//!    = self.float2tuple(val)` — integer part and *fractional digits*, not
//!    `divmod(val, 100)`. So `to_year(100.0)` == "ett hundre hundre" (high is
//!    100, not 1) and `to_year(-1.5)` == "minus en hundre og fem" (high -1,
//!    low 5), while `to_year(Decimal("100"))` takes the `divmod` arm and gives
//!    "en hundre". Both are corpus-pinned and reproduced in
//!    [`Lang::year_float_entry`].
//! 9. **`to_year`'s guard flips semantics between float and Decimal.**
//!    `(val // 100) % 10` *floors* for floats (`-21.0 // 100 == -1.0`, so
//!    -21.0 takes the "hundre" arm → "minus tjueen hundre") but *truncates*
//!    for Decimals (`Decimal("-21.0") // 100 == Decimal("-0")`, guard falsy →
//!    plain cardinal "minus tjueen"). Same number, different words, purely by
//!    input type.
//! 10. **`verify_ordinal` accepts whole floats and `-0.0`.** Both checks are
//!    numeric (`value == int(value)`, `abs(value) == value`), so
//!    `to_ordinal(5.0)` == "femte", `to_ordinal(-0.0)` == "null" (IEEE
//!    `abs(-0.0) == -0.0` is true), while any fractional value raises the
//!    *float* TypeError before a negative one can raise the *negative* one
//!    (-1.5 → errmsg_floatord, -2.0 → errmsg_negord).
//! 11. **`to_ordinal_num` glues "." onto `str(value)` verbatim**, so floats
//!    keep their repr: `to_ordinal_num(5.0)` == "5.0.", `Decimal("5.00")` →
//!    "5.00.", `1e16` → "1e+16.", `-0.0` → "-0.0.".
//!
//! # Currency surface
//!
//! `Num2Word_NO` declares its **own** `CURRENCY_FORMS` class attribute, so it
//! does *not* alias the shared `Num2Word_EUR.CURRENCY_FORMS` dict that
//! `Num2Word_EN.__init__` rewrites in place at import time. Verified against
//! the live interpreter (`Num2Word_NO.CURRENCY_FORMS is
//! Num2Word_EUR.CURRENCY_FORMS` → `False`), so the EUR-mutation trap described
//! in `PORTING_CURRENCY.md` does not apply here: NO sees exactly the three
//! codes its own literal lists, and EUR keeps the un-English `("euro", "euro")`
//! plural. Every other code — GBP, JPY, KWD, BHD, INR, CNY, CHF — raises
//! NotImplementedError, which is what 84 of the 108 corpus currency rows assert.
//!
//! `CURRENCY_PRECISION` is inherited from `Num2Word_Base` and is `{}`, so
//! `.get(code, 100)` is always 100 and [`Lang::currency_precision`]'s default
//! is already exact — it is deliberately **not** overridden. There is therefore
//! no reachable divisor-1000 (KWD/BHD) or divisor-1 (JPY) path in this
//! language: those codes die at the forms lookup first.
//!
//! `_money_verbose`, `_cents_verbose`, `_cents_terse` and `to_cheque` are all
//! `Num2Word_Base`'s, matching the trait defaults, so none are overridden.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Duplicated locally rather than imported from `lang_en` so this module has
/// no cross-language dependency (the registry is generated mechanically).
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    // Python: [u + t for t in tens for u in units] — `tens` is the OUTER loop.
    let mut out: Vec<String> = Vec::new();
    for t in tens {
        for u in units {
            out.push(format!("{}{}", u, t));
        }
    }
    out.reverse();

    const REPLACEMENTS: &[(&str, &str)] = &[
        ("novemn", "noven"),
        ("novemo", "novo"),
        ("octoo", "octo"),
        ("quintd", "quind"),
        ("quintn", "quin"),
        ("quintq", "quinq"),
        ("quints", "quins"),
        ("quintt", "quint"),
        ("quintv", "quinv"),
        ("septenn", "septen"),
        ("septent", "sept"),
        ("sexn", "sen"),
        ("sexs", "ses"),
        ("tresd", "tred"),
        ("tresn", "tren"),
        ("tress", "tres"),
        ("tresv", "trev"),
        ("unno", "uno"),
    ];
    for (k, v) in REPLACEMENTS {
        out = out.iter().map(|o| o.replace(k, v)).collect();
    }
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// Port of `Num2Word_Base.inflect` (the "trivial version").
///
/// `"".split("/")` is `[""]` in Python and `[""]` in Rust, so the empty-text
/// case agrees without a special branch.
fn inflect(value: &BigInt, text: &str) -> String {
    let parts: Vec<&str> = text.split('/').collect();
    if value.is_one() {
        parts[0].to_string()
    } else {
        parts.concat()
    }
}

/// Python `str(float)`/`repr(float)`, used only to build error-message text
/// (`errmsg_floatord`, `errmsg_negord`). The corpora compare exception
/// *types*; the text follows the Python format strings. Whole floats keep
/// their ".0" ("21.0"), |v| >= 1e16 switches to Python's exponent form
/// ("1e+20"), and -0.0 keeps its sign.
fn py_float_str(f: f64) -> String {
    if f.is_nan() {
        return "nan".to_string();
    }
    if f.is_infinite() {
        return if f.is_sign_positive() {
            "inf".to_string()
        } else {
            "-inf".to_string()
        };
    }
    let a = f.abs();
    if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        // Python exponent form: mantissa, 'e', sign, two-digit-minimum exponent.
        let s = format!("{:e}", f);
        if let Some((m, e)) = s.split_once('e') {
            let (sign, digits) = match e.strip_prefix('-') {
                Some(d) => ("-", d.to_string()),
                None => ("+", e.to_string()),
            };
            let digits = if digits.len() < 2 {
                format!("0{}", digits)
            } else {
                digits
            };
            return format!("{}e{}{}", m, sign, digits);
        }
        s
    } else if f.fract() == 0.0 {
        // repr keeps the trailing ".0" that Rust's `{}` would drop.
        format!("{:.1}", f)
    } else {
        format!("{}", f)
    }
}

/// Python `str(value)` for the float-or-Decimal union handed to the hooks.
fn py_num_str(v: &FloatValue) -> String {
    match v {
        FloatValue::Float { value, .. } => py_float_str(*value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// CPython `float_divmod`'s floor-division half (`x // y` for floats).
///
/// NOT `(x / y).floor()`: CPython computes fmod first and derives the
/// quotient from it, with a half-ulp correction. `to_year`'s guard hits this
/// with negative floats (`-21.0 // 100 == -1.0`), which is exactly where
/// floor and truncation part ways.
fn py_f64_floordiv(x: f64, y: f64) -> f64 {
    let m = x % y; // Rust f64 `%` is fmod, same as C's.
    let mut div = (x - m) / y;
    if m != 0.0 && ((y < 0.0) != (m < 0.0)) {
        // CPython also folds the fmod into the divisor's sign here; only
        // the quotient correction matters for `//`.
        div -= 1.0;
    }
    if div != 0.0 {
        let floordiv = div.floor();
        if div - floordiv > 0.5 {
            floordiv + 1.0
        } else {
            floordiv
        }
    } else {
        0.0f64.copysign(x / y)
    }
}

/// CPython `float_divmod`'s modulo half (`x % y` for floats): the result
/// takes the *divisor*'s sign, unlike fmod.
fn py_f64_mod(x: f64, y: f64) -> f64 {
    let mut m = x % y;
    if m != 0.0 {
        if (y < 0.0) != (m < 0.0) {
            m += y;
        }
    } else {
        m = 0.0f64.copysign(y);
    }
    m
}

/// `Num2Word_NO.CURRENCY_FORMS` — the class's own literal, not EUR's dict.
///
/// Three codes, and note the arity: every entry has exactly two forms, and for
/// EUR/USD/øre the two are *identical* ("euro"/"euro", "dollar"/"dollar",
/// "øre"/"øre"). That is not redundancy to collapse — `pluralize` indexes
/// `forms[0 if n == 1 else 1]`, so both slots must exist or the lookup would
/// IndexError. Only NOK inflects: "en krone" / "to kroner".
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert(
        "NOK",
        CurrencyForms::new(&["krone", "kroner"], &["\u{f8}re", "\u{f8}re"]),
    );
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cent"]));
    m.insert(
        "USD",
        CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"]),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited verbatim — `Num2Word_NO` does
/// not define its own. Only NOK and USD are reachable; see module bug (7).
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    [
        ("AUD", "Australian"),
        ("BYN", "Belarusian"),
        ("CAD", "Canadian"),
        ("EEK", "Estonian"),
        ("USD", "US"),
        ("RUB", "Russian"),
        ("NOK", "Norwegian"),
        ("MXN", "Mexican"),
        ("RON", "Romanian"),
        ("INR", "Indian"),
        ("HUF", "Hungarian"),
        ("ISK", "\u{ed}slenskar"),
        ("UZS", "Uzbekistan"),
        ("SAR", "Saudi"),
        ("JPY", "Japanese"),
        ("KRW", "Korean"),
    ]
    .into_iter()
    .collect()
}

pub struct LangNb {
    cards: Cards,
    maxval: BigInt,
    /// Built once in `new()`. Rebuilding per call made an earlier version of
    /// this port 10x slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
    /// Ordered, NOT a map: `to_ordinal` breaks on the first suffix hit and
    /// Python iterates this dict in insertion order. See module bug (1).
    ords_pl: Vec<(&'static str, &'static str)>,
    /// `{"en": "første"}` — applied in a second, unguarded pass.
    ords_sg: Vec<(&'static str, &'static str)>,
    exclude_title: Vec<String>,
}

impl Default for LangNb {
    fn default() -> Self {
        Self::new()
    }
}

impl LangNb {
    pub fn new() -> Self {
        // Num2Word_EUR.setup()
        let lows = ["non", "oct", "sept", "sext", "quint", "quadr", "tr", "b", "m"];
        let units = [
            "", "un", "duo", "tres", "quattuor", "quint", "sex", "septen", "octo", "novem",
        ];
        let tens = [
            "dec",
            "vigint",
            "trigint",
            "quadragint",
            "quinquagint",
            "sexagint",
            "septuagint",
            "octogint",
            "nonagint",
        ];
        let mut high = vec!["cent".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));

        let mut cards = Cards::new();

        // Num2Word_NO.set_high_numwords:
        //   cap = 3 + 6 * len(high)
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**n]     = word + GIGA_SUFFIX  ("illiard")
        //       cards[10**(n-3)] = word + MEGA_SUFFIX  ("illion")
        // len(high) == 100 and range(603, 3, -6) has 100 elements, so zip()
        // consumes both exactly; the `n <= 3` guard mirrors the range bound
        // for any hypothetical length mismatch (zip stops at the shorter).
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}illiard", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illion", word),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "tusen"),
                (100, "hundre"),
                (90, "nitti"),
                (80, "\u{e5}tti"),
                (70, "sytti"),
                (60, "seksti"),
                (50, "femti"),
                (40, "f\u{f8}rti"),
                (30, "tretti"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "tjue",
                "nitten",
                "atten",
                "sytten",
                "seksten",
                "femten",
                "fjorten",
                "tretten",
                "tolv",
                "elleve",
                "ti",
                "ni",
                "\u{e5}tte",
                "syv",
                "seks",
                "fem",
                "fire",
                "tre",
                "to",
                "en",
                "null",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]  → 1000 * 10^603 == 10^606
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // ORDER IS SEMANTICS — do not sort, do not turn into a HashMap.
        let ords_pl: Vec<(&'static str, &'static str)> = vec![
            ("to", "andre"),
            ("tre", "tredje"),
            ("fire", "fjerde"),
            ("fem", "femte"),
            ("seks", "sjette"),
            ("syv", "syvende"),
            ("\u{e5}tte", "\u{e5}ttende"),
            ("ni", "niende"),
            ("ti", "tiende"),
            ("elleve", "ellevte"),
            ("tolv", "tolvte"),
            ("fjorten", "fjortende"),
            ("femten", "femtende"),
            ("seksten", "sekstende"),
            ("sytten", "syttende"),
            ("atten", "attende"),
            ("nitten", "nittende"),
            ("tjue", "tjuende"),
            ("hundre", "hundrede"),
            ("tusen", "tusende"),
            ("million", "millionte"),
        ];
        let ords_sg: Vec<(&'static str, &'static str)> = vec![("en", "f\u{f8}rste")];

        LangNb {
            cards,
            maxval,
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
            ords_pl,
            ords_sg,
            exclude_title: vec!["og".into(), "komma".into(), "minus".into()],
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// The float branch (`errmsg_floatord`) is unreachable here: input is
    /// always an integer `BigInt`. Only the negative branch can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.verify_ordinal` for float/Decimal input, checks in
    /// Python's order:
    ///
    ///   1. `not value == int(value)` -> TypeError(errmsg_floatord)
    ///   2. `not abs(value) == value` -> TypeError(errmsg_negord)
    ///
    /// Both comparisons are *numeric*, so -0.0 passes both (`int(-0.0) ==
    /// -0.0` and `abs(-0.0) == -0.0` in IEEE): `to_ordinal(-0.0)` is "null",
    /// not an error. A negative fractional value (-1.5) fails check 1 first,
    /// so it raises the *float* message, as Python does (module bug 10).
    /// Returns the integral value for the integer-path continuation.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_num_str(value)
                    )))
                } else {
                    Ok(i)
                }
            }
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_num_str(value)
            ))),
        }
    }

    /// Port of `Num2Word_Base.to_splitnum`, restricted to integral input.
    ///
    /// Python tries `high, low = val` first, which raises `TypeError` for an
    /// `int` and falls through to `divmod(val, divisor)` — so the tuple-unpack
    /// path is dead for our inputs and only the `divmod` path is modelled.
    /// `divmod` floors, which is what makes `to_year` on negatives behave the
    /// way it does; see module bug (5).
    #[allow(clippy::too_many_arguments)]
    fn to_splitnum(
        &self,
        val: &BigInt,
        hightxt: &str,
        lowtxt: &str,
        jointxt: &str,
        divisor: &BigInt,
        longval: bool,
        cents: bool,
    ) -> Result<String> {
        let mut out: Vec<String> = Vec::new();
        let (high, low) = val.div_mod_floor(divisor);

        if !high.is_zero() {
            // Python rebinds `hightxt` to the inflected+titled form here, and
            // the `if hightxt:` tests below see the REBOUND value.
            let hightxt = self.title(&inflect(&high, hightxt));
            out.push(self.to_cardinal(&high)?);
            if !low.is_zero() {
                if longval {
                    if !hightxt.is_empty() {
                        out.push(hightxt);
                    }
                    if !jointxt.is_empty() {
                        out.push(self.title(jointxt));
                    }
                }
            } else if !hightxt.is_empty() {
                out.push(hightxt);
            }
        }

        if !low.is_zero() {
            if cents {
                out.push(self.to_cardinal(&low)?);
            } else {
                // Python: "%02d" % low. Unreachable from to_year (cents=True),
                // kept for fidelity. `low` is non-negative (floored divmod with
                // a positive divisor), so zero-padding to width 2 is exact.
                let s = low.to_string();
                out.push(if s.len() < 2 {
                    format!("{:0>2}", s)
                } else {
                    s
                });
            }
            if !lowtxt.is_empty() && longval {
                out.push(self.title(&inflect(&low, lowtxt)));
            }
        }

        Ok(out.join(" "))
    }
}

impl Lang for LangNb {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "NOK"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " og"
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
        "komma"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// Port of `Num2Word_NO.merge`. Arm order is exact: several arms overlap
    /// and only the first match may fire.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(10u8).pow(6);
        let milliard = BigInt::from(10u8).pow(9);

        if lnum.is_one() && rnum < &hundred {
            // 1 + <100: drop the "en" entirely (splitnum always emits it).
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            // Tens+units are written solid: 23 → "tjuetre", 42 → "førtito".
            (format!("{}{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            (format!("{} og {}", ltext, rtext), lnum + rnum)
        } else if rnum > lnum {
            if lnum.is_one() && (rnum == &hundred || rnum == &thousand) {
                // 'hundre'/'tusen' are neuter → the numeral 1 is "ett", not "en".
                (format!("ett {}", rtext), lnum * rnum)
            } else if lnum == &hundred && rnum == &thousand {
                // Hard-coded; discards ltext ("ett hundre"). See module bug (3).
                ("hundre tusen".to_string(), BigInt::from(100000))
            } else if rnum == &million {
                if lnum.is_one() {
                    ("en million".to_string(), lnum * rnum)
                } else {
                    (format!("{} millioner", ltext), lnum * rnum)
                }
            } else if rnum == &milliard {
                if lnum.is_one() {
                    ("en milliard".to_string(), lnum * rnum)
                } else {
                    (format!("{} milliarder", ltext), lnum * rnum)
                }
            } else {
                // Note: 10^12 and up get NO plural form — "to billion", not
                // "to billioner". Only million/milliard are special-cased.
                (format!("{} {}", ltext, rtext), lnum * rnum)
            }
        } else {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        }
    }

    /// Port of `Num2Word_NO.to_ordinal`. See module bugs (1) and (2).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut outword = self.to_cardinal(value)?;
        // 10**6 is "millionte", not "en millionte", which reads as the
        // fraction 1/10**6 (#252).
        if outword == "en million" {
            outword = "million".to_string();
        }

        // Python: outword[: len(outword) - len(key)] + ords_pl[key], then break.
        // `ends_with` guarantees the split point is a char boundary, so the
        // byte-based `strip_suffix` removes exactly the same substring Python's
        // char-based slice would — safe for "åtte"/"første" and friends.
        for (key, val) in self.ords_pl.iter() {
            if let Some(stem) = outword.strip_suffix(*key) {
                outword = format!("{}{}", stem, val);
                break;
            }
        }
        // Second pass is NOT guarded by whether the first one fired.
        for (key, val) in self.ords_sg.iter() {
            if let Some(stem) = outword.strip_suffix(*key) {
                outword = format!("{}{}", stem, val);
                break;
            }
        }
        Ok(outword)
    }

    /// Port of `Num2Word_NO.to_ordinal_num`: `str(value) + "."`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}.", value))
    }

    /// Port of `Num2Word_NO.to_year` (`longval` defaults to True).
    ///
    /// `if not (val // 100) % 10` → plain cardinal when the hundreds digit is
    /// zero (1000, 2000, 2024, 10000, and everything under 100). Otherwise the
    /// "<n> hundre og <n>" form via `to_splitnum`. Both `//` and `%` are
    /// Python's flooring operators — `div_floor`/`mod_floor`, not `/` and `%`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let hundred = BigInt::from(100);
        let ten = BigInt::from(10);
        if value.div_floor(&hundred).mod_floor(&ten).is_zero() {
            return self.to_cardinal(value);
        }
        self.to_splitnum(value, "hundre", "", "og", &hundred, true, true)
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float checks and `to_year`'s float
    // branches become reachable here. `to_cardinal` needs no override: NO
    // inherits base's `assert int(value) == value` routing, which is exactly
    // the trait default (whole -> int path, fractional -> float grammar).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "femte", Decimal("1E+2") ->
    /// "ett hundrede", -0.0 -> "null"); fractional or negative values raise
    /// TypeError (module bug 10). The cardinal of a whole float equals the
    /// cardinal of its integer, so delegating to the BigInt `to_ordinal`
    /// (cardinal + ords_pl/ords_sg suffix scan) is exact.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: verify_ordinal, then `str(value) +
    /// "."` — the repr survives verbatim (module bug 11): 5.0 -> "5.0.",
    /// Decimal("5.00") -> "5.00.", 1e16 -> "1e+16.", -0.0 -> "-0.0.".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        self.verify_ordinal_float(value)?;
        Ok(format!("{}.", repr_str))
    }

    /// `Num2Word_NO.to_year(float/Decimal)` — see module bugs 8 and 9.
    ///
    /// ```python
    /// if not (val // 100) % 10: return self.to_cardinal(val)
    /// return self.to_splitnum(val, hightxt="hundre", jointxt="og",
    ///                         longval=longval)
    /// ```
    ///
    /// The guard's `//` floors for floats but truncates for Decimals, and
    /// `to_splitnum`'s float branch is `float2tuple` (int part / fractional
    /// digits) while its Decimal branch is `divmod(val, 100)` — four distinct
    /// behaviours from two lines of Python, all corpus-pinned:
    /// 100.0 -> "ett hundre hundre", Decimal("100") -> "en hundre",
    /// -1.5 -> "minus en hundre og fem", Decimal("-3.0") -> "minus tre".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                // `(val // 100) % 10` with CPython float_divmod semantics.
                // NaN/inf never reach this entry (the dispatcher keeps them
                // on the Python side), so the guard is finite arithmetic.
                let guard = py_f64_mod(py_f64_floordiv(*f, 100.0), 10.0);
                if guard == 0.0 {
                    // Falsy guard (0.0 or -0.0): plain to_cardinal(val).
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum float branch: high/low from float2tuple, NOT
                // divmod by 100 (module bug 8). hightxt "hundre" is appended
                // whenever high is nonzero (inflect has no "/" to split on);
                // jointxt "og" only joins when low is nonzero; lowtxt is
                // empty and cents=True.
                let (high, low) = float2tuple(value);
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    out.push(self.to_cardinal(&high)?);
                    out.push("hundre".to_string());
                    if !low.is_zero() {
                        out.push("og".to_string());
                    }
                }
                if !low.is_zero() {
                    out.push(self.to_cardinal(&low)?);
                }
                Ok(out.join(" "))
            }
            FloatValue::Decimal { value: d, .. } => {
                // Decimal `//` truncates toward zero and `%` keeps the
                // dividend's sign, so the guard reduces to: trunc(d / 100)
                // not divisible by 10. Truncation commutes with the integer
                // divisor, so trunc(d) / 100 (BigInt `/`, truncating) is
                // exact.
                let pre = d.with_scale(0).as_bigint_and_exponent().0;
                let q: BigInt = &pre / BigInt::from(100);
                if (&q % BigInt::from(10)).is_zero() {
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum Decimal branch: `high, low = val` raises
                // TypeError (Decimal is not iterable), caught -> divmod(val,
                // 100): truncated quotient, remainder with the dividend's
                // sign and the dividend's scale (45.000 keeps exponent -3).
                let low = d.clone() - BigDecimal::from(&q * BigInt::from(100));
                let mut out: Vec<String> = Vec::new();
                if !q.is_zero() {
                    out.push(self.to_cardinal(&q)?);
                    out.push("hundre".to_string());
                    if !low.is_zero() {
                        out.push("og".to_string());
                    }
                }
                if !low.is_zero() {
                    // self.to_cardinal(Decimal): whole -> int path
                    // ("førtifem"), fractional -> float grammar with the
                    // remainder's own scale as precision.
                    let (_, exp) = low.as_bigint_and_exponent();
                    let fv = FloatValue::Decimal {
                        value: low.clone(),
                        precision: exp.unsigned_abs() as u32,
                    };
                    out.push(self.cardinal_float_entry(&fv, None)?);
                }
                Ok(out.join(" "))
            }
        }
    }

    // ---- currency ------------------------------------------------------

    /// The registry key is "nb", but the class behind it is `Num2Word_NO` —
    /// there is no `Num2Word_NB`. The NotImplementedError message interpolates
    /// `self.__class__.__name__`, so it must say "Num2Word_NO". Pinned against
    /// the live interpreter:
    /// `Currency code "GBP" not implemented for "Num2Word_NO"`.
    fn lang_name(&self) -> &str {
        "Num2Word_NO"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    // `currency_precision` is intentionally NOT overridden: CURRENCY_PRECISION
    // is Base's empty dict, so `.get(code, 100)` is always 100 — exactly the
    // trait default. See the module-level "Currency surface" note.

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a single-form entry with `n != 1`
    /// would raise IndexError. All three of NO's entries carry two forms, so
    /// that is unreachable — but it is mapped to `Index` rather than panicking
    /// so the exception *type* survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_NO.to_currency`: delegate to Base, then strip a
    /// zero-øre tail.
    ///
    /// ```python
    /// result = super().to_currency(val, currency=currency, cents=cents,
    ///                              separator=separator, adjective=adjective)
    /// result = result.replace(" og null øre", "")   # do not print "og null øre"
    /// return result
    /// ```
    ///
    /// The literal bakes in the default separator and the NOK subunit, so it is
    /// both separator- and currency-blind; see module bug (6). Nothing else is
    /// touched — a raised NotImplementedError propagates before the strip, so an
    /// unknown code still errors rather than returning a stripped string.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it through this language's own default (" og") to reproduce
        // Python's per-method default before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

        let result = crate::currency::default_to_currency(
            self, val, currency, cents, separator, adjective,
        )?;

        // Python's str.replace is unbounded — every occurrence, not just the
        // first. Rust's str::replace matches that.
        Ok(result.replace(" og null \u{f8}re", ""))
    }
}
