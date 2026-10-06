//! Port of `lang_DV.py` (Dhivehi / Maldivian).
//!
//! Shape: **self-contained**. `Num2Word_DV` inherits from nothing — it is a
//! bare `class Num2Word_DV:` with no base class at all, so `base.py`'s engine
//! (`splitnum`/`clean`/`merge`/`set_high_numwords` on `Num2Word_Base`) is
//! never involved. Every trait method here is an override; `cards()`,
//! `maxval()` and `merge()` stay at their `Lang` defaults and are unused.
//!
//! Note that `Num2Word_DV` defines its *own* `cards`, `MAXVAL` and
//! `set_high_numwords` that merely share names with `base.py`'s — they are
//! unrelated tables driving the module's own `convert_int`. They are modelled
//! as private fields/consts below rather than through the `Lang` trait, to
//! avoid the engine accidentally picking them up.
//!
//! # The numbering system
//!
//! Dhivehi groups digits Indian-style: units take 3 digits, then thousands
//! take 2 (`ހާސް`), then lakh takes 1 (`ލައްކަ`), and everything above that
//! reverts to Western 3-digit groups (million, billion, ... nonillion). That
//! is exactly what `self.grouping` encodes:
//!
//! ```text
//! [-3, -5, -6, -9, -12, -15, -18, -21, -24, -27, -30, -33, -36]
//! ```
//!
//! `convert_int` pairs `[len(digits)] + grouping` pairwise into `(end, start)`
//! and slices `digits_str[start:end]`, yielding
//! `parts[0] = last 3 digits`, `parts[1] = digits[-5:-3]` (thousands),
//! `parts[2] = digits[-6:-5]` (lakh), `parts[3] = digits[-9:-6]` (million), …
//! `parts[1..]` is then `zip`ped against `reversed(self.cards.items())`, i.e.
//! the multiplier words in ascending order. `zip` stops at the shorter side,
//! so `parts[12]` (`digits[-36:-33]`) is never consumed — unreachable anyway,
//! since MAXVAL caps input at 33 digits.
//!
//! # Stem vs nominal
//!
//! Every numeral has two forms: a *stem* (bound/attributive, `base_stem`) and
//! a *nominal* (free-standing, `base_nominal`). Cardinals end in the nominal
//! form, ordinals in the stem form — that is the whole of the `nominal` flag
//! threaded through `convert_int`. So `to_cardinal(3)` is `ތިނެއް` while
//! `to_ordinal(3)` is `ތިން ވަނަ`.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are verified against
//! the interpreter and against `bench/corpus.jsonl`:
//!
//! 1. **Negative cardinals emitted a double space (fixed, #160).** When
//!    `part_words` is empty (any |value| < 1000 with a non-zero last group)
//!    `convert_int` returns `"".join([]) + " " + word`, i.e. a string with a
//!    *leading* space. The non-negative path launders this via
//!    `result.strip()`, but Python's `sign == 1` path does
//!    `" ".join([negword, result])` with **no strip**, so `to_cardinal(-1)`
//!    was `"މައިނަސް އެކެއް"` — two spaces. The port strips `result` on the
//!    negative path too, so every negative gets one space.
//!
//! 2. **`convert_three2nominal` mixes stem and nominal for hundreds.** The
//!    `x00` branch uses `convert_two2nominal(digits_str[0])` while the `xyz`
//!    branch uses `convert_two2stem(digits_str[0])`. So cardinal 300 is
//!    `ތިނެއްސަތޭކަ` (*nominal* three + hundred) but cardinal 555 is
//!    `ފަސްސަތޭކަ ފަންސާސްފަހެއް` (*stem* five + hundred). Almost certainly a
//!    typo for `convert_two2stem`, but it is what Python emits.
//!
//! 3. **Hundreds spacing is asymmetric.** 100s and 200s concatenate with no
//!    separator (`ސަތޭކަ` + tail), while 300–900 insert a space before the tail
//!    (`convert_two2stem(d0) + base_stem[100] + " " + tail`). Hence 101 →
//!    `ސަތޭކައެކެއް` (no space) but 555 → `ފަސްސަތޭކަ ފަންސާސްފަހެއް` (space).
//!
//! 4. **Fixed (gladiaio/num2words2#159): the overflow bound is `MAXVAL`.**
//!    Python's guard reads
//!
//!    ```python
//!    if Decimal(self.MAXVAL).compare(abs(decimal_value).to_integral(ROUND_FLOOR)) < 1:
//!    ```
//!
//!    and `Decimal.__abs__` rounds to the context precision (28 significant
//!    digits) first, so every input from `10**33 - 50000` up rounded to 10^33
//!    and raised — `to_cardinal(10**33 - 1)` was an `OverflowError` although
//!    `maxval("dv")` is 10^33. The port compares the exact value instead, so
//!    the guard trips at `abs(value) >= 10**33` and everything below converts.
//!
//! 5. `to_ordinal`/`to_ordinal_num` raise `TypeError` (not ValueError) for any
//!    negative input, via `verify_ordinal`. `to_cardinal` is unaffected.
//!
//! # Currency
//!
//! `to_currency` shares nothing with `base.py`'s. The signature is
//!
//! ```python
//! def to_currency(self, value, currency="ރުފިޔާ", cents="ލާރި"):
//! ```
//!
//! — `currency` and `cents` are **words**, not an ISO code and a verbosity
//! flag. `Num2Word_DV` has no `CURRENCY_FORMS`, no `CURRENCY_PRECISION`, no
//! `pluralize`, and no `separator`/`adjective` kwargs, so those trait hooks all
//! stay at their defaults and are never consulted.
//!
//! Python interpolated `currency=` **verbatim** as the unit word, so
//! `to_currency(1, currency="EUR")` was `"އެއް EUR"` — a raw ISO code in
//! Dhivehi text. DV knows one currency, the rufiyaa: the port accepts its
//! default word ރުފިޔާ (what the binding passes when `currency=` is omitted)
//! and the code `MVR`, and raises `NotImplementedError` for every other code,
//! as `base.to_currency` does for an unknown one (#219).
//!
//! Continuing the quirk list above, all verified against the interpreter:
//!
//! 6. **Rounding (fixed, #170).** Python sets `int_part` to
//!    `value.to_integral_value()` (round half-even) and `frac_part` to the
//!    remainder times 100, so 1.5 printed "two rufiyaa **minus** fifty laari"
//!    and 99.99 "one hundred minus one laari". The port rounds `|value|`
//!    half-up to whole laari (as `base.to_currency` does), splits that into
//!    rufiyaa and laari, and puts the negword in front once.
//!
//! 7. **Either segment vanishes when its part is zero**, so 0.01 is
//!    `"އެއް ލާރި"` (no unit word) and 1.0 is `"އެއް ރުފިޔާ"` (no cents). Python
//!    returned the **empty string** when both parts round to zero (0.001); the
//!    port says zero units, like `to_currency(0)`.
//!
//! 8. **Large floats (fixed, #170).** Python's `to_integral_value` kept a
//!    positive exponent and `to_cardinal_float` read only the coefficient, so
//!    `to_currency(1e21)` was "one EUR". Counting in whole laari expands the
//!    exponent, so the full amount is read.
//!
//! `to_cheque` does not exist on the class at all (Python raised
//! `AttributeError`); the port raises NotImplementedError ("does not support
//! to='cheque'", #223).
//!
//! # Float/Decimal routing
//!
//! DV has no whole-value fast path: `to_cardinal(5.0)` goes through
//! `Decimal(str(5.0))` == `Decimal('5.0')`, whose `as_tuple()` has exponent
//! -1, so the pointword branch runs and the result is "five point zero"
//! (`ފަހެއް ޕޮއިންޓް ސުމެއް`). [`Lang::cardinal_float_entry`] is therefore
//! overridden to bypass the base's whole -> int routing entirely, and the
//! ordinal/year entries follow the class's own methods. More reproduced
//! quirks, all corpus-verified:
//!
//! 9.  **`str(float)`'s scientific regime collapsed to the coefficient
//!     (fixed, #190).** `str(1e16)` is `'1e+16'`, so `Decimal(str(1e16))` has
//!     digits `(1,)` and exponent 16, and Python's `convert_int` only ever
//!     saw the digit tuple: `to_cardinal(1e16)` and `to_cardinal(1e21)` were
//!     `އެކެއް` ("one"), as was `Decimal('1E+2')`. The port expands a
//!     positive exponent into trailing zeros, so the full value is read and
//!     anything from 10^33 up raises `OverflowError`. The tuple itself is
//!     still reconstructed in [`py_float_dec_tuple`] from Rust's
//!     shortest-round-trip digits (`{:e}`), which match `repr(float)`'s
//!     digits by uniqueness of the shortest representation; the regime
//!     boundary (scientific iff adjusted exponent >= 16 or < -4) is
//!     CPython's `float_repr_style` cutoff.
//! 10. **Whole floats keep their `.0`.** A fixed-point repr of an integral
//!     float appends ".0", giving the Decimal a trailing zero digit and
//!     exponent -1 — hence the trailing `ސުމެއް`/`ސުން` after the pointword
//!     in every `N.0` row.
//! 11. ~~**`cardinal(0.0)`, `cardinal(-0.0)` and `cardinal(".5")` raise
//!     IndexError**~~ — fixed (gladiaio/num2words2#158). `digits[:exponent]`
//!     is empty for any value below 1, and Python's `convert_int` died on
//!     `parts[0]`, so `0.5` raised while `1.5` converted. An empty integer
//!     part is now read as zero (with the fraction's implicit leading zeros
//!     restored), so `0.5` reads like `1.5` does.
//! 12. **`verify_ordinal` raises TypeError for non-integral floats first**,
//!     then for negatives (also TypeError); `-0.0` passes both checks
//!     (`-0.0 == int(-0.0)`, `abs(-0.0) == -0.0`); `to_ordinal_num` formats
//!     it as `-0.0 ވަނަ`.
//! 13. **Ordinal floats mix nominal and stem.** `to_cardinal_float(value,
//!     nominal=False)` hardcodes the *integer* part nominal — Python calls
//!     `convert_int(digits[:exponent])` with the default — while the
//!     fractional digits follow the flag as stems: `to_ordinal(1.0)` is
//!     `އެކެއް ޕޮއިންޓް ސުން ވަނަ` (nominal one, stem zero).
//! 14. **`to_year` in [1100, 2000) splits float years absurdly**:
//!     `to_year(1234.0)` computes `high, low = value // 100, value % 100` ==
//!     `12.0, 34.0` (both still floats!) and renders "twelve point zero
//!     hundred thirty-four point zero". Negative years go through `abs()`
//!     *before* the cardinal, so no negword and no double space:
//!     `to_year(-1.5)` == `އެކެއް ޕޮއިންޓް ފަހެއް ބީ.ސީ`.
//!
//! # Strings
//!
//! `str_to_number` is `to_decimal`, which turns `InvalidOperation` into
//! `TypeError("{} is not a valid value.")` — that is the whole override.
//! One deliberate shortcut: Python parses `"NaN"` into `Decimal('NaN')`
//! successfully and only raises later, when `to_cardinal_float`'s overflow
//! guard evaluates `Decimal(MAXVAL).compare(NaN) < 1` (ordered comparison
//! with a NaN Decimal -> `decimal.InvalidOperation`). The pyo3 shim
//! hardcodes `ParsedNumber::NaN -> ValueError` before any DV hook could
//! run, so this port raises the InvalidOperation from `str_to_number`
//! itself — the same observable exception for every corpus row ("NaN" only
//! appears under `to='cardinal'`). The one place the shortcut is visibly
//! unfaithful is `to_ordinal("NaN")`, where genuine Python raises
//! ValueError from `int(Decimal('NaN'))` — a row no corpus exercises.
//!
//! # Fraction
//!
//! `Num2Word_DV` has no `to_fraction` (Python raised AttributeError). DV has
//! no fraction rules, so `to='fraction'` and "n/d" strings raise
//! NotImplementedError, and "1/0" ZeroDivisionError, like every language
//! without fraction rules (#217).
//!
//! # Grammatical kwargs
//!
//! `to_cardinal(value, nominal=True)` is the only extra cardinal kwarg;
//! Python tests it with bare truthiness (`if not nominal`), so False, 0,
//! None and "" all select the stem form. `to_year`'s `suffix=""` rides
//! `to_year_kw`; a truthy suffix overrides the implicit `bcword` on
//! negative years. Everything else falls back to Python via the
//! `kw.only` guard.
//!
//! `cardinal_from_decimal` stays at its default: DV's `to_currency` never
//! produces fractional cents, since it counts in whole laari.

