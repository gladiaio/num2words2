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


class Num2WordsASTest(TestCase):
    """Comprehensive test cases for Assamese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="as"), "শূন্য")
        self.assertEqual(num2words(1, lang="as"), "এক")
        self.assertEqual(num2words(2, lang="as"), "দুই")
        self.assertEqual(num2words(3, lang="as"), "তিনি")
        self.assertEqual(num2words(4, lang="as"), "চাৰি")
        self.assertEqual(num2words(5, lang="as"), "পাঁচ")
        self.assertEqual(num2words(6, lang="as"), "ছয়")
        self.assertEqual(num2words(7, lang="as"), "সাত")
        self.assertEqual(num2words(8, lang="as"), "আঠ")
        self.assertEqual(num2words(9, lang="as"), "নয়")
        self.assertEqual(num2words(10, lang="as"), "দহ")
        self.assertEqual(num2words(11, lang="as"), "এঘাৰ")
        self.assertEqual(num2words(12, lang="as"), "বাৰ")
        self.assertEqual(num2words(13, lang="as"), "তেৰ")
        self.assertEqual(num2words(14, lang="as"), "চৈধ্য")
        self.assertEqual(num2words(15, lang="as"), "পোন্ধৰ")
        self.assertEqual(num2words(16, lang="as"), "ষোল্ল")
        self.assertEqual(num2words(17, lang="as"), "সোতৰ")
        self.assertEqual(num2words(18, lang="as"), "ওঠৰ")
        self.assertEqual(num2words(19, lang="as"), "ঊনৈশ")
        self.assertEqual(num2words(20, lang="as"), "বিশ")
        self.assertEqual(num2words(21, lang="as"), "একৈশ")
        self.assertEqual(num2words(22, lang="as"), "বাইশ")
        self.assertEqual(num2words(23, lang="as"), "তেইশ")
        self.assertEqual(num2words(24, lang="as"), "চৌবিশ")
        self.assertEqual(num2words(25, lang="as"), "পঁচিশ")
        self.assertEqual(num2words(26, lang="as"), "ছাবিশ")
        self.assertEqual(num2words(27, lang="as"), "সাতাইশ")
        self.assertEqual(num2words(28, lang="as"), "আঠাইশ")
        self.assertEqual(num2words(29, lang="as"), "ঊনত্ৰিশ")
        self.assertEqual(num2words(30, lang="as"), "ত্ৰিশ")
        self.assertEqual(num2words(31, lang="as"), "একত্ৰিশ")
        self.assertEqual(num2words(35, lang="as"), "পঁয়ত্ৰিশ")
        self.assertEqual(num2words(40, lang="as"), "চল্লিশ")
        self.assertEqual(num2words(45, lang="as"), "পঞ্চল্লিশ")
        self.assertEqual(num2words(50, lang="as"), "পঞ্চাশ")
        self.assertEqual(num2words(55, lang="as"), "পঁচপন")
        self.assertEqual(num2words(60, lang="as"), "ষাঠি")
        self.assertEqual(num2words(65, lang="as"), "পঁষষ্ঠি")
        self.assertEqual(num2words(70, lang="as"), "সত্তৰ")
        self.assertEqual(num2words(75, lang="as"), "পঁসত্তৰ")
        self.assertEqual(num2words(80, lang="as"), "আশী")
        self.assertEqual(num2words(85, lang="as"), "পঁচাশী")
        self.assertEqual(num2words(90, lang="as"), "নব্বৈ")
        self.assertEqual(num2words(95, lang="as"), "পঁচানব্বৈ")
        self.assertEqual(num2words(99, lang="as"), "নিৰান্নব্বৈ")
        self.assertEqual(num2words(100, lang="as"), "এক শ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="as"), "এক শ এক")
        self.assertEqual(num2words(110, lang="as"), "এক শ দহ")
        self.assertEqual(num2words(111, lang="as"), "এক শ এঘাৰ")
        self.assertEqual(num2words(120, lang="as"), "এক শ বিশ")
        self.assertEqual(num2words(125, lang="as"), "এক শ পঁচিশ")
        self.assertEqual(num2words(150, lang="as"), "এক শ পঞ্চাশ")
        self.assertEqual(num2words(175, lang="as"), "এক শ পঁসত্তৰ")
        self.assertEqual(num2words(199, lang="as"), "এক শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(200, lang="as"), "দুই শ")
        self.assertEqual(num2words(201, lang="as"), "দুই শ এক")
        self.assertEqual(num2words(210, lang="as"), "দুই শ দহ")
        self.assertEqual(num2words(220, lang="as"), "দুই শ বিশ")
        self.assertEqual(num2words(250, lang="as"), "দুই শ পঞ্চাশ")
        self.assertEqual(num2words(299, lang="as"), "দুই শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(300, lang="as"), "তিনি শ")
        self.assertEqual(num2words(333, lang="as"), "তিনি শ তেত্ৰিশ")
        self.assertEqual(num2words(400, lang="as"), "চাৰি শ")
        self.assertEqual(num2words(444, lang="as"), "চাৰি শ চৌৰাল্লিশ")
        self.assertEqual(num2words(500, lang="as"), "পাঁচ শ")
        self.assertEqual(num2words(555, lang="as"), "পাঁচ শ পঁচপন")
        self.assertEqual(num2words(600, lang="as"), "ছয় শ")
        self.assertEqual(num2words(666, lang="as"), "ছয় শ ছয়ষষ্ঠি")
        self.assertEqual(num2words(700, lang="as"), "সাত শ")
        self.assertEqual(num2words(777, lang="as"), "সাত শ সাতসত্তৰ")
        self.assertEqual(num2words(800, lang="as"), "আঠ শ")
        self.assertEqual(num2words(888, lang="as"), "আঠ শ আঠাশী")
        self.assertEqual(num2words(900, lang="as"), "নয় শ")
        self.assertEqual(num2words(999, lang="as"), "নয় শ নিৰান্নব্বৈ")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="as"), "এক হাজাৰ")
        self.assertEqual(num2words(1001, lang="as"), "এক হাজাৰ এক")
        self.assertEqual(num2words(1010, lang="as"), "এক হাজাৰ দহ")
        self.assertEqual(num2words(1100, lang="as"), "এক হাজাৰ এক শ")
        self.assertEqual(num2words(1111, lang="as"), "এক হাজাৰ এক শ এঘাৰ")
        self.assertEqual(num2words(1234, lang="as"), "এক হাজাৰ দুই শ চৌত্ৰিশ")
        self.assertEqual(num2words(1500, lang="as"), "এক হাজাৰ পাঁচ শ")
        self.assertEqual(num2words(1999, lang="as"), "এক হাজাৰ নয় শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(2000, lang="as"), "দুই হাজাৰ")
        self.assertEqual(num2words(2001, lang="as"), "দুই হাজাৰ এক")
        self.assertEqual(num2words(2020, lang="as"), "দুই হাজাৰ বিশ")
        self.assertEqual(num2words(2222, lang="as"), "দুই হাজাৰ দুই শ বাইশ")
        self.assertEqual(num2words(3000, lang="as"), "তিনি হাজাৰ")
        self.assertEqual(num2words(3333, lang="as"), "তিনি হাজাৰ তিনি শ তেত্ৰিশ")
        self.assertEqual(num2words(4000, lang="as"), "চাৰি হাজাৰ")
        self.assertEqual(num2words(4444, lang="as"), "চাৰি হাজাৰ চাৰি শ চৌৰাল্লিশ")
        self.assertEqual(num2words(5000, lang="as"), "পাঁচ হাজাৰ")
        self.assertEqual(num2words(5555, lang="as"), "পাঁচ হাজাৰ পাঁচ শ পঁচপন")
        self.assertEqual(num2words(6000, lang="as"), "ছয় হাজাৰ")
        self.assertEqual(num2words(6666, lang="as"), "ছয় হাজাৰ ছয় শ ছয়ষষ্ঠি")
        self.assertEqual(num2words(7000, lang="as"), "সাত হাজাৰ")
        self.assertEqual(num2words(7777, lang="as"), "সাত হাজাৰ সাত শ সাতসত্তৰ")
        self.assertEqual(num2words(8000, lang="as"), "আঠ হাজাৰ")
        self.assertEqual(num2words(8888, lang="as"), "আঠ হাজাৰ আঠ শ আঠাশী")
        self.assertEqual(num2words(9000, lang="as"), "নয় হাজাৰ")
        self.assertEqual(num2words(9999, lang="as"), "নয় হাজাৰ নয় শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(10000, lang="as"), "দহ হাজাৰ")
        self.assertEqual(num2words(10001, lang="as"), "দহ হাজাৰ এক")
        self.assertEqual(num2words(11111, lang="as"), "এঘাৰ হাজাৰ এক শ এঘাৰ")
        self.assertEqual(num2words(12345, lang="as"), "বাৰ হাজাৰ তিনি শ পঞ্চল্লিশ")
        self.assertEqual(num2words(20000, lang="as"), "বিশ হাজাৰ")
        self.assertEqual(num2words(50000, lang="as"), "পঞ্চাশ হাজাৰ")
        self.assertEqual(num2words(99999, lang="as"), "নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(100000, lang="as"), "এক লাখ")
        self.assertEqual(
            num2words(123456, lang="as"), "এক লাখ তেইশ হাজাৰ চাৰি শ ছাপন"
        )
        self.assertEqual(num2words(200000, lang="as"), "দুই লাখ")
        self.assertEqual(num2words(500000, lang="as"), "পাঁচ লাখ")
        self.assertEqual(
            num2words(654321, lang="as"), "ছয় লাখ চৌৱন হাজাৰ তিনি শ একৈশ"
        )
        self.assertEqual(
            num2words(999999, lang="as"), "নয় লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="as"), "দহ লাখ")
        self.assertEqual(num2words(1000001, lang="as"), "দহ লাখ এক")
        self.assertEqual(num2words(1111111, lang="as"), "এঘাৰ লাখ এঘাৰ হাজাৰ এক শ এঘাৰ")
        self.assertEqual(
            num2words(1234567, lang="as"), "বাৰ লাখ চৌত্ৰিশ হাজাৰ পাঁচ শ সাতষষ্ঠি"
        )
        self.assertEqual(num2words(2000000, lang="as"), "বিশ লাখ")
        self.assertEqual(num2words(5000000, lang="as"), "পঞ্চাশ লাখ")
        self.assertEqual(
            num2words(9999999, lang="as"),
            "নিৰান্নব্বৈ লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ",
        )
        self.assertEqual(num2words(10000000, lang="as"), "এক কোটি")
        self.assertEqual(
            num2words(12345678, lang="as"),
            "এক কোটি তেইশ লাখ পঞ্চল্লিশ হাজাৰ ছয় শ আঠসত্তৰ",
        )
        self.assertEqual(
            num2words(99999999, lang="as"),
            "নয় কোটি নিৰান্নব্বৈ লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ",
        )
        self.assertEqual(num2words(100000000, lang="as"), "দহ কোটি")
        self.assertEqual(
            num2words(123456789, lang="as"),
            "বাৰ কোটি চৌত্ৰিশ লাখ ছাপন হাজাৰ সাত শ ঊননব্বৈ",
        )
        self.assertEqual(
            num2words(999999999, lang="as"),
            "নিৰান্নব্বৈ কোটি নিৰান্নব্বৈ লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ",
        )
        self.assertEqual(num2words(1000000000, lang="as"), "এক শ কোটি")
        self.assertEqual(
            num2words(1234567890, lang="as"),
            "এক শ তেইশ কোটি পঞ্চল্লিশ লাখ সাতষষ্ঠি হাজাৰ আঠ শ নব্বৈ",
        )
        self.assertEqual(
            num2words(9999999999, lang="as"),
            "নয় শ নিৰান্নব্বৈ কোটি নিৰান্নব্বৈ লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ",
        )
        self.assertEqual(num2words(10000000000, lang="as"), "এক হাজাৰ কোটি")
        self.assertEqual(
            num2words(99999999999, lang="as"),
            "নয় হাজাৰ নয় শ নিৰান্নব্বৈ কোটি নিৰান্নব্বৈ লাখ নিৰান্নব্বৈ হাজাৰ নয় শ নিৰান্নব্বৈ",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="as"), "ঋণাত্মক এক")
        self.assertEqual(num2words(-2, lang="as"), "ঋণাত্মক দুই")
        self.assertEqual(num2words(-5, lang="as"), "ঋণাত্মক পাঁচ")
        self.assertEqual(num2words(-10, lang="as"), "ঋণাত্মক দহ")
        self.assertEqual(num2words(-11, lang="as"), "ঋণাত্মক এঘাৰ")
        self.assertEqual(num2words(-20, lang="as"), "ঋণাত্মক বিশ")
        self.assertEqual(num2words(-50, lang="as"), "ঋণাত্মক পঞ্চাশ")
        self.assertEqual(num2words(-99, lang="as"), "ঋণাত্মক নিৰান্নব্বৈ")
        self.assertEqual(num2words(-100, lang="as"), "ঋণাত্মক এক শ")
        self.assertEqual(num2words(-101, lang="as"), "ঋণাত্মক এক শ এক")
        self.assertEqual(num2words(-200, lang="as"), "ঋণাত্মক দুই শ")
        self.assertEqual(num2words(-999, lang="as"), "ঋণাত্মক নয় শ নিৰান্নব্বৈ")
        self.assertEqual(num2words(-1000, lang="as"), "ঋণাত্মক এক হাজাৰ")
        self.assertEqual(num2words(-1001, lang="as"), "ঋণাত্মক এক হাজাৰ এক")
        self.assertEqual(num2words(-10000, lang="as"), "ঋণাত্মক দহ হাজাৰ")
        self.assertEqual(num2words(-100000, lang="as"), "ঋণাত্মক এক লাখ")
        self.assertEqual(num2words(-1000000, lang="as"), "ঋণাত্মক দহ লাখ")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="as"), "শূন্য দশমিক এক")
        self.assertEqual(num2words(0.5, lang="as"), "শূন্য দশমিক পাঁচ")
        self.assertEqual(num2words(0.9, lang="as"), "শূন্য দশমিক নয়")
        self.assertEqual(num2words(1.1, lang="as"), "এক দশমিক এক")
        self.assertEqual(num2words(1.5, lang="as"), "এক দশমিক পাঁচ")
        self.assertEqual(num2words(2.5, lang="as"), "দুই দশমিক পাঁচ")
        self.assertEqual(num2words(3.14, lang="as"), "তিনি দশমিক এক চাৰি")
        self.assertEqual(num2words(10.5, lang="as"), "দহ দশমিক পাঁচ")
        self.assertEqual(num2words(11.11, lang="as"), "এঘাৰ দশমিক এক এক")
        self.assertEqual(num2words(20.2, lang="as"), "বিশ দশমিক দুই")
        self.assertEqual(num2words(99.99, lang="as"), "নিৰান্নব্বৈ দশমিক নয় নয়")
        self.assertEqual(num2words(100.01, lang="as"), "এক শ দশমিক শূন্য এক")
        self.assertEqual(num2words(100.5, lang="as"), "এক শ দশমিক পাঁচ")
        self.assertEqual(num2words(123.45, lang="as"), "এক শ তেইশ দশমিক চাৰি পাঁচ")
        self.assertEqual(num2words(1000.5, lang="as"), "এক হাজাৰ দশমিক পাঁচ")
        self.assertEqual(
            num2words(1234.56, lang="as"), "এক হাজাৰ দুই শ চৌত্ৰিশ দশমিক পাঁচ ছয়"
        )
        self.assertEqual(num2words(10000.01, lang="as"), "দহ হাজাৰ দশমিক শূন্য এক")
        self.assertEqual(num2words(-0.5, lang="as"), "ঋণাত্মক শূন্য দশমিক পাঁচ")
        self.assertEqual(num2words(-1.5, lang="as"), "ঋণাত্মক এক দশমিক পাঁচ")
        self.assertEqual(num2words(-10.5, lang="as"), "ঋণাত্মক দহ দশমিক পাঁচ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="as", ordinal=True), "এক -তম")
        self.assertEqual(num2words(2, lang="as", ordinal=True), "দুই -তম")
        self.assertEqual(num2words(3, lang="as", ordinal=True), "তিনি -তম")
        self.assertEqual(num2words(4, lang="as", ordinal=True), "চাৰি -তম")
        self.assertEqual(num2words(5, lang="as", ordinal=True), "পাঁচ -তম")
        self.assertEqual(num2words(6, lang="as", ordinal=True), "ছয় -তম")
        self.assertEqual(num2words(7, lang="as", ordinal=True), "সাত -তম")
        self.assertEqual(num2words(8, lang="as", ordinal=True), "আঠ -তম")
        self.assertEqual(num2words(9, lang="as", ordinal=True), "নয় -তম")
        self.assertEqual(num2words(10, lang="as", ordinal=True), "দহ -তম")
        self.assertEqual(num2words(11, lang="as", ordinal=True), "এঘাৰ -তম")
        self.assertEqual(num2words(12, lang="as", ordinal=True), "বাৰ -তম")
        self.assertEqual(num2words(13, lang="as", ordinal=True), "তেৰ -তম")
        self.assertEqual(num2words(14, lang="as", ordinal=True), "চৈধ্য -তম")
        self.assertEqual(num2words(15, lang="as", ordinal=True), "পোন্ধৰ -তম")
        self.assertEqual(num2words(16, lang="as", ordinal=True), "ষোল্ল -তম")
        self.assertEqual(num2words(17, lang="as", ordinal=True), "সোতৰ -তম")
        self.assertEqual(num2words(18, lang="as", ordinal=True), "ওঠৰ -তম")
        self.assertEqual(num2words(19, lang="as", ordinal=True), "ঊনৈশ -তম")
        self.assertEqual(num2words(20, lang="as", ordinal=True), "বিশ -তম")
        self.assertEqual(num2words(21, lang="as", ordinal=True), "একৈশ -তম")
        self.assertEqual(num2words(22, lang="as", ordinal=True), "বাইশ -তম")
        self.assertEqual(num2words(25, lang="as", ordinal=True), "পঁচিশ -তম")
        self.assertEqual(num2words(30, lang="as", ordinal=True), "ত্ৰিশ -তম")
        self.assertEqual(num2words(40, lang="as", ordinal=True), "চল্লিশ -তম")
        self.assertEqual(num2words(50, lang="as", ordinal=True), "পঞ্চাশ -তম")
        self.assertEqual(num2words(60, lang="as", ordinal=True), "ষাঠি -তম")
        self.assertEqual(num2words(70, lang="as", ordinal=True), "সত্তৰ -তম")
        self.assertEqual(num2words(80, lang="as", ordinal=True), "আশী -তম")
        self.assertEqual(num2words(90, lang="as", ordinal=True), "নব্বৈ -তম")
        self.assertEqual(num2words(100, lang="as", ordinal=True), "এক শ -তম")
        self.assertEqual(num2words(101, lang="as", ordinal=True), "এক শ এক -তম")
        self.assertEqual(num2words(200, lang="as", ordinal=True), "দুই শ -তম")
        self.assertEqual(num2words(500, lang="as", ordinal=True), "পাঁচ শ -তম")
        self.assertEqual(num2words(1000, lang="as", ordinal=True), "এক হাজাৰ -তম")
        self.assertEqual(num2words(1001, lang="as", ordinal=True), "এক হাজাৰ এক -তম")
        self.assertEqual(num2words(10000, lang="as", ordinal=True), "দহ হাজাৰ -তম")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="as", to="currency", currency="INR"), "শূন্য ৰুপী"
        )
        self.assertEqual(
            num2words(0.01, lang="as", to="currency", currency="INR"),
            "শূন্য ৰুপী আৰুএক পইচা",
        )
        self.assertEqual(
            num2words(0.5, lang="as", to="currency", currency="INR"),
            "শূন্য ৰুপী আৰুপঞ্চাশ পইচা",
        )
        self.assertEqual(
            num2words(1, lang="as", to="currency", currency="INR"), "এক ৰুপী"
        )
        self.assertEqual(
            num2words(1.5, lang="as", to="currency", currency="INR"),
            "এক ৰুপী আৰুপঞ্চাশ পইচা",
        )
        self.assertEqual(
            num2words(0, lang="as", to="currency", currency="USD"), "শূন্য ডলাৰ"
        )
        self.assertEqual(
            num2words(0.01, lang="as", to="currency", currency="USD"),
            "শূন্য ডলাৰ আৰুএক চেণ্ট",
        )
        self.assertEqual(
            num2words(0.5, lang="as", to="currency", currency="USD"),
            "শূন্য ডলাৰ আৰুপঞ্চাশ চেণ্ট",
        )
        self.assertEqual(
            num2words(1, lang="as", to="currency", currency="USD"), "এক ডলাৰ"
        )
        self.assertEqual(
            num2words(1.5, lang="as", to="currency", currency="USD"),
            "এক ডলাৰ আৰুপঞ্চাশ চেণ্ট",
        )
        self.assertEqual(
            num2words(0, lang="as", to="currency", currency="EUR"), "শূন্য ইউৰো"
        )
        self.assertEqual(
            num2words(0.01, lang="as", to="currency", currency="EUR"),
            "শূন্য ইউৰো আৰুএক চেণ্ট",
        )
        self.assertEqual(
            num2words(0.5, lang="as", to="currency", currency="EUR"),
            "শূন্য ইউৰো আৰুপঞ্চাশ চেণ্ট",
        )
        self.assertEqual(
            num2words(1, lang="as", to="currency", currency="EUR"), "এক ইউৰো"
        )
        self.assertEqual(
            num2words(1.5, lang="as", to="currency", currency="EUR"),
            "এক ইউৰো আৰুপঞ্চাশ চেণ্ট",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="as", to="year"), "চন এক হাজাৰ")
        self.assertEqual(num2words(1066, lang="as", to="year"), "চন এক হাজাৰ ছয়ষষ্ঠি")
        self.assertEqual(
            num2words(1492, lang="as", to="year"), "চন এক হাজাৰ চাৰি শ বিয়ান্নব্বৈ"
        )
        self.assertEqual(
            num2words(1776, lang="as", to="year"), "চন এক হাজাৰ সাত শ ছয়সত্তৰ"
        )
        self.assertEqual(num2words(1800, lang="as", to="year"), "চন এক হাজাৰ আঠ শ")
        self.assertEqual(num2words(1900, lang="as", to="year"), "চন এক হাজাৰ নয় শ")
        self.assertEqual(
            num2words(1984, lang="as", to="year"), "চন এক হাজাৰ নয় শ চৌৰাশী"
        )
        self.assertEqual(
            num2words(1999, lang="as", to="year"), "চন এক হাজাৰ নয় শ নিৰান্নব্বৈ"
        )
        self.assertEqual(num2words(2000, lang="as", to="year"), "চন দুই হাজাৰ")
        self.assertEqual(num2words(2001, lang="as", to="year"), "চন দুই হাজাৰ এক")
        self.assertEqual(num2words(2010, lang="as", to="year"), "চন দুই হাজাৰ দহ")
        self.assertEqual(num2words(2020, lang="as", to="year"), "চন দুই হাজাৰ বিশ")
        self.assertEqual(num2words(2024, lang="as", to="year"), "চন দুই হাজাৰ চৌবিশ")
        self.assertEqual(num2words(2100, lang="as", to="year"), "চন দুই হাজাৰ এক শ")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="as"), "শূন্য")
        self.assertEqual(num2words("1", lang="as"), "এক")
        self.assertEqual(num2words("10", lang="as"), "দহ")
        self.assertEqual(num2words("100", lang="as"), "এক শ")
        self.assertEqual(num2words("1000", lang="as"), "এক হাজাৰ")
        self.assertEqual(num2words("10000", lang="as"), "দহ হাজাৰ")
        self.assertEqual(num2words("100000", lang="as"), "এক লাখ")
        self.assertEqual(num2words("1000000", lang="as"), "দহ লাখ")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="as"), "শূন্য")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="as"), num2words("100", lang="as"))
        self.assertEqual(num2words(1000, lang="as"), num2words("1000", lang="as"))

