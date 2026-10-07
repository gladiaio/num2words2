# -*- coding: utf-8 -*-
"""gladiaio/num2words2#222: currency nouns are the language's own, in its
own script; a code with no sourced native noun raises NotImplementedError
instead of borrowing English."""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

CASES = [
    ("te", 1, None, "ఒకటి యూరో"),
    ("te", 2, "INR", "రెండు రూపాయలు"),
    ("te", 1, "USD", "ఒకటి డాలర్"),
    ("kn", -3, "USD", "(-) ಮೂರು ಡಾಲರ್"),
    ("kn", 2, "INR", "ಎರಡು ರೂಪಾಯಿ"),
    ("hu", 5, "USD", "öt dollár"),
    ("hu", 2.5, "EUR", "két euró, ötven cent"),
    ("mk", 5, "USD", "пет долари"),
    ("sv", 5, "USD", "fem dollar"),
    ("sv", 2.5, None, "två euro, femtio cent"),
    ("ka", 5, "USD", "ხუთი დოლარი"),
    ("ur", 5, "USD", "پانچ ڈالر"),
    ("mt", 2, "USD", "tnejn dollari"),
    ("nn", 2, "EUR", "to euro"),
]


@pytest.mark.parametrize("lang,value,code,expected", CASES)
def test_native_currency_nouns(lang, value, code, expected):
    kw = {"currency": code} if code else {}
    assert num2words(value, lang=lang, to="currency", **kw) == expected


@pytest.mark.parametrize(
    "lang,code",
    [
        ("sa", "USD"),
        ("yo", "EUR"),
        ("tk", "USD"),
        ("sv", "PLN"),
        ("hu", "SAR"),
        ("te", "GBP"),
        ("kn", "AED"),
        ("tet", "GBP"),
    ],
)
def test_unsourced_currency_raises(lang, code):
    with pytest.raises(NotImplementedError):
        num2words(2, lang=lang, to="currency", currency=code)
