"""Clock times in running text outside English (issues #192, #193).

`num2words_sentence` read "14:30" digit group by digit group and kept the
colon in every language but English ("catorce:treinta", "vierzehn:dreißig").
It now reads the language's spoken 24-hour time where the form is attested,
and leaves the time as written elsewhere. A dotted "14.30" is a time only
with a time context (it "alle 14.30", sv "kl. 14.30", nl "14.30 uur"); it
used to be read as a decimal ("quattordici virgola tre", #193).
"""

import re

import pytest

from num2words2 import num2words_sentence


@pytest.mark.parametrize(
    "lang, text, expected",
    [
        # es (RAE: "las trece treinta", "las trece y veinte", "cero, una…").
        ("es", "a las 14:30", "a las catorce treinta"),
        ("es", "a las 14:00", "a las catorce horas"),
        ("es", "a las 14:05", "a las catorce y cinco"),
        ("es", "a la 1:30", "a la una treinta"),
        ("es", "a la 1:00", "a la una"),
        ("es", "a las 21:15", "a las veintiuna quince"),
        ("es", "a las 0:00", "a las cero horas"),
        # fr (heure is feminine; singular below two).
        ("fr", "à 14:30", "à quatorze heures trente"),
        ("fr", "à 14:00", "à quatorze heures"),
        ("fr", "à 14:05", "à quatorze heures cinq"),
        ("fr", "à 1:00", "à une heure"),
        ("fr", "à 21:00", "à vingt-et-une heures"),
        ("fr", "à 0:10", "à zéro heure dix"),
        # de: bare H:MM reads like "… Uhr" (#183).
        ("de", "um 14:30", "um vierzehn Uhr dreißig"),
        ("de", "um 14:00", "um vierzehn Uhr"),
        ("de", "um 1:05", "um ein Uhr fünf"),
        ("de", "um 14:30 Uhr", "um vierzehn Uhr dreißig"),
        # it
        ("it", "alle 14:30", "alle quattordici e trenta"),
        ("it", "alle 14:00", "alle quattordici"),
        ("it", "alle 9:05", "alle nove e cinque"),
        # pt / pt_BR (hora is feminine).
        ("pt", "às 14:30", "às catorze e trinta"),
        ("pt", "às 14:00", "às catorze horas"),
        ("pt", "às 2:05", "às duas e cinco"),
        ("pt", "à 1:00", "à uma hora"),
        ("pt", "às 21:00", "às vinte e uma horas"),
        ("pt_BR", "às 14:30", "às catorze e trinta"),
        # sv
        ("sv", "klockan 14:30", "klockan fjorton och trettio"),
        ("sv", "klockan 14:00", "klockan fjorton"),
        ("sv", "klockan 9:05", "klockan nio och fem"),
        # nl
        ("nl", "om 14:30", "om veertien uur dertig"),
        ("nl", "om 14:00", "om veertien uur"),
        ("nl", "om 9:05 uur", "om negen uur vijf"),
    ],
)
def test_clock_time(lang, text, expected):
    assert num2words_sentence(text, lang=lang) == expected


@pytest.mark.parametrize(
    "lang, text, expected",
    [
        # #193: dotted times with a time context.
        ("it", "alle 14.30", "alle quattordici e trenta"),
        ("it", "dalle 9.00 alle 18.30", "dalle nove alle diciotto e trenta"),
        ("it", "ore 9.05", "ore nove e cinque"),
        ("sv", "kl. 14.30", "kl. fjorton och trettio"),
        ("sv", "klockan 7.45", "klockan sju och fyrtiofem"),
        ("nl", "om 14.30 uur", "om veertien uur dertig"),
        ("nl", "om 14.00 uur", "om veertien uur"),
        # Without one, a dotted number stays a decimal.
        ("it", "costa 14.30", "costa quattordici virgola tre zero"),
        ("sv", "3.14", "Tre komma ett fyra"),
        # A dotted date is not a time.
        ("it", "dalle 3.10.2024", "dalle 3.10.2024"),
    ],
)
def test_dotted_time_in_context(lang, text, expected):
    assert num2words_sentence(text, lang=lang) == expected


@pytest.mark.parametrize(
    "lang, text",
    [
        # Times with seconds are left as written.
        ("es", "14:30:45"),
        ("de", "um 14:30:45"),
        # Languages without a verified reading keep the time as written.
        ("ru", "в 14:30"),
        ("pl", "o 14:30"),
        ("ja", "14:30"),
    ],
)
def test_time_left_as_written(lang, text):
    assert num2words_sentence(text, lang=lang) == text


@pytest.mark.parametrize(
    "lang", ["es", "fr", "de", "it", "pt", "pt_BR", "sv", "nl", "ru", "pl", "ja", "tr"]
)
def test_no_word_glued_to_colon(lang):
    out = num2words_sentence("Um 9:05 und 14:30, 23:59.", lang=lang)
    assert not re.search(r"[^\W\d_]:|:[^\W\d_]", out), out


def test_english_unchanged():
    assert num2words_sentence("at 14:30", lang="en") == "at fourteen thirty"
    assert num2words_sentence("at 10:00", lang="en") == "at ten o'clock"
