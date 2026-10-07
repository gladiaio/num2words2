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
    "tt": ("нуль", "бер өтер биш", "минус бер", "нуленче"),
    "yi": ("נול", "איינס פּונקט פינף", "מינוס איינס", "נול-טער"),
    "gl": ("cero", "un coma cinco", "menos un", "cero-o"),
    "nn": ("null", "ein komma fem", "minus ein", "null"),
    "fo": ("null", "ein komma fimm", "minus ein", "null-ti"),
    "lb": ("null", "eent Komma fënnef", "minus eent", "null-ten"),
    "oc": ("zèro", "un virgula cinc", "mens un", "zèro-en"),
    "br": ("mann", "unan skej pemp", "lei unan", "mann-vet"),
    "haw": ("'ole", "'ekahi kiko 'elima", "'i'o 'ole 'ekahi", "ka 'ole"),
    "ht": ("zewo", "en vigil senk", "mwens en", "zewo-yèm"),
    "jv": ("nol", "siji koma lima", "minus siji", "nol-e"),
    "jw": ("nol", "siji koma lima", "minus siji", "nol-e"),
    "mg": ("aotra", "iray faingo dimy", "miiba iray", "faha-aotra"),
    "mi": ("kore", "tahi ira rima", "tōraro tahi", "tua kore"),
    "mt": ("żero", "wieħed punt ħamsa", "minus wieħed", "l-żero"),
    "so": ("eber", "kow dhibic shan", "taban kow", "eber-aad"),
    "tl": ("sero", "isa punto lima", "minus isa", "ika-sero"),
    "uz": ("nol", "bir vergul besh", "minus bir", "nolinchi"),
}


UNVERIFIED = {
    # Best-candidate words, flagged UNVERIFIED in the module headers (#154).
    "su": ("nol", "hiji koma lima", "minus hiji", "nol-na"),
    "yo": ("òdo", "ọkan ẹsẹ marun", "òdì ọkan", "òdo-kẹta"),
    "tk": ("nol", "bir otur bäş", "minus bir", "nolunjy"),
    "cnh": ("pakpalawng", "pakhat deh panga", "zuh pakhat", "pakpalawng-nak"),
    "ln": ("libúngútulú", "moko virgule mítáno", "moins moko", "libúngútulú-e"),
    "wo": ("tus", "benn virgule juróom", "moins benn", "tus-eel"),
}


@pytest.mark.parametrize("lang", sorted(CASES))
def test_native_words(lang):
    zero, one_and_half, minus_one, ordinal_zero = CASES[lang]
    assert num2words(0, lang=lang) == zero
    assert num2words(1.5, lang=lang) == one_and_half
    assert num2words(-1, lang=lang) == minus_one
    assert num2words(0, lang=lang, to="ordinal") == ordinal_zero


@pytest.mark.parametrize("lang", sorted(UNVERIFIED))
def test_best_candidate_words(lang):
    zero, one_and_half, minus_one, ordinal_zero = UNVERIFIED[lang]
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
    assert num2words(1.5, lang="sr", to="currency") == "један динар, педесет пара"
    assert (
        num2words(2.01, lang="sr", to="currency", currency="EUR")
        == "два евра, један цент"
    )


def test_sr_latn_currency_names_stay_latin():
    assert num2words(1.5, lang="sr_Latn", to="currency") == "jedan dinar, pedeset para"
