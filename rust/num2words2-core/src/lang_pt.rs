//! Port of `lang_PT.py` (European Portuguese), via its
//! `Num2Word_EUR` → `Num2Word_Base` ancestry.
//!
//! Shape: **engine**. `Num2Word_PT.setup` supplies `high_numwords` /
//! `mid_numwords` / `low_numwords` and overrides `merge`, so `cards` is built
//! and the default `to_cardinal` (`splitnum`/`clean`/`merge`) drives the
//! conversion. PT then *post-processes* the engine's output with a regex loop
//! (see [`LangPt::to_cardinal`]), which is the only reason `to_cardinal` is
//! overridden here at all.
//!
//! Inheritance chain actually used:
//!   * `Num2Word_EUR.set_high_numwords` — long-scale illiard/illion pairing.
//!     PT sets `GIGA_SUFFIX = None`, so **only** the `MEGA_SUFFIX` ("ilião")
//!     half of that method fires: one card per `10^(n-3)` for
//!     `n in range(57, 3, -6)`, i.e. 10^54 … 10^6.
//!   * `Num2Word_EUR.gen_high_numwords(units, tens, lows)` — PT calls it as
//!     `gen_high_numwords([], [], lows)`. With `tens == []` the comprehension
//!     `[u + t for t in tens for u in units]` is empty, so the Latin elision
//!     table never runs and the result is simply `lows`. Inlined below.
//!   * `Num2Word_Base.verify_ordinal` — the negative-input `TypeError`.
//!   * `Num2Word_Base.to_cardinal` — overflow check + `splitnum`/`clean`.
//!
//! The currency surface is ported; see [`LangPt::to_currency`]. `to_fraction`
//! is PT's own override (issue #584): idiomatic "meio"/"terço" for halves and
//! thirds, `cardinal(n) + ordinal(d)` (+"s" unless it already ends in "s")
//! otherwise — see [`Lang::to_fraction`] below. PT_BR inherits it (and ports
//! it separately, dispatching to its own cardinal/ordinal).
//!
//! # float/Decimal entry points (corpus: wholefloat slice)
//!
//! * `to_cardinal(float/Decimal)` — Base routing: whole -> int path,
//!   fractional -> `to_cardinal_float`. The trait default already does this;
//!   only the trailing EXTS sweep needed porting (see
//!   [`LangPt::to_cardinal_float`]).
//! * `to_ordinal(float/Decimal)` — **`value = int(value)` truncates toward
//!   zero before `verify_ordinal`**, so `to_ordinal(2.5)` == "segundo" and
//!   `to_ordinal(-1.5)` raises the *negative* TypeError, never the float one.
//!   `to_ordinal(0.5)` and `to_ordinal(-0.0)` truncate to 0, which Python
//!   renders as "" and the port rejects with ValueError (#160).
//! * `to_ordinal_num(float/Decimal)` — `verify_ordinal` on the **raw** value:
//!   fractional -> TypeError (`errmsg_floatord`), numerically negative ->
//!   TypeError (`errmsg_negord`). `-0.0` passes both checks
//!   (`abs(-0.0) == -0.0` numerically) and yields "-0.0º".
//! * `to_year(float/Decimal)` — `val < 0` is a numeric test (so -0.0 takes
//!   the positive branch), and the negative branch renders
//!   `to_cardinal(abs(val)) + " antes de Cristo"`, floats included:
//!   `to_year(-1.5)` == "um vírgula cinco antes de Cristo".
//!
//! `cardinal_from_decimal` is deliberately left at its trait default, which
//! now routes to the shared `floatpath` module rather than raising. That is
//! what PT wants: Python reaches the fractional-cents branch as
//! `self.to_cardinal(float(right))`, i.e. straight through the float path, and
//! the default reproduces it using this language's own `pointword`
//! ("vírgula") — `to_currency(1.011, currency="EUR")` gives
//! "um euro e um vírgula um cêntimos" in both. No corpus row exercises it
//! (every pt currency value has ≤ 2 decimals), so it is checked by hand only.
//!
//! # Faithfully reproduced Python oddities
//!
//! This is a port, not a rewrite. All of the following are what Python emits,
//! verified against the interpreter:
//!
//! 1. **`cards[10^6]` is literally `"milião"`** — a typo for "milhão". The
//!    library patches it up inside `merge` with an explicit
//!    `if ntext == "milião": ntext = "milhão"`. Reproduced verbatim rather
//!    than fixed at the table, because the *unpatched* spelling is observable:
//!    the `nnum % 1000000` branch rewrites it to "milhões" by string surgery
//!    (`ntext[:-4] + "lhões"` → "mi" + "lhões") **before** that correction
//!    would ever apply.
//! 2. **`merge`'s `if nnum < cnum` has two identical arms.** Python tests
//!    `if cnum < 100` and then returns the exact same expression in both the
//!    `if` and the fall-through. Collapsed into one branch here — this is the
//!    single place the Rust is not a statement-for-statement transcription,
//!    and it is provably behaviour-preserving.
//! 3. **Nonstandard ordinal spellings**, kept as-is: `ords[2][4]` is
//!    "quadrigentésimo" (standard PT is "quadringentésimo") and `ords[2][7]`
//!    is "septigentésimo" (standard is "septingentésimo"). The frozen corpus
//!    confirms both (`ordinal(700)` == "septigentésimo").
//! 4. **`to_ordinal(0)` returned the empty string** in Python — every digit
//!    maps through `ords[idx % 3][0]` == `""`. The port raises ValueError
//!    instead: Portuguese has no ordinal for zero (#160).
//! 5. **`to_ordinal` drops a leading "primeiro "** whenever the value is not
//!    exactly 1 (`result[9:]`), to avoid "primeiro milésimo". This is why
//!    `ordinal(1000)` == "milésimo" but `ordinal(2000)` == "segundo milésimo".
//! 6. **`to_ordinal` raised `KeyError` for values with ≥ 19 digits
//!    (fixed, #173)**, because `thousand_separators` stops at key 15. The
//!    port extends the table along the cardinal's long scale ("trilionésimo"
//!    10^18 … "nonilionésimo" 10^54) and raises `OverflowError` at `MAXVAL`
//!    (10^57), like the cardinal.
//!
//! # Cross-call mutable state: `to_currency` leaks `self.negword`
//!
//! **This port deliberately diverges from the frozen corpus on two rows.**
//! Read this before "fixing" them.
//!
//! `Num2Word_PT.to_currency` chops a character off `self.negword` and restores
//! it afterwards — with no `try/finally`:
//!
//! ```python
//! backup_negword = self.negword
//! self.negword = self.negword[:-1]          # "menos " -> "menos"
//! result = super().to_currency(val, ...)    # raises for an unknown code
//! self.negword = backup_negword             # never runs on the raise
//! ```
//!
//! `CONVERTER_CLASSES` holds one shared instance, so every float-path call
//! that raises eats one more character, permanently and process-wide:
//!
//! ```text
//! >>> num2words(-12.34, lang='pt', to='currency', currency='INR')
//! 'menos doze rupias e trinta e quatro paisas'
//! >>> for _ in range(6):                     # 6 x NotImplementedError
//! ...     try: num2words(1.0, lang='pt', to='currency', currency='KWD')
//! ...     except NotImplementedError: pass   # negword: "menos " -> ""
//! >>> num2words(-12.34, lang='pt', to='currency', currency='INR')
//! ' doze rupias e trinta e quatro paisas'    # same input, minus sign gone
//! >>> num2words(-1, lang='pt')
//! ' um'                                      # cardinal is corrupted too
//! ```
//!
//! This is `lang_SL`'s `ordflag` bug (FINDINGS.md §8a) with a different
//! attribute, and the fix upstream is the same one line: `try/finally`.
//!
//! The corpus **bakes the corruption in**, because `gen_corpus.py` walks
//! currencies in the order EUR, USD, GBP, JPY, KWD, BHD, INR, CNY, CHF on one
//! shared instance: KWD and BHD contribute 7 raising float rows each, which is
//! more than enough to erode "menos " to "" before INR is reached. Two rows
//! record the damage —
//!
//! ```text
//! {"lang":"pt","to":"currency:INR","arg":"-12.34","out":" doze rupias e trinta e quatro paisas"}
//! {"lang":"pt","to":"currency:CNY","arg":"-12.34","out":" doze yuans e trinta e quatro fen"}
//! ```
//!
//! — and the tail `w2n_cardinal` rows corroborate it independently: pt's are
//! recorded as `" um"` / `" sete"`, not `"menos um"` / `"menos sete"`, because
//! by then the same instance had lost its negword.
//!
//! Reproducing this would mean giving `LangPt` process-global mutable state
//! (an atomic — the trait hands out `&self` and `get_lang` returns a
//! `&'static (dyn Lang + Sync)`), making output depend on how many earlier
//! calls happened to fail, and on thread interleaving where Python at least
//! has the GIL. It would also silently corrupt `to_cardinal`, a path the
//! contract freezes. So this port stays stateless and matches a **pristine**
//! `Num2Word_PT`, per the precedent set for 8a/8b: "the Rust port declines to
//! reproduce the leak ... This is the one accepted divergence."
//!
//! Net: 115/117 pt currency+cheque corpus rows match; the 2 above are the
//! leak. Flagged rather than silently absorbed — see the agent report.
//!
//! `float2tuple` writes `self.precision`, but that one is genuinely restored
//! (a `finally` in `to_cardinal_float`) and is unreachable from
//! cardinal/ordinal/ordinal_num/year on integer input.

