//! Port of `num2words2/converters/sentence.py` (SentenceConverter).
//!
//! Language detection (langdetect/langid) stays on the Python side: the
//! shim only routes here when the caller named a language. NotImplemented
//! from here means "fall back to the Python converter".
//!
//! The port mirrors the Python class quirk-for-quirk:
//!   * extraction runs the same seven passes in the same order (plus the
//!     two additions described at the end of this header), with the
//!     same `used_positions` overlap rule (a later regex match that overlaps
//!     an earlier extraction is dropped whole, and scanning resumes after
//!     its end — so its tail is never re-matched);
//!   * every inner `num2words(...)` call reproduces the dispatcher's typed
//!     routing: ints take the integer path, floats take the float path with
//!     `precision = abs(Decimal(str(v)).as_tuple().exponent)`;
//!   * `convert_number`'s try/except laddering (year -> cardinal,
//!     anything -> English cardinal; currency is the exception, #230) is
//!     reproduced, with
//!     one refinement: a core error that means "hook not ported yet"
//!     (NotImplemented without Python's "Currency code ..." message) aborts
//!     the whole conversion instead, so the shim falls back to the original
//!     Python converter rather than guessing at the English fallback.
//!
//! Python's `re` allows lookbehind; the `regex` crate does not, so pass 7
//! (`(?<![a-zA-Z0-9])(-?\d+(?:[.,]\d+)?)(?![a-zA-Z0-9])`) is a hand-rolled
//! scanner that reproduces the backtracking semantics exactly (including
//! "drop the fractional part when the lookahead fails" — `1.5x` matches
//! just `1`).
//!
//! All positions are *character* indices, as in Python. Regex byte spans
//! are translated through a byte->char map.
//!
//! Deliberate departure from the Python original (#151): numbers written
//! with thousands separators (`1,000,000`, `$1,234.56`, `1.234,5`) are read
//! as one number: an extra pass right after the temperature passes claims
//! them (as a currency amount when a `$€£¥` symbol precedes) instead of
//! letting the later passes split them at the separator into a different
//! number. A lone `1,000` or `1.000` counts as grouping per the language's
//! notation (`strnum::number_notation`, #177): `1,000` in en/zh/ja/hi…,
//! `1.000` in de/es/it/pt… (de "1.000 Leute" is no longer the ordinal "1.").
//! Languages that group with spaces (fr, ru, pl, cs, sv, …) also accept a
//! plain ASCII space before exactly three digits: fr "10 000 personnes" is
//! "dix mille", not "dix zéro" (#233).
//!
//! Also deliberate (#152): English clock times `H:MM` get their own pass
//! ("10:30" -> "ten thirty", "10:00" -> "ten o'clock") instead of being read
//! as two numbers around a kept colon; a following am/pm marker, glued or
//! spaced, is kept as written ("10:30pm" -> "ten thirty pm", #178). The
//! English month-first date pattern takes a 1-2 digit day only, so the year
//! in "1st May 2024" is no longer read as an ordinal day ("May 2024th").
//!
//! Also deliberate (#183): German `N.` is an ordinal only when no digit
//! follows the dot. A Uhr time `14.30 Uhr` / `14:30 Uhr` reads "vierzehn Uhr
//! dreißig" ("14.00 Uhr" -> "vierzehn Uhr"), and any other dotted digit run
//! (`1.5`, `3.10.2024`, `Version 2.10`) is left as written: German writes
//! decimals with a comma, so `1.5` has no standard reading, and the ordinal
//! pass used to turn it into "Erste5".
//!
//! Also deliberate (#234): a plain or grouped number is read from its digits
//! as written, like `num2words("3.50")` — the integer path when whole, the
//! `Decimal` path otherwise — instead of through a Python float, which
//! dropped trailing zeros ("3.50" -> "three point five", "3.10" -> "three
//! point one"). Negative numbers and temperatures are read the same way, so
//! `-5` is "minus five" in every language, not the float path's "minus five
//! point zero" that the original produced through `abs(float)` (#225).
//! The sign is read by the converter itself, so the negative word is the
//! language's own (pt_BR "menos", ca "menys") rather than a small table
//! that fell back to English "minus" (#226). With `to="ordinal"` only
//! non-negative whole numbers become ordinals; decimals and negatives stay
//! cardinal (pt "3,50" was "terceiro", #231).
//!
//! Also deliberate (#228): Python's `\d` matched any Unicode digit; here
//! native decimal digits (Arabic-Indic, Devanagari, fullwidth, …) are mapped
//! to ASCII before extraction, as `num2words("١٢٣")` does, and every other
//! numeric character (`m²`, `½`) is plain text — a digit glued to one is left
//! as written instead of failing the whole call.
//!
//! Also deliberate (#229): digit groups joined by '-' or '.' are claimed
//! before the ordinal/plain passes. A two-group range (`1990-2000`, Y > X,
//! not a `555-1234` phone number) reads "X to Y" in English, where a bare
//! hyphen would make a compound ("ninety-two thousand"), and "X - Y" in other
//! languages. Every other run — `25.12.2023`, `2023-12-25`, `192.168.1.1`,
//! `v2.0.1`, phone numbers — is left as written rather than read as one
//! decimal with the rest glued on.
//!
//! Also deliberate (#230): a currency amount the language cannot name (no
//! word for the code, ko "€5") is left as written instead of being read as
//! a bare number, which made £ and ¥ indistinguishable. `R$`, `US$`, `C$`,
//! `A$` … are their own dollars rather than USD with the letters glued on,
//! a symbol after the number (`5€`, `5 €`) counts like one before it, and a
//! glued percentage (`50%`) is left as written — no converter has a percent
//! word.
//!
//! Also deliberate (#232): the ordinal registry knows the native notations
//! — ru/uk `1-й` (ru `2-я` feminine; case endings and the ambiguous `-е`
//! are left as written), pl/cs/sk/da/nb/fi/tr `1.` before a lowercase word
//! (a dot before a capital is a sentence end), es/pt/it `1°`/`1º`/`2ª`
//! (feminine for `ª`; pt/it take no `gender=`, so their -o ordinals are
//! turned -a), and the prefix forms keep their prefix word: ja `第1位` ->
//! `第一位`, ko `제1회` -> `제일회`, vi `thứ 2` -> `thứ hai` (`thứ nhất`,
//! `thứ tư`).
//!
//! Also deliberate (#235): a number after `.` is capitalised only when the
//! dot ends a sentence — not when it is the first character (`.5`) or ends
//! a listed abbreviation (`approx.`, `ca.`, `No.`, `e.g.`, `z.B.`, …).

use std::collections::HashMap;
use std::sync::OnceLock;

use std::str::FromStr;

use bigdecimal::BigDecimal;
use num2words2_core::base::{KwVal, Kwargs, Lang};
use num2words2_core::strnum::{
    groups_with_spaces, is_space_group_sep, number_notation, parse_grouped, unicode_digit,
    Grouped,
};
use num2words2_core::{get_lang_by_key, CurrencyValue, FloatValue, N2WError};
use num_bigint::BigInt;
use regex::Regex;

// ------------------------------------------------------------------ tables

/// `lang_registry._norm_lang` — lowercase/strip, keep as-is when it is a known
/// ordinal key, else fall back to the base subtag. Used for the
/// registry lookups (ordinal/date/month) that Python routes through
/// `get_ordinal_pattern` / `get_date_patterns` / `get_month_names`.
fn norm_lang(lang: &str) -> String {
    let l = lang.trim().to_lowercase();
    if l.is_empty() {
        return "en".to_string();
    }
    if ORDINAL_PATTERNS.iter().any(|(k, _, _)| *k == l) {
        return l;
    }
    l.split(['-', '_']).next().unwrap_or("").to_string()
}

/// `lang_registry.TEMP_PATTERNS` — (regex, scale_word, scale_unit) per lang.
/// Keyed by the raw lang string (Python: `if self.lang in self.temp_patterns`).
/// Yes, English says "Fahrenheit" even for `25°C`; the quirk is deliberate.
const TEMP_PATTERNS: &[(&str, &str, &str, &str)] = &[
    ("en", r"(-?\d+(?:[.,]\d+)?)\s+degrees?(?:\s+[Ff]ahrenheit|\s+[Cc]elsius)?", "degrees", "Fahrenheit"),
    ("fr", r"(-?\d+(?:[.,]\d+)?)\s+degr[ée]s?(?:\s+[Cc]elsius)?", "degrés", "Celsius"),
    ("es", r"(-?\d+(?:[.,]\d+)?)\s+grados?(?:\s+[Cc]elsius)?", "grados", "Celsius"),
    ("pt", r"(-?\d+(?:[.,]\d+)?)\s+graus?(?:\s+[Cc]elsius)?", "graus", "Celsius"),
    ("it", r"(-?\d+(?:[.,]\d+)?)\s+gradi?(?:\s+[Cc]elsius)?", "gradi", "Celsius"),
    ("de", r"(-?\d+(?:[.,]\d+)?)\s+[Gg]rad(?:\s+[Cc]elsius)?", "Grad", "Celsius"),
    ("nl", r"(-?\d+(?:[.,]\d+)?)\s+graden?(?:\s+[Cc]elsius)?", "graden", "Celsius"),
    ("sv", r"(-?\d+(?:[.,]\d+)?)\s+grader?(?:\s+[Cc]elsius)?", "grader", "Celsius"),
    ("da", r"(-?\d+(?:[.,]\d+)?)\s+grader?(?:\s+[Cc]elsius)?", "grader", "Celsius"),
    ("no", r"(-?\d+(?:[.,]\d+)?)\s+grader?(?:\s+[Cc]elsius)?", "grader", "Celsius"),
    ("fi", r"(-?\d+(?:[.,]\d+)?)\s+astetta?(?:\s+[Cc]elsiusta)?", "astetta", "Celsius"),
    ("ru", r"(-?\d+(?:[.,]\d+)?)\s+градус(?:а|ов)?", "градусов", "Цельсия"),
    ("uk", r"(-?\d+(?:[.,]\d+)?)\s+градус(?:а|ів)?", "градусів", "Цельсія"),
    ("pl", r"(-?\d+(?:[.,]\d+)?)\s+stopni(?:e|i)?", "stopni", "Celsjusza"),
    ("cs", r"(-?\d+(?:[.,]\d+)?)\s+stup(?:ňů|eň|ně)", "stupňů", "Celsia"),
    ("tr", r"(-?\d+(?:[.,]\d+)?)\s+derece", "derece", "santigrat"),
    ("ja", r"(-?\d+(?:[.,]\d+)?)\s*度", "度", "摂氏"),
    ("zh", r"(-?\d+(?:[.,]\d+)?)\s*度", "度", "摄氏"),
    ("ko", r"(-?\d+(?:[.,]\d+)?)\s*도", "도", "섭씨"),
    ("el", r"(-?\d+(?:[.,]\d+)?)\s+βαθμο[ίυ]?ς?", "βαθμοί", "Κελσίου"),
    ("ar", r"(-?\d+(?:[.,]\d+)?)\s+درجة", "درجة", "مئوية"),
    ("hi", r"(-?\d+(?:[.,]\d+)?)\s+डिग्री", "डिग्री", "सेल्सियस"),
];

