from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestKY(LangTest, TestCase):
    lang = "ky"

    cardinal_tests = [
        (0, "нөл"),
        (11, "он бир"),
        (22, "жыйырма эки"),
        (100, "жүз"),
        (101, "жүз бир"),
        (199, "жүз токсон тогуз"),
        (1000, "миң"),
        (2026, "эки миң жыйырма алты"),
        (100000, "жүз миң"),
        # No `m > 1` guard on the millions branch, so the leading "бир"
        # survives here where it is suppressed for жүз/миң.
        (1000000, "бир миллион"),
    ]

    # The ending is -(V)нч(V) and both vowels agree with the last vowel of
    # the stem; the linking vowel drops after a stem that ends in a vowel.
    ordinal_tests = [
        (1, "биринчи"),
        (2, "экинчи"),
        (3, "үчүнчү"),
        (4, "төртүнчү"),
        (5, "бешинчи"),
        (6, "алтынчы"),
        (7, "жетинчи"),
        (8, "сегизинчи"),
        (9, "тогузунчу"),
        (10, "онунчу"),
        (20, "жыйырманчы"),
        (30, "отузунчу"),
        (40, "кыркынчы"),
        (50, "элүүнчү"),
        (60, "алтымышынчы"),
        (70, "жетимишинчи"),
        (80, "сексенинчи"),
        (90, "токсонунчу"),
        (100, "жүзүнчү"),
        (1000, "миңинчи"),
        # Only the last word is inflected.
        (11, "он биринчи"),
        (123, "жүз жыйырма үчүнчү"),
        (2026, "эки миң жыйырма алтынчы"),
    ]

    # Digits, a hyphen, and the ending the spelled form would take.
    ordinal_num_tests = [
        (1, "1-инчи"),
        (3, "3-үнчү"),
        (6, "6-нчы"),
        (10, "10-унчу"),
        (40, "40-ынчы"),
        (50, "50-нчү"),
        (100, "100-үнчү"),
        (123, "123-үнчү"),
    ]

    float_tests = [
        (12.5, "он эки үтүр беш"),
        (12.51, "он эки үтүр беш бир"),
        (-0.4, "минус нөл үтүр төрт"),
    ]

    negative_tests = [
        (-1, "минус бир"),
        (-100, "минус жүз"),
        (-1999, "минус миң тогуз жүз токсон тогуз"),
    ]

    currency_tests = [
        (38.4, "отуз сегиз сом", {"cents": False, "currency": "KGS"}),
        ("0", "нөл сом", {"cents": True, "currency": "KGS"}),
        ("1.50", "бир сом элүү тыйын", {"cents": True, "currency": "KGS"}),
        (12.34, "он эки доллар отуз төрт сент", {"currency": "USD"}),
        (12.34, "он эки рубль отуз төрт копейка", {"currency": "RUB"}),
        # Unknown codes fall back to the first entry in CURRENCY_FORMS (KGS).
        (12.34, "он эки сом отуз төрт тыйын", {"currency": "GBP"}),
    ]

    # to_year ignores longval and delegates to to_cardinal, so there is no
    # year pairing: 1990 is the plain cardinal, not "nineteen ninety".
    year_tests = [
        (1990, "миң тогуз жүз токсон"),
        (2017, "эки миң он жети"),
        (1066, "миң алтымыш алты"),
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
        # Upstream Num2Word_KY never calls verify_ordinal, and this module
        # keeps that: only the ending changed, not the guard.
        self.assertEqual(num2words(0, to="ordinal", lang="ky"), "нөлүнчү")
        self.assertEqual(num2words(-1, to="ordinal", lang="ky"), "минус биринчи")
        self.assertEqual(num2words(-1, to="ordinal_num", lang="ky"), "-1-инчи")


def test_ky_is_written_in_cyrillic_not_transliteration():
    # lang_KY.py spelled every numeral in Latin transliteration ("bir",
    # "jüz", "jıyırma"). Kyrgyz is written in Cyrillic; guard the regression.
    out = num2words(568476685, lang="ky")
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "миллион" in out
    assert "миң" in out
    # A few forms a transliterated build would get wrong.
    assert num2words(1, lang="ky") == "бир"
    assert num2words(40, lang="ky") == "кырк"
    assert num2words(1000, lang="ky") == "миң"
    assert num2words(-1, lang="ky").startswith("минус")
    # pointword was the mixed-script "üтүр" (Latin u-umlaut + Cyrillic).
    assert "үтүр" in num2words(1.5, lang="ky")
    assert "ü" not in num2words(1.5, lang="ky")
