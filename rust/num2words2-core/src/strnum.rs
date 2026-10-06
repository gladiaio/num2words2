//! String-input parsing: the Rust side of `converter.str_to_number` and the
//! `Decimal(value)` constructor semantics it relies on.
//!
//! Python's dispatcher hands string input to `str_to_number`, whose base
//! implementation is `Decimal(value)`. That constructor is *not*
//! `BigDecimal::from_str`: it strips whitespace, accepts a trailing dot
//! ("12."), PEP-515 underscores ("1_000"), any Unicode decimal digit
//! ("١٢٣"), and the special values Infinity/NaN. Reproducing its acceptance
//! set exactly is what keeps `num2words("12.")` and `num2words("abc")`
//! behaving identically to the Python original.

use crate::base::{N2WError, Result};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

/// What `str_to_number` produced, including the per-language side effects
/// Python stashes on `self` for the same-call handshake.
#[derive(Debug, Clone)]
pub enum ParsedNumber {
    /// Plain `Decimal(value)`.
    Dec(BigDecimal),
    /// `Decimal("Infinity")` — representable in Python, but `int()` on it
    /// raises OverflowError, which the integer path does not catch.
    Inf { negative: bool },
    /// `Decimal("NaN")` — `int()` raises ValueError, which the base integer
    /// path *does* catch, sending NaN into the float path (where it raises).
    NaN,
    /// lang_ES: "1ro" stashes `(n, gender)`; `to_cardinal` consumes it and
    /// renders `to_ordinal(n, gender=...)` instead.
    EsOrdinal { n: BigInt, gender: char },
    /// lang_PT_BR: US-style "1.50" (dot, no comma) pronounces "ponto"
    /// instead of "vírgula" for this call only.
    DecPoint {
        value: BigDecimal,
        pointword: &'static str,
    },
}

fn invalid() -> N2WError {
    // decimal.InvalidOperation — a real class from the decimal module, so
    // `except decimal.InvalidOperation` in caller code keeps working.
    N2WError::Custom {
        module: "decimal",
        class: "InvalidOperation",
        msg: "[<class 'decimal.ConversionSyntax'>]".into(),
    }
}

/// The value of `ch` as a Unicode decimal digit, as Python's `int()` /
/// `Decimal()` accept (Nd category). Covers the blocks that show up in
/// practice; anything unlisted is rejected, which surfaces as
/// InvalidOperation exactly like a genuinely invalid character.
pub fn unicode_digit(ch: char) -> Option<u32> {
    if ch.is_ascii_digit() {
        return Some(ch as u32 - '0' as u32);
    }
    let c = ch as u32;
    for &start in &[
        0x0660, // Arabic-Indic
        0x06F0, // Extended Arabic-Indic
        0x0966, // Devanagari
        0x09E6, // Bengali
        0x0A66, // Gurmukhi
        0x0AE6, // Gujarati
        0x0B66, // Oriya
        0x0BE6, // Tamil
        0x0C66, // Telugu
        0x0CE6, // Kannada
        0x0D66, // Malayalam
        0x0E50, // Thai
        0x0ED0, // Lao
        0x0F20, // Tibetan
        0x1040, // Myanmar
        0x17E0, // Khmer
        0x1810, // Mongolian
        0xFF10, // Fullwidth
    ] {
        if (start..start + 10).contains(&c) {
            return Some(c - start);
        }
    }
    None
}

/// `any(ch.isdigit() for ch in s)` — the dispatcher's "does this string
/// contain digits" test that decides between the sentence fallback and
/// re-raising. str.isdigit() is wider than Nd (superscripts count), but the
/// decimal blocks above cover every case the library's callers hit.
pub fn has_py_digit(s: &str) -> bool {
    s.chars().any(|c| unicode_digit(c).is_some() || c.is_numeric())
}