fn temp_words(lang: &str) -> Option<(&'static str, &'static str)> {
    TEMP_PATTERNS
        .iter()
        .find(|(k, _, _, _)| *k == lang)
        .map(|(_, _, w, u)| (*w, *u))
}

/// `lang_registry.ORDINAL_PATTERNS` — the integer is captured in group 1.
/// Keyed by the normalised lang; a language may have several. Compiled
/// WITHOUT the ignore-case flag, matching Python's
/// `re.finditer(ordinal_pattern, sentence)` (no flags). The [`OrdForm`]
/// says how the match is read (#232).
const ORDINAL_PATTERNS: &[(&str, &str, OrdForm)] = &[
    ("en", r"(\d+)(?:st|nd|rd|th)\b", OrdForm::Suffix),
    ("de", r"(\d+)(?:\.|te|er)\b", OrdForm::Suffix),
    ("nl", r"(\d+)(?:ste|de|e)\b", OrdForm::Suffix),
    ("sv", r"(\d+):(?:a|e)\b", OrdForm::Suffix),
    ("af", r"(\d+)(?:ste|de)\b", OrdForm::Suffix),
    ("fr", r"(\d+)(?:er|ère|e|ème)\b", OrdForm::Suffix),
    ("es", r"(\d+)\.?[º°ª]", OrdForm::Symbol),
    ("pt", r"(\d+)\.?[º°ª]", OrdForm::Symbol),
    ("it", r"(\d+)\.?[º°ª]", OrdForm::Symbol),
    ("ca", r"(\d+)(?:r|n|t|è|a)\b", OrdForm::Suffix),
    ("el", r"(\d+)(?:ος|η|ο|ός)\b", OrdForm::Suffix),
    ("tr", r"(\d+)(?:inci|ıncı|uncu|üncü)\b", OrdForm::Suffix),
    ("az", r"(\d+)[-‐](?:ci|cu|cü|cı)\b", OrdForm::Suffix),
    ("hi", r"(\d+)(?:वां|वीं|वें)\b", OrdForm::Suffix),
    ("bn", r"(\d+)(?:তম|ম|য়|র্থ)\b", OrdForm::Suffix),
    ("ta", r"(\d+)(?:வது|ஆம்)\b", OrdForm::Suffix),
    ("fa", r"(\d+)(?:مین|ام|م)\b", OrdForm::Suffix),
    ("zh", r"第(\d+)", OrdForm::Suffix),
    ("ja", r"第(\d+)", OrdForm::Prefix),
    ("ja", r"(\d+)番目", OrdForm::Suffix),
    ("ko", r"제(\d+)", OrdForm::Prefix),
    ("ko", r"(\d+)번째", OrdForm::Suffix),
    ("vi", r"thứ\s*(\d+)", OrdForm::Vi),
    ("th", r"ที่\s*(\d+)", OrdForm::Suffix),
    ("id", r"ke[-‐](\d+)", OrdForm::Suffix),
    ("ms", r"ke[-‐](\d+)", OrdForm::Suffix),
    ("ia", r"(\d+)me\b", OrdForm::Suffix),
    // "1. miejsce", "1. místo", "1. plads", "1. sırada" (#232).
    ("pl", r"(\d+)\.", OrdForm::Dot),
    ("cs", r"(\d+)\.", OrdForm::Dot),
    ("sk", r"(\d+)\.", OrdForm::Dot),
    ("da", r"(\d+)\.", OrdForm::Dot),
    ("nb", r"(\d+)\.", OrdForm::Dot),
    ("no", r"(\d+)\.", OrdForm::Dot),
    ("fi", r"(\d+)\.", OrdForm::Dot),
    ("tr", r"(\d+)\.", OrdForm::Dot),
    // "1-й", "2-я" (#232).
    ("ru", r"(\d+)-(\p{Cyrillic}+)", OrdForm::Hyphen),
    ("uk", r"(\d+)-(\p{Cyrillic}+)", OrdForm::Hyphen),
];

/// How an [`ORDINAL_PATTERNS`] match is read.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OrdForm {
    /// The whole match is replaced by the ordinal.
    Suffix,
    /// `<n>º` / `<n>ª` (es, pt, it): the ordinal, feminine for `ª`; the
    /// marker must end the word (`\b` never matched after `°`).
    Symbol,
    /// A prefix word that stays (ja `第1位` -> `第一位`, ko `제1회` ->
    /// `제일회`): only the digits are read, as a cardinal.
    Prefix,
    /// vi `thứ 2` -> `thứ hai`: the prefix stays, the number is read
    /// `nhất`/`tư` for 1/4 and as a cardinal otherwise.
    Vi,
    /// `<n>.` followed by a lowercase word (pl `1. miejsce`); a dot before
    /// a capital or at the end is a sentence end, not an ordinal.
    Dot,
    /// ru/uk `<n>-<ending>`: the nominative endings say the gender; any
    /// other ending (case forms, "-летний") is left as written.
    Hyphen,
}

/// How a [`Typ::Ordinal`] is said.
#[derive(Clone, Copy)]
enum OrdKind {
    Masc,
    Fem,
    Cardinal,
    Vi,
}

/// `lang_registry.MONTH_NAMES` — month-name regex per lang (a non-capturing
/// group). Keyed by the normalised lang. Substituted into date templates
/// (word-boundary wrapped) and used to gate year detection (pass 5).
const MONTH_NAMES: &[(&str, &str)] = &[
    ("en", r"(?:January|February|March|April|May|June|July|August|September|October|November|December|Jan|Feb|Mar|Apr|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec)"),
    ("fr", r"(?:janvier|f[ée]vrier|mars|avril|mai|juin|juillet|ao[uû]t|septembre|octobre|novembre|d[ée]cembre)"),
    ("es", r"(?:enero|febrero|marzo|abril|mayo|junio|julio|agosto|septiembre|octubre|noviembre|diciembre)"),
    ("pt", r"(?:janeiro|fevereiro|mar[çc]o|abril|maio|junho|julho|agosto|setembro|outubro|novembro|dezembro)"),
    ("it", r"(?:gennaio|febbraio|marzo|aprile|maggio|giugno|luglio|agosto|settembre|ottobre|novembre|dicembre)"),
    ("de", r"(?:Januar|Februar|M[äa]rz|April|Mai|Juni|Juli|August|September|Oktober|November|Dezember|J[äa]nner)"),
    ("nl", r"(?:januari|februari|maart|april|mei|juni|juli|augustus|september|oktober|november|december)"),
    ("sv", r"(?:januari|februari|mars|april|maj|juni|juli|augusti|september|oktober|november|december)"),
    ("da", r"(?:januar|februar|marts|april|maj|juni|juli|august|september|oktober|november|december)"),
    ("no", r"(?:januar|februar|mars|april|mai|juni|juli|august|september|oktober|november|desember)"),
    ("fi", r"(?:tammikuu(?:ta)?|helmikuu(?:ta)?|maaliskuu(?:ta)?|huhtikuu(?:ta)?|toukokuu(?:ta)?|kes[äa]kuu(?:ta)?|hein[äa]kuu(?:ta)?|elokuu(?:ta)?|syyskuu(?:ta)?|lokakuu(?:ta)?|marraskuu(?:ta)?|joulukuu(?:ta)?)"),
    ("is", r"(?:jan[úu]ar|febr[úu]ar|mars|apr[íi]l|ma[íi]|j[úu]n[íi]|j[úu]l[íi]|[áa]g[úu]st|september|okt[óo]ber|n[óo]vember|desember)"),
    ("ru", r"(?:январ[ьея]|феврал[ьея]|март[а]?|апрел[ьея]|ма[йяе]|июн[ьея]|июл[ьея]|август[а]?|сентябр[ьея]|октябр[ьея]|ноябр[ьея]|декабр[ьея])"),
    ("uk", r"(?:січн[яеі]|лют[ого]|березн[яе]|квітн[яе]|травн[яе]|червн[яе]|липн[яе]|серпн[яе]|вересн[яе]|жовтн[яе]|листопад[а]?|грудн[яе])"),
    ("pl", r"(?:styczni[ae]|luty|lutego|marzec|marca|kwiecie[nń]|kwietnia|maj[a]?|czerwiec|czerwca|lipiec|lipca|sierpie[nń]|sierpnia|wrzesie[nń]|wrze[śs]nia|pa[źz]dziernik[a]?|listopad[a]?|grudzie[nń]|grudnia)"),
    ("cs", r"(?:ledn[aue]|[úu]nor[a]?|b[řr]ezn[aue]|duben|dubna|kv[ěe]ten|kv[ěe]tna|[čc]erven[ae]?|[čc]ervna|[čc]ervenec|[čc]ervence|srpen|srpna|z[áa][řr][íi]|[řr][íi]jen|[řr][íi]jna|listopad[au]?|prosinec|prosince)"),
    ("sk", r"(?:janu[áa]r[a]?|febru[áa]r[a]?|marec|marca|apr[íi]l[a]?|m[áa]j[a]?|j[úu]n[a]?|j[úu]l[a]?|august[a]?|september|septembra|okt[óo]ber|okt[óo]bra|november|novembra|december|decembra)"),
    ("ro", r"(?:ianuarie|februarie|martie|aprilie|mai|iunie|iulie|august|septembrie|octombrie|noiembrie|decembrie)"),
    ("el", r"(?:Ιανουαρίου|Φεβρουαρίου|Μαρτίου|Απριλίου|Μαΐου|Ιουνίου|Ιουλίου|Αυγούστου|Σεπτεμβρίου|Οκτωβρίου|Νοεμβρίου|Δεκεμβρίου|Ιανουάριος|Φεβρουάριος|Μάρτιος|Απρίλιος|Μάιος|Ιούνιος|Ιούλιος|Αύγουστος|Σεπτέμβριος|Οκτώβριος|Νοέμβριος|Δεκέμβριος)"),
    ("tr", r"(?:Ocak|Şubat|Mart|Nisan|Mayıs|Haziran|Temmuz|Ağustos|Eylül|Ekim|Kasım|Aralık)"),
    ("hu", r"(?:janu[áa]r|febru[áa]r|m[áa]rcius|[áa]prilis|m[áa]jus|j[úu]nius|j[úu]lius|augusztus|szeptember|okt[óo]ber|november|december)"),
    ("ar", r"(?:يناير|فبراير|مارس|أبريل|مايو|يونيو|يوليو|أغسطس|سبتمبر|أكتوبر|نوفمبر|ديسمبر|كانون|شباط|آذار|نيسان|أيار|حزيران|تموز|آب|أيلول|تشرين|تشرين)"),
    ("he", r"(?:ינואר|פברואר|מרץ|אפריל|מאי|יוני|יולי|אוגוסט|ספטמבר|אוקטובר|נובמבר|דצמבר)"),
    ("ja", r"(?:1月|2月|3月|4月|5月|6月|7月|8月|9月|10月|11月|12月|睦月|如月|弥生|卯月|皐月|水無月|文月|葉月|長月|神無月|霜月|師走)"),
    ("ko", r"(?:1월|2월|3월|4월|5월|6월|7월|8월|9월|10월|11월|12월)"),
    ("zh", r"(?:1月|2月|3月|4月|5月|6月|7月|8月|9月|10月|11月|12月|一月|二月|三月|四月|五月|六月|七月|八月|九月|十月|十一月|十二月)"),
    ("vi", r"(?:th[áa]ng\s*(?:m[ộo]t|hai|ba|b[ốo]n|n[ăa]m|s[áa]u|b[ảa]y|t[áa]m|ch[íi]n|m[ưu][ờo]i|m[ưu][ờo]i\s*m[ộo]t|m[ưu][ờo]i\s*hai|\d+))"),
    ("th", r"(?:มกราคม|กุมภาพันธ์|มีนาคม|เมษายน|พฤษภาคม|มิถุนายน|กรกฎาคม|สิงหาคม|กันยายน|ตุลาคม|พฤศจิกายน|ธันวาคม)"),
    ("id", r"(?:Januari|Februari|Maret|April|Mei|Juni|Juli|Agustus|September|Oktober|November|Desember)"),
    ("ms", r"(?:Januari|Februari|Mac|April|Mei|Jun|Julai|Ogos|September|Oktober|November|Disember)"),
    ("hi", r"(?:जनवरी|फ़रवरी|फरवरी|मार्च|अप्रैल|मई|जून|जुलाई|अगस्त|सितंबर|अक्टूबर|नवंबर|दिसंबर)"),
    ("bn", r"(?:জানুয়ারি|ফেব্রুয়ারি|মার্চ|এপ্রিল|মে|জুন|জুলাই|আগস্ট|সেপ্টেম্বর|অক্টোবর|নভেম্বর|ডিসেম্বর)"),
    ("fa", r"(?:ژانویه|فوریه|مارس|آوریل|می|ژوئن|ژوئیه|اوت|سپتامبر|اکتبر|نوامبر|دسامبر|فروردین|اردیبهشت|خرداد|تیر|مرداد|شهریور|مهر|آبان|آذر|دی|بهمن|اسفند)"),
];

