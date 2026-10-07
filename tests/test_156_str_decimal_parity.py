# -*- coding: utf-8 -*-
"""gladiaio/num2words2#156: str and Decimal input must read the same number
as the matching float/int, instead of silently dropping the fraction or the
sign (or raising KeyError)."""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words

LANGS = ["bg", "bn", "ce", "cy", "et", "ha", "sn"]


@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize(
    "f, s",
    [(1.5, "1.5"), (0.25, "0.25"), (-3, "-3"), (-1.5, "-1.5"), (12.345, "12.345")],
)
def test_float_str_decimal_agree(lang, f, s):
    if lang == "ha" and f == 12.345:
        pytest.skip(
            "ha's float arm words the binary remainder 0.3449999...; "
            "a Decimal is read exactly"
        )
    expected = num2words(f, lang=lang)
    assert num2words(s, lang=lang) == expected
    assert num2words(Decimal(s), lang=lang) == expected


@pytest.mark.parametrize(
    "lang, s, expected",
    [
        ("bg", "1.5", "един точка пет"),
        ("bg", "0.25", "нула точка две пет"),
        ("cy", "0.1", "dim pwynt un"),
        ("cy", "-1.5", "meinws un pwynt pump"),
        ("et", "1.5", "üks koma viis"),
        ("sn", "1.5", "motsi poindi shanu"),
        ("bn", "-3", "ঋণাত্মক তিন"),
        ("ce", "1.5", "цхьаъ а пхиъ"),
        ("ha", "1.5", "ɗaya wajen biyar"),
    ],
)
def test_examples_from_issue(lang, s, expected):
    assert num2words(s, lang=lang) == expected
    assert num2words(Decimal(s), lang=lang) == expected


def test_decimal_keeps_exact_digits():
    # #603: a fractional Decimal is read exactly, never through an f64
    # (float() would round this one to 98746251330.0). Kept below bg's 10**12
    # ceiling (#203).
    assert num2words(Decimal("98746251329.999999"), lang="bg").endswith(
        "двадесет и девет точка" + " девет" * 6
    )
