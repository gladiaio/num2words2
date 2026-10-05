# -*- coding: utf-8 -*-
"""Cross-language invariants, run over every supported language code.

Each check lists the languages that fail it today in an allow-list. The
test fails when a language outside its allow-list breaks the invariant,
and also when a language on the list starts passing, so a fix has to
shrink the list in the same change. The lists may shrink but never grow.
"""
from __future__ import unicode_literals

import re
import unicodedata
from decimal import Decimal

import pytest

import num2words2
from num2words2 import _rust, maxval, num2words

LANGS = sorted(_rust.supported_langs())

# English and its variants legitimately say zero/point/minus.
ENGLISH = {lang for lang in LANGS if lang.startswith("en")}

# Exceptions a caller may be expected to handle. Anything else
# (AttributeError, KeyError, IndexError, AssertionError, ...) is a bug.
EXPECTED_ERRORS = (TypeError, ValueError, OverflowError, NotImplementedError)

CONVERTERS = ["cardinal", "ordinal", "ordinal_num", "year", "currency"]
INPUTS = [0, 1, -1, 2, 11, 21, 100, 1100, 0.5, -0.5, 1.5, "12", "1.5",
          Decimal("0.1"), Decimal("1.5"), -42]


def _call(x, lang, to):
    try:
        return num2words(x, lang=lang, to=to), None
    except Exception as e:  # noqa: BLE001 - classifying is the point
        return None, e


def _exception_failures(lang):
    out = []
    for to in CONVERTERS:
        for x in INPUTS:
            _, err = _call(x, lang, to)
            if err is None:
                continue
            if not isinstance(err, EXPECTED_ERRORS) or not str(err):
                out.append((to, x, type(err).__name__, str(err)[:60]))
    return out


def _hygiene_failures(lang):
    out = []
    for to in CONVERTERS:
        for x in INPUTS:
            r, err = _call(x, lang, to)
            if err is not None:
                continue
            if not isinstance(r, str) or not r or r != r.strip() or "  " in r:
                out.append((to, x, r))
    return out


def _is_non_latin_letter(ch):
    return ch.isalpha() and not unicodedata.name(ch, "").startswith("LATIN")


def _english_word_failures(lang):
    """English base-class words leaking into another language: Latin letters
    mixed into non-Latin-script output, or the English decimal word."""
    out = []
    for x, to in [(0, "cardinal"), (1.5, "cardinal"), (-1, "cardinal"),
                  (0, "ordinal")]:
        r, err = _call(x, lang, to)
        if err is not None or not isinstance(r, str):
            continue
        mixed = re.search(r"[A-Za-z]", r) and any(map(_is_non_latin_letter, r))
        if mixed or re.search(r"(^|\s)point(\s|$)", r):
            out.append((to, x, r))
    return out


def _parity_failures(lang):
    out = []
    for f, s in [(1.5, "1.5"), (0.25, "0.25"), (-3, "-3")]:
        a, ea = _call(f, lang, "cardinal")
        b, eb = _call(s, lang, "cardinal")
        c, ec = _call(Decimal(s), lang, "cardinal")
        if ea or eb or ec or not (a == b == c):
            out.append((s, a, b, c, type(ea or eb or ec).__name__))
    return out


def _maxval_failures(lang):
    m = maxval(lang)
    out = []
    if m is not None:
        r, err = _call(m - 1, lang, "cardinal")
        if err is not None or not isinstance(r, str):
            out.append(("maxval-1", type(err).__name__ if err else repr(r)))
        r, err = _call(m, lang, "cardinal")
        if not isinstance(err, OverflowError):
            out.append(("maxval", type(err).__name__ if err else repr(r)[:40]))
    else:
        for e in (12, 18, 24, 30, 36, 48, 60, 66, 72, 100):
            r, err = _call(10**e, lang, "cardinal")
            if err is None and isinstance(r, str):
                continue
            if isinstance(err, OverflowError):
                break
            out.append(("10**%d" % e, type(err).__name__ if err else repr(r)))
            break
    return out


CHECKS = {
    "exceptions": _exception_failures,
    "hygiene": _hygiene_failures,
    "english_words": _english_word_failures,
    "parity": _parity_failures,
    "maxval": _maxval_failures,
}

# Languages whose own decimal word is spelled "point".
NATIVE_ENGLISH_LOOKALIKES = set()

# Languages failing each check on main. Fixes remove entries; nothing is
# ever added. Issue numbers point at the open work.
ALLOW = {
    "exceptions": set(),
    # gladiaio/num2words2#160
    "hygiene": set(),
    # gladiaio/num2words2#154
    "english_words": {
        "br", "haw", "ht", "jv", "jw", "kk", "ln", "mg", "mi", "mt", "sd",
        "so", "su", "tk", "tl", "uz", "wo", "yo",
    },
    # gladiaio/num2words2#156 (pt_BR differs on purpose, see #92)
    "parity": {
        "pt_BR",
    },
    "maxval": set(),
}


@pytest.mark.parametrize("check", sorted(CHECKS))
def test_invariant(check):
    allowed = ALLOW[check]
    if check == "english_words":
        langs = [lg for lg in LANGS if lg not in ENGLISH]
        allowed = allowed | NATIVE_ENGLISH_LOOKALIKES
    else:
        langs = LANGS
    failing = {}
    for lang in langs:
        f = CHECKS[check](lang)
        if f:
            failing[lang] = f
    new = {lg: f[:3] for lg, f in failing.items() if lg not in allowed}
    stale = sorted(lg for lg in ALLOW[check] if lg in langs and lg not in failing)
    assert not new, "%s: languages newly failing: %r" % (check, new)
    assert not stale, (
        "%s: these languages pass now; remove them from ALLOW[%r]: %s"
        % (check, check, stale)
    )


def test_version_matches_metadata():
    import importlib.metadata

    assert num2words2.__version__ == importlib.metadata.version("num2words2")
