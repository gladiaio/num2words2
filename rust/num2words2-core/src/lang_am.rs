//! Port of `lang_AM.py` (Amharic).
//!
//! Registry check: `CONVERTER_CLASSES["am"]` → `lang_AM.Num2Word_AM()`, which
//! is the class ported here.
//!
//! Shape: **engine**. `Num2Word_AM` subclasses `lang_EUR.Num2Word_EUR` →
//! `Num2Word_Base`, defines `mid_numwords`/`low_numwords` + `merge`, and lets
//! the base `splitnum`/`clean` machinery drive the fold. `to_cardinal` *is*
//! overridden, but only to bolt two things onto the base algorithm (the
//! negative-recursion form and the `value == 100` special case) — the
//! splitnum/clean core is still the engine, so `cards`/`maxval`/`merge` are
//! all live here.
//!
//! # Inheritance chain, resolved
//!
//! * `Num2Word_EUR.setup()` builds `self.high_numwords` from the Latin illion
//!   prefixes. **AM's `set_high_numwords` ignores its `high` argument
//!   entirely**, so none of that reaches `cards` — `gen_high_numwords` is dead
//!   code for this language and is deliberately not ported.
//! * `Num2Word_EUR.set_high_numwords` (the `word + GIGA_SUFFIX` routine that
//!   would emit mixed-script tokens like "trሚሊዮን") is fully overridden. The
//!   Python source comments on this explicitly.
//! * `Num2Word_Base.__init__` → `set_numwords()` calls `set_high_numwords`
//!   *first*, and AM's version does `self.cards.clear()` before populating.
//!   Since mid/low are inserted afterwards, the clear is harmless — but it
//!   does mean `cards` holds *exactly* the 10 high entries below plus mid+low,
//!   with no 10^36+ rungs. Hence `MAXVAL = 1000 * 10**33 = 10**36`.
//! * `Num2Word_Base.verify_ordinal` supplies the `TypeError` on negatives.
//! * `Num2Word_Base.to_splitnum` / `inflect` drive `to_year`'s second branch.
//!
//! * `Num2Word_Base.to_cardinal_float` / `float2tuple` are **not** overridden
//!   anywhere in AM's chain, so the float/Decimal path is the base one and
//!   [`Lang::to_cardinal_float`]'s default ([`crate::floatpath`]) already
//!   serves it. See "Float and Decimal input" below for why that is enough.
//!
//! * `Num2Word_EUR.pluralize` (`forms[0 if n == 1 else 1]`) and
//!   `Num2Word_EUR.CURRENCY_ADJECTIVES` are inherited as-is. `CURRENCY_FORMS`
//!   is *not*: AM shadows it with its own three-code table, so the
//!   `Num2Word_EN.__init__` mutation that rewrites `Num2Word_EUR`'s shared
//!   class dict in place never reaches AM. Verified against the live
//!   interpreter — `Num2Word_AM.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS`
//!   is `False`, and the table holds exactly ETB/USD/JPY. Every other code
//!   (EUR and GBP included) raises NotImplementedError.
//! * `CURRENCY_PRECISION` resolves to `Num2Word_Base`'s empty dict, so
//!   `.get(code, 100)` is 100 for every code and the trait's default
//!   `currency_precision` already matches. Not overridden here.
//!
//! # Float and Decimal input
//!
//! **No `to_cardinal_float` override belongs here.** AM reaches the float path
//! through its `to_cardinal` override, whose dispatch is byte-for-byte the one
//! `Num2Word_Base.to_cardinal` already performs:
//!
//! ```python
//! try:
//!     assert int(value) == value
//! except (ValueError, TypeError, AssertionError):
//!     return self.to_cardinal_float(value)
//! ```
//!
//! So AM only *looks* like one of the 146 languages that hand-roll float
//! handling inside `to_cardinal`; it delegates to the untouched
//! `Num2Word_Base.to_cardinal_float`, and the trait default over
//! [`crate::floatpath::default_to_cardinal_float`] is that method. The two
//! places AM's override actually diverges from Base — the negword recursion
//! and the `value == 100` short-circuit — are both reached *through* the float
//! path, because `to_cardinal_float` calls back into `self.to_cardinal(pre)`
//! for the integral part and once per fractional digit. That callback is a
//! virtual dispatch in Python and a trait dispatch here, so both land on AM's
//! override and both quirks survive:
//!
//! * `100.5` → "መቶ ነጥብ አምስት", *not* "አንድ መቶ ነጥብ አምስት" — the `== 100`
//!   short-circuit fires on `pre`. (Corpus-confirmed; contrast quirk 1, where
//!   a nested 100 keeps its "አንድ".) `-100.5` → "ሰልቢ መቶ ነጥብ አምስት" likewise.
//! * `-12.34` → "ሰልቢ አሥራ ሁለት ነጥብ ሦስት አራት" via `to_cardinal(-12)` recursing.
//!   Base would have produced the identical string through `"%s " %
//!   negword.strip()`, since AM's negword is "ሰልቢ " — trailing space included.
//! * `-0.5` → "ሰልቢ ዜሮ ነጥብ አምስት" comes from the *other* sign path:
//!   `int(-0.5) == 0` carries no minus, so `to_cardinal_float`'s own
//!   `if value < 0 and pre == 0` prepends `negword.strip()`. AM's recursion is
//!   never reached here.
//!
//! `0.0` and `1.0` never see this path at all: `int(0.0) == 0.0` holds, the
//! assert passes, and they take the integer branch — hence corpus rows "0.0" →
//! "ዜሮ" and "1.0" → "አንድ" with no "ነጥብ". Reaching `to_cardinal_float(2.0)`
//! *directly* does yield "ሁለት ነጥብ ዜሮ", which is why the entry test below pins
//! it separately from the dispatcher's behaviour.
//!
//! The f64 artefacts `float2tuple` depends on are load-bearing and are the
//! shared port's business, not AM's — `2.675` reaches it as
//! `674.9999999999998` and is rescued to `675` by the `< 0.01` heuristic, so
//! AM prints "ሁለት ነጥብ ስድስት ሰባት አምስት". Likewise the Float/Decimal split is
//! observable here: `98746251323029.99` ends "…ዘጠኝ ስምንት" as an f64 (its repr
//! is already `…029.98`) but "…ዘጠኝ ዘጠኝ" as a `Decimal`. Both are pinned below.
//!
//! # Faithfully reproduced Python quirks
//!
//! All of these are what the interpreter actually emits and are confirmed
//! against the frozen corpus. They are preserved verbatim, not fixed:
//!
//! 1. **`to_cardinal(100)` == "መቶ", but 100 nested inside a larger number is
//!    "አንድ መቶ".** The `if value == 100` short-circuit in `to_cardinal` only
//!    fires at top level; `merge` reaches its `rnum > lnum` arm for the
//!    (1, 100) pair and prefixes "አንድ". So `100` → "መቶ" while `1100` →
//!    "አንድ ሺህ አንድ መቶ" and `101` → "አንድ መቶ አንድ". Corpus-confirmed.
//! 2. **`low_numwords` has a typo at 16**: "አስራ ስድስት" — every other teen uses
//!    "አሥራ" (with ሥ), 16 alone uses "አስራ" (with ስ). Kept verbatim, so
//!    `to_cardinal(16)` == "አስራ ስድስት" and `to_ordinal(16)` == "አስራ ስድስተኛ".
//!    Note `ords` *does* contain the correctly-spelled "አሥራ ስድስት" key — which
//!    never matches anything (see 3).
//! 3. **The 9 multi-word `ords` keys are unreachable dead entries.**
//!    `to_ordinal` looks the suffix up as `outwords[-1].split("-")[-1]`, i.e. a
//!    single *space-delimited* token, so keys like "አሥራ አንድ" (which contain a
//!    space) can never be hit. They are retained here for table fidelity. The
//!    output is unaffected: 11 → "አሥራ አንደኛ" either way, because the fallback
//!    path ordinalises just the trailing "አንድ" → "አንደኛ".
//! 4. **`to_ordinal` splits on "-" but re-joins with " ".** EN joins back with
//!    "-"; AM uses `" ".join(lastwords)`. Latent, not observable: AM's `merge`
//!    never emits a hyphen, so `lastwords` always has exactly one element and
//!    the join is a no-op. Reproduced anyway — see [`LangAm::to_ordinal`].
//! 5. **`exclude_title` lists "አሉታዊ" but `negword` is "ሰልቢ ".** The exclusion
//!    list can therefore never protect the negative word. Moot in practice:
//!    `is_title` is `False` in `Num2Word_Base.__init__` and *nothing* in the
//!    library ever sets it True, so `title()` is unconditionally the identity.
//!    Both fields are carried over for completeness.
//! 6. **`to_year` on negatives leans on Python's floor `//` and `%`.**
//!    `to_year(-44)` == "ሰልቢ አንድ መቶ አምሳ ስድስት" — literally "negative one
//!    hundred fifty six" for the year 44 BC. This falls out of
//!    `divmod(-44, 100) == (-1, 56)`: the high part renders as `to_cardinal(-1)`
//!    → "ሰልቢ አንድ" and the low part as the *positive* 56. Rust's `/` and `%`
//!    truncate toward zero and would give `(0, -44)` instead, so this port uses
//!    `div_mod_floor`/`div_floor`/`mod_floor` throughout. Corpus-confirmed.
//!
//! # Faithfully reproduced Python quirks — currency
//!
//! 7. **`CURRENCY_FORMS` entries are 3-tuples, not 2-tuples.** Every other
//!    table in the library is `(unit_forms, subunit_forms)`; AM's carries a
//!    third element, a per-currency default separator (`" ከ"`). See
//!    `AmCurrency`. This single shape difference drives quirks 8 and 9.
//! 8. **`to_cheque` is unsupported.** The inherited `Num2Word_Base.to_cheque`
//!    unpacked `cr1, _cr2 = self.CURRENCY_FORMS[currency]`, which raised
//!    `ValueError: too many values to unpack` against AM's 3-tuples. The
//!    port raises NotImplementedError ("does not support to='cheque'") for
//!    every code instead (#223). See [`LangAm::to_cheque`].
//! 9. **`to_currency` ignores `CURRENCY_PRECISION` and hardcodes 100.** The
//!    divisor is the literal `100` in `has_fractional_cents`, and
//!    `parse_currency_parts` is called without a `divisor=` kwarg, taking its
//!    100 default. JPY therefore renders sen — `12.34 JPY` → "…ከ ሠላሳ አራት ሴን"
//!    — where `Num2Word_Base.to_currency` would consult the precision table
//!    and skip the subunit for a 0-decimal currency. Moot in practice, since
//!    AM's `CURRENCY_PRECISION` is empty anyway, but the two would diverge if
//!    it ever gained an entry. Corpus-confirmed.
//! 10. **`to_currency` accepts `adjective` and never reads it.** The parameter
//!    is in the signature but the body has no `if adjective:` branch, so
//!    `CURRENCY_ADJECTIVES` — inherited intact from EUR — is unreachable
//!    through this language. `adjective=True` changes nothing. The table is
//!    still exposed via [`Lang::currency_adjective`] for data fidelity, the
//!    same way the dead `ords` entries are kept above.
//! 11. **A caller-supplied `separator=","` is silently overridden.** The guard
//!    is `if separator == ","`, a *value* test, not an "was the kwarg omitted"
//!    test — so passing the base default explicitly still swaps in `" ከ"`.
//! 12. **A custom separator double-spaces.** The format string is
//!    `"%s%s %s%s %s %s"`: the separator is glued to the unit word with no
//!    space and followed by `" %s"`. AM's own `" ከ"` is written with a leading
//!    space to compensate, so `separator=" ና "` yields `"… ብር ና  ሠላሳ አራት …"`
//!    with two spaces. Falls out of the format string; not special-cased.
//!
//! # Error variants
//!
//! * `to_ordinal`/`to_ordinal_num` on a negative → `verify_ordinal` raises
//!   `TypeError` → [`N2WError::Type`]. Corpus-confirmed for -1, -7, -21, -42,
//!   -100, -999, -1000, -1000000.
//! * `to_cardinal(v)` for `v >= 10**36` → `OverflowError` → [`N2WError::Overflow`].
//!   (Not exercised by the corpus; the largest case is 10^21.)
//! * `to_currency(v, "<unknown>")` → `NotImplementedError` →
//!   [`N2WError::NotImplemented`]. Raised *before* `to_cardinal` runs, so
//!   `to_currency(10**40, "EUR")` is NotImplementedError while
//!   `to_currency(10**40, "USD")` is OverflowError. The ordering is preserved.
//! * `to_cheque` → `NotImplementedError` for every code (quirk 8).

