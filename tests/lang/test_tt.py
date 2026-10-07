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


class Num2WordsTTTest(TestCase):
    """Comprehensive test cases for Tatar language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="tt"), "нуль")
        self.assertEqual(num2words(1, lang="tt"), "бер")
        self.assertEqual(num2words(2, lang="tt"), "ике")
        self.assertEqual(num2words(3, lang="tt"), "өч")
        self.assertEqual(num2words(4, lang="tt"), "дүрт")
        self.assertEqual(num2words(5, lang="tt"), "биш")
        self.assertEqual(num2words(6, lang="tt"), "алты")
        self.assertEqual(num2words(7, lang="tt"), "җиде")
        self.assertEqual(num2words(8, lang="tt"), "сигез")
        self.assertEqual(num2words(9, lang="tt"), "тугыз")
        self.assertEqual(num2words(10, lang="tt"), "ун")
        self.assertEqual(num2words(11, lang="tt"), "ун бер")
        self.assertEqual(num2words(12, lang="tt"), "ун ике")
        self.assertEqual(num2words(13, lang="tt"), "ун өч")
        self.assertEqual(num2words(14, lang="tt"), "ун дүрт")
        self.assertEqual(num2words(15, lang="tt"), "ун биш")
        self.assertEqual(num2words(16, lang="tt"), "ун алты")
        self.assertEqual(num2words(17, lang="tt"), "ун җиде")
        self.assertEqual(num2words(18, lang="tt"), "ун сигез")
        self.assertEqual(num2words(19, lang="tt"), "ун тугыз")
        self.assertEqual(num2words(20, lang="tt"), "егерме")
        self.assertEqual(num2words(21, lang="tt"), "егерме бер")
        self.assertEqual(num2words(22, lang="tt"), "егерме ике")
        self.assertEqual(num2words(23, lang="tt"), "егерме өч")
        self.assertEqual(num2words(24, lang="tt"), "егерме дүрт")
        self.assertEqual(num2words(25, lang="tt"), "егерме биш")
        self.assertEqual(num2words(26, lang="tt"), "егерме алты")
        self.assertEqual(num2words(27, lang="tt"), "егерме җиде")
        self.assertEqual(num2words(28, lang="tt"), "егерме сигез")
        self.assertEqual(num2words(29, lang="tt"), "егерме тугыз")
        self.assertEqual(num2words(30, lang="tt"), "утыз")
        self.assertEqual(num2words(31, lang="tt"), "утыз бер")
        self.assertEqual(num2words(35, lang="tt"), "утыз биш")
        self.assertEqual(num2words(40, lang="tt"), "кырык")
        self.assertEqual(num2words(45, lang="tt"), "кырык биш")
        self.assertEqual(num2words(50, lang="tt"), "илле")
        self.assertEqual(num2words(55, lang="tt"), "илле биш")
        self.assertEqual(num2words(60, lang="tt"), "алтмыш")
        self.assertEqual(num2words(65, lang="tt"), "алтмыш биш")
        self.assertEqual(num2words(70, lang="tt"), "җитмеш")
        self.assertEqual(num2words(75, lang="tt"), "җитмеш биш")
        self.assertEqual(num2words(80, lang="tt"), "сиксән")
        self.assertEqual(num2words(85, lang="tt"), "сиксән биш")
        self.assertEqual(num2words(90, lang="tt"), "туксан")
        self.assertEqual(num2words(95, lang="tt"), "туксан биш")
        self.assertEqual(num2words(99, lang="tt"), "туксан тугыз")
        self.assertEqual(num2words(100, lang="tt"), "бер йөз")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="tt"), "бер йөз бер")
        self.assertEqual(num2words(110, lang="tt"), "бер йөз ун")
        self.assertEqual(num2words(111, lang="tt"), "бер йөз ун бер")
        self.assertEqual(num2words(120, lang="tt"), "бер йөз егерме")
        self.assertEqual(num2words(125, lang="tt"), "бер йөз егерме биш")
        self.assertEqual(num2words(150, lang="tt"), "бер йөз илле")
        self.assertEqual(num2words(175, lang="tt"), "бер йөз җитмеш биш")
        self.assertEqual(num2words(199, lang="tt"), "бер йөз туксан тугыз")
        self.assertEqual(num2words(200, lang="tt"), "ике йөз")
        self.assertEqual(num2words(201, lang="tt"), "ике йөз бер")
        self.assertEqual(num2words(210, lang="tt"), "ике йөз ун")
        self.assertEqual(num2words(220, lang="tt"), "ике йөз егерме")
        self.assertEqual(num2words(250, lang="tt"), "ике йөз илле")
        self.assertEqual(num2words(299, lang="tt"), "ике йөз туксан тугыз")
        self.assertEqual(num2words(300, lang="tt"), "өч йөз")
        self.assertEqual(num2words(333, lang="tt"), "өч йөз утыз өч")
        self.assertEqual(num2words(400, lang="tt"), "дүрт йөз")
        self.assertEqual(num2words(444, lang="tt"), "дүрт йөз кырык дүрт")
        self.assertEqual(num2words(500, lang="tt"), "биш йөз")
        self.assertEqual(num2words(555, lang="tt"), "биш йөз илле биш")
        self.assertEqual(num2words(600, lang="tt"), "алты йөз")
        self.assertEqual(num2words(666, lang="tt"), "алты йөз алтмыш алты")
        self.assertEqual(num2words(700, lang="tt"), "җиде йөз")
        self.assertEqual(num2words(777, lang="tt"), "җиде йөз җитмеш җиде")
        self.assertEqual(num2words(800, lang="tt"), "сигез йөз")
        self.assertEqual(num2words(888, lang="tt"), "сигез йөз сиксән сигез")
        self.assertEqual(num2words(900, lang="tt"), "тугыз йөз")
        self.assertEqual(num2words(999, lang="tt"), "тугыз йөз туксан тугыз")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="tt"), "бер мең")
        self.assertEqual(num2words(1001, lang="tt"), "бер мең бер")
        self.assertEqual(num2words(1010, lang="tt"), "бер мең ун")
        self.assertEqual(num2words(1100, lang="tt"), "бер мең бер йөз")
        self.assertEqual(num2words(1111, lang="tt"), "бер мең бер йөз ун бер")
        self.assertEqual(num2words(1234, lang="tt"), "бер мең ике йөз утыз дүрт")
        self.assertEqual(num2words(1500, lang="tt"), "бер мең биш йөз")
        self.assertEqual(num2words(1999, lang="tt"), "бер мең тугыз йөз туксан тугыз")
        self.assertEqual(num2words(2000, lang="tt"), "ике мең")
        self.assertEqual(num2words(2001, lang="tt"), "ике мең бер")
        self.assertEqual(num2words(2020, lang="tt"), "ике мең егерме")
        self.assertEqual(num2words(2222, lang="tt"), "ике мең ике йөз егерме ике")
        self.assertEqual(num2words(3000, lang="tt"), "өч мең")
        self.assertEqual(num2words(3333, lang="tt"), "өч мең өч йөз утыз өч")
        self.assertEqual(num2words(4000, lang="tt"), "дүрт мең")
        self.assertEqual(num2words(4444, lang="tt"), "дүрт мең дүрт йөз кырык дүрт")
        self.assertEqual(num2words(5000, lang="tt"), "биш мең")
        self.assertEqual(num2words(5555, lang="tt"), "биш мең биш йөз илле биш")
        self.assertEqual(num2words(6000, lang="tt"), "алты мең")
        self.assertEqual(num2words(6666, lang="tt"), "алты мең алты йөз алтмыш алты")
        self.assertEqual(num2words(7000, lang="tt"), "җиде мең")
        self.assertEqual(num2words(7777, lang="tt"), "җиде мең җиде йөз җитмеш җиде")
        self.assertEqual(num2words(8000, lang="tt"), "сигез мең")
        self.assertEqual(num2words(8888, lang="tt"), "сигез мең сигез йөз сиксән сигез")
        self.assertEqual(num2words(9000, lang="tt"), "тугыз мең")
        self.assertEqual(num2words(9999, lang="tt"), "тугыз мең тугыз йөз туксан тугыз")
        self.assertEqual(num2words(10000, lang="tt"), "ун мең")
        self.assertEqual(num2words(10001, lang="tt"), "ун мең бер")
        self.assertEqual(num2words(11111, lang="tt"), "ун бер мең бер йөз ун бер")
        self.assertEqual(num2words(12345, lang="tt"), "ун ике мең өч йөз кырык биш")
        self.assertEqual(num2words(20000, lang="tt"), "егерме мең")
        self.assertEqual(num2words(50000, lang="tt"), "илле мең")
        self.assertEqual(
            num2words(99999, lang="tt"), "туксан тугыз мең тугыз йөз туксан тугыз"
        )
        self.assertEqual(num2words(100000, lang="tt"), "бер йөз мең")
        self.assertEqual(
            num2words(123456, lang="tt"), "бер йөз егерме өч мең дүрт йөз илле алты"
        )
        self.assertEqual(num2words(200000, lang="tt"), "ике йөз мең")
        self.assertEqual(num2words(500000, lang="tt"), "биш йөз мең")
        self.assertEqual(
            num2words(654321, lang="tt"), "алты йөз илле дүрт мең өч йөз егерме бер"
        )
        self.assertEqual(
            num2words(999999, lang="tt"),
            "тугыз йөз туксан тугыз мең тугыз йөз туксан тугыз",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="tt"), "бер миллион")
        self.assertEqual(num2words(1000001, lang="tt"), "бер миллион бер")
        self.assertEqual(
            num2words(1111111, lang="tt"),
            "бер миллион бер йөз ун бер мең бер йөз ун бер",
        )
        self.assertEqual(
            num2words(1234567, lang="tt"),
            "бер миллион ике йөз утыз дүрт мең биш йөз алтмыш җиде",
        )
        self.assertEqual(num2words(2000000, lang="tt"), "ике миллион")
        self.assertEqual(num2words(5000000, lang="tt"), "биш миллион")
        self.assertEqual(
            num2words(9999999, lang="tt"),
            "тугыз миллион тугыз йөз туксан тугыз мең тугыз йөз туксан тугыз",
        )
        self.assertEqual(num2words(10000000, lang="tt"), "ун миллион")
        self.assertEqual(
            num2words(12345678, lang="tt"),
            "ун ике миллион өч йөз кырык биш мең алты йөз җитмеш сигез",
        )
        self.assertEqual(
            num2words(99999999, lang="tt"),
            "туксан тугыз миллион тугыз йөз туксан тугыз мең тугыз йөз туксан тугыз",
        )
        self.assertEqual(num2words(100000000, lang="tt"), "бер йөз миллион")
        self.assertEqual(
            num2words(123456789, lang="tt"),
            "бер йөз егерме өч миллион дүрт йөз илле алты мең җиде йөз сиксән тугыз",
        )
        self.assertEqual(
            num2words(999999999, lang="tt"),
            "тугыз йөз туксан тугыз миллион тугыз йөз туксан тугыз мең тугыз йөз туксан тугыз",
        )
        self.assertEqual(num2words(1000000000, lang="tt"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="tt"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="tt"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="tt"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="tt"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="tt"), "минус бер")
        self.assertEqual(num2words(-2, lang="tt"), "минус ике")
        self.assertEqual(num2words(-5, lang="tt"), "минус биш")
        self.assertEqual(num2words(-10, lang="tt"), "минус ун")
        self.assertEqual(num2words(-11, lang="tt"), "минус ун бер")
        self.assertEqual(num2words(-20, lang="tt"), "минус егерме")
        self.assertEqual(num2words(-50, lang="tt"), "минус илле")
        self.assertEqual(num2words(-99, lang="tt"), "минус туксан тугыз")
        self.assertEqual(num2words(-100, lang="tt"), "минус бер йөз")
        self.assertEqual(num2words(-101, lang="tt"), "минус бер йөз бер")
        self.assertEqual(num2words(-200, lang="tt"), "минус ике йөз")
        self.assertEqual(num2words(-999, lang="tt"), "минус тугыз йөз туксан тугыз")
        self.assertEqual(num2words(-1000, lang="tt"), "минус бер мең")
        self.assertEqual(num2words(-1001, lang="tt"), "минус бер мең бер")
        self.assertEqual(num2words(-10000, lang="tt"), "минус ун мең")
        self.assertEqual(num2words(-100000, lang="tt"), "минус бер йөз мең")
        self.assertEqual(num2words(-1000000, lang="tt"), "минус бер миллион")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="tt"), "нуль өтер бер")
        self.assertEqual(num2words(0.5, lang="tt"), "нуль өтер биш")
        self.assertEqual(num2words(0.9, lang="tt"), "нуль өтер тугыз")
        self.assertEqual(num2words(1.1, lang="tt"), "бер өтер бер")
        self.assertEqual(num2words(1.5, lang="tt"), "бер өтер биш")
        self.assertEqual(num2words(2.5, lang="tt"), "ике өтер биш")
        self.assertEqual(num2words(3.14, lang="tt"), "өч өтер бер дүрт")
        self.assertEqual(num2words(10.5, lang="tt"), "ун өтер биш")
        self.assertEqual(num2words(11.11, lang="tt"), "ун бер өтер бер бер")
        self.assertEqual(num2words(20.2, lang="tt"), "егерме өтер ике")
        self.assertEqual(num2words(99.99, lang="tt"), "туксан тугыз өтер тугыз тугыз")
        self.assertEqual(num2words(100.01, lang="tt"), "бер йөз өтер нуль бер")
        self.assertEqual(num2words(100.5, lang="tt"), "бер йөз өтер биш")
        self.assertEqual(
            num2words(123.45, lang="tt"), "бер йөз егерме өч өтер дүрт биш"
        )
        self.assertEqual(num2words(1000.5, lang="tt"), "бер мең өтер биш")
        self.assertEqual(
            num2words(1234.56, lang="tt"), "бер мең ике йөз утыз дүрт өтер биш алты"
        )
        self.assertEqual(num2words(10000.01, lang="tt"), "ун мең өтер нуль бер")
        self.assertEqual(num2words(-0.5, lang="tt"), "минус нуль өтер биш")
        self.assertEqual(num2words(-1.5, lang="tt"), "минус бер өтер биш")
        self.assertEqual(num2words(-10.5, lang="tt"), "минус ун өтер биш")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="tt", ordinal=True), "беренче")
        self.assertEqual(num2words(2, lang="tt", ordinal=True), "икенче")
        self.assertEqual(num2words(3, lang="tt", ordinal=True), "өченче")
        self.assertEqual(num2words(4, lang="tt", ordinal=True), "дүртенче")
        self.assertEqual(num2words(5, lang="tt", ordinal=True), "бишенче")
        self.assertEqual(num2words(6, lang="tt", ordinal=True), "алтынчы")
        self.assertEqual(num2words(7, lang="tt", ordinal=True), "җиденче")
        self.assertEqual(num2words(8, lang="tt", ordinal=True), "сигезенче")
        self.assertEqual(num2words(9, lang="tt", ordinal=True), "тугызынчы")
        self.assertEqual(num2words(10, lang="tt", ordinal=True), "унынчы")
        self.assertEqual(num2words(11, lang="tt", ordinal=True), "ун беренче")
        self.assertEqual(num2words(12, lang="tt", ordinal=True), "ун икенче")
        self.assertEqual(num2words(13, lang="tt", ordinal=True), "ун өченче")
        self.assertEqual(num2words(14, lang="tt", ordinal=True), "ун дүртенче")
        self.assertEqual(num2words(15, lang="tt", ordinal=True), "ун бишенче")
        self.assertEqual(num2words(16, lang="tt", ordinal=True), "ун алтынчы")
        self.assertEqual(num2words(17, lang="tt", ordinal=True), "ун җиденче")
        self.assertEqual(num2words(18, lang="tt", ordinal=True), "ун сигезенче")
        self.assertEqual(num2words(19, lang="tt", ordinal=True), "ун тугызынчы")
        self.assertEqual(num2words(20, lang="tt", ordinal=True), "егерменче")
        self.assertEqual(num2words(21, lang="tt", ordinal=True), "егерме беренче")
        self.assertEqual(num2words(22, lang="tt", ordinal=True), "егерме икенче")
        self.assertEqual(num2words(25, lang="tt", ordinal=True), "егерме бишенче")
        self.assertEqual(num2words(30, lang="tt", ordinal=True), "утызынчы")
        self.assertEqual(num2words(40, lang="tt", ordinal=True), "кырыгынчы")
        self.assertEqual(num2words(50, lang="tt", ordinal=True), "илленче")
        self.assertEqual(num2words(60, lang="tt", ordinal=True), "алтмышынчы")
        self.assertEqual(num2words(70, lang="tt", ordinal=True), "җитмешенче")
        self.assertEqual(num2words(80, lang="tt", ordinal=True), "сиксәненче")
        self.assertEqual(num2words(90, lang="tt", ordinal=True), "туксанынчы")
        self.assertEqual(num2words(100, lang="tt", ordinal=True), "бер йөзенче")
        self.assertEqual(num2words(101, lang="tt", ordinal=True), "бер йөз беренче")
        self.assertEqual(num2words(200, lang="tt", ordinal=True), "ике йөзенче")
        self.assertEqual(num2words(500, lang="tt", ordinal=True), "биш йөзенче")
        self.assertEqual(num2words(1000, lang="tt", ordinal=True), "бер меңенче")
        self.assertEqual(num2words(1001, lang="tt", ordinal=True), "бер мең беренче")
        self.assertEqual(num2words(10000, lang="tt", ordinal=True), "ун меңенче")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="tt", to="currency", currency="RUB"), "нуль сум"
        )
        self.assertEqual(
            num2words(0.01, lang="tt", to="currency", currency="RUB"),
            "нуль сум бер тиен",
        )
        self.assertEqual(
            num2words(0.5, lang="tt", to="currency", currency="RUB"),
            "нуль сум илле тиен",
        )
        self.assertEqual(
            num2words(1, lang="tt", to="currency", currency="RUB"), "бер сум"
        )
        self.assertEqual(
            num2words(1.5, lang="tt", to="currency", currency="RUB"),
            "бер сум илле тиен",
        )
        self.assertEqual(
            num2words(0, lang="tt", to="currency", currency="USD"), "нуль доллар"
        )
        self.assertEqual(
            num2words(0.01, lang="tt", to="currency", currency="USD"),
            "нуль доллар бер цент",
        )
        self.assertEqual(
            num2words(0.5, lang="tt", to="currency", currency="USD"),
            "нуль доллар илле цент",
        )
        self.assertEqual(
            num2words(1, lang="tt", to="currency", currency="USD"), "бер доллар"
        )
        self.assertEqual(
            num2words(1.5, lang="tt", to="currency", currency="USD"),
            "бер доллар илле цент",
        )
        self.assertEqual(
            num2words(0, lang="tt", to="currency", currency="EUR"), "нуль евро"
        )
        self.assertEqual(
            num2words(0.01, lang="tt", to="currency", currency="EUR"),
            "нуль евро бер цент",
        )
        self.assertEqual(
            num2words(0.5, lang="tt", to="currency", currency="EUR"),
            "нуль евро илле цент",
        )
        self.assertEqual(
            num2words(1, lang="tt", to="currency", currency="EUR"), "бер евро"
        )
        self.assertEqual(
            num2words(1.5, lang="tt", to="currency", currency="EUR"),
            "бер евро илле цент",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="tt", to="year"), "бер мең")
        self.assertEqual(num2words(1066, lang="tt", to="year"), "бер мең алтмыш алты")
        self.assertEqual(
            num2words(1492, lang="tt", to="year"), "бер мең дүрт йөз туксан ике"
        )
        self.assertEqual(
            num2words(1776, lang="tt", to="year"), "бер мең җиде йөз җитмеш алты"
        )
        self.assertEqual(num2words(1800, lang="tt", to="year"), "бер мең сигез йөз")
        self.assertEqual(num2words(1900, lang="tt", to="year"), "бер мең тугыз йөз")
        self.assertEqual(
            num2words(1984, lang="tt", to="year"), "бер мең тугыз йөз сиксән дүрт"
        )
        self.assertEqual(
            num2words(1999, lang="tt", to="year"), "бер мең тугыз йөз туксан тугыз"
        )
        self.assertEqual(num2words(2000, lang="tt", to="year"), "ике мең")
        self.assertEqual(num2words(2001, lang="tt", to="year"), "ике мең бер")
        self.assertEqual(num2words(2010, lang="tt", to="year"), "ике мең ун")
        self.assertEqual(num2words(2020, lang="tt", to="year"), "ике мең егерме")
        self.assertEqual(num2words(2024, lang="tt", to="year"), "ике мең егерме дүрт")
        self.assertEqual(num2words(2100, lang="tt", to="year"), "ике мең бер йөз")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="tt"), "нуль")
        self.assertEqual(num2words("1", lang="tt"), "бер")
        self.assertEqual(num2words("10", lang="tt"), "ун")
        self.assertEqual(num2words("100", lang="tt"), "бер йөз")
        self.assertEqual(num2words("1000", lang="tt"), "бер мең")
        self.assertEqual(num2words("10000", lang="tt"), "ун мең")
        self.assertEqual(num2words("100000", lang="tt"), "бер йөз мең")
        self.assertEqual(num2words("1000000", lang="tt"), "бер миллион")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="tt"), "нуль")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="tt"), num2words("100", lang="tt"))
        self.assertEqual(num2words(1000, lang="tt"), num2words("1000", lang="tt"))

