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


class Num2WordsSNTest(TestCase):
    """Comprehensive test cases for Shona language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sn"), "zero")
        self.assertEqual(num2words(1, lang="sn"), "motsi")
        self.assertEqual(num2words(2, lang="sn"), "piri")
        self.assertEqual(num2words(3, lang="sn"), "tatu")
        self.assertEqual(num2words(4, lang="sn"), "china")
        self.assertEqual(num2words(5, lang="sn"), "shanu")
        self.assertEqual(num2words(6, lang="sn"), "tanhatu")
        self.assertEqual(num2words(7, lang="sn"), "nomwe")
        self.assertEqual(num2words(8, lang="sn"), "sere")
        self.assertEqual(num2words(9, lang="sn"), "pfumbamwe")
        self.assertEqual(num2words(10, lang="sn"), "gumi")
        self.assertEqual(num2words(11, lang="sn"), "gumi neimwe")
        self.assertEqual(num2words(12, lang="sn"), "gumi nepiri")
        self.assertEqual(num2words(13, lang="sn"), "gumi netatu")
        self.assertEqual(num2words(14, lang="sn"), "gumi nechina")
        self.assertEqual(num2words(15, lang="sn"), "gumi neshanu")
        self.assertEqual(num2words(16, lang="sn"), "gumi nenhatu")
        self.assertEqual(num2words(17, lang="sn"), "gumi nenomwe")
        self.assertEqual(num2words(18, lang="sn"), "gumi nesere")
        self.assertEqual(num2words(19, lang="sn"), "gumi nepfumbamwe")
        self.assertEqual(num2words(20, lang="sn"), "makumi maviri")
        self.assertEqual(num2words(21, lang="sn"), "makumi maviri neimwe")
        self.assertEqual(num2words(22, lang="sn"), "makumi maviri nepiri")
        self.assertEqual(num2words(23, lang="sn"), "makumi maviri netatu")
        self.assertEqual(num2words(24, lang="sn"), "makumi maviri nechina")
        self.assertEqual(num2words(25, lang="sn"), "makumi maviri neshanu")
        self.assertEqual(num2words(26, lang="sn"), "makumi maviri nenhatu")
        self.assertEqual(num2words(27, lang="sn"), "makumi maviri nenomwe")
        self.assertEqual(num2words(28, lang="sn"), "makumi maviri nesere")
        self.assertEqual(num2words(29, lang="sn"), "makumi maviri nepfumbamwe")
        self.assertEqual(num2words(30, lang="sn"), "makumi matatu")
        self.assertEqual(num2words(31, lang="sn"), "makumi matatu neimwe")
        self.assertEqual(num2words(35, lang="sn"), "makumi matatu neshanu")
        self.assertEqual(num2words(40, lang="sn"), "makumi mana")
        self.assertEqual(num2words(45, lang="sn"), "makumi mana neshanu")
        self.assertEqual(num2words(50, lang="sn"), "makumi mashanu")
        self.assertEqual(num2words(55, lang="sn"), "makumi mashanu neshanu")
        self.assertEqual(num2words(60, lang="sn"), "makumi matanhatu")
        self.assertEqual(num2words(65, lang="sn"), "makumi matanhatu neshanu")
        self.assertEqual(num2words(70, lang="sn"), "makumi manomwe")
        self.assertEqual(num2words(75, lang="sn"), "makumi manomwe neshanu")
        self.assertEqual(num2words(80, lang="sn"), "makumi masere")
        self.assertEqual(num2words(85, lang="sn"), "makumi masere neshanu")
        self.assertEqual(num2words(90, lang="sn"), "makumi mapfumbamwe")
        self.assertEqual(num2words(95, lang="sn"), "makumi mapfumbamwe neshanu")
        self.assertEqual(num2words(99, lang="sn"), "makumi mapfumbamwe nepfumbamwe")
        self.assertEqual(num2words(100, lang="sn"), "zana")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sn"), "zana neimwe")
        self.assertEqual(num2words(110, lang="sn"), "zana negumi")
        self.assertEqual(num2words(111, lang="sn"), "zana negumi neimwe")
        self.assertEqual(num2words(120, lang="sn"), "zana nemakumi maviri")
        self.assertEqual(num2words(125, lang="sn"), "zana nemakumi maviri neshanu")
        self.assertEqual(num2words(150, lang="sn"), "zana nemakumi mashanu")
        self.assertEqual(num2words(175, lang="sn"), "zana nemakumi manomwe neshanu")
        self.assertEqual(
            num2words(199, lang="sn"), "zana nemakumi mapfumbamwe nepfumbamwe"
        )
        self.assertEqual(num2words(200, lang="sn"), "mazana maviri")
        self.assertEqual(num2words(201, lang="sn"), "mazana maviri nemotsi")
        self.assertEqual(num2words(210, lang="sn"), "mazana maviri negumi")
        self.assertEqual(num2words(220, lang="sn"), "mazana maviri nemakumi maviri")
        self.assertEqual(num2words(250, lang="sn"), "mazana maviri nemakumi mashanu")
        self.assertEqual(
            num2words(299, lang="sn"), "mazana maviri nemakumi mapfumbamwe nepfumbamwe"
        )
        self.assertEqual(num2words(300, lang="sn"), "mazana matatu")
        self.assertEqual(
            num2words(333, lang="sn"), "mazana matatu nemakumi matatu netatu"
        )
        self.assertEqual(num2words(400, lang="sn"), "mazana mana")
        self.assertEqual(num2words(444, lang="sn"), "mazana mana nemakumi mana nechina")
        self.assertEqual(num2words(500, lang="sn"), "mazana mashanu")
        self.assertEqual(
            num2words(555, lang="sn"), "mazana mashanu nemakumi mashanu neshanu"
        )
        self.assertEqual(num2words(600, lang="sn"), "mazana matanhatu")
        self.assertEqual(
            num2words(666, lang="sn"), "mazana matanhatu nemakumi matanhatu nenhatu"
        )
        self.assertEqual(num2words(700, lang="sn"), "mazana manomwe")
        self.assertEqual(
            num2words(777, lang="sn"), "mazana manomwe nemakumi manomwe nenomwe"
        )
        self.assertEqual(num2words(800, lang="sn"), "mazana masere")
        self.assertEqual(
            num2words(888, lang="sn"), "mazana masere nemakumi masere nesere"
        )
        self.assertEqual(num2words(900, lang="sn"), "mazana mapfumbamwe")
        self.assertEqual(
            num2words(999, lang="sn"),
            "mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sn"), "churu")
        self.assertEqual(num2words(1001, lang="sn"), "churu neimwe")
        self.assertEqual(num2words(1010, lang="sn"), "churu negumi")
        self.assertEqual(num2words(1100, lang="sn"), "churu nezana")
        self.assertEqual(num2words(1111, lang="sn"), "churu nezana negumi neimwe")
        self.assertEqual(
            num2words(1234, lang="sn"), "churu nemazana maviri nemakumi matatu nechina"
        )
        self.assertEqual(num2words(1500, lang="sn"), "churu nemazana mashanu")
        self.assertEqual(
            num2words(1999, lang="sn"),
            "churu nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(2000, lang="sn"), "zvuru zviviri")
        self.assertEqual(num2words(2001, lang="sn"), "zvuru zviviri nemotsi")
        self.assertEqual(num2words(2020, lang="sn"), "zvuru zviviri nemakumi maviri")
        self.assertEqual(
            num2words(2222, lang="sn"),
            "zvuru zviviri nemazana maviri nemakumi maviri nepiri",
        )
        self.assertEqual(num2words(3000, lang="sn"), "zvuru zvitatu")
        self.assertEqual(
            num2words(3333, lang="sn"),
            "zvuru zvitatu nemazana matatu nemakumi matatu netatu",
        )
        self.assertEqual(num2words(4000, lang="sn"), "zvuru zvina")
        self.assertEqual(
            num2words(4444, lang="sn"),
            "zvuru zvina nemazana mana nemakumi mana nechina",
        )
        self.assertEqual(num2words(5000, lang="sn"), "zvuru zvishanu")
        self.assertEqual(
            num2words(5555, lang="sn"),
            "zvuru zvishanu nemazana mashanu nemakumi mashanu neshanu",
        )
        self.assertEqual(num2words(6000, lang="sn"), "zvuru zvitanhatu")
        self.assertEqual(
            num2words(6666, lang="sn"),
            "zvuru zvitanhatu nemazana matanhatu nemakumi matanhatu nenhatu",
        )
        self.assertEqual(num2words(7000, lang="sn"), "zvuru zvinomwe")
        self.assertEqual(
            num2words(7777, lang="sn"),
            "zvuru zvinomwe nemazana manomwe nemakumi manomwe nenomwe",
        )
        self.assertEqual(num2words(8000, lang="sn"), "zvuru zvisere")
        self.assertEqual(
            num2words(8888, lang="sn"),
            "zvuru zvisere nemazana masere nemakumi masere nesere",
        )
        self.assertEqual(num2words(9000, lang="sn"), "zvuru zvipfumbamwe")
        self.assertEqual(
            num2words(9999, lang="sn"),
            "zvuru zvipfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(10000, lang="sn"), "zvuru gumi")
        self.assertEqual(num2words(10001, lang="sn"), "zvuru gumi neimwe")
        self.assertEqual(
            num2words(11111, lang="sn"), "zvuru gumi neimwe nezana negumi neimwe"
        )
        self.assertEqual(
            num2words(12345, lang="sn"),
            "zvuru gumi nepiri nemazana matatu nemakumi mana neshanu",
        )
        self.assertEqual(num2words(20000, lang="sn"), "zvuru makumi maviri")
        self.assertEqual(num2words(50000, lang="sn"), "zvuru makumi mashanu")
        self.assertEqual(
            num2words(99999, lang="sn"),
            "zvuru makumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(100000, lang="sn"), "zvuru zana")
        self.assertEqual(
            num2words(123456, lang="sn"),
            "zvuru zana nemakumi maviri netatu nemazana mana nemakumi mashanu nenhatu",
        )
        self.assertEqual(num2words(200000, lang="sn"), "zvuru mazana maviri")
        self.assertEqual(num2words(500000, lang="sn"), "zvuru mazana mashanu")
        self.assertEqual(
            num2words(654321, lang="sn"),
            "zvuru mazana matanhatu nemakumi mashanu nechina nemazana matatu nemakumi maviri neimwe",
        )
        self.assertEqual(
            num2words(999999, lang="sn"),
            "zvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sn"), "miriyoni")
        self.assertEqual(num2words(1000001, lang="sn"), "miriyoni neimwe")
        self.assertEqual(
            num2words(1111111, lang="sn"),
            "miriyoni nezvuru zana negumi neimwe nezana negumi neimwe",
        )
        self.assertEqual(
            num2words(1234567, lang="sn"),
            "miriyoni nezvuru mazana maviri nemakumi matatu nechina nemazana mashanu nemakumi matanhatu nenomwe",
        )
        self.assertEqual(num2words(2000000, lang="sn"), "miriyoni mbiri")
        self.assertEqual(num2words(5000000, lang="sn"), "miriyoni shanu")
        self.assertEqual(
            num2words(9999999, lang="sn"),
            "miriyoni pfumbamwe nezvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(10000000, lang="sn"), "miriyoni gumi")
        self.assertEqual(
            num2words(12345678, lang="sn"),
            "miriyoni gumi nepiri nezvuru mazana matatu nemakumi mana neshanu nemazana matanhatu nemakumi manomwe nesere",
        )
        self.assertEqual(
            num2words(99999999, lang="sn"),
            "miriyoni makumi mapfumbamwe nepfumbamwe nezvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(100000000, lang="sn"), "miriyoni zana")
        self.assertEqual(
            num2words(123456789, lang="sn"),
            "miriyoni zana nemakumi maviri netatu nezvuru mazana mana nemakumi mashanu nenhatu nemazana manomwe nemakumi masere nepfumbamwe",
        )
        self.assertEqual(
            num2words(999999999, lang="sn"),
            "miriyoni mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nezvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(1000000000, lang="sn"), "bhiriyoni")
        self.assertEqual(
            num2words(1234567890, lang="sn"),
            "bhiriyoni nemiriyoni mazana maviri nemakumi matatu nechina nezvuru mazana mashanu nemakumi matanhatu nenomwe nemazana masere nemakumi mapfumbamwe",
        )
        self.assertEqual(
            num2words(9999999999, lang="sn"),
            "bhiriyoni pfumbamwe nemiriyoni mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nezvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(10000000000, lang="sn"), "bhiriyoni gumi")
        self.assertEqual(
            num2words(99999999999, lang="sn"),
            "bhiriyoni makumi mapfumbamwe nepfumbamwe nemiriyoni mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nezvuru mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sn"), "minus motsi")
        self.assertEqual(num2words(-2, lang="sn"), "minus piri")
        self.assertEqual(num2words(-5, lang="sn"), "minus shanu")
        self.assertEqual(num2words(-10, lang="sn"), "minus gumi")
        self.assertEqual(num2words(-11, lang="sn"), "minus gumi neimwe")
        self.assertEqual(num2words(-20, lang="sn"), "minus makumi maviri")
        self.assertEqual(num2words(-50, lang="sn"), "minus makumi mashanu")
        self.assertEqual(
            num2words(-99, lang="sn"), "minus makumi mapfumbamwe nepfumbamwe"
        )
        self.assertEqual(num2words(-100, lang="sn"), "minus zana")
        self.assertEqual(num2words(-101, lang="sn"), "minus zana neimwe")
        self.assertEqual(num2words(-200, lang="sn"), "minus mazana maviri")
        self.assertEqual(
            num2words(-999, lang="sn"),
            "minus mazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(-1000, lang="sn"), "minus churu")
        self.assertEqual(num2words(-1001, lang="sn"), "minus churu neimwe")
        self.assertEqual(num2words(-10000, lang="sn"), "minus zvuru gumi")
        self.assertEqual(num2words(-100000, lang="sn"), "minus zvuru zana")
        self.assertEqual(num2words(-1000000, lang="sn"), "minus miriyoni")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sn"), "zero poindi motsi")
        self.assertEqual(num2words(0.5, lang="sn"), "zero poindi shanu")
        self.assertEqual(num2words(0.9, lang="sn"), "zero poindi pfumbamwe")
        self.assertEqual(num2words(1.1, lang="sn"), "motsi poindi motsi")
        self.assertEqual(num2words(1.5, lang="sn"), "motsi poindi shanu")
        self.assertEqual(num2words(2.5, lang="sn"), "piri poindi shanu")
        self.assertEqual(num2words(3.14, lang="sn"), "tatu poindi motsi china")
        self.assertEqual(num2words(10.5, lang="sn"), "gumi poindi shanu")
        self.assertEqual(num2words(11.11, lang="sn"), "gumi neimwe poindi motsi motsi")
        self.assertEqual(num2words(20.2, lang="sn"), "makumi maviri poindi piri")
        self.assertEqual(
            num2words(99.99, lang="sn"),
            "makumi mapfumbamwe nepfumbamwe poindi pfumbamwe pfumbamwe",
        )
        self.assertEqual(num2words(100.01, lang="sn"), "zana poindi zero motsi")
        self.assertEqual(num2words(100.5, lang="sn"), "zana poindi shanu")
        self.assertEqual(
            num2words(123.45, lang="sn"),
            "zana nemakumi maviri netatu poindi china shanu",
        )
        self.assertEqual(num2words(1000.5, lang="sn"), "churu poindi shanu")
        self.assertEqual(
            num2words(1234.56, lang="sn"),
            "churu nemazana maviri nemakumi matatu nechina poindi shanu tanhatu",
        )
        self.assertEqual(num2words(10000.01, lang="sn"), "zvuru gumi poindi zero motsi")
        self.assertEqual(num2words(-0.5, lang="sn"), "minus zero poindi shanu")
        self.assertEqual(num2words(-1.5, lang="sn"), "minus motsi poindi shanu")
        self.assertEqual(num2words(-10.5, lang="sn"), "minus gumi poindi shanu")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sn", ordinal=True), "wekutanga")
        self.assertEqual(num2words(2, lang="sn", ordinal=True), "wechipiri")
        self.assertEqual(num2words(3, lang="sn", ordinal=True), "wechitatu")
        self.assertEqual(num2words(4, lang="sn", ordinal=True), "wechina")
        self.assertEqual(num2words(5, lang="sn", ordinal=True), "wechishanu")
        self.assertEqual(num2words(6, lang="sn", ordinal=True), "wechitanhatu")
        self.assertEqual(num2words(7, lang="sn", ordinal=True), "wechinomwe")
        self.assertEqual(num2words(8, lang="sn", ordinal=True), "wechisere")
        self.assertEqual(num2words(9, lang="sn", ordinal=True), "wechipfumbamwe")
        self.assertEqual(num2words(10, lang="sn", ordinal=True), "wegumi")
        self.assertEqual(num2words(11, lang="sn", ordinal=True), "wegumi neimwe")
        self.assertEqual(num2words(12, lang="sn", ordinal=True), "wegumi nepiri")
        self.assertEqual(num2words(13, lang="sn", ordinal=True), "wegumi netatu")
        self.assertEqual(num2words(14, lang="sn", ordinal=True), "wegumi nechina")
        self.assertEqual(num2words(15, lang="sn", ordinal=True), "wegumi neshanu")
        self.assertEqual(num2words(16, lang="sn", ordinal=True), "wegumi nenhatu")
        self.assertEqual(num2words(17, lang="sn", ordinal=True), "wegumi nenomwe")
        self.assertEqual(num2words(18, lang="sn", ordinal=True), "wegumi nesere")
        self.assertEqual(num2words(19, lang="sn", ordinal=True), "wegumi nepfumbamwe")
        self.assertEqual(num2words(20, lang="sn", ordinal=True), "wemakumi maviri")
        self.assertEqual(
            num2words(21, lang="sn", ordinal=True), "wemakumi maviri neimwe"
        )
        self.assertEqual(
            num2words(22, lang="sn", ordinal=True), "wemakumi maviri nepiri"
        )
        self.assertEqual(
            num2words(25, lang="sn", ordinal=True), "wemakumi maviri neshanu"
        )
        self.assertEqual(num2words(30, lang="sn", ordinal=True), "wemakumi matatu")
        self.assertEqual(num2words(40, lang="sn", ordinal=True), "wemakumi mana")
        self.assertEqual(num2words(50, lang="sn", ordinal=True), "wemakumi mashanu")
        self.assertEqual(num2words(60, lang="sn", ordinal=True), "wemakumi matanhatu")
        self.assertEqual(num2words(70, lang="sn", ordinal=True), "wemakumi manomwe")
        self.assertEqual(num2words(80, lang="sn", ordinal=True), "wemakumi masere")
        self.assertEqual(num2words(90, lang="sn", ordinal=True), "wemakumi mapfumbamwe")
        self.assertEqual(num2words(100, lang="sn", ordinal=True), "wezana")
        self.assertEqual(num2words(101, lang="sn", ordinal=True), "wezana neimwe")
        self.assertEqual(num2words(200, lang="sn", ordinal=True), "wezana maviri")
        self.assertEqual(num2words(500, lang="sn", ordinal=True), "wezana mashanu")
        self.assertEqual(num2words(1000, lang="sn", ordinal=True), "wechuru")
        self.assertEqual(num2words(1001, lang="sn", ordinal=True), "wechuru neimwe")
        self.assertEqual(num2words(10000, lang="sn", ordinal=True), "weuru gumi")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sn", to="currency", currency="USD"), "madhora zero"
        )
        self.assertEqual(
            num2words(0.01, lang="sn", to="currency", currency="USD"),
            "madhora zero nesendi rimwe",
        )
        self.assertEqual(
            num2words(0.5, lang="sn", to="currency", currency="USD"),
            "madhora zero nesendi makumi mashanu",
        )
        self.assertEqual(
            num2words(1, lang="sn", to="currency", currency="USD"), "dhora rimwe"
        )
        self.assertEqual(
            num2words(1.5, lang="sn", to="currency", currency="USD"),
            "dhora rimwe nesendi makumi mashanu",
        )
        self.assertEqual(
            num2words(0, lang="sn", to="currency", currency="ZWL"), "madhora zero"
        )
        self.assertEqual(
            num2words(0.01, lang="sn", to="currency", currency="ZWL"),
            "madhora zero nesendi rimwe",
        )
        self.assertEqual(
            num2words(0.5, lang="sn", to="currency", currency="ZWL"),
            "madhora zero nesendi makumi mashanu",
        )
        self.assertEqual(
            num2words(1, lang="sn", to="currency", currency="ZWL"), "dhora rimwe"
        )
        self.assertEqual(
            num2words(1.5, lang="sn", to="currency", currency="ZWL"),
            "dhora rimwe nesendi makumi mashanu",
        )
        self.assertEqual(
            num2words(0, lang="sn", to="currency", currency="ZAR"), "marandi zero"
        )
        self.assertEqual(
            num2words(0.01, lang="sn", to="currency", currency="ZAR"),
            "marandi zero nesendi rimwe",
        )
        self.assertEqual(
            num2words(0.5, lang="sn", to="currency", currency="ZAR"),
            "marandi zero nesendi makumi mashanu",
        )
        self.assertEqual(
            num2words(1, lang="sn", to="currency", currency="ZAR"), "randi rimwe"
        )
        self.assertEqual(
            num2words(1.5, lang="sn", to="currency", currency="ZAR"),
            "randi rimwe nesendi makumi mashanu",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sn", to="year"), "churu")
        self.assertEqual(
            num2words(1066, lang="sn", to="year"), "churu nemakumi matanhatu nenhatu"
        )
        self.assertEqual(
            num2words(1492, lang="sn", to="year"),
            "churu nemazana mana nemakumi mapfumbamwe nepiri",
        )
        self.assertEqual(
            num2words(1776, lang="sn", to="year"),
            "churu nemazana manomwe nemakumi manomwe nenhatu",
        )
        self.assertEqual(num2words(1800, lang="sn", to="year"), "churu nemazana masere")
        self.assertEqual(
            num2words(1900, lang="sn", to="year"), "churu nemazana mapfumbamwe"
        )
        self.assertEqual(
            num2words(1984, lang="sn", to="year"),
            "churu nemazana mapfumbamwe nemakumi masere nechina",
        )
        self.assertEqual(
            num2words(1999, lang="sn", to="year"),
            "churu nemazana mapfumbamwe nemakumi mapfumbamwe nepfumbamwe",
        )
        self.assertEqual(num2words(2000, lang="sn", to="year"), "zvuru zviviri")
        self.assertEqual(num2words(2001, lang="sn", to="year"), "zvuru zviviri nemotsi")
        self.assertEqual(num2words(2010, lang="sn", to="year"), "zvuru zviviri negumi")
        self.assertEqual(
            num2words(2020, lang="sn", to="year"), "zvuru zviviri nemakumi maviri"
        )
        self.assertEqual(
            num2words(2024, lang="sn", to="year"),
            "zvuru zviviri nemakumi maviri nechina",
        )
        self.assertEqual(num2words(2100, lang="sn", to="year"), "zvuru zviviri nezana")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sn"), "zero")
        self.assertEqual(num2words("1", lang="sn"), "motsi")
        self.assertEqual(num2words("10", lang="sn"), "gumi")
        self.assertEqual(num2words("100", lang="sn"), "zana")
        self.assertEqual(num2words("1000", lang="sn"), "churu")
        self.assertEqual(num2words("10000", lang="sn"), "zvuru gumi")
        self.assertEqual(num2words("100000", lang="sn"), "zvuru zana")
        self.assertEqual(num2words("1000000", lang="sn"), "miriyoni")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sn"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sn"), num2words("100", lang="sn"))
        self.assertEqual(num2words(1000, lang="sn"), num2words("1000", lang="sn"))