/// `lang_registry.DATE_PATTERNS_TEMPLATE` — (template, is_ordinal) per lang.
/// `{month}` is replaced with the word-boundary-wrapped `MONTH_NAMES[lang]`
/// at build time (Python `get_date_patterns`); a language with no month-name
/// entry yields no date patterns.
const DATE_TEMPLATES: &[(&str, &[(&str, bool)])] = &[
    ("en", &[
        (r"(\d+)(?:st|nd|rd|th)\s+({month})", true),
        // Day is 1-2 digits: "May 2024" is month + year, not "May 2024th"
        // (#152).
        (r"({month})\s+(\d{1,2})\b", true),
        (r"(\d+)\s+({month})", true),
    ]),
    ("fr", &[
        (r"(\d+)er\s+({month})", true),
        (r"(\d+)e\s+({month})", false),
    ]),
    ("es", &[(r"(\d+)\s+de\s+({month})", false)]),
    ("de", &[(r"(\d+)\.\s+([A-ZÄÖÜ][a-zäöüß]+)", true)]),
    ("pt", &[(r"(\d+)\s+de\s+({month})", false)]),
    ("it", &[(r"(\d+)\s+({month})", false)]),
    ("nl", &[(r"(\d+)\s+({month})", true)]),
    ("sv", &[(r"(\d+)\s+({month})", true)]),
    ("da", &[(r"(\d+)\.\s+({month})", false)]),
    ("no", &[(r"(\d+)\.\s+({month})", false)]),
    ("fi", &[(r"(\d+)\.\s+({month})", false)]),
    ("is", &[(r"(\d+)\.\s+({month})", false)]),
    ("ro", &[(r"(\d+)\s+({month})", false)]),
    ("el", &[(r"(\d+)\s+({month})", false)]),
    ("tr", &[(r"(\d+)\s+({month})", false)]),
    ("hu", &[(r"({month})\s+(\d+)\.?", false)]),
    ("ja", &[
        (r"({month})\s*(\d+)日", false),
        (r"(\d+)月\s*(\d+)日", false),
    ]),
    ("zh", &[
        (r"({month})\s*(\d+)日?", false),
        (r"(\d+)月\s*(\d+)日?", false),
    ]),
    ("ko", &[(r"({month})\s*(\d+)일", false)]),
    ("vi", &[(r"(\d+)\s+({month})", false)]),
    ("th", &[(r"(\d+)\s+({month})", false)]),
    ("id", &[(r"(\d+)\s+({month})", false)]),
    ("ms", &[(r"(\d+)\s+({month})", false)]),
    ("hi", &[(r"(\d+)\s+({month})", false)]),
    ("bn", &[(r"(\d+)\s+({month})", false)]),
    ("fa", &[(r"(\d+)\s+({month})", false)]),
];

/// Languages whose written ordinal day form is `<n>.` — the trailing period is
/// consumed into the day token so replacement leaves no orphaned punctuation.
/// Checked against the raw lang, matching Python's `self.lang in ...`.
const DAY_TOKEN_TRAILS_PERIOD: &[&str] = &["de", "cs", "sk", "fi", "hu", "is", "no", "da"];

struct DatePat {
    re: Regex,
    is_ordinal: bool,
}

struct Res {
    temp_symbol: Regex,
    temps: Vec<(&'static str, Regex)>,
    ordinals: Vec<(&'static str, Regex, OrdForm)>,
    dates: Vec<(&'static str, Vec<DatePat>)>,
    year: Regex,
    currency: Regex,
    amount: Regex,
    clock: Regex,
    uhr: Regex,
}

impl Res {
    fn new() -> Res {
        // `\d` is ASCII-only here: native digits are normalised to ASCII
        // before extraction (#228), and any other Unicode digit is text.
        let re = |p: &str| Regex::new(&ascii_digits(p)).expect("static regex");

        let temps = TEMP_PATTERNS
            .iter()
            .map(|(lang, pat, _, _)| (*lang, re(pat)))
            .collect();

        let ordinals = ORDINAL_PATTERNS
            .iter()
            .map(|(lang, pat, form)| (*lang, re(pat), *form))
            .collect();

        // Build concrete date patterns: substitute `{month}` with the
        // boundary-wrapped month regex, compile case-insensitively (Python
        // passes `re.I` at finditer time). Templates without `{month}` (e.g.
        // German's "<n>. <Noun>") are used verbatim.
        let month_of = |lang: &str| MONTH_NAMES.iter().find(|(k, _)| *k == lang).map(|(_, v)| *v);
        let dates = DATE_TEMPLATES
            .iter()
            .filter_map(|(lang, tpls)| {
                let months = month_of(lang)?;
                let bounded = format!(r"\b{}\b", months);
                let pats = tpls
                    .iter()
                    .map(|(tpl, is_ordinal)| DatePat {
                        re: re(&format!("(?i){}", tpl.replace("{month}", &bounded))),
                        is_ordinal: *is_ordinal,
                    })
                    .collect();
                Some((*lang, pats))
            })
            .collect();

        Res {
            temp_symbol: re(r"(-?\d+(?:[.,]\d+)?)\s*°[CFcf]"),
            temps,
            ordinals,
            dates,
            year: re(r"\b(19\d{2}|20\d{2}|2100)\b"),
            currency: re(r"([$€£¥]\s*)(\d+(?:[.,]\d+)?)"),
            amount: re(r"\d+(?:[.,]\d+)?"),
            // The trailing boundary is checked in code: a glued "pm" (#178)
            // has no `\b` before it.
            clock: re(r"\b(\d{1,2}):(\d{2})"),
            uhr: re(r"\b(\d{1,2})[.:](\d{2})\s*Uhr\b"),
        }
    }

    /// Temperature regex — keyed by the raw lang (Python: `self.temp_patterns`).
    fn temp_re(&self, lang: &str) -> Option<&Regex> {
        self.temps.iter().find(|(k, _)| *k == lang).map(|(_, r)| r)
    }

    /// Ordinal regexes — keyed by the normalised lang (`get_ordinal_pattern`).
    fn ordinal_res(&self, lang: &str) -> Vec<(&Regex, OrdForm)> {
        let n = norm_lang(lang);
        self.ordinals
            .iter()
            .filter(|(k, _, _)| *k == n)
            .map(|(_, r, f)| (r, *f))
            .collect()
    }

    /// Date patterns — keyed by the normalised lang (`get_date_patterns`).
    fn date_pats(&self, lang: &str) -> Option<&Vec<DatePat>> {
        let n = norm_lang(lang);
        self.dates.iter().find(|(k, _)| *k == n).map(|(_, v)| v)
    }

    /// Month-name regex — keyed by the normalised lang (`get_month_names`).
    fn months_re(&self, lang: &str) -> Option<&'static str> {
        let n = norm_lang(lang);
        MONTH_NAMES.iter().find(|(k, _)| *k == n).map(|(_, v)| *v)
    }
}

/// A pattern with every `\d` narrowed to `[0-9]` (the regex crate's `\d` is
/// Unicode `Nd`).
fn ascii_digits(p: &str) -> String {
    p.replace(r"\d", "[0-9]")
}

/// The text with native decimal digits (Arabic-Indic, Devanagari,
/// fullwidth, …) mapped to ASCII, the way `num2words("١٢٣")` reads them, and
/// the Arabic decimal separator `٫` between two digits mapped to '.'. Char
/// for char, so positions are unchanged (#228).
fn normalize_digits(text: &str) -> String {
    let cs: Vec<char> = text.chars().collect();
    let digit = |c: char| unicode_digit(c).is_some();
    cs.iter()
        .enumerate()
        .map(|(i, &c)| match unicode_digit(c) {
            Some(d) => char::from(b'0' + d as u8),
            None if c == '\u{066B}'
                && i > 0
                && digit(cs[i - 1])
                && cs.get(i + 1).is_some_and(|&n| digit(n)) =>
            {
                '.'
            }
            None => c,
        })
        .collect()
}

fn res() -> &'static Res {
    static R: OnceLock<Res> = OnceLock::new();
    R.get_or_init(Res::new)
}

// ------------------------------------------------------------- char space

/// The sentence with a byte-offset -> char-offset map, so regex byte spans
/// become the char positions Python slices with.
struct Text<'a> {
    s: &'a str,
    chars: Vec<char>,
    b2c: HashMap<usize, usize>,
}

