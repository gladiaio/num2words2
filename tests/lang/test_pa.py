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


class Num2WordsPATest(TestCase):
    """Comprehensive test cases for Punjabi language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="pa"), "ਸਿਫਰ")
        self.assertEqual(num2words(1, lang="pa"), "ਇੱਕ")
        self.assertEqual(num2words(2, lang="pa"), "ਦੋ")
        self.assertEqual(num2words(3, lang="pa"), "ਤਿੰਨ")
        self.assertEqual(num2words(4, lang="pa"), "ਚਾਰ")
        self.assertEqual(num2words(5, lang="pa"), "ਪੰਜ")
        self.assertEqual(num2words(6, lang="pa"), "ਛੇ")
        self.assertEqual(num2words(7, lang="pa"), "ਸੱਤ")
        self.assertEqual(num2words(8, lang="pa"), "ਅੱਠ")
        self.assertEqual(num2words(9, lang="pa"), "ਨੌ")
        self.assertEqual(num2words(10, lang="pa"), "ਦਸ")
        self.assertEqual(num2words(11, lang="pa"), "ਗਿਆਰਾਂ")
        self.assertEqual(num2words(12, lang="pa"), "ਬਾਰਾਂ")
        self.assertEqual(num2words(13, lang="pa"), "ਤੇਰਾਂ")
        self.assertEqual(num2words(14, lang="pa"), "ਚੌਦਾਂ")
        self.assertEqual(num2words(15, lang="pa"), "ਪੰਦਰਾਂ")
        self.assertEqual(num2words(16, lang="pa"), "ਸੋਲਾਂ")
        self.assertEqual(num2words(17, lang="pa"), "ਸਤਾਰਾਂ")
        self.assertEqual(num2words(18, lang="pa"), "ਅਠਾਰਾਂ")
        self.assertEqual(num2words(19, lang="pa"), "ਉੱਨੀ")
        self.assertEqual(num2words(20, lang="pa"), "ਵੀਹ")
        self.assertEqual(num2words(21, lang="pa"), "ਇੱਕੀ")
        self.assertEqual(num2words(22, lang="pa"), "ਬਾਈ")
        self.assertEqual(num2words(23, lang="pa"), "ਤੇਈ")
        self.assertEqual(num2words(24, lang="pa"), "ਚੌਵੀ")
        self.assertEqual(num2words(25, lang="pa"), "ਪੱਚੀ")
        self.assertEqual(num2words(26, lang="pa"), "ਛੱਬੀ")
        self.assertEqual(num2words(27, lang="pa"), "ਸਤਾਈ")
        self.assertEqual(num2words(28, lang="pa"), "ਅਠਾਈ")
        self.assertEqual(num2words(29, lang="pa"), "ਉਣੱਤੀ")
        self.assertEqual(num2words(30, lang="pa"), "ਤੀਹ")
        self.assertEqual(num2words(31, lang="pa"), "ਇਕੱਤੀ")
        self.assertEqual(num2words(35, lang="pa"), "ਪੈਂਤੀ")
        self.assertEqual(num2words(40, lang="pa"), "ਚਾਲੀ")
        self.assertEqual(num2words(45, lang="pa"), "ਪੰਤਾਲੀ")
        self.assertEqual(num2words(50, lang="pa"), "ਪੰਜਾਹ")
        self.assertEqual(num2words(55, lang="pa"), "ਪਚਵੰਜਾ")
        self.assertEqual(num2words(60, lang="pa"), "ਸੱਠ")
        self.assertEqual(num2words(65, lang="pa"), "ਪੈਂਹਠ")
        self.assertEqual(num2words(70, lang="pa"), "ਸੱਤਰ")
        self.assertEqual(num2words(75, lang="pa"), "ਪੰਝੱਤਰ")
        self.assertEqual(num2words(80, lang="pa"), "ਅੱਸੀ")
        self.assertEqual(num2words(85, lang="pa"), "ਪਚਾਸੀ")
        self.assertEqual(num2words(90, lang="pa"), "ਨੱਬੇ")
        self.assertEqual(num2words(95, lang="pa"), "ਪਚਾਨਵੇਂ")
        self.assertEqual(num2words(99, lang="pa"), "ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(100, lang="pa"), "ਇੱਕ ਸੌ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="pa"), "ਇੱਕ ਸੌ ਇੱਕ")
        self.assertEqual(num2words(110, lang="pa"), "ਇੱਕ ਸੌ ਦਸ")
        self.assertEqual(num2words(111, lang="pa"), "ਇੱਕ ਸੌ ਗਿਆਰਾਂ")
        self.assertEqual(num2words(120, lang="pa"), "ਇੱਕ ਸੌ ਵੀਹ")
        self.assertEqual(num2words(125, lang="pa"), "ਇੱਕ ਸੌ ਪੱਚੀ")
        self.assertEqual(num2words(150, lang="pa"), "ਇੱਕ ਸੌ ਪੰਜਾਹ")
        self.assertEqual(num2words(175, lang="pa"), "ਇੱਕ ਸੌ ਪੰਝੱਤਰ")
        self.assertEqual(num2words(199, lang="pa"), "ਇੱਕ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(200, lang="pa"), "ਦੋ ਸੌ")
        self.assertEqual(num2words(201, lang="pa"), "ਦੋ ਸੌ ਇੱਕ")
        self.assertEqual(num2words(210, lang="pa"), "ਦੋ ਸੌ ਦਸ")
        self.assertEqual(num2words(220, lang="pa"), "ਦੋ ਸੌ ਵੀਹ")
        self.assertEqual(num2words(250, lang="pa"), "ਦੋ ਸੌ ਪੰਜਾਹ")
        self.assertEqual(num2words(299, lang="pa"), "ਦੋ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(300, lang="pa"), "ਤਿੰਨ ਸੌ")
        self.assertEqual(num2words(333, lang="pa"), "ਤਿੰਨ ਸੌ ਤੇਤੀ")
        self.assertEqual(num2words(400, lang="pa"), "ਚਾਰ ਸੌ")
        self.assertEqual(num2words(444, lang="pa"), "ਚਾਰ ਸੌ ਚੁਤਾਲੀ")
        self.assertEqual(num2words(500, lang="pa"), "ਪੰਜ ਸੌ")
        self.assertEqual(num2words(555, lang="pa"), "ਪੰਜ ਸੌ ਪਚਵੰਜਾ")
        self.assertEqual(num2words(600, lang="pa"), "ਛੇ ਸੌ")
        self.assertEqual(num2words(666, lang="pa"), "ਛੇ ਸੌ ਛਿਆਹਠ")
        self.assertEqual(num2words(700, lang="pa"), "ਸੱਤ ਸੌ")
        self.assertEqual(num2words(777, lang="pa"), "ਸੱਤ ਸੌ ਸਤੱਤਰ")
        self.assertEqual(num2words(800, lang="pa"), "ਅੱਠ ਸੌ")
        self.assertEqual(num2words(888, lang="pa"), "ਅੱਠ ਸੌ ਅਠਾਸੀ")
        self.assertEqual(num2words(900, lang="pa"), "ਨੌ ਸੌ")
        self.assertEqual(num2words(999, lang="pa"), "ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(1001, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਇੱਕ")
        self.assertEqual(num2words(1010, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਦਸ")
        self.assertEqual(num2words(1100, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਇੱਕ ਸੌ")
        self.assertEqual(num2words(1111, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਇੱਕ ਸੌ ਗਿਆਰਾਂ")
        self.assertEqual(num2words(1234, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਦੋ ਸੌ ਚੌਂਤੀ")
        self.assertEqual(num2words(1500, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਪੰਜ ਸੌ")
        self.assertEqual(num2words(1999, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(2000, lang="pa"), "ਦੋ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(2001, lang="pa"), "ਦੋ ਹਜ਼ਾਰ ਇੱਕ")
        self.assertEqual(num2words(2020, lang="pa"), "ਦੋ ਹਜ਼ਾਰ ਵੀਹ")
        self.assertEqual(num2words(2222, lang="pa"), "ਦੋ ਹਜ਼ਾਰ ਦੋ ਸੌ ਬਾਈ")
        self.assertEqual(num2words(3000, lang="pa"), "ਤਿੰਨ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(3333, lang="pa"), "ਤਿੰਨ ਹਜ਼ਾਰ ਤਿੰਨ ਸੌ ਤੇਤੀ")
        self.assertEqual(num2words(4000, lang="pa"), "ਚਾਰ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(4444, lang="pa"), "ਚਾਰ ਹਜ਼ਾਰ ਚਾਰ ਸੌ ਚੁਤਾਲੀ")
        self.assertEqual(num2words(5000, lang="pa"), "ਪੰਜ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(5555, lang="pa"), "ਪੰਜ ਹਜ਼ਾਰ ਪੰਜ ਸੌ ਪਚਵੰਜਾ")
        self.assertEqual(num2words(6000, lang="pa"), "ਛੇ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(6666, lang="pa"), "ਛੇ ਹਜ਼ਾਰ ਛੇ ਸੌ ਛਿਆਹਠ")
        self.assertEqual(num2words(7000, lang="pa"), "ਸੱਤ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(7777, lang="pa"), "ਸੱਤ ਹਜ਼ਾਰ ਸੱਤ ਸੌ ਸਤੱਤਰ")
        self.assertEqual(num2words(8000, lang="pa"), "ਅੱਠ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(8888, lang="pa"), "ਅੱਠ ਹਜ਼ਾਰ ਅੱਠ ਸੌ ਅਠਾਸੀ")
        self.assertEqual(num2words(9000, lang="pa"), "ਨੌ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(9999, lang="pa"), "ਨੌ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(10000, lang="pa"), "ਦਸ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(10001, lang="pa"), "ਦਸ ਹਜ਼ਾਰ ਇੱਕ")
        self.assertEqual(num2words(11111, lang="pa"), "ਗਿਆਰਾਂ ਹਜ਼ਾਰ ਇੱਕ ਸੌ ਗਿਆਰਾਂ")
        self.assertEqual(num2words(12345, lang="pa"), "ਬਾਰਾਂ ਹਜ਼ਾਰ ਤਿੰਨ ਸੌ ਪੰਤਾਲੀ")
        self.assertEqual(num2words(20000, lang="pa"), "ਵੀਹ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(50000, lang="pa"), "ਪੰਜਾਹ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(99999, lang="pa"), "ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(100000, lang="pa"), "ਇੱਕ ਲੱਖ")
        self.assertEqual(num2words(123456, lang="pa"), "ਇੱਕ ਲੱਖ ਤੇਈ ਹਜ਼ਾਰ ਚਾਰ ਸੌ ਛਪੰਜਾ")
        self.assertEqual(num2words(200000, lang="pa"), "ਦੋ ਲੱਖ")
        self.assertEqual(num2words(500000, lang="pa"), "ਪੰਜ ਲੱਖ")
        self.assertEqual(
            num2words(654321, lang="pa"), "ਛੇ ਲੱਖ ਚੁਰੰਜਾ ਹਜ਼ਾਰ ਤਿੰਨ ਸੌ ਇੱਕੀ"
        )
        self.assertEqual(
            num2words(999999, lang="pa"), "ਨੌ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="pa"), "ਦਸ ਲੱਖ")
        self.assertEqual(num2words(1000001, lang="pa"), "ਦਸ ਲੱਖ ਇੱਕ")
        self.assertEqual(
            num2words(1111111, lang="pa"), "ਗਿਆਰਾਂ ਲੱਖ ਗਿਆਰਾਂ ਹਜ਼ਾਰ ਇੱਕ ਸੌ ਗਿਆਰਾਂ"
        )
        self.assertEqual(
            num2words(1234567, lang="pa"), "ਬਾਰਾਂ ਲੱਖ ਚੌਂਤੀ ਹਜ਼ਾਰ ਪੰਜ ਸੌ ਸਤਾਹਠ"
        )
        self.assertEqual(num2words(2000000, lang="pa"), "ਵੀਹ ਲੱਖ")
        self.assertEqual(num2words(5000000, lang="pa"), "ਪੰਜਾਹ ਲੱਖ")
        self.assertEqual(
            num2words(9999999, lang="pa"), "ਨੜਿੰਨਵੇਂ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ"
        )
        self.assertEqual(num2words(10000000, lang="pa"), "ਇੱਕ ਕਰੋੜ")
        self.assertEqual(
            num2words(12345678, lang="pa"),
            "ਇੱਕ ਕਰੋੜ ਤੇਈ ਲੱਖ ਪੰਤਾਲੀ ਹਜ਼ਾਰ ਛੇ ਸੌ ਅਠੱਤਰ",
        )
        self.assertEqual(
            num2words(99999999, lang="pa"),
            "ਨੌ ਕਰੋੜ ਨੜਿੰਨਵੇਂ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ",
        )
        self.assertEqual(num2words(100000000, lang="pa"), "ਦਸ ਕਰੋੜ")
        self.assertEqual(
            num2words(123456789, lang="pa"),
            "ਬਾਰਾਂ ਕਰੋੜ ਚੌਂਤੀ ਲੱਖ ਛਪੰਜਾ ਹਜ਼ਾਰ ਸੱਤ ਸੌ ਉਣਾਨਵੇਂ",
        )
        self.assertEqual(
            num2words(999999999, lang="pa"),
            "ਨੜਿੰਨਵੇਂ ਕਰੋੜ ਨੜਿੰਨਵੇਂ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ",
        )
        self.assertEqual(num2words(1000000000, lang="pa"), "ਇੱਕ ਅਰਬ")
        self.assertEqual(
            num2words(1234567890, lang="pa"),
            "ਇੱਕ ਅਰਬ ਤੇਈ ਕਰੋੜ ਪੰਤਾਲੀ ਲੱਖ ਸਤਾਹਠ ਹਜ਼ਾਰ ਅੱਠ ਸੌ ਨੱਬੇ",
        )
        self.assertEqual(
            num2words(9999999999, lang="pa"),
            "ਨੌ ਅਰਬ ਨੜਿੰਨਵੇਂ ਕਰੋੜ ਨੜਿੰਨਵੇਂ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ",
        )
        self.assertEqual(num2words(10000000000, lang="pa"), "ਦਸ ਅਰਬ")
        self.assertEqual(
            num2words(99999999999, lang="pa"),
            "ਨੜਿੰਨਵੇਂ ਅਰਬ ਨੜਿੰਨਵੇਂ ਕਰੋੜ ਨੜਿੰਨਵੇਂ ਲੱਖ ਨੜਿੰਨਵੇਂ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="pa"), "ਮਾਇਨਸ ਇੱਕ")
        self.assertEqual(num2words(-2, lang="pa"), "ਮਾਇਨਸ ਦੋ")
        self.assertEqual(num2words(-5, lang="pa"), "ਮਾਇਨਸ ਪੰਜ")
        self.assertEqual(num2words(-10, lang="pa"), "ਮਾਇਨਸ ਦਸ")
        self.assertEqual(num2words(-11, lang="pa"), "ਮਾਇਨਸ ਗਿਆਰਾਂ")
        self.assertEqual(num2words(-20, lang="pa"), "ਮਾਇਨਸ ਵੀਹ")
        self.assertEqual(num2words(-50, lang="pa"), "ਮਾਇਨਸ ਪੰਜਾਹ")
        self.assertEqual(num2words(-99, lang="pa"), "ਮਾਇਨਸ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(-100, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਸੌ")
        self.assertEqual(num2words(-101, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਸੌ ਇੱਕ")
        self.assertEqual(num2words(-200, lang="pa"), "ਮਾਇਨਸ ਦੋ ਸੌ")
        self.assertEqual(num2words(-999, lang="pa"), "ਮਾਇਨਸ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ")
        self.assertEqual(num2words(-1000, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(-1001, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਹਜ਼ਾਰ ਇੱਕ")
        self.assertEqual(num2words(-10000, lang="pa"), "ਮਾਇਨਸ ਦਸ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(-100000, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਲੱਖ")
        self.assertEqual(num2words(-1000000, lang="pa"), "ਮਾਇਨਸ ਦਸ ਲੱਖ")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="pa"), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਇੱਕ")
        self.assertEqual(num2words(0.5, lang="pa"), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(0.9, lang="pa"), "ਸਿਫਰ ਦਸ਼ਮਲਵ ਨੌ")
        self.assertEqual(num2words(1.1, lang="pa"), "ਇੱਕ ਦਸ਼ਮਲਵ ਇੱਕ")
        self.assertEqual(num2words(1.5, lang="pa"), "ਇੱਕ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(2.5, lang="pa"), "ਦੋ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(3.14, lang="pa"), "ਤਿੰਨ ਦਸ਼ਮਲਵ ਇੱਕ ਚਾਰ")
        self.assertEqual(num2words(10.5, lang="pa"), "ਦਸ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(11.11, lang="pa"), "ਗਿਆਰਾਂ ਦਸ਼ਮਲਵ ਇੱਕ ਇੱਕ")
        self.assertEqual(num2words(20.2, lang="pa"), "ਵੀਹ ਦਸ਼ਮਲਵ ਦੋ")
        self.assertEqual(num2words(99.99, lang="pa"), "ਨੜਿੰਨਵੇਂ ਦਸ਼ਮਲਵ ਨੌ ਨੌ")
        self.assertEqual(num2words(100.01, lang="pa"), "ਇੱਕ ਸੌ ਦਸ਼ਮਲਵ ਸਿਫਰ ਇੱਕ")
        self.assertEqual(num2words(100.5, lang="pa"), "ਇੱਕ ਸੌ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(123.45, lang="pa"), "ਇੱਕ ਸੌ ਤੇਈ ਦਸ਼ਮਲਵ ਚਾਰ ਪੰਜ")
        self.assertEqual(num2words(1000.5, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(
            num2words(1234.56, lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ ਦੋ ਸੌ ਚੌਂਤੀ ਦਸ਼ਮਲਵ ਪੰਜ ਛੇ"
        )
        self.assertEqual(num2words(10000.01, lang="pa"), "ਦਸ ਹਜ਼ਾਰ ਦਸ਼ਮਲਵ ਸਿਫਰ ਇੱਕ")
        self.assertEqual(num2words(-0.5, lang="pa"), "ਮਾਇਨਸ ਸਿਫਰ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(-1.5, lang="pa"), "ਮਾਇਨਸ ਇੱਕ ਦਸ਼ਮਲਵ ਪੰਜ")
        self.assertEqual(num2words(-10.5, lang="pa"), "ਮਾਇਨਸ ਦਸ ਦਸ਼ਮਲਵ ਪੰਜ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="pa", ordinal=True), "ਪਹਿਲਾ")
        self.assertEqual(num2words(2, lang="pa", ordinal=True), "ਦੂਜਾ")
        self.assertEqual(num2words(3, lang="pa", ordinal=True), "ਤੀਜਾ")
        self.assertEqual(num2words(4, lang="pa", ordinal=True), "ਚੌਥਾ")
        self.assertEqual(num2words(5, lang="pa", ordinal=True), "ਪੰਜਵਾਂ")
        self.assertEqual(num2words(6, lang="pa", ordinal=True), "ਛੇਵਾਂ")
        self.assertEqual(num2words(7, lang="pa", ordinal=True), "ਸੱਤਵਾਂ")
        self.assertEqual(num2words(8, lang="pa", ordinal=True), "ਅੱਠਵਾਂ")
        self.assertEqual(num2words(9, lang="pa", ordinal=True), "ਨੌਵਾਂ")
        self.assertEqual(num2words(10, lang="pa", ordinal=True), "ਦਸਵਾਂ")
        self.assertEqual(num2words(11, lang="pa", ordinal=True), "ਗਿਆਰਾਂਵਾਂ")
        self.assertEqual(num2words(12, lang="pa", ordinal=True), "ਬਾਰਾਂਵਾਂ")
        self.assertEqual(num2words(13, lang="pa", ordinal=True), "ਤੇਰਾਂਵਾਂ")
        self.assertEqual(num2words(14, lang="pa", ordinal=True), "ਚੌਦਾਂਵਾਂ")
        self.assertEqual(num2words(15, lang="pa", ordinal=True), "ਪੰਦਰਾਂਵਾਂ")
        self.assertEqual(num2words(16, lang="pa", ordinal=True), "ਸੋਲਾਂਵਾਂ")
        self.assertEqual(num2words(17, lang="pa", ordinal=True), "ਸਤਾਰਾਂਵਾਂ")
        self.assertEqual(num2words(18, lang="pa", ordinal=True), "ਅਠਾਰਾਂਵਾਂ")
        self.assertEqual(num2words(19, lang="pa", ordinal=True), "ਉੱਨੀਵਾਂ")
        self.assertEqual(num2words(20, lang="pa", ordinal=True), "ਵੀਹਵਾਂ")
        self.assertEqual(num2words(21, lang="pa", ordinal=True), "ਇੱਕੀਵਾਂ")
        self.assertEqual(num2words(22, lang="pa", ordinal=True), "ਬਾਈਵਾਂ")
        self.assertEqual(num2words(25, lang="pa", ordinal=True), "ਪੱਚੀਵਾਂ")
        self.assertEqual(num2words(30, lang="pa", ordinal=True), "ਤੀਹਵਾਂ")
        self.assertEqual(num2words(40, lang="pa", ordinal=True), "ਚਾਲੀਵਾਂ")
        self.assertEqual(num2words(50, lang="pa", ordinal=True), "ਪੰਜਾਹਵਾਂ")
        self.assertEqual(num2words(60, lang="pa", ordinal=True), "ਸੱਠਵਾਂ")
        self.assertEqual(num2words(70, lang="pa", ordinal=True), "ਸੱਤਰਵਾਂ")
        self.assertEqual(num2words(80, lang="pa", ordinal=True), "ਅੱਸੀਵਾਂ")
        self.assertEqual(num2words(90, lang="pa", ordinal=True), "ਨੱਬੇਵਾਂ")
        self.assertEqual(num2words(100, lang="pa", ordinal=True), "ਇੱਕ ਸੌਵਾਂ")
        self.assertEqual(num2words(101, lang="pa", ordinal=True), "ਇੱਕ ਸੌ ਇੱਕਵਾਂ")
        self.assertEqual(num2words(200, lang="pa", ordinal=True), "ਦੋ ਸੌਵਾਂ")
        self.assertEqual(num2words(500, lang="pa", ordinal=True), "ਪੰਜ ਸੌਵਾਂ")
        self.assertEqual(num2words(1000, lang="pa", ordinal=True), "ਇੱਕ ਹਜ਼ਾਰਵਾਂ")
        self.assertEqual(num2words(1001, lang="pa", ordinal=True), "ਇੱਕ ਹਜ਼ਾਰ ਇੱਕਵਾਂ")
        self.assertEqual(num2words(10000, lang="pa", ordinal=True), "ਦਸ ਹਜ਼ਾਰਵਾਂ")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="pa", to="currency", currency="INR"), "ਸਿਫਰ ਰੁਪਏ"
        )
        self.assertEqual(
            num2words(0.01, lang="pa", to="currency", currency="INR"),
            "ਸਿਫਰ ਰੁਪਏ ਇੱਕ ਪੈਸਾ",
        )
        self.assertEqual(
            num2words(0.5, lang="pa", to="currency", currency="INR"),
            "ਸਿਫਰ ਰੁਪਏ ਪੰਜਾਹ ਪੈਸੇ",
        )
        self.assertEqual(
            num2words(1, lang="pa", to="currency", currency="INR"), "ਇੱਕ ਰੁਪਈਆ"
        )
        self.assertEqual(
            num2words(1.5, lang="pa", to="currency", currency="INR"),
            "ਇੱਕ ਰੁਪਈਆ ਪੰਜਾਹ ਪੈਸੇ",
        )
        self.assertEqual(
            num2words(0, lang="pa", to="currency", currency="USD"), "ਸਿਫਰ ਡਾਲਰ"
        )
        self.assertEqual(
            num2words(0.01, lang="pa", to="currency", currency="USD"),
            "ਸਿਫਰ ਡਾਲਰ ਇੱਕ ਸੈਂਟ",
        )
        self.assertEqual(
            num2words(0.5, lang="pa", to="currency", currency="USD"),
            "ਸਿਫਰ ਡਾਲਰ ਪੰਜਾਹ ਸੈਂਟ",
        )
        self.assertEqual(
            num2words(1, lang="pa", to="currency", currency="USD"), "ਇੱਕ ਡਾਲਰ"
        )
        self.assertEqual(
            num2words(1.5, lang="pa", to="currency", currency="USD"),
            "ਇੱਕ ਡਾਲਰ ਪੰਜਾਹ ਸੈਂਟ",
        )
        self.assertEqual(
            num2words(0, lang="pa", to="currency", currency="EUR"), "ਸਿਫਰ ਯੂਰੋ"
        )
        self.assertEqual(
            num2words(0.01, lang="pa", to="currency", currency="EUR"),
            "ਸਿਫਰ ਯੂਰੋ ਇੱਕ ਸੈਂਟ",
        )
        self.assertEqual(
            num2words(0.5, lang="pa", to="currency", currency="EUR"),
            "ਸਿਫਰ ਯੂਰੋ ਪੰਜਾਹ ਸੈਂਟ",
        )
        self.assertEqual(
            num2words(1, lang="pa", to="currency", currency="EUR"), "ਇੱਕ ਯੂਰੋ"
        )
        self.assertEqual(
            num2words(1.5, lang="pa", to="currency", currency="EUR"),
            "ਇੱਕ ਯੂਰੋ ਪੰਜਾਹ ਸੈਂਟ",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(1066, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਛਿਆਹਠ")
        self.assertEqual(
            num2words(1492, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਚਾਰ ਸੌ ਬਾਨਵੇਂ"
        )
        self.assertEqual(
            num2words(1776, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਸੱਤ ਸੌ ਛਿਹੱਤਰ"
        )
        self.assertEqual(num2words(1800, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਅੱਠ ਸੌ")
        self.assertEqual(num2words(1900, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਨੌ ਸੌ")
        self.assertEqual(
            num2words(1984, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਨੌ ਸੌ ਚੁਰਾਸੀ"
        )
        self.assertEqual(
            num2words(1999, lang="pa", to="year"), "ਇੱਕ ਹਜ਼ਾਰ ਨੌ ਸੌ ਨੜਿੰਨਵੇਂ"
        )
        self.assertEqual(num2words(2000, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ")
        self.assertEqual(num2words(2001, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ ਇੱਕ")
        self.assertEqual(num2words(2010, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ ਦਸ")
        self.assertEqual(num2words(2020, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ ਵੀਹ")
        self.assertEqual(num2words(2024, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ ਚੌਵੀ")
        self.assertEqual(num2words(2100, lang="pa", to="year"), "ਦੋ ਹਜ਼ਾਰ ਇੱਕ ਸੌ")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="pa"), "ਸਿਫਰ")
        self.assertEqual(num2words("1", lang="pa"), "ਇੱਕ")
        self.assertEqual(num2words("10", lang="pa"), "ਦਸ")
        self.assertEqual(num2words("100", lang="pa"), "ਇੱਕ ਸੌ")
        self.assertEqual(num2words("1000", lang="pa"), "ਇੱਕ ਹਜ਼ਾਰ")
        self.assertEqual(num2words("10000", lang="pa"), "ਦਸ ਹਜ਼ਾਰ")
        self.assertEqual(num2words("100000", lang="pa"), "ਇੱਕ ਲੱਖ")
        self.assertEqual(num2words("1000000", lang="pa"), "ਦਸ ਲੱਖ")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="pa"), "ਸਿਫਰ")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="pa"), num2words("100", lang="pa"))
        self.assertEqual(num2words(1000, lang="pa"), num2words("1000", lang="pa"))
