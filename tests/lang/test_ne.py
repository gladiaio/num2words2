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


class Num2WordsNETest(TestCase):
    """Comprehensive test cases for Nepali language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="ne"), "शून्य")
        self.assertEqual(num2words(1, lang="ne"), "एक")
        self.assertEqual(num2words(2, lang="ne"), "दुई")
        self.assertEqual(num2words(3, lang="ne"), "तीन")
        self.assertEqual(num2words(4, lang="ne"), "चार")
        self.assertEqual(num2words(5, lang="ne"), "पाँच")
        self.assertEqual(num2words(6, lang="ne"), "छ")
        self.assertEqual(num2words(7, lang="ne"), "सात")
        self.assertEqual(num2words(8, lang="ne"), "आठ")
        self.assertEqual(num2words(9, lang="ne"), "नौ")
        self.assertEqual(num2words(10, lang="ne"), "दस")
        self.assertEqual(num2words(11, lang="ne"), "एघार")
        self.assertEqual(num2words(12, lang="ne"), "बाह्र")
        self.assertEqual(num2words(13, lang="ne"), "तेह्र")
        self.assertEqual(num2words(14, lang="ne"), "चौध")
        self.assertEqual(num2words(15, lang="ne"), "पन्ध्र")
        self.assertEqual(num2words(16, lang="ne"), "सोह्र")
        self.assertEqual(num2words(17, lang="ne"), "सत्र")
        self.assertEqual(num2words(18, lang="ne"), "अठार")
        self.assertEqual(num2words(19, lang="ne"), "उन्नाइस")
        self.assertEqual(num2words(20, lang="ne"), "बीस")
        self.assertEqual(num2words(21, lang="ne"), "एक्काइस")
        self.assertEqual(num2words(22, lang="ne"), "बाइस")
        self.assertEqual(num2words(23, lang="ne"), "तेइस")
        self.assertEqual(num2words(24, lang="ne"), "चौबीस")
        self.assertEqual(num2words(25, lang="ne"), "पच्चीस")
        self.assertEqual(num2words(26, lang="ne"), "छब्बीस")
        self.assertEqual(num2words(27, lang="ne"), "सत्ताइस")
        self.assertEqual(num2words(28, lang="ne"), "अट्ठाइस")
        self.assertEqual(num2words(29, lang="ne"), "उनन्तीस")
        self.assertEqual(num2words(30, lang="ne"), "तीस")
        self.assertEqual(num2words(31, lang="ne"), "एकतीस")
        self.assertEqual(num2words(35, lang="ne"), "पैंतीस")
        self.assertEqual(num2words(40, lang="ne"), "चालीस")
        self.assertEqual(num2words(45, lang="ne"), "पैंतालीस")
        self.assertEqual(num2words(50, lang="ne"), "पचास")
        self.assertEqual(num2words(55, lang="ne"), "पचपन्न")
        self.assertEqual(num2words(60, lang="ne"), "साठी")
        self.assertEqual(num2words(65, lang="ne"), "पैंसट्ठी")
        self.assertEqual(num2words(70, lang="ne"), "सत्तरी")
        self.assertEqual(num2words(75, lang="ne"), "पचहत्तर")
        self.assertEqual(num2words(80, lang="ne"), "असी")
        self.assertEqual(num2words(85, lang="ne"), "पचासी")
        self.assertEqual(num2words(90, lang="ne"), "नब्बे")
        self.assertEqual(num2words(95, lang="ne"), "पन्चानब्बे")
        self.assertEqual(num2words(99, lang="ne"), "उनान्सय")
        self.assertEqual(num2words(100, lang="ne"), "एक सय")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="ne"), "एक सय एक")
        self.assertEqual(num2words(110, lang="ne"), "एक सय दस")
        self.assertEqual(num2words(111, lang="ne"), "एक सय एघार")
        self.assertEqual(num2words(120, lang="ne"), "एक सय बीस")
        self.assertEqual(num2words(125, lang="ne"), "एक सय पच्चीस")
        self.assertEqual(num2words(150, lang="ne"), "एक सय पचास")
        self.assertEqual(num2words(175, lang="ne"), "एक सय पचहत्तर")
        self.assertEqual(num2words(199, lang="ne"), "एक सय उनान्सय")
        self.assertEqual(num2words(200, lang="ne"), "दुई सय")
        self.assertEqual(num2words(201, lang="ne"), "दुई सय एक")
        self.assertEqual(num2words(210, lang="ne"), "दुई सय दस")
        self.assertEqual(num2words(220, lang="ne"), "दुई सय बीस")
        self.assertEqual(num2words(250, lang="ne"), "दुई सय पचास")
        self.assertEqual(num2words(299, lang="ne"), "दुई सय उनान्सय")
        self.assertEqual(num2words(300, lang="ne"), "तीन सय")
        self.assertEqual(num2words(333, lang="ne"), "तीन सय तेत्तीस")
        self.assertEqual(num2words(400, lang="ne"), "चार सय")
        self.assertEqual(num2words(444, lang="ne"), "चार सय चवालीस")
        self.assertEqual(num2words(500, lang="ne"), "पाँच सय")
        self.assertEqual(num2words(555, lang="ne"), "पाँच सय पचपन्न")
        self.assertEqual(num2words(600, lang="ne"), "छ सय")
        self.assertEqual(num2words(666, lang="ne"), "छ सय छयसट्ठी")
        self.assertEqual(num2words(700, lang="ne"), "सात सय")
        self.assertEqual(num2words(777, lang="ne"), "सात सय सतहत्तर")
        self.assertEqual(num2words(800, lang="ne"), "आठ सय")
        self.assertEqual(num2words(888, lang="ne"), "आठ सय अठासी")
        self.assertEqual(num2words(900, lang="ne"), "नौ सय")
        self.assertEqual(num2words(999, lang="ne"), "नौ सय उनान्सय")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="ne"), "एक हजार")
        self.assertEqual(num2words(1001, lang="ne"), "एक हजार एक")
        self.assertEqual(num2words(1010, lang="ne"), "एक हजार दस")
        self.assertEqual(num2words(1100, lang="ne"), "एक हजार एक सय")
        self.assertEqual(num2words(1111, lang="ne"), "एक हजार एक सय एघार")
        self.assertEqual(num2words(1234, lang="ne"), "एक हजार दुई सय चौंतीस")
        self.assertEqual(num2words(1500, lang="ne"), "एक हजार पाँच सय")
        self.assertEqual(num2words(1999, lang="ne"), "एक हजार नौ सय उनान्सय")
        self.assertEqual(num2words(2000, lang="ne"), "दुई हजार")
        self.assertEqual(num2words(2001, lang="ne"), "दुई हजार एक")
        self.assertEqual(num2words(2020, lang="ne"), "दुई हजार बीस")
        self.assertEqual(num2words(2222, lang="ne"), "दुई हजार दुई सय बाइस")
        self.assertEqual(num2words(3000, lang="ne"), "तीन हजार")
        self.assertEqual(num2words(3333, lang="ne"), "तीन हजार तीन सय तेत्तीस")
        self.assertEqual(num2words(4000, lang="ne"), "चार हजार")
        self.assertEqual(num2words(4444, lang="ne"), "चार हजार चार सय चवालीस")
        self.assertEqual(num2words(5000, lang="ne"), "पाँच हजार")
        self.assertEqual(num2words(5555, lang="ne"), "पाँच हजार पाँच सय पचपन्न")
        self.assertEqual(num2words(6000, lang="ne"), "छ हजार")
        self.assertEqual(num2words(6666, lang="ne"), "छ हजार छ सय छयसट्ठी")
        self.assertEqual(num2words(7000, lang="ne"), "सात हजार")
        self.assertEqual(num2words(7777, lang="ne"), "सात हजार सात सय सतहत्तर")
        self.assertEqual(num2words(8000, lang="ne"), "आठ हजार")
        self.assertEqual(num2words(8888, lang="ne"), "आठ हजार आठ सय अठासी")
        self.assertEqual(num2words(9000, lang="ne"), "नौ हजार")
        self.assertEqual(num2words(9999, lang="ne"), "नौ हजार नौ सय उनान्सय")
        self.assertEqual(num2words(10000, lang="ne"), "दस हजार")
        self.assertEqual(num2words(10001, lang="ne"), "दस हजार एक")
        self.assertEqual(num2words(11111, lang="ne"), "एघार हजार एक सय एघार")
        self.assertEqual(num2words(12345, lang="ne"), "बाह्र हजार तीन सय पैंतालीस")
        self.assertEqual(num2words(20000, lang="ne"), "बीस हजार")
        self.assertEqual(num2words(50000, lang="ne"), "पचास हजार")
        self.assertEqual(num2words(99999, lang="ne"), "उनान्सय हजार नौ सय उनान्सय")
        self.assertEqual(num2words(100000, lang="ne"), "एक लाख")
        self.assertEqual(num2words(123456, lang="ne"), "एक लाख तेइस हजार चार सय छपन्न")
        self.assertEqual(num2words(200000, lang="ne"), "दुई लाख")
        self.assertEqual(num2words(500000, lang="ne"), "पाँच लाख")
        self.assertEqual(
            num2words(654321, lang="ne"), "छ लाख चउन्न हजार तीन सय एक्काइस"
        )
        self.assertEqual(
            num2words(999999, lang="ne"), "नौ लाख उनान्सय हजार नौ सय उनान्सय"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="ne"), "दस लाख")
        self.assertEqual(num2words(1000001, lang="ne"), "दस लाख एक")
        self.assertEqual(num2words(1111111, lang="ne"), "एघार लाख एघार हजार एक सय एघार")
        self.assertEqual(
            num2words(1234567, lang="ne"), "बाह्र लाख चौंतीस हजार पाँच सय सतसट्ठी"
        )
        self.assertEqual(num2words(2000000, lang="ne"), "बीस लाख")
        self.assertEqual(num2words(5000000, lang="ne"), "पचास लाख")
        self.assertEqual(
            num2words(9999999, lang="ne"), "उनान्सय लाख उनान्सय हजार नौ सय उनान्सय"
        )
        self.assertEqual(num2words(10000000, lang="ne"), "एक करोड")
        self.assertEqual(
            num2words(12345678, lang="ne"),
            "एक करोड तेइस लाख पैंतालीस हजार छ सय अठहत्तर",
        )
        self.assertEqual(
            num2words(99999999, lang="ne"),
            "नौ करोड उनान्सय लाख उनान्सय हजार नौ सय उनान्सय",
        )
        self.assertEqual(num2words(100000000, lang="ne"), "दस करोड")
        self.assertEqual(
            num2words(123456789, lang="ne"),
            "बाह्र करोड चौंतीस लाख छपन्न हजार सात सय उनानब्बे",
        )
        self.assertEqual(
            num2words(999999999, lang="ne"),
            "उनान्सय करोड उनान्सय लाख उनान्सय हजार नौ सय उनान्सय",
        )
        self.assertEqual(num2words(1000000000, lang="ne"), "एक अर्ब")
        self.assertEqual(
            num2words(1234567890, lang="ne"),
            "एक अर्ब तेइस करोड पैंतालीस लाख सतसट्ठी हजार आठ सय नब्बे",
        )
        self.assertEqual(
            num2words(9999999999, lang="ne"),
            "नौ अर्ब उनान्सय करोड उनान्सय लाख उनान्सय हजार नौ सय उनान्सय",
        )
        self.assertEqual(num2words(10000000000, lang="ne"), "दस अर्ब")
        self.assertEqual(
            num2words(99999999999, lang="ne"),
            "उनान्सय अर्ब उनान्सय करोड उनान्सय लाख उनान्सय हजार नौ सय उनान्सय",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="ne"), "ऋण एक")
        self.assertEqual(num2words(-2, lang="ne"), "ऋण दुई")
        self.assertEqual(num2words(-5, lang="ne"), "ऋण पाँच")
        self.assertEqual(num2words(-10, lang="ne"), "ऋण दस")
        self.assertEqual(num2words(-11, lang="ne"), "ऋण एघार")
        self.assertEqual(num2words(-20, lang="ne"), "ऋण बीस")
        self.assertEqual(num2words(-50, lang="ne"), "ऋण पचास")
        self.assertEqual(num2words(-99, lang="ne"), "ऋण उनान्सय")
        self.assertEqual(num2words(-100, lang="ne"), "ऋण एक सय")
        self.assertEqual(num2words(-101, lang="ne"), "ऋण एक सय एक")
        self.assertEqual(num2words(-200, lang="ne"), "ऋण दुई सय")
        self.assertEqual(num2words(-999, lang="ne"), "ऋण नौ सय उनान्सय")
        self.assertEqual(num2words(-1000, lang="ne"), "ऋण एक हजार")
        self.assertEqual(num2words(-1001, lang="ne"), "ऋण एक हजार एक")
        self.assertEqual(num2words(-10000, lang="ne"), "ऋण दस हजार")
        self.assertEqual(num2words(-100000, lang="ne"), "ऋण एक लाख")
        self.assertEqual(num2words(-1000000, lang="ne"), "ऋण दस लाख")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="ne"), "शून्य दशमलव एक")
        self.assertEqual(num2words(0.5, lang="ne"), "शून्य दशमलव पाँच")
        self.assertEqual(num2words(0.9, lang="ne"), "शून्य दशमलव नौ")
        self.assertEqual(num2words(1.1, lang="ne"), "एक दशमलव एक")
        self.assertEqual(num2words(1.5, lang="ne"), "एक दशमलव पाँच")
        self.assertEqual(num2words(2.5, lang="ne"), "दुई दशमलव पाँच")
        self.assertEqual(num2words(3.14, lang="ne"), "तीन दशमलव एक चार")
        self.assertEqual(num2words(10.5, lang="ne"), "दस दशमलव पाँच")
        self.assertEqual(num2words(11.11, lang="ne"), "एघार दशमलव एक एक")
        self.assertEqual(num2words(20.2, lang="ne"), "बीस दशमलव दुई")
        self.assertEqual(num2words(99.99, lang="ne"), "उनान्सय दशमलव नौ नौ")
        self.assertEqual(num2words(100.01, lang="ne"), "एक सय दशमलव शून्य एक")
        self.assertEqual(num2words(100.5, lang="ne"), "एक सय दशमलव पाँच")
        self.assertEqual(num2words(123.45, lang="ne"), "एक सय तेइस दशमलव चार पाँच")
        self.assertEqual(num2words(1000.5, lang="ne"), "एक हजार दशमलव पाँच")
        self.assertEqual(
            num2words(1234.56, lang="ne"), "एक हजार दुई सय चौंतीस दशमलव पाँच छ"
        )
        self.assertEqual(num2words(10000.01, lang="ne"), "दस हजार दशमलव शून्य एक")
        self.assertEqual(num2words(-0.5, lang="ne"), "ऋण शून्य दशमलव पाँच")
        self.assertEqual(num2words(-1.5, lang="ne"), "ऋण एक दशमलव पाँच")
        self.assertEqual(num2words(-10.5, lang="ne"), "ऋण दस दशमलव पाँच")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="ne", ordinal=True), "पहिलो")
        self.assertEqual(num2words(2, lang="ne", ordinal=True), "दोस्रो")
        self.assertEqual(num2words(3, lang="ne", ordinal=True), "तेस्रो")
        self.assertEqual(num2words(4, lang="ne", ordinal=True), "चौथो")
        self.assertEqual(num2words(5, lang="ne", ordinal=True), "पाँचऔं")
        self.assertEqual(num2words(6, lang="ne", ordinal=True), "छऔं")
        self.assertEqual(num2words(7, lang="ne", ordinal=True), "सातऔं")
        self.assertEqual(num2words(8, lang="ne", ordinal=True), "आठऔं")
        self.assertEqual(num2words(9, lang="ne", ordinal=True), "नौऔं")
        self.assertEqual(num2words(10, lang="ne", ordinal=True), "दसऔं")
        self.assertEqual(num2words(11, lang="ne", ordinal=True), "एघारऔं")
        self.assertEqual(num2words(12, lang="ne", ordinal=True), "बाह्रऔं")
        self.assertEqual(num2words(13, lang="ne", ordinal=True), "तेह्रऔं")
        self.assertEqual(num2words(14, lang="ne", ordinal=True), "चौधऔं")
        self.assertEqual(num2words(15, lang="ne", ordinal=True), "पन्ध्रऔं")
        self.assertEqual(num2words(16, lang="ne", ordinal=True), "सोह्रऔं")
        self.assertEqual(num2words(17, lang="ne", ordinal=True), "सत्रऔं")
        self.assertEqual(num2words(18, lang="ne", ordinal=True), "अठारऔं")
        self.assertEqual(num2words(19, lang="ne", ordinal=True), "उन्नाइसऔं")
        self.assertEqual(num2words(20, lang="ne", ordinal=True), "बीसऔं")
        self.assertEqual(num2words(21, lang="ne", ordinal=True), "एक्काइसऔं")
        self.assertEqual(num2words(22, lang="ne", ordinal=True), "बाइसऔं")
        self.assertEqual(num2words(25, lang="ne", ordinal=True), "पच्चीसऔं")
        self.assertEqual(num2words(30, lang="ne", ordinal=True), "तीसऔं")
        self.assertEqual(num2words(40, lang="ne", ordinal=True), "चालीसऔं")
        self.assertEqual(num2words(50, lang="ne", ordinal=True), "पचासऔं")
        self.assertEqual(num2words(60, lang="ne", ordinal=True), "साठीऔं")
        self.assertEqual(num2words(70, lang="ne", ordinal=True), "सत्तरीऔं")
        self.assertEqual(num2words(80, lang="ne", ordinal=True), "असीऔं")
        self.assertEqual(num2words(90, lang="ne", ordinal=True), "नब्बेऔं")
        self.assertEqual(num2words(100, lang="ne", ordinal=True), "एक सयऔं")
        self.assertEqual(num2words(101, lang="ne", ordinal=True), "एक सय एकऔं")
        self.assertEqual(num2words(200, lang="ne", ordinal=True), "दुई सयऔं")
        self.assertEqual(num2words(500, lang="ne", ordinal=True), "पाँच सयऔं")
        self.assertEqual(num2words(1000, lang="ne", ordinal=True), "एक हजारऔं")
        self.assertEqual(num2words(1001, lang="ne", ordinal=True), "एक हजार एकऔं")
        self.assertEqual(num2words(10000, lang="ne", ordinal=True), "दस हजारऔं")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="ne", to="currency", currency="NPR"), "शून्य रुपैयाँ"
        )
        self.assertEqual(
            num2words(0.01, lang="ne", to="currency", currency="NPR"),
            "शून्य रुपैयाँ एक पैसा",
        )
        self.assertEqual(
            num2words(0.5, lang="ne", to="currency", currency="NPR"),
            "शून्य रुपैयाँ पचास पैसा",
        )
        self.assertEqual(
            num2words(1, lang="ne", to="currency", currency="NPR"), "एक रुपैयाँ"
        )
        self.assertEqual(
            num2words(1.5, lang="ne", to="currency", currency="NPR"),
            "एक रुपैयाँ पचास पैसा",
        )
        self.assertEqual(
            num2words(0, lang="ne", to="currency", currency="USD"), "शून्य डलर"
        )
        self.assertEqual(
            num2words(0.01, lang="ne", to="currency", currency="USD"),
            "शून्य डलर एक सेन्ट",
        )
        self.assertEqual(
            num2words(0.5, lang="ne", to="currency", currency="USD"),
            "शून्य डलर पचास सेन्ट",
        )
        self.assertEqual(
            num2words(1, lang="ne", to="currency", currency="USD"), "एक डलर"
        )
        self.assertEqual(
            num2words(1.5, lang="ne", to="currency", currency="USD"),
            "एक डलर पचास सेन्ट",
        )
        self.assertEqual(
            num2words(0, lang="ne", to="currency", currency="EUR"), "शून्य युरो"
        )
        self.assertEqual(
            num2words(0.01, lang="ne", to="currency", currency="EUR"),
            "शून्य युरो एक सेन्ट",
        )
        self.assertEqual(
            num2words(0.5, lang="ne", to="currency", currency="EUR"),
            "शून्य युरो पचास सेन्ट",
        )
        self.assertEqual(
            num2words(1, lang="ne", to="currency", currency="EUR"), "एक युरो"
        )
        self.assertEqual(
            num2words(1.5, lang="ne", to="currency", currency="EUR"),
            "एक युरो पचास सेन्ट",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="ne", to="year"), "एक हजार")
        self.assertEqual(num2words(1066, lang="ne", to="year"), "एक हजार छयसट्ठी")
        self.assertEqual(
            num2words(1492, lang="ne", to="year"), "एक हजार चार सय बयानब्बे"
        )
        self.assertEqual(
            num2words(1776, lang="ne", to="year"), "एक हजार सात सय छयहत्तर"
        )
        self.assertEqual(num2words(1800, lang="ne", to="year"), "एक हजार आठ सय")
        self.assertEqual(num2words(1900, lang="ne", to="year"), "एक हजार नौ सय")
        self.assertEqual(num2words(1984, lang="ne", to="year"), "एक हजार नौ सय चौरासी")
        self.assertEqual(num2words(1999, lang="ne", to="year"), "एक हजार नौ सय उनान्सय")
        self.assertEqual(num2words(2000, lang="ne", to="year"), "दुई हजार")
        self.assertEqual(num2words(2001, lang="ne", to="year"), "दुई हजार एक")
        self.assertEqual(num2words(2010, lang="ne", to="year"), "दुई हजार दस")
        self.assertEqual(num2words(2020, lang="ne", to="year"), "दुई हजार बीस")
        self.assertEqual(num2words(2024, lang="ne", to="year"), "दुई हजार चौबीस")
        self.assertEqual(num2words(2100, lang="ne", to="year"), "दुई हजार एक सय")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="ne"), "शून्य")
        self.assertEqual(num2words("1", lang="ne"), "एक")
        self.assertEqual(num2words("10", lang="ne"), "दस")
        self.assertEqual(num2words("100", lang="ne"), "एक सय")
        self.assertEqual(num2words("1000", lang="ne"), "एक हजार")
        self.assertEqual(num2words("10000", lang="ne"), "दस हजार")
        self.assertEqual(num2words("100000", lang="ne"), "एक लाख")
        self.assertEqual(num2words("1000000", lang="ne"), "दस लाख")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="ne"), "शून्य")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="ne"), num2words("100", lang="ne"))
        self.assertEqual(num2words(1000, lang="ne"), num2words("1000", lang="ne"))
