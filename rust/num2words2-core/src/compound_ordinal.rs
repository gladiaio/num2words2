//! Compound ordinals that inflect only their last component
//! (gladiaio/num2words2#248).
//!
//! In hr, sr, lt and lv a compound ordinal is the cardinal with its *last
//! word* replaced by that word's ordinal: hr 21 "dvadeset jedan" ->
//! "dvadeset prvi", lt 101 "vienas šimtas vienas" -> "vienas šimtas
//! pirmas". The Python originals glued a suffix onto the whole cardinal
//! instead ("dvadeset jedani"). This helper does the replacement for the
//! cases where the last word is certain to be one table entry.

/// The ordinal of `n` built from its own `cardinal` rendering, or `None`
/// when no rule applies (the caller keeps its previous behaviour).
///
/// * `n % 100 != 0`: the last word is the units word, or the whole teen or
///   round-tens word when `small` tables `n % 100`; it becomes `small(v)`.
/// * otherwise, with `hundreds` given and a non-zero hundreds digit, the
///   last word is that hundreds word and becomes `hundreds[digit]`.
pub fn last_word_ordinal(
    n: u64,
    cardinal: &str,
    small: impl Fn(u64) -> Option<&'static str>,
    hundreds: Option<&[&str; 10]>,
) -> Option<String> {
    let r = n % 100;
    let last = if r != 0 {
        small(r).or_else(|| small(r % 10))?
    } else {
        let h = ((n % 1000) / 100) as usize;
        if h == 0 {
            return None;
        }
        hundreds?[h]
    };
    Some(match cardinal.rsplit_once(' ') {
        Some((head, _)) => format!("{} {}", head, last),
        None => last.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(v: u64) -> Option<&'static str> {
        match v {
            1 => Some("first"),
            11 => Some("eleventh"),
            20 => Some("twentieth"),
            _ => None,
        }
    }

    #[test]
    fn replaces_only_the_last_word() {
        assert_eq!(last_word_ordinal(21, "twenty one", small, None).as_deref(),
                   Some("twenty first"));
        assert_eq!(last_word_ordinal(111, "hundred eleven", small, None).as_deref(),
                   Some("hundred eleventh"));
        assert_eq!(last_word_ordinal(120, "hundred twenty", small, None).as_deref(),
                   Some("hundred twentieth"));
        assert_eq!(last_word_ordinal(200, "two hundred", small, None), None);
        assert_eq!(last_word_ordinal(22, "twenty two", small, None), None);
    }
}
