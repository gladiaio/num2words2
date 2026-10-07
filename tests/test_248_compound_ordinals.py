# -*- coding: utf-8 -*-
"""Compound and large ordinals are real words (gladiaio/num2words2#248)."""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

CASES = {
    "sk": {
        0: "nultý",
        21: "dvadsiaty prvý",
        101: "stý prvý",
        200: "dvojstý",
        1001: "tisíci prvý",
        2021: "dvojtisíci dvadsiaty prvý",
        10**6: "miliónty",
        -1: "mínus prvý",
    },
    "hr": {
        0: "nulti",
        21: "dvadeset prvi",
        101: "sto prvi",
        200: "dvjestoti",
        345: "tristo četrdeset peti",
        1001: "tisuća prvi",
        10**6: "milijunti",
        -1: "minus prvi",
    },
    "sr": {
        0: "нулти",
        21: "двадесет први",
        101: "сто први",
        200: "двестоти",
        1100: "хиљада стоти",
        10**6: "милионити",
        -1: "минус први",
    },
    "sr_Latn": {
        21: "dvadeset prvi",
        42: "četrdeset drugi",
        200: "dvestoti",
        10**6: "milioniti",
        -1: "minus prvi",
    },
    "lt": {
        0: "nulinis",
        21: "dvidešimt pirmas",
        101: "vienas šimtas pirmas",
        1000: "tūkstantasis",
        1001: "vienas tūkstantis pirmas",
        10**6: "milijoninis",
        -1: "minus pirmas",
    },
    "lv": {
        0: "nultais",
        21: "divdesmit pirmais",
        101: "simtu pirmais",
        1001: "tūkstotis pirmais",
        -1: "mīnus pirmais",
    },
    "et": {
        21: "kahekümne esimene",
        40: "neljakümnes",
        101: "saja esimene",
        121: "saja kahekümne esimene",
        200: "kahesajas",
        1001: "tuhande esimene",
        2021: "kahe tuhande kahekümne esimene",
        10**6: "miljones",
    },
    "hu": {
        10**6: "egymilliomodik",
        10**9: "egymilliárdodik",
        10**12: "egybilliomodik",
        2 * 10**6: "kétmilliomodik",
    },
}


@pytest.mark.parametrize("lang", sorted(CASES))
def test_compound_ordinals(lang):
    for n, want in CASES[lang].items():
        assert num2words(n, lang=lang, to="ordinal") == want, (lang, n)
