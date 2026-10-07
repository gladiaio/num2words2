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


class Num2WordsWOTest(TestCase):
    """Comprehensive test cases for Wolof language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="wo"), "tus")
        self.assertEqual(num2words(1, lang="wo"), "benn")
        self.assertEqual(num2words(2, lang="wo"), "ñaar")
        self.assertEqual(num2words(3, lang="wo"), "ñett")
        self.assertEqual(num2words(4, lang="wo"), "ñeent")
        self.assertEqual(num2words(5, lang="wo"), "juróom")
        self.assertEqual(num2words(6, lang="wo"), "juróom-benn")
        self.assertEqual(num2words(7, lang="wo"), "juróom-ñaar")
        self.assertEqual(num2words(8, lang="wo"), "juróom-ñett")
        self.assertEqual(num2words(9, lang="wo"), "juróom-ñeent")
        self.assertEqual(num2words(10, lang="wo"), "fukk")
        self.assertEqual(num2words(11, lang="wo"), "fukk benn")
        self.assertEqual(num2words(12, lang="wo"), "fukk ñaar")
        self.assertEqual(num2words(13, lang="wo"), "fukk ñett")
        self.assertEqual(num2words(14, lang="wo"), "fukk ñeent")
        self.assertEqual(num2words(15, lang="wo"), "fukk juróom")
        self.assertEqual(num2words(16, lang="wo"), "fukk juróom-benn")
        self.assertEqual(num2words(17, lang="wo"), "fukk juróom-ñaar")
        self.assertEqual(num2words(18, lang="wo"), "fukk juróom-ñett")
        self.assertEqual(num2words(19, lang="wo"), "fukk juróom-ñeent")
        self.assertEqual(num2words(20, lang="wo"), "ñaar-fukk")
        self.assertEqual(num2words(21, lang="wo"), "ñaar-fukk benn")
        self.assertEqual(num2words(22, lang="wo"), "ñaar-fukk ñaar")
        self.assertEqual(num2words(23, lang="wo"), "ñaar-fukk ñett")
        self.assertEqual(num2words(24, lang="wo"), "ñaar-fukk ñeent")
        self.assertEqual(num2words(25, lang="wo"), "ñaar-fukk juróom")
        self.assertEqual(num2words(26, lang="wo"), "ñaar-fukk juróom-benn")
        self.assertEqual(num2words(27, lang="wo"), "ñaar-fukk juróom-ñaar")
        self.assertEqual(num2words(28, lang="wo"), "ñaar-fukk juróom-ñett")
        self.assertEqual(num2words(29, lang="wo"), "ñaar-fukk juróom-ñeent")
        self.assertEqual(num2words(30, lang="wo"), "ñett-fukk")
        self.assertEqual(num2words(31, lang="wo"), "ñett-fukk benn")
        self.assertEqual(num2words(35, lang="wo"), "ñett-fukk juróom")
        self.assertEqual(num2words(40, lang="wo"), "ñeent-fukk")
        self.assertEqual(num2words(45, lang="wo"), "ñeent-fukk juróom")
        self.assertEqual(num2words(50, lang="wo"), "juróom-fukk")
        self.assertEqual(num2words(55, lang="wo"), "juróom-fukk juróom")
        self.assertEqual(num2words(60, lang="wo"), "juróom-benn-fukk")
        self.assertEqual(num2words(65, lang="wo"), "juróom-benn-fukk juróom")
        self.assertEqual(num2words(70, lang="wo"), "juróom-ñaar-fukk")
        self.assertEqual(num2words(75, lang="wo"), "juróom-ñaar-fukk juróom")
        self.assertEqual(num2words(80, lang="wo"), "juróom-ñett-fukk")
        self.assertEqual(num2words(85, lang="wo"), "juróom-ñett-fukk juróom")
        self.assertEqual(num2words(90, lang="wo"), "juróom-ñeent-fukk")
        self.assertEqual(num2words(95, lang="wo"), "juróom-ñeent-fukk juróom")
        self.assertEqual(num2words(99, lang="wo"), "juróom-ñeent-fukk juróom-ñeent")
        self.assertEqual(num2words(100, lang="wo"), "benn téeméer")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="wo"), "benn téeméer benn")
        self.assertEqual(num2words(110, lang="wo"), "benn téeméer fukk")
        self.assertEqual(num2words(111, lang="wo"), "benn téeméer fukk benn")
        self.assertEqual(num2words(120, lang="wo"), "benn téeméer ñaar-fukk")
        self.assertEqual(num2words(125, lang="wo"), "benn téeméer ñaar-fukk juróom")
        self.assertEqual(num2words(150, lang="wo"), "benn téeméer juróom-fukk")
        self.assertEqual(
            num2words(175, lang="wo"), "benn téeméer juróom-ñaar-fukk juróom"
        )
        self.assertEqual(
            num2words(199, lang="wo"), "benn téeméer juróom-ñeent-fukk juróom-ñeent"
        )
        self.assertEqual(num2words(200, lang="wo"), "ñaar téeméer")
        self.assertEqual(num2words(201, lang="wo"), "ñaar téeméer benn")
        self.assertEqual(num2words(210, lang="wo"), "ñaar téeméer fukk")
        self.assertEqual(num2words(220, lang="wo"), "ñaar téeméer ñaar-fukk")
        self.assertEqual(num2words(250, lang="wo"), "ñaar téeméer juróom-fukk")
        self.assertEqual(
            num2words(299, lang="wo"), "ñaar téeméer juróom-ñeent-fukk juróom-ñeent"
        )
        self.assertEqual(num2words(300, lang="wo"), "ñett téeméer")
        self.assertEqual(num2words(333, lang="wo"), "ñett téeméer ñett-fukk ñett")
        self.assertEqual(num2words(400, lang="wo"), "ñeent téeméer")
        self.assertEqual(num2words(444, lang="wo"), "ñeent téeméer ñeent-fukk ñeent")
        self.assertEqual(num2words(500, lang="wo"), "juróom téeméer")
        self.assertEqual(num2words(555, lang="wo"), "juróom téeméer juróom-fukk juróom")
        self.assertEqual(num2words(600, lang="wo"), "juróom-benn téeméer")
        self.assertEqual(
            num2words(666, lang="wo"),
            "juróom-benn téeméer juróom-benn-fukk juróom-benn",
        )
        self.assertEqual(num2words(700, lang="wo"), "juróom-ñaar téeméer")
        self.assertEqual(
            num2words(777, lang="wo"),
            "juróom-ñaar téeméer juróom-ñaar-fukk juróom-ñaar",
        )
        self.assertEqual(num2words(800, lang="wo"), "juróom-ñett téeméer")
        self.assertEqual(
            num2words(888, lang="wo"),
            "juróom-ñett téeméer juróom-ñett-fukk juróom-ñett",
        )
        self.assertEqual(num2words(900, lang="wo"), "juróom-ñeent téeméer")
        self.assertEqual(
            num2words(999, lang="wo"),
            "juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="wo"), "benn junni")
        self.assertEqual(num2words(1001, lang="wo"), "benn junni benn")
        self.assertEqual(num2words(1010, lang="wo"), "benn junni fukk")
        self.assertEqual(num2words(1100, lang="wo"), "benn junni benn téeméer")
        self.assertEqual(
            num2words(1111, lang="wo"), "benn junni benn téeméer fukk benn"
        )
        self.assertEqual(
            num2words(1234, lang="wo"), "benn junni ñaar téeméer ñett-fukk ñeent"
        )
        self.assertEqual(num2words(1500, lang="wo"), "benn junni juróom téeméer")
        self.assertEqual(
            num2words(1999, lang="wo"),
            "benn junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(2000, lang="wo"), "ñaar junni")
        self.assertEqual(num2words(2001, lang="wo"), "ñaar junni benn")
        self.assertEqual(num2words(2020, lang="wo"), "ñaar junni ñaar-fukk")
        self.assertEqual(
            num2words(2222, lang="wo"), "ñaar junni ñaar téeméer ñaar-fukk ñaar"
        )
        self.assertEqual(num2words(3000, lang="wo"), "ñett junni")
        self.assertEqual(
            num2words(3333, lang="wo"), "ñett junni ñett téeméer ñett-fukk ñett"
        )
        self.assertEqual(num2words(4000, lang="wo"), "ñeent junni")
        self.assertEqual(
            num2words(4444, lang="wo"), "ñeent junni ñeent téeméer ñeent-fukk ñeent"
        )
        self.assertEqual(num2words(5000, lang="wo"), "juróom junni")
        self.assertEqual(
            num2words(5555, lang="wo"), "juróom junni juróom téeméer juróom-fukk juróom"
        )
        self.assertEqual(num2words(6000, lang="wo"), "juróom-benn junni")
        self.assertEqual(
            num2words(6666, lang="wo"),
            "juróom-benn junni juróom-benn téeméer juróom-benn-fukk juróom-benn",
        )
        self.assertEqual(num2words(7000, lang="wo"), "juróom-ñaar junni")
        self.assertEqual(
            num2words(7777, lang="wo"),
            "juróom-ñaar junni juróom-ñaar téeméer juróom-ñaar-fukk juróom-ñaar",
        )
        self.assertEqual(num2words(8000, lang="wo"), "juróom-ñett junni")
        self.assertEqual(
            num2words(8888, lang="wo"),
            "juróom-ñett junni juróom-ñett téeméer juróom-ñett-fukk juróom-ñett",
        )
        self.assertEqual(num2words(9000, lang="wo"), "juróom-ñeent junni")
        self.assertEqual(
            num2words(9999, lang="wo"),
            "juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(10000, lang="wo"), "fukk junni")
        self.assertEqual(num2words(10001, lang="wo"), "fukk junni benn")
        self.assertEqual(
            num2words(11111, lang="wo"), "fukk benn junni benn téeméer fukk benn"
        )
        self.assertEqual(
            num2words(12345, lang="wo"),
            "fukk ñaar junni ñett téeméer ñeent-fukk juróom",
        )
        self.assertEqual(num2words(20000, lang="wo"), "ñaar-fukk junni")
        self.assertEqual(num2words(50000, lang="wo"), "juróom-fukk junni")
        self.assertEqual(
            num2words(99999, lang="wo"),
            "juróom-ñeent-fukk juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(100000, lang="wo"), "benn téeméer junni")
        self.assertEqual(
            num2words(123456, lang="wo"),
            "benn téeméer ñaar-fukk ñett junni ñeent téeméer juróom-fukk juróom-benn",
        )
        self.assertEqual(num2words(200000, lang="wo"), "ñaar téeméer junni")
        self.assertEqual(num2words(500000, lang="wo"), "juróom téeméer junni")
        self.assertEqual(
            num2words(654321, lang="wo"),
            "juróom-benn téeméer juróom-fukk ñeent junni ñett téeméer ñaar-fukk benn",
        )
        self.assertEqual(
            num2words(999999, lang="wo"),
            "juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="wo"), "benn tamndareet")
        self.assertEqual(num2words(1000001, lang="wo"), "benn tamndareet benn")
        self.assertEqual(
            num2words(1111111, lang="wo"),
            "benn tamndareet benn téeméer fukk benn junni benn téeméer fukk benn",
        )
        self.assertEqual(
            num2words(1234567, lang="wo"),
            "benn tamndareet ñaar téeméer ñett-fukk ñeent junni juróom téeméer juróom-benn-fukk juróom-ñaar",
        )
        self.assertEqual(num2words(2000000, lang="wo"), "ñaar tamndareet")
        self.assertEqual(num2words(5000000, lang="wo"), "juróom tamndareet")
        self.assertEqual(
            num2words(9999999, lang="wo"),
            "juróom-ñeent tamndareet juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(10000000, lang="wo"), "fukk tamndareet")
        self.assertEqual(
            num2words(12345678, lang="wo"),
            "fukk ñaar tamndareet ñett téeméer ñeent-fukk juróom junni juróom-benn téeméer juróom-ñaar-fukk juróom-ñett",
        )
        self.assertEqual(
            num2words(99999999, lang="wo"),
            "juróom-ñeent-fukk juróom-ñeent tamndareet juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(100000000, lang="wo"), "benn téeméer tamndareet")
        self.assertEqual(
            num2words(123456789, lang="wo"),
            "benn téeméer ñaar-fukk ñett tamndareet ñeent téeméer juróom-fukk juróom-benn junni juróom-ñaar téeméer juróom-ñett-fukk juróom-ñeent",
        )
        self.assertEqual(
            num2words(999999999, lang="wo"),
            "juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent tamndareet juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        # No attested Wolof scale word above tamndareet (#147).
        with self.assertRaises(OverflowError):
            num2words(1000000000, lang="wo")
        with self.assertRaises(OverflowError):
            num2words(1234567890, lang="wo")
        with self.assertRaises(OverflowError):
            num2words(9999999999, lang="wo")
        with self.assertRaises(OverflowError):
            num2words(10000000000, lang="wo")
        with self.assertRaises(OverflowError):
            num2words(99999999999, lang="wo")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="wo"), "moins benn")
        self.assertEqual(num2words(-2, lang="wo"), "moins ñaar")
        self.assertEqual(num2words(-5, lang="wo"), "moins juróom")
        self.assertEqual(num2words(-10, lang="wo"), "moins fukk")
        self.assertEqual(num2words(-11, lang="wo"), "moins fukk benn")
        self.assertEqual(num2words(-20, lang="wo"), "moins ñaar-fukk")
        self.assertEqual(num2words(-50, lang="wo"), "moins juróom-fukk")
        self.assertEqual(
            num2words(-99, lang="wo"), "moins juróom-ñeent-fukk juróom-ñeent"
        )
        self.assertEqual(num2words(-100, lang="wo"), "moins benn téeméer")
        self.assertEqual(num2words(-101, lang="wo"), "moins benn téeméer benn")
        self.assertEqual(num2words(-200, lang="wo"), "moins ñaar téeméer")
        self.assertEqual(
            num2words(-999, lang="wo"),
            "moins juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(-1000, lang="wo"), "moins benn junni")
        self.assertEqual(num2words(-1001, lang="wo"), "moins benn junni benn")
        self.assertEqual(num2words(-10000, lang="wo"), "moins fukk junni")
        self.assertEqual(num2words(-100000, lang="wo"), "moins benn téeméer junni")
        self.assertEqual(num2words(-1000000, lang="wo"), "moins benn tamndareet")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="wo"), "tus kos benn")
        self.assertEqual(num2words(0.5, lang="wo"), "tus kos juróom")
        self.assertEqual(num2words(0.9, lang="wo"), "tus kos juróom-ñeent")
        self.assertEqual(num2words(1.1, lang="wo"), "benn kos benn")
        self.assertEqual(num2words(1.5, lang="wo"), "benn kos juróom")
        self.assertEqual(num2words(2.5, lang="wo"), "ñaar kos juróom")
        self.assertEqual(num2words(3.14, lang="wo"), "ñett kos benn ñeent")
        self.assertEqual(num2words(10.5, lang="wo"), "fukk kos juróom")
        self.assertEqual(num2words(11.11, lang="wo"), "fukk benn kos benn benn")
        self.assertEqual(num2words(20.2, lang="wo"), "ñaar-fukk kos ñaar")
        self.assertEqual(
            num2words(99.99, lang="wo"),
            "juróom-ñeent-fukk juróom-ñeent kos juróom-ñeent juróom-ñeent",
        )
        self.assertEqual(num2words(100.01, lang="wo"), "benn téeméer kos tus benn")
        self.assertEqual(num2words(100.5, lang="wo"), "benn téeméer kos juróom")
        self.assertEqual(
            num2words(123.45, lang="wo"),
            "benn téeméer ñaar-fukk ñett kos ñeent juróom",
        )
        self.assertEqual(num2words(1000.5, lang="wo"), "benn junni kos juróom")
        self.assertEqual(
            num2words(1234.56, lang="wo"),
            "benn junni ñaar téeméer ñett-fukk ñeent kos juróom juróom-benn",
        )
        self.assertEqual(num2words(10000.01, lang="wo"), "fukk junni kos tus benn")
        self.assertEqual(num2words(-0.5, lang="wo"), "moins tus kos juróom")
        self.assertEqual(num2words(-1.5, lang="wo"), "moins benn kos juróom")
        self.assertEqual(num2words(-10.5, lang="wo"), "moins fukk kos juróom")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="wo", ordinal=True), "benn-eel")
        self.assertEqual(num2words(2, lang="wo", ordinal=True), "ñaar-eel")
        self.assertEqual(num2words(3, lang="wo", ordinal=True), "ñett-eel")
        self.assertEqual(num2words(4, lang="wo", ordinal=True), "ñeent-eel")
        self.assertEqual(num2words(5, lang="wo", ordinal=True), "juróom-eel")
        self.assertEqual(num2words(6, lang="wo", ordinal=True), "juróom-benn-eel")
        self.assertEqual(num2words(7, lang="wo", ordinal=True), "juróom-ñaar-eel")
        self.assertEqual(num2words(8, lang="wo", ordinal=True), "juróom-ñett-eel")
        self.assertEqual(num2words(9, lang="wo", ordinal=True), "juróom-ñeent-eel")
        self.assertEqual(num2words(10, lang="wo", ordinal=True), "fukk-eel")
        self.assertEqual(num2words(11, lang="wo", ordinal=True), "fukk benn-eel")
        self.assertEqual(num2words(12, lang="wo", ordinal=True), "fukk ñaar-eel")
        self.assertEqual(num2words(13, lang="wo", ordinal=True), "fukk ñett-eel")
        self.assertEqual(num2words(14, lang="wo", ordinal=True), "fukk ñeent-eel")
        self.assertEqual(num2words(15, lang="wo", ordinal=True), "fukk juróom-eel")
        self.assertEqual(num2words(16, lang="wo", ordinal=True), "fukk juróom-benn-eel")
        self.assertEqual(num2words(17, lang="wo", ordinal=True), "fukk juróom-ñaar-eel")
        self.assertEqual(num2words(18, lang="wo", ordinal=True), "fukk juróom-ñett-eel")
        self.assertEqual(
            num2words(19, lang="wo", ordinal=True), "fukk juróom-ñeent-eel"
        )
        self.assertEqual(num2words(20, lang="wo", ordinal=True), "ñaar-fukk-eel")
        self.assertEqual(num2words(21, lang="wo", ordinal=True), "ñaar-fukk benn-eel")
        self.assertEqual(num2words(22, lang="wo", ordinal=True), "ñaar-fukk ñaar-eel")
        self.assertEqual(num2words(25, lang="wo", ordinal=True), "ñaar-fukk juróom-eel")
        self.assertEqual(num2words(30, lang="wo", ordinal=True), "ñett-fukk-eel")
        self.assertEqual(num2words(40, lang="wo", ordinal=True), "ñeent-fukk-eel")
        self.assertEqual(num2words(50, lang="wo", ordinal=True), "juróom-fukk-eel")
        self.assertEqual(num2words(60, lang="wo", ordinal=True), "juróom-benn-fukk-eel")
        self.assertEqual(num2words(70, lang="wo", ordinal=True), "juróom-ñaar-fukk-eel")
        self.assertEqual(num2words(80, lang="wo", ordinal=True), "juróom-ñett-fukk-eel")
        self.assertEqual(
            num2words(90, lang="wo", ordinal=True), "juróom-ñeent-fukk-eel"
        )
        self.assertEqual(num2words(100, lang="wo", ordinal=True), "benn téeméer-eel")
        self.assertEqual(
            num2words(101, lang="wo", ordinal=True), "benn téeméer benn-eel"
        )
        self.assertEqual(num2words(200, lang="wo", ordinal=True), "ñaar téeméer-eel")
        self.assertEqual(num2words(500, lang="wo", ordinal=True), "juróom téeméer-eel")
        self.assertEqual(num2words(1000, lang="wo", ordinal=True), "benn junni-eel")
        self.assertEqual(
            num2words(1001, lang="wo", ordinal=True), "benn junni benn-eel"
        )
        self.assertEqual(num2words(10000, lang="wo", ordinal=True), "fukk junni-eel")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="wo", to="currency", currency="XOF"), "tus dërëm"
        )
        self.assertEqual(
            num2words(0.01, lang="wo", to="currency", currency="XOF"),
            "tus dërëm benn santim",
        )
        self.assertEqual(
            num2words(0.5, lang="wo", to="currency", currency="XOF"),
            "tus dërëm juróom-fukk santim",
        )
        self.assertEqual(
            num2words(1, lang="wo", to="currency", currency="XOF"), "benn dërëm"
        )
        self.assertEqual(
            num2words(1.5, lang="wo", to="currency", currency="XOF"),
            "benn dërëm juróom-fukk santim",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="wo", to="currency", currency="USD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="wo", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="wo", to="year"), "benn junni")
        self.assertEqual(
            num2words(1066, lang="wo", to="year"),
            "benn junni juróom-benn-fukk juróom-benn",
        )
        self.assertEqual(
            num2words(1492, lang="wo", to="year"),
            "benn junni ñeent téeméer juróom-ñeent-fukk ñaar",
        )
        self.assertEqual(
            num2words(1776, lang="wo", to="year"),
            "benn junni juróom-ñaar téeméer juróom-ñaar-fukk juróom-benn",
        )
        self.assertEqual(
            num2words(1800, lang="wo", to="year"), "benn junni juróom-ñett téeméer"
        )
        self.assertEqual(
            num2words(1900, lang="wo", to="year"), "benn junni juróom-ñeent téeméer"
        )
        self.assertEqual(
            num2words(1984, lang="wo", to="year"),
            "benn junni juróom-ñeent téeméer juróom-ñett-fukk ñeent",
        )
        self.assertEqual(
            num2words(1999, lang="wo", to="year"),
            "benn junni juróom-ñeent téeméer juróom-ñeent-fukk juróom-ñeent",
        )
        self.assertEqual(num2words(2000, lang="wo", to="year"), "ñaar junni")
        self.assertEqual(num2words(2001, lang="wo", to="year"), "ñaar junni benn")
        self.assertEqual(num2words(2010, lang="wo", to="year"), "ñaar junni fukk")
        self.assertEqual(num2words(2020, lang="wo", to="year"), "ñaar junni ñaar-fukk")
        self.assertEqual(
            num2words(2024, lang="wo", to="year"), "ñaar junni ñaar-fukk ñeent"
        )
        self.assertEqual(
            num2words(2100, lang="wo", to="year"), "ñaar junni benn téeméer"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="wo"), "tus")
        self.assertEqual(num2words("1", lang="wo"), "benn")
        self.assertEqual(num2words("10", lang="wo"), "fukk")
        self.assertEqual(num2words("100", lang="wo"), "benn téeméer")
        self.assertEqual(num2words("1000", lang="wo"), "benn junni")
        self.assertEqual(num2words("10000", lang="wo"), "fukk junni")
        self.assertEqual(num2words("100000", lang="wo"), "benn téeméer junni")
        self.assertEqual(num2words("1000000", lang="wo"), "benn tamndareet")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="wo"), "tus")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="wo"), num2words("100", lang="wo"))
        self.assertEqual(num2words(1000, lang="wo"), num2words("1000", lang="wo"))
