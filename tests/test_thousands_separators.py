"""Numbers written with thousands separators (issue #151).

A numeric string like "1,000" used to be handed to the sentence converter,
which split it at the separator and returned words for a *different* number
("One"). It must now be read as the number it spells, or raise ValueError
when the separator is ambiguous — never silently change the value.
"""

import pytest

from num2words2 import num2words, num2words_sentence


@pytest.mark.parametrize(
    "value, kwargs, expected",
    [
        ("1,000", {}, "one thousand"),
        ("1,000,000", {}, "one million"),
        ("12,345", {}, "twelve thousand, three hundred and forty-five"),
        ("1,234.5", {}, "one thousand, two hundred and thirty-four point five"),
        ("24,120.10", {},
         "twenty-four thousand, one hundred and twenty point one zero"),
        ("-1,000", {}, "minus one thousand"),
        ("1,000", {"to": "ordinal"}, "one thousandth"),
        ("1,234.56", {"to": "currency"},
         "one thousand, two hundred and thirty-four euros, fifty-six cents"),
        ("1.000.000", {"lang": "de"}, "eine Million"),
        ("1.234,5", {"lang": "de"},
         "eintausendzweihundertvierunddreißig Komma fünf"),
        ("1'000'000", {"lang": "de"}, "eine Million"),
        ("1 000", {"lang": "fr"}, "mille"),
        ("1 000", {"lang": "fr"}, "mille"),
        ("1 000", {"lang": "fr"}, "mille"),
        ("1 000", {"lang": "fr"}, "mille"),
        # pt_BR keeps its decimal-mark handling (#92): ',' is "vírgula",
        # a US-style '.' decimal is "ponto".
        ("1.234,56", {"lang": "pt_BR"},
         "mil, duzentos e trinta e quatro vírgula cinco seis"),
        ("1,234.56", {"lang": "pt_BR"},
         "mil, duzentos e trinta e quatro ponto cinco seis"),
        # A single decimal comma is still a decimal, and no longer
        # sentence-cased.
        ("1,5", {}, "one point five"),
        ("1,5", {"lang": "de"}, "eins Komma fünf"),
    ],
)
def test_grouped_string_is_read_as_the_number(value, kwargs, expected):
    assert num2words(value, **kwargs) == expected


@pytest.mark.parametrize(
    "value, lang",
    [
        ("1,000", "de"),     # decimal comma or thousands separator?
        ("1,000", "pt_BR"),
        ("1,2,3", "en"),     # malformed groups
        ("1,0000,000", "en"),
        ("1.2.3", "de"),
        ("1 00", "fr"),
    ],
)
def test_ambiguous_or_malformed_grouping_raises(value, lang):
    with pytest.raises(ValueError):
        num2words(value, lang=lang)


def test_plain_decimal_strings_unchanged():
    # Decimal-parseable strings keep their existing reading.
    assert num2words("1.000") == "one"
    assert num2words("1.5") == "one point five"
    assert num2words("1ra", lang="es") == "primera"


@pytest.mark.parametrize(
    "text, lang, expected",
    [
        ("Population: 1,000,000 people", "en", "Population: one million people"),
        ("I paid $1,234.56", "en",
         "I paid one thousand, two hundred and thirty-four dollars, "
         "fifty-six cents"),
        ("-1,000 and 1,000.", "en", "Minus one thousand and one thousand."),
        ("1.000.000 Einwohner", "de", "Eine Million Einwohner"),
        ("Es kostet 1.234,56 Euro", "de",
         "Es kostet eintausendzweihundertvierunddreißig Komma fünf sechs Euro"),
        ("1 000 personnes", "fr", "Mille personnes"),
    ],
)
def test_sentence_reads_grouped_numbers_whole(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected


def test_sentence_non_grouping_separators_untouched():
    # Lists, versions and IP addresses are not thousands grouping.
    assert num2words_sentence("Items 1,2,3") == "Items one point two,three"
    assert "one hundred and ninety-two" in num2words_sentence("IP: 192.168.1.1")
    assert num2words_sentence("1st, 2nd, and 3rd place") == (
        "First, second, and third place")