use crate::base::{Kwargs, KwVal, Lang, N2WError, Result};
use crate::currency::CurrencyValue;
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{Signed, Zero};

const NEGWORD: &str = "މައިނަސް";
const ORDWORD: &str = "ވަނަ";
const BCWORD: &str = "ބީ.ސީ";

/// `to_currency`'s `cents=` default: the *word* ލާރި (laari, 1/100 rufiyaa),
/// not a verbosity flag. See the `to_currency` impl for why the trait's bool
/// cannot carry it.
const CENTSWORD: &str = "ލާރި";

/// `self.high_numwords`, in source order. `set_high_numwords` zips this with
/// `range(3 + 3*len(high), 3, -3)` == `30, 27, …, 6`, so index 0 is 10^30.
const HIGH_NUMWORDS: [&str; 9] = [
    "ނޮނިލި",
    "އޮކްޓިލި",
    "ސެޕްޓިލި",
    "ސެކްސްޓިލި",
    "ކުއިންޓިލި",
    "ކުއަޑްރި",
    "ޓްރި",
    "ބިލި",
    "މިލި",
];

/// The suffix `set_high_numwords` appends to every high numword.
const HIGH_SUFFIX: &str = "ޔަން";

/// `self.grouping`, i.e. `[-3, -5] + [(x * 3 + 6) * -1 for x in range(len(self.cards))]`
/// with `len(self.cards) == 11`.
const GROUPING: [isize; 13] = [
    -3, -5, -6, -9, -12, -15, -18, -21, -24, -27, -30, -33, -36,
];

/// `self.MAXVAL` == `list(self.cards.keys())[0] * 1000` == 10^30 * 1000.
const MAXVAL_EXP: usize = 33;

pub struct LangDv {
    /// `reversed(self.cards.items())` — multiplier words in ascending order of
    /// their key, which is the order `convert_int` zips `parts[1:]` against.
    /// Only the word is ever read (the comprehension binds `base` and drops
    /// it), so the keys are not carried.
    mult_words_asc: Vec<String>,
    maxval: BigInt,
}

pub fn new() -> LangDv {
    LangDv::new()
}

impl Default for LangDv {
    fn default() -> Self {
        LangDv::new()
    }
}

impl LangDv {
    pub fn new() -> LangDv {
        // Mirror of `set_high_numwords` + the two explicit inserts. Python
        // builds `cards` descending (10^30 … 10^6, then 100000, then 1000);
        // `convert_int` only ever consumes it reversed, so store it ascending.
        let mut cards_desc: Vec<String> = HIGH_NUMWORDS
            .iter()
            .map(|w| format!("{}{}", w, HIGH_SUFFIX))
            .collect();
        cards_desc.push("ލައްކަ".to_string()); // self.cards[100000]
        cards_desc.push("ހާސް".to_string()); // self.cards[1000]

        let mut mult_words_asc = cards_desc;
        mult_words_asc.reverse();

        LangDv {
            mult_words_asc,
            maxval: pow10(MAXVAL_EXP),
        }
    }

