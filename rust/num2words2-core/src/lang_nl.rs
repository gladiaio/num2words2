//! Port of `lang_NL.py` (via `lang_EUR` → `Num2Word_Base`).
//!
//! Engine-style: NL supplies cards + `merge` and lets the base `to_cardinal`
//! drive `splitnum`/`clean`.
//!
//! NL keeps EUR's long-scale `set_high_numwords` (step -6, pairing an
//! `iljard` at 10^n with an `iljoen` at 10^(n-3)) and EUR's
//! `gen_high_numwords`, but replaces the Latin stems with Dutch-flavoured
//! ones ("dez"/"okto"/"tre"/"quin"/"sept" instead of
//! "dec"/"octo"/"tres"/"quint"/"septen") and swaps the "cent" head for
//! "zend". Only five of EUR's eighteen elision rules can still fire against
//! the NL stems (novemn, novemo, sexn, sexs, unno) — the rest are dead code
//! here, but the list is ported whole because Python applies it whole.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result,
};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, float2tuple, FloatValue};
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Kept private to this module: `lang_en.rs` exports an identical function,
/// but the porting contract is one self-contained file per language.
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — tens is the outer loop.
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
    for &(k, v) in REPLACEMENTS {
        out = out.iter().map(|o| o.replace(k, v)).collect();
    }
    out.extend(lows.iter().map(|s| s.to_string()));
    out
}

/// `self.ords`, in Python dict insertion order.
///
/// `to_ordinal` scans this with `break` on the first suffix hit, so the order
/// is load-bearing: "erd" must precede "rd" ("honderd" → "honderdst", not
/// "hondeerdst"), and the spelled-out digits must precede the generic "ig" /
/// "end" / "joen" suffixes.
const ORDS: &[(&str, &str)] = &[
    ("nul", "nuld"),
    ("één", "eerst"),
    ("twee", "tweed"),
    ("drie", "derd"),
    ("vier", "vierd"),
    ("vijf", "vijfd"),
    ("zes", "zesd"),
    ("zeven", "zevend"),
    ("acht", "achtst"),
    ("negen", "negend"),
    ("tien", "tiend"),
    ("elf", "elfd"),
    ("twaalf", "twaalfd"),
    ("ig", "igst"),
    ("erd", "erdst"),
    ("end", "endst"),
    ("joen", "joenst"),
    ("rd", "rdst"),
];

/// `Num2Word_NL.CURRENCY_FORMS`.
///
/// Unlike the 16 classes that read `Num2Word_EUR`'s dict after
/// `Num2Word_EN.__init__` has mutated it in place, NL declares
/// `CURRENCY_FORMS` in its **own** class body. That shadows EUR's attribute
/// outright, so NL never sees either EUR's 22 entries or the ~24 codes EN
/// grafts on — the live interpreter confirms `Num2Word_NL.CURRENCY_FORMS is
/// Num2Word_EUR.CURRENCY_FORMS` is False and holds exactly these four keys.
///
/// Hence JPY, KWD, BHD, INR and CHF all raise NotImplementedError for NL,
/// which the corpus asserts row for row. Adding them "for completeness" would
/// break 60 cases.
///
/// The subunit arity is load-bearing. `Num2Word_NL.pluralize` returns
/// `forms[0]` unconditionally, so the ordinary cents path never reaches the
/// second form — but `base.to_currency`'s fractional-cents branch takes
/// `cr2[1]` *directly*, bypassing `pluralize` entirely. That splits GBP and
/// CNY across the two paths:
///
/// ```text
/// to_currency(12.34, "GBP") -> "twaalf pond en vierendertig penny"
/// to_currency(1.011, "GBP") -> "één pond en één komma één pence"
/// to_currency(1.011, "CNY") -> "één yuan en één komma één fen"
/// ```
///
/// So dropping "pence"/"fen" as apparently-dead plurals would silently change
/// output on any value with fractional subunits. All three verified against
/// the live interpreter.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cent"]));
    m.insert("GBP", CurrencyForms::new(&["pond", "pond"], &["penny", "pence"]));
    m.insert(
        "USD",
        CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"]),
    );
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuan"], &["jiao", "fen"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — NL declares no
/// adjectives of its own, and EN mutates only `CURRENCY_FORMS`, never this.
///
/// Nearly all of it is unreachable: `base.to_currency` looks up
/// `CURRENCY_FORMS` *before* consulting the adjectives, so any code outside
/// NL's four raises NotImplementedError first. Of those four only USD has an
/// entry, making `to_currency(12.34, "USD", adjective=True)` -> "twaalf US
/// dollar en vierendertig cent" the single reachable case (verified against
/// the live interpreter). The full table is kept because it is the inherited
/// data, exactly as `lang_en.rs` keeps it.
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

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the fractional
/// digit count of the shortest round-trip repr. Mirrors the private helper in
/// `floatpath`.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Best-effort `str(number)` for the TypeError messages (the corpus compares
/// exception types only, so the float arm's repr need not be byte-exact).
fn py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

