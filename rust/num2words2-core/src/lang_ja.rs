//! Port of `lang_JA.py` (Japanese).
//!
//! Shape: **engine**. `Num2Word_JA` subclasses `Num2Word_Base`, defines
//! `high_numwords`/`mid_numwords`/`low_numwords` + `merge`, and lets the
//! inherited `splitnum`/`clean` machinery drive the conversion. So `cards`,
//! `maxval` and `merge` are all supplied here and `base::splitnum`/
//! `base::clean` do the folding.
//!
//! `to_cardinal` is nevertheless overridden below rather than left to
//! `base::default_to_cardinal`, because JA ships its own copy that differs in
//! exactly one observable way: the negative prefix. Base does
//! `out = "%s " % self.negword.strip()` (word + space); JA does
//! `out = self.negword` verbatim, so there is **no space**:
//! `to_cardinal(-1) == "マイナス一"`, not "マイナス 一".
//!
//! # The `reading` / `prefer` collapse
//!
//! Python's JA module threads two extra keyword arguments — `reading` (kanji
//! vs. hiragana) and `prefer` (disambiguates alternative readings) — through
//! `to_cardinal`/`to_ordinal`/`to_year`, and its `splitnum` funnels every card
//! lookup through `select_text(text, reading, prefer)`. The plain `Lang`
//! methods expose no such parameters, so on that surface the only reachable
//! configuration is the default `reading=False, prefer=None`. (The `*_kw`
//! hooks DO reach the other configurations; see "Grammatical kwargs" below —
//! they run on a separate, unresolved card table.) Under the default
//! configuration `select_text` is a pure, input-independent projection:
//!
//!   * `reading=False` picks `text[0]`, i.e. the kanji half of every entry.
//!   * For entries whose kanji half is itself a tuple of alternatives — only
//!     `cards[0] == (("零", "〇"), ("ゼロ", "れい"))` — `prefer=None` makes
//!     `common` empty, so `len(common) == 1` is false and it falls back to
//!     `text[0]` again: **"零"** (never "〇").
//!   * The seven/four alternatives `("七", ("なな", "しち"))` and
//!     `("四", ("よん", "し"))` only branch on the *reading* half, so the
//!     kanji path takes "七"/"四" unconditionally.
//!
//! Because the projection is constant, the cards are pre-resolved to plain
//! kanji strings in `new()` and `base::splitnum` (which has no `select_text`
//! hook) becomes exactly equivalent. This was verified against the Python
//! interpreter over 12,060 values — every integer in -2000..5000, every power
//! of ten up to 10^51, the overflow boundary, and 5,000 random values drawn
//! from the full 0..10^51 range — with zero mismatches.
//!
//! # Grammatical kwargs (`reading=`, `prefer=`, `counter=`, `era=`)
//!
//! The `*_kw` hooks port the full Python surface:
//!
//!   * `to_cardinal(value, reading=False, prefer=None)` — `reading` truthy
//!     (including the string `"arabic"`) selects the hiragana half of every
//!     card; `prefer` disambiguates multi-alternative entries (0 kanji
//!     `("零", "〇")`; 0/4/7 kana `("ゼロ", "れい")`/`("よん", "し")`/
//!     `("なな", "しち")`) — a match count of exactly 1 wins, otherwise the
//!     first alternative. The kana path runs through `rendaku_merge_pairs`,
//!     whose rules genuinely fire there (さん+ひゃく → さんびゃく,
//!     いち+ちょう → いっちょう, ...).
//!   * `to_ordinal(value, reading=False, prefer=None, counter="番")` —
//!     `verify_ordinal` FIRST (so a negative raises TypeError even when the
//!     suffix would raise), then `_ordinal_suffix`: reading + any counter
//!     other than "番" raises `NotImplementedError("Reading not implemented
//!     for %s" % counter)`; non-reading appends `counter + "目"` verbatim
//!     (no counter validation: "つ" gives 十一つ目).
//!   * `to_ordinal_num(value, reading=False, counter="番")` — no `prefer`,
//!     no verify; `"-5個目"` is real.
//!   * `to_year(val, suffix=None, longval=True, reading=False, prefer=None,
//!     era=True)` — `suffix`/`longval` are accepted and never read. `era`
//!     falsy (including an explicit `era=None`) takes the Gregorian branch:
//!     `紀元前`/`きげんぜん` prefix for negatives, and on the reading path a
//!     year ending in 9 swaps its last three chars (きゅう) for く. The era
//!     branch renders year 1 as 元/がん, applies the same …九 → …く fix, and
//!     `reading == "arabic"` (exact string compare) keeps the kanji era name
//!     with an Arabic era-year. `prefer` also participates in the era search:
//!     on a duplicate start year it backtracks to an era whose kanji or kana
//!     name is preferred.
//!   * `to_currency(..., reading=False, prefer=None)` — slot 1 of
//!     CURRENCY_FORMS (the reading) replaces slot 0, cardinals go kana, but
//!     `minus_str` stays `self.negword` ("マイナスごえん").
//!   * `to_cardinal_float(value, reading=False, prefer=None)` — defaults
//!     `prefer` to `["れい"]`, so a kana fractional digit 0 reads れい while
//!     the kanji one stays 零 (れい matches nothing on the kanji side).
//!
//! The kwargs engine (`splitnum_kw`/`clean_kw`/`merge_kw`) is a verbatim
//! port of Python's `splitnum` + `Num2Word_Base.clean` over the *unresolved*
//! card table; with `reading=False, prefer=None` it reduces to the same
//! output as the pre-resolved fast path (the projection argument above).
//! The plain trait methods keep using the fast path.
//!
//! # Float / Decimal entry routing
//!
//! `to_ordinal`, `to_ordinal_num` and `to_year` accept floats/Decimals in
//! Python, and each does something different (the base defaults match none
//! of them):
//!
//!   * `to_ordinal(float)` → `Num2Word_Base.verify_ordinal`: non-whole →
//!     `TypeError("Cannot treat float %s as ordinal.")`; whole-negative →
//!     `TypeError("Cannot treat negative num %s as ordinal.")`; whole
//!     non-negative → integer cardinal + 番目. `-0.0` passes both checks
//!     (`int(-0.0) == -0.0`, `abs(-0.0) == -0.0`) → 零番目.
//!   * `to_ordinal_num(float)` → `"%s%s" % (value, "番目")` — the raw
//!     `str(value)`, so `"-1000000.0番目"`, `"1e+16番目"`, `"5.00番目"`.
//!   * `to_year(float)` → the era search runs on the raw number.
//!     Anything below 645 — all negatives, ±0.0, fractions — raises
//!     `ValueError("Can't convert years less than 645 to era")`. Above it,
//!     `era_year = year - start + 1` is computed in the value's own
//!     arithmetic: f64 for floats (so `to_year(1e16)` is 令和…七千九百八十年,
//!     the double rounding of `1e16 - 2019 + 1`, while `Decimal("1E+16")`
//!     would be exact) and BigDecimal for Decimals
//!     (`Decimal("1E+20")` → 令和九千九百九十九京…七千九百八十二年).
//!
//! # Rendaku is dead code on the plain-trait path
//!
//! `rendaku_merge_pairs` applies sequential-voicing euphony (三+ひゃく →
//! さんびゃく, 六+ひゃく → ろっぴゃく, ...). Every one of its rules is
//! gated on an equality test against a **hiragana** pair literal —
//! `rpair == ("ひゃく", 100)`, `lpair == ("さん", 3)`, and so on. On the
//! kanji path the pairs are `("百", 100)` / `("三", 3)`, which never compare
//! equal, so the function degrades to its fall-through
//! `("%s%s" % (ltext, rtext), lnum * rnum)`. Its `if lnum > rnum: raise
//! ValueError` guard is likewise unreachable, because the sole caller
//! (`merge`) only invokes it on the `lnum < rnum` branch. Verified
//! exhaustively against the interpreter for every kanji pair that can reach
//! it. `merge` below therefore inlines the fall-through and notes the
//! omission rather than porting rules that can never fire. (The kwargs
//! engine's `merge_kw` DOES port the full rule set — the kana path reaches
//! them.)
//!
//! # Inherited from `Num2Word_Base`
//!
//!   * `is_title` is False (JA never sets it), so `title()` is the identity.
//!     JA does populate `exclude_title = ["点", "マイナス"]`, but it is only
//!     consulted by `title()` when `is_title` is True — i.e. never. It is
//!     omitted here.
//!   * `verify_ordinal` supplies the negative-ordinal `TypeError`.
//!   * `MAXVAL = 1000 * list(self.cards.keys())[0]`. The OrderedDict's first
//!     *inserted* key is the first high numword, 10^48, so MAXVAL is 10^51 —
//!     which coincides with `Cards::highest()` here since the highs are both
//!     inserted first and the largest.
//!
//! # Notable faithful behaviours
//!
//! 1. `merge`'s `lnum == 1 and rnum < 10000` short-circuit is what makes
//!    Japanese drop the leading 一 below a myriad but keep it at and above
//!    one: `1000 -> "千"` (not "一千") yet `10000 -> "一万"`. The cutoff is
//!    the literal `10000`, not the card's own magnitude.
//! 2. `to_ordinal_num` never calls `verify_ordinal` and never calls
//!    `to_cardinal` — it string-formats the raw integer. So negatives pass
//!    straight through: `to_ordinal_num(-1) == "-1番目"`, while
//!    `to_ordinal(-1)` raises `TypeError`. That asymmetry is real.
//! 3. `to_year` raises `ValueError` (not `OverflowError`) for any year below
//!    645, the start of the first recorded era — including all negative
//!    years. There is no BCE handling on the reachable path.
//! 4. `to_year` past the last era (令和, 2019) keeps counting within it
//!    rather than failing: `to_year(9999) == "令和七千九百八十一年"`.
//! 5. Era year 1 renders as 元 ("gan", the origin year), not 一:
//!    `to_year(999) == "長保元年"`.
//!
//! # Currency
//!
//! `Num2Word_JA` overrides `to_currency` **wholesale** and shares almost
//! nothing with `Num2Word_Base.to_currency`. The differences are all
//! observable:
//!
//! 1. **`CURRENCY_FORMS` holds `(kanji, reading)`, not `(singular, plural)`.**
//!    Every other language's tuple is a plural table that `pluralize` indexes;
//!    JA's second slot is the *hiragana reading*, selected by the `reading=`
//!    kwarg. Only JPY actually differs across the two slots —
//!    `("円", "えん")` — the other four repeat themselves. This collision is
//!    load-bearing for `to_cheque`; see below.
//! 2. **`pluralize` is never called.** JA concatenates `cr1[0]` directly, so
//!    the abstract `Num2Word_Base.pluralize` (which raises) is never reached
//!    and is correctly left at the trait default here.
//! 3. **No separator, and no space anywhere.** The pieces are glued with
//!    `"%s%s%s%s%s"`. `separator=` is accepted and then never read — verified
//!    against the interpreter, `separator="XX"` changes nothing.
//! 4. **`minus_str` is `self.negword` verbatim**, not Base's
//!    `"%s " % negword.strip()`: `-12.34 EUR -> "マイナス十二ユーロ三十四セント"`.
//! 5. **No `has_decimal` guard.** Base prints a zero-cent segment for a float
//!    like `1.0`; JA gates the cents purely on `cr2 and right`, so
//!    `1.0 EUR -> "一ユーロ"` — same as the int `1`.
//! 6. **The divisor is the literal `100`, always.** JA defines no
//!    `CURRENCY_PRECISION`, so `currency_precision()` stays at the trait
//!    default of 100 and is deliberately not overridden. JPY is therefore a
//!    *2-decimal* currency here, with 銭 (sen) as its subunit:
//!    `12.34 JPY -> "十二円三十四銭"`. There is no `divisor == 1` or
//!    `divisor == 1000` path in this language, and KWD/BHD are simply absent.
//!
//! ## `cents=` and `adjective=` are inert
//!
//! `cents` reaches only `parse_currency_parts(val, is_int_with_cents=cents)`,
//! and by then `val` has been coerced to a float — so the flag lands in a
//! branch that is never taken. The one other reader,
//! `if (cents or abs(val) != left) and not cr2`, is guarded by `not cr2`, and
//! every `cr2` in JA's table is a non-empty tuple. `adjective` is likewise
//! dead: `CURRENCY_ADJECTIVES` is `{}`. All three were confirmed inert against
//! the interpreter over the full corpus argument set. This matters because the
//! corpus rows were generated with JA's own `cents=False` default while the
//! diff harness passes `cents=True`; for JA the two agree.
//!
//! ## `int -> float` is load-bearing
//!
//! `if isinstance(val, int): val = float(val)` runs *before*
//! `parse_currency_parts`, forcing it down its non-int branch. Handing a
//! `CurrencyValue::Int` straight to `parse_currency_parts` with
//! `is_int_with_cents=true` would instead split it into units+cents and render
//! `1000000 EUR` as "一万ユーロ" rather than "百万ユーロ".
//!
//! ## `to_cheque` is inherited, and inherits a bug
//!
//! `Num2Word_Base.to_cheque` takes `unit = cr1[-1]`, commented "cheque
//! convention always uses the plural currency name". JA's last slot is the
//! *reading*, so JPY cheques come out in hiragana while the amount stays in
//! kanji: `to_cheque(1234.56, "JPY") == "千二百三十四 AND 56/100 えん"`. The
//! corpus freezes exactly that, so `to_cheque` is left at the trait default —
//! `default_to_cheque` reproduces it once `currency_forms` carries the real
//! 2-arity tuples. (`.upper()` is a no-op on kana/kanji; only the literal
//! "AND" is ASCII.)

