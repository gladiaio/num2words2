from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestKOK(LangTest, TestCase):
    lang = "kok"

    cardinal_tests = [
        (0, "शून्य"),
        # Every number below a hundred has its own word (#247).
        (11, "इकरा"),
        (16, "सोळा"),
        (21, "एकवीस"),
        (42, "बेचाळीस"),
        (100, "एक शंभर"),
        (101, "एक शंभर आनी एक"),
        (1000, "एक हजार"),
        # The thousands branch joins with a bare space, not आनी — so 1001
        # has no connector where 101 does.
        (1001, "एक हजार एक"),
        (1234, "एक हजार दोन शंभर आनी चवतीस"),
    ]

    # 1-4 are suppletive; 6 and 9 contract; 5, 7, 8 and 10 are the plain
    # suffix. From 11 up the suffix arm applies unchanged.
    ordinal_tests = [
        (1, "पयलो"),
        (2, "दुसरो"),
        (3, "तिसरो"),
        (4, "चवथो"),
        (5, "पांचवो"),
        (6, "सव्वो"),
        (7, "सातवो"),
        (8, "आठवो"),
        (9, "नव्वो"),
        (10, "धावो"),
        (11, "इकरावो"),
    ]

    ordinal_num_tests = [
        (1, "1वो"),
        (10, "10वो"),
        (11, "11वो"),
        (102, "102वो"),
    ]

    float_tests = [
        (12.5, "बारा पुंतो पांच"),
        (-0.4, "रीण शून्य पुंतो चार"),
    ]

    negative_tests = [
        (-1, "रीण एक"),
        (-100, "रीण एक शंभर"),
    ]

    currency_tests = [
        # cents=False keeps the cents as digits (#220).
        (38.4, "आट्टीस रुपया 40 पैसो", {"cents": False, "currency": "INR"}),
        ("1.50", "एक रुपया पन्नास पैसो", {"cents": True, "currency": "INR"}),
        (12.34, "बारा डॉलर चवतीस सेंट", {"currency": "USD"}),
    ]

    # to_year ignores longval and delegates to to_cardinal — no year pairing.
    year_tests = [
        (1990, "एक हजार नव शंभर आनी नव्वद"),
        (2017, "दोन हजार सतरा"),
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

    def test_unknown_currency_code_raises(self):
        # Unknown codes used to fall back to the default currency (#219).
        with self.assertRaisesRegex(NotImplementedError, 'Currency code "GBP"'):
            num2words(12.34, lang="kok", to="currency", currency="GBP")

    def test_float(self):
        self._run_float_tests()

    def test_negative(self):
        self._run_negative_tests()

    def test_ordinal_still_accepts_zero_and_negatives(self):
        # Upstream Num2Word_KOK never calls verify_ordinal; unchanged here.
        # The 1..=10 test is on the signed value, so -1 takes the suffix arm.
        self.assertEqual(num2words(0, to="ordinal", lang="kok"), "शून्यवो")
        self.assertEqual(num2words(-1, to="ordinal", lang="kok"), "रीण एकवो")
        self.assertEqual(num2words(-1, to="ordinal_num", lang="kok"), "-1वो")


def test_kok_is_written_in_devanagari_not_transliteration():
    # lang_KOK.py spelled every numeral in Latin transliteration ("ek",
    # "chalis", "xambhar"). Devanagari is Konkani's official script.
    out = num2words(568476685, lang="kok")
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "लाख" in out
    assert "हजार" in out
    assert num2words(1, lang="kok") == "एक"
    assert num2words(40, lang="kok") == "चाळीस"
    assert num2words(-1, lang="kok").startswith("रीण")
    assert "पुंतो" in num2words(1.5, lang="kok")
