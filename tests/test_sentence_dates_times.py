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


@pytest.mark.parametrize(
    "text, expected",
    [
        # am/pm suffix, glued or spaced, kept as written (#178).
        ("at 10:30pm", "at ten thirty pm"),
        ("at 10:30 PM", "at ten thirty PM"),
        ("at 10:30 p.m. today", "at ten thirty p.m. today"),
        ("at 9:05am", "at nine oh five am"),
        ("at 12:00pm", "at twelve pm"),
        ("at 10:30A.M., then", "at ten thirty A.M., then"),
    ],
)
def test_clock_times_with_am_pm(text, expected):
    assert num2words_sentence(text) == expected


@pytest.mark.parametrize(
    "text, expected",
    [
        # Seconds and 24-hour values with a glued suffix are not claimed.
        ("at 10:30:15pm", "at ten:thirty:15pm"),
        ("at 14:30pm", "at fourteen:30pm"),
        # Not a suffix: a word that merely starts with "pm".
        ("at 10:30 pmfoo", "at ten thirty pmfoo"),
    ],
)
def test_clock_times_am_pm_left_alone(text, expected):
    assert num2words_sentence(text) == expected


def test_other_languages_dates_unchanged():
    assert num2words_sentence("le 1er mai 2024", lang="fr") == (
        "le premier mai deux mille vingt-quatre")
    assert num2words_sentence("am 1. Mai 2024", lang="de") == (
        "am ersten Mai zweitausendvierundzwanzig")


def test_failed_reading_falls_back_to_own_language_cardinal():
    # es has no ordinal for zero (#160); the sentence must not splice in an
    # English "zero".
    from num2words2 import num2words_sentence

    assert num2words_sentence("el 0º lugar", lang="es") == "el cero lugar"
