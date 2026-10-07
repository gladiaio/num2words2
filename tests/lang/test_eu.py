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


class Num2WordsEUTest(TestCase):
    """Comprehensive test cases for Basque language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="eu"), "zero")
        self.assertEqual(num2words(1, lang="eu"), "bat")
        self.assertEqual(num2words(2, lang="eu"), "bi")
        self.assertEqual(num2words(3, lang="eu"), "hiru")
        self.assertEqual(num2words(4, lang="eu"), "lau")
        self.assertEqual(num2words(5, lang="eu"), "bost")
        self.assertEqual(num2words(6, lang="eu"), "sei")
        self.assertEqual(num2words(7, lang="eu"), "zazpi")
        self.assertEqual(num2words(8, lang="eu"), "zortzi")
        self.assertEqual(num2words(9, lang="eu"), "bederatzi")
        self.assertEqual(num2words(10, lang="eu"), "hamar")
        self.assertEqual(num2words(11, lang="eu"), "hamabat")
        self.assertEqual(num2words(12, lang="eu"), "hamabi")
        self.assertEqual(num2words(13, lang="eu"), "hamahiru")
        self.assertEqual(num2words(14, lang="eu"), "hamalau")
        self.assertEqual(num2words(15, lang="eu"), "hamabost")
        self.assertEqual(num2words(16, lang="eu"), "hamasei")
        self.assertEqual(num2words(17, lang="eu"), "hamazazpi")
        self.assertEqual(num2words(18, lang="eu"), "hamazortzi")
        self.assertEqual(num2words(19, lang="eu"), "hamabederatzi")
        self.assertEqual(num2words(20, lang="eu"), "hogei")
        self.assertEqual(num2words(21, lang="eu"), "hogeita bat")
        self.assertEqual(num2words(22, lang="eu"), "hogeita bi")
        self.assertEqual(num2words(23, lang="eu"), "hogeita hiru")
        self.assertEqual(num2words(24, lang="eu"), "hogeita lau")
        self.assertEqual(num2words(25, lang="eu"), "hogeita bost")
        self.assertEqual(num2words(26, lang="eu"), "hogeita sei")
        self.assertEqual(num2words(27, lang="eu"), "hogeita zazpi")
        self.assertEqual(num2words(28, lang="eu"), "hogeita zortzi")
        self.assertEqual(num2words(29, lang="eu"), "hogeita bederatzi")
        self.assertEqual(num2words(30, lang="eu"), "hogeita hamar")
        self.assertEqual(num2words(31, lang="eu"), "hogeita hamarta bat")
        self.assertEqual(num2words(35, lang="eu"), "hogeita hamarta bost")
        self.assertEqual(num2words(40, lang="eu"), "berrogei")
        self.assertEqual(num2words(45, lang="eu"), "berrogeita bost")
        self.assertEqual(num2words(50, lang="eu"), "berrogeita hamar")
        self.assertEqual(num2words(55, lang="eu"), "berrogeita hamarta bost")
        self.assertEqual(num2words(60, lang="eu"), "hirurogei")
        self.assertEqual(num2words(65, lang="eu"), "hirurogeita bost")
        self.assertEqual(num2words(70, lang="eu"), "hirurogeita hamar")
        self.assertEqual(num2words(75, lang="eu"), "hirurogeita hamarta bost")
        self.assertEqual(num2words(80, lang="eu"), "laurogei")
        self.assertEqual(num2words(85, lang="eu"), "laurogeita bost")
        self.assertEqual(num2words(90, lang="eu"), "laurogeita hamar")
        self.assertEqual(num2words(95, lang="eu"), "laurogeita hamarta bost")
        self.assertEqual(num2words(99, lang="eu"), "laurogeita hamarta bederatzi")
        self.assertEqual(num2words(100, lang="eu"), "ehun")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="eu"), "ehun eta bat")
        self.assertEqual(num2words(110, lang="eu"), "ehun eta hamar")
        self.assertEqual(num2words(111, lang="eu"), "ehun eta hamabat")
        self.assertEqual(num2words(120, lang="eu"), "ehun eta hogei")
        self.assertEqual(num2words(125, lang="eu"), "ehun eta hogeita bost")
        self.assertEqual(num2words(150, lang="eu"), "ehun eta berrogeita hamar")
        self.assertEqual(num2words(175, lang="eu"), "ehun eta hirurogeita hamarta bost")
        self.assertEqual(
            num2words(199, lang="eu"), "ehun eta laurogeita hamarta bederatzi"
        )
        self.assertEqual(num2words(200, lang="eu"), "biehun")
        self.assertEqual(num2words(201, lang="eu"), "biehun eta bat")
        self.assertEqual(num2words(210, lang="eu"), "biehun eta hamar")
        self.assertEqual(num2words(220, lang="eu"), "biehun eta hogei")
        self.assertEqual(num2words(250, lang="eu"), "biehun eta berrogeita hamar")
        self.assertEqual(
            num2words(299, lang="eu"), "biehun eta laurogeita hamarta bederatzi"
        )
        self.assertEqual(num2words(300, lang="eu"), "hiruehun")
        self.assertEqual(num2words(333, lang="eu"), "hiruehun eta hogeita hamarta hiru")
        self.assertEqual(num2words(400, lang="eu"), "lauehun")
        self.assertEqual(num2words(444, lang="eu"), "lauehun eta berrogeita lau")
        self.assertEqual(num2words(500, lang="eu"), "bostehun")
        self.assertEqual(
            num2words(555, lang="eu"), "bostehun eta berrogeita hamarta bost"
        )
        self.assertEqual(num2words(600, lang="eu"), "seiehun")
        self.assertEqual(num2words(666, lang="eu"), "seiehun eta hirurogeita sei")
        self.assertEqual(num2words(700, lang="eu"), "zazpiehun")
        self.assertEqual(
            num2words(777, lang="eu"), "zazpiehun eta hirurogeita hamarta zazpi"
        )
        self.assertEqual(num2words(800, lang="eu"), "zortziehun")
        self.assertEqual(num2words(888, lang="eu"), "zortziehun eta laurogeita zortzi")
        self.assertEqual(num2words(900, lang="eu"), "bederatziehun")
        self.assertEqual(
            num2words(999, lang="eu"), "bederatziehun eta laurogeita hamarta bederatzi"
        )

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="eu"), "mila")
        self.assertEqual(num2words(1001, lang="eu"), "mila bat")
        self.assertEqual(num2words(1010, lang="eu"), "mila hamar")
        self.assertEqual(num2words(1100, lang="eu"), "mila ehun")
        self.assertEqual(num2words(1111, lang="eu"), "mila ehun eta hamabat")
        self.assertEqual(
            num2words(1234, lang="eu"), "mila biehun eta hogeita hamarta lau"
        )
        self.assertEqual(num2words(1500, lang="eu"), "mila bostehun")
        self.assertEqual(
            num2words(1999, lang="eu"),
            "mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(2000, lang="eu"), "bi mila")
        self.assertEqual(num2words(2001, lang="eu"), "bi mila bat")
        self.assertEqual(num2words(2020, lang="eu"), "bi mila hogei")
        self.assertEqual(num2words(2222, lang="eu"), "bi mila biehun eta hogeita bi")
        self.assertEqual(num2words(3000, lang="eu"), "hiru mila")
        self.assertEqual(
            num2words(3333, lang="eu"), "hiru mila hiruehun eta hogeita hamarta hiru"
        )
        self.assertEqual(num2words(4000, lang="eu"), "lau mila")
        self.assertEqual(
            num2words(4444, lang="eu"), "lau mila lauehun eta berrogeita lau"
        )
        self.assertEqual(num2words(5000, lang="eu"), "bost mila")
        self.assertEqual(
            num2words(5555, lang="eu"), "bost mila bostehun eta berrogeita hamarta bost"
        )
        self.assertEqual(num2words(6000, lang="eu"), "sei mila")
        self.assertEqual(
            num2words(6666, lang="eu"), "sei mila seiehun eta hirurogeita sei"
        )
        self.assertEqual(num2words(7000, lang="eu"), "zazpi mila")
        self.assertEqual(
            num2words(7777, lang="eu"),
            "zazpi mila zazpiehun eta hirurogeita hamarta zazpi",
        )
        self.assertEqual(num2words(8000, lang="eu"), "zortzi mila")
        self.assertEqual(
            num2words(8888, lang="eu"), "zortzi mila zortziehun eta laurogeita zortzi"
        )
        self.assertEqual(num2words(9000, lang="eu"), "bederatzi mila")
        self.assertEqual(
            num2words(9999, lang="eu"),
            "bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(10000, lang="eu"), "hamar mila")
        self.assertEqual(num2words(10001, lang="eu"), "hamar mila bat")
        self.assertEqual(num2words(11111, lang="eu"), "hamabat mila ehun eta hamabat")
        self.assertEqual(
            num2words(12345, lang="eu"), "hamabi mila hiruehun eta berrogeita bost"
        )
        self.assertEqual(num2words(20000, lang="eu"), "hogei mila")
        self.assertEqual(num2words(50000, lang="eu"), "berrogeita hamar mila")
        self.assertEqual(
            num2words(99999, lang="eu"),
            "laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(100000, lang="eu"), "ehun mila")
        self.assertEqual(
            num2words(123456, lang="eu"),
            "ehun eta hogeita hiru mila lauehun eta berrogeita hamarta sei",
        )
        self.assertEqual(num2words(200000, lang="eu"), "biehun mila")
        self.assertEqual(num2words(500000, lang="eu"), "bostehun mila")
        self.assertEqual(
            num2words(654321, lang="eu"),
            "seiehun eta berrogeita hamarta lau mila hiruehun eta hogeita bat",
        )
        self.assertEqual(
            num2words(999999, lang="eu"),
            "bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="eu"), "milioi bat")
        self.assertEqual(num2words(1000001, lang="eu"), "milioi bat bat")
        self.assertEqual(
            num2words(1111111, lang="eu"),
            "milioi bat ehun eta hamabat mila ehun eta hamabat",
        )
        self.assertEqual(
            num2words(1234567, lang="eu"),
            "milioi bat biehun eta hogeita hamarta lau mila bostehun eta hirurogeita zazpi",
        )
        self.assertEqual(num2words(2000000, lang="eu"), "bi milioi")
        self.assertEqual(num2words(5000000, lang="eu"), "bost milioi")
        self.assertEqual(
            num2words(9999999, lang="eu"),
            "bederatzi milioi bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(10000000, lang="eu"), "hamar milioi")
        self.assertEqual(
            num2words(12345678, lang="eu"),
            "hamabi milioi hiruehun eta berrogeita bost mila seiehun eta hirurogeita hamarta zortzi",
        )
        self.assertEqual(
            num2words(99999999, lang="eu"),
            "laurogeita hamarta bederatzi milioi bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(100000000, lang="eu"), "ehun milioi")
        self.assertEqual(
            num2words(123456789, lang="eu"),
            "ehun eta hogeita hiru milioi lauehun eta berrogeita hamarta sei mila zazpiehun eta laurogeita bederatzi",
        )
        self.assertEqual(
            num2words(999999999, lang="eu"),
            "bederatziehun eta laurogeita hamarta bederatzi milioi bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(1000000000, lang="eu"), "bat mila milioi")
        self.assertEqual(
            num2words(1234567890, lang="eu"),
            "bat mila milioi biehun eta hogeita hamarta lau milioi bostehun eta hirurogeita zazpi mila zortziehun eta laurogeita hamar",
        )
        self.assertEqual(
            num2words(9999999999, lang="eu"),
            "bederatzi mila milioi bederatziehun eta laurogeita hamarta bederatzi milioi bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(10000000000, lang="eu"), "hamar mila milioi")
        self.assertEqual(
            num2words(99999999999, lang="eu"),
            "laurogeita hamarta bederatzi mila milioi bederatziehun eta laurogeita hamarta bederatzi milioi bederatziehun eta laurogeita hamarta bederatzi mila bederatziehun eta laurogeita hamarta bederatzi",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="eu"), "minus bat")
        self.assertEqual(num2words(-2, lang="eu"), "minus bi")
        self.assertEqual(num2words(-5, lang="eu"), "minus bost")
        self.assertEqual(num2words(-10, lang="eu"), "minus hamar")
        self.assertEqual(num2words(-11, lang="eu"), "minus hamabat")
        self.assertEqual(num2words(-20, lang="eu"), "minus hogei")
        self.assertEqual(num2words(-50, lang="eu"), "minus berrogeita hamar")
        self.assertEqual(
            num2words(-99, lang="eu"), "minus laurogeita hamarta bederatzi"
        )
        self.assertEqual(num2words(-100, lang="eu"), "minus ehun")
        self.assertEqual(num2words(-101, lang="eu"), "minus ehun eta bat")
        self.assertEqual(num2words(-200, lang="eu"), "minus biehun")
        self.assertEqual(
            num2words(-999, lang="eu"),
            "minus bederatziehun eta laurogeita hamarta bederatzi",
        )
        self.assertEqual(num2words(-1000, lang="eu"), "minus mila")
        self.assertEqual(num2words(-1001, lang="eu"), "minus mila bat")
        self.assertEqual(num2words(-10000, lang="eu"), "minus hamar mila")
        self.assertEqual(num2words(-100000, lang="eu"), "minus ehun mila")
        self.assertEqual(num2words(-1000000, lang="eu"), "minus milioi bat")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="eu"), "zero koma bat")
        self.assertEqual(num2words(0.5, lang="eu"), "zero koma bost")
        self.assertEqual(num2words(0.9, lang="eu"), "zero koma bederatzi")
        self.assertEqual(num2words(1.1, lang="eu"), "bat koma bat")
        self.assertEqual(num2words(1.5, lang="eu"), "bat koma bost")
        self.assertEqual(num2words(2.5, lang="eu"), "bi koma bost")
        self.assertEqual(num2words(3.14, lang="eu"), "hiru koma bat lau")
        self.assertEqual(num2words(10.5, lang="eu"), "hamar koma bost")
        self.assertEqual(num2words(11.11, lang="eu"), "hamabat koma bat bat")
        self.assertEqual(num2words(20.2, lang="eu"), "hogei koma bi")
        self.assertEqual(
            num2words(99.99, lang="eu"),
            "laurogeita hamarta bederatzi koma bederatzi bederatzi",
        )
        self.assertEqual(num2words(100.01, lang="eu"), "ehun koma zero bat")
        self.assertEqual(num2words(100.5, lang="eu"), "ehun koma bost")
        self.assertEqual(
            num2words(123.45, lang="eu"), "ehun eta hogeita hiru koma lau bost"
        )
        self.assertEqual(num2words(1000.5, lang="eu"), "mila koma bost")
        self.assertEqual(
            num2words(1234.56, lang="eu"),
            "mila biehun eta hogeita hamarta lau koma bost sei",
        )
        self.assertEqual(num2words(10000.01, lang="eu"), "hamar mila koma zero bat")
        self.assertEqual(num2words(-0.5, lang="eu"), "minus zero koma bost")
        self.assertEqual(num2words(-1.5, lang="eu"), "minus bat koma bost")
        self.assertEqual(num2words(-10.5, lang="eu"), "minus hamar koma bost")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="eu", ordinal=True), "batgarren")
        self.assertEqual(num2words(2, lang="eu", ordinal=True), "bigarren")
        self.assertEqual(num2words(3, lang="eu", ordinal=True), "hirugarren")
        self.assertEqual(num2words(4, lang="eu", ordinal=True), "laugarren")
        self.assertEqual(num2words(5, lang="eu", ordinal=True), "bostgarren")
        self.assertEqual(num2words(6, lang="eu", ordinal=True), "seigarren")
        self.assertEqual(num2words(7, lang="eu", ordinal=True), "zazpigarren")
        self.assertEqual(num2words(8, lang="eu", ordinal=True), "zortzigarren")
        self.assertEqual(num2words(9, lang="eu", ordinal=True), "bederatzigarren")
        self.assertEqual(num2words(10, lang="eu", ordinal=True), "hamargarren")
        self.assertEqual(num2words(11, lang="eu", ordinal=True), "hamabatgarren")
        self.assertEqual(num2words(12, lang="eu", ordinal=True), "hamabigarren")
        self.assertEqual(num2words(13, lang="eu", ordinal=True), "hamahirugarren")
        self.assertEqual(num2words(14, lang="eu", ordinal=True), "hamalaugarren")
        self.assertEqual(num2words(15, lang="eu", ordinal=True), "hamabostgarren")
        self.assertEqual(num2words(16, lang="eu", ordinal=True), "hamaseigarren")
        self.assertEqual(num2words(17, lang="eu", ordinal=True), "hamazazpigarren")
        self.assertEqual(num2words(18, lang="eu", ordinal=True), "hamazortzigarren")
        self.assertEqual(num2words(19, lang="eu", ordinal=True), "hamabederatzigarren")
        self.assertEqual(num2words(20, lang="eu", ordinal=True), "hogeigarren")
        self.assertEqual(num2words(21, lang="eu", ordinal=True), "hogeita batgarren")
        self.assertEqual(num2words(22, lang="eu", ordinal=True), "hogeita bigarren")
        self.assertEqual(num2words(25, lang="eu", ordinal=True), "hogeita bostgarren")
        self.assertEqual(num2words(30, lang="eu", ordinal=True), "hogeita hamargarren")
        self.assertEqual(num2words(40, lang="eu", ordinal=True), "berrogeigarren")
        self.assertEqual(
            num2words(50, lang="eu", ordinal=True), "berrogeita hamargarren"
        )
        self.assertEqual(num2words(60, lang="eu", ordinal=True), "hirurogeigarren")
        self.assertEqual(
            num2words(70, lang="eu", ordinal=True), "hirurogeita hamargarren"
        )
        self.assertEqual(num2words(80, lang="eu", ordinal=True), "laurogeigarren")
        self.assertEqual(
            num2words(90, lang="eu", ordinal=True), "laurogeita hamargarren"
        )
        self.assertEqual(num2words(100, lang="eu", ordinal=True), "ehungarren")
        self.assertEqual(num2words(101, lang="eu", ordinal=True), "ehun eta batgarren")
        self.assertEqual(num2words(200, lang="eu", ordinal=True), "biehungarren")
        self.assertEqual(num2words(500, lang="eu", ordinal=True), "bostehungarren")
        self.assertEqual(num2words(1000, lang="eu", ordinal=True), "milagarren")
        self.assertEqual(num2words(1001, lang="eu", ordinal=True), "mila batgarren")
        self.assertEqual(num2words(10000, lang="eu", ordinal=True), "hamar milagarren")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="eu", to="currency", currency="EUR"), "zero euro"
        )
        self.assertEqual(
            num2words(0.01, lang="eu", to="currency", currency="EUR"),
            "zero euro eta bat zentimo",
        )
        self.assertEqual(
            num2words(0.5, lang="eu", to="currency", currency="EUR"),
            "zero euro eta berrogeita hamar zentimo",
        )
        self.assertEqual(
            num2words(1, lang="eu", to="currency", currency="EUR"), "bat euro"
        )
        self.assertEqual(
            num2words(1.5, lang="eu", to="currency", currency="EUR"),
            "bat euro eta berrogeita hamar zentimo",
        )
        self.assertEqual(
            num2words(0, lang="eu", to="currency", currency="USD"), "zero dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="eu", to="currency", currency="USD"),
            "zero dolar eta bat zentabo",
        )
        self.assertEqual(
            num2words(0.5, lang="eu", to="currency", currency="USD"),
            "zero dolar eta berrogeita hamar zentabo",
        )
        self.assertEqual(
            num2words(1, lang="eu", to="currency", currency="USD"), "bat dolar"
        )
        self.assertEqual(
            num2words(1.5, lang="eu", to="currency", currency="USD"),
            "bat dolar eta berrogeita hamar zentabo",
        )
        self.assertEqual(
            num2words(0, lang="eu", to="currency", currency="GBP"), "zero libera"
        )
        self.assertEqual(
            num2words(0.01, lang="eu", to="currency", currency="GBP"),
            "zero libera eta bat penike",
        )
        self.assertEqual(
            num2words(0.5, lang="eu", to="currency", currency="GBP"),
            "zero libera eta berrogeita hamar penike",
        )
        self.assertEqual(
            num2words(1, lang="eu", to="currency", currency="GBP"), "bat libera"
        )
        self.assertEqual(
            num2words(1.5, lang="eu", to="currency", currency="GBP"),
            "bat libera eta berrogeita hamar penike",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="eu", to="year"), "mila. urtea")
        self.assertEqual(
            num2words(1066, lang="eu", to="year"), "mila hirurogeita sei. urtea"
        )
        self.assertEqual(
            num2words(1492, lang="eu", to="year"),
            "mila lauehun eta laurogeita hamarta bi. urtea",
        )
        self.assertEqual(
            num2words(1776, lang="eu", to="year"),
            "mila zazpiehun eta hirurogeita hamarta sei. urtea",
        )
        self.assertEqual(
            num2words(1800, lang="eu", to="year"), "mila zortziehun. urtea"
        )
        self.assertEqual(
            num2words(1900, lang="eu", to="year"), "mila bederatziehun. urtea"
        )
        self.assertEqual(
            num2words(1984, lang="eu", to="year"),
            "mila bederatziehun eta laurogeita lau. urtea",
        )
        self.assertEqual(
            num2words(1999, lang="eu", to="year"),
            "mila bederatziehun eta laurogeita hamarta bederatzi. urtea",
        )
        self.assertEqual(num2words(2000, lang="eu", to="year"), "bi mila. urtea")
        self.assertEqual(num2words(2001, lang="eu", to="year"), "bi mila bat. urtea")
        self.assertEqual(num2words(2010, lang="eu", to="year"), "bi mila hamar. urtea")
        self.assertEqual(num2words(2020, lang="eu", to="year"), "bi mila hogei. urtea")
        self.assertEqual(
            num2words(2024, lang="eu", to="year"), "bi mila hogeita lau. urtea"
        )
        self.assertEqual(num2words(2100, lang="eu", to="year"), "bi mila ehun. urtea")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="eu"), "zero")
        self.assertEqual(num2words("1", lang="eu"), "bat")
        self.assertEqual(num2words("10", lang="eu"), "hamar")
        self.assertEqual(num2words("100", lang="eu"), "ehun")
        self.assertEqual(num2words("1000", lang="eu"), "mila")
        self.assertEqual(num2words("10000", lang="eu"), "hamar mila")
        self.assertEqual(num2words("100000", lang="eu"), "ehun mila")
        self.assertEqual(num2words("1000000", lang="eu"), "milioi bat")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="eu"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="eu"), num2words("100", lang="eu"))
        self.assertEqual(num2words(1000, lang="eu"), num2words("1000", lang="eu"))
