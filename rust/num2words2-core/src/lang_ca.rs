//! Port of `lang_CA.py` (Catalan), registered by `CONVERTER_CLASSES["ca"]`
//! as `Num2Word_CA`.
//!
//! Shape: **engine**. `Num2Word_CA` subclasses `Num2Word_EUR` →
//! `Num2Word_Base`, defines `high_numwords`/`mid_numwords`/`low_numwords` plus
//! `merge`, and lets the base `to_cardinal` drive `splitnum`/`clean`. It then
//! overrides `to_ordinal` / `to_ordinal_num` with hand-rolled algorithms.
//!
//! Inheritance chain actually walked:
//!   * `Num2Word_EUR.set_high_numwords` — pairs each high word with
//!     `range(3 + 6*len(high), 3, -6)`. CA sets `GIGA_SUFFIX = None` and
//!     `MEGA_SUFFIX = "ilió"`, so **only** the MEGA half fires and the cards
//!     land at 10^24/10^18/10^12/10^6. There is deliberately **no 10^9 card**
//!     — a milliard is spelled compositionally as "mil milions".
//!   * `Num2Word_EUR.gen_high_numwords([], [], lows)` — with empty units and
//!     tens the comprehension yields `[]`, so the Latin elision table is dead
//!     code here and the result is just `lows == ["quadr","tr","b","m"]`.
//!   * `Num2Word_Base.to_year(value)` → `self.to_cardinal(value)`; CA does not
//!     override it, so years are plain cardinals ("menys cinc-cents" for -500).
//!   * `Num2Word_Base.verify_ordinal` — negatives raise `TypeError`.
//!
//! `MAXVAL = 1000 * 10^24 = 10^27`; `to_cardinal(10^27)` raises OverflowError.
//!
//! # Faithfully reproduced Python bugs
//!
//! Verified against the interpreter; all of these are wrong-looking but real.
//!
//! 1. **`ords_3` has no key 0**, so every exact multiple of ten from 30 to 90
//!    raises `KeyError`: `to_ordinal(30)`, `(40)`, … `(90)` all blow up, while
//!    `to_ordinal(31)` == "trenta-unè" works. 30 dies in the `value <= 30` arm
//!    (`frac = 30 % 10 == 0`), 40–90 in the `value < 100` arm
//!    (`value - dec == 0`). See [`LangCa::ords_3`].
//! 2. **Missing space in the `< 1e18` arm.** The format is
//!    `"%s%s%s %s" % (cardinal, ords[dec], gender_stem, ...)` with no
//!    separator between `cardinal` and `ords[dec]`, so
//!    `to_ordinal(9999999)` == "nou**milionè** nou-cents…" and
//!    `to_ordinal(123456789)` == "cent vint-i-tres**milionè** quatre-cents…".
//!    The sibling `< 1e6` arm *does* include the space ("dotze mil …").
//! 3. **`value >= 1e18` chops a character and glues on "onè".**
//!    `part1[:-1] + "onè"` turns "un trilió" into "un trilionè" (fine) but
//!    "mil trilions" into "mil trilion**onè**" and
//!    `to_cardinal(10^27 - 1)` into a string ending "…noranta-no**onè**".
//! 4. **`int(math.log(v, 1000))` overshoots on two float-rounding windows.**
//!    `ln` is computed in `f64`, and for v just below 10^15 or 10^18 the
//!    result rounds up to exactly `5.0` / `6.0`:
//!    * v ∈ [999999999999996, 999999999999999] → `dec = 10^15 > v`, so
//!      `divmod` gives `high_part = 0`, `low_part = v`, and
//!      `to_ordinal(low_part)` recurses on the *same value* forever →
//!      Python raises **RecursionError**. See [`RECURSION_LIMIT`].
//!    * v ∈ [999999999999995072, 10^18) → `dec = 10^18`, which is absent from
//!      `ords` → **KeyError**. This is why `to_ordinal(10^18 - 1)` raises.
//!    [`log1000_trunc`] reproduces the float computation rather than taking an
//!    exact integer logarithm, precisely so these windows survive.
//! 5. **`to_ordinal(0)` returns the empty string** in Python, because the
//!    first arm sets `text = ""` and the function ends with `.strip()`. The
//!    public `to_ordinal` raises ValueError for 0 instead (#160); the inner
//!    recursion keeps the `""`. `to_ordinal(200)` == "dos-cents" (no ordinal suffix at all) for the same
//!    reason: the trailing `to_ordinal(0)` contributes nothing and `.strip()`
//!    eats the space.
//! 6. `to_ordinal(1234567890)` raises `KeyError` — it reaches
//!    `to_ordinal(90)`, which is bug 1.
//! 7. **Negative integer currency amounts got a double space** (fixed,
//!    #160), and `adjective=True` is ignored for ints but honoured for floats — CA's
//!    `to_currency` handles ints itself and delegates floats to the base
//!    class, and the two arms disagree. See [`LangCa::to_currency`].
//! 8. **`CURRENCY_FORMS` typos are load-bearing output**: `SLL` is
//!    `("leonE", "leones")` and `STD` is `("dobra", "dobrAs")` — mid-word
//!    capitals in the *source dict*, so `to_currency(1, "SLL")` really is
//!    "un leonE". `ALL`'s singular is misspelt `"qqindarka"`. Copied verbatim.
//! 9. **`GBP`/`INR`/… take masculine "un" against a feminine noun**
//!    ("un lliura", "un rupia"): the int arm renders the number with plain
//!    `to_cardinal`, which has no gender agreement. In the corpus as-is.
//! 10. **CA's `to_cardinal_float` un-fixes issue #603.** It calls
//!    `self.float2tuple(float(value))`, float-casting Decimal input that
//!    base.py deliberately keeps exact, so `Decimal("98746251323029.99")`
//!    renders "…coma noranta-vuit" (98) rather than 99, and
//!    `Decimal("1.10")` gives "un coma un" rather than "un coma un zero".
//!    Both are frozen in the corpus. See [`LangCa::to_cardinal_float`].
//! 11. **A fraction of exactly zero can leave a dangling "coma".** The
//!    pointword is appended unconditionally (base guards it with
//!    `if self.precision:`), while the fraction's own word is guarded by
//!    `int(post_str) > 0 or leading_zeros == 0`. At precision 0 — reachable
//!    for 17-significant-digit doubles like `1.2345678901234568e16` — both
//!    combine into "…cinc-cents seixanta-vuit coma" with nothing after it.
//!
//! # Error variants
//!
//! `KeyError` → [`N2WError::Key`], `TypeError` → [`N2WError::Type`],
//! `OverflowError` → [`N2WError::Overflow`].
//!
//! **RecursionError has no matching `N2WError` variant.** The four values in
//! bug 4's first window are mapped to [`N2WError::Value`] by the depth guard in
//! [`LangCa::to_ordinal_inner`]. That is knowingly the wrong exception *type*;
//! it is chosen because the faithful alternative — unbounded recursion — would
//! abort the process on stack overflow rather than return. Flagged in the port
//! report.

use crate::base::{clean, splitnum, Cards, Lang, N2WError, Node, Result};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;
use std::str::FromStr;

/// `self.gender_stem`. CA hardcodes "è"; `to_ordinal_num`'s
/// `"è" if self.gender_stem == "è" else "a"` therefore always picks "è".
const GENDER_STEM: &str = "è";

/// `self.negword`, kept with its trailing space exactly as Python sets it.
/// `to_cardinal` uses `negword.strip()` + `" "`, so the space is cosmetic.
const NEGWORD: &str = "menys ";

/// `Num2Word_CA.to_currency`'s own default `separator=" amb"`.
///
/// See [`SEPARATOR_UNSET`] for why this cannot simply be a parameter default.
const SEPARATOR_DEFAULT: &str = " amb";