use crate::base::{
    check_maxval, default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError,
    Result,
};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::{default_to_cardinal_float, FloatValue};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::HashMap;

/// `self.hundreds`, keys 1..=9. Index 0 is absent in Python.
const HUNDREDS: [&str; 9] = [
    "cento",
    "duzentos",
    "trezentos",
    "quatrocentos",
    "quinhentos",
    "seiscentos",
    "setecentos",
    "oitocentos",
    "novecentos",
];

/// `self.ords`: three dicts (units, tens, hundreds), keys 0..=9 with 0 → "".
///
/// "quadrigentésimo" and "septigentésimo" are the library's spellings; see the
/// module docs.
const ORDS: [[&str; 10]; 3] = [
    [
        "", "primeiro", "segundo", "terceiro", "quarto", "quinto", "sexto", "sétimo", "oitavo",
        "nono",
    ],
    [
        "",
        "décimo",
        "vigésimo",
        "trigésimo",
        "quadragésimo",
        "quinquagésimo",
        "sexagésimo",
        "septuagésimo",
        "octogésimo",
        "nonagésimo",
    ],
    [
        "",
        "centésimo",
        "ducentésimo",
        "tricentésimo",
        "quadrigentésimo",
        "quingentésimo",
        "seiscentésimo",
        "septigentésimo",
        "octigentésimo",
        "nongentésimo",
    ],
];

