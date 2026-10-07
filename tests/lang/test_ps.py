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


class Num2WordsPSTest(TestCase):
    """Comprehensive test cases for Pashto language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ps"), "صفر")
        self.assertEqual(num2words(1, lang="ps"), "یو")
        self.assertEqual(num2words(2, lang="ps"), "دوه")
        self.assertEqual(num2words(3, lang="ps"), "درې")
        self.assertEqual(num2words(4, lang="ps"), "څلور")
        self.assertEqual(num2words(5, lang="ps"), "پنځه")
        self.assertEqual(num2words(6, lang="ps"), "شپږ")
        self.assertEqual(num2words(7, lang="ps"), "اووه")
        self.assertEqual(num2words(8, lang="ps"), "اته")
        self.assertEqual(num2words(9, lang="ps"), "نهه")
        self.assertEqual(num2words(10, lang="ps"), "لس")
        self.assertEqual(num2words(11, lang="ps"), "لس یو")
        self.assertEqual(num2words(12, lang="ps"), "لس دوه")
        self.assertEqual(num2words(13, lang="ps"), "لس درې")
        self.assertEqual(num2words(14, lang="ps"), "لس څلور")
        self.assertEqual(num2words(15, lang="ps"), "لس پنځه")
        self.assertEqual(num2words(16, lang="ps"), "لس شپږ")
        self.assertEqual(num2words(17, lang="ps"), "لس اووه")
        self.assertEqual(num2words(18, lang="ps"), "لس اته")
        self.assertEqual(num2words(19, lang="ps"), "لس نهه")
        self.assertEqual(num2words(20, lang="ps"), "شل")
        self.assertEqual(num2words(21, lang="ps"), "شل یو")
        self.assertEqual(num2words(22, lang="ps"), "شل دوه")
        self.assertEqual(num2words(23, lang="ps"), "شل درې")
        self.assertEqual(num2words(24, lang="ps"), "شل څلور")
        self.assertEqual(num2words(25, lang="ps"), "شل پنځه")
        self.assertEqual(num2words(26, lang="ps"), "شل شپږ")
        self.assertEqual(num2words(27, lang="ps"), "شل اووه")
        self.assertEqual(num2words(28, lang="ps"), "شل اته")
        self.assertEqual(num2words(29, lang="ps"), "شل نهه")
        self.assertEqual(num2words(30, lang="ps"), "دېرش")
        self.assertEqual(num2words(31, lang="ps"), "دېرش یو")
        self.assertEqual(num2words(35, lang="ps"), "دېرش پنځه")
        self.assertEqual(num2words(40, lang="ps"), "څلوېښت")
        self.assertEqual(num2words(45, lang="ps"), "څلوېښت پنځه")
        self.assertEqual(num2words(50, lang="ps"), "پنځوس")
        self.assertEqual(num2words(55, lang="ps"), "پنځوس پنځه")
        self.assertEqual(num2words(60, lang="ps"), "شپېته")
        self.assertEqual(num2words(65, lang="ps"), "شپېته پنځه")
        self.assertEqual(num2words(70, lang="ps"), "اویا")
        self.assertEqual(num2words(75, lang="ps"), "اویا پنځه")
        self.assertEqual(num2words(80, lang="ps"), "اتیا")
        self.assertEqual(num2words(85, lang="ps"), "اتیا پنځه")
        self.assertEqual(num2words(90, lang="ps"), "نوي")
        self.assertEqual(num2words(95, lang="ps"), "نوي پنځه")
        self.assertEqual(num2words(99, lang="ps"), "نوي نهه")
        self.assertEqual(num2words(100, lang="ps"), "یو سل")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ps"), "یو سل یو")
        self.assertEqual(num2words(110, lang="ps"), "یو سل لس")
        self.assertEqual(num2words(111, lang="ps"), "یو سل لس یو")
        self.assertEqual(num2words(120, lang="ps"), "یو سل شل")
        self.assertEqual(num2words(125, lang="ps"), "یو سل شل پنځه")
        self.assertEqual(num2words(150, lang="ps"), "یو سل پنځوس")
        self.assertEqual(num2words(175, lang="ps"), "یو سل اویا پنځه")
        self.assertEqual(num2words(199, lang="ps"), "یو سل نوي نهه")
        self.assertEqual(num2words(200, lang="ps"), "دوه سل")
        self.assertEqual(num2words(201, lang="ps"), "دوه سل یو")
        self.assertEqual(num2words(210, lang="ps"), "دوه سل لس")
        self.assertEqual(num2words(220, lang="ps"), "دوه سل شل")
        self.assertEqual(num2words(250, lang="ps"), "دوه سل پنځوس")
        self.assertEqual(num2words(299, lang="ps"), "دوه سل نوي نهه")
        self.assertEqual(num2words(300, lang="ps"), "درې سل")
        self.assertEqual(num2words(333, lang="ps"), "درې سل دېرش درې")
        self.assertEqual(num2words(400, lang="ps"), "څلور سل")
        self.assertEqual(num2words(444, lang="ps"), "څلور سل څلوېښت څلور")
        self.assertEqual(num2words(500, lang="ps"), "پنځه سل")
        self.assertEqual(num2words(555, lang="ps"), "پنځه سل پنځوس پنځه")
        self.assertEqual(num2words(600, lang="ps"), "شپږ سل")
        self.assertEqual(num2words(666, lang="ps"), "شپږ سل شپېته شپږ")
        self.assertEqual(num2words(700, lang="ps"), "اووه سل")
        self.assertEqual(num2words(777, lang="ps"), "اووه سل اویا اووه")
        self.assertEqual(num2words(800, lang="ps"), "اته سل")
        self.assertEqual(num2words(888, lang="ps"), "اته سل اتیا اته")
        self.assertEqual(num2words(900, lang="ps"), "نهه سل")
        self.assertEqual(num2words(999, lang="ps"), "نهه سل نوي نهه")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ps"), "یو زره")
        self.assertEqual(num2words(1001, lang="ps"), "یو زره یو")
        self.assertEqual(num2words(1010, lang="ps"), "یو زره لس")
        self.assertEqual(num2words(1100, lang="ps"), "یو زره یو سل")
        self.assertEqual(num2words(1111, lang="ps"), "یو زره یو سل لس یو")
        self.assertEqual(num2words(1234, lang="ps"), "یو زره دوه سل دېرش څلور")
        self.assertEqual(num2words(1500, lang="ps"), "یو زره پنځه سل")
        self.assertEqual(num2words(1999, lang="ps"), "یو زره نهه سل نوي نهه")
        self.assertEqual(num2words(2000, lang="ps"), "دوه زره")
        self.assertEqual(num2words(2001, lang="ps"), "دوه زره یو")
        self.assertEqual(num2words(2020, lang="ps"), "دوه زره شل")
        self.assertEqual(num2words(2222, lang="ps"), "دوه زره دوه سل شل دوه")
        self.assertEqual(num2words(3000, lang="ps"), "درې زره")
        self.assertEqual(num2words(3333, lang="ps"), "درې زره درې سل دېرش درې")
        self.assertEqual(num2words(4000, lang="ps"), "څلور زره")
        self.assertEqual(num2words(4444, lang="ps"), "څلور زره څلور سل څلوېښت څلور")
        self.assertEqual(num2words(5000, lang="ps"), "پنځه زره")
        self.assertEqual(num2words(5555, lang="ps"), "پنځه زره پنځه سل پنځوس پنځه")
        self.assertEqual(num2words(6000, lang="ps"), "شپږ زره")
        self.assertEqual(num2words(6666, lang="ps"), "شپږ زره شپږ سل شپېته شپږ")
        self.assertEqual(num2words(7000, lang="ps"), "اووه زره")
        self.assertEqual(num2words(7777, lang="ps"), "اووه زره اووه سل اویا اووه")
        self.assertEqual(num2words(8000, lang="ps"), "اته زره")
        self.assertEqual(num2words(8888, lang="ps"), "اته زره اته سل اتیا اته")
        self.assertEqual(num2words(9000, lang="ps"), "نهه زره")
        self.assertEqual(num2words(9999, lang="ps"), "نهه زره نهه سل نوي نهه")
        self.assertEqual(num2words(10000, lang="ps"), "لس زره")
        self.assertEqual(num2words(10001, lang="ps"), "لس زره یو")
        self.assertEqual(num2words(11111, lang="ps"), "لس یو زره یو سل لس یو")
        self.assertEqual(num2words(12345, lang="ps"), "لس دوه زره درې سل څلوېښت پنځه")
        self.assertEqual(num2words(20000, lang="ps"), "شل زره")
        self.assertEqual(num2words(50000, lang="ps"), "پنځوس زره")
        self.assertEqual(num2words(99999, lang="ps"), "نوي نهه زره نهه سل نوي نهه")
        self.assertEqual(num2words(100000, lang="ps"), "یو سل زره")
        self.assertEqual(
            num2words(123456, lang="ps"), "یو سل شل درې زره څلور سل پنځوس شپږ"
        )
        self.assertEqual(num2words(200000, lang="ps"), "دوه سل زره")
        self.assertEqual(num2words(500000, lang="ps"), "پنځه سل زره")
        self.assertEqual(
            num2words(654321, lang="ps"), "شپږ سل پنځوس څلور زره درې سل شل یو"
        )
        self.assertEqual(
            num2words(999999, lang="ps"), "نهه سل نوي نهه زره نهه سل نوي نهه"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ps"), "یو میلیون")
        self.assertEqual(num2words(1000001, lang="ps"), "یو میلیون یو")
        self.assertEqual(
            num2words(1111111, lang="ps"), "یو میلیون یو سل لس یو زره یو سل لس یو"
        )
        self.assertEqual(
            num2words(1234567, lang="ps"),
            "یو میلیون دوه سل دېرش څلور زره پنځه سل شپېته اووه",
        )
        self.assertEqual(num2words(2000000, lang="ps"), "دوه میلیون")
        self.assertEqual(num2words(5000000, lang="ps"), "پنځه میلیون")
        self.assertEqual(
            num2words(9999999, lang="ps"),
            "نهه میلیون نهه سل نوي نهه زره نهه سل نوي نهه",
        )
        self.assertEqual(num2words(10000000, lang="ps"), "لس میلیون")
        self.assertEqual(
            num2words(12345678, lang="ps"),
            "لس دوه میلیون درې سل څلوېښت پنځه زره شپږ سل اویا اته",
        )
        self.assertEqual(
            num2words(99999999, lang="ps"),
            "نوي نهه میلیون نهه سل نوي نهه زره نهه سل نوي نهه",
        )
        self.assertEqual(num2words(100000000, lang="ps"), "یو سل میلیون")
        self.assertEqual(
            num2words(123456789, lang="ps"),
            "یو سل شل درې میلیون څلور سل پنځوس شپږ زره اووه سل اتیا نهه",
        )
        self.assertEqual(
            num2words(999999999, lang="ps"),
            "نهه سل نوي نهه میلیون نهه سل نوي نهه زره نهه سل نوي نهه",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="ps"),
            "یو میلیارد",
        )
        self.assertEqual(
            num2words(1234567890, lang="ps"),
            "یو میلیارد دوه سل دېرش څلور میلیون پنځه سل شپېته اووه زره اته سل نوي",
        )
        self.assertEqual(
            num2words(9999999999, lang="ps"),
            "نهه میلیارد نهه سل نوي نهه میلیون نهه سل نوي نهه زره نهه سل نوي نهه",
        )
        self.assertEqual(
            num2words(10000000000, lang="ps"),
            "لس میلیارد",
        )
        self.assertEqual(
            num2words(99999999999, lang="ps"),
            "نوي نهه میلیارد نهه سل نوي نهه میلیون نهه سل نوي نهه زره نهه سل نوي نهه",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ps"), "منفي یو")
        self.assertEqual(num2words(-2, lang="ps"), "منفي دوه")
        self.assertEqual(num2words(-5, lang="ps"), "منفي پنځه")
        self.assertEqual(num2words(-10, lang="ps"), "منفي لس")
        self.assertEqual(num2words(-11, lang="ps"), "منفي لس یو")
        self.assertEqual(num2words(-20, lang="ps"), "منفي شل")
        self.assertEqual(num2words(-50, lang="ps"), "منفي پنځوس")
        self.assertEqual(num2words(-99, lang="ps"), "منفي نوي نهه")
        self.assertEqual(num2words(-100, lang="ps"), "منفي یو سل")
        self.assertEqual(num2words(-101, lang="ps"), "منفي یو سل یو")
        self.assertEqual(num2words(-200, lang="ps"), "منفي دوه سل")
        self.assertEqual(num2words(-999, lang="ps"), "منفي نهه سل نوي نهه")
        self.assertEqual(num2words(-1000, lang="ps"), "منفي یو زره")
        self.assertEqual(num2words(-1001, lang="ps"), "منفي یو زره یو")
        self.assertEqual(num2words(-10000, lang="ps"), "منفي لس زره")
        self.assertEqual(num2words(-100000, lang="ps"), "منفي یو سل زره")
        self.assertEqual(num2words(-1000000, lang="ps"), "منفي یو میلیون")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ps"), "صفر اعشاریه یو")
        self.assertEqual(num2words(0.5, lang="ps"), "صفر اعشاریه پنځه")
        self.assertEqual(num2words(0.9, lang="ps"), "صفر اعشاریه نهه")
        self.assertEqual(num2words(1.1, lang="ps"), "یو اعشاریه یو")
        self.assertEqual(num2words(1.5, lang="ps"), "یو اعشاریه پنځه")
        self.assertEqual(num2words(2.5, lang="ps"), "دوه اعشاریه پنځه")
        self.assertEqual(num2words(3.14, lang="ps"), "درې اعشاریه یو څلور")
        self.assertEqual(num2words(10.5, lang="ps"), "لس اعشاریه پنځه")
        self.assertEqual(num2words(11.11, lang="ps"), "لس یو اعشاریه یو یو")
        self.assertEqual(num2words(20.2, lang="ps"), "شل اعشاریه دوه")
        self.assertEqual(num2words(99.99, lang="ps"), "نوي نهه اعشاریه نهه نهه")
        self.assertEqual(num2words(100.01, lang="ps"), "یو سل اعشاریه صفر یو")
        self.assertEqual(num2words(100.5, lang="ps"), "یو سل اعشاریه پنځه")
        self.assertEqual(num2words(123.45, lang="ps"), "یو سل شل درې اعشاریه څلور پنځه")
        self.assertEqual(num2words(1000.5, lang="ps"), "یو زره اعشاریه پنځه")
        self.assertEqual(
            num2words(1234.56, lang="ps"), "یو زره دوه سل دېرش څلور اعشاریه پنځه شپږ"
        )
        self.assertEqual(num2words(10000.01, lang="ps"), "لس زره اعشاریه صفر یو")
        self.assertEqual(num2words(-0.5, lang="ps"), "منفي صفر اعشاریه پنځه")
        self.assertEqual(num2words(-1.5, lang="ps"), "منفي یو اعشاریه پنځه")
        self.assertEqual(num2words(-10.5, lang="ps"), "منفي لس اعشاریه پنځه")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ps", ordinal=True), "یو-م")
        self.assertEqual(num2words(2, lang="ps", ordinal=True), "دوه-م")
        self.assertEqual(num2words(3, lang="ps", ordinal=True), "درې-م")
        self.assertEqual(num2words(4, lang="ps", ordinal=True), "څلور-م")
        self.assertEqual(num2words(5, lang="ps", ordinal=True), "پنځه-م")
        self.assertEqual(num2words(6, lang="ps", ordinal=True), "شپږ-م")
        self.assertEqual(num2words(7, lang="ps", ordinal=True), "اووه-م")
        self.assertEqual(num2words(8, lang="ps", ordinal=True), "اته-م")
        self.assertEqual(num2words(9, lang="ps", ordinal=True), "نهه-م")
        self.assertEqual(num2words(10, lang="ps", ordinal=True), "لس-م")
        self.assertEqual(num2words(11, lang="ps", ordinal=True), "لس یو-م")
        self.assertEqual(num2words(12, lang="ps", ordinal=True), "لس دوه-م")
        self.assertEqual(num2words(13, lang="ps", ordinal=True), "لس درې-م")
        self.assertEqual(num2words(14, lang="ps", ordinal=True), "لس څلور-م")
        self.assertEqual(num2words(15, lang="ps", ordinal=True), "لس پنځه-م")
        self.assertEqual(num2words(16, lang="ps", ordinal=True), "لس شپږ-م")
        self.assertEqual(num2words(17, lang="ps", ordinal=True), "لس اووه-م")
        self.assertEqual(num2words(18, lang="ps", ordinal=True), "لس اته-م")
        self.assertEqual(num2words(19, lang="ps", ordinal=True), "لس نهه-م")
        self.assertEqual(num2words(20, lang="ps", ordinal=True), "شل-م")
        self.assertEqual(num2words(21, lang="ps", ordinal=True), "شل یو-م")
        self.assertEqual(num2words(22, lang="ps", ordinal=True), "شل دوه-م")
        self.assertEqual(num2words(25, lang="ps", ordinal=True), "شل پنځه-م")
        self.assertEqual(num2words(30, lang="ps", ordinal=True), "دېرش-م")
        self.assertEqual(num2words(40, lang="ps", ordinal=True), "څلوېښت-م")
        self.assertEqual(num2words(50, lang="ps", ordinal=True), "پنځوس-م")
        self.assertEqual(num2words(60, lang="ps", ordinal=True), "شپېته-م")
        self.assertEqual(num2words(70, lang="ps", ordinal=True), "اویا-م")
        self.assertEqual(num2words(80, lang="ps", ordinal=True), "اتیا-م")
        self.assertEqual(num2words(90, lang="ps", ordinal=True), "نوي-م")
        self.assertEqual(num2words(100, lang="ps", ordinal=True), "یو سل-م")
        self.assertEqual(num2words(101, lang="ps", ordinal=True), "یو سل یو-م")
        self.assertEqual(num2words(200, lang="ps", ordinal=True), "دوه سل-م")
        self.assertEqual(num2words(500, lang="ps", ordinal=True), "پنځه سل-م")
        self.assertEqual(num2words(1000, lang="ps", ordinal=True), "یو زره-م")
        self.assertEqual(num2words(1001, lang="ps", ordinal=True), "یو زره یو-م")
        self.assertEqual(num2words(10000, lang="ps", ordinal=True), "لس زره-م")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ps", to="currency", currency="AFN"), "صفر افغانۍ"
        )
        self.assertEqual(
            num2words(0.01, lang="ps", to="currency", currency="AFN"),
            "صفر افغانۍ یو پول",
        )
        self.assertEqual(
            num2words(0.5, lang="ps", to="currency", currency="AFN"),
            "صفر افغانۍ پنځوس پول",
        )
        self.assertEqual(
            num2words(1, lang="ps", to="currency", currency="AFN"), "یو افغانۍ"
        )
        self.assertEqual(
            num2words(1.5, lang="ps", to="currency", currency="AFN"),
            "یو افغانۍ پنځوس پول",
        )
        self.assertEqual(
            num2words(0, lang="ps", to="currency", currency="USD"), "صفر ډالر"
        )
        self.assertEqual(
            num2words(0.01, lang="ps", to="currency", currency="USD"),
            "صفر ډالر یو سنټ",
        )
        self.assertEqual(
            num2words(0.5, lang="ps", to="currency", currency="USD"),
            "صفر ډالر پنځوس سنټ",
        )
        self.assertEqual(
            num2words(1, lang="ps", to="currency", currency="USD"), "یو ډالر"
        )
        self.assertEqual(
            num2words(1.5, lang="ps", to="currency", currency="USD"),
            "یو ډالر پنځوس سنټ",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="ps", to="currency", currency="EUR")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ps", to="year"), "یو زره")
        self.assertEqual(num2words(1066, lang="ps", to="year"), "یو زره شپېته شپږ")
        self.assertEqual(
            num2words(1492, lang="ps", to="year"), "یو زره څلور سل نوي دوه"
        )
        self.assertEqual(
            num2words(1776, lang="ps", to="year"), "یو زره اووه سل اویا شپږ"
        )
        self.assertEqual(num2words(1800, lang="ps", to="year"), "یو زره اته سل")
        self.assertEqual(num2words(1900, lang="ps", to="year"), "یو زره نهه سل")
        self.assertEqual(
            num2words(1984, lang="ps", to="year"), "یو زره نهه سل اتیا څلور"
        )
        self.assertEqual(num2words(1999, lang="ps", to="year"), "یو زره نهه سل نوي نهه")
        self.assertEqual(num2words(2000, lang="ps", to="year"), "دوه زره")
        self.assertEqual(num2words(2001, lang="ps", to="year"), "دوه زره یو")
        self.assertEqual(num2words(2010, lang="ps", to="year"), "دوه زره لس")
        self.assertEqual(num2words(2020, lang="ps", to="year"), "دوه زره شل")
        self.assertEqual(num2words(2024, lang="ps", to="year"), "دوه زره شل څلور")
        self.assertEqual(num2words(2100, lang="ps", to="year"), "دوه زره یو سل")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ps"), "صفر")
        self.assertEqual(num2words("1", lang="ps"), "یو")
        self.assertEqual(num2words("10", lang="ps"), "لس")
        self.assertEqual(num2words("100", lang="ps"), "یو سل")
        self.assertEqual(num2words("1000", lang="ps"), "یو زره")
        self.assertEqual(num2words("10000", lang="ps"), "لس زره")
        self.assertEqual(num2words("100000", lang="ps"), "یو سل زره")
        self.assertEqual(num2words("1000000", lang="ps"), "یو میلیون")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ps"), "صفر")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ps"), num2words("100", lang="ps"))
        self.assertEqual(num2words(1000, lang="ps"), num2words("1000", lang="ps"))

