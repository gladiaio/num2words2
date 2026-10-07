# -*- coding: utf-8 -*-
"""precision= reads the value as written, for every input type (#218)."""
from decimal import Decimal

import pytest

from num2words2 import _rust, num2words


def test_no_float_noise_past_17_digits():
    assert num2words(0.5, precision=25) == "zero point five" + " zero" * 24
    out = num2words(1.2345, precision=50)
    assert out == "one point two three four five" + " zero" * 46


def test_no_i128_cap():
    out = num2words(Decimal("1.2345"), precision=40)
    assert out.endswith(" zero" * 36)


@pytest.mark.parametrize("value", [0.1, Decimal("0.1"), "0.1"])
def test_applies_to_every_input_type(value):
    assert num2words(value, precision=3) == "zero point one zero zero"


def test_truncates_toward_zero():
    assert num2words(1.2345, precision=2) == "one point two three"
    assert num2words("1.2345", precision=2) == "one point two three"


def test_languages_that_ignored_it():
    assert num2words(0.1, lang="it", precision=3) == "zero virgola uno zero zero"
    assert num2words(1.2345, lang="ru", precision=2) == "одна целая двадцать три сотых"
    assert num2words(0.1, lang="tr", precision=2) == "sıfırvirgülon"


def test_unhonourable_precision_raises():
    with pytest.raises(
        NotImplementedError, match="lang='ja' does not " "support precision="
    ):
        num2words(0.1, lang="ja", precision=3)


def test_ms_ta_honour_precision_since_they_read_the_fraction():
    # ms/ta used to drop the fraction and so raised here; since #206 they
    # read it digit by digit, which precision= cuts and pads like en.
    assert num2words(1.25, lang="ms", precision=1) == "satu perpuluhan dua"
    assert (
        num2words(0.1, lang="ta", precision=3)
        == "பூஜ்ஜியம் புள்ளி ஒன்று பூஜ்ஜியம் பூஜ்ஜியம்"
    )


@pytest.mark.parametrize("lang", sorted(_rust.supported_langs()))
def test_never_leaks_internal_errors(lang):
    for value in (0.1, 1.2345, "2.5", Decimal("-0.25")):
        try:
            num2words(value, lang=lang, precision=3)
        except NotImplementedError:
            pass


@pytest.mark.parametrize("value", [1.5, "1.5", Decimal("1.5")])
def test_negative_precision_is_value_error(value):
    with pytest.raises(ValueError, match="precision="):
        num2words(value, precision=-1)