use crate::base::{
    clean, set_low_numwords, set_mid_numwords, splitnum, Cards, KwVal, Kwargs, Lang, N2WError,
    Node, Result,
};
use crate::currency::{parse_currency_parts, prefix_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::collections::HashMap;

/// `ERA_START` from `lang_JA.py`, source https://www.sljfaq.org/afaq/era-list.html
///
/// Ascending by start year, with **intentional duplicate years** — the module
/// comments "if there are multiple eras for the same year, use the last one",
/// and the binary search in [`LangJa::to_year`] does exactly that by walking
/// right past equal keys (see there).
///
/// Python stores `(year, (kanji, hiragana))`; both halves are carried here as
/// `(year, kanji, hiragana)`. The hiragana reading is reachable through the
/// `*_kw` hooks (`to_year(..., reading=True)` renders the era name in kana,
/// and the `prefer=` walk in the era search matches either half). See the
/// `reading`/`prefer` note in the module docs.
const ERA_START: &[(i64, &str, &str)] = &[
    (645, "大化", "たいか"),
    (650, "白雉", "はくち"),
    (686, "朱鳥", "しゅちょう"),
    (701, "大宝", "たいほう"),
    (704, "慶雲", "けいうん"),
    (708, "和銅", "わどう"),
    (715, "霊亀", "れいき"),
    (717, "養老", "ようろう"),
    (724, "神亀", "じんき"),
    (729, "天平", "てんぴょう"),
    (749, "天平感宝", "てんぴょうかんぽう"),
    (749, "天平勝宝", "てんぴょうしょうほう"),
    (757, "天平宝字", "てんぴょうじょうじ"),
    (765, "天平神護", "てんぴょうじんご"),
    (767, "神護景雲", "じんごけいうん"),
    (770, "宝亀", "ほうき"),
    (781, "天応", "てんおう"),
    (782, "延暦", "えんりゃく"),
    (806, "大同", "だいどう"),
    (810, "弘仁", "こうにん"),
    (823, "天長", "てんちょう"),
    (834, "承和", "じょうわ"),
    (848, "嘉祥", "かしょう"),
    (851, "仁寿", "にんじゅ"),
    (855, "斉衡", "さいこう"),
    (857, "天安", "てんあん"),
    (859, "貞観", "じょうがん"),
    (877, "元慶", "がんぎょう"),
    (885, "仁和", "にんな"),
    (889, "寛平", "かんぴょう"),
    (898, "昌泰", "しょうたい"),
    (901, "延喜", "えんぎ"),
    (923, "延長", "えんちょう"),
    (931, "承平", "じょうへい"),
    (938, "天慶", "てんぎょう"),
    (947, "天暦", "てんりゃく"),
    (957, "天徳", "てんとく"),
    (961, "応和", "おうわ"),
    (964, "康保", "こうほう"),
    (968, "安和", "あんな"),
    (970, "天禄", "てんろく"),
    (974, "天延", "てんえん"),
    (976, "貞元", "じょうげん"),
    (979, "天元", "てんげん"),
    (983, "永観", "えいかん"),
    (985, "寛和", "かんな"),
    (987, "永延", "えいえん"),
    (989, "永祚", "えいそ"),
    (990, "正暦", "しょうりゃく"),
    (995, "長徳", "ちょうとく"),
    (999, "長保", "ちょうほう"),
    (1004, "寛弘", "かんこう"),
    (1013, "長和", "ちょうわ"),
    (1017, "寛仁", "かんにん"),
    (1021, "治安", "じあん"),
    (1024, "万寿", "まんじゅ"),
    (1028, "長元", "ちょうげん"),
    (1037, "長暦", "ちょうりゃく"),
    (1040, "長久", "ちょうきゅう"),
    (1045, "寛徳", "かんとく"),
    (1046, "永承", "えいしょう"),
    (1053, "天喜", "てんぎ"),
    (1058, "康平", "こうへい"),
    (1065, "治暦", "じりゃく"),
    (1069, "延久", "えんきゅう"),
    (1074, "承保", "じょうほう"),
    (1078, "承暦", "じょうりゃく"),
    (1081, "永保", "えいほう"),
    (1084, "応徳", "おうとく"),
    (1087, "寛治", "かんじ"),
    (1095, "嘉保", "かほう"),
    (1097, "永長", "えいちょう"),
    (1098, "承徳", "じょうとく"),
    (1099, "康和", "こうわ"),
    (1104, "長治", "ちょうじ"),
    (1106, "嘉承", "かじょう"),
    (1108, "天仁", "てんにん"),
    (1110, "天永", "てんねい"),
    (1113, "永久", "えいきゅう"),
    (1118, "元永", "げんえい"),
    (1120, "保安", "ほうあん"),
    (1124, "天治", "てんじ"),
    (1126, "大治", "だいじ"),
    (1131, "天承", "てんしょう"),
    (1132, "長承", "ちょうしょう"),
    (1135, "保延", "ほうえん"),
    (1141, "永治", "えいじ"),
    (1142, "康治", "こうじ"),
    (1144, "天養", "てんよう"),
    (1145, "久安", "きゅうあん"),
    (1151, "仁平", "にんぺい"),
    (1154, "久寿", "きゅうじゅ"),
    (1156, "保元", "ほうげん"),
    (1159, "平治", "へいじ"),
    (1160, "永暦", "えいりゃく"),
    (1161, "応保", "おうほう"),
    (1163, "長寛", "ちょうかん"),
    (1165, "永万", "えいまん"),
    (1166, "仁安", "にんあん"),
    (1169, "嘉応", "かおう"),
    (1171, "承安", "しょうあん"),
    (1175, "安元", "あんげん"),
    (1177, "治承", "じしょう"),
    (1181, "養和", "ようわ"),
    (1182, "寿永", "じゅえい"),
    (1184, "元暦", "げんりゃく"),
    (1185, "文治", "ぶんじ"),
    (1190, "建久", "けんきゅう"),
    (1199, "正治", "しょうじ"),
    (1201, "建仁", "けんにん"),
    (1204, "元久", "げんきゅう"),
    (1206, "建永", "けんえい"),
    (1207, "承元", "じょうげん"),
    (1211, "建暦", "けんりゃく"),
    (1214, "建保", "けんぽう"),
    (1219, "承久", "じょうきゅう"),
    (1222, "貞応", "じょうおう"),
    (1225, "元仁", "げんにん"),
    (1225, "嘉禄", "かろく"),
    (1228, "安貞", "あんてい"),
    (1229, "寛喜", "かんき"),
    (1232, "貞永", "じょうえい"),
    (1233, "天福", "てんぷく"),
    (1235, "文暦", "ぶんりゃく"),
    (1235, "嘉禎", "かてい"),
    (1239, "暦仁", "りゃくにん"),
    (1239, "延応", "えんおう"),
    (1240, "仁治", "にんじ"),
    (1243, "寛元", "かんげん"),
    (1247, "宝治", "ほうじ"),
    (1249, "建長", "けんちょう"),
    (1256, "康元", "こうげん"),
    (1257, "正嘉", "しょうか"),
    (1259, "正元", "しょうげん"),
    (1260, "文応", "ぶんおう"),
    (1261, "弘長", "こうちょう"),
    (1264, "文永", "ぶんえい"),
    (1275, "健治", "けんじ"),
    (1278, "弘安", "こうあん"),
    (1288, "正応", "しょうおう"),
    (1293, "永仁", "えいにん"),
    (1299, "正安", "しょうあん"),
    (1303, "乾元", "けんげん"),
    (1303, "嘉元", "かげん"),
    (1307, "徳治", "とくじ"),
    (1308, "延慶", "えんきょう"),
    (1311, "応長", "おうちょう"),
    (1312, "正和", "しょうわ"),
    (1317, "文保", "ぶんぽう"),
    (1319, "元応", "げんおう"),
    (1321, "元亨", "げんこう"),
    (1325, "正中", "しょうちゅ"),
    (1326, "嘉暦", "かりゃく"),
    (1329, "元徳", "げんとく"),
    (1331, "元弘", "げんこう"),
    (1332, "正慶", "しょうけい"),
    (1334, "建武", "けんむ"),
    (1336, "延元", "えいげん"),
    (1338, "暦応", "りゃくおう"),
    (1340, "興国", "こうこく"),
    (1342, "康永", "こうえい"),
    (1345, "貞和", "じょうわ"),
    (1347, "正平", "しょうへい"),
    (1350, "観応", "かんおう"),
    (1352, "文和", "ぶんな"),
    (1356, "延文", "えんぶん"),
    (1361, "康安", "こうあん"),
    (1362, "貞治", "じょうじ"),
    (1368, "応安", "おうあん"),
    (1370, "建徳", "けんとく"),
    (1372, "文中", "ぶんちゅう"),
    (1375, "永和", "えいわ"),
    (1375, "天授", "てんじゅ"),
    (1379, "康暦", "こうりゃく"),
    (1381, "永徳", "えいとく"),
    (1381, "弘和", "こうわ"),
    (1384, "至徳", "しとく"),
    (1384, "元中", "げんちゅう"),
    (1387, "嘉慶", "かけい"),
    (1389, "康応", "こうおう"),
    (1390, "明徳", "めいとく"),
    (1394, "応永", "おうえい"),
    (1428, "正長", "しょうちょう"),
    (1429, "永享", "えいきょう"),
    (1441, "嘉吉", "かきつ"),
    (1444, "文安", "ぶんあん"),
    (1449, "宝徳", "ほうとく"),
    (1452, "享徳", "きょうとく"),
    (1455, "康正", "こうしょう"),
    (1457, "長禄", "ちょうろく"),
    (1461, "寛正", "かんしょう"),
    (1466, "文正", "ぶんしょう"),
    (1467, "応仁", "おうにん"),
    (1469, "文明", "ぶんめい"),
    (1487, "長享", "ちょうきょう"),
    (1489, "延徳", "えんとく"),
    (1492, "明応", "めいおう"),
    (1501, "文亀", "ぶんき"),
    (1504, "永正", "えいしょう"),
    (1521, "大永", "だいえい"),
    (1528, "享禄", "きょうろく"),
    (1532, "天文", "てんぶん"),
    (1555, "弘治", "こうじ"),
    (1558, "永禄", "えいろく"),
    (1570, "元亀", "げんき"),
    (1573, "天正", "てんしょう"),
    (1593, "文禄", "ぶんろく"),
    (1596, "慶長", "けいちょう"),
    (1615, "元和", "げんな"),
    (1624, "寛永", "かんえい"),
    (1645, "正保", "しょうほう"),
    (1648, "慶安", "けいあん"),
    (1652, "承応", "じょうおう"),
    (1655, "明暦", "めいれき"),
    (1658, "万治", "まんじ"),
    (1661, "寛文", "かんぶん"),
    (1673, "延宝", "えんぽう"),
    (1681, "天和", "てんな"),
    (1684, "貞享", "じょうきょう"),
    (1688, "元禄", "げんろく"),
    (1704, "宝永", "ほうえい"),
    (1711, "正徳", "しょうとく"),
    (1716, "享保", "きょうほう"),
    (1736, "元文", "げんぶん"),
    (1741, "寛保", "かんぽう"),
    (1744, "延享", "えんきょう"),
    (1748, "寛延", "かんえん"),
    (1751, "宝暦", "ほうれき"),
    (1764, "明和", "めいわ"),
    (1773, "安永", "あんえい"),
    (1781, "天明", "てんめい"),
    (1801, "寛政", "かんせい"),
    (1802, "享和", "きょうわ"),
    (1804, "文化", "ぶんか"),
    (1818, "文政", "ぶんせい"),
    (1831, "天保", "てんぽう"),
    (1845, "弘化", "こうか"),
    (1848, "嘉永", "かえい"),
    (1855, "安政", "あんせい"),
    (1860, "万延", "まんえい"),
    (1861, "文久", "ぶんきゅう"),
    (1864, "元治", "げんじ"),
    (1865, "慶応", "けいおう"),
    (1868, "明治", "めいじ"),
    (1912, "大正", "たいしょう"),
    (1926, "昭和", "しょうわ"),
    (1989, "平成", "へいせい"),
    (2019, "令和", "れいわ"),
];

/// `Num2Word_JA.CURRENCY_FORMS`, verbatim — all five codes, no inheritance.
///
/// **The two slots are `(kanji, reading)`, not `(singular, plural)`.** JA is
/// the odd one out: every other language stores a plural table that
/// `pluralize` indexes, whereas JA's second slot is what `reading=True`
/// selects. `to_currency` only ever reads slot 0 (`reading` is unreachable
/// through the trait), so the second slot looks dead — but
/// `Num2Word_Base.to_cheque` takes `cr1[-1]` believing it is a plural, which
/// is how "えん" reaches the JPY cheque output. Keep the 2-arity.
///
/// Nothing is inherited here: `Num2Word_JA` subclasses `Num2Word_Base`
/// directly, not `Num2Word_EUR`, so the shared dict that `Num2Word_EN.__init__`
/// mutates (adding AUD/CAD/CHF/KWD/... to EUR's table) never reaches JA. Every
/// code outside these five — including KWD, BHD, INR and CHF — raises
/// NotImplementedError, and the corpus freezes that.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    // Adding sen (1/100 yen) support — hence a 2-decimal JPY, not 0-decimal.
    m.insert("JPY", CurrencyForms::new(&["円", "えん"], &["銭", "銭"]));
    m.insert("EUR", CurrencyForms::new(&["ユーロ", "ユーロ"], &["セント", "セント"]));
    m.insert("USD", CurrencyForms::new(&["ドル", "ドル"], &["セント", "セント"]));
    m.insert("GBP", CurrencyForms::new(&["ポンド", "ポンド"], &["ペンス", "ペンス"]));
    // Chinese yuan with fen (1/100 yuan)
    m.insert("CNY", CurrencyForms::new(&["元", "元"], &["分", "分"]));
    m
}

