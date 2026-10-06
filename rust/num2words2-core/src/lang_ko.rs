//! Port of `lang_KO.py` (Korean).
//!
//! Shape: **engine**. `Num2Word_KO` subclasses `Num2Word_Base` directly and
//! supplies `high_numwords` / `mid_numwords` / `low_numwords` plus a `merge`,
//! so Python builds `self.cards` and lets `Num2Word_Base.to_cardinal` drive
//! `splitnum`/`clean`. Cards, maxval and merge are therefore all live here and
//! `to_cardinal` stays at the trait default (`base::default_to_cardinal`).
//!
//! # Card table
//!
//! Korean groups by **myriads** (10^4), not thousands. KO overrides
//! `set_high_numwords` with a step of -4:
//!
//! ```text
//! max = 4 * len(high)              # 4 * 17 = 68
//! zip(high, range(max, 0, -4))     # 68, 64, 60, ... 8, 4
//! ```
//!
//! giving 무량대수=10^68 … 조=10^12, 억=10^8, 만=10^4, then mid 천=1000,
//! 백=100 and low 십=10 … 일=1, 영=0. Insertion order is already strictly
//! descending, so `Cards`' sorted-descending storage reproduces Python's
//! `OrderedDict` iteration order exactly.
//!
//! `MAXVAL = 1000 * next(iter(cards))` = 1000 * 10^68 = **10^71**. Note the
//! `1000` factor is `Num2Word_Base.__init__`'s hardcoded short-scale
//! assumption; it does not match KO's myriad grouping, but it is what Python
//! computes, so the overflow threshold is 10^71 and not 10^72.
//!
//! # merge quirks worth knowing
//!
//! * `lnum == 1 and rnum <= 10000` swallows the leading 일, which is why
//!   10^4 renders "만" (bare) while 10^8 renders "일억" — 10^8 exceeds the
//!   `<= 10000` cutoff and falls through to the multiplying `else` arm.
//! * `lnum >= 10000 and lnum > rnum` is the only arm that emits a **space**,
//!   so spaces separate myriad groups: 12345 → "만 이천삼백사십오".
//!
//! # to_ordinal
//!
//! Only the **last space-separated word** is converted to native-Korean
//! ordinal counting words; everything left of it keeps Sino-Korean cardinal
//! form. Hence 999999 → "구십구만 구천구백아흔아홉 번째" (the leading
//! "구십구만" is untouched).
//!
//! `value % 100 == 0` skips the rewrite entirely, so 100 → "백 번째" and
//! 10^12 → "일조 번째" keep cardinal words.
//!
//! Python splits the last word with the zero-width regex
//! `re.split("(?<=천)|(?<=백)", w)`, i.e. cut *after* every 천 and 백.
//! [`split_after_mid`] reproduces this exactly, including the trailing empty
//! string Python yields when the word ends in 천/백 (e.g. "천백" →
//! `["천", "백", ""]`). That trailing-empty case is unreachable from
//! `to_ordinal` because a word ending in 천/백 implies `value % 100 == 0`,
//! which the guard above already excludes — but the helper matches Python
//! regardless.
//!
//! # Faithfully reproduced Python behaviour
//!
//! * `to_ordinal(0)` → "영 번째". `0 % 100 == 0` skips the ords rewrite, so
//!   the Sino-Korean 영 survives rather than becoming a counting word.
//! * `to_ordinal(1)` short-circuits to "첫 번째" before `to_cardinal` runs.
//! * The `if not ten_one[1]` arm rewrites 스물 → 스무 only for bare tens, so
//!   20 → "스무 번째" but 21 → "스물한 번째".
//! * `to_ordinal_num` ignores every table and just formats the digits:
//!   `"%s 번째" % value` → 0 → "0 번째".
//! * Negative input to `to_ordinal` / `to_ordinal_num` raises `TypeError` via
//!   `Num2Word_Base.verify_ordinal` (→ [`N2WError::Type`]). `to_cardinal` and
//!   `to_year` accept negatives.
//!
//! # Currency
//!
//! `Num2Word_KO` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base`'s version. It never calls `pluralize`,
//! `_money_verbose`, `_cents_verbose` or `_cents_terse` — it goes straight to
//! `to_cardinal` — so those hooks all stay at their trait defaults. See
//! [`LangKo::to_currency`] for the divergences, which are substantial.
//!
//! `to_cheque` is *not* overridden in Python, so `Num2Word_Base.to_cheque`
//! ran against KO's table and tripped over its 1-tuples (ValueError for the
//! default KRW). The port raises NotImplementedError ("does not support
//! to='cheque'", #223).
//!
//! `CURRENCY_PRECISION` and `CURRENCY_ADJECTIVES` are both `{}` for KO
//! (verified against the live interpreter: KO subclasses `Num2Word_Base`
//! directly, and `Num2Word_EN.__init__` *rebinds* `CURRENCY_PRECISION` on the
//! instance rather than mutating the class dict, so nothing leaks in). KO also
//! declares its own `CURRENCY_FORMS` class attribute, so EN's in-place
//! mutation of `Num2Word_EUR.CURRENCY_FORMS` cannot reach it — hence EUR/GBP
//! and EN's ~24 added codes all raise NotImplementedError here.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{parse_currency_parts, CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

