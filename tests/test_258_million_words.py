"""Regression tests for gladiaio/num2words2#258: suspect million words."""

import pytest

from num2words2 import num2words

CASES = [
    # hmn: "roob" is 10^6 (Python: the unattested "tawm rau").
    ("hmn", 10**6, "ib roob"),
    ("hmn", 2 * 10**6, "ob roob"),
    ("hmn", 10**7, "kaum roob"),
    # lus/miz: "nuai" is 10^5; 10^6 is "maktaduai".
    ("lus", 10**6, "pakhat maktaduai"),
    ("lus", 2 * 10**6, "pahnih maktaduai"),
    ("miz", 10**6, "pakhat maktaduai"),
    ("lus", 10**5, "pakhat za sang"),
    # su, ln, cnh: left as is (see the issue and the module docs).
    ("su", 10**6, "hiji juta"),
    ("ln", 10**6, "moko milio"),
    ("cnh", 10**6, "pakhat milin"),
]


@pytest.mark.parametrize("lang,value,expected", CASES)
def test_million_word(lang, value, expected):
    assert num2words(value, lang=lang) == expected
