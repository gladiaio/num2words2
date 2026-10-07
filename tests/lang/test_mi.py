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


class Num2WordsMITest(TestCase):
    """Comprehensive test cases for Māori language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="mi"), "zero")
        self.assertEqual(num2words(1, lang="mi"), "tahi")
        self.assertEqual(num2words(2, lang="mi"), "rua")
        self.assertEqual(num2words(3, lang="mi"), "toru")
        self.assertEqual(num2words(4, lang="mi"), "whā")
        self.assertEqual(num2words(5, lang="mi"), "rima")
        self.assertEqual(num2words(6, lang="mi"), "ono")
        self.assertEqual(num2words(7, lang="mi"), "whitu")
        self.assertEqual(num2words(8, lang="mi"), "waru")
        self.assertEqual(num2words(9, lang="mi"), "iwa")
        self.assertEqual(num2words(10, lang="mi"), "tekau")
        self.assertEqual(num2words(11, lang="mi"), "tekau tahi")
        self.assertEqual(num2words(12, lang="mi"), "tekau rua")
        self.assertEqual(num2words(13, lang="mi"), "tekau toru")
        self.assertEqual(num2words(14, lang="mi"), "tekau whā")
        self.assertEqual(num2words(15, lang="mi"), "tekau rima")
        self.assertEqual(num2words(16, lang="mi"), "tekau ono")
        self.assertEqual(num2words(17, lang="mi"), "tekau whitu")
        self.assertEqual(num2words(18, lang="mi"), "tekau waru")
        self.assertEqual(num2words(19, lang="mi"), "tekau iwa")
        self.assertEqual(num2words(20, lang="mi"), "rua tekau")
        self.assertEqual(num2words(21, lang="mi"), "rua tekau tahi")
        self.assertEqual(num2words(22, lang="mi"), "rua tekau rua")
        self.assertEqual(num2words(23, lang="mi"), "rua tekau toru")
        self.assertEqual(num2words(24, lang="mi"), "rua tekau whā")
        self.assertEqual(num2words(25, lang="mi"), "rua tekau rima")
        self.assertEqual(num2words(26, lang="mi"), "rua tekau ono")
        self.assertEqual(num2words(27, lang="mi"), "rua tekau whitu")
        self.assertEqual(num2words(28, lang="mi"), "rua tekau waru")
        self.assertEqual(num2words(29, lang="mi"), "rua tekau iwa")
        self.assertEqual(num2words(30, lang="mi"), "toru tekau")
        self.assertEqual(num2words(31, lang="mi"), "toru tekau tahi")
        self.assertEqual(num2words(35, lang="mi"), "toru tekau rima")
        self.assertEqual(num2words(40, lang="mi"), "whā tekau")
        self.assertEqual(num2words(45, lang="mi"), "whā tekau rima")
        self.assertEqual(num2words(50, lang="mi"), "rima tekau")
        self.assertEqual(num2words(55, lang="mi"), "rima tekau rima")
        self.assertEqual(num2words(60, lang="mi"), "ono tekau")
        self.assertEqual(num2words(65, lang="mi"), "ono tekau rima")
        self.assertEqual(num2words(70, lang="mi"), "whitu tekau")
        self.assertEqual(num2words(75, lang="mi"), "whitu tekau rima")
        self.assertEqual(num2words(80, lang="mi"), "waru tekau")
        self.assertEqual(num2words(85, lang="mi"), "waru tekau rima")
        self.assertEqual(num2words(90, lang="mi"), "iwa tekau")
        self.assertEqual(num2words(95, lang="mi"), "iwa tekau rima")
        self.assertEqual(num2words(99, lang="mi"), "iwa tekau iwa")
        self.assertEqual(num2words(100, lang="mi"), "tahi rau")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="mi"), "tahi rau tahi")
        self.assertEqual(num2words(110, lang="mi"), "tahi rau tekau")
        self.assertEqual(num2words(111, lang="mi"), "tahi rau tekau tahi")
        self.assertEqual(num2words(120, lang="mi"), "tahi rau rua tekau")
        self.assertEqual(num2words(125, lang="mi"), "tahi rau rua tekau rima")
        self.assertEqual(num2words(150, lang="mi"), "tahi rau rima tekau")
        self.assertEqual(num2words(175, lang="mi"), "tahi rau whitu tekau rima")
        self.assertEqual(num2words(199, lang="mi"), "tahi rau iwa tekau iwa")
        self.assertEqual(num2words(200, lang="mi"), "rua rau")
        self.assertEqual(num2words(201, lang="mi"), "rua rau tahi")
        self.assertEqual(num2words(210, lang="mi"), "rua rau tekau")
        self.assertEqual(num2words(220, lang="mi"), "rua rau rua tekau")
        self.assertEqual(num2words(250, lang="mi"), "rua rau rima tekau")
        self.assertEqual(num2words(299, lang="mi"), "rua rau iwa tekau iwa")
        self.assertEqual(num2words(300, lang="mi"), "toru rau")
        self.assertEqual(num2words(333, lang="mi"), "toru rau toru tekau toru")
        self.assertEqual(num2words(400, lang="mi"), "whā rau")
        self.assertEqual(num2words(444, lang="mi"), "whā rau whā tekau whā")
        self.assertEqual(num2words(500, lang="mi"), "rima rau")
        self.assertEqual(num2words(555, lang="mi"), "rima rau rima tekau rima")
        self.assertEqual(num2words(600, lang="mi"), "ono rau")
        self.assertEqual(num2words(666, lang="mi"), "ono rau ono tekau ono")
        self.assertEqual(num2words(700, lang="mi"), "whitu rau")
        self.assertEqual(num2words(777, lang="mi"), "whitu rau whitu tekau whitu")
        self.assertEqual(num2words(800, lang="mi"), "waru rau")
        self.assertEqual(num2words(888, lang="mi"), "waru rau waru tekau waru")
        self.assertEqual(num2words(900, lang="mi"), "iwa rau")
        self.assertEqual(num2words(999, lang="mi"), "iwa rau iwa tekau iwa")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="mi"), "tahi mano")
        self.assertEqual(num2words(1001, lang="mi"), "tahi mano tahi")
        self.assertEqual(num2words(1010, lang="mi"), "tahi mano tekau")
        self.assertEqual(num2words(1100, lang="mi"), "tahi mano tahi rau")
        self.assertEqual(num2words(1111, lang="mi"), "tahi mano tahi rau tekau tahi")
        self.assertEqual(num2words(1234, lang="mi"), "tahi mano rua rau toru tekau whā")
        self.assertEqual(num2words(1500, lang="mi"), "tahi mano rima rau")
        self.assertEqual(num2words(1999, lang="mi"), "tahi mano iwa rau iwa tekau iwa")
        self.assertEqual(num2words(2000, lang="mi"), "rua mano")
        self.assertEqual(num2words(2001, lang="mi"), "rua mano tahi")
        self.assertEqual(num2words(2020, lang="mi"), "rua mano rua tekau")
        self.assertEqual(num2words(2222, lang="mi"), "rua mano rua rau rua tekau rua")
        self.assertEqual(num2words(3000, lang="mi"), "toru mano")
        self.assertEqual(
            num2words(3333, lang="mi"), "toru mano toru rau toru tekau toru"
        )
        self.assertEqual(num2words(4000, lang="mi"), "whā mano")
        self.assertEqual(num2words(4444, lang="mi"), "whā mano whā rau whā tekau whā")
        self.assertEqual(num2words(5000, lang="mi"), "rima mano")
        self.assertEqual(
            num2words(5555, lang="mi"), "rima mano rima rau rima tekau rima"
        )
        self.assertEqual(num2words(6000, lang="mi"), "ono mano")
        self.assertEqual(num2words(6666, lang="mi"), "ono mano ono rau ono tekau ono")
        self.assertEqual(num2words(7000, lang="mi"), "whitu mano")
        self.assertEqual(
            num2words(7777, lang="mi"), "whitu mano whitu rau whitu tekau whitu"
        )
        self.assertEqual(num2words(8000, lang="mi"), "waru mano")
        self.assertEqual(
            num2words(8888, lang="mi"), "waru mano waru rau waru tekau waru"
        )
        self.assertEqual(num2words(9000, lang="mi"), "iwa mano")
        self.assertEqual(num2words(9999, lang="mi"), "iwa mano iwa rau iwa tekau iwa")
        self.assertEqual(num2words(10000, lang="mi"), "tekau mano")
        self.assertEqual(num2words(10001, lang="mi"), "tekau mano tahi")
        self.assertEqual(
            num2words(11111, lang="mi"), "tekau tahi mano tahi rau tekau tahi"
        )
        self.assertEqual(
            num2words(12345, lang="mi"), "tekau rua mano toru rau whā tekau rima"
        )
        self.assertEqual(num2words(20000, lang="mi"), "rua tekau mano")
        self.assertEqual(num2words(50000, lang="mi"), "rima tekau mano")
        self.assertEqual(
            num2words(99999, lang="mi"), "iwa tekau iwa mano iwa rau iwa tekau iwa"
        )
        self.assertEqual(num2words(100000, lang="mi"), "tahi rau mano")
        self.assertEqual(
            num2words(123456, lang="mi"),
            "tahi rau rua tekau toru mano whā rau rima tekau ono",
        )
        self.assertEqual(num2words(200000, lang="mi"), "rua rau mano")
        self.assertEqual(num2words(500000, lang="mi"), "rima rau mano")
        self.assertEqual(
            num2words(654321, lang="mi"),
            "ono rau rima tekau whā mano toru rau rua tekau tahi",
        )
        self.assertEqual(
            num2words(999999, lang="mi"),
            "iwa rau iwa tekau iwa mano iwa rau iwa tekau iwa",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="mi"), "tahi miriona")
        self.assertEqual(num2words(1000001, lang="mi"), "tahi miriona tahi")
        self.assertEqual(
            num2words(1111111, lang="mi"),
            "tahi miriona tahi rau tekau tahi mano tahi rau tekau tahi",
        )
        self.assertEqual(
            num2words(1234567, lang="mi"),
            "tahi miriona rua rau toru tekau whā mano rima rau ono tekau whitu",
        )
        self.assertEqual(num2words(2000000, lang="mi"), "rua miriona")
        self.assertEqual(num2words(5000000, lang="mi"), "rima miriona")
        self.assertEqual(
            num2words(9999999, lang="mi"),
            "iwa miriona iwa rau iwa tekau iwa mano iwa rau iwa tekau iwa",
        )
        self.assertEqual(num2words(10000000, lang="mi"), "tekau miriona")
        self.assertEqual(
            num2words(12345678, lang="mi"),
            "tekau rua miriona toru rau whā tekau rima mano ono rau whitu tekau waru",
        )
        self.assertEqual(
            num2words(99999999, lang="mi"),
            "iwa tekau iwa miriona iwa rau iwa tekau iwa mano iwa rau iwa tekau iwa",
        )
        self.assertEqual(num2words(100000000, lang="mi"), "tahi rau miriona")
        self.assertEqual(
            num2words(123456789, lang="mi"),
            "tahi rau rua tekau toru miriona whā rau rima tekau ono mano whitu rau waru tekau iwa",
        )
        self.assertEqual(
            num2words(999999999, lang="mi"),
            "iwa rau iwa tekau iwa miriona iwa rau iwa tekau iwa mano iwa rau iwa tekau iwa",
        )
        self.assertEqual(num2words(1000000000, lang="mi"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="mi"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="mi"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="mi"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="mi"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="mi"), "minus tahi")
        self.assertEqual(num2words(-2, lang="mi"), "minus rua")
        self.assertEqual(num2words(-5, lang="mi"), "minus rima")
        self.assertEqual(num2words(-10, lang="mi"), "minus tekau")
        self.assertEqual(num2words(-11, lang="mi"), "minus tekau tahi")
        self.assertEqual(num2words(-20, lang="mi"), "minus rua tekau")
        self.assertEqual(num2words(-50, lang="mi"), "minus rima tekau")
        self.assertEqual(num2words(-99, lang="mi"), "minus iwa tekau iwa")
        self.assertEqual(num2words(-100, lang="mi"), "minus tahi rau")
        self.assertEqual(num2words(-101, lang="mi"), "minus tahi rau tahi")
        self.assertEqual(num2words(-200, lang="mi"), "minus rua rau")
        self.assertEqual(num2words(-999, lang="mi"), "minus iwa rau iwa tekau iwa")
        self.assertEqual(num2words(-1000, lang="mi"), "minus tahi mano")
        self.assertEqual(num2words(-1001, lang="mi"), "minus tahi mano tahi")
        self.assertEqual(num2words(-10000, lang="mi"), "minus tekau mano")
        self.assertEqual(num2words(-100000, lang="mi"), "minus tahi rau mano")
        self.assertEqual(num2words(-1000000, lang="mi"), "minus tahi miriona")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="mi"), "zero point tahi")
        self.assertEqual(num2words(0.5, lang="mi"), "zero point rima")
        self.assertEqual(num2words(0.9, lang="mi"), "zero point iwa")
        self.assertEqual(num2words(1.1, lang="mi"), "tahi point tahi")
        self.assertEqual(num2words(1.5, lang="mi"), "tahi point rima")
        self.assertEqual(num2words(2.5, lang="mi"), "rua point rima")
        self.assertEqual(num2words(3.14, lang="mi"), "toru point tahi whā")
        self.assertEqual(num2words(10.5, lang="mi"), "tekau point rima")
        self.assertEqual(num2words(11.11, lang="mi"), "tekau tahi point tahi tahi")
        self.assertEqual(num2words(20.2, lang="mi"), "rua tekau point rua")
        self.assertEqual(num2words(99.99, lang="mi"), "iwa tekau iwa point iwa iwa")
        self.assertEqual(num2words(100.01, lang="mi"), "tahi rau point zero tahi")
        self.assertEqual(num2words(100.5, lang="mi"), "tahi rau point rima")
        self.assertEqual(
            num2words(123.45, lang="mi"), "tahi rau rua tekau toru point whā rima"
        )
        self.assertEqual(num2words(1000.5, lang="mi"), "tahi mano point rima")
        self.assertEqual(
            num2words(1234.56, lang="mi"),
            "tahi mano rua rau toru tekau whā point rima ono",
        )
        self.assertEqual(num2words(10000.01, lang="mi"), "tekau mano point zero tahi")
        self.assertEqual(num2words(-0.5, lang="mi"), "minus zero point rima")
        self.assertEqual(num2words(-1.5, lang="mi"), "minus tahi point rima")
        self.assertEqual(num2words(-10.5, lang="mi"), "minus tekau point rima")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="mi", ordinal=True), "tuatahi")
        self.assertEqual(num2words(2, lang="mi", ordinal=True), "tuarua")
        self.assertEqual(num2words(3, lang="mi", ordinal=True), "tuatoru")
        self.assertEqual(num2words(4, lang="mi", ordinal=True), "tuawhā")
        self.assertEqual(num2words(5, lang="mi", ordinal=True), "tuarima")
        self.assertEqual(num2words(6, lang="mi", ordinal=True), "tua ono")
        self.assertEqual(num2words(7, lang="mi", ordinal=True), "tua whitu")
        self.assertEqual(num2words(8, lang="mi", ordinal=True), "tua waru")
        self.assertEqual(num2words(9, lang="mi", ordinal=True), "tua iwa")
        self.assertEqual(num2words(10, lang="mi", ordinal=True), "tua tekau")
        self.assertEqual(num2words(11, lang="mi", ordinal=True), "tua tekau tahi")
        self.assertEqual(num2words(12, lang="mi", ordinal=True), "tua tekau rua")
        self.assertEqual(num2words(13, lang="mi", ordinal=True), "tua tekau toru")
        self.assertEqual(num2words(14, lang="mi", ordinal=True), "tua tekau whā")
        self.assertEqual(num2words(15, lang="mi", ordinal=True), "tua tekau rima")
        self.assertEqual(num2words(16, lang="mi", ordinal=True), "tua tekau ono")
        self.assertEqual(num2words(17, lang="mi", ordinal=True), "tua tekau whitu")
        self.assertEqual(num2words(18, lang="mi", ordinal=True), "tua tekau waru")
        self.assertEqual(num2words(19, lang="mi", ordinal=True), "tua tekau iwa")
        self.assertEqual(num2words(20, lang="mi", ordinal=True), "tua rua tekau")
        self.assertEqual(num2words(21, lang="mi", ordinal=True), "tua rua tekau tahi")
        self.assertEqual(num2words(22, lang="mi", ordinal=True), "tua rua tekau rua")
        self.assertEqual(num2words(25, lang="mi", ordinal=True), "tua rua tekau rima")
        self.assertEqual(num2words(30, lang="mi", ordinal=True), "tua toru tekau")
        self.assertEqual(num2words(40, lang="mi", ordinal=True), "tua whā tekau")
        self.assertEqual(num2words(50, lang="mi", ordinal=True), "tua rima tekau")
        self.assertEqual(num2words(60, lang="mi", ordinal=True), "tua ono tekau")
        self.assertEqual(num2words(70, lang="mi", ordinal=True), "tua whitu tekau")
        self.assertEqual(num2words(80, lang="mi", ordinal=True), "tua waru tekau")
        self.assertEqual(num2words(90, lang="mi", ordinal=True), "tua iwa tekau")
        self.assertEqual(num2words(100, lang="mi", ordinal=True), "tua tahi rau")
        self.assertEqual(num2words(101, lang="mi", ordinal=True), "tua tahi rau tahi")
        self.assertEqual(num2words(200, lang="mi", ordinal=True), "tua rua rau")
        self.assertEqual(num2words(500, lang="mi", ordinal=True), "tua rima rau")
        self.assertEqual(num2words(1000, lang="mi", ordinal=True), "tua tahi mano")
        self.assertEqual(num2words(1001, lang="mi", ordinal=True), "tua tahi mano tahi")
        self.assertEqual(num2words(10000, lang="mi", ordinal=True), "tua tekau mano")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="mi", to="currency", currency="NZD"), "zero tāra"
        )
        self.assertEqual(
            num2words(0.01, lang="mi", to="currency", currency="NZD"),
            "zero tāra tahi hēneti",
        )
        self.assertEqual(
            num2words(0.5, lang="mi", to="currency", currency="NZD"),
            "zero tāra rima tekau hēneti",
        )
        self.assertEqual(
            num2words(1, lang="mi", to="currency", currency="NZD"), "tahi tāra"
        )
        self.assertEqual(
            num2words(1.5, lang="mi", to="currency", currency="NZD"),
            "tahi tāra rima tekau hēneti",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="mi", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="mi", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="mi", to="year"), "tahi mano")
        self.assertEqual(
            num2words(1066, lang="mi", to="year"), "tahi mano ono tekau ono"
        )
        self.assertEqual(
            num2words(1492, lang="mi", to="year"), "tahi mano whā rau iwa tekau rua"
        )
        self.assertEqual(
            num2words(1776, lang="mi", to="year"), "tahi mano whitu rau whitu tekau ono"
        )
        self.assertEqual(num2words(1800, lang="mi", to="year"), "tahi mano waru rau")
        self.assertEqual(num2words(1900, lang="mi", to="year"), "tahi mano iwa rau")
        self.assertEqual(
            num2words(1984, lang="mi", to="year"), "tahi mano iwa rau waru tekau whā"
        )
        self.assertEqual(
            num2words(1999, lang="mi", to="year"), "tahi mano iwa rau iwa tekau iwa"
        )
        self.assertEqual(num2words(2000, lang="mi", to="year"), "rua mano")
        self.assertEqual(num2words(2001, lang="mi", to="year"), "rua mano tahi")
        self.assertEqual(num2words(2010, lang="mi", to="year"), "rua mano tekau")
        self.assertEqual(num2words(2020, lang="mi", to="year"), "rua mano rua tekau")
        self.assertEqual(
            num2words(2024, lang="mi", to="year"), "rua mano rua tekau whā"
        )
        self.assertEqual(num2words(2100, lang="mi", to="year"), "rua mano tahi rau")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="mi"), "zero")
        self.assertEqual(num2words("1", lang="mi"), "tahi")
        self.assertEqual(num2words("10", lang="mi"), "tekau")
        self.assertEqual(num2words("100", lang="mi"), "tahi rau")
        self.assertEqual(num2words("1000", lang="mi"), "tahi mano")
        self.assertEqual(num2words("10000", lang="mi"), "tekau mano")
        self.assertEqual(num2words("100000", lang="mi"), "tahi rau mano")
        self.assertEqual(num2words("1000000", lang="mi"), "tahi miriona")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="mi"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="mi"), num2words("100", lang="mi"))
        self.assertEqual(num2words(1000, lang="mi"), num2words("1000", lang="mi"))