    /// `self.base_stem` — bound/attributive forms. Missing key => `KeyError`.
    fn base_stem(&self, k: i64) -> Result<&'static str> {
        Ok(match k {
            0 => "ސުން",
            1 => "އެއް",
            2 => "ދެ",
            3 => "ތިން",
            4 => "ހަތަރު",
            5 => "ފަސް",
            6 => "ހަ",
            7 => "ހަތް",
            8 => "އައް",
            9 => "ނުވަ",
            10 => "ދިހަ",
            100 => "ސަތޭކަ",
            200 => "ދުއިސައްތަ",
            _ => return Err(N2WError::Key(k.to_string())),
        })
    }

    /// `self.base_nominal` — free-standing forms. Missing key => `KeyError`.
    fn base_nominal(&self, k: i64) -> Result<&'static str> {
        Ok(match k {
            0 => "ސުމެއް",
            1 => "އެކެއް",
            2 => "ދޭއް",
            3 => "ތިނެއް",
            4 => "ހަތަރެއް",
            5 => "ފަހެއް",
            6 => "ހައެއް",
            7 => "ހަތެއް",
            8 => "އަށެއް",
            9 => "ނުވައެއް",
            10 => "ދިހައެއް",
            11 => "އެގާރަ",
            12 => "ބާރަ",
            13 => "ތޭރަ",
            14 => "ސައުދަ",
            15 => "ފަނަރަ",
            16 => "ސޯޅަ",
            17 => "ސަތާރަ",
            18 => "އަށާރަ",
            19 => "ނަވާރަ",
            20 => "ވިހި",
            21 => "އެކާވީސް",
            22 => "ބާވީސް",
            23 => "ތޭވީސް",
            24 => "ސައުވީސް",
            25 => "ފަންސަވީސް",
            26 => "ސައްބީސް",
            27 => "ހަތާވީސް",
            28 => "އަށާވީސް",
            29 => "ނަވާވީސް",
            30 => "ތިރީސް",
            40 => "ސާޅީސް",
            50 => "ފަންސާސް",
            60 => "ފަސްދޮޅަސް",
            70 => "ހަތްދިހަ",
            80 => "އައްޑިހަ",
            90 => "ނުވަދިހަ",
            _ => return Err(N2WError::Key(k.to_string())),
        })
    }

    /// `convert_decade2nominal`: `self.base_nominal[int(decade) * 10]`.
    fn convert_decade2nominal(&self, decade: &str) -> Result<&'static str> {
        self.base_nominal(py_int(decade)? * 10)
    }

    /// `convert_decade2stem`: 1 => `base_stem[10]`, else the nominal decade.
    fn convert_decade2stem(&self, decade: &str) -> Result<&'static str> {
        if py_int(decade)? == 1 {
            return self.base_stem(10);
        }
        self.convert_decade2nominal(decade)
    }

    /// `convert_two2nominal`.
    ///
    /// Unlike its `2stem` sibling this does **not** do `str(int(...))` first,
    /// so it indexes the caller's string as-is. Callers only ever pass an
    /// already-trimmed 1-char string or an exactly-2-char `digits_str[1:3]`
    /// slice, so the `< 30` guard always covers the 1-char case and the
    /// `digits_str[1]` access below is never out of range.
    fn convert_two2nominal(&self, digits_str: &str) -> Result<String> {
        let n = py_int(digits_str)?;
        if n < 30 {
            return Ok(self.base_nominal(n)?.to_string());
        }

        let d1 = char_at(digits_str, 1)?;
        let d0 = char_at(digits_str, 0)?;
        if py_int(&d1)? == 0 {
            return Ok(self.convert_decade2stem(&d0)?.to_string());
        }

        Ok(format!(
            "{}{}",
            self.convert_decade2stem(&d0)?,
            self.base_nominal(py_int(&d1)?)?
        ))
    }

    /// `convert_two2stem`.
    fn convert_two2stem(&self, digits_str: &str) -> Result<String> {
        let n = py_int(digits_str)?;
        let ds = n.to_string(); // `str(int(digits_str))` — trim leading zeros

        if n < 11 {
            return Ok(self.base_stem(n)?.to_string());
        }
        if n < 30 {
            return Ok(self.base_nominal(n)?.to_string());
        }

        let d1 = char_at(&ds, 1)?;
        let d0 = char_at(&ds, 0)?;
        if py_int(&d1)? == 0 {
            return Ok(self.convert_decade2stem(&d0)?.to_string());
        }

        Ok(format!(
            "{}{}",
            self.convert_decade2stem(&d0)?,
            self.base_stem(py_int(&d1)?)?
        ))
    }

    /// `convert_three2stem`.
    ///
    /// 100/200 have dedicated stems and glue straight onto the tail; 300–900
    /// are built as `<digit stem> + ސަތޭކަ + " " + <tail>` — note the space,
    /// which the 100/200 branches do not emit. Quirk 3 in the module docs.
    fn convert_three2stem(&self, digits_str: &str) -> Result<String> {
        let ds = py_int(digits_str)?.to_string(); // trim leading zeros

        if ds.len() < 3 {
            return self.convert_two2stem(&ds);
        }

        let d0 = char_at(&ds, 0)?;
        let tail = py_slice(&ds, 1, 3); // digits_str[1:3]
        let tail_n = py_int(&tail)?;

        if py_int(&d0)? == 1 {
            if tail_n == 0 {
                return Ok(self.base_stem(100)?.to_string());
            }
            return Ok(format!(
                "{}{}",
                self.base_stem(100)?,
                self.convert_two2stem(&tail)?
            ));
        }

        if py_int(&d0)? == 2 {
            if tail_n == 0 {
                return Ok(self.base_stem(200)?.to_string());
            }
            return Ok(format!(
                "{}{}",
                self.base_stem(200)?,
                self.convert_two2stem(&tail)?
            ));
        }

        if tail_n == 0 {
            return Ok(format!(
                "{}{}",
                self.convert_two2stem(&d0)?,
                self.base_stem(100)?
            ));
        }

        Ok(format!(
            "{}{} {}",
            self.convert_two2stem(&d0)?,
            self.base_stem(100)?,
            self.convert_two2stem(&tail)?
        ))
    }

    /// `convert_three2nominal`.
    ///
    /// Mirrors `convert_three2stem` except the tails are nominal — **and** the
    /// `x00` branch uses `convert_two2nominal(digits_str[0])` where the `xyz`
    /// branch uses `convert_two2stem(digits_str[0])`. That inconsistency is
    /// quirk 2 in the module docs: cardinal 300 == `ތިނެއްސަތޭކަ` (nominal three)
    /// but cardinal 555 == `ފަސްސަތޭކަ ފަންސާސްފަހެއް` (stem five). Preserved verbatim.
    fn convert_three2nominal(&self, digits_str: &str) -> Result<String> {
        let ds = py_int(digits_str)?.to_string(); // trim leading zeros

        if ds.len() < 3 {
            return self.convert_two2nominal(&ds);
        }

        let d0 = char_at(&ds, 0)?;
        let tail = py_slice(&ds, 1, 3);
        let tail_n = py_int(&tail)?;

        if py_int(&d0)? == 1 {
            if tail_n == 0 {
                return Ok(self.base_stem(100)?.to_string());
            }
            return Ok(format!(
                "{}{}",
                self.base_stem(100)?,
                self.convert_two2nominal(&tail)?
            ));
        }

        if py_int(&d0)? == 2 {
            if tail_n == 0 {
                return Ok(self.base_stem(200)?.to_string());
            }
            return Ok(format!(
                "{}{}",
                self.base_stem(200)?,
                self.convert_two2nominal(&tail)?
            ));
        }

        if tail_n == 0 {
            // NOTE: nominal here, stem in the branch below. Python bug, kept.
            return Ok(format!(
                "{}{}",
                self.convert_two2nominal(&d0)?,
                self.base_stem(100)?
            ));
        }

        Ok(format!(
            "{}{} {}",
            self.convert_two2stem(&d0)?,
            self.base_stem(100)?,
            self.convert_two2nominal(&tail)?
        ))
    }

    /// `convert_int`. `digits_str` is the decimal magnitude, i.e. Python's
    /// `"".join(str(d) for d in digits)` where `digits` came from
    /// `Decimal.as_tuple()` (sign-free, and exponent 0 for integer input).
    fn convert_int(&self, digits_str: &str, nominal: bool) -> Result<String> {
        if digits_str == "0" {
            return Ok(if nominal {
                self.base_nominal(0)?.to_string()
            } else {
                self.base_stem(0)?.to_string()
            });
        }

        // parts = filter(truthy, digits_str[start:end] for end, start in
        //                pairwise([len(digits_str)] + self.grouping))
        //
        // Emptiness is monotone here: `s[-a:-b]` is non-empty iff `len(s) > b`,
        // and the `b`s increase along `grouping`, so the filter only ever trims
        // a suffix of the sequence and never renumbers a surviving element.
        let mut bounds: Vec<isize> = Vec::with_capacity(1 + GROUPING.len());
        bounds.push(digits_str.chars().count() as isize);
        bounds.extend_from_slice(&GROUPING);

        let mut parts: Vec<String> = Vec::new();
        for w in bounds.windows(2) {
            let (end, start) = (w[0], w[1]);
            let p = py_slice(digits_str, start, end);
            if !p.is_empty() {
                parts.push(p);
            }
        }

        // `parts[0]` — IndexError if the filter left nothing. Unreachable for
        // integer input (the magnitude always has >= 1 digit), but this is the
        // site where float input dies, so it is modelled rather than assumed.
        let head = match parts.first() {
            Some(h) => h.clone(),
            None => return Err(N2WError::Index("list index out of range".into())),
        };

        // zip(parts[1:], reversed(self.cards.items())) — stops at the shorter.
        let mut part_words: Vec<String> = Vec::new();
        for (num, word) in parts.iter().skip(1).zip(self.mult_words_asc.iter()) {
            if py_int(num)? != 0 {
                part_words.push(format!("{}{}", self.convert_three2stem(num)?, word));
            }
        }
        part_words.reverse();

        if py_int(&head)? == 0 {
            return Ok(part_words.join(" "));
        }

        // NOTE: when `part_words` is empty this yields a leading space. The
        // caller strips it only on the non-negative path — see quirk 1.
        let tail = if nominal {
            self.convert_three2nominal(&head)?
        } else {
            self.convert_three2stem(&head)?
        };
        Ok(format!("{} {}", part_words.join(" "), tail))
    }

    /// `to_cardinal_float`, restricted to integer input.
    fn to_cardinal_float(&self, value: &BigInt, nominal: bool) -> Result<String> {
        // if Decimal(self.MAXVAL).compare(abs(v).to_integral(ROUND_FLOOR)) < 1
        //
        // `compare(x) < 1` means `MAXVAL <= x`. Compared exactly, without
        // Python's 28-digit rounding (quirk 4).
        if value.abs() >= self.maxval {
            // Python formats the signed value, not its absolute value.
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                value, self.maxval
            )));
        }

        let result = self.convert_int(&value.abs().to_string(), nominal)?;

        if value.is_negative() {
            // " ".join([negword, result]) — Python skips the strip here and
            // double-spaces; stripped in the port (quirk 1, fixed).
            return Ok(format!("{} {}", NEGWORD, result.trim()));
        }
        Ok(result.trim().to_string())
    }

    /// `to_cardinal_float`, for the currency path — where the argument is an
    /// already-integral `Decimal` rather than an `int`.
    ///
    /// Python reads `as_tuple()`, i.e. the bare coefficient, and would drop a
    /// positive exponent (quirk 8). Since #170 the currency path only passes
    /// whole-laari splits with exponent 0, so the coefficient is the full
    /// value and this agrees with [`LangDv::to_cardinal_float`].
    fn to_cardinal_float_dec(&self, value: &BigDecimal, nominal: bool) -> Result<String> {
        // sign, digits, exponent = decimal_value.as_tuple()
        let (coefficient, scale) = value.as_bigint_and_exponent();
        let exponent = -scale;
        let digits = coefficient.abs().to_string();

        if self.overflows(&coefficient, exponent) {
            // Python formats the *Decimal*; with exponent 0 (the only case
            // since #170) that is the plain digits.
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                py_decimal_str(value),
                self.maxval
            )));
        }

        // The `exponent < 0` arm of `to_cardinal_float` (the pointword/
        // convert_discrete branch) is unreachable from here: every caller
        // feeds this an integral value with exponent 0, so only the
        // `convert_int` arm can run.
        let result = self.convert_int(&digits, nominal)?;

        if coefficient.is_negative() {
            // " ".join([negword, result]) — stripped, unlike Python (quirk 1,
            // fixed). Unreachable from the currency path, which passes
            // magnitudes only (#170).
            return Ok(format!("{} {}", NEGWORD, result.trim()));
        }
        Ok(result.trim().to_string())
    }

    /// `to_cardinal_float`'s overflow guard for a `Decimal` argument:
    /// `Decimal(MAXVAL).compare(abs(v).to_integral(ROUND_FLOOR)) < 1`, i.e.
    /// `MAXVAL <= |v|`, compared exactly — see quirk 4.
    ///
    /// `exponent` is Python's, so `>= 0` for every caller.
    fn overflows(&self, coefficient: &BigInt, exponent: i64) -> bool {
        if coefficient.is_zero() {
            return false;
        }
        // |v| >= 10^adjusted, where `adjusted` is the power of ten of the
        // leading digit. So adjusted >= 33 settles it as MAXVAL <= |v| without
        // materialising 10^exponent — which a Decimal("1E+999999999") would
        // otherwise demand, turning Python's instant OverflowError into an OOM.
        let ndigits = coefficient.abs().to_string().len() as i64;
        if exponent + ndigits - 1 >= MAXVAL_EXP as i64 {
            return true;
        }
        // |v| < 10^33 now, so the exact integer is cheap.
        let exact = coefficient.abs() * pow10(exponent.max(0) as usize);
        exact >= self.maxval
    }

    /// `verify_ordinal`. The `value == int(value)` float check cannot fail for
    /// integer input; only the negative check is reachable, and it raises
    /// `TypeError` rather than the `ValueError` one might expect.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `convert_discrete`: render each individual digit as a *free* word. The
    /// fractional tail of the float path is spelled digit by digit, so this
    /// never sees anything but single 0-9 characters and neither table can
    /// miss. `nominal` picks `base_nominal` (cardinals) over `base_stem`; the
    /// non-nominal branch is only reachable via ordinal-float input, which
    /// `verify_ordinal` rejects before it can arrive here.
    fn convert_discrete(&self, digits_str: &str, nominal: bool) -> Result<String> {
        let mut out: Vec<String> = Vec::new();
        for ch in digits_str.chars() {
            let d = py_int(&ch.to_string())?;
            out.push(if nominal {
                self.base_nominal(d)?.to_string()
            } else {
                self.base_stem(d)?.to_string()
            });
        }
        Ok(out.join(" "))
    }

    /// The *real* `to_cardinal_float(value, nominal)` body — the one Python
    /// reaches with genuine float/Decimal input. Not to be confused with
    /// [`LangDv::to_cardinal_float`] above, which shares the Python name but is
    /// only ever fed an already-integral argument by the integer/currency
    /// modes. Both the Float and Decimal arms of the trait entry point reduce
    /// to `decimal_value.as_tuple()` — `(sign_negative, digit_str, exponent)` —
    /// and converge here.
    ///
    /// `digit_str` is the coefficient with no leading zeros (bar a lone `"0"`);
    /// `exponent` is Python's `as_tuple` exponent. The overflow guard has
    /// already fired in the caller.
    fn cardinal_float_body(
        &self,
        sign_negative: bool,
        digit_str: &str,
        exponent: i64,
        nominal: bool,
    ) -> Result<String> {
        let result = if exponent < 0 {
            let n = digit_str.chars().count() as i64;
            // Python slices the coefficient tuple: `digits[:exponent]` is the
            // integer part, `digits[exponent:len(digits)]` the fractional one.
            let cut = n + exponent; // == len(digits) - |exponent|

            // `digits[:exponent]` — empty when |exponent| >= len(digits), which
            // made Python's convert_int die with IndexError, so 0.5 / 0.01 /
            // 0.0 raised while 1.5 converted (quirk 11). Fixed
            // (gladiaio/num2words2#158): an empty integer part is zero.
            let int_digits: String = if cut > 0 {
                digit_str.chars().take(cut as usize).collect()
            } else {
                "0".to_string()
            };
            // Python hardcodes nominal=True for the integer part here (the
            // default arg), independent of the flag threaded to the fraction.
            let int_part = self.convert_int(&int_digits, true)?;

            // `digits[exponent:len(digits)]` — the fractional digits. When
            // |exponent| exceeds the coefficient's length the leading zeros
            // are implicit (0.05 is digits "5", exponent -2), so restore them.
            let frac_digits: String = if cut > 0 {
                digit_str.chars().skip(cut as usize).collect()
            } else {
                format!("{}{}", "0".repeat((-cut) as usize), digit_str)
            };
            let frac_part = self.convert_discrete(&frac_digits, nominal)?;

            // " ".join([convert_int(...), pointword, convert_discrete(...)]).
            // int_part still carries convert_int's leading space; the strip on
            // the non-negative path below launders it (quirk 1).
            format!("{} {} {}", int_part, self.pointword(), frac_part)
        } else {
            // Python fed `convert_int` the coefficient alone, so a positive
            // exponent was dropped: 1e21 (`Decimal('1E+21')`, digits `(1,)`)
            // read "one" (quirk 9). Expand it into the full digit string.
            let mut full = digit_str.to_string();
            full.push_str(&"0".repeat(exponent as usize));
            self.convert_int(&full, nominal)?
        };

        if sign_negative {
            // " ".join([negword, result]) — stripped, unlike Python, so
            // convert_int's leading space no longer doubles up (quirk 1).
            Ok(format!("{} {}", NEGWORD, result.trim()))
        } else {
            Ok(result.trim().to_string())
        }
    }

    /// `to_cardinal_float`'s overflow guard for genuine float/Decimal input:
    /// `Decimal(MAXVAL).compare(abs(v).to_integral(ROUND_FLOOR)) < 1`, i.e.
    /// `MAXVAL <= floor(|v|)`, compared exactly (quirk 4). Unlike
    /// [`LangDv::overflows`], this floors the fractional part, matching
    /// `to_integral(ROUND_FLOOR)` — the value here can carry a real fraction.
    fn float_path_overflows(&self, digit_str: &str, exponent: i64) -> bool {
        let d = match BigInt::parse_bytes(digit_str.as_bytes(), 10) {
            Some(d) if !d.is_zero() => d,
            _ => return false,
        };
        // to_integral(ROUND_FLOOR): floor(d * 10**exponent), with d >= 0.
        let floored = if exponent >= 0 {
            d * pow10(exponent as usize)
        } else {
            d / pow10((-exponent) as usize) // truncation == floor for d >= 0
        };
        floored >= self.maxval
    }

    /// `to_cardinal_float(value, nominal)` for a value already reduced to its
    /// `as_tuple()` view: overflow guard, then [`LangDv::cardinal_float_body`].
    fn cardinal_tuple(
        &self,
        sign_negative: bool,
        digit_str: &str,
        exponent: i64,
        nominal: bool,
    ) -> Result<String> {
        if self.float_path_overflows(digit_str, exponent) {
            // Python formats the *Decimal* (`errmsg_toobig.format(decimal_value,
            // self.MAXVAL)`), so a positive exponent prints scientifically.
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                py_decimal_str_parts(sign_negative, digit_str, exponent),
                self.maxval
            )));
        }
        self.cardinal_float_body(sign_negative, digit_str, exponent, nominal)
    }

    /// The full `to_cardinal_float(value, nominal)` for genuine float/Decimal
    /// input — `to_decimal` (a no-op tuple reduction here), guard, body.
    fn cardinal_float_full(&self, value: &FloatValue, nominal: bool) -> Result<String> {
        let (sign_negative, digit_str, exponent) = float_value_tuple(value);
        self.cardinal_tuple(sign_negative, &digit_str, exponent, nominal)
    }

    /// `to_cardinal(f)` for an f64 the *port itself* produced (`to_year`'s
    /// `high`/`low`), routed through `Decimal(str(f))` like any float input.
    fn cardinal_from_f64(&self, f: f64, nominal: bool) -> Result<String> {
        let (sign_negative, digit_str, exponent) = py_float_dec_tuple(f);
        self.cardinal_tuple(sign_negative, &digit_str, exponent, nominal)
    }

    /// `verify_ordinal` for float/Decimal input.
    ///
    /// Order matters: the `value == int(value)` check runs first, so `-1.5`
    /// raises the *floatord* TypeError, not the negord one. `-0.0` passes
    /// both (`-0.0 == 0` and `abs(-0.0) == -0.0`) — quirk 12.
    fn verify_ordinal_float(&self, value: &FloatValue) -> Result<()> {
        let integral = match value {
            FloatValue::Float { value, .. } => *value == value.trunc(),
            FloatValue::Decimal { value, .. } => {
                let (coeff, scale) = value.as_bigint_and_exponent();
                scale <= 0 || (&coeff % pow10(scale as usize)).is_zero()
            }
        };
        if !integral {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                float_value_str(value)
            )));
        }
        let negative = match value {
            // Numeric `<`, not the sign bit: abs(-0.0) == -0.0 in Python.
            FloatValue::Float { value, .. } => *value < 0.0,
            // BigDecimal cannot carry a negative zero, so a negative
            // coefficient is a genuinely negative value.
            FloatValue::Decimal { value, .. } => {
                value.as_bigint_and_exponent().0.is_negative()
            }
        };
        if negative {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                float_value_str(value)
            )));
        }
        Ok(())
    }

    /// `to_year(value, suffix)` for integer input — the shared body behind
    /// the plain trait hook (`suffix=""`) and `to_year_kw`.
    fn year_int(&self, value: &BigInt, suffix: &str) -> Result<String> {
        let mut v = value.clone();
        let mut suffix = suffix.to_string();
        if v.is_negative() {
            v = v.abs();
            // suffix = self.bcword if not suffix else suffix
            if suffix.is_empty() {
                suffix = BCWORD.to_string();
            }
        }

        // `v` is non-negative by here, so Rust's truncating `/` and `%` agree
        // with Python's floor semantics.
        let hundred = BigInt::from(100);
        let high = &v / &hundred;
        let low = &v % &hundred;

        let mut result: Vec<String> = Vec::new();
        if v < BigInt::from(2000) && v >= BigInt::from(1100) {
            result.push(self.to_cardinal(&high)?);
            result.push("ސަތޭކަ".to_string()); // literal in Python, == base_stem[100]
            result.push(self.to_cardinal(&low)?);
        } else {
            result.push(self.to_cardinal(&v)?);
        }

        if !suffix.is_empty() {
            result.push(suffix);
        }

        Ok(result.join(" "))
    }
}