/// The separator the pyo3 binding passes when the Python caller omitted one.
///
/// `Num2Word_CA.to_currency` declares `separator=" amb"`, but the `Lang` trait
/// has no per-language defaults: `__init__.py`'s fast path (and `diff_test.py`)
/// substitute `kwargs.get("separator", ",")` — **`Num2Word_Base`'s** default —
/// before the value ever reaches Rust. By then "caller omitted separator" and
/// "caller explicitly passed a comma" are the same string, and the information
/// needed to tell them apart no longer exists on this side of the boundary.
///
/// So `,` is read back as the unset sentinel and CA's own default restored.
/// This is the only reading that matches the oracle: all 63 float rows of the
/// `ca` currency corpus were generated by `num2words(v, lang="ca",
/// to="currency", currency=c)` with no `separator=`, and every one of them
/// expects " amb".
///
/// The cost is narrow and known: a caller who *explicitly* passes
/// `separator=","` gets " amb" here where Python would give ",". Fixing that
/// properly needs `Option<&str>` in the trait signature, which lives in
/// `base.rs` — outside this port's remit. Flagged in the port report.
const SEPARATOR_UNSET: &str = ",";

/// Depth at which we stand in for CPython's RecursionError.
///
/// Python's default limit is 1000 frames and the ordinal recursion is only
/// ever unbounded for the four values in module-doc bug 4. The exact frame
/// count is not observable through the corpus (only the exception type is), so
/// any limit comfortably above the deepest legitimate nesting (~4) works.
const RECURSION_LIMIT: u32 = 900;

/// Python's `int(math.log(int(value), 1000))`.
///
/// CPython's `loghelper` converts the int with `PyLong_AsDouble` (round to
/// nearest) and calls C `log`; the two-argument form then divides by
/// `log(1000.0)`. Both call sites are guarded by `value < 1e18`, so `to_u64`
/// is lossless and `as f64` performs exactly `PyLong_AsDouble`'s rounding.
///
/// This deliberately does **not** compute an exact integer logarithm: the f64
/// round-off is load-bearing (module-doc bug 4).
fn log1000_trunc(value: &BigInt) -> u32 {
    // math.log(1000) == 0x1.ba18a998fffa0p+2; this literal round-trips to the
    // same bits under Rust's correctly-rounded float parsing.
    const LN_1000: f64 = 6.907755278982137_f64;
    let x = value
        .to_u64()
        .expect("log1000_trunc call sites are guarded by value < 1e18")
        as f64;
    // Python's int() truncates toward zero; the ratio is >= 1 here, so
    // truncation and floor agree.
    (x.ln() / LN_1000).trunc() as u32
}

fn pow10(e: u32) -> BigInt {
    BigInt::from(10u8).pow(e)
}

/// Python's `abs(Decimal(str(value)).as_tuple().exponent)` for an `f64` — the
/// precision `Num2Word_Base.float2tuple` derives from `repr` and assigns to
/// `self.precision` on every call.
///
/// CA needs its own copy because it float-casts **Decimal** input too (see
/// [`LangCa::to_cardinal_float`]), so the precision carried on
/// `FloatValue::Decimal` — `abs(exponent)` of the *exact* decimal — is not the
/// one Python ends up reading. `floatpath` has an equivalent helper but it is
/// private, and it counts the digits of `format!("{}", f)`, which disagrees
/// with `repr` at both ends of the range:
///
/// * `repr(5.0)` is `"5.0"` → precision **1**, but `format!("{}", 5.0)` is
///   `"5"` → 0. Not academic: `to_cardinal_float(5.0)` is "cinc coma zero",
///   and at precision 0 this port would emit a dangling "cinc coma".
/// * `repr(1e16)` is `"1e+16"` → `Decimal("1e+16").as_tuple().exponent == 16`
///   → precision **16**, but `format!("{}", 1e16)` is plain digits → 0. Python
///   really does say "deu mil bilions coma" followed by fifteen "zero".
///
/// CPython's `float_repr_style='short'` formats with David Gay's shortest
/// `dtoa` (a digit string plus `decpt`, the decimal point's position), then
/// picks fixed notation iff `-4 < decpt <= 16`, and in fixed notation appends
/// ".0" when no fractional digits were left. Rust's `{:e}` is the same
/// shortest-round-trip digit string, so `decpt == exp + 1` recovers Gay's.
/// `Decimal(repr(f)).as_tuple().exponent` is then `-1` for that ".0" case and
/// `decpt - ndigits` in every other, scientific notation included.
///
/// Note precision 0 *is* reachable, via the scientific arm: `1.2345678901234568e16`
/// has 17 digits at `decpt == 17`. Python then prints a bare trailing "coma";
/// see [`LangCa::to_cardinal_float`].
///
/// Verified against the interpreter over 4035 values — every case in this
/// module's tests plus 4000 random ones spanning 1e-30..1e30, both signs — with
/// no mismatch.
fn float_repr_precision(f: f64) -> u32 {
    // `{:e}` always emits the exponent, except for inf/nan; callers guard those
    // (Python raises out of `int(value)` first), so the fallback is unreachable.
    let s = format!("{:e}", f);
    let (mantissa, exp) = match s.split_once('e') {
        Some(parts) => parts,
        None => return 0,
    };
    let exp: i32 = match exp.parse() {
        Ok(e) => e,
        Err(_) => return 0,
    };
    // The sign and the point are formatting; only the digits count.
    let ndigits = mantissa.chars().filter(|c| c.is_ascii_digit()).count() as i32;
    let decpt = exp + 1;
    if decpt > -4 && decpt <= 16 && decpt >= ndigits {
        // Fixed notation with nothing after the point: repr appends ".0", so
        // the Decimal exponent is -1 rather than 0.
        1
    } else {
        (decpt - ndigits).unsigned_abs()
    }
}

/// Python's `float(value)` on a `Decimal`.
///
/// Goes through the decimal string rather than `BigDecimal::to_f64`: that
/// method trims the significand to ~25 digits before dividing, so it is only
/// approximately rounded, whereas `Decimal.__float__` (libmpdec) and Rust's
/// `f64::from_str` are both *correctly* rounded to the nearest double. They
/// agree on every corpus value either way, but only the string route is
/// guaranteed to.
fn decimal_to_f64(value: &BigDecimal) -> f64 {
    // BigDecimal's Display emits plain or `E`-notation decimal, both of which
    // Rust's float parser accepts; a value too large for f64 parses to
    // infinity, exactly as Python's float(Decimal) overflows to inf.
    f64::from_str(&value.to_string()).unwrap_or(f64::NAN)
}

pub struct LangCa {
    cards: Cards,
    maxval: BigInt,
    /// `self.ords`. Python mixes int keys (1..20, 30..90, 100..900) with float
    /// keys (1e3, 1e6, 1e9, 1e12, 1e15); dict lookup matches an int `dec`
    /// against a float key by numeric equality, and every such key is exactly
    /// representable, so a single integer-keyed map is equivalent.
    ords_tbl: HashMap<u128, &'static str>,
    exclude_title: Vec<String>,
    /// `Num2Word_CA.CURRENCY_FORMS`. CA *replaces* `Num2Word_EUR`'s dict
    /// wholesale — plain class-attribute shadowing, no merge anywhere in the
    /// chain — so EUR's entries are entirely absent. That is observable:
    /// EUR spells `PLN` with three forms `("zloty","zlotys","zlotu")` and
    /// `EUR` as `("euro","euro")`, whereas CA has two-form `("zloty","zlotys")`
    /// and `("euro","euros")` — hence "zero euros", not "zero euro".
    /// All 166 CA entries are 2+2, so `pluralize` never indexes past [1].
    ///
    /// Built once here rather than per call, as the porting contract requires.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `Num2Word_CA.CURRENCY_ADJECTIVES` — *not* defined by CA, so this is
    /// `Num2Word_EUR`'s dict inherited verbatim (English adjectives on a
    /// Catalan converter: `to_currency(100.0, "USD", adjective=True)` gives
    /// "cent US dòlars amb zero centaus"). Reproduced, not corrected.
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangCa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangCa {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords(["quadr", "tr", "b", "m"]).
        // cap = 3 + 6*4 = 27; zip(high, range(27, 3, -6)) -> 27, 21, 15, 9.
        // GIGA_SUFFIX is None so the 10**n insert is skipped entirely;
        // MEGA_SUFFIX = "ilió" gives cards[10**(n-3)].
        let high = ["quadr", "tr", "b", "m"];
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(pow10((n - 3) as u32), format!("{}ilió", word));
            n -= 6;
        }

