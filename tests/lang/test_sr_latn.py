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


def test_sr_latn_currency_feminine_unit():
    # gladiaio/num2words2#188
    assert num2words(21, lang="sr_Latn", to="currency", currency="RUB") == (
        "dvadeset jedna rublja"
    )
    assert num2words(2.02, lang="sr_Latn", to="currency", currency="RUB") == (
        "dve rublje, dve kopejke"
    )
    assert num2words(2, lang="sr_Latn", to="currency", currency="EUR") == "dva evra"
    assert (
        num2words(21.5, lang="sr_Latn", to="cheque", currency="RUB")
        == "DVADESET JEDNA AND 50/100 RUBALJA"
    )


def test_sr_latn_ruble_genitive_plural():
    # gladiaio/num2words2#198: standard genitive plural "rubalja", not "rublji"
    assert num2words(5, lang="sr_Latn", to="currency", currency="RUB") == (
        "pet rubalja"
    )
    assert num2words(5, lang="sr", to="currency", currency="RUB") == "пет рубаља"
    assert num2words(5, lang="sr_Cyrl", to="currency", currency="RUB") == (
        "пет рубаља"
    )
