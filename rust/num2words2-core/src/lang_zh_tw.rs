//! Port of `lang_ZH_TW.py` (Traditional Chinese, Taiwan).
//!
//! Registry check: `CONVERTER_CLASSES["zh_TW"]` → `lang_ZH_TW.Num2Word_ZH_TW`,
//! so this file ports the class the key actually resolves to.
//!
//! Inheritance chain: `Num2Word_ZH_TW` → `Num2Word_ZH` → `Num2Word_Base`.
//!
//! Shape: **engine**. `Num2Word_ZH.setup` (overridden wholesale by
//! `Num2Word_ZH_TW.setup`) defines `high_numwords`/`mid_numwords`/
//! `low_numwords`, so `Num2Word_Base.__init__` builds `self.cards` and sets
//! `MAXVAL = 1000 * 10**72 == 10**75`. `Num2Word_ZH.merge` folds the
//! `splitnum`/`clean` tree.
//!
//! `Num2Word_ZH_TW.to_cardinal` does **not** call `super()`. It re-implements
//! `Num2Word_Base.to_cardinal` inline (only to run `select_text` over
//! `negword`) and then post-processes, bypassing `Num2Word_ZH.to_cardinal`
//! entirely. The resulting order is load-bearing — see the `一十` note below:
//!
//! ```text
//! out = title(negword_prefix + words)   # is_title is False → identity
//! out = zh_to_cap(out, capital)         # BEFORE the space strip
//! return out.replace(" ", "")
//! ```
//!
//! # The tuple tables, and why this port stores plain strings
//!
//! Where `Num2Word_ZH` holds bare strings, `Num2Word_ZH_TW` re-declares every
//! numword as a nested tuple of `((hanzi, alt...), (bopomofo...))` — e.g.
//! `10` is `(("十", "拾"), ("ㄕˊ",))`. `select_text` resolves one at use time:
//!
//!   * `reading is True`  → `text[1]`, the 注音 (bopomofo) forms;
//!   * otherwise          → `text[0]`, the hanzi forms;
//!   * then, among the remaining alternatives, the single member of
//!     `set(text) & set(self.prefer)` if exactly one matches, else `text[0]`.
//!
//! The Rust `Lang` trait has no `reading`/`prefer` parameters, so this port
//! fixes them at their defaults (`reading=False`, `prefer=None`). Under those
//! defaults `select_text` is always "first form of the first tuple", which is
//! a pure function of the table — so the tables are pre-resolved to plain
//! strings here and `select_text` collapses to the identity. The capital
//! (`大寫`) forms "拾/玖/捌/…", the bopomofo readings, and `CAP_map` are
//! therefore intentionally absent: no in-scope call can reach them. See
//! `concerns` in the report.
//!
//! # Faithfully reproduced Python quirks
//!
//! 1. **`zh_to_cap` strips a leading `一` only from non-negative values.**
//!    `splitnum`/`merge` render 10 as "一十", and the non-capital branch of
//!    `zh_to_cap` fixes that up with
//!    `elif out.startswith(one + ten): out = out[len(one):]`. But for a
//!    negative value `to_cardinal` has already prefixed "負 ", so the string
//!    no longer *starts with* "一十" and the strip never fires:
//!      * `to_cardinal(12)`  == "十二"    (一 stripped)
//!      * `to_cardinal(-12)` == "負一十二" (一 survives)
//!    Confirmed against the frozen corpus via the float path, which routes
//!    through `to_cardinal(-12)`: `cardinal(-12.34)` == "負一十二點三四".
//!    The asymmetry is preserved exactly. See [`LangZhTw::zh_to_cap`].
//! 2. **The strip is prefix-anchored, not positional**, so it only ever
//!    touches a leading "一十…" — "一十萬" → "十萬" (100000), but "一千零一十"
//!    (1010) keeps its interior "一十", and "一百一十" (110) is untouched
//!    because it starts with "一百".
//! 3. `Num2Word_ZH.merge`'s `stuff_zero == 2` arm re-tests
//!    `len(str(lnum)) - len(str(rnum)) > 1`, which the enclosing `if` has
//!    already established. The re-test is dead but harmless; kept verbatim in
//!    [`LangZhTw::merge`] as a comment rather than as redundant code.
//! 4. `merge` falls through to `return no_zero` when `stuff_zero` is not one
//!    of 1/2/3. Unreachable at the default `stuff_zero == 2`.
//!
//! # Scope notes
//!
//! * `to_ordinal_num` never calls `verify_ordinal`, so negatives pass through
//!   unguarded: `to_ordinal_num(-1)` == "第-1" (corpus-confirmed).
//! * `to_ordinal` *does* call `verify_ordinal` → `TypeError` on negatives.
//! * `to_year`'s ROC-era branch (`民國…年`) is gated on the `era=True`
//!   keyword, served by `to_year_kw` (integers) and `year_float_kw`
//!   (float/Decimal: non-integral values raise the float-year TypeError).
//!
//! # The currency surface
//!
//! `Num2Word_ZH` overrides `to_currency` wholesale and reaches almost none of
//! the `Num2Word_Base` currency machinery — no `pluralize`, no cent words, no
//! `CURRENCY_PRECISION`, no adjectives, no separator. What it does instead is
//! concatenate `[currency name] [負] [cardinal] 元 [角/分 subunits]`. See
//! [`LangZhTw::to_currency`].
//!
//! `CURRENCY_FORMS` is likewise not the `(unit_forms, subunit_forms)` table
//! the rest of the library keeps: sixteen of its entries are bare currency
//! *name* strings and the seventeenth ("XXX") is a nested tuple. `to_cheque`
//! is inherited from Base unchanged and destructures that table with
//! `cr1, _cr2 = ...`, which on a `str` iterates characters — so it returns
//! half a currency name for the 2-character entries and raises `ValueError`
//! for the longer ones. Both behaviours are corpus-frozen. See
//! [`Form`] and [`LangZhTw::to_cheque`].

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, KwVal, Lang, N2WError,
    Result,
};
use crate::currency::{parse_currency_parts, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `negword`, `select_text`-resolved: `(("負",), ("ㄈㄨˋ",))` → "負".
const NEGWORD: &str = "負";
/// `pointword`, resolved: `(("點",), ("ㄉㄧㄢˇ",))` → "點". Float path only.
const POINTWORD: &str = "點";
/// `ord_prefix`, resolved: `(("第",), ("ㄉㄧˋ",))` → "第".
const ORD_PREFIX: &str = "第";
/// `year`, resolved: `(("年",), ("ㄋㄧㄢˊ",))` → "年".
const YEAR: &str = "年";
/// `year_prefix`, resolved: `(("公元", "西元"), (...))` → "公元".
///
/// Two alternatives, but with `prefer=None` the `set(text) & set(prefer)`
/// intersection is empty, so `select_text` falls back to `text[0]`.
const YEAR_PREFIX: &str = "公元";
/// `year_bce`, resolved: `(("前",), ("ㄑㄧㄢˊ",))` → "前".
const YEAR_BCE: &str = "前";

/// `Num2Word_ZH_TW.to_cardinal`'s `stuff_zero` default.
///
/// 2 == "discontinuous high numbers" — insert 零 between discontinuous
/// magnitudes only when the right operand's decimal width is not a multiple
/// of 4 (i.e. it does not sit flush on a 萬-group boundary). The trait has no
/// parameter for it, so the default is baked in.
const STUFF_ZERO: u8 = 2;

/// `low_numwords[-1]` resolved: `(("零", "〇"), ("ㄌㄧㄥˊ",))` → "零".
/// `merge` reaches for this as the infix zero.
const ZERO_WORD: &str = "零";

/// `Num2Word_ZH.CURRENCY_FLOATS` — the generic subunit words, 角 (tenths) and
/// 分 (hundredths). Plain `str`s that `ZH_TW` does not re-declare as tuples, so
/// `select_text` short-circuits on `strtype` and returns them unchanged.
const CURRENCY_FLOATS: [&str; 2] = ["角", "分"];

/// `CURRENCY_FORMS["XXX"]` — after `CURRENCY_FORMS_CHILD` replaces it — as both
/// of its consumers resolve it. They arrive at the same string by different
/// routes, which is why one constant serves both:
///
///   * `to_currency` → `select_text((("元",), ("ㄩㄢˊ",)))`: every item is a
///     tuple, and `reading is not True`, so `text = text[0] == ("元",)`; that
///     is still not a `str`, and `prefer` is empty, so `text = text[0]` → "元".
///   * `to_cheque` → `cr1, _cr2 = (("元",), ("ㄩㄢˊ",))` binds `cr1 = ("元",)`,
///     which `isinstance(cr1, tuple)` routes through `cr1[-1]` → "元".
const XXX_UNIT: &str = "元";

/// One `CURRENCY_FORMS` value.
///
/// Python's table is heterogeneous, and that is load-bearing. Sixteen entries
/// are plain `str` currency *names* ("歐元") rather than the `(unit_forms,
/// subunit_forms)` pair every other language stores, and
/// `Num2Word_ZH_TW.CURRENCY_FORMS_CHILD` replaces "XXX" with the nested tuple
/// `(("元",), ("ㄩㄢˊ",))`.
///
/// The two shapes diverge under `Num2Word_Base.to_cheque`'s `cr1, _cr2 = ...`
/// unpack, so flattening them would change observable behaviour. The proof is
/// the parent class: `Num2Word_ZH` leaves "XXX" as the *one-character* str
/// "元", and `zh.to_cheque(1234.56, currency="XXX")` raises
/// `ValueError: not enough values to unpack (expected 2, got 1)`, whereas
/// ZH_TW's tuple unpacks cleanly and yields "一千二百三十四 AND 56/100 元".
/// Both verified against the live interpreter.
#[derive(Debug, Clone, Copy)]
enum Form {
    /// A `str` entry. `select_text` returns it unchanged.
    Name(&'static str),
    /// The nested `(("元",), ("ㄩㄢˊ",))` tuple — see [`XXX_UNIT`].
    Nested,
}

/// `CURRENCY_FORMS` as `Num2Word_ZH_TW` sees it at runtime.
///
/// Built in two steps on purpose, mirroring Python: `Num2Word_ZH`'s class-body
/// dict first, then the `CURRENCY_FORMS_CHILD` entry that
/// `Num2Word_ZH_TW.__init__` copies over it. Pre-merging the two would hide
/// that the child's sole contribution is a *shape* change to "XXX", which is
/// exactly what makes `to_cheque("XXX")` work here and fail on the parent.
///
/// No `lang_EUR`-style mutation applies: `Num2Word_ZH` declares its own
/// `CURRENCY_FORMS` in its class body, so it never shares the dict that
/// `Num2Word_EN.__init__` rewrites in place — checked with
/// `Num2Word_ZH.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` → False, and
/// the live table carries none of EN's ~24 added codes. Conversely, ZH_TW's
/// own `self.CURRENCY_FORMS = self.CURRENCY_FORMS.copy()` means its write to
/// "XXX" cannot escape back into `Num2Word_ZH`.
fn build_currency_forms() -> HashMap<&'static str, Form> {
    // ---- Num2Word_ZH.CURRENCY_FORMS (class body) ----
    let mut m: HashMap<&'static str, Form> = [
        ("XXX", "元"), // Generic dollar — replaced by the child below.
        ("CNY", "人民幣"),
        ("NTD", "新台幣"),
        ("HKD", "港幣"),
        ("MOP", "澳門幣"),
        ("SGD", "新加坡元"),
        ("MYR", "馬來西亞令吉"),
        ("USD", "美元"),
        ("EUR", "歐元"),
        ("GBP", "英鎊"),
        ("JPY", "日元"),
        ("CHF", "瑞士法郎"),
        ("CAD", "加元"),
        ("AUD", "澳幣"),
        ("NZD", "紐西蘭元"),
        ("THB", "泰銖"),
        ("KRW", "韓元"),
    ]
    .into_iter()
    .map(|(code, name)| (code, Form::Name(name)))
    .collect();

    // ---- Num2Word_ZH_TW.__init__: CURRENCY_FORMS_CHILD copied on top ----
    // A single key, and only its shape changes: "元" → (("元",), ("ㄩㄢˊ",)).
    m.insert("XXX", Form::Nested);

    m
}

