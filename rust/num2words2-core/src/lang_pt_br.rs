//! Port of `lang_PT_BR.py` (Brazilian Portuguese).
//!
//! Registry key `"pt_BR"` → `Num2Word_PT_BR` (confirmed in `__init__.py`).
//!
//! Shape: **hybrid, reported as self-contained**. `Num2Word_PT_BR` overrides
//! `to_cardinal` with its own short-scale algorithm, but that algorithm calls
//! `super().to_cardinal(...)` for every value below 10^9, and *that* super is
//! `Num2Word_PT.to_cardinal` → `Num2Word_Base.to_cardinal`, i.e. the real
//! `splitnum`/`clean`/`merge` engine driven by `Num2Word_PT_BR.merge`.
//!
//! So both halves are live and both are ported:
//!   * [`LangPtBr::super_to_cardinal`] == `Num2Word_PT.to_cardinal`
//!     ([`base::default_to_cardinal`] + PT's regex fixups). `cards`, `maxval`
//!     and `merge` are supplied so the engine runs.
//!   * [`Lang::to_cardinal`] == `Num2Word_PT_BR.to_cardinal`, the short-scale
//!     bilhão/trilhão wrapper on top.
//!
//! # Inheritance chain
//!
//! `Num2Word_PT_BR` → `Num2Word_PT` → `Num2Word_EUR` → `Num2Word_Base`.
//! (`lang_EU` / `Num2Word_EU` is Basque and is *not* in this chain despite the
//! similar name.) Resolved members:
//!   * `setup`: EUR sets `high_numwords`, PT overwrites it, PT_BR overwrites
//!     `low_numwords` + `thousand_separators`. Only the final values matter —
//!     `set_numwords()` runs after `setup()` returns.
//!   * `set_high_numwords`: EUR's. `GIGA_SUFFIX = None` (PT) so the illiard
//!     rung is skipped entirely; only `MEGA_SUFFIX = "ilião"` fires.
//!   * `merge`: PT_BR's (below). PT's is fully shadowed.
//!   * `to_ordinal` / `to_ordinal_num` / `to_year`: PT's, inherited unchanged.
//!   * `verify_ordinal`: `Num2Word_Base`'s → TypeError on negatives.
//!
//! # Scale of the card table
//!
//! PT's `setup` replaces EUR's Latin-prefix tower with
//! `gen_high_numwords([], [], lows)`; with `tens == []` the comprehension is
//! empty, so `high_numwords` collapses to `lows` itself:
//! `["non","oct","sept","sext","quint","quatr","tr","b","m"]`. EUR's
//! `set_high_numwords` then pairs them with `range(57, 3, -6)` under
//! `MEGA_SUFFIX`, giving cards 10^54 "nonilião" … 10^6 "milião", and
//! `MAXVAL = 1000 * 10^54 = 10^57`. Verified against the interpreter.
//!
//! **Fixed (gladiaio/num2words2#159): `MAXVAL` is enforced.** Python's
//! `Num2Word_PT_BR.to_cardinal` only ever hands `super()` a value `< 10^9`
//! (see the per-branch notes below), so base's `>= MAXVAL` test never fired
//! and `to_cardinal(10**57)` returned "um bilhão trilhões trilhões trilhões
//! trilhões" although `maxval("pt_BR")` is 10^57. The port checks `MAXVAL` at
//! the top of [`Lang::to_cardinal`] instead, so 10^57 and above raise
//! `OverflowError`. No card above 10^6 is ever consulted; the table is still
//! built in full for fidelity.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are exactly what Python
//! emits, verified against the interpreter:
//!
//! 1. **"milião" leaks out of `merge` and is patched by string replace.** The
//!    card for 10^6 is literally `"milião"` (PT's European `MEGA_SUFFIX`).
//!    PT's `merge` repairs it with `if ntext == "milião": ntext = "milhão"`,
//!    but **PT_BR's `merge` drops that line**, so `super().to_cardinal(10**6)`
//!    really returns `"um milião"`. PT_BR instead scrubs it after the fact
//!    with `.replace("milião", "milhão")` at each call site — see
//!    [`fix_milhao`]. Reachable: `to_cardinal(1_001_500_000)` routes
//!    `1_500_000` through `super()`, which yields "um milião e quinhentos mil"
//!    before the scrub. The companion `.replace("miliões", "milhões")` is dead
//!    code under PT_BR's `merge` (it has no `"liões"` branch), but is kept.
//! 2. **Two regex passes fight over the same comma.** PT's `to_cardinal`
//!    *deletes* the "e" in "mil e cento" → "mil cento" ([`pt_drops_e`]), then
//!    PT_BR's `to_cardinal` *inserts* a comma → "mil, cento" ([`ptbr_comma`]).
//!    Net effect for 1101: "mil e cento e um" → "mil cento e um" → "mil,
//!    cento e um". Both passes are needed; neither alone is correct.
//! 3. **PT_BR's trailing regex loop only ever runs for `value < 10^6`.** Every
//!    other branch of `to_cardinal` `return`s early. That is why 1234567890 is
//!    "um bilhão, duzentos e trinta e quatro milhões quinhentos e sessenta e
//!    sete mil oitocentos e noventa" — comma after "bilhão" (added by hand by
//!    the billions branch), but *no* comma after "milhões" (the loop that
//!    would have added one is skipped by the early return).
//! 4. **`to_ordinal` mixes `self`- and `super`-style helpers inconsistently**,
//!    and its sibling `to_cardinal` does too: the millions branch converts the
//!    *count* with `super()` but the *remainder* with `self`, while the
//!    billions branch does the exact opposite. Preserved verbatim.
//! 5. **`ords[2][4]` is "quadrigentésimo"**, not the standard
//!    "quadringentésimo". Kept verbatim (drives `to_ordinal(123456)`).
//! 6. `to_ordinal` strips a leading "primeiro " with `result[9:]` so that
//!    1000 is "milésimo" rather than "primeiro milésimo" — but the guard is
//!    the *string* `value != "1"`, not the number.
//!
//! # The currency surface
//!
//! Three layers, all live, all reached on every call:
//!
//! ```text
//! Num2Word_PT_BR.to_currency   normalise whole float/Decimal -> int;
//!                              then patch in " de" using the *Brazilian*
//!                              scale words (bilhão/trilhão)
//!   -> Num2Word_PT.to_currency int and float split here; patches in " de"
//!                              using the *European* words (bilião/trilião)
//!     -> Num2Word_Base.to_currency   floats only
//! ```
//!
//! `CURRENCY_FORMS` is a fresh dict in PT_BR's own class body, so it shadows
//! PT's and EUR's outright and holds exactly three codes: BRL, EUR, USD.
//! Confirmed against the live interpreter — **PT_BR does not see EN's
//! mutations of the shared `Num2Word_EUR.CURRENCY_FORMS`** (the trap
//! `PORTING_CURRENCY.md` warns about), because it never reads that dict.
//! `CURRENCY_ADJECTIVES` *is* EUR's shared dict (PT_BR defines none), and EN
//! never writes to it, so it is EUR's 16 literal entries.
//! `CURRENCY_PRECISION` resolves to `Num2Word_Base`'s `{}` — EN *rebinds*
//! rather than mutates it, so nothing leaks in — hence the divisor is 100 for
//! every code and neither the 3-decimal (KWD/BHD) nor the 0-decimal (JPY)
//! branch is reachable. That is why [`Lang::currency_precision`] is left at
//! its default here.
//!
//! ## Faithfully reproduced currency quirks
//!
//! 7. **An unknown currency code plus an `int` does not raise — it returns a
//!    bare cardinal.** `Num2Word_PT.to_currency`'s integer branch answers a
//!    `KeyError` from `CURRENCY_FORMS` with `return self.to_cardinal(val)`,
//!    dropping the currency word entirely; PT_BR's own post-pass then catches
//!    its own `KeyError` and passes. So `to_currency(100, "JPY")` is `"cem"`,
//!    not `NotImplementedError`. Only the *float* path raises, and it raises
//!    from `Num2Word_Base.to_currency` before PT's `cr1[1]` lookup can turn
//!    into a `KeyError`. The corpus pins both halves: `currency:JPY` of `0`
//!    is `"zero"` while `currency:JPY` of `12.34` is `NotImplementedError`.
//! 8. **A whole float or Decimal is demoted to `int` before anything else**,
//!    so it takes the no-cents integer path. `to_currency(1.0, "EUR")` is
//!    `"um euro"`, not `"um euro e zero cêntimos"` — PT_BR undoes the very
//!    `isinstance(val, int)` distinction that `has_decimal` exists to carry.
//!    `1.0` with an *unknown* code therefore also stops raising and yields
//!    `"um"` (quirk 7 + this one compounding).
//! 9. **Both " de" passes run, over different word lists.** PT's list is the
//!    European "bilião/biliões/trilião/triliões"; PT_BR's is the Brazilian
//!    "bilhão/bilhões/trilhão/trilhões". PT_BR's `to_cardinal` only ever
//!    emits the Brazilian forms, so PT's pass is dead for those rungs and
//!    PT_BR's pass is what fires. "milhão"/"milhões" appear in *both* lists,
//!    but PT's pass runs first and its rewrite ("milhão" -> "milhão de")
//!    breaks the "{ext} {currency}" adjacency PT_BR's guard needs, so the
//!    second pass cannot double-insert. See [`insert_de`].
//! 10. **The float path never gets " de" above 10^6 — a real grammatical bug,
//!    reproduced.** PT_BR's Brazilian-word pass is gated on
//!    `isinstance(val, int)`, so a non-integral value only ever sees PT's
//!    European list, which cannot match "bilhão"/"trilhão". The two rungs
//!    therefore disagree with each other:
//!
//!    ```text
//!    1000000000    EUR -> "um bilhão de euros"                  (int path)
//!    1000000000.5  EUR -> "um bilhão euros e cinquenta cêntimos" (float path)
//!    ```
//!
//!    "milhão" escapes this because it is in both lists, so `1000000.5` does
//!    get its "de". Verified against the interpreter; do not "fix" it.
//! 11. **`adjective=True` is silently ignored on the integer path.** PT's
//!    integer branch never calls `prefix_currency`, unlike `Num2Word_Base`'s.
//!    Only the float path honours the flag. (Moot for all but USD anyway —
//!    it is the one code present in both `CURRENCY_ADJECTIVES` and PT_BR's
//!    three-code `CURRENCY_FORMS`.)
//!
//! # Strings, floats and fractions
//!
//! * `str_to_number` (issue #63): a string with '.' and no ',' is US-style
//!   and is pronounced "ponto" instead of "vírgula" — Python stashes
//!   `_pending_pointword` on the instance and `to_cardinal_float` consumes
//!   it. Ported as `ParsedNumber::DecPoint` + [`Lang::cardinal_with_pointword`].
//!   "Infinity"/"NaN" strings raise `decimal.InvalidOperation` (from PT_BR's
//!   own `to_cardinal`: `divmod(Inf, 10**12)` / the `NaN < 0` comparison),
//!   not the base OverflowError/ValueError — see [`Lang::str_to_number`].
//! * `to_ordinal(float)` truncates via `int(value)` **before**
//!   `verify_ordinal` (PT's code, inherited): 2.5 -> "segundo", -1.5 ->
//!   TypeError (negord). `to_ordinal_num(float)` verifies the raw value:
//!   fractional -> TypeError (floatord) first, then numerically-negative;
//!   -0.0 passes both and yields "-0.0º".
//! * `to_year(float)`: `val < 0` numeric; negative renders
//!   `to_cardinal(abs(val)) + " antes de Cristo"`. Fractional *Decimals* are
//!   declined (NotImplemented -> Python fallback) because year mode cannot
//!   see a dotted string's pending "ponto" — see [`Lang::year_float_entry`].
//! * `to_fraction` is PT's override, inherited: "meio"/"terço" idiomatic
//!   forms, conditional "s" plural, dispatching to PT_BR's own
//!   cardinal/ordinal ("1000000/3" -> "um milhão terços").
//!
//! # Error variants
//!
//! * `to_ordinal(n)` / `to_ordinal_num(n)` for any `n < 0` → `TypeError`
//!   (deliberate, from `Num2Word_Base.verify_ordinal`) → [`N2WError::Type`].
//! * `to_ordinal(n)` for `n >= 10^18` → **KeyError**: the digit loop asks for
//!   `thousand_separators[18]` and PT_BR's table stops at 15. This is a crash,
//!   not a deliberate raise, but the type is observable → [`N2WError::Key`].
//!   Note 18-digit values are fine (max idx 17, and 17 % 3 != 0); the failure
//!   starts at 19 digits.
//! * `to_cardinal` has no reachable error path at all (see "Scale" above).
//! * `to_currency` / `to_cheque` for a code outside {BRL, EUR, USD} →
//!   `NotImplementedError` → [`N2WError::NotImplemented`], but for
//!   `to_currency` *only* when the value is not an integer (quirk 7).