/// The `ext` list `to_cardinal` sweeps, in Python's tuple order. Order is
/// load-bearing: "mil" runs before "milhões", and each pass rewrites the
/// string the next pass sees.
const EXTS: [&str; 7] = [
    "mil",
    "milhão",
    "milhões",
    "mil milhões",
    "bilião",
    "biliões",
    "mil biliões",
];

/// The `ext` list `to_currency` sweeps to insert "de", in Python's tuple
/// order. Distinct from [`EXTS`], which `to_cardinal` uses: no "mil", no
/// "mil milhões", but "trilião"/"triliões" instead.
const CURRENCY_EXTS: [&str; 6] = [
    "milhão",
    "milhões",
    "bilião",
    "biliões",
    "trilião",
    "triliões",
];

/// `Num2Word_PT.CURRENCY_FORMS` — PT's **own** class-body dict.
///
/// PT *defines* `CURRENCY_FORMS`, so it shadows `Num2Word_EUR`'s rather than
/// sharing it. That is what saves it from the trap `Num2Word_EN.__init__`
/// sets: EN mutates EUR's dict *in place* at import time, and the 16 classes
/// that merely inherit it silently pick up English plurals ("två euros") plus
/// ~24 extra codes. PT sees none of that — its EUR entry is its own
/// `("euro", "euros")`, and CHF/KWD/BHD/... stay absent. Dumped from the live
/// interpreter rather than transcribed, per the porting contract.
///
/// Every entry is a 2-tuple on both sides, which is why the `cr1[1]` index in
/// `to_currency` and `pluralize`'s `forms[1]` can never raise IndexError here.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    // Python module-level: DOLLAR = ("dólar", "dólares"); CENTS = (...)
    const DOLLAR: [&str; 2] = ["dólar", "dólares"];
    const CENTS: [&str; 2] = ["cêntimo", "cêntimos"];

    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("AUD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert(
        "BRL",
        CurrencyForms::new(&["real", "reais"], &["centavo", "centavos"]),
    );
    m.insert("CAD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("NZD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("HKD", CurrencyForms::new(&DOLLAR, &CENTS));
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &CENTS));
    m.insert(
        "GBP",
        CurrencyForms::new(&["libra", "libras"], &["péni", "pence"]),
    );
    m.insert("CNY", CurrencyForms::new(&["yuan", "yuans"], &["fen", "fen"]));
    m.insert("JPY", CurrencyForms::new(&["iene", "ienes"], &["sen", "sen"]));
    m.insert(
        "INR",
        CurrencyForms::new(&["rupia", "rupias"], &["paisa", "paisas"]),
    );
    m.insert(
        "RUB",
        CurrencyForms::new(&["rublo", "rublos"], &["copeque", "copeques"]),
    );
    m.insert("KRW", CurrencyForms::new(&["won", "wons"], &["jeon", "jeons"]));
    m.insert(
        "MXN",
        CurrencyForms::new(&["peso", "pesos"], &["centavo", "centavos"]),
    );
    m.insert("USD", CurrencyForms::new(&DOLLAR, &CENTS));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged — PT defines none.
///
/// Unlike `CURRENCY_FORMS` this really is the shared EUR dict, but nothing
/// mutates it (verified against the live interpreter: it still matches the
/// `lang_EUR.py` literal), so the source is what runs.
///
/// Mostly dead weight, kept because it is what PT inherits. Only AUD, CAD,
/// USD, RUB, MXN, INR, JPY and KRW are reachable — `to_currency` looks the
/// *forms* up first and raises for any code PT lacks — and even those are
/// reachable only on the float path, since PT's int path ignores `adjective`
/// outright (see [`LangPt::to_currency`]).
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