/// Python's `Decimal(str)` constructor.
///
/// Grammar (case-insensitive, surrounding whitespace stripped):
///   sign? ( digits '.' digits? | '.' digits | digits ) ('e' sign? digits)?
///   sign? ('inf' | 'infinity')
///   sign? ('nan' | 'snan') digits?
/// PEP 515: '_' allowed only between two digits.
pub fn python_decimal_parse(s: &str) -> Result<ParsedNumber> {
    let t = s.trim();
    if t.is_empty() {
        return Err(invalid());
    }
    let lower = t.to_lowercase();
    let (sign_neg, rest) = match lower.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, lower.strip_prefix('+').unwrap_or(&lower)),
    };
    if rest == "inf" || rest == "infinity" {
        return Ok(ParsedNumber::Inf { negative: sign_neg });
    }
    if rest == "nan" || rest == "snan"
        || (rest.starts_with("nan") && rest[3..].chars().all(|c| c.is_ascii_digit()) && !rest[3..].is_empty())
        || (rest.starts_with("snan") && rest[4..].chars().all(|c| c.is_ascii_digit()) && !rest[4..].is_empty())
    {
        return Ok(ParsedNumber::NaN);
    }

    // Numeric form: normalise unicode digits to ASCII and validate the
    // shape, then let BigDecimal parse the clean ASCII.
    let src = t; // keep original case (irrelevant for digits) minus strip
    let mut out = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    if let Some(&c) = chars.peek() {
        if c == '+' || c == '-' {
            if c == '-' {
                out.push('-');
            }
            chars.next();
        }
    }
    let mut int_digits = 0usize;
    let mut frac_digits = 0usize;
    let mut seen_dot = false;
    let mut exp_part = String::new();
    let mut prev_was_digit = false;
    while let Some(c) = chars.next() {
        if let Some(d) = unicode_digit(c) {
            out.push(char::from(b'0' + d as u8));
            if seen_dot {
                frac_digits += 1;
            } else {
                int_digits += 1;
            }
            prev_was_digit = true;
        } else if c == '_' {
            // PEP 515: must sit between two digits.
            let next_is_digit = chars.peek().is_some_and(|&n| unicode_digit(n).is_some());
            if !prev_was_digit || !next_is_digit {
                return Err(invalid());
            }
            prev_was_digit = false;
        } else if c == '.' {
            if seen_dot {
                return Err(invalid());
            }
            seen_dot = true;
            out.push('.');
            prev_was_digit = false;
        } else if c == 'e' || c == 'E' {
            // exponent: sign? digits (underscores allowed between digits)
            exp_part.push('E');
            if let Some(&sc) = chars.peek() {
                if sc == '+' || sc == '-' {
                    exp_part.push(sc);
                    chars.next();
                }
            }
            let mut exp_digits = 0usize;
            let mut prev_exp_digit = false;
            for ec in chars.by_ref() {
                if let Some(d) = unicode_digit(ec) {
                    exp_part.push(char::from(b'0' + d as u8));
                    exp_digits += 1;
                    prev_exp_digit = true;
                } else if ec == '_' {
                    if !prev_exp_digit {
                        return Err(invalid());
                    }
                    prev_exp_digit = false;
                } else {
                    return Err(invalid());
                }
            }
            if exp_digits == 0 || !prev_exp_digit {
                return Err(invalid());
            }
            break;
        } else {
            return Err(invalid());
        }
    }
    if int_digits == 0 && frac_digits == 0 {
        return Err(invalid());
    }
    // "12." is valid; BigDecimal's parser rejects a trailing dot, so drop it.
    if out.ends_with('.') {
        out.pop();
    }
    // ".5" needs a leading zero for BigDecimal.
    if out.starts_with('.') {
        out.insert(0, '0');
    } else if out.starts_with("-.") {
        out.insert(1, '0');
    }
    out.push_str(&exp_part);
    BigDecimal::from_str(&out)
        .map(ParsedNumber::Dec)
        .map_err(|_| invalid())
}

/// Characters accepted as thousands separators in any language: space,
/// NBSP, narrow NBSP, thin space, ASCII apostrophe and U+2019 (Swiss style).
/// None of them can be a decimal mark, so a group built from them is never
/// ambiguous.
pub fn is_space_group_sep(c: char) -> bool {
    matches!(c, ' ' | '\u{00A0}' | '\u{202F}' | '\u{2009}' | '\'' | '\u{2019}')
}

