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


class Num2WordsNNTest(TestCase):
    """Comprehensive test cases for Norwegian Nynorsk language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="nn"), "null")
        self.assertEqual(num2words(1, lang="nn"), "ein")
        self.assertEqual(num2words(2, lang="nn"), "to")
        self.assertEqual(num2words(3, lang="nn"), "tre")
        self.assertEqual(num2words(4, lang="nn"), "fire")
        self.assertEqual(num2words(5, lang="nn"), "fem")
        self.assertEqual(num2words(6, lang="nn"), "seks")
        self.assertEqual(num2words(7, lang="nn"), "sju")
        self.assertEqual(num2words(8, lang="nn"), "åtte")
        self.assertEqual(num2words(9, lang="nn"), "ni")
        self.assertEqual(num2words(10, lang="nn"), "ti")
        self.assertEqual(num2words(11, lang="nn"), "elleve")
        self.assertEqual(num2words(12, lang="nn"), "tolv")
        self.assertEqual(num2words(13, lang="nn"), "tretten")
        self.assertEqual(num2words(14, lang="nn"), "fjorten")
        self.assertEqual(num2words(15, lang="nn"), "femten")
        self.assertEqual(num2words(16, lang="nn"), "seksten")
        self.assertEqual(num2words(17, lang="nn"), "sytten")
        self.assertEqual(num2words(18, lang="nn"), "atten")
        self.assertEqual(num2words(19, lang="nn"), "nitten")
        self.assertEqual(num2words(20, lang="nn"), "tjue")
        self.assertEqual(num2words(21, lang="nn"), "tjue ein")
        self.assertEqual(num2words(22, lang="nn"), "tjue to")
        self.assertEqual(num2words(23, lang="nn"), "tjue tre")
        self.assertEqual(num2words(24, lang="nn"), "tjue fire")
        self.assertEqual(num2words(25, lang="nn"), "tjue fem")
        self.assertEqual(num2words(26, lang="nn"), "tjue seks")
        self.assertEqual(num2words(27, lang="nn"), "tjue sju")
        self.assertEqual(num2words(28, lang="nn"), "tjue åtte")
        self.assertEqual(num2words(29, lang="nn"), "tjue ni")
        self.assertEqual(num2words(30, lang="nn"), "tretti")
        self.assertEqual(num2words(31, lang="nn"), "tretti ein")
        self.assertEqual(num2words(35, lang="nn"), "tretti fem")
        self.assertEqual(num2words(40, lang="nn"), "førti")
        self.assertEqual(num2words(45, lang="nn"), "førti fem")
        self.assertEqual(num2words(50, lang="nn"), "femti")
        self.assertEqual(num2words(55, lang="nn"), "femti fem")
        self.assertEqual(num2words(60, lang="nn"), "seksti")
        self.assertEqual(num2words(65, lang="nn"), "seksti fem")
        self.assertEqual(num2words(70, lang="nn"), "sytti")
        self.assertEqual(num2words(75, lang="nn"), "sytti fem")
        self.assertEqual(num2words(80, lang="nn"), "åtti")
        self.assertEqual(num2words(85, lang="nn"), "åtti fem")
        self.assertEqual(num2words(90, lang="nn"), "nitti")
        self.assertEqual(num2words(95, lang="nn"), "nitti fem")
        self.assertEqual(num2words(99, lang="nn"), "nitti ni")
        self.assertEqual(num2words(100, lang="nn"), "ein hundre")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="nn"), "ein hundre ein")
        self.assertEqual(num2words(110, lang="nn"), "ein hundre ti")
        self.assertEqual(num2words(111, lang="nn"), "ein hundre elleve")
        self.assertEqual(num2words(120, lang="nn"), "ein hundre tjue")
        self.assertEqual(num2words(125, lang="nn"), "ein hundre tjue fem")
        self.assertEqual(num2words(150, lang="nn"), "ein hundre femti")
        self.assertEqual(num2words(175, lang="nn"), "ein hundre sytti fem")
        self.assertEqual(num2words(199, lang="nn"), "ein hundre nitti ni")
        self.assertEqual(num2words(200, lang="nn"), "to hundre")
        self.assertEqual(num2words(201, lang="nn"), "to hundre ein")
        self.assertEqual(num2words(210, lang="nn"), "to hundre ti")
        self.assertEqual(num2words(220, lang="nn"), "to hundre tjue")
        self.assertEqual(num2words(250, lang="nn"), "to hundre femti")
        self.assertEqual(num2words(299, lang="nn"), "to hundre nitti ni")
        self.assertEqual(num2words(300, lang="nn"), "tre hundre")
        self.assertEqual(num2words(333, lang="nn"), "tre hundre tretti tre")
        self.assertEqual(num2words(400, lang="nn"), "fire hundre")
        self.assertEqual(num2words(444, lang="nn"), "fire hundre førti fire")
        self.assertEqual(num2words(500, lang="nn"), "fem hundre")
        self.assertEqual(num2words(555, lang="nn"), "fem hundre femti fem")
        self.assertEqual(num2words(600, lang="nn"), "seks hundre")
        self.assertEqual(num2words(666, lang="nn"), "seks hundre seksti seks")
        self.assertEqual(num2words(700, lang="nn"), "sju hundre")
        self.assertEqual(num2words(777, lang="nn"), "sju hundre sytti sju")
        self.assertEqual(num2words(800, lang="nn"), "åtte hundre")
        self.assertEqual(num2words(888, lang="nn"), "åtte hundre åtti åtte")
        self.assertEqual(num2words(900, lang="nn"), "ni hundre")
        self.assertEqual(num2words(999, lang="nn"), "ni hundre nitti ni")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="nn"), "ein tusen")
        self.assertEqual(num2words(1001, lang="nn"), "ein tusen ein")
        self.assertEqual(num2words(1010, lang="nn"), "ein tusen ti")
        self.assertEqual(num2words(1100, lang="nn"), "ein tusen ein hundre")
        self.assertEqual(num2words(1111, lang="nn"), "ein tusen ein hundre elleve")
        self.assertEqual(num2words(1234, lang="nn"), "ein tusen to hundre tretti fire")
        self.assertEqual(num2words(1500, lang="nn"), "ein tusen fem hundre")
        self.assertEqual(num2words(1999, lang="nn"), "ein tusen ni hundre nitti ni")
        self.assertEqual(num2words(2000, lang="nn"), "to tusen")
        self.assertEqual(num2words(2001, lang="nn"), "to tusen ein")
        self.assertEqual(num2words(2020, lang="nn"), "to tusen tjue")
        self.assertEqual(num2words(2222, lang="nn"), "to tusen to hundre tjue to")
        self.assertEqual(num2words(3000, lang="nn"), "tre tusen")
        self.assertEqual(num2words(3333, lang="nn"), "tre tusen tre hundre tretti tre")
        self.assertEqual(num2words(4000, lang="nn"), "fire tusen")
        self.assertEqual(
            num2words(4444, lang="nn"), "fire tusen fire hundre førti fire"
        )
        self.assertEqual(num2words(5000, lang="nn"), "fem tusen")
        self.assertEqual(num2words(5555, lang="nn"), "fem tusen fem hundre femti fem")
        self.assertEqual(num2words(6000, lang="nn"), "seks tusen")
        self.assertEqual(
            num2words(6666, lang="nn"), "seks tusen seks hundre seksti seks"
        )
        self.assertEqual(num2words(7000, lang="nn"), "sju tusen")
        self.assertEqual(num2words(7777, lang="nn"), "sju tusen sju hundre sytti sju")
        self.assertEqual(num2words(8000, lang="nn"), "åtte tusen")
        self.assertEqual(num2words(8888, lang="nn"), "åtte tusen åtte hundre åtti åtte")
        self.assertEqual(num2words(9000, lang="nn"), "ni tusen")
        self.assertEqual(num2words(9999, lang="nn"), "ni tusen ni hundre nitti ni")
        self.assertEqual(num2words(10000, lang="nn"), "ti tusen")
        self.assertEqual(num2words(10001, lang="nn"), "ti tusen ein")
        self.assertEqual(num2words(11111, lang="nn"), "elleve tusen ein hundre elleve")
        self.assertEqual(
            num2words(12345, lang="nn"), "tolv tusen tre hundre førti fem"
        )
        self.assertEqual(num2words(20000, lang="nn"), "tjue tusen")
        self.assertEqual(num2words(50000, lang="nn"), "femti tusen")
        self.assertEqual(
            num2words(99999, lang="nn"), "nitti ni tusen ni hundre nitti ni"
        )
        self.assertEqual(num2words(100000, lang="nn"), "ein hundre tusen")
        self.assertEqual(
            num2words(123456, lang="nn"),
            "ein hundre tjue tre tusen fire hundre femti seks",
        )
        self.assertEqual(num2words(200000, lang="nn"), "to hundre tusen")
        self.assertEqual(num2words(500000, lang="nn"), "fem hundre tusen")
        self.assertEqual(
            num2words(654321, lang="nn"),
            "seks hundre femti fire tusen tre hundre tjue ein",
        )
        self.assertEqual(
            num2words(999999, lang="nn"), "ni hundre nitti ni tusen ni hundre nitti ni"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="nn"), "ein million")
        self.assertEqual(num2words(1000001, lang="nn"), "ein million ein")
        self.assertEqual(
            num2words(1111111, lang="nn"),
            "ein million ein hundre elleve tusen ein hundre elleve",
        )
        self.assertEqual(
            num2words(1234567, lang="nn"),
            "ein million to hundre tretti fire tusen fem hundre seksti sju",
        )
        self.assertEqual(num2words(2000000, lang="nn"), "to million")
        self.assertEqual(num2words(5000000, lang="nn"), "fem million")
        self.assertEqual(
            num2words(9999999, lang="nn"),
            "ni million ni hundre nitti ni tusen ni hundre nitti ni",
        )
        self.assertEqual(num2words(10000000, lang="nn"), "ti million")
        self.assertEqual(
            num2words(12345678, lang="nn"),
            "tolv million tre hundre førti fem tusen seks hundre sytti åtte",
        )
        self.assertEqual(
            num2words(99999999, lang="nn"),
            "nitti ni million ni hundre nitti ni tusen ni hundre nitti ni",
        )
        self.assertEqual(num2words(100000000, lang="nn"), "ein hundre million")
        self.assertEqual(
            num2words(123456789, lang="nn"),
            "ein hundre tjue tre million fire hundre femti seks tusen sju hundre åtti ni",
        )
        self.assertEqual(
            num2words(999999999, lang="nn"),
            "ni hundre nitti ni million ni hundre nitti ni tusen ni hundre nitti ni",
        )
        self.assertEqual(num2words(1000000000, lang="nn"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="nn"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="nn"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="nn"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="nn"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="nn"), "minus ein")
        self.assertEqual(num2words(-2, lang="nn"), "minus to")
        self.assertEqual(num2words(-5, lang="nn"), "minus fem")
        self.assertEqual(num2words(-10, lang="nn"), "minus ti")
        self.assertEqual(num2words(-11, lang="nn"), "minus elleve")
        self.assertEqual(num2words(-20, lang="nn"), "minus tjue")
        self.assertEqual(num2words(-50, lang="nn"), "minus femti")
        self.assertEqual(num2words(-99, lang="nn"), "minus nitti ni")
        self.assertEqual(num2words(-100, lang="nn"), "minus ein hundre")
        self.assertEqual(num2words(-101, lang="nn"), "minus ein hundre ein")
        self.assertEqual(num2words(-200, lang="nn"), "minus to hundre")
        self.assertEqual(num2words(-999, lang="nn"), "minus ni hundre nitti ni")
        self.assertEqual(num2words(-1000, lang="nn"), "minus ein tusen")
        self.assertEqual(num2words(-1001, lang="nn"), "minus ein tusen ein")
        self.assertEqual(num2words(-10000, lang="nn"), "minus ti tusen")
        self.assertEqual(num2words(-100000, lang="nn"), "minus ein hundre tusen")
        self.assertEqual(num2words(-1000000, lang="nn"), "minus ein million")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="nn"), "null komma ein")
        self.assertEqual(num2words(0.5, lang="nn"), "null komma fem")
        self.assertEqual(num2words(0.9, lang="nn"), "null komma ni")
        self.assertEqual(num2words(1.1, lang="nn"), "ein komma ein")
        self.assertEqual(num2words(1.5, lang="nn"), "ein komma fem")
        self.assertEqual(num2words(2.5, lang="nn"), "to komma fem")
        self.assertEqual(num2words(3.14, lang="nn"), "tre komma ein fire")
        self.assertEqual(num2words(10.5, lang="nn"), "ti komma fem")
        self.assertEqual(num2words(11.11, lang="nn"), "elleve komma ein ein")
        self.assertEqual(num2words(20.2, lang="nn"), "tjue komma to")
        self.assertEqual(num2words(99.99, lang="nn"), "nitti ni komma ni ni")
        self.assertEqual(num2words(100.01, lang="nn"), "ein hundre komma null ein")
        self.assertEqual(num2words(100.5, lang="nn"), "ein hundre komma fem")
        self.assertEqual(
            num2words(123.45, lang="nn"), "ein hundre tjue tre komma fire fem"
        )
        self.assertEqual(num2words(1000.5, lang="nn"), "ein tusen komma fem")
        self.assertEqual(
            num2words(1234.56, lang="nn"),
            "ein tusen to hundre tretti fire komma fem seks",
        )
        self.assertEqual(num2words(10000.01, lang="nn"), "ti tusen komma null ein")
        self.assertEqual(num2words(-0.5, lang="nn"), "minus null komma fem")
        self.assertEqual(num2words(-1.5, lang="nn"), "minus ein komma fem")
        self.assertEqual(num2words(-10.5, lang="nn"), "minus ti komma fem")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="nn", ordinal=True), "ein-de")
        self.assertEqual(num2words(2, lang="nn", ordinal=True), "to-de")
        self.assertEqual(num2words(3, lang="nn", ordinal=True), "tre-de")
        self.assertEqual(num2words(4, lang="nn", ordinal=True), "fire-de")
        self.assertEqual(num2words(5, lang="nn", ordinal=True), "fem-de")
        self.assertEqual(num2words(6, lang="nn", ordinal=True), "seks-de")
        self.assertEqual(num2words(7, lang="nn", ordinal=True), "sju-de")
        self.assertEqual(num2words(8, lang="nn", ordinal=True), "åtte-de")
        self.assertEqual(num2words(9, lang="nn", ordinal=True), "ni-de")
        self.assertEqual(num2words(10, lang="nn", ordinal=True), "ti-de")
        self.assertEqual(num2words(11, lang="nn", ordinal=True), "elleve-de")
        self.assertEqual(num2words(12, lang="nn", ordinal=True), "tolv-de")
        self.assertEqual(num2words(13, lang="nn", ordinal=True), "tretten-de")
        self.assertEqual(num2words(14, lang="nn", ordinal=True), "fjorten-de")
        self.assertEqual(num2words(15, lang="nn", ordinal=True), "femten-de")
        self.assertEqual(num2words(16, lang="nn", ordinal=True), "seksten-de")
        self.assertEqual(num2words(17, lang="nn", ordinal=True), "sytten-de")
        self.assertEqual(num2words(18, lang="nn", ordinal=True), "atten-de")
        self.assertEqual(num2words(19, lang="nn", ordinal=True), "nitten-de")
        self.assertEqual(num2words(20, lang="nn", ordinal=True), "tjue-de")
        self.assertEqual(num2words(21, lang="nn", ordinal=True), "tjue ein-de")
        self.assertEqual(num2words(22, lang="nn", ordinal=True), "tjue to-de")
        self.assertEqual(num2words(25, lang="nn", ordinal=True), "tjue fem-de")
        self.assertEqual(num2words(30, lang="nn", ordinal=True), "tretti-de")
        self.assertEqual(num2words(40, lang="nn", ordinal=True), "førti-de")
        self.assertEqual(num2words(50, lang="nn", ordinal=True), "femti-de")
        self.assertEqual(num2words(60, lang="nn", ordinal=True), "seksti-de")
        self.assertEqual(num2words(70, lang="nn", ordinal=True), "sytti-de")
        self.assertEqual(num2words(80, lang="nn", ordinal=True), "åtti-de")
        self.assertEqual(num2words(90, lang="nn", ordinal=True), "nitti-de")
        self.assertEqual(num2words(100, lang="nn", ordinal=True), "ein hundre-de")
        self.assertEqual(num2words(101, lang="nn", ordinal=True), "ein hundre ein-de")
        self.assertEqual(num2words(200, lang="nn", ordinal=True), "to hundre-de")
        self.assertEqual(num2words(500, lang="nn", ordinal=True), "fem hundre-de")
        self.assertEqual(num2words(1000, lang="nn", ordinal=True), "ein tusen-de")
        self.assertEqual(num2words(1001, lang="nn", ordinal=True), "ein tusen ein-de")
        self.assertEqual(num2words(10000, lang="nn", ordinal=True), "ti tusen-de")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="nn", to="currency", currency="NOK"), "null kroner"
        )
        self.assertEqual(
            num2words(0.01, lang="nn", to="currency", currency="NOK"),
            "null kroner ein øre",
        )
        self.assertEqual(
            num2words(0.5, lang="nn", to="currency", currency="NOK"),
            "null kroner femti øre",
        )
        self.assertEqual(
            num2words(1, lang="nn", to="currency", currency="NOK"), "ein krone"
        )
        self.assertEqual(
            num2words(1.5, lang="nn", to="currency", currency="NOK"),
            "ein krone femti øre",
        )
        self.assertEqual(
            num2words(0, lang="nn", to="currency", currency="USD"), "null dollars"
        )
        self.assertEqual(
            num2words(0.01, lang="nn", to="currency", currency="USD"),
            "null dollars ein cent",
        )
        self.assertEqual(
            num2words(0.5, lang="nn", to="currency", currency="USD"),
            "null dollars femti cents",
        )
        self.assertEqual(
            num2words(1, lang="nn", to="currency", currency="USD"), "ein dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="nn", to="currency", currency="USD"),
            "ein dollar femti cents",
        )
        self.assertEqual(
            num2words(0, lang="nn", to="currency", currency="EUR"), "null euros"
        )
        self.assertEqual(
            num2words(0.01, lang="nn", to="currency", currency="EUR"),
            "null euros ein cent",
        )
        self.assertEqual(
            num2words(0.5, lang="nn", to="currency", currency="EUR"),
            "null euros femti cents",
        )
        self.assertEqual(
            num2words(1, lang="nn", to="currency", currency="EUR"), "ein euro"
        )
        self.assertEqual(
            num2words(1.5, lang="nn", to="currency", currency="EUR"),
            "ein euro femti cents",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="nn", to="year"), "ein tusen")
        self.assertEqual(num2words(1066, lang="nn", to="year"), "ein tusen seksti seks")
        self.assertEqual(
            num2words(1492, lang="nn", to="year"), "ein tusen fire hundre nitti to"
        )
        self.assertEqual(
            num2words(1776, lang="nn", to="year"), "ein tusen sju hundre sytti seks"
        )
        self.assertEqual(num2words(1800, lang="nn", to="year"), "ein tusen åtte hundre")
        self.assertEqual(num2words(1900, lang="nn", to="year"), "ein tusen ni hundre")
        self.assertEqual(
            num2words(1984, lang="nn", to="year"), "ein tusen ni hundre åtti fire"
        )
        self.assertEqual(
            num2words(1999, lang="nn", to="year"), "ein tusen ni hundre nitti ni"
        )
        self.assertEqual(num2words(2000, lang="nn", to="year"), "to tusen")
        self.assertEqual(num2words(2001, lang="nn", to="year"), "to tusen ein")
        self.assertEqual(num2words(2010, lang="nn", to="year"), "to tusen ti")
        self.assertEqual(num2words(2020, lang="nn", to="year"), "to tusen tjue")
        self.assertEqual(num2words(2024, lang="nn", to="year"), "to tusen tjue fire")
        self.assertEqual(num2words(2100, lang="nn", to="year"), "to tusen ein hundre")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="nn"), "null")
        self.assertEqual(num2words("1", lang="nn"), "ein")
        self.assertEqual(num2words("10", lang="nn"), "ti")
        self.assertEqual(num2words("100", lang="nn"), "ein hundre")
        self.assertEqual(num2words("1000", lang="nn"), "ein tusen")
        self.assertEqual(num2words("10000", lang="nn"), "ti tusen")
        self.assertEqual(num2words("100000", lang="nn"), "ein hundre tusen")
        self.assertEqual(num2words("1000000", lang="nn"), "ein million")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="nn"), "null")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="nn"), num2words("100", lang="nn"))
        self.assertEqual(num2words(1000, lang="nn"), num2words("1000", lang="nn"))



def test_teens_issue_216():
    # gladiaio/num2words2#216: Python had no teens ("ti ein" for 11).
    from num2words2 import num2words
    want = ["elleve", "tolv", "tretten", "fjorten", "femten", "seksten",
            "sytten", "atten", "nitten"]
    assert [num2words(i, lang="nn") for i in range(11, 20)] == want
    assert num2words(12.13, lang="nn", to="currency", currency="NOK") == (
        "tolv kroner tretten øre")
