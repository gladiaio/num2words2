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


class Num2WordsMYTest(TestCase):
    """Comprehensive test cases for Burmese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="my"), "သုည")
        self.assertEqual(num2words(1, lang="my"), "တစ်")
        self.assertEqual(num2words(2, lang="my"), "နှစ်")
        self.assertEqual(num2words(3, lang="my"), "သုံး")
        self.assertEqual(num2words(4, lang="my"), "လေး")
        self.assertEqual(num2words(5, lang="my"), "ငါး")
        self.assertEqual(num2words(6, lang="my"), "ခြောက်")
        self.assertEqual(num2words(7, lang="my"), "ခုနစ်")
        self.assertEqual(num2words(8, lang="my"), "ရှစ်")
        self.assertEqual(num2words(9, lang="my"), "ကိုး")
        self.assertEqual(num2words(10, lang="my"), "တစ်ဆယ်")
        self.assertEqual(num2words(11, lang="my"), "တစ်ဆယ့်တစ်")
        self.assertEqual(num2words(12, lang="my"), "တစ်ဆယ့်နှစ်")
        self.assertEqual(num2words(13, lang="my"), "တစ်ဆယ့်သုံး")
        self.assertEqual(num2words(14, lang="my"), "တစ်ဆယ့်လေး")
        self.assertEqual(num2words(15, lang="my"), "တစ်ဆယ့်ငါး")
        self.assertEqual(num2words(16, lang="my"), "တစ်ဆယ့်ခြောက်")
        self.assertEqual(num2words(17, lang="my"), "တစ်ဆယ့်ခုနစ်")
        self.assertEqual(num2words(18, lang="my"), "တစ်ဆယ့်ရှစ်")
        self.assertEqual(num2words(19, lang="my"), "တစ်ဆယ့်ကိုး")
        self.assertEqual(num2words(20, lang="my"), "နှစ်ဆယ်")
        self.assertEqual(num2words(21, lang="my"), "နှစ်ဆယ့်တစ်")
        self.assertEqual(num2words(22, lang="my"), "နှစ်ဆယ့်နှစ်")
        self.assertEqual(num2words(23, lang="my"), "နှစ်ဆယ့်သုံး")
        self.assertEqual(num2words(24, lang="my"), "နှစ်ဆယ့်လေး")
        self.assertEqual(num2words(25, lang="my"), "နှစ်ဆယ့်ငါး")
        self.assertEqual(num2words(26, lang="my"), "နှစ်ဆယ့်ခြောက်")
        self.assertEqual(num2words(27, lang="my"), "နှစ်ဆယ့်ခုနစ်")
        self.assertEqual(num2words(28, lang="my"), "နှစ်ဆယ့်ရှစ်")
        self.assertEqual(num2words(29, lang="my"), "နှစ်ဆယ့်ကိုး")
        self.assertEqual(num2words(30, lang="my"), "သုံးဆယ်")
        self.assertEqual(num2words(31, lang="my"), "သုံးဆယ့်တစ်")
        self.assertEqual(num2words(35, lang="my"), "သုံးဆယ့်ငါး")
        self.assertEqual(num2words(40, lang="my"), "လေးဆယ်")
        self.assertEqual(num2words(45, lang="my"), "လေးဆယ့်ငါး")
        self.assertEqual(num2words(50, lang="my"), "ငါးဆယ်")
        self.assertEqual(num2words(55, lang="my"), "ငါးဆယ့်ငါး")
        self.assertEqual(num2words(60, lang="my"), "ခြောက်ဆယ်")
        self.assertEqual(num2words(65, lang="my"), "ခြောက်ဆယ့်ငါး")
        self.assertEqual(num2words(70, lang="my"), "ခုနစ်ဆယ်")
        self.assertEqual(num2words(75, lang="my"), "ခုနစ်ဆယ့်ငါး")
        self.assertEqual(num2words(80, lang="my"), "ရှစ်ဆယ်")
        self.assertEqual(num2words(85, lang="my"), "ရှစ်ဆယ့်ငါး")
        self.assertEqual(num2words(90, lang="my"), "ကိုးဆယ်")
        self.assertEqual(num2words(95, lang="my"), "ကိုးဆယ့်ငါး")
        self.assertEqual(num2words(99, lang="my"), "ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(100, lang="my"), "တစ်ရာ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="my"), "တစ်ရာ့တစ်")
        self.assertEqual(num2words(110, lang="my"), "တစ်ရာ့တစ်ဆယ်")
        self.assertEqual(num2words(111, lang="my"), "တစ်ရာ့တစ်ဆယ့်တစ်")
        self.assertEqual(num2words(120, lang="my"), "တစ်ရာ့နှစ်ဆယ်")
        self.assertEqual(num2words(125, lang="my"), "တစ်ရာ့နှစ်ဆယ့်ငါး")
        self.assertEqual(num2words(150, lang="my"), "တစ်ရာ့ငါးဆယ်")
        self.assertEqual(num2words(175, lang="my"), "တစ်ရာ့ခုနစ်ဆယ့်ငါး")
        self.assertEqual(num2words(199, lang="my"), "တစ်ရာ့ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(200, lang="my"), "နှစ်ရာ")
        self.assertEqual(num2words(201, lang="my"), "နှစ်ရာ့တစ်")
        self.assertEqual(num2words(210, lang="my"), "နှစ်ရာ့တစ်ဆယ်")
        self.assertEqual(num2words(220, lang="my"), "နှစ်ရာ့နှစ်ဆယ်")
        self.assertEqual(num2words(250, lang="my"), "နှစ်ရာ့ငါးဆယ်")
        self.assertEqual(num2words(299, lang="my"), "နှစ်ရာ့ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(300, lang="my"), "သုံးရာ")
        self.assertEqual(num2words(333, lang="my"), "သုံးရာ့သုံးဆယ့်သုံး")
        self.assertEqual(num2words(400, lang="my"), "လေးရာ")
        self.assertEqual(num2words(444, lang="my"), "လေးရာ့လေးဆယ့်လေး")
        self.assertEqual(num2words(500, lang="my"), "ငါးရာ")
        self.assertEqual(num2words(555, lang="my"), "ငါးရာ့ငါးဆယ့်ငါး")
        self.assertEqual(num2words(600, lang="my"), "ခြောက်ရာ")
        self.assertEqual(num2words(666, lang="my"), "ခြောက်ရာ့ခြောက်ဆယ့်ခြောက်")
        self.assertEqual(num2words(700, lang="my"), "ခုနစ်ရာ")
        self.assertEqual(num2words(777, lang="my"), "ခုနစ်ရာ့ခုနစ်ဆယ့်ခုနစ်")
        self.assertEqual(num2words(800, lang="my"), "ရှစ်ရာ")
        self.assertEqual(num2words(888, lang="my"), "ရှစ်ရာ့ရှစ်ဆယ့်ရှစ်")
        self.assertEqual(num2words(900, lang="my"), "ကိုးရာ")
        self.assertEqual(num2words(999, lang="my"), "ကိုးရာ့ကိုးဆယ့်ကိုး")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="my"), "တစ်ထောင်")
        self.assertEqual(num2words(1001, lang="my"), "တစ်ထောင့်တစ်")
        self.assertEqual(num2words(1010, lang="my"), "တစ်ထောင့်တစ်ဆယ်")
        self.assertEqual(num2words(1100, lang="my"), "တစ်ထောင့်တစ်ရာ")
        self.assertEqual(num2words(1111, lang="my"), "တစ်ထောင့်တစ်ရာ့တစ်ဆယ့်တစ်")
        self.assertEqual(num2words(1234, lang="my"), "တစ်ထောင့်နှစ်ရာ့သုံးဆယ့်လေး")
        self.assertEqual(num2words(1500, lang="my"), "တစ်ထောင့်ငါးရာ")
        self.assertEqual(num2words(1999, lang="my"), "တစ်ထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(2000, lang="my"), "နှစ်ထောင်")
        self.assertEqual(num2words(2001, lang="my"), "နှစ်ထောင့်တစ်")
        self.assertEqual(num2words(2020, lang="my"), "နှစ်ထောင့်နှစ်ဆယ်")
        self.assertEqual(num2words(2222, lang="my"), "နှစ်ထောင့်နှစ်ရာ့နှစ်ဆယ့်နှစ်")
        self.assertEqual(num2words(3000, lang="my"), "သုံးထောင်")
        self.assertEqual(num2words(3333, lang="my"), "သုံးထောင့်သုံးရာ့သုံးဆယ့်သုံး")
        self.assertEqual(num2words(4000, lang="my"), "လေးထောင်")
        self.assertEqual(num2words(4444, lang="my"), "လေးထောင့်လေးရာ့လေးဆယ့်လေး")
        self.assertEqual(num2words(5000, lang="my"), "ငါးထောင်")
        self.assertEqual(num2words(5555, lang="my"), "ငါးထောင့်ငါးရာ့ငါးဆယ့်ငါး")
        self.assertEqual(num2words(6000, lang="my"), "ခြောက်ထောင်")
        self.assertEqual(
            num2words(6666, lang="my"), "ခြောက်ထောင့်ခြောက်ရာ့ခြောက်ဆယ့်ခြောက်"
        )
        self.assertEqual(num2words(7000, lang="my"), "ခုနစ်ထောင်")
        self.assertEqual(
            num2words(7777, lang="my"), "ခုနစ်ထောင့်ခုနစ်ရာ့ခုနစ်ဆယ့်ခုနစ်"
        )
        self.assertEqual(num2words(8000, lang="my"), "ရှစ်ထောင်")
        self.assertEqual(num2words(8888, lang="my"), "ရှစ်ထောင့်ရှစ်ရာ့ရှစ်ဆယ့်ရှစ်")
        self.assertEqual(num2words(9000, lang="my"), "ကိုးထောင်")
        self.assertEqual(num2words(9999, lang="my"), "ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(10000, lang="my"), "တစ်သောင်း")
        self.assertEqual(num2words(10001, lang="my"), "တစ်သောင်း တစ်")
        self.assertEqual(
            num2words(11111, lang="my"), "တစ်သောင်း တစ်ထောင့်တစ်ရာ့တစ်ဆယ့်တစ်"
        )
        self.assertEqual(
            num2words(12345, lang="my"), "တစ်သောင်း နှစ်ထောင့်သုံးရာ့လေးဆယ့်ငါး"
        )
        self.assertEqual(num2words(20000, lang="my"), "နှစ်သောင်း")
        self.assertEqual(num2words(50000, lang="my"), "ငါးသောင်း")
        self.assertEqual(
            num2words(99999, lang="my"), "ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး"
        )
        self.assertEqual(num2words(100000, lang="my"), "တစ်သိန်း")
        self.assertEqual(
            num2words(123456, lang="my"),
            "တစ်သိန်း နှစ်သောင်း သုံးထောင့်လေးရာ့ငါးဆယ့်ခြောက်",
        )
        self.assertEqual(num2words(200000, lang="my"), "နှစ်သိန်း")
        self.assertEqual(num2words(500000, lang="my"), "ငါးသိန်း")
        self.assertEqual(
            num2words(654321, lang="my"),
            "ခြောက်သိန်း ငါးသောင်း လေးထောင့်သုံးရာ့နှစ်ဆယ့်တစ်",
        )
        self.assertEqual(
            num2words(999999, lang="my"),
            "ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="my"), "တစ်သန်း")
        self.assertEqual(num2words(1000001, lang="my"), "တစ်သန်း တစ်")
        self.assertEqual(
            num2words(1111111, lang="my"),
            "တစ်သန်း တစ်သိန်း တစ်သောင်း တစ်ထောင့်တစ်ရာ့တစ်ဆယ့်တစ်",
        )
        self.assertEqual(
            num2words(1234567, lang="my"),
            "တစ်သန်း နှစ်သိန်း သုံးသောင်း လေးထောင့်ငါးရာ့ခြောက်ဆယ့်ခုနစ်",
        )
        self.assertEqual(num2words(2000000, lang="my"), "နှစ်သန်း")
        self.assertEqual(num2words(5000000, lang="my"), "ငါးသန်း")
        self.assertEqual(
            num2words(9999999, lang="my"),
            "ကိုးသန်း ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )
        self.assertEqual(num2words(10000000, lang="my"), "တစ် ကုဋေ")
        self.assertEqual(
            num2words(12345678, lang="my"),
            "တစ် ကုဋေ နှစ်သန်း သုံးသိန်း လေးသောင်း ငါးထောင့်ခြောက်ရာ့ခုနစ်ဆယ့်ရှစ်",
        )
        self.assertEqual(
            num2words(99999999, lang="my"),
            "ကိုး ကုဋေ ကိုးသန်း ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )
        self.assertEqual(num2words(100000000, lang="my"), "တစ်ဆယ် ကုဋေ")
        self.assertEqual(
            num2words(123456789, lang="my"),
            "တစ်ဆယ့်နှစ် ကုဋေ သုံးသန်း လေးသိန်း ငါးသောင်း ခြောက်ထောင့်ခုနစ်ရာ့ရှစ်ဆယ့်ကိုး",
        )
        self.assertEqual(
            num2words(999999999, lang="my"),
            "ကိုးဆယ့်ကိုး ကုဋေ ကိုးသန်း ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="my"),
            "တစ်ရာ ကုဋေ",
        )
        self.assertEqual(
            num2words(1234567890, lang="my"),
            "တစ်ရာ့နှစ်ဆယ့်သုံး ကုဋေ လေးသန်း ငါးသိန်း ခြောက်သောင်း ခုနစ်ထောင့်ရှစ်ရာ့ကိုးဆယ်",
        )
        self.assertEqual(
            num2words(9999999999, lang="my"),
            "ကိုးရာ့ကိုးဆယ့်ကိုး ကုဋေ ကိုးသန်း ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )
        self.assertEqual(
            num2words(10000000000, lang="my"),
            "တစ်ထောင် ကုဋေ",
        )
        self.assertEqual(
            num2words(99999999999, lang="my"),
            "ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး ကုဋေ ကိုးသန်း ကိုးသိန်း ကိုးသောင်း ကိုးထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="my"), "အနုတ် တစ်")
        self.assertEqual(num2words(-2, lang="my"), "အနုတ် နှစ်")
        self.assertEqual(num2words(-5, lang="my"), "အနုတ် ငါး")
        self.assertEqual(num2words(-10, lang="my"), "အနုတ် တစ်ဆယ်")
        self.assertEqual(num2words(-11, lang="my"), "အနုတ် တစ်ဆယ့်တစ်")
        self.assertEqual(num2words(-20, lang="my"), "အနုတ် နှစ်ဆယ်")
        self.assertEqual(num2words(-50, lang="my"), "အနုတ် ငါးဆယ်")
        self.assertEqual(num2words(-99, lang="my"), "အနုတ် ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(-100, lang="my"), "အနုတ် တစ်ရာ")
        self.assertEqual(num2words(-101, lang="my"), "အနုတ် တစ်ရာ့တစ်")
        self.assertEqual(num2words(-200, lang="my"), "အနုတ် နှစ်ရာ")
        self.assertEqual(num2words(-999, lang="my"), "အနုတ် ကိုးရာ့ကိုးဆယ့်ကိုး")
        self.assertEqual(num2words(-1000, lang="my"), "အနုတ် တစ်ထောင်")
        self.assertEqual(num2words(-1001, lang="my"), "အနုတ် တစ်ထောင့်တစ်")
        self.assertEqual(num2words(-10000, lang="my"), "အနုတ် တစ်သောင်း")
        self.assertEqual(num2words(-100000, lang="my"), "အနုတ် တစ်သိန်း")
        self.assertEqual(num2words(-1000000, lang="my"), "အနုတ် တစ်သန်း")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="my"), "သုည ဒသမ တစ်")
        self.assertEqual(num2words(0.5, lang="my"), "သုည ဒသမ ငါး")
        self.assertEqual(num2words(0.9, lang="my"), "သုည ဒသမ ကိုး")
        self.assertEqual(num2words(1.1, lang="my"), "တစ် ဒသမ တစ်")
        self.assertEqual(num2words(1.5, lang="my"), "တစ် ဒသမ ငါး")
        self.assertEqual(num2words(2.5, lang="my"), "နှစ် ဒသမ ငါး")
        self.assertEqual(num2words(3.14, lang="my"), "သုံး ဒသမ တစ် လေး")
        self.assertEqual(num2words(10.5, lang="my"), "တစ်ဆယ် ဒသမ ငါး")
        self.assertEqual(num2words(11.11, lang="my"), "တစ်ဆယ့်တစ် ဒသမ တစ် တစ်")
        self.assertEqual(num2words(20.2, lang="my"), "နှစ်ဆယ် ဒသမ နှစ်")
        self.assertEqual(num2words(99.99, lang="my"), "ကိုးဆယ့်ကိုး ဒသမ ကိုး ကိုး")
        self.assertEqual(num2words(100.01, lang="my"), "တစ်ရာ ဒသမ သုည တစ်")
        self.assertEqual(num2words(100.5, lang="my"), "တစ်ရာ ဒသမ ငါး")
        self.assertEqual(num2words(123.45, lang="my"), "တစ်ရာ့နှစ်ဆယ့်သုံး ဒသမ လေး ငါး")
        self.assertEqual(num2words(1000.5, lang="my"), "တစ်ထောင် ဒသမ ငါး")
        self.assertEqual(
            num2words(1234.56, lang="my"), "တစ်ထောင့်နှစ်ရာ့သုံးဆယ့်လေး ဒသမ ငါး ခြောက်"
        )
        self.assertEqual(num2words(10000.01, lang="my"), "တစ်သောင်း ဒသမ သုည တစ်")
        self.assertEqual(num2words(-0.5, lang="my"), "အနုတ် သုည ဒသမ ငါး")
        self.assertEqual(num2words(-1.5, lang="my"), "အနုတ် တစ် ဒသမ ငါး")
        self.assertEqual(num2words(-10.5, lang="my"), "အနုတ် တစ်ဆယ် ဒသမ ငါး")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="my", ordinal=True), "တစ်မြောက်")
        self.assertEqual(num2words(2, lang="my", ordinal=True), "နှစ်မြောက်")
        self.assertEqual(num2words(3, lang="my", ordinal=True), "သုံးမြောက်")
        self.assertEqual(num2words(4, lang="my", ordinal=True), "လေးမြောက်")
        self.assertEqual(num2words(5, lang="my", ordinal=True), "ငါးမြောက်")
        self.assertEqual(num2words(6, lang="my", ordinal=True), "ခြောက်မြောက်")
        self.assertEqual(num2words(7, lang="my", ordinal=True), "ခုနစ်မြောက်")
        self.assertEqual(num2words(8, lang="my", ordinal=True), "ရှစ်မြောက်")
        self.assertEqual(num2words(9, lang="my", ordinal=True), "ကိုးမြောက်")
        self.assertEqual(num2words(10, lang="my", ordinal=True), "တစ်ဆယ်မြောက်")
        self.assertEqual(num2words(11, lang="my", ordinal=True), "တစ်ဆယ့်တစ်မြောက်")
        self.assertEqual(num2words(12, lang="my", ordinal=True), "တစ်ဆယ့်နှစ်မြောက်")
        self.assertEqual(num2words(13, lang="my", ordinal=True), "တစ်ဆယ့်သုံးမြောက်")
        self.assertEqual(num2words(14, lang="my", ordinal=True), "တစ်ဆယ့်လေးမြောက်")
        self.assertEqual(num2words(15, lang="my", ordinal=True), "တစ်ဆယ့်ငါးမြောက်")
        self.assertEqual(num2words(16, lang="my", ordinal=True), "တစ်ဆယ့်ခြောက်မြောက်")
        self.assertEqual(num2words(17, lang="my", ordinal=True), "တစ်ဆယ့်ခုနစ်မြောက်")
        self.assertEqual(num2words(18, lang="my", ordinal=True), "တစ်ဆယ့်ရှစ်မြောက်")
        self.assertEqual(num2words(19, lang="my", ordinal=True), "တစ်ဆယ့်ကိုးမြောက်")
        self.assertEqual(num2words(20, lang="my", ordinal=True), "နှစ်ဆယ်မြောက်")
        self.assertEqual(num2words(21, lang="my", ordinal=True), "နှစ်ဆယ့်တစ်မြောက်")
        self.assertEqual(num2words(22, lang="my", ordinal=True), "နှစ်ဆယ့်နှစ်မြောက်")
        self.assertEqual(num2words(25, lang="my", ordinal=True), "နှစ်ဆယ့်ငါးမြောက်")
        self.assertEqual(num2words(30, lang="my", ordinal=True), "သုံးဆယ်မြောက်")
        self.assertEqual(num2words(40, lang="my", ordinal=True), "လေးဆယ်မြောက်")
        self.assertEqual(num2words(50, lang="my", ordinal=True), "ငါးဆယ်မြောက်")
        self.assertEqual(num2words(60, lang="my", ordinal=True), "ခြောက်ဆယ်မြောက်")
        self.assertEqual(num2words(70, lang="my", ordinal=True), "ခုနစ်ဆယ်မြောက်")
        self.assertEqual(num2words(80, lang="my", ordinal=True), "ရှစ်ဆယ်မြောက်")
        self.assertEqual(num2words(90, lang="my", ordinal=True), "ကိုးဆယ်မြောက်")
        self.assertEqual(num2words(100, lang="my", ordinal=True), "တစ်ရာမြောက်")
        self.assertEqual(num2words(101, lang="my", ordinal=True), "တစ်ရာ့တစ်မြောက်")
        self.assertEqual(num2words(200, lang="my", ordinal=True), "နှစ်ရာမြောက်")
        self.assertEqual(num2words(500, lang="my", ordinal=True), "ငါးရာမြောက်")
        self.assertEqual(num2words(1000, lang="my", ordinal=True), "တစ်ထောင်မြောက်")
        self.assertEqual(num2words(1001, lang="my", ordinal=True), "တစ်ထောင့်တစ်မြောက်")
        self.assertEqual(num2words(10000, lang="my", ordinal=True), "တစ်သောင်းမြောက်")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="my", to="currency", currency="MMK"), "သုည ကျပ်"
        )
        self.assertEqual(
            num2words(0.01, lang="my", to="currency", currency="MMK"),
            "သုည ကျပ် တစ် ပြား",
        )
        self.assertEqual(
            num2words(0.5, lang="my", to="currency", currency="MMK"),
            "သုည ကျပ် ငါးဆယ် ပြား",
        )
        self.assertEqual(
            num2words(1, lang="my", to="currency", currency="MMK"), "တစ် ကျပ်"
        )
        self.assertEqual(
            num2words(1.5, lang="my", to="currency", currency="MMK"),
            "တစ် ကျပ် ငါးဆယ် ပြား",
        )
        self.assertEqual(
            num2words(0, lang="my", to="currency", currency="USD"), "သုည ဒေါ်လာ"
        )
        self.assertEqual(
            num2words(0.01, lang="my", to="currency", currency="USD"),
            "သုည ဒေါ်လာ တစ် ဆင့်",
        )
        self.assertEqual(
            num2words(0.5, lang="my", to="currency", currency="USD"),
            "သုည ဒေါ်လာ ငါးဆယ် ဆင့်",
        )
        self.assertEqual(
            num2words(1, lang="my", to="currency", currency="USD"), "တစ် ဒေါ်လာ"
        )
        self.assertEqual(
            num2words(1.5, lang="my", to="currency", currency="USD"),
            "တစ် ဒေါ်လာ ငါးဆယ် ဆင့်",
        )
        self.assertEqual(
            num2words(0, lang="my", to="currency", currency="EUR"), "သုည ယူရို"
        )
        self.assertEqual(
            num2words(0.01, lang="my", to="currency", currency="EUR"),
            "သုည ယူရို တစ် ဆင့်",
        )
        self.assertEqual(
            num2words(0.5, lang="my", to="currency", currency="EUR"),
            "သုည ယူရို ငါးဆယ် ဆင့်",
        )
        self.assertEqual(
            num2words(1, lang="my", to="currency", currency="EUR"), "တစ် ယူရို"
        )
        self.assertEqual(
            num2words(1.5, lang="my", to="currency", currency="EUR"),
            "တစ် ယူရို ငါးဆယ် ဆင့်",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="my", to="year"), "တစ်ထောင် ခုနှစ်")
        self.assertEqual(
            num2words(1066, lang="my", to="year"), "တစ်ထောင့်ခြောက်ဆယ့်ခြောက် ခုနှစ်"
        )
        self.assertEqual(
            num2words(1492, lang="my", to="year"), "တစ်ထောင့်လေးရာ့ကိုးဆယ့်နှစ် ခုနှစ်"
        )
        self.assertEqual(
            num2words(1776, lang="my", to="year"),
            "တစ်ထောင့်ခုနစ်ရာ့ခုနစ်ဆယ့်ခြောက် ခုနှစ်",
        )
        self.assertEqual(
            num2words(1800, lang="my", to="year"), "တစ်ထောင့်ရှစ်ရာ ခုနှစ်"
        )
        self.assertEqual(
            num2words(1900, lang="my", to="year"), "တစ်ထောင့်ကိုးရာ ခုနှစ်"
        )
        self.assertEqual(
            num2words(1984, lang="my", to="year"), "တစ်ထောင့်ကိုးရာ့ရှစ်ဆယ့်လေး ခုနှစ်"
        )
        self.assertEqual(
            num2words(1999, lang="my", to="year"), "တစ်ထောင့်ကိုးရာ့ကိုးဆယ့်ကိုး ခုနှစ်"
        )
        self.assertEqual(num2words(2000, lang="my", to="year"), "နှစ်ထောင် ခုနှစ်")
        self.assertEqual(num2words(2001, lang="my", to="year"), "နှစ်ထောင့်တစ် ခုနှစ်")
        self.assertEqual(
            num2words(2010, lang="my", to="year"), "နှစ်ထောင့်တစ်ဆယ် ခုနှစ်"
        )
        self.assertEqual(
            num2words(2020, lang="my", to="year"), "နှစ်ထောင့်နှစ်ဆယ် ခုနှစ်"
        )
        self.assertEqual(
            num2words(2024, lang="my", to="year"), "နှစ်ထောင့်နှစ်ဆယ့်လေး ခုနှစ်"
        )
        self.assertEqual(
            num2words(2100, lang="my", to="year"), "နှစ်ထောင့်တစ်ရာ ခုနှစ်"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="my"), "သုည")
        self.assertEqual(num2words("1", lang="my"), "တစ်")
        self.assertEqual(num2words("10", lang="my"), "တစ်ဆယ်")
        self.assertEqual(num2words("100", lang="my"), "တစ်ရာ")
        self.assertEqual(num2words("1000", lang="my"), "တစ်ထောင်")
        self.assertEqual(num2words("10000", lang="my"), "တစ်သောင်း")
        self.assertEqual(num2words("100000", lang="my"), "တစ်သိန်း")
        self.assertEqual(num2words("1000000", lang="my"), "တစ်သန်း")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="my"), "သုည")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="my"), num2words("100", lang="my"))
        self.assertEqual(num2words(1000, lang="my"), num2words("1000", lang="my"))

