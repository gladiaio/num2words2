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


class Num2WordsKMTest(TestCase):
    """Comprehensive test cases for Khmer language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="km"), "សូន្យ")
        self.assertEqual(num2words(1, lang="km"), "មួយ")
        self.assertEqual(num2words(2, lang="km"), "ពីរ")
        self.assertEqual(num2words(3, lang="km"), "បី")
        self.assertEqual(num2words(4, lang="km"), "បួន")
        self.assertEqual(num2words(5, lang="km"), "ប្រាំ")
        self.assertEqual(num2words(6, lang="km"), "ប្រាំមួយ")
        self.assertEqual(num2words(7, lang="km"), "ប្រាំពីរ")
        self.assertEqual(num2words(8, lang="km"), "ប្រាំបី")
        self.assertEqual(num2words(9, lang="km"), "ប្រាំបួន")
        self.assertEqual(num2words(10, lang="km"), "ដប់")
        self.assertEqual(num2words(11, lang="km"), "ដប់មួយ")
        self.assertEqual(num2words(12, lang="km"), "ដប់ពីរ")
        self.assertEqual(num2words(13, lang="km"), "ដប់បី")
        self.assertEqual(num2words(14, lang="km"), "ដប់បួន")
        self.assertEqual(num2words(15, lang="km"), "ដប់ប្រាំ")
        self.assertEqual(num2words(16, lang="km"), "ដប់ប្រាំមួយ")
        self.assertEqual(num2words(17, lang="km"), "ដប់ប្រាំពីរ")
        self.assertEqual(num2words(18, lang="km"), "ដប់ប្រាំបី")
        self.assertEqual(num2words(19, lang="km"), "ដប់ប្រាំបួន")
        self.assertEqual(num2words(20, lang="km"), "ម្ភៃ")
        self.assertEqual(num2words(21, lang="km"), "ម្ភៃមួយ")
        self.assertEqual(num2words(22, lang="km"), "ម្ភៃពីរ")
        self.assertEqual(num2words(23, lang="km"), "ម្ភៃបី")
        self.assertEqual(num2words(24, lang="km"), "ម្ភៃបួន")
        self.assertEqual(num2words(25, lang="km"), "ម្ភៃប្រាំ")
        self.assertEqual(num2words(26, lang="km"), "ម្ភៃប្រាំមួយ")
        self.assertEqual(num2words(27, lang="km"), "ម្ភៃប្រាំពីរ")
        self.assertEqual(num2words(28, lang="km"), "ម្ភៃប្រាំបី")
        self.assertEqual(num2words(29, lang="km"), "ម្ភៃប្រាំបួន")
        self.assertEqual(num2words(30, lang="km"), "សាមសិប")
        self.assertEqual(num2words(31, lang="km"), "សាមសិបមួយ")
        self.assertEqual(num2words(35, lang="km"), "សាមសិបប្រាំ")
        self.assertEqual(num2words(40, lang="km"), "សែសិប")
        self.assertEqual(num2words(45, lang="km"), "សែសិបប្រាំ")
        self.assertEqual(num2words(50, lang="km"), "ហាសិប")
        self.assertEqual(num2words(55, lang="km"), "ហាសិបប្រាំ")
        self.assertEqual(num2words(60, lang="km"), "ហុកសិប")
        self.assertEqual(num2words(65, lang="km"), "ហុកសិបប្រាំ")
        self.assertEqual(num2words(70, lang="km"), "ចិតសិប")
        self.assertEqual(num2words(75, lang="km"), "ចិតសិបប្រាំ")
        self.assertEqual(num2words(80, lang="km"), "ប៉ែតសិប")
        self.assertEqual(num2words(85, lang="km"), "ប៉ែតសិបប្រាំ")
        self.assertEqual(num2words(90, lang="km"), "កៅសិប")
        self.assertEqual(num2words(95, lang="km"), "កៅសិបប្រាំ")
        self.assertEqual(num2words(99, lang="km"), "កៅសិបប្រាំបួន")
        self.assertEqual(num2words(100, lang="km"), "មួយរយ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="km"), "មួយរយ មួយ")
        self.assertEqual(num2words(110, lang="km"), "មួយរយ ដប់")
        self.assertEqual(num2words(111, lang="km"), "មួយរយ ដប់មួយ")
        self.assertEqual(num2words(120, lang="km"), "មួយរយ ម្ភៃ")
        self.assertEqual(num2words(125, lang="km"), "មួយរយ ម្ភៃប្រាំ")
        self.assertEqual(num2words(150, lang="km"), "មួយរយ ហាសិប")
        self.assertEqual(num2words(175, lang="km"), "មួយរយ ចិតសិបប្រាំ")
        self.assertEqual(num2words(199, lang="km"), "មួយរយ កៅសិបប្រាំបួន")
        self.assertEqual(num2words(200, lang="km"), "ពីររយ")
        self.assertEqual(num2words(201, lang="km"), "ពីររយ មួយ")
        self.assertEqual(num2words(210, lang="km"), "ពីររយ ដប់")
        self.assertEqual(num2words(220, lang="km"), "ពីររយ ម្ភៃ")
        self.assertEqual(num2words(250, lang="km"), "ពីររយ ហាសិប")
        self.assertEqual(num2words(299, lang="km"), "ពីររយ កៅសិបប្រាំបួន")
        self.assertEqual(num2words(300, lang="km"), "បីរយ")
        self.assertEqual(num2words(333, lang="km"), "បីរយ សាមសិបបី")
        self.assertEqual(num2words(400, lang="km"), "បួនរយ")
        self.assertEqual(num2words(444, lang="km"), "បួនរយ សែសិបបួន")
        self.assertEqual(num2words(500, lang="km"), "ប្រាំរយ")
        self.assertEqual(num2words(555, lang="km"), "ប្រាំរយ ហាសិបប្រាំ")
        self.assertEqual(num2words(600, lang="km"), "ប្រាំមួយរយ")
        self.assertEqual(num2words(666, lang="km"), "ប្រាំមួយរយ ហុកសិបប្រាំមួយ")
        self.assertEqual(num2words(700, lang="km"), "ប្រាំពីររយ")
        self.assertEqual(num2words(777, lang="km"), "ប្រាំពីររយ ចិតសិបប្រាំពីរ")
        self.assertEqual(num2words(800, lang="km"), "ប្រាំបីរយ")
        self.assertEqual(num2words(888, lang="km"), "ប្រាំបីរយ ប៉ែតសិបប្រាំបី")
        self.assertEqual(num2words(900, lang="km"), "ប្រាំបួនរយ")
        self.assertEqual(num2words(999, lang="km"), "ប្រាំបួនរយ កៅសិបប្រាំបួន")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="km"), "មួយពាន់")
        self.assertEqual(num2words(1001, lang="km"), "មួយពាន់ មួយ")
        self.assertEqual(num2words(1010, lang="km"), "មួយពាន់ ដប់")
        self.assertEqual(num2words(1100, lang="km"), "មួយពាន់ មួយរយ")
        self.assertEqual(num2words(1111, lang="km"), "មួយពាន់ មួយរយ ដប់មួយ")
        self.assertEqual(num2words(1234, lang="km"), "មួយពាន់ ពីររយ សាមសិបបួន")
        self.assertEqual(num2words(1500, lang="km"), "មួយពាន់ ប្រាំរយ")
        self.assertEqual(num2words(1999, lang="km"), "មួយពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន")
        self.assertEqual(num2words(2000, lang="km"), "ពីរពាន់")
        self.assertEqual(num2words(2001, lang="km"), "ពីរពាន់ មួយ")
        self.assertEqual(num2words(2020, lang="km"), "ពីរពាន់ ម្ភៃ")
        self.assertEqual(num2words(2222, lang="km"), "ពីរពាន់ ពីររយ ម្ភៃពីរ")
        self.assertEqual(num2words(3000, lang="km"), "បីពាន់")
        self.assertEqual(num2words(3333, lang="km"), "បីពាន់ បីរយ សាមសិបបី")
        self.assertEqual(num2words(4000, lang="km"), "បួនពាន់")
        self.assertEqual(num2words(4444, lang="km"), "បួនពាន់ បួនរយ សែសិបបួន")
        self.assertEqual(num2words(5000, lang="km"), "ប្រាំពាន់")
        self.assertEqual(num2words(5555, lang="km"), "ប្រាំពាន់ ប្រាំរយ ហាសិបប្រាំ")
        self.assertEqual(num2words(6000, lang="km"), "ប្រាំមួយពាន់")
        self.assertEqual(
            num2words(6666, lang="km"), "ប្រាំមួយពាន់ ប្រាំមួយរយ ហុកសិបប្រាំមួយ"
        )
        self.assertEqual(num2words(7000, lang="km"), "ប្រាំពីរពាន់")
        self.assertEqual(
            num2words(7777, lang="km"), "ប្រាំពីរពាន់ ប្រាំពីររយ ចិតសិបប្រាំពីរ"
        )
        self.assertEqual(num2words(8000, lang="km"), "ប្រាំបីពាន់")
        self.assertEqual(
            num2words(8888, lang="km"), "ប្រាំបីពាន់ ប្រាំបីរយ ប៉ែតសិបប្រាំបី"
        )
        self.assertEqual(num2words(9000, lang="km"), "ប្រាំបួនពាន់")
        self.assertEqual(
            num2words(9999, lang="km"), "ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន"
        )
        self.assertEqual(num2words(10000, lang="km"), "មួយម៉ឺន")
        self.assertEqual(num2words(10001, lang="km"), "មួយម៉ឺន មួយ")
        self.assertEqual(num2words(11111, lang="km"), "មួយម៉ឺន មួយពាន់ មួយរយ ដប់មួយ")
        self.assertEqual(num2words(12345, lang="km"), "មួយម៉ឺន ពីរពាន់ បីរយ សែសិបប្រាំ")
        self.assertEqual(num2words(20000, lang="km"), "ពីរម៉ឺន")
        self.assertEqual(num2words(50000, lang="km"), "ប្រាំម៉ឺន")
        self.assertEqual(
            num2words(99999, lang="km"),
            "ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        self.assertEqual(num2words(100000, lang="km"), "មួយសែន")
        self.assertEqual(
            num2words(123456, lang="km"), "មួយសែន ពីរម៉ឺន បីពាន់ បួនរយ ហាសិបប្រាំមួយ"
        )
        self.assertEqual(num2words(200000, lang="km"), "ពីរសែន")
        self.assertEqual(num2words(500000, lang="km"), "ប្រាំសែន")
        self.assertEqual(
            num2words(654321, lang="km"), "ប្រាំមួយសែន ប្រាំម៉ឺន បួនពាន់ បីរយ ម្ភៃមួយ"
        )
        self.assertEqual(
            num2words(999999, lang="km"),
            "ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="km"), "មួយ លាន")
        self.assertEqual(num2words(1000001, lang="km"), "មួយ លាន មួយ")
        self.assertEqual(
            num2words(1111111, lang="km"), "មួយ លាន មួយសែន មួយម៉ឺន មួយពាន់ មួយរយ ដប់មួយ"
        )
        self.assertEqual(
            num2words(1234567, lang="km"),
            "មួយ លាន ពីរសែន បីម៉ឺន បួនពាន់ ប្រាំរយ ហុកសិបប្រាំពីរ",
        )
        self.assertEqual(num2words(2000000, lang="km"), "ពីរ លាន")
        self.assertEqual(num2words(5000000, lang="km"), "ប្រាំ លាន")
        self.assertEqual(
            num2words(9999999, lang="km"),
            "ប្រាំបួន លាន ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        self.assertEqual(num2words(10000000, lang="km"), "ដប់ លាន")
        self.assertEqual(
            num2words(12345678, lang="km"),
            "ដប់ពីរ លាន បីសែន បួនម៉ឺន ប្រាំពាន់ ប្រាំមួយរយ ចិតសិបប្រាំបី",
        )
        self.assertEqual(
            num2words(99999999, lang="km"),
            "កៅសិបប្រាំបួន លាន ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        self.assertEqual(num2words(100000000, lang="km"), "មួយរយ លាន")
        self.assertEqual(
            num2words(123456789, lang="km"),
            "មួយរយ ម្ភៃបី លាន បួនសែន ប្រាំម៉ឺន ប្រាំមួយពាន់ ប្រាំពីររយ ប៉ែតសិបប្រាំបួន",
        )
        self.assertEqual(
            num2words(999999999, lang="km"),
            "ប្រាំបួនរយ កៅសិបប្រាំបួន លាន ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="km"),
            "មួយពាន់ លាន",
        )
        self.assertEqual(
            num2words(1234567890, lang="km"),
            "មួយពាន់ ពីររយ សាមសិបបួន លាន ប្រាំសែន ប្រាំមួយម៉ឺន ប្រាំពីរពាន់ ប្រាំបីរយ កៅសិប",
        )
        self.assertEqual(
            num2words(9999999999, lang="km"),
            "ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន លាន ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        self.assertEqual(
            num2words(10000000000, lang="km"),
            "មួយម៉ឺន លាន",
        )
        self.assertEqual(
            num2words(99999999999, lang="km"),
            "ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន លាន ប្រាំបួនសែន ប្រាំបួនម៉ឺន ប្រាំបួនពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="km"), "ដក មួយ")
        self.assertEqual(num2words(-2, lang="km"), "ដក ពីរ")
        self.assertEqual(num2words(-5, lang="km"), "ដក ប្រាំ")
        self.assertEqual(num2words(-10, lang="km"), "ដក ដប់")
        self.assertEqual(num2words(-11, lang="km"), "ដក ដប់មួយ")
        self.assertEqual(num2words(-20, lang="km"), "ដក ម្ភៃ")
        self.assertEqual(num2words(-50, lang="km"), "ដក ហាសិប")
        self.assertEqual(num2words(-99, lang="km"), "ដក កៅសិបប្រាំបួន")
        self.assertEqual(num2words(-100, lang="km"), "ដក មួយរយ")
        self.assertEqual(num2words(-101, lang="km"), "ដក មួយរយ មួយ")
        self.assertEqual(num2words(-200, lang="km"), "ដក ពីររយ")
        self.assertEqual(num2words(-999, lang="km"), "ដក ប្រាំបួនរយ កៅសិបប្រាំបួន")
        self.assertEqual(num2words(-1000, lang="km"), "ដក មួយពាន់")
        self.assertEqual(num2words(-1001, lang="km"), "ដក មួយពាន់ មួយ")
        self.assertEqual(num2words(-10000, lang="km"), "ដក មួយម៉ឺន")
        self.assertEqual(num2words(-100000, lang="km"), "ដក មួយសែន")
        self.assertEqual(num2words(-1000000, lang="km"), "ដក មួយ លាន")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="km"), "សូន្យ ចំណុច មួយ")
        self.assertEqual(num2words(0.5, lang="km"), "សូន្យ ចំណុច ប្រាំ")
        self.assertEqual(num2words(0.9, lang="km"), "សូន្យ ចំណុច ប្រាំបួន")
        self.assertEqual(num2words(1.1, lang="km"), "មួយ ចំណុច មួយ")
        self.assertEqual(num2words(1.5, lang="km"), "មួយ ចំណុច ប្រាំ")
        self.assertEqual(num2words(2.5, lang="km"), "ពីរ ចំណុច ប្រាំ")
        self.assertEqual(num2words(3.14, lang="km"), "បី ចំណុច មួយ បួន")
        self.assertEqual(num2words(10.5, lang="km"), "ដប់ ចំណុច ប្រាំ")
        self.assertEqual(num2words(11.11, lang="km"), "ដប់មួយ ចំណុច មួយ មួយ")
        self.assertEqual(num2words(20.2, lang="km"), "ម្ភៃ ចំណុច ពីរ")
        self.assertEqual(
            num2words(99.99, lang="km"), "កៅសិបប្រាំបួន ចំណុច ប្រាំបួន ប្រាំបួន"
        )
        self.assertEqual(num2words(100.01, lang="km"), "មួយរយ ចំណុច សូន្យ មួយ")
        self.assertEqual(num2words(100.5, lang="km"), "មួយរយ ចំណុច ប្រាំ")
        self.assertEqual(num2words(123.45, lang="km"), "មួយរយ ម្ភៃបី ចំណុច បួន ប្រាំ")
        self.assertEqual(num2words(1000.5, lang="km"), "មួយពាន់ ចំណុច ប្រាំ")
        self.assertEqual(
            num2words(1234.56, lang="km"),
            "មួយពាន់ ពីររយ សាមសិបបួន ចំណុច ប្រាំ ប្រាំមួយ",
        )
        self.assertEqual(num2words(10000.01, lang="km"), "មួយម៉ឺន ចំណុច សូន្យ មួយ")
        self.assertEqual(num2words(-0.5, lang="km"), "ដក សូន្យ ចំណុច ប្រាំ")
        self.assertEqual(num2words(-1.5, lang="km"), "ដក មួយ ចំណុច ប្រាំ")
        self.assertEqual(num2words(-10.5, lang="km"), "ដក ដប់ ចំណុច ប្រាំ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="km", ordinal=True), "ទីមួយ")
        self.assertEqual(num2words(2, lang="km", ordinal=True), "ទីពីរ")
        self.assertEqual(num2words(3, lang="km", ordinal=True), "ទីបី")
        self.assertEqual(num2words(4, lang="km", ordinal=True), "ទីបួន")
        self.assertEqual(num2words(5, lang="km", ordinal=True), "ទីប្រាំ")
        self.assertEqual(num2words(6, lang="km", ordinal=True), "ទីប្រាំមួយ")
        self.assertEqual(num2words(7, lang="km", ordinal=True), "ទីប្រាំពីរ")
        self.assertEqual(num2words(8, lang="km", ordinal=True), "ទីប្រាំបី")
        self.assertEqual(num2words(9, lang="km", ordinal=True), "ទីប្រាំបួន")
        self.assertEqual(num2words(10, lang="km", ordinal=True), "ទីដប់")
        self.assertEqual(num2words(11, lang="km", ordinal=True), "ទីដប់មួយ")
        self.assertEqual(num2words(12, lang="km", ordinal=True), "ទីដប់ពីរ")
        self.assertEqual(num2words(13, lang="km", ordinal=True), "ទីដប់បី")
        self.assertEqual(num2words(14, lang="km", ordinal=True), "ទីដប់បួន")
        self.assertEqual(num2words(15, lang="km", ordinal=True), "ទីដប់ប្រាំ")
        self.assertEqual(num2words(16, lang="km", ordinal=True), "ទីដប់ប្រាំមួយ")
        self.assertEqual(num2words(17, lang="km", ordinal=True), "ទីដប់ប្រាំពីរ")
        self.assertEqual(num2words(18, lang="km", ordinal=True), "ទីដប់ប្រាំបី")
        self.assertEqual(num2words(19, lang="km", ordinal=True), "ទីដប់ប្រាំបួន")
        self.assertEqual(num2words(20, lang="km", ordinal=True), "ទីម្ភៃ")
        self.assertEqual(num2words(21, lang="km", ordinal=True), "ទីម្ភៃមួយ")
        self.assertEqual(num2words(22, lang="km", ordinal=True), "ទីម្ភៃពីរ")
        self.assertEqual(num2words(25, lang="km", ordinal=True), "ទីម្ភៃប្រាំ")
        self.assertEqual(num2words(30, lang="km", ordinal=True), "ទីសាមសិប")
        self.assertEqual(num2words(40, lang="km", ordinal=True), "ទីសែសិប")
        self.assertEqual(num2words(50, lang="km", ordinal=True), "ទីហាសិប")
        self.assertEqual(num2words(60, lang="km", ordinal=True), "ទីហុកសិប")
        self.assertEqual(num2words(70, lang="km", ordinal=True), "ទីចិតសិប")
        self.assertEqual(num2words(80, lang="km", ordinal=True), "ទីប៉ែតសិប")
        self.assertEqual(num2words(90, lang="km", ordinal=True), "ទីកៅសិប")
        self.assertEqual(num2words(100, lang="km", ordinal=True), "ទីមួយរយ")
        self.assertEqual(num2words(101, lang="km", ordinal=True), "ទីមួយរយ មួយ")
        self.assertEqual(num2words(200, lang="km", ordinal=True), "ទីពីររយ")
        self.assertEqual(num2words(500, lang="km", ordinal=True), "ទីប្រាំរយ")
        self.assertEqual(num2words(1000, lang="km", ordinal=True), "ទីមួយពាន់")
        self.assertEqual(num2words(1001, lang="km", ordinal=True), "ទីមួយពាន់ មួយ")
        self.assertEqual(num2words(10000, lang="km", ordinal=True), "ទីមួយម៉ឺន")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="km", to="currency", currency="KHR"), "សូន្យ រៀល"
        )
        self.assertEqual(
            num2words(0.01, lang="km", to="currency", currency="KHR"),
            "សូន្យ រៀល មួយ សេន",
        )
        self.assertEqual(
            num2words(0.5, lang="km", to="currency", currency="KHR"),
            "សូន្យ រៀល ហាសិប សេន",
        )
        self.assertEqual(
            num2words(1, lang="km", to="currency", currency="KHR"), "មួយ រៀល"
        )
        self.assertEqual(
            num2words(1.5, lang="km", to="currency", currency="KHR"),
            "មួយ រៀល ហាសិប សេន",
        )
        self.assertEqual(
            num2words(0, lang="km", to="currency", currency="USD"), "សូន្យ ដុល្លារ"
        )
        self.assertEqual(
            num2words(0.01, lang="km", to="currency", currency="USD"),
            "សូន្យ ដុល្លារ មួយ សេន",
        )
        self.assertEqual(
            num2words(0.5, lang="km", to="currency", currency="USD"),
            "សូន្យ ដុល្លារ ហាសិប សេន",
        )
        self.assertEqual(
            num2words(1, lang="km", to="currency", currency="USD"), "មួយ ដុល្លារ"
        )
        self.assertEqual(
            num2words(1.5, lang="km", to="currency", currency="USD"),
            "មួយ ដុល្លារ ហាសិប សេន",
        )
        self.assertEqual(
            num2words(0, lang="km", to="currency", currency="EUR"), "សូន្យ អឺរ៉ូ"
        )
        self.assertEqual(
            num2words(0.01, lang="km", to="currency", currency="EUR"),
            "សូន្យ អឺរ៉ូ មួយ សេន",
        )
        self.assertEqual(
            num2words(0.5, lang="km", to="currency", currency="EUR"),
            "សូន្យ អឺរ៉ូ ហាសិប សេន",
        )
        self.assertEqual(
            num2words(1, lang="km", to="currency", currency="EUR"), "មួយ អឺរ៉ូ"
        )
        self.assertEqual(
            num2words(1.5, lang="km", to="currency", currency="EUR"),
            "មួយ អឺរ៉ូ ហាសិប សេន",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="km", to="year"), "ឆ្នាំ មួយពាន់")
        self.assertEqual(
            num2words(1066, lang="km", to="year"), "ឆ្នាំ មួយពាន់ ហុកសិបប្រាំមួយ"
        )
        self.assertEqual(
            num2words(1492, lang="km", to="year"), "ឆ្នាំ មួយពាន់ បួនរយ កៅសិបពីរ"
        )
        self.assertEqual(
            num2words(1776, lang="km", to="year"),
            "ឆ្នាំ មួយពាន់ ប្រាំពីររយ ចិតសិបប្រាំមួយ",
        )
        self.assertEqual(
            num2words(1800, lang="km", to="year"), "ឆ្នាំ មួយពាន់ ប្រាំបីរយ"
        )
        self.assertEqual(
            num2words(1900, lang="km", to="year"), "ឆ្នាំ មួយពាន់ ប្រាំបួនរយ"
        )
        self.assertEqual(
            num2words(1984, lang="km", to="year"), "ឆ្នាំ មួយពាន់ ប្រាំបួនរយ ប៉ែតសិបបួន"
        )
        self.assertEqual(
            num2words(1999, lang="km", to="year"),
            "ឆ្នាំ មួយពាន់ ប្រាំបួនរយ កៅសិបប្រាំបួន",
        )
        self.assertEqual(num2words(2000, lang="km", to="year"), "ឆ្នាំ ពីរពាន់")
        self.assertEqual(num2words(2001, lang="km", to="year"), "ឆ្នាំ ពីរពាន់ មួយ")
        self.assertEqual(num2words(2010, lang="km", to="year"), "ឆ្នាំ ពីរពាន់ ដប់")
        self.assertEqual(num2words(2020, lang="km", to="year"), "ឆ្នាំ ពីរពាន់ ម្ភៃ")
        self.assertEqual(num2words(2024, lang="km", to="year"), "ឆ្នាំ ពីរពាន់ ម្ភៃបួន")
        self.assertEqual(num2words(2100, lang="km", to="year"), "ឆ្នាំ ពីរពាន់ មួយរយ")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="km"), "សូន្យ")
        self.assertEqual(num2words("1", lang="km"), "មួយ")
        self.assertEqual(num2words("10", lang="km"), "ដប់")
        self.assertEqual(num2words("100", lang="km"), "មួយរយ")
        self.assertEqual(num2words("1000", lang="km"), "មួយពាន់")
        self.assertEqual(num2words("10000", lang="km"), "មួយម៉ឺន")
        self.assertEqual(num2words("100000", lang="km"), "មួយសែន")
        self.assertEqual(num2words("1000000", lang="km"), "មួយ លាន")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="km"), "សូន្យ")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="km"), num2words("100", lang="km"))
        self.assertEqual(num2words(1000, lang="km"), num2words("1000", lang="km"))
