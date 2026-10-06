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


class Num2WordsHATest(TestCase):
    """Comprehensive test cases for Hausa language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ha"), "sifiri")
        self.assertEqual(num2words(1, lang="ha"), "ɗaya")
        self.assertEqual(num2words(2, lang="ha"), "biyu")
        self.assertEqual(num2words(3, lang="ha"), "uku")
        self.assertEqual(num2words(4, lang="ha"), "huɗu")
        self.assertEqual(num2words(5, lang="ha"), "biyar")
        self.assertEqual(num2words(6, lang="ha"), "shida")
        self.assertEqual(num2words(7, lang="ha"), "bakwai")
        self.assertEqual(num2words(8, lang="ha"), "takwas")
        self.assertEqual(num2words(9, lang="ha"), "tara")
        self.assertEqual(num2words(10, lang="ha"), "goma")
        self.assertEqual(num2words(11, lang="ha"), "sha ɗaya")
        self.assertEqual(num2words(12, lang="ha"), "sha biyu")
        self.assertEqual(num2words(13, lang="ha"), "sha uku")
        self.assertEqual(num2words(14, lang="ha"), "sha huɗu")
        self.assertEqual(num2words(15, lang="ha"), "sha biyar")
        self.assertEqual(num2words(16, lang="ha"), "sha shida")
        self.assertEqual(num2words(17, lang="ha"), "sha bakwai")
        self.assertEqual(num2words(18, lang="ha"), "sha takwas")
        self.assertEqual(num2words(19, lang="ha"), "sha tara")
        self.assertEqual(num2words(20, lang="ha"), "ashirin")
        self.assertEqual(num2words(21, lang="ha"), "ashirin da ɗaya")
        self.assertEqual(num2words(22, lang="ha"), "ashirin da biyu")
        self.assertEqual(num2words(23, lang="ha"), "ashirin da uku")
        self.assertEqual(num2words(24, lang="ha"), "ashirin da huɗu")
        self.assertEqual(num2words(25, lang="ha"), "ashirin da biyar")
        self.assertEqual(num2words(26, lang="ha"), "ashirin da shida")
        self.assertEqual(num2words(27, lang="ha"), "ashirin da bakwai")
        self.assertEqual(num2words(28, lang="ha"), "ashirin da takwas")
        self.assertEqual(num2words(29, lang="ha"), "ashirin da tara")
        self.assertEqual(num2words(30, lang="ha"), "talatin")
        self.assertEqual(num2words(31, lang="ha"), "talatin da ɗaya")
        self.assertEqual(num2words(35, lang="ha"), "talatin da biyar")
        self.assertEqual(num2words(40, lang="ha"), "arba'in")
        self.assertEqual(num2words(45, lang="ha"), "arba'in da biyar")
        self.assertEqual(num2words(50, lang="ha"), "hamsin")
        self.assertEqual(num2words(55, lang="ha"), "hamsin da biyar")
        self.assertEqual(num2words(60, lang="ha"), "sittin")
        self.assertEqual(num2words(65, lang="ha"), "sittin da biyar")
        self.assertEqual(num2words(70, lang="ha"), "saba'in")
        self.assertEqual(num2words(75, lang="ha"), "saba'in da biyar")
        self.assertEqual(num2words(80, lang="ha"), "tamanin")
        self.assertEqual(num2words(85, lang="ha"), "tamanin da biyar")
        self.assertEqual(num2words(90, lang="ha"), "casa'in")
        self.assertEqual(num2words(95, lang="ha"), "casa'in da biyar")
        self.assertEqual(num2words(99, lang="ha"), "casa'in da tara")
        self.assertEqual(num2words(100, lang="ha"), "ɗari")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ha"), "ɗari da ɗaya")
        self.assertEqual(num2words(110, lang="ha"), "ɗari goma")
        self.assertEqual(num2words(111, lang="ha"), "ɗari sha ɗaya")
        self.assertEqual(num2words(120, lang="ha"), "ɗari ashirin")
        self.assertEqual(num2words(125, lang="ha"), "ɗari ashirin da biyar")
        self.assertEqual(num2words(150, lang="ha"), "ɗari hamsin")
        self.assertEqual(num2words(175, lang="ha"), "ɗari saba'in da biyar")
        self.assertEqual(num2words(199, lang="ha"), "ɗari casa'in da tara")
        self.assertEqual(num2words(200, lang="ha"), "ɗari biyu")
        self.assertEqual(num2words(201, lang="ha"), "ɗari biyu da ɗaya")
        self.assertEqual(num2words(210, lang="ha"), "ɗari biyu goma")
        self.assertEqual(num2words(220, lang="ha"), "ɗari biyu ashirin")
        self.assertEqual(num2words(250, lang="ha"), "ɗari biyu hamsin")
        self.assertEqual(num2words(299, lang="ha"), "ɗari biyu casa'in da tara")
        self.assertEqual(num2words(300, lang="ha"), "ɗari uku")
        self.assertEqual(num2words(333, lang="ha"), "ɗari uku talatin da uku")
        self.assertEqual(num2words(400, lang="ha"), "ɗari huɗu")
        self.assertEqual(num2words(444, lang="ha"), "ɗari huɗu arba'in da huɗu")
        self.assertEqual(num2words(500, lang="ha"), "ɗari biyar")
        self.assertEqual(num2words(555, lang="ha"), "ɗari biyar hamsin da biyar")
        self.assertEqual(num2words(600, lang="ha"), "ɗari shida")
        self.assertEqual(num2words(666, lang="ha"), "ɗari shida sittin da shida")
        self.assertEqual(num2words(700, lang="ha"), "ɗari bakwai")
        self.assertEqual(num2words(777, lang="ha"), "ɗari bakwai saba'in da bakwai")
        self.assertEqual(num2words(800, lang="ha"), "ɗari takwas")
        self.assertEqual(num2words(888, lang="ha"), "ɗari takwas tamanin da takwas")
        self.assertEqual(num2words(900, lang="ha"), "ɗari tara")
        self.assertEqual(num2words(999, lang="ha"), "ɗari tara casa'in da tara")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ha"), "dubu")
        self.assertEqual(num2words(1001, lang="ha"), "dubu da ɗaya")
        self.assertEqual(num2words(1010, lang="ha"), "dubu goma")
        self.assertEqual(num2words(1100, lang="ha"), "dubu ɗari")
        self.assertEqual(num2words(1111, lang="ha"), "dubu ɗari sha ɗaya")
        self.assertEqual(num2words(1234, lang="ha"), "dubu ɗari biyu talatin da huɗu")
        self.assertEqual(num2words(1500, lang="ha"), "dubu ɗari biyar")
        self.assertEqual(num2words(1999, lang="ha"), "dubu ɗari tara casa'in da tara")
        self.assertEqual(num2words(2000, lang="ha"), "dubu biyu")
        self.assertEqual(num2words(2001, lang="ha"), "dubu biyu da ɗaya")
        self.assertEqual(num2words(2020, lang="ha"), "dubu biyu ashirin")
        self.assertEqual(
            num2words(2222, lang="ha"), "dubu biyu ɗari biyu ashirin da biyu"
        )
        self.assertEqual(num2words(3000, lang="ha"), "dubu uku")
        self.assertEqual(num2words(3333, lang="ha"), "dubu uku ɗari uku talatin da uku")
        self.assertEqual(num2words(4000, lang="ha"), "dubu huɗu")
        self.assertEqual(
            num2words(4444, lang="ha"), "dubu huɗu ɗari huɗu arba'in da huɗu"
        )
        self.assertEqual(num2words(5000, lang="ha"), "dubu biyar")
        self.assertEqual(
            num2words(5555, lang="ha"), "dubu biyar ɗari biyar hamsin da biyar"
        )
        self.assertEqual(num2words(6000, lang="ha"), "dubu shida")
        self.assertEqual(
            num2words(6666, lang="ha"), "dubu shida ɗari shida sittin da shida"
        )
        self.assertEqual(num2words(7000, lang="ha"), "dubu bakwai")
        self.assertEqual(
            num2words(7777, lang="ha"), "dubu bakwai ɗari bakwai saba'in da bakwai"
        )
        self.assertEqual(num2words(8000, lang="ha"), "dubu takwas")
        self.assertEqual(
            num2words(8888, lang="ha"), "dubu takwas ɗari takwas tamanin da takwas"
        )
        self.assertEqual(num2words(9000, lang="ha"), "dubu tara")
        self.assertEqual(
            num2words(9999, lang="ha"), "dubu tara ɗari tara casa'in da tara"
        )
        self.assertEqual(num2words(10000, lang="ha"), "dubu goma")
        self.assertEqual(num2words(10001, lang="ha"), "dubu goma da ɗaya")
        self.assertEqual(num2words(11111, lang="ha"), "dubu sha ɗaya ɗari sha ɗaya")
        self.assertEqual(
            num2words(12345, lang="ha"), "dubu sha biyu ɗari uku arba'in da biyar"
        )
        self.assertEqual(num2words(20000, lang="ha"), "dubu ashirin")
        self.assertEqual(num2words(50000, lang="ha"), "dubu hamsin")
        self.assertEqual(
            num2words(99999, lang="ha"),
            "dubu casa'in da tara ɗari tara casa'in da tara",
        )
        self.assertEqual(num2words(100000, lang="ha"), "dubu ɗari")
        self.assertEqual(
            num2words(123456, lang="ha"),
            "dubu ɗari ashirin da uku ɗari huɗu hamsin da shida",
        )
        self.assertEqual(num2words(200000, lang="ha"), "dubu ɗari biyu")
        self.assertEqual(num2words(500000, lang="ha"), "dubu ɗari biyar")
        self.assertEqual(
            num2words(654321, lang="ha"),
            "dubu ɗari shida hamsin da huɗu ɗari uku ashirin da ɗaya",
        )
        self.assertEqual(
            num2words(999999, lang="ha"),
            "dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ha"), "miliyan")
        self.assertEqual(num2words(1000001, lang="ha"), "miliyan da ɗaya")
        self.assertEqual(
            num2words(1111111, lang="ha"), "miliyan dubu ɗari sha ɗaya ɗari sha ɗaya"
        )
        self.assertEqual(
            num2words(1234567, lang="ha"),
            "miliyan dubu ɗari biyu talatin da huɗu ɗari biyar sittin da bakwai",
        )
        self.assertEqual(num2words(2000000, lang="ha"), "miliyan biyu")
        self.assertEqual(num2words(5000000, lang="ha"), "miliyan biyar")
        self.assertEqual(
            num2words(9999999, lang="ha"),
            "miliyan tara dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )
        self.assertEqual(num2words(10000000, lang="ha"), "miliyan goma")
        self.assertEqual(
            num2words(12345678, lang="ha"),
            "miliyan sha biyu dubu ɗari uku arba'in da biyar ɗari shida saba'in da takwas",
        )
        self.assertEqual(
            num2words(99999999, lang="ha"),
            "miliyan casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )
        self.assertEqual(num2words(100000000, lang="ha"), "miliyan ɗari")
        self.assertEqual(
            num2words(123456789, lang="ha"),
            "miliyan ɗari ashirin da uku dubu ɗari huɗu hamsin da shida ɗari bakwai tamanin da tara",
        )
        self.assertEqual(
            num2words(999999999, lang="ha"),
            "miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )
        self.assertEqual(num2words(1000000000, lang="ha"), "biliyan")
        self.assertEqual(
            num2words(1234567890, lang="ha"),
            "biliyan miliyan ɗari biyu talatin da huɗu dubu ɗari biyar sittin da bakwai ɗari takwas casa'in",
        )
        self.assertEqual(
            num2words(9999999999, lang="ha"),
            "biliyan tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )
        self.assertEqual(num2words(10000000000, lang="ha"), "biliyan goma")
        self.assertEqual(
            num2words(99999999999, lang="ha"),
            "biliyan casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da tara",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ha"), "ban ɗaya")
        self.assertEqual(num2words(-2, lang="ha"), "ban biyu")
        self.assertEqual(num2words(-5, lang="ha"), "ban biyar")
        self.assertEqual(num2words(-10, lang="ha"), "ban goma")
        self.assertEqual(num2words(-11, lang="ha"), "ban sha ɗaya")
        self.assertEqual(num2words(-20, lang="ha"), "ban ashirin")
        self.assertEqual(num2words(-50, lang="ha"), "ban hamsin")
        self.assertEqual(num2words(-99, lang="ha"), "ban casa'in da tara")
        self.assertEqual(num2words(-100, lang="ha"), "ban ɗari")
        self.assertEqual(num2words(-101, lang="ha"), "ban ɗari da ɗaya")
        self.assertEqual(num2words(-200, lang="ha"), "ban ɗari biyu")
        self.assertEqual(num2words(-999, lang="ha"), "ban ɗari tara casa'in da tara")
        self.assertEqual(num2words(-1000, lang="ha"), "ban dubu")
        self.assertEqual(num2words(-1001, lang="ha"), "ban dubu da ɗaya")
        self.assertEqual(num2words(-10000, lang="ha"), "ban dubu goma")
        self.assertEqual(num2words(-100000, lang="ha"), "ban dubu ɗari")
        self.assertEqual(num2words(-1000000, lang="ha"), "ban miliyan")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ha"), "sifiri wajen ɗaya")
        self.assertEqual(num2words(0.5, lang="ha"), "sifiri wajen biyar")
        self.assertEqual(num2words(0.9, lang="ha"), "sifiri wajen tara")
        self.assertEqual(
            num2words(1.1, lang="ha"), "ɗaya wajen tiriliyan dubu goma da tara"
        )
        self.assertEqual(num2words(1.5, lang="ha"), "ɗaya wajen biyar")
        self.assertEqual(num2words(2.5, lang="ha"), "biyu wajen biyar")
        self.assertEqual(
            num2words(3.14, lang="ha"), "uku wajen tiriliyan dubu sha huɗu sha biyu"
        )
        self.assertEqual(num2words(10.5, lang="ha"), "goma wajen biyar")
        self.assertEqual(
            num2words(11.11, lang="ha"),
            "sha ɗaya wajen tiriliyan dubu goma ɗari tara casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara arba'in da uku",
        )
        self.assertEqual(
            num2words(20.2, lang="ha"),
            "ashirin wajen tiriliyan dubu ɗari tara casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara casa'in da uku",
        )
        self.assertEqual(
            num2words(99.99, lang="ha"),
            "casa'in da tara wajen tiriliyan dubu tara ɗari takwas casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari tara arba'in da tara",
        )
        self.assertEqual(
            num2words(100.01, lang="ha"),
            "ɗari wajen tiriliyan dubu goma dubu biyar ɗari sha shida",
        )
        self.assertEqual(num2words(100.5, lang="ha"), "ɗari wajen biyar")
        self.assertEqual(
            num2words(123.45, lang="ha"),
            "ɗari ashirin da uku wajen tiriliyan dubu arba'in da biyar ɗari biyu tamanin da huɗu",
        )
        self.assertEqual(num2words(1000.5, lang="ha"), "dubu wajen biyar")
        self.assertEqual(
            num2words(1234.56, lang="ha"),
            "dubu ɗari biyu talatin da huɗu wajen tiriliyan dubu biyar ɗari biyar casa'in da tara biliyan ɗari tara casa'in da tara miliyan ɗari tara casa'in da tara dubu ɗari tara casa'in da tara ɗari huɗu hamsin da huɗu",
        )
        self.assertEqual(
            num2words(10000.01, lang="ha"),
            "dubu goma wajen tiriliyan dubu goma dubu ɗari biyu sha takwas ɗari biyu saba'in da tara",
        )
        self.assertEqual(num2words(-0.5, lang="ha"), "ban sifiri wajen biyar")
        self.assertEqual(num2words(-1.5, lang="ha"), "ban ɗaya wajen biyar")
        self.assertEqual(num2words(-10.5, lang="ha"), "ban goma wajen biyar")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ha", ordinal=True), "na farko")
        self.assertEqual(num2words(2, lang="ha", ordinal=True), "na biyu")
        self.assertEqual(num2words(3, lang="ha", ordinal=True), "na uku")
        self.assertEqual(num2words(4, lang="ha", ordinal=True), "na huɗu")
        self.assertEqual(num2words(5, lang="ha", ordinal=True), "na biyar")
        self.assertEqual(num2words(6, lang="ha", ordinal=True), "na shida")
        self.assertEqual(num2words(7, lang="ha", ordinal=True), "na bakwai")
        self.assertEqual(num2words(8, lang="ha", ordinal=True), "na takwas")
        self.assertEqual(num2words(9, lang="ha", ordinal=True), "na tara")
        self.assertEqual(num2words(10, lang="ha", ordinal=True), "na goma")
        self.assertEqual(num2words(11, lang="ha", ordinal=True), "na sha ɗaya")
        self.assertEqual(num2words(12, lang="ha", ordinal=True), "na sha biyu")
        self.assertEqual(num2words(13, lang="ha", ordinal=True), "na sha uku")
        self.assertEqual(num2words(14, lang="ha", ordinal=True), "na sha huɗu")
        self.assertEqual(num2words(15, lang="ha", ordinal=True), "na sha biyar")
        self.assertEqual(num2words(16, lang="ha", ordinal=True), "na sha shida")
        self.assertEqual(num2words(17, lang="ha", ordinal=True), "na sha bakwai")
        self.assertEqual(num2words(18, lang="ha", ordinal=True), "na sha takwas")
        self.assertEqual(num2words(19, lang="ha", ordinal=True), "na sha tara")
        self.assertEqual(num2words(20, lang="ha", ordinal=True), "na ashirin")
        self.assertEqual(num2words(21, lang="ha", ordinal=True), "na ashirin da ɗaya")
        self.assertEqual(num2words(22, lang="ha", ordinal=True), "na ashirin da biyu")
        self.assertEqual(num2words(25, lang="ha", ordinal=True), "na ashirin da biyar")
        self.assertEqual(num2words(30, lang="ha", ordinal=True), "na talatin")
        self.assertEqual(num2words(40, lang="ha", ordinal=True), "na arba'in")
        self.assertEqual(num2words(50, lang="ha", ordinal=True), "na hamsin")
        self.assertEqual(num2words(60, lang="ha", ordinal=True), "na sittin")
        self.assertEqual(num2words(70, lang="ha", ordinal=True), "na saba'in")
        self.assertEqual(num2words(80, lang="ha", ordinal=True), "na tamanin")
        self.assertEqual(num2words(90, lang="ha", ordinal=True), "na casa'in")
        self.assertEqual(num2words(100, lang="ha", ordinal=True), "na ɗari")
        self.assertEqual(num2words(101, lang="ha", ordinal=True), "na ɗari da ɗaya")
        self.assertEqual(num2words(200, lang="ha", ordinal=True), "na ɗari biyu")
        self.assertEqual(num2words(500, lang="ha", ordinal=True), "na ɗari biyar")
        self.assertEqual(num2words(1000, lang="ha", ordinal=True), "na dubu")
        self.assertEqual(num2words(1001, lang="ha", ordinal=True), "na dubu da ɗaya")
        self.assertEqual(num2words(10000, lang="ha", ordinal=True), "na dubu goma")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="NGN"), "naira sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="NGN"), "kobo ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="NGN"), "kobo hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="NGN"), "naira ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="NGN"),
            "naira ɗaya da kobo hamsin",
        )
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="USD"), "dala sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="USD"), "cent ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="USD"), "cent hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="USD"), "dala ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="USD"),
            "dala ɗaya da cent hamsin",
        )
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="EUR"), "yuro sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="EUR"), "cent ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="EUR"), "cent hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="EUR"), "yuro ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="EUR"),
            "yuro ɗaya da cent hamsin",
        )
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="GBP"), "fam sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="GBP"), "pence ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="GBP"), "pence hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="GBP"), "fam ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="GBP"),
            "fam ɗaya da pence hamsin",
        )
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="JPY"), "yen sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="JPY"), "sen ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="JPY"), "sen hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="JPY"), "yen ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="JPY"),
            "yen ɗaya da sen hamsin",
        )
        self.assertEqual(
            num2words(0, lang="ha", to="currency", currency="CNY"), "yuan sifiri"
        )
        self.assertEqual(
            num2words(0.01, lang="ha", to="currency", currency="CNY"), "fen ɗaya"
        )
        self.assertEqual(
            num2words(0.5, lang="ha", to="currency", currency="CNY"), "fen hamsin"
        )
        self.assertEqual(
            num2words(1, lang="ha", to="currency", currency="CNY"), "yuan ɗaya"
        )
        self.assertEqual(
            num2words(1.5, lang="ha", to="currency", currency="CNY"),
            "yuan ɗaya da fen hamsin",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ha", to="year"), "dubu")
        self.assertEqual(num2words(1066, lang="ha", to="year"), "dubu sittin da shida")
        self.assertEqual(
            num2words(1492, lang="ha", to="year"), "dubu ɗari huɗu casa'in da biyu"
        )
        self.assertEqual(
            num2words(1776, lang="ha", to="year"), "dubu ɗari bakwai saba'in da shida"
        )
        self.assertEqual(num2words(1800, lang="ha", to="year"), "dubu ɗari takwas")
        self.assertEqual(num2words(1900, lang="ha", to="year"), "dubu ɗari tara")
        self.assertEqual(
            num2words(1984, lang="ha", to="year"), "dubu ɗari tara tamanin da huɗu"
        )
        self.assertEqual(
            num2words(1999, lang="ha", to="year"), "dubu ɗari tara casa'in da tara"
        )
        self.assertEqual(num2words(2000, lang="ha", to="year"), "dubu biyu")
        self.assertEqual(num2words(2001, lang="ha", to="year"), "dubu biyu da ɗaya")
        self.assertEqual(num2words(2010, lang="ha", to="year"), "dubu biyu goma")
        self.assertEqual(num2words(2020, lang="ha", to="year"), "dubu biyu ashirin")
        self.assertEqual(
            num2words(2024, lang="ha", to="year"), "dubu biyu ashirin da huɗu"
        )
        self.assertEqual(num2words(2100, lang="ha", to="year"), "dubu biyu ɗari")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ha"), "sifiri")
        self.assertEqual(num2words("1", lang="ha"), "ɗaya")
        self.assertEqual(num2words("10", lang="ha"), "goma")
        self.assertEqual(num2words("100", lang="ha"), "ɗari")
        self.assertEqual(num2words("1000", lang="ha"), "dubu")
        self.assertEqual(num2words("10000", lang="ha"), "dubu goma")
        self.assertEqual(num2words("100000", lang="ha"), "dubu ɗari")
        self.assertEqual(num2words("1000000", lang="ha"), "miliyan")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ha"), "sifiri")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ha"), num2words("100", lang="ha"))
        self.assertEqual(num2words(1000, lang="ha"), num2words("1000", lang="ha"))

        # A fractional ordinal raises TypeError, as in every language (#214).
        with self.assertRaises(TypeError):
            num2words(3.14, lang="ha", ordinal=True)





    def test_more_currency_cases(self):
        """Test additional currency cases."""
        # Test various amounts
        self.assertEqual(
            num2words(2, lang="ha", to="currency", currency="NGN"), "naira biyu"
        )
        self.assertEqual(
            num2words(10, lang="ha", to="currency", currency="NGN"), "naira goma"
        )
        self.assertEqual(
            num2words(100, lang="ha", to="currency", currency="NGN"), "naira ɗari"
        )
        self.assertEqual(
            num2words(1000, lang="ha", to="currency", currency="NGN"), "naira dubu"
        )

        # Test negative currency (single space after "ban", #160)
        self.assertEqual(
            num2words(-1, lang="ha", to="currency", currency="NGN"), "ban naira ɗaya"
        )
        self.assertEqual(
            num2words(-10, lang="ha", to="currency", currency="NGN"), "ban naira goma"
        )
        self.assertEqual(
            num2words(-1.5, lang="ha", to="currency", currency="NGN"),
            "ban naira ɗaya da kobo hamsin",
        )

        # Test currency without cents
        self.assertEqual(
            num2words(100, lang="ha", to="currency", currency="NGN", cents=False),
            "naira ɗari",
        )
        self.assertEqual(
            num2words(100.5, lang="ha", to="currency", currency="NGN", cents=False),
            "naira ɗari",
        )

        # Test unknown currency (should default to NGN)
        self.assertEqual(
            num2words(100, lang="ha", to="currency", currency="XYZ"), "naira ɗari"
        )

    def test_currency_with_fractional_cents(self):
        """Test currency with fractional cents."""
        # Test currency with fractional cents
        self.assertEqual(
            num2words(1.235, lang="ha", to="currency", currency="NGN"),
            "naira ɗaya da kobo ashirin da uku wajen biyar",
        )
        # Note: The fractional cent conversion produces complex output
        result = num2words(10.999, lang="ha", to="currency", currency="NGN")
        self.assertIn("naira goma", result)
        self.assertIn("kobo", result)

    def test_trillion(self):
        """Test trillion numbers."""
        self.assertEqual(num2words(1000000000000, lang="ha"), "tiriliyan")
        self.assertEqual(num2words(1000000000001, lang="ha"), "tiriliyan da ɗaya")
        self.assertEqual(num2words(2000000000000, lang="ha"), "tiriliyan biyu")
        self.assertEqual(
            num2words(1234567890123, lang="ha"),
            "tiriliyan biliyan ɗari biyu talatin da huɗu miliyan ɗari biyar sittin da bakwai dubu ɗari takwas casa'in ɗari ashirin da uku",
        )

