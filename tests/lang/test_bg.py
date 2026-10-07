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


class Num2WordsBGTest(TestCase):
    """Comprehensive test cases for Bulgarian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="bg"), "нула")
        self.assertEqual(num2words(1, lang="bg"), "един")
        self.assertEqual(num2words(2, lang="bg"), "два")
        self.assertEqual(num2words(3, lang="bg"), "три")
        self.assertEqual(num2words(4, lang="bg"), "четири")
        self.assertEqual(num2words(5, lang="bg"), "пет")
        self.assertEqual(num2words(6, lang="bg"), "шест")
        self.assertEqual(num2words(7, lang="bg"), "седем")
        self.assertEqual(num2words(8, lang="bg"), "осем")
        self.assertEqual(num2words(9, lang="bg"), "девет")
        self.assertEqual(num2words(10, lang="bg"), "десет")
        self.assertEqual(num2words(11, lang="bg"), "единадесет")
        self.assertEqual(num2words(12, lang="bg"), "дванадесет")
        self.assertEqual(num2words(13, lang="bg"), "тринадесет")
        self.assertEqual(num2words(14, lang="bg"), "четиринадесет")
        self.assertEqual(num2words(15, lang="bg"), "петнадесет")
        self.assertEqual(num2words(16, lang="bg"), "шестнадесет")
        self.assertEqual(num2words(17, lang="bg"), "седемнадесет")
        self.assertEqual(num2words(18, lang="bg"), "осемнадесет")
        self.assertEqual(num2words(19, lang="bg"), "деветнадесет")
        self.assertEqual(num2words(20, lang="bg"), "двадесет")
        self.assertEqual(num2words(21, lang="bg"), "двадесет и един")
        self.assertEqual(num2words(22, lang="bg"), "двадесет и два")
        self.assertEqual(num2words(23, lang="bg"), "двадесет и три")
        self.assertEqual(num2words(24, lang="bg"), "двадесет и четири")
        self.assertEqual(num2words(25, lang="bg"), "двадесет и пет")
        self.assertEqual(num2words(26, lang="bg"), "двадесет и шест")
        self.assertEqual(num2words(27, lang="bg"), "двадесет и седем")
        self.assertEqual(num2words(28, lang="bg"), "двадесет и осем")
        self.assertEqual(num2words(29, lang="bg"), "двадесет и девет")
        self.assertEqual(num2words(30, lang="bg"), "тридесет")
        self.assertEqual(num2words(31, lang="bg"), "тридесет и един")
        self.assertEqual(num2words(35, lang="bg"), "тридесет и пет")
        self.assertEqual(num2words(40, lang="bg"), "четиридесет")
        self.assertEqual(num2words(45, lang="bg"), "четиридесет и пет")
        self.assertEqual(num2words(50, lang="bg"), "петдесет")
        self.assertEqual(num2words(55, lang="bg"), "петдесет и пет")
        self.assertEqual(num2words(60, lang="bg"), "шестдесет")
        self.assertEqual(num2words(65, lang="bg"), "шестдесет и пет")
        self.assertEqual(num2words(70, lang="bg"), "седемдесет")
        self.assertEqual(num2words(75, lang="bg"), "седемдесет и пет")
        self.assertEqual(num2words(80, lang="bg"), "осемдесет")
        self.assertEqual(num2words(85, lang="bg"), "осемдесет и пет")
        self.assertEqual(num2words(90, lang="bg"), "деветдесет")
        self.assertEqual(num2words(95, lang="bg"), "деветдесет и пет")
        self.assertEqual(num2words(99, lang="bg"), "деветдесет и девет")
        self.assertEqual(num2words(100, lang="bg"), "сто")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="bg"), "сто един")
        self.assertEqual(num2words(110, lang="bg"), "сто десет")
        self.assertEqual(num2words(111, lang="bg"), "сто единадесет")
        self.assertEqual(num2words(120, lang="bg"), "сто двадесет")
        self.assertEqual(num2words(125, lang="bg"), "сто двадесет и пет")
        self.assertEqual(num2words(150, lang="bg"), "сто петдесет")
        self.assertEqual(num2words(175, lang="bg"), "сто седемдесет и пет")
        self.assertEqual(num2words(199, lang="bg"), "сто деветдесет и девет")
        self.assertEqual(num2words(200, lang="bg"), "двеста")
        self.assertEqual(num2words(201, lang="bg"), "двеста един")
        self.assertEqual(num2words(210, lang="bg"), "двеста десет")
        self.assertEqual(num2words(220, lang="bg"), "двеста двадесет")
        self.assertEqual(num2words(250, lang="bg"), "двеста петдесет")
        self.assertEqual(num2words(299, lang="bg"), "двеста деветдесет и девет")
        self.assertEqual(num2words(300, lang="bg"), "триста")
        self.assertEqual(num2words(333, lang="bg"), "триста тридесет и три")
        self.assertEqual(num2words(400, lang="bg"), "четиристотин")
        self.assertEqual(num2words(444, lang="bg"), "четиристотин четиридесет и четири")
        self.assertEqual(num2words(500, lang="bg"), "петстотин")
        self.assertEqual(num2words(555, lang="bg"), "петстотин петдесет и пет")
        self.assertEqual(num2words(600, lang="bg"), "шестстотин")
        self.assertEqual(num2words(666, lang="bg"), "шестстотин шестдесет и шест")
        self.assertEqual(num2words(700, lang="bg"), "седемстотин")
        self.assertEqual(num2words(777, lang="bg"), "седемстотин седемдесет и седем")
        self.assertEqual(num2words(800, lang="bg"), "осемстотин")
        self.assertEqual(num2words(888, lang="bg"), "осемстотин осемдесет и осем")
        self.assertEqual(num2words(900, lang="bg"), "деветстотин")
        self.assertEqual(num2words(999, lang="bg"), "деветстотин деветдесет и девет")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="bg"), "хиляда")
        self.assertEqual(num2words(1001, lang="bg"), "хиляда един")
        self.assertEqual(num2words(1010, lang="bg"), "хиляда десет")
        self.assertEqual(num2words(1100, lang="bg"), "хиляда сто")
        self.assertEqual(num2words(1111, lang="bg"), "хиляда сто единадесет")
        self.assertEqual(num2words(1234, lang="bg"), "хиляда двеста тридесет и четири")
        self.assertEqual(num2words(1500, lang="bg"), "хиляда петстотин")
        self.assertEqual(
            num2words(1999, lang="bg"), "хиляда деветстотин деветдесет и девет"
        )
        self.assertEqual(num2words(2000, lang="bg"), "две хиляди")
        self.assertEqual(num2words(2001, lang="bg"), "две хиляди един")
        self.assertEqual(num2words(2020, lang="bg"), "две хиляди двадесет")
        self.assertEqual(num2words(2222, lang="bg"), "две хиляди двеста двадесет и два")
        self.assertEqual(num2words(3000, lang="bg"), "три хиляди")
        self.assertEqual(num2words(3333, lang="bg"), "три хиляди триста тридесет и три")
        self.assertEqual(num2words(4000, lang="bg"), "четири хиляди")
        self.assertEqual(
            num2words(4444, lang="bg"),
            "четири хиляди четиристотин четиридесет и четири",
        )
        self.assertEqual(num2words(5000, lang="bg"), "пет хиляди")
        self.assertEqual(
            num2words(5555, lang="bg"), "пет хиляди петстотин петдесет и пет"
        )
        self.assertEqual(num2words(6000, lang="bg"), "шест хиляди")
        self.assertEqual(
            num2words(6666, lang="bg"), "шест хиляди шестстотин шестдесет и шест"
        )
        self.assertEqual(num2words(7000, lang="bg"), "седем хиляди")
        self.assertEqual(
            num2words(7777, lang="bg"), "седем хиляди седемстотин седемдесет и седем"
        )
        self.assertEqual(num2words(8000, lang="bg"), "осем хиляди")
        self.assertEqual(
            num2words(8888, lang="bg"), "осем хиляди осемстотин осемдесет и осем"
        )
        self.assertEqual(num2words(9000, lang="bg"), "девет хиляди")
        self.assertEqual(
            num2words(9999, lang="bg"), "девет хиляди деветстотин деветдесет и девет"
        )
        self.assertEqual(num2words(10000, lang="bg"), "десет хиляди")
        self.assertEqual(num2words(10001, lang="bg"), "десет хиляди един")
        self.assertEqual(
            num2words(11111, lang="bg"), "единадесет хиляди сто единадесет"
        )
        self.assertEqual(
            num2words(12345, lang="bg"), "дванадесет хиляди триста четиридесет и пет"
        )
        self.assertEqual(num2words(20000, lang="bg"), "двадесет хиляди")
        self.assertEqual(num2words(50000, lang="bg"), "петдесет хиляди")
        self.assertEqual(
            num2words(99999, lang="bg"),
            "деветдесет и девет хиляди деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(100000, lang="bg"), "сто хиляди")
        self.assertEqual(
            num2words(123456, lang="bg"),
            "сто двадесет и три хиляди четиристотин петдесет и шест",
        )
        self.assertEqual(num2words(200000, lang="bg"), "двеста хиляди")
        self.assertEqual(num2words(500000, lang="bg"), "петстотин хиляди")
        self.assertEqual(
            num2words(654321, lang="bg"),
            "шестстотин петдесет и четири хиляди триста двадесет и един",
        )
        self.assertEqual(
            num2words(999999, lang="bg"),
            "деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="bg"), "един милион")
        self.assertEqual(num2words(1000001, lang="bg"), "един милион един")
        self.assertEqual(
            num2words(1111111, lang="bg"),
            "един милион сто единадесет хиляди сто единадесет",
        )
        self.assertEqual(
            num2words(1234567, lang="bg"),
            "един милион двеста тридесет и четири хиляди петстотин шестдесет и седем",
        )
        self.assertEqual(num2words(2000000, lang="bg"), "два милиона")
        self.assertEqual(num2words(5000000, lang="bg"), "пет милиона")
        self.assertEqual(
            num2words(9999999, lang="bg"),
            "девет милиона деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(10000000, lang="bg"), "десет милиона")
        self.assertEqual(
            num2words(12345678, lang="bg"),
            "дванадесет милиона триста четиридесет и пет хиляди шестстотин седемдесет и осем",
        )
        self.assertEqual(
            num2words(99999999, lang="bg"),
            "деветдесет и девет милиона деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(100000000, lang="bg"), "сто милиона")
        self.assertEqual(
            num2words(123456789, lang="bg"),
            "сто двадесет и три милиона четиристотин петдесет и шест хиляди седемстотин осемдесет и девет",
        )
        self.assertEqual(
            num2words(999999999, lang="bg"),
            "деветстотин деветдесет и девет милиона деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(1000000000, lang="bg"), "един милиард")
        self.assertEqual(
            num2words(1234567890, lang="bg"),
            "един милиард двеста тридесет и четири милиона петстотин шестдесет и седем хиляди осемстотин деветдесет",
        )
        self.assertEqual(
            num2words(9999999999, lang="bg"),
            "девет милиарда деветстотин деветдесет и девет милиона деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(10000000000, lang="bg"), "десет милиарда")
        self.assertEqual(
            num2words(99999999999, lang="bg"),
            "деветдесет и девет милиарда деветстотин деветдесет и девет милиона деветстотин деветдесет и девет хиляди деветстотин деветдесет и девет",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="bg"), "минус един")
        self.assertEqual(num2words(-2, lang="bg"), "минус два")
        self.assertEqual(num2words(-5, lang="bg"), "минус пет")
        self.assertEqual(num2words(-10, lang="bg"), "минус десет")
        self.assertEqual(num2words(-11, lang="bg"), "минус единадесет")
        self.assertEqual(num2words(-20, lang="bg"), "минус двадесет")
        self.assertEqual(num2words(-50, lang="bg"), "минус петдесет")
        self.assertEqual(num2words(-99, lang="bg"), "минус деветдесет и девет")
        self.assertEqual(num2words(-100, lang="bg"), "минус сто")
        self.assertEqual(num2words(-101, lang="bg"), "минус сто един")
        self.assertEqual(num2words(-200, lang="bg"), "минус двеста")
        self.assertEqual(
            num2words(-999, lang="bg"), "минус деветстотин деветдесет и девет"
        )
        self.assertEqual(num2words(-1000, lang="bg"), "минус хиляда")
        self.assertEqual(num2words(-1001, lang="bg"), "минус хиляда един")
        self.assertEqual(num2words(-10000, lang="bg"), "минус десет хиляди")
        self.assertEqual(num2words(-100000, lang="bg"), "минус сто хиляди")
        self.assertEqual(num2words(-1000000, lang="bg"), "минус един милион")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="bg"), "нула точка едно")
        self.assertEqual(num2words(0.5, lang="bg"), "нула точка пет")
        self.assertEqual(num2words(0.9, lang="bg"), "нула точка девет")
        self.assertEqual(num2words(1.1, lang="bg"), "един точка едно")
        self.assertEqual(num2words(1.5, lang="bg"), "един точка пет")
        self.assertEqual(num2words(2.5, lang="bg"), "два точка пет")
        self.assertEqual(num2words(3.14, lang="bg"), "три точка едно четири")
        self.assertEqual(num2words(10.5, lang="bg"), "десет точка пет")
        self.assertEqual(num2words(11.11, lang="bg"), "единадесет точка едно едно")
        self.assertEqual(num2words(20.2, lang="bg"), "двадесет точка две")
        self.assertEqual(
            num2words(99.99, lang="bg"), "деветдесет и девет точка девет девет"
        )
        self.assertEqual(num2words(100.01, lang="bg"), "сто точка нула едно")
        self.assertEqual(num2words(100.5, lang="bg"), "сто точка пет")
        self.assertEqual(
            num2words(123.45, lang="bg"), "сто двадесет и три точка четири пет"
        )
        self.assertEqual(num2words(1000.5, lang="bg"), "хиляда точка пет")
        self.assertEqual(
            num2words(1234.56, lang="bg"),
            "хиляда двеста тридесет и четири точка пет шест",
        )
        self.assertEqual(num2words(10000.01, lang="bg"), "десет хиляди точка нула едно")
        self.assertEqual(num2words(-0.5, lang="bg"), "минус нула точка пет")
        self.assertEqual(num2words(-1.5, lang="bg"), "минус един точка пет")
        self.assertEqual(num2words(-10.5, lang="bg"), "минус десет точка пет")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="bg", ordinal=True), "първи")
        self.assertEqual(num2words(2, lang="bg", ordinal=True), "втори")
        self.assertEqual(num2words(3, lang="bg", ordinal=True), "трети")
        self.assertEqual(num2words(4, lang="bg", ordinal=True), "четвърти")
        self.assertEqual(num2words(5, lang="bg", ordinal=True), "пети")
        self.assertEqual(num2words(6, lang="bg", ordinal=True), "шести")
        self.assertEqual(num2words(7, lang="bg", ordinal=True), "седми")
        self.assertEqual(num2words(8, lang="bg", ordinal=True), "осми")
        self.assertEqual(num2words(9, lang="bg", ordinal=True), "девети")
        self.assertEqual(num2words(10, lang="bg", ordinal=True), "десети")
        self.assertEqual(num2words(11, lang="bg", ordinal=True), "единадесети")
        self.assertEqual(num2words(12, lang="bg", ordinal=True), "дванадесети")
        self.assertEqual(num2words(13, lang="bg", ordinal=True), "тринадесети")
        self.assertEqual(num2words(14, lang="bg", ordinal=True), "четиринадесети")
        self.assertEqual(num2words(15, lang="bg", ordinal=True), "петнадесети")
        self.assertEqual(num2words(16, lang="bg", ordinal=True), "шестнадесети")
        self.assertEqual(num2words(17, lang="bg", ordinal=True), "седемнадесети")
        self.assertEqual(num2words(18, lang="bg", ordinal=True), "осемнадесети")
        self.assertEqual(num2words(19, lang="bg", ordinal=True), "деветнадесети")
        self.assertEqual(num2words(20, lang="bg", ordinal=True), "двадесети")
        self.assertEqual(num2words(21, lang="bg", ordinal=True), "двадесет и първи")
        self.assertEqual(num2words(22, lang="bg", ordinal=True), "двадесет и втори")
        self.assertEqual(num2words(25, lang="bg", ordinal=True), "двадесет и пети")
        self.assertEqual(num2words(30, lang="bg", ordinal=True), "тридесети")
        self.assertEqual(num2words(40, lang="bg", ordinal=True), "четиридесети")
        self.assertEqual(num2words(50, lang="bg", ordinal=True), "петдесети")
        self.assertEqual(num2words(60, lang="bg", ordinal=True), "шестдесети")
        self.assertEqual(num2words(70, lang="bg", ordinal=True), "седемдесети")
        self.assertEqual(num2words(80, lang="bg", ordinal=True), "осемдесети")
        self.assertEqual(num2words(90, lang="bg", ordinal=True), "деветдесети")
        self.assertEqual(num2words(100, lang="bg", ordinal=True), "стотен")
        self.assertEqual(num2words(101, lang="bg", ordinal=True), "сто първи")
        self.assertEqual(num2words(200, lang="bg", ordinal=True), "двестотен")
        self.assertEqual(num2words(500, lang="bg", ordinal=True), "петстотен")
        self.assertEqual(num2words(1000, lang="bg", ordinal=True), "хиляден")
        self.assertEqual(num2words(1001, lang="bg", ordinal=True), "хиляда първи")
        self.assertEqual(num2words(10000, lang="bg", ordinal=True), "десетхиляден")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="bg", to="currency", currency="BGN"), "нула лева"
        )
        self.assertEqual(
            num2words(0.01, lang="bg", to="currency", currency="BGN"),
            "нула лева и един стотинка",
        )
        self.assertEqual(
            num2words(0.5, lang="bg", to="currency", currency="BGN"),
            "нула лева и петдесет стотинки",
        )
        self.assertEqual(
            num2words(1, lang="bg", to="currency", currency="BGN"), "един лев"
        )
        self.assertEqual(
            num2words(1.5, lang="bg", to="currency", currency="BGN"),
            "един лев и петдесет стотинки",
        )
        self.assertEqual(
            num2words(0, lang="bg", to="currency", currency="EUR"), "нула евро"
        )
        self.assertEqual(
            num2words(0.01, lang="bg", to="currency", currency="EUR"),
            "нула евро и един цент",
        )
        self.assertEqual(
            num2words(0.5, lang="bg", to="currency", currency="EUR"),
            "нула евро и петдесет цента",
        )
        self.assertEqual(
            num2words(1, lang="bg", to="currency", currency="EUR"), "един евро"
        )
        self.assertEqual(
            num2words(1.5, lang="bg", to="currency", currency="EUR"),
            "един евро и петдесет цента",
        )
        self.assertEqual(
            num2words(0, lang="bg", to="currency", currency="USD"), "нула долара"
        )
        self.assertEqual(
            num2words(0.01, lang="bg", to="currency", currency="USD"),
            "нула долара и един цент",
        )
        self.assertEqual(
            num2words(0.5, lang="bg", to="currency", currency="USD"),
            "нула долара и петдесет цента",
        )
        self.assertEqual(
            num2words(1, lang="bg", to="currency", currency="USD"), "един долар"
        )
        self.assertEqual(
            num2words(1.5, lang="bg", to="currency", currency="USD"),
            "един долар и петдесет цента",
        )
        self.assertEqual(
            num2words(0, lang="bg", to="currency", currency="GBP"), "нула паунда"
        )
        self.assertEqual(
            num2words(0.01, lang="bg", to="currency", currency="GBP"),
            "нула паунда и един пени",
        )
        self.assertEqual(
            num2words(0.5, lang="bg", to="currency", currency="GBP"),
            "нула паунда и петдесет пенса",
        )
        self.assertEqual(
            num2words(1, lang="bg", to="currency", currency="GBP"), "един паунд"
        )
        self.assertEqual(
            num2words(1.5, lang="bg", to="currency", currency="GBP"),
            "един паунд и петдесет пенса",
        )
        self.assertEqual(
            num2words(0, lang="bg", to="currency", currency="JPY"), "нула йени"
        )
        self.assertEqual(
            num2words(0.01, lang="bg", to="currency", currency="JPY"),
            "нула йени и един сен",
        )
        self.assertEqual(
            num2words(0.5, lang="bg", to="currency", currency="JPY"),
            "нула йени и петдесет сена",
        )
        self.assertEqual(
            num2words(1, lang="bg", to="currency", currency="JPY"), "един йена"
        )
        self.assertEqual(
            num2words(1.5, lang="bg", to="currency", currency="JPY"),
            "един йена и петдесет сена",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="bg", to="year"), "хиляда")
        self.assertEqual(
            num2words(1066, lang="bg", to="year"), "хиляда шестдесет и шест"
        )
        self.assertEqual(
            num2words(1492, lang="bg", to="year"),
            "хиляда четиристотин деветдесет и два",
        )
        self.assertEqual(
            num2words(1776, lang="bg", to="year"),
            "хиляда седемстотин седемдесет и шест",
        )
        self.assertEqual(num2words(1800, lang="bg", to="year"), "хиляда осемстотин")
        self.assertEqual(num2words(1900, lang="bg", to="year"), "хиляда деветстотин")
        self.assertEqual(
            num2words(1984, lang="bg", to="year"),
            "хиляда деветстотин осемдесет и четири",
        )
        self.assertEqual(
            num2words(1999, lang="bg", to="year"),
            "хиляда деветстотин деветдесет и девет",
        )
        self.assertEqual(num2words(2000, lang="bg", to="year"), "две хиляди")
        self.assertEqual(num2words(2001, lang="bg", to="year"), "две хиляди един")
        self.assertEqual(num2words(2010, lang="bg", to="year"), "две хиляди десет")
        self.assertEqual(num2words(2020, lang="bg", to="year"), "две хиляди двадесет")
        self.assertEqual(
            num2words(2024, lang="bg", to="year"), "две хиляди двадесет и четири"
        )
        self.assertEqual(num2words(2100, lang="bg", to="year"), "две хиляди сто")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="bg"), "нула")
        self.assertEqual(num2words("1", lang="bg"), "един")
        self.assertEqual(num2words("10", lang="bg"), "десет")
        self.assertEqual(num2words("100", lang="bg"), "сто")
        self.assertEqual(num2words("1000", lang="bg"), "хиляда")
        self.assertEqual(num2words("10000", lang="bg"), "десет хиляди")
        self.assertEqual(num2words("100000", lang="bg"), "сто хиляди")
        self.assertEqual(num2words("1000000", lang="bg"), "един милион")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="bg"), "нула")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="bg"), num2words("100", lang="bg"))
        self.assertEqual(num2words(1000, lang="bg"), num2words("1000", lang="bg"))
