//! Presentation / dispatch helpers ported from `num2words2/__init__.py`.
//!
//! These three functions were the last pieces of pure-Python logic living in
//! the `num2words2` binder: language-code resolution, the `style=`
//! post-processing, and the `cents=` mode mapping. They are conversion-agnostic
//! string/enum transforms, so they belong in the pure core rather than the
//! Python surface. The PyO3 binder classifies the caller's arguments into the
//! plain types below and calls these directly.

/// Resolve a caller's language code to a core key. Matching is
/// case-insensitive and accepts `-` or `_` between subtags
/// (gladiaio/num2words2#238):
///
/// 1. exact match, then hyphen -> underscore, then
/// 2. a case-insensitive match on the whole code (`EN`, `PT-br`, `zh-tw`,
///    `sr_latn`, `uz_cyrl`, `EN_AERO_FAA`), then
/// 3. BCP-47 subtags: the language lower-cased, a 4-letter script
///    Title-cased, a 2-letter / 3-digit region upper-cased. A script must
///    name a known `lang_Script` key or the language's default script
///    ([`DEFAULT_SCRIPTS`]); any other script is rejected rather than
///    silently served in another script. A region falls back to the bare
///    language when there is no `lang_REGION` key (`en-US` -> `en`), then
/// 4. the bare two-letter prefix (`english` -> `en`).
///
/// Returns `None` when none match; the binder turns that into a
/// `NotImplementedError`.
pub fn resolve_lang(raw: &str) -> Option<String> {
    let keys = crate::supported_lang_keys();
    // `keys.contains(&s)` cannot be used: the keys are `&'static str` and the
    // needle is a borrowed `&str`, so the slice `contains` type-checks fail.
    #[allow(clippy::manual_contains)]
    let known = |s: &str| keys.iter().any(|k| *k == s);

    if known(raw) {
        return Some(raw.to_string());
    }
    let nl = raw.replace('-', "_");
    if known(&nl) {
        return Some(nl);
    }
    let lower = nl.to_lowercase();
    if let Some(k) = keys.iter().find(|k| k.to_lowercase() == lower) {
        return Some((*k).to_string());
    }

    let parts: Vec<&str> = lower.split('_').collect();
    if parts.len() >= 2 {
        let lang = parts[0];
        let mut rest = &parts[1..];
        if is_script(rest[0]) {
            let script = title_case(rest[0]);
            let candidate = format!("{}_{}", lang, script);
            if known(&candidate) {
                return Some(candidate);
            }
            if !DEFAULT_SCRIPTS.contains(&(lang, script.as_str())) {
                // Unknown script: never serve another script silently.
                return None;
            }
            rest = &rest[1..];
        }
        if let Some(region) = rest.first().filter(|r| is_region(r)) {
            let candidate = format!("{}_{}", lang, region.to_uppercase());
            if known(&candidate) {
                return Some(candidate);
            }
        }
        if known(lang) {
            return Some(lang.to_string());
        }
    }
    let prefix: String = lower.chars().take(2).collect();
    if known(&prefix) {
        return Some(prefix);
    }
    None
}

/// The script the bare key of a language with `lang_Script` variants is
/// written in, so that naming it explicitly resolves to the bare key
/// (`uz-Latn` -> `uz`). Other `lang-Script` codes without a key of their own
/// are rejected, since the bare key's script is not recorded for them.
const DEFAULT_SCRIPTS: &[(&str, &str)] = &[("sr", "Cyrl"), ("uz", "Latn")];

/// A BCP-47 script subtag: four ASCII letters.
fn is_script(s: &str) -> bool {
    s.len() == 4 && s.chars().all(|c| c.is_ascii_alphabetic())
}

/// A BCP-47 region subtag: two ASCII letters or three digits.
fn is_region(s: &str) -> bool {
    (s.len() == 2 && s.chars().all(|c| c.is_ascii_alphabetic()))
        || (s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()))
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c.flat_map(char::to_lowercase)).collect(),
        None => String::new(),
    }
}

