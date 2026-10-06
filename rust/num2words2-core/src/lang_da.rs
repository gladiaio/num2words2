//! Port of `lang_DA.py` (Danish), via its `lang_EUR` → `Num2Word_Base`
//! ancestry. Registry key `"da"` resolves to `Num2Word_DA` (verified in
//! `__init__.py` CONVERTER_CLASSES).
//!
//! Shape: **engine**. `Num2Word_DA.setup` supplies `mid_numwords` /
//! `low_numwords` and inherits `Num2Word_EUR.set_high_numwords`, so
//! `Num2Word_Base.to_cardinal` drives `splitnum`/`clean`/`merge`. Only
//! `merge`, `to_ordinal`, `to_ordinal_num` and `to_year` are overridden.
//!
//! DA keeps EUR's long scale but pluralises both suffixes:
//! `GIGA_SUFFIX = "illiarder"`, `MEGA_SUFFIX = "illioner"`, so the cards
//! are "millioner"/"milliarder"; `merge` strips the plural "er" when the
//! count is exactly one (10^6 → "en million", 2·10^6 → "to millioner";
//! #163, upstream savoirfairelinux/num2words#688). `high` holds
//! 100 stems, so `cap = 3 + 6*100 = 603`, the top card is 10^603
//! ("centilliarder") and `MAXVAL = 1000 * 10^603 = 10^606`.
//!
//! # Cross-call mutable state — IMPORTANT for the dispatcher
//!
//! `Num2Word_DA` carries `self.ordflag`, set to `True` by `to_ordinal`,
//! read by `merge`, and reset to `False` afterwards. It changes `merge`'s
//! `cnum == 1` arm: with the flag set, a leading "et" is dropped even above
//! 10^6, so `to_cardinal(10**6)` == "en million" but the cardinal computed
//! *inside* `to_ordinal(10**6)` is just "millioner" (Python then emits
//! "millionerte"; the port strips the plural, see bug 3 / #172).
//!
//! Two consequences:
//!
//! 1. The Rust path is stateless: [`DaOrd`] is a thin wrapper that runs the
//!    same engine with `ordflag = true`, so `to_ordinal` reproduces the
//!    handshake without a mutable field.
//! 2. **Python leaks the flag on failure.** `to_ordinal` sets `ordflag = True`
//!    *before* `to_cardinal` and only clears it *after* a successful return,
//!    so an `OverflowError` from `to_cardinal(v >= 10**606)` leaves
//!    `ordflag == True` on the shared singleton in `CONVERTER_CLASSES`. Every
//!    later `to_cardinal` on that instance then silently drops the leading
//!    "et"/"en" ("millioner" instead of "en million"). The Rust port cannot
//!    reproduce a poisoned singleton and does not try to; a differential
//!    harness must not compare Python `to_cardinal` output taken *after* an
//!    overflowing `to_ordinal` on the same instance.
//!
//! # Faithfully reproduced Python bugs
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the frozen corpus:
//!
//! 1. **The "et" prefix is computed and then mostly thrown away.** `merge`
//!    unpacks `ctext, cnum, ntext, nnum = curr + next` *first*, then rebinds
//!    the local `next` to an "et"-prefixed copy. `ntext` still holds the
//!    original word, and the final `word = ctext + ntext` uses `ntext`. So
//!    the prefix only survives through the `cnum == 1 → return next` path:
//!    100 → "ethundrede", but 200 → "to" + "hundrede" = "tohundrede", never
//!    "toethundrede". Likewise 1100 builds a discarded "etethundrede".
//! 2. **`merge` has no separator above 100000.** The `cnum >= 1000 and
//!    cnum <= 100000` arm adds "e og ", but at 123000 the guard fails and
//!    *nothing* is appended, so 123456 == "ethundrede og treogtyvetusind"
//!    + "firehundrede og seksoghalvtreds" run together:
//!    "ethundrede og treogtyvetusindfirehundrede og seksoghalvtreds".
//! 3. **Plural millions — fixed (#163).** `MEGA_SUFFIX`/`GIGA_SUFFIX` are
//!    plural, and Python emits 10^6 == "en millioner". The port strips the
//!    "er" after "en" ("en million", "en milliard"), as upstream
//!    savoirfairelinux/num2words#688 does. Ordinals are fixed too (#172):
//!    Python ordinalises the plural ("millionerte"); the port builds on the
//!    singular stem, 10^6 → "millionte", 2·10^6 → "to millionte",
//!    10^9 → "milliardte".
//! 4. **Ordinal suffixes double up.** `to_ordinal` first rewrites a trailing
//!    cardinal via `ords`, *then* appends "te"/"ende" by `value % 100`, with
//!    no check that a rewrite happened. 30 has no `ords` entry ("tredive"
//!    matches no key) so it becomes "tredivete"; 40 → "fyrreende";
//!    50 → "halvtredsende"; 100 → "ethundredete".
//! 5. **The `ords` lookup is a suffix scan over a dict, so insertion order is
//!    load-bearing.** `for key in self.ords: if outword.endswith(key): ...
//!    break` takes the *first* insertion-ordered hit, not the longest. Ported
//!    as an ordered `Vec` — a `HashMap` would be non-deterministic and wrong.
//! 6. **`ords` values for the teens are truncated stems**, e.g.
//!    "tretten" → "trett", relying on the later "ende" append to finish the
//!    word ("trettende"). A teen that skipped the append would emit a stub.
//! 7. **`to_ordinal`'s `or value % 100 == 0` in the `elif` is dead code** —
//!    the `if` above already catches `% 100 == 0`. Kept for fidelity.
//! 8. **`to_year` on negatives floor-divides into nonsense.** Python's `//`
//!    and `%` floor toward -inf, so `to_year(-44)` takes
//!    `divmod(-44, 100) == (-1, 56)` and emits
//!    "minus et hundrede seksoghalvtreds". Reproduced with `div_mod_floor`;
//!    `num-bigint`'s bare `/` and `%` truncate and would give the wrong
//!    answer here.
//! 9. **Negative integers got a double space (fixed, #160).**
//!    `Num2Word_DA.to_currency`'s integer arm takes `minus_str = self.negword`
//!    *raw* and drops it into `"%s %s %s"`, so Python's `to_currency(-12,
//!    "DKK")` is "minus  tolv kroner". The port strips negword, as Base and
//!    the float arm do, and says "minus tolv kroner".
//! 10. **The integer arm silently ignores `adjective=`.** Only Base's float
//!     arm consults `CURRENCY_ADJECTIVES`, so `to_currency(2, "USD",
//!     adjective=True)` is "to dollars" while `to_currency(2.5, "USD",
//!     adjective=True)` is "to US dollars, halvtreds cent".
//! 11. **Floats/Decimals reach `to_ordinal`/`to_ordinal_num` through
//!     `Num2Word_Base.verify_ordinal`** — `not value == int(value)` raises
//!     TypeError(errmsg_floatord) for fractional input, then `not abs(value)
//!     == value` raises TypeError(errmsg_negord) for negative *whole* input
//!     (so `-1.5` gets the float message, `-1.0` the negative one). Both
//!     comparisons are numeric: `-0.0` passes both and ordinalises to
//!     "nulte" / "-0.0te". Whole values then continue on the integer path —
//!     `value % 100` on a non-negative whole float equals the integer mod,
//!     and `str(value)` (the dispatcher's repr) keeps its float/Decimal
//!     spelling: `to_ordinal_num(2.0)` == "2.0en",
//!     `to_ordinal_num(Decimal("5.00"))` == "5.00te".
//! 12. **`to_year`'s guard `(val // 100) % 10` splits float from Decimal.**
//!     Float `//` *floors* (CPython `float_divmod`), so `-21.0 // 100 ==
//!     -1.0` and every negative float in (-10**3, 0) takes the splitnum
//!     path; Decimal `//` *truncates* (`Decimal('-21.0') // 100 ==
//!     Decimal('-0')`), so the same value as a Decimal falls through to
//!     plain `to_cardinal`. Ported with `float_divmod` semantics on the f64
//!     arm and truncating BigInt division on the Decimal arm.
//! 13. **`to_splitnum` mangles floats.** Its float branch calls
//!     `float2tuple(val)` — (integer part, fractional digits) — instead of
//!     divmod by 100, so `to_year(100.0)` is "ethundrede hundrede" (high
//!     part 100, not 1) while `to_year(Decimal('100'))` divmods to
//!     "et hundrede"; `to_year(-1.5)` becomes "minus et hundrede fem"
//!     (high = int(-1.5) = -1, low = 5). The Decimal branch divmods with
//!     truncation, so `Decimal('12345.000')` splits into 123 / 45.000 —
//!     "ethundrede og treogtyve hundrede femogfyrre". Reproduced verbatim.
//!
//! # `CURRENCY_FORMS`: DA is *not* caught by the `lang_EUR` mutation trap
//!
//! `Num2Word_EN.__init__` mutates `Num2Word_EUR.CURRENCY_FORMS` in place, and
//! `__init__.py` instantiates `Num2Word_EN()` at import time, so every class
//! that merely *inherits* that dict sees English's rewrites (EUR pluralised to
//! "euros", ~24 extra codes added). `Num2Word_DA` defines its own
//! `CURRENCY_FORMS` class attribute, which shadows EUR's entirely — confirmed
//! at runtime with `Num2Word_DA.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS`
//! → `False`. So the six codes below are transcribed from the DA source
//! literal, and EN's additions (JPY, KWD, BHD, INR, CNY, CHF, ...) are
//! correctly absent → `NotImplementedError`, as the corpus requires.
//!
//! `CURRENCY_ADJECTIVES` is *not* redefined by DA, so it inherits EUR's
//! (which EN never touches). `CURRENCY_PRECISION` is inherited from
//! `Num2Word_Base` as `{}`, so every code resolves to the default divisor of
//! 100 — DA has no 3-decimal or 0-decimal currency and Base's `divisor == 1`
//! branch is unreachable here. Both are left to the trait defaults.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Inlined rather than imported from `lang_en` to keep this file free of
/// cross-language dependencies (the registry is generated mechanically).
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    // Python: [u + t for t in tens for u in units] — `tens` is the outer loop.
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

