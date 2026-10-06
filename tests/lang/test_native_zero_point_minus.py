# -*- coding: utf-8 -*-
"""Zero, decimal word and minus in the language itself, not English.

Regression tests for gladiaio/num2words2#154: these modules inherited the
English base-class words ("zero", "point", "minus") and the Rust port kept
them. Each row pins num2words(0), num2words(1.5), num2words(-1) and the
ordinal of 0.
"""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

CASES = {
    # lang: (0, 1.5, -1, ordinal 0)
    "mk": ("нула", "еден запирка пет", "минус еден", "нула-ти"),
    "ps": ("صفر", "یو اعشاریه پنځه", "منفي یو", "صفر-م"),
    "sa": ("शून्यम्", "एकम् दशमलव पञ्च", "ऋण एकम्", "शून्यम्-मः"),
    "si": ("බිංදුව", "එක දශම පහ", "සෘණ එක", "බිංදුව වැනි"),
    "tt": ("нуль", "бер өтер биш", "минус бер", "нуль-нче"),
    "yi": ("נול", "איינס פּונקט פינף", "מינוס איינס", "נול-טער"),
    "gl": ("cero", "un coma cinco", "menos un", "cero-o"),
    "nn": ("null", "ein komma fem", "minus ein", "null-de"),
    "fo": ("null", "ein komma fimm", "minus ein", "null-ti"),
    "lb": ("null", "eent Komma fënnef", "minus eent", "null-ten"),
    "oc": ("zèro", "un virgula cinc", "mens un", "zèro-en"),
}


@pytest.mark.parametrize("lang", sorted(CASES))
def test_native_words(lang):
    zero, one_and_half, minus_one, ordinal_zero = CASES[lang]
    assert num2words(0, lang=lang) == zero
    assert num2words(1.5, lang=lang) == one_and_half
    assert num2words(-1, lang=lang) == minus_one
    assert num2words(0, lang=lang, to="ordinal") == ordinal_zero


def test_sd_zero_and_minus():
    # The Sindhi decimal word is still the English "point" (needs a native
    # speaker), but zero and minus are Sindhi.
    assert num2words(0, lang="sd") == "ٻڙي"
    assert num2words(-1, lang="sd") == "منفي هڪ"


def test_sr_cyrillic_currency_names():
    assert (num2words(1.5, lang="sr", to="currency")
            == "један динар, педесет пара")
    assert (num2words(2.01, lang="sr", to="currency", currency="EUR")
            == "два евра, један цент")


def test_sr_latn_currency_names_stay_latin():
    assert (num2words(1.5, lang="sr_Latn", to="currency")
            == "jedan dinar, pedeset para")
