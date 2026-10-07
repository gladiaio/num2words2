# -*- coding: utf-8 -*-
"""S'gaw Karen (ksw), in the S'gaw Karen script (gladiaio/num2words2#143).

Words from Omniglot "Numbers in Sgaw Karen" and Wiktionary's S'gaw Karen
numerals; composition after Gilmore, A Grammar of the Sgaw Karen (1898).
10^4 and 10^5 from Wiktionary and the S'gaw Karen Common Bible (#262).
Only whole numbers 1..999999 have verified words; the rest raises.
"""
from __future__ import unicode_literals

from decimal import Decimal
from unittest import TestCase

from num2words2 import maxval, num2words


class Num2WordsKSWTest(TestCase):
    def test_units_and_tens(self):
        # Omniglot's table, 1..12 and the tens.
        expected = {
            1: "တ", 2: "ခံ", 3: "သၢ", 4: "လွံၢ်", 5: "ယဲၢ်", 6: "ဃု",
            7: "နွံ", 8: "ဃိး", 9: "ခွံ", 10: "တဆံ", 11: "တဆံတၢ",
            12: "တဆံခံ", 19: "တဆံခွံ", 20: "ခံဆံ", 30: "သၢဆံ",
            40: "လွံၢ်ဆံ", 90: "ခွံဆံ",
        }
        for n, word in expected.items():
            self.assertEqual(num2words(n, lang="ksw"), word)

    def test_composition(self):
        self.assertEqual(num2words(21, lang="ksw"), "ခံဆံတၢ")
        self.assertEqual(num2words(42, lang="ksw"), "လွံၢ်ဆံခံ")
        self.assertEqual(num2words(50, lang="ksw"), "ယဲၢ်ဆံ")
        self.assertEqual(num2words(100, lang="ksw"), "တကယၤ")
        self.assertEqual(num2words(200, lang="ksw"), "ခံကယၤ")
        self.assertEqual(num2words(101, lang="ksw"), "တကယၤ တၢ")
        self.assertEqual(num2words(1000, lang="ksw"), "တကထိ")
        self.assertEqual(num2words(2024, lang="ksw"), "ခံကထိ ခံဆံလွံၢ်")
        self.assertEqual(
            num2words(9999, lang="ksw"), "ခွံကထိ ခွံကယၤ ခွံဆံခွံ"
        )
        self.assertEqual(
            num2words(1984, lang="ksw", to="year"), "တကထိ ခွံကယၤ ဃိးဆံလွံၢ်"
        )
        self.assertEqual(num2words(Decimal("5"), lang="ksw"), "ယဲၢ်")

    def test_ten_and_hundred_thousand(self):
        # #262: Wiktionary ကလး / ကလီၢ်, and the KSWC Bible's counts
        # (Psalm 91:7; Numbers 1:46 = 603,550; Numbers 26:51 = 601,730).
        self.assertEqual(num2words(10000, lang="ksw"), "တကလး")
        self.assertEqual(num2words(20000, lang="ksw"), "ခံကလး")
        self.assertEqual(num2words(100000, lang="ksw"), "တကလီၢ်")
        self.assertEqual(
            num2words(603550, lang="ksw"), "ဃုကလီၢ် သၢကထိ ယဲၢ်ကယၤ ယဲၢ်ဆံ"
        )
        self.assertEqual(
            num2words(601730, lang="ksw"), "ဃုကလီၢ် တကထိ နွံကယၤ သၢဆံ"
        )
        self.assertEqual(num2words(10001, lang="ksw"), "တကလး တၢ")
        self.assertEqual(
            num2words(999999, lang="ksw"),
            "ခွံကလီၢ် ခွံကလး ခွံကထိ ခွံကယၤ ခွံဆံခွံ",
        )

    def test_ceiling(self):
        # No place word above 10^5 is attested twice (#262).
        self.assertEqual(maxval("ksw"), 10**6)
        for n in (10**6, -(10**6), 10**9, 10**21):
            with self.assertRaises(OverflowError):
                num2words(n, lang="ksw")

    def test_unverified_words_raise(self):
        cases = [
            (0, "cardinal"), (-1, "cardinal"), (1.5, "cardinal"),
            ("1.5", "cardinal"), (Decimal("1.5"), "cardinal"),
            (5.0, "cardinal"), (3, "ordinal"), (3, "ordinal_num"),
            (3, "currency"), (3, "cheque"), (0, "year"),
        ]
        for x, to in cases:
            with self.subTest(x=x, to=to):
                with self.assertRaises(NotImplementedError) as cm:
                    num2words(x, lang="ksw", to=to)
                self.assertIn("lang='ksw'", str(cm.exception))

    def test_no_latin(self):
        for n in (1, 7, 15, 99, 345, 6789):
            self.assertNotRegex(num2words(n, lang="ksw"), "[A-Za-z0-9]")
