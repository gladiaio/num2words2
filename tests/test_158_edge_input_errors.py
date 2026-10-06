# -*- coding: utf-8 -*-
"""gladiaio/num2words2#158: edge inputs (fractional / negative ordinals,
fractional years) raise a typed error with a message, never KeyError,
IndexError or an empty AssertionError."""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words


@pytest.mark.parametrize("lang, to, x", [
    ("az", "ordinal", 1.5), ("az", "ordinal_num", 1.5), ("az", "year", 1.5),
    ("az", "ordinal", "1.5"),
    ("hy", "ordinal", -1), ("hy", "ordinal", 0.5), ("hy", "ordinal", "1.5"),
    ("cy", "ordinal", -1), ("cy", "ordinal", 0.5), ("cy", "ordinal", -42),
    ("sn", "ordinal", 0.5), ("sn", "ordinal", Decimal("1.5")),
    ("ce", "ordinal", "1.50"), ("ce", "ordinal", Decimal("0.1")),
    ("ce", "ordinal", 1.5),
    ("bg", "year", 1.5), ("bg", "year", "1.5"), ("sn", "year", 1.5),
    ("sn", "year", Decimal("0.1")),
])
def test_typed_error(lang, to, x):
    with pytest.raises(TypeError) as exc:
        num2words(x, lang=lang, to=to)
    assert str(exc.value)


def test_whole_values_still_convert():
    assert num2words(2.0, lang="az", to="ordinal") == num2words(
        Decimal("2.0"), lang="az", to="ordinal")
    assert num2words(5.0, lang="cy", to="ordinal") == "pumed"
    assert num2words(5, lang="hy", to="ordinal") == "հինգերորդ"
    assert num2words(1999.0, lang="bg", to="year") == num2words(
        1999, lang="bg", to="year")


@pytest.mark.parametrize("x", [0.5, -0.5, 0.25, "0.5", Decimal("0.1")])
def test_dv_below_one_converts_like_above_one(x):
    # Python raised IndexError for every value below 1 (1.5 worked).
    out = num2words(x, lang="dv")
    assert "ޕޮއިންޓް" in out
    assert num2words(0.5, lang="dv") == "ސުމެއް ޕޮއިންޓް ފަހެއް"