use crate::base::{
    clean, set_low_numwords, set_mid_numwords, splitnum, Cards, Lang, N2WError, Node, Result,
};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, float2tuple, FloatValue};
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `GIGA_SUFFIX` on `Num2Word_AM`. Referenced via `self.GIGA_SUFFIX` in
/// `set_high_numwords`, so it lands at 10^9.
const GIGA_SUFFIX: &str = "ቢሊዮን";
/// `MEGA_SUFFIX` on `Num2Word_AM`. Lands at 10^6.
const MEGA_SUFFIX: &str = "ሚሊዮን";

/// The hundreds word, used both as a card and as `to_year`'s `hightxt`.
const HUNDRED_TXT: &str = "መቶ";

/// The generic ordinal suffix appended when `ords` misses.
const ORD_SUFFIX: &str = "ኛ";

/// The zero word, as `to_currency` hardcodes it.
///
/// Python writes the bare literal `"ዜሮ"` in the `right > 0` guard rather than
/// calling `self.to_cardinal(0)`. The two agree (card 0 is the same word), but
/// they are separate sources in the original and are kept separate here.
const ZERO_TXT: &str = "ዜሮ";

/// `to_currency`'s hardcoded subunit divisor.
///
/// Python writes `100` twice as a literal — once in `has_fractional_cents` and
/// once implicitly, by calling `parse_currency_parts` without `divisor=`. It
/// never consults `CURRENCY_PRECISION`. See module docs (9).
const CENTS_DIVISOR: i64 = 100;

