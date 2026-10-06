# -*- coding: utf-8 -*-
"""Language codes resolve case-insensitively, BCP-47 style (gladiaio/num2words2#238)."""
from __future__ import unicode_literals

import pytest

from num2words2 import _rust, maxval, num2words

LANGS = sorted(_rust.supported_langs())


def _out(lang):
    res = []
    for x, to in [(16, "cardinal"), (1.5, "cardinal"), (3, "ordinal"),
                  (2024, "year")]:
        try:
            res.append(num2words(x, lang=lang, to=to))
        except Exception as e:  # noqa: BLE001 - errors must match too
            res.append(type(e).__name__)
    return res


@pytest.mark.parametrize("code", LANGS)
def test_case_and_separator_variants_round_trip(code):
    want = _out(code)
    for variant in {code.lower(), code.upper(), code.replace("_", "-"),
                    code.upper().replace("_", "-")}:
        assert _out(variant) == want, variant
    assert maxval(code.upper()) == maxval(code)


@pytest.mark.parametrize("raw,canonical", [
    ("EN", "en"),
    ("PT-br", "pt_BR"),
    ("zh-tw", "zh_TW"),
    ("sr_latn", "sr_Latn"),
    ("sr-Latn", "sr_Latn"),
    ("uz_cyrl", "uz_Cyrl"),
    ("uz-Latn", "uz"),
    ("en-US", "en"),
    ("cz", "cs"),
    ("jw", "jv"),
    ("uz_cyr", "uz_Cyrl"),
    ("EN_AERO_ICAO", "en_AERO"),
])
def test_resolves_like_canonical(raw, canonical):
    assert _out(raw) == _out(canonical)


def test_script_is_not_switched_silently():
    assert num2words(16, lang="sr_latn") == "šesnaest"
    assert num2words(16, lang="uz_cyrl") == "ўн олти"
    for bad in ("sr_Latx", "uz-Arab", "sr-latx-RS"):
        with pytest.raises(NotImplementedError):
            num2words(16, lang=bad)
