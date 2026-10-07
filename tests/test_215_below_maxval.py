# -*- coding: utf-8 -*-
"""Correct words all the way up to maxval() (gladiaio/num2words2#215).

hy printed a Python tuple from 10**13, vi switched to English scale words
above 10**18, and currency raised decimal.InvalidOperation from 10**26 in
hy, bn, tet and vi, and AssertionError at maxval('ar') - 1 in ar.
"""
from __future__ import unicode_literals

import pytest

from num2words2 import maxval, num2words


@pytest.mark.parametrize(
    "value, expected",
    [
        (10**12, "մեկ տրիլիոն"),
        (10**13, "տասը տրիլիոն"),
        (
            1234567890,
            "մեկ միլիարդ երկու հարյուր երեսունչորս միլիոն "
            "հինգ հարյուր վաթսունյոթ հազար ութ հարյուր իննսուն",
        ),
        (5000000001, "հինգ միլիարդ մեկ"),
        (2500000000, "երկու միլիարդ հինգ հարյուր միլիոն"),
        (-(10**13), "մինուս տասը տրիլիոն"),
    ],
)
def test_hy_scale_words(value, expected):
    assert num2words(value, lang="hy") == expected


@pytest.mark.parametrize(
    "value, expected",
    [
        (10**15, "một triệu tỷ"),
        (10**17 + 1, "một trăm triệu tỷ lẻ một"),
        # Exact integer arithmetic: 2**53 + 1 used to read as 2**53.
        (
            2**53 + 1,
            "chín triệu tỷ bảy nghìn tỷ một trăm chín mươi chín tỷ "
            "hai trăm năm mươi bốn triệu bảy trăm bốn mươi nghìn "
            "chín trăm chín mươi ba",
        ),
    ],
)
def test_vi_vietnamese_scale_words(value, expected):
    assert num2words(value, lang="vi") == expected


@pytest.mark.parametrize("lang", ["hy", "vi", "tet", "ar"])
@pytest.mark.parametrize("to", ["cardinal", "ordinal", "currency"])
def test_powers_of_ten_below_maxval(lang, to):
    m = maxval(lang)
    e = 1
    while 10**e < m:
        r = num2words(10**e, lang=lang, to=to)
        assert isinstance(r, str) and r, (e, r)
        assert "(" not in r and "illion" not in r, (e, r)
        e += 1
    for v in (m - 1, -(m - 1)):
        if to != "ordinal" or v > 0:
            assert isinstance(num2words(v, lang=lang, to=to), str)
    for v in (m, -m):
        if to == "ordinal" and v < 0:
            continue
        with pytest.raises(OverflowError):
            num2words(v, lang=lang, to=to)


def test_ar_ordinal_ceiling():
    m = maxval("ar")
    # Every round scale word below maxval has an ordinal: الألف, المليون, ...
    for e in range(3, len(str(m)) - 1, 3):
        r = num2words(10**e, lang="ar", to="ordinal")
        assert r.startswith("ال") and " " not in r, (e, r)
    # Every value below maxval has one since #261; maxval itself raises.
    assert num2words(m - 1, lang="ar", to="ordinal").endswith("والتاسع والتسعون")
    with pytest.raises(OverflowError):
        num2words(m, lang="ar", to="ordinal")


@pytest.mark.parametrize("lang", ["bn", "tet"])
def test_currency_past_the_decimal_context(lang):
    # 10**26 overran Python's 28-digit decimal context in to_currency only.
    assert num2words(10**26, lang=lang) in num2words(
        10**26, lang=lang, to="currency"
    )


def test_ar_reads_29_to_31_digit_values_exactly():
    # The old 28-digit Decimal rounding read this as "nonillion and 999".
    r = num2words(10**30 - 1, lang="ar")
    assert r.startswith("تسعمائة وتسعة وتسعون أوكتيليوناً")
    assert r.endswith("ألفاً وتسعمائة وتسعة وتسعون")
