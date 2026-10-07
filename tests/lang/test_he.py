# -*- coding: utf-8 -*-
# Copyright (c) 2003, Taro Ogawa.  All Rights Reserved.
# Copyright (c) 2013, Savoir-faire Linux inc.  All Rights Reserved.

# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Lesser General Public License for more details.
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
# MA 02110-1301 USA

from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsHETest(TestCase):
    maxDiff = None

    def test_negative(self):
        self.assertEqual(num2words(-1, lang="he"), "מינוס אחת")

    def test_0(self):
        self.assertEqual(num2words(0, lang="he"), "אפס")

    def test_1_to_10(self):
        self.assertEqual(num2words(1, lang="he"), "אחת")
        self.assertEqual(num2words(2, lang="he"), "שתיים")
        self.assertEqual(num2words(7, lang="he"), "שבע")
        self.assertEqual(num2words(10, lang="he"), "עשר")
        self.assertEqual(num2words(10, lang="he", gender="m", construct=True), "עשרת")

    def test_11_to_19(self):
        self.assertEqual(num2words(11, lang="he"), "אחת עשרה")
        self.assertEqual(num2words(11, lang="he", gender="m"), "אחד עשר")
        self.assertEqual(num2words(13, lang="he"), "שלוש עשרה")
        self.assertEqual(num2words(13, lang="he", construct=True), "שלוש עשרה")
        self.assertEqual(num2words(15, lang="he"), "חמש עשרה")
        self.assertEqual(num2words(16, lang="he"), "שש עשרה")
        self.assertEqual(num2words(16, lang="he", gender="m"), "שישה עשר")
        self.assertEqual(
            num2words(16, lang="he", gender="m", construct=True), "שישה עשר"
        )
        self.assertEqual(num2words(19, lang="he"), "תשע עשרה")

    def test_20_to_99(self):
        self.assertEqual(num2words(20, lang="he"), "עשרים")
        self.assertEqual(num2words(23, lang="he"), "עשרים ושלוש")
        self.assertEqual(num2words(23, lang="he", gender="m"), "עשרים ושלושה")
        self.assertEqual(num2words(23, lang="he", construct=True), "עשרים ושלוש")
        self.assertEqual(
            num2words(23, lang="he", gender="m", construct=True), "עשרים ושלושה"
        )
        self.assertEqual(num2words(28, lang="he"), "עשרים ושמונה")
        self.assertEqual(num2words(31, lang="he"), "שלושים ואחת")
        self.assertEqual(num2words(40, lang="he"), "ארבעים")
        self.assertEqual(num2words(66, lang="he"), "שישים ושש")
        self.assertEqual(num2words(92, lang="he"), "תשעים ושתיים")

    def test_100_to_999(self):
        self.assertEqual(num2words(100, lang="he"), "מאה")
        self.assertEqual(num2words(100, lang="he", construct=True), "מאת")
        self.assertEqual(num2words(111, lang="he"), "מאה ואחת עשרה")
        self.assertEqual(num2words(111, lang="he", construct=True), "מאה ואחת עשרה")
        self.assertEqual(num2words(150, lang="he"), "מאה וחמישים")
        self.assertEqual(num2words(196, lang="he"), "מאה תשעים ושש")
        self.assertEqual(num2words(196, lang="he", gender="m"), "מאה תשעים ושישה")
        self.assertEqual(
            num2words(196, lang="he", gender="m", construct=True), "מאה תשעים ושישה"
        )
        self.assertEqual(num2words(200, lang="he"), "מאתיים")
        self.assertEqual(num2words(200, lang="he", construct=True), "מאתיים")
        self.assertEqual(num2words(210, lang="he"), "מאתיים ועשר")
        self.assertEqual(num2words(701, lang="he"), "שבע מאות ואחת")

    def test_1000_to_9999(self):
        self.assertEqual(num2words(1000, lang="he"), "אלף")
        self.assertEqual(num2words(1000, lang="he", construct=True), "אלף")
        self.assertEqual(num2words(1001, lang="he"), "אלף ואחת")
        self.assertEqual(num2words(1002, lang="he"), "אלף ושתיים")
        self.assertEqual(num2words(1002, lang="he", gender="m"), "אלף ושניים")
        self.assertEqual(
            num2words(1002, lang="he", gender="m", construct=True), "אלף ושניים"
        )
        self.assertEqual(num2words(1003, lang="he"), "אלף ושלוש")
        self.assertEqual(num2words(1003, lang="he", gender="m"), "אלף ושלושה")
        self.assertEqual(
            num2words(1003, lang="he", gender="m", construct=True), "אלף ושלושה"
        )
        self.assertEqual(num2words(1010, lang="he"), "אלף ועשר")
        self.assertEqual(num2words(1010, lang="he", gender="m"), "אלף ועשרה")
        self.assertEqual(
            num2words(1010, lang="he", gender="m", construct=True), "אלף ועשרה"
        )
        self.assertEqual(num2words(1500, lang="he"), "אלף וחמש מאות")
        self.assertEqual(num2words(2000, lang="he"), "אלפיים")
        self.assertEqual(num2words(2000, lang="he", construct=True), "אלפיים")
        self.assertEqual(num2words(2002, lang="he"), "אלפיים ושתיים")
        self.assertEqual(num2words(2002, lang="he", construct=True), "אלפיים ושתיים")
        self.assertEqual(num2words(3000, lang="he"), "שלושת אלפים")
        self.assertEqual(num2words(3000, lang="he", construct=True), "שלושת אלפי")
        self.assertEqual(num2words(3001, lang="he"), "שלושת אלפים ואחת")
        self.assertEqual(num2words(3001, lang="he", construct=True), "שלושת אלפים ואחת")
        self.assertEqual(num2words(3100, lang="he"), "שלושת אלפים ומאה")
        self.assertEqual(num2words(3100, lang="he", construct=True), "שלושת אלפים ומאה")
        self.assertEqual(num2words(6870, lang="he"), "ששת אלפים שמונה מאות ושבעים")
        self.assertEqual(
            num2words(7378, lang="he"), "שבעת אלפים שלוש מאות שבעים ושמונה"
        )
        self.assertEqual(num2words(9999, lang="he"), "תשעת אלפים תשע מאות תשעים ותשע")

    def test_10000_to_99999(self):
        self.assertEqual(num2words(10000, lang="he"), "עשרת אלפים")
        self.assertEqual(num2words(10000, lang="he", construct=True), "עשרת אלפי")
        self.assertEqual(num2words(10001, lang="he"), "עשרת אלפים ואחת")
        self.assertEqual(num2words(10001, lang="he", construct=True), "עשרת אלפים ואחת")
        self.assertEqual(num2words(10999, lang="he"), "עשרת אלפים תשע מאות תשעים ותשע")
        self.assertEqual(num2words(11000, lang="he"), "אחד עשר אלף")
        self.assertEqual(num2words(15000, lang="he"), "חמישה עשר אלף")
        self.assertEqual(num2words(15000, lang="he", gender="m"), "חמישה עשר אלף")
        self.assertEqual(num2words(20000, lang="he"), "עשרים אלף")
        self.assertEqual(num2words(20000, lang="he", construct=True), "עשרים אלף")
        self.assertEqual(num2words(21000, lang="he"), "עשרים ואחד אלף")
        self.assertEqual(num2words(25000, lang="he"), "עשרים וחמישה אלף")
        self.assertEqual(
            num2words(25000, lang="he", construct=True), "עשרים וחמישה אלף"
        )
        self.assertEqual(num2words(68700, lang="he"), "שישים ושמונה אלף ושבע מאות")
        self.assertEqual(
            num2words(73781, lang="he"), "שבעים ושלושה אלף שבע מאות שמונים ואחת"
        )
        self.assertEqual(
            num2words(99999, lang="he"), "תשעים ותשעה אלף תשע מאות תשעים ותשע"
        )

    def test_100000_to_999999(self):
        self.assertEqual(num2words(100000, lang="he"), "מאה אלף")
        self.assertEqual(num2words(100000, lang="he", construct=True), "מאה אלף")
        self.assertEqual(num2words(100001, lang="he"), "מאה אלף ואחת")
        self.assertEqual(
            num2words(199999, lang="he"), "מאה תשעים ותשעה אלף תשע מאות תשעים ותשע"
        )
        self.assertEqual(num2words(110000, lang="he"), "מאה ועשרה אלף")
        self.assertEqual(num2words(150000, lang="he"), "מאה וחמישים אלף")
        self.assertEqual(num2words(200000, lang="he"), "מאתיים אלף")
        self.assertEqual(num2words(210000, lang="he"), "מאתיים ועשרה אלף")
        self.assertEqual(num2words(687000, lang="he"), "שש מאות שמונים ושבעה אלף")
        self.assertEqual(
            num2words(687000, lang="he", construct=True), "שש מאות שמונים ושבעה אלף"
        )
        self.assertEqual(
            num2words(737812, lang="he"),
            "שבע מאות שלושים ושבעה אלף שמונה מאות ושתים עשרה",
        )
        self.assertEqual(
            num2words(999999, lang="he"), "תשע מאות תשעים ותשעה אלף תשע מאות תשעים ותשע"
        )

    def test_1000000_to_999999999999999(self):
        self.assertEqual(num2words(1000000, lang="he"), "מיליון")
        self.assertEqual(num2words(1000000, lang="he", construct=True), "מיליון")
        self.assertEqual(num2words(1000002, lang="he"), "מיליון ושתיים")
        self.assertEqual(num2words(1000002, lang="he", construct=True), "מיליון ושתיים")
        self.assertEqual(num2words(2000000, lang="he"), "שני מיליון")
        self.assertEqual(num2words(2000000, lang="he", construct=True), "שני מיליוני")
        self.assertEqual(num2words(3000000, lang="he"), "שלושה מיליון")
        self.assertEqual(num2words(3000000, lang="he", construct=True), "שלושת מיליוני")
        self.assertEqual(num2words(3000002, lang="he"), "שלושה מיליון ושתיים")
        self.assertEqual(
            num2words(3000002, lang="he", construct=True), "שלושה מיליון ושתיים"
        )
        self.assertEqual(num2words(10000000, lang="he"), "עשרה מיליון")
        self.assertEqual(num2words(10000000, lang="he", construct=True), "עשרת מיליוני")
        self.assertEqual(num2words(11000000, lang="he"), "אחד עשר מיליון")
        self.assertEqual(
            num2words(11000000, lang="he", construct=True), "אחד עשר מיליוני"
        )

        self.assertEqual(num2words(1000000000, lang="he"), "מיליארד")
        self.assertEqual(num2words(1000000000, lang="he", construct=True), "מיליארד")
        self.assertEqual(num2words(1000000002, lang="he"), "מיליארד ושתיים")
        self.assertEqual(
            num2words(1000000002, lang="he", construct=True), "מיליארד ושתיים"
        )
        self.assertEqual(num2words(2000000000, lang="he"), "שני מיליארד")
        self.assertEqual(
            num2words(2000000000, lang="he", construct=True), "שני מיליארדי"
        )
        self.assertEqual(num2words(3000000000, lang="he"), "שלושה מיליארד")
        self.assertEqual(
            num2words(3000000000, lang="he", construct=True), "שלושת מיליארדי"
        )
        self.assertEqual(num2words(3000000002, lang="he"), "שלושה מיליארד ושתיים")
        self.assertEqual(
            num2words(3000000002, lang="he", construct=True), "שלושה מיליארד ושתיים"
        )
        self.assertEqual(num2words(10000000000, lang="he"), "עשרה מיליארד")
        self.assertEqual(
            num2words(10000000000, lang="he", construct=True), "עשרת מיליארדי"
        )
        self.assertEqual(num2words(10000000002, lang="he"), "עשרה מיליארד ושתיים")
        self.assertEqual(
            num2words(10000000002, lang="he", construct=True), "עשרה מיליארד ושתיים"
        )
        self.assertEqual(num2words(11000000000, lang="he"), "אחד עשר מיליארד")
        self.assertEqual(
            num2words(11000000000, lang="he", construct=True), "אחד עשר מיליארדי"
        )

        self.assertEqual(num2words(1000000000000, lang="he"), "טריליון")
        self.assertEqual(num2words(1000000000000, lang="he", construct=True), "טריליון")
        self.assertEqual(num2words(1000000000002, lang="he"), "טריליון ושתיים")
        self.assertEqual(
            num2words(1000000000002, lang="he", construct=True), "טריליון ושתיים"
        )
        self.assertEqual(num2words(2000000000000, lang="he"), "שני טריליון")
        self.assertEqual(
            num2words(2000000000000, lang="he", construct=True), "שני טריליוני"
        )
        self.assertEqual(num2words(3000000000000, lang="he"), "שלושה טריליון")
        self.assertEqual(
            num2words(3000000000000, lang="he", construct=True), "שלושת טריליוני"
        )
        self.assertEqual(num2words(3000000000002, lang="he"), "שלושה טריליון ושתיים")
        self.assertEqual(
            num2words(3000000000002, lang="he", construct=True), "שלושה טריליון ושתיים"
        )
        self.assertEqual(num2words(10000000000000, lang="he"), "עשרה טריליון")
        self.assertEqual(
            num2words(10000000000000, lang="he", construct=True), "עשרת טריליוני"
        )
        self.assertEqual(num2words(10000000000002, lang="he"), "עשרה טריליון ושתיים")
        self.assertEqual(
            num2words(10000000000002, lang="he", construct=True), "עשרה טריליון ושתיים"
        )
        self.assertEqual(num2words(11000000000000, lang="he"), "אחד עשר טריליון")
        self.assertEqual(
            num2words(11000000000000, lang="he", construct=True), "אחד עשר טריליוני"
        )

        self.assertEqual(
            num2words(999999999999999, lang="he"),
            "תשע מאות תשעים ותשעה טריליון "
            "תשע מאות תשעים ותשעה מיליארד "
            "תשע מאות תשעים ותשעה מיליון "
            "תשע מאות תשעים ותשעה אלף תשע מאות תשעים ותשע",
        )
        self.assertEqual(
            num2words(999999999999999, lang="he", gender="m"),
            "תשע מאות תשעים ותשעה טריליון "
            "תשע מאות תשעים ותשעה מיליארד "
            "תשע מאות תשעים ותשעה מיליון "
            "תשע מאות תשעים ותשעה אלף תשע מאות תשעים ותשעה",
        )
        self.assertEqual(
            num2words(999999999999999, lang="he", construct=True),
            "תשע מאות תשעים ותשעה טריליון "
            "תשע מאות תשעים ותשעה מיליארד "
            "תשע מאות תשעים ותשעה מיליון "
            "תשע מאות תשעים ותשעה אלף תשע מאות תשעים ותשע",
        )
        self.assertEqual(
            num2words(999999999999999, lang="he", gender="m", construct=True),
            "תשע מאות תשעים ותשעה טריליון "
            "תשע מאות תשעים ותשעה מיליארד "
            "תשע מאות תשעים ותשעה מיליון "
            "תשע מאות תשעים ותשעה אלף תשע מאות תשעים ותשעה",
        )

    def test_cardinal_for_float_number(self):
        self.assertEqual(num2words(12.5, lang="he"), "שתים עשרה נקודה חמש")
        self.assertEqual(num2words(12.51, lang="he"), "שתים עשרה נקודה חמש אחת")
        self.assertEqual(num2words(12.53, lang="he"), "שתים עשרה נקודה חמש שלוש")
        self.assertEqual(num2words(12.59, lang="he"), "שתים עשרה נקודה חמש תשע")
        self.assertEqual(num2words(12.5, lang="he", gender="m"), "שנים עשר נקודה חמש")
        self.assertEqual(
            num2words(12.51, lang="he", gender="m"), "שנים עשר נקודה חמש אחת"
        )
        self.assertEqual(
            num2words(12.53, lang="he", gender="m"), "שנים עשר נקודה חמש שלוש"
        )
        self.assertEqual(
            num2words(12.59, lang="he", gender="m"), "שנים עשר נקודה חמש תשע"
        )
        self.assertEqual(
            num2words(12.594132, lang="he", gender="m"),
            "שנים עשר נקודה חמש תשע ארבע אחת שלוש שתיים",
        )
        # gender= on floats (gladiaio/num2words2#167): a whole float takes the
        # integer path, a Decimal string the same float grammar.
        self.assertEqual(num2words(12.0, lang="he", gender="m"), "שנים עשר")
        self.assertEqual(num2words("12.5", lang="he", gender="m"), "שנים עשר נקודה חמש")
        self.assertEqual(num2words(-1.5, lang="he", gender="m"), "מינוס אחד נקודה חמש")




    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="he"), "מינוס אפס נקודה ארבע")
        self.assertEqual(num2words(-0.5, lang="he"), "מינוס אפס נקודה חמש")
        self.assertEqual(num2words(-1.4, lang="he"), "מינוס אחת נקודה ארבע")