/// The suffix table shared by `to_ordinal_num` and its float entry:
///
/// ```python
/// vaerdte = (0, 1, 5, 6, 11, 12)
/// if value % 100 >= 30 and value % 100 <= 39 or value % 100 in vaerdte: "te"
/// elif value % 100 == 2: "en"
/// else: "ende"
/// ```
///
/// `m` is `value % 100`, already reduced by the caller.
fn ordinal_num_suffix(m: &BigInt) -> &'static str {
    let vaerdte = [0i64, 1, 5, 6, 11, 12];
    if (*m >= BigInt::from(30) && *m <= BigInt::from(39))
        || vaerdte.iter().any(|v| *m == BigInt::from(*v))
    {
        "te"
    } else if *m == BigInt::from(2) {
        "en"
    } else {
        "ende"
    }
}

/// `Num2Word_DA.merge`, with `self.ordflag` passed explicitly.
///
/// See the module docs for the `ntext` / `next` aliasing bug this preserves.
fn da_merge(l: (&str, &BigInt), r: (&str, &BigInt), ordflag: bool) -> (String, BigInt) {
    let (ctext_in, cnum) = l;
    let (ntext_in, nnum) = r;

    // Python unpacks `ctext, cnum, ntext, nnum = curr + next` up front. The
    // block below rebinds `next` but NOT `ntext`, so the two diverge.
    let mut ctext = ctext_in.to_string();
    let mut ntext = ntext_in.to_string();
    let mut next_text = ntext_in.to_string();
    let next_num = nnum.clone();

    let ten = BigInt::from(10);
    let hundred = BigInt::from(100);
    let thousand = BigInt::from(1000);
    let hundred_thousand = BigInt::from(100_000);
    let million = BigInt::from(1_000_000);

    if nnum == &hundred || nnum == &thousand {
        // These two return before the "et" prefix is applied.
        if cnum == &ten && nnum == &thousand {
            return ("ti tusind".to_string(), BigInt::from(10_000));
        } else if cnum == &hundred && nnum == &thousand {
            return ("ethundrede tusind".to_string(), BigInt::from(100_000));
        }
        next_text = format!("et{}", ntext_in);
    }

    if cnum.is_one() {
        if nnum < &million || ordflag {
            return (next_text, next_num);
        }
        ctext = "en".to_string();
        // The scale words are stored plural ("millioner"/"milliarder"); a
        // count of exactly one takes the singular: "en million" (#163,
        // upstream savoirfairelinux/num2words#688). Both suffixes end in
        // ASCII "er", so the byte slice is on a char boundary.
        if ntext.ends_with("er") {
            ntext.truncate(ntext.len() - 2);
        }
    }

    let val: BigInt;
    if nnum > cnum {
        if nnum >= &million {
            ctext.push(' ');
        }
        val = cnum * nnum;
    } else {
        if cnum >= &hundred && cnum < &thousand {
            ctext.push_str(" og ");
        } else if cnum >= &thousand && cnum <= &hundred_thousand {
            ctext.push_str("e og ");
        }
        // Python chained comparison: nnum < 10 and 10 < cnum and cnum < 100.
        if nnum < &ten && ten < *cnum && cnum < &hundred {
            if nnum.is_one() {
                ntext = "en".to_string();
            }
            // Python: `ntext, ctext = ctext, ntext + "og"` — RHS evaluated
            // first, so ntext takes the OLD ctext.
            let old_ctext = ctext;
            ctext = format!("{}og", ntext);
            ntext = old_ctext;
        } else if cnum >= &million {
            ctext.push(' ');
        }
        val = cnum + nnum;
    }
    (format!("{}{}", ctext, ntext), val)
}

