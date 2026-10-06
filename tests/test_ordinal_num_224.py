# -*- coding: utf-8 -*-
"""to='ordinal_num' agrees with the language's own ordinal (#224)."""
import pytest

from num2words2 import _rust, num2words


@pytest.mark.parametrize("n, expected", [
    (1, "1:a"), (2, "2:a"), (3, "3:e"), (11, "11:e"), (12, "12:e"),
    (21, "21:a"), (22, "22:a"), (101, "101:a"), (111, "111:e"),
    (112, "112:e"), (1001, "1001:a"),
])
def test_sv_suffix_follows_the_last_two_digits(n, expected):
    assert num2words(n, lang="sv", to="ordinal_num") == expected


def test_kn_numeral_plus_suffix():
    assert num2words(1, lang="kn", to="ordinal_num") == "1ನೇ"
    assert num2words(100, lang="kn", to="ordinal_num") == "100ನೇ"


def test_ha_raises_instead_of_english_suffixes():
    with pytest.raises(NotImplementedError,
                       match="lang='ha' does not support to='ordinal_num'"):
        num2words(2, lang="ha", to="ordinal_num")


@pytest.mark.parametrize("lang, expected", [
    ("gu", ["1લો", "2જો", "3જો", "4મો"]),
    ("mr", ["1ला", "2रा", "3रा", "4था"]),
])
def test_one_digit_script(lang, expected):
    assert [num2words(n, lang=lang, to="ordinal_num")
            for n in (1, 2, 3, 4)] == expected


@pytest.mark.parametrize("lang", sorted(_rust.supported_langs()))
@pytest.mark.parametrize("token", ["NaN", "inf", "-inf", "Infinity"])
def test_nan_and_inf_are_value_errors(lang, token):
    with pytest.raises(ValueError):
        num2words(token, lang=lang, to="ordinal_num")
