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


class Num2WordsLNTest(TestCase):
    """Comprehensive test cases for Lingala language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ln"), "libúngútulú")
        self.assertEqual(num2words(1, lang="ln"), "moko")
        self.assertEqual(num2words(2, lang="ln"), "míbalé")
        self.assertEqual(num2words(3, lang="ln"), "mísáto")
        self.assertEqual(num2words(4, lang="ln"), "mínei")
        self.assertEqual(num2words(5, lang="ln"), "mítáno")
        self.assertEqual(num2words(6, lang="ln"), "motóbá")
        self.assertEqual(num2words(7, lang="ln"), "sambo")
        self.assertEqual(num2words(8, lang="ln"), "mwambe")
        self.assertEqual(num2words(9, lang="ln"), "libwá")
        self.assertEqual(num2words(10, lang="ln"), "zómi")
        self.assertEqual(num2words(11, lang="ln"), "zómi moko")
        self.assertEqual(num2words(12, lang="ln"), "zómi míbalé")
        self.assertEqual(num2words(13, lang="ln"), "zómi mísáto")
        self.assertEqual(num2words(14, lang="ln"), "zómi mínei")
        self.assertEqual(num2words(15, lang="ln"), "zómi mítáno")
        self.assertEqual(num2words(16, lang="ln"), "zómi motóbá")
        self.assertEqual(num2words(17, lang="ln"), "zómi sambo")
        self.assertEqual(num2words(18, lang="ln"), "zómi mwambe")
        self.assertEqual(num2words(19, lang="ln"), "zómi libwá")
        self.assertEqual(num2words(20, lang="ln"), "ntuku míbalé")
        self.assertEqual(num2words(21, lang="ln"), "ntuku míbalé moko")
        self.assertEqual(num2words(22, lang="ln"), "ntuku míbalé míbalé")
        self.assertEqual(num2words(23, lang="ln"), "ntuku míbalé mísáto")
        self.assertEqual(num2words(24, lang="ln"), "ntuku míbalé mínei")
        self.assertEqual(num2words(25, lang="ln"), "ntuku míbalé mítáno")
        self.assertEqual(num2words(26, lang="ln"), "ntuku míbalé motóbá")
        self.assertEqual(num2words(27, lang="ln"), "ntuku míbalé sambo")
        self.assertEqual(num2words(28, lang="ln"), "ntuku míbalé mwambe")
        self.assertEqual(num2words(29, lang="ln"), "ntuku míbalé libwá")
        self.assertEqual(num2words(30, lang="ln"), "ntuku mísáto")
        self.assertEqual(num2words(31, lang="ln"), "ntuku mísáto moko")
        self.assertEqual(num2words(35, lang="ln"), "ntuku mísáto mítáno")
        self.assertEqual(num2words(40, lang="ln"), "ntuku mínei")
        self.assertEqual(num2words(45, lang="ln"), "ntuku mínei mítáno")
        self.assertEqual(num2words(50, lang="ln"), "ntuku mítáno")
        self.assertEqual(num2words(55, lang="ln"), "ntuku mítáno mítáno")
        self.assertEqual(num2words(60, lang="ln"), "ntuku motóbá")
        self.assertEqual(num2words(65, lang="ln"), "ntuku motóbá mítáno")
        self.assertEqual(num2words(70, lang="ln"), "ntuku sambo")
        self.assertEqual(num2words(75, lang="ln"), "ntuku sambo mítáno")
        self.assertEqual(num2words(80, lang="ln"), "ntuku mwambe")
        self.assertEqual(num2words(85, lang="ln"), "ntuku mwambe mítáno")
        self.assertEqual(num2words(90, lang="ln"), "ntuku libwá")
        self.assertEqual(num2words(95, lang="ln"), "ntuku libwá mítáno")
        self.assertEqual(num2words(99, lang="ln"), "ntuku libwá libwá")
        self.assertEqual(num2words(100, lang="ln"), "moko nkama")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ln"), "moko nkama moko")
        self.assertEqual(num2words(110, lang="ln"), "moko nkama zómi")
        self.assertEqual(num2words(111, lang="ln"), "moko nkama zómi moko")
        self.assertEqual(num2words(120, lang="ln"), "moko nkama ntuku míbalé")
        self.assertEqual(num2words(125, lang="ln"), "moko nkama ntuku míbalé mítáno")
        self.assertEqual(num2words(150, lang="ln"), "moko nkama ntuku mítáno")
        self.assertEqual(num2words(175, lang="ln"), "moko nkama ntuku sambo mítáno")
        self.assertEqual(num2words(199, lang="ln"), "moko nkama ntuku libwá libwá")
        self.assertEqual(num2words(200, lang="ln"), "míbalé nkama")
        self.assertEqual(num2words(201, lang="ln"), "míbalé nkama moko")
        self.assertEqual(num2words(210, lang="ln"), "míbalé nkama zómi")
        self.assertEqual(num2words(220, lang="ln"), "míbalé nkama ntuku míbalé")
        self.assertEqual(num2words(250, lang="ln"), "míbalé nkama ntuku mítáno")
        self.assertEqual(num2words(299, lang="ln"), "míbalé nkama ntuku libwá libwá")
        self.assertEqual(num2words(300, lang="ln"), "mísáto nkama")
        self.assertEqual(num2words(333, lang="ln"), "mísáto nkama ntuku mísáto mísáto")
        self.assertEqual(num2words(400, lang="ln"), "mínei nkama")
        self.assertEqual(num2words(444, lang="ln"), "mínei nkama ntuku mínei mínei")
        self.assertEqual(num2words(500, lang="ln"), "mítáno nkama")
        self.assertEqual(num2words(555, lang="ln"), "mítáno nkama ntuku mítáno mítáno")
        self.assertEqual(num2words(600, lang="ln"), "motóbá nkama")
        self.assertEqual(num2words(666, lang="ln"), "motóbá nkama ntuku motóbá motóbá")
        self.assertEqual(num2words(700, lang="ln"), "sambo nkama")
        self.assertEqual(num2words(777, lang="ln"), "sambo nkama ntuku sambo sambo")
        self.assertEqual(num2words(800, lang="ln"), "mwambe nkama")
        self.assertEqual(num2words(888, lang="ln"), "mwambe nkama ntuku mwambe mwambe")
        self.assertEqual(num2words(900, lang="ln"), "libwá nkama")
        self.assertEqual(num2words(999, lang="ln"), "libwá nkama ntuku libwá libwá")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ln"), "moko nkóto")
        self.assertEqual(num2words(1001, lang="ln"), "moko nkóto moko")
        self.assertEqual(num2words(1010, lang="ln"), "moko nkóto zómi")
        self.assertEqual(num2words(1100, lang="ln"), "moko nkóto moko nkama")
        self.assertEqual(num2words(1111, lang="ln"), "moko nkóto moko nkama zómi moko")
        self.assertEqual(
            num2words(1234, lang="ln"), "moko nkóto míbalé nkama ntuku mísáto mínei"
        )
        self.assertEqual(num2words(1500, lang="ln"), "moko nkóto mítáno nkama")
        self.assertEqual(
            num2words(1999, lang="ln"), "moko nkóto libwá nkama ntuku libwá libwá"
        )
        self.assertEqual(num2words(2000, lang="ln"), "míbalé nkóto")
        self.assertEqual(num2words(2001, lang="ln"), "míbalé nkóto moko")
        self.assertEqual(num2words(2020, lang="ln"), "míbalé nkóto ntuku míbalé")
        self.assertEqual(
            num2words(2222, lang="ln"), "míbalé nkóto míbalé nkama ntuku míbalé míbalé"
        )
        self.assertEqual(num2words(3000, lang="ln"), "mísáto nkóto")
        self.assertEqual(
            num2words(3333, lang="ln"), "mísáto nkóto mísáto nkama ntuku mísáto mísáto"
        )
        self.assertEqual(num2words(4000, lang="ln"), "mínei nkóto")
        self.assertEqual(
            num2words(4444, lang="ln"), "mínei nkóto mínei nkama ntuku mínei mínei"
        )
        self.assertEqual(num2words(5000, lang="ln"), "mítáno nkóto")
        self.assertEqual(
            num2words(5555, lang="ln"), "mítáno nkóto mítáno nkama ntuku mítáno mítáno"
        )
        self.assertEqual(num2words(6000, lang="ln"), "motóbá nkóto")
        self.assertEqual(
            num2words(6666, lang="ln"), "motóbá nkóto motóbá nkama ntuku motóbá motóbá"
        )
        self.assertEqual(num2words(7000, lang="ln"), "sambo nkóto")
        self.assertEqual(
            num2words(7777, lang="ln"), "sambo nkóto sambo nkama ntuku sambo sambo"
        )
        self.assertEqual(num2words(8000, lang="ln"), "mwambe nkóto")
        self.assertEqual(
            num2words(8888, lang="ln"), "mwambe nkóto mwambe nkama ntuku mwambe mwambe"
        )
        self.assertEqual(num2words(9000, lang="ln"), "libwá nkóto")
        self.assertEqual(
            num2words(9999, lang="ln"), "libwá nkóto libwá nkama ntuku libwá libwá"
        )
        self.assertEqual(num2words(10000, lang="ln"), "zómi nkóto")
        self.assertEqual(num2words(10001, lang="ln"), "zómi nkóto moko")
        self.assertEqual(
            num2words(11111, lang="ln"), "zómi moko nkóto moko nkama zómi moko"
        )
        self.assertEqual(
            num2words(12345, lang="ln"),
            "zómi míbalé nkóto mísáto nkama ntuku mínei mítáno",
        )
        self.assertEqual(num2words(20000, lang="ln"), "ntuku míbalé nkóto")
        self.assertEqual(num2words(50000, lang="ln"), "ntuku mítáno nkóto")
        self.assertEqual(
            num2words(99999, lang="ln"),
            "ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(100000, lang="ln"), "moko nkama nkóto")
        self.assertEqual(
            num2words(123456, lang="ln"),
            "moko nkama ntuku míbalé mísáto nkóto mínei nkama ntuku mítáno motóbá",
        )
        self.assertEqual(num2words(200000, lang="ln"), "míbalé nkama nkóto")
        self.assertEqual(num2words(500000, lang="ln"), "mítáno nkama nkóto")
        self.assertEqual(
            num2words(654321, lang="ln"),
            "motóbá nkama ntuku mítáno mínei nkóto mísáto nkama ntuku míbalé moko",
        )
        self.assertEqual(
            num2words(999999, lang="ln"),
            "libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ln"), "moko milio")
        self.assertEqual(num2words(1000001, lang="ln"), "moko milio moko")
        self.assertEqual(
            num2words(1111111, lang="ln"),
            "moko milio moko nkama zómi moko nkóto moko nkama zómi moko",
        )
        self.assertEqual(
            num2words(1234567, lang="ln"),
            "moko milio míbalé nkama ntuku mísáto mínei nkóto mítáno nkama ntuku motóbá sambo",
        )
        self.assertEqual(num2words(2000000, lang="ln"), "míbalé milio")
        self.assertEqual(num2words(5000000, lang="ln"), "mítáno milio")
        self.assertEqual(
            num2words(9999999, lang="ln"),
            "libwá milio libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(10000000, lang="ln"), "zómi milio")
        self.assertEqual(
            num2words(12345678, lang="ln"),
            "zómi míbalé milio mísáto nkama ntuku mínei mítáno nkóto motóbá nkama ntuku sambo mwambe",
        )
        self.assertEqual(
            num2words(99999999, lang="ln"),
            "ntuku libwá libwá milio libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(100000000, lang="ln"), "moko nkama milio")
        self.assertEqual(
            num2words(123456789, lang="ln"),
            "moko nkama ntuku míbalé mísáto milio mínei nkama ntuku mítáno motóbá nkóto sambo nkama ntuku mwambe libwá",
        )
        self.assertEqual(
            num2words(999999999, lang="ln"),
            "libwá nkama ntuku libwá libwá milio libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(1000000000, lang="ln"), "moko miliale")
        self.assertEqual(
            num2words(1234567890, lang="ln"),
            "moko miliale míbalé nkama ntuku mísáto mínei milio mítáno nkama ntuku motóbá sambo nkóto mwambe nkama ntuku libwá",
        )
        self.assertEqual(
            num2words(9999999999, lang="ln"),
            "libwá miliale libwá nkama ntuku libwá libwá milio libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(10000000000, lang="ln"), "zómi miliale")
        self.assertEqual(
            num2words(99999999999, lang="ln"),
            "ntuku libwá libwá miliale libwá nkama ntuku libwá libwá milio libwá nkama ntuku libwá libwá nkóto libwá nkama ntuku libwá libwá",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ln"), "moins moko")
        self.assertEqual(num2words(-2, lang="ln"), "moins míbalé")
        self.assertEqual(num2words(-5, lang="ln"), "moins mítáno")
        self.assertEqual(num2words(-10, lang="ln"), "moins zómi")
        self.assertEqual(num2words(-11, lang="ln"), "moins zómi moko")
        self.assertEqual(num2words(-20, lang="ln"), "moins ntuku míbalé")
        self.assertEqual(num2words(-50, lang="ln"), "moins ntuku mítáno")
        self.assertEqual(num2words(-99, lang="ln"), "moins ntuku libwá libwá")
        self.assertEqual(num2words(-100, lang="ln"), "moins moko nkama")
        self.assertEqual(num2words(-101, lang="ln"), "moins moko nkama moko")
        self.assertEqual(num2words(-200, lang="ln"), "moins míbalé nkama")
        self.assertEqual(
            num2words(-999, lang="ln"), "moins libwá nkama ntuku libwá libwá"
        )
        self.assertEqual(num2words(-1000, lang="ln"), "moins moko nkóto")
        self.assertEqual(num2words(-1001, lang="ln"), "moins moko nkóto moko")
        self.assertEqual(num2words(-10000, lang="ln"), "moins zómi nkóto")
        self.assertEqual(num2words(-100000, lang="ln"), "moins moko nkama nkóto")
        self.assertEqual(num2words(-1000000, lang="ln"), "moins moko milio")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ln"), "libúngútulú virgule moko")
        self.assertEqual(num2words(0.5, lang="ln"), "libúngútulú virgule mítáno")
        self.assertEqual(num2words(0.9, lang="ln"), "libúngútulú virgule libwá")
        self.assertEqual(num2words(1.1, lang="ln"), "moko virgule moko")
        self.assertEqual(num2words(1.5, lang="ln"), "moko virgule mítáno")
        self.assertEqual(num2words(2.5, lang="ln"), "míbalé virgule mítáno")
        self.assertEqual(num2words(3.14, lang="ln"), "mísáto virgule moko mínei")
        self.assertEqual(num2words(10.5, lang="ln"), "zómi virgule mítáno")
        self.assertEqual(num2words(11.11, lang="ln"), "zómi moko virgule moko moko")
        self.assertEqual(num2words(20.2, lang="ln"), "ntuku míbalé virgule míbalé")
        self.assertEqual(
            num2words(99.99, lang="ln"), "ntuku libwá libwá virgule libwá libwá"
        )
        self.assertEqual(
            num2words(100.01, lang="ln"), "moko nkama virgule libúngútulú moko"
        )
        self.assertEqual(num2words(100.5, lang="ln"), "moko nkama virgule mítáno")
        self.assertEqual(
            num2words(123.45, lang="ln"),
            "moko nkama ntuku míbalé mísáto virgule mínei mítáno",
        )
        self.assertEqual(num2words(1000.5, lang="ln"), "moko nkóto virgule mítáno")
        self.assertEqual(
            num2words(1234.56, lang="ln"),
            "moko nkóto míbalé nkama ntuku mísáto mínei virgule mítáno motóbá",
        )
        self.assertEqual(
            num2words(10000.01, lang="ln"), "zómi nkóto virgule libúngútulú moko"
        )
        self.assertEqual(num2words(-0.5, lang="ln"), "moins libúngútulú virgule mítáno")
        self.assertEqual(num2words(-1.5, lang="ln"), "moins moko virgule mítáno")
        self.assertEqual(num2words(-10.5, lang="ln"), "moins zómi virgule mítáno")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ln", ordinal=True), "moko-e")
        self.assertEqual(num2words(2, lang="ln", ordinal=True), "míbalé-e")
        self.assertEqual(num2words(3, lang="ln", ordinal=True), "mísáto-e")
        self.assertEqual(num2words(4, lang="ln", ordinal=True), "mínei-e")
        self.assertEqual(num2words(5, lang="ln", ordinal=True), "mítáno-e")
        self.assertEqual(num2words(6, lang="ln", ordinal=True), "motóbá-e")
        self.assertEqual(num2words(7, lang="ln", ordinal=True), "sambo-e")
        self.assertEqual(num2words(8, lang="ln", ordinal=True), "mwambe-e")
        self.assertEqual(num2words(9, lang="ln", ordinal=True), "libwá-e")
        self.assertEqual(num2words(10, lang="ln", ordinal=True), "zómi-e")
        self.assertEqual(num2words(11, lang="ln", ordinal=True), "zómi moko-e")
        self.assertEqual(num2words(12, lang="ln", ordinal=True), "zómi míbalé-e")
        self.assertEqual(num2words(13, lang="ln", ordinal=True), "zómi mísáto-e")
        self.assertEqual(num2words(14, lang="ln", ordinal=True), "zómi mínei-e")
        self.assertEqual(num2words(15, lang="ln", ordinal=True), "zómi mítáno-e")
        self.assertEqual(num2words(16, lang="ln", ordinal=True), "zómi motóbá-e")
        self.assertEqual(num2words(17, lang="ln", ordinal=True), "zómi sambo-e")
        self.assertEqual(num2words(18, lang="ln", ordinal=True), "zómi mwambe-e")
        self.assertEqual(num2words(19, lang="ln", ordinal=True), "zómi libwá-e")
        self.assertEqual(num2words(20, lang="ln", ordinal=True), "ntuku míbalé-e")
        self.assertEqual(num2words(21, lang="ln", ordinal=True), "ntuku míbalé moko-e")
        self.assertEqual(
            num2words(22, lang="ln", ordinal=True), "ntuku míbalé míbalé-e"
        )
        self.assertEqual(
            num2words(25, lang="ln", ordinal=True), "ntuku míbalé mítáno-e"
        )
        self.assertEqual(num2words(30, lang="ln", ordinal=True), "ntuku mísáto-e")
        self.assertEqual(num2words(40, lang="ln", ordinal=True), "ntuku mínei-e")
        self.assertEqual(num2words(50, lang="ln", ordinal=True), "ntuku mítáno-e")
        self.assertEqual(num2words(60, lang="ln", ordinal=True), "ntuku motóbá-e")
        self.assertEqual(num2words(70, lang="ln", ordinal=True), "ntuku sambo-e")
        self.assertEqual(num2words(80, lang="ln", ordinal=True), "ntuku mwambe-e")
        self.assertEqual(num2words(90, lang="ln", ordinal=True), "ntuku libwá-e")
        self.assertEqual(num2words(100, lang="ln", ordinal=True), "moko nkama-e")
        self.assertEqual(num2words(101, lang="ln", ordinal=True), "moko nkama moko-e")
        self.assertEqual(num2words(200, lang="ln", ordinal=True), "míbalé nkama-e")
        self.assertEqual(num2words(500, lang="ln", ordinal=True), "mítáno nkama-e")
        self.assertEqual(num2words(1000, lang="ln", ordinal=True), "moko nkóto-e")
        self.assertEqual(num2words(1001, lang="ln", ordinal=True), "moko nkóto moko-e")
        self.assertEqual(num2words(10000, lang="ln", ordinal=True), "zómi nkóto-e")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ln", to="currency", currency="CDF"),
            "libúngútulú faranga",
        )
        self.assertEqual(
            num2words(0.01, lang="ln", to="currency", currency="CDF"),
            "libúngútulú faranga moko santimi",
        )
        self.assertEqual(
            num2words(0.5, lang="ln", to="currency", currency="CDF"),
            "libúngútulú faranga ntuku mítáno santimi",
        )
        self.assertEqual(
            num2words(1, lang="ln", to="currency", currency="CDF"), "moko faranga"
        )
        self.assertEqual(
            num2words(1.5, lang="ln", to="currency", currency="CDF"),
            "moko faranga ntuku mítáno santimi",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="ln", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="ln", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ln", to="year"), "moko nkóto")
        self.assertEqual(
            num2words(1066, lang="ln", to="year"), "moko nkóto ntuku motóbá motóbá"
        )
        self.assertEqual(
            num2words(1492, lang="ln", to="year"),
            "moko nkóto mínei nkama ntuku libwá míbalé",
        )
        self.assertEqual(
            num2words(1776, lang="ln", to="year"),
            "moko nkóto sambo nkama ntuku sambo motóbá",
        )
        self.assertEqual(
            num2words(1800, lang="ln", to="year"), "moko nkóto mwambe nkama"
        )
        self.assertEqual(
            num2words(1900, lang="ln", to="year"), "moko nkóto libwá nkama"
        )
        self.assertEqual(
            num2words(1984, lang="ln", to="year"),
            "moko nkóto libwá nkama ntuku mwambe mínei",
        )
        self.assertEqual(
            num2words(1999, lang="ln", to="year"),
            "moko nkóto libwá nkama ntuku libwá libwá",
        )
        self.assertEqual(num2words(2000, lang="ln", to="year"), "míbalé nkóto")
        self.assertEqual(num2words(2001, lang="ln", to="year"), "míbalé nkóto moko")
        self.assertEqual(num2words(2010, lang="ln", to="year"), "míbalé nkóto zómi")
        self.assertEqual(
            num2words(2020, lang="ln", to="year"), "míbalé nkóto ntuku míbalé"
        )
        self.assertEqual(
            num2words(2024, lang="ln", to="year"), "míbalé nkóto ntuku míbalé mínei"
        )
        self.assertEqual(
            num2words(2100, lang="ln", to="year"), "míbalé nkóto moko nkama"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ln"), "libúngútulú")
        self.assertEqual(num2words("1", lang="ln"), "moko")
        self.assertEqual(num2words("10", lang="ln"), "zómi")
        self.assertEqual(num2words("100", lang="ln"), "moko nkama")
        self.assertEqual(num2words("1000", lang="ln"), "moko nkóto")
        self.assertEqual(num2words("10000", lang="ln"), "zómi nkóto")
        self.assertEqual(num2words("100000", lang="ln"), "moko nkama nkóto")
        self.assertEqual(num2words("1000000", lang="ln"), "moko milio")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ln"), "libúngútulú")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ln"), num2words("100", lang="ln"))
        self.assertEqual(num2words(1000, lang="ln"), num2words("1000", lang="ln"))