/// `style=` presentation post-processing (issues #535, #562), mirroring
/// `num2words2.__init__._apply_style`. Operates on the rendered string, so it
/// is conversion-independent:
///
/// * `style="terse"` + `to == "ordinal"` strips a leading `"one "`/`"un "`/
///   `"uno "` (but never reduces the whole string to empty).
/// * `style="us"` + an English `lang` replaces `" and "` with `" "`.
pub fn apply_style(result: &str, style: Option<&str>, to: &str, lang: &str) -> String {
    let mut out = result.to_string();
    if style == Some("terse") && to == "ordinal" {
        for prefix in ["one ", "un ", "uno "] {
            if out.starts_with(prefix) && out.len() > prefix.len() {
                out = out[prefix.len()..].to_string();
                break;
            }
        }
    }
    if style == Some("us") && lang.starts_with("en") {
        out = out.replace(" and ", " ");
    }
    out
}

/// The `cents=` argument as the binder classifies the caller's Python object.
pub enum CentsArg<'a> {
    /// The kwarg was not supplied at all (Python default `True`).
    Absent,
    /// A real Python `bool`.
    Bool(bool),
    /// A Python `str` (only the three keywords are meaningful).
    Str(&'a str),
    /// Any other type — Python keeps it as-is, and the caller's
    /// `_cents in (True, False)` guard then rejects it.
    Other,
}

