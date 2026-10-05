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

from num2words2 import maxval, num2words


class Num2WordsKKTest(TestCase):
    """Comprehensive test cases for Kazakh language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="kk"), "нөл")
        self.assertEqual(num2words(1, lang="kk"), "бір")
        self.assertEqual(num2words(2, lang="kk"), "екі")
        self.assertEqual(num2words(3, lang="kk"), "үш")
        self.assertEqual(num2words(4, lang="kk"), "төрт")
        self.assertEqual(num2words(5, lang="kk"), "бес")
        self.assertEqual(num2words(6, lang="kk"), "алты")
        self.assertEqual(num2words(7, lang="kk"), "жеті")
        self.assertEqual(num2words(8, lang="kk"), "сегіз")
        self.assertEqual(num2words(9, lang="kk"), "тоғыз")
        self.assertEqual(num2words(10, lang="kk"), "он")
        self.assertEqual(num2words(11, lang="kk"), "он бір")
        self.assertEqual(num2words(12, lang="kk"), "он екі")
        self.assertEqual(num2words(13, lang="kk"), "он үш")
        self.assertEqual(num2words(14, lang="kk"), "он төрт")
        self.assertEqual(num2words(15, lang="kk"), "он бес")
        self.assertEqual(num2words(16, lang="kk"), "он алты")
        self.assertEqual(num2words(17, lang="kk"), "он жеті")
        self.assertEqual(num2words(18, lang="kk"), "он сегіз")
        self.assertEqual(num2words(19, lang="kk"), "он тоғыз")
        self.assertEqual(num2words(20, lang="kk"), "жиырма")
        self.assertEqual(num2words(21, lang="kk"), "жиырма бір")
        self.assertEqual(num2words(22, lang="kk"), "жиырма екі")
        self.assertEqual(num2words(23, lang="kk"), "жиырма үш")
        self.assertEqual(num2words(24, lang="kk"), "жиырма төрт")
        self.assertEqual(num2words(25, lang="kk"), "жиырма бес")
        self.assertEqual(num2words(26, lang="kk"), "жиырма алты")
        self.assertEqual(num2words(27, lang="kk"), "жиырма жеті")
        self.assertEqual(num2words(28, lang="kk"), "жиырма сегіз")
        self.assertEqual(num2words(29, lang="kk"), "жиырма тоғыз")
        self.assertEqual(num2words(30, lang="kk"), "отыз")
        self.assertEqual(num2words(31, lang="kk"), "отыз бір")
        self.assertEqual(num2words(35, lang="kk"), "отыз бес")
        self.assertEqual(num2words(40, lang="kk"), "қырық")
        self.assertEqual(num2words(45, lang="kk"), "қырық бес")
        self.assertEqual(num2words(50, lang="kk"), "елу")
        self.assertEqual(num2words(55, lang="kk"), "елу бес")
        self.assertEqual(num2words(60, lang="kk"), "алпыс")
        self.assertEqual(num2words(65, lang="kk"), "алпыс бес")
        self.assertEqual(num2words(70, lang="kk"), "жетпіс")
        self.assertEqual(num2words(75, lang="kk"), "жетпіс бес")
        self.assertEqual(num2words(80, lang="kk"), "сексен")
        self.assertEqual(num2words(85, lang="kk"), "сексен бес")
        self.assertEqual(num2words(90, lang="kk"), "тоқсан")
        self.assertEqual(num2words(95, lang="kk"), "тоқсан бес")
        self.assertEqual(num2words(99, lang="kk"), "тоқсан тоғыз")
        self.assertEqual(num2words(100, lang="kk"), "бір жүз")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="kk"), "бір жүз бір")
        self.assertEqual(num2words(110, lang="kk"), "бір жүз он")
        self.assertEqual(num2words(111, lang="kk"), "бір жүз он бір")
        self.assertEqual(num2words(120, lang="kk"), "бір жүз жиырма")
        self.assertEqual(num2words(125, lang="kk"), "бір жүз жиырма бес")
        self.assertEqual(num2words(150, lang="kk"), "бір жүз елу")
        self.assertEqual(num2words(175, lang="kk"), "бір жүз жетпіс бес")
        self.assertEqual(num2words(199, lang="kk"), "бір жүз тоқсан тоғыз")
        self.assertEqual(num2words(200, lang="kk"), "екі жүз")
        self.assertEqual(num2words(201, lang="kk"), "екі жүз бір")
        self.assertEqual(num2words(210, lang="kk"), "екі жүз он")
        self.assertEqual(num2words(220, lang="kk"), "екі жүз жиырма")
        self.assertEqual(num2words(250, lang="kk"), "екі жүз елу")
        self.assertEqual(num2words(299, lang="kk"), "екі жүз тоқсан тоғыз")
        self.assertEqual(num2words(300, lang="kk"), "үш жүз")
        self.assertEqual(num2words(333, lang="kk"), "үш жүз отыз үш")
        self.assertEqual(num2words(400, lang="kk"), "төрт жүз")
        self.assertEqual(num2words(444, lang="kk"), "төрт жүз қырық төрт")
        self.assertEqual(num2words(500, lang="kk"), "бес жүз")
        self.assertEqual(num2words(555, lang="kk"), "бес жүз елу бес")
        self.assertEqual(num2words(600, lang="kk"), "алты жүз")
        self.assertEqual(num2words(666, lang="kk"), "алты жүз алпыс алты")
        self.assertEqual(num2words(700, lang="kk"), "жеті жүз")
        self.assertEqual(num2words(777, lang="kk"), "жеті жүз жетпіс жеті")
        self.assertEqual(num2words(800, lang="kk"), "сегіз жүз")
        self.assertEqual(num2words(888, lang="kk"), "сегіз жүз сексен сегіз")
        self.assertEqual(num2words(900, lang="kk"), "тоғыз жүз")
        self.assertEqual(num2words(999, lang="kk"), "тоғыз жүз тоқсан тоғыз")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="kk"), "бір мың")
        self.assertEqual(num2words(1001, lang="kk"), "бір мың бір")
        self.assertEqual(num2words(1010, lang="kk"), "бір мың он")
        self.assertEqual(num2words(1100, lang="kk"), "бір мың бір жүз")
        self.assertEqual(num2words(1111, lang="kk"), "бір мың бір жүз он бір")
        self.assertEqual(num2words(1234, lang="kk"), "бір мың екі жүз отыз төрт")
        self.assertEqual(num2words(1500, lang="kk"), "бір мың бес жүз")
        self.assertEqual(num2words(1999, lang="kk"), "бір мың тоғыз жүз тоқсан тоғыз")
        self.assertEqual(num2words(2000, lang="kk"), "екі мың")
        self.assertEqual(num2words(2001, lang="kk"), "екі мың бір")
        self.assertEqual(num2words(2020, lang="kk"), "екі мың жиырма")
        self.assertEqual(num2words(2222, lang="kk"), "екі мың екі жүз жиырма екі")
        self.assertEqual(num2words(3000, lang="kk"), "үш мың")
        self.assertEqual(num2words(3333, lang="kk"), "үш мың үш жүз отыз үш")
        self.assertEqual(num2words(4000, lang="kk"), "төрт мың")
        self.assertEqual(num2words(4444, lang="kk"), "төрт мың төрт жүз қырық төрт")
        self.assertEqual(num2words(5000, lang="kk"), "бес мың")
        self.assertEqual(num2words(5555, lang="kk"), "бес мың бес жүз елу бес")
        self.assertEqual(num2words(6000, lang="kk"), "алты мың")
        self.assertEqual(num2words(6666, lang="kk"), "алты мың алты жүз алпыс алты")
        self.assertEqual(num2words(7000, lang="kk"), "жеті мың")
        self.assertEqual(num2words(7777, lang="kk"), "жеті мың жеті жүз жетпіс жеті")
        self.assertEqual(num2words(8000, lang="kk"), "сегіз мың")
        self.assertEqual(num2words(8888, lang="kk"), "сегіз мың сегіз жүз сексен сегіз")
        self.assertEqual(num2words(9000, lang="kk"), "тоғыз мың")
        self.assertEqual(num2words(9999, lang="kk"), "тоғыз мың тоғыз жүз тоқсан тоғыз")
        self.assertEqual(num2words(10000, lang="kk"), "он мың")
        self.assertEqual(num2words(10001, lang="kk"), "он мың бір")
        self.assertEqual(num2words(11111, lang="kk"), "он бір мың бір жүз он бір")
        self.assertEqual(num2words(12345, lang="kk"), "он екі мың үш жүз қырық бес")
        self.assertEqual(num2words(20000, lang="kk"), "жиырма мың")
        self.assertEqual(num2words(50000, lang="kk"), "елу мың")
        self.assertEqual(
            num2words(99999, lang="kk"), "тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз"
        )
        self.assertEqual(num2words(100000, lang="kk"), "бір жүз мың")
        self.assertEqual(
            num2words(123456, lang="kk"), "бір жүз жиырма үш мың төрт жүз елу алты"
        )
        self.assertEqual(num2words(200000, lang="kk"), "екі жүз мың")
        self.assertEqual(num2words(500000, lang="kk"), "бес жүз мың")
        self.assertEqual(
            num2words(654321, lang="kk"), "алты жүз елу төрт мың үш жүз жиырма бір"
        )
        self.assertEqual(
            num2words(999999, lang="kk"),
            "тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="kk"), "бір миллион")
        self.assertEqual(num2words(1000001, lang="kk"), "бір миллион бір")
        self.assertEqual(
            num2words(1111111, lang="kk"),
            "бір миллион бір жүз он бір мың бір жүз он бір",
        )
        self.assertEqual(
            num2words(1234567, lang="kk"),
            "бір миллион екі жүз отыз төрт мың бес жүз алпыс жеті",
        )
        self.assertEqual(num2words(2000000, lang="kk"), "екі миллион")
        self.assertEqual(num2words(5000000, lang="kk"), "бес миллион")
        self.assertEqual(
            num2words(9999999, lang="kk"),
            "тоғыз миллион тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз",
        )
        self.assertEqual(num2words(10000000, lang="kk"), "он миллион")
        self.assertEqual(
            num2words(12345678, lang="kk"),
            "он екі миллион үш жүз қырық бес мың алты жүз жетпіс сегіз",
        )
        self.assertEqual(
            num2words(99999999, lang="kk"),
            "тоқсан тоғыз миллион тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз",
        )
        self.assertEqual(num2words(100000000, lang="kk"), "бір жүз миллион")
        self.assertEqual(
            num2words(123456789, lang="kk"),
            "бір жүз жиырма үш миллион төрт жүз елу алты мың жеті жүз сексен тоғыз",
        )
        self.assertEqual(
            num2words(999999999, lang="kk"),
            "тоғыз жүз тоқсан тоғыз миллион тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз",
        )
        self.assertEqual(num2words(1000000000, lang="kk"), "бір миллиард")
        self.assertEqual(num2words(1234567890, lang="kk"), "бір миллиард екі жүз отыз төрт миллион бес жүз алпыс жеті мың сегіз жүз тоқсан")
        self.assertEqual(num2words(9999999999, lang="kk"), "тоғыз миллиард тоғыз жүз тоқсан тоғыз миллион тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз")
        self.assertEqual(num2words(10000000000, lang="kk"), "он миллиард")
        self.assertEqual(num2words(99999999999, lang="kk"), "тоқсан тоғыз миллиард тоғыз жүз тоқсан тоғыз миллион тоғыз жүз тоқсан тоғыз мың тоғыз жүз тоқсан тоғыз")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="kk"), "минус бір")
        self.assertEqual(num2words(-2, lang="kk"), "минус екі")
        self.assertEqual(num2words(-5, lang="kk"), "минус бес")
        self.assertEqual(num2words(-10, lang="kk"), "минус он")
        self.assertEqual(num2words(-11, lang="kk"), "минус он бір")
        self.assertEqual(num2words(-20, lang="kk"), "минус жиырма")
        self.assertEqual(num2words(-50, lang="kk"), "минус елу")
        self.assertEqual(num2words(-99, lang="kk"), "минус тоқсан тоғыз")
        self.assertEqual(num2words(-100, lang="kk"), "минус бір жүз")
        self.assertEqual(num2words(-101, lang="kk"), "минус бір жүз бір")
        self.assertEqual(num2words(-200, lang="kk"), "минус екі жүз")
        self.assertEqual(num2words(-999, lang="kk"), "минус тоғыз жүз тоқсан тоғыз")
        self.assertEqual(num2words(-1000, lang="kk"), "минус бір мың")
        self.assertEqual(num2words(-1001, lang="kk"), "минус бір мың бір")
        self.assertEqual(num2words(-10000, lang="kk"), "минус он мың")
        self.assertEqual(num2words(-100000, lang="kk"), "минус бір жүз мың")
        self.assertEqual(num2words(-1000000, lang="kk"), "минус бір миллион")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="kk"), "нөл бүтін бір")
        self.assertEqual(num2words(0.5, lang="kk"), "нөл бүтін бес")
        self.assertEqual(num2words(0.9, lang="kk"), "нөл бүтін тоғыз")
        self.assertEqual(num2words(1.1, lang="kk"), "бір бүтін бір")
        self.assertEqual(num2words(1.5, lang="kk"), "бір бүтін бес")
        self.assertEqual(num2words(2.5, lang="kk"), "екі бүтін бес")
        self.assertEqual(num2words(3.14, lang="kk"), "үш бүтін он төрт")
        self.assertEqual(num2words(10.5, lang="kk"), "он бүтін бес")
        self.assertEqual(num2words(11.11, lang="kk"), "он бір бүтін он бір")
        self.assertEqual(num2words(20.2, lang="kk"), "жиырма бүтін екі")
        self.assertEqual(num2words(99.99, lang="kk"), "тоқсан тоғыз бүтін тоқсан тоғыз")
        self.assertEqual(num2words(100.01, lang="kk"), "бір жүз бүтін нөл бір")
        self.assertEqual(num2words(100.5, lang="kk"), "бір жүз бүтін бес")
        self.assertEqual(
            num2words(123.45, lang="kk"), "бір жүз жиырма үш бүтін қырық бес"
        )
        self.assertEqual(num2words(1000.5, lang="kk"), "бір мың бүтін бес")
        self.assertEqual(
            num2words(1234.56, lang="kk"), "бір мың екі жүз отыз төрт бүтін елу алты"
        )
        self.assertEqual(num2words(10000.01, lang="kk"), "он мың бүтін нөл бір")
        self.assertEqual(num2words(-0.5, lang="kk"), "минус нөл бүтін бес")
        self.assertEqual(num2words(-1.5, lang="kk"), "минус бір бүтін бес")
        self.assertEqual(num2words(-10.5, lang="kk"), "минус он бүтін бес")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="kk", ordinal=True), "бірінші")
        self.assertEqual(num2words(2, lang="kk", ordinal=True), "екінші")
        self.assertEqual(num2words(3, lang="kk", ordinal=True), "үшінші")
        self.assertEqual(num2words(4, lang="kk", ordinal=True), "төртінші")
        self.assertEqual(num2words(5, lang="kk", ordinal=True), "бесінші")
        self.assertEqual(num2words(6, lang="kk", ordinal=True), "алтыншы")
        self.assertEqual(num2words(7, lang="kk", ordinal=True), "жетінші")
        self.assertEqual(num2words(8, lang="kk", ordinal=True), "сегізінші")
        self.assertEqual(num2words(9, lang="kk", ordinal=True), "тоғызыншы")
        self.assertEqual(num2words(10, lang="kk", ordinal=True), "оныншы")
        self.assertEqual(num2words(11, lang="kk", ordinal=True), "он бірінші")
        self.assertEqual(num2words(12, lang="kk", ordinal=True), "он екінші")
        self.assertEqual(num2words(13, lang="kk", ordinal=True), "он үшінші")
        self.assertEqual(num2words(14, lang="kk", ordinal=True), "он төртінші")
        self.assertEqual(num2words(15, lang="kk", ordinal=True), "он бесінші")
        self.assertEqual(num2words(16, lang="kk", ordinal=True), "он алтыншы")
        self.assertEqual(num2words(17, lang="kk", ordinal=True), "он жетінші")
        self.assertEqual(num2words(18, lang="kk", ordinal=True), "он сегізінші")
        self.assertEqual(num2words(19, lang="kk", ordinal=True), "он тоғызыншы")
        self.assertEqual(num2words(20, lang="kk", ordinal=True), "жиырманшы")
        self.assertEqual(num2words(21, lang="kk", ordinal=True), "жиырма бірінші")
        self.assertEqual(num2words(22, lang="kk", ordinal=True), "жиырма екінші")
        self.assertEqual(num2words(25, lang="kk", ordinal=True), "жиырма бесінші")
        self.assertEqual(num2words(30, lang="kk", ordinal=True), "отызыншы")
        self.assertEqual(num2words(40, lang="kk", ordinal=True), "қырықінші")
        self.assertEqual(num2words(50, lang="kk", ordinal=True), "елуінші")
        self.assertEqual(num2words(60, lang="kk", ordinal=True), "алпысыншы")
        self.assertEqual(num2words(70, lang="kk", ordinal=True), "жетпісінші")
        self.assertEqual(num2words(80, lang="kk", ordinal=True), "сексенінші")
        self.assertEqual(num2words(90, lang="kk", ordinal=True), "тоқсаныншы")
        self.assertEqual(num2words(100, lang="kk", ordinal=True), "бір жүзінші")
        self.assertEqual(num2words(101, lang="kk", ordinal=True), "бір жүз бірінші")
        self.assertEqual(num2words(200, lang="kk", ordinal=True), "екі жүзінші")
        self.assertEqual(num2words(500, lang="kk", ordinal=True), "бес жүзінші")
        self.assertEqual(num2words(1000, lang="kk", ordinal=True), "бір мыңыншы")
        self.assertEqual(num2words(1001, lang="kk", ordinal=True), "бір мың бірінші")
        self.assertEqual(num2words(10000, lang="kk", ordinal=True), "он мыңыншы")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="kk", to="currency", currency="KZT"), "нөл теңге"
        )
        self.assertEqual(
            num2words(0.01, lang="kk", to="currency", currency="KZT"),
            "нөл теңге, бір тиын",
        )
        self.assertEqual(
            num2words(0.5, lang="kk", to="currency", currency="KZT"),
            "нөл теңге, елу тиын",
        )
        self.assertEqual(
            num2words(1, lang="kk", to="currency", currency="KZT"), "бір теңге"
        )
        self.assertEqual(
            num2words(1.5, lang="kk", to="currency", currency="KZT"),
            "бір теңге, елу тиын",
        )
        self.assertEqual(
            num2words(0, lang="kk", to="currency", currency="USD"), "нөл доллар"
        )
        self.assertEqual(
            num2words(0.01, lang="kk", to="currency", currency="USD"),
            "нөл доллар, бір цент",
        )
        self.assertEqual(
            num2words(0.5, lang="kk", to="currency", currency="USD"),
            "нөл доллар, елу цент",
        )
        self.assertEqual(
            num2words(1, lang="kk", to="currency", currency="USD"), "бір доллар"
        )
        self.assertEqual(
            num2words(1.5, lang="kk", to="currency", currency="USD"),
            "бір доллар, елу цент",
        )
        self.assertEqual(
            num2words(0, lang="kk", to="currency", currency="EUR"), "нөл еуро"
        )
        self.assertEqual(
            num2words(0.01, lang="kk", to="currency", currency="EUR"),
            "нөл еуро, бір цент",
        )
        self.assertEqual(
            num2words(0.5, lang="kk", to="currency", currency="EUR"),
            "нөл еуро, елу цент",
        )
        self.assertEqual(
            num2words(1, lang="kk", to="currency", currency="EUR"), "бір еуро"
        )
        self.assertEqual(
            num2words(1.5, lang="kk", to="currency", currency="EUR"),
            "бір еуро, елу цент",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="kk", to="year"), "бір мың")
        self.assertEqual(num2words(1066, lang="kk", to="year"), "бір мың алпыс алты")
        self.assertEqual(
            num2words(1492, lang="kk", to="year"), "бір мың төрт жүз тоқсан екі"
        )
        self.assertEqual(
            num2words(1776, lang="kk", to="year"), "бір мың жеті жүз жетпіс алты"
        )
        self.assertEqual(num2words(1800, lang="kk", to="year"), "бір мың сегіз жүз")
        self.assertEqual(num2words(1900, lang="kk", to="year"), "бір мың тоғыз жүз")
        self.assertEqual(
            num2words(1984, lang="kk", to="year"), "бір мың тоғыз жүз сексен төрт"
        )
        self.assertEqual(
            num2words(1999, lang="kk", to="year"), "бір мың тоғыз жүз тоқсан тоғыз"
        )
        self.assertEqual(num2words(2000, lang="kk", to="year"), "екі мың")
        self.assertEqual(num2words(2001, lang="kk", to="year"), "екі мың бір")
        self.assertEqual(num2words(2010, lang="kk", to="year"), "екі мың он")
        self.assertEqual(num2words(2020, lang="kk", to="year"), "екі мың жиырма")
        self.assertEqual(num2words(2024, lang="kk", to="year"), "екі мың жиырма төрт")
        self.assertEqual(num2words(2100, lang="kk", to="year"), "екі мың бір жүз")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="kk"), "нөл")
        self.assertEqual(num2words("1", lang="kk"), "бір")
        self.assertEqual(num2words("10", lang="kk"), "он")
        self.assertEqual(num2words("100", lang="kk"), "бір жүз")
        self.assertEqual(num2words("1000", lang="kk"), "бір мың")
        self.assertEqual(num2words("10000", lang="kk"), "он мың")
        self.assertEqual(num2words("100000", lang="kk"), "бір жүз мың")
        self.assertEqual(num2words("1000000", lang="kk"), "бір миллион")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="kk"), "нөл")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="kk"), num2words("100", lang="kk"))
        self.assertEqual(num2words(1000, lang="kk"), num2words("1000", lang="kk"))


    def test_native_zero_minus_decimal(self):
        # gladiaio/num2words2#154, #166: these used to be English
        # ("zero", "minus бір", "бір point бес").
        self.assertEqual(num2words(0, lang="kk"), "нөл")
        self.assertEqual(num2words(-1, lang="kk"), "минус бір")
        self.assertEqual(num2words(1.5, lang="kk"), "бір бүтін бес")
        self.assertEqual(num2words(0, lang="kk", to="ordinal"), "нөлінші")

    def test_ordinals_are_not_hyphen_glued(self):
        # Part of #148: used to be "бір-інші".
        self.assertEqual(num2words(1, lang="kk", to="ordinal"), "бірінші")
        self.assertEqual(num2words(6, lang="kk", to="ordinal"), "алтыншы")
        self.assertEqual(num2words(1, lang="kk", to="ordinal_num"), "1.")

    def test_billions_are_words(self):
        # Used to fall back to the digit string at 10**9.
        self.assertEqual(num2words(10**9, lang="kk"), "бір миллиард")

    def test_overflow(self):
        self.assertEqual(maxval("kk"), 10**33)
        self.assertTrue(num2words(10**33 - 1, lang="kk").startswith("тоғыз жүз"))
        with self.assertRaises(OverflowError):
            num2words(10**33, lang="kk")

    def test_currency_default_and_unknown_code(self):
        self.assertEqual(num2words(1, lang="kk", to="currency"), "бір теңге")
        # Used to fall back to теңге silently.
        with self.assertRaises(NotImplementedError):
            num2words(1, lang="kk", to="currency", currency="GBP")
