"""Regression tests for gladiaio/num2words2#259: leftover ordinal words."""

import pytest

from num2words2 import num2words

CASES = [
    # nb: 13 fell through to the "en" -> "første" rule; scale words stayed
    # cardinal.
    ("nb", 13, "trettende"),
    ("nb", 113, "ett hundre og trettende"),
    ("nb", 21, "tjueførste"),
    ("nb", 2 * 10**6, "to millionte"),
    ("nb", 10**7, "ti millionte"),
    ("nb", 10**9, "milliardte"),
    ("nb", 2 * 10**9, "to milliardte"),
    # sv: "två miljonerde", "en miljardde".
    ("sv", 2 * 10**6, "tvåmiljonte"),
    ("sv", 10**9, "miljardte"),
    ("sv", 2 * 10**9, "tvåmiljardte"),
    # bg: hundreds and round thousands.
    ("bg", 200, "двестотен"),
    ("bg", 300, "тристотен"),
    ("bg", 500, "петстотен"),
    ("bg", 1200, "хиляда двестотен"),
    ("bg", 2000, "двехиляден"),
    ("bg", 5000, "петхиляден"),
    # it: round multiples of a scale word are one word.
    ("it", 2 * 10**6, "duemilionesimo"),
    ("it", 23 * 10**6, "ventitremilionesimo"),
    ("it", 2 * 10**9, "duemiliardesimo"),
    # da: 60th.
    ("da", 50, "halvtredsende"),
    ("da", 60, "tressende"),
    ("da", 61, "enogtressende"),
    # af: no "een" before a bare scale word, as nl (#252).
    ("af", 100, "honderdste"),
    ("af", 10**6, "miljoenste"),
    ("af", 10**9, "miljardste"),
    ("af", 2 * 10**6, "twee miljoenste"),
    # nn: real ordinals instead of cardinal + "-de".
    ("nn", 1, "første"),
    ("nn", 7, "sjuande"),
    ("nn", 11, "ellevte"),
    ("nn", 13, "trettande"),
    ("nn", 20, "tjuande"),
    ("nn", 21, "tjue første"),
    ("nn", 30, "trettiande"),
    ("nn", 100, "hundrede"),
    ("nn", 1000, "tusende"),
    ("nn", 10**6, "millionte"),
    ("nn", 2 * 10**6, "to millionte"),
    # bs: real ordinals instead of cardinal + ".", as hr.
    ("bs", 0, "nulti"),
    ("bs", 4, "četvrti"),
    ("bs", 12, "dvanaesti"),
    ("bs", 21, "dvadeset prvi"),
    ("bs", 200, "dvjestoti"),
    ("bs", 1000, "hiljaditi"),
    ("bs", 10**6, "milioniti"),
    ("bs", -3, "minus treći"),
    # cs: thousands with a multiplier above 9.
    ("cs", 10**4, "desetitisící"),
    ("cs", 20000, "dvacetitisící"),
    ("cs", 10**5, "stotisící"),
    ("cs", 12001, "dvanáctitisící první"),
]


@pytest.mark.parametrize("lang,value,expected", CASES)
def test_ordinal_word(lang, value, expected):
    assert num2words(value, lang=lang, to="ordinal") == expected