/// `to_currency`'s "milhões euros" → "milhões de euros" sweep, shared by both
/// its int and float paths.
///
/// Python, with `unit` being `currency_str` (int path) or `cr1[1]` (float):
///
/// ```python
/// for ext in ("milhão", "milhões", "bilião", "biliões", "trilião", "triliões"):
///     if re.match(".*{} (?={})".format(ext, unit), result):
///         result = result.replace("{}".format(ext), "{} de".format(ext), 1)
/// ```
///
/// The test is a plain substring search, not a regex, and that is exact rather
/// than approximate:
///
/// * `re.match(".*X (?=Y)")` — the leading `.*` backtracks over any prefix
///   (and the subject never contains a newline, which `.` would not cross), so
///   the pattern succeeds iff `"X Y"` occurs somewhere in `result`.
/// * Neither the six `ext` literals nor any word in PT's `CURRENCY_FORMS`
///   contains a regex metacharacter, so interpolating `unit` into a pattern —
///   which is what Python does — cannot change the meaning.
///
/// Checked against the real `re.match` over 2,016 (currency, value, adjective)
/// combinations spanning all 14 PT codes plus 4 unknown ones: 0 mismatches.
///
/// Note `replacen(ext, ..., 1)` rewrites the **first** `ext` in the string,
/// which need not be the occurrence the test matched. That is exactly what
/// `str.replace(old, new, 1)` does, and it is preserved rather than tightened.
fn insert_de(mut result: String, unit: &str) -> String {
    for ext in CURRENCY_EXTS.iter() {
        if result.contains(&format!("{} {}", ext, unit)) {
            result = result.replacen(ext, &format!("{} de", ext), 1);
        }
    }
    result
}

/// `self.thousand_separators`. Python stops at 15 (a `KeyError` from 10^18,
/// #173); the port follows the cardinal's long scale up to the 10^54 card,
/// one "-ilionésimo" per "-ilião" stem, so every value below `MAXVAL`
/// (10^57) has an ordinal.
fn thousand_separator(idx: usize) -> Option<&'static str> {
    match idx {
        3 => Some("milésimo"),
        6 => Some("milionésimo"),
        9 => Some("milésimo milionésimo"),
        12 => Some("bilionésimo"),
        15 => Some("milésimo bilionésimo"),
        18 => Some("trilionésimo"),
        21 => Some("milésimo trilionésimo"),
        24 => Some("quatrilionésimo"),
        27 => Some("milésimo quatrilionésimo"),
        30 => Some("quintilionésimo"),
        33 => Some("milésimo quintilionésimo"),
        36 => Some("sextilionésimo"),
        39 => Some("milésimo sextilionésimo"),
        42 => Some("septilionésimo"),
        45 => Some("milésimo septilionésimo"),
        48 => Some("octilionésimo"),
        51 => Some("milésimo octilionésimo"),
        54 => Some("nonilionésimo"),
        _ => None,
    }
}

/// `self.hundreds[n]` for n in 1..=9.
fn hundreds_word(n: &BigInt) -> Option<&'static str> {
    let i = n.to_u32()?;
    if (1..=9).contains(&i) {
        Some(HUNDREDS[(i - 1) as usize])
    } else {
        None
    }
}

/// Python's `s[:-4]` — a **character** slice, not a byte slice.
///
/// Every string this is applied to ends in "ilião", whose "ã" is two bytes in
/// UTF-8, so byte slicing would split a codepoint and corrupt the stem
/// ("trilião" → "tri", "bilião" → "bi", "milião" → "mi"). Python returns ""
/// when the string is 4 chars or shorter.
fn drop_last_4_chars(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();
    if cs.len() <= 4 {
        String::new()
    } else {
        cs[..cs.len() - 4].iter().collect()
    }
}

/// Python `\w` for `str` patterns: Unicode alphanumerics plus underscore.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Python's `int(value)` on a float/Decimal: truncation toward zero
/// (`int(-1.5) == -1`, `int(-0.0) == 0`). NaN/inf raise what CPython raises —
/// unreachable through the shim (its `_finite` guard keeps them on the Python
/// side), kept for fidelity.
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
            // int(Decimal): with_scale(0) divides the mantissa by 10^scale in
            // BigInt, which truncates toward zero — exactly Python's int().
            Ok(d.with_scale(0).as_bigint_and_exponent().0)
        }
    }
}

/// Python's `val < 0` on a float/Decimal — a *numeric* comparison, unlike
/// `FloatValue::is_negative()` which is sign-bit aware. `-0.0 < 0` is False,
/// so `to_year(-0.0)` renders through the positive branch.
fn fv_numeric_negative(v: &FloatValue) -> bool {
    match v {
        FloatValue::Float { value, .. } => *value < 0.0,
        FloatValue::Decimal { value, .. } => value.is_negative(),
    }
}

/// Python's `abs(val)` in the value's own domain (float stays float,
/// Decimal stays Decimal), keeping the recorded precision.
fn fv_abs(v: &FloatValue) -> FloatValue {
    match v {
        FloatValue::Float { value, precision } => FloatValue::Float {
            value: value.abs(),
            precision: *precision,
        },
        FloatValue::Decimal { value, precision } => FloatValue::Decimal {
            value: value.abs(),
            precision: *precision,
        },
    }
}

/// Python's `re.sub("\\s+", " ", s)`.
fn collapse_ws(s: &str) -> String {
    let mut out = String::new();
    let mut in_ws = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out
}

