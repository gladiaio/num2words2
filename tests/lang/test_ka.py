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


class Num2WordsKATest(TestCase):
    """Comprehensive test cases for Georgian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ka"), "ნული")
        self.assertEqual(num2words(1, lang="ka"), "ერთი")
        self.assertEqual(num2words(2, lang="ka"), "ორი")
        self.assertEqual(num2words(3, lang="ka"), "სამი")
        self.assertEqual(num2words(4, lang="ka"), "ოთხი")
        self.assertEqual(num2words(5, lang="ka"), "ხუთი")
        self.assertEqual(num2words(6, lang="ka"), "ექვსი")
        self.assertEqual(num2words(7, lang="ka"), "შვიდი")
        self.assertEqual(num2words(8, lang="ka"), "რვა")
        self.assertEqual(num2words(9, lang="ka"), "ცხრა")
        self.assertEqual(num2words(10, lang="ka"), "ათი")
        self.assertEqual(num2words(11, lang="ka"), "თერთმეტი")
        self.assertEqual(num2words(12, lang="ka"), "თორმეტი")
        self.assertEqual(num2words(13, lang="ka"), "ცამეტი")
        self.assertEqual(num2words(14, lang="ka"), "თოთხმეტი")
        self.assertEqual(num2words(15, lang="ka"), "თხუთმეტი")
        self.assertEqual(num2words(16, lang="ka"), "თექვსმეტი")
        self.assertEqual(num2words(17, lang="ka"), "ჩვიდმეტი")
        self.assertEqual(num2words(18, lang="ka"), "თვრამეტი")
        self.assertEqual(num2words(19, lang="ka"), "ცხრამეტი")
        self.assertEqual(num2words(20, lang="ka"), "ოცი")
        self.assertEqual(num2words(21, lang="ka"), "ოცი ერთი")
        self.assertEqual(num2words(22, lang="ka"), "ოცი ორი")
        self.assertEqual(num2words(23, lang="ka"), "ოცი სამი")
        self.assertEqual(num2words(24, lang="ka"), "ოცი ოთხი")
        self.assertEqual(num2words(25, lang="ka"), "ოცი ხუთი")
        self.assertEqual(num2words(26, lang="ka"), "ოცი ექვსი")
        self.assertEqual(num2words(27, lang="ka"), "ოცი შვიდი")
        self.assertEqual(num2words(28, lang="ka"), "ოცი რვა")
        self.assertEqual(num2words(29, lang="ka"), "ოცი ცხრა")
        self.assertEqual(num2words(30, lang="ka"), "ოცდაათი")
        self.assertEqual(num2words(31, lang="ka"), "ოცდაათი ერთი")
        self.assertEqual(num2words(35, lang="ka"), "ოცდაათი ხუთი")
        self.assertEqual(num2words(40, lang="ka"), "ორმოცი")
        self.assertEqual(num2words(45, lang="ka"), "ორმოცი ხუთი")
        self.assertEqual(num2words(50, lang="ka"), "ორმოცდაათი")
        self.assertEqual(num2words(55, lang="ka"), "ორმოცდაათი ხუთი")
        self.assertEqual(num2words(60, lang="ka"), "სამოცი")
        self.assertEqual(num2words(65, lang="ka"), "სამოცი ხუთი")
        self.assertEqual(num2words(70, lang="ka"), "სამოცდაათი")
        self.assertEqual(num2words(75, lang="ka"), "სამოცდაათი ხუთი")
        self.assertEqual(num2words(80, lang="ka"), "ოთხმოცი")
        self.assertEqual(num2words(85, lang="ka"), "ოთხმოცი ხუთი")
        self.assertEqual(num2words(90, lang="ka"), "ოთხმოცდაათი")
        self.assertEqual(num2words(95, lang="ka"), "ოთხმოცდაათი ხუთი")
        self.assertEqual(num2words(99, lang="ka"), "ოთხმოცდაათი ცხრა")
        self.assertEqual(num2words(100, lang="ka"), "ერთი ასი")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ka"), "ერთი ასი ერთი")
        self.assertEqual(num2words(110, lang="ka"), "ერთი ასი ათი")
        self.assertEqual(num2words(111, lang="ka"), "ერთი ასი თერთმეტი")
        self.assertEqual(num2words(120, lang="ka"), "ერთი ასი ოცი")
        self.assertEqual(num2words(125, lang="ka"), "ერთი ასი ოცი ხუთი")
        self.assertEqual(num2words(150, lang="ka"), "ერთი ასი ორმოცდაათი")
        self.assertEqual(num2words(175, lang="ka"), "ერთი ასი სამოცდაათი ხუთი")
        self.assertEqual(num2words(199, lang="ka"), "ერთი ასი ოთხმოცდაათი ცხრა")
        self.assertEqual(num2words(200, lang="ka"), "ორი ასი")
        self.assertEqual(num2words(201, lang="ka"), "ორი ასი ერთი")
        self.assertEqual(num2words(210, lang="ka"), "ორი ასი ათი")
        self.assertEqual(num2words(220, lang="ka"), "ორი ასი ოცი")
        self.assertEqual(num2words(250, lang="ka"), "ორი ასი ორმოცდაათი")
        self.assertEqual(num2words(299, lang="ka"), "ორი ასი ოთხმოცდაათი ცხრა")
        self.assertEqual(num2words(300, lang="ka"), "სამი ასი")
        self.assertEqual(num2words(333, lang="ka"), "სამი ასი ოცდაათი სამი")
        self.assertEqual(num2words(400, lang="ka"), "ოთხი ასი")
        self.assertEqual(num2words(444, lang="ka"), "ოთხი ასი ორმოცი ოთხი")
        self.assertEqual(num2words(500, lang="ka"), "ხუთი ასი")
        self.assertEqual(num2words(555, lang="ka"), "ხუთი ასი ორმოცდაათი ხუთი")
        self.assertEqual(num2words(600, lang="ka"), "ექვსი ასი")
        self.assertEqual(num2words(666, lang="ka"), "ექვსი ასი სამოცი ექვსი")
        self.assertEqual(num2words(700, lang="ka"), "შვიდი ასი")
        self.assertEqual(num2words(777, lang="ka"), "შვიდი ასი სამოცდაათი შვიდი")
        self.assertEqual(num2words(800, lang="ka"), "რვა ასი")
        self.assertEqual(num2words(888, lang="ka"), "რვა ასი ოთხმოცი რვა")
        self.assertEqual(num2words(900, lang="ka"), "ცხრა ასი")
        self.assertEqual(num2words(999, lang="ka"), "ცხრა ასი ოთხმოცდაათი ცხრა")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ka"), "ერთი ათასი")
        self.assertEqual(num2words(1001, lang="ka"), "ერთი ათასი ერთი")
        self.assertEqual(num2words(1010, lang="ka"), "ერთი ათასი ათი")
        self.assertEqual(num2words(1100, lang="ka"), "ერთი ათასი ერთი ასი")
        self.assertEqual(num2words(1111, lang="ka"), "ერთი ათასი ერთი ასი თერთმეტი")
        self.assertEqual(num2words(1234, lang="ka"), "ერთი ათასი ორი ასი ოცდაათი ოთხი")
        self.assertEqual(num2words(1500, lang="ka"), "ერთი ათასი ხუთი ასი")
        self.assertEqual(
            num2words(1999, lang="ka"), "ერთი ათასი ცხრა ასი ოთხმოცდაათი ცხრა"
        )
        self.assertEqual(num2words(2000, lang="ka"), "ორი ათასი")
        self.assertEqual(num2words(2001, lang="ka"), "ორი ათასი ერთი")
        self.assertEqual(num2words(2020, lang="ka"), "ორი ათასი ოცი")
        self.assertEqual(num2words(2222, lang="ka"), "ორი ათასი ორი ასი ოცი ორი")
        self.assertEqual(num2words(3000, lang="ka"), "სამი ათასი")
        self.assertEqual(num2words(3333, lang="ka"), "სამი ათასი სამი ასი ოცდაათი სამი")
        self.assertEqual(num2words(4000, lang="ka"), "ოთხი ათასი")
        self.assertEqual(num2words(4444, lang="ka"), "ოთხი ათასი ოთხი ასი ორმოცი ოთხი")
        self.assertEqual(num2words(5000, lang="ka"), "ხუთი ათასი")
        self.assertEqual(
            num2words(5555, lang="ka"), "ხუთი ათასი ხუთი ასი ორმოცდაათი ხუთი"
        )
        self.assertEqual(num2words(6000, lang="ka"), "ექვსი ათასი")
        self.assertEqual(
            num2words(6666, lang="ka"), "ექვსი ათასი ექვსი ასი სამოცი ექვსი"
        )
        self.assertEqual(num2words(7000, lang="ka"), "შვიდი ათასი")
        self.assertEqual(
            num2words(7777, lang="ka"), "შვიდი ათასი შვიდი ასი სამოცდაათი შვიდი"
        )
        self.assertEqual(num2words(8000, lang="ka"), "რვა ათასი")
        self.assertEqual(num2words(8888, lang="ka"), "რვა ათასი რვა ასი ოთხმოცი რვა")
        self.assertEqual(num2words(9000, lang="ka"), "ცხრა ათასი")
        self.assertEqual(
            num2words(9999, lang="ka"), "ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა"
        )
        self.assertEqual(num2words(10000, lang="ka"), "ათი ათასი")
        self.assertEqual(num2words(10001, lang="ka"), "ათი ათასი ერთი")
        self.assertEqual(
            num2words(11111, lang="ka"), "თერთმეტი ათასი ერთი ასი თერთმეტი"
        )
        self.assertEqual(
            num2words(12345, lang="ka"), "თორმეტი ათასი სამი ასი ორმოცი ხუთი"
        )
        self.assertEqual(num2words(20000, lang="ka"), "ოცი ათასი")
        self.assertEqual(num2words(50000, lang="ka"), "ორმოცდაათი ათასი")
        self.assertEqual(
            num2words(99999, lang="ka"),
            "ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(100000, lang="ka"), "ერთი ასი ათასი")
        self.assertEqual(
            num2words(123456, lang="ka"),
            "ერთი ასი ოცი სამი ათასი ოთხი ასი ორმოცდაათი ექვსი",
        )
        self.assertEqual(num2words(200000, lang="ka"), "ორი ასი ათასი")
        self.assertEqual(num2words(500000, lang="ka"), "ხუთი ასი ათასი")
        self.assertEqual(
            num2words(654321, lang="ka"),
            "ექვსი ასი ორმოცდაათი ოთხი ათასი სამი ასი ოცი ერთი",
        )
        self.assertEqual(
            num2words(999999, lang="ka"),
            "ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ka"), "ერთი მილიონი")
        self.assertEqual(num2words(1000001, lang="ka"), "ერთი მილიონი ერთი")
        self.assertEqual(
            num2words(1111111, lang="ka"),
            "ერთი მილიონი ერთი ასი თერთმეტი ათასი ერთი ასი თერთმეტი",
        )
        self.assertEqual(
            num2words(1234567, lang="ka"),
            "ერთი მილიონი ორი ასი ოცდაათი ოთხი ათასი ხუთი ასი სამოცი შვიდი",
        )
        self.assertEqual(num2words(2000000, lang="ka"), "ორი მილიონი")
        self.assertEqual(num2words(5000000, lang="ka"), "ხუთი მილიონი")
        self.assertEqual(
            num2words(9999999, lang="ka"),
            "ცხრა მილიონი ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(10000000, lang="ka"), "ათი მილიონი")
        self.assertEqual(
            num2words(12345678, lang="ka"),
            "თორმეტი მილიონი სამი ასი ორმოცი ხუთი ათასი ექვსი ასი სამოცდაათი რვა",
        )
        self.assertEqual(
            num2words(99999999, lang="ka"),
            "ოთხმოცდაათი ცხრა მილიონი ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(100000000, lang="ka"), "ერთი ასი მილიონი")
        self.assertEqual(
            num2words(123456789, lang="ka"),
            "ერთი ასი ოცი სამი მილიონი ოთხი ასი ორმოცდაათი ექვსი ათასი შვიდი ასი ოთხმოცი ცხრა",
        )
        self.assertEqual(
            num2words(999999999, lang="ka"),
            "ცხრა ასი ოთხმოცდაათი ცხრა მილიონი ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(1000000000, lang="ka"), "ერთი მილიარდი")
        self.assertEqual(
            num2words(1234567890, lang="ka"),
            "ერთი მილიარდი ორი ასი ოცდაათი ოთხი მილიონი ხუთი ასი სამოცი შვიდი ათასი რვა ასი ოთხმოცდაათი",
        )
        self.assertEqual(
            num2words(9999999999, lang="ka"),
            "ცხრა მილიარდი ცხრა ასი ოთხმოცდაათი ცხრა მილიონი ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(10000000000, lang="ka"), "ათი მილიარდი")
        self.assertEqual(
            num2words(99999999999, lang="ka"),
            "ოთხმოცდაათი ცხრა მილიარდი ცხრა ასი ოთხმოცდაათი ცხრა მილიონი ცხრა ასი ოთხმოცდაათი ცხრა ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(num2words(10**12, lang="ka"), "ერთი ტრილიონი")
        self.assertEqual(num2words(2 * 10**12 + 5, lang="ka"), "ორი ტრილიონი ხუთი")
        self.assertEqual(num2words(10**15, lang="ka"), "ერთი კვადრილიონი")
        with self.assertRaises(OverflowError):
            num2words(10**18, lang="ka")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ka"), "მინუს ერთი")
        self.assertEqual(num2words(-2, lang="ka"), "მინუს ორი")
        self.assertEqual(num2words(-5, lang="ka"), "მინუს ხუთი")
        self.assertEqual(num2words(-10, lang="ka"), "მინუს ათი")
        self.assertEqual(num2words(-11, lang="ka"), "მინუს თერთმეტი")
        self.assertEqual(num2words(-20, lang="ka"), "მინუს ოცი")
        self.assertEqual(num2words(-50, lang="ka"), "მინუს ორმოცდაათი")
        self.assertEqual(num2words(-99, lang="ka"), "მინუს ოთხმოცდაათი ცხრა")
        self.assertEqual(num2words(-100, lang="ka"), "მინუს ერთი ასი")
        self.assertEqual(num2words(-101, lang="ka"), "მინუს ერთი ასი ერთი")
        self.assertEqual(num2words(-200, lang="ka"), "მინუს ორი ასი")
        self.assertEqual(num2words(-999, lang="ka"), "მინუს ცხრა ასი ოთხმოცდაათი ცხრა")
        self.assertEqual(num2words(-1000, lang="ka"), "მინუს ერთი ათასი")
        self.assertEqual(num2words(-1001, lang="ka"), "მინუს ერთი ათასი ერთი")
        self.assertEqual(num2words(-10000, lang="ka"), "მინუს ათი ათასი")
        self.assertEqual(num2words(-100000, lang="ka"), "მინუს ერთი ასი ათასი")
        self.assertEqual(num2words(-1000000, lang="ka"), "მინუს ერთი მილიონი")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ka"), "ნული წერტილი ერთი")
        self.assertEqual(num2words(0.5, lang="ka"), "ნული წერტილი ხუთი")
        self.assertEqual(num2words(0.9, lang="ka"), "ნული წერტილი ცხრა")
        self.assertEqual(num2words(1.1, lang="ka"), "ერთი წერტილი ერთი")
        self.assertEqual(num2words(1.5, lang="ka"), "ერთი წერტილი ხუთი")
        self.assertEqual(num2words(2.5, lang="ka"), "ორი წერტილი ხუთი")
        self.assertEqual(num2words(3.14, lang="ka"), "სამი წერტილი ერთი ოთხი")
        self.assertEqual(num2words(10.5, lang="ka"), "ათი წერტილი ხუთი")
        self.assertEqual(num2words(11.11, lang="ka"), "თერთმეტი წერტილი ერთი ერთი")
        self.assertEqual(num2words(20.2, lang="ka"), "ოცი წერტილი ორი")
        self.assertEqual(
            num2words(99.99, lang="ka"), "ოთხმოცდაათი ცხრა წერტილი ცხრა ცხრა"
        )
        self.assertEqual(num2words(100.01, lang="ka"), "ერთი ასი წერტილი ნული ერთი")
        self.assertEqual(num2words(100.5, lang="ka"), "ერთი ასი წერტილი ხუთი")
        self.assertEqual(
            num2words(123.45, lang="ka"), "ერთი ასი ოცი სამი წერტილი ოთხი ხუთი"
        )
        self.assertEqual(num2words(1000.5, lang="ka"), "ერთი ათასი წერტილი ხუთი")
        self.assertEqual(
            num2words(1234.56, lang="ka"),
            "ერთი ათასი ორი ასი ოცდაათი ოთხი წერტილი ხუთი ექვსი",
        )
        self.assertEqual(num2words(10000.01, lang="ka"), "ათი ათასი წერტილი ნული ერთი")
        self.assertEqual(num2words(-0.5, lang="ka"), "მინუს ნული წერტილი ხუთი")
        self.assertEqual(num2words(-1.5, lang="ka"), "მინუს ერთი წერტილი ხუთი")
        self.assertEqual(num2words(-10.5, lang="ka"), "მინუს ათი წერტილი ხუთი")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ka", ordinal=True), "პირველი")
        self.assertEqual(num2words(2, lang="ka", ordinal=True), "მეორე")
        self.assertEqual(num2words(3, lang="ka", ordinal=True), "მესამე")
        self.assertEqual(num2words(4, lang="ka", ordinal=True), "მეოთხე")
        self.assertEqual(num2words(5, lang="ka", ordinal=True), "მეხუთე")
        self.assertEqual(num2words(6, lang="ka", ordinal=True), "მეექვსე")
        self.assertEqual(num2words(7, lang="ka", ordinal=True), "მეშვიდე")
        self.assertEqual(num2words(8, lang="ka", ordinal=True), "მერვე")
        self.assertEqual(num2words(9, lang="ka", ordinal=True), "მეცხრე")
        self.assertEqual(num2words(10, lang="ka", ordinal=True), "მეათე")
        self.assertEqual(num2words(11, lang="ka", ordinal=True), "მე-თერთმეტი")
        self.assertEqual(num2words(12, lang="ka", ordinal=True), "მე-თორმეტი")
        self.assertEqual(num2words(13, lang="ka", ordinal=True), "მე-ცამეტი")
        self.assertEqual(num2words(14, lang="ka", ordinal=True), "მე-თოთხმეტი")
        self.assertEqual(num2words(15, lang="ka", ordinal=True), "მე-თხუთმეტი")
        self.assertEqual(num2words(16, lang="ka", ordinal=True), "მე-თექვსმეტი")
        self.assertEqual(num2words(17, lang="ka", ordinal=True), "მე-ჩვიდმეტი")
        self.assertEqual(num2words(18, lang="ka", ordinal=True), "მე-თვრამეტი")
        self.assertEqual(num2words(19, lang="ka", ordinal=True), "მე-ცხრამეტი")
        self.assertEqual(num2words(20, lang="ka", ordinal=True), "მე-ოცი")
        self.assertEqual(num2words(21, lang="ka", ordinal=True), "მე-ოცი ერთი")
        self.assertEqual(num2words(22, lang="ka", ordinal=True), "მე-ოცი ორი")
        self.assertEqual(num2words(25, lang="ka", ordinal=True), "მე-ოცი ხუთი")
        self.assertEqual(num2words(30, lang="ka", ordinal=True), "მე-ოცდაათი")
        self.assertEqual(num2words(40, lang="ka", ordinal=True), "მე-ორმოცი")
        self.assertEqual(num2words(50, lang="ka", ordinal=True), "მე-ორმოცდაათი")
        self.assertEqual(num2words(60, lang="ka", ordinal=True), "მე-სამოცი")
        self.assertEqual(num2words(70, lang="ka", ordinal=True), "მე-სამოცდაათი")
        self.assertEqual(num2words(80, lang="ka", ordinal=True), "მე-ოთხმოცი")
        self.assertEqual(num2words(90, lang="ka", ordinal=True), "მე-ოთხმოცდაათი")
        self.assertEqual(num2words(100, lang="ka", ordinal=True), "მე-ერთი ასი")
        self.assertEqual(num2words(101, lang="ka", ordinal=True), "მე-ერთი ასი ერთი")
        self.assertEqual(num2words(200, lang="ka", ordinal=True), "მე-ორი ასი")
        self.assertEqual(num2words(500, lang="ka", ordinal=True), "მე-ხუთი ასი")
        self.assertEqual(num2words(1000, lang="ka", ordinal=True), "მე-ერთი ათასი")
        self.assertEqual(num2words(1001, lang="ka", ordinal=True), "მე-ერთი ათასი ერთი")
        self.assertEqual(num2words(10000, lang="ka", ordinal=True), "მე-ათი ათასი")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ka", to="currency", currency="GEL"), "ნული ლარი"
        )
        self.assertEqual(
            num2words(0.01, lang="ka", to="currency", currency="GEL"),
            "ნული ლარი ერთი თეთრი",
        )
        self.assertEqual(
            num2words(0.5, lang="ka", to="currency", currency="GEL"),
            "ნული ლარი ორმოცდაათი თეთრი",
        )
        self.assertEqual(
            num2words(1, lang="ka", to="currency", currency="GEL"), "ერთი ლარი"
        )
        self.assertEqual(
            num2words(1.5, lang="ka", to="currency", currency="GEL"),
            "ერთი ლარი ორმოცდაათი თეთრი",
        )
        self.assertEqual(
            num2words(0, lang="ka", to="currency", currency="USD"), "ნული დოლარი"
        )
        self.assertEqual(
            num2words(0.01, lang="ka", to="currency", currency="USD"),
            "ნული დოლარი ერთი ცენტი",
        )
        self.assertEqual(
            num2words(0.5, lang="ka", to="currency", currency="USD"),
            "ნული დოლარი ორმოცდაათი ცენტი",
        )
        self.assertEqual(
            num2words(1, lang="ka", to="currency", currency="USD"), "ერთი დოლარი"
        )
        self.assertEqual(
            num2words(1.5, lang="ka", to="currency", currency="USD"),
            "ერთი დოლარი ორმოცდაათი ცენტი",
        )
        self.assertEqual(
            num2words(0, lang="ka", to="currency", currency="EUR"), "ნული ევრო"
        )
        self.assertEqual(
            num2words(0.01, lang="ka", to="currency", currency="EUR"),
            "ნული ევრო ერთი ცენტი",
        )
        self.assertEqual(
            num2words(0.5, lang="ka", to="currency", currency="EUR"),
            "ნული ევრო ორმოცდაათი ცენტი",
        )
        self.assertEqual(
            num2words(1, lang="ka", to="currency", currency="EUR"), "ერთი ევრო"
        )
        self.assertEqual(
            num2words(1.5, lang="ka", to="currency", currency="EUR"),
            "ერთი ევრო ორმოცდაათი ცენტი",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ka", to="year"), "ერთი ათასი")
        self.assertEqual(
            num2words(1066, lang="ka", to="year"), "ერთი ათასი სამოცი ექვსი"
        )
        self.assertEqual(
            num2words(1492, lang="ka", to="year"), "ერთი ათასი ოთხი ასი ოთხმოცდაათი ორი"
        )
        self.assertEqual(
            num2words(1776, lang="ka", to="year"),
            "ერთი ათასი შვიდი ასი სამოცდაათი ექვსი",
        )
        self.assertEqual(num2words(1800, lang="ka", to="year"), "ერთი ათასი რვა ასი")
        self.assertEqual(num2words(1900, lang="ka", to="year"), "ერთი ათასი ცხრა ასი")
        self.assertEqual(
            num2words(1984, lang="ka", to="year"), "ერთი ათასი ცხრა ასი ოთხმოცი ოთხი"
        )
        self.assertEqual(
            num2words(1999, lang="ka", to="year"),
            "ერთი ათასი ცხრა ასი ოთხმოცდაათი ცხრა",
        )
        self.assertEqual(num2words(2000, lang="ka", to="year"), "ორი ათასი")
        self.assertEqual(num2words(2001, lang="ka", to="year"), "ორი ათასი ერთი")
        self.assertEqual(num2words(2010, lang="ka", to="year"), "ორი ათასი ათი")
        self.assertEqual(num2words(2020, lang="ka", to="year"), "ორი ათასი ოცი")
        self.assertEqual(num2words(2024, lang="ka", to="year"), "ორი ათასი ოცი ოთხი")
        self.assertEqual(num2words(2100, lang="ka", to="year"), "ორი ათასი ერთი ასი")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ka"), "ნული")
        self.assertEqual(num2words("1", lang="ka"), "ერთი")
        self.assertEqual(num2words("10", lang="ka"), "ათი")
        self.assertEqual(num2words("100", lang="ka"), "ერთი ასი")
        self.assertEqual(num2words("1000", lang="ka"), "ერთი ათასი")
        self.assertEqual(num2words("10000", lang="ka"), "ათი ათასი")
        self.assertEqual(num2words("100000", lang="ka"), "ერთი ასი ათასი")
        self.assertEqual(num2words("1000000", lang="ka"), "ერთი მილიონი")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ka"), "ნული")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ka"), num2words("100", lang="ka"))
        self.assertEqual(num2words(1000, lang="ka"), num2words("1000", lang="ka"))
