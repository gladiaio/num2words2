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


class Num2WordsHITest(TestCase):
    """Comprehensive test cases for Hindi language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="hi"), "शून्य")
        self.assertEqual(num2words(1, lang="hi"), "एक")
        self.assertEqual(num2words(2, lang="hi"), "दो")
        self.assertEqual(num2words(3, lang="hi"), "तीन")
        self.assertEqual(num2words(4, lang="hi"), "चार")
        self.assertEqual(num2words(5, lang="hi"), "पाँच")
        self.assertEqual(num2words(6, lang="hi"), "छः")
        self.assertEqual(num2words(7, lang="hi"), "सात")
        self.assertEqual(num2words(8, lang="hi"), "आठ")
        self.assertEqual(num2words(9, lang="hi"), "नौ")
        self.assertEqual(num2words(10, lang="hi"), "दस")
        self.assertEqual(num2words(11, lang="hi"), "ग्यारह")
        self.assertEqual(num2words(12, lang="hi"), "बारह")
        self.assertEqual(num2words(13, lang="hi"), "तेरह")
        self.assertEqual(num2words(14, lang="hi"), "चौदह")
        self.assertEqual(num2words(15, lang="hi"), "पंद्रह")
        self.assertEqual(num2words(16, lang="hi"), "सोलह")
        self.assertEqual(num2words(17, lang="hi"), "सत्रह")
        self.assertEqual(num2words(18, lang="hi"), "अट्ठारह")
        self.assertEqual(num2words(19, lang="hi"), "उन्नीस")
        self.assertEqual(num2words(20, lang="hi"), "बीस")
        self.assertEqual(num2words(21, lang="hi"), "इक्कीस")
        self.assertEqual(num2words(22, lang="hi"), "बाईस")
        self.assertEqual(num2words(23, lang="hi"), "तेईस")
        self.assertEqual(num2words(24, lang="hi"), "चौबीस")
        self.assertEqual(num2words(25, lang="hi"), "पच्चीस")
        self.assertEqual(num2words(26, lang="hi"), "छब्बीस")
        self.assertEqual(num2words(27, lang="hi"), "सत्ताईस")
        self.assertEqual(num2words(28, lang="hi"), "अट्ठाईस")
        self.assertEqual(num2words(29, lang="hi"), "उनतीस")
        self.assertEqual(num2words(30, lang="hi"), "तीस")
        self.assertEqual(num2words(31, lang="hi"), "इकत्तीस")
        self.assertEqual(num2words(35, lang="hi"), "पैंतीस")
        self.assertEqual(num2words(40, lang="hi"), "चालीस")
        self.assertEqual(num2words(45, lang="hi"), "पैंतालीस")
        self.assertEqual(num2words(50, lang="hi"), "पचास")
        self.assertEqual(num2words(55, lang="hi"), "पचपन")
        self.assertEqual(num2words(60, lang="hi"), "साठ")
        self.assertEqual(num2words(65, lang="hi"), "पैंसठ")
        self.assertEqual(num2words(70, lang="hi"), "सत्तर")
        self.assertEqual(num2words(75, lang="hi"), "पचहत्तर")
        self.assertEqual(num2words(80, lang="hi"), "अस्सी")
        self.assertEqual(num2words(85, lang="hi"), "पचासी")
        self.assertEqual(num2words(90, lang="hi"), "नब्बे")
        self.assertEqual(num2words(95, lang="hi"), "पचानवे")
        self.assertEqual(num2words(99, lang="hi"), "निन्यानवे")
        self.assertEqual(num2words(100, lang="hi"), "सौ")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="hi"), "सौ एक")
        self.assertEqual(num2words(110, lang="hi"), "सौ दस")
        self.assertEqual(num2words(111, lang="hi"), "सौ ग्यारह")
        self.assertEqual(num2words(120, lang="hi"), "सौ बीस")
        self.assertEqual(num2words(125, lang="hi"), "सौ पच्चीस")
        self.assertEqual(num2words(150, lang="hi"), "सौ पचास")
        self.assertEqual(num2words(175, lang="hi"), "सौ पचहत्तर")
        self.assertEqual(num2words(199, lang="hi"), "सौ निन्यानवे")
        self.assertEqual(num2words(200, lang="hi"), "दो सौ")
        self.assertEqual(num2words(201, lang="hi"), "दो सौ एक")
        self.assertEqual(num2words(210, lang="hi"), "दो सौ दस")
        self.assertEqual(num2words(220, lang="hi"), "दो सौ बीस")
        self.assertEqual(num2words(250, lang="hi"), "दो सौ पचास")
        self.assertEqual(num2words(299, lang="hi"), "दो सौ निन्यानवे")
        self.assertEqual(num2words(300, lang="hi"), "तीन सौ")
        self.assertEqual(num2words(333, lang="hi"), "तीन सौ तैंतीस")
        self.assertEqual(num2words(400, lang="hi"), "चार सौ")
        self.assertEqual(num2words(444, lang="hi"), "चार सौ चौवालीस")
        self.assertEqual(num2words(500, lang="hi"), "पाँच सौ")
        self.assertEqual(num2words(555, lang="hi"), "पाँच सौ पचपन")
        self.assertEqual(num2words(600, lang="hi"), "छः सौ")
        self.assertEqual(num2words(666, lang="hi"), "छः सौ छियासठ")
        self.assertEqual(num2words(700, lang="hi"), "सात सौ")
        self.assertEqual(num2words(777, lang="hi"), "सात सौ सतहत्तर")
        self.assertEqual(num2words(800, lang="hi"), "आठ सौ")
        self.assertEqual(num2words(888, lang="hi"), "आठ सौ अट्ठासी")
        self.assertEqual(num2words(900, lang="hi"), "नौ सौ")
        self.assertEqual(num2words(999, lang="hi"), "नौ सौ निन्यानवे")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="hi"), "एक हज़ार")
        self.assertEqual(num2words(1001, lang="hi"), "एक हज़ार एक")
        self.assertEqual(num2words(1010, lang="hi"), "एक हज़ार दस")
        self.assertEqual(num2words(1100, lang="hi"), "एक हज़ार सौ")
        self.assertEqual(num2words(1111, lang="hi"), "एक हज़ार सौ ग्यारह")
        self.assertEqual(num2words(1234, lang="hi"), "एक हज़ार दो सौ चौंतीस")
        self.assertEqual(num2words(1500, lang="hi"), "एक हज़ार पाँच सौ")
        self.assertEqual(num2words(1999, lang="hi"), "एक हज़ार नौ सौ निन्यानवे")
        self.assertEqual(num2words(2000, lang="hi"), "दो हज़ार")
        self.assertEqual(num2words(2001, lang="hi"), "दो हज़ार एक")
        self.assertEqual(num2words(2020, lang="hi"), "दो हज़ार बीस")
        self.assertEqual(num2words(2222, lang="hi"), "दो हज़ार दो सौ बाईस")
        self.assertEqual(num2words(3000, lang="hi"), "तीन हज़ार")
        self.assertEqual(num2words(3333, lang="hi"), "तीन हज़ार तीन सौ तैंतीस")
        self.assertEqual(num2words(4000, lang="hi"), "चार हज़ार")
        self.assertEqual(num2words(4444, lang="hi"), "चार हज़ार चार सौ चौवालीस")
        self.assertEqual(num2words(5000, lang="hi"), "पाँच हज़ार")
        self.assertEqual(num2words(5555, lang="hi"), "पाँच हज़ार पाँच सौ पचपन")
        self.assertEqual(num2words(6000, lang="hi"), "छः हज़ार")
        self.assertEqual(num2words(6666, lang="hi"), "छः हज़ार छः सौ छियासठ")
        self.assertEqual(num2words(7000, lang="hi"), "सात हज़ार")
        self.assertEqual(num2words(7777, lang="hi"), "सात हज़ार सात सौ सतहत्तर")
        self.assertEqual(num2words(8000, lang="hi"), "आठ हज़ार")
        self.assertEqual(num2words(8888, lang="hi"), "आठ हज़ार आठ सौ अट्ठासी")
        self.assertEqual(num2words(9000, lang="hi"), "नौ हज़ार")
        self.assertEqual(num2words(9999, lang="hi"), "नौ हज़ार नौ सौ निन्यानवे")
        self.assertEqual(num2words(10000, lang="hi"), "दस हज़ार")
        self.assertEqual(num2words(10001, lang="hi"), "दस हज़ार एक")
        self.assertEqual(num2words(11111, lang="hi"), "ग्यारह हज़ार सौ ग्यारह")
        self.assertEqual(num2words(12345, lang="hi"), "बारह हज़ार तीन सौ पैंतालीस")
        self.assertEqual(num2words(20000, lang="hi"), "बीस हज़ार")
        self.assertEqual(num2words(50000, lang="hi"), "पचास हज़ार")
        self.assertEqual(num2words(99999, lang="hi"), "निन्यानवे हज़ार नौ सौ निन्यानवे")
        self.assertEqual(num2words(100000, lang="hi"), "लाख")
        self.assertEqual(num2words(123456, lang="hi"), "लाख तेईस हज़ार चार सौ छप्पन")
        self.assertEqual(num2words(200000, lang="hi"), "दो लाख")
        self.assertEqual(num2words(500000, lang="hi"), "पाँच लाख")
        self.assertEqual(
            num2words(654321, lang="hi"), "छः लाख चौवन हज़ार तीन सौ इक्कीस"
        )
        self.assertEqual(
            num2words(999999, lang="hi"), "नौ लाख निन्यानवे हज़ार नौ सौ निन्यानवे"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="hi"), "दस लाख")
        self.assertEqual(num2words(1000001, lang="hi"), "दस लाख एक")
        self.assertEqual(
            num2words(1111111, lang="hi"), "ग्यारह लाख ग्यारह हज़ार सौ ग्यारह"
        )
        self.assertEqual(
            num2words(1234567, lang="hi"), "बारह लाख चौंतीस हज़ार पाँच सौ सड़सठ"
        )
        self.assertEqual(num2words(2000000, lang="hi"), "बीस लाख")
        self.assertEqual(num2words(5000000, lang="hi"), "पचास लाख")
        self.assertEqual(
            num2words(9999999, lang="hi"),
            "निन्यानवे लाख निन्यानवे हज़ार नौ सौ निन्यानवे",
        )
        self.assertEqual(num2words(10000000, lang="hi"), "करोड़")
        self.assertEqual(
            num2words(12345678, lang="hi"),
            "करोड़ तेईस लाख पैंतालीस हज़ार छः सौ अठहत्तर",
        )
        self.assertEqual(
            num2words(99999999, lang="hi"),
            "नौ करोड़ निन्यानवे लाख निन्यानवे हज़ार नौ सौ निन्यानवे",
        )
        self.assertEqual(num2words(100000000, lang="hi"), "दस करोड़")
        self.assertEqual(
            num2words(123456789, lang="hi"),
            "बारह करोड़ चौंतीस लाख छप्पन हज़ार सात सौ नवासी",
        )
        self.assertEqual(
            num2words(999999999, lang="hi"),
            "निन्यानवे करोड़ निन्यानवे लाख निन्यानवे हज़ार नौ सौ निन्यानवे",
        )
        self.assertEqual(num2words(1000000000, lang="hi"), "एक अरब")
        self.assertEqual(
            num2words(1234567890, lang="hi"),
            "एक अरब तेईस करोड़ पैंतालीस लाख सड़सठ हज़ार आठ सौ नब्बे",
        )
        self.assertEqual(
            num2words(9999999999, lang="hi"),
            "नौ अरब निन्यानवे करोड़ निन्यानवे लाख निन्यानवे हज़ार नौ सौ निन्यानवे",
        )
        self.assertEqual(num2words(10000000000, lang="hi"), "दस अरब")
        self.assertEqual(
            num2words(99999999999, lang="hi"),
            "निन्यानवे अरब निन्यानवे करोड़ निन्यानवे लाख निन्यानवे हज़ार नौ सौ निन्यानवे",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="hi"), "माइनस एक")
        self.assertEqual(num2words(-2, lang="hi"), "माइनस दो")
        self.assertEqual(num2words(-5, lang="hi"), "माइनस पाँच")
        self.assertEqual(num2words(-10, lang="hi"), "माइनस दस")
        self.assertEqual(num2words(-11, lang="hi"), "माइनस ग्यारह")
        self.assertEqual(num2words(-20, lang="hi"), "माइनस बीस")
        self.assertEqual(num2words(-50, lang="hi"), "माइनस पचास")
        self.assertEqual(num2words(-99, lang="hi"), "माइनस निन्यानवे")
        self.assertEqual(num2words(-100, lang="hi"), "माइनस सौ")
        self.assertEqual(num2words(-101, lang="hi"), "माइनस सौ एक")
        self.assertEqual(num2words(-200, lang="hi"), "माइनस दो सौ")
        self.assertEqual(num2words(-999, lang="hi"), "माइनस नौ सौ निन्यानवे")
        self.assertEqual(num2words(-1000, lang="hi"), "माइनस एक हज़ार")
        self.assertEqual(num2words(-1001, lang="hi"), "माइनस एक हज़ार एक")
        self.assertEqual(num2words(-10000, lang="hi"), "माइनस दस हज़ार")
        self.assertEqual(num2words(-100000, lang="hi"), "माइनस लाख")
        self.assertEqual(num2words(-1000000, lang="hi"), "माइनस दस लाख")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="hi"), "शून्य दशमलव एक")
        self.assertEqual(num2words(0.5, lang="hi"), "शून्य दशमलव पाँच")
        self.assertEqual(num2words(0.9, lang="hi"), "शून्य दशमलव नौ")
        self.assertEqual(num2words(1.1, lang="hi"), "एक दशमलव एक")
        self.assertEqual(num2words(1.5, lang="hi"), "एक दशमलव पाँच")
        self.assertEqual(num2words(2.5, lang="hi"), "दो दशमलव पाँच")
        self.assertEqual(num2words(3.14, lang="hi"), "तीन दशमलव एक चार")
        self.assertEqual(num2words(10.5, lang="hi"), "दस दशमलव पाँच")
        self.assertEqual(num2words(11.11, lang="hi"), "ग्यारह दशमलव एक एक")
        self.assertEqual(num2words(20.2, lang="hi"), "बीस दशमलव दो")
        self.assertEqual(num2words(99.99, lang="hi"), "निन्यानवे दशमलव नौ नौ")
        self.assertEqual(num2words(100.01, lang="hi"), "सौ दशमलव शून्य एक")
        self.assertEqual(num2words(100.5, lang="hi"), "सौ दशमलव पाँच")
        self.assertEqual(num2words(123.45, lang="hi"), "सौ तेईस दशमलव चार पाँच")
        self.assertEqual(num2words(1000.5, lang="hi"), "एक हज़ार दशमलव पाँच")
        self.assertEqual(
            num2words(1234.56, lang="hi"), "एक हज़ार दो सौ चौंतीस दशमलव पाँच छः"
        )
        self.assertEqual(num2words(10000.01, lang="hi"), "दस हज़ार दशमलव शून्य एक")
        self.assertEqual(num2words(-0.5, lang="hi"), "माइनस शून्य दशमलव पाँच")
        self.assertEqual(num2words(-1.5, lang="hi"), "माइनस एक दशमलव पाँच")
        self.assertEqual(num2words(-10.5, lang="hi"), "माइनस दस दशमलव पाँच")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="hi", ordinal=True), "पहला")
        self.assertEqual(num2words(2, lang="hi", ordinal=True), "दूसरा")
        self.assertEqual(num2words(3, lang="hi", ordinal=True), "तीसरा")
        self.assertEqual(num2words(4, lang="hi", ordinal=True), "चौथा")
        self.assertEqual(num2words(5, lang="hi", ordinal=True), "पाँचवाँ")
        self.assertEqual(num2words(6, lang="hi", ordinal=True), "छठा")
        self.assertEqual(num2words(7, lang="hi", ordinal=True), "सातवाँ")
        self.assertEqual(num2words(8, lang="hi", ordinal=True), "आठवाँ")
        self.assertEqual(num2words(9, lang="hi", ordinal=True), "नौवाँ")
        self.assertEqual(num2words(10, lang="hi", ordinal=True), "दसवाँ")
        self.assertEqual(num2words(11, lang="hi", ordinal=True), "ग्यारहवाँ")
        self.assertEqual(num2words(12, lang="hi", ordinal=True), "बारहवाँ")
        self.assertEqual(num2words(13, lang="hi", ordinal=True), "तेरहवाँ")
        self.assertEqual(num2words(14, lang="hi", ordinal=True), "चौदहवाँ")
        self.assertEqual(num2words(15, lang="hi", ordinal=True), "पंद्रहवाँ")
        self.assertEqual(num2words(16, lang="hi", ordinal=True), "सोलहवाँ")
        self.assertEqual(num2words(17, lang="hi", ordinal=True), "सत्रहवाँ")
        self.assertEqual(num2words(18, lang="hi", ordinal=True), "अट्ठारहवाँ")
        self.assertEqual(num2words(19, lang="hi", ordinal=True), "उन्नीसवाँ")
        self.assertEqual(num2words(20, lang="hi", ordinal=True), "बीसवाँ")
        self.assertEqual(num2words(21, lang="hi", ordinal=True), "इक्कीसवाँ")
        self.assertEqual(num2words(22, lang="hi", ordinal=True), "बाईसवाँ")
        self.assertEqual(num2words(25, lang="hi", ordinal=True), "पच्चीसवाँ")
        self.assertEqual(num2words(30, lang="hi", ordinal=True), "तीसवाँ")
        self.assertEqual(num2words(40, lang="hi", ordinal=True), "चालीसवाँ")
        self.assertEqual(num2words(50, lang="hi", ordinal=True), "पचासवाँ")
        self.assertEqual(num2words(60, lang="hi", ordinal=True), "साठवाँ")
        self.assertEqual(num2words(70, lang="hi", ordinal=True), "सत्तरवाँ")
        self.assertEqual(num2words(80, lang="hi", ordinal=True), "अस्सीवाँ")
        self.assertEqual(num2words(90, lang="hi", ordinal=True), "नब्बेवाँ")
        self.assertEqual(num2words(100, lang="hi", ordinal=True), "सौवाँ")
        self.assertEqual(num2words(101, lang="hi", ordinal=True), "सौ एकवाँ")
        self.assertEqual(num2words(200, lang="hi", ordinal=True), "दो सौवाँ")
        self.assertEqual(num2words(500, lang="hi", ordinal=True), "पाँच सौवाँ")
        self.assertEqual(num2words(1000, lang="hi", ordinal=True), "एक हज़ारवाँ")
        self.assertEqual(num2words(1001, lang="hi", ordinal=True), "एक हज़ार एकवाँ")
        self.assertEqual(num2words(10000, lang="hi", ordinal=True), "दस हज़ारवाँ")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="hi", to="currency", currency="INR"), "शून्य रुपये"
        )
        self.assertEqual(
            num2words(0.01, lang="hi", to="currency", currency="INR"),
            "शून्य रुपये, एक पैसा",
        )
        self.assertEqual(
            num2words(0.5, lang="hi", to="currency", currency="INR"),
            "शून्य रुपये, पचास पैसे",
        )
        self.assertEqual(
            num2words(1, lang="hi", to="currency", currency="INR"), "एक रुपया"
        )
        self.assertEqual(
            num2words(1.5, lang="hi", to="currency", currency="INR"),
            "एक रुपया, पचास पैसे",
        )
        self.assertEqual(
            num2words(0, lang="hi", to="currency", currency="USD"), "शून्य डॉलर"
        )
        self.assertEqual(
            num2words(0.01, lang="hi", to="currency", currency="USD"),
            "शून्य डॉलर, एक सेंट",
        )
        self.assertEqual(
            num2words(0.5, lang="hi", to="currency", currency="USD"),
            "शून्य डॉलर, पचास सेंट",
        )
        self.assertEqual(
            num2words(1, lang="hi", to="currency", currency="USD"), "एक डॉलर"
        )
        self.assertEqual(
            num2words(1.5, lang="hi", to="currency", currency="USD"),
            "एक डॉलर, पचास सेंट",
        )
        self.assertEqual(
            num2words(0, lang="hi", to="currency", currency="EUR"), "शून्य यूरो"
        )
        self.assertEqual(
            num2words(0.01, lang="hi", to="currency", currency="EUR"),
            "शून्य यूरो, एक सेंट",
        )
        self.assertEqual(
            num2words(0.5, lang="hi", to="currency", currency="EUR"),
            "शून्य यूरो, पचास सेंट",
        )
        self.assertEqual(
            num2words(1, lang="hi", to="currency", currency="EUR"), "एक यूरो"
        )
        self.assertEqual(
            num2words(1.5, lang="hi", to="currency", currency="EUR"),
            "एक यूरो, पचास सेंट",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="hi", to="year"), "एक हज़ार")
        self.assertEqual(num2words(1066, lang="hi", to="year"), "एक हज़ार छियासठ")
        self.assertEqual(num2words(1492, lang="hi", to="year"), "एक हज़ार चार सौ बानवे")
        self.assertEqual(
            num2words(1776, lang="hi", to="year"), "एक हज़ार सात सौ छिहत्तर"
        )
        self.assertEqual(num2words(1800, lang="hi", to="year"), "एक हज़ार आठ सौ")
        self.assertEqual(num2words(1900, lang="hi", to="year"), "एक हज़ार नौ सौ")
        self.assertEqual(num2words(1984, lang="hi", to="year"), "एक हज़ार नौ सौ चौरासी")
        self.assertEqual(
            num2words(1999, lang="hi", to="year"), "एक हज़ार नौ सौ निन्यानवे"
        )
        self.assertEqual(num2words(2000, lang="hi", to="year"), "दो हज़ार")
        self.assertEqual(num2words(2001, lang="hi", to="year"), "दो हज़ार एक")
        self.assertEqual(num2words(2010, lang="hi", to="year"), "दो हज़ार दस")
        self.assertEqual(num2words(2020, lang="hi", to="year"), "दो हज़ार बीस")
        self.assertEqual(num2words(2024, lang="hi", to="year"), "दो हज़ार चौबीस")
        self.assertEqual(num2words(2100, lang="hi", to="year"), "दो हज़ार सौ")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="hi"), "शून्य")
        self.assertEqual(num2words("1", lang="hi"), "एक")
        self.assertEqual(num2words("10", lang="hi"), "दस")
        self.assertEqual(num2words("100", lang="hi"), "सौ")
        self.assertEqual(num2words("1000", lang="hi"), "एक हज़ार")
        self.assertEqual(num2words("10000", lang="hi"), "दस हज़ार")
        self.assertEqual(num2words("100000", lang="hi"), "लाख")
        self.assertEqual(num2words("1000000", lang="hi"), "दस लाख")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="hi"), "शून्य")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="hi"), num2words("100", lang="hi"))
        self.assertEqual(num2words(1000, lang="hi"), num2words("1000", lang="hi"))



class TestHINegativeOrdinalNum(TestCase):
    """Ports savoirfairelinux/num2words#672 by @santhreal.

    ``Num2Word_HI.to_ordinal_num`` maps the value character by character
    through a digit table, so a negative died on the sign with
    ``KeyError: "'-'"`` and a fractional value on the point with
    ``KeyError: "'.'"``. ``verify_ordinal`` now runs first.
    """

    def test_ordinal_num_rejects_negative_and_float(self):
        with self.assertRaises(TypeError):
            num2words(-1, lang="hi", to="ordinal_num")
        with self.assertRaises(TypeError):
            num2words(-21, lang="hi", to="ordinal_num")
        with self.assertRaises(TypeError):
            num2words(1.5, lang="hi", to="ordinal_num")

    def test_word_ordinals_still_accept_negatives(self):
        # Upstream's own test for #672 asserts this stays as it was.
        self.assertEqual(num2words(-1, lang="hi", to="ordinal"), "माइनस एकवाँ")

    def test_non_negative_ordinal_num_unchanged(self):
        self.assertEqual(num2words(1, lang="hi", to="ordinal_num"), "१ला")
        self.assertEqual(num2words(21, lang="hi", to="ordinal_num"), "२१वाँ")
