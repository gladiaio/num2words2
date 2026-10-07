# -*- coding: utf-8 -*-
"""South-Asian number words below a hundred (gladiaio/num2words2#247) and
above a crore (gladiaio/num2words2#147)."""
from __future__ import unicode_literals

import re

import pytest

from num2words2 import maxval, num2words

# Every number below a hundred has its own word (or, for ml/kn/si, one
# fused compound). Python joined "twenty" and "three".
BELOW_HUNDRED = [
    ("mr", 22, "बावीस"),
    ("mr", 23, "तेवीस"),
    ("mr", 31, "एकतीस"),
    ("ne", 21, "एक्काइस"),
    ("ne", 99, "उनान्सय"),
    ("ur", 21, "اکیس"),
    ("ur", 23, "تئیس"),
    ("gu", 23, "તેવીસ"),
    ("gu", 99, "નવ્વાણું"),
    ("pa", 21, "ਇੱਕੀ"),
    ("pa", 23, "ਤੇਈ"),
    ("as", 21, "একৈশ"),
    ("si", 11, "එකොළහ"),
    ("si", 23, "විසිතුන"),
    ("ml", 21, "ഇരുപത്തിയൊന്ന്"),
    ("ml", 23, "ഇരുപത്തിമൂന്ന്"),
    ("or", 42, "ବୟାଳିଶି"),
    ("kok", 11, "इकरा"),
    ("kok", 21, "एकवीस"),
    ("kn", 21, "ಇಪ್ಪತ್ತೊಂದು"),
    ("kn", 23, "ಇಪ್ಪತ್ತಮೂರು"),
    ("kn", 38, "ಮೂವತ್ತೆಂಟು"),
    ("sa", 11, "एकादश"),
    ("sa", 23, "त्रयोविंशति"),
    ("pli", 23, "tevīsati"),
    ("sd", 11, "يارهن"),
    ("sd", 23, "ٽريويهه"),
]


@pytest.mark.parametrize("lang,number,expected", BELOW_HUNDRED)
def test_below_hundred_is_one_word(lang, number, expected):
    assert num2words(number, lang=lang) == expected


def test_compounds_reach_larger_numbers_and_ordinals():
    assert num2words(123456, lang="mr") == "एक लाख तेवीस हजार चारशे छप्पन्न"
    assert num2words(23, lang="ne", to="ordinal") == "तेइसऔं"


@pytest.mark.parametrize("lang,expected", [
    ("ml", "ഒരു ലക്ഷം"),
    ("or", "ଏକ ଲକ୍ଷ"),
    ("kok", "एक लाख"),
    ("sa", "एकम् लक्षम्"),
    ("sd", "هڪ لک"),
    ("hi", "एक लाख"),
    ("te", "ఒక లక్ష"),
])
def test_lakh(lang, expected):
    # or/kok/sa/sd grouped by "ten lakh" millions, so 10**5 was "one
    # hundred thousand"; ml said "ഒന്ന് ലക്ഷം", te "ఒకటి లక్ష" and hi a
    # bare "लाख" (#247).
    assert num2words(10**5, lang=lang) == expected


def test_assamese_hundred_is_its_own_word():
    # Python glued "শ" onto the tens: "চাৰি শপঞ্চাশ ছয়" (#247).
    assert num2words(456, lang="as") == "চাৰি শ ছাপন"


SCALE_LANGS = ["kok", "ml", "ne", "or", "pa", "pli", "sa", "sd", "si", "ur"]


@pytest.mark.parametrize("lang", SCALE_LANGS)
def test_no_digits_up_to_maxval(lang):
    # Python returned str(number) from 10**9 up (#147).
    top = maxval(lang)
    for e in range(9, len(str(top)) - 1):
        for to in ("cardinal", "ordinal"):
            out = num2words(10**e, lang=lang, to=to)
            assert not re.search(r"[0-9]", out), (lang, e, to, out)
    with pytest.raises(OverflowError):
        num2words(top, lang=lang)


@pytest.mark.parametrize("lang,expected", [
    ("ne", "एक अर्ब"),
    ("ur", "ایک ارب"),
    ("pa", "ਇੱਕ ਅਰਬ"),
    ("sd", "هڪ ارب"),
    ("ml", "ഒന്ന് നൂറ് കോടി"),
    ("or", "ଏକ ଶହ କୋଟି"),
    ("kok", "एक शंभर कोटी"),
    ("si", "සියය කෝටිය"),
    ("sa", "एकम् शतम् कोटिः"),
    ("pli", "eka sata koṭi"),
])
def test_billion(lang, expected):
    assert num2words(10**9, lang=lang) == expected
