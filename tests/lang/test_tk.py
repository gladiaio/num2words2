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


class Num2WordsTKTest(TestCase):
    """Comprehensive test cases for Turkmen language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="tk"), "nol")
        self.assertEqual(num2words(1, lang="tk"), "bir")
        self.assertEqual(num2words(2, lang="tk"), "iki")
        self.assertEqual(num2words(3, lang="tk"), "üç")
        self.assertEqual(num2words(4, lang="tk"), "dört")
        self.assertEqual(num2words(5, lang="tk"), "bäş")
        self.assertEqual(num2words(6, lang="tk"), "alty")
        self.assertEqual(num2words(7, lang="tk"), "ýedi")
        self.assertEqual(num2words(8, lang="tk"), "sekiz")
        self.assertEqual(num2words(9, lang="tk"), "dokuz")
        self.assertEqual(num2words(10, lang="tk"), "on")
        self.assertEqual(num2words(11, lang="tk"), "on bir")
        self.assertEqual(num2words(12, lang="tk"), "on iki")
        self.assertEqual(num2words(13, lang="tk"), "on üç")
        self.assertEqual(num2words(14, lang="tk"), "on dört")
        self.assertEqual(num2words(15, lang="tk"), "on bäş")
        self.assertEqual(num2words(16, lang="tk"), "on alty")
        self.assertEqual(num2words(17, lang="tk"), "on ýedi")
        self.assertEqual(num2words(18, lang="tk"), "on sekiz")
        self.assertEqual(num2words(19, lang="tk"), "on dokuz")
        self.assertEqual(num2words(20, lang="tk"), "ýigrimi")
        self.assertEqual(num2words(21, lang="tk"), "ýigrimi bir")
        self.assertEqual(num2words(22, lang="tk"), "ýigrimi iki")
        self.assertEqual(num2words(23, lang="tk"), "ýigrimi üç")
        self.assertEqual(num2words(24, lang="tk"), "ýigrimi dört")
        self.assertEqual(num2words(25, lang="tk"), "ýigrimi bäş")
        self.assertEqual(num2words(26, lang="tk"), "ýigrimi alty")
        self.assertEqual(num2words(27, lang="tk"), "ýigrimi ýedi")
        self.assertEqual(num2words(28, lang="tk"), "ýigrimi sekiz")
        self.assertEqual(num2words(29, lang="tk"), "ýigrimi dokuz")
        self.assertEqual(num2words(30, lang="tk"), "otuz")
        self.assertEqual(num2words(31, lang="tk"), "otuz bir")
        self.assertEqual(num2words(35, lang="tk"), "otuz bäş")
        self.assertEqual(num2words(40, lang="tk"), "kyrk")
        self.assertEqual(num2words(45, lang="tk"), "kyrk bäş")
        self.assertEqual(num2words(50, lang="tk"), "elli")
        self.assertEqual(num2words(55, lang="tk"), "elli bäş")
        self.assertEqual(num2words(60, lang="tk"), "altmyş")
        self.assertEqual(num2words(65, lang="tk"), "altmyş bäş")
        self.assertEqual(num2words(70, lang="tk"), "ýetmiş")
        self.assertEqual(num2words(75, lang="tk"), "ýetmiş bäş")
        self.assertEqual(num2words(80, lang="tk"), "segsen")
        self.assertEqual(num2words(85, lang="tk"), "segsen bäş")
        self.assertEqual(num2words(90, lang="tk"), "togsan")
        self.assertEqual(num2words(95, lang="tk"), "togsan bäş")
        self.assertEqual(num2words(99, lang="tk"), "togsan dokuz")
        self.assertEqual(num2words(100, lang="tk"), "bir ýüz")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="tk"), "bir ýüz bir")
        self.assertEqual(num2words(110, lang="tk"), "bir ýüz on")
        self.assertEqual(num2words(111, lang="tk"), "bir ýüz on bir")
        self.assertEqual(num2words(120, lang="tk"), "bir ýüz ýigrimi")
        self.assertEqual(num2words(125, lang="tk"), "bir ýüz ýigrimi bäş")
        self.assertEqual(num2words(150, lang="tk"), "bir ýüz elli")
        self.assertEqual(num2words(175, lang="tk"), "bir ýüz ýetmiş bäş")
        self.assertEqual(num2words(199, lang="tk"), "bir ýüz togsan dokuz")
        self.assertEqual(num2words(200, lang="tk"), "iki ýüz")
        self.assertEqual(num2words(201, lang="tk"), "iki ýüz bir")
        self.assertEqual(num2words(210, lang="tk"), "iki ýüz on")
        self.assertEqual(num2words(220, lang="tk"), "iki ýüz ýigrimi")
        self.assertEqual(num2words(250, lang="tk"), "iki ýüz elli")
        self.assertEqual(num2words(299, lang="tk"), "iki ýüz togsan dokuz")
        self.assertEqual(num2words(300, lang="tk"), "üç ýüz")
        self.assertEqual(num2words(333, lang="tk"), "üç ýüz otuz üç")
        self.assertEqual(num2words(400, lang="tk"), "dört ýüz")
        self.assertEqual(num2words(444, lang="tk"), "dört ýüz kyrk dört")
        self.assertEqual(num2words(500, lang="tk"), "bäş ýüz")
        self.assertEqual(num2words(555, lang="tk"), "bäş ýüz elli bäş")
        self.assertEqual(num2words(600, lang="tk"), "alty ýüz")
        self.assertEqual(num2words(666, lang="tk"), "alty ýüz altmyş alty")
        self.assertEqual(num2words(700, lang="tk"), "ýedi ýüz")
        self.assertEqual(num2words(777, lang="tk"), "ýedi ýüz ýetmiş ýedi")
        self.assertEqual(num2words(800, lang="tk"), "sekiz ýüz")
        self.assertEqual(num2words(888, lang="tk"), "sekiz ýüz segsen sekiz")
        self.assertEqual(num2words(900, lang="tk"), "dokuz ýüz")
        self.assertEqual(num2words(999, lang="tk"), "dokuz ýüz togsan dokuz")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="tk"), "bir müň")
        self.assertEqual(num2words(1001, lang="tk"), "bir müň bir")
        self.assertEqual(num2words(1010, lang="tk"), "bir müň on")
        self.assertEqual(num2words(1100, lang="tk"), "bir müň bir ýüz")
        self.assertEqual(num2words(1111, lang="tk"), "bir müň bir ýüz on bir")
        self.assertEqual(num2words(1234, lang="tk"), "bir müň iki ýüz otuz dört")
        self.assertEqual(num2words(1500, lang="tk"), "bir müň bäş ýüz")
        self.assertEqual(num2words(1999, lang="tk"), "bir müň dokuz ýüz togsan dokuz")
        self.assertEqual(num2words(2000, lang="tk"), "iki müň")
        self.assertEqual(num2words(2001, lang="tk"), "iki müň bir")
        self.assertEqual(num2words(2020, lang="tk"), "iki müň ýigrimi")
        self.assertEqual(num2words(2222, lang="tk"), "iki müň iki ýüz ýigrimi iki")
        self.assertEqual(num2words(3000, lang="tk"), "üç müň")
        self.assertEqual(num2words(3333, lang="tk"), "üç müň üç ýüz otuz üç")
        self.assertEqual(num2words(4000, lang="tk"), "dört müň")
        self.assertEqual(num2words(4444, lang="tk"), "dört müň dört ýüz kyrk dört")
        self.assertEqual(num2words(5000, lang="tk"), "bäş müň")
        self.assertEqual(num2words(5555, lang="tk"), "bäş müň bäş ýüz elli bäş")
        self.assertEqual(num2words(6000, lang="tk"), "alty müň")
        self.assertEqual(num2words(6666, lang="tk"), "alty müň alty ýüz altmyş alty")
        self.assertEqual(num2words(7000, lang="tk"), "ýedi müň")
        self.assertEqual(num2words(7777, lang="tk"), "ýedi müň ýedi ýüz ýetmiş ýedi")
        self.assertEqual(num2words(8000, lang="tk"), "sekiz müň")
        self.assertEqual(num2words(8888, lang="tk"), "sekiz müň sekiz ýüz segsen sekiz")
        self.assertEqual(num2words(9000, lang="tk"), "dokuz müň")
        self.assertEqual(num2words(9999, lang="tk"), "dokuz müň dokuz ýüz togsan dokuz")
        self.assertEqual(num2words(10000, lang="tk"), "on müň")
        self.assertEqual(num2words(10001, lang="tk"), "on müň bir")
        self.assertEqual(num2words(11111, lang="tk"), "on bir müň bir ýüz on bir")
        self.assertEqual(num2words(12345, lang="tk"), "on iki müň üç ýüz kyrk bäş")
        self.assertEqual(num2words(20000, lang="tk"), "ýigrimi müň")
        self.assertEqual(num2words(50000, lang="tk"), "elli müň")
        self.assertEqual(
            num2words(99999, lang="tk"), "togsan dokuz müň dokuz ýüz togsan dokuz"
        )
        self.assertEqual(num2words(100000, lang="tk"), "bir ýüz müň")
        self.assertEqual(
            num2words(123456, lang="tk"), "bir ýüz ýigrimi üç müň dört ýüz elli alty"
        )
        self.assertEqual(num2words(200000, lang="tk"), "iki ýüz müň")
        self.assertEqual(num2words(500000, lang="tk"), "bäş ýüz müň")
        self.assertEqual(
            num2words(654321, lang="tk"), "alty ýüz elli dört müň üç ýüz ýigrimi bir"
        )
        self.assertEqual(
            num2words(999999, lang="tk"),
            "dokuz ýüz togsan dokuz müň dokuz ýüz togsan dokuz",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="tk"), "bir million")
        self.assertEqual(num2words(1000001, lang="tk"), "bir million bir")
        self.assertEqual(
            num2words(1111111, lang="tk"),
            "bir million bir ýüz on bir müň bir ýüz on bir",
        )
        self.assertEqual(
            num2words(1234567, lang="tk"),
            "bir million iki ýüz otuz dört müň bäş ýüz altmyş ýedi",
        )
        self.assertEqual(num2words(2000000, lang="tk"), "iki million")
        self.assertEqual(num2words(5000000, lang="tk"), "bäş million")
        self.assertEqual(
            num2words(9999999, lang="tk"),
            "dokuz million dokuz ýüz togsan dokuz müň dokuz ýüz togsan dokuz",
        )
        self.assertEqual(num2words(10000000, lang="tk"), "on million")
        self.assertEqual(
            num2words(12345678, lang="tk"),
            "on iki million üç ýüz kyrk bäş müň alty ýüz ýetmiş sekiz",
        )
        self.assertEqual(
            num2words(99999999, lang="tk"),
            "togsan dokuz million dokuz ýüz togsan dokuz müň dokuz ýüz togsan dokuz",
        )
        self.assertEqual(num2words(100000000, lang="tk"), "bir ýüz million")
        self.assertEqual(
            num2words(123456789, lang="tk"),
            "bir ýüz ýigrimi üç million dört ýüz elli alty müň ýedi ýüz segsen dokuz",
        )
        self.assertEqual(
            num2words(999999999, lang="tk"),
            "dokuz ýüz togsan dokuz million dokuz ýüz togsan dokuz müň dokuz ýüz togsan dokuz",
        )
        self.assertEqual(num2words(1000000000, lang="tk"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="tk"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="tk"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="tk"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="tk"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="tk"), "minus bir")
        self.assertEqual(num2words(-2, lang="tk"), "minus iki")
        self.assertEqual(num2words(-5, lang="tk"), "minus bäş")
        self.assertEqual(num2words(-10, lang="tk"), "minus on")
        self.assertEqual(num2words(-11, lang="tk"), "minus on bir")
        self.assertEqual(num2words(-20, lang="tk"), "minus ýigrimi")
        self.assertEqual(num2words(-50, lang="tk"), "minus elli")
        self.assertEqual(num2words(-99, lang="tk"), "minus togsan dokuz")
        self.assertEqual(num2words(-100, lang="tk"), "minus bir ýüz")
        self.assertEqual(num2words(-101, lang="tk"), "minus bir ýüz bir")
        self.assertEqual(num2words(-200, lang="tk"), "minus iki ýüz")
        self.assertEqual(num2words(-999, lang="tk"), "minus dokuz ýüz togsan dokuz")
        self.assertEqual(num2words(-1000, lang="tk"), "minus bir müň")
        self.assertEqual(num2words(-1001, lang="tk"), "minus bir müň bir")
        self.assertEqual(num2words(-10000, lang="tk"), "minus on müň")
        self.assertEqual(num2words(-100000, lang="tk"), "minus bir ýüz müň")
        self.assertEqual(num2words(-1000000, lang="tk"), "minus bir million")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="tk"), "nol point bir")
        self.assertEqual(num2words(0.5, lang="tk"), "nol point bäş")
        self.assertEqual(num2words(0.9, lang="tk"), "nol point dokuz")
        self.assertEqual(num2words(1.1, lang="tk"), "bir point bir")
        self.assertEqual(num2words(1.5, lang="tk"), "bir point bäş")
        self.assertEqual(num2words(2.5, lang="tk"), "iki point bäş")
        self.assertEqual(num2words(3.14, lang="tk"), "üç point bir dört")
        self.assertEqual(num2words(10.5, lang="tk"), "on point bäş")
        self.assertEqual(num2words(11.11, lang="tk"), "on bir point bir bir")
        self.assertEqual(num2words(20.2, lang="tk"), "ýigrimi point iki")
        self.assertEqual(num2words(99.99, lang="tk"), "togsan dokuz point dokuz dokuz")
        self.assertEqual(num2words(100.01, lang="tk"), "bir ýüz point nol bir")
        self.assertEqual(num2words(100.5, lang="tk"), "bir ýüz point bäş")
        self.assertEqual(
            num2words(123.45, lang="tk"), "bir ýüz ýigrimi üç point dört bäş"
        )
        self.assertEqual(num2words(1000.5, lang="tk"), "bir müň point bäş")
        self.assertEqual(
            num2words(1234.56, lang="tk"), "bir müň iki ýüz otuz dört point bäş alty"
        )
        self.assertEqual(num2words(10000.01, lang="tk"), "on müň point nol bir")
        self.assertEqual(num2words(-0.5, lang="tk"), "minus nol point bäş")
        self.assertEqual(num2words(-1.5, lang="tk"), "minus bir point bäş")
        self.assertEqual(num2words(-10.5, lang="tk"), "minus on point bäş")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="tk", ordinal=True), "birinji")
        self.assertEqual(num2words(2, lang="tk", ordinal=True), "ikinji")
        self.assertEqual(num2words(3, lang="tk", ordinal=True), "üçünji")
        self.assertEqual(num2words(4, lang="tk", ordinal=True), "dördünji")
        self.assertEqual(num2words(5, lang="tk", ordinal=True), "bäşinji")
        self.assertEqual(num2words(6, lang="tk", ordinal=True), "altynjy")
        self.assertEqual(num2words(7, lang="tk", ordinal=True), "ýedinji")
        self.assertEqual(num2words(8, lang="tk", ordinal=True), "sekizinji")
        self.assertEqual(num2words(9, lang="tk", ordinal=True), "dokuzynjy")
        self.assertEqual(num2words(10, lang="tk", ordinal=True), "onunjy")
        self.assertEqual(num2words(11, lang="tk", ordinal=True), "on birinji")
        self.assertEqual(num2words(12, lang="tk", ordinal=True), "on ikinji")
        self.assertEqual(num2words(13, lang="tk", ordinal=True), "on üçünji")
        self.assertEqual(num2words(14, lang="tk", ordinal=True), "on dördünji")
        self.assertEqual(num2words(15, lang="tk", ordinal=True), "on bäşinji")
        self.assertEqual(num2words(16, lang="tk", ordinal=True), "on altynjy")
        self.assertEqual(num2words(17, lang="tk", ordinal=True), "on ýedinji")
        self.assertEqual(num2words(18, lang="tk", ordinal=True), "on sekizinji")
        self.assertEqual(num2words(19, lang="tk", ordinal=True), "on dokuzynjy")
        self.assertEqual(num2words(20, lang="tk", ordinal=True), "ýigriminji")
        self.assertEqual(num2words(21, lang="tk", ordinal=True), "ýigrimi birinji")
        self.assertEqual(num2words(22, lang="tk", ordinal=True), "ýigrimi ikinji")
        self.assertEqual(num2words(25, lang="tk", ordinal=True), "ýigrimi bäşinji")
        self.assertEqual(num2words(30, lang="tk", ordinal=True), "otuzynjy")
        self.assertEqual(num2words(40, lang="tk", ordinal=True), "kyrkynjy")
        self.assertEqual(num2words(50, lang="tk", ordinal=True), "ellinji")
        self.assertEqual(num2words(60, lang="tk", ordinal=True), "altmyşynjy")
        self.assertEqual(num2words(70, lang="tk", ordinal=True), "ýetmişinji")
        self.assertEqual(num2words(80, lang="tk", ordinal=True), "segseninji")
        self.assertEqual(num2words(90, lang="tk", ordinal=True), "togsanynjy")
        self.assertEqual(num2words(100, lang="tk", ordinal=True), "bir ýüzünji")
        self.assertEqual(num2words(101, lang="tk", ordinal=True), "bir ýüz birinji")
        self.assertEqual(num2words(200, lang="tk", ordinal=True), "iki ýüzünji")
        self.assertEqual(num2words(500, lang="tk", ordinal=True), "bäş ýüzünji")
        self.assertEqual(num2words(1000, lang="tk", ordinal=True), "bir müňünji")
        self.assertEqual(num2words(1001, lang="tk", ordinal=True), "bir müň birinji")
        self.assertEqual(num2words(10000, lang="tk", ordinal=True), "on müňünji")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="tk", to="currency", currency="TMT"), "nol manat"
        )
        self.assertEqual(
            num2words(0.01, lang="tk", to="currency", currency="TMT"),
            "nol manat bir teňňe",
        )
        self.assertEqual(
            num2words(0.5, lang="tk", to="currency", currency="TMT"),
            "nol manat elli teňňe",
        )
        self.assertEqual(
            num2words(1, lang="tk", to="currency", currency="TMT"), "bir manat"
        )
        self.assertEqual(
            num2words(1.5, lang="tk", to="currency", currency="TMT"),
            "bir manat elli teňňe",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="tk", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="tk", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="tk", to="year"), "bir müň")
        self.assertEqual(num2words(1066, lang="tk", to="year"), "bir müň altmyş alty")
        self.assertEqual(
            num2words(1492, lang="tk", to="year"), "bir müň dört ýüz togsan iki"
        )
        self.assertEqual(
            num2words(1776, lang="tk", to="year"), "bir müň ýedi ýüz ýetmiş alty"
        )
        self.assertEqual(num2words(1800, lang="tk", to="year"), "bir müň sekiz ýüz")
        self.assertEqual(num2words(1900, lang="tk", to="year"), "bir müň dokuz ýüz")
        self.assertEqual(
            num2words(1984, lang="tk", to="year"), "bir müň dokuz ýüz segsen dört"
        )
        self.assertEqual(
            num2words(1999, lang="tk", to="year"), "bir müň dokuz ýüz togsan dokuz"
        )
        self.assertEqual(num2words(2000, lang="tk", to="year"), "iki müň")
        self.assertEqual(num2words(2001, lang="tk", to="year"), "iki müň bir")
        self.assertEqual(num2words(2010, lang="tk", to="year"), "iki müň on")
        self.assertEqual(num2words(2020, lang="tk", to="year"), "iki müň ýigrimi")
        self.assertEqual(num2words(2024, lang="tk", to="year"), "iki müň ýigrimi dört")
        self.assertEqual(num2words(2100, lang="tk", to="year"), "iki müň bir ýüz")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="tk"), "nol")
        self.assertEqual(num2words("1", lang="tk"), "bir")
        self.assertEqual(num2words("10", lang="tk"), "on")
        self.assertEqual(num2words("100", lang="tk"), "bir ýüz")
        self.assertEqual(num2words("1000", lang="tk"), "bir müň")
        self.assertEqual(num2words("10000", lang="tk"), "on müň")
        self.assertEqual(num2words("100000", lang="tk"), "bir ýüz müň")
        self.assertEqual(num2words("1000000", lang="tk"), "bir million")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="tk"), "nol")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="tk"), num2words("100", lang="tk"))
        self.assertEqual(num2words(1000, lang="tk"), num2words("1000", lang="tk"))