        for (k, v) in [
            (1000i64, "mil"),
            (100, "cent"),
            (90, "noranta"),
            (80, "vuitanta"),
            (70, "setanta"),
            (60, "seixanta"),
            (50, "cinquanta"),
            (40, "quaranta"),
            (30, "trenta"),
        ] {
            cards.insert(BigInt::from(k), v);
        }

        // set_low_numwords: words map to descending values ending at 0.
        let low = [
            "vint-i-nou",
            "vint-i-vuit",
            "vint-i-set",
            "vint-i-sis",
            "vint-i-cinc",
            "vint-i-quatre",
            "vint-i-tres",
            "vint-i-dos",
            "vint-i-un",
            "vint",
            "dinou",
            "divuit",
            "disset",
            "setze",
            "quinze",
            "catorze",
            "tretze",
            "dotze",
            "onze",
            "deu",
            "nou",
            "vuit",
            "set",
            "sis",
            "cinc",
            "quatre",
            "tres",
            "dos",
            "un",
            "zero",
        ];
        let len = low.len();
        for (i, word) in low.iter().enumerate() {
            cards.insert(BigInt::from(len - 1 - i), *word);
        }

        // MAXVAL = 1000 * list(cards)[0]; insertion order puts 10**24 first.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords_tbl: HashMap<u128, &'static str> = [
            (1u128, "primer"),
            (2, "segon"),
            (3, "tercer"),
            (4, "quart"),
            (5, "cinqu"),
            (6, "sis"),
            (7, "set"),
            (8, "vuit"),
            (9, "nov"),
            (10, "des"),
            (11, "onz"),
            (12, "dotz"),
            (13, "tretz"),
            (14, "catorz"),
            (15, "quinz"),
            (16, "setz"),
            (17, "disset"),
            (18, "divuit"),
            (19, "dinov"),
            (20, "vint"),
            (30, "trent"),
            (40, "quarant"),
            (50, "cinquant"),
            (60, "seixant"),
            (70, "setant"),
            (80, "vuitant"),
            (90, "norant"),
            (100, "cent"),
            (200, "dos-cent"),
            (300, "tres-cent"),
            (400, "quatre-cent"),
            (500, "cinc-cent"),
            (600, "sis-cent"),
            (700, "set-cent"),
            (800, "vuit-cent"),
            (900, "nou-cent"),
            (1_000, "mil"),
            (1_000_000, "milion"),
            (1_000_000_000, "mil milion"),
            (1_000_000_000_000, "bilion"),
            (1_000_000_000_000_000, "mil bilion"),
        ]
        .into_iter()
        .collect();

        // CURRENCY_FORMS / CURRENCY_ADJECTIVES are built once, here, and
        // stored on the struct: rebuilding a 166-entry table on every
        // to_currency call is what made an earlier revision of this port an
        // order of magnitude slower than the Python it replaces.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("EUR", &["euro", "euros"][..], &["cèntim", "cèntims"][..]),
            ("ESP", &["pesseta", "pessetes"][..], &["cèntim", "cèntims"][..]),
            ("USD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("PEN", &["sol", "sols"][..], &["cèntim", "cèntims"][..]),
            ("CRC", &["colón", "colons"][..], &["centau", "centaus"][..]),
            ("AUD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("CAD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("GBP", &["lliura", "lliures"][..], &["penic", "penics"][..]),
            ("RUB", &["ruble", "rubles"][..], &["copec", "copecs"][..]),
            ("SEK", &["corona", "corones"][..], &["öre", "öre"][..]),
            ("NOK", &["corona", "corones"][..], &["øre", "øre"][..]),
            ("PLN", &["zloty", "zlotys"][..], &["grosz", "groszy"][..]),
            ("MXN", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("RON", &["leu", "lei"][..], &["ban", "bani"][..]),
            ("INR", &["rupia", "rupies"][..], &["paisa", "paise"][..]),
            ("HUF", &["fòrint", "fòrints"][..], &["fillér", "fillérs"][..]),
            ("FRF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("CNY", &["iuan", "iuans"][..], &["fen", "jiao"][..]),
            ("CZK", &["corona", "corones"][..], &["haléř", "haléřů"][..]),
            ("NIO", &["córdoba", "córdobas"][..], &["centau", "centaus"][..]),
            ("VES", &["bolívar", "bolívars"][..], &["cèntim", "cèntims"][..]),
            ("BRL", &["real", "reals"][..], &["centau", "centaus"][..]),
            ("CHF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("JPY", &["ien", "iens"][..], &["sen", "sen"][..]),
            ("KRW", &["won", "wons"][..], &["jeon", "jeon"][..]),
            ("KPW", &["won", "wons"][..], &["chŏn", "chŏn"][..]),
            ("TRY", &["lira", "lires"][..], &["kuruş", "kuruş"][..]),
            ("ZAR", &["rand", "rands"][..], &["cèntim", "cèntims"][..]),
            ("KZT", &["tenge", "tenge"][..], &["tin", "tin"][..]),
            ("UAH", &["hrívnia", "hrívnies"][..], &["kopiika", "kopíok"][..]),
            ("THB", &["baht", "bahts"][..], &["satang", "satang"][..]),
            ("AED", &["dirham", "dirhams"][..], &["fils", "fulūs"][..]),
            ("AFN", &["afgani", "afganis"][..], &["puli", "puls"][..]),
            ("ALL", &["lek", "lekë"][..], &["qqindarka", "qindarkë"][..]),
            ("AMD", &["dram", "drams"][..], &["luma", "lumas"][..]),
            ("ANG", &["florí", "florins"][..], &["cèntim", "cèntims"][..]),
            ("AOA", &["kwanza", "kwanzes"][..], &["cèntim", "cèntims"][..]),
            ("ARS", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("AWG", &["florí", "florins"][..], &["centau", "centaus"][..]),
            ("AZN", &["manat", "manats"][..], &["qəpik", "qəpik"][..]),
            ("BBD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("BDT", &["taka", "taka"][..], &["poisha", "poisha"][..]),
            ("BGN", &["lev", "leva"][..], &["stotinka", "stotinki"][..]),
            ("BHD", &["dinar", "dinars"][..], &["fils", "fulūs"][..]),
            ("BIF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("BMD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("BND", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("BOB", &["boliviano", "bolivianos"][..], &["centau", "centaus"][..]),
            ("BSD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("BTN", &["ngultrum", "ngultrums"][..], &["chetrum", "chetrums"][..]),
            ("BWP", &["pula", "pula"][..], &["thebe", "thebe"][..]),
            ("BYN", &["ruble", "rubles"][..], &["copec", "copecs"][..]),
            ("BYR", &["ruble", "rubles"][..], &["copec", "copecs"][..]),
            ("BZD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("CDF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("CLP", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("COP", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("CUP", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("CVE", &["escut", "escuts"][..], &["centau", "centaus"][..]),
            ("CYP", &["lliura", "lliures"][..], &["cèntim", "cèntims"][..]),
            ("DJF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("DKK", &["corona", "corones"][..], &["øre", "øre"][..]),
            ("DOP", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("DZD", &["dinar", "dinars"][..], &["cèntim", "cèntims"][..]),
            ("ECS", &["sucre", "sucres"][..], &["centau", "centaus"][..]),
            ("EGP", &["lliura", "lliures"][..], &["piastre", "piastres"][..]),
            ("ERN", &["nakfa", "nakfes"][..], &["cèntim", "cèntims"][..]),
            ("ETB", &["birr", "birr"][..], &["cèntim", "cèntims"][..]),
            ("FJD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("FKP", &["lliura", "lliures"][..], &["penic", "penics"][..]),
            ("GEL", &["lari", "laris"][..], &["tetri", "tetri"][..]),
            ("GHS", &["cedi", "cedis"][..], &["pesewa", "pesewas"][..]),
            ("GIP", &["lliura", "lliures"][..], &["penic", "penics"][..]),
            ("GMD", &["dalasi", "dalasis"][..], &["butut", "bututs"][..]),
            ("GNF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("GTQ", &["quetzal", "quetzals"][..], &["centau", "centaus"][..]),
            ("GYD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("HKD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("HNL", &["lempira", "lempires"][..], &["centau", "centaus"][..]),
            ("HRK", &["kuna", "kuna"][..], &["lipa", "lipa"][..]),
            ("HTG", &["gourde", "gourdes"][..], &["cèntim", "cèntims"][..]),
            ("IDR", &["rúpia", "rúpies"][..], &["cèntim", "cèntims"][..]),
            ("ILS", &["xéquel", "xéquels"][..], &["agorà", "agorot"][..]),
            ("IQD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            ("IRR", &["rial", "rials"][..], &["dinar", "dinars"][..]),
            ("ISK", &["corona", "corones"][..], &["eyrir", "aurar"][..]),
            ("ITL", &["lira", "lires"][..], &["cèntim", "cèntims"][..]),
            ("JMD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("JOD", &["dinar", "dinars"][..], &["piastra", "piastres"][..]),
            ("KES", &["xiling", "xílings"][..], &["cèntim", "cèntims"][..]),
            ("KGS", &["som", "som"][..], &["tyiyn", "tyiyn"][..]),
            ("KHR", &["riel", "riels"][..], &["cèntim", "cèntims"][..]),
            ("KMF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("KWD", &["dinar", "dinars"][..], &["fils", "fils"][..]),
            ("KYD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("LAK", &["kip", "kips"][..], &["at", "at"][..]),
            ("LBP", &["lliura", "lliures"][..], &["piastra", "piastres"][..]),
            ("LKR", &["rúpia", "rúpies"][..], &["cèntim", "cèntims"][..]),
            ("LRD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("LSL", &["loti", "maloti"][..], &["sente", "lisente"][..]),
            ("LTL", &["lita", "litai"][..], &["cèntim", "cèntims"][..]),
            ("LYD", &["dinar", "dinars"][..], &["dírham", "dírhams"][..]),
            ("MAD", &["dírham", "dirhams"][..], &["cèntim", "cèntims"][..]),
            ("MDL", &["leu", "lei"][..], &["ban", "bani"][..]),
            ("MGA", &["ariary", "ariary"][..], &["iraimbilanja", "iraimbilanja"][..]),
            ("MKD", &["denar", "denari"][..], &["deni", "deni"][..]),
            ("MMK", &["kyat", "kyats"][..], &["pya", "pyas"][..]),
            ("MNT", &["tögrög", "tögrög"][..], &["möngö", "möngö"][..]),
            ("MOP", &["pataca", "pataques"][..], &["avo", "avos"][..]),
            ("MRO", &["ouguiya", "ouguiya"][..], &["khoums", "khoums"][..]),
            ("MRU", &["ouguiya", "ouguiya"][..], &["khoums", "khoums"][..]),
            ("MUR", &["rupia", "rúpies"][..], &["cèntim", "cèntims"][..]),
            ("MVR", &["rufiyaa", "rufiyaa"][..], &["laari", "laari"][..]),
            ("MWK", &["kwacha", "kwacha"][..], &["tambala", "tambala"][..]),
            ("MYR", &["ringgit", "ringgits"][..], &["sen", "sens"][..]),
            ("MZN", &["metical", "meticals"][..], &["centau", "centaus"][..]),
            ("NAD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("NGN", &["naira", "naires"][..], &["kobo", "kobos"][..]),
            ("NPR", &["rupia", "rupies"][..], &["paisa", "paises"][..]),
            ("NZD", &["dòlar", "dòlars"][..], &["centau", "centaus"][..]),
            ("OMR", &["rial", "rials"][..], &["baisa", "baisa"][..]),
            ("PAB", &["dòlar", "dòlars"][..], &["centésimo", "centésimos"][..]),
            ("PGK", &["kina", "kina"][..], &["toea", "toea"][..]),
            ("PHP", &["peso", "pesos"][..], &["centau", "centaus"][..]),
            ("PKR", &["rupia", "rupies"][..], &["paisa", "paise"][..]),
            ("PLZ", &["zloty", "zlotys"][..], &["grosz", "groszy"][..]),
            ("PYG", &["guaraní", "guaranís"][..], &["cèntim", "cèntims"][..]),
            ("QAR", &["rial", "rials"][..], &["dírham", "dírhams"][..]),
            ("QTQ", &["quetzal", "quetzals"][..], &["centau", "centaus"][..]),
            ("RSD", &["dinar", "dinars"][..], &["para", "para"][..]),
            ("RUR", &["ruble", "rubles"][..], &["copec", "copecs"][..]),
            ("RWF", &["franc", "francs"][..], &["cèntim", "cèntims"][..]),
            ("SAR", &["riyal", "riyals"][..], &["hàl·lala", "hàl·lalat"][..]),
            ("SBD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("SCR", &["rupia", "rupies"][..], &["cèntim", "cèntims"][..]),
            ("SDG", &["lliura", "lliures"][..], &["piastre", "piastres"][..]),
            ("SGD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("SHP", &["lliura", "lliures"][..], &["penic", "penics"][..]),
            ("SLL", &["leonE", "leones"][..], &["cèntim", "cèntims"][..]),
            ("SRD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("SSP", &["lliura", "lliures"][..], &["piastre", "piastres"][..]),
            ("STD", &["dobra", "dobrAs"][..], &["cèntim", "cèntims"][..]),
            ("SVC", &["colón", "colons"][..], &["centau", "centaus"][..]),
            ("SYP", &["lliura", "lliures"][..], &["piastre", "piastres"][..]),
            ("SZL", &["lilangeni", "emalangeni"][..], &["cèntim", "cèntims"][..]),
            ("TJS", &["somoni", "somoni"][..], &["diram", "diram"][..]),
            ("TMT", &["manat", "manats"][..], &["teňňesi", "teňňesi"][..]),
            ("TND", &["dinar", "dinars"][..], &["mil·lim", "mil·limat"][..]),
            ("TOP", &["paanga", "paangas"][..], &["seniti", "seniti"][..]),
            ("TTD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("TWD", &["nou dòlar", "nous dòlars"][..], &["fen", "fen"][..]),
            ("TZS", &["xíling", "xílings"][..], &["cèntim", "cèntims"][..]),
            ("UGX", &["xíling", "xílings"][..], &["cèntim", "cèntims"][..]),
            ("UYU", &["peso", "pesos"][..], &["centèsim", "centèsims"][..]),
            ("UZS", &["som", "som"][..], &["tiyin", "tiyin"][..]),
            ("VND", &["dong", "dongs"][..], &["xu", "xu"][..]),
            ("VUV", &["vatu", "vatus"][..], &["cèntim", "cèntims"][..]),
            ("WST", &["tala", "tala"][..], &["sene", "sene"][..]),
            ("XAF", &["franc CFA", "francs CFA"][..], &["cèntim", "cèntims"][..]),
            ("XCD", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
            ("XOF", &["franc CFA", "francs CFA"][..], &["cèntim", "cèntims"][..]),
            ("XPF", &["franc CFP", "francs CFP"][..], &["cèntim", "cèntims"][..]),
            ("YER", &["rial", "rials"][..], &["fils", "fils"][..]),
            ("YUM", &["dinar", "dinars"][..], &["para", "para"][..]),
            ("ZMW", &["kwacha", "kwacha"][..], &["ngwee", "ngwee"][..]),
            ("ZWL", &["dòlar", "dòlars"][..], &["cèntim", "cèntims"][..]),
        ]
        .into_iter()
        .map(|(k, u, s)| (k, CurrencyForms::new(u, s)))
        .collect();

        let currency_adjectives: HashMap<&'static str, &'static str> = [
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
            ("ISK", "íslenskar"),
            ("UZS", "Uzbekistan"),
            ("SAR", "Saudi"),
            ("JPY", "Japanese"),
            ("KRW", "Korean"),
        ]
        .into_iter()
        .collect();

        LangCa {
            cards,
            maxval,
            ords_tbl,
            // self.exclude_title = ["i", "menys", "coma"]
            exclude_title: vec!["i".into(), "menys".into(), "coma".into()],
            currency_forms,
            currency_adjectives,
        }
    }

    /// `self.ords[key]` — missing keys raise `KeyError`, which several inputs
    /// genuinely hit (`dec == 10**18`; see module-doc bugs 4 and 6).
    fn ords(&self, key: &BigInt) -> Result<&'static str> {
        key.to_u128()
            .and_then(|k| self.ords_tbl.get(&k).copied())
            .ok_or_else(|| N2WError::Key(format!("{}", key)))
    }

    /// `self.ords_3[key]` — keys are 1..=9 only. **There is no key 0**, so
    /// every round ten 30..90 raises `KeyError` (module-doc bug 1).
    fn ords_3(&self, key: &BigInt) -> Result<&'static str> {
        let word = match key.to_u32() {
            Some(1) => "unè",
            Some(2) => "dosè",
            Some(3) => "tresè",
            Some(4) => "quatrè",
            Some(5) => "cinquè",
            Some(6) => "sisè",
            Some(7) => "setè",
            Some(8) => "vuitè",
            Some(9) => "novè",
            _ => return Err(N2WError::Key(format!("{}", key))),
        };
        Ok(word)
    }

    /// `Num2Word_Base.verify_ordinal`. The float check (`value == int(value)`)
    /// is vacuous for integer input; only the negative check can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "El número negatiu {} no pot ser tractat com un ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.verify_ordinal` over the float/Decimal entry, plus the
    /// truncating `int(value)` the two checks perform first: NaN raises
    /// ValueError and ±inf OverflowError inside the first comparison;
    /// fractional values get CA's own `errmsg_floatord`, negative whole ones
    /// its `errmsg_negord` (both TypeError). `-0.0` passes both checks and
    /// ordinalises as zero → "" (bug 5). Returns the whole value as the
    /// integer the ordinal path then renders.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<BigInt> {
        if let FloatValue::Float { value: f, .. } = value {
            if f.is_nan() {
                return Err(N2WError::Value(
                    "cannot convert float NaN to integer".into(),
                ));
            }
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
        }
        match value.as_whole_int() {
            None => Err(N2WError::Type(
                "El float no pot ser tractat com un ordinal.".into(),
            )),
            Some(i) if i.is_negative() => Err(N2WError::Type(format!(
                "El número negatiu {} no pot ser tractat com un ordinal.",
                i
            ))),
            Some(i) => Ok(i),
        }
    }

    fn to_ordinal_inner(&self, value: &BigInt, depth: u32) -> Result<String> {
        self.verify_ordinal(value)?;
        if depth > RECURSION_LIMIT {
            // Stands in for CPython's RecursionError; see module docs.
            return Err(N2WError::Value(format!(
                "RecursionError: maximum recursion depth exceeded in to_ordinal({})",
                value
            )));
        }

        let five = BigInt::from(5);
        let ten = BigInt::from(10);
        let twenty = BigInt::from(20);
        let thirty = BigInt::from(30);
        let hundred = BigInt::from(100);
        let two_hundred = BigInt::from(200);
        let thousand = pow10(3);

        let text: String = if value.is_zero() {
            // to_ordinal(0) == "" (module-doc bug 5).
            String::new()
        } else if value < &five {
            self.ords(value)?.to_string()
        } else if value <= &twenty {
            format!("{}{}", self.ords(value)?, GENDER_STEM)
        } else if value <= &thirty {
            let frac = value % &ten;
            // PR savoirfairelinux/num2words#670: an exact ten ("trentè") has
            // no units part; the old `ords_3[0]` lookup raised KeyError.
            if frac.is_zero() {
                format!("{}{}", self.ords(value)?, GENDER_STEM)
            } else {
                format!(
                    "{}{}{}",
                    self.ords(&twenty)?,
                    "-i-",
                    self.ords_3(&frac)?
                )
            }
        } else if value < &hundred {
            let dec = (value / &ten) * &ten;
            let frac = value - &dec;
            // PR savoirfairelinux/num2words#670: an exact ten ("quarantè")
            // has no units part; the old `ords_3[0]` lookup raised KeyError.
            if frac.is_zero() {
                format!("{}{}", self.ords(&dec)?, GENDER_STEM)
            } else {
                format!(
                    "{}{}{}{}",
                    self.ords(&dec)?,
                    "a",
                    "-",
                    self.ords_3(&frac)?
                )
            }
        } else if value == &hundred {
            format!("{}{}", self.ords(value)?, GENDER_STEM)
        } else if value < &two_hundred {
            let cen = (value / &hundred) * &hundred;
            format!(
                "{} {}",
                self.ords(&cen)?,
                self.to_ordinal_inner(&(value - &cen), depth + 1)?
            )
        } else if value < &thousand {
            let cen = (value / &hundred) * &hundred;
            format!(
                "{}{} {}",
                self.ords(&cen)?,
                "s",
                self.to_ordinal_inner(&(value - &cen), depth + 1)?
            )
        } else if value == &thousand {
            format!("{}{}", self.ords(value)?, GENDER_STEM)
        } else if value < &pow10(6) {
            // dec is always 1000 here: the ratio lies in (1, 2) with no float
            // window wide enough to reach 2.0.
            let dec = BigInt::from(1000u16).pow(log1000_trunc(value));
            let (high_part, low_part) = value.div_mod_floor(&dec);
            let cardinal = if high_part.is_one() {
                String::new()
            } else {
                self.to_cardinal(&high_part)?
            };
            // Note the spaces — unlike the < 1e18 arm below.
            format!(
                "{} {} {}",
                cardinal,
                self.ords(&dec)?,
                self.to_ordinal_inner(&low_part, depth + 1)?
            )
        } else if value < &pow10(18) {
            let dec = BigInt::from(1000u16).pow(log1000_trunc(value));
            let (high_part, low_part) = value.div_mod_floor(&dec);
            let cardinal = if high_part.is_one() {
                String::new()
            } else {
                self.to_cardinal(&high_part)?
            };
            // Python builds the %-tuple left to right, so `self.ords[dec]`
            // raises KeyError *before* the recursive `to_ordinal(low_part)`
            // is evaluated. Binding it first preserves that ordering — it is
            // what makes to_ordinal(10**18 - 1) a KeyError rather than a
            // RecursionError (module-doc bug 4).
            let o = self.ords(&dec)?;
            // No separator between `cardinal` and `o` (module-doc bug 2).
            format!(
                "{}{}{} {}",
                cardinal,
                o,
                GENDER_STEM,
                self.to_ordinal_inner(&low_part, depth + 1)?
            )
        } else {
            let part1 = self.to_cardinal(value)?;
            // part1[:-1] is CHARACTER slicing: "un trilió" -> "un trili"
            // ("ó" is two bytes, so a byte slice would split it).
            let chars: Vec<char> = part1.chars().collect();
            let keep = chars.len().saturating_sub(1);
            format!(
                "{}{}",
                chars[..keep].iter().collect::<String>(),
                "onè"
            )
        };

        Ok(text.trim().to_string())
    }
}

impl Lang for LangCa {
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
        " amb"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        NEGWORD
    }
    fn pointword(&self) -> &str {
        "coma"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, cnum) = l;
        let (rtext, nnum) = r;
        let mut ctext = ltext.to_string();
        let mut ntext = rtext.to_string();

        let hundred = BigInt::from(100);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                // Python `return next` — the unmodified right tuple.
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        }

        if nnum < cnum {
            // Python spells out four arms here (cnum < 100 / nnum == 1 /
            // cnum == 100 / else) but only the first behaves differently:
            // it joins with "-", the other three are identical " " joins.
            if cnum < &hundred {
                return (format!("{}-{}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && cnum > &BigInt::one() {
            // ntext[:-3] + "lions" — CHARACTER slicing. "milió" is 5 chars but
            // 6 bytes, so a byte slice would yield "mil" and produce
            // "millions". Correct result: "mi" + "lions" == "milions".
            let chars: Vec<char> = ntext.chars().collect();
            let keep = chars.len().saturating_sub(3);
            ntext = chars[..keep].iter().collect::<String>() + "lions";
        }

        if nnum == &hundred {
            ntext.push('s');
            ctext.push('-');
        } else {
            ntext = format!(" {}", ntext);
        }
        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    /// `Num2Word_Base.to_cardinal` for integral input.
    ///
    /// Reimplemented rather than delegating to `default_to_cardinal` purely so
    /// the OverflowError carries CA's `errmsg_toobig`
    /// ("abs(%s) ha de ser inferior a %s.") instead of the base English text.
    /// The algorithm is identical.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = format!("{} ", self.negword().trim());
        }

        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) ha de ser inferior a {}.",
                v,
                self.maxval()
            )));
        }

        let tree = splitnum(self, &v).ok_or_else(|| {
            N2WError::Overflow(format!(
                "abs({}) ha de ser inferior a {}.",
                v,
                self.maxval()
            ))
        })?;
        let words = match clean(self, tree) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        Ok(self.title(&format!("{}{}", out, words)))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        // Catalan has no ordinal for zero; Python returns "" (bug 5, #160).
        if value.is_zero() {
            return Err(N2WError::Value(
                "El número 0 no pot ser tractat com un ordinal.".into(),
            ));
        }
        self.to_ordinal_inner(value, 0)
    }

    /// `to_ordinal(float/Decimal)`: CA's `to_ordinal` opens with
    /// `verify_ordinal(value)`, so a fractional or negative value raises
    /// TypeError; a whole non-negative one then runs the same arithmetic the
    /// integer path does (dict lookups hash `5.0` like `5`, `//`/`%` agree on
    /// whole floats), so the entry delegates to the integer ordinal —
    /// `5.0` → "cinquè", `0.0` → ValueError (bug 5), `1e20` → "cent triliononè".
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal(value)`, then the
    /// `ords_2` membership test — numeric, so `1.0`..`4.0` hit "1r".."4t" —
    /// and otherwise `"%s%s" % (value, "è")` with the repr verbatim:
    /// `5.0` → "5.0è", `Decimal("5.00")` → "5.00è", `-0.0` → "-0.0è";
    /// `0.5`/`-1.0` raise TypeError from the verify.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let i = self.verify_ordinal_float(value)?;
        match i.to_u32() {
            Some(1) => Ok("1r".to_string()),
            Some(2) => Ok("2n".to_string()),
            Some(3) => Ok("3r".to_string()),
            Some(4) => Ok("4t".to_string()),
            _ => Ok(format!("{}{}", repr_str, GENDER_STEM)),
        }
    }

    /// `Num2Word_CA.to_cardinal_float` — **not** `Num2Word_Base`'s.
    ///
    /// ```python
    /// def to_cardinal_float(self, value):
    ///     try:
    ///         float(value) == value
    ///     except (ValueError, TypeError):
    ///         raise TypeError(self.errmsg_nonnum % value)
    ///     pre, post = self.float2tuple(float(value))
    ///     post_str = str(post)
    ///     leading_zeros = self.precision - len(post_str)
    ///     post_str = "0" * leading_zeros + post_str
    ///     out = [self.to_cardinal(pre)]
    ///     if value < 0 and pre == 0:
    ///         out = [self.negword.strip()] + out
    ///     out.append(self.title(self.pointword))
    ///     for _ in range(leading_zeros):
    ///         out.append(self.to_cardinal(0))
    ///     if int(post_str) > 0 or leading_zeros == 0:
    ///         out.append(self.to_cardinal(int(post_str)))
    ///     return " ".join(out)
    /// ```
    ///
    /// Catalan reads the fraction as **one integer**, not digit by digit:
    /// `2.675` is "dos coma sis-cents setanta-cinc" where the shared base path
    /// would say "dos coma sis set cinc". Leading zeros are spelled out one by
    /// one *first*, so `0.01` is "zero coma zero un" — one "zero" from the
    /// padding loop, then `int("01") == 1` as "un".
    ///
    /// Four deliberate departures from `Num2Word_Base.to_cardinal_float`, all
    /// observable:
    ///
    /// 1. **`float2tuple(float(value))` — the cast is unconditional**, so
    ///    Decimal input never reaches float2tuple's exact arm and issue #603's
    ///    fix is silently undone. `Decimal("98746251323029.99")` collapses to
    ///    the double `98746251323029.98`, and the corpus row really does end
    ///    "coma noranta-vuit" rather than "…noranta-nou". `Decimal("1.10")`
    ///    likewise loses its trailing zero to `repr(1.1)` and yields "un coma
    ///    un", where base-path languages say "een komma een nul". This is why
    ///    the `FloatValue::Decimal` arm below re-derives precision from the
    ///    f64 instead of trusting the carried `abs(exponent)`.
    /// 2. **`pointword` is appended unconditionally**, where base guards it
    ///    with `if self.precision:`. Reachable: at precision 0 — e.g.
    ///    `1.2345678901234568e16`, 17 significant digits with `decpt == 17` —
    ///    `leading_zeros` goes to -1 and the final guard is False too, so
    ///    Python emits the integer, then a bare trailing "coma", and nothing
    ///    else. `en` prints no "point" at all for the same input.
    /// 3. **`precision=` is not a parameter here**, so `precision_override` is
    ///    ignored. That is not this port cutting a corner: `num2words`
    ///    implements the kwarg by assigning `converter.precision`, and
    ///    `float2tuple` — which runs first — unconditionally overwrites that
    ///    attribute with the repr-derived value before `leading_zeros` reads
    ///    it. `num2words(2.675, lang="ca", precision=1)` is byte-identical to
    ///    the plain call; so is the `en` equivalent. Verified in the
    ///    interpreter for overrides 1, 3 and 5.
    /// 4. **The final `to_cardinal` is skipped when the fraction is all
    ///    zeros** (`int(post_str) > 0 or leading_zeros == 0`), which is what
    ///    makes `1e16` "deu mil bilions coma" + fifteen "zero" with no
    ///    sixteenth word. When `leading_zeros == 0` the guard passes anyway, so
    ///    a lone zero still prints: `to_cardinal_float(5.0)` is "cinc coma
    ///    zero".
    ///
    /// Python's `except (ValueError, TypeError)` guard around `float(value)`
    /// has no analogue — `FloatValue` cannot hold a non-numeric — so
    /// `errmsg_nonnum` is unreachable from Rust.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `float(value)` — for BOTH arms; see departure 1 above.
        let f = match value {
            FloatValue::Float { value, .. } => *value,
            FloatValue::Decimal { value, .. } => decimal_to_f64(value),
        };

        // float2tuple opens with `pre = int(value)`, so a non-finite double
        // raises there, before the precision is ever computed.
        if f.is_nan() {
            return Err(N2WError::Value("cannot convert float NaN to integer".into()));
        }
        if f.is_infinite() {
            return Err(N2WError::Overflow(
                "cannot convert float infinity to integer".into(),
            ));
        }

        let precision = match value {
            // Already `abs(Decimal(str(f)).as_tuple().exponent)` — the Python
            // shim derived it from repr of this very double, which is exactly
            // what float2tuple would recompute.
            FloatValue::Float { precision, .. } => *precision,
            // Carried precision is the *Decimal's* scale; the float cast has
            // its own repr and hence its own precision (2 for the .99 row
            // above, 1 for Decimal("1.10")).
            FloatValue::Decimal { .. } => float_repr_precision(f),
        };

        // The f64 artefacts are load-bearing, so this reuses floatpath's
        // float2tuple rather than re-deriving from the decimal string: 2.675
        // really does give 674.9999999999998, rescued to 675 by the `< 0.01`
        // heuristic, and 1.005 gives 4.999999999999893, rescued to 5. Both
        // would come out one lower from an exact recomputation.
        //
        // (float2tuple rounds with `round_ties_even`, matching Python. Worth
        // noting the choice is provably inert *here*: the mode can only differ
        // on an exact .5 tie, and a tie is 0.5 away from its integer, so the
        // `< 0.01` guard rejects it and both modes fall through to the same
        // `floor`. The corpus cannot tell them apart either.)
        let (pre, post) = float2tuple(&FloatValue::Float { value: f, precision });

        let post_str = post.to_string();
        // Python lets this go negative; "0" * -1 == "" and range(-1) is empty,
        // so a negative count is a no-op rather than an error. It then feeds
        // the `leading_zeros == 0` guard below, where the sign matters.
        let leading_zeros = precision as i64 - post_str.chars().count() as i64;
        let post_str = if leading_zeros > 0 {
            format!("{}{}", "0".repeat(leading_zeros as usize), post_str)
        } else {
            post_str
        };

        let mut out = vec![self.to_cardinal(&pre)?];
        // `value < 0` tests the ORIGINAL, not the cast — but int(-0.5) == 0
        // carries no minus either way, so the sign has to be re-attached here.
        if value.is_negative() && pre.is_zero() {
            out.insert(0, self.negword().trim().to_string());
        }

        // Unconditional — departure 2.
        out.push(self.title(self.pointword()));

        for _ in 0..leading_zeros.max(0) {
            out.push(self.to_cardinal(&BigInt::zero())?);
        }

        // `int(post_str)` drops the padding again: "01" -> 1 -> "un".
        let post_val = BigInt::from_str(&post_str)
            .map_err(|e| N2WError::Value(e.to_string()))?;
        if post_val.is_positive() || leading_zeros == 0 {
            out.push(self.to_cardinal(&post_val)?);
        }

        Ok(out.join(" "))
    }

    /// The fractional-cents entry point: Python's
    /// `cents_str = self.to_cardinal(float(right))` inside
    /// `Num2Word_Base.to_currency`.
    ///
    /// Overridden because `base.rs`'s default routes to
    /// `floatpath::cardinal_from_bigdecimal`, which calls
    /// `default_to_cardinal_float` directly and so would bypass CA's override.
    /// Python goes through `self.to_cardinal`, which dispatches back to
    /// `Num2Word_CA.to_cardinal_float` for a non-integral float — and the two
    /// spell the fraction differently: `to_currency(1.0125, "USD")` is "un
    /// dòlar amb un coma **vint-i-cinc** centaus", not the base path's "un coma
    /// dos cinc". Verified against the interpreter.
    ///
    /// The `Float` variant is built explicitly to mirror Python casting with
    /// `float(right)` *before* `to_cardinal` sees the value.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        let f = decimal_to_f64(value);
        self.to_cardinal_float(
            &FloatValue::Float {
                value: f,
                precision: float_repr_precision(f),
            },
            None,
        )
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        // ords_2 = {1: "1r", 2: "2n", 3: "3r", 4: "4t"}; everything else gets
        // the digits plus "è" (gender_stem is always "è", so the conditional
        // `"è" if self.gender_stem == "è" else "a"` never picks "a").
        match value.to_u32() {
            Some(1) => Ok("1r".to_string()),
            Some(2) => Ok("2n".to_string()),
            Some(3) => Ok("3r".to_string()),
            Some(4) => Ok("4t".to_string()),
            _ => Ok(format!("{}{}", value, GENDER_STEM)),
        }
    }

    /// `Num2Word_Base.to_year` — CA does not override it, so a year is just a
    /// cardinal (`to_year(-500)` == "menys cinc-cents").
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency -----------------------------------------------------
    //
    // Not overridden, because CA inherits them unchanged and the base.rs
    // defaults already reproduce `Num2Word_Base`:
    //
    //   * `currency_precision` — `CURRENCY_PRECISION` is `{}` all the way up
    //     the chain (CA and EUR never define it), so
    //     `CURRENCY_PRECISION.get(code, 100)` is **always 100**. That is not a
    //     typo: it makes JPY and KWD ordinary 2-decimal currencies here, so
    //     `to_currency(12.34, "JPY")` == "dotze iens amb trenta-quatre sen"
    //     and `to_cheque(1234.56, "KWD")` ends "56/100 DINARS", not "560/1000".
    //     The divisor-1 shortcut in `default_to_currency` is therefore dead
    //     code for CA, and 0-decimal JPY never happens.
    //   * `money_verbose` / `cents_verbose` — both are `to_cardinal`.
    //   * `cents_terse` — `"%0*d" % (2, n)`; `default_cents_terse` at
    //     divisor 100 gives the same ("dotze euros amb 34 cèntims").
    //   * `to_cheque` — CA does not override `Num2Word_Base.to_cheque`, and
    //     `default_to_cheque` is a faithful port of it.

    fn lang_name(&self) -> &str {
        "Num2Word_CA"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize` — inherited; CA does not define its own.
    ///
    /// ```python
    /// def pluralize(self, n, forms):
    ///     form = 0 if n == 1 else 1
    ///     return forms[form]
    /// ```
    ///
    /// Note `n == 1`, not `abs(n) == 1`: the sign is not folded away. Every CA
    /// entry has two forms, so `forms[1]` is always in range and the IndexError
    /// arm below is unreachable in practice — it exists so that a one-form
    /// entry would crash the way Python's list indexing does rather than
    /// silently falling back to the singular.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_CA.to_currency`.
    ///
    /// CA intercepts **integers only** and hands every float straight back to
    /// `Num2Word_Base.to_currency` via `super()`. The two paths are not
    /// consistent with each other, and the differences are observable:
    ///
    /// 1. **Negative ints rendered a double space (fixed, #160).** Python's
    ///    int arm uses `minus_str = self.negword` (`"menys "`, trailing space
    ///    included) in `"%s %s %s"`, so `to_currency(-5, "USD")` was
    ///    "menys␣␣cinc dòlars". The port strips negword like the float arm:
    ///    "menys␣cinc dòlars".
    /// 2. **`adjective=True` is silently ignored for ints.** The int arm never
    ///    consults `CURRENCY_ADJECTIVES`, so `to_currency(100, "USD",
    ///    adjective=True)` == "cent dòlars" while the float `100.0` gives
    ///    "cent US dòlars amb zero centaus".
    /// 3. The int arm reimplements `pluralize`'s rule inline
    ///    (`cr1[0] if abs_val == 1 else cr1[1]`) but against `abs_val`, where
    ///    base passes `abs(val_int)` to `pluralize` — same answer either way.
    ///
    /// The `except (KeyError, AttributeError)` fallback re-enters base for an
    /// unknown code, which then raises NotImplementedError with CA's class
    /// name. `AttributeError` is unreachable (`CURRENCY_FORMS` always exists),
    /// so only the missing-key path is modelled.
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
        // Restore CA's own `separator=" amb"` default; see SEPARATOR_UNSET.
        let separator = if separator == SEPARATOR_UNSET {
            SEPARATOR_DEFAULT
        } else {
            separator
        };

        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                // KeyError -> super().to_currency(...), which raises
                // NotImplementedError from its own CURRENCY_FORMS lookup.
                None => {
                    return default_to_currency(self, val, currency, cents, separator, adjective)
                }
            };
            let cr1 = &forms.unit;

            let minus_str = if v.is_negative() { NEGWORD.trim() } else { "" };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            let currency_str = if abs_val.is_one() {
                &cr1[0]
            } else if cr1.len() > 1 {
                &cr1[1]
            } else {
                &cr1[0]
            };

            // Python: ("%s %s %s" % (...)).strip().
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats/Decimals: `super(Num2Word_CA, self).to_currency(...)`.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `precision` is what the Python shim derives from `repr(value)`.
    fn f(lang: &LangCa, value: f64, precision: u32) -> String {
        lang.to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    fn d(lang: &LangCa, lit: &str, precision: u32) -> String {
        lang.to_cardinal_float(
            &FloatValue::Decimal {
                value: BigDecimal::from_str(lit).unwrap(),
                precision,
            },
            None,
        )
        .unwrap()
    }

    /// The float `to: "cardinal"` rows of the frozen `ca` corpus.
    #[test]
    fn corpus_float_rows() {
        let l = LangCa::new();
        assert_eq!(f(&l, 0.5, 1), "zero coma cinc");
        assert_eq!(f(&l, 1.5, 1), "un coma cinc");
        assert_eq!(f(&l, 2.25, 2), "dos coma vint-i-cinc");
        assert_eq!(f(&l, 3.14, 2), "tres coma catorze");
        assert_eq!(f(&l, 0.01, 2), "zero coma zero un");
        assert_eq!(f(&l, 0.1, 1), "zero coma un");
        assert_eq!(f(&l, 0.99, 2), "zero coma noranta-nou");
        assert_eq!(f(&l, 1.01, 2), "un coma zero un");
        assert_eq!(f(&l, 12.34, 2), "dotze coma trenta-quatre");
        assert_eq!(f(&l, 99.99, 2), "noranta-nou coma noranta-nou");
        assert_eq!(f(&l, 100.5, 1), "cent coma cinc");
        assert_eq!(
            f(&l, 1234.56, 2),
            "mil dos-cents trenta-quatre coma cinquanta-sis"
        );
        // int(-0.5) == 0 carries no minus, so negword is re-attached.
        assert_eq!(f(&l, -0.5, 1), "menys zero coma cinc");
        assert_eq!(f(&l, -1.5, 1), "menys un coma cinc");
        assert_eq!(f(&l, -12.34, 2), "menys dotze coma trenta-quatre");
        // The f64 artefacts: 1.005 floors, 2.675 gives 674.9999999999998 and
        // is rescued to 675 by float2tuple's `< 0.01` heuristic.
        assert_eq!(f(&l, 1.005, 3), "un coma zero zero cinc");
        assert_eq!(f(&l, 2.675, 3), "dos coma sis-cents setanta-cinc");
    }

    /// The `to: "cardinal_dec"` rows — Decimal input, which CA float-casts.
    #[test]
    fn corpus_decimal_rows() {
        let l = LangCa::new();
        assert_eq!(d(&l, "0.01", 2), "zero coma zero un");
        // precision 1, not the Decimal's own 2: repr(float(Decimal("1.10")))
        // is "1.1". A base-path language says "un coma un zero".
        assert_eq!(d(&l, "1.10", 2), "un coma un");
        assert_eq!(d(&l, "12.345", 3), "dotze coma tres-cents quaranta-cinc");
        assert_eq!(d(&l, "0.001", 3), "zero coma zero zero un");
        // Issue #603 undone by CA's unconditional float cast: the exact
        // Decimal ends .99, but the double it collapses to ends .98.
        assert_eq!(
            d(&l, "98746251323029.99", 2),
            "noranta-vuit bilions set-cents quaranta-sis mil dos-cents \
             cinquanta-un milions tres-cents vint-i-tres mil vint-i-nou \
             coma noranta-vuit"
        );
    }

    /// The quirks that separate CA's override from `Num2Word_Base`'s.
    #[test]
    fn ca_specific_quirks() {
        let l = LangCa::new();
        // An integral float reaches here only on a direct call; precision is 1
        // because repr(5.0) == "5.0", and `leading_zeros == 0` lets the lone
        // zero through.
        assert_eq!(f(&l, 5.0, 1), "cinc coma zero");
        assert_eq!(f(&l, 0.0, 1), "zero coma zero");
        assert_eq!(f(&l, 100.0, 1), "cent coma zero");
        // repr(1e16) == "1e+16" -> precision 16; the fraction is all zeros so
        // the final to_cardinal is skipped: fifteen "zero" and no sixteenth.
        assert_eq!(
            f(&l, 1e16, 16),
            "deu mil bilions coma zero zero zero zero zero zero zero zero \
             zero zero zero zero zero zero zero"
        );
        // precision 0 (17 significant digits, decpt 17): a bare trailing
        // "coma" with nothing after it. `en` prints no "point" at all.
        assert_eq!(
            f(&l, 1.2345678901234568e16, 0),
            "dotze mil tres-cents quaranta-cinc bilions sis-cents setanta-vuit \
             mil nou-cents un milions dos-cents trenta-quatre mil cinc-cents \
             seixanta-vuit coma"
        );
        // precision= is inert: float2tuple overwrites self.precision first.
        let v = FloatValue::Float { value: 2.675, precision: 3 };
        for p in [None, Some(1), Some(3), Some(5)] {
            assert_eq!(
                l.to_cardinal_float(&v, p).unwrap(),
                "dos coma sis-cents setanta-cinc"
            );
        }
    }

    /// `float_repr_precision` reproduces `abs(Decimal(repr(f)).as_tuple()
    /// .exponent)`, including the ".0" and scientific-notation arms.
    #[test]
    fn repr_precision_matches_python() {
        for (v, want) in [
            (0.5, 1u32),
            (2.675, 3),
            (12.34, 2),
            (0.0, 1),
            (1.0, 1),
            (5.0, 1),
            (100.0, 1),
            (-1.5, 1),
            (1e15, 1),
            (1e16, 16),
            (1e17, 17),
            (1e21, 21),
            (1e-4, 4),
            (1e-5, 5),
            (1e-20, 20),
            (1.5e-25, 26),
            (1.2345e-7, 11),
            (98746251323029.98, 2),
            (1.2345678901234568e16, 0),
            (9.876543210987654e16, 1),
        ] {
            assert_eq!(float_repr_precision(v), want, "precision of {:e}", v);
        }
    }

    /// The fractional-cents hook must reach CA's override, not base's.
    /// `to_currency(1.0125, "USD")` is "…un coma vint-i-cinc centaus"; routing
    /// through `floatpath` instead would say "…un coma dos cinc centaus".
    #[test]
    fn fractional_cents_uses_ca_float_path() {
        let l = LangCa::new();
        assert_eq!(
            l.cardinal_from_decimal(&BigDecimal::from_str("1.2500").unwrap())
                .unwrap(),
            "un coma vint-i-cinc"
        );
        let v = CurrencyValue::parse("1.0125", false, true, true).unwrap();
        assert_eq!(
            l.to_currency(&v, "USD", true, None, false).unwrap(),
            "un dòlar amb un coma vint-i-cinc centaus"
        );
    }

    /// `int(value)` raises before anything else for a non-finite double.
    #[test]
    fn non_finite_raises_like_python() {
        let l = LangCa::new();
        assert!(matches!(
            l.to_cardinal_float(&FloatValue::Float { value: f64::NAN, precision: 1 }, None),
            Err(N2WError::Value(_))
        ));
        assert!(matches!(
            l.to_cardinal_float(
                &FloatValue::Float { value: f64::INFINITY, precision: 1 },
                None
            ),
            Err(N2WError::Overflow(_))
        ));
    }
}
