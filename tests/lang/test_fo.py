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


class Num2WordsFOTest(TestCase):
    """Comprehensive test cases for Faroese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="fo"), "null")
        self.assertEqual(num2words(1, lang="fo"), "ein")
        self.assertEqual(num2words(2, lang="fo"), "tvey")
        self.assertEqual(num2words(3, lang="fo"), "trý")
        self.assertEqual(num2words(4, lang="fo"), "fýra")
        self.assertEqual(num2words(5, lang="fo"), "fimm")
        self.assertEqual(num2words(6, lang="fo"), "seks")
        self.assertEqual(num2words(7, lang="fo"), "sjey")
        self.assertEqual(num2words(8, lang="fo"), "átta")
        self.assertEqual(num2words(9, lang="fo"), "níggju")
        self.assertEqual(num2words(10, lang="fo"), "tíggju")
        self.assertEqual(num2words(11, lang="fo"), "tíggju ein")
        self.assertEqual(num2words(12, lang="fo"), "tíggju tvey")
        self.assertEqual(num2words(13, lang="fo"), "tíggju trý")
        self.assertEqual(num2words(14, lang="fo"), "tíggju fýra")
        self.assertEqual(num2words(15, lang="fo"), "tíggju fimm")
        self.assertEqual(num2words(16, lang="fo"), "tíggju seks")
        self.assertEqual(num2words(17, lang="fo"), "tíggju sjey")
        self.assertEqual(num2words(18, lang="fo"), "tíggju átta")
        self.assertEqual(num2words(19, lang="fo"), "tíggju níggju")
        self.assertEqual(num2words(20, lang="fo"), "tjúgu")
        self.assertEqual(num2words(21, lang="fo"), "tjúgu ein")
        self.assertEqual(num2words(22, lang="fo"), "tjúgu tvey")
        self.assertEqual(num2words(23, lang="fo"), "tjúgu trý")
        self.assertEqual(num2words(24, lang="fo"), "tjúgu fýra")
        self.assertEqual(num2words(25, lang="fo"), "tjúgu fimm")
        self.assertEqual(num2words(26, lang="fo"), "tjúgu seks")
        self.assertEqual(num2words(27, lang="fo"), "tjúgu sjey")
        self.assertEqual(num2words(28, lang="fo"), "tjúgu átta")
        self.assertEqual(num2words(29, lang="fo"), "tjúgu níggju")
        self.assertEqual(num2words(30, lang="fo"), "tríati")
        self.assertEqual(num2words(31, lang="fo"), "tríati ein")
        self.assertEqual(num2words(35, lang="fo"), "tríati fimm")
        self.assertEqual(num2words(40, lang="fo"), "fjøruti")
        self.assertEqual(num2words(45, lang="fo"), "fjøruti fimm")
        self.assertEqual(num2words(50, lang="fo"), "fimmti")
        self.assertEqual(num2words(55, lang="fo"), "fimmti fimm")
        self.assertEqual(num2words(60, lang="fo"), "seksti")
        self.assertEqual(num2words(65, lang="fo"), "seksti fimm")
        self.assertEqual(num2words(70, lang="fo"), "sjeyti")
        self.assertEqual(num2words(75, lang="fo"), "sjeyti fimm")
        self.assertEqual(num2words(80, lang="fo"), "áttati")
        self.assertEqual(num2words(85, lang="fo"), "áttati fimm")
        self.assertEqual(num2words(90, lang="fo"), "níti")
        self.assertEqual(num2words(95, lang="fo"), "níti fimm")
        self.assertEqual(num2words(99, lang="fo"), "níti níggju")
        self.assertEqual(num2words(100, lang="fo"), "ein hundrað")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="fo"), "ein hundrað ein")
        self.assertEqual(num2words(110, lang="fo"), "ein hundrað tíggju")
        self.assertEqual(num2words(111, lang="fo"), "ein hundrað tíggju ein")
        self.assertEqual(num2words(120, lang="fo"), "ein hundrað tjúgu")
        self.assertEqual(num2words(125, lang="fo"), "ein hundrað tjúgu fimm")
        self.assertEqual(num2words(150, lang="fo"), "ein hundrað fimmti")
        self.assertEqual(num2words(175, lang="fo"), "ein hundrað sjeyti fimm")
        self.assertEqual(num2words(199, lang="fo"), "ein hundrað níti níggju")
        self.assertEqual(num2words(200, lang="fo"), "tvey hundrað")
        self.assertEqual(num2words(201, lang="fo"), "tvey hundrað ein")
        self.assertEqual(num2words(210, lang="fo"), "tvey hundrað tíggju")
        self.assertEqual(num2words(220, lang="fo"), "tvey hundrað tjúgu")
        self.assertEqual(num2words(250, lang="fo"), "tvey hundrað fimmti")
        self.assertEqual(num2words(299, lang="fo"), "tvey hundrað níti níggju")
        self.assertEqual(num2words(300, lang="fo"), "trý hundrað")
        self.assertEqual(num2words(333, lang="fo"), "trý hundrað tríati trý")
        self.assertEqual(num2words(400, lang="fo"), "fýra hundrað")
        self.assertEqual(num2words(444, lang="fo"), "fýra hundrað fjøruti fýra")
        self.assertEqual(num2words(500, lang="fo"), "fimm hundrað")
        self.assertEqual(num2words(555, lang="fo"), "fimm hundrað fimmti fimm")
        self.assertEqual(num2words(600, lang="fo"), "seks hundrað")
        self.assertEqual(num2words(666, lang="fo"), "seks hundrað seksti seks")
        self.assertEqual(num2words(700, lang="fo"), "sjey hundrað")
        self.assertEqual(num2words(777, lang="fo"), "sjey hundrað sjeyti sjey")
        self.assertEqual(num2words(800, lang="fo"), "átta hundrað")
        self.assertEqual(num2words(888, lang="fo"), "átta hundrað áttati átta")
        self.assertEqual(num2words(900, lang="fo"), "níggju hundrað")
        self.assertEqual(num2words(999, lang="fo"), "níggju hundrað níti níggju")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="fo"), "ein túsund")
        self.assertEqual(num2words(1001, lang="fo"), "ein túsund ein")
        self.assertEqual(num2words(1010, lang="fo"), "ein túsund tíggju")
        self.assertEqual(num2words(1100, lang="fo"), "ein túsund ein hundrað")
        self.assertEqual(
            num2words(1111, lang="fo"), "ein túsund ein hundrað tíggju ein"
        )
        self.assertEqual(
            num2words(1234, lang="fo"), "ein túsund tvey hundrað tríati fýra"
        )
        self.assertEqual(num2words(1500, lang="fo"), "ein túsund fimm hundrað")
        self.assertEqual(
            num2words(1999, lang="fo"), "ein túsund níggju hundrað níti níggju"
        )
        self.assertEqual(num2words(2000, lang="fo"), "tvey túsund")
        self.assertEqual(num2words(2001, lang="fo"), "tvey túsund ein")
        self.assertEqual(num2words(2020, lang="fo"), "tvey túsund tjúgu")
        self.assertEqual(
            num2words(2222, lang="fo"), "tvey túsund tvey hundrað tjúgu tvey"
        )
        self.assertEqual(num2words(3000, lang="fo"), "trý túsund")
        self.assertEqual(
            num2words(3333, lang="fo"), "trý túsund trý hundrað tríati trý"
        )
        self.assertEqual(num2words(4000, lang="fo"), "fýra túsund")
        self.assertEqual(
            num2words(4444, lang="fo"), "fýra túsund fýra hundrað fjøruti fýra"
        )
        self.assertEqual(num2words(5000, lang="fo"), "fimm túsund")
        self.assertEqual(
            num2words(5555, lang="fo"), "fimm túsund fimm hundrað fimmti fimm"
        )
        self.assertEqual(num2words(6000, lang="fo"), "seks túsund")
        self.assertEqual(
            num2words(6666, lang="fo"), "seks túsund seks hundrað seksti seks"
        )
        self.assertEqual(num2words(7000, lang="fo"), "sjey túsund")
        self.assertEqual(
            num2words(7777, lang="fo"), "sjey túsund sjey hundrað sjeyti sjey"
        )
        self.assertEqual(num2words(8000, lang="fo"), "átta túsund")
        self.assertEqual(
            num2words(8888, lang="fo"), "átta túsund átta hundrað áttati átta"
        )
        self.assertEqual(num2words(9000, lang="fo"), "níggju túsund")
        self.assertEqual(
            num2words(9999, lang="fo"), "níggju túsund níggju hundrað níti níggju"
        )
        self.assertEqual(num2words(10000, lang="fo"), "tíggju túsund")
        self.assertEqual(num2words(10001, lang="fo"), "tíggju túsund ein")
        self.assertEqual(
            num2words(11111, lang="fo"), "tíggju ein túsund ein hundrað tíggju ein"
        )
        self.assertEqual(
            num2words(12345, lang="fo"), "tíggju tvey túsund trý hundrað fjøruti fimm"
        )
        self.assertEqual(num2words(20000, lang="fo"), "tjúgu túsund")
        self.assertEqual(num2words(50000, lang="fo"), "fimmti túsund")
        self.assertEqual(
            num2words(99999, lang="fo"), "níti níggju túsund níggju hundrað níti níggju"
        )
        self.assertEqual(num2words(100000, lang="fo"), "ein hundrað túsund")
        self.assertEqual(
            num2words(123456, lang="fo"),
            "ein hundrað tjúgu trý túsund fýra hundrað fimmti seks",
        )
        self.assertEqual(num2words(200000, lang="fo"), "tvey hundrað túsund")
        self.assertEqual(num2words(500000, lang="fo"), "fimm hundrað túsund")
        self.assertEqual(
            num2words(654321, lang="fo"),
            "seks hundrað fimmti fýra túsund trý hundrað tjúgu ein",
        )
        self.assertEqual(
            num2words(999999, lang="fo"),
            "níggju hundrað níti níggju túsund níggju hundrað níti níggju",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="fo"), "ein millión")
        self.assertEqual(num2words(1000001, lang="fo"), "ein millión ein")
        self.assertEqual(
            num2words(1111111, lang="fo"),
            "ein millión ein hundrað tíggju ein túsund ein hundrað tíggju ein",
        )
        self.assertEqual(
            num2words(1234567, lang="fo"),
            "ein millión tvey hundrað tríati fýra túsund fimm hundrað seksti sjey",
        )
        self.assertEqual(num2words(2000000, lang="fo"), "tvey millión")
        self.assertEqual(num2words(5000000, lang="fo"), "fimm millión")
        self.assertEqual(
            num2words(9999999, lang="fo"),
            "níggju millión níggju hundrað níti níggju túsund níggju hundrað níti níggju",
        )
        self.assertEqual(num2words(10000000, lang="fo"), "tíggju millión")
        self.assertEqual(
            num2words(12345678, lang="fo"),
            "tíggju tvey millión trý hundrað fjøruti fimm túsund seks hundrað sjeyti átta",
        )
        self.assertEqual(
            num2words(99999999, lang="fo"),
            "níti níggju millión níggju hundrað níti níggju túsund níggju hundrað níti níggju",
        )
        self.assertEqual(num2words(100000000, lang="fo"), "ein hundrað millión")
        self.assertEqual(
            num2words(123456789, lang="fo"),
            "ein hundrað tjúgu trý millión fýra hundrað fimmti seks túsund sjey hundrað áttati níggju",
        )
        self.assertEqual(
            num2words(999999999, lang="fo"),
            "níggju hundrað níti níggju millión níggju hundrað níti níggju túsund níggju hundrað níti níggju",
        )
        self.assertEqual(num2words(1000000000, lang="fo"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="fo"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="fo"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="fo"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="fo"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="fo"), "minus ein")
        self.assertEqual(num2words(-2, lang="fo"), "minus tvey")
        self.assertEqual(num2words(-5, lang="fo"), "minus fimm")
        self.assertEqual(num2words(-10, lang="fo"), "minus tíggju")
        self.assertEqual(num2words(-11, lang="fo"), "minus tíggju ein")
        self.assertEqual(num2words(-20, lang="fo"), "minus tjúgu")
        self.assertEqual(num2words(-50, lang="fo"), "minus fimmti")
        self.assertEqual(num2words(-99, lang="fo"), "minus níti níggju")
        self.assertEqual(num2words(-100, lang="fo"), "minus ein hundrað")
        self.assertEqual(num2words(-101, lang="fo"), "minus ein hundrað ein")
        self.assertEqual(num2words(-200, lang="fo"), "minus tvey hundrað")
        self.assertEqual(num2words(-999, lang="fo"), "minus níggju hundrað níti níggju")
        self.assertEqual(num2words(-1000, lang="fo"), "minus ein túsund")
        self.assertEqual(num2words(-1001, lang="fo"), "minus ein túsund ein")
        self.assertEqual(num2words(-10000, lang="fo"), "minus tíggju túsund")
        self.assertEqual(num2words(-100000, lang="fo"), "minus ein hundrað túsund")
        self.assertEqual(num2words(-1000000, lang="fo"), "minus ein millión")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="fo"), "null komma ein")
        self.assertEqual(num2words(0.5, lang="fo"), "null komma fimm")
        self.assertEqual(num2words(0.9, lang="fo"), "null komma níggju")
        self.assertEqual(num2words(1.1, lang="fo"), "ein komma ein")
        self.assertEqual(num2words(1.5, lang="fo"), "ein komma fimm")
        self.assertEqual(num2words(2.5, lang="fo"), "tvey komma fimm")
        self.assertEqual(num2words(3.14, lang="fo"), "trý komma ein fýra")
        self.assertEqual(num2words(10.5, lang="fo"), "tíggju komma fimm")
        self.assertEqual(num2words(11.11, lang="fo"), "tíggju ein komma ein ein")
        self.assertEqual(num2words(20.2, lang="fo"), "tjúgu komma tvey")
        self.assertEqual(num2words(99.99, lang="fo"), "níti níggju komma níggju níggju")
        self.assertEqual(num2words(100.01, lang="fo"), "ein hundrað komma null ein")
        self.assertEqual(num2words(100.5, lang="fo"), "ein hundrað komma fimm")
        self.assertEqual(
            num2words(123.45, lang="fo"), "ein hundrað tjúgu trý komma fýra fimm"
        )
        self.assertEqual(num2words(1000.5, lang="fo"), "ein túsund komma fimm")
        self.assertEqual(
            num2words(1234.56, lang="fo"),
            "ein túsund tvey hundrað tríati fýra komma fimm seks",
        )
        self.assertEqual(num2words(10000.01, lang="fo"), "tíggju túsund komma null ein")
        self.assertEqual(num2words(-0.5, lang="fo"), "minus null komma fimm")
        self.assertEqual(num2words(-1.5, lang="fo"), "minus ein komma fimm")
        self.assertEqual(num2words(-10.5, lang="fo"), "minus tíggju komma fimm")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="fo", ordinal=True), "ein-ti")
        self.assertEqual(num2words(2, lang="fo", ordinal=True), "tvey-ti")
        self.assertEqual(num2words(3, lang="fo", ordinal=True), "trý-ti")
        self.assertEqual(num2words(4, lang="fo", ordinal=True), "fýra-ti")
        self.assertEqual(num2words(5, lang="fo", ordinal=True), "fimm-ti")
        self.assertEqual(num2words(6, lang="fo", ordinal=True), "seks-ti")
        self.assertEqual(num2words(7, lang="fo", ordinal=True), "sjey-ti")
        self.assertEqual(num2words(8, lang="fo", ordinal=True), "átta-ti")
        self.assertEqual(num2words(9, lang="fo", ordinal=True), "níggju-ti")
        self.assertEqual(num2words(10, lang="fo", ordinal=True), "tíggju-ti")
        self.assertEqual(num2words(11, lang="fo", ordinal=True), "tíggju ein-ti")
        self.assertEqual(num2words(12, lang="fo", ordinal=True), "tíggju tvey-ti")
        self.assertEqual(num2words(13, lang="fo", ordinal=True), "tíggju trý-ti")
        self.assertEqual(num2words(14, lang="fo", ordinal=True), "tíggju fýra-ti")
        self.assertEqual(num2words(15, lang="fo", ordinal=True), "tíggju fimm-ti")
        self.assertEqual(num2words(16, lang="fo", ordinal=True), "tíggju seks-ti")
        self.assertEqual(num2words(17, lang="fo", ordinal=True), "tíggju sjey-ti")
        self.assertEqual(num2words(18, lang="fo", ordinal=True), "tíggju átta-ti")
        self.assertEqual(num2words(19, lang="fo", ordinal=True), "tíggju níggju-ti")
        self.assertEqual(num2words(20, lang="fo", ordinal=True), "tjúgu-ti")
        self.assertEqual(num2words(21, lang="fo", ordinal=True), "tjúgu ein-ti")
        self.assertEqual(num2words(22, lang="fo", ordinal=True), "tjúgu tvey-ti")
        self.assertEqual(num2words(25, lang="fo", ordinal=True), "tjúgu fimm-ti")
        self.assertEqual(num2words(30, lang="fo", ordinal=True), "tríati-ti")
        self.assertEqual(num2words(40, lang="fo", ordinal=True), "fjøruti-ti")
        self.assertEqual(num2words(50, lang="fo", ordinal=True), "fimmti-ti")
        self.assertEqual(num2words(60, lang="fo", ordinal=True), "seksti-ti")
        self.assertEqual(num2words(70, lang="fo", ordinal=True), "sjeyti-ti")
        self.assertEqual(num2words(80, lang="fo", ordinal=True), "áttati-ti")
        self.assertEqual(num2words(90, lang="fo", ordinal=True), "níti-ti")
        self.assertEqual(num2words(100, lang="fo", ordinal=True), "ein hundrað-ti")
        self.assertEqual(num2words(101, lang="fo", ordinal=True), "ein hundrað ein-ti")
        self.assertEqual(num2words(200, lang="fo", ordinal=True), "tvey hundrað-ti")
        self.assertEqual(num2words(500, lang="fo", ordinal=True), "fimm hundrað-ti")
        self.assertEqual(num2words(1000, lang="fo", ordinal=True), "ein túsund-ti")
        self.assertEqual(num2words(1001, lang="fo", ordinal=True), "ein túsund ein-ti")
        self.assertEqual(num2words(10000, lang="fo", ordinal=True), "tíggju túsund-ti")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="fo", to="currency", currency="DKK"), "null krónur"
        )
        self.assertEqual(
            num2words(0.01, lang="fo", to="currency", currency="DKK"),
            "null krónur ein oyra",
        )
        self.assertEqual(
            num2words(0.5, lang="fo", to="currency", currency="DKK"),
            "null krónur fimmti oyru",
        )
        self.assertEqual(
            num2words(1, lang="fo", to="currency", currency="DKK"), "ein króna"
        )
        self.assertEqual(
            num2words(1.5, lang="fo", to="currency", currency="DKK"),
            "ein króna fimmti oyru",
        )
        self.assertEqual(
            num2words(0, lang="fo", to="currency", currency="USD"), "null dollarar"
        )
        self.assertEqual(
            num2words(0.01, lang="fo", to="currency", currency="USD"),
            "null dollarar ein sent",
        )
        self.assertEqual(
            num2words(0.5, lang="fo", to="currency", currency="USD"),
            "null dollarar fimmti sent",
        )
        self.assertEqual(
            num2words(1, lang="fo", to="currency", currency="USD"), "ein dollari"
        )
        self.assertEqual(
            num2words(1.5, lang="fo", to="currency", currency="USD"),
            "ein dollari fimmti sent",
        )
        self.assertEqual(
            num2words(0, lang="fo", to="currency", currency="EUR"), "null evrur"
        )
        self.assertEqual(
            num2words(0.01, lang="fo", to="currency", currency="EUR"),
            "null evrur ein sent",
        )
        self.assertEqual(
            num2words(0.5, lang="fo", to="currency", currency="EUR"),
            "null evrur fimmti sent",
        )
        self.assertEqual(
            num2words(1, lang="fo", to="currency", currency="EUR"), "ein evra"
        )
        self.assertEqual(
            num2words(1.5, lang="fo", to="currency", currency="EUR"),
            "ein evra fimmti sent",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="fo", to="year"), "ein túsund")
        self.assertEqual(
            num2words(1066, lang="fo", to="year"), "ein túsund seksti seks"
        )
        self.assertEqual(
            num2words(1492, lang="fo", to="year"), "ein túsund fýra hundrað níti tvey"
        )
        self.assertEqual(
            num2words(1776, lang="fo", to="year"), "ein túsund sjey hundrað sjeyti seks"
        )
        self.assertEqual(
            num2words(1800, lang="fo", to="year"), "ein túsund átta hundrað"
        )
        self.assertEqual(
            num2words(1900, lang="fo", to="year"), "ein túsund níggju hundrað"
        )
        self.assertEqual(
            num2words(1984, lang="fo", to="year"),
            "ein túsund níggju hundrað áttati fýra",
        )
        self.assertEqual(
            num2words(1999, lang="fo", to="year"),
            "ein túsund níggju hundrað níti níggju",
        )
        self.assertEqual(num2words(2000, lang="fo", to="year"), "tvey túsund")
        self.assertEqual(num2words(2001, lang="fo", to="year"), "tvey túsund ein")
        self.assertEqual(num2words(2010, lang="fo", to="year"), "tvey túsund tíggju")
        self.assertEqual(num2words(2020, lang="fo", to="year"), "tvey túsund tjúgu")
        self.assertEqual(
            num2words(2024, lang="fo", to="year"), "tvey túsund tjúgu fýra"
        )
        self.assertEqual(
            num2words(2100, lang="fo", to="year"), "tvey túsund ein hundrað"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="fo"), "null")
        self.assertEqual(num2words("1", lang="fo"), "ein")
        self.assertEqual(num2words("10", lang="fo"), "tíggju")
        self.assertEqual(num2words("100", lang="fo"), "ein hundrað")
        self.assertEqual(num2words("1000", lang="fo"), "ein túsund")
        self.assertEqual(num2words("10000", lang="fo"), "tíggju túsund")
        self.assertEqual(num2words("100000", lang="fo"), "ein hundrað túsund")
        self.assertEqual(num2words("1000000", lang="fo"), "ein millión")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="fo"), "null")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="fo"), num2words("100", lang="fo"))
        self.assertEqual(num2words(1000, lang="fo"), num2words("1000", lang="fo"))