/// `abs(Decimal(str(f)).as_tuple().exponent)` for an f64 — the number of
/// fractional digits in the shortest round-tripping decimal form.
///
/// `Num2Word_JA.to_cardinal_float` recomputes the precision from
/// `float(value)` (see the override), so a Decimal's own exponent is *not*
/// what is used: `Decimal("1.10")` yields precision 1 (from `repr(1.1)`),
/// not 2. Rust's `{}` for f64 is shortest-round-trip like Python's `repr`,
/// so counting the digits after the point matches for every reachable value.
/// (Rust never renders f64 in exponent form, so an extreme magnitude where
/// Python's `repr` switches to `1e21` would diverge — but no JA value reaches
/// that range, and for the common `FloatValue::Float` arm the carried,
/// Python-derived precision is used instead of this function.) This mirrors
/// the private `floatpath::float_repr_precision`, which is not re-exported.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
}

/// One kwargs-path card: the *unresolved* Python tuple entry, before
/// `select_text` collapses it. `kanji`/`kana` list the alternatives in
/// source order (first = the `text[0]` fallback).
struct KwCard {
    num: BigInt,
    kanji: Vec<&'static str>,
    kana: Vec<&'static str>,
}

/// `select_text(text, reading, prefer)` — reading picks the half, then a
/// multi-alternative half is disambiguated by `prefer`: Python takes
/// `set(text) & set(prefer)` and uses it only when the intersection has
/// exactly one element, else `text[0]`. The alternatives are distinct, so
/// counting which of them appear in `prefer` is the same set arithmetic.
fn select_text(card: &KwCard, reading: bool, prefer: &[String]) -> &'static str {
    let alts = if reading { &card.kana } else { &card.kanji };
    if alts.len() > 1 {
        let common: Vec<&'static str> = alts
            .iter()
            .copied()
            .filter(|a| prefer.iter().any(|p| p == a))
            .collect();
        if common.len() == 1 {
            return common[0];
        }
    }
    alts[0]
}

/// A node in the kwargs engine's `splitnum` tree — Python's tuple-vs-list.
#[derive(Clone)]
enum JaNode {
    Pair(String, BigInt),
    List(Vec<JaNode>),
}

/// `rendaku_merge_pairs`, in full. Only the kana path can satisfy the pair
/// literals; on the kanji path every rule misses and this is the plain
/// fall-through `("%s%s", lnum * rnum)` (which is what the fast path's
/// `merge` inlines). The `if lnum > rnum: raise ValueError` guard is not
/// ported: the sole caller only takes the `lnum < rnum` branch.
fn rendaku_merge_pairs(l: (String, BigInt), r: (String, BigInt)) -> (String, BigInt) {
    let (mut lt, ln) = l;
    let (mut rt, rn) = r;
    if rt == "ひゃく" && rn == BigInt::from(100) {
        if lt == "さん" && ln == BigInt::from(3) {
            rt = "びゃく".to_string();
        } else if lt == "ろく" && ln == BigInt::from(6) {
            lt = "ろっ".to_string();
            rt = "ぴゃく".to_string();
        } else if lt == "はち" && ln == BigInt::from(8) {
            lt = "はっ".to_string();
            rt = "ぴゃく".to_string();
        }
    } else if rt == "せん" && rn == BigInt::from(1000) {
        if lt == "さん" && ln == BigInt::from(3) {
            rt = "ぜん".to_string();
        } else if lt == "はち" && ln == BigInt::from(8) {
            lt = "はっ".to_string();
        }
    } else if rt == "ちょう" && rn == BigInt::from(10u64).pow(12) {
        if lt == "いち" && ln.is_one() {
            lt = "いっ".to_string();
        } else if lt == "はち" && ln == BigInt::from(8) {
            lt = "はっ".to_string();
        } else if lt == "じゅう" && ln == BigInt::from(10) {
            lt = "じゅっ".to_string();
        }
    } else if rt == "けい" && rn == BigInt::from(10u64).pow(16) {
        if lt == "いち" && ln.is_one() {
            lt = "いっ".to_string();
        } else if lt == "ろく" && ln == BigInt::from(6) {
            lt = "ろっ".to_string();
        } else if lt == "はち" && ln == BigInt::from(8) {
            lt = "はっ".to_string();
        } else if lt == "じゅう" && ln == BigInt::from(10) {
            lt = "じゅっ".to_string();
        } else if lt == "ひゃく" && ln == BigInt::from(100) {
            lt = "ひゃっ".to_string();
        }
    }
    (format!("{}{}", lt, rt), ln * rn)
}

/// The era binary search, shared by every `to_year` entry. `cmp(start)` is
/// `year <=> start` in whichever arithmetic the caller's year lives in
/// (BigInt / f64 / BigDecimal — Python compares the raw value against the
/// int table). `prefer` is the duplicate-start-year backtrack: Python guards
/// it with `if prefer:` (truthy), and a preferred name may match either the
/// kanji or the hiragana half.
///
/// Ported verbatim, including the quirk that first/last are still updated on
/// the iteration that sets era_idx — harmless, since the `while era_idx is
/// None` guard is re-checked immediately after. Signed indices mirror
/// Python: `last = mid - 1` could in principle reach -1 and wrap into
/// Python's negative indexing, but cannot here — that branch needs
/// `year < ERA_START[mid][0]`, which is false at mid == 0 because
/// `year >= 645 == ERA_START[0][0]` is enforced by every caller.
fn era_index(cmp: &dyn Fn(i64) -> Ordering, prefer: &[String]) -> usize {
    let last_era_idx = ERA_START.len() as i64 - 1;
    let mut first: i64 = 0;
    let mut last: i64 = last_era_idx;
    let mut era_idx: Option<i64> = None;
    while era_idx.is_none() {
        let mid = (first + last).div_euclid(2);
        let m = mid as usize;
        // `||` short-circuits exactly as Python's `or` does, so
        // ERA_START[mid + 1] is never touched when mid == last_era_idx.
        if mid == last_era_idx
            || (cmp(ERA_START[m].0) != Ordering::Less
                && cmp(ERA_START[m + 1].0) == Ordering::Less)
        {
            era_idx = Some(mid);
            // "if an era lasting less than a year is preferred, choose it"
            if !prefer.is_empty() {
                let mut i = mid - 1;
                while i >= 0 && cmp(ERA_START[i as usize].0) == Ordering::Equal {
                    let e = ERA_START[i as usize];
                    // set(ERA_START[i][1]) & set(prefer): non-empty wins.
                    if prefer.iter().any(|p| p == e.1 || p == e.2) {
                        era_idx = Some(i);
                        break;
                    }
                    i -= 1;
                }
            }
        }

        // Ends up at the last index where year >= ERA_START[mid][0]. On a
        // duplicate year the `else` arm keeps walking right, which is how
        // "use the last one" falls out.
        if cmp(ERA_START[m].0) == Ordering::Less {
            last = mid - 1;
        } else {
            first = mid + 1;
        }
    }
    era_idx.unwrap() as usize
}

/// The JA `reading=` kwarg, tri-state because `to_year` string-compares it:
/// `reading == "arabic"` takes the Arabic-era-year branch, any other truthy
/// value is the kana path, falsy (False/None/0/""/[]) is kanji.
#[derive(PartialEq)]
enum JaReading {
    Off,
    Kana,
    Arabic,
}

impl JaReading {
    /// Python truthiness — everywhere except the `== "arabic"` compare,
    /// "arabic" is just another truthy string.
    fn is_on(&self) -> bool {
        !matches!(self, JaReading::Off)
    }
}

/// Python truthiness of a kwarg value.
fn kw_truthy(v: &KwVal) -> bool {
    match v {
        KwVal::Bool(b) => *b,
        KwVal::Int(i) => *i != 0,
        KwVal::Str(s) => !s.is_empty(),
        KwVal::List(l) => !l.is_empty(),
        KwVal::None => false,
    }
}

fn parse_reading(kw: &Kwargs) -> JaReading {
    match kw.get("reading") {
        Option::None | Some(KwVal::None) => JaReading::Off,
        Some(KwVal::Str(s)) if s == "arabic" => JaReading::Arabic,
        Some(v) => {
            if kw_truthy(v) {
                JaReading::Kana
            } else {
                JaReading::Off
            }
        }
    }
}

