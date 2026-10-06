# -*- coding: utf-8 -*-
"""Everyday decimals read correctly in hu, mn and fa (gladiaio/num2words2#212).

hu raised KeyError from four decimals on and RecursionError for 1e16, mn an
empty NotImplementedError from seven decimals, fa IndexError from twelve.
"""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words


@pytest.mark.parametrize("value, expected", [
    (0.0001, "nulla egész egy tízezred"),
    (Decimal("0.0001"), "nulla egész egy tízezred"),
    (1e-05, "nulla egész egy százezred"),
    (0.123456, "nulla egész százhuszonháromezer-négyszázötvenhat milliomod"),
    (0.001, "nulla egész egy ezred"),
    (1.5, "egy egész öt tized"),
])
def test_hu_fraction_denominators(value, expected):
    assert num2words(value, lang="hu") == expected


@pytest.mark.parametrize("value", [1e16, 1e17, 1e20, 1e21])
def test_hu_whole_float_reads_like_the_int(value):
    assert num2words(value, lang="hu") == num2words(int(value), lang="hu")
    assert num2words(Decimal(int(value)), lang="hu") == num2words(
        int(value), lang="hu")


def test_mn_seven_and_more_decimals():
    assert num2words(0.1234567, lang="mn") == (
        "тэг, арван саяны нэг сая хоёр зуун гучин дөрвөн мянга таван зуун "
        "жаран долоо")
    assert num2words(Decimal("1.10000000"), lang="mn") == (
        "нэг, зуун саяны арван сая")
    with pytest.raises(NotImplementedError, match="11 places"):
        num2words(Decimal("0.123456789012"), lang="mn")


def test_fa_twelve_and_more_decimals():
    assert num2words(0.1 + 0.2, lang="fa") == "سی تریلیارد و چهار صدم تریلیاردیم"
    assert num2words(Decimal("0.123456789012"), lang="fa").endswith(
        "دوازده تریلیونیم")
    with pytest.raises(NotImplementedError, match="17 places"):
        num2words(Decimal("0.123456789012345678"), lang="fa")
