//! Port of `lang_SV.py` (`Num2Word_SV`), via `lang_EUR` → `Num2Word_Base`.
//!
//! Engine-style language: SV supplies `cards` + `merge` and lets the base
//! `to_cardinal` drive `splitnum`/`clean`. It overrides `to_ordinal`,
//! `to_ordinal_num` and `to_year` outright.
//!
//! SV redefines `set_high_numwords` with a body byte-identical to
//! `Num2Word_EUR`'s; the only real change is the suffix pair
//! `GIGA_SUFFIX = "iljarder"` / `MEGA_SUFFIX = "iljoner"`, which yields the
//! Swedish long scale: 10^6 "miljoner", 10^9 "miljarder", 10^12 "biljoner",
//! 10^15 "biljarder", … up to 10^600 "centiljoner" / 10^603 "centiljarder".
//! MAXVAL is therefore 10^606 — far past u64/i128, so everything stays BigInt.
//!
//! # Faithfully reproduced Python bugs
//!
//! Per the porting contract these are preserved verbatim, not fixed:
//!
//! 1. ~~**`ords["tjugo"]` was unreachable.**~~ `to_ordinal` looked the last
//!    word up by its last 4 chars, then its last 3, so the 5-char "tjugo"
//!    never matched and 20 read "tjugode". Fixed (gladiaio/num2words2#252):
//!    the probe tries 5 chars first, so 20 is "tjugonde" and 120
//!    "etthundratjugonde".
//!
//! 2. ~~**Plural mega/giga words got ordinalised as-is.**~~ The cards store
//!    the *plural* "miljoner"/"miljarder", and `merge` only singularises them
//!    on the `lnum == 1` path ("en miljon"). So `to_ordinal(2*10**6)` was
//!    "två miljonerde" and `to_ordinal(10**9)` "en miljardde". Fixed (#252,
//!    #259): a trailing scale word takes "te" on its singular stem, and a
//!    bare scale ordinal is one word — "miljonte", "tvåmiljonte",
//!    "miljardte", "tiobiljonte".
//!
//! 3. **`merge` returns `lnum + rnum` where a product is meant.** The
//!    `rnum >= 1000000` branches precede the `rnum > lnum` multiply branch, so
//!    merging ("ett",1) with ("miljoner",10^6) yields num 1_000_001, not
//!    10^6. The bogus totals never reach the output text (only the string is
//!    used at the top level, and the subsequent comparisons happen to land in
//!    the same branches), so cardinals are unaffected — but the numeric field
//!    of the tree is genuinely wrong and is reproduced as such.
//!
//! 4. **`to_ordinal_num` skips `verify_ordinal`.** Negatives are accepted:
//!    `to_ordinal_num(-1)` → "-1:a", `to_ordinal_num(-42)` → "-42:a". Python
//!    exempted only 11 and 12 from ":a", so 111 was "111:a" though the
//!    ordinal is "etthundraelfte"; the suffix now follows the last two digits
//!    (#224, see [`sv_ordinal_num_suffix`]).
//!
//! 5. **SV's `to_currency` double-spaced the minus on the int path (fixed,
//!    #160).** Python builds `"%s %s %s" % (minus_str, money, unit)` with the
//!    *un-stripped* `self.negword`, so `to_currency(-10, "EUR")` was
//!    "minus  tio euros". The port strips it, as the float path does:
//!    "minus tio euro" (Swedish nouns since #222).
//!
//! 6. **SV's int path silently ignores `adjective=`.** `Num2Word_Base`'s int
//!    path applies `prefix_currency`, but SV's override never consults
//!    `CURRENCY_ADJECTIVES`. So `to_currency(2, "USD", adjective=True)` is
//!    "två dollar" while `to_currency(2.0, "USD", adjective=True)` is
//!    "två US dollar, noll cent" (the adjectives are still English). `cents=` and `separator=` are likewise
//!    dead on the int path. `CURRENCY_ADJECTIVES` is still wired up below
//!    because the float path (via the base) does honour it.
//!
//! # Where SV's currency table actually comes from
//!
//! `Num2Word_SV` defines no `CURRENCY_FORMS` of its own and inherits
//! `Num2Word_EUR`'s. That dict is a *class* attribute, and
//! `Num2Word_EN.__init__` does `self.CURRENCY_FORMS["CHF"] = ...` — a
//! subscript assignment, which mutates the inherited dict **in place** rather
//! than shadowing it with an instance copy. `num2words2/__init__.py`
//! instantiates every converter at import time, so importing the package
//! rewrites `Num2Word_EUR.CURRENCY_FORMS` for all of its subclasses, SV
//! included.
//!
//! **Replaced by native nouns (gladiaio/num2words2#222).** That mutated
//! table is English ("två dollars", "två euros"), so the port no longer
//! mirrors it: it keeps only codes with a sourced Swedish unit and subunit
//! noun ("två dollar", "två euro", "två kronor", "femtio öre") and raises NotImplementedError for the rest. The notes
//! below describe Python's table.
//!
//! That is why Python's table is not what `lang_EUR.py` reads:
//!
//! * EUR is `("euro", "euros")`, not lang_EUR's `("euro", "euro")`.
//! * GBP is `("pound", "pounds")`, not `("pound sterling", "pounds sterling")`.
//! * SAR is `("riyal", "riyals")`, not `("saudi riyal", "saudi riyals")`.
//! * 17 codes absent from lang_EUR (CHF, CNY, KWD, BHD, NGN, …) are present.
//!
//! The 39 entries here are transcribed from the live post-import state, which
//! is what the frozen corpus was generated against. `Num2Word_EN` is the only
//! class that pollutes the EUR dict, and it is instantiated unconditionally,
//! so the state is deterministic — but see the port notes: this is global
//! mutable state and a Python-side reordering would silently invalidate it.
//!
//! `CURRENCY_PRECISION` is *not* affected: EN sets it with `self.X = {...}`
//! (a plain assignment), which binds an instance attribute and leaves
//! `Num2Word_Base.CURRENCY_PRECISION` an empty dict. SV therefore resolves
//! every code to the default divisor of 100 — including the ones that are
//! 3-decimal (KWD/BHD/OMR/…) or 0-decimal (JPY/KRW) elsewhere. Hence
//! Python's `to_currency(12.34, "KWD")` → "tolv dinars, trettiofyra fils"
//! (cents, not mils; KWD now raises) and `to_currency(12.34, "JPY")` → "tolv
//! yen, trettiofyra sen"
//! rather than rounding to a whole yen. The corpus confirms both. So
//! `currency_precision` is deliberately left at the trait default.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use crate::strnum::python_decimal_str;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// Port of `Num2Word_EUR.gen_high_numwords`.
///
/// Reimplemented locally rather than imported from `lang_en` so this file
/// stays self-contained (the registry wiring is generated mechanically).
fn gen_high_numwords(units: &[&str], tens: &[&str], lows: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    // Python: [u + t for t in tens for u in units] — `tens` is the outer loop.
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

/// Python's `s[-n:]` — by character, and clamping instead of panicking when
/// the string is shorter than `n`. Swedish words carry å/ä/ö, so byte
/// slicing would corrupt them (and can panic mid-codepoint).
fn last_n_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    s.chars().skip(count.saturating_sub(n)).collect()
}

