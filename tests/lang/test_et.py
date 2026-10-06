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


class Num2WordsETTest(TestCase):
    """Comprehensive test cases for Estonian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="et"), "null")
        self.assertEqual(num2words(1, lang="et"), "üks")
        self.assertEqual(num2words(2, lang="et"), "kaks")
        self.assertEqual(num2words(3, lang="et"), "kolm")
        self.assertEqual(num2words(4, lang="et"), "neli")
        self.assertEqual(num2words(5, lang="et"), "viis")
        self.assertEqual(num2words(6, lang="et"), "kuus")
        self.assertEqual(num2words(7, lang="et"), "seitse")
        self.assertEqual(num2words(8, lang="et"), "kaheksa")
        self.assertEqual(num2words(9, lang="et"), "üheksa")
        self.assertEqual(num2words(10, lang="et"), "kümme")
        self.assertEqual(num2words(11, lang="et"), "üksteist")
        self.assertEqual(num2words(12, lang="et"), "kaksteist")
        self.assertEqual(num2words(13, lang="et"), "kolmteist")
        self.assertEqual(num2words(14, lang="et"), "neliteist")
        self.assertEqual(num2words(15, lang="et"), "viisteist")
        self.assertEqual(num2words(16, lang="et"), "kuusteist")
        self.assertEqual(num2words(17, lang="et"), "seitseteist")
        self.assertEqual(num2words(18, lang="et"), "kaheksateist")
        self.assertEqual(num2words(19, lang="et"), "üheksateist")
        self.assertEqual(num2words(20, lang="et"), "kakskümmend")
        self.assertEqual(num2words(21, lang="et"), "kakskümmend üks")
        self.assertEqual(num2words(22, lang="et"), "kakskümmend kaks")
        self.assertEqual(num2words(23, lang="et"), "kakskümmend kolm")
        self.assertEqual(num2words(24, lang="et"), "kakskümmend neli")
        self.assertEqual(num2words(25, lang="et"), "kakskümmend viis")
        self.assertEqual(num2words(26, lang="et"), "kakskümmend kuus")
        self.assertEqual(num2words(27, lang="et"), "kakskümmend seitse")
        self.assertEqual(num2words(28, lang="et"), "kakskümmend kaheksa")
        self.assertEqual(num2words(29, lang="et"), "kakskümmend üheksa")
        self.assertEqual(num2words(30, lang="et"), "kolmkümmend")
        self.assertEqual(num2words(31, lang="et"), "kolmkümmend üks")
        self.assertEqual(num2words(35, lang="et"), "kolmkümmend viis")
        self.assertEqual(num2words(40, lang="et"), "nelikümmend")
        self.assertEqual(num2words(45, lang="et"), "nelikümmend viis")
        self.assertEqual(num2words(50, lang="et"), "viiskümmend")
        self.assertEqual(num2words(55, lang="et"), "viiskümmend viis")
        self.assertEqual(num2words(60, lang="et"), "kuuskümmend")
        self.assertEqual(num2words(65, lang="et"), "kuuskümmend viis")
        self.assertEqual(num2words(70, lang="et"), "seitsekümmend")
        self.assertEqual(num2words(75, lang="et"), "seitsekümmend viis")
        self.assertEqual(num2words(80, lang="et"), "kaheksakümmend")
        self.assertEqual(num2words(85, lang="et"), "kaheksakümmend viis")
        self.assertEqual(num2words(90, lang="et"), "üheksakümmend")
        self.assertEqual(num2words(95, lang="et"), "üheksakümmend viis")
        self.assertEqual(num2words(99, lang="et"), "üheksakümmend üheksa")
        self.assertEqual(num2words(100, lang="et"), "ükssada")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="et"), "ükssada üks")
        self.assertEqual(num2words(110, lang="et"), "ükssada kümme")
        self.assertEqual(num2words(111, lang="et"), "ükssada üksteist")
        self.assertEqual(num2words(120, lang="et"), "ükssada kakskümmend")
        self.assertEqual(num2words(125, lang="et"), "ükssada kakskümmend viis")
        self.assertEqual(num2words(150, lang="et"), "ükssada viiskümmend")
        self.assertEqual(num2words(175, lang="et"), "ükssada seitsekümmend viis")
        self.assertEqual(num2words(199, lang="et"), "ükssada üheksakümmend üheksa")
        self.assertEqual(num2words(200, lang="et"), "kakssada")
        self.assertEqual(num2words(201, lang="et"), "kakssada üks")
        self.assertEqual(num2words(210, lang="et"), "kakssada kümme")
        self.assertEqual(num2words(220, lang="et"), "kakssada kakskümmend")
        self.assertEqual(num2words(250, lang="et"), "kakssada viiskümmend")
        self.assertEqual(num2words(299, lang="et"), "kakssada üheksakümmend üheksa")
        self.assertEqual(num2words(300, lang="et"), "kolmsada")
        self.assertEqual(num2words(333, lang="et"), "kolmsada kolmkümmend kolm")
        self.assertEqual(num2words(400, lang="et"), "nelisada")
        self.assertEqual(num2words(444, lang="et"), "nelisada nelikümmend neli")
        self.assertEqual(num2words(500, lang="et"), "viissada")
        self.assertEqual(num2words(555, lang="et"), "viissada viiskümmend viis")
        self.assertEqual(num2words(600, lang="et"), "kuussada")
        self.assertEqual(num2words(666, lang="et"), "kuussada kuuskümmend kuus")
        self.assertEqual(num2words(700, lang="et"), "seitsesada")
        self.assertEqual(num2words(777, lang="et"), "seitsesada seitsekümmend seitse")
        self.assertEqual(num2words(800, lang="et"), "kaheksasada")
        self.assertEqual(
            num2words(888, lang="et"), "kaheksasada kaheksakümmend kaheksa"
        )
        self.assertEqual(num2words(900, lang="et"), "üheksasada")
        self.assertEqual(num2words(999, lang="et"), "üheksasada üheksakümmend üheksa")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="et"), "tuhat")
        self.assertEqual(num2words(1001, lang="et"), "tuhat üks")
        self.assertEqual(num2words(1010, lang="et"), "tuhat kümme")
        self.assertEqual(num2words(1100, lang="et"), "tuhat ükssada")
        self.assertEqual(num2words(1111, lang="et"), "tuhat ükssada üksteist")
        self.assertEqual(num2words(1234, lang="et"), "tuhat kakssada kolmkümmend neli")
        self.assertEqual(num2words(1500, lang="et"), "tuhat viissada")
        self.assertEqual(
            num2words(1999, lang="et"), "tuhat üheksasada üheksakümmend üheksa"
        )
        self.assertEqual(num2words(2000, lang="et"), "kaks tuhat")
        self.assertEqual(num2words(2001, lang="et"), "kaks tuhat üks")
        self.assertEqual(num2words(2020, lang="et"), "kaks tuhat kakskümmend")
        self.assertEqual(
            num2words(2222, lang="et"), "kaks tuhat kakssada kakskümmend kaks"
        )
        self.assertEqual(num2words(3000, lang="et"), "kolm tuhat")
        self.assertEqual(
            num2words(3333, lang="et"), "kolm tuhat kolmsada kolmkümmend kolm"
        )
        self.assertEqual(num2words(4000, lang="et"), "neli tuhat")
        self.assertEqual(
            num2words(4444, lang="et"), "neli tuhat nelisada nelikümmend neli"
        )
        self.assertEqual(num2words(5000, lang="et"), "viis tuhat")
        self.assertEqual(
            num2words(5555, lang="et"), "viis tuhat viissada viiskümmend viis"
        )
        self.assertEqual(num2words(6000, lang="et"), "kuus tuhat")
        self.assertEqual(
            num2words(6666, lang="et"), "kuus tuhat kuussada kuuskümmend kuus"
        )
        self.assertEqual(num2words(7000, lang="et"), "seitse tuhat")
        self.assertEqual(
            num2words(7777, lang="et"), "seitse tuhat seitsesada seitsekümmend seitse"
        )
        self.assertEqual(num2words(8000, lang="et"), "kaheksa tuhat")
        self.assertEqual(
            num2words(8888, lang="et"),
            "kaheksa tuhat kaheksasada kaheksakümmend kaheksa",
        )
        self.assertEqual(num2words(9000, lang="et"), "üheksa tuhat")
        self.assertEqual(
            num2words(9999, lang="et"), "üheksa tuhat üheksasada üheksakümmend üheksa"
        )
        self.assertEqual(num2words(10000, lang="et"), "kümme tuhat")
        self.assertEqual(num2words(10001, lang="et"), "kümme tuhat üks")
        self.assertEqual(num2words(11111, lang="et"), "üksteist tuhat ükssada üksteist")
        self.assertEqual(
            num2words(12345, lang="et"), "kaksteist tuhat kolmsada nelikümmend viis"
        )
        self.assertEqual(num2words(20000, lang="et"), "kakskümmend tuhat")
        self.assertEqual(num2words(50000, lang="et"), "viiskümmend tuhat")
        self.assertEqual(
            num2words(99999, lang="et"),
            "üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(100000, lang="et"), "sada tuhat")
        self.assertEqual(
            num2words(123456, lang="et"),
            "ükssada kakskümmend kolm tuhat nelisada viiskümmend kuus",
        )
        self.assertEqual(num2words(200000, lang="et"), "kakssada tuhat")
        self.assertEqual(num2words(500000, lang="et"), "viissada tuhat")
        self.assertEqual(
            num2words(654321, lang="et"),
            "kuussada viiskümmend neli tuhat kolmsada kakskümmend üks",
        )
        self.assertEqual(
            num2words(999999, lang="et"),
            "üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="et"), "üks miljon")
        self.assertEqual(num2words(1000001, lang="et"), "üks miljon üks")
        self.assertEqual(
            num2words(1111111, lang="et"),
            "üks miljon ükssada üksteist tuhat ükssada üksteist",
        )
        self.assertEqual(
            num2words(1234567, lang="et"),
            "üks miljon kakssada kolmkümmend neli tuhat viissada kuuskümmend seitse",
        )
        self.assertEqual(num2words(2000000, lang="et"), "kaks miljonit")
        self.assertEqual(num2words(5000000, lang="et"), "viis miljonit")
        self.assertEqual(
            num2words(9999999, lang="et"),
            "üheksa miljonit üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(10000000, lang="et"), "kümme miljonit")
        self.assertEqual(
            num2words(12345678, lang="et"),
            "kaksteist miljonit kolmsada nelikümmend viis tuhat kuussada seitsekümmend kaheksa",
        )
        self.assertEqual(
            num2words(99999999, lang="et"),
            "üheksakümmend üheksa miljonit üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(100000000, lang="et"), "ükssada miljonit")
        self.assertEqual(
            num2words(123456789, lang="et"),
            "ükssada kakskümmend kolm miljonit nelisada viiskümmend kuus tuhat seitsesada kaheksakümmend üheksa",
        )
        self.assertEqual(
            num2words(999999999, lang="et"),
            "üheksasada üheksakümmend üheksa miljonit üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(1000000000, lang="et"), "üks miljard")
        self.assertEqual(
            num2words(1234567890, lang="et"),
            "üks miljard kakssada kolmkümmend neli miljonit viissada kuuskümmend seitse tuhat kaheksasada üheksakümmend",
        )
        self.assertEqual(
            num2words(9999999999, lang="et"),
            "üheksa miljardit üheksasada üheksakümmend üheksa miljonit üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(10000000000, lang="et"), "kümme miljardit")
        self.assertEqual(
            num2words(99999999999, lang="et"),
            "üheksakümmend üheksa miljardit üheksasada üheksakümmend üheksa miljonit üheksasada üheksakümmend üheksa tuhat üheksasada üheksakümmend üheksa",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="et"), "miinus üks")
        self.assertEqual(num2words(-2, lang="et"), "miinus kaks")
        self.assertEqual(num2words(-5, lang="et"), "miinus viis")
        self.assertEqual(num2words(-10, lang="et"), "miinus kümme")
        self.assertEqual(num2words(-11, lang="et"), "miinus üksteist")
        self.assertEqual(num2words(-20, lang="et"), "miinus kakskümmend")
        self.assertEqual(num2words(-50, lang="et"), "miinus viiskümmend")
        self.assertEqual(num2words(-99, lang="et"), "miinus üheksakümmend üheksa")
        self.assertEqual(num2words(-100, lang="et"), "miinus ükssada")
        self.assertEqual(num2words(-101, lang="et"), "miinus ükssada üks")
        self.assertEqual(num2words(-200, lang="et"), "miinus kakssada")
        self.assertEqual(
            num2words(-999, lang="et"), "miinus üheksasada üheksakümmend üheksa"
        )
        self.assertEqual(num2words(-1000, lang="et"), "miinus tuhat")
        self.assertEqual(num2words(-1001, lang="et"), "miinus tuhat üks")
        self.assertEqual(num2words(-10000, lang="et"), "miinus kümme tuhat")
        self.assertEqual(num2words(-100000, lang="et"), "miinus sada tuhat")
        self.assertEqual(num2words(-1000000, lang="et"), "miinus üks miljon")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="et"), "null koma üks")
        self.assertEqual(num2words(0.5, lang="et"), "null koma viis")
        self.assertEqual(num2words(0.9, lang="et"), "null koma üheksa")
        self.assertEqual(num2words(1.1, lang="et"), "üks koma üks")
        self.assertEqual(num2words(1.5, lang="et"), "üks koma viis")
        self.assertEqual(num2words(2.5, lang="et"), "kaks koma viis")
        self.assertEqual(num2words(3.14, lang="et"), "kolm koma üks neli")
        self.assertEqual(num2words(10.5, lang="et"), "kümme koma viis")
        self.assertEqual(num2words(11.11, lang="et"), "üksteist koma üks üks")
        self.assertEqual(num2words(20.2, lang="et"), "kakskümmend koma kaks")
        self.assertEqual(
            num2words(99.99, lang="et"), "üheksakümmend üheksa koma üheksa üheksa"
        )
        self.assertEqual(num2words(100.01, lang="et"), "ükssada koma null üks")
        self.assertEqual(num2words(100.5, lang="et"), "ükssada koma viis")
        self.assertEqual(
            num2words(123.45, lang="et"), "ükssada kakskümmend kolm koma neli viis"
        )
        self.assertEqual(num2words(1000.5, lang="et"), "tuhat koma viis")
        self.assertEqual(
            num2words(1234.56, lang="et"),
            "tuhat kakssada kolmkümmend neli koma viis kuus",
        )
        self.assertEqual(num2words(10000.01, lang="et"), "kümme tuhat koma null üks")
        self.assertEqual(num2words(-0.5, lang="et"), "miinus null koma viis")
        self.assertEqual(num2words(-1.5, lang="et"), "miinus üks koma viis")
        self.assertEqual(num2words(-10.5, lang="et"), "miinus kümme koma viis")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="et", ordinal=True), "esimene")
        self.assertEqual(num2words(2, lang="et", ordinal=True), "teine")
        self.assertEqual(num2words(3, lang="et", ordinal=True), "kolmas")
        self.assertEqual(num2words(4, lang="et", ordinal=True), "neljas")
        self.assertEqual(num2words(5, lang="et", ordinal=True), "viies")
        self.assertEqual(num2words(6, lang="et", ordinal=True), "kuues")
        self.assertEqual(num2words(7, lang="et", ordinal=True), "seitsmes")
        self.assertEqual(num2words(8, lang="et", ordinal=True), "kaheksas")
        self.assertEqual(num2words(9, lang="et", ordinal=True), "üheksas")
        self.assertEqual(num2words(10, lang="et", ordinal=True), "kümnes")
        self.assertEqual(num2words(11, lang="et", ordinal=True), "üheteistkümnes")
        self.assertEqual(num2words(12, lang="et", ordinal=True), "kaheteistkümnes")
        self.assertEqual(num2words(13, lang="et", ordinal=True), "kolmeteistkümnes")
        self.assertEqual(num2words(14, lang="et", ordinal=True), "neljateistkümnes")
        self.assertEqual(num2words(15, lang="et", ordinal=True), "viieteistkümnes")
        self.assertEqual(num2words(16, lang="et", ordinal=True), "kuueteistkümnes")
        self.assertEqual(num2words(17, lang="et", ordinal=True), "seitsmeteistkümnes")
        self.assertEqual(num2words(18, lang="et", ordinal=True), "kaheksateistkümnes")
        self.assertEqual(num2words(19, lang="et", ordinal=True), "üheksateistkümnes")
        self.assertEqual(num2words(20, lang="et", ordinal=True), "kahekümnes")
        self.assertEqual(num2words(21, lang="et", ordinal=True), "kakskümmend esimene")
        self.assertEqual(num2words(22, lang="et", ordinal=True), "kakskümmend teine")
        self.assertEqual(num2words(25, lang="et", ordinal=True), "kakskümmend viies")
        self.assertEqual(num2words(30, lang="et", ordinal=True), "kolmekümnes")
        self.assertEqual(num2words(40, lang="et", ordinal=True), "nelikümnes")
        self.assertEqual(num2words(50, lang="et", ordinal=True), "viiekümnes")
        self.assertEqual(num2words(60, lang="et", ordinal=True), "kuuekümnes")
        self.assertEqual(num2words(70, lang="et", ordinal=True), "seitsmekümnes")
        self.assertEqual(num2words(80, lang="et", ordinal=True), "kaheksakümnes")
        self.assertEqual(num2words(90, lang="et", ordinal=True), "üheksakümnes")
        self.assertEqual(num2words(100, lang="et", ordinal=True), "sajas")
        self.assertEqual(num2words(101, lang="et", ordinal=True), "ükssada ükss")
        self.assertEqual(num2words(200, lang="et", ordinal=True), "kakssadas")
        self.assertEqual(num2words(500, lang="et", ordinal=True), "viissadas")
        self.assertEqual(num2words(1000, lang="et", ordinal=True), "tuhandes")
        self.assertEqual(num2words(1001, lang="et", ordinal=True), "tuhat ükss")
        self.assertEqual(num2words(10000, lang="et", ordinal=True), "kümme tuhats")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="EUR"), "null eurot"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="EUR"),
            "null eurot ja üks sent",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="EUR"),
            "null eurot ja viiskümmend senti",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="EUR"), "üks euro"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="EUR"),
            "üks euro ja viiskümmend senti",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="USD"), "null dollarit"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="USD"),
            "null dollarit ja üks sent",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="USD"),
            "null dollarit ja viiskümmend senti",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="USD"), "üks dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="USD"),
            "üks dollar ja viiskümmend senti",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="GBP"), "null naela"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="GBP"),
            "null naela ja üks penn",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="GBP"),
            "null naela ja viiskümmend penni",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="GBP"), "üks nael"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="GBP"),
            "üks nael ja viiskümmend penni",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="SEK"), "null krooni"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="SEK"),
            "null krooni ja üks ööri",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="SEK"),
            "null krooni ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="SEK"), "üks kroon"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="SEK"),
            "üks kroon ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="NOK"), "null krooni"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="NOK"),
            "null krooni ja üks ööri",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="NOK"),
            "null krooni ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="NOK"), "üks kroon"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="NOK"),
            "üks kroon ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="DKK"), "null krooni"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="DKK"),
            "null krooni ja üks ööri",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="DKK"),
            "null krooni ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="DKK"), "üks kroon"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="DKK"),
            "üks kroon ja viiskümmend ööri",
        )
        self.assertEqual(
            num2words(0, lang="et", to="currency", currency="RUB"), "null rubla"
        )
        self.assertEqual(
            num2words(0.01, lang="et", to="currency", currency="RUB"),
            "null rubla ja üks kopikas",
        )
        self.assertEqual(
            num2words(0.5, lang="et", to="currency", currency="RUB"),
            "null rubla ja viiskümmend kopikat",
        )
        self.assertEqual(
            num2words(1, lang="et", to="currency", currency="RUB"), "üks rubla"
        )
        self.assertEqual(
            num2words(1.5, lang="et", to="currency", currency="RUB"),
            "üks rubla ja viiskümmend kopikat",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="et", to="year"), "tuhat")
        self.assertEqual(
            num2words(1066, lang="et", to="year"), "tuhat kuuskümmend kuus"
        )
        self.assertEqual(
            num2words(1492, lang="et", to="year"), "tuhat nelisada üheksakümmend kaks"
        )
        self.assertEqual(
            num2words(1776, lang="et", to="year"), "tuhat seitsesada seitsekümmend kuus"
        )
        self.assertEqual(num2words(1800, lang="et", to="year"), "tuhat kaheksasada")
        self.assertEqual(num2words(1900, lang="et", to="year"), "tuhat üheksasada")
        self.assertEqual(
            num2words(1984, lang="et", to="year"),
            "tuhat üheksasada kaheksakümmend neli",
        )
        self.assertEqual(
            num2words(1999, lang="et", to="year"),
            "tuhat üheksasada üheksakümmend üheksa",
        )
        self.assertEqual(num2words(2000, lang="et", to="year"), "kaks tuhat")
        self.assertEqual(num2words(2001, lang="et", to="year"), "kaks tuhat üks")
        self.assertEqual(num2words(2010, lang="et", to="year"), "kaks tuhat kümme")
        self.assertEqual(
            num2words(2020, lang="et", to="year"), "kaks tuhat kakskümmend"
        )
        self.assertEqual(
            num2words(2024, lang="et", to="year"), "kaks tuhat kakskümmend neli"
        )
        self.assertEqual(num2words(2100, lang="et", to="year"), "kaks tuhat ükssada")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="et"), "null")
        self.assertEqual(num2words("1", lang="et"), "üks")
        self.assertEqual(num2words("10", lang="et"), "kümme")
        self.assertEqual(num2words("100", lang="et"), "ükssada")
        self.assertEqual(num2words("1000", lang="et"), "tuhat")
        self.assertEqual(num2words("10000", lang="et"), "kümme tuhat")
        self.assertEqual(num2words("100000", lang="et"), "sada tuhat")
        self.assertEqual(num2words("1000000", lang="et"), "üks miljon")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="et"), "null")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="et"), num2words("100", lang="et"))
        self.assertEqual(num2words(1000, lang="et"), num2words("1000", lang="et"))


    def test_negative_ordinal_raises(self):
        # gladiaio/num2words2#155: Python list indexing wrapped -1 to
        # "üheksas" (ninth), -10 to "" and crashed with IndexError below that.
        for n in (-1, -2, -10, -11, -42, -1000):
            with self.assertRaisesRegex(TypeError, "Cannot treat negative"):
                num2words(n, lang="et", to="ordinal")
        # A fractional value is a TypeError whatever its sign (#214).
        for n in (-0.5, -1.5):
            with self.assertRaisesRegex(TypeError, "Cannot treat float"):
                num2words(n, lang="et", to="ordinal")
        self.assertEqual(num2words(9, lang="et", to="ordinal"), "üheksas")
