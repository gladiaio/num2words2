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


# --- #227: a hyphen after a letter of any script is not a minus -----------

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("מקום 1 ו-2.", "he", "מקום אחת ו-שתיים."),
        ("é-2", "fr", "é-deux"),
        ("и-2", "ru", "и-два"),
        ("x-2", "en", "x-two"),
        ("и -2", "ru", "и минус два"),
        ("我有5个苹果", "zh", "我有五个苹果"),
        # Scripts without word spaces: the hyphen is a sign.
        ("温度是-5度", "zh", "温度是负五度"),
    ],
)
def test_hyphen_after_letter_is_not_minus(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected


# --- #228: unreadable numeric characters no longer fail the whole call ----

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("5 m²", "en", "Five m²"),
        ("½ cup and 3 eggs", "en", "½ cup and three eggs"),
        ("5½ x", "en", "5½ x"),
        ("10² x", "en", "10² x"),
        ("12٫5", "en", "Twelve point five"),
        ("١٢٣ x", "ar", num2words("١٢٣", lang="ar") + " x"),
        ("१२ x", "hi", "बारह x"),
        ("５個", "ja", "五個"),
    ],
)
def test_non_ascii_digits(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected


def test_arabic_decimal_separator_both_entry_points():
    expected = num2words("12.5", lang="ar")
    assert num2words_sentence("١٢٫٥", lang="ar") == expected
    assert num2words("١٢٫٥", lang="ar") == expected


# --- #229: ranges, phone numbers and dotted sequences ---------------------

@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("years 1990-2000", "en",
         "years one thousand, nine hundred and ninety to two thousand"),
        ("pages 10-20.", "en", "pages ten to twenty."),
        ("Jahre 1990-2000", "de",
         "Jahre eintausendneunhundertneunzig - zweitausend"),
        # Left as written: phone numbers, Y <= X, ISO and dotted dates,
        # IP addresses, versions, glued letters.
        ("call 555-1234", "en", "call 555-1234"),
        ("score 3-2", "en", "score 3-2"),
        ("2023-12-25", "en", "2023-12-25"),
        ("25.12.2023", "ru", "25.12.2023"),
        ("192.168.1.1", "en", "192.168.1.1"),
        ("v2.0.1", "en", "v2.0.1"),
        ("10-20km", "en", "10-20km"),
        ("version 3.10", "en", "version three point one zero"),
    ],
)
def test_ranges_and_sequences(text, lang, expected):
    assert num2words_sentence(text, lang=lang) == expected
