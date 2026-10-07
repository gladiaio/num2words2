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


class Num2WordsYITest(TestCase):
    """Comprehensive test cases for Yiddish language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="yi"), "נול")
        self.assertEqual(num2words(1, lang="yi"), "איינס")
        self.assertEqual(num2words(2, lang="yi"), "צוויי")
        self.assertEqual(num2words(3, lang="yi"), "דרײַ")
        self.assertEqual(num2words(4, lang="yi"), "פיר")
        self.assertEqual(num2words(5, lang="yi"), "פינף")
        self.assertEqual(num2words(6, lang="yi"), "זעקס")
        self.assertEqual(num2words(7, lang="yi"), "זיבן")
        self.assertEqual(num2words(8, lang="yi"), "אַכט")
        self.assertEqual(num2words(9, lang="yi"), "נײַן")
        self.assertEqual(num2words(10, lang="yi"), "צען")
        self.assertEqual(num2words(11, lang="yi"), "צען איינס")
        self.assertEqual(num2words(12, lang="yi"), "צען צוויי")
        self.assertEqual(num2words(13, lang="yi"), "צען דרײַ")
        self.assertEqual(num2words(14, lang="yi"), "צען פיר")
        self.assertEqual(num2words(15, lang="yi"), "צען פינף")
        self.assertEqual(num2words(16, lang="yi"), "צען זעקס")
        self.assertEqual(num2words(17, lang="yi"), "צען זיבן")
        self.assertEqual(num2words(18, lang="yi"), "צען אַכט")
        self.assertEqual(num2words(19, lang="yi"), "צען נײַן")
        self.assertEqual(num2words(20, lang="yi"), "צוואַנציק")
        self.assertEqual(num2words(21, lang="yi"), "צוואַנציק איינס")
        self.assertEqual(num2words(22, lang="yi"), "צוואַנציק צוויי")
        self.assertEqual(num2words(23, lang="yi"), "צוואַנציק דרײַ")
        self.assertEqual(num2words(24, lang="yi"), "צוואַנציק פיר")
        self.assertEqual(num2words(25, lang="yi"), "צוואַנציק פינף")
        self.assertEqual(num2words(26, lang="yi"), "צוואַנציק זעקס")
        self.assertEqual(num2words(27, lang="yi"), "צוואַנציק זיבן")
        self.assertEqual(num2words(28, lang="yi"), "צוואַנציק אַכט")
        self.assertEqual(num2words(29, lang="yi"), "צוואַנציק נײַן")
        self.assertEqual(num2words(30, lang="yi"), "דרײַסיק")
        self.assertEqual(num2words(31, lang="yi"), "דרײַסיק איינס")
        self.assertEqual(num2words(35, lang="yi"), "דרײַסיק פינף")
        self.assertEqual(num2words(40, lang="yi"), "פערציק")
        self.assertEqual(num2words(45, lang="yi"), "פערציק פינף")
        self.assertEqual(num2words(50, lang="yi"), "פופציק")
        self.assertEqual(num2words(55, lang="yi"), "פופציק פינף")
        self.assertEqual(num2words(60, lang="yi"), "זעכציק")
        self.assertEqual(num2words(65, lang="yi"), "זעכציק פינף")
        self.assertEqual(num2words(70, lang="yi"), "זיבעציק")
        self.assertEqual(num2words(75, lang="yi"), "זיבעציק פינף")
        self.assertEqual(num2words(80, lang="yi"), "אַכציק")
        self.assertEqual(num2words(85, lang="yi"), "אַכציק פינף")
        self.assertEqual(num2words(90, lang="yi"), "נײַנציק")
        self.assertEqual(num2words(95, lang="yi"), "נײַנציק פינף")
        self.assertEqual(num2words(99, lang="yi"), "נײַנציק נײַן")
        self.assertEqual(num2words(100, lang="yi"), "איינס הונדערט")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="yi"), "איינס הונדערט איינס")
        self.assertEqual(num2words(110, lang="yi"), "איינס הונדערט צען")
        self.assertEqual(num2words(111, lang="yi"), "איינס הונדערט צען איינס")
        self.assertEqual(num2words(120, lang="yi"), "איינס הונדערט צוואַנציק")
        self.assertEqual(num2words(125, lang="yi"), "איינס הונדערט צוואַנציק פינף")
        self.assertEqual(num2words(150, lang="yi"), "איינס הונדערט פופציק")
        self.assertEqual(num2words(175, lang="yi"), "איינס הונדערט זיבעציק פינף")
        self.assertEqual(num2words(199, lang="yi"), "איינס הונדערט נײַנציק נײַן")
        self.assertEqual(num2words(200, lang="yi"), "צוויי הונדערט")
        self.assertEqual(num2words(201, lang="yi"), "צוויי הונדערט איינס")
        self.assertEqual(num2words(210, lang="yi"), "צוויי הונדערט צען")
        self.assertEqual(num2words(220, lang="yi"), "צוויי הונדערט צוואַנציק")
        self.assertEqual(num2words(250, lang="yi"), "צוויי הונדערט פופציק")
        self.assertEqual(num2words(299, lang="yi"), "צוויי הונדערט נײַנציק נײַן")
        self.assertEqual(num2words(300, lang="yi"), "דרײַ הונדערט")
        self.assertEqual(num2words(333, lang="yi"), "דרײַ הונדערט דרײַסיק דרײַ")
        self.assertEqual(num2words(400, lang="yi"), "פיר הונדערט")
        self.assertEqual(num2words(444, lang="yi"), "פיר הונדערט פערציק פיר")
        self.assertEqual(num2words(500, lang="yi"), "פינף הונדערט")
        self.assertEqual(num2words(555, lang="yi"), "פינף הונדערט פופציק פינף")
        self.assertEqual(num2words(600, lang="yi"), "זעקס הונדערט")
        self.assertEqual(num2words(666, lang="yi"), "זעקס הונדערט זעכציק זעקס")
        self.assertEqual(num2words(700, lang="yi"), "זיבן הונדערט")
        self.assertEqual(num2words(777, lang="yi"), "זיבן הונדערט זיבעציק זיבן")
        self.assertEqual(num2words(800, lang="yi"), "אַכט הונדערט")
        self.assertEqual(num2words(888, lang="yi"), "אַכט הונדערט אַכציק אַכט")
        self.assertEqual(num2words(900, lang="yi"), "נײַן הונדערט")
        self.assertEqual(num2words(999, lang="yi"), "נײַן הונדערט נײַנציק נײַן")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="yi"), "איינס טויזנט")
        self.assertEqual(num2words(1001, lang="yi"), "איינס טויזנט איינס")
        self.assertEqual(num2words(1010, lang="yi"), "איינס טויזנט צען")
        self.assertEqual(num2words(1100, lang="yi"), "איינס טויזנט איינס הונדערט")
        self.assertEqual(
            num2words(1111, lang="yi"), "איינס טויזנט איינס הונדערט צען איינס"
        )
        self.assertEqual(
            num2words(1234, lang="yi"), "איינס טויזנט צוויי הונדערט דרײַסיק פיר"
        )
        self.assertEqual(num2words(1500, lang="yi"), "איינס טויזנט פינף הונדערט")
        self.assertEqual(
            num2words(1999, lang="yi"), "איינס טויזנט נײַן הונדערט נײַנציק נײַן"
        )
        self.assertEqual(num2words(2000, lang="yi"), "צוויי טויזנט")
        self.assertEqual(num2words(2001, lang="yi"), "צוויי טויזנט איינס")
        self.assertEqual(num2words(2020, lang="yi"), "צוויי טויזנט צוואַנציק")
        self.assertEqual(
            num2words(2222, lang="yi"), "צוויי טויזנט צוויי הונדערט צוואַנציק צוויי"
        )
        self.assertEqual(num2words(3000, lang="yi"), "דרײַ טויזנט")
        self.assertEqual(
            num2words(3333, lang="yi"), "דרײַ טויזנט דרײַ הונדערט דרײַסיק דרײַ"
        )
        self.assertEqual(num2words(4000, lang="yi"), "פיר טויזנט")
        self.assertEqual(
            num2words(4444, lang="yi"), "פיר טויזנט פיר הונדערט פערציק פיר"
        )
        self.assertEqual(num2words(5000, lang="yi"), "פינף טויזנט")
        self.assertEqual(
            num2words(5555, lang="yi"), "פינף טויזנט פינף הונדערט פופציק פינף"
        )
        self.assertEqual(num2words(6000, lang="yi"), "זעקס טויזנט")
        self.assertEqual(
            num2words(6666, lang="yi"), "זעקס טויזנט זעקס הונדערט זעכציק זעקס"
        )
        self.assertEqual(num2words(7000, lang="yi"), "זיבן טויזנט")
        self.assertEqual(
            num2words(7777, lang="yi"), "זיבן טויזנט זיבן הונדערט זיבעציק זיבן"
        )
        self.assertEqual(num2words(8000, lang="yi"), "אַכט טויזנט")
        self.assertEqual(
            num2words(8888, lang="yi"), "אַכט טויזנט אַכט הונדערט אַכציק אַכט"
        )
        self.assertEqual(num2words(9000, lang="yi"), "נײַן טויזנט")
        self.assertEqual(
            num2words(9999, lang="yi"), "נײַן טויזנט נײַן הונדערט נײַנציק נײַן"
        )
        self.assertEqual(num2words(10000, lang="yi"), "צען טויזנט")
        self.assertEqual(num2words(10001, lang="yi"), "צען טויזנט איינס")
        self.assertEqual(
            num2words(11111, lang="yi"), "צען איינס טויזנט איינס הונדערט צען איינס"
        )
        self.assertEqual(
            num2words(12345, lang="yi"), "צען צוויי טויזנט דרײַ הונדערט פערציק פינף"
        )
        self.assertEqual(num2words(20000, lang="yi"), "צוואַנציק טויזנט")
        self.assertEqual(num2words(50000, lang="yi"), "פופציק טויזנט")
        self.assertEqual(
            num2words(99999, lang="yi"), "נײַנציק נײַן טויזנט נײַן הונדערט נײַנציק נײַן"
        )
        self.assertEqual(num2words(100000, lang="yi"), "איינס הונדערט טויזנט")
        self.assertEqual(
            num2words(123456, lang="yi"),
            "איינס הונדערט צוואַנציק דרײַ טויזנט פיר הונדערט פופציק זעקס",
        )
        self.assertEqual(num2words(200000, lang="yi"), "צוויי הונדערט טויזנט")
        self.assertEqual(num2words(500000, lang="yi"), "פינף הונדערט טויזנט")
        self.assertEqual(
            num2words(654321, lang="yi"),
            "זעקס הונדערט פופציק פיר טויזנט דרײַ הונדערט צוואַנציק איינס",
        )
        self.assertEqual(
            num2words(999999, lang="yi"),
            "נײַן הונדערט נײַנציק נײַן טויזנט נײַן הונדערט נײַנציק נײַן",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="yi"), "איינס מיליאָן")
        self.assertEqual(num2words(1000001, lang="yi"), "איינס מיליאָן איינס")
        self.assertEqual(
            num2words(1111111, lang="yi"),
            "איינס מיליאָן איינס הונדערט צען איינס טויזנט איינס הונדערט צען איינס",
        )
        self.assertEqual(
            num2words(1234567, lang="yi"),
            "איינס מיליאָן צוויי הונדערט דרײַסיק פיר טויזנט פינף הונדערט זעכציק זיבן",
        )
        self.assertEqual(num2words(2000000, lang="yi"), "צוויי מיליאָן")
        self.assertEqual(num2words(5000000, lang="yi"), "פינף מיליאָן")
        self.assertEqual(
            num2words(9999999, lang="yi"),
            "נײַן מיליאָן נײַן הונדערט נײַנציק נײַן טויזנט נײַן הונדערט נײַנציק נײַן",
        )
        self.assertEqual(num2words(10000000, lang="yi"), "צען מיליאָן")
        self.assertEqual(
            num2words(12345678, lang="yi"),
            "צען צוויי מיליאָן דרײַ הונדערט פערציק פינף טויזנט זעקס הונדערט זיבעציק אַכט",
        )
        self.assertEqual(
            num2words(99999999, lang="yi"),
            "נײַנציק נײַן מיליאָן נײַן הונדערט נײַנציק נײַן טויזנט נײַן הונדערט נײַנציק נײַן",
        )
        self.assertEqual(num2words(100000000, lang="yi"), "איינס הונדערט מיליאָן")
        self.assertEqual(
            num2words(123456789, lang="yi"),
            "איינס הונדערט צוואַנציק דרײַ מיליאָן פיר הונדערט פופציק זעקס טויזנט זיבן הונדערט אַכציק נײַן",
        )
        self.assertEqual(
            num2words(999999999, lang="yi"),
            "נײַן הונדערט נײַנציק נײַן מיליאָן נײַן הונדערט נײַנציק נײַן טויזנט נײַן הונדערט נײַנציק נײַן",
        )
        self.assertEqual(num2words(1000000000, lang="yi"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="yi"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="yi"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="yi"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="yi"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="yi"), "מינוס איינס")
        self.assertEqual(num2words(-2, lang="yi"), "מינוס צוויי")
        self.assertEqual(num2words(-5, lang="yi"), "מינוס פינף")
        self.assertEqual(num2words(-10, lang="yi"), "מינוס צען")
        self.assertEqual(num2words(-11, lang="yi"), "מינוס צען איינס")
        self.assertEqual(num2words(-20, lang="yi"), "מינוס צוואַנציק")
        self.assertEqual(num2words(-50, lang="yi"), "מינוס פופציק")
        self.assertEqual(num2words(-99, lang="yi"), "מינוס נײַנציק נײַן")
        self.assertEqual(num2words(-100, lang="yi"), "מינוס איינס הונדערט")
        self.assertEqual(num2words(-101, lang="yi"), "מינוס איינס הונדערט איינס")
        self.assertEqual(num2words(-200, lang="yi"), "מינוס צוויי הונדערט")
        self.assertEqual(num2words(-999, lang="yi"), "מינוס נײַן הונדערט נײַנציק נײַן")
        self.assertEqual(num2words(-1000, lang="yi"), "מינוס איינס טויזנט")
        self.assertEqual(num2words(-1001, lang="yi"), "מינוס איינס טויזנט איינס")
        self.assertEqual(num2words(-10000, lang="yi"), "מינוס צען טויזנט")
        self.assertEqual(num2words(-100000, lang="yi"), "מינוס איינס הונדערט טויזנט")
        self.assertEqual(num2words(-1000000, lang="yi"), "מינוס איינס מיליאָן")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="yi"), "נול פּונקט איינס")
        self.assertEqual(num2words(0.5, lang="yi"), "נול פּונקט פינף")
        self.assertEqual(num2words(0.9, lang="yi"), "נול פּונקט נײַן")
        self.assertEqual(num2words(1.1, lang="yi"), "איינס פּונקט איינס")
        self.assertEqual(num2words(1.5, lang="yi"), "איינס פּונקט פינף")
        self.assertEqual(num2words(2.5, lang="yi"), "צוויי פּונקט פינף")
        self.assertEqual(num2words(3.14, lang="yi"), "דרײַ פּונקט איינס פיר")
        self.assertEqual(num2words(10.5, lang="yi"), "צען פּונקט פינף")
        self.assertEqual(num2words(11.11, lang="yi"), "צען איינס פּונקט איינס איינס")
        self.assertEqual(num2words(20.2, lang="yi"), "צוואַנציק פּונקט צוויי")
        self.assertEqual(num2words(99.99, lang="yi"), "נײַנציק נײַן פּונקט נײַן נײַן")
        self.assertEqual(num2words(100.01, lang="yi"), "איינס הונדערט פּונקט נול איינס")
        self.assertEqual(num2words(100.5, lang="yi"), "איינס הונדערט פּונקט פינף")
        self.assertEqual(
            num2words(123.45, lang="yi"), "איינס הונדערט צוואַנציק דרײַ פּונקט פיר פינף"
        )
        self.assertEqual(num2words(1000.5, lang="yi"), "איינס טויזנט פּונקט פינף")
        self.assertEqual(
            num2words(1234.56, lang="yi"),
            "איינס טויזנט צוויי הונדערט דרײַסיק פיר פּונקט פינף זעקס",
        )
        self.assertEqual(num2words(10000.01, lang="yi"), "צען טויזנט פּונקט נול איינס")
        self.assertEqual(num2words(-0.5, lang="yi"), "מינוס נול פּונקט פינף")
        self.assertEqual(num2words(-1.5, lang="yi"), "מינוס איינס פּונקט פינף")
        self.assertEqual(num2words(-10.5, lang="yi"), "מינוס צען פּונקט פינף")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="yi", ordinal=True), "איינס-טער")
        self.assertEqual(num2words(2, lang="yi", ordinal=True), "צוויי-טער")
        self.assertEqual(num2words(3, lang="yi", ordinal=True), "דרײַ-טער")
        self.assertEqual(num2words(4, lang="yi", ordinal=True), "פיר-טער")
        self.assertEqual(num2words(5, lang="yi", ordinal=True), "פינף-טער")
        self.assertEqual(num2words(6, lang="yi", ordinal=True), "זעקס-טער")
        self.assertEqual(num2words(7, lang="yi", ordinal=True), "זיבן-טער")
        self.assertEqual(num2words(8, lang="yi", ordinal=True), "אַכט-טער")
        self.assertEqual(num2words(9, lang="yi", ordinal=True), "נײַן-טער")
        self.assertEqual(num2words(10, lang="yi", ordinal=True), "צען-טער")
        self.assertEqual(num2words(11, lang="yi", ordinal=True), "צען איינס-טער")
        self.assertEqual(num2words(12, lang="yi", ordinal=True), "צען צוויי-טער")
        self.assertEqual(num2words(13, lang="yi", ordinal=True), "צען דרײַ-טער")
        self.assertEqual(num2words(14, lang="yi", ordinal=True), "צען פיר-טער")
        self.assertEqual(num2words(15, lang="yi", ordinal=True), "צען פינף-טער")
        self.assertEqual(num2words(16, lang="yi", ordinal=True), "צען זעקס-טער")
        self.assertEqual(num2words(17, lang="yi", ordinal=True), "צען זיבן-טער")
        self.assertEqual(num2words(18, lang="yi", ordinal=True), "צען אַכט-טער")
        self.assertEqual(num2words(19, lang="yi", ordinal=True), "צען נײַן-טער")
        self.assertEqual(num2words(20, lang="yi", ordinal=True), "צוואַנציק-טער")
        self.assertEqual(num2words(21, lang="yi", ordinal=True), "צוואַנציק איינס-טער")
        self.assertEqual(num2words(22, lang="yi", ordinal=True), "צוואַנציק צוויי-טער")
        self.assertEqual(num2words(25, lang="yi", ordinal=True), "צוואַנציק פינף-טער")
        self.assertEqual(num2words(30, lang="yi", ordinal=True), "דרײַסיק-טער")
        self.assertEqual(num2words(40, lang="yi", ordinal=True), "פערציק-טער")
        self.assertEqual(num2words(50, lang="yi", ordinal=True), "פופציק-טער")
        self.assertEqual(num2words(60, lang="yi", ordinal=True), "זעכציק-טער")
        self.assertEqual(num2words(70, lang="yi", ordinal=True), "זיבעציק-טער")
        self.assertEqual(num2words(80, lang="yi", ordinal=True), "אַכציק-טער")
        self.assertEqual(num2words(90, lang="yi", ordinal=True), "נײַנציק-טער")
        self.assertEqual(num2words(100, lang="yi", ordinal=True), "איינס הונדערט-טער")
        self.assertEqual(
            num2words(101, lang="yi", ordinal=True), "איינס הונדערט איינס-טער"
        )
        self.assertEqual(num2words(200, lang="yi", ordinal=True), "צוויי הונדערט-טער")
        self.assertEqual(num2words(500, lang="yi", ordinal=True), "פינף הונדערט-טער")
        self.assertEqual(num2words(1000, lang="yi", ordinal=True), "איינס טויזנט-טער")
        self.assertEqual(
            num2words(1001, lang="yi", ordinal=True), "איינס טויזנט איינס-טער"
        )
        self.assertEqual(num2words(10000, lang="yi", ordinal=True), "צען טויזנט-טער")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="yi", to="currency", currency="EUR"), "נול אייראָ"
        )
        self.assertEqual(
            num2words(0.01, lang="yi", to="currency", currency="EUR"),
            "נול אייראָ איינס צענט",
        )
        self.assertEqual(
            num2words(0.5, lang="yi", to="currency", currency="EUR"),
            "נול אייראָ פופציק צענט",
        )
        self.assertEqual(
            num2words(1, lang="yi", to="currency", currency="EUR"), "איינס אייראָ"
        )
        self.assertEqual(
            num2words(1.5, lang="yi", to="currency", currency="EUR"),
            "איינס אייראָ פופציק צענט",
        )
        self.assertEqual(
            num2words(0, lang="yi", to="currency", currency="USD"), "נול דאָלאַר"
        )
        self.assertEqual(
            num2words(0.01, lang="yi", to="currency", currency="USD"),
            "נול דאָלאַר איינס סענט",
        )
        self.assertEqual(
            num2words(0.5, lang="yi", to="currency", currency="USD"),
            "נול דאָלאַר פופציק סענט",
        )
        self.assertEqual(
            num2words(1, lang="yi", to="currency", currency="USD"), "איינס דאָלאַר"
        )
        self.assertEqual(
            num2words(1.5, lang="yi", to="currency", currency="USD"),
            "איינס דאָלאַר פופציק סענט",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="yi", to="year"), "איינס טויזנט")
        self.assertEqual(
            num2words(1066, lang="yi", to="year"), "איינס טויזנט זעכציק זעקס"
        )
        self.assertEqual(
            num2words(1492, lang="yi", to="year"),
            "איינס טויזנט פיר הונדערט נײַנציק צוויי",
        )
        self.assertEqual(
            num2words(1776, lang="yi", to="year"),
            "איינס טויזנט זיבן הונדערט זיבעציק זעקס",
        )
        self.assertEqual(
            num2words(1800, lang="yi", to="year"), "איינס טויזנט אַכט הונדערט"
        )
        self.assertEqual(
            num2words(1900, lang="yi", to="year"), "איינס טויזנט נײַן הונדערט"
        )
        self.assertEqual(
            num2words(1984, lang="yi", to="year"),
            "איינס טויזנט נײַן הונדערט אַכציק פיר",
        )
        self.assertEqual(
            num2words(1999, lang="yi", to="year"),
            "איינס טויזנט נײַן הונדערט נײַנציק נײַן",
        )
        self.assertEqual(num2words(2000, lang="yi", to="year"), "צוויי טויזנט")
        self.assertEqual(num2words(2001, lang="yi", to="year"), "צוויי טויזנט איינס")
        self.assertEqual(num2words(2010, lang="yi", to="year"), "צוויי טויזנט צען")
        self.assertEqual(
            num2words(2020, lang="yi", to="year"), "צוויי טויזנט צוואַנציק"
        )
        self.assertEqual(
            num2words(2024, lang="yi", to="year"), "צוויי טויזנט צוואַנציק פיר"
        )
        self.assertEqual(
            num2words(2100, lang="yi", to="year"), "צוויי טויזנט איינס הונדערט"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="yi"), "נול")
        self.assertEqual(num2words("1", lang="yi"), "איינס")
        self.assertEqual(num2words("10", lang="yi"), "צען")
        self.assertEqual(num2words("100", lang="yi"), "איינס הונדערט")
        self.assertEqual(num2words("1000", lang="yi"), "איינס טויזנט")
        self.assertEqual(num2words("10000", lang="yi"), "צען טויזנט")
        self.assertEqual(num2words("100000", lang="yi"), "איינס הונדערט טויזנט")
        self.assertEqual(num2words("1000000", lang="yi"), "איינס מיליאָן")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="yi"), "נול")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="yi"), num2words("100", lang="yi"))
        self.assertEqual(num2words(1000, lang="yi"), num2words("1000", lang="yi"))