use crate::base::{check_maxval, default_to_cardinal, set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::{default_to_currency, CurrencyForms, CurrencyValue};
use crate::floatpath::{float2tuple, FloatValue};
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{FromPrimitive, One, Signed, Zero};
use std::collections::HashMap;

/// `Num2Word_PT.hundreds`, keys 1..=9. Index 0 is absent in Python.
const HUNDREDS: [&str; 10] = [
    "", // absent in Python
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

/// `Num2Word_PT.ords` — three rungs of ten, units / tens / hundreds.
///
/// `ords[2][4]` is "quadrigentésimo" in the Python source (not
/// "quadringentésimo"); preserved verbatim.
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

/// Python's `\w` for `str`: `[a-zA-Z0-9_]` plus any Unicode alphanumeric.
///
/// The vocabulary here is Portuguese, so the accented letters (é, ã, ê, õ, ç,
/// í, ó, ú, á, â) must count as word characters — `is_alphanumeric` covers
/// them, whereas an ASCII-only test would not.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `result.replace("milião", "milhão").replace("miliões", "milhões")`.
///
/// See quirk 1 in the module docs: PT_BR's `merge` lacks PT's "milião" repair,
/// so the raw card word escapes and is scrubbed here instead. Python's
/// `str.replace` is replace-all, as is Rust's.
fn fix_milhao(s: &str) -> String {
    s.replace("milião", "milhão").replace("miliões", "milhões")
}

/// Python's `re.match(".*{ext} e \\w*entos? (?=.*e)", result)`.
///
/// `re.match` anchors at 0 but the leading `.*` (greedy, backtracking) makes
/// it an unanchored existence test. Decomposition of `\w*entos? `:
/// after the literal "{ext} e " at `j`, `\w*` greedily eats the word-char run
/// `[j, k)`, then backtracks to place "ento" + optional "s" ending at some
/// `m <= k` with `result[m] == ' '`. A space is not a word char, so `m == k`
/// is forced — i.e. the whole run must end in "ento"/"entos" and be followed
/// by a space. The `(?=.*e)` lookahead at `k + 1` then just asks whether an
/// 'e' occurs anywhere in the tail.
fn pt_match(s: &str, ext: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let pat: Vec<char> = format!("{} e ", ext).chars().collect();
    if chars.len() < pat.len() {
        return false;
    }
    for start in 0..=(chars.len() - pat.len()) {
        if chars[start..start + pat.len()] != pat[..] {
            continue;
        }
        let j = start + pat.len();
        let mut k = j;
        while k < chars.len() && is_word_char(chars[k]) {
            k += 1;
        }
        // `\w*entos?` must be followed by the pattern's literal space.
        if k >= chars.len() || chars[k] != ' ' {
            continue;
        }
        let w: String = chars[j..k].iter().collect();
        // "entos".ends_with("ento") is false, so both arms are needed.
        if !(w.ends_with("ento") || w.ends_with("entos")) {
            continue;
        }
        // Lookahead (?=.*e) evaluated after the consumed space.
        if chars[k + 1..].iter().any(|&c| c == 'e') {
            return true;
        }
    }
    false
}

/// `Num2Word_PT.to_cardinal`'s fixup: `result.replace("{ext} e", "{ext}")`.
///
/// Turns "mil e cento e catorze" into "mil cento e catorze". Replace-all, and
/// deliberately a plain string replace rather than a regex in Python.
fn pt_drops_e(result: &str, ext: &str) -> String {
    result.replace(&format!("{} e", ext), ext)
}

/// Python's `re.search("{ext} \\w*ento", r)` guard plus
/// `re.sub("({ext}) (\\w*entos?)", r"\1, \2", r, count=1)`.
///
/// Returns the rewritten string, or `None` when the guard fails.
///
/// Here `\w*ento` has nothing after it, so unlike [`pt_match`] the word run
/// need only *contain* "ento" (the greedy `\w*` backtracks to expose it), not
/// end with it. Guard and substitution share the same leftmost start: `s?` is
/// optional, so anywhere `\w*ento` matches, `\w*entos?` matches too.
///
/// The substitution is capture-preserving — `\1` is `{ext}` and `\2` is the
/// (possibly partial) word — so the whole rewrite reduces to inserting a comma
/// in place of that one space, leaving any unmatched tail of the word alone.
fn ptbr_comma(s: &str, ext: &str) -> Option<String> {
    let chars: Vec<char> = s.chars().collect();
    let pat: Vec<char> = format!("{} ", ext).chars().collect();
    if chars.len() < pat.len() {
        return None;
    }
    for start in 0..=(chars.len() - pat.len()) {
        if chars[start..start + pat.len()] != pat[..] {
            continue;
        }
        let j = start + pat.len();
        let mut k = j;
        while k < chars.len() && is_word_char(chars[k]) {
            k += 1;
        }
        let w: String = chars[j..k].iter().collect();
        if !w.contains("ento") {
            continue;
        }
        // Matched span is "{ext} {X}", replacement "{ext}, {X}": the space at
        // j - 1 becomes ", " and everything else is carried through.
        let mut out: String = chars[..j - 1].iter().collect();
        out.push_str(", ");
        out.extend(chars[j..].iter());
        return Some(out);
    }
    None
}

/// `Num2Word_PT_BR.CURRENCY_FORMS` — the class body's own dict, verbatim.
///
/// PT_BR rebinds the name in its class body, so PT's 14-code table and EUR's
/// 23-code table are both shadowed and *neither* is consulted at runtime.
/// That also puts PT_BR out of reach of `Num2Word_EN.__init__`'s in-place
/// mutation of `Num2Word_EUR.CURRENCY_FORMS`: verified with
/// `c.CURRENCY_FORMS is Num2Word_EUR.CURRENCY_FORMS` → `False`.
///
/// Every entry carries exactly two forms, which is all `Num2Word_EUR`'s
/// `pluralize` ever indexes.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    let mut m: HashMap<&'static str, CurrencyForms> = HashMap::new();
    m.insert("BRL", CurrencyForms::new(&["real", "reais"], &["centavo", "centavos"]));
    m.insert("EUR", CurrencyForms::new(&["euro", "euros"], &["cêntimo", "cêntimos"]));
    m.insert("USD", CurrencyForms::new(&["dólar", "dólares"], &["centavo", "centavos"]));
    m
}

/// `Num2Word_EUR.CURRENCY_ADJECTIVES`, inherited unchanged (PT_BR and PT both
/// define none, and EN never writes to this dict — only to `CURRENCY_FORMS`).
///
/// Almost entirely dead weight here: only USD overlaps PT_BR's three codes, so
/// USD is the single adjective that can ever be applied. Kept whole because it
/// is what the attribute lookup actually resolves to.
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

/// `Num2Word_PT.to_currency`'s scale words — European long scale.
const PT_DE_EXTS: [&str; 6] = ["milhão", "milhões", "bilião", "biliões", "trilião", "triliões"];

/// `Num2Word_PT_BR.to_currency`'s scale words — Brazilian short scale.
const PT_BR_DE_EXTS: [&str; 6] = ["milhão", "milhões", "bilhão", "bilhões", "trilhão", "trilhões"];

/// One iteration of the `" de"` insertion loop both PT and PT_BR run:
///
/// ```python
/// if re.match(".*{ext} (?={currency_str})".format(...), result):
///     result = result.replace("{ext}", "{ext} de".format(ext), 1)
/// ```
///
/// Returns the rewritten string, or `None` when the guard fails.
///
/// The guard is `re.match`, anchored at 0 — but the leading `.*` is greedy
/// *with backtracking*, so it degrades to a plain existence test for the
/// literal "{ext} {currency_str}". Two things make that exact rather than
/// approximate: every `ext` and every currency word in PT_BR's table is plain
/// text with no regex metacharacters, and no output ever contains a newline
/// (the one character `.` would refuse to cross).
///
/// The replacement is a **`str.replace` with `count=1`, not a `re.sub`**: it
/// rewrites the first occurrence of `ext` *anywhere* in the string, which need
/// not be the occurrence the guard matched. Preserved verbatim — with these
/// tables the two always coincide, because `to_cardinal` emits at most one
/// scale word per rung and "milhão" is not a substring of "milhões".
fn insert_de(result: &str, ext: &str, currency_str: &str) -> Option<String> {
    if result.contains(&format!("{} {}", ext, currency_str)) {
        Some(result.replacen(ext, &format!("{} de", ext), 1))
    } else {
        None
    }
}

/// Python's `re.sub("\\s+", " ", value)`.
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

/// `abs(Decimal(repr(f)).as_tuple().exponent)` for an f64 — the count of
/// fractional digits in its shortest round-trip repr.
///
/// Mirrors `floatpath::float_repr_precision` (private there). Used to re-derive
/// the precision of a float sub-value produced by float-domain `divmod`, which
/// is what `base.float2tuple` does when `Num2Word_PT_BR.to_cardinal` re-enters
/// the float path on a low group. Rust's `{}` for f64 is shortest round-trip,
/// the same contract as Python's `repr`.
fn float_repr_precision(f: f64) -> u32 {
    let s = format!("{}", f);
    match s.split_once('.') {
        Some((_, frac)) if !frac.contains('e') => frac.len() as u32,
        _ => 0,
    }
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

pub struct LangPtBr {
    cards: Cards,
    maxval: BigInt,
    hundreds: [&'static str; 10],
    thousand_separators: HashMap<usize, &'static str>,
    exclude_title: Vec<String>,
    currency_forms: HashMap<&'static str, CurrencyForms>,
    currency_adjectives: HashMap<&'static str, &'static str>,
}

impl Default for LangPtBr {
    fn default() -> Self {
        Self::new()
    }
}

impl LangPtBr {
    pub fn new() -> Self {
        // PT.setup: high_numwords = self.gen_high_numwords([], [], lows).
        // With tens == [] the list comprehension `[u + t for t in tens ...]`
        // is empty, so gen_high_numwords returns `[] + lows` == lows.
        let high = ["non", "oct", "sept", "sext", "quint", "quatr", "tr", "b", "m"];

        let mut cards = Cards::new();

        // Num2Word_EUR.set_high_numwords, with PT's GIGA_SUFFIX = None and
        // MEGA_SUFFIX = "ilião":
        //   cap = 3 + 6 * len(high) = 57
        //   for word, n in zip(high, range(cap, 3, -6)):
        //       if GIGA_SUFFIX: cards[10**n] = word + GIGA_SUFFIX   # skipped
        //       if MEGA_SUFFIX: cards[10**(n-3)] = word + MEGA_SUFFIX
        // zip() stops at the shorter sequence; both are length 9 here.
        let cap: i64 = 3 + 6 * high.len() as i64;
        let mut n = cap;
        for word in high.iter() {
            if n <= 3 {
                break; // range(cap, 3, -6) is exclusive of 3
            }
            cards.insert(
                BigInt::from(10u8).pow((n - 3) as u32),
                format!("{}ilião", word),
            );
            n -= 6;
        }

        // PT.setup mid_numwords.
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

        // PT_BR.setup low_numwords — overrides PT's list. Brazilian spelling
        // differs at 16 / 17 / 19 (dezesseis / dezessete / dezenove vs the
        // European dezasseis / dezassete / dezanove). 14 stays "catorze".
        set_low_numwords(
            &mut cards,
            &[
                "vinte",
                "dezenove",
                "dezoito",
                "dezessete",
                "dezesseis",
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

        // Num2Word_Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0].
        // The first key inserted is the largest (10^54), so `highest()` agrees
        // with Python's insertion-order indexing. => MAXVAL == 10^57.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        // PT_BR.setup thousand_separators — overrides PT's, which used the
        // European long scale (9: "milésimo milionésimo", 12: "bilionésimo").
        // Brazil is short scale: bilionésimo = 10^9, trilionésimo = 10^12.
        // Nothing above 15, hence the KeyError at 10^18 (see module docs).
        let thousand_separators: HashMap<usize, &'static str> = [
            (3usize, "milésimo"),
            (6, "milionésimo"),
            (9, "bilionésimo"),
            (12, "trilionésimo"),
            (15, "quatrilionésimo"),
        ]
        .into_iter()
        .collect();

        LangPtBr {
            cards,
            maxval,
            hundreds: HUNDREDS,
            thousand_separators,
            exclude_title: vec!["e".into(), "vírgula".into(), "menos".into()],
            // Built once here, never per call. `to_currency` only ever reads
            // these tables, and rebuilding them on every call is what made an
            // earlier revision of this port slower than the Python it
            // replaces. CURRENCY_PRECISION has no field: PT_BR resolves it to
            // Num2Word_Base's empty dict, so `.get(code, 100)` is always 100 —
            // exactly the trait default.
            currency_forms: build_currency_forms(),
            currency_adjectives: build_currency_adjectives(),
        }
    }

    /// `Num2Word_PT.hundreds[cnum]`.
    ///
    /// Python would raise KeyError outside 1..=9, but `merge` cannot return an
    /// error and the lookup is unreachable: it fires only when `nnum == 100`,
    /// which `splitnum` produces solely for `100 <= value < 1000`, giving
    /// `cnum = value // 100` in 1..=9 — and `cnum == 1` has already returned.
    /// So `cnum` is always 2..=9 here.
    fn hundreds_word(&self, cnum: &BigInt) -> &'static str {
        let idx = u32::try_from(cnum).unwrap_or(0) as usize;
        if idx < self.hundreds.len() {
            self.hundreds[idx]
        } else {
            ""
        }
    }

    /// `super(Num2Word_PT_BR, self).to_cardinal(value)` — i.e.
    /// `Num2Word_PT.to_cardinal`: the `Num2Word_Base` engine (splitnum /
    /// clean / PT_BR's merge) followed by PT's "mil e cento" → "mil cento"
    /// regex pass.
    ///
    /// Every caller passes `value < 10^9`, so neither the MAXVAL check nor any
    /// card above 10^6 is reachable through this path.
    fn super_to_cardinal(&self, value: &BigInt) -> Result<String> {
        let mut result = default_to_cardinal(self, value)?;

        // Transforms "mil e cento e catorze" into "mil cento e catorze".
        // Transforms "cem milhões e duzentos mil e duzentos e dez" into "cem
        // milhões duzentos mil duzentos e dez" but "cem milhões e duzentos mil
        // e duzentos" into "cem milhões duzentos mil e duzentos" and not into
        // "cem milhões duzentos mil duzentos" — the (?=.*e) lookahead is what
        // spares the final group.
        for ext in [
            "mil",
            "milhão",
            "milhões",
            "mil milhões",
            "bilião",
            "biliões",
            "mil biliões",
        ] {
            if pt_match(&result, ext) {
                result = pt_drops_e(&result, ext);
            }
        }
        Ok(result)
    }

    /// `Num2Word_PT.to_cardinal(value)` for a **non-integer** `value >= 0` —
    /// i.e. `Num2Word_Base.to_cardinal_float(value)` followed by PT's
    /// "mil e cento" → "mil cento" e-drop pass.
    ///
    /// `base.to_cardinal_float` renders the integer part with
    /// `self.to_cardinal(pre)` (so `pre >= 10^6` picks up PT_BR's short-scale
    /// commas — this is exactly the divergence a plain
    /// `default_to_cardinal_float` gets wrong: it would route the low group
    /// through the base engine and drop the comma), then joins "vírgula" and
    /// one word per fractional digit.
    ///
    /// `float2tuple` is recomputed here on the *leaf* sub-value, exactly as
    /// Python's `super().to_cardinal(rest)` re-enters `base.to_cardinal_float`.
    /// For a `Decimal` that is a no-op (precision is preserved exactly through
    /// `divmod`); for a `float` it is load-bearing — the low group's own
    /// `repr` precision is re-derived from the f64-error-corrupted remainder,
    /// which is how `98746251323029.99` differs from a large binary float.
    ///
    /// `pointword` is threaded in rather than read from `self`: Python's
    /// `Num2Word_PT_BR.to_cardinal_float` swaps `self.pointword` to the
    /// pending "ponto" for the duration of the call when `str_to_number` saw
    /// a US-style dotted string (see [`Lang::str_to_number`]); the swap is
    /// visible here and nowhere else.
    fn super_float(&self, fv: &FloatValue, pointword: &str) -> Result<String> {
        let precision = fv.precision();
        // The sub-value is always >= 0 (the sign is peeled off in
        // `to_cardinal_float`), so `pre` is non-negative and
        // `base.to_cardinal_float`'s `value < 0 and pre == 0` negword branch is
        // dead and omitted.
        let (pre, post) = float2tuple(fv);
        let pre = pre.abs();

        // post = str(post); left-padded to `precision` — same as base.
        let post_str = post.to_string();
        let post_str = format!(
            "{}{}",
            "0".repeat((precision as usize).saturating_sub(post_str.len())),
            post_str
        );

        let mut out = vec![self.to_cardinal(&pre)?];
        if precision > 0 {
            out.push(self.title(pointword));
        }
        for ch in post_str.chars().take(precision as usize) {
            let d = ch
                .to_digit(10)
                .ok_or_else(|| N2WError::Value(format!("non-digit {:?} in fractional part", ch)))?;
            out.push(self.to_cardinal(&BigInt::from(d))?);
        }
        let mut result = out.join(" ");

        // `Num2Word_PT.to_cardinal`'s e-drop pass, over the full float string.
        // A no-op in practice (the integer part was already processed by
        // `self.to_cardinal(pre)`, and "vírgula"/digit words never end in
        // "ento"), but applied verbatim for fidelity.
        for ext in [
            "mil",
            "milhão",
            "milhões",
            "mil milhões",
            "bilião",
            "biliões",
            "mil biliões",
        ] {
            if pt_match(&result, ext) {
                result = pt_drops_e(&result, ext);
            }
        }
        Ok(result)
    }

    /// `divmod(value, 10**pow)`, reproducing Python's per-type semantics on a
    /// non-negative sub-value. Returns `(quotient, remainder)` where the
    /// quotient is a whole `BigInt` and the remainder is a `FloatValue` of the
    /// same variant that carries the fraction on down.
    ///
    /// * `Decimal`: exact. `q = int(value) // 10**pow`, and the remainder keeps
    ///   the *original* precision, because subtracting an exponent-0 multiple in
    ///   `Decimal` arithmetic preserves the exponent.
    /// * `float`: IEEE `fmod`. For a non-negative dividend Python's
    ///   `float.__divmod__` yields `(round((v - fmod(v, d)) / d), fmod(v, d))`,
    ///   and Rust's `%` *is* `fmod`; the remainder then re-derives its own
    ///   `repr` precision — the whole point of doing this in the float domain.
    fn fv_divmod(&self, fv: &FloatValue, pow: u32) -> (BigInt, FloatValue) {
        match fv {
            FloatValue::Float { value, .. } => {
                let d = 10f64.powi(pow as i32);
                let rem = value % d;
                let q = ((value - rem) / d).round();
                (
                    BigInt::from(q as i128),
                    FloatValue::Float {
                        value: rem,
                        precision: float_repr_precision(rem),
                    },
                )
            }
            FloatValue::Decimal { value, precision } => {
                let divisor = BigInt::from(10).pow(pow);
                // int(value) truncates toward zero; value >= 0 so == floor.
                let pre = value.with_scale(0).as_bigint_and_exponent().0;
                let q = &pre / &divisor;
                let rem = value - BigDecimal::from(&q * &divisor);
                (
                    q,
                    FloatValue::Decimal {
                        value: rem,
                        precision: *precision,
                    },
                )
            }
        }
    }

    /// `Num2Word_PT_BR.to_cardinal(value)` for a **non-integer** `value >= 0`.
    ///
    /// Structurally identical to the integer [`Lang::to_cardinal`] short-scale
    /// wrapper, with three float-specific consequences of the fraction:
    ///   * the fraction rides in the lowest remainder, so `remainder` is always
    ///     truthy and every `if remainder` block is unconditionally taken;
    ///   * the tail renders through the float path ([`LangPtBr::super_float`],
    ///     or a recursive call for the millions branch) instead of the integer
    ///     `super_to_cardinal` / `to_cardinal`;
    ///   * the `< 10^6` leaf is the base float render plus the same
    ///     `fix_milhao` + comma pass the integer branch runs.
    ///
    /// The `divmod` is carried in the value's own domain ([`LangPtBr::fv_divmod`])
    /// rather than on a shared `pre`, so a large binary `float`'s low groups
    /// re-derive their own precision exactly as Python's float-domain `divmod`
    /// makes them. Every magnitude test reduces to the integer part because
    /// `0 <= frac < 1` (`remainder >= 100` ⇔ `int(remainder) >= 100`, and
    /// `value >= 10^n` ⇔ `int(value) >= 10^n`, since each `10^n` is integral).
    fn frac_cardinal(&self, fv: &FloatValue, pointword: &str) -> Result<String> {
        let hundred = BigInt::from(100);
        let e5 = BigInt::from(100_000u32);
        let e6 = BigInt::from(1_000_000u32);
        let e9 = BigInt::from(1_000_000_000u32);
        let e12 = BigInt::from(1_000_000_000_000u64);

        let pre = float2tuple(fv).0.abs();

        if pre >= e9 {
            // Handle trillions (10^12).
            if pre >= e12 {
                let (trillions, remainder) = self.fv_divmod(fv, 12);
                let mut result = if trillions.is_one() {
                    "um trilhão".to_string()
                } else {
                    format!("{} trilhões", self.to_cardinal(&trillions)?)
                };

                // remainder always carries the fraction → always truthy.
                let rem_pre = float2tuple(&remainder).0.abs();
                if rem_pre >= e9 {
                    let (billions, rest) = self.fv_divmod(&remainder, 9);
                    if billions.is_one() {
                        result.push_str(", um bilhão");
                    } else {
                        result.push_str(&format!(", {} bilhões", self.to_cardinal(&billions)?));
                    }
                    // rest always carries the fraction → always truthy.
                    let rest_str = fix_milhao(&self.super_float(&rest, pointword)?);
                    result.push_str(&format!(" e {}", rest_str));
                } else {
                    let remainder_str = fix_milhao(&self.super_float(&remainder, pointword)?);
                    result.push_str(&format!(" e {}", remainder_str));
                }
                return Ok(result);
            }

            // Handle billions (10^9).
            let (billions, remainder) = self.fv_divmod(fv, 9);
            let mut result = if billions.is_one() {
                "um bilhão".to_string()
            } else {
                format!("{} bilhões", self.to_cardinal(&billions)?)
            };

            // remainder always truthy (fraction present).
            let rem_pre = float2tuple(&remainder).0.abs();
            let remainder_str = fix_milhao(&self.super_float(&remainder, pointword)?);
            if rem_pre >= e5 {
                result.push_str(&format!(", {}", remainder_str));
            } else {
                result.push_str(&format!(" e {}", remainder_str));
            }
            return Ok(result);
        }

        // For values below 1 billion but above a million, handle specially.
        if pre >= e6 {
            let (millions, remainder) = self.fv_divmod(fv, 6);
            let mut result = if millions.is_one() {
                "um milhão".to_string()
            } else {
                format!("{} milhões", self.super_to_cardinal(&millions)?)
            };

            // remainder always truthy (fraction present); rendered via `self`,
            // so it recurses back into this routine (mirroring the integer
            // branch's `self.to_cardinal(remainder)`).
            let rem_pre = float2tuple(&remainder).0.abs();
            let remainder_str = self.frac_cardinal(&remainder, pointword)?;
            if rem_pre >= hundred {
                result.push_str(&format!(", {}", remainder_str));
            } else {
                result.push_str(&format!(" e {}", remainder_str));
            }
            return Ok(result);
        }

        // For values below 1 million: the base float render, then the same
        // fix_milhao + comma loop the integer `< 10^6` branch runs.
        let mut result = fix_milhao(&self.super_float(fv, pointword)?);
        for ext in [
            "mil",
            "milhão",
            "milhões",
            "bilhão",
            "bilhões",
            "trilhão",
            "trilhões",
            "quatrilhão",
            "quatrilhões",
        ] {
            if let Some(rewritten) = ptbr_comma(&result, ext) {
                result = rewritten;
            }
        }
        Ok(result)
    }

    /// `Num2Word_PT_BR.to_cardinal` for a **non-integer** value, with the
    /// pointword made explicit — the shared body behind both
    /// [`Lang::to_cardinal_float`] (plain "vírgula") and
    /// [`Lang::cardinal_with_pointword`] (the str_to_number "ponto"
    /// handshake). The sign is peeled here, matching the Python
    /// `"%s%s" % (negword, …)` wrapper (`base.to_cardinal_float`'s own
    /// `pre == 0` negword branch is therefore dead for PT_BR and is not
    /// reproduced).
    fn cardinal_float_with_pointword(
        &self,
        value: &FloatValue,
        pointword: &str,
    ) -> Result<String> {
        // `Num2Word_PT_BR.to_cardinal` negates first — `"%s%s" % (negword, …)`
        // — and routes the magnitude, so the sign is peeled here and the
        // routing sees a non-negative value throughout.
        let abs_fv = fv_abs(value);
        let body = self.frac_cardinal(&abs_fv, pointword)?;

        if value.is_negative() {
            // negword carries its own trailing space ("menos ").
            Ok(format!("{}{}", self.negword(), body))
        } else {
            Ok(body)
        }
    }

    /// `super(Num2Word_PT_BR, self).to_currency(...)` — i.e.
    /// `Num2Word_PT.to_currency`, which splits int from float and then runs
    /// the European-scale `" de"` pass.
    ///
    /// # The negword mutation is deliberately not modelled
    ///
    /// Both branches of the Python do
    /// `backup = self.negword; self.negword = self.negword[:-1]` — trimming
    /// "menos " to "menos" — and restore it afterwards. Within a call this is
    /// **unobservable**: every read of the mutated value goes through
    /// `self.negword.strip()`, which erases the very space that was removed,
    /// and the only `to_cardinal` calls in scope take `abs(...)` so they never
    /// reach `negword` at all. So this port simply reads `negword()` directly.
    ///
    /// Across calls it *is* observable, because the float branch restores
    /// without a `try/finally` and `Num2Word_Base.to_currency` raises for an
    /// unknown code — see the `concerns` note. That leak is cross-call
    /// converter state, which this port is stateless by design and does not
    /// reproduce; no corpus row depends on it.
    fn pt_to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: &str,
        adjective: bool,
    ) -> Result<String> {
        // is_integer = isinstance(val, int)
        if let CurrencyValue::Int(v) = val {
            let cr1 = match self.currency_forms.get(currency) {
                Some(f) => &f.unit,
                // except KeyError: return self.to_cardinal(val)
                //
                // Quirk 7: an unknown code does *not* raise here, it degrades
                // to a bare cardinal with no currency word. Note this returns
                // before the negword mutation, so `to_cardinal` sees the full
                // "menos " and negatives render normally.
                None => return self.to_cardinal(v),
            };

            let minus_str = if v.is_negative() {
                format!("{} ", self.negword().trim())
            } else {
                String::new()
            };
            // abs(int(val) if isinstance(val, float) else val) — val is an int
            // on this branch, so the float cast is dead.
            let abs_val = v.abs();
            let money_str = self.to_cardinal(&abs_val)?;

            // `cr1[0]` if abs_val == 1 else `cr1[1] if len(cr1) > 1 else cr1[0]`.
            // Note this open-codes the plural choice instead of calling
            // pluralize(), unlike Num2Word_Base.to_currency — same answer for
            // EUR's two-form rule, but it is what the source does.
            let currency_str = if abs_val.is_one() || cr1.len() <= 1 {
                cr1.first()
            } else {
                cr1.get(1)
            }
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?
            .clone();

            let mut result = format!("{}{} {}", minus_str, money_str, currency_str);

            // transforms "milhões euros" em "milhões de euros"
            for ext in PT_DE_EXTS {
                if let Some(rewritten) = insert_de(&result, ext, &currency_str) {
                    result = rewritten;
                }
            }
            return Ok(result.trim().to_string());
        }

        // For floats with a non-zero decimal part, use the parent class
        // implementation. This is the only path that can raise: Base's own
        // CURRENCY_FORMS lookup turns a missing code into NotImplementedError.
        let mut result = default_to_currency(self, val, currency, cents, separator, adjective)?;

        // `cr1, _ = self.CURRENCY_FORMS[currency]`. Python would raise a raw
        // KeyError here, but it is unreachable: Base already raised
        // NotImplementedError for exactly the same missing key.
        let cr1 = match self.currency_forms.get(currency) {
            Some(f) => &f.unit,
            None => return Err(N2WError::Key(format!("'{}'", currency))),
        };
        // Python indexes cr1[1] unconditionally — the plural — rather than
        // consulting pluralize(). IndexError on a one-form entry; every entry
        // in PT_BR's table has two, so this cannot fire.
        let plural = cr1
            .get(1)
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?
            .clone();
        for ext in PT_DE_EXTS {
            if let Some(rewritten) = insert_de(&result, ext, &plural) {
                result = rewritten;
            }
        }
        Ok(result)
    }

    /// `Num2Word_Base.verify_ordinal`. The float check cannot fire on BigInt
    /// input; only the negative check survives, and it raises TypeError.
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

impl Lang for LangPtBr {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "BRL"
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
        // PT.setup, the PT_BR class attribute and PT_BR.__init__ all agree on
        // "menos " — note the trailing space, which PT_BR's to_cardinal relies
        // on ("%s%s" % (negword, ...)) rather than re-adding one.
        "menos "
    }
    fn pointword(&self) -> &str {
        "vírgula"
    }
    fn exclude_title(&self) -> &[String] {
        &self.exclude_title
    }

    /// `Num2Word_PT_BR.merge`.
    ///
    /// Diverges from `Num2Word_PT.merge` in four ways, all preserved:
    ///   * the "cento" guard is `0 < nnum < 1000` rather than
    ///     `nnum % 1000 != 0` (so "cem mil" survives at 100 * 1000);
    ///   * the multiplicative test is `nnum > cnum`, so `nnum == cnum` falls
    ///     into the additive arm (PT tests `nnum < cnum` and sends equality
    ///     the other way). Unreachable from `splitnum`, but not equivalent.
    ///   * there is no `10^9` / "liões" rung, only the `10^6` / "lhões" one;
    ///   * PT's `if ntext == "milião": ntext = "milhão"` repair is **absent**
    ///     — see quirk 1 in the module docs.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ctext, cnum) = l;
        let (ntext, nnum) = r;
        let mut ctext = ctext.to_string();
        let mut ntext = ntext.to_string();

        let hundred = BigInt::from(100);
        let thousand = BigInt::from(1000);
        let million = BigInt::from(1_000_000);

        if cnum.is_one() {
            if nnum < &million {
                return (ntext, cnum * nnum);
            }
            ctext = "um".to_string();
        } else if cnum == &hundred && nnum > &BigInt::zero() && nnum < &thousand {
            // In Brazilian Portuguese, 100 + any number up to 999 becomes
            // "cento". But 100 * 1000 = 100000 stays as "cem mil".
            ctext = "cento".to_string();
        }

        if nnum > cnum {
            // Multiplicative case (e.g. "dois mil").
            if (nnum % &million).is_zero() && cnum > &BigInt::one() {
                if nnum == &million {
                    ntext = "milhões".to_string();
                } else {
                    // Python `ntext[:-4] + "lhões"` — slice by character, and
                    // tolerate strings shorter than 4 the way Python does.
                    // Unreachable: nnum >= 10^12 never reaches merge, because
                    // to_cardinal never hands super() a value >= 10^9.
                    let chars: Vec<char> = ntext.chars().collect();
                    let keep: String = chars[..chars.len().saturating_sub(4)].iter().collect();
                    ntext = format!("{}lhões", keep);
                }
            }

            if nnum == &hundred {
                ctext = self.hundreds_word(cnum).to_string();
                ntext = String::new();
            } else {
                ntext = format!(" {}", ntext);
            }

            (format!("{}{}", ctext, ntext), cnum * nnum)
        } else {
            // Additive case (e.g. "vinte e dois").
            (format!("{} e {}", ctext, ntext), cnum + nnum)
        }
    }

    /// `Num2Word_PT_BR.to_cardinal`.
    ///
    /// Brazil is short scale (bilhão = 10^9, trilhão = 10^12); the inherited
    /// PT engine is long scale (bilião = 10^12), so PT_BR peels the 10^9 and
    /// 10^12 rungs off by hand and only delegates the sub-10^9 tail.
    ///
    /// Note which helper each branch reaches for — the asymmetry is real:
    ///   * trillions branch: count via `self`, tail via `super()`;
    ///   * billions branch:  count via `self`, tail via `super()`;
    ///   * millions branch:  count via `super()`, tail via `self`;
    ///   * below 10^6:       via `super()`, and this is the *only* branch that
    ///     falls through to the trailing comma loop — every other branch
    ///     returns early (quirk 3).
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        // The MAXVAL check Python never reaches (module docs, #159).
        check_maxval(value, &self.maxval)?;
        // Handle negative numbers. negword already carries its trailing space.
        if value.is_negative() {
            return Ok(format!("{}{}", self.negword(), self.to_cardinal(&(-value))?));
        }

        let hundred = BigInt::from(100);
        let e5 = BigInt::from(100_000u32);
        let e6 = BigInt::from(1_000_000u32);
        let e9 = BigInt::from(1_000_000_000u32);
        let e12 = BigInt::from(1_000_000_000_000u64);

        if value >= &e9 {
            // Handle trillions (10^12).
            if value >= &e12 {
                let (trillions, remainder) = value.div_rem(&e12);
                let mut result = if trillions.is_one() {
                    "um trilhão".to_string()
                } else {
                    format!("{} trilhões", self.to_cardinal(&trillions)?)
                };

                if !remainder.is_zero() {
                    if remainder >= e9 {
                        // Has billions.
                        let (billions, rest) = remainder.div_rem(&e9);
                        if billions.is_one() {
                            result.push_str(", um bilhão");
                        } else {
                            result.push_str(&format!(", {} bilhões", self.to_cardinal(&billions)?));
                        }
                        if !rest.is_zero() {
                            let rest_str = fix_milhao(&self.super_to_cardinal(&rest)?);
                            result.push_str(&format!(" e {}", rest_str));
                        }
                    } else {
                        // No billions, just add remainder.
                        let remainder_str = fix_milhao(&self.super_to_cardinal(&remainder)?);
                        result.push_str(&format!(" e {}", remainder_str));
                    }
                }
                return Ok(result);
            }

            // Handle billions (10^9).
            let (billions, remainder) = value.div_rem(&e9);
            let mut result = if billions.is_one() {
                "um bilhão".to_string()
            } else {
                format!("{} bilhões", self.to_cardinal(&billions)?)
            };

            if !remainder.is_zero() {
                let remainder_str = fix_milhao(&self.super_to_cardinal(&remainder)?);
                // Comma if the remainder starts with hundreds (>= 100000).
                if remainder >= e5 {
                    result.push_str(&format!(", {}", remainder_str));
                } else {
                    result.push_str(&format!(" e {}", remainder_str));
                }
            }
            return Ok(result);
        }

        // For values below 1 billion but above a million, handle specially.
        if value >= &e6 {
            let (millions, remainder) = value.div_rem(&e6);
            let mut result = if millions.is_one() {
                "um milhão".to_string()
            } else {
                format!("{} milhões", self.super_to_cardinal(&millions)?)
            };

            if !remainder.is_zero() {
                let remainder_str = self.to_cardinal(&remainder)?;
                // Comma if the remainder starts with hundreds (>= 100).
                if remainder >= hundred {
                    result.push_str(&format!(", {}", remainder_str));
                } else {
                    result.push_str(&format!(" e {}", remainder_str));
                }
            }
            return Ok(result);
        }

        // For values below 1 million, use the parent implementation.
        let mut result = fix_milhao(&self.super_to_cardinal(value)?);

        // Transforms "mil e cento e catorze" into "mil, cento e catorze".
        // Reachable only from the branch above; "milhão" and up are dead
        // entries here, kept because Python iterates them all.
        for ext in [
            "mil",
            "milhão",
            "milhões",
            "bilhão",
            "bilhões",
            "trilhão",
            "trilhões",
            "quatrilhão",
            "quatrilhões",
        ] {
            if let Some(rewritten) = ptbr_comma(&result, ext) {
                result = rewritten;
            }
        }

        Ok(result)
    }

    /// `Num2Word_PT.to_ordinal` (inherited unchanged; PT_BR only swaps the
    /// `thousand_separators` table underneath it).
    ///
    /// Walks the decimal string right-to-left, emitting a scale word whenever
    /// a group boundary carries a non-zero digit — which is what avoids
    /// "segundo milionésimo milésimo" for 6000000.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        let s = value.to_string(); // Python: value = str(int(value))
        let mut result: Vec<String> = Vec::new();
        let mut thousand_separator: &str = "";

        for (idx, ch) in s.chars().rev().enumerate() {
            if idx != 0 && idx % 3 == 0 {
                // PT_BR's table stops at 15 → KeyError from idx 18 (i.e. any
                // value with 19+ digits). The lookup happens before the digit
                // is examined, so even 10^18 with all-zero groups blows up.
                thousand_separator = match self.thousand_separators.get(&idx) {
                    Some(t) => *t,
                    None => return Err(N2WError::Key(idx.to_string())),
                };
            }

            if ch != '0' && !thousand_separator.is_empty() {
                // Avoiding "segundo milionésimo milésimo" for 6000000.
                result.push(thousand_separator.to_string());
                thousand_separator = "";
            }

            let d = ch.to_digit(10).unwrap_or(0) as usize;
            result.push(ORDS[idx % 3][d].to_string());
        }

        result.reverse();
        let joined = result.join(" ");
        let result = collapse_ws(joined.trim());

        if result.starts_with("primeiro") && s != "1" {
            // Avoiding "primeiro milésimo", "primeiro milionésimo" and so on.
            // Python `result[9:]` slices by character and tolerates an index
            // past the end (only reachable if result were exactly "primeiro",
            // which requires value == 1 and is excluded by the guard).
            return Ok(result.chars().skip(9).collect());
        }
        Ok(result)
    }

    /// `Num2Word_PT.to_ordinal_num`: `"%sº" % value`, after verify_ordinal.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}º", value))
    }

    /// `Num2Word_PT.to_year`. Dispatches through `self`, so it picks up
    /// PT_BR's short-scale `to_cardinal`.
    fn to_year(&self, value: &BigInt) -> Result<String> {
        if value.is_negative() {
            return Ok(format!("{} antes de Cristo", self.to_cardinal(&value.abs())?));
        }
        self.to_cardinal(value)
    }

    // ---- float/Decimal entry points --------------------------------------
    //
    // `cardinal_float_entry` stays at the trait default: PT_BR's Python
    // `to_cardinal(float)` peels the short-scale rungs in the float domain,
    // but for a *whole* value every divmod is exact and the result is
    // identical to the int path, so the default's whole -> int routing is
    // faithful; fractional values land in `to_cardinal_float` above.

    /// `Num2Word_PT.to_ordinal` (inherited) on float/Decimal input.
    ///
    /// `value = int(value)` — truncation toward zero — runs *before*
    /// `verify_ordinal`, so `2.5` -> "segundo", `0.5` -> "", `-0.0` -> "",
    /// and `-1.5` raises the negative-num TypeError (never the float one).
    /// 10^18 and up still hit the `thousand_separators` KeyError.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.to_ordinal(&float_trunc_int(value)?)
    }

    /// `Num2Word_PT.to_ordinal_num` (inherited) on float/Decimal input:
    /// `verify_ordinal(value)` on the **raw** value, then `"%sº" % value`.
    ///
    /// Check order is Python's: the float test comes first, so `-1.5` raises
    /// `errmsg_floatord`, not `errmsg_negord`. Both messages interpolate
    /// `str(value)` — the binding's `repr_str`. The negative test is numeric:
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

    /// `Num2Word_PT.to_year` (inherited) on float/Decimal input. `val < 0` is
    /// numeric (-0.0 goes positive); the negative branch renders
    /// `to_cardinal(abs(val)) + " antes de Cristo"` — fractional floats
    /// through PT_BR's own float grammar: `to_year(-1.5)` ==
    /// "um vírgula cinco antes de Cristo".
    ///
    /// **Fractional `Decimal`s render with the pending "ponto"**, served
    /// natively. A fractional Decimal reaching year mode came from a dotted
    /// US-style *string* ("1.5") whose `str_to_number` stashed
    /// `_pending_pointword = "ponto"`: the binding threads that stash only to
    /// cardinal mode (`ParsedNumber::DecPoint` → `cardinal_with_pointword`),
    /// dropping it for year, so year_float_entry must reapply "ponto" itself.
    /// Every fractional value that arrives here as a `FloatValue::Decimal` is
    /// string-origin (a genuine `Decimal` fraction is not exercised in year
    /// mode; a genuine `float` fraction arrives as `FloatValue::Float` and
    /// keeps the default "vírgula"), so "ponto" is the right pointword for the
    /// whole Decimal arm — `to_year("1.5")` == "um ponto cinco".
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        let fractional_decimal =
            matches!(value, FloatValue::Decimal { .. }) && value.as_whole_int().is_none();
        // year delegates to to_cardinal; a fractional Decimal keeps "ponto",
        // everything else the instance default "vírgula".
        let render = |v: &FloatValue| -> Result<String> {
            if fractional_decimal {
                self.cardinal_float_with_pointword(v, "ponto")
            } else {
                self.cardinal_float_entry(v, None)
            }
        };
        if fv_numeric_negative(value) {
            return Ok(format!("{} antes de Cristo", render(&fv_abs(value))?));
        }
        render(value)
    }

    // ---- string inputs ----------------------------------------------------

    /// `Num2Word_PT_BR.str_to_number` (issue #63 ports
    /// savoirfairelinux/num2words#300).
    ///
    /// Brazilian Portuguese uses ',' as the decimal separator; a string
    /// written with '.' and no ',' is US-style and is pronounced "ponto"
    /// instead of "vírgula". Python stashes `_pending_pointword = "ponto"` on
    /// the instance and `to_cardinal_float` consumes it; here the handshake
    /// rides in `ParsedNumber::DecPoint`, consumed by
    /// [`Lang::cardinal_with_pointword`].
    ///
    /// "Infinity"/"NaN" parse fine as `Decimal`, but PT_BR's own
    /// `to_cardinal` then raises `decimal.InvalidOperation` — the `>= 10^9`
    /// branch runs `divmod(value, 10**12)` on Infinity, and NaN blows up
    /// even earlier, on the `value < 0` ordering comparison — instead of the
    /// base OverflowError/ValueError the binding's Inf/NaN arms model. Mapped
    /// here because str_to_number is the only per-language interception
    /// point; the corpus pins the cardinal rows (`InvalidOperation` for all
    /// three), and year/currency agree (`< 0` / `% 1` raise the same way).
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        // Python: s = str(value).strip(); "." in s and "," not in s
        let stripped = s.trim();
        let us_style = stripped.contains('.') && !stripped.contains(',');
        match python_decimal_parse(s)? {
            ParsedNumber::Dec(value) if us_style => Ok(ParsedNumber::DecPoint {
                value,
                pointword: "ponto",
            }),
            ParsedNumber::Inf { .. } | ParsedNumber::NaN => Err(N2WError::Custom {
                module: "decimal",
                class: "InvalidOperation",
                msg: "[<class 'decimal.InvalidOperation'>]".into(),
            }),
            other => Ok(other),
        }
    }

    // ---- fractions ---------------------------------------------------------

    /// `Num2Word_PT.to_fraction`, inherited unchanged — but `self.to_ordinal`
    /// and `self.to_cardinal` dispatch to PT_BR's own (short-scale) methods.
    ///
    /// Deviations from the Base default, all transcribed:
    ///   * idiomatic halves/thirds: "meio"/"meios", "terço"/"terços";
    ///   * the "s" plural is appended only when the ordinal does not already
    ///     end in "s" (Base appends unconditionally);
    ///   * `abs_n == 1` short-circuits the numerator to the literal "um".
    /// `denominator == 1` / `numerator == 0` return the *signed* cardinal,
    /// before any of that. `self.to_ordinal(abs_d)` can still raise (KeyError
    /// at 10^18+), which propagates exactly as in Python.
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

    /// `Num2Word_PT_BR.to_cardinal_float`.
    ///
    /// The corpus float/Decimal rows do **not** enter the base float path the
    /// way the 26 inheriting languages do. Python dispatches a non-integer to
    /// `Num2Word_PT_BR.to_cardinal(value)`, whose short-scale wrapper peels the
    /// 10^12 / 10^9 / 10^6 rungs off the *whole value* (fraction included) with
    /// `divmod`, and only the lowest group reaches the base
    /// `to_cardinal_float`. That routing renders the low group through
    /// `self.to_cardinal` (the millions branch, with its comma) rather than the
    /// base engine, so `default_to_cardinal_float` — which would call
    /// `to_cardinal(pre)` on the *entire* integer part and route the low group
    /// through the base engine — drops a comma:
    ///
    /// ```text
    /// Decimal("98746251323029.99")
    ///   default_to_cardinal_float: "… um milhões trezentos …"   (WRONG)
    ///   Num2Word_PT_BR.to_cardinal: "… um milhões, trezentos …" (right)
    /// ```
    ///
    /// So this override reproduces `Num2Word_PT_BR.to_cardinal` for a
    /// non-integer via [`LangPtBr::frac_cardinal`], which carries the `divmod`
    /// in the value's own domain so the f64 artefacts survive. The sign is
    /// peeled here, matching the Python `"%s%s" % (negword, …)` wrapper
    /// (`base.to_cardinal_float`'s own `pre == 0` negword branch is therefore
    /// dead for PT_BR and is not reproduced).
    ///
    /// `precision_override` is deliberately ignored: `Num2Word_PT_BR`'s Python
    /// override drops the `precision=` kwarg, and `base.float2tuple` resets the
    /// per-instance precision from `repr` regardless, so the live interpreter
    /// ignores it too (`num2words(12.34, lang='pt_BR', precision=1)` still
    /// yields two fractional digits).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        // No pending 'ponto' on this entry: a direct float/Decimal never went
        // through str_to_number, so the pointword is the instance default.
        self.cardinal_float_with_pointword(value, self.pointword())
    }

    /// `Num2Word_PT_BR.str_to_number` + `to_cardinal_float`'s pointword swap,
    /// consumed as one call (issue #63, savoirfairelinux/num2words#300).
    ///
    /// The binding routes `ParsedNumber::DecPoint` here for `to='cardinal'`.
    /// Python's `to_cardinal(Decimal)` first routes wholeness: a whole value
    /// ("12.") takes the int path and the pending "ponto" is never read, so
    /// `num2words("12.", lang='pt_BR')` is "doze"; a fractional one reaches
    /// `to_cardinal_float`, which swaps `self.pointword` to "ponto" for the
    /// duration of the call: ".5" -> "zero ponto cinco".
    fn cardinal_with_pointword(
        &self,
        value: &FloatValue,
        pointword: &str,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            return self.to_cardinal(&i);
        }
        self.cardinal_float_with_pointword(value, pointword)
    }

    // ---- currency -------------------------------------------------------
    //
    // PT_BR inherits `to_cheque`, `_money_verbose`, `_cents_verbose` and
    // `_cents_terse` from `Num2Word_Base` and `pluralize` from
    // `Num2Word_EUR`; all four Base methods are exactly the trait defaults, so
    // only `pluralize`, the two data tables, the class name and the rewritten
    // `to_currency` appear here. `currency_precision` is *not* overridden: see
    // the note in `new()`.

    fn lang_name(&self) -> &str {
        "Num2Word_PT_BR"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        self.currency_adjectives.get(code).copied()
    }

    /// `Num2Word_EUR.pluralize`: `forms[0 if n == 1 else 1]`.
    ///
    /// Python indexes the tuple directly, so a one-form entry with `n != 1`
    /// raises IndexError. Every entry in PT_BR's table has exactly two forms,
    /// so this is unreachable — but it is mapped to `Index` rather than
    /// panicking so the exception type survives if the table ever changes.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    /// `Num2Word_PT_BR.to_currency`.
    ///
    /// Two steps of its own around a `super()` call into
    /// [`LangPtBr::pt_to_currency`]: demote whole floats/Decimals to `int`
    /// first (quirk 8), then re-run the `" de"` insertion with the Brazilian
    /// scale words that PT's European list cannot match (quirk 9).
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // The trait hands us None when the caller omitted `separator=`;
        // resolve it to this language's own default (" e") before the body.
        let separator = separator.unwrap_or(self.default_separator());

        // if isinstance(val, Decimal) and val % 1 == 0:  val = int(val)
        // elif isinstance(val, float) and val == int(val): val = int(val)
        //
        // `CurrencyValue::Decimal` covers both Python types, and both tests
        // reduce to the same predicate — "has no fractional part". The
        // truncating `int()` and the sign-of-dividend `Decimal % 1` only
        // differ from a floor on *non*-integral input, which neither arm
        // accepts. So one `is_integer()` check serves both.
        //
        // This is quirk 8: it discards `has_decimal` and pushes 1.0 onto the
        // no-cents integer path, so the branch must come first, exactly here.
        let val: CurrencyValue = match val {
            CurrencyValue::Decimal { value, .. } if value.is_integer() => {
                CurrencyValue::Int(value.with_scale(0).as_bigint_and_exponent().0)
            }
            other => other.clone(),
        };

        // Use parent class implementation with our currency forms.
        let mut result = self.pt_to_currency(&val, currency, cents, separator, adjective)?;

        // For Brazilian Portuguese we need to add "de" after
        // millions/billions/trillions when they are round numbers.
        if let CurrencyValue::Int(v) = &val {
            // try: cr1, cr2 = self.CURRENCY_FORMS[currency] / except KeyError: pass
            if let Some(forms) = self.currency_forms.get(currency) {
                let cr1 = &forms.unit;
                // cr1[1] if abs(val) != 1 and len(cr1) > 1 else cr1[0]
                let abs_val = v.abs();
                let currency_str = if !abs_val.is_one() && cr1.len() > 1 {
                    cr1.get(1)
                } else {
                    cr1.first()
                }
                .ok_or_else(|| N2WError::Index("tuple index out of range".into()))?
                .clone();

                for ext in PT_BR_DE_EXTS {
                    if let Some(rewritten) = insert_de(&result, ext, &currency_str) {
                        result = rewritten;
                    }
                }
            }
        }

        // Note: no `.strip()` here, unlike PT's integer branch.
        Ok(result)
    }
}
