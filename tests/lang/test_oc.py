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


class Num2WordsOCTest(TestCase):
    """Comprehensive test cases for Occitan language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="oc"), "zèro")
        self.assertEqual(num2words(1, lang="oc"), "un")
        self.assertEqual(num2words(2, lang="oc"), "dos")
        self.assertEqual(num2words(3, lang="oc"), "tres")
        self.assertEqual(num2words(4, lang="oc"), "quatre")
        self.assertEqual(num2words(5, lang="oc"), "cinc")
        self.assertEqual(num2words(6, lang="oc"), "sièis")
        self.assertEqual(num2words(7, lang="oc"), "sèt")
        self.assertEqual(num2words(8, lang="oc"), "uèch")
        self.assertEqual(num2words(9, lang="oc"), "nòu")
        self.assertEqual(num2words(10, lang="oc"), "dètz")
        self.assertEqual(num2words(11, lang="oc"), "dètz un")
        self.assertEqual(num2words(12, lang="oc"), "dètz dos")
        self.assertEqual(num2words(13, lang="oc"), "dètz tres")
        self.assertEqual(num2words(14, lang="oc"), "dètz quatre")
        self.assertEqual(num2words(15, lang="oc"), "dètz cinc")
        self.assertEqual(num2words(16, lang="oc"), "dètz sièis")
        self.assertEqual(num2words(17, lang="oc"), "dètz sèt")
        self.assertEqual(num2words(18, lang="oc"), "dètz uèch")
        self.assertEqual(num2words(19, lang="oc"), "dètz nòu")
        self.assertEqual(num2words(20, lang="oc"), "vint")
        self.assertEqual(num2words(21, lang="oc"), "vint un")
        self.assertEqual(num2words(22, lang="oc"), "vint dos")
        self.assertEqual(num2words(23, lang="oc"), "vint tres")
        self.assertEqual(num2words(24, lang="oc"), "vint quatre")
        self.assertEqual(num2words(25, lang="oc"), "vint cinc")
        self.assertEqual(num2words(26, lang="oc"), "vint sièis")
        self.assertEqual(num2words(27, lang="oc"), "vint sèt")
        self.assertEqual(num2words(28, lang="oc"), "vint uèch")
        self.assertEqual(num2words(29, lang="oc"), "vint nòu")
        self.assertEqual(num2words(30, lang="oc"), "trenta")
        self.assertEqual(num2words(31, lang="oc"), "trenta un")
        self.assertEqual(num2words(35, lang="oc"), "trenta cinc")
        self.assertEqual(num2words(40, lang="oc"), "quaranta")
        self.assertEqual(num2words(45, lang="oc"), "quaranta cinc")
        self.assertEqual(num2words(50, lang="oc"), "cinquanta")
        self.assertEqual(num2words(55, lang="oc"), "cinquanta cinc")
        self.assertEqual(num2words(60, lang="oc"), "seissanta")
        self.assertEqual(num2words(65, lang="oc"), "seissanta cinc")
        self.assertEqual(num2words(70, lang="oc"), "setanta")
        self.assertEqual(num2words(75, lang="oc"), "setanta cinc")
        self.assertEqual(num2words(80, lang="oc"), "ochanta")
        self.assertEqual(num2words(85, lang="oc"), "ochanta cinc")
        self.assertEqual(num2words(90, lang="oc"), "nonanta")
        self.assertEqual(num2words(95, lang="oc"), "nonanta cinc")
        self.assertEqual(num2words(99, lang="oc"), "nonanta nòu")
        self.assertEqual(num2words(100, lang="oc"), "un cent")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="oc"), "un cent un")
        self.assertEqual(num2words(110, lang="oc"), "un cent dètz")
        self.assertEqual(num2words(111, lang="oc"), "un cent dètz un")
        self.assertEqual(num2words(120, lang="oc"), "un cent vint")
        self.assertEqual(num2words(125, lang="oc"), "un cent vint cinc")
        self.assertEqual(num2words(150, lang="oc"), "un cent cinquanta")
        self.assertEqual(num2words(175, lang="oc"), "un cent setanta cinc")
        self.assertEqual(num2words(199, lang="oc"), "un cent nonanta nòu")
        self.assertEqual(num2words(200, lang="oc"), "dos cent")
        self.assertEqual(num2words(201, lang="oc"), "dos cent un")
        self.assertEqual(num2words(210, lang="oc"), "dos cent dètz")
        self.assertEqual(num2words(220, lang="oc"), "dos cent vint")
        self.assertEqual(num2words(250, lang="oc"), "dos cent cinquanta")
        self.assertEqual(num2words(299, lang="oc"), "dos cent nonanta nòu")
        self.assertEqual(num2words(300, lang="oc"), "tres cent")
        self.assertEqual(num2words(333, lang="oc"), "tres cent trenta tres")
        self.assertEqual(num2words(400, lang="oc"), "quatre cent")
        self.assertEqual(num2words(444, lang="oc"), "quatre cent quaranta quatre")
        self.assertEqual(num2words(500, lang="oc"), "cinc cent")
        self.assertEqual(num2words(555, lang="oc"), "cinc cent cinquanta cinc")
        self.assertEqual(num2words(600, lang="oc"), "sièis cent")
        self.assertEqual(num2words(666, lang="oc"), "sièis cent seissanta sièis")
        self.assertEqual(num2words(700, lang="oc"), "sèt cent")
        self.assertEqual(num2words(777, lang="oc"), "sèt cent setanta sèt")
        self.assertEqual(num2words(800, lang="oc"), "uèch cent")
        self.assertEqual(num2words(888, lang="oc"), "uèch cent ochanta uèch")
        self.assertEqual(num2words(900, lang="oc"), "nòu cent")
        self.assertEqual(num2words(999, lang="oc"), "nòu cent nonanta nòu")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="oc"), "un mil")
        self.assertEqual(num2words(1001, lang="oc"), "un mil un")
        self.assertEqual(num2words(1010, lang="oc"), "un mil dètz")
        self.assertEqual(num2words(1100, lang="oc"), "un mil un cent")
        self.assertEqual(num2words(1111, lang="oc"), "un mil un cent dètz un")
        self.assertEqual(num2words(1234, lang="oc"), "un mil dos cent trenta quatre")
        self.assertEqual(num2words(1500, lang="oc"), "un mil cinc cent")
        self.assertEqual(num2words(1999, lang="oc"), "un mil nòu cent nonanta nòu")
        self.assertEqual(num2words(2000, lang="oc"), "dos mil")
        self.assertEqual(num2words(2001, lang="oc"), "dos mil un")
        self.assertEqual(num2words(2020, lang="oc"), "dos mil vint")
        self.assertEqual(num2words(2222, lang="oc"), "dos mil dos cent vint dos")
        self.assertEqual(num2words(3000, lang="oc"), "tres mil")
        self.assertEqual(num2words(3333, lang="oc"), "tres mil tres cent trenta tres")
        self.assertEqual(num2words(4000, lang="oc"), "quatre mil")
        self.assertEqual(
            num2words(4444, lang="oc"), "quatre mil quatre cent quaranta quatre"
        )
        self.assertEqual(num2words(5000, lang="oc"), "cinc mil")
        self.assertEqual(
            num2words(5555, lang="oc"), "cinc mil cinc cent cinquanta cinc"
        )
        self.assertEqual(num2words(6000, lang="oc"), "sièis mil")
        self.assertEqual(
            num2words(6666, lang="oc"), "sièis mil sièis cent seissanta sièis"
        )
        self.assertEqual(num2words(7000, lang="oc"), "sèt mil")
        self.assertEqual(num2words(7777, lang="oc"), "sèt mil sèt cent setanta sèt")
        self.assertEqual(num2words(8000, lang="oc"), "uèch mil")
        self.assertEqual(num2words(8888, lang="oc"), "uèch mil uèch cent ochanta uèch")
        self.assertEqual(num2words(9000, lang="oc"), "nòu mil")
        self.assertEqual(num2words(9999, lang="oc"), "nòu mil nòu cent nonanta nòu")
        self.assertEqual(num2words(10000, lang="oc"), "dètz mil")
        self.assertEqual(num2words(10001, lang="oc"), "dètz mil un")
        self.assertEqual(num2words(11111, lang="oc"), "dètz un mil un cent dètz un")
        self.assertEqual(
            num2words(12345, lang="oc"), "dètz dos mil tres cent quaranta cinc"
        )
        self.assertEqual(num2words(20000, lang="oc"), "vint mil")
        self.assertEqual(num2words(50000, lang="oc"), "cinquanta mil")
        self.assertEqual(
            num2words(99999, lang="oc"), "nonanta nòu mil nòu cent nonanta nòu"
        )
        self.assertEqual(num2words(100000, lang="oc"), "un cent mil")
        self.assertEqual(
            num2words(123456, lang="oc"),
            "un cent vint tres mil quatre cent cinquanta sièis",
        )
        self.assertEqual(num2words(200000, lang="oc"), "dos cent mil")
        self.assertEqual(num2words(500000, lang="oc"), "cinc cent mil")
        self.assertEqual(
            num2words(654321, lang="oc"),
            "sièis cent cinquanta quatre mil tres cent vint un",
        )
        self.assertEqual(
            num2words(999999, lang="oc"),
            "nòu cent nonanta nòu mil nòu cent nonanta nòu",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="oc"), "un milion")
        self.assertEqual(num2words(1000001, lang="oc"), "un milion un")
        self.assertEqual(
            num2words(1111111, lang="oc"),
            "un milion un cent dètz un mil un cent dètz un",
        )
        self.assertEqual(
            num2words(1234567, lang="oc"),
            "un milion dos cent trenta quatre mil cinc cent seissanta sèt",
        )
        self.assertEqual(num2words(2000000, lang="oc"), "dos milion")
        self.assertEqual(num2words(5000000, lang="oc"), "cinc milion")
        self.assertEqual(
            num2words(9999999, lang="oc"),
            "nòu milion nòu cent nonanta nòu mil nòu cent nonanta nòu",
        )
        self.assertEqual(num2words(10000000, lang="oc"), "dètz milion")
        self.assertEqual(
            num2words(12345678, lang="oc"),
            "dètz dos milion tres cent quaranta cinc mil sièis cent setanta uèch",
        )
        self.assertEqual(
            num2words(99999999, lang="oc"),
            "nonanta nòu milion nòu cent nonanta nòu mil nòu cent nonanta nòu",
        )
        self.assertEqual(num2words(100000000, lang="oc"), "un cent milion")
        self.assertEqual(
            num2words(123456789, lang="oc"),
            "un cent vint tres milion quatre cent cinquanta sièis mil sèt cent ochanta nòu",
        )
        self.assertEqual(
            num2words(999999999, lang="oc"),
            "nòu cent nonanta nòu milion nòu cent nonanta nòu mil nòu cent nonanta nòu",
        )
        self.assertEqual(num2words(1000000000, lang="oc"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="oc"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="oc"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="oc"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="oc"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="oc"), "mens un")
        self.assertEqual(num2words(-2, lang="oc"), "mens dos")
        self.assertEqual(num2words(-5, lang="oc"), "mens cinc")
        self.assertEqual(num2words(-10, lang="oc"), "mens dètz")
        self.assertEqual(num2words(-11, lang="oc"), "mens dètz un")
        self.assertEqual(num2words(-20, lang="oc"), "mens vint")
        self.assertEqual(num2words(-50, lang="oc"), "mens cinquanta")
        self.assertEqual(num2words(-99, lang="oc"), "mens nonanta nòu")
        self.assertEqual(num2words(-100, lang="oc"), "mens un cent")
        self.assertEqual(num2words(-101, lang="oc"), "mens un cent un")
        self.assertEqual(num2words(-200, lang="oc"), "mens dos cent")
        self.assertEqual(num2words(-999, lang="oc"), "mens nòu cent nonanta nòu")
        self.assertEqual(num2words(-1000, lang="oc"), "mens un mil")
        self.assertEqual(num2words(-1001, lang="oc"), "mens un mil un")
        self.assertEqual(num2words(-10000, lang="oc"), "mens dètz mil")
        self.assertEqual(num2words(-100000, lang="oc"), "mens un cent mil")
        self.assertEqual(num2words(-1000000, lang="oc"), "mens un milion")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="oc"), "zèro virgula un")
        self.assertEqual(num2words(0.5, lang="oc"), "zèro virgula cinc")
        self.assertEqual(num2words(0.9, lang="oc"), "zèro virgula nòu")
        self.assertEqual(num2words(1.1, lang="oc"), "un virgula un")
        self.assertEqual(num2words(1.5, lang="oc"), "un virgula cinc")
        self.assertEqual(num2words(2.5, lang="oc"), "dos virgula cinc")
        self.assertEqual(num2words(3.14, lang="oc"), "tres virgula un quatre")
        self.assertEqual(num2words(10.5, lang="oc"), "dètz virgula cinc")
        self.assertEqual(num2words(11.11, lang="oc"), "dètz un virgula un un")
        self.assertEqual(num2words(20.2, lang="oc"), "vint virgula dos")
        self.assertEqual(num2words(99.99, lang="oc"), "nonanta nòu virgula nòu nòu")
        self.assertEqual(num2words(100.01, lang="oc"), "un cent virgula zèro un")
        self.assertEqual(num2words(100.5, lang="oc"), "un cent virgula cinc")
        self.assertEqual(
            num2words(123.45, lang="oc"), "un cent vint tres virgula quatre cinc"
        )
        self.assertEqual(num2words(1000.5, lang="oc"), "un mil virgula cinc")
        self.assertEqual(
            num2words(1234.56, lang="oc"),
            "un mil dos cent trenta quatre virgula cinc sièis",
        )
        self.assertEqual(num2words(10000.01, lang="oc"), "dètz mil virgula zèro un")
        self.assertEqual(num2words(-0.5, lang="oc"), "mens zèro virgula cinc")
        self.assertEqual(num2words(-1.5, lang="oc"), "mens un virgula cinc")
        self.assertEqual(num2words(-10.5, lang="oc"), "mens dètz virgula cinc")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="oc", ordinal=True), "un-en")
        self.assertEqual(num2words(2, lang="oc", ordinal=True), "dos-en")
        self.assertEqual(num2words(3, lang="oc", ordinal=True), "tres-en")
        self.assertEqual(num2words(4, lang="oc", ordinal=True), "quatre-en")
        self.assertEqual(num2words(5, lang="oc", ordinal=True), "cinc-en")
        self.assertEqual(num2words(6, lang="oc", ordinal=True), "sièis-en")
        self.assertEqual(num2words(7, lang="oc", ordinal=True), "sèt-en")
        self.assertEqual(num2words(8, lang="oc", ordinal=True), "uèch-en")
        self.assertEqual(num2words(9, lang="oc", ordinal=True), "nòu-en")
        self.assertEqual(num2words(10, lang="oc", ordinal=True), "dètz-en")
        self.assertEqual(num2words(11, lang="oc", ordinal=True), "dètz un-en")
        self.assertEqual(num2words(12, lang="oc", ordinal=True), "dètz dos-en")
        self.assertEqual(num2words(13, lang="oc", ordinal=True), "dètz tres-en")
        self.assertEqual(num2words(14, lang="oc", ordinal=True), "dètz quatre-en")
        self.assertEqual(num2words(15, lang="oc", ordinal=True), "dètz cinc-en")
        self.assertEqual(num2words(16, lang="oc", ordinal=True), "dètz sièis-en")
        self.assertEqual(num2words(17, lang="oc", ordinal=True), "dètz sèt-en")
        self.assertEqual(num2words(18, lang="oc", ordinal=True), "dètz uèch-en")
        self.assertEqual(num2words(19, lang="oc", ordinal=True), "dètz nòu-en")
        self.assertEqual(num2words(20, lang="oc", ordinal=True), "vint-en")
        self.assertEqual(num2words(21, lang="oc", ordinal=True), "vint un-en")
        self.assertEqual(num2words(22, lang="oc", ordinal=True), "vint dos-en")
        self.assertEqual(num2words(25, lang="oc", ordinal=True), "vint cinc-en")
        self.assertEqual(num2words(30, lang="oc", ordinal=True), "trenta-en")
        self.assertEqual(num2words(40, lang="oc", ordinal=True), "quaranta-en")
        self.assertEqual(num2words(50, lang="oc", ordinal=True), "cinquanta-en")
        self.assertEqual(num2words(60, lang="oc", ordinal=True), "seissanta-en")
        self.assertEqual(num2words(70, lang="oc", ordinal=True), "setanta-en")
        self.assertEqual(num2words(80, lang="oc", ordinal=True), "ochanta-en")
        self.assertEqual(num2words(90, lang="oc", ordinal=True), "nonanta-en")
        self.assertEqual(num2words(100, lang="oc", ordinal=True), "un cent-en")
        self.assertEqual(num2words(101, lang="oc", ordinal=True), "un cent un-en")
        self.assertEqual(num2words(200, lang="oc", ordinal=True), "dos cent-en")
        self.assertEqual(num2words(500, lang="oc", ordinal=True), "cinc cent-en")
        self.assertEqual(num2words(1000, lang="oc", ordinal=True), "un mil-en")
        self.assertEqual(num2words(1001, lang="oc", ordinal=True), "un mil un-en")
        self.assertEqual(num2words(10000, lang="oc", ordinal=True), "dètz mil-en")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="oc", to="currency", currency="EUR"), "zèro èuros"
        )
        self.assertEqual(
            num2words(0.01, lang="oc", to="currency", currency="EUR"),
            "zèro èuros un centim",
        )
        self.assertEqual(
            num2words(0.5, lang="oc", to="currency", currency="EUR"),
            "zèro èuros cinquanta centims",
        )
        self.assertEqual(
            num2words(1, lang="oc", to="currency", currency="EUR"), "un èuro"
        )
        self.assertEqual(
            num2words(1.5, lang="oc", to="currency", currency="EUR"),
            "un èuro cinquanta centims",
        )
        self.assertEqual(
            num2words(0, lang="oc", to="currency", currency="USD"), "zèro dollars"
        )
        self.assertEqual(
            num2words(0.01, lang="oc", to="currency", currency="USD"),
            "zèro dollars un cent",
        )
        self.assertEqual(
            num2words(0.5, lang="oc", to="currency", currency="USD"),
            "zèro dollars cinquanta cents",
        )
        self.assertEqual(
            num2words(1, lang="oc", to="currency", currency="USD"), "un dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="oc", to="currency", currency="USD"),
            "un dollar cinquanta cents",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="oc", to="year"), "un mil")
        self.assertEqual(
            num2words(1066, lang="oc", to="year"), "un mil seissanta sièis"
        )
        self.assertEqual(
            num2words(1492, lang="oc", to="year"), "un mil quatre cent nonanta dos"
        )
        self.assertEqual(
            num2words(1776, lang="oc", to="year"), "un mil sèt cent setanta sièis"
        )
        self.assertEqual(num2words(1800, lang="oc", to="year"), "un mil uèch cent")
        self.assertEqual(num2words(1900, lang="oc", to="year"), "un mil nòu cent")
        self.assertEqual(
            num2words(1984, lang="oc", to="year"), "un mil nòu cent ochanta quatre"
        )
        self.assertEqual(
            num2words(1999, lang="oc", to="year"), "un mil nòu cent nonanta nòu"
        )
        self.assertEqual(num2words(2000, lang="oc", to="year"), "dos mil")
        self.assertEqual(num2words(2001, lang="oc", to="year"), "dos mil un")
        self.assertEqual(num2words(2010, lang="oc", to="year"), "dos mil dètz")
        self.assertEqual(num2words(2020, lang="oc", to="year"), "dos mil vint")
        self.assertEqual(num2words(2024, lang="oc", to="year"), "dos mil vint quatre")
        self.assertEqual(num2words(2100, lang="oc", to="year"), "dos mil un cent")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="oc"), "zèro")
        self.assertEqual(num2words("1", lang="oc"), "un")
        self.assertEqual(num2words("10", lang="oc"), "dètz")
        self.assertEqual(num2words("100", lang="oc"), "un cent")
        self.assertEqual(num2words("1000", lang="oc"), "un mil")
        self.assertEqual(num2words("10000", lang="oc"), "dètz mil")
        self.assertEqual(num2words("100000", lang="oc"), "un cent mil")
        self.assertEqual(num2words("1000000", lang="oc"), "un milion")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="oc"), "zèro")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="oc"), num2words("100", lang="oc"))
        self.assertEqual(num2words(1000, lang="oc"), num2words("1000", lang="oc"))