def test_he_currency_agreement_and_order():
    # gladiaio/num2words2#254: masculine numeral for shekel/dollar, feminine
    # for agora; 1 follows the noun, 2 takes the construct form, and the
    # conjunction ו is prefixed to the agorot part, never glued to the noun.
    from num2words2 import num2words

    def cur(v, **kw):
        return num2words(v, lang="he", to="currency", **kw)

    assert cur(1) == "שקל אחד"
    assert cur(2) == "שני שקלים"
    assert cur(5) == "חמישה שקלים"
    assert cur(21) == "עשרים ואחד שקלים"
    assert cur(2.5) == "שני שקלים וחמישים אגורות"
    assert cur(1.01) == "שקל אחד ואגורה אחת"
    assert cur(2.02) == "שני שקלים ושתי אגורות"
    assert cur(3.05) == "שלושה שקלים וחמש אגורות"
    assert cur(100) == "מאה שקלים"
    assert cur(5, currency="USD") == "חמישה דולרים"
    assert cur(1, currency="EUR") == "אירו אחד"
    assert cur(-3.75, currency="USD") == "מינוס שלושה דולרים ושבעים וחמישה סנטים"
    assert cur(2.5, cents=False) == "שני שקלים ו-50 אגורות"
    assert "ו " not in cur(100) and not cur(100).endswith("ו")