impl<'a> Text<'a> {
    fn new(s: &'a str) -> Text<'a> {
        let chars: Vec<char> = s.chars().collect();
        let mut b2c = HashMap::with_capacity(chars.len() + 1);
        for (ci, (bi, _)) in s.char_indices().enumerate() {
            b2c.insert(bi, ci);
        }
        b2c.insert(s.len(), chars.len());
        Text { s, chars, b2c }
    }

    fn span(&self, bstart: usize, bend: usize) -> (usize, usize) {
        (self.b2c[&bstart], self.b2c[&bend])
    }

    fn slice(&self, a: usize, b: usize) -> String {
        self.chars[a.min(self.chars.len())..b.min(self.chars.len())]
            .iter()
            .collect()
    }
}

// ------------------------------------------------------------- extraction

#[derive(Clone)]
enum Val {
    F(f64),
    I(BigInt),
    /// A number as written, canonicalised to ASCII (`-5`, `3.50`): read
    /// through the integer path when whole, else as a `Decimal`, so the
    /// digits are kept as written (#234).
    D(String),
}

enum Typ {
    TempSymbol,
    TempWord,
    Ordinal(OrdKind),
    OrdinalDate,
    DateNumber,
    Year,
    /// A currency amount and its ISO code; `None` (a symbol whose code is
    /// unknown, e.g. `Z$`) leaves the token as written (#230).
    Currency(Option<&'static str>),
    Number,
    /// English clock time `H:MM` as (hour, minute, am/pm suffix as
    /// written, e.g. "pm", "PM", "p.m.").
    Time(u32, u32, Option<String>),
    /// German clock time `H.MM Uhr` / `H:MM Uhr` as (hour, minute) (#183).
    UhrTime(u32, u32),
    /// A numeric range `X-Y` (#229) as (to, joined with "to"): en reads
    /// "X to Y", other languages "X - Y".
    Range(BigInt, bool),
}

struct Ext {
    start: usize,
    end: usize,
    text: String,
    val: Val,
    typ: Typ,
}

fn overlap(used: &[bool], a: usize, b: usize) -> bool {
    used[a.min(used.len())..b.min(used.len())].iter().any(|&u| u)
}

fn mark(used: &mut [bool], a: usize, b: usize) {
    let n = used.len();
    for p in a.min(n)..b.min(n) {
        used[p] = true;
    }
}

/// `float(text.replace(",", "."))` — inputs are guaranteed ASCII-digit
/// strings of the form `-?\d+([.,]\d+)?`, so parse failure/overflow only
/// happens on absurd inputs; bail so the Python original decides.
fn pyfloat(s: &str) -> Result<f64, N2WError> {
    let v: f64 = s
        .replace(',', ".")
        .parse()
        .map_err(|_| N2WError::Fallback("sentence: float parse".into()))?;
    if !v.is_finite() {
        return Err(N2WError::Fallback("sentence: non-finite float".into()));
    }
    Ok(v)
}

fn pyint(s: &str) -> Result<BigInt, N2WError> {
    s.parse::<BigInt>()
        .map_err(|_| N2WError::Fallback("sentence: int parse".into()))
}

/// Pass 7: Python's
/// `(?<![a-zA-Z0-9])(-?\d+(?:[.,]\d+)?)(?![a-zA-Z0-9])` (finditer), with
/// its backtracking reproduced by hand because the regex crate has no
/// lookaround. The lookbehind/lookahead classes are ASCII-only in the
/// original, so `is_ascii_alphanumeric` is exact. When the lookahead fails
/// after a fractional part, Python's engine gives back the fraction and
/// succeeds at the separator (`1.5x` -> `1`); shrinking digit runs can never
/// satisfy the lookahead (the next char would be a digit), so those retries
/// are omitted rather than simulated.
fn plain_number_spans(chars: &[char]) -> Vec<(usize, usize)> {
    let n = chars.len();
    let mut out = Vec::new();
    let mut i = 0;
    while i < n {
        if i > 0 && (chars[i - 1].is_ascii_alphanumeric() || chars[i - 1].is_numeric()) {
            i += 1;
            continue;
        }
        // A hyphen right after a letter of any script is not a minus sign
        // (he "ו-2" is "and 2", ru "и-2"; #227): skip it so the digits match
        // on their own. Only the sign test is Unicode-aware; the digit
        // boundaries stay ASCII so "有5个" still reads its 5.
        if chars[i] == '-' && i > 0 && is_word_letter(chars[i - 1]) {
            i += 1;
            continue;
        }
        let mut j = i;
        if chars[j] == '-' {
            j += 1;
        }
        let digits_start = j;
        while j < n && chars[j].is_ascii_digit() {
            j += 1;
        }
        if j == digits_start {
            i += 1;
            continue;
        }
        let int_end = j;
        let mut end = int_end;
        if j < n && (chars[j] == '.' || chars[j] == ',') {
            let mut k = j + 1;
            while k < n && chars[k].is_ascii_digit() {
                k += 1;
            }
            if k > j + 1 {
                end = k;
            }
        }
        // A digit glued to a character it cannot read ("5½", "10²") is
        // left alone rather than read as "five½" (#228).
        let ahead_ok =
            |e: usize| e >= n || !(chars[e].is_ascii_alphanumeric() || chars[e].is_numeric());
        let fin = if ahead_ok(end) {
            Some(end)
        } else if end > int_end {
            // Drop the fractional part; the lookahead then sees the
            // separator, which always passes.
            Some(int_end)
        } else {
            None
        };
        match fin {
            Some(e) => {
                out.push((i, e));
                i = e;
            }
            None => i += 1,
        }
    }
    out
}

/// A letter or digit that a hyphen after it joins to a word (#227). Scripts
/// written without spaces between words (Han, kana, Thai, Lao, Khmer,
/// Myanmar) are excluded: zh "温度是-5度" is minus five.
fn is_word_letter(c: char) -> bool {
    c.is_alphanumeric()
        && !matches!(c as u32,
            0x3040..=0x30FF | 0x31F0..=0x31FF // kana
            | 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF // Han
            | 0x0E00..=0x0EFF // Thai, Lao
            | 0x1000..=0x109F // Myanmar
            | 0x1780..=0x17FF) // Khmer
}

/// A number written with thousands separators starting at char `start`
/// (`1,000,000`, `1.234,56`, `-1 000`; #151). Returns the end of the token and
/// its value as a plain decimal string. Only tokens that really contain
/// grouping qualify, with [`parse_grouped`]'s rules and the language's
/// [`number_notation`] (so `1,000` counts only in `1,000.5` languages, `1.000`
/// only in `1.000,5` ones, #177, and `192.168.1.1` or `1,2,3` never match;
/// a dot not followed by exactly three digits is left to the later passes:
/// de "1. Mai" is an ordinal, de "1.5" is kept as written, #183); the
/// token must not touch an ASCII letter/digit on either side, like pass 7.
/// ASCII spaces are taken as separators in running text only for the
/// languages that group with spaces ([`groups_with_spaces`], #233: fr
/// "10 000 personnes"), and only before exactly three digits; elsewhere
/// ("between 2 100 and") only the no-break/thin spaces and apostrophes.
fn grouped_token(chars: &[char], start: usize, lang: &str) -> Option<(usize, String)> {
    let n = chars.len();
    if start > 0 && chars[start - 1].is_ascii_alphanumeric() {
        return None;
    }
    // Never start in the middle of a separated digit run ("3.14,159").
    if start > 1 && is_part_sep(chars[start - 1]) && chars[start - 2].is_ascii_digit() {
        return None;
    }
    let mut j = start;
    if j < n && chars[j] == '-' {
        // Not a sign after a letter of any script (#227).
        if start > 0 && is_word_letter(chars[start - 1]) {
            return None;
        }
        j += 1;
    }
    if j >= n || !chars[j].is_ascii_digit() {
        return None;
    }
    let is_part = |c: char| c.is_ascii_digit() || is_part_sep(c);
    let spaces = groups_with_spaces(lang);
    // " 000" followed by a non-digit: a space-grouped thousands group.
    let space_group = |k: usize| {
        spaces
            && chars[k] == ' '
            && k > 0
            && chars[k - 1].is_ascii_digit()
            && k + 3 < n
            && chars[k + 1..k + 4].iter().all(|c| c.is_ascii_digit())
            && (k + 4 >= n || !chars[k + 4].is_ascii_digit())
    };
    let mut end = j;
    while end < n && (is_part(chars[end]) || space_group(end)) {
        end += 1;
    }
    while end > j && !chars[end - 1].is_ascii_digit() {
        end -= 1;
    }
    if end < n && (chars[end].is_ascii_alphanumeric() || chars[end].is_numeric()) {
        return None;
    }
    let tok: String = chars[start..end].iter().collect();
    match parse_grouped(&tok, number_notation(lang)) {
        Grouped::Number { canonical, .. } => {
            let seps = tok
                .chars()
                .filter(|&c| !c.is_ascii_digit() && c != '-')
                .count();
            let has_fraction = canonical.contains('.') as usize;
            if seps > has_fraction {
                Some((end, canonical))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// A separator [`grouped_token`] reads inside a number in running text.
fn is_part_sep(c: char) -> bool {
    c == '.' || c == ',' || (c != ' ' && is_space_group_sep(c))
}

/// The currency symbol before a number starting at char `i` (`$5`, `€ 5`,
/// `R$ 3,50`): (start of the token, ISO code). A letter-prefixed dollar is
/// that country's dollar (`R$` BRL, `US$` USD, `C$` CAD, `A$` AUD, …), not
/// USD with the letters glued onto the words; any other letter run before
/// the symbol gives `None`, which leaves the token as written (#230).
fn currency_before(chars: &[char], i: usize) -> Option<(usize, Option<&'static str>)> {
    let mut k = i;
    while k > 0 && chars[k - 1].is_whitespace() {
        k -= 1;
    }
    let sym = *chars.get(k.checked_sub(1)?)?;
    let code = symbol_code(sym)?;
    let mut p = k - 1;
    while p > 0 && chars[p - 1].is_ascii_alphabetic() {
        p -= 1;
    }
    if p == k - 1 {
        return Some((p, Some(code)));
    }
    let prefix: String = chars[p..k - 1].iter().collect();
    let code = match (sym, prefix.as_str()) {
        _ if p > 0 && chars[p - 1].is_alphanumeric() => None,
        ('$', "US") => Some("USD"),
        ('$', "R") => Some("BRL"),
        ('$', "C" | "CA") => Some("CAD"),
        ('$', "A" | "AU") => Some("AUD"),
        ('$', "NZ") => Some("NZD"),
        ('$', "HK") => Some("HKD"),
        ('$', "S") => Some("SGD"),
        ('$', "MX") => Some("MXN"),
        _ => None,
    };
    Some((p, code))
}

/// A currency symbol after a number ending at char `e` (`5€`, `5 €`):
/// (end of the token, ISO code). The symbol must end the word and must not
/// open the next amount (`5 $10`).
fn currency_after(chars: &[char], e: usize) -> Option<(usize, &'static str)> {
    let n = chars.len();
    let mut k = e;
    if k < n && matches!(chars[k], ' ' | '\u{00A0}' | '\u{202F}') {
        k += 1;
    }
    let code = symbol_code(*chars.get(k)?)?;
    let mut after = k + 1;
    // The symbol ends the word: whitespace or punctuation follows ("5 €₹"
    // is not an amount).
    let ends = |c: char| {
        c.is_whitespace()
            || (c.is_ascii_punctuation() && !matches!(c, '$' | '%'))
            || matches!(c, '»' | '”' | '’' | '…')
    };
    if after < n && !ends(chars[after]) {
        return None;
    }
    while after < n && chars[after].is_whitespace() {
        after += 1;
    }
    if after < n && chars[after].is_ascii_digit() {
        return None;
    }
    Some((k + 1, code))
}

fn symbol_code(c: char) -> Option<&'static str> {
    match c {
        '$' => Some("USD"),
        '€' => Some("EUR"),
        '£' => Some("GBP"),
        '¥' => Some("JPY"),
        _ => None,
    }
}

/// An am/pm marker right after a clock time ending at char `at` (#178):
/// optional whitespace, then `am`/`pm`/`a.m.`/`p.m.` in any case, not
/// followed by a letter or digit. Returns the end of the marker and the
/// marker as written.
fn ampm_suffix(chars: &[char], at: usize) -> Option<(usize, String)> {
    let n = chars.len();
    let mut i = at;
    while i < n && chars[i].is_whitespace() {
        i += 1;
    }
    let start = i;
    if i >= n || !matches!(chars[i], 'a' | 'A' | 'p' | 'P') {
        return None;
    }
    i += 1;
    let dotted = i < n && chars[i] == '.';
    if dotted {
        i += 1;
    }
    if i >= n || !matches!(chars[i], 'm' | 'M') {
        return None;
    }
    i += 1;
    // "a.m." takes its closing dot; "am." leaves it to the sentence.
    if dotted && i < n && chars[i] == '.' {
        i += 1;
    }
    if i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
        return None;
    }
    Some((i, chars[start..i].iter().collect()))
}

/// `SentenceConverter.extract_numbers`, all seven passes in order (plus the
/// grouping and clock-time passes 2b/2c).
fn extract_numbers(t: &Text, lang: &str) -> Result<Vec<Ext>, N2WError> {
    let r = res();
    let n = t.chars.len();
    let mut used = vec![false; n];
    let mut exts: Vec<Ext> = Vec::new();

    // 1. Temperature with degree symbol (°C, °F).
    for m in r.temp_symbol.captures_iter(t.s) {
        let g0 = m.get(0).unwrap();
        let (s, e) = t.span(g0.start(), g0.end());
        if !overlap(&used, s, e) {
            let v = m.get(1).unwrap().as_str().replace(',', ".");
            exts.push(Ext {
                start: s,
                end: e,
                text: g0.as_str().to_string(),
                val: Val::D(v),
                typ: Typ::TempSymbol,
            });
            mark(&mut used, s, e);
        }
    }

    // 2. Temperature with language-specific words.
    if let Some(tre) = r.temp_re(lang) {
        for m in tre.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (s, e) = t.span(g0.start(), g0.end());
            if !overlap(&used, s, e) {
                let v = m.get(1).unwrap().as_str().replace(',', ".");
                exts.push(Ext {
                    start: s,
                    end: e,
                    text: g0.as_str().to_string(),
                    val: Val::D(v),
                    typ: Typ::TempWord,
                });
                mark(&mut used, s, e);
            }
        }
    }

    // 2b. Numbers with thousands grouping (#151), claimed before the
    // ordinal/date/currency/plain passes, which would split them at the
    // separator ("1.000.000" -> ordinal "1." in de, "1,000" -> "1" and "000").
    // A currency symbol right before the number makes it a currency amount.
    let mut i = 0;
    while i < n {
        if used[i] || !(t.chars[i].is_ascii_digit() || t.chars[i] == '-') {
            i += 1;
            continue;
        }
        match grouped_token(&t.chars, i, lang) {
            Some((e, canonical)) if !overlap(&used, i, e) => {
                // A currency symbol right before or after the number
                // ("$1,000", "5 000 000 €", #230); "1,000%" is left as
                // written.
                if e < n && t.chars[e] == '%' {
                    mark(&mut used, i, e + 1);
                    i = e + 1;
                    continue;
                }
                let neg = canonical.starts_with('-');
                let cur = (!neg)
                    .then(|| currency_before(&t.chars, i))
                    .flatten()
                    .filter(|&(k, _)| !overlap(&used, k, i))
                    .map(|(k, code)| (k, e, code))
                    .or_else(|| {
                        (!neg)
                            .then(|| currency_after(&t.chars, e))
                            .flatten()
                            .map(|(k, code)| (i, k, Some(code)))
                    });
                let (s, e, typ) = match cur {
                    Some((s, e, code)) => (s, e, Typ::Currency(code)),
                    None => (i, e, Typ::Number),
                };
                let val = match typ {
                    Typ::Currency(_) => Val::F(pyfloat(&canonical)?),
                    _ => Val::D(canonical),
                };
                exts.push(Ext {
                    start: s,
                    end: e,
                    text: t.slice(s, e),
                    val,
                    typ,
                });
                mark(&mut used, s, e);
                i = e;
            }
            _ => i += 1,
        }
    }

    // 2c. English clock times "10:30" -> "ten thirty" (#152); otherwise the
    // plain pass reads each side and leaves the colon ("ten:thirty").
    // "14:30:45" (seconds) and out-of-range values are left alone. An am/pm
    // suffix, glued or spaced ("10:30pm", "10:30 p.m."), is claimed with the
    // time and kept as written (#178); it requires a 1-12 hour.
    if lang == "en" {
        for m in r.clock.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (s, mut e) = t.span(g0.start(), g0.end());
            let h: u32 = m[1].parse().unwrap_or(99);
            let mi: u32 = m[2].parse().unwrap_or(99);
            let after_colon = s > 0 && t.chars[s - 1] == ':';
            let seconds = e + 1 < n && t.chars[e] == ':' && t.chars[e + 1].is_ascii_digit();
            let suffix = ampm_suffix(&t.chars, e);
            let is_word = |c: char| c.is_alphanumeric() || c == '_';
            let suffix = match suffix {
                Some((se, sfx)) if (1..=12).contains(&h) => {
                    e = se;
                    Some(sfx)
                }
                // Without a suffix the time must end at a word boundary,
                // as the old `\b` required.
                _ if e < n && is_word(t.chars[e]) => continue,
                _ => None,
            };
            if h > 23 || mi > 59 || after_colon || seconds || overlap(&used, s, e) {
                continue;
            }
            exts.push(Ext {
                start: s,
                end: e,
                text: t.slice(s, e),
                val: Val::I(BigInt::from(h)),
                typ: Typ::Time(h, mi, suffix),
            });
            mark(&mut used, s, e);
        }
    }

    // 2d. German "14.30 Uhr" / "14:30 Uhr" -> "vierzehn Uhr dreißig"; any
    // other dotted digit run ("1.5", "3.10.2024") is claimed and left as
    // written (#183). German decimals use a comma, so "1.5" has no standard
    // reading, and the ordinal pass would read "1." and glue on the "5".
    if norm_lang(lang) == "de" {
        for m in r.uhr.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (s, e) = t.span(g0.start(), g0.end());
            let h: u32 = m[1].parse().unwrap_or(99);
            let mi: u32 = m[2].parse().unwrap_or(99);
            let after_sep = s > 0 && matches!(t.chars[s - 1], '.' | ':');
            if h > 23 || mi > 59 || after_sep || overlap(&used, s, e) {
                continue;
            }
            exts.push(Ext {
                start: s,
                end: e,
                text: t.slice(s, e),
                val: Val::I(BigInt::from(h)),
                typ: Typ::UhrTime(h, mi),
            });
            mark(&mut used, s, e);
        }
        let mut i = 0;
        while i < n {
            if !t.chars[i].is_ascii_digit() || (i > 0 && t.chars[i - 1].is_ascii_alphanumeric()) {
                i += 1;
                continue;
            }
            let mut j = i;
            while j < n && t.chars[j].is_ascii_digit() {
                j += 1;
            }
            let mut end = j;
            while end + 1 < n && t.chars[end] == '.' && t.chars[end + 1].is_ascii_digit() {
                end += 1;
                while end < n && t.chars[end].is_ascii_digit() {
                    end += 1;
                }
            }
            if end > j && !overlap(&used, i, end) {
                mark(&mut used, i, end);
            }
            i = end;
        }
    }