impl Lang for LangDv {

    fn python_maxval(&self) -> Option<num_bigint::BigInt> {
        // Python class attribute MAXVAL (self-contained converter).
        Some(num_bigint::BigInt::from(10u32).pow(33))
    }
    /// `self.pointword`, read from the live Python instance.
    /// Unused by the four integer modes, so phase 1 never needed it —
    /// the float path is the first caller.
    fn pointword(&self) -> &str {
        "ޕޮއިންޓް"
    }

    fn lang_name(&self) -> &str {
        "Num2Word_DV"
    }

    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "ރުފިޔާ"
    }

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal_float(value, true)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!(
            "{} {}",
            self.to_cardinal_float(value, false)?,
            ORDWORD
        ))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{} {}", value, ORDWORD))
    }

    /// `to_year(value, suffix="")`. This plain hook carries no `suffix`;
    /// callers passing one go through [`Lang::to_year_kw`]. The default empty
    /// string only matters for negative years, where Python substitutes
    /// `bcword`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.year_int(value, "")
    }

    /// `to_year(value, suffix=...)` — the only extra kwarg the Python
    /// signature accepts. `suffix=None` is falsy in `if not suffix`, so it
    /// behaves like the default; a non-str suffix would only crash later in
    /// `" ".join`, so it is punted back to Python instead of guessed at.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let suffix = match kw.get("suffix") {
            Option::None | Some(KwVal::None) => String::new(),
            Some(KwVal::Str(s)) => s.clone(),
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        self.year_int(value, &suffix)
    }

    /// `to_cardinal(value, nominal=True)` — the extra cardinal kwarg. Python
    /// only ever tests it with bare truthiness, so any falsy value selects
    /// the stem form.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["nominal"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.to_cardinal_float(value, kw_nominal(kw))
    }

    /// The genuine float/Decimal cardinal path. `Num2Word_DV.to_cardinal(value)`
    /// forwards to `to_cardinal_float(value, nominal=True)`, which converts the
    /// value to a `Decimal` (a float via `Decimal(str(value))`, so `2.675`
    /// yields the exact string digits `2,6,7,5` — DV never touches `float2tuple`
    /// and its binary-artefact heuristic) and renders it through
    /// `decimal_value.as_tuple()`.
    ///
    /// `precision_override` is ignored: DV's `to_cardinal_float` signature is
    /// `(value, nominal=True)` — no `precision` kwarg — and DV carries no
    /// `self.precision` attribute for the dispatcher's `precision=` path to
    /// latch onto, so Python could never thread one in here.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_full(value, true)
    }

    /// `to_cardinal(float/Decimal)` — the full entry, whole values included.
    /// DV has no whole-value routing: `Decimal(str(5.0))` keeps its `.0`, so
    /// even integral floats run the pointword grammar (quirk 10), while a
    /// scientific repr reads as the whole number it is (quirk 9, #190).
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_full(value, true)
    }

    /// `to_ordinal(float/Decimal)`: `verify_ordinal(value)` (TypeError on
    /// non-integral, then on negative — quirk 12), then the float grammar
    /// with `nominal=False` (quirk 13) plus the ordword.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.verify_ordinal_float(value)?;
        Ok(format!(
            "{} {}",
            self.cardinal_float_full(value, false)?,
            ORDWORD
        ))
    }

    /// `to_ordinal_num(float/Decimal)`: `verify_ordinal`, then
    /// `"{} {}".format(value, ordword)` — `repr_str` is Python's
    /// `str(value)`, so `-0.0` (which passes verify) prints as `-0.0 ވަނަ`.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        self.verify_ordinal_float(value)?;
        Ok(format!("{} {}", repr_str, ORDWORD))
    }

    /// `to_year(float/Decimal)` — quirk 14. `value < 0` takes `abs()` first,
    /// so negative years carry no negword; [1100, 2000) splits into
    /// still-float `high`/`low` halves rendered independently.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value {
            FloatValue::Float { value: f, .. } => {
                let mut v = *f;
                let mut suffix = "";
                // Numeric `<`: -0.0 is not negative and keeps its sign into
                // the cardinal.
                if v < 0.0 {
                    v = -v;
                    suffix = BCWORD;
                }

                let mut result: Vec<String> = Vec::new();
                if v < 2000.0 && v >= 1100.0 {
                    // Python float `%` is fmod (positive operands here), and
                    // `//` is the floor of the corrected quotient.
                    let low = v % 100.0;
                    let high = ((v - low) / 100.0).floor();
                    result.push(self.cardinal_from_f64(high, true)?);
                    result.push("ސަތޭކަ".to_string());
                    result.push(self.cardinal_from_f64(low, true)?);
                } else {
                    result.push(self.cardinal_from_f64(v, true)?);
                }

                if !suffix.is_empty() {
                    result.push(suffix.to_string());
                }
                Ok(result.join(" "))
            }
            FloatValue::Decimal { value: d, .. } => {
                let mut v = d.clone();
                let mut suffix = "";
                if v < BigDecimal::from(0) {
                    v = v.abs();
                    suffix = BCWORD;
                }

                let mut result: Vec<String> = Vec::new();
                if v < BigDecimal::from(2000) && v >= BigDecimal::from(1100) {
                    // Decimal divide-integer (`//`) yields exponent 0; the
                    // remainder keeps the ideal exponent min(exp(v), 0).
                    let (coeff, scale) = v.as_bigint_and_exponent();
                    let hundred = BigInt::from(100);
                    let (q, low_coeff, low_exp) = if scale >= 0 {
                        let div = &hundred * pow10(scale as usize);
                        let q = &coeff / &div; // v >= 1100 > 0: trunc == floor
                        let low = &coeff - &q * &div;
                        (q, low, -scale)
                    } else {
                        let full = &coeff * pow10((-scale) as usize);
                        let q = &full / &hundred;
                        let low = &full - &q * &hundred;
                        (q, low, 0)
                    };
                    result.push(self.cardinal_tuple(false, &q.to_string(), 0, true)?);
                    result.push("ސަތޭކަ".to_string());
                    result.push(self.cardinal_tuple(false, &low_coeff.to_string(), low_exp, true)?);
                } else {
                    let (coeff, scale) = v.as_bigint_and_exponent();
                    result.push(self.cardinal_tuple(
                        coeff.is_negative(),
                        &coeff.abs().to_string(),
                        -scale,
                        true,
                    )?);
                }

                if !suffix.is_empty() {
                    result.push(suffix.to_string());
                }
                Ok(result.join(" "))
            }
        }
    }

    /// `to_cardinal(float/Decimal, nominal=...)` — the kwarg rides the float
    /// entry too; the integer part stays hardcoded-nominal either way
    /// (quirk 13), only the fractional digits and the no-point branch follow
    /// the flag.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&["nominal"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.cardinal_float_full(value, kw_nominal(kw))
    }

    /// `str_to_number` == `to_decimal`: `Decimal(value)` with
    /// `InvalidOperation` rewritten to `TypeError(errmsg_nonnum)`. The NaN
    /// arm short-circuits the InvalidOperation Python would raise a step
    /// later (`Decimal(MAXVAL).compare(NaN) < 1` in `to_cardinal_float`) —
    /// see the "Strings" section of the module docs for why it lives here.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s) {
            Ok(ParsedNumber::NaN) => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "comparison involving NaN".into(),
            }),
            // A successful parse is served natively. The `to="fraction"`
            // AttributeError (Num2Word_DV has no to_fraction) is now handled
            // by the binding's fraction arm, which probes `to_fraction(1,1)`
            // and surfaces the language's AttributeError — so the earlier
            // blanket defer-to-Python is no longer needed.
            Ok(other) => Ok(other),
            Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                ..
            }) => Err(N2WError::Type(format!("{} is not a valid value.", s))),
            Err(e) => Err(e),
        }
    }

    // `Num2Word_DV` had no `to_fraction` (AttributeError). DV has no fraction
    // rules, so the trait default raises NotImplementedError (#217).

    /// `to_currency(self, value, currency="ރުފިޔާ", cents="ލާރި")`.
    ///
    /// Shares no machinery with `base.to_currency`: no forms table, no
    /// precision, no pluralize, no separator, no adjective. `separator` and
    /// `adjective` are therefore ignored rather than defaulted — the Python
    /// method has no such parameters, so nothing could consume them.
    ///
    /// `has_decimal` is ignored for the same reason. `base.to_currency` needs
    /// it to decide whether the cents segment appears; DV decides that with a
    /// plain truthiness test on `frac_part` instead (quirk 7), and its
    /// `to_decimal` treats an `int` and a `float` identically.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Only the rufiyaa has words (#219): never print a raw ISO code.
        let currency = match currency {
            "ރުފިޔާ" | "MVR" => "ރުފިޔާ",
            other => {
                return Err(N2WError::NotImplemented(format!(
                    "Currency code \"{}\" not implemented for \"{}\"",
                    other,
                    self.lang_name()
                )))
            }
        };
        // decimal_value = self.to_decimal(value). Decimal(int) and
        // Decimal(str(float)) — which is what the shim already handed us.
        let decimal_value = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };

        if decimal_value.is_zero() {
            return Ok(format!("{} {}", self.base_stem(0)?, currency));
        }

        // Python rounds the whole value half-even to an integer and takes the
        // (possibly negative) remainder as cents, so 1.5 was "two rufiyaa
        // minus fifty laari" (#170). Instead: round |value| half-up to whole
        // laari, as `base.to_currency` does, split into rufiyaa and laari, and
        // put the sign in front once. Working in whole laari also expands a
        // positive exponent, so 1e21 is no longer read as "one".
        let negative = decimal_value.is_negative();
        let total = (decimal_value.abs() * BigDecimal::from(100) + BigDecimal::new(BigInt::from(5), 1))
            .with_scale(0)
            .as_bigint_and_exponent()
            .0;
        let (int_part, frac_part) = total.div_rem(&BigInt::from(100));
        if int_part.is_zero() && frac_part.is_zero() {
            // 0.001 rounds to nothing: say zero, not "".
            return Ok(format!("{} {}", self.base_stem(0)?, currency));
        }
        let int_part = BigDecimal::from(int_part);
        let frac_part = BigDecimal::from(frac_part);

        let mut result: Vec<String> = Vec::new();
        if negative {
            result.push(NEGWORD.to_string());
        }

        if !int_part.is_zero() {
            result.push(self.to_cardinal_float_dec(&int_part, false)?);
            result.push(currency.to_string());
        }

        if !frac_part.is_zero() {
            // Python appends `cents` — a *word*, defaulting to ލާރި; the trait
            // hands us `base.to_currency`'s verbosity flag instead. Python's
            // `cents=False` put the bool itself into the list and died in
            // `" ".join(...)`; it now gives the laari as digits, like every
            // other language (#220).
            if cents {
                result.push(self.to_cardinal_float_dec(&frac_part, false)?);
            } else {
                result.push(format!("{:0>2}", frac_part.to_string()));
            }
            result.push(CENTSWORD.to_string());
        }

        Ok(result.join(" "))
    }

    // `Num2Word_DV` had no `to_cheque` (AttributeError).
    // No cheque rules, so NotImplementedError ("lang='dv' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}

