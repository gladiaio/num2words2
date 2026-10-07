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

from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsHYTest(TestCase):
    """Test suite for Armenian number to words converter."""

    def test_basic_cardinal_numbers(self):
        """Test basic cardinal number conversion."""
        # Single digits
        self.assertEqual(num2words(0, lang="hy"), "զրո")
        self.assertEqual(num2words(1, lang="hy"), "մեկ")
        self.assertEqual(num2words(2, lang="hy"), "երկու")
        self.assertEqual(num2words(3, lang="hy"), "երեք")
        self.assertEqual(num2words(5, lang="hy"), "հինգ")
        self.assertEqual(num2words(9, lang="hy"), "ինը")

        # Teens and tens
        self.assertEqual(num2words(10, lang="hy"), "տասը")
        self.assertEqual(num2words(11, lang="hy"), "տասնմեկ")
        self.assertEqual(num2words(12, lang="hy"), "տասներկու")
        self.assertEqual(num2words(15, lang="hy"), "տասնհինգ")
        self.assertEqual(num2words(19, lang="hy"), "տասնինը")
        self.assertEqual(num2words(20, lang="hy"), "քսան")
        self.assertEqual(num2words(21, lang="hy"), "քսանմեկ")
        self.assertEqual(num2words(30, lang="hy"), "երեսուն")
        self.assertEqual(num2words(50, lang="hy"), "հիսուն")
        self.assertEqual(num2words(99, lang="hy"), "իննսուն ինը")

        # Hundreds
        self.assertEqual(num2words(100, lang="hy"), "հարյուր")
        self.assertEqual(num2words(101, lang="hy"), "հարյուր մեկ")
        self.assertEqual(num2words(111, lang="hy"), "հարյուր տասնմեկ")
        self.assertEqual(num2words(120, lang="hy"), "հարյուր քսան")
        self.assertEqual(num2words(200, lang="hy"), "երկու հարյուր")
        self.assertEqual(num2words(999, lang="hy"), "ինը հարյուր իննսուն ինը")

    def test_large_cardinal_numbers(self):
        """Test large cardinal number conversion."""
        # Thousands
        self.assertEqual(num2words(1000, lang="hy"), "հազար")
        self.assertEqual(num2words(1001, lang="hy"), "հազար մեկ")
        self.assertEqual(num2words(1111, lang="hy"), "հազար հարյուր տասնմեկ")
        self.assertEqual(num2words(2000, lang="hy"), "երկու հազար")
        self.assertEqual(num2words(10000, lang="hy"), "տասը հազար")
        self.assertEqual(num2words(100000, lang="hy"), "հարյուր հազար")

        # Millions and billions
        self.assertEqual(num2words(1000000, lang="hy"), "մեկ միլիոն")
        self.assertEqual(num2words(2000000, lang="hy"), "երկու միլիոն")
        self.assertEqual(num2words(1000000000, lang="hy"), "մեկ միլիարդ")
        self.assertEqual(num2words(4000000000, lang="hy"), "չորս միլիարդ")

        # Special cases for merge method
        self.assertEqual(num2words(142, lang="hy"), "հարյուր քառասուներկու")
        self.assertEqual(num2words(100042, lang="hy"), "հարյուր հազար քառասուներկու")

        # Specific scenarios in to_cardinal
        self.assertEqual(num2words(1100, lang="hy"), "հազար հարյուր")
        self.assertEqual(num2words(3000000, lang="hy"), "երեք միլիոն")
        self.assertEqual(num2words(3000000000, lang="hy"), "երեք միլիարդ")

    def test_ordinal_numbers(self):
        """Test ordinal number conversion."""
        self.assertEqual(num2words(0, lang="hy", to="ordinal"), "զրոերորդ")
        self.assertEqual(num2words(1, lang="hy", to="ordinal"), "առաջին")
        self.assertEqual(num2words(2, lang="hy", to="ordinal"), "երկրորդ")
        self.assertEqual(num2words(3, lang="hy", to="ordinal"), "երրորդ")
        self.assertEqual(num2words(4, lang="hy", to="ordinal"), "չորրորդ")
        self.assertEqual(num2words(5, lang="hy", to="ordinal"), "հինգերորդ")
        self.assertEqual(num2words(6, lang="hy", to="ordinal"), "վեցերորդ")
        self.assertEqual(num2words(7, lang="hy", to="ordinal"), "յոթերորդ")
        self.assertEqual(num2words(8, lang="hy", to="ordinal"), "ութերորդ")
        self.assertEqual(num2words(9, lang="hy", to="ordinal"), "իններորդ")
        self.assertEqual(num2words(10, lang="hy", to="ordinal"), "տասներորդ")
        self.assertEqual(num2words(11, lang="hy", to="ordinal"), "տասնմեկերորդ")
        self.assertEqual(num2words(12, lang="hy", to="ordinal"), "տասներկուերորդ")
        self.assertEqual(num2words(20, lang="hy", to="ordinal"), "քսաներորդ")
        self.assertEqual(num2words(21, lang="hy", to="ordinal"), "քսան առաջին")
        self.assertEqual(num2words(101, lang="hy", to="ordinal"), "հարյուր մեկերորդ")
        self.assertEqual(
            num2words(222, lang="hy", to="ordinal"),
            "երկու հարյուր քսաներկուերորդ",
        )

        # Large ordinal numbers
        self.assertEqual(num2words(1000, lang="hy", to="ordinal"), "հազարերորդ")
        self.assertEqual(num2words(1000000, lang="hy", to="ordinal"), "մեկ միլիոներորդ")

    def test_ordinal_with_suffix(self):
        """Test ordinal number with suffix conversion."""
        self.assertEqual(num2words(1, lang="hy", to="ordinal_num"), "1-րդ")
        self.assertEqual(num2words(2, lang="hy", to="ordinal_num"), "2-րդ")
        self.assertEqual(num2words(10, lang="hy", to="ordinal_num"), "10-րդ")
        self.assertEqual(num2words(21, lang="hy", to="ordinal_num"), "21-րդ")
        self.assertEqual(num2words(103, lang="hy", to="ordinal_num"), "103-րդ")
        self.assertEqual(num2words(1001, lang="hy", to="ordinal_num"), "1001-րդ")

    def test_basic_currency(self):
        """Test basic currency conversion."""
        # Basic currency tests
        self.assertEqual(
            num2words(1, lang="hy", to="currency", currency="AMD"),
            "մեկ դրամ",
        )
        self.assertEqual(
            num2words(2, lang="hy", to="currency", currency="AMD"),
            "երկու դրամ",
        )
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="EUR"),
            "հարյուր եվրո",
        )

        # Various currencies
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="AMD"),
            "հարյուր դրամ",
        )
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="RUB"),
            "հարյուր ռուբլի",
        )
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="JPY"),
            "հարյուր իեն",
        )
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="GBP"),
            "հարյուր ֆունտ ստեռլինգ",
        )
        self.assertEqual(
            num2words(100, lang="hy", to="currency", currency="CHF"),
            "հարյուր շվեյցարական ֆրանկ",
        )

    def test_billion_prefix_case(self):
        """Test special case for two billion conversion."""
        self.assertEqual(num2words(2000000000, lang="hy"), "երկու միլիարդ")

        self.assertEqual(num2words(3000000000, lang="hy"), "երեք միլիարդ")
        self.assertEqual(num2words(4000000000, lang="hy"), "չորս միլիարդ")

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="hy"), "մինուս զրո ամբողջ չորս")
        self.assertEqual(num2words(-0.5, lang="hy"), "մինուս զրո ամբողջ հինգ")
        self.assertEqual(num2words(-1.4, lang="hy"), "մինուս մեկ ամբողջ չորս")
