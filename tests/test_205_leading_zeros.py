# -*- coding: utf-8 -*-
"""Leading zeros of the fractional part are read (gladiaio/num2words2#205).

These languages worded the digits after the point as one integer (or read a
0 digit as an empty word), so 0.05 came out as the same words as 0.5.
"""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words

EXPECTED = {
    "sl": ("nič vejica nič pet", "tri vejica nič nič sedem"),
    "bn": ("শূন্য দশমিক শূন্য পাঁচ", "তিন দশমিক শূন্য শূন্য সাত"),
    "ha": ("sifiri wajen sifiri biyar", "uku wajen sifiri sifiri bakwai"),
    "uz_Cyrl": ("нол вергул нол беш", "уч вергул нол нол етти"),
    "fa": ("پنج صدم", "سه و هفت هزارم"),
    "et": ("null koma null viis", "kolm koma null null seitse"),
    # vi reads "%.2f" hundredths, so 3.007 is 3.01.
    "vi": ("không phẩy không năm", "ba phẩy không một"),
}


@pytest.mark.parametrize("lang", sorted(EXPECTED))
@pytest.mark.parametrize("kind", [float, Decimal, str])
def test_leading_zeros_are_read(lang, kind):
    small, three = EXPECTED[lang]
    assert num2words(kind("0.05"), lang=lang) == small
    assert num2words(kind("0.05"), lang=lang) != num2words(kind("0.5"), lang=lang)
    # vi rounds floats to two places.
    if kind is not float or lang != "vi":
        assert num2words(kind("3.007"), lang=lang) == three


@pytest.mark.parametrize("lang", sorted(EXPECTED))
def test_no_empty_words(lang):
    for v in (0.05, 1.005, 1.102, 2.05, 100.01):
        r = num2words(v, lang=lang)
        assert r == r.strip() and "  " not in r, (v, r)
