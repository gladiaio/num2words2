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

import pytest

from num2words2 import num2words


class Num2WordsGLTest(TestCase):
    """Comprehensive test cases for Galician language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="gl"), "cero")
        self.assertEqual(num2words(1, lang="gl"), "un")
        self.assertEqual(num2words(2, lang="gl"), "dous")
        self.assertEqual(num2words(3, lang="gl"), "tres")
        self.assertEqual(num2words(4, lang="gl"), "catro")
        self.assertEqual(num2words(5, lang="gl"), "cinco")
        self.assertEqual(num2words(6, lang="gl"), "seis")
        self.assertEqual(num2words(7, lang="gl"), "sete")
        self.assertEqual(num2words(8, lang="gl"), "oito")
        self.assertEqual(num2words(9, lang="gl"), "nove")
        self.assertEqual(num2words(10, lang="gl"), "dez")
        self.assertEqual(num2words(11, lang="gl"), "dez un")
        self.assertEqual(num2words(12, lang="gl"), "dez dous")
        self.assertEqual(num2words(13, lang="gl"), "dez tres")
        self.assertEqual(num2words(14, lang="gl"), "dez catro")
        self.assertEqual(num2words(15, lang="gl"), "dez cinco")
        self.assertEqual(num2words(16, lang="gl"), "dez seis")
        self.assertEqual(num2words(17, lang="gl"), "dez sete")
        self.assertEqual(num2words(18, lang="gl"), "dez oito")
        self.assertEqual(num2words(19, lang="gl"), "dez nove")
        self.assertEqual(num2words(20, lang="gl"), "vinte")
        self.assertEqual(num2words(21, lang="gl"), "vinte un")
        self.assertEqual(num2words(22, lang="gl"), "vinte dous")
        self.assertEqual(num2words(23, lang="gl"), "vinte tres")
        self.assertEqual(num2words(24, lang="gl"), "vinte catro")
        self.assertEqual(num2words(25, lang="gl"), "vinte cinco")
        self.assertEqual(num2words(26, lang="gl"), "vinte seis")
        self.assertEqual(num2words(27, lang="gl"), "vinte sete")
        self.assertEqual(num2words(28, lang="gl"), "vinte oito")
        self.assertEqual(num2words(29, lang="gl"), "vinte nove")
        self.assertEqual(num2words(30, lang="gl"), "trinta")
        self.assertEqual(num2words(31, lang="gl"), "trinta un")
        self.assertEqual(num2words(35, lang="gl"), "trinta cinco")
        self.assertEqual(num2words(40, lang="gl"), "corenta")
        self.assertEqual(num2words(45, lang="gl"), "corenta cinco")
        self.assertEqual(num2words(50, lang="gl"), "cincuenta")
        self.assertEqual(num2words(55, lang="gl"), "cincuenta cinco")
        self.assertEqual(num2words(60, lang="gl"), "sesenta")
        self.assertEqual(num2words(65, lang="gl"), "sesenta cinco")
        self.assertEqual(num2words(70, lang="gl"), "setenta")
        self.assertEqual(num2words(75, lang="gl"), "setenta cinco")
        self.assertEqual(num2words(80, lang="gl"), "oitenta")
        self.assertEqual(num2words(85, lang="gl"), "oitenta cinco")
        self.assertEqual(num2words(90, lang="gl"), "noventa")
        self.assertEqual(num2words(95, lang="gl"), "noventa cinco")
        self.assertEqual(num2words(99, lang="gl"), "noventa nove")
        self.assertEqual(num2words(100, lang="gl"), "un cento")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="gl"), "un cento un")
        self.assertEqual(num2words(110, lang="gl"), "un cento dez")
        self.assertEqual(num2words(111, lang="gl"), "un cento dez un")
        self.assertEqual(num2words(120, lang="gl"), "un cento vinte")
        self.assertEqual(num2words(125, lang="gl"), "un cento vinte cinco")
        self.assertEqual(num2words(150, lang="gl"), "un cento cincuenta")
        self.assertEqual(num2words(175, lang="gl"), "un cento setenta cinco")
        self.assertEqual(num2words(199, lang="gl"), "un cento noventa nove")
        self.assertEqual(num2words(200, lang="gl"), "dous cento")
        self.assertEqual(num2words(201, lang="gl"), "dous cento un")
        self.assertEqual(num2words(210, lang="gl"), "dous cento dez")
        self.assertEqual(num2words(220, lang="gl"), "dous cento vinte")
        self.assertEqual(num2words(250, lang="gl"), "dous cento cincuenta")
        self.assertEqual(num2words(299, lang="gl"), "dous cento noventa nove")
        self.assertEqual(num2words(300, lang="gl"), "tres cento")
        self.assertEqual(num2words(333, lang="gl"), "tres cento trinta tres")
        self.assertEqual(num2words(400, lang="gl"), "catro cento")
        self.assertEqual(num2words(444, lang="gl"), "catro cento corenta catro")
        self.assertEqual(num2words(500, lang="gl"), "cinco cento")
        self.assertEqual(num2words(555, lang="gl"), "cinco cento cincuenta cinco")
        self.assertEqual(num2words(600, lang="gl"), "seis cento")
        self.assertEqual(num2words(666, lang="gl"), "seis cento sesenta seis")
        self.assertEqual(num2words(700, lang="gl"), "sete cento")
        self.assertEqual(num2words(777, lang="gl"), "sete cento setenta sete")
        self.assertEqual(num2words(800, lang="gl"), "oito cento")
        self.assertEqual(num2words(888, lang="gl"), "oito cento oitenta oito")
        self.assertEqual(num2words(900, lang="gl"), "nove cento")
        self.assertEqual(num2words(999, lang="gl"), "nove cento noventa nove")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="gl"), "un mil")
        self.assertEqual(num2words(1001, lang="gl"), "un mil un")
        self.assertEqual(num2words(1010, lang="gl"), "un mil dez")
        self.assertEqual(num2words(1100, lang="gl"), "un mil un cento")
        self.assertEqual(num2words(1111, lang="gl"), "un mil un cento dez un")
        self.assertEqual(num2words(1234, lang="gl"), "un mil dous cento trinta catro")
        self.assertEqual(num2words(1500, lang="gl"), "un mil cinco cento")
        self.assertEqual(num2words(1999, lang="gl"), "un mil nove cento noventa nove")
        self.assertEqual(num2words(2000, lang="gl"), "dous mil")
        self.assertEqual(num2words(2001, lang="gl"), "dous mil un")
        self.assertEqual(num2words(2020, lang="gl"), "dous mil vinte")
        self.assertEqual(num2words(2222, lang="gl"), "dous mil dous cento vinte dous")
        self.assertEqual(num2words(3000, lang="gl"), "tres mil")
        self.assertEqual(num2words(3333, lang="gl"), "tres mil tres cento trinta tres")
        self.assertEqual(num2words(4000, lang="gl"), "catro mil")
        self.assertEqual(
            num2words(4444, lang="gl"), "catro mil catro cento corenta catro"
        )
        self.assertEqual(num2words(5000, lang="gl"), "cinco mil")
        self.assertEqual(
            num2words(5555, lang="gl"), "cinco mil cinco cento cincuenta cinco"
        )
        self.assertEqual(num2words(6000, lang="gl"), "seis mil")
        self.assertEqual(num2words(6666, lang="gl"), "seis mil seis cento sesenta seis")
        self.assertEqual(num2words(7000, lang="gl"), "sete mil")
        self.assertEqual(num2words(7777, lang="gl"), "sete mil sete cento setenta sete")
        self.assertEqual(num2words(8000, lang="gl"), "oito mil")
        self.assertEqual(num2words(8888, lang="gl"), "oito mil oito cento oitenta oito")
        self.assertEqual(num2words(9000, lang="gl"), "nove mil")
        self.assertEqual(num2words(9999, lang="gl"), "nove mil nove cento noventa nove")
        self.assertEqual(num2words(10000, lang="gl"), "dez mil")
        self.assertEqual(num2words(10001, lang="gl"), "dez mil un")
        self.assertEqual(num2words(11111, lang="gl"), "dez un mil un cento dez un")
        self.assertEqual(
            num2words(12345, lang="gl"), "dez dous mil tres cento corenta cinco"
        )
        self.assertEqual(num2words(20000, lang="gl"), "vinte mil")
        self.assertEqual(num2words(50000, lang="gl"), "cincuenta mil")
        self.assertEqual(
            num2words(99999, lang="gl"), "noventa nove mil nove cento noventa nove"
        )
        self.assertEqual(num2words(100000, lang="gl"), "un cento mil")
        self.assertEqual(
            num2words(123456, lang="gl"),
            "un cento vinte tres mil catro cento cincuenta seis",
        )
        self.assertEqual(num2words(200000, lang="gl"), "dous cento mil")
        self.assertEqual(num2words(500000, lang="gl"), "cinco cento mil")
        self.assertEqual(
            num2words(654321, lang="gl"),
            "seis cento cincuenta catro mil tres cento vinte un",
        )
        self.assertEqual(
            num2words(999999, lang="gl"),
            "nove cento noventa nove mil nove cento noventa nove",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="gl"), "un millón")
        self.assertEqual(num2words(1000001, lang="gl"), "un millón un")
        self.assertEqual(
            num2words(1111111, lang="gl"),
            "un millón un cento dez un mil un cento dez un",
        )
        self.assertEqual(
            num2words(1234567, lang="gl"),
            "un millón dous cento trinta catro mil cinco cento sesenta sete",
        )
        self.assertEqual(num2words(2000000, lang="gl"), "dous millón")
        self.assertEqual(num2words(5000000, lang="gl"), "cinco millón")
        self.assertEqual(
            num2words(9999999, lang="gl"),
            "nove millón nove cento noventa nove mil nove cento noventa nove",
        )
        self.assertEqual(num2words(10000000, lang="gl"), "dez millón")
        self.assertEqual(
            num2words(12345678, lang="gl"),
            "dez dous millón tres cento corenta cinco mil seis cento setenta oito",
        )
        self.assertEqual(
            num2words(99999999, lang="gl"),
            "noventa nove millón nove cento noventa nove mil nove cento noventa nove",
        )
        self.assertEqual(num2words(100000000, lang="gl"), "un cento millón")
        self.assertEqual(
            num2words(123456789, lang="gl"),
            "un cento vinte tres millón catro cento cincuenta seis mil sete cento oitenta nove",
        )
        self.assertEqual(
            num2words(999999999, lang="gl"),
            "nove cento noventa nove millón nove cento noventa nove mil nove cento noventa nove",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="gl"),
            "un mil millón",
        )
        self.assertEqual(
            num2words(1234567890, lang="gl"),
            "un mil dous cento trinta catro millón cinco cento sesenta sete mil oito cento noventa",
        )
        self.assertEqual(
            num2words(9999999999, lang="gl"),
            "nove mil nove cento noventa nove millón nove cento noventa nove mil nove cento noventa nove",
        )
        self.assertEqual(
            num2words(10000000000, lang="gl"),
            "dez mil millón",
        )
        self.assertEqual(
            num2words(99999999999, lang="gl"),
            "noventa nove mil nove cento noventa nove millón nove cento noventa nove mil nove cento noventa nove",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="gl"), "menos un")
        self.assertEqual(num2words(-2, lang="gl"), "menos dous")
        self.assertEqual(num2words(-5, lang="gl"), "menos cinco")
        self.assertEqual(num2words(-10, lang="gl"), "menos dez")
        self.assertEqual(num2words(-11, lang="gl"), "menos dez un")
        self.assertEqual(num2words(-20, lang="gl"), "menos vinte")
        self.assertEqual(num2words(-50, lang="gl"), "menos cincuenta")
        self.assertEqual(num2words(-99, lang="gl"), "menos noventa nove")
        self.assertEqual(num2words(-100, lang="gl"), "menos un cento")
        self.assertEqual(num2words(-101, lang="gl"), "menos un cento un")
        self.assertEqual(num2words(-200, lang="gl"), "menos dous cento")
        self.assertEqual(num2words(-999, lang="gl"), "menos nove cento noventa nove")
        self.assertEqual(num2words(-1000, lang="gl"), "menos un mil")
        self.assertEqual(num2words(-1001, lang="gl"), "menos un mil un")
        self.assertEqual(num2words(-10000, lang="gl"), "menos dez mil")
        self.assertEqual(num2words(-100000, lang="gl"), "menos un cento mil")
        self.assertEqual(num2words(-1000000, lang="gl"), "menos un millón")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="gl"), "cero coma un")
        self.assertEqual(num2words(0.5, lang="gl"), "cero coma cinco")
        self.assertEqual(num2words(0.9, lang="gl"), "cero coma nove")
        self.assertEqual(num2words(1.1, lang="gl"), "un coma un")
        self.assertEqual(num2words(1.5, lang="gl"), "un coma cinco")
        self.assertEqual(num2words(2.5, lang="gl"), "dous coma cinco")
        self.assertEqual(num2words(3.14, lang="gl"), "tres coma un catro")
        self.assertEqual(num2words(10.5, lang="gl"), "dez coma cinco")
        self.assertEqual(num2words(11.11, lang="gl"), "dez un coma un un")
        self.assertEqual(num2words(20.2, lang="gl"), "vinte coma dous")
        self.assertEqual(num2words(99.99, lang="gl"), "noventa nove coma nove nove")
        self.assertEqual(num2words(100.01, lang="gl"), "un cento coma cero un")
        self.assertEqual(num2words(100.5, lang="gl"), "un cento coma cinco")
        self.assertEqual(
            num2words(123.45, lang="gl"), "un cento vinte tres coma catro cinco"
        )
        self.assertEqual(num2words(1000.5, lang="gl"), "un mil coma cinco")
        self.assertEqual(
            num2words(1234.56, lang="gl"),
            "un mil dous cento trinta catro coma cinco seis",
        )
        self.assertEqual(num2words(10000.01, lang="gl"), "dez mil coma cero un")
        self.assertEqual(num2words(-0.5, lang="gl"), "menos cero coma cinco")
        self.assertEqual(num2words(-1.5, lang="gl"), "menos un coma cinco")
        self.assertEqual(num2words(-10.5, lang="gl"), "menos dez coma cinco")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="gl", ordinal=True), "un-o")
        self.assertEqual(num2words(2, lang="gl", ordinal=True), "dous-o")
        self.assertEqual(num2words(3, lang="gl", ordinal=True), "tres-o")
        self.assertEqual(num2words(4, lang="gl", ordinal=True), "catro-o")
        self.assertEqual(num2words(5, lang="gl", ordinal=True), "cinco-o")
        self.assertEqual(num2words(6, lang="gl", ordinal=True), "seis-o")
        self.assertEqual(num2words(7, lang="gl", ordinal=True), "sete-o")
        self.assertEqual(num2words(8, lang="gl", ordinal=True), "oito-o")
        self.assertEqual(num2words(9, lang="gl", ordinal=True), "nove-o")
        self.assertEqual(num2words(10, lang="gl", ordinal=True), "dez-o")
        self.assertEqual(num2words(11, lang="gl", ordinal=True), "dez un-o")
        self.assertEqual(num2words(12, lang="gl", ordinal=True), "dez dous-o")
        self.assertEqual(num2words(13, lang="gl", ordinal=True), "dez tres-o")
        self.assertEqual(num2words(14, lang="gl", ordinal=True), "dez catro-o")
        self.assertEqual(num2words(15, lang="gl", ordinal=True), "dez cinco-o")
        self.assertEqual(num2words(16, lang="gl", ordinal=True), "dez seis-o")
        self.assertEqual(num2words(17, lang="gl", ordinal=True), "dez sete-o")
        self.assertEqual(num2words(18, lang="gl", ordinal=True), "dez oito-o")
        self.assertEqual(num2words(19, lang="gl", ordinal=True), "dez nove-o")
        self.assertEqual(num2words(20, lang="gl", ordinal=True), "vinte-o")
        self.assertEqual(num2words(21, lang="gl", ordinal=True), "vinte un-o")
        self.assertEqual(num2words(22, lang="gl", ordinal=True), "vinte dous-o")
        self.assertEqual(num2words(25, lang="gl", ordinal=True), "vinte cinco-o")
        self.assertEqual(num2words(30, lang="gl", ordinal=True), "trinta-o")
        self.assertEqual(num2words(40, lang="gl", ordinal=True), "corenta-o")
        self.assertEqual(num2words(50, lang="gl", ordinal=True), "cincuenta-o")
        self.assertEqual(num2words(60, lang="gl", ordinal=True), "sesenta-o")
        self.assertEqual(num2words(70, lang="gl", ordinal=True), "setenta-o")
        self.assertEqual(num2words(80, lang="gl", ordinal=True), "oitenta-o")
        self.assertEqual(num2words(90, lang="gl", ordinal=True), "noventa-o")
        self.assertEqual(num2words(100, lang="gl", ordinal=True), "un cento-o")
        self.assertEqual(num2words(101, lang="gl", ordinal=True), "un cento un-o")
        self.assertEqual(num2words(200, lang="gl", ordinal=True), "dous cento-o")
        self.assertEqual(num2words(500, lang="gl", ordinal=True), "cinco cento-o")
        self.assertEqual(num2words(1000, lang="gl", ordinal=True), "un mil-o")
        self.assertEqual(num2words(1001, lang="gl", ordinal=True), "un mil un-o")
        self.assertEqual(num2words(10000, lang="gl", ordinal=True), "dez mil-o")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="gl", to="currency", currency="EUR"), "cero euros"
        )
        self.assertEqual(
            num2words(0.01, lang="gl", to="currency", currency="EUR"),
            "cero euros un céntimo",
        )
        self.assertEqual(
            num2words(0.5, lang="gl", to="currency", currency="EUR"),
            "cero euros cincuenta céntimos",
        )
        self.assertEqual(
            num2words(1, lang="gl", to="currency", currency="EUR"), "un euro"
        )
        self.assertEqual(
            num2words(1.5, lang="gl", to="currency", currency="EUR"),
            "un euro cincuenta céntimos",
        )
        self.assertEqual(
            num2words(0, lang="gl", to="currency", currency="USD"), "cero dólares"
        )
        self.assertEqual(
            num2words(0.01, lang="gl", to="currency", currency="USD"),
            "cero dólares un centavo",
        )
        self.assertEqual(
            num2words(0.5, lang="gl", to="currency", currency="USD"),
            "cero dólares cincuenta centavos",
        )
        self.assertEqual(
            num2words(1, lang="gl", to="currency", currency="USD"), "un dólar"
        )
        self.assertEqual(
            num2words(1.5, lang="gl", to="currency", currency="USD"),
            "un dólar cincuenta centavos",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="gl", to="year"), "un mil")
        self.assertEqual(num2words(1066, lang="gl", to="year"), "un mil sesenta seis")
        self.assertEqual(
            num2words(1492, lang="gl", to="year"), "un mil catro cento noventa dous"
        )
        self.assertEqual(
            num2words(1776, lang="gl", to="year"), "un mil sete cento setenta seis"
        )
        self.assertEqual(num2words(1800, lang="gl", to="year"), "un mil oito cento")
        self.assertEqual(num2words(1900, lang="gl", to="year"), "un mil nove cento")
        self.assertEqual(
            num2words(1984, lang="gl", to="year"), "un mil nove cento oitenta catro"
        )
        self.assertEqual(
            num2words(1999, lang="gl", to="year"), "un mil nove cento noventa nove"
        )
        self.assertEqual(num2words(2000, lang="gl", to="year"), "dous mil")
        self.assertEqual(num2words(2001, lang="gl", to="year"), "dous mil un")
        self.assertEqual(num2words(2010, lang="gl", to="year"), "dous mil dez")
        self.assertEqual(num2words(2020, lang="gl", to="year"), "dous mil vinte")
        self.assertEqual(num2words(2024, lang="gl", to="year"), "dous mil vinte catro")
        self.assertEqual(num2words(2100, lang="gl", to="year"), "dous mil un cento")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="gl"), "cero")
        self.assertEqual(num2words("1", lang="gl"), "un")
        self.assertEqual(num2words("10", lang="gl"), "dez")
        self.assertEqual(num2words("100", lang="gl"), "un cento")
        self.assertEqual(num2words("1000", lang="gl"), "un mil")
        self.assertEqual(num2words("10000", lang="gl"), "dez mil")
        self.assertEqual(num2words("100000", lang="gl"), "un cento mil")
        self.assertEqual(num2words("1000000", lang="gl"), "un millón")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="gl"), "cero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="gl"), num2words("100", lang="gl"))
        self.assertEqual(num2words(1000, lang="gl"), num2words("1000", lang="gl"))


def test_scales_above_million_issue_147():
    """Words up to the scale table, then OverflowError -- never digits."""
    assert num2words(10**12, lang="gl") == "un billón"
    assert num2words(10**15, lang="gl") == "un mil billón"
    assert num2words(10**18, lang="gl") == "un trillón"
    with pytest.raises(OverflowError):
        num2words(10**24, lang="gl")
