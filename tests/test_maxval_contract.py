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
    "az": 10**66,
    "be": 10**33,
    "ce": 10**34,
    "cs": 10**33,
    "cy": 999 * 10**33,
    "dv": 10**33,
    "hr": 10**33,
    "hu": 10**606,
    "it": 10**65,
    "kz": 10**33,
    "lij": 10**36,
    "lt": 10**33,
    "lv": 10**33,
    "mn": 10**69,
    "pl": 10**66,
    "pt_BR": 10**57,
    "rm": 10**65,
    "rm_puter": 10**65,
    "rm_surmiran": 10**65,
    "rm_sursilv": 10**65,
    "rm_sutsilv": 10**65,
    "rm_vallader": 10**65,
    "ru": 10**33,
    "sk": 10**33,
    "sr": 10**33,
    "sr_Latn": 10**33,
    "tr": 10**21 - 65536,
    "uk": 10**33,
    "uz_Cyrl": 10**33,
    "vi": 10**60,
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
