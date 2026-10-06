# -*- coding: utf-8 -*-
"""Currency options behave the same for every input type and language (#220)."""
from decimal import Decimal

import pytest

from num2words2 import _rust, num2words

LANGS = sorted(_rust.supported_langs())


# ko's default KRW has no subunit: 2.5 is a ValueError with any cents=.
@pytest.mark.parametrize("lang", [lg for lg in LANGS if lg != "ko"])
def test_cents_false_keeps_the_cents_as_digits(lang):
    try:
        out = num2words(2.5, lang=lang, to="currency", cents=False)
    except NotImplementedError:
        return
    assert "50" in out


def test_cents_false_examples():
    assert num2words(2.5, lang="en", to="currency", cents=False) == \
        "two euros, 50 cents"
    assert num2words(2.5, lang="gl", to="currency", cents=False) == \
        "dous euros 50 céntimos"
    assert num2words(2.5, lang="ja", to="currency", cents=False) == "二円50銭"
    assert num2words(2.5, lang="dv", to="currency", cents=False) == \
        "ދެ ރުފިޔާ 50 ލާރި"


@pytest.mark.parametrize("value", [1.99, Decimal("1.99"), "1.99"])
def test_cents_omit_truncates_every_input_type(value):
    assert num2words(value, to="currency", cents="omit") == "one euro"


@pytest.mark.parametrize("value", [-1.99, Decimal("-1.99"), "-1.99"])
def test_cents_omit_truncates_toward_zero(value):
    assert num2words(value, to="currency", cents="omit") == "minus one euro"


@pytest.mark.parametrize("value", [101.5, "101.5", Decimal("101.5")])
def test_style_us_on_currency(value):
    assert num2words(value, to="currency", style="us") == \
        "one hundred one euros, fifty cents"


def test_style_us_on_int_currency_and_cheque():
    assert num2words(101, to="currency", style="us") == "one hundred one euros"
    assert num2words(101.5, to="cheque", style="us") == \
        "ONE HUNDRED ONE AND 50/100 EUROS"


@pytest.mark.parametrize("value", [2, 2.5, "2.5"])
@pytest.mark.parametrize("cents", ["maybe", None, 3])
def test_invalid_cents_lists_the_valid_values(value, cents):
    with pytest.raises(ValueError, match="'verbose', 'terse' or 'omit'"):
        num2words(value, to="currency", cents=cents)