    // 2e. Digit groups joined by '-' or '.' (#229), which the later passes
    // would read as one compound number ("1990-2000" -> "...ninety-two
    // thousand") or a decimal with the rest glued on ("192.168.1.1"). A
    // two-group dash run that looks like a range (Y > X, no leading zero,
    // not a 3-4 phone number) is read "X to Y" in English and "X - Y"
    // elsewhere; every other run — dates (25.12.2023, 2023-12-25), IP
    // addresses, versions (v2.0.1), phone numbers (555-1234) — is claimed
    // and left as written.
    let mut i = 0;
    while i < n {
        let c = &t.chars;
        if !c[i].is_ascii_digit() || (i > 0 && c[i - 1].is_ascii_digit()) {
            i += 1;
            continue;
        }
        let digits_from = |mut k: usize| {
            while k < n && c[k].is_ascii_digit() {
                k += 1;
            }
            k
        };
        let mut groups = vec![(i, digits_from(i))];
        let mut j = groups[0].1;
        let sep = (j + 1 < n && matches!(c[j], '-' | '.') && c[j + 1].is_ascii_digit())
            .then(|| c[j]);
        if let Some(sp) = sep {
            while j + 1 < n && c[j] == sp && c[j + 1].is_ascii_digit() {
                let k = digits_from(j + 1);
                groups.push((j + 1, k));
                j = k;
            }
        }
        let end = j;
        let enough = match sep {
            Some('-') => groups.len() >= 2,
            _ => groups.len() >= 3,
        };
        // Inside a longer numeric token ("1.5-3", "-5-3", "1-2.5"): leave it
        // to the other passes.
        let num_sep = |k: usize| matches!(c[k], '-' | '.' | ',');
        let mid_token = (i > 0 && num_sep(i - 1))
            || (end + 1 < n && num_sep(end) && c[end + 1].is_ascii_digit());
        if !enough || mid_token || overlap(&used, i, end) {
            i = end;
            continue;
        }
        let glued = (i > 0 && c[i - 1].is_alphanumeric())
            || (end < n && c[end].is_alphanumeric());
        let range = (sep == Some('-') && groups.len() == 2 && !glued)
            .then(|| {
                let txt = |(a, b): (usize, usize)| t.slice(a, b);
                let (x, y) = (txt(groups[0]), txt(groups[1]));
                let lead0 = |g: &str| g.len() > 1 && g.starts_with('0');
                let phone = x.len() == 3 && y.len() == 4;
                let (xv, yv) = (pyint(&x).ok()?, pyint(&y).ok()?);
                (!lead0(&x) && !lead0(&y) && !phone && yv > xv).then_some((xv, yv))
            })
            .flatten();
        if let Some((xv, yv)) = range {
            exts.push(Ext {
                start: i,
                end,
                text: t.slice(i, end),
                val: Val::I(xv),
                typ: Typ::Range(yv, norm_lang(lang) == "en"),
            });
        }
        mark(&mut used, i, end);
        i = end;
    }

