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


class Num2WordsBRTest(TestCase):
    """Comprehensive test cases for Breton language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="br"), "mann")
        self.assertEqual(num2words(1, lang="br"), "unan")
        self.assertEqual(num2words(2, lang="br"), "daou")
        self.assertEqual(num2words(3, lang="br"), "tri")
        self.assertEqual(num2words(4, lang="br"), "pevar")
        self.assertEqual(num2words(5, lang="br"), "pemp")
        self.assertEqual(num2words(6, lang="br"), "c'hwec'h")
        self.assertEqual(num2words(7, lang="br"), "seizh")
        self.assertEqual(num2words(8, lang="br"), "eizh")
        self.assertEqual(num2words(9, lang="br"), "nav")
        self.assertEqual(num2words(10, lang="br"), "dek")
        self.assertEqual(num2words(11, lang="br"), "dek unan")
        self.assertEqual(num2words(12, lang="br"), "dek daou")
        self.assertEqual(num2words(13, lang="br"), "dek tri")
        self.assertEqual(num2words(14, lang="br"), "dek pevar")
        self.assertEqual(num2words(15, lang="br"), "dek pemp")
        self.assertEqual(num2words(16, lang="br"), "dek c'hwec'h")
        self.assertEqual(num2words(17, lang="br"), "dek seizh")
        self.assertEqual(num2words(18, lang="br"), "dek eizh")
        self.assertEqual(num2words(19, lang="br"), "dek nav")
        self.assertEqual(num2words(20, lang="br"), "ugent")
        self.assertEqual(num2words(21, lang="br"), "ugent unan")
        self.assertEqual(num2words(22, lang="br"), "ugent daou")
        self.assertEqual(num2words(23, lang="br"), "ugent tri")
        self.assertEqual(num2words(24, lang="br"), "ugent pevar")
        self.assertEqual(num2words(25, lang="br"), "ugent pemp")
        self.assertEqual(num2words(26, lang="br"), "ugent c'hwec'h")
        self.assertEqual(num2words(27, lang="br"), "ugent seizh")
        self.assertEqual(num2words(28, lang="br"), "ugent eizh")
        self.assertEqual(num2words(29, lang="br"), "ugent nav")
        self.assertEqual(num2words(30, lang="br"), "tregont")
        self.assertEqual(num2words(31, lang="br"), "tregont unan")
        self.assertEqual(num2words(35, lang="br"), "tregont pemp")
        self.assertEqual(num2words(40, lang="br"), "daou-ugent")
        self.assertEqual(num2words(45, lang="br"), "daou-ugent pemp")
        self.assertEqual(num2words(50, lang="br"), "hanter-kant")
        self.assertEqual(num2words(55, lang="br"), "hanter-kant pemp")
        self.assertEqual(num2words(60, lang="br"), "tri-ugent")
        self.assertEqual(num2words(65, lang="br"), "tri-ugent pemp")
        self.assertEqual(num2words(70, lang="br"), "dek ha tri-ugent")
        self.assertEqual(num2words(75, lang="br"), "dek ha tri-ugent pemp")
        self.assertEqual(num2words(80, lang="br"), "pevar-ugent")
        self.assertEqual(num2words(85, lang="br"), "pevar-ugent pemp")
        self.assertEqual(num2words(90, lang="br"), "dek ha pevar-ugent")
        self.assertEqual(num2words(95, lang="br"), "dek ha pevar-ugent pemp")
        self.assertEqual(num2words(99, lang="br"), "dek ha pevar-ugent nav")
        self.assertEqual(num2words(100, lang="br"), "unan kant")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="br"), "unan kant unan")
        self.assertEqual(num2words(110, lang="br"), "unan kant dek")
        self.assertEqual(num2words(111, lang="br"), "unan kant dek unan")
        self.assertEqual(num2words(120, lang="br"), "unan kant ugent")
        self.assertEqual(num2words(125, lang="br"), "unan kant ugent pemp")
        self.assertEqual(num2words(150, lang="br"), "unan kant hanter-kant")
        self.assertEqual(num2words(175, lang="br"), "unan kant dek ha tri-ugent pemp")
        self.assertEqual(num2words(199, lang="br"), "unan kant dek ha pevar-ugent nav")
        self.assertEqual(num2words(200, lang="br"), "daou kant")
        self.assertEqual(num2words(201, lang="br"), "daou kant unan")
        self.assertEqual(num2words(210, lang="br"), "daou kant dek")
        self.assertEqual(num2words(220, lang="br"), "daou kant ugent")
        self.assertEqual(num2words(250, lang="br"), "daou kant hanter-kant")
        self.assertEqual(num2words(299, lang="br"), "daou kant dek ha pevar-ugent nav")
        self.assertEqual(num2words(300, lang="br"), "tri kant")
        self.assertEqual(num2words(333, lang="br"), "tri kant tregont tri")
        self.assertEqual(num2words(400, lang="br"), "pevar kant")
        self.assertEqual(num2words(444, lang="br"), "pevar kant daou-ugent pevar")
        self.assertEqual(num2words(500, lang="br"), "pemp kant")
        self.assertEqual(num2words(555, lang="br"), "pemp kant hanter-kant pemp")
        self.assertEqual(num2words(600, lang="br"), "c'hwec'h kant")
        self.assertEqual(num2words(666, lang="br"), "c'hwec'h kant tri-ugent c'hwec'h")
        self.assertEqual(num2words(700, lang="br"), "seizh kant")
        self.assertEqual(num2words(777, lang="br"), "seizh kant dek ha tri-ugent seizh")
        self.assertEqual(num2words(800, lang="br"), "eizh kant")
        self.assertEqual(num2words(888, lang="br"), "eizh kant pevar-ugent eizh")
        self.assertEqual(num2words(900, lang="br"), "nav kant")
        self.assertEqual(num2words(999, lang="br"), "nav kant dek ha pevar-ugent nav")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="br"), "unan mil")
        self.assertEqual(num2words(1001, lang="br"), "unan mil unan")
        self.assertEqual(num2words(1010, lang="br"), "unan mil dek")
        self.assertEqual(num2words(1100, lang="br"), "unan mil unan kant")
        self.assertEqual(num2words(1111, lang="br"), "unan mil unan kant dek unan")
        self.assertEqual(num2words(1234, lang="br"), "unan mil daou kant tregont pevar")
        self.assertEqual(num2words(1500, lang="br"), "unan mil pemp kant")
        self.assertEqual(
            num2words(1999, lang="br"), "unan mil nav kant dek ha pevar-ugent nav"
        )
        self.assertEqual(num2words(2000, lang="br"), "daou mil")
        self.assertEqual(num2words(2001, lang="br"), "daou mil unan")
        self.assertEqual(num2words(2020, lang="br"), "daou mil ugent")
        self.assertEqual(num2words(2222, lang="br"), "daou mil daou kant ugent daou")
        self.assertEqual(num2words(3000, lang="br"), "tri mil")
        self.assertEqual(num2words(3333, lang="br"), "tri mil tri kant tregont tri")
        self.assertEqual(num2words(4000, lang="br"), "pevar mil")
        self.assertEqual(
            num2words(4444, lang="br"), "pevar mil pevar kant daou-ugent pevar"
        )
        self.assertEqual(num2words(5000, lang="br"), "pemp mil")
        self.assertEqual(
            num2words(5555, lang="br"), "pemp mil pemp kant hanter-kant pemp"
        )
        self.assertEqual(num2words(6000, lang="br"), "c'hwec'h mil")
        self.assertEqual(
            num2words(6666, lang="br"), "c'hwec'h mil c'hwec'h kant tri-ugent c'hwec'h"
        )
        self.assertEqual(num2words(7000, lang="br"), "seizh mil")
        self.assertEqual(
            num2words(7777, lang="br"), "seizh mil seizh kant dek ha tri-ugent seizh"
        )
        self.assertEqual(num2words(8000, lang="br"), "eizh mil")
        self.assertEqual(
            num2words(8888, lang="br"), "eizh mil eizh kant pevar-ugent eizh"
        )
        self.assertEqual(num2words(9000, lang="br"), "nav mil")
        self.assertEqual(
            num2words(9999, lang="br"), "nav mil nav kant dek ha pevar-ugent nav"
        )
        self.assertEqual(num2words(10000, lang="br"), "dek mil")
        self.assertEqual(num2words(10001, lang="br"), "dek mil unan")
        self.assertEqual(num2words(11111, lang="br"), "dek unan mil unan kant dek unan")
        self.assertEqual(
            num2words(12345, lang="br"), "dek daou mil tri kant daou-ugent pemp"
        )
        self.assertEqual(num2words(20000, lang="br"), "ugent mil")
        self.assertEqual(num2words(50000, lang="br"), "hanter-kant mil")
        self.assertEqual(
            num2words(99999, lang="br"),
            "dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav",
        )
        self.assertEqual(num2words(100000, lang="br"), "unan kant mil")
        self.assertEqual(
            num2words(123456, lang="br"),
            "unan kant ugent tri mil pevar kant hanter-kant c'hwec'h",
        )
        self.assertEqual(num2words(200000, lang="br"), "daou kant mil")
        self.assertEqual(num2words(500000, lang="br"), "pemp kant mil")
        self.assertEqual(
            num2words(654321, lang="br"),
            "c'hwec'h kant hanter-kant pevar mil tri kant ugent unan",
        )
        self.assertEqual(
            num2words(999999, lang="br"),
            "nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="br"), "unan milion")
        self.assertEqual(num2words(1000001, lang="br"), "unan milion unan")
        self.assertEqual(
            num2words(1111111, lang="br"),
            "unan milion unan kant dek unan mil unan kant dek unan",
        )
        self.assertEqual(
            num2words(1234567, lang="br"),
            "unan milion daou kant tregont pevar mil pemp kant tri-ugent seizh",
        )
        self.assertEqual(num2words(2000000, lang="br"), "daou milion")
        self.assertEqual(num2words(5000000, lang="br"), "pemp milion")
        self.assertEqual(
            num2words(9999999, lang="br"),
            "nav milion nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav",
        )
        self.assertEqual(num2words(10000000, lang="br"), "dek milion")
        self.assertEqual(
            num2words(12345678, lang="br"),
            "dek daou milion tri kant daou-ugent pemp mil c'hwec'h kant dek ha tri-ugent eizh",
        )
        self.assertEqual(
            num2words(99999999, lang="br"),
            "dek ha pevar-ugent nav milion nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav",
        )
        self.assertEqual(num2words(100000000, lang="br"), "unan kant milion")
        self.assertEqual(
            num2words(123456789, lang="br"),
            "unan kant ugent tri milion pevar kant hanter-kant c'hwec'h mil seizh kant pevar-ugent nav",
        )
        self.assertEqual(
            num2words(999999999, lang="br"),
            "nav kant dek ha pevar-ugent nav milion nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav",
        )
        self.assertEqual(num2words(1000000000, lang="br"), "unan miliard")
        self.assertEqual(num2words(1234567890, lang="br"), "unan miliard daou kant tregont pevar milion pemp kant tri-ugent seizh mil eizh kant dek ha pevar-ugent")
        self.assertEqual(num2words(9999999999, lang="br"), "nav miliard nav kant dek ha pevar-ugent nav milion nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav")
        self.assertEqual(num2words(10000000000, lang="br"), "dek miliard")
        self.assertEqual(num2words(99999999999, lang="br"), "dek ha pevar-ugent nav miliard nav kant dek ha pevar-ugent nav milion nav kant dek ha pevar-ugent nav mil nav kant dek ha pevar-ugent nav")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="br"), "lei unan")
        self.assertEqual(num2words(-2, lang="br"), "lei daou")
        self.assertEqual(num2words(-5, lang="br"), "lei pemp")
        self.assertEqual(num2words(-10, lang="br"), "lei dek")
        self.assertEqual(num2words(-11, lang="br"), "lei dek unan")
        self.assertEqual(num2words(-20, lang="br"), "lei ugent")
        self.assertEqual(num2words(-50, lang="br"), "lei hanter-kant")
        self.assertEqual(num2words(-99, lang="br"), "lei dek ha pevar-ugent nav")
        self.assertEqual(num2words(-100, lang="br"), "lei unan kant")
        self.assertEqual(num2words(-101, lang="br"), "lei unan kant unan")
        self.assertEqual(num2words(-200, lang="br"), "lei daou kant")
        self.assertEqual(
            num2words(-999, lang="br"), "lei nav kant dek ha pevar-ugent nav"
        )
        self.assertEqual(num2words(-1000, lang="br"), "lei unan mil")
        self.assertEqual(num2words(-1001, lang="br"), "lei unan mil unan")
        self.assertEqual(num2words(-10000, lang="br"), "lei dek mil")
        self.assertEqual(num2words(-100000, lang="br"), "lei unan kant mil")
        self.assertEqual(num2words(-1000000, lang="br"), "lei unan milion")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="br"), "mann skej unan")
        self.assertEqual(num2words(0.5, lang="br"), "mann skej pemp")
        self.assertEqual(num2words(0.9, lang="br"), "mann skej nav")
        self.assertEqual(num2words(1.1, lang="br"), "unan skej unan")
        self.assertEqual(num2words(1.5, lang="br"), "unan skej pemp")
        self.assertEqual(num2words(2.5, lang="br"), "daou skej pemp")
        self.assertEqual(num2words(3.14, lang="br"), "tri skej unan pevar")
        self.assertEqual(num2words(10.5, lang="br"), "dek skej pemp")
        self.assertEqual(num2words(11.11, lang="br"), "dek unan skej unan unan")
        self.assertEqual(num2words(20.2, lang="br"), "ugent skej daou")
        self.assertEqual(
            num2words(99.99, lang="br"), "dek ha pevar-ugent nav skej nav nav"
        )
        self.assertEqual(num2words(100.01, lang="br"), "unan kant skej mann unan")
        self.assertEqual(num2words(100.5, lang="br"), "unan kant skej pemp")
        self.assertEqual(
            num2words(123.45, lang="br"), "unan kant ugent tri skej pevar pemp"
        )
        self.assertEqual(num2words(1000.5, lang="br"), "unan mil skej pemp")
        self.assertEqual(
            num2words(1234.56, lang="br"),
            "unan mil daou kant tregont pevar skej pemp c'hwec'h",
        )
        self.assertEqual(num2words(10000.01, lang="br"), "dek mil skej mann unan")
        self.assertEqual(num2words(-0.5, lang="br"), "lei mann skej pemp")
        self.assertEqual(num2words(-1.5, lang="br"), "lei unan skej pemp")
        self.assertEqual(num2words(-10.5, lang="br"), "lei dek skej pemp")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="br", ordinal=True), "unan-vet")
        self.assertEqual(num2words(2, lang="br", ordinal=True), "daou-vet")
        self.assertEqual(num2words(3, lang="br", ordinal=True), "tri-vet")
        self.assertEqual(num2words(4, lang="br", ordinal=True), "pevar-vet")
        self.assertEqual(num2words(5, lang="br", ordinal=True), "pemp-vet")
        self.assertEqual(num2words(6, lang="br", ordinal=True), "c'hwec'h-vet")
        self.assertEqual(num2words(7, lang="br", ordinal=True), "seizh-vet")
        self.assertEqual(num2words(8, lang="br", ordinal=True), "eizh-vet")
        self.assertEqual(num2words(9, lang="br", ordinal=True), "nav-vet")
        self.assertEqual(num2words(10, lang="br", ordinal=True), "dek-vet")
        self.assertEqual(num2words(11, lang="br", ordinal=True), "dek unan-vet")
        self.assertEqual(num2words(12, lang="br", ordinal=True), "dek daou-vet")
        self.assertEqual(num2words(13, lang="br", ordinal=True), "dek tri-vet")
        self.assertEqual(num2words(14, lang="br", ordinal=True), "dek pevar-vet")
        self.assertEqual(num2words(15, lang="br", ordinal=True), "dek pemp-vet")
        self.assertEqual(num2words(16, lang="br", ordinal=True), "dek c'hwec'h-vet")
        self.assertEqual(num2words(17, lang="br", ordinal=True), "dek seizh-vet")
        self.assertEqual(num2words(18, lang="br", ordinal=True), "dek eizh-vet")
        self.assertEqual(num2words(19, lang="br", ordinal=True), "dek nav-vet")
        self.assertEqual(num2words(20, lang="br", ordinal=True), "ugent-vet")
        self.assertEqual(num2words(21, lang="br", ordinal=True), "ugent unan-vet")
        self.assertEqual(num2words(22, lang="br", ordinal=True), "ugent daou-vet")
        self.assertEqual(num2words(25, lang="br", ordinal=True), "ugent pemp-vet")
        self.assertEqual(num2words(30, lang="br", ordinal=True), "tregont-vet")
        self.assertEqual(num2words(40, lang="br", ordinal=True), "daou-ugent-vet")
        self.assertEqual(num2words(50, lang="br", ordinal=True), "hanter-kant-vet")
        self.assertEqual(num2words(60, lang="br", ordinal=True), "tri-ugent-vet")
        self.assertEqual(num2words(70, lang="br", ordinal=True), "dek ha tri-ugent-vet")
        self.assertEqual(num2words(80, lang="br", ordinal=True), "pevar-ugent-vet")
        self.assertEqual(
            num2words(90, lang="br", ordinal=True), "dek ha pevar-ugent-vet"
        )
        self.assertEqual(num2words(100, lang="br", ordinal=True), "unan kant-vet")
        self.assertEqual(num2words(101, lang="br", ordinal=True), "unan kant unan-vet")
        self.assertEqual(num2words(200, lang="br", ordinal=True), "daou kant-vet")
        self.assertEqual(num2words(500, lang="br", ordinal=True), "pemp kant-vet")
        self.assertEqual(num2words(1000, lang="br", ordinal=True), "unan mil-vet")
        self.assertEqual(num2words(1001, lang="br", ordinal=True), "unan mil unan-vet")
        self.assertEqual(num2words(10000, lang="br", ordinal=True), "dek mil-vet")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="br", to="currency", currency="EUR"), "mann euroioù"
        )
        self.assertEqual(
            num2words(0.01, lang="br", to="currency", currency="EUR"),
            "mann euroioù unan sentim",
        )
        self.assertEqual(
            num2words(0.5, lang="br", to="currency", currency="EUR"),
            "mann euroioù hanter-kant sentimoù",
        )
        self.assertEqual(
            num2words(1, lang="br", to="currency", currency="EUR"), "unan euro"
        )
        self.assertEqual(
            num2words(1.5, lang="br", to="currency", currency="EUR"),
            "unan euro hanter-kant sentimoù",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="br", to="currency", currency="USD")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="br", to="year"), "unan mil")
        self.assertEqual(
            num2words(1066, lang="br", to="year"), "unan mil tri-ugent c'hwec'h"
        )
        self.assertEqual(
            num2words(1492, lang="br", to="year"),
            "unan mil pevar kant dek ha pevar-ugent daou",
        )
        self.assertEqual(
            num2words(1776, lang="br", to="year"),
            "unan mil seizh kant dek ha tri-ugent c'hwec'h",
        )
        self.assertEqual(num2words(1800, lang="br", to="year"), "unan mil eizh kant")
        self.assertEqual(num2words(1900, lang="br", to="year"), "unan mil nav kant")
        self.assertEqual(
            num2words(1984, lang="br", to="year"), "unan mil nav kant pevar-ugent pevar"
        )
        self.assertEqual(
            num2words(1999, lang="br", to="year"),
            "unan mil nav kant dek ha pevar-ugent nav",
        )
        self.assertEqual(num2words(2000, lang="br", to="year"), "daou mil")
        self.assertEqual(num2words(2001, lang="br", to="year"), "daou mil unan")
        self.assertEqual(num2words(2010, lang="br", to="year"), "daou mil dek")
        self.assertEqual(num2words(2020, lang="br", to="year"), "daou mil ugent")
        self.assertEqual(num2words(2024, lang="br", to="year"), "daou mil ugent pevar")
        self.assertEqual(num2words(2100, lang="br", to="year"), "daou mil unan kant")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="br"), "mann")
        self.assertEqual(num2words("1", lang="br"), "unan")
        self.assertEqual(num2words("10", lang="br"), "dek")
        self.assertEqual(num2words("100", lang="br"), "unan kant")
        self.assertEqual(num2words("1000", lang="br"), "unan mil")
        self.assertEqual(num2words("10000", lang="br"), "dek mil")
        self.assertEqual(num2words("100000", lang="br"), "unan kant mil")
        self.assertEqual(num2words("1000000", lang="br"), "unan milion")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="br"), "mann")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="br"), num2words("100", lang="br"))
        self.assertEqual(num2words(1000, lang="br"), num2words("1000", lang="br"))

