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


class Num2WordsSATest(TestCase):
    """Comprehensive test cases for Sanskrit language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sa"), "शून्यम्")
        self.assertEqual(num2words(1, lang="sa"), "एकम्")
        self.assertEqual(num2words(2, lang="sa"), "द्वे")
        self.assertEqual(num2words(3, lang="sa"), "त्रीणि")
        self.assertEqual(num2words(4, lang="sa"), "चत्वारि")
        self.assertEqual(num2words(5, lang="sa"), "पञ्च")
        self.assertEqual(num2words(6, lang="sa"), "षट्")
        self.assertEqual(num2words(7, lang="sa"), "सप्त")
        self.assertEqual(num2words(8, lang="sa"), "अष्ट")
        self.assertEqual(num2words(9, lang="sa"), "नव")
        self.assertEqual(num2words(10, lang="sa"), "दश")
        self.assertEqual(num2words(11, lang="sa"), "दश एकम्")
        self.assertEqual(num2words(12, lang="sa"), "दश द्वे")
        self.assertEqual(num2words(13, lang="sa"), "दश त्रीणि")
        self.assertEqual(num2words(14, lang="sa"), "दश चत्वारि")
        self.assertEqual(num2words(15, lang="sa"), "दश पञ्च")
        self.assertEqual(num2words(16, lang="sa"), "दश षट्")
        self.assertEqual(num2words(17, lang="sa"), "दश सप्त")
        self.assertEqual(num2words(18, lang="sa"), "दश अष्ट")
        self.assertEqual(num2words(19, lang="sa"), "दश नव")
        self.assertEqual(num2words(20, lang="sa"), "विंशति")
        self.assertEqual(num2words(21, lang="sa"), "विंशति एकम्")
        self.assertEqual(num2words(22, lang="sa"), "विंशति द्वे")
        self.assertEqual(num2words(23, lang="sa"), "विंशति त्रीणि")
        self.assertEqual(num2words(24, lang="sa"), "विंशति चत्वारि")
        self.assertEqual(num2words(25, lang="sa"), "विंशति पञ्च")
        self.assertEqual(num2words(26, lang="sa"), "विंशति षट्")
        self.assertEqual(num2words(27, lang="sa"), "विंशति सप्त")
        self.assertEqual(num2words(28, lang="sa"), "विंशति अष्ट")
        self.assertEqual(num2words(29, lang="sa"), "विंशति नव")
        self.assertEqual(num2words(30, lang="sa"), "त्रिंशत्")
        self.assertEqual(num2words(31, lang="sa"), "त्रिंशत् एकम्")
        self.assertEqual(num2words(35, lang="sa"), "त्रिंशत् पञ्च")
        self.assertEqual(num2words(40, lang="sa"), "चत्वारिंशत्")
        self.assertEqual(num2words(45, lang="sa"), "चत्वारिंशत् पञ्च")
        self.assertEqual(num2words(50, lang="sa"), "पञ्चाशत्")
        self.assertEqual(num2words(55, lang="sa"), "पञ्चाशत् पञ्च")
        self.assertEqual(num2words(60, lang="sa"), "षष्टि")
        self.assertEqual(num2words(65, lang="sa"), "षष्टि पञ्च")
        self.assertEqual(num2words(70, lang="sa"), "सप्तति")
        self.assertEqual(num2words(75, lang="sa"), "सप्तति पञ्च")
        self.assertEqual(num2words(80, lang="sa"), "अशीति")
        self.assertEqual(num2words(85, lang="sa"), "अशीति पञ्च")
        self.assertEqual(num2words(90, lang="sa"), "नवति")
        self.assertEqual(num2words(95, lang="sa"), "नवति पञ्च")
        self.assertEqual(num2words(99, lang="sa"), "नवति नव")
        self.assertEqual(num2words(100, lang="sa"), "एकम् शतम्")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sa"), "एकम् शतम् एकम्")
        self.assertEqual(num2words(110, lang="sa"), "एकम् शतम् दश")
        self.assertEqual(num2words(111, lang="sa"), "एकम् शतम् दश एकम्")
        self.assertEqual(num2words(120, lang="sa"), "एकम् शतम् विंशति")
        self.assertEqual(num2words(125, lang="sa"), "एकम् शतम् विंशति पञ्च")
        self.assertEqual(num2words(150, lang="sa"), "एकम् शतम् पञ्चाशत्")
        self.assertEqual(num2words(175, lang="sa"), "एकम् शतम् सप्तति पञ्च")
        self.assertEqual(num2words(199, lang="sa"), "एकम् शतम् नवति नव")
        self.assertEqual(num2words(200, lang="sa"), "द्वे शतम्")
        self.assertEqual(num2words(201, lang="sa"), "द्वे शतम् एकम्")
        self.assertEqual(num2words(210, lang="sa"), "द्वे शतम् दश")
        self.assertEqual(num2words(220, lang="sa"), "द्वे शतम् विंशति")
        self.assertEqual(num2words(250, lang="sa"), "द्वे शतम् पञ्चाशत्")
        self.assertEqual(num2words(299, lang="sa"), "द्वे शतम् नवति नव")
        self.assertEqual(num2words(300, lang="sa"), "त्रीणि शतम्")
        self.assertEqual(num2words(333, lang="sa"), "त्रीणि शतम् त्रिंशत् त्रीणि")
        self.assertEqual(num2words(400, lang="sa"), "चत्वारि शतम्")
        self.assertEqual(num2words(444, lang="sa"), "चत्वारि शतम् चत्वारिंशत् चत्वारि")
        self.assertEqual(num2words(500, lang="sa"), "पञ्च शतम्")
        self.assertEqual(num2words(555, lang="sa"), "पञ्च शतम् पञ्चाशत् पञ्च")
        self.assertEqual(num2words(600, lang="sa"), "षट् शतम्")
        self.assertEqual(num2words(666, lang="sa"), "षट् शतम् षष्टि षट्")
        self.assertEqual(num2words(700, lang="sa"), "सप्त शतम्")
        self.assertEqual(num2words(777, lang="sa"), "सप्त शतम् सप्तति सप्त")
        self.assertEqual(num2words(800, lang="sa"), "अष्ट शतम्")
        self.assertEqual(num2words(888, lang="sa"), "अष्ट शतम् अशीति अष्ट")
        self.assertEqual(num2words(900, lang="sa"), "नव शतम्")
        self.assertEqual(num2words(999, lang="sa"), "नव शतम् नवति नव")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sa"), "एकम् सहस्रम्")
        self.assertEqual(num2words(1001, lang="sa"), "एकम् सहस्रम् एकम्")
        self.assertEqual(num2words(1010, lang="sa"), "एकम् सहस्रम् दश")
        self.assertEqual(num2words(1100, lang="sa"), "एकम् सहस्रम् एकम् शतम्")
        self.assertEqual(num2words(1111, lang="sa"), "एकम् सहस्रम् एकम् शतम् दश एकम्")
        self.assertEqual(
            num2words(1234, lang="sa"), "एकम् सहस्रम् द्वे शतम् त्रिंशत् चत्वारि"
        )
        self.assertEqual(num2words(1500, lang="sa"), "एकम् सहस्रम् पञ्च शतम्")
        self.assertEqual(num2words(1999, lang="sa"), "एकम् सहस्रम् नव शतम् नवति नव")
        self.assertEqual(num2words(2000, lang="sa"), "द्वे सहस्रम्")
        self.assertEqual(num2words(2001, lang="sa"), "द्वे सहस्रम् एकम्")
        self.assertEqual(num2words(2020, lang="sa"), "द्वे सहस्रम् विंशति")
        self.assertEqual(
            num2words(2222, lang="sa"), "द्वे सहस्रम् द्वे शतम् विंशति द्वे"
        )
        self.assertEqual(num2words(3000, lang="sa"), "त्रीणि सहस्रम्")
        self.assertEqual(
            num2words(3333, lang="sa"), "त्रीणि सहस्रम् त्रीणि शतम् त्रिंशत् त्रीणि"
        )
        self.assertEqual(num2words(4000, lang="sa"), "चत्वारि सहस्रम्")
        self.assertEqual(
            num2words(4444, lang="sa"),
            "चत्वारि सहस्रम् चत्वारि शतम् चत्वारिंशत् चत्वारि",
        )
        self.assertEqual(num2words(5000, lang="sa"), "पञ्च सहस्रम्")
        self.assertEqual(
            num2words(5555, lang="sa"), "पञ्च सहस्रम् पञ्च शतम् पञ्चाशत् पञ्च"
        )
        self.assertEqual(num2words(6000, lang="sa"), "षट् सहस्रम्")
        self.assertEqual(num2words(6666, lang="sa"), "षट् सहस्रम् षट् शतम् षष्टि षट्")
        self.assertEqual(num2words(7000, lang="sa"), "सप्त सहस्रम्")
        self.assertEqual(
            num2words(7777, lang="sa"), "सप्त सहस्रम् सप्त शतम् सप्तति सप्त"
        )
        self.assertEqual(num2words(8000, lang="sa"), "अष्ट सहस्रम्")
        self.assertEqual(
            num2words(8888, lang="sa"), "अष्ट सहस्रम् अष्ट शतम् अशीति अष्ट"
        )
        self.assertEqual(num2words(9000, lang="sa"), "नव सहस्रम्")
        self.assertEqual(num2words(9999, lang="sa"), "नव सहस्रम् नव शतम् नवति नव")
        self.assertEqual(num2words(10000, lang="sa"), "दश सहस्रम्")
        self.assertEqual(num2words(10001, lang="sa"), "दश सहस्रम् एकम्")
        self.assertEqual(
            num2words(11111, lang="sa"), "दश एकम् सहस्रम् एकम् शतम् दश एकम्"
        )
        self.assertEqual(
            num2words(12345, lang="sa"), "दश द्वे सहस्रम् त्रीणि शतम् चत्वारिंशत् पञ्च"
        )
        self.assertEqual(num2words(20000, lang="sa"), "विंशति सहस्रम्")
        self.assertEqual(num2words(50000, lang="sa"), "पञ्चाशत् सहस्रम्")
        self.assertEqual(num2words(99999, lang="sa"), "नवति नव सहस्रम् नव शतम् नवति नव")
        self.assertEqual(num2words(100000, lang="sa"), "एकम् शतम् सहस्रम्")
        self.assertEqual(
            num2words(123456, lang="sa"),
            "एकम् शतम् विंशति त्रीणि सहस्रम् चत्वारि शतम् पञ्चाशत् षट्",
        )
        self.assertEqual(num2words(200000, lang="sa"), "द्वे शतम् सहस्रम्")
        self.assertEqual(num2words(500000, lang="sa"), "पञ्च शतम् सहस्रम्")
        self.assertEqual(
            num2words(654321, lang="sa"),
            "षट् शतम् पञ्चाशत् चत्वारि सहस्रम् त्रीणि शतम् विंशति एकम्",
        )
        self.assertEqual(
            num2words(999999, lang="sa"), "नव शतम् नवति नव सहस्रम् नव शतम् नवति नव"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sa"), "एकम् दशलक्षम्")
        self.assertEqual(num2words(1000001, lang="sa"), "एकम् दशलक्षम् एकम्")
        self.assertEqual(
            num2words(1111111, lang="sa"),
            "एकम् दशलक्षम् एकम् शतम् दश एकम् सहस्रम् एकम् शतम् दश एकम्",
        )
        self.assertEqual(
            num2words(1234567, lang="sa"),
            "एकम् दशलक्षम् द्वे शतम् त्रिंशत् चत्वारि सहस्रम् पञ्च शतम् षष्टि सप्त",
        )
        self.assertEqual(num2words(2000000, lang="sa"), "द्वे दशलक्षम्")
        self.assertEqual(num2words(5000000, lang="sa"), "पञ्च दशलक्षम्")
        self.assertEqual(
            num2words(9999999, lang="sa"),
            "नव दशलक्षम् नव शतम् नवति नव सहस्रम् नव शतम् नवति नव",
        )
        self.assertEqual(num2words(10000000, lang="sa"), "दश दशलक्षम्")
        self.assertEqual(
            num2words(12345678, lang="sa"),
            "दश द्वे दशलक्षम् त्रीणि शतम् चत्वारिंशत् पञ्च सहस्रम् षट् शतम् सप्तति अष्ट",
        )
        self.assertEqual(
            num2words(99999999, lang="sa"),
            "नवति नव दशलक्षम् नव शतम् नवति नव सहस्रम् नव शतम् नवति नव",
        )
        self.assertEqual(num2words(100000000, lang="sa"), "एकम् शतम् दशलक्षम्")
        self.assertEqual(
            num2words(123456789, lang="sa"),
            "एकम् शतम् विंशति त्रीणि दशलक्षम् चत्वारि शतम् पञ्चाशत् षट् सहस्रम् सप्त शतम् अशीति नव",
        )
        self.assertEqual(
            num2words(999999999, lang="sa"),
            "नव शतम् नवति नव दशलक्षम् नव शतम् नवति नव सहस्रम् नव शतम् नवति नव",
        )
        self.assertEqual(num2words(1000000000, lang="sa"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="sa"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="sa"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="sa"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="sa"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sa"), "ऋण एकम्")
        self.assertEqual(num2words(-2, lang="sa"), "ऋण द्वे")
        self.assertEqual(num2words(-5, lang="sa"), "ऋण पञ्च")
        self.assertEqual(num2words(-10, lang="sa"), "ऋण दश")
        self.assertEqual(num2words(-11, lang="sa"), "ऋण दश एकम्")
        self.assertEqual(num2words(-20, lang="sa"), "ऋण विंशति")
        self.assertEqual(num2words(-50, lang="sa"), "ऋण पञ्चाशत्")
        self.assertEqual(num2words(-99, lang="sa"), "ऋण नवति नव")
        self.assertEqual(num2words(-100, lang="sa"), "ऋण एकम् शतम्")
        self.assertEqual(num2words(-101, lang="sa"), "ऋण एकम् शतम् एकम्")
        self.assertEqual(num2words(-200, lang="sa"), "ऋण द्वे शतम्")
        self.assertEqual(num2words(-999, lang="sa"), "ऋण नव शतम् नवति नव")
        self.assertEqual(num2words(-1000, lang="sa"), "ऋण एकम् सहस्रम्")
        self.assertEqual(num2words(-1001, lang="sa"), "ऋण एकम् सहस्रम् एकम्")
        self.assertEqual(num2words(-10000, lang="sa"), "ऋण दश सहस्रम्")
        self.assertEqual(num2words(-100000, lang="sa"), "ऋण एकम् शतम् सहस्रम्")
        self.assertEqual(num2words(-1000000, lang="sa"), "ऋण एकम् दशलक्षम्")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sa"), "शून्यम् दशमलव एकम्")
        self.assertEqual(num2words(0.5, lang="sa"), "शून्यम् दशमलव पञ्च")
        self.assertEqual(num2words(0.9, lang="sa"), "शून्यम् दशमलव नव")
        self.assertEqual(num2words(1.1, lang="sa"), "एकम् दशमलव एकम्")
        self.assertEqual(num2words(1.5, lang="sa"), "एकम् दशमलव पञ्च")
        self.assertEqual(num2words(2.5, lang="sa"), "द्वे दशमलव पञ्च")
        self.assertEqual(num2words(3.14, lang="sa"), "त्रीणि दशमलव एकम् चत्वारि")
        self.assertEqual(num2words(10.5, lang="sa"), "दश दशमलव पञ्च")
        self.assertEqual(num2words(11.11, lang="sa"), "दश एकम् दशमलव एकम् एकम्")
        self.assertEqual(num2words(20.2, lang="sa"), "विंशति दशमलव द्वे")
        self.assertEqual(num2words(99.99, lang="sa"), "नवति नव दशमलव नव नव")
        self.assertEqual(num2words(100.01, lang="sa"), "एकम् शतम् दशमलव शून्यम् एकम्")
        self.assertEqual(num2words(100.5, lang="sa"), "एकम् शतम् दशमलव पञ्च")
        self.assertEqual(
            num2words(123.45, lang="sa"), "एकम् शतम् विंशति त्रीणि दशमलव चत्वारि पञ्च"
        )
        self.assertEqual(num2words(1000.5, lang="sa"), "एकम् सहस्रम् दशमलव पञ्च")
        self.assertEqual(
            num2words(1234.56, lang="sa"),
            "एकम् सहस्रम् द्वे शतम् त्रिंशत् चत्वारि दशमलव पञ्च षट्",
        )
        self.assertEqual(num2words(10000.01, lang="sa"), "दश सहस्रम् दशमलव शून्यम् एकम्")
        self.assertEqual(num2words(-0.5, lang="sa"), "ऋण शून्यम् दशमलव पञ्च")
        self.assertEqual(num2words(-1.5, lang="sa"), "ऋण एकम् दशमलव पञ्च")
        self.assertEqual(num2words(-10.5, lang="sa"), "ऋण दश दशमलव पञ्च")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sa", ordinal=True), "एकम्-मः")
        self.assertEqual(num2words(2, lang="sa", ordinal=True), "द्वे-मः")
        self.assertEqual(num2words(3, lang="sa", ordinal=True), "त्रीणि-मः")
        self.assertEqual(num2words(4, lang="sa", ordinal=True), "चत्वारि-मः")
        self.assertEqual(num2words(5, lang="sa", ordinal=True), "पञ्च-मः")
        self.assertEqual(num2words(6, lang="sa", ordinal=True), "षट्-मः")
        self.assertEqual(num2words(7, lang="sa", ordinal=True), "सप्त-मः")
        self.assertEqual(num2words(8, lang="sa", ordinal=True), "अष्ट-मः")
        self.assertEqual(num2words(9, lang="sa", ordinal=True), "नव-मः")
        self.assertEqual(num2words(10, lang="sa", ordinal=True), "दश-मः")
        self.assertEqual(num2words(11, lang="sa", ordinal=True), "दश एकम्-मः")
        self.assertEqual(num2words(12, lang="sa", ordinal=True), "दश द्वे-मः")
        self.assertEqual(num2words(13, lang="sa", ordinal=True), "दश त्रीणि-मः")
        self.assertEqual(num2words(14, lang="sa", ordinal=True), "दश चत्वारि-मः")
        self.assertEqual(num2words(15, lang="sa", ordinal=True), "दश पञ्च-मः")
        self.assertEqual(num2words(16, lang="sa", ordinal=True), "दश षट्-मः")
        self.assertEqual(num2words(17, lang="sa", ordinal=True), "दश सप्त-मः")
        self.assertEqual(num2words(18, lang="sa", ordinal=True), "दश अष्ट-मः")
        self.assertEqual(num2words(19, lang="sa", ordinal=True), "दश नव-मः")
        self.assertEqual(num2words(20, lang="sa", ordinal=True), "विंशति-मः")
        self.assertEqual(num2words(21, lang="sa", ordinal=True), "विंशति एकम्-मः")
        self.assertEqual(num2words(22, lang="sa", ordinal=True), "विंशति द्वे-मः")
        self.assertEqual(num2words(25, lang="sa", ordinal=True), "विंशति पञ्च-मः")
        self.assertEqual(num2words(30, lang="sa", ordinal=True), "त्रिंशत्-मः")
        self.assertEqual(num2words(40, lang="sa", ordinal=True), "चत्वारिंशत्-मः")
        self.assertEqual(num2words(50, lang="sa", ordinal=True), "पञ्चाशत्-मः")
        self.assertEqual(num2words(60, lang="sa", ordinal=True), "षष्टि-मः")
        self.assertEqual(num2words(70, lang="sa", ordinal=True), "सप्तति-मः")
        self.assertEqual(num2words(80, lang="sa", ordinal=True), "अशीति-मः")
        self.assertEqual(num2words(90, lang="sa", ordinal=True), "नवति-मः")
        self.assertEqual(num2words(100, lang="sa", ordinal=True), "एकम् शतम्-मः")
        self.assertEqual(num2words(101, lang="sa", ordinal=True), "एकम् शतम् एकम्-मः")
        self.assertEqual(num2words(200, lang="sa", ordinal=True), "द्वे शतम्-मः")
        self.assertEqual(num2words(500, lang="sa", ordinal=True), "पञ्च शतम्-मः")
        self.assertEqual(num2words(1000, lang="sa", ordinal=True), "एकम् सहस्रम्-मः")
        self.assertEqual(
            num2words(1001, lang="sa", ordinal=True), "एकम् सहस्रम् एकम्-मः"
        )
        self.assertEqual(num2words(10000, lang="sa", ordinal=True), "दश सहस्रम्-मः")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sa", to="currency", currency="INR"), "शून्यम् रूप्यकाणि"
        )
        self.assertEqual(
            num2words(0.01, lang="sa", to="currency", currency="INR"),
            "शून्यम् रूप्यकाणि एकम् पैसा",
        )
        self.assertEqual(
            num2words(0.5, lang="sa", to="currency", currency="INR"),
            "शून्यम् रूप्यकाणि पञ्चाशत् पैसा",
        )
        self.assertEqual(
            num2words(1, lang="sa", to="currency", currency="INR"), "एकम् रूप्यकाणि"
        )
        self.assertEqual(
            num2words(1.5, lang="sa", to="currency", currency="INR"),
            "एकम् रूप्यकाणि पञ्चाशत् पैसा",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sa", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sa", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sa", to="year"), "एकम् सहस्रम्")
        self.assertEqual(
            num2words(1066, lang="sa", to="year"), "एकम् सहस्रम् षष्टि षट्"
        )
        self.assertEqual(
            num2words(1492, lang="sa", to="year"), "एकम् सहस्रम् चत्वारि शतम् नवति द्वे"
        )
        self.assertEqual(
            num2words(1776, lang="sa", to="year"), "एकम् सहस्रम् सप्त शतम् सप्तति षट्"
        )
        self.assertEqual(
            num2words(1800, lang="sa", to="year"), "एकम् सहस्रम् अष्ट शतम्"
        )
        self.assertEqual(num2words(1900, lang="sa", to="year"), "एकम् सहस्रम् नव शतम्")
        self.assertEqual(
            num2words(1984, lang="sa", to="year"), "एकम् सहस्रम् नव शतम् अशीति चत्वारि"
        )
        self.assertEqual(
            num2words(1999, lang="sa", to="year"), "एकम् सहस्रम् नव शतम् नवति नव"
        )
        self.assertEqual(num2words(2000, lang="sa", to="year"), "द्वे सहस्रम्")
        self.assertEqual(num2words(2001, lang="sa", to="year"), "द्वे सहस्रम् एकम्")
        self.assertEqual(num2words(2010, lang="sa", to="year"), "द्वे सहस्रम् दश")
        self.assertEqual(num2words(2020, lang="sa", to="year"), "द्वे सहस्रम् विंशति")
        self.assertEqual(
            num2words(2024, lang="sa", to="year"), "द्वे सहस्रम् विंशति चत्वारि"
        )
        self.assertEqual(
            num2words(2100, lang="sa", to="year"), "द्वे सहस्रम् एकम् शतम्"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sa"), "शून्यम्")
        self.assertEqual(num2words("1", lang="sa"), "एकम्")
        self.assertEqual(num2words("10", lang="sa"), "दश")
        self.assertEqual(num2words("100", lang="sa"), "एकम् शतम्")
        self.assertEqual(num2words("1000", lang="sa"), "एकम् सहस्रम्")
        self.assertEqual(num2words("10000", lang="sa"), "दश सहस्रम्")
        self.assertEqual(num2words("100000", lang="sa"), "एकम् शतम् सहस्रम्")
        self.assertEqual(num2words("1000000", lang="sa"), "एकम् दशलक्षम्")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sa"), "शून्यम्")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sa"), num2words("100", lang="sa"))
        self.assertEqual(num2words(1000, lang="sa"), num2words("1000", lang="sa"))

