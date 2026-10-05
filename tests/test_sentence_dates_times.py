"""English dates and clock times in num2words_sentence (issue #152)."""

import pytest

from num2words2 import num2words_sentence


@pytest.mark.parametrize(
    "text, expected",
    [
        ("1st May 2024", "First May two thousand and twenty-four"),
        ("2nd June 1999",
         "Second June one thousand, nine hundred and ninety-nine"),
        ("On 3rd March 2020 we met", "On third March two thousand and twenty we met"),
        # Month-first order was already right and must stay so.
        ("May 1st, 2024", "May first, two thousand and twenty-four"),
        ("December 25", "December twenty-fifth"),
    ],
)
def test_year_is_never_ordinal(text, expected):
    assert num2words_sentence(text) == expected


def test_issue_152_year_suggested_assertion():
    assert "twenty-fourth" not in num2words_sentence("1st May 2024")


@pytest.mark.parametrize(
    "text, expected",
    [
        ("The meeting is at 10:30", "The meeting is at ten thirty"),
        ("at 10:00", "at ten o'clock"),
        ("at 9:05", "at nine oh five"),
        ("at 14:00", "at fourteen hundred"),
    ],
)
def test_clock_times(text, expected):
    out = num2words_sentence(text)
    assert out == expected
    assert ":" not in out


def test_other_languages_dates_unchanged():
    assert num2words_sentence("le 1er mai 2024", lang="fr") == (
        "le premier mai deux mille vingt-quatre")
    assert num2words_sentence("am 1. Mai 2024", lang="de") == (
        "am ersten Mai zweitausendvierundzwanzig")
