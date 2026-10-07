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


class Num2WordsUZTest(TestCase):
    """Comprehensive test cases for Uzbek language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="uz"), "zero")
        self.assertEqual(num2words(1, lang="uz"), "bir")
        self.assertEqual(num2words(2, lang="uz"), "ikki")
        self.assertEqual(num2words(3, lang="uz"), "uch")
        self.assertEqual(num2words(4, lang="uz"), "to'rt")
        self.assertEqual(num2words(5, lang="uz"), "besh")
        self.assertEqual(num2words(6, lang="uz"), "olti")
        self.assertEqual(num2words(7, lang="uz"), "yetti")
        self.assertEqual(num2words(8, lang="uz"), "sakkiz")
        self.assertEqual(num2words(9, lang="uz"), "to'qqiz")
        self.assertEqual(num2words(10, lang="uz"), "o'n")
        self.assertEqual(num2words(11, lang="uz"), "o'n bir")
        self.assertEqual(num2words(12, lang="uz"), "o'n ikki")
        self.assertEqual(num2words(13, lang="uz"), "o'n uch")
        self.assertEqual(num2words(14, lang="uz"), "o'n to'rt")
        self.assertEqual(num2words(15, lang="uz"), "o'n besh")
        self.assertEqual(num2words(16, lang="uz"), "o'n olti")
        self.assertEqual(num2words(17, lang="uz"), "o'n yetti")
        self.assertEqual(num2words(18, lang="uz"), "o'n sakkiz")
        self.assertEqual(num2words(19, lang="uz"), "o'n to'qqiz")
        self.assertEqual(num2words(20, lang="uz"), "yigirma")
        self.assertEqual(num2words(21, lang="uz"), "yigirma bir")
        self.assertEqual(num2words(22, lang="uz"), "yigirma ikki")
        self.assertEqual(num2words(23, lang="uz"), "yigirma uch")
        self.assertEqual(num2words(24, lang="uz"), "yigirma to'rt")
        self.assertEqual(num2words(25, lang="uz"), "yigirma besh")
        self.assertEqual(num2words(26, lang="uz"), "yigirma olti")
        self.assertEqual(num2words(27, lang="uz"), "yigirma yetti")
        self.assertEqual(num2words(28, lang="uz"), "yigirma sakkiz")
        self.assertEqual(num2words(29, lang="uz"), "yigirma to'qqiz")
        self.assertEqual(num2words(30, lang="uz"), "o'ttiz")
        self.assertEqual(num2words(31, lang="uz"), "o'ttiz bir")
        self.assertEqual(num2words(35, lang="uz"), "o'ttiz besh")
        self.assertEqual(num2words(40, lang="uz"), "qirq")
        self.assertEqual(num2words(45, lang="uz"), "qirq besh")
        self.assertEqual(num2words(50, lang="uz"), "ellik")
        self.assertEqual(num2words(55, lang="uz"), "ellik besh")
        self.assertEqual(num2words(60, lang="uz"), "oltmish")
        self.assertEqual(num2words(65, lang="uz"), "oltmish besh")
        self.assertEqual(num2words(70, lang="uz"), "yetmish")
        self.assertEqual(num2words(75, lang="uz"), "yetmish besh")
        self.assertEqual(num2words(80, lang="uz"), "sakson")
        self.assertEqual(num2words(85, lang="uz"), "sakson besh")
        self.assertEqual(num2words(90, lang="uz"), "to'qson")
        self.assertEqual(num2words(95, lang="uz"), "to'qson besh")
        self.assertEqual(num2words(99, lang="uz"), "to'qson to'qqiz")
        self.assertEqual(num2words(100, lang="uz"), "bir yuz")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="uz"), "bir yuz bir")
        self.assertEqual(num2words(110, lang="uz"), "bir yuz o'n")
        self.assertEqual(num2words(111, lang="uz"), "bir yuz o'n bir")
        self.assertEqual(num2words(120, lang="uz"), "bir yuz yigirma")
        self.assertEqual(num2words(125, lang="uz"), "bir yuz yigirma besh")
        self.assertEqual(num2words(150, lang="uz"), "bir yuz ellik")
        self.assertEqual(num2words(175, lang="uz"), "bir yuz yetmish besh")
        self.assertEqual(num2words(199, lang="uz"), "bir yuz to'qson to'qqiz")
        self.assertEqual(num2words(200, lang="uz"), "ikki yuz")
        self.assertEqual(num2words(201, lang="uz"), "ikki yuz bir")
        self.assertEqual(num2words(210, lang="uz"), "ikki yuz o'n")
        self.assertEqual(num2words(220, lang="uz"), "ikki yuz yigirma")
        self.assertEqual(num2words(250, lang="uz"), "ikki yuz ellik")
        self.assertEqual(num2words(299, lang="uz"), "ikki yuz to'qson to'qqiz")
        self.assertEqual(num2words(300, lang="uz"), "uch yuz")
        self.assertEqual(num2words(333, lang="uz"), "uch yuz o'ttiz uch")
        self.assertEqual(num2words(400, lang="uz"), "to'rt yuz")
        self.assertEqual(num2words(444, lang="uz"), "to'rt yuz qirq to'rt")
        self.assertEqual(num2words(500, lang="uz"), "besh yuz")
        self.assertEqual(num2words(555, lang="uz"), "besh yuz ellik besh")
        self.assertEqual(num2words(600, lang="uz"), "olti yuz")
        self.assertEqual(num2words(666, lang="uz"), "olti yuz oltmish olti")
        self.assertEqual(num2words(700, lang="uz"), "yetti yuz")
        self.assertEqual(num2words(777, lang="uz"), "yetti yuz yetmish yetti")
        self.assertEqual(num2words(800, lang="uz"), "sakkiz yuz")
        self.assertEqual(num2words(888, lang="uz"), "sakkiz yuz sakson sakkiz")
        self.assertEqual(num2words(900, lang="uz"), "to'qqiz yuz")
        self.assertEqual(num2words(999, lang="uz"), "to'qqiz yuz to'qson to'qqiz")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="uz"), "bir ming")
        self.assertEqual(num2words(1001, lang="uz"), "bir ming bir")
        self.assertEqual(num2words(1010, lang="uz"), "bir ming o'n")
        self.assertEqual(num2words(1100, lang="uz"), "bir ming bir yuz")
        self.assertEqual(num2words(1111, lang="uz"), "bir ming bir yuz o'n bir")
        self.assertEqual(num2words(1234, lang="uz"), "bir ming ikki yuz o'ttiz to'rt")
        self.assertEqual(num2words(1500, lang="uz"), "bir ming besh yuz")
        self.assertEqual(
            num2words(1999, lang="uz"), "bir ming to'qqiz yuz to'qson to'qqiz"
        )
        self.assertEqual(num2words(2000, lang="uz"), "ikki ming")
        self.assertEqual(num2words(2001, lang="uz"), "ikki ming bir")
        self.assertEqual(num2words(2020, lang="uz"), "ikki ming yigirma")
        self.assertEqual(num2words(2222, lang="uz"), "ikki ming ikki yuz yigirma ikki")
        self.assertEqual(num2words(3000, lang="uz"), "uch ming")
        self.assertEqual(num2words(3333, lang="uz"), "uch ming uch yuz o'ttiz uch")
        self.assertEqual(num2words(4000, lang="uz"), "to'rt ming")
        self.assertEqual(num2words(4444, lang="uz"), "to'rt ming to'rt yuz qirq to'rt")
        self.assertEqual(num2words(5000, lang="uz"), "besh ming")
        self.assertEqual(num2words(5555, lang="uz"), "besh ming besh yuz ellik besh")
        self.assertEqual(num2words(6000, lang="uz"), "olti ming")
        self.assertEqual(num2words(6666, lang="uz"), "olti ming olti yuz oltmish olti")
        self.assertEqual(num2words(7000, lang="uz"), "yetti ming")
        self.assertEqual(
            num2words(7777, lang="uz"), "yetti ming yetti yuz yetmish yetti"
        )
        self.assertEqual(num2words(8000, lang="uz"), "sakkiz ming")
        self.assertEqual(
            num2words(8888, lang="uz"), "sakkiz ming sakkiz yuz sakson sakkiz"
        )
        self.assertEqual(num2words(9000, lang="uz"), "to'qqiz ming")
        self.assertEqual(
            num2words(9999, lang="uz"), "to'qqiz ming to'qqiz yuz to'qson to'qqiz"
        )
        self.assertEqual(num2words(10000, lang="uz"), "o'n ming")
        self.assertEqual(num2words(10001, lang="uz"), "o'n ming bir")
        self.assertEqual(num2words(11111, lang="uz"), "o'n bir ming bir yuz o'n bir")
        self.assertEqual(num2words(12345, lang="uz"), "o'n ikki ming uch yuz qirq besh")
        self.assertEqual(num2words(20000, lang="uz"), "yigirma ming")
        self.assertEqual(num2words(50000, lang="uz"), "ellik ming")
        self.assertEqual(
            num2words(99999, lang="uz"),
            "to'qson to'qqiz ming to'qqiz yuz to'qson to'qqiz",
        )
        self.assertEqual(num2words(100000, lang="uz"), "bir yuz ming")
        self.assertEqual(
            num2words(123456, lang="uz"),
            "bir yuz yigirma uch ming to'rt yuz ellik olti",
        )
        self.assertEqual(num2words(200000, lang="uz"), "ikki yuz ming")
        self.assertEqual(num2words(500000, lang="uz"), "besh yuz ming")
        self.assertEqual(
            num2words(654321, lang="uz"),
            "olti yuz ellik to'rt ming uch yuz yigirma bir",
        )
        self.assertEqual(
            num2words(999999, lang="uz"),
            "to'qqiz yuz to'qson to'qqiz ming to'qqiz yuz to'qson to'qqiz",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="uz"), "bir million")
        self.assertEqual(num2words(1000001, lang="uz"), "bir million bir")
        self.assertEqual(
            num2words(1111111, lang="uz"),
            "bir million bir yuz o'n bir ming bir yuz o'n bir",
        )
        self.assertEqual(
            num2words(1234567, lang="uz"),
            "bir million ikki yuz o'ttiz to'rt ming besh yuz oltmish yetti",
        )
        self.assertEqual(num2words(2000000, lang="uz"), "ikki million")
        self.assertEqual(num2words(5000000, lang="uz"), "besh million")
        self.assertEqual(
            num2words(9999999, lang="uz"),
            "to'qqiz million to'qqiz yuz to'qson to'qqiz ming to'qqiz yuz to'qson to'qqiz",
        )
        self.assertEqual(num2words(10000000, lang="uz"), "o'n million")
        self.assertEqual(
            num2words(12345678, lang="uz"),
            "o'n ikki million uch yuz qirq besh ming olti yuz yetmish sakkiz",
        )
        self.assertEqual(
            num2words(99999999, lang="uz"),
            "to'qson to'qqiz million to'qqiz yuz to'qson to'qqiz ming to'qqiz yuz to'qson to'qqiz",
        )
        self.assertEqual(num2words(100000000, lang="uz"), "bir yuz million")
        self.assertEqual(
            num2words(123456789, lang="uz"),
            "bir yuz yigirma uch million to'rt yuz ellik olti ming yetti yuz sakson to'qqiz",
        )
        self.assertEqual(
            num2words(999999999, lang="uz"),
            "to'qqiz yuz to'qson to'qqiz million to'qqiz yuz to'qson to'qqiz ming to'qqiz yuz to'qson to'qqiz",
        )
        self.assertEqual(num2words(1000000000, lang="uz"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="uz"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="uz"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="uz"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="uz"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="uz"), "minus bir")
        self.assertEqual(num2words(-2, lang="uz"), "minus ikki")
        self.assertEqual(num2words(-5, lang="uz"), "minus besh")
        self.assertEqual(num2words(-10, lang="uz"), "minus o'n")
        self.assertEqual(num2words(-11, lang="uz"), "minus o'n bir")
        self.assertEqual(num2words(-20, lang="uz"), "minus yigirma")
        self.assertEqual(num2words(-50, lang="uz"), "minus ellik")
        self.assertEqual(num2words(-99, lang="uz"), "minus to'qson to'qqiz")
        self.assertEqual(num2words(-100, lang="uz"), "minus bir yuz")
        self.assertEqual(num2words(-101, lang="uz"), "minus bir yuz bir")
        self.assertEqual(num2words(-200, lang="uz"), "minus ikki yuz")
        self.assertEqual(
            num2words(-999, lang="uz"), "minus to'qqiz yuz to'qson to'qqiz"
        )
        self.assertEqual(num2words(-1000, lang="uz"), "minus bir ming")
        self.assertEqual(num2words(-1001, lang="uz"), "minus bir ming bir")
        self.assertEqual(num2words(-10000, lang="uz"), "minus o'n ming")
        self.assertEqual(num2words(-100000, lang="uz"), "minus bir yuz ming")
        self.assertEqual(num2words(-1000000, lang="uz"), "minus bir million")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="uz"), "zero point bir")
        self.assertEqual(num2words(0.5, lang="uz"), "zero point besh")
        self.assertEqual(num2words(0.9, lang="uz"), "zero point to'qqiz")
        self.assertEqual(num2words(1.1, lang="uz"), "bir point bir")
        self.assertEqual(num2words(1.5, lang="uz"), "bir point besh")
        self.assertEqual(num2words(2.5, lang="uz"), "ikki point besh")
        self.assertEqual(num2words(3.14, lang="uz"), "uch point bir to'rt")
        self.assertEqual(num2words(10.5, lang="uz"), "o'n point besh")
        self.assertEqual(num2words(11.11, lang="uz"), "o'n bir point bir bir")
        self.assertEqual(num2words(20.2, lang="uz"), "yigirma point ikki")
        self.assertEqual(
            num2words(99.99, lang="uz"), "to'qson to'qqiz point to'qqiz to'qqiz"
        )
        self.assertEqual(num2words(100.01, lang="uz"), "bir yuz point zero bir")
        self.assertEqual(num2words(100.5, lang="uz"), "bir yuz point besh")
        self.assertEqual(
            num2words(123.45, lang="uz"), "bir yuz yigirma uch point to'rt besh"
        )
        self.assertEqual(num2words(1000.5, lang="uz"), "bir ming point besh")
        self.assertEqual(
            num2words(1234.56, lang="uz"),
            "bir ming ikki yuz o'ttiz to'rt point besh olti",
        )
        self.assertEqual(num2words(10000.01, lang="uz"), "o'n ming point zero bir")
        self.assertEqual(num2words(-0.5, lang="uz"), "minus zero point besh")
        self.assertEqual(num2words(-1.5, lang="uz"), "minus bir point besh")
        self.assertEqual(num2words(-10.5, lang="uz"), "minus o'n point besh")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="uz", ordinal=True), "birinchi")
        self.assertEqual(num2words(2, lang="uz", ordinal=True), "ikkinchi")
        self.assertEqual(num2words(3, lang="uz", ordinal=True), "uchinchi")
        self.assertEqual(num2words(4, lang="uz", ordinal=True), "to'rtinchi")
        self.assertEqual(num2words(5, lang="uz", ordinal=True), "beshinchi")
        self.assertEqual(num2words(6, lang="uz", ordinal=True), "oltinchi")
        self.assertEqual(num2words(7, lang="uz", ordinal=True), "yettinchi")
        self.assertEqual(num2words(8, lang="uz", ordinal=True), "sakkizinchi")
        self.assertEqual(num2words(9, lang="uz", ordinal=True), "to'qqizinchi")
        self.assertEqual(num2words(10, lang="uz", ordinal=True), "o'ninchi")
        self.assertEqual(num2words(11, lang="uz", ordinal=True), "o'n birinchi")
        self.assertEqual(num2words(12, lang="uz", ordinal=True), "o'n ikkinchi")
        self.assertEqual(num2words(13, lang="uz", ordinal=True), "o'n uchinchi")
        self.assertEqual(num2words(14, lang="uz", ordinal=True), "o'n to'rtinchi")
        self.assertEqual(num2words(15, lang="uz", ordinal=True), "o'n beshinchi")
        self.assertEqual(num2words(16, lang="uz", ordinal=True), "o'n oltinchi")
        self.assertEqual(num2words(17, lang="uz", ordinal=True), "o'n yettinchi")
        self.assertEqual(num2words(18, lang="uz", ordinal=True), "o'n sakkizinchi")
        self.assertEqual(num2words(19, lang="uz", ordinal=True), "o'n to'qqizinchi")
        self.assertEqual(num2words(20, lang="uz", ordinal=True), "yigirmanchi")
        self.assertEqual(num2words(21, lang="uz", ordinal=True), "yigirma birinchi")
        self.assertEqual(num2words(22, lang="uz", ordinal=True), "yigirma ikkinchi")
        self.assertEqual(num2words(25, lang="uz", ordinal=True), "yigirma beshinchi")
        self.assertEqual(num2words(30, lang="uz", ordinal=True), "o'ttizinchi")
        self.assertEqual(num2words(40, lang="uz", ordinal=True), "qirqinchi")
        self.assertEqual(num2words(50, lang="uz", ordinal=True), "elliginchi")
        self.assertEqual(num2words(60, lang="uz", ordinal=True), "oltmishinchi")
        self.assertEqual(num2words(70, lang="uz", ordinal=True), "yetmishinchi")
        self.assertEqual(num2words(80, lang="uz", ordinal=True), "saksoninchi")
        self.assertEqual(num2words(90, lang="uz", ordinal=True), "to'qsoninchi")
        self.assertEqual(num2words(100, lang="uz", ordinal=True), "bir yuzinchi")
        self.assertEqual(num2words(101, lang="uz", ordinal=True), "bir yuz birinchi")
        self.assertEqual(num2words(200, lang="uz", ordinal=True), "ikki yuzinchi")
        self.assertEqual(num2words(500, lang="uz", ordinal=True), "besh yuzinchi")
        self.assertEqual(num2words(1000, lang="uz", ordinal=True), "bir minginchi")
        self.assertEqual(num2words(1001, lang="uz", ordinal=True), "bir ming birinchi")
        self.assertEqual(num2words(10000, lang="uz", ordinal=True), "o'n minginchi")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="uz", to="currency", currency="UZS"), "zero so'm"
        )
        self.assertEqual(
            num2words(0.01, lang="uz", to="currency", currency="UZS"),
            "zero so'm bir tiyin",
        )
        self.assertEqual(
            num2words(0.5, lang="uz", to="currency", currency="UZS"),
            "zero so'm ellik tiyin",
        )
        self.assertEqual(
            num2words(1, lang="uz", to="currency", currency="UZS"), "bir so'm"
        )
        self.assertEqual(
            num2words(1.5, lang="uz", to="currency", currency="UZS"),
            "bir so'm ellik tiyin",
        )
        self.assertEqual(
            num2words(0, lang="uz", to="currency", currency="USD"), "zero dollar"
        )
        self.assertEqual(
            num2words(0.01, lang="uz", to="currency", currency="USD"),
            "zero dollar bir sent",
        )
        self.assertEqual(
            num2words(0.5, lang="uz", to="currency", currency="USD"),
            "zero dollar ellik sent",
        )
        self.assertEqual(
            num2words(1, lang="uz", to="currency", currency="USD"), "bir dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="uz", to="currency", currency="USD"),
            "bir dollar ellik sent",
        )
        self.assertEqual(
            num2words(0, lang="uz", to="currency", currency="EUR"), "zero yevro"
        )
        self.assertEqual(
            num2words(0.01, lang="uz", to="currency", currency="EUR"),
            "zero yevro bir sent",
        )
        self.assertEqual(
            num2words(0.5, lang="uz", to="currency", currency="EUR"),
            "zero yevro ellik sent",
        )
        self.assertEqual(
            num2words(1, lang="uz", to="currency", currency="EUR"), "bir yevro"
        )
        self.assertEqual(
            num2words(1.5, lang="uz", to="currency", currency="EUR"),
            "bir yevro ellik sent",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="uz", to="year"), "bir ming")
        self.assertEqual(num2words(1066, lang="uz", to="year"), "bir ming oltmish olti")
        self.assertEqual(
            num2words(1492, lang="uz", to="year"), "bir ming to'rt yuz to'qson ikki"
        )
        self.assertEqual(
            num2words(1776, lang="uz", to="year"), "bir ming yetti yuz yetmish olti"
        )
        self.assertEqual(num2words(1800, lang="uz", to="year"), "bir ming sakkiz yuz")
        self.assertEqual(num2words(1900, lang="uz", to="year"), "bir ming to'qqiz yuz")
        self.assertEqual(
            num2words(1984, lang="uz", to="year"), "bir ming to'qqiz yuz sakson to'rt"
        )
        self.assertEqual(
            num2words(1999, lang="uz", to="year"),
            "bir ming to'qqiz yuz to'qson to'qqiz",
        )
        self.assertEqual(num2words(2000, lang="uz", to="year"), "ikki ming")
        self.assertEqual(num2words(2001, lang="uz", to="year"), "ikki ming bir")
        self.assertEqual(num2words(2010, lang="uz", to="year"), "ikki ming o'n")
        self.assertEqual(num2words(2020, lang="uz", to="year"), "ikki ming yigirma")
        self.assertEqual(
            num2words(2024, lang="uz", to="year"), "ikki ming yigirma to'rt"
        )
        self.assertEqual(num2words(2100, lang="uz", to="year"), "ikki ming bir yuz")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="uz"), "zero")
        self.assertEqual(num2words("1", lang="uz"), "bir")
        self.assertEqual(num2words("10", lang="uz"), "o'n")
        self.assertEqual(num2words("100", lang="uz"), "bir yuz")
        self.assertEqual(num2words("1000", lang="uz"), "bir ming")
        self.assertEqual(num2words("10000", lang="uz"), "o'n ming")
        self.assertEqual(num2words("100000", lang="uz"), "bir yuz ming")
        self.assertEqual(num2words("1000000", lang="uz"), "bir million")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="uz"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="uz"), num2words("100", lang="uz"))
        self.assertEqual(num2words(1000, lang="uz"), num2words("1000", lang="uz"))

