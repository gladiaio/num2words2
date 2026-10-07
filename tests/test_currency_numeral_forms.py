# -*- coding: utf-8 -*-
"""gladiaio/num2words2#253: a numeral before a currency noun takes its
attributive form (de 'ein Euro', es 'veintiún euros', it 'un euro', hu 'két
forint', cs 'jedno euro', sl 'en evro'), on the int, float and str paths."""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words

CASES = [
    ("de", 1, "EUR", "ein Euro"),
    ("de", 101, "USD", "einhundertein Dollar"),
    ("de", 1.01, "EUR", "ein Euro und ein Cent"),
    ("de", "1.01", "EUR", "ein Euro und ein Cent"),
    ("de", 1.0, "INR", "eine Rupie und null Paisa"),
    ("de", 21, "EUR", "einundzwanzig Euro"),
    ("es", 21, "EUR", "veintiún euros"),
    ("es", 31, "USD", "treinta y un dólares"),
    ("es", 21.21, "USD", "veintiún dólares con veintiún centavos"),
    ("es", 1, "GBP", "una libra"),
    ("es", 21, "GBP", "veintiuna libras"),
    ("es", 1.01, "GBP", "una libra con un penique"),
    ("es", 101, "USD", "ciento un dólares"),
    ("it", 1, "EUR", "un euro"),
    ("it", 1.01, "EUR", "un euro e un centesimo"),
    ("it", 1, "GBP", "una sterlina"),
    ("it", 1, "JPY", "uno yen"),
    ("it", 21, "EUR", "ventuno euro"),
    ("hu", 2, "HUF", "két forint"),
    ("hu", 2.5, "HUF", "két forint, ötven fillér"),
    ("hu", "2.02", "HUF", "két forint, két fillér"),
    ("hu", 12, "HUF", "tizenkét forint"),
    ("cs", 1, "EUR", "jedno euro"),
    ("cs", 2, "EUR", "dvě eura"),
    ("cs", 5, "EUR", "pět eur"),
    ("cs", 0.01, "EUR", "nula eur, jeden cent"),
    ("cs", 1, "USD", "jeden dolar"),
    ("cs", 100, "USD", "sto dolarů"),
    ("cs", 2, "CZK", "dvě koruny"),
    ("cs", 1.01, "CZK", "jedna koruna, jeden haléř"),
    ("sl", 1, "EUR", "en evro"),
    ("sl", 2, "EUR", "dva evra"),
    ("sl", 5, "EUR", "pet evrov"),
    ("sl", 2.02, "USD", "dva dolarja dva centa"),
    ("sl", Decimal("101"), "EUR", "sto en evro nič centov"),
]


@pytest.mark.parametrize("lang,value,code,expected", CASES)
def test_attributive_numeral_before_currency_noun(lang, value, code, expected):
    assert num2words(value, lang=lang, to="currency", currency=code) == expected