/// One `Num2Word_AM.CURRENCY_FORMS` entry.
///
/// AM's entries are **3-tuples** — `(unit_forms, subunit_forms, separator)` —
/// where the rest of the library carries 2-tuples. The third slot is a
/// per-currency default separator that `to_currency` unpacks and substitutes
/// whenever `separator` is still `","`.
///
/// The extra element is modelled here rather than flattened because it is
/// load-bearing twice over: it supplies the separator, and its mere presence
/// is what makes the inherited `to_cheque` raise ValueError. See module docs
/// (7) and (8).
struct AmCurrency {
    /// The tuple's first two elements, in the shape the `Lang` trait wants.
    forms: CurrencyForms,
    /// The tuple's third element — `to_currency`'s `default_separator` local.
    separator: &'static str,
}

/// `Num2Word_AM.CURRENCY_FORMS`.
///
/// AM declares this in its own class body, shadowing `Num2Word_EUR`'s. That
/// matters: `Num2Word_EN.__init__` mutates *EUR's* dict in place at import
/// time (adding ~24 codes and rewriting EUR/GBP), and none of that lands here.
/// So the table is exactly these three codes — confirmed against the live
/// interpreter, and pinned by the corpus, where `currency:EUR` and
/// `currency:GBP` both expect NotImplementedError.
fn build_currency() -> HashMap<&'static str, AmCurrency> {
    // Both subunit tables are the same word twice; EUR's pluralize indexes 0
    // or 1 and gets the identical string either way. Kept as written.
    const SANTIM: [&str; 2] = ["ሳንቲም", "ሳንቲም"];
    const SEP: &str = " ከ";

    let mut m: HashMap<&'static str, AmCurrency> = HashMap::new();
    m.insert(
        "ETB",
        AmCurrency {
            forms: CurrencyForms::new(&["ብር", "ብር"], &SANTIM),
            separator: SEP,
        },
    );
    m.insert(
        "USD",
        AmCurrency {
            forms: CurrencyForms::new(&["ዶላር", "ዶላር"], &SANTIM),
            separator: SEP,
        },
    );
    m.insert(
        "JPY",
        AmCurrency {
            forms: CurrencyForms::new(&["የን", "የን"], &["ሴን", "ሴን"]),
            separator: SEP,
        },
    );
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged.
///
/// Dead for this language: `Num2Word_AM.to_currency` takes an `adjective`
/// parameter and never reads it, and nothing else consults the table. Kept for
/// data fidelity — see module docs (10). Note the codes here are mostly ones
/// AM has no `CURRENCY_FORMS` entry for, so they would raise before an
/// adjective could ever apply.
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
        ("ISK", "íslenskar"),
        ("UZS", "Uzbekistan"),
        ("SAR", "Saudi"),
        ("JPY", "Japanese"),
        ("KRW", "Korean"),
    ]
    .into_iter()
    .collect()
}

/// Best-effort `str(number)` for the TypeError messages (the corpus compares
/// exception types only, so the float arm's repr need not be byte-exact).
fn am_py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

pub struct LangAm {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    hundred: BigInt,
    ten: BigInt,
    currency: HashMap<&'static str, AmCurrency>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangAm {
    fn default() -> Self {
        Self::new()
    }
}

