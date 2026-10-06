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


class Num2WordsSDTest(TestCase):
    """Comprehensive test cases for Sindhi language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sd"), "ٻڙي")
        self.assertEqual(num2words(1, lang="sd"), "هڪ")
        self.assertEqual(num2words(2, lang="sd"), "ٻه")
        self.assertEqual(num2words(3, lang="sd"), "ٽي")
        self.assertEqual(num2words(4, lang="sd"), "چار")
        self.assertEqual(num2words(5, lang="sd"), "پنج")
        self.assertEqual(num2words(6, lang="sd"), "ڇهه")
        self.assertEqual(num2words(7, lang="sd"), "ست")
        self.assertEqual(num2words(8, lang="sd"), "اٺ")
        self.assertEqual(num2words(9, lang="sd"), "نو")
        self.assertEqual(num2words(10, lang="sd"), "ڏهه")
        self.assertEqual(num2words(11, lang="sd"), "ڏهه هڪ")
        self.assertEqual(num2words(12, lang="sd"), "ڏهه ٻه")
        self.assertEqual(num2words(13, lang="sd"), "ڏهه ٽي")
        self.assertEqual(num2words(14, lang="sd"), "ڏهه چار")
        self.assertEqual(num2words(15, lang="sd"), "ڏهه پنج")
        self.assertEqual(num2words(16, lang="sd"), "ڏهه ڇهه")
        self.assertEqual(num2words(17, lang="sd"), "ڏهه ست")
        self.assertEqual(num2words(18, lang="sd"), "ڏهه اٺ")
        self.assertEqual(num2words(19, lang="sd"), "ڏهه نو")
        self.assertEqual(num2words(20, lang="sd"), "ويهه")
        self.assertEqual(num2words(21, lang="sd"), "ويهه هڪ")
        self.assertEqual(num2words(22, lang="sd"), "ويهه ٻه")
        self.assertEqual(num2words(23, lang="sd"), "ويهه ٽي")
        self.assertEqual(num2words(24, lang="sd"), "ويهه چار")
        self.assertEqual(num2words(25, lang="sd"), "ويهه پنج")
        self.assertEqual(num2words(26, lang="sd"), "ويهه ڇهه")
        self.assertEqual(num2words(27, lang="sd"), "ويهه ست")
        self.assertEqual(num2words(28, lang="sd"), "ويهه اٺ")
        self.assertEqual(num2words(29, lang="sd"), "ويهه نو")
        self.assertEqual(num2words(30, lang="sd"), "ٽيهه")
        self.assertEqual(num2words(31, lang="sd"), "ٽيهه هڪ")
        self.assertEqual(num2words(35, lang="sd"), "ٽيهه پنج")
        self.assertEqual(num2words(40, lang="sd"), "چاليهه")
        self.assertEqual(num2words(45, lang="sd"), "چاليهه پنج")
        self.assertEqual(num2words(50, lang="sd"), "پنجاهه")
        self.assertEqual(num2words(55, lang="sd"), "پنجاهه پنج")
        self.assertEqual(num2words(60, lang="sd"), "سٺ")
        self.assertEqual(num2words(65, lang="sd"), "سٺ پنج")
        self.assertEqual(num2words(70, lang="sd"), "ستر")
        self.assertEqual(num2words(75, lang="sd"), "ستر پنج")
        self.assertEqual(num2words(80, lang="sd"), "اسي")
        self.assertEqual(num2words(85, lang="sd"), "اسي پنج")
        self.assertEqual(num2words(90, lang="sd"), "نوي")
        self.assertEqual(num2words(95, lang="sd"), "نوي پنج")
        self.assertEqual(num2words(99, lang="sd"), "نوي نو")
        self.assertEqual(num2words(100, lang="sd"), "هڪ سو")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sd"), "هڪ سو هڪ")
        self.assertEqual(num2words(110, lang="sd"), "هڪ سو ڏهه")
        self.assertEqual(num2words(111, lang="sd"), "هڪ سو ڏهه هڪ")
        self.assertEqual(num2words(120, lang="sd"), "هڪ سو ويهه")
        self.assertEqual(num2words(125, lang="sd"), "هڪ سو ويهه پنج")
        self.assertEqual(num2words(150, lang="sd"), "هڪ سو پنجاهه")
        self.assertEqual(num2words(175, lang="sd"), "هڪ سو ستر پنج")
        self.assertEqual(num2words(199, lang="sd"), "هڪ سو نوي نو")
        self.assertEqual(num2words(200, lang="sd"), "ٻه سو")
        self.assertEqual(num2words(201, lang="sd"), "ٻه سو هڪ")
        self.assertEqual(num2words(210, lang="sd"), "ٻه سو ڏهه")
        self.assertEqual(num2words(220, lang="sd"), "ٻه سو ويهه")
        self.assertEqual(num2words(250, lang="sd"), "ٻه سو پنجاهه")
        self.assertEqual(num2words(299, lang="sd"), "ٻه سو نوي نو")
        self.assertEqual(num2words(300, lang="sd"), "ٽي سو")
        self.assertEqual(num2words(333, lang="sd"), "ٽي سو ٽيهه ٽي")
        self.assertEqual(num2words(400, lang="sd"), "چار سو")
        self.assertEqual(num2words(444, lang="sd"), "چار سو چاليهه چار")
        self.assertEqual(num2words(500, lang="sd"), "پنج سو")
        self.assertEqual(num2words(555, lang="sd"), "پنج سو پنجاهه پنج")
        self.assertEqual(num2words(600, lang="sd"), "ڇهه سو")
        self.assertEqual(num2words(666, lang="sd"), "ڇهه سو سٺ ڇهه")
        self.assertEqual(num2words(700, lang="sd"), "ست سو")
        self.assertEqual(num2words(777, lang="sd"), "ست سو ستر ست")
        self.assertEqual(num2words(800, lang="sd"), "اٺ سو")
        self.assertEqual(num2words(888, lang="sd"), "اٺ سو اسي اٺ")
        self.assertEqual(num2words(900, lang="sd"), "نو سو")
        self.assertEqual(num2words(999, lang="sd"), "نو سو نوي نو")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sd"), "هڪ هزار")
        self.assertEqual(num2words(1001, lang="sd"), "هڪ هزار هڪ")
        self.assertEqual(num2words(1010, lang="sd"), "هڪ هزار ڏهه")
        self.assertEqual(num2words(1100, lang="sd"), "هڪ هزار هڪ سو")
        self.assertEqual(num2words(1111, lang="sd"), "هڪ هزار هڪ سو ڏهه هڪ")
        self.assertEqual(num2words(1234, lang="sd"), "هڪ هزار ٻه سو ٽيهه چار")
        self.assertEqual(num2words(1500, lang="sd"), "هڪ هزار پنج سو")
        self.assertEqual(num2words(1999, lang="sd"), "هڪ هزار نو سو نوي نو")
        self.assertEqual(num2words(2000, lang="sd"), "ٻه هزار")
        self.assertEqual(num2words(2001, lang="sd"), "ٻه هزار هڪ")
        self.assertEqual(num2words(2020, lang="sd"), "ٻه هزار ويهه")
        self.assertEqual(num2words(2222, lang="sd"), "ٻه هزار ٻه سو ويهه ٻه")
        self.assertEqual(num2words(3000, lang="sd"), "ٽي هزار")
        self.assertEqual(num2words(3333, lang="sd"), "ٽي هزار ٽي سو ٽيهه ٽي")
        self.assertEqual(num2words(4000, lang="sd"), "چار هزار")
        self.assertEqual(num2words(4444, lang="sd"), "چار هزار چار سو چاليهه چار")
        self.assertEqual(num2words(5000, lang="sd"), "پنج هزار")
        self.assertEqual(num2words(5555, lang="sd"), "پنج هزار پنج سو پنجاهه پنج")
        self.assertEqual(num2words(6000, lang="sd"), "ڇهه هزار")
        self.assertEqual(num2words(6666, lang="sd"), "ڇهه هزار ڇهه سو سٺ ڇهه")
        self.assertEqual(num2words(7000, lang="sd"), "ست هزار")
        self.assertEqual(num2words(7777, lang="sd"), "ست هزار ست سو ستر ست")
        self.assertEqual(num2words(8000, lang="sd"), "اٺ هزار")
        self.assertEqual(num2words(8888, lang="sd"), "اٺ هزار اٺ سو اسي اٺ")
        self.assertEqual(num2words(9000, lang="sd"), "نو هزار")
        self.assertEqual(num2words(9999, lang="sd"), "نو هزار نو سو نوي نو")
        self.assertEqual(num2words(10000, lang="sd"), "ڏهه هزار")
        self.assertEqual(num2words(10001, lang="sd"), "ڏهه هزار هڪ")
        self.assertEqual(num2words(11111, lang="sd"), "ڏهه هڪ هزار هڪ سو ڏهه هڪ")
        self.assertEqual(num2words(12345, lang="sd"), "ڏهه ٻه هزار ٽي سو چاليهه پنج")
        self.assertEqual(num2words(20000, lang="sd"), "ويهه هزار")
        self.assertEqual(num2words(50000, lang="sd"), "پنجاهه هزار")
        self.assertEqual(num2words(99999, lang="sd"), "نوي نو هزار نو سو نوي نو")
        self.assertEqual(num2words(100000, lang="sd"), "هڪ سو هزار")
        self.assertEqual(
            num2words(123456, lang="sd"), "هڪ سو ويهه ٽي هزار چار سو پنجاهه ڇهه"
        )
        self.assertEqual(num2words(200000, lang="sd"), "ٻه سو هزار")
        self.assertEqual(num2words(500000, lang="sd"), "پنج سو هزار")
        self.assertEqual(
            num2words(654321, lang="sd"), "ڇهه سو پنجاهه چار هزار ٽي سو ويهه هڪ"
        )
        self.assertEqual(num2words(999999, lang="sd"), "نو سو نوي نو هزار نو سو نوي نو")

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sd"), "هڪ لک")
        self.assertEqual(num2words(1000001, lang="sd"), "هڪ لک هڪ")
        self.assertEqual(
            num2words(1111111, lang="sd"), "هڪ لک هڪ سو ڏهه هڪ هزار هڪ سو ڏهه هڪ"
        )
        self.assertEqual(
            num2words(1234567, lang="sd"), "هڪ لک ٻه سو ٽيهه چار هزار پنج سو سٺ ست"
        )
        self.assertEqual(num2words(2000000, lang="sd"), "ٻه لک")
        self.assertEqual(num2words(5000000, lang="sd"), "پنج لک")
        self.assertEqual(
            num2words(9999999, lang="sd"), "نو لک نو سو نوي نو هزار نو سو نوي نو"
        )
        self.assertEqual(num2words(10000000, lang="sd"), "ڏهه لک")
        self.assertEqual(
            num2words(12345678, lang="sd"),
            "ڏهه ٻه لک ٽي سو چاليهه پنج هزار ڇهه سو ستر اٺ",
        )
        self.assertEqual(
            num2words(99999999, lang="sd"), "نوي نو لک نو سو نوي نو هزار نو سو نوي نو"
        )
        self.assertEqual(num2words(100000000, lang="sd"), "هڪ سو لک")
        self.assertEqual(
            num2words(123456789, lang="sd"),
            "هڪ سو ويهه ٽي لک چار سو پنجاهه ڇهه هزار ست سو اسي نو",
        )
        self.assertEqual(
            num2words(999999999, lang="sd"),
            "نو سو نوي نو لک نو سو نوي نو هزار نو سو نوي نو",
        )
        self.assertEqual(num2words(1000000000, lang="sd"), "1000000000")
        self.assertEqual(num2words(1234567890, lang="sd"), "1234567890")
        self.assertEqual(num2words(9999999999, lang="sd"), "9999999999")
        self.assertEqual(num2words(10000000000, lang="sd"), "10000000000")
        self.assertEqual(num2words(99999999999, lang="sd"), "99999999999")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sd"), "منفي هڪ")
        self.assertEqual(num2words(-2, lang="sd"), "منفي ٻه")
        self.assertEqual(num2words(-5, lang="sd"), "منفي پنج")
        self.assertEqual(num2words(-10, lang="sd"), "منفي ڏهه")
        self.assertEqual(num2words(-11, lang="sd"), "منفي ڏهه هڪ")
        self.assertEqual(num2words(-20, lang="sd"), "منفي ويهه")
        self.assertEqual(num2words(-50, lang="sd"), "منفي پنجاهه")
        self.assertEqual(num2words(-99, lang="sd"), "منفي نوي نو")
        self.assertEqual(num2words(-100, lang="sd"), "منفي هڪ سو")
        self.assertEqual(num2words(-101, lang="sd"), "منفي هڪ سو هڪ")
        self.assertEqual(num2words(-200, lang="sd"), "منفي ٻه سو")
        self.assertEqual(num2words(-999, lang="sd"), "منفي نو سو نوي نو")
        self.assertEqual(num2words(-1000, lang="sd"), "منفي هڪ هزار")
        self.assertEqual(num2words(-1001, lang="sd"), "منفي هڪ هزار هڪ")
        self.assertEqual(num2words(-10000, lang="sd"), "منفي ڏهه هزار")
        self.assertEqual(num2words(-100000, lang="sd"), "منفي هڪ سو هزار")
        self.assertEqual(num2words(-1000000, lang="sd"), "منفي هڪ لک")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sd"), "ٻڙي point هڪ")
        self.assertEqual(num2words(0.5, lang="sd"), "ٻڙي point پنج")
        self.assertEqual(num2words(0.9, lang="sd"), "ٻڙي point نو")
        self.assertEqual(num2words(1.1, lang="sd"), "هڪ point هڪ")
        self.assertEqual(num2words(1.5, lang="sd"), "هڪ point پنج")
        self.assertEqual(num2words(2.5, lang="sd"), "ٻه point پنج")
        self.assertEqual(num2words(3.14, lang="sd"), "ٽي point هڪ چار")
        self.assertEqual(num2words(10.5, lang="sd"), "ڏهه point پنج")
        self.assertEqual(num2words(11.11, lang="sd"), "ڏهه هڪ point هڪ هڪ")
        self.assertEqual(num2words(20.2, lang="sd"), "ويهه point ٻه")
        self.assertEqual(num2words(99.99, lang="sd"), "نوي نو point نو نو")
        self.assertEqual(num2words(100.01, lang="sd"), "هڪ سو point ٻڙي هڪ")
        self.assertEqual(num2words(100.5, lang="sd"), "هڪ سو point پنج")
        self.assertEqual(num2words(123.45, lang="sd"), "هڪ سو ويهه ٽي point چار پنج")
        self.assertEqual(num2words(1000.5, lang="sd"), "هڪ هزار point پنج")
        self.assertEqual(
            num2words(1234.56, lang="sd"), "هڪ هزار ٻه سو ٽيهه چار point پنج ڇهه"
        )
        self.assertEqual(num2words(10000.01, lang="sd"), "ڏهه هزار point ٻڙي هڪ")
        self.assertEqual(num2words(-0.5, lang="sd"), "منفي ٻڙي point پنج")
        self.assertEqual(num2words(-1.5, lang="sd"), "منفي هڪ point پنج")
        self.assertEqual(num2words(-10.5, lang="sd"), "منفي ڏهه point پنج")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sd", ordinal=True), "هڪ-و")
        self.assertEqual(num2words(2, lang="sd", ordinal=True), "ٻه-و")
        self.assertEqual(num2words(3, lang="sd", ordinal=True), "ٽي-و")
        self.assertEqual(num2words(4, lang="sd", ordinal=True), "چار-و")
        self.assertEqual(num2words(5, lang="sd", ordinal=True), "پنج-و")
        self.assertEqual(num2words(6, lang="sd", ordinal=True), "ڇهه-و")
        self.assertEqual(num2words(7, lang="sd", ordinal=True), "ست-و")
        self.assertEqual(num2words(8, lang="sd", ordinal=True), "اٺ-و")
        self.assertEqual(num2words(9, lang="sd", ordinal=True), "نو-و")
        self.assertEqual(num2words(10, lang="sd", ordinal=True), "ڏهه-و")
        self.assertEqual(num2words(11, lang="sd", ordinal=True), "ڏهه هڪ-و")
        self.assertEqual(num2words(12, lang="sd", ordinal=True), "ڏهه ٻه-و")
        self.assertEqual(num2words(13, lang="sd", ordinal=True), "ڏهه ٽي-و")
        self.assertEqual(num2words(14, lang="sd", ordinal=True), "ڏهه چار-و")
        self.assertEqual(num2words(15, lang="sd", ordinal=True), "ڏهه پنج-و")
        self.assertEqual(num2words(16, lang="sd", ordinal=True), "ڏهه ڇهه-و")
        self.assertEqual(num2words(17, lang="sd", ordinal=True), "ڏهه ست-و")
        self.assertEqual(num2words(18, lang="sd", ordinal=True), "ڏهه اٺ-و")
        self.assertEqual(num2words(19, lang="sd", ordinal=True), "ڏهه نو-و")
        self.assertEqual(num2words(20, lang="sd", ordinal=True), "ويهه-و")
        self.assertEqual(num2words(21, lang="sd", ordinal=True), "ويهه هڪ-و")
        self.assertEqual(num2words(22, lang="sd", ordinal=True), "ويهه ٻه-و")
        self.assertEqual(num2words(25, lang="sd", ordinal=True), "ويهه پنج-و")
        self.assertEqual(num2words(30, lang="sd", ordinal=True), "ٽيهه-و")
        self.assertEqual(num2words(40, lang="sd", ordinal=True), "چاليهه-و")
        self.assertEqual(num2words(50, lang="sd", ordinal=True), "پنجاهه-و")
        self.assertEqual(num2words(60, lang="sd", ordinal=True), "سٺ-و")
        self.assertEqual(num2words(70, lang="sd", ordinal=True), "ستر-و")
        self.assertEqual(num2words(80, lang="sd", ordinal=True), "اسي-و")
        self.assertEqual(num2words(90, lang="sd", ordinal=True), "نوي-و")
        self.assertEqual(num2words(100, lang="sd", ordinal=True), "هڪ سو-و")
        self.assertEqual(num2words(101, lang="sd", ordinal=True), "هڪ سو هڪ-و")
        self.assertEqual(num2words(200, lang="sd", ordinal=True), "ٻه سو-و")
        self.assertEqual(num2words(500, lang="sd", ordinal=True), "پنج سو-و")
        self.assertEqual(num2words(1000, lang="sd", ordinal=True), "هڪ هزار-و")
        self.assertEqual(num2words(1001, lang="sd", ordinal=True), "هڪ هزار هڪ-و")
        self.assertEqual(num2words(10000, lang="sd", ordinal=True), "ڏهه هزار-و")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sd", to="currency", currency="PKR"), "ٻڙي روپيا"
        )
        self.assertEqual(
            num2words(0.01, lang="sd", to="currency", currency="PKR"),
            "ٻڙي روپيا هڪ پئسو",
        )
        self.assertEqual(
            num2words(0.5, lang="sd", to="currency", currency="PKR"),
            "ٻڙي روپيا پنجاهه پئسا",
        )
        self.assertEqual(
            num2words(1, lang="sd", to="currency", currency="PKR"), "هڪ روپي"
        )
        self.assertEqual(
            num2words(1.5, lang="sd", to="currency", currency="PKR"),
            "هڪ روپي پنجاهه پئسا",
        )
        self.assertEqual(
            num2words(0, lang="sd", to="currency", currency="USD"), "ٻڙي dollars"
        )
        self.assertEqual(
            num2words(0.01, lang="sd", to="currency", currency="USD"),
            "ٻڙي dollars هڪ cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sd", to="currency", currency="USD"),
            "ٻڙي dollars پنجاهه cents",
        )
        self.assertEqual(
            num2words(1, lang="sd", to="currency", currency="USD"), "هڪ dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="sd", to="currency", currency="USD"),
            "هڪ dollar پنجاهه cents",
        )
        self.assertEqual(
            num2words(0, lang="sd", to="currency", currency="EUR"), "ٻڙي euros"
        )
        self.assertEqual(
            num2words(0.01, lang="sd", to="currency", currency="EUR"),
            "ٻڙي euros هڪ cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sd", to="currency", currency="EUR"),
            "ٻڙي euros پنجاهه cents",
        )
        self.assertEqual(
            num2words(1, lang="sd", to="currency", currency="EUR"), "هڪ euro"
        )
        self.assertEqual(
            num2words(1.5, lang="sd", to="currency", currency="EUR"),
            "هڪ euro پنجاهه cents",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sd", to="year"), "هڪ هزار")
        self.assertEqual(num2words(1066, lang="sd", to="year"), "هڪ هزار سٺ ڇهه")
        self.assertEqual(num2words(1492, lang="sd", to="year"), "هڪ هزار چار سو نوي ٻه")
        self.assertEqual(num2words(1776, lang="sd", to="year"), "هڪ هزار ست سو ستر ڇهه")
        self.assertEqual(num2words(1800, lang="sd", to="year"), "هڪ هزار اٺ سو")
        self.assertEqual(num2words(1900, lang="sd", to="year"), "هڪ هزار نو سو")
        self.assertEqual(num2words(1984, lang="sd", to="year"), "هڪ هزار نو سو اسي چار")
        self.assertEqual(num2words(1999, lang="sd", to="year"), "هڪ هزار نو سو نوي نو")
        self.assertEqual(num2words(2000, lang="sd", to="year"), "ٻه هزار")
        self.assertEqual(num2words(2001, lang="sd", to="year"), "ٻه هزار هڪ")
        self.assertEqual(num2words(2010, lang="sd", to="year"), "ٻه هزار ڏهه")
        self.assertEqual(num2words(2020, lang="sd", to="year"), "ٻه هزار ويهه")
        self.assertEqual(num2words(2024, lang="sd", to="year"), "ٻه هزار ويهه چار")
        self.assertEqual(num2words(2100, lang="sd", to="year"), "ٻه هزار هڪ سو")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sd"), "ٻڙي")
        self.assertEqual(num2words("1", lang="sd"), "هڪ")
        self.assertEqual(num2words("10", lang="sd"), "ڏهه")
        self.assertEqual(num2words("100", lang="sd"), "هڪ سو")
        self.assertEqual(num2words("1000", lang="sd"), "هڪ هزار")
        self.assertEqual(num2words("10000", lang="sd"), "ڏهه هزار")
        self.assertEqual(num2words("100000", lang="sd"), "هڪ سو هزار")
        self.assertEqual(num2words("1000000", lang="sd"), "هڪ لک")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sd"), "ٻڙي")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sd"), num2words("100", lang="sd"))
        self.assertEqual(num2words(1000, lang="sd"), num2words("1000", lang="sd"))

