# -*- coding: utf-8 -*-
# Author: Mehedi Hasan Khondoker
# Email: mehedihasankhondoker [at] gmail.com
# Copyright (c) 2024, Mehedi Hasan Khondoker.  All Rights Reserved.

# This library is build for Bangladesh format Number to Word conversion.
# You are welcome as contributor to the library.

# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Lesser General Public License for more details.

from __future__ import unicode_literals

from decimal import Decimal
from unittest import TestCase

from num2words2 import num2words


class Num2WordsBNTest(TestCase):
    maxDiff = None

    def test_negative(self):
        self.assertEqual(num2words(-1, lang="bn"), "ঋণাত্মক এক")

    def test_0(self):
        self.assertEqual(num2words(0, lang="bn"), "শূন্য")

    def test_1_to_10(self):
        self.assertEqual(num2words(1, lang="bn"), "এক")
        self.assertEqual(num2words(2, lang="bn"), "দুই")
        self.assertEqual(num2words(7, lang="bn"), "সাত")
        self.assertEqual(num2words(10, lang="bn"), "দশ")

    def test_11_to_19(self):
        self.assertEqual(num2words(11, lang="bn"), "এগারো")
        self.assertEqual(num2words(13, lang="bn"), "তেরো")
        self.assertEqual(num2words(15, lang="bn"), "পনের")
        self.assertEqual(num2words(16, lang="bn"), "ষোল")
        self.assertEqual(num2words(19, lang="bn"), "উনিশ")

    def test_20_to_99(self):
        self.assertEqual(num2words(20, lang="bn"), "বিশ")
        self.assertEqual(num2words(23, lang="bn"), "তেইশ")
        self.assertEqual(num2words(28, lang="bn"), "আটাশ")
        self.assertEqual(num2words(31, lang="bn"), "একত্রিশ")
        self.assertEqual(num2words(40, lang="bn"), "চল্লিশ")
        self.assertEqual(num2words(66, lang="bn"), "ছিষট্টি")
        self.assertEqual(num2words(92, lang="bn"), "বিরানব্বই")

    def test_100_to_999(self):
        self.assertEqual(num2words(100, lang="bn"), "একশত")
        self.assertEqual(num2words(111, lang="bn"), "একশত এগারো")
        self.assertEqual(num2words(150, lang="bn"), "একশত পঞ্চাশ")
        self.assertEqual(num2words(196, lang="bn"), "একশত ছিয়ানব্বই")
        self.assertEqual(num2words(200, lang="bn"), "দুইশত")
        self.assertEqual(num2words(210, lang="bn"), "দুইশত দশ")
        self.assertEqual(num2words(701, lang="bn"), "সাতশত এক")
        self.assertEqual(num2words(999, lang="bn"), "নয়শত নিরানব্বই")

    def test_1000_to_9999(self):
        self.assertEqual(num2words(1000, lang="bn"), "এক হাজার")
        self.assertEqual(num2words(1001, lang="bn"), "এক হাজার এক")
        self.assertEqual(num2words(1002, lang="bn"), "এক হাজার দুই")
        self.assertEqual(num2words(1010, lang="bn"), "এক হাজার দশ")
        self.assertEqual(num2words(1110, lang="bn"), "এক হাজার একশত দশ")
        self.assertEqual(num2words(1111, lang="bn"), "এক হাজার একশত এগারো")
        self.assertEqual(num2words(1500, lang="bn"), "এক হাজার পাঁচশত")
        self.assertEqual(num2words(2000, lang="bn"), "দুই হাজার")
        self.assertEqual(num2words(2042, lang="bn"), "দুই হাজার বিয়াল্লিশ")
        self.assertEqual(num2words(3000, lang="bn"), "তিন হাজার")
        self.assertEqual(num2words(3301, lang="bn"), "তিন হাজার তিনশত এক")
        self.assertEqual(num2words(3108, lang="bn"), "তিন হাজার একশত আট")
        self.assertEqual(num2words(6870, lang="bn"), "ছয় হাজার আটশত সত্তর")
        self.assertEqual(num2words(7378, lang="bn"), "সাত হাজার তিনশত আটাত্তর")
        self.assertEqual(num2words(9999, lang="bn"), "নয় হাজার নয়শত নিরানব্বই")

    def test_10000_to_99999(self):
        self.assertEqual(num2words(10000, lang="bn"), "দশ হাজার")
        self.assertEqual(num2words(10501, lang="bn"), "দশ হাজার পাঁচশত এক")
        self.assertEqual(num2words(10999, lang="bn"), "দশ হাজার নয়শত নিরানব্বই")
        self.assertEqual(num2words(13000, lang="bn"), "তেরো হাজার")
        self.assertEqual(num2words(15333, lang="bn"), "পনের হাজার তিনশত তেত্রিশ")
        self.assertEqual(num2words(21111, lang="bn"), "একুশ হাজার একশত এগারো")
        self.assertEqual(num2words(21003, lang="bn"), "একুশ হাজার তিন")
        self.assertEqual(num2words(25020, lang="bn"), "পঁচিশ হাজার বিশ")
        self.assertEqual(num2words(68700, lang="bn"), "আটষট্টি হাজার সাতশত")
        self.assertEqual(num2words(73781, lang="bn"), "তিয়াত্তর হাজার সাতশত একাশি")
        self.assertEqual(num2words(99999, lang="bn"), "নিরানব্বই হাজার নয়শত নিরানব্বই")

    def test_100000_to_999999(self):
        self.assertEqual(num2words(100000, lang="bn"), "এক লাখ")
        self.assertEqual(num2words("100000", lang="bn"), "এক লাখ")
        self.assertEqual(
            num2words(199999, lang="bn"), "এক লাখ নিরানব্বই হাজার নয়শত নিরানব্বই"
        )
        self.assertEqual(num2words(110000, lang="bn"), "এক লাখ দশ হাজার")
        self.assertEqual(num2words(150010, lang="bn"), "এক লাখ পঞ্চাশ হাজার দশ")
        self.assertEqual(num2words("200200", lang="bn"), "দুই লাখ দুইশত")
        self.assertEqual(
            num2words(737812, lang="bn"), "সাত লাখ সাতত্রিশ হাজার আটশত বারো"
        )
        self.assertEqual(
            num2words("999999", lang="bn"), "নয় লাখ নিরানব্বই হাজার নয়শত নিরানব্বই"
        )

    def test_1000000_to_9999999999999999(self):
        self.assertEqual(num2words(1000000, lang="bn"), "দশ লাখ")
        self.assertEqual(num2words(20000000, lang="bn"), "দুই কোটি")
        self.assertEqual(num2words(300000000, lang="bn"), "ত্রিশ কোটি")
        self.assertEqual(num2words(4000000000, lang="bn"), "চারশত কোটি")
        self.assertEqual(num2words(50000000000, lang="bn"), "পাঁচ হাজার কোটি")
        self.assertEqual(num2words(600000000000, lang="bn"), "ষাট হাজার কোটি")
        self.assertEqual(num2words(7000000000000, lang="bn"), "সাত লাখ কোটি")
        self.assertEqual(num2words(80000000000000, lang="bn"), "আশি লাখ কোটি")
        self.assertEqual(num2words(900000000000000, lang="bn"), "নয় কোটি কোটি")
        self.assertEqual(
            num2words(999999999999999, lang="bn"),
            "নয় কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই",
        )  # noqa: E501
        self.assertEqual(
            num2words(9999999999999999, lang="bn"),
            "নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই",
        )  # noqa: E501

    def test_dosomik_0_to_999999999999999999(self):
        self.assertEqual(num2words(0.56, lang="bn"), "শূন্য দশমিক পাঁচ ছয়")
        self.assertEqual(num2words(1.11, lang="bn"), "এক দশমিক এক এক")
        self.assertEqual(num2words(2.66, lang="bn"), "দুই দশমিক ছয় ছয়")
        self.assertEqual(num2words(7.68, lang="bn"), "সাত দশমিক ছয় আট")
        self.assertEqual(num2words("10.35", lang="bn"), "দশ দশমিক তিন পাঁচ")
        self.assertEqual(num2words("11.47", lang="bn"), "এগারো দশমিক চার সাত")
        self.assertEqual(num2words(13.69, lang="bn"), "তেরো দশমিক ছয় নয়")
        self.assertEqual(num2words(15.96, lang="bn"), "পনের দশমিক নয় ছয়")
        self.assertEqual(num2words(16.9999, lang="bn"), "ষোল দশমিক নয় নয় নয় নয়")
        self.assertEqual(
            num2words(19.56587, lang="bn"), "উনিশ দশমিক পাঁচ ছয় পাঁচ আট সাত"
        )
        self.assertEqual(num2words(31.31, lang="bn"), "একত্রিশ দশমিক তিন এক")
        self.assertEqual(num2words(40.85, lang="bn"), "চল্লিশ দশমিক আট পাঁচ")
        self.assertEqual(num2words(66.66, lang="bn"), "ছিষট্টি দশমিক ছয় ছয়")
        self.assertEqual(num2words(92.978, lang="bn"), "বিরানব্বই দশমিক নয় সাত আট")
        self.assertEqual(num2words(1000001.10, lang="bn"), "দশ লাখ এক দশমিক এক")
        self.assertEqual(num2words(20000000.22, lang="bn"), "দুই কোটি দশমিক দুই দুই")
        self.assertEqual(
            num2words(300030000.33, lang="bn"), "ত্রিশ কোটি ত্রিশ হাজার দশমিক তিন তিন"
        )
        self.assertEqual(
            num2words("4004000444.44", lang="bn"),
            "চারশত কোটি চল্লিশ লাখ চারশত চৌচল্লিশ দশমিক চার চার",
        )
        self.assertEqual(
            num2words(50000000001.50, lang="bn"), "পাঁচ হাজার কোটি এক দশমিক পাঁচ"
        )
        self.assertEqual(
            num2words(600000000600.66, lang="bn"), "ষাট হাজার কোটি ছয়শত দশমিক ছয় ছয়"
        )
        self.assertEqual(
            num2words(7000000000000.77, lang="bn"), "সাত লাখ কোটি দশমিক সাত সাত"
        )
        self.assertEqual(
            num2words(80000000000888.88, lang="bn"),
            "আশি লাখ কোটি আটশত আটাশি দশমিক আট আট",
        )
        self.assertEqual(
            num2words(900000000000000.9, lang="bn"), "নয় কোটি কোটি দশমিক নয়"
        )
        self.assertEqual(
            num2words(999999999999999.9, lang="bn"),
            "নয় কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই দশমিক নয়",
        )  # noqa: E501
        self.assertEqual(num2words(9999999999999999.99, lang="bn"), "একশত কোটি কোটি")
        self.assertEqual(
            num2words(99999999999999999.99, lang="bn"), "এক হাজার কোটি কোটি"
        )
        self.assertEqual(
            num2words("999999999999999999.9", lang="bn"),
            "নয় হাজার নয়শত নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই কোটি নিরানব্বই লাখ নিরানব্বই হাজার নয়শত নিরানব্বই দশমিক নয়",
        )  # noqa: E501

    def test_ordinal_words_250(self):
        # to='ordinal' returned the cardinal (#250).
        for value, word in [
            (1, "প্রথম"),
            (2, "দ্বিতীয়"),
            (3, "তৃতীয়"),
            (4, "চতুর্থ"),
            (5, "পঞ্চম"),
            (6, "ষষ্ঠ"),
            (10, "দশম"),
            (11, "এগারোতম"),
            (21, "একুশতম"),
            (100, "একশততম"),
            (5.0, "পঞ্চম"),
        ]:
            self.assertEqual(num2words(value, lang="bn", to="ordinal"), word)
        with self.assertRaises(TypeError):
            num2words(-1, lang="bn", to="ordinal")

    def test_ordinal_num_abbreviations_250(self):
        for value, abbr in [
            (1, "১ম"),
            (2, "২য়"),
            (3, "৩য়"),
            (4, "৪র্থ"),
            (5, "৫ম"),
            (6, "৬ষ্ঠ"),
            (7, "৭ম"),
            (10, "১০ম"),
            (11, "১১তম"),
            (21, "২১তম"),
            (100, "১০০তম"),
        ]:
            self.assertEqual(num2words(value, lang="bn", to="ordinal_num"), abbr)

    def test_negative_year_keeps_the_era_250(self):
        self.assertEqual(num2words(44, lang="bn", to="year"), "চৌচল্লিশ সাল")
        self.assertEqual(
            num2words(-44, lang="bn", to="year"), "খ্রিস্টপূর্ব চৌচল্লিশ সাল"
        )
        self.assertEqual(
            num2words(-44.0, lang="bn", to="year"), "খ্রিস্টপূর্ব চৌচল্লিশ সাল"
        )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="bn"), "ঋণাত্মক শূন্য দশমিক চার")
        self.assertEqual(num2words(-0.5, lang="bn"), "ঋণাত্মক শূন্য দশমিক পাঁচ")
        self.assertEqual(num2words(-1.4, lang="bn"), "ঋণাত্মক এক দশমিক চার")


def test_bn_currency_single_digit_paisa():
    # gladiaio/num2words2#255: 3.05 is 5 paisa, not 50
    for v in (3.05, "3.05", Decimal("3.05")):
        assert num2words(v, lang="bn", to="currency") == "তিন টাকা পাঁচ পয়সা"
    assert num2words(3.5, lang="bn", to="currency") == "তিন টাকা পঞ্চাশ পয়সা"
    assert num2words(3.07, lang="bn", to="currency") == "তিন টাকা সাত পয়সা"
    assert num2words(0.01, lang="bn", to="currency") == "শূন্য টাকা এক পয়সা"