/// How a language writes the number 1234.5 (#177), as far as grouping with
/// '.' and ',' is concerned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notation {
    /// `1,234.5`: '.' is the decimal mark, so a single ',' followed by
    /// exactly three digits ("1,000") is a thousands separator.
    CommaGroups,
    /// `1.234,5`: ',' is the decimal mark and '.' the standard thousands
    /// separator, so a single '.' followed by exactly three digits ("1.000")
    /// is a thousands separator.
    DotGroups,
    /// Not known to be either — including the comma-decimal languages that
    /// group with spaces (`1 234,5`: fr, ru, pl, sv, …), where a lone "1.000"
    /// is not standard notation. A single ',' or '.' followed by three digits
    /// keeps its old reading (ambiguous ',' raises, '.' is a decimal point).
    Unspecified,
}

/// The [`Notation`] of a language key. Deliberately limited to languages
/// whose convention is well established; regional variants are listed
/// explicitly rather than inherited (Spain writes `1.000,5`, Mexico and
/// Central America `1,000.5`), except en_* and zh_*, which all write
/// `1,000.5`.
pub fn number_notation(lang: &str) -> Notation {
    let key = lang.to_ascii_lowercase().replace('-', "_");
    let base = key.split('_').next().unwrap_or("");
    if base == "en" || base == "zh" {
        return Notation::CommaGroups;
    }
    match key.as_str() {
        "ja" | "ko" | "hi" | "bn" | "gu" | "kn" | "ml" | "mr" | "pa" | "ta" | "te" | "ur"
        | "th" | "he" | "ms" | "fil" | "tl" | "mt" | "cy" | "sw" => Notation::CommaGroups,
        "de" | "es" | "it" | "pt" | "pt_br" | "nl" | "da" | "id" | "tr" | "el" | "ro" | "hr"
        | "sl" | "sr" | "sr_latn" | "vi" | "ca" | "gl" | "is" => Notation::DotGroups,
        _ => Notation::Unspecified,
    }
}

/// Whether a language groups thousands with a space (`10 000`, `1 234,5`):
/// fr, ru, uk, pl, cs, sk, sv, nb/nn/no, fi, et, lt, lv, bg, hu. Typed text
/// uses a plain ASCII space as often as a no-break one, so the sentence
/// converter accepts it as grouping for these languages only (#233).
pub fn groups_with_spaces(lang: &str) -> bool {
    let key = lang.to_ascii_lowercase().replace('-', "_");
    matches!(
        key.split('_').next().unwrap_or(""),
        "fr" | "ru" | "uk" | "pl" | "cs" | "sk" | "sv" | "nb" | "nn" | "no" | "fi" | "et"
            | "lt" | "lv" | "bg" | "hu"
    )
}

/// Outcome of [`parse_grouped`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grouped {
    /// Not a pure numeric string with grouping/decimal separators — leave it
    /// to the existing routing (Decimal, sentence converter).
    NotGrouped,
    /// The number, as a plain ASCII decimal string (`-1234.5`), plus whether
    /// the original decimal mark was a comma.
    Number { canonical: String, decimal_comma: bool },
    /// A separator-bearing numeric string that cannot be read safely
    /// (ambiguous or malformed). The message says why.
    Invalid(String),
}

