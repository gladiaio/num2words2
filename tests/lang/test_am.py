from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestAM(LangTest, TestCase):
    lang = "am"

    cardinal_tests = [
        (100, "መቶ"),
        (100000, "አንድ መቶ ሺህ"),
        (101, "አንድ መቶ አንድ"),
        (199, "አንድ መቶ ዘጠና ዘጠኝ"),
    ]

    ordinal_tests = [
        (1, "አንደኛ"),
        (13, "አሥራ ሦስተኛ"),
        (22, "ሃያ ሁለተኛ"),
        (10000, "አሥር ሺህኛ"),
    ]

    ordinal_num_tests = [
        (10, "10ኛ"),
        (21, "21ኛ"),
        (102, "102ኛ"),
    ]

    float_tests = [
        (12.5, "አሥራ ሁለት ነጥብ አምስት"),
        (12.51, "አሥራ ሁለት ነጥብ አምስት አንድ"),
        (12.53, "አሥራ ሁለት ነጥብ አምስት ሦስት"),
        (-0.4, "ሰልቢ ዜሮ ነጥብ አራት"),
        (-0.5, "ሰልቢ ዜሮ ነጥብ አምስት"),
        (-1.4, "ሰልቢ አንድ ነጥብ አራት"),
    ]

    currency_tests = [
        (38.4, "ሠላሳ ስምንት ብር ከ 40 ሳንቲም", {"cents": False, "currency": "ETB"}),
        (
            "0",
            "ዜሮ ብር እና ዜሮ ሳንቲም",
            {"separator": " እና", "cents": True, "currency": "ETB"},
        ),
        ("1.50", "አንድ ብር ከ አምሳ ሳንቲም", {"cents": True, "currency": "ETB"}),
    ]

    year_tests = [
        (1990, "አሥራ ዘጠኝ መቶ ዘጠና"),
        (5555, "አምሳ አምስት መቶ አምሳ አምስት"),
        (2017, "ሁለት ሺህ አሥራ ሰባት"),
        (1066, "አንድ ሺህ ስድሳ ስድስት"),
        (1865, "አሥራ ስምንት መቶ ስድሳ አምስት"),
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
        # Negative floats are handled by float_tests, no integer negative_tests here.
        self._run_negative_tests()

    def test_to_overflow(self):
        with self.assertRaises(OverflowError):
            num2words(
                "1000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "0000000000000000000000000000000000000000000000000000000"
                "00000000000000000000000000000000",
                lang="am",
            )


def test_am_no_latin_prefix_in_high_words():
    # Regression for savoirfairelinux/num2words#591 — stray Latin "m"
    # appeared between thousands and "ሚሊዮን" in big numbers.
    from num2words2 import num2words

    out = num2words(568476685, lang="am")
    # No ASCII letters allowed; everything must be Ethiopic + spaces.
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "ሚሊዮን" in out
    out_b = num2words(1_000_000_000, lang="am")
    assert "ቢሊዮን" in out_b
    assert "b" not in out_b.lower()