/// Evaluates `re.match(".*{ext} e \\w*entos? (?=.*e)", result)` as a boolean,
/// without a regex engine (the crate takes no regex dependency).
///
/// Reasoning behind the hand-rolled matcher:
///
/// * `re.match(".*P")` is equivalent to `re.search(P)` here — the leading `.*`
///   backtracks over any prefix and the subject never contains a newline (the
///   one character `.` will not cross). So this scans every occurrence of the
///   literal `"{ext} e "` rather than anchoring at 0.
/// * `\w*entos?` cannot match a space, and it is followed by a mandatory
///   literal `" "`. So it must consume *exactly* the word sitting between that
///   occurrence and the next space. Letting `\w*` backtrack, such a word
///   matches iff it is all word characters and ends in "ento" or "entos".
///   (If there is no following space — i.e. the word ends the string — the
///   mandatory `" "` fails and the occurrence is rejected. That is precisely
///   why `to_cardinal(1100)` keeps its "e": "mil e cem" has no trailing space.)
/// * The trailing `(?=.*e)` is a lookahead applied *after* that space, so it
///   asks only whether some 'e' character occurs later in the string.
fn re_match_ext(result: &str, ext: &str) -> bool {
    let hay: Vec<char> = result.chars().collect();
    let needle: Vec<char> = format!("{} e ", ext).chars().collect();
    if needle.len() > hay.len() {
        return false;
    }
    for i in 0..=(hay.len() - needle.len()) {
        if hay[i..i + needle.len()] != needle[..] {
            continue;
        }
        let j = i + needle.len();
        // `\w*entos?` runs to the next space; that space is the literal " ".
        let k = match hay[j..].iter().position(|c| *c == ' ') {
            Some(off) => j + off,
            None => continue,
        };
        let w = &hay[j..k];
        if !w.iter().all(|c| is_word_char(*c)) {
            continue;
        }
        let word: String = w.iter().collect();
        if !(word.ends_with("ento") || word.ends_with("entos")) {
            continue;
        }
        // Lookahead `(?=.*e)` at the position just past the literal space.
        if hay[k + 1..].iter().any(|c| *c == 'e') {
            return true;
        }
    }
    false
}

pub struct LangPt {
    cards: Cards,
    maxval: BigInt,
    exclude_title: Vec<String>,
    /// Built once here, not per call — a per-call table cost an earlier
    /// version of this port 10x against the Python it replaces.
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangPt {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPt {
    pub fn new() -> Self {
        // setup(): high_numwords = gen_high_numwords([], [], lows) == lows,
        // because `tens` is empty (see module docs). Note "quatr", not the
        // "quadr" that lang_EUR's own setup() uses — PT overrides the list.
        let high = ["non", "oct", "sept", "sext", "quint", "quatr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords, with GIGA_SUFFIX = None:
        //   cap = 3 + 6*len(high) = 57
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       cards[10**(n-3)] = word + MEGA_SUFFIX      # "ilião"
        // zip stops at the shorter side; here both are 9 long, and the
        // `n <= 3` break mirrors the range running out.
        let cap = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break;
            }
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}ilião", word),
            );
            n -= 6;
        }

        set_mid_numwords(
            &mut cards,
            &[
                (1000, "mil"),
                (100, "cem"),
                (90, "noventa"),
                (80, "oitenta"),
                (70, "setenta"),
                (60, "sessenta"),
                (50, "cinquenta"),
                (40, "quarenta"),
                (30, "trinta"),
            ],
        );
        set_low_numwords(
            &mut cards,
            &[
                "vinte",
                "dezanove",
                "dezoito",
                "dezassete",
                "dezasseis",
                "quinze",
                "catorze",
                "treze",
                "doze",
                "onze",
                "dez",
                "nove",
                "oito",
                "sete",
                "seis",
                "cinco",
                "quatro",
                "três",
                "dois",
                "um",
                "zero",
            ],
        );