pub struct LangZhTw {
    cards: Cards,
    maxval: BigInt,
    /// `select_text(self.cards[1])` — cached for `zh_to_cap`.
    one: String,
    /// `select_text(self.cards[10])` — cached for `zh_to_cap`.
    ten: String,
    /// `CURRENCY_FORMS`. Built once here, never per call.
    currency_forms: HashMap<&'static str, Form>,
}

impl Default for LangZhTw {
    fn default() -> Self {
        Self::new()
    }
}

impl LangZhTw {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // Num2Word_ZH_TW.setup: high_numwords, listed 10**4 → 10**72 and then
        // `.reverse()`d. Num2Word_ZH.set_high_numwords zips the reversed list
        // against range(4 * len(high), 0, -4) == range(72, 0, -4), i.e.
        // high[0] ("不可說") ↦ 10**72 down to high[17] ("萬") ↦ 10**4.
        // Both sequences are 18 long, so zip() truncates nothing.
        const HIGH: [&str; 18] = [
            "不可說",     // 10**72
            "無量",       // 10**68
            "不可思議",   // 10**64
            "那由他",     // 10**60
            "阿僧祇",     // 10**56
            "恆河沙",     // 10**52
            "極",         // 10**48
            "載",         // 10**44
            "正",         // 10**40
            "澗",         // 10**36
            "溝",         // 10**32
            "穣",         // 10**28
            "秭",         // 10**24
            "垓",         // 10**20
            "京",         // 10**16
            "兆",         // 10**12
            "億",         // 10**8
            "萬",         // 10**4
        ];
        let max_exp: u32 = 4 * HIGH.len() as u32; // 72
        for (i, word) in HIGH.iter().enumerate() {
            let exp = max_exp - 4 * i as u32;
            cards.insert(BigInt::from(10u8).pow(exp), *word);
        }

        // mid_numwords. Note ZH_TW drops the parent's (10, "十") entry —
        // 10 arrives via low_numwords instead (see below). Same card either
        // way; only the OrderedDict insertion order differs, and that order
        // stays descending in both classes, which is all splitnum relies on.
        set_mid_numwords(&mut cards, &[(1000, "千"), (100, "百")]);