    // 3. Standalone ordinals (registry-driven) — before dates. The ordinal
    // surface form owns its full span (digit + suffix); the date pass then
    // only fires where no ordinal was consumed. The integer is the first
    // non-empty capture group (CJK forms alternate which branch fills it).
    for (ore, form) in r.ordinal_res(lang) {
        for m in ore.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (mut s, mut e) = t.span(g0.start(), g0.end());
            if overlap(&used, s, e) {
                continue;
            }
            // de "1." is an ordinal only when no digit follows the dot
            // ("1.5" is not "Erste5", #183).
            if g0.as_str().ends_with('.') && e < n && t.chars[e].is_ascii_digit() {
                continue;
            }
            let g1 = m.get(1).unwrap();
            let c = &t.chars;
            // The forms added for #232 check their boundaries by hand: the
            // number must not continue a word or a number, the marker must
            // end the word.
            if form != OrdForm::Suffix {
                let (ds, _) = t.span(g1.start(), g1.end());
                if ds == s
                    && ds > 0
                    && (c[ds - 1].is_ascii_alphanumeric()
                        || matches!(c[ds - 1], '.' | ',' | '-'))
                {
                    continue;
                }
                if matches!(form, OrdForm::Symbol | OrdForm::Hyphen)
                    && e < n
                    && c[e].is_alphanumeric()
                {
                    continue;
                }
            }
            let kind = match form {
                OrdForm::Suffix => OrdKind::Masc,
                OrdForm::Symbol if g0.as_str().contains('ª') => OrdKind::Fem,
                OrdForm::Symbol => OrdKind::Masc,
                OrdForm::Dot => {
                    let mut k = e;
                    while k < n && c[k].is_whitespace() {
                        k += 1;
                    }
                    if k == e || k >= n || !c[k].is_lowercase() {
                        continue;
                    }
                    OrdKind::Masc
                }
                OrdForm::Hyphen => {
                    let ending = m.get(2).unwrap().as_str();
                    match (norm_lang(lang).as_str(), ending) {
                        ("ru", "й" | "ый" | "ий" | "ой") | ("uk", "й" | "ий") => OrdKind::Masc,
                        ("ru", "я" | "ая" | "ья") => OrdKind::Fem,
                        // Case forms ("-го", "-м"), "-е" (neuter "1-е место"
                        // or plural "90-е годы"), "-летний"…: as written.
                        _ => {
                            mark(&mut used, s, e);
                            continue;
                        }
                    }
                }
                OrdForm::Prefix | OrdForm::Vi => {
                    (s, e) = t.span(g1.start(), g1.end());
                    if form == OrdForm::Vi {
                        OrdKind::Vi
                    } else {
                        OrdKind::Cardinal
                    }
                }
            };
            // Python `int(groups[0])`; a parse failure is a `ValueError`
            // -> `continue`, not an abort.
            let v = match g1.as_str().parse::<BigInt>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            exts.push(Ext {
                start: s,
                end: e,
                text: t.slice(s, e),
                val: Val::I(v),
                typ: Typ::Ordinal(kind),
            });
            mark(&mut used, s, e);
        }
    }

    // 4. Dates (registry-driven). The day is the first all-digit capture
    // group; the month is the alpha one. For langs whose written day form is
    // "<n>." the trailing period is consumed into the day token.
    if let Some(pats) = r.date_pats(lang) {
        let trails_period = DAY_TOKEN_TRAILS_PERIOD.contains(&lang);
        for p in pats {
            for m in p.re.captures_iter(t.s) {
                // Auto-locate the day (numeric capture).
                let day = (1..m.len()).find_map(|i| {
                    m.get(i).filter(|mm| {
                        let g = mm.as_str();
                        !g.is_empty() && g.chars().all(|c| c.is_ascii_digit())
                    })
                });
                let g = match day {
                    Some(g) => g,
                    None => continue,
                };
                let (ns, mut ne) = t.span(g.start(), g.end());
                let mut day_text = g.as_str().to_string();
                if trails_period && ne < t.chars.len() && t.chars[ne] == '.' {
                    ne += 1;
                    day_text.push('.');
                }
                if overlap(&used, ns, ne) {
                    continue;
                }
                let v = match g.as_str().parse::<BigInt>() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                exts.push(Ext {
                    start: ns,
                    end: ne,
                    text: day_text,
                    val: Val::I(v),
                    typ: if p.is_ordinal {
                        Typ::OrdinalDate
                    } else {
                        Typ::DateNumber
                    },
                });
                mark(&mut used, ns, ne);
            }
        }
    }

    // 5. Years (1900-2100), only right after a "<month> day," prefix in the
    // active language (registry month list, so it extends with each new lang).
    if let Some(months) = r.months_re(lang) {
        let year_ctx =
            Regex::new(&ascii_digits(&format!(r"(?i){}\s+\d+,\s*$", months)))
                .expect("year-ctx regex");
        for m in r.year.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (s, e) = t.span(g0.start(), g0.end());
            if overlap(&used, s, e) {
                continue;
            }
            let before = t.slice(0, s);
            if year_ctx.is_match(before.trim()) {
                exts.push(Ext {
                    start: s,
                    end: e,
                    text: g0.as_str().to_string(),
                    val: Val::I(pyint(g0.as_str())?),
                    typ: Typ::Year,
                });
                mark(&mut used, s, e);
            }
        }
    }

    // 6. Currency: a symbol before the number ("$5", "R$ 3,50"), then
    // after it ("5€", "5 €", #230). A percentage glued to its number
    // ("50%") is claimed and left as written: no converter has a word for
    // it.
    for m in r.currency.captures_iter(t.s) {
        let g2 = m.get(2).unwrap();
        let (i, e) = t.span(g2.start(), g2.end());
        let Some((s, code)) = currency_before(&t.chars, i) else {
            continue;
        };
        if !overlap(&used, s, e) {
            let v = pyfloat(g2.as_str())?;
            exts.push(Ext {
                start: s,
                end: e,
                text: t.slice(s, e),
                val: Val::F(v),
                typ: Typ::Currency(code),
            });
            mark(&mut used, s, e);
        }
    }
    for m in r.amount.captures_iter(t.s) {
        let g0 = m.get(0).unwrap();
        let (s, e) = t.span(g0.start(), g0.end());
        let c = &t.chars;
        let num_sep = |k: usize| matches!(c[k], '-' | '.' | ',');
        if (s > 0 && (c[s - 1].is_ascii_alphanumeric() || num_sep(s - 1)))
            || overlap(&used, s, e)
        {
            continue;
        }
        if e < n && c[e] == '%' {
            mark(&mut used, s, e + 1);
            continue;
        }
        if let Some((end, code)) = currency_after(c, e) {
            exts.push(Ext {
                start: s,
                end,
                text: t.slice(s, end),
                val: Val::F(pyfloat(g0.as_str())?),
                typ: Typ::Currency(Some(code)),
            });
            mark(&mut used, s, end);
        }
    }

    // 7. Plain (possibly negative, possibly decimal) numbers.
    for (s, e) in plain_number_spans(&t.chars) {
        if !overlap(&used, s, e) {
            let text = t.slice(s, e);
            exts.push(Ext {
                start: s,
                end: e,
                val: Val::D(text.replace(',', ".")),
                text,
                typ: Typ::Number,
            });
            mark(&mut used, s, e);
        }
    }

    exts.sort_by_key(|e| e.start);
    Ok(exts)
}

// ------------------------------------------------------------- conversion

/// Whether a core error means "this hook is not ported" (bail out to the
/// Python converter) rather than "Python raised here too" (follow the
/// original's except-ladder). The one NotImplemented that *is* a genuine
/// Python raise on this path is the unknown-currency message, which the
/// core emits verbatim.
fn is_bail(e: &N2WError) -> bool {
    match e {
        N2WError::ReturnsNone => true,
        // A decline (unported hook / out-of-range repr) bails to Python.
        N2WError::Fallback(_) => true,
        // A genuine NotImplementedError (unknown currency, Welsh >100) is a
        // real Python raise — follow convert_number's except ladder instead.
        _ => false,
    }
}

/// Python `str(float)` for the plain-format range, plus the precision the
/// dispatcher derives from it (`abs(Decimal(str(v)).as_tuple().exponent)`).
/// Outside the plain range Python switches to exponent notation, which we
/// do not reimplement — bail to the original.
fn py_float_repr(v: f64) -> Result<(String, u32), N2WError> {
    if !v.is_finite() {
        return Err(N2WError::Fallback("sentence: non-finite float".into()));
    }
    let a = v.abs();
    if a >= 1e16 || (a != 0.0 && a < 1e-4) {
        return Err(N2WError::NotImplemented(
            "sentence: float repr out of plain range".into(),
        ));
    }
    // Rust's Display is shortest-round-trip like Python's repr, and never
    // uses exponent form; only the trailing ".0" on whole values differs.
    let mut s = format!("{}", v);
    if !s.contains('.') {
        s.push_str(".0");
    }
    let prec = s.rsplit('.').next().unwrap().chars().count() as u32;
    Ok((s, prec))
}

struct Ctx<'a> {
    /// `self.lang`, exactly as passed (keys the word tables).
    raw: &'a str,
    /// The converter `num2words(lang=self.lang)` resolves to, if any.
    conv: Option<&'static (dyn Lang + Sync)>,
    /// The English-fallback converter of `convert_number`'s outer except.
    en: &'static (dyn Lang + Sync),
    /// `self.conversion_type == "ordinal"` (any other value acts cardinal).
    ord_mode: bool,
}

impl Ctx<'_> {
    /// A failed resolution makes every inner `num2words` call raise
    /// NotImplementedError, which `convert_number` catches like any other
    /// exception — so surface it as a "genuine Python raise" error.
    fn lang(&self) -> Result<&'static (dyn Lang + Sync), N2WError> {
        self.conv
            .ok_or_else(|| N2WError::Value(format!("lang '{}' unresolved", self.raw)))
    }
}

impl Val {
    fn f(&self) -> f64 {
        match self {
            Val::F(v) => *v,
            Val::D(s) => s.parse().unwrap_or(0.0),
            Val::I(_) => 0.0, // unreachable by construction
        }
    }

    fn i(&self) -> &BigInt {
        match self {
            Val::I(n) => n,
            _ => unreachable_bigint(), // unreachable by construction
        }
    }
}

fn unreachable_bigint() -> &'static BigInt {
    static Z: OnceLock<BigInt> = OnceLock::new();
    Z.get_or_init(|| BigInt::from(0))
}

/// `num2words(v, lang=...)` with a float — the dispatcher's float cardinal
/// path.
fn cardinal_float(l: &(dyn Lang + Sync), v: f64) -> Result<String, N2WError> {
    let (_, prec) = py_float_repr(v)?;
    l.cardinal_float_entry(&FloatValue::Float { value: v, precision: prec }, None)
}