        // MAXVAL = 1000 * list(self.cards.keys())[0]. The OrderedDict's first
        // key is the first inserted, i.e. the 10^54 card, so MAXVAL == 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        LangPt {
            cards,
            maxval,
            exclude_title: vec!["e".into(), "vírgula".into(), "menos".into()],
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_Base.verify_ordinal`. The float check is unreachable for
    /// integer input, so only the negative check survives.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

impl Lang for LangPt {
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
        " e"
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
        "vírgula"
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
        let billion = BigInt::from(1_000_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, nnum.clone());
            }
            ctext = "um".to_string();
        } else if cnum == &hundred && !(nnum % &thousand).is_zero() {
            // Python: `cnum == 100 and not nnum % 1000 == 0`, which parses as
            // `cnum == 100 and not (nnum % 1000 == 0)` — `==` binds tighter
            // than `not`. So: "cem" becomes "cento" only when what follows is
            // not a whole thousand ("cento e um", but "cem mil").
            ctext = "cento".to_string();
        }

        if nnum < cnum {
            // Python branches on `cnum < 100` and returns the identical
            // expression either way; see module docs, item 2.
            return (format!("{} e {}", ctext, ntext), cnum + nnum);
        } else if (nnum % &billion).is_zero() && cnum > &BigInt::one() {
            ntext = format!("{}liões", drop_last_4_chars(&ntext));
        } else if (nnum % &million).is_zero() && cnum > &BigInt::one() {
            ntext = format!("{}lhões", drop_last_4_chars(&ntext));
        }

        // Corrects the "milião" card typo — but only when the plural surgery
        // above did not already rewrite it. See module docs, item 1.
        if ntext == "milião" {
            ntext = "milhão".to_string();
        }
        if nnum == &hundred {
            // `self.hundreds[cnum]`. Reachable only with cnum in 2..=9:
            // ("cem", 100) is emitted solely by the `elem == 100` arm of
            // splitnum, which requires 100 <= value < 1000, so the sibling
            // carries div = value/100 in 1..=9 — and div == 1 already
            // returned above. Larger cnum (e.g. 1000 for "mil e cem") takes
            // the `nnum < cnum` return and never reaches here. Verified by
            // instrumenting the real dict over ~80k values: only 2..=9 hit.
            ctext = hundreds_word(cnum)
                .unwrap_or_else(|| {
                    panic!(
                        "lang_pt merge: self.hundreds[{}] missing (Python raises KeyError)",
                        cnum
                    )
                })
                .to_string();
            ntext = String::new();
        } else {
            ntext = format!(" {}", ntext);
        }

        (format!("{}{}", ctext, ntext), cnum * nnum)
    }

    /// `Num2Word_Base.to_cardinal`, then PT's regex clean-up sweep.
    ///
    /// The engine renders every junction with "e", giving
    /// "mil e cento e catorze"; this loop deletes the "e" after a scale word
    /// when a *hundreds* group follows and something ("... e ...") still
    /// follows that. Hence "mil cento e catorze", but "mil e cem" and
    /// "mil e um" keep theirs.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut result = default_to_cardinal(self, value)?;

        for ext in EXTS.iter() {
            if re_match_ext(&result, ext) {
                // Python's str.replace with no count: every occurrence goes.
                // That is deliberate — it is what turns
                // "cem milhões e duzentos mil e duzentos e dez" into
                // "cem milhões duzentos mil duzentos e dez".
                result = result.replace(&format!("{} e", ext), ext);
            }
        }

        Ok(result)
    }

    /// `Num2Word_PT.to_cardinal` on **float / Decimal** input.
    ///
    /// Python routes non-integers through `to_cardinal`, whose `super()` call
    /// lands on `Num2Word_Base.to_cardinal` → `to_cardinal_float` (neither PT
    /// nor EUR overrides the latter, so the shared [`default_to_cardinal_float`]
    /// reproduces it exactly). The catch is that `PT.to_cardinal` then runs its
    /// EXTS sweep over the **whole** returned string — fraction included — and
    /// that trailing sweep is *not* a no-op:
    ///
    /// The recursive `to_cardinal(pre)` already swept the integer part in
    /// isolation, but a "…{scale} e {…}entos" junction sitting at the very end
    /// of that part is left untouched, because the pattern's mandatory trailing
    /// space is missing (nothing follows the "entos" word). Once the fraction
    /// is appended, that word gains a following space, and any fractional digit
    /// word carrying an 'e' — "zero", "sete", "nove" — satisfies the pattern's
    /// `(?=.*e)` lookahead. So the second, whole-string sweep can fire where the
    /// first could not: `1200.7` → "mil duzentos vírgula sete", while `1200.5`
    /// stays "mil e duzentos vírgula cinco" and the integer `1200` stays
    /// "mil e duzentos". Verified against the live interpreter.
    ///
    /// This outer sweep is the *sole* reason PT diverges from the shared float
    /// path. Everything upstream of it — `float2tuple`, the banker's-rounding
    /// heuristic, the `precision_override` handling, the zero-`pre` negword
    /// insertion — is inherited unchanged, so it is delegated rather than
    /// re-derived. The sweep body is identical to [`LangPt::to_cardinal`]'s and
    /// is inlined here to leave that frozen integer path untouched.
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        let mut result = default_to_cardinal_float(self, value, precision_override)?;

        for ext in EXTS.iter() {
            if re_match_ext(&result, ext) {
                result = result.replace(&format!("{} e", ext), ext);
            }
        }

        Ok(result)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        // Portuguese has no ordinal for zero; Python returns "" here (#160).
        if value.is_zero() {
            return Err(N2WError::Value("Cannot treat 0 as ordinal.".into()));
        }

        // The scale table ends at the 10^54 card, like the cardinal (#173).
        check_maxval(value, &self.maxval)?;

        // Python reassigns `value = str(value)` and later compares it to "1".
        let value_str = value.to_string();
        let mut result: Vec<&str> = Vec::new();
        let mut thousand_sep: &str = "";

        for (idx, ch) in value_str.chars().rev().enumerate() {
            if idx != 0 && idx % 3 == 0 {
                // `self.thousand_separators[idx]`. Covers every idx below
                // MAXVAL (checked above), so the `None` arm is unreachable.
                thousand_sep = match thousand_separator(idx) {
                    Some(s) => s,
                    None => return Err(N2WError::Key(format!("{}", idx))),
                };
            }

            if ch != '0' && !thousand_sep.is_empty() {
                // Held back until a non-zero digit turns up, so that 6000000
                // is "sexto milionésimo" and not "sexto milionésimo milésimo".
                result.push(thousand_sep);
                thousand_sep = "";
            }

            let d = ch.to_digit(10).expect("BigInt renders as digits only") as usize;
            result.push(ORDS[idx % 3][d]);
        }

        result.reverse();
        let joined = result.join(" ");
        let collapsed = collapse_ws(joined.trim());

        if collapsed.starts_with("primeiro") && value_str != "1" {
            // Python's `result[9:]` drops "primeiro " — a char slice, and the
            // tail routinely starts with "milésimo"/"bilionésimo", so this
            // must not be a byte slice.
            return Ok(collapsed.chars().skip(9).collect());
        }

        Ok(collapsed)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!(
                "{} antes de Cristo",
                self.to_cardinal(&value.abs())?
            ));
        }
        self.to_cardinal(value)
    }

    // ---- float/Decimal entry points --------------------------------------
    //
    // `cardinal_float_entry` stays at the trait default (Base's
    // `assert int(value) == value` routing: whole -> int path, fractional ->
    // to_cardinal_float), which is exactly what PT inherits from
    // Num2Word_Base. Only ordinal / ordinal_num / year need overrides.

    /// `Num2Word_PT.to_ordinal` on float/Decimal input.
    ///
    /// The first statement is `value = int(value)` — truncation toward zero —
    /// and `verify_ordinal` runs on the *truncated* int. So `2.5` ->
    /// "segundo", `0.5` / `-0.0` -> ValueError (zero, #160), and `-1.5` raises the
    /// negative-num TypeError (never the float one). Values of `MAXVAL`
    /// (10^57) and up raise OverflowError (module docs, item 6).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_ordinal(&float_trunc_int(value)?)
    }

    /// `Num2Word_PT.to_ordinal_num` on float/Decimal input:
    /// `verify_ordinal(value)` on the **raw** value, then `"%sº" % value`.
    ///
    /// Check order is Python's: the float test (`value == int(value)`) comes
    /// first, so `-1.5` raises `errmsg_floatord`, not `errmsg_negord`. Both
    /// messages interpolate `str(value)` — the binding's `repr_str`. The
    /// negative test is `not abs(value) == value`, a numeric comparison:
    /// `-0.0` passes (abs(-0.0) == -0.0) and yields "-0.0º".
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        match value.as_whole_int() {
            None => Err(N2WError::Type(format!(
                "Cannot treat float {} as ordinal.",
                repr_str
            ))),
            Some(i) if i.is_negative() => Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                repr_str
            ))),
            Some(_) => Ok(format!("{}º", repr_str)),
        }
    }

    /// `Num2Word_PT.to_year` on float/Decimal input. `val < 0` is numeric
    /// (-0.0 goes positive), and the negative branch renders
    /// `to_cardinal(abs(val))` — whole floats through the int path,
    /// fractional ones through the float grammar: `to_year(-1.5)` ==
    /// "um vírgula cinco antes de Cristo".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        if fv_numeric_negative(value) {
            return Ok(format!(
                "{} antes de Cristo",
                self.cardinal_float_entry(&fv_abs(value), None)?
            ));
        }
        self.cardinal_float_entry(value, None)
    }

    // ---- fractions --------------------------------------------------------

    /// `Num2Word_PT.to_fraction` (issue #584).
    ///
    /// Deviations from the Base default, all transcribed:
    ///   * idiomatic halves/thirds: "meio"/"meios", "terço"/"terços";
    ///   * the "s" plural is appended only when the ordinal does not already
    ///     end in "s" (Base appends unconditionally);
    ///   * `abs_n == 1` short-circuits the numerator to the literal "um".
    /// `denominator == 1` / `numerator == 0` return the *signed* cardinal,
    /// before any of that. `self.to_ordinal(abs_d)` can still raise
    /// (OverflowError at `MAXVAL`; Python's KeyError at 10^18+ is #173).
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        if denominator.is_zero() {
            return Err(N2WError::ZeroDivision(
                "denominator must not be zero".into(),
            ));
        }
        if denominator.is_one() || numerator.is_zero() {
            return self.to_cardinal(numerator);
        }
        let is_negative = numerator.is_negative() ^ denominator.is_negative();
        let abs_n = numerator.abs();
        let abs_d = denominator.abs();

        let den_word = if abs_d == BigInt::from(2) {
            if abs_n.is_one() { "meio" } else { "meios" }.to_string()
        } else if abs_d == BigInt::from(3) {
            if abs_n.is_one() { "terço" } else { "terços" }.to_string()
        } else {
            let mut w = self.to_ordinal(&abs_d)?;
            if !abs_n.is_one() && !w.ends_with('s') {
                w.push('s');
            }
            w
        };
        let num_word = if abs_n.is_one() {
            "um".to_string()
        } else {
            self.to_cardinal(&abs_n)?
        };
        let sign = if is_negative {
            format!("{} ", self.negword().trim())
        } else {
            String::new()
        };
        Ok(format!("{}{} {}", sign, num_word, den_word))
    }

    // ---- currency -------------------------------------------------------
    //
    // PT inherits `to_cheque`, `_money_verbose`, `_cents_verbose` and
    // `_cents_terse` unchanged from `Num2Word_Base` (the trait defaults
    // already mirror those), and `pluralize` from `Num2Word_EUR`. It defines
    // its own `CURRENCY_FORMS` and its own `to_currency`.
    //
    // `currency_precision` is deliberately NOT overridden: neither PT nor EUR
    // defines `CURRENCY_PRECISION`, so it is Base's empty dict and *every*
    // code divides by 100 — including JPY, which therefore keeps a sen
    // subunit here ("doze ienes e trinta e quatro sen") instead of being the
    // 0-decimal currency it is elsewhere. The corpus pins that. It also means
    // `default_to_currency`'s `divisor == 1` branch is unreachable for PT, and
    // that KWD/BHD are not 3-decimal here — they are simply absent from the
    // table, which is a different code path entirely (see `to_currency`).

    fn lang_name(&self) -> &str {
        "Num2Word_PT"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Reached only from the float path — PT's int path inlines the same rule
    /// instead of calling this. Python indexes the tuple directly, so a
    /// one-form entry with `n != 1` raises IndexError; every PT entry has two
    /// forms, so that is unreachable, but it is mapped to `Index` rather than
    /// left to panic in case the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_PT.to_currency`.
    ///
    /// Two entirely separate paths, split on `isinstance(val, int)`:
    ///
    /// * **int** — hand-rolled here, and it never shows cents. Two quirks
    ///   worth naming, both preserved:
    ///   1. An unknown currency code does **not** raise. Python's
    ///      `except KeyError: return self.to_cardinal(val)` silently drops the
    ///      currency name, so `num2words(100, lang="pt", to="currency",
    ///      currency="KWD")` is just `"cem"`. The corpus pins this for
    ///      KWD/BHD/CHF, which is why those codes have *passing* int rows and
    ///      *raising* float rows.
    ///   2. `adjective` is ignored outright — the int branch never consults
    ///      `CURRENCY_ADJECTIVES`, so `adjective=True` is a no-op on ints
    ///      while it works on floats.
    /// * **float/Decimal** — delegated to `Num2Word_Base.to_currency`, then
    ///   post-processed with the same "de" sweep.
    ///
    /// Python also chops and restores `self.negword` around both paths. That
    /// is not modelled: every read goes through `.strip()`, so "menos " and
    /// "menos" are indistinguishable and the chop is dead — *except* when the
    /// wrapped call raises and the restore is skipped, which is the leak the
    /// module docs describe and this port declines to reproduce.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        if let CurrencyValue::Int(v) = val {
            let forms = match self.currency_forms(currency) {
                Some(f) => f,
                // `except KeyError: return self.to_cardinal(val)` — note it
                // takes the *signed* value, so -5 KWD is "menos cinco".
                None => return self.to_cardinal(v),
            };

