"""Dotted numbers and clock times in running text (issue #183).

German `N.` is an ordinal ("1. Mai"), but the ordinal pass also fired when a
digit followed the dot: "1.5 Leute" -> "Erste5 Leute", "um 14.30 Uhr" ->
"um vierzehnte30 Uhr". A dot followed by a digit is no longer an ordinal;
"14.30 Uhr" is a clock time, and any other dotted digit run is left as
written (German decimals use a comma, so "1.5" has no standard reading).
"""

import re

import pytest

from num2words2 import num2words_sentence


@pytest.mark.parametrize(
    "text, expected",
    [
        # The issue's repros.
        ("1.5 Leute", "1.5 Leute"),
        ("um 14.30 Uhr", "um vierzehn Uhr dreißig"),
        # Clock times with "Uhr", dot or colon.
        ("um 14.00 Uhr", "um vierzehn Uhr"),
        ("um 14:30 Uhr", "um vierzehn Uhr dreißig"),
        ("um 14:00 Uhr", "um vierzehn Uhr"),
        ("um 9.05 Uhr", "um neun Uhr fünf"),
        ("um 1.30 Uhr", "um ein Uhr dreißig"),
        ("um 0.15 Uhr", "um null Uhr fünfzehn"),
        # Other dotted digit runs stay as written.
        ("Version 2.10", "Version 2.10"),
        ("am 3.10.2024", "am 3.10.2024"),
        ("um 25.30 Uhr", "um 25.30 Uhr"),
        ("um 14.75 Uhr", "um 14.75 Uhr"),
        # Unchanged: ordinals without a digit after the dot, comma decimals,
        # dot thousands grouping (#177).
        ("1. Mai", "Erster Mai"),  # strong ending, no article (#195)
        ("am 3. Oktober", "am dritten Oktober"),
        ("der 1.", "der eins."),
        ("1,5 Leute", "Eins Komma fünf Leute"),
        ("Es kamen 1.000 Leute", "Es kamen eintausend Leute"),
    ],
)
def test_de_dotted_numbers(text, expected):
    assert num2words_sentence(text, lang="de") == expected


# A letter directly followed by a digit, or a digit directly followed by a
# letter: a number word glued to leftover digits ("Erste5").
GLUED = re.compile(r"[^\W\d_]\d|\d[^\W\d_]")


@pytest.mark.parametrize(
    "lang, text",
    [
        ("de", "1.5 Leute kamen um 14.30 Uhr."),
        ("de", "Am 3. Oktober um 9.05 Uhr, Version 2.10."),
        ("de", "Es kostet 3,50 Euro, 1.000 Leute, 3.14 und 14:00 Uhr."),
        ("de", "Der 1. Mai und am 3.10.2024 um 0.15 Uhr."),
        ("fr", "1.5 personnes à 14:30, 3.14 et 2,5."),
        ("fr", "Le 1er mai, 1,5 kg."),
        ("es", "1.5 personas a las 14:30, 1.000 personas, 3.14."),
        ("es", "El 1º de mayo, 1,5 kilos."),
        ("it", "1.5 persone alle 14:30 e alle 14.30, 1,5 kg."),
        ("it", "Il 1º maggio, 3.14."),
    ],
)
def test_no_word_glued_to_digits(lang, text):
    out = num2words_sentence(text, lang=lang)
    assert not GLUED.search(out), out