/// Emulates Python's `re.split("(?<=천)|(?<=백)", s)`.
///
/// Both alternatives are zero-width lookbehinds, so the split points sit
/// *after* each 천 / 백. Python 3.7+ splits on empty matches, which means a
/// word ending in 천/백 produces a trailing empty string — reproduced here.
///
/// ```text
/// "천이백삼십사" -> ["천", "이백", "삼십사"]
/// "구백구십구"   -> ["구백", "구십구"]
/// "십"           -> ["십"]
/// "천백"         -> ["천", "백", ""]
/// ```
fn split_after_mid(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        cur.push(c);
        if c == '천' || c == '백' {
            out.push(std::mem::take(&mut cur));
        }
    }
    out.push(cur);
    out
}

/// `Num2Word_KO.CURRENCY_FORMS`, exactly as the class body declares it:
///
/// ```python
/// CURRENCY_FORMS = {
///     "KRW": ("원",),            # KRW doesn't use fractional units
///     "USD": ("달러", "센트"),
///     "JPY": ("엔",),            # JPY doesn't use fractional units
/// }
/// ```
///
/// The arity is load-bearing twice over, so the 1-tuples map to an **empty**
/// `subunit` rather than a duplicated or invented one:
///
/// * `to_currency` does `minor = curr_forms[1] if len(curr_forms) > 1 else None`
///   — an empty `subunit` is what makes KRW/JPY take the no-minor-unit branch.
/// * `to_cheque` does `cr1, _cr2 = self.CURRENCY_FORMS[currency]`, a 2-tuple
///   unpack that *raises* on the 1-tuples.
///
/// Note each side holds a single form, not a singular/plural pair: Korean does
/// not inflect for number, and KO never calls `pluralize`.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("KRW", CurrencyForms::new(&["원"], &[]));
    m.insert("USD", CurrencyForms::new(&["달러"], &["센트"]));
    m.insert("JPY", CurrencyForms::new(&["엔"], &[]));
    m
}

