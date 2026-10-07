# -*- coding: utf-8 -*-
"""Turkic ordinals take a harmonised ending on the last word (#148).

kk, tt, uz, tk and ba used to glue a fixed suffix onto the cardinal with a
hyphen ("бір-інші", "bir-chi", "üç-nji", "өс-се"). Expected forms are the
ordinals listed in Wiktionary's number tables
(https://en.wiktionary.org/wiki/Module:number_list/data/<lang>), with the
rules from uz.wikipedia "Son (tilshunoslik)" (uz), enedilim.com "Sanlar"
(tk) and ba.wikipedia usage (ba: миллионынсы, Нуленсе меридиан).
"""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

CASES = {
    "uz": {
        1: "birinchi", 2: "ikkinchi", 3: "uchinchi", 4: "to'rtinchi",
        6: "oltinchi", 7: "yettinchi", 10: "o'ninchi", 20: "yigirmanchi",
        40: "qirqinchi", 50: "elliginchi", 90: "to'qsoninchi",
        123: "bir yuz yigirma uchinchi", 10**6: "bir millioninchi",
    },
    "tk": {
        1: "birinji", 2: "ikinji", 3: "üçünji", 4: "dördünji", 5: "bäşinji",
        6: "altynjy", 7: "ýedinji", 9: "dokuzynjy", 10: "onunjy",
        20: "ýigriminji", 30: "otuzynjy", 40: "kyrkynjy", 50: "ellinji",
        90: "togsanynjy", 100: "bir ýüzünji", 1000: "bir müňünji",
        123: "bir ýüz ýigrimi üçünji", 10**6: "bir millionynjy",
    },
    "tt": {
        0: "нуленче", 1: "беренче", 3: "өченче", 4: "дүртенче",
        6: "алтынчы", 9: "тугызынчы", 10: "унынчы", 20: "егерменче",
        40: "кырыгынчы", 80: "сиксәненче", 90: "туксанынчы",
        100: "бер йөзенче", 1000: "бер меңенче", 10**6: "бер миллионынчы",
    },
    "ba": {
        0: "нуленсе", 1: "беренсе", 3: "өсөнсө", 4: "дүртенсе",
        6: "алтынсы", 9: "туғыҙынсы", 10: "унынсы", 20: "егерменсе",
        40: "ҡырҡынсы", 80: "һикһәненсе", 100: "бер йөҙөнсө",
        123: "бер йөҙ егерме өсөнсө", 10**6: "бер миллионынсы",
    },
    "kk": {
        0: "нөлінші", 1: "бірінші", 6: "алтыншы", 9: "тоғызыншы",
        10: "оныншы", 20: "жиырмасыншы", 40: "қырқыншы", 50: "елуінші",
        90: "тоқсаныншы", 100: "бір жүзінші", 10**9: "бір миллиардыншы",
    },
}


@pytest.mark.parametrize("lang,n,expected", [
    (lang, n, word) for lang, rows in sorted(CASES.items())
    for n, word in sorted(rows.items())
])
def test_turkic_ordinal(lang, n, expected):
    assert num2words(n, lang=lang, to="ordinal") == expected
    assert "-" not in expected