        // low_numwords: 11 entries → set_low_numwords assigns 10 down to 0.
        set_low_numwords(
            &mut cards,
            &[
                "十", // 10
                "九", // 9
                "八", // 8
                "七", // 7
                "六", // 6
                "五", // 5
                "四", // 4
                "三", // 3
                "二", // 2
                "一", // 1
                "零", // 0
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0] == 1000 * 10**72 == 10**75
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let one = cards.get(&BigInt::one()).unwrap_or("").to_string();
        let ten = cards.get(&BigInt::from(10)).unwrap_or("").to_string();

        LangZhTw {
            cards,
            maxval,
            one,
            ten,
            currency_forms: build_currency_forms(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`, negative arm only.
    ///
    /// The float arm (`errmsg_floatord`) is unreachable: input is an integer.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `Num2Word_ZH.zh_to_cap`, non-capital branch.
    ///
    /// ```python
    /// one, ten = self.select_text(self.cards[1]), self.select_text(self.cards[10])
    /// if capital: ...            # unreachable here: reading != "capital"
    /// elif out.startswith(one + ten):
    ///     out = out[len(one):]
    /// return out
    /// ```
    ///
    /// `capital` is `reading == "capital"`, and `reading` is fixed at `False`
    /// for every in-scope call, so only the `elif` can fire.
    ///
    /// Python's `out[len(one):]` slices by *character*. Here the strip only
    /// runs once `one` is known to be a prefix of `value`, so `one.len()`
    /// (bytes) is exactly the byte offset of that character boundary — no
    /// char-index arithmetic needed, and no panic risk.
    fn zh_to_cap(&self, value: &str) -> String {
        let prefix = format!("{}{}", self.one, self.ten);
        if value.starts_with(&prefix) {
            return value[self.one.len()..].to_string();
        }
        value.to_string()
    }

    /// `select_text(self.cards[d])` for a single digit.
    ///
    /// `low_numwords` populates 0..=10, so 0..=9 always hit and Python's
    /// `KeyError` on `self.cards[...]` is unreachable — same reasoning, and
    /// same `unwrap_or("")`, as [`LangZhTw::to_year`]'s digit loop.
    fn card_digit(&self, d: u32) -> String {
        self.cards.get(&BigInt::from(d)).unwrap_or("").to_string()
    }

    /// `Num2Word_ZH.to_currency_float`.
    ///
    /// ```python
    /// cents = "%02d" % value
    /// out = []
    /// if int(cents) > 0:
    ///     if not (int(cents[0]) == 0 and reading == "capital"):
    ///         out += [self.cards[int(cents[0])]]
    ///     if int(cents[0]) > 0:
    ///         out += [self.CURRENCY_FLOATS[0]]
    ///     if int(cents[1]) > 0:
    ///         out += [self.cards[int(cents[1])], self.CURRENCY_FLOATS[1]]
    /// return out
    /// ```
    ///
    /// 角 (tenths) is spoken only when the tens digit is non-zero, 分
    /// (hundredths) only when the units digit is; but the tens digit itself is
    /// always emitted, so a lone 零 survives as a placeholder. That is what
    /// makes `0.01` read "零元零一分" while `0.5` stops at "零元五角".
    ///
    /// `value` is the subunit count `parse_currency_parts` yields, hence
    /// non-negative — so `int("%02d" % value) == value` and the `int(cents) > 0`
    /// guard is a plain positivity test.
    fn to_currency_float(&self, value: &BigInt) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        if !value.is_positive() {
            return out;
        }

        // "%02d". Formatting `to_string()` rather than the BigInt directly
        // keeps the zero-fill independent of BigInt's Display width handling.
        // A hypothetical value >= 100 widens the string exactly as Python's
        // %02d does, and cents[0]/cents[1] still read the first two chars.
        let cents = format!("{:0>2}", value.to_string());
        let digits: Vec<u32> = cents
            .chars()
            .map(|c| c.to_digit(10).expect("%02d of a non-negative int is digits"))
            .collect();
        let (d0, d1) = (digits[0], digits[1]);

        // Python guards this with `not (int(cents[0]) == 0 and reading ==
        // "capital")`. `reading` is fixed at False for every in-scope call, so
        // the guard is always True and the 零 placeholder is unconditional.
        out.push(self.card_digit(d0));
        if d0 > 0 {
            out.push(CURRENCY_FLOATS[0].to_string());
        }
        if d1 > 0 {
            out.push(self.card_digit(d1));
            out.push(CURRENCY_FLOATS[1].to_string());
        }
        out
    }

    /// `Num2Word_ZH_TW.to_cardinal(value, stuff_zero, reading, prefer)` with
    /// the kwargs live. Same body as the plain [`Lang::to_cardinal`], but the
    /// card table, negword, merge-zero and `zh_to_cap` inputs are all
    /// `select_text`-resolved under (`reading`, `prefer`), and `stuff_zero`
    /// rides through `merge`. Order preserved: `zh_to_cap` runs BEFORE the
    /// space strip, and the capital arm performs **no** leading-一 strip
    /// (11 → "壹拾壹"; reading=True strips ㄧ off "ㄧㄕˊㄧ" → "ㄕˊㄧ").
    fn kw_cardinal(
        &self,
        value: &BigInt,
        stuff_zero: i64,
        reading: Reading,
        prefer: Option<&[String]>,
    ) -> Result<String> {
        let eng = TwKwEngine {
            base: self,
            cards: resolved_cards(reading, prefer),
            negword: select2(NEG_ALTS, reading, prefer),
            stuff_zero,
            zero_word: select2(TW_LOW[0], reading, prefer),
        };
        let out = default_to_cardinal(&eng, value)?;
        let out = match reading {
            Reading::Capital => apply_cap_map(&out, &TW_CAP_MAP),
            _ => {
                let one = select2(TW_LOW[1], reading, prefer);
                let ten = select2(TW_LOW[10], reading, prefer);
                let prefix = format!("{}{}", one, ten);
                if out.starts_with(&prefix) {
                    out[one.len()..].to_string()
                } else {
                    out
                }
            }
        };
        Ok(out.replace(' ', ""))
    }

    /// `Num2Word_ZH_TW.to_cardinal_float(value, reading, prefer)` with the
    /// kwargs live — unlike ZH/ZH_HK, this override *forwards* `reading`/
    /// `prefer` into every inner `to_cardinal` call, so the kwargs survive
    /// the float path. `stuff_zero` does not: the inner calls leave it at
    /// its default 2. Same float-cast/precision semantics as the plain
    /// [`Lang::to_cardinal_float`] — see its docs.
    fn kw_cardinal_float(
        &self,
        value: &FloatValue,
        reading: Reading,
        prefer: Option<&[String]>,
    ) -> Result<String> {
        let f: f64 = match value {
            FloatValue::Float { value, .. } => *value,
            FloatValue::Decimal { value, .. } => value
                .to_f64()
                .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?,
        };
        let precision = float_repr_precision(f);
        let (pre, post) = float2tuple(&FloatValue::Float {
            value: f,
            precision,
        });

        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        let mut out: Vec<String> = vec![self.kw_cardinal(&pre, 2, reading, prefer)?];

        let is_negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if is_negative && pre.is_zero() {
            // select_text(self.negword).strip() — no whitespace to strip in
            // either script, kept for shape.
            out.insert(0, select2(NEG_ALTS, reading, prefer).trim().to_string());
        }

        if precision > 0 {
            out.push(select2(POINT_ALTS, reading, prefer).to_string());
        }

        for ch in post_str.chars().take(precision as usize) {
            let d = ch
                .to_digit(10)
                .ok_or_else(|| N2WError::Value(format!("non-digit {:?} in fractional part", ch)))?;
            out.push(self.kw_cardinal(&BigInt::from(d), 2, reading, prefer)?);
        }

        // out = zh_to_cap(" ".join(out), self.capital); return out.replace(" ","")
        let joined = out.join(" ");
        let capped = match reading {
            Reading::Capital => apply_cap_map(&joined, &TW_CAP_MAP),
            _ => {
                let one = select2(TW_LOW[1], reading, prefer);
                let ten = select2(TW_LOW[10], reading, prefer);
                let prefix = format!("{}{}", one, ten);
                if joined.starts_with(&prefix) {
                    joined[one.len()..].to_string()
                } else {
                    joined
                }
            }
        };
        Ok(capped.replace(' ', ""))
    }

    /// `Num2Word_ZH.zh_to_cap(value, capital)` with the (`reading`, `prefer`)
    /// selection live — the per-component form `to_currency`'s loop applies.
    /// Capital arm: sequential `CAP_map` replacement. Otherwise: strip the
    /// leading `one` of a `one + ten` prefix, both resolved under the call's
    /// reading (`"一十"` for hanzi, `"ㄧㄕˊ"` for bopomofo).
    fn kw_zh_to_cap(&self, value: &str, reading: Reading, prefer: Option<&[String]>) -> String {
        match reading {
            Reading::Capital => apply_cap_map(value, &TW_CAP_MAP),
            _ => {
                let one = select2(TW_LOW[1], reading, prefer);
                let ten = select2(TW_LOW[10], reading, prefer);
                let prefix = format!("{}{}", one, ten);
                if value.starts_with(&prefix) {
                    value[one.len()..].to_string()
                } else {
                    value.to_string()
                }
            }
        }
    }

    /// `Num2Word_ZH.to_year(value, reading, prefer)` with the kwargs live —
    /// the `era=False` path of `Num2Word_ZH_TW.to_year`. Digit-by-digit
    /// through `select_text(self.cards[int(s)])`, so 2025 is 二零二五年 (or
    /// ㄦˋㄌㄧㄥˊㄦˋㄨˇㄋㄧㄢˊ under `reading=True`); negative years get
    /// 公元前, and — unreachable at the plain default but live here —
    /// `reading == "capital"` prefixes 公元 on *positive* years too.
    fn kw_year(&self, value: &BigInt, reading: Reading, prefer: Option<&[String]>) -> Result<String> {
        let mut out = String::new();
        if value.is_negative() {
            out.push_str(select2(YEAR_PREFIX_ALTS, reading, prefer));
            out.push_str(select2(YEAR_BCE_ALTS, reading, prefer));
        } else if reading == Reading::Capital {
            out.push_str(select2(YEAR_PREFIX_ALTS, reading, prefer));
        }
        for ch in value.abs().to_string().chars() {
            let d = ch.to_digit(10).expect("BigInt::to_string emits digits") as usize;
            out.push_str(select2(TW_LOW[d], reading, prefer));
        }
        out.push_str(select2(YEAR_W_ALTS, reading, prefer));
        Ok(out)
    }

    /// `Num2Word_ZH.to_currency_float(value, reading, prefer)` with the
    /// kwargs live. Same shape as [`LangZhTw::to_currency_float`], but the
    /// digit words come from the (reading/prefer)-selected card table and the
    /// capital guard `not (int(cents[0]) == 0 and reading == "capital")` is
    /// real: capital mode drops the lone 零 placeholder before 分.
    fn kw_currency_float(
        &self,
        value: &BigInt,
        reading: Reading,
        prefer: Option<&[String]>,
    ) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        if !value.is_positive() {
            return out;
        }
        let cents = format!("{:0>2}", value.to_string());
        let digits: Vec<u32> = cents
            .chars()
            .map(|c| c.to_digit(10).expect("%02d of a non-negative int is digits"))
            .collect();
        let (d0, d1) = (digits[0], digits[1]);

        if !(d0 == 0 && reading == Reading::Capital) {
            out.push(select2(TW_LOW[d0 as usize], reading, prefer).to_string());
        }
        if d0 > 0 {
            out.push(CURRENCY_FLOATS[0].to_string());
        }
        if d1 > 0 {
            out.push(select2(TW_LOW[d1 as usize], reading, prefer).to_string());
            out.push(CURRENCY_FLOATS[1].to_string());
        }
        out
    }
}

/// `len(str(n))` for a non-negative `n`: its decimal digit count.
///
/// `merge` compares magnitudes by decimal width, so this has to match
/// Python's `len(str(...))` exactly. `BigInt::to_string` emits bare ASCII
/// digits for non-negative values (no separators, no sign), so byte length is
/// the digit count. Every `lnum`/`rnum` reaching `merge` is a card value or a
/// sum/product of card values, hence non-negative — `to_cardinal` strips the
/// sign before `splitnum` ever runs.
fn dec_len(n: &BigInt) -> usize {
    n.to_string().len()
}

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the precision
/// `base.float2tuple`'s float branch derives on every call.
///
/// `Num2Word_ZH_TW.to_cardinal_float` recomputes precision from
/// `float(value)` even for Decimal input, so this is the one true precision
/// for both arms — not the value's stored precision.
///
/// Counting `format!("{}", f)`'s fractional digits (what
/// `floatpath::float_repr_precision` does) is NOT enough here: Python's
/// `repr` switches to scientific notation outside `1e-4 ..= <1e16`, and
/// `Decimal('1e+20').as_tuple().exponent` is **+20**, so
/// `float(Decimal("99999999999999999999.99")) == 1e20` carries precision 20
/// — "一垓點" plus twenty 零 — where Rust's always-positional `{}` would give
/// 0. Decimal input is the reachable route: non-integral floats are all
/// `< 2**53 < 1e16`, but a Decimal's float cast can land anywhere.
///
/// Both `{:e}` and Python's `repr` emit the (unique) shortest round-trip
/// digits, so the mantissa/exponent split here reproduces repr exactly:
///
///   * scientific form (`exp10 < -4 || exp10 >= 16`): Decimal exponent is
///     `exp10 - (nd - 1)` — e.g. `'1.5e+20'` → 19, `'1e-07'` → 7;
///   * positional integral (`nd - 1 <= exp10`): repr appends `'.0'` → 1
///     (e.g. `repr(5e15) == '5000000000000000.0'`);
///   * positional fractional: `nd - 1 - exp10` digits after the point.
fn float_repr_precision(f: f64) -> u32 {
    if f == 0.0 {
        return 1; // repr(0.0) == '0.0' (and '-0.0') → exponent -1.
    }
    let sci = format!("{:e}", f.abs()); // "d[.ddd]e<exp>", shortest digits
    let (mant, exp) = sci.split_once('e').expect("{:e} always carries an exponent");
    let exp10: i64 = exp.parse().expect("{:e} exponent is an integer");
    let nd = (mant.len() - usize::from(mant.contains('.'))) as i64;
    if exp10 < -4 || exp10 >= 16 {
        (exp10 - (nd - 1)).unsigned_abs() as u32
    } else if nd - 1 <= exp10 {
        1
    } else {
        (nd - 1 - exp10) as u32
    }
}

/// `Num2Word_ZH.merge`, verbatim, with the two per-call inputs live:
/// `self.stuff_zero` (the `to_cardinal` kwarg) and
/// `select_text(self.low_numwords[-1])` (the reading/prefer-selected zero).
fn zh_merge(
    l: (&str, &BigInt),
    r: (&str, &BigInt),
    stuff_zero: i64,
    zero_word: &str,
) -> (String, BigInt) {
    let (ltext, lnum) = l;
    let (rtext, rnum) = r;

    // Ignore lpair when lnum is 1 and rnum is a single digit: 一 + 二 → 二.
    if lnum.is_one() && rnum < &BigInt::from(10) {
        return (rtext.to_string(), rnum.clone());
    }

    let sum = lnum + rnum;
    let with_zero = (format!("{}{}{}", ltext, zero_word, rtext), sum.clone());
    let no_zero = (format!("{}{}", ltext, rtext), sum);

    let (llen, rlen) = (dec_len(lnum), dec_len(rnum));

    if llen as isize - rlen as isize > 1 {
        // Discontinuous magnitudes — decide whether 零 is spoken.
        // http://www.hkame.org.hk/uploaded_files/magazine/15/271.pdf
        match stuff_zero {
            // 凡「零」必讀 — every discontinuity gets a 零.
            1 => with_zero,
            // Discontinuous *high* numbers only. Python re-tests
            // `llen - rlen > 1` here; the enclosing `if` already
            // guarantees it, so only the `% 4` test can decide.
            2 => {
                if rlen % 4 != 0 {
                    with_zero
                } else {
                    no_zero
                }
            }
            // 凡「零」不讀 — never.
            3 => no_zero,
            // Python's if/elif chain has no else: any other stuff_zero
            // (None, 4, "2", ...) falls through to the trailing
            // `return no_zero` — `None == 1` etc. are all simply False.
            _ => no_zero,
        }
    } else if rnum > lnum {
        // Multiplicative: 二 × 十 → 二十, 一百 × 萬 → 一百萬.
        (format!("{}{}", ltext, rtext), lnum * rnum)
    } else {
        no_zero
    }
}

// ---- grammatical kwargs (reading / prefer / stuff_zero / counter / era) ----

/// One tuple-table entry as `lang_ZH_TW.py` declares it:
/// `((hanzi, alt...), (bopomofo...))`. `select_text` picks side 1 when
/// `reading is True`, side 0 otherwise, then intersects with `prefer`.
type Alts = (&'static [&'static str], &'static [&'static str]);

