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


class Num2WordsJWTest(TestCase):
    """Comprehensive test cases for Javanese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="jw"), "nol")
        self.assertEqual(num2words(1, lang="jw"), "siji")
        self.assertEqual(num2words(2, lang="jw"), "loro")
        self.assertEqual(num2words(3, lang="jw"), "telu")
        self.assertEqual(num2words(4, lang="jw"), "papat")
        self.assertEqual(num2words(5, lang="jw"), "lima")
        self.assertEqual(num2words(6, lang="jw"), "enem")
        self.assertEqual(num2words(7, lang="jw"), "pitu")
        self.assertEqual(num2words(8, lang="jw"), "wolu")
        self.assertEqual(num2words(9, lang="jw"), "sanga")
        self.assertEqual(num2words(10, lang="jw"), "sepuluh")
        self.assertEqual(num2words(11, lang="jw"), "sepuluh siji")
        self.assertEqual(num2words(12, lang="jw"), "sepuluh loro")
        self.assertEqual(num2words(13, lang="jw"), "sepuluh telu")
        self.assertEqual(num2words(14, lang="jw"), "sepuluh papat")
        self.assertEqual(num2words(15, lang="jw"), "sepuluh lima")
        self.assertEqual(num2words(16, lang="jw"), "sepuluh enem")
        self.assertEqual(num2words(17, lang="jw"), "sepuluh pitu")
        self.assertEqual(num2words(18, lang="jw"), "sepuluh wolu")
        self.assertEqual(num2words(19, lang="jw"), "sepuluh sanga")
        self.assertEqual(num2words(20, lang="jw"), "rong puluh")
        self.assertEqual(num2words(21, lang="jw"), "rong puluh siji")
        self.assertEqual(num2words(22, lang="jw"), "rong puluh loro")
        self.assertEqual(num2words(23, lang="jw"), "rong puluh telu")
        self.assertEqual(num2words(24, lang="jw"), "rong puluh papat")
        self.assertEqual(num2words(25, lang="jw"), "rong puluh lima")
        self.assertEqual(num2words(26, lang="jw"), "rong puluh enem")
        self.assertEqual(num2words(27, lang="jw"), "rong puluh pitu")
        self.assertEqual(num2words(28, lang="jw"), "rong puluh wolu")
        self.assertEqual(num2words(29, lang="jw"), "rong puluh sanga")
        self.assertEqual(num2words(30, lang="jw"), "telung puluh")
        self.assertEqual(num2words(31, lang="jw"), "telung puluh siji")
        self.assertEqual(num2words(35, lang="jw"), "telung puluh lima")
        self.assertEqual(num2words(40, lang="jw"), "patang puluh")
        self.assertEqual(num2words(45, lang="jw"), "patang puluh lima")
        self.assertEqual(num2words(50, lang="jw"), "seket")
        self.assertEqual(num2words(55, lang="jw"), "seket lima")
        self.assertEqual(num2words(60, lang="jw"), "sewidak")
        self.assertEqual(num2words(65, lang="jw"), "sewidak lima")
        self.assertEqual(num2words(70, lang="jw"), "pitung puluh")
        self.assertEqual(num2words(75, lang="jw"), "pitung puluh lima")
        self.assertEqual(num2words(80, lang="jw"), "wolung puluh")
        self.assertEqual(num2words(85, lang="jw"), "wolung puluh lima")
        self.assertEqual(num2words(90, lang="jw"), "sanga puluh")
        self.assertEqual(num2words(95, lang="jw"), "sanga puluh lima")
        self.assertEqual(num2words(99, lang="jw"), "sanga puluh sanga")
        self.assertEqual(num2words(100, lang="jw"), "siji atus")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="jw"), "siji atus siji")
        self.assertEqual(num2words(110, lang="jw"), "siji atus sepuluh")
        self.assertEqual(num2words(111, lang="jw"), "siji atus sepuluh siji")
        self.assertEqual(num2words(120, lang="jw"), "siji atus rong puluh")
        self.assertEqual(num2words(125, lang="jw"), "siji atus rong puluh lima")
        self.assertEqual(num2words(150, lang="jw"), "siji atus seket")
        self.assertEqual(num2words(175, lang="jw"), "siji atus pitung puluh lima")
        self.assertEqual(num2words(199, lang="jw"), "siji atus sanga puluh sanga")
        self.assertEqual(num2words(200, lang="jw"), "loro atus")
        self.assertEqual(num2words(201, lang="jw"), "loro atus siji")
        self.assertEqual(num2words(210, lang="jw"), "loro atus sepuluh")
        self.assertEqual(num2words(220, lang="jw"), "loro atus rong puluh")
        self.assertEqual(num2words(250, lang="jw"), "loro atus seket")
        self.assertEqual(num2words(299, lang="jw"), "loro atus sanga puluh sanga")
        self.assertEqual(num2words(300, lang="jw"), "telu atus")
        self.assertEqual(num2words(333, lang="jw"), "telu atus telung puluh telu")
        self.assertEqual(num2words(400, lang="jw"), "papat atus")
        self.assertEqual(num2words(444, lang="jw"), "papat atus patang puluh papat")
        self.assertEqual(num2words(500, lang="jw"), "lima atus")
        self.assertEqual(num2words(555, lang="jw"), "lima atus seket lima")
        self.assertEqual(num2words(600, lang="jw"), "enem atus")
        self.assertEqual(num2words(666, lang="jw"), "enem atus sewidak enem")
        self.assertEqual(num2words(700, lang="jw"), "pitu atus")
        self.assertEqual(num2words(777, lang="jw"), "pitu atus pitung puluh pitu")
        self.assertEqual(num2words(800, lang="jw"), "wolu atus")
        self.assertEqual(num2words(888, lang="jw"), "wolu atus wolung puluh wolu")
        self.assertEqual(num2words(900, lang="jw"), "sanga atus")
        self.assertEqual(num2words(999, lang="jw"), "sanga atus sanga puluh sanga")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="jw"), "siji ewu")
        self.assertEqual(num2words(1001, lang="jw"), "siji ewu siji")
        self.assertEqual(num2words(1010, lang="jw"), "siji ewu sepuluh")
        self.assertEqual(num2words(1100, lang="jw"), "siji ewu siji atus")
        self.assertEqual(num2words(1111, lang="jw"), "siji ewu siji atus sepuluh siji")
        self.assertEqual(
            num2words(1234, lang="jw"), "siji ewu loro atus telung puluh papat"
        )
        self.assertEqual(num2words(1500, lang="jw"), "siji ewu lima atus")
        self.assertEqual(
            num2words(1999, lang="jw"), "siji ewu sanga atus sanga puluh sanga"
        )
        self.assertEqual(num2words(2000, lang="jw"), "loro ewu")
        self.assertEqual(num2words(2001, lang="jw"), "loro ewu siji")
        self.assertEqual(num2words(2020, lang="jw"), "loro ewu rong puluh")
        self.assertEqual(
            num2words(2222, lang="jw"), "loro ewu loro atus rong puluh loro"
        )
        self.assertEqual(num2words(3000, lang="jw"), "telu ewu")
        self.assertEqual(
            num2words(3333, lang="jw"), "telu ewu telu atus telung puluh telu"
        )
        self.assertEqual(num2words(4000, lang="jw"), "papat ewu")
        self.assertEqual(
            num2words(4444, lang="jw"), "papat ewu papat atus patang puluh papat"
        )
        self.assertEqual(num2words(5000, lang="jw"), "lima ewu")
        self.assertEqual(num2words(5555, lang="jw"), "lima ewu lima atus seket lima")
        self.assertEqual(num2words(6000, lang="jw"), "enem ewu")
        self.assertEqual(num2words(6666, lang="jw"), "enem ewu enem atus sewidak enem")
        self.assertEqual(num2words(7000, lang="jw"), "pitu ewu")
        self.assertEqual(
            num2words(7777, lang="jw"), "pitu ewu pitu atus pitung puluh pitu"
        )
        self.assertEqual(num2words(8000, lang="jw"), "wolu ewu")
        self.assertEqual(
            num2words(8888, lang="jw"), "wolu ewu wolu atus wolung puluh wolu"
        )
        self.assertEqual(num2words(9000, lang="jw"), "sanga ewu")
        self.assertEqual(
            num2words(9999, lang="jw"), "sanga ewu sanga atus sanga puluh sanga"
        )
        self.assertEqual(num2words(10000, lang="jw"), "sepuluh ewu")
        self.assertEqual(num2words(10001, lang="jw"), "sepuluh ewu siji")
        self.assertEqual(
            num2words(11111, lang="jw"), "sepuluh siji ewu siji atus sepuluh siji"
        )
        self.assertEqual(
            num2words(12345, lang="jw"), "sepuluh loro ewu telu atus patang puluh lima"
        )
        self.assertEqual(num2words(20000, lang="jw"), "rong puluh ewu")
        self.assertEqual(num2words(50000, lang="jw"), "seket ewu")
        self.assertEqual(
            num2words(99999, lang="jw"),
            "sanga puluh sanga ewu sanga atus sanga puluh sanga",
        )
        self.assertEqual(num2words(100000, lang="jw"), "siji atus ewu")
        self.assertEqual(
            num2words(123456, lang="jw"),
            "siji atus rong puluh telu ewu papat atus seket enem",
        )
        self.assertEqual(num2words(200000, lang="jw"), "loro atus ewu")
        self.assertEqual(num2words(500000, lang="jw"), "lima atus ewu")
        self.assertEqual(
            num2words(654321, lang="jw"),
            "enem atus seket papat ewu telu atus rong puluh siji",
        )
        self.assertEqual(
            num2words(999999, lang="jw"),
            "sanga atus sanga puluh sanga ewu sanga atus sanga puluh sanga",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="jw"), "siji yuta")
        self.assertEqual(num2words(1000001, lang="jw"), "siji yuta siji")
        self.assertEqual(
            num2words(1111111, lang="jw"),
            "siji yuta siji atus sepuluh siji ewu siji atus sepuluh siji",
        )
        self.assertEqual(
            num2words(1234567, lang="jw"),
            "siji yuta loro atus telung puluh papat ewu lima atus sewidak pitu",
        )
        self.assertEqual(num2words(2000000, lang="jw"), "loro yuta")
        self.assertEqual(num2words(5000000, lang="jw"), "lima yuta")
        self.assertEqual(
            num2words(9999999, lang="jw"),
            "sanga yuta sanga atus sanga puluh sanga ewu sanga atus sanga puluh sanga",
        )
        self.assertEqual(num2words(10000000, lang="jw"), "sepuluh yuta")
        self.assertEqual(
            num2words(12345678, lang="jw"),
            "sepuluh loro yuta telu atus patang puluh lima ewu enem atus pitung puluh wolu",
        )
        self.assertEqual(
            num2words(99999999, lang="jw"),
            "sanga puluh sanga yuta sanga atus sanga puluh sanga ewu sanga atus sanga puluh sanga",
        )
        self.assertEqual(num2words(100000000, lang="jw"), "siji atus yuta")
        self.assertEqual(
            num2words(123456789, lang="jw"),
            "siji atus rong puluh telu yuta papat atus seket enem ewu pitu atus wolung puluh sanga",
        )
        self.assertEqual(
            num2words(999999999, lang="jw"),
            "sanga atus sanga puluh sanga yuta sanga atus sanga puluh sanga ewu sanga atus sanga puluh sanga",
        )
        self.assertEqual(num2words(1000000000, lang="jw"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="jw"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="jw"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="jw"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="jw"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="jw"), "minus siji")
        self.assertEqual(num2words(-2, lang="jw"), "minus loro")
        self.assertEqual(num2words(-5, lang="jw"), "minus lima")
        self.assertEqual(num2words(-10, lang="jw"), "minus sepuluh")
        self.assertEqual(num2words(-11, lang="jw"), "minus sepuluh siji")
        self.assertEqual(num2words(-20, lang="jw"), "minus rong puluh")
        self.assertEqual(num2words(-50, lang="jw"), "minus seket")
        self.assertEqual(num2words(-99, lang="jw"), "minus sanga puluh sanga")
        self.assertEqual(num2words(-100, lang="jw"), "minus siji atus")
        self.assertEqual(num2words(-101, lang="jw"), "minus siji atus siji")
        self.assertEqual(num2words(-200, lang="jw"), "minus loro atus")
        self.assertEqual(
            num2words(-999, lang="jw"), "minus sanga atus sanga puluh sanga"
        )
        self.assertEqual(num2words(-1000, lang="jw"), "minus siji ewu")
        self.assertEqual(num2words(-1001, lang="jw"), "minus siji ewu siji")
        self.assertEqual(num2words(-10000, lang="jw"), "minus sepuluh ewu")
        self.assertEqual(num2words(-100000, lang="jw"), "minus siji atus ewu")
        self.assertEqual(num2words(-1000000, lang="jw"), "minus siji yuta")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="jw"), "nol koma siji")
        self.assertEqual(num2words(0.5, lang="jw"), "nol koma lima")
        self.assertEqual(num2words(0.9, lang="jw"), "nol koma sanga")
        self.assertEqual(num2words(1.1, lang="jw"), "siji koma siji")
        self.assertEqual(num2words(1.5, lang="jw"), "siji koma lima")
        self.assertEqual(num2words(2.5, lang="jw"), "loro koma lima")
        self.assertEqual(num2words(3.14, lang="jw"), "telu koma siji papat")
        self.assertEqual(num2words(10.5, lang="jw"), "sepuluh koma lima")
        self.assertEqual(num2words(11.11, lang="jw"), "sepuluh siji koma siji siji")
        self.assertEqual(num2words(20.2, lang="jw"), "rong puluh koma loro")
        self.assertEqual(
            num2words(99.99, lang="jw"), "sanga puluh sanga koma sanga sanga"
        )
        self.assertEqual(num2words(100.01, lang="jw"), "siji atus koma nol siji")
        self.assertEqual(num2words(100.5, lang="jw"), "siji atus koma lima")
        self.assertEqual(
            num2words(123.45, lang="jw"), "siji atus rong puluh telu koma papat lima"
        )
        self.assertEqual(num2words(1000.5, lang="jw"), "siji ewu koma lima")
        self.assertEqual(
            num2words(1234.56, lang="jw"),
            "siji ewu loro atus telung puluh papat koma lima enem",
        )
        self.assertEqual(num2words(10000.01, lang="jw"), "sepuluh ewu koma nol siji")
        self.assertEqual(num2words(-0.5, lang="jw"), "minus nol koma lima")
        self.assertEqual(num2words(-1.5, lang="jw"), "minus siji koma lima")
        self.assertEqual(num2words(-10.5, lang="jw"), "minus sepuluh koma lima")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="jw", ordinal=True), "siji-e")
        self.assertEqual(num2words(2, lang="jw", ordinal=True), "loro-e")
        self.assertEqual(num2words(3, lang="jw", ordinal=True), "telu-e")
        self.assertEqual(num2words(4, lang="jw", ordinal=True), "papat-e")
        self.assertEqual(num2words(5, lang="jw", ordinal=True), "lima-e")
        self.assertEqual(num2words(6, lang="jw", ordinal=True), "enem-e")
        self.assertEqual(num2words(7, lang="jw", ordinal=True), "pitu-e")
        self.assertEqual(num2words(8, lang="jw", ordinal=True), "wolu-e")
        self.assertEqual(num2words(9, lang="jw", ordinal=True), "sanga-e")
        self.assertEqual(num2words(10, lang="jw", ordinal=True), "sepuluh-e")
        self.assertEqual(num2words(11, lang="jw", ordinal=True), "sepuluh siji-e")
        self.assertEqual(num2words(12, lang="jw", ordinal=True), "sepuluh loro-e")
        self.assertEqual(num2words(13, lang="jw", ordinal=True), "sepuluh telu-e")
        self.assertEqual(num2words(14, lang="jw", ordinal=True), "sepuluh papat-e")
        self.assertEqual(num2words(15, lang="jw", ordinal=True), "sepuluh lima-e")
        self.assertEqual(num2words(16, lang="jw", ordinal=True), "sepuluh enem-e")
        self.assertEqual(num2words(17, lang="jw", ordinal=True), "sepuluh pitu-e")
        self.assertEqual(num2words(18, lang="jw", ordinal=True), "sepuluh wolu-e")
        self.assertEqual(num2words(19, lang="jw", ordinal=True), "sepuluh sanga-e")
        self.assertEqual(num2words(20, lang="jw", ordinal=True), "rong puluh-e")
        self.assertEqual(num2words(21, lang="jw", ordinal=True), "rong puluh siji-e")
        self.assertEqual(num2words(22, lang="jw", ordinal=True), "rong puluh loro-e")
        self.assertEqual(num2words(25, lang="jw", ordinal=True), "rong puluh lima-e")
        self.assertEqual(num2words(30, lang="jw", ordinal=True), "telung puluh-e")
        self.assertEqual(num2words(40, lang="jw", ordinal=True), "patang puluh-e")
        self.assertEqual(num2words(50, lang="jw", ordinal=True), "seket-e")
        self.assertEqual(num2words(60, lang="jw", ordinal=True), "sewidak-e")
        self.assertEqual(num2words(70, lang="jw", ordinal=True), "pitung puluh-e")
        self.assertEqual(num2words(80, lang="jw", ordinal=True), "wolung puluh-e")
        self.assertEqual(num2words(90, lang="jw", ordinal=True), "sanga puluh-e")
        self.assertEqual(num2words(100, lang="jw", ordinal=True), "siji atus-e")
        self.assertEqual(num2words(101, lang="jw", ordinal=True), "siji atus siji-e")
        self.assertEqual(num2words(200, lang="jw", ordinal=True), "loro atus-e")
        self.assertEqual(num2words(500, lang="jw", ordinal=True), "lima atus-e")
        self.assertEqual(num2words(1000, lang="jw", ordinal=True), "siji ewu-e")
        self.assertEqual(num2words(1001, lang="jw", ordinal=True), "siji ewu siji-e")
        self.assertEqual(num2words(10000, lang="jw", ordinal=True), "sepuluh ewu-e")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="jw", to="currency", currency="IDR"), "nol rupiah"
        )
        self.assertEqual(
            num2words(0.01, lang="jw", to="currency", currency="IDR"),
            "nol rupiah siji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="jw", to="currency", currency="IDR"),
            "nol rupiah seket sen",
        )
        self.assertEqual(
            num2words(1, lang="jw", to="currency", currency="IDR"), "siji rupiah"
        )
        self.assertEqual(
            num2words(1.5, lang="jw", to="currency", currency="IDR"),
            "siji rupiah seket sen",
        )
        self.assertEqual(
            num2words(0, lang="jw", to="currency", currency="USD"), "nol dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="jw", to="currency", currency="USD"),
            "nol dolar siji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="jw", to="currency", currency="USD"),
            "nol dolar seket sen",
        )
        self.assertEqual(
            num2words(1, lang="jw", to="currency", currency="USD"), "siji dolar"
        )
        self.assertEqual(
            num2words(1.5, lang="jw", to="currency", currency="USD"),
            "siji dolar seket sen",
        )
        self.assertEqual(
            num2words(0, lang="jw", to="currency", currency="EUR"), "nol euro"
        )
        self.assertEqual(
            num2words(0.01, lang="jw", to="currency", currency="EUR"),
            "nol euro siji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="jw", to="currency", currency="EUR"),
            "nol euro seket sen",
        )
        self.assertEqual(
            num2words(1, lang="jw", to="currency", currency="EUR"), "siji euro"
        )
        self.assertEqual(
            num2words(1.5, lang="jw", to="currency", currency="EUR"),
            "siji euro seket sen",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="jw", to="year"), "siji ewu")
        self.assertEqual(num2words(1066, lang="jw", to="year"), "siji ewu sewidak enem")
        self.assertEqual(
            num2words(1492, lang="jw", to="year"),
            "siji ewu papat atus sanga puluh loro",
        )
        self.assertEqual(
            num2words(1776, lang="jw", to="year"),
            "siji ewu pitu atus pitung puluh enem",
        )
        self.assertEqual(num2words(1800, lang="jw", to="year"), "siji ewu wolu atus")
        self.assertEqual(num2words(1900, lang="jw", to="year"), "siji ewu sanga atus")
        self.assertEqual(
            num2words(1984, lang="jw", to="year"),
            "siji ewu sanga atus wolung puluh papat",
        )
        self.assertEqual(
            num2words(1999, lang="jw", to="year"),
            "siji ewu sanga atus sanga puluh sanga",
        )
        self.assertEqual(num2words(2000, lang="jw", to="year"), "loro ewu")
        self.assertEqual(num2words(2001, lang="jw", to="year"), "loro ewu siji")
        self.assertEqual(num2words(2010, lang="jw", to="year"), "loro ewu sepuluh")
        self.assertEqual(num2words(2020, lang="jw", to="year"), "loro ewu rong puluh")
        self.assertEqual(
            num2words(2024, lang="jw", to="year"), "loro ewu rong puluh papat"
        )
        self.assertEqual(num2words(2100, lang="jw", to="year"), "loro ewu siji atus")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="jw"), "nol")
        self.assertEqual(num2words("1", lang="jw"), "siji")
        self.assertEqual(num2words("10", lang="jw"), "sepuluh")
        self.assertEqual(num2words("100", lang="jw"), "siji atus")
        self.assertEqual(num2words("1000", lang="jw"), "siji ewu")
        self.assertEqual(num2words("10000", lang="jw"), "sepuluh ewu")
        self.assertEqual(num2words("100000", lang="jw"), "siji atus ewu")
        self.assertEqual(num2words("1000000", lang="jw"), "siji yuta")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="jw"), "nol")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="jw"), num2words("100", lang="jw"))
        self.assertEqual(num2words(1000, lang="jw"), num2words("1000", lang="jw"))

