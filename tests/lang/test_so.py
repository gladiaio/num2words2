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


class Num2WordsSOTest(TestCase):
    """Comprehensive test cases for Somali language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="so"), "eber")
        self.assertEqual(num2words(1, lang="so"), "kow")
        self.assertEqual(num2words(2, lang="so"), "laba")
        self.assertEqual(num2words(3, lang="so"), "saddex")
        self.assertEqual(num2words(4, lang="so"), "afar")
        self.assertEqual(num2words(5, lang="so"), "shan")
        self.assertEqual(num2words(6, lang="so"), "lix")
        self.assertEqual(num2words(7, lang="so"), "toddoba")
        self.assertEqual(num2words(8, lang="so"), "siddeed")
        self.assertEqual(num2words(9, lang="so"), "sagaal")
        self.assertEqual(num2words(10, lang="so"), "toban")
        self.assertEqual(num2words(11, lang="so"), "toban kow")
        self.assertEqual(num2words(12, lang="so"), "toban laba")
        self.assertEqual(num2words(13, lang="so"), "toban saddex")
        self.assertEqual(num2words(14, lang="so"), "toban afar")
        self.assertEqual(num2words(15, lang="so"), "toban shan")
        self.assertEqual(num2words(16, lang="so"), "toban lix")
        self.assertEqual(num2words(17, lang="so"), "toban toddoba")
        self.assertEqual(num2words(18, lang="so"), "toban siddeed")
        self.assertEqual(num2words(19, lang="so"), "toban sagaal")
        self.assertEqual(num2words(20, lang="so"), "labaatan")
        self.assertEqual(num2words(21, lang="so"), "labaatan kow")
        self.assertEqual(num2words(22, lang="so"), "labaatan laba")
        self.assertEqual(num2words(23, lang="so"), "labaatan saddex")
        self.assertEqual(num2words(24, lang="so"), "labaatan afar")
        self.assertEqual(num2words(25, lang="so"), "labaatan shan")
        self.assertEqual(num2words(26, lang="so"), "labaatan lix")
        self.assertEqual(num2words(27, lang="so"), "labaatan toddoba")
        self.assertEqual(num2words(28, lang="so"), "labaatan siddeed")
        self.assertEqual(num2words(29, lang="so"), "labaatan sagaal")
        self.assertEqual(num2words(30, lang="so"), "soddon")
        self.assertEqual(num2words(31, lang="so"), "soddon kow")
        self.assertEqual(num2words(35, lang="so"), "soddon shan")
        self.assertEqual(num2words(40, lang="so"), "afartan")
        self.assertEqual(num2words(45, lang="so"), "afartan shan")
        self.assertEqual(num2words(50, lang="so"), "konton")
        self.assertEqual(num2words(55, lang="so"), "konton shan")
        self.assertEqual(num2words(60, lang="so"), "lixdan")
        self.assertEqual(num2words(65, lang="so"), "lixdan shan")
        self.assertEqual(num2words(70, lang="so"), "toddobaatan")
        self.assertEqual(num2words(75, lang="so"), "toddobaatan shan")
        self.assertEqual(num2words(80, lang="so"), "siddeetan")
        self.assertEqual(num2words(85, lang="so"), "siddeetan shan")
        self.assertEqual(num2words(90, lang="so"), "sagaashan")
        self.assertEqual(num2words(95, lang="so"), "sagaashan shan")
        self.assertEqual(num2words(99, lang="so"), "sagaashan sagaal")
        self.assertEqual(num2words(100, lang="so"), "kow boqol")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="so"), "kow boqol kow")
        self.assertEqual(num2words(110, lang="so"), "kow boqol toban")
        self.assertEqual(num2words(111, lang="so"), "kow boqol toban kow")
        self.assertEqual(num2words(120, lang="so"), "kow boqol labaatan")
        self.assertEqual(num2words(125, lang="so"), "kow boqol labaatan shan")
        self.assertEqual(num2words(150, lang="so"), "kow boqol konton")
        self.assertEqual(num2words(175, lang="so"), "kow boqol toddobaatan shan")
        self.assertEqual(num2words(199, lang="so"), "kow boqol sagaashan sagaal")
        self.assertEqual(num2words(200, lang="so"), "laba boqol")
        self.assertEqual(num2words(201, lang="so"), "laba boqol kow")
        self.assertEqual(num2words(210, lang="so"), "laba boqol toban")
        self.assertEqual(num2words(220, lang="so"), "laba boqol labaatan")
        self.assertEqual(num2words(250, lang="so"), "laba boqol konton")
        self.assertEqual(num2words(299, lang="so"), "laba boqol sagaashan sagaal")
        self.assertEqual(num2words(300, lang="so"), "saddex boqol")
        self.assertEqual(num2words(333, lang="so"), "saddex boqol soddon saddex")
        self.assertEqual(num2words(400, lang="so"), "afar boqol")
        self.assertEqual(num2words(444, lang="so"), "afar boqol afartan afar")
        self.assertEqual(num2words(500, lang="so"), "shan boqol")
        self.assertEqual(num2words(555, lang="so"), "shan boqol konton shan")
        self.assertEqual(num2words(600, lang="so"), "lix boqol")
        self.assertEqual(num2words(666, lang="so"), "lix boqol lixdan lix")
        self.assertEqual(num2words(700, lang="so"), "toddoba boqol")
        self.assertEqual(num2words(777, lang="so"), "toddoba boqol toddobaatan toddoba")
        self.assertEqual(num2words(800, lang="so"), "siddeed boqol")
        self.assertEqual(num2words(888, lang="so"), "siddeed boqol siddeetan siddeed")
        self.assertEqual(num2words(900, lang="so"), "sagaal boqol")
        self.assertEqual(num2words(999, lang="so"), "sagaal boqol sagaashan sagaal")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="so"), "kow kun")
        self.assertEqual(num2words(1001, lang="so"), "kow kun kow")
        self.assertEqual(num2words(1010, lang="so"), "kow kun toban")
        self.assertEqual(num2words(1100, lang="so"), "kow kun kow boqol")
        self.assertEqual(num2words(1111, lang="so"), "kow kun kow boqol toban kow")
        self.assertEqual(num2words(1234, lang="so"), "kow kun laba boqol soddon afar")
        self.assertEqual(num2words(1500, lang="so"), "kow kun shan boqol")
        self.assertEqual(
            num2words(1999, lang="so"), "kow kun sagaal boqol sagaashan sagaal"
        )
        self.assertEqual(num2words(2000, lang="so"), "laba kun")
        self.assertEqual(num2words(2001, lang="so"), "laba kun kow")
        self.assertEqual(num2words(2020, lang="so"), "laba kun labaatan")
        self.assertEqual(
            num2words(2222, lang="so"), "laba kun laba boqol labaatan laba"
        )
        self.assertEqual(num2words(3000, lang="so"), "saddex kun")
        self.assertEqual(
            num2words(3333, lang="so"), "saddex kun saddex boqol soddon saddex"
        )
        self.assertEqual(num2words(4000, lang="so"), "afar kun")
        self.assertEqual(num2words(4444, lang="so"), "afar kun afar boqol afartan afar")
        self.assertEqual(num2words(5000, lang="so"), "shan kun")
        self.assertEqual(num2words(5555, lang="so"), "shan kun shan boqol konton shan")
        self.assertEqual(num2words(6000, lang="so"), "lix kun")
        self.assertEqual(num2words(6666, lang="so"), "lix kun lix boqol lixdan lix")
        self.assertEqual(num2words(7000, lang="so"), "toddoba kun")
        self.assertEqual(
            num2words(7777, lang="so"), "toddoba kun toddoba boqol toddobaatan toddoba"
        )
        self.assertEqual(num2words(8000, lang="so"), "siddeed kun")
        self.assertEqual(
            num2words(8888, lang="so"), "siddeed kun siddeed boqol siddeetan siddeed"
        )
        self.assertEqual(num2words(9000, lang="so"), "sagaal kun")
        self.assertEqual(
            num2words(9999, lang="so"), "sagaal kun sagaal boqol sagaashan sagaal"
        )
        self.assertEqual(num2words(10000, lang="so"), "toban kun")
        self.assertEqual(num2words(10001, lang="so"), "toban kun kow")
        self.assertEqual(
            num2words(11111, lang="so"), "toban kow kun kow boqol toban kow"
        )
        self.assertEqual(
            num2words(12345, lang="so"), "toban laba kun saddex boqol afartan shan"
        )
        self.assertEqual(num2words(20000, lang="so"), "labaatan kun")
        self.assertEqual(num2words(50000, lang="so"), "konton kun")
        self.assertEqual(
            num2words(99999, lang="so"),
            "sagaashan sagaal kun sagaal boqol sagaashan sagaal",
        )
        self.assertEqual(num2words(100000, lang="so"), "kow boqol kun")
        self.assertEqual(
            num2words(123456, lang="so"),
            "kow boqol labaatan saddex kun afar boqol konton lix",
        )
        self.assertEqual(num2words(200000, lang="so"), "laba boqol kun")
        self.assertEqual(num2words(500000, lang="so"), "shan boqol kun")
        self.assertEqual(
            num2words(654321, lang="so"),
            "lix boqol konton afar kun saddex boqol labaatan kow",
        )
        self.assertEqual(
            num2words(999999, lang="so"),
            "sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="so"), "kow milyan")
        self.assertEqual(num2words(1000001, lang="so"), "kow milyan kow")
        self.assertEqual(
            num2words(1111111, lang="so"),
            "kow milyan kow boqol toban kow kun kow boqol toban kow",
        )
        self.assertEqual(
            num2words(1234567, lang="so"),
            "kow milyan laba boqol soddon afar kun shan boqol lixdan toddoba",
        )
        self.assertEqual(num2words(2000000, lang="so"), "laba milyan")
        self.assertEqual(num2words(5000000, lang="so"), "shan milyan")
        self.assertEqual(
            num2words(9999999, lang="so"),
            "sagaal milyan sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal",
        )
        self.assertEqual(num2words(10000000, lang="so"), "toban milyan")
        self.assertEqual(
            num2words(12345678, lang="so"),
            "toban laba milyan saddex boqol afartan shan kun lix boqol toddobaatan siddeed",
        )
        self.assertEqual(
            num2words(99999999, lang="so"),
            "sagaashan sagaal milyan sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal",
        )
        self.assertEqual(num2words(100000000, lang="so"), "kow boqol milyan")
        self.assertEqual(
            num2words(123456789, lang="so"),
            "kow boqol labaatan saddex milyan afar boqol konton lix kun toddoba boqol siddeetan sagaal",
        )
        self.assertEqual(
            num2words(999999999, lang="so"),
            "sagaal boqol sagaashan sagaal milyan sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal",
        )
        self.assertEqual(num2words(1000000000, lang="so"), "kow bilyan")
        self.assertEqual(num2words(1234567890, lang="so"), "kow bilyan laba boqol soddon afar milyan shan boqol lixdan toddoba kun siddeed boqol sagaashan")
        self.assertEqual(num2words(9999999999, lang="so"), "sagaal bilyan sagaal boqol sagaashan sagaal milyan sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal")
        self.assertEqual(num2words(10000000000, lang="so"), "toban bilyan")
        self.assertEqual(num2words(99999999999, lang="so"), "sagaashan sagaal bilyan sagaal boqol sagaashan sagaal milyan sagaal boqol sagaashan sagaal kun sagaal boqol sagaashan sagaal")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="so"), "minus kow")
        self.assertEqual(num2words(-2, lang="so"), "minus laba")
        self.assertEqual(num2words(-5, lang="so"), "minus shan")
        self.assertEqual(num2words(-10, lang="so"), "minus toban")
        self.assertEqual(num2words(-11, lang="so"), "minus toban kow")
        self.assertEqual(num2words(-20, lang="so"), "minus labaatan")
        self.assertEqual(num2words(-50, lang="so"), "minus konton")
        self.assertEqual(num2words(-99, lang="so"), "minus sagaashan sagaal")
        self.assertEqual(num2words(-100, lang="so"), "minus kow boqol")
        self.assertEqual(num2words(-101, lang="so"), "minus kow boqol kow")
        self.assertEqual(num2words(-200, lang="so"), "minus laba boqol")
        self.assertEqual(
            num2words(-999, lang="so"), "minus sagaal boqol sagaashan sagaal"
        )
        self.assertEqual(num2words(-1000, lang="so"), "minus kow kun")
        self.assertEqual(num2words(-1001, lang="so"), "minus kow kun kow")
        self.assertEqual(num2words(-10000, lang="so"), "minus toban kun")
        self.assertEqual(num2words(-100000, lang="so"), "minus kow boqol kun")
        self.assertEqual(num2words(-1000000, lang="so"), "minus kow milyan")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="so"), "eber dhibic kow")
        self.assertEqual(num2words(0.5, lang="so"), "eber dhibic shan")
        self.assertEqual(num2words(0.9, lang="so"), "eber dhibic sagaal")
        self.assertEqual(num2words(1.1, lang="so"), "kow dhibic kow")
        self.assertEqual(num2words(1.5, lang="so"), "kow dhibic shan")
        self.assertEqual(num2words(2.5, lang="so"), "laba dhibic shan")
        self.assertEqual(num2words(3.14, lang="so"), "saddex dhibic kow afar")
        self.assertEqual(num2words(10.5, lang="so"), "toban dhibic shan")
        self.assertEqual(num2words(11.11, lang="so"), "toban kow dhibic kow kow")
        self.assertEqual(num2words(20.2, lang="so"), "labaatan dhibic laba")
        self.assertEqual(
            num2words(99.99, lang="so"), "sagaashan sagaal dhibic sagaal sagaal"
        )
        self.assertEqual(num2words(100.01, lang="so"), "kow boqol dhibic eber kow")
        self.assertEqual(num2words(100.5, lang="so"), "kow boqol dhibic shan")
        self.assertEqual(
            num2words(123.45, lang="so"), "kow boqol labaatan saddex dhibic afar shan"
        )
        self.assertEqual(num2words(1000.5, lang="so"), "kow kun dhibic shan")
        self.assertEqual(
            num2words(1234.56, lang="so"),
            "kow kun laba boqol soddon afar dhibic shan lix",
        )
        self.assertEqual(num2words(10000.01, lang="so"), "toban kun dhibic eber kow")
        self.assertEqual(num2words(-0.5, lang="so"), "minus eber dhibic shan")
        self.assertEqual(num2words(-1.5, lang="so"), "minus kow dhibic shan")
        self.assertEqual(num2words(-10.5, lang="so"), "minus toban dhibic shan")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="so", ordinal=True), "kow-aad")
        self.assertEqual(num2words(2, lang="so", ordinal=True), "laba-aad")
        self.assertEqual(num2words(3, lang="so", ordinal=True), "saddex-aad")
        self.assertEqual(num2words(4, lang="so", ordinal=True), "afar-aad")
        self.assertEqual(num2words(5, lang="so", ordinal=True), "shan-aad")
        self.assertEqual(num2words(6, lang="so", ordinal=True), "lix-aad")
        self.assertEqual(num2words(7, lang="so", ordinal=True), "toddoba-aad")
        self.assertEqual(num2words(8, lang="so", ordinal=True), "siddeed-aad")
        self.assertEqual(num2words(9, lang="so", ordinal=True), "sagaal-aad")
        self.assertEqual(num2words(10, lang="so", ordinal=True), "toban-aad")
        self.assertEqual(num2words(11, lang="so", ordinal=True), "toban kow-aad")
        self.assertEqual(num2words(12, lang="so", ordinal=True), "toban laba-aad")
        self.assertEqual(num2words(13, lang="so", ordinal=True), "toban saddex-aad")
        self.assertEqual(num2words(14, lang="so", ordinal=True), "toban afar-aad")
        self.assertEqual(num2words(15, lang="so", ordinal=True), "toban shan-aad")
        self.assertEqual(num2words(16, lang="so", ordinal=True), "toban lix-aad")
        self.assertEqual(num2words(17, lang="so", ordinal=True), "toban toddoba-aad")
        self.assertEqual(num2words(18, lang="so", ordinal=True), "toban siddeed-aad")
        self.assertEqual(num2words(19, lang="so", ordinal=True), "toban sagaal-aad")
        self.assertEqual(num2words(20, lang="so", ordinal=True), "labaatan-aad")
        self.assertEqual(num2words(21, lang="so", ordinal=True), "labaatan kow-aad")
        self.assertEqual(num2words(22, lang="so", ordinal=True), "labaatan laba-aad")
        self.assertEqual(num2words(25, lang="so", ordinal=True), "labaatan shan-aad")
        self.assertEqual(num2words(30, lang="so", ordinal=True), "soddon-aad")
        self.assertEqual(num2words(40, lang="so", ordinal=True), "afartan-aad")
        self.assertEqual(num2words(50, lang="so", ordinal=True), "konton-aad")
        self.assertEqual(num2words(60, lang="so", ordinal=True), "lixdan-aad")
        self.assertEqual(num2words(70, lang="so", ordinal=True), "toddobaatan-aad")
        self.assertEqual(num2words(80, lang="so", ordinal=True), "siddeetan-aad")
        self.assertEqual(num2words(90, lang="so", ordinal=True), "sagaashan-aad")
        self.assertEqual(num2words(100, lang="so", ordinal=True), "kow boqol-aad")
        self.assertEqual(num2words(101, lang="so", ordinal=True), "kow boqol kow-aad")
        self.assertEqual(num2words(200, lang="so", ordinal=True), "laba boqol-aad")
        self.assertEqual(num2words(500, lang="so", ordinal=True), "shan boqol-aad")
        self.assertEqual(num2words(1000, lang="so", ordinal=True), "kow kun-aad")
        self.assertEqual(num2words(1001, lang="so", ordinal=True), "kow kun kow-aad")
        self.assertEqual(num2words(10000, lang="so", ordinal=True), "toban kun-aad")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="so", to="currency", currency="SOS"), "eber shilin"
        )
        self.assertEqual(
            num2words(0.01, lang="so", to="currency", currency="SOS"),
            "eber shilin kow sent",
        )
        self.assertEqual(
            num2words(0.5, lang="so", to="currency", currency="SOS"),
            "eber shilin konton sent",
        )
        self.assertEqual(
            num2words(1, lang="so", to="currency", currency="SOS"), "kow shilin"
        )
        self.assertEqual(
            num2words(1.5, lang="so", to="currency", currency="SOS"),
            "kow shilin konton sent",
        )
        self.assertEqual(
            num2words(0, lang="so", to="currency", currency="USD"), "eber doolar"
        )
        self.assertEqual(
            num2words(0.01, lang="so", to="currency", currency="USD"),
            "eber doolar kow senti",
        )
        self.assertEqual(
            num2words(0.5, lang="so", to="currency", currency="USD"),
            "eber doolar konton senti",
        )
        self.assertEqual(
            num2words(1, lang="so", to="currency", currency="USD"), "kow doolar"
        )
        self.assertEqual(
            num2words(1.5, lang="so", to="currency", currency="USD"),
            "kow doolar konton senti",
        )
        self.assertEqual(
            num2words(0, lang="so", to="currency", currency="EUR"), "eber yuuro"
        )
        self.assertEqual(
            num2words(0.01, lang="so", to="currency", currency="EUR"),
            "eber yuuro kow senti",
        )
        self.assertEqual(
            num2words(0.5, lang="so", to="currency", currency="EUR"),
            "eber yuuro konton senti",
        )
        self.assertEqual(
            num2words(1, lang="so", to="currency", currency="EUR"), "kow yuuro"
        )
        self.assertEqual(
            num2words(1.5, lang="so", to="currency", currency="EUR"),
            "kow yuuro konton senti",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="so", to="year"), "kow kun")
        self.assertEqual(num2words(1066, lang="so", to="year"), "kow kun lixdan lix")
        self.assertEqual(
            num2words(1492, lang="so", to="year"), "kow kun afar boqol sagaashan laba"
        )
        self.assertEqual(
            num2words(1776, lang="so", to="year"),
            "kow kun toddoba boqol toddobaatan lix",
        )
        self.assertEqual(num2words(1800, lang="so", to="year"), "kow kun siddeed boqol")
        self.assertEqual(num2words(1900, lang="so", to="year"), "kow kun sagaal boqol")
        self.assertEqual(
            num2words(1984, lang="so", to="year"), "kow kun sagaal boqol siddeetan afar"
        )
        self.assertEqual(
            num2words(1999, lang="so", to="year"),
            "kow kun sagaal boqol sagaashan sagaal",
        )
        self.assertEqual(num2words(2000, lang="so", to="year"), "laba kun")
        self.assertEqual(num2words(2001, lang="so", to="year"), "laba kun kow")
        self.assertEqual(num2words(2010, lang="so", to="year"), "laba kun toban")
        self.assertEqual(num2words(2020, lang="so", to="year"), "laba kun labaatan")
        self.assertEqual(
            num2words(2024, lang="so", to="year"), "laba kun labaatan afar"
        )
        self.assertEqual(num2words(2100, lang="so", to="year"), "laba kun kow boqol")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="so"), "eber")
        self.assertEqual(num2words("1", lang="so"), "kow")
        self.assertEqual(num2words("10", lang="so"), "toban")
        self.assertEqual(num2words("100", lang="so"), "kow boqol")
        self.assertEqual(num2words("1000", lang="so"), "kow kun")
        self.assertEqual(num2words("10000", lang="so"), "toban kun")
        self.assertEqual(num2words("100000", lang="so"), "kow boqol kun")
        self.assertEqual(num2words("1000000", lang="so"), "kow milyan")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="so"), "eber")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="so"), num2words("100", lang="so"))
        self.assertEqual(num2words(1000, lang="so"), num2words("1000", lang="so"))

