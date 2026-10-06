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


class Num2WordsSQTest(TestCase):
    """Comprehensive test cases for Albanian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sq"), "zero")
        self.assertEqual(num2words(1, lang="sq"), "një")
        self.assertEqual(num2words(2, lang="sq"), "dy")
        self.assertEqual(num2words(3, lang="sq"), "tre")
        self.assertEqual(num2words(4, lang="sq"), "katër")
        self.assertEqual(num2words(5, lang="sq"), "pesë")
        self.assertEqual(num2words(6, lang="sq"), "gjashtë")
        self.assertEqual(num2words(7, lang="sq"), "shtatë")
        self.assertEqual(num2words(8, lang="sq"), "tetë")
        self.assertEqual(num2words(9, lang="sq"), "nëntë")
        self.assertEqual(num2words(10, lang="sq"), "dhjetë")
        self.assertEqual(num2words(11, lang="sq"), "njëmbëdhjetë")
        self.assertEqual(num2words(12, lang="sq"), "dymbëdhjetë")
        self.assertEqual(num2words(13, lang="sq"), "trembëdhjetë")
        self.assertEqual(num2words(14, lang="sq"), "katërmbëdhjetë")
        self.assertEqual(num2words(15, lang="sq"), "pesëmbëdhjetë")
        self.assertEqual(num2words(16, lang="sq"), "gjashtëmbëdhjetë")
        self.assertEqual(num2words(17, lang="sq"), "shtatëmbëdhjetë")
        self.assertEqual(num2words(18, lang="sq"), "tetëmbëdhjetë")
        self.assertEqual(num2words(19, lang="sq"), "nëntëmbëdhjetë")
        self.assertEqual(num2words(20, lang="sq"), "njëzet")
        self.assertEqual(num2words(21, lang="sq"), "njëzet e një")
        self.assertEqual(num2words(22, lang="sq"), "njëzet e dy")
        self.assertEqual(num2words(23, lang="sq"), "njëzet e tre")
        self.assertEqual(num2words(24, lang="sq"), "njëzet e katër")
        self.assertEqual(num2words(25, lang="sq"), "njëzet e pesë")
        self.assertEqual(num2words(26, lang="sq"), "njëzet e gjashtë")
        self.assertEqual(num2words(27, lang="sq"), "njëzet e shtatë")
        self.assertEqual(num2words(28, lang="sq"), "njëzet e tetë")
        self.assertEqual(num2words(29, lang="sq"), "njëzet e nëntë")
        self.assertEqual(num2words(30, lang="sq"), "tridhjetë")
        self.assertEqual(num2words(31, lang="sq"), "tridhjetë e një")
        self.assertEqual(num2words(35, lang="sq"), "tridhjetë e pesë")
        self.assertEqual(num2words(40, lang="sq"), "dyzet")
        self.assertEqual(num2words(45, lang="sq"), "dyzet e pesë")
        self.assertEqual(num2words(50, lang="sq"), "pesëdhjetë")
        self.assertEqual(num2words(55, lang="sq"), "pesëdhjetë e pesë")
        self.assertEqual(num2words(60, lang="sq"), "gjashtëdhjetë")
        self.assertEqual(num2words(65, lang="sq"), "gjashtëdhjetë e pesë")
        self.assertEqual(num2words(70, lang="sq"), "shtatëdhjetë")
        self.assertEqual(num2words(75, lang="sq"), "shtatëdhjetë e pesë")
        self.assertEqual(num2words(80, lang="sq"), "tetëdhjetë")
        self.assertEqual(num2words(85, lang="sq"), "tetëdhjetë e pesë")
        self.assertEqual(num2words(90, lang="sq"), "nëntëdhjetë")
        self.assertEqual(num2words(95, lang="sq"), "nëntëdhjetë e pesë")
        self.assertEqual(num2words(99, lang="sq"), "nëntëdhjetë e nëntë")
        self.assertEqual(num2words(100, lang="sq"), "njëqind")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sq"), "njëqind e një")
        self.assertEqual(num2words(110, lang="sq"), "njëqind e dhjetë")
        self.assertEqual(num2words(111, lang="sq"), "njëqind e njëmbëdhjetë")
        self.assertEqual(num2words(120, lang="sq"), "njëqind e njëzet")
        self.assertEqual(num2words(125, lang="sq"), "njëqind e njëzet e pesë")
        self.assertEqual(num2words(150, lang="sq"), "njëqind e pesëdhjetë")
        self.assertEqual(num2words(175, lang="sq"), "njëqind e shtatëdhjetë e pesë")
        self.assertEqual(num2words(199, lang="sq"), "njëqind e nëntëdhjetë e nëntë")
        self.assertEqual(num2words(200, lang="sq"), "dy qind")
        self.assertEqual(num2words(201, lang="sq"), "dy qind e një")
        self.assertEqual(num2words(210, lang="sq"), "dy qind e dhjetë")
        self.assertEqual(num2words(220, lang="sq"), "dy qind e njëzet")
        self.assertEqual(num2words(250, lang="sq"), "dy qind e pesëdhjetë")
        self.assertEqual(num2words(299, lang="sq"), "dy qind e nëntëdhjetë e nëntë")
        self.assertEqual(num2words(300, lang="sq"), "tre qind")
        self.assertEqual(num2words(333, lang="sq"), "tre qind e tridhjetë e tre")
        self.assertEqual(num2words(400, lang="sq"), "katër qind")
        self.assertEqual(num2words(444, lang="sq"), "katër qind e dyzet e katër")
        self.assertEqual(num2words(500, lang="sq"), "pesë qind")
        self.assertEqual(num2words(555, lang="sq"), "pesë qind e pesëdhjetë e pesë")
        self.assertEqual(num2words(600, lang="sq"), "gjashtë qind")
        self.assertEqual(
            num2words(666, lang="sq"), "gjashtë qind e gjashtëdhjetë e gjashtë"
        )
        self.assertEqual(num2words(700, lang="sq"), "shtatë qind")
        self.assertEqual(
            num2words(777, lang="sq"), "shtatë qind e shtatëdhjetë e shtatë"
        )
        self.assertEqual(num2words(800, lang="sq"), "tetë qind")
        self.assertEqual(num2words(888, lang="sq"), "tetë qind e tetëdhjetë e tetë")
        self.assertEqual(num2words(900, lang="sq"), "nëntë qind")
        self.assertEqual(num2words(999, lang="sq"), "nëntë qind e nëntëdhjetë e nëntë")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sq"), "një mijë")
        self.assertEqual(num2words(1001, lang="sq"), "një mijë e një")
        self.assertEqual(num2words(1010, lang="sq"), "një mijë e dhjetë")
        self.assertEqual(num2words(1100, lang="sq"), "një mijë e njëqind")
        self.assertEqual(
            num2words(1111, lang="sq"), "një mijë e njëqind e njëmbëdhjetë"
        )
        self.assertEqual(
            num2words(1234, lang="sq"), "një mijë e dy qind e tridhjetë e katër"
        )
        self.assertEqual(num2words(1500, lang="sq"), "një mijë e pesë qind")
        self.assertEqual(
            num2words(1999, lang="sq"), "një mijë e nëntë qind e nëntëdhjetë e nëntë"
        )
        self.assertEqual(num2words(2000, lang="sq"), "dy mijë")
        self.assertEqual(num2words(2001, lang="sq"), "dy mijë e një")
        self.assertEqual(num2words(2020, lang="sq"), "dy mijë e njëzet")
        self.assertEqual(num2words(2222, lang="sq"), "dy mijë e dy qind e njëzet e dy")
        self.assertEqual(num2words(3000, lang="sq"), "tre mijë")
        self.assertEqual(
            num2words(3333, lang="sq"), "tre mijë e tre qind e tridhjetë e tre"
        )
        self.assertEqual(num2words(4000, lang="sq"), "katër mijë")
        self.assertEqual(
            num2words(4444, lang="sq"), "katër mijë e katër qind e dyzet e katër"
        )
        self.assertEqual(num2words(5000, lang="sq"), "pesë mijë")
        self.assertEqual(
            num2words(5555, lang="sq"), "pesë mijë e pesë qind e pesëdhjetë e pesë"
        )
        self.assertEqual(num2words(6000, lang="sq"), "gjashtë mijë")
        self.assertEqual(
            num2words(6666, lang="sq"),
            "gjashtë mijë e gjashtë qind e gjashtëdhjetë e gjashtë",
        )
        self.assertEqual(num2words(7000, lang="sq"), "shtatë mijë")
        self.assertEqual(
            num2words(7777, lang="sq"),
            "shtatë mijë e shtatë qind e shtatëdhjetë e shtatë",
        )
        self.assertEqual(num2words(8000, lang="sq"), "tetë mijë")
        self.assertEqual(
            num2words(8888, lang="sq"), "tetë mijë e tetë qind e tetëdhjetë e tetë"
        )
        self.assertEqual(num2words(9000, lang="sq"), "nëntë mijë")
        self.assertEqual(
            num2words(9999, lang="sq"), "nëntë mijë e nëntë qind e nëntëdhjetë e nëntë"
        )
        self.assertEqual(num2words(10000, lang="sq"), "dhjetë mijë")
        self.assertEqual(num2words(10001, lang="sq"), "dhjetë mijë e një")
        self.assertEqual(
            num2words(11111, lang="sq"), "njëmbëdhjetë mijë e njëqind e njëmbëdhjetë"
        )
        self.assertEqual(
            num2words(12345, lang="sq"), "dymbëdhjetë mijë e tre qind e dyzet e pesë"
        )
        self.assertEqual(num2words(20000, lang="sq"), "njëzet mijë")
        self.assertEqual(num2words(50000, lang="sq"), "pesëdhjetë mijë")
        self.assertEqual(
            num2words(99999, lang="sq"),
            "nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(100000, lang="sq"), "njëqind mijë")
        self.assertEqual(
            num2words(123456, lang="sq"),
            "njëqind e njëzet e tre mijë e katër qind e pesëdhjetë e gjashtë",
        )
        self.assertEqual(num2words(200000, lang="sq"), "dy qind mijë")
        self.assertEqual(num2words(500000, lang="sq"), "pesë qind mijë")
        self.assertEqual(
            num2words(654321, lang="sq"),
            "gjashtë qind e pesëdhjetë e katër mijë e tre qind e njëzet e një",
        )
        self.assertEqual(
            num2words(999999, lang="sq"),
            "nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sq"), "një milion")
        self.assertEqual(num2words(1000001, lang="sq"), "një milion e një")
        self.assertEqual(
            num2words(1111111, lang="sq"),
            "një milion e njëqind e njëmbëdhjetë mijë e njëqind e njëmbëdhjetë",
        )
        self.assertEqual(
            num2words(1234567, lang="sq"),
            "një milion e dy qind e tridhjetë e katër mijë e pesë qind e gjashtëdhjetë e shtatë",
        )
        self.assertEqual(num2words(2000000, lang="sq"), "dy milion")
        self.assertEqual(num2words(5000000, lang="sq"), "pesë milion")
        self.assertEqual(
            num2words(9999999, lang="sq"),
            "nëntë milion e nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(10000000, lang="sq"), "dhjetë milion")
        self.assertEqual(
            num2words(12345678, lang="sq"),
            "dymbëdhjetë milion e tre qind e dyzet e pesë mijë e gjashtë qind e shtatëdhjetë e tetë",
        )
        self.assertEqual(
            num2words(99999999, lang="sq"),
            "nëntëdhjetë e nëntë milion e nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(100000000, lang="sq"), "njëqind milion")
        self.assertEqual(
            num2words(123456789, lang="sq"),
            "njëqind e njëzet e tre milion e katër qind e pesëdhjetë e gjashtë mijë e shtatë qind e tetëdhjetë e nëntë",
        )
        self.assertEqual(
            num2words(999999999, lang="sq"),
            "nëntë qind e nëntëdhjetë e nëntë milion e nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(1000000000, lang="sq"), "një miliard")
        self.assertEqual(
            num2words(1234567890, lang="sq"),
            "një miliard e dy qind e tridhjetë e katër milion e pesë qind e gjashtëdhjetë e shtatë mijë e tetë qind e nëntëdhjetë",
        )
        self.assertEqual(
            num2words(9999999999, lang="sq"),
            "nëntë miliard e nëntë qind e nëntëdhjetë e nëntë milion e nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(10000000000, lang="sq"), "dhjetë miliard")
        self.assertEqual(
            num2words(99999999999, lang="sq"),
            "nëntëdhjetë e nëntë miliard e nëntë qind e nëntëdhjetë e nëntë milion e nëntë qind e nëntëdhjetë e nëntë mijë e nëntë qind e nëntëdhjetë e nëntë",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sq"), "minus një")
        self.assertEqual(num2words(-2, lang="sq"), "minus dy")
        self.assertEqual(num2words(-5, lang="sq"), "minus pesë")
        self.assertEqual(num2words(-10, lang="sq"), "minus dhjetë")
        self.assertEqual(num2words(-11, lang="sq"), "minus njëmbëdhjetë")
        self.assertEqual(num2words(-20, lang="sq"), "minus njëzet")
        self.assertEqual(num2words(-50, lang="sq"), "minus pesëdhjetë")
        self.assertEqual(num2words(-99, lang="sq"), "minus nëntëdhjetë e nëntë")
        self.assertEqual(num2words(-100, lang="sq"), "minus njëqind")
        self.assertEqual(num2words(-101, lang="sq"), "minus njëqind e një")
        self.assertEqual(num2words(-200, lang="sq"), "minus dy qind")
        self.assertEqual(
            num2words(-999, lang="sq"), "minus nëntë qind e nëntëdhjetë e nëntë"
        )
        self.assertEqual(num2words(-1000, lang="sq"), "minus një mijë")
        self.assertEqual(num2words(-1001, lang="sq"), "minus një mijë e një")
        self.assertEqual(num2words(-10000, lang="sq"), "minus dhjetë mijë")
        self.assertEqual(num2words(-100000, lang="sq"), "minus njëqind mijë")
        self.assertEqual(num2words(-1000000, lang="sq"), "minus një milion")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sq"), "zero presje një")
        self.assertEqual(num2words(0.5, lang="sq"), "zero presje pesë")
        self.assertEqual(num2words(0.9, lang="sq"), "zero presje nëntë")
        self.assertEqual(num2words(1.1, lang="sq"), "një presje një")
        self.assertEqual(num2words(1.5, lang="sq"), "një presje pesë")
        self.assertEqual(num2words(2.5, lang="sq"), "dy presje pesë")
        self.assertEqual(num2words(3.14, lang="sq"), "tre presje një katër")
        self.assertEqual(num2words(10.5, lang="sq"), "dhjetë presje pesë")
        self.assertEqual(num2words(11.11, lang="sq"), "njëmbëdhjetë presje një një")
        self.assertEqual(num2words(20.2, lang="sq"), "njëzet presje dy")
        self.assertEqual(
            num2words(99.99, lang="sq"), "nëntëdhjetë e nëntë presje nëntë nëntë"
        )
        self.assertEqual(num2words(100.01, lang="sq"), "njëqind presje zero një")
        self.assertEqual(num2words(100.5, lang="sq"), "njëqind presje pesë")
        self.assertEqual(
            num2words(123.45, lang="sq"), "njëqind e njëzet e tre presje katër pesë"
        )
        self.assertEqual(num2words(1000.5, lang="sq"), "një mijë presje pesë")
        self.assertEqual(
            num2words(1234.56, lang="sq"),
            "një mijë e dy qind e tridhjetë e katër presje pesë gjashtë",
        )
        self.assertEqual(num2words(10000.01, lang="sq"), "dhjetë mijë presje zero një")
        self.assertEqual(num2words(-0.5, lang="sq"), "minus zero presje pesë")
        self.assertEqual(num2words(-1.5, lang="sq"), "minus një presje pesë")
        self.assertEqual(num2words(-10.5, lang="sq"), "minus dhjetë presje pesë")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sq", ordinal=True), "i pari")
        self.assertEqual(num2words(2, lang="sq", ordinal=True), "i dyti")
        self.assertEqual(num2words(3, lang="sq", ordinal=True), "i treti")
        self.assertEqual(num2words(4, lang="sq", ordinal=True), "i katërti")
        self.assertEqual(num2words(5, lang="sq", ordinal=True), "i pesti")
        self.assertEqual(num2words(6, lang="sq", ordinal=True), "i gjashti")
        self.assertEqual(num2words(7, lang="sq", ordinal=True), "i shtati")
        self.assertEqual(num2words(8, lang="sq", ordinal=True), "i teti")
        self.assertEqual(num2words(9, lang="sq", ordinal=True), "i nënti")
        self.assertEqual(num2words(10, lang="sq", ordinal=True), "i dhjeti")
        self.assertEqual(num2words(11, lang="sq", ordinal=True), "i njëmbëdhjetë")
        self.assertEqual(num2words(12, lang="sq", ordinal=True), "i dymbëdhjetë")
        self.assertEqual(num2words(13, lang="sq", ordinal=True), "i trembëdhjetë")
        self.assertEqual(num2words(14, lang="sq", ordinal=True), "i katërmbëdhjetë")
        self.assertEqual(num2words(15, lang="sq", ordinal=True), "i pesëmbëdhjetë")
        self.assertEqual(num2words(16, lang="sq", ordinal=True), "i gjashtëmbëdhjetë")
        self.assertEqual(num2words(17, lang="sq", ordinal=True), "i shtatëmbëdhjetë")
        self.assertEqual(num2words(18, lang="sq", ordinal=True), "i tetëmbëdhjetë")
        self.assertEqual(num2words(19, lang="sq", ordinal=True), "i nëntëmbëdhjetë")
        self.assertEqual(num2words(20, lang="sq", ordinal=True), "i njezeti")
        self.assertEqual(num2words(21, lang="sq", ordinal=True), "i njëzet e një")
        self.assertEqual(num2words(22, lang="sq", ordinal=True), "i njëzet e dy")
        self.assertEqual(num2words(25, lang="sq", ordinal=True), "i njëzet e pesë")
        self.assertEqual(num2words(30, lang="sq", ordinal=True), "i tridhjetë")
        self.assertEqual(num2words(40, lang="sq", ordinal=True), "i dyzet")
        self.assertEqual(num2words(50, lang="sq", ordinal=True), "i pesëdhjetë")
        self.assertEqual(num2words(60, lang="sq", ordinal=True), "i gjashtëdhjetë")
        self.assertEqual(num2words(70, lang="sq", ordinal=True), "i shtatëdhjetë")
        self.assertEqual(num2words(80, lang="sq", ordinal=True), "i tetëdhjetë")
        self.assertEqual(num2words(90, lang="sq", ordinal=True), "i nëntëdhjetë")
        self.assertEqual(num2words(100, lang="sq", ordinal=True), "i njëqindi")
        self.assertEqual(num2words(101, lang="sq", ordinal=True), "i njëqind e një")
        self.assertEqual(num2words(200, lang="sq", ordinal=True), "i dy qind")
        self.assertEqual(num2words(500, lang="sq", ordinal=True), "i pesë qind")
        self.assertEqual(num2words(1000, lang="sq", ordinal=True), "i një mijti")
        self.assertEqual(num2words(1001, lang="sq", ordinal=True), "i një mijë e një")
        self.assertEqual(num2words(10000, lang="sq", ordinal=True), "i dhjetë mijë")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="ALL"), "zero lekë")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="ALL"), "një qindarkë"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="ALL"),
            "pesëdhjetë qindarkë",
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="ALL"), "një lek"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="ALL"),
            "një lek, pesëdhjetë qindarkë",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="EUR"), "zero euro")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="EUR"), "një cent"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="EUR"), "pesëdhjetë centë"
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="EUR"), "një euro"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="EUR"),
            "një euro, pesëdhjetë centë",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="USD"), "zero dollarë")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="USD"), "një cent"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="USD"), "pesëdhjetë centë"
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="USD"), "një dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="USD"),
            "një dollar, pesëdhjetë centë",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="GBP"), "zero paund")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="GBP"), "një peni"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="GBP"), "pesëdhjetë pence"
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="GBP"), "një paund"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="GBP"),
            "një paund, pesëdhjetë pence",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="CHF"), "zero franka zvicer")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="CHF"), "një centim"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="CHF"),
            "pesëdhjetë centimë",
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="CHF"), "një frank zviceran"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="CHF"),
            "një frank zviceran, pesëdhjetë centimë",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="JPY"), "zero jenë")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="JPY"), "një sen"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="JPY"), "pesëdhjetë senë"
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="JPY"), "një jen"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="JPY"),
            "një jen, pesëdhjetë senë",
        )
        self.assertEqual(num2words(0, lang="sq", to="currency", currency="RUB"), "zero rubla")
        self.assertEqual(
            num2words(0.01, lang="sq", to="currency", currency="RUB"), "një kopek"
        )
        self.assertEqual(
            num2words(0.5, lang="sq", to="currency", currency="RUB"),
            "pesëdhjetë kopekë",
        )
        self.assertEqual(
            num2words(1, lang="sq", to="currency", currency="RUB"), "një rubël"
        )
        self.assertEqual(
            num2words(1.5, lang="sq", to="currency", currency="RUB"),
            "një rubël, pesëdhjetë kopekë",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sq", to="year"), "një mijë")
        self.assertEqual(
            num2words(1066, lang="sq", to="year"), "një mijë e gjashtëdhjetë e gjashtë"
        )
        self.assertEqual(
            num2words(1492, lang="sq", to="year"),
            "një mijë e katër qind e nëntëdhjetë e dy",
        )
        self.assertEqual(
            num2words(1776, lang="sq", to="year"),
            "një mijë e shtatë qind e shtatëdhjetë e gjashtë",
        )
        self.assertEqual(num2words(1800, lang="sq", to="year"), "një mijë e tetë qind")
        self.assertEqual(num2words(1900, lang="sq", to="year"), "një mijë e nëntë qind")
        self.assertEqual(
            num2words(1984, lang="sq", to="year"),
            "një mijë e nëntë qind e tetëdhjetë e katër",
        )
        self.assertEqual(
            num2words(1999, lang="sq", to="year"),
            "një mijë e nëntë qind e nëntëdhjetë e nëntë",
        )
        self.assertEqual(num2words(2000, lang="sq", to="year"), "dy mijë")
        self.assertEqual(num2words(2001, lang="sq", to="year"), "dy mijë e një")
        self.assertEqual(num2words(2010, lang="sq", to="year"), "dy mijë e dhjetë")
        self.assertEqual(num2words(2020, lang="sq", to="year"), "dy mijë e njëzet")
        self.assertEqual(
            num2words(2024, lang="sq", to="year"), "dy mijë e njëzet e katër"
        )
        self.assertEqual(num2words(2100, lang="sq", to="year"), "dy mijë e njëqind")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sq"), "zero")
        self.assertEqual(num2words("1", lang="sq"), "një")
        self.assertEqual(num2words("10", lang="sq"), "dhjetë")
        self.assertEqual(num2words("100", lang="sq"), "njëqind")
        self.assertEqual(num2words("1000", lang="sq"), "një mijë")
        self.assertEqual(num2words("10000", lang="sq"), "dhjetë mijë")
        self.assertEqual(num2words("100000", lang="sq"), "njëqind mijë")
        self.assertEqual(num2words("1000000", lang="sq"), "një milion")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sq"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sq"), num2words("100", lang="sq"))
        self.assertEqual(num2words(1000, lang="sq"), num2words("1000", lang="sq"))