/// `num2words("3.50", lang=...)`: a whole number takes the integer path, a
/// decimal the `Decimal` path, which keeps its digits as written ("three
/// point five zero", #234). `s` is canonical ASCII (`-?\d+(\.\d+)?`).
fn cardinal_str(l: &(dyn Lang + Sync), s: &str) -> Result<String, N2WError> {
    if !s.contains('.') {
        return l.to_cardinal(&pyint(s)?);
    }
    let value = BigDecimal::from_str(s)
        .map_err(|_| N2WError::Fallback("sentence: decimal parse".into()))?;
    let precision = value.as_bigint_and_exponent().1.unsigned_abs() as u32;
    l.cardinal_float_entry(&FloatValue::Decimal { value, precision }, None)
}

/// A [`Val::D`] split into (is negative, magnitude). "-0" is not negative.
fn split_sign(val: &Val) -> Result<(bool, &str), N2WError> {
    match val {
        Val::D(s) => match s.strip_prefix('-') {
            Some(m) => Ok((m.chars().any(|c| c.is_ascii_digit() && c != '0'), m)),
            None => Ok((false, s.as_str())),
        },
        _ => Err(N2WError::Fallback("sentence: number value".into())),
    }
}

/// A [`Val::D`] with "-0" normalised to "0".
fn signed(val: &Val) -> Result<String, N2WError> {
    let (neg, num) = split_sign(val)?;
    Ok(if neg { format!("-{}", num) } else { num.to_string() })
}

/// `num2words(v, to="currency", currency=code, lang=...)` with a float:
/// cents=True, separator/adjective at the language's own defaults.
fn currency_conv(
    l: &(dyn Lang + Sync),
    v: f64,
    code: &str,
) -> Result<String, N2WError> {
    let (s, _) = py_float_repr(v)?;
    // Sentence extractions convert through a Python *float*, so the origin
    // bit is true — SQ renders cents for these.
    let cv = CurrencyValue::parse(&s, false, true, true)?;
    l.to_currency(&cv, code, true, None, l.default_adjective())
}

/// The outer `except Exception: return num2words(value, lang="en")`.
fn fallback_en(ctx: &Ctx, val: &Val) -> Result<String, N2WError> {
    match val {
        Val::I(n) => ctx.en.to_cardinal(n),
        Val::F(v) => cardinal_float(ctx.en, *v),
        Val::D(s) => cardinal_str(ctx.en, s),
    }
}

/// The plain cardinal in the sentence's own language.
fn cardinal_own(ctx: &Ctx, val: &Val) -> Result<String, N2WError> {
    let l = ctx.lang()?;
    match val {
        Val::I(n) => l.to_cardinal(n),
        Val::F(v) => cardinal_float(l, *v),
        Val::D(s) => cardinal_str(l, s),
    }
}

/// `SentenceConverter.convert_number`. When the specific reading fails (an
/// ordinal the language has no word for, e.g. es "0º"), say the cardinal in
/// the sentence's language before falling back to English, so a Spanish
/// sentence never gets an English "zero" spliced into it.
fn convert_number(ctx: &Ctx, val: &Val, typ: &Typ) -> Result<String, N2WError> {
    match convert_inner(ctx, val, typ) {
        Ok(s) => Ok(s),
        Err(e) if is_bail(&e) => Err(e),
        Err(_) => cardinal_own(ctx, val).or_else(|_| fallback_en(ctx, val)),
    }
}

fn convert_inner(ctx: &Ctx, val: &Val, typ: &Typ) -> Result<String, N2WError> {
    match typ {
        Typ::TempSymbol => {
            let (temp_word, celsius_word) =
                temp_words(ctx.raw).unwrap_or(("degrees", "Celsius"));
            let num = signed(val)?;
            let l = ctx.lang()?;
            Ok(format!("{} {} {}", cardinal_str(l, &num)?, temp_word, celsius_word))
        }
        Typ::TempWord => cardinal_str(ctx.lang()?, &signed(val)?),
        Typ::Ordinal(kind) => {
            let l = ctx.lang()?;
            let n = val.i();
            match kind {
                OrdKind::Masc => l.to_ordinal(n),
                OrdKind::Cardinal => l.to_cardinal(n),
                // Vietnamese: "thứ nhất", "thứ tư", else the cardinal.
                OrdKind::Vi if *n == BigInt::from(1) => Ok("nhất".to_string()),
                OrdKind::Vi if *n == BigInt::from(4) => Ok("tư".to_string()),
                OrdKind::Vi => l.to_cardinal(n),
                OrdKind::Fem => {
                    let kw = Kwargs(vec![("gender".into(), KwVal::Str("f".into()))]);
                    match l.to_ordinal_kw(n, &kw) {
                        Ok(s) => Ok(s),
                        Err(e) if !matches!(e, N2WError::Fallback(_)) => Err(e),
                        // pt/it take no gender=: their ordinals are
                        // masculine words in -o, each of which turns -a
                        // ("vigésimo primeiro" -> "vigésima primeira").
                        Err(_) => {
                            let m = l.to_ordinal(n)?;
                            let base = norm_lang(ctx.raw);
                            if matches!(base.as_str(), "pt" | "it")
                                && m.split(' ').all(|w| w.ends_with('o'))
                            {
                                Ok(m.split(' ')
                                    .map(|w| format!("{}a", &w[..w.len() - 1]))
                                    .collect::<Vec<_>>()
                                    .join(" "))
                            } else {
                                Ok(m)
                            }
                        }
                    }
                }
            }
        }
        Typ::OrdinalDate => {
            if ctx.raw == "fr" && *val.i() == BigInt::from(1) {
                return Ok("premier".to_string());
            }
            // German case agreement is applied at replacement time.
            ctx.lang()?.to_ordinal(val.i())
        }
        Typ::DateNumber => ctx.lang()?.to_cardinal(val.i()),
        Typ::Time(h, minute, Some(sfx)) => {
            // "10:30pm" -> "ten thirty pm", "10:00 PM" -> "ten PM" (#178).
            let l = ctx.lang()?;
            let hour = l.to_cardinal(&BigInt::from(*h))?;
            Ok(match *minute {
                0 => format!("{} {}", hour, sfx),
                m if m < 10 => {
                    format!("{} oh {} {}", hour, l.to_cardinal(&BigInt::from(m))?, sfx)
                }
                m => format!("{} {} {}", hour, l.to_cardinal(&BigInt::from(m))?, sfx),
            })
        }
        Typ::Time(h, minute, None) => {
            let l = ctx.lang()?;
            let hour = l.to_cardinal(&BigInt::from(*h))?;
            Ok(match *minute {
                0 if (1..=12).contains(h) => format!("{} o'clock", hour),
                // 24-hour full hours: "thirteen hundred", "zero hundred".
                0 => format!("{} hundred", hour),
                m if m < 10 => format!("{} oh {}", hour, l.to_cardinal(&BigInt::from(m))?),
                m => format!("{} {}", hour, l.to_cardinal(&BigInt::from(m))?),
            })
        }
        Typ::UhrTime(h, minute) => {
            // "14.30 Uhr" -> "vierzehn Uhr dreißig", "14.00 Uhr" ->
            // "vierzehn Uhr"; the hour 1 is "ein Uhr", not "eins Uhr" (#183).
            let l = ctx.lang()?;
            let hour = match *h {
                1 => "ein".to_string(),
                h => l.to_cardinal(&BigInt::from(h))?,
            };
            Ok(match *minute {
                0 => format!("{} Uhr", hour),
                m => format!("{} Uhr {}", hour, l.to_cardinal(&BigInt::from(m))?),
            })
        }
        Typ::Range(y, to) => {
            let l = ctx.lang()?;
            let (x, y) = (l.to_cardinal(val.i())?, l.to_cardinal(y)?);
            Ok(if *to { format!("{} to {}", x, y) } else { format!("{} - {}", x, y) })
        }
        Typ::Year => {
            let l = ctx.lang()?;
            match l.to_year(val.i()) {
                Ok(s) => Ok(s),
                Err(e) if is_bail(&e) => Err(e),
                // Python: fall back to the regular cardinal.
                Err(_) => l.to_cardinal(val.i()),
            }
        }
        Typ::Currency(code) => {
            // `convert` leaves the token as written when this fails (#230).
            let code = code.ok_or_else(|| N2WError::Value("unknown currency".into()))?;
            currency_conv(ctx.lang()?, val.f(), code)
        }
        Typ::Number => {
            let (neg, num) = split_sign(val)?;
            let l = ctx.lang()?;
            if neg {
                // A negative is read like num2words(-7): an integer stays on
                // the integer path (ru "минус семь", not "минус семь целых
                // ноль десятых", #225), with the converter's own negative
                // word (pt_BR "menos", not "minus", #226).
                cardinal_str(l, &signed(val)?)
            } else if !num.contains('.') {
                let n = pyint(num)?;
                if ctx.ord_mode {
                    l.to_ordinal(&n)
                } else {
                    l.to_cardinal(&n)
                }
            } else {
                // Decimals stay cardinal in ordinal mode too: "3,50" has no
                // ordinal reading (pt used to say "terceiro", it "terzo
                // virgola cinque", #231).
                cardinal_str(l, num)
            }
        }
    }
}

// ------------------------------------------------------------ replacement

/// `converted[0].upper() + converted[1:]` (chars, Unicode uppercase).
fn capitalize_first(s: &str) -> String {
    let mut it = s.chars();
    match it.next() {
        None => String::new(),
        Some(c) => {
            let mut out: String = c.to_uppercase().collect();
            out.push_str(it.as_str());
            out
        }
    }
}

/// `re.match(r"(-?\d+(?:[.,]\d+)?)", original)` — the leading number of a
/// temperature-word match (always present by construction).
fn leading_number(text: &str) -> Option<String> {
    let cs: Vec<char> = text.chars().collect();
    let mut j = 0;
    if cs.first() == Some(&'-') {
        j = 1;
    }
    let digits_start = j;
    while j < cs.len() && cs[j].is_ascii_digit() {
        j += 1;
    }
    if j == digits_start {
        return None;
    }
    if j < cs.len() && (cs[j] == '.' || cs[j] == ',') {
        let mut k = j + 1;
        while k < cs.len() && cs[k].is_ascii_digit() {
            k += 1;
        }
        if k > j + 1 {
            j = k;
        }
    }
    Some(cs[..j].iter().collect())
}

/// Abbreviations whose dot does not end a sentence (#235), lowercased,
/// inner dots removed ("e.g." -> "eg", "z.B." -> "zb").
const ABBREVIATIONS: &[&str] = &[
    "approx", "appr", "ca", "cca", "no", "nos", "nr", "num", "vs", "eg", "ie", "cf",
    "fig", "vol", "ch", "chap", "sec", "p", "pp", "dr", "mr", "mrs", "ms", "st",
    "prof", "jr", "sr", "tel",
    "zb", "bzw", "ggf", "usw", "inkl", "zzgl", "vgl", "evtl", "s", "abs", "env", "aprox",
    "pág", "pag", "núm", "nº", "blz", "str", "ок", "стр", "см",
];