pub struct LangKo {
    cards: Cards,
    maxval: BigInt,
    ords: HashMap<&'static str, &'static str>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangKo {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKo {
    pub fn new() -> Self {
        // Python `high_numwords`, most significant first.
        const HIGH: [&str; 17] = [
            "무량대수",
            "불가사의",
            "나유타",
            "아승기",
            "항하사",
            "극",
            "재",
            "정",
            "간",
            "구",
            "양",
            "자",
            "해",
            "경",
            "조",
            "억",
            "만",
        ];

        let mut cards = Cards::new();

        // KO's set_high_numwords: max = 4 * len(high); zip(high, range(max, 0, -4)).
        // zip stops at the shorter sequence; here both have 17 entries.
        let max = 4 * HIGH.len() as u32;
        let mut n = max;
        for word in HIGH.iter() {
            if n == 0 {
                break;
            }
            cards.insert(BigInt::from(10u8).pow(n), *word);
            n -= 4;
        }

        set_mid_numwords(&mut cards, &[(1000, "천"), (100, "백")]);
        set_low_numwords(
            &mut cards,
            &["십", "구", "팔", "칠", "육", "오", "사", "삼", "이", "일", "영"],
        );

        // Num2Word_Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0]
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let ords: HashMap<&str, &str> = [
            ("일", "한"),
            ("이", "두"),
            ("삼", "세"),
            ("사", "네"),
            ("오", "다섯"),
            ("육", "여섯"),
            ("칠", "일곱"),
            ("팔", "여덟"),
            ("구", "아홉"),
            ("십", "열"),
            ("이십", "스물"),
            ("삼십", "서른"),
            ("사십", "마흔"),
            ("오십", "쉰"),
            ("육십", "예순"),
            ("칠십", "일흔"),
            ("팔십", "여든"),
            ("구십", "아흔"),
        ]
        .into_iter()
        .collect();

        LangKo {
            cards,
            maxval,
            ords,
            // Built once here, never per call.
            currency_forms: build_currency_forms(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check is vacuous for BigInt
    /// input; only the negative check can fire.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `self.ords.get(key, key)` — miss returns the key itself.
    fn ord_or_self(&self, key: &str) -> String {
        self.ords
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }
}

impl Lang for LangKo {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "KRW"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        " "
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    fn negword(&self) -> &str {
        "마이너스 "
    }

    fn pointword(&self) -> &str {
        "점"
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let myriad = BigInt::from(10000);

        if lnum.is_one() && rnum <= &myriad {
            // Swallows the leading 일: 10^4 -> "만", not "일만".
            (rtext.to_string(), rnum.clone())
        } else if &myriad > lnum && lnum > rnum {
            (format!("{}{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &myriad && lnum > rnum {
            // The only arm that emits a space — separates myriad groups.
            (format!("{} {}", ltext, rtext), lnum + rnum)
        } else {
            (format!("{}{}", ltext, rtext), lnum * rnum)
        }
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        if value.is_one() {
            return Ok("첫 번째".to_string());
        }

        let cardinal = self.to_cardinal(value)?;
        let mut outwords: Vec<String> = cardinal.split(' ').map(|s| s.to_string()).collect();

        if !(value % BigInt::from(100)).is_zero() {
            // `outwords` is never empty: split(' ') always yields >= 1 item.
            let last = outwords[outwords.len() - 1].clone();
            let mut lastwords = split_after_mid(&last);
            let li = lastwords.len() - 1;

            if lastwords[li].contains('십') {
                // str.split("십") on a string containing 십 yields >= 2 parts,
                // so ten_one[0] / ten_one[1] are always in range. Guarded
                // anyway: Python would raise IndexError here.
                let mut ten_one: Vec<String> =
                    lastwords[li].split('십').map(|s| s.to_string()).collect();
                if ten_one.len() < 2 {
                    return Err(N2WError::Index("list index out of range".into()));
                }
                ten_one[0] = self.ord_or_self(&format!("{}십", ten_one[0]));
                ten_one[1] = self.ord_or_self(&ten_one[1].clone());
                if ten_one[1].is_empty() {
                    // Bare tens: 스물 -> 스무 (20 -> "스무 번째").
                    ten_one[0] = ten_one[0].replace("스물", "스무");
                }
                lastwords[li] = ten_one.concat();
            } else {
                lastwords[li] = self.ord_or_self(&lastwords[li].clone());
            }

            let n = outwords.len();
            outwords[n - 1] = lastwords.concat();
        }

        // Python: " ".join(x.strip() for x in outwords if x.strip()) + " 번째"
        let joined = outwords
            .iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        Ok(format!("{} 번째", joined))
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{} 번째", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // Python signature: to_year(val, suffix=None, longval=True). The trait
        // exposes only `value`, so `suffix` is always None on entry and can
        // only be set by the negative branch.
        let mut val = value.clone();
        let mut suffix: Option<&str> = None;
        if val.is_negative() {
            val = val.abs();
            suffix = Some("기원전");
        }
        let valtext = self.to_cardinal(&val)?;
        Ok(match suffix {
            Some(s) => format!("{} {}", s, valtext),
            None => valtext,
        })
    }

    /// `to_ordinal(float/Decimal)`. `verify_ordinal` runs first: a
    /// non-integral value raises TypeError (`errmsg_floatord`), then the
    /// negative check raises TypeError (`errmsg_negord`) — the latter is
    /// reproduced inside `to_ordinal(BigInt)`'s own verify. `-0.0` passes
    /// both checks (`abs(-0.0) == -0.0`) and lands on the 0 path.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            // Whole: to_ordinal's own verify_ordinal rejects negatives.
            Some(i) => self.to_ordinal(&i),
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_ordinal_num(float/Decimal)`: same `verify_ordinal`, then
    /// `"%s 번째" % value` — the `%s` is Python's `str(value)`, i.e. the
    /// repr the binding computed ("5.0 번째", "1e+16 번째", "-0.0 번째").
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            Some(i) => {
                if i.is_negative() {
                    return Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        repr_str
                    )));
                }
                Ok(format!("{} 번째", repr_str))
            }
            None => Err(N2WError::Type(
                "Cannot treat float as ordinal.".into(),
            )),
        }
    }

    /// `to_year(float/Decimal)`: `if val < 0` — a *numeric* test, so `-0.0`
    /// takes the positive arm — then `기원전 ` + cardinal of the absolute
    /// value, which keeps its float grammar ("기원전 일 점 오").
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if negative {
            let abs = match value {
                FloatValue::Float { value, precision } => FloatValue::Float {
                    value: value.abs(),
                    precision: *precision,
                },
                FloatValue::Decimal { value, precision } => FloatValue::Decimal {
                    value: value.abs(),
                    precision: *precision,
                },
            };
            Ok(format!("기원전 {}", self.cardinal_float_entry(&abs, None)?))
        } else {
            self.cardinal_float_entry(value, None)
        }
    }

    // ---- currency -------------------------------------------------------
    //
    // `currency_precision` and `currency_adjective` stay at their defaults:
    // KO's `CURRENCY_PRECISION` / `CURRENCY_ADJECTIVES` are both `{}`, so
    // `.get(code, 100)` is always 100 and the adjective lookup always misses.
    //
    // `pluralize`, `money_verbose`, `cents_verbose` and `cents_terse` also
    // stay at their defaults — KO's `to_currency` never calls any of them.
    // (`money_verbose`'s default *is* reached, but only via `to_cheque`.)

    fn lang_name(&self) -> &str {
        "Num2Word_KO"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_KO.to_currency` — a wholesale override that shares almost
    /// nothing with `Num2Word_Base.to_currency`. The divergences are all
    /// deliberate ports, not cleanups:
    ///
    /// * **No space before the unit.** Python emits `"%s%s%s"`, so 100 USD is
    ///   "백달러", not "백 달러". Base would insert a space.
    /// * **`negword` is used unstripped.** Python writes `self.negword`, not
    ///   `self.negword.strip()` as Base does. KO's negword already carries the
    ///   trailing space ("마이너스 "), so the output happens to look right —
    ///   but it arrives by a different route, and a caller who changed negword
    ///   would see the difference.
    /// * **The divisor is hardcoded to 100.** Python calls
    ///   `parse_currency_parts(...)` without a `divisor=` argument and computes
    ///   `(decimal_val * 100) % 1`, never consulting `CURRENCY_PRECISION`. So
    ///   `self.currency_precision()` is deliberately *not* called here. It is
    ///   moot today (KO's table is empty → always 100) but it is why 100.5 KRW
    ///   raises instead of rounding to a whole won: cents are still extracted
    ///   at 1/100 even for a currency KO itself calls fractionless.
    /// * **`has_decimal` is never consulted.** Base gates the cents segment on
    ///   `isinstance(val, float) or "." in str(val)`; KO gates it purely on
    ///   `isinstance(val, int)`. So `Decimal("5")` renders "오달러 영센트"
    ///   here where Base would say "five dollars". Hence the `has_decimal`
    ///   field of [`CurrencyValue::Decimal`] is intentionally ignored.
    /// * **`adjective` is accepted and silently discarded.** No
    ///   `prefix_currency` call exists in KO, so the kwarg is inert.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // Python's `separator=" "` default; `None` means the kwarg was omitted.
        let separator = separator.unwrap_or(self.default_separator());

        // is_integer_input = isinstance(val, int)
        let is_integer_input = matches!(val, CurrencyValue::Int(_));

        // decimal_val = Decimal(str(val))
        // has_fractional_cents = (decimal_val * 100) % 1 != 0
        //
        // The literal 100 is Python's, not CURRENCY_PRECISION — see above.
        // Testing `!= 0` makes truncation-vs-floor irrelevant for negatives:
        // both leave a zero remainder exactly when the value is integral.
        let decimal_val: BigDecimal = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        // parse_currency_parts(val, is_int_with_cents=False,
        //                      keep_precision=has_fractional_cents)
        // — divisor left at its 100 default.
        let (left, right, is_negative) = parse_currency_parts(val, false, has_fractional_cents, 100);

        // try: curr_forms = self.CURRENCY_FORMS[currency] ... except KeyError
        //
        // Note the message: KO says `Currency "X" ...`, where Base's
        // `to_cheque` (and `to_currency`) say `Currency code "X" ...`. Both
        // spellings are live in this file; do not unify them.
        let forms = self.currency_forms(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        // major = curr_forms[0] — every KO entry has one, so the fallback is
        // unreachable; Python would raise IndexError on an empty tuple.
        let major = forms.unit.first().map(String::as_str).unwrap_or("");
        // minor = curr_forms[1] if len(curr_forms) > 1 else None
        let minor: Option<&str> = forms.subunit.first().map(String::as_str);

        // minus_str = self.negword if is_negative else ""  (NOT .strip()'d)
        let minus_str = if is_negative { self.negword() } else { "" };
        let money_str = self.to_cardinal(&left)?;

        // Currencies without a minor unit (KRW, JPY).
        let minor = match minor {
            None => {
                // A float carrying cents is rejected outright rather than
                // rounded down to a whole unit the way Base would. Note the
                // single quotes around the code — Python's own spelling.
                if !is_integer_input && right > BigDecimal::zero() {
                    return Err(N2WError::Value(format!(
                        "Currency '{}' does not support decimals",
                        currency
                    )));
                }
                return Ok(format!("{}{}{}", minus_str, money_str, major));
            }
            Some(m) => m,
        };

        // Pure ints never show cents.
        if is_integer_input {
            return Ok(format!("{}{}{}", minus_str, money_str, major));
        }

        // Python: `isinstance(right, Decimal) and has_fractional_cents`.
        // `keep_precision` *is* `has_fractional_cents`, and it is exactly what
        // decides whether parse_currency_parts returns a Decimal or an int, so
        // the isinstance check is redundant with the flag and collapses away.
        let cents_str = if has_fractional_cents {
            // cents_str = self.to_cardinal_float(float(right))
            //
            // `right` is provably non-integral here: cents = (val - int(val))
            // * 100 differs from val * 100 only by an integer, so it carries
            // the same non-zero fraction. That makes the trait's
            // `cardinal_from_decimal` an exact stand-in — its f64 cast and
            // repr-derived precision reproduce `to_cardinal_float(float(...))`
            // step for step.
            self.cardinal_from_decimal(&right)?
        } else if cents {
            // self.to_cardinal(int(right)) if right > 0 else self.to_cardinal(0)
            // Both arms are to_cardinal(right) since right >= 0; kept split to
            // mirror the source.
            let right_int = right.with_scale(0).as_bigint_and_exponent().0;
            if right_int > BigInt::zero() {
                self.to_cardinal(&right_int)?
            } else {
                self.to_cardinal(&BigInt::zero())?
            }
        } else {
            // str(int(right)) — bare, NOT zero-padded the way Base's
            // `_cents_terse` would be. 0.01 USD gives "1센트", not "01센트".
            right.with_scale(0).as_bigint_and_exponent().0.to_string()
        };

        Ok(format!(
            "{}{}{}{}{}{}",
            minus_str, money_str, major, separator, cents_str, minor
        ))
    }

    // KO's KRW/JPY forms are 1-tuples, so Base's unpack raised ValueError for
    // the default currency; only USD produced a cheque.
    // No cheque rules, so NotImplementedError ("lang='ko' does not support
    // to='cheque'", #223).
    fn to_cheque(&self, _val: &BigDecimal, _currency: &str) -> Result<String> {
        Err(crate::base::unsupported_mode("cheque"))
    }
}