/// `prefer=` as the item set Python builds. A list is itself; a string is
/// its characters (`set("〇")`); falsy scalars behave like `prefer or set()`
/// -> empty. A *truthy* non-iterable (True, 5) would raise TypeError only
/// if/when `set(prefer)` is evaluated inside `select_text` — too
/// value-dependent to model, so it defers to Python via NotImplemented.
fn parse_prefer(kw: &Kwargs) -> Result<Vec<String>> {
    match kw.get("prefer") {
        Option::None | Some(KwVal::None) => Ok(Vec::new()),
        Some(KwVal::List(l)) => Ok(l.clone()),
        Some(KwVal::Str(s)) => Ok(s.chars().map(String::from).collect()),
        Some(KwVal::Bool(false)) | Some(KwVal::Int(0)) => Ok(Vec::new()),
        Some(_) => Err(N2WError::Fallback("kwargs".into())),
    }
}

/// Python's `s[:-3]`, by characters (the …きゅう → …く year fix).
fn chop_last3(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let keep = chars.len().saturating_sub(3);
    chars[..keep].iter().collect()
}

/// `str(value)` for the verify_ordinal error messages, python-style. Only
/// the exception *type* is corpus-checked; the message mirrors the source's
/// `%s` interpolation as closely as Rust formatting allows.
fn py_value_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value: f, .. } => {
            if f.abs() >= 1e16 {
                // repr picks exponent form: 1e+16, 1.5e+20, ...
                let s = format!("{:e}", f);
                match s.split_once('e') {
                    Some((m, e)) if !e.starts_with('-') => format!("{}e+{}", m, e),
                    _ => s,
                }
            } else if f.fract() == 0.0 {
                format!("{:.1}", f) // "5.0", "-0.0"
            } else {
                format!("{}", f)
            }
        }
        FloatValue::Decimal { value: d, .. } => crate::strnum::python_decimal_str(d),
    }
}

pub struct LangJa {
    cards: Cards,
    maxval: BigInt,
    /// The unresolved card table the kwargs engine selects from at call time
    /// (descending, insertion order of Python's OrderedDict).
    kw_cards: Vec<KwCard>,
    /// Built once in `new()`. `to_currency` and `to_cheque` only read it, and
    /// rebuilding it per call is what made an earlier revision of this port
    /// slower than the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangJa {
    fn default() -> Self {
        Self::new()
    }
}

impl LangJa {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // setup(): high_numwords listed low->high, then `.reverse()`d.
        // Reversed order, which is what set_high_numwords consumes:
        let high = [
            "極", // 10**48 goku
            "載", // 10**44 sai
            "正", // 10**40 sei
            "澗", // 10**36 kan
            "溝", // 10**32 kō
            "穣", // 10**28 jō
            "秭", // 10**24 shi
            "垓", // 10**20 gai
            "京", // 10**16 kei
            "兆", // 10**12 chō
            "億", // 10**8 oku
            "万", // 10**4 man
        ];

        // JA's set_high_numwords: `max = 4 * len(high)` (= 48), then
        // `zip(high, range(max, 0, -4))` -> 48, 44, ..., 4. The zip is exact
        // (12 words, 12 exponents), so nothing is silently dropped.
        let mut n: u32 = 4 * high.len() as u32;
        for word in high.iter() {
            cards.insert(BigInt::from(10u8).pow(n), *word);
            n -= 4;
        }

        set_mid_numwords(&mut cards, &[(1000, "千"), (100, "百")]);

        // set_low_numwords maps these to 10, 9, ... 1, 0 by position.
        // "零" is select_text's resolution of (("零", "〇"), ("ゼロ", "れい"))
        // under reading=False/prefer=None; "七" and "四" likewise collapse
        // their (kana, kana) alternative halves away. See module docs.
        set_low_numwords(
            &mut cards,
            &[
                "十", // 10 jū
                "九", // 9 kyū
                "八", // 8 hachi
                "七", // 7 nana / shichi
                "六", // 6 roku
                "五", // 5 go
                "四", // 4 yon / shi
                "三", // 3 san
                "二", // 2 ni
                "一", // 1 ichi
                "零", // 0 ZERO / rei
            ],
        );

        // MAXVAL = 1000 * first-inserted card key = 1000 * 10**48 = 10**51.
        let maxval = cards.highest().cloned().unwrap() * BigInt::from(1000);

        // The unresolved table for the kwargs engine: same keys, same
        // descending order, but both halves of every Python tuple kept, with
        // the multi-alternative entries (0 kanji, 0/4/7 kana) intact.
        let mut kw_cards: Vec<KwCard> = Vec::new();
        let kw_high: [(u32, &str, &str); 12] = [
            (48, "極", "ごく"),
            (44, "載", "さい"),
            (40, "正", "せい"),
            (36, "澗", "かん"),
            (32, "溝", "こう"),
            (28, "穣", "じょう"),
            (24, "秭", "し"),
            (20, "垓", "がい"),
            (16, "京", "けい"),
            (12, "兆", "ちょう"),
            (8, "億", "おく"),
            (4, "万", "まん"),
        ];
        for (exp, kanji, kana) in kw_high {
            kw_cards.push(KwCard {
                num: BigInt::from(10u8).pow(exp),
                kanji: vec![kanji],
                kana: vec![kana],
            });
        }
        kw_cards.push(KwCard { num: BigInt::from(1000), kanji: vec!["千"], kana: vec!["せん"] });
        kw_cards.push(KwCard { num: BigInt::from(100), kanji: vec!["百"], kana: vec!["ひゃく"] });
        let kw_low: [(u32, Vec<&'static str>, Vec<&'static str>); 11] = [
            (10, vec!["十"], vec!["じゅう"]),
            (9, vec!["九"], vec!["きゅう"]),
            (8, vec!["八"], vec!["はち"]),
            (7, vec!["七"], vec!["なな", "しち"]),
            (6, vec!["六"], vec!["ろく"]),
            (5, vec!["五"], vec!["ご"]),
            (4, vec!["四"], vec!["よん", "し"]),
            (3, vec!["三"], vec!["さん"]),
            (2, vec!["二"], vec!["に"]),
            (1, vec!["一"], vec!["いち"]),
            (0, vec!["零", "〇"], vec!["ゼロ", "れい"]),
        ];
        for (n, kanji, kana) in kw_low {
            kw_cards.push(KwCard { num: BigInt::from(n), kanji, kana });
        }

