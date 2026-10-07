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


class Num2WordsLOTest(TestCase):
    """Comprehensive test cases for Lao language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="lo"), "ສູນ")
        self.assertEqual(num2words(1, lang="lo"), "ໜຶ່ງ")
        self.assertEqual(num2words(2, lang="lo"), "ສອງ")
        self.assertEqual(num2words(3, lang="lo"), "ສາມ")
        self.assertEqual(num2words(4, lang="lo"), "ສີ່")
        self.assertEqual(num2words(5, lang="lo"), "ຫ້າ")
        self.assertEqual(num2words(6, lang="lo"), "ຫົກ")
        self.assertEqual(num2words(7, lang="lo"), "ເຈັດ")
        self.assertEqual(num2words(8, lang="lo"), "ແປດ")
        self.assertEqual(num2words(9, lang="lo"), "ເກົ້າ")
        self.assertEqual(num2words(10, lang="lo"), "ສິບ")
        self.assertEqual(num2words(11, lang="lo"), "ສິບໜຶ່ງ")
        self.assertEqual(num2words(12, lang="lo"), "ສິບສອງ")
        self.assertEqual(num2words(13, lang="lo"), "ສິບສາມ")
        self.assertEqual(num2words(14, lang="lo"), "ສິບສີ່")
        self.assertEqual(num2words(15, lang="lo"), "ສິບຫ້າ")
        self.assertEqual(num2words(16, lang="lo"), "ສິບຫົກ")
        self.assertEqual(num2words(17, lang="lo"), "ສິບເຈັດ")
        self.assertEqual(num2words(18, lang="lo"), "ສິບແປດ")
        self.assertEqual(num2words(19, lang="lo"), "ສິບເກົ້າ")
        self.assertEqual(num2words(20, lang="lo"), "ຊາວ")
        self.assertEqual(num2words(21, lang="lo"), "ຊາວໜຶ່ງ")
        self.assertEqual(num2words(22, lang="lo"), "ຊາວສອງ")
        self.assertEqual(num2words(23, lang="lo"), "ຊາວສາມ")
        self.assertEqual(num2words(24, lang="lo"), "ຊາວສີ່")
        self.assertEqual(num2words(25, lang="lo"), "ຊາວຫ້າ")
        self.assertEqual(num2words(26, lang="lo"), "ຊາວຫົກ")
        self.assertEqual(num2words(27, lang="lo"), "ຊາວເຈັດ")
        self.assertEqual(num2words(28, lang="lo"), "ຊາວແປດ")
        self.assertEqual(num2words(29, lang="lo"), "ຊາວເກົ້າ")
        self.assertEqual(num2words(30, lang="lo"), "ສາມສິບ")
        self.assertEqual(num2words(31, lang="lo"), "ສາມສິບໜຶ່ງ")
        self.assertEqual(num2words(35, lang="lo"), "ສາມສິບຫ້າ")
        self.assertEqual(num2words(40, lang="lo"), "ສີ່ສິບ")
        self.assertEqual(num2words(45, lang="lo"), "ສີ່ສິບຫ້າ")
        self.assertEqual(num2words(50, lang="lo"), "ຫ້າສິບ")
        self.assertEqual(num2words(55, lang="lo"), "ຫ້າສິບຫ້າ")
        self.assertEqual(num2words(60, lang="lo"), "ຫົກສິບ")
        self.assertEqual(num2words(65, lang="lo"), "ຫົກສິບຫ້າ")
        self.assertEqual(num2words(70, lang="lo"), "ເຈັດສິບ")
        self.assertEqual(num2words(75, lang="lo"), "ເຈັດສິບຫ້າ")
        self.assertEqual(num2words(80, lang="lo"), "ແປດສິບ")
        self.assertEqual(num2words(85, lang="lo"), "ແປດສິບຫ້າ")
        self.assertEqual(num2words(90, lang="lo"), "ເກົ້າສິບ")
        self.assertEqual(num2words(95, lang="lo"), "ເກົ້າສິບຫ້າ")
        self.assertEqual(num2words(99, lang="lo"), "ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(100, lang="lo"), "ໜຶ່ງຮ້ອຍ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="lo"), "ໜຶ່ງຮ້ອຍ ໜຶ່ງ")
        self.assertEqual(num2words(110, lang="lo"), "ໜຶ່ງຮ້ອຍ ສິບ")
        self.assertEqual(num2words(111, lang="lo"), "ໜຶ່ງຮ້ອຍ ສິບໜຶ່ງ")
        self.assertEqual(num2words(120, lang="lo"), "ໜຶ່ງຮ້ອຍ ຊາວ")
        self.assertEqual(num2words(125, lang="lo"), "ໜຶ່ງຮ້ອຍ ຊາວຫ້າ")
        self.assertEqual(num2words(150, lang="lo"), "ໜຶ່ງຮ້ອຍ ຫ້າສິບ")
        self.assertEqual(num2words(175, lang="lo"), "ໜຶ່ງຮ້ອຍ ເຈັດສິບຫ້າ")
        self.assertEqual(num2words(199, lang="lo"), "ໜຶ່ງຮ້ອຍ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(200, lang="lo"), "ສອງຮ້ອຍ")
        self.assertEqual(num2words(201, lang="lo"), "ສອງຮ້ອຍ ໜຶ່ງ")
        self.assertEqual(num2words(210, lang="lo"), "ສອງຮ້ອຍ ສິບ")
        self.assertEqual(num2words(220, lang="lo"), "ສອງຮ້ອຍ ຊາວ")
        self.assertEqual(num2words(250, lang="lo"), "ສອງຮ້ອຍ ຫ້າສິບ")
        self.assertEqual(num2words(299, lang="lo"), "ສອງຮ້ອຍ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(300, lang="lo"), "ສາມຮ້ອຍ")
        self.assertEqual(num2words(333, lang="lo"), "ສາມຮ້ອຍ ສາມສິບສາມ")
        self.assertEqual(num2words(400, lang="lo"), "ສີ່ຮ້ອຍ")
        self.assertEqual(num2words(444, lang="lo"), "ສີ່ຮ້ອຍ ສີ່ສິບສີ່")
        self.assertEqual(num2words(500, lang="lo"), "ຫ້າຮ້ອຍ")
        self.assertEqual(num2words(555, lang="lo"), "ຫ້າຮ້ອຍ ຫ້າສິບຫ້າ")
        self.assertEqual(num2words(600, lang="lo"), "ຫົກຮ້ອຍ")
        self.assertEqual(num2words(666, lang="lo"), "ຫົກຮ້ອຍ ຫົກສິບຫົກ")
        self.assertEqual(num2words(700, lang="lo"), "ເຈັດຮ້ອຍ")
        self.assertEqual(num2words(777, lang="lo"), "ເຈັດຮ້ອຍ ເຈັດສິບເຈັດ")
        self.assertEqual(num2words(800, lang="lo"), "ແປດຮ້ອຍ")
        self.assertEqual(num2words(888, lang="lo"), "ແປດຮ້ອຍ ແປດສິບແປດ")
        self.assertEqual(num2words(900, lang="lo"), "ເກົ້າຮ້ອຍ")
        self.assertEqual(num2words(999, lang="lo"), "ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="lo"), "ໜຶ່ງພັນ")
        self.assertEqual(num2words(1001, lang="lo"), "ໜຶ່ງພັນ ໜຶ່ງ")
        self.assertEqual(num2words(1010, lang="lo"), "ໜຶ່ງພັນ ສິບ")
        self.assertEqual(num2words(1100, lang="lo"), "ໜຶ່ງພັນ ໜຶ່ງຮ້ອຍ")
        self.assertEqual(num2words(1111, lang="lo"), "ໜຶ່ງພັນ ໜຶ່ງຮ້ອຍ ສິບໜຶ່ງ")
        self.assertEqual(num2words(1234, lang="lo"), "ໜຶ່ງພັນ ສອງຮ້ອຍ ສາມສິບສີ່")
        self.assertEqual(num2words(1500, lang="lo"), "ໜຶ່ງພັນ ຫ້າຮ້ອຍ")
        self.assertEqual(num2words(1999, lang="lo"), "ໜຶ່ງພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(2000, lang="lo"), "ສອງພັນ")
        self.assertEqual(num2words(2001, lang="lo"), "ສອງພັນ ໜຶ່ງ")
        self.assertEqual(num2words(2020, lang="lo"), "ສອງພັນ ຊາວ")
        self.assertEqual(num2words(2222, lang="lo"), "ສອງພັນ ສອງຮ້ອຍ ຊາວສອງ")
        self.assertEqual(num2words(3000, lang="lo"), "ສາມພັນ")
        self.assertEqual(num2words(3333, lang="lo"), "ສາມພັນ ສາມຮ້ອຍ ສາມສິບສາມ")
        self.assertEqual(num2words(4000, lang="lo"), "ສີ່ພັນ")
        self.assertEqual(num2words(4444, lang="lo"), "ສີ່ພັນ ສີ່ຮ້ອຍ ສີ່ສິບສີ່")
        self.assertEqual(num2words(5000, lang="lo"), "ຫ້າພັນ")
        self.assertEqual(num2words(5555, lang="lo"), "ຫ້າພັນ ຫ້າຮ້ອຍ ຫ້າສິບຫ້າ")
        self.assertEqual(num2words(6000, lang="lo"), "ຫົກພັນ")
        self.assertEqual(num2words(6666, lang="lo"), "ຫົກພັນ ຫົກຮ້ອຍ ຫົກສິບຫົກ")
        self.assertEqual(num2words(7000, lang="lo"), "ເຈັດພັນ")
        self.assertEqual(num2words(7777, lang="lo"), "ເຈັດພັນ ເຈັດຮ້ອຍ ເຈັດສິບເຈັດ")
        self.assertEqual(num2words(8000, lang="lo"), "ແປດພັນ")
        self.assertEqual(num2words(8888, lang="lo"), "ແປດພັນ ແປດຮ້ອຍ ແປດສິບແປດ")
        self.assertEqual(num2words(9000, lang="lo"), "ເກົ້າພັນ")
        self.assertEqual(num2words(9999, lang="lo"), "ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(10000, lang="lo"), "ໜຶ່ງໝື່ນ")
        self.assertEqual(num2words(10001, lang="lo"), "ໜຶ່ງໝື່ນ ໜຶ່ງ")
        self.assertEqual(
            num2words(11111, lang="lo"), "ໜຶ່ງໝື່ນ ໜຶ່ງພັນ ໜຶ່ງຮ້ອຍ ສິບໜຶ່ງ"
        )
        self.assertEqual(
            num2words(12345, lang="lo"), "ໜຶ່ງໝື່ນ ສອງພັນ ສາມຮ້ອຍ ສີ່ສິບຫ້າ"
        )
        self.assertEqual(num2words(20000, lang="lo"), "ສອງໝື່ນ")
        self.assertEqual(num2words(50000, lang="lo"), "ຫ້າໝື່ນ")
        self.assertEqual(
            num2words(99999, lang="lo"), "ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ"
        )
        self.assertEqual(num2words(100000, lang="lo"), "ໜຶ່ງແສນ")
        self.assertEqual(
            num2words(123456, lang="lo"), "ໜຶ່ງແສນ ສອງໝື່ນ ສາມພັນ ສີ່ຮ້ອຍ ຫ້າສິບຫົກ"
        )
        self.assertEqual(num2words(200000, lang="lo"), "ສອງແສນ")
        self.assertEqual(num2words(500000, lang="lo"), "ຫ້າແສນ")
        self.assertEqual(
            num2words(654321, lang="lo"), "ຫົກແສນ ຫ້າໝື່ນ ສີ່ພັນ ສາມຮ້ອຍ ຊາວໜຶ່ງ"
        )
        self.assertEqual(
            num2words(999999, lang="lo"),
            "ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="lo"), "ໜຶ່ງ ລ້ານ")
        self.assertEqual(num2words(1000001, lang="lo"), "ໜຶ່ງ ລ້ານ ໜຶ່ງ")
        self.assertEqual(
            num2words(1111111, lang="lo"),
            "ໜຶ່ງ ລ້ານ ໜຶ່ງແສນ ໜຶ່ງໝື່ນ ໜຶ່ງພັນ ໜຶ່ງຮ້ອຍ ສິບໜຶ່ງ",
        )
        self.assertEqual(
            num2words(1234567, lang="lo"),
            "ໜຶ່ງ ລ້ານ ສອງແສນ ສາມໝື່ນ ສີ່ພັນ ຫ້າຮ້ອຍ ຫົກສິບເຈັດ",
        )
        self.assertEqual(num2words(2000000, lang="lo"), "ສອງ ລ້ານ")
        self.assertEqual(num2words(5000000, lang="lo"), "ຫ້າ ລ້ານ")
        self.assertEqual(
            num2words(9999999, lang="lo"),
            "ເກົ້າ ລ້ານ ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )
        self.assertEqual(num2words(10000000, lang="lo"), "ສິບ ລ້ານ")
        self.assertEqual(
            num2words(12345678, lang="lo"),
            "ສິບສອງ ລ້ານ ສາມແສນ ສີ່ໝື່ນ ຫ້າພັນ ຫົກຮ້ອຍ ເຈັດສິບແປດ",
        )
        self.assertEqual(
            num2words(99999999, lang="lo"),
            "ເກົ້າສິບເກົ້າ ລ້ານ ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )
        self.assertEqual(num2words(100000000, lang="lo"), "ໜຶ່ງຮ້ອຍ ລ້ານ")
        self.assertEqual(
            num2words(123456789, lang="lo"),
            "ໜຶ່ງຮ້ອຍ ຊາວສາມ ລ້ານ ສີ່ແສນ ຫ້າໝື່ນ ຫົກພັນ ເຈັດຮ້ອຍ ແປດສິບເກົ້າ",
        )
        self.assertEqual(
            num2words(999999999, lang="lo"),
            "ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ ລ້ານ ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="lo"),
            "ໜຶ່ງພັນ ລ້ານ",
        )
        self.assertEqual(
            num2words(1234567890, lang="lo"),
            "ໜຶ່ງພັນ ສອງຮ້ອຍ ສາມສິບສີ່ ລ້ານ ຫ້າແສນ ຫົກໝື່ນ ເຈັດພັນ ແປດຮ້ອຍ ເກົ້າສິບ",
        )
        self.assertEqual(
            num2words(9999999999, lang="lo"),
            "ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ ລ້ານ ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )
        self.assertEqual(
            num2words(10000000000, lang="lo"),
            "ໜຶ່ງໝື່ນ ລ້ານ",
        )
        self.assertEqual(
            num2words(99999999999, lang="lo"),
            "ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ ລ້ານ ເກົ້າແສນ ເກົ້າໝື່ນ ເກົ້າພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="lo"), "ລົບ ໜຶ່ງ")
        self.assertEqual(num2words(-2, lang="lo"), "ລົບ ສອງ")
        self.assertEqual(num2words(-5, lang="lo"), "ລົບ ຫ້າ")
        self.assertEqual(num2words(-10, lang="lo"), "ລົບ ສິບ")
        self.assertEqual(num2words(-11, lang="lo"), "ລົບ ສິບໜຶ່ງ")
        self.assertEqual(num2words(-20, lang="lo"), "ລົບ ຊາວ")
        self.assertEqual(num2words(-50, lang="lo"), "ລົບ ຫ້າສິບ")
        self.assertEqual(num2words(-99, lang="lo"), "ລົບ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(-100, lang="lo"), "ລົບ ໜຶ່ງຮ້ອຍ")
        self.assertEqual(num2words(-101, lang="lo"), "ລົບ ໜຶ່ງຮ້ອຍ ໜຶ່ງ")
        self.assertEqual(num2words(-200, lang="lo"), "ລົບ ສອງຮ້ອຍ")
        self.assertEqual(num2words(-999, lang="lo"), "ລົບ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ")
        self.assertEqual(num2words(-1000, lang="lo"), "ລົບ ໜຶ່ງພັນ")
        self.assertEqual(num2words(-1001, lang="lo"), "ລົບ ໜຶ່ງພັນ ໜຶ່ງ")
        self.assertEqual(num2words(-10000, lang="lo"), "ລົບ ໜຶ່ງໝື່ນ")
        self.assertEqual(num2words(-100000, lang="lo"), "ລົບ ໜຶ່ງແສນ")
        self.assertEqual(num2words(-1000000, lang="lo"), "ລົບ ໜຶ່ງ ລ້ານ")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="lo"), "ສູນ ຈຸດ ໜຶ່ງ")
        self.assertEqual(num2words(0.5, lang="lo"), "ສູນ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(0.9, lang="lo"), "ສູນ ຈຸດ ເກົ້າ")
        self.assertEqual(num2words(1.1, lang="lo"), "ໜຶ່ງ ຈຸດ ໜຶ່ງ")
        self.assertEqual(num2words(1.5, lang="lo"), "ໜຶ່ງ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(2.5, lang="lo"), "ສອງ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(3.14, lang="lo"), "ສາມ ຈຸດ ໜຶ່ງ ສີ່")
        self.assertEqual(num2words(10.5, lang="lo"), "ສິບ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(11.11, lang="lo"), "ສິບໜຶ່ງ ຈຸດ ໜຶ່ງ ໜຶ່ງ")
        self.assertEqual(num2words(20.2, lang="lo"), "ຊາວ ຈຸດ ສອງ")
        self.assertEqual(num2words(99.99, lang="lo"), "ເກົ້າສິບເກົ້າ ຈຸດ ເກົ້າ ເກົ້າ")
        self.assertEqual(num2words(100.01, lang="lo"), "ໜຶ່ງຮ້ອຍ ຈຸດ ສູນ ໜຶ່ງ")
        self.assertEqual(num2words(100.5, lang="lo"), "ໜຶ່ງຮ້ອຍ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(123.45, lang="lo"), "ໜຶ່ງຮ້ອຍ ຊາວສາມ ຈຸດ ສີ່ ຫ້າ")
        self.assertEqual(num2words(1000.5, lang="lo"), "ໜຶ່ງພັນ ຈຸດ ຫ້າ")
        self.assertEqual(
            num2words(1234.56, lang="lo"), "ໜຶ່ງພັນ ສອງຮ້ອຍ ສາມສິບສີ່ ຈຸດ ຫ້າ ຫົກ"
        )
        self.assertEqual(num2words(10000.01, lang="lo"), "ໜຶ່ງໝື່ນ ຈຸດ ສູນ ໜຶ່ງ")
        self.assertEqual(num2words(-0.5, lang="lo"), "ລົບ ສູນ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(-1.5, lang="lo"), "ລົບ ໜຶ່ງ ຈຸດ ຫ້າ")
        self.assertEqual(num2words(-10.5, lang="lo"), "ລົບ ສິບ ຈຸດ ຫ້າ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="lo", ordinal=True), "ທີ່ໜຶ່ງ")
        self.assertEqual(num2words(2, lang="lo", ordinal=True), "ທີ່ສອງ")
        self.assertEqual(num2words(3, lang="lo", ordinal=True), "ທີ່ສາມ")
        self.assertEqual(num2words(4, lang="lo", ordinal=True), "ທີ່ສີ່")
        self.assertEqual(num2words(5, lang="lo", ordinal=True), "ທີ່ຫ້າ")
        self.assertEqual(num2words(6, lang="lo", ordinal=True), "ທີ່ຫົກ")
        self.assertEqual(num2words(7, lang="lo", ordinal=True), "ທີ່ເຈັດ")
        self.assertEqual(num2words(8, lang="lo", ordinal=True), "ທີ່ແປດ")
        self.assertEqual(num2words(9, lang="lo", ordinal=True), "ທີ່ເກົ້າ")
        self.assertEqual(num2words(10, lang="lo", ordinal=True), "ທີ່ສິບ")
        self.assertEqual(num2words(11, lang="lo", ordinal=True), "ທີ່ສິບໜຶ່ງ")
        self.assertEqual(num2words(12, lang="lo", ordinal=True), "ທີ່ສິບສອງ")
        self.assertEqual(num2words(13, lang="lo", ordinal=True), "ທີ່ສິບສາມ")
        self.assertEqual(num2words(14, lang="lo", ordinal=True), "ທີ່ສິບສີ່")
        self.assertEqual(num2words(15, lang="lo", ordinal=True), "ທີ່ສິບຫ້າ")
        self.assertEqual(num2words(16, lang="lo", ordinal=True), "ທີ່ສິບຫົກ")
        self.assertEqual(num2words(17, lang="lo", ordinal=True), "ທີ່ສິບເຈັດ")
        self.assertEqual(num2words(18, lang="lo", ordinal=True), "ທີ່ສິບແປດ")
        self.assertEqual(num2words(19, lang="lo", ordinal=True), "ທີ່ສິບເກົ້າ")
        self.assertEqual(num2words(20, lang="lo", ordinal=True), "ທີ່ຊາວ")
        self.assertEqual(num2words(21, lang="lo", ordinal=True), "ທີ່ຊາວໜຶ່ງ")
        self.assertEqual(num2words(22, lang="lo", ordinal=True), "ທີ່ຊາວສອງ")
        self.assertEqual(num2words(25, lang="lo", ordinal=True), "ທີ່ຊາວຫ້າ")
        self.assertEqual(num2words(30, lang="lo", ordinal=True), "ທີ່ສາມສິບ")
        self.assertEqual(num2words(40, lang="lo", ordinal=True), "ທີ່ສີ່ສິບ")
        self.assertEqual(num2words(50, lang="lo", ordinal=True), "ທີ່ຫ້າສິບ")
        self.assertEqual(num2words(60, lang="lo", ordinal=True), "ທີ່ຫົກສິບ")
        self.assertEqual(num2words(70, lang="lo", ordinal=True), "ທີ່ເຈັດສິບ")
        self.assertEqual(num2words(80, lang="lo", ordinal=True), "ທີ່ແປດສິບ")
        self.assertEqual(num2words(90, lang="lo", ordinal=True), "ທີ່ເກົ້າສິບ")
        self.assertEqual(num2words(100, lang="lo", ordinal=True), "ທີ່ໜຶ່ງຮ້ອຍ")
        self.assertEqual(num2words(101, lang="lo", ordinal=True), "ທີ່ໜຶ່ງຮ້ອຍ ໜຶ່ງ")
        self.assertEqual(num2words(200, lang="lo", ordinal=True), "ທີ່ສອງຮ້ອຍ")
        self.assertEqual(num2words(500, lang="lo", ordinal=True), "ທີ່ຫ້າຮ້ອຍ")
        self.assertEqual(num2words(1000, lang="lo", ordinal=True), "ທີ່ໜຶ່ງພັນ")
        self.assertEqual(num2words(1001, lang="lo", ordinal=True), "ທີ່ໜຶ່ງພັນ ໜຶ່ງ")
        self.assertEqual(num2words(10000, lang="lo", ordinal=True), "ທີ່ໜຶ່ງໝື່ນ")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="lo", to="currency", currency="LAK"), "ສູນ ກີບ"
        )
        self.assertEqual(
            num2words(0.01, lang="lo", to="currency", currency="LAK"),
            "ສູນ ກີບ ໜຶ່ງ ອັດ",
        )
        self.assertEqual(
            num2words(0.5, lang="lo", to="currency", currency="LAK"),
            "ສູນ ກີບ ຫ້າສິບ ອັດ",
        )
        self.assertEqual(
            num2words(1, lang="lo", to="currency", currency="LAK"), "ໜຶ່ງ ກີບ"
        )
        self.assertEqual(
            num2words(1.5, lang="lo", to="currency", currency="LAK"),
            "ໜຶ່ງ ກີບ ຫ້າສິບ ອັດ",
        )
        self.assertEqual(
            num2words(0, lang="lo", to="currency", currency="USD"), "ສູນ ໂດລາ"
        )
        self.assertEqual(
            num2words(0.01, lang="lo", to="currency", currency="USD"),
            "ສູນ ໂດລາ ໜຶ່ງ ເຊັນ",
        )
        self.assertEqual(
            num2words(0.5, lang="lo", to="currency", currency="USD"),
            "ສູນ ໂດລາ ຫ້າສິບ ເຊັນ",
        )
        self.assertEqual(
            num2words(1, lang="lo", to="currency", currency="USD"), "ໜຶ່ງ ໂດລາ"
        )
        self.assertEqual(
            num2words(1.5, lang="lo", to="currency", currency="USD"),
            "ໜຶ່ງ ໂດລາ ຫ້າສິບ ເຊັນ",
        )
        self.assertEqual(
            num2words(0, lang="lo", to="currency", currency="EUR"), "ສູນ ເອີໂຣ"
        )
        self.assertEqual(
            num2words(0.01, lang="lo", to="currency", currency="EUR"),
            "ສູນ ເອີໂຣ ໜຶ່ງ ເຊັນ",
        )
        self.assertEqual(
            num2words(0.5, lang="lo", to="currency", currency="EUR"),
            "ສູນ ເອີໂຣ ຫ້າສິບ ເຊັນ",
        )
        self.assertEqual(
            num2words(1, lang="lo", to="currency", currency="EUR"), "ໜຶ່ງ ເອີໂຣ"
        )
        self.assertEqual(
            num2words(1.5, lang="lo", to="currency", currency="EUR"),
            "ໜຶ່ງ ເອີໂຣ ຫ້າສິບ ເຊັນ",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ")
        self.assertEqual(num2words(1066, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ຫົກສິບຫົກ")
        self.assertEqual(
            num2words(1492, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ສີ່ຮ້ອຍ ເກົ້າສິບສອງ"
        )
        self.assertEqual(
            num2words(1776, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ເຈັດຮ້ອຍ ເຈັດສິບຫົກ"
        )
        self.assertEqual(num2words(1800, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ແປດຮ້ອຍ")
        self.assertEqual(num2words(1900, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ເກົ້າຮ້ອຍ")
        self.assertEqual(
            num2words(1984, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ເກົ້າຮ້ອຍ ແປດສິບສີ່"
        )
        self.assertEqual(
            num2words(1999, lang="lo", to="year"), "ປີ ໜຶ່ງພັນ ເກົ້າຮ້ອຍ ເກົ້າສິບເກົ້າ"
        )
        self.assertEqual(num2words(2000, lang="lo", to="year"), "ປີ ສອງພັນ")
        self.assertEqual(num2words(2001, lang="lo", to="year"), "ປີ ສອງພັນ ໜຶ່ງ")
        self.assertEqual(num2words(2010, lang="lo", to="year"), "ປີ ສອງພັນ ສິບ")
        self.assertEqual(num2words(2020, lang="lo", to="year"), "ປີ ສອງພັນ ຊາວ")
        self.assertEqual(num2words(2024, lang="lo", to="year"), "ປີ ສອງພັນ ຊາວສີ່")
        self.assertEqual(num2words(2100, lang="lo", to="year"), "ປີ ສອງພັນ ໜຶ່ງຮ້ອຍ")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="lo"), "ສູນ")
        self.assertEqual(num2words("1", lang="lo"), "ໜຶ່ງ")
        self.assertEqual(num2words("10", lang="lo"), "ສິບ")
        self.assertEqual(num2words("100", lang="lo"), "ໜຶ່ງຮ້ອຍ")
        self.assertEqual(num2words("1000", lang="lo"), "ໜຶ່ງພັນ")
        self.assertEqual(num2words("10000", lang="lo"), "ໜຶ່ງໝື່ນ")
        self.assertEqual(num2words("100000", lang="lo"), "ໜຶ່ງແສນ")
        self.assertEqual(num2words("1000000", lang="lo"), "ໜຶ່ງ ລ້ານ")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="lo"), "ສູນ")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="lo"), num2words("100", lang="lo"))
        self.assertEqual(num2words(1000, lang="lo"), num2words("1000", lang="lo"))
