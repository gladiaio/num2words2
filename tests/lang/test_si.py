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


class Num2WordsSITest(TestCase):
    """Comprehensive test cases for Sinhala language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="si"), "බිංදුව")
        self.assertEqual(num2words(1, lang="si"), "එක")
        self.assertEqual(num2words(2, lang="si"), "දෙක")
        self.assertEqual(num2words(3, lang="si"), "තුන")
        self.assertEqual(num2words(4, lang="si"), "හතර")
        self.assertEqual(num2words(5, lang="si"), "පහ")
        self.assertEqual(num2words(6, lang="si"), "හය")
        self.assertEqual(num2words(7, lang="si"), "හත")
        self.assertEqual(num2words(8, lang="si"), "අට")
        self.assertEqual(num2words(9, lang="si"), "නවය")
        self.assertEqual(num2words(10, lang="si"), "දහය")
        self.assertEqual(num2words(11, lang="si"), "එකොළහ")
        self.assertEqual(num2words(12, lang="si"), "දොළහ")
        self.assertEqual(num2words(13, lang="si"), "දහතුන")
        self.assertEqual(num2words(14, lang="si"), "දහහතර")
        self.assertEqual(num2words(15, lang="si"), "පහළොව")
        self.assertEqual(num2words(16, lang="si"), "දහසය")
        self.assertEqual(num2words(17, lang="si"), "දහහත")
        self.assertEqual(num2words(18, lang="si"), "දහඅට")
        self.assertEqual(num2words(19, lang="si"), "දහනවය")
        self.assertEqual(num2words(20, lang="si"), "විස්ස")
        self.assertEqual(num2words(21, lang="si"), "විසිඑක")
        self.assertEqual(num2words(22, lang="si"), "විසිදෙක")
        self.assertEqual(num2words(23, lang="si"), "විසිතුන")
        self.assertEqual(num2words(24, lang="si"), "විසිහතර")
        self.assertEqual(num2words(25, lang="si"), "විසිපහ")
        self.assertEqual(num2words(26, lang="si"), "විසිහය")
        self.assertEqual(num2words(27, lang="si"), "විසිහත")
        self.assertEqual(num2words(28, lang="si"), "විසිඅට")
        self.assertEqual(num2words(29, lang="si"), "විසිනවය")
        self.assertEqual(num2words(30, lang="si"), "තිහ")
        self.assertEqual(num2words(31, lang="si"), "තිස්එක")
        self.assertEqual(num2words(35, lang="si"), "තිස්පහ")
        self.assertEqual(num2words(40, lang="si"), "හතළිහ")
        self.assertEqual(num2words(45, lang="si"), "හතළිස්පහ")
        self.assertEqual(num2words(50, lang="si"), "පනහ")
        self.assertEqual(num2words(55, lang="si"), "පනස්පහ")
        self.assertEqual(num2words(60, lang="si"), "හැට")
        self.assertEqual(num2words(65, lang="si"), "හැටපහ")
        self.assertEqual(num2words(70, lang="si"), "හැත්තෑව")
        self.assertEqual(num2words(75, lang="si"), "හැත්තෑපහ")
        self.assertEqual(num2words(80, lang="si"), "අසූව")
        self.assertEqual(num2words(85, lang="si"), "අසූපහ")
        self.assertEqual(num2words(90, lang="si"), "අනූව")
        self.assertEqual(num2words(95, lang="si"), "අනූපහ")
        self.assertEqual(num2words(99, lang="si"), "අනූනවය")
        self.assertEqual(num2words(100, lang="si"), "සියය")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="si"), "සියය එක")
        self.assertEqual(num2words(110, lang="si"), "සියය දහය")
        self.assertEqual(num2words(111, lang="si"), "සියය එකොළහ")
        self.assertEqual(num2words(120, lang="si"), "සියය විස්ස")
        self.assertEqual(num2words(125, lang="si"), "සියය විසිපහ")
        self.assertEqual(num2words(150, lang="si"), "සියය පනහ")
        self.assertEqual(num2words(175, lang="si"), "සියය හැත්තෑපහ")
        self.assertEqual(num2words(199, lang="si"), "සියය අනූනවය")
        self.assertEqual(num2words(200, lang="si"), "දෙක සියය")
        self.assertEqual(num2words(201, lang="si"), "දෙක සියය එක")
        self.assertEqual(num2words(210, lang="si"), "දෙක සියය දහය")
        self.assertEqual(num2words(220, lang="si"), "දෙක සියය විස්ස")
        self.assertEqual(num2words(250, lang="si"), "දෙක සියය පනහ")
        self.assertEqual(num2words(299, lang="si"), "දෙක සියය අනූනවය")
        self.assertEqual(num2words(300, lang="si"), "තුන සියය")
        self.assertEqual(num2words(333, lang="si"), "තුන සියය තිස්තුන")
        self.assertEqual(num2words(400, lang="si"), "හතර සියය")
        self.assertEqual(num2words(444, lang="si"), "හතර සියය හතළිස්හතර")
        self.assertEqual(num2words(500, lang="si"), "පහ සියය")
        self.assertEqual(num2words(555, lang="si"), "පහ සියය පනස්පහ")
        self.assertEqual(num2words(600, lang="si"), "හය සියය")
        self.assertEqual(num2words(666, lang="si"), "හය සියය හැටහය")
        self.assertEqual(num2words(700, lang="si"), "හත සියය")
        self.assertEqual(num2words(777, lang="si"), "හත සියය හැත්තෑහත")
        self.assertEqual(num2words(800, lang="si"), "අට සියය")
        self.assertEqual(num2words(888, lang="si"), "අට සියය අසූඅට")
        self.assertEqual(num2words(900, lang="si"), "නවය සියය")
        self.assertEqual(num2words(999, lang="si"), "නවය සියය අනූනවය")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="si"), "දහස")
        self.assertEqual(num2words(1001, lang="si"), "දහස එක")
        self.assertEqual(num2words(1010, lang="si"), "දහස දහය")
        self.assertEqual(num2words(1100, lang="si"), "දහස සියය")
        self.assertEqual(num2words(1111, lang="si"), "දහස සියය එකොළහ")
        self.assertEqual(num2words(1234, lang="si"), "දහස දෙක සියය තිස්හතර")
        self.assertEqual(num2words(1500, lang="si"), "දහස පහ සියය")
        self.assertEqual(num2words(1999, lang="si"), "දහස නවය සියය අනූනවය")
        self.assertEqual(num2words(2000, lang="si"), "දෙක දහස")
        self.assertEqual(num2words(2001, lang="si"), "දෙක දහස එක")
        self.assertEqual(num2words(2020, lang="si"), "දෙක දහස විස්ස")
        self.assertEqual(num2words(2222, lang="si"), "දෙක දහස දෙක සියය විසිදෙක")
        self.assertEqual(num2words(3000, lang="si"), "තුන දහස")
        self.assertEqual(num2words(3333, lang="si"), "තුන දහස තුන සියය තිස්තුන")
        self.assertEqual(num2words(4000, lang="si"), "හතර දහස")
        self.assertEqual(num2words(4444, lang="si"), "හතර දහස හතර සියය හතළිස්හතර")
        self.assertEqual(num2words(5000, lang="si"), "පහ දහස")
        self.assertEqual(num2words(5555, lang="si"), "පහ දහස පහ සියය පනස්පහ")
        self.assertEqual(num2words(6000, lang="si"), "හය දහස")
        self.assertEqual(num2words(6666, lang="si"), "හය දහස හය සියය හැටහය")
        self.assertEqual(num2words(7000, lang="si"), "හත දහස")
        self.assertEqual(num2words(7777, lang="si"), "හත දහස හත සියය හැත්තෑහත")
        self.assertEqual(num2words(8000, lang="si"), "අට දහස")
        self.assertEqual(num2words(8888, lang="si"), "අට දහස අට සියය අසූඅට")
        self.assertEqual(num2words(9000, lang="si"), "නවය දහස")
        self.assertEqual(num2words(9999, lang="si"), "නවය දහස නවය සියය අනූනවය")
        self.assertEqual(num2words(10000, lang="si"), "දහය දහස")
        self.assertEqual(num2words(10001, lang="si"), "දහය දහස එක")
        self.assertEqual(num2words(11111, lang="si"), "එකොළහ දහස සියය එකොළහ")
        self.assertEqual(num2words(12345, lang="si"), "දොළහ දහස තුන සියය හතළිස්පහ")
        self.assertEqual(num2words(20000, lang="si"), "විස්ස දහස")
        self.assertEqual(num2words(50000, lang="si"), "පනහ දහස")
        self.assertEqual(num2words(99999, lang="si"), "අනූනවය දහස නවය සියය අනූනවය")
        self.assertEqual(num2words(100000, lang="si"), "ලක්ෂය")
        self.assertEqual(
            num2words(123456, lang="si"), "ලක්ෂය විසිතුන දහස හතර සියය පනස්හය"
        )
        self.assertEqual(num2words(200000, lang="si"), "දෙක ලක්ෂය")
        self.assertEqual(num2words(500000, lang="si"), "පහ ලක්ෂය")
        self.assertEqual(
            num2words(654321, lang="si"), "හය ලක්ෂය පනස්හතර දහස තුන සියය විසිඑක"
        )
        self.assertEqual(
            num2words(999999, lang="si"), "නවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="si"), "දහය ලක්ෂය")
        self.assertEqual(num2words(1000001, lang="si"), "දහය ලක්ෂය එක")
        self.assertEqual(
            num2words(1111111, lang="si"), "එකොළහ ලක්ෂය එකොළහ දහස සියය එකොළහ"
        )
        self.assertEqual(
            num2words(1234567, lang="si"), "දොළහ ලක්ෂය තිස්හතර දහස පහ සියය හැටහත"
        )
        self.assertEqual(num2words(2000000, lang="si"), "විස්ස ලක්ෂය")
        self.assertEqual(num2words(5000000, lang="si"), "පනහ ලක්ෂය")
        self.assertEqual(
            num2words(9999999, lang="si"),
            "අනූනවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය",
        )
        self.assertEqual(num2words(10000000, lang="si"), "කෝටිය")
        self.assertEqual(
            num2words(12345678, lang="si"),
            "කෝටිය විසිතුන ලක්ෂය හතළිස්පහ දහස හය සියය හැත්තෑඅට",
        )
        self.assertEqual(
            num2words(99999999, lang="si"),
            "නවය කෝටිය අනූනවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය",
        )
        self.assertEqual(num2words(100000000, lang="si"), "දහය කෝටිය")
        self.assertEqual(
            num2words(123456789, lang="si"),
            "දොළහ කෝටිය තිස්හතර ලක්ෂය පනස්හය දහස හත සියය අසූනවය",
        )
        self.assertEqual(
            num2words(999999999, lang="si"),
            "අනූනවය කෝටිය අනූනවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය",
        )
        self.assertEqual(num2words(1000000000, lang="si"), "සියය කෝටිය")
        self.assertEqual(num2words(1234567890, lang="si"), "සියය විසිතුන කෝටිය හතළිස්පහ ලක්ෂය හැටහත දහස අට සියය අනූව")
        self.assertEqual(num2words(9999999999, lang="si"), "නවය සියය අනූනවය කෝටිය අනූනවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය")
        self.assertEqual(num2words(10000000000, lang="si"), "දහස කෝටිය")
        self.assertEqual(num2words(99999999999, lang="si"), "නවය දහස නවය සියය අනූනවය කෝටිය අනූනවය ලක්ෂය අනූනවය දහස නවය සියය අනූනවය")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="si"), "සෘණ එක")
        self.assertEqual(num2words(-2, lang="si"), "සෘණ දෙක")
        self.assertEqual(num2words(-5, lang="si"), "සෘණ පහ")
        self.assertEqual(num2words(-10, lang="si"), "සෘණ දහය")
        self.assertEqual(num2words(-11, lang="si"), "සෘණ එකොළහ")
        self.assertEqual(num2words(-20, lang="si"), "සෘණ විස්ස")
        self.assertEqual(num2words(-50, lang="si"), "සෘණ පනහ")
        self.assertEqual(num2words(-99, lang="si"), "සෘණ අනූනවය")
        self.assertEqual(num2words(-100, lang="si"), "සෘණ සියය")
        self.assertEqual(num2words(-101, lang="si"), "සෘණ සියය එක")
        self.assertEqual(num2words(-200, lang="si"), "සෘණ දෙක සියය")
        self.assertEqual(num2words(-999, lang="si"), "සෘණ නවය සියය අනූනවය")
        self.assertEqual(num2words(-1000, lang="si"), "සෘණ දහස")
        self.assertEqual(num2words(-1001, lang="si"), "සෘණ දහස එක")
        self.assertEqual(num2words(-10000, lang="si"), "සෘණ දහය දහස")
        self.assertEqual(num2words(-100000, lang="si"), "සෘණ ලක්ෂය")
        self.assertEqual(num2words(-1000000, lang="si"), "සෘණ දහය ලක්ෂය")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="si"), "බිංදුව දශම එක")
        self.assertEqual(num2words(0.5, lang="si"), "බිංදුව දශම පහ")
        self.assertEqual(num2words(0.9, lang="si"), "බිංදුව දශම නවය")
        self.assertEqual(num2words(1.1, lang="si"), "එක දශම එක")
        self.assertEqual(num2words(1.5, lang="si"), "එක දශම පහ")
        self.assertEqual(num2words(2.5, lang="si"), "දෙක දශම පහ")
        self.assertEqual(num2words(3.14, lang="si"), "තුන දශම එක හතර")
        self.assertEqual(num2words(10.5, lang="si"), "දහය දශම පහ")
        self.assertEqual(num2words(11.11, lang="si"), "එකොළහ දශම එක එක")
        self.assertEqual(num2words(20.2, lang="si"), "විස්ස දශම දෙක")
        self.assertEqual(num2words(99.99, lang="si"), "අනූනවය දශම නවය නවය")
        self.assertEqual(num2words(100.01, lang="si"), "සියය දශම බිංදුව එක")
        self.assertEqual(num2words(100.5, lang="si"), "සියය දශම පහ")
        self.assertEqual(num2words(123.45, lang="si"), "සියය විසිතුන දශම හතර පහ")
        self.assertEqual(num2words(1000.5, lang="si"), "දහස දශම පහ")
        self.assertEqual(
            num2words(1234.56, lang="si"), "දහස දෙක සියය තිස්හතර දශම පහ හය"
        )
        self.assertEqual(num2words(10000.01, lang="si"), "දහය දහස දශම බිංදුව එක")
        self.assertEqual(num2words(-0.5, lang="si"), "සෘණ බිංදුව දශම පහ")
        self.assertEqual(num2words(-1.5, lang="si"), "සෘණ එක දශම පහ")
        self.assertEqual(num2words(-10.5, lang="si"), "සෘණ දහය දශම පහ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="si", ordinal=True), "එක වැනි")
        self.assertEqual(num2words(2, lang="si", ordinal=True), "දෙක වැනි")
        self.assertEqual(num2words(3, lang="si", ordinal=True), "තුන වැනි")
        self.assertEqual(num2words(4, lang="si", ordinal=True), "හතර වැනි")
        self.assertEqual(num2words(5, lang="si", ordinal=True), "පහ වැනි")
        self.assertEqual(num2words(6, lang="si", ordinal=True), "හය වැනි")
        self.assertEqual(num2words(7, lang="si", ordinal=True), "හත වැනි")
        self.assertEqual(num2words(8, lang="si", ordinal=True), "අට වැනි")
        self.assertEqual(num2words(9, lang="si", ordinal=True), "නවය වැනි")
        self.assertEqual(num2words(10, lang="si", ordinal=True), "දහය වැනි")
        self.assertEqual(num2words(11, lang="si", ordinal=True), "එකොළහ වැනි")
        self.assertEqual(num2words(12, lang="si", ordinal=True), "දොළහ වැනි")
        self.assertEqual(num2words(13, lang="si", ordinal=True), "දහතුන වැනි")
        self.assertEqual(num2words(14, lang="si", ordinal=True), "දහහතර වැනි")
        self.assertEqual(num2words(15, lang="si", ordinal=True), "පහළොව වැනි")
        self.assertEqual(num2words(16, lang="si", ordinal=True), "දහසය වැනි")
        self.assertEqual(num2words(17, lang="si", ordinal=True), "දහහත වැනි")
        self.assertEqual(num2words(18, lang="si", ordinal=True), "දහඅට වැනි")
        self.assertEqual(num2words(19, lang="si", ordinal=True), "දහනවය වැනි")
        self.assertEqual(num2words(20, lang="si", ordinal=True), "විස්ස වැනි")
        self.assertEqual(num2words(21, lang="si", ordinal=True), "විසිඑක වැනි")
        self.assertEqual(num2words(22, lang="si", ordinal=True), "විසිදෙක වැනි")
        self.assertEqual(num2words(25, lang="si", ordinal=True), "විසිපහ වැනි")
        self.assertEqual(num2words(30, lang="si", ordinal=True), "තිහ වැනි")
        self.assertEqual(num2words(40, lang="si", ordinal=True), "හතළිහ වැනි")
        self.assertEqual(num2words(50, lang="si", ordinal=True), "පනහ වැනි")
        self.assertEqual(num2words(60, lang="si", ordinal=True), "හැට වැනි")
        self.assertEqual(num2words(70, lang="si", ordinal=True), "හැත්තෑව වැනි")
        self.assertEqual(num2words(80, lang="si", ordinal=True), "අසූව වැනි")
        self.assertEqual(num2words(90, lang="si", ordinal=True), "අනූව වැනි")
        self.assertEqual(num2words(100, lang="si", ordinal=True), "සියය වැනි")
        self.assertEqual(num2words(101, lang="si", ordinal=True), "සියය එක වැනි")
        self.assertEqual(num2words(200, lang="si", ordinal=True), "දෙක සියය වැනි")
        self.assertEqual(num2words(500, lang="si", ordinal=True), "පහ සියය වැනි")
        self.assertEqual(num2words(1000, lang="si", ordinal=True), "දහස වැනි")
        self.assertEqual(num2words(1001, lang="si", ordinal=True), "දහස එක වැනි")
        self.assertEqual(num2words(10000, lang="si", ordinal=True), "දහය දහස වැනි")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="si", to="currency", currency="LKR"), "බිංදුව රුපියල්"
        )
        self.assertEqual(
            num2words(0.01, lang="si", to="currency", currency="LKR"),
            "බිංදුව රුපියල් එක සත",
        )
        self.assertEqual(
            num2words(0.5, lang="si", to="currency", currency="LKR"),
            "බිංදුව රුපියල් පනහ සත",
        )
        self.assertEqual(
            num2words(1, lang="si", to="currency", currency="LKR"), "එක රුපියල්"
        )
        self.assertEqual(
            num2words(1.5, lang="si", to="currency", currency="LKR"),
            "එක රුපියල් පනහ සත",
        )
        self.assertEqual(
            num2words(0, lang="si", to="currency", currency="USD"), "බිංදුව ඩොලර්"
        )
        self.assertEqual(
            num2words(0.01, lang="si", to="currency", currency="USD"),
            "බිංදුව ඩොලර් එක ශත",
        )
        self.assertEqual(
            num2words(0.5, lang="si", to="currency", currency="USD"),
            "බිංදුව ඩොලර් පනහ ශත",
        )
        self.assertEqual(
            num2words(1, lang="si", to="currency", currency="USD"), "එක ඩොලර්"
        )
        self.assertEqual(
            num2words(1.5, lang="si", to="currency", currency="USD"),
            "එක ඩොලර් පනහ ශත",
        )
        self.assertEqual(
            num2words(0, lang="si", to="currency", currency="EUR"), "බිංදුව යූරෝ"
        )
        self.assertEqual(
            num2words(0.01, lang="si", to="currency", currency="EUR"),
            "බිංදුව යූරෝ එක ශත",
        )
        self.assertEqual(
            num2words(0.5, lang="si", to="currency", currency="EUR"),
            "බිංදුව යූරෝ පනහ ශත",
        )
        self.assertEqual(
            num2words(1, lang="si", to="currency", currency="EUR"), "එක යූරෝ"
        )
        self.assertEqual(
            num2words(1.5, lang="si", to="currency", currency="EUR"),
            "එක යූරෝ පනහ ශත",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="si", to="year"), "දහස")
        self.assertEqual(num2words(1066, lang="si", to="year"), "දහස හැටහය")
        self.assertEqual(num2words(1492, lang="si", to="year"), "දහස හතර සියය අනූදෙක")
        self.assertEqual(
            num2words(1776, lang="si", to="year"), "දහස හත සියය හැත්තෑහය"
        )
        self.assertEqual(num2words(1800, lang="si", to="year"), "දහස අට සියය")
        self.assertEqual(num2words(1900, lang="si", to="year"), "දහස නවය සියය")
        self.assertEqual(num2words(1984, lang="si", to="year"), "දහස නවය සියය අසූහතර")
        self.assertEqual(num2words(1999, lang="si", to="year"), "දහස නවය සියය අනූනවය")
        self.assertEqual(num2words(2000, lang="si", to="year"), "දෙක දහස")
        self.assertEqual(num2words(2001, lang="si", to="year"), "දෙක දහස එක")
        self.assertEqual(num2words(2010, lang="si", to="year"), "දෙක දහස දහය")
        self.assertEqual(num2words(2020, lang="si", to="year"), "දෙක දහස විස්ස")
        self.assertEqual(num2words(2024, lang="si", to="year"), "දෙක දහස විසිහතර")
        self.assertEqual(num2words(2100, lang="si", to="year"), "දෙක දහස සියය")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="si"), "බිංදුව")
        self.assertEqual(num2words("1", lang="si"), "එක")
        self.assertEqual(num2words("10", lang="si"), "දහය")
        self.assertEqual(num2words("100", lang="si"), "සියය")
        self.assertEqual(num2words("1000", lang="si"), "දහස")
        self.assertEqual(num2words("10000", lang="si"), "දහය දහස")
        self.assertEqual(num2words("100000", lang="si"), "ලක්ෂය")
        self.assertEqual(num2words("1000000", lang="si"), "දහය ලක්ෂය")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="si"), "බිංදුව")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="si"), num2words("100", lang="si"))
        self.assertEqual(num2words(1000, lang="si"), num2words("1000", lang="si"))

