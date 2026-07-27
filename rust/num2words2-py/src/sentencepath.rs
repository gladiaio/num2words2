//! Port of `num2words2/converters/sentence.py` (SentenceConverter).
//!
//! Language detection (langdetect/langid) stays on the Python side: the
//! shim only routes here when the caller named a language. NotImplemented
//! from here means "fall back to the Python converter".
//!
//! The port mirrors the Python class quirk-for-quirk:
//!   * extraction runs the same seven passes in the same order, with the
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

use std::collections::HashMap;
use std::sync::OnceLock;

use bigdecimal::num_traits::FromPrimitive;
use num2words2_core::base::Lang;
use num2words2_core::{get_lang_by_key, CurrencyValue, FloatValue, N2WError};
use num_bigint::BigInt;
use regex::Regex;

// ------------------------------------------------------------------ tables

/// `lang_registry.NEGATIVE_WORDS` — the negative-marker word per language.
/// Keyed by the raw lang string (Python: `self.negative_words.get(self.lang,
/// "minus")`), default "minus".
const NEGATIVE_WORDS: &[(&str, &str)] = &[
    ("en", "minus"), ("fr", "moins"), ("es", "menos"), ("it", "meno"),
    ("pt", "menos"), ("de", "minus"), ("nl", "min"), ("sv", "minus"),
    ("da", "minus"), ("no", "minus"), ("is", "mínus"), ("fi", "miinus"),
    ("et", "miinus"), ("lt", "minus"), ("lv", "mīnus"),
    ("ru", "минус"), ("uk", "мінус"), ("be", "мінус"), ("bg", "минус"),
    ("pl", "minus"), ("cs", "mínus"), ("sk", "mínus"), ("sl", "minus"),
    ("hr", "minus"), ("sr", "минус"), ("mk", "минус"),
    ("el", "πλην"), ("ro", "minus"), ("hu", "mínusz"), ("tr", "eksi"),
    ("az", "mənfi"),
    ("ar", "سالب"), ("he", "מינוס"), ("fa", "منفی"),
    ("hi", "माइनस"), ("bn", "মাইনাস"), ("ta", "மைனஸ்"), ("te", "మైనస్"),
    ("ja", "マイナス"), ("zh", "负"), ("zh-cn", "负"), ("ko", "마이너스"),
    ("vi", "âm"), ("th", "ติดลบ"),
    ("id", "minus"), ("ms", "minus"),
    ("eo", "minus"), ("la", "minus"), ("rm", "minus"),
];

fn negative_word(lang: &str) -> &'static str {
    NEGATIVE_WORDS
        .iter()
        .find(|(k, _)| *k == lang)
        .map(|(_, v)| *v)
        .unwrap_or("minus")
}

/// `lang_registry._norm_lang` — lowercase/strip, keep as-is when it is a known
/// ordinal or negative key, else fall back to the base subtag. Used for the
/// registry lookups (ordinal/date/month) that Python routes through
/// `get_ordinal_pattern` / `get_date_patterns` / `get_month_names`.
fn norm_lang(lang: &str) -> String {
    let l = lang.trim().to_lowercase();
    if l.is_empty() {
        return "en".to_string();
    }
    if ORDINAL_PATTERNS.iter().any(|(k, _)| *k == l)
        || NEGATIVE_WORDS.iter().any(|(k, _)| *k == l)
    {
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
        (r"({month})\s+(\d+)", true),
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

/// `SentenceConverter.extract_numbers`, all seven passes in order.
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
            let v = pyfloat(m.get(1).unwrap().as_str())?;
            exts.push(Ext {
                start: s,
                end: e,
                text: g0.as_str().to_string(),
                val: Val::F(v),
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
                let v = pyfloat(m.get(1).unwrap().as_str())?;
                exts.push(Ext {
                    start: s,
                    end: e,
                    text: g0.as_str().to_string(),
                    val: Val::F(v),
                    typ: Typ::TempWord,
                });
                mark(&mut used, s, e);
            }
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
            let v = pyfloat(&text)?;
            exts.push(Ext {
                start: s,
                end: e,
                text,
                val: Val::F(v),
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
            Val::I(_) => 0.0, // unreachable by construction
        }
    }

    fn i(&self) -> &BigInt {
        match self {
            Val::I(n) => n,
            Val::F(_) => unreachable_bigint(), // unreachable by construction
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
    }
}

/// `SentenceConverter.convert_number`.
fn convert_number(ctx: &Ctx, val: &Val, typ: &Typ) -> Result<String, N2WError> {
    match convert_inner(ctx, val, typ) {
        Ok(s) => Ok(s),
        Err(e) if is_bail(&e) => Err(e),
        Err(_) => fallback_en(ctx, val),
    }
}

fn convert_inner(ctx: &Ctx, val: &Val, typ: &Typ) -> Result<String, N2WError> {
    match typ {
        Typ::TempSymbol => {
            let (temp_word, celsius_word) =
                temp_words(ctx.raw).unwrap_or(("degrees", "Celsius"));
            let v = val.f();
            let l = ctx.lang()?;
            if v < 0.0 {
                let neg = negative_word(ctx.raw);
                Ok(format!(
                    "{} {} {} {}",
                    neg,
                    cardinal_float(l, v.abs())?,
                    temp_word,
                    celsius_word
                ))
            } else {
                Ok(format!(
                    "{} {} {}",
                    cardinal_float(l, v)?,
                    temp_word,
                    celsius_word
                ))
            }
        }
        Typ::TempWord => {
            let v = val.f();
            let l = ctx.lang()?;
            if v < 0.0 {
                Ok(format!(
                    "{} {}",
                    negative_word(ctx.raw),
                    cardinal_float(l, v.abs())?
                ))
            } else {
                cardinal_float(l, v)
            }
        }
        Typ::Ordinal => ctx.lang()?.to_ordinal(val.i()),
        Typ::OrdinalDate => {
            if ctx.raw == "fr" && *val.i() == BigInt::from(1) {
                return Ok("premier".to_string());
            }
            // German case agreement is applied at replacement time.
            ctx.lang()?.to_ordinal(val.i())
        }
        Typ::DateNumber => ctx.lang()?.to_cardinal(val.i()),
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
            let v = val.f();
            let l = ctx.lang()?;
            if v < 0.0 {
                // abs() keeps the float type, so even -7 renders through
                // the float path (ru: "семь целых ноль десятых").
                let w = if ctx.ord_mode {
                    ordinal_float(l, v.abs())?
                } else {
                    cardinal_float(l, v.abs())?
                };
                Ok(format!("{} {}", negative_word(ctx.raw), w))
            } else if v == v.trunc() {
                let n = BigInt::from_f64(v).ok_or_else(|| {
                    N2WError::Overflow("cannot convert float infinity to integer".into())
                })?;
                if ctx.ord_mode {
                    l.to_ordinal(&n)
                } else {
                    l.to_cardinal(&n)
                }
            } else if ctx.ord_mode {
                ordinal_float(l, v)
            } else {
                cardinal_float(l, v)
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
        if ctx.raw == "pt_BR" && matches!(e.typ, Typ::Number) && e.text.contains('.') {
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