/// Whether the text before a number ends a sentence, so the number is
/// capitalised: `!`/`?`, or a `.` that is neither the very first character
/// (".5") nor the dot of an abbreviation ("approx. 5", "No. 5", #235).
fn ends_sentence(bt: &str) -> bool {
    match bt.chars().last() {
        Some('!') | Some('?') => true,
        Some('.') => {
            let head = bt[..bt.len() - 1].trim_end();
            if head.is_empty() {
                return false;
            }
            let word: String = head
                .chars()
                .rev()
                .take_while(|c| c.is_alphabetic() || *c == '.' || *c == 'º')
                .filter(|c| *c != '.')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>()
                .to_lowercase();
            !ABBREVIATIONS.contains(&word.as_str())
        }
        _ => false,
    }
}

/// `num2words`' language-key resolution (full key, "-"->"_", "xx_YY"
/// candidate, first part, first two chars).
fn resolve_num2words_lang(lang: &str) -> Option<&'static (dyn Lang + Sync)> {
    if let Some(l) = get_lang_by_key(lang) {
        return Some(l);
    }
    let normalized = lang.replace('-', "_");
    let mut cur: String;
    if get_lang_by_key(&normalized).is_some() {
        cur = normalized;
    } else {
        let parts: Vec<&str> = normalized.split('_').collect();
        if parts.len() >= 2 {
            let candidate = format!(
                "{}_{}",
                parts[0].to_lowercase(),
                parts[1].to_uppercase()
            );
            if get_lang_by_key(&candidate).is_some() {
                cur = candidate;
            } else {
                cur = parts[0].to_string();
            }
        } else {
            cur = normalized;
        }
    }
    if get_lang_by_key(&cur).is_none() {
        cur = cur.chars().take(2).collect();
    }
    get_lang_by_key(&cur)
}

// ----------------------------------------------------------------- entry

/// `SentenceConverter.convert(sentence, lang, to)` — lang always given.
/// `SentenceConverter.detect_language`, re-based on lingua-rs.
///
/// The Python original chains langdetect (seed=0, prob > 0.7) -> langid
/// (disabled on macOS by default) -> regex heuristics -> "en". langdetect's
/// profiles and sampling cannot be reproduced exactly (and mis-detect short
/// text badly: "Compré 6 manzanas" -> 'en', Chinese -> 'ko'), so detection
/// deliberately swaps the engine for lingua while keeping the same
/// *decision shape*: confident hit -> its ISO code, otherwise the ported
/// regex heuristics, otherwise "en". Detection is best-effort by contract —
/// the sentence corpus is generated with explicit languages, and lang=None
/// rows are expected to differ from Python wherever langdetect itself was
/// wrong.
#[cfg(not(feature = "lang-detect"))]
pub fn detect_language(_text: &str) -> Option<String> {
    // Slim build: no models. The caller declines and the shim falls back to
    // the original Python chain (langdetect -> langid -> heuristics).
    None
}

#[cfg(feature = "lang-detect")]
pub fn detect_language(text: &str) -> Option<String> {
    Some(detect_language_impl(text))
}

#[cfg(feature = "lang-detect")]
fn detect_language_impl(text: &str) -> String {
    use lingua::{LanguageDetector, LanguageDetectorBuilder};
    use std::sync::OnceLock;

    // Script-exclusive languages first: a Unicode range identifies them with
    // near-certainty at zero model cost, so their (large — CJK especially)
    // lingua models need not ship in the binary at all. Precedence within
    // CJK: any kana -> Japanese, any hangul -> Korean, else Han -> Chinese.
    let mut han = false;
    for c in text.chars() {
        let u = c as u32;
        match u {
            0x3040..=0x30FF | 0x31F0..=0x31FF => return "ja".into(), // kana
            0xAC00..=0xD7AF | 0x1100..=0x11FF => return "ko".into(), // hangul
            0x4E00..=0x9FFF | 0x3400..=0x4DBF => han = true,
            0x0600..=0x06FF | 0x0750..=0x077F => return "ar".into(),
            0x0590..=0x05FF => return "he".into(),
            0x0E00..=0x0E7F => return "th".into(),
            0x0900..=0x097F => return "hi".into(), // Devanagari
            0x0370..=0x03FF | 0x1F00..=0x1FFF => return "el".into(),
            _ => {}
        }
    }
    if han {
        return "zh".into();
    }

    static DETECTOR: OnceLock<LanguageDetector> = OnceLock::new();
    // Lazy model loading (the default): the first detection call pays the
    // decompression cost per language, keeping import time and baseline RSS
    // flat for the vast majority of callers who always pass lang=.
    let det = DETECTOR.get_or_init(|| {
        LanguageDetectorBuilder::from_all_languages().build()
    });

    // No numeric confidence gate: lingua's normalized confidences over many
    // candidates rarely clear langdetect's 0.7 on short text, and pushing
    // those cases to the regex heuristics is strictly worse ("I bought 6
    // apples" -> the Italian \bi\b pattern matches English "I"). lingua's
    // own Some/None already encodes "reliable enough".
    if let Some(lang) = det.detect_language_of(text) {
        // ISO 639-1, lowercased; Chinese has no script split here, so
        // "zh" comes out directly (Python maps zh-cn -> zh by hand).
        // lingua's Norwegian is Bokmål -> "nb"; Python's langdetect
        // says "no", and the converter registry treats them as aliases.
        let code = lang.iso_code_639_1().to_string().to_lowercase();
        return if code == "nb" { "no".into() } else { code };
    }

    // The Python regex heuristics, in the same order, case-insensitive.
    static HEUR: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    let heur = HEUR.get_or_init(|| {
        [
            (r"(?i)\b(le|la|les|un|une|de|et|est|pour|avec)\b", "fr"),
            (r"(?i)\b(der|die|das|ein|und|ist|mit|für)\b", "de"),
            (r"(?i)\b(el|la|los|las|y|es|con|para)\b", "es"),
            (r"(?i)\b(il|la|i|le|e|è|con|per)\b", "it"),
            (r"(?i)\b(o|a|os|as|e|é|com|para)\b", "pt"),
            (r"(?i)\b(the|a|an|and|is|with|for|in|on)\b", "en"),
        ]
        .into_iter()
        .map(|(p, l)| (Regex::new(p).expect("static regex"), l))
        .collect()
    });
    for (re, l) in heur {
        if re.is_match(text) {
            return (*l).to_string();
        }
    }
    "en".to_string()
}

/// `num2words_sentence(text)` with `lang=None`: detect, then convert.
pub fn convert_auto(text: &str, to: &str) -> Result<String, N2WError> {
    match detect_language(text) {
        Some(lang) => convert(text, &lang, to),
        None => Err(N2WError::NotImplemented(
            "sentence: built without lang-detect".into(),
        )),
    }
}

pub fn convert(text: &str, lang: &str, to: &str) -> Result<String, N2WError> {
    // Validate language is supported (same message as the Python raise; the
    // shim's NotImplementedError catch re-runs the original, which raises
    // it identically).
    let first2: String = lang.chars().take(2).collect();
    if get_lang_by_key(lang).is_none() && get_lang_by_key(&first2).is_none() {
        return Err(N2WError::NotImplemented(format!(
            "Language '{}' is not supported",
            lang
        )));
    }

    // Native decimal digits are read like ASCII ones; other numeric
    // characters ("m²", "½") are left as written (#228). Extraction runs on
    // the normalised text, replacement splices into the original, so an
    // unconverted token keeps its own digits.
    let norm = normalize_digits(text);
    let t = Text::new(&norm);
    let exts = extract_numbers(&t, lang)?;
    if exts.is_empty() {
        return Ok(text.to_string());
    }

    let ctx = Ctx {
        raw: lang,
        conv: resolve_num2words_lang(lang),
        en: get_lang_by_key("en").expect("en converter"),
        ord_mode: to == "ordinal",
    };

    // Replace from end to beginning to preserve positions.
    let mut result: Vec<char> = text.chars().collect();
    for e in exts.iter().rev() {
        let mut converted = match &e.typ {
            // An amount the language cannot name (no word for the code, or
            // an unknown symbol) is left as written: the bare number would
            // drop the unit, and £ and ¥ would read the same (#230).
            Typ::Currency(_) => match convert_inner(&ctx, &e.val, &e.typ) {
                Ok(s) => s,
                Err(err) if is_bail(&err) => return Err(err),
                Err(_) => continue,
            },
            _ => convert_number(&ctx, &e.val, &e.typ)?,
        };

        // Brazilian Portuguese: a US-style '.' decimal in the token is
        // pronounced "ponto", not the default "vírgula". Pure Python reaches
        // this through a state leak (str_to_number stashes _pending_pointword
        // for any dot-bearing string and the sentence converter reuses it);
        // the token's own separator is the faithful, stateless equivalent.
        // The decimal mark is the last '.'/',' ("1.234,56" is a comma).
        if ctx.raw == "pt_BR"
            && matches!(e.typ, Typ::Number)
            && e.text.chars().rev().find(|&c| c == '.' || c == ',') == Some('.')
        {
            converted = converted.replace("vírgula", "ponto");
        }

        // Sentence-start / after-.!? capitalization, judged on the
        // *original* sentence.
        let needs_cap = if e.start == 0 {
            true
        } else {
            let before = t.slice(0, e.start);
            ends_sentence(before.trim_end())
        };
        if needs_cap && !converted.is_empty() {
            converted = capitalize_first(&converted);
        }

        let replacement = match &e.typ {
            // Smart replacement for temperature words: swap only the number
            // inside the matched text.
            Typ::TempWord if temp_words(ctx.raw).is_some() => {
                match leading_number(&e.text) {
                    Some(numonly) => e.text.replacen(&numonly, &converted, 1),
                    None => converted,
                }
            }
            // German ordinal dates need case agreement.
            Typ::OrdinalDate if ctx.raw == "de" => {
                let before = t.slice(0, e.start);
                let b = before.trim().to_lowercase();
                if b.ends_with("am")
                    || b.ends_with("zum")
                    || b.ends_with("vom")
                    || b.ends_with("den")
                {
                    if !converted.ends_with('n') {
                        converted.push('n');
                    }
                } else if (b.is_empty() || b.ends_with(['.', '!', '?', ':', ';', ',', '(']))
                    && converted.ends_with('e')
                {
                    // No article or preposition before it: strong
                    // nominative, "1. Mai ist frei" -> "Erster Mai" (#195).
                    converted.push('r');
                }
                converted
            }
            _ => converted,
        };

        // Python slicing tolerates end > len (fr's num_end+2 quirk).
        let end = e.end.min(result.len());
        let start = e.start.min(end);
        result.splice(start..end, replacement.chars());
    }

    Ok(result.into_iter().collect())
}
