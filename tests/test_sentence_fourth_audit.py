"""Regression tests for the fourth audit pass over num2words_sentence
(#195, #225-#235)."""

import pytest

from num2words2 import num2words, num2words_sentence


# --- #234 / #194: decimals keep their digits as written -------------------

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("3.50 dollars, Python 3.10", "en",
         "Three point five zero dollars, Python three point one zero"),
        ("3,50 €", "fr", "Trois virgule cinq zéro €"),
        ("Es kostet 3,50 Euro", "de", "Es kostet drei Komma fünf null Euro"),
        ("1,234.50 x", "en",
         "One thousand, two hundred and thirty-four point five zero x"),
    ],
)
def test_trailing_zeros_kept(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected


@pytest.mark.parametrize("lang", ["en", "fr", "ru", "ja", "pt_BR"])
def test_decimal_matches_num2words_string(lang):
    word = num2words_sentence("x 3.50", lang=lang).split(" ", 1)[1]
    expected = num2words("3.50", lang=lang)
    if lang == "pt_BR":
        expected = expected.replace("vírgula", "ponto")
    assert word == expected


# --- #225: negatives and temperatures take the integer path ---------------

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("-5 x", "ru", "Минус пять x"),
        ("-5 x", "cs", "Mínus pět x"),
        ("-5 x", "id", "Minus lima x"),
        ("25°C", "ru", "Двадцать пять градусов Цельсия"),
        ("-5°C", "ru", "Минус пять градусов Цельсия"),
        ("-5 x", "en", "Minus five x"),
        ("-0 x", "en", "Zero x"),
    ],
)
def test_negative_integers_not_read_as_decimals(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected
