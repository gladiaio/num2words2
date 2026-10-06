# -*- coding: utf-8 -*-
"""Input normalisation in the shared dispatcher, checked for every language.

Values are normalised once, before any language converter sees them:
exponent notation (gladiaio/num2words2#211, #199, #210), integral values of
any input type in the integer modes (#213), non-integral ordinals and years
(#214), numeric types (#236) and input edge cases (#237).
"""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import _rust, num2words

LANGS = sorted(_rust.supported_langs())


def _call(x, **kw):
    try:
        return num2words(x, **kw)
    except Exception as e:  # noqa: BLE001 - comparing outcomes is the point
        return type(e)


# ---- #211 / #199 / #210: exponent notation --------------------------------

@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize("to", ["cardinal", "ordinal", "ordinal_num", "year"])
def test_integer_in_exponent_form_reads_as_the_integer(lang, to):
    want = _call(1000, lang=lang, to=to)
    for x in (Decimal("1E+3"), "1e3", "1E3", Decimal("1.0E+3")):
        assert _call(x, lang=lang, to=to) == want, (x, lang, to)
    assert _call(1e21, lang=lang, to=to) == _call(10**21, lang=lang, to=to)


@pytest.mark.parametrize("lang", LANGS)
def test_small_float_reads_like_its_decimal(lang):
    # repr(1e-05) == '1e-05': read like Decimal('0.00001'), not its mantissa.
    assert (_call(1e-05, lang=lang) == _call(Decimal("0.00001"), lang=lang)
            == _call("1e-5", lang=lang))


@pytest.mark.parametrize("lang", ["pl", "uk", "en", "en_AERO", "fr"])
def test_currency_in_exponent_form(lang):
    assert (num2words(Decimal("1E+3"), lang=lang, to="currency")
            == num2words(Decimal("1000"), lang=lang, to="currency"))


def test_issue_examples():
    assert num2words(Decimal("1E+3"), lang="pl") == "tysiąc"
    assert num2words("1e3", lang="gl") == "un mil"
    assert num2words(1e21, lang="uk") == "один секстильйон"
    # #199: these raised IndexError
    assert num2words(1e21, lang="ce") == num2words(10**21, lang="ce")
    assert num2words(1e21, lang="cy") == num2words(10**21, lang="cy")
    assert num2words(1e21, lang="rm") == num2words(10**21, lang="rm")
    assert num2words(1.5e20, lang="ce") == num2words(15 * 10**19, lang="ce")


@pytest.mark.parametrize("lang", [
    "en_AERO", "en_Aero_ICAO", "en_aero_icao", "en_x_aero_icao", "en_Aero_FAA",
    "en_Aero_NATO", "en_Aero_USN", "en_Aero_US_Navy", "en_Aero_US_Army",
])
def test_en_aero_exponent_and_non_finite(lang):
    # #210: '1e-05' was read digit by digit ("wun zero fife").
    assert (num2words(1e-05, lang=lang)
            == num2words(Decimal("0.00001"), lang=lang))
    assert num2words(1e21, lang=lang) == num2words(10**21, lang=lang)
    # Non-finite strings raise as they do in en, instead of '' / 'minus'.
    for s in ("NaN", "inf", "-inf"):
        assert _call(s, lang=lang) == _call(s, lang="en")
        assert _call(s, lang=lang) in (ValueError, OverflowError)


# ---- #213: integral values of any type in the integer modes ---------------

@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize("to", ["ordinal", "ordinal_num", "year"])
def test_integral_value_of_any_type_reads_as_the_integer(lang, to):
    want = _call(1999, lang=lang, to=to)
    for x in ("1999", Decimal("1999"), 1999.0, Decimal("1999.0"), "1999.0",
              Decimal("1.999E+3")):
        assert _call(x, lang=lang, to=to) == want, (x, lang, to)


def test_issue_213_examples():
    year = num2words(1999, lang="et", to="year")
    assert num2words("1999", lang="et", to="year") == year
    assert (num2words(1999.0, lang="ms", to="year")
            == num2words(1999, lang="ms", to="year"))
    assert (num2words(Decimal("1999"), lang="ta", to="year")
            == num2words(1999, lang="ta", to="year"))
    assert num2words("5", lang="bn", to="ordinal_num") == "পঞ্চম"
    assert (num2words(1999.0, lang="uk", to="ordinal")
            == num2words(1999, lang="uk", to="ordinal"))
    assert (num2words(Decimal("1999.0"), lang="hi", to="ordinal_num")
            == num2words(1999, lang="hi", to="ordinal_num"))
    # A whole Decimal no longer keeps its scale: "5th", not "5.00th".
    assert num2words(Decimal("5.00"), to="ordinal_num") == "5th"