impl LangAm {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_AM.set_high_numwords: clears cards, then installs exactly
        // these ten rungs. The `high` argument (the EUR Latin-prefix illion
        // list) is never read. Descending order matches the dict literal in
        // the Python source, which is what fixes `list(cards.keys())[0]`.
        let ten_big = BigInt::from(10u8);
        for (exp, word) in [
            (33u32, "ዴሲሊዮን"),
            (30, "ኖኒሊዮን"),
            (27, "ኦክቲሊዮን"),
            (24, "ሴፕቲሊዮን"),
            (21, "ሴክስቲሊዮን"),
            (18, "ኲንቲሊዮን"),
            (15, "ኳድሪሊዮን"),
            (12, "ትሪሊዮን"),
            (9, GIGA_SUFFIX),
            (6, MEGA_SUFFIX),
        ] {
            cards.insert(ten_big.pow(exp), word);
        }

        // Num2Word_AM.setup: mid_numwords
        set_mid_numwords(
            &mut cards,
            &[
                (1000, "ሺህ"),
                (100, HUNDRED_TXT),
                (90, "ዘጠና"),
                (80, "ሰማንያ"),
                (70, "ሰባ"),
                (60, "ስድሳ"),
                (50, "አምሳ"),
                (40, "አርባ"),
                (30, "ሠላሳ"),
            ],
        );

        // Num2Word_AM.setup: low_numwords → values 20 down to 0.
        // "አስራ ስድስት" (16) is the source's own typo — see module docs (2).
        set_low_numwords(
            &mut cards,
            &[
                "ሃያ",
                "አሥራ ዘጠኝ",
                "አሥራ ስምንት",
                "አሥራ ሰባት",
                "አስራ ስድስት",
                "አሥራ አምስት",
                "አሥራ አራት",
                "አሥራ ሦስት",
                "አሥራ ሁለት",
                "አሥራ አንድ",
                "አሥር",
                "ዘጠኝ",
                "ስምንት",
                "ሰባት",
                "ስድስት",
                "አምስት",
                "አራት",
                "ሦስት",
                "ሁለት",
                "አንድ",
                "ዜሮ",
            ],
        );

        // Num2Word_Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0].
        // Cards is stored descending, so `highest()` is that first key: 10^33.
        // → MAXVAL = 10^36.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // Num2Word_AM.setup: self.ords. The nine multi-word entries below are
        // dead — see module docs (3) — but are kept for table fidelity.
        let ords: HashMap<&str, &str> = [
            ("አንድ", "አንደኛ"),
            ("ሁለት", "ሁለተኛ"),
            ("ሦስት", "ሦስተኛ"),
            ("አራት", "አራተኛ"),
            ("አምስት", "አምስተኛ"),
            ("ስድስት", "ስድስተኛ"),
            ("ሰባት", "ሰባተኛ"),
            ("ስምንት", "ስምንተኛ"),
            ("ዘጠኝ", "ዘጠነኛ"),
            ("አሥር", "አሥረኛ"),
            // --- unreachable from here down (keys contain a space) ---
            ("አሥራ አንድ", "አሥራ አንደኛ"),
            ("አሥራ ሁለት", "አሥራ ሁለተኛ"),
            ("አሥራ ሦስት", "አሥራ ሦስተኛ"),
            ("አሥራ አራት", "አሥራ አራተኛ"),
            ("አሥራ አምስት", "አሥራ አምስተኛ"),
            ("አሥራ ስድስት", "አሥራ ስድስተኛ"),
            ("አሥራ ሰባት", "አሥራ ሰባተኛ"),
            ("አሥራ ስምንት", "አሥራ ስምንተኛ"),
            ("አሥራ ዘጠኝ", "አሥራ ዘጠነኛ"),
        ]
        .into_iter()
        .collect();

        LangAm {
            cards,
            maxval,
            ords,
            // Num2Word_AM.setup: self.exclude_title. Never consulted, since
            // is_title is always False — see module docs (5).
            exclude_title: vec!["እና".into(), "ነጥብ".into(), "አሉታዊ".into()],
            hundred: BigInt::from(100),
            ten: BigInt::from(10),
            // Built once here, never per call. `to_currency`/`to_cheque` only
            // read these tables, and rebuilding them per call is what made an
            // earlier revision of this port slower than the Python it replaces.
            currency: build_currency(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float branch (`errmsg_floatord`)
    /// cannot fire for BigInt input; only the negative branch is reachable.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `verify_ordinal` for a float/Decimal: the float check fires first,
    /// then the negative one — both TypeError with Base's default wording
    /// (AM never overrides the messages). `-0.0` passes both. Returns the
    /// whole value on success.
    fn verify_ordinal_num(&self, value: &FloatValue) -> Result<BigInt> {
        let whole = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Cannot treat float {} as ordinal.",
                    am_py_num_str(value)
                )))
            }
        };
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                am_py_num_str(value)
            )));
        }
        Ok(whole)
    }

    /// Python's `self.to_cardinal(x)` for a Decimal — AM's own `to_cardinal`
    /// asserts `int(value) == value` (whole → integer path, with the bare
    /// "መቶ" for 100) and otherwise falls into `Num2Word_Base
    /// .to_cardinal_float` ("ነጥብ" + a word per digit).
    fn cardinal_num_dec(&self, x: &BigDecimal) -> Result<String> {
        if x.is_integer() {
            return self.to_cardinal(&x.with_scale(0).as_bigint_and_exponent().0);
        }
        let precision = x.as_bigint_and_exponent().1.max(0) as u32;
        default_to_cardinal_float(
            self,
            &FloatValue::Decimal {
                value: x.clone(),
                precision,
            },
            None,
        )
    }

    /// `Num2Word_Base.inflect`, specialised to AM's only caller.
    ///
    /// Python: `text.split("/")`; return `text[0]` if `value == 1` else
    /// `"".join(text)`. AM only ever passes "መቶ", which contains no "/", so
    /// both arms collapse to "መቶ" for every `value` — including the negative
    /// `high` that `to_year` hands it. Kept as a function to keep the call
    /// shape honest.
    fn inflect(&self, _value: &BigInt, text: &str) -> String {
        // No "/" in AM's hightxt, so split/join round-trips unchanged.
        text.to_string()
    }

    /// The `except KeyError: raise NotImplementedError(...)` handler that
    /// `to_currency` and the inherited `to_cheque` spell out identically.
    fn currency_not_implemented(&self, currency: &str) -> N2WError {
        N2WError::NotImplemented(format!(
            "Currency code \"{}\" not implemented for \"{}\"",
            currency,
            self.lang_name()
        ))
    }
}