/// `low_numwords` indexed by value 0..=10.
const TW_LOW: [Alts; 11] = [
    (&["零", "〇"], &["ㄌㄧㄥˊ"]), // 0
    (&["一", "壹"], &["ㄧ"]),      // 1
    (&["二", "貳"], &["ㄦˋ"]),     // 2
    (&["三", "參"], &["ㄙㄢ"]),    // 3
    (&["四", "肆"], &["ㄙˋ"]),     // 4
    (&["五", "伍"], &["ㄨˇ"]),     // 5
    (&["六", "陸"], &["ㄌㄧㄡˋ"]), // 6
    (&["七", "柒"], &["ㄑㄧ"]),    // 7
    (&["八", "捌"], &["ㄅㄚ"]),    // 8
    (&["九", "玖"], &["ㄐㄧㄡˇ"]), // 9
    (&["十", "拾"], &["ㄕˊ"]),     // 10
];

const TW_MID: [(i64, Alts); 2] = [
    (1000, (&["千"], &["ㄑㄧㄢ"])),
    (100, (&["百"], &["ㄅㄞˇ"])),
];

/// `high_numwords`, keyed by the power of ten `set_high_numwords` assigns.
const TW_HIGH: [(u32, Alts); 18] = [
    (72, (&["不可說"], &["ㄅㄨˋㄎㄜˇㄕㄨㄛ"])),
    (68, (&["無量"], &["ㄨˊㄌㄧㄤˋ"])),
    (64, (&["不可思議"], &["ㄅㄨˋㄎㄜˇㄙㄧˋ"])),
    (60, (&["那由他"], &["ㄋㄚˋㄧㄡˊㄊㄚ"])),
    (56, (&["阿僧祇"], &["ㄚㄙㄥㄑㄧˊ"])),
    (52, (&["恆河沙"], &["ㄏㄥˊㄏㄜˊㄕㄚ"])),
    (48, (&["極"], &["ㄐㄧˊ"])),
    (44, (&["載"], &["ㄗㄞˇ"])),
    (40, (&["正"], &["ㄓㄥˋ"])),
    (36, (&["澗"], &["ㄐㄧㄢˋ"])),
    (32, (&["溝"], &["ㄍㄡ"])),
    (28, (&["穣"], &["ㄖㄤ"])),
    (24, (&["秭"], &["ㄗˇ"])),
    (20, (&["垓"], &["ㄍㄞ"])),
    (16, (&["京"], &["ㄐㄧㄥ"])),
    (12, (&["兆"], &["ㄓㄠˋ"])),
    (8, (&["億"], &["ㄧˋ"])),
    (4, (&["萬"], &["ㄨㄢˋ"])),
];

const NEG_ALTS: Alts = (&["負"], &["ㄈㄨˋ"]);
const POINT_ALTS: Alts = (&["點"], &["ㄉㄧㄢˇ"]);
const ORD_ALTS: Alts = (&["第"], &["ㄉㄧˋ"]);
const YEAR_W_ALTS: Alts = (&["年"], &["ㄋㄧㄢˊ"]);
/// Note the bopomofo 公元 carries an interior space that survives to the
/// output — `to_year` joins without stripping ("ㄍㄨㄥ ㄩㄢˊㄑㄧㄢˊ…").
const YEAR_PREFIX_ALTS: Alts = (&["公元", "西元"], &["ㄍㄨㄥ ㄩㄢˊ", "ㄒㄧㄩㄢˊ"]);
const YEAR_BCE_ALTS: Alts = (&["前"], &["ㄑㄧㄢˊ"]);
const ROC_ERA_ALTS: Alts = (&["民國"], &["ㄇㄧㄣˊㄍㄨㄛˊ"]);
/// The ROC first-year word `(("元",), ("ㄩㄢˊ",))` — also the shape of
/// `CURRENCY_FORMS["XXX"]`, which `to_currency`'s loop selects the same way.
const YUAN_ALTS: Alts = (&["元"], &["ㄩㄢˊ"]);
const CHEQUE_ALTS: Alts = (&["正"], &["ㄓㄥˋ"]);

/// `Num2Word_ZH.CAP_map`, inherited unchanged by ZH_TW (traditional
/// capitals, including the `("正", "整")` row ZH_CN drops).
const TW_CAP_MAP: [(&str, &str); 14] = [
    ("千", "仟"),
    ("百", "佰"),
    ("十", "拾"),
    ("九", "玖"),
    ("八", "捌"),
    ("七", "柒"),
    ("六", "陸"),
    ("五", "伍"),
    ("四", "肆"),
    ("三", "叁"),
    ("二", "貳"),
    ("一", "壹"),
    ("元", "圓"),
    ("正", "整"),
];

/// `Num2Word_ZH_TW.counters` — the 注音 readings `to_ordinal`/`to_ordinal_num`
/// swap in when `reading is True`.
fn tw_counter(counter: &str) -> Option<&'static str> {
    match counter {
        "個" => Some("˙ㄍㄜ"),
        "名" => Some("ㄇㄧㄥˊ"),
        "位" => Some("ㄨㄟˋ"),
        _ => None,
    }
}

/// The three states `reading` can put `select_text`/`zh_to_cap` in: Python
/// tests `reading is True` (only the bool `True`) and `reading == "capital"`;
/// everything else behaves like the default `False`.
#[derive(Clone, Copy, PartialEq)]
enum Reading {
    Plain,
    True,
    Capital,
}

fn parse_reading(kw: &Kwargs) -> Reading {
    match kw.get("reading") {
        Some(KwVal::Bool(true)) => Reading::True,
        Some(KwVal::Str(s)) if s == "capital" => Reading::Capital,
        _ => Reading::Plain,
    }
}

