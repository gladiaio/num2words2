# -*- coding: utf-8 -*-
"""S'gaw Karen (ksw), in the S'gaw Karen script (gladiaio/num2words2#143).

Words from Omniglot "Numbers in Sgaw Karen" and Wiktionary's S'gaw Karen
numerals; composition after Gilmore, A Grammar of the Sgaw Karen (1898),
written as one word like the S'gaw Karen Common Bible (#263).
10^4 and up from Wiktionary and the S'gaw Karen Common Bible (#262), as
is the ordinal frame; zero, minus, decimal point and kyat are best
candidates (#262, UNVERIFIED in lang_ksw.rs).
"""
from __future__ import unicode_literals

from decimal import Decimal
from unittest import TestCase

from num2words2 import maxval, num2words


class Num2WordsKSWTest(TestCase):
    def test_units_and_tens(self):
        # Omniglot's table, 1..12 and the tens.
        expected = {
            1: "တ",
            2: "ခံ",
            3: "သၢ",
            4: "လွံၢ်",
            5: "ယဲၢ်",
            6: "ဃု",
            7: "နွံ",
            8: "ဃိး",
            9: "ခွံ",
            10: "တဆံ",
            11: "တဆံတၢ",
            12: "တဆံခံ",
            19: "တဆံခွံ",
            20: "ခံဆံ",
            30: "သၢဆံ",
            40: "လွံၢ်ဆံ",
            90: "ခွံဆံ",
        }
        for n, word in expected.items():
            self.assertEqual(num2words(n, lang="ksw"), word)

    def test_composition(self):
        self.assertEqual(num2words(21, lang="ksw"), "ခံဆံတၢ")
        self.assertEqual(num2words(42, lang="ksw"), "လွံၢ်ဆံခံ")
        self.assertEqual(num2words(50, lang="ksw"), "ယဲၢ်ဆံ")
        self.assertEqual(num2words(100, lang="ksw"), "တကယၤ")
        self.assertEqual(num2words(200, lang="ksw"), "ခံကယၤ")
        # #263: places are written as one word, as in the KSWC Bible
        # (Genesis 5:6 တကယၤယဲၢ် = 105, Genesis 50:26 တကယၤတဆံ = 110).
        self.assertEqual(num2words(101, lang="ksw"), "တကယၤတၢ")
        self.assertEqual(num2words(105, lang="ksw"), "တကယၤယဲၢ်")
        self.assertEqual(num2words(110, lang="ksw"), "တကယၤတဆံ")
        self.assertEqual(num2words(162, lang="ksw"), "တကယၤဃုဆံခံ")
        self.assertEqual(num2words(1000, lang="ksw"), "တကထိ")
        self.assertEqual(num2words(2024, lang="ksw"), "ခံကထိခံဆံလွံၢ်")
        self.assertEqual(num2words(9999, lang="ksw"), "ခွံကထိခွံကယၤခွံဆံခွံ")
        self.assertEqual(num2words(1984, lang="ksw", to="year"), "တကထိခွံကယၤဃိးဆံလွံၢ်")
        self.assertEqual(num2words(Decimal("5"), lang="ksw"), "ယဲၢ်")

    def test_ten_and_hundred_thousand(self):
        # #262: Wiktionary ကလး / ကလီၢ်, and the KSWC Bible's counts
        # (Psalm 91:7; Numbers 1:46 = 603,550; Numbers 26:51 = 601,730).
        self.assertEqual(num2words(10000, lang="ksw"), "တကလး")
        self.assertEqual(num2words(20000, lang="ksw"), "ခံကလး")
        self.assertEqual(num2words(100000, lang="ksw"), "တကလီၢ်")
        self.assertEqual(num2words(603550, lang="ksw"), "ဃုကလီၢ်သၢကထိယဲၢ်ကယၤယဲၢ်ဆံ")
        self.assertEqual(num2words(601730, lang="ksw"), "ဃုကလီၢ်တကထိနွံကယၤသၢဆံ")
        self.assertEqual(num2words(10001, lang="ksw"), "တကလးတၢ")
        self.assertEqual(
            num2words(999999, lang="ksw"),
            "ခွံကလီၢ်ခွံကလးခွံကထိခွံကယၤခွံဆံခွံ",
        )

    def test_million_and_up(self):
        # #262: KSWC 1 Chronicles 22:14 (တကကွဲၢ် = a thousand thousand) and
        # Revelation 9:16 (ကကွဲၢ်ခံကယၤ = 200,000,000).
        self.assertEqual(num2words(10**6, lang="ksw"), "တကကွဲၢ်")
        self.assertEqual(num2words(1000001, lang="ksw"), "တကကွဲၢ်တၢ")
        self.assertEqual(num2words(3500000, lang="ksw"), "သၢကကွဲၢ်ယဲၢ်ကလီၢ်")
        self.assertEqual(num2words(2 * 10**8, lang="ksw"), "ကကွဲၢ်ခံကယၤ")
        self.assertEqual(num2words(10**7, lang="ksw"), "ကကွဲၢ်တဆံ")
        # The space keeps 205,000,000 and 200,000,005 apart.
        self.assertEqual(num2words(205000000, lang="ksw"), "ကကွဲၢ်ခံကယၤယဲၢ်")
        self.assertEqual(num2words(200000005, lang="ksw"), "ကကွဲၢ်ခံကယၤ ယဲၢ်")

    def test_ceiling(self):
        self.assertEqual(maxval("ksw"), 10**12)
        for n in (10**12, -(10**12), 10**21):
            with self.assertRaises(OverflowError):
                num2words(n, lang="ksw")

    def test_zero_minus_decimal(self):
        # #262, best candidates (UNVERIFIED): Burmese/Pali သုည, အနုတ်, ဒသမ.
        self.assertEqual(num2words(0, lang="ksw"), "သုည")
        for x in (-3, "-3", Decimal("-3")):
            self.assertEqual(num2words(x, lang="ksw"), "အနုတ် သၢ")
        for x in (1.5, "1.5", Decimal("1.5")):
            self.assertEqual(num2words(x, lang="ksw"), "တ ဒသမ ယဲၢ်")
        self.assertEqual(num2words(1.05, lang="ksw"), "တ ဒသမ သုည ယဲၢ်")
        self.assertEqual(num2words(-0.5, lang="ksw"), "အနုတ် သုည ဒသမ ယဲၢ်")
        self.assertEqual(num2words(5.0, lang="ksw"), "ယဲၢ် ဒသမ သုည")

    def test_ordinals(self):
        # #262: the KSWC's N-CL one-CL frame (Genesis 1:8 မုၢ်ခံနံၤတနံၤ, the
        # second day; Revelation 21:20 တဆံတၢဖျၢၣ်တဖျၢၣ်, the eleventh stone)
        # with the generic classifier ခါ; "first" is အခီၣ်ထံး (Genesis 2:11).
        self.assertEqual(num2words(1, lang="ksw", to="ordinal"), "အခီၣ်ထံးတခါ")
        self.assertEqual(num2words(2, lang="ksw", to="ordinal"), "ခံခါတခါ")
        self.assertEqual(num2words(11, lang="ksw", to="ordinal"), "တဆံတၢခါတခါ")
        self.assertEqual(num2words(101, lang="ksw", to="ordinal"), "တကယၤတၢခါတခါ")
        self.assertEqual(num2words(3, lang="ksw", to="ordinal_num"), "3ခါတခါ")
        for to in ("ordinal", "ordinal_num"):
            with self.assertRaises(TypeError):
                num2words(-1, lang="ksw", to=to)

    def test_currency(self):
        # #262, best candidates (UNVERIFIED): kyat ကၠး, pya ပၠး.
        self.assertEqual(num2words(1, lang="ksw", to="currency"), "တ ကၠး")
        self.assertEqual(num2words(2.5, lang="ksw", to="currency"), "ခံ ကၠး ယဲၢ်ဆံ ပၠး")
        self.assertEqual(
            num2words(2.5, lang="ksw", to="currency", currency="MMK"),
            "ခံ ကၠး ယဲၢ်ဆံ ပၠး",
        )
        self.assertEqual(
            num2words(-2.5, lang="ksw", to="currency"), "အနုတ် ခံ ကၠး ယဲၢ်ဆံ ပၠး"
        )

    def test_unsupported_raise(self):
        # No agreed candidate for these currencies; cheque's shared format
        # is Latin ("AND").
        for code in ("USD", "EUR", "THB"):
            with self.assertRaises(NotImplementedError):
                num2words(2, lang="ksw", to="currency", currency=code)
        with self.assertRaises(NotImplementedError) as cm:
            num2words(3, lang="ksw", to="cheque")
        self.assertIn("lang='ksw'", str(cm.exception))

    def test_no_latin(self):
        for n in (0, -1, 1.5, 1, 7, 15, 99, 345, 6789, 10**9):
            self.assertNotRegex(num2words(n, lang="ksw"), "[A-Za-z0-9]")