/// `cents='omit'|'verbose'|'terse'` -> the legacy bool the core expects
/// (issue #554), mirroring `num2words2.__init__._normalize_cents` **plus** the
/// caller's follow-up `_cents in (True, False)` guard.
///
/// Returns `Some((cents_bool, drop_cents))` when the value resolves to a usable
/// bool (absent, a real bool, or one of the three keywords); `drop_cents` means
/// the value should be truncated to an int so no cents segment appears.
/// Returns `None` when Python would have kept a non-bool value and the guard
/// would reject it (the binder then declines the call).
pub fn normalize_cents(cents: CentsArg) -> Option<(bool, bool)> {
    match cents {
        CentsArg::Absent => Some((true, false)),
        CentsArg::Str("omit") => Some((true, true)),
        CentsArg::Str("verbose") => Some((true, false)),
        CentsArg::Str("terse") => Some((false, false)),
        CentsArg::Bool(b) => Some((b, false)),
        CentsArg::Str(_) | CentsArg::Other => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Identity of the converter behind a key. Several languages are
    /// zero-sized types, so the data address alone is not unique; the
    /// vtable half of the wide pointer tells them apart.
    fn ptr(k: &str) -> *const dyn crate::base::Lang {
        let l: &dyn crate::base::Lang = crate::get_lang_by_key(k).unwrap();
        l as *const dyn crate::base::Lang
    }

    #[test]
    fn resolve_exact_and_hyphen() {
        assert_eq!(resolve_lang("en").as_deref(), Some("en"));
        assert_eq!(resolve_lang("pt_BR").as_deref(), Some("pt_BR"));
        // hyphen -> underscore, exact key
        assert_eq!(resolve_lang("pt-BR").as_deref(), Some("pt_BR"));
    }

    #[test]
    fn resolve_casing_and_prefix() {
        // en-US: no en_US key, casing candidate missing, prefix "en" wins.
        assert_eq!(resolve_lang("en-US").as_deref(), Some("en"));
        assert_eq!(resolve_lang("en_US").as_deref(), Some("en"));
        // Two-letter-prefix fallback on a longer word.
        assert_eq!(resolve_lang("english").as_deref(), Some("en"));
    }

    #[test]
    fn resolve_case_insensitive_bcp47() {
        // gladiaio/num2words2#238
        for (raw, want) in [
            ("EN", "en"),
            ("PT-br", "pt_BR"),
            ("zh-tw", "zh_TW"),
            ("sr_latn", "sr_Latn"),
            ("SR-LATN", "sr_Latn"),
            ("sr-Cyrl", "sr_Cyrl"),
            ("uz_cyrl", "uz_Cyrl"),
            ("uz-Latn", "uz"),
            ("uz_cyr", "uz_cyr"),
            ("CZ", "cz"),
            ("en_aero_icao", "en_aero_icao"),
            ("EN-us", "en"),
            ("sr-latn-RS", "sr_Latn"),
            ("English", "en"),
        ] {
            assert_eq!(resolve_lang(raw).as_deref(), Some(want), "{raw}");
        }
    }

    #[test]
    fn resolve_unknown_script_is_rejected() {
        // Never fall back to the other script (#238).
        assert_eq!(resolve_lang("sr_Latx"), None);
        assert_eq!(resolve_lang("uz-Arab"), None);
        assert_eq!(resolve_lang("sr-Latx-RS"), None);
    }

    #[test]
    fn lowercase_collisions_share_a_converter() {
        // The case-insensitive step picks any key with the same lowercase
        // spelling; that is only sound while they share a converter.
        let keys = crate::supported_lang_keys();
        for a in &keys {
            for b in &keys {
                if a.to_lowercase() == b.to_lowercase() {
                    assert_eq!(ptr(a), ptr(b), "{a} vs {b}");
                }
            }
        }
    }

    #[test]
    fn lang_aliases_cover_every_shared_converter() {
        let keys = crate::supported_lang_keys();
        for (alias, canonical) in crate::LANG_ALIASES {
            assert!(keys.contains(alias) && keys.contains(canonical), "{alias}");
            assert!(!crate::LANG_ALIASES.iter().any(|(a, _)| a == canonical));
            assert_eq!(ptr(alias), ptr(canonical), "{alias} -> {canonical}");
        }
        // Every key outside the alias list has a converter of its own.
        let own: Vec<_> =
            keys.iter().filter(|k| !crate::LANG_ALIASES.iter().any(|(a, _)| a == *k)).collect();
        for (i, a) in own.iter().enumerate() {
            for b in &own[i + 1..] {
                assert_ne!(ptr(a), ptr(b), "{a} and {b} share a converter; list one as an alias");
            }
        }
    }

    #[test]
    fn resolve_unknown() {
        assert_eq!(resolve_lang("unknown_lang"), None);
        assert_eq!(resolve_lang("zz"), None);
    }

    #[test]
    fn style_terse_strips_leading_one() {
        assert_eq!(
            apply_style("one hundredth", Some("terse"), "ordinal", "en"),
            "hundredth"
        );
        // Exactly "first" is untouched (no leading "one ").
        assert_eq!(apply_style("first", Some("terse"), "ordinal", "en"), "first");
        // terse only applies to ordinal.
        assert_eq!(
            apply_style("one hundred", Some("terse"), "cardinal", "en"),
            "one hundred"
        );
    }

    #[test]
    fn style_us_drops_and() {
        assert_eq!(
            apply_style("one thousand and one", Some("us"), "cardinal", "en"),
            "one thousand one"
        );
        // Non-English lang is untouched.
        assert_eq!(
            apply_style("mil y uno", Some("us"), "cardinal", "es"),
            "mil y uno"
        );
        // No style is a no-op.
        assert_eq!(apply_style("x and y", None, "cardinal", "en"), "x and y");
    }

    #[test]
    fn cents_modes() {
        assert_eq!(normalize_cents(CentsArg::Absent), Some((true, false)));
        assert_eq!(normalize_cents(CentsArg::Str("omit")), Some((true, true)));
        assert_eq!(normalize_cents(CentsArg::Str("verbose")), Some((true, false)));
        assert_eq!(normalize_cents(CentsArg::Str("terse")), Some((false, false)));
        assert_eq!(normalize_cents(CentsArg::Bool(true)), Some((true, false)));
        assert_eq!(normalize_cents(CentsArg::Bool(false)), Some((false, false)));
        assert_eq!(normalize_cents(CentsArg::Str("nope")), None);
        assert_eq!(normalize_cents(CentsArg::Other), None);
    }
}
