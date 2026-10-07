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


class Num2WordsSWTest(TestCase):
    """Comprehensive test cases for Swahili language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sw"), "sifuri")
        self.assertEqual(num2words(1, lang="sw"), "moja")
        self.assertEqual(num2words(2, lang="sw"), "mbili")
        self.assertEqual(num2words(3, lang="sw"), "tatu")
        self.assertEqual(num2words(4, lang="sw"), "nne")
        self.assertEqual(num2words(5, lang="sw"), "tano")
        self.assertEqual(num2words(6, lang="sw"), "sita")
        self.assertEqual(num2words(7, lang="sw"), "saba")
        self.assertEqual(num2words(8, lang="sw"), "nane")
        self.assertEqual(num2words(9, lang="sw"), "tisa")
        self.assertEqual(num2words(10, lang="sw"), "kumi")
        self.assertEqual(num2words(11, lang="sw"), "kumi na moja")
        self.assertEqual(num2words(12, lang="sw"), "kumi na mbili")
        self.assertEqual(num2words(13, lang="sw"), "kumi na tatu")
        self.assertEqual(num2words(14, lang="sw"), "kumi na nne")
        self.assertEqual(num2words(15, lang="sw"), "kumi na tano")
        self.assertEqual(num2words(16, lang="sw"), "kumi na sita")
        self.assertEqual(num2words(17, lang="sw"), "kumi na saba")
        self.assertEqual(num2words(18, lang="sw"), "kumi na nane")
        self.assertEqual(num2words(19, lang="sw"), "kumi na tisa")
        self.assertEqual(num2words(20, lang="sw"), "ishirini")
        self.assertEqual(num2words(21, lang="sw"), "ishirini na moja")
        self.assertEqual(num2words(22, lang="sw"), "ishirini na mbili")
        self.assertEqual(num2words(23, lang="sw"), "ishirini na tatu")
        self.assertEqual(num2words(24, lang="sw"), "ishirini na nne")
        self.assertEqual(num2words(25, lang="sw"), "ishirini na tano")
        self.assertEqual(num2words(26, lang="sw"), "ishirini na sita")
        self.assertEqual(num2words(27, lang="sw"), "ishirini na saba")
        self.assertEqual(num2words(28, lang="sw"), "ishirini na nane")
        self.assertEqual(num2words(29, lang="sw"), "ishirini na tisa")
        self.assertEqual(num2words(30, lang="sw"), "thelathini")
        self.assertEqual(num2words(31, lang="sw"), "thelathini na moja")
        self.assertEqual(num2words(35, lang="sw"), "thelathini na tano")
        self.assertEqual(num2words(40, lang="sw"), "arobaini")
        self.assertEqual(num2words(45, lang="sw"), "arobaini na tano")
        self.assertEqual(num2words(50, lang="sw"), "hamsini")
        self.assertEqual(num2words(55, lang="sw"), "hamsini na tano")
        self.assertEqual(num2words(60, lang="sw"), "sitini")
        self.assertEqual(num2words(65, lang="sw"), "sitini na tano")
        self.assertEqual(num2words(70, lang="sw"), "sabini")
        self.assertEqual(num2words(75, lang="sw"), "sabini na tano")
        self.assertEqual(num2words(80, lang="sw"), "themanini")
        self.assertEqual(num2words(85, lang="sw"), "themanini na tano")
        self.assertEqual(num2words(90, lang="sw"), "tisini")
        self.assertEqual(num2words(95, lang="sw"), "tisini na tano")
        self.assertEqual(num2words(99, lang="sw"), "tisini na tisa")
        self.assertEqual(num2words(100, lang="sw"), "mia moja")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sw"), "mia moja na moja")
        self.assertEqual(num2words(110, lang="sw"), "mia moja na kumi")
        self.assertEqual(num2words(111, lang="sw"), "mia moja na kumi na moja")
        self.assertEqual(num2words(120, lang="sw"), "mia moja na ishirini")
        self.assertEqual(num2words(125, lang="sw"), "mia moja na ishirini na tano")
        self.assertEqual(num2words(150, lang="sw"), "mia moja na hamsini")
        self.assertEqual(num2words(175, lang="sw"), "mia moja na sabini na tano")
        self.assertEqual(num2words(199, lang="sw"), "mia moja na tisini na tisa")
        self.assertEqual(num2words(200, lang="sw"), "mia mbili")
        self.assertEqual(num2words(201, lang="sw"), "mia mbili na moja")
        self.assertEqual(num2words(210, lang="sw"), "mia mbili na kumi")
        self.assertEqual(num2words(220, lang="sw"), "mia mbili na ishirini")
        self.assertEqual(num2words(250, lang="sw"), "mia mbili na hamsini")
        self.assertEqual(num2words(299, lang="sw"), "mia mbili na tisini na tisa")
        self.assertEqual(num2words(300, lang="sw"), "mia tatu")
        self.assertEqual(num2words(333, lang="sw"), "mia tatu na thelathini na tatu")
        self.assertEqual(num2words(400, lang="sw"), "mia nne")
        self.assertEqual(num2words(444, lang="sw"), "mia nne na arobaini na nne")
        self.assertEqual(num2words(500, lang="sw"), "mia tano")
        self.assertEqual(num2words(555, lang="sw"), "mia tano na hamsini na tano")
        self.assertEqual(num2words(600, lang="sw"), "mia sita")
        self.assertEqual(num2words(666, lang="sw"), "mia sita na sitini na sita")
        self.assertEqual(num2words(700, lang="sw"), "mia saba")
        self.assertEqual(num2words(777, lang="sw"), "mia saba na sabini na saba")
        self.assertEqual(num2words(800, lang="sw"), "mia nane")
        self.assertEqual(num2words(888, lang="sw"), "mia nane na themanini na nane")
        self.assertEqual(num2words(900, lang="sw"), "mia tisa")
        self.assertEqual(num2words(999, lang="sw"), "mia tisa na tisini na tisa")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sw"), "elfu moja")
        self.assertEqual(num2words(1001, lang="sw"), "elfu moja na moja")
        self.assertEqual(num2words(1010, lang="sw"), "elfu moja na kumi")
        self.assertEqual(num2words(1100, lang="sw"), "elfu moja na mia moja")
        self.assertEqual(
            num2words(1111, lang="sw"), "elfu moja na mia moja na kumi na moja"
        )
        self.assertEqual(
            num2words(1234, lang="sw"), "elfu moja na mia mbili na thelathini na nne"
        )
        self.assertEqual(num2words(1500, lang="sw"), "elfu moja na mia tano")
        self.assertEqual(
            num2words(1999, lang="sw"), "elfu moja na mia tisa na tisini na tisa"
        )
        self.assertEqual(num2words(2000, lang="sw"), "elfu mbili")
        self.assertEqual(num2words(2001, lang="sw"), "elfu mbili na moja")
        self.assertEqual(num2words(2020, lang="sw"), "elfu mbili na ishirini")
        self.assertEqual(
            num2words(2222, lang="sw"), "elfu mbili na mia mbili na ishirini na mbili"
        )
        self.assertEqual(num2words(3000, lang="sw"), "elfu tatu")
        self.assertEqual(
            num2words(3333, lang="sw"), "elfu tatu na mia tatu na thelathini na tatu"
        )
        self.assertEqual(num2words(4000, lang="sw"), "elfu nne")
        self.assertEqual(
            num2words(4444, lang="sw"), "elfu nne na mia nne na arobaini na nne"
        )
        self.assertEqual(num2words(5000, lang="sw"), "elfu tano")
        self.assertEqual(
            num2words(5555, lang="sw"), "elfu tano na mia tano na hamsini na tano"
        )
        self.assertEqual(num2words(6000, lang="sw"), "elfu sita")
        self.assertEqual(
            num2words(6666, lang="sw"), "elfu sita na mia sita na sitini na sita"
        )
        self.assertEqual(num2words(7000, lang="sw"), "elfu saba")
        self.assertEqual(
            num2words(7777, lang="sw"), "elfu saba na mia saba na sabini na saba"
        )
        self.assertEqual(num2words(8000, lang="sw"), "elfu nane")
        self.assertEqual(
            num2words(8888, lang="sw"), "elfu nane na mia nane na themanini na nane"
        )
        self.assertEqual(num2words(9000, lang="sw"), "elfu tisa")
        self.assertEqual(
            num2words(9999, lang="sw"), "elfu tisa na mia tisa na tisini na tisa"
        )
        self.assertEqual(num2words(10000, lang="sw"), "elfu kumi")
        self.assertEqual(num2words(10001, lang="sw"), "elfu kumi na moja")
        self.assertEqual(
            num2words(11111, lang="sw"), "elfu kumi na moja na mia moja na kumi na moja"
        )
        self.assertEqual(
            num2words(12345, lang="sw"),
            "elfu kumi na mbili na mia tatu na arobaini na tano",
        )
        self.assertEqual(num2words(20000, lang="sw"), "elfu ishirini")
        self.assertEqual(num2words(50000, lang="sw"), "elfu hamsini")
        self.assertEqual(
            num2words(99999, lang="sw"),
            "elfu tisini na tisa na mia tisa na tisini na tisa",
        )
        self.assertEqual(num2words(100000, lang="sw"), "laki moja")
        self.assertEqual(
            num2words(123456, lang="sw"),
            "laki moja na elfu ishirini na tatu na mia nne na hamsini na sita",
        )
        self.assertEqual(num2words(200000, lang="sw"), "laki mbili")
        self.assertEqual(num2words(500000, lang="sw"), "laki tano")
        self.assertEqual(
            num2words(654321, lang="sw"),
            "laki sita na elfu hamsini na nne na mia tatu na ishirini na moja",
        )
        self.assertEqual(
            num2words(999999, lang="sw"),
            "laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sw"), "milioni moja")
        self.assertEqual(num2words(1000001, lang="sw"), "milioni moja na moja")
        self.assertEqual(
            num2words(1111111, lang="sw"),
            "milioni moja na laki moja na elfu kumi na moja na mia moja na kumi na moja",
        )
        self.assertEqual(
            num2words(1234567, lang="sw"),
            "milioni moja na laki mbili na elfu thelathini na nne na mia tano na sitini na saba",
        )
        self.assertEqual(num2words(2000000, lang="sw"), "milioni mbili")
        self.assertEqual(num2words(5000000, lang="sw"), "milioni tano")
        self.assertEqual(
            num2words(9999999, lang="sw"),
            "milioni tisa na laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa",
        )
        self.assertEqual(num2words(10000000, lang="sw"), "milioni kumi")
        self.assertEqual(
            num2words(12345678, lang="sw"),
            "milioni kumi na mbili na laki tatu na elfu arobaini na tano na mia sita na sabini na nane",
        )
        self.assertEqual(
            num2words(99999999, lang="sw"),
            "milioni tisini na tisa na laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa",
        )
        self.assertEqual(num2words(100000000, lang="sw"), "milioni mia moja")
        self.assertEqual(
            num2words(123456789, lang="sw"),
            "milioni mia moja na ishirini na tatu na laki nne na elfu hamsini na sita na mia saba na themanini na tisa",
        )
        self.assertEqual(
            num2words(999999999, lang="sw"),
            "milioni mia tisa na tisini na tisa na laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa",
        )
        self.assertEqual(num2words(1000000000, lang="sw"), "bilioni moja")
        self.assertEqual(
            num2words(1234567890, lang="sw"),
            "milioni mia mbili na thelathini na nne na laki tano na elfu sitini na saba na mia nane na tisini bilioni moja",
        )
        self.assertEqual(
            num2words(9999999999, lang="sw"),
            "milioni mia tisa na tisini na tisa na laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa bilioni tisa",
        )
        self.assertEqual(num2words(10000000000, lang="sw"), "bilioni kumi")
        self.assertEqual(
            num2words(99999999999, lang="sw"),
            "milioni mia tisa na tisini na tisa na laki tisa na elfu tisini na tisa na mia tisa na tisini na tisa bilioni tisini na tisa",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sw"), "hasi moja")
        self.assertEqual(num2words(-2, lang="sw"), "hasi mbili")
        self.assertEqual(num2words(-5, lang="sw"), "hasi tano")
        self.assertEqual(num2words(-10, lang="sw"), "hasi kumi")
        self.assertEqual(num2words(-11, lang="sw"), "hasi kumi na moja")
        self.assertEqual(num2words(-20, lang="sw"), "hasi ishirini")
        self.assertEqual(num2words(-50, lang="sw"), "hasi hamsini")
        self.assertEqual(num2words(-99, lang="sw"), "hasi tisini na tisa")
        self.assertEqual(num2words(-100, lang="sw"), "hasi mia moja")
        self.assertEqual(num2words(-101, lang="sw"), "hasi mia moja na moja")
        self.assertEqual(num2words(-200, lang="sw"), "hasi mia mbili")
        self.assertEqual(num2words(-999, lang="sw"), "hasi mia tisa na tisini na tisa")
        self.assertEqual(num2words(-1000, lang="sw"), "hasi elfu moja")
        self.assertEqual(num2words(-1001, lang="sw"), "hasi elfu moja na moja")
        self.assertEqual(num2words(-10000, lang="sw"), "hasi elfu kumi")
        self.assertEqual(num2words(-100000, lang="sw"), "hasi laki moja")
        self.assertEqual(num2words(-1000000, lang="sw"), "hasi milioni moja")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sw"), "sifuri nukta moja")
        self.assertEqual(num2words(0.5, lang="sw"), "sifuri nukta tano")
        self.assertEqual(num2words(0.9, lang="sw"), "sifuri nukta tisa")
        self.assertEqual(num2words(1.1, lang="sw"), "moja nukta moja")
        self.assertEqual(num2words(1.5, lang="sw"), "moja nukta tano")
        self.assertEqual(num2words(2.5, lang="sw"), "mbili nukta tano")
        self.assertEqual(num2words(3.14, lang="sw"), "tatu nukta moja nne")
        self.assertEqual(num2words(10.5, lang="sw"), "kumi nukta tano")
        self.assertEqual(num2words(11.11, lang="sw"), "kumi na moja nukta moja moja")
        self.assertEqual(num2words(20.2, lang="sw"), "ishirini nukta mbili")
        self.assertEqual(num2words(99.99, lang="sw"), "tisini na tisa nukta tisa tisa")
        self.assertEqual(num2words(100.01, lang="sw"), "mia moja nukta sifuri moja")
        self.assertEqual(num2words(100.5, lang="sw"), "mia moja nukta tano")
        self.assertEqual(
            num2words(123.45, lang="sw"), "mia moja na ishirini na tatu nukta nne tano"
        )
        self.assertEqual(num2words(1000.5, lang="sw"), "elfu moja nukta tano")
        self.assertEqual(
            num2words(1234.56, lang="sw"),
            "elfu moja na mia mbili na thelathini na nne nukta tano sita",
        )
        self.assertEqual(num2words(10000.01, lang="sw"), "elfu kumi nukta sifuri moja")
        self.assertEqual(num2words(-0.5, lang="sw"), "hasi sifuri nukta tano")
        self.assertEqual(num2words(-1.5, lang="sw"), "hasi moja nukta tano")
        self.assertEqual(num2words(-10.5, lang="sw"), "hasi kumi nukta tano")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sw", ordinal=True), "wa kwanza")
        self.assertEqual(num2words(2, lang="sw", ordinal=True), "wa pili")
        self.assertEqual(num2words(3, lang="sw", ordinal=True), "wa tatu")
        self.assertEqual(num2words(4, lang="sw", ordinal=True), "wa nne")
        self.assertEqual(num2words(5, lang="sw", ordinal=True), "wa tano")
        self.assertEqual(num2words(6, lang="sw", ordinal=True), "wa sita")
        self.assertEqual(num2words(7, lang="sw", ordinal=True), "wa saba")
        self.assertEqual(num2words(8, lang="sw", ordinal=True), "wa nane")
        self.assertEqual(num2words(9, lang="sw", ordinal=True), "wa tisa")
        self.assertEqual(num2words(10, lang="sw", ordinal=True), "wa kumi")
        self.assertEqual(num2words(11, lang="sw", ordinal=True), "wa kumi na moja")
        self.assertEqual(num2words(12, lang="sw", ordinal=True), "wa kumi na mbili")
        self.assertEqual(num2words(13, lang="sw", ordinal=True), "wa kumi na tatu")
        self.assertEqual(num2words(14, lang="sw", ordinal=True), "wa kumi na nne")
        self.assertEqual(num2words(15, lang="sw", ordinal=True), "wa kumi na tano")
        self.assertEqual(num2words(16, lang="sw", ordinal=True), "wa kumi na sita")
        self.assertEqual(num2words(17, lang="sw", ordinal=True), "wa kumi na saba")
        self.assertEqual(num2words(18, lang="sw", ordinal=True), "wa kumi na nane")
        self.assertEqual(num2words(19, lang="sw", ordinal=True), "wa kumi na tisa")
        self.assertEqual(num2words(20, lang="sw", ordinal=True), "wa ishirini")
        self.assertEqual(num2words(21, lang="sw", ordinal=True), "wa ishirini na moja")
        self.assertEqual(num2words(22, lang="sw", ordinal=True), "wa ishirini na mbili")
        self.assertEqual(num2words(25, lang="sw", ordinal=True), "wa ishirini na tano")
        self.assertEqual(num2words(30, lang="sw", ordinal=True), "wa thelathini")
        self.assertEqual(num2words(40, lang="sw", ordinal=True), "wa arobaini")
        self.assertEqual(num2words(50, lang="sw", ordinal=True), "wa hamsini")
        self.assertEqual(num2words(60, lang="sw", ordinal=True), "wa sitini")
        self.assertEqual(num2words(70, lang="sw", ordinal=True), "wa sabini")
        self.assertEqual(num2words(80, lang="sw", ordinal=True), "wa themanini")
        self.assertEqual(num2words(90, lang="sw", ordinal=True), "wa tisini")
        self.assertEqual(num2words(100, lang="sw", ordinal=True), "wa mia moja")
        self.assertEqual(num2words(101, lang="sw", ordinal=True), "wa mia moja na moja")
        self.assertEqual(num2words(200, lang="sw", ordinal=True), "wa mia mbili")
        self.assertEqual(num2words(500, lang="sw", ordinal=True), "wa mia tano")
        self.assertEqual(num2words(1000, lang="sw", ordinal=True), "wa elfu moja")
        self.assertEqual(
            num2words(1001, lang="sw", ordinal=True), "wa elfu moja na moja"
        )
        self.assertEqual(num2words(10000, lang="sw", ordinal=True), "wa elfu kumi")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="TZS"), "sifuri shilingi"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="TZS"),
            "sifuri shilingi na moja senti",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="TZS"),
            "sifuri shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="TZS"), "moja shilingi"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="TZS"),
            "moja shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="KES"), "sifuri shilingi"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="KES"),
            "sifuri shilingi na moja senti",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="KES"),
            "sifuri shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="KES"), "moja shilingi"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="KES"),
            "moja shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="UGX"), "sifuri shilingi"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="UGX"),
            "sifuri shilingi na moja senti",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="UGX"),
            "sifuri shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="UGX"), "moja shilingi"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="UGX"),
            "moja shilingi na hamsini senti",
        )
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="USD"), "sifuri dola"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="USD"),
            "sifuri dola na moja senti",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="USD"),
            "sifuri dola na hamsini senti",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="USD"), "moja dola"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="USD"),
            "moja dola na hamsini senti",
        )
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="EUR"), "sifuri yuro"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="EUR"),
            "sifuri yuro na moja senti",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="EUR"),
            "sifuri yuro na hamsini senti",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="EUR"), "moja yuro"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="EUR"),
            "moja yuro na hamsini senti",
        )
        self.assertEqual(
            num2words(0, lang="sw", to="currency", currency="GBP"), "sifuri pauni"
        )
        self.assertEqual(
            num2words(0.01, lang="sw", to="currency", currency="GBP"),
            "sifuri pauni na moja peni",
        )
        self.assertEqual(
            num2words(0.5, lang="sw", to="currency", currency="GBP"),
            "sifuri pauni na hamsini peni",
        )
        self.assertEqual(
            num2words(1, lang="sw", to="currency", currency="GBP"), "moja pauni"
        )
        self.assertEqual(
            num2words(1.5, lang="sw", to="currency", currency="GBP"),
            "moja pauni na hamsini peni",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sw", to="year"), "elfu moja")
        self.assertEqual(
            num2words(1066, lang="sw", to="year"), "elfu moja na sitini na sita"
        )
        self.assertEqual(
            num2words(1492, lang="sw", to="year"),
            "elfu moja na mia nne na tisini na mbili",
        )
        self.assertEqual(
            num2words(1776, lang="sw", to="year"),
            "elfu moja na mia saba na sabini na sita",
        )
        self.assertEqual(num2words(1800, lang="sw", to="year"), "elfu moja na mia nane")
        self.assertEqual(num2words(1900, lang="sw", to="year"), "elfu moja na mia tisa")
        self.assertEqual(
            num2words(1984, lang="sw", to="year"),
            "elfu moja na mia tisa na themanini na nne",
        )
        self.assertEqual(
            num2words(1999, lang="sw", to="year"),
            "elfu moja na mia tisa na tisini na tisa",
        )
        self.assertEqual(num2words(2000, lang="sw", to="year"), "elfu mbili")
        self.assertEqual(num2words(2001, lang="sw", to="year"), "elfu mbili na moja")
        self.assertEqual(num2words(2010, lang="sw", to="year"), "elfu mbili na kumi")
        self.assertEqual(
            num2words(2020, lang="sw", to="year"), "elfu mbili na ishirini"
        )
        self.assertEqual(
            num2words(2024, lang="sw", to="year"), "elfu mbili na ishirini na nne"
        )
        self.assertEqual(
            num2words(2100, lang="sw", to="year"), "elfu mbili na mia moja"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sw"), "sifuri")
        self.assertEqual(num2words("1", lang="sw"), "moja")
        self.assertEqual(num2words("10", lang="sw"), "kumi")
        self.assertEqual(num2words("100", lang="sw"), "mia moja")
        self.assertEqual(num2words("1000", lang="sw"), "elfu moja")
        self.assertEqual(num2words("10000", lang="sw"), "elfu kumi")
        self.assertEqual(num2words("100000", lang="sw"), "laki moja")
        self.assertEqual(num2words("1000000", lang="sw"), "milioni moja")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sw"), "sifuri")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sw"), num2words("100", lang="sw"))
        self.assertEqual(num2words(1000, lang="sw"), num2words("1000", lang="sw"))