/// 10^n.
fn pow10(n: usize) -> BigInt {
    let mut p = BigInt::from(1);
    for _ in 0..n {
        p *= 10;
    }
    p
}

/// The shortest round-trip decimal digits of `f` and its adjusted (power of
/// the leading digit) exponent, via Rust's `{:e}` — whose default precision
/// is "the smallest number of digits necessary to represent the value
/// uniquely", i.e. the same digits CPython's `repr(float)` prints, since the
/// shortest representation of a double is unique.
fn shortest_digits(f: f64) -> (String, i64) {
    let s = format!("{:e}", f); // e.g. "1.234e3", "-5e-1", "0e0"
    let (mant, exp) = s.split_once('e').expect("LowerExp always contains 'e'");
    let digits: String = mant.chars().filter(|c| c.is_ascii_digit()).collect();
    let adj: i64 = exp.parse().expect("LowerExp exponent is an integer");
    (digits, adj)
}

/// Reproduce `Decimal(str(f)).as_tuple()` as `(sign_negative, digit_str,
/// exponent)`, matching Python's `to_decimal(float)`.
///
/// `repr(float)` prints the shortest round-trip digits, fixed-point while the
/// adjusted exponent is in [-4, 16) and scientific outside. The regimes give
/// the Decimal *different* tuples:
///
///   * scientific: the coefficient is the bare digits (`'1e+16'` -> `(1,)`,
///     exponent 16) — quirk 9; the body expands the exponent (#190);
///   * fixed-point, integral value: repr appends ".0", so the coefficient
///     gains the padding zeros plus one more and the exponent is -1
///     (`'1000.0'` -> `(1,0,0,0,0)`, exponent -1) — quirk 10;
///   * fixed-point, fractional value: digits as-is with the natural exponent
///     (`'12.34'` -> `(1,2,3,4)`, exponent -2). The tiny-scientific regime
///     (`'1e-05'`) lands on the same tuple as its fixed expansion would,
///     because `Decimal` strips leading coefficient zeros.
fn py_float_dec_tuple(f: f64) -> (bool, String, i64) {
    let sign_negative = f.is_sign_negative();
    let (m, adj) = shortest_digits(f);
    let ndig = m.chars().count() as i64;
    let e_nat = adj - (ndig - 1);
    if adj >= 16 || adj < -4 {
        // repr is scientific either way; both directions keep the bare digits.
        return (sign_negative, m, e_nat);
    }
    if e_nat >= 0 {
        // Integral fixed-point repr: '.0' appended.
        let mut d = m;
        for _ in 0..=e_nat {
            d.push('0');
        }
        // 0.0's coefficient collapses back to a single 0.
        let trimmed = d.trim_start_matches('0');
        let d = if trimmed.is_empty() { "0".to_string() } else { trimmed.to_string() };
        (sign_negative, d, -1)
    } else {
        (sign_negative, m, e_nat)
    }
}

