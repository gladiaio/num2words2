"""Tests for Serbian Latin script (sr_Latn) — issue #73."""

import pytest

from num2words2 import num2words


def test_sr_latn_basic_cardinals():
    assert num2words(1, lang="sr_Latn") == "jedan"
    assert num2words(5, lang="sr_Latn") == "pet"
    assert num2words(10, lang="sr_Latn") == "deset"
    assert num2words(24, lang="sr_Latn") == "dvadeset četiri"
    assert num2words(100, lang="sr_Latn") == "sto"
    assert num2words(1000, lang="sr_Latn") == "hiljada"
    assert num2words(1000000, lang="sr_Latn") == "milion"


def test_sr_latn_special_letters():
    # Verify the Č/Ć/Š/Ž/Đ/Lj/Nj/Dž digraphs all transliterate.
    assert num2words(4, lang="sr_Latn") == "četiri"
    assert num2words(1234, lang="sr_Latn") == "hiljada dvesta trideset četiri"


def test_sr_cyrl_alias_matches_sr_default():
    assert num2words(42, lang="sr_Cyrl") == num2words(42, lang="sr")


def test_sr_cyrl_uses_cyrillic_script():
    out = num2words(42, lang="sr_Cyrl")
    # All Latin letters should be absent in pure-Cyrillic cardinals.
    assert all(not c.isascii() or c.isspace() for c in out), out


def test_sr_latn_currency():
    assert num2words(1.50, lang="sr_Latn", to="currency") == "jedan dinar, pedeset para"


def test_sr_latn_currency_code_and_cheque():
    # gladiaio/num2words2#176
    assert num2words(1, lang="sr_Latn", to="currency", currency="EUR") == "jedan evro"
    assert num2words(5, lang="sr_Latn", to="currency", currency="EUR") == "pet evra"
    with pytest.raises(NotImplementedError):
        num2words(100, lang="sr_Latn", to="currency", currency="JPY")
    assert (
        num2words(1234.56, lang="sr_Latn", to="cheque", currency="EUR")
        == "HILJADA DVESTA TRIDESET ČETIRI AND 56/100 EVRA"
    )
