# -*- coding: utf-8 -*-
"""Numbers from 10^9 up are spelled or raise, never returned as digits (#147).

These modules fell off the end of their scale table at a million and
returned str(number). Each now has the attested scale words (sources in the
module docs and the commit) and raises OverflowError at 1000 times the
largest one; cnh and wo have no attested word above a million and raise
from 10^9.
"""
from __future__ import unicode_literals

import pytest

from num2words2 import maxval, num2words

BILLION = {
    "br": "unan miliard",
    "haw": "'ekahi piliona",
    "ht": "en milya",
    "jv": "siji milyar",
    "jw": "siji milyar",
    "ky": "бир миллиард",
    "ln": "moko miliale",
    "mg": "iray lavitrisa",
    "mi": "tahi piriona",
    "mt": "wieħed biljun",
    "so": "kow bilyan",
    "su": "hiji miliar",
    "tk": "bir milliard",
    "tl": "isa bilyon",
    "tt": "бер миллиард",
    "uz": "bir milliard",
    "yo": "ọkan biliọnu",
}

TRILLION = {
    "br": "unan bilion",
    "haw": "'ekahi kiliona",
    "jv": "siji triliun",
    "ky": "бир триллион",
    "mt": "wieħed triljun",
    "su": "hiji triliun",
    "tk": "bir trillion",
    "tl": "isa trilyon",
    "tt": "бер триллион",
    "uz": "bir trillion",
}

LANGS = sorted(set(BILLION) | {"cnh", "wo"})


@pytest.mark.parametrize("lang", sorted(BILLION))
def test_billion(lang):
    assert num2words(10**9, lang=lang) == BILLION[lang]


@pytest.mark.parametrize("lang", sorted(TRILLION))
def test_trillion(lang):
    assert num2words(10**12, lang=lang) == TRILLION[lang]


@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize("to", ["cardinal", "ordinal", "year", "currency"])
def test_no_digits_below_maxval_and_overflow_at_it(lang, to):
    m = maxval(lang)
    assert m in (10**9, 10**12, 10**15)
    r = num2words(m - 1, lang=lang, to=to)
    assert not any(c.isdigit() for c in r), r
    with pytest.raises(OverflowError):
        num2words(m, lang=lang, to=to)


def test_turkic_ordinals_of_new_scale_words():
    assert num2words(10**9, lang="uz", to="ordinal") == "bir milliardinchi"
    assert num2words(10**12, lang="tk", to="ordinal") == "bir trillionynjy"
    assert num2words(10**9, lang="tt", to="ordinal") == "бер миллиардынчы"
    assert num2words(10**9, lang="ky", to="ordinal") == "бир миллиардынчы"
