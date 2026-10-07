# -*- coding: utf-8 -*-
"""to='cheque': no low-level crashes, and string/Decimal input works (#223)."""
from decimal import Decimal

import pytest

from num2words2 import _rust, num2words

UNSUPPORTED = [
    "am",
    "sl",
    "zh",
    "zh_CN",
    "zh_HK",
    "cn",
    "ko",
    "ce",
    "bn",
    "dv",
    "id",
    "vi",
]


@pytest.mark.parametrize("lang", UNSUPPORTED)
def test_unsupported_languages_say_so(lang):
    for value in (12.5, "12.5", Decimal("12.5"), 12):
        with pytest.raises(
            NotImplementedError, match="lang='%s' does not support to='cheque'" % lang
        ):
            num2words(value, lang=lang, to="cheque")


@pytest.mark.parametrize("lang", sorted(_rust.supported_langs()))
def test_no_low_level_crash(lang):
    try:
        num2words(12.5, lang=lang, to="cheque")
    except NotImplementedError:
        pass


@pytest.mark.parametrize(
    "value", [12.5, "12.5", Decimal("12.5"), Decimal("12.50"), " 12.5 "]
)
def test_string_and_decimal_input(value):
    assert num2words(value, to="cheque") == "TWELVE AND 50/100 EUROS"


def test_string_whole_and_currency():
    assert num2words("12", to="cheque", currency="USD") == "TWELVE AND 00/100 DOLLARS"