/// `Num2Word_DA.CURRENCY_FORMS`, transcribed from the class body.
///
/// Safe to read literally: DA shadows `Num2Word_EUR.CURRENCY_FORMS` with its
/// own dict, so English's import-time mutation never reaches it (see module
/// docs). Every entry is a 2-tuple, which makes `pluralize`'s IndexError and
/// the int arm's `len(cr1) > 1` fallback unreachable.
///
/// DA overrides EUR's SEK: `("krona", "kronor")/("öre", "öre")` becomes
/// `("krone", "kroner")/("øre", "øre")` — Danish words for the Swedish
/// currency. Kept as DA has it.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m = HashMap::new();
    m.insert(
        "DKK",
        CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"]),
    );
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cent"]));
    m.insert(
        "USD",
        CurrencyForms::new(&["dollar", "dollars"], &["cent", "cent"]),
    );
    m.insert(
        "GBP",
        CurrencyForms::new(&["pund", "pund"], &["penny", "pence"]),
    );
    m.insert(
        "SEK",
        CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"]),
    );
    m.insert(
        "NOK",
        CurrencyForms::new(&["krone", "kroner"], &["øre", "øre"]),
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged (DA does not
/// redefine it, and EN only ever mutates `CURRENCY_FORMS`).
///
/// 14 of these 16 codes are dead weight: the adjective is only consulted
/// *after* the `CURRENCY_FORMS` lookup succeeds, and DA's table has just six
/// codes. Only USD ("US") and NOK ("Norwegian") are reachable — and only on
/// the float arm, since DA's integer arm ignores `adjective=` (bug 10). The
/// full table is kept because `currency_adjective` is a public hook that
/// mirrors the Python attribute.
fn build_currency_adjectives() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("AUD", "Australian");
    m.insert("BYN", "Belarusian");
    m.insert("CAD", "Canadian");
    m.insert("EEK", "Estonian");
    m.insert("USD", "US");
    m.insert("RUB", "Russian");
    m.insert("NOK", "Norwegian");
    m.insert("MXN", "Mexican");
    m.insert("RON", "Romanian");
    m.insert("INR", "Indian");
    m.insert("HUF", "Hungarian");
    m.insert("ISK", "íslenskar");
    m.insert("UZS", "Uzbekistan");
    m.insert("SAR", "Saudi");
    m.insert("JPY", "Japanese");
    m.insert("KRW", "Korean");
    m
}

pub struct LangDa {
    cards: Cards,
    maxval: BigInt,
    /// `self.ords` as an **ordered** list: `to_ordinal` scans it in Python
    /// dict insertion order and breaks on the first suffix hit.
    ords: Vec<(&'static str, &'static str)>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangDa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangDa {
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

        // Num2Word_EUR.set_high_numwords, with DA's plural suffixes:
        //   cap = 3 + 6*len(high) = 603
        //   zip(high, range(cap, 3, -6))  → 100 pairs, n = 603 .. 9
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // mirrors range(cap, 3, -6) exhausting
            }
            cards.insert(
                BigInt::from(10u8).pow(n as u32),
                format!("{}illiarder", word), // GIGA_SUFFIX
            );
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illioner", word), // MEGA_SUFFIX
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "tusind"),
                (100, "hundrede"),
                (90, "halvfems"),
                (80, "firs"),
                (70, "halvfjerds"),
                (60, "treds"),
                (50, "halvtreds"),
                (40, "fyrre"),
                (30, "tredive"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "tyve", "nitten", "atten", "sytten", "seksten", "femten", "fjorten", "tretten",
                "tolv", "elleve", "ti", "ni", "otte", "syv", "seks", "fem", "fire", "tre", "to",
                "et", "nul",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0] = 1000 * 10**603
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Insertion order matters — see module docs, bug 5.
        let ords: Vec<(&'static str, &'static str)> = vec![
            ("nul", "nul"),
            ("et", "første"), // Python source writes "f\xf8rste"
            ("to", "anden"),
            ("tre", "tredje"),
            ("fire", "fjerde"),
            ("fem", "femte"),
            ("seks", "sjette"),
            ("syv", "syvende"),
            ("otte", "ottende"),
            ("ni", "niende"),
            ("ti", "tiende"),
            ("elleve", "ellevte"),
            ("tolv", "tolvte"),
            ("tretten", "trett"),
            ("fjorten", "fjort"),
            ("femten", "femt"),
            ("seksten", "sekst"),
            ("sytten", "sytt"),
            ("atten", "att"),
            ("nitten", "nitt"),
            ("tyve", "tyv"),
        ];

        LangDa {
            cards,
            maxval,
            ords,
            exclude_title: vec!["og".into(), "komma".into(), "minus".into()],
            // Built once here, never per call: these are Python *class*
            // attributes, so rebuilding them inside `to_currency` would be
            // both wrong in spirit and needlessly slow.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check is vacuous for BigInt;
    /// the negative check raises TypeError (not ValueError).
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
    /// -0.0` and `abs(-0.0) == -0.0` in IEEE): `to_ordinal(-0.0)` is
    /// "nulte", not an error. A negative fractional value (-1.5) fails
    /// check 1 first, so it raises the *float* message, as Python does.
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

    /// `Num2Word_Base.to_splitnum` specialised to DA's only call site:
    /// `to_splitnum(val, hightxt="hundrede", longval=True)` — so
    /// `lowtxt`/`jointxt` are empty, `divisor` is 100 and `cents` is True.
    fn to_splitnum(&self, val: &BigInt) -> Result<String> {
        // Python's divmod floors; -44 → (-1, 56). See module docs, bug 8.
        let (high, low) = val.div_mod_floor(&BigInt::from(100));
        let mut out: Vec<String> = Vec::new();

        // inflect(high, "hundrede"): "hundrede".split("/") has no separator,
        // so both the ==1 and the join branch yield "hundrede" for every
        // value. self.title() is identity while is_title is False.
        let hightxt = "hundrede";

        if !high.is_zero() {
            out.push(self.to_cardinal(&high)?);
            if !low.is_zero() {
                // longval is always True here; jointxt is "" so it is skipped.
                out.push(hightxt.to_string());
            } else {
                out.push(hightxt.to_string());
            }
        }
        if !low.is_zero() {
            out.push(self.to_cardinal(&low)?);
            // lowtxt is "" → the inflect append is skipped.
        }
        Ok(out.join(" "))
    }
}

/// Stateless stand-in for `self.ordflag == True`.
///
/// Runs the identical engine and card table, but `merge` takes the ordinal
/// branch. `to_ordinal` uses this instead of mutating a field.
struct DaOrd<'a> {
    base: &'a LangDa,
}