        LangJa {
            cards,
            maxval,
            kw_cards,
            currency_forms: build_currency_forms(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check above it is vacuous for
    /// integer input, so only the negative guard survives.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `_ordinal_suffix(reading=False, counter="番")` -> `counter + "目"`.
    ///
    /// The `reading=True` arm raises `NotImplementedError` for any counter
    /// other than "番"; neither `reading` nor `counter` is reachable through
    /// the trait, so only the kanji default is modelled.
    fn ordinal_suffix(&self) -> &'static str {
        "番目"
    }

    /// `to_currency`'s fractional-cents arm: `self.to_cardinal_float(right / 100.0)`.
    ///
    /// This looks like it needs the float path, and does not. `right` arrives
    /// from `parse_currency_parts(..., keep_precision=False)` — JA never passes
    /// `keep_precision`, so `right` is `int(fraction * 100)` and is **always a
    /// whole 0..=99**, even on the branch Python calls "fractional cents".
    /// Which is the bug: `right / 100.0` divides the *already-rounded* whole
    /// cents by 100 again, so `2.567 EUR` renders "二ユーロ零点五七セント" —
    /// "two euro zero point five seven cents". Faithfully reproduced.
    ///
    /// Over that closed 100-value domain `Num2Word_JA.to_cardinal_float`
    /// reduces to a constant projection, so no `repr(float)`/`float2tuple`
    /// machinery is needed:
    ///
    ///   * `pre = int(right / 100.0)` is 0, so the head is always `to_cardinal(0)`.
    ///   * `precision = abs(Decimal(repr(right / 100.0)).as_tuple().exponent)`.
    ///     `repr` of a k/100 double is its shortest round-tripping form, i.e.
    ///     "0.0"/"0.1".."0.9" when `right` is a multiple of ten and
    ///     "0.01".."0.99" otherwise — so precision is 1 or 2, never 0, and the
    ///     `if self.precision:` guard on the pointword always passes.
    ///   * `post` is then `right / 10` or `right`, zero-padded to `precision`.
    ///   * The `if value < 0 and pre == 0` negword arm is unreachable: `right`
    ///     is non-negative, so `right / 100.0` never is.
    ///
    /// Checked against the interpreter for every `right` in 0..=99: exact match.
    /// Note JA's `to_cardinal_float` joins with `""` (Base joins with `" "`),
    /// which is why this builds the string by concatenation.
    fn cents_as_hundredths(&self, right: &BigInt) -> Result<String> {
        let r = match right.to_u32() {
            Some(r) if r <= 99 => r,
            // parse_currency_parts with divisor=100 and keep_precision=false
            // cannot produce anything else; see the doc comment above.
            _ => unreachable!("JA cents out of the 0..=99 domain: {}", right),
        };
        let (precision, post) = if r % 10 == 0 { (1usize, r / 10) } else { (2usize, r) };

        let mut out = self.to_cardinal(&BigInt::zero())?;
        // title() is the identity for JA; pointword() is ("点", "てん")[0].
        out.push_str(self.pointword());
        for ch in crate::strnum::zero_pad_left(&post.to_string(), precision).chars() {
            let digit = BigInt::from(ch.to_digit(10).unwrap());
            out.push_str(&self.to_cardinal(&digit)?);
        }
        Ok(out)
    }

    // ---- kwargs engine ----------------------------------------------------
    //
    // A verbatim port of Python's `Num2Word_JA.splitnum` +
    // `Num2Word_Base.clean` over the unresolved card table, threading
    // `reading`/`prefer` through every `select_text`. With
    // `reading=False, prefer=[]` it produces exactly what the pre-resolved
    // fast path (`to_cardinal` above) does; the plain trait methods keep the
    // fast path, and only the `*_kw` hooks come through here.

    /// `self.cards[1]`, unresolved.
    fn kw_card_one(&self) -> &KwCard {
        self.kw_cards.iter().find(|c| c.num.is_one()).unwrap()
    }

    /// `Num2Word_JA.splitnum(value, reading, prefer)`.
    fn splitnum_kw(&self, value: &BigInt, reading: bool, prefer: &[String]) -> Vec<JaNode> {
        for card in &self.kw_cards {
            if card.num > *value {
                continue;
            }
            // Card 0 is only selected when value == 0, which takes the
            // special (1, 0) divmod — so the division below never sees a
            // zero card.
            let (div, mod_) = if value.is_zero() {
                (BigInt::one(), BigInt::zero())
            } else {
                (value / &card.num, value % &card.num)
            };

            let mut out: Vec<JaNode> = Vec::new();
            if div.is_one() {
                out.push(JaNode::Pair(
                    select_text(self.kw_card_one(), reading, prefer).to_string(),
                    BigInt::one(),
                ));
            } else if &div == value {
                // The tally branch ("eg Roman Numerals"). Unreachable for
                // JA's dense card table (it needs elem == 1 with value > 1,
                // but every value >= 2 matches a card >= 2 first); ported
                // for shape.
                let n = div.to_usize().expect("JA tally count");
                return vec![JaNode::Pair(
                    select_text(card, reading, prefer).repeat(n),
                    div * &card.num,
                )];
            } else {
                out.push(JaNode::List(self.splitnum_kw(&div, reading, prefer)));
            }

            out.push(JaNode::Pair(
                select_text(card, reading, prefer).to_string(),
                card.num.clone(),
            ));

            if !mod_.is_zero() {
                out.push(JaNode::List(self.splitnum_kw(&mod_, reading, prefer)));
            }
            return out;
        }
        unreachable!("JA splitnum_kw: no card <= {}", value)
    }

    /// `Num2Word_JA.merge` with `rendaku_merge_pairs` live (kana pairs can
    /// satisfy its literals). Same three arms as the fast path's `merge`,
    /// same unreachable `lnum == rnum` fall-off.
    fn merge_kw(&self, l: (String, BigInt), r: (String, BigInt)) -> (String, BigInt) {
        let (lt, ln) = l;
        let (rt, rn) = r;
        if ln.is_one() && rn < BigInt::from(10000) {
            return (rt, rn);
        }
        match ln.cmp(&rn) {
            Ordering::Greater => (format!("{}{}", lt, rt), ln + rn),
            Ordering::Less => rendaku_merge_pairs((lt, ln), (rt, rn)),
            Ordering::Equal => unreachable!(
                "JA merge_kw: lnum == rnum ({}) is unreachable; Python returns None here",
                ln
            ),
        }
    }

    /// `Num2Word_Base.clean`, over `JaNode`s and `merge_kw`.
    fn clean_kw(&self, mut val: Vec<JaNode>) -> (String, BigInt) {
        while val.len() != 1 {
            let first_two_pairs =
                matches!(&val[0], JaNode::Pair(..)) && matches!(&val[1], JaNode::Pair(..));
            let mut out: Vec<JaNode> = Vec::new();
            if first_two_pairs {
                let mut it = val.into_iter();
                let (a, b) = (it.next().unwrap(), it.next().unwrap());
                if let (JaNode::Pair(lt, ln), JaNode::Pair(rt, rn)) = (a, b) {
                    let (t, n) = self.merge_kw((lt, ln), (rt, rn));
                    out.push(JaNode::Pair(t, n));
                } else {
                    unreachable!()
                }
                // `if val[2:]: out.append(val[2:])` — the rest, as ONE
                // nested list.
                let rest: Vec<JaNode> = it.collect();
                if !rest.is_empty() {
                    out.push(JaNode::List(rest));
                }
            } else {
                for elem in val {
                    match elem {
                        JaNode::List(l) => {
                            if l.len() == 1 {
                                out.push(l.into_iter().next().unwrap());
                            } else {
                                let (t, n) = self.clean_kw(l);
                                out.push(JaNode::Pair(t, n));
                            }
                        }
                        pair => out.push(pair),
                    }
                }
            }
            val = out;
        }
        match val.into_iter().next().unwrap() {
            JaNode::Pair(t, n) => (t, n),
            // splitnum never returns a 1-element list, so the single
            // survivor is always a pair; kept total for safety.
            JaNode::List(l) => self.clean_kw(l),
        }
    }

    /// `Num2Word_JA.to_cardinal(value, reading, prefer)`, integer arm.
    fn cardinal_kw_int(&self, value: &BigInt, reading: bool, prefer: &[String]) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = self.negword().to_string(); // verbatim, no space
        }
        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v,
                self.maxval()
            )));
        }
        let tree = self.splitnum_kw(&v, reading, prefer);
        let (words, _) = self.clean_kw(tree);
        // title() is the identity for JA.
        Ok(format!("{}{}", out, words))
    }

    /// `Num2Word_JA.to_cardinal_float(value, reading, prefer)` — the same
    /// float-casting/precision semantics as the plain `to_cardinal_float`
    /// override below, plus the `prefer = prefer or ["れい"]` default, which
    /// is inert on the kanji side (れい matches no kanji alternative) but
    /// makes a kana fractional digit 0 read れい instead of ゼロ.
    fn cardinal_float_impl(
        &self,
        value: &FloatValue,
        reading: bool,
        prefer: &[String],
    ) -> Result<String> {
        let eff_prefer: Vec<String> = if prefer.is_empty() {
            vec!["れい".to_string()]
        } else {
            prefer.to_vec()
        };
        let (f, precision) = match value {
            FloatValue::Float { value, precision } => (*value, *precision),
            FloatValue::Decimal { value, .. } => {
                let f = value.to_f64().ok_or_else(|| {
                    N2WError::Value(format!("cannot represent {} as f64", value))
                })?;
                (f, float_repr_precision(f))
            }
        };
        let (pre, post) = float2tuple(&FloatValue::Float { value: f, precision });

        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        let mut out = vec![self.cardinal_kw_int(&pre, reading, &eff_prefer)?];
        if value.is_negative() && pre.is_zero() {
            out.insert(0, self.negword().to_string());
        }
        if precision > 0 {
            // pointword = ("点", "てん")[1 if reading else 0].
            out.push((if reading { "てん" } else { "点" }).to_string());
        }
        for ch in post_str.chars().take(precision as usize) {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("non-digit {:?} in fractional part", ch))
            })?;
            out.push(self.cardinal_kw_int(&BigInt::from(d), reading, &eff_prefer)?);
        }
        Ok(out.join(""))
    }

    /// [`Self::cents_as_hundredths`] with `reading`/`prefer` threaded — the
    /// kwargs twin, same closed 0..=99 domain, same re-divided-cents bug.
    fn cents_as_hundredths_kw(
        &self,
        right: &BigInt,
        reading: bool,
        prefer: &[String],
    ) -> Result<String> {
        let r = match right.to_u32() {
            Some(r) if r <= 99 => r,
            _ => unreachable!("JA cents out of the 0..=99 domain: {}", right),
        };
        let (precision, post) = if r % 10 == 0 { (1usize, r / 10) } else { (2usize, r) };
        let eff_prefer: Vec<String> = if prefer.is_empty() {
            vec!["れい".to_string()]
        } else {
            prefer.to_vec()
        };
        let mut out = self.cardinal_kw_int(&BigInt::zero(), reading, &eff_prefer)?;
        out.push_str(if reading { "てん" } else { "点" });
        for ch in crate::strnum::zero_pad_left(&post.to_string(), precision).chars() {
            let digit = BigInt::from(ch.to_digit(10).unwrap());
            out.push_str(&self.cardinal_kw_int(&digit, reading, &eff_prefer)?);
        }
        Ok(out)
    }

    /// `_ordinal_suffix(reading, counter)`, kwargs-visible form.
    ///
    /// The genuine Python `NotImplementedError` (reading with a counter other
    /// than 番) is raised as a `Custom` builtins.NotImplementedError rather
    /// than `N2WError::NotImplemented`: the latter is this codebase's "hook
    /// not ported" marker and the two deserve distinct spellings even though
    /// they map to the same Python class at the boundary. Note the wrapper's
    /// `except NotImplementedError` cannot tell them apart, so it still falls
    /// back to Python for these rows — which re-raises the identical error,
    /// keeping the observable behaviour byte-for-byte.
    fn ordinal_suffix_kw(&self, reading: bool, counter: &str) -> Result<String> {
        if reading {
            if counter == "番" {
                Ok("ばんめ".to_string())
            } else {
                Err(N2WError::Custom {
                    module: "builtins",
                    class: "NotImplementedError",
                    msg: format!("Reading not implemented for {}", counter),
                })
            }
        } else {
            Ok(format!("{}目", counter))
        }
    }

    /// `to_year`'s era branch for an integer year, all reading modes.
    ///
    /// The kanji arm calls the fast-path `to_cardinal`: `prefer` cannot
    /// change a kanji cardinal for any nonzero value (the only kanji
    /// alternative is at 0, and splitnum emits the zero card only for the
    /// value 0 itself — era years are >= 1), so this is exactly Python's
    /// `to_cardinal(era_year, reading=False, prefer=prefer)`.
    fn year_era_int(&self, year: &BigInt, reading: &JaReading, prefer: &[String]) -> Result<String> {
        let min_year = ERA_START[0].0; // 645
        if *year < BigInt::from(min_year) {
            return Err(N2WError::Value(format!(
                "Can't convert years less than {} to era",
                min_year
            )));
        }
        let idx = era_index(&|s| year.cmp(&BigInt::from(s)), prefer);
        let era = ERA_START[idx];
        let era_year = year - BigInt::from(era.0) + BigInt::one();
        match reading {
            // `reading == "arabic"`: kanji era name, Arabic era year, no
            // 元/がん special case.
            JaReading::Arabic => Ok(format!("{}{}年", era.1, era_year)),
            JaReading::Kana => {
                let mut words = if era_year.is_one() {
                    "がん".to_string()
                } else {
                    self.cardinal_kw_int(&era_year, true, prefer)?
                };
                // …きゅう (3 chars) -> …く for years ending in 9.
                if &era_year % BigInt::from(10) == BigInt::from(9) {
                    words = format!("{}く", chop_last3(&words));
                }
                Ok(format!("{}{}ねん", era.2, words))
            }
            JaReading::Off => {
                let words = if era_year.is_one() {
                    "元".to_string()
                } else {
                    self.to_cardinal(&era_year)?
                };
                Ok(format!("{}{}年", era.1, words))
            }
        }
    }
}

