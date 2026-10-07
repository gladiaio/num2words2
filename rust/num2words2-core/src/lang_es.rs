//! Port of `lang_ES.py` (via `lang_EUR` → `base.py`).
//!
//! Engine-style: ES supplies cards + `merge` and lets the base engine drive
//! `splitnum`/`clean`. It overrides `to_cardinal` only to reach the Spanish
//! overflow message; the numeric behaviour is `Num2Word_Base.to_cardinal`.
//!
//! Notable inherited details, all reproduced here:
//!
//! * ES's `setup` calls `gen_high_numwords([], [], lows)` with **empty**
//!   units/tens, so the Latin-prefix machinery contributes nothing and
//!   `high_numwords` is just `lows` (9 entries). ES also does not call
//!   `super().setup()`, so EUR's `["cent"] + ...` list never applies.
//! * `GIGA_SUFFIX = None` disables EUR's `illiard` cards, leaving only the
//!   `illón` (long-scale) series at 10^6, 10^12, ... 10^54. Hence
//!   `MAXVAL = 1000 * 10^54 = 10^57`.
//! * `is_title` is false, so `title()` is the identity.
//!
//! # Float/Decimal cardinal path
//!
//! `Num2Word_ES` does **not** override `to_cardinal_float`, and its
//! `to_cardinal` override is a transparent pass-through for numeric input
//! (the `_pending_ordinal` branch only fires for `"1ro"`-style *string*
//! entry, never for a float or Decimal). So the float/Decimal cardinal path
//! is `Num2Word_Base.to_cardinal_float` unchanged — reached in Python via
//! `super().to_cardinal(value)` → `Base.to_cardinal` → `to_cardinal_float`.
//!
//! The Rust trait default (`floatpath::default_to_cardinal_float`) reproduces
//! that exactly: it calls `self.to_cardinal(&pre)` and `self.to_cardinal(digit)`,
//! which route through this file's `to_cardinal` override just as Python routes
//! through `ES.to_cardinal`. Hence ES needs **no** `to_cardinal_float`
//! override, matching its EUR-family siblings (`gl`, `pt`, `fr`, `it`). Verified
//! byte-for-byte against the `cardinal`/`cardinal_dec` corpus rows, the live
//! interpreter, and a ~7000-case fuzz sweep (float + Decimal), zero mismatches
//! — including the load-bearing f64 artefacts (`1.005`, `2.675`) and the
//! trillion-scale Decimal `98746251323029.99` that must stay exact.
//!
//! # `str_to_number` ("1ro" -> ordinal) is deliberately NOT ported
//!
//! Python's `ES.str_to_number` stashes `_pending_ordinal` and that stash
//! *leaks* observably: `num2words("1ro", to="year")` returns "primero" and
//! `num2words("2da", to="currency")` returns "segunda euros", because ES's
//! `to_year`/`to_currency` call `self.to_cardinal(...)` internally and the
//! pending handshake fires there too. The dispatcher's
//! `ParsedNumber::EsOrdinal` reproduces the handshake only for cardinal mode;
//! porting the hook would therefore *regress* the year/currency string rows.
//! Left unported, every "1ro"/"2da"-style string stays on the Python fallback
//! path (`str_to_number` raises InvalidOperation -> dispatcher code 1), which
//! reproduces the leak byte-for-byte. Verified against `corpus_strings.jsonl`.

