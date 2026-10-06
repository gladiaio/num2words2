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


class Num2WordsMSTest(TestCase):
    """Comprehensive test cases for Malay language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ms"), "kosong")
        self.assertEqual(num2words(1, lang="ms"), "satu")
        self.assertEqual(num2words(2, lang="ms"), "dua")
        self.assertEqual(num2words(3, lang="ms"), "tiga")
        self.assertEqual(num2words(4, lang="ms"), "empat")
        self.assertEqual(num2words(5, lang="ms"), "lima")
        self.assertEqual(num2words(6, lang="ms"), "enam")
        self.assertEqual(num2words(7, lang="ms"), "tujuh")
        self.assertEqual(num2words(8, lang="ms"), "lapan")
        self.assertEqual(num2words(9, lang="ms"), "sembilan")
        self.assertEqual(num2words(10, lang="ms"), "sepuluh")
        self.assertEqual(num2words(11, lang="ms"), "sebelas")
        self.assertEqual(num2words(12, lang="ms"), "dua belas")
        self.assertEqual(num2words(13, lang="ms"), "tiga belas")
        self.assertEqual(num2words(14, lang="ms"), "empat belas")
        self.assertEqual(num2words(15, lang="ms"), "lima belas")
        self.assertEqual(num2words(16, lang="ms"), "enam belas")
        self.assertEqual(num2words(17, lang="ms"), "tujuh belas")
        self.assertEqual(num2words(18, lang="ms"), "lapan belas")
        self.assertEqual(num2words(19, lang="ms"), "sembilan belas")
        self.assertEqual(num2words(20, lang="ms"), "dua puluh")
        self.assertEqual(num2words(21, lang="ms"), "dua puluh satu")
        self.assertEqual(num2words(22, lang="ms"), "dua puluh dua")
        self.assertEqual(num2words(23, lang="ms"), "dua puluh tiga")
        self.assertEqual(num2words(24, lang="ms"), "dua puluh empat")
        self.assertEqual(num2words(25, lang="ms"), "dua puluh lima")
        self.assertEqual(num2words(26, lang="ms"), "dua puluh enam")
        self.assertEqual(num2words(27, lang="ms"), "dua puluh tujuh")
        self.assertEqual(num2words(28, lang="ms"), "dua puluh lapan")
        self.assertEqual(num2words(29, lang="ms"), "dua puluh sembilan")
        self.assertEqual(num2words(30, lang="ms"), "tiga puluh")
        self.assertEqual(num2words(31, lang="ms"), "tiga puluh satu")
        self.assertEqual(num2words(35, lang="ms"), "tiga puluh lima")
        self.assertEqual(num2words(40, lang="ms"), "empat puluh")
        self.assertEqual(num2words(45, lang="ms"), "empat puluh lima")
        self.assertEqual(num2words(50, lang="ms"), "lima puluh")
        self.assertEqual(num2words(55, lang="ms"), "lima puluh lima")
        self.assertEqual(num2words(60, lang="ms"), "enam puluh")
        self.assertEqual(num2words(65, lang="ms"), "enam puluh lima")
        self.assertEqual(num2words(70, lang="ms"), "tujuh puluh")
        self.assertEqual(num2words(75, lang="ms"), "tujuh puluh lima")
        self.assertEqual(num2words(80, lang="ms"), "lapan puluh")
        self.assertEqual(num2words(85, lang="ms"), "lapan puluh lima")
        self.assertEqual(num2words(90, lang="ms"), "sembilan puluh")
        self.assertEqual(num2words(95, lang="ms"), "sembilan puluh lima")
        self.assertEqual(num2words(99, lang="ms"), "sembilan puluh sembilan")
        self.assertEqual(num2words(100, lang="ms"), "seratus")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ms"), "seratus satu")
        self.assertEqual(num2words(110, lang="ms"), "seratus sepuluh")
        self.assertEqual(num2words(111, lang="ms"), "seratus sebelas")
        self.assertEqual(num2words(120, lang="ms"), "seratus dua puluh")
        self.assertEqual(num2words(125, lang="ms"), "seratus dua puluh lima")
        self.assertEqual(num2words(150, lang="ms"), "seratus lima puluh")
        self.assertEqual(num2words(175, lang="ms"), "seratus tujuh puluh lima")
        self.assertEqual(num2words(199, lang="ms"), "seratus sembilan puluh sembilan")
        self.assertEqual(num2words(200, lang="ms"), "dua ratus")
        self.assertEqual(num2words(201, lang="ms"), "dua ratus satu")
        self.assertEqual(num2words(210, lang="ms"), "dua ratus sepuluh")
        self.assertEqual(num2words(220, lang="ms"), "dua ratus dua puluh")
        self.assertEqual(num2words(250, lang="ms"), "dua ratus lima puluh")
        self.assertEqual(num2words(299, lang="ms"), "dua ratus sembilan puluh sembilan")
        self.assertEqual(num2words(300, lang="ms"), "tiga ratus")
        self.assertEqual(num2words(333, lang="ms"), "tiga ratus tiga puluh tiga")
        self.assertEqual(num2words(400, lang="ms"), "empat ratus")
        self.assertEqual(num2words(444, lang="ms"), "empat ratus empat puluh empat")
        self.assertEqual(num2words(500, lang="ms"), "lima ratus")
        self.assertEqual(num2words(555, lang="ms"), "lima ratus lima puluh lima")
        self.assertEqual(num2words(600, lang="ms"), "enam ratus")
        self.assertEqual(num2words(666, lang="ms"), "enam ratus enam puluh enam")
        self.assertEqual(num2words(700, lang="ms"), "tujuh ratus")
        self.assertEqual(num2words(777, lang="ms"), "tujuh ratus tujuh puluh tujuh")
        self.assertEqual(num2words(800, lang="ms"), "lapan ratus")
        self.assertEqual(num2words(888, lang="ms"), "lapan ratus lapan puluh lapan")
        self.assertEqual(num2words(900, lang="ms"), "sembilan ratus")
        self.assertEqual(
            num2words(999, lang="ms"), "sembilan ratus sembilan puluh sembilan"
        )

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ms"), "seribu")
        self.assertEqual(num2words(1001, lang="ms"), "seribu satu")
        self.assertEqual(num2words(1010, lang="ms"), "seribu sepuluh")
        self.assertEqual(num2words(1100, lang="ms"), "seribu seratus")
        self.assertEqual(num2words(1111, lang="ms"), "seribu seratus sebelas")
        self.assertEqual(
            num2words(1234, lang="ms"), "seribu dua ratus tiga puluh empat"
        )
        self.assertEqual(num2words(1500, lang="ms"), "seribu lima ratus")
        self.assertEqual(
            num2words(1999, lang="ms"), "seribu sembilan ratus sembilan puluh sembilan"
        )
        self.assertEqual(num2words(2000, lang="ms"), "dua ribu")
        self.assertEqual(num2words(2001, lang="ms"), "dua ribu satu")
        self.assertEqual(num2words(2020, lang="ms"), "dua ribu dua puluh")
        self.assertEqual(num2words(2222, lang="ms"), "dua ribu dua ratus dua puluh dua")
        self.assertEqual(num2words(3000, lang="ms"), "tiga ribu")
        self.assertEqual(
            num2words(3333, lang="ms"), "tiga ribu tiga ratus tiga puluh tiga"
        )
        self.assertEqual(num2words(4000, lang="ms"), "empat ribu")
        self.assertEqual(
            num2words(4444, lang="ms"), "empat ribu empat ratus empat puluh empat"
        )
        self.assertEqual(num2words(5000, lang="ms"), "lima ribu")
        self.assertEqual(
            num2words(5555, lang="ms"), "lima ribu lima ratus lima puluh lima"
        )
        self.assertEqual(num2words(6000, lang="ms"), "enam ribu")
        self.assertEqual(
            num2words(6666, lang="ms"), "enam ribu enam ratus enam puluh enam"
        )
        self.assertEqual(num2words(7000, lang="ms"), "tujuh ribu")
        self.assertEqual(
            num2words(7777, lang="ms"), "tujuh ribu tujuh ratus tujuh puluh tujuh"
        )
        self.assertEqual(num2words(8000, lang="ms"), "lapan ribu")
        self.assertEqual(
            num2words(8888, lang="ms"), "lapan ribu lapan ratus lapan puluh lapan"
        )
        self.assertEqual(num2words(9000, lang="ms"), "sembilan ribu")
        self.assertEqual(
            num2words(9999, lang="ms"),
            "sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(10000, lang="ms"), "sepuluh ribu")
        self.assertEqual(num2words(10001, lang="ms"), "sepuluh ribu satu")
        self.assertEqual(num2words(11111, lang="ms"), "sebelas ribu seratus sebelas")
        self.assertEqual(
            num2words(12345, lang="ms"), "dua belas ribu tiga ratus empat puluh lima"
        )
        self.assertEqual(num2words(20000, lang="ms"), "dua puluh ribu")
        self.assertEqual(num2words(50000, lang="ms"), "lima puluh ribu")
        self.assertEqual(
            num2words(99999, lang="ms"),
            "sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(100000, lang="ms"), "seratus ribu")
        self.assertEqual(
            num2words(123456, lang="ms"),
            "seratus dua puluh tiga ribu empat ratus lima puluh enam",
        )
        self.assertEqual(num2words(200000, lang="ms"), "dua ratus ribu")
        self.assertEqual(num2words(500000, lang="ms"), "lima ratus ribu")
        self.assertEqual(
            num2words(654321, lang="ms"),
            "enam ratus lima puluh empat ribu tiga ratus dua puluh satu",
        )
        self.assertEqual(
            num2words(999999, lang="ms"),
            "sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ms"), "satu juta")
        self.assertEqual(num2words(1000001, lang="ms"), "satu juta satu")
        self.assertEqual(
            num2words(1111111, lang="ms"),
            "satu juta seratus sebelas ribu seratus sebelas",
        )
        self.assertEqual(
            num2words(1234567, lang="ms"),
            "satu juta dua ratus tiga puluh empat ribu lima ratus enam puluh tujuh",
        )
        self.assertEqual(num2words(2000000, lang="ms"), "dua juta")
        self.assertEqual(num2words(5000000, lang="ms"), "lima juta")
        self.assertEqual(
            num2words(9999999, lang="ms"),
            "sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(10000000, lang="ms"), "sepuluh juta")
        self.assertEqual(
            num2words(12345678, lang="ms"),
            "dua belas juta tiga ratus empat puluh lima ribu enam ratus tujuh puluh lapan",
        )
        self.assertEqual(
            num2words(99999999, lang="ms"),
            "sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(100000000, lang="ms"), "seratus juta")
        self.assertEqual(
            num2words(123456789, lang="ms"),
            "seratus dua puluh tiga juta empat ratus lima puluh enam ribu tujuh ratus lapan puluh sembilan",
        )
        self.assertEqual(
            num2words(999999999, lang="ms"),
            "sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(1000000000, lang="ms"), "satu bilion")
        self.assertEqual(
            num2words(1234567890, lang="ms"),
            "satu bilion dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu lapan ratus sembilan puluh",
        )
        self.assertEqual(
            num2words(9999999999, lang="ms"),
            "sembilan bilion sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(10000000000, lang="ms"), "sepuluh bilion")
        self.assertEqual(
            num2words(99999999999, lang="ms"),
            "sembilan puluh sembilan bilion sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ms"), "negatif satu")
        self.assertEqual(num2words(-2, lang="ms"), "negatif dua")
        self.assertEqual(num2words(-5, lang="ms"), "negatif lima")
        self.assertEqual(num2words(-10, lang="ms"), "negatif sepuluh")
        self.assertEqual(num2words(-11, lang="ms"), "negatif sebelas")
        self.assertEqual(num2words(-20, lang="ms"), "negatif dua puluh")
        self.assertEqual(num2words(-50, lang="ms"), "negatif lima puluh")
        self.assertEqual(num2words(-99, lang="ms"), "negatif sembilan puluh sembilan")
        self.assertEqual(num2words(-100, lang="ms"), "negatif seratus")
        self.assertEqual(num2words(-101, lang="ms"), "negatif seratus satu")
        self.assertEqual(num2words(-200, lang="ms"), "negatif dua ratus")
        self.assertEqual(
            num2words(-999, lang="ms"), "negatif sembilan ratus sembilan puluh sembilan"
        )
        self.assertEqual(num2words(-1000, lang="ms"), "negatif seribu")
        self.assertEqual(num2words(-1001, lang="ms"), "negatif seribu satu")
        self.assertEqual(num2words(-10000, lang="ms"), "negatif sepuluh ribu")
        self.assertEqual(num2words(-100000, lang="ms"), "negatif seratus ribu")
        self.assertEqual(num2words(-1000000, lang="ms"), "negatif satu juta")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ms"), "kosong")
        self.assertEqual(num2words(0.5, lang="ms"), "kosong")
        self.assertEqual(num2words(0.9, lang="ms"), "kosong")
        self.assertEqual(num2words(1.1, lang="ms"), "satu")
        self.assertEqual(num2words(1.5, lang="ms"), "satu")
        self.assertEqual(num2words(2.5, lang="ms"), "dua")
        self.assertEqual(num2words(3.14, lang="ms"), "tiga")
        self.assertEqual(num2words(10.5, lang="ms"), "sepuluh")
        self.assertEqual(num2words(11.11, lang="ms"), "sebelas")
        self.assertEqual(num2words(20.2, lang="ms"), "dua puluh")
        self.assertEqual(num2words(99.99, lang="ms"), "sembilan puluh sembilan")
        self.assertEqual(num2words(100.01, lang="ms"), "seratus")
        self.assertEqual(num2words(100.5, lang="ms"), "seratus")
        self.assertEqual(num2words(123.45, lang="ms"), "seratus dua puluh tiga")
        self.assertEqual(num2words(1000.5, lang="ms"), "seribu")
        self.assertEqual(
            num2words(1234.56, lang="ms"), "seribu dua ratus tiga puluh empat"
        )
        self.assertEqual(num2words(10000.01, lang="ms"), "sepuluh ribu")
        self.assertEqual(num2words(-0.5, lang="ms"), "kosong")
        self.assertEqual(num2words(-1.5, lang="ms"), "negatif satu")
        self.assertEqual(num2words(-10.5, lang="ms"), "negatif sepuluh")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ms", ordinal=True), "pertama")
        self.assertEqual(num2words(2, lang="ms", ordinal=True), "kedua")
        self.assertEqual(num2words(3, lang="ms", ordinal=True), "ketiga")
        self.assertEqual(num2words(4, lang="ms", ordinal=True), "keempat")
        self.assertEqual(num2words(5, lang="ms", ordinal=True), "kelima")
        self.assertEqual(num2words(6, lang="ms", ordinal=True), "keenam")
        self.assertEqual(num2words(7, lang="ms", ordinal=True), "ketujuh")
        self.assertEqual(num2words(8, lang="ms", ordinal=True), "kelapan")
        self.assertEqual(num2words(9, lang="ms", ordinal=True), "kesembilan")
        self.assertEqual(num2words(10, lang="ms", ordinal=True), "kesepuluh")
        self.assertEqual(num2words(11, lang="ms", ordinal=True), "ke-sebelas")
        self.assertEqual(num2words(12, lang="ms", ordinal=True), "ke-dua belas")
        self.assertEqual(num2words(13, lang="ms", ordinal=True), "ke-tiga belas")
        self.assertEqual(num2words(14, lang="ms", ordinal=True), "ke-empat belas")
        self.assertEqual(num2words(15, lang="ms", ordinal=True), "ke-lima belas")
        self.assertEqual(num2words(16, lang="ms", ordinal=True), "ke-enam belas")
        self.assertEqual(num2words(17, lang="ms", ordinal=True), "ke-tujuh belas")
        self.assertEqual(num2words(18, lang="ms", ordinal=True), "ke-lapan belas")
        self.assertEqual(num2words(19, lang="ms", ordinal=True), "ke-sembilan belas")
        self.assertEqual(num2words(20, lang="ms", ordinal=True), "ke-dua puluh")
        self.assertEqual(num2words(21, lang="ms", ordinal=True), "ke-dua puluh satu")
        self.assertEqual(num2words(22, lang="ms", ordinal=True), "ke-dua puluh dua")
        self.assertEqual(num2words(25, lang="ms", ordinal=True), "ke-dua puluh lima")
        self.assertEqual(num2words(30, lang="ms", ordinal=True), "ke-tiga puluh")
        self.assertEqual(num2words(40, lang="ms", ordinal=True), "ke-empat puluh")
        self.assertEqual(num2words(50, lang="ms", ordinal=True), "ke-lima puluh")
        self.assertEqual(num2words(60, lang="ms", ordinal=True), "ke-enam puluh")
        self.assertEqual(num2words(70, lang="ms", ordinal=True), "ke-tujuh puluh")
        self.assertEqual(num2words(80, lang="ms", ordinal=True), "ke-lapan puluh")
        self.assertEqual(num2words(90, lang="ms", ordinal=True), "ke-sembilan puluh")
        self.assertEqual(num2words(100, lang="ms", ordinal=True), "ke-seratus")
        self.assertEqual(num2words(101, lang="ms", ordinal=True), "ke-seratus satu")
        self.assertEqual(num2words(200, lang="ms", ordinal=True), "ke-dua ratus")
        self.assertEqual(num2words(500, lang="ms", ordinal=True), "ke-lima ratus")
        self.assertEqual(num2words(1000, lang="ms", ordinal=True), "ke-seribu")
        self.assertEqual(num2words(1001, lang="ms", ordinal=True), "ke-seribu satu")
        self.assertEqual(num2words(10000, lang="ms", ordinal=True), "ke-sepuluh ribu")

    def test_currency_int_is_units(self):
        # gladiaio/num2words2#171: an int was read as cents.
        for v in (1, 42, 100, -5, 1000000):
            self.assertEqual(
                num2words(v, lang="ms", to="currency"),
                num2words(float(v), lang="ms", to="currency"),
            )
        self.assertEqual(
            num2words(42, lang="ms", to="currency"), "empat puluh dua ringgit"
        )

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="MYR"), "kosong ringgit"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="MYR"),
            "kosong ringgit satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="MYR"),
            "kosong ringgit lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="MYR"),
            "satu ringgit",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="MYR"),
            "satu ringgit lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="SGD"), "kosong dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="SGD"),
            "kosong dolar satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="SGD"),
            "kosong dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="SGD"),
            "satu dolar",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="SGD"),
            "satu dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="USD"), "kosong dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="USD"),
            "kosong dolar satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="USD"),
            "kosong dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="USD"),
            "satu dolar",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="USD"),
            "satu dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="EUR"), "kosong euro"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="EUR"),
            "kosong euro satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="EUR"),
            "kosong euro lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="EUR"),
            "satu euro",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="EUR"),
            "satu euro lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="GBP"), "kosong paun"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="GBP"),
            "kosong paun satu peni",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="GBP"),
            "kosong paun lima puluh peni",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="GBP"),
            "satu paun",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="GBP"),
            "satu paun lima puluh peni",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="IDR"), "kosong rupiah"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="IDR"),
            "kosong rupiah satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="IDR"),
            "kosong rupiah lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="IDR"),
            "satu rupiah",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="IDR"),
            "satu rupiah lima puluh sen",
        )
        self.assertEqual(
            num2words(0, lang="ms", to="currency", currency="BND"), "kosong dolar"
        )
        self.assertEqual(
            num2words(0.01, lang="ms", to="currency", currency="BND"),
            "kosong dolar satu sen",
        )
        self.assertEqual(
            num2words(0.5, lang="ms", to="currency", currency="BND"),
            "kosong dolar lima puluh sen",
        )
        self.assertEqual(
            num2words(1, lang="ms", to="currency", currency="BND"),
            "satu dolar",
        )
        self.assertEqual(
            num2words(1.5, lang="ms", to="currency", currency="BND"),
            "satu dolar lima puluh sen",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ms", to="year"), "seribu")
        self.assertEqual(
            num2words(1066, lang="ms", to="year"), "seribu enam puluh enam"
        )
        self.assertEqual(
            num2words(1492, lang="ms", to="year"),
            "seribu empat ratus sembilan puluh dua",
        )
        self.assertEqual(
            num2words(1776, lang="ms", to="year"), "seribu tujuh ratus tujuh puluh enam"
        )
        self.assertEqual(num2words(1800, lang="ms", to="year"), "seribu lapan ratus")
        self.assertEqual(num2words(1900, lang="ms", to="year"), "seribu sembilan ratus")
        self.assertEqual(
            num2words(1984, lang="ms", to="year"),
            "seribu sembilan ratus lapan puluh empat",
        )
        self.assertEqual(
            num2words(1999, lang="ms", to="year"),
            "seribu sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(2000, lang="ms", to="year"), "dua ribu")
        self.assertEqual(num2words(2001, lang="ms", to="year"), "dua ribu satu")
        self.assertEqual(num2words(2010, lang="ms", to="year"), "dua ribu sepuluh")
        self.assertEqual(num2words(2020, lang="ms", to="year"), "dua ribu dua puluh")
        self.assertEqual(
            num2words(2024, lang="ms", to="year"), "dua ribu dua puluh empat"
        )
        self.assertEqual(num2words(2100, lang="ms", to="year"), "dua ribu seratus")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ms"), "kosong")
        self.assertEqual(num2words("1", lang="ms"), "satu")
        self.assertEqual(num2words("10", lang="ms"), "sepuluh")
        self.assertEqual(num2words("100", lang="ms"), "seratus")
        self.assertEqual(num2words("1000", lang="ms"), "seribu")
        self.assertEqual(num2words("10000", lang="ms"), "sepuluh ribu")
        self.assertEqual(num2words("100000", lang="ms"), "seratus ribu")
        self.assertEqual(num2words("1000000", lang="ms"), "satu juta")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ms"), "kosong")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ms"), num2words("100", lang="ms"))
        self.assertEqual(num2words(1000, lang="ms"), num2words("1000", lang="ms"))


    def test_negative_ordinal_raises(self):
        # gladiaio/num2words2#155: Python list indexing wrapped -1 to
        # "kesepuluh" (tenth), -11 to "" and crashed with IndexError below.
        for n in (-1, -2, -11, -12, -42, -1000, -0.5, -1.5):
            with self.assertRaisesRegex(TypeError, "Cannot treat negative"):
                num2words(n, lang="ms", to="ordinal")
        self.assertEqual(num2words(10, lang="ms", to="ordinal"), "kesepuluh")