impl Lang for DaOrd<'_> {
    fn cards(&self) -> &Cards {
        &self.base.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.base.maxval
    }
    fn negword(&self) -> &str {
        "minus "
    }
    fn pointword(&self) -> &str {
        "komma"
    }
    fn exclude_title(&self) -> &[String] {
        &self.base.exclude_title
    }
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        da_merge(l, r, true)
    }
}

impl Lang for LangDa {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "DKK"
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
        "komma"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        da_merge(l, r, false)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        // self.ordflag = True; outword = self.to_cardinal(value); ordflag = False
        let engine = DaOrd { base: self };
        let mut outword = default_to_cardinal(&engine, value)?;

        // First insertion-ordered suffix hit wins, then break.
        for (key, rep) in self.ords.iter() {
            if outword.ends_with(key) {
                // ends_with guarantees a char boundary; keys are ASCII anyway.
                let cut = outword.len() - key.len();
                outword = format!("{}{}", &outword[..cut], rep);
                break;
            }
        }

        // The scale words are stored plural ("millioner"/"milliarder"); the
        // ordinal is built on the singular stem: "millionte", "to
        // milliardte" (#172), the way "tusind" takes "te" directly. Only a
        // value that ends on the scale word itself reaches here with that
        // suffix, and `% 100 == 0` then appends "te".
        if outword.ends_with("illioner") || outword.ends_with("illiarder") {
            outword.truncate(outword.len() - 2);
        }

