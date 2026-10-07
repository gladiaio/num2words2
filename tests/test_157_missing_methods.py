# -*- coding: utf-8 -*-
"""gladiaio/num2words2#157: modes and input types that raised AttributeError
from the old Python class now convert or raise a typed, explained error."""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words

RM = ["rm", "rm_puter", "rm_surmiran", "rm_sursilv", "rm_sutsilv", "rm_vallader"]


@pytest.mark.parametrize("lang", RM)
def test_rm_strings_and_decimals_read_like_numbers(lang):
    assert num2words("12", lang=lang) == num2words(12, lang=lang)
    assert num2words("-3", lang=lang) == num2words(-3, lang=lang)
    assert num2words("1.5", lang=lang) == num2words(1.5, lang=lang)
    assert num2words(Decimal("1.5"), lang=lang) == num2words(1.5, lang=lang)
    assert num2words(Decimal("0.1"), lang=lang) == num2words(0.1, lang=lang)
    assert num2words("12", lang=lang, to="ordinal") == num2words(
        12, lang=lang, to="ordinal"
    )


@pytest.mark.parametrize("lang", RM)
def test_rm_year_is_the_cardinal(lang):
    assert num2words(1999, lang=lang, to="year") == num2words(1999, lang=lang)
    assert num2words(1999.0, lang=lang, to="year") == num2words(1999, lang=lang)
    with pytest.raises(TypeError):
        num2words(1.5, lang=lang, to="year")


@pytest.mark.parametrize("lang", RM)
@pytest.mark.parametrize("to", ["ordinal_num", "currency"])
def test_rm_unsupported_modes_say_so(lang, to):
    with pytest.raises(
        NotImplementedError, match="lang='%s' does not support " "to='%s'" % (lang, to)
    ):
        num2words(1, lang=lang, to=to)


def test_rm_decimal_example_from_issue():
    assert num2words(Decimal("0.1"), lang="rm") == "nulla comma in"


def test_vi_string_input():
    assert num2words("12", lang="vi") == num2words(12, lang="vi") == "mười hai"
    assert num2words("1.5", lang="vi") == num2words(1.5, lang="vi")


@pytest.mark.parametrize(
    "lang, minus", [("fa", "منفی"), ("lij", "meno"), ("cy", "meinws")]
)
def test_negative_currency_and_fraction_use_the_minus_word(lang, minus):
    for x in (-0.5, -12.34):
        out = num2words(x, lang=lang, to="currency")
        assert out.startswith(minus + " ")
        assert out[len(minus) + 1 :] == num2words(-x, lang=lang, to="currency")
    # None of the three has fraction rules: they raise instead of the old
    # "ordinal + s" fallback (#217).
    with pytest.raises(
        NotImplementedError, match="lang='%s' does not support to='fraction'" % lang
    ):
        num2words("-3/4", lang=lang)


def test_fa_lij_negative_int_currency():
    assert num2words(-1, lang="fa", to="currency") == "منفی یک تومان"
    assert num2words(-42, lang="lij", to="currency").startswith("meno ")


@pytest.mark.parametrize(
    "x, expected", [(1, "1"), (-1, "-1"), ("12", "12"), (2.0, "2")]
)
def test_sr_latn_ordinal_num_matches_sr(x, expected):
    assert num2words(x, lang="sr_Latn", to="ordinal_num") == expected
    assert num2words(x, lang="sr", to="ordinal_num") == expected