/// Python's `str(float)` — repr with the same regime split as
/// [`py_float_dec_tuple`]. Only used for exception messages
/// (`errmsg_floatord` / `errmsg_negord` format the original value); the
/// corpora compare exception types, not text.
fn py_float_str(f: f64) -> String {
    let sign = if f.is_sign_negative() { "-" } else { "" };
    let (m, adj) = shortest_digits(f);
    if (-4..16).contains(&adj) {
        let intlen = adj + 1;
        if intlen <= 0 {
            format!("{}0.{}{}", sign, "0".repeat((-intlen) as usize), m)
        } else if intlen as usize >= m.len() {
            format!("{}{}{}.0", sign, m, "0".repeat(intlen as usize - m.len()))
        } else {
            let (a, b) = m.split_at(intlen as usize);
            format!("{}{}.{}", sign, a, b)
        }
    } else {
        let mut chars = m.chars();
        let lead = chars.next().unwrap_or('0');
        let rest: String = chars.collect();
        let coeff = if rest.is_empty() {
            lead.to_string()
        } else {
            format!("{}.{}", lead, rest)
        };
        // Python zero-pads the exponent to two digits: 1e-05, 1e+16.
        format!(
            "{}{}e{}{:02}",
            sign,
            coeff,
            if adj < 0 { '-' } else { '+' },
            adj.abs()
        )
    }
}

/// `self.to_decimal(value).as_tuple()` — the `(sign, digits, exponent)` view
/// every DV float path renders from.
fn float_value_tuple(value: &FloatValue) -> (bool, String, i64) {
    match value {
        // float -> Decimal(str(value)), reconstructed from the f64 repr.
        FloatValue::Float { value, .. } => py_float_dec_tuple(*value),
        // Decimal -> the Decimal unchanged. as_bigint_and_exponent() is
        // (coefficient, scale); Python's exponent is -scale. bigdecimal
        // preserves trailing zeros from the source string, so Decimal("1.10")
        // keeps coefficient 110 / exponent -2 — the trailing zero the corpus
        // depends on.
        FloatValue::Decimal { value, .. } => {
            let (coeff, scale) = value.as_bigint_and_exponent();
            (coeff.is_negative(), coeff.abs().to_string(), -scale)
        }
    }
}

/// `"{}".format(value)` for verify_ordinal's error messages: `str(float)`
/// for a float, `str(Decimal)` for a Decimal.
fn float_value_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => py_float_str(*value),
        FloatValue::Decimal { .. } => {
            let (sign, digits, exponent) = float_value_tuple(value);
            py_decimal_str_parts(sign, &digits, exponent)
        }
    }
}

/// The `nominal=` kwarg under Python truthiness — `if not nominal` is the
/// only test the class ever applies, so False/0/None/""/[] all mean stem.
fn kw_nominal(kw: &Kwargs) -> bool {
    match kw.get("nominal") {
        Option::None => true, // not passed: the default
        Some(KwVal::Bool(b)) => *b,
        Some(KwVal::Int(i)) => *i != 0,
        Some(KwVal::None) => false,
        Some(KwVal::Str(s)) => !s.is_empty(),
        Some(KwVal::List(l)) => !l.is_empty(),
    }
}

/// `str(Decimal)` (the spec's to-scientific-string) from a
/// `(sign, digit_str, exponent)` tuple. Only reached to build an OverflowError
/// *message*; the differential harness compares exception *type*, not text, so
/// this is never on the parity path — implemented in full for fidelity anyway.
fn py_decimal_str_parts(sign_negative: bool, digit_str: &str, exponent: i64) -> String {
    let sign = if sign_negative { "-" } else { "" };
    let chars: Vec<char> = digit_str.chars().collect();
    let ndigits = chars.len() as i64;
    let adjusted = exponent + ndigits - 1;

    if exponent <= 0 && adjusted >= -6 {
        // Fixed-point form.
        if exponent == 0 {
            return format!("{}{}", sign, digit_str);
        }
        let leftdigits = exponent + ndigits; // where the point falls
        let (intpart, fracpart) = if leftdigits <= 0 {
            (
                "0".to_string(),
                format!(".{}{}", "0".repeat((-leftdigits) as usize), digit_str),
            )
        } else {
            let l = leftdigits as usize;
            (
                chars[..l].iter().collect::<String>(),
                format!(".{}", chars[l..].iter().collect::<String>()),
            )
        };
        return format!("{}{}{}", sign, intpart, fracpart);
    }

    // Scientific form.
    let coeff = if ndigits == 1 {
        chars[0].to_string()
    } else {
        format!("{}.{}", chars[0], chars[1..].iter().collect::<String>())
    };
    let esign = if adjusted < 0 { "-" } else { "+" };
    format!("{}{}E{}{}", sign, coeff, esign, adjusted.abs())
}

/// Python's `str(Decimal)` — the spec's *to-scientific-string* — restricted to
/// the integral values this file can reach it with.
///
/// Only two of the spec's three branches can fire. Exponent 0 prints the
/// coefficient plainly; a positive exponent switches to scientific notation
/// keyed on the adjusted exponent, so `Decimal('1.5E+33')` is `"1.5E+33"` and
/// not `"1500000000000000000000000000000000"`. A negative exponent would need
/// the third branch, which integral callers rule out.
fn py_decimal_str(d: &BigDecimal) -> String {
    let (coefficient, scale) = d.as_bigint_and_exponent();
    let exponent = -scale;
    let digits = coefficient.abs().to_string();
    let sign = if coefficient.is_negative() { "-" } else { "" };

    if exponent <= 0 {
        return format!("{}{}", sign, digits);
    }

    // adjusted exponent = exponent + len(digits) - 1
    let adjusted = exponent + digits.chars().count() as i64 - 1;
    let mut chars = digits.chars();
    let lead = chars.next().unwrap_or('0');
    let rest: String = chars.collect();
    let coeff = if rest.is_empty() {
        lead.to_string()
    } else {
        format!("{}.{}", lead, rest)
    };
    // Python prints the exponent's sign explicitly and never zero-pads it.
    let esign = if adjusted < 0 { "-" } else { "+" };
    format!("{}{}E{}{}", sign, coeff, esign, adjusted.abs())
}

/// Python's `int(s)` over an ASCII digit string. Every call site feeds it a
/// slice of at most 3 digits, so `i64` cannot overflow.
fn py_int(s: &str) -> Result<i64> {
    s.parse::<i64>().map_err(|_| {
        N2WError::Value(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        ))
    })
}

/// Python's `s[i]`, raising `IndexError` past the end.
fn char_at(s: &str, i: usize) -> Result<String> {
    s.chars()
        .nth(i)
        .map(|c| c.to_string())
        .ok_or_else(|| N2WError::Index("string index out of range".into()))
}

