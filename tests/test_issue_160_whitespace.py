# -*- coding: utf-8 -*-
"""Regression tests for gladiaio/num2words2#160: stray whitespace and empty
strings. Output must be non-empty, stripped and free of double spaces."""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import num2words


def _clean(r):
    return bool(r) and r == r.strip() and "  " not in r


# 1. Negative *int* currency used the raw negword ("menos ") and doubled the
#    space. Int, float and str input now agree.
@pytest.mark.parametrize(
    "lang,expected",
    [
        ("es", "menos cinco euros"),
        ("es_CO", "menos cinco pesos"),
        ("es_CR", "menos cinco colónes"),
        ("es_GT", "menos cinco quetzales"),
        ("es_VE", "menos cinco bolívares"),
        ("ca", "menys cinc euros"),
        ("da", "minus fem kroner"),
        ("fi", "miinus viisi euroa"),
        ("hu", "mínusz öt forint"),
        ("nl", "min vijf euro"),
        ("sv", "minus fem euro"),
        ("ha", "ban naira biyar"),
        ("dv", "މައިނަސް ފަސް ރުފިޔާ"),
        ("hi", "माइनस पाँच रुपये"),
        ("is", "mínus fimm krónur"),
    ],
)
def test_negative_int_currency_single_space(lang, expected):
    assert num2words(-5, lang=lang, to="currency") == expected


def test_int_and_str_currency_agree():
    # (is is left out: its str/float path leaks a forms tuple, a separate bug.)
    for lang in ("es", "da", "nl", "sv", "fi", "hu", "hi", "ca"):
        assert num2words(-5, lang=lang, to="currency") == num2words(
            "-5", lang=lang, to="currency"
        )


# 2. Other stray whitespace.
def test_dv_negative_cardinal():
    assert num2words(-1, lang="dv") == "މައިނަސް އެކެއް"
    assert _clean(num2words(-42, lang="dv"))


def test_te_pointword_and_fifty():
    assert num2words(0.5, lang="te") == "సున్న బిందువు అయిదు"
    assert _clean(num2words(1.5, lang="te"))
    assert _clean(num2words(50, lang="te"))
    assert _clean(num2words(1.5, lang="te", to="currency"))


def test_uz_cyrl_ordinal_keeps_interior_bir():
    # Matches its own cardinal "бир минг бир юз" and uz Latin.
    assert num2words(1100, lang="uz_Cyrl") == "бир минг бир юз"
    assert num2words(1100, lang="uz_Cyrl", to="ordinal") == "бир минг бир юзинчи"
    # Python blanked the interior "бир": "йигирма  мингинчи" (= 20000th).
    assert num2words(21000, lang="uz_Cyrl", to="ordinal") == "йигирма бир мингинчи"
    # A leading "бир" is still dropped, as before.
    assert num2words(100, lang="uz_Cyrl", to="ordinal") == "юзинчи"
    assert num2words(1000, lang="uz_Cyrl", to="ordinal") == "мингинчи"


def test_fa_currency_spacing():
    assert num2words(0.5, lang="fa", to="currency") == "صفر تومان و پنجاه"
    assert (
        num2words(1.5, lang="fa", to="currency", currency="EUR")
        == "یک یورو و پنجاه سنت"
    )


def test_ar_currency_zero_integer_part():
    assert num2words(0.5, lang="ar", to="currency") == "صفر ريال وخمسون هللة"
    assert num2words(1.5, lang="ar", to="currency") == "واحد ريال وخمسون هللة"
    assert num2words(-0.01, lang="ar", to="currency") == "سالب صفر ريال وإحدى هللة"


# 3. Empty strings.
@pytest.mark.parametrize(
    "lang",
    [
        "es",
        "es_CO",
        "es_CR",
        "es_GT",
        "es_HN",
        "es_NI",
        "es_VE",
        "ca",
        "pt",
        "pt_BR",
    ],
)
def test_ordinal_zero_raises(lang):
    with pytest.raises(ValueError):
        num2words(0, lang=lang, to="ordinal")
    # Ordinals of other values are unaffected.
    assert num2words(1, lang=lang, to="ordinal")


def test_es_ordinal_zero_with_gender_raises():
    with pytest.raises(ValueError):
        num2words(0, lang="es", to="ordinal", gender="f")
    assert num2words(0, lang="es", to="ordinal_num") == "0º"


@pytest.mark.parametrize("value", [0.5, -0.5, Decimal("0.1")])
def test_pt_fraction_truncating_to_zero_raises(value):
    for lang in ("pt", "pt_BR"):
        # A TypeError since #214, like every fractional ordinal.
        with pytest.raises(TypeError):
            num2words(value, lang=lang, to="ordinal")


def test_hy_currency_zero():
    assert num2words(0, lang="hy", to="currency") == "զրո դրամ"


def test_sq_currency_zero_and_decimal():
    assert num2words(0, lang="sq", to="currency") == "zero lekë"
    assert (
        num2words(Decimal("0.1"), lang="sq", to="currency")
        == num2words(0.1, lang="sq", to="currency")
        == "dhjetë qindarkë"
    )
    assert num2words("1.5", lang="sq", to="currency") == num2words(
        1.5, lang="sq", to="currency"
    )


@pytest.mark.parametrize("value", [0.5, 1.5, "1.5", Decimal("0.1")])
def test_tr_fractional_ordinal_raises(value):
    with pytest.raises(TypeError):
        num2words(value, lang="tr", to="ordinal")
    assert num2words(0, lang="tr", to="ordinal") == "sıfırıncı"
