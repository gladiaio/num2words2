from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestTI(LangTest, TestCase):
    lang = "ti"

    cardinal_tests = [
        (100, "ሚእቲ"),
        (101, "ሚእቲ ን ሓደ"),
        (199, "ሚእቲ ን ተስዓ ን ትሽዓተ"),
        (1000, "ሽሕ"),
        (100000, "ሚእቲ ሽሕ"),
        # Teens are not lexicalised — 11 is built as "ten and one".
        (11, "ዓሰርተ ን ሓደ"),
        (22, "ዕስራ ን ክልተ"),
    ]

    # 1..10 are suppletive; 11 and up take the free-standing word መበል
    # before the plain, uninflected cardinal.
    ordinal_tests = [
        (1, "ቀዳማይ"),
        (2, "ካልኣይ"),
        (3, "ሳልሳይ"),
        (10, "ዓስራይ"),
        (11, "መበል ዓሰርተ ን ሓደ"),
        (22, "መበል ዕስራ ን ክልተ"),
        (100, "መበል ሚእቲ"),
        (10000, "መበል ዓሰርተ ሽሕ"),
    ]

    # The digit form mirrors the spoken rule: ይ on the digit up to 10,
    # then the መበል prefix on the bare digit.
    ordinal_num_tests = [
        (1, "1ይ"),
        (10, "10ይ"),
        (11, "መበል 11"),
        (21, "መበል 21"),
        (102, "መበል 102"),
    ]

    float_tests = [
        (12.5, "ዓሰርተ ን ክልተ ነጥቢ ሓሙሽተ"),
        (12.51, "ዓሰርተ ን ክልተ ነጥቢ ሓሙሽተ ሓደ"),
        (-0.4, "ኣሉታ ባዶ ነጥቢ ኣርባዕተ"),
        (-0.5, "ኣሉታ ባዶ ነጥቢ ሓሙሽተ"),
        (-1.4, "ኣሉታ ሓደ ነጥቢ ኣርባዕተ"),
    ]

    negative_tests = [
        (-1, "ኣሉታ ሓደ"),
        (-100, "ኣሉታ ሚእቲ"),
        (-1999, "ኣሉታ ሽሕ ን ትሽዓተ ሚእቲ ን ተስዓ ን ትሽዓተ"),
    ]

    currency_tests = [
        (38.4, "ሰላሳ ን ሸሞንተ ብር", {"cents": False, "currency": "ETB"}),
        ("0", "ባዶ ብር", {"cents": True, "currency": "ETB"}),
        ("1.50", "ሓደ ብር ሓምሳ ሳንቲም", {"cents": True, "currency": "ETB"}),
        (12.34, "ዓሰርተ ን ክልተ ናቕፋ ሰላሳ ን ኣርባዕተ ሳንቲም", {"currency": "ERN"}),
    ]

    # to_year ignores longval and delegates to to_cardinal, so there is no
    # year-pairing here: 1990 is the plain cardinal, not "nineteen ninety".
    year_tests = [
        (1990, "ሽሕ ን ትሽዓተ ሚእቲ ን ተስዓ"),
        (2017, "ክልተ ሽሕ ን ዓሰርተ ን ሸውዓተ"),
        (1066, "ሽሕ ን ስሳ ን ሽዱሽተ"),
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

    def test_ordinal_rejects_zero_negative_and_float(self):
        # Tigrinya has no ordinal for zero, and verify_ordinal rejects
        # negatives and non-integers as elsewhere in the library.
        for mode in ("ordinal", "ordinal_num"):
            for value in (0, -1, -11, 3.14):
                with self.subTest(to=mode, value=value):
                    with self.assertRaises(TypeError):
                        num2words(value, to=mode, lang="ti")

    def test_whole_floats_are_valid_ordinals(self):
        # An integer-valued float is not an error; it takes the integer path.
        self.assertEqual(num2words(5.0, to="ordinal", lang="ti"), "ሓምሻይ")
        self.assertEqual(num2words(11.0, to="ordinal", lang="ti"), "መበል ዓሰርተ ን ሓደ")


def test_ti_is_written_in_geez_not_transliteration():
    # Upstream lang_TI.py spelled every numeral in Latin transliteration
    # ("ḥade", "mi'ti"). Tigrinya is written in Ge'ez; guard the regression.
    out = num2words(568476685, lang="ti")
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "ሚልዮን" in out
    assert "ሽሕ" in out
    # A few forms that a transliterated build would get wrong.
    assert num2words(1, lang="ti") == "ሓደ"
    assert num2words(60, lang="ti") == "ስሳ"
    assert num2words(-1, lang="ti").startswith("ኣሉታ")
