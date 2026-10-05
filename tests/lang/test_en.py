from unittest import TestCase

from num2words2 import num2words  # To get access to OverflowError
from tests.basetest import LangTest


class TestEN(LangTest, TestCase):
    lang = "en"

    cardinal_tests = [
        (0, "zero"),
        (1, "one"),
        (10, "ten"),
        (12, "twelve"),
        (21, "twenty-one"),
        (100, "one hundred"),
        (101, "one hundred and one"),
        (199, "one hundred and ninety-nine"),
        (1003, "one thousand and three"),
        (12345, "twelve thousand, three hundred and forty-five"),
        (1000000, "one million"),
    ]

    ordinal_tests = [
        (0, "zeroth"),
        (1, "first"),
        (12, "twelfth"),
        (13, "thirteenth"),
        (22, "twenty-second"),
        (130, "one hundred and thirtieth"),
        (1003, "one thousand and third"),
    ]

    ordinal_num_tests = [
        (10, "10th"),
        (21, "21st"),
        (102, "102nd"),
        (73, "73rd"),
    ]

    year_tests = [
        (1990, "nineteen ninety"),
        (5555, "fifty-five fifty-five"),
        (2017, "twenty seventeen"),
        (1066, "ten sixty-six"),
        (1865, "eighteen sixty-five"),
        (3000, "three thousand"),
        (2001, "two thousand and one"),
        (1901, "nineteen oh-one"),
        (2000, "two thousand"),
        (905, "nine oh-five"),
        (6600, "sixty-six hundred"),
        (1900, "nineteen hundred"),
        (600, "six hundred"),
        (50, "fifty"),
        (0, "zero"),
        (-44, "forty-four BC"),
        (-44, "forty-four BCE", {"suffix": "BCE"}),
        (1, "one AD", {"suffix": "AD"}),
        (66, "sixty-six m.y.a.", {"suffix": "m.y.a."}),
        (-66000000, "sixty-six million BC"),
    ]

    currency_tests = [
        (
            "38.4",
            "thirty-eight dollars and 40 cents",
            {"separator": " and", "cents": False, "currency": "USD"},
        ),
        ("0", "zero dollars", {"separator": " and", "cents": False, "currency": "USD"}),
        (
            "1.01",
            "one dollar and one cent",
            {"separator": " and", "cents": True, "currency": "USD"},
        ),
        (
            "4778.00",
            "four thousand, seven hundred and seventy-eight US dollars and zero cents",
            {"separator": " and", "cents": True, "currency": "USD", "adjective": True},
        ),
        (
            "4778.00",
            "four thousand, seven hundred and seventy-eight dollars and zero cents",
            {"separator": " and", "cents": True, "currency": "USD"},
        ),
        (
            "1.1",
            "one peso and ten cents",
            {"separator": " and", "cents": True, "currency": "MXN"},
        ),
        (
            "158.3",
            "one hundred and fifty-eight pesos and thirty cents",
            {"separator": " and", "cents": True, "currency": "MXN"},
        ),
        (
            "2000.00",
            "two thousand pesos and zero cents",
            {"separator": " and", "cents": True, "currency": "MXN"},
        ),
        (
            "4.01",
            "four pesos and one cent",
            {"separator": " and", "cents": True, "currency": "MXN"},
        ),
        (
            "2000.00",
            "two thousand sums and zero tiyins",
            {"separator": " and", "cents": True, "currency": "UZS"},
        ),
        (
            "2000.00",
            "two thousand yen and zero sen",
            {"separator": " and", "cents": True, "currency": "JPY"},
        ),
        (
            "2000.00",
            "two thousand won and zero jeon",
            {"separator": " and", "cents": True, "currency": "KRW"},
        ),
    ]

    float_tests = [
        (0.12, "zero point one two"),
        (-0.12, "minus zero point one two"),
        (12.5, "twelve point five"),
        (12.51, "twelve point five one"),
        (-0.4, "minus zero point four"),
        (-1.4, "minus one point four"),
        (-10.25, "minus ten point two five"),
        (-0.001, "minus zero point zero zero one"),
    ]

    negative_tests = [
        (-1, "minus one"),
        (-12, "minus twelve"),
        (-100, "minus one hundred"),
    ]

    def test_cardinal(self):
        self._run_cardinal_tests()

    def test_ordinal(self):
        self._run_ordinal_tests()

    def test_ordinal_num(self):
        self._run_ordinal_num_tests()

    def test_year(self):
        self._run_year_tests()

    def test_currency(self):
        self._run_currency_tests()

    def test_float(self):
        self._run_float_tests()

    def test_negative(self):
        self._run_negative_tests()

    def test_overflow(self):
        # This specific test does not fit the generic test structure, so it's kept as a specific test.
        with self.assertRaises(OverflowError):
            num2words(
                "1000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "00000000000000000000000000000000"
            )


def test_en_currency_audit_includes_common_codes():
    # Regression for num2words2#74 (ports savoirfairelinux/num2words#590).
    from num2words2 import num2words
    assert "dollar" in num2words(12, lang="en", to="currency", currency="SGD")
    assert "franc" in num2words(12, lang="en", to="currency", currency="CHF")
    assert "dirham" in num2words(12, lang="en", to="currency", currency="AED")
    assert "yen" in num2words(12, lang="en", to="currency", currency="JPY")
    assert "rupee" in num2words(12, lang="en", to="currency", currency="INR")
    # No NotImplementedError for these codes
    for c in ["NZD", "HKD", "CNY", "KRW", "MXN", "BRL", "ZAR", "SAR", "QAR", "KWD"]:
        num2words(12, lang="en", to="currency", currency=c)


def test_en_mixed_text_and_numerals():
    # Regression for num2words2#61 (ports savoirfairelinux/num2words#281).
    from num2words2 import num2words
    assert num2words("text 1", lang="en") == "text one"
    assert num2words("I have 5 apples", lang="en") == "I have five apples"
    # Pure numeric strings still work.
    assert num2words("5", lang="en") == "five"
    assert num2words("5.5", lang="en") == "five point five"


def test_en_year_rejects_non_integer_float():
    # Regression for num2words2#67 (ports savoirfairelinux/num2words#316).
    import pytest

    from num2words2 import num2words
    with pytest.raises(TypeError):
        num2words(1980.6, lang="en", to="year")
    # Integer floats are accepted (1980.0 == 1980).
    assert num2words(1980.0, lang="en", to="year") == "nineteen eighty"
    assert num2words(1980, lang="en", to="year") == "nineteen eighty"
