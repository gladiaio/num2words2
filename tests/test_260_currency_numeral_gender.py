"""Regression tests for gladiaio/num2words2#260: the numeral before a
currency noun agrees with the noun's gender (fo, lb, sv)."""

import pytest

from num2words2 import num2words

CASES = [
    # fo: evra/króna are feminine, dollari masculine, oyra neuter.
    ("fo", 2, "EUR", "tvær evrur"),
    ("fo", 3, "EUR", "tríggjar evrur"),
    ("fo", 22, "EUR", "tjúgu tvær evrur"),
    ("fo", 2, "DKK", "tvær krónur"),
    ("fo", 2, "USD", "tveir dollarar"),
    ("fo", 3, "USD", "tríggir dollarar"),
    ("fo", 1.01, "DKK", "ein króna eitt oyra"),
    ("fo", 4, "EUR", "fýra evrur"),
    # lb: Euro, Dollar and Cent are masculine: een / zwee.
    ("lb", 1, "EUR", "een Euro"),
    ("lb", 2, "EUR", "zwee Euro"),
    ("lb", 2.02, "USD", "zwee Dollar zwee Cent"),
    # sv: en before common-gender nouns, ett before pund/öre; "en" after
    # tens except with öre.
    ("sv", 1, "USD", "en dollar"),
    ("sv", 1, "EUR", "en euro"),
    ("sv", 1, "SEK", "en krona"),
    ("sv", 1, "GBP", "ett pund"),
    ("sv", 1.01, "USD", "en dollar, en cent"),
    ("sv", 1.01, "SEK", "en krona, ett öre"),
    ("sv", 21, "SEK", "tjugoen kronor"),
    ("sv", 21, "GBP", "tjugoen pund"),
    ("sv", 21.21, "SEK", "tjugoen kronor, tjugoett öre"),
]


@pytest.mark.parametrize("lang,value,currency,expected", CASES)
def test_currency_numeral_gender(lang, value, currency, expected):
    assert num2words(value, lang=lang, to="currency", currency=currency) == expected


@pytest.mark.parametrize(
    "lang,currency,expected",
    [
        ("fo", "EUR", "TVÆR AND 00/100 EVRUR"),
        ("lb", "EUR", "ZWEE AND 00/100 EURO"),
        ("sv", "USD", "TVÅ AND 00/100 DOLLAR"),
    ],
)
def test_cheque_agrees(lang, currency, expected):
    assert num2words(2, lang=lang, to="cheque", currency=currency) == expected