            let minus_str = if v.is_negative() {
                format!("{} ", self.negword().trim())
            } else {
                String::new()
            };
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            // PT re-implements EUR's `pluralize` inline rather than calling
            // it. Same rule, so the duplication is only historical — but the
            // `len(cr1) > 1` guard it carries is real, and transcribed.
            let currency_str = if abs_val.is_one() {
                &forms.unit[0]
            } else {
                forms.unit.get(1).unwrap_or(&forms.unit[0])
            };

            let result = insert_de(
                format!("{}{} {}", minus_str, money_str, currency_str),
                currency_str,
            );
            // `return result.strip()`. The float path below has no strip().
            return Ok(result.trim().to_string());
        }

        // `super().to_currency(...)` — Num2Word_EUR does not override it, so
        // this lands on Num2Word_Base. `separator=None` means the caller
        // omitted the kwarg; resolving it is the default body's job, and this
        // override replaces that body, so it is done here.
        let result = crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )?;

        // `cr1, _ = self.CURRENCY_FORMS[currency]`, *after* the super() call —
        // so an unknown code has already raised NotImplementedError above and
        // this KeyError is unreachable.
        let forms = match self.currency_forms(currency) {
            Some(f) => f,
            None => return Err(N2WError::Key(format!("'{}'", currency))),
        };
        // Python indexes `cr1[1]` unconditionally: the plural form, whatever
        // `pluralize` actually picked. So at val=1.0 the sweep tests against
        // "euros" while the rendered string says "euro" — harmless, since a
        // value of exactly 1 can never contain "milhão". Unreachable
        // IndexError, as every PT entry is a 2-tuple.
        let plural = match forms.unit.get(1) {
            Some(p) => p,
            None => return Err(N2WError::Index("tuple index out of range".into())),
        };
        Ok(insert_de(result, plural))
    }
}