/// Recognise a *pure* numeric string written with thousands separators
/// (`1,000`, `1.000.000`, `1 000`, `1'000`, `1,234.5`, `1.234,5`) — issue #151.
///
/// Rules, chosen so the result is never a different number than the one
/// written:
///   * space-like separators and apostrophes are always grouping;
///   * a string with both '.' and ',' uses the last one as decimal mark;
///   * a '.' or ',' occurring twice or more is a group separator;
///   * a single ',' or '.' next to space-like grouping is the decimal mark;
///   * otherwise a single ',' followed by exactly three digits is grouping
///     only for [`Notation::CommaGroups`], else ambiguous; a single ','
///     followed by any other digit count is a decimal comma;
///   * a single '.' followed by exactly three digits after a 1-3 digit head
///     not starting with 0 is grouping only for [`Notation::DotGroups`]; any
///     other lone '.' is a plain decimal and left alone (`NotGrouped`);
///   * a lone ',' after a leading 0 ("0,500") is always a decimal comma;
///   * groups must be a 1–3 digit head followed by exact 3-digit groups.
///
/// Only ASCII digits are recognised; anything else yields `NotGrouped`.
pub fn parse_grouped(s: &str, notation: Notation) -> Grouped {
    let t = s.trim();
    let (neg, body) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if body.is_empty()
        || !body.starts_with(|c: char| c.is_ascii_digit())
        || !body.ends_with(|c: char| c.is_ascii_digit())
        || !body
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == ',' || is_space_group_sep(c))
    {
        return Grouped::NotGrouped;
    }
    let n_dot = body.matches('.').count();
    let n_comma = body.matches(',').count();
    let has_space = body.chars().any(is_space_group_sep);
    if !has_space && n_dot + n_comma == 0 {
        return Grouped::NotGrouped;
    }
    // A single '.' with nothing else is a plain decimal — Decimal handles
    // it; never reinterpret it here — unless the language groups with '.'
    // and it is followed by exactly three digits ("1.000" in de, #177).
    if !has_space && n_comma == 0 && n_dot == 1 {
        let (head, tail) = body.split_once('.').unwrap();
        if notation != Notation::DotGroups
            || head.len() > 3
            || head.starts_with('0')
            || tail.len() != 3
        {
            return Grouped::NotGrouped;
        }
    }
    let bad = |why: &str| Grouped::Invalid(format!("cannot read {:?} as a number: {}", t, why));

    // Pick the decimal mark (if any); every other separator is grouping.
    let decimal: Option<char> = if n_dot > 0 && n_comma > 0 {
        let last = body.rfind(['.', ',']).map(|i| body.as_bytes()[i] as char).unwrap();
        Some(last)
    } else if n_dot + n_comma == 0 {
        None
    } else {
        let (c, count) = if n_dot > 0 { ('.', n_dot) } else { (',', n_comma) };
        if count >= 2 {
            None
        } else if has_space {
            Some(c)
        } else if c == '.' {
            // A lone '.' only gets here as DotGroups grouping (see above).
            None
        } else {
            // Exactly one ',' (a lone '.' was handled above).
            let frac_len = body.len() - body.find(',').unwrap() - 1;
            // "0,500" has no thousands to group: a decimal comma.
            if frac_len != 3 || body.starts_with('0') {
                Some(',')
            } else if notation == Notation::CommaGroups {
                None
            } else {
                return bad("',' could be a decimal mark or a thousands separator");
            }
        }
    };
    let (int_part, frac_part) = match decimal {
        Some(d) => {
            if body.matches(d).count() != 1 {
                return bad("more than one decimal mark");
            }
            let (i, f) = body.split_once(d).unwrap();
            (i, Some(f))
        }
        None => (body, None),
    };
    if let Some(f) = frac_part {
        if f.is_empty() || !f.chars().all(|c| c.is_ascii_digit()) {
            return bad("separator after the decimal mark");
        }
    }
    // Group separators: one kind only.
    let mut gsep: Option<char> = None;
    for c in int_part.chars().filter(|c| !c.is_ascii_digit()) {
        match gsep {
            None => gsep = Some(c),
            Some(g) if g == c => {}
            Some(_) => return bad("mixed thousands separators"),
        }
    }
    let mut digits = String::new();
    if gsep.is_some() {
        let groups: Vec<&str> = int_part.split(gsep.unwrap()).collect();
        if groups[0].is_empty() || groups[0].len() > 3 || groups[1..].iter().any(|g| g.len() != 3) {
            return bad("thousands groups must have exactly three digits");
        }
        for g in groups {
            digits.push_str(g);
        }
    } else {
        digits.push_str(int_part);
    }
    let mut canonical = String::new();
    if neg {
        canonical.push('-');
    }
    canonical.push_str(&digits);
    if let Some(f) = frac_part {
        canonical.push('.');
        canonical.push_str(f);
    }
    Grouped::Number { canonical, decimal_comma: decimal == Some(',') }
}