use crate::base::{
    default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Kwargs, Lang, N2WError,
    Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_int_parse, ParsedNumber};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.gender_stem`, set once in `setup` and never mutated. `to_ordinal`
/// shadows it with a *local* of the same name, so `merge` always sees "o".
const GENDER_STEM: &str = "o";

/// `self.ords`. Python keys 1e3..1e15 are floats, but `ords[1000]` still hits
/// `1e3` because Python hashes 1000 and 1000.0 alike — so integer keys are a
/// faithful representation.
const ORDS: &[(i64, &str)] = &[
    (1, "primer"),
    (2, "segund"),
    (3, "tercer"),
    (4, "cuart"),
    (5, "quint"),
    (6, "sext"),
    (7, "séptim"),
    (8, "octav"),
    (9, "noven"),
    (10, "décim"),
    (20, "vigésim"),
    (30, "trigésim"),
    (40, "cuadragésim"),
    (50, "quincuagésim"),
    (60, "sexagésim"),
    (70, "septuagésim"),
    (80, "octogésim"),
    (90, "nonagésim"),
    (100, "centésim"),
    (200, "ducentésim"),
    (300, "tricentésim"),
    (400, "cuadrigentésim"),
    (500, "quingentésim"),
    (600, "sexcentésim"),
    (700, "septigentésim"),
    (800, "octigentésim"),
    (900, "noningentésim"),
    (1_000, "milésim"),
    (1_000_000, "millonésim"),
    (1_000_000_000, "billonésim"),
    (1_000_000_000_000, "trillonésim"),
    (1_000_000_000_000_000, "cuadrillonésim"),
];

/// `Num2Word_ES.CURRENCY_FORMS`, as `(code, unit_forms, subunit_forms)`.
///
/// ES declares `CURRENCY_FORMS` in its own class body, which *shadows*
/// `Num2Word_EUR`'s outright — Python merges nothing, and neither
/// `Num2Word_Base.__init__` nor `lang_ES` touches the inherited dict. So
/// EUR's three-form `PLN` (`("zloty", "zlotys", "zlotu")`) is unreachable
/// from `es`; all 171 entries below are plain 2-tuples, verified against the
/// live class.
///
/// The oddities are Python's and are kept verbatim:
///
/// * `ALL`'s singular is `"lek "` — with a trailing space.
/// * `MAD` is `("dírham", "dirhams")`: the accent survives only in the
///   singular.
/// * `CNY`'s subunits are `("fen", "jiaos")` — two different coins, so
///   `0.01` renders "un fen" but `0.34` renders "treinta y cuatro jiaos".
/// * `ZWL` is listed twice in the source; the dict keeps one entry.
const CURRENCY_FORMS: &[(&str, &[&str], &[&str])] = &[
    ("EUR", &["euro", "euros"], &["céntimo", "céntimos"]),
    ("ESP", &["peseta", "pesetas"], &["céntimo", "céntimos"]),
    ("USD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("PEN", &["sol", "soles"], &["céntimo", "céntimos"]),
    ("CRC", &["colón", "colones"], &["céntimo", "céntimos"]),
    ("AUD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("CAD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("GBP", &["libra", "libras"], &["penique", "peniques"]),
    ("RUB", &["rublo", "rublos"], &["kopeyka", "kopeykas"]),
    ("SEK", &["corona", "coronas"], &["öre", "öre"]),
    ("NOK", &["corona", "coronas"], &["øre", "øre"]),
    ("PLN", &["zloty", "zlotys"], &["grosz", "groszy"]),
    ("MXN", &["peso", "pesos"], &["centavo", "centavos"]),
    ("RON", &["leu", "leus"], &["ban", "bani"]),
    ("INR", &["rupia", "rupias"], &["paisa", "paisas"]),
    ("HUF", &["florín", "florines"], &["fillér", "fillér"]),
    ("FRF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("CNY", &["yuan", "yuanes"], &["fen", "jiaos"]),
    ("CZK", &["corona", "coronas"], &["haléř", "haléř"]),
    ("NIO", &["córdoba", "córdobas"], &["centavo", "centavos"]),
    ("VES", &["bolívar", "bolívares"], &["céntimo", "céntimos"]),
    ("BRL", &["real", "reales"], &["centavo", "centavos"]),
    ("CHF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("JPY", &["yen", "yenes"], &["sen", "sen"]),
    ("KRW", &["won", "wones"], &["jeon", "jeon"]),
    ("KPW", &["won", "wones"], &["chon", "chon"]),
    ("TRY", &["lira", "liras"], &["kuruş", "kuruş"]),
    ("ZAR", &["rand", "rands"], &["céntimo", "céntimos"]),
    ("KZT", &["tenge", "tenges"], &["tïın", "tïın"]),
    ("UAH", &["hryvnia", "hryvnias"], &["kopiyka", "kopiykas"]),
    ("THB", &["baht", "bahts"], &["satang", "satang"]),
    ("AED", &["dirham", "dirhams"], &["fils", "fils"]),
    ("AFN", &["afghani", "afghanis"], &["pul", "puls"]),
    ("ALL", &["lek ", "leke"], &["qindarkë", "qindarka"]),
    ("AMD", &["dram", "drams"], &["luma", "lumas"]),
    ("ANG", &["florín", "florines"], &["centavo", "centavos"]),
    ("AOA", &["kwanza", "kwanzas"], &["céntimo", "céntimos"]),
    ("ARS", &["peso", "pesos"], &["centavo", "centavos"]),
    ("AWG", &["florín", "florines"], &["centavo", "centavos"]),
    ("AZN", &["manat", "manat"], &["qəpik", "qəpik"]),
    ("BBD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("BDT", &["taka", "takas"], &["paisa", "paisas"]),
    ("BGN", &["lev", "leva"], &["stotinka", "stotinki"]),
    ("BHD", &["dinar", "dinares"], &["fils", "fils"]),
    ("BIF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("BMD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("BND", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("BOB", &["boliviano", "bolivianos"], &["centavo", "centavos"]),
    ("BSD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("BTN", &["ngultrum", "ngultrum"], &["chetrum", "chetrum"]),
    ("BWP", &["pula", "pulas"], &["thebe", "thebes"]),
    ("BYN", &["rublo", "rublos"], &["kópek", "kópeks"]),
    ("BYR", &["rublo", "rublos"], &["kópek", "kópeks"]),
    ("BZD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("CDF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("CLP", &["peso", "pesos"], &["centavo", "centavos"]),
    ("COP", &["peso", "pesos"], &["centavo", "centavos"]),
    ("CUP", &["peso", "pesos"], &["centavo", "centavos"]),
    ("CVE", &["escudo", "escudos"], &["centavo", "centavos"]),
    ("CYP", &["libra", "libras"], &["céntimo", "céntimos"]),
    ("DJF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("DKK", &["corona", "coronas"], &["øre", "øre"]),
    ("DOP", &["peso", "pesos"], &["centavo", "centavos"]),
    ("DZD", &["dinar", "dinares"], &["céntimo", "céntimos"]),
    ("ECS", &["sucre", "sucres"], &["centavo", "centavos"]),
    ("EGP", &["libra", "libras"], &["piastra", "piastras"]),
    ("ERN", &["nakfa", "nakfas"], &["céntimo", "céntimos"]),
    ("ETB", &["birr", "birrs"], &["céntimo", "céntimos"]),
    ("FJD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("FKP", &["libra", "libras"], &["penique", "peniques"]),
    ("GEL", &["lari", "laris"], &["tetri", "tetris"]),
    ("GHS", &["cedi", "cedis"], &["pesewa", "pesewas"]),
    ("GIP", &["libra", "libras"], &["penique", "peniques"]),
    ("GMD", &["dalasi", "dalasis"], &["butut", "bututs"]),
    ("GNF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("GTQ", &["quetzal", "quetzales"], &["centavo", "centavos"]),
    ("GYD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("HKD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("HNL", &["lempira", "lempiras"], &["centavo", "centavos"]),
    ("HRK", &["kuna", "kunas"], &["lipa", "lipas"]),
    ("HTG", &["gourde", "gourdes"], &["céntimo", "céntimos"]),
    ("IDR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
    ("ILS", &["séquel", "séqueles"], &["agora", "agoras"]),
    ("IQD", &["dinar", "dinares"], &["fils", "fils"]),
    ("IRR", &["rial", "riales"], &["dinar", "dinares"]),
    ("ISK", &["corona", "coronas"], &["eyrir", "aurar"]),
    ("ITL", &["lira", "liras"], &["céntimo", "céntimos"]),
    ("JMD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("JOD", &["dinar", "dinares"], &["piastra", "piastras"]),
    ("KES", &["chelín", "chelines"], &["céntimo", "céntimos"]),
    ("KGS", &["som", "som"], &["tyiyn", "tyiyn"]),
    ("KHR", &["riel", "rieles"], &["céntimo", "céntimos"]),
    ("KMF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("KWD", &["dinar", "dinares"], &["fils", "fils"]),
    ("KYD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("LAK", &["kip", "kips"], &["att", "att"]),
    ("LBP", &["libra", "libras"], &["piastra", "piastras"]),
    ("LKR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
    ("LRD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("LSL", &["loti", "lotis"], &["céntimo", "céntimos"]),
    ("LTL", &["lita", "litas"], &["céntimo", "céntimos"]),
    ("LVL", &["lat", "lats"], &["céntimo", "céntimos"]),
    ("LYD", &["dinar", "dinares"], &["dírham", "dírhams"]),
    ("MAD", &["dírham", "dirhams"], &["céntimo", "céntimos"]),
    ("MDL", &["leu", "lei"], &["ban", "bani"]),
    ("MGA", &["ariary", "ariaris"], &["iraimbilanja", "iraimbilanja"]),
    ("MKD", &["denar", "denares"], &["deni", "denis"]),
    ("MMK", &["kiat", "kiats"], &["pya", "pyas"]),
    ("MNT", &["tugrik", "tugriks"], &["möngö", "möngö"]),
    ("MOP", &["pataca", "patacas"], &["avo", "avos"]),
    ("MRO", &["ouguiya", "ouguiyas"], &["khoums", "khoums"]),
    ("MRU", &["ouguiya", "ouguiyas"], &["khoums", "khoums"]),
    ("MUR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
    ("MVR", &["rufiyaa", "rufiyaas"], &["laari", "laari"]),
    ("MWK", &["kuacha", "kuachas"], &["tambala", "tambalas"]),
    ("MYR", &["ringgit", "ringgit"], &["céntimo", "céntimos"]),
    ("MZN", &["metical", "metical"], &["centavo", "centavos"]),
    ("NAD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("NGN", &["naira", "nairas"], &["kobo", "kobo"]),
    ("NPR", &["rupia", "rupias"], &["paisa", "paisas"]),
    ("NZD", &["dólar", "dólares"], &["centavo", "centavos"]),
    ("OMR", &["rial", "riales"], &["baisa", "baisa"]),
    ("PAB", &["balboa", "balboas"], &["centésimo", "centésimos"]),
    ("PGK", &["kina", "kinas"], &["toea", "toea"]),
    ("PHP", &["peso", "pesos"], &["centavo", "centavos"]),
    ("PKR", &["rupia", "rupias"], &["paisa", "paisas"]),
    ("PLZ", &["zloty", "zlotys"], &["grosz", "groszy"]),
    ("PYG", &["guaraní", "guaranís"], &["céntimo", "céntimos"]),
    ("QAR", &["rial", "riales"], &["dírham", "dírhams"]),
    ("QTQ", &["quetzal", "quetzales"], &["centavo", "centavos"]),
    ("RSD", &["dinar", "dinares"], &["para", "para"]),
    ("RUR", &["rublo", "rublos"], &["kopek", "kopeks"]),
    ("RWF", &["franco", "francos"], &["céntimo", "céntimos"]),
    ("SAR", &["riyal", "riales"], &["halala", "halalas"]),
    ("SBD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("SCR", &["rupia", "rupias"], &["céntimo", "céntimos"]),
    ("SDG", &["libra", "libras"], &["piastra", "piastras"]),
    ("SGD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("SHP", &["libra", "libras"], &["penique", "peniques"]),
    ("SKK", &["corona", "coronas"], &["halier", "haliers"]),
    ("SLL", &["leona", "leonas"], &["céntimo", "céntimos"]),
    ("SRD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("SSP", &["libra", "libras"], &["piastra", "piastras"]),
    ("STD", &["dobra", "dobras"], &["céntimo", "céntimos"]),
    ("SVC", &["colón", "colones"], &["centavo", "centavos"]),
    ("SYP", &["libra", "libras"], &["piastra", "piastras"]),
    ("SZL", &["lilangeni", "emalangeni"], &["céntimo", "céntimos"]),
    ("TJS", &["somoni", "somonis"], &["dirame", "dirames"]),
    ("TMT", &["manat", "manat"], &["tenge", "tenge"]),
    ("TND", &["dinar", "dinares"], &["milésimo", "milésimos"]),
    ("TOP", &["paanga", "paangas"], &["céntimo", "céntimos"]),
    ("TTD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("TWD", &["nuevo dólar", "nuevos dólares"], &["céntimo", "céntimos"]),
    ("TZS", &["chelín", "chelines"], &["céntimo", "céntimos"]),
    ("UAG", &["hryvnia", "hryvnias"], &["kopiyka", "kopiykas"]),
    ("UGX", &["chelín", "chelines"], &["céntimo", "céntimos"]),
    ("UYU", &["peso", "pesos"], &["centésimo", "centésimos"]),
    ("UZS", &["sum", "sum"], &["tiyin", "tiyin"]),
    ("VEF", &["bolívar fuerte", "bolívares fuertes"], &["céntimo", "céntimos"]),
    ("VND", &["dong", "dongs"], &["xu", "xu"]),
    ("VUV", &["vatu", "vatu"], &["nenhum", "nenhum"]),
    ("WST", &["tala", "tala"], &["centavo", "centavos"]),
    ("XAF", &["franco CFA", "francos CFA"], &["céntimo", "céntimos"]),
    ("XCD", &["dólar", "dólares"], &["céntimo", "céntimos"]),
    ("XOF", &["franco CFA", "francos CFA"], &["céntimo", "céntimos"]),
    ("XPF", &["franco CFP", "francos CFP"], &["céntimo", "céntimos"]),
    ("YER", &["rial", "riales"], &["fils", "fils"]),
    ("YUM", &["dinar", "dinares"], &["para", "para"]),
    ("ZMW", &["kwacha", "kwachas"], &["ngwee", "ngwee"]),
    ("ZRZ", &["zaire", "zaires"], &["likuta", "makuta"]),
    ("ZWL", &["dólar", "dólares"], &["céntimo", "céntimos"]),
];

/// `CURRENCY_ADJECTIVES`, inherited unchanged from `Num2Word_EUR` — ES never
/// defines its own, so this is EUR's dict verbatim (English adjectives on a
/// Spanish converter, plus the stray Icelandic `"íslenskar"`).
///
/// Only the float path can reach it: ES's integer branch never passes
/// `adjective` on to the base implementation. See `to_currency`.
const CURRENCY_ADJECTIVES: &[(&str, &str)] = &[
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
];

/// Python's `s[:-n]`, counted in **characters**. "millón"[:-3] must yield
/// "mil", and the 'ó' is two bytes — byte slicing would split it.
/// Python yields "" when the string is shorter than `n`.
fn drop_last_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        return String::new();
    }
    s.chars().take(count - n).collect()
}

/// `verify_ordinal`'s float leg. Python evaluates `int(value)` *first*
/// (inside `value == int(value)`), so NaN raises ValueError and ±inf raises
/// OverflowError before any TypeError; a finite fractional value fails the
/// comparison and raises TypeError with ES's `errmsg_floatord`.
fn float_ordinal_reject(value: &FloatValue) -> N2WError {
    if let FloatValue::Float { value: f, .. } = value {
        if f.is_nan() {
            return N2WError::Value("cannot convert float NaN to integer".into());
        }
        if f.is_infinite() {
            return N2WError::Overflow("cannot convert float infinity to integer".into());
        }
    }
    let shown = match value {
        FloatValue::Float { value: f, .. } => format!("{}", f),
        FloatValue::Decimal { value: d, .. } => crate::strnum::python_decimal_str(d),
    };
    N2WError::Type(format!(
        "El float {} no puede ser tratado como un ordinal.",
        shown
    ))
}

/// Python's `int(val)` on a float/Decimal: truncation toward zero (so
/// `int(-1.5) == -1`, `int(-0.0) == 0`). NaN/inf raise what CPython raises.
fn float_trunc_int(value: &FloatValue) -> Result<BigInt> {
    match value {
        FloatValue::Float { value: f, .. } => {
            if f.is_nan() {
                return Err(N2WError::Value(
                    "cannot convert float NaN to integer".into(),
                ));
            }
            if f.is_infinite() {
                return Err(N2WError::Overflow(
                    "cannot convert float infinity to integer".into(),
                ));
            }
            // Exact: every whole f64 is exactly representable as an integer.
            Ok(BigInt::from_f64(f.trunc()).expect("finite whole f64 converts exactly"))
        }
        FloatValue::Decimal { value: d, .. } => {
            // int(Decimal): with_scale(0) divides by 10^scale in BigInt,
            // which truncates toward zero — exactly Python's int().
            Ok(d.with_scale(0).as_bigint_and_exponent().0)
        }
    }
}

/// `Num2Word_ES.str_to_number` — the "1ro"/"2da" ordinal-suffix handshake.
///
/// A string whose last two chars are an ordinal suffix over an all-digit
/// stem stashes `_pending_ordinal` in Python, which `to_cardinal` consumes
/// to render the ordinal instead. `ParsedNumber::EsOrdinal` carries that
/// across the boundary; anything else takes the plain `Decimal(value)` path.
/// Shared by `es` and every `es_XX` regional variant (which subclass it).
pub fn es_str_to_number(s: &str) -> Result<ParsedNumber> {
    let t = s.trim();
    // Suffix -> gender: masculine ro/do/mo/to/no/vo, feminine ra/da/ma/ta/na/va.
    if t.chars().count() >= 3 {
        let suffix: String = t.chars().rev().take(2).collect::<Vec<_>>()
            .into_iter().rev().collect();
        let gender = match suffix.as_str() {
            "ro" | "do" | "mo" | "to" | "no" | "vo" => Some('m'),
            "ra" | "da" | "ma" | "ta" | "na" | "va" => Some('f'),
            _ => None,
        };
        if let Some(g) = gender {
            let stem: String = {
                let mut c: Vec<char> = t.chars().collect();
                c.truncate(c.len() - 2);
                c.into_iter().collect()
            };
            // Python: `digits.isdigit()` — ASCII (and Unicode Nd) digits only,
            // non-empty. python_int_parse enforces that (no sign/underscore
            // matters here since isdigit rejects them; guard the sign case).
            if !stem.is_empty()
                && stem.chars().all(|ch| crate::strnum::unicode_digit(ch).is_some())
            {
                if let Some(n) = python_int_parse(&stem) {
                    return Ok(ParsedNumber::EsOrdinal { n, gender: g });
                }
            }
        }
    }
    python_decimal_parse(t)
}

/// Feminine unit nouns in the currency table: they take "una"/"veintiuna".
const FEMININE_UNITS: [&str; 6] = ["peseta", "libra", "corona", "rupia", "lira", "hryvnia"];

/// The cardinal as it stands before a currency noun (#253): a final "uno"
/// apocopates before a masculine noun ("un euro", "treinta y un dólares",
/// "veintiún euros") and becomes "una" before a feminine one ("una libra",
/// "veintiuna libras"). Only the last word changes; "un millón" is already
/// apocopated by the cardinal.
fn es_attributive(cardinal: String, feminine: bool) -> String {
    if let Some(stem) = cardinal.strip_suffix("veintiuno") {
        return format!("{}{}", stem, if feminine { "veintiuna" } else { "veintiún" });
    }
    match cardinal.strip_suffix("uno") {
        Some(stem) if stem.is_empty() || stem.ends_with(' ') => {
            format!("{}{}", stem, if feminine { "una" } else { "un" })
        }
        _ => cardinal,
    }
}

pub struct LangEs {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    /// `CURRENCY_FORMS`, materialised once in `new()`. In Python this is a
    /// class attribute built at import time; rebuilding it per `to_currency`
    /// call would put 171 allocations on every conversion.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangEs {
    fn default() -> Self {
        Self::new()
    }
}

impl LangEs {
    pub fn new() -> Self {
        // setup(): high_numwords = gen_high_numwords([], [], lows) == lows,
        // because the comprehension `[u + t for t in tens for u in units]`
        // over empty tens/units is empty.
        let high = ["non", "oct", "sept", "sext", "quint", "cuatr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords, with GIGA_SUFFIX=None (skipped) and
        // MEGA_SUFFIX="illón": cap = 3 + 6*9 = 57; zip(high, range(57, 3, -6)).
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // mirrors range()'s exclusive stop at 3
            }
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}illón", word),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mil"),
                (100, "cien"),
                (90, "noventa"),
                (80, "ochenta"),
                (70, "setenta"),
                (60, "sesenta"),
                (50, "cincuenta"),
                (40, "cuarenta"),
                (30, "treinta"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "veintinueve",
                "veintiocho",
                "veintisiete",
                "veintiséis",
                "veinticinco",
                "veinticuatro",
                "veintitrés",
                "veintidós",
                "veintiuno",
                "veinte",
                "diecinueve",
                "dieciocho",
                "diecisiete",
                "dieciséis",
                "quince",
                "catorce",
                "trece",
                "doce",
                "once",
                "diez",
                "nueve",
                "ocho",
                "siete",
                "seis",
                "cinco",
                "cuatro",
                "tres",
                "dos",
                "uno",
                "cero",
            ],
        );

        // MAXVAL = 1000 * first card = 1000 * 10^54 = 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangEs {
            cards,
            maxval,
            exclude_title: vec!["y".into(), "menos".into(), "punto".into()],
            currency_forms: CURRENCY_FORMS
                .iter()
                .map(|(code, unit, sub)| (*code, CurrencyForms::new(unit, sub)))
                .collect(),
            currency_adjectives: CURRENCY_ADJECTIVES.iter().copied().collect(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check is vacuous for BigInt;
    /// the negative check raises TypeError (`errmsg_negord`).
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// `self.ords[key]`. A miss is Python's KeyError — see `to_ordinal_small`.
    fn ord_stem(&self, key: i64) -> Result<&'static str> {
        ORDS.iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .ok_or_else(|| N2WError::Key(key.to_string()))
    }

    /// `to_ordinal` for `value < 1e18`, where Python's own arithmetic
    /// (`divmod`, `math.log`) stays in machine-int range.
    fn to_ordinal_small(&self, value: i64, gender: &str) -> Result<String> {
        let mut gender_stem = if gender == "f" { "a" } else { "o" };

        let text: String = if value == 0 {
            String::new()
        } else if value <= 10 {
            format!("{}{}", self.ord_stem(value)?, gender_stem)
        } else if value <= 29 && value != 20 {
            // RAE: simple forms up to 30. The accent is dropped
            // (sobreesdrújula ortography) and the stem is forced masculine,
            // but the *unit* keeps the caller's gender.
            gender_stem = "o";
            let dec = (value / 10) * 10;
            format!(
                "{}{}{}",
                self.ord_stem(dec)?.replace('é', "e"),
                gender_stem,
                self.to_ordinal_small(value % 10, gender)?
            )
        } else if value <= 100 {
            let dec = (value / 10) * 10;
            format!(
                "{}{} {}",
                self.ord_stem(dec)?,
                gender_stem,
                self.to_ordinal_small(value - dec, gender)?
            )
        } else if value <= 1000 {
            let cen = (value / 100) * 100;
            format!(
                "{}{} {}",
                self.ord_stem(cen)?,
                gender_stem,
                self.to_ordinal_small(value - cen, gender)?
            )
        } else {
            // Python: dec = 1000 ** int(math.log(int(value), 1000)).
            //
            // This is a *float* log and its rounding is load-bearing, so it is
            // replicated rather than computed exactly. CPython's loghelper
            // converts the int to a double (the exact-frexp path only triggers
            // when that conversion overflows, i.e. >~1.8e308), then
            // math.log(x, base) returns log(x)/log(base). i64 -> f64 rounds
            // half-to-even exactly like PyLong_AsDouble.
            //
            // Near a power of 1000 from below the quotient overshoots — e.g.
            // int(math.log(999999999999999, 1000)) == 5, not 4 — which sends
            // Python into the broken paths handled below. Computing the true
            // floor here would silently *diverge* from Python.
            let k = ((value as f64).ln() / 1000f64.ln()).trunc();
            let dec = 1000i64
                .checked_pow(k as u32)
                .ok_or_else(|| N2WError::Key(format!("1000^{}", k)))?;

            let (high_part, low_part) = (value / dec, value % dec);
            let cardinal = if high_part != 1 {
                self.to_cardinal(&BigInt::from(high_part))?
            } else {
                String::new()
            };

            // Python builds the %-tuple left to right, so ords[dec] is
            // evaluated (and may KeyError) before the recursive call.
            let stem = self.ord_stem(dec)?;

            if high_part == 0 {
                // Only reachable via the math.log overshoot above, where
                // low_part == value and Python recurses until RecursionError.
                // Rust would abort on stack overflow, so surface an error.
                return Err(N2WError::NotImplemented(format!(
                    "RecursionError: to_ordinal({}) recurses forever in Python",
                    value
                )));
            }

            format!(
                "{}{}{} {}",
                cardinal,
                stem,
                gender_stem,
                self.to_ordinal_small(low_part, gender)?
            )
        };

        // "decimooctavo" -> "decimoctavo". Applied at every recursion level,
        // exactly as Python does.
        Ok(text.trim().replace("oo", "o"))
    }

    /// `Num2Word_ES.to_ordinal(value, gender="m")`, gender threaded — the
    /// full entry (verify + the >= 1e18 cardinal fallthrough), shared by the
    /// trait's `to_ordinal` ("m") and `to_ordinal_kw` (caller's gender).
    fn to_ordinal_gender(&self, value: &BigInt, gender: &str) -> Result<String> {
        self.verify_ordinal(value)?;

        // Python: `elif value < 1e18: ... else: text = self.to_cardinal(value)`
        let limit = BigInt::from(10u8).pow(18u32);
        if value >= &limit {
            return Ok(self.to_cardinal(value)?.trim().replace("oo", "o"));
        }

        // Non-negative and < 1e18, so this always fits.
        let v = value
            .to_i64()
            .ok_or_else(|| N2WError::Type(format!("valor fuera de rango: {}", value)))?;
        self.to_ordinal_small(v, gender)
    }
}

/// Spanish has no ordinal for zero: the ported arithmetic renders it as ""
/// (Python's `to_ordinal(0)` really returns the empty string). Every
/// ES-family `to_ordinal` entry point raises this instead (#160).
/// `to_ordinal_num(0)` == "0º" is unaffected.
pub(crate) fn es_reject_zero_ordinal(value: &BigInt) -> Result<()> {
    if value.is_zero() {
        return Err(N2WError::Value(
            "El número 0 no puede ser tratado como un ordinal.".into(),
        ));
    }
    Ok(())
}

impl Lang for LangEs {

    fn str_to_number(&self, s: &str) -> crate::base::Result<crate::strnum::ParsedNumber> {
        crate::lang_es::es_str_to_number(s)
    }
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
        " con"
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }
    fn maxval(&self) -> &BigInt {
        &self.maxval
    }
    fn negword(&self) -> &str {
        "menos "
    }
    fn pointword(&self) -> &str {
        "punto"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext0, cnum) = l;
        let (ntext0, nnum) = r;
        let mut ctext = ctext0.to_string();
        let mut ntext = ntext0.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            ctext = "un".to_string();
        } else if cnum == &hundred && !(nnum % &thousand).is_zero() {
            // "cien" -> "ciento", but only when a non-round remainder follows:
            // 100_000 stays "cien mil".
            ctext.push('t');
            ctext.push_str(GENDER_STEM);
        }

        if nnum < cnum {
            if cnum < &hundred {
                return (format!("{} y {}", ctext, ntext), cnum + nnum);
            }
            return (format!("{} {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &million).is_zero() && !cnum.is_one() {
            // "millón" -> "millones", "billón" -> "billones", ...
            ntext = format!("{}lones", drop_last_chars(&ntext, 3));
        }

        if nnum == &hundred {
            if cnum == &BigInt::from(5) {
                ctext = "quinien".to_string();
                ntext = String::new();
            } else if cnum == &BigInt::from(7) {
                ctext = "sete".to_string();
            } else if cnum == &BigInt::from(9) {
                ctext = "nove".to_string();
            }
            ntext = format!("{}t{}s", ntext, GENDER_STEM);
        } else {
            // Spanish apocopates 'uno' before a noun like 'mil'/'millones':
            // 31000 -> "treinta y un mil", 21000 -> "veintiún mil".
            if nnum >= &thousand {
                if ctext.ends_with("veintiuno") {
                    ctext = format!("{}ún", drop_last_chars(&ctext, 3));
                } else if ctext.ends_with("uno") {
                    ctext = drop_last_chars(&ctext, 1);
                }
            }
            ntext = format!(" {}", ntext);
        }

        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // Python's ES.to_cardinal consults `_pending_ordinal`, which only
        // `str_to_number` ever sets (for "1ro"-style string input). Numeric
        // entry always finds it None, so this is the plain super() call.
        //
        // The overflow check is duplicated from the base engine solely to
        // carry ES's `errmsg_toobig` ("deber", sic).
        let v = value.abs();
        if v >= self.maxval {
            return Err(N2WError::Overflow(format!(
                "abs({}) deber ser inferior a {}.",
                v, self.maxval
            )));
        }
        default_to_cardinal(self, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        crate::lang_es::es_reject_zero_ordinal(value)?;
        self.to_ordinal_gender(value, "m")
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        // ES.to_year ignores suffix/longval and never rejects negatives.
        self.to_cardinal(value)
    }

    // ---- float/Decimal entry (corpus: wholefloat slice) -------------------
    //
    // `Num2Word_ES.to_ordinal`/`to_ordinal_num` start with `verify_ordinal`,
    // so float/Decimal input is accepted only when whole and non-negative:
    // fractional -> TypeError (`errmsg_floatord`), negative whole -> TypeError
    // (`errmsg_negord`). -0.0 *passes* both checks (abs(-0.0) == -0.0) and
    // renders like 0 — to_ordinal(-0.0) raises ValueError like 0 (#160),
    // to_ordinal_num(-0.0) == "-0.0º".
    // `to_year` truncates via `int(val)`: to_year(-1.5) == "menos uno".

    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        match value.as_whole_int() {
            // A whole value renders exactly like the int: Python's dict
            // lookups hash 5.0 like 5, so `ords[5.0]` hits. to_ordinal
            // re-runs the (still reachable) negative check.
            Some(i) => self.to_ordinal(&i),
            None => Err(float_ordinal_reject(value)),
        }
    }

    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            // `"%s%s" % (value, "º")` — the Python str of the value, so
            // "5.00º", "1e+16º", "-0.0º".
            Some(i) if !i.is_negative() => Ok(format!("{}º", repr_str)),
            Some(_) => Err(N2WError::Type(format!(
                "El número negativo {} no puede ser tratado como un ordinal.",
                repr_str
            ))),
            None => Err(float_ordinal_reject(value)),
        }
    }

    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        // `return self.to_cardinal(int(val))` — truncation toward zero, so
        // to_year(0.5) == "cero" and to_year(3.25) == "tres".
        self.to_cardinal(&float_trunc_int(value)?)
    }

    // ---- grammatical kwargs (corpus: kwargs slice) -------------------------

    /// `to_ordinal(value, gender="m")` — ES's one ordinal kwarg. Anything not
    /// in the Python signature -> NotImplemented, so the dispatcher falls back
    /// to Python, which raises the original TypeError. Any gender value other
    /// than "f" (an explicit None, "x", an int, ...) is masculine:
    /// `gender_stem = "a" if gender == "f" else "o"`.
    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        crate::lang_es::es_reject_zero_ordinal(value)?;
        let gender = if kw.str("gender") == Some("f") { "f" } else { "m" };
        self.to_ordinal_gender(value, gender)
    }

    /// `to_ordinal_num(value, gender="m")`: digits + "ª" for "f", "º" else.
    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["gender"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.verify_ordinal(value)?;
        let marker = if kw.str("gender") == Some("f") { "ª" } else { "º" };
        Ok(format!("{}{}", value, marker))
    }

    /// `to_year(val, suffix=None, longval=True)`: both kwargs are accepted
    /// and ignored — the body is `to_cardinal(int(val))` regardless.
    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&["suffix", "longval"]) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.to_year(value)
    }

    /// `Num2Word_ES.to_fraction` (issue #584): idiomatic medio/tercio/cuarto
    /// for denominators 2/3/4, ordinal-as-noun otherwise (a bare "s" plural
    /// appended only when the ordinal isn't s-final already), and the
    /// apocopated "un" for numerator 1. `denominator == 1` (not -1!) or
    /// `numerator == 0` short-circuits to the plain cardinal.
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision("denominator must not be zero".into()));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();

        let den_word = if abs_d == BigInt::from(2) {
            (if abs_n.is_one() { "medio" } else { "medios" }).to_string()
        } else if abs_d == BigInt::from(3) {
            (if abs_n.is_one() { "tercio" } else { "tercios" }).to_string()
        } else if abs_d == BigInt::from(4) {
            (if abs_n.is_one() { "cuarto" } else { "cuartos" }).to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() && !w.ends_with('s') {
                w.push('s');
            }
            w
        };

        // "un" (never "uno") before the denominator noun; the sign is
        // `"%s " % self.negword.strip()`.
        let num_word = if abs_n.is_one() {
            "un".to_string()
        } else {
            self.to_cardinal(&abs_n)?
        };
        let sign = if is_negative { "menos " } else { "" };
        Ok(format!("{}{} {}", sign, num_word, den_word))
    }

    // ---- currency ----------------------------------------------------
    //
    // `currency_precision` is deliberately NOT overridden: `CURRENCY_PRECISION`
    // is `{}` on `Num2Word_Base` and neither `lang_EUR` nor `lang_ES` defines
    // one, so `.get(code, 100)` yields 100 for *every* code. That is why `es`
    // renders JPY with subunits ("doce yenes con treinta y cuatro sen") and
    // cuts KWD/BHD cheques at two digits ("... AND 56/100 DINARES") instead of
    // three — the 0- and 3-decimal special cases never fire for this language.
    //
    // `money_verbose` / `cents_verbose` give the numeral in its attributive
    // form, apocopated before a masculine noun (#253). `cents_terse` /
    // `to_cheque` are not overridden: ES inherits `Num2Word_Base`'s, which the
    // trait defaults already reproduce.

    fn lang_name(&self) -> &str {
        "Num2Word_ES"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        let feminine = self
            .currency_forms
            .get(currency)
            .and_then(|f| f.unit.first())
            .is_some_and(|n| FEMININE_UNITS.contains(&n.as_str()));
        Ok(es_attributive(self.to_cardinal(number)?, feminine))
    }

    /// Every subunit noun in the table is masculine ("un céntimo").
    fn cents_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        Ok(es_attributive(self.to_cardinal(number)?, false))
    }

    /// `Num2Word_EUR.pluralize`: `forms[0]` for exactly 1, `forms[1]` else.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_ES.to_currency`.
    ///
    /// Two paths, and the split is on `isinstance(val, int)`, not on whether
    /// the number happens to be whole: `1` is "un euro", `1.0` is "un euro con
    /// cero céntimos".
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
        // `Num2Word_ES.to_currency` declares `separator=" con"` in its own
        // signature, but the Python->Rust bridge (`__init__.py`) and
        // `bench/diff_test.py` both hand us `kwargs.get("separator", ",")` —
        // `Num2Word_Base`'s default, not ES's. The trait signature cannot tell
        // "caller said ','" from "caller said nothing", so the base default is
        // read as "unspecified" and ES's own default applied. Every float row
        // in the corpus ("doce euros con treinta y cuatro céntimos") depends on
        // this; an explicit `separator=","` is the one input it gets wrong, and
        // the bridge cannot express that case anyway. See `concerns`.
        let separator = if separator == "," { " con" } else { separator };

        // ---- integer path: no cents, ever ----
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms.get(currency) {
                Some(f) => f,
                // Python catches the KeyError and delegates to
                // `super().to_currency`, whose integer branch repeats the same
                // lookup and raises NotImplementedError from it.
                None => {
                    return Err(N2WError::NotImplemented(format!(
                        "Currency code \"{}\" not implemented for \"{}\"",
                        currency,
                        self.lang_name()
                    )))
                }
            };

            // `adjective` is dropped on the floor here — ES's integer branch
            // never consults CURRENCY_ADJECTIVES and never forwards the flag.
            // Only the float path below applies it.
            let _ = adjective;

            // Python uses `negword` raw here, so its trailing space collided
            // with the format string's ("menos  un euro"). Stripped like every
            // other minus site so the output has single spaces (#160).
            let minus_str = if v.is_negative() { self.negword().trim() } else { "" };
            let abs_val = v.abs();

            // The attributive numeral: "un euro", "veintiún euros", "una libra"
            // (#253).
            let money_str = self.money_verbose(&abs_val, currency)?;
            let currency_str = if abs_val.is_one() {
                forms.unit.first()
            } else {
                forms.unit.get(1).or_else(|| forms.unit.first())
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?;

            return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                .trim()
                .to_string());
        }

        // ---- float path: base implementation ----
        // Python patched "uno" -> "un" afterwards with six blind string
        // substitutions keyed on a few nouns ("cero libras con uno penique"
        // slipped through); the attributive numeral now comes from
        // `money_verbose`/`cents_verbose` instead (#253).
        let result =
            crate::currency::default_to_currency(self, val, currency, cents, separator, adjective)?;
        Ok(result)
    }
}