impl Lang for LangJa {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "JPY"
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
        // NB: no trailing space, and to_cardinal below must not add one.
        "マイナス"
    }

    fn pointword(&self) -> &str {
        // pointword = ("点", "てん"); reading=False selects index 0.
        // Float paths are out of scope; carried for completeness.
        "点"
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let myriad = BigInt::from(10000);

        if lnum.is_one() && rnum < &myriad {
            // "ignore lpair if lnum is 1 and rnum is less than 10000"
            // -> 1000 renders "千", but 10000 renders "一万".
            (rtext.to_string(), rnum.clone())
        } else if lnum > rnum {
            // rnum is added to lnum
            (format!("{}{}", ltext, rtext), lnum + rnum)
        } else if lnum < rnum {
            // rnum is multiplied by lnum. Python delegates to
            // rendaku_merge_pairs, whose every rule is gated on hiragana
            // literals and so cannot fire on the kanji path; this is its
            // fall-through. See module docs.
            (format!("{}{}", ltext, rtext), lnum * rnum)
        } else {
            // lnum == rnum: Python falls off the end of `merge` and returns
            // None, which `clean` would then spin on forever. Unreachable:
            // splitnum only ever hands `clean` a (div, elem) pair with
            // div < elem (elem is the largest card <= value, and adjacent
            // cards differ by a factor of at least 10), or an
            // (accumulated, mod) pair with mod < elem <= accumulated.
            // Confirmed by instrumenting the Python original over 12,060
            // values: zero hits.
            unreachable!(
                "JA merge: lnum == rnum ({}) is unreachable; Python returns None here",
                lnum
            )
        }
    }

    /// `Num2Word_JA.to_cardinal`. Identical to the base implementation except
    /// the negative prefix is `self.negword` verbatim (no strip + space).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut out = String::new();
        let mut v = value.clone();
        if v.is_negative() {
            v = v.abs();
            out = self.negword().to_string();
        }

        if &v >= self.maxval() {
            return Err(N2WError::Overflow(format!(
                "abs({}) must be less than {}.",
                v,
                self.maxval()
            )));
        }

        // splitnum cannot return None here: v >= 0 and cards contains 0, so
        // the scan always finds a card <= v. Mapped to Overflow to match the
        // base engine's handling rather than panicking.
        let tree = splitnum(self, &v).ok_or_else(|| {
            N2WError::Overflow(format!("abs({}) must be less than {}.", v, self.maxval()))
        })?;
        let words = match clean(self, tree) {
            Node::Leaf(t, _) => t,
            Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
        };
        // is_title is False for JA, so title() is the identity; kept for parity.
        Ok(self.title(&format!("{}{}", out, words)))
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let base = self.to_cardinal(value)?;
        Ok(format!("{}{}", base, self.ordinal_suffix()))
    }

    /// Note: no `verify_ordinal`, and no `to_cardinal`. Python interpolates the
    /// raw value with `"%s%s"`, so negatives survive as "-1番目" and there is
    /// no MAXVAL ceiling.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(format!("{}{}", value, self.ordinal_suffix()))
    }

    /// `to_year(val, era=True, reading=False, prefer=None)` — the era
    /// calendar at its defaults. The Gregorian (`era=False`), kana and
    /// `"arabic"` branches, and the `prefer` backtrack in the search, are
    /// reachable only through [`Lang::to_year_kw`] below; the search itself
    /// lives in [`era_index`]. Termination and in-range `mid` were verified
    /// against the interpreter for every year in 645..=12000.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.year_era_int(value, &JaReading::Off, &[])
    }

    // ---- float / Decimal entries -----------------------------------------

    /// `to_ordinal(float/Decimal)`: `Num2Word_Base.verify_ordinal` runs on
    /// the raw value — non-whole raises the float TypeError, whole-negative
    /// the negative one, and a whole non-negative value takes the integer
    /// cardinal + 番目. `-0.0` (float or Decimal) passes BOTH checks
    /// (`int(-0.0) == -0.0` and `abs(-0.0) == -0.0`), so it is 零番目, not
    /// an error — which is why this must not use the sign-bit-aware
    /// `FloatValue::is_negative`.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            Option::None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                py_value_str(value)
            ))),
            Some(i) => {
                if i.is_negative() {
                    return Err(N2WError::Type(format!(
                        "Cannot treat negative num {} as ordinal.",
                        py_value_str(value)
                    )));
                }
                Ok(format!("{}{}", self.to_cardinal(&i)?, self.ordinal_suffix()))
            }
        }
    }

    /// `to_ordinal_num(float/Decimal)`: `"%s%s" % (value, "番目")` — the raw
    /// `str(value)`, no verify_ordinal, no cardinal. `"-0.0番目"`,
    /// `"1e+16番目"` and `"5.00番目"` are all real.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(format!("{}{}", repr_str, self.ordinal_suffix()))
    }

    /// `to_year(float/Decimal)`: the era search runs on the raw number, so
    /// everything below 645 — negatives, ±0.0, fractions — raises the same
    /// ValueError as an integer year would. `era_year = year - start + 1`
    /// stays in the value's own arithmetic: two chained f64 ops for a float
    /// (`to_year(1e16)` really is 令和…七千九百八十年, the double-rounded
    /// `1e16 - 2019 + 1`; `1e20 - 2018` rounds back to 1e20 → 令和一垓年),
    /// exact BigDecimal for a Decimal (`1E+20` → …七千九百八十二年). A whole
    /// era year takes the integer cardinal; a fractional one (year 1000.5)
    /// goes through JA's float grammar, matching `to_cardinal`'s own
    /// routing.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        value.reject_non_finite()?; // #204: no panic on NaN/inf
        let min_year = ERA_START[0].0; // 645
        let too_early = || {
            N2WError::Value(format!("Can't convert years less than {} to era", min_year))
        };
        match value {
            FloatValue::Float { value: f, .. } => {
                let f = *f;
                if f < min_year as f64 {
                    return Err(too_early());
                }
                // NaN/±inf never reach the float entries (the dispatcher
                // keeps them on the Python side), so partial_cmp is total.
                let idx = era_index(
                    &|s| f.partial_cmp(&(s as f64)).expect("finite year"),
                    &[],
                );
                let era = ERA_START[idx];
                // Python evaluates `year - era[0] + 1` left to right in f64.
                let era_year = f - era.0 as f64 + 1.0;
                let words = if era_year == 1.0 {
                    "元".to_string()
                } else {
                    let fv = FloatValue::Float {
                        value: era_year,
                        precision: float_repr_precision(era_year),
                    };
                    match fv.as_whole_int() {
                        Some(i) => self.to_cardinal(&i)?,
                        Option::None => self.to_cardinal_float(&fv, None)?,
                    }
                };
                Ok(format!("{}{}年", era.1, words))
            }
            FloatValue::Decimal { value: d, .. } => {
                if *d < BigDecimal::from(min_year) {
                    return Err(too_early());
                }
                let idx = era_index(&|s| d.cmp(&BigDecimal::from(s)), &[]);
                let era = ERA_START[idx];
                let era_year = d - BigDecimal::from(era.0) + BigDecimal::from(1);
                let words = if era_year == BigDecimal::from(1) {
                    "元".to_string()
                } else {
                    let fv = FloatValue::Decimal { value: era_year, precision: 0 };
                    match fv.as_whole_int() {
                        Some(i) => self.to_cardinal(&i)?,
                        // JA's to_cardinal_float float-casts the Decimal and
                        // recomputes the precision from the cast double, so
                        // the carried 0 is never read.
                        Option::None => self.to_cardinal_float(&fv, None)?,
                    }
                };
                Ok(format!("{}{}年", era.1, words))
            }
        }
    }

    // ---- grammatical kwargs -----------------------------------------------

    /// `to_cardinal(value, reading=False, prefer=None)`.
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_cardinal(value);
        }
        if !kw.only(&["reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        self.cardinal_kw_int(value, reading.is_on(), &prefer)
    }

    /// `to_ordinal(value, reading=False, prefer=None, counter="番")`.
    /// verify_ordinal runs FIRST — a negative raises TypeError even when
    /// the counter would raise NotImplementedError.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal(value);
        }
        if !kw.only(&["reading", "prefer", "counter"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        let counter = match kw.get("counter") {
            Option::None => "番",
            Some(KwVal::Str(s)) => s.as_str(),
            // A non-str counter TypeErrors in Python only after the
            // verify/cardinal work; defer the whole call.
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        self.verify_ordinal(value)?;
        let base = self.cardinal_kw_int(value, reading.is_on(), &prefer)?;
        let suffix = self.ordinal_suffix_kw(reading.is_on(), counter)?;
        Ok(format!("{}{}", base, suffix))
    }

    /// `to_ordinal_num(value, reading=False, counter="番")` — note: NO
    /// `prefer` in this signature, and still no verify_ordinal ("-5個目").
    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal_num(value);
        }
        if !kw.only(&["reading", "counter"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let counter = match kw.get("counter") {
            Option::None => "番",
            Some(KwVal::Str(s)) => s.as_str(),
            Some(_) => return Err(N2WError::Fallback("kwargs".into())),
        };
        let suffix = self.ordinal_suffix_kw(reading.is_on(), counter)?;
        Ok(format!("{}{}", value, suffix))
    }

    /// `to_year(val, suffix=None, longval=True, reading=False, prefer=None,
    /// era=True)`. `suffix` and `longval` are accepted and never read (any
    /// value passes). An explicit `era=None` is falsy — `if not era:` — and
    /// takes the Gregorian branch, unlike most None-means-default kwargs.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_year(value);
        }
        if !kw.only(&["suffix", "longval", "reading", "prefer", "era"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        let era = match kw.get("era") {
            Option::None => true,
            Some(v) => kw_truthy(v),
        };

        if !era {
            // Gregorian calendar: 紀元前/きげんぜん for BCE, and on the
            // reading path a year ending in 9 swaps …きゅう for …く.
            // The kana cardinal is used for ANY truthy reading, "arabic"
            // included — only the era branch string-compares it.
            let mut year = value.clone();
            let mut prefix = "";
            if year.is_negative() {
                year = -year;
                prefix = if reading.is_on() { "きげんぜん" } else { "紀元前" };
            }
            let mut words = self.cardinal_kw_int(&year, reading.is_on(), &prefer)?;
            if reading.is_on() && &year % BigInt::from(10) == BigInt::from(9) {
                words = format!("{}く", chop_last3(&words));
            }
            return Ok(format!(
                "{}{}{}",
                prefix,
                words,
                if reading.is_on() { "ねん" } else { "年" }
            ));
        }

        self.year_era_int(value, &reading, &prefer)
    }

    /// `to_cardinal(float/Decimal, reading=..., prefer=...)` — the full
    /// entry: `assert int(value) == value` sends a whole value down the
    /// integer path with the same kwargs.
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.cardinal_float_entry(value, precision_override);
        }
        if !kw.only(&["reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw);
        let prefer = parse_prefer(kw)?;
        if let Some(i) = value.as_whole_int() {
            return self.cardinal_kw_int(&i, reading.is_on(), &prefer);
        }
        // precision_override dropped: JA's signature has no `precision=`.
        self.cardinal_float_impl(value, reading.is_on(), &prefer)
    }

    /// `to_currency(..., reading=False, prefer=None)`. A structural copy of
    /// [`Lang::to_currency`] below with the two kwargs threaded — kept
    /// separate so the frozen non-kwargs path stays byte-identical. Slot 1
    /// of CURRENCY_FORMS (the hiragana reading) replaces slot 0, the
    /// cardinals go kana, but `minus_str` stays `self.negword` verbatim:
    /// `-5 JPY, reading=True` → "マイナスごえん".
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.to_currency(val, currency, cents, separator, adjective);
        }
        if !kw.only(&["reading", "prefer"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        let reading = parse_reading(kw).is_on();
        let prefer = parse_prefer(kw)?;
        let ri = if reading { 1 } else { 0 };

        // From here on this mirrors to_currency; see its comments.
        let decimal_val = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();
        let as_float = CurrencyValue::Decimal {
            value: decimal_val,
            has_decimal: false,
            is_float: true,
        };
        let (left, right, is_negative) = parse_currency_parts(&as_float, cents, false, 100);
        let right = right.as_bigint_and_exponent().0;

        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let mut cr1 = forms.unit.clone();
        let cr2 = &forms.subunit;

        if adjective {
            if let Some(adj) = self.currency_adjective(currency) {
                cr1 = prefix_currency(adj, &cr1);
            }
        }

        let minus_str = if is_negative { self.negword() } else { "" };

        let (right_str, cr2_str) = if has_fractional_cents && !cr2.is_empty() {
            (
                self.cents_as_hundredths_kw(&right, reading, &prefer)?,
                cr2[ri].as_str(),
            )
        } else if !cr2.is_empty() && !right.is_zero() {
            (
                self.cardinal_kw_int(&right, reading, &prefer)?,
                cr2[ri].as_str(),
            )
        } else {
            (String::new(), "")
        };

        Ok(format!(
            "{}{}{}{}{}",
            minus_str,
            self.cardinal_kw_int(&left, reading, &prefer)?,
            cr1[ri],
            right_str,
            cr2_str,
        ))
    }

    /// `Num2Word_JA.to_cardinal_float`. Two deliberate divergences from
    /// `Num2Word_Base.to_cardinal_float` (i.e. from `default_to_cardinal_float`):
    ///
    /// 1. **It float-casts everything.** Python does
    ///    `pre, post = self.float2tuple(float(value))` unconditionally — it
    ///    never takes Base's Decimal-preserving arm. So a `Decimal` goes
    ///    through the lossy binary path too, and the precision is recomputed
    ///    from `repr(float(value))` rather than the Decimal's own exponent.
    ///    This is load-bearing: `Decimal("1.10") -> "一点一"` (precision 1 from
    ///    `repr(1.1)`, **not** "一点一零"), and
    ///    `Decimal("98746251323029.99") -> "…点九八"` — the issue-#603 silent
    ///    trillion-scale rounding, which is the *correct* JA output, not a bug
    ///    to repair.
    /// 2. **It joins with `""`**, where Base joins with `" "`.
    ///
    /// The `negword.strip()` / `pointword[0]` / `value < 0 and pre == 0`
    /// details are identical to Base once resolved: JA's `negword` "マイナス"
    /// has no spaces to strip, its `pointword` slot 0 is "点", and `title()` is
    /// the identity (JA never sets `is_title`).
    ///
    /// JA's signature is `(value, reading=False, prefer=None)` — no `precision=`
    /// kwarg — so `precision_override` is dropped, matching the interpreter:
    /// `num2words(1.5, lang='ja', precision=3) == "一点五"`. The `prefer` default
    /// `["れい"]` is inert on the `reading=False` path (it is a hiragana reading,
    /// absent from every kanji card tuple), so digit 0 still resolves to "零".
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // `float(value)`: identity for a float, lossy binary cast for a
        // Decimal. `precision = abs(Decimal(str(f)).as_tuple().exponent)`.
        // For the `Float` arm the precision was already derived from
        // `repr(value)` on the Python side (the binding computed it the same
        // way JA would), so it is used as-is; only the `Decimal` arm needs the
        // recompute from the cast double.
        let (f, precision) = match value {
            FloatValue::Float { value, precision } => (*value, *precision),
            FloatValue::Decimal { value, .. } => {
                let f = value.to_f64().ok_or_else(|| {
                    N2WError::Value(format!("cannot represent {} as f64", value))
                })?;
                (f, float_repr_precision(f))
            }
        };

        let (pre, post) = float2tuple(&FloatValue::Float { value: f, precision });

        // post = str(post); post = "0" * (precision - len(post)) + post
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        let mut out = vec![self.to_cardinal(&pre)?];
        // `if value < 0 and pre == 0`: the sign is otherwise lost because
        // int(-0.5) == 0. `self.negword.strip()` == "マイナス" (nothing to strip).
        if value.is_negative() && pre.is_zero() {
            out.insert(0, self.negword().to_string());
        }
        // `if self.precision:` — pointword slot 0 is "点"; title() is identity.
        if precision > 0 {
            out.push(self.title(self.pointword()));
        }
        for ch in post_str.chars().take(precision as usize) {
            let d = ch.to_digit(10).ok_or_else(|| {
                N2WError::Value(format!("non-digit {:?} in fractional part", ch))
            })?;
            out.push(self.to_cardinal(&BigInt::from(d))?);
        }
        // JA joins with "" (Base joins with " ").
        Ok(out.join(""))
    }

    // ---- currency -------------------------------------------------------
    //
    // JA overrides `to_currency` outright; see the module docs for the full
    // list of ways it diverges from `Num2Word_Base.to_currency`. Everything
    // else on the currency surface is left at the trait default *because the
    // default is already right*:
    //
    //   * `currency_precision` — JA's CURRENCY_PRECISION is `{}`, so
    //     `.get(code, 100)` is a constant 100, exactly the trait default.
    //     Overriding it would be a no-op at best and a JPY regression at worst.
    //   * `currency_adjective` — CURRENCY_ADJECTIVES is `{}` -> always None.
    //   * `pluralize` — unreachable; JA indexes `cr1[0]` itself.
    //   * `money_verbose` / `cents_verbose` / `cents_terse` — JA's
    //     `to_currency` calls none of them. `default_to_cheque` does call
    //     `money_verbose`, whose default (`self.to_cardinal`) dispatches back
    //     to JA's override, which is what Python's `_money_verbose` does too.
    //   * `to_cheque` — inherited unchanged from Num2Word_Base.
    //   * `cardinal_from_decimal` — JA's fractional-cents arm goes through its
    //     own `to_cardinal_float`, not through `_cents_verbose`, so this hook
    //     is never reached. Left alone.

    fn lang_name(&self) -> &str {
        "Num2Word_JA"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    /// `Num2Word_JA.to_currency`.
    ///
    /// `separator` is accepted and never read — that is the Python signature,
    /// not an oversight here (`separator="XX"` changes nothing upstream). The
    /// output is a bare `"%s%s%s%s%s"` concatenation with no spaces at all.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        _separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // `decimal_val = Decimal(str(val))` — the *original* value, captured
        // before the int -> float coercion below.
        let decimal_val = match val {
            CurrencyValue::Int(i) => BigDecimal::from(i.clone()),
            CurrencyValue::Decimal { value, .. } => value.clone(),
        };

        // `has_fractional_cents = (decimal_val * 100) % 1 != 0`. The 100 is
        // JA's own literal, not currency_precision(currency) — JA hardcodes it
        // and ignores CURRENCY_PRECISION entirely. Testing "is the scaled
        // value an integer" is sign-agnostic, so it does not matter whether
        // with_scale(0) truncates or rounds.
        let scaled = &decimal_val * BigDecimal::from(100);
        let has_fractional_cents = &scaled - scaled.with_scale(0) != BigDecimal::zero();

        // `if isinstance(val, int): val = float(val)`. Load-bearing: it forces
        // parse_currency_parts down its non-int branch, the only one that
        // never reads is_int_with_cents. Passing the Int through instead would
        // let `cents=true` split it into units+cents and render 1000000 EUR as
        // "一万ユーロ" rather than "百万ユーロ".
        //
        // has_decimal is irrelevant here — JA has no `has_decimal` guard, which
        // is why the float 1.0 prints no cents, unlike Base.
        let as_float = CurrencyValue::Decimal {
            value: decimal_val,
            has_decimal: false,
            is_float: true,
        };

        // is_int_with_cents=cents mirrors the Python call but is inert: the
        // value is a float by now. divisor=100 / keep_precision=false are
        // parse_currency_parts' own defaults, which JA never overrides — so
        // `right` is whole cents, 0..=99, on every path.
        let (left, right, is_negative) = parse_currency_parts(&as_float, cents, false, 100);
        let right = right.as_bigint_and_exponent().0;

        // Python looks the currency up *after* parsing, inside a try/except
        // KeyError. Order is only observable when parsing itself raises, which
        // it cannot here; kept anyway.
        let forms = self.currency_forms.get(currency).ok_or_else(|| {
            N2WError::NotImplemented(format!(
                "Currency code \"{}\" not implemented for \"{}\"",
                currency,
                self.lang_name()
            ))
        })?;
        let mut cr1 = forms.unit.clone();
        let cr2 = &forms.subunit;
        // `if (cents or abs(val) != left) and not cr2: raise ValueError(...)`
        // is unreachable — every cr2 in JA's table is a non-empty tuple — so
        // no ValueError arm is modelled. Note it sits *inside* the try that
        // only catches KeyError, so it would escape as a real ValueError.

        // Dead for JA (CURRENCY_ADJECTIVES is empty, so currency_adjective is
        // always None), but ported because it is in the source.
        if adjective {
            if let Some(adj) = self.currency_adjective(currency) {
                cr1 = prefix_currency(adj, &cr1);
            }
        }

        // `self.negword` verbatim — NOT Base's `"%s " % negword.strip()`.
        let minus_str = if is_negative { self.negword() } else { "" };

        // cr1/cr2 slot 0 is the kanji: `cr1[1] if reading else cr1[0]`, and
        // reading is unreachable through the trait.
        let (right_str, cr2_str) = if has_fractional_cents && !cr2.is_empty() {
            // NB: cr2_str is unconditional on this arm — Python gates it on
            // `if cr2 else ""`, not on `right` — so 0.001 EUR still emits the
            // subunit: "零ユーロ零点零セント".
            (self.cents_as_hundredths(&right)?, cr2[0].as_str())
        } else if !cr2.is_empty() && !right.is_zero() {
            (self.to_cardinal(&right)?, cr2[0].as_str())
        } else {
            (String::new(), "")
        };

        Ok(format!(
            "{}{}{}{}{}",
            minus_str,
            self.to_cardinal(&left)?,
            cr1[0],
            right_str,
            cr2_str,
        ))
    }
}

