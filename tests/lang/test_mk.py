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

from unittest import TestCase

from num2words2 import num2words


class Num2WordsMKTest(TestCase):
    """Comprehensive test cases for Macedonian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="mk"), "нула")
        self.assertEqual(num2words(1, lang="mk"), "еден")
        self.assertEqual(num2words(2, lang="mk"), "два")
        self.assertEqual(num2words(3, lang="mk"), "три")
        self.assertEqual(num2words(4, lang="mk"), "четири")
        self.assertEqual(num2words(5, lang="mk"), "пет")
        self.assertEqual(num2words(6, lang="mk"), "шест")
        self.assertEqual(num2words(7, lang="mk"), "седум")
        self.assertEqual(num2words(8, lang="mk"), "осум")
        self.assertEqual(num2words(9, lang="mk"), "девет")
        self.assertEqual(num2words(10, lang="mk"), "десет")
        self.assertEqual(num2words(11, lang="mk"), "десет еден")
        self.assertEqual(num2words(12, lang="mk"), "десет два")
        self.assertEqual(num2words(13, lang="mk"), "десет три")
        self.assertEqual(num2words(14, lang="mk"), "десет четири")
        self.assertEqual(num2words(15, lang="mk"), "десет пет")
        self.assertEqual(num2words(16, lang="mk"), "десет шест")
        self.assertEqual(num2words(17, lang="mk"), "десет седум")
        self.assertEqual(num2words(18, lang="mk"), "десет осум")
        self.assertEqual(num2words(19, lang="mk"), "десет девет")
        self.assertEqual(num2words(20, lang="mk"), "дваесет")
        self.assertEqual(num2words(21, lang="mk"), "дваесет еден")
        self.assertEqual(num2words(22, lang="mk"), "дваесет два")
        self.assertEqual(num2words(23, lang="mk"), "дваесет три")
        self.assertEqual(num2words(24, lang="mk"), "дваесет четири")
        self.assertEqual(num2words(25, lang="mk"), "дваесет пет")
        self.assertEqual(num2words(26, lang="mk"), "дваесет шест")
        self.assertEqual(num2words(27, lang="mk"), "дваесет седум")
        self.assertEqual(num2words(28, lang="mk"), "дваесет осум")
        self.assertEqual(num2words(29, lang="mk"), "дваесет девет")
        self.assertEqual(num2words(30, lang="mk"), "триесет")
        self.assertEqual(num2words(31, lang="mk"), "триесет еден")
        self.assertEqual(num2words(35, lang="mk"), "триесет пет")
        self.assertEqual(num2words(40, lang="mk"), "четириесет")
        self.assertEqual(num2words(45, lang="mk"), "четириесет пет")
        self.assertEqual(num2words(50, lang="mk"), "педесет")
        self.assertEqual(num2words(55, lang="mk"), "педесет пет")
        self.assertEqual(num2words(60, lang="mk"), "шеесет")
        self.assertEqual(num2words(65, lang="mk"), "шеесет пет")
        self.assertEqual(num2words(70, lang="mk"), "седумдесет")
        self.assertEqual(num2words(75, lang="mk"), "седумдесет пет")
        self.assertEqual(num2words(80, lang="mk"), "осумдесет")
        self.assertEqual(num2words(85, lang="mk"), "осумдесет пет")
        self.assertEqual(num2words(90, lang="mk"), "деведесет")
        self.assertEqual(num2words(95, lang="mk"), "деведесет пет")
        self.assertEqual(num2words(99, lang="mk"), "деведесет девет")
        self.assertEqual(num2words(100, lang="mk"), "еден сто")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="mk"), "еден сто еден")
        self.assertEqual(num2words(110, lang="mk"), "еден сто десет")
        self.assertEqual(num2words(111, lang="mk"), "еден сто десет еден")
        self.assertEqual(num2words(120, lang="mk"), "еден сто дваесет")
        self.assertEqual(num2words(125, lang="mk"), "еден сто дваесет пет")
        self.assertEqual(num2words(150, lang="mk"), "еден сто педесет")
        self.assertEqual(num2words(175, lang="mk"), "еден сто седумдесет пет")
        self.assertEqual(num2words(199, lang="mk"), "еден сто деведесет девет")
        self.assertEqual(num2words(200, lang="mk"), "два сто")
        self.assertEqual(num2words(201, lang="mk"), "два сто еден")
        self.assertEqual(num2words(210, lang="mk"), "два сто десет")
        self.assertEqual(num2words(220, lang="mk"), "два сто дваесет")
        self.assertEqual(num2words(250, lang="mk"), "два сто педесет")
        self.assertEqual(num2words(299, lang="mk"), "два сто деведесет девет")
        self.assertEqual(num2words(300, lang="mk"), "три сто")
        self.assertEqual(num2words(333, lang="mk"), "три сто триесет три")
        self.assertEqual(num2words(400, lang="mk"), "четири сто")
        self.assertEqual(num2words(444, lang="mk"), "четири сто четириесет четири")
        self.assertEqual(num2words(500, lang="mk"), "пет сто")
        self.assertEqual(num2words(555, lang="mk"), "пет сто педесет пет")
        self.assertEqual(num2words(600, lang="mk"), "шест сто")
        self.assertEqual(num2words(666, lang="mk"), "шест сто шеесет шест")
        self.assertEqual(num2words(700, lang="mk"), "седум сто")
        self.assertEqual(num2words(777, lang="mk"), "седум сто седумдесет седум")
        self.assertEqual(num2words(800, lang="mk"), "осум сто")
        self.assertEqual(num2words(888, lang="mk"), "осум сто осумдесет осум")
        self.assertEqual(num2words(900, lang="mk"), "девет сто")
        self.assertEqual(num2words(999, lang="mk"), "девет сто деведесет девет")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="mk"), "еден илјада")
        self.assertEqual(num2words(1001, lang="mk"), "еден илјада еден")
        self.assertEqual(num2words(1010, lang="mk"), "еден илјада десет")
        self.assertEqual(num2words(1100, lang="mk"), "еден илјада еден сто")
        self.assertEqual(num2words(1111, lang="mk"), "еден илјада еден сто десет еден")
        self.assertEqual(
            num2words(1234, lang="mk"), "еден илјада два сто триесет четири"
        )
        self.assertEqual(num2words(1500, lang="mk"), "еден илјада пет сто")
        self.assertEqual(
            num2words(1999, lang="mk"), "еден илјада девет сто деведесет девет"
        )
        self.assertEqual(num2words(2000, lang="mk"), "два илјада")
        self.assertEqual(num2words(2001, lang="mk"), "два илјада еден")
        self.assertEqual(num2words(2020, lang="mk"), "два илјада дваесет")
        self.assertEqual(num2words(2222, lang="mk"), "два илјада два сто дваесет два")
        self.assertEqual(num2words(3000, lang="mk"), "три илјада")
        self.assertEqual(num2words(3333, lang="mk"), "три илјада три сто триесет три")
        self.assertEqual(num2words(4000, lang="mk"), "четири илјада")
        self.assertEqual(
            num2words(4444, lang="mk"), "четири илјада четири сто четириесет четири"
        )
        self.assertEqual(num2words(5000, lang="mk"), "пет илјада")
        self.assertEqual(num2words(5555, lang="mk"), "пет илјада пет сто педесет пет")
        self.assertEqual(num2words(6000, lang="mk"), "шест илјада")
        self.assertEqual(num2words(6666, lang="mk"), "шест илјада шест сто шеесет шест")
        self.assertEqual(num2words(7000, lang="mk"), "седум илјада")
        self.assertEqual(
            num2words(7777, lang="mk"), "седум илјада седум сто седумдесет седум"
        )
        self.assertEqual(num2words(8000, lang="mk"), "осум илјада")
        self.assertEqual(
            num2words(8888, lang="mk"), "осум илјада осум сто осумдесет осум"
        )
        self.assertEqual(num2words(9000, lang="mk"), "девет илјада")
        self.assertEqual(
            num2words(9999, lang="mk"), "девет илјада девет сто деведесет девет"
        )
        self.assertEqual(num2words(10000, lang="mk"), "десет илјада")
        self.assertEqual(num2words(10001, lang="mk"), "десет илјада еден")
        self.assertEqual(
            num2words(11111, lang="mk"), "десет еден илјада еден сто десет еден"
        )
        self.assertEqual(
            num2words(12345, lang="mk"), "десет два илјада три сто четириесет пет"
        )
        self.assertEqual(num2words(20000, lang="mk"), "дваесет илјада")
        self.assertEqual(num2words(50000, lang="mk"), "педесет илјада")
        self.assertEqual(
            num2words(99999, lang="mk"),
            "деведесет девет илјада девет сто деведесет девет",
        )
        self.assertEqual(num2words(100000, lang="mk"), "еден сто илјада")
        self.assertEqual(
            num2words(123456, lang="mk"),
            "еден сто дваесет три илјада четири сто педесет шест",
        )
        self.assertEqual(num2words(200000, lang="mk"), "два сто илјада")
        self.assertEqual(num2words(500000, lang="mk"), "пет сто илјада")
        self.assertEqual(
            num2words(654321, lang="mk"),
            "шест сто педесет четири илјада три сто дваесет еден",
        )
        self.assertEqual(
            num2words(999999, lang="mk"),
            "девет сто деведесет девет илјада девет сто деведесет девет",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="mk"), "еден милион")
        self.assertEqual(num2words(1000001, lang="mk"), "еден милион еден")
        self.assertEqual(
            num2words(1111111, lang="mk"),
            "еден милион еден сто десет еден илјада еден сто десет еден",
        )
        self.assertEqual(
            num2words(1234567, lang="mk"),
            "еден милион два сто триесет четири илјада пет сто шеесет седум",
        )
        self.assertEqual(num2words(2000000, lang="mk"), "два милион")
        self.assertEqual(num2words(5000000, lang="mk"), "пет милион")
        self.assertEqual(
            num2words(9999999, lang="mk"),
            "девет милион девет сто деведесет девет илјада девет сто деведесет девет",
        )
        self.assertEqual(num2words(10000000, lang="mk"), "десет милион")
        self.assertEqual(
            num2words(12345678, lang="mk"),
            "десет два милион три сто четириесет пет илјада шест сто седумдесет осум",
        )
        self.assertEqual(
            num2words(99999999, lang="mk"),
            "деведесет девет милион девет сто деведесет девет илјада девет сто деведесет девет",
        )
        self.assertEqual(num2words(100000000, lang="mk"), "еден сто милион")
        self.assertEqual(
            num2words(123456789, lang="mk"),
            "еден сто дваесет три милион четири сто педесет шест илјада седум сто осумдесет девет",
        )
        self.assertEqual(
            num2words(999999999, lang="mk"),
            "девет сто деведесет девет милион девет сто деведесет девет илјада девет сто деведесет девет",
        )
        self.assertEqual(
            num2words(1000000000, lang="mk"),
            "еден милијарда",
        )
        self.assertEqual(
            num2words(1234567890, lang="mk"),
            "еден милијарда два сто триесет четири милион пет сто шеесет седум илјада осум сто деведесет",
        )
        self.assertEqual(
            num2words(9999999999, lang="mk"),
            "девет милијарда девет сто деведесет девет милион девет сто деведесет девет илјада девет сто деведесет девет",
        )
        self.assertEqual(
            num2words(10000000000, lang="mk"),
            "десет милијарда",
        )
        self.assertEqual(
            num2words(99999999999, lang="mk"),
            "деведесет девет милијарда девет сто деведесет девет милион девет сто деведесет девет илјада девет сто деведесет девет",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="mk"), "минус еден")
        self.assertEqual(num2words(-2, lang="mk"), "минус два")
        self.assertEqual(num2words(-5, lang="mk"), "минус пет")
        self.assertEqual(num2words(-10, lang="mk"), "минус десет")
        self.assertEqual(num2words(-11, lang="mk"), "минус десет еден")
        self.assertEqual(num2words(-20, lang="mk"), "минус дваесет")
        self.assertEqual(num2words(-50, lang="mk"), "минус педесет")
        self.assertEqual(num2words(-99, lang="mk"), "минус деведесет девет")
        self.assertEqual(num2words(-100, lang="mk"), "минус еден сто")
        self.assertEqual(num2words(-101, lang="mk"), "минус еден сто еден")
        self.assertEqual(num2words(-200, lang="mk"), "минус два сто")
        self.assertEqual(num2words(-999, lang="mk"), "минус девет сто деведесет девет")
        self.assertEqual(num2words(-1000, lang="mk"), "минус еден илјада")
        self.assertEqual(num2words(-1001, lang="mk"), "минус еден илјада еден")
        self.assertEqual(num2words(-10000, lang="mk"), "минус десет илјада")
        self.assertEqual(num2words(-100000, lang="mk"), "минус еден сто илјада")
        self.assertEqual(num2words(-1000000, lang="mk"), "минус еден милион")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="mk"), "нула запирка еден")
        self.assertEqual(num2words(0.5, lang="mk"), "нула запирка пет")
        self.assertEqual(num2words(0.9, lang="mk"), "нула запирка девет")
        self.assertEqual(num2words(1.1, lang="mk"), "еден запирка еден")
        self.assertEqual(num2words(1.5, lang="mk"), "еден запирка пет")
        self.assertEqual(num2words(2.5, lang="mk"), "два запирка пет")
        self.assertEqual(num2words(3.14, lang="mk"), "три запирка еден четири")
        self.assertEqual(num2words(10.5, lang="mk"), "десет запирка пет")
        self.assertEqual(num2words(11.11, lang="mk"), "десет еден запирка еден еден")
        self.assertEqual(num2words(20.2, lang="mk"), "дваесет запирка два")
        self.assertEqual(
            num2words(99.99, lang="mk"), "деведесет девет запирка девет девет"
        )
        self.assertEqual(num2words(100.01, lang="mk"), "еден сто запирка нула еден")
        self.assertEqual(num2words(100.5, lang="mk"), "еден сто запирка пет")
        self.assertEqual(
            num2words(123.45, lang="mk"), "еден сто дваесет три запирка четири пет"
        )
        self.assertEqual(num2words(1000.5, lang="mk"), "еден илјада запирка пет")
        self.assertEqual(
            num2words(1234.56, lang="mk"),
            "еден илјада два сто триесет четири запирка пет шест",
        )
        self.assertEqual(
            num2words(10000.01, lang="mk"), "десет илјада запирка нула еден"
        )
        self.assertEqual(num2words(-0.5, lang="mk"), "минус нула запирка пет")
        self.assertEqual(num2words(-1.5, lang="mk"), "минус еден запирка пет")
        self.assertEqual(num2words(-10.5, lang="mk"), "минус десет запирка пет")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="mk", ordinal=True), "еден-ти")
        self.assertEqual(num2words(2, lang="mk", ordinal=True), "два-ти")
        self.assertEqual(num2words(3, lang="mk", ordinal=True), "три-ти")
        self.assertEqual(num2words(4, lang="mk", ordinal=True), "четири-ти")
        self.assertEqual(num2words(5, lang="mk", ordinal=True), "пет-ти")
        self.assertEqual(num2words(6, lang="mk", ordinal=True), "шест-ти")
        self.assertEqual(num2words(7, lang="mk", ordinal=True), "седум-ти")
        self.assertEqual(num2words(8, lang="mk", ordinal=True), "осум-ти")
        self.assertEqual(num2words(9, lang="mk", ordinal=True), "девет-ти")
        self.assertEqual(num2words(10, lang="mk", ordinal=True), "десет-ти")
        self.assertEqual(num2words(11, lang="mk", ordinal=True), "десет еден-ти")
        self.assertEqual(num2words(12, lang="mk", ordinal=True), "десет два-ти")
        self.assertEqual(num2words(13, lang="mk", ordinal=True), "десет три-ти")
        self.assertEqual(num2words(14, lang="mk", ordinal=True), "десет четири-ти")
        self.assertEqual(num2words(15, lang="mk", ordinal=True), "десет пет-ти")
        self.assertEqual(num2words(16, lang="mk", ordinal=True), "десет шест-ти")
        self.assertEqual(num2words(17, lang="mk", ordinal=True), "десет седум-ти")
        self.assertEqual(num2words(18, lang="mk", ordinal=True), "десет осум-ти")
        self.assertEqual(num2words(19, lang="mk", ordinal=True), "десет девет-ти")
        self.assertEqual(num2words(20, lang="mk", ordinal=True), "дваесет-ти")
        self.assertEqual(num2words(21, lang="mk", ordinal=True), "дваесет еден-ти")
        self.assertEqual(num2words(22, lang="mk", ordinal=True), "дваесет два-ти")
        self.assertEqual(num2words(25, lang="mk", ordinal=True), "дваесет пет-ти")
        self.assertEqual(num2words(30, lang="mk", ordinal=True), "триесет-ти")
        self.assertEqual(num2words(40, lang="mk", ordinal=True), "четириесет-ти")
        self.assertEqual(num2words(50, lang="mk", ordinal=True), "педесет-ти")
        self.assertEqual(num2words(60, lang="mk", ordinal=True), "шеесет-ти")
        self.assertEqual(num2words(70, lang="mk", ordinal=True), "седумдесет-ти")
        self.assertEqual(num2words(80, lang="mk", ordinal=True), "осумдесет-ти")
        self.assertEqual(num2words(90, lang="mk", ordinal=True), "деведесет-ти")
        self.assertEqual(num2words(100, lang="mk", ordinal=True), "еден сто-ти")
        self.assertEqual(num2words(101, lang="mk", ordinal=True), "еден сто еден-ти")
        self.assertEqual(num2words(200, lang="mk", ordinal=True), "два сто-ти")
        self.assertEqual(num2words(500, lang="mk", ordinal=True), "пет сто-ти")
        self.assertEqual(num2words(1000, lang="mk", ordinal=True), "еден илјада-ти")
        self.assertEqual(
            num2words(1001, lang="mk", ordinal=True), "еден илјада еден-ти"
        )
        self.assertEqual(num2words(10000, lang="mk", ordinal=True), "десет илјада-ти")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="mk", to="currency", currency="MKD"), "нула денари"
        )
        self.assertEqual(
            num2words(0.01, lang="mk", to="currency", currency="MKD"),
            "нула денари еден дени",
        )
        self.assertEqual(
            num2words(0.5, lang="mk", to="currency", currency="MKD"),
            "нула денари педесет дени",
        )
        self.assertEqual(
            num2words(1, lang="mk", to="currency", currency="MKD"), "еден денар"
        )
        self.assertEqual(
            num2words(1.5, lang="mk", to="currency", currency="MKD"),
            "еден денар педесет дени",
        )
        self.assertEqual(
            num2words(0, lang="mk", to="currency", currency="USD"), "нула долари"
        )
        self.assertEqual(
            num2words(0.01, lang="mk", to="currency", currency="USD"),
            "нула долари еден цент",
        )
        self.assertEqual(
            num2words(0.5, lang="mk", to="currency", currency="USD"),
            "нула долари педесет центи",
        )
        self.assertEqual(
            num2words(1, lang="mk", to="currency", currency="USD"), "еден долар"
        )
        self.assertEqual(
            num2words(1.5, lang="mk", to="currency", currency="USD"),
            "еден долар педесет центи",
        )
        self.assertEqual(
            num2words(0, lang="mk", to="currency", currency="EUR"), "нула евра"
        )
        self.assertEqual(
            num2words(0.01, lang="mk", to="currency", currency="EUR"),
            "нула евра еден цент",
        )
        self.assertEqual(
            num2words(0.5, lang="mk", to="currency", currency="EUR"),
            "нула евра педесет центи",
        )
        self.assertEqual(
            num2words(1, lang="mk", to="currency", currency="EUR"), "еден евро"
        )
        self.assertEqual(
            num2words(1.5, lang="mk", to="currency", currency="EUR"),
            "еден евро педесет центи",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="mk", to="year"), "еден илјада")
        self.assertEqual(
            num2words(1066, lang="mk", to="year"), "еден илјада шеесет шест"
        )
        self.assertEqual(
            num2words(1492, lang="mk", to="year"),
            "еден илјада четири сто деведесет два",
        )
        self.assertEqual(
            num2words(1776, lang="mk", to="year"),
            "еден илјада седум сто седумдесет шест",
        )
        self.assertEqual(num2words(1800, lang="mk", to="year"), "еден илјада осум сто")
        self.assertEqual(num2words(1900, lang="mk", to="year"), "еден илјада девет сто")
        self.assertEqual(
            num2words(1984, lang="mk", to="year"),
            "еден илјада девет сто осумдесет четири",
        )
        self.assertEqual(
            num2words(1999, lang="mk", to="year"),
            "еден илјада девет сто деведесет девет",
        )
        self.assertEqual(num2words(2000, lang="mk", to="year"), "два илјада")
        self.assertEqual(num2words(2001, lang="mk", to="year"), "два илјада еден")
        self.assertEqual(num2words(2010, lang="mk", to="year"), "два илјада десет")
        self.assertEqual(num2words(2020, lang="mk", to="year"), "два илјада дваесет")
        self.assertEqual(
            num2words(2024, lang="mk", to="year"), "два илјада дваесет четири"
        )
        self.assertEqual(num2words(2100, lang="mk", to="year"), "два илјада еден сто")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="mk"), "нула")
        self.assertEqual(num2words("1", lang="mk"), "еден")
        self.assertEqual(num2words("10", lang="mk"), "десет")
        self.assertEqual(num2words("100", lang="mk"), "еден сто")
        self.assertEqual(num2words("1000", lang="mk"), "еден илјада")
        self.assertEqual(num2words("10000", lang="mk"), "десет илјада")
        self.assertEqual(num2words("100000", lang="mk"), "еден сто илјада")
        self.assertEqual(num2words("1000000", lang="mk"), "еден милион")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="mk"), "нула")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="mk"), num2words("100", lang="mk"))
        self.assertEqual(num2words(1000, lang="mk"), num2words("1000", lang="mk"))