/// Python's `s[:-n]` — by character. `n` larger than the string yields "",
/// matching Python (`"ett"[:-4] == ""`), which the `ords` lookup relies on:
/// "ett" is found via its 4-char probe, then sliced by 4 down to "".
fn drop_last_n_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    s.chars().take(count.saturating_sub(n)).collect()
}

pub struct LangSv {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS` as it exists *after* package import — see the module
    /// docs. Built once here; constructing it per call is what made an
    /// earlier revision of this port 10x slower than the Python.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    /// `CURRENCY_ADJECTIVES`, inherited unmodified from `Num2Word_EUR`.
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangSv {
    fn default() -> Self {
        Self::new()
    }
}

impl LangSv {
    pub fn new() -> Self {
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

        // SV's set_high_numwords: cap = 3 + 6*len(high); zip(high, range(cap, 3, -6)).
        // Long scale, step -6: each word supplies BOTH a giga (10^n) and a
        // mega (10^(n-3)) card. len(high) == 100, so cap == 603 and the
        // range yields exactly 100 values (603, 597, …, 9) — zip consumes
        // every high word.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            // Both suffixes are non-empty for SV, so both branches always fire.
            cards.insert(
                BigInt::from(10u8).pow(n as u32),
                format!("{}{}", word, "iljarder"),
            );
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}{}", word, "iljoner"),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "tusen"),
                (100, "hundra"),
                (90, "nittio"),
                (80, "åttio"),
                (70, "sjuttio"),
                (60, "sextio"),
                (50, "femtio"),
                (40, "fyrtio"),
                (30, "trettio"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "tjugo", "nitton", "arton", "sjutton", "sexton", "femton", "fjorton", "tretton",
                "tolv", "elva", "tio", "nio", "åtta", "sju", "sex", "fem", "fyra", "tre", "två",
                "ett", "noll",
            ],
        );

        // MAXVAL = 1000 * highest card = 1000 * 10^603 = 10^606.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<&str, &str> = [
            ("noll", "nollte"),
            ("ett", "första"),
            ("två", "andra"),
            ("tre", "tredje"),
            ("fyra", "fjärde"),
            ("fem", "femte"),
            ("sex", "sjätte"),
            ("sju", "sjunde"),
            ("åtta", "åttonde"),
            ("nio", "nionde"),
            ("tio", "tionde"),
            ("elva", "elfte"),
            ("tolv", "tolfte"),
            ("tjugo", "tjugonde"),
        ]
        .into_iter()
        .collect();

        // Swedish nouns (#222). Python inherited Num2Word_EUR's table as
        // rewritten by Num2Word_EN, i.e. English ("två dollars", "två
        // euros"). Codes without a sourced Swedish unit *and* subunit are
        // left out and raise NotImplementedError.
        let currency_forms: HashMap<&'static str, CurrencyForms> = [
            ("AUD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
            ("CAD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
            ("CHF", CurrencyForms::new(&["franc", "francer"], &["rappen", "rappen"])),
            ("EUR", CurrencyForms::new(&["euro", "euro"], &["cent", "cent"])),
            ("GBP", CurrencyForms::new(&["pund", "pund"], &["penny", "pence"])),
            ("HKD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
            ("INR", CurrencyForms::new(&["rupie", "rupier"], &["paisa", "paise"])),
            ("JPY", CurrencyForms::new(&["yen", "yen"], &["sen", "sen"])),
            ("MXN", CurrencyForms::new(&["peso", "peso"], &["centavo", "centavos"])),
            ("NOK", CurrencyForms::new(&["krona", "kronor"], &["öre", "öre"])),
            ("NZD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
            ("RUB", CurrencyForms::new(&["rubel", "rubel"], &["kopek", "kopek"])),
            ("SEK", CurrencyForms::new(&["krona", "kronor"], &["öre", "öre"])),
            ("SGD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
            ("USD", CurrencyForms::new(&["dollar", "dollar"], &["cent", "cent"])),
        ]
        .into_iter()
        .collect();

        let currency_adjectives: HashMap<&'static str, &'static str> = [
            ("AUD", "Australian"),
            ("BYN", "Belarusian"),
            ("CAD", "Canadian"),
            ("EEK", "Estonian"),
            ("HUF", "Hungarian"),
            ("INR", "Indian"),
            ("ISK", "íslenskar"),
            ("JPY", "Japanese"),
            ("KRW", "Korean"),
            ("MXN", "Mexican"),
            ("NOK", "Norwegian"),
            ("RON", "Romanian"),
            ("RUB", "Russian"),
            ("SAR", "Saudi"),
            ("USD", "US"),
            ("UZS", "Uzbekistan"),
        ]
        .into_iter()
        .collect();

        LangSv {
            cards,
            maxval,
            ords,
            exclude_title: vec!["och".into(), "komma".into(), "minus".into()],
            currency_forms,
            currency_adjectives,
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`. The float check is vacuous for
    /// BigInt input; only the negative check can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.sign() == num_bigint::Sign::Minus {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `verify_ordinal` for a float/Decimal: the float check fires first,
    /// then the negative one — both TypeError with Base's default wording.
    /// `-0.0` passes both. Returns the whole value on success.
    fn verify_ordinal_num(&self, value: &FloatValue) -> Result<BigInt> {
        let whole = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Cannot treat float {} as ordinal.",
                    sv_py_num_str(value)
                )))
            }
        };
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                sv_py_num_str(value)
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
                precision: sv_float_repr_precision(x),
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
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64. Mirrors the
/// private helper in `floatpath`.
fn sv_float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// Best-effort `str(number)` for the TypeError messages (the corpus compares
/// exception types only, so the float arm's repr need not be byte-exact).
fn sv_py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// The `:a`/`:e` of a Swedish numeric ordinal: ":a" after first/second
/// (första/andra: last two digits ending in 1 or 2, but not 11/12, which
/// read "elfte"/"tolfte"), ":e" otherwise (#224). 111 is "111:e".
fn sv_ordinal_num_suffix(value: &BigInt) -> &'static str {
    let last_two = (value.abs() % BigInt::from(100)).to_u32().unwrap_or(0);
    if matches!(last_two % 10, 1 | 2) && !matches!(last_two, 11 | 12) {
        ":a"
    } else {
        ":e"
    }
}

