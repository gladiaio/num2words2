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


class Num2WordsMTTest(TestCase):
    """Comprehensive test cases for Maltese language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="mt"), "żero")
        self.assertEqual(num2words(1, lang="mt"), "wieħed")
        self.assertEqual(num2words(2, lang="mt"), "tnejn")
        self.assertEqual(num2words(3, lang="mt"), "tlieta")
        self.assertEqual(num2words(4, lang="mt"), "erbgħa")
        self.assertEqual(num2words(5, lang="mt"), "ħamsa")
        self.assertEqual(num2words(6, lang="mt"), "sitta")
        self.assertEqual(num2words(7, lang="mt"), "sebgħa")
        self.assertEqual(num2words(8, lang="mt"), "tmienja")
        self.assertEqual(num2words(9, lang="mt"), "disgħa")
        self.assertEqual(num2words(10, lang="mt"), "għaxra")
        self.assertEqual(num2words(11, lang="mt"), "għaxra wieħed")
        self.assertEqual(num2words(12, lang="mt"), "għaxra tnejn")
        self.assertEqual(num2words(13, lang="mt"), "għaxra tlieta")
        self.assertEqual(num2words(14, lang="mt"), "għaxra erbgħa")
        self.assertEqual(num2words(15, lang="mt"), "għaxra ħamsa")
        self.assertEqual(num2words(16, lang="mt"), "għaxra sitta")
        self.assertEqual(num2words(17, lang="mt"), "għaxra sebgħa")
        self.assertEqual(num2words(18, lang="mt"), "għaxra tmienja")
        self.assertEqual(num2words(19, lang="mt"), "għaxra disgħa")
        self.assertEqual(num2words(20, lang="mt"), "għoxrin")
        self.assertEqual(num2words(21, lang="mt"), "għoxrin wieħed")
        self.assertEqual(num2words(22, lang="mt"), "għoxrin tnejn")
        self.assertEqual(num2words(23, lang="mt"), "għoxrin tlieta")
        self.assertEqual(num2words(24, lang="mt"), "għoxrin erbgħa")
        self.assertEqual(num2words(25, lang="mt"), "għoxrin ħamsa")
        self.assertEqual(num2words(26, lang="mt"), "għoxrin sitta")
        self.assertEqual(num2words(27, lang="mt"), "għoxrin sebgħa")
        self.assertEqual(num2words(28, lang="mt"), "għoxrin tmienja")
        self.assertEqual(num2words(29, lang="mt"), "għoxrin disgħa")
        self.assertEqual(num2words(30, lang="mt"), "tletin")
        self.assertEqual(num2words(31, lang="mt"), "tletin wieħed")
        self.assertEqual(num2words(35, lang="mt"), "tletin ħamsa")
        self.assertEqual(num2words(40, lang="mt"), "erbgħin")
        self.assertEqual(num2words(45, lang="mt"), "erbgħin ħamsa")
        self.assertEqual(num2words(50, lang="mt"), "ħamsin")
        self.assertEqual(num2words(55, lang="mt"), "ħamsin ħamsa")
        self.assertEqual(num2words(60, lang="mt"), "sittin")
        self.assertEqual(num2words(65, lang="mt"), "sittin ħamsa")
        self.assertEqual(num2words(70, lang="mt"), "sebgħin")
        self.assertEqual(num2words(75, lang="mt"), "sebgħin ħamsa")
        self.assertEqual(num2words(80, lang="mt"), "tmenin")
        self.assertEqual(num2words(85, lang="mt"), "tmenin ħamsa")
        self.assertEqual(num2words(90, lang="mt"), "disgħin")
        self.assertEqual(num2words(95, lang="mt"), "disgħin ħamsa")
        self.assertEqual(num2words(99, lang="mt"), "disgħin disgħa")
        self.assertEqual(num2words(100, lang="mt"), "wieħed mija")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="mt"), "wieħed mija wieħed")
        self.assertEqual(num2words(110, lang="mt"), "wieħed mija għaxra")
        self.assertEqual(num2words(111, lang="mt"), "wieħed mija għaxra wieħed")
        self.assertEqual(num2words(120, lang="mt"), "wieħed mija għoxrin")
        self.assertEqual(num2words(125, lang="mt"), "wieħed mija għoxrin ħamsa")
        self.assertEqual(num2words(150, lang="mt"), "wieħed mija ħamsin")
        self.assertEqual(num2words(175, lang="mt"), "wieħed mija sebgħin ħamsa")
        self.assertEqual(num2words(199, lang="mt"), "wieħed mija disgħin disgħa")
        self.assertEqual(num2words(200, lang="mt"), "tnejn mija")
        self.assertEqual(num2words(201, lang="mt"), "tnejn mija wieħed")
        self.assertEqual(num2words(210, lang="mt"), "tnejn mija għaxra")
        self.assertEqual(num2words(220, lang="mt"), "tnejn mija għoxrin")
        self.assertEqual(num2words(250, lang="mt"), "tnejn mija ħamsin")
        self.assertEqual(num2words(299, lang="mt"), "tnejn mija disgħin disgħa")
        self.assertEqual(num2words(300, lang="mt"), "tlieta mija")
        self.assertEqual(num2words(333, lang="mt"), "tlieta mija tletin tlieta")
        self.assertEqual(num2words(400, lang="mt"), "erbgħa mija")
        self.assertEqual(num2words(444, lang="mt"), "erbgħa mija erbgħin erbgħa")
        self.assertEqual(num2words(500, lang="mt"), "ħamsa mija")
        self.assertEqual(num2words(555, lang="mt"), "ħamsa mija ħamsin ħamsa")
        self.assertEqual(num2words(600, lang="mt"), "sitta mija")
        self.assertEqual(num2words(666, lang="mt"), "sitta mija sittin sitta")
        self.assertEqual(num2words(700, lang="mt"), "sebgħa mija")
        self.assertEqual(num2words(777, lang="mt"), "sebgħa mija sebgħin sebgħa")
        self.assertEqual(num2words(800, lang="mt"), "tmienja mija")
        self.assertEqual(num2words(888, lang="mt"), "tmienja mija tmenin tmienja")
        self.assertEqual(num2words(900, lang="mt"), "disgħa mija")
        self.assertEqual(num2words(999, lang="mt"), "disgħa mija disgħin disgħa")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="mt"), "wieħed elf")
        self.assertEqual(num2words(1001, lang="mt"), "wieħed elf wieħed")
        self.assertEqual(num2words(1010, lang="mt"), "wieħed elf għaxra")
        self.assertEqual(num2words(1100, lang="mt"), "wieħed elf wieħed mija")
        self.assertEqual(
            num2words(1111, lang="mt"), "wieħed elf wieħed mija għaxra wieħed"
        )
        self.assertEqual(
            num2words(1234, lang="mt"), "wieħed elf tnejn mija tletin erbgħa"
        )
        self.assertEqual(num2words(1500, lang="mt"), "wieħed elf ħamsa mija")
        self.assertEqual(
            num2words(1999, lang="mt"), "wieħed elf disgħa mija disgħin disgħa"
        )
        self.assertEqual(num2words(2000, lang="mt"), "tnejn elf")
        self.assertEqual(num2words(2001, lang="mt"), "tnejn elf wieħed")
        self.assertEqual(num2words(2020, lang="mt"), "tnejn elf għoxrin")
        self.assertEqual(
            num2words(2222, lang="mt"), "tnejn elf tnejn mija għoxrin tnejn"
        )
        self.assertEqual(num2words(3000, lang="mt"), "tlieta elf")
        self.assertEqual(
            num2words(3333, lang="mt"), "tlieta elf tlieta mija tletin tlieta"
        )
        self.assertEqual(num2words(4000, lang="mt"), "erbgħa elf")
        self.assertEqual(
            num2words(4444, lang="mt"), "erbgħa elf erbgħa mija erbgħin erbgħa"
        )
        self.assertEqual(num2words(5000, lang="mt"), "ħamsa elf")
        self.assertEqual(
            num2words(5555, lang="mt"), "ħamsa elf ħamsa mija ħamsin ħamsa"
        )
        self.assertEqual(num2words(6000, lang="mt"), "sitta elf")
        self.assertEqual(
            num2words(6666, lang="mt"), "sitta elf sitta mija sittin sitta"
        )
        self.assertEqual(num2words(7000, lang="mt"), "sebgħa elf")
        self.assertEqual(
            num2words(7777, lang="mt"), "sebgħa elf sebgħa mija sebgħin sebgħa"
        )
        self.assertEqual(num2words(8000, lang="mt"), "tmienja elf")
        self.assertEqual(
            num2words(8888, lang="mt"), "tmienja elf tmienja mija tmenin tmienja"
        )
        self.assertEqual(num2words(9000, lang="mt"), "disgħa elf")
        self.assertEqual(
            num2words(9999, lang="mt"), "disgħa elf disgħa mija disgħin disgħa"
        )
        self.assertEqual(num2words(10000, lang="mt"), "għaxra elf")
        self.assertEqual(num2words(10001, lang="mt"), "għaxra elf wieħed")
        self.assertEqual(
            num2words(11111, lang="mt"), "għaxra wieħed elf wieħed mija għaxra wieħed"
        )
        self.assertEqual(
            num2words(12345, lang="mt"), "għaxra tnejn elf tlieta mija erbgħin ħamsa"
        )
        self.assertEqual(num2words(20000, lang="mt"), "għoxrin elf")
        self.assertEqual(num2words(50000, lang="mt"), "ħamsin elf")
        self.assertEqual(
            num2words(99999, lang="mt"), "disgħin disgħa elf disgħa mija disgħin disgħa"
        )
        self.assertEqual(num2words(100000, lang="mt"), "wieħed mija elf")
        self.assertEqual(
            num2words(123456, lang="mt"),
            "wieħed mija għoxrin tlieta elf erbgħa mija ħamsin sitta",
        )
        self.assertEqual(num2words(200000, lang="mt"), "tnejn mija elf")
        self.assertEqual(num2words(500000, lang="mt"), "ħamsa mija elf")
        self.assertEqual(
            num2words(654321, lang="mt"),
            "sitta mija ħamsin erbgħa elf tlieta mija għoxrin wieħed",
        )
        self.assertEqual(
            num2words(999999, lang="mt"),
            "disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="mt"), "wieħed miljun")
        self.assertEqual(num2words(1000001, lang="mt"), "wieħed miljun wieħed")
        self.assertEqual(
            num2words(1111111, lang="mt"),
            "wieħed miljun wieħed mija għaxra wieħed elf wieħed mija għaxra wieħed",
        )
        self.assertEqual(
            num2words(1234567, lang="mt"),
            "wieħed miljun tnejn mija tletin erbgħa elf ħamsa mija sittin sebgħa",
        )
        self.assertEqual(num2words(2000000, lang="mt"), "tnejn miljun")
        self.assertEqual(num2words(5000000, lang="mt"), "ħamsa miljun")
        self.assertEqual(
            num2words(9999999, lang="mt"),
            "disgħa miljun disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa",
        )
        self.assertEqual(num2words(10000000, lang="mt"), "għaxra miljun")
        self.assertEqual(
            num2words(12345678, lang="mt"),
            "għaxra tnejn miljun tlieta mija erbgħin ħamsa elf sitta mija sebgħin tmienja",
        )
        self.assertEqual(
            num2words(99999999, lang="mt"),
            "disgħin disgħa miljun disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa",
        )
        self.assertEqual(num2words(100000000, lang="mt"), "wieħed mija miljun")
        self.assertEqual(
            num2words(123456789, lang="mt"),
            "wieħed mija għoxrin tlieta miljun erbgħa mija ħamsin sitta elf sebgħa mija tmenin disgħa",
        )
        self.assertEqual(
            num2words(999999999, lang="mt"),
            "disgħa mija disgħin disgħa miljun disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa",
        )
        self.assertEqual(num2words(1000000000, lang="mt"), "wieħed biljun")
        self.assertEqual(num2words(1234567890, lang="mt"), "wieħed biljun tnejn mija tletin erbgħa miljun ħamsa mija sittin sebgħa elf tmienja mija disgħin")
        self.assertEqual(num2words(9999999999, lang="mt"), "disgħa biljun disgħa mija disgħin disgħa miljun disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa")
        self.assertEqual(num2words(10000000000, lang="mt"), "għaxra biljun")
        self.assertEqual(num2words(99999999999, lang="mt"), "disgħin disgħa biljun disgħa mija disgħin disgħa miljun disgħa mija disgħin disgħa elf disgħa mija disgħin disgħa")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="mt"), "minus wieħed")
        self.assertEqual(num2words(-2, lang="mt"), "minus tnejn")
        self.assertEqual(num2words(-5, lang="mt"), "minus ħamsa")
        self.assertEqual(num2words(-10, lang="mt"), "minus għaxra")
        self.assertEqual(num2words(-11, lang="mt"), "minus għaxra wieħed")
        self.assertEqual(num2words(-20, lang="mt"), "minus għoxrin")
        self.assertEqual(num2words(-50, lang="mt"), "minus ħamsin")
        self.assertEqual(num2words(-99, lang="mt"), "minus disgħin disgħa")
        self.assertEqual(num2words(-100, lang="mt"), "minus wieħed mija")
        self.assertEqual(num2words(-101, lang="mt"), "minus wieħed mija wieħed")
        self.assertEqual(num2words(-200, lang="mt"), "minus tnejn mija")
        self.assertEqual(num2words(-999, lang="mt"), "minus disgħa mija disgħin disgħa")
        self.assertEqual(num2words(-1000, lang="mt"), "minus wieħed elf")
        self.assertEqual(num2words(-1001, lang="mt"), "minus wieħed elf wieħed")
        self.assertEqual(num2words(-10000, lang="mt"), "minus għaxra elf")
        self.assertEqual(num2words(-100000, lang="mt"), "minus wieħed mija elf")
        self.assertEqual(num2words(-1000000, lang="mt"), "minus wieħed miljun")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="mt"), "żero punt wieħed")
        self.assertEqual(num2words(0.5, lang="mt"), "żero punt ħamsa")
        self.assertEqual(num2words(0.9, lang="mt"), "żero punt disgħa")
        self.assertEqual(num2words(1.1, lang="mt"), "wieħed punt wieħed")
        self.assertEqual(num2words(1.5, lang="mt"), "wieħed punt ħamsa")
        self.assertEqual(num2words(2.5, lang="mt"), "tnejn punt ħamsa")
        self.assertEqual(num2words(3.14, lang="mt"), "tlieta punt wieħed erbgħa")
        self.assertEqual(num2words(10.5, lang="mt"), "għaxra punt ħamsa")
        self.assertEqual(
            num2words(11.11, lang="mt"), "għaxra wieħed punt wieħed wieħed"
        )
        self.assertEqual(num2words(20.2, lang="mt"), "għoxrin punt tnejn")
        self.assertEqual(
            num2words(99.99, lang="mt"), "disgħin disgħa punt disgħa disgħa"
        )
        self.assertEqual(num2words(100.01, lang="mt"), "wieħed mija punt żero wieħed")
        self.assertEqual(num2words(100.5, lang="mt"), "wieħed mija punt ħamsa")
        self.assertEqual(
            num2words(123.45, lang="mt"),
            "wieħed mija għoxrin tlieta punt erbgħa ħamsa",
        )
        self.assertEqual(num2words(1000.5, lang="mt"), "wieħed elf punt ħamsa")
        self.assertEqual(
            num2words(1234.56, lang="mt"),
            "wieħed elf tnejn mija tletin erbgħa punt ħamsa sitta",
        )
        self.assertEqual(num2words(10000.01, lang="mt"), "għaxra elf punt żero wieħed")
        self.assertEqual(num2words(-0.5, lang="mt"), "minus żero punt ħamsa")
        self.assertEqual(num2words(-1.5, lang="mt"), "minus wieħed punt ħamsa")
        self.assertEqual(num2words(-10.5, lang="mt"), "minus għaxra punt ħamsa")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="mt", ordinal=True), "l-ewwel")
        self.assertEqual(num2words(2, lang="mt", ordinal=True), "it-tieni")
        self.assertEqual(num2words(3, lang="mt", ordinal=True), "it-tielet")
        self.assertEqual(num2words(4, lang="mt", ordinal=True), "ir-raba'")
        self.assertEqual(num2words(5, lang="mt", ordinal=True), "il-ħames")
        self.assertEqual(num2words(6, lang="mt", ordinal=True), "is-sitt")
        self.assertEqual(num2words(7, lang="mt", ordinal=True), "is-seba'")
        self.assertEqual(num2words(8, lang="mt", ordinal=True), "it-tmien")
        self.assertEqual(num2words(9, lang="mt", ordinal=True), "id-disa'")
        self.assertEqual(num2words(10, lang="mt", ordinal=True), "l-għaxar")
        self.assertEqual(num2words(11, lang="mt", ordinal=True), "l-għaxra wieħed")
        self.assertEqual(num2words(12, lang="mt", ordinal=True), "l-għaxra tnejn")
        self.assertEqual(num2words(13, lang="mt", ordinal=True), "l-għaxra tlieta")
        self.assertEqual(num2words(14, lang="mt", ordinal=True), "l-għaxra erbgħa")
        self.assertEqual(num2words(15, lang="mt", ordinal=True), "l-għaxra ħamsa")
        self.assertEqual(num2words(16, lang="mt", ordinal=True), "l-għaxra sitta")
        self.assertEqual(num2words(17, lang="mt", ordinal=True), "l-għaxra sebgħa")
        self.assertEqual(num2words(18, lang="mt", ordinal=True), "l-għaxra tmienja")
        self.assertEqual(num2words(19, lang="mt", ordinal=True), "l-għaxra disgħa")
        self.assertEqual(num2words(20, lang="mt", ordinal=True), "l-għoxrin")
        self.assertEqual(num2words(21, lang="mt", ordinal=True), "l-għoxrin wieħed")
        self.assertEqual(num2words(22, lang="mt", ordinal=True), "l-għoxrin tnejn")
        self.assertEqual(num2words(25, lang="mt", ordinal=True), "l-għoxrin ħamsa")
        self.assertEqual(num2words(30, lang="mt", ordinal=True), "l-tletin")
        self.assertEqual(num2words(40, lang="mt", ordinal=True), "l-erbgħin")
        self.assertEqual(num2words(50, lang="mt", ordinal=True), "l-ħamsin")
        self.assertEqual(num2words(60, lang="mt", ordinal=True), "l-sittin")
        self.assertEqual(num2words(70, lang="mt", ordinal=True), "l-sebgħin")
        self.assertEqual(num2words(80, lang="mt", ordinal=True), "l-tmenin")
        self.assertEqual(num2words(90, lang="mt", ordinal=True), "l-disgħin")
        self.assertEqual(num2words(100, lang="mt", ordinal=True), "l-wieħed mija")
        self.assertEqual(
            num2words(101, lang="mt", ordinal=True), "l-wieħed mija wieħed"
        )
        self.assertEqual(num2words(200, lang="mt", ordinal=True), "l-tnejn mija")
        self.assertEqual(num2words(500, lang="mt", ordinal=True), "l-ħamsa mija")
        self.assertEqual(num2words(1000, lang="mt", ordinal=True), "l-wieħed elf")
        self.assertEqual(
            num2words(1001, lang="mt", ordinal=True), "l-wieħed elf wieħed"
        )
        self.assertEqual(num2words(10000, lang="mt", ordinal=True), "l-għaxra elf")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="mt", to="currency", currency="EUR"), "żero ewro"
        )
        self.assertEqual(
            num2words(0.01, lang="mt", to="currency", currency="EUR"),
            "żero ewro wieħed ċenteżmu",
        )
        self.assertEqual(
            num2words(0.5, lang="mt", to="currency", currency="EUR"),
            "żero ewro ħamsin ċenteżmi",
        )
        self.assertEqual(
            num2words(1, lang="mt", to="currency", currency="EUR"), "wieħed ewro"
        )
        self.assertEqual(
            num2words(1.5, lang="mt", to="currency", currency="EUR"),
            "wieħed ewro ħamsin ċenteżmi",
        )
        self.assertEqual(
            num2words(0, lang="mt", to="currency", currency="USD"), "żero dollari"
        )
        self.assertEqual(
            num2words(0.01, lang="mt", to="currency", currency="USD"),
            "żero dollari wieħed ċenteżmu",
        )
        self.assertEqual(
            num2words(0.5, lang="mt", to="currency", currency="USD"),
            "żero dollari ħamsin ċenteżmi",
        )
        self.assertEqual(
            num2words(1, lang="mt", to="currency", currency="USD"), "wieħed dollaru"
        )
        self.assertEqual(
            num2words(1.5, lang="mt", to="currency", currency="USD"),
            "wieħed dollaru ħamsin ċenteżmi",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="mt", to="year"), "wieħed elf")
        self.assertEqual(
            num2words(1066, lang="mt", to="year"), "wieħed elf sittin sitta"
        )
        self.assertEqual(
            num2words(1492, lang="mt", to="year"),
            "wieħed elf erbgħa mija disgħin tnejn",
        )
        self.assertEqual(
            num2words(1776, lang="mt", to="year"),
            "wieħed elf sebgħa mija sebgħin sitta",
        )
        self.assertEqual(
            num2words(1800, lang="mt", to="year"), "wieħed elf tmienja mija"
        )
        self.assertEqual(
            num2words(1900, lang="mt", to="year"), "wieħed elf disgħa mija"
        )
        self.assertEqual(
            num2words(1984, lang="mt", to="year"),
            "wieħed elf disgħa mija tmenin erbgħa",
        )
        self.assertEqual(
            num2words(1999, lang="mt", to="year"),
            "wieħed elf disgħa mija disgħin disgħa",
        )
        self.assertEqual(num2words(2000, lang="mt", to="year"), "tnejn elf")
        self.assertEqual(num2words(2001, lang="mt", to="year"), "tnejn elf wieħed")
        self.assertEqual(num2words(2010, lang="mt", to="year"), "tnejn elf għaxra")
        self.assertEqual(num2words(2020, lang="mt", to="year"), "tnejn elf għoxrin")
        self.assertEqual(
            num2words(2024, lang="mt", to="year"), "tnejn elf għoxrin erbgħa"
        )
        self.assertEqual(num2words(2100, lang="mt", to="year"), "tnejn elf wieħed mija")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="mt"), "żero")
        self.assertEqual(num2words("1", lang="mt"), "wieħed")
        self.assertEqual(num2words("10", lang="mt"), "għaxra")
        self.assertEqual(num2words("100", lang="mt"), "wieħed mija")
        self.assertEqual(num2words("1000", lang="mt"), "wieħed elf")
        self.assertEqual(num2words("10000", lang="mt"), "għaxra elf")
        self.assertEqual(num2words("100000", lang="mt"), "wieħed mija elf")
        self.assertEqual(num2words("1000000", lang="mt"), "wieħed miljun")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="mt"), "żero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="mt"), num2words("100", lang="mt"))
        self.assertEqual(num2words(1000, lang="mt"), num2words("1000", lang="mt"))

