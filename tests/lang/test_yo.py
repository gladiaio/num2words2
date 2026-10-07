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


class Num2WordsYOTest(TestCase):
    """Comprehensive test cases for Yoruba language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="yo"), "òdo")
        self.assertEqual(num2words(1, lang="yo"), "ọkan")
        self.assertEqual(num2words(2, lang="yo"), "méjì")
        self.assertEqual(num2words(3, lang="yo"), "mẹta")
        self.assertEqual(num2words(4, lang="yo"), "mẹrin")
        self.assertEqual(num2words(5, lang="yo"), "marun")
        self.assertEqual(num2words(6, lang="yo"), "mẹfa")
        self.assertEqual(num2words(7, lang="yo"), "meje")
        self.assertEqual(num2words(8, lang="yo"), "mẹjọ")
        self.assertEqual(num2words(9, lang="yo"), "mẹsan")
        self.assertEqual(num2words(10, lang="yo"), "mẹwa")
        self.assertEqual(num2words(11, lang="yo"), "mẹwa ọkan")
        self.assertEqual(num2words(12, lang="yo"), "mẹwa méjì")
        self.assertEqual(num2words(13, lang="yo"), "mẹwa mẹta")
        self.assertEqual(num2words(14, lang="yo"), "mẹwa mẹrin")
        self.assertEqual(num2words(15, lang="yo"), "mẹwa marun")
        self.assertEqual(num2words(16, lang="yo"), "mẹwa mẹfa")
        self.assertEqual(num2words(17, lang="yo"), "mẹwa meje")
        self.assertEqual(num2words(18, lang="yo"), "mẹwa mẹjọ")
        self.assertEqual(num2words(19, lang="yo"), "mẹwa mẹsan")
        self.assertEqual(num2words(20, lang="yo"), "ogún")
        self.assertEqual(num2words(21, lang="yo"), "ogún ọkan")
        self.assertEqual(num2words(22, lang="yo"), "ogún méjì")
        self.assertEqual(num2words(23, lang="yo"), "ogún mẹta")
        self.assertEqual(num2words(24, lang="yo"), "ogún mẹrin")
        self.assertEqual(num2words(25, lang="yo"), "ogún marun")
        self.assertEqual(num2words(26, lang="yo"), "ogún mẹfa")
        self.assertEqual(num2words(27, lang="yo"), "ogún meje")
        self.assertEqual(num2words(28, lang="yo"), "ogún mẹjọ")
        self.assertEqual(num2words(29, lang="yo"), "ogún mẹsan")
        self.assertEqual(num2words(30, lang="yo"), "ọgbọn")
        self.assertEqual(num2words(31, lang="yo"), "ọgbọn ọkan")
        self.assertEqual(num2words(35, lang="yo"), "ọgbọn marun")
        self.assertEqual(num2words(40, lang="yo"), "ogójì")
        self.assertEqual(num2words(45, lang="yo"), "ogójì marun")
        self.assertEqual(num2words(50, lang="yo"), "àádọta")
        self.assertEqual(num2words(55, lang="yo"), "àádọta marun")
        self.assertEqual(num2words(60, lang="yo"), "ọgọta")
        self.assertEqual(num2words(65, lang="yo"), "ọgọta marun")
        self.assertEqual(num2words(70, lang="yo"), "àádọrin")
        self.assertEqual(num2words(75, lang="yo"), "àádọrin marun")
        self.assertEqual(num2words(80, lang="yo"), "ọgọrin")
        self.assertEqual(num2words(85, lang="yo"), "ọgọrin marun")
        self.assertEqual(num2words(90, lang="yo"), "àádọrún")
        self.assertEqual(num2words(95, lang="yo"), "àádọrún marun")
        self.assertEqual(num2words(99, lang="yo"), "àádọrún mẹsan")
        self.assertEqual(num2words(100, lang="yo"), "ọkan ọgọrun")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="yo"), "ọkan ọgọrun ọkan")
        self.assertEqual(num2words(110, lang="yo"), "ọkan ọgọrun mẹwa")
        self.assertEqual(num2words(111, lang="yo"), "ọkan ọgọrun mẹwa ọkan")
        self.assertEqual(num2words(120, lang="yo"), "ọkan ọgọrun ogún")
        self.assertEqual(num2words(125, lang="yo"), "ọkan ọgọrun ogún marun")
        self.assertEqual(num2words(150, lang="yo"), "ọkan ọgọrun àádọta")
        self.assertEqual(num2words(175, lang="yo"), "ọkan ọgọrun àádọrin marun")
        self.assertEqual(num2words(199, lang="yo"), "ọkan ọgọrun àádọrún mẹsan")
        self.assertEqual(num2words(200, lang="yo"), "méjì ọgọrun")
        self.assertEqual(num2words(201, lang="yo"), "méjì ọgọrun ọkan")
        self.assertEqual(num2words(210, lang="yo"), "méjì ọgọrun mẹwa")
        self.assertEqual(num2words(220, lang="yo"), "méjì ọgọrun ogún")
        self.assertEqual(num2words(250, lang="yo"), "méjì ọgọrun àádọta")
        self.assertEqual(num2words(299, lang="yo"), "méjì ọgọrun àádọrún mẹsan")
        self.assertEqual(num2words(300, lang="yo"), "mẹta ọgọrun")
        self.assertEqual(num2words(333, lang="yo"), "mẹta ọgọrun ọgbọn mẹta")
        self.assertEqual(num2words(400, lang="yo"), "mẹrin ọgọrun")
        self.assertEqual(num2words(444, lang="yo"), "mẹrin ọgọrun ogójì mẹrin")
        self.assertEqual(num2words(500, lang="yo"), "marun ọgọrun")
        self.assertEqual(num2words(555, lang="yo"), "marun ọgọrun àádọta marun")
        self.assertEqual(num2words(600, lang="yo"), "mẹfa ọgọrun")
        self.assertEqual(num2words(666, lang="yo"), "mẹfa ọgọrun ọgọta mẹfa")
        self.assertEqual(num2words(700, lang="yo"), "meje ọgọrun")
        self.assertEqual(num2words(777, lang="yo"), "meje ọgọrun àádọrin meje")
        self.assertEqual(num2words(800, lang="yo"), "mẹjọ ọgọrun")
        self.assertEqual(num2words(888, lang="yo"), "mẹjọ ọgọrun ọgọrin mẹjọ")
        self.assertEqual(num2words(900, lang="yo"), "mẹsan ọgọrun")
        self.assertEqual(num2words(999, lang="yo"), "mẹsan ọgọrun àádọrún mẹsan")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="yo"), "ọkan ẹgbẹrun")
        self.assertEqual(num2words(1001, lang="yo"), "ọkan ẹgbẹrun ọkan")
        self.assertEqual(num2words(1010, lang="yo"), "ọkan ẹgbẹrun mẹwa")
        self.assertEqual(num2words(1100, lang="yo"), "ọkan ẹgbẹrun ọkan ọgọrun")
        self.assertEqual(
            num2words(1111, lang="yo"), "ọkan ẹgbẹrun ọkan ọgọrun mẹwa ọkan"
        )
        self.assertEqual(
            num2words(1234, lang="yo"), "ọkan ẹgbẹrun méjì ọgọrun ọgbọn mẹrin"
        )
        self.assertEqual(num2words(1500, lang="yo"), "ọkan ẹgbẹrun marun ọgọrun")
        self.assertEqual(
            num2words(1999, lang="yo"), "ọkan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan"
        )
        self.assertEqual(num2words(2000, lang="yo"), "méjì ẹgbẹrun")
        self.assertEqual(num2words(2001, lang="yo"), "méjì ẹgbẹrun ọkan")
        self.assertEqual(num2words(2020, lang="yo"), "méjì ẹgbẹrun ogún")
        self.assertEqual(
            num2words(2222, lang="yo"), "méjì ẹgbẹrun méjì ọgọrun ogún méjì"
        )
        self.assertEqual(num2words(3000, lang="yo"), "mẹta ẹgbẹrun")
        self.assertEqual(
            num2words(3333, lang="yo"), "mẹta ẹgbẹrun mẹta ọgọrun ọgbọn mẹta"
        )
        self.assertEqual(num2words(4000, lang="yo"), "mẹrin ẹgbẹrun")
        self.assertEqual(
            num2words(4444, lang="yo"), "mẹrin ẹgbẹrun mẹrin ọgọrun ogójì mẹrin"
        )
        self.assertEqual(num2words(5000, lang="yo"), "marun ẹgbẹrun")
        self.assertEqual(
            num2words(5555, lang="yo"), "marun ẹgbẹrun marun ọgọrun àádọta marun"
        )
        self.assertEqual(num2words(6000, lang="yo"), "mẹfa ẹgbẹrun")
        self.assertEqual(
            num2words(6666, lang="yo"), "mẹfa ẹgbẹrun mẹfa ọgọrun ọgọta mẹfa"
        )
        self.assertEqual(num2words(7000, lang="yo"), "meje ẹgbẹrun")
        self.assertEqual(
            num2words(7777, lang="yo"), "meje ẹgbẹrun meje ọgọrun àádọrin meje"
        )
        self.assertEqual(num2words(8000, lang="yo"), "mẹjọ ẹgbẹrun")
        self.assertEqual(
            num2words(8888, lang="yo"), "mẹjọ ẹgbẹrun mẹjọ ọgọrun ọgọrin mẹjọ"
        )
        self.assertEqual(num2words(9000, lang="yo"), "mẹsan ẹgbẹrun")
        self.assertEqual(
            num2words(9999, lang="yo"), "mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan"
        )
        self.assertEqual(num2words(10000, lang="yo"), "mẹwa ẹgbẹrun")
        self.assertEqual(num2words(10001, lang="yo"), "mẹwa ẹgbẹrun ọkan")
        self.assertEqual(
            num2words(11111, lang="yo"), "mẹwa ọkan ẹgbẹrun ọkan ọgọrun mẹwa ọkan"
        )
        self.assertEqual(
            num2words(12345, lang="yo"), "mẹwa méjì ẹgbẹrun mẹta ọgọrun ogójì marun"
        )
        self.assertEqual(num2words(20000, lang="yo"), "ogún ẹgbẹrun")
        self.assertEqual(num2words(50000, lang="yo"), "àádọta ẹgbẹrun")
        self.assertEqual(
            num2words(99999, lang="yo"),
            "àádọrún mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )
        self.assertEqual(num2words(100000, lang="yo"), "ọkan ọgọrun ẹgbẹrun")
        self.assertEqual(
            num2words(123456, lang="yo"),
            "ọkan ọgọrun ogún mẹta ẹgbẹrun mẹrin ọgọrun àádọta mẹfa",
        )
        self.assertEqual(num2words(200000, lang="yo"), "méjì ọgọrun ẹgbẹrun")
        self.assertEqual(num2words(500000, lang="yo"), "marun ọgọrun ẹgbẹrun")
        self.assertEqual(
            num2words(654321, lang="yo"),
            "mẹfa ọgọrun àádọta mẹrin ẹgbẹrun mẹta ọgọrun ogún ọkan",
        )
        self.assertEqual(
            num2words(999999, lang="yo"),
            "mẹsan ọgọrun àádọrún mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="yo"), "ọkan miliọnu")
        self.assertEqual(num2words(1000001, lang="yo"), "ọkan miliọnu ọkan")
        self.assertEqual(
            num2words(1111111, lang="yo"),
            "ọkan miliọnu ọkan ọgọrun mẹwa ọkan ẹgbẹrun ọkan ọgọrun mẹwa ọkan",
        )
        self.assertEqual(
            num2words(1234567, lang="yo"),
            "ọkan miliọnu méjì ọgọrun ọgbọn mẹrin ẹgbẹrun marun ọgọrun ọgọta meje",
        )
        self.assertEqual(num2words(2000000, lang="yo"), "méjì miliọnu")
        self.assertEqual(num2words(5000000, lang="yo"), "marun miliọnu")
        self.assertEqual(
            num2words(9999999, lang="yo"),
            "mẹsan miliọnu mẹsan ọgọrun àádọrún mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )
        self.assertEqual(num2words(10000000, lang="yo"), "mẹwa miliọnu")
        self.assertEqual(
            num2words(12345678, lang="yo"),
            "mẹwa méjì miliọnu mẹta ọgọrun ogójì marun ẹgbẹrun mẹfa ọgọrun àádọrin mẹjọ",
        )
        self.assertEqual(
            num2words(99999999, lang="yo"),
            "àádọrún mẹsan miliọnu mẹsan ọgọrun àádọrún mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )
        self.assertEqual(num2words(100000000, lang="yo"), "ọkan ọgọrun miliọnu")
        self.assertEqual(
            num2words(123456789, lang="yo"),
            "ọkan ọgọrun ogún mẹta miliọnu mẹrin ọgọrun àádọta mẹfa ẹgbẹrun meje ọgọrun ọgọrin mẹsan",
        )
        self.assertEqual(
            num2words(999999999, lang="yo"),
            "mẹsan ọgọrun àádọrún mẹsan miliọnu mẹsan ọgọrun àádọrún mẹsan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )
        self.assertEqual(num2words(1000000000, lang="yo"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="yo"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="yo"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="yo"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="yo"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="yo"), "minus ọkan")
        self.assertEqual(num2words(-2, lang="yo"), "minus méjì")
        self.assertEqual(num2words(-5, lang="yo"), "minus marun")
        self.assertEqual(num2words(-10, lang="yo"), "minus mẹwa")
        self.assertEqual(num2words(-11, lang="yo"), "minus mẹwa ọkan")
        self.assertEqual(num2words(-20, lang="yo"), "minus ogún")
        self.assertEqual(num2words(-50, lang="yo"), "minus àádọta")
        self.assertEqual(num2words(-99, lang="yo"), "minus àádọrún mẹsan")
        self.assertEqual(num2words(-100, lang="yo"), "minus ọkan ọgọrun")
        self.assertEqual(num2words(-101, lang="yo"), "minus ọkan ọgọrun ọkan")
        self.assertEqual(num2words(-200, lang="yo"), "minus méjì ọgọrun")
        self.assertEqual(num2words(-999, lang="yo"), "minus mẹsan ọgọrun àádọrún mẹsan")
        self.assertEqual(num2words(-1000, lang="yo"), "minus ọkan ẹgbẹrun")
        self.assertEqual(num2words(-1001, lang="yo"), "minus ọkan ẹgbẹrun ọkan")
        self.assertEqual(num2words(-10000, lang="yo"), "minus mẹwa ẹgbẹrun")
        self.assertEqual(num2words(-100000, lang="yo"), "minus ọkan ọgọrun ẹgbẹrun")
        self.assertEqual(num2words(-1000000, lang="yo"), "minus ọkan miliọnu")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="yo"), "òdo point ọkan")
        self.assertEqual(num2words(0.5, lang="yo"), "òdo point marun")
        self.assertEqual(num2words(0.9, lang="yo"), "òdo point mẹsan")
        self.assertEqual(num2words(1.1, lang="yo"), "ọkan point ọkan")
        self.assertEqual(num2words(1.5, lang="yo"), "ọkan point marun")
        self.assertEqual(num2words(2.5, lang="yo"), "méjì point marun")
        self.assertEqual(num2words(3.14, lang="yo"), "mẹta point ọkan mẹrin")
        self.assertEqual(num2words(10.5, lang="yo"), "mẹwa point marun")
        self.assertEqual(num2words(11.11, lang="yo"), "mẹwa ọkan point ọkan ọkan")
        self.assertEqual(num2words(20.2, lang="yo"), "ogún point méjì")
        self.assertEqual(num2words(99.99, lang="yo"), "àádọrún mẹsan point mẹsan mẹsan")
        self.assertEqual(num2words(100.01, lang="yo"), "ọkan ọgọrun point òdo ọkan")
        self.assertEqual(num2words(100.5, lang="yo"), "ọkan ọgọrun point marun")
        self.assertEqual(
            num2words(123.45, lang="yo"), "ọkan ọgọrun ogún mẹta point mẹrin marun"
        )
        self.assertEqual(num2words(1000.5, lang="yo"), "ọkan ẹgbẹrun point marun")
        self.assertEqual(
            num2words(1234.56, lang="yo"),
            "ọkan ẹgbẹrun méjì ọgọrun ọgbọn mẹrin point marun mẹfa",
        )
        self.assertEqual(num2words(10000.01, lang="yo"), "mẹwa ẹgbẹrun point òdo ọkan")
        self.assertEqual(num2words(-0.5, lang="yo"), "minus òdo point marun")
        self.assertEqual(num2words(-1.5, lang="yo"), "minus ọkan point marun")
        self.assertEqual(num2words(-10.5, lang="yo"), "minus mẹwa point marun")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="yo", ordinal=True), "ọkan-kẹta")
        self.assertEqual(num2words(2, lang="yo", ordinal=True), "méjì-kẹta")
        self.assertEqual(num2words(3, lang="yo", ordinal=True), "mẹta-kẹta")
        self.assertEqual(num2words(4, lang="yo", ordinal=True), "mẹrin-kẹta")
        self.assertEqual(num2words(5, lang="yo", ordinal=True), "marun-kẹta")
        self.assertEqual(num2words(6, lang="yo", ordinal=True), "mẹfa-kẹta")
        self.assertEqual(num2words(7, lang="yo", ordinal=True), "meje-kẹta")
        self.assertEqual(num2words(8, lang="yo", ordinal=True), "mẹjọ-kẹta")
        self.assertEqual(num2words(9, lang="yo", ordinal=True), "mẹsan-kẹta")
        self.assertEqual(num2words(10, lang="yo", ordinal=True), "mẹwa-kẹta")
        self.assertEqual(num2words(11, lang="yo", ordinal=True), "mẹwa ọkan-kẹta")
        self.assertEqual(num2words(12, lang="yo", ordinal=True), "mẹwa méjì-kẹta")
        self.assertEqual(num2words(13, lang="yo", ordinal=True), "mẹwa mẹta-kẹta")
        self.assertEqual(num2words(14, lang="yo", ordinal=True), "mẹwa mẹrin-kẹta")
        self.assertEqual(num2words(15, lang="yo", ordinal=True), "mẹwa marun-kẹta")
        self.assertEqual(num2words(16, lang="yo", ordinal=True), "mẹwa mẹfa-kẹta")
        self.assertEqual(num2words(17, lang="yo", ordinal=True), "mẹwa meje-kẹta")
        self.assertEqual(num2words(18, lang="yo", ordinal=True), "mẹwa mẹjọ-kẹta")
        self.assertEqual(num2words(19, lang="yo", ordinal=True), "mẹwa mẹsan-kẹta")
        self.assertEqual(num2words(20, lang="yo", ordinal=True), "ogún-kẹta")
        self.assertEqual(num2words(21, lang="yo", ordinal=True), "ogún ọkan-kẹta")
        self.assertEqual(num2words(22, lang="yo", ordinal=True), "ogún méjì-kẹta")
        self.assertEqual(num2words(25, lang="yo", ordinal=True), "ogún marun-kẹta")
        self.assertEqual(num2words(30, lang="yo", ordinal=True), "ọgbọn-kẹta")
        self.assertEqual(num2words(40, lang="yo", ordinal=True), "ogójì-kẹta")
        self.assertEqual(num2words(50, lang="yo", ordinal=True), "àádọta-kẹta")
        self.assertEqual(num2words(60, lang="yo", ordinal=True), "ọgọta-kẹta")
        self.assertEqual(num2words(70, lang="yo", ordinal=True), "àádọrin-kẹta")
        self.assertEqual(num2words(80, lang="yo", ordinal=True), "ọgọrin-kẹta")
        self.assertEqual(num2words(90, lang="yo", ordinal=True), "àádọrún-kẹta")
        self.assertEqual(num2words(100, lang="yo", ordinal=True), "ọkan ọgọrun-kẹta")
        self.assertEqual(
            num2words(101, lang="yo", ordinal=True), "ọkan ọgọrun ọkan-kẹta"
        )
        self.assertEqual(num2words(200, lang="yo", ordinal=True), "méjì ọgọrun-kẹta")
        self.assertEqual(num2words(500, lang="yo", ordinal=True), "marun ọgọrun-kẹta")
        self.assertEqual(num2words(1000, lang="yo", ordinal=True), "ọkan ẹgbẹrun-kẹta")
        self.assertEqual(
            num2words(1001, lang="yo", ordinal=True), "ọkan ẹgbẹrun ọkan-kẹta"
        )
        self.assertEqual(num2words(10000, lang="yo", ordinal=True), "mẹwa ẹgbẹrun-kẹta")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="yo", to="currency", currency="NGN"), "òdo náírà"
        )
        self.assertEqual(
            num2words(0.01, lang="yo", to="currency", currency="NGN"),
            "òdo náírà ọkan kóbò",
        )
        self.assertEqual(
            num2words(0.5, lang="yo", to="currency", currency="NGN"),
            "òdo náírà àádọta kóbò",
        )
        self.assertEqual(
            num2words(1, lang="yo", to="currency", currency="NGN"), "ọkan náírà"
        )
        self.assertEqual(
            num2words(1.5, lang="yo", to="currency", currency="NGN"),
            "ọkan náírà àádọta kóbò",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="yo", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="yo", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="yo", to="year"), "ọkan ẹgbẹrun")
        self.assertEqual(
            num2words(1066, lang="yo", to="year"), "ọkan ẹgbẹrun ọgọta mẹfa"
        )
        self.assertEqual(
            num2words(1492, lang="yo", to="year"),
            "ọkan ẹgbẹrun mẹrin ọgọrun àádọrún méjì",
        )
        self.assertEqual(
            num2words(1776, lang="yo", to="year"),
            "ọkan ẹgbẹrun meje ọgọrun àádọrin mẹfa",
        )
        self.assertEqual(
            num2words(1800, lang="yo", to="year"), "ọkan ẹgbẹrun mẹjọ ọgọrun"
        )
        self.assertEqual(
            num2words(1900, lang="yo", to="year"), "ọkan ẹgbẹrun mẹsan ọgọrun"
        )
        self.assertEqual(
            num2words(1984, lang="yo", to="year"),
            "ọkan ẹgbẹrun mẹsan ọgọrun ọgọrin mẹrin",
        )
        self.assertEqual(
            num2words(1999, lang="yo", to="year"),
            "ọkan ẹgbẹrun mẹsan ọgọrun àádọrún mẹsan",
        )
        self.assertEqual(num2words(2000, lang="yo", to="year"), "méjì ẹgbẹrun")
        self.assertEqual(num2words(2001, lang="yo", to="year"), "méjì ẹgbẹrun ọkan")
        self.assertEqual(num2words(2010, lang="yo", to="year"), "méjì ẹgbẹrun mẹwa")
        self.assertEqual(num2words(2020, lang="yo", to="year"), "méjì ẹgbẹrun ogún")
        self.assertEqual(
            num2words(2024, lang="yo", to="year"), "méjì ẹgbẹrun ogún mẹrin"
        )
        self.assertEqual(
            num2words(2100, lang="yo", to="year"), "méjì ẹgbẹrun ọkan ọgọrun"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="yo"), "òdo")
        self.assertEqual(num2words("1", lang="yo"), "ọkan")
        self.assertEqual(num2words("10", lang="yo"), "mẹwa")
        self.assertEqual(num2words("100", lang="yo"), "ọkan ọgọrun")
        self.assertEqual(num2words("1000", lang="yo"), "ọkan ẹgbẹrun")
        self.assertEqual(num2words("10000", lang="yo"), "mẹwa ẹgbẹrun")
        self.assertEqual(num2words("100000", lang="yo"), "ọkan ọgọrun ẹgbẹrun")
        self.assertEqual(num2words("1000000", lang="yo"), "ọkan miliọnu")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="yo"), "òdo")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="yo"), num2words("100", lang="yo"))
        self.assertEqual(num2words(1000, lang="yo"), num2words("1000", lang="yo"))

