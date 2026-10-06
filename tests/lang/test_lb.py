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


class Num2WordsLBTest(TestCase):
    """Comprehensive test cases for Luxembourgish language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="lb"), "null")
        self.assertEqual(num2words(1, lang="lb"), "eent")
        self.assertEqual(num2words(2, lang="lb"), "zwou")
        self.assertEqual(num2words(3, lang="lb"), "dräi")
        self.assertEqual(num2words(4, lang="lb"), "véier")
        self.assertEqual(num2words(5, lang="lb"), "fënnef")
        self.assertEqual(num2words(6, lang="lb"), "sechs")
        self.assertEqual(num2words(7, lang="lb"), "siwen")
        self.assertEqual(num2words(8, lang="lb"), "aacht")
        self.assertEqual(num2words(9, lang="lb"), "néng")
        self.assertEqual(num2words(10, lang="lb"), "zéng")
        self.assertEqual(num2words(11, lang="lb"), "zéng eent")
        self.assertEqual(num2words(12, lang="lb"), "zéng zwou")
        self.assertEqual(num2words(13, lang="lb"), "zéng dräi")
        self.assertEqual(num2words(14, lang="lb"), "zéng véier")
        self.assertEqual(num2words(15, lang="lb"), "zéng fënnef")
        self.assertEqual(num2words(16, lang="lb"), "zéng sechs")
        self.assertEqual(num2words(17, lang="lb"), "zéng siwen")
        self.assertEqual(num2words(18, lang="lb"), "zéng aacht")
        self.assertEqual(num2words(19, lang="lb"), "zéng néng")
        self.assertEqual(num2words(20, lang="lb"), "zwanzeg")
        self.assertEqual(num2words(21, lang="lb"), "zwanzeg eent")
        self.assertEqual(num2words(22, lang="lb"), "zwanzeg zwou")
        self.assertEqual(num2words(23, lang="lb"), "zwanzeg dräi")
        self.assertEqual(num2words(24, lang="lb"), "zwanzeg véier")
        self.assertEqual(num2words(25, lang="lb"), "zwanzeg fënnef")
        self.assertEqual(num2words(26, lang="lb"), "zwanzeg sechs")
        self.assertEqual(num2words(27, lang="lb"), "zwanzeg siwen")
        self.assertEqual(num2words(28, lang="lb"), "zwanzeg aacht")
        self.assertEqual(num2words(29, lang="lb"), "zwanzeg néng")
        self.assertEqual(num2words(30, lang="lb"), "drësseg")
        self.assertEqual(num2words(31, lang="lb"), "drësseg eent")
        self.assertEqual(num2words(35, lang="lb"), "drësseg fënnef")
        self.assertEqual(num2words(40, lang="lb"), "véierzeg")
        self.assertEqual(num2words(45, lang="lb"), "véierzeg fënnef")
        self.assertEqual(num2words(50, lang="lb"), "fofzeg")
        self.assertEqual(num2words(55, lang="lb"), "fofzeg fënnef")
        self.assertEqual(num2words(60, lang="lb"), "sechzeg")
        self.assertEqual(num2words(65, lang="lb"), "sechzeg fënnef")
        self.assertEqual(num2words(70, lang="lb"), "siwwenzeg")
        self.assertEqual(num2words(75, lang="lb"), "siwwenzeg fënnef")
        self.assertEqual(num2words(80, lang="lb"), "achtzeg")
        self.assertEqual(num2words(85, lang="lb"), "achtzeg fënnef")
        self.assertEqual(num2words(90, lang="lb"), "nongzeg")
        self.assertEqual(num2words(95, lang="lb"), "nongzeg fënnef")
        self.assertEqual(num2words(99, lang="lb"), "nongzeg néng")
        self.assertEqual(num2words(100, lang="lb"), "eent honnert")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="lb"), "eent honnert eent")
        self.assertEqual(num2words(110, lang="lb"), "eent honnert zéng")
        self.assertEqual(num2words(111, lang="lb"), "eent honnert zéng eent")
        self.assertEqual(num2words(120, lang="lb"), "eent honnert zwanzeg")
        self.assertEqual(num2words(125, lang="lb"), "eent honnert zwanzeg fënnef")
        self.assertEqual(num2words(150, lang="lb"), "eent honnert fofzeg")
        self.assertEqual(num2words(175, lang="lb"), "eent honnert siwwenzeg fënnef")
        self.assertEqual(num2words(199, lang="lb"), "eent honnert nongzeg néng")
        self.assertEqual(num2words(200, lang="lb"), "zwou honnert")
        self.assertEqual(num2words(201, lang="lb"), "zwou honnert eent")
        self.assertEqual(num2words(210, lang="lb"), "zwou honnert zéng")
        self.assertEqual(num2words(220, lang="lb"), "zwou honnert zwanzeg")
        self.assertEqual(num2words(250, lang="lb"), "zwou honnert fofzeg")
        self.assertEqual(num2words(299, lang="lb"), "zwou honnert nongzeg néng")
        self.assertEqual(num2words(300, lang="lb"), "dräi honnert")
        self.assertEqual(num2words(333, lang="lb"), "dräi honnert drësseg dräi")
        self.assertEqual(num2words(400, lang="lb"), "véier honnert")
        self.assertEqual(num2words(444, lang="lb"), "véier honnert véierzeg véier")
        self.assertEqual(num2words(500, lang="lb"), "fënnef honnert")
        self.assertEqual(num2words(555, lang="lb"), "fënnef honnert fofzeg fënnef")
        self.assertEqual(num2words(600, lang="lb"), "sechs honnert")
        self.assertEqual(num2words(666, lang="lb"), "sechs honnert sechzeg sechs")
        self.assertEqual(num2words(700, lang="lb"), "siwen honnert")
        self.assertEqual(num2words(777, lang="lb"), "siwen honnert siwwenzeg siwen")
        self.assertEqual(num2words(800, lang="lb"), "aacht honnert")
        self.assertEqual(num2words(888, lang="lb"), "aacht honnert achtzeg aacht")
        self.assertEqual(num2words(900, lang="lb"), "néng honnert")
        self.assertEqual(num2words(999, lang="lb"), "néng honnert nongzeg néng")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="lb"), "eent dausend")
        self.assertEqual(num2words(1001, lang="lb"), "eent dausend eent")
        self.assertEqual(num2words(1010, lang="lb"), "eent dausend zéng")
        self.assertEqual(num2words(1100, lang="lb"), "eent dausend eent honnert")
        self.assertEqual(
            num2words(1111, lang="lb"), "eent dausend eent honnert zéng eent"
        )
        self.assertEqual(
            num2words(1234, lang="lb"), "eent dausend zwou honnert drësseg véier"
        )
        self.assertEqual(num2words(1500, lang="lb"), "eent dausend fënnef honnert")
        self.assertEqual(
            num2words(1999, lang="lb"), "eent dausend néng honnert nongzeg néng"
        )
        self.assertEqual(num2words(2000, lang="lb"), "zwou dausend")
        self.assertEqual(num2words(2001, lang="lb"), "zwou dausend eent")
        self.assertEqual(num2words(2020, lang="lb"), "zwou dausend zwanzeg")
        self.assertEqual(
            num2words(2222, lang="lb"), "zwou dausend zwou honnert zwanzeg zwou"
        )
        self.assertEqual(num2words(3000, lang="lb"), "dräi dausend")
        self.assertEqual(
            num2words(3333, lang="lb"), "dräi dausend dräi honnert drësseg dräi"
        )
        self.assertEqual(num2words(4000, lang="lb"), "véier dausend")
        self.assertEqual(
            num2words(4444, lang="lb"), "véier dausend véier honnert véierzeg véier"
        )
        self.assertEqual(num2words(5000, lang="lb"), "fënnef dausend")
        self.assertEqual(
            num2words(5555, lang="lb"), "fënnef dausend fënnef honnert fofzeg fënnef"
        )
        self.assertEqual(num2words(6000, lang="lb"), "sechs dausend")
        self.assertEqual(
            num2words(6666, lang="lb"), "sechs dausend sechs honnert sechzeg sechs"
        )
        self.assertEqual(num2words(7000, lang="lb"), "siwen dausend")
        self.assertEqual(
            num2words(7777, lang="lb"), "siwen dausend siwen honnert siwwenzeg siwen"
        )
        self.assertEqual(num2words(8000, lang="lb"), "aacht dausend")
        self.assertEqual(
            num2words(8888, lang="lb"), "aacht dausend aacht honnert achtzeg aacht"
        )
        self.assertEqual(num2words(9000, lang="lb"), "néng dausend")
        self.assertEqual(
            num2words(9999, lang="lb"), "néng dausend néng honnert nongzeg néng"
        )
        self.assertEqual(num2words(10000, lang="lb"), "zéng dausend")
        self.assertEqual(num2words(10001, lang="lb"), "zéng dausend eent")
        self.assertEqual(
            num2words(11111, lang="lb"), "zéng eent dausend eent honnert zéng eent"
        )
        self.assertEqual(
            num2words(12345, lang="lb"),
            "zéng zwou dausend dräi honnert véierzeg fënnef",
        )
        self.assertEqual(num2words(20000, lang="lb"), "zwanzeg dausend")
        self.assertEqual(num2words(50000, lang="lb"), "fofzeg dausend")
        self.assertEqual(
            num2words(99999, lang="lb"),
            "nongzeg néng dausend néng honnert nongzeg néng",
        )
        self.assertEqual(num2words(100000, lang="lb"), "eent honnert dausend")
        self.assertEqual(
            num2words(123456, lang="lb"),
            "eent honnert zwanzeg dräi dausend véier honnert fofzeg sechs",
        )
        self.assertEqual(num2words(200000, lang="lb"), "zwou honnert dausend")
        self.assertEqual(num2words(500000, lang="lb"), "fënnef honnert dausend")
        self.assertEqual(
            num2words(654321, lang="lb"),
            "sechs honnert fofzeg véier dausend dräi honnert zwanzeg eent",
        )
        self.assertEqual(
            num2words(999999, lang="lb"),
            "néng honnert nongzeg néng dausend néng honnert nongzeg néng",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="lb"), "eent Millioun")
        self.assertEqual(num2words(1000001, lang="lb"), "eent Millioun eent")
        self.assertEqual(
            num2words(1111111, lang="lb"),
            "eent Millioun eent honnert zéng eent dausend eent honnert zéng eent",
        )
        self.assertEqual(
            num2words(1234567, lang="lb"),
            "eent Millioun zwou honnert drësseg véier dausend fënnef honnert sechzeg siwen",
        )
        self.assertEqual(num2words(2000000, lang="lb"), "zwou Millioun")
        self.assertEqual(num2words(5000000, lang="lb"), "fënnef Millioun")
        self.assertEqual(
            num2words(9999999, lang="lb"),
            "néng Millioun néng honnert nongzeg néng dausend néng honnert nongzeg néng",
        )
        self.assertEqual(num2words(10000000, lang="lb"), "zéng Millioun")
        self.assertEqual(
            num2words(12345678, lang="lb"),
            "zéng zwou Millioun dräi honnert véierzeg fënnef dausend sechs honnert siwwenzeg aacht",
        )
        self.assertEqual(
            num2words(99999999, lang="lb"),
            "nongzeg néng Millioun néng honnert nongzeg néng dausend néng honnert nongzeg néng",
        )
        self.assertEqual(num2words(100000000, lang="lb"), "eent honnert Millioun")
        self.assertEqual(
            num2words(123456789, lang="lb"),
            "eent honnert zwanzeg dräi Millioun véier honnert fofzeg sechs dausend siwen honnert achtzeg néng",
        )
        self.assertEqual(
            num2words(999999999, lang="lb"),
            "néng honnert nongzeg néng Millioun néng honnert nongzeg néng dausend néng honnert nongzeg néng",
        )
        self.assertEqual(num2words(1000000000, lang="lb"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="lb"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="lb"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="lb"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="lb"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="lb"), "minus eent")
        self.assertEqual(num2words(-2, lang="lb"), "minus zwou")
        self.assertEqual(num2words(-5, lang="lb"), "minus fënnef")
        self.assertEqual(num2words(-10, lang="lb"), "minus zéng")
        self.assertEqual(num2words(-11, lang="lb"), "minus zéng eent")
        self.assertEqual(num2words(-20, lang="lb"), "minus zwanzeg")
        self.assertEqual(num2words(-50, lang="lb"), "minus fofzeg")
        self.assertEqual(num2words(-99, lang="lb"), "minus nongzeg néng")
        self.assertEqual(num2words(-100, lang="lb"), "minus eent honnert")
        self.assertEqual(num2words(-101, lang="lb"), "minus eent honnert eent")
        self.assertEqual(num2words(-200, lang="lb"), "minus zwou honnert")
        self.assertEqual(num2words(-999, lang="lb"), "minus néng honnert nongzeg néng")
        self.assertEqual(num2words(-1000, lang="lb"), "minus eent dausend")
        self.assertEqual(num2words(-1001, lang="lb"), "minus eent dausend eent")
        self.assertEqual(num2words(-10000, lang="lb"), "minus zéng dausend")
        self.assertEqual(num2words(-100000, lang="lb"), "minus eent honnert dausend")
        self.assertEqual(num2words(-1000000, lang="lb"), "minus eent Millioun")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="lb"), "null Komma eent")
        self.assertEqual(num2words(0.5, lang="lb"), "null Komma fënnef")
        self.assertEqual(num2words(0.9, lang="lb"), "null Komma néng")
        self.assertEqual(num2words(1.1, lang="lb"), "eent Komma eent")
        self.assertEqual(num2words(1.5, lang="lb"), "eent Komma fënnef")
        self.assertEqual(num2words(2.5, lang="lb"), "zwou Komma fënnef")
        self.assertEqual(num2words(3.14, lang="lb"), "dräi Komma eent véier")
        self.assertEqual(num2words(10.5, lang="lb"), "zéng Komma fënnef")
        self.assertEqual(num2words(11.11, lang="lb"), "zéng eent Komma eent eent")
        self.assertEqual(num2words(20.2, lang="lb"), "zwanzeg Komma zwou")
        self.assertEqual(num2words(99.99, lang="lb"), "nongzeg néng Komma néng néng")
        self.assertEqual(num2words(100.01, lang="lb"), "eent honnert Komma null eent")
        self.assertEqual(num2words(100.5, lang="lb"), "eent honnert Komma fënnef")
        self.assertEqual(
            num2words(123.45, lang="lb"), "eent honnert zwanzeg dräi Komma véier fënnef"
        )
        self.assertEqual(num2words(1000.5, lang="lb"), "eent dausend Komma fënnef")
        self.assertEqual(
            num2words(1234.56, lang="lb"),
            "eent dausend zwou honnert drësseg véier Komma fënnef sechs",
        )
        self.assertEqual(num2words(10000.01, lang="lb"), "zéng dausend Komma null eent")
        self.assertEqual(num2words(-0.5, lang="lb"), "minus null Komma fënnef")
        self.assertEqual(num2words(-1.5, lang="lb"), "minus eent Komma fënnef")
        self.assertEqual(num2words(-10.5, lang="lb"), "minus zéng Komma fënnef")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="lb", ordinal=True), "eent-ten")
        self.assertEqual(num2words(2, lang="lb", ordinal=True), "zwou-ten")
        self.assertEqual(num2words(3, lang="lb", ordinal=True), "dräi-ten")
        self.assertEqual(num2words(4, lang="lb", ordinal=True), "véier-ten")
        self.assertEqual(num2words(5, lang="lb", ordinal=True), "fënnef-ten")
        self.assertEqual(num2words(6, lang="lb", ordinal=True), "sechs-ten")
        self.assertEqual(num2words(7, lang="lb", ordinal=True), "siwen-ten")
        self.assertEqual(num2words(8, lang="lb", ordinal=True), "aacht-ten")
        self.assertEqual(num2words(9, lang="lb", ordinal=True), "néng-ten")
        self.assertEqual(num2words(10, lang="lb", ordinal=True), "zéng-ten")
        self.assertEqual(num2words(11, lang="lb", ordinal=True), "zéng eent-ten")
        self.assertEqual(num2words(12, lang="lb", ordinal=True), "zéng zwou-ten")
        self.assertEqual(num2words(13, lang="lb", ordinal=True), "zéng dräi-ten")
        self.assertEqual(num2words(14, lang="lb", ordinal=True), "zéng véier-ten")
        self.assertEqual(num2words(15, lang="lb", ordinal=True), "zéng fënnef-ten")
        self.assertEqual(num2words(16, lang="lb", ordinal=True), "zéng sechs-ten")
        self.assertEqual(num2words(17, lang="lb", ordinal=True), "zéng siwen-ten")
        self.assertEqual(num2words(18, lang="lb", ordinal=True), "zéng aacht-ten")
        self.assertEqual(num2words(19, lang="lb", ordinal=True), "zéng néng-ten")
        self.assertEqual(num2words(20, lang="lb", ordinal=True), "zwanzeg-ten")
        self.assertEqual(num2words(21, lang="lb", ordinal=True), "zwanzeg eent-ten")
        self.assertEqual(num2words(22, lang="lb", ordinal=True), "zwanzeg zwou-ten")
        self.assertEqual(num2words(25, lang="lb", ordinal=True), "zwanzeg fënnef-ten")
        self.assertEqual(num2words(30, lang="lb", ordinal=True), "drësseg-ten")
        self.assertEqual(num2words(40, lang="lb", ordinal=True), "véierzeg-ten")
        self.assertEqual(num2words(50, lang="lb", ordinal=True), "fofzeg-ten")
        self.assertEqual(num2words(60, lang="lb", ordinal=True), "sechzeg-ten")
        self.assertEqual(num2words(70, lang="lb", ordinal=True), "siwwenzeg-ten")
        self.assertEqual(num2words(80, lang="lb", ordinal=True), "achtzeg-ten")
        self.assertEqual(num2words(90, lang="lb", ordinal=True), "nongzeg-ten")
        self.assertEqual(num2words(100, lang="lb", ordinal=True), "eent honnert-ten")
        self.assertEqual(
            num2words(101, lang="lb", ordinal=True), "eent honnert eent-ten"
        )
        self.assertEqual(num2words(200, lang="lb", ordinal=True), "zwou honnert-ten")
        self.assertEqual(num2words(500, lang="lb", ordinal=True), "fënnef honnert-ten")
        self.assertEqual(num2words(1000, lang="lb", ordinal=True), "eent dausend-ten")
        self.assertEqual(
            num2words(1001, lang="lb", ordinal=True), "eent dausend eent-ten"
        )
        self.assertEqual(num2words(10000, lang="lb", ordinal=True), "zéng dausend-ten")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="lb", to="currency", currency="EUR"), "null euro"
        )
        self.assertEqual(
            num2words(0.01, lang="lb", to="currency", currency="EUR"),
            "null euro eent cent",
        )
        self.assertEqual(
            num2words(0.5, lang="lb", to="currency", currency="EUR"),
            "null euro fofzeg cents",
        )
        self.assertEqual(
            num2words(1, lang="lb", to="currency", currency="EUR"), "eent euro"
        )
        self.assertEqual(
            num2words(1.5, lang="lb", to="currency", currency="EUR"),
            "eent euro fofzeg cents",
        )
        self.assertEqual(
            num2words(0, lang="lb", to="currency", currency="USD"), "null dollars"
        )
        self.assertEqual(
            num2words(0.01, lang="lb", to="currency", currency="USD"),
            "null dollars eent cent",
        )
        self.assertEqual(
            num2words(0.5, lang="lb", to="currency", currency="USD"),
            "null dollars fofzeg cents",
        )
        self.assertEqual(
            num2words(1, lang="lb", to="currency", currency="USD"), "eent dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="lb", to="currency", currency="USD"),
            "eent dollar fofzeg cents",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="lb", to="year"), "eent dausend")
        self.assertEqual(
            num2words(1066, lang="lb", to="year"), "eent dausend sechzeg sechs"
        )
        self.assertEqual(
            num2words(1492, lang="lb", to="year"),
            "eent dausend véier honnert nongzeg zwou",
        )
        self.assertEqual(
            num2words(1776, lang="lb", to="year"),
            "eent dausend siwen honnert siwwenzeg sechs",
        )
        self.assertEqual(
            num2words(1800, lang="lb", to="year"), "eent dausend aacht honnert"
        )
        self.assertEqual(
            num2words(1900, lang="lb", to="year"), "eent dausend néng honnert"
        )
        self.assertEqual(
            num2words(1984, lang="lb", to="year"),
            "eent dausend néng honnert achtzeg véier",
        )
        self.assertEqual(
            num2words(1999, lang="lb", to="year"),
            "eent dausend néng honnert nongzeg néng",
        )
        self.assertEqual(num2words(2000, lang="lb", to="year"), "zwou dausend")
        self.assertEqual(num2words(2001, lang="lb", to="year"), "zwou dausend eent")
        self.assertEqual(num2words(2010, lang="lb", to="year"), "zwou dausend zéng")
        self.assertEqual(num2words(2020, lang="lb", to="year"), "zwou dausend zwanzeg")
        self.assertEqual(
            num2words(2024, lang="lb", to="year"), "zwou dausend zwanzeg véier"
        )
        self.assertEqual(
            num2words(2100, lang="lb", to="year"), "zwou dausend eent honnert"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="lb"), "null")
        self.assertEqual(num2words("1", lang="lb"), "eent")
        self.assertEqual(num2words("10", lang="lb"), "zéng")
        self.assertEqual(num2words("100", lang="lb"), "eent honnert")
        self.assertEqual(num2words("1000", lang="lb"), "eent dausend")
        self.assertEqual(num2words("10000", lang="lb"), "zéng dausend")
        self.assertEqual(num2words("100000", lang="lb"), "eent honnert dausend")
        self.assertEqual(num2words("1000000", lang="lb"), "eent Millioun")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="lb"), "null")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="lb"), num2words("100", lang="lb"))
        self.assertEqual(num2words(1000, lang="lb"), num2words("1000", lang="lb"))

