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


class Num2WordsMGTest(TestCase):
    """Comprehensive test cases for Malagasy language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="mg"), "aotra")
        self.assertEqual(num2words(1, lang="mg"), "iray")
        self.assertEqual(num2words(2, lang="mg"), "roa")
        self.assertEqual(num2words(3, lang="mg"), "telo")
        self.assertEqual(num2words(4, lang="mg"), "efatra")
        self.assertEqual(num2words(5, lang="mg"), "dimy")
        self.assertEqual(num2words(6, lang="mg"), "enina")
        self.assertEqual(num2words(7, lang="mg"), "fito")
        self.assertEqual(num2words(8, lang="mg"), "valo")
        self.assertEqual(num2words(9, lang="mg"), "sivy")
        self.assertEqual(num2words(10, lang="mg"), "folo")
        self.assertEqual(num2words(11, lang="mg"), "folo iray")
        self.assertEqual(num2words(12, lang="mg"), "folo roa")
        self.assertEqual(num2words(13, lang="mg"), "folo telo")
        self.assertEqual(num2words(14, lang="mg"), "folo efatra")
        self.assertEqual(num2words(15, lang="mg"), "folo dimy")
        self.assertEqual(num2words(16, lang="mg"), "folo enina")
        self.assertEqual(num2words(17, lang="mg"), "folo fito")
        self.assertEqual(num2words(18, lang="mg"), "folo valo")
        self.assertEqual(num2words(19, lang="mg"), "folo sivy")
        self.assertEqual(num2words(20, lang="mg"), "roapolo")
        self.assertEqual(num2words(21, lang="mg"), "roapolo iray")
        self.assertEqual(num2words(22, lang="mg"), "roapolo roa")
        self.assertEqual(num2words(23, lang="mg"), "roapolo telo")
        self.assertEqual(num2words(24, lang="mg"), "roapolo efatra")
        self.assertEqual(num2words(25, lang="mg"), "roapolo dimy")
        self.assertEqual(num2words(26, lang="mg"), "roapolo enina")
        self.assertEqual(num2words(27, lang="mg"), "roapolo fito")
        self.assertEqual(num2words(28, lang="mg"), "roapolo valo")
        self.assertEqual(num2words(29, lang="mg"), "roapolo sivy")
        self.assertEqual(num2words(30, lang="mg"), "telopolo")
        self.assertEqual(num2words(31, lang="mg"), "telopolo iray")
        self.assertEqual(num2words(35, lang="mg"), "telopolo dimy")
        self.assertEqual(num2words(40, lang="mg"), "efapolo")
        self.assertEqual(num2words(45, lang="mg"), "efapolo dimy")
        self.assertEqual(num2words(50, lang="mg"), "dimampolo")
        self.assertEqual(num2words(55, lang="mg"), "dimampolo dimy")
        self.assertEqual(num2words(60, lang="mg"), "enimpolo")
        self.assertEqual(num2words(65, lang="mg"), "enimpolo dimy")
        self.assertEqual(num2words(70, lang="mg"), "fitopolo")
        self.assertEqual(num2words(75, lang="mg"), "fitopolo dimy")
        self.assertEqual(num2words(80, lang="mg"), "valopolo")
        self.assertEqual(num2words(85, lang="mg"), "valopolo dimy")
        self.assertEqual(num2words(90, lang="mg"), "sivifolo")
        self.assertEqual(num2words(95, lang="mg"), "sivifolo dimy")
        self.assertEqual(num2words(99, lang="mg"), "sivifolo sivy")
        self.assertEqual(num2words(100, lang="mg"), "iray zato")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="mg"), "iray zato iray")
        self.assertEqual(num2words(110, lang="mg"), "iray zato folo")
        self.assertEqual(num2words(111, lang="mg"), "iray zato folo iray")
        self.assertEqual(num2words(120, lang="mg"), "iray zato roapolo")
        self.assertEqual(num2words(125, lang="mg"), "iray zato roapolo dimy")
        self.assertEqual(num2words(150, lang="mg"), "iray zato dimampolo")
        self.assertEqual(num2words(175, lang="mg"), "iray zato fitopolo dimy")
        self.assertEqual(num2words(199, lang="mg"), "iray zato sivifolo sivy")
        self.assertEqual(num2words(200, lang="mg"), "roa zato")
        self.assertEqual(num2words(201, lang="mg"), "roa zato iray")
        self.assertEqual(num2words(210, lang="mg"), "roa zato folo")
        self.assertEqual(num2words(220, lang="mg"), "roa zato roapolo")
        self.assertEqual(num2words(250, lang="mg"), "roa zato dimampolo")
        self.assertEqual(num2words(299, lang="mg"), "roa zato sivifolo sivy")
        self.assertEqual(num2words(300, lang="mg"), "telo zato")
        self.assertEqual(num2words(333, lang="mg"), "telo zato telopolo telo")
        self.assertEqual(num2words(400, lang="mg"), "efatra zato")
        self.assertEqual(num2words(444, lang="mg"), "efatra zato efapolo efatra")
        self.assertEqual(num2words(500, lang="mg"), "dimy zato")
        self.assertEqual(num2words(555, lang="mg"), "dimy zato dimampolo dimy")
        self.assertEqual(num2words(600, lang="mg"), "enina zato")
        self.assertEqual(num2words(666, lang="mg"), "enina zato enimpolo enina")
        self.assertEqual(num2words(700, lang="mg"), "fito zato")
        self.assertEqual(num2words(777, lang="mg"), "fito zato fitopolo fito")
        self.assertEqual(num2words(800, lang="mg"), "valo zato")
        self.assertEqual(num2words(888, lang="mg"), "valo zato valopolo valo")
        self.assertEqual(num2words(900, lang="mg"), "sivy zato")
        self.assertEqual(num2words(999, lang="mg"), "sivy zato sivifolo sivy")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="mg"), "iray arivo")
        self.assertEqual(num2words(1001, lang="mg"), "iray arivo iray")
        self.assertEqual(num2words(1010, lang="mg"), "iray arivo folo")
        self.assertEqual(num2words(1100, lang="mg"), "iray arivo iray zato")
        self.assertEqual(num2words(1111, lang="mg"), "iray arivo iray zato folo iray")
        self.assertEqual(
            num2words(1234, lang="mg"), "iray arivo roa zato telopolo efatra"
        )
        self.assertEqual(num2words(1500, lang="mg"), "iray arivo dimy zato")
        self.assertEqual(
            num2words(1999, lang="mg"), "iray arivo sivy zato sivifolo sivy"
        )
        self.assertEqual(num2words(2000, lang="mg"), "roa arivo")
        self.assertEqual(num2words(2001, lang="mg"), "roa arivo iray")
        self.assertEqual(num2words(2020, lang="mg"), "roa arivo roapolo")
        self.assertEqual(num2words(2222, lang="mg"), "roa arivo roa zato roapolo roa")
        self.assertEqual(num2words(3000, lang="mg"), "telo arivo")
        self.assertEqual(
            num2words(3333, lang="mg"), "telo arivo telo zato telopolo telo"
        )
        self.assertEqual(num2words(4000, lang="mg"), "efatra arivo")
        self.assertEqual(
            num2words(4444, lang="mg"), "efatra arivo efatra zato efapolo efatra"
        )
        self.assertEqual(num2words(5000, lang="mg"), "dimy arivo")
        self.assertEqual(
            num2words(5555, lang="mg"), "dimy arivo dimy zato dimampolo dimy"
        )
        self.assertEqual(num2words(6000, lang="mg"), "enina arivo")
        self.assertEqual(
            num2words(6666, lang="mg"), "enina arivo enina zato enimpolo enina"
        )
        self.assertEqual(num2words(7000, lang="mg"), "fito arivo")
        self.assertEqual(
            num2words(7777, lang="mg"), "fito arivo fito zato fitopolo fito"
        )
        self.assertEqual(num2words(8000, lang="mg"), "valo arivo")
        self.assertEqual(
            num2words(8888, lang="mg"), "valo arivo valo zato valopolo valo"
        )
        self.assertEqual(num2words(9000, lang="mg"), "sivy arivo")
        self.assertEqual(
            num2words(9999, lang="mg"), "sivy arivo sivy zato sivifolo sivy"
        )
        self.assertEqual(num2words(10000, lang="mg"), "folo arivo")
        self.assertEqual(num2words(10001, lang="mg"), "folo arivo iray")
        self.assertEqual(
            num2words(11111, lang="mg"), "folo iray arivo iray zato folo iray"
        )
        self.assertEqual(
            num2words(12345, lang="mg"), "folo roa arivo telo zato efapolo dimy"
        )
        self.assertEqual(num2words(20000, lang="mg"), "roapolo arivo")
        self.assertEqual(num2words(50000, lang="mg"), "dimampolo arivo")
        self.assertEqual(
            num2words(99999, lang="mg"), "sivifolo sivy arivo sivy zato sivifolo sivy"
        )
        self.assertEqual(num2words(100000, lang="mg"), "iray zato arivo")
        self.assertEqual(
            num2words(123456, lang="mg"),
            "iray zato roapolo telo arivo efatra zato dimampolo enina",
        )
        self.assertEqual(num2words(200000, lang="mg"), "roa zato arivo")
        self.assertEqual(num2words(500000, lang="mg"), "dimy zato arivo")
        self.assertEqual(
            num2words(654321, lang="mg"),
            "enina zato dimampolo efatra arivo telo zato roapolo iray",
        )
        self.assertEqual(
            num2words(999999, lang="mg"),
            "sivy zato sivifolo sivy arivo sivy zato sivifolo sivy",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="mg"), "iray tapitrisa")
        self.assertEqual(num2words(1000001, lang="mg"), "iray tapitrisa iray")
        self.assertEqual(
            num2words(1111111, lang="mg"),
            "iray tapitrisa iray zato folo iray arivo iray zato folo iray",
        )
        self.assertEqual(
            num2words(1234567, lang="mg"),
            "iray tapitrisa roa zato telopolo efatra arivo dimy zato enimpolo fito",
        )
        self.assertEqual(num2words(2000000, lang="mg"), "roa tapitrisa")
        self.assertEqual(num2words(5000000, lang="mg"), "dimy tapitrisa")
        self.assertEqual(
            num2words(9999999, lang="mg"),
            "sivy tapitrisa sivy zato sivifolo sivy arivo sivy zato sivifolo sivy",
        )
        self.assertEqual(num2words(10000000, lang="mg"), "folo tapitrisa")
        self.assertEqual(
            num2words(12345678, lang="mg"),
            "folo roa tapitrisa telo zato efapolo dimy arivo enina zato fitopolo valo",
        )
        self.assertEqual(
            num2words(99999999, lang="mg"),
            "sivifolo sivy tapitrisa sivy zato sivifolo sivy arivo sivy zato sivifolo sivy",
        )
        self.assertEqual(num2words(100000000, lang="mg"), "iray zato tapitrisa")
        self.assertEqual(
            num2words(123456789, lang="mg"),
            "iray zato roapolo telo tapitrisa efatra zato dimampolo enina arivo fito zato valopolo sivy",
        )
        self.assertEqual(
            num2words(999999999, lang="mg"),
            "sivy zato sivifolo sivy tapitrisa sivy zato sivifolo sivy arivo sivy zato sivifolo sivy",
        )
        self.assertEqual(num2words(1000000000, lang="mg"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="mg"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="mg"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="mg"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="mg"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="mg"), "minus iray")
        self.assertEqual(num2words(-2, lang="mg"), "minus roa")
        self.assertEqual(num2words(-5, lang="mg"), "minus dimy")
        self.assertEqual(num2words(-10, lang="mg"), "minus folo")
        self.assertEqual(num2words(-11, lang="mg"), "minus folo iray")
        self.assertEqual(num2words(-20, lang="mg"), "minus roapolo")
        self.assertEqual(num2words(-50, lang="mg"), "minus dimampolo")
        self.assertEqual(num2words(-99, lang="mg"), "minus sivifolo sivy")
        self.assertEqual(num2words(-100, lang="mg"), "minus iray zato")
        self.assertEqual(num2words(-101, lang="mg"), "minus iray zato iray")
        self.assertEqual(num2words(-200, lang="mg"), "minus roa zato")
        self.assertEqual(num2words(-999, lang="mg"), "minus sivy zato sivifolo sivy")
        self.assertEqual(num2words(-1000, lang="mg"), "minus iray arivo")
        self.assertEqual(num2words(-1001, lang="mg"), "minus iray arivo iray")
        self.assertEqual(num2words(-10000, lang="mg"), "minus folo arivo")
        self.assertEqual(num2words(-100000, lang="mg"), "minus iray zato arivo")
        self.assertEqual(num2words(-1000000, lang="mg"), "minus iray tapitrisa")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="mg"), "aotra faingo iray")
        self.assertEqual(num2words(0.5, lang="mg"), "aotra faingo dimy")
        self.assertEqual(num2words(0.9, lang="mg"), "aotra faingo sivy")
        self.assertEqual(num2words(1.1, lang="mg"), "iray faingo iray")
        self.assertEqual(num2words(1.5, lang="mg"), "iray faingo dimy")
        self.assertEqual(num2words(2.5, lang="mg"), "roa faingo dimy")
        self.assertEqual(num2words(3.14, lang="mg"), "telo faingo iray efatra")
        self.assertEqual(num2words(10.5, lang="mg"), "folo faingo dimy")
        self.assertEqual(num2words(11.11, lang="mg"), "folo iray faingo iray iray")
        self.assertEqual(num2words(20.2, lang="mg"), "roapolo faingo roa")
        self.assertEqual(num2words(99.99, lang="mg"), "sivifolo sivy faingo sivy sivy")
        self.assertEqual(num2words(100.01, lang="mg"), "iray zato faingo aotra iray")
        self.assertEqual(num2words(100.5, lang="mg"), "iray zato faingo dimy")
        self.assertEqual(
            num2words(123.45, lang="mg"), "iray zato roapolo telo faingo efatra dimy"
        )
        self.assertEqual(num2words(1000.5, lang="mg"), "iray arivo faingo dimy")
        self.assertEqual(
            num2words(1234.56, lang="mg"),
            "iray arivo roa zato telopolo efatra faingo dimy enina",
        )
        self.assertEqual(num2words(10000.01, lang="mg"), "folo arivo faingo aotra iray")
        self.assertEqual(num2words(-0.5, lang="mg"), "minus aotra faingo dimy")
        self.assertEqual(num2words(-1.5, lang="mg"), "minus iray faingo dimy")
        self.assertEqual(num2words(-10.5, lang="mg"), "minus folo faingo dimy")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="mg", ordinal=True), "voalohany")
        self.assertEqual(num2words(2, lang="mg", ordinal=True), "faha-roa")
        self.assertEqual(num2words(3, lang="mg", ordinal=True), "faha-telo")
        self.assertEqual(num2words(4, lang="mg", ordinal=True), "faha-efatra")
        self.assertEqual(num2words(5, lang="mg", ordinal=True), "faha-dimy")
        self.assertEqual(num2words(6, lang="mg", ordinal=True), "faha-enina")
        self.assertEqual(num2words(7, lang="mg", ordinal=True), "faha-fito")
        self.assertEqual(num2words(8, lang="mg", ordinal=True), "faha-valo")
        self.assertEqual(num2words(9, lang="mg", ordinal=True), "faha-sivy")
        self.assertEqual(num2words(10, lang="mg", ordinal=True), "faha-folo")
        self.assertEqual(num2words(11, lang="mg", ordinal=True), "faha-folo iray")
        self.assertEqual(num2words(12, lang="mg", ordinal=True), "faha-folo roa")
        self.assertEqual(num2words(13, lang="mg", ordinal=True), "faha-folo telo")
        self.assertEqual(num2words(14, lang="mg", ordinal=True), "faha-folo efatra")
        self.assertEqual(num2words(15, lang="mg", ordinal=True), "faha-folo dimy")
        self.assertEqual(num2words(16, lang="mg", ordinal=True), "faha-folo enina")
        self.assertEqual(num2words(17, lang="mg", ordinal=True), "faha-folo fito")
        self.assertEqual(num2words(18, lang="mg", ordinal=True), "faha-folo valo")
        self.assertEqual(num2words(19, lang="mg", ordinal=True), "faha-folo sivy")
        self.assertEqual(num2words(20, lang="mg", ordinal=True), "faha-roapolo")
        self.assertEqual(num2words(21, lang="mg", ordinal=True), "faha-roapolo iray")
        self.assertEqual(num2words(22, lang="mg", ordinal=True), "faha-roapolo roa")
        self.assertEqual(num2words(25, lang="mg", ordinal=True), "faha-roapolo dimy")
        self.assertEqual(num2words(30, lang="mg", ordinal=True), "faha-telopolo")
        self.assertEqual(num2words(40, lang="mg", ordinal=True), "faha-efapolo")
        self.assertEqual(num2words(50, lang="mg", ordinal=True), "faha-dimampolo")
        self.assertEqual(num2words(60, lang="mg", ordinal=True), "faha-enimpolo")
        self.assertEqual(num2words(70, lang="mg", ordinal=True), "faha-fitopolo")
        self.assertEqual(num2words(80, lang="mg", ordinal=True), "faha-valopolo")
        self.assertEqual(num2words(90, lang="mg", ordinal=True), "faha-sivifolo")
        self.assertEqual(num2words(100, lang="mg", ordinal=True), "faha-iray zato")
        self.assertEqual(num2words(101, lang="mg", ordinal=True), "faha-iray zato iray")
        self.assertEqual(num2words(200, lang="mg", ordinal=True), "faha-roa zato")
        self.assertEqual(num2words(500, lang="mg", ordinal=True), "faha-dimy zato")
        self.assertEqual(num2words(1000, lang="mg", ordinal=True), "faha-iray arivo")
        self.assertEqual(
            num2words(1001, lang="mg", ordinal=True), "faha-iray arivo iray"
        )
        self.assertEqual(num2words(10000, lang="mg", ordinal=True), "faha-folo arivo")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="mg", to="currency", currency="MGA"), "aotra ariary"
        )
        self.assertEqual(
            num2words(0.01, lang="mg", to="currency", currency="MGA"),
            "aotra ariary iray iraimbilanja",
        )
        self.assertEqual(
            num2words(0.5, lang="mg", to="currency", currency="MGA"),
            "aotra ariary dimampolo iraimbilanja",
        )
        self.assertEqual(
            num2words(1, lang="mg", to="currency", currency="MGA"), "iray ariary"
        )
        self.assertEqual(
            num2words(1.5, lang="mg", to="currency", currency="MGA"),
            "iray ariary dimampolo iraimbilanja",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="mg", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="mg", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="mg", to="year"), "iray arivo")
        self.assertEqual(
            num2words(1066, lang="mg", to="year"), "iray arivo enimpolo enina"
        )
        self.assertEqual(
            num2words(1492, lang="mg", to="year"), "iray arivo efatra zato sivifolo roa"
        )
        self.assertEqual(
            num2words(1776, lang="mg", to="year"), "iray arivo fito zato fitopolo enina"
        )
        self.assertEqual(num2words(1800, lang="mg", to="year"), "iray arivo valo zato")
        self.assertEqual(num2words(1900, lang="mg", to="year"), "iray arivo sivy zato")
        self.assertEqual(
            num2words(1984, lang="mg", to="year"),
            "iray arivo sivy zato valopolo efatra",
        )
        self.assertEqual(
            num2words(1999, lang="mg", to="year"), "iray arivo sivy zato sivifolo sivy"
        )
        self.assertEqual(num2words(2000, lang="mg", to="year"), "roa arivo")
        self.assertEqual(num2words(2001, lang="mg", to="year"), "roa arivo iray")
        self.assertEqual(num2words(2010, lang="mg", to="year"), "roa arivo folo")
        self.assertEqual(num2words(2020, lang="mg", to="year"), "roa arivo roapolo")
        self.assertEqual(
            num2words(2024, lang="mg", to="year"), "roa arivo roapolo efatra"
        )
        self.assertEqual(num2words(2100, lang="mg", to="year"), "roa arivo iray zato")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="mg"), "aotra")
        self.assertEqual(num2words("1", lang="mg"), "iray")
        self.assertEqual(num2words("10", lang="mg"), "folo")
        self.assertEqual(num2words("100", lang="mg"), "iray zato")
        self.assertEqual(num2words("1000", lang="mg"), "iray arivo")
        self.assertEqual(num2words("10000", lang="mg"), "folo arivo")
        self.assertEqual(num2words("100000", lang="mg"), "iray zato arivo")
        self.assertEqual(num2words("1000000", lang="mg"), "iray tapitrisa")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="mg"), "aotra")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="mg"), num2words("100", lang="mg"))
        self.assertEqual(num2words(1000, lang="mg"), num2words("1000", lang="mg"))

