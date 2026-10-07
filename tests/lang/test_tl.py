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


class Num2WordsTLTest(TestCase):
    """Comprehensive test cases for Tagalog/Filipino language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="tl"), "sero")
        self.assertEqual(num2words(1, lang="tl"), "isa")
        self.assertEqual(num2words(2, lang="tl"), "dalawa")
        self.assertEqual(num2words(3, lang="tl"), "tatlo")
        self.assertEqual(num2words(4, lang="tl"), "apat")
        self.assertEqual(num2words(5, lang="tl"), "lima")
        self.assertEqual(num2words(6, lang="tl"), "anim")
        self.assertEqual(num2words(7, lang="tl"), "pito")
        self.assertEqual(num2words(8, lang="tl"), "walo")
        self.assertEqual(num2words(9, lang="tl"), "siyam")
        self.assertEqual(num2words(10, lang="tl"), "sampu")
        self.assertEqual(num2words(11, lang="tl"), "sampu isa")
        self.assertEqual(num2words(12, lang="tl"), "sampu dalawa")
        self.assertEqual(num2words(13, lang="tl"), "sampu tatlo")
        self.assertEqual(num2words(14, lang="tl"), "sampu apat")
        self.assertEqual(num2words(15, lang="tl"), "sampu lima")
        self.assertEqual(num2words(16, lang="tl"), "sampu anim")
        self.assertEqual(num2words(17, lang="tl"), "sampu pito")
        self.assertEqual(num2words(18, lang="tl"), "sampu walo")
        self.assertEqual(num2words(19, lang="tl"), "sampu siyam")
        self.assertEqual(num2words(20, lang="tl"), "dalawampu")
        self.assertEqual(num2words(21, lang="tl"), "dalawampu isa")
        self.assertEqual(num2words(22, lang="tl"), "dalawampu dalawa")
        self.assertEqual(num2words(23, lang="tl"), "dalawampu tatlo")
        self.assertEqual(num2words(24, lang="tl"), "dalawampu apat")
        self.assertEqual(num2words(25, lang="tl"), "dalawampu lima")
        self.assertEqual(num2words(26, lang="tl"), "dalawampu anim")
        self.assertEqual(num2words(27, lang="tl"), "dalawampu pito")
        self.assertEqual(num2words(28, lang="tl"), "dalawampu walo")
        self.assertEqual(num2words(29, lang="tl"), "dalawampu siyam")
        self.assertEqual(num2words(30, lang="tl"), "tatlumpu")
        self.assertEqual(num2words(31, lang="tl"), "tatlumpu isa")
        self.assertEqual(num2words(35, lang="tl"), "tatlumpu lima")
        self.assertEqual(num2words(40, lang="tl"), "apatnapu")
        self.assertEqual(num2words(45, lang="tl"), "apatnapu lima")
        self.assertEqual(num2words(50, lang="tl"), "limampu")
        self.assertEqual(num2words(55, lang="tl"), "limampu lima")
        self.assertEqual(num2words(60, lang="tl"), "animnapu")
        self.assertEqual(num2words(65, lang="tl"), "animnapu lima")
        self.assertEqual(num2words(70, lang="tl"), "pitumpu")
        self.assertEqual(num2words(75, lang="tl"), "pitumpu lima")
        self.assertEqual(num2words(80, lang="tl"), "walumpu")
        self.assertEqual(num2words(85, lang="tl"), "walumpu lima")
        self.assertEqual(num2words(90, lang="tl"), "siyamnapu")
        self.assertEqual(num2words(95, lang="tl"), "siyamnapu lima")
        self.assertEqual(num2words(99, lang="tl"), "siyamnapu siyam")
        self.assertEqual(num2words(100, lang="tl"), "isa daan")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="tl"), "isa daan isa")
        self.assertEqual(num2words(110, lang="tl"), "isa daan sampu")
        self.assertEqual(num2words(111, lang="tl"), "isa daan sampu isa")
        self.assertEqual(num2words(120, lang="tl"), "isa daan dalawampu")
        self.assertEqual(num2words(125, lang="tl"), "isa daan dalawampu lima")
        self.assertEqual(num2words(150, lang="tl"), "isa daan limampu")
        self.assertEqual(num2words(175, lang="tl"), "isa daan pitumpu lima")
        self.assertEqual(num2words(199, lang="tl"), "isa daan siyamnapu siyam")
        self.assertEqual(num2words(200, lang="tl"), "dalawa daan")
        self.assertEqual(num2words(201, lang="tl"), "dalawa daan isa")
        self.assertEqual(num2words(210, lang="tl"), "dalawa daan sampu")
        self.assertEqual(num2words(220, lang="tl"), "dalawa daan dalawampu")
        self.assertEqual(num2words(250, lang="tl"), "dalawa daan limampu")
        self.assertEqual(num2words(299, lang="tl"), "dalawa daan siyamnapu siyam")
        self.assertEqual(num2words(300, lang="tl"), "tatlo daan")
        self.assertEqual(num2words(333, lang="tl"), "tatlo daan tatlumpu tatlo")
        self.assertEqual(num2words(400, lang="tl"), "apat daan")
        self.assertEqual(num2words(444, lang="tl"), "apat daan apatnapu apat")
        self.assertEqual(num2words(500, lang="tl"), "lima daan")
        self.assertEqual(num2words(555, lang="tl"), "lima daan limampu lima")
        self.assertEqual(num2words(600, lang="tl"), "anim daan")
        self.assertEqual(num2words(666, lang="tl"), "anim daan animnapu anim")
        self.assertEqual(num2words(700, lang="tl"), "pito daan")
        self.assertEqual(num2words(777, lang="tl"), "pito daan pitumpu pito")
        self.assertEqual(num2words(800, lang="tl"), "walo daan")
        self.assertEqual(num2words(888, lang="tl"), "walo daan walumpu walo")
        self.assertEqual(num2words(900, lang="tl"), "siyam daan")
        self.assertEqual(num2words(999, lang="tl"), "siyam daan siyamnapu siyam")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="tl"), "isa libo")
        self.assertEqual(num2words(1001, lang="tl"), "isa libo isa")
        self.assertEqual(num2words(1010, lang="tl"), "isa libo sampu")
        self.assertEqual(num2words(1100, lang="tl"), "isa libo isa daan")
        self.assertEqual(num2words(1111, lang="tl"), "isa libo isa daan sampu isa")
        self.assertEqual(
            num2words(1234, lang="tl"), "isa libo dalawa daan tatlumpu apat"
        )
        self.assertEqual(num2words(1500, lang="tl"), "isa libo lima daan")
        self.assertEqual(
            num2words(1999, lang="tl"), "isa libo siyam daan siyamnapu siyam"
        )
        self.assertEqual(num2words(2000, lang="tl"), "dalawa libo")
        self.assertEqual(num2words(2001, lang="tl"), "dalawa libo isa")
        self.assertEqual(num2words(2020, lang="tl"), "dalawa libo dalawampu")
        self.assertEqual(
            num2words(2222, lang="tl"), "dalawa libo dalawa daan dalawampu dalawa"
        )
        self.assertEqual(num2words(3000, lang="tl"), "tatlo libo")
        self.assertEqual(
            num2words(3333, lang="tl"), "tatlo libo tatlo daan tatlumpu tatlo"
        )
        self.assertEqual(num2words(4000, lang="tl"), "apat libo")
        self.assertEqual(
            num2words(4444, lang="tl"), "apat libo apat daan apatnapu apat"
        )
        self.assertEqual(num2words(5000, lang="tl"), "lima libo")
        self.assertEqual(num2words(5555, lang="tl"), "lima libo lima daan limampu lima")
        self.assertEqual(num2words(6000, lang="tl"), "anim libo")
        self.assertEqual(
            num2words(6666, lang="tl"), "anim libo anim daan animnapu anim"
        )
        self.assertEqual(num2words(7000, lang="tl"), "pito libo")
        self.assertEqual(num2words(7777, lang="tl"), "pito libo pito daan pitumpu pito")
        self.assertEqual(num2words(8000, lang="tl"), "walo libo")
        self.assertEqual(num2words(8888, lang="tl"), "walo libo walo daan walumpu walo")
        self.assertEqual(num2words(9000, lang="tl"), "siyam libo")
        self.assertEqual(
            num2words(9999, lang="tl"), "siyam libo siyam daan siyamnapu siyam"
        )
        self.assertEqual(num2words(10000, lang="tl"), "sampu libo")
        self.assertEqual(num2words(10001, lang="tl"), "sampu libo isa")
        self.assertEqual(
            num2words(11111, lang="tl"), "sampu isa libo isa daan sampu isa"
        )
        self.assertEqual(
            num2words(12345, lang="tl"), "sampu dalawa libo tatlo daan apatnapu lima"
        )
        self.assertEqual(num2words(20000, lang="tl"), "dalawampu libo")
        self.assertEqual(num2words(50000, lang="tl"), "limampu libo")
        self.assertEqual(
            num2words(99999, lang="tl"),
            "siyamnapu siyam libo siyam daan siyamnapu siyam",
        )
        self.assertEqual(num2words(100000, lang="tl"), "isa daan libo")
        self.assertEqual(
            num2words(123456, lang="tl"),
            "isa daan dalawampu tatlo libo apat daan limampu anim",
        )
        self.assertEqual(num2words(200000, lang="tl"), "dalawa daan libo")
        self.assertEqual(num2words(500000, lang="tl"), "lima daan libo")
        self.assertEqual(
            num2words(654321, lang="tl"),
            "anim daan limampu apat libo tatlo daan dalawampu isa",
        )
        self.assertEqual(
            num2words(999999, lang="tl"),
            "siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="tl"), "isa milyon")
        self.assertEqual(num2words(1000001, lang="tl"), "isa milyon isa")
        self.assertEqual(
            num2words(1111111, lang="tl"),
            "isa milyon isa daan sampu isa libo isa daan sampu isa",
        )
        self.assertEqual(
            num2words(1234567, lang="tl"),
            "isa milyon dalawa daan tatlumpu apat libo lima daan animnapu pito",
        )
        self.assertEqual(num2words(2000000, lang="tl"), "dalawa milyon")
        self.assertEqual(num2words(5000000, lang="tl"), "lima milyon")
        self.assertEqual(
            num2words(9999999, lang="tl"),
            "siyam milyon siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )
        self.assertEqual(num2words(10000000, lang="tl"), "sampu milyon")
        self.assertEqual(
            num2words(12345678, lang="tl"),
            "sampu dalawa milyon tatlo daan apatnapu lima libo anim daan pitumpu walo",
        )
        self.assertEqual(
            num2words(99999999, lang="tl"),
            "siyamnapu siyam milyon siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )
        self.assertEqual(num2words(100000000, lang="tl"), "isa daan milyon")
        self.assertEqual(
            num2words(123456789, lang="tl"),
            "isa daan dalawampu tatlo milyon apat daan limampu anim libo pito daan walumpu siyam",
        )
        self.assertEqual(
            num2words(999999999, lang="tl"),
            "siyam daan siyamnapu siyam milyon siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )
        self.assertEqual(num2words(1000000000, lang="tl"), "isa bilyon")
        self.assertEqual(
            num2words(1234567890, lang="tl"),
            "isa bilyon dalawa daan tatlumpu apat milyon lima daan animnapu pito libo walo daan siyamnapu",
        )
        self.assertEqual(
            num2words(9999999999, lang="tl"),
            "siyam bilyon siyam daan siyamnapu siyam milyon siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )
        self.assertEqual(num2words(10000000000, lang="tl"), "sampu bilyon")
        self.assertEqual(
            num2words(99999999999, lang="tl"),
            "siyamnapu siyam bilyon siyam daan siyamnapu siyam milyon siyam daan siyamnapu siyam libo siyam daan siyamnapu siyam",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="tl"), "minus isa")
        self.assertEqual(num2words(-2, lang="tl"), "minus dalawa")
        self.assertEqual(num2words(-5, lang="tl"), "minus lima")
        self.assertEqual(num2words(-10, lang="tl"), "minus sampu")
        self.assertEqual(num2words(-11, lang="tl"), "minus sampu isa")
        self.assertEqual(num2words(-20, lang="tl"), "minus dalawampu")
        self.assertEqual(num2words(-50, lang="tl"), "minus limampu")
        self.assertEqual(num2words(-99, lang="tl"), "minus siyamnapu siyam")
        self.assertEqual(num2words(-100, lang="tl"), "minus isa daan")
        self.assertEqual(num2words(-101, lang="tl"), "minus isa daan isa")
        self.assertEqual(num2words(-200, lang="tl"), "minus dalawa daan")
        self.assertEqual(num2words(-999, lang="tl"), "minus siyam daan siyamnapu siyam")
        self.assertEqual(num2words(-1000, lang="tl"), "minus isa libo")
        self.assertEqual(num2words(-1001, lang="tl"), "minus isa libo isa")
        self.assertEqual(num2words(-10000, lang="tl"), "minus sampu libo")
        self.assertEqual(num2words(-100000, lang="tl"), "minus isa daan libo")
        self.assertEqual(num2words(-1000000, lang="tl"), "minus isa milyon")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="tl"), "sero punto isa")
        self.assertEqual(num2words(0.5, lang="tl"), "sero punto lima")
        self.assertEqual(num2words(0.9, lang="tl"), "sero punto siyam")
        self.assertEqual(num2words(1.1, lang="tl"), "isa punto isa")
        self.assertEqual(num2words(1.5, lang="tl"), "isa punto lima")
        self.assertEqual(num2words(2.5, lang="tl"), "dalawa punto lima")
        self.assertEqual(num2words(3.14, lang="tl"), "tatlo punto isa apat")
        self.assertEqual(num2words(10.5, lang="tl"), "sampu punto lima")
        self.assertEqual(num2words(11.11, lang="tl"), "sampu isa punto isa isa")
        self.assertEqual(num2words(20.2, lang="tl"), "dalawampu punto dalawa")
        self.assertEqual(
            num2words(99.99, lang="tl"), "siyamnapu siyam punto siyam siyam"
        )
        self.assertEqual(num2words(100.01, lang="tl"), "isa daan punto sero isa")
        self.assertEqual(num2words(100.5, lang="tl"), "isa daan punto lima")
        self.assertEqual(
            num2words(123.45, lang="tl"), "isa daan dalawampu tatlo punto apat lima"
        )
        self.assertEqual(num2words(1000.5, lang="tl"), "isa libo punto lima")
        self.assertEqual(
            num2words(1234.56, lang="tl"),
            "isa libo dalawa daan tatlumpu apat punto lima anim",
        )
        self.assertEqual(num2words(10000.01, lang="tl"), "sampu libo punto sero isa")
        self.assertEqual(num2words(-0.5, lang="tl"), "minus sero punto lima")
        self.assertEqual(num2words(-1.5, lang="tl"), "minus isa punto lima")
        self.assertEqual(num2words(-10.5, lang="tl"), "minus sampu punto lima")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="tl", ordinal=True), "una")
        self.assertEqual(num2words(2, lang="tl", ordinal=True), "ikalawa")
        self.assertEqual(num2words(3, lang="tl", ordinal=True), "ika-tatlo")
        self.assertEqual(num2words(4, lang="tl", ordinal=True), "ika-apat")
        self.assertEqual(num2words(5, lang="tl", ordinal=True), "ika-lima")
        self.assertEqual(num2words(6, lang="tl", ordinal=True), "ika-anim")
        self.assertEqual(num2words(7, lang="tl", ordinal=True), "ika-pito")
        self.assertEqual(num2words(8, lang="tl", ordinal=True), "ika-walo")
        self.assertEqual(num2words(9, lang="tl", ordinal=True), "ika-siyam")
        self.assertEqual(num2words(10, lang="tl", ordinal=True), "ika-sampu")
        self.assertEqual(num2words(11, lang="tl", ordinal=True), "ika-sampu isa")
        self.assertEqual(num2words(12, lang="tl", ordinal=True), "ika-sampu dalawa")
        self.assertEqual(num2words(13, lang="tl", ordinal=True), "ika-sampu tatlo")
        self.assertEqual(num2words(14, lang="tl", ordinal=True), "ika-sampu apat")
        self.assertEqual(num2words(15, lang="tl", ordinal=True), "ika-sampu lima")
        self.assertEqual(num2words(16, lang="tl", ordinal=True), "ika-sampu anim")
        self.assertEqual(num2words(17, lang="tl", ordinal=True), "ika-sampu pito")
        self.assertEqual(num2words(18, lang="tl", ordinal=True), "ika-sampu walo")
        self.assertEqual(num2words(19, lang="tl", ordinal=True), "ika-sampu siyam")
        self.assertEqual(num2words(20, lang="tl", ordinal=True), "ika-dalawampu")
        self.assertEqual(num2words(21, lang="tl", ordinal=True), "ika-dalawampu isa")
        self.assertEqual(num2words(22, lang="tl", ordinal=True), "ika-dalawampu dalawa")
        self.assertEqual(num2words(25, lang="tl", ordinal=True), "ika-dalawampu lima")
        self.assertEqual(num2words(30, lang="tl", ordinal=True), "ika-tatlumpu")
        self.assertEqual(num2words(40, lang="tl", ordinal=True), "ika-apatnapu")
        self.assertEqual(num2words(50, lang="tl", ordinal=True), "ika-limampu")
        self.assertEqual(num2words(60, lang="tl", ordinal=True), "ika-animnapu")
        self.assertEqual(num2words(70, lang="tl", ordinal=True), "ika-pitumpu")
        self.assertEqual(num2words(80, lang="tl", ordinal=True), "ika-walumpu")
        self.assertEqual(num2words(90, lang="tl", ordinal=True), "ika-siyamnapu")
        self.assertEqual(num2words(100, lang="tl", ordinal=True), "ika-isa daan")
        self.assertEqual(num2words(101, lang="tl", ordinal=True), "ika-isa daan isa")
        self.assertEqual(num2words(200, lang="tl", ordinal=True), "ika-dalawa daan")
        self.assertEqual(num2words(500, lang="tl", ordinal=True), "ika-lima daan")
        self.assertEqual(num2words(1000, lang="tl", ordinal=True), "ika-isa libo")
        self.assertEqual(num2words(1001, lang="tl", ordinal=True), "ika-isa libo isa")
        self.assertEqual(num2words(10000, lang="tl", ordinal=True), "ika-sampu libo")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="tl", to="currency", currency="PHP"), "sero piso"
        )
        self.assertEqual(
            num2words(0.01, lang="tl", to="currency", currency="PHP"),
            "sero piso isa sentimo",
        )
        self.assertEqual(
            num2words(0.5, lang="tl", to="currency", currency="PHP"),
            "sero piso limampu sentimo",
        )
        self.assertEqual(
            num2words(1, lang="tl", to="currency", currency="PHP"), "isa piso"
        )
        self.assertEqual(
            num2words(1.5, lang="tl", to="currency", currency="PHP"),
            "isa piso limampu sentimo",
        )
        self.assertEqual(
            num2words(0, lang="tl", to="currency", currency="USD"), "sero dolyar"
        )
        self.assertEqual(
            num2words(0.01, lang="tl", to="currency", currency="USD"),
            "sero dolyar isa sentimo",
        )
        self.assertEqual(
            num2words(0.5, lang="tl", to="currency", currency="USD"),
            "sero dolyar limampu sentimo",
        )
        self.assertEqual(
            num2words(1, lang="tl", to="currency", currency="USD"), "isa dolyar"
        )
        self.assertEqual(
            num2words(1.5, lang="tl", to="currency", currency="USD"),
            "isa dolyar limampu sentimo",
        )
        self.assertEqual(
            num2words(0, lang="tl", to="currency", currency="EUR"), "sero euro"
        )
        self.assertEqual(
            num2words(0.01, lang="tl", to="currency", currency="EUR"),
            "sero euro isa sentimo",
        )
        self.assertEqual(
            num2words(0.5, lang="tl", to="currency", currency="EUR"),
            "sero euro limampu sentimo",
        )
        self.assertEqual(
            num2words(1, lang="tl", to="currency", currency="EUR"), "isa euro"
        )
        self.assertEqual(
            num2words(1.5, lang="tl", to="currency", currency="EUR"),
            "isa euro limampu sentimo",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="tl", to="year"), "isa libo")
        self.assertEqual(
            num2words(1066, lang="tl", to="year"), "isa libo animnapu anim"
        )
        self.assertEqual(
            num2words(1492, lang="tl", to="year"), "isa libo apat daan siyamnapu dalawa"
        )
        self.assertEqual(
            num2words(1776, lang="tl", to="year"), "isa libo pito daan pitumpu anim"
        )
        self.assertEqual(num2words(1800, lang="tl", to="year"), "isa libo walo daan")
        self.assertEqual(num2words(1900, lang="tl", to="year"), "isa libo siyam daan")
        self.assertEqual(
            num2words(1984, lang="tl", to="year"), "isa libo siyam daan walumpu apat"
        )
        self.assertEqual(
            num2words(1999, lang="tl", to="year"), "isa libo siyam daan siyamnapu siyam"
        )
        self.assertEqual(num2words(2000, lang="tl", to="year"), "dalawa libo")
        self.assertEqual(num2words(2001, lang="tl", to="year"), "dalawa libo isa")
        self.assertEqual(num2words(2010, lang="tl", to="year"), "dalawa libo sampu")
        self.assertEqual(num2words(2020, lang="tl", to="year"), "dalawa libo dalawampu")
        self.assertEqual(
            num2words(2024, lang="tl", to="year"), "dalawa libo dalawampu apat"
        )
        self.assertEqual(num2words(2100, lang="tl", to="year"), "dalawa libo isa daan")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="tl"), "sero")
        self.assertEqual(num2words("1", lang="tl"), "isa")
        self.assertEqual(num2words("10", lang="tl"), "sampu")
        self.assertEqual(num2words("100", lang="tl"), "isa daan")
        self.assertEqual(num2words("1000", lang="tl"), "isa libo")
        self.assertEqual(num2words("10000", lang="tl"), "sampu libo")
        self.assertEqual(num2words("100000", lang="tl"), "isa daan libo")
        self.assertEqual(num2words("1000000", lang="tl"), "isa milyon")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="tl"), "sero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="tl"), num2words("100", lang="tl"))
        self.assertEqual(num2words(1000, lang="tl"), num2words("1000", lang="tl"))
