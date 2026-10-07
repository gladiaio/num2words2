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


class Num2WordsSUTest(TestCase):
    """Comprehensive test cases for Sundanese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="su"), "nol")
        self.assertEqual(num2words(1, lang="su"), "hiji")
        self.assertEqual(num2words(2, lang="su"), "dua")
        self.assertEqual(num2words(3, lang="su"), "tilu")
        self.assertEqual(num2words(4, lang="su"), "opat")
        self.assertEqual(num2words(5, lang="su"), "lima")
        self.assertEqual(num2words(6, lang="su"), "genep")
        self.assertEqual(num2words(7, lang="su"), "tujuh")
        self.assertEqual(num2words(8, lang="su"), "dalapan")
        self.assertEqual(num2words(9, lang="su"), "salapan")
        self.assertEqual(num2words(10, lang="su"), "sapuluh")
        self.assertEqual(num2words(11, lang="su"), "sapuluh hiji")
        self.assertEqual(num2words(12, lang="su"), "sapuluh dua")
        self.assertEqual(num2words(13, lang="su"), "sapuluh tilu")
        self.assertEqual(num2words(14, lang="su"), "sapuluh opat")
        self.assertEqual(num2words(15, lang="su"), "sapuluh lima")
        self.assertEqual(num2words(16, lang="su"), "sapuluh genep")
        self.assertEqual(num2words(17, lang="su"), "sapuluh tujuh")
        self.assertEqual(num2words(18, lang="su"), "sapuluh dalapan")
        self.assertEqual(num2words(19, lang="su"), "sapuluh salapan")
        self.assertEqual(num2words(20, lang="su"), "dua puluh")
        self.assertEqual(num2words(21, lang="su"), "dua puluh hiji")
        self.assertEqual(num2words(22, lang="su"), "dua puluh dua")
        self.assertEqual(num2words(23, lang="su"), "dua puluh tilu")
        self.assertEqual(num2words(24, lang="su"), "dua puluh opat")
        self.assertEqual(num2words(25, lang="su"), "dua puluh lima")
        self.assertEqual(num2words(26, lang="su"), "dua puluh genep")
        self.assertEqual(num2words(27, lang="su"), "dua puluh tujuh")
        self.assertEqual(num2words(28, lang="su"), "dua puluh dalapan")
        self.assertEqual(num2words(29, lang="su"), "dua puluh salapan")
        self.assertEqual(num2words(30, lang="su"), "tilu puluh")
        self.assertEqual(num2words(31, lang="su"), "tilu puluh hiji")
        self.assertEqual(num2words(35, lang="su"), "tilu puluh lima")
        self.assertEqual(num2words(40, lang="su"), "opat puluh")
        self.assertEqual(num2words(45, lang="su"), "opat puluh lima")
        self.assertEqual(num2words(50, lang="su"), "lima puluh")
        self.assertEqual(num2words(55, lang="su"), "lima puluh lima")
        self.assertEqual(num2words(60, lang="su"), "genep puluh")
        self.assertEqual(num2words(65, lang="su"), "genep puluh lima")
        self.assertEqual(num2words(70, lang="su"), "tujuh puluh")
        self.assertEqual(num2words(75, lang="su"), "tujuh puluh lima")
        self.assertEqual(num2words(80, lang="su"), "dalapan puluh")
        self.assertEqual(num2words(85, lang="su"), "dalapan puluh lima")
        self.assertEqual(num2words(90, lang="su"), "salapan puluh")
        self.assertEqual(num2words(95, lang="su"), "salapan puluh lima")
        self.assertEqual(num2words(99, lang="su"), "salapan puluh salapan")
        self.assertEqual(num2words(100, lang="su"), "hiji ratus")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="su"), "hiji ratus hiji")
        self.assertEqual(num2words(110, lang="su"), "hiji ratus sapuluh")
        self.assertEqual(num2words(111, lang="su"), "hiji ratus sapuluh hiji")
        self.assertEqual(num2words(120, lang="su"), "hiji ratus dua puluh")
        self.assertEqual(num2words(125, lang="su"), "hiji ratus dua puluh lima")
        self.assertEqual(num2words(150, lang="su"), "hiji ratus lima puluh")
        self.assertEqual(num2words(175, lang="su"), "hiji ratus tujuh puluh lima")
        self.assertEqual(num2words(199, lang="su"), "hiji ratus salapan puluh salapan")
        self.assertEqual(num2words(200, lang="su"), "dua ratus")
        self.assertEqual(num2words(201, lang="su"), "dua ratus hiji")
        self.assertEqual(num2words(210, lang="su"), "dua ratus sapuluh")
        self.assertEqual(num2words(220, lang="su"), "dua ratus dua puluh")
        self.assertEqual(num2words(250, lang="su"), "dua ratus lima puluh")
        self.assertEqual(num2words(299, lang="su"), "dua ratus salapan puluh salapan")
        self.assertEqual(num2words(300, lang="su"), "tilu ratus")
        self.assertEqual(num2words(333, lang="su"), "tilu ratus tilu puluh tilu")
        self.assertEqual(num2words(400, lang="su"), "opat ratus")
        self.assertEqual(num2words(444, lang="su"), "opat ratus opat puluh opat")
        self.assertEqual(num2words(500, lang="su"), "lima ratus")
        self.assertEqual(num2words(555, lang="su"), "lima ratus lima puluh lima")
        self.assertEqual(num2words(600, lang="su"), "genep ratus")
        self.assertEqual(num2words(666, lang="su"), "genep ratus genep puluh genep")
        self.assertEqual(num2words(700, lang="su"), "tujuh ratus")
        self.assertEqual(num2words(777, lang="su"), "tujuh ratus tujuh puluh tujuh")
        self.assertEqual(num2words(800, lang="su"), "dalapan ratus")
        self.assertEqual(
            num2words(888, lang="su"), "dalapan ratus dalapan puluh dalapan"
        )
        self.assertEqual(num2words(900, lang="su"), "salapan ratus")
        self.assertEqual(
            num2words(999, lang="su"), "salapan ratus salapan puluh salapan"
        )

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="su"), "hiji rebu")
        self.assertEqual(num2words(1001, lang="su"), "hiji rebu hiji")
        self.assertEqual(num2words(1010, lang="su"), "hiji rebu sapuluh")
        self.assertEqual(num2words(1100, lang="su"), "hiji rebu hiji ratus")
        self.assertEqual(
            num2words(1111, lang="su"), "hiji rebu hiji ratus sapuluh hiji"
        )
        self.assertEqual(
            num2words(1234, lang="su"), "hiji rebu dua ratus tilu puluh opat"
        )
        self.assertEqual(num2words(1500, lang="su"), "hiji rebu lima ratus")
        self.assertEqual(
            num2words(1999, lang="su"), "hiji rebu salapan ratus salapan puluh salapan"
        )
        self.assertEqual(num2words(2000, lang="su"), "dua rebu")
        self.assertEqual(num2words(2001, lang="su"), "dua rebu hiji")
        self.assertEqual(num2words(2020, lang="su"), "dua rebu dua puluh")
        self.assertEqual(num2words(2222, lang="su"), "dua rebu dua ratus dua puluh dua")
        self.assertEqual(num2words(3000, lang="su"), "tilu rebu")
        self.assertEqual(
            num2words(3333, lang="su"), "tilu rebu tilu ratus tilu puluh tilu"
        )
        self.assertEqual(num2words(4000, lang="su"), "opat rebu")
        self.assertEqual(
            num2words(4444, lang="su"), "opat rebu opat ratus opat puluh opat"
        )
        self.assertEqual(num2words(5000, lang="su"), "lima rebu")
        self.assertEqual(
            num2words(5555, lang="su"), "lima rebu lima ratus lima puluh lima"
        )
        self.assertEqual(num2words(6000, lang="su"), "genep rebu")
        self.assertEqual(
            num2words(6666, lang="su"), "genep rebu genep ratus genep puluh genep"
        )
        self.assertEqual(num2words(7000, lang="su"), "tujuh rebu")
        self.assertEqual(
            num2words(7777, lang="su"), "tujuh rebu tujuh ratus tujuh puluh tujuh"
        )
        self.assertEqual(num2words(8000, lang="su"), "dalapan rebu")
        self.assertEqual(
            num2words(8888, lang="su"),
            "dalapan rebu dalapan ratus dalapan puluh dalapan",
        )
        self.assertEqual(num2words(9000, lang="su"), "salapan rebu")
        self.assertEqual(
            num2words(9999, lang="su"),
            "salapan rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(10000, lang="su"), "sapuluh rebu")
        self.assertEqual(num2words(10001, lang="su"), "sapuluh rebu hiji")
        self.assertEqual(
            num2words(11111, lang="su"), "sapuluh hiji rebu hiji ratus sapuluh hiji"
        )
        self.assertEqual(
            num2words(12345, lang="su"), "sapuluh dua rebu tilu ratus opat puluh lima"
        )
        self.assertEqual(num2words(20000, lang="su"), "dua puluh rebu")
        self.assertEqual(num2words(50000, lang="su"), "lima puluh rebu")
        self.assertEqual(
            num2words(99999, lang="su"),
            "salapan puluh salapan rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(100000, lang="su"), "hiji ratus rebu")
        self.assertEqual(
            num2words(123456, lang="su"),
            "hiji ratus dua puluh tilu rebu opat ratus lima puluh genep",
        )
        self.assertEqual(num2words(200000, lang="su"), "dua ratus rebu")
        self.assertEqual(num2words(500000, lang="su"), "lima ratus rebu")
        self.assertEqual(
            num2words(654321, lang="su"),
            "genep ratus lima puluh opat rebu tilu ratus dua puluh hiji",
        )
        self.assertEqual(
            num2words(999999, lang="su"),
            "salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="su"), "hiji juta")
        self.assertEqual(num2words(1000001, lang="su"), "hiji juta hiji")
        self.assertEqual(
            num2words(1111111, lang="su"),
            "hiji juta hiji ratus sapuluh hiji rebu hiji ratus sapuluh hiji",
        )
        self.assertEqual(
            num2words(1234567, lang="su"),
            "hiji juta dua ratus tilu puluh opat rebu lima ratus genep puluh tujuh",
        )
        self.assertEqual(num2words(2000000, lang="su"), "dua juta")
        self.assertEqual(num2words(5000000, lang="su"), "lima juta")
        self.assertEqual(
            num2words(9999999, lang="su"),
            "salapan juta salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(10000000, lang="su"), "sapuluh juta")
        self.assertEqual(
            num2words(12345678, lang="su"),
            "sapuluh dua juta tilu ratus opat puluh lima rebu genep ratus tujuh puluh dalapan",
        )
        self.assertEqual(
            num2words(99999999, lang="su"),
            "salapan puluh salapan juta salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(100000000, lang="su"), "hiji ratus juta")
        self.assertEqual(
            num2words(123456789, lang="su"),
            "hiji ratus dua puluh tilu juta opat ratus lima puluh genep rebu tujuh ratus dalapan puluh salapan",
        )
        self.assertEqual(
            num2words(999999999, lang="su"),
            "salapan ratus salapan puluh salapan juta salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(1000000000, lang="su"), "hiji miliar")
        self.assertEqual(num2words(1234567890, lang="su"), "hiji miliar dua ratus tilu puluh opat juta lima ratus genep puluh tujuh rebu dalapan ratus salapan puluh")
        self.assertEqual(num2words(9999999999, lang="su"), "salapan miliar salapan ratus salapan puluh salapan juta salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan")
        self.assertEqual(num2words(10000000000, lang="su"), "sapuluh miliar")
        self.assertEqual(num2words(99999999999, lang="su"), "salapan puluh salapan miliar salapan ratus salapan puluh salapan juta salapan ratus salapan puluh salapan rebu salapan ratus salapan puluh salapan")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="su"), "mineus hiji")
        self.assertEqual(num2words(-2, lang="su"), "mineus dua")
        self.assertEqual(num2words(-5, lang="su"), "mineus lima")
        self.assertEqual(num2words(-10, lang="su"), "mineus sapuluh")
        self.assertEqual(num2words(-11, lang="su"), "mineus sapuluh hiji")
        self.assertEqual(num2words(-20, lang="su"), "mineus dua puluh")
        self.assertEqual(num2words(-50, lang="su"), "mineus lima puluh")
        self.assertEqual(num2words(-99, lang="su"), "mineus salapan puluh salapan")
        self.assertEqual(num2words(-100, lang="su"), "mineus hiji ratus")
        self.assertEqual(num2words(-101, lang="su"), "mineus hiji ratus hiji")
        self.assertEqual(num2words(-200, lang="su"), "mineus dua ratus")
        self.assertEqual(
            num2words(-999, lang="su"), "mineus salapan ratus salapan puluh salapan"
        )
        self.assertEqual(num2words(-1000, lang="su"), "mineus hiji rebu")
        self.assertEqual(num2words(-1001, lang="su"), "mineus hiji rebu hiji")
        self.assertEqual(num2words(-10000, lang="su"), "mineus sapuluh rebu")
        self.assertEqual(num2words(-100000, lang="su"), "mineus hiji ratus rebu")
        self.assertEqual(num2words(-1000000, lang="su"), "mineus hiji juta")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="su"), "nol koma hiji")
        self.assertEqual(num2words(0.5, lang="su"), "nol koma lima")
        self.assertEqual(num2words(0.9, lang="su"), "nol koma salapan")
        self.assertEqual(num2words(1.1, lang="su"), "hiji koma hiji")
        self.assertEqual(num2words(1.5, lang="su"), "hiji koma lima")
        self.assertEqual(num2words(2.5, lang="su"), "dua koma lima")
        self.assertEqual(num2words(3.14, lang="su"), "tilu koma hiji opat")
        self.assertEqual(num2words(10.5, lang="su"), "sapuluh koma lima")
        self.assertEqual(num2words(11.11, lang="su"), "sapuluh hiji koma hiji hiji")
        self.assertEqual(num2words(20.2, lang="su"), "dua puluh koma dua")
        self.assertEqual(
            num2words(99.99, lang="su"), "salapan puluh salapan koma salapan salapan"
        )
        self.assertEqual(num2words(100.01, lang="su"), "hiji ratus koma nol hiji")
        self.assertEqual(num2words(100.5, lang="su"), "hiji ratus koma lima")
        self.assertEqual(
            num2words(123.45, lang="su"), "hiji ratus dua puluh tilu koma opat lima"
        )
        self.assertEqual(num2words(1000.5, lang="su"), "hiji rebu koma lima")
        self.assertEqual(
            num2words(1234.56, lang="su"),
            "hiji rebu dua ratus tilu puluh opat koma lima genep",
        )
        self.assertEqual(num2words(10000.01, lang="su"), "sapuluh rebu koma nol hiji")
        self.assertEqual(num2words(-0.5, lang="su"), "mineus nol koma lima")
        self.assertEqual(num2words(-1.5, lang="su"), "mineus hiji koma lima")
        self.assertEqual(num2words(-10.5, lang="su"), "mineus sapuluh koma lima")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="su", ordinal=True), "hiji-na")
        self.assertEqual(num2words(2, lang="su", ordinal=True), "dua-na")
        self.assertEqual(num2words(3, lang="su", ordinal=True), "tilu-na")
        self.assertEqual(num2words(4, lang="su", ordinal=True), "opat-na")
        self.assertEqual(num2words(5, lang="su", ordinal=True), "lima-na")
        self.assertEqual(num2words(6, lang="su", ordinal=True), "genep-na")
        self.assertEqual(num2words(7, lang="su", ordinal=True), "tujuh-na")
        self.assertEqual(num2words(8, lang="su", ordinal=True), "dalapan-na")
        self.assertEqual(num2words(9, lang="su", ordinal=True), "salapan-na")
        self.assertEqual(num2words(10, lang="su", ordinal=True), "sapuluh-na")
        self.assertEqual(num2words(11, lang="su", ordinal=True), "sapuluh hiji-na")
        self.assertEqual(num2words(12, lang="su", ordinal=True), "sapuluh dua-na")
        self.assertEqual(num2words(13, lang="su", ordinal=True), "sapuluh tilu-na")
        self.assertEqual(num2words(14, lang="su", ordinal=True), "sapuluh opat-na")
        self.assertEqual(num2words(15, lang="su", ordinal=True), "sapuluh lima-na")
        self.assertEqual(num2words(16, lang="su", ordinal=True), "sapuluh genep-na")
        self.assertEqual(num2words(17, lang="su", ordinal=True), "sapuluh tujuh-na")
        self.assertEqual(num2words(18, lang="su", ordinal=True), "sapuluh dalapan-na")
        self.assertEqual(num2words(19, lang="su", ordinal=True), "sapuluh salapan-na")
        self.assertEqual(num2words(20, lang="su", ordinal=True), "dua puluh-na")
        self.assertEqual(num2words(21, lang="su", ordinal=True), "dua puluh hiji-na")
        self.assertEqual(num2words(22, lang="su", ordinal=True), "dua puluh dua-na")
        self.assertEqual(num2words(25, lang="su", ordinal=True), "dua puluh lima-na")
        self.assertEqual(num2words(30, lang="su", ordinal=True), "tilu puluh-na")
        self.assertEqual(num2words(40, lang="su", ordinal=True), "opat puluh-na")
        self.assertEqual(num2words(50, lang="su", ordinal=True), "lima puluh-na")
        self.assertEqual(num2words(60, lang="su", ordinal=True), "genep puluh-na")
        self.assertEqual(num2words(70, lang="su", ordinal=True), "tujuh puluh-na")
        self.assertEqual(num2words(80, lang="su", ordinal=True), "dalapan puluh-na")
        self.assertEqual(num2words(90, lang="su", ordinal=True), "salapan puluh-na")
        self.assertEqual(num2words(100, lang="su", ordinal=True), "hiji ratus-na")
        self.assertEqual(num2words(101, lang="su", ordinal=True), "hiji ratus hiji-na")
        self.assertEqual(num2words(200, lang="su", ordinal=True), "dua ratus-na")
        self.assertEqual(num2words(500, lang="su", ordinal=True), "lima ratus-na")
        self.assertEqual(num2words(1000, lang="su", ordinal=True), "hiji rebu-na")
        self.assertEqual(num2words(1001, lang="su", ordinal=True), "hiji rebu hiji-na")
        self.assertEqual(num2words(10000, lang="su", ordinal=True), "sapuluh rebu-na")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="su", to="currency", currency="IDR"), "nol rupiah"
        )
        self.assertEqual(
            num2words(0.01, lang="su", to="currency", currency="IDR"),
            "nol rupiah hiji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="su", to="currency", currency="IDR"),
            "nol rupiah lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="su", to="currency", currency="IDR"), "hiji rupiah"
        )
        self.assertEqual(
            num2words(1.5, lang="su", to="currency", currency="IDR"),
            "hiji rupiah lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="su", to="currency", currency="USD"), "nol dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="su", to="currency", currency="USD"),
            "nol dolar hiji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="su", to="currency", currency="USD"),
            "nol dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="su", to="currency", currency="USD"), "hiji dolar"
        )
        self.assertEqual(
            num2words(1.5, lang="su", to="currency", currency="USD"),
            "hiji dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="su", to="currency", currency="EUR"), "nol euro"
        )
        self.assertEqual(
            num2words(0.01, lang="su", to="currency", currency="EUR"),
            "nol euro hiji sen",
        )
        self.assertEqual(
            num2words(0.5, lang="su", to="currency", currency="EUR"),
            "nol euro lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="su", to="currency", currency="EUR"), "hiji euro"
        )
        self.assertEqual(
            num2words(1.5, lang="su", to="currency", currency="EUR"),
            "hiji euro lima puluh sen",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="su", to="year"), "hiji rebu")
        self.assertEqual(
            num2words(1066, lang="su", to="year"), "hiji rebu genep puluh genep"
        )
        self.assertEqual(
            num2words(1492, lang="su", to="year"),
            "hiji rebu opat ratus salapan puluh dua",
        )
        self.assertEqual(
            num2words(1776, lang="su", to="year"),
            "hiji rebu tujuh ratus tujuh puluh genep",
        )
        self.assertEqual(
            num2words(1800, lang="su", to="year"), "hiji rebu dalapan ratus"
        )
        self.assertEqual(
            num2words(1900, lang="su", to="year"), "hiji rebu salapan ratus"
        )
        self.assertEqual(
            num2words(1984, lang="su", to="year"),
            "hiji rebu salapan ratus dalapan puluh opat",
        )
        self.assertEqual(
            num2words(1999, lang="su", to="year"),
            "hiji rebu salapan ratus salapan puluh salapan",
        )
        self.assertEqual(num2words(2000, lang="su", to="year"), "dua rebu")
        self.assertEqual(num2words(2001, lang="su", to="year"), "dua rebu hiji")
        self.assertEqual(num2words(2010, lang="su", to="year"), "dua rebu sapuluh")
        self.assertEqual(num2words(2020, lang="su", to="year"), "dua rebu dua puluh")
        self.assertEqual(
            num2words(2024, lang="su", to="year"), "dua rebu dua puluh opat"
        )
        self.assertEqual(num2words(2100, lang="su", to="year"), "dua rebu hiji ratus")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="su"), "nol")
        self.assertEqual(num2words("1", lang="su"), "hiji")
        self.assertEqual(num2words("10", lang="su"), "sapuluh")
        self.assertEqual(num2words("100", lang="su"), "hiji ratus")
        self.assertEqual(num2words("1000", lang="su"), "hiji rebu")
        self.assertEqual(num2words("10000", lang="su"), "sapuluh rebu")
        self.assertEqual(num2words("100000", lang="su"), "hiji ratus rebu")
        self.assertEqual(num2words("1000000", lang="su"), "hiji juta")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="su"), "nol")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="su"), num2words("100", lang="su"))
        self.assertEqual(num2words(1000, lang="su"), num2words("1000", lang="su"))

