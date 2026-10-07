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


class Num2WordsMRTest(TestCase):
    """Comprehensive test cases for Marathi language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="mr"), "शून्य")
        self.assertEqual(num2words(1, lang="mr"), "एक")
        self.assertEqual(num2words(2, lang="mr"), "दोन")
        self.assertEqual(num2words(3, lang="mr"), "तीन")
        self.assertEqual(num2words(4, lang="mr"), "चार")
        self.assertEqual(num2words(5, lang="mr"), "पाच")
        self.assertEqual(num2words(6, lang="mr"), "सहा")
        self.assertEqual(num2words(7, lang="mr"), "सात")
        self.assertEqual(num2words(8, lang="mr"), "आठ")
        self.assertEqual(num2words(9, lang="mr"), "नऊ")
        self.assertEqual(num2words(10, lang="mr"), "दहा")
        self.assertEqual(num2words(11, lang="mr"), "अकरा")
        self.assertEqual(num2words(12, lang="mr"), "बारा")
        self.assertEqual(num2words(13, lang="mr"), "तेरा")
        self.assertEqual(num2words(14, lang="mr"), "चौदा")
        self.assertEqual(num2words(15, lang="mr"), "पंधरा")
        self.assertEqual(num2words(16, lang="mr"), "सोळा")
        self.assertEqual(num2words(17, lang="mr"), "सतरा")
        self.assertEqual(num2words(18, lang="mr"), "अठरा")
        self.assertEqual(num2words(19, lang="mr"), "एकोणीस")
        self.assertEqual(num2words(20, lang="mr"), "वीस")
        self.assertEqual(num2words(21, lang="mr"), "एकवीस")
        self.assertEqual(num2words(22, lang="mr"), "बावीस")
        self.assertEqual(num2words(23, lang="mr"), "तेवीस")
        self.assertEqual(num2words(24, lang="mr"), "चोवीस")
        self.assertEqual(num2words(25, lang="mr"), "पंचवीस")
        self.assertEqual(num2words(26, lang="mr"), "सव्वीस")
        self.assertEqual(num2words(27, lang="mr"), "सत्तावीस")
        self.assertEqual(num2words(28, lang="mr"), "अठ्ठावीस")
        self.assertEqual(num2words(29, lang="mr"), "एकोणतीस")
        self.assertEqual(num2words(30, lang="mr"), "तीस")
        self.assertEqual(num2words(31, lang="mr"), "एकतीस")
        self.assertEqual(num2words(35, lang="mr"), "पस्तीस")
        self.assertEqual(num2words(40, lang="mr"), "चाळीस")
        self.assertEqual(num2words(45, lang="mr"), "पंचेचाळीस")
        self.assertEqual(num2words(50, lang="mr"), "पन्नास")
        self.assertEqual(num2words(55, lang="mr"), "पंचावन्न")
        self.assertEqual(num2words(60, lang="mr"), "साठ")
        self.assertEqual(num2words(65, lang="mr"), "पासष्ट")
        self.assertEqual(num2words(70, lang="mr"), "सत्तर")
        self.assertEqual(num2words(75, lang="mr"), "पंच्याहत्तर")
        self.assertEqual(num2words(80, lang="mr"), "ऐंशी")
        self.assertEqual(num2words(85, lang="mr"), "पंच्याऐंशी")
        self.assertEqual(num2words(90, lang="mr"), "नव्वद")
        self.assertEqual(num2words(95, lang="mr"), "पंच्याण्णव")
        self.assertEqual(num2words(99, lang="mr"), "नव्याण्णव")
        self.assertEqual(num2words(100, lang="mr"), "एकशे")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="mr"), "एकशे एक")
        self.assertEqual(num2words(110, lang="mr"), "एकशे दहा")
        self.assertEqual(num2words(111, lang="mr"), "एकशे अकरा")
        self.assertEqual(num2words(120, lang="mr"), "एकशे वीस")
        self.assertEqual(num2words(125, lang="mr"), "एकशे पंचवीस")
        self.assertEqual(num2words(150, lang="mr"), "एकशे पन्नास")
        self.assertEqual(num2words(175, lang="mr"), "एकशे पंच्याहत्तर")
        self.assertEqual(num2words(199, lang="mr"), "एकशे नव्याण्णव")
        self.assertEqual(num2words(200, lang="mr"), "दोनशे")
        self.assertEqual(num2words(201, lang="mr"), "दोनशे एक")
        self.assertEqual(num2words(210, lang="mr"), "दोनशे दहा")
        self.assertEqual(num2words(220, lang="mr"), "दोनशे वीस")
        self.assertEqual(num2words(250, lang="mr"), "दोनशे पन्नास")
        self.assertEqual(num2words(299, lang="mr"), "दोनशे नव्याण्णव")
        self.assertEqual(num2words(300, lang="mr"), "तीनशे")
        self.assertEqual(num2words(333, lang="mr"), "तीनशे तेहतीस")
        self.assertEqual(num2words(400, lang="mr"), "चारशे")
        self.assertEqual(num2words(444, lang="mr"), "चारशे चव्वेचाळीस")
        self.assertEqual(num2words(500, lang="mr"), "पाचशे")
        self.assertEqual(num2words(555, lang="mr"), "पाचशे पंचावन्न")
        self.assertEqual(num2words(600, lang="mr"), "सहाशे")
        self.assertEqual(num2words(666, lang="mr"), "सहाशे सहासष्ट")
        self.assertEqual(num2words(700, lang="mr"), "सातशे")
        self.assertEqual(num2words(777, lang="mr"), "सातशे सत्याहत्तर")
        self.assertEqual(num2words(800, lang="mr"), "आठशे")
        self.assertEqual(num2words(888, lang="mr"), "आठशे अठ्ठ्याऐंशी")
        self.assertEqual(num2words(900, lang="mr"), "नऊशे")
        self.assertEqual(num2words(999, lang="mr"), "नऊशे नव्याण्णव")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="mr"), "एक हजार")
        self.assertEqual(num2words(1001, lang="mr"), "एक हजार एक")
        self.assertEqual(num2words(1010, lang="mr"), "एक हजार दहा")
        self.assertEqual(num2words(1100, lang="mr"), "एक हजार एकशे")
        self.assertEqual(num2words(1111, lang="mr"), "एक हजार एकशे अकरा")
        self.assertEqual(num2words(1234, lang="mr"), "एक हजार दोनशे चौतीस")
        self.assertEqual(num2words(1500, lang="mr"), "एक हजार पाचशे")
        self.assertEqual(num2words(1999, lang="mr"), "एक हजार नऊशे नव्याण्णव")
        self.assertEqual(num2words(2000, lang="mr"), "दोन हजार")
        self.assertEqual(num2words(2001, lang="mr"), "दोन हजार एक")
        self.assertEqual(num2words(2020, lang="mr"), "दोन हजार वीस")
        self.assertEqual(num2words(2222, lang="mr"), "दोन हजार दोनशे बावीस")
        self.assertEqual(num2words(3000, lang="mr"), "तीन हजार")
        self.assertEqual(num2words(3333, lang="mr"), "तीन हजार तीनशे तेहतीस")
        self.assertEqual(num2words(4000, lang="mr"), "चार हजार")
        self.assertEqual(num2words(4444, lang="mr"), "चार हजार चारशे चव्वेचाळीस")
        self.assertEqual(num2words(5000, lang="mr"), "पाच हजार")
        self.assertEqual(num2words(5555, lang="mr"), "पाच हजार पाचशे पंचावन्न")
        self.assertEqual(num2words(6000, lang="mr"), "सहा हजार")
        self.assertEqual(num2words(6666, lang="mr"), "सहा हजार सहाशे सहासष्ट")
        self.assertEqual(num2words(7000, lang="mr"), "सात हजार")
        self.assertEqual(num2words(7777, lang="mr"), "सात हजार सातशे सत्याहत्तर")
        self.assertEqual(num2words(8000, lang="mr"), "आठ हजार")
        self.assertEqual(num2words(8888, lang="mr"), "आठ हजार आठशे अठ्ठ्याऐंशी")
        self.assertEqual(num2words(9000, lang="mr"), "नऊ हजार")
        self.assertEqual(num2words(9999, lang="mr"), "नऊ हजार नऊशे नव्याण्णव")
        self.assertEqual(num2words(10000, lang="mr"), "दहा हजार")
        self.assertEqual(num2words(10001, lang="mr"), "दहा हजार एक")
        self.assertEqual(num2words(11111, lang="mr"), "अकरा हजार एकशे अकरा")
        self.assertEqual(num2words(12345, lang="mr"), "बारा हजार तीनशे पंचेचाळीस")
        self.assertEqual(num2words(20000, lang="mr"), "वीस हजार")
        self.assertEqual(num2words(50000, lang="mr"), "पन्नास हजार")
        self.assertEqual(num2words(99999, lang="mr"), "नव्याण्णव हजार नऊशे नव्याण्णव")
        self.assertEqual(num2words(100000, lang="mr"), "एक लाख")
        self.assertEqual(
            num2words(123456, lang="mr"), "एक लाख तेवीस हजार चारशे छप्पन्न"
        )
        self.assertEqual(num2words(200000, lang="mr"), "दोन लाख")
        self.assertEqual(num2words(500000, lang="mr"), "पाच लाख")
        self.assertEqual(
            num2words(654321, lang="mr"), "सहा लाख चोपन्न हजार तीनशे एकवीस"
        )
        self.assertEqual(
            num2words(999999, lang="mr"), "नऊ लाख नव्याण्णव हजार नऊशे नव्याण्णव"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="mr"), "दहा लाख")
        self.assertEqual(num2words(1000001, lang="mr"), "दहा लाख एक")
        self.assertEqual(num2words(1111111, lang="mr"), "अकरा लाख अकरा हजार एकशे अकरा")
        self.assertEqual(
            num2words(1234567, lang="mr"), "बारा लाख चौतीस हजार पाचशे सदुसष्ट"
        )
        self.assertEqual(num2words(2000000, lang="mr"), "वीस लाख")
        self.assertEqual(num2words(5000000, lang="mr"), "पन्नास लाख")
        self.assertEqual(
            num2words(9999999, lang="mr"), "नव्याण्णव लाख नव्याण्णव हजार नऊशे नव्याण्णव"
        )
        self.assertEqual(num2words(10000000, lang="mr"), "एक कोटी")
        self.assertEqual(
            num2words(12345678, lang="mr"),
            "एक कोटी तेवीस लाख पंचेचाळीस हजार सहाशे अठ्ठ्याहत्तर",
        )
        self.assertEqual(
            num2words(99999999, lang="mr"),
            "नऊ कोटी नव्याण्णव लाख नव्याण्णव हजार नऊशे नव्याण्णव",
        )
        self.assertEqual(num2words(100000000, lang="mr"), "दहा कोटी")
        self.assertEqual(
            num2words(123456789, lang="mr"),
            "बारा कोटी चौतीस लाख छप्पन्न हजार सातशे एकोणनव्वद",
        )
        self.assertEqual(
            num2words(999999999, lang="mr"),
            "नव्याण्णव कोटी नव्याण्णव लाख नव्याण्णव हजार नऊशे नव्याण्णव",
        )
        self.assertEqual(num2words(1000000000, lang="mr"), "एक अब्ज")
        self.assertEqual(
            num2words(1234567890, lang="mr"),
            "एक अब्ज तेवीस कोटी पंचेचाळीस लाख सदुसष्ट हजार आठशे नव्वद",
        )
        self.assertEqual(
            num2words(9999999999, lang="mr"),
            "नऊ अब्ज नव्याण्णव कोटी नव्याण्णव लाख नव्याण्णव हजार नऊशे नव्याण्णव",
        )
        self.assertEqual(num2words(10000000000, lang="mr"), "दहा अब्ज")
        self.assertEqual(
            num2words(99999999999, lang="mr"),
            "नव्याण्णव अब्ज नव्याण्णव कोटी नव्याण्णव लाख नव्याण्णव हजार नऊशे नव्याण्णव",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="mr"), "ऋण एक")
        self.assertEqual(num2words(-2, lang="mr"), "ऋण दोन")
        self.assertEqual(num2words(-5, lang="mr"), "ऋण पाच")
        self.assertEqual(num2words(-10, lang="mr"), "ऋण दहा")
        self.assertEqual(num2words(-11, lang="mr"), "ऋण अकरा")
        self.assertEqual(num2words(-20, lang="mr"), "ऋण वीस")
        self.assertEqual(num2words(-50, lang="mr"), "ऋण पन्नास")
        self.assertEqual(num2words(-99, lang="mr"), "ऋण नव्याण्णव")
        self.assertEqual(num2words(-100, lang="mr"), "ऋण एकशे")
        self.assertEqual(num2words(-101, lang="mr"), "ऋण एकशे एक")
        self.assertEqual(num2words(-200, lang="mr"), "ऋण दोनशे")
        self.assertEqual(num2words(-999, lang="mr"), "ऋण नऊशे नव्याण्णव")
        self.assertEqual(num2words(-1000, lang="mr"), "ऋण एक हजार")
        self.assertEqual(num2words(-1001, lang="mr"), "ऋण एक हजार एक")
        self.assertEqual(num2words(-10000, lang="mr"), "ऋण दहा हजार")
        self.assertEqual(num2words(-100000, lang="mr"), "ऋण एक लाख")
        self.assertEqual(num2words(-1000000, lang="mr"), "ऋण दहा लाख")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="mr"), "शून्य दशांश एक")
        self.assertEqual(num2words(0.5, lang="mr"), "शून्य दशांश पाच")
        self.assertEqual(num2words(0.9, lang="mr"), "शून्य दशांश नऊ")
        self.assertEqual(num2words(1.1, lang="mr"), "एक दशांश एक")
        self.assertEqual(num2words(1.5, lang="mr"), "एक दशांश पाच")
        self.assertEqual(num2words(2.5, lang="mr"), "दोन दशांश पाच")
        self.assertEqual(num2words(3.14, lang="mr"), "तीन दशांश एक चार")
        self.assertEqual(num2words(10.5, lang="mr"), "दहा दशांश पाच")
        self.assertEqual(num2words(11.11, lang="mr"), "अकरा दशांश एक एक")
        self.assertEqual(num2words(20.2, lang="mr"), "वीस दशांश दोन")
        self.assertEqual(num2words(99.99, lang="mr"), "नव्याण्णव दशांश नऊ नऊ")
        self.assertEqual(num2words(100.01, lang="mr"), "एकशे दशांश शून्य एक")
        self.assertEqual(num2words(100.5, lang="mr"), "एकशे दशांश पाच")
        self.assertEqual(num2words(123.45, lang="mr"), "एकशे तेवीस दशांश चार पाच")
        self.assertEqual(num2words(1000.5, lang="mr"), "एक हजार दशांश पाच")
        self.assertEqual(
            num2words(1234.56, lang="mr"), "एक हजार दोनशे चौतीस दशांश पाच सहा"
        )
        self.assertEqual(num2words(10000.01, lang="mr"), "दहा हजार दशांश शून्य एक")
        self.assertEqual(num2words(-0.5, lang="mr"), "ऋण शून्य दशांश पाच")
        self.assertEqual(num2words(-1.5, lang="mr"), "ऋण एक दशांश पाच")
        self.assertEqual(num2words(-10.5, lang="mr"), "ऋण दहा दशांश पाच")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="mr", ordinal=True), "पहिला")
        self.assertEqual(num2words(2, lang="mr", ordinal=True), "दुसरा")
        self.assertEqual(num2words(3, lang="mr", ordinal=True), "तिसरा")
        self.assertEqual(num2words(4, lang="mr", ordinal=True), "चौथा")
        self.assertEqual(num2words(5, lang="mr", ordinal=True), "पाचवा")
        self.assertEqual(num2words(6, lang="mr", ordinal=True), "सहावा")
        self.assertEqual(num2words(7, lang="mr", ordinal=True), "सातवा")
        self.assertEqual(num2words(8, lang="mr", ordinal=True), "आठवा")
        self.assertEqual(num2words(9, lang="mr", ordinal=True), "नववा")
        self.assertEqual(num2words(10, lang="mr", ordinal=True), "दहावा")
        self.assertEqual(num2words(11, lang="mr", ordinal=True), "अकरावा")
        self.assertEqual(num2words(12, lang="mr", ordinal=True), "बारावा")
        self.assertEqual(num2words(13, lang="mr", ordinal=True), "तेरावा")
        self.assertEqual(num2words(14, lang="mr", ordinal=True), "चौदावा")
        self.assertEqual(num2words(15, lang="mr", ordinal=True), "पंधरावा")
        self.assertEqual(num2words(16, lang="mr", ordinal=True), "सोळावा")
        self.assertEqual(num2words(17, lang="mr", ordinal=True), "सतरावा")
        self.assertEqual(num2words(18, lang="mr", ordinal=True), "अठरावा")
        self.assertEqual(num2words(19, lang="mr", ordinal=True), "एकोणीसवा")
        self.assertEqual(num2words(20, lang="mr", ordinal=True), "वीसवा")
        self.assertEqual(num2words(21, lang="mr", ordinal=True), "एकवीसवा")
        self.assertEqual(num2words(22, lang="mr", ordinal=True), "बावीसवा")
        self.assertEqual(num2words(25, lang="mr", ordinal=True), "पंचवीसवा")
        self.assertEqual(num2words(30, lang="mr", ordinal=True), "तीसवा")
        self.assertEqual(num2words(40, lang="mr", ordinal=True), "चाळीसवा")
        self.assertEqual(num2words(50, lang="mr", ordinal=True), "पन्नासवा")
        self.assertEqual(num2words(60, lang="mr", ordinal=True), "साठवा")
        self.assertEqual(num2words(70, lang="mr", ordinal=True), "सत्तरवा")
        self.assertEqual(num2words(80, lang="mr", ordinal=True), "ऐंशीवा")
        self.assertEqual(num2words(90, lang="mr", ordinal=True), "नव्वदवा")
        self.assertEqual(num2words(100, lang="mr", ordinal=True), "एकशेवा")
        self.assertEqual(num2words(101, lang="mr", ordinal=True), "एकशे एकवा")
        self.assertEqual(num2words(200, lang="mr", ordinal=True), "दोनशेवा")
        self.assertEqual(num2words(500, lang="mr", ordinal=True), "पाचशेवा")
        self.assertEqual(num2words(1000, lang="mr", ordinal=True), "एक हजारवा")
        self.assertEqual(num2words(1001, lang="mr", ordinal=True), "एक हजार एकवा")
        self.assertEqual(num2words(10000, lang="mr", ordinal=True), "दहा हजारवा")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="mr", to="currency", currency="INR"), "शून्य रुपये"
        )
        self.assertEqual(
            num2words(0.01, lang="mr", to="currency", currency="INR"),
            "शून्य रुपये आणि एक पैसा",
        )
        self.assertEqual(
            num2words(0.5, lang="mr", to="currency", currency="INR"),
            "शून्य रुपये आणि पन्नास पैसे",
        )
        self.assertEqual(
            num2words(1, lang="mr", to="currency", currency="INR"), "एक रुपया"
        )
        self.assertEqual(
            num2words(1.5, lang="mr", to="currency", currency="INR"),
            "एक रुपया आणि पन्नास पैसे",
        )
        self.assertEqual(
            num2words(0, lang="mr", to="currency", currency="USD"), "शून्य डॉलर"
        )
        self.assertEqual(
            num2words(0.01, lang="mr", to="currency", currency="USD"),
            "शून्य डॉलर आणि एक सेंट",
        )
        self.assertEqual(
            num2words(0.5, lang="mr", to="currency", currency="USD"),
            "शून्य डॉलर आणि पन्नास सेंट्स",
        )
        self.assertEqual(
            num2words(1, lang="mr", to="currency", currency="USD"), "एक डॉलर"
        )
        self.assertEqual(
            num2words(1.5, lang="mr", to="currency", currency="USD"),
            "एक डॉलर आणि पन्नास सेंट्स",
        )
        self.assertEqual(
            num2words(0, lang="mr", to="currency", currency="EUR"), "शून्य युरो"
        )
        self.assertEqual(
            num2words(0.01, lang="mr", to="currency", currency="EUR"),
            "शून्य युरो आणि एक सेंट",
        )
        self.assertEqual(
            num2words(0.5, lang="mr", to="currency", currency="EUR"),
            "शून्य युरो आणि पन्नास सेंट्स",
        )
        self.assertEqual(
            num2words(1, lang="mr", to="currency", currency="EUR"), "एक युरो"
        )
        self.assertEqual(
            num2words(1.5, lang="mr", to="currency", currency="EUR"),
            "एक युरो आणि पन्नास सेंट्स",
        )
        self.assertEqual(
            num2words(0, lang="mr", to="currency", currency="GBP"), "शून्य पाउंड"
        )
        self.assertEqual(
            num2words(0.01, lang="mr", to="currency", currency="GBP"),
            "शून्य पाउंड आणि एक पेन्स",
        )
        self.assertEqual(
            num2words(0.5, lang="mr", to="currency", currency="GBP"),
            "शून्य पाउंड आणि पन्नास पेन्स",
        )
        self.assertEqual(
            num2words(1, lang="mr", to="currency", currency="GBP"), "एक पाउंड"
        )
        self.assertEqual(
            num2words(1.5, lang="mr", to="currency", currency="GBP"),
            "एक पाउंड आणि पन्नास पेन्स",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="mr", to="year"), "सन एक हजार")
        self.assertEqual(num2words(1066, lang="mr", to="year"), "सन एक हजार सहासष्ट")
        self.assertEqual(
            num2words(1492, lang="mr", to="year"), "सन एक हजार चारशे ब्याण्णव"
        )
        self.assertEqual(
            num2words(1776, lang="mr", to="year"), "सन एक हजार सातशे शहात्तर"
        )
        self.assertEqual(num2words(1800, lang="mr", to="year"), "सन एक हजार आठशे")
        self.assertEqual(num2words(1900, lang="mr", to="year"), "सन एक हजार नऊशे")
        self.assertEqual(
            num2words(1984, lang="mr", to="year"), "सन एक हजार नऊशे चौऱ्याऐंशी"
        )
        self.assertEqual(
            num2words(1999, lang="mr", to="year"), "सन एक हजार नऊशे नव्याण्णव"
        )
        self.assertEqual(num2words(2000, lang="mr", to="year"), "सन दोन हजार")
        self.assertEqual(num2words(2001, lang="mr", to="year"), "सन दोन हजार एक")
        self.assertEqual(num2words(2010, lang="mr", to="year"), "सन दोन हजार दहा")
        self.assertEqual(num2words(2020, lang="mr", to="year"), "सन दोन हजार वीस")
        self.assertEqual(num2words(2024, lang="mr", to="year"), "सन दोन हजार चोवीस")
        self.assertEqual(num2words(2100, lang="mr", to="year"), "सन दोन हजार एकशे")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="mr"), "शून्य")
        self.assertEqual(num2words("1", lang="mr"), "एक")
        self.assertEqual(num2words("10", lang="mr"), "दहा")
        self.assertEqual(num2words("100", lang="mr"), "एकशे")
        self.assertEqual(num2words("1000", lang="mr"), "एक हजार")
        self.assertEqual(num2words("10000", lang="mr"), "दहा हजार")
        self.assertEqual(num2words("100000", lang="mr"), "एक लाख")
        self.assertEqual(num2words("1000000", lang="mr"), "दहा लाख")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="mr"), "शून्य")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="mr"), num2words("100", lang="mr"))
        self.assertEqual(num2words(1000, lang="mr"), num2words("1000", lang="mr"))