/// A single token that is plainly meant as a number but is not one Python's
/// `Decimal()` (or [`parse_grouped`]) accepts: only ASCII digits, `_` and
/// `.` after an optional sign ("1__0", "_1", "1_000_", "1..2"), or a
/// `0x`/`0o`/`0b` literal ("0x10"). The dispatcher raises `ValueError` for
/// these instead of handing them to the sentence converter, which read
/// "1__0" as "One__zero" (gladiaio/num2words2#237). Anything with other
/// letters ("H2O", "1st", "10%") is still text.
pub fn is_malformed_number(s: &str) -> bool {
    let t = s.trim();
    let body = t.strip_prefix(['+', '-']).unwrap_or(t);
    if !body.bytes().any(|b| b.is_ascii_digit()) {
        return false;
    }
    if body.bytes().all(|b| b.is_ascii_digit() || b == b'_' || b == b'.') {
        return true;
    }
    let lower = body.to_ascii_lowercase();
    let radix = |p: &str, ok: fn(u8) -> bool| {
        lower.strip_prefix(p).is_some_and(|r| {
            !r.is_empty() && r.bytes().all(|b| ok(b) || b == b'_')
        })
    };
    radix("0x", |b| b.is_ascii_hexdigit())
        || radix("0o", |b| (b'0'..=b'7').contains(&b))
        || radix("0b", |b| b == b'0' || b == b'1')
}

/// Python's `int(str)` — used by the "n/d" fraction-string branch. Accepts
/// surrounding whitespace, a sign, PEP-515 underscores and Unicode digits;
/// no dot, no exponent.
pub fn python_int_parse(s: &str) -> Option<BigInt> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    if rest.is_empty() {
        return None;
    }
    let mut ascii = String::with_capacity(rest.len());
    let mut prev_was_digit = false;
    let mut iter = rest.chars().peekable();
    while let Some(c) = iter.next() {
        if let Some(d) = unicode_digit(c) {
            ascii.push(char::from(b'0' + d as u8));
            prev_was_digit = true;
        } else if c == '_' {
            let next_is_digit = iter.peek().is_some_and(|&n| unicode_digit(n).is_some());
            if !prev_was_digit || !next_is_digit {
                return None;
            }
            prev_was_digit = false;
        } else {
            return None;
        }
    }
    let mut v = BigInt::from_str(&ascii).ok()?;
    if neg {
        v = -v;
    }
    Some(v)
}

/// Python's `str(Decimal)` in positional notation — the fixed-point branch of
/// the General Decimal Arithmetic to-scientific-string algorithm, applied to
/// every value. Python switches to scientific form (`1E+3`, `1E-7`) for a
/// positive exponent or a small adjusted exponent; every language reader
/// downstream `int()`s or digit-walks this string, so the exponent form made
/// them crash or read the mantissa (gladiaio/num2words2#211). Values are
/// therefore always written out: `Decimal('1E+3')` -> `"1000"`,
/// `Decimal('1E-7')` -> `"0.0000001"`. Needed because `has_decimal` checks
/// `"." in str(number)` and the language readers walk its digits.
pub fn python_decimal_str(d: &BigDecimal) -> String {
    let (mant, scale) = d.as_bigint_and_exponent();
    let exponent = -scale; // Python's as_tuple().exponent
    let neg = mant.sign() == num_bigint::Sign::Minus;
    let digits = mant.magnitude().to_string();
    let ndigits = digits.len() as i64;
    let sign = if neg { "-" } else { "" };

    if exponent >= 0 {
        return format!("{}{}{}", sign, digits, "0".repeat(exponent as usize));
    }
    let point = ndigits + exponent;
    if point <= 0 {
        return format!("{}0.{}{}", sign, "0".repeat((-point) as usize), digits);
    }
    let (i, f) = digits.split_at(point as usize);
    format!("{}{}.{}", sign, i, f)
}

#[cfg(test)]
mod malformed_tests {
    use super::*;

    #[test]
    fn malformed_numbers() {
        for s in ["0x10", "0X1F", "0b101", "0o17", "1__0", "_1", "1_000_", "-_1",
                  "1..2", " 1__0 "] {
            assert!(is_malformed_number(s), "{}", s);
        }
        for s in ["H2O", "1st", "10%", "abc", "1-2", "5 kg", "x1", "0xZZ", "", "_"] {
            assert!(!is_malformed_number(s), "{}", s);
        }
    }
}

