# -*- coding: utf-8 -*-
"""maxval(lang) matches what the converter does (gladiaio/num2words2#159).

maxval is the exclusive ceiling: maxval - 1 converts to words, maxval
raises OverflowError (never KeyError, NotImplementedError, None or a
placeholder string). Languages with no ceiling report None.
"""
from __future__ import unicode_literals

import pytest

from num2words2 import maxval, num2words

CEILINGS = {
    "as": 10**14,
    "az": 10**66,
    "ba": 10**12,
    "ban": 10**9,
    "be": 10**33,
    "bg": 10**12,
    "bm": 10**12,
    "bo": 10**16,
    "bs": 10**24,
    "ce": 10**34,
    "ceb": 10**15,
    "ckb": 10**12,
    "cs": 10**33,
    "cy": 999 * 10**33,
    "dv": 10**33,
    "et": 10**15,
    "eu": 10**12,
    "fa": 10**18,
    "ff": 10**12,
    "fil": 10**15,
    "fo": 10**24,
    "gl": 10**24,
    "gu": 10**22,
    "ha": 10**15,
    "hmn": 10**9,
    "hr": 10**33,
    "hu": 10**606,
    "hy": 10**15,
    "it": 10**65,
    "ka": 10**18,
    "ki": 10**9,
    "km": 10**18,
    "ku": 10**15,
    "kz": 10**33,
    "lb": 10**15,
    "lg": 10**12,
    "lij": 10**36,
    "lo": 10**18,
    "lt": 10**33,
    "lus": 10**9,
    "lv": 10**33,
    "miz": 10**9,
    "mk": 10**21,
    "mn": 10**69,
    "mr": 10**22,
    "ms": 10**15,
    "my": 10**14,
    "nn": 10**21,
    "oc": 10**15,
    "om": 10**12,
    "pap": 10**12,
    "pl": 10**66,
    "ps": 10**12,
    "pt_BR": 10**57,
    "rm": 10**65,
    "rm_puter": 10**65,
    "rm_surmiran": 10**65,
    "rm_sursilv": 10**65,
    "rm_sutsilv": 10**65,
    "rm_vallader": 10**65,
    "ru": 10**33,
    "rw": 10**15,
    "sk": 10**33,
    "sn": 10**15,
    "sr": 10**33,
    "sr_Latn": 10**33,
    "ta": 10**14,
    "ti": 10**12,
    "tr": 10**21 - 65536,
    "uk": 10**33,
    "uz_Cyrl": 10**33,
    "vi": 10**18,
    "xh": 10**12,
    "yi": 10**12,
    "zu": 10**12,
}

UNBOUNDED = [
    "th", "en_AERO", "en_Aero_FAA", "en_Aero_ICAO", "en_Aero_NATO",
    "en_Aero_USN", "en_Aero_US_Army", "en_Aero_US_Navy",
]


@pytest.mark.parametrize("lang", sorted(CEILINGS))
def test_maxval_is_the_real_ceiling(lang):
    m = CEILINGS[lang]
    assert maxval(lang) == m
    for v in (m - 1, -(m - 1)):
        assert isinstance(num2words(v, lang=lang), str)
    for v in (m, -m, m * 1000):
        with pytest.raises(OverflowError):
            num2words(v, lang=lang)


@pytest.mark.parametrize("lang", UNBOUNDED)
def test_unbounded_languages_report_none(lang):
    assert maxval(lang) is None
    for e in (12, 33, 100, 400):
        assert isinstance(num2words(10**e, lang=lang), str)


@pytest.mark.parametrize("lang", ["ru", "uk", "pl"])
def test_ordinal_overflows_too(lang):
    with pytest.raises(OverflowError):
        num2words(maxval(lang), lang=lang, to="ordinal")


def test_tr_message_names_the_largest_convertible_number():
    m = maxval("tr")
    with pytest.raises(OverflowError) as exc:
        num2words(m, lang="tr")
    assert str(m - 1) in str(exc.value)
    assert "en büyük rakam %d." % m not in str(exc.value)


def test_overflow_message_style():
    with pytest.raises(OverflowError, match=r"must be less than 10{33}\.$"):
        num2words(10**33, lang="ru")


# gladiaio/num2words2#203: these recursed over their top scale word with no
# ceiling, and a large enough integer overflowed the native stack (SIGSEGV).
NO_LONGER_UNBOUNDED = ["as", "ba", "bg", "bo", "et", "eu", "gu", "ha", "mr",
                       "ms", "sn", "ta"]


@pytest.mark.parametrize("lang", NO_LONGER_UNBOUNDED)
@pytest.mark.parametrize("to", ["cardinal", "ordinal", "year", "currency"])
def test_203_every_mode_overflows_at_the_ceiling(lang, to):
    m = maxval(lang)
    # Negative ordinals are a TypeError before any ceiling is consulted.
    values = (m, str(m)) if to == "ordinal" else (m, -m, str(m))
    for v in values:
        with pytest.raises(OverflowError):
            num2words(v, lang=lang, to=to)