#[allow(clippy::approx_constant)] // 3.14-style literals are test inputs, not π
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Drive a corpus row exactly as `num2words2-py`'s binding does, so the
    /// argument literal decides int-vs-float the way `repr(value)` did on the
    /// Python side: `100` is an int, `1.0` a float, and the two take different
    /// paths through `parse_currency_parts`. `cents=true` / `separator=None` /
    /// `adjective=false` mirror `bench/diff_test.py`; for JA all three are
    /// inert (see module docs and `inert_kwargs`).
    fn run_currency(arg: &str, code: &str) -> Result<String> {
        let is_int = !arg.contains('.') && !arg.to_lowercase().contains('e');
        let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
        LangJa::new().to_currency(&v, code, true, None, false)
    }

    /// The corpus records failures by Python exception name only.
    fn err_name(e: &N2WError) -> &'static str {
        match e {
            N2WError::NotImplemented(_) => "NotImplementedError",
            N2WError::Overflow(_) => "OverflowError",
            N2WError::Type(_) => "TypeError",
            N2WError::Value(_) => "ValueError",
            N2WError::Index(_) => "IndexError",
            N2WError::Key(_) => "KeyError",
            _ => "OtherError",
        }
    }

    /// Every `ja` currency row in the frozen corpus, verbatim.
    #[test]
    fn corpus_currency() {
        let cases: &[(&str, &str, std::result::Result<&str, &str>)] = &[
        ("0",        "EUR",   Ok("零ユーロ")),
        ("1",        "EUR",   Ok("一ユーロ")),
        ("2",        "EUR",   Ok("二ユーロ")),
        ("100",      "EUR",   Ok("百ユーロ")),
        ("12.34",    "EUR",   Ok("十二ユーロ三十四セント")),
        ("0.01",     "EUR",   Ok("零ユーロ一セント")),
        ("1.0",      "EUR",   Ok("一ユーロ")),
        ("99.99",    "EUR",   Ok("九十九ユーロ九十九セント")),
        ("1234.56",  "EUR",   Ok("千二百三十四ユーロ五十六セント")),
        ("-12.34",   "EUR",   Ok("マイナス十二ユーロ三十四セント")),
        ("1000000",  "EUR",   Ok("百万ユーロ")),
        ("0.5",      "EUR",   Ok("零ユーロ五十セント")),
        ("0",        "USD",   Ok("零ドル")),
        ("1",        "USD",   Ok("一ドル")),
        ("2",        "USD",   Ok("二ドル")),
        ("100",      "USD",   Ok("百ドル")),
        ("12.34",    "USD",   Ok("十二ドル三十四セント")),
        ("0.01",     "USD",   Ok("零ドル一セント")),
        ("1.0",      "USD",   Ok("一ドル")),
        ("99.99",    "USD",   Ok("九十九ドル九十九セント")),
        ("1234.56",  "USD",   Ok("千二百三十四ドル五十六セント")),
        ("-12.34",   "USD",   Ok("マイナス十二ドル三十四セント")),
        ("1000000",  "USD",   Ok("百万ドル")),
        ("0.5",      "USD",   Ok("零ドル五十セント")),
        ("0",        "GBP",   Ok("零ポンド")),
        ("1",        "GBP",   Ok("一ポンド")),
        ("2",        "GBP",   Ok("二ポンド")),
        ("100",      "GBP",   Ok("百ポンド")),
        ("12.34",    "GBP",   Ok("十二ポンド三十四ペンス")),
        ("0.01",     "GBP",   Ok("零ポンド一ペンス")),
        ("1.0",      "GBP",   Ok("一ポンド")),
        ("99.99",    "GBP",   Ok("九十九ポンド九十九ペンス")),
        ("1234.56",  "GBP",   Ok("千二百三十四ポンド五十六ペンス")),
        ("-12.34",   "GBP",   Ok("マイナス十二ポンド三十四ペンス")),
        ("1000000",  "GBP",   Ok("百万ポンド")),
        ("0.5",      "GBP",   Ok("零ポンド五十ペンス")),
        ("0",        "JPY",   Ok("零円")),
        ("1",        "JPY",   Ok("一円")),
        ("2",        "JPY",   Ok("二円")),
        ("100",      "JPY",   Ok("百円")),
        ("12.34",    "JPY",   Ok("十二円三十四銭")),
        ("0.01",     "JPY",   Ok("零円一銭")),
        ("1.0",      "JPY",   Ok("一円")),
        ("99.99",    "JPY",   Ok("九十九円九十九銭")),
        ("1234.56",  "JPY",   Ok("千二百三十四円五十六銭")),
        ("-12.34",   "JPY",   Ok("マイナス十二円三十四銭")),
        ("1000000",  "JPY",   Ok("百万円")),
        ("0.5",      "JPY",   Ok("零円五十銭")),
        ("0",        "KWD",   Err("NotImplementedError")),
        ("1",        "KWD",   Err("NotImplementedError")),
        ("2",        "KWD",   Err("NotImplementedError")),
        ("100",      "KWD",   Err("NotImplementedError")),
        ("12.34",    "KWD",   Err("NotImplementedError")),
        ("0.01",     "KWD",   Err("NotImplementedError")),
        ("1.0",      "KWD",   Err("NotImplementedError")),
        ("99.99",    "KWD",   Err("NotImplementedError")),
        ("1234.56",  "KWD",   Err("NotImplementedError")),
        ("-12.34",   "KWD",   Err("NotImplementedError")),
        ("1000000",  "KWD",   Err("NotImplementedError")),
        ("0.5",      "KWD",   Err("NotImplementedError")),
        ("0",        "BHD",   Err("NotImplementedError")),
        ("1",        "BHD",   Err("NotImplementedError")),
        ("2",        "BHD",   Err("NotImplementedError")),
        ("100",      "BHD",   Err("NotImplementedError")),
        ("12.34",    "BHD",   Err("NotImplementedError")),
        ("0.01",     "BHD",   Err("NotImplementedError")),
        ("1.0",      "BHD",   Err("NotImplementedError")),
        ("99.99",    "BHD",   Err("NotImplementedError")),
        ("1234.56",  "BHD",   Err("NotImplementedError")),
        ("-12.34",   "BHD",   Err("NotImplementedError")),
        ("1000000",  "BHD",   Err("NotImplementedError")),
        ("0.5",      "BHD",   Err("NotImplementedError")),
        ("0",        "INR",   Err("NotImplementedError")),
        ("1",        "INR",   Err("NotImplementedError")),
        ("2",        "INR",   Err("NotImplementedError")),
        ("100",      "INR",   Err("NotImplementedError")),
        ("12.34",    "INR",   Err("NotImplementedError")),
        ("0.01",     "INR",   Err("NotImplementedError")),
        ("1.0",      "INR",   Err("NotImplementedError")),
        ("99.99",    "INR",   Err("NotImplementedError")),
        ("1234.56",  "INR",   Err("NotImplementedError")),
        ("-12.34",   "INR",   Err("NotImplementedError")),
        ("1000000",  "INR",   Err("NotImplementedError")),
        ("0.5",      "INR",   Err("NotImplementedError")),
        ("0",        "CNY",   Ok("零元")),
        ("1",        "CNY",   Ok("一元")),
        ("2",        "CNY",   Ok("二元")),
        ("100",      "CNY",   Ok("百元")),
        ("12.34",    "CNY",   Ok("十二元三十四分")),
        ("0.01",     "CNY",   Ok("零元一分")),
        ("1.0",      "CNY",   Ok("一元")),
        ("99.99",    "CNY",   Ok("九十九元九十九分")),
        ("1234.56",  "CNY",   Ok("千二百三十四元五十六分")),
        ("-12.34",   "CNY",   Ok("マイナス十二元三十四分")),
        ("1000000",  "CNY",   Ok("百万元")),
        ("0.5",      "CNY",   Ok("零元五十分")),
        ("0",        "CHF",   Err("NotImplementedError")),
        ("1",        "CHF",   Err("NotImplementedError")),
        ("2",        "CHF",   Err("NotImplementedError")),
        ("100",      "CHF",   Err("NotImplementedError")),
        ("12.34",    "CHF",   Err("NotImplementedError")),
        ("0.01",     "CHF",   Err("NotImplementedError")),
        ("1.0",      "CHF",   Err("NotImplementedError")),
        ("99.99",    "CHF",   Err("NotImplementedError")),
        ("1234.56",  "CHF",   Err("NotImplementedError")),
        ("-12.34",   "CHF",   Err("NotImplementedError")),
        ("1000000",  "CHF",   Err("NotImplementedError")),
        ("0.5",      "CHF",   Err("NotImplementedError")),
        ];
        for (arg, code, want) in cases {
            let got = run_currency(arg, code);
            match (want, &got) {
                (Ok(w), Ok(g)) => assert_eq!(g, w, "currency:{} arg={}", code, arg),
                (Err(w), Err(g)) => assert_eq!(&err_name(g), w, "currency:{} arg={}", code, arg),
                _ => panic!("currency:{} arg={}: want {:?}, got {:?}", code, arg, want, got),
            }
        }
    }

    /// Every `ja` cheque row in the frozen corpus, verbatim. JPY's expected
    /// output ends in hiragana "えん" — Base's `cr1[-1]` "plural" reaching
    /// into JA's reading slot. See the module docs.
    #[test]
    fn corpus_cheque() {
        let cases: &[(&str, &str, std::result::Result<&str, &str>)] = &[
        ("1234.56",  "EUR",   Ok("千二百三十四 AND 56/100 ユーロ")),
        ("1234.56",  "USD",   Ok("千二百三十四 AND 56/100 ドル")),
        ("1234.56",  "GBP",   Ok("千二百三十四 AND 56/100 ポンド")),
        ("1234.56",  "JPY",   Ok("千二百三十四 AND 56/100 えん")),
        ("1234.56",  "KWD",   Err("NotImplementedError")),
        ("1234.56",  "BHD",   Err("NotImplementedError")),
        ("1234.56",  "INR",   Err("NotImplementedError")),
        ("1234.56",  "CNY",   Ok("千二百三十四 AND 56/100 元")),
        ("1234.56",  "CHF",   Err("NotImplementedError")),
        ];
        for (arg, code, want) in cases {
            let d = BigDecimal::from_str(arg).unwrap();
            let got = LangJa::new().to_cheque(&d, code);
            match (want, &got) {
                (Ok(w), Ok(g)) => assert_eq!(g, w, "cheque:{} arg={}", code, arg),
                (Err(w), Err(g)) => assert_eq!(&err_name(g), w, "cheque:{} arg={}", code, arg),
                _ => panic!("cheque:{} arg={}: want {:?}, got {:?}", code, arg, want, got),
            }
        }
    }

    /// The exact NotImplementedError text, which the corpus only records by
    /// type. Python: 'Currency code "%s" not implemented for "%s"'.
    #[test]
    fn missing_currency_message() {
        match run_currency("1.0", "KWD") {
            Err(N2WError::NotImplemented(m)) => {
                assert_eq!(m, "Currency code \"KWD\" not implemented for \"Num2Word_JA\"")
            }
            other => panic!("expected NotImplemented, got {:?}", other),
        }
    }

    /// `cents`, `separator` and `adjective` are all dead in JA's to_currency.
    /// Load-bearing: the corpus rows were generated with JA's own
    /// `cents=False` default, but the diff harness passes `cents=true`. If
    /// `cents` ever went live, `1000000` would split into units+cents and
    /// render "一万ユーロ" instead of "百万ユーロ".
    #[test]
    fn inert_kwargs() {
        let l = LangJa::new();
        for arg in ["0", "1", "2", "100", "1000000", "12.34", "0.01", "1.0", "99.99",
                    "1234.56", "-12.34", "0.5"] {
            let is_int = !arg.contains('.');
            let v = CurrencyValue::parse(arg, is_int, !is_int, !is_int).unwrap();
            for code in ["JPY", "EUR", "USD", "GBP", "CNY"] {
                let base = l.to_currency(&v, code, false, None, false).unwrap();
                assert_eq!(l.to_currency(&v, code, true, None, false).unwrap(), base,
                           "cents= changed {} {}", code, arg);
                assert_eq!(l.to_currency(&v, code, false, Some("XX"), false).unwrap(), base,
                           "separator= changed {} {}", code, arg);
                assert_eq!(l.to_currency(&v, code, false, None, true).unwrap(), base,
                           "adjective= changed {} {}", code, arg);
            }
        }
    }

    /// Build the `FloatValue` a genuine float takes, deriving precision the
    /// same way the py binding does (`abs(Decimal(repr(f)).as_tuple().exponent)`).
    fn float_val(f: f64) -> FloatValue {
        FloatValue::Float { value: f, precision: float_repr_precision(f) }
    }

    /// Every non-integer `ja` float `cardinal` row in the frozen corpus. The
    /// integer-valued floats (`0.0`, `1.0`) route to the integer path in JA's
    /// `to_cardinal` (`int(value) == value`), never reaching `to_cardinal_float`.
    #[test]
    fn corpus_cardinal_float() {
        let cases: &[(f64, &str)] = &[
            (0.5, "零点五"),
            (1.5, "一点五"),
            (2.25, "二点二五"),
            (3.14, "三点一四"),
            (0.01, "零点零一"),
            (0.1, "零点一"),
            (0.99, "零点九九"),
            (1.01, "一点零一"),
            (12.34, "十二点三四"),
            (99.99, "九十九点九九"),
            (100.5, "百点五"),
            (1234.56, "千二百三十四点五六"),
            (-0.5, "マイナス零点五"),
            (-1.5, "マイナス一点五"),
            (-12.34, "マイナス十二点三四"),
            (1.005, "一点零零五"),   // f64 artefact: 0.005*1000 -> 5.000..4
            (2.675, "二点六七五"),   // f64 artefact: 0.675*1000 -> 674.9999..8 rescued to 675
        ];
        for (f, want) in cases {
            assert_eq!(
                LangJa::new().to_cardinal_float(&float_val(*f), None).unwrap(),
                *want,
                "cardinal float arg={}",
                f
            );
        }
    }

    /// Every `ja` `cardinal_dec` (Decimal input) row in the frozen corpus. JA
    /// float-casts the Decimal, so precision comes from `repr(float)` — hence
    /// `1.10 -> "一点一"` (not "一点一零") and `98746251323029.99 -> "…点九八"`
    /// (the #603 rounding, correct for JA). The carried `precision` field is
    /// ignored by the override, so it is set to the Decimal's own exponent
    /// here only for realism.
    #[test]
    fn corpus_cardinal_dec() {
        let cases: &[(&str, u32, &str)] = &[
            ("0.01", 2, "零点零一"),
            ("1.10", 2, "一点一"),
            ("12.345", 3, "十二点三四五"),
            ("98746251323029.99", 2, "九十八兆七千四百六十二億五千百三十二万三千二十九点九八"),
            ("0.001", 3, "零点零零一"),
        ];
        for (s, prec, want) in cases {
            let v = FloatValue::Decimal {
                value: BigDecimal::from_str(s).unwrap(),
                precision: *prec,
            };
            assert_eq!(
                LangJa::new().to_cardinal_float(&v, None).unwrap(),
                *want,
                "cardinal_dec arg={}",
                s
            );
        }
    }

    /// Extra float/Decimal cases beyond the frozen corpus, values taken from
    /// the live interpreter. Guards the f64-artefact heuristic (`1.239999999`
    /// must survive as nine 9s, not collapse to `1.24`), the large-value
    /// integer head, and the Decimal float-cast dropping trailing zeros
    /// (`Decimal("1.230000")` -> precision 2, not 6).
    #[test]
    fn interpreter_cross_check() {
        let l = LangJa::new();
        let floats: &[(f64, &str)] = &[
            (0.001, "零点零零一"),
            (1000000.5, "百万点五"),
            (123456.789, "十二万三千四百五十六点七八九"),
            (0.7071, "零点七零七一"),
            (1.239999999, "一点二三九九九九九九九"),
            (-0.01, "マイナス零点零一"),
            (-0.999, "マイナス零点九九九"),
            (0.0009, "零点零零零九"),
            (1000000000.5, "十億点五"),
            (0.1234567, "零点一二三四五六七"),
        ];
        for (f, want) in floats {
            assert_eq!(l.to_cardinal_float(&float_val(*f), None).unwrap(), *want, "float {}", f);
        }
        let decs: &[(&str, &str)] = &[
            ("1.230000", "一点二三"),
            ("0.50", "零点五"),
            ("1000.00001", "千点零零零零一"),
            ("1.999999999999", "一点九九九九九九九九九九九九"),
        ];
        for (s, want) in decs {
            let v = FloatValue::Decimal { value: BigDecimal::from_str(s).unwrap(), precision: 0 };
            assert_eq!(l.to_cardinal_float(&v, None).unwrap(), *want, "dec {}", s);
        }
    }

    /// The fractional-cents arm, which no corpus row reaches. Values taken
    /// from the Python interpreter. Note `2.567` -> "零点五七セント": JA
    /// re-divides the already-rounded whole cents by 100. That is the bug,
    /// reproduced deliberately.
    #[test]
    fn fractional_cents_arm() {
        for (arg, want) in [
            ("1.005", "一ユーロ零点零一セント"),
            ("0.001", "零ユーロ零点零セント"),
            ("1.011", "一ユーロ零点零一セント"),
            ("0.005", "零ユーロ零点零一セント"),
            ("2.567", "二ユーロ零点五七セント"),
            ("0.999", "一ユーロ零点零セント"),
            ("12.345", "十二ユーロ零点三五セント"),
            ("-1.005", "マイナス一ユーロ零点零一セント"),
            ("0.7071", "零ユーロ零点七一セント"),
        ] {
            assert_eq!(run_currency(arg, "EUR").unwrap(), want, "arg={}", arg);
        }
    }
}
