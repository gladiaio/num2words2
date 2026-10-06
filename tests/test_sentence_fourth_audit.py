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
        ("-5 x", "id", "Min lima x"),  # id says "min" (#226)
        ("25°C", "ru", "Двадцать пять градусов Цельсия"),
        ("-5°C", "ru", "Минус пять градусов Цельсия"),
        ("-5 x", "en", "Minus five x"),
        ("-0 x", "en", "Zero x"),
    ],
)
def test_negative_integers_not_read_as_decimals(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected


# --- #226: the negative word comes from the converter ---------------------

@pytest.mark.parametrize(
    "lang", ["pt_BR", "ca", "zh_CN", "fr_CH", "sr_Latn", "es_CO", "el", "ja"]
)
def test_negative_word_from_converter(lang):
    assert num2words_sentence("x -7", lang=lang) == "x " + num2words(-7, lang=lang)


def test_negative_temperature_uses_converter_word():
    assert num2words_sentence("-7°C", lang="pt_BR").startswith("Menos sete")


# --- #231: to="ordinal" keeps decimals (and negatives) cardinal -----------

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("Custa 3,50 euros, 1.234,5 unidades.", "pt",
         "Custa três vírgula cinco zero euros, mil duzentos e trinta e quatro"
         " vírgula cinco unidades."),
        ("Stojí 3,50 Kč.", "cs", "Stojí tři čárka pět Kč."),
        ("Costa 3,50 euro", "it", "Costa tre virgola cinque zero euro"),
        ("3,5 x", "ru", "Три целых пять десятых x"),
        ("It costs 3.50 euros", "en", "It costs three point five zero euros"),
        ("3 x", "pt", "Terceiro x"),
    ],
)
def test_ordinal_mode_decimals_stay_cardinal(text, lang, expected):
    assert num2words_sentence(text, lang=lang, to="ordinal") == expected