pub struct LangNl {
    cards: Cards,
    maxval: BigInt,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangNl {
    fn default() -> Self {
        Self::new()
    }
}

impl LangNl {
    pub fn new() -> Self {
        let lows = ["non", "okt", "sept", "sext", "quint", "quadr", "tr", "b", "m"];
        let units = [
            "", "un", "duo", "tre", "quattuor", "quin", "sex", "sept", "okto", "novem",
        ];
        let tens = [
            "dez",
            "vigint",
            "trigint",
            "quadragint",
            "quinquagint",
            "sexagint",
            "septuagint",
            "oktogint",
            "nonagint",
        ];
        // NL.setup() calls super().setup() (which sets high_numwords with the
        // "cent" head) and then overwrites high_numwords outright, so only
        // this assignment survives.
        let mut high = vec!["zend".to_string()];
        high.extend(gen_high_numwords(&units, &tens, &lows));

        let mut cards = Cards::new();

        // EUR.set_high_numwords: cap = 3 + 6*len(high); zip(high, range(cap, 3, -6)).
        // GIGA_SUFFIX = "iljard", MEGA_SUFFIX = "iljoen" — both non-empty for
        // NL, so every step inserts a pair. len(high) is 100 and the range
        // yields exactly 100 values (603 down to 9), so zip truncates neither.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n as u32), format!("{}iljard", word));
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}iljoen", word),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "duizend"),
                (100, "honderd"),
                (90, "negentig"),
                (80, "tachtig"),
                (70, "zeventig"),
                (60, "zestig"),
                (50, "vijftig"),
                (40, "veertig"),
                (30, "dertig"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "twintig",
                "negentien",
                "achttien",
                "zeventien",
                "zestien",
                "vijftien",
                "veertien",
                "dertien",
                "twaalf",
                "elf",
                "tien",
                "negen",
                "acht",
                "zeven",
                "zes",
                "vijf",
                "vier",
                "drie",
                "twee",
                "één",
                "nul",
            ],
        );

        // MAXVAL = 1000 * first card key. Python's OrderedDict is filled
        // high-first, so key[0] is 10^603 — the same as the max key.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangNl {
            cards,
            maxval,
            // Built once here, never per call: `to_currency` only reads these,
            // and rebuilding them per call is what made an earlier revision of
            // this port slower than the Python it replaces.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal` for a float/Decimal: the float check
    /// (`errmsg_floatord`) fires first, then the negative one — both with
    /// NL's Dutch wording. `-0.0` passes both (`abs(-0.0) == -0.0`).
    /// Returns the whole value on success.
    fn verify_ordinal_num(&self, value: &FloatValue) -> Result<BigInt> {
        let whole = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Het zwevende puntnummer {} kan niet omgezet worden naar een ordernummer.",
                    py_num_str(value)
                )))
            }
        };
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Het negatieve getal {} kan niet omgezet worden naar een ordernummer.",
                py_num_str(value)
            )));
        }
        Ok(whole)
    }

    /// Python's `self.to_cardinal(x)` for an f64 — Base semantics: whole →
    /// integer path, fractional → the base float grammar ("komma" + digits).
    fn cardinal_num_f64(&self, x: f64) -> Result<String> {
        if x.fract() == 0.0 {
            if let Some(i) = BigInt::from_f64(x) {
                return self.to_cardinal(&i);
            }
        }
        default_to_cardinal_float(
            self,
            &FloatValue::Float {
                value: x,
                precision: float_repr_precision(x),
            },
            None,
        )
    }

    /// Python's `self.to_cardinal(x)` for a Decimal.
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

    /// `Num2Word_Base.verify_ordinal` with NL's `errmsg_negord`.
    /// The float branch is out of scope (integer input only).
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Het negatieve getal {} kan niet omgezet worden naar een ordernummer.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_Base.to_splitnum` specialised to NL's only call site:
    /// `hightxt="honderd"`, `lowtxt=""`, `jointxt=""`, `divisor=100`,
    /// `longval=True`, `cents=True`.
    ///
    /// `inflect(high, "honderd")` returns "honderd" for every `high`
    /// ("honderd".split("/") has one element, so both branches agree), and
    /// `title` is the identity because NL leaves `is_title` False.
    fn to_splitnum(&self, val: &BigInt) -> Result<String> {
        // Python's divmod is floor-based; this matters for negative years,
        // e.g. divmod(-44, 100) == (-1, 56).
        let (high, low) = val.div_mod_floor(&BigInt::from(100));
        let mut out: Vec<String> = Vec::new();

        if !high.is_zero() {
            out.push(self.to_cardinal(&high)?);
            if !low.is_zero() {
                // longval is True and hightxt is non-empty; jointxt is ""
                // so the join segment is skipped.
                out.push("honderd".to_string());
            } else {
                out.push("honderd".to_string());
            }
        }

        if !low.is_zero() {
            // cents=True, so the low part is spelled out rather than "%02d".
            out.push(self.to_cardinal(&low)?);
        }

        Ok(out.join(" "))
    }
}

impl Lang for LangNl {
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
        " en"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "min "
    }
    fn pointword(&self) -> &str {
        "komma"
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // NL overrides errmsg_toobig, so the overflow check is done here with
        // NL's wording rather than leaving it to `default_to_cardinal`'s
        // base-class message. The predicate is identical, so the base check
        // can never fire after this one passes.
        let v = value.abs();
        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "Het getal {} moet minder zijn dan {}.",
                v,
                self.maxval()
            )));
        }
        default_to_cardinal(self, value)
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let ten = BigInt::from(10);
        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            // "één miljoen" would be wrong; the multiplier drops its accents.
            ctext = "een".to_string();
        }

        let val;
        if nnum > cnum {
            if nnum >= &million {
                ctext.push(' ');
            }
            val = cnum * nnum;
        } else {
            // Python's chained `nnum < 10 < cnum < 100`.
            if nnum < &ten && ten < *cnum && cnum < &hundred {
                if nnum.is_one() {
                    ntext = "een".to_string();
                }
                if ntext.ends_with('e') {
                    ntext.push_str("ën");
                } else {
                    ntext.push_str("en");
                }
                // Python: `ntext, ctext = ctext, ntext` — the unit word moves
                // in front of the ten ("eenentwintig").
                std::mem::swap(&mut ntext, &mut ctext);
            } else if cnum >= &thousand {
                ctext.push(' ');
            }
            val = cnum + nnum;
        }

        (format!("{}{}", ctext, ntext), val)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let mut outword = self.to_cardinal(value)?;
        for &(key, rep) in ORDS {
            // Python slices `outword[:len(outword) - len(key)]` by character
            // count. Because the suffix matched, that split point is exactly
            // where `strip_suffix` cuts — no byte/char discrepancy survives
            // (this matters for "één": 3 chars, 5 bytes).
            if let Some(head) = outword.strip_suffix(key) {
                outword = format!("{}{}", head, rep);
                break;
            }
        }
        Ok(outword + "e")
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}e", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        let hundred = BigInt::from(100);

        // Years shaped "[N]honderd[M]" (1100..9999), written without spaces —
        // except the 2000..2099 window, which keeps the "tweeduizend X" form.
        if *value >= BigInt::from(1100)
            && *value <= BigInt::from(9999)
            && !(*value >= BigInt::from(2000) && *value <= BigInt::from(2099))
        {
            let hundreds = value / &hundred;
            let remainder = value % &hundred;
            let mut result = self.to_cardinal(&hundreds)?.replace(' ', "") + "honderd";
            if !remainder.is_zero() {
                result.push_str(&self.to_cardinal(&remainder)?.replace(' ', ""));
            }
            return Ok(result);
        }

        // `if not (val // 100) % 10` — Python floor-division and Python
        // modulo (result takes the divisor's sign), hence div_floor/mod_floor.
        // For val = -44 this is (-1) % 10 == 9, which is truthy, so negative
        // years fall through to the to_splitnum branch.
        if (value.div_floor(&hundred))
            .mod_floor(&BigInt::from(10))
            .is_zero()
        {
            return self.to_cardinal(value);
        }

        self.to_splitnum(value)
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for any
    /// fractional or negative value; a whole non-negative one flows through
    /// `to_cardinal` + the `ords` suffix table — identical to the integer
    /// ordinal ("5.0" → "vijfde", `1e+16` → "tien biljardste").
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        self.to_ordinal(&whole)
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal` then
    /// `str(value) + "e"` — "5.0e", "1E+2e", "-0.0e".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        self.verify_ordinal_num(value)?;
        Ok(format!("{}e", repr_str))
    }

    /// `to_year(float/Decimal)` — `Num2Word_NL.to_year`'s arithmetic run on
    /// the raw float/Decimal, quirks and all:
    ///
    /// * 1100..=9999 (minus the 2000..2099 window): "[N]honderd[M]" from
    ///   `val // 100` and `val % 100` — `1234.0` → "twaalfhonderdvierendertig".
    /// * hundreds digit zero (`not (val // 100) % 10`): plain cardinal.
    /// * otherwise `to_splitnum(val, hightxt="honderd")`, whose **float** arm
    ///   goes through `float2tuple` — so `100.0` splits into `(100, 0)` and
    ///   reads "honderd honderd", and `-21.0` into `(-21, 0)` →
    ///   "min eenentwintig honderd" — while the **Decimal** arm divmods by
    ///   100: `Decimal("100")` → "één honderd",
    ///   `Decimal("12345.000")` → "honderddrieëntwintig honderd vijfenveertig".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: v, .. } => {
                let v = *v;
                if (1100.0..=9999.0).contains(&v) && !(2000.0..=2099.0).contains(&v) {
                    let hundreds = (v / 100.0).floor();
                    let remainder = v.rem_euclid(100.0);
                    let mut result =
                        format!("{}honderd", self.cardinal_num_f64(hundreds)?.replace(' ', ""));
                    if remainder != 0.0 {
                        result.push_str(&self.cardinal_num_f64(remainder)?.replace(' ', ""));
                    }
                    return Ok(result);
                }
                if (v / 100.0).floor().rem_euclid(10.0) == 0.0 {
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum, float arm: high/low from base.float2tuple —
                // int() truncation for `pre`, repr digits for `post`.
                let (high, low) = float2tuple(value);
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    out.push(self.to_cardinal(&high)?);
                    out.push("honderd".to_string());
                }
                if !low.is_zero() {
                    out.push(self.to_cardinal(&low)?);
                }
                Ok(out.join(" "))
            }
            FloatValue::Decimal { value: d, .. } => {
                let hundred = BigDecimal::from(100);
                let lo = BigDecimal::from(1100);
                let hi = BigDecimal::from(9999);
                let w_lo = BigDecimal::from(2000);
                let w_hi = BigDecimal::from(2099);
                if d >= &lo && d <= &hi && !(d >= &w_lo && d <= &w_hi) {
                    // Decimal // and % truncate toward zero; positive here.
                    let hundreds = (d / &hundred).with_scale_round(
                        0,
                        bigdecimal::RoundingMode::Down,
                    );
                    let remainder = d - &hundreds * &hundred;
                    let mut result = format!(
                        "{}honderd",
                        self.cardinal_num_dec(&hundreds)?.replace(' ', "")
                    );
                    if !remainder.is_zero() {
                        result.push_str(&self.cardinal_num_dec(&remainder)?.replace(' ', ""));
                    }
                    return Ok(result);
                }
                let q = (d / &hundred).with_scale_round(0, bigdecimal::RoundingMode::Down);
                let q_int = q.as_bigint_and_exponent().0;
                // Python `%` on the int-valued Decimal quotient: sign of the
                // dividend (truncated), matching Rust's BigInt `%`.
                if (&q_int % BigInt::from(10)).is_zero() {
                    return self.cardinal_float_entry(value, None);
                }
                // to_splitnum, Decimal arm: `high, low = divmod(val, 100)`.
                let high = q;
                let low = d - &high * &hundred;
                let mut out: Vec<String> = Vec::new();
                if !high.is_zero() {
                    out.push(self.cardinal_num_dec(&high)?);
                    out.push("honderd".to_string());
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
    // NL overrides only `to_currency` and `pluralize`; `to_cheque`,
    // `_money_verbose`, `_cents_verbose` and `_cents_terse` all come from
    // `Num2Word_Base` unchanged, and `CURRENCY_PRECISION` stays base's empty
    // dict (EN *rebinds* it on the instance rather than mutating the class
    // attribute, so its 3-decimal entries never leak here). Every code is
    // therefore divisor 100 and the trait defaults already match — hence no
    // `currency_precision`, `money_verbose` or `to_cheque` override below.

    fn lang_name(&self) -> &str {
        "Num2Word_NL"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_NL.pluralize`: `return forms[0]` — the plural form is *never*
    /// selected, overriding `Num2Word_EUR`'s `forms[0 if n == 1 else 1]`.
    ///
    /// The docstring cites onzetaal.nl on euro/euro's and admits uncertainty
    /// about the scope of the rule ("not sure if it's applied only to euro").
    /// Its practical effect is that GBP reads "vierendertig penny" rather than
    /// "...pence", and CNY "vierendertig jiao" rather than "...fen". The
    /// corpus pins both of those singular readings, so this is behaviour to
    /// preserve, not a wrinkle to fix. (The plurals are still reachable via
    /// the fractional-cents branch, which skips this method — see
    /// `build_currency_forms`.)
    ///
    /// Python indexes the tuple directly, so an empty entry would raise
    /// IndexError. Unreachable for NL's table — mapped rather than panicking
    /// so the exception type survives if the data ever changes.
    fn pluralize(&self, _n: &BigInt, forms: &[String]) -> Result<String> {
        forms
            .first()
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_NL.to_currency`.
    ///
    /// NL intercepts the `isinstance(val, int)` case with its own body and
    /// delegates everything else to `Num2Word_Base.to_currency`. The int arm
    /// diverges from base's in three ways, all of them bugs kept for parity:
    ///
    /// 1. `minus_str = self.negword` keeps negword's **trailing space** where
    ///    base does `"%s " % self.negword.strip()`, so Python's
    ///    `to_currency(-1)` is `"min  één euro"`. Fixed here (#160): the port
    ///    strips negword and says `"min één euro"`.
    /// 2. `adjective` is accepted and then **ignored** — the int arm never
    ///    calls `prefix_currency`. So `to_currency(1, "USD", adjective=True)`
    ///    is "één dollar" while the float `1.0` gives "één US dollar en nul
    ///    cent".
    /// 3. The arm inlines a plural rule (`cr1[0] if abs_val == 1 else cr1[1]`)
    ///    that contradicts NL's own `pluralize` (always `forms[0]`). Harmless
    ///    in practice: every NL unit tuple has cr1[0] == cr1[1].
    ///
    /// The `except (KeyError, AttributeError)` fallback re-enters base for an
    /// unknown code, which raises NotImplementedError with NL's class name.
    /// `AttributeError` is unreachable (`CURRENCY_FORMS` always exists), so
    /// only the missing-key path is modelled.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait hands us None when the caller omitted separator=; resolve it
        // to NL's own default (" en") before the ported body runs.
        let separator = separator.unwrap_or(self.default_separator());

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

            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            let currency_str = if abs_val.is_one() {
                &cr1[0]
            } else if cr1.len() > 1 {
                &cr1[1]
            } else {
                &cr1[0]
            };

            // Python: ("%s %s %s" % (...)).strip() — see bug 1 above.
            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // Floats/Decimals: `super(Num2Word_NL, self).to_currency(...)`.
        default_to_currency(self, val, currency, cents, separator, adjective)
    }
}
