# -*- coding: utf-8 -*-
"""Scale words above a million (gladiaio/num2words2#147).

These languages used to return the digits of str(number) from 10^9. Each
now spells its attested scale word, or raises OverflowError at maxval
where none is attested (ceilings are pinned in test_maxval_contract.py).
"""
from __future__ import unicode_literals

import pytest

from num2words2 import num2words

FIRST_NEW_SCALE = [
    ("bm", 1000000000, "miliyari"),
    ("bs", 1000000000, "milijarda"),
    ("ceb", 1000000000, "usa bilyon"),
    ("ckb", 1000000000, "یەک ملیار"),
    ("ff", 1000000000, "miliyaar go'o"),  # count follows the noun (#263)
    ("fil", 1000000000, "isa bilyon"),
    ("fo", 1000000000, "ein milliard"),
    ("gl", 1000000000, "un mil millón"),
    ("ka", 1000000000000, "ერთი ტრილიონი"),
    ("km", 1000000000, "មួយពាន់ លាន"),
    ("ku", 1000000000, "yek milyar"),
    ("lb", 1000000000, "eent Milliard"),
    ("lg", 1000000000, "kawumbi"),
    ("lo", 1000000000, "ໜຶ່ງພັນ ລ້ານ"),
    ("mk", 1000000000, "еден милијарда"),
    ("my", 1000000000, "တစ်ရာ ကုဋေ"),
    ("nn", 1000000000, "ein milliard"),
    ("oc", 1000000000, "un miliard"),
    ("om", 1000000000, "biliyoona"),
    ("pap", 1000000000, "un biyon"),
    ("ps", 1000000000, "یو میلیارد"),
    ("rw", 1000000000, "miliyari"),
    ("ti", 1000000000, "ቢልዮን"),
    ("xh", 1000000000, "ibhiliyoni"),
    ("yi", 1000000000, "איינס מיליאַרד"),
    ("zu", 1000000000, "isigidigidi"),
]

CEILING_ONLY = ["ban", "hmn", "ki", "lus", "miz"]


@pytest.mark.parametrize("lang,n,words", FIRST_NEW_SCALE)
def test_first_new_scale_is_spelled(lang, n, words):
    assert num2words(n, lang=lang) == words


@pytest.mark.parametrize("lang", CEILING_ONLY)
def test_no_attested_billion_raises(lang):
    assert num2words(999999999, lang=lang)
    for to in ("cardinal", "ordinal", "year", "currency"):
        with pytest.raises(OverflowError):
            num2words(10**9, lang=lang, to=to)
