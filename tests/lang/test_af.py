from unittest import TestCase

from num2words2 import num2words
from tests.basetest import LangTest


class TestAF(LangTest, TestCase):  # Inherit from LangTest and TestCase
    lang = "af"

    cardinal_tests = [
        # test_cardinal_basic_numbers
        (0, "nul"),
        (1, "een"),
        (2, "twee"),
        (3, "drie"),
        (4, "vier"),
        (5, "vyf"),
        (6, "ses"),
        (7, "sewe"),
        (8, "agt"),
        (9, "nege"),
        (10, "tien"),
        (11, "elf"),
        (12, "twaalf"),
        (13, "dertien"),
        (14, "veertien"),
        (15, "vyftien"),
        (16, "sestien"),
        (17, "sewentien"),
        (18, "agttien"),
        (19, "negentien"),
        (20, "twintig"),
        # test_cardinal_tens
        (20, "twintig"),
        (30, "dertig"),
        (40, "veertig"),
        (50, "vyftig"),
        (60, "sestig"),
        (70, "sewentig"),
        (80, "tagtig"),
        (90, "negentig"),
        # test_cardinal_compound_numbers
        (21, "een-en-twintig"),
        (34, "vier-en-dertig"),
        (56, "ses-en-vyftig"),
        (87, "sewe-en-tagtig"),
        (99, "nege-en-negentig"),
        # test_cardinal_hundreds
        (100, "een honderd"),
        (200, "twee honderd"),
        (300, "drie honderd"),
        (500, "vyf honderd"),
        (900, "nege honderd"),
        # test_cardinal_hundreds_with_compound
        (101, "een honderd een"),
        (125, "een honderd vyf-en-twintig"),
        (234, "twee honderd vier-en-dertig"),
        (456, "vier honderd ses-en-vyftig"),
        (789, "sewe honderd nege-en-tagtig"),
        # test_cardinal_thousands
        (1000, "een duisend"),
        (2000, "twee duisend"),
        (5000, "vyf duisend"),
        (10000, "tien duisend"),
        (11000, "elf duisend"),
        (100000, "een honderd duisend"),
        # test_cardinal_thousands_with_compound
        (1001, "een duisend een"),
        (1234, "een duisend twee honderd vier-en-dertig"),
        (12345, "twaalf duisend drie honderd vyf-en-veertig"),
        # test_cardinal_millions
        (1000000, "een miljoen"),
        (2000000, "twee miljoen"),
        (5000000, "vyf miljoen"),
        # test_cardinal_large_numbers
        (1000000000, "een miljard"),
        (2000000000, "twee miljard"),
        (1000000000000, "een biljoen"),
        # test_edge_cases (very large number)
        (
            999999999,
            "nege honderd nege-en-negentig miljoen nege honderd nege-en-negentig duisend nege honderd nege-en-negentig",
        ),
    ]

    ordinal_tests = [
        # test_ordinal_basic_numbers
        (1, "eerste"),
        (2, "tweede"),
        (3, "derde"),
        (4, "vierde"),
        (5, "vyfde"),
        (6, "sesde"),
        (7, "sewende"),
        (8, "agste"),
        (9, "negende"),
        (10, "tiende"),
        # test_ordinal_teens
        (11, "elfde"),
        (12, "twaalfde"),
        (13, "dertiende"),
        (14, "veertiende"),
        (15, "vyftiende"),
        (16, "sestiende"),
        (17, "sewentiende"),
        (18, "agttiende"),
        (19, "negentiende"),
        (20, "twintigste"),
        # test_ordinal_tens
        (30, "dertigste"),
        (40, "veertigste"),
        (50, "vyftigste"),
        (60, "sestigste"),
        (70, "sewentigste"),
        (80, "tagtigste"),
        (90, "negentigste"),
        # test_ordinal_hundreds_and_larger
        (100, "honderdste"),  # not "een honderdste" (#259)
        (1000, "duisendste"),
        (1000000, "miljoenste"),
        # test_ordinal_compound_numbers
        (21, "een-en-twintigste"),
        (34, "vier-en-dertigste"),
        (101, "een honderd eerste"),
        # test_special_ordinal_cases
        (0, "nullde"),
    ]

    ordinal_num_tests = [
        (1, "1ste"),
        (2, "2de"),
        (3, "3de"),
        (4, "4de"),
        (21, "21ste"),
        (22, "22ste"),
        (23, "23ste"),
    ]

    float_tests = [
        (3.14, "drie komma een vier"),
        (0.5, "nul komma vyf"),
        (12.34, "twaalf komma drie vier"),
        (123.456, "een honderd drie-en-twintig komma vier vyf ses"),
        # test_negative_decimals
        (-0.4, "minus nul komma vier"),
        (-0.5, "minus nul komma vyf"),
        (-0.04, "minus nul komma nul vier"),
        (-1.4, "minus een komma vier"),
        (-10.25, "minus tien komma twee vyf"),
    ]

    negative_tests = [
        (-1, "minus een"),
        (-12, "minus twaalf"),
        (-100, "minus een honderd"),
        (-1234, "minus een duisend twee honderd vier-en-dertig"),
    ]

    currency_tests = [
        # TEST_CASES_TO_CURRENCY_ZAR
        (1.00, "een rand en nul sent", {"currency": "ZAR"}),
        (2.01, "twee rand en een sent", {"currency": "ZAR"}),
        (8.10, "agt rand en tien sent", {"currency": "ZAR"}),
        (12.26, "twaalf rand en ses-en-twintig sent", {"currency": "ZAR"}),
        (21.29, "een-en-twintig rand en nege-en-twintig sent", {"currency": "ZAR"}),
        (81.25, "een-en-tagtig rand en vyf-en-twintig sent", {"currency": "ZAR"}),
        (100.00, "een honderd rand en nul sent", {"currency": "ZAR"}),
        # TEST_CASES_TO_CURRENCY_EUR
        (1.00, "een euro en nul sent", {"currency": "EUR"}),
        (2.01, "twee euro en een sent", {"currency": "EUR"}),
        (8.10, "agt euro en tien sent", {"currency": "EUR"}),
        (12.26, "twaalf euro en ses-en-twintig sent", {"currency": "EUR"}),
        (21.29, "een-en-twintig euro en nege-en-twintig sent", {"currency": "EUR"}),
        (81.25, "een-en-tagtig euro en vyf-en-twintig sent", {"currency": "EUR"}),
        (100.00, "een honderd euro en nul sent", {"currency": "EUR"}),
        # TEST_CASES_TO_CURRENCY_USD
        (1.00, "een dollar en nul sent", {"currency": "USD"}),
        (2.01, "twee dollar en een sent", {"currency": "USD"}),
        (8.10, "agt dollar en tien sent", {"currency": "USD"}),
        (12.26, "twaalf dollar en ses-en-twintig sent", {"currency": "USD"}),
        (21.29, "een-en-twintig dollar en nege-en-twintig sent", {"currency": "USD"}),
        (81.25, "een-en-tagtig dollar en vyf-en-twintig sent", {"currency": "USD"}),
        (100.00, "een honderd dollar en nul sent", {"currency": "USD"}),
        # test_currency_with_no_cents
        (100, "een honderd rand", {"currency": "ZAR", "cents": False}),
        (50.25, "vyftig euro en 25 sent", {"currency": "EUR", "cents": False}),
        # test_currency_with_different_separator
        (
            15.75,
            "vyftien dollar plus  vyf-en-sewentig sent",
            {"currency": "USD", "separator": " plus "},
        ),
    ]

    year_tests = [
        (2023, "twintig drie-en-twintig"),
        (1999, "negentien nege-en-negentig"),
        (2000, "twee duisend"),
        (2001, "twee duisend een"),
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

    # Specific tests that don't fit the generic structure
    def test_ordinal_negative_numbers_raise_error(self):
        self.assertRaises(TypeError, num2words, -1, ordinal=True, lang=self.lang)
        self.assertRaises(TypeError, num2words, -12, ordinal=True, lang=self.lang)

    def test_ordinal_float_numbers_raise_error(self):
        self.assertRaises(TypeError, num2words, 3.14, ordinal=True, lang=self.lang)
        self.assertRaises(TypeError, num2words, 0.5, ordinal=True, lang=self.lang)
