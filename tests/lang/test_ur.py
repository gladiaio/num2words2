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


class Num2WordsURTest(TestCase):
    """Comprehensive test cases for Urdu language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ur"), "صفر")
        self.assertEqual(num2words(1, lang="ur"), "ایک")
        self.assertEqual(num2words(2, lang="ur"), "دو")
        self.assertEqual(num2words(3, lang="ur"), "تین")
        self.assertEqual(num2words(4, lang="ur"), "چار")
        self.assertEqual(num2words(5, lang="ur"), "پانچ")
        self.assertEqual(num2words(6, lang="ur"), "چھ")
        self.assertEqual(num2words(7, lang="ur"), "سات")
        self.assertEqual(num2words(8, lang="ur"), "آٹھ")
        self.assertEqual(num2words(9, lang="ur"), "نو")
        self.assertEqual(num2words(10, lang="ur"), "دس")
        self.assertEqual(num2words(11, lang="ur"), "گیارہ")
        self.assertEqual(num2words(12, lang="ur"), "بارہ")
        self.assertEqual(num2words(13, lang="ur"), "تیرہ")
        self.assertEqual(num2words(14, lang="ur"), "چودہ")
        self.assertEqual(num2words(15, lang="ur"), "پندرہ")
        self.assertEqual(num2words(16, lang="ur"), "سولہ")
        self.assertEqual(num2words(17, lang="ur"), "سترہ")
        self.assertEqual(num2words(18, lang="ur"), "اٹھارہ")
        self.assertEqual(num2words(19, lang="ur"), "انیس")
        self.assertEqual(num2words(20, lang="ur"), "بیس")
        self.assertEqual(num2words(21, lang="ur"), "اکیس")
        self.assertEqual(num2words(22, lang="ur"), "بائیس")
        self.assertEqual(num2words(23, lang="ur"), "تئیس")
        self.assertEqual(num2words(24, lang="ur"), "چوبیس")
        self.assertEqual(num2words(25, lang="ur"), "پچیس")
        self.assertEqual(num2words(26, lang="ur"), "چھبیس")
        self.assertEqual(num2words(27, lang="ur"), "ستائیس")
        self.assertEqual(num2words(28, lang="ur"), "اٹھائیس")
        self.assertEqual(num2words(29, lang="ur"), "انتیس")
        self.assertEqual(num2words(30, lang="ur"), "تیس")
        self.assertEqual(num2words(31, lang="ur"), "اکتیس")
        self.assertEqual(num2words(35, lang="ur"), "پینتیس")
        self.assertEqual(num2words(40, lang="ur"), "چالیس")
        self.assertEqual(num2words(45, lang="ur"), "پینتالیس")
        self.assertEqual(num2words(50, lang="ur"), "پچاس")
        self.assertEqual(num2words(55, lang="ur"), "پچپن")
        self.assertEqual(num2words(60, lang="ur"), "ساٹھ")
        self.assertEqual(num2words(65, lang="ur"), "پینسٹھ")
        self.assertEqual(num2words(70, lang="ur"), "ستر")
        self.assertEqual(num2words(75, lang="ur"), "پچھتر")
        self.assertEqual(num2words(80, lang="ur"), "اسی")
        self.assertEqual(num2words(85, lang="ur"), "پچاسی")
        self.assertEqual(num2words(90, lang="ur"), "نوے")
        self.assertEqual(num2words(95, lang="ur"), "پچانوے")
        self.assertEqual(num2words(99, lang="ur"), "ننانوے")
        self.assertEqual(num2words(100, lang="ur"), "ایک سو")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ur"), "ایک سو ایک")
        self.assertEqual(num2words(110, lang="ur"), "ایک سو دس")
        self.assertEqual(num2words(111, lang="ur"), "ایک سو گیارہ")
        self.assertEqual(num2words(120, lang="ur"), "ایک سو بیس")
        self.assertEqual(num2words(125, lang="ur"), "ایک سو پچیس")
        self.assertEqual(num2words(150, lang="ur"), "ایک سو پچاس")
        self.assertEqual(num2words(175, lang="ur"), "ایک سو پچھتر")
        self.assertEqual(num2words(199, lang="ur"), "ایک سو ننانوے")
        self.assertEqual(num2words(200, lang="ur"), "دو سو")
        self.assertEqual(num2words(201, lang="ur"), "دو سو ایک")
        self.assertEqual(num2words(210, lang="ur"), "دو سو دس")
        self.assertEqual(num2words(220, lang="ur"), "دو سو بیس")
        self.assertEqual(num2words(250, lang="ur"), "دو سو پچاس")
        self.assertEqual(num2words(299, lang="ur"), "دو سو ننانوے")
        self.assertEqual(num2words(300, lang="ur"), "تین سو")
        self.assertEqual(num2words(333, lang="ur"), "تین سو تینتیس")
        self.assertEqual(num2words(400, lang="ur"), "چار سو")
        self.assertEqual(num2words(444, lang="ur"), "چار سو چوالیس")
        self.assertEqual(num2words(500, lang="ur"), "پانچ سو")
        self.assertEqual(num2words(555, lang="ur"), "پانچ سو پچپن")
        self.assertEqual(num2words(600, lang="ur"), "چھ سو")
        self.assertEqual(num2words(666, lang="ur"), "چھ سو چھیاسٹھ")
        self.assertEqual(num2words(700, lang="ur"), "سات سو")
        self.assertEqual(num2words(777, lang="ur"), "سات سو ستتر")
        self.assertEqual(num2words(800, lang="ur"), "آٹھ سو")
        self.assertEqual(num2words(888, lang="ur"), "آٹھ سو اٹھاسی")
        self.assertEqual(num2words(900, lang="ur"), "نو سو")
        self.assertEqual(num2words(999, lang="ur"), "نو سو ننانوے")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ur"), "ایک ہزار")
        self.assertEqual(num2words(1001, lang="ur"), "ایک ہزار ایک")
        self.assertEqual(num2words(1010, lang="ur"), "ایک ہزار دس")
        self.assertEqual(num2words(1100, lang="ur"), "ایک ہزار ایک سو")
        self.assertEqual(num2words(1111, lang="ur"), "ایک ہزار ایک سو گیارہ")
        self.assertEqual(num2words(1234, lang="ur"), "ایک ہزار دو سو چونتیس")
        self.assertEqual(num2words(1500, lang="ur"), "ایک ہزار پانچ سو")
        self.assertEqual(num2words(1999, lang="ur"), "ایک ہزار نو سو ننانوے")
        self.assertEqual(num2words(2000, lang="ur"), "دو ہزار")
        self.assertEqual(num2words(2001, lang="ur"), "دو ہزار ایک")
        self.assertEqual(num2words(2020, lang="ur"), "دو ہزار بیس")
        self.assertEqual(num2words(2222, lang="ur"), "دو ہزار دو سو بائیس")
        self.assertEqual(num2words(3000, lang="ur"), "تین ہزار")
        self.assertEqual(num2words(3333, lang="ur"), "تین ہزار تین سو تینتیس")
        self.assertEqual(num2words(4000, lang="ur"), "چار ہزار")
        self.assertEqual(num2words(4444, lang="ur"), "چار ہزار چار سو چوالیس")
        self.assertEqual(num2words(5000, lang="ur"), "پانچ ہزار")
        self.assertEqual(num2words(5555, lang="ur"), "پانچ ہزار پانچ سو پچپن")
        self.assertEqual(num2words(6000, lang="ur"), "چھ ہزار")
        self.assertEqual(num2words(6666, lang="ur"), "چھ ہزار چھ سو چھیاسٹھ")
        self.assertEqual(num2words(7000, lang="ur"), "سات ہزار")
        self.assertEqual(num2words(7777, lang="ur"), "سات ہزار سات سو ستتر")
        self.assertEqual(num2words(8000, lang="ur"), "آٹھ ہزار")
        self.assertEqual(num2words(8888, lang="ur"), "آٹھ ہزار آٹھ سو اٹھاسی")
        self.assertEqual(num2words(9000, lang="ur"), "نو ہزار")
        self.assertEqual(num2words(9999, lang="ur"), "نو ہزار نو سو ننانوے")
        self.assertEqual(num2words(10000, lang="ur"), "دس ہزار")
        self.assertEqual(num2words(10001, lang="ur"), "دس ہزار ایک")
        self.assertEqual(num2words(11111, lang="ur"), "گیارہ ہزار ایک سو گیارہ")
        self.assertEqual(num2words(12345, lang="ur"), "بارہ ہزار تین سو پینتالیس")
        self.assertEqual(num2words(20000, lang="ur"), "بیس ہزار")
        self.assertEqual(num2words(50000, lang="ur"), "پچاس ہزار")
        self.assertEqual(num2words(99999, lang="ur"), "ننانوے ہزار نو سو ننانوے")
        self.assertEqual(num2words(100000, lang="ur"), "ایک لاکھ")
        self.assertEqual(num2words(123456, lang="ur"), "ایک لاکھ تئیس ہزار چار سو چھپن")
        self.assertEqual(num2words(200000, lang="ur"), "دو لاکھ")
        self.assertEqual(num2words(500000, lang="ur"), "پانچ لاکھ")
        self.assertEqual(num2words(654321, lang="ur"), "چھ لاکھ چون ہزار تین سو اکیس")
        self.assertEqual(
            num2words(999999, lang="ur"), "نو لاکھ ننانوے ہزار نو سو ننانوے"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ur"), "دس لاکھ")
        self.assertEqual(num2words(1000001, lang="ur"), "دس لاکھ ایک")
        self.assertEqual(
            num2words(1111111, lang="ur"), "گیارہ لاکھ گیارہ ہزار ایک سو گیارہ"
        )
        self.assertEqual(
            num2words(1234567, lang="ur"), "بارہ لاکھ چونتیس ہزار پانچ سو سڑسٹھ"
        )
        self.assertEqual(num2words(2000000, lang="ur"), "بیس لاکھ")
        self.assertEqual(num2words(5000000, lang="ur"), "پچاس لاکھ")
        self.assertEqual(
            num2words(9999999, lang="ur"), "ننانوے لاکھ ننانوے ہزار نو سو ننانوے"
        )
        self.assertEqual(num2words(10000000, lang="ur"), "ایک کروڑ")
        self.assertEqual(
            num2words(12345678, lang="ur"),
            "ایک کروڑ تئیس لاکھ پینتالیس ہزار چھ سو اٹھہتر",
        )
        self.assertEqual(
            num2words(99999999, lang="ur"),
            "نو کروڑ ننانوے لاکھ ننانوے ہزار نو سو ننانوے",
        )
        self.assertEqual(num2words(100000000, lang="ur"), "دس کروڑ")
        self.assertEqual(
            num2words(123456789, lang="ur"),
            "بارہ کروڑ چونتیس لاکھ چھپن ہزار سات سو نواسی",
        )
        self.assertEqual(
            num2words(999999999, lang="ur"),
            "ننانوے کروڑ ننانوے لاکھ ننانوے ہزار نو سو ننانوے",
        )
        self.assertEqual(num2words(1000000000, lang="ur"), "ایک ارب")
        self.assertEqual(
            num2words(1234567890, lang="ur"),
            "ایک ارب تئیس کروڑ پینتالیس لاکھ سڑسٹھ ہزار آٹھ سو نوے",
        )
        self.assertEqual(
            num2words(9999999999, lang="ur"),
            "نو ارب ننانوے کروڑ ننانوے لاکھ ننانوے ہزار نو سو ننانوے",
        )
        self.assertEqual(num2words(10000000000, lang="ur"), "دس ارب")
        self.assertEqual(
            num2words(99999999999, lang="ur"),
            "ننانوے ارب ننانوے کروڑ ننانوے لاکھ ننانوے ہزار نو سو ننانوے",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ur"), "منفی ایک")
        self.assertEqual(num2words(-2, lang="ur"), "منفی دو")
        self.assertEqual(num2words(-5, lang="ur"), "منفی پانچ")
        self.assertEqual(num2words(-10, lang="ur"), "منفی دس")
        self.assertEqual(num2words(-11, lang="ur"), "منفی گیارہ")
        self.assertEqual(num2words(-20, lang="ur"), "منفی بیس")
        self.assertEqual(num2words(-50, lang="ur"), "منفی پچاس")
        self.assertEqual(num2words(-99, lang="ur"), "منفی ننانوے")
        self.assertEqual(num2words(-100, lang="ur"), "منفی ایک سو")
        self.assertEqual(num2words(-101, lang="ur"), "منفی ایک سو ایک")
        self.assertEqual(num2words(-200, lang="ur"), "منفی دو سو")
        self.assertEqual(num2words(-999, lang="ur"), "منفی نو سو ننانوے")
        self.assertEqual(num2words(-1000, lang="ur"), "منفی ایک ہزار")
        self.assertEqual(num2words(-1001, lang="ur"), "منفی ایک ہزار ایک")
        self.assertEqual(num2words(-10000, lang="ur"), "منفی دس ہزار")
        self.assertEqual(num2words(-100000, lang="ur"), "منفی ایک لاکھ")
        self.assertEqual(num2words(-1000000, lang="ur"), "منفی دس لاکھ")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ur"), "صفر اعشاریہ ایک")
        self.assertEqual(num2words(0.5, lang="ur"), "صفر اعشاریہ پانچ")
        self.assertEqual(num2words(0.9, lang="ur"), "صفر اعشاریہ نو")
        self.assertEqual(num2words(1.1, lang="ur"), "ایک اعشاریہ ایک")
        self.assertEqual(num2words(1.5, lang="ur"), "ایک اعشاریہ پانچ")
        self.assertEqual(num2words(2.5, lang="ur"), "دو اعشاریہ پانچ")
        self.assertEqual(num2words(3.14, lang="ur"), "تین اعشاریہ ایک چار")
        self.assertEqual(num2words(10.5, lang="ur"), "دس اعشاریہ پانچ")
        self.assertEqual(num2words(11.11, lang="ur"), "گیارہ اعشاریہ ایک ایک")
        self.assertEqual(num2words(20.2, lang="ur"), "بیس اعشاریہ دو")
        self.assertEqual(num2words(99.99, lang="ur"), "ننانوے اعشاریہ نو نو")
        self.assertEqual(num2words(100.01, lang="ur"), "ایک سو اعشاریہ صفر ایک")
        self.assertEqual(num2words(100.5, lang="ur"), "ایک سو اعشاریہ پانچ")
        self.assertEqual(num2words(123.45, lang="ur"), "ایک سو تئیس اعشاریہ چار پانچ")
        self.assertEqual(num2words(1000.5, lang="ur"), "ایک ہزار اعشاریہ پانچ")
        self.assertEqual(
            num2words(1234.56, lang="ur"), "ایک ہزار دو سو چونتیس اعشاریہ پانچ چھ"
        )
        self.assertEqual(num2words(10000.01, lang="ur"), "دس ہزار اعشاریہ صفر ایک")
        self.assertEqual(num2words(-0.5, lang="ur"), "منفی صفر اعشاریہ پانچ")
        self.assertEqual(num2words(-1.5, lang="ur"), "منفی ایک اعشاریہ پانچ")
        self.assertEqual(num2words(-10.5, lang="ur"), "منفی دس اعشاریہ پانچ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ur", ordinal=True), "پہلا")
        self.assertEqual(num2words(2, lang="ur", ordinal=True), "دوسرا")
        self.assertEqual(num2words(3, lang="ur", ordinal=True), "تیسرا")
        self.assertEqual(num2words(4, lang="ur", ordinal=True), "چوتھا")
        self.assertEqual(num2words(5, lang="ur", ordinal=True), "پانچواں")
        self.assertEqual(num2words(6, lang="ur", ordinal=True), "چھٹا")
        self.assertEqual(num2words(7, lang="ur", ordinal=True), "ساتواں")
        self.assertEqual(num2words(8, lang="ur", ordinal=True), "آٹھواں")
        self.assertEqual(num2words(9, lang="ur", ordinal=True), "نواں")
        self.assertEqual(num2words(10, lang="ur", ordinal=True), "دسواں")
        self.assertEqual(num2words(11, lang="ur", ordinal=True), "گیارہواں")
        self.assertEqual(num2words(12, lang="ur", ordinal=True), "بارہواں")
        self.assertEqual(num2words(13, lang="ur", ordinal=True), "تیرہواں")
        self.assertEqual(num2words(14, lang="ur", ordinal=True), "چودہواں")
        self.assertEqual(num2words(15, lang="ur", ordinal=True), "پندرہواں")
        self.assertEqual(num2words(16, lang="ur", ordinal=True), "سولہواں")
        self.assertEqual(num2words(17, lang="ur", ordinal=True), "سترہواں")
        self.assertEqual(num2words(18, lang="ur", ordinal=True), "اٹھارہواں")
        self.assertEqual(num2words(19, lang="ur", ordinal=True), "انیسواں")
        self.assertEqual(num2words(20, lang="ur", ordinal=True), "بیسواں")
        self.assertEqual(num2words(21, lang="ur", ordinal=True), "اکیسواں")
        self.assertEqual(num2words(22, lang="ur", ordinal=True), "بائیسواں")
        self.assertEqual(num2words(25, lang="ur", ordinal=True), "پچیسواں")
        self.assertEqual(num2words(30, lang="ur", ordinal=True), "تیسواں")
        self.assertEqual(num2words(40, lang="ur", ordinal=True), "چالیسواں")
        self.assertEqual(num2words(50, lang="ur", ordinal=True), "پچاسواں")
        self.assertEqual(num2words(60, lang="ur", ordinal=True), "ساٹھواں")
        self.assertEqual(num2words(70, lang="ur", ordinal=True), "سترواں")
        self.assertEqual(num2words(80, lang="ur", ordinal=True), "اسیواں")
        self.assertEqual(num2words(90, lang="ur", ordinal=True), "نوےواں")
        self.assertEqual(num2words(100, lang="ur", ordinal=True), "ایک سوواں")
        self.assertEqual(num2words(101, lang="ur", ordinal=True), "ایک سو ایکواں")
        self.assertEqual(num2words(200, lang="ur", ordinal=True), "دو سوواں")
        self.assertEqual(num2words(500, lang="ur", ordinal=True), "پانچ سوواں")
        self.assertEqual(num2words(1000, lang="ur", ordinal=True), "ایک ہزارواں")
        self.assertEqual(num2words(1001, lang="ur", ordinal=True), "ایک ہزار ایکواں")
        self.assertEqual(num2words(10000, lang="ur", ordinal=True), "دس ہزارواں")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ur", to="currency", currency="PKR"), "صفر روپے"
        )
        self.assertEqual(
            num2words(0.01, lang="ur", to="currency", currency="PKR"),
            "صفر روپے ایک پیسہ",
        )
        self.assertEqual(
            num2words(0.5, lang="ur", to="currency", currency="PKR"),
            "صفر روپے پچاس پیسے",
        )
        self.assertEqual(
            num2words(1, lang="ur", to="currency", currency="PKR"), "ایک روپیہ"
        )
        self.assertEqual(
            num2words(1.5, lang="ur", to="currency", currency="PKR"),
            "ایک روپیہ پچاس پیسے",
        )
        self.assertEqual(
            num2words(0, lang="ur", to="currency", currency="USD"), "صفر ڈالر"
        )
        self.assertEqual(
            num2words(0.01, lang="ur", to="currency", currency="USD"),
            "صفر ڈالر ایک سینٹ",
        )
        self.assertEqual(
            num2words(0.5, lang="ur", to="currency", currency="USD"),
            "صفر ڈالر پچاس سینٹ",
        )
        self.assertEqual(
            num2words(1, lang="ur", to="currency", currency="USD"), "ایک ڈالر"
        )
        self.assertEqual(
            num2words(1.5, lang="ur", to="currency", currency="USD"),
            "ایک ڈالر پچاس سینٹ",
        )
        self.assertEqual(
            num2words(0, lang="ur", to="currency", currency="EUR"), "صفر یورو"
        )
        self.assertEqual(
            num2words(0.01, lang="ur", to="currency", currency="EUR"),
            "صفر یورو ایک سینٹ",
        )
        self.assertEqual(
            num2words(0.5, lang="ur", to="currency", currency="EUR"),
            "صفر یورو پچاس سینٹ",
        )
        self.assertEqual(
            num2words(1, lang="ur", to="currency", currency="EUR"), "ایک یورو"
        )
        self.assertEqual(
            num2words(1.5, lang="ur", to="currency", currency="EUR"),
            "ایک یورو پچاس سینٹ",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ur", to="year"), "ایک ہزار")
        self.assertEqual(num2words(1066, lang="ur", to="year"), "ایک ہزار چھیاسٹھ")
        self.assertEqual(num2words(1492, lang="ur", to="year"), "ایک ہزار چار سو بانوے")
        self.assertEqual(num2words(1776, lang="ur", to="year"), "ایک ہزار سات سو چھہتر")
        self.assertEqual(num2words(1800, lang="ur", to="year"), "ایک ہزار آٹھ سو")
        self.assertEqual(num2words(1900, lang="ur", to="year"), "ایک ہزار نو سو")
        self.assertEqual(num2words(1984, lang="ur", to="year"), "ایک ہزار نو سو چوراسی")
        self.assertEqual(num2words(1999, lang="ur", to="year"), "ایک ہزار نو سو ننانوے")
        self.assertEqual(num2words(2000, lang="ur", to="year"), "دو ہزار")
        self.assertEqual(num2words(2001, lang="ur", to="year"), "دو ہزار ایک")
        self.assertEqual(num2words(2010, lang="ur", to="year"), "دو ہزار دس")
        self.assertEqual(num2words(2020, lang="ur", to="year"), "دو ہزار بیس")
        self.assertEqual(num2words(2024, lang="ur", to="year"), "دو ہزار چوبیس")
        self.assertEqual(num2words(2100, lang="ur", to="year"), "دو ہزار ایک سو")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ur"), "صفر")
        self.assertEqual(num2words("1", lang="ur"), "ایک")
        self.assertEqual(num2words("10", lang="ur"), "دس")
        self.assertEqual(num2words("100", lang="ur"), "ایک سو")
        self.assertEqual(num2words("1000", lang="ur"), "ایک ہزار")
        self.assertEqual(num2words("10000", lang="ur"), "دس ہزار")
        self.assertEqual(num2words("100000", lang="ur"), "ایک لاکھ")
        self.assertEqual(num2words("1000000", lang="ur"), "دس لاکھ")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ur"), "صفر")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ur"), num2words("100", lang="ur"))
        self.assertEqual(num2words(1000, lang="ur"), num2words("1000", lang="ur"))