/// `prefer` as the item set `select_text` intersects with. A list keeps its
/// members, a str iterates characters; a non-iterable raises TypeError inside
/// Python's `set()` — NotImplemented lets the dispatcher fall back to the
/// original, which owns that raise.
fn parse_prefer(kw: &Kwargs) -> Result<Option<Vec<String>>> {
    match kw.get("prefer") {
        None | Some(KwVal::None) => Ok(None),
        Some(KwVal::List(l)) => Ok(Some(l.clone())),
        Some(KwVal::Str(s)) => Ok(Some(s.chars().map(|c| c.to_string()).collect())),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

/// `stuff_zero`, defaulting to 2. Bools are Python ints; any other type
/// compares unequal to 1/2/3 and takes `zh_merge`'s fall-through arm (0).
fn parse_stuff_zero(kw: &Kwargs) -> i64 {
    match kw.get("stuff_zero") {
        None => 2,
        Some(KwVal::Int(i)) => *i,
        Some(KwVal::Bool(b)) => *b as i64,
        Some(_) => 0,
    }
}

/// `counter`: only a `str` short-circuits `select_text`; anything else hits
/// `len(text)` there and raises TypeError — delegated back to Python.
fn parse_counter(kw: &Kwargs) -> Result<&str> {
    match kw.get("counter") {
        None => Ok(""),
        Some(KwVal::Str(s)) => Ok(s),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

/// Python truthiness for the `era` kwarg (`if not era:`).
fn kw_truthy(v: Option<&KwVal>) -> bool {
    match v {
        None | Some(KwVal::None) => false,
        Some(KwVal::Bool(b)) => *b,
        Some(KwVal::Int(i)) => *i != 0,
        Some(KwVal::Str(s)) => !s.is_empty(),
        Some(KwVal::List(l)) => !l.is_empty(),
    }
}

/// The prefer half of `select_text`: the single member of
/// `set(alts) & set(prefer)` when exactly one matches, else the first.
fn select_alt<'a>(alts: &[&'a str], prefer: Option<&[String]>) -> &'a str {
    if let Some(p) = prefer {
        let hits: Vec<&str> = alts
            .iter()
            .copied()
            .filter(|a| p.iter().any(|x| x == a))
            .collect();
        if hits.len() == 1 {
            return hits[0];
        }
    }
    alts[0]
}

/// Full `select_text` over a ZH_TW nested-tuple entry: side 1 (注音) when
/// `reading is True`, side 0 (hanzi + capital alternates) otherwise —
/// `"capital"` is *not* `True`, so it reads side 0 like the default — then
/// the prefer intersection.
fn select2(alts: Alts, reading: Reading, prefer: Option<&[String]>) -> &'static str {
    let side = if reading == Reading::True { alts.1 } else { alts.0 };
    select_alt(side, prefer)
}

/// `zh_to_cap`'s capital arm: sequential `str.replace` over `CAP_map`.
fn apply_cap_map(value: &str, map: &[(&str, &str)]) -> String {
    let mut out = value.to_string();
    for (plain, cap) in map {
        if out.contains(plain) {
            out = out.replace(plain, cap);
        }
    }
    out
}

/// The card table as `select_text` resolves it under a live
/// (`reading`, `prefer`) pair — what `splitnum` reads on the Python side.
fn resolved_cards(reading: Reading, prefer: Option<&[String]>) -> Cards {
    let mut cards = Cards::new();
    for (exp, alts) in TW_HIGH.iter() {
        cards.insert(BigInt::from(10u8).pow(*exp), select2(*alts, reading, prefer));
    }
    for (v, alts) in TW_MID.iter() {
        cards.insert(BigInt::from(*v), select2(*alts, reading, prefer));
    }
    for (d, alts) in TW_LOW.iter().enumerate() {
        cards.insert(BigInt::from(d), select2(*alts, reading, prefer));
    }
    cards
}

/// A per-call view of the engine with the `to_cardinal` kwargs live.
struct TwKwEngine<'a> {
    base: &'a LangZhTw,
    cards: Cards,
    negword: &'static str,
    stuff_zero: i64,
    zero_word: &'static str,
}

impl Lang for TwKwEngine<'_> {
    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        self.base.maxval()
    }
    fn negword(&self) -> &str {
        self.negword
    }
    fn pointword(&self) -> &str {
        self.base.pointword()
    }
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        zh_merge(l, r, self.stuff_zero, self.zero_word)
    }
}

/// Python's `str(<float>)` for a finite value — fills the `%s` of the
/// TypeError format strings (`errmsg_floatord`/`errmsg_negord`/
/// `errmsg_floatyear`). The corpora record only the type; the message
/// mirrors CPython's spelling.
fn py_float_str(value: f64, precision: u32) -> String {
    let a = value.abs();
    let mag = if a != 0.0 && (a >= 1e16 || a < 1e-4) {
        let s = format!("{:e}", a);
        let (mant, exp) = s.split_once('e').expect("LowerExp always emits an e");
        let exp: i32 = exp.parse().expect("exponent is a small int");
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else {
        format!("{:.*}", precision as usize, a)
    };
    if value.is_sign_negative() {
        format!("-{}", mag)
    } else {
        mag
    }
}

/// `str(value)` for either FloatValue arm (Decimal's `%s` is its `str`).
fn py_value_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, precision } => py_float_str(*value, *precision),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

impl Lang for LangZhTw {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "XXX"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ""
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    fn negword(&self) -> &str {
        // base's default_to_cardinal does `format!("{} ", negword().trim())`,
        // matching Python's `"%s " % select_text(self.negword).strip()`.
        NEGWORD
    }

    fn pointword(&self) -> &str {
        POINTWORD
    }

    // is_title stays False (Num2Word_Base.__init__ sets it; neither ZH nor
    // ZH_TW touches it), so `title` is the identity and `exclude_title`
    // — `[negword, pointword]` in Python — is never consulted.

