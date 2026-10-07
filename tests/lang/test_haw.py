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


class Num2WordsHAWTest(TestCase):
    """Comprehensive test cases for Hawaiian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="haw"), "'ole")
        self.assertEqual(num2words(1, lang="haw"), "'ekahi")
        self.assertEqual(num2words(2, lang="haw"), "'elua")
        self.assertEqual(num2words(3, lang="haw"), "'ekolu")
        self.assertEqual(num2words(4, lang="haw"), "'ehā")
        self.assertEqual(num2words(5, lang="haw"), "'elima")
        self.assertEqual(num2words(6, lang="haw"), "'eono")
        self.assertEqual(num2words(7, lang="haw"), "'ehiku")
        self.assertEqual(num2words(8, lang="haw"), "'ewalu")
        self.assertEqual(num2words(9, lang="haw"), "'eiwa")
        self.assertEqual(num2words(10, lang="haw"), "'umi")
        self.assertEqual(num2words(11, lang="haw"), "'umi 'ekahi")
        self.assertEqual(num2words(12, lang="haw"), "'umi 'elua")
        self.assertEqual(num2words(13, lang="haw"), "'umi 'ekolu")
        self.assertEqual(num2words(14, lang="haw"), "'umi 'ehā")
        self.assertEqual(num2words(15, lang="haw"), "'umi 'elima")
        self.assertEqual(num2words(16, lang="haw"), "'umi 'eono")
        self.assertEqual(num2words(17, lang="haw"), "'umi 'ehiku")
        self.assertEqual(num2words(18, lang="haw"), "'umi 'ewalu")
        self.assertEqual(num2words(19, lang="haw"), "'umi 'eiwa")
        self.assertEqual(num2words(20, lang="haw"), "iwakālua")
        self.assertEqual(num2words(21, lang="haw"), "iwakālua 'ekahi")
        self.assertEqual(num2words(22, lang="haw"), "iwakālua 'elua")
        self.assertEqual(num2words(23, lang="haw"), "iwakālua 'ekolu")
        self.assertEqual(num2words(24, lang="haw"), "iwakālua 'ehā")
        self.assertEqual(num2words(25, lang="haw"), "iwakālua 'elima")
        self.assertEqual(num2words(26, lang="haw"), "iwakālua 'eono")
        self.assertEqual(num2words(27, lang="haw"), "iwakālua 'ehiku")
        self.assertEqual(num2words(28, lang="haw"), "iwakālua 'ewalu")
        self.assertEqual(num2words(29, lang="haw"), "iwakālua 'eiwa")
        self.assertEqual(num2words(30, lang="haw"), "kanakolu")
        self.assertEqual(num2words(31, lang="haw"), "kanakolu 'ekahi")
        self.assertEqual(num2words(35, lang="haw"), "kanakolu 'elima")
        self.assertEqual(num2words(40, lang="haw"), "kanahā")
        self.assertEqual(num2words(45, lang="haw"), "kanahā 'elima")
        self.assertEqual(num2words(50, lang="haw"), "kanalima")
        self.assertEqual(num2words(55, lang="haw"), "kanalima 'elima")
        self.assertEqual(num2words(60, lang="haw"), "kanaono")
        self.assertEqual(num2words(65, lang="haw"), "kanaono 'elima")
        self.assertEqual(num2words(70, lang="haw"), "kanahiku")
        self.assertEqual(num2words(75, lang="haw"), "kanahiku 'elima")
        self.assertEqual(num2words(80, lang="haw"), "kanawalu")
        self.assertEqual(num2words(85, lang="haw"), "kanawalu 'elima")
        self.assertEqual(num2words(90, lang="haw"), "kanaiwa")
        self.assertEqual(num2words(95, lang="haw"), "kanaiwa 'elima")
        self.assertEqual(num2words(99, lang="haw"), "kanaiwa 'eiwa")
        self.assertEqual(num2words(100, lang="haw"), "'ekahi haneli")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="haw"), "'ekahi haneli 'ekahi")
        self.assertEqual(num2words(110, lang="haw"), "'ekahi haneli 'umi")
        self.assertEqual(num2words(111, lang="haw"), "'ekahi haneli 'umi 'ekahi")
        self.assertEqual(num2words(120, lang="haw"), "'ekahi haneli iwakālua")
        self.assertEqual(num2words(125, lang="haw"), "'ekahi haneli iwakālua 'elima")
        self.assertEqual(num2words(150, lang="haw"), "'ekahi haneli kanalima")
        self.assertEqual(num2words(175, lang="haw"), "'ekahi haneli kanahiku 'elima")
        self.assertEqual(num2words(199, lang="haw"), "'ekahi haneli kanaiwa 'eiwa")
        self.assertEqual(num2words(200, lang="haw"), "'elua haneli")
        self.assertEqual(num2words(201, lang="haw"), "'elua haneli 'ekahi")
        self.assertEqual(num2words(210, lang="haw"), "'elua haneli 'umi")
        self.assertEqual(num2words(220, lang="haw"), "'elua haneli iwakālua")
        self.assertEqual(num2words(250, lang="haw"), "'elua haneli kanalima")
        self.assertEqual(num2words(299, lang="haw"), "'elua haneli kanaiwa 'eiwa")
        self.assertEqual(num2words(300, lang="haw"), "'ekolu haneli")
        self.assertEqual(num2words(333, lang="haw"), "'ekolu haneli kanakolu 'ekolu")
        self.assertEqual(num2words(400, lang="haw"), "'ehā haneli")
        self.assertEqual(num2words(444, lang="haw"), "'ehā haneli kanahā 'ehā")
        self.assertEqual(num2words(500, lang="haw"), "'elima haneli")
        self.assertEqual(num2words(555, lang="haw"), "'elima haneli kanalima 'elima")
        self.assertEqual(num2words(600, lang="haw"), "'eono haneli")
        self.assertEqual(num2words(666, lang="haw"), "'eono haneli kanaono 'eono")
        self.assertEqual(num2words(700, lang="haw"), "'ehiku haneli")
        self.assertEqual(num2words(777, lang="haw"), "'ehiku haneli kanahiku 'ehiku")
        self.assertEqual(num2words(800, lang="haw"), "'ewalu haneli")
        self.assertEqual(num2words(888, lang="haw"), "'ewalu haneli kanawalu 'ewalu")
        self.assertEqual(num2words(900, lang="haw"), "'eiwa haneli")
        self.assertEqual(num2words(999, lang="haw"), "'eiwa haneli kanaiwa 'eiwa")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="haw"), "'ekahi kaukani")
        self.assertEqual(num2words(1001, lang="haw"), "'ekahi kaukani 'ekahi")
        self.assertEqual(num2words(1010, lang="haw"), "'ekahi kaukani 'umi")
        self.assertEqual(num2words(1100, lang="haw"), "'ekahi kaukani 'ekahi haneli")
        self.assertEqual(
            num2words(1111, lang="haw"), "'ekahi kaukani 'ekahi haneli 'umi 'ekahi"
        )
        self.assertEqual(
            num2words(1234, lang="haw"), "'ekahi kaukani 'elua haneli kanakolu 'ehā"
        )
        self.assertEqual(num2words(1500, lang="haw"), "'ekahi kaukani 'elima haneli")
        self.assertEqual(
            num2words(1999, lang="haw"), "'ekahi kaukani 'eiwa haneli kanaiwa 'eiwa"
        )
        self.assertEqual(num2words(2000, lang="haw"), "'elua kaukani")
        self.assertEqual(num2words(2001, lang="haw"), "'elua kaukani 'ekahi")
        self.assertEqual(num2words(2020, lang="haw"), "'elua kaukani iwakālua")
        self.assertEqual(
            num2words(2222, lang="haw"), "'elua kaukani 'elua haneli iwakālua 'elua"
        )
        self.assertEqual(num2words(3000, lang="haw"), "'ekolu kaukani")
        self.assertEqual(
            num2words(3333, lang="haw"), "'ekolu kaukani 'ekolu haneli kanakolu 'ekolu"
        )
        self.assertEqual(num2words(4000, lang="haw"), "'ehā kaukani")
        self.assertEqual(
            num2words(4444, lang="haw"), "'ehā kaukani 'ehā haneli kanahā 'ehā"
        )
        self.assertEqual(num2words(5000, lang="haw"), "'elima kaukani")
        self.assertEqual(
            num2words(5555, lang="haw"), "'elima kaukani 'elima haneli kanalima 'elima"
        )
        self.assertEqual(num2words(6000, lang="haw"), "'eono kaukani")
        self.assertEqual(
            num2words(6666, lang="haw"), "'eono kaukani 'eono haneli kanaono 'eono"
        )
        self.assertEqual(num2words(7000, lang="haw"), "'ehiku kaukani")
        self.assertEqual(
            num2words(7777, lang="haw"), "'ehiku kaukani 'ehiku haneli kanahiku 'ehiku"
        )
        self.assertEqual(num2words(8000, lang="haw"), "'ewalu kaukani")
        self.assertEqual(
            num2words(8888, lang="haw"), "'ewalu kaukani 'ewalu haneli kanawalu 'ewalu"
        )
        self.assertEqual(num2words(9000, lang="haw"), "'eiwa kaukani")
        self.assertEqual(
            num2words(9999, lang="haw"), "'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa"
        )
        self.assertEqual(num2words(10000, lang="haw"), "'umi kaukani")
        self.assertEqual(num2words(10001, lang="haw"), "'umi kaukani 'ekahi")
        self.assertEqual(
            num2words(11111, lang="haw"),
            "'umi 'ekahi kaukani 'ekahi haneli 'umi 'ekahi",
        )
        self.assertEqual(
            num2words(12345, lang="haw"),
            "'umi 'elua kaukani 'ekolu haneli kanahā 'elima",
        )
        self.assertEqual(num2words(20000, lang="haw"), "iwakālua kaukani")
        self.assertEqual(num2words(50000, lang="haw"), "kanalima kaukani")
        self.assertEqual(
            num2words(99999, lang="haw"),
            "kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa",
        )
        self.assertEqual(num2words(100000, lang="haw"), "'ekahi haneli kaukani")
        self.assertEqual(
            num2words(123456, lang="haw"),
            "'ekahi haneli iwakālua 'ekolu kaukani 'ehā haneli kanalima 'eono",
        )
        self.assertEqual(num2words(200000, lang="haw"), "'elua haneli kaukani")
        self.assertEqual(num2words(500000, lang="haw"), "'elima haneli kaukani")
        self.assertEqual(
            num2words(654321, lang="haw"),
            "'eono haneli kanalima 'ehā kaukani 'ekolu haneli iwakālua 'ekahi",
        )
        self.assertEqual(
            num2words(999999, lang="haw"),
            "'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="haw"), "'ekahi miliona")
        self.assertEqual(num2words(1000001, lang="haw"), "'ekahi miliona 'ekahi")
        self.assertEqual(
            num2words(1111111, lang="haw"),
            "'ekahi miliona 'ekahi haneli 'umi 'ekahi kaukani 'ekahi haneli 'umi 'ekahi",
        )
        self.assertEqual(
            num2words(1234567, lang="haw"),
            "'ekahi miliona 'elua haneli kanakolu 'ehā kaukani 'elima haneli kanaono 'ehiku",
        )
        self.assertEqual(num2words(2000000, lang="haw"), "'elua miliona")
        self.assertEqual(num2words(5000000, lang="haw"), "'elima miliona")
        self.assertEqual(
            num2words(9999999, lang="haw"),
            "'eiwa miliona 'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa",
        )
        self.assertEqual(num2words(10000000, lang="haw"), "'umi miliona")
        self.assertEqual(
            num2words(12345678, lang="haw"),
            "'umi 'elua miliona 'ekolu haneli kanahā 'elima kaukani 'eono haneli kanahiku 'ewalu",
        )
        self.assertEqual(
            num2words(99999999, lang="haw"),
            "kanaiwa 'eiwa miliona 'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa",
        )
        self.assertEqual(num2words(100000000, lang="haw"), "'ekahi haneli miliona")
        self.assertEqual(
            num2words(123456789, lang="haw"),
            "'ekahi haneli iwakālua 'ekolu miliona 'ehā haneli kanalima 'eono kaukani 'ehiku haneli kanawalu 'eiwa",
        )
        self.assertEqual(
            num2words(999999999, lang="haw"),
            "'eiwa haneli kanaiwa 'eiwa miliona 'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa",
        )
        self.assertEqual(num2words(1000000000, lang="haw"), "'ekahi piliona")
        self.assertEqual(num2words(1234567890, lang="haw"), "'ekahi piliona 'elua haneli kanakolu 'ehā miliona 'elima haneli kanaono 'ehiku kaukani 'ewalu haneli kanaiwa")
        self.assertEqual(num2words(9999999999, lang="haw"), "'eiwa piliona 'eiwa haneli kanaiwa 'eiwa miliona 'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa")
        self.assertEqual(num2words(10000000000, lang="haw"), "'umi piliona")
        self.assertEqual(num2words(99999999999, lang="haw"), "kanaiwa 'eiwa piliona 'eiwa haneli kanaiwa 'eiwa miliona 'eiwa haneli kanaiwa 'eiwa kaukani 'eiwa haneli kanaiwa 'eiwa")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="haw"), "'i'o 'ole 'ekahi")
        self.assertEqual(num2words(-2, lang="haw"), "'i'o 'ole 'elua")
        self.assertEqual(num2words(-5, lang="haw"), "'i'o 'ole 'elima")
        self.assertEqual(num2words(-10, lang="haw"), "'i'o 'ole 'umi")
        self.assertEqual(num2words(-11, lang="haw"), "'i'o 'ole 'umi 'ekahi")
        self.assertEqual(num2words(-20, lang="haw"), "'i'o 'ole iwakālua")
        self.assertEqual(num2words(-50, lang="haw"), "'i'o 'ole kanalima")
        self.assertEqual(num2words(-99, lang="haw"), "'i'o 'ole kanaiwa 'eiwa")
        self.assertEqual(num2words(-100, lang="haw"), "'i'o 'ole 'ekahi haneli")
        self.assertEqual(num2words(-101, lang="haw"), "'i'o 'ole 'ekahi haneli 'ekahi")
        self.assertEqual(num2words(-200, lang="haw"), "'i'o 'ole 'elua haneli")
        self.assertEqual(
            num2words(-999, lang="haw"), "'i'o 'ole 'eiwa haneli kanaiwa 'eiwa"
        )
        self.assertEqual(num2words(-1000, lang="haw"), "'i'o 'ole 'ekahi kaukani")
        self.assertEqual(num2words(-1001, lang="haw"), "'i'o 'ole 'ekahi kaukani 'ekahi")
        self.assertEqual(num2words(-10000, lang="haw"), "'i'o 'ole 'umi kaukani")
        self.assertEqual(num2words(-100000, lang="haw"), "'i'o 'ole 'ekahi haneli kaukani")
        self.assertEqual(num2words(-1000000, lang="haw"), "'i'o 'ole 'ekahi miliona")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="haw"), "'ole kiko 'ekahi")
        self.assertEqual(num2words(0.5, lang="haw"), "'ole kiko 'elima")
        self.assertEqual(num2words(0.9, lang="haw"), "'ole kiko 'eiwa")
        self.assertEqual(num2words(1.1, lang="haw"), "'ekahi kiko 'ekahi")
        self.assertEqual(num2words(1.5, lang="haw"), "'ekahi kiko 'elima")
        self.assertEqual(num2words(2.5, lang="haw"), "'elua kiko 'elima")
        self.assertEqual(num2words(3.14, lang="haw"), "'ekolu kiko 'ekahi 'ehā")
        self.assertEqual(num2words(10.5, lang="haw"), "'umi kiko 'elima")
        self.assertEqual(
            num2words(11.11, lang="haw"), "'umi 'ekahi kiko 'ekahi 'ekahi"
        )
        self.assertEqual(num2words(20.2, lang="haw"), "iwakālua kiko 'elua")
        self.assertEqual(
            num2words(99.99, lang="haw"), "kanaiwa 'eiwa kiko 'eiwa 'eiwa"
        )
        self.assertEqual(
            num2words(100.01, lang="haw"), "'ekahi haneli kiko 'ole 'ekahi"
        )
        self.assertEqual(num2words(100.5, lang="haw"), "'ekahi haneli kiko 'elima")
        self.assertEqual(
            num2words(123.45, lang="haw"),
            "'ekahi haneli iwakālua 'ekolu kiko 'ehā 'elima",
        )
        self.assertEqual(num2words(1000.5, lang="haw"), "'ekahi kaukani kiko 'elima")
        self.assertEqual(
            num2words(1234.56, lang="haw"),
            "'ekahi kaukani 'elua haneli kanakolu 'ehā kiko 'elima 'eono",
        )
        self.assertEqual(
            num2words(10000.01, lang="haw"), "'umi kaukani kiko 'ole 'ekahi"
        )
        self.assertEqual(num2words(-0.5, lang="haw"), "'i'o 'ole 'ole kiko 'elima")
        self.assertEqual(num2words(-1.5, lang="haw"), "'i'o 'ole 'ekahi kiko 'elima")
        self.assertEqual(num2words(-10.5, lang="haw"), "'i'o 'ole 'umi kiko 'elima")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="haw", ordinal=True), "ka mua")
        self.assertEqual(num2words(2, lang="haw", ordinal=True), "ka lua")
        self.assertEqual(num2words(3, lang="haw", ordinal=True), "ka 'ekolu")
        self.assertEqual(num2words(4, lang="haw", ordinal=True), "ka 'ehā")
        self.assertEqual(num2words(5, lang="haw", ordinal=True), "ka 'elima")
        self.assertEqual(num2words(6, lang="haw", ordinal=True), "ka 'eono")
        self.assertEqual(num2words(7, lang="haw", ordinal=True), "ka 'ehiku")
        self.assertEqual(num2words(8, lang="haw", ordinal=True), "ka 'ewalu")
        self.assertEqual(num2words(9, lang="haw", ordinal=True), "ka 'eiwa")
        self.assertEqual(num2words(10, lang="haw", ordinal=True), "ka 'umi")
        self.assertEqual(num2words(11, lang="haw", ordinal=True), "ka 'umi 'ekahi")
        self.assertEqual(num2words(12, lang="haw", ordinal=True), "ka 'umi 'elua")
        self.assertEqual(num2words(13, lang="haw", ordinal=True), "ka 'umi 'ekolu")
        self.assertEqual(num2words(14, lang="haw", ordinal=True), "ka 'umi 'ehā")
        self.assertEqual(num2words(15, lang="haw", ordinal=True), "ka 'umi 'elima")
        self.assertEqual(num2words(16, lang="haw", ordinal=True), "ka 'umi 'eono")
        self.assertEqual(num2words(17, lang="haw", ordinal=True), "ka 'umi 'ehiku")
        self.assertEqual(num2words(18, lang="haw", ordinal=True), "ka 'umi 'ewalu")
        self.assertEqual(num2words(19, lang="haw", ordinal=True), "ka 'umi 'eiwa")
        self.assertEqual(num2words(20, lang="haw", ordinal=True), "ka iwakālua")
        self.assertEqual(num2words(21, lang="haw", ordinal=True), "ka iwakālua 'ekahi")
        self.assertEqual(num2words(22, lang="haw", ordinal=True), "ka iwakālua 'elua")
        self.assertEqual(num2words(25, lang="haw", ordinal=True), "ka iwakālua 'elima")
        self.assertEqual(num2words(30, lang="haw", ordinal=True), "ka kanakolu")
        self.assertEqual(num2words(40, lang="haw", ordinal=True), "ka kanahā")
        self.assertEqual(num2words(50, lang="haw", ordinal=True), "ka kanalima")
        self.assertEqual(num2words(60, lang="haw", ordinal=True), "ka kanaono")
        self.assertEqual(num2words(70, lang="haw", ordinal=True), "ka kanahiku")
        self.assertEqual(num2words(80, lang="haw", ordinal=True), "ka kanawalu")
        self.assertEqual(num2words(90, lang="haw", ordinal=True), "ka kanaiwa")
        self.assertEqual(num2words(100, lang="haw", ordinal=True), "ka 'ekahi haneli")
        self.assertEqual(
            num2words(101, lang="haw", ordinal=True), "ka 'ekahi haneli 'ekahi"
        )
        self.assertEqual(num2words(200, lang="haw", ordinal=True), "ka 'elua haneli")
        self.assertEqual(num2words(500, lang="haw", ordinal=True), "ka 'elima haneli")
        self.assertEqual(num2words(1000, lang="haw", ordinal=True), "ka 'ekahi kaukani")
        self.assertEqual(
            num2words(1001, lang="haw", ordinal=True), "ka 'ekahi kaukani 'ekahi"
        )
        self.assertEqual(num2words(10000, lang="haw", ordinal=True), "ka 'umi kaukani")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="haw", to="currency", currency="USD"), "'ole kālā"
        )
        self.assertEqual(
            num2words(0.01, lang="haw", to="currency", currency="USD"),
            "'ole kālā 'ekahi keneka",
        )
        self.assertEqual(
            num2words(0.5, lang="haw", to="currency", currency="USD"),
            "'ole kālā kanalima keneka",
        )
        self.assertEqual(
            num2words(1, lang="haw", to="currency", currency="USD"), "'ekahi kālā"
        )
        self.assertEqual(
            num2words(1.5, lang="haw", to="currency", currency="USD"),
            "'ekahi kālā kanalima keneka",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="haw", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="haw", to="year"), "'ekahi kaukani")
        self.assertEqual(
            num2words(1066, lang="haw", to="year"), "'ekahi kaukani kanaono 'eono"
        )
        self.assertEqual(
            num2words(1492, lang="haw", to="year"),
            "'ekahi kaukani 'ehā haneli kanaiwa 'elua",
        )
        self.assertEqual(
            num2words(1776, lang="haw", to="year"),
            "'ekahi kaukani 'ehiku haneli kanahiku 'eono",
        )
        self.assertEqual(
            num2words(1800, lang="haw", to="year"), "'ekahi kaukani 'ewalu haneli"
        )
        self.assertEqual(
            num2words(1900, lang="haw", to="year"), "'ekahi kaukani 'eiwa haneli"
        )
        self.assertEqual(
            num2words(1984, lang="haw", to="year"),
            "'ekahi kaukani 'eiwa haneli kanawalu 'ehā",
        )
        self.assertEqual(
            num2words(1999, lang="haw", to="year"),
            "'ekahi kaukani 'eiwa haneli kanaiwa 'eiwa",
        )
        self.assertEqual(num2words(2000, lang="haw", to="year"), "'elua kaukani")
        self.assertEqual(num2words(2001, lang="haw", to="year"), "'elua kaukani 'ekahi")
        self.assertEqual(num2words(2010, lang="haw", to="year"), "'elua kaukani 'umi")
        self.assertEqual(
            num2words(2020, lang="haw", to="year"), "'elua kaukani iwakālua"
        )
        self.assertEqual(
            num2words(2024, lang="haw", to="year"), "'elua kaukani iwakālua 'ehā"
        )
        self.assertEqual(
            num2words(2100, lang="haw", to="year"), "'elua kaukani 'ekahi haneli"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="haw"), "'ole")
        self.assertEqual(num2words("1", lang="haw"), "'ekahi")
        self.assertEqual(num2words("10", lang="haw"), "'umi")
        self.assertEqual(num2words("100", lang="haw"), "'ekahi haneli")
        self.assertEqual(num2words("1000", lang="haw"), "'ekahi kaukani")
        self.assertEqual(num2words("10000", lang="haw"), "'umi kaukani")
        self.assertEqual(num2words("100000", lang="haw"), "'ekahi haneli kaukani")
        self.assertEqual(num2words("1000000", lang="haw"), "'ekahi miliona")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="haw"), "'ole")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="haw"), num2words("100", lang="haw"))
        self.assertEqual(num2words(1000, lang="haw"), num2words("1000", lang="haw"))

