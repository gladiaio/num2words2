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


class Num2WordsMLTest(TestCase):
    """Comprehensive test cases for Malayalam language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ml"), "പൂജ്യം")
        self.assertEqual(num2words(1, lang="ml"), "ഒന്ന്")
        self.assertEqual(num2words(2, lang="ml"), "രണ്ട്")
        self.assertEqual(num2words(3, lang="ml"), "മൂന്ന്")
        self.assertEqual(num2words(4, lang="ml"), "നാല്")
        self.assertEqual(num2words(5, lang="ml"), "അഞ്ച്")
        self.assertEqual(num2words(6, lang="ml"), "ആറ്")
        self.assertEqual(num2words(7, lang="ml"), "ഏഴ്")
        self.assertEqual(num2words(8, lang="ml"), "എട്ട്")
        self.assertEqual(num2words(9, lang="ml"), "ഒൻപത്")
        self.assertEqual(num2words(10, lang="ml"), "പത്ത്")
        self.assertEqual(num2words(11, lang="ml"), "പതിനൊന്ന്")
        self.assertEqual(num2words(12, lang="ml"), "പന്ത്രണ്ട്")
        self.assertEqual(num2words(13, lang="ml"), "പതിമൂന്ന്")
        self.assertEqual(num2words(14, lang="ml"), "പതിനാല്")
        self.assertEqual(num2words(15, lang="ml"), "പതിനഞ്ച്")
        self.assertEqual(num2words(16, lang="ml"), "പതിനാറ്")
        self.assertEqual(num2words(17, lang="ml"), "പതിനേഴ്")
        self.assertEqual(num2words(18, lang="ml"), "പതിനെട്ട്")
        self.assertEqual(num2words(19, lang="ml"), "പത്തൊൻപത്")
        self.assertEqual(num2words(20, lang="ml"), "ഇരുപത്")
        self.assertEqual(num2words(21, lang="ml"), "ഇരുപത്തിയൊന്ന്")
        self.assertEqual(num2words(22, lang="ml"), "ഇരുപത്തിരണ്ട്")
        self.assertEqual(num2words(23, lang="ml"), "ഇരുപത്തിമൂന്ന്")
        self.assertEqual(num2words(24, lang="ml"), "ഇരുപത്തിനാല്")
        self.assertEqual(num2words(25, lang="ml"), "ഇരുപത്തിയഞ്ച്")
        self.assertEqual(num2words(26, lang="ml"), "ഇരുപത്തിയാറ്")
        self.assertEqual(num2words(27, lang="ml"), "ഇരുപത്തിയേഴ്")
        self.assertEqual(num2words(28, lang="ml"), "ഇരുപത്തിയെട്ട്")
        self.assertEqual(num2words(29, lang="ml"), "ഇരുപത്തിയൊൻപത്")
        self.assertEqual(num2words(30, lang="ml"), "മുപ്പത്")
        self.assertEqual(num2words(31, lang="ml"), "മുപ്പത്തിയൊന്ന്")
        self.assertEqual(num2words(35, lang="ml"), "മുപ്പത്തിയഞ്ച്")
        self.assertEqual(num2words(40, lang="ml"), "നാല്പത്")
        self.assertEqual(num2words(45, lang="ml"), "നാല്പത്തിയഞ്ച്")
        self.assertEqual(num2words(50, lang="ml"), "അമ്പത്")
        self.assertEqual(num2words(55, lang="ml"), "അമ്പത്തിയഞ്ച്")
        self.assertEqual(num2words(60, lang="ml"), "അറുപത്")
        self.assertEqual(num2words(65, lang="ml"), "അറുപത്തിയഞ്ച്")
        self.assertEqual(num2words(70, lang="ml"), "എഴുപത്")
        self.assertEqual(num2words(75, lang="ml"), "എഴുപത്തിയഞ്ച്")
        self.assertEqual(num2words(80, lang="ml"), "എൺപത്")
        self.assertEqual(num2words(85, lang="ml"), "എൺപത്തിയഞ്ച്")
        self.assertEqual(num2words(90, lang="ml"), "തൊണ്ണൂറ്")
        self.assertEqual(num2words(95, lang="ml"), "തൊണ്ണൂറ്റിയഞ്ച്")
        self.assertEqual(num2words(99, lang="ml"), "തൊണ്ണൂറ്റിയൊൻപത്")
        self.assertEqual(num2words(100, lang="ml"), "ഒന്ന് നൂറ്")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ml"), "ഒന്ന് നൂറ് ഒന്ന്")
        self.assertEqual(num2words(110, lang="ml"), "ഒന്ന് നൂറ് പത്ത്")
        self.assertEqual(num2words(111, lang="ml"), "ഒന്ന് നൂറ് പതിനൊന്ന്")
        self.assertEqual(num2words(120, lang="ml"), "ഒന്ന് നൂറ് ഇരുപത്")
        self.assertEqual(num2words(125, lang="ml"), "ഒന്ന് നൂറ് ഇരുപത്തിയഞ്ച്")
        self.assertEqual(num2words(150, lang="ml"), "ഒന്ന് നൂറ് അമ്പത്")
        self.assertEqual(num2words(175, lang="ml"), "ഒന്ന് നൂറ് എഴുപത്തിയഞ്ച്")
        self.assertEqual(num2words(199, lang="ml"), "ഒന്ന് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്")
        self.assertEqual(num2words(200, lang="ml"), "രണ്ട് നൂറ്")
        self.assertEqual(num2words(201, lang="ml"), "രണ്ട് നൂറ് ഒന്ന്")
        self.assertEqual(num2words(210, lang="ml"), "രണ്ട് നൂറ് പത്ത്")
        self.assertEqual(num2words(220, lang="ml"), "രണ്ട് നൂറ് ഇരുപത്")
        self.assertEqual(num2words(250, lang="ml"), "രണ്ട് നൂറ് അമ്പത്")
        self.assertEqual(num2words(299, lang="ml"), "രണ്ട് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്")
        self.assertEqual(num2words(300, lang="ml"), "മൂന്ന് നൂറ്")
        self.assertEqual(num2words(333, lang="ml"), "മൂന്ന് നൂറ് മുപ്പത്തിമൂന്ന്")
        self.assertEqual(num2words(400, lang="ml"), "നാല് നൂറ്")
        self.assertEqual(num2words(444, lang="ml"), "നാല് നൂറ് നാല്പത്തിനാല്")
        self.assertEqual(num2words(500, lang="ml"), "അഞ്ച് നൂറ്")
        self.assertEqual(num2words(555, lang="ml"), "അഞ്ച് നൂറ് അമ്പത്തിയഞ്ച്")
        self.assertEqual(num2words(600, lang="ml"), "ആറ് നൂറ്")
        self.assertEqual(num2words(666, lang="ml"), "ആറ് നൂറ് അറുപത്തിയാറ്")
        self.assertEqual(num2words(700, lang="ml"), "ഏഴ് നൂറ്")
        self.assertEqual(num2words(777, lang="ml"), "ഏഴ് നൂറ് എഴുപത്തിയേഴ്")
        self.assertEqual(num2words(800, lang="ml"), "എട്ട് നൂറ്")
        self.assertEqual(num2words(888, lang="ml"), "എട്ട് നൂറ് എൺപത്തിയെട്ട്")
        self.assertEqual(num2words(900, lang="ml"), "ഒൻപത് നൂറ്")
        self.assertEqual(num2words(999, lang="ml"), "ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ml"), "ഒന്ന് ആയിരം")
        self.assertEqual(num2words(1001, lang="ml"), "ഒന്ന് ആയിരം ഒന്ന്")
        self.assertEqual(num2words(1010, lang="ml"), "ഒന്ന് ആയിരം പത്ത്")
        self.assertEqual(num2words(1100, lang="ml"), "ഒന്ന് ആയിരം ഒന്ന് നൂറ്")
        self.assertEqual(num2words(1111, lang="ml"), "ഒന്ന് ആയിരം ഒന്ന് നൂറ് പതിനൊന്ന്")
        self.assertEqual(
            num2words(1234, lang="ml"), "ഒന്ന് ആയിരം രണ്ട് നൂറ് മുപ്പത്തിനാല്"
        )
        self.assertEqual(num2words(1500, lang="ml"), "ഒന്ന് ആയിരം അഞ്ച് നൂറ്")
        self.assertEqual(
            num2words(1999, lang="ml"), "ഒന്ന് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്"
        )
        self.assertEqual(num2words(2000, lang="ml"), "രണ്ട് ആയിരം")
        self.assertEqual(num2words(2001, lang="ml"), "രണ്ട് ആയിരം ഒന്ന്")
        self.assertEqual(num2words(2020, lang="ml"), "രണ്ട് ആയിരം ഇരുപത്")
        self.assertEqual(
            num2words(2222, lang="ml"), "രണ്ട് ആയിരം രണ്ട് നൂറ് ഇരുപത്തിരണ്ട്"
        )
        self.assertEqual(num2words(3000, lang="ml"), "മൂന്ന് ആയിരം")
        self.assertEqual(
            num2words(3333, lang="ml"), "മൂന്ന് ആയിരം മൂന്ന് നൂറ് മുപ്പത്തിമൂന്ന്"
        )
        self.assertEqual(num2words(4000, lang="ml"), "നാല് ആയിരം")
        self.assertEqual(
            num2words(4444, lang="ml"), "നാല് ആയിരം നാല് നൂറ് നാല്പത്തിനാല്"
        )
        self.assertEqual(num2words(5000, lang="ml"), "അഞ്ച് ആയിരം")
        self.assertEqual(
            num2words(5555, lang="ml"), "അഞ്ച് ആയിരം അഞ്ച് നൂറ് അമ്പത്തിയഞ്ച്"
        )
        self.assertEqual(num2words(6000, lang="ml"), "ആറ് ആയിരം")
        self.assertEqual(num2words(6666, lang="ml"), "ആറ് ആയിരം ആറ് നൂറ് അറുപത്തിയാറ്")
        self.assertEqual(num2words(7000, lang="ml"), "ഏഴ് ആയിരം")
        self.assertEqual(num2words(7777, lang="ml"), "ഏഴ് ആയിരം ഏഴ് നൂറ് എഴുപത്തിയേഴ്")
        self.assertEqual(num2words(8000, lang="ml"), "എട്ട് ആയിരം")
        self.assertEqual(
            num2words(8888, lang="ml"), "എട്ട് ആയിരം എട്ട് നൂറ് എൺപത്തിയെട്ട്"
        )
        self.assertEqual(num2words(9000, lang="ml"), "ഒൻപത് ആയിരം")
        self.assertEqual(
            num2words(9999, lang="ml"), "ഒൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്"
        )
        self.assertEqual(num2words(10000, lang="ml"), "പത്ത് ആയിരം")
        self.assertEqual(num2words(10001, lang="ml"), "പത്ത് ആയിരം ഒന്ന്")
        self.assertEqual(
            num2words(11111, lang="ml"), "പതിനൊന്ന് ആയിരം ഒന്ന് നൂറ് പതിനൊന്ന്"
        )
        self.assertEqual(
            num2words(12345, lang="ml"), "പന്ത്രണ്ട് ആയിരം മൂന്ന് നൂറ് നാല്പത്തിയഞ്ച്"
        )
        self.assertEqual(num2words(20000, lang="ml"), "ഇരുപത് ആയിരം")
        self.assertEqual(num2words(50000, lang="ml"), "അമ്പത് ആയിരം")
        self.assertEqual(
            num2words(99999, lang="ml"),
            "തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(100000, lang="ml"), "ഒരു ലക്ഷം")
        self.assertEqual(
            num2words(123456, lang="ml"),
            "ഒരു ലക്ഷം ഇരുപത്തിമൂന്ന് ആയിരം നാല് നൂറ് അമ്പത്തിയാറ്",
        )
        self.assertEqual(num2words(200000, lang="ml"), "രണ്ട് ലക്ഷം")
        self.assertEqual(num2words(500000, lang="ml"), "അഞ്ച് ലക്ഷം")
        self.assertEqual(
            num2words(654321, lang="ml"),
            "ആറ് ലക്ഷം അമ്പത്തിനാല് ആയിരം മൂന്ന് നൂറ് ഇരുപത്തിയൊന്ന്",
        )
        self.assertEqual(
            num2words(999999, lang="ml"),
            "ഒൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ml"), "പത്ത് ലക്ഷം")
        self.assertEqual(num2words(1000001, lang="ml"), "പത്ത് ലക്ഷം ഒന്ന്")
        self.assertEqual(
            num2words(1111111, lang="ml"),
            "പതിനൊന്ന് ലക്ഷം പതിനൊന്ന് ആയിരം ഒന്ന് നൂറ് പതിനൊന്ന്",
        )
        self.assertEqual(
            num2words(1234567, lang="ml"),
            "പന്ത്രണ്ട് ലക്ഷം മുപ്പത്തിനാല് ആയിരം അഞ്ച് നൂറ് അറുപത്തിയേഴ്",
        )
        self.assertEqual(num2words(2000000, lang="ml"), "ഇരുപത് ലക്ഷം")
        self.assertEqual(num2words(5000000, lang="ml"), "അമ്പത് ലക്ഷം")
        self.assertEqual(
            num2words(9999999, lang="ml"),
            "തൊണ്ണൂറ്റിയൊൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(10000000, lang="ml"), "ഒരു കോടി")
        self.assertEqual(
            num2words(12345678, lang="ml"),
            "ഒരു കോടി ഇരുപത്തിമൂന്ന് ലക്ഷം നാല്പത്തിയഞ്ച് ആയിരം ആറ് നൂറ് എഴുപത്തിയെട്ട്",
        )
        self.assertEqual(
            num2words(99999999, lang="ml"),
            "ഒൻപത് കോടി തൊണ്ണൂറ്റിയൊൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(100000000, lang="ml"), "പത്ത് കോടി")
        self.assertEqual(
            num2words(123456789, lang="ml"),
            "പന്ത്രണ്ട് കോടി മുപ്പത്തിനാല് ലക്ഷം അമ്പത്തിയാറ് ആയിരം ഏഴ് നൂറ് എൺപത്തിയൊൻപത്",
        )
        self.assertEqual(
            num2words(999999999, lang="ml"),
            "തൊണ്ണൂറ്റിയൊൻപത് കോടി തൊണ്ണൂറ്റിയൊൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(1000000000, lang="ml"), "ഒന്ന് നൂറ് കോടി")
        self.assertEqual(
            num2words(1234567890, lang="ml"),
            "ഒന്ന് നൂറ് ഇരുപത്തിമൂന്ന് കോടി നാല്പത്തിയഞ്ച് ലക്ഷം അറുപത്തിയേഴ് ആയിരം എട്ട് നൂറ് തൊണ്ണൂറ്",
        )
        self.assertEqual(
            num2words(9999999999, lang="ml"),
            "ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത് കോടി തൊണ്ണൂറ്റിയൊൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(10000000000, lang="ml"), "ഒന്ന് ആയിരം കോടി")
        self.assertEqual(
            num2words(99999999999, lang="ml"),
            "ഒൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത് കോടി തൊണ്ണൂറ്റിയൊൻപത് ലക്ഷം തൊണ്ണൂറ്റിയൊൻപത് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ml"), "മൈനസ് ഒന്ന്")
        self.assertEqual(num2words(-2, lang="ml"), "മൈനസ് രണ്ട്")
        self.assertEqual(num2words(-5, lang="ml"), "മൈനസ് അഞ്ച്")
        self.assertEqual(num2words(-10, lang="ml"), "മൈനസ് പത്ത്")
        self.assertEqual(num2words(-11, lang="ml"), "മൈനസ് പതിനൊന്ന്")
        self.assertEqual(num2words(-20, lang="ml"), "മൈനസ് ഇരുപത്")
        self.assertEqual(num2words(-50, lang="ml"), "മൈനസ് അമ്പത്")
        self.assertEqual(num2words(-99, lang="ml"), "മൈനസ് തൊണ്ണൂറ്റിയൊൻപത്")
        self.assertEqual(num2words(-100, lang="ml"), "മൈനസ് ഒന്ന് നൂറ്")
        self.assertEqual(num2words(-101, lang="ml"), "മൈനസ് ഒന്ന് നൂറ് ഒന്ന്")
        self.assertEqual(num2words(-200, lang="ml"), "മൈനസ് രണ്ട് നൂറ്")
        self.assertEqual(
            num2words(-999, lang="ml"), "മൈനസ് ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്"
        )
        self.assertEqual(num2words(-1000, lang="ml"), "മൈനസ് ഒന്ന് ആയിരം")
        self.assertEqual(num2words(-1001, lang="ml"), "മൈനസ് ഒന്ന് ആയിരം ഒന്ന്")
        self.assertEqual(num2words(-10000, lang="ml"), "മൈനസ് പത്ത് ആയിരം")
        self.assertEqual(num2words(-100000, lang="ml"), "മൈനസ് ഒരു ലക്ഷം")
        self.assertEqual(num2words(-1000000, lang="ml"), "മൈനസ് പത്ത് ലക്ഷം")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ml"), "പൂജ്യം പോയിന്റ് ഒന്ന്")
        self.assertEqual(num2words(0.5, lang="ml"), "പൂജ്യം പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(0.9, lang="ml"), "പൂജ്യം പോയിന്റ് ഒൻപത്")
        self.assertEqual(num2words(1.1, lang="ml"), "ഒന്ന് പോയിന്റ് ഒന്ന്")
        self.assertEqual(num2words(1.5, lang="ml"), "ഒന്ന് പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(2.5, lang="ml"), "രണ്ട് പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(3.14, lang="ml"), "മൂന്ന് പോയിന്റ് ഒന്ന് നാല്")
        self.assertEqual(num2words(10.5, lang="ml"), "പത്ത് പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(11.11, lang="ml"), "പതിനൊന്ന് പോയിന്റ് ഒന്ന് ഒന്ന്")
        self.assertEqual(num2words(20.2, lang="ml"), "ഇരുപത് പോയിന്റ് രണ്ട്")
        self.assertEqual(
            num2words(99.99, lang="ml"), "തൊണ്ണൂറ്റിയൊൻപത് പോയിന്റ് ഒൻപത് ഒൻപത്"
        )
        self.assertEqual(
            num2words(100.01, lang="ml"), "ഒന്ന് നൂറ് പോയിന്റ് പൂജ്യം ഒന്ന്"
        )
        self.assertEqual(num2words(100.5, lang="ml"), "ഒന്ന് നൂറ് പോയിന്റ് അഞ്ച്")
        self.assertEqual(
            num2words(123.45, lang="ml"),
            "ഒന്ന് നൂറ് ഇരുപത്തിമൂന്ന് പോയിന്റ് നാല് അഞ്ച്",
        )
        self.assertEqual(num2words(1000.5, lang="ml"), "ഒന്ന് ആയിരം പോയിന്റ് അഞ്ച്")
        self.assertEqual(
            num2words(1234.56, lang="ml"),
            "ഒന്ന് ആയിരം രണ്ട് നൂറ് മുപ്പത്തിനാല് പോയിന്റ് അഞ്ച് ആറ്",
        )
        self.assertEqual(
            num2words(10000.01, lang="ml"), "പത്ത് ആയിരം പോയിന്റ് പൂജ്യം ഒന്ന്"
        )
        self.assertEqual(num2words(-0.5, lang="ml"), "മൈനസ് പൂജ്യം പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(-1.5, lang="ml"), "മൈനസ് ഒന്ന് പോയിന്റ് അഞ്ച്")
        self.assertEqual(num2words(-10.5, lang="ml"), "മൈനസ് പത്ത് പോയിന്റ് അഞ്ച്")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ml", ordinal=True), "ഒന്നാം")
        self.assertEqual(num2words(2, lang="ml", ordinal=True), "രണ്ടാം")
        self.assertEqual(num2words(3, lang="ml", ordinal=True), "മൂന്നാം")
        self.assertEqual(num2words(4, lang="ml", ordinal=True), "നാലാം")
        self.assertEqual(num2words(5, lang="ml", ordinal=True), "അഞ്ചാം")
        self.assertEqual(num2words(6, lang="ml", ordinal=True), "ആറ്ാം")
        self.assertEqual(num2words(7, lang="ml", ordinal=True), "ഏഴ്ാം")
        self.assertEqual(num2words(8, lang="ml", ordinal=True), "എട്ട്ാം")
        self.assertEqual(num2words(9, lang="ml", ordinal=True), "ഒൻപത്ാം")
        self.assertEqual(num2words(10, lang="ml", ordinal=True), "പത്ത്ാം")
        self.assertEqual(num2words(11, lang="ml", ordinal=True), "പതിനൊന്ന്ാം")
        self.assertEqual(num2words(12, lang="ml", ordinal=True), "പന്ത്രണ്ട്ാം")
        self.assertEqual(num2words(13, lang="ml", ordinal=True), "പതിമൂന്ന്ാം")
        self.assertEqual(num2words(14, lang="ml", ordinal=True), "പതിനാല്ാം")
        self.assertEqual(num2words(15, lang="ml", ordinal=True), "പതിനഞ്ച്ാം")
        self.assertEqual(num2words(16, lang="ml", ordinal=True), "പതിനാറ്ാം")
        self.assertEqual(num2words(17, lang="ml", ordinal=True), "പതിനേഴ്ാം")
        self.assertEqual(num2words(18, lang="ml", ordinal=True), "പതിനെട്ട്ാം")
        self.assertEqual(num2words(19, lang="ml", ordinal=True), "പത്തൊൻപത്ാം")
        self.assertEqual(num2words(20, lang="ml", ordinal=True), "ഇരുപത്ാം")
        self.assertEqual(num2words(21, lang="ml", ordinal=True), "ഇരുപത്തിയൊന്ന്ാം")
        self.assertEqual(num2words(22, lang="ml", ordinal=True), "ഇരുപത്തിരണ്ട്ാം")
        self.assertEqual(num2words(25, lang="ml", ordinal=True), "ഇരുപത്തിയഞ്ച്ാം")
        self.assertEqual(num2words(30, lang="ml", ordinal=True), "മുപ്പത്ാം")
        self.assertEqual(num2words(40, lang="ml", ordinal=True), "നാല്പത്ാം")
        self.assertEqual(num2words(50, lang="ml", ordinal=True), "അമ്പത്ാം")
        self.assertEqual(num2words(60, lang="ml", ordinal=True), "അറുപത്ാം")
        self.assertEqual(num2words(70, lang="ml", ordinal=True), "എഴുപത്ാം")
        self.assertEqual(num2words(80, lang="ml", ordinal=True), "എൺപത്ാം")
        self.assertEqual(num2words(90, lang="ml", ordinal=True), "തൊണ്ണൂറ്ാം")
        self.assertEqual(num2words(100, lang="ml", ordinal=True), "ഒന്ന് നൂറ്ാം")
        self.assertEqual(num2words(101, lang="ml", ordinal=True), "ഒന്ന് നൂറ് ഒന്ന്ാം")
        self.assertEqual(num2words(200, lang="ml", ordinal=True), "രണ്ട് നൂറ്ാം")
        self.assertEqual(num2words(500, lang="ml", ordinal=True), "അഞ്ച് നൂറ്ാം")
        self.assertEqual(num2words(1000, lang="ml", ordinal=True), "ഒന്ന് ആയിരംാം")
        self.assertEqual(
            num2words(1001, lang="ml", ordinal=True), "ഒന്ന് ആയിരം ഒന്ന്ാം"
        )
        self.assertEqual(num2words(10000, lang="ml", ordinal=True), "പത്ത് ആയിരംാം")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ml", to="currency", currency="INR"), "പൂജ്യം രൂപ"
        )
        self.assertEqual(
            num2words(0.01, lang="ml", to="currency", currency="INR"),
            "പൂജ്യം രൂപ ഒന്ന് പൈസ",
        )
        self.assertEqual(
            num2words(0.5, lang="ml", to="currency", currency="INR"),
            "പൂജ്യം രൂപ അമ്പത് പൈസ",
        )
        self.assertEqual(
            num2words(1, lang="ml", to="currency", currency="INR"), "ഒന്ന് രൂപ"
        )
        self.assertEqual(
            num2words(1.5, lang="ml", to="currency", currency="INR"),
            "ഒന്ന് രൂപ അമ്പത് പൈസ",
        )
        self.assertEqual(
            num2words(0, lang="ml", to="currency", currency="USD"), "പൂജ്യം ഡോളർ"
        )
        self.assertEqual(
            num2words(0.01, lang="ml", to="currency", currency="USD"),
            "പൂജ്യം ഡോളർ ഒന്ന് സെന്റ്",
        )
        self.assertEqual(
            num2words(0.5, lang="ml", to="currency", currency="USD"),
            "പൂജ്യം ഡോളർ അമ്പത് സെന്റ്",
        )
        self.assertEqual(
            num2words(1, lang="ml", to="currency", currency="USD"), "ഒന്ന് ഡോളർ"
        )
        self.assertEqual(
            num2words(1.5, lang="ml", to="currency", currency="USD"),
            "ഒന്ന് ഡോളർ അമ്പത് സെന്റ്",
        )
        self.assertEqual(
            num2words(0, lang="ml", to="currency", currency="EUR"), "പൂജ്യം യൂറോ"
        )
        self.assertEqual(
            num2words(0.01, lang="ml", to="currency", currency="EUR"),
            "പൂജ്യം യൂറോ ഒന്ന് സെന്റ്",
        )
        self.assertEqual(
            num2words(0.5, lang="ml", to="currency", currency="EUR"),
            "പൂജ്യം യൂറോ അമ്പത് സെന്റ്",
        )
        self.assertEqual(
            num2words(1, lang="ml", to="currency", currency="EUR"), "ഒന്ന് യൂറോ"
        )
        self.assertEqual(
            num2words(1.5, lang="ml", to="currency", currency="EUR"),
            "ഒന്ന് യൂറോ അമ്പത് സെന്റ്",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ml", to="year"), "ഒന്ന് ആയിരം")
        self.assertEqual(
            num2words(1066, lang="ml", to="year"), "ഒന്ന് ആയിരം അറുപത്തിയാറ്"
        )
        self.assertEqual(
            num2words(1492, lang="ml", to="year"),
            "ഒന്ന് ആയിരം നാല് നൂറ് തൊണ്ണൂറ്റിരണ്ട്",
        )
        self.assertEqual(
            num2words(1776, lang="ml", to="year"), "ഒന്ന് ആയിരം ഏഴ് നൂറ് എഴുപത്തിയാറ്"
        )
        self.assertEqual(
            num2words(1800, lang="ml", to="year"), "ഒന്ന് ആയിരം എട്ട് നൂറ്"
        )
        self.assertEqual(
            num2words(1900, lang="ml", to="year"), "ഒന്ന് ആയിരം ഒൻപത് നൂറ്"
        )
        self.assertEqual(
            num2words(1984, lang="ml", to="year"), "ഒന്ന് ആയിരം ഒൻപത് നൂറ് എൺപത്തിനാല്"
        )
        self.assertEqual(
            num2words(1999, lang="ml", to="year"),
            "ഒന്ന് ആയിരം ഒൻപത് നൂറ് തൊണ്ണൂറ്റിയൊൻപത്",
        )
        self.assertEqual(num2words(2000, lang="ml", to="year"), "രണ്ട് ആയിരം")
        self.assertEqual(num2words(2001, lang="ml", to="year"), "രണ്ട് ആയിരം ഒന്ന്")
        self.assertEqual(num2words(2010, lang="ml", to="year"), "രണ്ട് ആയിരം പത്ത്")
        self.assertEqual(num2words(2020, lang="ml", to="year"), "രണ്ട് ആയിരം ഇരുപത്")
        self.assertEqual(
            num2words(2024, lang="ml", to="year"), "രണ്ട് ആയിരം ഇരുപത്തിനാല്"
        )
        self.assertEqual(
            num2words(2100, lang="ml", to="year"), "രണ്ട് ആയിരം ഒന്ന് നൂറ്"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ml"), "പൂജ്യം")
        self.assertEqual(num2words("1", lang="ml"), "ഒന്ന്")
        self.assertEqual(num2words("10", lang="ml"), "പത്ത്")
        self.assertEqual(num2words("100", lang="ml"), "ഒന്ന് നൂറ്")
        self.assertEqual(num2words("1000", lang="ml"), "ഒന്ന് ആയിരം")
        self.assertEqual(num2words("10000", lang="ml"), "പത്ത് ആയിരം")
        self.assertEqual(num2words("100000", lang="ml"), "ഒരു ലക്ഷം")
        self.assertEqual(num2words("1000000", lang="ml"), "പത്ത് ലക്ഷം")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ml"), "പൂജ്യം")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ml"), num2words("100", lang="ml"))
        self.assertEqual(num2words(1000, lang="ml"), num2words("1000", lang="ml"))
