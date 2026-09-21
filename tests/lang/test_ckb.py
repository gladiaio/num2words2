from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestCKB(LangTest, TestCase):
    lang = "ckb"

    cardinal_tests = [
        (0, "سفر"),
        (11, "یانزە"),
        (17, "حەڤدە"),
        (21, "بیست و یەک"),
        (42, "چل و دوو"),
        (90, "نەوەد"),
        (100, "سەد"),
        (101, "سەد و یەک"),
        # The thousands and millions branches keep the leading یەک, unlike
        # the hundreds branch.
        (1000, "یەک هەزار"),
        (1000000, "یەک ملیۆن"),
    ]

    # ـەم is the real Sorani ordinal marker. These are the consonant-final
    # stems, where gluing it on is already correct. See the module docs for
    # the vowel-final gap, which this change deliberately does not touch.
    ordinal_tests = [
        (1, "یەکەم"),
        (4, "چوارەم"),
        (5, "پێنجەم"),
        (6, "شەشەم"),
        (7, "حەوتەم"),
        (8, "هەشتەم"),
        (20, "بیستەم"),
        (40, "چلەم"),
        (100, "سەدەم"),
    ]

    ordinal_num_tests = [
        (1, "1ەم"),
        (11, "11ەم"),
        (102, "102ەم"),
    ]

    float_tests = [
        (12.5, "دوانزە خاڵ پێنج"),
        (-0.4, "نێگەتیڤ سفر خاڵ چوار"),
    ]

    negative_tests = [
        (-1, "نێگەتیڤ یەک"),
        (-100, "نێگەتیڤ سەد"),
    ]

    currency_tests = [
        (38.4, "سی و هەشت دینار", {"cents": False, "currency": "IQD"}),
        ("1.50", "یەک دینار پەنجا فلس", {"cents": True, "currency": "IQD"}),
        (12.34, "دوانزە دۆلار سی و چوار سەنت", {"currency": "USD"}),
        (12.34, "دوانزە یۆرۆ سی و چوار سەنت", {"currency": "EUR"}),
        # Unknown codes fall back to the first CURRENCY_FORMS entry (IQD).
        (12.34, "دوانزە دینار سی و چوار فلس", {"currency": "GBP"}),
    ]

    # to_year ignores longval and delegates to to_cardinal — no year pairing.
    year_tests = [
        (1990, "یەک هەزار و نۆ سەد و نەوەد"),
        (2017, "دوو هەزار و حەڤدە"),
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

    def test_ordinal_still_accepts_zero_and_negatives(self):
        # Upstream Num2Word_CKB never calls verify_ordinal; unchanged here.
        self.assertEqual(num2words(0, to="ordinal", lang="ckb"), "سفرەم")
        self.assertEqual(num2words(-1, to="ordinal", lang="ckb"), "نێگەتیڤ یەکەم")
        self.assertEqual(num2words(-1, to="ordinal_num", lang="ckb"), "-1ەم")

    def test_ckb_is_not_the_same_as_kurmanji(self):
        # ckb and ku both rendered Kurdish in Latin before this change, and
        # for some values the two were indistinguishable. ckb is Sorani in
        # the Perso-Arabic script; ku stays Kurmanji in Latin.
        self.assertNotEqual(num2words(42, lang="ckb"), num2words(42, lang="ku"))
        self.assertEqual(num2words(42, lang="ckb"), "چل و دوو")
        self.assertEqual(num2words(42, lang="ku"), "çil û du")


def test_ckb_is_written_in_sorani_script_not_transliteration():
    # lang_CKB.py spelled every numeral in Latin transliteration ("yek",
    # "hezar"). Central Kurdish is written in the Perso-Arabic Sorani
    # alphabet; the Latin alphabet is Kurmanji's (lang ku). Guard it.
    out = num2words(568476685, lang="ckb")
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "ملیۆن" in out
    assert "هەزار" in out
    assert num2words(1, lang="ckb") == "یەک"
    assert num2words(40, lang="ckb") == "چل"
    assert num2words(-1, lang="ckb").startswith("نێگەتیڤ")
    assert "خاڵ" in num2words(1.5, lang="ckb")