        // value is non-negative here, so % == mod_floor.
        let m = value.mod_floor(&BigInt::from(100));
        if (m >= BigInt::from(30) && m <= BigInt::from(39)) || m.is_zero() {
            outword.push_str("te");
        } else if m > BigInt::from(12) || m.is_zero() {
            // `|| m.is_zero()` is dead — the arm above already took it.
            outword.push_str("ende");
        }
        Ok(outword)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let m = value.mod_floor(&BigInt::from(100));
        Ok(format!("{}{}", value, ordinal_num_suffix(&m)))
    }

    // ---- float/Decimal entries -------------------------------------------
    //
    // Python's dispatcher hands floats/Decimals straight to the converter
    // methods, so `verify_ordinal`'s float checks and `to_year`'s float
    // branches become reachable here. `to_cardinal` needs no override: DA
    // inherits base's `assert int(value) == value` routing, which is exactly
    // the trait default (whole -> int path, fractional -> float grammar).

    /// `to_ordinal(float/Decimal)`: verify_ordinal, then the integer path.
    /// Whole values ordinalise (5.0 -> "femte", Decimal("1E+2") ->
    /// "ethundredete", -0.0 -> "nulte"); fractional or negative values raise
    /// TypeError (see module docs, bug 11). `value % 100` on a non-negative
    /// whole float/Decimal equals the integer mod, so delegating to the
    /// BigInt `to_ordinal` (ordflag engine + ords scan + suffix) is exact.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `str(value)` + the `% 100` suffix.
    /// The numeric half is Python's `str(value)` (`repr_str`), so
    /// Decimal("5.00") yields "5.00te", 2.0 yields "2.0en", 1e+16 yields
    /// "1e+16te" and -0.0 yields "-0.0te".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        let m = i.mod_floor(&BigInt::from(100));
        Ok(format!("{}{}", repr_str, ordinal_num_suffix(&m)))
    }

    /// `Num2Word_DA.to_year(float/Decimal)` — see module docs, bugs 12/13.
    ///
    /// ```python
    /// if val == 1: return "en"
    /// if not (val // 100) % 10: return self.to_cardinal(val)
    /// return self.to_splitnum(val, hightxt="hundrede", longval=longval)
    /// ```
    ///
    /// The guard's `//` floors for floats but truncates for Decimals, and
    /// `to_splitnum`'s float branch is `float2tuple` (int part / fractional
    /// digits) while its Decimal branch is `divmod(val, 100)` — four
    /// distinct behaviours from three lines of Python, all load-bearing:
    /// 100.0 -> "ethundrede hundrede", Decimal('100') -> "et hundrede",
    /// -1.5 -> "minus et hundrede fem", Decimal('-3.0') -> "minus tre".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                if *f == 1.0 {
                    return Ok("en".to_string());
                }
                // `(val // 100) % 10` with CPython float_divmod semantics.
                // NaN/inf never reach this entry (the dispatcher keeps them
                // on the Python side), so the guard is finite arithmetic.
                let guard = py_f64_mod(py_f64_floordiv(*f, 100.0), 10.0);
                if guard == 0.0 {
                    // Falsy guard (0.0 or -0.0): plain to_cardinal(val).
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum float branch: high/low from float2tuple, NOT
                // divmod by 100 (bug 13). hightxt "hundrede" is appended
                // whenever high is nonzero (inflect has no "/" to split on);
                // lowtxt/jointxt are empty and cents=True.
                let (high, low) = float2tuple(value);
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    out.push(self.to_cardinal(&high)?);
                    out.push("hundrede".to_string());
                }
                if !low.is_zero() {
                    out.push(self.to_cardinal(&low)?);
                }
                Ok(out.join(" "))
            }
            FloatValue::Decimal { value: d, .. } => {
                if *d == BigDecimal::from(1) {
                    return Ok("en".to_string());
                }
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
                    out.push("hundrede".to_string());
                }
                if !low.is_zero() {
                    // self.to_cardinal(Decimal): whole -> int path
                    // ("femogfyrre"), fractional -> float grammar with the
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

    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_one() {
            return Ok("en".to_string());
        }
        // Python: `if not (val // 100) % 10` — floored div, floored mod.
        if value
            .div_floor(&BigInt::from(100))
            .mod_floor(&BigInt::from(10))
            .is_zero()
        {
            return self.to_cardinal(value);
        }
        self.to_splitnum(value)
    }

    // ---- currency -------------------------------------------------------
    //
    // DA overrides only `to_currency` (its integer arm). `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` come from
    // `Num2Word_Base` unchanged, and `pluralize` from `Num2Word_EUR`, so the
    // trait defaults already cover them. `CURRENCY_PRECISION` is Base's empty
    // dict — every code is divisor 100 — so `currency_precision` is left at
    // its default too.

    fn lang_name(&self) -> &str {
        "Num2Word_DA"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// would raise IndexError. Every DA entry is a 2-tuple, making that
    /// unreachable — mapped to `Index` rather than a panic so the exception
    /// type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_DA.to_currency`.
    ///
    /// Python special-cases `isinstance(val, int)` to print the unit with no
    /// cents segment, and hands everything else — including whole floats like
    /// `1.0` — to `super()`. That int/float split is exactly the
    /// [`CurrencyValue`] split, so it maps across directly.
    ///
    /// The `longval=True` parameter in the Python signature is accepted and
    /// then never read; there is nothing to port.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `None` == caller omitted `separator=`, so DA's own default (",")
        // applies. The trait resolves this for the *default* body; an override
        // has to do it itself.
        let separator = separator.unwrap_or(self.default_separator());

        if let CurrencyValue::Int(v) = val {
            // Python wraps the lookup in `except (KeyError, AttributeError)`
            // and hands the call to `super()` on a miss — which repeats the
            // lookup and turns the KeyError into NotImplementedError.
            // Delegating rather than raising here keeps that message coming
            // from a single place. (The AttributeError half is unreachable:
            // `CURRENCY_FORMS` always exists via the MRO.)
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };

            // `minus_str = self.negword if val < 0 else ""` — stripped here,
            // unlike Python, so the `"%s %s %s"` template below yields
            // "minus tolv kroner", not "minus  tolv kroner" (#160).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();
            // Python calls `self.to_cardinal` here, not `self._money_verbose`
            // — identical for DA, which does not override `_money_verbose`.
            let money_str = self.to_cardinal(&abs_val)?;

            // `cr1[0]` when abs_val == 1, else `cr1[1] if len(cr1) > 1 else
            // cr1[0]`. Hand-inlined in Python rather than routed through
            // `pluralize`, though it agrees with EUR's rule for 2-tuples.
            let cr1 = &forms.unit;
            let currency_str = if abs_val.is_one() {
                cr1.first()
            } else {
                cr1.get(1).or_else(|| cr1.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            // NOTE: `adjective` is deliberately not consulted on this arm —
            // Python's integer branch never reads CURRENCY_ADJECTIVES. See
            // module docs, bug 10. (It *is* honoured on the float arm below.)
            //
            // Python: ("%s %s %s" % (...)).strip().
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats/Decimals: `super(Num2Word_DA, self).to_currency(...)`, which
        // is `Num2Word_Base.to_currency` (EUR defines none). This is the arm
        // that honours `adjective=` and renders the cents segment.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