#[cfg(test)]
mod decimal_str_tests {
    use super::*;

    #[test]
    fn never_scientific() {
        for (s, want) in [("1E+3", "1000"), ("1.5E+3", "1500"), ("1E-7", "0.0000001"),
                          ("0.00001", "0.00001"), ("-12.50", "-12.50"), ("0", "0"),
                          ("0.0", "0.0"), ("123", "123")] {
            let d = BigDecimal::from_str(s).unwrap();
            assert_eq!(python_decimal_str(&d), want, "{}", s);
        }
    }
}

#[cfg(test)]
mod grouped_tests {
    use super::*;

    const CG: Notation = Notation::CommaGroups;
    const DG: Notation = Notation::DotGroups;
    const UN: Notation = Notation::Unspecified;

    fn num(c: &str, dc: bool) -> Grouped {
        Grouped::Number { canonical: c.into(), decimal_comma: dc }
    }

    #[test]
    fn grouping_is_parsed() {
        assert_eq!(parse_grouped("1,000", CG), num("1000", false));
        assert_eq!(parse_grouped("1,000,000", UN), num("1000000", false));
        assert_eq!(parse_grouped("-12,345", CG), num("-12345", false));
        assert_eq!(parse_grouped("1,234.5", UN), num("1234.5", false));
        assert_eq!(parse_grouped("1.234,5", CG), num("1234.5", true));
        assert_eq!(parse_grouped("1.000.000", UN), num("1000000", false));
        assert_eq!(parse_grouped("1 000", UN), num("1000", false));
        assert_eq!(parse_grouped("1\u{202F}000,25", UN), num("1000.25", true));
        assert_eq!(parse_grouped("1'000'000", UN), num("1000000", false));
        assert_eq!(parse_grouped("1,5", UN), num("1.5", true));
        assert_eq!(parse_grouped("0,500", CG), num("0.500", true));
        assert_eq!(parse_grouped("0,500", UN), num("0.500", true));
    }

    #[test]
    fn ambiguous_or_malformed_is_rejected() {
        for (s, cg) in [("1,000", UN), ("1,2,3", CG), ("1,0000,000", CG),
                        ("1.2.3", CG), ("1,000.000,5", CG), ("1 00", UN),
                        ("1 000'000", UN)] {
            assert!(matches!(parse_grouped(s, cg), Grouped::Invalid(_)), "{}", s);
        }
    }

    #[test]
    fn untouched_inputs() {
        for s in ["1.5", "1000", "abc", "1,000 people", "1e5", "", "-", "1,"] {
            assert_eq!(parse_grouped(s, CG), Grouped::NotGrouped, "{}", s);
        }
    }

    #[test]
    fn lone_dot_groups_only_in_dot_grouping_languages() {
        // #177: "1.000" is a thousand in de, a decimal elsewhere.
        assert_eq!(parse_grouped("1.000", DG), num("1000", false));
        assert_eq!(parse_grouped("-12.345", DG), num("-12345", false));
        for (s, nt) in [("1.000", CG), ("1.000", UN), ("1.5", DG), ("1.50", DG),
                        ("1.0000", DG), ("1234.567", DG), ("0.123", DG)] {
            assert_eq!(parse_grouped(s, nt), Grouped::NotGrouped, "{} {:?}", s, nt);
        }
    }

    #[test]
    fn notation_table() {
        assert_eq!(number_notation("en"), CG);
        assert_eq!(number_notation("en_IN"), CG);
        assert_eq!(number_notation("zh_TW"), CG);
        assert_eq!(number_notation("ja"), CG);
        assert_eq!(number_notation("de"), DG);
        assert_eq!(number_notation("pt_BR"), DG);
        assert_eq!(number_notation("es"), DG);
        // Regional variants are not inherited; space-grouping languages and
        // unknown keys keep the conservative reading.
        assert_eq!(number_notation("es_gt"), UN);
        assert_eq!(number_notation("fr"), UN);
        assert_eq!(number_notation("ru"), UN);
        assert_eq!(number_notation("ar"), UN);
    }
}
