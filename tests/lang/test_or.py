from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestOR(LangTest, TestCase):
    lang = "or"

    cardinal_tests = [
        (0, "ଶୂନ୍ୟ"),
        # Teens are lexicalised; 20..99 are composed as tens + ଓ + units.
        (11, "ଏଗାର"),
        (17, "ସତର"),
        (20, "କୋଡ଼ିଏ"),
        (21, "କୋଡ଼ିଏ ଓ ଏକ"),
        (42, "ଚାଳିଶ ଓ ଦୁଇ"),
        (100, "ଏକ ଶହ"),
        (101, "ଏକ ଶହ ଓ ଏକ"),
        (1000, "ଏକ ହଜାର"),
    ]

    # tens[7] used to be a verbatim duplicate of teens[7], so 70 and 17 both
    # rendered as ସତର. These three pin the fix.
    seventy_tests = [
        (17, "ସତର"),
        (70, "ସତୁରୀ"),
        (77, "ସତୁରୀ ଓ ସାତ"),
    ]

    # 1..10 are suppletive Sanskrit-derived forms; 11 and up take ମ.
    ordinal_tests = [
        (1, "ପ୍ରଥମ"),
        (2, "ଦ୍ୱିତୀୟ"),
        (3, "ତୃତୀୟ"),
        (4, "ଚତୁର୍ଥ"),
        (5, "ପଞ୍ଚମ"),
        (6, "ଷଷ୍ଠ"),
        (7, "ସପ୍ତମ"),
        (8, "ଅଷ୍ଟମ"),
        (9, "ନବମ"),
        (10, "ଦଶମ"),
        (11, "ଏଗାରମ"),
        (20, "କୋଡ଼ିଏମ"),
        (100, "ଏକ ଶହମ"),
    ]

    ordinal_num_tests = [
        (1, "1ମ"),
        (2, "2ମ"),
        (10, "10ମ"),
        (11, "11ମ"),
        (102, "102ମ"),
    ]

    float_tests = [
        (12.5, "ବାର ଦଶମିକ ପାଞ୍ଚ"),
        (12.51, "ବାର ଦଶମିକ ପାଞ୍ଚ ଏକ"),
        (-0.4, "ଋଣ ଶୂନ୍ୟ ଦଶମିକ ଚାରି"),
    ]

    negative_tests = [
        (-1, "ଋଣ ଏକ"),
        (-100, "ଋଣ ଏକ ଶହ"),
    ]

    currency_tests = [
        (38.4, "ତିରିଶ ଓ ଆଠ ଟଙ୍କା", {"cents": False, "currency": "INR"}),
        ("0", "ଶୂନ୍ୟ ଟଙ୍କା", {"cents": True, "currency": "INR"}),
        ("1.50", "ଏକ ଟଙ୍କା ପଚାଶ ପଇସା", {"cents": True, "currency": "INR"}),
        (12.34, "ବାର ଡଲାର ତିରିଶ ଓ ଚାରି ସେଣ୍ଟ", {"currency": "USD"}),
        (12.34, "ବାର ୟୁରୋ ତିରିଶ ଓ ଚାରି ସେଣ୍ଟ", {"currency": "EUR"}),
    ]

    # to_year ignores longval and delegates to to_cardinal — no year pairing.
    year_tests = [
        (1990, "ଏକ ହଜାର ନଅ ଶହ ଓ ନବେ"),
        (2017, "ଦୁଇ ହଜାର ସତର"),
        (1066, "ଏକ ହଜାର ଷାଠିଏ ଓ ଛଅ"),
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

    def test_seventy_is_distinct_from_seventeen(self):
        for num, expected in self.seventy_tests:
            with self.subTest(num=num):
                self.assertEqual(num2words(num, lang="or"), expected)

    def test_ordinal_still_accepts_zero_and_negatives(self):
        # Upstream Num2Word_OR never calls verify_ordinal, and this module
        # keeps that: only the 1..=10 forms changed, not the guard. The
        # 1..=10 test is on the signed value, so -1 takes the suffix arm.
        self.assertEqual(num2words(0, to="ordinal", lang="or"), "ଶୂନ୍ୟମ")
        self.assertEqual(num2words(-1, to="ordinal", lang="or"), "ଋଣ ଏକମ")
        self.assertEqual(num2words(-1, to="ordinal_num", lang="or"), "-1ମ")


def test_or_is_written_in_odia_not_transliteration():
    # lang_OR.py spelled every numeral in IAST transliteration ("eka",
    # "calīśa", "hajāra"). Odia is written in the Odia script; guard it.
    out = num2words(568476685, lang="or")
    assert all(c.isspace() or not c.isascii() for c in out), out
    assert "ଲକ୍ଷ" in out
    assert "ହଜାର" in out
    # A few forms a transliterated build would get wrong.
    assert num2words(1, lang="or") == "ଏକ"
    assert num2words(40, lang="or") == "ଚାଳିଶ"
    assert num2words(-1, lang="or").startswith("ଋଣ")
    assert "ଦଶମିକ" in num2words(1.5, lang="or")
