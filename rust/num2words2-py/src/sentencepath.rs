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
//!   * `convert_number`'s try/except laddering (currency -> cardinal,
//!     year -> cardinal, anything -> English cardinal) is reproduced, with
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
//! that fell back to English "minus" (#226).

use std::collections::HashMap;
use std::sync::OnceLock;

use std::str::FromStr;

use bigdecimal::BigDecimal;
use num2words2_core::base::Lang;
use num2words2_core::strnum::{is_space_group_sep, number_notation, parse_grouped, Grouped};
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
    if ORDINAL_PATTERNS.iter().any(|(k, _)| *k == l) {
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

/// `lang_registry.ORDINAL_PATTERNS` — the integer is captured in group 1 (or,
/// for the CJK bare-form alternations, whichever branch fires). Keyed by the
/// normalised lang. Compiled WITHOUT the ignore-case flag, matching Python's
/// `re.finditer(ordinal_pattern, sentence)` (no flags).
const ORDINAL_PATTERNS: &[(&str, &str)] = &[
    ("en", r"(\d+)(?:st|nd|rd|th)\b"),
    ("de", r"(\d+)(?:\.|te|er)\b"),
    ("nl", r"(\d+)(?:ste|de|e)\b"),
    ("sv", r"(\d+):(?:a|e)\b"),
    ("af", r"(\d+)(?:ste|de)\b"),
    ("fr", r"(\d+)(?:er|ère|e|ème)\b"),
    ("es", r"(\d+)(?:º|°|ª)\b"),
    ("pt", r"(\d+)(?:º|°|ª)\b"),
    ("it", r"(\d+)(?:º|°|ª)\b"),
    ("ca", r"(\d+)(?:r|n|t|è|a)\b"),
    ("el", r"(\d+)(?:ος|η|ο|ός)\b"),
    ("tr", r"(\d+)(?:inci|ıncı|uncu|üncü)\b"),
    ("az", r"(\d+)[-‐](?:ci|cu|cü|cı)\b"),
    ("hi", r"(\d+)(?:वां|वीं|वें)\b"),
    ("bn", r"(\d+)(?:তম|ম|য়|র্থ)\b"),
    ("ta", r"(\d+)(?:வது|ஆம்)\b"),
    ("fa", r"(\d+)(?:مین|ام|م)\b"),
    ("zh", r"第(\d+)"),
    ("ja", r"第(\d+)|(\d+)番目"),
    ("ko", r"제(\d+)|(\d+)번째"),
    ("vi", r"thứ\s*(\d+)"),
    ("th", r"ที่\s*(\d+)"),
    ("id", r"ke[-‐](\d+)"),
    ("ms", r"ke[-‐](\d+)"),
    ("ia", r"(\d+)me\b"),
];

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
    ordinals: Vec<(&'static str, Regex)>,
    dates: Vec<(&'static str, Vec<DatePat>)>,
    year: Regex,
    currency: Regex,
    clock: Regex,
    uhr: Regex,
}

impl Res {
    fn new() -> Res {
        let re = |p: &str| Regex::new(p).expect("static regex");

        let temps = TEMP_PATTERNS
            .iter()
            .map(|(lang, pat, _, _)| (*lang, re(pat)))
            .collect();

        let ordinals = ORDINAL_PATTERNS
            .iter()
            .map(|(lang, pat)| (*lang, re(pat)))
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

    /// Ordinal regex — keyed by the normalised lang (`get_ordinal_pattern`).
    fn ordinal_re(&self, lang: &str) -> Option<&Regex> {
        let n = norm_lang(lang);
        self.ordinals.iter().find(|(k, _)| *k == n).map(|(_, r)| r)
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
    Ordinal,
    OrdinalDate,
    DateNumber,
    Year,
    Currency(char),
    Number,
    /// English clock time `H:MM` as (hour, minute, am/pm suffix as
    /// written, e.g. "pm", "PM", "p.m.").
    Time(u32, u32, Option<String>),
    /// German clock time `H.MM Uhr` / `H:MM Uhr` as (hour, minute) (#183).
    UhrTime(u32, u32),
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
        if i > 0 && chars[i - 1].is_ascii_alphanumeric() {
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
        let ahead_ok = |e: usize| e >= n || !chars[e].is_ascii_alphanumeric();
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

/// A number written with thousands separators starting at char `start`
/// (`1,000,000`, `1.234,56`, `-1 000`; #151). Returns the end of the token and
/// its value as a plain decimal string. Only tokens that really contain
/// grouping qualify, with [`parse_grouped`]'s rules and the language's
/// [`number_notation`] (so `1,000` counts only in `1,000.5` languages, `1.000`
/// only in `1.000,5` ones, #177, and `192.168.1.1` or `1,2,3` never match;
/// a dot not followed by exactly three digits is left to the later passes:
/// de "1. Mai" is an ordinal, de "1.5" is kept as written, #183); the
/// token must not touch an ASCII letter/digit on either side, like pass 7.
/// ASCII spaces are not taken as separators in running text ("between
/// 2 100 and"), only the no-break/thin spaces and apostrophes.
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
        j += 1;
    }
    if j >= n || !chars[j].is_ascii_digit() {
        return None;
    }
    let is_part = |c: char| c.is_ascii_digit() || is_part_sep(c);
    let mut end = j;
    while end < n && is_part(chars[end]) {
        end += 1;
    }
    while end > j && !chars[end - 1].is_ascii_digit() {
        end -= 1;
    }
    if end < n && chars[end].is_ascii_alphanumeric() {
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
                // `[$€£¥]\s*` immediately before the number.
                let mut k = i;
                while k > 0 && t.chars[k - 1].is_whitespace() {
                    k -= 1;
                }
                let sym = (k > 0 && !canonical.starts_with('-'))
                    .then(|| t.chars[k - 1])
                    .filter(|c| matches!(c, '$' | '€' | '£' | '¥'));
                let (s, typ) = match sym {
                    Some(c) if !overlap(&used, k - 1, i) => (k - 1, Typ::Currency(c)),
                    _ => (i, Typ::Number),
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

    // 3. Standalone ordinals (registry-driven) — before dates. The ordinal
    // surface form owns its full span (digit + suffix); the date pass then
    // only fires where no ordinal was consumed. The integer is the first
    // non-empty capture group (CJK forms alternate which branch fills it).
    if let Some(ore) = r.ordinal_re(lang) {
        for m in ore.captures_iter(t.s) {
            let g0 = m.get(0).unwrap();
            let (s, e) = t.span(g0.start(), g0.end());
            if overlap(&used, s, e) {
                continue;
            }
            // de "1." is an ordinal only when no digit follows the dot
            // ("1.5" is not "Erste5", #183).
            if g0.as_str().ends_with('.') && e < n && t.chars[e].is_ascii_digit() {
                continue;
            }
            let grp = (1..m.len())
                .filter_map(|i| m.get(i))
                .map(|mm| mm.as_str())
                .find(|x| !x.is_empty());
            let grp = match grp {
                Some(x) => x,
                None => continue,
            };
            // Python `int(groups[0])`; a parse failure is a `ValueError`
            // -> `continue`, not an abort.
            let v = match grp.parse::<BigInt>() {
                Ok(v) => v,
                Err(_) => continue,
            };
            exts.push(Ext {
                start: s,
                end: e,
                text: g0.as_str().to_string(),
                val: Val::I(v),
                typ: Typ::Ordinal,
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
            Regex::new(&format!(r"(?i){}\s+\d+,\s*$", months)).expect("year-ctx regex");
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

    // 6. Currency.
    for m in r.currency.captures_iter(t.s) {
        let g0 = m.get(0).unwrap();
        let (s, e) = t.span(g0.start(), g0.end());
        if !overlap(&used, s, e) {
            let v = pyfloat(m.get(2).unwrap().as_str())?;
            let sym = m
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .chars()
                .next()
                .unwrap_or('$');
            exts.push(Ext {
                start: s,
                end: e,
                text: g0.as_str().to_string(),
                val: Val::F(v),
                typ: Typ::Currency(sym),
            });
            mark(&mut used, s, e);
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

/// `num2words(v, to="ordinal", lang=...)` with a float.
fn ordinal_float(l: &(dyn Lang + Sync), v: f64) -> Result<String, N2WError> {
    let (_, prec) = py_float_repr(v)?;
    l.ordinal_float_entry(&FloatValue::Float { value: v, precision: prec })
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
        Typ::Ordinal => ctx.lang()?.to_ordinal(val.i()),
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
        Typ::Year => {
            let l = ctx.lang()?;
            match l.to_year(val.i()) {
                Ok(s) => Ok(s),
                Err(e) if is_bail(&e) => Err(e),
                // Python: fall back to the regular cardinal.
                Err(_) => l.to_cardinal(val.i()),
            }
        }
        Typ::Currency(sym) => {
            let l = ctx.lang()?;
            let code = match sym {
                '$' => "USD",
                '€' => "EUR",
                '£' => "GBP",
                '¥' => "JPY",
                _ => "USD",
            };
            match currency_conv(l, val.f(), code) {
                Ok(s) => Ok(s),
                Err(e) if is_bail(&e) => Err(e),
                // Python: fall back to the plain cardinal (float path).
                Err(_) => cardinal_float(l, val.f()),
            }
        }
        Typ::Number => {
            let (neg, num) = split_sign(val)?;
            let v = val.f();
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
            } else if ctx.ord_mode {
                ordinal_float(l, v)
            } else {
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

    // Python's \d matches any Unicode decimal digit and float()/int() accept
    // them; this port only handles ASCII digits — anything else goes back to
    // the original converter.
    if text.chars().any(|c| c.is_numeric() && !c.is_ascii_digit()) {
        return Err(N2WError::NotImplemented(
            "sentence: non-ascii digits".into(),
        ));
    }

    let t = Text::new(text);
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
    let mut result: Vec<char> = t.chars.clone();
    for e in exts.iter().rev() {
        let mut converted = convert_number(&ctx, &e.val, &e.typ)?;

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
            let bt = before.trim_end();
            matches!(bt.chars().last(), Some('.') | Some('!') | Some('?'))
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
