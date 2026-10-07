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


class Num2WordsHTTest(TestCase):
    """Comprehensive test cases for Haitian Creole language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ht"), "zewo")
        self.assertEqual(num2words(1, lang="ht"), "en")
        self.assertEqual(num2words(2, lang="ht"), "de")
        self.assertEqual(num2words(3, lang="ht"), "twa")
        self.assertEqual(num2words(4, lang="ht"), "kat")
        self.assertEqual(num2words(5, lang="ht"), "senk")
        self.assertEqual(num2words(6, lang="ht"), "sis")
        self.assertEqual(num2words(7, lang="ht"), "sèt")
        self.assertEqual(num2words(8, lang="ht"), "uit")
        self.assertEqual(num2words(9, lang="ht"), "nèf")
        self.assertEqual(num2words(10, lang="ht"), "dis")
        self.assertEqual(num2words(11, lang="ht"), "dis en")
        self.assertEqual(num2words(12, lang="ht"), "dis de")
        self.assertEqual(num2words(13, lang="ht"), "dis twa")
        self.assertEqual(num2words(14, lang="ht"), "dis kat")
        self.assertEqual(num2words(15, lang="ht"), "dis senk")
        self.assertEqual(num2words(16, lang="ht"), "dis sis")
        self.assertEqual(num2words(17, lang="ht"), "dis sèt")
        self.assertEqual(num2words(18, lang="ht"), "dis uit")
        self.assertEqual(num2words(19, lang="ht"), "dis nèf")
        self.assertEqual(num2words(20, lang="ht"), "ven")
        self.assertEqual(num2words(21, lang="ht"), "ven en")
        self.assertEqual(num2words(22, lang="ht"), "ven de")
        self.assertEqual(num2words(23, lang="ht"), "ven twa")
        self.assertEqual(num2words(24, lang="ht"), "ven kat")
        self.assertEqual(num2words(25, lang="ht"), "ven senk")
        self.assertEqual(num2words(26, lang="ht"), "ven sis")
        self.assertEqual(num2words(27, lang="ht"), "ven sèt")
        self.assertEqual(num2words(28, lang="ht"), "ven uit")
        self.assertEqual(num2words(29, lang="ht"), "ven nèf")
        self.assertEqual(num2words(30, lang="ht"), "trant")
        self.assertEqual(num2words(31, lang="ht"), "trant en")
        self.assertEqual(num2words(35, lang="ht"), "trant senk")
        self.assertEqual(num2words(40, lang="ht"), "karant")
        self.assertEqual(num2words(45, lang="ht"), "karant senk")
        self.assertEqual(num2words(50, lang="ht"), "senkant")
        self.assertEqual(num2words(55, lang="ht"), "senkant senk")
        self.assertEqual(num2words(60, lang="ht"), "swasant")
        self.assertEqual(num2words(65, lang="ht"), "swasant senk")
        self.assertEqual(num2words(70, lang="ht"), "swasantdis")
        self.assertEqual(num2words(75, lang="ht"), "swasantdis senk")
        self.assertEqual(num2words(80, lang="ht"), "katreven")
        self.assertEqual(num2words(85, lang="ht"), "katreven senk")
        self.assertEqual(num2words(90, lang="ht"), "katrevendis")
        self.assertEqual(num2words(95, lang="ht"), "katrevendis senk")
        self.assertEqual(num2words(99, lang="ht"), "katrevendis nèf")
        self.assertEqual(num2words(100, lang="ht"), "en san")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ht"), "en san en")
        self.assertEqual(num2words(110, lang="ht"), "en san dis")
        self.assertEqual(num2words(111, lang="ht"), "en san dis en")
        self.assertEqual(num2words(120, lang="ht"), "en san ven")
        self.assertEqual(num2words(125, lang="ht"), "en san ven senk")
        self.assertEqual(num2words(150, lang="ht"), "en san senkant")
        self.assertEqual(num2words(175, lang="ht"), "en san swasantdis senk")
        self.assertEqual(num2words(199, lang="ht"), "en san katrevendis nèf")
        self.assertEqual(num2words(200, lang="ht"), "de san")
        self.assertEqual(num2words(201, lang="ht"), "de san en")
        self.assertEqual(num2words(210, lang="ht"), "de san dis")
        self.assertEqual(num2words(220, lang="ht"), "de san ven")
        self.assertEqual(num2words(250, lang="ht"), "de san senkant")
        self.assertEqual(num2words(299, lang="ht"), "de san katrevendis nèf")
        self.assertEqual(num2words(300, lang="ht"), "twa san")
        self.assertEqual(num2words(333, lang="ht"), "twa san trant twa")
        self.assertEqual(num2words(400, lang="ht"), "kat san")
        self.assertEqual(num2words(444, lang="ht"), "kat san karant kat")
        self.assertEqual(num2words(500, lang="ht"), "senk san")
        self.assertEqual(num2words(555, lang="ht"), "senk san senkant senk")
        self.assertEqual(num2words(600, lang="ht"), "sis san")
        self.assertEqual(num2words(666, lang="ht"), "sis san swasant sis")
        self.assertEqual(num2words(700, lang="ht"), "sèt san")
        self.assertEqual(num2words(777, lang="ht"), "sèt san swasantdis sèt")
        self.assertEqual(num2words(800, lang="ht"), "uit san")
        self.assertEqual(num2words(888, lang="ht"), "uit san katreven uit")
        self.assertEqual(num2words(900, lang="ht"), "nèf san")
        self.assertEqual(num2words(999, lang="ht"), "nèf san katrevendis nèf")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ht"), "en mil")
        self.assertEqual(num2words(1001, lang="ht"), "en mil en")
        self.assertEqual(num2words(1010, lang="ht"), "en mil dis")
        self.assertEqual(num2words(1100, lang="ht"), "en mil en san")
        self.assertEqual(num2words(1111, lang="ht"), "en mil en san dis en")
        self.assertEqual(num2words(1234, lang="ht"), "en mil de san trant kat")
        self.assertEqual(num2words(1500, lang="ht"), "en mil senk san")
        self.assertEqual(num2words(1999, lang="ht"), "en mil nèf san katrevendis nèf")
        self.assertEqual(num2words(2000, lang="ht"), "de mil")
        self.assertEqual(num2words(2001, lang="ht"), "de mil en")
        self.assertEqual(num2words(2020, lang="ht"), "de mil ven")
        self.assertEqual(num2words(2222, lang="ht"), "de mil de san ven de")
        self.assertEqual(num2words(3000, lang="ht"), "twa mil")
        self.assertEqual(num2words(3333, lang="ht"), "twa mil twa san trant twa")
        self.assertEqual(num2words(4000, lang="ht"), "kat mil")
        self.assertEqual(num2words(4444, lang="ht"), "kat mil kat san karant kat")
        self.assertEqual(num2words(5000, lang="ht"), "senk mil")
        self.assertEqual(num2words(5555, lang="ht"), "senk mil senk san senkant senk")
        self.assertEqual(num2words(6000, lang="ht"), "sis mil")
        self.assertEqual(num2words(6666, lang="ht"), "sis mil sis san swasant sis")
        self.assertEqual(num2words(7000, lang="ht"), "sèt mil")
        self.assertEqual(num2words(7777, lang="ht"), "sèt mil sèt san swasantdis sèt")
        self.assertEqual(num2words(8000, lang="ht"), "uit mil")
        self.assertEqual(num2words(8888, lang="ht"), "uit mil uit san katreven uit")
        self.assertEqual(num2words(9000, lang="ht"), "nèf mil")
        self.assertEqual(num2words(9999, lang="ht"), "nèf mil nèf san katrevendis nèf")
        self.assertEqual(num2words(10000, lang="ht"), "dis mil")
        self.assertEqual(num2words(10001, lang="ht"), "dis mil en")
        self.assertEqual(num2words(11111, lang="ht"), "dis en mil en san dis en")
        self.assertEqual(num2words(12345, lang="ht"), "dis de mil twa san karant senk")
        self.assertEqual(num2words(20000, lang="ht"), "ven mil")
        self.assertEqual(num2words(50000, lang="ht"), "senkant mil")
        self.assertEqual(
            num2words(99999, lang="ht"), "katrevendis nèf mil nèf san katrevendis nèf"
        )
        self.assertEqual(num2words(100000, lang="ht"), "en san mil")
        self.assertEqual(
            num2words(123456, lang="ht"), "en san ven twa mil kat san senkant sis"
        )
        self.assertEqual(num2words(200000, lang="ht"), "de san mil")
        self.assertEqual(num2words(500000, lang="ht"), "senk san mil")
        self.assertEqual(
            num2words(654321, lang="ht"), "sis san senkant kat mil twa san ven en"
        )
        self.assertEqual(
            num2words(999999, lang="ht"),
            "nèf san katrevendis nèf mil nèf san katrevendis nèf",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ht"), "en milyon")
        self.assertEqual(num2words(1000001, lang="ht"), "en milyon en")
        self.assertEqual(
            num2words(1111111, lang="ht"), "en milyon en san dis en mil en san dis en"
        )
        self.assertEqual(
            num2words(1234567, lang="ht"),
            "en milyon de san trant kat mil senk san swasant sèt",
        )
        self.assertEqual(num2words(2000000, lang="ht"), "de milyon")
        self.assertEqual(num2words(5000000, lang="ht"), "senk milyon")
        self.assertEqual(
            num2words(9999999, lang="ht"),
            "nèf milyon nèf san katrevendis nèf mil nèf san katrevendis nèf",
        )
        self.assertEqual(num2words(10000000, lang="ht"), "dis milyon")
        self.assertEqual(
            num2words(12345678, lang="ht"),
            "dis de milyon twa san karant senk mil sis san swasantdis uit",
        )
        self.assertEqual(
            num2words(99999999, lang="ht"),
            "katrevendis nèf milyon nèf san katrevendis nèf mil nèf san katrevendis nèf",
        )
        self.assertEqual(num2words(100000000, lang="ht"), "en san milyon")
        self.assertEqual(
            num2words(123456789, lang="ht"),
            "en san ven twa milyon kat san senkant sis mil sèt san katreven nèf",
        )
        self.assertEqual(
            num2words(999999999, lang="ht"),
            "nèf san katrevendis nèf milyon nèf san katrevendis nèf mil nèf san katrevendis nèf",
        )
        self.assertEqual(num2words(1000000000, lang="ht"), "en milya")
        self.assertEqual(num2words(1234567890, lang="ht"), "en milya de san trant kat milyon senk san swasant sèt mil uit san katrevendis")
        self.assertEqual(num2words(9999999999, lang="ht"), "nèf milya nèf san katrevendis nèf milyon nèf san katrevendis nèf mil nèf san katrevendis nèf")
        self.assertEqual(num2words(10000000000, lang="ht"), "dis milya")
        self.assertEqual(num2words(99999999999, lang="ht"), "katrevendis nèf milya nèf san katrevendis nèf milyon nèf san katrevendis nèf mil nèf san katrevendis nèf")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ht"), "mwens en")
        self.assertEqual(num2words(-2, lang="ht"), "mwens de")
        self.assertEqual(num2words(-5, lang="ht"), "mwens senk")
        self.assertEqual(num2words(-10, lang="ht"), "mwens dis")
        self.assertEqual(num2words(-11, lang="ht"), "mwens dis en")
        self.assertEqual(num2words(-20, lang="ht"), "mwens ven")
        self.assertEqual(num2words(-50, lang="ht"), "mwens senkant")
        self.assertEqual(num2words(-99, lang="ht"), "mwens katrevendis nèf")
        self.assertEqual(num2words(-100, lang="ht"), "mwens en san")
        self.assertEqual(num2words(-101, lang="ht"), "mwens en san en")
        self.assertEqual(num2words(-200, lang="ht"), "mwens de san")
        self.assertEqual(num2words(-999, lang="ht"), "mwens nèf san katrevendis nèf")
        self.assertEqual(num2words(-1000, lang="ht"), "mwens en mil")
        self.assertEqual(num2words(-1001, lang="ht"), "mwens en mil en")
        self.assertEqual(num2words(-10000, lang="ht"), "mwens dis mil")
        self.assertEqual(num2words(-100000, lang="ht"), "mwens en san mil")
        self.assertEqual(num2words(-1000000, lang="ht"), "mwens en milyon")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ht"), "zewo vigil en")
        self.assertEqual(num2words(0.5, lang="ht"), "zewo vigil senk")
        self.assertEqual(num2words(0.9, lang="ht"), "zewo vigil nèf")
        self.assertEqual(num2words(1.1, lang="ht"), "en vigil en")
        self.assertEqual(num2words(1.5, lang="ht"), "en vigil senk")
        self.assertEqual(num2words(2.5, lang="ht"), "de vigil senk")
        self.assertEqual(num2words(3.14, lang="ht"), "twa vigil en kat")
        self.assertEqual(num2words(10.5, lang="ht"), "dis vigil senk")
        self.assertEqual(num2words(11.11, lang="ht"), "dis en vigil en en")
        self.assertEqual(num2words(20.2, lang="ht"), "ven vigil de")
        self.assertEqual(num2words(99.99, lang="ht"), "katrevendis nèf vigil nèf nèf")
        self.assertEqual(num2words(100.01, lang="ht"), "en san vigil zewo en")
        self.assertEqual(num2words(100.5, lang="ht"), "en san vigil senk")
        self.assertEqual(num2words(123.45, lang="ht"), "en san ven twa vigil kat senk")
        self.assertEqual(num2words(1000.5, lang="ht"), "en mil vigil senk")
        self.assertEqual(
            num2words(1234.56, lang="ht"), "en mil de san trant kat vigil senk sis"
        )
        self.assertEqual(num2words(10000.01, lang="ht"), "dis mil vigil zewo en")
        self.assertEqual(num2words(-0.5, lang="ht"), "mwens zewo vigil senk")
        self.assertEqual(num2words(-1.5, lang="ht"), "mwens en vigil senk")
        self.assertEqual(num2words(-10.5, lang="ht"), "mwens dis vigil senk")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ht", ordinal=True), "en-yèm")
        self.assertEqual(num2words(2, lang="ht", ordinal=True), "de-yèm")
        self.assertEqual(num2words(3, lang="ht", ordinal=True), "twa-yèm")
        self.assertEqual(num2words(4, lang="ht", ordinal=True), "kat-yèm")
        self.assertEqual(num2words(5, lang="ht", ordinal=True), "senk-yèm")
        self.assertEqual(num2words(6, lang="ht", ordinal=True), "sis-yèm")
        self.assertEqual(num2words(7, lang="ht", ordinal=True), "sèt-yèm")
        self.assertEqual(num2words(8, lang="ht", ordinal=True), "uit-yèm")
        self.assertEqual(num2words(9, lang="ht", ordinal=True), "nèf-yèm")
        self.assertEqual(num2words(10, lang="ht", ordinal=True), "dis-yèm")
        self.assertEqual(num2words(11, lang="ht", ordinal=True), "dis en-yèm")
        self.assertEqual(num2words(12, lang="ht", ordinal=True), "dis de-yèm")
        self.assertEqual(num2words(13, lang="ht", ordinal=True), "dis twa-yèm")
        self.assertEqual(num2words(14, lang="ht", ordinal=True), "dis kat-yèm")
        self.assertEqual(num2words(15, lang="ht", ordinal=True), "dis senk-yèm")
        self.assertEqual(num2words(16, lang="ht", ordinal=True), "dis sis-yèm")
        self.assertEqual(num2words(17, lang="ht", ordinal=True), "dis sèt-yèm")
        self.assertEqual(num2words(18, lang="ht", ordinal=True), "dis uit-yèm")
        self.assertEqual(num2words(19, lang="ht", ordinal=True), "dis nèf-yèm")
        self.assertEqual(num2words(20, lang="ht", ordinal=True), "ven-yèm")
        self.assertEqual(num2words(21, lang="ht", ordinal=True), "ven en-yèm")
        self.assertEqual(num2words(22, lang="ht", ordinal=True), "ven de-yèm")
        self.assertEqual(num2words(25, lang="ht", ordinal=True), "ven senk-yèm")
        self.assertEqual(num2words(30, lang="ht", ordinal=True), "trant-yèm")
        self.assertEqual(num2words(40, lang="ht", ordinal=True), "karant-yèm")
        self.assertEqual(num2words(50, lang="ht", ordinal=True), "senkant-yèm")
        self.assertEqual(num2words(60, lang="ht", ordinal=True), "swasant-yèm")
        self.assertEqual(num2words(70, lang="ht", ordinal=True), "swasantdis-yèm")
        self.assertEqual(num2words(80, lang="ht", ordinal=True), "katreven-yèm")
        self.assertEqual(num2words(90, lang="ht", ordinal=True), "katrevendis-yèm")
        self.assertEqual(num2words(100, lang="ht", ordinal=True), "en san-yèm")
        self.assertEqual(num2words(101, lang="ht", ordinal=True), "en san en-yèm")
        self.assertEqual(num2words(200, lang="ht", ordinal=True), "de san-yèm")
        self.assertEqual(num2words(500, lang="ht", ordinal=True), "senk san-yèm")
        self.assertEqual(num2words(1000, lang="ht", ordinal=True), "en mil-yèm")
        self.assertEqual(num2words(1001, lang="ht", ordinal=True), "en mil en-yèm")
        self.assertEqual(num2words(10000, lang="ht", ordinal=True), "dis mil-yèm")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ht", to="currency", currency="HTG"), "zewo goud"
        )
        self.assertEqual(
            num2words(0.01, lang="ht", to="currency", currency="HTG"),
            "zewo goud en santim",
        )
        self.assertEqual(
            num2words(0.5, lang="ht", to="currency", currency="HTG"),
            "zewo goud senkant santim",
        )
        self.assertEqual(
            num2words(1, lang="ht", to="currency", currency="HTG"), "en goud"
        )
        self.assertEqual(
            num2words(1.5, lang="ht", to="currency", currency="HTG"),
            "en goud senkant santim",
        )
        self.assertEqual(
            num2words(0, lang="ht", to="currency", currency="USD"), "zewo dola"
        )
        self.assertEqual(
            num2words(0.01, lang="ht", to="currency", currency="USD"),
            "zewo dola en santim",
        )
        self.assertEqual(
            num2words(0.5, lang="ht", to="currency", currency="USD"),
            "zewo dola senkant santim",
        )
        self.assertEqual(
            num2words(1, lang="ht", to="currency", currency="USD"), "en dola"
        )
        self.assertEqual(
            num2words(1.5, lang="ht", to="currency", currency="USD"),
            "en dola senkant santim",
        )
        self.assertEqual(
            num2words(0, lang="ht", to="currency", currency="EUR"), "zewo ewo"
        )
        self.assertEqual(
            num2words(0.01, lang="ht", to="currency", currency="EUR"),
            "zewo ewo en santim",
        )
        self.assertEqual(
            num2words(0.5, lang="ht", to="currency", currency="EUR"),
            "zewo ewo senkant santim",
        )
        self.assertEqual(
            num2words(1, lang="ht", to="currency", currency="EUR"), "en ewo"
        )
        self.assertEqual(
            num2words(1.5, lang="ht", to="currency", currency="EUR"),
            "en ewo senkant santim",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ht", to="year"), "en mil")
        self.assertEqual(num2words(1066, lang="ht", to="year"), "en mil swasant sis")
        self.assertEqual(
            num2words(1492, lang="ht", to="year"), "en mil kat san katrevendis de"
        )
        self.assertEqual(
            num2words(1776, lang="ht", to="year"), "en mil sèt san swasantdis sis"
        )
        self.assertEqual(num2words(1800, lang="ht", to="year"), "en mil uit san")
        self.assertEqual(num2words(1900, lang="ht", to="year"), "en mil nèf san")
        self.assertEqual(
            num2words(1984, lang="ht", to="year"), "en mil nèf san katreven kat"
        )
        self.assertEqual(
            num2words(1999, lang="ht", to="year"), "en mil nèf san katrevendis nèf"
        )
        self.assertEqual(num2words(2000, lang="ht", to="year"), "de mil")
        self.assertEqual(num2words(2001, lang="ht", to="year"), "de mil en")
        self.assertEqual(num2words(2010, lang="ht", to="year"), "de mil dis")
        self.assertEqual(num2words(2020, lang="ht", to="year"), "de mil ven")
        self.assertEqual(num2words(2024, lang="ht", to="year"), "de mil ven kat")
        self.assertEqual(num2words(2100, lang="ht", to="year"), "de mil en san")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ht"), "zewo")
        self.assertEqual(num2words("1", lang="ht"), "en")
        self.assertEqual(num2words("10", lang="ht"), "dis")
        self.assertEqual(num2words("100", lang="ht"), "en san")
        self.assertEqual(num2words("1000", lang="ht"), "en mil")
        self.assertEqual(num2words("10000", lang="ht"), "dis mil")
        self.assertEqual(num2words("100000", lang="ht"), "en san mil")
        self.assertEqual(num2words("1000000", lang="ht"), "en milyon")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ht"), "zewo")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ht"), num2words("100", lang="ht"))
        self.assertEqual(num2words(1000, lang="ht"), num2words("1000", lang="ht"))