impl Lang for LangSv {
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
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);
        let million = BigInt::from(1_000_000);

        if lnum.is_one() && rnum < &hundred {
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            (format!("{}{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            (format!("{}{}", ltext, rtext), lnum + rnum)
        } else if rnum >= &million && lnum.is_one() {
            // Singularise the plural card word: "miljoner" -> "miljon",
            // "miljarder" -> "miljard". Python: "%s %s" % ("en", rtext[:-2]).
            // Note the `lnum + rnum` — a product is clearly intended; see the
            // module docs, bug 3.
            (format!("en {}", drop_last_n_chars(rtext, 2)), lnum + rnum)
        } else if rnum >= &million && lnum > &BigInt::one() {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        } else if rnum > lnum {
            // Swedish drops "ett" before "hundra" in the hundred-thousand
            // compound: 100000 is "hundratusen", not "etthundratusen".
            if lnum == &hundred && rnum == &BigInt::from(1000) {
                return ("hundratusen".to_string(), BigInt::from(100_000));
            }
            // No triple consonants: "ett" + "tusen" = "etttusen" -> "ettusen";
            // likewise 21000 -> "tjugoettusen". Python's str.replace is
            // replace-all, and so is Rust's.
            (
                format!("{}{}", ltext, rtext).replace("ttt", "tt"),
                lnum * rnum,
            )
        } else {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();
        // to_cardinal never returns "", so split(' ') always yields >= 1 item.
        let lastword = outwords.last().unwrap().clone();

        // Probe ords by the last 4 chars, then the last 3. Python swallows the
        // KeyErrors; the final fallback is the generic "de" suffix with
        // ending_length left at its initial 0.
        // A trailing scale word is ordinalised on its singular stem, without
        // the "en" of "en miljon": "miljonte" (#252), "tvåmiljonte",
        // "miljardte" — Python glued "de" onto it: "två miljonerde", "en
        // miljardde" (#259).
        let stem = lastword
            .strip_suffix("er")
            .filter(|s| s.ends_with("jon") || s.ends_with("jard"))
            .unwrap_or(&lastword);
        if stem.ends_with("jon") || stem.ends_with("jard") {
            let n = outwords.len();
            outwords[n - 1] = format!("{}te", stem);
            // A bare scale ordinal is one word, like "tvåtusende":
            // "tvåmiljonte", and "miljonte" without the "en".
            if n == 2 {
                if outwords[0] == "en" {
                    outwords.remove(0);
                }
                return Ok(outwords.concat());
            }
            return Ok(outwords.join(" "));
        }
        // Python probed only 4 and 3 chars, so "tjugo" never matched and
        // 20 read "tjugode"; probe 5 first (#252).
        let mut ending_length: usize = 0;
        let mut lastword_ending = "de".to_string();
        for k in [5, 4, 3] {
            if let Some(e) = self.ords.get(last_n_chars(&lastword, k).as_str()) {
                ending_length = k;
                lastword_ending = (*e).to_string();
                break;
            }
        }

        // Python compares the *value* to "de", so an ords entry that happened
        // to equal "de" would also take the no-truncation path. None does.
        let lastword_first_part = if lastword_ending == "de" {
            self.title(&lastword)
        } else {
            drop_last_n_chars(&self.title(&lastword), ending_length)
        };

        let n = outwords.len();
        outwords[n - 1] = format!("{}{}", lastword_first_part, lastword_ending);
        Ok(outwords.join(" "))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        // No verify_ordinal call here — negatives pass straight through.
        Ok(format!("{}{}", value, sv_ordinal_num_suffix(value)))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        let thousand = BigInt::from(1000);
        let two_thousand = BigInt::from(2000);

        if value.sign() == num_bigint::Sign::Minus {
            return Ok(format!("{} f.Kr.", self.to_cardinal(&(-value))?));
        }
        if value < &thousand {
            return self.to_cardinal(value);
        }
        if value < &two_thousand {
            // 1000..1999 read as "<century>hundra<remainder>": 1492 ->
            // "fjortonhundranittiotvå". Both operands are non-negative here,
            // so Python's floor division matches a plain BigInt divide.
            let hundred = BigInt::from(100);
            let century = value / &hundred;
            let remainder = value % &hundred;
            if remainder.is_zero() {
                return Ok(format!("{}hundra", self.to_cardinal(&century)?));
            }
            return Ok(format!(
                "{}hundra{}",
                self.to_cardinal(&century)?,
                self.to_cardinal(&remainder)?
            ));
        }
        // 2000+ read as a plain cardinal: "tvåtusentjugofyra".
        self.to_cardinal(value)
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal` raises TypeError for
    /// any fractional or negative value; a whole non-negative one flows
    /// through `to_cardinal` + the `ords` suffix table — identical to the
    /// integer ordinal ("5.0" → "femte", `1e+16` → "tio biljarderde").
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = self.verify_ordinal_num(value)?;
        self.to_ordinal(&whole)
    }

    /// `to_ordinal_num(float/Decimal)` — no verify_ordinal at all:
    ///
    /// ```python
    /// if value == 1 or value == 2:                       str(value) + ":a"
    /// elif str(value).endswith(("1","2")) and value not in (11,12):  ":a"
    /// else:                                              str(value) + ":e"
    /// ```
    ///
    /// Numeric equality plus a *lexical* suffix test on `str(value)` — so
    /// "1.0"/"2.0" take ":a" (== 1/2), "42.0" takes ":e" (ends "0"), and
    /// "1E+2" takes ":a" (ends "2").
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => Ok(format!("{}{}", repr_str, sv_ordinal_num_suffix(&i))),
            None => Ok(format!("{}:e", repr_str)),
        }
    }

    /// `to_year(float/Decimal)` — `Num2Word_SV.to_year`'s arithmetic on the
    /// raw value: BC years render the *absolute* cardinal + " f.Kr."
    /// (`-1.5` → "ett komma fem f.Kr."), 1000..1999 splits into
    /// "<century>hundra<remainder>" ("1234.0" → "tolvhundratrettiofyra"),
    /// everything else is a plain cardinal.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: v, precision } => {
                let v = *v;
                if v < 0.0 {
                    // to_cardinal(-val) + " f.Kr."
                    let abs = FloatValue::Float {
                        value: -v,
                        precision: *precision,
                    };
                    return Ok(format!("{} f.Kr.", self.cardinal_float_entry(&abs, None)?));
                }
                if v < 1000.0 {
                    return self.cardinal_float_entry(value, None);
                }
                if v < 2000.0 {
                    let century = (v / 100.0).floor();
                    let remainder = v.rem_euclid(100.0);
                    if remainder == 0.0 {
                        return Ok(format!("{}hundra", self.cardinal_num_f64(century)?));
                    }
                    return Ok(format!(
                        "{}hundra{}",
                        self.cardinal_num_f64(century)?,
                        self.cardinal_num_f64(remainder)?
                    ));
                }
                self.cardinal_float_entry(value, None)
            }
            FloatValue::Decimal { value: d, precision } => {
                if d.is_negative() {
                    let abs = FloatValue::Decimal {
                        value: -d.clone(),
                        precision: *precision,
                    };
                    return Ok(format!("{} f.Kr.", self.cardinal_float_entry(&abs, None)?));
                }
                if d < &BigDecimal::from(1000) {
                    return self.cardinal_float_entry(value, None);
                }
                if d < &BigDecimal::from(2000) {
                    let hundred = BigDecimal::from(100);
                    let century =
                        (d / &hundred).with_scale_round(0, bigdecimal::RoundingMode::Down);
                    let remainder = d - &century * &hundred;
                    if remainder.is_zero() {
                        return Ok(format!("{}hundra", self.cardinal_num_dec(&century)?));
                    }
                    return Ok(format!(
                        "{}hundra{}",
                        self.cardinal_num_dec(&century)?,
                        self.cardinal_num_dec(&remainder)?
                    ));
                }
                self.cardinal_float_entry(value, None)
            }
        }
    }

    // ---- currency ----------------------------------------------------
    //
    // `currency_precision` is intentionally NOT overridden: SV resolves
    // `CURRENCY_PRECISION` to `Num2Word_Base`'s empty dict, so every code
    // takes the default divisor of 100. See the module docs.
    //
    // `money_verbose` / `cents_verbose` / `cents_terse` are likewise left
    // alone — SV inherits the base versions, which the trait defaults already
    // mirror (`to_cardinal`, `to_cardinal`, and zero-padded digits at the
    // currency's precision).
    //
    // `to_cheque` is not overridden either: SV inherits
    // `Num2Word_Base.to_cheque` verbatim, which `default_to_cheque` ports.
    // With divisor 100 it renders "<words> AND <NN>/100 <PLURAL UNIT>"
    // upper-cased — e.g. 1234.56 EUR → "ETTUSEN TVÅHUNDRATRETTIOFYRA AND
    // 56/100 EUROS". All nine cheque corpus rows land on that default.

    fn lang_name(&self) -> &str {
        "Num2Word_SV"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// Port of `Num2Word_EUR.pluralize`: `form = 0 if n == 1 else 1`.
    ///
    /// Reached only from the base float path — SV's own int branch open-codes
    /// the same choice instead of calling this.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        // Python would raise IndexError on a 1-element tuple with n != 1.
        // Unreachable with SV's table (every entry has >= 2 forms), but the
        // exception *type* is part of the contract if it ever were.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// Port of `Num2Word_SV.to_currency`.
    ///
    /// Only true ints take SV's own branch; everything else is handed to
    /// `Num2Word_Base.to_currency` verbatim, which is what Python's
    /// `super().to_currency(...)` does. The int/non-int split is the whole
    /// point of the override — `1` renders "ett euro" while `1.0` renders
    /// "ett euro, noll cent".
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
        let v = match val {
            CurrencyValue::Int(v) => v,
            // "For floats, use the parent class implementation."
            CurrencyValue::Decimal { value: _, .. } => {
                return crate::currency::default_to_currency(
                    self, val, currency, cents, separator, adjective,
                )
            }
        };

        let forms = match self.currency_forms.get(currency) {
            Some(f) => f,
            // Python catches (KeyError, AttributeError) and re-enters
            // super().to_currency(), whose int branch looks the same code up
            // and raises NotImplementedError. Delegating reproduces that
            // exactly — including the class name in the message — without
            // duplicating the raise.
            None => {
                return crate::currency::default_to_currency(
                    self, val, currency, cents, separator, adjective,
                )
            }
        };

        // `minus_str = self.negword` — stripped here, unlike Python, whose
        // trailing space plus the format's own separator double-spaced the
        // output; see the module docs, bug 5.
        let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
        let abs_val = v.abs();
        // Python calls to_cardinal directly here, not _money_verbose. Same
        // result for SV, but kept literal.
        let money_str = self.to_cardinal(&abs_val)?;

        // Open-coded rather than routed through `pluralize`, mirroring the
        // Python. The isinstance(cr1, tuple) guards are vacuous — every entry
        // is a tuple — so this reduces to forms[0] / forms[1], with the
        // len-1 fallback preserved for structural fidelity.
        let currency_str = if abs_val.is_one() {
            &forms.unit[0]
        } else if forms.unit.len() > 1 {
            &forms.unit[1]
        } else {
            &forms.unit[0]
        };

        // `adjective` and `cents`/`separator` are deliberately unused here —
        // Python's int branch ignores all three. See the module docs, bug 6.
        Ok(format!("{} {} {}", minus_str, money_str, currency_str)
            .trim()
            .to_string())
    }
}
