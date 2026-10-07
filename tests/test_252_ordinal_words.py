# -*- coding: utf-8 -*-
"""Wrong ordinal words in major languages (gladiaio/num2words2#252)."""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

SPANISH = ["es", "es_CO", "es_CR", "es_GT", "es_HN", "es_NI", "es_VE"]


@pytest.mark.parametrize("lang", SPANISH)
def test_es_vigesimo_keeps_its_accent(lang):
    assert num2words(20, lang=lang, to="ordinal") == "vigésimo"
    assert num2words(20, lang=lang, to="ordinal", gender="f") == "vigésima"
    assert num2words(120, lang=lang, to="ordinal") == "centésimo vigésimo"
    # The fused 21..29 forms drop the accent of the first element (RAE).
    assert num2words(21, lang=lang, to="ordinal") == "vigesimoprimero"


@pytest.mark.parametrize(
    "value, expected",
    [
        (20, "tjugonde"),
        (120, "etthundratjugonde"),
        (10**6, "miljonte"),
    ],
)
def test_sv(value, expected):
    assert num2words(value, lang="sv", to="ordinal") == expected


@pytest.mark.parametrize(
    "value, expected",
    [
        (30, "tredivte"),
        (40, "fyrrende"),
        (100, "hundrede"),
        (200, "tohundrede"),
        (1000, "tusinde"),
        (2000, "totusinde"),
    ],
)
def test_da(value, expected):
    assert num2words(value, lang="da", to="ordinal") == expected


@pytest.mark.parametrize(
    "value, expected",
    [
        (100, "al o sutălea"),
        (101, "al o sută unulea"),
        (1000, "al o miilea"),
        (1001, "al o mie unulea"),
        (2000, "al două miilea"),
        (10**6, "al un milionulea"),
    ],
)
def test_ro(value, expected):
    assert num2words(value, lang="ro", to="ordinal") == expected


@pytest.mark.parametrize(
    "lang, expected",
    [
        ("it", "milionesimo"),
        ("nl", "miljoenste"),
        ("nb", "millionte"),
        ("sv", "miljonte"),
        ("bg", "милионен"),
    ],
)
def test_one_millionth_has_no_article(lang, expected):
    assert num2words(10**6, lang=lang, to="ordinal") == expected