    /// `Num2Word_ZH.merge`.
    ///
    /// ```python
    /// ltext, rtext = self.select_text(ltext), self.select_text(rtext)
    /// if lnum == 1 and rnum < 10:
    ///     return (rtext, rnum)
    /// with_zero = ("%s%s%s" % (ltext, select_text(low_numwords[-1]), rtext), lnum + rnum)
    /// no_zero   = ("%s%s"   % (ltext, rtext),                              lnum + rnum)
    /// if len(str(lnum)) - len(str(rnum)) > 1:
    ///     if self.stuff_zero == 1: return with_zero
    ///     elif self.stuff_zero == 2:
    ///         if len(str(lnum)) - len(str(rnum)) > 1 and len(str(rnum)) % 4 != 0:
    ///             return with_zero
    ///         return no_zero
    ///     elif self.stuff_zero == 3: return no_zero
    /// elif rnum > lnum:
    ///     return ("%s%s" % (ltext, rtext), lnum * rnum)
    /// return no_zero
    /// ```
    ///
    /// The `select_text` calls are no-ops here: the cards are pre-resolved
    /// strings, and Python's `select_text` short-circuits on `strtype` — which
    /// is also what happens on the Python side from the second merge onward,
    /// since `merge` returns a plain `str`.
    ///
    /// The body lives in [`zh_merge`], shared with the kwargs engine, which
    /// carries the live `stuff_zero` and the (reading/prefer)-selected zero.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        zh_merge(l, r, STUFF_ZERO as i64, ZERO_WORD)
    }

    /// `Num2Word_ZH_TW.to_cardinal`.
    ///
    /// The `assert int(value) == value` guard and its `to_cardinal_float`
    /// fallback are elided: input is always integral here.
    ///
    /// `default_to_cardinal` covers the shared body exactly — the "負 " prefix
    /// (`negword` + a trailing space), the `value >= MAXVAL` OverflowError,
    /// `splitnum` → `clean` → `merge`, and the no-op `title`.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let out = default_to_cardinal(self, value)?;
        // zh_to_cap runs BEFORE the space strip — the reason the leading 一 of
        // e.g. "負 一十二" survives. See the module docs.
        let out = self.zh_to_cap(&out);
        Ok(out.replace(' ', ""))
    }

    /// `Num2Word_ZH_TW.to_ordinal` → `Num2Word_ZH.to_ordinal`.
    ///
    /// The `counter` keyword defaults to `""`; `select_text("")` returns `""`
    /// (it short-circuits on `strtype`), so the suffix is empty and the
    /// `counters` lookup — plus its `NotImplementedError` — is unreachable
    /// without `reading=True`.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let base = self.to_cardinal(value)?;
        Ok(format!("{}{}", ORD_PREFIX, base))
    }

    /// `Num2Word_ZH_TW.to_ordinal_num` → `Num2Word_ZH.to_ordinal_num`.
    ///
    /// `"%s%s%s" % (select_text(ord_prefix), value, select_text(counter))`.
    /// No `verify_ordinal`, so negatives render literally: -1 → "第-1".
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", ORD_PREFIX, value))
    }

    /// `Num2Word_ZH_TW.to_year` with `era=False` → `Num2Word_ZH.to_year`.
    ///
    /// ```python
    /// out = []
    /// if value < 0:                     out += [self.year_prefix, self.year_bce]
    /// elif reading == "capital":        out += [self.year_prefix]
    /// out += [self.cards[int(s)] for s in str(abs(int(value)))]
    /// out += [self.year]
    /// return "".join(self.select_text(s) for s in out)
    /// ```
    ///
    /// Digit-by-digit, not cardinal: 2025 → 二零二五年, and 1010 → 一零一零年
    /// (`cards[0]` is 零, so no 〇 and no merge-driven 零-elision).
    /// Negative → 公元前…年. The `reading == "capital"` arm cannot fire.
    ///
    /// `errmsg_floatyear`'s TypeError is unreachable (integral input), and
    /// `cards[0..=9]` are all populated by `low_numwords`, so the digit lookup
    /// cannot KeyError.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        if value.is_negative() {
            out.push_str(YEAR_PREFIX);
            out.push_str(YEAR_BCE);
        }
        // str(abs(int(value))) — for 0 this is "0", giving "零年".
        for ch in value.abs().to_string().chars() {
            let digit = BigInt::from(ch.to_digit(10).expect("BigInt::to_string emits digits"));
            out.push_str(self.cards.get(&digit).unwrap_or(""));
        }
        out.push_str(YEAR);
        Ok(out)
    }

    /// `Num2Word_ZH_TW.to_cardinal_float`.
    ///
    /// ```python
    /// def to_cardinal_float(self, value, reading=False, prefer=None):
    ///     ...                                   # errmsg_nonnum guard, unreachable here
    ///     pre, post = self.float2tuple(float(value))
    ///     post = str(post)
    ///     post = "0" * (self.precision - len(post)) + post
    ///     out = [self.to_cardinal(pre, reading=reading, prefer=prefer)]
    ///     if value < 0 and pre == 0:
    ///         out = [self.select_text(self.negword).strip()] + out
    ///     if self.precision:
    ///         out.append(self.select_text(self.title(self.pointword)))
    ///     for i in range(self.precision):
    ///         out.append(to_s(self.to_cardinal(int(post[i]), reading, prefer)))
    ///     out = self.zh_to_cap(" ".join(out), self.capital)
    ///     return out.replace(" ", "")
    /// ```
    ///
    /// Two departures from `Num2Word_Base.to_cardinal_float`, both load-bearing:
    ///
    /// 1. **It always casts `float(value)`** — even a `Decimal`. Base preserves
    ///    Decimal precision via `isinstance(value, Decimal)`; ZH_TW does not, so
    ///    both `FloatValue` arms funnel through the *float* branch of
    ///    `float2tuple`, and precision is recomputed from the f64's repr rather
    ///    than kept from the Decimal. Hence `Decimal("1.10")` → "一點一" (the
    ///    trailing zero is dropped, precision 2 → 1) and
    ///    `Decimal("98746251323029.99")` → "…三千零二十九點九八": the float cast
    ///    rounds `.99` → `.98` at trillion scale (issue #603's trap), reproduced
    ///    not repaired. Both corpus-confirmed.
    /// 2. **It re-runs `zh_to_cap` on the space-joined string and then strips
    ///    every space** (Base joins with " " and stops). The inner
    ///    `self.to_cardinal(pre)` has already stripped any leading 一 of "一十…",
    ///    and the negative prefix is "負", so the outer `zh_to_cap` never fires
    ///    in practice — but it is reproduced verbatim.
    ///
    /// `reading`/`prefer`/`capital` are fixed at their defaults (`False`,
    /// `None`, `False`), so `select_text` collapses to identity and `zh_to_cap`
    /// takes its non-capital branch, exactly as in the integer path.
    ///
    /// `precision_override` (the `precision=` kwarg) is **ignored**: ZH_TW's
    /// override drops the parameter Base accepts, so `num2words(x,
    /// lang="zh_TW", precision=N)` has no effect — verified live (`precision=4`
    /// still renders all five digits of 1.23456).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // pre, post = self.float2tuple(float(value)) — float(value) collapses
        // both arms to an f64 before float2tuple ever branches.
        let f: f64 = match value {
            FloatValue::Float { value, .. } => *value,
            FloatValue::Decimal { value, .. } => value
                .to_f64()
                .ok_or_else(|| N2WError::Value(format!("cannot represent {} as f64", value)))?,
        };

        // float2tuple's float branch sets self.precision from the f64's repr;
        // reuse it (rather than the FloatValue's stored precision, which for a
        // Decimal is its own scale and would keep the trailing zero).
        let precision = float_repr_precision(f);
        let (pre, post) = float2tuple(&FloatValue::Float {
            value: f,
            precision,
        });

        // post = str(post); post = "0"*(precision - len(post)) + post
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        // out = [self.to_cardinal(pre)] — the full ZH_TW cardinal, itself
        // zh_to_cap'd and space-stripped (so "十二", never "一十二", for +12).
        let mut out: Vec<String> = vec![self.to_cardinal(&pre)?];

        // if value < 0 and pre == 0: prepend the bare negword "負". The sign
        // test reads the ORIGINAL value, not the toward-zero-truncated pre —
        // for -0.5 that is the only place the minus can come from.
        let is_negative = match value {
            FloatValue::Float { value, .. } => *value < 0.0,
            FloatValue::Decimal { value, .. } => value.is_negative(),
        };
        if is_negative && pre.is_zero() {
            out.insert(0, NEGWORD.to_string());
        }

        // if self.precision: out.append(select_text(title(pointword)))
        if precision > 0 {
            out.push(POINTWORD.to_string());
        }

        // for i in range(precision): out.append(to_cardinal(int(post[i])))
        for ch in post_str.chars().take(precision as usize) {
            let d = ch
                .to_digit(10)
                .ok_or_else(|| N2WError::Value(format!("non-digit {:?} in fractional part", ch)))?;
            out.push(self.to_cardinal(&BigInt::from(d))?);
        }

        // out = zh_to_cap(" ".join(out), capital=False); return out.replace(" ","")
        let joined = out.join(" ");
        Ok(self.zh_to_cap(&joined).replace(' ', ""))
    }

    /// `to_ordinal(float/Decimal)` — `Num2Word_ZH.to_ordinal` runs
    /// `verify_ordinal(value)` first:
    ///
    /// ```python
    /// if not value == int(value):
    ///     raise TypeError(self.errmsg_floatord % value)   # any fractional value
    /// if not abs(value) == value:
    ///     raise TypeError(self.errmsg_negord % value)     # any negative value
    /// ```
    ///
    /// So `0.5`/`-1.5` are TypeError, `-1.0` is TypeError, but `-0.0` passes
    /// **both** checks (`-0.0 == 0` and `abs(-0.0) == -0.0`) and renders
    /// "第零". A whole positive value takes the integer cardinal —
    /// `to_ordinal(5.0)` == "第五", `to_ordinal(1e16)` == "第一京" — because
    /// `to_cardinal`'s `assert int(value) == value` routes it to the int path.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let Some(i) = value.as_whole_int() else {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_value_str(value)
            )));
        };
        if i.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                py_value_str(value)
            )));
        }
        Ok(format!("{}{}", ORD_PREFIX, self.to_cardinal(&i)?))
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s%s" % (第, value, "")` — no
    /// `verify_ordinal`, so every value renders literally: "第-1000000.0",
    /// "第0.5", "第1e+16", "第5.00".
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", ORD_PREFIX, repr_str))
    }

    /// `to_year(float/Decimal)` — `Num2Word_ZH.to_year`:
    ///
    /// ```python
    /// if not value == int(value):
    ///     raise TypeError(self.errmsg_floatyear % value)
    /// ```
    ///
    /// then digit-by-digit over `str(abs(int(value)))` — so whole floats
    /// reduce to the integer path exactly (`-21.0` → "公元前二一年",
    /// `1e20` → 一垓's twenty digits as 一 + nineteen 零 + 年), and `-0.0`
    /// truncates to 0, losing its sign: "零年".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let Some(i) = value.as_whole_int() else {
            return Err(N2WError::Type(format!(
                "Cannot treat float {} as year.",
                py_value_str(value)
            )));
        };
        self.to_year(&i)
    }

    // ZH_TW inherited Base's fraction recipe ("三 第四s"; a negative fraction
    // crashed on the tuple negword). It has no fraction rules, so the trait
    // default raises NotImplementedError (#217).

    // ---- currency -------------------------------------------------------
    //
    // `Num2Word_ZH.to_currency` is a wholesale override that shares nothing
    // with `Num2Word_Base.to_currency`, and its own body comments record what
    // it drops. Consequently `pluralize`, `_cents_verbose`, `_cents_terse`,
    // `CURRENCY_ADJECTIVES` and `CURRENCY_PRECISION` are all unreachable from
    // it, and their trait defaults are left alone:
    //
    //   * `CURRENCY_PRECISION` is `{}` (Base's, never touched), so
    //     `.get(code, 100)` is the default 100 for every code.
    //   * `CURRENCY_ADJECTIVES` is `{}` too, and `to_currency` ignores the
    //     `adjective` kwarg regardless ("# CURRENCY_ADJECTIVES are not
    //     implemented").
    //   * `pluralize` stays abstract, exactly as in Python — Chinese has no
    //     plural form to select, and nothing calls it.
    //
    // `_money_verbose` is inherited unchanged (`self.to_cardinal(number)`), so
    // the trait default already matches and `to_cheque` below uses it.

    fn lang_name(&self) -> &str {
        "Num2Word_ZH_TW"
    }

    /// `Num2Word_ZH.to_currency`.
    ///
    /// ```python
    /// self.set_str_selection(reading, prefer)
    /// left, right, is_negative = parse_currency_parts(val, is_int_with_cents=False)
    /// try:
    ///     cr = self.CURRENCY_FORMS[currency]
    /// except KeyError:
    ///     raise NotImplementedError('Currency code "%s" not implemented for "%s"' % ...)
    /// minus_str = self.negword if is_negative else ""
    /// money_str = self.to_cardinal(left, reading=reading, prefer=prefer)
    /// if currency == "XXX":
    ///     cr_pre, cr_post = ("", cr)
    /// else:
    ///     cr_pre, cr_post = (cr, self.CURRENCY_FORMS["XXX"])
    /// cents_str = self.to_currency_float(right, reading=reading, prefer=prefer)
    /// cheque = self.cheque_suffix if len(cents_str) == 0 and reading == "capital" else ""
    /// for c in [minus_str, money_str, cr_post, *cents_str, cheque]:
    ///     cr_pre += self.zh_to_cap(self.select_text(c), reading == "capital")
    /// return cr_pre
    /// ```
    ///
    /// Departures from `Num2Word_Base.to_currency`, all deliberate on Python's
    /// side and all preserved:
    ///
    /// 1. **`cents`, `separator` and `adjective` are accepted and ignored.**
    ///    Nothing in the body reads them, so the separator never appears and no
    ///    cent word is ever pluralized.
    /// 2. **`has_decimal` is ignored** ("# has_decimal is not implemented"), so
    ///    the int/float split that drives Base collapses: `1` and `1.0` both
    ///    give "一元", because `to_currency_float(0)` returns `[]` either way.
    /// 3. **`CURRENCY_PRECISION` is never consulted** — `parse_currency_parts`
    ///    runs at its default `divisor=100` for every code. JPY therefore grows
    ///    the 角/分 subunits it does not have: `12.34 JPY` → "日元十二元三角四分"
    ///    (corpus-confirmed), where Base would round to a whole yen.
    /// 4. **The currency name precedes the sign**, since `cr_pre` seeds the
    ///    accumulator ahead of `minus_str`: −12.34 EUR → "歐元負十二元三角四分".
    /// 5. **`minus_str` is `self.negword` verbatim** — no `"%s " % ...strip()`,
    ///    so no space is introduced.
    /// 6. `money_str` is built from the already-absolute `left`, so `zh_to_cap`
    ///    *does* strip the leading 一 of "一十二" here. The float cardinal path
    ///    keeps it ("負一十二點三四") because its "負 " prefix blocks the
    ///    `startswith` test — see the module docs. Same number, two spellings.
    ///
    /// `cr_pre` is also the one component never passed through `zh_to_cap`.
    /// Immaterial at `reading=False` (no currency name starts with "一十"), but
    /// it is why 元 inside a name would survive capital mode uncapitalized.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
    ) -> Result<String> {
        // parse_currency_parts(val, is_int_with_cents=False); the rest stays at
        // the Python defaults (keep_precision=False, divisor=100). Python
        // parses before the CURRENCY_FORMS lookup — order kept, though this
        // call cannot fail once the shim has parsed the value.
        let (left, right, is_negative) = parse_currency_parts(val, false, false, 100);

        let form = *self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        let minus_str = if is_negative { NEGWORD } else { "" };
        let money_str = self.to_cardinal(&left)?;

        // `cr_post` is `CURRENCY_FORMS["XXX"]` in both arms — as `cr` itself
        // when currency == "XXX", by re-lookup otherwise — so it always
        // resolves to 元 and only `cr_pre` actually branches.
        let cr_pre: &str = if currency == "XXX" {
            ""
        } else {
            match form {
                Form::Name(name) => name,
                // Unreachable: "XXX" is the table's only nested entry, and it
                // took the other arm. Python would reach `tuple += str` here.
                Form::Nested => {
                    return Err(N2WError::Type(
                        "can only concatenate tuple (not \"str\") to tuple".into(),
                    ))
                }
            }
        };

        // `right` carries scale 0 — parse_currency_parts truncates it with
        // `int(fraction * divisor)` — so this is exact.
        let cents_str = self.to_currency_float(&right.as_bigint_and_exponent().0);

        // `cheque = self.cheque_suffix if len(cents_str) == 0 and reading ==
        // "capital" else ""`. `reading` is fixed at False, so 正 never appears
        // and the trailing component is always "".

        // for c in [minus_str, money_str, cr_post, *cents_str, cheque]
        let mut out = String::from(cr_pre);
        out.push_str(&self.zh_to_cap(minus_str));
        out.push_str(&self.zh_to_cap(&money_str));
        out.push_str(&self.zh_to_cap(XXX_UNIT));
        for c in &cents_str {
            out.push_str(&self.zh_to_cap(c));
        }
        Ok(out)
    }

    /// `Num2Word_Base.to_cheque`, which `Num2Word_ZH`/`_ZH_TW` inherit
    /// unchanged — but which cannot delegate to `currency::default_to_cheque`,
    /// because Base's `cr1, _cr2 = self.CURRENCY_FORMS[currency]` is here
    /// destructuring ZH's *name strings* rather than a (unit, subunit) pair.
    ///
    /// ```python
    /// try:
    ///     cr1, _cr2 = self.CURRENCY_FORMS[currency]
    /// except KeyError:
    ///     raise NotImplementedError('Currency code "%s" not implemented for "%s"' % ...)
    /// ...
    /// unit = cr1[-1] if isinstance(cr1, tuple) else cr1
    /// ```
    ///
    /// Unpacking a `str` iterates its characters, so the outcome hinges on the
    /// name's *length* — a Python bug, reproduced verbatim:
    ///
    /// | entry | `cr1` | result |
    /// |---|---|---|
    /// | "歐元" (2 chars) | "歐" | "… AND 56/100 歐" — half the currency name |
    /// | "人民幣" (3 chars) | — | **ValueError**; `except KeyError` misses it |
    /// | `(("元",), ("ㄩㄢˊ",))` | `("元",)` | tuple → `cr1[-1]` → "元" |
    ///
    /// So `to_cheque` succeeds only for "XXX" and the nine two-character names
    /// (AUD CAD EUR GBP HKD JPY KRW THB USD), and raises ValueError for the
    /// seven longer ones (CHF CNY MOP MYR NTD NZD SGD). Corpus-confirmed on
    /// both sides: `cheque:EUR` → "一千二百三十四 AND 56/100 歐", `cheque:CNY`
    /// → ValueError, `cheque:KWD` → NotImplementedError.
    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        let form = *self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        // cr1, _cr2 = <form>
        // unit = cr1[-1] if isinstance(cr1, tuple) else cr1
        let unit: String = match form {
            Form::Nested => XXX_UNIT.to_string(),
            Form::Name(name) => match name.chars().count() {
                // cr1 is the first character, a `str` — so `unit = cr1`.
                2 => name
                    .chars()
                    .next()
                    .expect("a 2-char name has a first char")
                    .to_string(),
                n if n > 2 => {
                    return Err(N2WError::Value(
                        "too many values to unpack (expected 2)".into(),
                    ))
                }
                // Unreachable for ZH_TW's table, but live for the parent
                // `Num2Word_ZH`, whose "XXX" stays the 1-char str "元".
                n => {
                    return Err(N2WError::Value(format!(
                        "not enough values to unpack (expected 2, got {})",
                        n
                    )))
                }
            },
        };

        // CURRENCY_PRECISION is `{}`, so this is always 100 and `fraction_str`
        // is always the "NN/100" form — the `divisor > 1` else-arm below is
        // dead. Kept so the shape still matches Base.
        let divisor = self.currency_precision(currency);
        let is_negative = val.is_negative();
        let abs_val = val.abs();
        // int(abs_val) — truncation, which on a non-negative value is floor.
        let whole = abs_val.with_scale(0).as_bigint_and_exponent().0;

        let fraction_str = if divisor > 1 {
            let sub = (&abs_val - BigDecimal::from(whole.clone())) * BigDecimal::from(divisor);
            let sub = sub.with_scale(0).as_bigint_and_exponent().0;
            let digits = divisor.to_string().len() - 1;
            format!("{:0>width$}/{}", sub.to_string(), divisor, width = digits)
        } else {
            String::new()
        };

        let words = self.money_verbose(&whole, currency)?;
        let sign = if is_negative { "MINUS " } else { "" };
        let body = if fraction_str.is_empty() {
            format!("{} {}", words, unit)
        } else {
            format!("{} AND {} {}", words, fraction_str, unit)
        };
        // Python's `.upper()`. A no-op on CJK, and "AND"/"MINUS" are already
        // uppercase — but it is what Base does, so it is what runs.
        Ok(format!("{}{}", sign, body).to_uppercase())
    }

    // ---- grammatical kwargs ----------------------------------------------
    //
    // The live signatures, from lang_ZH.py / lang_ZH_TW.py:
    //
    //   to_cardinal(value, stuff_zero=2, reading=False, prefer=None)
    //   to_ordinal(value, counter="", reading=False, prefer=None)
    //   to_ordinal_num(value, counter="", reading=False, prefer=None)
    //   to_year(value, era=False, reading=False, prefer=None)
    //   to_currency(val, currency, cents, separator, adjective,
    //               reading=False, prefer=None)
    //
    // Each hook guards with `kw.only(...)` so an unknown kwarg falls back to
    // Python, which raises the original TypeError.

    /// `Num2Word_ZH_TW.to_cardinal` with `stuff_zero`/`reading`/`prefer` live.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_cardinal(value);
        }
        if !kw.only(&["stuff_zero", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        self.kw_cardinal(value, parse_stuff_zero(kw), reading, prefer.as_deref())
    }

    /// `Num2Word_ZH_TW.to_ordinal` with `counter`/`reading`/`prefer` live.
    ///
    /// ```python
    /// if reading is True:
    ///     if counter not in self.counters and counter:
    ///         raise NotImplementedError(f"Reading not implemented for {counter}")
    ///     counter = self.counters.get(counter, "")
    /// return super().to_ordinal(value, counter=counter, ...)
    /// ```
    ///
    /// With `reading` anything but the bool `True`, the counter passes
    /// through verbatim — "第一个" for `counter="个"` — and `verify_ordinal`
    /// still rejects negatives with TypeError.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal(value);
        }
        if !kw.only(&["counter", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        let mut counter = parse_counter(kw)?;
        if reading == Reading::True {
            match tw_counter(counter) {
                Some(c) => counter = c,
                None if !counter.is_empty() => {
                    // Python raises NotImplementedError here with this exact
                    // message; surfacing NotImplemented sends the call back to
                    // Python, which raises the real thing.
                    return Err(N2WError::NotImplemented(format!(
                        "Reading not implemented for {}",
                        counter
                    )));
                }
                None => counter = "",
            }
        }
        self.verify_ordinal(value)?;
        let base = self.kw_cardinal(value, STUFF_ZERO as i64, reading, prefer.as_deref())?;
        Ok(format!(
            "{}{}{}",
            select2(ORD_ALTS, reading, prefer.as_deref()),
            base,
            counter
        ))
    }

    /// `Num2Word_ZH_TW.to_ordinal_num` with the kwargs live — same counter
    /// handling as `to_ordinal`, but no `verify_ordinal`, so "第-5个".
    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal_num(value);
        }
        if !kw.only(&["counter", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        let mut counter = parse_counter(kw)?;
        if reading == Reading::True {
            match tw_counter(counter) {
                Some(c) => counter = c,
                None if !counter.is_empty() => {
                    return Err(N2WError::NotImplemented(format!(
                        "Reading not implemented for {}",
                        counter
                    )));
                }
                None => counter = "",
            }
        }
        Ok(format!(
            "{}{}{}",
            select2(ORD_ALTS, reading, prefer.as_deref()),
            value,
            counter
        ))
    }

    /// `to_year(float/Decimal, era=..., reading=..., prefer=...)`: both the
    /// era branch's own `value == int(value)` guard and the inherited ZH one
    /// raise the same TypeError for a non-integral value ("Cannot treat
    /// float 2020.1 as year."); a whole value continues into the integer
    /// `to_year_kw`.
    fn year_float_kw(&self, value: &FloatValue, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.year_float_entry(value);
        }
        if !kw.only(&["era", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        match value.as_whole_int() {
            Some(i) => self.to_year_kw(&i, kw),
            None => self.year_float_entry(value),
        }
    }

    /// `Num2Word_ZH_TW.to_year` with `era`/`reading`/`prefer` live.
    ///
    /// `era` falsy → `Num2Word_ZH.to_year` (digit-by-digit, 公元前 for
    /// negatives, 公元 prefix for positive years under `reading="capital"`).
    ///
    /// `era` truthy → the ROC branch:
    ///
    /// ```python
    /// min_year = 1912
    /// if value < min_year:
    ///     raise ValueError("Can't convert years less than %s to ROC era" % min_year)
    /// era_year = abs(int(value - min_year + 1))
    /// if reading == "arabic":  era_year_words = era_year
    /// elif era_year == 1:      era_year_words = select_text((("元",), ("ㄩㄢˊ",)))
    /// elif era_year < 101:     era_year_words = self.to_cardinal(era_year)
    /// else:                    era_year_words = "".join(select_text(cards[int(s)]) ...)
    /// return "%s%s%s" % (select_text(ROC_era), era_year_words, select_text(year))
    /// ```
    ///
    /// Note the `< 101` arm calls `to_cardinal` at its **defaults**, whose
    /// `set_str_selection(False, None)` resets the live selection — so the
    /// surrounding 民國/年 come out in hanzi even under `reading=True`.
    /// Reproduced by dropping to `Reading::Plain` for the tail in that arm.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_year(value);
        }
        if !kw.only(&["era", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        if !kw_truthy(kw.get("era")) {
            return self.kw_year(value, reading, prefer.as_deref());
        }

        // `if not value == int(value)` cannot fire: input is integral here.
        let min_year = BigInt::from(1912);
        if value < &min_year {
            return Err(N2WError::Value(
                "Can't convert years less than 1912 to ROC era".into(),
            ));
        }
        // abs(int(value - 1912 + 1)) — non-negative once value >= 1912.
        let era_year = value - BigInt::from(1911);

        // The selection state the trailing 民國/年 renders under; the
        // to_cardinal arm resets it, exactly as Python's side effect does.
        let mut tail_reading = reading;
        let mut tail_prefer = prefer.clone();

        let era_year_words: String = if kw.str("reading") == Some("arabic") {
            era_year.to_string()
        } else if era_year.is_one() {
            select2(YUAN_ALTS, reading, prefer.as_deref()).to_string()
        } else if era_year < BigInt::from(101) {
            // self.to_cardinal(era_year) — defaults, which also reset
            // self.reading/self.prefer for the format below.
            tail_reading = Reading::Plain;
            tail_prefer = None;
            self.to_cardinal(&era_year)?
        } else {
            era_year
                .to_string()
                .chars()
                .map(|c| {
                    select2(
                        TW_LOW[c.to_digit(10).expect("BigInt digits") as usize],
                        reading,
                        prefer.as_deref(),
                    )
                })
                .collect()
        };

        Ok(format!(
            "{}{}{}",
            select2(ROC_ERA_ALTS, tail_reading, tail_prefer.as_deref()),
            era_year_words,
            select2(YEAR_W_ALTS, tail_reading, tail_prefer.as_deref())
        ))
    }

    /// `Num2Word_ZH.to_currency` with `reading`/`prefer` live — the capital
    /// mode is where the 大寫 forms and the 整 cheque suffix appear:
    /// `to_currency(1234, reading="capital")` == "壹仟貳佰叁拾肆圓整".
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        _cents: bool,
        _separator: Option<&str>,
        _adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.to_currency(val, currency, _cents, _separator, _adjective);
        }
        if !kw.only(&["reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        let prefer = prefer.as_deref();

        let (left, right, is_negative) = parse_currency_parts(val, false, false, 100);

        let form = *self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;

        // minus_str = self.negword — resolved by select_text in the loop.
        let minus_str = if is_negative {
            select2(NEG_ALTS, reading, prefer)
        } else {
            ""
        };
        let money_str = self.kw_cardinal(&left, STUFF_ZERO as i64, reading, prefer)?;

        let cr_pre: &str = if currency == "XXX" {
            ""
        } else {
            match form {
                Form::Name(name) => name,
                Form::Nested => {
                    return Err(N2WError::Type(
                        "can only concatenate tuple (not \"str\") to tuple".into(),
                    ))
                }
            }
        };
        // cr_post — CURRENCY_FORMS["XXX"], the nested tuple, select_text'd.
        let cr_post = select2(YUAN_ALTS, reading, prefer);

        let cents_str = self.kw_currency_float(&right.as_bigint_and_exponent().0, reading, prefer);

        // cheque = cheque_suffix if len(cents_str) == 0 and reading == "capital"
        let cheque = if cents_str.is_empty() && reading == Reading::Capital {
            select2(CHEQUE_ALTS, reading, prefer)
        } else {
            ""
        };

        // for c in [minus_str, money_str, cr_post, *cents_str, cheque]:
        //     cr_pre += self.zh_to_cap(self.select_text(c), reading == "capital")
        let mut out = String::from(cr_pre);
        out.push_str(&self.kw_zh_to_cap(minus_str, reading, prefer));
        out.push_str(&self.kw_zh_to_cap(&money_str, reading, prefer));
        out.push_str(&self.kw_zh_to_cap(cr_post, reading, prefer));
        for c in &cents_str {
            out.push_str(&self.kw_zh_to_cap(c, reading, prefer));
        }
        out.push_str(&self.kw_zh_to_cap(cheque, reading, prefer));
        Ok(out)
    }

    /// `Num2Word_ZH_TW.to_cardinal(float, stuff_zero, reading, prefer)`: the
    /// `assert int(value) == value` routes whole values to the (kwargs-live)
    /// integer path — where `stuff_zero` applies — and fractional values to
    /// `to_cardinal_float(value, reading, prefer)`, whose inner `to_cardinal`
    /// calls reset `stuff_zero` to its default 2 (Python re-assigns
    /// `self.stuff_zero` on every call).
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.cardinal_float_entry(value, precision_override);
        }
        if !kw.only(&["stuff_zero", "reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        if let Some(i) = value.as_whole_int() {
            return self.kw_cardinal(&i, parse_stuff_zero(kw), reading, prefer.as_deref());
        }
        self.kw_cardinal_float(value, reading, prefer.as_deref())
    }
}