/// Python's `s[start:end]` with negative-index and clamping semantics.
/// Operates on chars; every caller passes an ASCII digit string.
fn py_slice(s: &str, start: isize, end: isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len() as isize;

    let norm = |i: isize| -> usize {
        if i < 0 {
            let j = len + i;
            if j < 0 {
                0
            } else {
                j as usize
            }
        } else if i > len {
            len as usize
        } else {
            i as usize
        }
    };

    let (a, b) = (norm(start), norm(end));
    if a >= b {
        return String::new();
    }
    chars[a..b].iter().collect()
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use std::str::FromStr;

    fn f(value: f64, precision: u32) -> Result<String> {
        // Fully-qualified: the inherent to_cardinal_float(&BigInt, bool) would
        // otherwise shadow the trait method under method-call syntax.
        Lang::to_cardinal_float(
            &LangDv::new(),
            &FloatValue::Float { value, precision },
            None,
        )
    }

    fn d(s: &str) -> Result<String> {
        Lang::to_cardinal_float(
            &LangDv::new(),
            &FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                // DV ignores precision; a plausible value all the same.
                precision: s.split_once('.').map_or(0, |(_, fr)| fr.len() as u32),
            },
            None,
        )
    }

    /// Values below 1 raised IndexError in Python (quirk 11); since #158
    /// they read with a zero integer part, like `1.5` reads with "one".
    fn reads_zero_point(r: &Result<String>) -> bool {
        matches!(r, Ok(w) if w.contains("ސުމެއް ޕޮއިންޓް"))
    }

    #[test]
    fn float_cardinal_matches_corpus_and_interpreter() {
        // Quirk 11 (fixed, #158): Python raised IndexError below 1.
        assert_eq!(f(0.5, 1).unwrap(), "ސުމެއް ޕޮއިންޓް ފަހެއް");
        assert_eq!(f(0.05, 2).unwrap(), "ސުމެއް ޕޮއިންޓް ސުމެއް ފަހެއް");
        assert_eq!(f(1.5, 1).unwrap(), "އެކެއް ޕޮއިންޓް ފަހެއް");
        assert_eq!(f(3.14, 2).unwrap(), "ތިނެއް ޕޮއިންޓް އެކެއް ހަތަރެއް");
        assert_eq!(f(12.34, 2).unwrap(), "ބާރަ ޕޮއިންޓް ތިނެއް ހަތަރެއް");
        assert_eq!(f(-12.34, 2).unwrap(), "މައިނަސް ބާރަ ޕޮއިންޓް ތިނެއް ހަތަރެއް");
        assert_eq!(f(1.005, 3).unwrap(), "އެކެއް ޕޮއިންޓް ސުމެއް ސުމެއް ފަހެއް");
        assert_eq!(f(2.675, 3).unwrap(), "ދޭއް ޕޮއިންޓް ހައެއް ހަތެއް ފަހެއް");
        assert!(reads_zero_point(&f(0.01, 2)));
        assert_eq!(f(99.99, 2).unwrap(), "ނުވަދިހަނުވައެއް ޕޮއިންޓް ނުވައެއް ނުވައެއް");
        assert_eq!(f(1.0, 1).unwrap(), "އެކެއް ޕޮއިންޓް ސުމެއް");
        assert!(reads_zero_point(&f(0.0, 1)));
        assert!(reads_zero_point(&f(-0.5, 1)));
        assert_eq!(f(100.5, 1).unwrap(), "ސަތޭކަ ޕޮއިންޓް ފަހެއް");
        assert_eq!(
            f(1234.56, 2).unwrap(),
            "އެއްހާސް ދުއިސައްތަތިރީސްހަތަރެއް ޕޮއިންޓް ފަހެއް ހައެއް"
        );
        assert_eq!(f(-1.5, 1).unwrap(), "މައިނަސް އެކެއް ޕޮއިންޓް ފަހެއް");
        assert!(reads_zero_point(&f(0.1, 1)));
        assert!(reads_zero_point(&f(0.99, 2)));
        assert_eq!(f(1.01, 2).unwrap(), "އެކެއް ޕޮއިންޓް ސުމެއް އެކެއް");
        assert_eq!(f(2.25, 2).unwrap(), "ދޭއް ޕޮއިންޓް ދޭއް ފަހެއް");
    }

    #[test]
    fn decimal_cardinal_matches_corpus() {
        assert!(reads_zero_point(&d("0.01")));
        assert_eq!(d("1.10").unwrap(), "އެކެއް ޕޮއިންޓް އެކެއް ސުމެއް");
        assert_eq!(d("12.345").unwrap(), "ބާރަ ޕޮއިންޓް ތިނެއް ހަތަރެއް ފަހެއް");
        assert_eq!(
            d("98746251323029.99").unwrap(),
            "ނުވަދިހައައްޓްރިޔަން ހަތްސަތޭކަ ސާޅީސްހަބިލިޔަން ދުއިސައްތަފަންސާސްއެއްމިލިޔަން ތިންލައްކަ ތޭވީސްހާސް ނަވާވީސް ޕޮއިންޓް ނުވައެއް ނުވައެއް"
        );
        assert!(reads_zero_point(&d("0.001")));
        assert_eq!(d("1.005").unwrap(), "އެކެއް ޕޮއިންޓް ސުމެއް ސުމެއް ފަހެއް");
        assert_eq!(d("2.675").unwrap(), "ދޭއް ޕޮއިންޓް ހައެއް ހަތެއް ފަހެއް");
        assert!(reads_zero_point(&d("0.50")));
    }

    #[test]
    fn whole_float_entries_match_corpus() {
        let l = LangDv::new();
        let fv = |v: f64| FloatValue::Float { value: v, precision: 1 };

        // cardinal: no whole-value fast path (quirk 10) + scientific reprs
        // (quirk 9, fixed in #190) + zero integer part (quirk 11, fixed in #158).
        assert_eq!(
            l.cardinal_float_entry(&fv(5.0), None).unwrap(),
            "ފަހެއް ޕޮއިންޓް ސުމެއް"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv(-2.0), None).unwrap(),
            "މައިނަސް ދޭއް ޕޮއިންޓް ސުމެއް"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv(-1000.0), None).unwrap(),
            "މައިނަސް އެއްހާސް ޕޮއިންޓް ސުމެއް"
        );
        // Scientific reprs read the full value, not the coefficient (#190).
        let ten = |e: u32| BigInt::from(10).pow(e);
        assert_eq!(
            l.cardinal_float_entry(&fv(1e16), None).unwrap(),
            l.to_cardinal(&ten(16)).unwrap()
        );
        assert_eq!(
            l.cardinal_float_entry(&fv(1e20), None).unwrap(),
            l.to_cardinal(&ten(20)).unwrap()
        );
        assert_eq!(
            l.cardinal_float_entry(&fv(0.0), None).unwrap(),
            "ސުމެއް ޕޮއިންޓް ސުމެއް"
        );
        assert_eq!(
            l.cardinal_float_entry(&fv(-0.0), None).unwrap(),
            "މައިނަސް ސުމެއް ޕޮއިންޓް ސުމެއް"
        );

        // ordinal: verify_ordinal order (quirk 12) + nominal/stem mix (13).
        assert_eq!(
            l.ordinal_float_entry(&fv(1.0)).unwrap(),
            "އެކެއް ޕޮއިންޓް ސުން ވަނަ"
        );
        assert_eq!(
            l.ordinal_float_entry(&fv(1e16)).unwrap(),
            l.to_ordinal(&ten(16)).unwrap()
        );
        assert!(matches!(l.ordinal_float_entry(&fv(2.5)), Err(N2WError::Type(_))));
        assert!(matches!(l.ordinal_float_entry(&fv(-1.5)), Err(N2WError::Type(_))));
        assert!(matches!(l.ordinal_float_entry(&fv(-1.0)), Err(N2WError::Type(_))));
        assert_eq!(
            l.ordinal_float_entry(&fv(-0.0)).unwrap(),
            "މައިނަސް ސުމެއް ޕޮއިންޓް ސުން ވަނަ"
        );

        // ordinal_num: -0.0 passes verify and echoes the repr.
        assert_eq!(
            l.ordinal_num_float_entry(&fv(-0.0), "-0.0").unwrap(),
            "-0.0 ވަނަ"
        );
        assert!(matches!(
            l.ordinal_num_float_entry(&fv(-1.0), "-1.0"),
            Err(N2WError::Type(_))
        ));

        // year: abs-before-cardinal + the [1100, 2000) float split (quirk 14).
        assert_eq!(
            l.year_float_entry(&fv(-1.5)).unwrap(),
            "އެކެއް ޕޮއިންޓް ފަހެއް ބީ.ސީ"
        );
        assert_eq!(
            l.year_float_entry(&fv(1234.0)).unwrap(),
            "ބާރަ ޕޮއިންޓް ސުމެއް ސަތޭކަ ތިރީސްހަތަރެއް ޕޮއިންޓް ސުމެއް"
        );
        assert_eq!(
            l.year_float_entry(&fv(1e16)).unwrap(),
            l.to_cardinal(&ten(16)).unwrap()
        );
        assert_eq!(
            l.year_float_entry(&fv(-0.0)).unwrap(),
            "މައިނަސް ސުމެއް ޕޮއިންޓް ސުމެއް"
        );
    }

    #[test]
    fn decimal_entries_match_corpus() {
        let l = LangDv::new();
        let dv = |s: &str| FloatValue::Decimal {
            value: BigDecimal::from_str(s).unwrap(),
            precision: 0,
        };

        // A positive exponent is expanded, not dropped (#190).
        let ten = |e: u32| BigInt::from(10).pow(e);
        assert_eq!(
            l.cardinal_float_entry(&dv("1E+2"), None).unwrap(),
            l.to_cardinal(&ten(2)).unwrap()
        );
        assert_eq!(
            l.cardinal_float_entry(&dv("1E+20"), None).unwrap(),
            l.to_cardinal(&ten(20)).unwrap()
        );
        assert_eq!(l.ordinal_float_entry(&dv("0")).unwrap(), "ސުން ވަނަ");
        assert_eq!(l.ordinal_float_entry(&dv("5")).unwrap(), "ފަސް ވަނަ");
        assert_eq!(l.ordinal_float_entry(&dv("100")).unwrap(), "ސަތޭކަ ވަނަ");
        assert_eq!(
            l.ordinal_float_entry(&dv("5.00")).unwrap(),
            "ފަހެއް ޕޮއިންޓް ސުން ސުން ވަނަ"
        );
        assert_eq!(
            l.ordinal_float_entry(&dv("1E+2")).unwrap(),
            l.to_ordinal(&ten(2)).unwrap()
        );
        assert!(matches!(l.ordinal_float_entry(&dv("-3.0")), Err(N2WError::Type(_))));
        assert_eq!(
            l.year_float_entry(&dv("-3.0")).unwrap(),
            "ތިނެއް ޕޮއިންޓް ސުމެއް ބީ.ސީ"
        );
        assert_eq!(
            l.year_float_entry(&dv("1E+2")).unwrap(),
            l.to_year(&ten(2)).unwrap()
        );
        assert_eq!(
            l.year_float_entry(&dv("12345.000")).unwrap(),
            "ބާރަހާސް ތިންސަތޭކަ ސާޅީސްފަހެއް ޕޮއިންޓް ސުމެއް ސުމެއް ސުމެއް"
        );
    }

    #[test]
    fn strings_fraction_and_kwargs_match_corpus() {
        use crate::base::{KwVal, Kwargs};
        let l = LangDv::new();

        // str_to_number: TypeError instead of InvalidOperation; NaN
        // short-circuits the compare's InvalidOperation.
        assert!(matches!(l.str_to_number("abc"), Err(N2WError::Type(_))));
        assert!(matches!(l.str_to_number(""), Err(N2WError::Type(_))));
        assert!(matches!(
            l.str_to_number("NaN"),
            Err(N2WError::Custom { class: "InvalidOperation", .. })
        ));
        assert!(matches!(l.str_to_number("1e3"), Ok(ParsedNumber::Dec(_))));

        // to_fraction: no fraction rules (#217) — NotImplementedError, and
        // ZeroDivisionError for 1/0.
        assert!(matches!(
            l.to_fraction(&BigInt::from(2), &BigInt::from(3)),
            Err(N2WError::NotImplemented(_))
        ));
        assert!(matches!(
            l.to_fraction(&BigInt::from(1), &BigInt::from(0)),
            Err(N2WError::ZeroDivision(_))
        ));

        // nominal= kwarg (the kwargs corpus rows).
        let kw = |v: KwVal| Kwargs(vec![("nominal".to_string(), v)]);
        assert_eq!(
            l.to_cardinal_kw(&BigInt::from(1234), &kw(KwVal::Bool(false))).unwrap(),
            "އެއްހާސް ދުއިސައްތަތިރީސްހަތަރު"
        );
        assert_eq!(
            l.to_cardinal_kw(&BigInt::from(-5), &kw(KwVal::Bool(false))).unwrap(),
            "މައިނަސް ފަސް"
        );
        assert_eq!(
            l.to_cardinal_kw(&BigInt::from(0), &kw(KwVal::Bool(false))).unwrap(),
            "ސުން"
        );
        assert_eq!(
            l.to_cardinal_kw(&BigInt::from(0), &kw(KwVal::Bool(true))).unwrap(),
            "ސުމެއް"
        );
        assert!(matches!(
            l.to_cardinal_kw(
                &BigInt::from(1),
                &Kwargs(vec![("case".to_string(), KwVal::Str("dative".into()))])
            ),
            Err(N2WError::Fallback(_))
        ));

        // suffix= kwarg on to_year.
        let skw = Kwargs(vec![("suffix".to_string(), KwVal::Str("AD".into()))]);
        assert_eq!(l.to_year_kw(&BigInt::from(5), &skw).unwrap(), "ފަހެއް AD");
        assert_eq!(l.to_year_kw(&BigInt::from(-17), &skw).unwrap(), "ސަތާރަ AD");
        assert_eq!(
            l.to_year_kw(
                &BigInt::from(-17),
                &Kwargs(vec![("suffix".to_string(), KwVal::None)])
            )
            .unwrap(),
            "ސަތާރަ ބީ.ސީ"
        );
    }

    #[test]
    fn overflow_boundary_is_maxval() {
        // OverflowError exactly when floor(|v|) >= 10**33 (quirk 4, #159:
        // Python's 28-digit rounding of abs() is no longer reproduced).
        let overflow = |s: &str| matches!(d(s), Err(N2WError::Overflow(_)));

        assert!(d(&"9".repeat(33)).is_ok()); // 10**33 - 1 converts
        assert!(overflow(&format!("1{}", "0".repeat(33)))); // 10**33
        assert!(d(&(10u128.pow(33) - 50000).to_string()).is_ok());
        assert!(d(&(10u128.pow(33) - 50001).to_string()).is_ok()); // just under
        assert!(d(&"9".repeat(28)).is_ok()); // 28 digits, no rounding
        assert!(d(&format!("1{}", "0".repeat(32))).is_ok()); // 10**32
        // A fractional value floors below the bound and converts fine.
        assert!(d(&format!("{}.5", "9".repeat(32))).is_ok());
    }
}