impl Lang for LangAm {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ETB"
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
        // Note the trailing space: AM's to_cardinal concatenates negword
        // directly (`self.negword + ...`) rather than going through the
        // base's `"%s " % negword.strip()`.
        "ሰልቢ "
    }
    fn pointword(&self) -> &str {
        "ነጥብ"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// `Num2Word_AM.merge`.
    ///
    /// The final `else` is reachable for AM (unlike EN's ", " arm which is
    /// mostly cosmetic): e.g. 1100 folds ("አንድ ሺህ", 1000) with ("አንድ መቶ", 100)
    /// where `lnum >= 100` and `rnum == 100`, so the `lnum >= 100 > rnum`
    /// guard fails (100 > 100 is false) and control falls through to the
    /// additive default. Both that arm and the `100 > lnum > rnum` arm emit a
    /// plain space, so they are textually identical — only the returned
    /// magnitude differs, and both add.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = &self.hundred;

        if lnum.is_one() && rnum < hundred {
            (rtext.to_string(), rnum.clone())
        } else if hundred > lnum && lnum > rnum {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        } else if lnum >= hundred && hundred > rnum {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        } else if rnum > lnum {
            (format!("{} {}", ltext, rtext), lnum * rnum)
        } else {
            // Default case: lnum > rnum, both could be >= 100
            (format!("{} {}", ltext, rtext), lnum + rnum)
        }
    }

    /// `Num2Word_AM.to_cardinal`.
    ///
    /// Diverges from `Num2Word_Base.to_cardinal` in two ways, both preserved:
    /// negatives *recurse* (`negword + to_cardinal(-value)`) instead of being
    /// flattened with `abs()`, and `value == 100` short-circuits to a bare
    /// "መቶ". The negative check runs **before** the MAXVAL check, so
    /// `to_cardinal(-10**36)` still overflows — via the recursive call.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            let inner = self.to_cardinal(&(-value))?;
            return Ok(format!("{}{}", self.negword(), inner));
        }

        if value >= &self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                value, self.maxval
            )));
        }

        // `out` is "" at this point in Python; the concatenations below are
        // therefore no-ops, but the 100 short-circuit is not.
        if value == &self.hundred {
            return Ok(self.title(HUNDRED_TXT));
        }

        // Unreachable in practice: card 0 exists and `value >= 0` here, so
        // splitnum always finds a rung. Python would hit
        // `TypeError: object of type 'NoneType' has no len()` inside clean().
        let tree = splitnum(self, value)
            .ok_or_else(|| N2WError::Type("object of type 'NoneType' has no len()".into()))?;
        let words = match clean(self, tree) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        Ok(self.title(&words))
    }

    /// `Num2Word_AM.to_ordinal`.
    ///
    /// Ordinalises only the final space-delimited token: look it up in `ords`,
    /// else append "ኛ". The `split("-")` / `" ".join(...)` asymmetry is the
    /// source's, not a transcription slip — see module docs (4).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        // Python `.split(" ")` — a single-space split that keeps empties,
        // which is Rust's `split(' ')`, *not* `split_whitespace()`.
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(String::from).collect();

        // `outwords` is never empty: "".split(" ") == [""] in Python too.
        let last = outwords[outwords.len() - 1].clone();
        let mut lastwords: Vec<String> = last.split('-').map(String::from).collect();
        let lastword = lastwords[lastwords.len() - 1].to_lowercase();

        // Ethiopic has no case, so `.lower()` is the identity here; kept for
        // faithfulness to the Python.
        let newlast = match self.ords.get(lastword.as_str()) {
            Some(o) => (*o).to_string(),
            None => format!("{}{}", lastword, ORD_SUFFIX),
        };

        let li = lastwords.len() - 1;
        lastwords[li] = self.title(&newlast);
        let oi = outwords.len() - 1;
        // Python: `outwords[-1] = " ".join(lastwords)` — split on "-", joined
        // on " ". No-op in practice (lastwords has length 1 always).
        outwords[oi] = lastwords.join(" ");

        Ok(outwords.join(" "))
    }

    /// `Num2Word_AM.to_ordinal_num`: `"%s%s" % (value, self.to_ordinal(value)[-1:])`.
    ///
    /// `[-1:]` is a *character* slice on a Python str, so this takes the last
    /// Unicode scalar — always "ኛ" — and glues it to the decimal digits.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let ord = self.to_ordinal(value)?;
        // `[-1:]` on an empty string yields "" rather than raising.
        let suffix: String = ord.chars().last().map(String::from).unwrap_or_default();
        Ok(format!("{}{}", value, suffix))
    }

    /// `Num2Word_AM.to_year` (with `longval=True`, the default) plus the
    /// `Num2Word_Base.to_splitnum` it delegates to.
    ///
    /// `if not (val // 100) % 10` → plain cardinal when the hundreds digit is
    /// zero (covers 0..99, and X000/X0000-style values). Otherwise the
    /// "<high> መቶ <low>" split form. All arithmetic is Python-floored: see
    /// module docs (6) for why that is load-bearing on negative years.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value
            .div_floor(&self.hundred)
            .mod_floor(&self.ten)
            .is_zero()
        {
            return self.to_cardinal(value);
        }

        // --- Num2Word_Base.to_splitnum(val, hightxt="መቶ", longval=True) ---
        // `high, low = val` raises TypeError on an int, so Python falls into
        // the `divmod(val, divisor)` branch with divisor=100.
        let (high, low) = value.div_mod_floor(&self.hundred);
        let mut out: Vec<String> = Vec::new();

        if !high.is_zero() {
            // hightxt = self.title(self.inflect(high, hightxt)) → always "መቶ".
            let hightxt = self.title(&self.inflect(&high, HUNDRED_TXT));
            out.push(self.to_cardinal(&high)?);
            if !low.is_zero() {
                // longval is True; jointxt is "" so its append is skipped.
                if !hightxt.is_empty() {
                    out.push(hightxt);
                }
            } else if !hightxt.is_empty() {
                out.push(hightxt);
            }
        }

        if !low.is_zero() {
            // cents defaults True → words, not "%02d".
            out.push(self.to_cardinal(&low)?);
            // lowtxt is "" → the trailing inflect append is skipped.
        }

        Ok(out.join(" "))
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for any
    /// fractional or negative value; a whole non-negative one flows through
    /// `to_cardinal` + the `ords` last-word rewrite — identical to the
    /// integer ordinal ("5.0" → "አምስተኛ", `100.0` → "መቶኛ").
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        self.to_ordinal(&whole)
    }

    /// `to_ordinal_num(float/Decimal)`:
    /// `"%s%s" % (value, self.to_ordinal(value)[-1:])` — the raw `str(value)`
    /// glued to the ordinal's final character (always "ኛ"): "5.0ኛ",
    /// "1e+16ኛ". Negatives/fractions die in `verify_ordinal` first.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        let ord = self.to_ordinal(&whole)?;
        let suffix: String = ord.chars().last().map(String::from).unwrap_or_default();
        Ok(format!("{}{}", repr_str, suffix))
    }

    /// `to_year(float/Decimal)` — `Num2Word_AM.to_year`'s arithmetic on the
    /// raw value. `(val // 100) % 10` uses Python's per-type semantics
    /// (float floor-division/modulo, Decimal truncation), and the fallthrough
    /// is `to_splitnum(val, hightxt="መቶ")`, whose **float** arm goes through
    /// `float2tuple` (`-21.0` → `(-21, 0)` → "ሰልቢ ሃያ አንድ መቶ",
    /// `-1.5` → `(-1, 5)` → "ሰልቢ አንድ መቶ አምስት") while the **Decimal** arm
    /// divmods by 100 (`Decimal("100")` → "አንድ መቶ").
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: v, .. } => {
                let v = *v;
                if (v / 100.0).floor().rem_euclid(10.0) == 0.0 {
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum, float arm: high/low from base.float2tuple.
                let (high, low) = float2tuple(value);
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    let hightxt = self.title(&self.inflect(&high, HUNDRED_TXT));
                    out.push(self.to_cardinal(&high)?);
                    if !hightxt.is_empty() {
                        out.push(hightxt);
                    }
                }
                if !low.is_zero() {
                    out.push(self.to_cardinal(&low)?);
                }
                Ok(out.join(" "))
            }
            FloatValue::Decimal { value: d, .. } => {
                let hundred = BigDecimal::from(100);
                // Decimal // truncates toward zero, and its % keeps the
                // dividend's sign — BigInt's truncating % matches.
                let q = (d / &hundred).with_scale_round(0, bigdecimal::RoundingMode::Down);
                let q_int = q.as_bigint_and_exponent().0;
                if (&q_int % BigInt::from(10)).is_zero() {
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum, Decimal arm: `high, low = divmod(val, 100)`.
                let high = q;
                let low = d - &high * &hundred;
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    out.push(self.cardinal_num_dec(&high)?);
                    out.push(HUNDRED_TXT.to_string());
                }
                if !low.is_zero() {
                    out.push(self.cardinal_num_dec(&low)?);
                }
                Ok(out.join(" "))
            }
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_AM` overrides `to_currency` outright and reaches `to_cardinal`
    // directly, so `_money_verbose`, `_cents_verbose` and `_cents_terse` are
    // never called and their trait defaults stand. `CURRENCY_PRECISION` is
    // Base's empty dict, so `currency_precision`'s default 100 is already
    // right. `to_cheque` is *not* overridden in Python, but the inherited body
    // cannot succeed against AM's 3-tuples, so it is spelled out below.

    fn lang_name(&self) -> &str {
        "Num2Word_AM"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency.get(code).map(|e| &e.forms)
    }

    /// `Num2Word_EUR.CURRENCY_ADJECTIVES[code]`. Unreachable for AM — see
    /// module docs (10).
    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a single-form entry with `n != 1`
    /// would raise IndexError. All three AM entries carry two forms, so that
    /// is unreachable — and since both forms are the *same word* in every
    /// entry, the plural rule is entirely unobservable for this language.
    /// Mapped to `Index` rather than panicking so the exception type survives
    /// if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_AM.to_currency`.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        // Python declares `adjective=False` and never reads it — module docs (10).
        _adjective: bool,
    ) -> Result<String> {
        // `is_integer_input = isinstance(val, int)`. Load-bearing: a true int
        // skips the cents segment entirely, while the float 1.0 still renders
        // "… ከ ዜሮ ሳንቲም".
        //
        // Note this is AM's *only* test. It never asks Base's
        // `isinstance(val, float) or "." in str(val)` question, so
        // `has_decimal` is deliberately discarded below — `Decimal("5")`
        // carries `has_decimal == false` yet still prints "አምስት ዶላር ከ ዜሮ ሳንቲም"
        // here, where `default_to_currency` would suppress the cents. Verified
        // against the live interpreter.
        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // `has_fractional_cents` is computed on the float branch only, and it
        // doubles as Python's `isinstance(right, Decimal)` discriminator below:
        // `parse_currency_parts` returns a Decimal `cents` exactly when
        // `keep_precision` is set, and a plain int otherwise.
        let (left, right, is_negative, has_fractional_cents) = match val {
            // left = abs(val); right = 0; is_negative = val < 0
            CurrencyValue::Int(v) => (v.abs(), BigDecimal::zero(), v.is_negative(), false),
            CurrencyValue::Decimal { value, .. } => {
                // has_fractional_cents = (Decimal(str(val)) * 100) % 1 != 0.
                // The value is a whole number of cents iff scaling by 100
                // leaves nothing after the point. `with_scale(0)` truncates
                // and Decimal's `%` takes the sign of the dividend, so the
                // `!= 0` answer agrees on negatives either way.
                let scaled = value * BigDecimal::from(CENTS_DIVISOR);
                let has_fc = &scaled - scaled.with_scale(0) != BigDecimal::zero();
                // Python passes no `divisor=`, taking the 100 default. Note
                // the ROUND_HALF_UP quantize inside is a no-op for AM: it only
                // runs when has_fc is False, i.e. when the value already has
                // at most two decimal places.
                let (l, r, neg) = parse_currency_parts(val, false, has_fc, CENTS_DIVISOR);
                (l, r, neg, has_fc)
            }
        };

        // `cr1, cr2, default_separator = self.CURRENCY_FORMS[currency]` — the
        // 3-element unpack — inside a `try` that maps KeyError to
        // NotImplementedError. This runs *before* `to_cardinal`, so an unknown
        // code beats an overflowing value to the raise:
        // `to_currency(10**40, "EUR")` is NotImplementedError, not Overflow.
        let entry = self
            .currency
            .get(currency)
            .ok_or_else(|| self.currency_not_implemented(currency))?;
        let cr1 = &entry.forms.unit;
        let cr2 = &entry.forms.subunit;

        // `if separator == ",": separator = default_separator`.
        //
        // Python tests the *value*, not whether the kwarg was supplied, so an
        // explicit `separator=","` is swapped out too. `None` means the caller
        // omitted it; resolving that through `default_separator()` (Base's
        // ",") before the test reproduces both cases with one comparison.
        let mut separator = separator.unwrap_or(self.default_separator());
        if separator == "," {
            separator = entry.separator;
        }

        // negword is "ሰልቢ " with a trailing space; `"%s " % negword.strip()`
        // normalises it back to exactly one.
        let minus_str = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        let money_str = self.to_cardinal(&left)?;

        if is_integer_input {
            // Integer: no cents. `cents=` and `adjective=` are both ignored.
            return Ok(format!(
                "{}{} {}",
                minus_str,
                money_str,
                self.pluralize(&left, cr1)?
            ));
        }

        // For floats, always show cents — even if zero.
        let right_int = right.as_bigint_and_exponent().0;
        let cents_str = if cents {
            if has_fractional_cents {
                // `self.to_cardinal_float(float(right)) if right > 0 else "ዜሮ"`.
                // `cardinal_from_decimal` is exactly that: it casts to f64 and
                // derives precision from the repr, which is what Python's
                // `float(right)` plus `float2tuple`'s float arm do together.
                if right > BigDecimal::zero() {
                    self.cardinal_from_decimal(&right)?
                } else {
                    ZERO_TXT.to_string()
                }
            } else if right_int.is_positive() {
                self.to_cardinal(&right_int)?
            } else {
                ZERO_TXT.to_string()
            }
        } else {
            // `str(float(right) if isinstance(right, Decimal) else right)`.
            if has_fractional_cents {
                // right is in [0, 100), so the cast cannot overflow. Python's
                // float() would yield inf if it ever did, and str(inf) ==
                // "inf" matches Rust's Display for f64::INFINITY.
                format!("{}", right.to_f64().unwrap_or(f64::INFINITY))
            } else {
                right_int.to_string()
            }
        };

        // `self.pluralize(right, cr2)`.
        //
        // `right` is a Python int on the whole-cents branch and a Decimal on
        // the fractional one; EUR's pluralize only asks `n == 1`, which is
        // defined for both. The trait hook is typed `&BigInt`, so the Decimal
        // branch is answered inline — and its answer is never in doubt:
        // keep_precision is set exactly when `right` has a non-zero fractional
        // part, so `right == 1` is impossible there and the index is always 1.
        let cents_word = if has_fractional_cents {
            cr2.get(1)
                .cloned()
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?
        } else {
            self.pluralize(&right_int, cr2)?
        };

        // "%s%s %s%s %s %s" — the separator is glued to the unit word with no
        // space of its own, which is why AM's own " ከ" carries a leading one.
        // A caller-supplied " ና " therefore double-spaces; see module docs (12).
        Ok(format!(
            "{}{} {}{} {} {}",
            minus_str,
            money_str,
            self.pluralize(&left, cr1)?,
            separator,
            cents_str,
            cents_word
        ))
    }

    // AM's CURRENCY_FORMS entries are 3-tuples, so Base's `cr1, _cr2 = ...` unpack
    // raised ValueError for every known code.
    // No cheque rules, so NotImplementedError ("lang='am' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod float_tests {
    use super::*;
    use crate::floatpath::FloatValue;
    use std::str::FromStr;

    /// Drive the float arm the way the binding does.
    ///
    /// `precision` is *not* derived here: Python computes it as
    /// `abs(Decimal(repr(value)).as_tuple().exponent)`, which depends on
    /// `repr(float)` and is deliberately not reimplemented in the core. Every
    /// value below carries the precision the live interpreter reported for it.
    fn f(value: f64, precision: u32) -> String {
        LangAm::new()
            .to_cardinal_float(&FloatValue::Float { value, precision }, None)
            .unwrap()
    }

    /// Drive the Decimal arm — exact arbitrary precision, never an f64 cast.
    fn d(s: &str, precision: u32) -> String {
        LangAm::new()
            .to_cardinal_float(
                &FloatValue::Decimal {
                    value: BigDecimal::from_str(s).unwrap(),
                    precision,
                },
                None,
            )
            .unwrap()
    }

    /// Every `"to": "cardinal"` corpus row for `am` whose `arg` has a dot and
    /// is non-integral, i.e. every row that actually reaches the float path.
    /// ("0.0" and "1.0" are corpus rows too, but they satisfy
    /// `int(v) == v` and take the integer branch — see `int_valued_floats`.)
    #[test]
    fn corpus_cardinal_float() {
        let rows: &[(f64, u32, &str)] = &[
            (0.5, 1, "ዜሮ ነጥብ አምስት"),
            (1.5, 1, "አንድ ነጥብ አምስት"),
            (2.25, 2, "ሁለት ነጥብ ሁለት አምስት"),
            (3.14, 2, "ሦስት ነጥብ አንድ አራት"),
            (0.01, 2, "ዜሮ ነጥብ ዜሮ አንድ"),
            (0.1, 1, "ዜሮ ነጥብ አንድ"),
            (0.99, 2, "ዜሮ ነጥብ ዘጠኝ ዘጠኝ"),
            (1.01, 2, "አንድ ነጥብ ዜሮ አንድ"),
            (12.34, 2, "አሥራ ሁለት ነጥብ ሦስት አራት"),
            (99.99, 2, "ዘጠና ዘጠኝ ነጥብ ዘጠኝ ዘጠኝ"),
            // `pre == 100` → AM's `to_cardinal` short-circuit, reached via the
            // float path's callback. "መቶ", not "አንድ መቶ".
            (100.5, 1, "መቶ ነጥብ አምስት"),
            (1234.56, 2, "አንድ ሺህ ሁለት መቶ ሠላሳ አራት ነጥብ አምስት ስድስት"),
            // pre == 0 → the sign comes from to_cardinal_float, not to_cardinal.
            (-0.5, 1, "ሰልቢ ዜሮ ነጥብ አምስት"),
            // pre != 0 → the sign comes from AM's negword recursion.
            (-1.5, 1, "ሰልቢ አንድ ነጥብ አምስት"),
            (-12.34, 2, "ሰልቢ አሥራ ሁለት ነጥብ ሦስት አራት"),
            // The f64-artefact pair. 1.005 → post 4.99999999999989…,
            // 2.675 → post 674.9999999999998; the `< 0.01` heuristic in
            // float2tuple rounds both back up rather than flooring them.
            (1.005, 3, "አንድ ነጥብ ዜሮ ዜሮ አምስት"),
            (2.675, 3, "ሁለት ነጥብ ስድስት ሰባት አምስት"),
        ];
        for &(v, p, want) in rows {
            assert_eq!(f(v, p), want, "float {}", v);
        }
    }

    /// Every `"to": "cardinal_dec"` corpus row for `am` — Decimal input.
    #[test]
    fn corpus_cardinal_dec() {
        let rows: &[(&str, u32, &str)] = &[
            ("0.01", 2, "ዜሮ ነጥብ ዜሮ አንድ"),
            // Trailing zero is significant: Decimal("1.10") has exponent -2, so
            // precision is 2 and the final "ዜሮ" is spoken.
            ("1.10", 2, "አንድ ነጥብ አንድ ዜሮ"),
            ("12.345", 3, "አሥራ ሁለት ነጥብ ሦስት አራት አምስት"),
            (
                "98746251323029.99",
                2,
                "ዘጠና ስምንት ትሪሊዮን ሰባት መቶ አርባ ስድስት ቢሊዮን ሁለት መቶ አምሳ አንድ ሚሊዮን \
                 ሦስት መቶ ሃያ ሦስት ሺህ ሃያ ዘጠኝ ነጥብ ዘጠኝ ዘጠኝ",
            ),
            ("0.001", 3, "ዜሮ ነጥብ ዜሮ ዜሮ አንድ"),
        ];
        for &(s, p, want) in rows {
            assert_eq!(d(s, p), want, "decimal {}", s);
        }
    }

    /// Issue #603, visible through Amharic: the same literal takes different
    /// arms and must not agree. As an f64 it is already `…029.98` at trillion
    /// scale (that *is* its shortest round-trip repr), so it ends "ዘጠኝ ስምንት";
    /// as a Decimal it keeps the exact .99 and ends "ዘጠኝ ዘጠኝ".
    #[test]
    fn float_and_decimal_arms_differ_at_trillion_scale() {
        let as_float = f(98746251323029.99, 2);
        let as_decimal = d("98746251323029.99", 2);
        assert!(as_float.ends_with("ነጥብ ዘጠኝ ስምንት"), "{}", as_float);
        assert!(as_decimal.ends_with("ነጥብ ዘጠኝ ዘጠኝ"), "{}", as_decimal);
        assert_ne!(as_float, as_decimal);
    }

    /// Live-interpreter rows the corpus does not carry.
    #[test]
    fn extra_live_interpreter_rows() {
        // Negative *and* pre == -100: both AM divergences at once — the
        // recursion supplies "ሰልቢ ", and the recursive call's own `== 100`
        // short-circuit supplies the bare "መቶ".
        assert_eq!(f(-100.5, 1), "ሰልቢ መቶ ነጥብ አምስት");
        // Negative with pre == 0 and a leading zero in the fraction.
        assert_eq!(f(-0.01, 2), "ሰልቢ ዜሮ ነጥብ ዜሮ አንድ");
        // repr(1e-3) == "0.001" → precision 3, not 1.
        assert_eq!(f(0.001, 3), "ዜሮ ነጥብ ዜሮ ዜሮ አንድ");
        // A large float: exercises the high cards through the float path.
        assert_eq!(
            f(12345678901234.5, 1),
            "አሥራ ሁለት ትሪሊዮን ሦስት መቶ አርባ አምስት ቢሊዮን ስድስት መቶ ሰባ ስምንት ሚሊዮን \
             ዘጠኝ መቶ አንድ ሺህ ሁለት መቶ ሠላሳ አራት ነጥብ አምስት"
        );
    }

    /// Integral floats reach `to_cardinal_float` only if a caller calls it
    /// directly — `num2words` sends them down the integer branch instead.
    /// Pinned because the two answers differ and the split is easy to lose.
    #[test]
    fn int_valued_floats() {
        assert_eq!(f(2.0, 1), "ሁለት ነጥብ ዜሮ");
        assert_eq!(f(100.0, 1), "መቶ ነጥብ ዜሮ");
        // …whereas the integer branch, which is what "1.0" → "አንድ" records.
        assert_eq!(LangAm::new().to_cardinal(&BigInt::from(1)).unwrap(), "አንድ");
    }
}
